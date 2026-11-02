//! `Havok SDK/run_00e04a60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e04a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `record`; the code in `updateMetadataInplace` at
/// `00e04d5c` stores its address): when the byte at `record + 0xc` is `0x14` and the byte at
/// `record + 0xd` is 2, they become `0x1d` and 0.
pub fn fn_00e04a60(e: &mut Engine, record: Ptr) {
    let at = record.addr();
    if e.mem.u8(at.wrapping_add(0xc)) == 0x14 && e.mem.u8(at.wrapping_add(0xd)) == 2 {
        e.mem.set_u8(at.wrapping_add(0xc), 0x1d);
        e.mem.set_u8(at.wrapping_add(0xd), 0);
    }
}

// Translated from 00e04a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `record`; the code in `updateMetadataInplace` at
/// `00e04d72` stores its address). The record keeps two bytes at `+0xc` and `+0xd` and a 16-bit
/// flag word at `+0x10`. When the byte at `+0xc` is `0x13`, it takes the byte at `+0xd`, that one
/// becomes 0 and bit `0x400` is set in the flag word. Then, when the byte at `+0xc` is `0x18` or
/// `0x1f`: the flag bit `8` (tested on the low byte) is cleared and the byte at `+0xd` set to 4;
/// then flag bit `0x10` is cleared and the byte at `+0xd` set to 6; then flag bit `0x20` is
/// cleared and the byte at `+0xd` set to 8; each in turn, tested on the flag byte as the earlier
/// ones left it.
pub fn fn_00e04a80(e: &mut Engine, record: Ptr) {
    let at = record.addr();
    let flags = at.wrapping_add(0x10);
    if e.mem.u8(at.wrapping_add(0xc)) == 0x13 {
        let next = e.mem.u8(at.wrapping_add(0xd));
        e.mem.set_u8(at.wrapping_add(0xc), next);
        e.mem.set_u8(at.wrapping_add(0xd), 0);
        let word = e.mem.u16(flags) | 0x400;
        e.mem.set_u16(flags, word);
    }
    let kind = e.mem.u8(at.wrapping_add(0xc));
    if kind == 0x18 || kind == 0x1f {
        if e.mem.u8(flags) & 8 != 0 {
            e.mem.set_u8(at.wrapping_add(0xd), 4);
            let word = e.mem.u16(flags) & 0xfff7;
            e.mem.set_u16(flags, word);
        }
        if e.mem.u8(flags) & 0x10 != 0 {
            e.mem.set_u8(at.wrapping_add(0xd), 6);
            let word = e.mem.u16(flags) & 0xffef;
            e.mem.set_u16(flags, word);
        }
        if e.mem.u8(flags) & 0x20 != 0 {
            e.mem.set_u8(at.wrapping_add(0xd), 8);
            let word = e.mem.u16(flags) & 0xffdf;
            e.mem.set_u16(flags, word);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e04a60, fn_00e04a60(Ptr)),
        entry!(0x00e04a80, fn_00e04a80(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A record with the bytes `kind` and `second` at `+0xc` and `+0xd` and the flag word.
    fn record_00e04a60(e: &mut Engine, kind: u8, second: u8, flags: u16) -> u32 {
        let record = e.mem.alloc(0x20);
        e.mem.set_u8(record + 0xc, kind);
        e.mem.set_u8(record + 0xd, second);
        e.mem.set_u16(record + 0x10, flags);
        record
    }

    #[test]
    fn fn_00e04a60_turns_kind_0x14_with_second_byte_2_into_0x1d() {
        let mut e = Engine::new();
        let record = record_00e04a60(&mut e, 0x14, 2, 0);
        e.call(0x00e0_4a60, &args![record]);
        assert_eq!((e.mem.u8(record + 0xc), e.mem.u8(record + 0xd)), (0x1d, 0));
        let other = record_00e04a60(&mut e, 0x14, 3, 0);
        e.call(0x00e0_4a60, &args![other]);
        assert_eq!((e.mem.u8(other + 0xc), e.mem.u8(other + 0xd)), (0x14, 3));
        let other = record_00e04a60(&mut e, 0x15, 2, 0);
        e.call(0x00e0_4a60, &args![other]);
        assert_eq!((e.mem.u8(other + 0xc), e.mem.u8(other + 0xd)), (0x15, 2));
    }

    #[test]
    fn fn_00e04a80_moves_the_second_byte_down_for_kind_0x13() {
        let mut e = Engine::new();
        let record = record_00e04a60(&mut e, 0x13, 0x21, 0x0001);
        e.call(0x00e0_4a80, &args![record]);
        assert_eq!((e.mem.u8(record + 0xc), e.mem.u8(record + 0xd)), (0x21, 0));
        assert_eq!(e.mem.u16(record + 0x10), 0x0401);
    }

    #[test]
    fn fn_00e04a80_rewrites_the_flags_of_kind_0x18_and_0x1f() {
        for kind in [0x18, 0x1f] {
            let mut e = Engine::new();
            let record = record_00e04a60(&mut e, kind, 0x77, 0xff08);
            e.call(0x00e0_4a80, &args![record]);
            assert_eq!(e.mem.u8(record + 0xd), 4);
            assert_eq!(e.mem.u16(record + 0x10), 0xff00);
            let record = record_00e04a60(&mut e, kind, 0x77, 0x0010);
            e.call(0x00e0_4a80, &args![record]);
            assert_eq!(e.mem.u8(record + 0xd), 6);
            assert_eq!(e.mem.u16(record + 0x10), 0);
            let record = record_00e04a60(&mut e, kind, 0x77, 0x0020);
            e.call(0x00e0_4a80, &args![record]);
            assert_eq!(e.mem.u8(record + 0xd), 8);
            assert_eq!(e.mem.u16(record + 0x10), 0);
            // All three bits: the last one decides the second byte.
            let record = record_00e04a60(&mut e, kind, 0x77, 0x0038);
            e.call(0x00e0_4a80, &args![record]);
            assert_eq!(e.mem.u8(record + 0xd), 8);
            assert_eq!(e.mem.u16(record + 0x10), 0);
        }
    }

    #[test]
    fn fn_00e04a80_leaves_other_kinds_and_clear_flags_alone() {
        let mut e = Engine::new();
        let record = record_00e04a60(&mut e, 0x19, 0x77, 0x0038);
        e.call(0x00e0_4a80, &args![record]);
        assert_eq!(e.mem.u8(record + 0xd), 0x77);
        assert_eq!(e.mem.u16(record + 0x10), 0x0038);
        let record = record_00e04a60(&mut e, 0x18, 0x77, 0x0001);
        e.call(0x00e0_4a80, &args![record]);
        assert_eq!(e.mem.u8(record + 0xd), 0x77);
        assert_eq!(e.mem.u16(record + 0x10), 0x0001);
    }
}
