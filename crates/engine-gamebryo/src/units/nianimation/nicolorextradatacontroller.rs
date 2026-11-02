//! `NiAnimation/nicolorextradatacontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiColorExtraDataController::ms_RTTI` (the class's `NiRTTI` record).
const NI_COLOR_EXTRA_DATA_CONTROLLER_RTTI: u32 = 0x011f_3fdc;

// Translated from 00a4fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_color_extra_data_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_COLOR_EXTRA_DATA_CONTROLLER_RTTI)
}

/// `00a57100`: the jump target of `00a4eb00`.
const JUMP_TARGET_00A4EB00: u32 = 0x00a5_7100;

/// `00a571e0`: the jump target of `00a4faf0`.
const JUMP_TARGET_00A4FAF0: u32 = 0x00a5_71e0;

// Translated from 00a4eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a57100` (engine map: `NiExtraDataController::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiColorExtraDataController@@UAA_NAAVNiStream@@@Z`.
pub fn ni_color_extra_data_controller_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A4EB00, &args![this, stream]).bool()
}

// Translated from 00a4faf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::IsEqual` (Xbox PDB): a 5-byte jump to `00a571e0` (engine map: `NiExtraDataController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiColorExtraDataController@@UAA_NPAVNiObject@@@Z`.
pub fn ni_color_extra_data_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A4FAF0, &args![this, other]).bool()
}

// Translated from 00a4fb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body at `00a572d0`, then the sized delete (`00aa1460`, `(this, size)`) of 0x40
/// bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_color_extra_data_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_72d0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x40u32]);
    }
    this
}

