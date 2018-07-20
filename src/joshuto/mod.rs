extern crate ncurses;

use std;
use std::env;
use std::ffi;
use std::fs;
use std::path;
use std::process;

use JoshutoConfig;

mod sort;
mod unix;
mod structs;

const QUIT      : i32 = 'q' as i32;
const ENTER     : i32 = '\n' as i32;

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
    ncurses::raw();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    ncurses::start_color();
    ncurses::use_default_colors();

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

    ncurses::init_pair(ERR_COLOR, ncurses::COLOR_WHITE, ncurses::COLOR_RED);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

pub fn list_dirent(path : &str) -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    match fs::read_dir(path) {
        Ok(results) => {
            let mut result_vec : Vec<fs::DirEntry> = results
                    .filter_map(sort::filter_func_hidden_files)
                    .collect();
            Ok(result_vec)
        },
        Err(e) => {
            Err(e)
        },
    }
}

pub fn list_dirent_hidden(path : &str) -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    match fs::read_dir(path) {
        Ok(results) => {
            let results : Result<Vec<fs::DirEntry>, _> = results.collect();
            results
        },
        Err(e) => {
            Err(e)
        },
    }
}

pub fn read_dir_list(path : &str, show_hidden : bool) -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    if show_hidden {
        list_dirent_hidden(path)
    } else {
        list_dirent(path)
    }
}

