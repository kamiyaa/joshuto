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

    pub fn move_to_top(&self)
    {
        ncurses::top_panel(self.panel);
    }

    pub fn move_to_bottom(&self)
    {
        ncurses::bottom_panel(self.panel);
    }

    pub fn redraw(&mut self, rows: i32, cols: i32, coords: (usize, usize))
    {
        self.destroy();
        self.create(rows, cols, coords);
    }

    pub fn destroy(&self)
    {
        ncurses::del_panel(self.panel);
        ncurses::delwin(self.win);
    }

    fn create(&mut self, rows: i32, cols: i32, coords: (usize, usize))
    {
        self.rows = rows;
        self.cols = cols;
        self.coords = coords;
        self.win = ncurses::newwin(self.rows, self.cols, self.coords.0 as i32,
                self.coords.1 as i32);
        self.panel = ncurses::new_panel(self.win);
        ncurses::leaveok(self.win, true);
        ncurses::wnoutrefresh(self.win);
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

        let win_xy: (i32, i32) = (1, term_cols);
        let win_coord: (usize, usize) = (0, 0);
        let top_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * win_ratio.0 as i32) - 2);
        let win_coord: (usize, usize) = (1, 0);
        let left_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * win_ratio.1 as i32) - 2);
        let win_coord: (usize, usize) = (1, term_divide as usize * win_ratio.0);
        let mid_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * win_ratio.2 as i32) - 2);
        let win_coord: (usize, usize) = (1, term_divide as usize * win_ratio.2);
        let right_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (1, term_cols);
        let win_coord: (usize, usize) = (term_rows as usize - 1, 0);
        let bot_win = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);

/*
        let load_bar = JoshutoPanel::new(win_xy.0, win_xy.1, win_coord);
        load_bar.move_to_bottom();
*/

        ncurses::refresh();

        JoshutoView {
            top_win,
            left_win,
            mid_win,
            right_win,
            bot_win,
            win_ratio,
        }
    }

    pub fn redraw_views(&mut self) {
        let sum_ratio: usize = self.win_ratio.0 + self.win_ratio.1 + self.win_ratio.2;

        let mut term_rows : i32 = 0;
        let mut term_cols : i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);
        let term_divide: i32 = term_cols / sum_ratio as i32;

        let win_xy: (i32, i32) = (1, term_cols);
        let win_coord: (usize, usize) = (0, 0);
        self.top_win.redraw(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * self.win_ratio.0 as i32) - 2);
        let win_coord: (usize, usize) = (1, 0);
        self.left_win.redraw(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * self.win_ratio.1 as i32) - 2);
        let win_coord: (usize, usize) = (1, term_divide as usize * self.win_ratio.0);
        self.mid_win.redraw(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * self.win_ratio.2 as i32) - 2);
        let win_coord: (usize, usize) = (1, term_divide as usize * self.win_ratio.2);
        self.right_win.redraw(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (1, term_cols);
        let win_coord: (usize, usize) = (term_rows as usize - 1, 0);
        self.bot_win.redraw(win_xy.0, win_xy.1, win_coord);

/*
        self.load_bar.redraw(win_xy.0, win_xy.1, win_coord);
        self.load_bar.move_to_bottom();
*/
    }
}
