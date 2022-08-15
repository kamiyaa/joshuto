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

    ChangeDirectory(path::PathBuf),
    ParentDirectory,
    PreviousDirectory,

    CommandLine(String, String),

    CutFiles,
    CopyFiles,
    CopyFileName,
    CopyFileNameWithoutExtension,
    CopyFilePath,
    CopyDirPath,
    SymlinkFiles,
    PasteFiles(FileOperationOptions),

    DeleteFiles { background: bool },

    CursorMoveUp(usize),
    CursorMoveDown(usize),
    CursorMoveHome,
    CursorMoveEnd,
    CursorMovePageUp(f64),
    CursorMovePageDown(f64),
    CursorMovePageHome,
    CursorMovePageMiddle,
    CursorMovePageEnd,

    ParentCursorMoveUp(usize),
    ParentCursorMoveDown(usize),

    PreviewCursorMoveUp(usize),
    PreviewCursorMoveDown(usize),

    // ChildCursorMoveUp(usize),
    // ChildCursorMoveDown(usize),
    NewDirectory(path::PathBuf),
    OpenFile,
    OpenFileWith(Option<usize>),

    Quit(QuitAction),

    ReloadDirList,
    RenameFile(path::PathBuf),
    RenameFileAppend,
    RenameFilePrepend,
    TouchFile(String),

    SearchGlob(String),
    SearchString(String),
    SearchIncremental(String),
    SearchNext,
    SearchPrev,

    SelectFiles(String, SelectOption),
    SetMode,
    SubProcess(Vec<String>, bool),
    ShowTasks,

    ToggleHiddenFiles,

    SwitchLineNums(LineNumberStyle),

    Flat(usize),

    Sort(SortType),
    SortReverse,

    NewTab,
    CloseTab,
    TabSwitch(i32),
    TabSwitchIndex(u32),
    Help,

    SearchFzf,
    SubdirFzf,
    Zoxide(String),
    ZoxideInteractive,
}