/// `NiColorExtraDataController` (Xbox PDB, 0x40 bytes; the PC code uses the
/// same offsets): `m_uFlags` +0x08, `m_fScaledTime` +0x28, `m_pkTarget`
/// +0x2C, `m_spInterpolator` +0x34 and `m_spExtraData` +0x3C (the colour of
/// an `NiColorExtraData` at its +0xC, four floats).
const FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
const EXTRA_DATA: u32 = 0x3c;
const EXTRA_DATA_COLOR: u32 = 0xc;
/// The class's size and vtable (RTTI `.?AVNiColorExtraDataController@@`).
const NI_COLOR_EXTRA_DATA_CONTROLLER_SIZE: u32 = 0x40;
const NI_COLOR_EXTRA_DATA_CONTROLLER_VTABLE: u32 = 0x0109_947c;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiExtraDataController`'s constructor (`00a572a0`; `thiscall`, no stack
/// word) and the member copy `CreateClone` runs (`00c312f0`; `thiscall`:
/// clone, cloning process).
const EXTRA_DATA_CONTROLLER_CONSTRUCT: u32 = 0x00a5_72a0;
const COPY_MEMBERS: u32 = 0x00c3_12f0;
/// The base `LinkObject` (`00a57630`; `thiscall`: stream; the engine map's
/// name for that address is a folded body's).
const BASE_LINK_OBJECT: u32 = 0x00a5_7630;
/// `NiBlendColorInterpolator::NiBlendColorInterpolator` (`00a55f80`;
/// `thiscall`: flag, threshold, count).
const BLEND_COLOR_INTERPOLATOR_CONSTRUCT: u32 = 0x00a5_5f80;
/// The `NiColorInterpolator` constructors taking a colour by value
/// (`00a3bd80`, four words) and colour data (`00a3bd10`), and the colour
/// pose setter `00a3bc60` (four words), all `thiscall`.
const COLOR_INTERPOLATOR_CONSTRUCT_WITH_COLOR: u32 = 0x00a3_bd80;
const COLOR_INTERPOLATOR_CONSTRUCT_WITH_DATA: u32 = 0x00a3_bd10;
const COLOR_INTERPOLATOR_SET_POSE_COLOR: u32 = 0x00a3_bc60;
/// `NiColorInterpolator::ms_RTTI` (`011f3730`).
const NI_COLOR_INTERPOLATOR_RTTI: u32 = 0x011f_3730;
/// `00653290` (`thiscall` on an object, `RET 4`: an `NiRTTI` record): the
/// object when it is of that class (or derived), else 0.
const OBJECT_CAST: u32 = 0x0065_3290;
/// `NiColorExtraData::SetValue` (`00a893b0`; `thiscall`: a colour by value,
/// four words).
const COLOR_EXTRA_DATA_SET_VALUE: u32 = 0x00a8_93b0;
/// `NiTimeController::DontDoUpdate` (`00a36250`; `thiscall`: time).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The float the scaled time is set to when flag 0x20 is set (`01096324`).
const SCALED_TIME_RESET: u32 = 0x0109_6324;
/// `NiStream::GetObjectFromLinkID` (`00a63430`; `thiscall` on the stream).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8; files before 10.1.0.104
/// (`0x0a010068`) link colour data instead of an interpolator.
const STREAM_FILE_VERSION: u32 = 0xd8;
const VERSION_10_1_0_104: u32 = 0x0a01_0068;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Virtual slots (Xbox PDB names): `NiRefObject::DeleteThis` +0x04, the
/// interpolator's `IsColorAValueSupported` +0xB4, `Collapse` +0xBC,
/// `AlwaysUpdate` +0xD4 and colour `Update` +0x90 (`(time, target,
/// &colour)`).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_IS_COLOR_VALUE_SUPPORTED: u32 = 0xb4;
const SLOT_COLLAPSE: u32 = 0xbc;
const SLOT_ALWAYS_UPDATE: u32 = 0xd4;
const SLOT_UPDATE_COLOR: u32 = 0x90;
/// `NiExtraDataController::GetViewerStrings` (`00a57420`), `NiGetViewerString`
/// (`00a7d330`, `cdecl`: the class name) and the viewer-string array's grow
/// (`0096ad30`) and store (`0096ae90`).
const EXTRA_DATA_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a5_7420;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// The four words of the extra data's colour.
fn extra_data_color(e: &mut Engine, this: u32) -> [u32; 4] {
    let extra = e.mem.u32(this.wrapping_add(EXTRA_DATA));
    let color = extra.wrapping_add(EXTRA_DATA_COLOR);
    [0u32, 4, 8, 12].map(|offset| e.mem.u32(color.wrapping_add(offset)))
}

// Translated from 00a4fa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::CreateBlendInterpolator` (Xbox PDB;
/// `thiscall`, five words, `RET 0x14`; the first and third are not read): a
/// 0x44-byte `NiBlendColorInterpolator` built from `(flag, threshold,
/// count)` (the float copied through the x87 stack); 0 when the allocation
/// fails.
pub fn ni_color_extra_data_controller_create_blend_interpolator(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    flag: u32,
    _unused_3: u32,
    threshold: u32,
    count: u32,
) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x44u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(
        BLEND_COLOR_INTERPOLATOR_CONSTRUCT,
        &args![block, flag, x87_quiet_bits(threshold), count],
    )
    .ptr()
}

// Translated from 00a4faa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::InterpolatorIsCorrectType` (Xbox PDB;
/// `RET 8`: interpolator, index; `this` and the index are not read): the
/// interpolator's `IsColorAValueSupported` (slot +0xB4).
pub fn ni_color_extra_data_controller_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) -> bool {
    e.vcall(interpolator.addr(), SLOT_IS_COLOR_VALUE_SUPPORTED, &args![])
        .bool()
}

// Translated from 00a4fac0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::CreateObject` (Ghidra has no function here;
/// the loader's factory by its body): allocates 0x40 bytes and, when that
/// succeeds, runs `NiExtraDataController`'s constructor and stores the
/// vtable. Returns the object, or 0.
pub fn ni_color_extra_data_controller_create_object(e: &mut Engine) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![NI_COLOR_EXTRA_DATA_CONTROLLER_SIZE])
        .u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(EXTRA_DATA_CONTROLLER_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, NI_COLOR_EXTRA_DATA_CONTROLLER_VTABLE);
    Ptr::new(block)
}

