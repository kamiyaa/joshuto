extern crate ncurses;

use std;
use std::env;
use std::ffi;
use std::fs;
use std::path;
use std::process;

use JoshutoConfig;
use joshuto_sort;
use joshuto_unix;

const QUIT: i32 = 'q' as i32;
const ENTER: i32 = '\n' as i32;

pub fn init_ncurses()
{
    ncurses::initscr();
    ncurses::raw();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    ncurses::start_color();
    ncurses::use_default_colors();

    /* directories */
    ncurses::init_pair(1, ncurses::COLOR_BLUE, -1);
    ncurses::init_pair(2, ncurses::COLOR_CYAN, -1);

    /* Sockets */
    ncurses::init_pair(4, ncurses::COLOR_CYAN, -1);

    /* executables */
    ncurses::init_pair(11, ncurses::COLOR_GREEN, -1);

    ncurses::init_pair(99, ncurses::COLOR_WHITE, ncurses::COLOR_RED);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn file_attron(win : ncurses::WINDOW, mode : u32)
{
    use joshuto::joshuto_unix;

    match mode & joshuto_unix::BITMASK {
        joshuto_unix::S_IFDIR => {
            ncurses::wattron(win, ncurses::COLOR_PAIR(1));
        },
        joshuto_unix::S_IFLNK | joshuto_unix::S_IFCHR | joshuto_unix::S_IFBLK
         => {
            ncurses::wattron(win, ncurses::COLOR_PAIR(2));
        },
        joshuto_unix::S_IFSOCK | joshuto_unix::S_IFIFO => {
            ncurses::wattron(win, ncurses::COLOR_PAIR(4));
        },
        joshuto_unix::S_IFREG => {
            if joshuto_unix::is_executable(mode) {
                ncurses::wattron(win, ncurses::A_BOLD());
                ncurses::wattron(win, ncurses::COLOR_PAIR(11));
            }
        },
        _ => {},
    };
}

fn file_attroff(win : ncurses::WINDOW, mode : u32)
{
    use joshuto::joshuto_unix;

    match mode & joshuto_unix::BITMASK {
        joshuto_unix::S_IFDIR => {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(1));
        },
        joshuto_unix::S_IFLNK | joshuto_unix::S_IFCHR |
        joshuto_unix::S_IFBLK => {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(2));
        },
        joshuto_unix::S_IFSOCK | joshuto_unix::S_IFIFO => {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(4));
        },
        joshuto_unix::S_IFREG => {
            if joshuto_unix::is_executable(mode) {
                ncurses::wattroff(win, ncurses::A_BOLD());
                ncurses::wattroff(win, ncurses::COLOR_PAIR(11));
            }
        },
        _ => {},
    };
}

fn print_file(win : ncurses::WINDOW, file : &fs::DirEntry) {

    use std::os::unix::fs::PermissionsExt;
    use joshuto::joshuto_unix;

    let mut mode : u32 = joshuto_unix::S_IFREG;

    if let Ok(metadata) = file.metadata() {
        mode = metadata.permissions().mode();
    }
    if mode != joshuto_unix::S_IFREG {
        file_attron(win, mode);
    }

    match file.file_name().into_string() {
        Ok(file_name) => {
            ncurses::wprintw(win, " ");
            ncurses::wprintw(win, &file_name);
        },
        Err(e) => {
            ncurses::wprintw(win, format!("{:?}", e).as_str());
        },
    };
    if mode != joshuto_unix::S_IFREG {
        file_attroff(win, mode);
    }

    ncurses::wprintw(win, "\n");
}

pub fn win_print_err_msg(win : ncurses::WINDOW, err_msg : &str)
{
    ncurses::wclear(win);
    ncurses::wattron(win, ncurses::COLOR_PAIR(99));
    ncurses::mvwprintw(win, 0, 0, err_msg);
    ncurses::wattron(win, ncurses::COLOR_PAIR(99));
    ncurses::wrefresh(win);
}

pub fn dirent_list(path : &path::PathBuf) -> Result<Vec<fs::DirEntry>, std::io::Error>
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

