use std::fs::File;
use std::io::Write;
use std::path;

use ratatui::layout::Rect;
use ratatui::termion::event::Event;
use ratatui::widgets::Clear;

use crate::config::bookmarks::{BookmarkRaw, BookmarksRaw};
use crate::error::AppResult;
use crate::run::process_event;
use crate::traits::config::search_directories;
use crate::traits::ToString;
use crate::types::config_type::ConfigType;
use crate::types::event::AppEvent;
use crate::types::state::AppState;
use crate::ui::views::TuiView;
use crate::ui::widgets::TuiMenu;
use crate::ui::AppBackend;
use crate::utils::unix;

use crate::{BOOKMARKS_T, CONFIG_HIERARCHY};

use super::change_directory::change_directory;

fn find_bookmark_file() -> Option<path::PathBuf> {
    for p in CONFIG_HIERARCHY.iter() {
        if p.exists() {
            return Some(p.clone());
        }
    }
    None
}

pub fn add_bookmark(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    let cwd = std::env::current_dir()?;

    let bookmark_path =
        match search_directories(ConfigType::Bookmarks.as_filename(), &CONFIG_HIERARCHY) {
            Some(file_path) => Some(file_path),
            None => find_bookmark_file(),
        };

    if let Some(bookmark_path) = bookmark_path {
        let key = poll_for_bookmark_key(app_state, backend);
        if let Some(key) = key {
            if let Ok(mut bookmark) = BOOKMARKS_T.lock() {
                bookmark.insert(key, cwd.to_string_lossy().to_string());
            }
            let new_bookmarks_vec: Vec<_> = BOOKMARKS_T
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

pub fn change_directory_bookmark(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    let key = poll_for_bookmark_key(app_state, backend);

    if let Some(key) = key {
        if let Ok(bookmarks) = BOOKMARKS_T.lock() {
            if let Some(p) = bookmarks.get(&key) {
                let path = unix::expand_shell_string(p);
                change_directory(app_state, &path)?;
            }
        }
    }
    Ok(())
}

fn poll_for_bookmark_key(app_state: &mut AppState, backend: &mut AppBackend) -> Option<Event> {
    app_state.flush_event();

    let mut bookmarks: Vec<String> = BOOKMARKS_T
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
