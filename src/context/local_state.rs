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

    pub fn set_paths<'a, I>(&mut self, vals: I)
    where
        I: Iterator<Item = &'a path::Path>,
    {
        self.paths = vals.map(|p| p.to_path_buf()).collect();
    }
}
