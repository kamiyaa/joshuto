use std::path;

use crate::commands::case_sensitivity::SetType;
use crate::commands::quit::QuitAction;
use crate::commands::select::SelectOption;
use crate::commands::stdout::PostProcessor;
use crate::commands::sub_process::SubprocessCallMode;
use crate::config::clean::app::display::line_mode::LineMode;
use crate::config::clean::app::display::line_number::LineNumberStyle;
use crate::config::clean::app::display::new_tab::NewTabMode;
use crate::config::clean::app::display::sort_type::SortType;
use crate::config::clean::app::search::CaseSensitivity;
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
    RenameFileAppendBase,
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
        mode: SubprocessCallMode,
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

    Sort {
        sort_type: SortType,
        reverse: Option<bool>,
    },
    SortReverse,

    FilterGlob {
        pattern: String,
    },
    FilterRegex {
        pattern: String,
    },
    FilterString {
        pattern: String,
    },

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
    StdOutPostProcess {
        processor: PostProcessor,
    },
    Zoxide(String),
    ZoxideInteractive,

    CustomSearch(Vec<String>),
    CustomSearchInteractive(Vec<String>),

    BookmarkAdd,
    BookmarkChangeDirectory,
}
