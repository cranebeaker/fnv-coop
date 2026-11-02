//! `NiAnimation/niblendcolorinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendColorInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_COLOR_INTERPOLATOR_RTTI: u32 = 0x011f_4074;

// Translated from 00a56060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_color_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_COLOR_INTERPOLATOR_RTTI)
}

// Translated from 00a56070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a37170`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x44 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_blend_color_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_7170;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x44u32]);
    }
    this
}

/// `NiBlendColorInterpolator`'s vtable and size.
const VTABLE: u32 = 0x0109_a58c;
const OBJECT_SIZE: u32 = 0x44;
/// `NiBlendColorInterpolator` fields (Xbox PDB, same offsets on PC):
/// `m_kColorValue` +0x30 (`NiColorA`, four floats), `m_bValidValue` +0x40.
const COLOR_VALUE: u32 = 0x30;
const VALID_VALUE: u32 = 0x40;
/// `NiBlendInterpolator` fields (Xbox PDB): `m_fLastTime` +8, `m_uFlags`
/// +0xC (bit 0 manager controlled), `m_ucArraySize` +0xD, `m_ucInterpCount`
/// +0xE, `m_pkInterpArray` +0x14 (items of 0x18: interpolator +0,
/// normalized weight +8, update time +0x14), `m_pkSingleInterpolator`
/// +0x18, `m_fSingleTime` +0x20.
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
/// The "no color" value (`011a9430`, four words) and
/// `NiInterpolator::INVALID_FLOAT` (`0109696c`).
const INVALID_COLOR: u32 = 0x011a_9430;
const INVALID_TIME: u32 = 0x0109_696c;
/// `NiInterpolator`'s `IsColorAValueSupported` (vtable +0xB4) and color
/// `Update` (+0x90, `(time, target, &color)`).
const SLOT_IS_COLOR_VALUE_SUPPORTED: u32 = 0xb4;
const SLOT_UPDATE_COLOR: u32 = 0x90;
/// The `NiBlendInterpolator` functions this class builds on: the
/// constructor (`00a370b0`, `(this, manager controlled, threshold, array
/// size)`), the default constructor (`00a36890`), the member copy
/// (`00a37510`), `ComputeNormalizedWeights` (`00a37260`), `IsEqual`
/// (`00a36fd0`), `SaveBinary` (`00a37ec0`), `LoadBinary` (`00a37980`) and
/// `GetViewerStrings` (`00a38080`).
const BLEND_CONSTRUCT: u32 = 0x00a3_70b0;
const BLEND_CONSTRUCT_DEFAULT: u32 = 0x00a3_6890;
const BLEND_COPY_MEMBERS: u32 = 0x00a3_7510;
const COMPUTE_NORMALIZED_WEIGHTS: u32 = 0x00a3_7260;
const BLEND_IS_EQUAL: u32 = 0x00a3_6fd0;
const BLEND_SAVE_BINARY: u32 = 0x00a3_7ec0;
const BLEND_LOAD_BINARY: u32 = 0x00a3_7980;
const BLEND_GET_VIEWER_STRINGS: u32 = 0x00a3_8080;
/// `NiColorA::SaveBinary` (`00a698a0`), `LoadBinary` (`00a69800`), the
/// color comparison `LoadBinary` and `IsEqual` use (`004a4c90`, `thiscall
/// (color, &other)`) and the color's viewer string (`00a69720`, `thiscall
/// (color, name)`).
const COLOR_SAVE_BINARY: u32 = 0x00a6_98a0;
const COLOR_LOAD_BINARY: u32 = 0x00a6_9800;
const COLOR_COMPARE: u32 = 0x004a_4c90;
const COLOR_VIEWER_STRING: u32 = 0x00a6_9720;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`) and the name
/// `"m_kColorValue"`.
const VIEWER_STRING: u32 = 0x00a7_d330;
const COLOR_VALUE_NAME: u32 = 0x0109_6f1c;

fn copy_color(e: &mut Engine, from: u32, to: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// The fields after the base constructor: the vtable, the "no color" value
/// and no valid value.
fn init_members(e: &mut Engine, object: u32) {
    e.mem.set_u32(object, VTABLE);
    copy_color(e, INVALID_COLOR, object + COLOR_VALUE);
    e.mem.set_u8(object + VALID_VALUE, 0);
}

// Translated from 00a55f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::NiBlendColorInterpolator` (Xbox PDB): the
/// `NiBlendInterpolator` constructor with the three arguments (the threshold
/// through `FLD`/`FSTP`), the class's vtable, the "no color" value and no
/// valid value. Answers `this`.
pub fn ni_blend_color_interpolator_ni_blend_color_interpolator(
    e: &mut Engine,
    this: Ptr,
    manager_controlled: u32,
    weight_threshold: f32,
    array_size: u32,
) -> Ptr {
    let threshold = x87_quiet_bits(weight_threshold.to_bits());
    e.call(
        BLEND_CONSTRUCT,
        &args![this, manager_controlled, threshold, array_size],
    );
    init_members(e, this.addr());
    this
}