pub fn win_print_path(win : ncurses::WINDOW, path : &path::PathBuf)
{
    ncurses::wclear(win);
    let path_str : &str =
        match path.to_str() {
            Some(s) => s,
            None => "Error",
        };

    ncurses::mvwprintw(win, 0, 0, path_str);
    ncurses::wrefresh(win);
}

pub fn win_contents_refresh(win : ncurses::WINDOW,
                dir_contents: &Vec<fs::DirEntry>, win_rows : usize) {

    let vec_len = dir_contents.len();

    if vec_len == 0 {
        win_print_err_msg(win, "empty");
        return;
    }

    let mut i : usize = 0;
    let win_rows : usize = win_rows + i;

    ncurses::wclear(win);
    ncurses::wmove(win, 0, 0);
    while i < vec_len && i < win_rows {
        print_file(win, &dir_contents[i]);
        i += 1;
    }
    ncurses::wrefresh(win);
}
pub fn win_contents_refresh_indexed_short(win : ncurses::WINDOW,
                    dir_contents: &Vec<fs::DirEntry>,
                    win_rows : usize, index : usize) {
    let vec_len = dir_contents.len();

    if vec_len == 0 {
        win_print_err_msg(win, "empty");
        return;
    }

    let mut i : usize = 0;
    let win_rows : usize = win_rows + i;

    ncurses::wclear(win);
    ncurses::wmove(win, 0, 0);

    while i < vec_len && i < win_rows {
        if i == index {
            ncurses::wattron(win, ncurses::A_REVERSE());
            print_file(win, &dir_contents[i]);
            ncurses::wattroff(win, ncurses::A_REVERSE());
        } else {
            print_file(win, &dir_contents[i]);
        }
        i += 1;
    }
    ncurses::wrefresh(win);
}

pub fn win_contents_refresh_indexed(win : ncurses::WINDOW,
                    dir_contents: &Vec<fs::DirEntry>,
                    win_rows : usize, index : usize) {

    let vec_len = dir_contents.len();

    if win_rows >= vec_len {
        win_contents_refresh_indexed_short(win, dir_contents, win_rows, index);
        return;
    }

    let offset : usize = 5;
    let start : usize;
    let end : usize;
    if index <= offset {
        start = 0;
        end = win_rows;
    } else if index - offset + win_rows >= vec_len {
        start = vec_len - win_rows;
        end = vec_len;
    } else {
        start = index - offset;
        end = start + win_rows;
    }

    ncurses::wclear(win);
    ncurses::wmove(win, 0, 0);

    for i in start..end {
        if index == i {
            ncurses::wattron(win, ncurses::A_REVERSE());
            print_file(win, &dir_contents[i]);
            ncurses::wattroff(win, ncurses::A_REVERSE());
        } else {
            print_file(win, &dir_contents[i]);
        }
    }
    ncurses::wrefresh(win);
}

pub fn win_print_parent_dir(win : ncurses::WINDOW, path : &path::PathBuf, index : usize, length : usize)
{
    ncurses::wclear(win);
    if let Some(ppath) = path.parent() {
        match fs::read_dir(ppath) {
            Ok(results) => {
                let results : Result<Vec<fs::DirEntry>, _> = results.collect();
                if let Ok(mut dir_contents) = results {
                    dir_contents.sort_by(joshuto_sort::alpha_sort);
                    win_contents_refresh_indexed(win, &dir_contents, length, index);
                }
            },
            Err(e) => {
                win_print_err_msg(win, format!("{}", e).as_str());
            },
        };
    }
    ncurses::wrefresh(win);
}

