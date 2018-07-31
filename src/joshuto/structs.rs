extern crate ncurses;

use std;
use std::fs;
use std::path;
use std::time;

use joshuto::sort;

#[derive(Debug)]
pub struct JoshutoDirEntry {
    pub index : usize,
    pub need_update : bool,
    pub modified : time::SystemTime,
    pub contents : Option<Vec<fs::DirEntry>>,
    pub selection : Vec<fs::DirEntry>,
}

impl JoshutoDirEntry {

    pub fn new(path : &path::Path,
            sort_func : fn (&fs::DirEntry, &fs::DirEntry) -> std::cmp::Ordering,
            show_hidden : bool) -> Result<JoshutoDirEntry, std::io::Error>
    {
        let mut dir_contents : Vec<fs::DirEntry> = read_dir_list(path, show_hidden)?;
        dir_contents.sort_by(&sort_func);

        let modified = std::fs::metadata(&path)?.modified()?;

        Ok(JoshutoDirEntry {
            index: 0,
            need_update : false,
            modified: modified,
            contents: Some(dir_contents),
            selection: Vec::new(),
        })
    }

    pub fn update(&mut self, path : &path::Path,
        sort_func : fn (&fs::DirEntry, &fs::DirEntry) -> std::cmp::Ordering,
        show_hidden : bool)
    {
        self.need_update = false;

        if let Ok(mut dir_contents) = read_dir_list(path, show_hidden) {
            dir_contents.sort_by(&sort_func);
            self.contents = Some(dir_contents);
            if self.index >= self.contents.as_ref().unwrap().len() {
                if self.contents.as_ref().unwrap().len() > 0 {
                    self.index = self.contents.as_ref().unwrap().len() - 1;
                }
            }
        }

        if let Ok(metadata) = std::fs::metadata(&path) {
            match metadata.modified() {
                Ok(s) => { self.modified = s; },
                Err(e) => { eprintln!("{}", e); },
            };
        }
    }
}

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

        ncurses::refresh();
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
        let mut term_rows : i32 = 0;
        let mut term_cols : i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let term_divide : usize = term_cols as usize / 7;
        let top_win = JoshutoWindow::new(1, term_cols, (0, 0));

        let left_win = JoshutoWindow::new(term_rows - 2,
            (term_divide * win_ratio.0) as i32, (1, 0));

        let mid_win = JoshutoWindow::new(term_rows - 2,
            (term_divide * win_ratio.1) as i32,
            (1, term_divide * win_ratio.0));

        let right_win = JoshutoWindow::new(term_rows - 2,
            term_divide as i32 * 3, (1, term_divide * win_ratio.2));
        let bot_win = JoshutoWindow::new(1, term_cols,
                (term_rows as usize - 1, 0));

        ncurses::leaveok(top_win.win, true);
        ncurses::leaveok(left_win.win, true);
        ncurses::leaveok(mid_win.win, true);
        ncurses::leaveok(right_win.win, true);

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
        let mut term_rows : i32 = 0;
        let mut term_cols : i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let term_divide : usize = term_cols as usize / 7;

        self.top_win.redraw(1, term_cols, (0, 0));
        ncurses::scrollok(self.top_win.win, true);

        self.left_win.redraw(term_rows - 2,
            (term_divide * self.win_ratio.0) as i32, (1, 0));

        self.mid_win.redraw(term_rows - 2,
            (term_divide * self.win_ratio.1) as i32,
            (1, term_divide * self.win_ratio.0));

        self.right_win.redraw(term_rows - 2,
            term_divide as i32 * 3,
            (1, term_divide * self.win_ratio.2));
        self.bot_win.redraw(1, term_cols, (term_rows as usize - 1, 0));
    }
}


fn list_dirent(path : &path::Path) -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    match fs::read_dir(path) {
        Ok(results) => {
            let mut result_vec : Vec<fs::DirEntry> = results
                    .filter_map(sort::filter_func_hidden_files)
                    .collect();
            Ok(result_vec)
        },
        Err(e) => {
            Err(e)
        },
    }
}

fn list_dirent_hidden(path : &path::Path) -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    match fs::read_dir(path) {
        Ok(results) => {
            let results : Result<Vec<fs::DirEntry>, _> = results.collect();
            results
        },
        Err(e) => {
            Err(e)
        },
    }
}

fn read_dir_list(path : &path::Path, show_hidden : bool) -> Result<Vec<fs::DirEntry>, std::io::Error>
{
    if show_hidden {
        list_dirent_hidden(path)
    } else {
        list_dirent(path)
    }
}
