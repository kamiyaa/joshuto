extern crate ncurses;
extern crate wcwidth;

use std::ffi;
use std::fs;
use std::path;

use joshuto::config;
use joshuto::structs;
use joshuto::unix;
use joshuto::window;
use joshuto::keymapll::Keycode;

pub const DIR_COLOR     : i16 = 1;
pub const SOCK_COLOR    : i16 = 4;
pub const EXEC_COLOR    : i16 = 11;
pub const IMG_COLOR     : i16 = 12;
pub const VID_COLOR     : i16 = 13;
pub const SELECT_COLOR  : i16 = 25;
pub const ERR_COLOR     : i16 = 40;

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
    ncurses::init_pair(SELECT_COLOR, ncurses::COLOR_YELLOW, ncurses::COLOR_BLACK);
    /* error message */
    ncurses::init_pair(ERR_COLOR, ncurses::COLOR_WHITE, ncurses::COLOR_RED);

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

pub fn wprint_err(win : &window::JoshutoPanel, msg : &str)
{
    ncurses::werase(win.win);
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
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
    ncurses::mvwaddstr(win.win, 0, 0,
            format!("{}@{} ", username, hostname).as_str());
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(EXEC_COLOR));

    ncurses::wattron(win.win, ncurses::COLOR_PAIR(DIR_COLOR));
    ncurses::waddstr(win.win, path_str);
    ncurses::waddstr(win.win, "/");
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(DIR_COLOR));
    ncurses::waddstr(win.win, file_name);
    ncurses::wattroff(win.win, ncurses::A_BOLD());
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_mimetype(win: &window::JoshutoPanel, mimetype: &str)
{
    ncurses::werase(win.win);
    wprint_msg(&win, mimetype);
    ncurses::wnoutrefresh(win.win);
}

fn wprint_file_size(win: &window::JoshutoPanel, file: &fs::DirEntry,
    coord: (i32, i32)) -> usize
{
    const FILE_UNITS: [&str ; 6] = ["B", "K", "M", "G", "T", "E"];
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

fn wprint_file_name(win: &window::JoshutoPanel, file : &fs::DirEntry,
    coord: (i32, i32), offset: usize)
{
    let offset = offset + 3;
    ncurses::wmove(win.win, coord.0, coord.1);
    match file.file_name().into_string() {
        Ok(file_name) => {
            ncurses::waddstr(win.win, " ");
            let name_len = wcwidth::str_width(file_name.as_str()).unwrap_or(win.cols as usize);
            if name_len >= win.cols as usize - 1 {
                let mut trim_index: usize = win.cols as usize - offset;
                let mut total: usize = 0;
                for (index, ch) in file_name.char_indices() {
                    if total >= win.cols as usize - offset {
                        trim_index = index;
                        break;
                    }
                    total = total + wcwidth::char_width(ch).unwrap_or(2) as usize;
                }
                ncurses::waddstr(win.win, &file_name[..trim_index]);
                ncurses::waddstr(win.win, "…");
            } else {
                ncurses::waddstr(win.win, &file_name);
            }
        },
        Err(e) => {
            ncurses::waddstr(win.win, format!("{:?}", e).as_str());
        },
    };
}

pub fn wprint_file_info(win: ncurses::WINDOW, file: &fs::DirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    const FILE_UNITS: [&str ; 6] = ["B", "KB", "MB", "GB", "TB", "ExB"];
    const CONV_RATE: f64 = 1024.0;

    ncurses::werase(win);
    ncurses::wmove(win, 0, 0);
    match file.metadata() {
        Ok(metadata) => {
            let permissions : fs::Permissions = metadata.permissions();
            let mode = permissions.mode();

            let mut file_size = metadata.len() as f64;
            let mut index = 0;
            while file_size > CONV_RATE {
                file_size = file_size / CONV_RATE;
                index += 1;
            }
            let file_type: &str = unix::get_unix_filetype(mode);

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
            ncurses::waddstr(win, file_type);
        },
        Err(e) => {
            ncurses::waddstr(win, format!("{:?}", e).as_str());
        },
    };
    ncurses::wnoutrefresh(win);
}

pub fn wprint_direntry(win: &window::JoshutoPanel,
        file: &fs::DirEntry, coord: (i32, i32))
{
//    let offset = wprint_file_size(win, file, coord);
    let offset = 6;
    wprint_file_name(win, file, coord, offset);
}

pub fn display_contents(win : &window::JoshutoPanel,
        entry : &structs::JoshutoDirList) {
    use std::os::unix::fs::PermissionsExt;

    let mut mode : u32 = 0;

    let index = entry.index as usize;
    let dir_contents = &entry.contents;
    let vec_len = dir_contents.len();
    if vec_len == 0 {
        wprint_err(win, "empty");
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
    } else if index - offset + win.rows as usize >= vec_len {
        start = vec_len - win.rows as usize;
        end = vec_len;
    } else {
        start = index - offset;
        end = start + win.rows as usize;
    }

    ncurses::werase(win.win);
    ncurses::wmove(win.win, 0, 0);

    for i in start..end {
        let coord: (i32, i32) = (i as i32 - start as i32, 0);
        wprint_direntry(win, &dir_contents[i].entry, coord);

        if let Ok(metadata) = &dir_contents[i].entry.metadata() {
            mode = metadata.permissions().mode();
        }

        if dir_contents[i].selected {
            if index == i {
                ncurses::mvwchgat(win.win, coord.0, coord.1, -1, ncurses::A_BOLD() | ncurses::A_STANDOUT(), SELECT_COLOR);
            } else {
                ncurses::mvwchgat(win.win, coord.0, coord.1, -1, ncurses::A_BOLD(), SELECT_COLOR);
            }
        } else if mode != 0 {
            if index == i {
                file_attr_apply(win.win, coord, mode,
                    dir_contents[i].entry.path().extension(), ncurses::A_STANDOUT());
            } else {
                file_attr_apply(win.win, coord, mode,
                    dir_contents[i].entry.path().extension(), ncurses::A_NORMAL());
            }
        }

    }
    ncurses::wnoutrefresh(win.win);
}

pub fn display_options(win: &window::JoshutoPanel, vals: &Vec<String>)
{
    ncurses::werase(win.win);
    ncurses::mvwhline(win.win, 0, 0, '-' as u32, 10000);

    for (i, val) in vals.iter().enumerate() {
        ncurses::wmove(win.win, (i+1) as i32, 0);
        ncurses::waddstr(win.win, val.as_str());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn redraw_view(win: &window::JoshutoPanel,
        view: Option<&structs::JoshutoDirList>
        )
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
            if let Ok(file_name) = dirent.entry.file_name().into_string() {
                wprint_path(&joshuto_view.top_win, username, hostname,
                        curr_path, file_name.as_str());
                wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);
            }
        }
    }
}

