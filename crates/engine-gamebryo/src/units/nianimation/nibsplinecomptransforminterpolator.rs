//! `NiAnimation/nibsplinecomptransforminterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineCompTransformInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_COMP_TRANSFORM_INTERPOLATOR_RTTI: u32 = 0x011f_4004;

// Translated from 00a50e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompTransformInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_comp_transform_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_COMP_TRANSFORM_INTERPOLATOR_RTTI)
}

// Translated from 00a50e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompTransformInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x60 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_comp_transform_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x60u32]);
    }
    this
}

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiBSplineTransformInterpolator`'s constructor (`00a4fff0`), member copy (`00a4fec0`)
/// and `LoadBinary` (`00a50470`).
const TRANSFORM_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a4_fff0;
const TRANSFORM_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a4_fec0;
const TRANSFORM_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a5_0470;
/// `NiBSplineData::GetCompactedValueDegree3` (`00a526b0`; `thiscall` on the data: time,
/// output, dimension, basis data, handle, offset, half range).
const GET_COMPACTED_VALUE_DEGREE3: u32 = 0x00a5_26b0;
/// `NiQuaternion::FastNormalize` (`00a4ff00`; the quaternion in `ECX`).
const QUATERNION_FAST_NORMALIZE: u32 = 0x00a4_ff00;
/// The `NiBSplineCompTransformInterpolator` virtual table (`0109991c`).
const COMP_TRANSFORM_INTERPOLATOR_VTABLE: u32 = 0x0109_991c;
/// The float the six compact scalars start as (`01099908`).
const SCALAR_DEFAULT: u32 = 0x0109_9908;
/// The double the update compares the transform parts with (`010241b0`, the invalid
/// value).
const INVALID_VALUE: u32 = 0x0102_41b0;
/// The control point handle meaning "no curve".
const NO_CURVE: u32 = 0xffff;

/// The constructor `CreateClone` and the create function inline: the transform
/// interpolator's constructor with no data, no curves and no basis data, the class's
/// virtual table and the six compact scalars from the float at `01099908` (one `FLD`,
/// six `FST`s).
fn construct_in_place(e: &mut Engine, object: u32) {
    e.call(
        TRANSFORM_INTERPOLATOR_CONSTRUCTOR,
        &args![object, 0u32, NO_CURVE, NO_CURVE, NO_CURVE, 0u32],
    );
    let scalar = x87_quiet_bits(e.global::<u32>(SCALAR_DEFAULT));
    e.mem.set_u32(object, COMP_TRANSFORM_INTERPOLATOR_VTABLE);
    for offset in (0x48..0x60).step_by(4) {
        e.mem.set_u32(object + offset, scalar);
    }
}

// Translated from 00a50f00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's create function (not a Ghidra function; the bytes at `00a50f00`, plain
/// `RET`): allocates 0x60 bytes and, when that succeeds, constructs a
/// `NiBSplineCompTransformInterpolator` in them (see `construct_in_place`). Answers the
/// object, null when the allocation failed.
pub fn fn_00a50f00(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x60u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct_in_place(e, block);
    Ptr::new(block)
}

// Translated from 00a50e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompTransformInterpolator::CreateClone` (Xbox PDB; `thiscall`, `RET 4`:
/// cloning process): a new object (null when the allocation fails), the transform
/// interpolator's member copy (`00a4fec0`, from `this`), then the six compact scalars
/// (`+0x48`..`+0x5C`, x87 copies). Answers the clone.
pub fn ni_b_spline_comp_transform_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x60u32]).u32();
    if block != 0 {
        construct_in_place(e, block);
    }
    e.call(
        TRANSFORM_INTERPOLATOR_COPY_MEMBERS,
        &args![this, block, cloning],
    );
    for offset in (0x48..0x60).step_by(4) {
        let scalar = x87_quiet_bits(e.mem.u32(this.addr() + offset));
        e.mem.set_u32(block.wrapping_add(offset), scalar);
    }
    Ptr::new(block)
}

// Translated from 00a51140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompTransformInterpolator::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`:
/// stream): the transform interpolator's `LoadBinary` (`00a50470`), then the six compact
/// scalars in one read of 0x18 bytes (component size 4).
pub fn ni_b_spline_comp_transform_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TRANSFORM_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let destination = this.addr() + 0x48;
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read, &args![input, destination, 0x18u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
}

