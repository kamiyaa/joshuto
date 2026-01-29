use std::io;
use std::path;

use notify;
use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::termion::event::{Event, Key, MouseButton, MouseEvent};
use signal_hook::consts::signal;
use uuid::Uuid;

use crate::commands::tab_ops;
use crate::commands::{cursor_move, parent_cursor_move, reload};
use crate::error::AppResult;
use crate::fs::JoshutoDirList;
use crate::preview::preview_dir::PreviewDirState;
use crate::preview::preview_file::PreviewFileState;
use crate::traits::app_execute::AppExecute;
use crate::types::command::Command;
use crate::types::event::AppEvent;
use crate::types::event::PreviewData;
use crate::types::io::IoTaskProgressMessage;
use crate::types::io::IoTaskStat;
use crate::types::keybind::CommandKeybind;
use crate::types::keybind::KeyMapping;
use crate::types::keymap::AppKeyMapping;
use crate::types::state::AppState;
use crate::ui;
use crate::ui::views::TuiCommandMenu;
use crate::utils::format;

pub fn poll_event_until_simple_keybind<'a>(
    app_state: &mut AppState,
    backend: &mut ui::AppBackend,
    keymap: &'a KeyMapping,
) -> Option<&'a Vec<Command>> {
    let mut keymap = keymap;

    app_state.flush_event();

    loop {
        backend.render(TuiCommandMenu::new(app_state, keymap));

        let event = match app_state.poll_event() {
            Ok(event) => event,
            _ => continue,
        };

        match event {
            AppEvent::TerminalEvent(event) => {
                match event {
                    Event::Key(Key::Esc) => return None,
                    event => match keymap.get(&event) {
                        Some(CommandKeybind::SimpleKeybind { commands, .. }) => {
                            return Some(commands);
                        }
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            keymap = m;
                        }
                        None => return None,
                    },
                }
                app_state.flush_event();
            }
            event => process_noninteractive(event, app_state),
        }
    }
}

pub fn process_noninteractive(event: AppEvent, app_state: &mut AppState) {
    match event {
        AppEvent::NewIoTask => process_new_io_task(app_state),
        AppEvent::IoTaskStart(stats) => process_io_task_start(app_state, stats),
        AppEvent::IoTaskProgress(res) => process_io_task_progress(app_state, res),
        AppEvent::IoTaskResult(res) => process_finished_io_task(app_state, res),
        AppEvent::PreviewDir { id, path, res } => process_dir_preview(app_state, id, path, *res),
        AppEvent::PreviewFile { path, res } => process_file_preview(app_state, path, res),
        AppEvent::Signal(signal::SIGWINCH) => {}
        AppEvent::Filesystem(e) => process_filesystem_event(e, app_state),
        AppEvent::ChildProcessComplete(child_id) => {
            app_state.state.thread_pool.join_child(child_id);
        }
        _ => {}
    }
}

fn process_filesystem_event(_event: notify::Event, app_state: &mut AppState) {
    let _ = reload::soft_reload_curr_tab(app_state);
}

pub fn process_new_io_task(app_state: &mut AppState) {
    if app_state.state.worker_state_ref().is_busy() {
        return;
    }
    if app_state.state.worker_state_ref().is_empty() {
        return;
    }
    let _ = app_state.state.worker_state_mut().start_next_job();
}

pub fn process_io_task_start(app_state: &mut AppState, stats: IoTaskStat) {
    app_state.state.worker_state.progress = Some(stats);
}

pub fn process_io_task_progress(app_state: &mut AppState, res: IoTaskProgressMessage) {
    let worker_state = app_state.state.worker_state_mut();
    if let Some(observer) = worker_state.progress.as_mut() {
        observer.process_msg(res);
        observer.update_msg();
    }
}

