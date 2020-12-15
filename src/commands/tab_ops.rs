use std::path;

use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::tab::JoshutoTab;
use crate::util::load_child::LoadChild;

use crate::HOME_DIR;

use super::quit;

fn _tab_switch(new_index: usize, context: &mut JoshutoContext) -> std::io::Result<()> {
    context.tab_context_mut().set_index(new_index);
    let path = context.tab_context_ref().curr_tab_ref().pwd().to_path_buf();
    std::env::set_current_dir(path.as_path())?;

    let options = context.config_ref().sort_option.clone();
    context
        .tab_context_mut()
        .curr_tab_mut()
        .history_mut()
        .create_or_soft_update(path.as_path(), &options)?;
    Ok(())
}

pub fn tab_switch(offset: i32, context: &mut JoshutoContext) -> std::io::Result<()> {
    let index = context.tab_context_ref().get_index();
    let num_tabs = context.tab_context_ref().len();
    let new_index = (index as i32 + num_tabs as i32 + offset) as usize % num_tabs;

    _tab_switch(new_index, context)
}

pub fn new_tab(context: &mut JoshutoContext) -> JoshutoResult<()> {
    /* start the new tab in $HOME or root */
    let curr_path = match HOME_DIR.as_ref() {
        Some(s) => s.clone(),
        None => path::PathBuf::from("/"),
    };

    let tab = JoshutoTab::new(curr_path, &context.config_ref().sort_option)?;
    context.tab_context_mut().push_tab(tab);
    let new_index = context.tab_context_ref().len() - 1;
    context.tab_context_mut().set_index(new_index);
    _tab_switch(new_index, context)?;
    LoadChild::load_child(context)?;
    Ok(())
}

pub fn close_tab(context: &mut JoshutoContext) -> JoshutoResult<()> {
    if context.tab_context_ref().len() <= 1 {
        return quit::quit(context);
    }
    let mut tab_index = context.tab_context_ref().get_index();

    let _ = context.tab_context_mut().pop_tab(tab_index);
    let num_tabs = context.tab_context_ref().len();
    if tab_index >= num_tabs {
        tab_index = num_tabs - 1;
    }
    _tab_switch(tab_index, context)?;
    Ok(())
}
