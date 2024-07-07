use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CustomCommand {
    pub name: String,
    pub command: String,
}
