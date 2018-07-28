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
        curr : &structs::JoshutoDirEntry, preview_view : Option<structs::JoshutoDirEntry>,
        sort_func : fn (&fs::DirEntry, &fs::DirEntry) -> std::cmp::Ordering,
        show_hidden : bool, offset : i32) -> Option<structs::JoshutoDirEntry>
{
    let index : usize = curr.index;
    let old_path = &curr.contents.as_ref().unwrap()[(index as i32 - offset) as usize].path();
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

    let preview : Option<structs::JoshutoDirEntry>;
    if new_path.is_dir() {
        preview = match history::get_or_create(history, new_path.as_path(), sort_func, show_hidden) {
            Ok(s) => { Some(s) },
            Err(e) => {
                ui::wprintmsg(&joshuto_view.right_win, format!("{}", e).as_str());
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
        ui::wprintmsg(&joshuto_view.right_win, "Not a directory");
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

    /* keep track of */
    let mut history : HashMap<String, structs::JoshutoDirEntry>
        = HashMap::new();

    let mut curr_path : path::PathBuf =
        match env::current_dir() {
            Ok(path) => { path },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

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
                match structs::JoshutoDirEntry::new(&parent, sort_func, show_hidden) {
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
    } else {
        preview_view = None;
    }

    ui::wprint_path(&joshuto_view.top_win, username.as_str(), hostname.as_str(),
            &curr_path);

    match parent_view.as_ref() {
        Some(s) => {
            joshuto_view.left_win.display_contents(&s);
            ncurses::wnoutrefresh(joshuto_view.left_win.win);
        },
        None => {},
    };

    joshuto_view.mid_win.display_contents(curr_view.as_ref().unwrap());
    ncurses::wnoutrefresh(joshuto_view.mid_win.win);

    match preview_view.as_ref() {
        Some(s) => {
            joshuto_view.right_win.display_contents(&s);
            ncurses::wnoutrefresh(joshuto_view.right_win.win);
        },
        None => {},
    };

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

            match parent_view.as_ref() {
                Some(s) => {
                    joshuto_view.left_win.display_contents(&s);
                    ncurses::wnoutrefresh(joshuto_view.left_win.win);
                },
                None => {},
            };

            joshuto_view.mid_win.display_contents(curr_view.as_ref().unwrap());
            ncurses::wnoutrefresh(joshuto_view.mid_win.win);

            match preview_view.as_ref() {
                Some(s) => {
                    joshuto_view.right_win.display_contents(&s);
                    ncurses::wnoutrefresh(joshuto_view.right_win.win);
                },
                None => {},
            };

            let index : usize = curr_view.as_ref().unwrap().index;
            let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap()[index];

            ui::wprint_file_info(joshuto_view.bot_win.win, dirent);

            ncurses::doupdate();

        } else if ch == ncurses::KEY_UP {
            if curr_view.as_ref().unwrap().index == 0 {
                continue;
            }
            curr_view.as_mut().unwrap().index =
                    curr_view.as_ref().unwrap().index - 1;

            preview_view = updown(&mut history, &joshuto_view,
                    curr_view.as_ref().unwrap(), preview_view, sort_func,
                    show_hidden, -1);

        } else if ch == ncurses::KEY_DOWN {
            if curr_view.as_ref().unwrap().index + 1 >= curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() {
                continue;
            }
            curr_view.as_mut().unwrap().index =
                curr_view.as_ref().unwrap().index + 1;

            preview_view = updown(&mut history, &joshuto_view,
                    curr_view.as_ref().unwrap(), preview_view, sort_func,
                    show_hidden, 1);

        } else if ch == ncurses::KEY_HOME {
            if curr_view.as_ref().unwrap().index == 0 {
                continue;
            }
            curr_view.as_mut().unwrap().index = 0;

        } else if ch == ncurses::KEY_END {
            if curr_view.as_ref().unwrap().index == curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() - 1 {
                continue;
            }
            curr_view.as_mut().unwrap().index =
                curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() - 1;

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
                    ui::wprintmsg(&joshuto_view.bot_win, format!("{}", e).as_str());
                },
            };
            match curr_path.parent() {
                Some(parent) => {
                    parent_view = match history::get_or_create(&mut history, parent, sort_func, show_hidden) {
                        Ok(s) => { Some(s) },
                        Err(e) => {
                            ui::wprintmsg(&joshuto_view.left_win, format!("{}", e).as_str());
                            None
                        },
                    };
                    joshuto_view.left_win.display_contents(parent_view.as_ref().unwrap());
                    ncurses::wnoutrefresh(joshuto_view.left_win.win);
                },
                None => {
                    ncurses::werase(joshuto_view.left_win.win);
                    ncurses::wnoutrefresh(joshuto_view.left_win.win);
                    parent_view = None;
                },
            };
            ui::wprint_path(&joshuto_view.top_win, username.as_str(), hostname.as_str(),
                    &curr_path);
            let index : usize = curr_view.as_ref().unwrap().index;
            let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                            .contents.as_ref().unwrap()[index];
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

                            ui::wprint_path(&joshuto_view.top_win, username.as_str(), hostname.as_str(),
                                    &curr_path);
                            ncurses::doupdate();
                            continue;
                        }

                        let index : usize = curr_view.as_ref().unwrap().index;

                        let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                                                        .contents.as_ref().unwrap()[index];
                        let new_path = dirent.path();

                        if new_path.is_dir() {
                            preview_view = match history::get_or_create(&mut history, new_path.as_path(), sort_func, show_hidden) {
                                Ok(s) => { Some(s) },
                                Err(e) => {
                                    ui::wprintmsg(&joshuto_view.right_win, format!("{}", e).as_str());
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
                            ui::wprintmsg(&joshuto_view.right_win, "Not a directory");
                        }
                        ui::wprint_path(&joshuto_view.top_win, username.as_str(), hostname.as_str(),
                                &curr_path);
                    }
                    Err(e) => {
                        ui::wprintmsg(&joshuto_view.bot_win, format!("{}", e).as_str());
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
                        ui::wprintmsg(&joshuto_view.right_win, format!("Don't know how to open: {}", mime_type).as_str());
                    }
                } else {
                    ui::wprintmsg(&joshuto_view.right_win, format!("Don't know how to open: {}", mime_type).as_str());
                }
            }
            ncurses::doupdate();
        } else if ch == 'z' as i32 {

            let ch2 : i32 = ncurses::getch();
            if ch2 == 'h' as i32 {
                show_hidden = !show_hidden;
                history::depecrate_all_entries(&mut history);
            }
/*
            curr_view.unwrap().update(&curr_path, sort_func, show_hidden);
            if curr_path.parent() != None {
                parent_view.unwrap().update(curr_path.parent().unwrap(), sort_func, show_hidden);
            }
            if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() > 0 {
                let index : usize = curr_view.as_ref().unwrap().index;
                let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[index];

                preview_view.unwrap().update(dirent.path().as_path(), sort_func, show_hidden);
            }*/
        } else if ch == 330 { // delete button
            if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                continue;
            }
            let index = curr_view.as_ref().unwrap().index;
            let path = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index].path();
            let name = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index].file_name();
            ui::wprintmsg(&joshuto_view.bot_win,
                format!("Delete {:?}? (y/n)", name).as_str());
            ncurses::doupdate();
            let ch2 = ncurses::getch();
            if ch2 == 'y' as i32 {
                std::fs::remove_file(path);
                curr_view.as_mut().unwrap().update(&curr_path, sort_func, show_hidden);
                joshuto_view.mid_win.display_contents(curr_view.as_ref().unwrap());
                ncurses::wnoutrefresh(joshuto_view.mid_win.win);
            }
            ncurses::doupdate();
        } else {
            ui::wprintmsg(&joshuto_view.bot_win,
                format!("Unknown keychar: ({}: {})", ch, ch as u8 as char).as_str());
        }
        // eprintln!("{:?}\n\n", history);
    }
    ncurses::endwin();
}
