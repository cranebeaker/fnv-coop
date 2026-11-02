//! `NiAnimation/niblendquaternioninterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendQuaternionInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_QUATERNION_INTERPOLATOR_RTTI: u32 = 0x011f_4064;

// Translated from 00a55500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_quaternion_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_QUATERNION_INTERPOLATOR_RTTI)
}

/// The vtable of `NiBlendQuaternionInterpolator` (RTTI
/// `.?AVNiBlendQuaternionInterpolator@@`) and the object's size.
const VTABLE: u32 = 0x0109_a3ac;
const OBJECT_SIZE: u32 = 0x40;
/// The quaternion (four words at `011f4208`, filled by the static
/// initializers) the value is reset to when nothing could be blended.
const DEFAULT_QUATERNION: u32 = 0x011f_4208;
/// `NiInterpolator::INVALID_FLOAT` (`0109696c`, Xbox PDB static).
const INVALID_FLOAT: u32 = 0x0109_696c;
/// `operator new` (`00aa13e0`, `cdecl (size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The `NiBlendInterpolator` default constructor body (`00a36890`,
/// `thiscall`, no words), its `CopyMembers` (`00a37510`, `thiscall (this;
/// dest, cloning)`), `IsEqual` (`00a36fd0`), `LoadBinary` (`00a37980`),
/// `SaveBinary` (`00a37ec0`), `GetViewerStrings` (`00a38080`) and
/// `ComputeNormalizedWeights` (`00a37260`, `thiscall`, no words); engine
/// map names.
const BLEND_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_6890;
const BLEND_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a3_7510;
const BLEND_INTERPOLATOR_IS_EQUAL: u32 = 0x00a3_6fd0;
const BLEND_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a3_7980;
const BLEND_INTERPOLATOR_SAVE_BINARY: u32 = 0x00a3_7ec0;
const BLEND_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a3_8080;
const COMPUTE_NORMALIZED_WEIGHTS: u32 = 0x00a3_7260;
/// The four floats' `LoadBinary` (`00a69800`, the engine map's
/// `NiColorA::LoadBinary` on that body; `thiscall (floats; stream)`) and
/// `NiQuaternion::SaveBinary` (`00a6e4a0`, engine map; `thiscall
/// (quaternion; stream)`).
const QUATERNION_LOAD_BINARY: u32 = 0x00a6_9800;
const QUATERNION_SAVE_BINARY: u32 = 0x00a6_e4a0;
/// `0064a290` (`thiscall (quaternion; other)`, no engine map name): true
/// when any of the four floats differs (`FUCOMPP` not equal).
const QUATERNION_NOT_EQUAL: u32 = 0x0064_a290;
/// `NiQuaternion::operator-` (`00a6dbd0`, `thiscall (q; &result)`, answers
/// the result's address), the product with a float (`00a39120`, no engine
/// map name; `thiscall (q; &result, scalar)`, answers the result's
/// address) and `NiQuaternion::Normalize` (`00a38560`, `thiscall`).
const QUATERNION_NEGATE: u32 = 0x00a6_dbd0;
const QUATERNION_TIMES_FLOAT: u32 = 0x00a3_9120;
const QUATERNION_NORMALIZE: u32 = 0x00a3_8560;
/// `NiQuaternion::GetViewerString` (`00a6e190`, engine map; `thiscall
/// (quaternion; name)`) and `"m_kQuaternionValue"`.
const QUATERNION_GET_VIEWER_STRING: u32 = 0x00a6_e190;
const NAME_QUATERNION_VALUE: u32 = 0x0109_8800;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`); the viewer-string
/// array's grow (`0096ad30`, `thiscall (array; new capacity)`) and store
/// (`0096ae90`, `thiscall (array; index, &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The quaternion `Update` of an interpolator (vtable +0x98, `(time,
/// target, NiQuaternion*)`, answers whether it gave a value) and its
/// `IsQuaternionValueSupported` (+0xAC; Xbox PDB slot names).
const UPDATE_QUATERNION_SLOT: u32 = 0x98;
const IS_QUATERNION_VALUE_SUPPORTED_SLOT: u32 = 0xac;

/// `NiBlendInterpolator` fields (Xbox PDB): `m_fLastTime` +8, `m_uFlags`
/// +0xC (bit 0: manager controlled), `m_ucArraySize` +0xD,
/// `m_ucInterpCount` +0xE, `m_pkInterpArray` +0x14,
/// `m_pkSingleInterpolator` +0x18, `m_fSingleTime` +0x20; items of 0x18
/// bytes (interpolator +0, normalized weight +8, update time +0x14). This
/// class's `m_kQuaternionValue` +0x30.
const LAST_TIME: u32 = 0x8;
const FLAGS: u32 = 0xc;
const ARRAY_SIZE: u32 = 0xd;
const INTERP_COUNT: u32 = 0xe;
const INTERP_ARRAY: u32 = 0x14;
const SINGLE_INTERPOLATOR: u32 = 0x18;
const SINGLE_TIME: u32 = 0x20;
const ITEM_SIZE: u32 = 0x18;
const ITEM_NORMALIZED_WEIGHT: u32 = 0x8;
const ITEM_UPDATE_TIME: u32 = 0x14;
const QUATERNION_VALUE: u32 = 0x30;

/// Copies four words.
fn copy_quaternion(e: &mut Engine, to: u32, from: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// Appends `text` to a viewer-string array as the exe's inlined `Add`
/// does: when the 16-bit size (+0xA) has reached the capacity (+0x8) the
/// array grows to the step (+0xE) plus the size, then the string is stored
/// at the old size (through a stack slot holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// The construction both `CreateObject` and `CreateClone` inline: the
/// `NiBlendInterpolator` constructor, the vtable and the default
/// quaternion (copied as words).
fn construct_in(e: &mut Engine, object: u32) {
    e.call(BLEND_INTERPOLATOR_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE);
    copy_quaternion(e, object + QUATERNION_VALUE, DEFAULT_QUATERNION);
}

// Translated from 00a55450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::InterpolatorIsCorrectType` (Xbox PDB,
/// matched by the engine map; `thiscall`, one word: the interpolator): the
/// interpolator's `IsQuaternionValueSupported` (vtable +0xAC).
pub fn ni_blend_quaternion_interpolator_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: u32,
) -> bool {
    e.vcall(interpolator, IS_QUATERNION_VALUE_SUPPORTED_SLOT, &args![])
        .bool()
}

// Translated from 00a55470 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's `CreateObject` (`cdecl`, no words; registered for
/// `"NiBlendQuaternionInterpolator"` by `00a2d8a0`; Ghidra has no function
/// here and the engine map no name): a 0x40-byte block (`00aa13e0`)
/// constructed as [`construct_in`] does, or null when the allocation
/// fails.
pub fn fn_00a55470(e: &mut Engine) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    construct_in(e, object);
    Ptr::new(object)
}

