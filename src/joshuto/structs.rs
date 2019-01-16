use std;
use std::fs;
use std::ffi;
use std::path;
use std::process;
use std::time;

use joshuto::config;
use joshuto::history;
use joshuto::sort;
use joshuto::ui;
use joshuto::window;

#[derive(Clone, Debug)]
pub struct JoshutoMetadata {
    pub len: u64,
    pub modified: time::SystemTime,
    pub permissions: fs::Permissions,
    pub file_type: fs::FileType,
}

impl JoshutoMetadata {
    pub fn from(metadata: &fs::Metadata) -> Result<Self, std::io::Error>
    {
        let len = metadata.len();
        let modified = metadata.modified()?;
        let permissions = metadata.permissions();
        let file_type = metadata.file_type();

        Ok(JoshutoMetadata {
            len,
            modified,
            permissions,
            file_type
        })
    }
}

#[derive(Clone, Debug)]
pub struct JoshutoDirEntry {
    pub file_name: ffi::OsString,
    pub file_name_as_string: String,
    pub path: path::PathBuf,
    pub metadata: JoshutoMetadata,
    pub selected: bool,
    pub marked: bool,
}

impl JoshutoDirEntry {

    pub fn from(direntry: &fs::DirEntry) -> Result<Self, std::io::Error>
    {
        let file_name = direntry.file_name();
        let file_name_as_string: String = file_name.clone().into_string().unwrap();
        let path = direntry.path();

        let metadata = direntry.metadata()?;
        let metadata = JoshutoMetadata::from(&metadata)?;

        let dir_entry = JoshutoDirEntry {
            file_name,
            file_name_as_string,
            path,
            metadata,
            selected: false,
            marked: false,
        };
        Ok(dir_entry)
    }

}

#[derive(Debug)]
pub struct JoshutoDirList {
    pub index: i32,
    pub path: path::PathBuf,
    pub update_needed: bool,
    pub metadata: JoshutoMetadata,
    pub contents: Vec<JoshutoDirEntry>,
    pub pagestate: window::JoshutoPageState,
}

impl JoshutoDirList {
    fn read_dir_list(path : &path::Path, sort_type: &sort::SortType)
            -> Result<Vec<JoshutoDirEntry>, std::io::Error>
    {
        let filter_func = sort_type.filter_func();

        let results = fs::read_dir(path)?;
        let result_vec : Vec<JoshutoDirEntry> = results
                .filter_map(filter_func)
                .collect();
        Ok(result_vec)
    }

    pub fn new(path: path::PathBuf, sort_type: &sort::SortType) -> Result<Self, std::io::Error>
    {
        let mut contents = Self::read_dir_list(path.as_path(), sort_type)?;
        contents.sort_by(&sort_type.compare_func());

        let index = if contents.len() > 0 {
                0
            } else {
                -1
            };

        let metadata = fs::metadata(&path)?;
        let metadata = JoshutoMetadata::from(&metadata)?;
        let pagestate = window::JoshutoPageState::new();

        Ok(JoshutoDirList {
            index,
            path,
            update_needed: false,
            metadata,
            contents,
            pagestate,
        })
    }

    pub fn need_update(&self) -> bool
    {
        if self.update_needed {
            return true;
        }
        if let Ok(metadata) = std::fs::metadata(&self.path) {
            if let Ok(modified) = metadata.modified() {
                return self.metadata.modified < modified;
            }
        }
        return true;
    }

    pub fn update_contents(&mut self, sort_type: &sort::SortType) -> Result<(), std::io::Error>
    {
        let sort_func = sort_type.compare_func();
        self.update_needed = false;

        let mut contents = Self::read_dir_list(&self.path, sort_type)?;
        contents.sort_by(&sort_func);

        let contents_len = contents.len() as i32;

        if contents_len == 0 {
            self.index = -1;
        } else if self.index >= contents_len {
            self.index = contents_len - 1;
        } else if self.index >= 0 && self.index < contents_len {
            let index = self.index;
            let curr_file_name = &self.contents[index as usize].file_name;

            for (i, entry) in contents.iter().enumerate() {
                if *curr_file_name == entry.file_name {
                    self.index = i as i32;
                    break;
                }
            }
        } else {
            self.index = 0;
        }
        self.contents = contents;

        let metadata = std::fs::metadata(&self.path)?;
        let metadata = JoshutoMetadata::from(&metadata)?;
        self.metadata = metadata;
        Ok(())
    }

    pub fn get_curr_ref(&self) -> Option<&JoshutoDirEntry>
    {
        self.get_curr_ref_(self.index)
    }

    pub fn get_curr_mut(&mut self) -> Option<&mut JoshutoDirEntry>
    {
        let index = self.index;
        self.get_curr_mut_(index)
    }

    fn get_curr_mut_(&mut self, index: i32) -> Option<&mut JoshutoDirEntry>
    {
        if index >= 0 && (index as usize) < self.contents.len() {
            Some(&mut self.contents[index as usize])
        } else {
            None
        }
    }

    fn get_curr_ref_(&self, index: i32) -> Option<&JoshutoDirEntry>
    {
        if index >= 0 && (index as usize) < self.contents.len() {
            Some(&self.contents[index as usize])
        } else {
            None
        }
    }

    pub fn curr_toggle_select(&mut self)
    {
        let index = self.index;
        self.toggle_select(index);
    }

    fn toggle_select(&mut self, index: i32) {
        if index >= 0 && (index as usize) < self.contents.len() {
            let tmp_bool = !self.contents[index as usize].selected;
            self.contents[index as usize].selected = tmp_bool;
        }
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
        /* keep track of where we are in directories */
        let mut history = history::DirHistory::new();
        history.populate_to_root(&curr_path, sort_type);

        /* load up directories */
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

    pub fn refresh(&mut self, views: &window::JoshutoView,
            theme_t: &config::JoshutoTheme, config_t: &config::JoshutoConfig,
            username: &str, hostname: &str)
    {
        self.refresh_(views, theme_t, config_t.scroll_offset,
                username, hostname);
    }

    pub fn refresh_(&mut self, views: &window::JoshutoView,
            theme_t: &config::JoshutoTheme, scroll_offset: usize,
            username: &str, hostname: &str)
    {
        self.refresh_curr(&views.mid_win, theme_t, scroll_offset);
        self.refresh_parent(&views.left_win, theme_t, scroll_offset);
        self.refresh_file_status(&views.bot_win);
        self.refresh_path_status(&views.top_win, theme_t, username, hostname);
    }

    pub fn refresh_curr(&mut self, win: &window::JoshutoPanel,
            theme_t: &config::JoshutoTheme, scroll_offset: usize)
    {
        if let Some(ref mut s) = self.curr_list {
            win.display_contents_detailed(theme_t, s, scroll_offset);
            ncurses::wnoutrefresh(win.win);
        }
    }

    pub fn refresh_parent(&mut self, win: &window::JoshutoPanel,
            theme_t: &config::JoshutoTheme, scroll_offset: usize)
    {
        if let Some(ref mut s) = self.parent_list {
            win.display_contents(theme_t, s, scroll_offset);
            ncurses::wnoutrefresh(win.win);
        }
    }

    pub fn refresh_file_status(&self, win: &window::JoshutoPanel)
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

    pub fn refresh_path_status(&self, win: &window::JoshutoPanel,
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
