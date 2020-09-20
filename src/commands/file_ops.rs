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
            let mut options = options;
            options.kind = state.file_op;
            let worker_thread = IOWorkerThread::new(options, state.paths, dest);
            context.add_worker(worker_thread);
            Ok(())
        }
        _ => Err(JoshutoError::new(
            JoshutoErrorKind::IOInvalidData,
            "no files selected".to_string(),
        )),
    }
}
