use super::{AppCommand, Command};

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChangeDirectory(p) => write!(f, "{} {:?}", self.command(), p),
            Self::CommandLine(s, p) => write!(f, "{} {} {}", self.command(), s, p),
            Self::CursorMoveUp(i) => write!(f, "{} {}", self.command(), i),
            Self::CursorMoveDown(i) => write!(f, "{} {}", self.command(), i),

            Self::NewDirectory(d) => write!(f, "{} {:?}", self.command(), d),

            Self::PasteFiles(options) => write!(f, "{}  {}", self.command(), options),
            Self::DeleteFiles { background: false } => {
                write!(f, "{} --foreground=true", self.command(),)
            }

            Self::RenameFile(name) => write!(f, "{} {:?}", self.command(), name),

            Self::SearchGlob(s) => write!(f, "{} {}", self.command(), s),
            Self::SearchString(s) => write!(f, "{} {}", self.command(), s),
            Self::SelectFiles(pattern, options) => {
                write!(f, "{} {} {}", self.command(), pattern, options)
            }
            Self::SubProcess(c, _) => write!(f, "{} {:?}", self.command(), c),
            Self::Sort(t) => write!(f, "{} {}", self.command(), t),
            Self::TabSwitch(i) => write!(f, "{} {}", self.command(), i),
            Self::TabSwitchIndex(i) => write!(f, "{} {}", self.command(), i),
            _ => write!(f, "{}", self.command()),
        }
    }
}
