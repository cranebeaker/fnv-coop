use crate::error::{Error, Result};
use crate::math::{Mat3, Vec3};

/// Little-endian reader over a byte slice, reporting absolute offsets.
pub(crate) struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
    base: usize,
}

impl<'a> Reader<'a> {
    /// `base` is the absolute file offset of `buf[0]`.
    pub fn new(buf: &'a [u8], base: usize) -> Self {
        Self { buf, pos: 0, base }
    }

    /// Starts reading `buf` at `pos` (both relative to the same slice).
    pub fn at(buf: &'a [u8], pos: usize) -> Self {
        Self { buf, pos, base: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn offset(&self) -> usize {
        self.base + self.pos
    }

    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }

    pub fn take(&mut self, n: usize, what: &str) -> Result<&'a [u8]> {
        let slice = self
            .buf
            .get(self.pos..self.pos.saturating_add(n))
            .ok_or_else(|| Error::Malformed {
                offset: self.offset(),
                reason: format!("data ends in the middle of {what}"),
            })?;
        self.pos += n;
        Ok(slice)
    }

    fn array<const N: usize>(&mut self, what: &str) -> Result<[u8; N]> {
        Ok(self.take(N, what)?.try_into().expect("length checked"))
    }

    pub fn u8(&mut self, what: &str) -> Result<u8> {
        Ok(self.array::<1>(what)?[0])
    }

    pub fn bool(&mut self, what: &str) -> Result<bool> {
        Ok(self.u8(what)? != 0)
    }

    pub fn u16(&mut self, what: &str) -> Result<u16> {
        Ok(u16::from_le_bytes(self.array(what)?))
    }

    pub fn u32(&mut self, what: &str) -> Result<u32> {
        Ok(u32::from_le_bytes(self.array(what)?))
    }

    pub fn i32(&mut self, what: &str) -> Result<i32> {
        Ok(i32::from_le_bytes(self.array(what)?))
    }

    pub fn f32(&mut self, what: &str) -> Result<f32> {
        Ok(f32::from_le_bytes(self.array(what)?))
    }

    pub fn vec3(&mut self, what: &str) -> Result<Vec3> {
        Ok([self.f32(what)?, self.f32(what)?, self.f32(what)?])
    }

    /// A 3x3 matrix stored row by row.
    pub fn mat3(&mut self, what: &str) -> Result<Mat3> {
        Ok([self.vec3(what)?, self.vec3(what)?, self.vec3(what)?])
    }

    /// A count followed by that many items; guards against absurd counts
    /// in corrupt files by checking there are enough bytes left.
    pub fn counted<T>(
        &mut self,
        count: usize,
        item_size: usize,
        what: &str,
        mut read: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<T>> {
        if count.saturating_mul(item_size) > self.remaining() {
            return Err(Error::Malformed {
                offset: self.offset(),
                reason: format!("{what} claims {count} entries, more than the data holds"),
            });
        }
        (0..count).map(|_| read(self)).collect()
    }

    /// A u32 count followed by that many i32 block references.
    pub fn ref_list(&mut self, what: &str) -> Result<Vec<i32>> {
        let n = self.u32(what)? as usize;
        self.counted(n, 4, what, |r| r.i32(what))
    }

    /// A string stored inline as a u32 length and that many bytes.
    pub fn sized_string(&mut self, what: &str) -> Result<String> {
        let n = self.u32(what)? as usize;
        let bytes = self.take(n, what)?;
        Ok(latin1(bytes))
    }
}

/// Decodes a NUL-terminated or plain byte string. Paths and names in these
/// files are ASCII in practice; other bytes map to the matching Latin-1
/// character rather than failing.
pub(crate) fn latin1(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    bytes[..end].iter().map(|&b| char::from(b)).collect()
}
