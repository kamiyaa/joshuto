use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};

#[derive(Clone, Debug, Copy)]
pub enum LineMode {
    Size,
    MTime,
    SizeMTime,
}

impl LineMode {
    pub fn from_string(name: &str) -> JoshutoResult<LineMode> {
        match name {
            "size" => Ok(LineMode::Size),
            "mtime" => Ok(LineMode::MTime),
            "sizemtime" => Ok(LineMode::SizeMTime),
            _ => Err(JoshutoError::new(
                JoshutoErrorKind::InvalidParameters,
                format!("Linemode '{}' unknown.", name),
            )),
        }
    }
}
