use std::io;
use std::process::{Command, Stdio};

use crate::context::{AppContext, LocalStateContext};
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::io::{FileOperation, FileOperationOptions, IoWorkerThread};

fn new_local_state(context: &mut AppContext, file_op: FileOperation) -> Option<()> {
    let list = context.tab_context_ref().curr_tab_ref().curr_list_ref()?;
    let selected = list.get_selected_paths();

    let mut local_state = LocalStateContext::new();
    local_state.set_paths(selected.into_iter());
    local_state.set_file_op(file_op);

    context.set_local_state(local_state);
    Some(())
}

pub fn cut(context: &mut AppContext) -> JoshutoResult {
    new_local_state(context, FileOperation::Cut);
    Ok(())
}

pub fn copy(context: &mut AppContext) -> JoshutoResult {
    new_local_state(context, FileOperation::Copy);
    Ok(())
}

pub fn symlink_absolute(context: &mut AppContext) -> JoshutoResult {
    new_local_state(context, FileOperation::Symlink { relative: false });
    Ok(())
}

pub fn symlink_relative(context: &mut AppContext) -> JoshutoResult {
    new_local_state(context, FileOperation::Symlink { relative: true });
    Ok(())
}

pub fn paste(context: &mut AppContext, options: FileOperationOptions) -> JoshutoResult {
    match context.take_local_state() {
        Some(state) if !state.paths.is_empty() => {
            let dest = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
            let worker_thread = IoWorkerThread::new(state.file_op, state.paths, dest, options);
            context.worker_context_mut().push_worker(worker_thread);
            Ok(())
        }
        _ => Err(JoshutoError::new(
            JoshutoErrorKind::Io(io::ErrorKind::InvalidData),
            "no files selected".to_string(),
        )),
    }
}

pub fn copy_filename(context: &mut AppContext) -> JoshutoResult {
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

pub fn copy_filename_without_extension(context: &mut AppContext) -> JoshutoResult {
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

pub fn copy_filepath(context: &mut AppContext, all: bool) -> JoshutoResult {
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

pub fn copy_dirpath(context: &mut AppContext) -> JoshutoResult {
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

fn copy_string_to_buffer(string: String) -> JoshutoResult {
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
    Err(JoshutoError::new(
        JoshutoErrorKind::ClipboardError,
        "Failed to copy to clipboard".to_string(),
    ))
}
