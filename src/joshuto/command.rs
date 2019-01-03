extern crate fs_extra;
extern crate ncurses;

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

mod cursor_move;
pub use self::cursor_move::CursorMove;
pub use self::cursor_move::CursorMovePageUp;
pub use self::cursor_move::CursorMovePageDown;
pub use self::cursor_move::CursorMoveHome;
pub use self::cursor_move::CursorMoveEnd;

mod file_operation;
pub use self::file_operation::CutFiles;
pub use self::file_operation::CopyFiles;
pub use self::file_operation::PasteFiles;
pub use self::file_operation::DeleteFiles;
pub use self::file_operation::RenameFile;
pub use self::file_operation::RenameFileMethod;

mod show_hidden;
pub use self::show_hidden::ToggleHiddenFiles;

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


pub fn from_args(args: &[&str]) -> Option<Box<dyn JoshutoCommand>>
{
    let args_len = args.len();

    if args_len == 0 {
        return None;
    }

    match args[0] {
        "quit" => Some(Box::new(self::Quit::new())),
        "parent_directory" => Some(Box::new(self::ParentDirectory::new())),

        "open_file" => Some(Box::new(self::OpenFile::new())),
        "open_file_with" => Some(Box::new(self::OpenFileWith::new())),
        "change_directory" => {
            if args_len > 1 {
                let path = path::PathBuf::from(args[1]);
                Some(Box::new(self::ChangeDirectory::new(path)))
            } else {
                None
            }
        },

        "cursor_move" => {
            if args_len > 1 {
                match args[1].parse::<i32>() {
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
        },
        "cursor_move_home" => Some(Box::new(self::CursorMoveHome::new())),
        "cursor_move_end" => Some(Box::new(self::CursorMoveEnd::new())),
        "cursor_move_page_up" => Some(Box::new(self::CursorMovePageUp::new())),
        "cursor_move_page_down" => Some(Box::new(self::CursorMovePageDown::new())),

        "cut_files" => Some(Box::new(self::CutFiles::new())),
        "copy_files" => Some(Box::new(self::CopyFiles::new())),
        "paste_files" => {
            let mut options = fs_extra::dir::CopyOptions::new();
            for arg in &args[1..] {
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
            let paste = self::PasteFiles::new(options);
            Some(Box::new(paste))
        }
        "delete_files" => Some(Box::new(self::DeleteFiles::new())),
        "rename_file" => {
            let method: self::file_operation::RenameFileMethod;
            if args_len == 1 {
                method = self::RenameFileMethod::Append;
            } else {
                method = match args[1] {
                    "prepend" => self::RenameFileMethod::Prepend,
                    "overwrite" => self::RenameFileMethod::Overwrite,
                    "append" => self::RenameFileMethod::Append,
                    _ => self::RenameFileMethod::Append,
                }
            }
            Some(Box::new(self::RenameFile::new(method)))
        }
        "toggle_hidden" => Some(Box::new(self::ToggleHiddenFiles::new())),
        _ => None,
    }
}
