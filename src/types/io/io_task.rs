use std::path;

use crate::utils::format;

use super::{FileOperation, FileOperationOptions, FileOperationProgress};

#[derive(Clone, Debug)]
pub struct IoTask {
    pub operation: FileOperation,
    pub options: FileOperationOptions,
    pub paths: Vec<path::PathBuf>,
    pub dest: path::PathBuf,
}

impl IoTask {
    pub fn new(
        operation: FileOperation,
        paths: Vec<path::PathBuf>,
        dest: path::PathBuf,
        options: FileOperationOptions,
    ) -> Self {
        Self {
            operation,
            options,
            paths,
            dest,
        }
    }

    pub fn get_operation_type(&self) -> FileOperation {
        self.operation
    }
}

#[derive(Debug)]
pub struct IoTaskProgress {
    pub progress: FileOperationProgress,
    pub msg: String,
    pub src: path::PathBuf,
    pub dest: path::PathBuf,
}

impl IoTaskProgress {
    pub fn new(progress: FileOperationProgress, src: path::PathBuf, dest: path::PathBuf) -> Self {
        let msg = generate_worker_msg(&progress);
        Self {
            progress,
            dest,
            src,
            msg,
        }
    }

    pub fn process_msg(&mut self, msg: IoTaskProgressMessage) {
        match msg {
            IoTaskProgressMessage::FileStart { file_path } => {
                self.progress.current_file = file_path;
            }
            IoTaskProgressMessage::FileComplete { file_size } => {
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

#[derive(Clone, Debug)]
pub enum IoTaskProgressMessage {
    FileStart { file_path: path::PathBuf },
    FileComplete { file_size: u64 },
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
