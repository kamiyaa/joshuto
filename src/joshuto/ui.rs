extern crate ncurses;
extern crate wcwidth;
extern crate chrono;

use std::fs;
use std::path;
use std::time;

use joshuto;
use joshuto::config;
use joshuto::preview;
use joshuto::structs;
use joshuto::unix;
use joshuto::window;

pub const ERR_COLOR: i16 = 240;
pub const EMPTY_COLOR: i16 = 241;

pub fn init_ncurses(theme_t: &config::JoshutoTheme)
{
    let locale_conf = ncurses::LcCategory::all;

    ncurses::setlocale(locale_conf, "");

    ncurses::initscr();
    ncurses::cbreak();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::start_color();
    ncurses::use_default_colors();
    ncurses::noecho();
    ncurses::set_escdelay(0);

    process_theme(&theme_t);

    ncurses::printw("Loading...");
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    ncurses::refresh();
}

fn process_theme(theme_t: &config::JoshutoTheme)
{
    for pair in theme_t.colorpair.iter() {
        ncurses::init_pair(pair.id, pair.fg, pair.bg);
    }

    /* error message */
    ncurses::init_pair(ERR_COLOR, ncurses::COLOR_RED, -1);
    /* empty */
    ncurses::init_pair(EMPTY_COLOR, ncurses::COLOR_WHITE, ncurses::COLOR_RED);
}

pub fn end_ncurses()
{
        ncurses::endwin();
}

