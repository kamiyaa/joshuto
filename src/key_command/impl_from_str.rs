use std::path;

use dirs_next::home_dir;
use shellexpand::tilde_with_context;

use crate::error::{JoshutoError, JoshutoErrorKind};
use crate::io::IoWorkerOptions;
use crate::util::select::SelectOption;
use crate::util::sort::SortType;

use crate::HOME_DIR;

use super::constants::*;
use super::KeyCommand;

impl std::str::FromStr for KeyCommand {
    type Err = JoshutoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix(':') {
            return Ok(Self::CommandLine(stripped.to_owned(), "".to_owned()));
        }

        let (command, arg) = match s.find(' ') {
            Some(i) => (&s[..i], s[i..].trim_start()),
            None => (s, ""),
        };

        if command == CMD_BULK_RENAME {
            Ok(Self::BulkRename)
        } else if command == CMD_CHANGE_DIRECTORY {
            match arg {
                "" => match HOME_DIR.as_ref() {
                    Some(s) => Ok(Self::ChangeDirectory(s.clone())),
                    None => Err(JoshutoError::new(
                        JoshutoErrorKind::EnvVarNotPresent,
                        format!("{}: Cannot find home directory", command),
                    )),
                },
                ".." => Ok(Self::ParentDirectory),
                arg => Ok({
                    let path_accepts_tilde = tilde_with_context(arg, home_dir);
                    Self::ChangeDirectory(path::PathBuf::from(path_accepts_tilde.as_ref()))
                }),
            }
        } else if command == CMD_CLOSE_TAB {
            Ok(Self::CloseTab)
        } else if command == CMD_COPY_FILES {
            Ok(Self::CopyFiles)
        } else if command == CMD_COPY_FILENAME {
            Ok(Self::CopyFileName)
        } else if command == CMD_COPY_FILENAME_WITHOUT_EXTENSION {
            Ok(Self::CopyFileNameWithoutExtension)
        } else if command == CMD_COPY_FILEPATH {
            Ok(Self::CopyFilePath)
        } else if command == CMD_COPY_DIRECTORY_PATH {
            Ok(Self::CopyDirPath)
        } else if command == CMD_CURSOR_MOVE_HOME {
            Ok(Self::CursorMoveHome)
        } else if command == CMD_CURSOR_MOVE_END {
            Ok(Self::CursorMoveEnd)
        } else if command == CMD_CURSOR_MOVE_PAGEUP {
            Ok(Self::CursorMovePageUp)
        } else if command == CMD_CURSOR_MOVE_PAGEDOWN {
            Ok(Self::CursorMovePageDown)
        } else if command == CMD_CURSOR_MOVE_DOWN {
            match arg {
                "" => Ok(Self::CursorMoveDown(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveDown(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            }
        } else if command == CMD_CURSOR_MOVE_UP {
            match arg {
                "" => Ok(Self::CursorMoveUp(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveUp(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            }
        } else if command == CMD_PARENT_CURSOR_MOVE_DOWN {
            match arg {
                "" => Ok(Self::ParentCursorMoveDown(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::ParentCursorMoveDown(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            }
        } else if command == CMD_PARENT_CURSOR_MOVE_UP {
            match arg {
                "" => Ok(Self::ParentCursorMoveUp(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::ParentCursorMoveUp(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            }
        } else if command == CMD_CUT_FILES {
            Ok(Self::CutFiles)
        } else if command == CMD_DELETE_FILES {
            Ok(Self::DeleteFiles)
        } else if command == CMD_FORCE_QUIT {
            Ok(Self::ForceQuit)
        } else if command == CMD_NEW_DIRECTORY {
            if arg.is_empty() {
                Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: no directory name given", command),
                ))
            } else {
                Ok(Self::NewDirectory(path::PathBuf::from(arg)))
            }
        } else if command == CMD_OPEN_FILE {
            Ok(Self::OpenFile)
        } else if command == CMD_OPEN_FILE_WITH {
            match arg {
                "" => Ok(Self::OpenFileWith(None)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::OpenFileWith(Some(s))),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            }
        } else if command == CMD_PASTE_FILES {
            let mut options = IoWorkerOptions::default();
            for arg in arg.split_whitespace() {
                match arg {
                    "--overwrite=true" => options.overwrite = true,
                    "--skip_exist=true" => options.skip_exist = true,
                    "--overwrite=false" => options.overwrite = false,
                    "--skip_exist=false" => options.skip_exist = false,
                    _ => {
                        return Err(JoshutoError::new(
                            JoshutoErrorKind::UnrecognizedArgument,
                            format!("{}: unknown option '{}'", command, arg),
                        ));
                    }
                }
            }
            Ok(Self::PasteFiles(options))
        } else if command == CMD_NEW_TAB {
            Ok(Self::NewTab)
        } else if command == CMD_QUIT {
            Ok(Self::Quit)
        } else if command == CMD_QUIT_TO_CURRENT_DIRECTORY {
            Ok(Self::QuitToCurrentDirectory)
        } else if command == CMD_RELOAD_DIRECTORY_LIST {
            Ok(Self::ReloadDirList)
        } else if command == CMD_RENAME_FILE {
            match arg {
                "" => Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => {
                    let path: path::PathBuf = path::PathBuf::from(arg);
                    Ok(Self::RenameFile(path))
                }
            }
        } else if command == CMD_RENAME_FILE_APPEND {
            Ok(Self::RenameFileAppend)
        } else if command == CMD_RENAME_FILE_PREPEND {
            Ok(Self::RenameFilePrepend)
        } else if command == CMD_SEARCH_STRING {
            match arg {
                "" => Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => Ok(Self::SearchString(arg.to_string())),
            }
        } else if command == CMD_SEARCH_GLOB {
            match arg {
                "" => Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => Ok(Self::SearchGlob(arg.to_string())),
            }
        } else if command == CMD_SEARCH_SKIM {
            Ok(Self::SearchSkim)
        } else if command == CMD_SEARCH_NEXT {
            Ok(Self::SearchNext)
        } else if command == CMD_SEARCH_PREV {
            Ok(Self::SearchPrev)
        } else if command == CMD_SELECT_FILES {
            let mut options = SelectOption::default();
            let mut pattern = "";
            match shell_words::split(arg) {
                Ok(args) => {
                    for arg in args.iter() {
                        match arg.as_str() {
                            "--toggle=true" => options.toggle = true,
                            "--all=true" => options.all = true,
                            "--toggle=false" => options.toggle = false,
                            "--all=false" => options.all = false,
                            "--deselect=true" => options.reverse = true,
                            "--deselect=false" => options.reverse = false,
                            s => pattern = s,
                        }
                    }
                    Ok(Self::SelectFiles(pattern.to_string(), options))
                }
                Err(e) => Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: {}", arg, e),
                )),
            }
        } else if command == CMD_SET_MODE {
            Ok(Self::SetMode)
        } else if command == CMD_SUBPROCESS_FOREGROUND || command == CMD_SUBPROCESS_BACKGROUND {
            match shell_words::split(arg) {
                Ok(s) if !s.is_empty() => Ok(Self::SubProcess(s, command == "spawn")),
                Ok(_) => Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: No commands given", command),
                )),
                Err(e) => Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: {}", arg, e),
                )),
            }
        } else if command == CMD_SHOW_WORKERS {
            Ok(Self::ShowWorkers)
        } else if command == CMD_SORT {
            match arg {
                "reverse" => Ok(Self::SortReverse),
                arg => match SortType::parse(arg) {
                    Some(s) => Ok(Self::Sort(s)),
                    None => Err(JoshutoError::new(
                        JoshutoErrorKind::InvalidParameters,
                        format!("{}: Unknown option '{}'", command, arg),
                    )),
                },
            }
        } else if command == CMD_TAB_SWITCH {
            match arg.parse::<i32>() {
                Ok(s) => Ok(Self::TabSwitch(s)),
                Err(e) => Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: {}", command, e.to_string()),
                )),
            }
        } else if command == CMD_TAB_SWITCH_INDEX {
            match arg.parse::<u32>() {
                Ok(s) => Ok(Self::TabSwitchIndex(s)),
                Err(e) => Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: {}", command, e.to_string()),
                )),
            }
        } else if command == CMD_TOGGLE_HIDDEN {
            Ok(Self::ToggleHiddenFiles)
        } else if command == CMD_HELP {
            Ok(Self::Help)
        } else if command == CMD_TOUCH_FILE {
            Ok(Self::TouchFile(arg.to_string()))
        } else {
            Err(JoshutoError::new(
                JoshutoErrorKind::UnrecognizedCommand,
                format!("Unrecognized command '{}'", command),
            ))
        }
    }
}