// Translated from 00a55fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::InterpolatorIsCorrectType` (Xbox PDB;
/// `thiscall`, one stack word): the interpolator's `IsColorAValueSupported`
/// (vtable +0xB4); `this` is not read.
pub fn ni_blend_color_interpolator_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: Ptr,
) -> bool {
    e.vcall(interpolator.addr(), SLOT_IS_COLOR_VALUE_SUPPORTED, &args![])
        .bool()
}

// Translated from 00a55ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (registered through `NiStream::RegisterLoader` at
/// `00a2d8ad`; Ghidra made no function of it): 0x44 bytes from the Ni
/// allocator, the `NiBlendInterpolator` default constructor, the vtable,
/// the "no color" value and no valid value. Null when the allocation fails.
pub fn fn_00a55ff0(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(BLEND_CONSTRUCT_DEFAULT, &args![object]);
    init_members(e, object);
    Ptr::new(object)
}

// Translated from 00a56040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::SaveBinary` (Xbox PDB): `NiBlendInterpolator`'s
/// `SaveBinary`, then the color (`NiColorA::SaveBinary`).
pub fn ni_blend_color_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BLEND_SAVE_BINARY, &args![this, stream]);
    e.call(COLOR_SAVE_BINARY, &args![this.addr() + COLOR_VALUE, stream]);
}

// Translated from 00a56090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::CreateClone` (Xbox PDB): a new object as the
/// factory makes it (null when the allocation fails), the
/// `NiBlendInterpolator` members copied into it, then the color and the
/// valid flag; the exe stores them through the clone pointer even when it is
/// null.
pub fn ni_blend_color_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = fn_00a55ff0(e);
    e.call(BLEND_COPY_MEMBERS, &args![this, clone, cloning]);
    copy_color(
        e,
        this.addr() + COLOR_VALUE,
        clone.addr().wrapping_add(COLOR_VALUE),
    );
    let valid = e.mem.u8(this.addr() + VALID_VALUE);
    e.mem.set_u8(clone.addr().wrapping_add(VALID_VALUE), valid);
    clone
}

// Translated from 00a56110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::StoreSingleValue` (Xbox PDB; `thiscall (time,
/// target, &value)`): the time is the single time when manager controlled
/// (flag 1); at `INVALID_FLOAT` false. Otherwise the single interpolator's
/// color `Update` (vtable +0x90) into this one's color: on success the color
/// is copied to `value` and true answered; on failure `value` and the color
/// become the "no color" value and false is answered.
pub fn ni_blend_color_interpolator_store_single_value(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: Ptr,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let time = if e.mem.u8(object + FLAGS) & 1 != 0 {
        x87_quiet_bits(e.mem.u32(object + SINGLE_TIME))
    } else {
        x87_quiet_bits(time.to_bits())
    };
    if x87_load32(e.mem.u32(INVALID_TIME)) == x87_load32(time) {
        return false;
    }
    let single = e.mem.u32(object + SINGLE_INTERPOLATOR);
    let color = object + COLOR_VALUE;
    if e.vcall(single, SLOT_UPDATE_COLOR, &args![time, target, color])
        .bool()
    {
        copy_color(e, color, value.addr());
        return true;
    }
    copy_color(e, INVALID_COLOR, value.addr());
    copy_color(e, value.addr(), color);
    false
}

