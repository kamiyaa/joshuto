use lazy_static::lazy_static;
use std::path;
use std::sync::{atomic, mpsc, Mutex};
use std::thread;
use std::time;

use crate::commands::{self, JoshutoCommand, JoshutoRunnable, ProgressInfo};
use crate::context::JoshutoContext;
use crate::structs::JoshutoDirList;
use crate::window::JoshutoView;

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
        let tab_src_index = TAB_SRC.load(atomic::Ordering::SeqCst);

        let options = self.options.clone();
        let mut destination = context.tabs[tab_dest].curr_path.clone();

        let dest_ino = destination.metadata()?.st_dev();
        let path_ino;
        {
            let paths = SELECTED_FILES.lock().unwrap();
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
                let mut paths = SELECTED_FILES.lock().unwrap();
                let mut progress_info = ProgressInfo {
                    bytes_finished: 1,
                    total_bytes: paths.len() as u64 + 1,
                };

                for path in (*paths).iter() {
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
                            std::fs::rename(&path, &destination);
                        }
                    } else {
                        std::fs::rename(&path, &destination);
                    }
                    destination.pop();
                    progress_info.bytes_finished += 1;
                    tx.send(progress_info.clone()).unwrap();
                }
                paths.clear();
                0
            })
        } else {
            thread::spawn(move || {
                let mut paths = SELECTED_FILES.lock().unwrap();

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
        let tab_src_index = TAB_SRC.load(atomic::Ordering::SeqCst);

        let mut destination = context.tabs[tab_dest].curr_path.clone();
        let options = self.options.clone();

        {
            let paths = SELECTED_FILES.lock().unwrap();
            if paths.len() == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "no files selected",
                ));
            }
        }
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut paths = SELECTED_FILES.lock().unwrap();

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
        let tab_src_index = TAB_SRC.load(atomic::Ordering::SeqCst);

        let destination = context.tabs[tab_dest].curr_path.clone();
        let options = self.options.clone();

        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let paths = SELECTED_FILES.lock().unwrap();

            let handle = |process_info: fs_extra::TransitProcess| {
                let progress_info = ProgressInfo {
                    bytes_finished: process_info.copied_bytes,
                    total_bytes: process_info.total_bytes,
                };
                tx.send(progress_info).unwrap();
                fs_extra::dir::TransitProcessResult::ContinueOrAbort
            };
            fs_extra::copy_items_with_progress(&paths, &destination, &options, handle);
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
    fn execute(&self, context: &mut JoshutoContext, _: &JoshutoView) {
        let file_operation = FILE_OPERATION.lock().unwrap();

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
