use crate::commands::{self, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::textfield::JoshutoTextField;
use crate::ui;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct CommandLine {
    prefix: Option<String>,
}

impl CommandLine {
    pub fn new(prefix: Option<String>) -> Self {
        CommandLine { prefix }
    }
    pub const fn command() -> &'static str {
        "console"
    }

    pub fn readline(
        context: &mut JoshutoContext,
        view: &JoshutoView,
        prefix: Option<&String>,
    ) -> Result<(), JoshutoError> {
        const PROMPT: &str = ":";
        let (term_rows, term_cols) = ui::getmaxyx();
        let user_input: Option<String> = {
            let textfield = JoshutoTextField::new(
                1,
                term_cols,
                (term_rows as usize - 1, 0),
                PROMPT.to_string(),
            );
            match prefix {
                Some(s) => textfield.readline_with_initial((s, "")),
                None => textfield.readline(),
            }
        };

        if let Some(s) = user_input {
            let trimmed = s.trim_start();
            match trimmed.find(' ') {
                Some(ind) => {
                    let (command, xs) = trimmed.split_at(ind);
                    let xs = xs.trim_start();
                    match commands::from_args(command, xs) {
                        Ok(s) => s.execute(context, view),
                        Err(e) => Err(JoshutoError::Keymap(e)),
                    }
                }
                None => match commands::from_args(trimmed, "") {
                    Ok(s) => s.execute(context, view),
                    Err(e) => Err(JoshutoError::Keymap(e)),
                },
            }
        } else {
            Ok(())
        }
    }
}

impl JoshutoCommand for CommandLine {}

impl std::fmt::Display for CommandLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.prefix.as_ref() {
            Some(s) => write!(f, "{}: {}", Self::command(), s),
            None => write!(f, "{}", Self::command()),
        }
    }
}

impl JoshutoRunnable for CommandLine {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let res = Self::readline(context, view, self.prefix.as_ref());
        ncurses::doupdate();
        res
    }
}
