//! `fallout shared/facegen/bsfacegenkeyframe.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSFaceGenKeyframe` (0xc bytes: vtable, `eType`, `fTime`) and
//! `BSFaceGenKeyframeMultiple` (0x14 bytes: plus `pQuantities` and
//! `iNumQuantities`; its layout is declared in `unplaced/run_00649ba0.rs`).
//! The virtual slots the functions call are the Xbox PDB's (the PC offsets
//! the code uses match them): +0x4 `GetType`, +0x8 `SetType`, +0xc `GetTime`,
//! +0x10 `SetTime`, +0x2c `Compare`, +0x30 `GetStrength`, +0x44 `IsMultiple`,
//! +0x4c `IsValidQuantity`, +0x50 `ResetQuantityArray`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::unplaced::run_00649ba0::BSFaceGenKeyframeMultiple;

/// `BSFaceGenKeyframe`'s vtable.
const KEYFRAME_VTABLE: u32 = 0x0106_53fc;
/// `BSFaceGenKeyframeMultiple`'s vtable.
const MULTIPLE_VTABLE: u32 = 0x0106_544c;
/// The virtual slots (byte offsets) named in the module comment.
const SLOT_GET_TYPE: u32 = 0x4;
const SLOT_SET_TYPE: u32 = 0x8;
const SLOT_GET_TIME: u32 = 0xc;
const SLOT_SET_TIME: u32 = 0x10;
const SLOT_COMPARE: u32 = 0x2c;
const SLOT_GET_STRENGTH: u32 = 0x30;
const SLOT_IS_MULTIPLE: u32 = 0x44;
const SLOT_IS_VALID_QUANTITY: u32 = 0x4c;
const SLOT_RESET_QUANTITY_ARRAY: u32 = 0x50;
/// `0044edb0` (thiscall on a keyframe, returns a word: its quantity count by its
/// use; the map's name `BaseProcess::GetCurrentProcedureIndex` comes from folded code).
const QUANTITY_COUNT_0044EDB0: u32 = 0x0044_edb0;
/// `005dd4e0` (thiscall on a keyframe, one word: an index; the quantity as a
/// `float` in `ST0` by its use).
const QUANTITY_005DD4E0: u32 = 0x005d_d4e0;
/// `0064a4d0` (thiscall on a keyframe: index, `float`; returns `AL`, whether the
/// quantity changed, by its use).
const SET_QUANTITY_0064A4D0: u32 = 0x0064_a4d0;
/// `operator new` (`00401000`) and `operator delete` (`00401030`), cdecl.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `0044ddc0` (thiscall on a list, returns a word; unnamed).
const LIST_0044DDC0: u32 = 0x0044_ddc0;
/// `007b5370` (thiscall on a list, returns the address of an item; unnamed).
const LIST_ITEM_007B5370: u32 = 0x007b_5370;
/// `004ed8c0` (thiscall on a list, one word: the address of the item to add; unnamed).
const LIST_ADD_004ED8C0: u32 = 0x004e_d8c0;
/// `0040eb10` (cdecl, three `float`s, returns `AL`; unnamed): compares a value with
/// another within a tolerance, by its use.
const NEARLY_EQUAL_0040EB10: u32 = 0x0040_eb10;
/// `00408860` (cdecl, one `float`, returns `ST0`; unnamed): its absolute value by its use.
const ABSOLUTE_00408860: u32 = 0x0040_8860;
/// `BSFaceGenManager::SetToZero` (`00653e20`, cdecl: array, count).
const MANAGER_SET_TO_ZERO: u32 = 0x0065_3e20;
/// `00653dc0` (cdecl: array, count, word; unnamed, `bsfacegenmanager.cpp`).
const MANAGER_RESET_00653DC0: u32 = 0x0065_3dc0;
/// `00403e20` (thiscall on a setting, returns the address of its value; unnamed).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings `fn_0064fb10` and `fn_0064fb50` read.
const SETTING_011D57C4: u32 = 0x011d_57c4;
const SETTING_011D577C: u32 = 0x011d_577c;
/// `0.0` and `1.0` (`double`).
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
/// `0.001` (`float`): the tolerance `MergeMorphDataFrom` passes to `0040eb10`.
const TOLERANCE: u32 = 0x0101_7d00;
/// `FLT_MAX` as a `double` and as a `float`.
const MAXIMUM: u32 = 0x0102_31b0;
const MAXIMUM_FLOAT: u32 = 0x0101_6970;

/// A quantity of `keyframe` (`005dd4e0`), in `ST0`.
fn quantity(e: &mut Engine, keyframe: u32, index: u32) -> f64 {
    e.call(QUANTITY_005DD4E0, &args![keyframe, index]).f64()
}

/// Sets a quantity of `keyframe` (`0064a4d0`); whether it changed.
fn set_quantity(e: &mut Engine, keyframe: u32, index: u32, value: u32) -> u8 {
    e.call(SET_QUANTITY_0064A4D0, &args![keyframe, index, value])
        .u8()
}

/// The quantity count of `keyframe` (`0044edb0`).
fn quantity_count(e: &mut Engine, keyframe: u32) -> u32 {
    e.call(QUANTITY_COUNT_0044EDB0, &args![keyframe]).u32()
}

/// `other` when it is a multiple keyframe (slot +0x44), else null.
fn as_multiple(e: &mut Engine, other: u32) -> u32 {
    if other != 0 && e.vcall(other, SLOT_IS_MULTIPLE, &[]).bool() {
        other
    } else {
        0
    }
}

/// Whether `other` has this keyframe's type and quantity count.
fn same_shape(e: &mut Engine, this: u32, other: u32) -> bool {
    let first = e.vcall(this, SLOT_GET_TYPE, &[]).u32();
    let second = e.vcall(other, SLOT_GET_TYPE, &[]).u32();
    first == second && quantity_count(e, this) == quantity_count(e, other)
}

// Translated from 00488d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe::GetTime` (Xbox PDB): `fTime`, loaded into `ST0`.
pub fn bs_face_gen_keyframe_get_time(e: &mut Engine, this: Ptr) -> f64 {
    x87_load32(e.mem.u32(this.addr() + 8))
}

// Translated from 0064ea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe`'s constructor (unnamed in the map; `RET 4`): the vtable, `eType`
/// and an `fTime` of 0. Returns `this`.
pub fn fn_0064ea80(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.mem.set_u32(this.addr(), KEYFRAME_VTABLE);
    e.mem.set_u32(this.addr() + 4, kind);
    e.mem.set_u32(this.addr() + 8, x87_store32(0.0));
    this
}

// Translated from 0064eae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe`'s destructor body (unnamed in the map): stores its vtable.
pub fn fn_0064eae0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), KEYFRAME_VTABLE);
}

// Translated from 0064eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe::SetType` (Xbox PDB): sets `eType`; whether it changed.
pub fn bs_face_gen_keyframe_set_type(e: &mut Engine, this: Ptr, kind: u32) -> bool {
    if kind == e.mem.u32(this.addr() + 4) {
        false
    } else {
        e.mem.set_u32(this.addr() + 4, kind);
        true
    }
}

// Translated from 0064eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe::SetTime` (Xbox PDB): sets `fTime` (quiet) unless it equals `time`
/// (`FUCOMPP`: a NaN is never equal); whether it changed.
pub fn bs_face_gen_keyframe_set_time(e: &mut Engine, this: Ptr, time: f32) -> bool {
    let current = x87_load32(e.mem.u32(this.addr() + 8));
    if current == x87_load32(time.to_bits()) {
        false
    } else {
        e.mem
            .set_u32(this.addr() + 8, x87_quiet_bits(time.to_bits()));
        true
    }
}

// Translated from 0064eb60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064eb60 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple`'s constructor (unnamed in the map; `RET 8`): the base with
/// `kind`, its vtable, no quantities, then `ResetQuantityArray(count, 1)`. Returns `this`.
pub fn fn_0064eb60(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    kind: u32,
    count: u32,
) -> Ptr<BSFaceGenKeyframeMultiple> {
    fn_0064ea80(e, this.cast(), kind);
    e.mem.set_u32(this.addr(), MULTIPLE_VTABLE);
    e.set(this, BSFaceGenKeyframeMultiple::pQuantities, 0);
    e.set(this, BSFaceGenKeyframeMultiple::iNumQuantities, 0);
    bs_face_gen_keyframe_multiple_reset_quantity_array(e, this, count, 1);
    this
}

// Translated from 0064ebe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple`'s scalar deleting destructor (Xbox PDB): the destructor
/// body (`fn_0064ed00`), then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_face_gen_keyframe_multiple_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    flags: u32,
) -> Ptr<BSFaceGenKeyframeMultiple> {
    // `NV_SPEECH_CHECK` (diagnostic only, reads memory): a keyframe deleted while listed.
    speech_check::deleting(e, this.addr(), flags);
    fn_0064ed00(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0064ec10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ec10 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple`'s copy constructor (unnamed in the map; `RET 4`): the base
/// with type -1, the vtable, `other`'s type and time (`SetType`, `SetTime`), a quantity
/// array of `other`'s count (`ResetQuantityArray(count, 1)`) and each of its quantities.
/// Returns `this`.
pub fn fn_0064ec10(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    other: u32,
) -> Ptr<BSFaceGenKeyframeMultiple> {
    let base = this.addr();
    fn_0064ea80(e, this.cast(), 0xffff_ffff);
    e.mem.set_u32(base, MULTIPLE_VTABLE);
    let kind = e.vcall(other, SLOT_GET_TYPE, &[]).u32();
    bs_face_gen_keyframe_set_type(e, this.cast(), kind);
    let time = e.vcall(other, SLOT_GET_TIME, &[]).f64();
    bs_face_gen_keyframe_set_time(e, this.cast(), f32::from_bits(x87_store32(time)));
    e.set(this, BSFaceGenKeyframeMultiple::pQuantities, 0);
    e.set(this, BSFaceGenKeyframeMultiple::iNumQuantities, 0);
    let count = quantity_count(e, other);
    bs_face_gen_keyframe_multiple_reset_quantity_array(e, this, count, 1);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        let value = x87_store32(quantity(e, other, i));
        set_quantity(e, base, i, value);
        i += 1;
    }
    this
}

// Translated from 0064ed00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ed00 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple`'s destructor body (unnamed in the map; Ghidra shows a
/// library-match name): the vtable, `ResetQuantityArray(0, 1)` (frees the array), then
/// the base's destructor body.
pub fn fn_0064ed00(e: &mut Engine, this: Ptr<BSFaceGenKeyframeMultiple>) {
    e.mem.set_u32(this.addr(), MULTIPLE_VTABLE);
    bs_face_gen_keyframe_multiple_reset_quantity_array(e, this, 0, 1);
    fn_0064eae0(e, this.cast());
}

// Translated from 0064ed60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ed60 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple::AddToList` (Xbox PDB, `RET 4`): adds a copy of this
/// keyframe (0x14 bytes, `fn_0064ec10`; null when the allocation fails) to `list`
/// (`004ed8c0`) and returns true. Refused (false) without a list, for a negative time,
/// or when the list is not empty (`0044ddc0`) and its item (`007b5370`) is not a
/// multiple keyframe of the same type.
pub fn bs_face_gen_keyframe_multiple_add_to_list(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    list: u32,
) -> bool {
    // `NV_SPEECH_CHECK` (diagnostic only, reads memory): the list is watched, and this
    // thread is inside it while the translation below runs.
    speech_check::track_list(e, list);
    speech_check::enter(e, list, "AddToList 0064ed60");
    let added = add_to_list(e, this, list);
    speech_check::leave(e, list, "AddToList 0064ed60");
    added
}

/// [`bs_face_gen_keyframe_multiple_add_to_list`]'s translation.
fn add_to_list(e: &mut Engine, this: Ptr<BSFaceGenKeyframeMultiple>, list: u32) -> bool {
    let base = this.addr();
    if list == 0 {
        return false;
    }
    let time = e.vcall(base, SLOT_GET_TIME, &[]).f64();
    let zero: f64 = e.global(ZERO);
    // FCOMP; TEST AH,5; JP: refused only below zero.
    if time < zero {
        return false;
    }
    if e.call(LIST_0044DDC0, &args![list]).u32() != 0 {
        let slot = e.call(LIST_ITEM_007B5370, &args![list]).u32();
        let item = e.mem.u32(slot);
        let multiple = as_multiple(e, item);
        if multiple == 0 {
            return false;
        }
        let kind = e.vcall(multiple, SLOT_GET_TYPE, &[]).u32();
        if kind != e.vcall(base, SLOT_GET_TYPE, &[]).u32() {
            return false;
        }
    }
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        fn_0064ec10(e, Ptr::new(block), base).addr()
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), copy);
        e.call(LIST_ADD_004ED8C0, &args![list, slot]);
    });
    true
}

