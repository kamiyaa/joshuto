use crate::context::JoshutoContext;
use crate::error::JoshutoResult;

use super::cursor_move;

pub fn select_files(context: &mut JoshutoContext, toggle: bool, all: bool) -> JoshutoResult<()> {
    if toggle {
        if !all {
            if let Some(s) = context
                .tab_context_mut()
                .curr_tab_mut()
                .curr_list_mut()
                .and_then(|s| s.curr_entry_mut())
            {
                s.set_selected(!s.is_selected());
                cursor_move::down(context, 1)?;
            }
        } else if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
            for curr in &mut curr_list.contents {
                curr.set_selected(!curr.is_selected());
            }
        }
    } else if !all {
        if let Some(s) = context
            .tab_context_mut()
            .curr_tab_mut()
            .curr_list_mut()
            .and_then(|s| s.curr_entry_mut())
        {
            s.set_selected(!s.is_selected());
            cursor_move::down(context, 1)?;
        }
    } else if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        curr_list
            .contents
            .iter_mut()
            .for_each(|c| c.set_selected(true));
    }
    Ok(())
}
