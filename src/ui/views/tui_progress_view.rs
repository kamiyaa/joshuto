use ratatui::buffer::Buffer;
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Text, Widget};

use super::{TuiDirList, TuiDirListDetailed, TuiFooter, TuiTabBar, TuiTopBar};
use crate::context::AppContext;

const TAB_VIEW_WIDTH: u16 = 15;

pub struct TuiProgressView<'a> {
    pub context: &'a AppContext,
}

impl<'a> TuiProgressView<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self {
            context,
            show_bottom_status: true,
        }
    }
}

impl<'a> Widget for TuiProgressView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let f_size = area;

        let layout_rect = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .split(f_size);

        let terminal = backend.terminal_mut();

        loop {
            terminal.draw(|mut frame| {}).unwrap();

            if let Ok(event) = context.events.next() {
                match event {
                    Event::IOWorkerProgress(_) => {}
                    Event::Input(key) => {
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
                            Key::Up => {}
                            Key::Down => {}
                            Key::Esc => {
                                return None;
                            }
                            Key::Char('\t') => {
                                if completion_tracker.is_none() {
                                    let res = completer
                                        .complete_path(line_buffer.as_str(), line_buffer.pos());
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
                                        s.index += 1;
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
                            }
                            _ => {}
                        }
                        context.events.flush();
                    }
                    _ => {}
                };
            }
        }
    }
}
