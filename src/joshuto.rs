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

const QUIT      : i32 = 'q' as i32;
const ENTER     : i32 = '\n' as i32;

const DIR_COLOR     : i16 = 1;
const SOCK_COLOR    : i16 = 4;
const EXEC_COLOR    : i16 = 11;
const IMG_COLOR     : i16 = 12;
const VID_COLOR     : i16 = 13;
const ERR_COLOR     : i16 = 40;

pub struct JoshutoWindow {
    win : ncurses::WINDOW,
    rows : i32,
    cols : i32,
    coords : (i32, i32)
}

impl JoshutoWindow {
    pub fn new(rows : i32, cols : i32, coords : (i32, i32)) -> JoshutoWindow
    {
        let win = ncurses::newwin(rows, cols, coords.0, coords.1);

        ncurses::refresh();
        JoshutoWindow {
            win: win,
            rows: rows,
            cols: cols,
            coords: coords,
        }
    }

    pub fn redraw(&mut self, rows : i32, cols : i32, coords : (i32, i32))
    {
        ncurses::delwin(self.win);
        self.win = ncurses::newwin(rows, cols, coords.0, coords.1);
        self.rows = rows;
        self.cols = cols;
        self.coords = coords;
        ncurses::refresh();
    }

    pub fn display_contents(&self, dir_contents: &Vec<fs::DirEntry>,
            index : usize) {

        let vec_len = dir_contents.len();

        if vec_len == 0 {
            win_print_err_msg(self, "empty");
            return;
        }

        let offset : usize = 5;
        let start : usize;
        let end : usize;

        if self.rows as usize >= vec_len {
            start = 0;
            end = vec_len;
        } else if index <= offset {
            start = 0;
            end = self.rows as usize;
        } else if index - offset + self.rows as usize >= vec_len {
            start = vec_len - self.rows as usize;
            end = vec_len;
        } else {
            start = index - offset;
            end = start + self.rows as usize;
        }

        ncurses::wclear(self.win);
        ncurses::wmove(self.win, 0, 0);

        for i in start..end {
            if index == i {
                ncurses::wattron(self.win, ncurses::A_REVERSE());
                print_file(self, &dir_contents[i]);
                ncurses::wattroff(self.win, ncurses::A_REVERSE());
            } else {
                print_file(self, &dir_contents[i]);
            }
        }
        ncurses::wrefresh(self.win);
    }
}

