use crate::error::{AppError, AppErrorKind, AppResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineMode {
    pub mode: [LineModeArgs; 8],
    pub size: usize,
}

impl LineMode {
    pub const fn all() -> Self {
        Self {
            mode: [
                LineModeArgs::Size,
                LineModeArgs::ModifyTime,
                LineModeArgs::BirthTime,
                LineModeArgs::AccessTime,
                LineModeArgs::User,
                LineModeArgs::Group,
                LineModeArgs::Permission,
                LineModeArgs::Null,
            ],
            size: 7,
        }
    }

    pub const fn empty() -> Self {
        Self {
            mode: [LineModeArgs::Null; 8],
            size: 0,
        }
    }

    pub fn add_mode(&mut self, mode: LineModeArgs) {
        if self.mode.contains(&mode) {
            return;
        }

        self.mode[self.size] = mode;
        self.size += 1;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineModeArgs {
    Size,
    ModifyTime,
    BirthTime, // unsupport now
    AccessTime,// unsupport now
    User,
    Group,
    Permission,
    #[default]
    Null,
}

impl AsRef<str> for LineModeArgs {
    fn as_ref(&self) -> &str {
        match self {
            LineModeArgs::Size => "size",
            LineModeArgs::ModifyTime => "mtime",
            LineModeArgs::BirthTime => "ctime",
            LineModeArgs::AccessTime => "atime",
            LineModeArgs::User => "user",
            LineModeArgs::Group => "group",
            LineModeArgs::Permission => "perm",
            LineModeArgs::Null => unreachable!(),
        }
    }
}

impl Default for LineMode {
    fn default() -> Self {
        let mut mode = [Default::default(); 8];
        mode[0] = LineModeArgs::Size;

        Self { size: 1, mode }
    }
}

impl LineMode {
    pub fn from_string(name: &str) -> AppResult<LineMode> {
        match name {
            "all" => Ok(LineMode::all()),
            "none" => Ok(LineMode::empty()),
            _ => {
                let mut flags = name.split('|');

                let mut line_mode = LineMode::empty();

                flags.try_for_each(|flag| {
                    match flag.trim() {
                        "size" => line_mode.add_mode(LineModeArgs::Size),
                        "mtime" => line_mode.add_mode(LineModeArgs::ModifyTime),
                        "user" => line_mode.add_mode(LineModeArgs::User),
                        "group" => line_mode.add_mode(LineModeArgs::Group),
                        "perm" => line_mode.add_mode(LineModeArgs::Permission),
                        flag => {
                            return Err(AppError::new(
                                AppErrorKind::InvalidParameters,
                                format!("Linemode '{}' unknown.", flag),
                            ))
                        }
                    }

                    Ok(())
                })?;

                Ok(line_mode)
            }
        }
    }

    pub fn as_string(&self) -> String {
        let modes: Vec<&str> = self
            .mode
            .iter()
            .take(self.size)
            .map(AsRef::as_ref)
            .collect();

        modes.join(" | ")
    }
}
