//! `Havok SDK/hkaexpandreal.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte that says the decoder tables are ready (`0126ef84`) and the function that prepares
/// them (`00e63c60`, `hkaProcessFlyingColors` in the Xbox PDB, cdecl, one word: the flag's
/// address), the table of scale factors by bit width (`010ec880`, floats) and the double added
/// to every stored integer before scaling (`01011588`).
const TABLES_READY: u32 = 0x0126_ef84;
const PREPARE_TABLES: u32 = 0x00e6_3c60;
const SCALE_BY_WIDTH: u32 = 0x010e_c880;
const ROUNDING_OFFSET: u32 = 0x0101_1588;

/// One stored integer turned into a float: `(raw + half) * scale + offset`, each step an x87
/// operation, the sum rounded to a float when stored. `offset_bits` is read from the header
/// (float at +8) for every value.
fn expand_value(e: &Engine, header: u32, scale: f64, half: f64, raw: i32) -> u32 {
    let widened = x87_add(f64::from(raw), half);
    let scaled = x87_mul(widened, scale);
    // `FADD dword ptr [header + 8]`: the offset is a memory operand.
    let offset = x87_m32(e.mem.u32(header + 8));
    x87_store32(x87_add(scaled, offset))
}

/// An unaligned little-endian 16-bit read at `at`, byte by byte as the exe does.
fn read_word(e: &Engine, at: u32) -> u32 {
    u32::from(e.mem.u8(at)) | (u32::from(e.mem.u8(at.wrapping_add(1))) << 8)
}