// Translated from 0064eea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::MergeMorphDataFrom` (Xbox PDB, `RET 0x10`): blends the
/// quantities of `other` (a multiple keyframe of the same type and count, not this one)
/// into this one with `weight` clamped to 0..1: each quantity becomes
/// `other * w + this * (1 - w)`, or `other`'s when `weight >= 1` or this one is
/// `FLT_MAX`; with `skip_unset` clear, `other`'s quantities within 0.001 of 0
/// (`0040eb10`) are left alone; an `other` quantity of `FLT_MAX` sets `FLT_MAX` only with
/// `keep_unset`. Returns whether any quantity changed (false when refused).
pub fn bs_face_gen_keyframe_multiple_merge_morph_data_from(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    other: u32,
    weight: f32,
    skip_unset: u8,
    keep_unset: u8,
) -> bool {
    let base = this.addr();
    let source = as_multiple(e, other);
    let mut changed = 0u8;
    if source == 0 || source == base || !same_shape(e, base, source) {
        return false;
    }
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    let w = x87_load32(weight.to_bits());
    // FCOMP 0; TEST AH,5; JP: 0 only below zero. FCOMP 1; TEST AH,0x41; JNZ: the
    // weight itself unless above one.
    let clamped = if w < zero {
        x87_store32(0.0)
    } else if !(w > one) {
        x87_quiet_bits(weight.to_bits())
    } else {
        x87_store32(1.0)
    };
    let maximum: f64 = e.global(MAXIMUM);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        if skip_unset == 0 {
            let value = x87_store32(quantity(e, source, i));
            let tolerance = x87_quiet_bits(e.global::<u32>(TOLERANCE));
            if e.call(
                NEARLY_EQUAL_0040EB10,
                &args![value, x87_store32(0.0), tolerance],
            )
            .bool()
            {
                i += 1;
                continue;
            }
        }
        let from = quantity(e, source, i);
        // FCOMP FLT_MAX; TEST AH,5; JP: set quantities only below FLT_MAX.
        if from < maximum {
            let mut copy = !(w < one);
            if !copy {
                let current = quantity(e, base, i);
                copy = !(current < maximum);
            }
            let value = if copy {
                x87_store32(quantity(e, source, i))
            } else {
                let current = quantity(e, base, i);
                let rest = x87_sub(1.0, x87_load32(clamped));
                let kept = x87_mul(current, rest);
                let from = quantity(e, source, i);
                let blended = x87_add(x87_mul(from, x87_m32(clamped)), kept);
                x87_quiet_bits(x87_store32(blended))
            };
            changed |= set_quantity(e, base, i, value);
        } else if keep_unset != 0 {
            let value = x87_quiet_bits(e.global::<u32>(MAXIMUM_FLOAT));
            changed |= set_quantity(e, base, i, value);
        }
        i += 1;
    }
    changed != 0
}

// Translated from 0064f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::ClearData` (Xbox PDB, `RET 4`): sets the time to 0 and
/// resets the quantity array (`ResetQuantityArray(count, flag)`); returns whether there
/// was anything to clear (a time other than 0, or a valid quantity).
pub fn bs_face_gen_keyframe_multiple_clear_data(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    flag: u8,
) -> bool {
    let base = this.addr();
    let mut had_data = false;
    let time = e.vcall(base, SLOT_GET_TIME, &[]).f64();
    let zero: f64 = e.global(ZERO);
    // FCOMP; TEST AH,0x44; JNP: nothing only when equal to zero.
    if !(time == zero) {
        had_data = true;
    }
    e.vcall(base, SLOT_SET_TIME, &args![x87_store32(0.0)]);
    let mut i = 0u32;
    while i < quantity_count(e, base) {
        if e.vcall(base, SLOT_IS_VALID_QUANTITY, &args![i]).bool() {
            had_data = true;
            break;
        }
        i += 1;
    }
    let count = quantity_count(e, base);
    e.vcall(
        base,
        SLOT_RESET_QUANTITY_ARRAY,
        &args![count, u32::from(flag)],
    );
    had_data
}

// Translated from 0064f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::CopyFrom` (Xbox PDB, `RET 4`): from a multiple keyframe
/// `other`, takes its type and time (slots +0x8, +0x10), a quantity array of its count
/// (`ResetQuantityArray(count, 1)`) and each quantity.
pub fn bs_face_gen_keyframe_multiple_copy_from(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    other: u32,
) {
    let base = this.addr();
    let source = as_multiple(e, other);
    if source == 0 {
        return;
    }
    let kind = e.vcall(source, SLOT_GET_TYPE, &[]).u32();
    e.vcall(base, SLOT_SET_TYPE, &args![kind]);
    let time = e.vcall(source, SLOT_GET_TIME, &[]).f64();
    e.vcall(base, SLOT_SET_TIME, &args![x87_store32(time)]);
    let count = quantity_count(e, source);
    e.vcall(base, SLOT_RESET_QUANTITY_ARRAY, &args![count, 1u32]);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        let value = x87_store32(quantity(e, source, i));
        set_quantity(e, base, i, value);
        i += 1;
    }
}

// Translated from 0064f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::Compare` (Xbox PDB, `RET 4`): true when `other` is not a
/// multiple keyframe of the same type and count or any quantity differs (`FUCOMPP`: a NaN
/// differs), false when all are equal.
pub fn bs_face_gen_keyframe_multiple_compare(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    other: u32,
) -> bool {
    let base = this.addr();
    let source = as_multiple(e, other);
    if source == 0 || !same_shape(e, source, base) {
        return true;
    }
    let mut i = 0u32;
    while i < quantity_count(e, base) {
        let from = quantity(e, source, i);
        let current = quantity(e, base, i);
        if !(from == current) {
            return true;
        }
        i += 1;
    }
    false
}

// Translated from 0064f760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::GetStrength` (Xbox PDB): the largest valid quantity (slot
/// +0x4c), at least 0; in `ST0`.
pub fn bs_face_gen_keyframe_multiple_get_strength(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
) -> f64 {
    let base = this.addr();
    let mut strength = x87_store32(0.0);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        if e.vcall(base, SLOT_IS_VALID_QUANTITY, &args![i]).bool() {
            let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
            let value = e.mem.u32(array.wrapping_add(i.wrapping_mul(4)));
            // FCOMPP (strength against the quantity); TEST AH,5; JP: taken only above.
            if x87_load32(strength) < x87_load32(value) {
                strength = x87_quiet_bits(value);
            }
        }
        i += 1;
    }
    x87_load32(strength)
}

// Translated from 0064f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::SetToZero` (Xbox PDB): `BSFaceGenManager::SetToZero` on the
/// quantity array.
pub fn bs_face_gen_keyframe_multiple_set_to_zero(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
) {
    let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
    let count = e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities);
    e.call(MANAGER_SET_TO_ZERO, &args![array, count]);
}

// Translated from 0064f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::IsTargetAvailable` (Xbox PDB): whether any quantity is in
/// `(0, 1]` (the exe's three compares; a NaN never is).
pub fn bs_face_gen_keyframe_multiple_is_target_available(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
) -> bool {
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        let value = x87_load32(e.mem.u32(array.wrapping_add(i.wrapping_mul(4))));
        i += 1;
        // FCOMP 0; TEST AH,5; JNP: skipped below zero.
        if value < zero {
            continue;
        }
        // FCOMP 1; TEST AH,0x41; JNZ: skipped above one.
        if value > one {
            continue;
        }
        // FCOMP 0; TEST AH,0x41; JNZ: skipped unless above zero.
        if !(value > zero) {
            continue;
        }
        return true;
    }
    false
}

// Translated from 0064f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::UpdateToTarget` (Xbox PDB, `RET 8`): moves each valid
/// quantity towards `target`'s by at most `step = (strength * s2 + s1) * time` (`s1`, `s2`
/// from `fn_0064fb10`, `fn_0064fb50`, the strength from `target`'s slot +0x30), reaching
/// it when it is nearer than that. Only for a positive `time` (or NaN) and a multiple
/// `target` of the same type and count that differs (`Compare`); returns whether it ran.
pub fn bs_face_gen_keyframe_multiple_update_to_target(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    time: f32,
    target: u32,
) -> bool {
    let base = this.addr();
    let source = as_multiple(e, target);
    let zero: f64 = e.global(ZERO);
    let t = x87_load32(time.to_bits());
    // FCOMP; TEST AH,0x41; JNP: refused below or equal to zero.
    if t <= zero {
        return false;
    }
    if source == 0
        || !same_shape(e, source, base)
        || !e.vcall(base, SLOT_COMPARE, &args![source]).bool()
    {
        return false;
    }
    let offset = fn_0064fb10(e);
    let scale = fn_0064fb50(e);
    let strength = e.vcall(target, SLOT_GET_STRENGTH, &[]).f64();
    let step = x87_mul(
        x87_add(x87_mul(strength, scale), offset),
        x87_m32(time.to_bits()),
    );
    let step = x87_store32(step);
    let mut i = 0u32;
    while i < quantity_count(e, base) {
        if !e.vcall(base, SLOT_IS_VALID_QUANTITY, &args![i]).bool() {
            i += 1;
            continue;
        }
        let goal = quantity(e, source, i);
        let current = quantity(e, base, i);
        let difference = x87_store32(x87_sub(goal, current));
        let distance = e
            .call(ABSOLUTE_00408860, &args![x87_quiet_bits(difference)])
            .f64();
        let d = x87_load32(difference);
        // FCOMPP (step against the distance); TEST AH,0x41; JNZ: reached only when the
        // step is above it.
        if x87_load32(step) > distance {
            let value = x87_store32(quantity(e, source, i));
            set_quantity(e, base, i, value);
        } else if d < zero {
            let value = x87_store32(x87_sub(quantity(e, base, i), x87_m32(step)));
            set_quantity(e, base, i, x87_quiet_bits(value));
        } else if d > zero {
            let value = x87_store32(x87_add(quantity(e, base, i), x87_m32(step)));
            set_quantity(e, base, i, x87_quiet_bits(value));
        }
        i += 1;
    }
    true
}

/// A setting's `float` (`00403e20`) when it is above zero, else 0; in `ST0`.
fn positive_setting(e: &mut Engine, setting: u32) -> f64 {
    let address = e.call(SETTING_VALUE, &args![setting]).u32();
    let zero: f64 = e.global(ZERO);
    // FCOMP; TEST AH,0x41; JNZ: 0 unless above zero.
    let value = if x87_load32(e.mem.u32(address)) > zero {
        let address = e.call(SETTING_VALUE, &args![setting]).u32();
        x87_quiet_bits(e.mem.u32(address))
    } else {
        x87_store32(0.0)
    };
    x87_load32(value)
}

// Translated from 0064fb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the setting at `011d57c4` when above zero, else 0 (`ST0`).
pub fn fn_0064fb10(e: &mut Engine) -> f64 {
    positive_setting(e, SETTING_011D57C4)
}

// Translated from 0064fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the setting at `011d577c` when above zero, else 0 (`ST0`).
pub fn fn_0064fb50(e: &mut Engine) -> f64 {
    positive_setting(e, SETTING_011D577C)
}

// Translated from 0064fb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::IsValidQuantity` (Xbox PDB, `RET 4`): whether `index` is
/// in range and its quantity is in `[0, 1]` (a NaN is not).
pub fn bs_face_gen_keyframe_multiple_is_valid_quantity(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    index: u32,
) -> bool {
    if index >= e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        return false;
    }
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
    let value = x87_load32(e.mem.u32(array.wrapping_add(index.wrapping_mul(4))));
    // FCOMP 0; TEST AH,1; JNZ: invalid below zero or unordered.
    if value < zero || value.is_nan() {
        return false;
    }
    // FCOMP 1; TEST AH,0x41; JP: valid only below or equal to one.
    value <= one
}

// Translated from 0064fc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::ResetQuantityArray` (Xbox PDB, `RET 8`): for a new count,
/// frees the array and allocates `count` floats (`operator new`, the size all ones when
/// `count * 4` overflows), filled by `00653dc0` with `flag`; then fills the array again
/// (`00653dc0`) whenever there are quantities.
pub fn bs_face_gen_keyframe_multiple_reset_quantity_array(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    count: u32,
    flag: u8,
) {
    if count != e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        if array != 0 {
            e.call(OPERATOR_DELETE, &args![array]);
            e.set(this, BSFaceGenKeyframeMultiple::pQuantities, 0);
        }
        e.set(this, BSFaceGenKeyframeMultiple::iNumQuantities, count);
        if count != 0 {
            let size = count.checked_mul(4).unwrap_or(u32::MAX);
            let block = e.call(OPERATOR_NEW, &args![size]).u32();
            e.set(this, BSFaceGenKeyframeMultiple::pQuantities, block);
            e.call(
                MANAGER_RESET_00653DC0,
                &args![block, count, u32::from(flag)],
            );
        }
    }
    let count = e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities);
    if count != 0 {
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        e.call(
            MANAGER_RESET_00653DC0,
            &args![array, count, u32::from(flag)],
        );
    }
}

// Translated from 0064eab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframe`'s scalar deleting destructor (Xbox PDB, `RET 4`): the destructor
/// body (`fn_0064eae0`), then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_face_gen_keyframe_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0064eae0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// A new copy of the multiple keyframe `source` (0x14 bytes, `fn_0064ec10`), or null when
/// the allocation fails.
fn new_copy(e: &mut Engine, source: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        0
    } else {
        fn_0064ec10(e, Ptr::new(block), source).addr()
    }
}

