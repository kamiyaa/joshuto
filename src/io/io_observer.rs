use std::path;
use std::thread;

use crate::io::{FileOp, IoWorkerProgress};
use crate::util::format;

#[derive(Debug)]
pub struct IoWorkerObserver {
    pub handle: thread::JoinHandle<()>,
    pub progress: Option<IoWorkerProgress>,
    msg: String,
    src: path::PathBuf,
    dest: path::PathBuf,
}

impl IoWorkerObserver {
    pub fn new(handle: thread::JoinHandle<()>, src: path::PathBuf, dest: path::PathBuf) -> Self {
        Self {
            handle,
            progress: None,
            src,
            dest,
            msg: String::new(),
        }
    }

    pub fn join(self) -> bool {
        matches!(self.handle.join(), Ok(_))
    }
    pub fn set_progress(&mut self, progress: IoWorkerProgress) {
        self.progress = Some(progress);
    }
    pub fn update_msg(&mut self) {
        match self.progress.as_ref() {
            None => {}
            Some(progress) => {
                let size_str = format::file_size_to_string(progress.processed());
                let op_str = match progress.kind() {
                    FileOp::Cut => "Moving",
                    FileOp::Copy => "Copying",
                };

                let msg = format!(
                    "{} ({}/{}) {} completed",
                    op_str,
                    progress.index() + 1,
                    progress.len(),
                    size_str
                );
                self.msg = msg;
            }
        }
    }
    pub fn get_msg(&self) -> &str {
        self.msg.as_str()
    }
    pub fn src_path(&self) -> &path::Path {
        self.src.as_path()
    }
    pub fn dest_path(&self) -> &path::Path {
        self.dest.as_path()
    }
}
