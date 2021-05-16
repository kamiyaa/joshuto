use serde_derive::Deserialize;
use std::fmt;
use std::io::Read;
use std::process;

#[derive(Clone, Debug, Deserialize)]
pub struct AppList {
    #[serde(default, rename = "inherit")]
    _inherit: String,
    #[serde(default, rename = "app_list")]
    _app_list: Vec<AppMimetypeEntry>,
}

impl AppList {
    pub fn new(_inherit: String, _app_list: Vec<AppMimetypeEntry>) -> Self {
        Self {
            _inherit,
            _app_list,
        }
    }

    pub fn parent(&self) -> &str {
        self._inherit.as_str()
    }

    pub fn app_list(&self) -> &[AppMimetypeEntry] {
        &self._app_list.as_slice()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppMimetypeEntry {
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
}

impl AppMimetypeEntry {
    pub fn new(command: String) -> Self {
        Self {
            _command: command,
            _args: Vec::new(),
            _fork: false,
            _silent: false,
            _confirm_exit: false,
        }
    }

    #[allow(dead_code)]
    pub fn arg<S: std::convert::Into<String>>(&mut self, arg: S) -> &mut Self {
        self._args.push(arg.into());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: Iterator<Item = S>,
        S: std::convert::Into<String>,
    {
        args.for_each(|arg| self._args.push(arg.into()));
        self
    }

    #[allow(dead_code)]
    pub fn fork(&mut self, fork: bool) -> &mut Self {
        self._fork = fork;
        self
    }

    #[allow(dead_code)]
    pub fn silent(&mut self, silent: bool) -> &mut Self {
        self._silent = silent;
        self
    }

    #[allow(dead_code)]
    pub fn confirm_exit(&mut self, confirm_exit: bool) -> &mut Self {
        self._confirm_exit = confirm_exit;
        self
    }

    pub fn get_command(&self) -> &str {
        self._command.as_str()
    }

    pub fn get_args(&self) -> &[String] {
        &self._args
    }

    pub fn get_fork(&self) -> bool {
        self._fork
    }

    pub fn get_silent(&self) -> bool {
        self._silent
    }

    pub fn get_confirm_exit(&self) -> bool {
        self._confirm_exit
    }

    pub fn execute_with<I, S>(&self, paths: I) -> std::io::Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let program = String::from(self.get_command());

        let mut command = process::Command::new(program);
        if self.get_silent() {
            command.stdout(process::Stdio::null());
            command.stderr(process::Stdio::null());
        }

        command.args(self.get_args());
        command.args(paths);

        let mut handle = command.spawn()?;
        if !self.get_fork() {
            handle.wait()?;
            if self.get_confirm_exit() {
                println!(" --- Press ENTER to continue --- ");
                std::io::stdin().bytes().next();
            }
        }
        Ok(())
    }
}

impl std::default::Default for AppMimetypeEntry {
    fn default() -> Self {
        Self {
            _command: "".to_string(),
            _args: Vec::new(),
            _fork: false,
            _silent: false,
            _confirm_exit: false,
        }
    }
}

impl std::fmt::Display for AppMimetypeEntry {
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
        f.write_str("")
    }
}
