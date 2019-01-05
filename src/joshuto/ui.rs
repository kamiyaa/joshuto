extern crate ncurses;
extern crate wcwidth;

use std::ffi;
use std::fs;
use std::path;

use joshuto::structs;
use joshuto::unix;
use joshuto::window;
use joshuto::keymapll::Keycode;

pub const DIR_COLOR     : i16 = 1;
pub const SOCK_COLOR    : i16 = 4;
pub const EXEC_COLOR    : i16 = 11;
pub const IMG_COLOR     : i16 = 12;
pub const VID_COLOR     : i16 = 13;
pub const SELECT_COLOR  : i16 = 25;
pub const ERR_COLOR     : i16 = 40;
pub const EMPTY_COLOR   : i16 = 50;

pub fn init_ncurses()
{
    let locale_conf = ncurses::LcCategory::all;

    ncurses::setlocale(locale_conf, "");
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    ncurses::initscr();
    ncurses::cbreak();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::start_color();
    ncurses::use_default_colors();
    ncurses::noecho();
    ncurses::set_escdelay(0);

    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

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
    /* selected files */
    ncurses::init_pair(SELECT_COLOR, ncurses::COLOR_YELLOW, -1);
    /* error message */
    ncurses::init_pair(ERR_COLOR, ncurses::COLOR_RED, -1);
    /* empty */
    ncurses::init_pair(EMPTY_COLOR, ncurses::COLOR_WHITE, ncurses::COLOR_RED);

    ncurses::printw("Loading...");

    ncurses::refresh();
}

pub fn end_ncurses()
{
        ncurses::endwin();
}

pub fn wprint_msg(win : &window::JoshutoPanel, msg : &str)
{
    ncurses::werase(win.win);
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_err(win: &window::JoshutoPanel, msg : &str)
{
    ncurses::werase(win.win);
    ncurses::wattron(win.win, ncurses::A_BOLD());
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(ERR_COLOR));
    ncurses::wattroff(win.win, ncurses::A_BOLD());
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_empty(win: &window::JoshutoPanel, msg : &str)
{
    ncurses::werase(win.win);
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(EMPTY_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(EMPTY_COLOR));
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_path(win: &window::JoshutoPanel, username: &str,
        hostname: &str, path: &path::PathBuf, file_name: &str)
{
    ncurses::werase(win.win);
    let path_str: &str = match path.to_str() {
            Some(s) => s,
            None => "Error",
        };
    ncurses::wattron(win.win, ncurses::A_BOLD());
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(EXEC_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0,
            format!("{}@{} ", username, hostname).as_str());
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(EXEC_COLOR));

    ncurses::wattron(win.win, ncurses::COLOR_PAIR(DIR_COLOR));
    ncurses::waddstr(win.win, path_str);
    ncurses::waddstr(win.win, "/");
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(DIR_COLOR));
    ncurses::waddstr(win.win, file_name);
    ncurses::wattroff(win.win, ncurses::A_BOLD());
    ncurses::wnoutrefresh(win.win);
}

/*

fn wprint_file_size(win: &window::JoshutoPanel, file: &fs::DirEntry,
    coord: (i32, i32)) -> usize
{
    const FILE_UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "E"];
    const CONV_RATE: f64 = 1024.0;

    match file.metadata() {
        Ok(metadata) => {
            let mut file_size = metadata.len() as f64;
            let mut index = 0;
            while file_size > CONV_RATE {
                file_size = file_size / CONV_RATE;
                index += 1;
            }

            ncurses::wmove(win.win, coord.0, win.cols - 6);
            if file_size >= 1000.0 {
                ncurses::waddstr(win.win,
                    format!("{:.0} {}", file_size, FILE_UNITS[index]).as_str());
            } else if file_size >= 100.0 {
                ncurses::waddstr(win.win,
                    format!(" {:.0} {}", file_size, FILE_UNITS[index]).as_str());
            } else if file_size >= 10.0 {
                ncurses::waddstr(win.win,
                    format!("{:.1} {}", file_size, FILE_UNITS[index]).as_str());
            } else {
                ncurses::waddstr(win.win,
                    format!("{:.2} {}", file_size, FILE_UNITS[index]).as_str());
            }
        },
        Err(e) => {
            ncurses::waddstr(win.win, format!("{:?}", e).as_str());
        },
    };
    6
}
*/

