use std::path;

use crate::config::option::{LineNumberStyle, SelectOption, SortType};
use crate::io::IoWorkerOptions;

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Command {
    BulkRename,

    ChangeDirectory(path::PathBuf),
    ParentDirectory,
    PreviousDirectory,

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
    CursorMovePageUp(f64),
    CursorMovePageDown(f64),

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
    SearchIncremental(String),
    SearchFzf,
    SearchNext,
    SearchPrev,

    SubdirFzf,

    SelectFiles(String, SelectOption),
    SetMode,
    SubProcess(Vec<String>, bool),
    ShowWorkers,

    ToggleHiddenFiles,

    SwitchLineNums(LineNumberStyle),

    Sort(SortType),
    SortReverse,

    NewTab,
    CloseTab,
    TabSwitch(i32),
    TabSwitchIndex(u32),
    Help,
}
