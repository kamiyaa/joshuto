use lazy_static::lazy_static;
use std::path;
use std::sync::{atomic, mpsc, Mutex};
use std::thread;
use std::time;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ProgressInfo};
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

struct LocalState;

impl LocalState {
    pub fn set_file_op(operation: FileOp) {
        let mut data = FILE_OPERATION.lock().unwrap();
        *data = operation;
    }

    pub fn set_tab_src(tab_index: usize) {
        TAB_SRC.store(tab_index, atomic::Ordering::Release);
    }

    pub fn repopulated_selected_files(dirlist: &JoshutoDirList) -> bool {
        let mut data = SELECTED_FILES.lock().unwrap();
        match dirlist.get_selected_paths() {
            Some(s) => {
                *data = Some(s);
                true
            }
            None => false,
        }
    }
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
        if LocalState::repopulated_selected_files(&curr_tab.curr_list) {
            LocalState::set_file_op(FileOp::Cut);
            LocalState::set_tab_src(context.curr_tab_index);
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
        if LocalState::repopulated_selected_files(&curr_tab.curr_list) {
            LocalState::set_file_op(FileOp::Copy);
            LocalState::set_tab_src(context.curr_tab_index);
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
            ncurses::timeout(0);
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

    fn cut_paste(
        &self,
        context: &mut JoshutoContext,
    ) -> Result<FileOperationThread, std::io::Error> {
        let paths = SELECTED_FILES.lock().unwrap().take();
        match paths {
            Some(paths) => {
                if paths.len() == 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "no files selected",
                    ));
                }

                let tab_src = TAB_SRC.load(atomic::Ordering::SeqCst);
                let tab_dest = context.curr_tab_index;
                let destination = context.tabs[tab_dest].curr_path.clone();

                let options = self.options.clone();

                let (tx, rx) = mpsc::channel();

                let handle = thread::spawn(move || fs_cut_thread(options, tx, destination, paths));

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
    ) -> Result<FileOperationThread, std::io::Error> {
        let tab_dest = context.curr_tab_index;
        let destination = context.tabs[tab_dest].curr_path.clone();

        let tab_src = TAB_SRC.load(atomic::Ordering::SeqCst);
        let options = self.options.clone();

        let (tx, rx) = mpsc::channel();

        let paths = SELECTED_FILES.lock().unwrap().take();
        match paths {
            Some(paths) => {
                if paths.len() == 0 {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "no files selected",
                    ))
                } else {
                    let handle =
                        thread::spawn(move || fs_copy_thread(options, tx, destination, paths));

                    let thread = FileOperationThread {
                        tab_src,
                        tab_dest,
                        handle,
                        recv: rx,
                    };
                    Ok(thread)
                }
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no files selected",
            )),
        }
    }
}

fn fs_cut_thread(
    options: fs_extra::dir::CopyOptions,
    tx: mpsc::Sender<ProgressInfo>,
    dest: path::PathBuf,
    paths: Vec<path::PathBuf>,
) -> std::result::Result<(), std::io::Error> {
    let mut progress_info = ProgressInfo {
        bytes_finished: 4,
        total_bytes: paths.len() as u64 + 4,
    };

    let mut destination = dest;

    for path in paths {
        let file_name = path.file_name().unwrap().to_os_string();

        destination.push(file_name.clone());
        if !options.skip_exist {
            let mut i = 0;
            while destination.exists() {
                destination.pop();

                let mut file_name = file_name.clone();
                file_name.push(&format!("_{}", i));

                destination.push(file_name);
                i += 1;
            }
        }
        match std::fs::rename(&path, &destination) {
            Ok(_) => {}
            Err(_) => {
                if path.symlink_metadata()?.is_dir() {
                    std::fs::create_dir(&destination)?;
                    let cpath: Vec<path::PathBuf> = std::fs::read_dir(&path)?
                        .filter_map(|s| match s {
                            Ok(s) => Some(s.path()),
                            _ => None,
                        })
                        .collect();

                    let handle = |process_info: fs_extra::TransitProcess| {
                        let progress_info = ProgressInfo {
                                bytes_finished: process_info.copied_bytes,
                                total_bytes: process_info.total_bytes,
                            };
                        tx.send(progress_info.clone()).unwrap();
                        fs_extra::dir::TransitProcessResult::ContinueOrAbort
                    };

                    match fs_extra::move_items_with_progress(&cpath, &destination, &options, handle) {
                        Err(e) => {
                            let err =
                                std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e));
                            return Err(err);
                        }
                        _ => {}
                    }
                    std::fs::remove_dir_all(&path)?;
                } else {
                    std::fs::copy(&path, &destination)?;
                    std::fs::remove_file(&path)?;
                }
            }
        }
        destination.pop();
        progress_info.bytes_finished += 1;
        tx.send(progress_info.clone()).unwrap();
    }
    return Ok(());
}

fn fs_copy_thread(
    options: fs_extra::dir::CopyOptions,
    tx: mpsc::Sender<ProgressInfo>,
    dest: path::PathBuf,
    paths: Vec<path::PathBuf>,
) -> std::result::Result<(), std::io::Error> {
    let mut progress_info = ProgressInfo {
        bytes_finished: 1,
        total_bytes: paths.len() as u64 + 1,
    };

    let mut destination = dest;

    for path in &paths {
        let file_name = path.file_name().unwrap().to_os_string();

        if path.symlink_metadata()?.is_dir() {
            destination.push(file_name.clone());
            if !options.skip_exist {
                let mut i = 0;
                while destination.exists() {
                    destination.pop();

                    let mut file_name = file_name.clone();
                    file_name.push(&format!("_{}", i));

                    destination.push(file_name);
                    i += 1;
                }
            }
            std::fs::create_dir(&destination)?;
            let path: Vec<path::PathBuf> = std::fs::read_dir(path)?
                .filter_map(|s| match s {
                    Ok(s) => Some(s.path()),
                    _ => None,
                })
                .collect();

            let handle = |process_info: fs_extra::TransitProcess| {
                let progress_info = ProgressInfo {
                        bytes_finished: process_info.copied_bytes,
                        total_bytes: process_info.total_bytes,
                    };
                tx.send(progress_info.clone()).unwrap();
                fs_extra::dir::TransitProcessResult::ContinueOrAbort
            };

            match fs_extra::copy_items_with_progress(&path, &destination, &options, handle) {
                Err(e) => {
                    let err = std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e));
                    return Err(err);
                }
                _ => {}
            }
        } else {
            destination.push(file_name.clone());
            if !options.skip_exist {
                let mut i = 0;
                while destination.exists() {
                    destination.pop();

                    let mut file_name = file_name.clone();
                    file_name.push(&format!("_{}", i));

                    destination.push(file_name);
                    i += 1;
                }
            }
            std::fs::copy(&path, &destination)?;
        }
        destination.pop();
        progress_info.bytes_finished += 1;
        tx.send(progress_info.clone()).unwrap();
    }
    return Ok(());
}
