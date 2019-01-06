extern crate ncurses;
extern crate wcwidth;

use std::fs;
use std::path;

use joshuto::structs;
use joshuto::config::theme;
use joshuto::unix;
use joshuto::window;

pub const DIR_COLOR: i16 = 1;
pub const SOCK_COLOR: i16 = 4;
pub const EXEC_COLOR: i16 = 11;
pub const IMG_COLOR: i16 = 12;
pub const VID_COLOR: i16 = 13;
pub const SELECT_COLOR: i16 = 25;
pub const ERR_COLOR: i16 = 40;
pub const EMPTY_COLOR: i16 = 50;

lazy_static! {
    pub static ref theme_t: theme::JoshutoTheme = theme::JoshutoTheme::get_config();
}

pub fn init_ncurses()
{
    let locale_conf = ncurses::LcCategory::all;

    ncurses::setlocale(locale_conf, "");
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    ncurses::initscr();
    ncurses::cbreak();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::start_color();
    ncurses::use_default_colors();
    ncurses::noecho();
    ncurses::set_escdelay(0);

    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* directories */
    ncurses::init_pair(DIR_COLOR, ncurses::COLOR_BLUE, -1);
    /* Sockets */
    ncurses::init_pair(SOCK_COLOR, ncurses::COLOR_CYAN, -1);
    /* executables */
    ncurses::init_pair(EXEC_COLOR, ncurses::COLOR_GREEN, -1);
    /* image files */
    ncurses::init_pair(IMG_COLOR, ncurses::COLOR_YELLOW, -1);
    /* video files */
    ncurses::init_pair(VID_COLOR, ncurses::COLOR_MAGENTA, -1);
    /* selected files */
    ncurses::init_pair(SELECT_COLOR, ncurses::COLOR_YELLOW, -1);
    /* error message */
    ncurses::init_pair(ERR_COLOR, ncurses::COLOR_RED, -1);
    /* empty */
    ncurses::init_pair(EMPTY_COLOR, ncurses::COLOR_WHITE, ncurses::COLOR_RED);

    ncurses::printw("Loading...");

    ncurses::refresh();
}

pub fn end_ncurses()
{
        ncurses::endwin();
}

pub fn wprint_msg(win : &window::JoshutoPanel, msg : &str)
{
    ncurses::werase(win.win);
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_err(win: &window::JoshutoPanel, msg : &str)
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

pub fn wprint_path(win: &window::JoshutoPanel, username: &str,
        hostname: &str, path: &path::PathBuf, file_name: &str)
{
    ncurses::werase(win.win);
    let path_str: &str = match path.to_str() {
            Some(s) => s,
            None => "Error",
        };
    ncurses::wattron(win.win, ncurses::A_BOLD());
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(EXEC_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0, username);
    ncurses::waddstr(win.win, "@");
    ncurses::waddstr(win.win, hostname);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(EXEC_COLOR));

    ncurses::waddstr(win.win, " ");

    ncurses::wattron(win.win, ncurses::COLOR_PAIR(DIR_COLOR));
    ncurses::waddstr(win.win, path_str);
    ncurses::waddstr(win.win, "/");
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(DIR_COLOR));
    ncurses::waddstr(win.win, file_name);
    ncurses::wattroff(win.win, ncurses::A_BOLD());
    ncurses::wnoutrefresh(win.win);
}

/*

fn wprint_file_size(win: &window::JoshutoPanel, file: &fs::DirEntry,
    coord: (i32, i32)) -> usize
{
    const FILE_UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "E"];
    const CONV_RATE: f64 = 1024.0;

    match file.metadata() {
        Ok(metadata) => {
            let mut file_size = metadata.len() as f64;
            let mut index = 0;
            while file_size > CONV_RATE {
                file_size = file_size / CONV_RATE;
                index += 1;
            }

            ncurses::wmove(win.win, coord.0, win.cols - 6);
            if file_size >= 1000.0 {
                ncurses::waddstr(win.win,
                    format!("{:.0} {}", file_size, FILE_UNITS[index]).as_str());
            } else if file_size >= 100.0 {
                ncurses::waddstr(win.win,
                    format!(" {:.0} {}", file_size, FILE_UNITS[index]).as_str());
            } else if file_size >= 10.0 {
                ncurses::waddstr(win.win,
                    format!("{:.1} {}", file_size, FILE_UNITS[index]).as_str());
            } else {
                ncurses::waddstr(win.win,
                    format!("{:.2} {}", file_size, FILE_UNITS[index]).as_str());
            }
        },
        Err(e) => {
            ncurses::waddstr(win.win, format!("{:?}", e).as_str());
        },
    };
    6
}
*/

