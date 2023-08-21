use super::{AppCommand, Command};

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChangeDirectory { path } => write!(f, "{} {:?}", self.command(), path),
            Self::CommandLine { prefix, suffix } => {
                write!(f, "{} {} || {}", self.command(), prefix, suffix)
            }
            Self::CursorMoveUp { offset } => write!(f, "{} {}", self.command(), offset),
            Self::CursorMoveDown { offset } => write!(f, "{} {}", self.command(), offset),

            Self::SetLineMode(mode) => write!(f, "{} {}", self.command(), mode.as_string()),

            Self::ParentCursorMoveUp { offset } => write!(f, "{} {}", self.command(), offset),
            Self::ParentCursorMoveDown { offset } => write!(f, "{} {}", self.command(), offset),

            Self::PreviewCursorMoveUp { offset } => write!(f, "{} {}", self.command(), offset),
            Self::PreviewCursorMoveDown { offset } => write!(f, "{} {}", self.command(), offset),

            Self::NewDirectory { path } => write!(f, "{} {:?}", self.command(), path),

            Self::SymlinkFiles { relative } => {
                write!(f, "{} --relative={}", self.command(), relative)
            }
            Self::PasteFiles { options } => write!(f, "{}  {}", self.command(), options),
            Self::DeleteFiles {
                background,
                permanently,
                noconfirm,
            } => {
                write!(
                    f,
                    "{}{}{}{}",
                    self.command(),
                    if !background {
                        " --foreground=true"
                    } else {
                        ""
                    },
                    if *permanently { " --permanently" } else { "" },
                    if *noconfirm { " --noconfirm" } else { "" },
                )
            }

            Self::RenameFile { new_name } => write!(f, "{} {:?}", self.command(), new_name),

            Self::SearchGlob { pattern } => write!(f, "{} {}", self.command(), pattern),
            Self::SearchRegex { pattern } => write!(f, "{} {}", self.command(), pattern),
            Self::SearchString { pattern } => write!(f, "{} {}", self.command(), pattern),
            Self::SelectFiles { pattern, options } => {
                write!(f, "{} {} {}", self.command(), pattern, options)
            }
            Self::SubProcess { words, .. } => write!(f, "{} {:?}", self.command(), words),
            Self::Sort(t) => write!(f, "{} {}", self.command(), t),
            Self::TabSwitch { offset } => write!(f, "{} {}", self.command(), offset),
            Self::TabSwitchIndex { index } => write!(f, "{} {}", self.command(), index),
            _ => write!(f, "{}", self.command()),
        }
    }
}
