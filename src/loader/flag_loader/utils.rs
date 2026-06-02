#[derive(PartialEq, Clone, Copy)]
pub enum FlagSection {
    /// This is the variant to be used for debug or internal flags (e.g. GenerateDocs. This will
    /// only generate docs when a specific env var is set. And the user should not worry about it.)
    None,
    Basics,
    Behavior,
    Files,
    Functions,
    Pipe,
}
impl std::fmt::Display for FlagSection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::Basics => write!(f, "BASICS"),
            Self::Behavior => write!(f, "BEHAVIOR"),
            Self::Files => write!(f, "FILES"),
            Self::Functions => write!(f, "FUNCTIONS"),
            Self::Pipe => write!(f, "PIPE"),
        }
    }
}

#[derive(Debug)]
pub enum ParseError<'a> {
    MissingValue(&'static str),
    UnknownFlag(&'a str),
}

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MissingValue(flag) => write!(f, "flag `{flag}` requires a value"),
            Self::UnknownFlag(flag) => write!(f, "unknown flag `{flag}`"),
        }
    }
}
