//! `NiAnimation/nifloatsextradatapoint3controller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatsExtraDataPoint3Controller::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_RTTI: u32 = 0x011f_3fc4;

// Translated from 00a4eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_floats_extra_data_point3_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_RTTI)
}

/// `00a57630`: the jump target of `00a4eaf0`.
const JUMP_TARGET_00A4EAF0: u32 = 0x00a5_7630;

// Translated from 00a4eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::LinkObject` (Xbox PDB): a 5-byte jump to `00a57630` (engine map: `NiPSysModifierCtlr::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiFloatsExtraDataPoint3Controller@@UAAXAAVNiStream@@@Z`.
pub fn ni_floats_extra_data_point3_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A4EAF0, &args![this, stream]);
}

/// `NiFloatsExtraDataPoint3Controller`'s vtable (the address its constructors and destructor
/// store) and size.
const NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_VTABLE: u32 = 0x0109_917c;
const NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_SIZE: u32 = 0x48;
/// Fields (Xbox PDB names, PC offsets as the code uses them): `NiTimeController`'s flags byte
/// +0x08, `m_fScaledTime` +0x28, `m_pkTarget` +0x2c,
/// `NiSingleInterpController::m_spInterpolator` +0x34, `NiExtraDataController`'s
/// `m_kExtraDataName` +0x38 and `m_spExtraData` +0x3c, then this class's
/// `m_iFloatsExtraDataIndex` +0x40 and `m_kCtlrID` +0x44.
const TIME_FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
const EXTRA_DATA_NAME: u32 = 0x38;
const EXTRA_DATA: u32 = 0x3c;
const FLOATS_INDEX: u32 = 0x40;
const CONTROLLER_ID: u32 = 0x44;
/// The empty fixed string's handle (`0109b220`) and `NiPoint3Interpolator::ms_RTTI`
/// (`011f3f70`).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
const NI_POINT3_INTERPOLATOR_RTTI: u32 = 0x011f_3f70;
/// Callees (thiscall unless noted): `NiFloatsExtraData::GetValue` (`00a89020`, the index;
/// a float in ST0) and `SetValue` (`00a89000`, index and float); the `NiPoint3Interpolator`
/// constructor from a point (`00a49650`) and `SetPoseValue` (`00a494b0`), the point by
/// value; `00653290` (the object when it is of the class, else 0); the base class's
/// constructor (`00a572a0`), destructor (`00a572d0`), `LoadBinary` (`00a571a0`),
/// `GetViewerStrings` (`00a57420`) and `CopyMembers` (`00c312f0`; the engine map names some
/// after other classes, folded code); `NiTimeController::DontDoUpdate` (`00a36250`).
const FLOATS_GET_VALUE: u32 = 0x00a8_9020;
const FLOATS_SET_VALUE: u32 = 0x00a8_9000;
const POINT3_INTERPOLATOR_CONSTRUCT: u32 = 0x00a4_9650;
const POINT3_INTERPOLATOR_SET_POSE_VALUE: u32 = 0x00a4_94b0;
const DYNAMIC_CAST: u32 = 0x0065_3290;
const BASE_CONSTRUCT: u32 = 0x00a5_72a0;
const BASE_DESTRUCT: u32 = 0x00a5_72d0;
const BASE_LOAD_BINARY: u32 = 0x00a5_71a0;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a5_7420;
const BASE_COPY_MEMBERS: u32 = 0x00c3_12f0;
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The float a forced controller's scaled time becomes (`01096324`).
const FORCED_TIME: u32 = 0x0109_6324;
/// `00aa13e0` (cdecl, one word): allocates from the Ni allocator; `00aa1070` / `00aa10f0`
/// (cdecl: allocate / free); the sized delete (`00aa1460`); `NiSprintf` (`00a305b0`, cdecl)
/// and its format `"%s[%d]"`; `00a5b690` (cdecl): the fixed-string handle for a C string.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
const NI_SPRINTF: u32 = 0x00a3_05b0;
const FORMAT_NAME_INDEX: u32 = 0x0109_9268;
const GET_FIXED_STRING: u32 = 0x00a5_b690;
/// `InterlockedDecrement` (import slot); the interpolator's slots `0xb0` (point3 values
/// supported), `0xd4` (always update) and `0x94` (point3 update).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SLOT_IS_POINT3_VALUE_SUPPORTED: u32 = 0xb0;
const SLOT_ALWAYS_UPDATE: u32 = 0xd4;
const SLOT_UPDATE_POINT3: u32 = 0x94;
/// `NiStream::m_pkIstr` +0x24c (Xbox PDB name, PC offset) and the binary stream's position
/// (+4) and read function (+8).
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
/// Viewer strings: `NiGetViewerString` (`00a7d330`) and its signed form (`00a7d4d0`), cdecl;
/// the array's resize (`0096ad30`) and `SetAt` (`0096ae90`); the label
/// `"m_iFloatsExtraDataIndex"`.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;
const TEXT_FLOATS_INDEX: u32 = 0x0109_9270;

