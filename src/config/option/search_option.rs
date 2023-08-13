use std::str::FromStr;

use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};

/// Search and selection options globally valid for Joshuto (for all tabs)
#[derive(Clone, Debug)]
pub struct SearchOption {
    pub string_case_sensitivity: CaseSensitivity,
    pub glob_case_sensitivity: CaseSensitivity,
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
            fzf_case_sensitivity: CaseSensitivity::Insensitive,
        }
    }
}

impl FromStr for CaseSensitivity {
    type Err = JoshutoError;

    fn from_str(s: &str) -> JoshutoResult<Self> {
        match s {
            "insensitive" => Ok(Self::Insensitive),
            "sensitive" => Ok(Self::Sensitive),
            "smart" => Ok(Self::Smart),
            otherwise => Err(JoshutoError::new(
                JoshutoErrorKind::InvalidParameters,
                format!("Case sensitivity '{otherwise}' unknown"),
            )),
        }
    }
}
