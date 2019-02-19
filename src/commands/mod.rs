extern crate wordexp;

mod change_directory;
mod cursor_move;
mod delete_files;
mod file_operations;
mod new_directory;
mod open_file;
mod parent_directory;
mod quit;
mod reload_dir;
mod rename_file;
mod search;
mod selection;
mod set_mode;
mod show_hidden;
mod tab_operations;
mod tab_switch;

pub use self::change_directory::ChangeDirectory;
pub use self::cursor_move::{
    CursorMove, CursorMoveEnd, CursorMoveHome, CursorMovePageDown, CursorMovePageUp,
};
pub use self::delete_files::DeleteFiles;
pub use self::file_operations::{CopyFiles, CutFiles, FileOperationThread, PasteFiles};
pub use self::new_directory::NewDirectory;
pub use self::open_file::{OpenFile, OpenFileWith};
pub use self::parent_directory::ParentDirectory;
pub use self::quit::ForceQuit;
pub use self::quit::Quit;
pub use self::reload_dir::ReloadDirList;
pub use self::rename_file::{RenameFile, RenameFileMethod};
pub use self::search::Search;
pub use self::selection::SelectFiles;
pub use self::set_mode::SetMode;
pub use self::show_hidden::ToggleHiddenFiles;
pub use self::tab_operations::{CloseTab, NewTab};
pub use self::tab_switch::TabSwitch;

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use context::JoshutoContext;
use structs;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind(Box<dyn JoshutoCommand>),
    CompositeKeybind(HashMap<i32, CommandKeybind>),
}

pub trait JoshutoRunnable {
    fn execute(&self, context: &mut JoshutoContext);
}

pub trait JoshutoCommand: JoshutoRunnable + std::fmt::Display + std::fmt::Debug {}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind(s) => write!(f, "{}", s),
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProgressInfo {
    pub bytes_finished: u64,
    pub total_bytes: u64,
}

pub fn from_args(command: &str, args: Option<&Vec<String>>) -> Option<Box<dyn JoshutoCommand>> {
    match command {
        "cd" => {
            if let Some(args) = args {
                match wordexp::wordexp(args[0].as_str(), 0) {
                    Ok(exp_strs) => {
                        if let Some(exp_str) = exp_strs.into_iter().next() {
                            let path = PathBuf::from(exp_str);
                            return Some(Box::new(self::ChangeDirectory::new(path)));
                        }
                    }
                    Err(_) => {
                        eprintln!("Failed to parse: {:?}", args[0]);
                    }
                }
            }
            None
        }
        "close_tab" => Some(Box::new(self::CloseTab::new())),
        "copy_files" => Some(Box::new(self::CopyFiles::new())),
        "cursor_move" => {
            if let Some(args) = args {
                if !args.is_empty() {
                    match args[0].parse::<i32>() {
                        Ok(s) => {
                            return Some(Box::new(self::CursorMove::new(s)));
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                }
            }
            None
        }
        "cursor_move_home" => Some(Box::new(self::CursorMoveHome::new())),
        "cursor_move_end" => Some(Box::new(self::CursorMoveEnd::new())),
        "cursor_move_page_up" => Some(Box::new(self::CursorMovePageUp::new())),
        "cursor_move_page_down" => Some(Box::new(self::CursorMovePageDown::new())),
        "cut_files" => Some(Box::new(self::CutFiles::new())),
        "delete_files" => Some(Box::new(self::DeleteFiles::new())),
        "force_quit" => Some(Box::new(self::ForceQuit::new())),
        "mkdir" => Some(Box::new(self::NewDirectory::new())),
        "new_tab" => Some(Box::new(self::NewTab::new())),
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
                            }
                            "skip_exist" => {
                                if let Ok(s) = splitarg[1].parse::<bool>() {
                                    options.skip_exist = s;
                                } else {
                                    eprintln!("Failed to parse: {}", arg);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            let paste = self::PasteFiles::new(options);
            Some(Box::new(paste))
        }
        "quit" => Some(Box::new(self::Quit::new())),
        "reload_dir_list" => Some(Box::new(self::ReloadDirList::new())),
        "rename_file" => {
            let method: RenameFileMethod;
            if let Some(args) = args {
                if !args.is_empty() {
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
                            }
                            "all" => {
                                if let Ok(s) = splitarg[1].parse::<bool>() {
                                    all = s;
                                } else {
                                    eprintln!("Failed to parse: {}", arg);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Some(Box::new(self::SelectFiles::new(toggle, all)))
        }
        "set_mode" => Some(Box::new(self::SetMode::new())),
        "tab_switch" => {
            if let Some(args) = args {
                if !args.is_empty() {
                    match args[0].parse::<i32>() {
                        Ok(s) => {
                            return Some(Box::new(self::TabSwitch::new(s)));
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                }
            }
            None
        }
        "toggle_hidden" => Some(Box::new(self::ToggleHiddenFiles::new())),
        _ => None,
    }
}

pub fn collect_selected_paths(dirlist: &structs::JoshutoDirList) -> Option<Vec<PathBuf>> {
    if dirlist.index < 0 {
        return None;
    }

    let selected: Vec<PathBuf> = dirlist
        .contents
        .iter()
        .filter(|entry| entry.selected)
        .map(|entry| entry.path.clone())
        .collect();
    if !selected.is_empty() {
        Some(selected)
    } else {
        Some(vec![dirlist.contents[dirlist.index as usize].path.clone()])
    }
}

#[allow(dead_code)]
pub fn split_shell_style(line: &str) -> Vec<&str> {
    let mut args: Vec<&str> = Vec::new();
    let mut char_ind = line.char_indices();

    while let Some((i, ch)) = char_ind.next() {
        if ch.is_whitespace() {
            continue;
        }
        if ch == '\'' {
            while let Some((j, ch)) = char_ind.next() {
                if ch == '\'' {
                    args.push(&line[i + 1..j]);
                    break;
                }
            }
        } else if ch == '"' {
            while let Some((j, ch)) = char_ind.next() {
                if ch == '"' {
                    args.push(&line[i + 1..j]);
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
