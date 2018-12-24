extern crate ncurses;

#[derive(Debug)]
pub struct JoshutoWindow {
    pub win     : ncurses::WINDOW,
    pub rows    : i32,
    pub cols    : i32,
    pub coords  : (usize, usize)
}

impl JoshutoWindow {
    pub fn new(rows : i32, cols : i32, coords : (usize, usize)) -> JoshutoWindow
    {
        let win = ncurses::newwin(rows, cols, coords.0 as i32, coords.1 as i32);
        ncurses::leaveok(win, true);

        ncurses::wnoutrefresh(win);
        JoshutoWindow {
            win: win,
            rows: rows,
            cols: cols,
            coords: coords,
        }
    }

    pub fn redraw(&mut self, rows : i32, cols : i32, coords : (usize, usize))
    {
        ncurses::delwin(self.win);
        self.rows = rows;
        self.cols = cols;
        self.coords = coords;
        self.win = ncurses::newwin(self.rows, self.cols, self.coords.0 as i32,
                self.coords.1 as i32);
        ncurses::leaveok(self.win, true);
        ncurses::wnoutrefresh(self.win);
    }
}

pub struct JoshutoPanel {
    pub win: ncurses::WINDOW,
    pub panel: ncurses::PANEL,
    pub rows: i32,
    pub cols: i32,
    pub coords: (usize, usize)
}

impl JoshutoPanel {
    pub fn new(rows : i32, cols : i32, coords : (usize, usize)) -> JoshutoWindow
    {
        let win = ncurses::newwin(rows, cols, coords.0 as i32, coords.1 as i32);
        let panel = ncurses::panel::new_panel(win);
        ncurses::leaveok(win, true);

        ncurses::wnoutrefresh(win);
        JoshutoWindow {
            win,
            panel,
            rows,
            cols,
            coords,
        }
    }

    pub fn redraw(&mut self, rows : i32, cols : i32, coords : (usize, usize))
    {
        ncurses::delwin(self.win);
        self.rows = rows;
        self.cols = cols;
        self.coords = coords;
        self.win = ncurses::newwin(self.rows, self.cols, self.coords.0 as i32,
                self.coords.1 as i32);
        ncurses::leaveok(self.win, true);
        ncurses::wnoutrefresh(self.win);
    }
}

#[derive(Debug)]
pub struct JoshutoView {
    pub top_win : JoshutoWindow,
    pub left_win : JoshutoWindow,
    pub mid_win : JoshutoWindow,
    pub right_win : JoshutoWindow,
    pub bot_win : JoshutoWindow,
    pub win_ratio : (usize, usize, usize),
}

impl JoshutoView {
    pub fn new(win_ratio : (usize, usize, usize)) -> JoshutoView
    {
        let sum_ratio: usize = win_ratio.0 + win_ratio.1 + win_ratio.2;

        let mut term_rows: i32 = 0;
        let mut term_cols: i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);
        let term_divide: i32 = term_cols / sum_ratio as i32;

        let win_xy: (i32, i32) = (1, term_cols);
        let win_coord: (usize, usize) = (0, 0);
        let top_win = JoshutoWindow::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * win_ratio.0 as i32) - 2);
        let win_coord: (usize, usize) = (1, 0);
        let left_win = JoshutoWindow::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * win_ratio.1 as i32) - 2);
        let win_coord: (usize, usize) = (1, term_divide as usize * win_ratio.0);
        let mid_win = JoshutoWindow::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (term_rows - 2, (term_divide * win_ratio.2 as i32) - 2);
        let win_coord: (usize, usize) = (1, term_divide as usize * win_ratio.2);
        let right_win = JoshutoWindow::new(win_xy.0, win_xy.1, win_coord);

        let win_xy: (i32, i32) = (1, term_cols);
        let win_coord: (usize, usize) = (term_rows as usize - 1, 0);
        let bot_win = JoshutoWindow::new(win_xy.0, win_xy.1, win_coord);

        ncurses::scrollok(top_win.win, true);
        ncurses::scrollok(bot_win.win, true);

/*
        ncurses::scrollok(top_win.win, true);
        ncurses::scrollok(left_win.win, true);
        ncurses::scrollok(mid_win.win, true);
        ncurses::scrollok(right_win.win, true);
        ncurses::idlok(left_win.win, true);
        ncurses::idlok(mid_win.win, true);
        ncurses::idlok(right_win.win, true); */

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

        ncurses::scrollok(self.top_win.win, true);
        ncurses::scrollok(self.bot_win.win, true);
    }
}
