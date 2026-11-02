use std::fmt;

use crate::types::{FormId, FourCC};

/// Everything that can go wrong while reading a plugin.
#[derive(Debug)]
pub enum Error {
    /// The file could not be read from disk.
    Io(std::io::Error),
    /// The data ended before a complete structure could be read.
    UnexpectedEof {
        offset: usize,
        needed: usize,
        available: usize,
        context: &'static str,
    },
    /// The file does not start with a `TES4` header record.
    NotAPlugin { found: FourCC },
    /// A group header is inconsistent with the data around it.
    BadGroup { offset: usize, reason: String },
    /// A record or subrecord is inconsistent with the data around it.
    BadRecord { offset: usize, reason: String },
    /// A compressed record's zlib stream could not be decoded.
    Decompress { form_id: FormId, reason: String },
    /// An error while decoding the contents of a specific record.
    InRecord {
        kind: FourCC,
        form_id: FormId,
        source: Box<Error>,
    },
    /// An error while loading one plugin of a load order.
    InPlugin { name: String, source: Box<Error> },
    /// A plugin depends on a master that isn't loaded.
    MissingMaster { plugin: String, master: String },
    /// More plugins than the 255 the form ID scheme can address.
    TooManyPlugins { count: usize },
    /// A problem with the Data folder as a whole.
    DataDir(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::UnexpectedEof {
                offset,
                needed,
                available,
                context,
            } => write!(
                f,
                "unexpected end of data at offset {offset:#x} while reading {context} \
                 (needed {needed} bytes, {available} left)"
            ),
            Error::NotAPlugin { found } => write!(
                f,
                "not a Fallout 3 / New Vegas plugin: the file starts with '{found}' instead of 'TES4'"
            ),
            Error::BadGroup { offset, reason } => {
                write!(f, "malformed group at offset {offset:#x}: {reason}")
            }
            Error::BadRecord { offset, reason } => {
                write!(f, "malformed record at offset {offset:#x}: {reason}")
            }
            Error::Decompress { form_id, reason } => {
                write!(f, "could not decompress record {form_id}: {reason}")
            }
            Error::InRecord {
                kind,
                form_id,
                source,
            } => write!(f, "in {kind} record {form_id}: {source}"),
            Error::InPlugin { name, source } => write!(f, "in {name}: {source}"),
            Error::MissingMaster { plugin, master } => write!(
                f,
                "{plugin} requires {master}, which isn't loaded"
            ),
            Error::TooManyPlugins { count } => write!(
                f,
                "{count} plugins are active, but the game can only load 255"
            ),
            Error::DataDir(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::InRecord { source, .. } | Error::InPlugin { source, .. } => {
                Some(source.as_ref())
            }
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
