//! `NiXenonRenderer/nixenonrenderstate.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a76540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelFormat::operator==` (engine map `NiPixelFormat::operator_`), `thiscall` with the
/// other format. Equal when `m_eFormat` (+4), `m_uFlags` (+0), `m_eTiling` (+8) and
/// `m_ucBitsPerPixel` (+1) match, in that order, `m_uiRendererHint` (+0xc) matches or either
/// one is 0xffffffff, and the first four of `m_akComponents` (+0x14, 0xc bytes each:
/// `m_eComponent`, `m_eRepresentation`, the bytes `m_ucBitsPerComponent` and `m_bSigned`; Xbox
/// PDB names, the offsets the code reads) match.
pub fn ni_pixel_format_operator_eq(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let mine = this.addr();
    let theirs = other.addr();
    if e.mem.u32(theirs.wrapping_add(4)) != e.mem.u32(mine.wrapping_add(4))
        || e.mem.u8(theirs) != e.mem.u8(mine)
        || e.mem.u32(theirs.wrapping_add(8)) != e.mem.u32(mine.wrapping_add(8))
        || e.mem.u8(theirs.wrapping_add(1)) != e.mem.u8(mine.wrapping_add(1))
    {
        return false;
    }
    let their_hint = e.mem.u32(theirs.wrapping_add(0xc));
    let my_hint = e.mem.u32(mine.wrapping_add(0xc));
    if their_hint != my_hint && my_hint != 0xffff_ffff && their_hint != 0xffff_ffff {
        return false;
    }
    for component in 0..4u32 {
        let at = 0x14 + 0xc * component;
        let same = e.mem.u32(theirs.wrapping_add(at)) == e.mem.u32(mine.wrapping_add(at))
            && e.mem.u32(theirs.wrapping_add(at + 4)) == e.mem.u32(mine.wrapping_add(at + 4))
            && e.mem.u8(theirs.wrapping_add(at + 8)) == e.mem.u8(mine.wrapping_add(at + 8))
            && e.mem.u8(theirs.wrapping_add(at + 9)) == e.mem.u8(mine.wrapping_add(at + 9));
        if !same {
            return false;
        }
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_6540, ni_pixel_format_operator_eq(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod b051_tests {
    use super::*;

    fn format(e: &mut Engine) -> u32 {
        let format = e.mem.alloc(0x50);
        e.mem.set_u8(format, 1);
        e.mem.set_u8(format + 1, 32);
        e.mem.set_u32(format + 4, 6);
        e.mem.set_u32(format + 8, 2);
        e.mem.set_u32(format + 0xc, 0x15);
        for component in 0..4 {
            let at = format + 0x14 + 0xc * component;
            e.mem.set_u32(at, component);
            e.mem.set_u32(at + 4, 1);
            e.mem.set_u8(at + 8, 8);
        }
        format
    }

    #[test]
    fn ni_pixel_format_operator_eq_compares_the_fields_it_reads() {
        let mut e = Engine::new();
        let mine = format(&mut e);
        let theirs = format(&mut e);
        let equal = |e: &mut Engine| e.call(0x00a7_6540, &args![mine, theirs]).bool();
        assert!(equal(&mut e));
        // The renderer hint: equal, or either one 0xffffffff.
        e.mem.set_u32(theirs + 0xc, 0x16);
        assert!(!equal(&mut e));
        e.mem.set_u32(theirs + 0xc, 0xffff_ffff);
        assert!(equal(&mut e));
        e.mem.set_u32(theirs + 0xc, 0x16);
        e.mem.set_u32(mine + 0xc, 0xffff_ffff);
        assert!(equal(&mut e));
        // Each field it reads matters (the last: component 3's `m_bSigned`).
        for (offset, wide) in [
            (4u32, true),
            (0, false),
            (8, true),
            (1, false),
            (0x14, true),
            (0x18, true),
            (0x1c, false),
            (0x1d, false),
            (0x41, false),
        ] {
            let other = format(&mut e);
            if wide {
                e.mem.set_u32(other + offset, 0x77);
            } else {
                e.mem.set_u8(other + offset, 0x77);
            }
            assert!(
                !e.call(0x00a7_6540, &args![mine, other]).bool(),
                "{offset:x}"
            );
        }
        // The extra data (+0x10) and what follows the fourth component are not read.
        let other = format(&mut e);
        e.mem.set_u32(other + 0x10, 0x77);
        e.mem.set_u32(other + 0x44, 0x77);
        assert!(e.call(0x00a7_6540, &args![mine, other]).bool());
    }
}
