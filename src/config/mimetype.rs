use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::path::PathBuf;
use std::process;

use super::{parse_config_file, ConfigStructure};
use crate::MIMETYPE_FILE;

const fn default_false() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct JoshutoMimetypeEntry {
    command: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default = "default_false")]
    fork: bool,
    #[serde(default = "default_false")]
    silent: bool,
    #[serde(default = "default_false")]
    confirm_exit: bool,
}

#[allow(dead_code)]
impl JoshutoMimetypeEntry {
    pub fn new(command: String) -> Self {
        Self {
            command,
            args: Vec::new(),
            fork: false,
            silent: false,
            confirm_exit: false,
        }
    }

    pub fn add_arg<S: std::convert::Into<String>>(&mut self, arg: S) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    pub fn add_args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: std::convert::Into<String>,
    {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    pub fn set_fork(&mut self, set: bool) -> &mut Self {
        self.fork = set;
        self
    }

    pub fn set_silent(&mut self, set: bool) -> &mut Self {
        self.silent = set;
        self
    }

    pub fn set_confirm_exit(&mut self, set: bool) -> &mut Self {
        self.confirm_exit = set;
        self
    }

    pub fn get_command(&self) -> &str {
        &self.command
    }

    pub fn get_args(&self) -> &[String] {
        &self.args
    }

    pub fn get_fork(&self) -> bool {
        self.fork
    }

    pub fn get_silent(&self) -> bool {
        self.silent
    }

    pub fn get_confirm_exit(&self) -> bool {
        self.confirm_exit
    }

    pub fn execute_with(&self, paths: &[&PathBuf]) -> std::io::Result<()> {
        let program = String::from(self.get_command());

        let mut command = process::Command::new(program);
        if self.get_silent() {
            command.stdout(process::Stdio::null());
            command.stderr(process::Stdio::null());
        }

        command.args(self.get_args());
        command.args(paths.iter().map(|path| path.as_os_str()));

        let mut handle = command.spawn()?;
        if !self.get_fork() {
            ncurses::savetty();
            ncurses::endwin();
            handle.wait()?;
            if self.get_confirm_exit() {
                println!(" --- Press ENTER to continue --- ");
                std::io::stdin().bytes().next();
            }
            ncurses::resetty();
            ncurses::refresh();
        }
        Ok(())
    }
}

impl std::fmt::Display for JoshutoMimetypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.get_command()).unwrap();
        self.get_args()
            .iter()
            .for_each(|arg| write!(f, " {}", arg).unwrap());

        f.write_str("\t[").unwrap();
        if self.get_fork() {
            f.write_str("fork,").unwrap();
        }
        if self.get_silent() {
            f.write_str("silent").unwrap();
        }
        f.write_str("]")
    }
}

#[derive(Debug, Deserialize)]
pub struct JoshutoMimetype {
    #[serde(default, skip)]
    empty_vec: Vec<JoshutoMimetypeEntry>,
    #[serde(default)]
    pub extension: HashMap<String, Vec<JoshutoMimetypeEntry>>,
    #[serde(default)]
    pub mimetype: HashMap<String, Vec<JoshutoMimetypeEntry>>,
}

impl JoshutoMimetype {
    pub fn get_entries_for_ext(&self, extension: &str) -> &[JoshutoMimetypeEntry] {
        match self.extension.get(extension) {
            Some(s) => s,
            None => &self.empty_vec,
        }
    }
    pub fn get_entries_for_mimetype(&self, mimetype: &str) -> &[JoshutoMimetypeEntry] {
        match self.mimetype.get(mimetype) {
            Some(s) => s,
            None => &self.empty_vec,
        }
    }
}

impl ConfigStructure for JoshutoMimetype {
    fn get_config() -> Self {
        parse_config_file::<JoshutoMimetype>(MIMETYPE_FILE).unwrap_or_else(Self::default)
    }
}

impl std::default::Default for JoshutoMimetype {
    fn default() -> Self {
        JoshutoMimetype {
            empty_vec: Vec::new(),
            mimetype: HashMap::new(),
            extension: HashMap::new(),
        }
    }
}
