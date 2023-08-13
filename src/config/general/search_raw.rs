use std::convert::From;
use std::str::FromStr;

use serde_derive::Deserialize;

use crate::config::option::{CaseSensitivity, SearchOption};

fn default_string_case_sensitivity() -> String {
    "insensitive".to_string()
}

fn default_glob_case_sensitivity() -> String {
    "sensitive".to_string()
}

fn default_fzf_case_sensitivity() -> String {
    "insensitive".to_string()
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchOptionRaw {
    #[serde(default = "default_string_case_sensitivity")]
    pub string_case_sensitivity: String,

    #[serde(default = "default_glob_case_sensitivity")]
    pub glob_case_sensitivity: String,

    #[serde(default = "default_fzf_case_sensitivity")]
    pub fzf_case_sensitivity: String,
}

impl std::default::Default for SearchOptionRaw {
    fn default() -> Self {
        SearchOptionRaw {
            string_case_sensitivity: default_string_case_sensitivity(),
            glob_case_sensitivity: default_glob_case_sensitivity(),
            fzf_case_sensitivity: default_fzf_case_sensitivity(),
        }
    }
}

impl From<SearchOptionRaw> for SearchOption {
    fn from(raw: SearchOptionRaw) -> Self {
        let string_case_sensitivity =
            CaseSensitivity::from_str(raw.string_case_sensitivity.as_str())
                .unwrap_or(CaseSensitivity::Insensitive);

        let glob_case_sensitivity = CaseSensitivity::from_str(raw.glob_case_sensitivity.as_str())
            .unwrap_or(CaseSensitivity::Sensitive);

        let fzf_case_sensitivity = CaseSensitivity::from_str(raw.fzf_case_sensitivity.as_str())
            .unwrap_or(CaseSensitivity::Insensitive);

        Self {
            string_case_sensitivity,
            glob_case_sensitivity,
            fzf_case_sensitivity,
        }
    }
}
