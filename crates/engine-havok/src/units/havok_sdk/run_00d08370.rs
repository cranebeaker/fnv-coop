//! `Havok SDK/run_00d08370` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00cd2a00` (`this` only): returns the word at `this + 0x88`.
const READ_WORD_AT_88: u32 = 0x00cd_2a00;

// Translated from 00d08370 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method that does nothing (its only reference is the vtable slot at `010cd06c`; the
/// class is unconfirmed): the `RET 4` takes one word it never reads.
pub fn fn_00d08370(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d08380 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method that does nothing (its only reference is the vtable slot at `010cd028`): the
/// `RET 4` takes one word it never reads.
pub fn fn_00d08380(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d08390 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (vtable slot at `010cd09c`): stores `count` at `+0x38` and returns `count << 5`.
pub fn fn_00d08390(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    e.mem.set_u32(this.addr() + 0x38, count);
    count << 5
}

// Translated from 00d083a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (vtable slot at `010cd0a0`): stores `data` at `+0x30` and `data + (the word at
/// +0x38) * 16` at `+0x34` (the end of 16-byte elements).
pub fn fn_00d083a0(e: &mut Engine, this: Ptr, data: u32) {
    let base = this.addr();
    let count = e.mem.u32(base + 0x38);
    e.mem.set_u32(base + 0x30, data);
    e.mem.set_u32(base + 0x34, (count << 4).wrapping_add(data));
}

// Translated from 00d083e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// One stack word `(object)` (`RET 4`, `ECX` unused): calls `00cd2a00` on `object + (signed byte at +0x10)`
/// (the main part of a multiply-inheriting object) and returns its result, the word at `+0x88`
/// of that part.
pub fn fn_00d083e0(e: &mut Engine, object: Ptr) -> u32 {
    let delta = e.mem.u8(object.addr() + 0x10) as i8 as i32 as u32;
    let part = object.addr().wrapping_add(delta);
    e.call(READ_WORD_AT_88, &args![part]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_8370, fn_00d08370(Ptr, u32)),
        entry!(0x00d0_8380, fn_00d08380(Ptr, u32)),
        entry!(0x00d0_8390, fn_00d08390(Ptr, u32) -> u32),
        entry!(0x00d0_83a0, fn_00d083a0(Ptr, u32)),
        entry!(0x00d0_83e0, fn_00d083e0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_empty_methods_change_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        e.call(0x00d0_8370, &args![this, 1u32]);
        e.call(0x00d0_8380, &args![this, 1u32]);
        assert_eq!(e.mem.bytes(this, 0x10), vec![0xaa; 0x10]);
    }

    /// The count is stored and returned shifted by 5; the data pointer and its end follow.
    #[test]
    fn count_and_data_pointers_are_stored() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00d0_8390, &args![this, 3u32]).u32(), 96);
        assert_eq!(e.mem.u32(this + 0x38), 3);
        e.call(0x00d0_83a0, &args![this, 0x1000u32]);
        assert_eq!(e.mem.u32(this + 0x30), 0x1000);
        assert_eq!(e.mem.u32(this + 0x34), 0x1030);
    }

    /// The helper runs on the part the signed byte at `+0x10` selects (negative and positive)
    /// and its result is returned.
    #[test]
    fn the_adjusted_part_is_asked_for_its_word() {
        let mut e = Engine::new();
        e.register_double(READ_WORD_AT_88, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x88),
            ..Ret::default()
        });
        let block = e.mem.alloc(0x200);
        let object = block + 0x50;
        for (delta, value) in [(0x20i8, 0x111u32), (-0x10, 0x222)] {
            e.mem.set_u8(object + 0x10, delta as u8);
            let part = object.wrapping_add(delta as i32 as u32);
            e.mem.set_u32(part + 0x88, value);
            assert_eq!(e.call(0x00d0_83e0, &args![object]).u32(), value);
        }
    }
}
