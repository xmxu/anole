
use std::error::Error as StdError;
pub(crate) type InnerError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub(crate) enum Kind {
    ParseValue
}

pub struct Error {
    kind: Kind,
    inner: Option<InnerError>
}

impl Error {
    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> Error where E: Into<InnerError> {
        Error {kind, inner: source.map(Into::into)}
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("anole::Error").field("kind", &self.kind).field("source", &self.inner).finish()
    }
}

pub(crate) fn parse_value(e: InnerError) -> Error {
    Error::new(Kind::ParseValue, Some(e))
}