pub fn win_print_err_msg(win : &JoshutoWindow, err_msg : &str)
{
    ncurses::wclear(win.win);
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::mvwprintw(win.win, 0, 0, err_msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::wrefresh(win.win);
}

pub struct JoshutoView {
    top_win : JoshutoWindow,
    left_win : JoshutoWindow,
    mid_win : JoshutoWindow,
    right_win : JoshutoWindow,
    bot_win : JoshutoWindow,
    win_ratio : (i32, i32, i32),
}

impl JoshutoView {
    pub fn new(win_ratio : (i32, i32, i32)) -> JoshutoView
    {
        let mut term_rows : i32 = 0;
        let mut term_cols : i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let term_divide : i32 = term_cols / 7;
        let top_win = JoshutoWindow::new(1, term_cols, (0, 0));
        ncurses::scrollok(top_win.win, true);

        let left_win = JoshutoWindow::new(term_rows - 2,
            term_divide * win_ratio.0, (1, 0));

        let mid_win = JoshutoWindow::new(term_rows - 2,
            term_divide * win_ratio.1, (1, term_divide * win_ratio.0));

        let right_win = JoshutoWindow::new(term_rows - 2,
            term_divide * 3, (1, term_divide * win_ratio.2));
        let bot_win = JoshutoWindow::new(1, term_cols, (term_rows - 1, 0));

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

    fn redraw_views(&mut self) {
        let mut term_rows : i32 = 0;
        let mut term_cols : i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let term_divide : i32 = term_cols / 7;

        self.top_win.redraw(1, term_cols, (0, 0));
        ncurses::scrollok(self.top_win.win, true);

        self.left_win.redraw(term_rows - 2,
            term_divide * self.win_ratio.0, (1, 0));

        self.mid_win.redraw(term_rows - 2,
            term_divide * self.win_ratio.1, (1, term_divide * self.win_ratio.0));

        self.right_win.redraw(term_rows - 2,
            term_divide * 3, (1, term_divide * self.win_ratio.2));
        self.bot_win.redraw(1, term_cols, (term_rows - 1, 0));
    }

}

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

fn file_attron(win : ncurses::WINDOW, mode : u32,
        file_extension : Option<&ffi::OsStr>)
{
    match mode & joshuto_unix::BITMASK {
        joshuto_unix::S_IFDIR => {
            ncurses::wattron(win, ncurses::A_BOLD());
            ncurses::wattron(win, ncurses::COLOR_PAIR(DIR_COLOR));
        },
        joshuto_unix::S_IFLNK | joshuto_unix::S_IFCHR | joshuto_unix::S_IFBLK
         => {
            ncurses::wattron(win, ncurses::A_BOLD());
            ncurses::wattron(win, ncurses::COLOR_PAIR(SOCK_COLOR));
        },
        joshuto_unix::S_IFSOCK | joshuto_unix::S_IFIFO => {
            ncurses::wattron(win, ncurses::A_BOLD());
            ncurses::wattron(win, ncurses::COLOR_PAIR(SOCK_COLOR));
        },
        joshuto_unix::S_IFREG => {
            if joshuto_unix::is_executable(mode) == true {
                ncurses::wattron(win, ncurses::A_BOLD());
                ncurses::wattron(win, ncurses::COLOR_PAIR(EXEC_COLOR));
            }
            else if let Some(extension) = file_extension {
                if let Some(ext) = extension.to_str() {
                    file_ext_attron(win, ext);
                }
            }
        },
        _ => {},
    };
}

fn file_attroff(win : ncurses::WINDOW, mode : u32,
        file_extension : Option<&ffi::OsStr>)
{
    match mode & joshuto_unix::BITMASK {
        joshuto_unix::S_IFDIR => {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(DIR_COLOR));
            ncurses::wattroff(win, ncurses::A_BOLD());
        },
        joshuto_unix::S_IFLNK | joshuto_unix::S_IFCHR |
        joshuto_unix::S_IFBLK => {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(SOCK_COLOR));
            ncurses::wattroff(win, ncurses::A_BOLD());
        },
        joshuto_unix::S_IFSOCK | joshuto_unix::S_IFIFO => {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(SOCK_COLOR));
            ncurses::wattroff(win, ncurses::A_BOLD());
        },
        joshuto_unix::S_IFREG => {
            if joshuto_unix::is_executable(mode) {
                ncurses::wattroff(win, ncurses::COLOR_PAIR(EXEC_COLOR));
                ncurses::wattroff(win, ncurses::A_BOLD());
            } else if let Some(extension) = file_extension {
                if let Some(ext) = extension.to_str() {
                    file_ext_attroff(win, ext);
                }
            }
        },
        _ => {},
    };
}

fn file_ext_attron(win : ncurses::WINDOW, ext : &str)
{
    match ext {
        "png" | "jpg" | "jpeg" | "gif" => {
            ncurses::wattron(win, ncurses::COLOR_PAIR(IMG_COLOR));
        },
        "mkv" | "mp4" | "mp3" | "flac" | "ogg" | "avi" | "wmv" | "wav" |
        "m4a" => {
            ncurses::wattron(win, ncurses::COLOR_PAIR(VID_COLOR));
        },
        _ => {},
    }
}

fn file_ext_attroff(win : ncurses::WINDOW, ext : &str)
{
    match ext {
        "png" | "jpg" | "jpeg" | "gif" => {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(IMG_COLOR));
        },
        "mkv" | "mp4" | "mp3" | "flac" | "ogg" | "avi" | "wmv" | "wav" |
        "m4a" => {
            ncurses::wattroff(win, ncurses::COLOR_PAIR(IMG_COLOR));
        },
        _ => {},
    }
}


