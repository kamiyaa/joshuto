use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppErrorKind, AppResult};

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

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum CaseSensitivity {
    #[default]
    #[serde(rename = "sensitive")]
    Sensitive,
    #[serde(rename = "insensitive")]
    Insensitive,
    #[serde(rename = "smart")]
    Smart,
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

impl FromStr for CaseSensitivity {
    type Err = AppError;

    fn from_str(s: &str) -> AppResult<Self> {
        match s {
            "insensitive" => Ok(Self::Insensitive),
            "sensitive" => Ok(Self::Sensitive),
            "smart" => Ok(Self::Smart),
            otherwise => Err(AppError::new(
                AppErrorKind::InvalidParameters,
                format!("Case sensitivity '{otherwise}' unknown"),
            )),
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
