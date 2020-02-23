use std::io::Write;

use rustyline::completion::{Candidate, Completer, FilenameCompleter, Pair};
use rustyline::line_buffer;

use termion::cursor::Goto;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use unicode_width::UnicodeWidthStr;

use crate::context::JoshutoContext;
use crate::ui::TuiBackend;
use crate::util::event::{Event, Events};

use super::{TuiMenu, TuiView};

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

pub struct TuiTextField<'a> {
    _prompt: &'a str,
    _prefix: &'a str,
    _suffix: &'a str,
    _menu: Option<&'a mut TuiMenu<'a>>,
}

impl<'a> TuiTextField<'a> {
    pub fn menu(mut self, menu: &'a mut TuiMenu<'a>) -> Self {
        self._menu = Some(menu);
        self
    }

    pub fn prompt(mut self, prompt: &'a str) -> Self {
        self._prompt = prompt;
        self
    }

    pub fn prefix(mut self, prefix: &'a str) -> Self {
        self._prefix = prefix;
        self
    }

    pub fn suffix(mut self, suffix: &'a str) -> Self {
        self._suffix = suffix;
        self
    }

    pub fn get_input(
        &mut self,
        backend: &mut TuiBackend,
        context: &JoshutoContext,
    ) -> Option<String> {
        let mut line_buffer = line_buffer::LineBuffer::with_capacity(255);
        let completer = FilenameCompleter::new();

        let mut completion_tracker: Option<CompletionTracker> = None;

        let mut char_idx = self
            ._prefix
            .char_indices()
            .last()
            .map(|(i, c)| i)
            .unwrap_or(0);

        line_buffer.insert_str(0, self._prefix);
        line_buffer.insert_str(line_buffer.len(), self._suffix);
        line_buffer.set_pos(char_idx);

        backend.terminal.show_cursor();
        let mut cursor_xpos = line_buffer.pos() + 1;
        {
            let frame = backend.terminal.get_frame();
            let f_size = frame.size();
            backend
                .terminal
                .set_cursor(cursor_xpos as u16, f_size.height - 1);
        }

        loop {
            backend.terminal.draw(|mut frame| {
                let f_size = frame.size();
                if f_size.height == 0 {
                    return;
                }

                {
                    let mut view = TuiView::new(&context);
                    view.show_bottom_status = false;
                    view.render(&mut frame, f_size);
                }

                let top_rect = Rect {
                    x: 0,
                    y: 0,
                    width: f_size.width,
                    height: 1,
                };

                if let Some(menu) = self._menu.as_mut() {
                    menu.render(&mut frame, top_rect);
                }

                let cmd_prompt_style = Style::default().fg(Color::LightGreen);

                let text = [
                    Text::styled(self._prompt, cmd_prompt_style),
                    Text::raw(line_buffer.as_str()),
                ];

                let textfield_rect = Rect {
                    x: 0,
                    y: f_size.height - 1,
                    width: f_size.width,
                    height: 1,
                };

                Paragraph::new(text.iter())
                    .wrap(true)
                    .render(&mut frame, textfield_rect);
            });

            if let Ok(event) = context.events.next() {
                match event {
                    Event::Input(Key::Backspace) => {
                        if line_buffer.backspace(1) {
                            completion_tracker.take();
                        }
                    }
                    Event::Input(Key::Left) => {
                        if line_buffer.move_backward(1) {
                            completion_tracker.take();
                        }
                    }
                    Event::Input(Key::Right) => {
                        if line_buffer.move_forward(1) {
                            completion_tracker.take();
                        }
                    }
                    Event::Input(Key::Delete) => {
                        if line_buffer.delete(1).is_some() {
                            completion_tracker.take();
                        }
                    }
                    Event::Input(Key::Home) => {
                        line_buffer.move_end();
                        completion_tracker.take();
                    }
                    Event::Input(Key::End) => {
                        line_buffer.move_end();
                        completion_tracker.take();
                    }
                    Event::Input(Key::Up) => {}
                    Event::Input(Key::Down) => {}
                    Event::Input(Key::Esc) => {
                        backend.terminal.hide_cursor();
                        return None;
                    }
                    Event::Input(Key::Char('\t')) => {
                        if completion_tracker.is_none() {
                            let res =
                                completer.complete_path(line_buffer.as_str(), line_buffer.pos());
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
                    }
                    Event::Input(Key::Char('\n')) => {
                        break;
                    }
                    Event::Input(Key::Char(c)) => {
                        if line_buffer.insert(c, 1).is_some() {
                            completion_tracker.take();
                        }
                    }
                    _ => {}
                };
            }
            cursor_xpos = line_buffer.pos() + 1;
            {
                let frame = backend.terminal.get_frame();
                let f_size = frame.size();
                backend
                    .terminal
                    .set_cursor(cursor_xpos as u16, f_size.height - 1);
            }
        }
        backend.terminal.hide_cursor();
        if line_buffer.as_str().is_empty() {
            None
        } else {
            let strin = line_buffer.to_string();
            Some(strin)
        }
    }
}

impl<'a> std::default::Default for TuiTextField<'a> {
    fn default() -> Self {
        Self {
            _prompt: "",
            _prefix: "",
            _suffix: "",
            _menu: None,
        }
    }
}
