use std::fs;
use std::time;

use users::UsersCache;
use users::mock::{Groups, Users};

use crate::config::{JoshutoColorTheme, JoshutoConfig};
use crate::context::JoshutoContext;
use crate::io::{JoshutoDirEntry, JoshutoDirList};
use crate::unix;
use crate::window;

use crate::THEME_T;

pub const ERR_COLOR: i16 = 240;
pub const EMPTY_COLOR: i16 = 241;

const MIN_WIN_WIDTH: usize = 4;

pub struct DisplayOptions {
    pub detailed: bool,
}

pub const PRIMARY_DISPLAY_OPTION: DisplayOptions = DisplayOptions { detailed: true };
pub const SECONDARY_DISPLAY_OPTION: DisplayOptions = DisplayOptions { detailed: false };

pub fn init_ncurses() {
    ncurses::setlocale(ncurses::LcCategory::all, "");

    ncurses::initscr();
    ncurses::cbreak();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::start_color();
    ncurses::use_default_colors();
    ncurses::noecho();
    ncurses::set_escdelay(0);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    process_theme();

    ncurses::addstr("Loading...");
    ncurses::refresh();
}

fn process_theme() {
    for pair in THEME_T.colorpair.iter() {
        ncurses::init_pair(pair.id, pair.fg, pair.bg);
    }

    /* error message */
    ncurses::init_pair(ERR_COLOR, ncurses::COLOR_RED, -1);
    /* empty */
    ncurses::init_pair(EMPTY_COLOR, ncurses::COLOR_WHITE, ncurses::COLOR_RED);
}

pub fn end_ncurses() {
    ncurses::endwin();
}

pub fn getmaxyx() -> (i32, i32) {
    let mut term_rows: i32 = 0;
    let mut term_cols: i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);
    (term_rows, term_cols)
}

