extern crate ncurses;
extern crate whoami;

use std;
use std::env;
use std::fs;
use std::path;
use std::process;
use std::collections::HashMap;

mod history;
mod sort;
mod structs;
mod ui;
mod unix;
pub mod config;

pub fn refresh_handler(joshuto_view : &mut structs::JoshutoView,
        curr_path : &path::PathBuf, parent_view : Option<&structs::JoshutoDirList>,
        curr_view : Option<&structs::JoshutoDirList>,
        preview_view : Option<&structs::JoshutoDirList>)
{
    let username : String = whoami::username();
    let hostname : String = whoami::hostname();

    joshuto_view.redraw_views();
    ncurses::refresh();

    ui::wprint_path(&joshuto_view.top_win, username.as_str(),
        hostname.as_str(), curr_path);

    if let Some(s) = parent_view {
        ui::display_contents(&joshuto_view.left_win, &s);
        ncurses::wnoutrefresh(joshuto_view.left_win.win);
    }

    ui::display_contents(&joshuto_view.mid_win, curr_view.unwrap());
    ncurses::wnoutrefresh(joshuto_view.mid_win.win);

    if let Some(s) = preview_view {
        ui::display_contents(&joshuto_view.right_win, &s);
        ncurses::wnoutrefresh(joshuto_view.right_win.win);
    }

    let index : usize = curr_view.unwrap().index;
    let dirent : &structs::JoshutoDirEntry = &curr_view.unwrap().contents
                                    .as_ref().unwrap()[index];

    ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);

    ncurses::doupdate();
}

/*
pub fn updown(history : &mut HashMap<String, structs::JoshutoDirList>,
        joshuto_view : &structs::JoshutoView,
        old_path : &path::Path,
        curr : &structs::JoshutoDirList,
        preview_view : Option<structs::JoshutoDirList>,
        sort_func : fn (&structs::JoshutoDirEntry, &structs::JoshutoDirEntry) -> std::cmp::Ordering,
        show_hidden : bool) -> Option<structs::JoshutoDirList>
{
    let index : usize = curr.index;
    let dirent : &structs::JoshutoDirEntry = &curr.contents.as_ref().unwrap()[index];
    let new_path = dirent.entry.path();

    let old_osstr = match old_path.to_str() {
            Some(s) => s,
            None => return None,
        };

    let tmp_key : String = format!("{}", old_osstr);

    match preview_view {
        Some(s) => { history.insert(tmp_key, s); },
        None => {},
    };

    let username : String = whoami::username();
    let hostname : String = whoami::hostname();

    ui::wprint_path(&joshuto_view.top_win, username.as_str(), hostname.as_str(),
            &new_path);

    ui::display_contents(&joshuto_view.mid_win, curr);
    ncurses::wnoutrefresh(joshuto_view.mid_win.win);

    ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);

    let preview : Option<structs::JoshutoDirList>;
    if new_path.is_dir() {
        match history::pop_or_create(history, new_path.as_path(), sort_func, show_hidden) {
            Ok(s) => {
                preview = Some(s);
                ui::display_contents(&joshuto_view.right_win, preview.as_ref().unwrap());
            },
            Err(e) => {
                preview = None;
                ui::wprint_err(&joshuto_view.right_win, format!("{}", e).as_str());
                ncurses::wnoutrefresh(joshuto_view.right_win.win);
            },
        };
    } else {
        use std::os::unix::fs::PermissionsExt;

        preview = None;
        ncurses::werase(joshuto_view.right_win.win);
        if let Ok(metadata) = dirent.entry.metadata() {
            let permissions : fs::Permissions = metadata.permissions();
            let mode = permissions.mode();
            if unix::is_reg(mode) {
                let mime_type = unix::get_mime_type(&dirent.entry);
                ui::wprint_msg(&joshuto_view.right_win, mime_type.as_str());
            } else {
                ui::wprint_msg(&joshuto_view.right_win,
                    unix::get_unix_filetype(mode));
            }
        }
    }
    ncurses::wnoutrefresh(joshuto_view.right_win.win);
    ncurses::doupdate();

    preview
}
*/

