use std::path;
use std::sync::mpsc;

use termion::event::Key;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::io::{FileOperation, FileOperationOptions, IoWorkerThread};
use crate::ui::widgets::TuiPrompt;
use crate::ui::AppBackend;

fn delete_files(
    context: &mut AppContext,
    backend: &mut AppBackend,
    background: bool,
) -> std::io::Result<()> {
    let paths = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|s| s.get_selected_paths())
        .unwrap_or_else(Vec::new);
    let paths_len = paths.len();
    if paths_len == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no files selected",
        ));
    }

    let ch = {
        let prompt_str = format!("Delete {} files? (Y/n)", paths_len);
        let mut prompt = TuiPrompt::new(&prompt_str);
        prompt.get_key(backend, context)
    };

    match ch {
        Key::Char('Y') | Key::Char('y') | Key::Char('\n') => {
            let confirm_delete = if paths_len > 1 {
                // prompt user again for deleting multiple files
                let ch = {
                    let prompt_str = "Are you sure? (y/N)";
                    let mut prompt = TuiPrompt::new(prompt_str);
                    prompt.get_key(backend, context)
                };
                ch == Key::Char('y')
            } else {
                true
            };
            if confirm_delete {
                let file_op = FileOperation::Delete;
                let options = FileOperationOptions {
                    overwrite: false,
                    skip_exist: false,
                    permanently: !context.config_ref().use_trash,
                };

                let dest = path::PathBuf::new();
                let worker_thread = IoWorkerThread::new(file_op, paths, dest, options);
                if background {
                    context.worker_context_mut().push_worker(worker_thread);
                } else {
                    let (wtx, _) = mpsc::channel();
                    worker_thread.start(wtx)?;
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn _delete_selected_files(
    context: &mut AppContext,
    backend: &mut AppBackend,
) -> std::io::Result<()> {
    delete_files(context, backend, false)?;

    let curr_tab = context.tab_context_ref().curr_tab_ref();
    let options = context.config_ref().display_options_ref().clone();
    let curr_path = curr_tab.cwd().to_path_buf();
    let tab_option = curr_tab.option_ref().clone();
    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut()
            .reload(&curr_path, &options, &tab_option)?;
    }
    Ok(())
}

pub fn delete_selected_files(context: &mut AppContext, backend: &mut AppBackend) -> JoshutoResult {
    _delete_selected_files(context, backend)?;
    Ok(())
}

fn _delete_selected_files_background(
    context: &mut AppContext,
    backend: &mut AppBackend,
) -> std::io::Result<()> {
    delete_files(context, backend, true)?;

    let curr_tab = context.tab_context_ref().curr_tab_ref();
    let options = context.config_ref().display_options_ref().clone();
    let curr_path = curr_tab.cwd().to_path_buf();
    let tab_option = curr_tab.option_ref().clone();
    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut()
            .reload(&curr_path, &options, &tab_option)?;
    }
    Ok(())
}

pub fn delete_selected_files_background(
    context: &mut AppContext,
    backend: &mut AppBackend,
) -> JoshutoResult {
    _delete_selected_files(context, backend)?;
    Ok(())
}
