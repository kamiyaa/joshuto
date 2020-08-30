use crate::context::JoshutoContext;
use crate::history::DirectoryHistory;
use crate::io::FileOp;

use super::format;

pub fn process_worker_progress(context: &mut JoshutoContext, res: (FileOp, u64)) {
    let (file_op, progress) = res;
    let prog_str = format::file_size_to_string(progress);
    match file_op {
        FileOp::Cut => {
            context.set_worker_msg(format!("{} moved", prog_str));
        }
        FileOp::Copy => {
            context.set_worker_msg(format!("{} copied", prog_str));
        }
    }
}

pub fn process_finished_worker(
    context: &mut JoshutoContext,
    res: (FileOp, Result<u64, std::io::Error>),
) {
    let (file_op, status) = res;
    let observer = context.remove_job().unwrap();
    let options = context.config_t.sort_option.clone();
    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().reload(observer.get_dest_path(), &options);
        tab.history_mut().reload(observer.get_src_path(), &options);
    }
    match status {
        Ok(p) => {
            let msg = match file_op {
                FileOp::Copy => format!(
                    "copied {} to {:?}",
                    format::file_size_to_string(p),
                    observer.get_dest_path()
                ),
                FileOp::Cut => format!(
                    "moved {} to {:?}",
                    format::file_size_to_string(p),
                    observer.get_dest_path()
                ),
            };
            context.push_msg(msg);
        }
        Err(e) => {
            let msg = format!("{}", e);
            context.push_msg(msg);
        }
    }
    observer.join();
}