// Translated from 00a561c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::BlendValues` (Xbox PDB; `thiscall (time,
/// target, &value)`): the color starts at zero, the valid flag clear and the
/// weight total at 1. Each item with an interpolator and a positive
/// normalized weight is updated (vtable +0x90, at its update time when
/// manager controlled) into a zeroed local color: on success the weighted
/// local color is added to the color and the flag set; otherwise (also for a
/// weight of 0 or a time at `INVALID_FLOAT`) its weight is taken off the
/// total. With the flag set the color is divided by the total, copied to
/// `value` and true answered; without, the color and `value` become the "no
/// color" value and false is answered. Every product, sum and quotient is an
/// x87 operation stored as a `float`, in the exe's order.
pub fn ni_blend_color_interpolator_blend_values(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: Ptr,
    value: Ptr,
) -> bool {
    let object = this.addr();
    for i in 0..4 {
        e.mem.set_u32(object + COLOR_VALUE + 4 * i, 0);
    }
    let mut total = 1.0f32.to_bits();
    e.mem.set_u8(object + VALID_VALUE, 0);
    let mut i = 0u8;
    while i < e.mem.u8(object + ARRAY_SIZE) {
        let offset = u32::from(i) * ITEM_SIZE;
        let entry = e.mem.u32(object + INTERP_ARRAY).wrapping_add(offset);
        let interpolator = e.mem.u32(entry);
        let weight = |e: &Engine, at: u32| x87_m32(e.mem.u32(at + ITEM_NORMALIZED_WEIGHT));
        if interpolator != 0 && 0.0 < weight(e, entry) {
            let subtract = |e: &mut Engine, at: u32, total: u32| {
                x87_store32(x87_sub(x87_load32(total), weight(e, at)))
            };
            let mut update = None;
            if 0.0 != weight(e, entry) {
                let item_time = if e.mem.u8(object + FLAGS) & 1 != 0 {
                    x87_quiet_bits(e.mem.u32(entry + ITEM_UPDATE_TIME))
                } else {
                    x87_quiet_bits(time.to_bits())
                };
                if x87_load32(e.mem.u32(INVALID_TIME)) != x87_load32(item_time) {
                    update = Some(item_time);
                }
            }
            match update {
                None => total = subtract(e, entry, total),
                Some(item_time) => {
                    let (updated, local) = e.with_stack(0x10, |e, local| {
                        let local = local.addr();
                        let updated = e
                            .vcall(
                                interpolator,
                                SLOT_UPDATE_COLOR,
                                &args![item_time, target, local],
                            )
                            .bool();
                        let words: [u32; 4] =
                            std::array::from_fn(|k| e.mem.u32(local + 4 * k as u32));
                        (updated, words)
                    });
                    let entry = e.mem.u32(object + INTERP_ARRAY).wrapping_add(offset);
                    if updated {
                        // `FLD`, `FSTP` to a local and `FLD` again.
                        let w = x87_load32(e.mem.u32(entry + ITEM_NORMALIZED_WEIGHT));
                        let products = [
                            x87_store32(x87_mul(x87_load32(local[0]), w)),
                            x87_store32(x87_mul(x87_load32(local[1]), w)),
                            x87_store32(x87_mul(x87_load32(local[2]), w)),
                            x87_store32(x87_mul(w, x87_m32(local[3]))),
                        ];
                        for (k, product) in products.iter().enumerate() {
                            let at = object + COLOR_VALUE + 4 * k as u32;
                            let sum = x87_add(x87_load32(e.mem.u32(at)), x87_m32(*product));
                            e.mem.set_u32(at, x87_store32(sum));
                        }
                        e.mem.set_u8(object + VALID_VALUE, 1);
                    } else {
                        total = subtract(e, entry, total);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if e.mem.u8(object + VALID_VALUE) == 0 {
        copy_color(e, INVALID_COLOR, object + COLOR_VALUE);
        copy_color(e, object + COLOR_VALUE, value.addr());
        return false;
    }
    let total = x87_load32(total);
    for k in 0..3 {
        let at = object + COLOR_VALUE + 4 * k;
        let quotient = x87_div(x87_load32(e.mem.u32(at)), total);
        e.mem.set_u32(at, x87_store32(quotient));
    }
    let at = object + COLOR_VALUE + 0xc;
    let quotient = x87_div(x87_m32(e.mem.u32(at)), total);
    e.mem.set_u32(at, x87_store32(quotient));
    copy_color(e, object + COLOR_VALUE, value.addr());
    true
}

// Translated from 00a56400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::Update` (Xbox PDB; `thiscall (time, target,
/// &value)`): with one interpolator `StoreSingleValue`; with more the
/// weights are normalized and the values blended (`BlendValues`); with none
/// false. The last time takes `time` (through the FPU) in every case; the
/// answer is the called function's.
pub fn ni_blend_color_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: Ptr,
    value: Ptr,
) -> bool {
    let count = e.mem.u8(this.addr() + INTERP_COUNT);
    let time_bits = x87_quiet_bits(time.to_bits());
    let answer = if count == 1 {
        ni_blend_color_interpolator_store_single_value(
            e,
            this,
            f32::from_bits(time_bits),
            target,
            value,
        )
    } else if count != 0 {
        e.call(COMPUTE_NORMALIZED_WEIGHTS, &args![this]);
        ni_blend_color_interpolator_blend_values(e, this, f32::from_bits(time_bits), target, value)
    } else {
        false
    };
    e.mem.set_u32(this.addr() + LAST_TIME, time_bits);
    answer
}

// Translated from 00a56460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::LoadBinary` (Xbox PDB): `NiBlendInterpolator`'s
/// `LoadBinary`, the color (`NiColorA::LoadBinary`), and the valid flag set
/// when the color comparison (`004a4c90`) with the "no color" value answers
/// true.
pub fn ni_blend_color_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BLEND_LOAD_BINARY, &args![this, stream]);
    let color = this.addr() + COLOR_VALUE;
    e.call(COLOR_LOAD_BINARY, &args![color, stream]);
    if e.call(COLOR_COMPARE, &args![color, INVALID_COLOR]).bool() {
        e.mem.set_u8(this.addr() + VALID_VALUE, 1);
    }
}

// Translated from 00a564a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::IsEqual` (Xbox PDB): `NiBlendInterpolator`'s
/// `IsEqual`, then true when the color comparison (`004a4c90`) of the two
/// colors answers false.
pub fn ni_blend_color_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BLEND_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    !e.call(
        COLOR_COMPARE,
        &args![this.addr() + COLOR_VALUE, other.addr() + COLOR_VALUE],
    )
    .bool()
}

// Translated from 00a564d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendColorInterpolator::GetViewerStrings` (Xbox PDB):
/// `NiBlendInterpolator`'s strings, then the class name and the color's
/// string named `"m_kColorValue"` (both through the argument's stack word).
pub fn ni_blend_color_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    use crate::units::nianimation::niblendinterpolator::append_viewer_string;
    e.call(BLEND_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.mem.u32(NI_BLEND_COLOR_INTERPOLATOR_RTTI);
        let text = e.call(VIEWER_STRING, &args![class_name]).u32();
        append_viewer_string(e, strings, slot, text);
        let text = e
            .call(
                COLOR_VIEWER_STRING,
                &args![this.addr() + COLOR_VALUE, COLOR_VALUE_NAME],
            )
            .u32();
        append_viewer_string(e, strings, slot, text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a56060, ni_blend_color_interpolator_get_rtti() -> Ptr),
        entry!(0x00a56070, ni_blend_color_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a5_5f80,
            ni_blend_color_interpolator_ni_blend_color_interpolator(Ptr, u32, f32, u32) -> Ptr
        ),
        entry!(
            0x00a5_5fd0,
            ni_blend_color_interpolator_interpolator_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(0x00a5_5ff0, fn_00a55ff0() -> Ptr),
        entry!(
            0x00a5_6040,
            ni_blend_color_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_6090,
            ni_blend_color_interpolator_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a5_6110,
            ni_blend_color_interpolator_store_single_value(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_61c0,
            ni_blend_color_interpolator_blend_values(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_6400,
            ni_blend_color_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_6460,
            ni_blend_color_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_64a0,
            ni_blend_color_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_64d0,
            ni_blend_color_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_color_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a56060, &args![]).u32(),
            NI_BLEND_COLOR_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_blend_color_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a3_7170, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a56070, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a56070, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x44]]);
        assert_eq!(calls_to(&e, 0x00a3_7170), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::{calls, viewer_strings};

    const CHILD_VTABLE: u32 = 0x7400_0000;
    fn slot(offset: u32) -> u32 {
        0x7500_0000 + offset
    }

    /// The "no color" value (-1, -1, -1, -1), `INVALID_FLOAT`, and a child
    /// vtable whose color `Update` writes the child's four floats (+8) to the
    /// color unless the child's word +0x18 is 0, answering that word.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.map(0x0109_6000, 0x1000);
        for i in 0..4 {
            e.mem.set_f32(INVALID_COLOR + 4 * i, -1.0);
        }
        e.mem.set_f32(INVALID_TIME, f32::MIN);
        let slots: Vec<u32> = (0..0x40).map(|i| slot(4 * i)).collect();
        e.put_vtable(CHILD_VTABLE, &slots);
        e.register(slot(SLOT_IS_COLOR_VALUE_SUPPORTED), |_, _| 1u32.into_ret());
        e.register(slot(SLOT_UPDATE_COLOR), |e, a| {
            let ok = e.mem.u32(a[0] + 0x18);
            if ok != 0 {
                for i in 0..4 {
                    let v = e.mem.u32(a[0] + 8 + 4 * i);
                    e.mem.set_u32(a[3] + 4 * i, v);
                }
            }
            ok.into_ret()
        });
        e
    }

    fn child(e: &mut Engine, color: [f32; 4], ok: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, CHILD_VTABLE);
        for (i, c) in color.iter().enumerate() {
            e.mem.set_f32(object + 8 + 4 * i as u32, *c);
        }
        e.mem.set_u32(object + 0x18, ok);
        object
    }

    fn blend(e: &mut Engine, size: u8) -> Ptr {
        let this = e.mem.alloc(OBJECT_SIZE);
        let array = e.mem.alloc(ITEM_SIZE * u32::from(size).max(1));
        e.mem.set_u32(this + INTERP_ARRAY, array);
        e.mem.set_u8(this + ARRAY_SIZE, size);
        Ptr::new(this)
    }

    fn put(e: &mut Engine, this: Ptr, index: u32, interpolator: u32, weight: f32) {
        let at = e.mem.u32(this.addr() + INTERP_ARRAY) + index * ITEM_SIZE;
        e.mem.set_u32(at, interpolator);
        e.mem.set_f32(at + ITEM_NORMALIZED_WEIGHT, weight);
    }

    fn color(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|i| e.mem.f32(at + 4 * i as u32))
    }

    #[test]
    fn constructor_and_factory_start_without_a_value() {
        let mut e = engine();
        e.register(BLEND_CONSTRUCT, |_, _| Ret::default());
        e.register(BLEND_CONSTRUCT_DEFAULT, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_u8(this + VALID_VALUE, 1);
        e.call_log = Some(vec![]);
        let words = args![this, 1u32, 0x7f80_0001u32, 3u32];
        assert_eq!(e.call(0x00a5_5f80, &words).u32(), this);
        assert_eq!(
            calls(&e, BLEND_CONSTRUCT),
            vec![vec![this, 1, 0x7fc0_0001, 3]]
        );
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(color(&e, this + COLOR_VALUE), [-1.0; 4]);
        assert_eq!(e.mem.u8(this + VALID_VALUE), 0);
        let object = e.call(0x00a5_5ff0, &args![]).u32();
        assert_eq!(e.mem.u32(object), VTABLE);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a5_5ff0, &args![]).u32(), 0);
    }

    #[test]
    fn interpolator_is_correct_type_asks_for_color_support() {
        let mut e = engine();
        let interp = child(&mut e, [0.0; 4], 1);
        assert!(e.call(0x00a5_5fd0, &args![0u32, interp]).bool());
    }

    #[test]
    fn save_and_load_binary_handle_the_color() {
        let mut e = engine();
        for f in [
            BLEND_SAVE_BINARY,
            BLEND_LOAD_BINARY,
            COLOR_SAVE_BINARY,
            COLOR_LOAD_BINARY,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(COLOR_COMPARE, |_, _| 1u32.into_ret());
        let this = blend(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_6040, &args![this, 0x900u32]);
        assert_eq!(
            calls(&e, COLOR_SAVE_BINARY),
            vec![vec![this.addr() + 0x30, 0x900]]
        );
        e.call(0x00a5_6460, &args![this, 0x900u32]);
        assert_eq!(
            calls(&e, COLOR_LOAD_BINARY),
            vec![vec![this.addr() + 0x30, 0x900]]
        );
        assert_eq!(
            calls(&e, COLOR_COMPARE),
            vec![vec![this.addr() + 0x30, INVALID_COLOR]]
        );
        assert_eq!(e.mem.u8(this.addr() + VALID_VALUE), 1);
        e.register(COLOR_COMPARE, |_, _| 0u32.into_ret());
        e.mem.set_u8(this.addr() + VALID_VALUE, 0);
        e.call(0x00a5_6460, &args![this, 0x900u32]);
        assert_eq!(e.mem.u8(this.addr() + VALID_VALUE), 0);
    }

    #[test]
    fn create_clone_copies_the_color_and_flag() {
        let mut e = engine();
        e.register(BLEND_CONSTRUCT_DEFAULT, |_, _| Ret::default());
        e.register(BLEND_COPY_MEMBERS, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = blend(&mut e, 0);
        e.mem.set_f32(this.addr() + COLOR_VALUE + 8, 0.5);
        e.mem.set_u8(this.addr() + VALID_VALUE, 1);
        let clone = e.call(0x00a5_6090, &args![this, 0u32]).u32();
        assert_eq!(e.mem.f32(clone + COLOR_VALUE + 8), 0.5);
        assert_eq!(e.mem.u8(clone + VALID_VALUE), 1);
    }

    #[test]
    fn store_single_value_copies_or_resets_the_color() {
        let mut e = engine();
        let this = blend(&mut e, 0);
        let single = child(&mut e, [0.25, 0.5, 0.75, 1.0], 1);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, single);
        let value = e.mem.alloc(0x10);
        assert!(e
            .call(0x00a5_6110, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(color(&e, value), [0.25, 0.5, 0.75, 1.0]);
        assert_eq!(color(&e, this.addr() + COLOR_VALUE), [0.25, 0.5, 0.75, 1.0]);
        e.mem.set_u32(single + 0x18, 0);
        assert!(!e
            .call(0x00a5_6110, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(color(&e, value), [-1.0; 4]);
        assert_eq!(color(&e, this.addr() + COLOR_VALUE), [-1.0; 4]);
        let untouched = e.mem.alloc(0x10);
        assert!(!e
            .call(0x00a5_6110, &args![this, f32::MIN, 0u32, untouched])
            .bool());
        assert_eq!(color(&e, untouched), [0.0; 4]);
    }

    #[test]
    fn blend_values_weights_the_updated_colors_and_divides_by_the_rest() {
        let mut e = engine();
        let this = blend(&mut e, 3);
        let a = child(&mut e, [1.0, 2.0, 4.0, 8.0], 1);
        let b = child(&mut e, [9.0; 4], 0);
        put(&mut e, this, 0, a, 0.5);
        put(&mut e, this, 1, b, 0.5);
        put(&mut e, this, 2, a, 0.0);
        let value = e.mem.alloc(0x10);
        assert!(e
            .call(0x00a5_61c0, &args![this, 1.0f32, 0u32, value])
            .bool());
        // (0.5 * colour) / (1 - 0.5): the failed item's weight is taken off.
        assert_eq!(color(&e, value), [1.0, 2.0, 4.0, 8.0]);
        assert_eq!(e.mem.u8(this.addr() + VALID_VALUE), 1);
        e.mem.set_u32(a + 0x18, 0);
        assert!(!e
            .call(0x00a5_61c0, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(color(&e, value), [-1.0; 4]);
        assert_eq!(e.mem.u8(this.addr() + VALID_VALUE), 0);
    }

    #[test]
    fn update_dispatches_by_count_and_stores_the_time() {
        let mut e = engine();
        e.register(COMPUTE_NORMALIZED_WEIGHTS, |_, _| Ret::default());
        let this = blend(&mut e, 1);
        let single = child(&mut e, [0.5; 4], 1);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, single);
        let value = e.mem.alloc(0x10);
        e.mem.set_u8(this.addr() + INTERP_COUNT, 1);
        assert!(e
            .call(0x00a5_6400, &args![this, 2.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(this.addr() + LAST_TIME), 2.0);
        e.mem.set_u8(this.addr() + INTERP_COUNT, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00a5_6400, &args![this, 3.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(this.addr() + LAST_TIME), 3.0);
        assert!(calls(&e, COMPUTE_NORMALIZED_WEIGHTS).is_empty());
        put(&mut e, this, 0, single, 1.0);
        e.mem.set_u8(this.addr() + INTERP_COUNT, 2);
        assert!(e
            .call(0x00a5_6400, &args![this, 4.0f32, 0u32, value])
            .bool());
        assert_eq!(
            calls(&e, COMPUTE_NORMALIZED_WEIGHTS),
            vec![vec![this.addr()]]
        );
        assert_eq!(color(&e, value), [0.5; 4]);
    }

    #[test]
    fn is_equal_negates_the_color_comparison() {
        let mut e = engine();
        e.register(BLEND_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(COLOR_COMPARE, |_, _| 0u32.into_ret());
        assert!(e.call(0x00a5_64a0, &args![0x100u32, 0x200u32]).bool());
        e.register(COLOR_COMPARE, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00a5_64a0, &args![0x100u32, 0x200u32]).bool());
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_the_color() {
        let mut e = engine();
        e.register(BLEND_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(COLOR_VIEWER_STRING, |_, a| a[1].into_ret());
        let (strings, stored) = viewer_strings(&mut e);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(NI_BLEND_COLOR_INTERPOLATOR_RTTI, 0x0109_a000);
        let this = blend(&mut e, 0);
        e.call(0x00a5_64d0, &args![this, strings]);
        assert_eq!(*stored.borrow(), vec![0x0109_a000, COLOR_VALUE_NAME]);
    }
}
