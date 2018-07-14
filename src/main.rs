extern crate ncurses;

mod joshuto;

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


fn main()
{
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    joshuto::init_ncurses();

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

    let mut tmp_result : Result<Vec<fs::DirEntry>, _> = fs::read_dir(".").unwrap().collect();
    let mut dir_contents : Vec<fs::DirEntry> = tmp_result.unwrap();
    dir_contents.sort_by(joshuto::alpha_sort);

    joshuto::win_print_curr_dir(top_win);
    joshuto::win_print_parent_dir(left_win, pindex, (term_rows - 1) as usize);

    joshuto::win_print_dir(mid_win, &dir_contents, index, (term_rows - 1) as usize);

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
                    joshuto::win_print_select_file(right_win, &dir_contents[index], (term_rows - 1) as usize);
                }
            }
            ncurses::KEY_DOWN => {
                if index + 1 < dir_contents.len() {
                    index = index + 1;
                    joshuto::win_print_select_file(right_win, &dir_contents[index], (term_rows - 1) as usize);
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
                                tmp_result = fs::read_dir(".").unwrap().collect();
                                dir_contents = tmp_result.unwrap();
                                dir_contents.sort_by(joshuto::alpha_sort);
                                index = pindex;
                                pindex = 0;
                                if pathbuf.eq(&path::Path::new("/")) {
                                    ncurses::wclear(left_win);
                                    ncurses::wrefresh(left_win);
                                } else {
                                    joshuto::win_print_curr_dir(top_win);
                                    joshuto::win_print_parent_dir(left_win, pindex, (term_rows - 1) as usize);
                                }
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
                                    tmp_result = fs::read_dir(".").unwrap().collect();
                                    dir_contents = tmp_result.unwrap();
                                    dir_contents.sort_by(joshuto::alpha_sort);
                                    pindex = index;
                                    index = 0;

                                    joshuto::win_print_curr_dir(top_win);
                                    joshuto::win_print_parent_dir(left_win, pindex, (term_rows - 1) as usize);
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
                                    tmp_result = fs::read_dir(".").unwrap().collect();
                                    dir_contents = tmp_result.unwrap();
                                    dir_contents.sort_by(joshuto::alpha_sort);
                                    pindex = index;
                                    index = 0;

                                    joshuto::win_print_curr_dir(top_win);
                                    joshuto::win_print_parent_dir(left_win, pindex, (term_rows - 1) as usize);
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

        joshuto::win_print_dir(mid_win, &dir_contents, index, (term_rows - 1) as usize);
    }
    ncurses::endwin();
}
