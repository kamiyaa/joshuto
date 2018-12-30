#[allow(dead_code)]
extern crate ncurses;

use std;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;
use std::process;
use std::thread;

pub mod config;
pub mod keymap;
pub mod mimetype;
mod command;
mod history;
mod navigation;
mod sort;
mod structs;
mod ui;
mod unix;
mod window;

mod keymapll;

use self::command::JoshutoCommand;
use self::keymapll::Keycode;

fn recurse_get_keycommand<'a>(keymap: &'a HashMap<i32, JoshutoCommand>)
    -> Option<&'a JoshutoCommand>
{
    let mut term_rows: i32 = 0;
    let mut term_cols: i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

    let keymap_len = keymap.len();

    let mut win = window::JoshutoPanel::new(keymap_len as i32 + 1, term_cols,
            ((term_rows - keymap_len as i32 - 2) as usize, 0));

    let mut display_vec: Vec<String> = Vec::with_capacity(keymap_len);
    for (key, val) in keymap {
        display_vec.push(format!("  {}\t{}", *key as u8 as char, val));
    }
    display_vec.sort();

    win.move_to_top();
    ui::display_options(&win, &display_vec);
    ncurses::doupdate();

    let ch: i32 = ncurses::getch();

    win.destroy();
    ncurses::update_panels();
    ncurses::doupdate();

    if ch == Keycode::ESCAPE as i32 {
        None
    } else {
        match keymap.get(&ch) {
            Some(JoshutoCommand::CompositeKeybind(m)) => {
                recurse_get_keycommand(&m)
            },
            Some(s) => {
                Some(s)
            },
            _ => {
                None
            }
        }
    }
}

fn open_with(mimetypes: &HashMap<String, Vec<Vec<String>>>,
        direntry: &fs::DirEntry)
{
    let mut term_rows: i32 = 0;
    let mut term_cols: i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

    let pathbuf = direntry.path();
    let mimetype = unix::get_mime_type(pathbuf.as_path());

    let mut empty_vec: Vec<Vec<String>> = Vec::new();
    let mimetype_options: &Vec<Vec<String>>;
    match mimetypes.get(&mimetype) {
        Some(s) => {
            mimetype_options = s;
        },
        None => {
            mimetype_options = &empty_vec;
        },
    }

    let option_size = mimetype_options.len();
    let mut win = window::JoshutoPanel::new(option_size as i32 + 2, term_cols,
            (term_rows as usize - option_size - 2, 0));

    let mut display_vec: Vec<String> = Vec::with_capacity(option_size);
    for (i, val) in mimetype_options.iter().enumerate() {
        display_vec.push(format!("  {}\t{}", i, val.join(" ")));
    }
    display_vec.sort();

    win.move_to_top();
    ui::display_options(&win, &display_vec);
    // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
    ncurses::doupdate();

    ncurses::wmove(win.win, option_size as i32 + 1, 0);
    ncurses::wprintw(win.win, ":open_with ");

    let mut cur_ind = ":open_with ".len();

    let mut user_input: String = String::new();
    loop {
        ncurses::wprintw(win.win, "_");
        ncurses::wmove(win.win, option_size as i32 + 1, cur_ind as i32);
        let ch: i32 = ncurses::wgetch(win.win);
        if ch == Keycode::ESCAPE as i32 {
            win.destroy();
            ncurses::update_panels();
            ncurses::doupdate();
            return;
        }
        if ch == Keycode::ENTER as i32 {
            break;
        }
        if ch == Keycode::BACKSPACE as i32 || ch == 127 {
            match user_input.pop() {
                Some(_) => {
                    cur_ind = cur_ind - 1;
                    ncurses::mvwdelch(win.win, option_size as i32 + 1, cur_ind as i32);
                },
                None => {},
            }
//            ncurses::wmove(win.win, option_size as i32 + 1, cur_ind as i32);
            continue;
        }
        user_input.push(ch as u8 as char);
        cur_ind = cur_ind + 1;

        ncurses::wprintw(win.win, (ch as u8 as char).to_string().as_str());
    }

    win.destroy();
    // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::update_panels();
    ncurses::doupdate();

    match user_input.parse::<usize>() {
        Ok(s) => {
            if s < mimetype_options.len() {
                ncurses::savetty();
                ncurses::endwin();
                unix::open_with(pathbuf.as_path(), &mimetype_options[s]);
                ncurses::resetty();
                ncurses::refresh();
            }
        }
        Err(_) => {
            let args: Vec<String> = user_input.split_whitespace().map(|x| String::from(x)).collect();
            ncurses::savetty();
            ncurses::endwin();
            unix::open_with(pathbuf.as_path(), &args);
            ncurses::resetty();
            ncurses::refresh();
        }
    }
}