// Translated from 00a4ead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::InterpolatorIsCorrectType` (Xbox PDB; `this` and the
/// index word unread): the interpolator's answer to its slot `0xb0` (point3 values
/// supported).
pub fn ni_floats_extra_data_point3_controller_interpolator_is_correct_type(
    e: &mut Engine,
    _unused_0: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) -> u32 {
    e.vcall(
        interpolator.addr(),
        SLOT_IS_POINT3_VALUE_SUPPORTED,
        &args![],
    )
    .u32()
}

/// The three floats of the extra data at the controller's index (`GetValue` at index,
/// index + 1 and index + 2), each stored as a float.
fn floats(e: &mut Engine, this: u32) -> [u32; 3] {
    let data = e.mem.u32(this.wrapping_add(EXTRA_DATA));
    [0u32, 1, 2].map(|k| {
        let index = e.mem.u32(this.wrapping_add(FLOATS_INDEX)).wrapping_add(k);
        x87_store32(e.call(FLOATS_GET_VALUE, &args![data, index]).f64())
    })
}

// Translated from 00a4eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::CreatePoseInterpolator` (Xbox PDB; the index word is
/// unread): a new `NiPoint3Interpolator` (0x20 bytes, `00a49650`) posed at the extra data's
/// three floats from the controller's index; 0 when the allocation fails.
pub fn ni_floats_extra_data_point3_controller_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> u32 {
    let point = floats(e, this.addr());
    let block = e.call(NI_ALLOCATE, &args![0x20u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        POINT3_INTERPOLATOR_CONSTRUCT,
        &args![block, point[0], point[1], point[2]],
    )
    .u32()
}

/// The inline `NiFixedString::operator=` the controller ID's setters use: the slot takes the
/// handle for `text` (`00a5b690`) unless it already is `text`, and the old handle is
/// uncounted (unless it is the empty string's).
fn set_controller_id(e: &mut Engine, slot: u32, text: u32) {
    let old = e.mem.u32(slot);
    if old == text {
        return;
    }
    let handle = e.call(GET_FIXED_STRING, &args![text]).u32();
    e.mem.set_u32(slot, handle);
    if old != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
    }
}

// Translated from 00a4ebd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nifloatsextradatapoint3controller.cpp`): the controller
/// ID (+0x44), built on first use: while it is the empty string's handle, a non-null handle
/// is first replaced by the one for a null string (`00a5b690(0)`); then, with an extra data
/// name, `"%s[%d]"` of the name and the index (`NiSprintf` into a block of the name's length
/// + 15 bytes from `00aa1070`) becomes the ID and the block is freed. Returns the ID.
pub fn fn_00a4ebd0(e: &mut Engine, this: Ptr) -> u32 {
    let controller = this.addr();
    let slot = controller.wrapping_add(CONTROLLER_ID);
    let current = e.mem.u32(slot);
    if current != e.mem.u32(EMPTY_FIXED_STRING) {
        return current;
    }
    if current != 0 {
        set_controller_id(e, slot, 0);
    }
    let name = e.mem.u32(controller.wrapping_add(EXTRA_DATA_NAME));
    if name != 0 {
        let length = e.mem.cstr(name).len() as u32;
        let size = length.wrapping_add(0xf);
        let text = e.call(NI_ALLOC, &args![size]).u32();
        let index = e.mem.u32(controller.wrapping_add(FLOATS_INDEX));
        e.call(
            NI_SPRINTF,
            &args![text, size, FORMAT_NAME_INDEX, name, index],
        );
        set_controller_id(e, slot, text);
        e.call(NI_FREE, &args![text]);
    }
    e.mem.u32(slot)
}

