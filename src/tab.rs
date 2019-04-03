use std::path::PathBuf;

use crate::config;
use crate::history;
use crate::sort;
use crate::structs::JoshutoDirList;
use crate::ui;
use crate::window::{JoshutoPanel, JoshutoView};

use crate::THEME_T;

pub struct JoshutoTab {
    pub history: history::DirHistory,
    pub curr_path: PathBuf,
    pub parent_list: Option<JoshutoDirList>,
    pub curr_list: Option<JoshutoDirList>,
}

impl JoshutoTab {
    pub fn new(curr_path: PathBuf, sort_option: &sort::SortOption) -> Result<Self, std::io::Error> {
        let mut history = history::DirHistory::new();
        history.populate_to_root(&curr_path, sort_option);

        let curr_list: Option<JoshutoDirList> = Some(history.pop_or_create(&curr_path, sort_option)?);

        let parent_list: Option<JoshutoDirList> = match curr_path.parent() {
            Some(parent) => Some(history.pop_or_create(&parent, sort_option)?),
            None => None,
        };

        let tab = JoshutoTab {
            curr_path,
            history,
            curr_list,
            parent_list,
        };
        Ok(tab)
    }

    pub fn reload_contents(&mut self, sort_option: &sort::SortOption) {
        let mut list = self.curr_list.take();
        if let Some(ref mut s) = list {
            if s.path.exists() {
                s.update_contents(sort_option).unwrap();
            }
        };
        self.curr_list = list;

        list = self.parent_list.take();
        if let Some(ref mut s) = list {
            if s.path.exists() {
                s.update_contents(sort_option).unwrap();
            }
        };
        self.parent_list = list;
    }

    pub fn refresh(
        &mut self,
        views: &JoshutoView,
        config_t: &config::JoshutoConfig,
        username: &str,
        hostname: &str,
    ) {
        self.refresh_(
            views,
            config_t.tilde_in_titlebar,
            config_t.scroll_offset,
            username,
            hostname,
        );
    }

    pub fn refresh_(
        &mut self,
        views: &JoshutoView,
        tilde_in_titlebar: bool,
        scroll_offset: usize,
        username: &str,
        hostname: &str,
    ) {
        self.refresh_curr(&views.mid_win, scroll_offset);
        self.refresh_parent(&views.left_win, scroll_offset);
        self.refresh_path_status(&views.top_win, username, hostname, tilde_in_titlebar);
        self.refresh_file_status(&views.bot_win);
    }

    pub fn refresh_curr(&mut self, win: &JoshutoPanel, scroll_offset: usize) {
        if let Some(ref mut s) = self.curr_list {
            win.display_contents_detailed(s, scroll_offset);
            win.queue_for_refresh();
        }
    }

    pub fn refresh_parent(&mut self, win: &JoshutoPanel, scroll_offset: usize) {
        if let Some(ref mut s) = self.parent_list {
            win.display_contents(s, scroll_offset);
            win.queue_for_refresh();
        }
    }

    pub fn refresh_file_status(&self, win: &JoshutoPanel) {
        if let Some(ref dirlist) = self.curr_list {
            ncurses::werase(win.win);
            ncurses::wmove(win.win, 0, 0);

            if let Some(entry) = dirlist.get_curr_ref() {
                ui::wprint_file_mode(win.win, entry);
                ncurses::waddstr(win.win, " ");
                if let Some(index) = dirlist.index {
                    ncurses::waddstr(
                        win.win,
                        format!("{}/{} ", index + 1, dirlist.contents.len()).as_str(),
                    );
                }
                ncurses::waddstr(win.win, "  ");
                ui::wprint_file_info(win.win, entry);
            }
            ncurses::wnoutrefresh(win.win);
        }
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
        if let Some(ref dirlist) = self.curr_list {
            if let Some(entry) = dirlist.get_curr_ref() {
                ncurses::waddstr(win.win, &entry.file_name_as_string);
            }
        }
        ncurses::wattroff(win.win, ncurses::A_BOLD());
        win.queue_for_refresh();
    }
}