/// The validity test both ends of the update share: true unless the scale (`+0x38`), the
/// rotation word at `+0x2C` and the translation word at `+0x1C` all equal the invalid
/// double (x87 compares; a NaN is unequal).
fn transform_is_set(e: &Engine, this: u32) -> bool {
    let invalid = f64::from_bits(e.mem.u64(INVALID_VALUE));
    x87_load32(e.mem.u32(this + 0x38)) != invalid
        || x87_load32(e.mem.u32(this + 0x2c)) != invalid
        || x87_load32(e.mem.u32(this + 0x1c)) != invalid
}

/// The eight transform words (`+0x1C`..`+0x3B`) copied to `output` (`REP MOVSD`).
fn copy_transform(e: &mut Engine, this: u32, output: u32) {
    for index in 0..8 {
        let word = e.mem.u32(this + 0x1c + 4 * index);
        e.mem.set_u32(output + 4 * index, word);
    }
}

// Translated from 00a50f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompTransformInterpolator::Update` (Xbox PDB; `thiscall`, `RET 0xC`: time,
/// target (not read), output transform). For the time of the last update the stored
/// transform is copied out and the answer is whether any part is set. Otherwise the time
/// is normalized to `(time - start) / (end - start)` (stored as a float) and every part
/// with a curve (handle not `0xFFFF`) is evaluated by
/// `NiBSplineData::GetCompactedValueDegree3` (`00a526b0`; translation dimension 3, rotation
/// 4 then `FastNormalize`, scale 1; each with its offset and half range from the compact
/// scalars): translation and scale pass through the x87 stack, the rotation is copied as
/// words. The transform is copied out; when a part is set the time is kept and the answer
/// is true, else false.
pub fn ni_b_spline_comp_transform_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _target: u32,
    output: Ptr,
) -> bool {
    let object = this.addr();
    let last = x87_load32(e.mem.u32(object + 8));
    let now = x87_load32(time.to_bits());
    if now == last {
        copy_transform(e, object, output.addr());
        return transform_is_set(e, object);
    }
    let start = x87_m32(e.mem.u32(object + 0xc));
    let elapsed = x87_sub(now, start);
    let end = x87_load32(e.mem.u32(object + 0x10));
    let length = x87_sub(end, x87_m32(e.mem.u32(object + 0xc)));
    let normalized = x87_store32(x87_div(elapsed, length));
    e.with_stack(16, |e, values| {
        let parts = [(0x3c, 3u32, 0x48), (0x40, 4, 0x50), (0x44, 1, 0x58)];
        for (handle_offset, dimension, scalars) in parts {
            let handle = e.mem.u32(object + handle_offset);
            if handle == NO_CURVE {
                continue;
            }
            let half_range = x87_quiet_bits(e.mem.u32(object + scalars + 4));
            let offset = x87_quiet_bits(e.mem.u32(object + scalars));
            let data = e.mem.u32(object + 0x14);
            let basis_data = e.mem.u32(object + 0x18);
            e.call(
                GET_COMPACTED_VALUE_DEGREE3,
                &args![
                    data,
                    x87_quiet_bits(normalized),
                    values,
                    dimension,
                    basis_data,
                    handle,
                    offset,
                    half_range
                ],
            );
            match dimension {
                3 => {
                    for index in 0..3 {
                        let value = x87_quiet_bits(e.mem.u32(values.addr() + 4 * index));
                        e.mem.set_u32(object + 0x1c + 4 * index, value);
                    }
                }
                4 => {
                    for index in 0..4 {
                        let word = e.mem.u32(values.addr() + 4 * index);
                        e.mem.set_u32(object + 0x28 + 4 * index, word);
                    }
                    e.call(QUATERNION_FAST_NORMALIZE, &args![object + 0x28]);
                }
                _ => {
                    let value = x87_quiet_bits(e.mem.u32(values.addr()));
                    e.mem.set_u32(object + 0x38, value);
                }
            }
        }
    });
    copy_transform(e, object, output.addr());
    if !transform_is_set(e, object) {
        return false;
    }
    e.mem.set_u32(object + 8, x87_quiet_bits(time.to_bits()));
    true
}

