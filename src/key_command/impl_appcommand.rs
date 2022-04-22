use super::constants::*;
use super::{AppCommand, Command};

impl AppCommand for Command {
    fn command(&self) -> &'static str {
        match self {
            Self::Help => CMD_HELP,

            Self::Quit(_) => CMD_QUIT,

            Self::BulkRename => CMD_BULK_RENAME,

            Self::ChangeDirectory(_) => CMD_CHANGE_DIRECTORY,
            Self::ParentDirectory => CMD_PARENT_DIRECTORY,
            Self::PreviousDirectory => CMD_PREVIOUS_DIRECTORY,

            Self::NewTab => CMD_NEW_TAB,
            Self::CloseTab => CMD_CLOSE_TAB,
            Self::CommandLine(_, _) => CMD_COMMAND_LINE,

            Self::CutFiles => CMD_CUT_FILES,
            Self::CopyFiles => CMD_COPY_FILES,
            Self::PasteFiles(_) => CMD_PASTE_FILES,
            Self::CopyFileName => CMD_COPY_FILENAME,
            Self::CopyFileNameWithoutExtension => CMD_COPY_FILENAME_WITHOUT_EXTENSION,
            Self::CopyFilePath => CMD_COPY_FILEPATH,
            Self::CopyDirPath => CMD_COPY_DIRECTORY_PATH,

            Self::CursorMoveUp(_) => CMD_CURSOR_MOVE_UP,
            Self::CursorMoveDown(_) => CMD_CURSOR_MOVE_DOWN,
            Self::CursorMoveHome => CMD_CURSOR_MOVE_HOME,
            Self::CursorMoveEnd => CMD_CURSOR_MOVE_END,
            Self::CursorMovePageUp(_) => CMD_CURSOR_MOVE_PAGEUP,
            Self::CursorMovePageDown(_) => CMD_CURSOR_MOVE_PAGEDOWN,
            Self::CursorMovePageHome => CMD_CURSOR_MOVE_PAGEUP,
            Self::CursorMovePageMiddle => CMD_CURSOR_MOVE_PAGEDOWN,
            Self::CursorMovePageEnd => CMD_CURSOR_MOVE_PAGEDOWN,

            Self::ParentCursorMoveUp(_) => CMD_PARENT_CURSOR_MOVE_UP,
            Self::ParentCursorMoveDown(_) => CMD_PARENT_CURSOR_MOVE_DOWN,

            Self::PreviewCursorMoveUp(_) => CMD_PREVIEW_CURSOR_MOVE_UP,
            Self::PreviewCursorMoveDown(_) => CMD_PREVIEW_CURSOR_MOVE_DOWN,

            Self::DeleteFiles => CMD_DELETE_FILES,
            Self::NewDirectory(_) => CMD_NEW_DIRECTORY,
            Self::OpenFile => CMD_OPEN_FILE,
            Self::OpenFileWith(_) => CMD_OPEN_FILE_WITH,

            Self::ReloadDirList => CMD_RELOAD_DIRECTORY_LIST,
            Self::RenameFile(_) => CMD_RENAME_FILE,
            Self::RenameFileAppend => CMD_RENAME_FILE_APPEND,
            Self::RenameFilePrepend => CMD_RENAME_FILE_PREPEND,

            Self::SearchString(_) => CMD_SEARCH_STRING,
            Self::SearchIncremental(_) => CMD_SEARCH_INCREMENTAL,
            Self::SearchGlob(_) => CMD_SEARCH_GLOB,
            Self::SearchFzf => CMD_SEARCH_FZF,
            Self::SearchNext => CMD_SEARCH_NEXT,
            Self::SearchPrev => CMD_SEARCH_PREV,

            Self::SubdirFzf => CMD_SUBDIR_FZF,

            Self::SelectFiles(_, _) => CMD_SELECT_FILES,
            Self::SetMode => CMD_SET_MODE,

            Self::Sort(_) => CMD_SORT,
            Self::SortReverse => CMD_SORT_REVERSE,

            Self::SubProcess(_, false) => CMD_SUBPROCESS_FOREGROUND,
            Self::SubProcess(_, true) => CMD_SUBPROCESS_BACKGROUND,
            Self::ShowWorkers => CMD_SHOW_WORKERS,

            Self::TabSwitch(_) => CMD_TAB_SWITCH,
            Self::TabSwitchIndex(_) => CMD_TAB_SWITCH_INDEX,
            Self::ToggleHiddenFiles => CMD_TOGGLE_HIDDEN,
            Self::SwitchLineNums(_) => CMD_SWITCH_LINE_NUMBERS,
            Self::TouchFile(_) => CMD_TOUCH_FILE,
        }
    }
}
