use std::path;

use crate::commands::quit::QuitAction;
use crate::config::option::{LineNumberStyle, SelectOption, SortType};
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
    CopyFilePath,
    CopyDirPath,
    SymlinkFiles {
        relative: bool,
    },
    PasteFiles {
        options: FileOperationOptions,
    },

    DeleteFiles {
        background: bool,
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
    TouchFile {
        file_name: String,
    },

    SearchGlob {
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

    SelectFiles {
        pattern: String,
        options: SelectOption,
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

    NewTab,
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
    Zoxide(String),
    ZoxideInteractive,
}