pub fn win_print_file_preview(win : ncurses::WINDOW, file : &fs::DirEntry,
                                length : usize)
{
    use std::os::unix::fs::PermissionsExt;
    use joshuto::joshuto_unix;

    ncurses::wclear(win);
    if let Ok(metadata) = file.metadata() {
        let permissions : fs::Permissions = metadata.permissions();
        let mode = permissions.mode();

        match mode & joshuto_unix::BITMASK {
            joshuto_unix::S_IFDIR => {
                match fs::read_dir(&file.path()) {
                    Ok(results) => {
                        let results : Result<Vec<fs::DirEntry>, _> = results.collect();
                        if let Ok(mut dir_contents) = results {
                            dir_contents.sort_by(joshuto_sort::alpha_sort);
                            win_contents_refresh(win, &dir_contents, length);
                        }
                    },
                    Err(e) => {
                        win_print_err_msg(win, format!("{}", e).as_str());
                    },
                };
            },
            joshuto_unix::S_IFLNK => {
                let mut file_path = file.path();
                match fs::read_link(&file_path) {
                    Ok(sym_path) => {
                        file_path.pop();
                        file_path.push(sym_path.as_path());
                        if file_path.as_path().is_dir() {
                            match fs::read_dir(file_path) {
                                Ok(results) => {
                                    let results : Result<Vec<fs::DirEntry>, _> = results.collect();
                                    if let Ok(mut dir_contents) = results {
                                        dir_contents.sort_by(joshuto_sort::alpha_sort);
                                        win_contents_refresh(win, &dir_contents, length);
                                    }
                                },
                                Err(e) => {
                                    win_print_err_msg(win, format!("{}", e).as_str());
                                },
                            };
                        } else {
                            ncurses::wprintw(win, "Symlink pointing to a file");
                        }
                    },
                    Err(e) => {
                        win_print_err_msg(win, format!("{}", e).as_str());
                    },
                };
            },
            joshuto_unix::S_IFBLK => {
                ncurses::wprintw(win, "Block file");
            },
            joshuto_unix::S_IFSOCK => {
                ncurses::wprintw(win, "Socket file");
            },
            joshuto_unix::S_IFCHR => {
                ncurses::wprintw(win, "Character file");
            },
            joshuto_unix::S_IFIFO => {
                ncurses::wprintw(win, "FIFO file");
            },
            joshuto_unix::S_IFREG => {
                ncurses::wprintw(win, "Plain file");
            },
            _ => {
                ncurses::wprintw(win, "Unknown file");
            },
        }
    }
    ncurses::wrefresh(win);
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
            ncurses::wprintw(win, joshuto_unix::stringify_mode(mode).as_str());
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

pub struct JoshutoView {
    top_win : ncurses::WINDOW,
    left_win : ncurses::WINDOW,
    mid_win : ncurses::WINDOW,
    right_win : ncurses::WINDOW,
    bot_win : ncurses::WINDOW,
    win_ratio : (i32, i32, i32),
}

pub fn create_joshuto_view(term_rows : i32, term_cols : i32,
        win_ratio : (i32, i32, i32)) -> JoshutoView
{
    let term_divide : i32 = term_cols / 7;
    let top_win = ncurses::newwin(1, term_cols, 0, 0);
    ncurses::scrollok(top_win, true);

    let left_win = ncurses::newwin(term_rows - 2,
        term_divide * win_ratio.0, 1, 0);

    let mid_win = ncurses::newwin(term_rows - 2,
        term_divide * win_ratio.1, 1, term_divide * win_ratio.0);

    let right_win = ncurses::newwin(term_rows - 2,
        term_divide * 3, 1, term_divide * win_ratio.2);
    let bot_win = ncurses::newwin(1, term_cols, term_rows - 1, 0);

    ncurses::refresh();

    JoshutoView {
        top_win,
        left_win,
        mid_win,
        right_win,
        bot_win,
        win_ratio,
    }
}

impl JoshutoView {
    fn destroy_views(&mut self) {
        let windows : [ncurses::WINDOW ; 5] = [
            self.top_win,
            self.mid_win,
            self.left_win,
            self.right_win,
            self.bot_win
            ];
        for win in windows.iter() {
            ncurses::delwin(*win);
        }
    }

    fn init_views(&mut self, term_rows : i32, term_cols : i32) {
        let term_divide : i32 = term_cols / 7;
        self.top_win = ncurses::newwin(1, term_cols, 0, 0);

        self.left_win = ncurses::newwin(term_rows - 2,
            term_divide * self.win_ratio.0, 1, 0);

        self.mid_win = ncurses::newwin(term_rows - 2,
            term_divide * self.win_ratio.1, 1, term_divide * self.win_ratio.0);

        self.right_win = ncurses::newwin(term_rows - 2,
            term_divide * 3, 1, term_divide * self.win_ratio.2);
        self.bot_win = ncurses::newwin(1, term_cols, term_rows - 1, 0);
    }

}

pub fn run(_config : &JoshutoConfig)
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
    let mut joshuto_view : JoshutoView =
            create_joshuto_view(term_rows, term_cols, (1, 3, 4));

    /* TODO: mutable in the future */
    let sort_func : fn(file1 : &std::fs::DirEntry, file2 : &std::fs::DirEntry) -> std::cmp::Ordering
        = joshuto_sort::alpha_sort;

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
        match dirent_list(&curr_path) {
            Ok(s) => {
                s
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
    dir_contents.sort_by(&sort_func);

    win_print_path(joshuto_view.top_win, &curr_path);

    win_print_parent_dir(joshuto_view.left_win, &curr_path, pindex, (term_rows - 2) as usize);

    win_contents_refresh_indexed(joshuto_view.mid_win, &dir_contents, (term_rows - 2) as usize, index);

    if dir_contents.len() > 0 {
        win_print_file_preview(joshuto_view.right_win, &dir_contents[index],
                                (term_rows - 2) as usize);
        win_print_file_info(joshuto_view.bot_win, &dir_contents[index]);
    }

    ncurses::refresh();

    loop {
        let ch = ncurses::getch();

        match ch {
            QUIT => {
                break;
            },
            ncurses::KEY_RESIZE => {
                ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

                joshuto_view.destroy_views();
                ncurses::clear();
                ncurses::refresh();
                joshuto_view.init_views(term_rows, term_cols);
                ncurses::refresh();

                win_print_path(joshuto_view.top_win, &curr_path);
                win_print_parent_dir(joshuto_view.left_win, &curr_path, pindex, (term_rows - 2) as usize);
                win_contents_refresh_indexed(joshuto_view.mid_win, &dir_contents,
                                            (term_rows - 2) as usize, index);
                if dir_contents.len() > 0 {
                    win_print_file_preview(joshuto_view.right_win, &dir_contents[index],
                                            (term_rows - 2) as usize);
                    win_print_file_info(joshuto_view.bot_win, &dir_contents[index]);
                }

                ncurses::refresh();

            },
            ncurses::KEY_HOME => {
                if index != 0 {
                    index = 0;
                    win_print_file_preview(joshuto_view.right_win, &dir_contents[index],
                            (term_rows - 2) as usize);
                }
            },
            ncurses::KEY_END => {
                let tmp_len = dir_contents.len();
                if index + 1 != tmp_len {
                    index = tmp_len - 1;
                    win_print_file_preview(joshuto_view.right_win, &dir_contents[index],
                            (term_rows - 2) as usize);
                }
            },
            ncurses::KEY_UP => {
                if index > 0 {
                    index = index - 1;
                    win_print_file_preview(joshuto_view.right_win, &dir_contents[index],
                            (term_rows - 2) as usize);
                }
            },
            ncurses::KEY_DOWN => {
                if index + 1 < dir_contents.len() {
                    index = index + 1;
                    win_print_file_preview(joshuto_view.right_win, &dir_contents[index],
                            (term_rows - 2) as usize);
                }
            },
            ncurses::KEY_NPAGE => {
                let tmp_len = dir_contents.len();
                if index + 1 == tmp_len {
                    continue;
                }
                if index + 5 < tmp_len {
                    index = index + 5;
                } else {
                    index = tmp_len - 1;
                }
                win_print_file_preview(joshuto_view.right_win, &dir_contents[index],
                        (term_rows - 2) as usize);
            },
            ncurses::KEY_PPAGE => {
                if index == 0 {
                    continue;
                }
                if index >= 5 {
                    index = index - 5;
                } else {
                    index = 0;
                }
                win_print_file_preview(joshuto_view.right_win, &dir_contents[index],
                        (term_rows - 2) as usize);
            },
            ncurses::KEY_LEFT => {
                if curr_path.parent() == None {
                        ncurses::wclear(joshuto_view.left_win);
                        ncurses::wrefresh(joshuto_view.left_win);
                        continue;
                }
                if curr_path.pop() == false {
                        continue;
                }
                match dirent_list(&curr_path) {
                    Ok(s) => {
                        dir_contents = s;
                        dir_contents.sort_by(&sort_func);

                        index = pindex;

                        win_print_parent_dir(joshuto_view.left_win, &curr_path, pindex,
                            (term_rows - 2) as usize);

                        win_print_path(joshuto_view.top_win, &curr_path);
                        win_print_file_preview(joshuto_view.right_win,
                            &dir_contents[index],
                            (term_rows - 2) as usize);
                    },
                    Err(e) => {
                        win_print_err_msg(joshuto_view.bot_win, format!("{}", e).as_str());
                    },
                };
            },
            ncurses::KEY_RIGHT | ENTER => {
                if let Ok(file_type) = &dir_contents[index as usize].file_type() {
                    if file_type.is_dir() {
                        let tmp_name : ffi::OsString = dir_contents[index as usize].file_name();
                        let tmp_name2 = tmp_name.as_os_str().to_str().unwrap();
                        let file_name = path::Path::new(tmp_name2);
                        curr_path.push(file_name);
                        match dirent_list(&curr_path) {
                            Ok(s) => {
                                dir_contents = s;
                                dir_contents.sort_by(&sort_func);
                            }
                            Err(_e) => {
                                process::exit(1);
                            }
                        }
                        index = 0;

                        win_print_path(joshuto_view.top_win, &curr_path);
                        win_print_parent_dir(joshuto_view.left_win, &curr_path, pindex,
                            (term_rows - 2) as usize);
                        if dir_contents.len() > 0 {
                            win_print_file_preview(joshuto_view.right_win,
                                &dir_contents[index],
                                (term_rows - 2) as usize);
                        }
                    } else if file_type.is_symlink() {
                        let mut file_path : path::PathBuf =
                                dir_contents[index as usize].path();
                        match fs::read_link(&file_path) {
                            Ok(sym_path) => {
                                file_path.pop();
                                file_path.push(sym_path.as_path());
                                if file_path.as_path().is_dir() {
                                    let tmp_name : ffi::OsString = dir_contents[index as usize].file_name();
                                    let tmp_name2 = tmp_name.as_os_str().to_str().unwrap();
                                    let file_name = path::Path::new(tmp_name2);
                                    curr_path.push(file_name);
                                    match dirent_list(&curr_path) {
                                        Ok(s) => {
                                            dir_contents = s;
                                            dir_contents.sort_by(&sort_func);
                                        }
                                        Err(_e) => {
                                            process::exit(1);
                                        }
                                    }
                                    index = 0;

                                    win_print_path(joshuto_view.top_win, &curr_path);
                                    win_print_parent_dir(joshuto_view.left_win, &curr_path, pindex,
                                        (term_rows - 2) as usize);
                                    if dir_contents.len() > 0 {
                                        win_print_file_preview(joshuto_view.right_win,
                                            &dir_contents[index],
                                            (term_rows - 2) as usize);
                                    }
                                }
                            },
                            Err(e) => {
                                win_print_err_msg(joshuto_view.bot_win, format!("{}", e).as_str());
                            },
                        };
                    } else {
                        win_print_err_msg(joshuto_view.right_win, "Nice");
                    }
                }
            },
            _ => {
                eprintln!("Unknown keychar: ({}: {})", ch as u32, ch);
            },
        };

        win_contents_refresh_indexed(joshuto_view.mid_win, &dir_contents,
                                            (term_rows - 2) as usize, index);
        win_print_file_info(joshuto_view.bot_win, &dir_contents[index]);
    }
    ncurses::endwin();
}
