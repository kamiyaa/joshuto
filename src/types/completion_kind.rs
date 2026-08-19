/// What kind of values to offer when tab-completing a command's argument.
#[derive(Clone, Debug)]
pub enum CompletionKind<'a> {
    /// Complete against executables on `$PATH`.
    Bin,
    /// Complete against a fixed list of values.
    Custom(Vec<&'a str>),
    /// Complete against directories, optionally restricted to a subset of paths.
    Dir(Option<Vec<&'a str>>),
    /// Complete against files.
    File,
}