// Translated from 00a4ec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::CreateClone` (Xbox PDB): a new controller (0x48 bytes:
/// the base constructor `00a572a0`, the vtable, index -1, the empty string's handle as the
/// ID), the base's `CopyMembers` (`00c312f0`, also for a null clone, as the exe does) and the
/// index.
pub fn ni_floats_extra_data_point3_controller_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> u32 {
    let object = e
        .call(
            NI_ALLOCATE,
            &args![NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_SIZE],
        )
        .u32();
    if object != 0 {
        construct_in(e, object);
    }
    e.call(BASE_COPY_MEMBERS, &args![this, object, cloning]);
    let index = e.mem.u32(this.addr().wrapping_add(FLOATS_INDEX));
    e.mem.set_u32(object.wrapping_add(FLOATS_INDEX), index);
    object
}

/// The constructor body `CreateObject` and `CreateClone` run on a new block: the base
/// constructor, the vtable, index -1 and the empty string's handle (not counted) as the ID.
fn construct_in(e: &mut Engine, object: u32) {
    e.call(BASE_CONSTRUCT, &args![object]);
    e.mem
        .set_u32(object, NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_VTABLE);
    e.mem.set_u32(object.wrapping_add(FLOATS_INDEX), u32::MAX);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(object.wrapping_add(CONTROLLER_ID), empty);
}

// Translated from 00a4ecd0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::CreateObject` (registered as the
/// `"NiFloatsExtraDataPoint3Controller"` loader at `00a2bde0`; Ghidra has no function here):
/// allocates 0x48 bytes and, when that succeeds, constructs an empty controller in it.
/// Returns it, or 0.
pub fn ni_floats_extra_data_point3_controller_create_object(e: &mut Engine) -> u32 {
    let object = e
        .call(
            NI_ALLOCATE,
            &args![NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_SIZE],
        )
        .u32();
    if object == 0 {
        return 0;
    }
    construct_in(e, object);
    object
}

// Translated from 00a4ed10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::_scalar_deleting_destructor_` (Xbox PDB): stores the
/// vtable, uncounts the controller ID (unless it is the empty string's), runs the base
/// destructor (`00a572d0`) and, when bit 0 of `flags` is set, the sized delete of 0x48 bytes.
/// Returns `this`.
pub fn ni_floats_extra_data_point3_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let object = this.addr();
    e.mem
        .set_u32(object, NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_VTABLE);
    let id = e.mem.u32(object.wrapping_add(CONTROLLER_ID));
    if id != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![id.wrapping_sub(8)]);
    }
    e.call(BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(
            NI_SIZED_DELETE,
            &args![this, NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_SIZE],
        );
    }
    this
}

// Translated from 00a4ed50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::SynchronizePoseInterpolator` (Xbox PDB; the index word
/// is unread): `NiPoint3Interpolator::SetPoseValue` with the extra data's three floats on the
/// interpolator as an `NiPoint3Interpolator` (`00653290`), or on a null one when
/// `interpolator` is null.
pub fn ni_floats_extra_data_point3_controller_synchronize_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: u32,
    _unused_2: u32,
) {
    let target = if interpolator == 0 {
        0
    } else {
        e.call(
            DYNAMIC_CAST,
            &args![interpolator, NI_POINT3_INTERPOLATOR_RTTI],
        )
        .u32()
    };
    let point = floats(e, this.addr());
    e.call(
        POINT3_INTERPOLATOR_SET_POSE_VALUE,
        &args![target, point[0], point[1], point[2]],
    );
}

