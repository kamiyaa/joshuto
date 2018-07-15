extern crate ncurses;

use std;
use std::env;
use std::fs;
use std::path;
use std::process;

use JoshutoConfig;

const QUIT: i32 = 'q' as i32;
const ENTER: i32 = '\n' as i32;

mod joshuto_rwx {
    extern crate libc;

/*
    pub fn get_mode_str(mode : u32)
    {
        let mode_str : &str = match mode / 0o1000 {
            0o20 => "D",
            0o40 => "d",
            0o100 => "_",
            0o120 => "l",
        };
        let mode_str = concat!(mode_str, stringify_mode(mode));
    }*/


    pub fn stringify_mode(mode : u32) -> String
    {
        let mut mode_str : String = String::with_capacity(10);

        mode_str.push(match mode / 0o1000 {
                0o20 => '_',
                0o40 => 'd',
                0o100 => '-',
                0o120 => 'l',
                _ => '-',
            });

        const LIBC_VALS : [(u32, char) ; 9] = [
                (libc::S_IRUSR, 'r'),
                (libc::S_IWUSR, 'w'),
                (libc::S_IXUSR, 'x'),
                (libc::S_IRGRP, 'r'),
                (libc::S_IWGRP, 'w'),
                (libc::S_IXGRP, 'x'),
                (libc::S_IROTH, 'r'),
                (libc::S_IWOTH, 'w'),
                (libc::S_IXOTH, 'x'),
        ];

        for val in LIBC_VALS.iter() {
            if mode & val.0 != 0 {
                mode_str.push(val.1);
            } else {
                mode_str.push('-');
            }
        }
        mode_str
    }
}

pub mod joshuto_sort {

    use std::cmp;
    use std::fs;
    use std;

