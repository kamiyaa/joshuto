use crate::context::JoshutoContext;
use crate::history::DirectoryHistory;
use crate::io::{FileOp, IOWorkerProgress};

use super::format;

pub fn process_worker_progress(context: &mut JoshutoContext, res: IOWorkerProgress) {
    let size_str = format::file_size_to_string(res.processed);
    match res.kind {
        FileOp::Cut => {
            let msg = format!("moving ({}/{}) {} completed",
                res.index + 1, res.len, size_str);
            context.set_worker_msg(msg);
        }
        FileOp::Copy => {
            let msg = format!("copying ({}/{}) {} completed",
                res.index + 1, res.len, size_str);
            context.set_worker_msg(msg);
        }
    }
}

pub fn process_finished_worker(
    context: &mut JoshutoContext,
    res: std::io::Result<IOWorkerProgress>,
) {
    let observer = context.remove_job().unwrap();
    let options = context.config_t.sort_option.clone();
    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().reload(observer.get_dest_path(), &options);
        tab.history_mut().reload(observer.get_src_path(), &options);
    }
    match res {
        Ok(progress) => {
            let op = match progress.kind {
                FileOp::Copy => "copied",
                FileOp::Cut => "moved",
            };
            let size_str = format::file_size_to_string(progress.processed);
            let msg = format!("successfully {} {} items ({})",
                op, progress.len, size_str);
            context.push_msg(msg);
        }
        Err(e) => {
            let msg = format!("{}", e);
            context.push_msg(msg);
        }
    }
    observer.join();
}
