//! `NiMain/run_00ee89f0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A five-word record: a fixed-string handle at +0 and four words (the record type is not
//! named here).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{
    acquire_fixed_string, release_fixed_string, EMPTY_FIXED_STRING,
};

// Translated from 00ee89f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the record's constructor: the
/// handle at +0 becomes the empty fixed string's (the word at `0109b220`), the four words
/// after it 0. Answers `this`.
pub fn fn_00ee89f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.mem.set_u32(base, empty);
    for offset in [4u32, 8, 0xc, 0x10] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

// Translated from 00ee8a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the other record; `RET 4`): the record's
/// assignment: when the handles differ the other's is counted and this one's released
/// (neither when it is the empty string's; the count at -8) and the handle copied; then the
/// four words copied. Answers `this`.
pub fn fn_00ee8a10(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let base = this.addr();
    let source = other.addr();
    let handle = e.mem.u32(source);
    if e.mem.u32(base) != handle {
        acquire_fixed_string(e, handle);
        let old = e.mem.u32(base);
        release_fixed_string(e, old);
        e.mem.set_u32(base, handle);
    }
    for offset in [4u32, 8, 0xc, 0x10] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(base + offset, word);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_89f0, fn_00ee89f0(Ptr) -> Ptr),
        entry!(0x00ee_8a10, fn_00ee8a10(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::engine_with_counting;

    #[test]
    fn constructs_an_empty_record() {
        let mut e = engine_with_counting();
        let this = e.mem.alloc(0x14);
        e.mem.write(this, &[0xaa; 0x14]);
        assert_eq!(e.call(0x00ee_89f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0109_b228);
        assert_eq!(e.mem.bytes(this + 4, 0x10), vec![0; 0x10]);
    }

    #[test]
    fn assignment_counts_the_new_handle_and_releases_the_old() {
        let mut e = engine_with_counting();
        let old = e.mem.alloc(0x10) + 8;
        let new = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(old - 8, 2);
        e.mem.set_u32(new - 8, 2);
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        e.mem.set_u32(this, old);
        e.mem.set_u32(other, new);
        for i in 1..5u32 {
            e.mem.set_u32(other + 4 * i, i);
        }
        assert_eq!(e.call(0x00ee_8a10, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u32(this), new);
        assert_eq!((e.mem.u32(old - 8), e.mem.u32(new - 8)), (1, 3));
        assert_eq!(e.mem.bytes(this + 4, 0x10), e.mem.bytes(other + 4, 0x10));
        // The same handle: no counting.
        e.call(0x00ee_8a10, &args![this, other]);
        assert_eq!(e.mem.u32(new - 8), 3);
        // The empty handle is never counted.
        e.mem.set_u32(other, 0x0109_b228);
        e.call(0x00ee_8a10, &args![this, other]);
        assert_eq!((e.mem.u32(this), e.mem.u32(new - 8)), (0x0109_b228, 2));
    }
}
