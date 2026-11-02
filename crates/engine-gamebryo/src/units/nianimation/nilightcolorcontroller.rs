//! `NiAnimation/nilightcolorcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLightColorController::ms_RTTI` (the class's `NiRTTI` record).
const NI_LIGHT_COLOR_CONTROLLER_RTTI: u32 = 0x011f_3fa8;

// Translated from 00a4d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_light_color_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LIGHT_COLOR_CONTROLLER_RTTI)
}

// Translated from 00a4d5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a57d10`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f3fa8`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_light_color_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_7d10;
    const CLASS_NAME_POINTER: u32 = 0x011f_3fa8;
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

/// The base constructor `00a57c80` runs first (`00a38e00`, named
/// `NiBoolInterpController::CopyMembers` in the engine map; thiscall).
const INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a3_8e00;
/// `NiPoint3InterpController`'s vtable (RTTI
/// `.?AVNiPoint3InterpController@@`).
const NI_POINT3_INTERP_CONTROLLER_VTABLE: u32 = 0x0109_ad74;

// Translated from 00a57c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiLightColorController::CopyMembers` in the engine map, but the
/// body is the `NiPoint3InterpController` constructor: the base constructor
/// (`00a38e00`), then that class's vtable. Returns `this`.
pub fn fn_00a57c80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(INTERP_CONTROLLER_CONSTRUCT, &args![this]);
    e.mem
        .set_u32(this.addr(), NI_POINT3_INTERP_CONTROLLER_VTABLE);
    this
}