pub fn resize_handler(config_t: &config::JoshutoConfig,
        joshuto_view: &mut window::JoshutoView,
        curr_path: &path::PathBuf,
        parent_view: Option<&structs::JoshutoDirList>,
        curr_view: Option<&structs::JoshutoDirList>,
        preview_view: Option<&structs::JoshutoDirList>)
{
    joshuto_view.redraw_views();
    ncurses::refresh();

    redraw_view(&joshuto_view.left_win, parent_view);
    redraw_view(&joshuto_view.mid_win, curr_view);
    redraw_view(&joshuto_view.right_win, preview_view);

    redraw_status(joshuto_view, curr_view, curr_path,
            &config_t.username, &config_t.hostname);

    ncurses::doupdate();
}

pub fn get_str(win: &window::JoshutoPanel, coord: (i32, i32)) -> Option<String>
{
    let mut user_input: String = String::new();

    let mut curs_x = coord.1;

    ncurses::mvwchgat(win.win, coord.0, curs_x, 1, ncurses::A_STANDOUT(), 0);
    ncurses::wrefresh(win.win);

    loop {
        let ch: i32 = ncurses::wgetch(win.win);

        ncurses::mvwchgat(win.win, coord.0, curs_x, 1, ncurses::A_NORMAL(), 0);

        if ch == Keycode::ESCAPE as i32 {
            return None;
        } else if ch == Keycode::ENTER as i32 {
            break;
        } else if ch == Keycode::BACKSPACE as i32 {
            let user_input_len = user_input.len();
            ncurses::mvwprintw(win.win, coord.0, coord.1 + user_input_len as i32 - 1, " ");
            if user_input_len > 0 {
                if curs_x - coord.1 >= user_input_len as i32 {
                    curs_x = curs_x - 1;
                    ncurses::mvwdelch(win.win, coord.0, curs_x);
                    user_input.pop();
                } else {
                    user_input.remove((curs_x - coord.1 - 1) as usize);
                    curs_x = curs_x - 1;
                }
            }
        } else if ch == Keycode::LEFT as i32 {
            if curs_x > coord.1 {
                curs_x = curs_x - 1;
            }
        } else if ch == Keycode::RIGHT as i32 {
            if curs_x < coord.1 + user_input.len() as i32 {
                curs_x = curs_x + 1;
            }
        } else {
            if curs_x - coord.1 >= user_input.len() as i32 {
                user_input.push(ch as u8 as char);
                curs_x = curs_x + 1;
            } else {
                user_input.insert((curs_x - coord.1) as usize, ch as u8 as char);
                curs_x = curs_x + 1;
            }
        }
        ncurses::mvwprintw(win.win, coord.0, coord.1, user_input.as_str());

        ncurses::mvwchgat(win.win, coord.0, curs_x, 1, ncurses::A_STANDOUT(), 0);
        ncurses::wrefresh(win.win);
    }

    return Some(user_input);
}


fn file_attr_apply(win : ncurses::WINDOW, coord : (i32, i32), mode : u32,
        file_extension : Option<&ffi::OsStr>, attr : ncurses::attr_t)
{
    match mode & unix::BITMASK {
        unix::S_IFDIR => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, DIR_COLOR);
        },
        unix::S_IFLNK | unix::S_IFCHR | unix::S_IFBLK => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, SOCK_COLOR);
        },
        unix::S_IFSOCK | unix::S_IFIFO => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, SOCK_COLOR);
        },
        unix::S_IFREG => {
            if unix::is_executable(mode) == true {
                ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, EXEC_COLOR);
            }
            else if let Some(extension) = file_extension {
                if let Some(ext) = extension.to_str() {
                    file_ext_attr_apply(win, coord, ext, attr);
                }
            } else {
                    ncurses::mvwchgat(win, coord.0, coord.1, -1, attr, 0);
            }
        },
        _ => {},
    };
}

fn file_ext_attr_apply(win : ncurses::WINDOW, coord : (i32, i32), ext : &str,
        attr : ncurses::attr_t)
{
    match ext {
        "png" | "jpg" | "jpeg" | "gif" => {
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
