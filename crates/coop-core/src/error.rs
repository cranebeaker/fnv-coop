//! Crate error type.

use thiserror::Error;

/// Everything that can go wrong in encode/decode and protocol checks.
#[derive(Debug, Error)]
pub enum Error {
    /// postcard failed to encode or decode.
    #[error("codec error: {0}")]
    Codec(#[from] postcard::Error),
    /// Snapshot did not start with the expected magic bytes.
    #[error("bad snapshot magic")]
    BadMagic,
    /// Snapshot or message version is not understood.
    #[error("unsupported version {0}")]
    Version(u8),
    /// LZ4 decompression failed or lied about its size.
    #[error("decompression failed: {0}")]
    Decompress(String),
    /// A buffer exceeded a hard size limit.
    #[error("too large: {len} bytes, limit {max}")]
    TooLarge {
        /// Actual size.
        len: usize,
        /// Allowed size.
        max: usize,
    },
    /// Bytes remained after a complete message.
    #[error("trailing bytes after message")]
    Trailing,
    /// Snapshot chunk framing problem.
    #[error("bad chunk: {0}")]
    Chunk(&'static str),
}

/// Crate result alias.
pub type Result<T> = std::result::Result<T, Error>;
