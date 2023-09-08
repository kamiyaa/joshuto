use std::str::FromStr;

use crate::{
    config::raw::app::display::search::SearchOptionRaw,
    error::{AppError, AppErrorKind, AppResult},
};

/// Search and selection options globally valid for Joshuto (for all tabs)
#[derive(Clone, Debug)]
pub struct SearchOption {
    pub string_case_sensitivity: CaseSensitivity,
    pub glob_case_sensitivity: CaseSensitivity,
    pub regex_case_sensitivity: CaseSensitivity,
    pub fzf_case_sensitivity: CaseSensitivity,
}

#[derive(Clone, Copy, Debug)]
pub enum CaseSensitivity {
    Insensitive,
    Sensitive,
    Smart,
}

impl std::default::Default for SearchOption {
    fn default() -> Self {
        Self {
            string_case_sensitivity: CaseSensitivity::Insensitive,
            glob_case_sensitivity: CaseSensitivity::Sensitive,
            regex_case_sensitivity: CaseSensitivity::Sensitive,
            fzf_case_sensitivity: CaseSensitivity::Insensitive,
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

impl From<SearchOptionRaw> for SearchOption {
    fn from(raw: SearchOptionRaw) -> Self {
        let string_case_sensitivity =
            CaseSensitivity::from_str(raw.string_case_sensitivity.as_str())
                .unwrap_or(CaseSensitivity::Insensitive);

        let glob_case_sensitivity = CaseSensitivity::from_str(raw.glob_case_sensitivity.as_str())
            .unwrap_or(CaseSensitivity::Sensitive);

        let regex_case_sensitivity = CaseSensitivity::from_str(raw.regex_case_sensitivity.as_str())
            .unwrap_or(CaseSensitivity::Sensitive);

        let fzf_case_sensitivity = CaseSensitivity::from_str(raw.fzf_case_sensitivity.as_str())
            .unwrap_or(CaseSensitivity::Insensitive);

        Self {
            string_case_sensitivity,
            glob_case_sensitivity,
            regex_case_sensitivity,
            fzf_case_sensitivity,
        }
    }
}