/// `NiLightColorController`'s vtable (RTTI `.?AVNiLightColorController@@`, the address the
/// destructor and `CreateClone` store) and size.
const NI_LIGHT_COLOR_CONTROLLER_VTABLE: u32 = 0x0109_8dd4;
const NI_LIGHT_COLOR_CONTROLLER_SIZE: u32 = 0x3c;
/// Fields as the PC code uses them (Xbox PDB names): `NiTimeController`'s flags byte +0x08,
/// `m_fScaledTime` +0x28, `m_pkTarget` +0x2c, `NiSingleInterpController::m_spInterpolator`
/// +0x34 and this class's 16-bit flags +0x38 (bit 0: the ambient color).
const TIME_FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
const COLOR_FLAGS: u32 = 0x38;
/// The light's colors as `GetTargetPoint3Value` reads them: +0xc8 (ambient) and +0xd4
/// (diffuse), three floats each.
const LIGHT_AMBIENT: u32 = 0xc8;
const LIGHT_DIFFUSE: u32 = 0xd4;
/// The names `"Ambient"` and `"Diffuse"`.
const TEXT_AMBIENT: u32 = 0x0109_8dc4;
const TEXT_DIFFUSE: u32 = 0x0109_8dbc;
/// `NiLight::ms_RTTI` (`011f4a28`) and the color `Update` gives the other channel
/// (`011f4998`, three floats).
const NI_LIGHT_RTTI: u32 = 0x011f_4a28;
const OTHER_COLOR: u32 = 0x011f_4998;
/// `GetRTTI` (virtual slot `0x08`); an `NiRTTI` record's base-class pointer is at +4.
const SLOT_GET_RTTI: u32 = 0x08;
/// Callees: the base `RegisterStreamables` (`00a57780`), `SaveBinary` (`00a57c00`),
/// `IsEqual` (`00a57c10`), `LinkObject` (`00a57630`), `CopyMembers` (`00a57620`) and
/// `LoadBinary` (`00a57c70`), all thiscall; the engine map names some after other classes
/// (folded code).
const BASE_REGISTER_STREAMABLES: u32 = 0x00a5_7780;
const BASE_SAVE_BINARY: u32 = 0x00a5_7c00;
const BASE_IS_EQUAL: u32 = 0x00a5_7c10;
const BASE_LINK_OBJECT: u32 = 0x00a5_7630;
const BASE_COPY_MEMBERS: u32 = 0x00a5_7620;
const BASE_LOAD_BINARY: u32 = 0x00a5_7c70;
/// `NiTimeController::DontDoUpdate` (`00a36250`, thiscall, the float time) and the float a
/// forced controller's scaled time becomes (`01096324`).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
const FORCED_TIME: u32 = 0x0109_6324;
/// `00653290` (thiscall on an object, an `NiRTTI` record): the object when it is of that
/// class, else 0; and the light's color setters `004bc280` / `004bc2e0` (thiscall, the
/// color's address).
const DYNAMIC_CAST: u32 = 0x0065_3290;
const LIGHT_SET_AMBIENT: u32 = 0x004b_c280;
const LIGHT_SET_DIFFUSE: u32 = 0x004b_c2e0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`) and `00a645b0` (thiscall on the stream).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `00aa13e0` (cdecl, one word): allocates from the Ni allocator; the `NiPoint3Interpolator`
/// constructor from data (`00a495f0`, thiscall); the sized delete (`00aa1460`); the
/// `NiTimeController` destructor (`00a6d290`).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const POINT3_INTERPOLATOR_CONSTRUCT: u32 = 0x00a4_95f0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots) and `DeleteThis` (virtual
/// slot 1); the interpolator's slots `0xd4` (always update), `0x94` (point3 update) and
/// `0xbc` (collapse).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_ALWAYS_UPDATE: u32 = 0xd4;
const SLOT_UPDATE_POINT3: u32 = 0x94;
const SLOT_COLLAPSE: u32 = 0xbc;
/// `NiStream::m_uiNifFileVersion` +0xd8, `m_pkIstr` +0x24c and `m_pkOstr` +0x250 (Xbox PDB
/// names, PC offsets), the stream's 16-bit word +0x286 old files take the flags from, and the
/// binary stream's position (+4), read (+8) and write (+0xc) functions.
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const STREAM_OLD_FLAGS: u32 = 0x286;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_WRITE: u32 = 0xc;

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at +4 and, when it
/// reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// One transfer of `size` bytes in one component through the binary stream at
/// `stream + side` (input or output) with its function at `function`: the word at `slot` is
/// set to `size` and passed as the component size; the position advances by the bytes moved.
fn stream_transfer(
    e: &mut Engine,
    stream: u32,
    side: u32,
    function: u32,
    buffer: u32,
    size: u32,
    slot: u32,
) {
    let binary = e.mem.u32(stream.wrapping_add(side));
    let transfer = e.mem.u32(binary.wrapping_add(function));
    e.mem.set_u32(slot, size);
    let bytes = e
        .call(transfer, &args![binary, buffer, size, slot, 1u32])
        .u32();
    let position = e.mem.u32(binary.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        binary.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

// Translated from 00a461c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::RegisterStreamables` (Xbox PDB name in the engine map): whether
/// the base's `RegisterStreamables` (`00a57780`) succeeded.
pub fn ni_light_color_controller_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00a4b840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::SaveBinary` (Xbox PDB name in the engine map): the base's
/// `SaveBinary` (`00a57c00`), then the 16-bit flags (2 bytes; the argument's own stack word is
/// the component size word).
pub fn ni_light_color_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stream.addr());
        let flags = this.addr().wrapping_add(COLOR_FLAGS);
        let output = (STREAM_OUTPUT, BINARY_STREAM_WRITE);
        stream_transfer(e, stream.addr(), output.0, output.1, flags, 2, slot.addr());
    });
}

// Translated from 00a4d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::GetTargetPoint3Value` (Xbox PDB): with a target, its ambient
/// color (+0xc8) when flag bit 0 is set, its diffuse color (+0xd4) otherwise, copied into
/// `value` (through the x87 stack).
pub fn ni_light_color_controller_get_target_point3_value(e: &mut Engine, this: Ptr, value: Ptr) {
    let target = e.mem.u32(this.addr().wrapping_add(TARGET));
    if target == 0 {
        return;
    }
    let color = if e.mem.u8(this.addr().wrapping_add(COLOR_FLAGS)) & 1 != 0 {
        LIGHT_AMBIENT
    } else {
        LIGHT_DIFFUSE
    };
    let source = target.wrapping_add(color);
    let lanes = [0u32, 4, 8].map(|lane| e.mem.u32(source.wrapping_add(lane)));
    for (k, bits) in lanes.iter().enumerate() {
        let at = value.addr().wrapping_add(4 * k as u32);
        e.mem.set_u32(at, x87_quiet_bits(*bits));
    }
}

// Translated from 00a4d1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::GetCtlrID` (Xbox PDB): `"Ambient"` when flag bit 0 is set,
/// `"Diffuse"` otherwise.
pub fn ni_light_color_controller_get_ctlr_id(e: &mut Engine, this: Ptr) -> u32 {
    if e.mem.u8(this.addr().wrapping_add(COLOR_FLAGS)) & 1 != 0 {
        TEXT_AMBIENT
    } else {
        TEXT_DIFFUSE
    }
}

