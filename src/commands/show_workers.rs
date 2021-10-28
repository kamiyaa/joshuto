use termion::event::{Event, Key};

use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::event::AppEvent;
use crate::ui::views::TuiWorkerView;
use crate::ui::TuiBackend;
use crate::util::input;

pub fn show_workers(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    _keymap_t: &AppKeyMapping,
) -> JoshutoResult<()> {
    context.flush_event();

    loop {
        backend.render(TuiWorkerView::new(context));

        if let Ok(event) = context.poll_event() {
            match event {
                AppEvent::Termion(event) => {
                    match event {
                        Event::Key(Key::Esc) => break,
                        _ => {}
                    }
                    context.flush_event();
                }
                event => input::process_noninteractive(event, context),
            };
        }
    }
    Ok(())
}
