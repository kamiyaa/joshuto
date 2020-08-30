use std::process;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct ShellCommand {
    pub words: Vec<String>,
}

impl ShellCommand {
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }
    pub const fn command() -> &'static str {
        "console"
    }

    pub fn shell_command(&self, context: &mut JoshutoContext) -> std::io::Result<()> {
        let mut command = process::Command::new(self.words[0].clone());
        for word in self.words.iter().skip(1) {
            match word.as_str() {
                "%s" => {
                    if let Some(curr_list) =
                        context.tab_context_ref().curr_tab_ref().curr_list_ref()
                    {
                        let mut i = 0;
                        for entry in curr_list.selected_entries().map(|e| e.file_name()) {
                            command.arg(entry);
                            i += 1;
                        }
                        if i == 0 {
                            if let Some(entry) = curr_list.get_curr_ref() {
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
}

impl JoshutoCommand for ShellCommand {}

impl std::fmt::Display for ShellCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {:?}", Self::command(), self.words.join(" "))
    }
}

impl JoshutoRunnable for ShellCommand {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        backend.terminal_drop();
        let res = self.shell_command(context);
        ReloadDirList::soft_reload(context.tab_context_ref().get_index(), context)?;
        context.push_msg(format!("Finished: {}", self.words.join(" ")));
        backend.terminal_restore()?;
        res?;
        Ok(())
    }
}
