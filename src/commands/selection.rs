use crate::commands::{CursorMoveDown, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

#[derive(Debug, Clone)]
pub struct SelectFiles {
    toggle: bool,
    all: bool,
}

impl SelectFiles {
    pub fn new(toggle: bool, all: bool) -> Self {
        SelectFiles { toggle, all }
    }
    pub const fn command() -> &'static str {
        "select_files"
    }
}

impl JoshutoCommand for SelectFiles {}

impl std::fmt::Display for SelectFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command()).unwrap();
        if self.toggle {
            f.write_str(" --toggle").unwrap();
        }
        if self.all {
            f.write_str(" --all").unwrap();
        }
        f.write_str("")
    }
}

impl JoshutoRunnable for SelectFiles {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        if self.toggle {
            if !self.all {
                if let Some(s) = context
                    .tab_context_mut()
                    .curr_tab_mut()
                    .curr_list_mut()
                    .and_then(|s| s.get_curr_mut())
                {
                    s.set_selected(!s.is_selected());
                    CursorMoveDown::new(1).execute(context, backend)?;
                }
            } else {
                if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
                    for curr in &mut curr_list.contents {
                        curr.set_selected(!curr.is_selected());
                    }
                }
            }
        } else if !self.all {
            if let Some(s) = context
                .tab_context_mut()
                .curr_tab_mut()
                .curr_list_mut()
                .and_then(|s| s.get_curr_mut())
            {
                s.set_selected(!s.is_selected());
                CursorMoveDown::new(1).execute(context, backend)?;
            }
        } else {
            if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
                for curr in &mut curr_list.contents {
                    curr.set_selected(true);
                }
            }
        }
        Ok(())
    }
}
