use std::iter::Iterator;
use std::path;

use crate::types::io::FileOperation;

/// A pending cut/copy/symlink selection, stashed until a `paste_files` command applies it.
#[derive(Clone, Debug)]
pub struct LocalStateState {
    pub paths: Vec<path::PathBuf>,
    pub file_op: FileOperation,
}

impl LocalStateState {
    /// Creates an empty pending selection defaulting to a copy operation.
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            file_op: FileOperation::Copy,
        }
    }

    /// Sets which operation (cut/copy/symlink) will be applied on paste.
    pub fn set_file_op(&mut self, operation: FileOperation) {
        self.file_op = operation;
    }

    /// Replaces the pending selection's paths.
    pub fn set_paths<I>(&mut self, vals: I)
    where
        I: Iterator<Item = path::PathBuf>,
    {
        self.paths = vals.collect();
    }
}
