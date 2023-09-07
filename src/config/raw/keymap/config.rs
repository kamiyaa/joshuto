use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CommandKeymap {
    pub keys: Vec<String>,
    pub commands: Vec<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppModeKeyMapping {
    #[serde(default)]
    pub keymap: Vec<CommandKeymap>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppKeyMappingRaw {
    pub default_view: AppModeKeyMapping,
    pub task_view: AppModeKeyMapping,
    pub help_view: AppModeKeyMapping,
}
