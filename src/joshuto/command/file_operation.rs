use std;
use std::path;
use std::sync;
use std::thread;
use lazy_static::lazy_static;

use crate::joshuto::command;
use crate::joshuto::command::ProgressInfo;
use crate::joshuto::command::JoshutoCommand;
use crate::joshuto::command::JoshutoRunnable;
use crate::joshuto::context::JoshutoContext;
use crate::joshuto::preview;
use crate::joshuto::structs::JoshutoDirList;

lazy_static! {
    static ref selected_files: sync::Mutex<Vec<path::PathBuf>> = sync::Mutex::new(vec![]);
    static ref fileop: sync::Mutex<FileOp> = sync::Mutex::new(FileOp::Copy);
}

fn set_file_op(operation: FileOp)
{
    let mut data = fileop.lock().unwrap();
    *data = operation;
}

fn repopulated_selected_files(dirlist: &JoshutoDirList) -> bool
{
    if let Some(contents) = command::collect_selected_paths(dirlist) {
        let mut data = selected_files.lock().unwrap();
        *data = contents;
        return true;
    }
    return false;
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
    pub fn new() -> Self { CutFiles }
    pub const fn command() -> &'static str { "cut_files" }
}

impl JoshutoCommand for CutFiles {}

impl std::fmt::Display for CutFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CutFiles {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let curr_tab = &context.tabs[context.curr_tab_index];
        if let Some(s) = curr_tab.curr_list.as_ref() {
            if repopulated_selected_files(s) {
                set_file_op(FileOp::Cut);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CopyFiles;

impl CopyFiles {
    pub fn new() -> Self { CopyFiles }
    pub const fn command() -> &'static str { "copy_files" }
}

impl JoshutoCommand for CopyFiles {}

impl std::fmt::Display for CopyFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CopyFiles {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let curr_tab = &context.tabs[context.curr_tab_index];
        if let Some(s) = curr_tab.curr_list.as_ref() {
            if repopulated_selected_files(s) {
                set_file_op(FileOp::Copy);
            }
        }
    }
}

pub struct PasteFiles {
    options: fs_extra::dir::CopyOptions,
}

impl PasteFiles {
    pub fn new(options: fs_extra::dir::CopyOptions) -> Self
    {
        PasteFiles {
            options,
        }
    }
    pub const fn command() -> &'static str { "paste_files" }

    fn cut(&self, destination: &path::PathBuf)
            -> (sync::mpsc::Receiver<command::ProgressInfo>, thread::JoinHandle<i32>)
    {
        let (tx, rx) = sync::mpsc::channel();

        let mut destination = destination.clone();
        let options = self.options.clone();

        let child = thread::spawn(move || {
            let mut paths = selected_files.lock().unwrap();

            let mut progress_info = ProgressInfo {
                    bytes_finished: 1,
                    total_bytes: paths.len() as u64 + 1,
                };

            for path in (*paths).iter() {
                let mut file_name = path.file_name().unwrap().to_os_string();

                while path::Path::new(&file_name).exists() {
                    file_name.push("_0");
                }

                destination.push(file_name);
                if options.skip_exist && destination.exists() {
                    continue;
                }

                match std::fs::rename(&path, &destination) {
                    Ok(_) => {
                        destination.pop();
                    },
                    Err(_) => {
                        if let Ok(metadata) = std::fs::symlink_metadata(path) {
                            if metadata.is_dir() {
                                destination.pop();
                                match fs_extra::dir::move_dir(&path, &destination, &options) {
                                    Ok(_) => {},
                                    Err(e) => eprintln!("dir: {}", e),
                                }
                            } else {
                                if let Ok(_) = std::fs::copy(&path, &destination) {
                                    std::fs::remove_file(&path).unwrap();
                                }
                                destination.pop();
                            }
                        } else {
                            destination.pop();
                        }
                    }
                }

                progress_info.bytes_finished = progress_info.bytes_finished + 1;
                tx.send(progress_info.clone()).unwrap();
            }

            paths.clear();
            0
        });

        (rx, child)
    }

    fn copy(&self, destination: &path::PathBuf)
            -> (sync::mpsc::Receiver<command::ProgressInfo>, thread::JoinHandle<i32>)
    {
        let (tx, rx) = sync::mpsc::channel();

        let destination = destination.clone();
        let options = self.options.clone();

        let child = thread::spawn(move || {
            let files = selected_files.lock().unwrap();

            let handle = |process_info: fs_extra::TransitProcess| {
                let progress_info = ProgressInfo {
                        bytes_finished: process_info.copied_bytes,
                        total_bytes: process_info.total_bytes,
                    };
                tx.send(progress_info).unwrap();
                fs_extra::dir::TransitProcessResult::ContinueOrAbort
            };

            match fs_extra::copy_items_with_progress(&files, &destination, &options, handle) {
                Ok(_) => {},
                Err(_) => {},
            }
            0
        });
        (rx, child)
    }
}

impl JoshutoCommand for PasteFiles {}

impl std::fmt::Display for PasteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{} overwrite={}", Self::command(), self.options.overwrite)
    }
}

impl std::fmt::Debug for PasteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for PasteFiles {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let file_operation = fileop.lock().unwrap();

        let curr_tab = &mut context.tabs[context.curr_tab_index];
        let cprocess = match *file_operation {
                FileOp::Copy => self.copy(&curr_tab.curr_path),
                FileOp::Cut => self.cut(&curr_tab.curr_path),
            };
        context.threads.push(cprocess);

        curr_tab.reload_contents(&context.config_t.sort_type);
        curr_tab.refresh(&context.views, &context.config_t,
            &context.username, &context.hostname);
        ncurses::timeout(0);
        ncurses::doupdate();
    }
}