pub fn process_finished_io_task(app_state: &mut AppState, res: AppResult) {
    match res {
        Err(err) => {
            let msg = format!("{err}");
            app_state.state.message_queue_mut().push_error(msg);
        }
        Ok(_) => {
            let worker_state = app_state.state.worker_state_mut();
            if let Some(io_stat) = worker_state.remove_io_stat() {
                let io_path = io_stat.dest_path();
                if io_path.exists() {
                    let _ = tab_ops::reload_all_tabs(app_state, io_path);
                } else {
                    tab_ops::remove_entry_from_all_tabs(app_state, io_path);
                }

                let io_path = io_stat.src_path();
                if io_path.exists() {
                    let _ = tab_ops::reload_all_tabs(app_state, io_path);
                } else {
                    tab_ops::remove_entry_from_all_tabs(app_state, io_path);
                }

                let progress = io_stat.progress;
                let op = progress.kind.actioned_str();
                let processed_size = format::file_size_to_string(progress.bytes_processed);
                let total_size = format::file_size_to_string(progress.total_bytes);
                let msg = format!(
                    "successfully {} {} items ({}/{})",
                    op, progress.total_files, processed_size, total_size,
                );
                app_state.state.message_queue_mut().push_success(msg);
            }
        }
    }
    app_state.state.worker_state_mut().progress = None;
    process_new_io_task(app_state);
}

pub fn process_dir_preview(
    app_state: &mut AppState,
    id: Uuid,
    path: path::PathBuf,
    res: io::Result<JoshutoDirList>,
) {
    for (tab_id, tab) in app_state.state.tab_state_mut().iter_mut() {
        if *tab_id == id {
            match res {
                Ok(dirlist) => {
                    // remove from loading state
                    tab.history_metadata_mut().remove(dirlist.file_path());

                    let history = tab.history_mut();
                    let dir_path = dirlist.file_path().to_path_buf();
                    history.insert(dir_path, dirlist);
                }
                Err(e) => {
                    // set to false so we don't load again
                    tab.history_metadata_mut().insert(
                        path,
                        PreviewDirState::Error {
                            message: e.to_string(),
                        },
                    );
                }
            }
            break;
        }
    }
}

pub fn process_file_preview(
    app_state: &mut AppState,
    path: path::PathBuf,
    res: io::Result<PreviewData>,
) {
    let preview_state = app_state.state.preview_state_mut();
    match res {
        Ok(PreviewData::Script(output)) if output.status.code().is_some() => {
            preview_state
                .previews_mut()
                .insert(path, PreviewFileState::Success(*output));
        }
        Ok(PreviewData::Script(_)) => {
            preview_state
                .previews_mut()
                .insert(path, PreviewFileState::Error("status error".to_owned()));
        }
        Ok(PreviewData::Image(protocol)) => {
            preview_state.set_image_preview(Some((path, protocol)));
        }
        Err(e) => {
            preview_state
                .previews_mut()
                .insert(path, PreviewFileState::Error(e.to_string()));
        }
    };
}

pub fn process_unsupported(
    app_state: &mut AppState,
    backend: &mut ui::AppBackend,
    keymap_t: &AppKeyMapping,
    event: Vec<u8>,
) {
    match event.as_slice() {
        [27, 79, 65] => {
            let command = Command::CursorMoveUp { offset: 1 };
            if let Err(e) = command.execute(app_state, backend, keymap_t) {
                app_state
                    .state
                    .message_queue_mut()
                    .push_error(e.to_string());
            }
        }
        [27, 79, 66] => {
            let command = Command::CursorMoveDown { offset: 1 };
            if let Err(e) = command.execute(app_state, backend, keymap_t) {
                app_state
                    .state
                    .message_queue_mut()
                    .push_error(e.to_string());
            }
        }
        _ => {}
    }
}

enum Panel {
    Parent,
    Current,
    Preview,
}

fn children_cursor_move(app_state: &mut AppState, new_index: usize) {
    let mut new_index = new_index;
    let ui_state = app_state.state.ui_state_ref().clone();
    let display_options = &app_state.config.display_options;
    if let Some(children_list) = app_state
        .state
        .tab_state_mut()
        .curr_tab_mut()
        .child_list_mut()
    {
        if !children_list.is_empty() {
            let dir_len = children_list.len();
            if new_index >= dir_len {
                new_index = dir_len - 1;
            }
            children_list.set_index(Some(new_index), &ui_state, display_options);
        }
    }
}