// Translated from 00e51660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkExpandReal` (Xbox PDB; cdecl `(stream, header, out, total)`): expands the packed floats
/// of a compressed animation block into `total` floats at `out`.
///
/// The header has the bit width of the packed values at +0, the number `raw` of leading
/// uncompressed floats at +1, a scale at +4 and an offset at +8 (floats). The first `raw`
/// floats are copied from the (unaligned) stream, a signalling NaN quieted as the load/store
/// pair does. Then the byte at `0126ef84` must be set: when it is clear the function at
/// `00e63c60` is called with its address, and when it is still clear afterwards the function
/// ends. The remaining `total - raw` values each become `(stored + 0.5) * scale + offset` where
/// `scale` is the table entry for the bit width times the header's scale (rounded to a float),
/// written after the raw floats. A width of 8 reads bytes and a width of 16 reads 16-bit
/// words (the exe unrolls both four times, the order of reads and writes per value is the
/// same); any other width reads a bit stream of 16-bit words, least significant bits first,
/// the stored value being the low 16 bits of each field. A width of 0 never ends in the exe
/// (the field loop never exits) and is a panic here.
pub fn hk_expand_real(e: &mut Engine, stream: Ptr, header: Ptr, out: Ptr, total: u32) {
    let head = header.addr();
    let out = out.addr();
    let mut source = stream.addr();
    let raw_count = u32::from(e.mem.u8(head + 1));
    for index in 0..raw_count {
        let word = read_word(e, source) | (read_word(e, source.wrapping_add(2)) << 16);
        source = source.wrapping_add(4);
        e.mem
            .set_u32(out.wrapping_add(4 * index), x87_quiet_bits(word));
    }
    if e.mem.u8(TABLES_READY) == 0 {
        e.call(PREPARE_TABLES, &args![TABLES_READY]);
        if e.mem.u8(TABLES_READY) == 0 {
            return;
        }
    }
    let width = u32::from(e.mem.u8(head));
    let raw_count = u32::from(e.mem.u8(head + 1));
    let values = out.wrapping_add(raw_count * 4);
    let remaining = total.wrapping_sub(raw_count);
    // `FLD dword` of the table entry, `FMUL dword ptr [header + 4]` (a memory operand).
    let scale_bits = x87_store32(x87_mul(
        x87_load32(e.mem.u32(SCALE_BY_WIDTH.wrapping_add(width * 4))),
        x87_m32(e.mem.u32(head + 4)),
    ));
    let scale = x87_load32(scale_bits);
    let half = e.mem.f64(ROUNDING_OFFSET);
    if width == 8 {
        for index in 0..remaining {
            let raw = i32::from(e.mem.u8(source));
            source = source.wrapping_add(1);
            let value = expand_value(e, head, scale, half, raw);
            e.mem.set_u32(values.wrapping_add(4 * index), value);
        }
    } else if width == 0x10 {
        for index in 0..remaining {
            let raw = read_word(e, source) as i32;
            source = source.wrapping_add(2);
            let value = expand_value(e, head, scale, half, raw);
            e.mem.set_u32(values.wrapping_add(4 * index), value);
        }
    } else {
        // Nothing is read or written when there is nothing to expand.
        if remaining == 0 {
            return;
        }
        assert!(
            width != 0,
            "hkExpandReal (00e51660): a bit width of 0 never ends in the exe"
        );
        let mask = 1u32.wrapping_shl(width).wrapping_sub(1) & 0xffff;
        let mut bits = 0u32;
        let mut available = 0i32;
        let mut index = 0u32;
        loop {
            while available >= width as i32 {
                // The exe counts the values past `remaining` too (they are not written).
                if index < remaining {
                    let value = expand_value(e, head, scale, half, (bits & mask) as i32);
                    e.mem.set_u32(values.wrapping_add(4 * index), value);
                }
                bits = bits.wrapping_shr(width);
                available -= width as i32;
                index = index.wrapping_add(1);
            }
            bits |= read_word(e, source).wrapping_shl(available as u32);
            source = source.wrapping_add(2);
            available += 16;
            if index >= remaining {
                return;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e5_1660, hk_expand_real(Ptr, Ptr, Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    /// The scale-by-width table entry the exe holds for `width` (two to the minus width).
    fn table_entry(width: u32) -> f32 {
        2.0f32.powi(-(width as i32))
    }

    /// What the exe stores for the integer `raw` with the given header values.
    fn expected(width: u32, header_scale: f32, offset: f32, raw: u32) -> u32 {
        let scale = (f64::from(table_entry(width)) * f64::from(header_scale)) as f32;
        (((f64::from(raw) + 0.5) * f64::from(scale) + f64::from(offset)) as f32).to_bits()
    }

    struct Block {
        e: Engine,
        header: u32,
        out: u32,
        stream: u32,
        calls: Calls,
    }

    /// An engine with the tables of the exe for `width`, a header, an output block of 40 words
    /// followed by a guard word, the stream (placed at an odd address) and a double for the
    /// table preparation that sets the ready flag when `prepares` is true.
    fn make_block(
        width: u32,
        raw_count: u8,
        scale: f32,
        offset: f32,
        stream_bytes: &[u8],
        prepares: bool,
    ) -> Block {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(ROUNDING_OFFSET, 0.5);
        e.mem
            .set_u32(SCALE_BY_WIDTH + 4 * width, table_entry(width).to_bits());
        let header = e.mem.alloc(16);
        e.mem.set_u8(header, width as u8);
        e.mem.set_u8(header + 1, raw_count);
        e.mem.set_u32(header + 4, scale.to_bits());
        e.mem.set_u32(header + 8, offset.to_bits());
        let out = e.mem.alloc(4 * 41);
        e.mem.set_u32(out + 4 * 40, 0xdead_beef);
        let stream = e.mem.alloc(stream_bytes.len() as u32 + 8) + 1;
        for (index, byte) in stream_bytes.iter().enumerate() {
            e.mem.set_u8(stream + index as u32, *byte);
        }
        let calls: Calls = Rc::default();
        let seen = Rc::clone(&calls);
        e.register_double(PREPARE_TABLES, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            if prepares {
                e.mem.set_u8(TABLES_READY, 1);
            }
            Ret::default()
        });
        Block {
            e,
            header,
            out,
            stream,
            calls,
        }
    }

    impl Block {
        fn run(&mut self, total: u32) {
            hk_expand_real(
                &mut self.e,
                Ptr::new(self.stream),
                Ptr::new(self.header),
                Ptr::new(self.out),
                total,
            );
        }

        fn word(&self, index: u32) -> u32 {
            self.e.mem.u32(self.out + 4 * index)
        }

        fn guard(&self) -> u32 {
            self.e.mem.u32(self.out + 4 * 40)
        }
    }

    #[test]
    fn leading_raw_floats_are_copied_and_quieted_from_an_unaligned_stream() {
        let mut stream = Vec::new();
        for bits in [0x3f80_0000u32, 0x7f80_0001, 0xc040_0000] {
            stream.extend_from_slice(&bits.to_le_bytes());
        }
        let mut block = make_block(8, 3, 1.0, 0.0, &stream, true);
        set_ready(&mut block.e);
        block.run(3);
        assert_eq!(block.word(0), 0x3f80_0000);
        assert_eq!(block.word(1), 0x7fc0_0001);
        assert_eq!(block.word(2), 0xc040_0000);
        assert_eq!(block.word(3), 0);
        assert_eq!(block.guard(), 0xdead_beef);
    }

    fn set_ready(e: &mut Engine) {
        e.mem.set_u8(TABLES_READY, 1);
    }

    #[test]
    fn a_clear_ready_flag_calls_the_preparation_and_goes_on_when_it_sets_the_flag() {
        let mut block = make_block(8, 0, 2.0, 1.0, &[10, 20], true);
        block.run(2);
        assert_eq!(*block.calls.borrow(), vec![vec![TABLES_READY]]);
        assert_eq!(block.word(0), expected(8, 2.0, 1.0, 10));
        assert_eq!(block.word(1), expected(8, 2.0, 1.0, 20));
    }

    #[test]
    fn a_flag_that_stays_clear_ends_after_the_raw_floats() {
        let mut stream = 1.5f32.to_bits().to_le_bytes().to_vec();
        stream.extend_from_slice(&[10, 20]);
        let mut block = make_block(8, 1, 2.0, 1.0, &stream, false);
        block.run(3);
        assert_eq!(block.calls.borrow().len(), 1);
        assert_eq!(block.word(0), 1.5f32.to_bits());
        assert_eq!(block.word(1), 0);
    }

    #[test]
    fn a_set_flag_skips_the_preparation() {
        let mut block = make_block(8, 0, 1.0, 0.0, &[1], false);
        set_ready(&mut block.e);
        block.run(1);
        assert!(block.calls.borrow().is_empty());
        assert_eq!(block.word(0), expected(8, 1.0, 0.0, 1));
    }

    #[test]
    fn eight_bit_values_are_read_as_bytes_after_the_raw_floats() {
        let mut stream = 0.25f32.to_bits().to_le_bytes().to_vec();
        let bytes: Vec<u8> = (0..11).map(|index| 250 - index * 23).collect();
        stream.extend_from_slice(&bytes);
        // 11 values: two groups of four and a tail of three.
        let mut block = make_block(8, 1, 3.0, -0.75, &stream, true);
        block.run(12);
        assert_eq!(block.word(0), 0.25f32.to_bits());
        for (index, byte) in bytes.iter().enumerate() {
            assert_eq!(
                block.word(1 + index as u32),
                expected(8, 3.0, -0.75, u32::from(*byte)),
                "{index}"
            );
        }
        assert_eq!(block.word(12), 0);
    }

    #[test]
    fn sixteen_bit_values_are_read_as_words() {
        let words: Vec<u16> = (0..6).map(|index| 0xfff0 - index * 0x1111).collect();
        let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
        let mut block = make_block(16, 0, 0.5, 100.0, &bytes, true);
        block.run(6);
        for (index, word) in words.iter().enumerate() {
            assert_eq!(
                block.word(index as u32),
                expected(16, 0.5, 100.0, u32::from(*word)),
                "{index}"
            );
        }
        assert_eq!(block.word(6), 0);
    }

    /// The fields an independent bit reader finds in `bytes`, `width` bits each, low bits first.
    fn bit_fields(bytes: &[u8], width: usize, count: usize) -> Vec<u32> {
        (0..count)
            .map(|field| {
                (0..width).fold(0u32, |value, bit| {
                    let position = field * width + bit;
                    let set = (bytes[position / 8] >> (position % 8)) & 1;
                    value | (u32::from(set) << bit)
                })
            })
            .collect()
    }

    #[test]
    fn other_widths_read_a_bit_stream() {
        let bytes: Vec<u8> = (0..40u32).map(|index| (index * 37 + 11) as u8).collect();
        for width in [1u32, 3, 5, 7, 12, 15] {
            let count = 19usize;
            let mut block = make_block(width, 0, 1.25, 0.5, &bytes, true);
            block.run(count as u32);
            for (index, field) in bit_fields(&bytes, width as usize, count).iter().enumerate() {
                assert_eq!(
                    block.word(index as u32),
                    expected(width, 1.25, 0.5, *field),
                    "width {width} value {index}"
                );
            }
            assert_eq!(block.word(count as u32), 0, "width {width}");
        }
    }

    #[test]
    fn bit_stream_widths_never_write_past_the_requested_count() {
        let bytes = [0xffu8; 16];
        // Width 12: the second 16-bit word completes fields beyond the count of 1.
        let mut block = make_block(12, 0, 1.0, 0.0, &bytes, true);
        block.run(1);
        assert_eq!(block.word(0), expected(12, 1.0, 0.0, 0xfff));
        assert_eq!(block.word(1), 0);
    }

    #[test]
    fn nothing_is_expanded_when_the_total_equals_the_raw_count() {
        let mut block = make_block(5, 0, 1.0, 0.0, &[1, 2, 3], true);
        block.run(0);
        assert_eq!(block.word(0), 0);
    }

    #[test]
    #[should_panic(expected = "never ends")]
    fn a_zero_width_never_ends_in_the_exe() {
        let mut block = make_block(0, 0, 1.0, 0.0, &[1, 2, 3, 4], true);
        block.run(2);
    }

    #[test]
    fn a_nan_scale_spreads_into_every_value() {
        let mut block = make_block(8, 0, f32::from_bits(0x7f80_0001), 1.0, &[3, 4], true);
        block.run(2);
        assert_eq!(block.word(0), 0x7fc0_0001);
        assert_eq!(block.word(1), 0x7fc0_0001);
    }
}
