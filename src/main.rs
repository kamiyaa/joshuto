extern crate ncurses;

use std::fs;
use std::path;
use std::env;

const QUIT: i32 = 'q' as i32;
const ENTER: i32 = '\n' as i32;

/*
pub struct joshuto_win {
    win : ncurses::WINDOW,
    offset : usize,
    row_size : i32,
    col_size : i32,
}
*/

mod joshuto {
    extern crate ncurses;
    use std::fs;

    pub fn init_ncurses()
    {
        ncurses::initscr();
        ncurses::raw();

        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::noecho();
        ncurses::start_color();


        ncurses::init_pair(1, ncurses::COLOR_BLUE, ncurses::COLOR_BLACK);
        ncurses::init_pair(2, ncurses::COLOR_CYAN, ncurses::COLOR_BLACK);
    //    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }

    pub fn init_window(win_rows : i32, win_cols : i32, x : i32, y : i32) -> ncurses::WINDOW
    {
        let win = ncurses::newwin(win_rows, win_cols, y, x);

        ncurses::mv(y, x);
        ncurses::wrefresh(win);
        win
    }

    pub fn win_print_dir(win : ncurses::WINDOW,
                        dir_contents: &Vec<fs::DirEntry>,
                        index : usize, win_rows : usize) {
        let offset = 5;
        let vec_len = dir_contents.len();

        let mut i : usize = 0;
        if vec_len >= win_rows && index > offset {
            i = index - offset;
        }
        let win_rows : usize = win_rows + i;

        ncurses::wclear(win);
        ncurses::mvwprintw(win, 0, 0, "");

        while i < vec_len && i < win_rows {
            match dir_contents[i].file_type() {
                Ok(file_type) => {
                    if i == index {
                        ncurses::wattron(win, ncurses::A_REVERSE());
                    }
                    if file_type.is_dir() {
                        ncurses::wattron(win, ncurses::COLOR_PAIR(1));
                    } else if file_type.is_symlink() {
                        ncurses::wattron(win, ncurses::COLOR_PAIR(2));
                    }

                    match dir_contents[i].file_name().into_string() {
                        Ok(file_name) => {
                            ncurses::wprintw(win, " ");
                            ncurses::wprintw(win, file_name.as_str());
                        },
                        Err(_e) => {
                            ncurses::wprintw(win, "Error");
                        },
                    };


                    if i == index {
                        ncurses::wattroff(win, ncurses::A_REVERSE());
                    }

                    if file_type.is_dir() {
                        ncurses::wattroff(win, ncurses::COLOR_PAIR(1));
                    } else if file_type.is_symlink() {
                        ncurses::wattroff(win, ncurses::COLOR_PAIR(2));
                    }
                },
                Err(_e) => {
                    ncurses::wprintw(win, "Error");
                }
            }

            ncurses::wprintw(win, "\n");
            i = i + 1;
        }
        ncurses::wrefresh(win);
    }

    pub fn print_dir(dir_contents: &Vec<fs::DirEntry>, index : usize, row_size : usize) {
        let offset = 5;
        let vec_len = dir_contents.len();

        let mut i : usize = 0;
        if vec_len >= row_size && index > offset {
            i = index - offset;
        }
        let row_size : usize = row_size + i;

        ncurses::clear();

        while i < vec_len && i < row_size {
            if i == index {
                ncurses::attron(ncurses::A_REVERSE());
            }
            match dir_contents[i].file_name().into_string() {
                Ok(file_name) => {
                    ncurses::printw(" ");
                    ncurses::printw(file_name.as_str());
                },
                Err(_e) => {
                    ncurses::printw("Error");
                },
            };

            if i == index {
                ncurses::attroff(ncurses::A_REVERSE());
            }
            ncurses::printw("\n");
            i = i + 1;
        }
        ncurses::refresh();
    }
}

fn direntry_sort_func(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> std::cmp::Ordering
{
    fn res_ordering(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> Result<std::cmp::Ordering, std::io::Error> {
        let f1_type = file1.file_type()?;
        let f2_type = file2.file_type()?;

        if !f1_type.is_file() && f2_type.is_file() {
            Ok(std::cmp::Ordering::Less)
        } else if !f2_type.is_file() && f1_type.is_file() {
            Ok(std::cmp::Ordering::Greater)
        } else {
            let f1_name : std::string::String =
                file1.file_name().as_os_str().to_str().unwrap().to_lowercase();
            let f2_name : std::string::String =
                file2.file_name().as_os_str().to_str().unwrap().to_lowercase();
            if f1_name <= f2_name {
                Ok(std::cmp::Ordering::Less)
            } else {
                Ok(std::cmp::Ordering::Greater)
            }
        }
    }
    res_ordering(file1, file2).unwrap_or(std::cmp::Ordering::Less)
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let mut result_tmp_dir : Result<Vec<fs::DirEntry>, _> = fs::read_dir(".").unwrap().collect();
    let mut dir_contents : Vec<fs::DirEntry> = result_tmp_dir.unwrap();
    dir_contents.sort_by(direntry_sort_func);


    joshuto::init_ncurses();

    let mut term_rows : i32 = 0;
    let mut term_cols : i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

    let mut index : usize = 0;

    let top_win = joshuto::init_window(1, term_cols, 0, 0);
    let mid_win = joshuto::init_window(term_rows - 2, term_cols / 7 * 3,
                                        term_cols / 7, 1);
    let left_win = joshuto::init_window(term_rows - 2, term_cols / 7,
                                        0, 1);
    let right_win = joshuto::init_window(term_rows - 2, term_cols / 7 * 3,
                                        term_cols / 7 * 4, 1);

    let cwd_path : path::PathBuf = env::current_dir().unwrap();
    let path_str = cwd_path.to_str().unwrap();
    ncurses::mvwprintw(top_win, 0, 0, path_str);
    ncurses::wrefresh(top_win);

    joshuto::win_print_dir(mid_win, &dir_contents, index, (term_rows - 1) as usize);
    joshuto::win_print_dir(left_win, &dir_contents, index, (term_rows - 1) as usize);
    joshuto::win_print_dir(right_win, &dir_contents, index, (term_rows - 1) as usize);

    loop {
        let ch = ncurses::getch();

        match ch {
            QUIT => {
                break;
            }
            ncurses::KEY_UP => {
                if index > 0 {
                    index = index - 1;
                }
            }
            ncurses::KEY_DOWN => {
                if index + 1 < dir_contents.len() {
                    index = index + 1;
                }
            }
            ncurses::KEY_LEFT => {
                match env::current_dir() {
                    Ok(mut pathbuf) => {
                        if pathbuf.eq(&path::Path::new("/")) {
                            continue;
                        }
                        if pathbuf.pop() == false {
                            continue;
                        }
                        match env::set_current_dir(pathbuf) {
                            Ok(_s) => {
                                result_tmp_dir = fs::read_dir(".").unwrap().collect();
                                dir_contents = result_tmp_dir.unwrap();
                                dir_contents.sort_by(direntry_sort_func);
                                index = 0;
                            },
                            Err(_e) => {
                                ncurses::printw("None");
                            },
                        };
                    }
                    Err(e) => {
                        println!("{}", e);
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
                                    result_tmp_dir = fs::read_dir(".").unwrap().collect();
                                    dir_contents = result_tmp_dir.unwrap();
                                    dir_contents.sort_by(direntry_sort_func);
                                    index = 0;
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
                                    result_tmp_dir = fs::read_dir(".").unwrap().collect();
                                    dir_contents = result_tmp_dir.unwrap();
                                    dir_contents.sort_by(direntry_sort_func);
                                    index = 0;
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
        let cwd_path = env::current_dir().unwrap();
        let path_str = cwd_path.to_str().unwrap();
        ncurses::wclear(top_win);
        ncurses::mvwprintw(top_win, 0, 0, path_str);
        ncurses::wrefresh(top_win);

        joshuto::win_print_dir(mid_win, &dir_contents, index, (term_rows - 1) as usize);
        joshuto::win_print_dir(left_win, &dir_contents, index, (term_rows - 1) as usize);
        joshuto::win_print_dir(right_win, &dir_contents, index, (term_rows - 1) as usize);
    }
    ncurses::endwin();
}
