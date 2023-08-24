use std::path;
use std::thread;

use crate::io::FileOperationProgress;
use crate::util::format;

#[derive(Debug)]
pub struct IoWorkerObserver {
    pub handle: thread::JoinHandle<()>,
    pub progress: Option<FileOperationProgress>,
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
        self.handle.join().is_ok()
    }
    pub fn set_progress(&mut self, progress: FileOperationProgress) {
        self.progress = Some(progress);
    }
    pub fn update_msg(&mut self) {
        match self.progress.as_ref() {
            None => {}
            Some(progress) => {
                let op_str = progress.kind().actioning_str();
                let processed_size = format::file_size_to_string(progress.bytes_processed());
                let total_size = format::file_size_to_string(progress.total_bytes());

                let msg = format!(
                    "{} ({}/{}) ({}/{}) completed",
                    op_str,
                    progress.files_processed() + 1,
                    progress.total_files(),
                    processed_size,
                    total_size,
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
