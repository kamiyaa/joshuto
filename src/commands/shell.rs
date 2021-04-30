use std::process;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

use super::reload;

pub fn shell_command(context: &mut AppContext, words: &[String]) -> std::io::Result<()> {
    let mut command = process::Command::new(words[0].clone());
    for word in words.iter().skip(1) {
        match (*word).as_str() {
            "%s" => {
                if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
                    let mut i = 0;
                    for entry in curr_list.selected_entries().map(|e| e.file_name()) {
                        command.arg(entry);
                        i += 1;
                    }
                    if i == 0 {
                        if let Some(entry) = curr_list.curr_entry_ref() {
                            command.arg(entry.file_name());
                        }
                    }
                }
            }
            s => {
                command.arg(s);
            }
        };
    }
    command.status()?;
    Ok(())
}

pub fn shell(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    words: &[String],
) -> JoshutoResult<()> {
    backend.terminal_drop();
    let res = shell_command(context, words);
    reload::soft_reload(context.tab_context_ref().get_index(), context)?;
    context.push_msg(format!("Finished: {}", words.join(" ")));
    backend.terminal_restore()?;
    res?;
    Ok(())
}
