use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::config::option::CaseSensitivity;

#[derive(Clone, Debug, Default)]
pub enum FilterContext {
    String {
        pattern: String,
        actual_case_sensitivity: CaseSensitivity,
    },
    #[default]
    None,
}

impl FilterContext {
    pub fn new_string(pattern: &str, case_sensitivity: CaseSensitivity) -> Self {
        let pattern_lower = pattern.to_lowercase();

        let (pattern, actual_case_sensitivity) = match case_sensitivity {
            CaseSensitivity::Insensitive => (pattern_lower, CaseSensitivity::Insensitive),
            CaseSensitivity::Sensitive => (pattern.to_string(), CaseSensitivity::Sensitive),
            // Determine the actual case sensitivity by whether an uppercase letter occurs.
            CaseSensitivity::Smart => {
                if pattern_lower == pattern {
                    (pattern_lower, CaseSensitivity::Insensitive)
                } else {
                    (pattern.to_string(), CaseSensitivity::Sensitive)
                }
            }
        };

        FilterContext::String {
            pattern,
            actual_case_sensitivity,
        }
    }

    pub fn filter(&self, main: &str) -> bool {
        match self {
            Self::None => true,
            Self::String {
                pattern,
                actual_case_sensitivity,
            } => Self::filter_string(main, pattern, *actual_case_sensitivity),
        }
    }

    fn filter_string(main: &str, pattern: &str, actual_case_sensitivity: CaseSensitivity) -> bool {
        match actual_case_sensitivity {
            CaseSensitivity::Insensitive => main.to_lowercase().contains(pattern),
            CaseSensitivity::Sensitive => main.contains(pattern),
            CaseSensitivity::Smart => unreachable!(),
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, FilterContext::None)
    }
}

impl Display for FilterContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::String { pattern, .. } => write!(f, "{pattern}"),
            Self::None => Ok(()),
        }
    }
}
