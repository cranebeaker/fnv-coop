//! `NiAnimation/nimaterialcolorcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiMaterialColorController::ms_RTTI` (the class's `NiRTTI` record).
const NI_MATERIAL_COLOR_CONTROLLER_RTTI: u32 = 0x011f_3f88;

// Translated from 00a4b570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_material_color_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_MATERIAL_COLOR_CONTROLLER_RTTI)
}

// Translated from 00a4b880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a57d10`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f3f88`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_material_color_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_7d10;
    const CLASS_NAME_POINTER: u32 = 0x011f_3f88;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
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

layout! {
    /// `NiMaterialColorController` (Xbox PDB), 0x3C bytes on PC (the same).
    pub struct NiMaterialColorController: 0x3c {
        /// `m_uFlags` (Xbox PDB, from `NiTimeController`).
        0x08 m_uFlagsTime: u16,
        /// `m_fScaledTime` (Xbox PDB, from `NiTimeController`).
        0x28 m_fScaledTime: f32,
        /// `m_pkTarget` (Xbox PDB, from `NiTimeController`): the material.
        0x2C m_pkTarget: Ptr,
        /// `m_spInterpolator` (Xbox PDB, from `NiInterpController`).
        0x34 m_spInterpolator: Ptr,
        /// `m_uFlags` (Xbox PDB): the color the controller drives in bits
        /// 0-2.
        0x38 m_uFlags: u16,
    }
}