// Translated from 00a4fb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::CreatePoseInterpolator` (Xbox PDB; `RET 4`;
/// the index word is not read): allocates 0x24 bytes and constructs an
/// `NiColorInterpolator` holding the extra data's colour (`00a3bd80`, the
/// four words by value). Answers the constructor's result, 0 when the
/// allocation fails.
pub fn ni_color_extra_data_controller_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x24u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    let [red, green, blue, alpha] = extra_data_color(e, this.addr());
    e.call(
        COLOR_INTERPOLATOR_CONSTRUCT_WITH_COLOR,
        &args![block, red, green, blue, alpha],
    )
    .ptr()
}

// Translated from 00a4fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::CreateClone` (Xbox PDB; `RET 4`: cloning
/// process): a new 0x40-byte controller (`NiExtraDataController`'s
/// constructor, then this class's vtable), filled by the member copy
/// (`00c312f0`; also called with a null clone when the allocation fails).
/// Answers the clone.
pub fn ni_color_extra_data_controller_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![NI_COLOR_EXTRA_DATA_CONTROLLER_SIZE])
        .u32();
    if block == 0 {
        e.call(COPY_MEMBERS, &args![this, 0u32, cloning]);
        return Ptr::new(0);
    }
    e.call(EXTRA_DATA_CONTROLLER_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, NI_COLOR_EXTRA_DATA_CONTROLLER_VTABLE);
    e.call(COPY_MEMBERS, &args![this, block, cloning]);
    Ptr::new(block)
}

// Translated from 00a4fc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::LinkObject` (Xbox PDB): the base
/// `LinkObject` (`00a57630`); for files before 10.1.0.104 the linked colour
/// data (`NiStream::GetObjectFromLinkID`) gets a 0x24-byte
/// `NiColorInterpolator` (`00a3bd10`; 0 when the allocation fails), which
/// becomes the interpolator (counted, the old one released), and its
/// `Collapse` (slot +0xBC) runs.
pub fn ni_color_extra_data_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION)) >= VERSION_10_1_0_104 {
        return;
    }
    let object = this.addr();
    let data = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let block = e.call(NI_ALLOCATE, &args![0x24u32]).u32();
    let interpolator = if block == 0 {
        0
    } else {
        e.call(COLOR_INTERPOLATOR_CONSTRUCT_WITH_DATA, &args![block, data])
            .u32()
    };
    let old = e.mem.u32(object.wrapping_add(INTERPOLATOR));
    if old != interpolator {
        if old != 0
            && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(old, SLOT_DELETE_THIS, &args![]);
        }
        e.mem
            .set_u32(object.wrapping_add(INTERPOLATOR), interpolator);
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator.wrapping_add(4)]);
        }
    }
    let interpolator = e.mem.u32(object.wrapping_add(INTERPOLATOR));
    e.vcall(interpolator, SLOT_COLLAPSE, &args![]);
}

// Translated from 00a4fc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::SynchronizePoseInterpolator` (Xbox PDB;
/// `RET 8`: interpolator, index; the index word is not read): the
/// interpolator cast to `NiColorInterpolator` (`00653290`; a null
/// interpolator gives 0 without the call) gets the extra data's colour
/// through `00a3bc60` (the four words by value; called on the cast result
/// even when it is 0).
pub fn ni_color_extra_data_controller_synchronize_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) {
    let cast = if interpolator.addr() == 0 {
        0
    } else {
        e.call(
            OBJECT_CAST,
            &args![interpolator, NI_COLOR_INTERPOLATOR_RTTI],
        )
        .u32()
    };
    let [red, green, blue, alpha] = extra_data_color(e, this.addr());
    e.call(
        COLOR_INTERPOLATOR_SET_POSE_COLOR,
        &args![cast, red, green, blue, alpha],
    );
}

