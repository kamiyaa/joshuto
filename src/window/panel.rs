use ncurses;

use crate::structs;
use crate::ui;

const MIN_WIN_WIDTH: usize = 4;

#[derive(Debug, Clone)]
pub struct JoshutoPanel {
    pub win: ncurses::WINDOW,
    pub panel: ncurses::PANEL,
    pub rows: i32,
    pub cols: i32,
    /* coords (y, x) */
    pub coords: (usize, usize),
}

impl std::ops::Drop for JoshutoPanel {
    fn drop(&mut self) {
        ncurses::del_panel(self.panel);
        ncurses::delwin(self.win);
        ncurses::update_panels();
    }
}

impl JoshutoPanel {
    pub fn new(rows: i32, cols: i32, coords: (usize, usize)) -> Self {
        let win = ncurses::newwin(rows, cols, coords.0 as i32, coords.1 as i32);
        let panel = ncurses::new_panel(win);
        ncurses::leaveok(win, true);

        ncurses::wnoutrefresh(win);
        JoshutoPanel {
            win,
            panel,
            rows,
            cols,
            coords,
        }
    }

    pub fn move_to_top(&self) {
        ncurses::top_panel(self.panel);
    }
    pub fn queue_for_refresh(&self) {
        ncurses::wnoutrefresh(self.win);
    }

    pub fn display_contents(&self, dirlist: &mut structs::JoshutoDirList, scroll_offset: usize) {
        if self.non_empty_dir_checks(dirlist, scroll_offset) {
            Self::draw_dir_list(self, dirlist, ui::wprint_entry);
        }
    }

    pub fn display_contents_detailed(
        &self,
        dirlist: &mut structs::JoshutoDirList,
        scroll_offset: usize,
    ) {
        if self.non_empty_dir_checks(dirlist, scroll_offset) {
            Self::draw_dir_list(self, dirlist, ui::wprint_entry_detailed);
        }
    }

    pub fn draw_dir_list(
        win: &JoshutoPanel,
        dirlist: &structs::JoshutoDirList,
        draw_func: fn(&JoshutoPanel, &structs::JoshutoDirEntry, (usize, &str), (i32, i32)),
    ) {
        let dir_contents = &dirlist.contents;
        let (start, end) = (dirlist.pagestate.start, dirlist.pagestate.end);

        let curr_index = dirlist.index.unwrap();

        for i in start..end {
            let coord: (i32, i32) = (i as i32 - start as i32, 0);

            ncurses::wmove(win.win, coord.0, coord.1);
            let entry = &dir_contents[i];

            let mut attr: ncurses::attr_t = 0;
            if i == curr_index {
                attr |= ncurses::A_STANDOUT();
            }
            let attrs = ui::get_theme_attr(attr, entry);

            draw_func(win, entry, attrs.0, coord);

            ncurses::mvwchgat(win.win, coord.0, coord.1, -1, attrs.1, attrs.2);
        }
    }

    fn non_empty_dir_checks(
        &self,
        dirlist: &mut structs::JoshutoDirList,
        scroll_offset: usize,
    ) -> bool {
        if self.cols < MIN_WIN_WIDTH as i32 {
            return false;
        }
        let vec_len = dirlist.contents.len();
        if vec_len == 0 {
            ui::wprint_empty(self, "empty");
            return false;
        }
        ncurses::werase(self.win);

        if let Some(index) = dirlist.index {
            dirlist
                .pagestate
                .update_page_state(index, self.rows, vec_len, scroll_offset);
        }
        ncurses::wmove(self.win, 0, 0);
        true
    }
}