fn update_views(joshuto_view : &window::JoshutoView,
        parent_view: Option<&mut structs::JoshutoDirList>,
        curr_view: Option<&mut structs::JoshutoDirList>,
        preview_view: Option<&mut structs::JoshutoDirList>,
        config_t: &config::JoshutoConfig,
        )
{
    if let Some(s) = parent_view {
        if s.update_needed || s.need_update() {
            s.update(&config_t.sort_type);
            s.display_contents(&joshuto_view.left_win);
            ncurses::wnoutrefresh(joshuto_view.left_win.win);
        }
    }

    if let Some(s) = curr_view {
        if s.update_needed || s.need_update() {
            s.update(&config_t.sort_type);
            s.display_contents(&joshuto_view.mid_win);
            ncurses::wnoutrefresh(joshuto_view.mid_win.win);
        }
    }

    if let Some(s) = preview_view {
        if s.update_needed || s.need_update() {
            s.update(&config_t.sort_type);
            s.display_contents(&joshuto_view.right_win);
            ncurses::wnoutrefresh(joshuto_view.right_win.win);
        }
    }

    ncurses::doupdate();
}

pub fn run(mut config_t: config::JoshutoConfig,
    keymap_t: keymap::JoshutoKeymap,
    mimetype_t: mimetype::JoshutoMimetype)
{
    let mut curr_path : path::PathBuf = match env::current_dir() {
            Ok(path) => { path },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

    ui::init_ncurses();

    ncurses::printw("Loading...");
    /* keep track of where we are in directories */
    let mut history = history::DirHistory::new();
    history.populate_to_root(&curr_path, &config_t.sort_type);

    let mut joshuto_view: window::JoshutoView =
        window::JoshutoView::new(config_t.column_ratio);

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

    let mut preview_view: Option<structs::JoshutoDirList>;
    if let Some(s) = curr_view.as_ref() {
        match s.get_curr_entry() {
            Some(dirent) => {
                let preview_path = dirent.entry.path();
                if preview_path.is_dir() {
                    preview_view = match history.pop_or_create(&preview_path, &config_t.sort_type) {
                        Ok(s) => { Some(s) },
                        Err(e) => {
                            eprintln!("{}", e);
                            None
                        },
                    };
                } else {
                    preview_view = None;
                }
            },
            None => {
                preview_view = None;
            }
        }
    } else {
        preview_view = None
    }

    let mut clipboard = history::FileClipboard::new();

    ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
            &config_t.username, &config_t.hostname);

    ui::redraw_view(&joshuto_view.left_win, parent_view.as_ref());
    ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
    ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

    ncurses::doupdate();

    loop {
        let ch: i32 = ncurses::getch();
        if ch == ncurses::KEY_RESIZE {
            ui::resize_handler(&config_t,
                    &mut joshuto_view, &curr_path,
                    parent_view.as_ref(),
                    curr_view.as_ref(),
                    preview_view.as_ref());
            continue;
        }

        let keycommand: &JoshutoCommand;

        match keymap_t.keymaps.get(&ch) {
            Some(JoshutoCommand::CompositeKeybind(m)) => {
                match recurse_get_keycommand(&m) {
                    Some(s) => {
                        ncurses::update_panels();
                        ncurses::doupdate();
                        keycommand = &s;
                    }
                    None => {
                        ncurses::update_panels();
                        ncurses::doupdate();
                        continue
                    },
                }

            },
            Some(s) => {
                keycommand = &s;
            },
            None => {
                continue;
            }
        }

        match *keycommand {
            JoshutoCommand::Quit => break,
            JoshutoCommand::ReloadDirList => {
                if let Some(s) = curr_view.as_mut() {
                    s.update(&config_t.sort_type);
                }

                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);

                ncurses::doupdate();
            },
            JoshutoCommand::CursorMove(s) => {
                let curr_index = curr_view.as_ref().unwrap().index;
                let dir_len = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap().len() as i32;
                if curr_index as i32 + s <= 0 && curr_index == 0 ||
                        curr_index as i32 + s >= dir_len && curr_index == dir_len - 1 {
                    continue;
                }

                preview_view = match navigation::set_dir_cursor_index(&mut history,
                        curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type,
                        curr_index + s) {
                    Ok(s) => s,
                    Err(e) => {
                        ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                        None
                    },
                };

                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);

                ncurses::doupdate();
            },
            JoshutoCommand::CursorMovePageUp => {
                let curr_index = curr_view.as_ref().unwrap().index as usize;
                if curr_index <= 0 {
                    continue;
                }

                let half_page: i32 = joshuto_view.mid_win.cols / 2;
                let curr_index = if curr_index < half_page as usize {
                    0
                } else {
                    curr_index as i32 - half_page
                };

                preview_view = match navigation::set_dir_cursor_index(&mut history,
                        curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type,
                        curr_index) {
                    Ok(s) => s,
                    Err(e) => {
                        ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                        None
                    },
                };

                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);

                ncurses::doupdate();
            },
            JoshutoCommand::CursorMovePageDown => {
                let curr_index = curr_view.as_ref().unwrap().index as usize;
                let dir_len = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap().len();

                if curr_index == dir_len - 1 {
                    continue;
                }

                let half_page: i32 = joshuto_view.mid_win.cols / 2;
                let curr_index = if curr_index + half_page as usize >= dir_len {
                    (dir_len - 1) as i32
                } else {
                    curr_index as i32 + half_page
                };

                preview_view = match navigation::set_dir_cursor_index(&mut history,
                        curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type,
                        curr_index) {
                    Ok(s) => s,
                    Err(e) => {
                        ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                        None
                    },
                };

                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);

                ncurses::doupdate();
            },
            JoshutoCommand::CursorMoveHome => {
                let curr_index = curr_view.as_ref().unwrap().index;
                if curr_index <= 0 {
                    continue;
                }

                preview_view = match navigation::set_dir_cursor_index(&mut history,
                        curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type, 0) {
                    Ok(s) => s,
                    Err(e) => {
                        ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                        None
                    },
                };

                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);

                ncurses::doupdate();
            },
            JoshutoCommand::CursorMoveEnd => {
                let curr_index = curr_view.as_ref().unwrap().index;
                let dir_len = curr_view.as_ref().unwrap()
                                .contents.as_ref().unwrap().len() as i32;
                if curr_index == dir_len - 1 {
                    continue;
                }

                preview_view = match navigation::set_dir_cursor_index(&mut history,
                        curr_view.as_mut().unwrap(), preview_view, &config_t.sort_type,
                        (dir_len - 1) as i32) {
                    Ok(s) => s,
                    Err(e) => {
                        ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                        None
                    },
                };

                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);

                ncurses::doupdate();
            },
            JoshutoCommand::ParentDirectory => {
                if curr_path.pop() == false {
                    continue;
                }

                match env::set_current_dir(curr_path.as_path()) {
                    Ok(_) => {
                        history.put_back(preview_view);

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
                        ui::redraw_view(&joshuto_view.left_win, parent_view.as_ref());
                        ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                        ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                        ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                                &config_t.username, &config_t.hostname);
                    },
                    Err(e) => {
                        ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                    },
                };

                ncurses::doupdate();
            },
            JoshutoCommand::ChangeDirectory(ref s) => {
                if !s.exists() {
                    ui::wprint_err(&joshuto_view.bot_win, "Error: No such file or directory");
                    ncurses::doupdate();
                    continue;
                }
                curr_path = s.clone();

                history.put_back(parent_view);
                history.put_back(curr_view);
                history.put_back(preview_view);

                curr_view = match history.pop_or_create(&curr_path, &config_t.sort_type) {
                    Ok(s) => { Some(s) },
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    },
                };

                parent_view =
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

                if let Some(s) = curr_view.as_ref() {
                    match s.get_curr_entry() {
                        Some(dirent) => {
                            let preview_path = dirent.entry.path();
                            if preview_path.is_dir() {
                                preview_view = match history.pop_or_create(&preview_path, &config_t.sort_type) {
                                    Ok(s) => { Some(s) },
                                    Err(e) => {
                                        eprintln!("{}", e);
                                        None
                                    },
                                };
                            } else {
                                preview_view = None;
                            }
                        },
                        None => {
                            preview_view = None;
                        }
                    }
                } else {
                    preview_view = None
                }

                ui::redraw_view(&joshuto_view.left_win, parent_view.as_ref());
                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);

                ncurses::doupdate();
            },
            JoshutoCommand::MarkFiles{toggle, all} => {
                if toggle && !all {
                    if let Some(s) = curr_view.as_mut() {
                        s.mark_curr_toggle();
                        let movement = 1;

                        let curr_index = s.index;
                        let dir_len = s.contents.as_ref().unwrap().len() as i32;
                        if curr_index as i32 + movement <= 0 && curr_index == 0 ||
                                curr_index as i32 + movement >= dir_len && curr_index == dir_len - 1 {
                            continue;
                        }

                        preview_view = match navigation::set_dir_cursor_index(&mut history,
                                s, preview_view, &config_t.sort_type,
                                curr_index + movement) {
                            Ok(s) => s,
                            Err(e) => {
                                ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                                None
                            },
                        };
                    }
                }

                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);

                ncurses::doupdate();
            },
            JoshutoCommand::RenameFile => {

            },
            JoshutoCommand::CutFiles => {
                if let Some(s) = curr_view.as_ref() {
                    clipboard.prepare_cut(s);
                }
            },
            JoshutoCommand::CopyFiles => {
                if let Some(s) = curr_view.as_ref() {
                    clipboard.prepare_copy(s);
                }
            },
            JoshutoCommand::PasteFiles(ref options) => {
                let pathclone = curr_path.to_path_buf().clone();
                let options = options.clone();

                let child = thread::spawn(move || {
                    clipboard.paste(pathclone, &options);
                });

                let res = child.join();

                clipboard = history::FileClipboard::new();

                update_views(&joshuto_view, parent_view.as_mut(), curr_view.as_mut(), preview_view.as_mut(), &config_t);
            },
            JoshutoCommand::DeleteFiles => {
                let mut clipboard = history::DeleteClipboard::new();
                clipboard.prepare(curr_view.as_ref().unwrap());

                ui::wprint_msg(&joshuto_view.bot_win,
                    format!("Delete selected files? (Y/n)").as_str());
                ncurses::doupdate();

                let ch = ncurses::wgetch(joshuto_view.bot_win.win);
                if ch == Keycode::LOWER_Y as i32 || ch == Keycode::ENTER as i32 {
                    match clipboard.execute() {
                        Ok(()) => {},
                        Err(e) => {
                            eprintln!("{}", e);
                        },
                    }
                    update_views(&joshuto_view, None.as_mut(), curr_view.as_mut(), preview_view.as_mut(), &config_t);
                }

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);
                ui::wprint_msg(&joshuto_view.bot_win, "Deleted files");
                ncurses::doupdate();
            },
            JoshutoCommand::Open => {
                if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                    continue;
                }

                let index = curr_view.as_ref().unwrap().index as usize;
                let path = &curr_view.as_ref().unwrap()
                                    .contents.as_ref()
                                    .unwrap()[index].entry.path();

                if path.is_file() {
                    unix::open_file(&mimetype_t.mimetypes, &joshuto_view.bot_win, path);
                    continue;
                }

                if path.is_dir() {
                    match env::set_current_dir(&path) {
                        Ok(_) => {
                            history.put_back(parent_view);

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

                            ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                                    &config_t.username, &config_t.hostname);

                            if curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                                ui::redraw_view(&joshuto_view.left_win, parent_view.as_ref());
                                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                                ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                                ncurses::doupdate();
                                continue;
                            }

                            let index: usize = curr_view.as_ref().unwrap().index as usize;
                            let dirent: &structs::JoshutoDirEntry = &curr_view.as_ref().unwrap()
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
                            } else {
                                ncurses::werase(joshuto_view.right_win.win);
                            }

                            ui::redraw_view(&joshuto_view.left_win, parent_view.as_ref());
                            ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                            ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                            ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                                    &config_t.username, &config_t.hostname);

                            ncurses::doupdate();
                        }
                        Err(e) => {
                            ui::wprint_err(&joshuto_view.bot_win, format!("{}: {:?}", e, path).as_str());
                        }
                    }
                }
            },
            JoshutoCommand::OpenWith => {
                if let Some(s) = curr_view.as_ref() {
                    if let Some(entry) = s.get_curr_entry() {
                        open_with(&mimetype_t.mimetypes, &entry.entry);
                    }
                }
            },
            JoshutoCommand::ToggleHiddenFiles => {
                {
                    let opposite = !config_t.sort_type.show_hidden();
                    config_t.sort_type.set_show_hidden(opposite);
                    history.depecrate_all_entries();
                }

                if let Some(s) = curr_view.as_mut() {
                    s.update(&config_t.sort_type);
                }
                if let Some(s) = parent_view.as_mut() {
                    s.update(&config_t.sort_type);
                }
                if let Some(s) = preview_view.as_mut() {
                    s.update(&config_t.sort_type);
                }

                ui::redraw_view(&joshuto_view.left_win, parent_view.as_ref());
                ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

                ncurses::doupdate();
            },
            _ => {
                ui::wprint_err(&joshuto_view.bot_win,
                    format!("Unknown keychar: ({}: {})", ch, ch as u8 as char).as_str());
            },
        }
    }
    ncurses::endwin();
}
