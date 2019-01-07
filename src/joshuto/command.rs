extern crate fs_extra;
extern crate ncurses;
extern crate wordexp;

use std;
use std::collections::HashMap;
use std::fmt;
use std::path;

use joshuto;

mod quit;
pub use self::quit::Quit;

mod parent_directory;
pub use self::parent_directory::ParentDirectory;

mod open_file;
pub use self::open_file::OpenFile;
pub use self::open_file::OpenFileWith;

mod change_directory;
pub use self::change_directory::ChangeDirectory;
mod reload_dir;
pub use self::reload_dir::ReloadDirList;

mod cursor_move;
pub use self::cursor_move::CursorMove;
pub use self::cursor_move::CursorMovePageUp;
pub use self::cursor_move::CursorMovePageDown;
pub use self::cursor_move::CursorMoveHome;
pub use self::cursor_move::CursorMoveEnd;

mod file_operation;
pub use self::file_operation::ProgressInfo;
pub use self::file_operation::CutFiles;
pub use self::file_operation::CopyFiles;
pub use self::file_operation::PasteFiles;
pub use self::file_operation::DeleteFiles;
pub use self::file_operation::RenameFile;
pub use self::file_operation::RenameFileMethod;

mod new_directory;
pub use self::new_directory::NewDirectory;

mod search;
pub use self::search::Search;

mod show_hidden;
pub use self::show_hidden::ToggleHiddenFiles;

mod selection;
pub use self::selection::SelectFiles;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind(Box<dyn JoshutoCommand>),
    CompositeKeybind(HashMap<i32, CommandKeybind>),
}

pub trait Runnable {
    fn execute(&self, context: &mut joshuto::JoshutoContext);
}

pub trait JoshutoCommand: Runnable + std::fmt::Display + std::fmt::Debug {}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind(s) => write!(f, "{}", s),
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}

#[allow(dead_code)]
pub fn split_shell_style(line: &String) -> Vec<&str>
{
    let mut args: Vec<&str> = Vec::new();
    let mut char_ind = line.char_indices();

    while let Some((i, ch)) = char_ind.next() {
        if ch.is_whitespace() {
            continue;
        }
        if ch == '\'' {
            while let Some((j, ch)) = char_ind.next() {
                if ch == '\'' {
                    args.push(&line[i+1..j]);
                    break;
                }
            }
        } else if ch == '"'{
            while let Some((j, ch)) = char_ind.next() {
                if ch == '"' {
                    args.push(&line[i+1..j]);
                    break;
                }
            }
        } else {
            while let Some((j, ch)) = char_ind.next() {
                if ch.is_whitespace() {
                    args.push(&line[i..j]);
                    break;
                }
            }
        }
    }
    args
}


pub fn from_args(command: &str, args: Option<&Vec<String>>) -> Option<Box<dyn JoshutoCommand>>
{
    match command {
        "cd" => {
            if let Some(args) = args {
                let exp_strs = wordexp::wordexp(args[0].as_str(), 0);
                if exp_strs.len() > 0 {
                    let path = path::PathBuf::from(exp_strs[0].as_str());
                    Some(Box::new(self::ChangeDirectory::new(path)))
                } else {
                    None
                }
            } else {
                None
            }
        },
        "copy_files" => Some(Box::new(self::CopyFiles::new())),
        "cursor_move" => {
            if let Some(args) = args {
                if args.len() > 0 {
                    match args[0].parse::<i32>() {
                        Ok(s) => {
                            Some(Box::new(self::CursorMove::new(s)))
                        },
                        Err(e) => {
                            eprintln!("{}", e);
                            None
                        },
                    }
                } else {
                    None
                }
            } else {
                None
            }
        },
        "cursor_move_home" => Some(Box::new(self::CursorMoveHome::new())),
        "cursor_move_end" => Some(Box::new(self::CursorMoveEnd::new())),
        "cursor_move_page_up" => Some(Box::new(self::CursorMovePageUp::new())),
        "cursor_move_page_down" => Some(Box::new(self::CursorMovePageDown::new())),
        "cut_files" => Some(Box::new(self::CutFiles::new())),
        "delete_files" => Some(Box::new(self::DeleteFiles::new())),
        "mkdir" => Some(Box::new(self::NewDirectory::new())),
        "open_file" => Some(Box::new(self::OpenFile::new())),
        "open_file_with" => Some(Box::new(self::OpenFileWith::new())),
        "parent_directory" => Some(Box::new(self::ParentDirectory::new())),
        "paste_files" => {
            let mut options = fs_extra::dir::CopyOptions::new();
            if let Some(args) = args {
                for arg in args {
                    let splitarg: Vec<&str> = arg.split('=').collect();
                    if splitarg.len() == 2 {
                        match splitarg[0] {
                            "overwrite" => {
                                if let Ok(s) = splitarg[1].parse::<bool>() {
                                    options.overwrite = s;
                                } else {
                                    eprintln!("Failed to parse: {}", arg);
                                }
                            },
                            "skip_exist" => {
                                if let Ok(s) = splitarg[1].parse::<bool>() {
                                    options.skip_exist = s;
                                } else {
                                    eprintln!("Failed to parse: {}", arg);
                                }
                            },
                            _ => {},
                        }
                    }
                }
            }
            let paste = self::PasteFiles::new(options);
            Some(Box::new(paste))
        },
        "quit" => Some(Box::new(self::Quit::new())),
        "reload_dir_list" => Some(Box::new(self::ReloadDirList::new())),
        "rename_file" => {
            let method: self::file_operation::RenameFileMethod;
            if let Some(args) = args {
                if args.len() > 0 {
                    method = match args[0].as_str() {
                        "prepend" => self::RenameFileMethod::Prepend,
                        "overwrite" => self::RenameFileMethod::Overwrite,
                        "append" => self::RenameFileMethod::Append,
                        _ => self::RenameFileMethod::Append,
                    };
                } else {
                    method = self::RenameFileMethod::Append;
                }
            } else {
                method = self::RenameFileMethod::Append;
            }
            Some(Box::new(self::RenameFile::new(method)))
        }
        "search" => Some(Box::new(self::Search::new())),
        "select_files" => {
            let mut toggle = false;
            let mut all = false;
            if let Some(args) = args {
                for arg in args {
                    let splitarg: Vec<&str> = arg.split('=').collect();
                    if splitarg.len() == 2 {
                        match splitarg[0] {
                            "toggle" => {
                                if let Ok(s) = splitarg[1].parse::<bool>() {
                                    toggle = s;
                                } else {
                                    eprintln!("Failed to parse: {}", arg);
                                }
                            },
                            "all" => {
                                if let Ok(s) = splitarg[1].parse::<bool>() {
                                    all = s;
                                } else {
                                    eprintln!("Failed to parse: {}", arg);
                                }
                            },
                            _ => {},
                        }
                    }
                }
            }
            Some(Box::new(self::SelectFiles::new(toggle, all)))
        },
        "toggle_hidden" => Some(Box::new(self::ToggleHiddenFiles::new())),
        _ => None,
    }
}
