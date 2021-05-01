use globset::GlobMatcher;

#[derive(Clone, Debug)]
pub enum SearchPattern {
    Glob(GlobMatcher),
    String(String),
}