// Translated from 00a4fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::Update` (Xbox PDB; `RET 4`: the update
/// data, whose first word is the time). With flag 0x20 of the flags byte
/// the scaled time becomes the float at `01096324`; otherwise
/// `NiTimeController::DontDoUpdate` decides, and when it says skip the
/// update still runs if the interpolator's `AlwaysUpdate` (slot +0xD4)
/// answers true. With an interpolator, a stack colour set to four zeros
/// (`FLDZ`) goes to its colour `Update` (slot +0x90: scaled time, target,
/// address of the colour); on success, with extra data,
/// `NiColorExtraData::SetValue` takes the colour (four words by value).
pub fn ni_color_extra_data_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
    let object = this.addr();
    if (e.mem.u8(object.wrapping_add(FLAGS)) >> 5) & 1 != 0 {
        let reset = x87_quiet_bits(e.global::<u32>(SCALED_TIME_RESET));
        e.mem.set_u32(object.wrapping_add(SCALED_TIME), reset);
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
    e.with_stack(16, |e, color| {
        let color = color.addr();
        for offset in [0u32, 4, 8, 12] {
            e.mem.set_u32(color + offset, 0);
        }
        let target = e.mem.u32(object.wrapping_add(TARGET));
        let scaled_time = x87_quiet_bits(e.mem.u32(object.wrapping_add(SCALED_TIME)));
        if !e
            .vcall(
                interpolator,
                SLOT_UPDATE_COLOR,
                &args![scaled_time, target, color],
            )
            .bool()
        {
            return;
        }
        let extra = e.mem.u32(object.wrapping_add(EXTRA_DATA));
        if extra == 0 {
            return;
        }
        let [red, green, blue, alpha] = [0u32, 4, 8, 12].map(|offset| e.mem.u32(color + offset));
        e.call(
            COLOR_EXTRA_DATA_SET_VALUE,
            &args![extra, red, green, blue, alpha],
        );
    });
}

