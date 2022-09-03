use std::io;
use std::path;

use notify;
use signal_hook::consts::signal;
use termion::event::{Event, Key, MouseButton, MouseEvent};
use tui::layout::{Constraint, Direction, Layout};
use uuid::Uuid;

use crate::commands::{cursor_move, parent_cursor_move, reload};
use crate::config::{AppKeyMapping, KeyMapping};
use crate::context::AppContext;
use crate::event::AppEvent;
use crate::fs::JoshutoDirList;
use crate::history::DirectoryHistory;
use crate::io::FileOperationProgress;
use crate::key_command::{AppExecute, Command, CommandKeybind};
use crate::preview::preview_dir::PreviewDirState;
use crate::preview::preview_file::{FilePreview, PreviewFileState};
use crate::ui;
use crate::ui::views::TuiCommandMenu;
use crate::util::format;

pub fn get_input_while_composite<'a>(
    backend: &mut ui::AppBackend,
    context: &mut AppContext,
    keymap: &'a KeyMapping,
) -> Option<&'a Command> {
    let mut keymap = keymap;

    context.flush_event();

    loop {
        backend.render(TuiCommandMenu::new(context, keymap));

        if let Ok(event) = context.poll_event() {
            match event {
                AppEvent::Termion(event) => {
                    match event {
                        Event::Key(Key::Esc) => return None,
                        event => match keymap.get(&event) {
                            Some(CommandKeybind::SimpleKeybind(s)) => {
                                return Some(s);
                            }
                            Some(CommandKeybind::CompositeKeybind(m)) => {
                                keymap = m;
                            }
                            None => return None,
                        },
                    }
                    context.flush_event();
                }
                event => process_noninteractive(event, context),
            }
        }
    }
}

pub fn process_noninteractive(event: AppEvent, context: &mut AppContext) {
    match event {
        AppEvent::IoWorkerCreate => process_new_worker(context),
        AppEvent::FileOperationProgress(res) => process_worker_progress(context, res),
        AppEvent::IoWorkerResult(res) => process_finished_worker(context, res),
        AppEvent::PreviewDir { id, path, res } => process_dir_preview(context, id, path, *res),
        AppEvent::PreviewFile { path, res } => process_file_preview(context, path, *res),
        AppEvent::Signal(signal::SIGWINCH) => {}
        AppEvent::Filesystem(e) => process_filesystem_event(e, context),
        AppEvent::ChildProcessComplete(child_id) => {
            context.worker_context_mut().join_child(child_id);
        }
        _ => {}
    }
}

fn process_filesystem_event(_event: notify::Event, context: &mut AppContext) {
    let _ = reload::soft_reload_curr_tab(context);
}

pub fn process_new_worker(context: &mut AppContext) {
    if !context.worker_context_ref().is_busy() && !context.worker_context_ref().is_empty() {
        context.worker_context_mut().start_next_job();
    }
}

pub fn process_worker_progress(context: &mut AppContext, res: FileOperationProgress) {
    let worker_context = context.worker_context_mut();
    worker_context.set_progress(res);
    worker_context.update_msg();
}

pub fn process_finished_worker(
    context: &mut AppContext,
    res: std::io::Result<FileOperationProgress>,
) {
    let worker_context = context.worker_context_mut();
    let observer = worker_context.remove_worker().unwrap();
    let options = context.config_ref().display_options_ref().clone();
    for (_, tab) in context.tab_context_mut().iter_mut() {
        let tab_options = tab.option_ref().clone();
        if observer.dest_path().exists() {
            let _ = tab
                .history_mut()
                .reload(observer.dest_path(), &options, &tab_options);
        } else {
            tab.history_mut().remove(observer.dest_path());
        }
        if observer.src_path().exists() {
            let _ = tab
                .history_mut()
                .reload(observer.src_path(), &options, &tab_options);
        } else {
            tab.history_mut().remove(observer.src_path());
        }
    }

    observer.join();
    match res {
        Ok(progress) => {
            let op = progress.kind().actioned_str();
            let processed_size = format::file_size_to_string(progress.bytes_processed());
            let total_size = format::file_size_to_string(progress.total_bytes());
            let msg = format!(
                "successfully {} {} items ({}/{})",
                op,
                progress.total_files(),
                processed_size,
                total_size,
            );
            context.message_queue_mut().push_success(msg);
        }
        Err(e) => {
            let msg = format!("{}", e);
            context.message_queue_mut().push_error(msg);
        }
    }

    if !context.worker_context_ref().is_busy() && !context.worker_context_ref().is_empty() {
        context.worker_context_mut().start_next_job();
    }
}