pub fn read_dir_list(config : &JoshutoConfig, path : &str) -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    fn filter_func(result : Result<fs::DirEntry, std::io::Error>) -> Option<fs::DirEntry>
    {
        match result {
            Ok(direntry) => {
                match direntry.file_name().into_string() {
                    Ok(file_name) => {
                        if file_name.starts_with(".") {
                            None
                        } else {
                            Some(direntry)
                        }
                    },
                    Err(e) => {
                        None
                    },
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    fn list_dirent(path : &str) -> Result<Vec<fs::DirEntry>, std::io::Error>
    {
        match fs::read_dir(path) {
            Ok(results) => {
                let mut result_vec : Vec<fs::DirEntry> = results
                        .filter_map(filter_func)
                        .collect();
                Ok(result_vec)
            },
            Err(e) => {
                Err(e)
            },
        }
    }

    fn list_dirent_hidden(path : &str) -> Result<Vec<fs::DirEntry>, std::io::Error>
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


    match config.show_hidden {
        Some(show) => {
            if show {
                list_dirent_hidden(path)
            } else {
                list_dirent(path)
            }
        },
        None => {
            list_dirent(path)
        },
    }

}

pub fn curr_dirent_list() -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    match fs::read_dir(".") {
        Ok(results) => {
            let results : Result<Vec<fs::DirEntry>, _> = results.collect();
            results
        },
        Err(e) => {
            Err(e)
        },
    }
}

pub fn win_print_path(win : &JoshutoWindow, path : &path::PathBuf)
{
    ncurses::wclear(win.win);
    let path_str : &str =
        match path.to_str() {
            Some(s) => s,
            None => "Error",
        };

    ncurses::wattron(win.win, ncurses::A_BOLD());
    ncurses::mvwprintw(win.win, 0, 0, path_str);
    ncurses::wattroff(win.win, ncurses::A_BOLD());
    ncurses::wrefresh(win.win);
}

fn print_file(win : &JoshutoWindow, file : &fs::DirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    let mut mode : u32 = 0;

    if let Ok(metadata) = file.metadata() {
        mode = metadata.permissions().mode();
    }
    if mode != 0 {
        file_attron(win.win, mode, file.path().extension());
    }

    match file.file_name().into_string() {
        Ok(file_name) => {
            ncurses::wprintw(win.win, " ");
            if file_name.len() + 1 >= win.cols as usize {
                let mut shortened = String::with_capacity(win.cols as usize);
                let mut iter = file_name.chars();
                for _i in 0..win.cols - 5 {
                    if let Some(ch) = iter.next() {
                        shortened.push(ch);
                    }
                }
                ncurses::wprintw(win.win, &shortened);
                ncurses::wprintw(win.win, "…");
//                ncurses::wprintw(win.win, "...");
            } else {
                ncurses::wprintw(win.win, &file_name);
            }
        },
        Err(e) => {
            ncurses::wprintw(win.win, format!("{:?}", e).as_str());
        },
    };

    if mode != 0 {
        file_attroff(win.win, mode, file.path().extension());
    }
    ncurses::wprintw(win.win, "\n");
}

pub fn win_contents_refresh(win : &JoshutoWindow,
                dir_contents: &Vec<fs::DirEntry>) {

    let vec_len = dir_contents.len();

    if vec_len == 0 {
        win_print_err_msg(win, "empty");
        return;
    }

    let mut i : usize = 0;
    let win_rows : usize = i + win.rows as usize;

    ncurses::wclear(win.win);
    ncurses::wmove(win.win, 0, 0);
    while i < vec_len && i < win_rows {
        print_file(win, &dir_contents[i]);
        i += 1;
    }
    ncurses::wrefresh(win.win);
}

pub fn win_print_parent_dir(win : &JoshutoWindow, path : &path::PathBuf, index : usize)
{
    ncurses::wclear(win.win);
    if let Some(ppath) = path.parent() {
        match fs::read_dir(ppath) {
            Ok(results) => {
                let results : Result<Vec<fs::DirEntry>, _> = results.collect();
                if let Ok(mut dir_contents) = results {
                    dir_contents.sort_by(joshuto_sort::alpha_sort);
                    win.display_contents(&dir_contents, index);
                }
            },
            Err(e) => {
                win_print_err_msg(win, format!("{}", e).as_str());
            },
        };
    }
    ncurses::wrefresh(win.win);
}

pub fn win_print_file_preview(win : &JoshutoWindow, file : &fs::DirEntry)
{
    use std::os::unix::fs::PermissionsExt;
    use joshuto::joshuto_unix;

    ncurses::wclear(win.win);
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
                            win_contents_refresh(&win, &dir_contents);
                        }
                    },
                    Err(e) => {
                        win_print_err_msg(&win, format!("{}", e).as_str());
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
                                        win_contents_refresh(&win, &dir_contents);
                                    }
                                },
                                Err(e) => {
                                    win_print_err_msg(&win, format!("{}", e).as_str());
                                },
                            };
                        } else {
                            ncurses::wprintw(win.win, "Symlink pointing to a file");
                        }
                    },
                    Err(e) => {
                        win_print_err_msg(&win, format!("{}", e).as_str());
                    },
                };
            },
            joshuto_unix::S_IFBLK => {
                ncurses::wprintw(win.win, "Block file");
            },
            joshuto_unix::S_IFSOCK => {
                ncurses::wprintw(win.win, "Socket file");
            },
            joshuto_unix::S_IFCHR => {
                ncurses::wprintw(win.win, "Character file");
            },
            joshuto_unix::S_IFIFO => {
                ncurses::wprintw(win.win, "FIFO file");
            },
            joshuto_unix::S_IFREG => {
                ncurses::wprintw(win.win, "Plain file");
            },
            _ => {
                ncurses::wprintw(win.win, "Unknown file");
            },
        }
    }
    ncurses::wrefresh(win.win);
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