fn wprint_file_name(win: &window::JoshutoPanel, file: &structs::JoshutoDirEntry,
        coord: (i32, i32))
{
    ncurses::mvwaddstr(win.win, coord.0, coord.1, " ");

    let file_name = &file.file_name_as_string;
    let name_visual_space = wcwidth::str_width(file_name).unwrap_or(win.cols as usize);
    if name_visual_space + 1 < win.cols as usize {
        ncurses::waddstr(win.win, &file_name);
        return;
    }

    let mut win_cols = win.cols;

    if let Some(ext) = file_name.rfind('.') {
        let extension: &str = &file_name[ext..];
        let ext_len = wcwidth::str_width(extension).unwrap_or(extension.len());
        win_cols = win_cols - ext_len as i32;
        ncurses::mvwaddstr(win.win, coord.0, win_cols, &extension);
    }
    win_cols = win_cols - 2;

    ncurses::wmove(win.win, coord.0, coord.1 + 1);

    let mut trim_index: usize = file_name.len();

    let mut total: usize = 0;
    for (index, ch) in file_name.char_indices() {
        if total >= win_cols as usize {
            trim_index = index;
            break;
        }
        total = total + wcwidth::char_width(ch).unwrap_or(2) as usize;
    }

    ncurses::waddstr(win.win, &file_name[..trim_index]);
    ncurses::waddstr(win.win, "…");
}

pub fn wprint_file_info(win: ncurses::WINDOW, file: &structs::JoshutoDirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    const FILE_UNITS: [&str ; 6] = ["B", "KB", "MB", "GB", "TB", "ExB"];
    const CONV_RATE: f64 = 1024.0;

    ncurses::werase(win);
    ncurses::wmove(win, 0, 0);
    match fs::symlink_metadata(&file.path) {
        Ok(metadata) => {
            let permissions: fs::Permissions = metadata.permissions();
            let mode = permissions.mode();

            let mut file_size = metadata.len() as f64;
            let mut index = 0;
            while file_size > CONV_RATE {
                file_size = file_size / CONV_RATE;
                index += 1;
            }

            ncurses::waddstr(win, unix::stringify_mode(mode).as_str());
            ncurses::waddstr(win, "  ");
            if file_size >= 1000.0 {
                ncurses::waddstr(win,
                    format!("{:.0}{}", file_size, FILE_UNITS[index]).as_str());
            } else if file_size >= 100.0 {
                ncurses::waddstr(win,
                    format!(" {:.0}{}", file_size, FILE_UNITS[index]).as_str());
            } else if file_size >= 10.0 {
                ncurses::waddstr(win,
                    format!("{:.1}{}", file_size, FILE_UNITS[index]).as_str());
            } else {
                ncurses::waddstr(win,
                    format!("{:.2}{}", file_size, FILE_UNITS[index]).as_str());
            }
            ncurses::waddstr(win, " ");
            if mode >> 9 & unix::S_IFLNK >> 9 == mode >> 9 {
                if let Ok(path) = fs::read_link(&file.path) {
                    ncurses::waddstr(win, " -> ");
                    ncurses::waddstr(win, path.to_str().unwrap());
                }
            }
        },
        Err(e) => {
            ncurses::waddstr(win, format!("{:?}", e).as_str());
        },
    };
    ncurses::wnoutrefresh(win);
}

pub fn wprint_direntry(win: &window::JoshutoPanel,
        file: &structs::JoshutoDirEntry, coord: (i32, i32))
{
//    let offset = wprint_file_size(win, file, coord);
//    let offset = 3;
    wprint_file_name(win, file, coord);
}