// Translated from 00a4d1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::IsEqual` (Xbox PDB): the base's `IsEqual` (`00a57c10`; its false
/// is returned as is) and the same flag bit 0.
pub fn ni_light_color_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> u8 {
    let base = e.call(BASE_IS_EQUAL, &args![this, other]).u8();
    if base == 0 {
        return base;
    }
    let theirs = e.mem.u8(other.addr().wrapping_add(COLOR_FLAGS));
    let ours = e.mem.u8(this.addr().wrapping_add(COLOR_FLAGS));
    (!theirs ^ ours) & 1
}

// Translated from 00a4d210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::InterpTargetIsCorrectType` (Xbox PDB; `this` unread): whether
/// `target` is non-null and an `NiLight` (its `GetRTTI` or a base is `011f4a28`).
pub fn ni_light_color_controller_interp_target_is_correct_type(
    e: &mut Engine,
    _unused_0: Ptr,
    target: Ptr,
) -> bool {
    if target.addr() == 0 {
        return false;
    }
    let mut record = e.vcall(target.addr(), SLOT_GET_RTTI, &args![]).u32();
    while record != 0 {
        if record == NI_LIGHT_RTTI {
            return true;
        }
        record = e.mem.u32(record.wrapping_add(4));
    }
    false
}

/// A color channel as the exe's compares clamp it: below 0 gives 0, above 1 gives 1,
/// anything else (NaN included) is kept as loaded.
fn clamp_channel(bits: u32) -> u32 {
    let value = x87_load32(bits);
    if value < 0.0 {
        0.0f32.to_bits()
    } else if value > 1.0 {
        1.0f32.to_bits()
    } else {
        x87_quiet_bits(bits)
    }
}

// Translated from 00a4d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::Update` (Xbox PDB): a forced controller (flag bit 5 of the byte
/// at +8) sets its scaled time to the float at `01096324`; otherwise, when `DontDoUpdate`
/// (with the update's time) says so, it stops unless the interpolator's slot `0xd4` answers
/// true. The interpolator's point3 update (slot `0x94`: scaled time, target, out color) gives
/// a color whose channels are clamped to 0..1; with a target, the light (`00653290` with
/// `NiLight`'s record) gets it as its ambient color (`004bc280`) and the color at `011f4998`
/// as its diffuse one (`004bc2e0`) when flag bit 0 is set, the other way round otherwise.
pub fn ni_light_color_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
    let object = this.addr();
    if (e.mem.u8(object.wrapping_add(TIME_FLAGS)) >> 5) & 1 != 0 {
        let forced = x87_quiet_bits(e.mem.u32(FORCED_TIME));
        e.mem.set_u32(object.wrapping_add(SCALED_TIME), forced);
    } else {
        let time = x87_quiet_bits(e.mem.u32(update.addr()));
        if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
            let interpolator = e.mem.u32(object.wrapping_add(INTERPOLATOR));
            if interpolator == 0 {
                return;
            }
            if !e.vcall(interpolator, SLOT_ALWAYS_UPDATE, &args![]).bool() {
                return;
            }
        }
    }
    let interpolator = e.mem.u32(object.wrapping_add(INTERPOLATOR));
    if interpolator == 0 {
        return;
    }
    // Frame: the clamped color +0, the interpolated color +0xc.
    e.with_stack(0x18, |e, frame| {
        let clamped = frame.addr();
        let color = clamped + 0xc;
        let time = x87_quiet_bits(e.mem.u32(object.wrapping_add(SCALED_TIME)));
        let target = e.mem.u32(object.wrapping_add(TARGET));
        if !e
            .vcall(
                interpolator,
                SLOT_UPDATE_POINT3,
                &args![time, target, color],
            )
            .bool()
        {
            return;
        }
        for lane in [0u32, 4, 8] {
            let bits = clamp_channel(e.mem.u32(color + lane));
            e.mem.set_u32(clamped + lane, bits);
        }
        let target = e.mem.u32(object.wrapping_add(TARGET));
        if target == 0 {
            return;
        }
        let light = e.call(DYNAMIC_CAST, &args![target, NI_LIGHT_RTTI]).u32();
        if e.mem.u8(object.wrapping_add(COLOR_FLAGS)) & 1 != 0 {
            e.call(LIGHT_SET_AMBIENT, &args![light, clamped]);
            e.call(LIGHT_SET_DIFFUSE, &args![light, OTHER_COLOR]);
        } else {
            e.call(LIGHT_SET_AMBIENT, &args![light, OTHER_COLOR]);
            e.call(LIGHT_SET_DIFFUSE, &args![light, clamped]);
        }
    });
}

