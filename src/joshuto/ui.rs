extern crate ncurses;
extern crate unicode_width;

use std::fs;
use std::time;

use joshuto::context::JoshutoContext;
use joshuto::structs;
use joshuto::config;
use joshuto::unix;
use joshuto::window;

pub const ERR_COLOR: i16 = 240;
pub const EMPTY_COLOR: i16 = 241;

pub fn init_ncurses(theme_t: &config::JoshutoTheme)
{
    let locale_conf = ncurses::LcCategory::all;

    ncurses::setlocale(locale_conf, "");

    ncurses::initscr();
    ncurses::cbreak();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::start_color();
    ncurses::use_default_colors();
    ncurses::noecho();
    ncurses::set_escdelay(0);

    process_theme(&theme_t);

    ncurses::printw("Loading...");
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    ncurses::refresh();
}

fn process_theme(theme_t: &config::JoshutoTheme)
{
    for pair in theme_t.colorpair.iter() {
        ncurses::init_pair(pair.id, pair.fg, pair.bg);
    }

    /* error message */
    ncurses::init_pair(ERR_COLOR, ncurses::COLOR_RED, -1);
    /* empty */
    ncurses::init_pair(EMPTY_COLOR, ncurses::COLOR_WHITE, ncurses::COLOR_RED);
}

pub fn end_ncurses()
{
        ncurses::endwin();
}

pub fn getmaxyx() -> (i32, i32)
{
    let mut term_rows: i32 = 0;
    let mut term_cols: i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);
    (term_rows, term_cols)
}

