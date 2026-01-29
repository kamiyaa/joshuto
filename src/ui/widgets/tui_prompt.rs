use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::termion::event::{Event, Key};
use ratatui::text::Span;
use ratatui::widgets::{Clear, Paragraph, Wrap};

use crate::run::process_event;
use crate::types::event::AppEvent;
use crate::types::state::AppState;
use crate::ui::views::TuiView;
use crate::ui::AppBackend;

pub struct TuiPrompt<'a> {
    prompt: &'a str,
}

impl<'a> TuiPrompt<'a> {
    pub fn new(prompt: &'a str) -> Self {
        Self { prompt }
    }

    pub fn get_key(&mut self, app_state: &mut AppState, backend: &mut AppBackend) -> Key {
        let terminal = backend.terminal_mut();

        app_state.flush_event();
        loop {
            let _ = terminal.draw(|frame| {
                let f_size: Rect = frame.area();
                if f_size.height == 0 {
                    return;
                }

                {
                    let mut view = TuiView::new(app_state);
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

                frame.render_widget(Clear, textfield_rect);
                frame.render_widget(
                    Paragraph::new(text).wrap(Wrap { trim: true }),
                    textfield_rect,
                );
            });

            if let Ok(event) = app_state.poll_event() {
                match event {
                    AppEvent::TerminalEvent(Event::Key(key)) => {
                        return key;
                    }
                    AppEvent::TerminalEvent(_) => {
                        app_state.flush_event();
                    }
                    event => process_event::process_noninteractive(event, app_state),
                };
            }
        }
    }
}