pub fn wprintmsg(win : &structs::JoshutoWindow, err_msg : &str)
{
    ncurses::wclear(win.win);
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::mvwprintw(win.win, 0, 0, err_msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    win.commit();
}

pub fn wprint_path(win : &structs::JoshutoWindow, path : &path::PathBuf)
{
    ncurses::wclear(win.win);
    let path_str : &str = match path.to_str() {
            Some(s) => s,
            None => "Error",
        };

    ncurses::wattron(win.win, ncurses::A_BOLD());
    ncurses::mvwprintw(win.win, 0, 0, path_str);
    ncurses::wattroff(win.win, ncurses::A_BOLD());
    win.commit();
}

pub fn wprint_pdir(win : &structs::JoshutoWindow, path : &path::PathBuf,
        index : usize, 
        sort_func : fn (&fs::DirEntry, &fs::DirEntry) -> std::cmp::Ordering,
        show_hidden : bool)
{
    ncurses::wclear(win.win);
    if let Some(ppath) = path.parent() {
        match read_dir_list(ppath.to_str().unwrap(), show_hidden) {
            Ok(mut dir_contents) => {
                dir_contents.sort_by(sort_func);
                win.display_contents(&dir_contents, index);
            },
            Err(e) => {
                wprintmsg(win, format!("{}", e).as_str());
            },
        };
    }
    win.commit();
}

pub fn wprint_file_preview(win : &structs::JoshutoWindow,
        direntry : &fs::DirEntry, show_hidden : bool)
{
    use std::os::unix::fs::PermissionsExt;

    ncurses::wclear(win.win);
    if let Ok(metadata) = direntry.metadata() {
        let permissions : fs::Permissions = metadata.permissions();
        let mode = permissions.mode();

        match mode & unix::BITMASK {
            unix::S_IFDIR => {
                match read_dir_list(&direntry.path().to_str().unwrap(), show_hidden) {
                    Ok(mut dir_contents) => {
                        dir_contents.sort_by(sort::alpha_sort);
                        win.display_contents(&dir_contents, 0);
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
                            match read_dir_list(file_path.to_str().unwrap(), show_hidden) {
                                Ok(mut dir_contents) => {
                                    dir_contents.sort_by(sort::alpha_sort);
                                    win.display_contents(&dir_contents, 0);
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
    win.commit();
}

pub fn win_print_file_info(win : ncurses::WINDOW, file : &fs::DirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    const FILE_UNITS : [&str ; 6] = ["B", "KB", "MB", "GB", "TB", "ExB"];
    const CONV_RATE : u64 = 1024;

    ncurses::wclear(win);
    ncurses::wmove(win, 0, 0);
    match file.metadata() {
        Ok(metadata) => {
            let permissions : fs::Permissions = metadata.permissions();
            let mode = permissions.mode();
            ncurses::wprintw(win, format!("{:?}", mode).as_str());
            ncurses::wprintw(win, " ");
            ncurses::wprintw(win, unix::stringify_mode(mode).as_str());
            ncurses::wprintw(win, "  ");

            let mut file_size = metadata.len();
            let mut index = 0;
            while file_size > CONV_RATE {
                file_size = file_size / CONV_RATE;
                index += 1;
            }
            ncurses::wprintw(win, format!("{} {}", file_size, FILE_UNITS[index]).as_str());
        },
        Err(e) => {
            ncurses::wprintw(win, format!("{:?}", e).as_str());
        },
    };
    ncurses::wrefresh(win);
}

pub fn enter_dir(direntry : &fs::DirEntry,
        view : &structs::JoshutoView, curr_path : &mut path::PathBuf,
        index : &mut usize,
        sort_func : fn (&fs::DirEntry, &fs::DirEntry) -> std::cmp::Ordering,
        show_hidden : bool) -> Option<Vec<fs::DirEntry>>
{
    let tmp_name : ffi::OsString = direntry.file_name();
    let tmp_name2 = tmp_name.as_os_str().to_str().unwrap();
    let file_name = path::Path::new(tmp_name2);
    curr_path.push(file_name);

    let dir_contents : Vec<fs::DirEntry>;

    match env::set_current_dir(&curr_path) {
        Ok(_s) => {
            match read_dir_list(".", show_hidden) {
                Ok(s) => {
                    dir_contents = s;
                }
                Err(e) => {
                    wprintmsg(&view.bot_win, format!("{}", e).as_str());
                    return None;
                }
            }
            *index = 0;

            wprint_path(&view.top_win, &curr_path);
            wprint_pdir(&view.left_win, &curr_path, *index, sort_func,
                    show_hidden);

            if dir_contents.len() > 0 {
                wprint_file_preview(&view.right_win, direntry, show_hidden);
            }
        },
        Err(e) => {
            wprintmsg(&view.bot_win,
                format!("{}", e).as_str());
            return None;
        }
    }

    return Some(dir_contents);
}


pub fn run(config : &mut JoshutoConfig)
{
    init_ncurses();

    let mut term_rows : i32 = 0;
    let mut term_cols : i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

    let mut index : usize = 0;
    let pindex : usize = 0;
    // let mut cindex : usize = 0;

    ncurses::refresh();

    /* height, width, y, x */
    let mut joshuto_view : structs::JoshutoView = structs::JoshutoView::new((1, 3, 4));

    /* TODO: mutable in the future */
    let sort_func : fn(file1 : &std::fs::DirEntry, file2 : &std::fs::DirEntry) -> std::cmp::Ordering
        = match config.sort_method {
            Some(ref method) => {
                if method == "natural" {
                    sort::alpha_sort
                } else {
                    sort::alpha_sort
                }
            },
            None => {
                sort::alpha_sort
            }
        };

    let mut show_hidden : bool = match config.show_hidden {
        Some(s) => s,
        None => false,
        };

    let mut curr_path : path::PathBuf =
        match env::current_dir() {
            Ok(path) => {
                path
            },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

    let mut dir_contents : Vec<fs::DirEntry> =
        match read_dir_list(".", show_hidden) {
            Ok(s) => {
                s
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
    dir_contents.sort_by(&sort_func);

    wprint_path(&joshuto_view.top_win, &curr_path);

    wprint_pdir(&joshuto_view.left_win, &curr_path, pindex, sort_func,
            show_hidden);

    joshuto_view.mid_win.display_contents(&dir_contents, index);
    joshuto_view.mid_win.commit();

    if dir_contents.len() > 0 {
        wprint_file_preview(&joshuto_view.right_win, &dir_contents[index],
            show_hidden);
        win_print_file_info(joshuto_view.bot_win.win, &dir_contents[index]);
    }

    ncurses::refresh();

    loop {
        let ch : i32 = ncurses::getch();

        if ch == QUIT {
            break;
        }
        if ch == 'z' as i32 {
            let ch2 : i32 = ncurses::getch();
            if ch2 == 'h' as i32 {
                show_hidden = !show_hidden;

                match read_dir_list(".", show_hidden) {
                    Ok(s) => {
                        dir_contents = s;
                        dir_contents.sort_by(&sort_func);

                        index = 0;

                        wprint_pdir(&joshuto_view.left_win, &curr_path, pindex,
                                sort_func, show_hidden);

                        if dir_contents.len() > 0 {
                            wprint_file_preview(&joshuto_view.right_win,
                                    &dir_contents[index], show_hidden);
                            win_print_file_info(joshuto_view.bot_win.win,
                                    &dir_contents[index]);
                        }
                    },
                    Err(e) => {
                        wprintmsg(&joshuto_view.bot_win,
                                format!("{}", e).as_str());
                    },
                };

                ncurses::refresh();
            }
        } else if ch == ncurses::KEY_RESIZE {
            ncurses::clear();
            joshuto_view.redraw_views();
            ncurses::refresh();

            wprint_path(&joshuto_view.top_win, &curr_path);
            wprint_pdir(&joshuto_view.left_win, &curr_path, pindex, sort_func,
                    show_hidden);
            joshuto_view.mid_win.display_contents(&dir_contents, index);
            joshuto_view.mid_win.commit();
            if dir_contents.len() > 0 {
                wprint_file_preview(&joshuto_view.right_win,
                        &dir_contents[index], show_hidden);
                win_print_file_info(joshuto_view.bot_win.win,
                        &dir_contents[index]);
            }

            ncurses::refresh();

        } else if ch == ncurses::KEY_HOME {
            if index != 0 {
                index = 0;
                wprint_file_preview(&joshuto_view.right_win,
                        &dir_contents[index], show_hidden);
            }
        } else if ch == ncurses::KEY_END {
            let tmp_len = dir_contents.len() - 1;
            if index != tmp_len {
                index = tmp_len;
                wprint_file_preview(&joshuto_view.right_win,
                        &dir_contents[index], show_hidden);
            }
        } else if ch == ncurses::KEY_UP {
            if index > 0 {
                index = index - 1;
                wprint_file_preview(&joshuto_view.right_win,
                        &dir_contents[index], show_hidden);
            }
        } else if ch == ncurses::KEY_DOWN {
            if index + 1 < dir_contents.len() {
                index = index + 1;
                wprint_file_preview(&joshuto_view.right_win,
                        &dir_contents[index], show_hidden);
            }
        } else if ch == ncurses::KEY_NPAGE {
            let tmp_len = dir_contents.len();
            if index + 1 == tmp_len {
                continue;
            }
            if index + 5 < tmp_len {
                index = index + 5;
            } else {
                index = tmp_len - 1;
            }
            wprint_file_preview(&joshuto_view.right_win,
                    &dir_contents[index], show_hidden);
        } else if ch == ncurses::KEY_PPAGE {
            if index == 0 {
                continue;
            }
            if index >= 5 {
                index = index - 5;
            } else {
                index = 0;
            }
            wprint_file_preview(&joshuto_view.right_win,
                    &dir_contents[index], show_hidden);
        } else if ch == ncurses::KEY_LEFT {
            if curr_path.parent() == None {
                    ncurses::wclear(joshuto_view.left_win.win);
                    ncurses::wrefresh(joshuto_view.left_win.win);
                    continue;
            }
            if curr_path.pop() == false {
                    continue;
            }
            match env::set_current_dir(curr_path.as_path()) {
                Ok(_s) => {
                    match read_dir_list(".", show_hidden) {
                        Ok(s) => {
                            dir_contents = s;
                            dir_contents.sort_by(&sort_func);

                            index = pindex;

                            wprint_pdir(&joshuto_view.left_win, &curr_path,
                                pindex, sort_func, show_hidden);

                            wprint_path(&joshuto_view.top_win, &curr_path);
                            if dir_contents.len() > 0 {
                                wprint_file_preview(&joshuto_view.right_win,
                                        &dir_contents[index], show_hidden);
                            }
                        },
                        Err(e) => {
                            wprintmsg(&joshuto_view.bot_win, format!("{}", e).as_str());
                        },
                    };
                },
                Err(e) => {
                    eprintln!("{}", e);
                },
            };
        } else if ch == ncurses::KEY_RIGHT || ch == ENTER {
            if let Ok(file_type) = &dir_contents[index as usize].file_type() {
                if file_type.is_dir() {
                    if let Some(s) = enter_dir(&dir_contents[index as usize],
                            &joshuto_view, &mut curr_path, &mut index, sort_func,
                            show_hidden) {
                        dir_contents = s;
                        dir_contents.sort_by(&sort_func);
                        if dir_contents.len() > 0 {
                            wprint_file_preview(&joshuto_view.right_win,
                                    &dir_contents[index], show_hidden);
                        }
                    }
                } else if file_type.is_symlink() {
                    let mut file_path : path::PathBuf =
                            dir_contents[index as usize].path();
                    match fs::read_link(&file_path) {
                        Ok(sym_path) => {
                            file_path.pop();
                            file_path.push(sym_path.as_path());
                            if file_path.as_path().is_dir() {
                                if let Some(s) = enter_dir(&dir_contents[index as usize], &joshuto_view, &mut curr_path, &mut index, sort_func, show_hidden) {
                                    dir_contents = s;
                                    dir_contents.sort_by(&sort_func);
                                }
                            }
                        },
                        Err(e) => {
                            wprintmsg(&joshuto_view.bot_win,
                                format!("{}", e).as_str());
                        },
                    };
                } else {
                    let mut arg_list : Vec<String> = Vec::new();
                    arg_list.push(dir_contents[index as usize].file_name().into_string().unwrap());
                    wprintmsg(&joshuto_view.right_win, "Nice");
                }
            }
        } else {
            eprintln!("Unknown keychar: ({}: {})", ch as u32, ch);
        }

        if dir_contents.len() > 0 {
            joshuto_view.mid_win.display_contents(&dir_contents, index);
            joshuto_view.mid_win.commit();
            win_print_file_info(joshuto_view.bot_win.win, &dir_contents[index]);
        } else {
            wprintmsg(&joshuto_view.mid_win, "empty");
            ncurses::wclear(joshuto_view.right_win.win);
            ncurses::wrefresh(joshuto_view.right_win.win);
        }
    }
    ncurses::endwin();
}