pub fn enter_dir(config : &JoshutoConfig, direntry : &fs::DirEntry, joshuto_view : &JoshutoView,
        curr_path : &mut path::PathBuf, index : &mut usize) -> Option<Vec<fs::DirEntry>>
{
    let tmp_name : ffi::OsString = direntry.file_name();
    let tmp_name2 = tmp_name.as_os_str().to_str().unwrap();
    let file_name = path::Path::new(tmp_name2);
    curr_path.push(file_name);

    let dir_contents : Vec<fs::DirEntry>;

    match env::set_current_dir(&curr_path) {
        Ok(_s) => {
            match read_dir_list(config, ".") {
                Ok(s) => {
                    dir_contents = s;
                }
                Err(e) => {
                    win_print_err_msg(&joshuto_view.bot_win,
                        format!("{}", e).as_str());
                    return None;
                }
            }
            *index = 0;

            win_print_path(&joshuto_view.top_win, &curr_path);
            win_print_parent_dir(&joshuto_view.left_win,
                    &curr_path, *index);

            if dir_contents.len() > 0 {
                win_print_file_preview(&joshuto_view.right_win, direntry);
            }
        },
        Err(e) => {
            win_print_err_msg(&joshuto_view.bot_win,
                format!("{}", e).as_str());
            return None;
        }
    }

    return Some(dir_contents);
}


