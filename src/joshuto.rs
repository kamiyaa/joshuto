extern crate ncurses;

use std;
use std::env;
use std::fs;
use std::path;
use std::process;

const QUIT: i32 = 'q' as i32;
const ENTER: i32 = '\n' as i32;

pub mod joshuto_sort {

    use std::cmp;
    use std::fs;
    use std;

    pub fn alpha_sort(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> cmp::Ordering
    {
        fn res_ordering(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> Result<cmp::Ordering, std::io::Error> {
            let f1_type = file1.metadata()?;
            let f2_type = file2.metadata()?;

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

pub fn run()
{
    init_ncurses();

    let mut term_rows : i32 = 0;
    let mut term_cols : i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

    let mut index : usize = 0;
    let mut pindex : usize = 0;
    let mut cindex : usize = 0;

    ncurses::refresh();

    let mut top_win = ncurses::newwin(1, term_cols, 0, 0);
    let mut mid_win = ncurses::newwin(term_rows - 2, term_cols / 7 * 3,
                                        1, term_cols / 7);
    let mut left_win = ncurses::newwin(term_rows - 2, term_cols / 7, 1, 0);
    let mut right_win = ncurses::newwin(term_rows - 2, term_cols / 7 * 3,
                                        1, term_cols / 7 * 4);

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
    win_print_parent_dir(left_win, pindex, (term_rows - 1) as usize);

    win_contents_refresh_indexed(mid_win, &dir_contents, (term_rows - 1) as usize, index);

    ncurses::refresh();

    loop {
        let ch = ncurses::getch();

        match ch {
            QUIT => {
                break;
            }
            ncurses::KEY_RESIZE => {
                ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);
                top_win = ncurses::newwin(1, term_cols, 0, 0);
                mid_win = ncurses::newwin(term_rows - 2, term_cols / 7 * 3,
                                          term_cols / 7, 1);
                left_win = ncurses::newwin(term_rows - 2, term_cols / 7, 0, 1);
                right_win = ncurses::newwin(term_rows - 2, term_cols / 7 * 3,
                                            term_cols / 7 * 4, 1);
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
                match env::current_dir() {
                    Ok(mut pathbuf) => {
                        if pathbuf.eq(&path::Path::new("/")) {
                            ncurses::wclear(left_win);
                            ncurses::wrefresh(left_win);
                            continue;
                        }
                        if pathbuf.pop() == false {
                            continue;
                        }
                        match env::set_current_dir(&pathbuf) {
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
                                index = pindex;
                                pindex = 0;

                                if pathbuf.eq(&path::Path::new("/")) {
                                    ncurses::wclear(left_win);
                                    ncurses::wrefresh(left_win);
                                } else {
                                    win_print_curr_path(top_win);
                                    win_print_parent_dir(left_win, pindex, (term_rows - 1) as usize);
                                }
                            },
                            Err(_e) => {
                                ncurses::printw("None");
                            },
                        };
                    }
                    Err(_e) => {
                        ncurses::printw("None");
                    }
                };
            }
            ncurses::KEY_RIGHT => {
                match dir_contents[index as usize].file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() {
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
                                    win_print_parent_dir(left_win, pindex, (term_rows - 1) as usize);
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
            ENTER => {
                match dir_contents[index as usize].file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() {
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
                                    win_print_parent_dir(left_win, pindex, (term_rows - 1) as usize);
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
    //    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn init_window(win_rows : i32, win_cols : i32, x : i32, y : i32) -> ncurses::WINDOW
{
    ncurses::newwin(win_rows, win_cols, y, x)
}

pub fn win_contents_refresh(win : ncurses::WINDOW,
                dir_contents: &Vec<fs::DirEntry>, win_rows : usize) {
    let vec_len = dir_contents.len();

    ncurses::wclear(win);
    ncurses::mvwprintw(win, 0, 0, "");
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
        match dir_contents[i].metadata() {
            Ok(metadata) => {
                if metadata.is_dir() {
                    ncurses::wattron(win, ncurses::COLOR_PAIR(1));
                }
/*                else if metadata.is_symlink() {
                    ncurses::wattron(win, ncurses::COLOR_PAIR(2));
                }*/

                match dir_contents[i].file_name().into_string() {
                    Ok(file_name) => {
                        ncurses::wprintw(win, " ");
                        ncurses::wprintw(win, &file_name);
                    },
                    Err(_e) => {
                        ncurses::wprintw(win, "file_name Error");
                    },
                };

                if metadata.is_dir() {
                    ncurses::wattroff(win, ncurses::COLOR_PAIR(1));
                }
/*                else if metadata.is_symlink() {
                    ncurses::wattroff(win, ncurses::COLOR_PAIR(2));
                }*/
            },
            Err(_e) => {
                ncurses::wprintw(win, "metadata Error");
            }
        }

        ncurses::wprintw(win, "\n");
        i = i + 1;
    }
    ncurses::wrefresh(win);
}

pub fn win_contents_refresh_indexed(win : ncurses::WINDOW,
                    dir_contents: &Vec<fs::DirEntry>,
                    win_rows : usize, index : usize) {
    let offset = 5;
    let vec_len = dir_contents.len();

    ncurses::wclear(win);
    ncurses::mvwprintw(win, 0, 0, "");
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
        match dir_contents[i].metadata() {
            Ok(metadata) => {
                if i == index {
                    ncurses::wattron(win, ncurses::A_REVERSE());
                }
                if metadata.is_dir() {
                    ncurses::wattron(win, ncurses::COLOR_PAIR(1));
                }
/*                else if metadata.is_symlink() {
                    ncurses::wattron(win, ncurses::COLOR_PAIR(2));
                }*/

                match dir_contents[i].file_name().into_string() {
                    Ok(file_name) => {
                        ncurses::wprintw(win, " ");
                        ncurses::wprintw(win, file_name.as_str());
                    },
                    Err(_e) => {
                        ncurses::wprintw(win, "file_name Error");
                    },
                };


                if i == index {
                    ncurses::wattroff(win, ncurses::A_REVERSE());
                }

                if metadata.is_dir() {
                    ncurses::wattroff(win, ncurses::COLOR_PAIR(1));
                }
/*                else if metadata.is_symlink() {
                    ncurses::wattron(win, ncurses::COLOR_PAIR(2));
                }*/

            },
            Err(_e) => {
                ncurses::wprintw(win, "metadata Error");
            }
        }

        ncurses::wprintw(win, "\n");
        i = i + 1;
    }
    ncurses::wrefresh(win);
}

pub fn cwd_contents() -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    let tmp_result : Result<Vec<fs::DirEntry>, _> = fs::read_dir(".").unwrap().collect();
    // let dir_contents : Vec<fs::DirEntry> = tmp_result.unwrap();
    tmp_result
}


pub fn win_print_curr_path(win : ncurses::WINDOW)
{
    use std::env;

    let cwd_path = env::current_dir().unwrap();
    let path_str = cwd_path.to_str().unwrap();
    ncurses::wclear(win);
    ncurses::mvwprintw(win, 0, 0, path_str);
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
