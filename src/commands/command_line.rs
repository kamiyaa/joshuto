use crate::commands::{self, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::textfield::JoshutoTextField;
use crate::ui;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct CommandLine {
    pub prefix: String,
    pub suffix: String,
}

impl CommandLine {
    pub fn new(prefix: String, suffix: String) -> Self {
        CommandLine { prefix, suffix }
    }
    pub const fn command() -> &'static str {
        "console"
    }

    pub fn readline(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        const PROMPT: &str = ":";
        let (term_rows, term_cols) = ui::getmaxyx();
        let user_input: Option<String> = {
            let textfield = JoshutoTextField::new(
                1,
                term_cols,
                (term_rows as usize - 1, 0),
                PROMPT,
                &self.prefix,
                &self.suffix,
            );
            textfield.readline()
        };

        if let Some(s) = user_input {
            let trimmed = s.trim_start();
            match trimmed.find(' ') {
                Some(ind) => {
                    let (command, xs) = trimmed.split_at(ind);
                    let xs = xs.trim_start();
                    let wexp = wordexp::wordexp(xs, wordexp::Wordexp::new(0), 0);
                    let args: Vec<&str> = match wexp.as_ref() {
                        Ok(wexp) => wexp.iter().collect(),
                        Err(_) => Vec::new(),
                    };
                    match commands::from_args(command, &args) {
                        Ok(s) => s.execute(context, view),
                        Err(e) => Err(JoshutoError::Keymap(e)),
                    }
                }
                None => match commands::from_args(trimmed, &Vec::new()) {
                    Ok(s) => s.execute(context, view),
                    Err(e) => Err(JoshutoError::Keymap(e)),
                },
            }
        } else {
            Ok(())
        }
    }
    pub fn readline_with(
        context: &mut JoshutoContext,
        view: &JoshutoView,
        textfield: JoshutoTextField,
    ) -> Result<(), JoshutoError> {
        drop(textfield);
        Ok(())
    }
}

impl JoshutoCommand for CommandLine {}

impl std::fmt::Display for CommandLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {} {}", Self::command(), self.prefix, self.suffix)
    }
}

impl JoshutoRunnable for CommandLine {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let res = self.readline(context, view);
        ncurses::doupdate();
        res
    }
}
