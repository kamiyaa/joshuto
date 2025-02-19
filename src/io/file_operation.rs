use std::path;

#[derive(Clone, Copy, Debug)]
pub enum FileOperation {
    Cut,
    Copy,
    Delete,
    Symlink,
}

impl FileOperation {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Cut => "Cut",
            Self::Copy => "Copy",
            Self::Delete => "Delete",
            Self::Symlink => "Symlink",
        }
    }

    pub fn actioning_str(&self) -> &'static str {
        match *self {
            Self::Cut => "Moving",
            Self::Copy => "Copying",
            Self::Delete => "Deleting",
            Self::Symlink => "Symlinking",
        }
    }
    pub fn actioned_str(&self) -> &'static str {
        match *self {
            Self::Cut => "moved",
            Self::Copy => "copied",
            Self::Delete => "deleted",
            Self::Symlink => "symlinked",
        }
    }
}

impl std::fmt::Display for FileOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FileOperationOptions {
    // symlink
    pub _symlink: bool,
    pub symlink_relative: bool,

    // cut, copy
    pub overwrite: bool,
    pub skip_exist: bool,

    // delete
    pub permanently: bool,
}

impl std::fmt::Display for FileOperationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "overwrite={} skip_exist={}",
            self.overwrite, self.skip_exist
        )
    }
}

#[derive(Clone, Debug)]
pub struct FileOperationProgress {
    pub kind: FileOperation,
    pub current_file: path::PathBuf,
    pub files_processed: usize,
    pub total_files: usize,
    pub bytes_processed: u64,
    pub total_bytes: u64,
}
