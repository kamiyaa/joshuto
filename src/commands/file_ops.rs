use std::process::Command;

use crate::context::{JoshutoContext, LocalStateContext};
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::io::FileOp;

use crate::io::{IOWorkerOptions, IOWorkerThread};

pub fn cut(context: &mut JoshutoContext) -> JoshutoResult<()> {
    if let Some(list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        let selected = list.get_selected_paths();

        let mut local_state = LocalStateContext::new();
        local_state.set_paths(selected.into_iter());
        local_state.set_file_op(FileOp::Cut);

        context.set_local_state(local_state);
    }
    Ok(())
}

pub fn copy(context: &mut JoshutoContext) -> JoshutoResult<()> {
    if let Some(list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        let selected = list.get_selected_paths();

        let mut local_state = LocalStateContext::new();
        local_state.set_paths(selected.into_iter());
        local_state.set_file_op(FileOp::Copy);

        context.set_local_state(local_state);
    }
    Ok(())
}

pub fn paste(context: &mut JoshutoContext, options: IOWorkerOptions) -> JoshutoResult<()> {
    match context.take_local_state() {
        Some(state) if !state.paths.is_empty() => {
            let dest = context.tab_context_ref().curr_tab_ref().pwd().to_path_buf();
            let worker_thread = IOWorkerThread::new(state.file_op, state.paths, dest, options);
            context.add_worker(worker_thread);
            Ok(())
        }
        _ => Err(JoshutoError::new(
            JoshutoErrorKind::IOInvalidData,
            "no files selected".to_string(),
        )),
    }
}

pub fn copy_filename(context: &mut JoshutoContext) -> JoshutoResult<()> {
    let entry_file_name = match context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|c| c.curr_entry_ref())
    {
        Some(entry) => Some(entry.file_name().to_string()),
        None => None,
    };
    if let Some(file_name) = entry_file_name {
        let clipboards = [
            (
                "wl-copy",
                format!("printf '%s' {} | {} 2> /dev/null", file_name, "wl-copy"),
            ),
            (
                "xsel",
                format!("printf '%s' {} | {} -ib 2> /dev/null", file_name, "xsel"),
            ),
            (
                "xclip",
                format!(
                    "printf '%s' {} | {} -selection clipboard 2> /dev/null",
                    file_name, "xclip"
                ),
            ),
        ];

        for (clipboard, command) in clipboards.iter() {
            match Command::new("sh").args(&["-c", command.as_str()]).status() {
                Ok(s) if s.success() => return Ok(()),
                _ => {}
            }
        }
        let err = Err(JoshutoError::new(
            JoshutoErrorKind::ClipboardError,
            "Failed to copy to clipboard".to_string(),
        ));
        return err;
    }
    Ok(())
}
