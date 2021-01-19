use termion::event::{Event, Key};

use tui::layout::Rect;

use crate::context::JoshutoContext;
use crate::ui::widgets::TuiWorker;
use crate::ui::TuiBackend;
use crate::util::event::JoshutoEvent;
use crate::util::input;

pub struct TuiWorkerView {}

impl TuiWorkerView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn display(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) {
        context.flush_event();
        let terminal = backend.terminal_mut();

        loop {
            let _ = terminal.draw(|frame| {
                let f_size: Rect = frame.size();
                if f_size.height == 0 {
                    return;
                }
                {
                    let view = TuiWorker::new(&context);
                    frame.render_widget(view, f_size);
                }
            });

            if let Ok(event) = context.poll_event() {
                match event {
                    JoshutoEvent::Termion(event) => {
                        match event {
                            Event::Key(Key::Esc) => {
                                break;
                            }
                            _ => {}
                        }
                        context.flush_event();
                    }
                    event => input::process_noninteractive(event, context),
                };
            }
        }
    }
}
