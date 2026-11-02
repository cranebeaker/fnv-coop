//! `Havok SDK/hkcrc32streamwriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The reflected CRC-32 polynomial.
const POLYNOMIAL: u32 = 0xedb8_8320;

// Translated from 00db7790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The CRC-32 table step (cdecl, one word): eight rounds of "shift right by one, and xor in
/// the polynomial `0xEDB88320` when the bit shifted out was set". The whole word takes part,
/// not only its low byte.
pub fn fn_00db7790(_e: &mut Engine, value: u32) -> u32 {
    let mut crc = value;
    for _ in 0..8 {
        crc = if crc & 1 != 0 {
            (crc >> 1) ^ POLYNOMIAL
        } else {
            crc >> 1
        };
    }
    crc
}

// Translated from 00db7810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCrc32StreamWriter::write` (Xbox PDB; `this` = the writer, then `(data, length)`,
/// `RET 8`): feeds `length` bytes into the running CRC kept at `this + 8` (for each byte, the
/// table step of `(byte ^ crc) & 0xFF`, xored into the CRC shifted right by 8). A length of
/// 0 or below does nothing. Answers the length.
pub fn hk_crc32_stream_writer_write(e: &mut Engine, this: Ptr, data: Ptr, length: i32) -> i32 {
    let mut crc = e.mem.u32(this.addr() + 8);
    let mut index = 0i32;
    while index < length {
        let byte = u32::from(e.mem.u8(data.addr().wrapping_add(index as u32)));
        let step = fn_00db7790(e, (byte ^ crc) & 0xff);
        crc = (crc >> 8) ^ step;
        index += 1;
    }
    e.mem.set_u32(this.addr() + 8, crc);
    length
}

// Translated from 00db7860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The finished CRC: the bitwise complement of the running CRC at `this + 8`.
pub fn fn_00db7860(e: &mut Engine, this: Ptr) -> u32 {
    !e.mem.u32(this.addr() + 8)
}

// Translated from 00db7870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (`this` = the writer, then the initial CRC, `RET 4`): sets the word at
/// `+6` to 1, the vtable to `0x010D538C` and the running CRC (`+8`) to the complement of the
/// initial CRC. Answers `this`.
pub fn fn_00db7870(e: &mut Engine, this: Ptr, initial: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), 0x010d_538c);
    e.mem.set_u32(this.addr() + 8, !initial);
    this
}

// Translated from 00db7890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCrc32StreamWriter::isOk` (Xbox PDB; `this` = the writer, which is not used, then
/// `(out)`, `RET 4`): stores 1 at the byte `out` and answers `out`.
pub fn hk_crc32_stream_writer_is_ok(e: &mut Engine, _this: Ptr, out: Ptr) -> Ptr {
    e.mem.set_u8(out.addr(), 1);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db_7790, fn_00db7790(u32) -> u32),
        entry!(0x00db_7810, hk_crc32_stream_writer_write(Ptr, Ptr, i32) -> i32),
        entry!(0x00db_7860, fn_00db7860(Ptr) -> u32),
        entry!(0x00db_7870, fn_00db7870(Ptr, u32) -> Ptr),
        entry!(0x00db_7890, hk_crc32_stream_writer_is_ok(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_step_matches_the_crc_table() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00db_7790, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x00db_7790, &args![1u32]).u32(), 0x7707_3096);
        assert_eq!(e.call(0x00db_7790, &args![0xffu32]).u32(), 0x2d02_ef8d);
    }

    #[test]
    fn table_step_uses_the_whole_word() {
        let mut e = Engine::new();
        // Bit 8 only reaches bit 1 after the eight rounds, so nothing is xored in.
        assert_eq!(e.call(0x00db_7790, &args![0x100u32]).u32(), 1);
        // Bit 31 is only shifted down to bit 23 (xor 0x0080_0000 into the table value of 1).
        assert_eq!(
            e.call(0x00db_7790, &args![0x8000_0001u32]).u32(),
            0x7787_3096
        );
    }

    #[test]
    fn writer_computes_the_standard_crc() {
        let mut e = Engine::new();
        let writer = e.mem.alloc(0x10);
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"123456789");
        assert_eq!(e.call(0x00db_7870, &args![writer, 0u32]).u32(), writer);
        assert_eq!(e.mem.u16(writer + 6), 1);
        assert_eq!(e.mem.u32(writer), 0x010d_538c);
        assert_eq!(e.mem.u32(writer + 8), 0xffff_ffff);
        let written = e.call(0x00db_7810, &args![writer, text, 9u32]).u32();
        assert_eq!(written, 9);
        assert_eq!(e.call(0x00db_7860, &args![writer]).u32(), 0xcbf4_3926);
    }

    #[test]
    fn writing_in_pieces_gives_the_same_crc() {
        let mut e = Engine::new();
        let writer = e.mem.alloc(0x10);
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"123456789");
        e.call(0x00db_7870, &args![writer, 0u32]);
        e.call(0x00db_7810, &args![writer, text, 4u32]);
        e.call(0x00db_7810, &args![writer, text + 4, 5u32]);
        assert_eq!(e.call(0x00db_7860, &args![writer]).u32(), 0xcbf4_3926);
    }

    #[test]
    fn write_without_a_positive_length_changes_nothing_and_answers_the_length() {
        let mut e = Engine::new();
        let writer = e.mem.alloc(0x10);
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"abc");
        e.call(0x00db_7870, &args![writer, 0x1234u32]);
        assert_eq!(e.call(0x00db_7860, &args![writer]).u32(), 0x1234);
        assert_eq!(e.call(0x00db_7810, &args![writer, text, 0u32]).u32(), 0);
        assert_eq!(
            e.call(0x00db_7810, &args![writer, text, 0xffff_fffeu32])
                .u32(),
            0xffff_fffe
        );
        assert_eq!(e.call(0x00db_7860, &args![writer]).u32(), 0x1234);
    }

    #[test]
    fn is_ok_sets_the_byte_and_answers_it() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00db_7890, &args![0u32, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
    }
}
