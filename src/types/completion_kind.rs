#[derive(Clone, Debug)]
pub enum CompletionKind<'a> {
    Bin,
    Custom(Vec<&'a str>),
    Dir(Option<Vec<&'a str>>),
    File,
}
