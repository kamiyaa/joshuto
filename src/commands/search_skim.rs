use std::borrow;
use std::io;
use std::sync;
use std::thread;

use skim::prelude::*;

use crate::commands::cursor_move;
use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::TuiBackend;
use crate::util::search::SearchPattern;

#[derive(Clone, Debug)]
pub struct JoshutoSkimItem {
    pub idx: usize,
    pub value: String,
}

impl SkimItem for JoshutoSkimItem {
    fn text(&self) -> Cow<str> {
        borrow::Cow::Borrowed(self.value.as_str())
    }
}

pub fn search_skim(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .build()
        .unwrap();

    let items = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|list| {
            let v: Vec<JoshutoSkimItem> = list
                .iter()
                .enumerate()
                .map(|(i, e)| JoshutoSkimItem {
                    idx: i,
                    value: e.file_name().to_string(),
                })
                .collect();
            v
        })
        .unwrap_or(vec![]);

    if items.is_empty() {
        return Err(JoshutoError::new(
            JoshutoErrorKind::Io(io::ErrorKind::InvalidData),
            "no files to select".to_string(),
        ));
    }

    let (s, r): (SkimItemSender, SkimItemReceiver) = unbounded();
    let thread = thread::spawn(move || {
        for item in items {
            let _ = s.send(sync::Arc::new(item));
        }
    });

    backend.terminal_drop();

    let skim_output = Skim::run_with(&options, Some(r));

    backend.terminal_restore()?;

    let _ = thread.join();

    if let Some(skim_output) = skim_output {
        if skim_output.final_key == Key::ESC {
            return Ok(());
        }

        let query = skim_output.query;
        if !query.is_empty() {
            context.set_search_context(SearchPattern::String(query));
        }

        for sk_item in skim_output.selected_items {
            let item: Option<&JoshutoSkimItem> =
                (*sk_item).as_any().downcast_ref::<JoshutoSkimItem>();

            match item {
                Some(item) => cursor_move::cursor_move(item.idx, context)?,
                None => {
                    return Err(JoshutoError::new(
                        JoshutoErrorKind::Io(io::ErrorKind::InvalidData),
                        "Error casting".to_string(),
                    ))
                }
            }
        }
    }

    Ok(())
}