// Translated from 00a4edd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::Update` (Xbox PDB): a forced controller (flag bit 5 of
/// the byte at +8) sets its scaled time to the float at `01096324`; otherwise, when
/// `DontDoUpdate` (with the update's time) says so, it stops unless the interpolator's slot
/// `0xd4` answers true. The interpolator's point3 update (slot `0x94`: scaled time, target,
/// out point) gives three floats that, with extra data, become its values at the index,
/// index + 1 and index + 2 (`SetValue`).
pub fn ni_floats_extra_data_point3_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
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
    e.with_stack(0xc, |e, point| {
        let point = point.addr();
        let time = x87_quiet_bits(e.mem.u32(object.wrapping_add(SCALED_TIME)));
        let target = e.mem.u32(object.wrapping_add(TARGET));
        if !e
            .vcall(
                interpolator,
                SLOT_UPDATE_POINT3,
                &args![time, target, point],
            )
            .bool()
        {
            return;
        }
        let data = e.mem.u32(object.wrapping_add(EXTRA_DATA));
        if data == 0 {
            return;
        }
        for k in 0..3u32 {
            let value = x87_quiet_bits(e.mem.u32(point + 4 * k));
            let index = e.mem.u32(object.wrapping_add(FLOATS_INDEX)).wrapping_add(k);
            e.call(FLOATS_SET_VALUE, &args![data, index, value]);
        }
    });
}

// Translated from 00a4ee90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::LoadBinary` (Xbox PDB): the base's `LoadBinary`
/// (`00a571a0`), then the index (4 bytes; the argument's own stack word is the component
/// size word).
pub fn ni_floats_extra_data_point3_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
        let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
        e.mem.set_u32(slot, 4);
        let index = this.addr().wrapping_add(FLOATS_INDEX);
        let bytes = e.call(read, &args![input, index, 4u32, slot, 1u32]).u32();
        let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
        e.mem.set_u32(
            input.wrapping_add(BINARY_STREAM_POSITION),
            position.wrapping_add(bytes),
        );
    });
}

