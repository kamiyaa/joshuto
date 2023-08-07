use std::fmt::{Display, Formatter, Result as FmtResult};

use globset::{GlobBuilder, GlobMatcher};

use crate::config::option::CaseSensitivity;
use crate::error::JoshutoResult;

#[derive(Clone, Debug, Default)]
pub enum MatchContext {
    Glob(GlobMatcher),
    String {
        pattern: String,
        actual_case_sensitivity: CaseSensitivity,
    },
    #[default]
    None,
}

impl MatchContext {
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

    pub fn is_match(&self, main: &str) -> bool {
        match self {
            Self::Glob(glob_matcher) => Self::is_match_glob(main, glob_matcher),
            Self::String {
                pattern,
                actual_case_sensitivity,
            } => Self::is_match_string(main, pattern, *actual_case_sensitivity),
            Self::None => true,
        }
    }

    fn is_match_glob(main: &str, glob_matcher: &GlobMatcher) -> bool {
        glob_matcher.is_match(main)
    }

    fn is_match_string(
        main: &str,
        pattern: &str,
        actual_case_sensitivity: CaseSensitivity,
    ) -> bool {
        match actual_case_sensitivity {
            CaseSensitivity::Insensitive => main.to_lowercase().contains(pattern),
            CaseSensitivity::Sensitive => main.contains(pattern),
            CaseSensitivity::Smart => unreachable!(),
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl Display for MatchContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Glob(glob_matcher) => write!(f, "{}", glob_matcher.glob().glob()),
            Self::String { pattern, .. } => write!(f, "{pattern}"),
            Self::None => Ok(()),
        }
    }
}