// Translated from 00a554c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::LoadBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream):
/// `NiBlendInterpolator::LoadBinary`, then the quaternion value's four
/// floats (`00a69800`).
pub fn ni_blend_quaternion_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BLEND_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    e.call(
        QUATERNION_LOAD_BINARY,
        &args![this.addr() + QUATERNION_VALUE, stream],
    );
}

// Translated from 00a554e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::SaveBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream):
/// `NiBlendInterpolator::SaveBinary`, then `NiQuaternion::SaveBinary` on
/// the quaternion value.
pub fn ni_blend_quaternion_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BLEND_INTERPOLATOR_SAVE_BINARY, &args![this, stream]);
    e.call(
        QUATERNION_SAVE_BINARY,
        &args![this.addr() + QUATERNION_VALUE, stream],
    );
}

// Translated from 00a55510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::CreateClone` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the cloning process): a new object
/// as `CreateObject` builds it (null when the allocation fails),
/// `CopyMembers` (`00a37510`) into it and the quaternion value copied as
/// words (into address 0x30 when the allocation failed, as the exe does).
/// Answers the new object.
pub fn ni_blend_quaternion_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: u32,
) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![OBJECT_SIZE]).u32();
    if object != 0 {
        construct_in(e, object);
    }
    e.call(
        BLEND_INTERPOLATOR_COPY_MEMBERS,
        &args![this, object, cloning],
    );
    copy_quaternion(
        e,
        object.wrapping_add(QUATERNION_VALUE),
        this.addr() + QUATERNION_VALUE,
    );
    Ptr::new(object)
}

