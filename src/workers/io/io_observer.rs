use std::path;
use std::thread;

use crate::utils::format;
use crate::workers::io::FileOperationProgress;

use super::IoWorkerProgressMessage;

#[derive(Debug)]
pub struct IoWorkerObserver {
    pub handle: thread::JoinHandle<()>,
    pub progress: FileOperationProgress,
    pub msg: String,
    pub src: path::PathBuf,
    pub dest: path::PathBuf,
}

impl IoWorkerObserver {
    pub fn new(
        handle: thread::JoinHandle<()>,
        progress: FileOperationProgress,
        src: path::PathBuf,
        dest: path::PathBuf,
    ) -> Self {
        let msg = generate_worker_msg(&progress);
        Self {
            handle,
            progress,
            dest,
            src,
            msg,
        }
    }

    pub fn join(self) -> bool {
        self.handle.join().is_ok()
    }

    pub fn process_msg(&mut self, msg: IoWorkerProgressMessage) {
        match msg {
            IoWorkerProgressMessage::FileStart { file_path } => {
                self.progress.current_file = file_path;
            }
            IoWorkerProgressMessage::FileComplete { file_size } => {
                self.progress.bytes_processed += file_size;
                self.progress.files_processed += 1;
            }
        }
    }

    pub fn update_msg(&mut self) {
        self.msg = generate_worker_msg(&self.progress);
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

pub fn generate_worker_msg(progress: &FileOperationProgress) -> String {
    let op_str = progress.kind.actioning_str();
    let processed_size = format::file_size_to_string(progress.bytes_processed);
    let total_size = format::file_size_to_string(progress.total_bytes);

    format!(
        "{} ({}/{}) ({}/{}) completed",
        op_str,
        progress.files_processed + 1,
        progress.total_files,
        processed_size,
        total_size,
    )
}
