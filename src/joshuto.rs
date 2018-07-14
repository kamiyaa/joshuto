extern crate ncurses;

use std::fs;
use std::cmp;
use std;

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

pub fn win_print_dir_basic(win : ncurses::WINDOW,
                dir_contents: &Vec<fs::DirEntry>, win_rows : usize) {
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
                        ncurses::wprintw(win, file_name.as_str());
                    },
                    Err(_e) => {
                        ncurses::wprintw(win, "Error");
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
                ncurses::wprintw(win, "Error");
            }
        }

        ncurses::wprintw(win, "\n");
        i = i + 1;
    }
    ncurses::wrefresh(win);
}

pub fn win_print_dir(win : ncurses::WINDOW,
                dir_contents: &Vec<fs::DirEntry>,
                    index : usize, win_rows : usize) {
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
                        ncurses::wprintw(win, "Error");
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
                ncurses::wprintw(win, "Error");
            }
        }

        ncurses::wprintw(win, "\n");
        i = i + 1;
    }
    ncurses::wrefresh(win);
}


pub fn win_print_curr_dir(win : ncurses::WINDOW)
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
    tmp_pdir.sort_by(alpha_sort);
    win_print_dir(win, &tmp_pdir, index, length);
}

pub fn win_print_select_file(win : ncurses::WINDOW, file : &fs::DirEntry, length : usize)
{
    ncurses::wclear(win);
    match file.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                let tmp_result : Result<Vec<fs::DirEntry>, _> = fs::read_dir(&file.path()).unwrap().collect();
                let mut tmp_cdir : Vec<fs::DirEntry> = tmp_result.unwrap();
                tmp_cdir.sort_by(alpha_sort);
                win_print_dir_basic(win, &tmp_cdir, length);
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
