use ratatui::termion::event::{Event, Key};

use crate::error::AppResult;
use crate::global;
use crate::run::process_event;
use crate::state::remove_external_preview;
use crate::state::AppState;
use crate::tasks::AppEvent;
use crate::ui::widgets::TuiLogs;
use crate::ui::AppBackend;

/// Implements `show_logs`: runs a nested event loop showing everything `tracing` has logged,
/// scrollable with the arrow keys and page up/down. Opens scrolled to the newest entries.
pub fn show_logs(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    app_state.flush_event();

    // usize::MAX is clamped to the last page by the widget
    let mut offset = usize::MAX;

    loop {
        // re-read every frame so entries logged while the view is open show up
        let contents = global::APP_LOG.contents();
        let lines: Vec<&str> = contents.lines().collect();

        remove_external_preview(app_state);
        backend.render(TuiLogs::new(&lines, &mut offset));

        let page_height = backend
            .terminal_ref()
            .size()
            .map(|size| TuiLogs::page_height(size.height))
            .unwrap_or(1)
            .max(1);

        let event = match app_state.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        match event {
            AppEvent::TerminalEvent(event) => {
                match event {
                    Event::Key(Key::Esc) | Event::Key(Key::Char('q' | '#')) => break,
                    Event::Key(Key::Up) => offset = offset.saturating_sub(1),
                    Event::Key(Key::Down) => offset = offset.saturating_add(1),
                    Event::Key(Key::PageUp) => offset = offset.saturating_sub(page_height),
                    Event::Key(Key::PageDown) => offset = offset.saturating_add(page_height),
                    Event::Key(Key::Home) => offset = 0,
                    Event::Key(Key::End) => offset = usize::MAX,
                    _ => (),
                }
                app_state.flush_event();
            }
            _ => process_event::process_noninteractive(event, app_state),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use ratatui::termion::event::{Event, Key};

    use crate::app_command::AppCommand;
    use crate::config::{AppKeybindConfig, KeyEventAction};

    #[test]
    fn hash_opens_logs_by_default() {
        let keymap = AppKeybindConfig::try_default().expect("default keymap parses");
        match keymap.default_view.get(&Event::Key(Key::Char('#'))) {
            Some(KeyEventAction::Base { commands, .. }) => {
                assert!(matches!(commands.as_slice(), [AppCommand::ShowLogs]))
            }
            other => panic!("`#` is not bound to show_logs: {other:?}"),
        }
    }
}
