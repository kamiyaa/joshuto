use std::path;

use crate::commands::case_sensitivity::SetType;
use crate::commands::quit::QuitAction;
use crate::config::option::{
    CaseSensitivity, LineMode, LineNumberStyle, NewTabMode, SelectOption, SortType,
    TabBarDisplayMode,
};
use crate::io::FileOperationOptions;

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Command {
    Escape,
    ToggleVisualMode,
    BulkRename,

    ChangeDirectory {
        path: path::PathBuf,
    },
    ParentDirectory,
    PreviousDirectory,

    CommandLine {
        prefix: String,
        suffix: String,
    },

    CutFiles,
    CopyFiles,
    CopyFileName,
    CopyFileNameWithoutExtension,
    CopyFilePath {
        all_selected: bool,
    },
    CopyDirPath,
    SymlinkFiles {
        relative: bool,
    },
    PasteFiles {
        options: FileOperationOptions,
    },

    DeleteFiles {
        background: bool,
        permanently: bool,
        noconfirm: bool,
    },

    CursorMoveUp {
        offset: usize,
    },
    CursorMoveDown {
        offset: usize,
    },
    CursorMoveHome,
    CursorMoveEnd,
    CursorMovePageUp(f64),
    CursorMovePageDown(f64),
    CursorMovePageHome,
    CursorMovePageMiddle,
    CursorMovePageEnd,

    SetLineMode(LineMode),

    ParentCursorMoveUp {
        offset: usize,
    },
    ParentCursorMoveDown {
        offset: usize,
    },

    PreviewCursorMoveUp {
        offset: usize,
    },
    PreviewCursorMoveDown {
        offset: usize,
    },

    // ChildCursorMoveUp(usize),
    // ChildCursorMoveDown(usize),
    NewDirectory {
        path: path::PathBuf,
    },
    OpenFile,
    OpenFileWith {
        index: Option<usize>,
    },
    Quit(QuitAction),

    ReloadDirList,
    RenameFile {
        new_name: path::PathBuf,
    },
    RenameFileAppend,
    RenameFilePrepend,
    RenameFileKeepExt,
    TouchFile {
        file_name: String,
    },

    SearchGlob {
        pattern: String,
    },
    SearchRegex {
        pattern: String,
    },
    SearchString {
        pattern: String,
    },
    SearchIncremental {
        pattern: String,
    },
    SearchNext,
    SearchPrev,

    SelectGlob {
        pattern: String,
        options: SelectOption,
    },
    SelectRegex {
        pattern: String,
        options: SelectOption,
    },
    SelectString {
        pattern: String,
        options: SelectOption,
    },

    SetCaseSensitivity {
        case_sensitivity: CaseSensitivity,
        set_type: SetType,
    },
    SetMode,
    SubProcess {
        words: Vec<String>,
        spawn: bool,
    },
    ShowTasks,

    ToggleHiddenFiles,
    SwitchLineNums(LineNumberStyle),

    Flat {
        depth: usize,
    },
    NumberedCommand {
        initial: char,
    },

    Sort(SortType),
    SortReverse,

    FilterString {
        pattern: String,
    },

    SetTabBarDisplayMode(TabBarDisplayMode),
    NewTab {
        mode: NewTabMode,
    },
    CloseTab,
    TabSwitch {
        offset: i32,
    },
    TabSwitchIndex {
        index: usize,
    },
    Help,

    SearchFzf,
    SubdirFzf,
    SelectFzf {
        options: SelectOption,
    },
    Zoxide(String),
    ZoxideInteractive,

    BookmarkAdd,
    BookmarkChangeDirectory,
}
