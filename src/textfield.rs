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
        JoshutoTextField { win, prompt }
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

        let mut curr_pos = unicode_width::UnicodeWidthStr::width(prefix);

        loop {
            ncurses::mvwaddstr(win, coord.0, coord.1 as i32, line_buffer.as_str());
            ncurses::waddstr(win, "    ");

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
            } else if ch == ncurses::KEY_END {
                line_buffer.move_end();
                curr_pos = unicode_width::UnicodeWidthStr::width(line_buffer.as_str());
            } else if ch == ncurses::KEY_LEFT {
                if line_buffer.move_backward(1) {
                    let pos = line_buffer.pos();
                    curr_pos = unicode_width::UnicodeWidthStr::width(&line_buffer.as_str()[..pos]);
                }
            } else if ch == ncurses::KEY_RIGHT {
                if line_buffer.move_forward(1) {
                    let pos = line_buffer.pos();
                    curr_pos = unicode_width::UnicodeWidthStr::width(&line_buffer.as_str()[..pos]);
                }
            } else if ch == keymap::BACKSPACE {
                if line_buffer.backspace(1) {
                    let pos = line_buffer.pos();
                    curr_pos = unicode_width::UnicodeWidthStr::width(&line_buffer.as_str()[..pos]);
                }
            } else if ch == ncurses::KEY_DC {
                line_buffer.delete(1);
            } else if ch == ncurses::KEY_UP {

            } else if ch == ncurses::KEY_DOWN {

            } else if let Some(ch) = std::char::from_u32(ch as u32) {
                match line_buffer.insert(ch, 1) {
                    Some(_) => {
                        curr_pos += unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);
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