    pub fn alpha_sort(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> cmp::Ordering
    {
        fn res_ordering(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> Result<cmp::Ordering, std::io::Error> {
            let f1_type = file1.file_type()?;
            let f2_type = file2.file_type()?;

            if !f1_type.is_file() && f2_type.is_file() {
                Ok(cmp::Ordering::Less)
            } else if !f2_type.is_file() && f1_type.is_file() {
                Ok(cmp::Ordering::Greater)
            } else {
                let f1_name : std::string::String =
                    file1.file_name().as_os_str().to_str().unwrap().to_lowercase();
                let f2_name : std::string::String =
                    file2.file_name().as_os_str().to_str().unwrap().to_lowercase();
                if f1_name <= f2_name {
                    Ok(cmp::Ordering::Less)
                } else {
                    Ok(cmp::Ordering::Greater)
                }
            }
        }
        res_ordering(file1, file2).unwrap_or(cmp::Ordering::Less)
    }
}

pub fn run(config : &JoshutoConfig)
{
    init_ncurses();

    let mut term_rows : i32 = 0;
    let mut term_cols : i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

    let mut index : usize = 0;
    let mut pindex : usize = 0;
    // let mut cindex : usize = 0;

    ncurses::refresh();

    let mut top_win = ncurses::newwin(1, term_cols, 0, 0);
    let mut mid_win = ncurses::newwin(term_rows - 2, term_cols / 7 * 3,
                                        1, term_cols / 7);
    let mut left_win = ncurses::newwin(term_rows - 2, term_cols / 7, 1, 0);
    let mut right_win = ncurses::newwin(term_rows - 2, term_cols / 7 * 3,
                                        1, term_cols / 7 * 4);
    let mut bottom_win = ncurses::newwin(1, term_cols, term_rows - 1, 0);

    // ncurses::scrollok(mid_win, true);

    /* TODO: mutable in the future */
    let sort_func : fn(file1 : &std::fs::DirEntry, file2 : &std::fs::DirEntry) -> std::cmp::Ordering
        = joshuto_sort::alpha_sort;

    let mut dir_contents : Vec<fs::DirEntry>; // = Vec::new();
    match cwd_contents() {
        Ok(s) => {
            dir_contents = s;
            dir_contents.sort_by(sort_func);
        }
        Err(_e) => {
            process::exit(1);
        }
    }

    win_print_curr_path(top_win);
    win_print_curr_path(bottom_win);
    win_print_parent_dir(left_win, pindex, (term_rows - 1) as usize);

    win_contents_refresh_indexed(mid_win, &dir_contents, (term_rows - 1) as usize, index);

    if dir_contents.len() > 0 {
        win_print_select_file(right_win, &dir_contents[index], (term_rows - 1) as usize);
        win_print_file_info(bottom_win, &dir_contents[index]);
    }

    ncurses::refresh();

    loop {
        let ch = ncurses::getch();

        match ch {
            QUIT => {
                break;
            }
            ncurses::KEY_RESIZE => {
                ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

                ncurses::delwin(top_win);
                ncurses::delwin(mid_win);
                ncurses::delwin(left_win);
                ncurses::delwin(right_win);
                ncurses::delwin(bottom_win);


                top_win = ncurses::newwin(1, term_cols, 0, 0);
                mid_win = ncurses::newwin(term_rows - 2, term_cols / 7 * 3,
                                          term_cols / 7, 1);
                left_win = ncurses::newwin(term_rows - 2, term_cols / 7, 0, 1);
                right_win = ncurses::newwin(term_rows - 2, term_cols / 7 * 3,
                                            term_cols / 7 * 4, 1);
                bottom_win = ncurses::newwin(1, term_cols, term_rows - 1, 0);
                ncurses::refresh();
            }
            ncurses::KEY_UP => {
                if index > 0 {
                    index = index - 1;
                    win_print_select_file(right_win, &dir_contents[index], (term_rows - 1) as usize);
                }
            }
            ncurses::KEY_DOWN => {
                if index + 1 < dir_contents.len() {
                    index = index + 1;
                    win_print_select_file(right_win, &dir_contents[index], (term_rows - 1) as usize);
                }
            }
            ncurses::KEY_LEFT => {
                if let Ok(mut pathbuf) = env::current_dir() {
                    if pathbuf.eq(&path::Path::new("/")) {
                        ncurses::wclear(left_win);
                        ncurses::wrefresh(left_win);
                        continue;
                    }
                    if pathbuf.pop() == false {
                        continue;
                    }
                    match env::set_current_dir(&pathbuf) {
                        Ok(_) => {
                            if let Ok(tmp_contents) = cwd_contents() {
                                dir_contents = tmp_contents;
                                dir_contents.sort_by(sort_func);

                                index = pindex;
                                pindex = 0;

                                if pathbuf.eq(&path::Path::new("/")) {
                                    ncurses::wclear(left_win);
                                    ncurses::wrefresh(left_win);
                                } else {
                                    win_print_curr_path(top_win);
                                    win_print_parent_dir(left_win, pindex,
                                        (term_rows - 1) as usize);
                                }
                                win_print_select_file(right_win,
                                    &dir_contents[index],
                                    (term_rows - 1) as usize);
                            }
                        },
                        Err(e) => {
                            win_print_err_msg(bottom_win, format!("{}", e).as_str());
                        },
                    };
                }
            }
            ncurses::KEY_RIGHT => {
                match dir_contents[index as usize].file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() || file_type.is_symlink() {
                            let new_path : path::PathBuf = dir_contents[index as usize].path();
                            match env::set_current_dir(new_path) {
                                Ok(_s) => {
                                    match cwd_contents() {
                                        Ok(s) => {
                                            dir_contents = s;
                                            dir_contents.sort_by(sort_func);
                                        }
                                        Err(_e) => {
                                            process::exit(1);
                                        }
                                    }
                                    pindex = index;
                                    index = 0;

                                    win_print_curr_path(top_win);
                                    win_print_parent_dir(left_win, pindex,
                                        (term_rows - 1) as usize);
                                    if dir_contents.len() > 0 {
                                        win_print_select_file(right_win,
                                            &dir_contents[index],
                                            (term_rows - 1) as usize);
                                    }
                                },
                                Err(e) => {
                                    win_print_err_msg(bottom_win, format!("{}", e).as_str());
                                },
                            };
                        }
                    },
                    Err(_e) => {
                        ncurses::printw("None");
                    },
                };
            }
            ENTER => {
                match dir_contents[index as usize].file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() || file_type.is_symlink() {
                            let new_path : path::PathBuf = dir_contents[index as usize].path();
                            match env::set_current_dir(new_path) {
                                Ok(_s) => {
                                    match cwd_contents() {
                                        Ok(s) => {
                                            dir_contents = s;
                                            dir_contents.sort_by(sort_func);
                                        }
                                        Err(_e) => {
                                            process::exit(1);
                                        }
                                    }
                                    pindex = index;
                                    index = 0;

                                    win_print_curr_path(top_win);
                                    win_print_parent_dir(left_win, pindex,
                                        (term_rows - 1) as usize);
                                    if dir_contents.len() > 0 {
                                        win_print_select_file(right_win,
                                            &dir_contents[index],
                                            (term_rows - 1) as usize);
                                    }
                                },
                                Err(_e) => {
                                    ncurses::printw("None");
                                },
                            };
                        }
                    },
                    Err(_e) => {
                        ncurses::printw("None");
                    },
                };
            }
            _ => {
                    ncurses::wprintw(mid_win, format!("pressed: {}\n",
			            std::char::from_u32(ch as u32).expect("Invalid char")).as_ref());
            }
        };

        win_contents_refresh_indexed(mid_win, &dir_contents,
                                            (term_rows - 1) as usize, index);
        win_print_file_info(bottom_win, &dir_contents[index]);
    }
    ncurses::endwin();
}

