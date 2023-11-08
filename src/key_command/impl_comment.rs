use crate::{
    config::clean::app::display::{line_mode::LineMode, sort_type::SortType},
    io::FileOperationOptions,
};

use super::{Command, CommandComment};

impl CommandComment for Command {
    // These comments are displayed at the help page
    fn comment(&self) -> &'static str {
        match self {
            Self::SetLineMode(linemode) => match *linemode {
                LineMode::size => "Show files with size",
                LineMode::mtime => "Show files with modified time",
                LineMode::user => "Show files with user",
                LineMode::group => "Show files with group",
                LineMode::perm => "Show files with permission",
                _ => "Show files with multi-attribution",
            },
            Self::Escape => "Escape from visual mode (cancel)",
            Self::BulkRename => "Bulk rename",

            Self::ToggleVisualMode => "Toggle visual mode",

            Self::ChangeDirectory { .. } => "Change directory",
            Self::ParentDirectory => "CD to parent directory",
            Self::PreviousDirectory => "CD to the last dir in history",

            Self::NewTab { .. } => "Open a new tab",
            Self::CloseTab => "Close current tab",
            Self::CommandLine { prefix, .. } => match prefix.trim() {
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
            Self::CopyFileName => "Copy filename",
            Self::CopyFileNameWithoutExtension => "Copy filename without extension",
            Self::CopyFilePath { all_selected: true } => "Copy all selected paths to file",
            Self::CopyFilePath { .. } => "Copy path to file",
            Self::CopyDirPath => "Copy directory name",
            Self::SymlinkFiles { .. } => "Symlink selected files",

            Self::PasteFiles {
                options:
                    FileOperationOptions {
                        overwrite,
                        skip_exist,
                        ..
                    },
            } => match (overwrite, skip_exist) {
                (true, false) => "Paste, overwrite",
                (false, true) => "Paste, skip existing files",
                _ => "Paste",
            },
            Self::DeleteFiles { .. } => "Delete selected files",

            Self::CursorMoveUp { .. } => "Move cursor up",
            Self::CursorMoveDown { .. } => "Move cursor down",
            Self::CursorMoveHome => "Move cursor to the very top",
            Self::CursorMoveEnd => "Move cursor to the ver bottom",
            Self::CursorMovePageUp(_) => "Move cursor one page up",
            Self::CursorMovePageDown(_) => "Move cursor one page down",

            Self::CursorMovePageHome => "Move cursor to top of page",
            Self::CursorMovePageMiddle => "Move cursor to middle of page",
            Self::CursorMovePageEnd => "Move cursor to bottom of page",

            Self::ParentCursorMoveUp { .. } => "Cursor up in parent list",
            Self::ParentCursorMoveDown { .. } => "Cursor down in parent list",

            Self::PreviewCursorMoveUp { .. } => "Cursor up in file preview",
            Self::PreviewCursorMoveDown { .. } => "Cursor down in file preview",

            Self::NewDirectory { .. } => "Make a new directory",
            Self::OpenFile => "Open a file",
            Self::OpenFileWith { .. } => "Open using selected program",

            Self::Quit(_) => "Quit the program",
            Self::ReloadDirList => "Reload current dir listing",
            Self::RenameFile { .. } => "Rename file",
            Self::TouchFile { .. } => "Touch file",
            Self::RenameFileAppend => "Rename a file",
            Self::RenameFileAppendBase => "Rename a file",
            Self::RenameFilePrepend => "Rename a file",
            Self::RenameFileKeepExt => "Rename a file",

            Self::SearchString { .. } => "Search",
            Self::SearchIncremental { .. } => "Search as you type",
            Self::SearchGlob { .. } => "Search with globbing",
            Self::SearchRegex { .. } => "Search with regex",
            Self::SearchNext => "Next search entry",
            Self::SearchPrev => "Previous search entry",

            Self::SelectGlob { .. } => "Select files with globbing",
            Self::SelectRegex { .. } => "Select files with regex",
            Self::SelectString { .. } => "Select files",

            Self::SetCaseSensitivity { .. } => "Set case sensitivity",
            Self::SetMode => "Set file permissions",
            Self::SubProcess { spawn: false, .. } => "Run a shell command",
            Self::SubProcess { spawn: true, .. } => "Run command in background",
            Self::ShowTasks => "Show running background tasks",

            Self::ToggleHiddenFiles => "Toggle hidden files displaying",

            Self::SwitchLineNums(_) => "Switch line numbering",

            Self::Flat { .. } => "Flattern directory list",
            Self::NumberedCommand { .. } => "Jump via input number",

            Self::Sort(sort_type) => match sort_type {
                SortType::Lexical => "Sort lexically",
                SortType::Mtime => "Sort by modification time",
                SortType::Natural => "Sort naturally",
                SortType::Size => "Sort by size",
                SortType::Ext => "Sort by extension",
            },
            Self::SortReverse => "Reverse sort order",

            Self::FilterGlob { .. } => "Filter directory list with globbing",
            Self::FilterRegex { .. } => "Filter directory list with regex",
            Self::FilterString { .. } => "Filter directory list",

            Self::TabSwitch { .. } => "Switch to the next tab",
            Self::TabSwitchIndex { .. } => "Switch to a given tab",
            Self::Help => "Open this help page",

            Self::SearchFzf => "Search via fzf",
            Self::SubdirFzf => "Switch to a child directory via fzf",
            Self::SelectFzf { .. } => "Select via fzf",
            Self::Zoxide(_) => "Zoxide",
            Self::ZoxideInteractive => "Zoxide interactive",

            Self::BookmarkAdd => "Add a bookmark",
            Self::BookmarkChangeDirectory => "Navigate to a bookmark",
            Self::CustomSearch(_) => "Find file based on the custom command",
            Self::CustomSearchInteractive(_) => {
                "Interactively find file based on the custom command"
            }
        }
    }
}
