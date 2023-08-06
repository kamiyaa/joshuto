use globset::{GlobBuilder, GlobMatcher};

use crate::config::option::CaseSensitivity;
use crate::error::JoshutoResult;

#[derive(Clone, Debug)]
pub enum SearchContext {
    Glob(GlobMatcher),
    String {
        pattern: String,
        actual_case_sensitivity: CaseSensitivity,
    },
}

impl SearchContext {
    pub fn new_glob(pattern: &str, case_sensitivity: CaseSensitivity) -> JoshutoResult<Self> {
        let pattern_lower = pattern.to_lowercase();

        let (pattern, actual_case_sensitivity) = match case_sensitivity {
            CaseSensitivity::Insensitive => (pattern_lower.as_str(), CaseSensitivity::Insensitive),
            CaseSensitivity::Sensitive => (pattern, CaseSensitivity::Sensitive),
            // Determine the actual case sensitivity by whether an uppercase letter occurs.
            CaseSensitivity::Smart => {
                if pattern_lower == pattern {
                    (pattern_lower.as_str(), CaseSensitivity::Insensitive)
                } else {
                    (pattern, CaseSensitivity::Sensitive)
                }
            }
        };

        let glob = GlobBuilder::new(pattern)
            .case_insensitive(matches!(
                actual_case_sensitivity,
                CaseSensitivity::Insensitive
            ))
            .build()?
            .compile_matcher();

        Ok(Self::Glob(glob))
    }

    pub fn new_string(pattern: &str, case_sensitivity: CaseSensitivity) -> Self {
        let (pattern, actual_case_sensitivity) = match case_sensitivity {
            CaseSensitivity::Insensitive => (pattern.to_lowercase(), CaseSensitivity::Insensitive),
            CaseSensitivity::Sensitive => (pattern.to_string(), CaseSensitivity::Sensitive),
            // Determine the actual case sensitivity by whether an uppercase letter occurs.
            CaseSensitivity::Smart => {
                if pattern.chars().all(|c| c.is_lowercase()) {
                    // All characters in `pattern` is lowercase
                    (pattern.to_string(), CaseSensitivity::Insensitive)
                } else {
                    (pattern.to_string(), CaseSensitivity::Sensitive)
                }
            }
        };

        Self::String {
            pattern,
            actual_case_sensitivity,
        }
    }
}