// Translated from 0064f0b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064f0b0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple::MergeMorphDataFrom_ov2` (Xbox PDB, `RET 0xc`): makes this
/// keyframe a blend of the multiple keyframes `first` and `second` (of one type, not the
/// same one; a copy stands in for either when it is this one, destroyed afterwards through
/// slot 0 with 1): this keyframe takes `first`'s type, then for each of its quantities
/// (slot +0x4c on the sources: whether valid) `first`'s when `weight` (clamped to 0..1) is
/// 0 or `second`'s is not valid, `second`'s when it is 1 or `first`'s is not valid, else
/// `first * (1 - w) + second * w`. The quantity counts of both are read (`0044edb0`) but
/// not used. Returns whether the last quantity set changed (false when refused).
pub fn bs_face_gen_keyframe_multiple_merge_morph_data_from_ov2(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    first: u32,
    second: u32,
    weight: f32,
) -> bool {
    let base = this.addr();
    let mut from_first = as_multiple(e, first);
    let mut from_second = as_multiple(e, second);
    let mut first_copied = false;
    let mut second_copied = false;
    let mut changed = 0u8;
    if from_first == 0 || from_second == 0 || from_first == from_second {
        return false;
    }
    let first_kind = e.vcall(from_first, SLOT_GET_TYPE, &[]).u32();
    let second_kind = e.vcall(from_second, SLOT_GET_TYPE, &[]).u32();
    if first_kind != second_kind {
        return false;
    }
    if from_first == base {
        from_first = new_copy(e, base);
        first_copied = true;
    } else if from_second == base {
        from_second = new_copy(e, base);
        second_copied = true;
    }
    let zero: f64 = e.global(ZERO);
    let one: f64 = e.global(ONE);
    let w = x87_load32(weight.to_bits());
    // FCOMP 0; TEST AH,5; JP: 0 only below zero. FCOMP 1; TEST AH,0x41; JNZ: the
    // weight itself unless above one.
    let clamped = if w < zero {
        x87_store32(0.0)
    } else if !(w > one) {
        x87_quiet_bits(weight.to_bits())
    } else {
        x87_store32(1.0)
    };
    // The smaller of the two counts is computed and kept in a local nobody reads.
    let first_count = quantity_count(e, from_first);
    let second_count = quantity_count(e, from_second);
    if first_count < second_count {
        quantity_count(e, from_first);
    } else {
        quantity_count(e, from_second);
    }
    let kind = e.vcall(from_first, SLOT_GET_TYPE, &[]).u32();
    e.vcall(base, SLOT_SET_TYPE, &args![kind]);
    let w = x87_load32(clamped);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        // FCOMP; TEST AH,0x44; JNP: taken when equal.
        let use_first = w == zero
            || !e
                .vcall(from_second, SLOT_IS_VALID_QUANTITY, &args![i])
                .bool();
        if use_first {
            if e.vcall(from_first, SLOT_IS_VALID_QUANTITY, &args![i])
                .bool()
            {
                let value = x87_store32(quantity(e, from_first, i));
                changed = set_quantity(e, base, i, value);
            }
        } else if w == one
            || !e
                .vcall(from_first, SLOT_IS_VALID_QUANTITY, &args![i])
                .bool()
        {
            if e.vcall(from_second, SLOT_IS_VALID_QUANTITY, &args![i])
                .bool()
            {
                let value = x87_store32(quantity(e, from_second, i));
                changed = set_quantity(e, base, i, value);
            }
        } else if e
            .vcall(from_first, SLOT_IS_VALID_QUANTITY, &args![i])
            .bool()
            && e.vcall(from_second, SLOT_IS_VALID_QUANTITY, &args![i])
                .bool()
        {
            let kept = quantity(e, from_first, i);
            let rest = x87_sub(1.0, x87_load32(clamped));
            // FSTP double: the x87 value as a double.
            let kept = x87_mul(kept, rest);
            let other = quantity(e, from_second, i);
            let blended = x87_add(x87_mul(other, x87_m32(clamped)), kept);
            let value = x87_quiet_bits(x87_store32(blended));
            changed = set_quantity(e, base, i, value);
        }
        i += 1;
    }
    if first_copied && from_first != 0 {
        e.vcall(from_first, 0, &args![1u32]);
    }
    if second_copied && from_second != 0 {
        e.vcall(from_second, 0, &args![1u32]);
    }
    changed != 0
}

// Translated from 0064f530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064f530 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenKeyframeMultiple::CreateCopy` (Xbox PDB): a new copy of this keyframe (0x14
/// bytes, `fn_0064ec10`), or null when the allocation fails.
pub fn bs_face_gen_keyframe_multiple_create_copy(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
) -> u32 {
    new_copy(e, this.addr())
}

// Translated from 0064f7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenKeyframeMultiple::IsEmpty` (Xbox PDB): false when any valid quantity (slot
/// +0x4c) is above 0, else true.
pub fn bs_face_gen_keyframe_multiple_is_empty(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
) -> bool {
    let base = this.addr();
    let zero: f64 = e.global(ZERO);
    let mut i = 0u32;
    while i < e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        if e.vcall(base, SLOT_IS_VALID_QUANTITY, &args![i]).bool() {
            let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
            let value = x87_load32(e.mem.u32(array.wrapping_add(i.wrapping_mul(4))));
            // FCOMP 0; TEST AH,0x41; JNZ: not empty only above zero.
            if value > zero {
                return false;
            }
        }
        i += 1;
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_eab0, bs_face_gen_keyframe_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x0064_f0b0,
            bs_face_gen_keyframe_multiple_merge_morph_data_from_ov2(
                Ptr<BSFaceGenKeyframeMultiple>,
                u32,
                u32,
                f32,
            ) -> bool
        ),
        entry!(
            0x0064_f530,
            bs_face_gen_keyframe_multiple_create_copy(Ptr<BSFaceGenKeyframeMultiple>) -> u32
        ),
        entry!(
            0x0064_f7e0,
            bs_face_gen_keyframe_multiple_is_empty(Ptr<BSFaceGenKeyframeMultiple>) -> bool
        ),
        entry!(0x0048_8d50, bs_face_gen_keyframe_get_time(Ptr) -> f64),
        entry!(0x0064_ea80, fn_0064ea80(Ptr, u32) -> Ptr),
        entry!(0x0064_eae0, fn_0064eae0(Ptr)),
        entry!(0x0064_eb00, bs_face_gen_keyframe_set_type(Ptr, u32) -> bool),
        entry!(0x0064_eb30, bs_face_gen_keyframe_set_time(Ptr, f32) -> bool),
        entry!(
            0x0064_eb60,
            fn_0064eb60(Ptr<BSFaceGenKeyframeMultiple>, u32, u32) -> Ptr<BSFaceGenKeyframeMultiple>
        ),
        entry!(
            0x0064_ebe0,
            bs_face_gen_keyframe_multiple_scalar_deleting_destructor(
                Ptr<BSFaceGenKeyframeMultiple>,
                u32,
            ) -> Ptr<
                BSFaceGenKeyframeMultiple,
            >
        ),
        entry!(
            0x0064_ec10,
            fn_0064ec10(Ptr<BSFaceGenKeyframeMultiple>, u32) -> Ptr<BSFaceGenKeyframeMultiple>
        ),
        entry!(0x0064_ed00, fn_0064ed00(Ptr<BSFaceGenKeyframeMultiple>)),
        entry!(
            0x0064_ed60,
            bs_face_gen_keyframe_multiple_add_to_list(Ptr<BSFaceGenKeyframeMultiple>, u32) -> bool
        ),
        entry!(
            0x0064_eea0,
            bs_face_gen_keyframe_multiple_merge_morph_data_from(
                Ptr<BSFaceGenKeyframeMultiple>,
                u32,
                f32,
                u8,
                u8,
            ) -> bool
        ),
        entry!(
            0x0064_f480,
            bs_face_gen_keyframe_multiple_clear_data(Ptr<BSFaceGenKeyframeMultiple>, u8) -> bool
        ),
        entry!(
            0x0064_f5b0,
            bs_face_gen_keyframe_multiple_copy_from(Ptr<BSFaceGenKeyframeMultiple>, u32)
        ),
        entry!(
            0x0064_f690,
            bs_face_gen_keyframe_multiple_compare(Ptr<BSFaceGenKeyframeMultiple>, u32) -> bool
        ),
        entry!(
            0x0064_f760,
            bs_face_gen_keyframe_multiple_get_strength(Ptr<BSFaceGenKeyframeMultiple>) -> f64
        ),
        entry!(
            0x0064_f850,
            bs_face_gen_keyframe_multiple_set_to_zero(Ptr<BSFaceGenKeyframeMultiple>)
        ),
        entry!(
            0x0064_f880,
            bs_face_gen_keyframe_multiple_is_target_available(
                Ptr<BSFaceGenKeyframeMultiple>,
            ) -> bool
        ),
        entry!(
            0x0064_f900,
            bs_face_gen_keyframe_multiple_update_to_target(
                Ptr<BSFaceGenKeyframeMultiple>,
                f32,
                u32,
            ) -> bool
        ),
        entry!(0x0064_fb10, fn_0064fb10() -> f64),
        entry!(0x0064_fb50, fn_0064fb50() -> f64),
        entry!(
            0x0064_fb90,
            bs_face_gen_keyframe_multiple_is_valid_quantity(
                Ptr<BSFaceGenKeyframeMultiple>,
                u32,
            ) -> bool
        ),
        entry!(
            0x0064_fc00,
            bs_face_gen_keyframe_multiple_reset_quantity_array(
                Ptr<BSFaceGenKeyframeMultiple>,
                u32,
                u8,
            )
        ),
    ]
}

/// `NV_SPEECH_CHECK`: diagnostic only, not exe behaviour. Reads, never writes, game memory.
///
/// Watches the keyframe lists of each `BSFaceGenAnimationData` it meets (the speech lists:
/// modifiers `+0x88`, phonemes `+0xbc`, custom morphs `+0xf0`) and any other list
/// `AddToList` (`0064ed60`) is handed, and stops the run (a panic whose message starts
/// `speech check:`, after `speech:` lines with the details) at the first of:
///
/// - the same keyframe in two list slots, or one node in two lists;
/// - a keyframe deleted (`0064ebe0` with bit 0) or its block freed (`00aa4060`, `free`)
///   while still in a list. `ClearSpeechKeyframeLists` (`0064aec0`) deletes the head's
///   keyframe before it removes the head (as the exe does), so a deleted head is let be
///   until its deleter removes it (`007b5390`), on the same thread, before it returns;
/// - a listed item that is not a live `BSFaceGenKeyframeMultiple` (vtable `0106544c`);
/// - a list node freed (`0045cee0`), handed out (`0043a010`) or found on the node pool's
///   free list (`011c5f58`, looked at every call) while still linked;
/// - a list whose links do not match its count, tail or previous links;
/// - two game threads in one list at once (inside `AddToList`, `UpdateKeyframeList`
///   (`0064ca60`), `ClearSpeechKeyframeLists`' loops or `RemoveHead`).
///
/// Every event is numbered and keeps its list, node, keyframe, game thread, the face's lock
/// words (`+0x1a0`) and call stack; the stop names the earlier event the violation conflicts
/// with and the event that found it.
///
/// - Any value but `0`: checks at the list functions, deletes, frees and node-pool calls,
///   and the pool's free-list head at every call.
/// - With `calls` (comma-separated): also walks every list at every call (slow), naming the
///   call where a list changed outside the list functions.
/// - With `nooverlap`: two threads in one list at once is not a stop.
///
/// It takes `Engine::call_hook` (calling on to a hook that was there first) and
/// `Engine::free_hook`.
pub mod speech_check {
    use super::MULTIPLE_VTABLE;
    use crate::prelude::*;
    use std::cell::Cell;
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::fmt::Write as _;
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use std::sync::Mutex;

    /// `Engine::call_hook`'s type.
    type CallHook = fn(&mut Engine, u32, bool);

    /// What the check does (`NV_SPEECH_CHECK`).
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Settings {
        pub on: bool,
        pub calls: bool,
        pub overlap: bool,
    }

    impl Settings {
        pub const OFF: Settings = Settings {
            on: false,
            calls: false,
            overlap: false,
        };

        /// The settings for a value of `NV_SPEECH_CHECK` (`None`: not set).
        pub fn parse(value: Option<&str>) -> Settings {
            match value {
                None => Self::OFF,
                Some(v) if v.trim().is_empty() || v.trim() == "0" => Self::OFF,
                Some(v) => {
                    let has =
                        |word: &str| v.split(',').any(|w| w.trim().eq_ignore_ascii_case(word));
                    Settings {
                        on: true,
                        calls: has("calls"),
                        overlap: !has("nooverlap"),
                    }
                }
            }
        }
    }

    /// The node pool's free-list head (`0043a010` pops, `0045cee0` pushes).
    const POOL_FREE_HEAD: u32 = 0x011c_5f58;
    /// The speech lists of a `BSFaceGenAnimationData`.
    const FACE_LISTS: [u32; 3] = [0x88, 0xbc, 0xf0];
    /// The face's `AnimLock` (PC `+0x1a0`).
    const ANIM_LOCK: u32 = 0x1a0;
    /// `BSFaceGenKeyframeMultiple`'s scalar deleting destructor.
    const DELETING_DESTRUCTOR: u32 = 0x0064_ebe0;
    /// Where a walk gives up (a cycle) when the count is no guide.
    const MAX_WALK: usize = 1 << 16;
    /// How many free nodes past a new pool head are looked at.
    const POOL_LOOK: usize = 16;
    const STACK_KEEP: usize = 16;
    const RECENT_KEEP: usize = 48;
    const HISTORY_KEEP: usize = 1 << 17;
    /// How many slots a stop prints of the list.
    const DUMP_SLOTS: usize = 48;

