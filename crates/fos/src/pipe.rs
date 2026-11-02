//! The game's save buffer (`BGSSaveGameBuffer` (Xbox PDB)): every value
//! it writes is followed by a `|` byte (`00865ce0`).

use crate::{latin1, Error, RefId, Result};

/// The separator after every value in a pipe buffer.
pub const BAR: u8 = b'|';

/// Reads a pipe buffer: values each followed by `|`.
#[derive(Debug, Clone)]
pub struct Pipe<'a> {
    data: &'a [u8],
    pos: usize,
    /// Where `data` starts in the file, for error offsets.
    base: usize,
}

impl<'a> Pipe<'a> {
    /// `base` is where `data` starts in the file (0 if unknown).
    pub fn new(data: &'a [u8], base: usize) -> Self {
        Self { data, pos: 0, base }
    }

    /// Bytes read so far.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Position in the original source, including a rebased reader's origin.
    /// Inventory capture only calls this on readers backed by a parsed save.
    pub(crate) fn source_position(&self) -> usize {
        self.base + self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    fn error(&self, message: impl Into<String>) -> Error {
        Error::new(self.base + self.pos, message)
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.remaining() < n {
            return Err(self.error(format!("needs {n} bytes, {} left", self.remaining())));
        }
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    fn bar(&mut self) -> Result<()> {
        let at = self.pos;
        match self.take(1)?[0] {
            BAR => Ok(()),
            other => Err(Error::new(
                self.base + at,
                format!("expected '|', found {other:#04x}"),
            )),
        }
    }

    /// `n` bytes with no `|` after them: a block whose size was written
    /// before it (a vsval the writer filled in afterwards, `00865ff0`),
    /// made of the inner writer's own values and bars.
    pub fn raw(&mut self, n: usize) -> Result<&'a [u8]> {
        self.take(n)
    }

    /// `n` raw bytes and their `|`.
    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        let b = self.take(n)?;
        self.bar()?;
        Ok(b)
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.bytes(1)?[0])
    }

    pub fn u16(&mut self) -> Result<u16> {
        let b = self.bytes(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    pub fn u32(&mut self) -> Result<u32> {
        let b = self.bytes(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub fn i32(&mut self) -> Result<i32> {
        Ok(self.u32()? as i32)
    }

    pub fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.u32()?))
    }

    pub fn f64(&mut self) -> Result<f64> {
        let b = self.bytes(8)?;
        let mut a = [0; 8];
        a.copy_from_slice(b);
        Ok(f64::from_le_bytes(a))
    }

    /// A variable-sized value (`00865ff0`): the low two bits of the first
    /// byte give its size (1, 2 or 4 bytes), the rest is the value.
    pub fn vsval(&mut self) -> Result<u32> {
        let first = *self
            .data
            .get(self.pos)
            .ok_or_else(|| self.error("variable-sized value: no bytes left"))?;
        let value = match first & 3 {
            0 => u32::from(self.take(1)?[0]),
            1 => {
                let b = self.take(2)?;
                u32::from(u16::from_le_bytes([b[0], b[1]]))
            }
            2 => {
                let b = self.take(4)?;
                u32::from_le_bytes([b[0], b[1], b[2], b[3]])
            }
            _ => return Err(self.error("variable-sized value: size code 3")),
        };
        self.bar()?;
        Ok(value >> 2)
    }

    /// A refID: three bytes, most significant first (`00853500`).
    pub fn ref_id(&mut self) -> Result<RefId> {
        let b = self.bytes(3)?;
        Ok(RefId(
            u32::from(b[0]) << 16 | u32::from(b[1]) << 8 | u32::from(b[2]),
        ))
    }

    /// A string (`00865e70`): `u16` length, then the characters if any.
    pub fn wstr(&mut self) -> Result<String> {
        let n = self.u16()? as usize;
        if n == 0 {
            return Ok(String::new());
        }
        Ok(latin1(self.bytes(n)?))
    }

    /// Checks everything was read.
    pub fn finish(&self, what: &str) -> Result<()> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self.error(format!("{what}: {} bytes left over", self.remaining())))
        }
    }
}
