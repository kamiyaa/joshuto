use std::collections::hash_map::Entry;

use signal_hook::consts::signal;
use termion::event::{MouseButton, MouseEvent};
use tui::layout::{Constraint, Direction, Layout};

use crate::commands::{cursor_move, parent_cursor_move, AppExecute, KeyCommand};
use crate::context::AppContext;
use crate::event::AppEvent;
use crate::fs::JoshutoDirList;
use crate::history::DirectoryHistory;
use crate::io::{FileOp, IoWorkerProgress};
use crate::preview::preview_file::FilePreview;
use crate::ui;
use crate::util::format;

pub fn process_noninteractive(event: AppEvent, context: &mut AppContext) {
    match event {
        AppEvent::IoWorkerProgress(res) => process_worker_progress(context, res),
        AppEvent::IoWorkerResult(res) => process_finished_worker(context, res),
        AppEvent::PreviewDir(Ok(dirlist)) => process_dir_preview(context, dirlist),
        AppEvent::PreviewFile(file_preview) => process_file_preview(context, file_preview),
        AppEvent::Signal(signal::SIGWINCH) => {}
        _ => {}
    }
}

pub fn process_worker_progress(context: &mut AppContext, res: IoWorkerProgress) {
    let worker_context = context.worker_context_mut();
    worker_context.set_progress(res);
    worker_context.update_msg();
}

pub fn process_finished_worker(context: &mut AppContext, res: std::io::Result<IoWorkerProgress>) {
    let worker_context = context.worker_context_mut();
    let observer = worker_context.remove_worker().unwrap();
    let options = context.config_ref().display_options_ref().clone();
    for tab in context.tab_context_mut().iter_mut() {
        let _ = tab.history_mut().reload(observer.dest_path(), &options);
        let _ = tab.history_mut().reload(observer.src_path(), &options);
    }
    observer.join();
    match res {
        Ok(progress) => {
            let op = match progress.kind() {
                FileOp::Copy => "copied",
                FileOp::Cut => "moved",
            };
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
}

pub fn process_dir_preview(context: &mut AppContext, dirlist: JoshutoDirList) {
    let history = context.tab_context_mut().curr_tab_mut().history_mut();

    let dir_path = dirlist.file_path().to_path_buf();
    match history.entry(dir_path) {
        Entry::Occupied(mut entry) => {
            let old_dirlist = entry.get();
            if old_dirlist.need_update() {
                entry.insert(dirlist);
            }
        }
        Entry::Vacant(entry) => {
            entry.insert(dirlist);
        }
    }
}

pub fn process_file_preview(context: &mut AppContext, file_preview: FilePreview) {
    match file_preview.status.code() {
        None => {}
        Some(_) => {
            context
                .preview_context_mut()
                .insert_preview(file_preview._path.clone(), file_preview);
        }
    }
}

pub fn process_mouse(event: MouseEvent, context: &mut AppContext, backend: &mut ui::TuiBackend) {
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
                let command = KeyCommand::ParentCursorMoveUp(1);
                if let Err(e) = command.execute(context, backend) {
                    context.message_queue_mut().push_error(e.to_string());
                }
            } else if x < layout_rect[2].x {
                let command = KeyCommand::CursorMoveUp(1);
                if let Err(e) = command.execute(context, backend) {
                    context.message_queue_mut().push_error(e.to_string());
                }
            } else {
                // TODO: scroll in child list
            }
        }
        MouseEvent::Press(MouseButton::WheelDown, x, _) => {
            if x < layout_rect[1].x {
                let command = KeyCommand::ParentCursorMoveDown(1);
                if let Err(e) = command.execute(context, backend) {
                    context.message_queue_mut().push_error(e.to_string());
                }
            } else if x < layout_rect[2].x {
                let command = KeyCommand::CursorMoveDown(1);
                if let Err(e) = command.execute(context, backend) {
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
                    let skip_dist =
                        dirlist.first_index_for_viewport(layout_rect[1].height as usize);
                    let new_index = skip_dist + (y - layout_rect[1].y - 1) as usize;
                    if let Err(e) = if is_parent {
                        parent_cursor_move::parent_cursor_move(new_index, context)
                    } else {
                        cursor_move::cursor_move(new_index, context)
                    } {
                        context.message_queue_mut().push_error(e.to_string());
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
