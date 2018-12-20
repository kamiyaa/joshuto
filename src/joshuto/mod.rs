#[allow(dead_code)]
extern crate ncurses;

use std;
use std::env;
use std::path;
use std::process;
// use std::collections::HashMap;

pub mod config;
pub mod sort;
mod history;
mod structs;
mod ui;
mod unix;
mod navigation;

fn redraw_views(joshuto_view : &structs::JoshutoView,
        parent_view: Option<&structs::JoshutoDirList>,
        curr_view: Option<&structs::JoshutoDirList>,
        preview_view: Option<&structs::JoshutoDirList>)
{
    if let Some(s) = parent_view {
        s.display_contents(&joshuto_view.left_win);
        ncurses::wnoutrefresh(joshuto_view.left_win.win);
    }

    if let Some(s) = curr_view {
        s.display_contents(&joshuto_view.mid_win);
        ncurses::wnoutrefresh(joshuto_view.mid_win.win);
    }

    if let Some(s) = preview_view {
        s.display_contents(&joshuto_view.right_win);
        ncurses::wnoutrefresh(joshuto_view.right_win.win);
    }
}

fn refresh_handler(config_t: &config::JoshutoConfig,
        joshuto_view : &mut structs::JoshutoView,
        curr_path : &path::PathBuf,
        parent_view : Option<&structs::JoshutoDirList>,
        curr_view : Option<&structs::JoshutoDirList>,
        preview_view : Option<&structs::JoshutoDirList>)
{
    joshuto_view.redraw_views();
    ncurses::refresh();

    ui::wprint_path(&joshuto_view.top_win, &config_t.username,
        &config_t.hostname, curr_path);

    redraw_views(joshuto_view, parent_view, curr_view, preview_view);

    let index = curr_view.unwrap().index;
    if index >= 0 {
        let index : usize = index as usize;
        let dirent : &structs::JoshutoDirEntry = &curr_view.unwrap().contents
                                        .as_ref().unwrap()[index];

        ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);
    }

    ncurses::doupdate();
}

