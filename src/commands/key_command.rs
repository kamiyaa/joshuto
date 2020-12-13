use std::path;

use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::io::IOWorkerOptions;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;
use crate::util::sort::SortType;

use crate::HOME_DIR;

use super::*;

#[derive(Clone, Debug)]
pub enum KeyCommand {
    BulkRename,
    ChangeDirectory(path::PathBuf),
    CommandLine(String, String),

    CutFiles,
    CopyFiles,
    PasteFiles(IOWorkerOptions),

    CursorMoveUp(usize),
    CursorMoveDown(usize),
    CursorMoveHome,
    CursorMoveEnd,
    CursorMovePageUp,
    CursorMovePageDown,

    ParentCursorMoveUp(usize),
    ParentCursorMoveDown(usize),

    DeleteFiles,
    NewDirectory(path::PathBuf),
    OpenFile,
    OpenFileWith,
    ParentDirectory,

    Quit,
    ForceQuit,
    ReloadDirList,
    RenameFile(path::PathBuf),
    RenameFileAppend,
    RenameFilePrepend,

    Search(String),
    SearchNext,
    SearchPrev,

    SelectFiles { toggle: bool, all: bool },
    SetMode,
    ShellCommand(Vec<String>),
    ShowWorkers,

    ToggleHiddenFiles,

    Sort(SortType),
    SortReverse,

    NewTab,
    CloseTab,
    TabSwitch(i32),
}

