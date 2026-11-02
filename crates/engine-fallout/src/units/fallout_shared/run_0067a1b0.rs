//! `fallout shared/run_0067a1b0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of one-bit flag accessors of `TESPackage`: getters and setters for
//! bits of `PackData.iPackFlags` (+0x1C) and `PackData.iFOBehaviorFlags`
//! (+0x22), plus `0067a1b0`, which copies the whole set from another
//! package. The exe has no Xbox names for them, so they are named by
//! address. Two flags (`0x0800_0000`, `0x1000_0000`) are stored inverted:
//! their getter is true when the bit is clear and their setter sets the bit
//! for a zero argument. Setters take one stack word and read its low byte.
//!
//! `0067acd0` looks up a table of pointers (at `0119bcb0`) by the package
//! type. All functions of the unit are translated.

#[allow(unused_imports)]
use crate::prelude::*;
pub use crate::units::fallout_shared::package::TESPackage;

/// Setter of the flag read by `0067a460` (bit `0x0080_0000`, `package.cpp`).
const SET_FLAG_00671A20: u32 = 0x0067_1a20;
/// Getter of the `0x0020_0000` flag (`modelloader.cpp`).
const GET_FLAG_00441B00: u32 = 0x0044_1b00;
/// Reads the word at +0x48 (`pCombatStyle`; the map names it `LowProcess::GetSecondGenericLocation` after folded code).
const GET_SECOND_GENERIC_LOCATION: u32 = 0x0067_33e0;
/// Writes the word at +0x48 (map name `LowProcess::SetSecondGenericLocation`, folded).
const SET_SECOND_GENERIC_LOCATION: u32 = 0x0067_3400;
/// Getter at `0041ca90` (`extradatalist.cpp`): the sign-extended byte at +0x20 (`cPackType`).
const GET_PACK_TYPE_0041CA90: u32 = 0x0041_ca90;
/// Table of pointers indexed by the package type.
const PACK_TYPE_TABLE: u32 = 0x0119_bcb0;

// Translated from 0067a1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies every flag accessor's value from `source` to `this`, in the exe's
/// order: the `iPackFlags` bits, the `iFOBehaviorFlags` bits, and the word at
/// +0x48. Getters and setters outside this file are called by address.
pub fn fn_0067a1b0(e: &mut Engine, this: Ptr<TESPackage>, source: Ptr<TESPackage>) {
    let v = fn_0067a380(e, source);
    fn_0067a3a0(e, this, v as u8);
    let v = fn_0067a850(e, source);
    fn_0067a870(e, this, v as u8);
    let v = fn_0067a8d0(e, source);
    fn_0067a8f0(e, this, v as u8);
    let v = fn_0067a950(e, source);
    fn_0067a970(e, this, v as u8);
    let v = fn_0067a9d0(e, source);
    fn_0067a9f0(e, this, v as u8);
    let v = fn_0067aa50(e, source);
    fn_0067aa70(e, this, v as u8);
    let v = fn_0067aad0(e, source);
    fn_0067aaf0(e, this, v as u8);
    let v = fn_0067ab50(e, source);
    fn_0067ab70(e, this, v as u8);
    let v = fn_0067abd0(e, source);
    fn_0067abf0(e, this, v as u8);
    let v = fn_0067ac50(e, source);
    fn_0067ac70(e, this, v as u8);
    let v = fn_0067a460(e, source);
    e.call(SET_FLAG_00671A20, &args![this, v as u32]);
    let v = fn_0067a690(e, source);
    fn_0067a6b0(e, this, v as u8);
    let v = e.call(GET_FLAG_00441B00, &args![source]).u8();
    fn_0067a640(e, this, v);
    let v = fn_0067a480(e, source);
    fn_0067a4a0(e, this, v as u8);
    let v = fn_0067a4f0(e, source);
    fn_0067a510(e, this, v as u8);
    let v = fn_0067a770(e, source);
    fn_0067a790(e, this, v as u8);
    let v = e.call(GET_SECOND_GENERIC_LOCATION, &args![source]).u32();
    e.call(SET_SECOND_GENERIC_LOCATION, &args![this, v]);
    let v = fn_0067a700(e, source);
    fn_0067a720(e, this, v as u8);
    let v = fn_0067a3f0(e, source);
    fn_0067a410(e, this, v as u8);
    let v = fn_0067a5d0(e, source);
    fn_0067a5f0(e, this, v as u8);
    let v = fn_0067a560(e, source);
    fn_0067a580(e, this, v as u8);
    let v = fn_0067a7e0(e, source);
    fn_0067a800(e, this, v as u8);
}

