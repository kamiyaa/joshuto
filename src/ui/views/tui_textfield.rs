use std::str::FromStr;

use rustyline::completion::{Candidate, Completer, FilenameCompleter, Pair};
use rustyline::line_buffer;

use termion::event::{Event, Key};
use tui::layout::Rect;
use tui::widgets::Clear;
use unicode_width::UnicodeWidthStr;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::key_command::{complete_command, Command, InteractiveExecute};
use crate::ui::views::TuiView;
use crate::ui::widgets::{TuiMenu, TuiMultilineText};
use crate::ui::TuiBackend;
use crate::util::input;

struct CompletionTracker {
    pub index: usize,
    pub pos: usize,
    pub _original: String,
    pub candidates: Vec<Pair>,
}

impl CompletionTracker {
    pub fn new(pos: usize, candidates: Vec<Pair>, _original: String) -> Self {
        CompletionTracker {
            index: 0,
            pos,
            _original,
            candidates,
        }
    }
}

pub struct CursorInfo {
    pub x: usize,
    pub y: usize,
}

pub struct TuiTextField<'a> {
    _prompt: &'a str,
    _prefix: &'a str,
    _suffix: &'a str,
    _menu_items: Vec<&'a str>,
}

impl<'a> TuiTextField<'a> {
    pub fn menu_items<I>(&mut self, items: I) -> &mut Self
    where
        I: Iterator<Item = &'a str>,
    {
        self._menu_items = items.collect();
        self
    }

    pub fn prompt(&mut self, prompt: &'a str) -> &mut Self {
        self._prompt = prompt;
        self
    }

    pub fn prefix(&mut self, prefix: &'a str) -> &mut Self {
        self._prefix = prefix;
        self
    }

    pub fn suffix(&mut self, suffix: &'a str) -> &mut Self {
        self._suffix = suffix;
        self
    }

