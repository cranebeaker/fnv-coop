use std::fmt;

/// Everything that can go wrong while reading an archive.
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    /// The file doesn't start with the `BSA\0` magic.
    NotAnArchive {
        found: [u8; 4],
    },
    /// A valid BSA, but a version or feature this reader doesn't handle.
    Unsupported(String),
    /// The archive's structure is inconsistent.
    Malformed {
        offset: u64,
        reason: String,
    },
    /// A compressed file's data could not be decoded.
    Decompress {
        path: String,
        reason: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::NotAnArchive {
                found: [b'B', b'T', b'D', b'X'],
            } => f.write_str(
                "this is a BA2 archive from Fallout 4 or Starfield, not a New Vegas BSA",
            ),
            Error::NotAnArchive { found } => write!(
                f,
                "not a BSA archive: the file starts with {:?} instead of \"BSA\\0\"",
                String::from_utf8_lossy(found)
            ),
            Error::Unsupported(what) => write!(f, "unsupported archive: {what}"),
            Error::Malformed { offset, reason } => {
                write!(f, "malformed archive at offset {offset:#x}: {reason}")
            }
            Error::Decompress { path, reason } => {
                write!(f, "could not decompress {path}: {reason}")
            }
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