/// `NiMaterialColorController`'s vtable (RTTI
/// `.?AVNiMaterialColorController@@`).
const NI_MATERIAL_COLOR_CONTROLLER_VTABLE: u32 = 0x0109_8a14;
/// `00aa13e0` (cdecl, byte count): `NiNew`'s allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `operator delete` with a size (`00aa1460`, cdecl: pointer, size).
const NI_DELETE: u32 = 0x00aa_1460;
/// The `NiPoint3InterpController` constructor (`00a57c80`, thiscall).
const POINT3_INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a5_7c80;
/// The base destructor (`00a6d290`, thiscall).
const BASE_DESTRUCT: u32 = 0x00a6_d290;
/// The base `CopyMembers` (`00a57620`, thiscall: clone, cloning process).
const BASE_COPY_MEMBERS: u32 = 0x00a5_7620;
/// `NiFloatInterpController::LoadBinary` (`00a57c70`, thiscall, stream).
const BASE_LOAD_BINARY: u32 = 0x00a5_7c70;
/// The base `LinkObject` (`00a57630`, thiscall, stream).
const BASE_LINK_OBJECT: u32 = 0x00a5_7630;
/// `00a645b0` (thiscall on the stream).
const STREAM_SKIP_LINK: u32 = 0x00a6_45b0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, thiscall).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// The `NiPoint3Interpolator` constructor from data (`00a495f0`, thiscall,
/// data).
const POINT3_INTERPOLATOR_CONSTRUCT: u32 = 0x00a4_95f0;
/// `NiTimeController::DontDoUpdate` (`00a36250`, thiscall, float time).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The float the scaled time is set to when the controller is forced
/// (`01096324`).
const FORCED_TIME: u32 = 0x0109_6324;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiStream`'s file version (+0xD8) and input file (+0x24C).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;

/// Releases a reference to `object` (non-null): `DeleteThis` (vtable +4)
/// when the count at +4 drops to zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// The member values of a new controller after its base constructor: the
/// class vtable (stored after the flags in the exe's order) and flags 0.
fn init_members(e: &mut Engine, object: u32) {
    e.mem.set_u16(object + 0x38, 0);
    e.mem.set_u32(object, NI_MATERIAL_COLOR_CONTROLLER_VTABLE);
}

/// Clamps a color channel as the exe's x87 compares do: below 0 gives 0,
/// above 1 gives 1, anything else (NaN included) is kept.
fn clamp_channel(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

// Translated from 00a4b790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController`'s create function (no function in Ghidra;
/// read from the bytes): allocates 0x3C bytes, runs the
/// `NiPoint3InterpController` constructor, clears the flags and stores the
/// vtable; null when the allocation fails.
pub fn fn_00a4b790(e: &mut Engine) -> Ptr<NiMaterialColorController> {
    let object = e.call(NI_ALLOCATE, &args![0x3cu32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(POINT3_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    init_members(e, object);
    Ptr::new(object)
}

// Translated from 00a4b7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController::LoadBinary` (Xbox PDB name in the engine
/// map): the base's `LoadBinary`; files before version 0x0A000102 take the
/// flags from bits 5-12 of the stream's word at +0x286, newer ones read
/// them (2 bytes; the stream argument's stack word is the byte-count word);
/// files before 0x0A010068 then run `00a645b0` on the stream.
pub fn ni_material_color_controller_load_binary(
    e: &mut Engine,
    this: Ptr<NiMaterialColorController>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(s + STREAM_VERSION) < 0x0a00_0102 {
        let flags = (e.mem.u16(s + 0x286) >> 5) & 0xff;
        e.set(this, NiMaterialColorController::m_uFlags, flags);
    } else {
        e.with_stack(4, |e, count| {
            let input = e.mem.u32(s + STREAM_INPUT);
            let read = e.mem.u32(input + 8);
            e.mem.set_u32(count.addr(), 2);
            let destination = this.addr() + NiMaterialColorController::m_uFlags.off;
            let done = e
                .call(read, &args![input, destination, 2u32, count, 1u32])
                .u32();
            let position = e.mem.u32(input + 4).wrapping_add(done);
            e.mem.set_u32(input + 4, position);
        });
    }
    if e.mem.u32(s + STREAM_VERSION) < 0x0a01_0068 {
        e.call(STREAM_SKIP_LINK, &args![stream]);
    }
}

// Translated from 00a4b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController::LinkObject` (Xbox PDB name in the engine
/// map): the base's `LinkObject`; for files before version 0x0A010068 makes
/// an `NiPoint3Interpolator` (0x20 bytes, `00a495f0`) on the data of the
/// stream's next link, makes it the interpolator (`NiPointer` assignment)
/// and calls its vtable +0xBC.
pub fn ni_material_color_controller_link_object(
    e: &mut Engine,
    this: Ptr<NiMaterialColorController>,
    stream: Ptr,
) {
    let t = this.addr();
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= 0x0a01_0068 {
        return;
    }
    let data = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let block = e.call(NI_ALLOCATE, &args![0x20u32]).u32();
    let interpolator = if block != 0 {
        e.call(POINT3_INTERPOLATOR_CONSTRUCT, &args![block, data])
            .u32()
    } else {
        0
    };
    let old = e.mem.u32(t + 0x34);
    if old != interpolator {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(t + 0x34, interpolator);
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator + 4]);
        }
    }
    let current = e.mem.u32(t + 0x34);
    e.vcall(current, 0xbc, &[]);
}

// Translated from 00a4b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController::GetCtlrID` (Xbox PDB name in the engine
/// map): the name of the driven color by flag bits 0-2: "AMB", "DIFF",
/// "SPEC", "SELF_ILLUM" (the jump table's four cases), null otherwise.
pub fn ni_material_color_controller_get_ctlr_id(
    e: &mut Engine,
    this: Ptr<NiMaterialColorController>,
) -> u32 {
    match e.get(this, NiMaterialColorController::m_uFlags) & 7 {
        0 => 0x0109_8a0c,
        1 => 0x0109_8a04,
        2 => 0x0109_89fc,
        3 => 0x0109_89f0,
        _ => 0,
    }
}