    /// What an event was.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum What {
        Track,
        Add,
        Remove,
        Delete,
        Free,
        NodeFree,
        NodeAlloc,
        /// The pool's head moved; at entry to (`false`) or return from (`true`) a call.
        Pool(u32, bool),
        Enter(&'static str),
        Leave(&'static str),
        /// A check at a call (`calls`) or at a named point.
        Call(u32, bool),
        At(&'static str),
    }

    impl std::fmt::Display for What {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                What::Track => write!(f, "list first seen"),
                What::Add => write!(f, "added (00559a70)"),
                What::Remove => write!(f, "head removed (RemoveHead 007b5390)"),
                What::Delete => write!(f, "keyframe deleted (0064ebe0)"),
                What::Free => write!(f, "block freed (00aa4060 / free)"),
                What::NodeFree => write!(f, "node freed (0045cee0)"),
                What::NodeAlloc => write!(f, "node handed out (0043a010)"),
                What::Pool(addr, true) => write!(f, "pool head moved, return from {addr:08x}"),
                What::Pool(addr, false) => write!(f, "pool head moved, entry to {addr:08x}"),
                What::Enter(name) => write!(f, "entered {name}"),
                What::Leave(name) => write!(f, "left {name}"),
                What::Call(addr, true) => write!(f, "check at return from {addr:08x}"),
                What::Call(addr, false) => write!(f, "check at entry to {addr:08x}"),
                What::At(name) => write!(f, "check at {name}"),
            }
        }
    }

    /// One numbered event.
    #[derive(Clone, Debug)]
    struct Event {
        seq: u64,
        what: What,
        list: u32,
        node: u32,
        keyframe: u32,
        thread: u32,
        /// The face's lock words (`+0x1a0`, `+0x1a4`) when the list's face is known.
        lock: Option<(u32, u32)>,
        /// The call stack, innermost first.
        stack: Vec<u32>,
    }

