use std::iter::Iterator;
use std::path;

use crate::io::FileOperation;

pub struct LocalStateState {
    pub paths: Vec<path::PathBuf>,
    pub file_op: FileOperation,
}

impl LocalStateState {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            file_op: FileOperation::Copy,
        }
    }

    pub fn set_file_op(&mut self, operation: FileOperation) {
        self.file_op = operation;
    }

    pub fn set_paths<I>(&mut self, vals: I)
    where
        I: Iterator<Item = path::PathBuf>,
    {
        self.paths = vals.collect();
    }
}
