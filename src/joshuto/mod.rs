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
curr_view.as_mut().unwrap().index = curr_view.as_ref().unwrap().index - 1;

            let index : usize = curr_view.as_ref().unwrap().index;
            let old_path = &curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[index+1].path();
            let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                                            .contents.as_ref().unwrap()[index];
            let new_path = dirent.path();

            if new_path.is_dir() {
                let old_osstr = match old_path.to_str() {
                        Some(s) => s,
                        None => continue,
                    };

                let tmp_key : String = format!("{}", old_osstr);

                match preview_view {
                    Some(s) => {
                        history.insert(tmp_key, s);
                        preview_view = match structs::JoshutoDirEntry::new(&new_path, sort_func, show_hidden) {
                            Ok(s) => { Some(s) },
                            Err(e) => {
                                eprintln!("{}", e);
                                None
                            },
                        };
                    },
                    None => {},
                };

                match preview_view.as_ref() {
                    Some(s) => {
                        joshuto_view.right_win.display_contents(&s);
                        ncurses::wnoutrefresh(joshuto_view.right_win.win);
                    },
                    None => {},
                };
                ui::wprint_file_info(joshuto_view.bot_win.win, dirent);
            } else {
                ncurses::werase(joshuto_view.right_win.win);
                ncurses::wnoutrefresh(joshuto_view.right_win.win);
            }

            joshuto_view.mid_win.display_contents(curr_view.as_ref().unwrap());
            // ncurses::wscrl(joshuto_view.mid_win.win, 1);
            ncurses::wnoutrefresh(joshuto_view.mid_win.win);

            ncurses::doupdate();

        } else if ch == ncurses::KEY_DOWN {
            if curr_view.as_ref().unwrap().index + 1 >= curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() {
                continue;
            }
            curr_view.as_mut().unwrap().index = curr_view.as_ref().unwrap().index + 1;

            let index : usize = curr_view.as_ref().unwrap().index;
            let old_path = &curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap()[index-1].path();
            let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                                            .contents.as_ref().unwrap()[index];
            let new_path = dirent.path();

            if new_path.is_dir() {
                let old_osstr = match old_path.to_str() {
                        Some(s) => s,
                        None => continue,
                    };

                let tmp_key : String = format!("{}", old_osstr);

                match preview_view {
                    Some(s) => {
                        history.insert(tmp_key, s);
                        preview_view = match structs::JoshutoDirEntry::new(&new_path, sort_func, show_hidden) {
                            Ok(s) => { Some(s) },
                            Err(e) => {
                                eprintln!("{}", e);
                                None
                            },
                        };
                    },
                    None => {},
                };

                match preview_view.as_ref() {
                    Some(s) => {
                        joshuto_view.right_win.display_contents(&s);
                        ncurses::wnoutrefresh(joshuto_view.right_win.win);
                    },
                    None => {},
                };
                ui::wprint_file_info(joshuto_view.bot_win.win, dirent);
            } else {
                ncurses::werase(joshuto_view.right_win.win);
                ncurses::wnoutrefresh(joshuto_view.right_win.win);
            }

            joshuto_view.mid_win.display_contents(curr_view.as_ref().unwrap());
            // ncurses::wscrl(joshuto_view.mid_win.win, -1);
            ncurses::wnoutrefresh(joshuto_view.mid_win.win);

            ncurses::doupdate();

        } else if ch == ncurses::KEY_LEFT {
            let tmp_key : String = format!("{}",
                                        curr_path.as_path()
                                                 .to_str().unwrap()
                                    );

            if curr_path.pop() == false {
                continue;
            }

            match env::set_current_dir(curr_path.as_path()) {
                Ok(_s) => {
                    // eprintln!("LEFT Storing: {}, {:?}", tmp_key, curr_view.as_ref().unwrap().contents.as_ref().unwrap());
                    match preview_view {
                        Some(s) => { history.insert(tmp_key, s); },
                        None => {},
                    };

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
                    eprintln!("{}", e);
                },
            };
            match curr_path.parent() {
                Some(parent) => {
                    parent_view = history::get_or_create(&mut history, parent, sort_func, show_hidden);
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
            ncurses::doupdate();

        } else if ch == ncurses::KEY_RIGHT || ch == ENTER {
            let index = curr_view.as_ref().unwrap().index;
            let path = &curr_view.as_ref().unwrap()
                                .contents.as_ref()
                                .unwrap()[index].path();
            if path.is_dir() {
                match env::set_current_dir(&path) {
                    Ok(_s) => {
                        let tmp_key : String = format!("{}", curr_path.as_path().parent().unwrap().to_str().unwrap());

                        curr_path.push(path.file_name().unwrap().to_str().unwrap());

                        history.insert(tmp_key, parent_view.unwrap());
                        parent_view = curr_view;
                        curr_view = preview_view;
                        preview_view = None;

                        joshuto_view.left_win.display_contents(
                            parent_view.as_ref().unwrap());
                        ncurses::wnoutrefresh(joshuto_view.left_win.win);
                        joshuto_view.mid_win.display_contents(
                            curr_view.as_ref().unwrap());
                        ncurses::wnoutrefresh(joshuto_view.mid_win.win);

                        let index : usize = curr_view.as_ref().unwrap().index;
                        let dirent : &fs::DirEntry = &curr_view.as_ref().unwrap()
                                                        .contents.as_ref().unwrap()[index];
                        let new_path = dirent.path();

                        if new_path.is_dir() {
                            preview_view = history::get_or_create(&mut history, new_path.as_path(), sort_func, show_hidden);
                            match preview_view.as_ref() {
                                Some(s) => {
                                    joshuto_view.right_win.display_contents(&s);
                                    ncurses::wnoutrefresh(joshuto_view.right_win.win);
                                },
                                None => {},
                            };
                            ui::wprint_file_info(joshuto_view.bot_win.win, dirent);
                        } else {
                            ncurses::werase(joshuto_view.right_win.win);
                            ncurses::wnoutrefresh(joshuto_view.right_win.win);
                        }
                        ncurses::doupdate();
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                };
            }
        } else {
            eprintln!("Unknown keychar: ({}: {})", ch, ch as u8 as char);
        }
    }
    ncurses::endwin();
}
