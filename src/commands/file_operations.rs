use lazy_static::lazy_static;
use std::path;
use std::sync::{atomic, mpsc, Mutex};
use std::thread;
use std::time;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::fs::{fs_extra_extra, JoshutoDirList};
use crate::window::JoshutoView;

lazy_static! {
    static ref SELECTED_FILES: Mutex<Option<Vec<path::PathBuf>>> = Mutex::new(None);
    static ref FILE_OPERATION: Mutex<FileOp> = Mutex::new(FileOp::Copy);
    static ref TAB_SRC: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
}

enum FileOp {
    Cut,
    Copy,
}

struct LocalState;

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
}

pub struct FileOperationThread<T, Q> {
    pub tab_src: usize,
    pub tab_dest: usize,
    pub handle: thread::JoinHandle<std::io::Result<T>>,
    pub recv: mpsc::Receiver<Q>,
}

impl<T, Q> FileOperationThread<T, Q> {
    pub fn recv_timeout(
        &self,
        wait_duration: &time::Duration,
    ) -> Result<Q, mpsc::RecvTimeoutError> {
        self.recv.recv_timeout(*wait_duration)
    }
}

#[derive(Clone, Debug)]
pub struct CutFiles;

impl CutFiles {
    pub fn new() -> Self {
        CutFiles
    }
    pub const fn command() -> &'static str {
        "cut_files"
    }
}

impl JoshutoCommand for CutFiles {}

impl std::fmt::Display for CutFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CutFiles {
    fn execute(&self, context: &mut JoshutoContext, _: &JoshutoView) -> JoshutoResult<()> {
        let curr_tab = context.curr_tab_ref();
        LocalState::repopulated_selected_files(&curr_tab.curr_list)?;
        LocalState::set_file_op(FileOp::Cut);
        LocalState::set_tab_src(context.curr_tab_index);
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CopyFiles;

impl CopyFiles {
    pub fn new() -> Self {
        CopyFiles
    }
    pub const fn command() -> &'static str {
        "copy_files"
    }
}

impl JoshutoCommand for CopyFiles {}

impl std::fmt::Display for CopyFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CopyFiles {
    fn execute(&self, context: &mut JoshutoContext, _: &JoshutoView) -> JoshutoResult<()> {
        let curr_tab = context.curr_tab_ref();
        LocalState::repopulated_selected_files(&curr_tab.curr_list)?;
        LocalState::set_file_op(FileOp::Copy);
        LocalState::set_tab_src(context.curr_tab_index);
        Ok(())
    }
}

pub struct PasteFiles {
    options: fs_extra::dir::CopyOptions,
}

impl JoshutoCommand for PasteFiles {}

impl std::fmt::Display for PasteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} overwrite={} skip_exist={}",
            Self::command(),
            self.options.overwrite,
            self.options.skip_exist,
        )
    }
}

impl std::fmt::Debug for PasteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for PasteFiles {
    fn execute(&self, context: &mut JoshutoContext, _: &JoshutoView) -> JoshutoResult<()> {
        let file_operation = FILE_OPERATION.lock().unwrap();

        let thread = match *file_operation {
            FileOp::Copy => self.copy_paste(context),
            FileOp::Cut => self.cut_paste(context),
        }?;

        context.threads.push(thread);
        Ok(())
    }
}

impl PasteFiles {
    pub fn new(options: fs_extra::dir::CopyOptions) -> Self {
        PasteFiles { options }
    }
    pub const fn command() -> &'static str {
        "paste_files"
    }

    fn cut_paste(
        &self,
        context: &mut JoshutoContext,
    ) -> std::io::Result<FileOperationThread<u64, fs_extra::TransitProcess>> {
        let tab_src = TAB_SRC.load(atomic::Ordering::SeqCst);
        let tab_dest = context.curr_tab_index;
        let destination = context.tabs[tab_dest].curr_path.clone();

        let options = self.options.clone();

        let (tx, rx) = mpsc::channel();

        let paths = SELECTED_FILES.lock().unwrap().take();
        match paths {
            Some(paths) => {
                if paths.is_empty() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "no files selected",
                    ));
                }

                let handle = thread::spawn(move || {
                    let progress_handle = |process_info: fs_extra::TransitProcess| {
                        tx.send(process_info);
                        fs_extra::dir::TransitProcessResult::ContinueOrAbort
                    };
                    fs_extra_extra::fs_cut_with_progress(
                        &paths,
                        &destination,
                        options.clone(),
                        progress_handle,
                    )
                });

                let thread = FileOperationThread {
                    tab_src,
                    tab_dest,
                    handle,
                    recv: rx,
                };
                Ok(thread)
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no files selected",
            )),
        }
    }

    fn copy_paste(
        &self,
        context: &mut JoshutoContext,
    ) -> std::io::Result<FileOperationThread<u64, fs_extra::TransitProcess>> {
        let tab_dest = context.curr_tab_index;
        let destination = context.tabs[tab_dest].curr_path.clone();

        let tab_src = TAB_SRC.load(atomic::Ordering::SeqCst);
        let options = self.options.clone();

        let (tx, rx) = mpsc::channel();

        let paths = SELECTED_FILES.lock().unwrap().take();
        match paths {
            Some(paths) => {
                if paths.is_empty() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "no files selected",
                    ));
                }

                let handle = thread::spawn(move || {
                    let progress_handle = |process_info: fs_extra::TransitProcess| {
                        tx.send(process_info);
                        fs_extra::dir::TransitProcessResult::ContinueOrAbort
                    };
                    fs_extra_extra::fs_copy_with_progress(
                        &paths,
                        &destination,
                        options.clone(),
                        progress_handle,
                    )
                });

                let thread = FileOperationThread {
                    tab_src,
                    tab_dest,
                    handle,
                    recv: rx,
                };
                Ok(thread)
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no files selected",
            )),
        }
    }
}
