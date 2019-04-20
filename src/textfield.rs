use std::iter::FromIterator;

use crate::config::keymap;
use crate::window;

pub struct JoshutoTextField {
    pub win: window::JoshutoPanel,
    pub prompt: String,
}

impl JoshutoTextField {
    pub fn new(rows: i32, cols: i32, coord: (usize, usize), prompt: String) -> Self {
        let win = window::JoshutoPanel::new(rows, cols, coord);
        ncurses::keypad(win.win, true);
        ncurses::scrollok(win.win, true);
        JoshutoTextField { win, prompt }
    }

    pub fn readline_with_initial(&self, prefix: &str, suffix: &str) -> Option<String> {
        self.win.move_to_top();
        ncurses::timeout(-1);

        let completer = rustyline::completion::FilenameCompleter::new();

        let win = self.win.win;
        let prompt_len = self.prompt.len();
        let coord = (0, self.win.coords.1 + prompt_len);
        ncurses::wmove(win, self.win.coords.0 as i32, self.win.coords.1 as i32);
        ncurses::waddstr(win, &self.prompt);

        let mut line_buffer = rustyline::line_buffer::LineBuffer::with_capacity(self.win.cols as usize);

        // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
        line_buffer.insert_str(0, &prefix);
        let mut curr_pos = unicode_width::UnicodeWidthStr::width(line_buffer.as_str());
        line_buffer.insert_str(line_buffer.len(), &suffix);
        line_buffer.set_pos(curr_pos);

        loop {
            ncurses::wmove(win, coord.0, coord.1 as i32);
            let line_str = line_buffer.as_str();
            ncurses::waddstr(win, line_str);
            ncurses::waddstr(win, "    ");

            ncurses::mvwchgat(win, coord.0 as i32, (coord.1 + curr_pos) as i32, 1, ncurses::A_STANDOUT(), 0);

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
            } else if ch == ncurses::KEY_END {
                line_buffer.move_end();
                curr_pos = unicode_width::UnicodeWidthStr::width(line_buffer.as_str());
            } else if ch == ncurses::KEY_LEFT {
                if line_buffer.move_backward(1) {
                    curr_pos -= 1;
                }
            } else if ch == ncurses::KEY_RIGHT {
                if line_buffer.move_forward(1) {
                    curr_pos += 1;
                }
            } else if ch == keymap::BACKSPACE {
                if line_buffer.backspace(1) {
                    curr_pos -= 1;
                }
            } else if ch == ncurses::KEY_DC {
                line_buffer.delete(1);
            } else if let Some(ch) = std::char::from_u32(ch as u32) {
                match line_buffer.insert(ch, 1) {
                    Some(true) => curr_pos += 1,
                    _ => {},
                }
            }
        }
        let lbstr = line_buffer.to_string();
        if lbstr.len() == 0 {
            None
        } else {
            Some(lbstr)
        }
    }

    fn readline_(
        &self,
        mut buffer: Vec<char>,
        mut curs_x: i32,
        mut curr_index: usize,
    ) -> Option<String> {
        self.win.move_to_top();
        let completer = rustyline::completion::FilenameCompleter::new();

        let prompt_len = self.prompt.len();
        let win = self.win.win;

        ncurses::wmove(win, self.win.rows - 1, 0);
        ncurses::waddstr(win, &self.prompt);

        ncurses::doupdate();

        let coord = (0, self.win.coords.1 + prompt_len);

        loop {
            ncurses::wmove(win, coord.0, coord.1 as i32);
            ncurses::waddstr(win, "    ");

            ncurses::mvwchgat(win, coord.0 as i32, curs_x, 1, ncurses::A_STANDOUT(), 0);
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
            } else if ch == keymap::TAB {
            } else if ch == ncurses::KEY_HOME {
                if curr_index != 0 {
                    curs_x = coord.1 as i32;
                    curr_index = 0;
                }
            } else if ch == ncurses::KEY_END {
                let buffer_len = buffer.len();
                if curr_index != buffer_len {
                    for i in curr_index..buffer_len {
                        curs_x +=
                            unicode_width::UnicodeWidthChar::width(buffer[i]).unwrap_or(1) as i32;
                    }
                    curr_index = buffer_len;
                }
            } else if ch == ncurses::KEY_LEFT {
                if curr_index > 0 {
                    curr_index -= 1;
                    curs_x -= unicode_width::UnicodeWidthChar::width(buffer[curr_index])
                        .unwrap_or(1) as i32;
                }
            } else if ch == ncurses::KEY_RIGHT {
                let buffer_len = buffer.len();
                if curr_index < buffer_len {
                    curs_x += unicode_width::UnicodeWidthChar::width(buffer[curr_index])
                        .unwrap_or(1) as i32;
                    curr_index += 1;
                }
            } else if ch == keymap::BACKSPACE {
                let buffer_len = buffer.len();
                if buffer_len == 0 {
                    continue;
                }

                if curr_index == buffer_len {
                    curr_index -= 1;
                    if let Some(ch) = buffer.pop() {
                        curs_x -= unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as i32;
                    }
                } else if curr_index > 0 {
                    curr_index -= 1;
                    let ch = buffer.remove(curr_index);
                    curs_x -= unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as i32;
                }
            } else if ch == ncurses::KEY_DC {
                let buffer_len = buffer.len();

                if buffer_len == 0 || curr_index == buffer_len {
                    continue;
                }

                if curr_index > 0 {
                    let ch = buffer.remove(curr_index);
                    if curr_index > buffer_len {
                        curr_index -= 1;
                        curs_x -= unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as i32;
                    }
                } else if curr_index == 0 {
                    buffer.remove(curr_index);
                }
            } else if let Some(ch) = std::char::from_u32(ch as u32) {
                let char_len = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);

                buffer.insert(curr_index, ch);

                curs_x += char_len as i32;
                curr_index += 1;
            }
        }
        let user_str: String = buffer.iter().collect();

        Some(user_str)
    }
}