/// `NiBSplineTransformInterpolator::IsEqual` (`00a501b0`), `SaveBinary`
/// (`00a503e0`) and `GetViewerStrings` (`00a50500`); engine map names,
/// `thiscall`, one word each.
const TRANSFORM_INTERPOLATOR_IS_EQUAL: u32 = 0x00a5_01b0;
const TRANSFORM_INTERPOLATOR_SAVE_BINARY: u32 = 0x00a5_03e0;
const TRANSFORM_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a5_0500;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`); the viewer-string
/// array's grow (`0096ad30`, `thiscall (array; new capacity)`) and store
/// (`0096ae90`, `thiscall (array; index, &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `m_afCompScalars` (Xbox PDB): six floats from +0x48.
const COMP_SCALARS: u32 = 0x48;

// Translated from 00a50e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompTransformInterpolator::IsEqual` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the other object): the transform
/// interpolator's `IsEqual` (`00a501b0`) and all six compact scalars equal
/// (`FUCOMPP`, in order; a NaN is unequal).
pub fn ni_b_spline_comp_transform_interpolator_is_equal(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    if !e
        .call(TRANSFORM_INTERPOLATOR_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    for i in 0..6 {
        let ours = x87_load32(e.mem.u32(this.addr() + COMP_SCALARS + 4 * i));
        let theirs = x87_load32(e.mem.u32(other.addr() + COMP_SCALARS + 4 * i));
        if theirs != ours {
            return false;
        }
    }
    true
}

// Translated from 00a51180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompTransformInterpolator::SaveBinary` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: the stream): the transform
/// interpolator's `SaveBinary` (`00a503e0`), then the six compact scalars
/// in one write of 0x18 bytes (component size 4) through the stream
/// writer (+0x250, its function at +0xC), the answer added to the writer's
/// +4.
pub fn ni_b_spline_comp_transform_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TRANSFORM_INTERPOLATOR_SAVE_BINARY, &args![this, stream]);
    let output = e.mem.u32(stream.addr() + 0x250);
    let write = e.mem.u32(output + 0xc);
    let source = this.addr() + COMP_SCALARS;
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(write, &args![output, source, 0x18u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(output + 4).wrapping_add(answer);
    e.mem.set_u32(output + 4, total);
}

// Translated from 00a511c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompTransformInterpolator::GetViewerStrings` (Xbox PDB,
/// matched by the engine map; `thiscall`, one word: the strings array): the
/// transform interpolator's strings (`00a50500`), then the class name (the
/// first word of the `NiRTTI` record) appended: when the 16-bit size (+0xA)
/// has reached the capacity (+0x8) the array grows to the step (+0xE) plus
/// the size first, then the string is stored at the old size.
pub fn ni_b_spline_comp_transform_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    e.call(
        TRANSFORM_INTERPOLATOR_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    let class_name: u32 = e.global(NI_B_SPLINE_COMP_TRANSFORM_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_0e00,
            ni_b_spline_comp_transform_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_1180,
            ni_b_spline_comp_transform_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_11c0,
            ni_b_spline_comp_transform_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a50e40, ni_b_spline_comp_transform_interpolator_get_rtti() -> Ptr),
        entry!(0x00a50e50, ni_b_spline_comp_transform_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a5_0f00, fn_00a50f00() -> Ptr),
        entry!(0x00a5_0e70, ni_b_spline_comp_transform_interpolator_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a5_1140,
            ni_b_spline_comp_transform_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a5_0f60, ni_b_spline_comp_transform_interpolator_update(Ptr, f32, u32, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_comp_transform_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a50e40, &args![]).u32(),
            NI_B_SPLINE_COMP_TRANSFORM_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_comp_transform_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_7f80, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a50e50, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a50e50, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x60]]);
        assert_eq!(calls_to(&e, 0x00a5_7f80), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(TRANSFORM_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(TRANSFORM_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        e.register(TRANSFORM_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(QUATERNION_FAST_NORMALIZE, |_, _| Ret::default());
        e.map(SCALAR_DEFAULT, 4);
        e.set_global(SCALAR_DEFAULT, f32::MAX.to_bits());
        e.map(INVALID_VALUE, 8);
        e.mem.set_u64(INVALID_VALUE, f64::from(f32::MIN).to_bits());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_function_builds_the_interpolator() {
        let mut e = engine();
        let block = e.mem.alloc(0x60);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        assert_eq!(e.call(0x00a5_0f00, &args![]).u32(), block);
        assert_eq!(
            calls(&e, TRANSFORM_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![block, 0, 0xffff, 0xffff, 0xffff, 0]]
        );
        assert_eq!(e.mem.u32(block), COMP_TRANSFORM_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.f32(block + 0x48), f32::MAX);
        assert_eq!(e.mem.f32(block + 0x5c), f32::MAX);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_0f00, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_the_compact_scalars() {
        let mut e = engine();
        let block = e.mem.alloc(0x60);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        let this = e.mem.alloc(0x60);
        for (index, offset) in (0x48..0x60).step_by(4).enumerate() {
            e.mem.set_u32(this + offset, (index as f32).to_bits());
        }
        assert_eq!(e.call(0x00a5_0e70, &args![this, 0x700u32]).u32(), block);
        assert_eq!(
            calls(&e, TRANSFORM_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, block, 0x700]]
        );
        for (index, offset) in (0x48..0x60).step_by(4).enumerate() {
            assert_eq!(e.mem.f32(block + offset), index as f32);
        }
    }

    #[test]
    fn load_binary_reads_the_six_scalars_at_once() {
        let mut e = engine();
        const READ: u32 = 0x0300_0000;
        e.register(READ, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (0x18, 4, 1));
            a[2].into_ret()
        });
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x60);
        e.call(0x00a5_1140, &args![this, stream]);
        assert_eq!(
            calls(&e, TRANSFORM_INTERPOLATOR_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls(&e, READ)[0][1], this + 0x48);
        assert_eq!(e.mem.u32(input + 4), 0x18);
    }

    fn interpolator(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 8, 0.25f32.to_bits());
        e.mem.set_u32(this + 0xc, 1.0f32.to_bits());
        e.mem.set_u32(this + 0x10, 3.0f32.to_bits());
        e.mem.set_u32(this + 0x14, 0xd000);
        e.mem.set_u32(this + 0x18, 0xb000);
        for (index, offset) in (0x48..0x60).step_by(4).enumerate() {
            e.mem.set_u32(this + offset, (index as f32 + 0.5).to_bits());
        }
        for offset in (0x1c..0x3c).step_by(4) {
            e.mem.set_u32(this + offset, f32::MIN.to_bits());
        }
        this
    }

    #[test]
    fn update_evaluates_every_curve_and_keeps_the_time() {
        let mut e = engine();
        // The data double writes dimension values 10 * dimension + index.
        e.register(GET_COMPACTED_VALUE_DEGREE3, |e, a| {
            for index in 0..a[3] {
                let value = (10 * a[3] + index) as f32;
                e.mem.set_u32(a[2] + 4 * index, value.to_bits());
            }
            Ret::default()
        });
        let this = interpolator(&mut e);
        e.mem.set_u32(this + 0x3c, 1);
        e.mem.set_u32(this + 0x40, 2);
        e.mem.set_u32(this + 0x44, 3);
        let output = e.mem.alloc(0x20);
        let set = e
            .call(0x00a5_0f60, &args![this, 2.0f32.to_bits(), 0u32, output])
            .bool();
        assert!(set);
        let evaluations = calls(&e, GET_COMPACTED_VALUE_DEGREE3);
        assert_eq!(evaluations.len(), 3);
        let half = 0.5f32.to_bits();
        assert_eq!(
            evaluations[0][..2],
            [0xd000, half],
            "data and normalized time"
        );
        assert_eq!(
            evaluations[0][3..],
            [3, 0xb000, 1, 0.5f32.to_bits(), 1.5f32.to_bits()]
        );
        assert_eq!(
            evaluations[1][3..],
            [4, 0xb000, 2, 2.5f32.to_bits(), 3.5f32.to_bits()]
        );
        assert_eq!(
            evaluations[2][3..],
            [1, 0xb000, 3, 4.5f32.to_bits(), 5.5f32.to_bits()]
        );
        assert_eq!(
            calls(&e, QUATERNION_FAST_NORMALIZE),
            vec![vec![this + 0x28]]
        );
        let expected = [30.0f32, 31.0, 32.0, 40.0, 41.0, 42.0, 43.0, 10.0];
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(e.mem.f32(output + 4 * index as u32), *value);
        }
        assert_eq!(e.mem.f32(this + 8), 2.0);
    }

    #[test]
    fn update_without_curves_answers_false_and_keeps_the_old_time() {
        let mut e = engine();
        let this = interpolator(&mut e);
        for offset in [0x3c, 0x40, 0x44] {
            e.mem.set_u32(this + offset, NO_CURVE);
        }
        let output = e.mem.alloc(0x20);
        let set = e
            .call(0x00a5_0f60, &args![this, 2.0f32.to_bits(), 0u32, output])
            .bool();
        assert!(!set);
        assert!(calls(&e, GET_COMPACTED_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(this + 8), 0.25);
        assert_eq!(e.mem.f32(output), f32::MIN);
    }

    #[test]
    fn update_at_the_last_time_copies_the_stored_transform() {
        let mut e = engine();
        let this = interpolator(&mut e);
        e.mem.set_u32(this + 0x1c, 7.0f32.to_bits());
        let output = e.mem.alloc(0x20);
        let set = e
            .call(0x00a5_0f60, &args![this, 0.25f32.to_bits(), 0u32, output])
            .bool();
        assert!(set);
        assert_eq!(e.mem.f32(output), 7.0);
        e.mem.set_u32(this + 0x1c, f32::MIN.to_bits());
        let set = e
            .call(0x00a5_0f60, &args![this, 0.25f32.to_bits(), 0u32, output])
            .bool();
        assert!(!set);
        assert!(calls(&e, GET_COMPACTED_VALUE_DEGREE3).is_empty());
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

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

    #[test]
    fn is_equal_compares_the_scalars_after_the_base() {
        let mut e = Engine::new();
        e.register(TRANSFORM_INTERPOLATOR_IS_EQUAL, |_, _| word(1));
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        assert!(e.call(0x00a5_0e00, &args![this, other]).bool());
        assert_eq!(
            calls(&e, TRANSFORM_INTERPOLATOR_IS_EQUAL),
            vec![vec![this, other]]
        );
        for i in 0..6u32 {
            let at = COMP_SCALARS + 4 * i;
            e.mem.set_f32(other + at, 1.0);
            assert!(!e.call(0x00a5_0e00, &args![this, other]).bool());
            e.mem.set_f32(other + at, 0.0);
        }
        e.register(TRANSFORM_INTERPOLATOR_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_0e00, &args![this, other]).bool());
    }

    #[test]
    fn save_binary_writes_the_scalars_at_once() {
        const WRITE: u32 = 0x0300_0800;
        let mut e = Engine::new();
        e.register(TRANSFORM_INTERPOLATOR_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
            word(a[2])
        });
        e.call_log = Some(vec![]);
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer + 0xc, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, writer);
        let this = e.mem.alloc(0x60);
        e.call(0x00a5_1180, &args![this, stream]);
        assert_eq!(
            calls(&e, TRANSFORM_INTERPOLATOR_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(
            calls(&e, WRITE)[0][..3],
            [writer, this + COMP_SCALARS, 4 * 6u32]
        );
        assert_eq!(e.mem.u32(writer + 4), 4 * 6u32);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        e.map(NI_B_SPLINE_COMP_TRANSFORM_INTERPOLATOR_RTTI, 4);
        e.set_global(
            NI_B_SPLINE_COMP_TRANSFORM_INTERPOLATOR_RTTI,
            0x0109_eeee_u32,
        );
        e.register(TRANSFORM_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x81));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x81);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 5);
        e.call(0x00a5_11c0, &args![0x1000u32, strings]);
        assert_eq!(
            calls(&e, TRANSFORM_INTERPOLATOR_GET_VIEWER_STRINGS),
            vec![vec![0x1000, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_eeee]]);
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 7]]);
        assert_eq!(calls(&e, STRINGS_SET_AT)[0][..2], [strings, 2]);
    }
}
