use lazy_static::lazy_static;
use std::path;
use std::sync::{atomic, mpsc, Mutex};
use std::thread;
use std::time;

use crate::commands::{self, JoshutoCommand, JoshutoRunnable, ProgressInfo};
use crate::context::JoshutoContext;
use crate::structs::JoshutoDirList;

lazy_static! {
    static ref selected_files: Mutex<Vec<path::PathBuf>> = Mutex::new(vec![]);
    static ref fileop: Mutex<FileOp> = Mutex::new(FileOp::Copy);
    static ref tab_src: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
}

pub struct FileOperationThread {
    pub tab_src: usize,
    pub tab_dest: usize,
    pub handle: thread::JoinHandle<i32>,
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
    let mut data = fileop.lock().unwrap();
    *data = operation;
}

fn set_tab_src(tab_index: usize) {
    tab_src.store(tab_index, atomic::Ordering::Release);
}

fn repopulated_selected_files(dirlist: &JoshutoDirList) -> bool {
    if let Some(contents) = commands::collect_selected_paths(dirlist) {
        let mut data = selected_files.lock().unwrap();
        *data = contents;
        return true;
    }
    false
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
    fn execute(&self, context: &mut JoshutoContext) {
        let curr_tab = context.curr_tab_ref();
        if let Some(s) = curr_tab.curr_list.as_ref() {
            if repopulated_selected_files(s) {
                set_file_op(FileOp::Cut);
                set_tab_src(context.curr_tab_index);
            }
        }
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
    fn execute(&self, context: &mut JoshutoContext) {
        let curr_tab = context.curr_tab_ref();
        if let Some(s) = curr_tab.curr_list.as_ref() {
            if repopulated_selected_files(s) {
                set_file_op(FileOp::Copy);
                set_tab_src(context.curr_tab_index);
            }
        }
    }
}

pub struct PasteFiles {
    options: fs_extra::dir::CopyOptions,
}

impl PasteFiles {
    pub fn new(options: fs_extra::dir::CopyOptions) -> Self {
        PasteFiles { options }
    }
    pub const fn command() -> &'static str {
        "paste_files"
    }

    #[cfg(target_os = "linux")]
    fn cut(&self, context: &mut JoshutoContext) -> Result<FileOperationThread, std::io::Error> {
        use std::os::linux::fs::MetadataExt;

        let tab_dest = context.curr_tab_index;
        let tab_src_index = tab_src.load(atomic::Ordering::SeqCst);

        let mut destination = context.tabs[tab_dest].curr_path.clone();
        let options = self.options.clone();

        let dest_ino = destination.metadata()?.st_dev();
        let path_ino;
        {
            let paths = selected_files.lock().unwrap();
            if paths.len() == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "no files selected",
                ));
            }
            path_ino = paths[0].metadata()?.st_dev();
        }

        let (tx, rx) = mpsc::channel();
        let handle = if dest_ino == path_ino {
            thread::spawn(move || {
                let mut paths = selected_files.lock().unwrap();
                let mut progress_info = ProgressInfo {
                    bytes_finished: 1,
                    total_bytes: paths.len() as u64 + 1,
                };

                for path in (*paths).iter() {
                    let mut file_name = path.file_name().unwrap().to_os_string();

                    if options.skip_exist && destination.exists() {
                        continue;
                    }

                    while path::Path::new(&file_name).exists() {
                        file_name.push("_0");
                    }

                    destination.push(file_name);
                    std::fs::rename(&path, &destination).unwrap();
                    destination.pop();

                    progress_info.bytes_finished += 1;
                    tx.send(progress_info.clone()).unwrap();
                }
                paths.clear();
                0
            })
        } else {
            thread::spawn(move || {
                let mut paths = selected_files.lock().unwrap();

                let handle = |process_info: fs_extra::TransitProcess| {
                    let progress_info = ProgressInfo {
                        bytes_finished: process_info.copied_bytes,
                        total_bytes: process_info.total_bytes,
                    };
                    tx.send(progress_info).unwrap();
                    fs_extra::dir::TransitProcessResult::ContinueOrAbort
                };

                fs_extra::move_items_with_progress(&paths, &destination, &options, handle).unwrap();
                paths.clear();
                0
            })
        };
        let thread = FileOperationThread {
            tab_src: tab_src_index,
            tab_dest,
            handle,
            recv: rx,
        };
        Ok(thread)
    }

    #[cfg(not(target_os = "linux"))]
    fn cut(&self, context: &mut JoshutoContext) -> Result<FileOperationThread, std::io::Error> {
        let tab_dest = context.curr_tab_index;
        let tab_src_index = tab_src.load(atomic::Ordering::SeqCst);

        let mut destination = context.tabs[tab_dest].curr_path.clone();
        let options = self.options.clone();

        {
            let paths = selected_files.lock().unwrap();
            if paths.len() == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "no files selected",
                ));
            }
        }
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut paths = selected_files.lock().unwrap();

            let handle = |process_info: fs_extra::TransitProcess| {
                let progress_info = ProgressInfo {
                    bytes_finished: process_info.copied_bytes,
                    total_bytes: process_info.total_bytes,
                };
                tx.send(progress_info).unwrap();
                fs_extra::dir::TransitProcessResult::ContinueOrAbort
            };

            fs_extra::move_items_with_progress(&paths, &destination, &options, handle).unwrap();
            paths.clear();
            0
        });
        let thread = FileOperationThread {
            tab_src: tab_src_index,
            tab_dest,
            handle,
            recv: rx,
        };
        Ok(thread)
    }

    fn copy(&self, context: &mut JoshutoContext) -> Result<FileOperationThread, std::io::Error> {
        let tab_dest = context.curr_tab_index;
        let tab_src_index = tab_src.load(atomic::Ordering::SeqCst);

        let destination = context.tabs[tab_dest].curr_path.clone();
        let options = self.options.clone();

        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut paths = selected_files.lock().unwrap();

            let handle = |process_info: fs_extra::TransitProcess| {
                let progress_info = ProgressInfo {
                    bytes_finished: process_info.copied_bytes,
                    total_bytes: process_info.total_bytes,
                };
                tx.send(progress_info).unwrap();
                fs_extra::dir::TransitProcessResult::ContinueOrAbort
            };
            fs_extra::copy_items_with_progress(&paths, &destination, &options, handle);
            paths.clear();
            0
        });
        let thread = FileOperationThread {
            tab_src: tab_src_index,
            tab_dest,
            handle,
            recv: rx,
        };
        Ok(thread)
    }
}

impl JoshutoCommand for PasteFiles {}

impl std::fmt::Display for PasteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} overwrite={}",
            Self::command(),
            self.options.overwrite
        )
    }
}

impl std::fmt::Debug for PasteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for PasteFiles {
    fn execute(&self, context: &mut JoshutoContext) {
        let file_operation = fileop.lock().unwrap();

        let thread = match *file_operation {
            FileOp::Copy => self.copy(context),
            FileOp::Cut => self.cut(context),
        };

        if let Ok(s) = thread {
            ncurses::timeout(-1);
            context.threads.push(s);
        }
    }
}
