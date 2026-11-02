//! Reading bits most significant first, the order MPEG audio stores them in.

pub(crate) struct BitReader<'a> {
    data: &'a [u8],
    /// Position in bits from the start of `data`.
    pos: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitReader { data, pos: 0 }
    }

    /// A reader starting `bit` bits into `data`.
    pub fn at(data: &'a [u8], bit: usize) -> Self {
        BitReader { data, pos: bit }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    /// The next `n` bits (at most 32) without moving on. Past the end of
    /// the data, zeros are read, so damaged data can't read out of bounds;
    /// callers check positions against their own limits.
    pub fn peek(&self, n: u32) -> u32 {
        debug_assert!(n <= 32);
        if n == 0 {
            return 0;
        }
        let byte = self.pos >> 3;
        let shift = self.pos & 7;
        let mut word = 0u64;
        if let Some(chunk) = self.data.get(byte..byte + 8) {
            word = u64::from_be_bytes(chunk.try_into().unwrap());
        } else {
            for i in 0..8 {
                word = (word << 8) | u64::from(self.data.get(byte + i).copied().unwrap_or(0));
            }
        }
        ((word << shift) >> (64 - n)) as u32
    }

    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    /// Reads `n` bits (at most 32).
    pub fn read(&mut self, n: u32) -> u32 {
        let v = self.peek(n);
        self.pos += n as usize;
        v
    }

    /// Reads one bit.
    pub fn bit(&mut self) -> u32 {
        let byte = self.data.get(self.pos >> 3).copied().unwrap_or(0);
        let v = (byte >> (7 - (self.pos & 7))) & 1;
        self.pos += 1;
        u32::from(v)
    }

    pub fn flag(&mut self) -> bool {
        self.bit() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_across_bytes_and_past_the_end() {
        let data = [0b1010_1100, 0b0101_0011, 0xFF];
        let mut r = BitReader::new(&data);
        assert_eq!(r.read(3), 0b101);
        assert_eq!(r.read(7), 0b011_0001);
        assert_eq!(r.bit(), 0);
        assert_eq!(r.peek(4), 0b1001);
        assert_eq!(r.read(13), 0b1_0011_1111_1111);
        // Past the end: zeros.
        assert_eq!(r.position(), 24);
        assert_eq!(r.read(32), 0);
        let r = BitReader::at(&data, 16);
        assert_eq!(r.peek(8), 0xFF);
    }
}
