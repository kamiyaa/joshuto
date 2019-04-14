use lazy_static::lazy_static;
use std::path;
use std::sync::{atomic, mpsc, Mutex};
use std::thread;
use std::time;

use crate::commands::{self, JoshutoCommand, JoshutoRunnable, ProgressInfo};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::structs::JoshutoDirList;
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

#[derive(Clone, Debug)]
pub struct CopyOptions {
    pub overwrite: bool,
    pub skip_exist: bool,
}

pub struct FileOperationThread {
    pub tab_src: usize,
    pub tab_dest: usize,
    pub handle: thread::JoinHandle<Result<(), std::io::Error>>,
    pub recv: mpsc::Receiver<ProgressInfo>,
}

impl FileOperationThread {
    pub fn recv_timeout(
        &self,
        wait_duration: &time::Duration,
    ) -> Result<ProgressInfo, mpsc::RecvTimeoutError> {
        self.recv.recv_timeout(*wait_duration)
    }
}

fn set_file_op(operation: FileOp) {
    let mut data = FILE_OPERATION.lock().unwrap();
    *data = operation;
}

fn set_tab_src(tab_index: usize) {
    TAB_SRC.store(tab_index, atomic::Ordering::Release);
}

fn repopulated_selected_files(dirlist: &JoshutoDirList) -> bool {
    match commands::collect_selected_paths(dirlist) {
        Some(s) => {
            let mut data = SELECTED_FILES.lock().unwrap();
            *data = Some(s);
            true
        }
        None => false,
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
    fn execute(&self, context: &mut JoshutoContext, _: &JoshutoView) -> Result<(), JoshutoError> {
        let curr_tab = context.curr_tab_ref();
        if let Some(s) = curr_tab.curr_list.as_ref() {
            if repopulated_selected_files(s) {
                set_file_op(FileOp::Cut);
                set_tab_src(context.curr_tab_index);
            }
        }
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
    fn execute(&self, context: &mut JoshutoContext, _: &JoshutoView) -> Result<(), JoshutoError> {
        let curr_tab = context.curr_tab_ref();
        if let Some(s) = curr_tab.curr_list.as_ref() {
            if repopulated_selected_files(s) {
                set_file_op(FileOp::Copy);
                set_tab_src(context.curr_tab_index);
            }
        }
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
    fn execute(&self, context: &mut JoshutoContext, _: &JoshutoView) -> Result<(), JoshutoError> {
        let file_operation = FILE_OPERATION.lock().unwrap();

        let thread = match *file_operation {
            FileOp::Copy => self.copy_paste(context),
            FileOp::Cut => self.cut_paste(context),
        };

        if let Ok(s) = thread {
            ncurses::timeout(-1);
            context.threads.push(s);
        }
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

    fn same_fs_cut(
        &self,
        context: &mut JoshutoContext,
    ) -> Result<FileOperationThread, std::io::Error> {
        let options = self.options.clone();

        let (tx, rx) = mpsc::channel();

        let tab_dest = context.curr_tab_index;
        let tab_src = TAB_SRC.load(atomic::Ordering::SeqCst);
        let mut destination = context.tabs[tab_dest].curr_path.clone();

        let handle = thread::spawn(move || {
            let paths: Option<Vec<path::PathBuf>> = SELECTED_FILES.lock().unwrap().take();
            match paths {
                None => {}
                Some(s) => {
                    let mut progress_info = ProgressInfo {
                        bytes_finished: 1,
                        total_bytes: s.len() as u64 + 1,
                    };

                    for path in &s {
                        let file_name = path.file_name().unwrap().to_os_string();

                        destination.push(file_name.clone());
                        if destination.exists() {
                            if !options.skip_exist {
                                for i in 0.. {
                                    if !destination.exists() {
                                        break;
                                    }
                                    destination.pop();
                                    let mut file_name = file_name.clone();
                                    file_name.push(&format!("_{}", i));
                                    destination.push(file_name);
                                }
                            }
                        }
                        std::fs::rename(&path, &destination)?;
                        destination.pop();

                        progress_info.bytes_finished += 1;
                        tx.send(progress_info.clone()).unwrap();
                    }
                }
            }
            return Ok(());
        });

        let thread = FileOperationThread {
            tab_src,
            tab_dest,
            handle,
            recv: rx,
        };
        Ok(thread)
    }

    #[cfg(target_os = "linux")]
    fn cut_paste(
        &self,
        context: &mut JoshutoContext,
    ) -> Result<FileOperationThread, std::io::Error> {
        use std::os::linux::fs::MetadataExt;

        let src_ino;
        {
            let paths = SELECTED_FILES.lock().unwrap();
            match *paths {
                Some(ref s) => {
                    if s.len() == 0 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "no files selected",
                        ));
                    }
                    src_ino = s[0].metadata()?.st_dev();
                }
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "no files selected",
                    ));
                }
            }
        }

        let tab_dest = context.curr_tab_index;
        let destination = &context.tabs[tab_dest].curr_path;

        let dest_ino = destination.metadata()?.st_dev();
        if dest_ino == src_ino {
            self.same_fs_cut(context)
        } else {
            self.same_fs_cut(context)
        }
    }

    fn copy_paste(
        &self,
        context: &mut JoshutoContext,
    ) -> Result<FileOperationThread, std::io::Error> {
        let tab_dest = context.curr_tab_index;
        let destination = context.tabs[tab_dest].curr_path.clone();

        let tab_src = TAB_SRC.load(atomic::Ordering::SeqCst);
        let options = self.options.clone();

        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let paths = SELECTED_FILES.lock().unwrap();
            match *paths {
                Some(ref s) => {
                    let progress_info = ProgressInfo {
                        bytes_finished: 0,
                        total_bytes: 0,
                    };
                    match tx.send(progress_info) {
                        Ok(_) => {}
                        Err(e) => {}
                    }
                    return Ok(());
                }
                None => return Ok(()),
            }
        });
        let thread = FileOperationThread {
            tab_src,
            tab_dest,
            handle,
            recv: rx,
        };
        Ok(thread)
    }
}
