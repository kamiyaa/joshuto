use std::path::PathBuf;

use crate::config;
use crate::history::{DirectoryHistory, JoshutoHistory};
use crate::preview;
use crate::sort;
use crate::structs::JoshutoDirList;
use crate::ui;
use crate::window::{JoshutoPanel, JoshutoView};

use crate::{HOSTNAME, USERNAME};

use crate::THEME_T;

pub struct JoshutoTab {
    pub history: JoshutoHistory,
    pub curr_path: PathBuf,
    pub curr_list: JoshutoDirList,
}

impl JoshutoTab {
    pub fn new(curr_path: PathBuf, sort_option: &sort::SortOption) -> Result<Self, std::io::Error> {
        let mut history = JoshutoHistory::new();
        history.populate_to_root(&curr_path, sort_option);

        let curr_list = history.pop_or_create(&curr_path, sort_option)?;

        let tab = JoshutoTab {
            curr_path,
            history,
            curr_list,
        };
        Ok(tab)
    }

    pub fn refresh(&mut self, views: &JoshutoView, config_t: &config::JoshutoConfig) {
        self.refresh_curr(&views.mid_win);
        self.refresh_parent(&views.left_win, config_t);
        self.refresh_preview(&views.right_win, config_t);
        self.refresh_path_status(&views.top_win, config_t.tilde_in_titlebar);
        self.refresh_file_status(&views.bot_win);
    }

    pub fn refresh_curr(&self, win: &JoshutoPanel) {
        ui::display_contents(win, &self.curr_list, &ui::PRIMARY_DISPLAY_OPTION);
        win.queue_for_refresh();
    }

    pub fn refresh_parent(&mut self, win: &JoshutoPanel, config_t: &config::JoshutoConfig) {
        preview::preview_parent(self, win, config_t);
    }

    pub fn refresh_preview(&mut self, win: &JoshutoPanel, config_t: &config::JoshutoConfig) {
        if config_t.show_preview {
            preview::preview_entry(self, win, config_t);
        }
    }

    pub fn refresh_file_status(&self, win: &JoshutoPanel) {
        ncurses::werase(win.win);
        ncurses::wmove(win.win, 0, 0);

        if let Some(entry) = self.curr_list.get_curr_ref() {
            ui::wprint_file_mode(win.win, entry);
            ncurses::waddstr(win.win, " ");
            if let Some(index) = self.curr_list.index {
                ncurses::waddstr(
                    win.win,
                    format!("{}/{} ", index + 1, self.curr_list.contents.len()).as_str(),
                );
            }
            ncurses::waddstr(win.win, "  ");
            ui::wprint_file_info(win.win, entry);
        }
        win.queue_for_refresh();
    }

    pub fn refresh_path_status(&self, win: &JoshutoPanel, tilde_in_titlebar: bool) {
        let path_str: &str = self.curr_path.to_str().unwrap();

        ncurses::werase(win.win);
        ncurses::wattron(win.win, ncurses::A_BOLD());
        ncurses::mvwaddstr(win.win, 0, 0, (*USERNAME).as_str());
        ncurses::waddstr(win.win, "@");
        ncurses::waddstr(win.win, (*HOSTNAME).as_str());

        ncurses::waddstr(win.win, " ");

        ncurses::wattron(win.win, ncurses::COLOR_PAIR(THEME_T.directory.colorpair));
        if tilde_in_titlebar {
            let path_str = &path_str.replace(&format!("/home/{}", (*USERNAME).as_str()), "~");
            ncurses::waddstr(win.win, path_str);
        } else {
            ncurses::waddstr(win.win, path_str);
        }
        ncurses::waddstr(win.win, "/");
        ncurses::wattroff(win.win, ncurses::COLOR_PAIR(THEME_T.directory.colorpair));
        if let Some(entry) = self.curr_list.get_curr_ref() {
            ncurses::waddstr(win.win, &entry.file_name_as_string);
        }
        ncurses::wattroff(win.win, ncurses::A_BOLD());
        win.queue_for_refresh();
    }
}
