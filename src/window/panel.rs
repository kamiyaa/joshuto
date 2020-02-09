#[derive(Clone, Debug)]
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
}
