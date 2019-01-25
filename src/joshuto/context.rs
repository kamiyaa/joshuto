extern crate ncurses;
extern crate whoami;

use std::env;
use std::path;
use std::process;
use std::sync;
use std::thread;

use joshuto::command;
use joshuto::config;
use joshuto::history;
use joshuto::sort;
use joshuto::structs::JoshutoDirList;
use joshuto::ui;
use joshuto::window::JoshutoView;
use joshuto::window::JoshutoPanel;

pub struct JoshutoContext {
    pub username: String,
    pub hostname: String,
    pub threads: Vec<(sync::mpsc::Receiver<command::ProgressInfo>, thread::JoinHandle<i32>)>,
    pub views: JoshutoView,
    pub curr_tab_index: usize,
    pub tabs: Vec<JoshutoTab>,

    pub config_t: config::JoshutoConfig,
    pub mimetype_t: config::JoshutoMimetype,
    pub theme_t: config::JoshutoTheme,
}

impl<'a> JoshutoContext {
    pub fn new(config_t: config::JoshutoConfig,
        mimetype_t: config::JoshutoMimetype,
        theme_t: config::JoshutoTheme) -> Self
    {
        let username: String = whoami::username();
        let hostname: String = whoami::hostname();

        let views: JoshutoView =
            JoshutoView::new(config_t.column_ratio);

        let curr_path: path::PathBuf = env::current_dir().unwrap();

        let tab = JoshutoTab::new(curr_path, &config_t.sort_type);
        let tabs = vec![tab];

        JoshutoContext {
            username,
            hostname,
            threads: Vec::new(),
            views,
            curr_tab_index: 0,
            tabs,
            config_t,
            mimetype_t,
            theme_t
        }
    }
    pub fn curr_tab_ref(&'a self) -> &'a JoshutoTab
    {
        &self.tabs[self.curr_tab_index]
    }
    pub fn curr_tab_mut(&'a mut self) -> &'a mut JoshutoTab
    {
        &mut self.tabs[self.curr_tab_index]
    }
}

pub struct JoshutoTab {
    pub history: history::DirHistory,
    pub curr_path: path::PathBuf,
    pub parent_list: Option<JoshutoDirList>,
    pub curr_list: Option<JoshutoDirList>,
}

impl JoshutoTab {
    pub fn new(curr_path: path::PathBuf, sort_type: &sort::SortType) -> Self
    {
        let mut history = history::DirHistory::new();
        history.populate_to_root(&curr_path, sort_type);

        let curr_list: Option<JoshutoDirList> =
            match history.pop_or_create(&curr_path, sort_type) {
                Ok(s) => { Some(s) },
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                },
            };

        let parent_list: Option<JoshutoDirList> =
            match curr_path.parent() {
                Some(parent) => {
                    match history.pop_or_create(&parent, sort_type) {
                        Ok(s) => { Some(s) },
                        Err(e) => {
                            eprintln!("{}", e);
                            process::exit(1);
                        },
                    }
                },
                None => { None },
            };

        JoshutoTab {
            curr_path,
            history,
            curr_list,
            parent_list,
        }
    }

    pub fn reload_contents(&mut self, sort_type: &sort::SortType)
    {
        let mut gone = false;
        if let Some(s) = self.curr_list.as_mut() {
            if s.path.exists() {
                s.update_contents(sort_type).unwrap();
            } else {
                gone = true;
            }
        }
        if gone {
            self.curr_list = None;
        }

        let mut gone = false;
        if let Some(s) = self.parent_list.as_mut() {
            if s.path.exists() {
                s.update_contents(sort_type).unwrap();
            } else {
                gone = true;
            }
        }
        if gone {
            self.parent_list = None;
        }
    }

    pub fn refresh(&mut self, views: &JoshutoView,
            theme_t: &config::JoshutoTheme, config_t: &config::JoshutoConfig,
            username: &str, hostname: &str)
    {
        self.refresh_(views, theme_t, config_t.scroll_offset,
                username, hostname);
    }

    pub fn refresh_(&mut self, views: &JoshutoView,
            theme_t: &config::JoshutoTheme, scroll_offset: usize,
            username: &str, hostname: &str)
    {
        self.refresh_curr(&views.mid_win, theme_t, scroll_offset);
        self.refresh_parent(&views.left_win, theme_t, scroll_offset);
        self.refresh_path_status(&views.top_win, theme_t, username, hostname);
        self.refresh_file_status(&views.bot_win);
    }

    pub fn refresh_curr(&mut self, win: &JoshutoPanel,
            theme_t: &config::JoshutoTheme, scroll_offset: usize)
    {
        if let Some(ref mut s) = self.curr_list {
            win.display_contents_detailed(theme_t, s, scroll_offset);
            ncurses::wnoutrefresh(win.win);
        }
    }

    pub fn refresh_parent(&mut self, win: &JoshutoPanel,
            theme_t: &config::JoshutoTheme, scroll_offset: usize)
    {
        if let Some(ref mut s) = self.parent_list {
            win.display_contents(theme_t, s, scroll_offset);
            ncurses::wnoutrefresh(win.win);
        }
    }

    pub fn refresh_file_status(&self, win: &JoshutoPanel)
    {
        if let Some(ref dirlist) = self.curr_list {
            ncurses::werase(win.win);
            ncurses::wmove(win.win, 0, 0);

            if let Some(entry) = dirlist.get_curr_ref() {
                ui::wprint_file_mode(win.win, entry);
                ncurses::waddstr(win.win, " ");
                ncurses::waddstr(win.win, format!("{}/{} ", dirlist.index + 1, dirlist.contents.len()).as_str());
                ncurses::waddstr(win.win, "  ");
                ui::wprint_file_info(win.win, entry);
            }
            ncurses::wnoutrefresh(win.win);
        }
    }

    pub fn refresh_path_status(&self, win: &JoshutoPanel,
            theme_t: &config::JoshutoTheme, username: &str, hostname: &str)
    {
        let path_str: &str = match self.curr_path.to_str() {
                Some(s) => s,
                None => "Error",
            };
        ncurses::werase(win.win);
        ncurses::wattron(win.win, ncurses::A_BOLD());
        ncurses::mvwaddstr(win.win, 0, 0, username);
        ncurses::waddstr(win.win, "@");
        ncurses::waddstr(win.win, hostname);

        ncurses::waddstr(win.win, " ");

        ncurses::wattron(win.win, ncurses::COLOR_PAIR(theme_t.directory.colorpair));
        ncurses::waddstr(win.win, path_str);
        ncurses::waddstr(win.win, "/");
        ncurses::wattroff(win.win, ncurses::COLOR_PAIR(theme_t.directory.colorpair));
        if let Some(ref dirlist) = self.curr_list {
            if let Some(entry) = dirlist.get_curr_ref() {
                ncurses::waddstr(win.win, &entry.file_name_as_string);
            }
        }
        ncurses::wattroff(win.win, ncurses::A_BOLD());
        ncurses::wnoutrefresh(win.win);
    }
}
