use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

pub fn lazy_load_directory_size(context: &mut AppContext) {
    let directory_size = match context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|l| l.curr_entry_ref())
    {
        Some(curr_entry) => {
            if let Some(size) = curr_entry.metadata.directory_size() {
                let history = context.tab_context_ref().curr_tab_ref().history_ref();
                match history.get(curr_entry.file_path()).map(|d| d.len()) {
                    Some(len) if size != len => Some(len),
                    _ => None,
                }
            } else if !curr_entry.metadata.file_type().is_dir() {
                None
            } else {
                let history = context.tab_context_ref().curr_tab_ref().history_ref();
                history.get(curr_entry.file_path()).map(|d| d.len())
            }
        }
        None => None,
    };

    if let Some(s) = directory_size {
        if let Some(curr_entry) = context
            .tab_context_mut()
            .curr_tab_mut()
            .curr_list_mut()
            .and_then(|l| l.curr_entry_mut())
        {
            curr_entry.metadata.update_directory_size(s);
        }
    }
}

pub fn cursor_move(context: &mut AppContext, new_index: usize) {
    lazy_load_directory_size(context);
    let mut new_index = new_index;
    let ui_context = context.ui_context_ref().clone();
    let display_options = context.config_ref().display_options_ref().clone();
    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        if !curr_list.is_empty() {
            let dir_len = curr_list.len();
            if new_index >= dir_len {
                new_index = dir_len - 1;
            }
            curr_list.set_index(Some(new_index), &ui_context, &display_options);
        }
    }
}

pub fn up(context: &mut AppContext, u: usize) -> JoshutoResult {
    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => curr_list
            .get_index()
            .map(|idx| if idx > u { idx - u } else { 0 }),
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(context, s);
    }
    Ok(())
}

pub fn down(context: &mut AppContext, u: usize) -> JoshutoResult {
    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => curr_list.get_index().map(|idx| idx + u),
        None => None,
    };
    if let Some(s) = movement {
        cursor_move(context, s);
    }
    Ok(())
}

pub fn home(context: &mut AppContext) -> JoshutoResult {
    let movement: Option<usize> = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            let len = curr_list.len();
            if len == 0 {
                None
            } else {
                Some(0)
            }
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(context, s);
    }
    Ok(())
}

pub fn end(context: &mut AppContext) -> JoshutoResult {
    let movement: Option<usize> = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            let len = curr_list.len();
            if len == 0 {
                None
            } else {
                Some(len - 1)
            }
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(context, s);
    }
    Ok(())
}

fn get_page_size(context: &AppContext, backend: &TuiBackend) -> Option<usize> {
    let config = context.config_ref();
    let rect = backend.terminal.as_ref().map(|t| t.size())?.ok()?;

    let rect_height = rect.height as usize;
    if config.display_options_ref().show_borders() {
        if rect_height >= 4 {
            Some(rect_height - 4)
        } else {
            None
        }
    } else if rect_height >= 2 {
        Some(rect_height - 2)
    } else {
        None
    }
}

pub fn page_up(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    proportion: f64,
) -> JoshutoResult {
    let page_size = get_page_size(context, backend).unwrap_or(10) as f64 * proportion;
    let page_size = page_size as usize;

    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            curr_list
                .get_index()
                .map(|idx| if idx > page_size { idx - page_size } else { 0 })
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(context, s);
    }
    Ok(())
}

pub fn page_down(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    proportion: f64,
) -> JoshutoResult {
    let page_size = get_page_size(context, backend).unwrap_or(10) as f64 * proportion;
    let page_size = page_size as usize;

    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            let dir_len = curr_list.len();
            curr_list.get_index().map(|idx| {
                if idx + page_size > dir_len - 1 {
                    dir_len - 1
                } else {
                    idx + page_size
                }
            })
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(context, s);
    }
    Ok(())
}
