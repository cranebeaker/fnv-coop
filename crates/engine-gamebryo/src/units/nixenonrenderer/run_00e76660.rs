//! `NiXenonRenderer/run_00e76660` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte tables (one byte used per word) the entry's three codes index: `011bee68` (by the
/// word at `+0xc`), `011beeb0` (by `+0x10`) and `011beecc` (by `+0x14`).
const TYPE_TABLE_011BEE68: u32 = 0x011b_ee68;
const METHOD_TABLE_011BEEB0: u32 = 0x011b_eeb0;
const USAGE_TABLE_011BEECC: u32 = 0x011b_eecc;
/// The 0x12 size words (`0126f508`) that `fn_00e7a760` fills; the answer for a type below 0x12.
const SIZE_TABLE_0126F508: u32 = 0x0126_f508;

// Translated from 00e76660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two stack words: an entry description and a 16-bit
/// stream number in the low half of the second word). Appends one 8-byte element to the array
/// at `+0x2c`, at the index the word `+0x18` holds: the stream (16 bits), the description's
/// 16-bit word `+4`, the bytes of `011bee68`, `011beeb0` and `011beecc` at four times the
/// description's words `+0xc`, `+0x10` and `+0x14`, and the description's byte `+0x18`; then
/// the index goes up by one. Answers the size word at `0126f508` for the code `+0xc` when it is
/// below 0x12 (signed compare: a negative code reads before the table, as the exe does),
/// otherwise 0.
pub fn fn_00e76660(e: &mut Engine, this: Ptr, entry: Ptr, stream: u32) -> u32 {
    let base = this.addr();
    let source = entry.addr();
    let slot = |e: &Engine| {
        let index = e.mem.u32(base + 0x18);
        e.mem.u32(base + 0x2c).wrapping_add(index.wrapping_mul(8))
    };
    let at = slot(e);
    e.mem.set_u16(at, stream as u16);
    let at = slot(e);
    let offset = e.mem.u16(source + 4);
    e.mem.set_u16(at.wrapping_add(2), offset);
    let code = e.mem.u32(source + 0xc);
    let value = e
        .mem
        .u8(TYPE_TABLE_011BEE68.wrapping_add(code.wrapping_mul(4)));
    let at = slot(e);
    e.mem.set_u8(at.wrapping_add(4), value);
    let code = e.mem.u32(source + 0x10);
    let value = e
        .mem
        .u8(METHOD_TABLE_011BEEB0.wrapping_add(code.wrapping_mul(4)));
    let at = slot(e);
    e.mem.set_u8(at.wrapping_add(5), value);
    let code = e.mem.u32(source + 0x14);
    let value = e
        .mem
        .u8(USAGE_TABLE_011BEECC.wrapping_add(code.wrapping_mul(4)));
    let at = slot(e);
    e.mem.set_u8(at.wrapping_add(6), value);
    let value = e.mem.u8(source + 0x18);
    let at = slot(e);
    e.mem.set_u8(at.wrapping_add(7), value);
    let index = e.mem.u32(base + 0x18).wrapping_add(1);
    e.mem.set_u32(base + 0x18, index);
    let code = e.mem.u32(source + 0xc) as i32;
    if code >= 0x12 {
        return 0;
    }
    e.mem
        .u32(SIZE_TABLE_0126F508.wrapping_add((code as u32).wrapping_mul(4)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e7_6660, fn_00e76660(Ptr, Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011b_e000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        // The tables sit next to each other (the method table has 7 words before the usage
        // table), so only their first entries are set.
        for index in 0..7u32 {
            e.mem
                .set_u8(TYPE_TABLE_011BEE68 + index * 4, 0x40 + index as u8);
            e.mem
                .set_u8(METHOD_TABLE_011BEEB0 + index * 4, 0x60 + index as u8);
            e.mem
                .set_u8(USAGE_TABLE_011BEECC + index * 4, 0x80 + index as u8);
        }
        for index in 0..0x12u32 {
            e.mem
                .set_u32(SIZE_TABLE_0126F508 + index * 4, 0x100 + index);
        }
        let this = e.mem.alloc(0x30);
        let array = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x2c, array);
        e.mem.set_u32(this + 0x18, 1);
        let entry = e.mem.alloc(0x1c);
        e.mem.set_u16(entry + 4, 0x1234);
        e.mem.set_u32(entry + 0xc, 3);
        e.mem.set_u32(entry + 0x10, 2);
        e.mem.set_u32(entry + 0x14, 5);
        e.mem.set_u8(entry + 0x18, 9);
        (e, this, array, entry)
    }

    #[test]
    fn fn_00e76660_appends_the_element_and_answers_the_size() {
        let (mut e, this, array, entry) = engine();
        let answer = e
            .call(0x00e7_6660, &args![this, entry, 0xffff_0007u32])
            .u32();
        assert_eq!(answer, 0x103);
        assert_eq!(e.mem.u16(array + 8), 7);
        assert_eq!(e.mem.u16(array + 10), 0x1234);
        assert_eq!(e.mem.u8(array + 12), 0x43);
        assert_eq!(e.mem.u8(array + 13), 0x62);
        assert_eq!(e.mem.u8(array + 14), 0x85);
        assert_eq!(e.mem.u8(array + 15), 9);
        assert_eq!(e.mem.u32(this + 0x18), 2);
    }

    #[test]
    fn fn_00e76660_answers_0_for_a_code_past_the_table() {
        let (mut e, this, _, entry) = engine();
        // The byte tables are read at the code as the exe does (inside the mapped page here).
        e.mem.set_u32(entry + 0xc, 0x12);
        assert_eq!(e.call(0x00e7_6660, &args![this, entry, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x18), 2);
    }
}
