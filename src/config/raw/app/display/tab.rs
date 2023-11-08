use serde::Deserialize;

fn default_home_page() -> String {
    "home".to_string()
}

#[derive(Clone, Debug, Deserialize)]
pub struct TabOptionRaw {
    #[serde(default = "default_home_page")]
    pub home_page: String,
}

impl std::default::Default for TabOptionRaw {
    fn default() -> Self {
        Self {
            home_page: default_home_page(),
        }
    }
}