pub fn run(config : &JoshutoConfig)
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
    let mut joshuto_view : JoshutoView = JoshutoView::new((1, 3, 4));

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
        match read_dir_list(config, ".") {
            Ok(s) => {
                s
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
    dir_contents.sort_by(&sort_func);

    win_print_path(&joshuto_view.top_win, &curr_path);

    win_print_parent_dir(&joshuto_view.left_win, &curr_path, pindex);

    joshuto_view.mid_win.display_contents(&dir_contents, index);

    if dir_contents.len() > 0 {
        win_print_file_preview(&joshuto_view.right_win, &dir_contents[index]);
        win_print_file_info(joshuto_view.bot_win.win, &dir_contents[index]);
    }

    ncurses::refresh();

    loop {
        let ch = ncurses::getch();

        match ch {
            QUIT => {
                break;
            },
            ncurses::KEY_RESIZE => {
                ncurses::clear();
                joshuto_view.redraw_views();
                ncurses::refresh();

                win_print_path(&joshuto_view.top_win, &curr_path);
                win_print_parent_dir(&joshuto_view.left_win, &curr_path, pindex);
                joshuto_view.mid_win.display_contents(&dir_contents, index);
                if dir_contents.len() > 0 {
                    win_print_file_preview(&joshuto_view.right_win,
                            &dir_contents[index]);
                    win_print_file_info(joshuto_view.bot_win.win,
                            &dir_contents[index]);
                }

                ncurses::refresh();

            },
            ncurses::KEY_HOME => {
                if index != 0 {
                    index = 0;
                    win_print_file_preview(&joshuto_view.right_win,
                            &dir_contents[index]);
                }
            },
            ncurses::KEY_END => {
                let tmp_len = dir_contents.len();
                if index + 1 != tmp_len {
                    index = tmp_len - 1;
                    win_print_file_preview(&joshuto_view.right_win,
                            &dir_contents[index]);
                }
            },
            ncurses::KEY_UP => {
                if index > 0 {
                    index = index - 1;
                    win_print_file_preview(&joshuto_view.right_win,
                            &dir_contents[index]);
                }
            },
            ncurses::KEY_DOWN => {
                if index + 1 < dir_contents.len() {
                    index = index + 1;
                    win_print_file_preview(&joshuto_view.right_win,
                            &dir_contents[index]);
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
                win_print_file_preview(&joshuto_view.right_win,
                        &dir_contents[index]);
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
                win_print_file_preview(&joshuto_view.right_win,
                        &dir_contents[index]);
            },
            ncurses::KEY_LEFT => {
                if curr_path.parent() == None {
                        ncurses::wclear(joshuto_view.left_win.win);
                        ncurses::wrefresh(joshuto_view.left_win.win);
                        continue;
                }
                if curr_path.pop() == false {
                        continue;
                }
                env::set_current_dir(curr_path.as_path());
                match read_dir_list(config, ".") {
                    Ok(s) => {
                        dir_contents = s;
                        dir_contents.sort_by(&sort_func);

                        index = pindex;

                        win_print_parent_dir(&joshuto_view.left_win,
                                &curr_path, pindex);

                        win_print_path(&joshuto_view.top_win, &curr_path);
                        win_print_file_preview(&joshuto_view.right_win,
                                &dir_contents[index]);
                    },
                    Err(e) => {
                        win_print_err_msg(&joshuto_view.bot_win,
                                format!("{}", e).as_str());
                    },
                };
            },
            ncurses::KEY_RIGHT | ENTER => {
                if let Ok(file_type) = &dir_contents[index as usize].file_type() {
                    if file_type.is_dir() {
                        if let Some(s) = enter_dir(config, &dir_contents[index as usize],
                                &joshuto_view, &mut curr_path, &mut index) {
                            dir_contents = s;
                            dir_contents.sort_by(&sort_func);
                        }
                    } else if file_type.is_symlink() {
                        let mut file_path : path::PathBuf =
                                dir_contents[index as usize].path();
                        match fs::read_link(&file_path) {
                            Ok(sym_path) => {
                                file_path.pop();
                                file_path.push(sym_path.as_path());
                                if file_path.as_path().is_dir() {
                                    if let Some(s) = enter_dir(config, &dir_contents[index as usize],
                                            &joshuto_view, &mut curr_path, &mut index) {
                                        dir_contents = s;
                                        dir_contents.sort_by(&sort_func);
                                    }
                                }
                            },
                            Err(e) => {
                                win_print_err_msg(&joshuto_view.bot_win,
                                    format!("{}", e).as_str());
                            },
                        };
                    } else {
                        win_print_err_msg(&joshuto_view.right_win, "Nice");
                    }
                }
            },
            _ => {
                eprintln!("Unknown keychar: ({}: {})", ch as u32, ch);
            },
        };

        joshuto_view.mid_win.display_contents(&dir_contents, index);
        win_print_file_info(joshuto_view.bot_win.win, &dir_contents[index]);
    }
    ncurses::endwin();
}
