use std;
use std::collections::HashMap;
use std::fmt;
use std::path;

#[derive(Debug)]
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
    PasteFiles{ overwrite: bool },
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
            JoshutoCommand::PasteFiles{overwrite} => write!(f, "paste overwrite={}", overwrite),
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

    pub fn from_args(command: &[&str]) -> Option<Self>
    {
        let command_len = command.len();
        if command_len > 0 {
            match command[0] {
                "Quit" => Some(JoshutoCommand::Quit),
                "ReloadDirList" => Some(JoshutoCommand::ReloadDirList),

                "CursorMove" => {
                    if command_len > 1 {
                        match command[1].parse::<i32>() {
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
                    if command_len > 1 {
                        let path = path::PathBuf::from(&command[1]);
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
                    Some(JoshutoCommand::PasteFiles{overwrite: false})
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
            JoshutoCommand::PasteFiles{overwrite} => JoshutoCommand::PasteFiles{overwrite: *overwrite},

            JoshutoCommand::Open => JoshutoCommand::Open,
            JoshutoCommand::OpenWith => JoshutoCommand::OpenWith,
            JoshutoCommand::ToggleHiddenFiles => JoshutoCommand::ToggleHiddenFiles,

            JoshutoCommand::MarkFiles{toggle, all} => JoshutoCommand::MarkFiles{toggle: *toggle, all: *all},

            JoshutoCommand::CompositeKeybind(_) => JoshutoCommand::Quit,

            _ => JoshutoCommand::Quit,
        }
    }
}
