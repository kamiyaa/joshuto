use std::process::{Command, Stdio};

use crate::error::{AppError, AppErrorKind, AppResult};
use crate::types::io::{FileOperation, FileOperationOptions, IoTask};
use crate::types::state::{AppState, LocalStateState};

fn new_local_state(app_state: &mut AppState, file_op: FileOperation) -> Option<()> {
    let list = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()?;
    let selected = list.get_selected_paths();

    let mut local_state = LocalStateState::new();
    local_state.set_paths(selected.into_iter());
    local_state.set_file_op(file_op);

    app_state.state.set_local_state(local_state);
    Some(())
}

pub fn cut(app_state: &mut AppState) -> AppResult {
    new_local_state(app_state, FileOperation::Cut);
    Ok(())
}

pub fn copy(app_state: &mut AppState) -> AppResult {
    new_local_state(app_state, FileOperation::Copy);
    Ok(())
}

pub fn create_io_task(
    app_state: &mut AppState,
    operation: FileOperation,
    options: FileOperationOptions,
) -> AppResult {
    let local_state = app_state.state.take_local_state().ok_or_else(|| {
        let err_msg = "No files selected";
        AppError::new(AppErrorKind::InvalidParameters, err_msg.to_string())
    })?;

    if local_state.paths.is_empty() {
        let err_msg = "No files selected";
        let err = AppError::new(AppErrorKind::InvalidParameters, err_msg.to_string());
        return Err(err);
    }

    let dest = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .get_cwd()
        .to_path_buf();
    let worker_thread = IoTask::new(operation, local_state.paths, dest, options);
    app_state.state.worker_state_mut().push_task(worker_thread);

    Ok(())
}

pub fn create_io_paste_task(app_state: &mut AppState, options: FileOperationOptions) -> AppResult {
    let local_state = app_state.state.take_local_state().ok_or_else(|| {
        let err_msg = "No files selected";
        AppError::new(AppErrorKind::InvalidParameters, err_msg.to_string())
    })?;

    if local_state.paths.is_empty() {
        let err_msg = "No files selected";
        let err = AppError::new(AppErrorKind::InvalidParameters, err_msg.to_string());
        return Err(err);
    }

    let dest = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .get_cwd()
        .to_path_buf();
    let worker_thread = IoTask::new(local_state.file_op, local_state.paths, dest, options);
    app_state.state.worker_state_mut().push_task(worker_thread);

    Ok(())
}

pub fn copy_filename(app_state: &mut AppState) -> AppResult {
    let entry_file_name = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|c| c.curr_entry_ref())
        .map(|entry| entry.file_name().to_string());

    if let Some(file_name) = entry_file_name {
        copy_string_to_buffer(&file_name)?;
    }
    Ok(())
}

pub fn copy_filename_without_extension(app_state: &mut AppState) -> AppResult {
    let entry_file_name = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|c| c.curr_entry_ref())
        .map(|entry| {
            entry
                .file_name()
                .rsplit_once('.')
                .map(|(name, _)| name.to_string())
                .unwrap_or_else(|| entry.file_name().to_string())
        });

    if let Some(file_name) = entry_file_name {
        copy_string_to_buffer(&file_name)?;
    }
    Ok(())
}

pub fn copy_filepath(app_state: &mut AppState, all: bool) -> AppResult {
    let selected = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref();
    let entry_file_path = {
        if all {
            selected.map(|c| c.get_selected_paths()).and_then(|sel| {
                sel.into_iter().try_fold(String::new(), |mut acc, x| {
                    if let Some(s) = x.to_str() {
                        acc.push_str(s);
                        acc.push('\n');
                    }
                    Some(acc)
                })
            })
        } else {
            selected
                .and_then(|c| c.curr_entry_ref())
                .and_then(|entry| entry.file_path().to_str())
                .map(|s| s.to_string())
        }
    };
    if let Some(file_path) = entry_file_path {
        copy_string_to_buffer(&file_path)?;
    }
    Ok(())
}

pub fn copy_dirpath(app_state: &mut AppState) -> AppResult {
    let opt_entry = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|dirlist| dirlist.file_path());

    if let Some(s) = opt_entry.and_then(|p| p.to_str().map(String::from)) {
        copy_string_to_buffer(&s)?
    };
    Ok(())
}

fn copy_string_to_buffer(s: &str) -> AppResult {
    let escaped_string = escape_string(s);
    let clipboards = [
        (
            "wl-copy",
            format!("printf '%s' '{}' | {}", escaped_string, "wl-copy"),
        ),
        (
            "xsel",
            format!("printf '%s' '{}' | {} -ib", escaped_string, "xsel"),
        ),
        (
            "pbcopy",
            format!("printf '%s' '{}' | {}", escaped_string, "pbcopy"),
        ),
        (
            "xclip",
            format!(
                "printf '%s' '{}' | {} -selection clipboard",
                escaped_string, "xclip"
            ),
        ),
    ];

    for (_, cmd) in clipboards.iter() {
        let status = Command::new("sh")
            .args(["-c", cmd.as_str()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match status {
            Ok(s) if s.success() => return Ok(()),
            _ => {}
        }
    }
    Err(AppError::new(
        AppErrorKind::Clipboard,
        "Failed to copy to clipboard".to_string(),
    ))
}

pub fn escape_string(s: &str) -> String {
    s.replace("'", "'\\''")
}
