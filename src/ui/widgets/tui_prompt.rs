use termion::event::Key;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Paragraph, Wrap};

use crate::context::JoshutoContext;
use crate::ui::TuiBackend;
use crate::util::event::Event;
use crate::util::worker;

use super::TuiView;

pub struct TuiPrompt<'a> {
    prompt: &'a str,
}

impl<'a> TuiPrompt<'a> {
    pub fn new(prompt: &'a str) -> Self {
        Self { prompt }
    }

    pub fn get_key(&mut self, backend: &mut TuiBackend, context: &mut JoshutoContext) -> Key {
        let terminal = backend.terminal_mut();

        context.flush_event();
        loop {
            terminal.draw(|frame| {
                let f_size: Rect = frame.size();
                if f_size.height == 0 {
                    return;
                }

                {
                    let mut view = TuiView::new(&context);
                    view.show_bottom_status = false;
                    frame.render_widget(view, f_size);
                }

                let prompt_style = Style::default().fg(Color::LightYellow);

                let text = Span::styled(self.prompt, prompt_style);

                let textfield_rect = Rect {
                    x: 0,
                    y: f_size.height - 1,
                    width: f_size.width,
                    height: 1,
                };

                frame.render_widget(
                    Paragraph::new(text).wrap(Wrap { trim: true }),
                    textfield_rect,
                );
            });

            if let Ok(event) = context.poll_event() {
                match event {
                    Event::IOWorkerProgress(res) => {
                        worker::process_worker_progress(context, res);
                    }
                    Event::IOWorkerResult(res) => {
                        worker::process_finished_worker(context, res);
                    }
                    Event::Input(key) => {
                        return key;
                    }
                };
            }
        }
    }
}
