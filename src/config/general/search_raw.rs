use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::{CaseSensitivity, SearchOption};

fn default_case_sensitivity() -> String {
    "insensitive".to_string()
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchOptionRaw {
    #[serde(default = "default_case_sensitivity")]
    pub case_sensitivity: String,
}

impl std::default::Default for SearchOptionRaw {
    fn default() -> Self {
        SearchOptionRaw {
            case_sensitivity: default_case_sensitivity(),
        }
    }
}

impl From<SearchOptionRaw> for SearchOption {
    fn from(raww: SearchOptionRaw) -> Self {
        let case_sensitivity = match raww.case_sensitivity.as_str() {
            "sensitive" => CaseSensitivity::Sensitive,
            "smart" => CaseSensitivity::Smart,
            _ => CaseSensitivity::Insensitive,
        };

        Self {
            _case_sensitivity: case_sensitivity,
        }
    }
}
