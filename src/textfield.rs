use crate::config::keymap;
use crate::window;

use rustyline::completion::{Candidate, Completer, FilenameCompleter, Pair};
use rustyline::line_buffer;

struct CompletionTracker {
    pub index: usize,
    pub pos: usize,
    pub original: String,
    pub candidates: Vec<Pair>,
}

impl CompletionTracker {
    pub fn new(pos: usize, candidates: Vec<Pair>, original: String) -> Self {
        CompletionTracker {
            index: 0,
            pos,
            original,
            candidates,
        }
    }
}

pub struct JoshutoTextField {
    pub win: window::JoshutoPanel,
    pub prompt: String,
    pub prefix: String,
    pub suffix: String,
}

impl JoshutoTextField {
    pub fn new(
        rows: i32,
        cols: i32,
        coord: (usize, usize),
        prompt: String,
        prefix: String,
        suffix: String,
    ) -> Self {
        let win = window::JoshutoPanel::new(rows, cols, coord);
        ncurses::keypad(win.win, true);
        JoshutoTextField {
            win,
            prompt,
            prefix,
            suffix,
        }
    }

    pub fn readline(&self) -> Option<String> {
        self.win.move_to_top();
        ncurses::timeout(-1);
        let win = self.win.win;

        let prompt_len = self.prompt.len();
        let coord = (0, self.win.coords.1 + prompt_len);

        ncurses::mvwaddstr(win, 0, 0, &self.prompt);

        let mut line_buffer = line_buffer::LineBuffer::with_capacity(255);
        let completer = FilenameCompleter::new();

        line_buffer.insert_str(0, &self.prefix);
        line_buffer.insert_str(line_buffer.len(), &self.suffix);
        line_buffer.set_pos(self.prefix.as_bytes().len());

        let mut completion_tracker: Option<CompletionTracker> = None;

        let mut curr_pos = unicode_width::UnicodeWidthStr::width(self.prefix.as_str());
        loop {
            ncurses::mvwaddstr(win, coord.0, coord.1 as i32, line_buffer.as_str());
            ncurses::wclrtoeol(win);

            /* draws cursor */
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
                completion_tracker.take();
            } else if ch == ncurses::KEY_END {
                line_buffer.move_end();
                curr_pos = unicode_width::UnicodeWidthStr::width(line_buffer.as_str());
                completion_tracker.take();
            } else if ch == ncurses::KEY_LEFT {
                if line_buffer.move_backward(1) {
                    let pos = line_buffer.pos();
                    curr_pos = unicode_width::UnicodeWidthStr::width(&line_buffer.as_str()[..pos]);
                    completion_tracker.take();
                }
            } else if ch == ncurses::KEY_RIGHT {
                if line_buffer.move_forward(1) {
                    let pos = line_buffer.pos();
                    curr_pos = unicode_width::UnicodeWidthStr::width(&line_buffer.as_str()[..pos]);
                    completion_tracker.take();
                }
            } else if ch == keymap::BACKSPACE {
                if line_buffer.backspace(1) {
                    let pos = line_buffer.pos();
                    curr_pos = unicode_width::UnicodeWidthStr::width(&line_buffer.as_str()[..pos]);
                    completion_tracker.take();
                }
            } else if ch == ncurses::KEY_DC {
                if line_buffer.delete(1).is_some() {
                    completion_tracker.take();
                }
            } else if ch == keymap::TAB {
                if completion_tracker.is_none() {
                    let res = completer.complete_path(line_buffer.as_str(), line_buffer.pos());
                    if let Ok((pos, mut candidates)) = res {
                        candidates.sort_by(|x, y| {
                            x.display()
                                .partial_cmp(y.display())
                                .unwrap_or(std::cmp::Ordering::Less)
                        });
                        let ct = CompletionTracker::new(
                            pos,
                            candidates,
                            String::from(line_buffer.as_str()),
                        );
                        completion_tracker = Some(ct);
                    }
                }

                if let Some(ref mut s) = completion_tracker {
                    if s.index < s.candidates.len() {
                        let candidate = &s.candidates[s.index];
                        completer.update(&mut line_buffer, s.pos, candidate.replacement());
                        s.index += 1;
                    }
                }
                curr_pos = unicode_width::UnicodeWidthStr::width(
                    &line_buffer.as_str()[..line_buffer.pos()],
                );
            } else if ch == ncurses::KEY_UP {
                completion_tracker.take();
            } else if ch == ncurses::KEY_DOWN {
                completion_tracker.take();
            } else if let Some(ch) = std::char::from_u32(ch as u32) {
                if line_buffer.insert(ch, 1).is_some() {
                    curr_pos += unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);
                    completion_tracker.take();
                }
            }
        }
        if line_buffer.as_str().is_empty() {
            None
        } else {
            //            let strin = rustyline::completion::unescape(line_buffer.as_str(), ESCAPE_CHAR).into_owned();
            let strin = line_buffer.to_string();
            Some(strin)
        }
    }
}