// Translated from 00a55590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::IsEqual` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the other object):
/// `NiBlendInterpolator::IsEqual` and the same quaternion value (`0064a290`
/// answering false).
pub fn ni_blend_quaternion_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(BLEND_INTERPOLATOR_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    !e.call(
        QUATERNION_NOT_EQUAL,
        &args![
            this.addr() + QUATERNION_VALUE,
            other.addr() + QUATERNION_VALUE
        ],
    )
    .bool()
}

// Translated from 00a555c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::StoreSingleValue` (Xbox PDB, matched by
/// the engine map; `thiscall (this; time, target, value)`, `value` a
/// `NiQuaternion*`): the single interpolator's quaternion at `time` (the
/// single time, through the FPU, when manager controlled). At the invalid
/// time (`FUCOMPP` equal), or when its `Update` (vtable +0x98, into the
/// quaternion value) gives nothing, `*value` becomes the default
/// quaternion, the value is copied from it and the answer is false;
/// otherwise the value is copied out and the answer is true.
pub fn ni_blend_quaternion_interpolator_store_single_value(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: u32,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let quaternion = object + QUATERNION_VALUE;
    let mut time_bits = time.to_bits();
    if e.mem.u8(object + FLAGS) & 1 != 0 {
        time_bits = x87_quiet_bits(e.mem.u32(object + SINGLE_TIME));
    }
    let time = x87_load32(time_bits);
    let invalid = x87_load32(e.mem.u32(INVALID_FLOAT));
    if invalid == time {
        copy_quaternion(e, value.addr(), DEFAULT_QUATERNION);
        copy_quaternion(e, quaternion, value.addr());
        return false;
    }
    let single = e.mem.u32(object + SINGLE_INTERPOLATOR);
    let updated = e
        .vcall(
            single,
            UPDATE_QUATERNION_SLOT,
            &args![x87_store32(time), target, quaternion],
        )
        .bool();
    if !updated {
        copy_quaternion(e, value.addr(), DEFAULT_QUATERNION);
        copy_quaternion(e, quaternion, value.addr());
        return false;
    }
    copy_quaternion(e, value.addr(), quaternion);
    true
}

