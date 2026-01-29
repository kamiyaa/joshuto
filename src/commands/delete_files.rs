use std::path;
use std::sync::mpsc;

use ratatui::termion::event::Key;

use crate::error::{AppError, AppErrorKind, AppResult};
use crate::run::process_io::process_io_task;
use crate::types::io::{FileOperation, FileOperationOptions, IoTask};
use crate::types::state::AppState;
use crate::ui::widgets::TuiPrompt;
use crate::ui::AppBackend;

use super::tab_ops;

fn prompt(app_state: &mut AppState, backend: &mut AppBackend, paths_len: usize) -> bool {
    let ch = {
        let prompt_str = format!("Delete {} files? (Y/n)", paths_len);
        let mut prompt = TuiPrompt::new(&prompt_str);
        prompt.get_key(app_state, backend)
    };

    match ch {
        Key::Char('Y') | Key::Char('y') | Key::Char('\n') => {
            if paths_len > 1 {
                // prompt user again for deleting multiple files
                let ch = {
                    let prompt_str = "Are you sure? (y/N)";
                    let mut prompt = TuiPrompt::new(prompt_str);
                    prompt.get_key(app_state, backend)
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
    app_state: &mut AppState,
    paths: Vec<path::PathBuf>,
    background: bool,
    permanently: bool,
) -> AppResult {
    let file_op = FileOperation::Delete;
    let options = FileOperationOptions {
        permanently: !app_state.config.use_trash || permanently,
        ..Default::default()
    };

    let dest = path::PathBuf::new();
    let io_task = IoTask::new(file_op, paths.clone(), dest, options);
    if background {
        app_state.state.worker_state_mut().push_task(io_task);
    } else {
        let (wtx, _) = mpsc::channel();
        process_io_task(&io_task, &wtx)?;
    }

    let history = app_state.state.tab_state_mut().curr_tab_mut().history_mut();
    for path in paths.iter().filter(|p| p.is_dir()) {
        history.remove(path);
    }

    Ok(())
}

pub fn delete_selected_files(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    background: bool,
    permanently: bool,
    noconfirm: bool,
) -> AppResult {
    let paths = app_state
        .state
        .tab_state_ref()
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

    if noconfirm || prompt(app_state, backend, paths_len) {
        delete_files(app_state, paths, background, permanently)?;
    }

    let curr_path = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .get_cwd()
        .to_path_buf();
    tab_ops::reload_all_tabs(app_state, curr_path.as_path())?;
    Ok(())
}
