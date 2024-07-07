use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub enum LineNumberStyle {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}
