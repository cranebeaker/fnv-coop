use crate::error::{Error, Result};
use crate::types::FourCC;

/// Little-endian reader over a byte slice that reports absolute file
/// offsets in its errors.
#[derive(Clone)]
pub(crate) struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
    base: usize,
}

impl<'a> Cursor<'a> {
    /// `base` is the absolute offset of `buf[0]`, used only for error messages.
    pub fn new(buf: &'a [u8], base: usize) -> Self {
        Self { buf, pos: 0, base }
    }

    pub fn offset(&self) -> usize {
        self.base + self.pos
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.buf.len()
    }

    pub fn take(&mut self, n: usize, context: &'static str) -> Result<&'a [u8]> {
        if self.remaining() < n {
            return Err(Error::UnexpectedEof {
                offset: self.offset(),
                needed: n,
                available: self.remaining(),
                context,
            });
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    pub fn array<const N: usize>(&mut self, context: &'static str) -> Result<[u8; N]> {
        let mut a = [0u8; N];
        a.copy_from_slice(self.take(N, context)?);
        Ok(a)
    }

    pub fn u8(&mut self, context: &'static str) -> Result<u8> {
        Ok(self.array::<1>(context)?[0])
    }

    pub fn u16(&mut self, context: &'static str) -> Result<u16> {
        Ok(u16::from_le_bytes(self.array(context)?))
    }

    pub fn i16(&mut self, context: &'static str) -> Result<i16> {
        Ok(i16::from_le_bytes(self.array(context)?))
    }

    pub fn u32(&mut self, context: &'static str) -> Result<u32> {
        Ok(u32::from_le_bytes(self.array(context)?))
    }

    pub fn i32(&mut self, context: &'static str) -> Result<i32> {
        Ok(i32::from_le_bytes(self.array(context)?))
    }

    pub fn f32(&mut self, context: &'static str) -> Result<f32> {
        Ok(f32::from_le_bytes(self.array(context)?))
    }

    pub fn fourcc(&mut self, context: &'static str) -> Result<FourCC> {
        Ok(FourCC(self.array(context)?))
    }
}
