#[allow(dead_code)]
extern crate ncurses;

use std;
use std::env;
use std::fs;
use std::path;
use std::process;
use std::collections::HashMap;

pub mod config;
pub mod keymap;
pub mod mimetype;
mod history;
mod navigation;
mod sort;
mod structs;
mod ui;
mod unix;
mod window;

mod keymapll;

use self::keymapll::JoshutoCommand;
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

    let mimetype_len = mimetypes.len() as i32;

    let pathbuf = direntry.path();
    let mimetype = unix::get_mime_type(pathbuf.as_path());

    let mut win = window::JoshutoPanel::new(mimetype_len + 1, term_cols,
            ((term_rows - mimetype_len - 2) as usize, 0));

    let mut display_vec: Vec<String> = Vec::new();
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

    display_vec.reserve(mimetype_options.len());
    for (i, val) in mimetype_options.iter().enumerate() {
        display_vec.push(format!("  {}\t{}", i+1, val.join(" ")));
    }
    display_vec.sort();

    win.move_to_top();
    ui::display_options(&win, &display_vec);
    ncurses::doupdate();

    let mut user_input: String = String::new();
    ncurses::echo();
    ncurses::wmove(win.win, mimetype_len + 1, 0);
    ncurses::printw(":open_with ");
    ncurses::wgetstr(win.win, &mut user_input);
    eprintln!("{}", user_input);

    win.destroy();
    ncurses::update_panels();
    ncurses::doupdate();
/*
    let index = ch - '1' as i32;
    if index >= 0 && index < mimetype_options.len() as i32 {
        ncurses::savetty();
        ncurses::endwin();
        unix::open_with(pathbuf.as_path(), &mimetype_options[index as usize]);
        ncurses::resetty();
        ncurses::refresh();
    }*/
}
/*

fn refresh_view(joshuto_view : &window::JoshutoView,
        parent_view: Option<&structs::JoshutoDirList>,
        curr_view: Option<&structs::JoshutoDirList>,
        preview_view: Option<&structs::JoshutoDirList>,
        config_t: &config::JoshutoConfig,

        )
{
    if let Some(s) = parent_view {
        s.update(
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
*/

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

    /* keep track of where we are in directories */
    let mut history = history::History::new();
    history.populate_to_root(&curr_path, &config_t.sort_type);

    ui::init_ncurses();
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
        match s.get_dir_entry(s.index) {
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
                    },
                    Err(e) => {
                        ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                    },
                };

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);

                ncurses::doupdate();
            },
            JoshutoCommand::ChangeDirectory(ref s) => {
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
                    match s.get_dir_entry(s.index) {
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
            JoshutoCommand::DeleteFiles => {
                let index = curr_view.as_ref().unwrap().index;
                if index < 0 || curr_view.as_ref().unwrap().contents.as_ref().unwrap().len() == 0 {
                    continue;
                }
                let index = index as usize;
                let file_name = &curr_view.as_ref()
                                    .unwrap().contents.as_ref()
                                    .unwrap()[index].entry.file_name().into_string()
                                    .unwrap();

                ui::wprint_msg(&joshuto_view.bot_win,
                    format!("Delete {}? (y/n)", file_name).as_str());
                ncurses::doupdate();
                let ch2 = ncurses::wgetch(joshuto_view.bot_win.win);
                if ch2 == 'y' as i32 {
                    let path = &curr_view.as_ref().unwrap()
                                        .contents.as_ref()
                                        .unwrap()[index].entry.path();
                    match std::fs::remove_file(path) {
                        Ok(_s) => {
                            ui::wprint_msg(&joshuto_view.bot_win,
                                format!("Deleted {:?}!", file_name).as_str());
                            if let Some(s) = curr_view.as_mut() {
                                s.update(&config_t.sort_type);
                            }
                            if let Some(s) = preview_view.as_mut() {
                                s.update(&config_t.sort_type);
                            }
                            ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
                            ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());
                        },
                        Err(e) => {
                            ui::wprint_err(&joshuto_view.bot_win,
                                format!("{}", e).as_str());
                        }
                    }
                }

                ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                        &config_t.username, &config_t.hostname);
                ncurses::doupdate();
            },
            JoshutoCommand::RenameFile => {

            },
            JoshutoCommand::CutFiles => {

            },
            JoshutoCommand::CopyFiles => {

            },
            JoshutoCommand::PasteFiles => {

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
                            ui::wprint_err(&joshuto_view.bot_win, format!("{}", e).as_str());
                        }
                    };
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
