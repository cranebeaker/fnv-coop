// Follows libogg 1.1.3 (src/bitwise.c), Copyright (c) 2002 Xiph.org Foundation, BSD licence:
// see ../LICENSE-libvorbis.
//! The packet bit reader (libogg's `oggpack_*` read side, LSb first), with its behaviour at the
//! end of a packet: a read past the end answers -1 and still moves the position on, so every
//! later read fails too; a look past the end answers -1 and moves nothing.

const MASK: [u32; 33] = {
    let mut m = [0u32; 33];
    let mut i = 1;
    while i < 33 {
        m[i] = if i == 32 { u32::MAX } else { (1u32 << i) - 1 };
        i += 1;
    }
    m
};

/// A reader over one packet.
#[derive(Debug, Clone)]
pub struct Bits<'a> {
    data: &'a [u8],
    /// `endbyte` and `endbit`: the byte and bit the next read starts at.
    byte: i64,
    bit: u32,
}

impl<'a> Bits<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Bits {
            data,
            byte: 0,
            bit: 0,
        }
    }

    fn at(&self, i: i64) -> u32 {
        // Bytes past the packet read as 0: libogg reads them only where the mask drops them.
        if i >= 0 && (i as usize) < self.data.len() {
            u32::from(self.data[i as usize])
        } else {
            0
        }
    }

    fn gather(&self, bits: u32) -> u32 {
        // `bits` includes the bit offset, as in libogg.
        let b = self.bit;
        let mut ret = self.at(self.byte) >> b;
        if bits > 8 {
            ret |= self.at(self.byte + 1) << (8 - b);
            if bits > 16 {
                ret |= self.at(self.byte + 2) << (16 - b);
                if bits > 24 {
                    ret |= self.at(self.byte + 3).wrapping_shl(24 - b);
                    if bits > 32 && b != 0 {
                        ret |= self.at(self.byte + 4).wrapping_shl(32 - b);
                    }
                }
            }
        }
        ret
    }

    fn past_end(&self, bits: u32) -> bool {
        let storage = self.data.len() as i64;
        self.byte + 4 >= storage && self.byte * 8 + i64::from(bits) > storage * 8
    }

    /// `oggpack_look`: the next `bits` bits without moving, or -1 past the end.
    pub fn look(&self, bits: u32) -> i64 {
        let m = MASK[bits as usize];
        let total = bits + self.bit;
        if self.past_end(total) {
            return -1;
        }
        i64::from(m & self.gather(total))
    }

    /// `oggpack_adv`.
    pub fn adv(&mut self, bits: u32) {
        let total = bits + self.bit;
        self.byte += i64::from(total / 8);
        self.bit = total & 7;
    }

    /// `oggpack_read`: the next `bits` bits (as libogg's `long`: 32 bits read as signed), or -1
    /// past the end.
    pub fn read(&mut self, bits: u32) -> i32 {
        let m = MASK[bits as usize];
        let total = bits + self.bit;
        let ret = if self.past_end(total) {
            -1
        } else {
            (m & self.gather(total)) as i32
        };
        self.byte += i64::from(total / 8);
        self.bit = total & 7;
        ret
    }
}

/// `ilog` as in `sharedbook.c`, `floor1.c` and `res0.c`: the number of bits in `v`.
pub fn ilog(v: u32) -> u32 {
    32 - v.leading_zeros()
}

/// `ilog` as in `mapping0.c` and `ilog2` as in `block.c`: `ceil(log2(v))` (0 for 0 and 1).
pub fn ilog_ceil(v: u32) -> u32 {
    if v == 0 {
        0
    } else {
        ilog(v - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_lsb_first_and_fails_past_the_end() {
        let d = [0b1010_1100u8, 0xff];
        let mut b = Bits::new(&d);
        assert_eq!(b.read(3), 0b100);
        assert_eq!(b.look(5), 0b10101);
        assert_eq!(b.read(9), 0b1_1111_0101);
        assert_eq!(b.read(4), 0b1111);
        assert_eq!(b.look(1), -1);
        assert_eq!(b.read(0), 0);
        assert_eq!(b.read(1), -1);
        assert_eq!(b.read(0), -1);
        assert_eq!(b.look(1), -1);
        assert_eq!(b.read(1), -1);
        let mut c = Bits::new(&[0xff, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(c.read(32), -1);
        assert_eq!(c.read(8), 255);
    }
}