    impl std::fmt::Display for Event {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "#{} {}: list {:08x} node {:08x} keyframe {:08x}, thread {}",
                self.seq, self.what, self.list, self.node, self.keyframe, self.thread
            )?;
            if let Some((a, b)) = self.lock {
                write!(f, ", face lock [{a:08x} {b:08x}]")?;
            }
            write!(f, ", stack {}", stack_text(&self.stack))
        }
    }

    impl Event {
        /// The event in a few words (the panic message).
        fn short(&self) -> String {
            let frames: Vec<String> = self
                .stack
                .iter()
                .take(4)
                .map(|a| format!("{a:08x}"))
                .collect();
            format!(
                "#{} {} on thread {} in {}",
                self.seq,
                self.what,
                self.thread,
                frames.join(" < ")
            )
        }
    }

    /// A watched list as the last clean walk left it.
    #[derive(Default)]
    struct List {
        /// Its face, 0 when not known.
        face: u32,
        /// (node, item) from the head.
        slots: Vec<(u32, u32)>,
        /// A `RemoveHead` under way (the list is half changed).
        busy: Option<Event>,
        /// The threads inside a function that uses the list, with the function.
        users: Vec<(u32, &'static str, Event)>,
        /// The last event that left it clean.
        last: Option<Event>,
    }

    /// A head keyframe deleted before its node is removed (`0064aec0`'s order).
    #[derive(Clone)]
    struct Pending {
        keyframe: u32,
        list: u32,
        node: u32,
        thread: u32,
        /// The deleter must stay at `call_stack[frame - 1]` (0: any frame).
        frame: usize,
        deleter: u32,
        event: Event,
    }

    #[derive(Default)]
    struct State {
        seq: u64,
        lists: HashMap<u32, List>,
        /// Each linked node's list.
        node_of: HashMap<u32, u32>,
        /// Each listed keyframe's list and node.
        slot_of: HashMap<u32, (u32, u32)>,
        /// The event that linked each node.
        added: HashMap<u32, Event>,
        /// The last event that named each keyframe.
        history: HashMap<u32, Event>,
        /// Keyframes deleted since they were last added.
        deleted: HashMap<u32, Event>,
        pending: Option<Pending>,
        recent: VecDeque<Event>,
        pool_head: u32,
        installed: bool,
        stopped: bool,
    }

    /// A violation: what, the earlier event it conflicts with, the event that found it.
    struct Violation {
        summary: String,
        earlier: Option<Event>,
        now: Event,
        list: u32,
        extra: Vec<String>,
    }

    // The state is one for the process, not per OS thread: the thread host runs each game
    // thread on its own OS thread (one at a time), and a per-thread state let the second
    // thread take the call hook again, with the check itself as the hook it calls on: an
    // endless recursion at start-up (p190).
    static STATE: Mutex<Option<State>> = Mutex::new(None);
    /// A call hook that was there before the check took it (called on; never the check's own).
    static PREVIOUS_HOOK: Mutex<Option<CallHook>> = Mutex::new(None);
    static HAS_PREVIOUS_HOOK: AtomicBool = AtomicBool::new(false);
    /// The pool head at the last call (`on_call`'s quick test).
    static POOL_SEEN: AtomicU32 = AtomicU32::new(0);
    /// Set while a hook of the check runs: a hook reached again from inside one returns at
    /// once (the hooks call nothing of the engine's, so this is a safeguard).
    static IN_HOOK: AtomicBool = AtomicBool::new(false);

    thread_local! {
        static FORCED: Cell<Option<Settings>> = const { Cell::new(None) };
        static FORCED_THREAD: Cell<Option<u32>> = const { Cell::new(None) };
    }

    /// The settings from `NV_SPEECH_CHECK` (read once), unless a test forced some.
    pub fn settings() -> Settings {
        if let Some(settings) = FORCED.with(Cell::get) {
            return settings;
        }
        static SETTINGS: std::sync::OnceLock<Settings> = std::sync::OnceLock::new();
        *SETTINGS.get_or_init(|| Settings::parse(std::env::var("NV_SPEECH_CHECK").ok().as_deref()))
    }

    #[inline]
    fn on() -> bool {
        settings().on
    }

    /// Sets the settings for this thread and forgets what was seen (tests; the state is the
    /// process's, so tests that turn the check on must not run beside each other).
    pub fn force(settings: Settings) {
        FORCED.with(|forced| forced.set(Some(settings)));
        FORCED_THREAD.with(|t| t.set(None));
        set_previous_hook(None);
        POOL_SEEN.store(0, Ordering::Relaxed);
        IN_HOOK.store(false, Ordering::Relaxed);
        *lock_state() = None;
    }

    fn lock_state() -> std::sync::MutexGuard<'static, Option<State>> {
        STATE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn previous_hook() -> Option<CallHook> {
        if !HAS_PREVIOUS_HOOK.load(Ordering::Relaxed) {
            return None;
        }
        *PREVIOUS_HOOK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn set_previous_hook(hook: Option<CallHook>) {
        // Never the check's own hook: calling on to it would recurse for ever.
        let hook = hook.filter(|h| *h as usize != on_call as CallHook as usize);
        *PREVIOUS_HOOK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = hook;
        HAS_PREVIOUS_HOOK.store(hook.is_some(), Ordering::Relaxed);
    }

    /// Makes the check see `thread` as the running game thread (tests).
    pub fn force_thread(thread: Option<u32>) {
        FORCED_THREAD.with(|t| t.set(thread));
    }

    fn thread(e: &Engine) -> u32 {
        FORCED_THREAD
            .with(Cell::get)
            .unwrap_or_else(|| engine_core::thread_host::current_id(e))
    }

    fn read(e: &Engine, addr: u32) -> Option<u32> {
        (addr != 0 && e.mem.is_mapped(addr) && e.mem.is_mapped(addr.wrapping_add(3)))
            .then(|| e.mem.u32(addr))
    }

    fn word(e: &Engine, addr: u32) -> u32 {
        read(e, addr).unwrap_or(0)
    }

    fn stack_text(stack: &[u32]) -> String {
        if stack.is_empty() {
            return "(top)".into();
        }
        stack
            .iter()
            .map(|a| format!("{a:08x}"))
            .collect::<Vec<_>>()
            .join(" < ")
    }

    /// A new event (also kept in the recent ring).
    fn event(e: &Engine, st: &mut State, what: What, list: u32, node: u32, keyframe: u32) -> Event {
        st.seq += 1;
        let face = st.lists.get(&list).map_or(0, |l| l.face);
        let lock = (face != 0)
            .then(|| Some((read(e, face + ANIM_LOCK)?, read(e, face + ANIM_LOCK + 4)?)))
            .flatten();
        let event = Event {
            seq: st.seq,
            what,
            list,
            node,
            keyframe,
            thread: thread(e),
            lock,
            stack: e
                .call_stack
                .iter()
                .rev()
                .take(STACK_KEEP)
                .copied()
                .collect(),
        };
        if !matches!(what, What::Call(..) | What::Pool(..)) {
            if st.recent.len() >= RECENT_KEEP {
                st.recent.pop_front();
            }
            st.recent.push_back(event.clone());
        }
        event
    }

    /// Takes the call and free hooks the first time the check runs.
    fn install(e: &mut Engine, st: &mut State) {
        if st.installed {
            return;
        }
        st.installed = true;
        st.pool_head = word(e, POOL_FREE_HEAD);
        POOL_SEEN.store(st.pool_head, Ordering::Relaxed);
        set_previous_hook(e.call_hook);
        e.call_hook = Some(on_call);
        e.free_hook = Some(on_free);
    }

    /// Runs `body` on the state when the check is on and has not stopped; stops the run at
    /// the violation it answers. Not again from inside itself (`IN_HOOK`).
    fn with_state(e: &mut Engine, body: impl FnOnce(&mut Engine, &mut State) -> Option<Violation>) {
        if !on() || IN_HOOK.swap(true, Ordering::Acquire) {
            return;
        }
        let violation = {
            let mut guard = match STATE.try_lock() {
                Ok(guard) => guard,
                Err(std::sync::TryLockError::Poisoned(p)) => p.into_inner(),
                Err(std::sync::TryLockError::WouldBlock) => {
                    IN_HOOK.store(false, Ordering::Release);
                    return;
                }
            };
            let st = guard.get_or_insert_with(State::default);
            if st.stopped {
                None
            } else {
                install(e, st);
                let violation = body(e, st);
                if st.history.len() > HISTORY_KEEP {
                    let State {
                        history,
                        slot_of,
                        deleted,
                        ..
                    } = &mut *st;
                    history.retain(|k, _| slot_of.contains_key(k) || deleted.contains_key(k));
                }
                if st.deleted.len() > HISTORY_KEEP {
                    st.deleted.clear();
                }
                if violation.is_some() {
                    st.stopped = true;
                }
                violation
            }
        };
        IN_HOOK.store(false, Ordering::Release);
        if let Some(violation) = violation {
            stop(e, violation);
        }
    }

    /// Whether the pending deleted head may still be listed now: the same thread, with its
    /// deleter still running.
    fn pending_allowed(e: &Engine, p: &Pending) -> bool {
        thread(e) == p.thread
            && (p.frame == 0
                || (e.call_stack.len() >= p.frame && e.call_stack[p.frame - 1] == p.deleter))
    }

    /// The pending deleted head: resolved once its node is gone, a violation when it is
    /// still listed and its deleter no longer runs.
    fn check_pending(e: &Engine, st: &mut State, now: &Event) -> Option<Violation> {
        let p = st.pending.clone()?;
        if st.node_of.get(&p.node) != Some(&p.list) {
            st.pending = None;
            return None;
        }
        if pending_allowed(e, &p) {
            return None;
        }
        Some(Violation {
            summary: format!(
                "keyframe {:08x} was deleted while still the head of list {:08x} (node {:08x}) \
                 and is still listed after its deleter {:08x} on thread {} stopped running it",
                p.keyframe, p.list, p.node, p.deleter, p.thread
            ),
            earlier: Some(p.event.clone()),
            now: now.clone(),
            list: p.list,
            extra: added_line(st, p.node),
        })
    }

    fn added_line(st: &State, node: u32) -> Vec<String> {
        st.added
            .get(&node)
            .map(|a| vec![format!("node {node:08x} was linked at {a}")])
            .unwrap_or_default()
    }

    /// Another thread inside a function that uses `list`.
    fn check_overlap(st: &State, list: u32, now: &Event) -> Option<Violation> {
        if !settings().overlap {
            return None;
        }
        let l = st.lists.get(&list)?;
        let (thread, inside, earlier) =
            if let Some(user) = l.users.iter().find(|u| u.0 != now.thread) {
                (user.0, user.1, user.2.clone())
            } else {
                let busy = l.busy.as_ref().filter(|b| b.thread != now.thread)?;
                (busy.thread, "RemoveHead 007b5390", busy.clone())
            };
        Some(Violation {
            summary: format!(
                "list {list:08x} used by two game threads at once: thread {thread} is inside \
                 {inside} while thread {} {}",
                now.thread, now.what
            ),
            earlier: Some(earlier),
            now: now.clone(),
            list,
            extra: Vec::new(),
        })
    }

    /// Walks `list` as memory has it now: its (node, item) slots, or what is wrong and the
    /// earlier event that names it.
    #[allow(clippy::type_complexity, clippy::result_large_err)]
    fn walk(
        e: &Engine,
        st: &State,
        list: u32,
    ) -> Result<Vec<(u32, u32)>, (String, Option<Event>, Vec<String>)> {
        let last = st.lists.get(&list).and_then(|l| l.last.clone());
        let head = word(e, list);
        let tail = word(e, list + 4);
        let count = word(e, list + 8);
        let mut slots: Vec<(u32, u32)> = Vec::new();
        let mut seen: HashMap<u32, u32> = HashMap::new();
        let mut nodes: HashSet<u32> = HashSet::new();
        let mut prev = 0u32;
        let mut node = head;
        while node != 0 {
            let index = slots.len();
            if read(e, node).is_none() || read(e, node + 8).is_none() {
                let from = if index == 0 {
                    "the head".to_string()
                } else {
                    format!("node {prev:08x}'s next")
                };
                return Err((
                    format!("list {list:08x}: {from} (link {index} of {count}) is {node:08x}, which is not mapped"),
                    last,
                    Vec::new(),
                ));
            }
            if index >= MAX_WALK || index as u64 >= u64::from(count) || !nodes.insert(node) {
                return Err((
                    format!(
                        "list {list:08x} is longer than its count {count} (link {index} is \
                         node {node:08x}; a cycle or a lost count)"
                    ),
                    last,
                    Vec::new(),
                ));
            }
            let back = word(e, node + 4);
            if back != prev {
                return Err((
                    format!(
                        "list {list:08x}: node {node:08x} (slot {index}) links back to \
                         {back:08x}, not {prev:08x}"
                    ),
                    st.added.get(&node).cloned().or(last),
                    Vec::new(),
                ));
            }
            if let Some(&other) = st.node_of.get(&node) {
                if other != list {
                    return Err((
                        format!(
                            "node {node:08x} is in two lists: {other:08x} and {list:08x} \
                             (slot {index})"
                        ),
                        st.added.get(&node).cloned(),
                        Vec::new(),
                    ));
                }
            }
            let item = word(e, node + 8);
            if item != 0 {
                if let Some(&first) = seen.get(&item) {
                    return Err((
                        format!(
                            "keyframe {item:08x} is in two slots of list {list:08x}: nodes \
                             {first:08x} and {node:08x} (slot {index})"
                        ),
                        st.added.get(&first).cloned(),
                        added_line(st, node),
                    ));
                }
                seen.insert(item, node);
                if let Some(&(other, other_node)) = st.slot_of.get(&item) {
                    if other != list {
                        return Err((
                            format!(
                                "keyframe {item:08x} is in two lists: {other:08x} (node \
                                 {other_node:08x}) and {list:08x} (node {node:08x}, slot {index})"
                            ),
                            st.added.get(&other_node).cloned(),
                            added_line(st, node),
                        ));
                    }
                }
                let pending = st.pending.as_ref().is_some_and(|p| p.keyframe == item);
                if !pending {
                    if let Some(deleted) = st.deleted.get(&item) {
                        return Err((
                            format!(
                                "keyframe {item:08x} in slot {index} of list {list:08x} (node \
                                 {node:08x}) was deleted while still listed"
                            ),
                            Some(deleted.clone()),
                            added_line(st, node),
                        ));
                    }
                    let vtable = read(e, item);
                    if vtable != Some(MULTIPLE_VTABLE) {
                        let vtable = vtable
                            .map(|v| format!("{v:08x}"))
                            .unwrap_or_else(|| "unmapped".into());
                        return Err((
                            format!(
                                "list {list:08x} slot {index} (node {node:08x}) holds {item:08x}, \
                                 which is not a live keyframe (vtable {vtable})"
                            ),
                            st.history
                                .get(&item)
                                .cloned()
                                .or_else(|| st.added.get(&node).cloned()),
                            added_line(st, node),
                        ));
                    }
                }
            }
            slots.push((node, item));
            prev = node;
            node = word(e, node);
        }
        if slots.len() as u64 != u64::from(count) {
            return Err((
                format!(
                    "list {list:08x} has {} nodes but its count is {count}",
                    slots.len()
                ),
                last,
                Vec::new(),
            ));
        }
        if tail != prev {
            return Err((
                format!("list {list:08x}: tail is {tail:08x} but the last node is {prev:08x}"),
                last,
                Vec::new(),
            ));
        }
        Ok(slots)
    }

    /// Keeps `slots` as `list`'s picture, `now` as its last clean event.
    fn commit(st: &mut State, list: u32, slots: Vec<(u32, u32)>, now: &Event) {
        let Some(l) = st.lists.get_mut(&list) else {
            return;
        };
        let old = std::mem::replace(&mut l.slots, slots);
        l.last = Some(now.clone());
        for (node, item) in old {
            if st.node_of.get(&node) == Some(&list) {
                st.node_of.remove(&node);
            }
            if st.slot_of.get(&item).is_some_and(|s| s.0 == list) {
                st.slot_of.remove(&item);
            }
        }
        let l = &st.lists[&list];
        for &(node, item) in &l.slots {
            st.node_of.insert(node, list);
            if item != 0 {
                st.slot_of.insert(item, (list, node));
            }
        }
    }

    /// Walks `list` and keeps it, or the violation.
    fn walk_and_commit(e: &Engine, st: &mut State, list: u32, now: &Event) -> Option<Violation> {
        if !matches!(st.lists.get(&list), Some(l) if l.busy.is_none()) {
            return None;
        }
        match walk(e, st, list) {
            Ok(slots) => {
                commit(st, list, slots, now);
                None
            }
            Err((summary, earlier, extra)) => Some(Violation {
                summary,
                earlier,
                now: now.clone(),
                list,
                extra,
            }),
        }
    }

    fn track(e: &Engine, st: &mut State, list: u32, face: u32) -> Option<Violation> {
        if list == 0 {
            return None;
        }
        if let Some(l) = st.lists.get_mut(&list) {
            if l.face == 0 {
                l.face = face;
            }
            return None;
        }
        st.lists.insert(
            list,
            List {
                face,
                ..List::default()
            },
        );
        let now = event(e, st, What::Track, list, 0, 0);
        walk_and_commit(e, st, list, &now)
    }

    /// Watches the face's three speech lists (from `Update`, the adders and the copy).
    pub fn track_face(e: &mut Engine, face: u32) {
        if face == 0 {
            return;
        }
        with_state(e, |e, st| {
            FACE_LISTS
                .iter()
                .find_map(|offset| track(e, st, face + offset, face))
        });
    }

    /// Watches `list` (from `AddToList`).
    pub fn track_list(e: &mut Engine, list: u32) {
        with_state(e, |e, st| track(e, st, list, 0));
    }

    /// The face's lists checked at a named point (`Update`'s start).
    pub fn at(e: &mut Engine, face: u32, name: &'static str) {
        with_state(e, |e, st| {
            for offset in FACE_LISTS {
                let list = face + offset;
                if let Some(v) = track(e, st, list, face) {
                    return Some(v);
                }
                let now = event(e, st, What::At(name), list, 0, 0);
                if let Some(v) = check_pending(e, st, &now) {
                    return Some(v);
                }
                if let Some(v) = walk_and_commit(e, st, list, &now) {
                    return Some(v);
                }
            }
            None
        });
    }

    /// The running thread enters `name`, which uses `list`.
    pub fn enter(e: &mut Engine, list: u32, name: &'static str) {
        with_state(e, |e, st| {
            if !st.lists.contains_key(&list) {
                return None;
            }
            let now = event(e, st, What::Enter(name), list, 0, 0);
            if let Some(v) = check_overlap(st, list, &now) {
                return Some(v);
            }
            if let Some(l) = st.lists.get_mut(&list) {
                l.users.push((now.thread, name, now.clone()));
            }
            None
        });
    }

    /// The running thread leaves `name`.
    pub fn leave(e: &mut Engine, list: u32, name: &'static str) {
        with_state(e, |e, st| {
            if !st.lists.contains_key(&list) {
                return None;
            }
            let now = event(e, st, What::Leave(name), list, 0, 0);
            if let Some(l) = st.lists.get_mut(&list) {
                if let Some(i) = l
                    .users
                    .iter()
                    .rposition(|(t, n, _)| *t == now.thread && *n == name)
                {
                    l.users.remove(i);
                }
            }
            check_pending(e, st, &now).or_else(|| walk_and_commit(e, st, list, &now))
        });
    }

    /// `node` was linked at the tail of `list` (`00559a70`'s end).
    pub fn linked(e: &mut Engine, list: u32, node: u32) {
        with_state(e, |e, st| {
            if !st.lists.contains_key(&list) {
                return None;
            }
            let item = word(e, node + 8);
            let now = event(e, st, What::Add, list, node, item);
            if let Some(v) = check_overlap(st, list, &now) {
                return Some(v);
            }
            if let Some(v) = check_pending(e, st, &now) {
                return Some(v);
            }
            if let Some(&other) = st.node_of.get(&node) {
                return Some(Violation {
                    summary: format!(
                        "node {node:08x} linked into list {list:08x} while still linked in \
                         list {other:08x}"
                    ),
                    earlier: st.added.get(&node).cloned(),
                    now,
                    list,
                    extra: Vec::new(),
                });
            }
            if item != 0 {
                if let Some(&(other, other_node)) = st.slot_of.get(&item) {
                    // A stale slot whose keyframe's block came back from the heap as this one.
                    let extra = st
                        .deleted
                        .get(&item)
                        .map(|d| vec![format!("the block {item:08x} was deleted at {d}")])
                        .unwrap_or_default();
                    return Some(Violation {
                        summary: format!(
                            "keyframe {item:08x} added to list {list:08x} (node {node:08x}) while \
                             already in list {other:08x} (node {other_node:08x})"
                        ),
                        earlier: st.added.get(&other_node).cloned(),
                        now,
                        list,
                        extra,
                    });
                }
                // A new keyframe in a block a deleted one had (the heap reuses blocks).
                st.deleted.remove(&item);
            }
            let result = walk_and_commit(e, st, list, &now);
            st.added.insert(node, now.clone());
            if item != 0 {
                st.history.insert(item, now);
            }
            result
        });
    }

    /// `RemoveHead` (`007b5390`) starts on `list`.
    pub fn removing(e: &mut Engine, list: u32) {
        with_state(e, |e, st| {
            if !st.lists.contains_key(&list) {
                return None;
            }
            let node = word(e, list);
            let item = word(e, node + 8);
            let now = event(e, st, What::Remove, list, node, item);
            if let Some(v) = check_overlap(st, list, &now) {
                return Some(v);
            }
            if let Some(v) = walk_and_commit(e, st, list, &now) {
                return Some(v);
            }
            if let Some(l) = st.lists.get_mut(&list) {
                l.busy = Some(now.clone());
                if l.slots.first().is_some_and(|s| s.0 == node) {
                    l.slots.remove(0);
                }
            }
            if st.node_of.get(&node) == Some(&list) {
                st.node_of.remove(&node);
            }
            if st.slot_of.get(&item) == Some(&(list, node)) {
                st.slot_of.remove(&item);
            }
            if st.pending.as_ref().is_some_and(|p| p.node == node) {
                st.pending = None;
            }
            if item != 0 {
                st.history.insert(item, now);
            }
            None
        });
    }

    /// `RemoveHead` (`007b5390`) ends on `list`.
    pub fn removed(e: &mut Engine, list: u32) {
        with_state(e, |e, st| {
            let l = st.lists.get_mut(&list)?;
            let start = l.busy.take()?;
            let now = event(
                e,
                st,
                What::At("RemoveHead's end"),
                list,
                start.node,
                start.keyframe,
            );
            walk_and_commit(e, st, list, &now)
        });
    }

    /// `0064ebe0` starts on `keyframe` with `flags` (bit 0: the block is freed).
    pub fn deleting(e: &mut Engine, keyframe: u32, flags: u32) {
        if flags & 1 == 0 {
            return;
        }
        with_state(e, |e, st| {
            let slot = st.slot_of.get(&keyframe).copied();
            let (list, node) = slot.unwrap_or((0, 0));
            let now = event(e, st, What::Delete, list, node, keyframe);
            let violation = slot.and_then(|slot| deleted_while_listed(e, st, keyframe, slot, &now));
            st.deleted.insert(keyframe, now.clone());
            st.history.insert(keyframe, now);
            violation
        });
    }

    /// A listed keyframe being deleted: its list's head becomes the pending deleted head;
    /// any other slot is a violation.
    fn deleted_while_listed(
        e: &Engine,
        st: &mut State,
        keyframe: u32,
        (list, node): (u32, u32),
        now: &Event,
    ) -> Option<Violation> {
        if let Some(v) = check_overlap(st, list, now) {
            return Some(v);
        }
        let index = st.lists[&list].slots.iter().position(|s| s.0 == node);
        if index != Some(0) {
            return Some(Violation {
                summary: format!(
                    "keyframe {keyframe:08x} deleted while still in list {list:08x} at \
                             slot {} (node {node:08x}), not its head",
                    index.map_or("?".into(), |i| i.to_string())
                ),
                earlier: st.added.get(&node).cloned(),
                now: now.clone(),
                list,
                extra: Vec::new(),
            });
        }
        if let Some(p) = st.pending.clone() {
            if p.keyframe != keyframe && st.node_of.get(&p.node) == Some(&p.list) {
                return Some(Violation {
                    summary: format!(
                        "keyframe {keyframe:08x} (head of list {list:08x}) deleted while \
                                 the deleted head {:08x} of list {:08x} is still listed",
                        p.keyframe, p.list
                    ),
                    earlier: Some(p.event),
                    now: now.clone(),
                    list,
                    extra: Vec::new(),
                });
            }
        }
        let stack = &e.call_stack;
        let (frame, deleter) = match stack.split_last() {
            Some((&DELETING_DESTRUCTOR, rest)) => (rest.len(), rest.last().copied().unwrap_or(0)),
            Some((&last, _)) => (stack.len(), last),
            None => (0, 0),
        };
        st.pending = Some(Pending {
            keyframe,
            list,
            node,
            thread: now.thread,
            frame,
            deleter,
            event: now.clone(),
        });
        None
    }

    /// `Engine::free_hook`: a heap block is being freed.
    fn on_free(e: &mut Engine, block: u32) {
        with_state(e, |e, st| {
            let known = st.history.contains_key(&block);
            let slot = st.slot_of.get(&block).copied();
            // Lists inside the block are gone with it.
            if !st.lists.is_empty() {
                let size = e.mem.block_size(block).unwrap_or(0);
                let inside: Vec<u32> = st
                    .lists
                    .keys()
                    .copied()
                    .filter(|l| l.wrapping_sub(block) < size)
                    .collect();
                for list in inside {
                    forget(st, list);
                }
            }
            if !known && slot.is_none() {
                return None;
            }
            let (list, node) = slot.unwrap_or((0, 0));
            let now = event(e, st, What::Free, list, node, block);
            let violation = slot.and_then(|(list, node)| {
                let allowed = st
                    .pending
                    .as_ref()
                    .is_some_and(|p| p.keyframe == block && pending_allowed(e, p));
                (!allowed).then(|| Violation {
                    summary: format!(
                        "keyframe {block:08x} freed while still in list {list:08x} (node \
                         {node:08x})"
                    ),
                    earlier: st.added.get(&node).cloned(),
                    now: now.clone(),
                    list,
                    extra: st
                        .deleted
                        .get(&block)
                        .map(|d| vec![format!("it was deleted at {d}")])
                        .unwrap_or_default(),
                })
            });
            if slot.is_none() {
                st.deleted.entry(block).or_insert_with(|| now.clone());
            }
            st.history.insert(block, now);
            violation
        });
    }

    fn forget(st: &mut State, list: u32) {
        if let Some(l) = st.lists.remove(&list) {
            for (node, item) in l.slots {
                if st.node_of.get(&node) == Some(&list) {
                    st.node_of.remove(&node);
                }
                if st.slot_of.get(&item).is_some_and(|s| s.0 == list) {
                    st.slot_of.remove(&item);
                }
            }
        }
        if st.pending.as_ref().is_some_and(|p| p.list == list) {
            st.pending = None;
        }
    }

    /// The face is being destroyed (after its lists were cleared): its lists are no longer
    /// watched.
    pub fn forget_face(e: &mut Engine, face: u32) {
        with_state(e, |_, st| {
            for offset in FACE_LISTS {
                forget(st, face + offset);
            }
            None
        });
    }

    fn linked_node_violation(st: &State, node: u32, now: Event, how: &str) -> Option<Violation> {
        let list = *st.node_of.get(&node)?;
        Some(Violation {
            summary: format!("node {node:08x} {how} while still linked in list {list:08x}"),
            earlier: st.added.get(&node).cloned(),
            now,
            list,
            extra: Vec::new(),
        })
    }

    /// `0045cee0` starts: `node` goes back to the pool.
    pub fn node_freeing(e: &mut Engine, node: u32) {
        with_state(e, |e, st| {
            if !st.node_of.contains_key(&node) {
                return None;
            }
            let list = st.node_of[&node];
            let item = word(e, node + 8);
            let now = event(e, st, What::NodeFree, list, node, item);
            linked_node_violation(st, node, now, "freed to the node pool (0045cee0)")
        });
    }

    /// `0043a010` ends: `node` came from the pool.
    pub fn node_allocated(e: &mut Engine, node: u32) {
        with_state(e, |e, st| {
            if !st.node_of.contains_key(&node) {
                return None;
            }
            let list = st.node_of[&node];
            let now = event(e, st, What::NodeAlloc, list, node, 0);
            linked_node_violation(st, node, now, "handed out by the node pool (0043a010)")
        });
    }

    /// `Engine::call_hook`: the pool's free list (every call) and, with `calls`, every list.
    fn on_call(e: &mut Engine, addr: u32, returning: bool) {
        if let Some(previous) = previous_hook() {
            previous(e, addr, returning);
        }
        // The usual call: nothing to do unless the pool's head moved (kept cheap, it runs at
        // every call).
        if !settings().calls {
            let head = word(e, POOL_FREE_HEAD);
            if POOL_SEEN.swap(head, Ordering::Relaxed) == head {
                return;
            }
        }
        with_state(e, |e, st| {
            let head = word(e, POOL_FREE_HEAD);
            if head != st.pool_head {
                let old = st.pool_head;
                st.pool_head = head;
                if !st.node_of.is_empty() {
                    let mut node = head;
                    for _ in 0..POOL_LOOK {
                        if node == 0 || node == old {
                            break;
                        }
                        if st.node_of.contains_key(&node) {
                            let list = st.node_of[&node];
                            let now = event(e, st, What::Pool(addr, returning), list, node, 0);
                            return linked_node_violation(
                                st,
                                node,
                                now,
                                "is on the node pool's free list (011c5f58)",
                            );
                        }
                        let Some(next) = read(e, node) else {
                            break;
                        };
                        node = next;
                    }
                }
            }
            if settings().calls && !st.lists.is_empty() {
                let lists: Vec<u32> = st.lists.keys().copied().collect();
                let now = event(e, st, What::Call(addr, returning), 0, 0, 0);
                if let Some(v) = check_pending(e, st, &now) {
                    return Some(v);
                }
                for list in lists {
                    let mut now = now.clone();
                    now.list = list;
                    if let Some(v) = walk_and_commit(e, st, list, &now) {
                        return Some(v);
                    }
                }
            }
            None
        });
    }

    /// Prints what was found and stops the run.
    fn stop(e: &mut Engine, v: Violation) -> ! {
        // The check must not run again while the run unwinds.
        e.call_hook = previous_hook();
        e.free_hook = None;
        let mut out = String::new();
        let _ = writeln!(out, "speech: {}", v.summary);
        match &v.earlier {
            Some(earlier) => {
                let _ = writeln!(out, "speech: earlier: {earlier}");
            }
            None => {
                let _ = writeln!(out, "speech: earlier: (no event recorded for it)");
            }
        }
        let _ = writeln!(out, "speech: now: {}", v.now);
        for line in &v.extra {
            let _ = writeln!(out, "speech: {line}");
        }
        if v.list != 0 && read(e, v.list).is_some() {
            let (head, tail, count) = (word(e, v.list), word(e, v.list + 4), word(e, v.list + 8));
            let _ = writeln!(
                out,
                "speech: list {:08x} in memory: head {head:08x} tail {tail:08x} count {count}",
                v.list
            );
            let mut node = head;
            for index in 0..DUMP_SLOTS {
                if node == 0 {
                    break;
                }
                let Some(next) = read(e, node) else {
                    let _ = writeln!(out, "speech:   [{index}] node {node:08x} unmapped");
                    break;
                };
                let item = word(e, node + 8);
                let vtable = read(e, item)
                    .map(|w| format!("{w:08x}"))
                    .unwrap_or_else(|| "unmapped".into());
                let _ = writeln!(
                    out,
                    "speech:   [{index}] node {node:08x} next {next:08x} prev {:08x} item \
                     {item:08x} (vtable {vtable})",
                    word(e, node + 4)
                );
                node = next;
            }
        }
        {
            let guard = lock_state();
            if let Some(st) = guard.as_ref() {
                if let Some(l) = st.lists.get(&v.list) {
                    let _ = writeln!(
                        out,
                        "speech: list {:08x} at its last clean event ({}): {}",
                        v.list,
                        l.last
                            .as_ref()
                            .map_or("none".into(), |e| format!("#{}", e.seq)),
                        l.slots
                            .iter()
                            .take(DUMP_SLOTS)
                            .map(|(n, i)| format!("{n:08x}:{i:08x}"))
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }
                let _ = writeln!(out, "speech: recent events, oldest first:");
                for event in &st.recent {
                    let _ = writeln!(out, "speech:   {event}");
                }
            }
        }
        print!("{out}");
        let earlier = v
            .earlier
            .as_ref()
            .map_or("none recorded".into(), Event::short);
        panic!(
            "speech check: {}; earlier: {earlier}; now: {} (NV_SPEECH_CHECK; more on the \
             speech: lines)",
            v.summary,
            v.now.short()
        );
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const VTABLE_PAGE: u32 = 0x0106_5000;
        const LIST_COUNT: u32 = 0x0044_ddc0;
        const ADD_TAIL: u32 = 0x004e_d8c0;
        const REMOVE_HEAD: u32 = 0x007b_5390;
        const FREE_NODE: u32 = 0x0045_cee0;
        const CLEAR_SPEECH_KEYFRAME_LISTS: u32 = 0x0064_aec0;
        const EVENTS: Settings = Settings {
            on: true,
            calls: false,
            overlap: true,
        };
        const CALLS: Settings = Settings {
            on: true,
            calls: true,
            overlap: true,
        };

        /// The check's state is the process's: its tests run one at a time.
        fn serial() -> std::sync::MutexGuard<'static, ()> {
            static SERIAL: Mutex<()> = Mutex::new(());
            SERIAL
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
        }

        fn stop_message(run: impl FnOnce()) -> String {
            let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(run))
                .expect_err("the check should have stopped the run");
            stop.downcast_ref::<String>().cloned().unwrap_or_default()
        }

        /// Every translated function, spin locks that do nothing, `operator delete` through
        /// the platform's `Deallocate`, the keyframe vtable (slot 0: the scalar deleting
        /// destructor), a pool of 64 free nodes and a face (0x1c0 zeroed bytes: empty lists),
        /// watched.
        fn engine(settings: Settings) -> (Engine, u32) {
            force(settings);
            let mut e = Engine::new();
            let mut funcs = Vec::new();
            engine_core::units::funcs(&mut funcs);
            crate::units::funcs(&mut funcs);
            for (addr, f) in funcs {
                e.register(addr, f);
            }
            e.register(0x0040_fbf0, |_, _| Ret::default());
            e.register(0x0040_fba0, |_, _| Ret::default());
            e.register(0x0040_1030, |e, a| {
                engine_core::units::platform::memory_manager_deallocate(
                    e,
                    Ptr::new(0),
                    Ptr::new(a[0]),
                );
                Ret::default()
            });
            e.map(0x011c_5000, 0x1000);
            e.map(VTABLE_PAGE, 0x1000);
            e.mem.set_u32(MULTIPLE_VTABLE, DELETING_DESTRUCTOR);
            let pool = e.mem.alloc(12 * 64);
            for i in 0..64u32 {
                let next = if i == 63 { 0 } else { pool + 12 * (i + 1) };
                e.mem.set_u32(pool + 12 * i, next);
            }
            e.mem.set_u32(POOL_FREE_HEAD, pool);
            let face = e.mem.alloc(0x1c0);
            track_face(&mut e, face);
            (e, face)
        }

        fn keyframe(e: &mut Engine) -> u32 {
            let keyframe = e.mem.alloc(0x14);
            e.mem.set_u32(keyframe, MULTIPLE_VTABLE);
            keyframe
        }

        /// `keyframe` added at the tail of `list` (`004ed8c0`, as `AddToList` does).
        fn add(e: &mut Engine, list: u32, keyframe: u32) {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), keyframe);
                e.call(ADD_TAIL, &args![list, slot]);
            });
        }

        fn filled(e: &mut Engine, list: u32, n: usize) -> Vec<u32> {
            let keyframes: Vec<u32> = (0..n).map(|_| keyframe(e)).collect();
            for &k in &keyframes {
                add(e, list, k);
            }
            keyframes
        }

        #[test]
        fn nv_speech_check_reads_its_flags() {
            assert_eq!(Settings::parse(None), Settings::OFF);
            assert_eq!(Settings::parse(Some("0")), Settings::OFF);
            assert_eq!(Settings::parse(Some("1")), EVENTS);
            assert_eq!(Settings::parse(Some("calls")), CALLS);
            let quiet = Settings::parse(Some("calls,nooverlap"));
            assert!(quiet.on && quiet.calls && !quiet.overlap);
        }

        #[test]
        fn removing_then_deleting_and_the_clears_order_pass_the_check() {
            let _serial = serial();
            for settings in [EVENTS, CALLS] {
                let (mut e, face) = engine(settings);
                let (modifiers, phonemes) = (face + 0x88, face + 0xbc);
                let keyframes = filled(&mut e, modifiers, 4);
                filled(&mut e, phonemes, 2);
                // `UpdateKeyframeList`'s order: the head removed, then its keyframe deleted.
                assert_eq!(e.call(REMOVE_HEAD, &args![modifiers]).u32(), keyframes[0]);
                e.call(DELETING_DESTRUCTOR, &args![keyframes[0], 1u32]);
                // `ClearSpeechKeyframeLists`' order: the head's keyframe deleted, then the head
                // removed, while it runs.
                e.call(
                    CLEAR_SPEECH_KEYFRAME_LISTS,
                    &args![face, 0u32, 1u32, 1u32, 0u32],
                );
                assert_eq!(e.mem.u32(modifiers + 8), 0);
                assert_eq!(e.mem.u32(phonemes + 8), 0);
                at(&mut e, face, "the test's end");
                // The nodes are back on the pool and can be used again.
                filled(&mut e, modifiers, 3);
                e.call(LIST_COUNT, &args![modifiers]);
            }
        }

        #[test]
        fn a_keyframe_deleted_while_listed_but_not_the_head_stops_the_run() {
            let _serial = serial();
            let (mut e, face) = engine(EVENTS);
            let keyframes = filled(&mut e, face + 0x88, 2);
            let message = stop_message(|| {
                e.call(DELETING_DESTRUCTOR, &args![keyframes[1], 1u32]);
            });
            assert!(
                message.contains(&format!(
                    "keyframe {:08x} deleted while still in list",
                    keyframes[1]
                )),
                "{message}"
            );
            assert!(message.contains("earlier: #"), "{message}");
            assert!(message.contains("added (00559a70)"), "{message}");
            assert!(message.contains("now: #"), "{message}");
            assert!(message.contains("keyframe deleted (0064ebe0)"), "{message}");
        }

        #[test]
        fn a_listed_keyframe_freed_another_way_stops_the_run() {
            let _serial = serial();
            let (mut e, face) = engine(EVENTS);
            let keyframes = filled(&mut e, face + 0xbc, 2);
            let message = stop_message(|| {
                engine_core::units::platform::memory_manager_deallocate(
                    &mut e,
                    Ptr::new(0),
                    Ptr::new(keyframes[0]),
                );
            });
            assert!(
                message.contains(&format!(
                    "keyframe {:08x} freed while still in list",
                    keyframes[0]
                )),
                "{message}"
            );
            assert!(message.contains("block freed"), "{message}");
        }

        #[test]
        fn a_linked_node_freed_to_the_pool_stops_the_run() {
            let _serial = serial();
            let (mut e, face) = engine(EVENTS);
            let list = face + 0x88;
            filled(&mut e, list, 1);
            let node = e.mem.u32(list);
            let message = stop_message(|| {
                e.call(FREE_NODE, &args![0u32, node]);
            });
            assert!(
                message.contains(&format!("node {node:08x} freed to the node pool")),
                "{message}"
            );
            assert!(message.contains("added (00559a70)"), "{message}");
        }

        #[test]
        fn a_linked_node_pushed_on_the_pool_elsewhere_stops_at_the_next_call() {
            let _serial = serial();
            let (mut e, face) = engine(EVENTS);
            let list = face + 0xf0;
            filled(&mut e, list, 2);
            // A push of the second node written in place, as inlined pool code does.
            let node = e.mem.u32(e.mem.u32(list));
            let head = e.mem.u32(POOL_FREE_HEAD);
            e.mem.set_u32(node, head);
            e.mem.set_u32(POOL_FREE_HEAD, node);
            let message = stop_message(|| {
                e.call(LIST_COUNT, &args![list]);
            });
            assert!(
                message.contains(&format!("node {node:08x} is on the node pool's free list")),
                "{message}"
            );
            assert!(message.contains("entry to 0044ddc0"), "{message}");
        }

        #[test]
        fn the_same_keyframe_added_twice_stops_the_run() {
            let _serial = serial();
            let (mut e, face) = engine(EVENTS);
            let keyframes = filled(&mut e, face + 0x88, 1);
            let message = stop_message(|| add(&mut e, face + 0xbc, keyframes[0]));
            assert!(message.contains("while already in list"), "{message}");
        }

        #[test]
        fn a_count_that_does_not_match_the_links_stops_the_run() {
            let _serial = serial();
            let (mut e, face) = engine(EVENTS);
            let list = face + 0x88;
            filled(&mut e, list, 1);
            e.mem.set_u32(list + 8, 2);
            let message = stop_message(|| {
                filled(&mut e, list, 1);
            });
            assert!(
                message.contains("has 2 nodes but its count is 3"),
                "{message}"
            );
        }

        #[test]
        fn a_deleted_head_still_listed_when_another_thread_runs_stops_the_run() {
            let _serial = serial();
            let (mut e, face) = engine(EVENTS);
            let list = face + 0x88;
            force_thread(Some(1));
            let keyframes = filled(&mut e, list, 2);
            // Deleted as `ClearSpeechKeyframeLists` does, without its RemoveHead yet.
            e.call(DELETING_DESTRUCTOR, &args![keyframes[0], 1u32]);
            force_thread(Some(7));
            let message = stop_message(|| {
                filled(&mut e, list, 1);
            });
            assert!(
                message.contains(&format!(
                    "keyframe {:08x} was deleted while still the head",
                    keyframes[0]
                )),
                "{message}"
            );
            assert!(message.contains("on thread 1"), "{message}");
            assert!(message.contains("on thread 7"), "{message}");
        }

        #[test]
        fn two_game_threads_in_one_list_stop_the_run() {
            let _serial = serial();
            let (mut e, face) = engine(EVENTS);
            let list = face + 0x88;
            force_thread(Some(1));
            filled(&mut e, list, 1);
            enter(&mut e, list, "UpdateKeyframeList 0064ca60");
            force_thread(Some(2));
            let message = stop_message(|| {
                filled(&mut e, list, 1);
            });
            assert!(
                message.contains("used by two game threads at once"),
                "{message}"
            );
            assert!(message.contains("UpdateKeyframeList 0064ca60"), "{message}");
        }

        /// p190: a second game thread (its own OS thread) found the check's own hook in
        /// `Engine::call_hook` and took it as the hook to call on, so each call recursed for
        /// ever. The check now never calls on to itself.
        #[test]
        fn a_second_install_never_calls_the_check_on_from_itself() {
            let _serial = serial();
            let (mut e, face) = engine(EVENTS);
            assert!(e.call_hook.is_some());
            // As a fresh state (another thread's, before the fix) would install again.
            *lock_state() = None;
            track_face(&mut e, face);
            assert!(previous_hook().is_none());
            filled(&mut e, face + 0x88, 2);
            assert_eq!(e.call(LIST_COUNT, &args![face + 0x88]).u32(), 2);
        }

        static FREED_BY_HOOK: AtomicU32 = AtomicU32::new(0);

        /// A call hook that was there before the check: at the entry to `0044ddc0` it frees a
        /// block (the free hook runs inside the call hook) and makes a call of its own (the
        /// call hook runs inside itself).
        fn earlier_hook(e: &mut Engine, addr: u32, returning: bool) {
            if addr == LIST_COUNT && !returning {
                let block = FREED_BY_HOOK.swap(0, Ordering::Relaxed);
                if block != 0 {
                    engine_core::units::platform::memory_manager_deallocate(
                        e,
                        Ptr::new(0),
                        Ptr::new(block),
                    );
                    e.call(0x0073_ac90, &args![block]);
                }
            }
        }

        #[test]
        fn a_free_and_a_call_inside_the_hooks_come_back() {
            let _serial = serial();
            force(EVENTS);
            let (mut e, face) = {
                let mut e = Engine::new();
                e.call_hook = Some(earlier_hook);
                let (mut checked, face) = engine(EVENTS);
                checked.call_hook = e.call_hook.take();
                checked.free_hook = None;
                // Installed afresh over the earlier hook.
                *lock_state() = None;
                track_face(&mut checked, face);
                (checked, face)
            };
            assert!(previous_hook().is_some());
            let block = e.mem.alloc(0x20);
            FREED_BY_HOOK.store(block, Ordering::Relaxed);
            filled(&mut e, face + 0xbc, 1);
            assert_eq!(e.call(LIST_COUNT, &args![face + 0xbc]).u32(), 1);
            assert_eq!(FREED_BY_HOOK.load(Ordering::Relaxed), 0);
            // A hook reached from inside the check returns at once.
            with_state(&mut e, |e, _| {
                on_free(e, block);
                on_call(e, LIST_COUNT, false);
                deleting(e, 0x1234_5678, 1);
                None
            });
            assert!(!IN_HOOK.load(Ordering::Relaxed));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    const GET_TYPE: u32 = 0x0340_0004;
    const IS_MULTIPLE: u32 = 0x0340_0044;

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The keyframe vtable for the tests: this unit's translations in the slots it
    /// has, doubles for the type (the word at +4) and `IsMultiple` (the word at +0x14).
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x60);
        for (slot, target) in [
            (SLOT_GET_TYPE, GET_TYPE),
            (SLOT_SET_TYPE, 0x0064_eb00),
            (SLOT_GET_TIME, 0x0048_8d50),
            (SLOT_SET_TIME, 0x0064_eb30),
            (SLOT_COMPARE, 0x0064_f690),
            (SLOT_GET_STRENGTH, 0x0064_f760),
            (SLOT_IS_MULTIPLE, IS_MULTIPLE),
            (SLOT_IS_VALID_QUANTITY, 0x0064_fb90),
            (SLOT_RESET_QUANTITY_ARRAY, 0x0064_fc00),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        e.register(GET_TYPE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(IS_MULTIPLE, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(QUANTITY_COUNT_0044EDB0, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(QUANTITY_005DD4E0, |e, a| Ret {
            st0: f64::from(e.mem.f32(e.mem.u32(a[0] + 0xc) + 4 * a[1])),
            ..Ret::default()
        });
        e.register(SET_QUANTITY_0064A4D0, |e, a| {
            let slot = e.mem.u32(a[0] + 0xc) + 4 * a[1];
            let changed = e.mem.u32(slot) != a[2];
            e.mem.set_u32(slot, a[2]);
            u32::from(changed).into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        answer(&mut e, OPERATOR_DELETE, 0);
        answer(&mut e, MANAGER_RESET_00653DC0, 0);
        for (addr, value) in [(ZERO, 0.0f64), (ONE, 1.0), (MAXIMUM, f64::from(f32::MAX))] {
            e.map(addr, 8);
            e.mem.set_f64(addr, value);
        }
        e.map(TOLERANCE, 4);
        e.mem.set_f32(TOLERANCE, 0.001);
        e.map(MAXIMUM_FLOAT, 4);
        e.mem.set_f32(MAXIMUM_FLOAT, f32::MAX);
        e.call_log = Some(vec![]);
        (e, vtable)
    }

    /// A multiple keyframe of `kind` with `values` (0x18 bytes: the multiple flag at +0x14).
    fn keyframe(
        e: &mut Engine,
        vtable: u32,
        kind: u32,
        values: &[f32],
    ) -> Ptr<BSFaceGenKeyframeMultiple> {
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 4, kind);
        let array = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(array + 4 * i as u32, *v);
        }
        e.mem.set_u32(this + 0xc, array);
        e.mem.set_u32(this + 0x10, values.len() as u32);
        e.mem.set_u32(this + 0x14, 1);
        Ptr::new(this)
    }

    fn values(e: &Engine, this: Ptr<BSFaceGenKeyframeMultiple>) -> Vec<f32> {
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        let count = e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities);
        (0..count).map(|i| e.mem.f32(array + 4 * i)).collect()
    }

    #[test]
    fn keyframe_base_type_and_time() {
        let (mut e, _) = engine();
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0064_ea80, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), KEYFRAME_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert_eq!(e.mem.f32(this + 8), 0.0);
        assert!(!e.call(0x0064_eb00, &args![this, 3u32]).bool());
        assert!(e.call(0x0064_eb00, &args![this, 4u32]).bool());
        assert_eq!(e.mem.u32(this + 4), 4);
        assert!(!e.call(0x0064_eb30, &args![this, 0.0f32]).bool());
        assert!(e.call(0x0064_eb30, &args![this, 2.5f32]).bool());
        assert_eq!(e.call(0x0048_8d50, &args![this]).f64(), 2.5);
        // A NaN never equals: always set, quiet.
        assert!(e.call(0x0064_eb30, &args![this, 0x7f80_0001u32]).bool());
        assert_eq!(e.mem.u32(this + 8), 0x7fc0_0001);
        assert!(e.call(0x0064_eb30, &args![this, 0x7f80_0001u32]).bool());
        e.mem.set_u32(this, 0);
        e.call(0x0064_eae0, &args![this]);
        assert_eq!(e.mem.u32(this), KEYFRAME_VTABLE);
    }

    #[test]
    fn multiple_construct_reset_and_destroy() {
        let (mut e, _) = engine();
        let this = Ptr::<BSFaceGenKeyframeMultiple>::new(e.mem.alloc(0x14));
        e.call(0x0064_eb60, &args![this, 2u32, 3u32]);
        assert_eq!(e.mem.u32(this.addr()), MULTIPLE_VTABLE);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::eType), 2);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities), 3);
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![12]]);
        assert_eq!(
            calls(&e, MANAGER_RESET_00653DC0),
            vec![vec![array, 3, 1], vec![array, 3, 1]]
        );
        // The same count only refills.
        e.call_log = Some(vec![]);
        e.call(0x0064_fc00, &args![this, 3u32, 0u32]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert_eq!(calls(&e, MANAGER_RESET_00653DC0), vec![vec![array, 3, 0]]);
        // A huge count asks for all ones.
        e.call_log = Some(vec![]);
        answer(&mut e, OPERATOR_NEW, 0x7000);
        e.call(0x0064_fc00, &args![this, 0x4000_0001u32, 0u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![array]]);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![u32::MAX]]);
        // Destroying frees the array; the deleting destructor frees the object on bit 0.
        e.call_log = Some(vec![]);
        e.call(0x0064_ebe0, &args![this, 1u32]);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![0x7000], vec![this.addr()]]
        );
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities), 0);
        assert_eq!(e.mem.u32(this.addr()), KEYFRAME_VTABLE);
    }

    #[test]
    fn copy_constructor_copies_type_time_and_quantities() {
        let (mut e, vtable) = engine();
        let other = keyframe(&mut e, vtable, 5, &[0.25, 0.5]);
        e.set(other, BSFaceGenKeyframeMultiple::fTime, 1.5);
        let this = Ptr::<BSFaceGenKeyframeMultiple>::new(e.mem.alloc(0x14));
        e.call(0x0064_ec10, &args![this, other]);
        assert_eq!(e.mem.u32(this.addr()), MULTIPLE_VTABLE);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::eType), 5);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::fTime), 1.5);
        assert_eq!(values(&e, this), vec![0.25, 0.5]);
    }

    #[test]
    fn add_to_list_copies_matching_keyframes() {
        let (mut e, vtable) = engine();
        let this = keyframe(&mut e, vtable, 5, &[0.25]);
        answer(&mut e, LIST_ADD_004ED8C0, 0);
        answer(&mut e, LIST_0044DDC0, 0);
        assert!(!e.call(0x0064_ed60, &args![this, 0u32]).bool());
        e.set(this, BSFaceGenKeyframeMultiple::fTime, -1.0);
        assert!(!e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
        e.set(this, BSFaceGenKeyframeMultiple::fTime, 0.0);
        e.register(LIST_ADD_004ED8C0, |e, a| {
            let copy = e.mem.u32(a[1]);
            assert_eq!(e.mem.u32(copy), MULTIPLE_VTABLE);
            assert_eq!(e.mem.u32(copy + 4), 5);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
        assert_eq!(calls(&e, OPERATOR_NEW)[0], vec![0x14]);
        assert_eq!(calls(&e, LIST_ADD_004ED8C0).len(), 1);
        // A non-empty list: only behind a multiple keyframe of the same type.
        answer(&mut e, LIST_0044DDC0, 1);
        let last = keyframe(&mut e, vtable, 6, &[]);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, last.addr());
        answer(&mut e, LIST_ITEM_007B5370, slot);
        assert!(!e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
        e.mem.set_u32(last.addr() + 4, 5);
        assert!(e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
        e.mem.set_u32(last.addr() + 0x14, 0);
        assert!(!e.call(0x0064_ed60, &args![this, 0x7000u32]).bool());
    }

    #[test]
    fn merge_blends_by_the_clamped_weight() {
        let (mut e, vtable) = engine();
        e.register(NEARLY_EQUAL_0040EB10, |_, a| {
            u32::from(f32::from_bits(a[0]).abs() <= f32::from_bits(a[2])).into_ret()
        });
        let this = keyframe(&mut e, vtable, 5, &[0.0, 1.0, f32::MAX, 0.5]);
        let other = keyframe(&mut e, vtable, 5, &[1.0, 0.0, 0.5, f32::MAX]);
        // Weight 0.25: 0.25, 0.75; FLT_MAX here takes the other's; FLT_MAX there is left.
        assert!(e
            .call(0x0064_eea0, &args![this, other, 0.25f32, 1u32, 0u32])
            .bool());
        assert_eq!(values(&e, this), vec![0.25, 0.75, 0.5, 0.5]);
        // keep_unset writes FLT_MAX; skip_unset clear leaves the other's zero alone.
        let this = keyframe(&mut e, vtable, 5, &[0.0, 1.0, 0.25, 0.5]);
        assert!(e
            .call(0x0064_eea0, &args![this, other, 2.0f32, 0u32, 1u32])
            .bool());
        assert_eq!(values(&e, this), vec![1.0, 1.0, 0.5, f32::MAX]);
        // A negative weight blends with 0.
        let this = keyframe(&mut e, vtable, 5, &[0.0, 1.0, 0.25, 0.5]);
        assert!(!e
            .call(0x0064_eea0, &args![this, other, -1.0f32, 1u32, 0u32])
            .bool());
        // Refused: itself, another type or count, not multiple.
        assert!(!e
            .call(0x0064_eea0, &args![this, this, 0.5f32, 1u32, 0u32])
            .bool());
        let shorter = keyframe(&mut e, vtable, 5, &[1.0]);
        assert!(!e
            .call(0x0064_eea0, &args![this, shorter, 0.5f32, 1u32, 0u32])
            .bool());
        let typed = keyframe(&mut e, vtable, 6, &[1.0, 0.0, 0.5, 0.5]);
        assert!(!e
            .call(0x0064_eea0, &args![this, typed, 0.5f32, 1u32, 0u32])
            .bool());
        assert!(!e
            .call(0x0064_eea0, &args![this, 0u32, 0.5f32, 1u32, 0u32])
            .bool());
    }

    #[test]
    fn clear_copy_and_compare() {
        let (mut e, vtable) = engine();
        let this = keyframe(&mut e, vtable, 5, &[2.0, 0.5]);
        e.set(this, BSFaceGenKeyframeMultiple::fTime, 1.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0064_f480, &args![this, 0u32]).bool());
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::fTime), 0.0);
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        assert_eq!(calls(&e, MANAGER_RESET_00653DC0), vec![vec![array, 2, 0]]);
        // Time 0 and no valid quantity: nothing to clear.
        let empty = keyframe(&mut e, vtable, 5, &[2.0, -1.0]);
        assert!(!e.call(0x0064_f480, &args![empty, 1u32]).bool());
        // CopyFrom takes the other's type, time and values.
        let other = keyframe(&mut e, vtable, 7, &[0.75, 0.125]);
        e.set(other, BSFaceGenKeyframeMultiple::fTime, 3.0);
        e.call(0x0064_f5b0, &args![this, other]);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::eType), 7);
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::fTime), 3.0);
        assert_eq!(values(&e, this), vec![0.75, 0.125]);
        assert!(!e.call(0x0064_f690, &args![this, other]).bool());
        let array = e.get(other, BSFaceGenKeyframeMultiple::pQuantities);
        e.mem.set_f32(array + 4, f32::NAN);
        assert!(e.call(0x0064_f690, &args![this, other]).bool());
        assert!(e.call(0x0064_f690, &args![this, 0u32]).bool());
        let typed = keyframe(&mut e, vtable, 8, &[0.75, 0.125]);
        assert!(e.call(0x0064_f690, &args![this, typed]).bool());
    }

    #[test]
    fn strength_availability_and_validity() {
        let (mut e, vtable) = engine();
        let this = keyframe(&mut e, vtable, 5, &[0.25, 1.5, 0.75, -0.5, f32::NAN]);
        assert_eq!(e.call(0x0064_f760, &args![this]).f64(), 0.75);
        assert!(e.call(0x0064_f880, &args![this]).bool());
        let unavailable = keyframe(&mut e, vtable, 5, &[0.0, 1.5, -1.0, f32::NAN]);
        assert!(!e.call(0x0064_f880, &args![unavailable]).bool());
        assert_eq!(e.call(0x0064_f760, &args![unavailable]).f64(), 0.0);
        for (index, valid) in [
            (0u32, true),
            (1, false),
            (2, true),
            (3, false),
            (4, false),
            (5, false),
        ] {
            assert_eq!(
                e.call(0x0064_fb90, &args![this, index]).bool(),
                valid,
                "{index}"
            );
        }
        let edges = keyframe(&mut e, vtable, 5, &[0.0, 1.0]);
        assert!(e.call(0x0064_fb90, &args![edges, 0u32]).bool());
        assert!(e.call(0x0064_fb90, &args![edges, 1u32]).bool());
        answer(&mut e, MANAGER_SET_TO_ZERO, 0);
        e.call_log = Some(vec![]);
        e.call(0x0064_f850, &args![this]);
        let array = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
        assert_eq!(calls(&e, MANAGER_SET_TO_ZERO), vec![vec![array, 5]]);
    }

    #[test]
    fn positive_settings() {
        let (mut e, _) = engine();
        let value = e.mem.alloc(4);
        answer(&mut e, SETTING_VALUE, value);
        e.mem.set_f32(value, 0.5);
        assert_eq!(e.call(0x0064_fb10, &[]).f64(), 0.5);
        assert_eq!(e.call(0x0064_fb50, &[]).f64(), 0.5);
        e.mem.set_f32(value, -0.5);
        assert_eq!(e.call(0x0064_fb10, &[]).f64(), 0.0);
        assert_eq!(
            calls(&e, SETTING_VALUE)[..3],
            [
                vec![SETTING_011D57C4],
                vec![SETTING_011D57C4],
                vec![SETTING_011D577C]
            ]
        );
    }

    #[test]
    fn update_to_target_steps_towards_the_target() {
        let (mut e, vtable) = engine();
        // Offset 0.25 and scale 0: the step is 0.25 * time.
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 0.25);
        let none = e.mem.alloc(4);
        e.register_double(SETTING_VALUE, move |_, a| {
            (if a[0] == SETTING_011D57C4 {
                value
            } else {
                none
            })
            .into_ret()
        });
        e.register(ABSOLUTE_00408860, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).abs()),
            ..Ret::default()
        });
        let this = keyframe(&mut e, vtable, 5, &[0.0, 1.0, 0.5, 0.0]);
        let target = keyframe(&mut e, vtable, 5, &[1.0, 0.0, 0.625, 0.0]);
        assert!(e.call(0x0064_f900, &args![this, 2.0f32, target]).bool());
        // Step 0.5: 0.5 up, 0.5 down, reached (0.125 < 0.5), unchanged.
        assert_eq!(values(&e, this), vec![0.5, 0.5, 0.625, 0.0]);
        // No time, the same values, or another type: refused.
        assert!(!e.call(0x0064_f900, &args![this, 0.0f32, target]).bool());
        let same = keyframe(&mut e, vtable, 5, &[0.5, 0.5, 0.625, 0.0]);
        assert!(!e.call(0x0064_f900, &args![this, 1.0f32, same]).bool());
        let typed = keyframe(&mut e, vtable, 6, &[1.0, 0.0, 0.625, 0.0]);
        assert!(!e.call(0x0064_f900, &args![this, 1.0f32, typed]).bool());
    }

    #[test]
    fn keyframe_scalar_deleting_destructor() {
        let (mut e, _) = engine();
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0064_eab0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), KEYFRAME_VTABLE);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0064_eab0, &args![this, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    /// Gives the exe's multiple-keyframe vtable the test slots, and `operator new` a block
    /// with the multiple flag at +0x14.
    fn copies_work(e: &mut Engine, vtable: u32) {
        e.map(MULTIPLE_VTABLE, 0x60);
        for slot in (0..0x60).step_by(4) {
            let target = e.mem.u32(vtable + slot);
            e.mem.set_u32(MULTIPLE_VTABLE + slot, target);
        }
        e.mem.set_u32(MULTIPLE_VTABLE, 0x0064_ebe0);
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0].max(0x18));
            e.mem.set_u32(block + 0x14, 1);
            block.into_ret()
        });
    }

    #[test]
    fn merge_from_two_blends_valid_quantities() {
        let (mut e, vtable) = engine();
        copies_work(&mut e, vtable);
        let this = keyframe(&mut e, vtable, 5, &[0.0, 0.0, 0.0, 0.0]);
        e.mem.set_u32(this.addr() + 4, 7);
        let first = keyframe(&mut e, vtable, 5, &[0.5, 2.0, 0.25, 2.0]);
        let second = keyframe(&mut e, vtable, 5, &[1.0, 0.75, 2.0, 2.0]);
        assert!(e
            .call(0x0064_f0b0, &args![this, first, second, 0.5f32])
            .bool());
        // Takes first's type; blends 0, second's 1, first's 2, leaves 3.
        assert_eq!(e.get(this, BSFaceGenKeyframeMultiple::eType), 5);
        assert_eq!(values(&e, this), vec![0.75, 0.75, 0.25, 0.0]);
        // Weight 0 (clamped from below): first's where valid; the last set unchanged.
        assert!(!e
            .call(0x0064_f0b0, &args![this, first, second, -1.0f32])
            .bool());
        assert_eq!(values(&e, this), vec![0.5, 0.75, 0.25, 0.0]);
        // Weight 1 (clamped from above): second's where valid, else first's.
        e.call(0x0064_f0b0, &args![this, first, second, 3.0f32]);
        assert_eq!(values(&e, this), vec![1.0, 0.75, 0.25, 0.0]);
        // Refused: the same source twice, a null one, or another type.
        assert!(!e
            .call(0x0064_f0b0, &args![this, first, first, 0.5f32])
            .bool());
        assert!(!e
            .call(0x0064_f0b0, &args![this, 0u32, first, 0.5f32])
            .bool());
        let typed = keyframe(&mut e, vtable, 6, &[0.0; 4]);
        assert!(!e
            .call(0x0064_f0b0, &args![this, first, typed, 0.5f32])
            .bool());
    }

    #[test]
    fn merge_from_two_copies_this_and_destroys_the_copy() {
        let (mut e, vtable) = engine();
        copies_work(&mut e, vtable);
        let this = keyframe(&mut e, vtable, 5, &[0.5, 0.5]);
        let second = keyframe(&mut e, vtable, 5, &[1.0, 0.0]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0064_f0b0, &args![this, this, second, 0.5f32])
            .bool());
        assert_eq!(values(&e, this), vec![0.75, 0.25]);
        assert_eq!(calls(&e, OPERATOR_NEW)[0], vec![0x14]);
        // The copy was destroyed and freed (its array, then itself).
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
        // As the second source too.
        e.call_log = Some(vec![]);
        e.call(0x0064_f0b0, &args![this, second, this, 0.5f32]);
        assert_eq!(values(&e, this), vec![0.875, 0.125]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
    }

    #[test]
    fn create_copy_and_is_empty() {
        let (mut e, vtable) = engine();
        copies_work(&mut e, vtable);
        let this = keyframe(&mut e, vtable, 5, &[0.0, 0.25]);
        let copy = Ptr::<BSFaceGenKeyframeMultiple>::new(e.call(0x0064_f530, &args![this]).u32());
        assert_eq!(e.mem.u32(copy.addr()), MULTIPLE_VTABLE);
        assert_eq!(values(&e, copy), vec![0.0, 0.25]);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0064_f530, &args![this]).u32(), 0);
        // Not empty: a valid quantity above 0. Empty: zero, invalid (2.0) or NaN.
        assert!(!e.call(0x0064_f7e0, &args![this]).bool());
        let empty = keyframe(&mut e, vtable, 5, &[0.0, 2.0, f32::from_bits(0x7fc0_0000)]);
        assert!(e.call(0x0064_f7e0, &args![empty]).bool());
    }
}
