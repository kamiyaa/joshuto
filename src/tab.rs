use std::collections::HashMap;
use std::path::PathBuf;

use crate::config;
use crate::history::DirectoryHistory;
use crate::sort;
use crate::structs::JoshutoDirList;
use crate::ui;
use crate::window::{JoshutoPanel, JoshutoView};

use crate::THEME_T;

pub struct JoshutoTab {
    pub history: HashMap<PathBuf, JoshutoDirList>,
    pub curr_path: PathBuf,
    pub curr_list: JoshutoDirList,
}

impl JoshutoTab {
    pub fn new(curr_path: PathBuf, sort_option: &sort::SortOption) -> Result<Self, std::io::Error> {
        let mut history = HashMap::new();
        history.populate_to_root(&curr_path, sort_option);

        let curr_list = history.pop_or_create(&curr_path, sort_option)?;

        let tab = JoshutoTab {
            curr_path,
            history,
            curr_list,
        };
        Ok(tab)
    }

    pub fn reload_contents(
        &mut self,
        sort_option: &sort::SortOption,
    ) -> Result<(), std::io::Error> {
        if self.curr_list.path.exists() {
            self.curr_list.update_contents(sort_option)?;
        }
        Ok(())
    }

    pub fn refresh(
        &mut self,
        views: &JoshutoView,
        config_t: &config::JoshutoConfig,
        username: &str,
        hostname: &str,
    ) {
        self.refresh_curr(&views.mid_win, config_t.scroll_offset);
        self.refresh_parent(&views.left_win, config_t);
        self.refresh_path_status(
            &views.top_win,
            username,
            hostname,
            config_t.tilde_in_titlebar,
        );
        self.refresh_file_status(&views.bot_win);
    }

    pub fn refresh_curr(&mut self, win: &JoshutoPanel, scroll_offset: usize) {
        win.display_contents_detailed(&mut self.curr_list, scroll_offset);
        win.queue_for_refresh();
    }

    pub fn refresh_parent(&mut self, win: &JoshutoPanel, config_t: &config::JoshutoConfig) {
        if let Some(parent) = self.curr_list.path.parent() {
            if let Ok(parent_list) = self
                .history
                .get_mut_or_create(&parent, &config_t.sort_option)
            {
                win.display_contents_detailed(parent_list, config_t.scroll_offset);
                win.queue_for_refresh();
            }
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

    pub fn refresh_path_status(
        &self,
        win: &JoshutoPanel,
        username: &str,
        hostname: &str,
        tilde_in_titlebar: bool,
    ) {
        let path_str: &str = self.curr_path.to_str().unwrap();

        ncurses::werase(win.win);
        ncurses::wattron(win.win, ncurses::A_BOLD());
        ncurses::mvwaddstr(win.win, 0, 0, username);
        ncurses::waddstr(win.win, "@");
        ncurses::waddstr(win.win, hostname);

        ncurses::waddstr(win.win, " ");

        ncurses::wattron(win.win, ncurses::COLOR_PAIR(THEME_T.directory.colorpair));
        if tilde_in_titlebar {
            let path_str = &path_str.replace(&format!("/home/{}", username), "~");
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