pub fn display_contents(win: &window::JoshutoPanel,
        entry: &structs::JoshutoDirList) {
    use std::os::unix::fs::PermissionsExt;

    ncurses::werase(win.win);
    if win.cols <= 6 {
        return;
    }

    let mut mode: u32 = 0;

    let index = entry.index as usize;
    let dir_contents = &entry.contents;
    let vec_len = dir_contents.len();
    if vec_len == 0 {
        wprint_empty(win, "empty");
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
    } else if index + win.rows as usize >= vec_len + offset  {
        start = vec_len - win.rows as usize;
        end = vec_len;
    } else {
        start = index - offset;
        end = start + win.rows as usize;
    }

    ncurses::wmove(win.win, 0, 0);

    for i in start..end {
        let coord: (i32, i32) = (i as i32 - start as i32, 0);
        wprint_direntry(win, &dir_contents[i], coord);

        if let Ok(metadata) = fs::symlink_metadata(&dir_contents[i].path) {
            mode = metadata.permissions().mode();
        }

        if dir_contents[i].selected {
            if index == i {
                ncurses::mvwchgat(win.win, coord.0, coord.1, -1, ncurses::A_BOLD() | ncurses::A_STANDOUT(), SELECT_COLOR);
            } else {
                ncurses::mvwchgat(win.win, coord.0, coord.1, -1, ncurses::A_BOLD(), SELECT_COLOR);
            }
        } else if mode != 0 {
            if index == i {
                file_attr_apply(win.win, coord, mode,
                    dir_contents[i].path.extension(), ncurses::A_STANDOUT());
            } else {
                file_attr_apply(win.win, coord, mode,
                    dir_contents[i].path.extension(), ncurses::A_NORMAL());
            }
        }

    }
    ncurses::wnoutrefresh(win.win);
}

