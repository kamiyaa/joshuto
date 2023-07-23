use std::fs::File;
use std::io::Write;
use std::path;

use ratatui::layout::Rect;
use ratatui::widgets::Clear;
use termion::event::Event;

use crate::config::{search_directories, BookmarkRaw, BookmarksRaw};
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::event::{process_event, AppEvent};
use crate::traits::ToString;
use crate::ui::views::TuiView;
use crate::ui::widgets::TuiMenu;
use crate::ui::AppBackend;
use crate::util::unix;

use crate::{BOOKMARKS_FILE, BOOKMARKS_T, CONFIG_HIERARCHY};

use super::change_directory::change_directory;

fn find_bookmark_file() -> Option<path::PathBuf> {
    for p in CONFIG_HIERARCHY.iter() {
        if p.exists() {
            return Some(p.clone());
        }
    }
    None
}

pub fn add_bookmark(context: &mut AppContext, backend: &mut AppBackend) -> JoshutoResult {
    let cwd = std::env::current_dir()?;

    let bookmark_path = match search_directories(BOOKMARKS_FILE, &CONFIG_HIERARCHY) {
        Some(file_path) => Some(file_path),
        None => find_bookmark_file(),
    };

    if let Some(bookmark_path) = bookmark_path {
        let key = poll_for_bookmark_key(context, backend);
        if let Some(key) = key {
            if let Ok(mut bookmark) = BOOKMARKS_T.lock() {
                bookmark.insert(key, cwd.to_string_lossy().to_string());
            }
            let new_bookmarks_vec: Vec<BookmarkRaw> = BOOKMARKS_T
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

pub fn change_directory_bookmark(
    context: &mut AppContext,
    backend: &mut AppBackend,
) -> JoshutoResult {
    let key = poll_for_bookmark_key(context, backend);

    if let Some(key) = key {
        if let Ok(bookmarks) = BOOKMARKS_T.lock() {
            if let Some(p) = bookmarks.get(&key) {
                let path = unix::expand_shell_string(p);
                change_directory(context, &path)?;
            }
        }
    }
    Ok(())
}

fn poll_for_bookmark_key(context: &mut AppContext, backend: &mut AppBackend) -> Option<Event> {
    context.flush_event();

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
            let area = frame.size();
            if area.height < 5 {
                return;
            }
            // redraw view
            {
                let mut view = TuiView::new(context);
                view.show_bottom_status = false;
                frame.render_widget(view, area);
            }

            let menu_widget = TuiMenu::new(bookmarks_str.as_slice());
            let menu_len = menu_widget.len();
            let menu_y = if menu_len + 1 > area.height as usize {
                0
            } else {
                (area.height as usize - menu_len - 1) as u16
            };

            let menu_rect = Rect {
                x: 0,
                y: menu_y - 1,
                width: area.width,
                height: menu_len as u16 + 1,
            };
            frame.render_widget(Clear, menu_rect);
            frame.render_widget(menu_widget, menu_rect);
        });

        if let Ok(event) = context.poll_event() {
            match event {
                AppEvent::Termion(key) => return Some(key),
                event => process_event::process_noninteractive(event, context),
            };
        }
    }
}