/// The viewer-string array append the compiler inlined with the string in the stack word
/// `slot`: grows the array (16-bit size +0xA, capacity +8, grow-by +0xE) when full, then
/// `SetAt` at the old size with the slot's address.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a4eed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::GetViewerStrings` (Xbox PDB): the base's
/// `GetViewerStrings` (`00a57420`), then the class name (`011f3fc4`) and
/// `"m_iFloatsExtraDataIndex"` with the index (`00a7d4d0`), each held in the argument's own
/// stack word.
pub fn ni_floats_extra_data_point3_controller_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, strings);
        let class_name = e.mem.u32(NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let index = e.mem.u32(this.addr().wrapping_add(FLOATS_INDEX));
        let text = e
            .call(NI_GET_VIEWER_STRING_INT, &args![TEXT_FLOATS_INDEX, index])
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4eb40, ni_floats_extra_data_point3_controller_get_rtti() -> Ptr),
        entry!(
            0x00a4_eaf0,
            ni_floats_extra_data_point3_controller_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00a4_ead0,
            ni_floats_extra_data_point3_controller_interpolator_is_correct_type(Ptr, Ptr, u32)
                -> u32
        ),
        entry!(
            0x00a4_eb50,
            ni_floats_extra_data_point3_controller_create_pose_interpolator(Ptr, u32) -> u32
        ),
        entry!(0x00a4_ebd0, fn_00a4ebd0(Ptr) -> u32),
        entry!(
            0x00a4_ec80,
            ni_floats_extra_data_point3_controller_create_clone(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00a4_ecd0,
            ni_floats_extra_data_point3_controller_create_object() -> u32
        ),
        entry!(
            0x00a4_ed10,
            ni_floats_extra_data_point3_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a4_ed50,
            ni_floats_extra_data_point3_controller_synchronize_pose_interpolator(Ptr, u32, u32)
        ),
        entry!(
            0x00a4_edd0,
            ni_floats_extra_data_point3_controller_update(Ptr, Ptr)
        ),
        entry!(
            0x00a4_ee90,
            ni_floats_extra_data_point3_controller_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a4_eed0,
            ni_floats_extra_data_point3_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_floats_extra_data_point3_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4eb40, &args![]).u32(),
            NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a4eaf0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_eaf0;
        const TARGET: u32 = 0x00a5_7630;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

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

    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// `GetValue` answering index + 0.5 in ST0.
    fn values(e: &mut Engine) {
        e.register(FLOATS_GET_VALUE, |_, a| Ret {
            st0: f64::from(a[1]) + 0.5,
            ..Ret::default()
        });
    }

    #[test]
    fn interpolator_is_correct_type_asks_for_point3_values() {
        let mut e = Engine::new();
        let answer = Rc::new(Cell::new(0x0101u32));
        answers(&mut e, 0x00ee_a000, &answer);
        let interpolator = object(&mut e, 8, &[(SLOT_IS_POINT3_VALUE_SUPPORTED, 0x00ee_a000)]);
        let result = e.call(0x00a4_ead0, &args![0u32, interpolator, 3u32]).u32();
        assert_eq!(result, 0x0101);
    }

    #[test]
    fn create_pose_interpolator_poses_at_the_extra_data_floats() {
        let mut e = Engine::new();
        let log = Log::default();
        values(&mut e);
        let block = Rc::new(Cell::new(0u32));
        answers(&mut e, NI_ALLOCATE, &block);
        logged(&mut e, &log, POINT3_INTERPOLATOR_CONSTRUCT, 0x9000);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + EXTRA_DATA, 0x6000);
        e.mem.set_u32(this + FLOATS_INDEX, 2);
        assert_eq!(e.call(0x00a4_eb50, &args![this, 0u32]).u32(), 0);
        block.set(0x9000);
        assert_eq!(e.call(0x00a4_eb50, &args![this, 0u32]).u32(), 0x9000);
        assert_eq!(
            calls(&log, POINT3_INTERPOLATOR_CONSTRUCT),
            [vec![
                0x9000,
                2.5f32.to_bits(),
                3.5f32.to_bits(),
                4.5f32.to_bits()
            ]]
        );
    }

    fn empty_string(e: &mut Engine) {
        e.map(EMPTY_FIXED_STRING, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0900_0008);
    }

    #[test]
    fn controller_id_is_built_from_the_name_and_index() {
        let mut e = Engine::new();
        let log = Log::default();
        empty_string(&mut e);
        let handles = RefCell::new(vec![0x0a00_0008u32, 0x0b00_0008]);
        let made = log.clone();
        e.register_double(GET_FIXED_STRING, move |_, a| {
            made.borrow_mut().push((GET_FIXED_STRING, a.to_vec()));
            ret(handles.borrow_mut().remove(0))
        });
        logged(&mut e, &log, INTERLOCKED_DECREMENT, 0);
        logged(&mut e, &log, NI_SPRINTF, 0);
        logged(&mut e, &log, NI_FREE, 0);
        let text = e.mem.alloc(0x20);
        logged(&mut e, &log, NI_ALLOC, text);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Bone");
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + EXTRA_DATA_NAME, name);
        e.mem.set_u32(this + FLOATS_INDEX, 3);
        e.mem.set_u32(this + CONTROLLER_ID, 0x0900_0008);
        assert_eq!(e.call(0x00a4_ebd0, &args![this]).u32(), 0x0b00_0008);
        assert_eq!(calls(&log, GET_FIXED_STRING), [vec![0], vec![text]]);
        assert_eq!(calls(&log, NI_ALLOC), [vec![4 + 0xf]]);
        assert_eq!(
            calls(&log, NI_SPRINTF),
            [vec![text, 4 + 0xf, FORMAT_NAME_INDEX, name, 3]]
        );
        // The null string's handle is uncounted when replaced.
        assert_eq!(calls(&log, INTERLOCKED_DECREMENT), [vec![0x0a00_0000]]);
        assert_eq!(calls(&log, NI_FREE), [vec![text]]);
        // Built once: later calls answer it.
        assert_eq!(e.call(0x00a4_ebd0, &args![this]).u32(), 0x0b00_0008);
        assert_eq!(calls(&log, NI_ALLOC).len(), 1);
    }

    #[test]
    fn create_clone_and_create_object_build_the_controller() {
        let mut e = Engine::new();
        let log = Log::default();
        empty_string(&mut e);
        logged(&mut e, &log, BASE_CONSTRUCT, 0);
        logged(&mut e, &log, BASE_COPY_MEMBERS, 0);
        let block = Rc::new(Cell::new(0u32));
        answers(&mut e, NI_ALLOCATE, &block);
        assert_eq!(e.call(0x00a4_ecd0, &args![]).u32(), 0);
        let object = e.mem.alloc(0x48);
        block.set(object);
        assert_eq!(e.call(0x00a4_ecd0, &args![]).u32(), object);
        assert_eq!(
            e.mem.u32(object),
            NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_VTABLE
        );
        assert_eq!(e.mem.u32(object + FLOATS_INDEX), u32::MAX);
        assert_eq!(e.mem.u32(object + CONTROLLER_ID), 0x0900_0008);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + FLOATS_INDEX, 6);
        assert_eq!(e.call(0x00a4_ec80, &args![this, 0x9000u32]).u32(), object);
        assert_eq!(e.mem.u32(object + FLOATS_INDEX), 6);
        assert_eq!(calls(&log, BASE_COPY_MEMBERS), [vec![this, object, 0x9000]]);
        assert_eq!(calls(&log, BASE_CONSTRUCT), [vec![object], vec![object]]);
    }

    #[test]
    fn scalar_deleting_destructor_uncounts_the_id() {
        let mut e = Engine::new();
        let log = Log::default();
        empty_string(&mut e);
        logged(&mut e, &log, INTERLOCKED_DECREMENT, 0);
        logged(&mut e, &log, BASE_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + CONTROLLER_ID, 0x0900_0008);
        assert_eq!(e.call(0x00a4_ed10, &args![this, 0u32]).u32(), this);
        assert_eq!(
            e.mem.u32(this),
            NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_VTABLE
        );
        assert!(calls(&log, INTERLOCKED_DECREMENT).is_empty());
        e.mem.set_u32(this + CONTROLLER_ID, 0x0c00_0008);
        e.call(0x00a4_ed10, &args![this, 1u32]);
        assert_eq!(calls(&log, INTERLOCKED_DECREMENT), [vec![0x0c00_0000]]);
        assert_eq!(calls(&log, NI_SIZED_DELETE), [vec![this, 0x48]]);
        assert_eq!(calls(&log, BASE_DESTRUCT).len(), 2);
    }

    #[test]
    fn synchronize_pose_interpolator_sets_the_cast_interpolator_pose() {
        let mut e = Engine::new();
        let log = Log::default();
        values(&mut e);
        logged(&mut e, &log, DYNAMIC_CAST, 0x8800);
        logged(&mut e, &log, POINT3_INTERPOLATOR_SET_POSE_VALUE, 0);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + EXTRA_DATA, 0x6000);
        e.call(0x00a4_ed50, &args![this, 0u32, 1u32]);
        e.call(0x00a4_ed50, &args![this, 0x4400u32, 1u32]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            [vec![0x4400, NI_POINT3_INTERPOLATOR_RTTI]]
        );
        let point = [0.5f32, 1.5, 2.5].map(f32::to_bits);
        assert_eq!(
            calls(&log, POINT3_INTERPOLATOR_SET_POSE_VALUE),
            [
                vec![0, point[0], point[1], point[2]],
                vec![0x8800, point[0], point[1], point[2]]
            ]
        );
    }

    #[test]
    fn update_writes_the_point_into_the_extra_data() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(FORCED_TIME, 4);
        e.mem.set_f32(FORCED_TIME, 9.0);
        let skip = Rc::new(Cell::new(1u32));
        answers(&mut e, DONT_DO_UPDATE, &skip);
        let always = Rc::new(Cell::new(0u32));
        answers(&mut e, 0x00ee_a100, &always);
        let updated = Rc::new(Cell::new(1u32));
        let answer = updated.clone();
        e.register_double(0x00ee_a110, move |e, a| {
            for k in 0..3u32 {
                e.mem.set_f32(a[3] + 4 * k, k as f32 + 0.25);
            }
            ret(answer.get())
        });
        logged(&mut e, &log, FLOATS_SET_VALUE, 0);
        let interpolator = object(
            &mut e,
            8,
            &[
                (SLOT_ALWAYS_UPDATE, 0x00ee_a100),
                (SLOT_UPDATE_POINT3, 0x00ee_a110),
            ],
        );
        let this = e.mem.alloc(0x48);
        let update = e.mem.alloc(8);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_u32(this + EXTRA_DATA, 0x6000);
        e.mem.set_u32(this + FLOATS_INDEX, 4);
        e.call(0x00a4_edd0, &args![this, update]);
        assert!(calls(&log, FLOATS_SET_VALUE).is_empty());
        always.set(1);
        e.call(0x00a4_edd0, &args![this, update]);
        let values = [0.25f32, 1.25, 2.25].map(f32::to_bits);
        assert_eq!(
            calls(&log, FLOATS_SET_VALUE),
            [
                vec![0x6000, 4, values[0]],
                vec![0x6000, 5, values[1]],
                vec![0x6000, 6, values[2]]
            ]
        );
        e.mem.set_u8(this + TIME_FLAGS, 0x20);
        updated.set(0);
        e.call(0x00a4_edd0, &args![this, update]);
        assert_eq!(e.mem.f32(this + SCALED_TIME), 9.0);
        updated.set(1);
        e.mem.set_u32(this + EXTRA_DATA, 0);
        e.call(0x00a4_edd0, &args![this, update]);
        assert_eq!(calls(&log, FLOATS_SET_VALUE).len(), 3);
    }

    #[test]
    fn load_binary_reads_the_index() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BASE_LOAD_BINARY, 0);
        e.register(0x00ee_a200, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x11);
            ret(a[2])
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + BINARY_STREAM_READ, 0x00ee_a200);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        let this = e.mem.alloc(0x48);
        e.call(0x00a4_ee90, &args![this, stream]);
        assert_eq!(e.mem.u32(this + FLOATS_INDEX), 0x11);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 4);
        assert_eq!(calls(&log, BASE_LOAD_BINARY), [vec![this, stream]]);
    }

    #[test]
    fn get_viewer_strings_adds_the_name_and_index() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_RTTI, 4);
        e.mem
            .set_u32(NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_RTTI, 0x0155_0000);
        logged(&mut e, &log, BASE_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x0166_0000);
        logged(&mut e, &log, NI_GET_VIEWER_STRING_INT, 0x0177_0000);
        logged(&mut e, &log, VIEWER_STRINGS_GROW, 0);
        let seen = Rc::new(RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(VIEWER_STRINGS_SET_AT, move |e, a| {
            record.borrow_mut().push(e.mem.u32(a[2]));
            Ret::default()
        });
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + FLOATS_INDEX, 7);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 1);
        e.call(0x00a4_eed0, &args![this, strings]);
        assert_eq!(*seen.borrow(), [0x0166_0000, 0x0177_0000]);
        assert_eq!(
            calls(&log, NI_GET_VIEWER_STRING_INT),
            [vec![TEXT_FLOATS_INDEX, 7]]
        );
        assert_eq!(calls(&log, VIEWER_STRINGS_GROW).len(), 2);
    }
}