pub fn run(mut config_t : config::JoshutoConfig)
{
    let mut curr_path : path::PathBuf = match env::current_dir() {
            Ok(path) => { path },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };
    println!("pwd: {:?}", curr_path);

    /* keep track of where we are in directories */
    let mut history = history::History::new();
    history.populate_to_root(&curr_path, &config_t.sort_type);
//    println!("History:\n{:#?}", history.map);

    ui::init_ncurses();

    let mut joshuto_view: structs::JoshutoView =
        structs::JoshutoView::new(config_t.column_ratio);

    /* load up directories */
    let mut curr_view: Option<structs::JoshutoDirList> =
        match history.pop_or_create(&curr_path, &config_t.sort_type) {
            Ok(s) => { Some(s) },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

    let mut parent_view: Option<structs::JoshutoDirList> =
        match curr_path.parent() {
            Some(parent) => {
                match history.pop_or_create(&parent, &config_t.sort_type) {
                    Ok(s) => { Some(s) },
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    },
                }
            },
            None => { None },
        };

    let mut preview_view : Option<structs::JoshutoDirList>;
    if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() > 0 &&
        curr_view.as_ref().unwrap().index >= 0 {
        let index : usize = curr_view.as_ref().unwrap().index as usize;
        let dirent : &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
                        .contents.as_ref().unwrap()[index];
        ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);

        let preview_path = dirent.entry.path();
        if preview_path.is_dir() {
            preview_view = match structs::JoshutoDirList::new(&preview_path, &config_t.sort_type) {
                Ok(s) => { Some(s) },
                Err(e) => {
                    eprintln!("{}", e);
                    None
                },
            };
        } else {
            preview_view = None;
            ncurses::werase(joshuto_view.right_win.win);
            let mime_type = unix::get_mime_type(&dirent.entry);
            ui::wprint_msg(&joshuto_view.right_win, mime_type.as_str());
            ncurses::wnoutrefresh(joshuto_view.right_win.win);
        }
    } else {
        preview_view = None;
    }
    ui::wprint_path(&joshuto_view.top_win, &config_t.username, &config_t.hostname,
            &curr_path);

    redraw_views(&joshuto_view,
                    parent_view.as_ref(),
                    curr_view.as_ref(),
                    preview_view.as_ref());

    ncurses::doupdate();

    loop {
        let ch : i32 = ncurses::getch();
        if ch == 'q' as i32 {
            break;
        }

        if ch == ncurses::KEY_RESIZE {
            refresh_handler(&config_t, &mut joshuto_view, &curr_path,
                    parent_view.as_ref(),
                    curr_view.as_ref(),
                    preview_view.as_ref());
        } else if ch == ncurses::KEY_UP {
            let curr_index = curr_view.as_ref().unwrap().index;
            if curr_index <= 0 {
                continue;
            }

            preview_view = match navigation::set_dir_cursor_index(&mut history,
                    curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type,
                    curr_index - 1) {
                Ok(s) => Some(s),
                Err(e) => {
                    ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                    None
                },
            };

            redraw_views(&joshuto_view,
                            None.as_ref(),
                            curr_view.as_ref(),
                            preview_view.as_ref());
            ncurses::doupdate();

        } else if ch == ncurses::KEY_DOWN {
            let curr_index = curr_view.as_ref().unwrap().index;
            let dir_len = curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap().len() as i32;

            if curr_index + 1 >= dir_len {
                continue;
            }

            preview_view = match navigation::set_dir_cursor_index(&mut history,
                    curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type,
                    curr_index + 1) {
                Ok(s) => Some(s),
                Err(e) => {
                    ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                    None
                },
            };

            redraw_views(&joshuto_view,
                            None.as_ref(),
                            curr_view.as_ref(),
                            preview_view.as_ref());
            ncurses::doupdate();

        } else if ch == ncurses::KEY_HOME {
            let curr_index = curr_view.as_ref().unwrap().index;
            if curr_index <= 0 {
                continue;
            }

            preview_view = match navigation::set_dir_cursor_index(&mut history,
                    curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type, 0) {
                Ok(s) => Some(s),
                Err(e) => {
                    ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                    None
                },
            };

            redraw_views(&joshuto_view,
                            None.as_ref(),
                            curr_view.as_ref(),
                            preview_view.as_ref());
            ncurses::doupdate();

        } else if ch == ncurses::KEY_END {
            let curr_index = curr_view.as_ref().unwrap().index;
            let dir_len = curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap().len() as i32;
            if curr_index == dir_len - 1 {
                continue;
            }

            preview_view = match navigation::set_dir_cursor_index(&mut history,
                    curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type,
                    (dir_len - 1) as i32) {
                Ok(s) => Some(s),
                Err(e) => {
                    ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                    None
                },
            };

            redraw_views(&joshuto_view,
                            None.as_ref(),
                            curr_view.as_ref(),
                            preview_view.as_ref());
            ncurses::doupdate();

        } else if ch == ncurses::KEY_NPAGE {
            let curr_index = curr_view.as_ref().unwrap().index as usize;
            let dir_len = curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap().len();

            if curr_index == dir_len - 1 {
                continue;
            }

            let half_page : i32 = joshuto_view.mid_win.cols / 2;
            let curr_index = if curr_index + half_page as usize >= dir_len {
                (dir_len - 1) as i32
            } else {
                curr_index as i32 + half_page
            };

            preview_view = match navigation::set_dir_cursor_index(&mut history,
                    curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type,
                    curr_index) {
                Ok(s) => Some(s),
                Err(e) => {
                    ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                    None
                },
            };

            redraw_views(&joshuto_view,
                            None.as_ref(),
                            curr_view.as_ref(),
                            preview_view.as_ref());
            ncurses::doupdate();

        } else if ch == ncurses::KEY_PPAGE {
            let curr_index = curr_view.as_ref().unwrap().index as usize;
            if curr_index <= 0 {
                continue;
            }

            let half_page : i32 = joshuto_view.mid_win.cols / 2;
            let curr_index = if curr_index < half_page as usize {
                0
            } else {
                curr_index as i32 - half_page
            };

            preview_view = match navigation::set_dir_cursor_index(&mut history,
                    curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type,
                    curr_index) {
                Ok(s) => Some(s),
                Err(e) => {
                    ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                    None
                },
            };

            redraw_views(&joshuto_view,
                            None.as_ref(),
                            curr_view.as_ref(),
                            preview_view.as_ref());
            ncurses::doupdate();

        } else if ch == ncurses::KEY_LEFT {
            if curr_path.pop() == false {
                continue;
            }

            match env::set_current_dir(curr_path.as_path()) {
                Ok(_) => {
                    if let Some(s) = preview_view {
                        let index : usize = curr_view.as_ref().unwrap().index as usize;
                        let path : path::PathBuf = curr_view.as_ref().unwrap()
                                            .contents.as_ref()
                                            .unwrap()[index].entry.path();
                        history.insert(path, s);
                    }

                    preview_view = curr_view;
                    curr_view = parent_view;

                    match curr_path.parent() {
                        Some(parent) => {
                            parent_view = match history.pop_or_create(&parent, &config_t.sort_type) {
                                Ok(s) => { Some(s) },
                                Err(e) => {
                                    ui::wprint_err(&joshuto_view.left_win, format!("{}", e).as_str());
                                    None
                                },
                            };
                            parent_view.as_ref().unwrap().display_contents(&joshuto_view.left_win);
                        },
                        None => {
                            ncurses::werase(joshuto_view.left_win.win);
                            ncurses::wnoutrefresh(joshuto_view.left_win.win);
                            parent_view = None;
                        },
                    };

                    redraw_views(&joshuto_view,
                            parent_view.as_ref(),
                            curr_view.as_ref(),
                            preview_view.as_ref());
                },
                Err(e) => {
                    ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                },
            };

            let index : usize = curr_view.as_ref().unwrap().index as usize;
            let dirent : &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap()[index];

            ui::wprint_path(&joshuto_view.top_win, &config_t.username,
                    &config_t.hostname, &curr_path);
            ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);

            ncurses::doupdate();

        } else if ch == ncurses::KEY_RIGHT || ch == '\n' as i32 {
            if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                continue;
            }

            let index = curr_view.as_ref().unwrap().index as usize;
            let path = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index].entry.path();
            if path.is_dir() {
                match env::set_current_dir(&path) {
                    Ok(_) => {
                        if let Some(s) = parent_view {
                            if let Some(path) = curr_path.parent() {
                                history.insert(path.to_path_buf(), s);
                            }
                        }

                        parent_view = curr_view;
                        curr_view = preview_view;
                        preview_view = None;

                        /* update curr_path */
                        match path.strip_prefix(curr_path.as_path()) {
                            Ok(s) => curr_path.push(s),
                            Err(e) => {
                                ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                                continue;
                            }
                        }

                        ui::wprint_path(&joshuto_view.top_win, &config_t.username, &config_t.hostname,
                                &curr_path);

                        if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                            redraw_views(&joshuto_view,
                                    parent_view.as_ref(),
                                    curr_view.as_ref(),
                                    preview_view.as_ref());
                            ncurses::doupdate();
                            continue;
                        }

                        let index : usize = curr_view.as_ref().unwrap().index as usize;
                        let dirent : &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[index];
                        let new_path = dirent.entry.path();

                        if new_path.is_dir() {
                            preview_view = match history.pop_or_create(new_path.as_path(), &config_t.sort_type) {
                                Ok(s) => { Some(s) },
                                Err(e) => {
                                    ui::wprint_err(&joshuto_view.right_win,
                                            format!("{}", e).as_str());
                                    None
                                },
                            };
                            ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);
                        } else {
                            ncurses::werase(joshuto_view.right_win.win);
                            ui::wprint_err(&joshuto_view.right_win, "Not a directory");
                        }

                        redraw_views(&joshuto_view,
                                parent_view.as_ref(),
                                curr_view.as_ref(),
                                preview_view.as_ref());

                        ncurses::doupdate();
                    }
                    Err(e) => {
                        ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                    }
                };
            }
        } else if ch == 'z' as i32 {
            let ch2 : i32 = ncurses::getch();
            /* toggle show hidden */
            if ch2 == 'h' as i32 {
                {
                    let opposite = !config_t.sort_type.show_hidden();
                    config_t.sort_type.set_show_hidden(opposite);
                    history.depecrate_all_entries();
                }

                if let Some(s) = curr_view.as_mut() {
                    s.update(&curr_path, &config_t.sort_type);
                    ui::display_contents(&joshuto_view.mid_win, &s);
                    ncurses::wnoutrefresh(joshuto_view.mid_win.win);
                }

                if let Some(s) = parent_view.as_mut() {
                    if curr_path.parent() != None {
                        s.update(curr_path.parent().unwrap(), &config_t.sort_type);
                        ui::display_contents(&joshuto_view.left_win, &s);
                        ncurses::wnoutrefresh(joshuto_view.left_win.win);
                    }
                }

                if let Some(s) = preview_view.as_mut() {
                    if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() > 0 {
                        let index : usize = curr_view.as_ref().unwrap().index as usize;
                        let dirent : &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
                                        .contents.as_ref().unwrap()[index];

                        ui::wprint_path(&joshuto_view.top_win, &config_t.username,
                            &config_t.hostname, &dirent.entry.path());

                        s.update(dirent.entry.path().as_path(), &config_t.sort_type);

                        ui::display_contents(&joshuto_view.right_win, &s);
                        ncurses::wnoutrefresh(joshuto_view.right_win.win);

                        ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);
                    }
                }
                ncurses::doupdate();
            }
        } else if ch == ncurses::KEY_DC { // delete button
            if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                continue;
            }
            let index = curr_view.as_ref().unwrap().index as usize;
            let file_name = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index].entry.file_name();
            ui::wprint_msg(&joshuto_view.bot_win,
                format!("Delete {:?}? (y/n)", file_name).as_str());
            ncurses::doupdate();
            let ch2 = ncurses::wgetch(joshuto_view.bot_win.win);
            if ch2 == 'y' as i32 {
                let path = &curr_view.as_ref().unwrap()
                                    .contents.as_ref()
                                    .unwrap()[index].entry.path();
                match std::fs::remove_file(path) {
                    Ok(_s) => {
                        curr_view.as_mut().unwrap().update(&curr_path,
                            &config_t.sort_type);
                        ui::display_contents(&joshuto_view.mid_win,
                            curr_view.as_ref().unwrap());
                        ui::wprint_msg(&joshuto_view.bot_win,
                            format!("Deleted {:?}!", file_name).as_str());
                        ncurses::wnoutrefresh(joshuto_view.mid_win.win);
                    },
                    Err(e) => {
                        ui::wprint_err(&joshuto_view.bot_win,
                            format!("{}", e).as_str());
                    }
                }
            } else {
                let dirent : &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index];
                ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);
            }
            ncurses::doupdate();
        } else if ch == 'r' as i32 {
            if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                continue;
            }

            let index = curr_view.as_ref().unwrap().index as usize;
            let dirent = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index];

            ncurses::wclear(joshuto_view.bot_win.win);
            let mut str_input : String = String::new();
            ncurses::mvwprintw(joshuto_view.bot_win.win, 0, 0, "open_with:");
            ncurses::wgetstr(joshuto_view.bot_win.win, &mut str_input);

            ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);
            ncurses::doupdate();

        } else if ch == ' ' as i32 {
            let index = curr_view.as_ref().unwrap().index as usize;
            curr_view.as_mut().unwrap().contents.as_mut().unwrap()[index].selected
                = !curr_view.as_ref().unwrap().contents.as_ref().unwrap()[index].selected;
            let dir_len = curr_view.as_ref().unwrap()
                    .contents.as_ref().unwrap().len();

            if index + 1 >= dir_len {
                continue;
            }
            curr_view.as_mut().unwrap().index =
                curr_view.as_ref().unwrap().index + 1;

            ui::display_contents(&joshuto_view.mid_win, curr_view.as_ref().unwrap());
            ncurses::wnoutrefresh(joshuto_view.mid_win.win);
            ncurses::doupdate();
        } else {
            ui::wprint_err(&joshuto_view.bot_win,
                format!("Unknown keychar: ({}: {})", ch, ch as u8 as char).as_str());
        }
        // eprintln!("{:?}\n\n", history);
    }
    ncurses::endwin();
}
