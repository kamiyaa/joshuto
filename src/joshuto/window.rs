extern crate ncurses;

#[derive(Debug, Clone)]
pub struct JoshutoPanel {
    pub win: ncurses::WINDOW,
    pub panel: ncurses::PANEL,
    pub rows: i32,
    pub cols: i32,
    /* coords (y, x) */
    pub coords: (usize, usize)
}

impl JoshutoPanel {
    pub fn new(rows : i32, cols : i32, coords : (usize, usize)) -> Self
    {
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

    pub fn move_to_top(&self) { ncurses::top_panel(self.panel); }
    #[allow(dead_code)]
    pub fn move_to_bottom(&self) { ncurses::bottom_panel(self.panel); }

    pub fn destroy(&self)
    {
        ncurses::del_panel(self.panel);
        ncurses::delwin(self.win);
    }
}

/*
impl std::ops::Drop for Joshuto {
    fn drop(&mut self)
    {
        self.destroy();
    }
}
*/

#[derive(Debug)]
pub struct JoshutoView {
    pub top_win: JoshutoPanel,
    pub tab_win: JoshutoPanel,
    pub left_win: JoshutoPanel,
    pub mid_win: JoshutoPanel,
    pub right_win: JoshutoPanel,
    pub bot_win: JoshutoPanel,
    pub win_ratio: (usize, usize, usize),
}

impl JoshutoView {
    pub fn new(win_ratio : (usize, usize, usize)) -> Self
    {
        let sum_ratio: usize = win_ratio.0 + win_ratio.1 + win_ratio.2;

        let mut term_rows: i32 = 0;
        let mut term_cols: i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);
        let term_divide: i32 = term_cols / sum_ratio as i32;

        let win_xy: (i32, i32) = (1, term_cols - 5);
        let win_coord: (usize, usize) = (0, 0);
        let top_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (1, 5);
        let win_coord: (usize, usize) = (0, term_cols as usize - 5);
        let tab_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);


        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * win_ratio.0 as i32) - 1);
        let win_coord: (usize, usize) = (1, 0);
        let left_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * win_ratio.1 as i32) - 1);
        let win_coord: (usize, usize) = (1, term_divide as usize * win_ratio.0);
        let mid_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * win_ratio.2 as i32) - 1);
        let win_coord: (usize, usize) = (1, term_divide as usize * win_ratio.2);
        let right_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);


        let win_xy: (i32, i32) = (1, term_cols);
        let win_coord: (usize, usize) = (term_rows as usize - 1, 0);
        let bot_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);

        JoshutoView {
            top_win,
            tab_win,
            left_win,
            mid_win,
            right_win,
            bot_win,
            win_ratio,
        }
    }

    fn destroy(&self)
    {
        self.top_win.destroy();
        self.bot_win.destroy();
        self.tab_win.destroy();
        self.left_win.destroy();
        self.mid_win.destroy();
        self.right_win.destroy();
    }

    pub fn resize_views(&mut self)
    {
        self.destroy();
        let new_view = Self::new(self.win_ratio);

        self.top_win = new_view.top_win;
        self.bot_win = new_view.bot_win;
        self.tab_win = new_view.tab_win;
        self.left_win = new_view.left_win;
        self.mid_win = new_view.mid_win;
        self.right_win = new_view.right_win;
    }
}
