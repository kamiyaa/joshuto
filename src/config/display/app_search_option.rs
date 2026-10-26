use serde::{Deserialize, Serialize};

use crate::config::CaseSensitivity;

/// Default case-sensitivity for each search mode (plain string, glob, regex, fzf).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchOption {
    #[serde(default = "default_string_case_sensitivity")]
    pub string_case_sensitivity: CaseSensitivity,
    #[serde(default = "default_glob_case_sensitivity")]
    pub glob_case_sensitivity: CaseSensitivity,
    #[serde(default = "default_regex_case_sensitivity")]
    pub regex_case_sensitivity: CaseSensitivity,
    #[serde(default = "default_fzf_case_sensitivity")]
    pub fzf_case_sensitivity: CaseSensitivity,
}

impl std::default::Default for SearchOption {
    fn default() -> Self {
        Self {
            string_case_sensitivity: default_string_case_sensitivity(),
            glob_case_sensitivity: default_glob_case_sensitivity(),
            regex_case_sensitivity: default_regex_case_sensitivity(),
            fzf_case_sensitivity: default_fzf_case_sensitivity(),
        }
    }
}

const fn default_string_case_sensitivity() -> CaseSensitivity {
    CaseSensitivity::Insensitive
}
const fn default_glob_case_sensitivity() -> CaseSensitivity {
    CaseSensitivity::Sensitive
}
const fn default_regex_case_sensitivity() -> CaseSensitivity {
    CaseSensitivity::Sensitive
}
const fn default_fzf_case_sensitivity() -> CaseSensitivity {
    CaseSensitivity::Insensitive
}
