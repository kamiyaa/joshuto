use crate::config::keymap;
use crate::window;

use rustyline::completion::{Candidate, Completer};

struct CompletionTracker {
    pub index: usize,
    pub pos: usize,
    pub original: String,
    pub candidates: Vec<rustyline::completion::Pair>,
    pub need_update: bool,
}

impl CompletionTracker {
    pub fn new(pos: usize, candidates: Vec<rustyline::completion::Pair>, original: String) -> Self {
        CompletionTracker {
            index: 0,
            pos,
            original,
            candidates,
            need_update: false,
        }
    }
}

pub struct JoshutoTextField {
    pub win: window::JoshutoPanel,
    pub prompt: String,
}

impl JoshutoTextField {
    pub fn new(rows: i32, cols: i32, coord: (usize, usize), prompt: String) -> Self {
        let win = window::JoshutoPanel::new(rows, cols, coord);
        ncurses::keypad(win.win, true);
        JoshutoTextField { win, prompt }
    }

    pub fn readline(&self) -> Option<String> {
        self.readline_with_initial("", "")
    }

    pub fn readline_with_initial(&self, prefix: &str, suffix: &str) -> Option<String> {
        self.win.move_to_top();
        ncurses::timeout(-1);
        let win = self.win.win;

        let prompt_len = self.prompt.len();
        let coord = (0, self.win.coords.1 + prompt_len);

        ncurses::mvwaddstr(win, 0, 0, &self.prompt);

        let mut line_buffer = rustyline::line_buffer::LineBuffer::with_capacity(255);

        line_buffer.insert_str(0, &prefix);
        line_buffer.insert_str(line_buffer.len(), &suffix);
        line_buffer.set_pos(prefix.as_bytes().len());

        let completer = rustyline::completion::FilenameCompleter::new();
        let mut completion_tracker: Option<CompletionTracker> = None;

        let mut curr_pos = unicode_width::UnicodeWidthStr::width(prefix);
        loop {
            ncurses::mvwaddstr(win, coord.0, coord.1 as i32, line_buffer.as_str());
            ncurses::wclrtoeol(win);

            ncurses::mvwchgat(
                win,
                coord.0,
                (coord.1 + curr_pos) as i32,
                1,
                ncurses::A_STANDOUT(),
                0,
            );
            ncurses::wrefresh(win);

            let ch = ncurses::wget_wch(win).unwrap();
            let ch = match ch {
                ncurses::WchResult::Char(s) => s as i32,
                ncurses::WchResult::KeyCode(s) => s,
            };

            if ch == keymap::ESCAPE {
                return None;
            } else if ch == keymap::ENTER {
                break;
            } else if ch == ncurses::KEY_HOME {
                line_buffer.move_home();
                curr_pos = 0;
                if let Some(ref mut s) = completion_tracker {
                    s.need_update = true;
                }
            } else if ch == ncurses::KEY_END {
                line_buffer.move_end();
                curr_pos = unicode_width::UnicodeWidthStr::width(line_buffer.as_str());
                if let Some(ref mut s) = completion_tracker {
                    s.need_update = true;
                }
            } else if ch == ncurses::KEY_LEFT {
                if line_buffer.move_backward(1) {
                    let pos = line_buffer.pos();
                    curr_pos = unicode_width::UnicodeWidthStr::width(&line_buffer.as_str()[..pos]);
                    if let Some(ref mut s) = completion_tracker {
                        s.need_update = true;
                    }
                }
            } else if ch == ncurses::KEY_RIGHT {
                if line_buffer.move_forward(1) {
                    let pos = line_buffer.pos();
                    curr_pos = unicode_width::UnicodeWidthStr::width(&line_buffer.as_str()[..pos]);
                    if let Some(ref mut s) = completion_tracker {
                        s.need_update = true;
                    }
                }
            } else if ch == keymap::BACKSPACE {
                if line_buffer.backspace(1) {
                    let pos = line_buffer.pos();
                    curr_pos = unicode_width::UnicodeWidthStr::width(&line_buffer.as_str()[..pos]);
                    if let Some(ref mut s) = completion_tracker {
                        s.need_update = true;
                    }
                }
            } else if ch == ncurses::KEY_DC {
                if let Some(_) = line_buffer.delete(1) {
                    if let Some(ref mut s) = completion_tracker {
                        s.need_update = true;
                    }
                }
            } else if ch == 0x9 {
                match completion_tracker {
                    None => {
                        if line_buffer.len() == line_buffer.pos() {
                            let res = completer.complete(line_buffer.as_str(), line_buffer.len());
                            if let Ok((pos, candidates)) = res {
                                let ct = CompletionTracker::new(
                                    pos,
                                    candidates,
                                    String::from(line_buffer.as_str()),
                                );
                                completion_tracker = Some(ct);
                            }
                        }
                    }
                    Some(ref mut s) => {
                        if s.need_update {
                            completion_tracker = None;
                            if line_buffer.len() == line_buffer.pos() {
                                let res =
                                    completer.complete(line_buffer.as_str(), line_buffer.len());
                                if let Ok((pos, candidates)) = res {
                                    let ct = CompletionTracker::new(
                                        pos,
                                        candidates,
                                        String::from(line_buffer.as_str()),
                                    );
                                    completion_tracker = Some(ct);
                                }
                            }
                        }
                    }
                }
                match completion_tracker {
                    Some(ref mut s) => {
                        if s.index < s.candidates.len() {
                            let candidate = &s.candidates[s.index];
                            line_buffer.kill_line();
                            line_buffer.yank_pop(line_buffer.pos(), candidate.display());
                            s.index += 1;
                        } else {
                            line_buffer.kill_line();
                            line_buffer.yank_pop(line_buffer.pos(), &s.original);
                            s.index = 0;
                        }
                    }
                    None => {}
                }
                curr_pos = unicode_width::UnicodeWidthStr::width(
                    &line_buffer.as_str()[..line_buffer.pos()],
                );
            } else if ch == ncurses::KEY_UP {

            } else if ch == ncurses::KEY_DOWN {

            } else if let Some(ch) = std::char::from_u32(ch as u32) {
                match line_buffer.insert(ch, 1) {
                    Some(_) => {
                        curr_pos += unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);
                        if let Some(ref mut s) = completion_tracker {
                            s.need_update = true;
                        }
                    }
                    None => {}
                };
            }
        }
        let lbstr = line_buffer.to_string();
        if lbstr.len() == 0 {
            None
        } else {
            Some(lbstr)
        }
    }
}
