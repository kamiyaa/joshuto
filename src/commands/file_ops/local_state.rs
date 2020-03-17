use lazy_static::lazy_static;

use std::path;
use std::sync::{atomic, Mutex};

use crate::fs::JoshutoDirList;

lazy_static! {
    static ref SELECTED_FILES: Mutex<Option<Vec<path::PathBuf>>> = Mutex::new(None);
    static ref FILE_OPERATION: Mutex<FileOp> = Mutex::new(FileOp::Copy);
    static ref TAB_SRC: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
}

#[derive(Clone, Debug)]
pub enum FileOp {
    Cut,
    Copy,
}

pub struct LocalState;

impl LocalState {
    pub fn set_file_op(operation: FileOp) {
        let mut data = FILE_OPERATION.lock().unwrap();
        *data = operation;
    }

    pub fn set_tab_src(tab_index: usize) {
        TAB_SRC.store(tab_index, atomic::Ordering::Release);
    }

    pub fn repopulated_selected_files(dirlist: &JoshutoDirList) -> std::io::Result<()> {
        let selected = dirlist.get_selected_paths();
        if selected.is_empty() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no files selected",
            ))
        } else {
            let selected_clone: Vec<path::PathBuf> =
                selected.iter().map(|p| (*p).clone()).collect();
            let mut data = SELECTED_FILES.lock().unwrap();
            *data = Some(selected_clone);
            Ok(())
        }
    }

    pub fn take_selected_files() -> Option<Vec<path::PathBuf>> {
        SELECTED_FILES.lock().unwrap().take()
    }

    pub fn get_file_operation() -> FileOp {
        (*FILE_OPERATION.lock().unwrap()).clone()
    }
}
