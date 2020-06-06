use lazy_static::lazy_static;

use crate::fs::JoshutoDirList;
use crate::io::FileOp;
use std::path;
use std::sync::Mutex;

lazy_static! {
    static ref LOCAL_STATE: Mutex<LocalState> = Mutex::new(LocalState::new());
}

pub struct LocalState {
    pub paths: Vec<path::PathBuf>,
    pub file_op: FileOp,
}

impl LocalState {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            file_op: FileOp::Copy,
        }
    }

    pub fn set_file_op(operation: FileOp) {
        let mut data = LOCAL_STATE.lock().unwrap();
        (*data).file_op = operation;
    }

    pub fn repopulate(dirlist: &JoshutoDirList) -> std::io::Result<()> {
        let selected = dirlist.get_selected_paths();
        if selected.is_empty() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no files selected",
            ))
        } else {
            let selected_clone: Vec<path::PathBuf> =
                selected.iter().map(|p| (*p).clone()).collect();
            let mut data = LOCAL_STATE.lock().unwrap();
            (*data).paths = selected_clone;
            Ok(())
        }
    }

    pub fn take() -> LocalState {
        let mut m = LOCAL_STATE.lock().unwrap();
        let mut v = LocalState::new();
        std::mem::swap(&mut (*m), &mut v);
        v
    }
}
