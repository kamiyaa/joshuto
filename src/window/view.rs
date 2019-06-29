use crate::ui;
use crate::window::JoshutoPanel;

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
    pub fn new(win_ratio: (usize, usize, usize)) -> Self {
        let sum_ratio: usize = win_ratio.0 + win_ratio.1 + win_ratio.2;

        let (term_rows, term_cols) = ui::getmaxyx();
        let term_divide: f64 = term_cols as f64 / sum_ratio as f64;

        /* window for tabs */
        let win_rows = 1;
        let win_cols = 10;
        let win_coord: (usize, usize) = (0, term_cols as usize - win_cols as usize);
        let tab_win = JoshutoPanel::new(win_rows, win_cols, win_coord);

        /* rows, cols */
        let win_rows = 1;
        let win_cols = term_cols - tab_win.cols;
        let win_coord: (usize, usize) = (0, 0);
        let top_win = JoshutoPanel::new(win_rows, win_cols, win_coord);

        let offset: i32 = 0;

        let win_rows = term_rows - 2;
        let win_cols = (term_divide * win_ratio.0 as f64) as i32 - 1;
        let win_coord: (usize, usize) = (1, offset as usize);
        let left_win = JoshutoPanel::new(win_rows, win_cols, win_coord);

        let offset = offset + win_cols + 1;

        let win_rows = term_rows - 2;
        let win_cols = (term_divide * win_ratio.1 as f64) as i32 - 1;
        let win_coord: (usize, usize) = (1, offset as usize);
        let mid_win = JoshutoPanel::new(win_rows, win_cols, win_coord);

        let offset = offset + win_cols + 1;

        let win_rows = term_rows - 2;
        let win_cols = term_cols - offset;
        let win_coord: (usize, usize) = (1, offset as usize);
        let right_win = JoshutoPanel::new(win_rows, win_cols, win_coord);

        let win_rows = 1;
        let win_cols = term_cols;
        let win_coord: (usize, usize) = (term_rows as usize - 1, 0);
        let bot_win = JoshutoPanel::new(win_rows, win_cols, win_coord);

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

    pub fn resize_views(&mut self) {
        let new_view = Self::new(self.win_ratio);

        self.top_win = new_view.top_win;
        self.bot_win = new_view.bot_win;
        self.tab_win = new_view.tab_win;
        self.left_win = new_view.left_win;
        self.mid_win = new_view.mid_win;
        self.right_win = new_view.right_win;
    }
}