pub fn process_dir_preview(
    context: &mut AppContext,
    id: Uuid,
    path: path::PathBuf,
    res: io::Result<JoshutoDirList>,
) {
    for (tab_id, tab) in context.tab_context_mut().iter_mut() {
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
    context: &mut AppContext,
    path: path::PathBuf,
    res: io::Result<FilePreview>,
) {
    match res {
        Ok(preview) => {
            if preview.status.code().is_some() {
                context
                    .preview_context_mut()
                    .previews_mut()
                    .insert(path, PreviewFileState::Success { data: preview });
            } else {
                context.preview_context_mut().previews_mut().insert(
                    path,
                    PreviewFileState::Error {
                        message: "Unknown error".to_string(),
                    },
                );
            }
        }
        Err(e) => {
            context.preview_context_mut().previews_mut().insert(
                path,
                PreviewFileState::Error {
                    message: e.to_string(),
                },
            );
        }
    }
}

pub fn process_unsupported(
    context: &mut AppContext,
    backend: &mut ui::AppBackend,
    keymap_t: &AppKeyMapping,
    event: Vec<u8>,
) {
    match event.as_slice() {
        [27, 79, 65] => {
            let command = Command::CursorMoveUp { offset: 1 };
            if let Err(e) = command.execute(context, backend, keymap_t) {
                context.message_queue_mut().push_error(e.to_string());
            }
        }
        [27, 79, 66] => {
            let command = Command::CursorMoveDown { offset: 1 };
            if let Err(e) = command.execute(context, backend, keymap_t) {
                context.message_queue_mut().push_error(e.to_string());
            }
        }
        _ => {}
    }
}

pub fn process_mouse(
    event: MouseEvent,
    context: &mut AppContext,
    backend: &mut ui::AppBackend,
    keymap_t: &AppKeyMapping,
) {
    let f_size = backend.terminal.as_ref().unwrap().size().unwrap();

    let constraints: &[Constraint; 3] = &context.config_ref().display_options_ref().default_layout;
    let vertical_margin = if context.config_ref().display_options_ref().show_borders() {
        2
    } else {
        1
    };

    let layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(vertical_margin)
        .constraints(constraints.as_ref())
        .split(f_size);

    match event {
        MouseEvent::Press(MouseButton::WheelUp, x, _) => {
            if x < layout_rect[1].x {
                let command = Command::ParentCursorMoveUp { offset: 1 };
                if let Err(e) = command.execute(context, backend, keymap_t) {
                    context.message_queue_mut().push_error(e.to_string());
                }
            } else if x < layout_rect[2].x {
                let command = Command::CursorMoveUp { offset: 1 };
                if let Err(e) = command.execute(context, backend, keymap_t) {
                    context.message_queue_mut().push_error(e.to_string());
                }
            } else {
                // TODO: scroll in child list
            }
        }
        MouseEvent::Press(MouseButton::WheelDown, x, _) => {
            if x < layout_rect[1].x {
                let command = Command::ParentCursorMoveDown { offset: 1 };
                if let Err(e) = command.execute(context, backend, keymap_t) {
                    context.message_queue_mut().push_error(e.to_string());
                }
            } else if x < layout_rect[2].x {
                let command = Command::CursorMoveDown { offset: 1 };
                if let Err(e) = command.execute(context, backend, keymap_t) {
                    context.message_queue_mut().push_error(e.to_string());
                }
            } else {
                // TODO: scroll in child list
            }
        }
        MouseEvent::Press(MouseButton::Left, x, y)
            if y > layout_rect[1].y && y <= layout_rect[1].y + layout_rect[1].height =>
        {
            if x < layout_rect[2].x {
                let (dirlist, is_parent) = if x < layout_rect[1].x {
                    (
                        context.tab_context_ref().curr_tab_ref().parent_list_ref(),
                        true,
                    )
                } else {
                    (
                        context.tab_context_ref().curr_tab_ref().curr_list_ref(),
                        false,
                    )
                };
                if let Some(dirlist) = dirlist {
                    let skip_dist = dirlist.first_index_for_viewport();
                    let new_index = skip_dist + (y - layout_rect[1].y - 1) as usize;
                    if is_parent {
                        if let Err(e) = parent_cursor_move::parent_cursor_move(context, new_index) {
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    } else {
                        cursor_move::cursor_move(context, new_index);
                    }
                }
            } else {
            }
        }
        MouseEvent::Press(MouseButton::Left, _, y)
            if y > layout_rect[1].y && y <= layout_rect[1].y + layout_rect[1].height => {}
        _ => {}
    }
    context.flush_event();
}
