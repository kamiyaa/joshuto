extern crate ncurses;

use std::ffi;
use std::fs;
use std::path;

use joshuto::structs;
use joshuto::unix;

pub const DIR_COLOR     : i16 = 1;
pub const SOCK_COLOR    : i16 = 4;
pub const EXEC_COLOR    : i16 = 11;
pub const IMG_COLOR     : i16 = 12;
pub const VID_COLOR     : i16 = 13;
pub const ERR_COLOR     : i16 = 40;

pub fn init_ncurses()
{
    let locale_conf = ncurses::LcCategory::all;

    ncurses::setlocale(locale_conf, "");

    ncurses::initscr();
    ncurses::cbreak();
    ncurses::raw();
    ncurses::noecho();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::start_color();
    ncurses::use_default_colors();

//    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

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
    /* error message */
    ncurses::init_pair(ERR_COLOR, ncurses::COLOR_WHITE, ncurses::COLOR_RED);

    ncurses::refresh();
}

pub fn wprint_msg(win : &structs::JoshutoWindow, err_msg : &str)
{
    ncurses::werase(win.win);
    ncurses::mvwaddstr(win.win, 0, 0, err_msg);
    ncurses::wnoutrefresh(win.win);
}
pub fn wprint_err(win : &structs::JoshutoWindow, err_msg : &str)
{
    ncurses::werase(win.win);
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0, err_msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_path(win : &structs::JoshutoWindow, username : &str,
        hostname : &str, path : &path::PathBuf)
{
    ncurses::werase(win.win);
    let path_str : &str = match path.to_str() {
            Some(s) => s,
            None => "Error",
        };
    ncurses::wattron(win.win, ncurses::A_BOLD());
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(EXEC_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0,
            format!("{}@{} ", username, hostname).as_str());
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(EXEC_COLOR));

    ncurses::waddstr(win.win, path_str);
    ncurses::wattroff(win.win, ncurses::A_BOLD());
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_file_info(win : ncurses::WINDOW, file : &fs::DirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    const FILE_UNITS : [&str ; 6] = ["B", "KB", "MB", "GB", "TB", "ExB"];
    const CONV_RATE : u64 = 1024;

    ncurses::werase(win);
    ncurses::wmove(win, 0, 0);
    match file.metadata() {
        Ok(metadata) => {
            let permissions : fs::Permissions = metadata.permissions();
            let mode = permissions.mode();

            let mut file_size = metadata.len();
            let mut index = 0;
            while file_size > CONV_RATE {
                file_size = file_size / CONV_RATE;
                index += 1;
            }

            ncurses::waddstr(win,
                format!("{:?} {}  {} {}", mode, unix::stringify_mode(mode),
                    file_size, FILE_UNITS[index]).as_str()
                );
        },
        Err(e) => {
            ncurses::waddstr(win, format!("{:?}", e).as_str());
        },
    };
    ncurses::wnoutrefresh(win);
}

pub fn display_contents(win : &structs::JoshutoWindow, entry : &structs::JoshutoDirEntry) {
    use std::os::unix::fs::PermissionsExt;

    let mut mode : u32 = 0;

    let index = entry.index;
    let dir_contents = entry.contents.as_ref().unwrap();
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
        ncurses::wmove(win.win, i as i32 - start as i32, 0);
        wprint_file(win, &dir_contents[i]);

        if let Ok(metadata) = &dir_contents[i].metadata() {
            mode = metadata.permissions().mode();
        }
        if mode != 0 {
            if index == i {
                file_attr_apply(win.win, (i as i32 - start as i32, 0), mode,
                    dir_contents[i].path().extension(), ncurses::A_STANDOUT());
            } else {
                file_attr_apply(win.win, (i as i32 - start as i32, 0), mode,
                    dir_contents[i].path().extension(), ncurses::A_NORMAL());
            }
        }

    }
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_file(win : &structs::JoshutoWindow, file : &fs::DirEntry)
{
    match file.file_name().into_string() {
        Ok(file_name) => {
            ncurses::waddstr(win.win, " ");
            let name_len = file_name.len();
            if name_len >= win.cols as usize {
                let mut shortened = String::with_capacity(
                        win.cols as usize);
                let mut iter = file_name.chars();
                let mut i : usize = 0;
                while i < win.cols as usize {
                    if let Some(ch) = iter.next() {
                        shortened.push(ch);
                        i += ch.len_utf8();
                    }
                }
                shortened.push('…');
                ncurses::waddstr(win.win, &shortened);
            } else {
                ncurses::waddstr(win.win, &file_name);
            }
        },
        Err(e) => {
            ncurses::waddstr(win.win, format!("{:?}", e).as_str());
        },
    };
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