fn wprint_file_name(win: &window::JoshutoPanel, file: &structs::JoshutoDirEntry,
        coord: (i32, i32))
{
    ncurses::mvwaddstr(win.win, coord.0, coord.1, " ");

    let file_name = &file.file_name_as_string;
    let name_visual_space = wcwidth::str_width(file_name).unwrap_or(win.cols as usize);
    if name_visual_space + 1 < win.cols as usize {
        ncurses::waddstr(win.win, &file_name);
        return;
    }

    let mut win_cols = win.cols;

    if let Some(ext) = file_name.rfind('.') {
        let extension: &str = &file_name[ext..];
        let ext_len = wcwidth::str_width(extension).unwrap_or(extension.len());
        win_cols = win_cols - ext_len as i32;
        ncurses::mvwaddstr(win.win, coord.0, win_cols, &extension);
    }
    win_cols = win_cols - 2;

    ncurses::wmove(win.win, coord.0, coord.1 + 1);

    let mut trim_index: usize = file_name.len();

    let mut total: usize = 0;
    for (index, ch) in file_name.char_indices() {
        if total >= win_cols as usize {
            trim_index = index;
            break;
        }
        total = total + wcwidth::char_width(ch).unwrap_or(2) as usize;
    }

    ncurses::waddstr(win.win, &file_name[..trim_index]);
    ncurses::waddstr(win.win, "…");
}

pub fn wprint_file_info(win: ncurses::WINDOW, file: &structs::JoshutoDirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    const FILE_UNITS: [&str ; 6] = ["B", "KB", "MB", "GB", "TB", "EB"];
    const CONV_RATE: f64 = 1024.0;

    ncurses::werase(win);
    ncurses::wmove(win, 0, 0);
    match fs::symlink_metadata(&file.path) {
        Ok(metadata) => {
            let permissions: fs::Permissions = metadata.permissions();
            let mode = permissions.mode();

            let mut file_size = metadata.len() as f64;
            let mut index = 0;
            while file_size > CONV_RATE {
                file_size = file_size / CONV_RATE;
                index += 1;
            }

            ncurses::waddstr(win, unix::stringify_mode(mode).as_str());
            ncurses::waddstr(win, "  ");
            if file_size >= 1000.0 {
                ncurses::waddstr(win,
                    format!("{:.0}{}", file_size, FILE_UNITS[index]).as_str());
            } else if file_size >= 100.0 {
                ncurses::waddstr(win,
                    format!(" {:.0}{}", file_size, FILE_UNITS[index]).as_str());
            } else if file_size >= 10.0 {
                ncurses::waddstr(win,
                    format!("{:.1}{}", file_size, FILE_UNITS[index]).as_str());
            } else {
                ncurses::waddstr(win,
                    format!("{:.2}{}", file_size, FILE_UNITS[index]).as_str());
            }
            ncurses::waddstr(win, " ");
            if mode >> 9 & unix::S_IFLNK >> 9 == mode >> 9 {
                if let Ok(path) = fs::read_link(&file.path) {
                    ncurses::waddstr(win, " -> ");
                    ncurses::waddstr(win, path.to_str().unwrap());
                }
            }
        },
        Err(e) => {
            ncurses::waddstr(win, e.to_string().as_str());
        },
    };
    ncurses::wnoutrefresh(win);
}

pub fn wprint_direntry(win: &window::JoshutoPanel,
        file: &structs::JoshutoDirEntry, coord: (i32, i32))
{
//    let offset = wprint_file_size(win, file, coord);
//    let offset = 3;
    wprint_file_name(win, file, coord);
}