// Translated from 0067a380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x1000` of `iPackFlags` is set.
pub fn fn_0067a380(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x1000) != 0
}

// Translated from 0067a3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x1000` of `iPackFlags` when `value != 0`, else clears it.
pub fn fn_0067a3a0(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value != 0 {
        flags | 0x1000
    } else {
        flags & !0x1000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x0040_0000` of `iPackFlags` is set.
pub fn fn_0067a3f0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x0040_0000) != 0
}

// Translated from 0067a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x0040_0000` of `iPackFlags` when `value != 0`, else clears it.
pub fn fn_0067a410(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value != 0 {
        flags | 0x0040_0000
    } else {
        flags & !0x0040_0000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x0080_0000` of `iPackFlags` is set.
pub fn fn_0067a460(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x0080_0000) != 0
}

// Translated from 0067a480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x2000` of `iPackFlags` is set.
pub fn fn_0067a480(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x2000) != 0
}

// Translated from 0067a4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x2000` of `iPackFlags` when `value != 0`, else clears it.
pub fn fn_0067a4a0(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value != 0 {
        flags | 0x2000
    } else {
        flags & !0x2000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x0002_0000` of `iPackFlags` is set.
pub fn fn_0067a4f0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x0002_0000) != 0
}

// Translated from 0067a510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x0002_0000` of `iPackFlags` when `value != 0`, else clears it.
pub fn fn_0067a510(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value != 0 {
        flags | 0x0002_0000
    } else {
        flags & !0x0002_0000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x0004_0000` of `iPackFlags` is set.
pub fn fn_0067a560(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x0004_0000) != 0
}

// Translated from 0067a580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x0004_0000` of `iPackFlags` when `value != 0`, else clears it.
pub fn fn_0067a580(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value != 0 {
        flags | 0x0004_0000
    } else {
        flags & !0x0004_0000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x0008_0000` of `iPackFlags` is set.
pub fn fn_0067a5d0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x0008_0000) != 0
}

// Translated from 0067a5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x0008_0000` of `iPackFlags` when `value != 0`, else clears it.
pub fn fn_0067a5f0(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value != 0 {
        flags | 0x0008_0000
    } else {
        flags & !0x0008_0000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x0020_0000` of `iPackFlags` when `value != 0`, else clears it.
pub fn fn_0067a640(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value != 0 {
        flags | 0x0020_0000
    } else {
        flags & !0x0020_0000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x0010_0000` of `iPackFlags` is set.
pub fn fn_0067a690(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x0010_0000) != 0
}

// Translated from 0067a6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x0010_0000` of `iPackFlags` when `value != 0`, else clears it.
pub fn fn_0067a6b0(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value != 0 {
        flags | 0x0010_0000
    } else {
        flags & !0x0010_0000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x0400_0000` of `iPackFlags` is set.
pub fn fn_0067a700(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x0400_0000) != 0
}

// Translated from 0067a720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x0400_0000` of `iPackFlags` when `value != 0`, else clears it.
pub fn fn_0067a720(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value != 0 {
        flags | 0x0400_0000
    } else {
        flags & !0x0400_0000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x0800_0000` of `iPackFlags` is clear.
pub fn fn_0067a770(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x0800_0000) == 0
}

// Translated from 0067a790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x0800_0000` of `iPackFlags` when `value == 0`, else clears it.
pub fn fn_0067a790(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value == 0 {
        flags | 0x0800_0000
    } else {
        flags & !0x0800_0000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x1000_0000` of `iPackFlags` is clear.
pub fn fn_0067a7e0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iPackFlags);
    (flags & 0x1000_0000) == 0
}

// Translated from 0067a800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x1000_0000` of `iPackFlags` when `value == 0`, else clears it.
pub fn fn_0067a800(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if value == 0 {
        flags | 0x1000_0000
    } else {
        flags & !0x1000_0000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 0067a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x1` of `iFOBehaviorFlags` is set.
pub fn fn_0067a850(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    (flags & 0x1) != 0
}

// Translated from 0067a870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x1` of `iFOBehaviorFlags` when `value != 0`, else clears it.
pub fn fn_0067a870(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    let flags = if value != 0 {
        flags | 0x1
    } else {
        flags & !0x1
    };
    e.set(this, TESPackage::iFOBehaviorFlags, flags);
}

// Translated from 0067a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x2` of `iFOBehaviorFlags` is set.
pub fn fn_0067a8d0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    (flags & 0x2) != 0
}

// Translated from 0067a8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x2` of `iFOBehaviorFlags` when `value != 0`, else clears it.
pub fn fn_0067a8f0(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    let flags = if value != 0 {
        flags | 0x2
    } else {
        flags & !0x2
    };
    e.set(this, TESPackage::iFOBehaviorFlags, flags);
}

// Translated from 0067a950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x4` of `iFOBehaviorFlags` is set.
pub fn fn_0067a950(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    (flags & 0x4) != 0
}

// Translated from 0067a970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x4` of `iFOBehaviorFlags` when `value != 0`, else clears it.
pub fn fn_0067a970(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    let flags = if value != 0 {
        flags | 0x4
    } else {
        flags & !0x4
    };
    e.set(this, TESPackage::iFOBehaviorFlags, flags);
}

// Translated from 0067a9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x8` of `iFOBehaviorFlags` is set.
pub fn fn_0067a9d0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    (flags & 0x8) != 0
}

// Translated from 0067a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x8` of `iFOBehaviorFlags` when `value != 0`, else clears it.
pub fn fn_0067a9f0(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    let flags = if value != 0 {
        flags | 0x8
    } else {
        flags & !0x8
    };
    e.set(this, TESPackage::iFOBehaviorFlags, flags);
}

// Translated from 0067aa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x10` of `iFOBehaviorFlags` is set.
pub fn fn_0067aa50(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    (flags & 0x10) != 0
}

// Translated from 0067aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x10` of `iFOBehaviorFlags` when `value != 0`, else clears it.
pub fn fn_0067aa70(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    let flags = if value != 0 {
        flags | 0x10
    } else {
        flags & !0x10
    };
    e.set(this, TESPackage::iFOBehaviorFlags, flags);
}

// Translated from 0067aad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x20` of `iFOBehaviorFlags` is set.
pub fn fn_0067aad0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    (flags & 0x20) != 0
}

// Translated from 0067aaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x20` of `iFOBehaviorFlags` when `value != 0`, else clears it.
pub fn fn_0067aaf0(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    let flags = if value != 0 {
        flags | 0x20
    } else {
        flags & !0x20
    };
    e.set(this, TESPackage::iFOBehaviorFlags, flags);
}

// Translated from 0067ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x40` of `iFOBehaviorFlags` is set.
pub fn fn_0067ab50(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    (flags & 0x40) != 0
}

// Translated from 0067ab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x40` of `iFOBehaviorFlags` when `value != 0`, else clears it.
pub fn fn_0067ab70(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    let flags = if value != 0 {
        flags | 0x40
    } else {
        flags & !0x40
    };
    e.set(this, TESPackage::iFOBehaviorFlags, flags);
}

// Translated from 0067abd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x80` of `iFOBehaviorFlags` is set.
pub fn fn_0067abd0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    (flags & 0x80) != 0
}

// Translated from 0067abf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x80` of `iFOBehaviorFlags` when `value != 0`, else clears it.
pub fn fn_0067abf0(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    let flags = if value != 0 {
        flags | 0x80
    } else {
        flags & !0x80
    };
    e.set(this, TESPackage::iFOBehaviorFlags, flags);
}

// Translated from 0067ac50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag getter: true when bit `0x100` of `iFOBehaviorFlags` is set.
pub fn fn_0067ac50(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    (flags & 0x100) != 0
}

// Translated from 0067ac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package flag setter: sets bit `0x100` of `iFOBehaviorFlags` when `value != 0`, else clears it.
pub fn fn_0067ac70(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    let flags = e.get(this, TESPackage::iFOBehaviorFlags);
    let flags = if value != 0 {
        flags | 0x100
    } else {
        flags & !0x100
    };
    e.set(this, TESPackage::iFOBehaviorFlags, flags);
}

// Translated from 0067acd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the entry of the pointer table at `0119bcb0` for the package type
/// (`0041ca90` reads the sign-extended byte at +0x20).
pub fn fn_0067acd0(e: &mut Engine, this: Ptr<TESPackage>) -> u32 {
    let index = e.call(GET_PACK_TYPE_0041CA90, &args![this]).u32();
    e.mem
        .u32(PACK_TYPE_TABLE.wrapping_add(index.wrapping_mul(4)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067a1b0, fn_0067a1b0(Ptr<TESPackage>, Ptr<TESPackage>)),
        entry!(0x0067a380, fn_0067a380(Ptr<TESPackage>) -> bool),
        entry!(0x0067a3a0, fn_0067a3a0(Ptr<TESPackage>, u8)),
        entry!(0x0067a3f0, fn_0067a3f0(Ptr<TESPackage>) -> bool),
        entry!(0x0067a410, fn_0067a410(Ptr<TESPackage>, u8)),
        entry!(0x0067a460, fn_0067a460(Ptr<TESPackage>) -> bool),
        entry!(0x0067a480, fn_0067a480(Ptr<TESPackage>) -> bool),
        entry!(0x0067a4a0, fn_0067a4a0(Ptr<TESPackage>, u8)),
        entry!(0x0067a4f0, fn_0067a4f0(Ptr<TESPackage>) -> bool),
        entry!(0x0067a510, fn_0067a510(Ptr<TESPackage>, u8)),
        entry!(0x0067a560, fn_0067a560(Ptr<TESPackage>) -> bool),
        entry!(0x0067a580, fn_0067a580(Ptr<TESPackage>, u8)),
        entry!(0x0067a5d0, fn_0067a5d0(Ptr<TESPackage>) -> bool),
        entry!(0x0067a5f0, fn_0067a5f0(Ptr<TESPackage>, u8)),
        entry!(0x0067a640, fn_0067a640(Ptr<TESPackage>, u8)),
        entry!(0x0067a690, fn_0067a690(Ptr<TESPackage>) -> bool),
        entry!(0x0067a6b0, fn_0067a6b0(Ptr<TESPackage>, u8)),
        entry!(0x0067a700, fn_0067a700(Ptr<TESPackage>) -> bool),
        entry!(0x0067a720, fn_0067a720(Ptr<TESPackage>, u8)),
        entry!(0x0067a770, fn_0067a770(Ptr<TESPackage>) -> bool),
        entry!(0x0067a790, fn_0067a790(Ptr<TESPackage>, u8)),
        entry!(0x0067a7e0, fn_0067a7e0(Ptr<TESPackage>) -> bool),
        entry!(0x0067a800, fn_0067a800(Ptr<TESPackage>, u8)),
        entry!(0x0067a850, fn_0067a850(Ptr<TESPackage>) -> bool),
        entry!(0x0067a870, fn_0067a870(Ptr<TESPackage>, u8)),
        entry!(0x0067a8d0, fn_0067a8d0(Ptr<TESPackage>) -> bool),
        entry!(0x0067a8f0, fn_0067a8f0(Ptr<TESPackage>, u8)),
        entry!(0x0067a950, fn_0067a950(Ptr<TESPackage>) -> bool),
        entry!(0x0067a970, fn_0067a970(Ptr<TESPackage>, u8)),
        entry!(0x0067a9d0, fn_0067a9d0(Ptr<TESPackage>) -> bool),
        entry!(0x0067a9f0, fn_0067a9f0(Ptr<TESPackage>, u8)),
        entry!(0x0067aa50, fn_0067aa50(Ptr<TESPackage>) -> bool),
        entry!(0x0067aa70, fn_0067aa70(Ptr<TESPackage>, u8)),
        entry!(0x0067aad0, fn_0067aad0(Ptr<TESPackage>) -> bool),
        entry!(0x0067aaf0, fn_0067aaf0(Ptr<TESPackage>, u8)),
        entry!(0x0067ab50, fn_0067ab50(Ptr<TESPackage>) -> bool),
        entry!(0x0067ab70, fn_0067ab70(Ptr<TESPackage>, u8)),
        entry!(0x0067abd0, fn_0067abd0(Ptr<TESPackage>) -> bool),
        entry!(0x0067abf0, fn_0067abf0(Ptr<TESPackage>, u8)),
        entry!(0x0067ac50, fn_0067ac50(Ptr<TESPackage>) -> bool),
        entry!(0x0067ac70, fn_0067ac70(Ptr<TESPackage>, u8)),
        entry!(0x0067acd0, fn_0067acd0(Ptr<TESPackage>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0067a380_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(!fn_0067a380(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x1000);
        assert!(fn_0067a380(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x1000);
        assert!(!fn_0067a380(&mut e, p));
    }

    #[test]
    fn fn_0067a3a0_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a3a0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x1000);
        fn_0067a3a0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a3a0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
        fn_0067a3a0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x1000);
    }

    #[test]
    fn fn_0067a3f0_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(!fn_0067a3f0(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x0040_0000);
        assert!(fn_0067a3f0(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x0040_0000);
        assert!(!fn_0067a3f0(&mut e, p));
    }

    #[test]
    fn fn_0067a410_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a410(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x0040_0000);
        fn_0067a410(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a410(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
        fn_0067a410(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x0040_0000);
    }

    #[test]
    fn fn_0067a460_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(!fn_0067a460(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x0080_0000);
        assert!(fn_0067a460(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x0080_0000);
        assert!(!fn_0067a460(&mut e, p));
    }

    #[test]
    fn fn_0067a480_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(!fn_0067a480(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x2000);
        assert!(fn_0067a480(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x2000);
        assert!(!fn_0067a480(&mut e, p));
    }

    #[test]
    fn fn_0067a4a0_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a4a0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x2000);
        fn_0067a4a0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a4a0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
        fn_0067a4a0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x2000);
    }

    #[test]
    fn fn_0067a4f0_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(!fn_0067a4f0(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x0002_0000);
        assert!(fn_0067a4f0(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x0002_0000);
        assert!(!fn_0067a4f0(&mut e, p));
    }

    #[test]
    fn fn_0067a510_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a510(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x0002_0000);
        fn_0067a510(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a510(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
        fn_0067a510(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x0002_0000);
    }

    #[test]
    fn fn_0067a560_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(!fn_0067a560(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x0004_0000);
        assert!(fn_0067a560(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x0004_0000);
        assert!(!fn_0067a560(&mut e, p));
    }

    #[test]
    fn fn_0067a580_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a580(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x0004_0000);
        fn_0067a580(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a580(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
        fn_0067a580(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x0004_0000);
    }

    #[test]
    fn fn_0067a5d0_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(!fn_0067a5d0(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x0008_0000);
        assert!(fn_0067a5d0(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x0008_0000);
        assert!(!fn_0067a5d0(&mut e, p));
    }

    #[test]
    fn fn_0067a5f0_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a5f0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x0008_0000);
        fn_0067a5f0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a5f0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
        fn_0067a5f0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x0008_0000);
    }

    #[test]
    fn fn_0067a640_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a640(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x0020_0000);
        fn_0067a640(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a640(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
        fn_0067a640(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x0020_0000);
    }

    #[test]
    fn fn_0067a690_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(!fn_0067a690(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x0010_0000);
        assert!(fn_0067a690(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x0010_0000);
        assert!(!fn_0067a690(&mut e, p));
    }

    #[test]
    fn fn_0067a6b0_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a6b0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x0010_0000);
        fn_0067a6b0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a6b0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
        fn_0067a6b0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x0010_0000);
    }

    #[test]
    fn fn_0067a700_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(!fn_0067a700(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x0400_0000);
        assert!(fn_0067a700(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x0400_0000);
        assert!(!fn_0067a700(&mut e, p));
    }

    #[test]
    fn fn_0067a720_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a720(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x0400_0000);
        fn_0067a720(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a720(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
        fn_0067a720(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x0400_0000);
    }

    #[test]
    fn fn_0067a770_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(fn_0067a770(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x0800_0000);
        assert!(!fn_0067a770(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x0800_0000);
        assert!(fn_0067a770(&mut e, p));
    }

    #[test]
    fn fn_0067a790_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a790(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        fn_0067a790(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x0800_0000);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a790(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x0800_0000);
        fn_0067a790(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
    }

    #[test]
    fn fn_0067a7e0_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        assert!(fn_0067a7e0(&mut e, p));
        e.set(p, TESPackage::iPackFlags, 0x1000_0000);
        assert!(!fn_0067a7e0(&mut e, p));
        e.set(p, TESPackage::iPackFlags, !0x1000_0000);
        assert!(fn_0067a7e0(&mut e, p));
    }

    #[test]
    fn fn_0067a800_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iPackFlags, 0);
        fn_0067a800(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        fn_0067a800(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x1000_0000);
        e.set(p, TESPackage::iPackFlags, !0);
        fn_0067a800(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0x1000_0000);
        fn_0067a800(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iPackFlags), !0);
    }

    #[test]
    fn fn_0067a850_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        assert!(!fn_0067a850(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, 0x1);
        assert!(fn_0067a850(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, !0x1);
        assert!(!fn_0067a850(&mut e, p));
    }

    #[test]
    fn fn_0067a870_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        fn_0067a870(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0x1);
        fn_0067a870(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        e.set(p, TESPackage::iFOBehaviorFlags, !0);
        fn_0067a870(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0);
        fn_0067a870(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0x1);
    }

    #[test]
    fn fn_0067a8d0_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        assert!(!fn_0067a8d0(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, 0x2);
        assert!(fn_0067a8d0(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, !0x2);
        assert!(!fn_0067a8d0(&mut e, p));
    }

    #[test]
    fn fn_0067a8f0_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        fn_0067a8f0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0x2);
        fn_0067a8f0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        e.set(p, TESPackage::iFOBehaviorFlags, !0);
        fn_0067a8f0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0);
        fn_0067a8f0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0x2);
    }

    #[test]
    fn fn_0067a950_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        assert!(!fn_0067a950(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, 0x4);
        assert!(fn_0067a950(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, !0x4);
        assert!(!fn_0067a950(&mut e, p));
    }

    #[test]
    fn fn_0067a970_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        fn_0067a970(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0x4);
        fn_0067a970(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        e.set(p, TESPackage::iFOBehaviorFlags, !0);
        fn_0067a970(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0);
        fn_0067a970(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0x4);
    }

    #[test]
    fn fn_0067a9d0_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        assert!(!fn_0067a9d0(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, 0x8);
        assert!(fn_0067a9d0(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, !0x8);
        assert!(!fn_0067a9d0(&mut e, p));
    }

    #[test]
    fn fn_0067a9f0_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        fn_0067a9f0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0x8);
        fn_0067a9f0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        e.set(p, TESPackage::iFOBehaviorFlags, !0);
        fn_0067a9f0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0);
        fn_0067a9f0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0x8);
    }

    #[test]
    fn fn_0067aa50_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        assert!(!fn_0067aa50(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, 0x10);
        assert!(fn_0067aa50(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, !0x10);
        assert!(!fn_0067aa50(&mut e, p));
    }

    #[test]
    fn fn_0067aa70_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        fn_0067aa70(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0x10);
        fn_0067aa70(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        e.set(p, TESPackage::iFOBehaviorFlags, !0);
        fn_0067aa70(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0);
        fn_0067aa70(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0x10);
    }

    #[test]
    fn fn_0067aad0_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        assert!(!fn_0067aad0(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, 0x20);
        assert!(fn_0067aad0(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, !0x20);
        assert!(!fn_0067aad0(&mut e, p));
    }

    #[test]
    fn fn_0067aaf0_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        fn_0067aaf0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0x20);
        fn_0067aaf0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        e.set(p, TESPackage::iFOBehaviorFlags, !0);
        fn_0067aaf0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0);
        fn_0067aaf0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0x20);
    }

    #[test]
    fn fn_0067ab50_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        assert!(!fn_0067ab50(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, 0x40);
        assert!(fn_0067ab50(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, !0x40);
        assert!(!fn_0067ab50(&mut e, p));
    }

    #[test]
    fn fn_0067ab70_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        fn_0067ab70(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0x40);
        fn_0067ab70(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        e.set(p, TESPackage::iFOBehaviorFlags, !0);
        fn_0067ab70(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0);
        fn_0067ab70(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0x40);
    }

    #[test]
    fn fn_0067abd0_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        assert!(!fn_0067abd0(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, 0x80);
        assert!(fn_0067abd0(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, !0x80);
        assert!(!fn_0067abd0(&mut e, p));
    }

    #[test]
    fn fn_0067abf0_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        fn_0067abf0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0x80);
        fn_0067abf0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        e.set(p, TESPackage::iFOBehaviorFlags, !0);
        fn_0067abf0(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0);
        fn_0067abf0(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0x80);
    }

    #[test]
    fn fn_0067ac50_reads_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        assert!(!fn_0067ac50(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, 0x100);
        assert!(fn_0067ac50(&mut e, p));
        e.set(p, TESPackage::iFOBehaviorFlags, !0x100);
        assert!(!fn_0067ac50(&mut e, p));
    }

    #[test]
    fn fn_0067ac70_sets_or_clears_the_bit() {
        let mut e = Engine::new();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::iFOBehaviorFlags, 0);
        fn_0067ac70(&mut e, p, 1);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0x100);
        fn_0067ac70(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        e.set(p, TESPackage::iFOBehaviorFlags, !0);
        fn_0067ac70(&mut e, p, 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), !0x100);
    }

    #[test]
    fn fn_0067acd0_indexes_the_table_by_type() {
        let mut e = Engine::new();
        e.register(GET_PACK_TYPE_0041CA90, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.map(0x0119_b000, 0x1000);
        e.mem.set_u32(PACK_TYPE_TABLE + 8, 0x1234_5678);
        let p = e.new_object::<TESPackage>();
        assert_eq!(fn_0067acd0(&mut e, p), 0x1234_5678);
    }

    #[test]
    fn fn_0067a1b0_copies_every_flag() {
        let mut e = Engine::new();
        e.register(SET_FLAG_00671A20, |e, a| {
            e.mem.set_u32(0x7004, a[1]);
            Ret::default()
        });
        e.register(GET_FLAG_00441B00, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(GET_SECOND_GENERIC_LOCATION, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x48),
            ..Ret::default()
        });
        e.register(SET_SECOND_GENERIC_LOCATION, |e, a| {
            e.mem.set_u32(a[0] + 0x48, a[1]);
            Ret::default()
        });
        e.map(0x7000, 0x1000);
        let source = e.new_object::<TESPackage>();
        let this = e.new_object::<TESPackage>();
        e.mem.set_u32(source.addr() + 0x48, 0xdead_beef);
        // Source: bit 0x1000 and the 0x100 behavior bit set, the inverted
        // 0x0800_0000 bit set (getter false), the other inverted bit clear.
        e.set(source, TESPackage::iPackFlags, 0x1000 | 0x0800_0000);
        e.set(source, TESPackage::iFOBehaviorFlags, 0x100 | 0x80);
        e.set(this, TESPackage::iPackFlags, 0x0080_0000);
        e.call(0x0067a1b0, &args![this, source]);
        // 0x1000 copied. The 0x0800_0000 bit is set in the source, so its
        // inverted getter is false and the setter (zero argument) sets it.
        let flags = e.get(this, TESPackage::iPackFlags);
        assert_eq!(flags & 0x1000, 0x1000);
        assert_eq!(flags & 0x0800_0000, 0x0800_0000);
        assert_eq!(flags & 0x1000_0000, 0);
        // The 0x0020_0000 getter double returned 1, so the bit is set.
        assert_eq!(flags & 0x0020_0000, 0x0020_0000);
        // Flags set in `this` beforehand but not handled by the stubbed
        // setter of 0x0080_0000 stay as they were.
        assert_eq!(flags & 0x0080_0000, 0x0080_0000);
        assert_eq!(e.get(this, TESPackage::iFOBehaviorFlags), 0x80 | 0x100);
        assert_eq!(e.mem.u32(0x7004), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x48), 0xdead_beef);
    }
}