// Translated from 00a4b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController::Update` (Xbox PDB name in the engine map):
/// a forced controller (flag bit 5 of the byte at +8) sets its scaled time
/// to the float at `01096324`; otherwise, when `DontDoUpdate` (with the
/// update's time) says so, it stops unless the interpolator's vtable +0xD4
/// answers true. Then the interpolator (vtable +0x94: scaled time, target,
/// out color) gives a color; each channel is clamped to 0..1 and, for the
/// specular (2) or emissive (3) color, stored in the material (+0x1C or
/// +0x28) with its revision word (+0x44) incremented. The ambient and
/// diffuse cases store nothing.
pub fn ni_material_color_controller_update(
    e: &mut Engine,
    this: Ptr<NiMaterialColorController>,
    update: Ptr,
) {
    let t = this.addr();
    if (e.mem.u8(t + 8) >> 5) & 1 != 0 {
        let forced = x87_quiet_bits(e.global::<u32>(FORCED_TIME));
        e.mem.set_u32(t + 0x28, forced);
    } else {
        let time = x87_quiet_bits(e.mem.u32(update.addr()));
        if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
            let interpolator = e.mem.u32(t + 0x34);
            if interpolator == 0 || !e.vcall(interpolator, 0xd4, &[]).bool() {
                return;
            }
        }
    }
    let interpolator = e.mem.u32(t + 0x34);
    if interpolator == 0 {
        return;
    }
    let color = e.with_stack(0xc, |e, out| {
        let time = x87_quiet_bits(e.mem.u32(t + 0x28));
        let target = e.mem.u32(t + 0x2c);
        let ok = e
            .vcall(interpolator, 0x94, &args![time, target, out])
            .bool();
        ok.then(|| {
            [0u32, 4, 8].map(|lane| f32::from_bits(x87_quiet_bits(e.mem.u32(out.addr() + lane))))
        })
    });
    let Some(color) = color else {
        return;
    };
    let target = e.mem.u32(t + 0x2c);
    let clamped = color.map(|channel| clamp_channel(channel).to_bits());
    let at = match e.get(this, NiMaterialColorController::m_uFlags) & 7 {
        2 => 0x1c,
        3 => 0x28,
        _ => return,
    };
    let revision = e.mem.u32(target + 0x44).wrapping_add(1);
    e.mem.set_u32(target + 0x44, revision);
    for (lane, bits) in clamped.iter().enumerate() {
        e.mem.set_u32(target + at + 4 * lane as u32, *bits);
    }
}

// Translated from 00a4b740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController::CreateClone` (Xbox PDB name in the engine
/// map): a new controller (as `00a4b790` makes it), the base's
/// `CopyMembers` (also run for a null clone, as the exe does) and the
/// flags.
pub fn ni_material_color_controller_create_clone(
    e: &mut Engine,
    this: Ptr<NiMaterialColorController>,
    cloning: Ptr,
) -> u32 {
    let object = e.call(NI_ALLOCATE, &args![0x3cu32]).u32();
    if object != 0 {
        e.call(POINT3_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
        e.mem.set_u32(object, NI_MATERIAL_COLOR_CONTROLLER_VTABLE);
        e.mem.set_u16(object + 0x38, 0);
    }
    e.call(BASE_COPY_MEMBERS, &args![this, object, cloning]);
    let flags = e.get(this, NiMaterialColorController::m_uFlags);
    e.mem.set_u16(object.wrapping_add(0x38), flags);
    object
}

// Translated from 00a4b6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController`'s scalar deleting destructor (Xbox PDB name
/// in the engine map): stores the vtable, releases the interpolator, runs
/// the base destructor (`00a6d290`) and frees the 0x3C bytes when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_material_color_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiMaterialColorController>,
    flags: u32,
) -> Ptr<NiMaterialColorController> {
    e.mem
        .set_u32(this.addr(), NI_MATERIAL_COLOR_CONTROLLER_VTABLE);
    let interpolator = e.mem.u32(this.addr() + 0x34);
    if interpolator != 0 {
        release(e, interpolator);
    }
    e.call(BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x3cu32]);
    }
    this
}

/// The color the ambient and diffuse cases of `GetTargetPoint3Value` give (`011f4998`,
/// three floats).
const BLACK_COLOR: u32 = 0x011f_4998;
/// The base `IsEqual` (`00a57c10`, thiscall, the other controller).
const BASE_IS_EQUAL: u32 = 0x00a5_7c10;