pub fn display_menu(win: &window::JoshutoPanel, vals: &[String]) {
    ncurses::werase(win.win);
    ncurses::mvwhline(win.win, 0, 0, 0, win.cols);

    for (i, val) in vals.iter().enumerate() {
        ncurses::wmove(win.win, (i + 1) as i32, 0);
        ncurses::waddstr(win.win, val.as_str());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_msg(win: &window::JoshutoPanel, msg: &str) {
    ncurses::werase(win.win);
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_err(win: &window::JoshutoPanel, msg: &str) {
    let attr = ncurses::A_BOLD() | ncurses::COLOR_PAIR(ERR_COLOR);

    ncurses::werase(win.win);
    ncurses::wattron(win.win, attr);

    ncurses::mvwaddstr(win.win, 0, 0, msg);

    ncurses::wattroff(win.win, attr);
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_empty(win: &window::JoshutoPanel, msg: &str) {
    ncurses::werase(win.win);
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(EMPTY_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(EMPTY_COLOR));
    ncurses::wnoutrefresh(win.win);
}

fn wprint_file_name(
    win: &window::JoshutoPanel,
    file_name: &str,
    coord: (i32, i32),
    mut space_avail: usize,
) {
    let name_visual_space = unicode_width::UnicodeWidthStr::width(file_name);
    if name_visual_space < space_avail {
        ncurses::mvwaddstr(win.win, coord.0, coord.1, &file_name);
        return;
    }
    if let Some(ext) = file_name.rfind('.') {
        let extension: &str = &file_name[ext..];
        let ext_len = unicode_width::UnicodeWidthStr::width(extension);
        if space_avail > ext_len {
            space_avail -= ext_len;
            ncurses::mvwaddstr(win.win, coord.0, space_avail as i32, &extension);
        }
    }
    if space_avail < 2 {
        return;
    } else {
        space_avail -= 2;
    }

    ncurses::wmove(win.win, coord.0, coord.1);

    let mut trim_index: usize = file_name.len();

    let mut total: usize = 0;
    for (index, ch) in file_name.char_indices() {
        if total >= space_avail {
            trim_index = index;
            break;
        }
        total += unicode_width::UnicodeWidthChar::width(ch).unwrap_or(2);
    }
    ncurses::waddstr(win.win, &file_name[..trim_index]);
    ncurses::waddstr(win.win, "…");
}

fn wprint_entry(
    win: &window::JoshutoPanel,
    file: &JoshutoDirEntry,
    prefix: (usize, &str),
    coord: (i32, i32),
) {
    if win.cols <= prefix.0 as i32 {
        return;
    }
    ncurses::waddstr(win.win, prefix.1);
    let space_avail = win.cols as usize - prefix.0;

    wprint_file_name(
        &win,
        file.file_name(),
        (coord.0, coord.1 + prefix.0 as i32),
        space_avail,
    );
}

fn wprint_entry_detailed(
    win: &window::JoshutoPanel,
    file: &JoshutoDirEntry,
    prefix: (usize, &str),
    coord: (i32, i32),
) {
    if win.cols <= prefix.0 as i32 {
        return;
    }
    ncurses::waddstr(win.win, prefix.1);
    let mut space_avail = win.cols as usize - prefix.0;

    let coord = (coord.0, coord.1 + prefix.0 as i32);

    if file.file_path().is_dir() {
    } else {
        let file_size_string = file_size_to_string(file.metadata.len as f64);
        if space_avail > file_size_string.len() {
            space_avail -= file_size_string.len();
            ncurses::mvwaddstr(win.win, coord.0, space_avail as i32, &file_size_string);
        }
    }
    wprint_file_name(win, file.file_name(), coord, space_avail);
}

pub fn display_contents(
    win: &window::JoshutoPanel,
    dirlist: &mut JoshutoDirList,
    config_t: &JoshutoConfig,
    options: &DisplayOptions,
) {
    if win.cols < MIN_WIN_WIDTH as i32 {
        return;
    }
    let dir_len = dirlist.contents.len();
    if dir_len == 0 {
        wprint_empty(win, "empty");
        return;
    }
    ncurses::werase(win.win);
    ncurses::wmove(win.win, 0, 0);

    let draw_func = if options.detailed {
        wprint_entry_detailed
    } else {
        wprint_entry
    };

    let curr_index = dirlist.index.unwrap();
    dirlist
        .pagestate
        .update_page_state(curr_index, win.rows, dir_len, config_t.scroll_offset);

    let (start, end) = (dirlist.pagestate.start, dirlist.pagestate.end);
    let dir_contents = &dirlist.contents[start..end];

    ncurses::werase(win.win);
    ncurses::wmove(win.win, 0, 0);

    for (i, entry) in dir_contents.iter().enumerate() {
        let coord: (i32, i32) = (i as i32, 0);

        ncurses::wmove(win.win, coord.0, coord.1);

        let attr = if i + start == curr_index {
            ncurses::A_STANDOUT()
        } else {
            0
        };
        let attrs = get_theme_attr(attr, entry);

        draw_func(win, entry, attrs.0, coord);

        ncurses::mvwchgat(win.win, coord.0, coord.1, -1, attrs.1, attrs.2);
    }
    win.queue_for_refresh();
}

pub fn wprint_file_status(win: &window::JoshutoPanel, entry: &JoshutoDirEntry, index: usize, len: usize) {
    wprint_file_mode(win.win, entry);

    ncurses::waddch(win.win, ' ' as ncurses::chtype);
    ncurses::waddstr(
        win.win,
        format!("{}/{} ", index + 1, len).as_str(),
    );

    let usercache: UsersCache = UsersCache::new();
    match usercache.get_user_by_uid(entry.metadata.uid) {
        Some(s) => match s.name().to_str() {
            Some(name) => ncurses::waddstr(win.win, name),
            None => ncurses::waddstr(win.win, "OsStr error"),
        }
        None => ncurses::waddstr(win.win, "unknown user"),
    };
    ncurses::waddch(win.win, ' ' as ncurses::chtype);
    match usercache.get_group_by_gid(entry.metadata.gid) {
        Some(s) => match s.name().to_str() {
            Some(name) => ncurses::waddstr(win.win, name),
            None => ncurses::waddstr(win.win, "OsStr error"),
        }
        None => ncurses::waddstr(win.win, "unknown user"),
    };

    ncurses::waddstr(win.win, "  ");
    wprint_file_info(win.win, entry);
    win.queue_for_refresh();
}

pub fn wprint_file_mode(win: ncurses::WINDOW, file: &JoshutoDirEntry) {
    use std::os::unix::fs::PermissionsExt;

    let mode = file.metadata.permissions.mode();

    ncurses::wattron(win, ncurses::COLOR_PAIR(6));
    ncurses::waddstr(win, unix::stringify_mode(mode).as_str());
    ncurses::wattroff(win, ncurses::COLOR_PAIR(6));
}

pub fn wprint_file_info(win: ncurses::WINDOW, file: &JoshutoDirEntry) {
    #[cfg(unix)]
    {
    use std::os::unix::fs::PermissionsExt;

    let mode = file.metadata.permissions.mode();

    let mtime_string = file_mtime_to_string(file.metadata.modified);
    ncurses::waddstr(win, &mtime_string);
    ncurses::waddch(win, ' ' as ncurses::chtype);

    if file.file_path().is_dir() {
        let is_link: u32 = libc::S_IFLNK as u32;
        if mode >> 9 & is_link >> 9 == mode >> 9 {
            if let Ok(path) = fs::read_link(&file.file_path()) {
                ncurses::waddstr(win, " -> ");
                ncurses::waddstr(win, path.to_str().unwrap());
            }
        }
    } else {
        let file_size_string = file_size_to_string_detailed(file.metadata.len as f64);
        ncurses::waddstr(win, &file_size_string);
    }

    /*
        ncurses::waddstr(win, "    ");
        if let Some(s) = tree_magic::from_filepath(&file.file_path()) {
            ncurses::waddstr(win, &s);
        }
    */
    }
}

pub fn redraw_tab_view(win: &window::JoshutoPanel, context: &JoshutoContext) {
    let tab_len = context.tabs.len();
    ncurses::werase(win.win);
    if tab_len == 1 {
    } else if tab_len >= 6 {
        ncurses::wmove(win.win, 0, 0);
        ncurses::wattron(win.win, ncurses::A_BOLD() | ncurses::A_STANDOUT());
        ncurses::waddstr(win.win, &format!("{}", context.curr_tab_index + 1));
        ncurses::wattroff(win.win, ncurses::A_STANDOUT());
        ncurses::waddstr(win.win, &format!(" {}", tab_len));
        ncurses::wattroff(win.win, ncurses::A_BOLD());
    } else {
        ncurses::wattron(win.win, ncurses::A_BOLD());
        for i in 0..tab_len {
            if i == context.curr_tab_index {
                ncurses::wattron(win.win, ncurses::A_STANDOUT());
                ncurses::waddstr(win.win, &format!("{} ", i + 1));
                ncurses::wattroff(win.win, ncurses::A_STANDOUT());
            } else {
                ncurses::waddstr(win.win, &format!("{} ", i + 1));
            }
        }
        ncurses::wattroff(win.win, ncurses::A_BOLD());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn draw_progress_bar(win: &window::JoshutoPanel, percentage: f32) {
    let cols: i32 = (win.cols as f32 * percentage) as i32;
    ncurses::mvwchgat(
        win.win,
        0,
        0,
        cols,
        ncurses::A_STANDOUT(),
        THEME_T.selection.colorpair,
    );
    win.queue_for_refresh();
}

pub fn get_theme_attr(
    mut attr: ncurses::attr_t,
    entry: &JoshutoDirEntry,
) -> ((usize, &str), ncurses::attr_t, i16) {
    use std::os::unix::fs::FileTypeExt;
    use std::os::unix::fs::PermissionsExt;

    let theme: &JoshutoColorTheme;
    let colorpair: i16;

    let file_type = &entry.metadata.file_type;
    if entry.is_selected() {
        theme = &THEME_T.selection;
        colorpair = THEME_T.selection.colorpair;
    } else if file_type.is_dir() {
        theme = &THEME_T.directory;
        colorpair = THEME_T.directory.colorpair;
    } else if file_type.is_symlink() {
        theme = &THEME_T.link;
        colorpair = THEME_T.link.colorpair;
    } else if file_type.is_block_device()
        || file_type.is_char_device()
        || file_type.is_fifo()
        || file_type.is_socket()
    {
        theme = &THEME_T.socket;
        colorpair = THEME_T.link.colorpair;
    } else {
        let mode = entry.metadata.permissions.mode();
        if unix::is_executable(mode) {
            theme = &THEME_T.executable;
            colorpair = THEME_T.executable.colorpair;
        } else if let Some(ext) = entry.file_name().rfind('.') {
            let extension: &str = &entry.file_name()[ext + 1..];
            if let Some(s) = THEME_T.ext.get(extension) {
                theme = &s;
                colorpair = theme.colorpair;
            } else {
                theme = &THEME_T.regular;
                colorpair = theme.colorpair;
            }
        } else {
            theme = &THEME_T.regular;
            colorpair = theme.colorpair;
        }
    }

    if theme.bold {
        attr |= ncurses::A_BOLD();
    }
    if theme.underline {
        attr |= ncurses::A_UNDERLINE();
    }

    let prefix = match theme.prefix.as_ref() {
        Some(p) => (p.size(), p.prefix()),
        None => (1, " "),
    };

    (prefix, attr, colorpair)
}

fn file_size_to_string_detailed(mut file_size: f64) -> String {
    const FILE_UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "EB"];
    const CONV_RATE: f64 = 1024.0;

    let mut index = 0;
    while file_size > CONV_RATE {
        file_size /= CONV_RATE;
        index += 1;
    }

    if file_size >= 1000.0 {
        format!("{:.0}{}", file_size, FILE_UNITS[index])
    } else if file_size >= 100.0 {
        format!(" {:.0}{}", file_size, FILE_UNITS[index])
    } else if file_size >= 10.0 {
        format!("{:.1}{}", file_size, FILE_UNITS[index])
    } else {
        format!("{:.2}{}", file_size, FILE_UNITS[index])
    }
}

fn file_mtime_to_string(mtime: time::SystemTime) -> String {
    const MTIME_FORMATTING: &str = "%Y-%m-%d %H:%M";

    let datetime: chrono::DateTime<chrono::offset::Utc> = mtime.into();
    datetime.format(MTIME_FORMATTING).to_string()
}

fn file_size_to_string(mut file_size: f64) -> String {
    const FILE_UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "E"];
    const CONV_RATE: f64 = 1024.0;

    let mut index = 0;
    while file_size > CONV_RATE {
        file_size /= CONV_RATE;
        index += 1;
    }

    if file_size >= 100.0 {
        format!(" {:.0} {}", file_size, FILE_UNITS[index])
    } else if file_size >= 10.0 {
        format!(" {:.1} {}", file_size, FILE_UNITS[index])
    } else {
        format!(" {:.2} {}", file_size, FILE_UNITS[index])
    }
}
