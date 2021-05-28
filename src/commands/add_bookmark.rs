use crate::bookmarks;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::error::{JoshutoError, JoshutoErrorKind};
use crate::ui::views::TuiBookmarkMenu;
use crate::ui::TuiBackend;
use termion::event::Event;
use termion::event::Key;

pub fn add_bookmark(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let mut tbm = TuiBookmarkMenu::new();
    match tbm.get_any_char_event(backend, context) {
        Some(Event::Key(Key::Char(c))) => {
            let opt_entry = context
                .tab_context_ref()
                .curr_tab_ref()
                .curr_list_ref()
                .map(|dirlist| dirlist.file_path());

            if let Some(pathbuf) = opt_entry {
                if let Some(dir) = pathbuf.to_str().map(String::from) {
                    let path = std::path::PathBuf::from(dir);
                    let bookmarks = &mut context.bookmarks;
                    bookmarks::insert_bookmark(bookmarks, path, c)?;
                    return Ok(());
                }
            }
        }
        _ => {}
    }

    Err(JoshutoError::new(
        JoshutoErrorKind::UnrecognizedCommand,
        "Bookmark should be a character!".to_string(),
    ))
}