pub fn display_options(win: &window::JoshutoPanel, vals: &Vec<String>)
{
    ncurses::werase(win.win);

    let ch = '-' as ncurses::chtype;
    ncurses::mvwhline(win.win, 0, 0, ch, 10000);

    for (i, val) in vals.iter().enumerate() {
        ncurses::wmove(win.win, (i+1) as i32, 0);
        ncurses::waddstr(win.win, val.as_str());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn redraw_view(win: &window::JoshutoPanel,
        view: Option<&structs::JoshutoDirList>)
{
    if let Some(s) = view {
        s.display_contents(win);
        ncurses::wnoutrefresh(win.win);
    } else {
        ncurses::werase(win.win);
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn redraw_status(joshuto_view : &window::JoshutoView,
    curr_view: Option<&structs::JoshutoDirList>, curr_path: &path::PathBuf,
    username: &str, hostname: &str)
{
    if let Some(s) = curr_view.as_ref() {
        let dirent = s.get_curr_entry();
        if let Some(dirent) = dirent {
            wprint_path(&joshuto_view.top_win, username, hostname,
                    curr_path, dirent.file_name_as_string.as_str());
            wprint_file_info(joshuto_view.bot_win.win, &dirent);
        }
    }
}

pub fn draw_loading_bar(win: &window::JoshutoPanel, percentage: f32)
{
    let cols: i32 = (win.cols as f32 * percentage) as i32;
    ncurses::mvwchgat(win.win, 0, 0, cols, ncurses::A_STANDOUT(), SELECT_COLOR);
}

pub fn display_contents(win: &window::JoshutoPanel,
        entry: &structs::JoshutoDirList) {
    use std::os::unix::fs::PermissionsExt;

    ncurses::werase(win.win);
    if win.cols <= 6 {
        return;
    }

    let mut mode: u32 = 0;

    let index = entry.index as usize;
    let dir_contents = &entry.contents;
    let vec_len = dir_contents.len();
    if vec_len == 0 {
        wprint_empty(win, "empty");
        return;
    }

    let offset : usize = 8;
    let start : usize;
    let end : usize;

    if win.rows as usize >= vec_len {
        start = 0;
        end = vec_len;
    } else if index <= offset {
        start = 0;
        end = win.rows as usize;
    } else if index + win.rows as usize >= vec_len + offset  {
        start = vec_len - win.rows as usize;
        end = vec_len;
    } else {
        start = index - offset;
        end = start + win.rows as usize;
    }

    ncurses::wmove(win.win, 0, 0);

    for i in start..end {
        let coord: (i32, i32) = (i as i32 - start as i32, 0);
        wprint_direntry(win, &dir_contents[i], coord);

        if let Ok(metadata) = fs::symlink_metadata(&dir_contents[i].path) {
            mode = metadata.permissions().mode();
        }

        if dir_contents[i].selected {
            if index == i {
                ncurses::mvwchgat(win.win, coord.0, coord.1, -1, ncurses::A_BOLD() | ncurses::A_STANDOUT(), SELECT_COLOR);
            } else {
                ncurses::mvwchgat(win.win, coord.0, coord.1, -1, ncurses::A_BOLD(), SELECT_COLOR);
            }
        } else if mode != 0 {
            let file_name = &dir_contents[i].file_name_as_string;
            let mut extension: &str = "";
            if let Some(ext) = file_name.rfind('.') {
                extension = &file_name[ext+1..];
            }

            if index == i {
                file_attr_apply(win.win, coord, mode,
                    extension, ncurses::A_STANDOUT());
            } else {
                file_attr_apply(win.win, coord, mode,
                    extension, ncurses::A_NORMAL());
            }
        }

    }
    ncurses::wnoutrefresh(win.win);
}

fn file_attr_apply(win: ncurses::WINDOW, coord: (i32, i32), mode: u32,
        extension: &str, attr: ncurses::attr_t)
{
    match mode & unix::BITMASK {
        unix::S_IFLNK | unix::S_IFCHR | unix::S_IFBLK => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, SOCK_COLOR);
        },
        unix::S_IFSOCK | unix::S_IFIFO => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, SOCK_COLOR);
        },
        unix::S_IFDIR => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, DIR_COLOR);
        },
        unix::S_IFREG => {
            if unix::is_executable(mode) == true {
                ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, EXEC_COLOR);
            }
            else if extension.len() > 0 {
                file_ext_attr_apply(win, coord, extension, attr);
            } else {
                    ncurses::mvwchgat(win, coord.0, coord.1, -1, attr, 0);
            }
        },
        _ => {},
    };
}

fn file_ext_attr_apply(win: ncurses::WINDOW, coord: (i32, i32), ext: &str,
        attr: ncurses::attr_t)
{
    match ext {
        "png" | "jpg" | "jpeg" | "gif" | "svg" => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, attr, IMG_COLOR);
        },
        "mkv" | "mp4" | "mp3" | "flac" | "ogg" | "avi" | "wmv" | "wav" |
        "m4a" => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, attr, VID_COLOR);
        },
        _ => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, attr, 0);
        },
    }
}
