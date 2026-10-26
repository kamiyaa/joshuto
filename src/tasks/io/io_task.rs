use std::path;

use crate::utils::format;

use super::{FileOperation, FileOperationOptions, FileOperationProgress};

/// A queued file operation (cut/copy/delete/symlink) awaiting execution on a background thread.
#[derive(Clone, Debug)]
pub struct IoTask {
    pub operation: FileOperation,
    pub options: FileOperationOptions,
    pub paths: Vec<path::PathBuf>,
    pub dest: path::PathBuf,
}

impl IoTask {
    /// Builds a task to perform `operation` on `paths`, writing results to `dest`.
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

    /// Returns the kind of operation this task performs.
    pub fn get_operation_type(&self) -> FileOperation {
        self.operation
    }
}

/// The current status of a running [`IoTask`], including a display-ready progress message.
#[derive(Debug)]
pub struct IoTaskStat {
    pub progress: FileOperationProgress,
    pub msg: String,
    pub src: path::PathBuf,
    pub dest: path::PathBuf,
}

impl IoTaskStat {
    /// Builds a new stat tracker at zero progress for a task moving `src` to `dest`.
    pub fn new(progress: FileOperationProgress, src: path::PathBuf, dest: path::PathBuf) -> Self {
        let msg = generate_worker_msg(&progress);
        Self {
            progress,
            dest,
            src,
            msg,
        }
    }

    /// Applies a progress update from the worker thread and refreshes the display message.
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

    /// Regenerates the display message from the current progress.
    pub fn update_msg(&mut self) {
        self.msg = generate_worker_msg(&self.progress);
    }
    /// Returns the current display-ready progress message.
    pub fn get_msg(&self) -> &str {
        self.msg.as_str()
    }
    /// Returns the source path of this task.
    pub fn src_path(&self) -> &path::Path {
        self.src.as_path()
    }
    /// Returns the destination path of this task.
    pub fn dest_path(&self) -> &path::Path {
        self.dest.as_path()
    }
}

/// A progress update sent from a background file-operation worker thread.
#[derive(Clone, Debug)]
pub enum IoTaskProgressMessage {
    FileStart { file_path: path::PathBuf },
    FileComplete { file_size: u64 },
}

/// Formats a human-readable progress message like `"Copying (3/10) (1.2MB/5MB) completed"`.
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
