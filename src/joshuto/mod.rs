extern crate ncurses;
extern crate whoami;

use std;
use std::env;
use std::fs;
use std::path;
use std::process;
use std::collections::HashMap;

use JoshutoConfig;

mod history;
mod sort;
mod structs;
mod ui;
mod unix;

const QUIT      : i32 = 'q' as i32;
const ENTER     : i32 = '\n' as i32;

pub fn parse_sort_func(sort_method : &Option<String>)
        -> fn(&std::fs::DirEntry, &std::fs::DirEntry) -> std::cmp::Ordering
{
    match sort_method {
        Some(ref method) => {
            if method == "natural" {
                sort::alpha_sort
            } else {
                sort::alpha_sort
            }
        },
        None => {
            sort::alpha_sort
        }
    }
}

pub fn updown(history : &mut HashMap<String, structs::JoshutoDirEntry>,
        joshuto_view : &structs::JoshutoView,
        old_path : &path::Path,
        curr : &structs::JoshutoDirEntry,
        preview_view : Option<structs::JoshutoDirEntry>,
        sort_func : fn (&fs::DirEntry, &fs::DirEntry) -> std::cmp::Ordering,
        show_hidden : bool) -> Option<structs::JoshutoDirEntry>
{
    let index : usize = curr.index;
    let dirent : &fs::DirEntry = &curr.contents.as_ref().unwrap()[index];
    let new_path = dirent.path();

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

    let preview : Option<structs::JoshutoDirEntry>;
    if new_path.is_dir() {
        preview = match history::get_or_create(history, new_path.as_path(), sort_func, show_hidden) {
            Ok(s) => { Some(s) },
            Err(e) => {
                ui::wprint_err(&joshuto_view.right_win, format!("{}", e).as_str());
                ncurses::wnoutrefresh(joshuto_view.right_win.win);
                None
            },
        };
        if let Some(s) = preview.as_ref() {
            joshuto_view.right_win.display_contents(&s);
            ncurses::wnoutrefresh(joshuto_view.right_win.win);
        }
    } else {
        preview = None;
        ncurses::werase(joshuto_view.right_win.win);
        let mime_type = unix::get_mime_type(&dirent);
        ui::wprint_msg(&joshuto_view.right_win, mime_type.as_str());
        ncurses::wnoutrefresh(joshuto_view.right_win.win);
    }

    joshuto_view.mid_win.display_contents(curr);
    ncurses::wnoutrefresh(joshuto_view.mid_win.win);

    ui::wprint_file_info(joshuto_view.bot_win.win, dirent);

    ncurses::doupdate();

    preview
}

