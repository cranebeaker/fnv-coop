//! The bit reader Bink uses: the stream is a run of little-endian 32-bit
//! words, and bits are taken from the least significant end of each word.

/// Reads bits from a byte slice, least significant bit first. Reading past
/// the end gives zero bits and sets [`BitReader::overrun`], so a damaged
/// stream ends a frame early instead of panicking.
pub(crate) struct BitReader<'a> {
    data: &'a [u8],
    /// Index of the next bit to read.
    pos: usize,
    overrun: bool,
}

impl<'a> BitReader<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        BitReader {
            data,
            pos: 0,
            overrun: false,
        }
    }

    /// Bits read so far.
    pub(crate) fn position(&self) -> usize {
        self.pos
    }

    /// Total bits in the stream.
    pub(crate) fn len(&self) -> usize {
        self.data.len() * 8
    }

    /// Whether a read went past the end.
    pub(crate) fn overrun(&self) -> bool {
        self.overrun
    }

    /// Up to 32 bits, without moving.
    pub(crate) fn peek(&self, n: u32) -> u32 {
        debug_assert!(n <= 32);
        if n == 0 {
            return 0;
        }
        let byte = self.pos / 8;
        let shift = (self.pos % 8) as u32;
        let mut word: u64 = 0;
        for i in 0..5 {
            if let Some(&b) = self.data.get(byte + i) {
                word |= (b as u64) << (8 * i);
            }
        }
        ((word >> shift) & ((1u64 << n) - 1)) as u32
    }

    /// Up to 32 bits; the first bit read is bit 0 of the result.
    pub(crate) fn bits(&mut self, n: u32) -> u32 {
        let v = self.peek(n);
        self.skip(n as usize);
        v
    }

    pub(crate) fn bit(&mut self) -> bool {
        self.bits(1) != 0
    }

    pub(crate) fn skip(&mut self, n: usize) {
        self.pos += n;
        if self.pos > self.len() {
            self.overrun = true;
        }
    }

    /// Moves to the next multiple of 32 bits (planes start on word
    /// boundaries).
    pub(crate) fn align32(&mut self) {
        let r = self.pos % 32;
        if r != 0 {
            self.skip(32 - r);
        }
    }
}
