use termion::event::{Event, Key};

use tui::layout::Rect;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::ui::widgets::{TuiTopBar, TuiWorker};
use crate::ui::TuiBackend;
use crate::util::input;

pub struct TuiWorkerView {
    exit_key: Key,
}

impl TuiWorkerView {
    pub fn new(exit_key: Key) -> Self {
        Self { exit_key }
    }

    pub fn display(&self, context: &mut AppContext, backend: &mut TuiBackend) {
        let terminal = backend.terminal_mut();

        loop {
            let _ = terminal.draw(|frame| {
                let area: Rect = frame.size();
                if area.height == 0 {
                    return;
                }

                let rect = Rect { height: 1, ..area };
                let curr_tab = context.tab_context_ref().curr_tab_ref();
                let view = TuiTopBar::new(context, curr_tab.cwd());
                frame.render_widget(view, rect);

                let rect = Rect {
                    x: 0,
                    y: 1,
                    width: area.width,
                    height: area.height - 1,
                };
                let view = TuiWorker::new(&context);
                frame.render_widget(view, rect);
            });

            if let Ok(event) = context.poll_event() {
                match event {
                    AppEvent::Termion(event) => {
                        match event {
                            Event::Key(Key::Esc) => break,
                            Event::Key(k) if k == self.exit_key => break,
                            _ => {},
                        }
                        context.flush_event();
                    }
                    event => input::process_noninteractive(event, context),
                };
            }
        }
    }
}
