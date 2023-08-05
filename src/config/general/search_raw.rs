use std::convert::From;
use std::str::FromStr;

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
    fn from(raw: SearchOptionRaw) -> Self {
        let case_sensitivity = CaseSensitivity::from_str(raw.case_sensitivity.as_str())
            .unwrap_or(CaseSensitivity::Insensitive);

        Self {
            _case_sensitivity: case_sensitivity,
        }
    }
}
