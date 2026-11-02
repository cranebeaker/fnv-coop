//! `NiAnimation/niblendpoint3interpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendPoint3Interpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_POINT3_INTERPOLATOR_RTTI: u32 = 0x011f_406c;

// Translated from 00a55af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_point3_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_POINT3_INTERPOLATOR_RTTI)
}

// Translated from 00a55b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a37170`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x40 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_blend_point3_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_7170;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x40u32]);
    }
    this
}

/// `NiBlendPoint3Interpolator`'s vtable (RTTI
/// `.?AVNiBlendPoint3Interpolator@@`).
const NI_BLEND_POINT3_INTERPOLATOR_VTABLE: u32 = 0x0109_a49c;
/// The point the interpolators store as "no value" (three floats at
/// `011a9424`).
const NO_POINT: u32 = 0x011a_9424;
/// The point the blend starts from (three floats at `011f426c`).
const BLEND_START: u32 = 0x011f_426c;
/// `-FLT_MAX` (`0109696c`): the time that means "no value".
const NO_TIME: u32 = 0x0109_696c;
/// `00aa13e0` (cdecl, byte count): `NiNew`'s allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiBlendInterpolator` constructor (`00a36890`, thiscall).
const BLEND_CONSTRUCT: u32 = 0x00a3_6890;
/// The `NiBlendInterpolator` copy of the members (`00a37510`, thiscall:
/// clone, cloning process).
const BLEND_COPY_MEMBERS: u32 = 0x00a3_7510;
/// `NiBlendInterpolator::LoadBinary` (`00a37980`, thiscall, stream).
const BLEND_LOAD_BINARY: u32 = 0x00a3_7980;
/// `NiBlendInterpolator::ComputeNormalizedWeights` (`00a37260`, thiscall).
const COMPUTE_NORMALIZED_WEIGHTS: u32 = 0x00a3_7260;
/// `00a59be0` (`NiColor::LoadBinary` in the engine map, thiscall, stream):
/// reads three floats.
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;

/// Copies the three words of a point.
fn copy_point(e: &mut Engine, dest: u32, source: u32) {
    for lane in 0..3 {
        let word = e.mem.u32(source + 4 * lane);
        e.mem.set_u32(dest + 4 * lane, word);
    }
}

/// The member values of a new interpolator after its base constructor: the
/// vtable, the "no value" point (+0x30), the "has value" byte (+0x3C) 0.
fn init_members(e: &mut Engine, object: u32) {
    e.mem.set_u32(object, NI_BLEND_POINT3_INTERPOLATOR_VTABLE);
    copy_point(e, object + 0x30, NO_POINT);
    e.mem.set_u8(object + 0x3c, 0);
}

// Translated from 00a55a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator`'s create function (no function in Ghidra;
/// read from the bytes): allocates 0x40 bytes, runs the
/// `NiBlendInterpolator` constructor and sets the members; null when the
/// allocation fails.
pub fn fn_00a55a80(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x40u32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(BLEND_CONSTRUCT, &args![object]);
    init_members(e, object);
    Ptr::new(object)
}

// Translated from 00a55e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::LoadBinary` (Xbox PDB name in the engine
/// map): the base's `LoadBinary`, the point (+0x30), and the "has value"
/// byte (+0x3C) set when the point is not the "no value" point.
pub fn ni_blend_point3_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let t = this.addr();
    e.call(BLEND_LOAD_BINARY, &args![this, stream]);
    e.call(POINT_LOAD_BINARY, &args![t + 0x30, stream]);
    let same = (0..3).all(|lane| {
        x87_load32(e.mem.u32(t + 0x30 + 4 * lane)) == x87_load32(e.mem.u32(NO_POINT + 4 * lane))
    });
    if !same {
        e.mem.set_u8(t + 0x3c, 1);
    }
}