pub fn display_options(win: &window::JoshutoPanel, vals: &Vec<String>)
{
    ncurses::werase(win.win);

    let ch = '-' as ncurses::chtype;
    ncurses::mvwhline(win.win, 0, 0, ch, 10000);

    for (i, val) in vals.iter().enumerate() {
        ncurses::wmove(win.win, (i+1) as i32, 0);
        ncurses::waddstr(win.win, val.as_str());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn redraw_view(win: &window::JoshutoPanel,
        view: Option<&structs::JoshutoDirList>
        )
{
    if let Some(s) = view {
        s.display_contents(win);
        ncurses::wnoutrefresh(win.win);
    } else {
        ncurses::werase(win.win);
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn redraw_status(joshuto_view : &window::JoshutoView,
    curr_view: Option<&structs::JoshutoDirList>, curr_path: &path::PathBuf,
    username: &str, hostname: &str)
{
    if let Some(s) = curr_view.as_ref() {
        let dirent = s.get_curr_entry();
        if let Some(dirent) = dirent {
            wprint_path(&joshuto_view.top_win, username, hostname,
                    curr_path, dirent.file_name_as_string.as_str());
            wprint_file_info(joshuto_view.bot_win.win, &dirent);
        }
    }
}

pub fn get_str(win: &window::JoshutoPanel,
        coord: (i32, i32)) -> Option<String>
{
    let user_input: Vec<(u8, char)> = Vec::new();
    get_str_prefill(win, coord, user_input, coord.1, 0)
}

pub fn get_str_append(win: &window::JoshutoPanel,
        coord: (i32, i32), start_str: String) -> Option<String>
{
    let mut user_input: Vec<(u8, char)> = Vec::new();
    for (_, ch) in start_str.char_indices() {
        let char_len = wcwidth::char_width(ch).unwrap_or(1);
        user_input.push((char_len, ch));
    }
    let mut curs_x = coord.1;
    for (size, _) in &user_input {
        curs_x = curs_x + (*size) as i32;
    }
    let curr_index = user_input.len();
    get_str_prefill(win, coord, user_input, curs_x, curr_index)
}

pub fn get_str_prepend(win: &window::JoshutoPanel,
        coord: (i32, i32), start_str: String) -> Option<String>
{
    let mut user_input: Vec<(u8, char)> = Vec::new();
    for (_, ch) in start_str.char_indices() {
        let char_len = wcwidth::char_width(ch).unwrap_or(1);
        user_input.push((char_len, ch));
    }
    get_str_prefill(win, coord, user_input, coord.1, 0)
}

pub fn get_str_prefill(win: &window::JoshutoPanel,
        coord: (i32, i32), mut user_input: Vec<(u8, char)>,
        mut curs_x: i32, mut curr_index: usize) -> Option<String>
{
    loop {
        ncurses::wmove(win.win, coord.0, coord.1);
        for (_, ch) in &user_input {
            ncurses::waddstr(win.win, ch.to_string().as_str());
        }
        ncurses::waddstr(win.win, "    ");

        ncurses::mvwchgat(win.win, coord.0, curs_x, 1,
                ncurses::A_STANDOUT(), 0);
        ncurses::wrefresh(win.win);

        let ch: i32 = ncurses::wgetch(win.win);

        if ch == Keycode::ESCAPE as i32 {
            return None;
        } else if ch == Keycode::ENTER as i32 {
            break;
        } else if ch == Keycode::HOME as i32 {
            if curr_index != 0 {
                curs_x = coord.1;
                curr_index = 0;
            }
        } else if ch == Keycode::END as i32 {
            let user_input_len = user_input.len();
            if curr_index != user_input_len {
                for i in curr_index..user_input_len {
                    curs_x = curs_x + user_input[i].0 as i32;
                }
                curr_index = user_input_len;
            }
        } else if ch == Keycode::LEFT as i32 {
            if curr_index > 0 {
                curr_index = curr_index - 1;
                curs_x = curs_x - user_input[curr_index].0 as i32;
            }
        } else if ch == Keycode::RIGHT as i32 {
            let user_input_len = user_input.len();
            if curr_index < user_input_len {
                curs_x = curs_x + user_input[curr_index].0 as i32;
                curr_index = curr_index + 1;
            }
        } else if ch == Keycode::BACKSPACE as i32 {
            let user_input_len = user_input.len();

            if user_input_len == 0 {
                continue;
            }

            if curr_index == user_input_len {
                curr_index = curr_index - 1;
                if let Some((size, _)) = user_input.pop() {
                    curs_x = curs_x - size as i32;
                }
            } else if curr_index > 0 {
                curr_index = curr_index - 1;
                let (size, _) = user_input.remove(curr_index);
                curs_x = curs_x - size as i32;
            }
        } else if ch == Keycode::DELETE as i32 {
            let user_input_len = user_input.len();

            if user_input_len == 0 || curr_index == user_input_len {
                continue;
            }

            if curr_index > 0 {
                let (size, _) = user_input.remove(curr_index);
                if curr_index > user_input_len {
                    curr_index = curr_index - 1;
                    curs_x = curs_x - size as i32;
                }
            } else if curr_index == 0 {
                user_input.remove(curr_index);
            }
        } else {
            let user_input_len = user_input.len();

            let ch = ch as u8 as char;
            let char_len = wcwidth::char_width(ch).unwrap_or(1);
            let size_ch = (char_len, ch);

            if curr_index == user_input_len {
                user_input.push(size_ch);
            } else {
                user_input.insert(curr_index, size_ch);
            }
            curs_x = curs_x + user_input[curr_index].0 as i32;
            curr_index = curr_index + 1;
        }
    }
    let user_str: String = user_input.iter().map(|(_, ch)| ch).collect();

    return Some(user_str);

}

pub fn draw_loading_bar(win: &window::JoshutoPanel, percentage: f32)
{
    let cols: i32 = (win.cols as f32 * percentage) as i32;
    ncurses::mvwchgat(win.win, 0, 0, cols, ncurses::A_STANDOUT(), 0);
}

fn file_attr_apply(win: ncurses::WINDOW, coord : (i32, i32), mode : u32,
        file_extension: Option<&ffi::OsStr>, attr : ncurses::attr_t)
{
    match mode & unix::BITMASK {
        unix::S_IFLNK | unix::S_IFCHR | unix::S_IFBLK => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, SOCK_COLOR);
        },
        unix::S_IFSOCK | unix::S_IFIFO => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, SOCK_COLOR);
        },
        unix::S_IFDIR => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, DIR_COLOR);
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
