use globset::GlobMatcher;

use crate::config::option::CaseSensitivity;

#[derive(Clone, Debug)]
pub enum SearchContext {
    Glob(GlobMatcher),
    String {
        pattern: String,
        actual_case_sensitivity: CaseSensitivity,
    },
}