// Translated from 00a4d3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::LinkObject` (Xbox PDB): the base's `LinkObject` (`00a57630`);
/// files before version `0x0a010068` make an `NiPoint3Interpolator` (0x20 bytes, `00a495f0`;
/// null when the allocation fails) on the data of the stream's next link (counted while it
/// is used), make it the interpolator (counted; the old one released) and collapse it (slot
/// `0xbc`).
pub fn ni_light_color_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    let object = this.addr();
    if e.mem.u32(stream.addr().wrapping_add(STREAM_VERSION)) >= 0x0a01_0068 {
        return;
    }
    let data = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    if data != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data.wrapping_add(4)]);
    }
    let block = e.call(NI_ALLOCATE, &args![0x20u32]).u32();
    let interpolator = if block != 0 {
        e.call(POINT3_INTERPOLATOR_CONSTRUCT, &args![block, data])
            .u32()
    } else {
        0
    };
    let old = e.mem.u32(object.wrapping_add(INTERPOLATOR));
    if old != interpolator {
        if old != 0 {
            release(e, old);
        }
        e.mem
            .set_u32(object.wrapping_add(INTERPOLATOR), interpolator);
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator.wrapping_add(4)]);
        }
    }
    let current = e.mem.u32(object.wrapping_add(INTERPOLATOR));
    e.vcall(current, SLOT_COLLAPSE, &args![]);
    if data != 0 {
        release(e, data);
    }
}

// Translated from 00a4d460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable,
/// releases the interpolator, runs the `NiTimeController` destructor (`00a6d290`) and frees
/// the 0x3C bytes when bit 0 of `flags` is set. Returns `this`.
pub fn ni_light_color_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_LIGHT_COLOR_CONTROLLER_VTABLE);
    let interpolator = e.mem.u32(this.addr().wrapping_add(INTERPOLATOR));
    if interpolator != 0 {
        release(e, interpolator);
    }
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(
            NI_SIZED_DELETE,
            &args![this, NI_LIGHT_COLOR_CONTROLLER_SIZE],
        );
    }
    this
}

// Translated from 00a4d4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::CreateClone` (Xbox PDB): a new controller (0x3C bytes; the
/// `NiPoint3InterpController` constructor `00a57c80`, the vtable, flags 0), the base's
/// `CopyMembers` (`00a57620`, also for a null clone, as the exe does) and the flags.
pub fn ni_light_color_controller_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> u32 {
    let object = e
        .call(NI_ALLOCATE, &args![NI_LIGHT_COLOR_CONTROLLER_SIZE])
        .u32();
    if object != 0 {
        fn_00a57c80(e, Ptr::new(object));
        e.mem.set_u32(object, NI_LIGHT_COLOR_CONTROLLER_VTABLE);
        e.mem.set_u16(object.wrapping_add(COLOR_FLAGS), 0);
    }
    e.call(BASE_COPY_MEMBERS, &args![this, object, cloning]);
    let flags = e.mem.u16(this.addr().wrapping_add(COLOR_FLAGS));
    e.mem.set_u16(object.wrapping_add(COLOR_FLAGS), flags);
    object
}

// Translated from 00a4d500 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::CreateObject` (registered as the `"NiLightColorController"`
/// loader at `00a2bde0`; Ghidra has no function here): allocates 0x3C bytes and, when that
/// succeeds, runs the `NiPoint3InterpController` constructor (`00a57c80`), clears the flags
/// and stores the vtable. Returns it, or 0.
pub fn ni_light_color_controller_create_object(e: &mut Engine) -> u32 {
    let object = e
        .call(NI_ALLOCATE, &args![NI_LIGHT_COLOR_CONTROLLER_SIZE])
        .u32();
    if object == 0 {
        return 0;
    }
    fn_00a57c80(e, Ptr::new(object));
    e.mem.set_u16(object.wrapping_add(COLOR_FLAGS), 0);
    e.mem.set_u32(object, NI_LIGHT_COLOR_CONTROLLER_VTABLE);
    object
}