pub fn run(config : config::JoshutoConfig)
{
    ui::init_ncurses();

    let username : String = whoami::username();
    let hostname : String = whoami::hostname();

    /* height, width, y, x */
    let mut joshuto_view : structs::JoshutoView =
        structs::JoshutoView::new(config.column_ratio);

    let mut curr_path : path::PathBuf = match env::current_dir() {
            Ok(path) => { path },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

    /* keep track of where we are in directories */
    let mut history = history::History::new();
    history.populate_to_root(&curr_path, &config.sort_type);

    /* load up directories */
    let mut curr_view : Option<structs::JoshutoDirList> =
        match history.pop_or_create(&parent, &config.sort_type) {
            Ok(s) => { Some(s) },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

    let mut parent_view : Option<structs::JoshutoDirList> =
        match curr_path.parent() {
            Some(parent) => {
                match history.pop_or_create(&parent, &config.sort_type) {
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
    if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() > 0 {
        let index : usize = curr_view.as_ref().unwrap().index;
        let dirent : &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
                        .contents.as_ref().unwrap()[index];
        ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);

        let preview_path = dirent.entry.path();
        if preview_path.is_dir() {
            preview_view = match structs::JoshutoDirList::new(&preview_path, &config.sort_type) {
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
        ui::wprint_path(&joshuto_view.top_win, username.as_str(), hostname.as_str(),
                &preview_path);
    } else {
        preview_view = None;
    }

    if let Some(s) = parent_view.as_ref() {
        ui::display_contents(&joshuto_view.left_win, &s);
        ncurses::wnoutrefresh(joshuto_view.left_win.win);
    }

    ui::display_contents(&joshuto_view.mid_win, curr_view.as_ref().unwrap());
    ncurses::wnoutrefresh(joshuto_view.mid_win.win);

    if let Some(s) = preview_view.as_ref() {
        ui::display_contents(&joshuto_view.right_win, &s);
        ncurses::wnoutrefresh(joshuto_view.right_win.win);
    }

    ncurses::doupdate();

    loop {
        let ch : i32 = ncurses::getch();
        if ch == 'q' as i32 {
            break;
        }

        if ch == ncurses::KEY_RESIZE {
            refresh_handler(&mut joshuto_view, &curr_path, parent_view.as_ref(),
                    curr_view.as_ref(), preview_view.as_ref());
        } else if ch == ncurses::KEY_UP {
            let curr_index = curr_view.as_ref().unwrap().index;
            if curr_index == 0 {
                continue;
            }
            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].entry.path();

            curr_view.as_mut().unwrap().index =
                    curr_view.as_ref().unwrap().index - 1;

/*
            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden); */

        } else if ch == ncurses::KEY_DOWN {
            let curr_index = curr_view.as_ref().unwrap().index;
            let dir_len = curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap().len();

            if curr_index + 1 >= dir_len {
                continue;
            }
            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].entry.path();

            curr_view.as_mut().unwrap().index =
                curr_view.as_ref().unwrap().index + 1;

/*
            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden);*/

        } else if ch == ncurses::KEY_HOME {
            let curr_index = curr_view.as_ref().unwrap().index;

            if curr_index == 0 {
                continue;
            }
            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].entry.path();

            curr_view.as_mut().unwrap().index = 0;

/*
            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden);
*/

        } else if ch == ncurses::KEY_END {
            let curr_index = curr_view.as_ref().unwrap().index;
            let dir_len = curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap().len();

            if curr_index == dir_len - 1 {
                continue;
            }
            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].entry.path();

            curr_view.as_mut().unwrap().index = dir_len - 1;

/*
            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden);
*/

        } else if ch == ncurses::KEY_NPAGE {
            let curr_index = curr_view.as_ref().unwrap().index;
            let dir_len = curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap().len();

            if curr_index == dir_len - 1 {
                continue;
            }

            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].entry.path();

            let half_page : i32 = joshuto_view.mid_win.cols / 2;
            if curr_index + half_page as usize >= dir_len {
                curr_view.as_mut().unwrap().index = dir_len - 1;
            } else {
                curr_view.as_mut().unwrap().index = curr_view.as_ref().unwrap().index
                    + half_page as usize;
            }

/*
            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden);
*/

        } else if ch == ncurses::KEY_PPAGE {
            let curr_index = curr_view.as_ref().unwrap().index;
            if curr_index == 0 {
                continue;
            }

            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].entry.path();

            let half_page : i32 = joshuto_view.mid_win.cols / 2;
            if curr_index < half_page as usize {
                curr_view.as_mut().unwrap().index = 0;
            } else {
                curr_view.as_mut().unwrap().index = curr_view.as_ref().unwrap().index
                    - half_page as usize;
            }

/*
            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden);
*/

        } else if ch == ncurses::KEY_LEFT {
            if curr_path.pop() == false {
                continue;
            }

            match env::set_current_dir(curr_path.as_path()) {
                Ok(_s) => {
                    if let Some(s) = preview_view {
                        let index : usize = curr_view.as_ref().unwrap().index;
                        let path : path::PathBuf = curr_view.as_ref().unwrap()
                                            .contents.as_ref()
                                            .unwrap()[index].entry.path();
                        history.map.insert(path, s);
                    }

                    preview_view = curr_view;
                    curr_view = parent_view;
                    ui::display_contents(&joshuto_view.mid_win,
                            curr_view.as_ref().unwrap());
                    ncurses::wnoutrefresh(joshuto_view.mid_win.win);
                    ui::display_contents(&joshuto_view.right_win,
                            preview_view.as_ref().unwrap());
                    ncurses::wnoutrefresh(joshuto_view.right_win.win);
                },
                Err(e) => {
                    ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                },
            };
            match curr_path.parent() {
                Some(parent) => {
                    parent_view = match history.pop_or_create(&parent, &config.sort_type) {
                        Ok(s) => { Some(s) },
                        Err(e) => {
                            ui::wprint_err(&joshuto_view.left_win, format!("{}", e).as_str());
                            None
                        },
                    };
                    ui::display_contents(&joshuto_view.left_win,
                            parent_view.as_ref().unwrap());
                    ncurses::wnoutrefresh(joshuto_view.left_win.win);
                },
                None => {
                    ncurses::werase(joshuto_view.left_win.win);
                    ncurses::wnoutrefresh(joshuto_view.left_win.win);
                    parent_view = None;
                },
            };
            let index : usize = curr_view.as_ref().unwrap().index;
            let dirent : &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap()[index];
            ui::wprint_path(&joshuto_view.top_win, username.as_str(),
                    hostname.as_str(), &dirent.entry.path());
            ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);
            ncurses::doupdate();

        } else if ch == ncurses::KEY_RIGHT || ch == '\n' as i32 {
            if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                continue;
            }

            let index = curr_view.as_ref().unwrap().index;
            let path = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index].entry.path();
            if path.is_dir() {
                match env::set_current_dir(&path) {
                    Ok(_s) => {
                        if let Some(s) = parent_view {
                            history.map.insert(curr_path.clone(), s);
                        }

                        curr_path.push(path.file_name().unwrap().to_str().unwrap());

                        parent_view = curr_view;
                        curr_view = preview_view;
                        preview_view = None;

                        if let Some(s) = parent_view.as_ref() {
                            ui::display_contents(&joshuto_view.left_win, s);
                            ncurses::wnoutrefresh(joshuto_view.left_win.win);
                        }

                        ui::display_contents(&joshuto_view.mid_win,
                                curr_view.as_ref().unwrap());
                        ncurses::wnoutrefresh(joshuto_view.mid_win.win);

                        if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                            ncurses::werase(joshuto_view.right_win.win);
                            ncurses::wnoutrefresh(joshuto_view.right_win.win);
                            ncurses::doupdate();
                            continue;
                        }

                        let index : usize = curr_view.as_ref().unwrap().index;
                        let dirent : &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[index];
                        let new_path = dirent.entry.path();
                        ui::wprint_path(&joshuto_view.top_win,
                                username.as_str(), hostname.as_str(), &new_path);

                        if new_path.is_dir() {
                            preview_view = match history.pop_or_create(new_path.as_path(), &config.sort_type) {
                                Ok(s) => { Some(s) },
                                Err(e) => {
                                    ui::wprint_err(&joshuto_view.right_win,
                                            format!("{}", e).as_str());
                                    None
                                },
                            };
                            if let Some(s) = preview_view.as_ref() {
                                ui::display_contents(&joshuto_view.right_win, &s);
                                ncurses::wnoutrefresh(joshuto_view.right_win.win);
                            }
                            ui::wprint_file_info(joshuto_view.bot_win.win, &dirent.entry);
                        } else {
                            ncurses::werase(joshuto_view.right_win.win);
                            ui::wprint_err(&joshuto_view.right_win, "Not a directory");
                        }
                        ui::wprint_path(&joshuto_view.top_win, username.as_str(), hostname.as_str(),
                                &curr_path);
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
                show_hidden = !show_hidden;
                history.depecrate_all_entries();

                if let Some(s) = curr_view.as_mut() {
                    s.update(&curr_path, &config.sort_type);
                    ui::display_contents(&joshuto_view.mid_win, &s);
                    ncurses::wnoutrefresh(joshuto_view.mid_win.win);
                }

                if let Some(s) = parent_view.as_mut() {
                    if curr_path.parent() != None {
                        s.update(curr_path.parent().unwrap(), &config.sort_type);
                        ui::display_contents(&joshuto_view.left_win, &s);
                        ncurses::wnoutrefresh(joshuto_view.left_win.win);
                    }
                }

                if let Some(s) = preview_view.as_mut() {
                    if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() > 0 {
                        let index : usize = curr_view.as_ref().unwrap().index;
                        let dirent : &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
                                        .contents.as_ref().unwrap()[index];

                        ui::wprint_path(&joshuto_view.top_win, username.as_str(),
                            hostname.as_str(), &dirent.entry.path());

                        s.update(dirent.entry.path().as_path(), &config.sort_type);

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
            let index = curr_view.as_ref().unwrap().index;
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
                            &config.sort_type);
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

            let index = curr_view.as_ref().unwrap().index;
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
            let index = curr_view.as_ref().unwrap().index;
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