pub fn init_ncurses()
{
    ncurses::initscr();
    ncurses::raw();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    ncurses::start_color();
    ncurses::use_default_colors();

    ncurses::init_pair(1, ncurses::COLOR_BLUE, -1);
    ncurses::init_pair(2, ncurses::COLOR_CYAN, -1);
    ncurses::init_pair(3, ncurses::COLOR_WHITE, ncurses::COLOR_RED);

    ncurses::init_pair(4, ncurses::COLOR_MAGENTA, -1);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn print_file(win : ncurses::WINDOW, file : &fs::DirEntry) {

    if let Ok(file_type) = file.file_type() {
        if file_type.is_dir() {
            ncurses::wattron(win, ncurses::COLOR_PAIR(1));
        }
        else if file_type.is_symlink() {
            ncurses::wattron(win, ncurses::COLOR_PAIR(2));
        }
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
    if let Ok(file_type) = file.file_type() {
        if file_type.is_dir() {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(1));
        }
        else if file_type.is_symlink() {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(2));
        }
    }

    ncurses::wprintw(win, "\n");
}

pub fn win_contents_refresh(win : ncurses::WINDOW,
                dir_contents: &Vec<fs::DirEntry>, win_rows : usize) {

    let vec_len = dir_contents.len();

    ncurses::wclear(win);
    ncurses::wmove(win, 0, 0);

    if vec_len == 0 {
        ncurses::wattron(win, ncurses::COLOR_PAIR(3));
        ncurses::wprintw(win, "empty");
        ncurses::wattroff(win, ncurses::COLOR_PAIR(3));
        ncurses::wrefresh(win);
        return;
    }

    let mut i : usize = 0;
    let win_rows : usize = win_rows + i;

    while i < vec_len && i < win_rows {
        print_file(win, &dir_contents[i]);
        i = i + 1;
    }
    ncurses::wrefresh(win);
}
pub fn win_contents_refresh_indexed_short(win : ncurses::WINDOW,
                    dir_contents: &Vec<fs::DirEntry>,
                    win_rows : usize, index : usize) {
    let vec_len = dir_contents.len();

    ncurses::wclear(win);
    ncurses::wmove(win, 0, 0);

    if vec_len == 0 {
        ncurses::wattron(win, ncurses::COLOR_PAIR(3));
        ncurses::wprintw(win, "empty");
        ncurses::wattroff(win, ncurses::COLOR_PAIR(3));
        ncurses::wrefresh(win);
        return;
    }

    let mut i : usize = 0;
    let win_rows : usize = win_rows + i;

    while i < vec_len && i < win_rows {
        if i == index {
            ncurses::wattron(win, ncurses::A_REVERSE());
            print_file(win, &dir_contents[i]);
            ncurses::wattroff(win, ncurses::A_REVERSE());
        } else {
            print_file(win, &dir_contents[i]);
        }
        i = i + 1;
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
        end = win_rows - 1;
    } else if index - offset + win_rows >= vec_len {
        start = vec_len - win_rows;
        end = vec_len - 1;
    } else {
        start = index - offset;
        end = start + win_rows - 1;
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

/*
pub fn win_contents_refresh_indexed(win : ncurses::WINDOW,
                    dir_contents: &Vec<fs::DirEntry>,
                    win_rows : usize, index : usize) {
    let offset = 5;
    let vec_len = dir_contents.len();

    ncurses::wclear(win);
    ncurses::wmove(win, 0, 0);
    if vec_len == 0 {
        ncurses::wattron(win, ncurses::COLOR_PAIR(3));
        ncurses::wprintw(win, "empty");
        ncurses::wattroff(win, ncurses::COLOR_PAIR(3));
        ncurses::wrefresh(win);
        return;
    }

    let mut i : usize = 0;
    if vec_len >= win_rows && index > offset {
        i = index - offset;
    }
    let win_rows : usize = win_rows + i;

    while i < vec_len && i < win_rows {
        if index == i {
            ncurses::wattron(win, ncurses::A_REVERSE());
            print_file(win, &dir_contents[i]);
            ncurses::wattroff(win, ncurses::A_REVERSE());
        } else {
            print_file(win, &dir_contents[i]);
        }
        i = i + 1;
    }
    ncurses::wrefresh(win);
}*/

pub fn cwd_contents() -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    let tmp_result : Result<Vec<fs::DirEntry>, _> = fs::read_dir(".").unwrap().collect();
    // let dir_contents : Vec<fs::DirEntry> = tmp_result.unwrap();
    tmp_result
}


pub fn win_print_curr_path(win : ncurses::WINDOW)
{
    ncurses::wclear(win);
    if let Ok(cwd_path) = std::env::current_dir() {
        if let Some(path_str) = cwd_path.to_str() {
            ncurses::mvwprintw(win, 0, 0, path_str);
        }
    }
    ncurses::wrefresh(win);
}

pub fn win_print_parent_dir(win : ncurses::WINDOW, index : usize, length : usize)
{
    let tmp_result : Result<Vec<fs::DirEntry>, _> = fs::read_dir("..").unwrap().collect();
    let mut tmp_pdir : Vec<fs::DirEntry> = tmp_result.unwrap();
    tmp_pdir.sort_by(joshuto_sort::alpha_sort);
    win_contents_refresh_indexed(win, &tmp_pdir, length, index);
}

pub fn win_print_select_file(win : ncurses::WINDOW, file : &fs::DirEntry, length : usize)
{
    ncurses::wclear(win);
    match file.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                let tmp_result : Result<Vec<fs::DirEntry>, _> = fs::read_dir(&file.path()).unwrap().collect();
                let mut tmp_cdir : Vec<fs::DirEntry> = tmp_result.unwrap();
                tmp_cdir.sort_by(joshuto_sort::alpha_sort);
                win_contents_refresh(win, &tmp_cdir, length);
            }
        }
        Err(_e) => {
            ncurses::wattron(win, ncurses::COLOR_PAIR(3));
            ncurses::wprintw(win, "Error");
            ncurses::wattroff(win, ncurses::COLOR_PAIR(3));
        }
    }
    ncurses::wrefresh(win);
}


