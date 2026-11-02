//! `Havok SDK/hkablockdecode.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// An unaligned little-endian 16-bit read at `at`, byte by byte as the exe does.
fn read_word(e: &Engine, at: u32) -> u32 {
    u32::from(e.mem.u8(at)) | (u32::from(e.mem.u8(at.wrapping_add(1))) << 8)
}

/// The cursor over the per-item flag bits of `hk_block_decode`: one bit per item, least
/// significant bit of each byte first.
struct FlagBits {
    byte: u32,
    mask: u8,
}

impl FlagBits {
    fn is_set(&self, e: &Engine) -> bool {
        e.mem.u8(self.byte) & self.mask != 0
    }

    /// Moves to the next bit; after the eighth the next flag byte starts.
    fn advance(&mut self) {
        self.mask <<= 1;
        if self.mask == 0 {
            self.byte = self.byte.wrapping_add(1);
            self.mask = 1;
        }
    }
}

// Translated from 00e51a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBlockDecode` (Xbox PDB; cdecl `(source, header, dest, total_bytes)`, returns the number of
/// bytes of the block it accounts for): unpacks a block of `count` items into `dest`.
///
/// The header (dwords) holds the item width in bits at +0, a constant at +4 and the number
/// `raw` of leading uncompressed dwords at +8. `count` is `(total_bytes - 4 * raw) * 8 / width`
/// (unsigned; a width of 0 is the divide error of the exe and a panic here). The source holds
/// the `raw` dwords (copied to `dest` as a load/store pair copies floats: a signalling NaN is
/// quieted), then `ceil(count / 8)` flag bytes, then a stream of 16-bit words that holds the
/// items whose flag is clear. Per item, flag set: the constant is taken; flag clear: the next
/// `width` bits of the stream (low 16 bits of them, masked to `width` bits) are taken. The items
/// are written after the raw dwords: width 8 as bytes, width 16 as byte pairs (low byte
/// first) and any other width packed `width` bits at a time, least significant bit first,
/// flushed two bytes at a time with the last one or two bytes written at the end. The result is
/// `(raw * 32 + 7 + width * (items taken from the stream) + count) >> 3`.
pub fn hk_block_decode(
    e: &mut Engine,
    source: u32,
    header: Ptr,
    dest: Ptr,
    total_bytes: u32,
) -> u32 {
    let head = header.addr();
    let width = e.mem.u32(head);
    let raw_words = e.mem.u32(head + 8);
    let bytes_after_raw = total_bytes.wrapping_sub(raw_words.wrapping_mul(4));
    assert!(
        width != 0,
        "hkBlockDecode (00e51a60): divide error, the bit width in the header is 0"
    );
    let count = bytes_after_raw.wrapping_mul(8) / width;
    for index in 0..raw_words {
        let word = read_word(e, source.wrapping_add(4 * index))
            | (read_word(e, source.wrapping_add(4 * index).wrapping_add(2)) << 16);
        e.mem
            .set_u32(dest.addr().wrapping_add(4 * index), x87_quiet_bits(word));
    }
    let mut flags = FlagBits {
        byte: source.wrapping_add(raw_words.wrapping_mul(4)),
        mask: 1,
    };
    let mut stream = flags.byte.wrapping_add(count.wrapping_add(7) >> 3);
    let mut buffer = read_word(e, stream);
    stream = stream.wrapping_add(2);
    let mut out = dest.addr().wrapping_add(raw_words.wrapping_mul(4));
    // The bits left in `buffer`, and the count of items taken from the stream.
    let mut available = 0x10u8;
    let mut taken = 0u32;
    let mask = 1u32.wrapping_shl(width).wrapping_sub(1) & 0xffff;
    if width == 8 {
        for _ in 0..count {
            if flags.is_set(e) {
                let constant = e.mem.u8(head + 4);
                e.mem.set_u8(out, constant);
            } else {
                if available < 8 {
                    buffer |= read_word(e, stream).wrapping_shl(u32::from(available));
                    stream = stream.wrapping_add(2);
                    available = available.wrapping_add(0x10);
                }
                e.mem.set_u8(out, (buffer & mask) as u8);
                buffer >>= 8;
                available = available.wrapping_sub(8);
                taken += 1;
            }
            out = out.wrapping_add(1);
            flags.advance();
        }
    } else if width == 0x10 {
        for _ in 0..count {
            if flags.is_set(e) {
                let (low, high) = (e.mem.u8(head + 4), e.mem.u8(head + 5));
                e.mem.set_u8(out, low);
                e.mem.set_u8(out.wrapping_add(1), high);
            } else {
                if available < 0x10 {
                    buffer |= read_word(e, stream).wrapping_shl(u32::from(available));
                    stream = stream.wrapping_add(2);
                    available = available.wrapping_add(0x10);
                }
                e.mem.set_u8(out, buffer as u8);
                e.mem.set_u8(out.wrapping_add(1), (buffer >> 8) as u8);
                buffer >>= 16;
                available = available.wrapping_sub(0x10);
                taken += 1;
            }
            out = out.wrapping_add(2);
            flags.advance();
        }
    } else if count > 0 {
        // The output bits collected so far, and how many of them there are.
        let mut collected = 0u32;
        let mut collected_bits = 0u8;
        for _ in 0..count {
            let flagged = flags.is_set(e);
            let item = if flagged {
                e.mem.u32(head + 4)
            } else {
                if u32::from(available) < width {
                    buffer |= read_word(e, stream).wrapping_shl(u32::from(available));
                    stream = stream.wrapping_add(2);
                    available = available.wrapping_add(0x10);
                }
                (buffer & 0xffff) & mask
            };
            collected |= item.wrapping_shl(u32::from(collected_bits));
            collected_bits = collected_bits.wrapping_add(width as u8);
            if collected_bits > 0xf {
                e.mem.set_u8(out, collected as u8);
                e.mem.set_u8(out.wrapping_add(1), (collected >> 8) as u8);
                out = out.wrapping_add(2);
                collected >>= 16;
                collected_bits = collected_bits.wrapping_add(0xf0);
            }
            if !flagged {
                buffer = buffer.wrapping_shr(width);
                available = available.wrapping_sub(width as u8);
                taken += 1;
            }
            flags.advance();
        }
        if collected_bits != 0 {
            e.mem.set_u8(out, collected as u8);
            if collected_bits > 7 {
                e.mem.set_u8(out.wrapping_add(1), (collected >> 8) as u8);
            }
        }
    }
    raw_words
        .wrapping_shl(5)
        .wrapping_add(width.wrapping_mul(taken).wrapping_add(count))
        .wrapping_add(7)
        >> 3
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e5_1a60, hk_block_decode(u32, Ptr, Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A block laid out the way the exe reads it: raw dwords, flag bytes, then the stream of
    /// 16-bit words holding the items whose flag is clear, each `width` bits, low bits first.
    struct Input {
        bytes: Vec<u8>,
        total: u32,
    }

    /// Packs `(value, bits)` fields into bytes, least significant bit first.
    fn pack_bits(fields: &[(u32, u32)]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut position = 0usize;
        for (value, bits) in fields {
            for bit in 0..*bits as usize {
                if position / 8 == bytes.len() {
                    bytes.push(0);
                }
                if (value >> bit) & 1 != 0 {
                    bytes[position / 8] |= 1 << (position % 8);
                }
                position += 1;
            }
        }
        bytes
    }

    /// `items` are `Some(value)` for an item read from the stream and `None` for a flagged one.
    fn input(raw: &[u32], width: u32, items: &[Option<u32>]) -> Input {
        let count = items.len() as u32;
        let mut bytes: Vec<u8> = raw.iter().flat_map(|word| word.to_le_bytes()).collect();
        let mut flag_bytes = vec![0u8; items.len().div_ceil(8)];
        for (index, item) in items.iter().enumerate() {
            if item.is_none() {
                flag_bytes[index / 8] |= 1 << (index % 8);
            }
        }
        bytes.extend_from_slice(&flag_bytes);
        let fields: Vec<(u32, u32)> = items
            .iter()
            .flatten()
            .map(|value| (*value, width))
            .collect();
        bytes.extend_from_slice(&pack_bits(&fields));
        let total = 4 * raw.len() as u32 + (count * width).div_ceil(8);
        assert_eq!(
            (total - 4 * raw.len() as u32) * 8 / width,
            count,
            "the test items must fit the width"
        );
        Input { bytes, total }
    }

    struct Decoded {
        out: Vec<u8>,
        result: u32,
    }

    /// Runs the decoder with the header (`width`, `constant`, raw dword count) on `input`; the
    /// output block is 64 bytes of 0xee, `out` returns the bytes written.
    fn decode(input: &Input, width: u32, constant: u32, raw_words: u32) -> Decoded {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let source = e.mem.alloc(input.bytes.len() as u32 + 16);
        for (index, byte) in input.bytes.iter().enumerate() {
            e.mem.set_u8(source + index as u32, *byte);
        }
        let header = e.mem.alloc(16);
        e.mem.set_u32(header, width);
        e.mem.set_u32(header + 4, constant);
        e.mem.set_u32(header + 8, raw_words);
        let dest = e.mem.alloc(64);
        for index in 0..64 {
            e.mem.set_u8(dest + index, 0xee);
        }
        let result = hk_block_decode(
            &mut e,
            source,
            Ptr::new(header),
            Ptr::new(dest),
            input.total,
        );
        let out = (0..64).map(|index| e.mem.u8(dest + index)).collect();
        Decoded { out, result }
    }

    /// The 0xee fill must be untouched from `written` on.
    fn assert_untouched_from(decoded: &Decoded, written: usize) {
        assert!(
            decoded.out[written..].iter().all(|byte| *byte == 0xee),
            "bytes past {written} were written"
        );
    }

    #[test]
    fn eight_bit_items_take_the_constant_when_flagged() {
        let items: Vec<Option<u32>> = vec![
            None,
            Some(0x11),
            None,
            Some(0x22),
            Some(0x33),
            Some(0x44),
            Some(0x55),
            Some(0x66),
            Some(0x77),
            None,
        ];
        let input = input(&[], 8, &items);
        let decoded = decode(&input, 8, 0xab, 0);
        assert_eq!(
            decoded.out[..10],
            [0xab, 0x11, 0xab, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0xab]
        );
        assert_untouched_from(&decoded, 10);
        // Seven items came from the stream: (0 + 7 + 8 * 7 + 10) >> 3.
        assert_eq!(decoded.result, 9);
    }

    #[test]
    fn eight_bit_items_refill_the_buffer_every_other_item() {
        let items: Vec<Option<u32>> = (0..16).map(|index| Some(0xf0 - index * 7)).collect();
        let input = input(&[], 8, &items);
        let decoded = decode(&input, 8, 0, 0);
        let expected: Vec<u8> = (0..16u32).map(|index| (0xf0 - index * 7) as u8).collect();
        assert_eq!(decoded.out[..16], expected[..]);
        assert_untouched_from(&decoded, 16);
        assert_eq!(decoded.result, (7 + 8 * 16 + 16) >> 3);
    }

    #[test]
    fn sixteen_bit_items_are_written_low_byte_first() {
        let items = [Some(0x1234), None, Some(0xfffe), Some(0x0001), Some(0x8000)];
        let input = input(&[], 16, &items);
        let decoded = decode(&input, 16, 0xbeef, 0);
        assert_eq!(
            decoded.out[..10],
            [0x34, 0x12, 0xef, 0xbe, 0xfe, 0xff, 0x01, 0x00, 0x00, 0x80]
        );
        assert_untouched_from(&decoded, 10);
        assert_eq!(decoded.result, (7 + 16 * 4 + 5) >> 3);
    }

    #[test]
    fn other_widths_pack_the_items_tightly() {
        for (width, count) in [
            (3u32, 16usize),
            (5, 16),
            (7, 8),
            (12, 6),
            (13, 8),
            (1, 8),
            (15, 8),
        ] {
            let limit = 1u32 << width;
            let items: Vec<Option<u32>> = (0..count as u32)
                .map(|index| {
                    if index % 3 == 1 {
                        None
                    } else {
                        Some(index.wrapping_mul(2_654_435_761).wrapping_add(width) % limit)
                    }
                })
                .collect();
            let constant = (limit - 1) / 2;
            let input = input(&[], width, &items);
            let decoded = decode(&input, width, constant, 0);
            let fields: Vec<(u32, u32)> = items
                .iter()
                .map(|item| (item.unwrap_or(constant), width))
                .collect();
            let expected = pack_bits(&fields);
            assert_eq!(decoded.out[..expected.len()], expected[..], "width {width}");
            // The exe writes the second byte of the last pair when more than 7 bits are left,
            // so with exactly 8 left it writes a zero byte past the packed bytes.
            let bits = count as u32 * width;
            let left = bits % 16;
            let written = 2 * (bits / 16) as usize + usize::from(left > 0) + usize::from(left > 7);
            assert_untouched_from(&decoded, written);
            assert!(decoded.out[expected.len()..written]
                .iter()
                .all(|byte| *byte == 0));
            let taken = items.iter().flatten().count() as u32;
            assert_eq!(
                decoded.result,
                (7 + width * taken + count as u32) >> 3,
                "width {width}"
            );
        }
    }

    #[test]
    fn more_than_seven_collected_bits_write_both_bytes_of_the_pair() {
        // Width 5, 3 items: 15 bits, so two bytes are written.
        let fifteen = input(&[], 5, &[Some(21), Some(10), Some(31)]);
        let decoded = decode(&fifteen, 5, 0, 0);
        assert_eq!(
            decoded.out[..2],
            pack_bits(&[(21, 5), (10, 5), (31, 5)])[..]
        );
        assert_untouched_from(&decoded, 2);
    }

    #[test]
    fn up_to_seven_bits_write_only_one_byte() {
        // Width 3 with 2 items: 6 bits, one byte; the second byte of the pair stays 0xee.
        let short = input(&[], 3, &[Some(5), Some(2)]);
        let decoded = decode(&short, 3, 0, 0);
        assert_eq!(decoded.out[0], 5 | (2 << 3));
        assert_untouched_from(&decoded, 1);
    }

    #[test]
    fn raw_dwords_are_copied_first_with_a_signalling_nan_quieted() {
        let input = input(&[0x3f80_0000, 0x7f80_0001], 8, &[Some(9), None]);
        let decoded = decode(&input, 8, 0x77, 2);
        let words: Vec<u32> = decoded.out[..8]
            .chunks(4)
            .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        assert_eq!(words, vec![0x3f80_0000, 0x7fc0_0001]);
        assert_eq!(decoded.out[8..10], [9, 0x77]);
        assert_untouched_from(&decoded, 10);
        assert_eq!(decoded.result, (64 + 7 + 8 + 2) >> 3);
    }

    #[test]
    fn nothing_is_written_when_there_are_no_items() {
        for width in [8u32, 16, 5] {
            let input = input(&[], width, &[]);
            let decoded = decode(&input, width, 1, 0);
            assert_untouched_from(&decoded, 0);
            assert_eq!(decoded.result, 0, "width {width}");
        }
    }

    #[test]
    #[should_panic(expected = "divide error")]
    fn a_zero_width_is_a_divide_error() {
        let input = Input {
            bytes: vec![0; 16],
            total: 4,
        };
        decode(&input, 0, 0, 0);
    }
}