// Translated from 00a55b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::StoreSingleValue` (Xbox PDB name in the
/// engine map): the single interpolator's value. The time is the item's
/// own (+0x20) when flag 1 of the byte at +0xC is set; the "no value" time
/// answers false at once. Otherwise the single interpolator (+0x18, vtable
/// +0x94: time, target, this + 0x30) fills the point, which is copied to
/// `value` (true); when it fails the point becomes the "no value" point,
/// copied to `value` (false).
pub fn ni_blend_point3_interpolator_store_single_value(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: u32,
    value: Ptr,
) -> bool {
    let t = this.addr();
    let mut time_bits = time.to_bits();
    if e.mem.u8(t + 0xc) & 1 != 0 {
        time_bits = x87_quiet_bits(e.mem.u32(t + 0x20));
    }
    let loaded = x87_load32(time_bits);
    if x87_load32(e.global::<u32>(NO_TIME)) == loaded {
        return false;
    }
    let single = e.mem.u32(t + 0x18);
    let point = t + 0x30;
    if e.vcall(single, 0x94, &args![x87_store32(loaded), target, point])
        .bool()
    {
        copy_point(e, value.addr(), point);
        return true;
    }
    copy_point(e, point, NO_POINT);
    copy_point(e, value.addr(), point);
    false
}

// Translated from 00a55c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::BlendValues` (Xbox PDB name in the engine
/// map): blends the values of the items (array at +0x14, 0x18 bytes each:
/// interpolator +0, normalized weight +8, own time +0x14; count byte at
/// +0xD) into the point at +0x30, starting from the point at `011f426c`.
/// Items with an interpolator and a weight above 0 count; each one whose
/// time (its own when flag 1 of +0xC is set) is the "no value" time, or
/// whose interpolator (vtable +0x94) gives nothing, takes its weight off the
/// total (1.0); the others add weight * value. With any value the point is
/// divided by the remaining total and copied to `value` (true); without,
/// the point becomes the "no value" point, copied to `value` (false). The
/// x87 operations run in the exe's order and operand order.
pub fn ni_blend_point3_interpolator_blend_values(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: u32,
    value: Ptr,
) -> bool {
    let t = this.addr();
    let mut total = x87_store32(1.0);
    e.mem.set_u8(t + 0x3c, 0);
    copy_point(e, t + 0x30, BLEND_START);
    let time_bits = x87_quiet_bits(time.to_bits());
    let no_time = x87_load32(e.global::<u32>(NO_TIME));
    let mut index = 0u8;
    if e.mem.u8(t + 0xd) > index {
        e.with_stack(0xc, |e, sample| {
            let s = sample.addr();
            loop {
                let offset = u32::from(index) * 0x18;
                let item = e.mem.u32(t + 0x14).wrapping_add(offset);
                let interpolator = e.mem.u32(item);
                if interpolator != 0 && 0.0 < x87_m32(e.mem.u32(item + 8)) {
                    let mut item_time = time_bits;
                    // A zero weight is never seen here (it was just found
                    // above 0); the exe tests it again all the same.
                    let zero_weight = x87_m32(e.mem.u32(item + 8)) == 0.0;
                    if !zero_weight && e.mem.u8(t + 0xc) & 1 != 0 {
                        item_time = x87_quiet_bits(e.mem.u32(item + 0x14));
                    }
                    let subtract = zero_weight
                        || no_time == x87_load32(item_time)
                        || !e
                            .vcall(interpolator, 0x94, &args![item_time, target, s])
                            .bool();
                    if subtract {
                        let item = e.mem.u32(t + 0x14).wrapping_add(offset);
                        let weight = x87_m32(e.mem.u32(item + 8));
                        total = x87_store32(x87_sub(x87_load32(total), weight));
                    } else {
                        let item = e.mem.u32(t + 0x14).wrapping_add(offset);
                        let weight_bits = x87_store32(x87_load32(e.mem.u32(item + 8)));
                        let weight = x87_load32(weight_bits);
                        let part0 = x87_store32(x87_mul(x87_load32(e.mem.u32(s)), weight));
                        let part1 = x87_store32(x87_mul(x87_load32(e.mem.u32(s + 4)), weight));
                        let part2 = x87_store32(x87_mul(weight, x87_m32(e.mem.u32(s + 8))));
                        let x = x87_add(x87_load32(e.mem.u32(t + 0x30)), x87_m32(part0));
                        e.mem.set_u32(t + 0x30, x87_store32(x));
                        let y = x87_add(x87_load32(part1), x87_m32(e.mem.u32(t + 0x34)));
                        e.mem.set_u32(t + 0x34, x87_store32(y));
                        let z = x87_add(x87_load32(e.mem.u32(t + 0x38)), x87_m32(part2));
                        e.mem.set_u32(t + 0x38, x87_store32(z));
                        e.mem.set_u8(t + 0x3c, 1);
                    }
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u8(t + 0xd) {
                    break;
                }
            }
        });
    }
    if e.mem.u8(t + 0x3c) == 0 {
        copy_point(e, t + 0x30, NO_POINT);
        copy_point(e, value.addr(), t + 0x30);
        return false;
    }
    let inverse = x87_load32(x87_store32(x87_div(1.0, x87_load32(total))));
    let x = x87_mul(x87_load32(e.mem.u32(t + 0x30)), inverse);
    e.mem.set_u32(t + 0x30, x87_store32(x));
    let y = x87_mul(x87_load32(e.mem.u32(t + 0x34)), inverse);
    e.mem.set_u32(t + 0x34, x87_store32(y));
    let z = x87_mul(inverse, x87_m32(e.mem.u32(t + 0x38)));
    e.mem.set_u32(t + 0x38, x87_store32(z));
    copy_point(e, value.addr(), t + 0x30);
    true
}

// Translated from 00a55e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::Update` (Xbox PDB name in the engine map):
/// with one interpolator (byte at +0xE is 1) `StoreSingleValue`, with more
/// `ComputeNormalizedWeights` and `BlendValues`, with none false; the time
/// is stored as the last time (+8) in every case.
pub fn ni_blend_point3_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: u32,
    value: Ptr,
) -> bool {
    let t = this.addr();
    let count = e.mem.u8(t + 0xe);
    let time_bits = x87_quiet_bits(time.to_bits());
    let result = if count == 1 {
        ni_blend_point3_interpolator_store_single_value(
            e,
            this,
            f32::from_bits(time_bits),
            target,
            value,
        )
    } else if count > 0 {
        e.call(COMPUTE_NORMALIZED_WEIGHTS, &args![this]);
        ni_blend_point3_interpolator_blend_values(e, this, f32::from_bits(time_bits), target, value)
    } else {
        false
    };
    e.mem.set_u32(t + 8, time_bits);
    result
}

