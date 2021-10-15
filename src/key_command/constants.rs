pub const CMD_HELP: &str = "help";

pub const CMD_QUIT: &str = "quit";
pub const CMD_QUIT_TO_CURRENT_DIRECTORY: &str = "quit_to_cwd";
pub const CMD_FORCE_QUIT: &str = "force_quit";

pub const CMD_BULK_RENAME: &str = "bulk_rename";
pub const CMD_CHANGE_DIRECTORY: &str = "cd";
pub const CMD_NEW_TAB: &str = "new_tab";
pub const CMD_CLOSE_TAB: &str = "close_tab";
pub const CMD_COMMAND_LINE: &str = ":";
pub const CMD_CUT_FILES: &str = "cut_files";
pub const CMD_COPY_FILES: &str = "copy_files";
pub const CMD_PASTE_FILES: &str = "paste_files";
pub const CMD_COPY_FILENAME: &str = "copy_filename";
pub const CMD_COPY_FILENAME_WITHOUT_EXTENSION: &str = "copy_filename_without_extension";
pub const CMD_COPY_FILEPATH: &str = "copy_filepath";
pub const CMD_COPY_DIRECTORY_PATH: &str = "copy_dirpath";
pub const CMD_CURSOR_MOVE_UP: &str = "cursor_move_up";
pub const CMD_CURSOR_MOVE_DOWN: &str = "cursor_move_down";
pub const CMD_CURSOR_MOVE_HOME: &str = "cursor_move_home";
pub const CMD_CURSOR_MOVE_END: &str = "cursor_move_end";
pub const CMD_CURSOR_MOVE_PAGEUP: &str = "cursor_move_page_up";
pub const CMD_CURSOR_MOVE_PAGEDOWN: &str = "cursor_move_page_down";
pub const CMD_PARENT_CURSOR_MOVE_UP: &str = "parent_cursor_move_up";
pub const CMD_PARENT_CURSOR_MOVE_DOWN: &str = "parent_cursor_move_down";
pub const CMD_DELETE_FILES: &str = "delete_files";
pub const CMD_NEW_DIRECTORY: &str = "mkdir";
pub const CMD_OPEN_FILE: &str = "open";
pub const CMD_OPEN_FILE_WITH: &str = "open_with";
pub const CMD_PARENT_DIRECTORY: &str = "cd ..";
pub const CMD_RELOAD_DIRECTORY_LIST: &str = "reload_dirlist";
pub const CMD_RENAME_FILE: &str = "rename";
pub const CMD_RENAME_FILE_APPEND: &str = "rename_append";
pub const CMD_RENAME_FILE_PREPEND: &str = "rename_prepend";

pub const CMD_SEARCH_STRING: &str = "search";
pub const CMD_SEARCH_GLOB: &str = "search_glob";
pub const CMD_SEARCH_FZF: &str = "search_fzf";
pub const CMD_SEARCH_NEXT: &str = "search_next";
pub const CMD_SEARCH_PREV: &str = "search_prev";

pub const CMD_SUBDIR_FZF: &str = "subdir_fzf";

pub const CMD_SELECT_FILES: &str = "select";
pub const CMD_SET_MODE: &str = "set_mode";
pub const CMD_SORT: &str = "sort";
pub const CMD_SORT_REVERSE: &str = "sort reverse";
pub const CMD_SUBPROCESS_FOREGROUND: &str = "shell";
pub const CMD_SUBPROCESS_BACKGROUND: &str = "spawn";
pub const CMD_SHOW_WORKERS: &str = "show_workers";
pub const CMD_TAB_SWITCH: &str = "tab_switch";
pub const CMD_TAB_SWITCH_INDEX: &str = "tab_switch_index";
pub const CMD_TOGGLE_HIDDEN: &str = "toggle_hidden";
pub const CMD_TOUCH_FILE: &str = "touch";
