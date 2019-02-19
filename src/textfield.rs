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
        let mut buf_vec: Vec<char> = Vec::with_capacity(prefix.len() + suffix.len());
        let mut curs_x: i32 = self.prompt.len() as i32;
        for ch in prefix.chars() {
            let char_len = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);
            buf_vec.push(ch);
            curs_x += char_len as i32;
        }
        let curr_index: usize = buf_vec.len();

        for ch in suffix.chars() {
            buf_vec.push(ch);
        }
        ncurses::timeout(-1);
        self.readline_(buf_vec, curs_x, curr_index)
    }

    fn readline_(
        &self,
        mut buffer: Vec<(char)>,
        mut curs_x: i32,
        mut curr_index: usize,
    ) -> Option<String> {
        self.win.move_to_top();

        let prompt_len = self.prompt.len();
        let win = self.win.win;
        ncurses::wmove(win, self.win.rows - 1, 0);
        ncurses::waddstr(win, &self.prompt);

        ncurses::doupdate();

        let coord = (0, self.win.coords.1 + prompt_len);

        loop {
            ncurses::wmove(win, coord.0, coord.1 as i32);
            {
                let str_ch: String = String::from_iter(&buffer);
                ncurses::waddstr(win, &str_ch);
            }
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
            } else if ch == ncurses::KEY_HOME {
                if curr_index != 0 {
                    curs_x = coord.1 as i32;
                    curr_index = 0;
                }
            } else if ch == ncurses::KEY_END {
                let buffer_len = buffer.len();
                if curr_index != buffer_len {
                    for x in &buffer {
                        curs_x += *x as i32;
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
            } else {
                let ch = std::char::from_u32(ch as u32).unwrap();
                let char_len = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);

                buffer.insert(curr_index, ch);

                curs_x += char_len as i32;
                curr_index += 1;
            }
        }
        let user_str: String = buffer.iter().map(|ch| ch).collect();

        Some(user_str)
    }
}
