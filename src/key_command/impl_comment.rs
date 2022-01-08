use crate::config::option::SortType;
use crate::io::IoWorkerOptions;

use super::{Command, CommandComment};

impl CommandComment for Command {
    // These comments are displayed at the help page
    fn comment(&self) -> &'static str {
        match self {
            Self::BulkRename => "Bulk rename",

            Self::ChangeDirectory(_) => "Change directory",
            Self::ParentDirectory => "CD to parent directory",
            Self::PreviousDirectory => "CD to the last dir in history",

            Self::NewTab => "Open a new tab",
            Self::CloseTab => "Close current tab",
            Self::CommandLine(command, _) => match command.trim() {
                "cd" => "Change directory",
                "search" => "Open a search prompt",
                "search_glob" => "Glob search",
                "rename" => "Rename selected file",
                "touch" => "Touch file",
                "mkdir" => "Make a new directory",
                _ => "Open a command line",
            },

            Self::CutFiles => "Cut selected files",
            Self::CopyFiles => "Copy selected files",
            Self::PasteFiles(IoWorkerOptions {
                overwrite,
                skip_exist,
            }) => match (overwrite, skip_exist) {
                (true, false) => "Paste, overwrite",
                (false, true) => "Paste, skip existing files",
                _ => "Paste",
            },
            Self::CopyFileName => "Copy filename",
            Self::CopyFileNameWithoutExtension => "Copy filename without extension",
            Self::CopyFilePath => "Copy path to file",
            Self::CopyDirPath => "Copy directory name",

            Self::CursorMoveUp(_) => "Move cursor up",
            Self::CursorMoveDown(_) => "Move cursor down",
            Self::CursorMoveHome => "Move cursor to the very top",
            Self::CursorMoveEnd => "Move cursor to the ver bottom",
            Self::CursorMovePageUp => "Move cursor one page up",
            Self::CursorMovePageDown => "Move cursor one page down",

            Self::ParentCursorMoveUp(_) => "Cursor up in parent list",
            Self::ParentCursorMoveDown(_) => "Cursor down in parent list",

            Self::PreviewCursorMoveUp(_) => "Cursor up in file preview",
            Self::PreviewCursorMoveDown(_) => "Cursor down in file preview",

            Self::DeleteFiles => "Delete selected files",
            Self::NewDirectory(_) => "Make a new directory",
            Self::OpenFile => "Open a file",
            Self::OpenFileWith(_) => "Open using selected program",

            Self::Quit => "Quit the program",
            Self::QuitToCurrentDirectory => "Quit to current directory",
            Self::ForceQuit => "Force quit",
            Self::ReloadDirList => "Reload current dir listing",
            Self::RenameFile(_) => "Rename file",
            Self::TouchFile(_) => "Touch file",
            Self::RenameFileAppend => "Rename a file",
            Self::RenameFilePrepend => "Rename a file",

            Self::SearchString(_) => "Search",
            Self::SearchGlob(_) => "Search with globbing",
            Self::SearchFzf => "Search via fzf",
            Self::SearchNext => "Next search entry",
            Self::SearchPrev => "Previous search entry",

            Self::SubdirFzf => "Switch to a child directory via fzf",

            Self::SelectFiles(_, _) => "Select file",
            Self::SetMode => "Set file permissions",
            Self::SubProcess(_, false) => "Run a shell command",
            Self::SubProcess(_, true) => "Run commmand in background",
            Self::ShowWorkers => "Show IO workers",

            Self::ToggleHiddenFiles => "Toggle hidden files displaying",

            Self::SwitchLineNums(_) => "Switch line numbering",

            Self::Sort(sort_type) => match sort_type {
                SortType::Lexical => "Sort lexically",
                SortType::Mtime => "Sort by modifiaction time",
                SortType::Natural => "Sort naturally",
                SortType::Size => "Sort by size",
                SortType::Ext => "Sort by extension",
            },
            Self::SortReverse => "Reverse sort order",

            Self::TabSwitch(_) => "Swith to the next tab",
            Self::TabSwitchIndex(_) => "Swith to a given tab",
            Self::Help => "Open this help page",
        }
    }
}
