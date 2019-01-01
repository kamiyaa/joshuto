extern crate fs_extra;

use std;
use std::fmt;
use std::fs;
use std::path;
use std::sync;

use joshuto;
use joshuto::command;
use joshuto::structs;

lazy_static! {
    static ref selected_files: sync::Mutex<Vec<path::PathBuf>> = sync::Mutex::new(vec![]);
    static ref fileop: sync::Mutex<FileOp> = sync::Mutex::new(FileOp::Copy);
}

fn set_file_op(operation: FileOp)
{
    let mut data = fileop.lock().unwrap();
    *data = operation;
}

pub fn get_selected_files(dirlist: &structs::JoshutoDirList)
        -> Option<Vec<path::PathBuf>>
{
    let selected: Vec<path::PathBuf> = dirlist.contents.iter()
            .filter(|entry| entry.selected)
            .map(|entry| entry.entry.path()).collect();
    if selected.len() > 0 {
        Some(selected)
    } else if dirlist.index >= 0 {
        Some(vec![dirlist.contents[dirlist.index as usize].entry.path()])
    } else {
        None
    }
}

fn repopulated_selected_files(dirlist: &Option<structs::JoshutoDirList>) -> bool
{
    if let Some(s) = dirlist.as_ref() {
        if let Some(contents) = get_selected_files(s) {
            let mut data = selected_files.lock().unwrap();
            *data = contents;
            return true;
        }
    }
    return false;
}

enum FileOp {
    Cut,
    Copy,
}

pub struct FileClipboard {
    files: Vec<path::PathBuf>,
    fileop: FileOp,
}

#[derive(Debug)]
pub struct Cut;

impl Cut {
    pub fn new() -> Self { Cut }
    pub fn command() -> &'static str { "Cut" }
}

impl command::JoshutoCommand for Cut {}

impl std::fmt::Display for Cut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for Cut {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if repopulated_selected_files(&context.curr_list) {
            set_file_op(FileOp::Cut);
        }
    }
}

#[derive(Debug)]
pub struct Copy;

impl Copy {
    pub fn new() -> Self { Copy }
    pub fn command() -> &'static str { "Copy" }
}

impl command::JoshutoCommand for Copy {}

impl std::fmt::Display for Copy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for Copy {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if repopulated_selected_files(&context.curr_list) {
            set_file_op(FileOp::Copy);
        }
    }
}

pub struct Paste {
    options: fs_extra::dir::CopyOptions,
}

impl Paste {
    pub fn new(options: fs_extra::dir::CopyOptions) -> Self
    {
        Paste {
            options,
        }
    }
    pub fn command() -> &'static str { "Paste" }


    fn cut(&self, destination: &path::PathBuf, options: &fs_extra::dir::CopyOptions) {
        let mut destination = destination;
        let handle = |process_info: fs_extra::TransitProcess| {
            eprintln!("{}", process_info.copied_bytes);
            fs_extra::dir::TransitProcessResult::ContinueOrAbort
        };

        let mut files = selected_files.lock().unwrap();

        match fs_extra::move_items_with_progress(&files, &destination, &options, handle)
        {
            Ok(s) => {},
            Err(e) => {},
        }
        files.clear();
    }

    fn copy(&self, destination: &path::PathBuf, options: &fs_extra::dir::CopyOptions) {
        let mut destination = destination;
        let handle = |process_info: fs_extra::TransitProcess| {
            eprintln!("{}", process_info.copied_bytes);
            fs_extra::dir::TransitProcessResult::ContinueOrAbort
        };

        let mut files = selected_files.lock().unwrap();

        match fs_extra::copy_items_with_progress(&files, &destination, &options, handle)
        {
            Ok(s) => {},
            Err(e) => {},
        }
        files.clear();
    }
}

impl command::JoshutoCommand for Paste {}

impl std::fmt::Display for Paste {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} overwrite={}", Self::command(), self.options.overwrite)
    }
}

impl std::fmt::Debug for Paste {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for Paste {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let destination = &context.curr_path;
        let file_operation = fileop.lock().unwrap();

        match *file_operation {
            FileOp::Copy => self.copy(destination, &self.options),
            FileOp::Cut => self.cut(destination, &self.options),
        }
    }
}


/*
pub struct DeleteClipboard {
    files: Vec<path::PathBuf>,
}

impl DeleteClipboard {
    pub fn new() -> Self
    {
        DeleteClipboard {
            files: Vec::new(),
        }
    }

    pub fn prepare(&mut self, dirlist: &structs::JoshutoDirList)
    {
        match FileClipboard::prepare(dirlist) {
            Some(s) => {
                self.files = s;
            }
            None => {},
        }
    }

    pub fn execute(&mut self) -> std::io::Result<()>
    {
        for path in &self.files {
            if path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            } else {
                std::fs::remove_file(&path)?;
            }
        }
        self.files.clear();
        Ok(())
    }
}
*/