// Translated from 00a4b3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController::GetTargetPoint3Value` (Xbox PDB name in the engine map): by
/// flag bits 0-2 (the jump table's four cases), the color at `011f4998` for the ambient (0)
/// and diffuse (1) colors, the material's specular color (+0x1c) for 2, and for 3 the color
/// the material's pointer at +0x34 points to, or its emissive color (+0x28) when that is
/// null; 0.0 for the other values. Copied into `value` through the x87 stack.
pub fn ni_material_color_controller_get_target_point3_value(
    e: &mut Engine,
    this: Ptr<NiMaterialColorController>,
    value: Ptr,
) {
    let target = e.mem.u32(this.addr().wrapping_add(0x2c));
    let source = match e.get(this, NiMaterialColorController::m_uFlags) & 7 {
        0 | 1 => Some(BLACK_COLOR),
        2 => Some(target.wrapping_add(0x1c)),
        3 => {
            let pointer = e.mem.u32(target.wrapping_add(0x34));
            Some(if pointer == 0 {
                target.wrapping_add(0x28)
            } else {
                pointer
            })
        }
        _ => None,
    };
    let lanes = match source {
        Some(source) => [0u32, 4, 8].map(|lane| e.mem.u32(source.wrapping_add(lane))),
        None => [0; 3],
    };
    for (k, bits) in lanes.iter().enumerate() {
        let at = value.addr().wrapping_add(4 * k as u32);
        e.mem.set_u32(at, x87_quiet_bits(*bits));
    }
}

