use std::collections::{hash_map::Entry, HashMap};
use std::io::BufRead;
use std::path;
use std::process;

use crate::config::JoshutoConfig;
use crate::structs::JoshutoDirList;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::window::panel::JoshutoPanel;
use crate::MIMETYPE_T;

pub fn preview_parent(curr_tab: &mut JoshutoTab, win: &JoshutoPanel, config_t: &JoshutoConfig) {
    if let Some(path) = curr_tab.curr_path.parent() {
        preview_directory(&mut curr_tab.history, path, win, config_t);
    }
}

pub fn preview_entry(curr_tab: &mut JoshutoTab, win: &JoshutoPanel, config_t: &JoshutoConfig) {
    ncurses::werase(win.win);
    match curr_tab.curr_list.get_curr_ref() {
        Some(s) => {
            if s.path.is_dir() {
                preview_directory(&mut curr_tab.history, s.path.as_path(), win, config_t);
            } else if s.metadata.len <= config_t.max_preview_size {
                preview_file(s.path.as_path(), win);
            } else {
                ui::wprint_err(win, "File size exceeds max preview size");
            }
        }
        None => {}
    };
    win.queue_for_refresh();
}

fn preview_directory(
    history: &mut HashMap<path::PathBuf, JoshutoDirList>,
    path: &path::Path,
    win: &JoshutoPanel,
    config_t: &JoshutoConfig,
) {
    match history.entry(path.clone().to_path_buf()) {
        Entry::Occupied(mut entry) => {
            win.display_contents(entry.get_mut(), config_t.scroll_offset);
        }
        Entry::Vacant(entry) => {
            if let Ok(s) = JoshutoDirList::new(path.clone().to_path_buf(), &config_t.sort_option) {
                win.display_contents(entry.insert(s), config_t.scroll_offset);
            }
        }
    }
    win.queue_for_refresh();
}

fn preview_file(path: &path::Path, win: &JoshutoPanel) {
    match path.extension() {
        Some(_file_ext) => {
            let mimetype_str = tree_magic::from_filepath(&path);
            /* mime subtype have second priority */
            if let Some(_s) = MIMETYPE_T.mimetype.get(&mimetype_str) {}

            /* generic mime type have last priority */
            if let Some(s) = mimetype_str.find('/') {
                let mimetype_type = &mimetype_str[..s];
                if mimetype_type == "text" {
                    preview_text(path, win);
                }
            }
        }
        None => {
            let mimetype_str = tree_magic::from_filepath(&path);
            /* mime subtype have second priority */
            if let Some(_s) = MIMETYPE_T.mimetype.get(&mimetype_str) {}

            /* generic mime type have last priority */
            if let Some(s) = mimetype_str.find('/') {
                let mimetype_type = &mimetype_str[..s];
                if mimetype_type == "text" {
                    preview_text(path, win);
                }
            }
        }
    }
}

fn preview_text(path: &path::Path, win: &JoshutoPanel) {
    let mut command = process::Command::new("head");
    command.arg("-n");
    command.arg(win.cols.to_string());
    command.arg(path.as_os_str());
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

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
        Err(e) => ui::wprint_err(win, e.to_string().as_str()),
    }
    // bat joshuto.rs --terminal-width 20 --wrap=never --line-range 0:26 --style='numbers'
}
