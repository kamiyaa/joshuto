use rustyline::completion::Pair;

pub const CMD_COMMAND_LINE: &str = ":";

macro_rules! cmd_constants {
    ($( ($cmd_name:ident, $cmd_value:literal), )*) => {
        $(
            pub const $cmd_name: &str = $cmd_value;
        )*

        pub fn commands() -> Vec<&'static str> {
            vec![$($cmd_value,)*]
        }
    };
}

cmd_constants![
    (CMD_QUIT, "quit"),
    (CMD_BULK_RENAME, "bulk_rename"),
    (CMD_CHANGE_DIRECTORY, "cd"),
    (CMD_PARENT_DIRECTORY, "cd .."),
    (CMD_PREVIOUS_DIRECTORY, "cd -"),
    (CMD_NEW_TAB, "new_tab"),
    (CMD_CLOSE_TAB, "close_tab"),
    (CMD_CUT_FILES, "cut_files"),
    (CMD_COPY_FILES, "copy_files"),
    (CMD_COPY_FILENAME, "copy_filename"),
    (
        CMD_COPY_FILENAME_WITHOUT_EXTENSION,
        "copy_filename_without_extension"
    ),
    (CMD_COPY_FILEPATH, "copy_filepath"),
    (CMD_COPY_DIRECTORY_PATH, "copy_dirpath"),
    (CMD_SYMLINK_FILES, "symlink_files"),
    (CMD_PASTE_FILES, "paste_files"),
    (CMD_DELETE_FILES, "delete_files"),
    (CMD_CURSOR_MOVE_UP, "cursor_move_up"),
    (CMD_CURSOR_MOVE_DOWN, "cursor_move_down"),
    (CMD_CURSOR_MOVE_HOME, "cursor_move_home"),
    (CMD_CURSOR_MOVE_END, "cursor_move_end"),
    (CMD_CURSOR_MOVE_PAGEUP, "cursor_move_page_up"),
    (CMD_CURSOR_MOVE_PAGEDOWN, "cursor_move_page_down"),
    (CMD_CURSOR_MOVE_PAGEHOME, "cursor_move_page_home"),
    (CMD_CURSOR_MOVE_PAGEMIDDLE, "cursor_move_page_middle"),
    (CMD_CURSOR_MOVE_PAGEEND, "cursor_move_page_end"),
    (CMD_PARENT_CURSOR_MOVE_UP, "parent_cursor_move_up"),
    (CMD_PARENT_CURSOR_MOVE_DOWN, "parent_cursor_move_down"),
    (CMD_PREVIEW_CURSOR_MOVE_UP, "preview_cursor_move_up"),
    (CMD_PREVIEW_CURSOR_MOVE_DOWN, "preview_cursor_move_down"),
    (CMD_NEW_DIRECTORY, "mkdir"),
    (CMD_OPEN_FILE, "open"),
    (CMD_OPEN_FILE_WITH, "open_with"),
    (CMD_RELOAD_DIRECTORY_LIST, "reload_dirlist"),
    (CMD_RENAME_FILE, "rename"),
    (CMD_RENAME_FILE_APPEND, "rename_append"),
    (CMD_RENAME_FILE_APPEND_BASE, "rename_append_base"),
    (CMD_RENAME_FILE_PREPEND, "rename_prepend"),
    (CMD_RENAME_FILE_KEEP_EXT, "rename_keep_ext"),
    (CMD_SEARCH_STRING, "search"),
    (CMD_SEARCH_INCREMENTAL, "search_inc"),
    (CMD_SEARCH_GLOB, "search_glob"),
    (CMD_SEARCH_REGEX, "search_regex"),
    (CMD_SEARCH_NEXT, "search_next"),
    (CMD_SEARCH_PREV, "search_prev"),
    (CMD_SELECT_GLOB, "select_glob"),
    (CMD_SELECT_REGEX, "select_regex"),
    (CMD_SELECT_STRING, "select"),
    (CMD_SET_CASE_SENSITIVITY, "set_case_sensitivity"),
    (CMD_SET_MODE, "set_mode"),
    (CMD_SORT, "sort"),
    (CMD_SORT_REVERSE, "sort reverse"),
    (CMD_SUBPROCESS_FOREGROUND, "shell"),
    (CMD_SUBPROCESS_BACKGROUND, "spawn"),
    (CMD_SHOW_TASKS, "show_tasks"),
    (CMD_TAB_SWITCH, "tab_switch"),
    (CMD_TAB_SWITCH_INDEX, "tab_switch_index"),
    (CMD_TOGGLE_HIDDEN, "toggle_hidden"),
    (CMD_TOGGLE_VISUAL, "toggle_visual"),
    (CMD_SWITCH_LINE_NUMBERS, "line_nums"),
    (CMD_SET_LINEMODE, "linemode"),
    (CMD_TOUCH_FILE, "touch"),
    (CMD_HELP, "help"),
    (CMD_SEARCH_FZF, "search_fzf"),
    (CMD_SUBDIR_FZF, "subdir_fzf"),
    (CMD_SELECT_FZF, "select_fzf"),
    (CMD_ZOXIDE, "z"),
    (CMD_ZOXIDE_INTERACTIVE, "zi"),
    (CMD_NUMBERED_COMMAND, "numbered_command"),
    (CMD_FLAT, "flat"),
    (CMD_ESCAPE, "escape"),
    (CMD_FILTER_GLOB, "filter_glob"),
    (CMD_FILTER_REGEX, "filter_regex"),
    (CMD_FILTER_STRING, "filter"),
    (CMD_BOOKMARK_ADD, "add_bookmark"),
    (CMD_BOOKMARK_CHANGE_DIRECTORY, "cd_bookmark"),
    (CMD_CUSTOM_SEARCH, "custom_search"),
    (CMD_CUSTOM_SEARCH_INTERACTIVE, "custom_search_interactive"),
];

pub fn complete_command(partial_command: &str) -> Vec<Pair> {
    commands()
        .into_iter()
        .filter(|command| command.starts_with(partial_command))
        .map(|command| Pair {
            display: command.to_string(),
            replacement: command.to_string(),
        })
        .collect()
}