pub fn display_options(win: &window::JoshutoPanel, vals: &Vec<String>)
{
    ncurses::werase(win.win);
    ncurses::mvwhline(win.win, 0, 0, 0, win.cols);

    for (i, val) in vals.iter().enumerate() {
        ncurses::wmove(win.win, (i+1) as i32, 0);
        ncurses::waddstr(win.win, val.as_str());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_msg(win: &window::JoshutoPanel, msg: &str)
{
    ncurses::werase(win.win);
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_err(win: &window::JoshutoPanel, msg: &str)
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

pub fn wprint_path(win: &window::JoshutoPanel, theme_t: &config::JoshutoTheme,
        username: &str, hostname: &str, path: &path::PathBuf, file_name: &str)
{
    ncurses::werase(win.win);
    let path_str: &str = match path.to_str() {
            Some(s) => s,
            None => "Error",
        };
    ncurses::wattron(win.win, ncurses::A_BOLD());
    ncurses::mvwaddstr(win.win, 0, 0, username);
    ncurses::waddstr(win.win, "@");
    ncurses::waddstr(win.win, hostname);

    ncurses::waddstr(win.win, " ");

    ncurses::wattron(win.win, ncurses::COLOR_PAIR(theme_t.directory.colorpair));
    ncurses::waddstr(win.win, path_str);
    ncurses::waddstr(win.win, "/");
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(theme_t.directory.colorpair));
    ncurses::waddstr(win.win, file_name);
    ncurses::wattroff(win.win, ncurses::A_BOLD());
}

fn wprint_file_size(win: ncurses::WINDOW, mut file_size: f64)
{
    const FILE_UNITS: [&str ; 6] = ["B", "KB", "MB", "GB", "TB", "EB"];
    const CONV_RATE: f64 = 1024.0;

    let mut index = 0;
    while file_size > CONV_RATE {
        file_size = file_size / CONV_RATE;
        index += 1;
    }

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
}

pub fn wprint_file_mtime(win: ncurses::WINDOW, mtime: time::SystemTime)
{
    const MTIME_FORMATTING: &str = "%Y-%m-%d %H:%M";

    let datetime: chrono::DateTime<chrono::offset::Utc> = mtime.into();
    ncurses::waddstr(win, format!("{}", datetime.format(MTIME_FORMATTING)).as_str());
}

fn wprint_file_mode(win: ncurses::WINDOW, file: &structs::JoshutoDirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    let mode = file.metadata.permissions.mode();

    ncurses::wattron(win, ncurses::COLOR_PAIR(6));
    ncurses::waddstr(win, unix::stringify_mode(mode).as_str());
    ncurses::wattroff(win, ncurses::COLOR_PAIR(6));
}

pub fn wprint_file_info(win: ncurses::WINDOW, file: &structs::JoshutoDirEntry)
{
    use std::os::unix::fs::PermissionsExt;

    let mode = file.metadata.permissions.mode();

    wprint_file_mtime(win, file.metadata.modified);

    ncurses::waddstr(win, " ");

    if file.path.is_dir() {
        if mode >> 9 & unix::S_IFLNK >> 9 == mode >> 9 {
            if let Ok(path) = fs::read_link(&file.path) {
                ncurses::waddstr(win, " -> ");
                ncurses::waddstr(win, path.to_str().unwrap());
            }
        }
    } else {
        let file_size = file.metadata.len as f64;
        wprint_file_size(win, file_size);
    }
}

fn wprint_file_name(win: &window::JoshutoPanel, file: &structs::JoshutoDirEntry,
        coord: (i32, i32))
{
    ncurses::mvwaddstr(win.win, coord.0, coord.1, " ");

    let file_name = &file.file_name_as_string;
    let name_visual_space = wcwidth::str_width(file_name).unwrap_or(win.cols as usize);
    if name_visual_space < win.cols as usize {
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

pub fn wprint_directory_len(win: ncurses::WINDOW, curr_list: &structs::JoshutoDirList)
{
    if curr_list.index >= 0 {
        ncurses::waddstr(win, format!("{}/{}", curr_list.index + 1, curr_list.contents.len()).as_str());
    }
}

pub fn wprint_direntry(win: &window::JoshutoPanel,
        file: &structs::JoshutoDirEntry, coord: (i32, i32))
{
//    let offset = wprint_file_size(win, file, coord);
//    let offset = 3;
    wprint_file_name(win, file, coord);
}

pub fn refresh(context: &mut joshuto::JoshutoContext)
{
    if context.tabs.len() == 0 {
        return;
    }

    {
        let curr_tab = &mut context.tabs[context.tab_index];

        redraw_view(&context.config_t, &context.theme_t,
                &context.views.left_win, curr_tab.parent_list.as_mut());
        redraw_view_detailed(&context.config_t, &context.theme_t,
                &context.views.mid_win, curr_tab.curr_list.as_mut());
        redraw_status(&context.theme_t, &context.views,
                curr_tab.curr_list.as_ref(),
                &curr_tab.curr_path,
                &context.username, &context.hostname);
    }

    preview::preview_file(context);
}

pub fn redraw_view(config_t: &config::JoshutoConfig, theme_t: &config::JoshutoTheme, win: &window::JoshutoPanel,
        mut view: Option<&mut structs::JoshutoDirList>)
{
    if let Some(s) = view.as_mut() {
        display_contents(config_t, theme_t, win, s);
        ncurses::wnoutrefresh(win.win);
    } else {
        ncurses::werase(win.win);
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn redraw_view_detailed(config_t: &config::JoshutoConfig, theme_t: &config::JoshutoTheme, win: &window::JoshutoPanel,
        mut view: Option<&mut structs::JoshutoDirList>)
{
    if let Some(ref mut s) = view {
        display_contents(config_t, theme_t, win, s);
        ncurses::wnoutrefresh(win.win);
    } else {
        ncurses::werase(win.win);
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn redraw_status(
    theme_t: &config::JoshutoTheme,
    joshuto_view: &window::JoshutoView,
    curr_view: Option<&structs::JoshutoDirList>,
    curr_path: &path::PathBuf,
    username: &str, hostname: &str)
{
    if let Some(s) = curr_view.as_ref() {
        let dirent = s.get_curr_entry();
        if let Some(dirent) = dirent {
            wprint_path(&joshuto_view.top_win,
                    theme_t, username, hostname,
                    curr_path, dirent.file_name_as_string.as_str());
            ncurses::wnoutrefresh(joshuto_view.top_win.win);

            ncurses::werase(joshuto_view.bot_win.win);
            ncurses::wmove(joshuto_view.bot_win.win, 0, 0);
            wprint_file_mode(joshuto_view.bot_win.win, &dirent);
            ncurses::waddstr(joshuto_view.bot_win.win, "  ");

            wprint_directory_len(joshuto_view.bot_win.win, s);
            ncurses::waddstr(joshuto_view.bot_win.win, "  ");

            wprint_file_info(joshuto_view.bot_win.win, &dirent);
            ncurses::wnoutrefresh(joshuto_view.bot_win.win);
        }
    }
}

pub fn redraw_tab_view(win: &window::JoshutoPanel, context: &joshuto::JoshutoContext)
{
    let tab_len = context.tabs.len();
    if tab_len == 1 {
        ncurses::werase(win.win);
    } else {
        ncurses::wmove(win.win, 0, 0);
        ncurses::wattron(win.win, ncurses::A_BOLD());
        ncurses::waddstr(win.win, format!("{} {}", context.tab_index + 1, tab_len).as_str());
        ncurses::wattroff(win.win, ncurses::A_BOLD());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn draw_progress_bar(theme_t: &config::JoshutoTheme,
        win: &window::JoshutoPanel, percentage: f32)
{
    let cols: i32 = (win.cols as f32 * percentage) as i32;
    ncurses::mvwchgat(win.win, 0, 0, cols, ncurses::A_STANDOUT(),
            theme_t.selection.colorpair);
}

pub fn display_contents(config_t: &config::JoshutoConfig,
        theme_t: &config::JoshutoTheme, win: &window::JoshutoPanel,
        dirlist: &mut structs::JoshutoDirList)
{
    let index = dirlist.index;
    let vec_len = dirlist.contents.len();
    if vec_len == 0 {
        wprint_empty(win, "empty");
        return;
    }
    if index >= 0 {
        dirlist.pagestate.update_page_state(index as usize, win.rows, vec_len, config_t.scroll_offset);
    }
    draw_contents(theme_t, win, dirlist);
}

pub fn draw_contents(theme_t: &config::JoshutoTheme,
        win: &window::JoshutoPanel, dirlist: &structs::JoshutoDirList)
{
    use std::os::unix::fs::PermissionsExt;

    ncurses::werase(win.win);
    ncurses::wmove(win.win, 0, 0);

    let dir_contents = &dirlist.contents;

    let (start, end) = (dirlist.pagestate.start, dirlist.pagestate.end);

    for i in start..end {
        let coord: (i32, i32) = (i as i32 - start as i32, 0);
        wprint_direntry(win, &dir_contents[i], coord);

        let mode = dir_contents[i].metadata.permissions.mode();

        let mut attr: ncurses::attr_t = 0;
        if dirlist.index as usize == i {
            attr = attr | ncurses::A_STANDOUT();
        }

        if dir_contents[i].selected {
            ncurses::mvwchgat(win.win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, theme_t.selection.colorpair);
        } else if mode != 0 {
            let file_name = &dir_contents[i].file_name_as_string;
            let mut extension: &str = "";
            if let Some(ext) = file_name.rfind('.') {
                extension = &file_name[ext+1..];
            }

            file_attr_apply(theme_t, win.win, coord, mode, extension, attr);
        }

    }
    ncurses::wnoutrefresh(win.win);
}

fn file_attr_apply(theme_t: &config::JoshutoTheme,
        win: ncurses::WINDOW, coord: (i32, i32), mode: u32,
        extension: &str, attr: ncurses::attr_t)
{
    match mode & unix::BITMASK {
        unix::S_IFLNK | unix::S_IFCHR | unix::S_IFBLK => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, theme_t.socket.colorpair);
        },
        unix::S_IFSOCK | unix::S_IFIFO => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, theme_t.socket.colorpair);
        },
        unix::S_IFDIR => {
            ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, theme_t.directory.colorpair);
        },
        unix::S_IFREG => {
            if unix::is_executable(mode) == true {
                ncurses::mvwchgat(win, coord.0, coord.1, -1, ncurses::A_BOLD() | attr, theme_t.executable.colorpair);
            }
            else if let Some(s) = theme_t.ext.get(extension) {
                ncurses::mvwchgat(win, coord.0, coord.1, -1, attr, s.colorpair);
            } else {
                ncurses::mvwchgat(win, coord.0, coord.1, -1, attr, 0);
            }
        },
        _ => {},
    };
}
