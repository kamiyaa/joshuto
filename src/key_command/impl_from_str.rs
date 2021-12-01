use std::path;

use dirs_next::home_dir;
use shellexpand::tilde_with_context;

use crate::config::option::{SelectOption, SortType};
use crate::error::{JoshutoError, JoshutoErrorKind};
use crate::io::IoWorkerOptions;

use crate::HOME_DIR;

use super::constants::*;
use super::Command;

macro_rules! simple_command_conversion_case {
    ($command: ident, $command_match: ident, $enum_name: expr) => {
        if $command == $command_match {
            return Ok($enum_name);
        }
    };
}

impl std::str::FromStr for Command {
    type Err = JoshutoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix(':') {
            return Ok(Self::CommandLine(stripped.to_owned(), "".to_owned()));
        }

        let (command, arg) = match s.find(' ') {
            Some(i) => (&s[..i], s[i..].trim_start()),
            None => (s, ""),
        };

        simple_command_conversion_case!(command, CMD_QUIT, Self::Quit);
        simple_command_conversion_case!(
            command,
            CMD_QUIT_TO_CURRENT_DIRECTORY,
            Self::QuitToCurrentDirectory
        );
        simple_command_conversion_case!(command, CMD_FORCE_QUIT, Self::ForceQuit);

        simple_command_conversion_case!(command, CMD_NEW_TAB, Self::NewTab);
        simple_command_conversion_case!(command, CMD_CLOSE_TAB, Self::CloseTab);

        simple_command_conversion_case!(command, CMD_HELP, Self::Help);

        simple_command_conversion_case!(command, CMD_CURSOR_MOVE_HOME, Self::CursorMoveHome);
        simple_command_conversion_case!(command, CMD_CURSOR_MOVE_END, Self::CursorMoveEnd);
        simple_command_conversion_case!(command, CMD_CURSOR_MOVE_PAGEUP, Self::CursorMovePageUp);
        simple_command_conversion_case!(
            command,
            CMD_CURSOR_MOVE_PAGEDOWN,
            Self::CursorMovePageDown
        );

        simple_command_conversion_case!(command, CMD_CUT_FILES, Self::CutFiles);
        simple_command_conversion_case!(command, CMD_DELETE_FILES, Self::DeleteFiles);

        simple_command_conversion_case!(command, CMD_COPY_FILES, Self::CopyFiles);
        simple_command_conversion_case!(command, CMD_COPY_FILENAME, Self::CopyFileName);
        simple_command_conversion_case!(
            command,
            CMD_COPY_FILENAME_WITHOUT_EXTENSION,
            Self::CopyFileNameWithoutExtension
        );
        simple_command_conversion_case!(command, CMD_COPY_FILEPATH, Self::CopyFilePath);
        simple_command_conversion_case!(command, CMD_COPY_DIRECTORY_PATH, Self::CopyDirPath);

        simple_command_conversion_case!(command, CMD_OPEN_FILE, Self::OpenFile);

        simple_command_conversion_case!(command, CMD_RELOAD_DIRECTORY_LIST, Self::ReloadDirList);
        simple_command_conversion_case!(command, CMD_RENAME_FILE_APPEND, Self::RenameFileAppend);
        simple_command_conversion_case!(command, CMD_RENAME_FILE_PREPEND, Self::RenameFilePrepend);
        simple_command_conversion_case!(command, CMD_SEARCH_FZF, Self::SearchFzf);
        simple_command_conversion_case!(command, CMD_SEARCH_NEXT, Self::SearchNext);
        simple_command_conversion_case!(command, CMD_SEARCH_PREV, Self::SearchPrev);
        simple_command_conversion_case!(command, CMD_SUBDIR_FZF, Self::SubdirFzf);
        simple_command_conversion_case!(command, CMD_SHOW_WORKERS, Self::ShowWorkers);
        simple_command_conversion_case!(command, CMD_SET_MODE, Self::SetMode);
        simple_command_conversion_case!(command, CMD_TOGGLE_HIDDEN, Self::ToggleHiddenFiles);
        simple_command_conversion_case!(command, CMD_BULK_RENAME, Self::BulkRename);

        if command == CMD_CHANGE_DIRECTORY {
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
        } else if command == CMD_PREVIEW_CURSOR_MOVE_DOWN {
            match arg {
                "" => Ok(Self::PreviewCursorMoveDown(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::PreviewCursorMoveDown(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            }
        } else if command == CMD_PREVIEW_CURSOR_MOVE_UP {
            match arg {
                "" => Ok(Self::PreviewCursorMoveUp(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::PreviewCursorMoveUp(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            }
        } else if command == CMD_NEW_DIRECTORY {
            if arg.is_empty() {
                Err(JoshutoError::new(
                    JoshutoErrorKind::InvalidParameters,
                    format!("{}: no directory name given", command),
                ))
            } else {
                Ok(Self::NewDirectory(path::PathBuf::from(arg)))
            }
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
        } else if command == CMD_TOUCH_FILE {
            Ok(Self::TouchFile(arg.to_string()))
        } else if command == CMD_SWITCH_LINE_NUMBERS {
            let policy = match arg {
                "no" => 0,
                "absolute" => 1,
                "relative" => 2,
                s => match s.parse::<u8>() {
                    Ok(n) if (0..3).contains(&n) => n,
                    _ => {
                        return Err(JoshutoError::new(
                            JoshutoErrorKind::InvalidParameters,
                            format!("{}: {}", command, "Invalid argument. Must be 0/1/2"),
                        ))
                    }
                },
            };
            Ok(Self::SwitchLineNums(policy))
        } else {
            Err(JoshutoError::new(
                JoshutoErrorKind::UnrecognizedCommand,
                format!("Unrecognized command '{}'", command),
            ))
        }
    }
}
