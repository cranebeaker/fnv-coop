//! `fallout shared/havok/run_00623290` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two accessors of a list of cast hits: the address of hit `index` (an
//! array of 0x70-byte entries at +0x10 of `this`, the shape of the all-hits
//! point collector in `cameracaster.rs`) and the object that the collidable
//! stored at +0x48 of that entry belongs to.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00460140`: a plain getter that returns `this + 0x10`, the address of the
/// hit array.
const HIT_ARRAY: u32 = 0x0046_0140;
/// `00807770(array, index)`: `*array + index * 0x70`, the address of entry
/// `index` of an array of 0x70-byte entries (`RET 4`).
const HIT_ENTRY: u32 = 0x0080_7770;
/// `004b59d0`: `this` moved on by the signed byte at `this + 0x10` (a
/// collidable's owner offset), the address of the object that holds it.
const OWNER_ADDRESS: u32 = 0x004b_59d0;
/// `004b5a50(object)` (cdecl, one argument): calls `00c85ef0(local, object,
/// 01267b70)` and returns `00559450` (the `NiPointer` getter) on what that
/// returns. The caller tests the result against 0.
const OBJECT_CAST: u32 = 0x004b_5a50;
/// `0044ddc0`: the word at `this + 8`.
const WORD_AT_8: u32 = 0x0044_ddc0;

/// The word at `+0x48` of a hit entry (a pointer to a root collidable; the
/// entry is 0x70 bytes, laid out like `hkpRootCdPoint` in `cameracaster.rs`).
const ENTRY_COLLIDABLE: u32 = 0x48;

// Translated from 006232f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of hit `index`: `00807770` on the array that `00460140` gives
/// for `this`, with `index` (`RET 4`, returns the entry address).
///
/// The decompiler hangs `index` on the `00460140` call; the disassembly
/// pushes it for `00807770` (`00460140` ends in a plain `RET`).
pub fn fn_006232f0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let array = e.call(HIT_ARRAY, &args![this]).u32();
    e.call(HIT_ENTRY, &args![array, index]).u32()
}

// Translated from 00623290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object hit by hit `index`: the owner of the collidable at `+0x48` of
/// the entry (`004b59d0`), converted by `004b5a50`, and then the word at
/// `+8` of what that gives (`0044ddc0`). 0 when the entry has no collidable
/// or `004b5a50` gives 0.
pub fn fn_00623290(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let entry = fn_006232f0(e, this, index);
    let collidable = e.mem.u32(entry + ENTRY_COLLIDABLE);
    if collidable == 0 {
        return 0;
    }
    let owner = e.call(OWNER_ADDRESS, &args![collidable]).u32();
    let object = e.call(OBJECT_CAST, &args![owner]).u32();
    if object == 0 {
        return 0;
    }
    e.call(WORD_AT_8, &args![object]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_3290, fn_00623290(Ptr, u32) -> u32),
        entry!(0x0062_32f0, fn_006232f0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `this + 0x10`, and `*array + index * 0x70` with the array address
    /// itself standing for `*array`.
    fn engine_with_array_doubles() -> Engine {
        let mut e = Engine::new();
        e.register(HIT_ARRAY, |_, a| (a[0] + 0x10).into_ret());
        e.register(HIT_ENTRY, |_, a| (a[0] + a[1] * 0x70).into_ret());
        e
    }

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn entry_address_is_the_array_element() {
        let mut e = engine_with_array_doubles();
        e.call_log = Some(vec![]);
        let entry = e.call(0x0062_32f0, &args![0x4000u32, 3u32]).u32();
        assert_eq!(entry, 0x4010 + 3 * 0x70);
        assert_eq!(logged(&e, HIT_ARRAY), vec![vec![0x4000]]);
        // `index` goes to the element getter, not to the array getter.
        assert_eq!(logged(&e, HIT_ENTRY), vec![vec![0x4010, 3]]);
    }

    #[test]
    fn hit_object_follows_collidable_owner_cast_and_word() {
        let mut e = engine_with_array_doubles();
        let this = e.mem.alloc(0x400);
        let entry = this + 0x10 + 2 * 0x70;
        e.mem.set_u32(entry + 0x48, 0x7000);
        e.register(OWNER_ADDRESS, |_, a| (a[0] + 0x100).into_ret());
        e.register(OBJECT_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(WORD_AT_8, |_, a| (a[0] + 0x20).into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_3290, &args![this, 2u32]).u32();
        assert_eq!(result, 0x7000 + 0x100 + 1 + 0x20);
        assert_eq!(logged(&e, OWNER_ADDRESS), vec![vec![0x7000]]);
        assert_eq!(logged(&e, OBJECT_CAST), vec![vec![0x7100]]);
        assert_eq!(logged(&e, WORD_AT_8), vec![vec![0x7101]]);
    }

    #[test]
    fn entry_without_collidable_gives_null() {
        let mut e = engine_with_array_doubles();
        let this = e.mem.alloc(0x400);
        e.register(OWNER_ADDRESS, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_3290, &args![this, 1u32]).u32(), 0);
        assert!(logged(&e, OWNER_ADDRESS).is_empty());
    }

    #[test]
    fn failed_cast_gives_null_without_reading_the_word() {
        let mut e = engine_with_array_doubles();
        let this = e.mem.alloc(0x400);
        e.mem.set_u32(this + 0x10 + 0x48, 0x7000);
        e.register(OWNER_ADDRESS, |_, a| a[0].into_ret());
        e.register(OBJECT_CAST, |_, _| 0u32.into_ret());
        e.register(WORD_AT_8, |_, _| 0xdead_beefu32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_3290, &args![this, 0u32]).u32(), 0);
        assert_eq!(logged(&e, OBJECT_CAST), vec![vec![0x7000]]);
        assert!(logged(&e, WORD_AT_8).is_empty());
    }
}