impl KeyCommand {
    pub fn command(&self) -> &'static str {
        match self {
            Self::BulkRename => "bulk_rename",
            Self::ChangeDirectory(_) => "cd",
            Self::NewTab => "new_tab",
            Self::CloseTab => "close_tab",
            Self::CommandLine(_, _) => "console",

            Self::CutFiles => "cut_files",
            Self::CopyFiles => "copy_files",
            Self::PasteFiles(_) => "paste_files",

            Self::CursorMoveUp(_) => "cursor_move_up",
            Self::CursorMoveDown(_) => "cursor_move_down",
            Self::CursorMoveHome => "cursor_move_home",
            Self::CursorMoveEnd => "cursor_move_end",
            Self::CursorMovePageUp => "cursor_move_page_up",
            Self::CursorMovePageDown => "cursor_move_page_down",

            Self::ParentCursorMoveUp(_) => "parent_cursor_move_up",
            Self::ParentCursorMoveDown(_) => "parent_cursor_move_down",

            Self::DeleteFiles => "delete_files",
            Self::NewDirectory(_) => "new_directory",
            Self::OpenFile => "open",
            Self::OpenFileWith => "open_with",
            Self::ParentDirectory => "cd ..",

            Self::Quit => "quit",
            Self::ForceQuit => "force_quit",
            Self::ReloadDirList => "reload_dirlist",
            Self::RenameFile(_) => "rename_file",
            Self::RenameFileAppend => "rename_append",
            Self::RenameFilePrepend => "rename_prepend",

            Self::Search(_) => "search",
            Self::SearchNext => "search_next",
            Self::SearchPrev => "search_prev",

            Self::SelectFiles { toggle: _, all: _ } => "select",
            Self::SetMode => "set_mode",
            Self::ShellCommand(_) => "shell",
            Self::ShowWorkers => "show_workers",

            Self::ToggleHiddenFiles => "toggle_hidden",

            Self::Sort(_) => "sort",
            Self::SortReverse => "sort reverse",

            Self::TabSwitch(_) => "tab_switch",
        }
    }

    pub fn parse_command(s: &str) -> JoshutoResult<Self> {
        let (command, arg) = match s.find(' ') {
            Some(i) => (&s[..i], s[i + 1..].trim_start()),
            None => (s, ""),
        };

        match command {
            "bulk_rename" => Ok(Self::BulkRename),
            "cd" => match arg {
                "" => match HOME_DIR.as_ref() {
                    Some(s) => Ok(Self::ChangeDirectory(s.clone())),
                    None => Err(JoshutoError::new(
                        JoshutoErrorKind::EnvVarNotPresent,
                        format!("{}: Cannot find home directory", command),
                    )),
                },
                ".." => Ok(Self::ParentDirectory),
                arg => Ok(Self::ChangeDirectory(path::PathBuf::from(arg))),
            },
            "close_tab" => Ok(Self::CloseTab),
            "copy_files" => Ok(Self::CopyFiles),
            "console" => Ok(Self::CommandLine(arg.to_owned(), "".to_owned())),
            "cursor_move_home" => Ok(Self::CursorMoveHome),
            "cursor_move_end" => Ok(Self::CursorMoveEnd),
            "cursor_move_page_up" => Ok(Self::CursorMovePageUp),
            "cursor_move_page_down" => Ok(Self::CursorMovePageDown),
            "cursor_move_down" => match arg {
                "" => Ok(Self::CursorMoveDown(1)),
                arg => match arg.parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveDown(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            },
            "cursor_move_up" => match arg {
                "" => Ok(Self::CursorMoveUp(1)),
                arg => match arg.parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveUp(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            },
            "parent_cursor_move_down" => match arg {
                "" => Ok(Self::ParentCursorMoveDown(1)),
                arg => match arg.parse::<usize>() {
                    Ok(s) => Ok(Self::ParentCursorMoveDown(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            },
            "parent_cursor_move_up" => match arg {
                "" => Ok(Self::ParentCursorMoveUp(1)),
                arg => match arg.parse::<usize>() {
                    Ok(s) => Ok(Self::ParentCursorMoveUp(s)),
                    Err(e) => Err(JoshutoError::new(
                        JoshutoErrorKind::ParseError,
                        e.to_string(),
                    )),
                },
            },
            "cut_files" => Ok(Self::CutFiles),
            "delete_files" => Ok(Self::DeleteFiles),
            "force_quit" => Ok(Self::ForceQuit),
            "mkdir" => match arg {
                "" => Err(JoshutoError::new(
                    JoshutoErrorKind::IOInvalidData,
                    format!("{}: missing additional parameter", command),
                )),
                arg => Ok(Self::NewDirectory(path::PathBuf::from(arg))),
            },
            "new_tab" => Ok(Self::NewTab),

            "open_file" => Ok(Self::OpenFile),
            "open_file_with" => Ok(Self::OpenFileWith),
            "paste_files" => {
                let mut options = IOWorkerOptions::default();
                for arg in arg.split_whitespace() {
                    match arg {
                        "--overwrite" => options.overwrite = true,
                        "--skip_exist" => options.skip_exist = true,
                        _ => {
                            return Err(JoshutoError::new(
                                JoshutoErrorKind::IOInvalidData,
                                format!("{}: unknown option {}", command, arg),
                            ));
                        }
                    }
                }
                Ok(Self::PasteFiles(options))
            }
            "quit" => Ok(Self::Quit),
            "reload_dir_list" => Ok(Self::ReloadDirList),
            "rename" => match arg {
                "" => Err(JoshutoError::new(
                    JoshutoErrorKind::IOInvalidData,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => {
                    let path: path::PathBuf = path::PathBuf::from(arg);
                    Ok(Self::RenameFile(path))
                }
            },
            "rename_append" => Ok(Self::RenameFileAppend),
            "rename_prepend" => Ok(Self::RenameFilePrepend),
            "search" => match arg {
                "" => Err(JoshutoError::new(
                    JoshutoErrorKind::IOInvalidData,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => Ok(Self::Search(arg.to_string())),
            },
            "search_next" => Ok(Self::SearchNext),
            "search_prev" => Ok(Self::SearchPrev),
            "select_files" => {
                let mut toggle = false;
                let mut all = false;
                for arg in arg.split_whitespace() {
                    match arg {
                        "--toggle" => toggle = true,
                        "--all" => all = true,
                        _ => {
                            return Err(JoshutoError::new(
                                JoshutoErrorKind::IOInvalidData,
                                format!("{}: unknown option {}", command, arg),
                            ));
                        }
                    }
                }
                Ok(Self::SelectFiles { toggle, all })
            }
            "set_mode" => Ok(Self::SetMode),
            "shell" => match shell_words::split(arg) {
                Ok(s) if !s.is_empty() => Ok(Self::ShellCommand(s)),
                Ok(_) => Err(JoshutoError::new(
                    JoshutoErrorKind::IOInvalidData,
                    format!("sort: args {}", arg),
                )),
                Err(e) => Err(JoshutoError::new(
                    JoshutoErrorKind::IOInvalidData,
                    format!("{}: {}", arg, e),
                )),
            },
            "show_workers" => Ok(Self::ShowWorkers),
            "sort" => match arg {
                "reverse" => Ok(Self::SortReverse),
                arg => match SortType::parse(arg) {
                    Some(s) => Ok(Self::Sort(s)),
                    None => Err(JoshutoError::new(
                        JoshutoErrorKind::IOInvalidData,
                        format!("sort: Unknown option {}", arg),
                    )),
                },
            },
            "tab_switch" => match arg.parse::<i32>() {
                Ok(s) => Ok(Self::TabSwitch(s)),
                Err(e) => Err(JoshutoError::new(
                    JoshutoErrorKind::IOInvalidData,
                    format!("{}: {}", command, e.to_string()),
                )),
            },
            "toggle_hidden" => Ok(Self::ToggleHiddenFiles),
            inp => Err(JoshutoError::new(
                JoshutoErrorKind::UnknownCommand,
                format!("Unknown command: {}", inp),
            )),
        }
    }
}

impl JoshutoRunnable for KeyCommand {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        match &*self {
            Self::BulkRename => bulk_rename::bulk_rename(context, backend),
            Self::ChangeDirectory(p) => {
                change_directory::change_directory(context, p.as_path())?;
                LoadChild::load_child(context)?;
                Ok(())
            }
            Self::NewTab => tab_ops::new_tab(context),
            Self::CloseTab => tab_ops::close_tab(context),
            Self::CommandLine(p, s) => {
                command_line::readline(context, backend, p.as_str(), s.as_str())
            }
            Self::CutFiles => file_ops::cut(context),
            Self::CopyFiles => file_ops::copy(context),
            Self::PasteFiles(options) => file_ops::paste(context, options.clone()),

            Self::CursorMoveUp(u) => cursor_move::up(context, *u),
            Self::CursorMoveDown(u) => cursor_move::down(context, *u),
            Self::CursorMoveHome => cursor_move::home(context),
            Self::CursorMoveEnd => cursor_move::end(context),
            Self::CursorMovePageUp => cursor_move::page_up(context, backend),
            Self::CursorMovePageDown => cursor_move::page_down(context, backend),

            Self::ParentCursorMoveUp(u) => parent_cursor_move::parent_up(context, *u),
            Self::ParentCursorMoveDown(u) => parent_cursor_move::parent_down(context, *u),

            Self::DeleteFiles => {
                delete_files::delete_selected_files(context, backend)?;
                Ok(())
            }
            Self::NewDirectory(p) => new_directory::new_directory(context, p.as_path()),
            Self::OpenFile => open_file::open(context, backend),
            Self::OpenFileWith => open_file::open_with(context, backend),
            Self::ParentDirectory => parent_directory::parent_directory(context),

            Self::Quit => quit::quit(context),
            Self::ForceQuit => quit::force_quit(context),
            Self::ReloadDirList => reload::reload_dirlist(context),
            Self::RenameFile(p) => rename_file::rename_file(context, p.as_path()),
            Self::RenameFileAppend => rename_file::rename_file_append(context, backend),
            Self::RenameFilePrepend => rename_file::rename_file_prepend(context, backend),
            Self::Search(pattern) => search::search(context, pattern.as_str()),
            Self::SearchNext => search::search_next(context),
            Self::SearchPrev => search::search_prev(context),

            Self::SelectFiles { toggle, all } => selection::select_files(context, *toggle, *all),
            Self::SetMode => set_mode::set_mode(context, backend),
            Self::ShellCommand(v) => shell::shell(context, backend, v.as_slice()),
            Self::ShowWorkers => show_workers::show_workers(context, backend),

            Self::ToggleHiddenFiles => show_hidden::toggle_hidden(context),

            Self::Sort(t) => sort::set_sort(context, *t),
            Self::SortReverse => sort::toggle_reverse(context),

            Self::TabSwitch(i) => {
                tab_ops::tab_switch(*i, context)?;
                Ok(())
            }
        }
    }
}

impl std::fmt::Display for KeyCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Self::ChangeDirectory(p) => write!(f, "{}    {:?}", self.command(), p),
            Self::CommandLine(s, p) => write!(f, "{} {} {}", self.command(), s, p),
            Self::PasteFiles(options) => write!(f, "{}    {}", self.command(), options),
            Self::CursorMoveUp(i) => write!(f, "{} {}", self.command(), i),
            Self::CursorMoveDown(i) => write!(f, "{} {}", self.command(), i),
            Self::NewDirectory(d) => write!(f, "{} {:?}", self.command(), d),
            Self::RenameFile(name) => write!(f, "{} {:?}", self.command(), name),

            Self::Search(s) => write!(f, "{} {}", self.command(), s),
            Self::SelectFiles { toggle, all } => {
                write!(f, "{} toggle={} all={}", self.command(), toggle, all)
            }
            Self::ShellCommand(c) => write!(f, "{} {:?}", self.command(), c),
            Self::Sort(t) => write!(f, "{} {}", self.command(), t),
            Self::TabSwitch(i) => write!(f, "{} {}", self.command(), i),
            _ => write!(f, "{}", self.command()),
        }
    }
}

impl JoshutoCommand for KeyCommand {}