// Translated from 00a55b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::CreateClone` (Xbox PDB name in the engine
/// map): a new interpolator (as `00a55a80` makes it), the base's copy of the
/// members (also run for a null clone, as the exe does), then the point and
/// the "has value" byte.
pub fn ni_blend_point3_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> u32 {
    let t = this.addr();
    let object = e.call(NI_ALLOCATE, &args![0x40u32]).u32();
    if object != 0 {
        e.call(BLEND_CONSTRUCT, &args![object]);
        init_members(e, object);
    }
    e.call(BLEND_COPY_MEMBERS, &args![this, object, cloning]);
    copy_point(e, object.wrapping_add(0x30), t + 0x30);
    let has_value = e.mem.u8(t + 0x3c);
    e.mem.set_u8(object.wrapping_add(0x3c), has_value);
    object
}

/// `NiInterpolator::IsPoint3ValueSupported` (vtable +0xB0, Xbox PDB slot
/// order).
const SLOT_IS_POINT3_VALUE_SUPPORTED: u32 = 0xb0;
/// `NiBlendInterpolator::IsEqual` (`00a36fd0`), `SaveBinary` (`00a37ec0`)
/// and `GetViewerStrings` (`00a38080`).
const BLEND_IS_EQUAL: u32 = 0x00a3_6fd0;
const BLEND_SAVE_BINARY: u32 = 0x00a3_7ec0;
const BLEND_GET_VIEWER_STRINGS: u32 = 0x00a3_8080;
/// `NiPoint3::SaveBinary` (`00a69780`, `thiscall (point, stream)`), the
/// point comparison `IsEqual` negates (`00439090`, `thiscall (point,
/// &other)`) and `NiPoint3`'s viewer string (`00a59af0`, `thiscall (point,
/// name)`, answering the string).
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
const POINT_COMPARE: u32 = 0x0043_9090;
const POINT_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the class name) and the name
/// `"m_kPoint3Value"`.
const VIEWER_STRING: u32 = 0x00a7_d330;
const POINT_VALUE_NAME: u32 = 0x0109_8924;
/// `NiBlendPoint3Interpolator::m_kPoint3Value` (Xbox PDB) +0x30.
const POINT_VALUE: u32 = 0x30;

// Translated from 00a55a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::InterpolatorIsCorrectType` (Xbox PDB;
/// `thiscall`, one stack word): the interpolator's `IsPoint3ValueSupported`
/// (vtable +0xB0); `this` is not read.
pub fn ni_blend_point3_interpolator_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: Ptr,
) -> bool {
    e.vcall(
        interpolator.addr(),
        SLOT_IS_POINT3_VALUE_SUPPORTED,
        &args![],
    )
    .bool()
}

