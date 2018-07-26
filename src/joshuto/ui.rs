extern crate ncurses;

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
    // ncurses::raw();
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

pub fn wprintmsg(win : &structs::JoshutoWindow, err_msg : &str)
{
    ncurses::werase(win.win);
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::mvwprintw(win.win, 0, 0, err_msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
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
    ncurses::mvwprintw(win.win, 0, 0,
            format!("{}@{} ", username, hostname).as_str());
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(EXEC_COLOR));

    ncurses::wprintw(win.win, path_str);
    ncurses::wattroff(win.win, ncurses::A_BOLD());
    ncurses::wnoutrefresh(win.win);
}

/*
pub fn wprint_file_preview(win : &structs::JoshutoWindow,
        direntry : &fs::DirEntry,
        sort_func : fn (&fs::DirEntry, &fs::DirEntry) -> std::cmp::Ordering,
        show_hidden : bool)
{
    use std::os::unix::fs::PermissionsExt;

    ncurses::werase(win.win);
    if let Ok(metadata) = direntry.metadata() {
        let permissions : fs::Permissions = metadata.permissions();
        let mode = permissions.mode();

        match mode & unix::BITMASK {
            unix::S_IFDIR => {
                match joshuto::read_dir_list(&direntry.path().to_str().unwrap(), show_hidden) {
                    Ok(mut dir_contents) => {
                        dir_contents.sort_by(sort_func);
                    //    win.display_contents(&dir_contents, 0);
                    },
                    Err(e) => {
                        wprintmsg(&win, format!("{}", e).as_str());
                    },
                };
            },
            unix::S_IFLNK => {
                let mut file_path = direntry.path();
                match fs::read_link(&file_path) {
                    Ok(sym_path) => {
                        file_path.pop();
                        file_path.push(sym_path.as_path());
                        if file_path.as_path().is_dir() {
                            match joshuto::read_dir_list(file_path.to_str().unwrap(), show_hidden) {
                                Ok(mut dir_contents) => {
                                    dir_contents.sort_by(sort_func);
                                //    win.display_contents(&dir_contents, 0);
                                },
                                Err(e) => {
                                    wprintmsg(&win, format!("{}", e).as_str());
                                },
                            };
                        } else {
                            ncurses::wprintw(win.win, "Symlink pointing to a file");
                        }
                    },
                    Err(e) => {
                        wprintmsg(&win, format!("{}", e).as_str());
                    },
                };
            },
            unix::S_IFBLK => {
                ncurses::wprintw(win.win, "Block file");
            },
            unix::S_IFSOCK => {
                ncurses::wprintw(win.win, "Socket file");
            },
            unix::S_IFCHR => {
                ncurses::wprintw(win.win, "Character file");
            },
            unix::S_IFIFO => {
                ncurses::wprintw(win.win, "FIFO file");
            },
            unix::S_IFREG => {
                ncurses::wprintw(win.win, "Plain file");
            },
            _ => {
                ncurses::wprintw(win.win, "Unknown file");
            },
        }
    }
    ncurses::wnoutrefresh(win.win);
}*/

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

            ncurses::wprintw(win,
                format!("{:?} {}  {} {}", mode, unix::stringify_mode(mode),
                    file_size, FILE_UNITS[index]).as_str()
                );
        },
        Err(e) => {
            ncurses::wprintw(win, format!("{:?}", e).as_str());
        },
    };
    ncurses::wnoutrefresh(win);
}
