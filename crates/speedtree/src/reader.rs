//! The `.spt` file's tokens: the game's reader (`00b06090` holds the bytes
//! and a position) gives little-endian 32-bit tokens and integers
//! (`00b3ed80`), floats (`00b3ed20`), single bytes for flags (`00b06180`,
//! `00b40800`), 32-bit lengths followed by that many characters for
//! strings (`00b3ee40`), and three floats for vectors (`00b3ef80`).
//! `00b3ede0` reads 32 bits and skips 32 more.

use std::fmt;

/// Something the reader couldn't make sense of, with the byte offset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SptError {
    pub offset: usize,
    pub message: String,
}

impl fmt::Display for SptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (at byte {})", self.message, self.offset)
    }
}

impl std::error::Error for SptError {}

pub type Result<T> = std::result::Result<T, SptError>;

/// A read position in the file's bytes.
pub(crate) struct Reader<'a> {
    bytes: &'a [u8],
    pub pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Reader { bytes, pos: 0 }
    }

    pub fn error<T>(&self, message: impl Into<String>) -> Result<T> {
        Err(SptError {
            offset: self.pos,
            message: message.into(),
        })
    }

    pub fn at_end(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.pos + n > self.bytes.len() {
            return self.error("premature end of file");
        }
        let s = &self.bytes[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    /// `00b3ed80`.
    pub fn int(&mut self) -> Result<i32> {
        let b = self.take(4)?;
        Ok(i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// `00b3efe0`: the next token without moving on.
    pub fn peek(&self) -> Option<i32> {
        self.bytes
            .get(self.pos..self.pos + 4)
            .map(|b| i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// `00b3ed20`.
    pub fn float(&mut self) -> Result<f32> {
        let b = self.take(4)?;
        Ok(f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// `00b06180`.
    pub fn flag(&mut self) -> Result<bool> {
        Ok(self.take(1)?[0] != 0)
    }

    /// `00b40800`.
    pub fn byte(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    /// `00b3ef80`.
    pub fn vec3(&mut self) -> Result<[f32; 3]> {
        Ok([self.float()?, self.float()?, self.float()?])
    }

    /// `00b3ede0`: 32 bits, then 32 more skipped.
    pub fn int_skip(&mut self) -> Result<u32> {
        let v = self.int()? as u32;
        self.take(4)?;
        Ok(v)
    }

    /// `00b3ee40`.
    pub fn string(&mut self) -> Result<String> {
        let n = self.int()?;
        if n < 0 {
            return self.error("negative string length");
        }
        let b = self.take(n as usize)?;
        Ok(b.iter().map(|&c| c as char).collect())
    }

    /// The next token, which must be `want`.
    pub fn expect(&mut self, want: i32, what: &str) -> Result<()> {
        let t = self.int()?;
        if t != want {
            self.pos -= 4;
            return self.error(format!("{what}: expected token {want}, found {t}"));
        }
        Ok(())
    }
}
