use std::path;

use crate::commands::case_sensitivity::SetType;
use crate::commands::quit::QuitAction;
use crate::commands::select::SelectOption;
use crate::config::clean::app::display::line_mode::LineMode;
use crate::config::clean::app::display::line_number::LineNumberStyle;
use crate::config::clean::app::display::new_tab::NewTabMode;
use crate::config::clean::app::display::sort_type::SortType;
use crate::config::clean::app::search::CaseSensitivity;
use crate::error::{AppError, AppErrorKind};
use crate::io::FileOperationOptions;
use crate::util::unix;

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
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix(':') {
            return Ok(Self::CommandLine {
                prefix: stripped.to_owned(),
                suffix: "".to_owned(),
            });
        }

        let (command, arg) = match s.find(' ') {
            Some(i) => (&s[..i], s[i..].trim_start()),
            None => (s, ""),
        };

        simple_command_conversion_case!(command, CMD_ESCAPE, Self::Escape);

        simple_command_conversion_case!(command, CMD_TOGGLE_VISUAL, Self::ToggleVisualMode);

        simple_command_conversion_case!(command, CMD_CLOSE_TAB, Self::CloseTab);

        simple_command_conversion_case!(command, CMD_HELP, Self::Help);

        simple_command_conversion_case!(command, CMD_BOOKMARK_ADD, Self::BookmarkAdd);
        simple_command_conversion_case!(
            command,
            CMD_BOOKMARK_CHANGE_DIRECTORY,
            Self::BookmarkChangeDirectory
        );

        simple_command_conversion_case!(command, CMD_CURSOR_MOVE_HOME, Self::CursorMoveHome);
        simple_command_conversion_case!(command, CMD_CURSOR_MOVE_END, Self::CursorMoveEnd);
        simple_command_conversion_case!(
            command,
            CMD_CURSOR_MOVE_PAGEHOME,
            Self::CursorMovePageHome
        );
        simple_command_conversion_case!(
            command,
            CMD_CURSOR_MOVE_PAGEMIDDLE,
            Self::CursorMovePageMiddle
        );
        simple_command_conversion_case!(command, CMD_CURSOR_MOVE_PAGEEND, Self::CursorMovePageEnd);

        simple_command_conversion_case!(command, CMD_CUT_FILES, Self::CutFiles);
        simple_command_conversion_case!(command, CMD_COPY_FILES, Self::CopyFiles);
        simple_command_conversion_case!(command, CMD_COPY_FILENAME, Self::CopyFileName);
        simple_command_conversion_case!(
            command,
            CMD_COPY_FILENAME_WITHOUT_EXTENSION,
            Self::CopyFileNameWithoutExtension
        );
        // simple_command_conversion_case!(command, CMD_COPY_FILEPATH, Self::CopyFilePath);
        simple_command_conversion_case!(command, CMD_COPY_DIRECTORY_PATH, Self::CopyDirPath);

        simple_command_conversion_case!(command, CMD_OPEN_FILE, Self::OpenFile);

        simple_command_conversion_case!(command, CMD_RELOAD_DIRECTORY_LIST, Self::ReloadDirList);
        simple_command_conversion_case!(command, CMD_RENAME_FILE_APPEND, Self::RenameFileAppend);
        simple_command_conversion_case!(
            command,
            CMD_RENAME_FILE_APPEND_BASE,
            Self::RenameFileAppendBase
        );
        simple_command_conversion_case!(command, CMD_RENAME_FILE_PREPEND, Self::RenameFilePrepend);
        simple_command_conversion_case!(command, CMD_RENAME_FILE_KEEP_EXT, Self::RenameFileKeepExt);
        simple_command_conversion_case!(command, CMD_SEARCH_NEXT, Self::SearchNext);
        simple_command_conversion_case!(command, CMD_SEARCH_PREV, Self::SearchPrev);
        simple_command_conversion_case!(command, CMD_SHOW_TASKS, Self::ShowTasks);
        simple_command_conversion_case!(command, CMD_SET_MODE, Self::SetMode);
        simple_command_conversion_case!(command, CMD_TOGGLE_HIDDEN, Self::ToggleHiddenFiles);
        simple_command_conversion_case!(command, CMD_BULK_RENAME, Self::BulkRename);

        simple_command_conversion_case!(command, CMD_SEARCH_FZF, Self::SearchFzf);
        simple_command_conversion_case!(
            command,
            CMD_CUSTOM_SEARCH,
            Self::CustomSearch(arg.split(' ').map(|x| x.to_string()).collect())
        );
        simple_command_conversion_case!(
            command,
            CMD_CUSTOM_SEARCH_INTERACTIVE,
            Self::CustomSearchInteractive(arg.split(' ').map(|x| x.to_string()).collect())
        );
        simple_command_conversion_case!(command, CMD_SUBDIR_FZF, Self::SubdirFzf);
        simple_command_conversion_case!(command, CMD_ZOXIDE, Self::Zoxide(arg.to_string()));
        simple_command_conversion_case!(command, CMD_ZOXIDE_INTERACTIVE, Self::ZoxideInteractive);

        if command == CMD_QUIT {
            match arg {
                "--force" => Ok(Self::Quit(QuitAction::Force)),
                "--output-current-directory" => Ok(Self::Quit(QuitAction::OutputCurrentDirectory)),
                "--output-selected-files" => Ok(Self::Quit(QuitAction::OutputSelectedFiles)),
                _ => Ok(Self::Quit(QuitAction::Noop)),
            }
        } else if command == CMD_NEW_TAB {
            Ok(Self::NewTab {
                mode: NewTabMode::from_str(arg),
            })
        } else if command == CMD_CHANGE_DIRECTORY {
            match arg {
                "" => match HOME_DIR.as_ref() {
                    Some(s) => Ok(Self::ChangeDirectory { path: s.clone() }),
                    None => Err(AppError::new(
                        AppErrorKind::EnvVarNotPresent,
                        format!("{}: Cannot find home directory", command),
                    )),
                },
                ".." => Ok(Self::ParentDirectory),
                "-" => Ok(Self::PreviousDirectory),
                arg => {
                    let new_path = unix::expand_shell_string(arg);
                    Ok(Self::ChangeDirectory { path: new_path })
                }
            }
        } else if command == CMD_CURSOR_MOVE_DOWN {
            match arg {
                "" => Ok(Self::CursorMoveDown { offset: 1 }),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveDown { offset: s }),
                    Err(e) => Err(AppError::new(AppErrorKind::ParseError, e.to_string())),
                },
            }
        } else if command == CMD_CURSOR_MOVE_PAGEUP {
            let p = arg.trim().parse::<f64>().unwrap_or(1.);
            Ok(Self::CursorMovePageUp(p))
        } else if command == CMD_CURSOR_MOVE_PAGEDOWN {
            let p = arg.trim().parse::<f64>().unwrap_or(1.);
            Ok(Self::CursorMovePageDown(p))
        } else if command == CMD_CURSOR_MOVE_UP {
            match arg {
                "" => Ok(Self::CursorMoveUp { offset: 1 }),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveUp { offset: s }),
                    Err(e) => Err(AppError::new(AppErrorKind::ParseError, e.to_string())),
                },
            }
        } else if command == CMD_PARENT_CURSOR_MOVE_DOWN {
            match arg {
                "" => Ok(Self::ParentCursorMoveDown { offset: 1 }),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::ParentCursorMoveDown { offset: s }),
                    Err(e) => Err(AppError::new(AppErrorKind::ParseError, e.to_string())),
                },
            }
        } else if command == CMD_PARENT_CURSOR_MOVE_UP {
            match arg {
                "" => Ok(Self::ParentCursorMoveUp { offset: 1 }),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::ParentCursorMoveUp { offset: s }),
                    Err(e) => Err(AppError::new(AppErrorKind::ParseError, e.to_string())),
                },
            }
        } else if command == CMD_PREVIEW_CURSOR_MOVE_DOWN {
            match arg {
                "" => Ok(Self::PreviewCursorMoveDown { offset: 1 }),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::PreviewCursorMoveDown { offset: s }),
                    Err(e) => Err(AppError::new(AppErrorKind::ParseError, e.to_string())),
                },
            }
        } else if command == CMD_PREVIEW_CURSOR_MOVE_UP {
            match arg {
                "" => Ok(Self::PreviewCursorMoveUp { offset: 1 }),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::PreviewCursorMoveUp { offset: s }),
                    Err(e) => Err(AppError::new(AppErrorKind::ParseError, e.to_string())),
                },
            }
        } else if command == CMD_NEW_DIRECTORY {
            if arg.is_empty() {
                Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: no directory name given", command),
                ))
            } else {
                let path = path::PathBuf::from(arg);
                Ok(Self::NewDirectory { path })
            }
        } else if command == CMD_OPEN_FILE_WITH {
            match arg {
                "" => Ok(Self::OpenFileWith { index: None }),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::OpenFileWith { index: Some(s) }),
                    Err(e) => Err(AppError::new(AppErrorKind::ParseError, e.to_string())),
                },
            }
        } else if command == CMD_SYMLINK_FILES {
            let mut relative = false;
            for arg in arg.split_whitespace() {
                match arg {
                    "--relative=true" => relative = true,
                    "--relative=false" => relative = false,
                    _ => {
                        return Err(AppError::new(
                            AppErrorKind::UnrecognizedArgument,
                            format!("{}: unknown option '{}'", command, arg),
                        ));
                    }
                }
            }
            Ok(Self::SymlinkFiles { relative })
        } else if command == CMD_COPY_FILEPATH {
            let mut all_selected = false;
            for arg in arg.split_whitespace() {
                match arg {
                    "--all-selected=true" => all_selected = true,
                    "" => all_selected = false,
                    _ => {
                        return Err(AppError::new(
                            AppErrorKind::UnrecognizedArgument,
                            format!("{}: unknown option '{}'", command, arg),
                        ));
                    }
                }
            }
            Ok(Self::CopyFilePath { all_selected })
        } else if command == CMD_PASTE_FILES {
            let mut options = FileOperationOptions::default();
            for arg in arg.split_whitespace() {
                match arg {
                    "--overwrite=true" => options.overwrite = true,
                    "--skip_exist=true" => options.skip_exist = true,
                    "--overwrite=false" => options.overwrite = false,
                    "--skip_exist=false" => options.skip_exist = false,
                    _ => {
                        return Err(AppError::new(
                            AppErrorKind::UnrecognizedArgument,
                            format!("{}: unknown option '{}'", command, arg),
                        ));
                    }
                }
            }
            Ok(Self::PasteFiles { options })
        } else if command == CMD_DELETE_FILES {
            let [mut permanently, mut background, mut noconfirm] = [false; 3];
            for arg in arg.split_whitespace() {
                match arg {
                    "--background=true" => background = true,
                    "--background=false" => background = false,
                    "--permanently" => permanently = true,
                    "--noconfirm" => noconfirm = true,
                    _ => {
                        return Err(AppError::new(
                            AppErrorKind::UnrecognizedArgument,
                            format!("{}: unknown option '{}'", command, arg),
                        ))
                    }
                }
            }
            Ok(Self::DeleteFiles {
                background,
                permanently,
                noconfirm,
            })
        } else if command == CMD_RENAME_FILE {
            match arg {
                "" => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => {
                    let path: path::PathBuf = path::PathBuf::from(arg);
                    Ok(Self::RenameFile { new_name: path })
                }
            }
        } else if command == CMD_SEARCH_STRING {
            match arg {
                "" => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => Ok(Self::SearchString {
                    pattern: arg.to_string(),
                }),
            }
        } else if command == CMD_SEARCH_INCREMENTAL {
            Ok(Self::SearchIncremental {
                pattern: arg.to_string(),
            })
        } else if command == CMD_SEARCH_GLOB {
            match arg {
                "" => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => Ok(Self::SearchGlob {
                    pattern: arg.to_string(),
                }),
            }
        } else if command == CMD_SEARCH_REGEX {
            match arg {
                "" => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => Ok(Self::SearchRegex {
                    pattern: arg.to_string(),
                }),
            }
        } else if command == CMD_SELECT_GLOB {
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
                    if pattern.is_empty() {
                        return Err(AppError::new(
                            AppErrorKind::InvalidParameters,
                            format!("{}: Expected 1, got 0", command),
                        ));
                    }
                    Ok(Self::SelectGlob {
                        pattern: pattern.to_string(),
                        options,
                    })
                }
                Err(e) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: {}", arg, e),
                )),
            }
        } else if command == CMD_SELECT_REGEX {
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
                    if pattern.is_empty() {
                        return Err(AppError::new(
                            AppErrorKind::InvalidParameters,
                            format!("{}: Expected 1, got 0", command),
                        ));
                    }
                    Ok(Self::SelectRegex {
                        pattern: pattern.to_string(),
                        options,
                    })
                }
                Err(e) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: {}", arg, e),
                )),
            }
        } else if command == CMD_SELECT_STRING {
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
                    Ok(Self::SelectString {
                        pattern: pattern.to_string(),
                        options,
                    })
                }
                Err(e) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: {}", arg, e),
                )),
            }
        } else if command == CMD_SELECT_FZF {
            let mut options = SelectOption::default();
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
                            _ => {}
                        }
                    }
                    Ok(Self::SelectFzf { options })
                }
                Err(e) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: {}", arg, e),
                )),
            }
        } else if command == CMD_SET_CASE_SENSITIVITY {
            match shell_words::split(arg) {
                Ok(args) => {
                    let mut set_type = SetType::String;
                    let mut value = "";

                    for arg in args.iter() {
                        match arg.as_str() {
                            "--type=string" => set_type = SetType::String,
                            "--type=glob" => set_type = SetType::Glob,
                            "--type=regex" => set_type = SetType::Regex,
                            "--type=fzf" => set_type = SetType::Fzf,
                            s => value = s,
                        }
                    }

                    match CaseSensitivity::from_str(value) {
                        Ok(case_sensitivity) => Ok(Self::SetCaseSensitivity {
                            case_sensitivity,
                            set_type,
                        }),
                        Err(e) => Err(AppError::new(
                            AppErrorKind::InvalidParameters,
                            format!("{}: {}", arg, e),
                        )),
                    }
                }
                Err(e) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: {}", arg, e),
                )),
            }
        } else if command == CMD_SUBPROCESS_FOREGROUND || command == CMD_SUBPROCESS_BACKGROUND {
            match shell_words::split(arg) {
                Ok(s) if !s.is_empty() => Ok(Self::SubProcess {
                    words: s,
                    spawn: command == "spawn",
                }),
                Ok(_) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: No commands given", command),
                )),
                Err(e) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: {}", arg, e),
                )),
            }
        } else if command == CMD_SORT {
            match arg {
                "reverse" => Ok(Self::SortReverse),
                arg => match SortType::from_str(arg) {
                    Some(s) => Ok(Self::Sort(s)),
                    None => Err(AppError::new(
                        AppErrorKind::InvalidParameters,
                        format!("{}: Unknown option '{}'", command, arg),
                    )),
                },
            }
        } else if command == CMD_SET_LINEMODE {
            Ok(Self::SetLineMode(LineMode::from_string(arg)?))
        } else if command == CMD_TAB_SWITCH {
            match arg.parse::<i32>() {
                Ok(s) => Ok(Self::TabSwitch { offset: s }),
                Err(e) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: {}", command, e),
                )),
            }
        } else if command == CMD_TAB_SWITCH_INDEX {
            match arg.parse::<usize>() {
                Ok(s) => Ok(Self::TabSwitchIndex { index: s }),
                Err(e) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: {}", command, e),
                )),
            }
        } else if command == CMD_TOUCH_FILE {
            Ok(Self::TouchFile {
                file_name: arg.to_string(),
            })
        } else if command == CMD_SWITCH_LINE_NUMBERS {
            let policy = match arg {
                "absolute" | "1" => LineNumberStyle::Absolute,
                "relative" | "2" => LineNumberStyle::Relative,
                _ => LineNumberStyle::None,
            };
            Ok(Self::SwitchLineNums(policy))
        } else if command == CMD_FLAT {
            match arg.parse::<usize>() {
                Ok(i) => Ok(Self::Flat { depth: i }),
                Err(e) => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: {}", command, e),
                )),
            }
        } else if command == CMD_NUMBERED_COMMAND {
            let c = arg.chars().next();
            match c {
                Some(c) => Ok(Self::NumberedCommand { initial: c }),
                None => Err(AppError::new(
                    AppErrorKind::InvalidParameters,
                    format!("{}: no starting character given", command),
                )),
            }
        } else if command == CMD_FILTER_GLOB {
            Ok(Self::FilterGlob {
                pattern: arg.to_string(),
            })
        } else if command == CMD_FILTER_REGEX {
            Ok(Self::FilterRegex {
                pattern: arg.to_string(),
            })
        } else if command == CMD_FILTER_STRING {
            Ok(Self::FilterString {
                pattern: arg.to_string(),
            })
        } else {
            Err(AppError::new(
                AppErrorKind::UnrecognizedCommand,
                format!("Unrecognized command '{}'", command),
            ))
        }
    }
}