pub fn process_mouse(
    app_state: &mut AppState,
    backend: &mut ui::AppBackend,
    keymap_t: &AppKeyMapping,
    event: MouseEvent,
) {
    let f_size = backend.terminal.as_ref().unwrap().size().unwrap();

    let rect = Rect {
        x: 0,
        y: 0,
        width: f_size.width,
        height: f_size.height,
    };

    let constraints: &[Constraint; 3] = &app_state.config.display_options.default_layout;
    let vertical_margin = if app_state.config.display_options.show_borders {
        2
    } else {
        1
    };

    let layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(vertical_margin)
        .constraints(constraints.as_ref())
        .split(rect);

    match event {
        MouseEvent::Press(MouseButton::WheelUp, x, y) => {
            if y == 1 {
                let command = Command::TabSwitch { offset: -1 };
                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                    app_state
                        .state
                        .message_queue_mut()
                        .push_error(e.to_string());
                }
            } else if x < layout_rect[1].x {
                let command = Command::ParentCursorMoveUp { offset: 1 };
                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                    app_state
                        .state
                        .message_queue_mut()
                        .push_error(e.to_string());
                }
            } else if x < layout_rect[2].x {
                let command = Command::CursorMoveUp { offset: 1 };
                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                    app_state
                        .state
                        .message_queue_mut()
                        .push_error(e.to_string());
                }
            } else {
                let command = Command::PreviewCursorMoveUp { offset: 1 };
                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                    app_state
                        .state
                        .message_queue_mut()
                        .push_error(e.to_string());
                }
            }
        }
        MouseEvent::Press(MouseButton::WheelDown, x, y) => {
            if y == 1 {
                let command = Command::TabSwitch { offset: 1 };
                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                    app_state
                        .state
                        .message_queue_mut()
                        .push_error(e.to_string());
                }
            } else if x < layout_rect[1].x {
                let command = Command::ParentCursorMoveDown { offset: 1 };
                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                    app_state
                        .state
                        .message_queue_mut()
                        .push_error(e.to_string());
                }
            } else if x < layout_rect[2].x {
                let command = Command::CursorMoveDown { offset: 1 };
                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                    app_state
                        .state
                        .message_queue_mut()
                        .push_error(e.to_string());
                }
            } else {
                let command = Command::PreviewCursorMoveDown { offset: 1 };
                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                    app_state
                        .state
                        .message_queue_mut()
                        .push_error(e.to_string());
                }
            }
        }
        MouseEvent::Press(button @ MouseButton::Left, x, y)
        | MouseEvent::Press(button @ MouseButton::Right, x, y) => {
            if y > layout_rect[1].y && y <= layout_rect[1].y + layout_rect[1].height {
                let (dirlist, panel) = if x < layout_rect[1].x {
                    (
                        app_state
                            .state
                            .tab_state_mut()
                            .curr_tab_mut()
                            .parent_list_ref(),
                        Some(Panel::Parent),
                    )
                } else if x < layout_rect[2].x {
                    (
                        app_state
                            .state
                            .tab_state_mut()
                            .curr_tab_mut()
                            .curr_list_ref(),
                        Some(Panel::Current),
                    )
                } else {
                    (
                        app_state
                            .state
                            .tab_state_mut()
                            .curr_tab_mut()
                            .child_list_ref(),
                        Some(Panel::Preview),
                    )
                };
                if let Some(dirlist) = dirlist {
                    let skip_dist = dirlist.first_index_for_viewport();
                    let new_index = skip_dist + (y - layout_rect[1].y - 1) as usize;
                    match panel {
                        Some(Panel::Parent) => {
                            if let Err(e) =
                                parent_cursor_move::parent_cursor_move(app_state, new_index)
                            {
                                app_state
                                    .state
                                    .message_queue_mut()
                                    .push_error(e.to_string());
                            };
                            if button == MouseButton::Left {
                                let command = Command::ChangeDirectory {
                                    path: path::PathBuf::from(".."),
                                };
                                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                                    app_state
                                        .state
                                        .message_queue_mut()
                                        .push_error(e.to_string());
                                }
                            };
                        }
                        Some(Panel::Current) => {
                            cursor_move::cursor_move(app_state, new_index);
                            if button == MouseButton::Right {
                                let command = Command::OpenFile;
                                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                                    app_state
                                        .state
                                        .message_queue_mut()
                                        .push_error(e.to_string());
                                }
                            }
                        }
                        Some(Panel::Preview) => {
                            children_cursor_move(app_state, new_index);
                            if button == MouseButton::Left {
                                let command = Command::OpenFile;
                                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                                    app_state
                                        .state
                                        .message_queue_mut()
                                        .push_error(e.to_string());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    app_state.flush_event();
}