// Translated from 00a4d530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightColorController::LoadBinary` (Xbox PDB): the base's `LoadBinary` (`00a57c70`);
/// files before version `0x0a000102` take the flags from bits 5-10 of the stream's word at
/// +0x286, newer ones read them (2 bytes; the argument's own stack word is the component
/// size word); files before `0x0a010068` then run `00a645b0` on the stream.
pub fn ni_light_color_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let object = this.addr();
    let stream = stream.addr();
    if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) < 0x0a00_0102 {
        let old = e.mem.u16(stream.wrapping_add(STREAM_OLD_FLAGS));
        e.mem
            .set_u16(object.wrapping_add(COLOR_FLAGS), (old >> 5) & 0x3f);
    } else {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), stream);
            let flags = object.wrapping_add(COLOR_FLAGS);
            let input = (STREAM_INPUT, BINARY_STREAM_READ);
            stream_transfer(e, stream, input.0, input.1, flags, 2, slot.addr());
        });
    }
    if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) < 0x0a01_0068 {
        e.call(STREAM_READ_LINK_ID, &args![stream]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4d200, ni_light_color_controller_get_rtti() -> Ptr),
        entry!(
            0x00a4d5a0,
            ni_light_color_controller_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a5_7c80, fn_00a57c80(Ptr) -> Ptr),
        entry!(
            0x00a4_61c0,
            ni_light_color_controller_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_b840, ni_light_color_controller_save_binary(Ptr, Ptr)),
        entry!(
            0x00a4_d150,
            ni_light_color_controller_get_target_point3_value(Ptr, Ptr)
        ),
        entry!(0x00a4_d1b0, ni_light_color_controller_get_ctlr_id(Ptr) -> u32),
        entry!(0x00a4_d1d0, ni_light_color_controller_is_equal(Ptr, Ptr) -> u8),
        entry!(
            0x00a4_d210,
            ni_light_color_controller_interp_target_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_d240, ni_light_color_controller_update(Ptr, Ptr)),
        entry!(0x00a4_d3a0, ni_light_color_controller_link_object(Ptr, Ptr)),
        entry!(
            0x00a4_d460,
            ni_light_color_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a4_d4b0,
            ni_light_color_controller_create_clone(Ptr, Ptr) -> u32
        ),
        entry!(0x00a4_d500, ni_light_color_controller_create_object() -> u32),
        entry!(0x00a4_d530, ni_light_color_controller_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_light_color_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4d200, &args![]).u32(),
            NI_LIGHT_COLOR_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_light_color_controller_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a5_7d10;
        const NAME_POINTER: u32 = 0x011f_3fa8;
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
            ni_light_color_controller_get_viewer_strings(&mut e, this, Ptr::new(strings));
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

    #[test]
    fn fn_00a57c80_runs_the_base_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(INTERP_CONTROLLER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00a5_7c80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_POINT3_INTERP_CONTROLLER_VTABLE);
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Transfers = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    fn answers(e: &mut Engine, address: u32, value: &Rc<Cell<u32>>) {
        let value = value.clone();
        e.register_double(address, move |_, _| ret(value.get()));
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object whose vtable has `slots` set to the given targets.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A stream whose input and output binary streams record each transfer as (buffer,
    /// size, component size) and move 2 bytes `0x34 0x12` in.
    fn stream(e: &mut Engine, transfers: &Transfers, version: u32) -> u32 {
        let record = transfers.clone();
        e.register_double(0x00ee_5000, move |e, a| {
            record.borrow_mut().push((a[1], a[2], e.mem.u32(a[3])));
            e.mem.set_u16(a[1], 0x1234);
            ret(a[2])
        });
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + BINARY_STREAM_READ, 0x00ee_5000);
        e.mem.set_u32(binary + BINARY_STREAM_WRITE, 0x00ee_5000);
        let stream = e.mem.alloc(0x290);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT, binary);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        stream
    }

    #[test]
    fn register_streamables_answers_the_base() {
        let mut e = Engine::new();
        let base = Rc::new(Cell::new(0x100u32));
        answers(&mut e, BASE_REGISTER_STREAMABLES, &base);
        assert!(!e.call(0x00a4_61c0, &args![0x1000u32, 0x2000u32]).bool());
        base.set(2);
        assert_eq!(e.call(0x00a4_61c0, &args![0x1000u32, 0x2000u32]).u32(), 1);
    }

    #[test]
    fn save_binary_writes_the_flags() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BASE_SAVE_BINARY, 0);
        let transfers = Transfers::default();
        let stream = stream(&mut e, &transfers, 0x1401_0003);
        let this = e.mem.alloc(0x3c);
        e.call(0x00a4_b840, &args![this, stream]);
        assert_eq!(calls(&log, BASE_SAVE_BINARY), [vec![this, stream]]);
        assert_eq!(*transfers.borrow(), [(this + COLOR_FLAGS, 2, 2)]);
    }

    #[test]
    fn get_target_point3_value_reads_ambient_or_diffuse() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x3c);
        let value = e.mem.alloc(0xc);
        e.call(0x00a4_d150, &args![this, value]);
        assert_eq!(e.mem.bytes(value, 0xc), vec![0; 0xc]);
        let light = e.mem.alloc(0xe0);
        for k in 0..3 {
            e.mem.set_f32(light + LIGHT_AMBIENT + 4 * k, k as f32 + 1.0);
            e.mem.set_f32(light + LIGHT_DIFFUSE + 4 * k, k as f32 + 5.0);
        }
        e.mem.set_u32(light + LIGHT_AMBIENT + 8, 0x7f80_0003);
        e.mem.set_u32(this + TARGET, light);
        e.call(0x00a4_d150, &args![this, value]);
        assert_eq!(e.mem.f32(value), 5.0);
        assert_eq!(e.mem.f32(value + 8), 7.0);
        e.mem.set_u16(this + COLOR_FLAGS, 1);
        e.call(0x00a4_d150, &args![this, value]);
        assert_eq!(e.mem.f32(value + 4), 2.0);
        assert_eq!(e.mem.u32(value + 8), 0x7fc0_0003);
    }

    #[test]
    fn get_ctlr_id_names_the_driven_color() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x3c);
        assert_eq!(e.call(0x00a4_d1b0, &args![this]).u32(), TEXT_DIFFUSE);
        e.mem.set_u16(this + COLOR_FLAGS, 3);
        assert_eq!(e.call(0x00a4_d1b0, &args![this]).u32(), TEXT_AMBIENT);
    }

    #[test]
    fn is_equal_compares_flag_bit_0() {
        let mut e = Engine::new();
        let base = Rc::new(Cell::new(1u32));
        answers(&mut e, BASE_IS_EQUAL, &base);
        let this = e.mem.alloc(0x3c);
        let other = e.mem.alloc(0x3c);
        e.mem.set_u16(this + COLOR_FLAGS, 3);
        e.mem.set_u16(other + COLOR_FLAGS, 1);
        assert_eq!(e.call(0x00a4_d1d0, &args![this, other]).u8(), 1);
        e.mem.set_u16(other + COLOR_FLAGS, 2);
        assert_eq!(e.call(0x00a4_d1d0, &args![this, other]).u8(), 0);
        base.set(0);
        e.mem.set_u16(other + COLOR_FLAGS, 1);
        assert_eq!(e.call(0x00a4_d1d0, &args![this, other]).u8(), 0);
    }

    #[test]
    fn interp_target_is_correct_type_wants_a_light() {
        let mut e = Engine::new();
        e.map(NI_LIGHT_RTTI, 8);
        let record = Rc::new(Cell::new(NI_LIGHT_RTTI));
        answers(&mut e, 0x00ee_5100, &record);
        let target = object(&mut e, 8, &[(SLOT_GET_RTTI, 0x00ee_5100)]);
        assert!(!e.call(0x00a4_d210, &args![0u32, 0u32]).bool());
        assert!(e.call(0x00a4_d210, &args![0u32, target]).bool());
        let other = e.mem.alloc(8);
        record.set(other);
        assert!(!e.call(0x00a4_d210, &args![0u32, target]).bool());
    }

    #[test]
    fn update_clamps_the_color_into_the_light() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(FORCED_TIME, 4);
        e.mem.set_f32(FORCED_TIME, 9.0);
        let skip = Rc::new(Cell::new(0u32));
        answers(&mut e, DONT_DO_UPDATE, &skip);
        let always = Rc::new(Cell::new(0u32));
        answers(&mut e, 0x00ee_5200, &always);
        let colors = Rc::new(RefCell::new(vec![]));
        let record = colors.clone();
        let updated = Rc::new(Cell::new(1u32));
        let answer = updated.clone();
        e.register_double(0x00ee_5210, move |e, a| {
            record.borrow_mut().push(a[1]);
            e.mem.set_f32(a[3], -0.5);
            e.mem.set_f32(a[3] + 4, 0.25);
            e.mem.set_f32(a[3] + 8, f32::NAN);
            ret(answer.get())
        });
        logged(&mut e, &log, DYNAMIC_CAST, 0x8000);
        let seen = Rc::new(RefCell::new(vec![]));
        for address in [LIGHT_SET_AMBIENT, LIGHT_SET_DIFFUSE] {
            let seen = seen.clone();
            e.register_double(address, move |e, a| {
                let color = [0u32, 4, 8].map(|k| e.mem.u32(a[1] + k));
                seen.borrow_mut()
                    .push((address, a[1] == OTHER_COLOR, color));
                Ret::default()
            });
        }
        e.map(OTHER_COLOR, 0xc);
        let interpolator = object(
            &mut e,
            8,
            &[
                (SLOT_ALWAYS_UPDATE, 0x00ee_5200),
                (SLOT_UPDATE_POINT3, 0x00ee_5210),
            ],
        );
        let this = e.mem.alloc(0x3c);
        let update = e.mem.alloc(8);
        e.mem.set_f32(update, 3.0);
        e.mem.set_f32(this + SCALED_TIME, 4.0);
        // DontDoUpdate says skip and the interpolator does not always update: nothing.
        skip.set(1);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.call(0x00a4_d240, &args![this, update]);
        assert!(colors.borrow().is_empty());
        // Always update: the color goes to the diffuse channel (flag bit 0 clear) without a
        // target nothing is set.
        always.set(1);
        e.call(0x00a4_d240, &args![this, update]);
        assert_eq!(*colors.borrow(), [4.0f32.to_bits()]);
        let light = 0x9000u32;
        e.mem.set_u32(this + TARGET, light);
        e.call(0x00a4_d240, &args![this, update]);
        let clamped = [0.0f32.to_bits(), 0.25f32.to_bits(), f32::NAN.to_bits()];
        assert_eq!(
            *seen.borrow(),
            [
                (LIGHT_SET_AMBIENT, true, [0; 3]),
                (LIGHT_SET_DIFFUSE, false, clamped)
            ]
        );
        assert_eq!(calls(&log, DYNAMIC_CAST), [vec![light, NI_LIGHT_RTTI]]);
        // Forced, ambient: the forced time, the color to the ambient channel.
        seen.borrow_mut().clear();
        e.mem.set_u8(this + TIME_FLAGS, 0x20);
        e.mem.set_u16(this + COLOR_FLAGS, 1);
        e.call(0x00a4_d240, &args![this, update]);
        assert_eq!(e.mem.f32(this + SCALED_TIME), 9.0);
        assert_eq!(
            *seen.borrow(),
            [
                (LIGHT_SET_AMBIENT, false, clamped),
                (LIGHT_SET_DIFFUSE, true, [0; 3])
            ]
        );
        // A failed update sets nothing; no interpolator does nothing.
        updated.set(0);
        e.call(0x00a4_d240, &args![this, update]);
        e.mem.set_u32(this + INTERPOLATOR, 0);
        e.call(0x00a4_d240, &args![this, update]);
        assert_eq!(seen.borrow().len(), 2);
    }

    #[test]
    fn clamp_channel_bounds_to_zero_and_one() {
        assert_eq!(clamp_channel(1.5f32.to_bits()), 1.0f32.to_bits());
        assert_eq!(clamp_channel((-0.0f32).to_bits()), (-0.0f32).to_bits());
        assert_eq!(clamp_channel(0x7f80_0001), 0x7fc0_0001);
    }

    /// The doubles of the interlocked import slots, counting in memory.
    fn interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
    }

    #[test]
    fn link_object_makes_the_interpolator_of_old_files() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e);
        logged(&mut e, &log, BASE_LINK_OBJECT, 0);
        logged(&mut e, &log, 0x00ee_5300, 0);
        logged(&mut e, &log, 0x00ee_5310, 0);
        let data = object(&mut e, 8, &[(SLOT_DELETE_THIS, 0x00ee_5310)]);
        logged(&mut e, &log, GET_OBJECT_FROM_LINK_ID, data);
        let made = object(&mut e, 0x20, &[(SLOT_COLLAPSE, 0x00ee_5300)]);
        e.register_double(NI_ALLOCATE, move |_, _| ret(made));
        logged(&mut e, &log, POINT3_INTERPOLATOR_CONSTRUCT, made);
        let transfers = Transfers::default();
        let stream = stream(&mut e, &transfers, 0x0a01_0068);
        let this = e.mem.alloc(0x3c);
        e.call(0x00a4_d3a0, &args![this, stream]);
        assert!(calls(&log, GET_OBJECT_FROM_LINK_ID).is_empty());
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0067);
        e.call(0x00a4_d3a0, &args![this, stream]);
        assert_eq!(calls(&log, BASE_LINK_OBJECT).len(), 2);
        assert_eq!(
            calls(&log, POINT3_INTERPOLATOR_CONSTRUCT),
            [vec![made, data]]
        );
        assert_eq!(e.mem.u32(this + INTERPOLATOR), made);
        assert_eq!(e.mem.u32(made + 4), 1);
        assert_eq!(calls(&log, 0x00ee_5300), [vec![made]]);
        // The data's temporary reference was its only one.
        assert_eq!(calls(&log, 0x00ee_5310), [vec![data]]);
    }

    #[test]
    fn scalar_deleting_destructor_releases_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e);
        logged(&mut e, &log, TIME_CONTROLLER_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        logged(&mut e, &log, 0x00ee_5400, 0);
        let interpolator = object(&mut e, 8, &[(SLOT_DELETE_THIS, 0x00ee_5400)]);
        e.mem.set_u32(interpolator + 4, 1);
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        assert_eq!(e.call(0x00a4_d460, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_LIGHT_COLOR_CONTROLLER_VTABLE);
        assert_eq!(calls(&log, 0x00ee_5400), [vec![interpolator]]);
        assert!(calls(&log, NI_SIZED_DELETE).is_empty());
        e.mem.set_u32(this + INTERPOLATOR, 0);
        e.call(0x00a4_d460, &args![this, 1u32]);
        assert_eq!(calls(&log, NI_SIZED_DELETE), [vec![this, 0x3c]]);
        assert_eq!(calls(&log, TIME_CONTROLLER_DESTRUCT).len(), 2);
    }

    #[test]
    fn create_clone_and_create_object_build_the_controller() {
        let mut e = Engine::new();
        let log = Log::default();
        e.register(INTERP_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        logged(&mut e, &log, BASE_COPY_MEMBERS, 0);
        let block = Rc::new(Cell::new(0u32));
        answers(&mut e, NI_ALLOCATE, &block);
        assert_eq!(e.call(0x00a4_d500, &args![]).u32(), 0);
        let object = e.mem.alloc(0x3c);
        e.mem.set_u16(object + COLOR_FLAGS, 7);
        block.set(object);
        assert_eq!(e.call(0x00a4_d500, &args![]).u32(), object);
        assert_eq!(e.mem.u32(object), NI_LIGHT_COLOR_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u16(object + COLOR_FLAGS), 0);
        let this = e.mem.alloc(0x3c);
        e.mem.set_u16(this + COLOR_FLAGS, 1);
        assert_eq!(e.call(0x00a4_d4b0, &args![this, 0x6000u32]).u32(), object);
        assert_eq!(calls(&log, BASE_COPY_MEMBERS), [vec![this, object, 0x6000]]);
        assert_eq!(e.mem.u16(object + COLOR_FLAGS), 1);
    }

    #[test]
    fn load_binary_takes_old_flags_or_reads_them() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BASE_LOAD_BINARY, 0);
        logged(&mut e, &log, STREAM_READ_LINK_ID, 0);
        let transfers = Transfers::default();
        let stream = stream(&mut e, &transfers, 0x0a00_0101);
        e.mem.set_u16(stream + STREAM_OLD_FLAGS, 0x0fe0);
        let this = e.mem.alloc(0x3c);
        e.call(0x00a4_d530, &args![this, stream]);
        assert_eq!(e.mem.u16(this + COLOR_FLAGS), 0x3f);
        assert!(transfers.borrow().is_empty());
        assert_eq!(calls(&log, STREAM_READ_LINK_ID), [vec![stream]]);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0068);
        e.call(0x00a4_d530, &args![this, stream]);
        assert_eq!(e.mem.u16(this + COLOR_FLAGS), 0x1234);
        assert_eq!(*transfers.borrow(), [(this + COLOR_FLAGS, 2, 2)]);
        assert_eq!(calls(&log, STREAM_READ_LINK_ID).len(), 1);
        assert_eq!(calls(&log, BASE_LOAD_BINARY).len(), 2);
    }
}