pub fn run(config : &mut JoshutoConfig)
{
    ui::init_ncurses();

    let username : String = whoami::username();
    let hostname : String = whoami::hostname();

    /* height, width, y, x */
    let mut joshuto_view : structs::JoshutoView = structs::JoshutoView::new((1, 3, 4));

    /* TODO: mutable in the future */
    let sort_func : fn(&std::fs::DirEntry, &std::fs::DirEntry) -> std::cmp::Ordering
        = parse_sort_func(&config.sort_method);

    let mut show_hidden : bool =
        match config.show_hidden {
            Some(s) => s,
            None => false,
        };

    /* keep track of where we are in directories */
    let mut history : HashMap<String, structs::JoshutoDirEntry>
            = history::init_path_history(sort_func, show_hidden);

    let mut curr_path : path::PathBuf =
        match env::current_dir() {
            Ok(path) => { path },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

    /* load up directories */
    let mut curr_view : Option<structs::JoshutoDirEntry> =
        match structs::JoshutoDirEntry::new(&curr_path.as_path(), sort_func, show_hidden) {
            Ok(s) => { Some(s) },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

    let mut parent_view : Option<structs::JoshutoDirEntry> =
        match curr_path.parent() {
            Some(parent) => {
                match history::get_or_create(&mut history, &parent, sort_func, show_hidden) {
                    Ok(s) => { Some(s) },
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    },
                }
            },
            None => { None },
        };

    let mut preview_view : Option<structs::JoshutoDirEntry>;
    if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() > 0 {
        let index : usize = curr_view.as_ref().unwrap().index;
        let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                        .contents.as_ref().unwrap()[index];
        ui::wprint_file_info(joshuto_view.bot_win.win, dirent);

        let preview_path = dirent.path();
        preview_view = match structs::JoshutoDirEntry::new(&preview_path, sort_func, show_hidden) {
            Ok(s) => { Some(s) },
            Err(e) => {
                eprintln!("{}", e);
                None
            },
        };
        ui::wprint_path(&joshuto_view.top_win, username.as_str(), hostname.as_str(),
                &preview_path);
    } else {
        preview_view = None;
    }


    if let Some(s) = parent_view.as_ref() {
        joshuto_view.left_win.display_contents(&s);
        ncurses::wnoutrefresh(joshuto_view.left_win.win);
    }

    joshuto_view.mid_win.display_contents(curr_view.as_ref().unwrap());
    ncurses::wnoutrefresh(joshuto_view.mid_win.win);

    if let Some(s) = preview_view.as_ref() {
        joshuto_view.right_win.display_contents(&s);
        ncurses::wnoutrefresh(joshuto_view.right_win.win);
    }

    ncurses::doupdate();

    loop {
        let ch : i32 = ncurses::getch();

        if ch == QUIT {
            break;
        }

        if ch == ncurses::KEY_RESIZE {
            joshuto_view.redraw_views();
            ncurses::refresh();

            ui::wprint_path(&joshuto_view.top_win, username.as_str(),
                hostname.as_str(), &curr_path);

            if let Some(s) = parent_view.as_ref() {
                joshuto_view.left_win.display_contents(&s);
                ncurses::wnoutrefresh(joshuto_view.left_win.win);
            }

            joshuto_view.mid_win.display_contents(curr_view.as_ref().unwrap());
            ncurses::wnoutrefresh(joshuto_view.mid_win.win);

            if let Some(s) = preview_view.as_ref() {
                joshuto_view.right_win.display_contents(&s);
                ncurses::wnoutrefresh(joshuto_view.right_win.win);
            }

            let index : usize = curr_view.as_ref().unwrap().index;
            let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap()[index];

            ui::wprint_file_info(joshuto_view.bot_win.win, dirent);

            ncurses::doupdate();

        } else if ch == ncurses::KEY_UP {
            let curr_index = curr_view.as_ref().unwrap().index;
            if curr_index == 0 {
                continue;
            }

            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].path();

            curr_view.as_mut().unwrap().index =
                    curr_view.as_ref().unwrap().index - 1;

            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden);

        } else if ch == ncurses::KEY_DOWN {
            let curr_index = curr_view.as_ref().unwrap().index;
            let dir_len = curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap().len();

            if curr_index + 1 >= dir_len {
                continue;
            }
            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].path();

            curr_view.as_mut().unwrap().index =
                curr_view.as_ref().unwrap().index + 1;

            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden);

        } else if ch == ncurses::KEY_HOME {
            let curr_index = curr_view.as_ref().unwrap().index;

            if curr_index == 0 {
                continue;
            }
            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].path();

            curr_view.as_mut().unwrap().index = 0;

            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden);

        } else if ch == ncurses::KEY_END {
            let curr_index = curr_view.as_ref().unwrap().index;
            let dir_len = curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap().len();

            if curr_index == dir_len - 1 {
                continue;
            }
            let old_path : path::PathBuf = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[curr_index].path();

            curr_view.as_mut().unwrap().index = dir_len - 1;

            preview_view = updown(&mut history, &joshuto_view, &old_path,
                    curr_view.as_ref().unwrap(), preview_view,
                    sort_func, show_hidden);

        } else if ch == ncurses::KEY_LEFT {
            if curr_path.pop() == false {
                continue;
            }

            match env::set_current_dir(curr_path.as_path()) {
                Ok(_s) => {
                    if let Some(s) = preview_view {
                        let index = curr_view.as_ref().unwrap().index;
                        let path = &curr_view.as_ref().unwrap()
                                            .contents.as_ref()
                                            .unwrap()[index].path();

                        let tmp_key : String = format!("{}", path.as_path().to_str().unwrap());
                        history.insert(tmp_key, s);
                    }

                    preview_view = curr_view;
                    curr_view = parent_view;
                    joshuto_view.mid_win.display_contents(
                            curr_view.as_ref().unwrap());
                    ncurses::wnoutrefresh(joshuto_view.mid_win.win);
                    joshuto_view.right_win.display_contents(
                            preview_view.as_ref().unwrap());
                    ncurses::wnoutrefresh(joshuto_view.right_win.win);
                },
                Err(e) => {
                    ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                },
            };
            match curr_path.parent() {
                Some(parent) => {
                    parent_view = match history::get_or_create(&mut history, parent, sort_func, show_hidden) {
                        Ok(s) => { Some(s) },
                        Err(e) => {
                            ui::wprint_err(&joshuto_view.left_win, format!("{}", e).as_str());
                            None
                        },
                    };
                    joshuto_view.left_win.display_contents(
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
            let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap()[index];
            ui::wprint_path(&joshuto_view.top_win, username.as_str(),
                    hostname.as_str(), &dirent.path());
            ui::wprint_file_info(joshuto_view.bot_win.win, dirent);
            ncurses::doupdate();

        } else if ch == ncurses::KEY_RIGHT || ch == ENTER {
            if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                continue;
            }

            let index = curr_view.as_ref().unwrap().index;
            let path = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index].path();
            if path.is_dir() {
                match env::set_current_dir(&path) {
                    Ok(_s) => {
                        if let Some(s) = parent_view {
                            let tmp_key : String = format!("{}", curr_path.as_path().parent().unwrap()
                                    .to_str().unwrap());
                            history.insert(tmp_key, s);
                        }

                        curr_path.push(path.file_name().unwrap().to_str().unwrap());

                        parent_view = curr_view;
                        curr_view = preview_view;
                        preview_view = None;

                        if let Some(s) = parent_view.as_ref() {
                            joshuto_view.left_win.display_contents(s);
                            ncurses::wnoutrefresh(joshuto_view.left_win.win);
                        }

                        joshuto_view.mid_win.display_contents(
                                curr_view.as_ref().unwrap());
                        ncurses::wnoutrefresh(joshuto_view.mid_win.win);

                        if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                            ncurses::werase(joshuto_view.right_win.win);
                            ncurses::wnoutrefresh(joshuto_view.right_win.win);
                            ncurses::doupdate();
                            continue;
                        }

                        let index : usize = curr_view.as_ref().unwrap().index;

                        let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                                                        .contents.as_ref().unwrap()[index];
                        let new_path = dirent.path();
                        ui::wprint_path(&joshuto_view.top_win,
                                username.as_str(), hostname.as_str(), &new_path);

                        if new_path.is_dir() {
                            preview_view = match history::get_or_create(&mut history, new_path.as_path(), sort_func, show_hidden) {
                                Ok(s) => { Some(s) },
                                Err(e) => {
                                    ui::wprint_err(&joshuto_view.right_win, format!("{}", e).as_str());
                                    None
                                },
                            };
                            if let Some(s) = preview_view.as_ref() {
                                joshuto_view.right_win.display_contents(&s);
                                ncurses::wnoutrefresh(joshuto_view.right_win.win);
                            }

                            ui::wprint_file_info(joshuto_view.bot_win.win, dirent);
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
            } else {
                let index : usize = curr_view.as_ref().unwrap().index;
                let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                                                .contents.as_ref().unwrap()[index];
                let mime_type : String =
                    unix::get_mime_type(&dirent);

                /* check if there is a BTreeMap of programs to execute */
                if let Some(mime_map) = &config.mimetypes {
                    if let Some(mime_args) = unix::get_exec_program(mime_type.as_str(), mime_map) {
                        let mime_args_len = mime_args.len();
                        if mime_args_len > 0 {
                            let program_name = mime_args[0].clone();

                            let mut args_list : Vec<String> = Vec::with_capacity(mime_args_len);
                            for i in 1..mime_args_len {
                                args_list.push(mime_args[i].clone());
                            }
                            args_list.push(dirent.file_name().into_string().unwrap());

                            ncurses::savetty();
                            ncurses::endwin();
                            unix::exec_with(program_name, args_list);
                            ncurses::resetty();
                            ncurses::refresh();
                        }
                    } else {
                        ui::wprint_err(&joshuto_view.right_win, format!("Don't know how to open: {}", mime_type).as_str());
                    }
                } else {
                    ui::wprint_err(&joshuto_view.right_win, format!("Don't know how to open: {}", mime_type).as_str());
                }
            }
            ncurses::doupdate();
        } else if ch == 'z' as i32 {
            let ch2 : i32 = ncurses::getch();
            /* toggle show hidden */
            if ch2 == 'h' as i32 {
                show_hidden = !show_hidden;
                history::depecrate_all_entries(&mut history);

                if let Some(s) = curr_view.as_mut() {
                    s.update(&curr_path, sort_func, show_hidden);
                }

                if let Some(s) = parent_view.as_mut() {
                    if curr_path.parent() != None {
                        s.update(curr_path.parent().unwrap(), sort_func, show_hidden);
                    }
                }

                if let Some(s) = preview_view.as_mut() {
                    if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() > 0 {
                        let index : usize = curr_view.as_ref().unwrap().index;
                        let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                                        .contents.as_ref().unwrap()[index];

                        s.update(dirent.path().as_path(), sort_func, show_hidden);
                    }
                }
            }
        } else if ch == 330 { // delete button
            if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                continue;
            }
            let index = curr_view.as_ref().unwrap().index;
            let file_name = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index].file_name();
            ui::wprint_msg(&joshuto_view.bot_win,
                format!("Delete {:?}? (y/n)", file_name).as_str());
            ncurses::doupdate();
            let ch2 = ncurses::wgetch(joshuto_view.bot_win.win);
            if ch2 == 'y' as i32 {
                let path = &curr_view.as_ref().unwrap()
                                    .contents.as_ref()
                                    .unwrap()[index].path();
                match std::fs::remove_file(path) {
                    Ok(_s) => {
                        curr_view.as_mut().unwrap().update(
                            &curr_path, sort_func, show_hidden);
                        joshuto_view.mid_win.display_contents(
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
                let dirent = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index];
                ui::wprint_file_info(joshuto_view.bot_win.win, dirent);
            }
            ncurses::doupdate();
        } else {
            ui::wprint_err(&joshuto_view.bot_win,
                format!("Unknown keychar: ({}: {})", ch, ch as u8 as char).as_str());
        }
        // eprintln!("{:?}\n\n", history);
    }
    ncurses::endwin();
}