pub fn display_options(win: &window::JoshutoPanel, vals: &Vec<String>)
{
    ncurses::werase(win.win);
    ncurses::mvwhline(win.win, 0, 0, 0, win.cols);

    for (i, val) in vals.iter().enumerate() {
        ncurses::wmove(win.win, (i+1) as i32, 0);
        ncurses::waddstr(win.win, val.as_str());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_msg(win: &window::JoshutoPanel, msg: &str)
{
    ncurses::werase(win.win);
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_err(win: &window::JoshutoPanel, msg: &str)
{
    ncurses::werase(win.win);
    ncurses::wattron(win.win, ncurses::A_BOLD());
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::wattroff(win.win, ncurses::A_BOLD());
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_empty(win: &window::JoshutoPanel, msg : &str)
{
    ncurses::werase(win.win);
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(EMPTY_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(EMPTY_COLOR));
    ncurses::wnoutrefresh(win.win);
}

fn wprint_file_name(win: ncurses::WINDOW, file_name: &str,
        coord: (i32, i32), mut space_avail: usize)
{
    let name_visual_space = unicode_width::UnicodeWidthStr::width(file_name);
    if name_visual_space < space_avail {
        ncurses::waddstr(win, &file_name);
        return;
    }
    if let Some(ext) = file_name.rfind('.') {
        let extension: &str = &file_name[ext..];
        let ext_len = unicode_width::UnicodeWidthStr::width(extension);
        if space_avail > ext_len {
            space_avail = space_avail - ext_len;
            ncurses::mvwaddstr(win, coord.0, space_avail as i32, &extension);
        }
    }
    if space_avail > 2 {
        space_avail = space_avail - 2;
    }

    ncurses::wmove(win, coord.0, coord.1);

    let mut trim_index: usize = file_name.len();

    let mut total: usize = 0;
    for (index, ch) in file_name.char_indices() {
        if total >= space_avail {
            trim_index = index;
            break;
        }
        total = total + unicode_width::UnicodeWidthChar::width(ch).unwrap_or(2);
    }
    ncurses::waddstr(win, &file_name[..trim_index]);
    ncurses::waddstr(win, "…");
}

pub fn wprint_entry(win: &window::JoshutoPanel,
        file: &structs::JoshutoDirEntry, coord: (i32, i32))
{
    let space_avail: usize;
    if win.cols >= 1 {
        space_avail = win.cols as usize - 1;
    } else {
        space_avail = 0;
    }

    ncurses::mvwaddstr(win.win, coord.0, coord.1, " ");
    wprint_file_name(win.win, &file.file_name_as_string, (coord.0, coord.1 + 1), space_avail);
}

pub fn wprint_entry_detailed(win: &window::JoshutoPanel,
        file: &structs::JoshutoDirEntry, coord: (i32, i32))
{
    let mut space_avail: usize = win.cols as usize - 1;
    ncurses::mvwaddstr(win.win, coord.0, coord.1, " ");
    if !file.path.is_dir() {
        let file_size_string = file_size_to_string(file.metadata.len as f64);
        if space_avail > file_size_string.len() {
            space_avail = space_avail - file_size_string.len();
            ncurses::mvwaddstr(win.win, coord.0, space_avail as i32, &file_size_string);
        }
        ncurses::wmove(win.win, coord.0, coord.1 + 1);
        if space_avail > 1 {
            space_avail = space_avail - 1;
        }
    }
    wprint_file_name(win.win, &file.file_name_as_string, (coord.0, coord.1 + 1), space_avail);
}

pub fn wprint_file_mode(win: ncurses::WINDOW, file: &structs::JoshutoDirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    let mode = file.metadata.permissions.mode();

    ncurses::wattron(win, ncurses::COLOR_PAIR(6));
    ncurses::waddstr(win, unix::stringify_mode(mode).as_str());
    ncurses::wattroff(win, ncurses::COLOR_PAIR(6));
}

pub fn wprint_file_info(win: ncurses::WINDOW, file: &structs::JoshutoDirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    let mode = file.metadata.permissions.mode();

    let mtime_string = file_mtime_to_string(file.metadata.modified);
    ncurses::waddstr(win, &mtime_string);
    ncurses::waddch(win, ' ' as ncurses::chtype);

    if file.path.is_dir() {
        if mode >> 9 & unix::S_IFLNK >> 9 == mode >> 9 {
            if let Ok(path) = fs::read_link(&file.path) {
                ncurses::waddstr(win, " -> ");
                ncurses::waddstr(win, path.to_str().unwrap());
            }
        }
    } else {
        let file_size_string = file_size_to_string_detailed(file.metadata.len as f64);
        ncurses::waddstr(win, &file_size_string);
    }
}

pub fn redraw_tab_view(win: &window::JoshutoPanel, context: &JoshutoContext)
{
    let tab_len = context.tabs.len();
    if tab_len == 1 {
        ncurses::werase(win.win);
    } else {
        ncurses::wmove(win.win, 0, 0);
        ncurses::wattron(win.win, ncurses::A_BOLD());
        ncurses::waddstr(win.win, format!("{} {}", context.curr_tab_index + 1, tab_len).as_str());
        ncurses::wattroff(win.win, ncurses::A_BOLD());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn draw_progress_bar(theme_t: &config::JoshutoTheme,
        win: &window::JoshutoPanel, percentage: f32)
{
    let cols: i32 = (win.cols as f32 * percentage) as i32;
    ncurses::mvwchgat(win.win, 0, 0, cols, ncurses::A_STANDOUT(),
            theme_t.selection.colorpair);
}

pub fn file_attr_apply(theme_t: &config::JoshutoTheme,
        win: ncurses::WINDOW, coord: (i32, i32), mode: u32,
        extension: &str, attr: ncurses::attr_t)
{
    match mode & unix::BITMASK {
        unix::S_IFLNK | unix::S_IFCHR | unix::S_IFBLK => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, theme_t.socket.colorpair);
        },
        unix::S_IFSOCK | unix::S_IFIFO => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, theme_t.socket.colorpair);
        },
        unix::S_IFDIR => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, theme_t.directory.colorpair);
        },
        unix::S_IFREG => {
            if unix::is_executable(mode) == true {
                ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, theme_t.executable.colorpair);
            }
            else if let Some(s) = theme_t.ext.get(extension) {
                ncurses::mvwchgat(win, coord.0, coord.1, -1, attr, s.colorpair);
            } else {
                ncurses::mvwchgat(win, coord.0, coord.1, -1, attr, 0);
            }
        },
        _ => {},
    };
}

fn file_size_to_string_detailed(mut file_size: f64) -> String
{
    const FILE_UNITS: [&str ; 6] = ["B", "KB", "MB", "GB", "TB", "EB"];
    const CONV_RATE: f64 = 1024.0;

    let mut index = 0;
    while file_size > CONV_RATE {
        file_size = file_size / CONV_RATE;
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

fn file_mtime_to_string(mtime: time::SystemTime) -> String
{
    const MTIME_FORMATTING: &str = "%Y-%m-%d %H:%M";

    let datetime: chrono::DateTime<chrono::offset::Utc> = mtime.into();
    datetime.format(MTIME_FORMATTING).to_string()
}


fn file_size_to_string(mut file_size: f64) -> String
{
    const FILE_UNITS: [&str ; 6] = ["B", "K", "M", "G", "T", "E"];
    const CONV_RATE: f64 = 1024.0;

    let mut index = 0;
    while file_size > CONV_RATE {
        file_size = file_size / CONV_RATE;
        index += 1;
    }

    if file_size >= 1000.0 {
        format!("{:.0} {}", file_size, FILE_UNITS[index])
    } else if file_size >= 100.0 {
        format!(" {:.0} {}", file_size, FILE_UNITS[index])
    } else if file_size >= 10.0 {
        format!("{:.1} {}", file_size, FILE_UNITS[index])
    } else {
        format!("{:.2} {}", file_size, FILE_UNITS[index])
    }
}
