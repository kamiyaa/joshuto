use crate::io::FileOp;

use std::iter::Iterator;
use std::path;

pub struct LocalStateContext {
    pub paths: Vec<path::PathBuf>,
    pub file_op: FileOp,
}

impl LocalStateContext {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            file_op: FileOp::Copy,
        }
    }

    pub fn set_file_op(&mut self, operation: FileOp) {
        self.file_op = operation;
    }

    pub fn set_paths<I>(&mut self, vals: I)
    where
        I: Iterator<Item = path::PathBuf>,
    {
        self.paths = vals.collect();
    }
}
