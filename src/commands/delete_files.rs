use std::path;
use std::sync::mpsc;

use termion::event::Key;

use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::io::{FileOperation, FileOperationOptions, IoWorkerThread};
use crate::ui::widgets::TuiPrompt;
use crate::ui::AppBackend;

use super::tab_ops;

fn prompt(context: &mut AppContext, backend: &mut AppBackend, paths_len: usize) -> bool {
    let ch = {
        let prompt_str = format!("Delete {} files? (Y/n)", paths_len);
        let mut prompt = TuiPrompt::new(&prompt_str);
        prompt.get_key(backend, context)
    };

    match ch {
        Key::Char('Y') | Key::Char('y') | Key::Char('\n') => {
            if paths_len > 1 {
                // prompt user again for deleting multiple files
                let ch = {
                    let prompt_str = "Are you sure? (y/N)";
                    let mut prompt = TuiPrompt::new(prompt_str);
                    prompt.get_key(backend, context)
                };
                ch == Key::Char('y')
            } else {
                true
            }
        }
        _ => false,
    }
}

fn delete_files(
    context: &mut AppContext,
    paths: Vec<path::PathBuf>,
    background: bool,
    permanently: bool,
) -> AppResult<()> {
    let file_op = FileOperation::Delete;
    let options = FileOperationOptions {
        overwrite: false,
        skip_exist: false,
        permanently: !context.config_ref().use_trash || permanently,
    };

    let dest = path::PathBuf::new();
    let worker_thread = IoWorkerThread::new(file_op, paths.clone(), dest, options);
    if background {
        context.worker_context_mut().push_worker(worker_thread);
    } else {
        let (wtx, _) = mpsc::channel();
        worker_thread.start(wtx)?;
    }

    let history = context.tab_context_mut().curr_tab_mut().history_mut();
    for path in paths.iter().filter(|p| p.is_dir()) {
        history.remove(path);
    }

    Ok(())
}

pub fn delete_selected_files(
    context: &mut AppContext,
    backend: &mut AppBackend,
    background: bool,
    permanently: bool,
    noconfirm: bool,
) -> AppResult {
    let paths = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|s| s.get_selected_paths())
        .unwrap_or_default();

    let paths_len = paths.len();
    if paths_len == 0 {
        let err = AppError::new(
            AppErrorKind::InvalidParameters,
            "no files selected".to_string(),
        );
        return Err(err);
    }

    if noconfirm || prompt(context, backend, paths_len) {
        delete_files(context, paths, background, permanently)?;
    }

    let curr_path = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
    tab_ops::reload_all_tabs(context, curr_path.as_path())?;
    Ok(())
}
