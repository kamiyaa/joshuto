use std::path;

#[derive(Clone, Copy, Debug)]
pub enum FileOperation {
    Cut,
    Copy,
    Symlink { relative: bool },
    Delete,
}

impl FileOperation {
    pub fn actioning_str(&self) -> &'static str {
        match *self {
            Self::Cut => "Moving",
            Self::Copy => "Copying",
            Self::Symlink { .. } => "Symlinking",
            Self::Delete => "Deleting",
        }
    }
    pub fn actioned_str(&self) -> &'static str {
        match *self {
            Self::Cut => "moved",
            Self::Copy => "copied",
            Self::Symlink { .. } => "symlinked",
            Self::Delete => "deleted",
        }
    }
}

impl std::fmt::Display for FileOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Cut => write!(f, "Cut"),
            Self::Copy => write!(f, "Copy"),
            Self::Symlink { relative } => write!(f, "Symlink --relative={}", relative),
            Self::Delete => write!(f, "Delete"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FileOperationOptions {
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