// Translated from 00a556b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::BlendValues` (Xbox PDB, matched by the
/// engine map; `thiscall (this; time, target, value)`): the quaternion
/// value and a weight sum start at 0. For each array item (`m_ucArraySize`
/// items, re-read each round) with an interpolator, a normalized weight
/// above 0 (`FCOM`) and not equal to 0, at its time (the item's update time
/// when manager controlled) unless that is the invalid time: its `Update`
/// (vtable +0x98) into a stack quaternion; when it gives a value, the
/// quaternion is negated (`00a6dbd0`) if the sum is above 0 and its dot
/// product with the value (x87, stored as a `float`) is below 0, scaled by
/// the item's weight (`00a39120`); the value becomes the value scaled by
/// the sum plus that quaternion (x87 adds, component by component),
/// normalized (`00a38560`), and the weight joins the sum. When any item
/// gave a value it is copied out and the answer is true; otherwise the
/// value (and `*value`) become the default quaternion and the answer is
/// false.
pub fn ni_blend_quaternion_interpolator_blend_values(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: u32,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let quaternion = object + QUATERNION_VALUE;
    // FLDZ, FST into the four floats and the weight sum.
    let zero = x87_store32(0.0);
    for i in 0..4 {
        e.mem.set_u32(quaternion + 4 * i, zero);
    }
    let mut sum = zero;
    let mut blended = false;
    if e.mem.u8(object + ARRAY_SIZE) != 0 {
        // The frame: the update's quaternion +0, the negation's result
        // +0x10, the two products' results +0x20 and +0x30.
        e.with_stack(0x40, |e, frame| {
            let local = frame.addr();
            let negated = local + 0x10;
            let scaled = local + 0x20;
            let scaled_value = local + 0x30;
            let mut index = 0u8;
            loop {
                let offset = u32::from(index) * ITEM_SIZE;
                let item = e.mem.u32(object + INTERP_ARRAY).wrapping_add(offset);
                let interpolator = e.mem.u32(item);
                let weight = x87_m32(e.mem.u32(item.wrapping_add(ITEM_NORMALIZED_WEIGHT)));
                // FCOM (0 against the weight), TEST AH,5 / JP: only a
                // weight above 0 goes on; FCOM, TEST AH,0x44 / JNP: an
                // equal one stops.
                if interpolator != 0 && 0.0 < weight && 0.0 != weight {
                    let mut item_time = x87_quiet_bits(time.to_bits());
                    if e.mem.u8(object + FLAGS) & 1 != 0 {
                        item_time = x87_quiet_bits(e.mem.u32(item.wrapping_add(ITEM_UPDATE_TIME)));
                    }
                    let item_time = x87_load32(item_time);
                    let invalid = x87_load32(e.mem.u32(INVALID_FLOAT));
                    if invalid != item_time
                        && e.vcall(
                            interpolator,
                            UPDATE_QUATERNION_SLOT,
                            &args![x87_store32(item_time), target, local],
                        )
                        .bool()
                    {
                        if 0.0 < x87_m32(sum) {
                            let y = x87_mul(
                                x87_load32(e.mem.u32(quaternion + 4)),
                                x87_m32(e.mem.u32(local + 4)),
                            );
                            let x = x87_mul(
                                x87_load32(e.mem.u32(quaternion)),
                                x87_m32(e.mem.u32(local)),
                            );
                            let dot = x87_add(y, x);
                            let z = x87_mul(
                                x87_load32(e.mem.u32(quaternion + 8)),
                                x87_m32(e.mem.u32(local + 8)),
                            );
                            let dot = x87_add(dot, z);
                            let w = x87_mul(
                                x87_load32(e.mem.u32(quaternion + 0xc)),
                                x87_m32(e.mem.u32(local + 0xc)),
                            );
                            let dot = x87_store32(x87_add(dot, w));
                            // FCOMP (0 against the dot), TEST AH,0x41 /
                            // JNZ: only a dot below 0 negates.
                            if x87_m32(dot) < 0.0 {
                                let result =
                                    e.call(QUATERNION_NEGATE, &args![local, negated]).u32();
                                copy_quaternion(e, local, result);
                            }
                        }
                        let item = e.mem.u32(object + INTERP_ARRAY).wrapping_add(offset);
                        let weight =
                            x87_quiet_bits(e.mem.u32(item.wrapping_add(ITEM_NORMALIZED_WEIGHT)));
                        let result = e
                            .call(QUATERNION_TIMES_FLOAT, &args![local, scaled, weight])
                            .u32();
                        copy_quaternion(e, local, result);
                        let result = e
                            .call(
                                QUATERNION_TIMES_FLOAT,
                                &args![quaternion, scaled_value, x87_quiet_bits(sum)],
                            )
                            .u32();
                        copy_quaternion(e, quaternion, result);
                        let w = x87_quiet_bits(e.mem.u32(quaternion + 0xc));
                        let z = x87_quiet_bits(e.mem.u32(quaternion + 8));
                        let y = x87_quiet_bits(e.mem.u32(quaternion + 4));
                        let x =
                            x87_add(x87_load32(e.mem.u32(local)), x87_m32(e.mem.u32(quaternion)));
                        e.mem.set_u32(quaternion, x87_store32(x));
                        for (i, part) in [(1u32, y), (2, z), (3, w)] {
                            let sum_part =
                                x87_add(x87_load32(e.mem.u32(local + 4 * i)), x87_m32(part));
                            e.mem.set_u32(quaternion + 4 * i, x87_store32(sum_part));
                        }
                        e.call(QUATERNION_NORMALIZE, &args![quaternion]);
                        let item = e.mem.u32(object + INTERP_ARRAY).wrapping_add(offset);
                        let weight = e.mem.u32(item.wrapping_add(ITEM_NORMALIZED_WEIGHT));
                        blended = true;
                        sum = x87_store32(x87_add(x87_load32(weight), x87_m32(sum)));
                    }
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u8(object + ARRAY_SIZE) {
                    break;
                }
            }
        });
        if blended {
            copy_quaternion(e, value.addr(), quaternion);
            return true;
        }
    }
    copy_quaternion(e, quaternion, DEFAULT_QUATERNION);
    copy_quaternion(e, value.addr(), quaternion);
    false
}

// Translated from 00a55920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::Update` (Xbox PDB, matched by the
/// engine map; `thiscall (this; time, target, value)`): one interpolator
/// stores its single value ([`ni_blend_quaternion_interpolator_store_single_value`]),
/// several are weighted (`ComputeNormalizedWeights`, `00a37260`) and
/// blended ([`ni_blend_quaternion_interpolator_blend_values`]), none gives
/// false; the time (through the FPU) becomes the last time either way.
pub fn ni_blend_quaternion_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: u32,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let count = e.mem.u8(object + INTERP_COUNT);
    let time_bits = x87_quiet_bits(time.to_bits());
    let answer = if count == 1 {
        ni_blend_quaternion_interpolator_store_single_value(
            e,
            this,
            f32::from_bits(time_bits),
            target,
            value,
        )
    } else if count != 0 {
        e.call(COMPUTE_NORMALIZED_WEIGHTS, &args![this]);
        ni_blend_quaternion_interpolator_blend_values(
            e,
            this,
            f32::from_bits(time_bits),
            target,
            value,
        )
    } else {
        false
    };
    e.mem.set_u32(object + LAST_TIME, time_bits);
    answer
}