// Translated from 00a4b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialColorController::IsEqual` (Xbox PDB name in the engine map): the base's
/// `IsEqual` (`00a57c10`; its false is returned as is) and the same flag bits 0-2.
pub fn ni_material_color_controller_is_equal(
    e: &mut Engine,
    this: Ptr<NiMaterialColorController>,
    other: Ptr,
) -> u8 {
    let base = e.call(BASE_IS_EQUAL, &args![this, other]).u8();
    if base == 0 {
        return base;
    }
    let theirs = e.mem.u8(other.addr().wrapping_add(0x38));
    let ours = e.mem.u8(this.addr().wrapping_add(0x38));
    u8::from((theirs ^ ours) & 7 == 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4b570, ni_material_color_controller_get_rtti() -> Ptr),
        entry!(
            0x00a4b880,
            ni_material_color_controller_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a4_b790, fn_00a4b790() -> Ptr<NiMaterialColorController>),
        entry!(
            0x00a4_b3c0,
            ni_material_color_controller_get_target_point3_value(
                Ptr<NiMaterialColorController>,
                Ptr,
            )
        ),
        entry!(
            0x00a4_b540,
            ni_material_color_controller_is_equal(Ptr<NiMaterialColorController>, Ptr) -> u8
        ),
        entry!(
            0x00a4_b7c0,
            ni_material_color_controller_load_binary(Ptr<NiMaterialColorController>, Ptr)
        ),
        entry!(
            0x00a4_b4b0,
            ni_material_color_controller_link_object(Ptr<NiMaterialColorController>, Ptr)
        ),
        entry!(
            0x00a4_b470,
            ni_material_color_controller_get_ctlr_id(Ptr<NiMaterialColorController>) -> u32
        ),
        entry!(
            0x00a4_b580,
            ni_material_color_controller_update(Ptr<NiMaterialColorController>, Ptr)
        ),
        entry!(
            0x00a4_b740,
            ni_material_color_controller_create_clone(Ptr<NiMaterialColorController>, Ptr) -> u32
        ),
        entry!(
            0x00a4_b6f0,
            ni_material_color_controller_scalar_deleting_destructor(
                Ptr<NiMaterialColorController>,
                u32,
            ) -> Ptr<
                NiMaterialColorController,
            >
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_material_color_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4b570, &args![]).u32(),
            NI_MATERIAL_COLOR_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_material_color_controller_get_viewer_strings_appends_the_class_name_growing_a_full_array()
    {
        const PARENT: u32 = 0x00a5_7d10;
        const NAME_POINTER: u32 = 0x011f_3f88;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            ni_material_color_controller_get_viewer_strings(&mut e, this, Ptr::new(strings));
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
    }

    fn refcount_doubles(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
    }

    /// An interpolator object (vtable `0x9100_0000`): +4 its count; vtable
    /// +4 `DeleteThis`, +0x94 writes the color at its +0x10 and answers its
    /// byte at +0x1c, +0xBC and +0xD4 answer its byte at +0x1d.
    fn interpolator(e: &mut Engine, count: u32) -> u32 {
        e.put_vtable(0x9100_0000, &[0; 0x36]);
        for (slot, target) in [
            (4u32, 0x7000_0004u32),
            (0x94, 0x7000_0094),
            (0xbc, 0x7000_00bc),
            (0xd4, 0x7000_00d4),
        ] {
            e.mem.set_u32(0x9100_0000 + slot, target);
        }
        e.register(0x7000_0004, |_, _| Ret::default());
        e.register(0x7000_0094, |e, a| {
            let words = e.mem.bytes(a[0] + 0x10, 0xc);
            e.mem.write(a[3], &words);
            Ret {
                eax: e.mem.u8(a[0] + 0x1c) as u32,
                ..Ret::default()
            }
        });
        e.register(0x7000_00bc, |_, _| Ret::default());
        e.register(0x7000_00d4, |e, a| Ret {
            eax: e.mem.u8(a[0] + 0x1d) as u32,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, 0x9100_0000);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn fn_00a4b790_creates_a_controller() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(POINT3_INTERP_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_b790, &[]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xaa; 0x3c]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let object = e.call(0x00a4_b790, &[]).u32();
        assert_eq!(e.mem.u32(object), NI_MATERIAL_COLOR_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u16(object + 0x38), 0);
    }

    #[test]
    fn load_binary_takes_old_flags_from_the_stream_or_reads_them() {
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_SKIP_LINK, |_, _| Ret::default());
        e.register(0x7000_0008, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 2);
            e.mem.set_u16(a[1], 0x0003);
            Ret {
                eax: 2,
                ..Ret::default()
            }
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, 0x7000_0008);
        let stream = e.mem.alloc(0x290);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u16(stream + 0x286, 0xffff);
        e.mem.set_u32(stream + 0xd8, 0x0a00_0101);
        let this = e.mem.alloc(0x3c);
        e.call_log = Some(vec![]);
        e.call(0x00a4_b7c0, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x38), 0xff);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(STREAM_SKIP_LINK, vec![stream])));
        e.mem.set_u32(stream + 0xd8, 0x0a01_0068);
        e.call_log = Some(vec![]);
        e.call(0x00a4_b7c0, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x38), 3);
        assert_eq!(e.mem.u32(input + 4), 2);
        assert!(!e
            .call_log
            .take()
            .unwrap()
            .iter()
            .any(|c| c.0 == STREAM_SKIP_LINK));
    }

    #[test]
    fn link_object_makes_an_interpolator_for_old_files() {
        let mut e = Engine::new();
        refcount_doubles(&mut e);
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| Ret {
            eax: 0x5500,
            ..Ret::default()
        });
        let made = interpolator(&mut e, 0);
        e.register_double(NI_ALLOCATE, move |_, _| Ret {
            eax: made,
            ..Ret::default()
        });
        e.register(POINT3_INTERPOLATOR_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0xd8, 0x0a01_0068);
        let this = e.mem.alloc(0x3c);
        e.call_log = Some(vec![]);
        e.call(0x00a4_b4b0, &args![this, stream]);
        assert_eq!(e.call_log.take().unwrap().len(), 2, "new files: base only");
        e.mem.set_u32(stream + 0xd8, 0x0a01_0067);
        let old = interpolator(&mut e, 1);
        e.mem.set_u32(this + 0x34, old);
        e.call_log = Some(vec![]);
        e.call(0x00a4_b4b0, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(POINT3_INTERPOLATOR_CONSTRUCT, vec![made, 0x5500])));
        assert!(log.contains(&(0x7000_0004, vec![old])), "old one deleted");
        assert!(log.contains(&(0x7000_00bc, vec![made])));
        assert_eq!(e.mem.u32(this + 0x34), made);
        assert_eq!(e.mem.u32(made + 4), 1);
    }

    #[test]
    fn get_ctlr_id_names_the_color() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x3c);
        for (flags, name) in [
            (0u16, 0x0109_8a0cu32),
            (9, 0x0109_8a04),
            (2, 0x0109_89fc),
            (3, 0x0109_89f0),
            (4, 0),
            (7, 0),
        ] {
            e.mem.set_u16(this + 0x38, flags);
            assert_eq!(e.call(0x00a4_b470, &args![this]).u32(), name);
        }
    }

    #[test]
    fn update_clamps_and_stores_the_specular_or_emissive_color() {
        let mut e = Engine::new();
        e.register(DONT_DO_UPDATE, |e, a| Ret {
            eax: e.mem.u8(a[0] + 0x3a) as u32,
            ..Ret::default()
        });
        e.map(FORCED_TIME, 4);
        e.set_global(FORCED_TIME, 0xff7f_ffffu32);
        let interp = interpolator(&mut e, 1);
        e.mem.set_f32(interp + 0x10, -0.5);
        e.mem.set_f32(interp + 0x14, 0.25);
        e.mem.set_f32(interp + 0x18, 2.0);
        e.mem.set_u8(interp + 0x1c, 1);
        let this = e.mem.alloc(0x3c);
        let material = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x2c, material);
        e.mem.set_u32(this + 0x34, interp);
        e.mem.set_f32(this + 0x28, 0.5);
        let update = e.mem.alloc(4);
        e.mem.set_f32(update, 3.0);
        e.mem.set_u16(this + 0x38, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a4_b580, &args![this, update]);
        assert_eq!(e.mem.f32(material + 0x1c), 0.0);
        assert_eq!(e.mem.f32(material + 0x20), 0.25);
        assert_eq!(e.mem.f32(material + 0x24), 1.0);
        assert_eq!(e.mem.u32(material + 0x44), 1);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(DONT_DO_UPDATE, vec![this, 3.0f32.to_bits()])));
        let asked: Vec<_> = log.iter().filter(|c| c.0 == 0x7000_0094).collect();
        assert_eq!(
            asked[0].1[..3].to_vec(),
            vec![interp, 0.5f32.to_bits(), material]
        );

        e.mem.set_u16(this + 0x38, 3);
        e.mem.set_u8(this + 8, 0x20);
        e.call(0x00a4_b580, &args![this, update]);
        assert_eq!(e.mem.u32(this + 0x28), 0xff7f_ffff, "forced time");
        assert_eq!(e.mem.f32(material + 0x2c), 0.25);
        assert_eq!(e.mem.u32(material + 0x44), 2);

        e.mem.set_u16(this + 0x38, 1);
        e.call(0x00a4_b580, &args![this, update]);
        assert_eq!(e.mem.u32(material + 0x44), 2, "diffuse stores nothing");

        // DontDoUpdate says skip and the interpolator does not insist.
        e.mem.set_u8(this + 8, 0);
        e.mem.set_u8(this + 0x3a, 1);
        e.mem.set_u16(this + 0x38, 2);
        e.call(0x00a4_b580, &args![this, update]);
        assert_eq!(e.mem.u32(material + 0x44), 2);
        e.mem.set_u8(interp + 0x1d, 1);
        e.call(0x00a4_b580, &args![this, update]);
        assert_eq!(e.mem.u32(material + 0x44), 3);
        // The interpolator gives no color.
        e.mem.set_u8(interp + 0x1c, 0);
        e.call(0x00a4_b580, &args![this, update]);
        assert_eq!(e.mem.u32(material + 0x44), 3);
    }

    #[test]
    fn clamp_channel_keeps_nan_and_signed_zero() {
        assert!(clamp_channel(f32::NAN).is_nan());
        assert_eq!(clamp_channel(-0.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(clamp_channel(1.0), 1.0);
    }

    #[test]
    fn create_clone_builds_and_copies_the_flags() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(POINT3_INTERP_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x3c);
        e.mem.set_u16(this + 0x38, 3);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a4_b740, &args![this, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_MATERIAL_COLOR_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u16(clone + 0x38), 3);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(BASE_COPY_MEMBERS, vec![this, clone, 0x6000])));
    }

    #[test]
    fn scalar_deleting_destructor_releases_and_frees_on_bit_0() {
        let mut e = Engine::new();
        refcount_doubles(&mut e);
        e.register(BASE_DESTRUCT, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        let interp = interpolator(&mut e, 1);
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this + 0x34, interp);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_b6f0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_MATERIAL_COLOR_CONTROLLER_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x7000_0004, vec![interp])));
        assert!(log.contains(&(BASE_DESTRUCT, vec![this])));
        assert!(log.contains(&(NI_DELETE, vec![this, 0x3c])));
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn get_target_point3_value_picks_the_driven_color() {
        let mut e = Engine::new();
        e.map(BLACK_COLOR, 0xc);
        e.mem.set_f32(BLACK_COLOR, 0.5);
        let material = e.mem.alloc(0x48);
        for k in 0..3 {
            e.mem.set_f32(material + 0x1c + 4 * k, 2.0 + k as f32);
            e.mem.set_f32(material + 0x28 + 4 * k, 6.0 + k as f32);
        }
        let this = Ptr::<NiMaterialColorController>::new(e.mem.alloc(0x3c));
        e.mem.set_u32(this.addr() + 0x2c, material);
        let value = e.mem.alloc(0xc);
        let read = |e: &mut Engine, flags: u16| -> [f32; 3] {
            e.set(this, NiMaterialColorController::m_uFlags, flags);
            e.call(0x00a4_b3c0, &args![this, value]);
            [e.mem.f32(value), e.mem.f32(value + 4), e.mem.f32(value + 8)]
        };
        assert_eq!(read(&mut e, 0), [0.5, 0.0, 0.0]);
        assert_eq!(read(&mut e, 9), [0.5, 0.0, 0.0]);
        assert_eq!(read(&mut e, 2), [2.0, 3.0, 4.0]);
        assert_eq!(read(&mut e, 3), [6.0, 7.0, 8.0]);
        assert_eq!(read(&mut e, 4), [0.0, 0.0, 0.0]);
        let elsewhere = e.mem.alloc(0xc);
        e.mem.set_u32(elsewhere, 0x7f80_0009);
        e.mem.set_u32(material + 0x34, elsewhere);
        e.set(this, NiMaterialColorController::m_uFlags, 0xfff3);
        e.call(0x00a4_b3c0, &args![this, value]);
        assert_eq!(e.mem.u32(value), 0x7fc0_0009);
    }

    #[test]
    fn is_equal_compares_flag_bits_0_to_2() {
        let mut e = Engine::new();
        let base = Rc::new(Cell::new(1u32));
        let answer = base.clone();
        e.register_double(BASE_IS_EQUAL, move |_, _| Ret {
            eax: answer.get(),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x3c);
        let other = e.mem.alloc(0x3c);
        e.mem.set_u16(this + 0x38, 0x0b);
        e.mem.set_u16(other + 0x38, 0x03);
        assert_eq!(e.call(0x00a4_b540, &args![this, other]).u8(), 1);
        e.mem.set_u16(other + 0x38, 0x02);
        assert_eq!(e.call(0x00a4_b540, &args![this, other]).u8(), 0);
        base.set(0);
        e.mem.set_u16(other + 0x38, 0x03);
        assert_eq!(e.call(0x00a4_b540, &args![this, other]).u8(), 0);
    }
}