// Translated from 00a4fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraDataController::GetViewerStrings` (Xbox PDB):
/// `NiExtraDataController::GetViewerStrings`, then the class name (the
/// `NiRTTI` record's first word through `NiGetViewerString`), appended
/// (the array grown by its step when full) through the argument's own
/// stack word.
pub fn ni_color_extra_data_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(
        EXTRA_DATA_CONTROLLER_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    let strings = strings.addr();
    let name = e.mem.u32(NI_COLOR_EXTRA_DATA_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
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
        entry!(0x00a4fb00, ni_color_extra_data_controller_get_rtti() -> Ptr),
        entry!(0x00a4_eb00, ni_color_extra_data_controller_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a4_faf0, ni_color_extra_data_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a4fb10, ni_color_extra_data_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a4_fa70,
            ni_color_extra_data_controller_create_blend_interpolator(Ptr, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00a4_faa0,
            ni_color_extra_data_controller_interpolator_is_correct_type(Ptr, Ptr, u32) -> bool
        ),
        entry!(0x00a4_fac0, ni_color_extra_data_controller_create_object() -> Ptr),
        entry!(
            0x00a4_fb30,
            ni_color_extra_data_controller_create_pose_interpolator(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a4_fb80,
            ni_color_extra_data_controller_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a4_fc00,
            ni_color_extra_data_controller_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00a4_fc90,
            ni_color_extra_data_controller_synchronize_pose_interpolator(Ptr, Ptr, u32)
        ),
        entry!(0x00a4_fce0, ni_color_extra_data_controller_update(Ptr, Ptr)),
        entry!(
            0x00a4_fd90,
            ni_color_extra_data_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_color_extra_data_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4fb00, &args![]).u32(),
            NI_COLOR_EXTRA_DATA_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a4eb00_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_eb00;
        const TARGET: u32 = 0x00a5_7100;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a4faf0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_faf0;
        const TARGET: u32 = 0x00a5_71e0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn ni_color_extra_data_controller_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_72d0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let deletes = |e: &Engine| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == 0x00aa_1460)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a4fb10, &args![this, 0xfeu32]).u32(), this);
        assert!(deletes(&e).is_empty());
        assert_eq!(e.call(0x00a4fb10, &args![this, 1u32]).u32(), this);
        assert_eq!(deletes(&e), vec![vec![this, 0x40]]);
        let bodies = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x00a5_72d0)
            .count();
        assert_eq!(bodies, 2);
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    const DELETE_THIS: u32 = 0x0340_0000;
    const IS_COLOR: u32 = 0x0340_0010;
    const COLLAPSE: u32 = 0x0340_0020;
    const ALWAYS: u32 = 0x0340_0030;
    const UPDATE: u32 = 0x0340_0040;

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
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        for address in [DELETE_THIS, COLLAPSE] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem
            .set_u32(vtable + SLOT_IS_COLOR_VALUE_SUPPORTED, IS_COLOR);
        e.mem.set_u32(vtable + SLOT_COLLAPSE, COLLAPSE);
        e.mem.set_u32(vtable + SLOT_ALWAYS_UPDATE, ALWAYS);
        e.mem.set_u32(vtable + SLOT_UPDATE_COLOR, UPDATE);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// A controller whose extra data holds the colour words 1, 2, 3, 4.
    fn controller(e: &mut Engine) -> u32 {
        let extra = e.mem.alloc(0x20);
        for (i, word) in [1u32, 2, 3, 4].iter().enumerate() {
            e.mem
                .set_u32(extra + EXTRA_DATA_COLOR + 4 * i as u32, *word);
        }
        let this = e.mem.alloc(NI_COLOR_EXTRA_DATA_CONTROLLER_SIZE);
        e.mem.set_u32(this + EXTRA_DATA, extra);
        this
    }

    #[test]
    fn create_blend_interpolator_builds_a_colour_blend() {
        let mut e = engine();
        let block = e.mem.alloc(0x44);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.register(BLEND_COLOR_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        let answer = e
            .call(
                0x00a4_fa70,
                &args![0x5000u32, 0u32, 1u32, 0u32, 0xff80_0001u32, 2u32],
            )
            .u32();
        assert_eq!(answer, block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x44]]);
        assert_eq!(
            calls(&e, BLEND_COLOR_INTERPOLATOR_CONSTRUCT),
            vec![vec![block, 1, 0xffc0_0001, 2]]
        );
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00a4_fa70, &args![0u32, 0u32, 0u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(calls(&e, BLEND_COLOR_INTERPOLATOR_CONSTRUCT).len(), 1);
    }

    #[test]
    fn interpolator_is_correct_type_asks_the_interpolator() {
        let mut e = engine();
        let mut answers = vec![1u32, 0];
        e.register_double(IS_COLOR, move |_, _| answers.remove(0).into_ret());
        let interpolator = object(&mut e, 1);
        assert!(e.call(0x00a4_faa0, &args![0u32, interpolator, 3u32]).bool());
        assert!(!e.call(0x00a4_faa0, &args![0u32, interpolator, 3u32]).bool());
        assert_eq!(calls(&e, IS_COLOR), vec![vec![interpolator]; 2]);
    }

    #[test]
    fn create_object_builds_a_controller_or_answers_null() {
        let mut e = engine();
        e.register(EXTRA_DATA_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        let block = e.mem.alloc(0x40);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        assert_eq!(e.call(0x00a4_fac0, &args![]).u32(), block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x40]]);
        assert_eq!(
            calls(&e, EXTRA_DATA_CONTROLLER_CONSTRUCT),
            vec![vec![block]]
        );
        assert_eq!(e.mem.u32(block), NI_COLOR_EXTRA_DATA_CONTROLLER_VTABLE);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_fac0, &args![]).u32(), 0);
        assert_eq!(calls(&e, EXTRA_DATA_CONTROLLER_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_pose_interpolator_holds_the_extra_data_colour() {
        let mut e = engine();
        let block = e.mem.alloc(0x24);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.register(COLOR_INTERPOLATOR_CONSTRUCT_WITH_COLOR, |_, a| {
            a[0].into_ret()
        });
        let this = controller(&mut e);
        assert_eq!(e.call(0x00a4_fb30, &args![this, 0u32]).u32(), block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x24]]);
        assert_eq!(
            calls(&e, COLOR_INTERPOLATOR_CONSTRUCT_WITH_COLOR),
            vec![vec![block, 1, 2, 3, 4]]
        );
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_fb30, &args![this, 0u32]).u32(), 0);
        assert_eq!(calls(&e, COLOR_INTERPOLATOR_CONSTRUCT_WITH_COLOR).len(), 1);
    }

    #[test]
    fn create_clone_builds_and_copies() {
        let mut e = engine();
        e.register(EXTRA_DATA_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(COPY_MEMBERS, |_, _| Ret::default());
        let block = e.mem.alloc(0x40);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        assert_eq!(
            e.call(0x00a4_fb80, &args![0x3000u32, 0x4000u32]).u32(),
            block
        );
        assert_eq!(e.mem.u32(block), NI_COLOR_EXTRA_DATA_CONTROLLER_VTABLE);
        assert_eq!(calls(&e, COPY_MEMBERS), vec![vec![0x3000, block, 0x4000]]);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_fb80, &args![0x3000u32, 0x4000u32]).u32(), 0);
        assert_eq!(calls(&e, COPY_MEMBERS)[1], vec![0x3000, 0, 0x4000]);
        assert_eq!(calls(&e, EXTRA_DATA_CONTROLLER_CONSTRUCT).len(), 1);
    }

    #[test]
    fn link_object_of_an_old_file_wraps_the_colour_data() {
        let mut e = engine();
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_GET_OBJECT_FROM_LINK_ID, |_, _| 0x6600u32.into_ret());
        let block = e.mem.alloc(0x24);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        let interpolator = object(&mut e, 0);
        e.register_double(COLOR_INTERPOLATOR_CONSTRUCT_WITH_DATA, move |_, _| {
            interpolator.into_ret()
        });
        let stream = e.mem.alloc(0x100);
        e.mem
            .set_u32(stream + STREAM_FILE_VERSION, VERSION_10_1_0_104);
        let this = controller(&mut e);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + INTERPOLATOR, old);
        // A new file: only the base.
        e.call(0x00a4_fc00, &args![this, stream]);
        assert!(calls(&e, STREAM_GET_OBJECT_FROM_LINK_ID).is_empty());
        e.mem
            .set_u32(stream + STREAM_FILE_VERSION, VERSION_10_1_0_104 - 1);
        e.call(0x00a4_fc00, &args![this, stream]);
        assert_eq!(calls(&e, BASE_LINK_OBJECT), vec![vec![this, stream]; 2]);
        assert_eq!(
            calls(&e, COLOR_INTERPOLATOR_CONSTRUCT_WITH_DATA),
            vec![vec![block, 0x6600]]
        );
        assert_eq!(e.mem.u32(this + INTERPOLATOR), interpolator);
        assert_eq!(e.mem.u32(interpolator + 4), 1);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![old]]);
        assert_eq!(calls(&e, COLLAPSE), vec![vec![interpolator]]);
    }

    #[test]
    fn synchronize_pose_interpolator_passes_the_colour() {
        let mut e = engine();
        e.register(OBJECT_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(COLOR_INTERPOLATOR_SET_POSE_COLOR, |_, _| Ret::default());
        let this = controller(&mut e);
        e.call(0x00a4_fc90, &args![this, 0x5000u32, 0u32]);
        assert_eq!(
            calls(&e, OBJECT_CAST),
            vec![vec![0x5000, NI_COLOR_INTERPOLATOR_RTTI]]
        );
        e.call(0x00a4_fc90, &args![this, 0u32, 0u32]);
        assert_eq!(calls(&e, OBJECT_CAST).len(), 1);
        assert_eq!(
            calls(&e, COLOR_INTERPOLATOR_SET_POSE_COLOR),
            vec![vec![0x5001, 1, 2, 3, 4], vec![0, 1, 2, 3, 4]]
        );
    }

    /// A controller with an interpolator whose colour `Update` checks the
    /// zeroed colour, writes 5, 6, 7, 8 and answers true.
    fn updating(e: &mut Engine, flags: u8, always: bool) -> u32 {
        let interpolator = object(e, 1);
        e.register(UPDATE, |e, a| {
            for i in 0..4 {
                assert_eq!(e.mem.u32(a[3] + 4 * i), 0);
                e.mem.set_u32(a[3] + 4 * i, 5 + i);
            }
            1u32.into_ret()
        });
        e.register_double(ALWAYS, move |_, _| u32::from(always).into_ret());
        e.register(COLOR_EXTRA_DATA_SET_VALUE, |_, _| Ret::default());
        let this = controller(e);
        e.mem.set_u8(this + FLAGS, flags);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_f32(this + SCALED_TIME, 2.5);
        e.mem.set_u32(this + TARGET, 0x6000);
        this
    }

    #[test]
    fn update_hands_the_colour_to_the_extra_data() {
        let mut e = engine();
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        let this = updating(&mut e, 0, false);
        let update = e.mem.alloc(4);
        e.mem.set_f32(update, 1.0);
        e.call(0x00a4_fce0, &args![this, update]);
        assert_eq!(
            calls(&e, DONT_DO_UPDATE),
            vec![vec![this, 1.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, UPDATE)[0][1..3], [2.5f32.to_bits(), 0x6000]);
        let extra = e.mem.u32(this + EXTRA_DATA);
        assert_eq!(
            calls(&e, COLOR_EXTRA_DATA_SET_VALUE),
            vec![vec![extra, 5, 6, 7, 8]]
        );
        e.mem.set_u32(this + EXTRA_DATA, 0);
        e.call(0x00a4_fce0, &args![this, update]);
        assert_eq!(calls(&e, COLOR_EXTRA_DATA_SET_VALUE).len(), 1);
    }

    #[test]
    fn update_skips_unless_always_and_resets_with_flag_0x20() {
        let mut e = engine();
        e.register(DONT_DO_UPDATE, |_, _| 1u32.into_ret());
        let update = e.mem.alloc(4);
        let this = updating(&mut e, 0, false);
        e.call(0x00a4_fce0, &args![this, update]);
        assert!(calls(&e, UPDATE).is_empty());
        let this = updating(&mut e, 0, true);
        e.call(0x00a4_fce0, &args![this, update]);
        assert_eq!(calls(&e, UPDATE).len(), 1);
        e.map(SCALED_TIME_RESET, 4);
        e.set_global(SCALED_TIME_RESET, f32::MIN.to_bits());
        let this = updating(&mut e, 0x20, false);
        e.call(0x00a4_fce0, &args![this, update]);
        assert_eq!(calls(&e, DONT_DO_UPDATE).len(), 2);
        assert_eq!(calls(&e, UPDATE)[1][1], f32::MIN.to_bits());
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = engine();
        e.register(EXTRA_DATA_CONTROLLER_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x7100u32.into_ret());
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x7100);
            Ret::default()
        });
        e.map(NI_COLOR_EXTRA_DATA_CONTROLLER_RTTI, 4);
        e.mem.set_u32(NI_COLOR_EXTRA_DATA_CONTROLLER_RTTI, 0x5151);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 3);
        e.call(0x00a4_fd90, &args![0x3000u32, strings]);
        assert_eq!(
            calls(&e, EXTRA_DATA_CONTROLLER_GET_VIEWER_STRINGS),
            vec![vec![0x3000, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151]]);
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 5]]);
        assert_eq!(calls(&e, STRINGS_SET_AT)[0][..2], [strings, 2]);
    }
}
