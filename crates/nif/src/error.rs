use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    /// The file doesn't start with a Gamebryo/NetImmerse header line.
    NotANif {
        found: String,
    },
    /// A NIF, but from a version or game this reader doesn't handle.
    Unsupported(String),
    /// The file's structure is inconsistent.
    Malformed {
        offset: usize,
        reason: String,
    },
    /// An error while decoding one block.
    InBlock {
        index: usize,
        type_name: String,
        source: Box<Error>,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::NotANif { found } => write!(
                f,
                "not a NIF file: it starts with {found:?} instead of a Gamebryo header"
            ),
            Error::Unsupported(what) => write!(f, "unsupported NIF: {what}"),
            Error::Malformed { offset, reason } => {
                write!(f, "malformed NIF at offset {offset:#x}: {reason}")
            }
            Error::InBlock {
                index,
                type_name,
                source,
            } => write!(f, "in block {index} ({type_name}): {source}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::InBlock { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
