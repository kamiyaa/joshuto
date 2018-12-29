extern crate fs_extra;

use std;
use std::collections::HashMap;
use std::fmt;
use std::path;

pub enum JoshutoCommand {
    Quit,

    ReloadDirList,

    CursorMove(i32),
    CursorMovePageUp,
    CursorMovePageDown,
    CursorMoveHome,
    CursorMoveEnd,
    ParentDirectory,

    ChangeDirectory(path::PathBuf),

    DeleteFiles,
    RenameFile,
    CutFiles,
    CopyFiles,
    PasteFiles(fs_extra::dir::CopyOptions),
    Open,
    OpenWith,
    ToggleHiddenFiles,

    MarkFiles{ toggle: bool, all: bool },

    CompositeKeybind(HashMap<i32, JoshutoCommand>),
}

impl std::fmt::Display for JoshutoCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JoshutoCommand::Quit => write!(f, "quit"),
            JoshutoCommand::ReloadDirList => write!(f, "reload_directory"),

            JoshutoCommand::CursorMove(s) => write!(f, "move {}", s),
            JoshutoCommand::CursorMovePageUp => write!(f, "page_up"),
            JoshutoCommand::CursorMovePageDown => write!(f, "page_down"),
            JoshutoCommand::CursorMoveHome => write!(f, "home"),
            JoshutoCommand::CursorMoveEnd => write!(f, "end"),
            JoshutoCommand::ParentDirectory => write!(f, "parent_directory"),

            JoshutoCommand::ChangeDirectory(s) => write!(f, "cd {}", s.to_str().unwrap()),

            JoshutoCommand::DeleteFiles => write!(f, "delete"),
            JoshutoCommand::RenameFile => write!(f, "rename"),
            JoshutoCommand::CutFiles => write!(f, "cut"),
            JoshutoCommand::CopyFiles => write!(f, "copy"),
            JoshutoCommand::PasteFiles(options) => write!(f, "paste overwrite={}", options.overwrite),
            JoshutoCommand::Open => write!(f, "open"),
            JoshutoCommand::OpenWith => write!(f, "open_with"),
            JoshutoCommand::ToggleHiddenFiles => write!(f, "toggle_hidden"),

            JoshutoCommand::MarkFiles{toggle, all} => write!(f, "mark_files toggle={} all={}", toggle, all),

            JoshutoCommand::CompositeKeybind(_) => write!(f, "..."),

            _ => write!(f, "unknown command"),
        }
    }

}

impl JoshutoCommand {
    pub fn split_shell_style(line: &String) -> Vec<&str>
    {
        let mut args: Vec<&str> = Vec::new();
        let mut char_ind = line.char_indices();

        while let Some((i, ch)) = char_ind.next() {
            if ch.is_whitespace() {
                continue;
            }
            let mut end_ind = i;
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

    pub fn from_args(args: &[&str]) -> Option<Self>
    {
        let args_len = args.len();
        if args_len > 0 {
            match args[0] {
                "Quit" => Some(JoshutoCommand::Quit),
                "ReloadDirList" => Some(JoshutoCommand::ReloadDirList),

                "CursorMove" => {
                    if args_len > 1 {
                        match args[1].parse::<i32>() {
                            Ok(s) => {
                                Some(JoshutoCommand::CursorMove(s))
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
                "CursorMovePageUp" => Some(JoshutoCommand::CursorMovePageUp),
                "CursorMovePageDown" => Some(JoshutoCommand::CursorMovePageDown),
                "CursorMoveHome" => Some(JoshutoCommand::CursorMoveHome),
                "CursorMoveEnd" => Some(JoshutoCommand::CursorMoveEnd),
                "ParentDirectory" => Some(JoshutoCommand::ParentDirectory),

                "ChangeDirectory" => {
                    if args_len > 1 {
                        let path = path::PathBuf::from(&args[1]);
                        Some(JoshutoCommand::ChangeDirectory(path))
                    } else {
                        None
                    }
                },

                "DeleteFiles" => Some(JoshutoCommand::DeleteFiles),
                "RenameFile" => Some(JoshutoCommand::RenameFile),
                "CutFiles" => Some(JoshutoCommand::CutFiles),
                "CopyFiles" => Some(JoshutoCommand::CopyFiles),
                "PasteFiles" => {
                    let mut options = fs_extra::dir::CopyOptions::new();
                    for i in 1..args_len {
                        let splitargs: Vec<&str> = args[i].split('=').collect();
                        if splitargs.len() == 2 {
                            match splitargs[0] {
                                "overwrite" => {
                                    if let Ok(s) = splitargs[1].parse::<bool>() {
                                        options.overwrite = s;
                                    }
                                },
                                _ => eprintln!("Unknown option for PasteFile: {}", args[i]),
                            }
                        }
                    }
                    Some(JoshutoCommand::PasteFiles(options))
                },
                "Open" => Some(JoshutoCommand::Open),
                "OpenWith" => Some(JoshutoCommand::OpenWith),
                "ToggleHiddenFiles" => Some(JoshutoCommand::ToggleHiddenFiles),

                "MarkFiles" => Some(JoshutoCommand::MarkFiles{toggle: true, all: false}),

                _ => None,
            }
        } else {
            None
        }
    }

    pub fn clone(&self) -> Self
    {
        match self {
            JoshutoCommand::Quit => JoshutoCommand::Quit,
            JoshutoCommand::ReloadDirList => JoshutoCommand::ReloadDirList,

            JoshutoCommand::CursorMove(s) => JoshutoCommand::CursorMove(*s),
            JoshutoCommand::CursorMovePageUp => JoshutoCommand::CursorMovePageUp,
            JoshutoCommand::CursorMovePageDown => JoshutoCommand::CursorMovePageDown,
            JoshutoCommand::CursorMoveHome => JoshutoCommand::CursorMoveHome,
            JoshutoCommand::CursorMoveEnd => JoshutoCommand::CursorMoveEnd,
            JoshutoCommand::ParentDirectory => JoshutoCommand::ParentDirectory,

            JoshutoCommand::ChangeDirectory(s) => JoshutoCommand::ChangeDirectory(s.clone()),

            JoshutoCommand::DeleteFiles => JoshutoCommand::DeleteFiles,
            JoshutoCommand::RenameFile => JoshutoCommand::RenameFile,
            JoshutoCommand::CutFiles => JoshutoCommand::CutFiles,
            JoshutoCommand::CopyFiles => JoshutoCommand::CopyFiles,

            JoshutoCommand::Open => JoshutoCommand::Open,
            JoshutoCommand::OpenWith => JoshutoCommand::OpenWith,
            JoshutoCommand::ToggleHiddenFiles => JoshutoCommand::ToggleHiddenFiles,

            JoshutoCommand::MarkFiles{toggle, all} => JoshutoCommand::MarkFiles{toggle: *toggle, all: *all},

            JoshutoCommand::CompositeKeybind(_) => JoshutoCommand::Quit,

            _ => JoshutoCommand::Quit,
        }
    }
}
