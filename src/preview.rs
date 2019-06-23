use std::collections::{hash_map::Entry, HashMap};
use std::io::BufRead;
use std::path;
use std::process;

use crate::config::{JoshutoConfig, JoshutoPreviewEntry};
use crate::fs::{JoshutoDirEntry, JoshutoDirList};
use crate::tab::JoshutoTab;
use crate::ui;
use crate::window::panel::JoshutoPanel;
use crate::PREVIEW_T;

pub fn preview_parent(curr_tab: &mut JoshutoTab, win: &JoshutoPanel, config_t: &JoshutoConfig) {
    if let Some(path) = curr_tab.curr_path.parent() {
        preview_directory(&mut curr_tab.history, path, win, config_t);
    } else {
        ncurses::werase(win.win);
        win.queue_for_refresh();
    }
}

pub fn preview_entry(curr_tab: &mut JoshutoTab, win: &JoshutoPanel, config_t: &JoshutoConfig) {
    ncurses::werase(win.win);
    if let Some(s) = curr_tab.curr_list.get_curr_ref() {
        if s.file_path().is_dir() {
            preview_directory(
                &mut curr_tab.history,
                s.file_path().as_path(),
                win,
                config_t,
            );
        } else if s.metadata.file_type.is_file() {
            if s.metadata.len <= config_t.max_preview_size {
                // preview_file(s, win);
            } else {
                ui::wprint_err(win, "File size exceeds max preview size");
            }
        } else {
            ui::wprint_err(win, "Not a regular file");
        }
    }
    win.queue_for_refresh();
}

fn preview_directory(
    history: &mut HashMap<path::PathBuf, JoshutoDirList>,
    path: &path::Path,
    win: &JoshutoPanel,
    config_t: &JoshutoConfig,
) {
    match history.entry(path.to_path_buf().clone()) {
        Entry::Occupied(mut entry) => {
            ui::display_contents(
                win,
                entry.get_mut(),
                config_t,
                &ui::SECONDARY_DISPLAY_OPTION,
            );
        }
        Entry::Vacant(entry) => {
            if let Ok(s) = JoshutoDirList::new(path.to_path_buf().clone(), &config_t.sort_option) {
                ui::display_contents(
                    win,
                    entry.insert(s),
                    config_t,
                    &ui::SECONDARY_DISPLAY_OPTION,
                );
            }
        }
    }
    win.queue_for_refresh();
}

fn preview_file(entry: &JoshutoDirEntry, win: &JoshutoPanel) {
    let path = entry.file_path();
    match path.extension() {
        Some(file_ext) => match PREVIEW_T.extension.get(file_ext.to_str().unwrap()) {
            Some(s) => preview_with(path, win, &s),
            None => {} /*
                                   None => if let Some(mimetype) = tree_magic::from_filepath(&path) {
                                       match PREVIEW_T.mimetype.get(mimetype.as_str()) {
                                           Some(s) => preview_with(path, win, &s),
                                           None => if let Some(ind) = mimetype.find('/') {
                                               let supertype = &mimetype[..ind];
                                               if supertype == "text" {
                                                   preview_text(path, win);
                                               } else if let Some(s) = PREVIEW_T.mimetype.get(supertype) {
                                                   preview_with(path, win, &s);
                                               }
                                           },
                                       }
                                   }
                       */
        },
        None => {} /*
                           if let Some(mimetype) = tree_magic::from_filepath(&path) {
                               match PREVIEW_T.mimetype.get(mimetype.as_str()) {
                                   Some(s) => preview_with(path, win, &s),
                                   None => if let Some(ind) = mimetype.find('/') {
                                       let supertype = &mimetype[..ind];
                                       if supertype == "text" {
                                           preview_text(path, win);
                                       } else if let Some(s) = PREVIEW_T.mimetype.get(supertype) {
                                           preview_with(path, win, &s);
                                       }
                                   },
                               }
                           }
                   */
    }
}

fn preview_with(path: &path::Path, win: &JoshutoPanel, entry: &JoshutoPreviewEntry) {
    let mut command = process::Command::new(&entry.program);
    command
        .args(entry.args.as_ref().unwrap_or(&Vec::new()))
        .arg(path.as_os_str())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    match command.spawn() {
        Ok(child) => {
            if let Some(output) = child.stdout {
                let reader = std::io::BufReader::new(output);

                let mut i = 0;
                for line in reader.lines() {
                    if let Ok(line) = line {
                        ncurses::mvwaddnstr(win.win, i, 0, &line, win.cols);
                        i += 1;
                    }
                    if i == win.rows {
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            ui::wprint_err(win, e.to_string().as_str());
        }
    }
}

fn preview_text(path: &path::Path, win: &JoshutoPanel) {
    match std::fs::File::open(path) {
        Err(e) => ui::wprint_err(win, e.to_string().as_str()),
        Ok(f) => {
            let reader = std::io::BufReader::new(f);
            for (i, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    ncurses::mvwaddstr(win.win, i as i32, 0, &line);
                }
                if i >= win.rows as usize {
                    break;
                }
            }
        }
    }
}