// Translated from 00a55ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::SaveBinary` (Xbox PDB):
/// `NiBlendInterpolator`'s `SaveBinary`, then the point value
/// (`NiPoint3::SaveBinary`).
pub fn ni_blend_point3_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BLEND_SAVE_BINARY, &args![this, stream]);
    e.call(POINT_SAVE_BINARY, &args![this.addr() + POINT_VALUE, stream]);
}

// Translated from 00a55ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::IsEqual` (Xbox PDB): `NiBlendInterpolator`'s
/// `IsEqual`, then true when the point comparison (`00439090`) of the two
/// point values answers false.
pub fn ni_blend_point3_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BLEND_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    !e.call(
        POINT_COMPARE,
        &args![this.addr() + POINT_VALUE, other.addr() + POINT_VALUE],
    )
    .bool()
}

// Translated from 00a55ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendPoint3Interpolator::GetViewerStrings` (Xbox PDB):
/// `NiBlendInterpolator`'s strings, then the class name and the point
/// value's string named `"m_kPoint3Value"` (both through the argument's
/// stack word).
pub fn ni_blend_point3_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    use crate::units::nianimation::niblendinterpolator::append_viewer_string;
    e.call(BLEND_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.mem.u32(NI_BLEND_POINT3_INTERPOLATOR_RTTI);
        let text = e.call(VIEWER_STRING, &args![class_name]).u32();
        append_viewer_string(e, strings, slot, text);
        let text = e
            .call(
                POINT_VIEWER_STRING,
                &args![this.addr() + POINT_VALUE, POINT_VALUE_NAME],
            )
            .u32();
        append_viewer_string(e, strings, slot, text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a55af0, ni_blend_point3_interpolator_get_rtti() -> Ptr),
        entry!(0x00a55b00, ni_blend_point3_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a5_5a80, fn_00a55a80() -> Ptr),
        entry!(
            0x00a5_5e60,
            ni_blend_point3_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a5_5b90, ni_blend_point3_interpolator_store_single_value(Ptr, f32, u32, Ptr) -> bool),
        entry!(0x00a5_5c30, ni_blend_point3_interpolator_blend_values(Ptr, f32, u32, Ptr) -> bool),
        entry!(0x00a5_5e00, ni_blend_point3_interpolator_update(Ptr, f32, u32, Ptr) -> bool),
        entry!(0x00a5_5b20, ni_blend_point3_interpolator_create_clone(Ptr, Ptr) -> u32),
        entry!(
            0x00a5_5a60,
            ni_blend_point3_interpolator_interpolator_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_5ad0,
            ni_blend_point3_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_5ec0,
            ni_blend_point3_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_5ef0,
            ni_blend_point3_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_point3_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a55af0, &args![]).u32(),
            NI_BLEND_POINT3_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_blend_point3_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00a55b00, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a55b00, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x40]]);
        assert_eq!(calls_to(&e, 0x00a3_7170), vec![vec![this]; 2]);
    }

    fn points(e: &mut Engine) {
        e.map(NO_POINT, 0xc);
        e.map(BLEND_START, 0xc);
        e.map(NO_TIME, 4);
        for lane in 0..3 {
            e.mem.set_f32(NO_POINT + 4 * lane, -3.0e38);
        }
        e.mem.set_u32(NO_TIME, 0xff7f_ffff);
    }

    /// An interpolator whose vtable +0x94 writes the point at its +8 and
    /// answers its byte at +0x14.
    fn source(e: &mut Engine, value: [f32; 3], ok: bool) -> u32 {
        e.put_vtable(0x9100_0000, &[0; 0x26]);
        e.mem.set_u32(0x9100_0094, 0x7000_0094);
        e.register(0x7000_0094, |e, a| {
            let words = e.mem.bytes(a[0] + 8, 0xc);
            e.mem.write(a[3], &words);
            Ret {
                eax: e.mem.u8(a[0] + 0x14) as u32,
                ..Ret::default()
            }
        });
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, 0x9100_0000);
        for (lane, v) in value.iter().enumerate() {
            e.mem.set_f32(object + 8 + 4 * lane as u32, *v);
        }
        e.mem.set_u8(object + 0x14, ok as u8);
        object
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    #[test]
    fn fn_00a55a80_creates_an_interpolator() {
        let mut e = Engine::new();
        points(&mut e);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(BLEND_CONSTRUCT, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_5a80, &[]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xaa; 0x40]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let object = e.call(0x00a5_5a80, &[]).u32();
        assert_eq!(e.mem.u32(object), NI_BLEND_POINT3_INTERPOLATOR_VTABLE);
        assert_eq!(point(&e, object + 0x30), [-3.0e38; 3]);
        assert_eq!(e.mem.u8(object + 0x3c), 0);
    }

    #[test]
    fn load_binary_marks_a_real_point() {
        let mut e = Engine::new();
        points(&mut e);
        e.register(BLEND_LOAD_BINARY, |_, _| Ret::default());
        e.register(POINT_LOAD_BINARY, |e, a| {
            e.mem.set_f32(a[0], -3.0e38);
            e.mem.set_f32(a[0] + 4, -3.0e38);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x38, -3.0e38);
        e.call(0x00a5_5e60, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u8(this + 0x3c), 0);
        e.mem.set_f32(this + 0x38, 1.0);
        e.call(0x00a5_5e60, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u8(this + 0x3c), 1);
    }

    #[test]
    fn store_single_value_asks_the_single_interpolator() {
        let mut e = Engine::new();
        points(&mut e);
        let this = e.mem.alloc(0x40);
        let single = source(&mut e, [1.0, 2.0, 3.0], true);
        e.mem.set_u32(this + 0x18, single);
        let value = e.mem.alloc(0xc);
        let store = |e: &mut Engine, time: f32| {
            e.call(0x00a5_5b90, &args![this, time, 0x6000u32, value])
                .bool()
        };
        assert!(!store(&mut e, f32::from_bits(0xff7f_ffff)), "no time");
        e.call_log = Some(vec![]);
        assert!(store(&mut e, 0.5));
        assert_eq!(point(&e, value), [1.0, 2.0, 3.0]);
        let asked = e.call_log.take().unwrap()[1].clone();
        assert_eq!(
            asked,
            (
                0x7000_0094,
                vec![single, 0.5f32.to_bits(), 0x6000, this + 0x30]
            )
        );
        // Its own time (+0x20) when flag 1 is set.
        e.mem.set_u8(this + 0xc, 1);
        e.mem.set_f32(this + 0x20, 0.75);
        e.call_log = Some(vec![]);
        store(&mut e, 0.5);
        assert_eq!(e.call_log.take().unwrap()[1].1[1], 0.75f32.to_bits());
        e.mem.set_u8(single + 0x14, 0);
        assert!(!store(&mut e, 0.5));
        assert_eq!(point(&e, value), [-3.0e38; 3]);
        assert_eq!(point(&e, this + 0x30), [-3.0e38; 3]);
    }

    /// Items at +0x14 (count byte at +0xD): interpolator, weight.
    fn items(e: &mut Engine, this: u32, list: &[(u32, f32)]) {
        let array = e.mem.alloc(0x18 * list.len() as u32);
        for (i, (interp, weight)) in list.iter().enumerate() {
            let item = array + 0x18 * i as u32;
            e.mem.set_u32(item, *interp);
            e.mem.set_f32(item + 8, *weight);
        }
        e.mem.set_u32(this + 0x14, array);
        e.mem.set_u8(this + 0xd, list.len() as u8);
    }

    #[test]
    fn blend_values_weights_the_items_and_renormalizes() {
        let mut e = Engine::new();
        points(&mut e);
        let this = e.mem.alloc(0x40);
        let a = source(&mut e, [2.0, 4.0, 8.0], true);
        let b = source(&mut e, [4.0, 8.0, 16.0], true);
        let failing = source(&mut e, [100.0; 3], false);
        items(
            &mut e,
            this,
            &[(a, 0.25), (0, 0.5), (b, 0.25), (failing, 0.5), (a, 0.0)],
        );
        let value = e.mem.alloc(0xc);
        assert!(e
            .call(0x00a5_5c30, &args![this, 1.0f32, 0u32, value])
            .bool());
        // (0.25 * a + 0.25 * b) / (1 - 0.5)
        assert_eq!(point(&e, value), [3.0, 6.0, 12.0]);
        assert_eq!(e.mem.u8(this + 0x3c), 1);

        items(&mut e, this, &[(failing, 1.0)]);
        assert!(!e
            .call(0x00a5_5c30, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(point(&e, value), [-3.0e38; 3]);
        // The "no value" time skips every item.
        items(&mut e, this, &[(a, 1.0)]);
        assert!(!e
            .call(0x00a5_5c30, &args![this, 0xff7f_ffffu32, 0u32, value])
            .bool());
        e.mem.set_u8(this + 0xd, 0);
        assert!(!e
            .call(0x00a5_5c30, &args![this, 1.0f32, 0u32, value])
            .bool());
    }

    #[test]
    fn update_picks_single_blend_or_nothing_and_stores_the_time() {
        let mut e = Engine::new();
        points(&mut e);
        e.register(COMPUTE_NORMALIZED_WEIGHTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let a = source(&mut e, [2.0, 4.0, 8.0], true);
        let value = e.mem.alloc(0xc);
        assert!(!e
            .call(0x00a5_5e00, &args![this, 1.5f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(this + 8), 1.5);
        e.mem.set_u8(this + 0xe, 1);
        e.mem.set_u32(this + 0x18, a);
        assert!(e
            .call(0x00a5_5e00, &args![this, 2.5f32, 0u32, value])
            .bool());
        assert_eq!(point(&e, value), [2.0, 4.0, 8.0]);
        e.mem.set_u8(this + 0xe, 2);
        items(&mut e, this, &[(a, 0.5)]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_5e00, &args![this, 3.5f32, 0u32, value])
            .bool());
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(COMPUTE_NORMALIZED_WEIGHTS, vec![this])));
        assert_eq!(e.mem.f32(this + 8), 3.5);
    }

    #[test]
    fn create_clone_copies_the_point_and_flag() {
        let mut e = Engine::new();
        points(&mut e);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(BLEND_CONSTRUCT, |_, _| Ret::default());
        e.register(BLEND_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x34, 9.0);
        e.mem.set_u8(this + 0x3c, 1);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a5_5b20, &args![this, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_BLEND_POINT3_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(clone + 0x34), 9.0);
        assert_eq!(e.mem.u8(clone + 0x3c), 1);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(BLEND_COPY_MEMBERS, vec![this, clone, 0x6000])));
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::{calls, viewer_strings};

    const VTABLE: u32 = 0x7400_0000;
    const SUPPORTED: u32 = 0x7500_0000;

    #[test]
    fn interpolator_is_correct_type_asks_for_point3_support() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x40];
        slots[(SLOT_IS_POINT3_VALUE_SUPPORTED / 4) as usize] = SUPPORTED;
        e.put_vtable(VTABLE, &slots);
        e.register(SUPPORTED, |_, _| 1u32.into_ret());
        let interp = e.mem.alloc(8);
        e.mem.set_u32(interp, VTABLE);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a5_5a60, &args![0u32, interp]).bool());
        assert_eq!(calls(&e, SUPPORTED), vec![vec![interp]]);
    }

    #[test]
    fn save_binary_saves_the_base_then_the_point() {
        let mut e = Engine::new();
        e.register(BLEND_SAVE_BINARY, |_, _| Ret::default());
        e.register(POINT_SAVE_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a5_5ad0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls(&e, BLEND_SAVE_BINARY), vec![vec![0x1000, 0x2000]]);
        assert_eq!(calls(&e, POINT_SAVE_BINARY), vec![vec![0x1030, 0x2000]]);
    }

    #[test]
    fn is_equal_negates_the_point_comparison() {
        let mut e = Engine::new();
        e.register(BLEND_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(POINT_COMPARE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a5_5ec0, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(calls(&e, POINT_COMPARE), vec![vec![0x1030, 0x2030]]);
        e.register(POINT_COMPARE, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00a5_5ec0, &args![0x1000u32, 0x2000u32]).bool());
        e.register(BLEND_IS_EQUAL, |_, _| 0u32.into_ret());
        e.register(POINT_COMPARE, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a5_5ec0, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_the_point() {
        let mut e = Engine::new();
        e.register(BLEND_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(POINT_VIEWER_STRING, |_, a| (a[1] + 1).into_ret());
        let (strings, stored) = viewer_strings(&mut e);
        e.map(0x011f_4000, 0x1000);
        e.mem
            .set_u32(NI_BLEND_POINT3_INTERPOLATOR_RTTI, 0x0109_8000);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00a5_5ef0, &args![this, strings]);
        assert_eq!(*stored.borrow(), vec![0x0109_8000, POINT_VALUE_NAME + 1]);
        assert_eq!(
            calls(&e, POINT_VIEWER_STRING),
            vec![vec![this + 0x30, POINT_VALUE_NAME]]
        );
    }
}
