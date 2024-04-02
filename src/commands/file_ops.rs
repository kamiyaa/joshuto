use std::fs::{self, File};
use std::io::{self, Read};
use std::io::Write;
use std::path;
use std::process::{Command, Stdio};

use crate::config::{search_directories, ConfigType};
use crate::context::{AppContext, LocalStateContext};
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::io::{FileOperation, FileOperationOptions, IoWorkerThread};

use crate::{BOOKMARKS_T, CONFIG_HIERARCHY};

fn find_state_context_file() -> Option<path::PathBuf> {
    for p in CONFIG_HIERARCHY.iter() {
        if p.exists() {
            return Some(p.clone());
        }
    }
    None
}

fn new_local_state(context: &mut AppContext, file_op: FileOperation) -> Option<LocalStateContext> {
    let list = context.tab_context_ref().curr_tab_ref().curr_list_ref()?;
    let selected = list.get_selected_paths();

    let mut local_state = LocalStateContext::new();
    local_state.set_paths(selected.into_iter());
    local_state.set_file_op(file_op);

    Some(local_state)
}

pub fn cut(context: &mut AppContext) -> AppResult {
    let local_state = new_local_state(context, FileOperation::Cut).unwrap_or(LocalStateContext::new());
    context.set_local_state(local_state);
    Ok(())
}

pub fn copy(context: &mut AppContext) -> AppResult {
    let local_state = new_local_state(context, FileOperation::Copy).unwrap_or(LocalStateContext::new());
    context.set_local_state(local_state);
    Ok(())
}

pub fn symlink_absolute(context: &mut AppContext) -> AppResult {
    let local_state = new_local_state(context, FileOperation::Symlink { relative: false }).unwrap_or(LocalStateContext::new());
    context.set_local_state(local_state);
    Ok(())
}

pub fn symlink_relative(context: &mut AppContext) -> AppResult {
    let local_state = new_local_state(context, FileOperation::Symlink { relative: true }).unwrap_or(LocalStateContext::new());
    context.set_local_state(local_state);
    Ok(())
}

pub fn save_local_state(local_state: &LocalStateContext) -> AppResult {
    let local_state_path = match search_directories(ConfigType::StateContext.as_filename(), &CONFIG_HIERARCHY) {
        Some(file_path) => Some(file_path),
        None => find_state_context_file(),
    };

    if let Some(local_state_path) = local_state_path {
        if let Ok(content) = toml::to_string(&local_state) {
            let mut file = File::create(local_state_path)?;
            file.write_all(content.as_bytes())?;
        }
    }
    Ok(())
}

pub fn paste(context: &mut AppContext, options: FileOperationOptions) -> AppResult {
    match context.take_local_state() {
        Some(state) if !state.paths.is_empty() => {
            let dest = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
            let worker_thread = IoWorkerThread::new(state.file_op, state.paths, dest, options);
            context.worker_context_mut().push_worker(worker_thread);
            Ok(())
        }
        _ => Err(AppError::new(
            AppErrorKind::Io(io::ErrorKind::InvalidData),
            "no files selected".to_string(),
        )),
    }
}

pub fn cut_export(context: &mut AppContext) -> AppResult {
    let local_state = new_local_state(context, FileOperation::Cut).unwrap_or(LocalStateContext::new());
    let _ = save_local_state(&local_state);
    Ok(())
}

pub fn copy_export(context: &mut AppContext) -> AppResult {
    let local_state = new_local_state(context, FileOperation::Copy).unwrap_or(LocalStateContext::new());
    let _ = save_local_state(&local_state);
    Ok(())
}
pub fn symlink_absolute_export(context: &mut AppContext) -> AppResult {
    let local_state = new_local_state(context, FileOperation::Symlink { relative: false }).unwrap_or(LocalStateContext::new());
    let _ = save_local_state(&local_state);
    Ok(())
}

pub fn symlink_relative_export(context: &mut AppContext) -> AppResult {
    let local_state = new_local_state(context, FileOperation::Symlink { relative: true }).unwrap_or(LocalStateContext::new());
    let _ = save_local_state(&local_state);
    Ok(())
}

pub fn paste_import(context: &mut AppContext, options: FileOperationOptions) -> AppResult {

    let local_state_path = match search_directories(ConfigType::StateContext.as_filename(), &CONFIG_HIERARCHY) {
        Some(file_path) => Some(file_path),
        None => find_state_context_file(),
    };

    if let Some(local_state_path) = local_state_path {
        let file_contents = fs::read_to_string(&local_state_path)?;
        let local_state = toml::from_str::<LocalStateContext>(&file_contents).unwrap();

        let dest = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
        let worker_thread = IoWorkerThread::new(local_state.file_op, local_state.paths, dest, options);
        context.worker_context_mut().push_worker(worker_thread);
    }
    Ok(())
}


pub fn copy_filename(context: &mut AppContext) -> AppResult {
    let entry_file_name = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|c| c.curr_entry_ref())
        .map(|entry| entry.file_name().to_string());

    if let Some(file_name) = entry_file_name {
        copy_string_to_buffer(file_name)?;
    }
    Ok(())
}

pub fn copy_filename_without_extension(context: &mut AppContext) -> AppResult {
    let entry_file_name = context
        .tab_context_ref()
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
        copy_string_to_buffer(file_name)?;
    }
    Ok(())
}

pub fn copy_filepath(context: &mut AppContext, all: bool) -> AppResult {
    let selected = context.tab_context_ref().curr_tab_ref().curr_list_ref();
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
        copy_string_to_buffer(file_path)?;
    }
    Ok(())
}

pub fn copy_dirpath(context: &mut AppContext) -> AppResult {
    let opt_entry = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|dirlist| dirlist.file_path());

    if let Some(s) = opt_entry.and_then(|p| p.to_str().map(String::from)) {
        copy_string_to_buffer(s)?
    };
    Ok(())
}

fn copy_string_to_buffer(string: String) -> AppResult {
    let clipboards = [
        (
            "wl-copy",
            format!("printf '%s' '{}' | {}", string, "wl-copy"),
        ),
        ("xsel", format!("printf '%s' '{}' | {} -ib", string, "xsel")),
        ("pbcopy", format!("printf '%s' '{}' | {}", string, "pbcopy")),
        (
            "xclip",
            format!(
                "printf '%s' '{}' | {} -selection clipboard",
                string, "xclip"
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
        AppErrorKind::ClipboardError,
        "Failed to copy to clipboard".to_string(),
    ))
}
