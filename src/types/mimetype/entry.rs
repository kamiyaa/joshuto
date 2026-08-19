use serde::Deserialize;
use std::env;
use std::fmt;

/// A configured program to open a file with, plus how to run it (fork, silent, pager, etc.).
#[derive(Clone, Debug, Deserialize)]
pub struct ProgramEntry {
    #[serde(rename = "command")]
    _command: String,
    #[serde(default, rename = "args")]
    _args: Vec<String>,
    #[serde(default, rename = "fork")]
    _fork: bool,
    #[serde(default, rename = "silent")]
    _silent: bool,
    #[serde(default, rename = "confirm_exit")]
    _confirm_exit: bool,
    #[serde(default, rename = "pager")]
    _pager: bool,
}

impl ProgramEntry {
    /// Creates a program entry that runs `command` with no arguments or special flags.
    pub fn new(command: String) -> Self {
        Self {
            _command: command,
            _args: Vec::new(),
            _fork: false,
            _silent: false,
            _confirm_exit: false,
            _pager: false,
        }
    }

    /// Appends a single argument, for builder-style construction.
    #[allow(dead_code)]
    pub fn arg<S: std::convert::Into<String>>(&mut self, arg: S) -> &mut Self {
        self._args.push(arg.into());
        self
    }

    /// Appends multiple arguments, for builder-style construction.
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: Iterator<Item = S>,
        S: std::convert::Into<String>,
    {
        args.for_each(|arg| self._args.push(arg.into()));
        self
    }

    /// Sets whether the program should be run detached (forked) instead of blocking joshuto.
    #[allow(dead_code)]
    pub fn fork(&mut self, fork: bool) -> &mut Self {
        self._fork = fork;
        self
    }

    /// Sets whether the program's output should be suppressed.
    #[allow(dead_code)]
    pub fn silent(&mut self, silent: bool) -> &mut Self {
        self._silent = silent;
        self
    }

    /// Returns the command to run.
    pub fn get_command(&self) -> &str {
        self._command.as_str()
    }

    /// Returns the configured arguments.
    pub fn get_args(&self) -> &[String] {
        &self._args
    }

    /// Returns `true` if the program should be run detached (forked).
    pub fn get_fork(&self) -> bool {
        self._fork
    }

    /// Returns `true` if the program's output should be suppressed.
    pub fn get_silent(&self) -> bool {
        self._silent
    }

    /// Returns `true` if joshuto should wait for a keypress before returning after this program
    /// exits.
    pub fn get_confirm_exit(&self) -> bool {
        self._confirm_exit
    }

    /// Returns `true` if this program is a pager and should be run accordingly.
    pub fn get_pager(&self) -> bool {
        self._pager
    }

    // TODO: Windows support
    /// Returns `true` if this program's command is found on `$PATH`.
    pub fn program_exists(&self) -> bool {
        let program = self.get_command();
        env::var_os("PATH")
            .map(|path| env::split_paths(&path).any(|dir| dir.join(program).is_file()))
            .unwrap_or(false)
    }
}

impl std::default::Default for ProgramEntry {
    fn default() -> Self {
        Self {
            _command: "".to_string(),
            _args: Vec::new(),
            _fork: false,
            _silent: false,
            _confirm_exit: false,
            _pager: false,
        }
    }
}

impl std::fmt::Display for ProgramEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.get_command()).unwrap();
        self.get_args()
            .iter()
            .for_each(|arg| write!(f, " {}", arg).unwrap());

        f.write_str("        ").unwrap();
        if self.get_fork() {
            f.write_str("[fork]").unwrap();
        }
        if self.get_silent() {
            f.write_str("[silent]").unwrap();
        }
        if self.get_confirm_exit() {
            f.write_str("[confirm-exit]").unwrap();
        }
        if self.get_pager() {
            f.write_str("[pager]").unwrap();
        }
        f.write_str("")
    }
}
