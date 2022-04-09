use std::io;
use std::process::Command;

use crate::context::{AppContext, LocalStateContext};
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::io::FileOp;

use crate::io::{IoWorkerOptions, IoWorkerThread};

pub fn cut(context: &mut AppContext) -> JoshutoResult {
    if let Some(list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        let selected = list.get_selected_paths();

        let mut local_state = LocalStateContext::new();
        local_state.set_paths(selected.into_iter());
        local_state.set_file_op(FileOp::Cut);

        context.set_local_state(local_state);
    }
    Ok(())
}

pub fn copy(context: &mut AppContext) -> JoshutoResult {
    if let Some(list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        let selected = list.get_selected_paths();

        let mut local_state = LocalStateContext::new();
        local_state.set_paths(selected.into_iter());
        local_state.set_file_op(FileOp::Copy);

        context.set_local_state(local_state);
    }
    Ok(())
}

pub fn paste(context: &mut AppContext, options: IoWorkerOptions) -> JoshutoResult {
    match context.take_local_state() {
        Some(state) if !state.paths.is_empty() => {
            let dest = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
            let worker_thread = IoWorkerThread::new(state.file_op, state.paths, dest, options);
            context.worker_context_mut().push(worker_thread);
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
        .map(|entry| match entry.file_name().rsplit_once('.') {
            Some((name, _)) => name.to_string(),
            _ => entry.file_name().to_string(),
        });

    if let Some(file_name) = entry_file_name {
        copy_string_to_buffer(file_name)?;
    }
    Ok(())
}

pub fn copy_filepath(context: &mut AppContext) -> JoshutoResult {
    let entry_file_path = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|c| c.curr_entry_ref())
        .and_then(|entry| entry.file_path().to_str())
        .map(|s| s.to_string());

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

    if let Some(pathbuf) = opt_entry {
        if let Some(dir) = pathbuf.to_str().map(String::from) {
            copy_string_to_buffer(dir)?
        }
    };
    Ok(())
}

fn copy_string_to_buffer(string: String) -> JoshutoResult {
    let clipboards = [
        (
            "wl-copy",
            format!("printf '%s' '{}' | {} 2> /dev/null", string, "wl-copy"),
        ),
        (
            "xsel",
            format!("printf '%s' '{}' | {} -ib 2> /dev/null", string, "xsel"),
        ),
        (
            "xclip",
            format!(
                "printf '%s' '{}' | {} -selection clipboard 2> /dev/null",
                string, "xclip"
            ),
        ),
    ];

    for (_, command) in clipboards.iter() {
        match Command::new("sh").args(&["-c", command.as_str()]).status() {
            Ok(s) if s.success() => return Ok(()),
            _ => {}
        }
    }
    Err(JoshutoError::new(
        JoshutoErrorKind::ClipboardError,
        "Failed to copy to clipboard".to_string(),
    ))
}
