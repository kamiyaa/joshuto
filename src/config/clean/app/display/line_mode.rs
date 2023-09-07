use serde::Deserialize;

use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
    #[serde(transparent)]
    pub struct LineMode: u8 {
        const size  = 1 << 0;
        const mtime = 1 << 1;
        const user  = 1 << 2;
        const group = 1 << 3;
        const perm  = 1 << 4;
    }
}

impl Default for LineMode {
    fn default() -> Self {
        Self::size
    }
}

impl LineMode {
    pub fn from_string(name: &str) -> JoshutoResult<LineMode> {
        match name {
            "all" => Ok(LineMode::all()),
            "none" => Ok(LineMode::empty()),
            _ => {
                let mut flags = name.split('|');

                let mut linemode = LineMode::empty();

                flags.try_for_each(|flag| {
                    match flag.trim() {
                        "size" => linemode |= LineMode::size,
                        "mtime" => linemode |= LineMode::mtime,
                        "user" => linemode |= LineMode::user,
                        "group" => linemode |= LineMode::group,
                        "perm" => linemode |= LineMode::perm,
                        flag => {
                            return Err(JoshutoError::new(
                                JoshutoErrorKind::InvalidParameters,
                                format!("Linemode '{}' unknown.", flag),
                            ))
                        }
                    }

                    Ok(())
                })?;

                Ok(linemode)
            }
        }
    }

    pub fn as_string(&self) -> String {
        self.iter_names()
            .map(|f| f.0)
            .collect::<Vec<_>>()
            .join(" | ")
    }
}
