#[derive(Clone, Copy, Debug)]
pub enum TabHomePage {
    Inherit,
    Home,
    Root,
}

impl TabHomePage {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "inherit" => Some(Self::Inherit),
            "home" => Some(Self::Home),
            "root" => Some(Self::Root),
            _ => None,
        }
    }
}
