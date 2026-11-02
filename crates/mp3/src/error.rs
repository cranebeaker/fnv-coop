use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// No MPEG audio frames were found in the data.
    NoFrames,
    /// MPEG audio this decoder doesn't handle (Layer I or II).
    Unsupported(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoFrames => write!(f, "no MP3 frames found"),
            Error::Unsupported(what) => write!(f, "unsupported audio: {what}"),
        }
    }
}

impl std::error::Error for Error {}
