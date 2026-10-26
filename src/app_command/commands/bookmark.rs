use std::fs::File;
use std::io::Write;
use std::path;

use ratatui::layout::Rect;
use ratatui::termion::event::Event;
use ratatui::widgets::Clear;

use crate::config::utils::search_directories;
use crate::config::ConfigType;
use crate::config::{BookmarkRaw, BookmarksRaw};
use crate::error::AppResult;
use crate::run::process_event;
use crate::state::AppState;
use crate::tasks::AppEvent;
use crate::ui::views::TuiView;
use crate::ui::widgets::TuiMenu;
use crate::ui::AppBackend;
use crate::utils::to_string::ToString;
use crate::utils::unix_helper;

use crate::global::{APP_BOOKMARKS, CONFIG_HIERARCHY};

use super::change_directory::change_directory;

/// Returns the first config-hierarchy directory that already exists, as a fallback location to
/// write a new `bookmarks.toml` when none is found.
fn find_bookmark_file() -> Option<path::PathBuf> {
    for p in CONFIG_HIERARCHY.iter() {
        if p.exists() {
            return Some(p.clone());
        }
    }
    None
}

/// Implements `add_bookmark`: prompts for a key, binds it to the current directory, and
/// persists the updated bookmark list to `bookmarks.toml`.
pub fn add_bookmark(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    let cwd = std::env::current_dir()?;

    let bookmark_path =
        match search_directories(&CONFIG_HIERARCHY, ConfigType::Bookmarks.as_filename()) {
            Some(file_path) => Some(file_path),
            None => find_bookmark_file(),
        };

    if let Some(bookmark_path) = bookmark_path {
        let key = poll_for_bookmark_key(app_state, backend);
        if let Some(key) = key {
            if let Ok(mut bookmark) = APP_BOOKMARKS.lock() {
                bookmark.insert(key, cwd.to_string_lossy().to_string());
            }
            let new_bookmarks_vec: Vec<_> = APP_BOOKMARKS
                .lock()
                .unwrap()
                .clone()
                .drain()
                .map(|(k, v)| BookmarkRaw {
                    key: k.to_string(),
                    path: v,
                })
                .collect();
            let bookmarks_raw = BookmarksRaw {
                bookmark: new_bookmarks_vec,
            };

            if let Ok(content) = toml::to_string(&bookmarks_raw) {
                let mut file = File::create(bookmark_path)?;
                file.write_all(content.as_bytes())?;
            }
        }
    }

    Ok(())
}

/// Implements `cd_bookmark`: prompts for a bookmark key and changes into its bound directory.
pub fn change_directory_bookmark(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    let key = poll_for_bookmark_key(app_state, backend);

    if let Some(key) = key {
        if let Ok(bookmarks) = APP_BOOKMARKS.lock() {
            if let Some(p) = bookmarks.get(&key) {
                let path = unix_helper::expand_shell_string(p);
                change_directory(app_state, &path)?;
            }
        }
    }
    Ok(())
}

/// Renders a menu of existing bookmarks and blocks until the user presses a key, returning it.
fn poll_for_bookmark_key(app_state: &mut AppState, backend: &mut AppBackend) -> Option<Event> {
    app_state.flush_event();

    let mut bookmarks: Vec<String> = APP_BOOKMARKS
        .lock()
        .unwrap()
        .iter()
        .map(|(k, v)| format!("    {}    {:?}", k.to_string(), v))
        .collect();
    bookmarks.sort();
    let bookmarks_str: Vec<&str> = bookmarks.iter().map(|s| s.as_str()).collect();

    let terminal = backend.terminal_mut();
    loop {
        let _ = terminal.draw(|frame| {
            let area = frame.area();
            if area.height < 5 {
                return;
            }
            // redraw view
            {
                let mut view = TuiView::new(app_state);
                view.show_bottom_status = false;
                frame.render_widget(view, area);
            }

            let (menu_widget, menu_y) = if bookmarks_str.len() > area.height as usize - 1 {
                (TuiMenu::new(&bookmarks_str[0..area.height as usize - 1]), 0)
            } else {
                (
                    TuiMenu::new(bookmarks_str.as_slice()),
                    (area.height as usize - bookmarks_str.len() - 1) as u16,
                )
            };

            let menu_rect = Rect {
                x: 0,
                y: menu_y,
                width: area.width,
                height: menu_widget.len() as u16 + 1,
            };
            frame.render_widget(Clear, menu_rect);
            frame.render_widget(menu_widget, menu_rect);
        });

        if let Ok(event) = app_state.poll_event() {
            match event {
                AppEvent::TerminalEvent(key) => return Some(key),
                event => process_event::process_noninteractive(event, app_state),
            };
        }
    }
}
