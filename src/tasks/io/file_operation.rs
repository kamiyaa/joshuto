use std::path;

/// The kind of background file operation being performed.
#[derive(Clone, Copy, Debug)]
pub enum FileOperation {
    Cut,
    Copy,
    Delete,
    Symlink,
}

impl FileOperation {
    /// Returns the operation's name in imperative form (e.g. `"Cut"`).
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Cut => "Cut",
            Self::Copy => "Copy",
            Self::Delete => "Delete",
            Self::Symlink => "Symlink",
        }
    }

    /// Returns the operation's name in present-progressive form (e.g. `"Moving"`), for progress
    /// messages.
    pub fn actioning_str(&self) -> &'static str {
        match *self {
            Self::Cut => "Moving",
            Self::Copy => "Copying",
            Self::Delete => "Deleting",
            Self::Symlink => "Symlinking",
        }
    }
    /// Returns the operation's name in past-tense form (e.g. `"moved"`), for completion messages.
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

/// Flags controlling how a file operation behaves, applicable subset depending on `kind`.
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

/// Running progress of a background file operation, used to render the task/progress views.
#[derive(Clone, Debug)]
pub struct FileOperationProgress {
    pub kind: FileOperation,
    pub current_file: path::PathBuf,
    pub files_processed: usize,
    pub total_files: usize,
    pub bytes_processed: u64,
    pub total_bytes: u64,
}