// Translated from 00a55980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendQuaternionInterpolator::GetViewerStrings` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: the strings array):
/// `NiBlendInterpolator::GetViewerStrings` (`00a38080`), then the class
/// name (the first word of the `NiRTTI` record) and the quaternion value's
/// string (`NiQuaternion::GetViewerString` with `"m_kQuaternionValue"`),
/// each appended.
pub fn ni_blend_quaternion_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    e.call(BLEND_INTERPOLATOR_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_BLEND_QUATERNION_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let text = e
        .call(
            QUATERNION_GET_VIEWER_STRING,
            &args![this.addr() + QUATERNION_VALUE, NAME_QUATERNION_VALUE],
        )
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a55500, ni_blend_quaternion_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a5_5450,
            ni_blend_quaternion_interpolator_interpolator_is_correct_type(Ptr, u32) -> bool
        ),
        entry!(0x00a5_5470, fn_00a55470() -> Ptr),
        entry!(
            0x00a5_54c0,
            ni_blend_quaternion_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_54e0,
            ni_blend_quaternion_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_5510,
            ni_blend_quaternion_interpolator_create_clone(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a5_5590,
            ni_blend_quaternion_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_55c0,
            ni_blend_quaternion_interpolator_store_single_value(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_56b0,
            ni_blend_quaternion_interpolator_blend_values(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_5920,
            ni_blend_quaternion_interpolator_update(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_5980,
            ni_blend_quaternion_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_quaternion_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a55500, &args![]).u32(),
            NI_BLEND_QUATERNION_INTERPOLATOR_RTTI
        );
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    const UPDATE: u32 = 0x0001_0200;
    const SUPPORTED: u32 = 0x0001_0210;
    const VTABLE_A: u32 = 0x0002_0000;
    const DEFAULT: [f32; 4] = [1.0, 0.0, 0.0, 0.0];

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quat(e: &Engine, at: u32) -> [f32; 4] {
        [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
    }

    fn set_quat(e: &mut Engine, at: u32, q: [f32; 4]) {
        for (i, v) in q.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    /// An engine with the default quaternion and the invalid float mapped,
    /// quaternion helpers that compute, and an interpolator vtable whose
    /// `Update` writes the quaternion the interpolator holds at +8 (and
    /// answers its +4 byte).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(DEFAULT_QUATERNION, 0x10);
        e.map(INVALID_FLOAT, 4);
        set_quat(&mut e, DEFAULT_QUATERNION, DEFAULT);
        e.mem.set_f32(INVALID_FLOAT, f32::MIN);
        e.register(QUATERNION_NEGATE, |e, a| {
            for i in 0..4 {
                let v = -e.mem.f32(a[0] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            word(a[1])
        });
        e.register(QUATERNION_TIMES_FLOAT, |e, a| {
            let s = f32::from_bits(a[2]);
            for i in 0..4 {
                let v = e.mem.f32(a[0] + 4 * i) * s;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            word(a[1])
        });
        e.register(QUATERNION_NORMALIZE, |_, _| Ret::default());
        e.register(COMPUTE_NORMALIZED_WEIGHTS, |_, _| Ret::default());
        e.register(UPDATE, |e, a| {
            let source = a[0] + 8;
            for i in 0..4 {
                let v = e.mem.u32(source + 4 * i);
                e.mem.set_u32(a[3] + 4 * i, v);
            }
            word(u32::from(e.mem.u8(a[0] + 4)))
        });
        e.register(SUPPORTED, |_, a| word(a[0] & 1));
        let mut slots = vec![0u32; 0x2c];
        slots[(UPDATE_QUATERNION_SLOT / 4) as usize] = UPDATE;
        slots[(IS_QUATERNION_VALUE_SUPPORTED_SLOT / 4) as usize] = SUPPORTED;
        e.put_vtable(VTABLE_A, &slots);
        e.call_log = Some(vec![]);
        e
    }

    /// An interpolator whose update gives `q` (or fails when `ok` is false).
    fn source(e: &mut Engine, ok: bool, q: [f32; 4]) -> u32 {
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, VTABLE_A);
        e.mem.set_u8(object + 4, u8::from(ok));
        set_quat(e, object + 8, q);
        object
    }

    fn blend(e: &mut Engine, items: &[(u32, f32, f32)]) -> u32 {
        let this = e.mem.alloc(OBJECT_SIZE);
        let array = e.mem.alloc(ITEM_SIZE * items.len() as u32 + 4);
        for (i, (interp, weight, time)) in items.iter().enumerate() {
            let item = array + ITEM_SIZE * i as u32;
            e.mem.set_u32(item, *interp);
            e.mem.set_f32(item + ITEM_NORMALIZED_WEIGHT, *weight);
            e.mem.set_f32(item + ITEM_UPDATE_TIME, *time);
        }
        e.mem.set_u32(this + INTERP_ARRAY, array);
        e.mem.set_u8(this + ARRAY_SIZE, items.len() as u8);
        this
    }

    #[test]
    fn interpolator_is_correct_type_asks_the_interpolator() {
        let mut e = engine();
        let interp = source(&mut e, true, DEFAULT);
        assert!(!e.call(0x00a5_5450, &args![0x1000u32, interp]).bool());
        assert_eq!(calls(&e, SUPPORTED), vec![vec![interp]]);
        let odd = e.mem.alloc(0x10) | 1;
        e.mem.set_u32(odd, VTABLE_A);
        assert!(e.call(0x00a5_5450, &args![0x1000u32, odd]).bool());
    }

    #[test]
    fn create_object_starts_at_the_default_quaternion() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| word(e.mem.alloc(a[0])));
        e.register(BLEND_INTERPOLATOR_CONSTRUCT, |_, a| word(a[0]));
        let object = e.call(0x00a5_5470, &args![]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x40]]);
        assert_eq!(calls(&e, BLEND_INTERPOLATOR_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(quat(&e, object + QUATERNION_VALUE), DEFAULT);
        e.register(OPERATOR_NEW, |_, _| word(0));
        assert_eq!(e.call(0x00a5_5470, &args![]).u32(), 0);
    }

    #[test]
    fn load_and_save_binary_add_the_quaternion() {
        let mut e = engine();
        for addr in [
            BLEND_INTERPOLATOR_LOAD_BINARY,
            BLEND_INTERPOLATOR_SAVE_BINARY,
            QUATERNION_LOAD_BINARY,
            QUATERNION_SAVE_BINARY,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call(0x00a5_54c0, &args![0x1000u32, 0x2000u32]);
        e.call(0x00a5_54e0, &args![0x1000u32, 0x2000u32]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap();
        let inner: Vec<_> = log
            .iter()
            .filter(|c| c.0 != 0x00a5_54c0 && c.0 != 0x00a5_54e0)
            .cloned()
            .collect();
        assert_eq!(
            inner,
            vec![
                (BLEND_INTERPOLATOR_LOAD_BINARY, vec![0x1000, 0x2000]),
                (QUATERNION_LOAD_BINARY, vec![0x1030, 0x2000]),
                (BLEND_INTERPOLATOR_SAVE_BINARY, vec![0x1000, 0x2000]),
                (QUATERNION_SAVE_BINARY, vec![0x1030, 0x2000]),
            ]
        );
    }

    #[test]
    fn create_clone_copies_members_and_value() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| word(e.mem.alloc(a[0])));
        e.register(BLEND_INTERPOLATOR_CONSTRUCT, |_, a| word(a[0]));
        e.register(BLEND_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(OBJECT_SIZE);
        set_quat(&mut e, this + QUATERNION_VALUE, [0.5, 0.25, 0.125, 2.0]);
        let clone = e.call(0x00a5_5510, &args![this, 0x900u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(
            calls(&e, BLEND_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, clone, 0x900]]
        );
        assert_eq!(quat(&e, clone + QUATERNION_VALUE), [0.5, 0.25, 0.125, 2.0]);
        // A failed allocation copies into address 0 (+0x30).
        e.map(0, 0x40);
        e.register(OPERATOR_NEW, |_, _| word(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_5510, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(
            calls(&e, BLEND_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, 0, 0x900]]
        );
        assert_eq!(quat(&e, QUATERNION_VALUE), [0.5, 0.25, 0.125, 2.0]);
    }

    #[test]
    fn is_equal_needs_the_base_and_the_same_value() {
        let mut e = engine();
        e.register(BLEND_INTERPOLATOR_IS_EQUAL, |_, _| word(1));
        e.register(QUATERNION_NOT_EQUAL, |_, _| word(0));
        assert!(e.call(0x00a5_5590, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(calls(&e, QUATERNION_NOT_EQUAL), vec![vec![0x1030, 0x2030]]);
        e.register(QUATERNION_NOT_EQUAL, |_, _| word(1));
        assert!(!e.call(0x00a5_5590, &args![0x1000u32, 0x2000u32]).bool());
        e.register(BLEND_INTERPOLATOR_IS_EQUAL, |_, _| word(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a5_5590, &args![0x1000u32, 0x2000u32]).bool());
        assert!(calls(&e, QUATERNION_NOT_EQUAL).is_empty());
    }

    #[test]
    fn store_single_value_updates_or_falls_back_to_the_default() {
        let mut e = engine();
        let this = e.mem.alloc(OBJECT_SIZE);
        let value = e.mem.alloc(0x10);
        let single = source(&mut e, true, [0.0, 1.0, 0.0, 0.0]);
        e.mem.set_u32(this + SINGLE_INTERPOLATOR, single);
        assert!(e
            .call(0x00a5_55c0, &args![this, 2.0f32, 0x77u32, value])
            .bool());
        assert_eq!(
            calls(&e, UPDATE),
            vec![vec![
                single,
                2.0f32.to_bits(),
                0x77,
                this + QUATERNION_VALUE
            ]]
        );
        assert_eq!(quat(&e, value), [0.0, 1.0, 0.0, 0.0]);
        // Manager controlled: the single time, here the invalid one.
        e.mem.set_u8(this + FLAGS, 1);
        e.mem.set_f32(this + SINGLE_TIME, f32::MIN);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00a5_55c0, &args![this, 2.0f32, 0x77u32, value])
            .bool());
        assert!(calls(&e, UPDATE).is_empty());
        assert_eq!(quat(&e, value), DEFAULT);
        assert_eq!(quat(&e, this + QUATERNION_VALUE), DEFAULT);
        // A failing update.
        e.mem.set_f32(this + SINGLE_TIME, 3.0);
        e.mem.set_u8(single + 4, 0);
        set_quat(&mut e, value, [9.0; 4]);
        assert!(!e
            .call(0x00a5_55c0, &args![this, 2.0f32, 0x77u32, value])
            .bool());
        assert_eq!(calls(&e, UPDATE)[0][1], 3.0f32.to_bits());
        assert_eq!(quat(&e, value), DEFAULT);
        assert_eq!(quat(&e, this + QUATERNION_VALUE), DEFAULT);
    }

    #[test]
    fn blend_values_sums_weighted_quaternions_flipping_opposite_ones() {
        let mut e = engine();
        let first = source(&mut e, true, [0.0, 1.0, 0.0, 0.0]);
        let second = source(&mut e, true, [0.0, -1.0, 0.0, 0.0]);
        let failing = source(&mut e, false, [5.0; 4]);
        let this = blend(
            &mut e,
            &[
                (first, 0.25, 0.0),
                (0, 0.5, 0.0),
                (failing, 0.5, 0.0),
                (first, 0.0, 0.0),
                (second, 0.75, 0.0),
            ],
        );
        let value = e.mem.alloc(0x10);
        assert!(e
            .call(0x00a5_56b0, &args![this, 2.0f32, 0x77u32, value])
            .bool());
        let updates = calls(&e, UPDATE);
        assert_eq!(updates.len(), 3);
        assert_eq!(updates[0][1..3], [2.0f32.to_bits(), 0x77]);
        // The second quaternion opposes the sum so far: negated.
        assert_eq!(calls(&e, QUATERNION_NEGATE).len(), 1);
        assert_eq!(calls(&e, QUATERNION_NORMALIZE).len(), 2);
        // 0.25 * (0,1,0,0), then 0.25 * that + 0.75 * (0,1,0,0).
        assert_eq!(quat(&e, this + QUATERNION_VALUE), [0.0, 0.8125, 0.0, 0.0]);
        assert_eq!(quat(&e, value), [0.0, 0.8125, 0.0, 0.0]);
        let products = calls(&e, QUATERNION_TIMES_FLOAT);
        assert_eq!(products[1][2], 0.0f32.to_bits());
        assert_eq!(products[3][2], 0.25f32.to_bits());
    }

    #[test]
    fn blend_values_uses_item_times_and_falls_back_to_the_default() {
        let mut e = engine();
        let first = source(&mut e, true, [0.0, 1.0, 0.0, 0.0]);
        let this = blend(&mut e, &[(first, 1.0, f32::MIN)]);
        e.mem.set_u8(this + FLAGS, 1);
        let value = e.mem.alloc(0x10);
        assert!(!e
            .call(0x00a5_56b0, &args![this, 2.0f32, 0x77u32, value])
            .bool());
        assert!(calls(&e, UPDATE).is_empty());
        assert_eq!(quat(&e, value), DEFAULT);
        assert_eq!(quat(&e, this + QUATERNION_VALUE), DEFAULT);
        // The item's own time when manager controlled.
        let item = e.mem.u32(this + INTERP_ARRAY);
        e.mem.set_f32(item + ITEM_UPDATE_TIME, 4.0);
        assert!(e
            .call(0x00a5_56b0, &args![this, 2.0f32, 0x77u32, value])
            .bool());
        assert_eq!(calls(&e, UPDATE)[0][1], 4.0f32.to_bits());
        // An empty array: the default.
        e.mem.set_u8(this + ARRAY_SIZE, 0);
        assert!(!e
            .call(0x00a5_56b0, &args![this, 2.0f32, 0x77u32, value])
            .bool());
        assert_eq!(quat(&e, value), DEFAULT);
    }

    #[test]
    fn update_dispatches_on_the_interpolator_count() {
        let mut e = engine();
        let single = source(&mut e, true, [0.0, 0.0, 1.0, 0.0]);
        let this = blend(&mut e, &[(single, 1.0, 0.0)]);
        e.mem.set_u32(this + SINGLE_INTERPOLATOR, single);
        let value = e.mem.alloc(0x10);
        // None: false, the time still recorded.
        assert!(!e
            .call(0x00a5_5920, &args![this, 1.5f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(this + LAST_TIME), 1.5);
        assert!(calls(&e, UPDATE).is_empty());
        // One: the single interpolator.
        e.mem.set_u8(this + INTERP_COUNT, 1);
        assert!(e
            .call(0x00a5_5920, &args![this, 2.5f32, 0u32, value])
            .bool());
        assert!(calls(&e, COMPUTE_NORMALIZED_WEIGHTS).is_empty());
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.5);
        // Several: weights, then the blend.
        e.mem.set_u8(this + INTERP_COUNT, 2);
        assert!(e
            .call(0x00a5_5920, &args![this, 3.5f32, 0u32, value])
            .bool());
        assert_eq!(calls(&e, COMPUTE_NORMALIZED_WEIGHTS), vec![vec![this]]);
        assert_eq!(quat(&e, value), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.mem.f32(this + LAST_TIME), 3.5);
    }

    #[test]
    fn get_viewer_strings_appends_name_and_value() {
        let mut e = engine();
        e.map(NI_BLEND_QUATERNION_INTERPOLATOR_RTTI, 4);
        e.set_global(NI_BLEND_QUATERNION_INTERPOLATOR_RTTI, 0x0109_6666u32);
        e.register(BLEND_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0xf1));
        e.register(QUATERNION_GET_VIEWER_STRING, |_, _| word(0xf2));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a5_5980, &args![0x1000u32, strings]);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_6666]]);
        assert_eq!(
            calls(&e, QUATERNION_GET_VIEWER_STRING),
            vec![vec![0x1030, NAME_QUATERNION_VALUE]]
        );
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 3]]);
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 2);
    }
}
