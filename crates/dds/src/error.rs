use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    /// The data doesn't start with `DDS `.
    NotADds {
        found: [u8; 4],
    },
    /// A DDS in a format or layout this reader doesn't handle.
    Unsupported(String),
    /// The header and data disagree.
    Malformed(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::NotADds { found } => write!(
                f,
                "not a DDS texture: it starts with {:?} instead of \"DDS \"",
                String::from_utf8_lossy(found)
            ),
            Error::Unsupported(what) => write!(f, "unsupported texture: {what}"),
            Error::Malformed(what) => write!(f, "malformed texture: {what}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
