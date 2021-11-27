use std::path;

use crate::config::option::{SelectOption, SortType};
use crate::io::IoWorkerOptions;

#[derive(Clone, Debug)]
pub enum Command {
    BulkRename,
    ChangeDirectory(path::PathBuf),
    CommandLine(String, String),

    CutFiles,
    CopyFiles,
    PasteFiles(IoWorkerOptions),
    CopyFileName,
    CopyFileNameWithoutExtension,
    CopyFilePath,
    CopyDirPath,

    CursorMoveUp(usize),
    CursorMoveDown(usize),
    CursorMoveHome,
    CursorMoveEnd,
    CursorMovePageUp,
    CursorMovePageDown,

    ParentCursorMoveUp(usize),
    ParentCursorMoveDown(usize),

    PreviewCursorMoveUp(usize),
    PreviewCursorMoveDown(usize),

    // ChildCursorMoveUp(usize),
    // ChildCursorMoveDown(usize),
    DeleteFiles,
    NewDirectory(path::PathBuf),
    OpenFile,
    OpenFileWith(Option<usize>),
    ParentDirectory,

    Quit,
    QuitToCurrentDirectory,
    ForceQuit,
    ReloadDirList,
    RenameFile(path::PathBuf),
    RenameFileAppend,
    RenameFilePrepend,
    TouchFile(String),

    SearchGlob(String),
    SearchString(String),
    SearchFzf,
    SearchNext,
    SearchPrev,

    SubdirFzf,

    SelectFiles(String, SelectOption),
    SetMode,
    SubProcess(Vec<String>, bool),
    ShowWorkers,

    ToggleHiddenFiles,

    Sort(SortType),
    SortReverse,

    NewTab,
    CloseTab,
    TabSwitch(i32),
    TabSwitchIndex(u32),
    Help,
}