pub fn win_print_file_info(win : ncurses::WINDOW, file : &fs::DirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    const FILE_UNITS : [&str ; 5] = ["B", "KB", "MB", "GB", "TB"];
    const CONV_RATE : u64 = 1024;

    ncurses::wclear(win);
    ncurses::wmove(win, 0, 0);
    match file.metadata() {
        Ok(metadata) => {
            let permissions : fs::Permissions = metadata.permissions();
            let mode = permissions.mode();
            ncurses::wprintw(win, format!("{:?}", mode).as_str());
            ncurses::wprintw(win, " ");
            ncurses::wprintw(win, joshuto_rwx::stringify_mode(mode).as_str());
            ncurses::wprintw(win, "  ");

            let mut file_size = metadata.len();
            let mut index = 0;
            while file_size > CONV_RATE {
                file_size = file_size / CONV_RATE;
                index = index + 1;
            }
            ncurses::wprintw(win, format!("{} {}", file_size, FILE_UNITS[index]).as_str());
        },
        Err(e) => {
            ncurses::wprintw(win, format!("{:?}", e).as_str());
        },
    };
    ncurses::wrefresh(win);
}

pub fn win_print_err_msg(win : ncurses::WINDOW, err_msg : &str)
{
    ncurses::wclear(win);
    ncurses::mvwprintw(win, 0, 0, err_msg);
    ncurses::wrefresh(win);
}
