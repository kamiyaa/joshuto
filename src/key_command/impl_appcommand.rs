use super::constants::*;
use super::{AppCommand, Command};

impl AppCommand for Command {
    fn command(&self) -> &'static str {
        match self {
            Self::Help => CMD_HELP,

            Self::Quit(_) => CMD_QUIT,

            Self::ToggleVisualMode => CMD_TOGGLE_VISUAL,
            Self::Escape => CMD_ESCAPE,

            Self::BulkRename => CMD_BULK_RENAME,

            Self::ChangeDirectory { .. } => CMD_CHANGE_DIRECTORY,
            Self::ParentDirectory => CMD_PARENT_DIRECTORY,
            Self::PreviousDirectory => CMD_PREVIOUS_DIRECTORY,

            Self::NewTab { .. } => CMD_NEW_TAB,
            Self::CloseTab => CMD_CLOSE_TAB,
            Self::CommandLine { .. } => CMD_COMMAND_LINE,

            Self::CutFiles => CMD_CUT_FILES,
            Self::CopyFiles => CMD_COPY_FILES,
            Self::CopyFileName => CMD_COPY_FILENAME,
            Self::CopyFileNameWithoutExtension => CMD_COPY_FILENAME_WITHOUT_EXTENSION,
            Self::CopyFilePath { .. } => CMD_COPY_FILEPATH,
            Self::CopyDirPath => CMD_COPY_DIRECTORY_PATH,
            Self::SymlinkFiles { .. } => CMD_SYMLINK_FILES,
            Self::PasteFiles { .. } => CMD_PASTE_FILES,

            Self::DeleteFiles { .. } => CMD_DELETE_FILES,

            Self::CursorMoveUp { .. } => CMD_CURSOR_MOVE_UP,
            Self::CursorMoveDown { .. } => CMD_CURSOR_MOVE_DOWN,
            Self::CursorMoveHome => CMD_CURSOR_MOVE_HOME,
            Self::CursorMoveEnd => CMD_CURSOR_MOVE_END,
            Self::CursorMovePageUp(_) => CMD_CURSOR_MOVE_PAGEUP,
            Self::CursorMovePageDown(_) => CMD_CURSOR_MOVE_PAGEDOWN,
            Self::CursorMovePageHome => CMD_CURSOR_MOVE_PAGEUP,
            Self::CursorMovePageMiddle => CMD_CURSOR_MOVE_PAGEDOWN,
            Self::CursorMovePageEnd => CMD_CURSOR_MOVE_PAGEDOWN,

            Self::ParentCursorMoveUp { .. } => CMD_PARENT_CURSOR_MOVE_UP,
            Self::ParentCursorMoveDown { .. } => CMD_PARENT_CURSOR_MOVE_DOWN,

            Self::PreviewCursorMoveUp { .. } => CMD_PREVIEW_CURSOR_MOVE_UP,
            Self::PreviewCursorMoveDown { .. } => CMD_PREVIEW_CURSOR_MOVE_DOWN,

            Self::NewDirectory { .. } => CMD_NEW_DIRECTORY,
            Self::OpenFile => CMD_OPEN_FILE,
            Self::OpenFileWith { .. } => CMD_OPEN_FILE_WITH,

            Self::ReloadDirList => CMD_RELOAD_DIRECTORY_LIST,
            Self::RenameFile { .. } => CMD_RENAME_FILE,
            Self::RenameFileAppend => CMD_RENAME_FILE_APPEND,
            Self::RenameFileAppendBase => CMD_RENAME_FILE_APPEND_BASE,
            Self::RenameFilePrepend => CMD_RENAME_FILE_PREPEND,
            Self::RenameFileKeepExt => CMD_RENAME_FILE_KEEP_EXT,

            Self::SearchString { .. } => CMD_SEARCH_STRING,
            Self::SearchIncremental { .. } => CMD_SEARCH_INCREMENTAL,
            Self::SearchGlob { .. } => CMD_SEARCH_GLOB,
            Self::SearchRegex { .. } => CMD_SEARCH_REGEX,
            Self::SearchNext => CMD_SEARCH_NEXT,
            Self::SearchPrev => CMD_SEARCH_PREV,

            Self::SelectGlob { .. } => CMD_SELECT_GLOB,
            Self::SelectRegex { .. } => CMD_SELECT_REGEX,
            Self::SelectString { .. } => CMD_SELECT_STRING,

            Self::SetCaseSensitivity { .. } => CMD_SET_CASE_SENSITIVITY,
            Self::SetMode => CMD_SET_MODE,

            Self::ShowTasks => CMD_SHOW_TASKS,

            Self::Flat { .. } => CMD_FLAT,
            Self::NumberedCommand { .. } => CMD_NUMBERED_COMMAND,

            Self::Sort(_) => CMD_SORT,
            Self::SortReverse => CMD_SORT_REVERSE,

            Self::FilterGlob { .. } => CMD_FILTER_GLOB,
            Self::FilterRegex { .. } => CMD_FILTER_REGEX,
            Self::FilterString { .. } => CMD_FILTER_STRING,

            Self::SubProcess { spawn: false, .. } => CMD_SUBPROCESS_FOREGROUND,
            Self::SubProcess { spawn: true, .. } => CMD_SUBPROCESS_BACKGROUND,
            Self::SwitchLineNums(_) => CMD_SWITCH_LINE_NUMBERS,
            Self::SetLineMode(_) => CMD_SET_LINEMODE,

            Self::TabSwitch { .. } => CMD_TAB_SWITCH,
            Self::TabSwitchIndex { .. } => CMD_TAB_SWITCH_INDEX,
            Self::ToggleHiddenFiles => CMD_TOGGLE_HIDDEN,
            Self::TouchFile { .. } => CMD_TOUCH_FILE,

            Self::SearchFzf => CMD_SEARCH_FZF,
            Self::SubdirFzf => CMD_SUBDIR_FZF,
            Self::SelectFzf { .. } => CMD_SELECT_FZF,
            Self::Zoxide(_) => CMD_ZOXIDE,
            Self::ZoxideInteractive => CMD_ZOXIDE_INTERACTIVE,

            Self::CustomSearch(_) => CMD_CUSTOM_SEARCH,
            Self::CustomSearchInteractive(_) => CMD_CUSTOM_SEARCH_INTERACTIVE,

            Self::BookmarkAdd => CMD_BOOKMARK_ADD,
            Self::BookmarkChangeDirectory => CMD_BOOKMARK_CHANGE_DIRECTORY,
        }
    }
}
