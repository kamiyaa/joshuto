use crate::config::clean::app::display::sort_type::SortType;
use crate::context::AppContext;
use crate::error::AppResult;
use crate::history::DirectoryHistory;

use super::reload;

pub fn set_sort(context: &mut AppContext, method: SortType) -> AppResult {
    let curr_tab = context.tab_context_mut().curr_tab_mut();
    curr_tab
        .option_mut()
        .sort_options_mut()
        .set_sort_method(method);
    curr_tab.history_mut().depreciate_all_entries();
    refresh(context)
}

pub fn toggle_reverse(context: &mut AppContext) -> AppResult {
    let curr_tab = context.tab_context_mut().curr_tab_mut();
    let reversed = !curr_tab.option_mut().sort_options_ref().reverse;
    curr_tab.option_mut().sort_options_mut().reverse = reversed;
    curr_tab.history_mut().depreciate_all_entries();
    refresh(context)
}

fn refresh(context: &mut AppContext) -> AppResult {
    reload::soft_reload_curr_tab(context)?;

    let ui_context = context.ui_context_ref().clone();
    let display_options = context.config_ref().display_options_ref().clone();
    let curr_tab = context.tab_context_mut().curr_tab_mut();

    macro_rules! update_viewport {
        ($x_list_mut: ident) => {
            if let Some(list) = curr_tab.$x_list_mut() {
                list.update_viewport(&ui_context, &display_options);
            }
        };
    }

    update_viewport!(curr_list_mut);
    update_viewport!(parent_list_mut);
    update_viewport!(child_list_mut);

    Ok(())
}
