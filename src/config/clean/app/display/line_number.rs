#[derive(Clone, Copy, Debug)]
pub enum LineNumberStyle {
    None,
    Relative,
    Absolute,
}

impl LineNumberStyle {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "absolute" => Some(Self::Absolute),
            "relative" => Some(Self::Relative),
            "none" => Some(Self::None),
            _ => None,
        }
    }
}