    pub fn get_input(
        &mut self,
        backend: &mut TuiBackend,
        context: &mut AppContext,
    ) -> Option<String> {
        let mut line_buffer = line_buffer::LineBuffer::with_capacity(255);
        let completer = FilenameCompleter::new();

        let mut completion_tracker: Option<CompletionTracker> = None;

        let char_idx = self._prefix.chars().map(|c| c.len_utf8()).sum();

        line_buffer.insert_str(0, self._suffix);
        line_buffer.insert_str(0, self._prefix);
        line_buffer.set_pos(char_idx);

        let terminal = backend.terminal_mut();
        let _ = terminal.show_cursor();

        let mut curr_history_index = context.commandline_context_ref().history_ref().len();

        loop {
            terminal
                .draw(|frame| {
                    let area: Rect = frame.size();
                    if area.height == 0 {
                        return;
                    }
                    {
                        let mut view = TuiView::new(context);
                        view.show_bottom_status = false;
                        frame.render_widget(view, area);
                    }

                    let area_width = area.width as usize;
                    let buffer_str = line_buffer.as_str();
                    let cursor_xpos = line_buffer.pos();

                    let line_str = format!("{}{}", self._prompt, buffer_str);
                    let multiline = TuiMultilineText::new(line_str.as_str(), area_width);
                    let multiline_height = multiline.height();

                    // render menu
                    {
                        let menu_widget = TuiMenu::new(self._menu_items.as_slice());
                        let menu_len = menu_widget.len();
                        let menu_y = if menu_len + 1 > area.height as usize {
                            0
                        } else {
                            (area.height as usize - menu_len - 1) as u16
                        };

                        let menu_rect = Rect {
                            x: 0,
                            y: menu_y - multiline_height as u16,
                            width: area.width,
                            height: menu_len as u16 + 1,
                        };
                        frame.render_widget(Clear, menu_rect);
                        frame.render_widget(menu_widget, menu_rect);
                    }

                    let multiline_rect = Rect {
                        x: 0,
                        y: area.height - multiline_height as u16,
                        width: area.width,
                        height: multiline_height as u16,
                    };
                    let mut cursor_info = CursorInfo {
                        x: 0,
                        y: area.height as usize,
                    };

                    // get cursor render position
                    let cursor_prefix_width =
                        buffer_str[0..cursor_xpos].width() + self._prompt.len();
                    let y_offset = cursor_prefix_width / area_width;
                    cursor_info.y = area.height as usize - multiline_height + y_offset;
                    cursor_info.x = cursor_prefix_width % area_width + y_offset;

                    // render multiline textfield
                    frame.render_widget(Clear, multiline_rect);
                    frame.render_widget(multiline, multiline_rect);

                    // render cursor
                    frame.set_cursor(cursor_info.x as u16, cursor_info.y as u16);
                })
                .unwrap();

            if let Ok(event) = context.poll_event() {
                match event {
                    AppEvent::Termion(Event::Key(key)) => {
                        match key {
                            Key::Backspace => {
                                if line_buffer.backspace(1) {
                                    completion_tracker.take();
                                }
                            }
                            Key::Left => {
                                if line_buffer.move_backward(1) {
                                    completion_tracker.take();
                                }
                            }
                            Key::Right => {
                                if line_buffer.move_forward(1) {
                                    completion_tracker.take();
                                }
                            }
                            Key::Delete => {
                                if line_buffer.delete(1).is_some() {
                                    completion_tracker.take();
                                }
                            }
                            Key::Home => {
                                line_buffer.move_home();
                                completion_tracker.take();
                            }
                            Key::End => {
                                line_buffer.move_end();
                                completion_tracker.take();
                            }
                            Key::Up => {
                                curr_history_index = if curr_history_index > 0 {
                                    curr_history_index - 1
                                } else {
                                    0
                                };
                                line_buffer.move_home();
                                line_buffer.kill_line();
                                if let Some(s) = context
                                    .commandline_context_ref()
                                    .history_ref()
                                    .get(curr_history_index)
                                {
                                    line_buffer.insert_str(0, s);
                                }
                            }
                            Key::Down => {
                                curr_history_index = if curr_history_index
                                    < context.commandline_context_ref().history_ref().len()
                                {
                                    curr_history_index + 1
                                } else {
                                    curr_history_index
                                };
                                line_buffer.move_home();
                                line_buffer.kill_line();
                                if let Some(s) = context
                                    .commandline_context_ref()
                                    .history_ref()
                                    .get(curr_history_index)
                                {
                                    line_buffer.insert_str(0, s);
                                }
                            }
                            Key::Esc => {
                                let _ = terminal.hide_cursor();
                                return None;
                            }
                            Key::Char('\t') => {
                                if completion_tracker.is_none() {
                                    let line = line_buffer.as_str().split_once(' ');
                                    let res = match line {
                                        None => Ok((0, complete_command(line_buffer.as_str()))),
                                        Some((_command, _files)) => completer
                                            .complete_path(line_buffer.as_str(), line_buffer.pos()),
                                    };

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
                                        completer.update(
                                            &mut line_buffer,
                                            s.pos,
                                            candidate.display(),
                                        );
                                        s.index = (s.index + 1) % s.candidates.len();
                                    }
                                }
                            }
                            Key::Char('\n') => {
                                break;
                            }
                            Key::Char(c) => {
                                if line_buffer.insert(c, 1).is_some() {
                                    completion_tracker.take();
                                }

                                if let Ok(command) = Command::from_str(line_buffer.as_str()) {
                                    command.interactive_execute(context)
                                }
                            }
                            _ => {}
                        }
                        context.flush_event();
                    }
                    AppEvent::Termion(_) => {
                        context.flush_event();
                    }
                    event => input::process_noninteractive(event, context),
                };
            }
        }
        let _ = terminal.hide_cursor();

        if line_buffer.as_str().is_empty() {
            None
        } else {
            let input_string = line_buffer.to_string();
            Some(input_string)
        }
    }
}

impl<'a> std::default::Default for TuiTextField<'a> {
    fn default() -> Self {
        Self {
            _prompt: "",
            _prefix: "",
            _suffix: "",
            _menu_items: vec![],
        }
    }
}
