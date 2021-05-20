use crate::bookmarks;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::views::TuiBookmarkMenu;
use crate::ui::TuiBackend;
use termion::event::Event;
use termion::event::Key;
// use crate::error::{JoshutoError, JoshutoErrorKind};

pub fn add_bookmark(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let mut tbm = TuiBookmarkMenu::new();
    let maybe_char = tbm.get_any_char(backend, context);

    match maybe_char {
        Some(Event::Key(Key::Char(c))) => {
            let opt_entry = context
                .tab_context_ref()
                .curr_tab_ref()
                .curr_list_ref()
                .map(|dirlist| dirlist.file_path());

            if let Some(pathbuf) = opt_entry {
                if let Some(dir) = pathbuf.to_str().map(|s| String::from(s)) {
                    let path = std::path::PathBuf::from(dir);
                    let event = Event::Key(Key::Char(c));
                    let bookmarks = &mut context.bookmarks;
                    bookmarks::insert_bookmark(bookmarks, path, event)?;
                    return Ok(());
                }
            }
        }
        _ => return Ok(()),
    }
    Ok(())
}
