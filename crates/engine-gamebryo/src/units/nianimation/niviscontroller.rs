//! `NiAnimation/niviscontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiVisController::ms_RTTI` (the class's `NiRTTI` record).
const NI_VIS_CONTROLLER_RTTI: u32 = 0x011f_3f08;

// Translated from 00a44eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVisController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_vis_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_VIS_CONTROLLER_RTTI)
}

// Translated from 00a44f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVisController::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a57800`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f3f08`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_vis_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_7800;
    const CLASS_NAME_POINTER: u32 = 0x011f_3f08;
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

/// The vtable of `NiVisController` (RTTI `.?AVNiVisController@@`).
const NI_VIS_CONTROLLER_VTABLE: u32 = 0x0109_7e8c;
/// `NiVisController` size (the `CreateObject` allocation).
const NI_VIS_CONTROLLER_SIZE: u32 = 0x38;

/// `NiTimeController` fields as the PC code uses them: the flags byte +0x08
/// (bit 5 forces the update), the last scaled time +0x28 and the target
/// +0x2c; `NiSingleInterpController::m_spInterpolator` (Xbox PDB) +0x34.
const FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
/// The target's flags word (`NiAVObject::m_uFlags`, Xbox PDB, +0x30); bit 0
/// is the bit the controller drives (clear for a true value).
const TARGET_FLAGS: u32 = 0x30;

/// The scaled time a forced update uses (a float in `.rdata`).
const FORCED_UPDATE_TIME: u32 = 0x0109_6324;
/// `NiStream::m_uiNifFileVersion` +0xd8 (Xbox PDB; PC offset).
const STREAM_VERSION: u32 = 0xd8;
/// The first file version whose visibility controllers link their
/// interpolator (older ones link bool data).
const VERSION_WITH_INTERPOLATOR: u32 = 0x0a01_0068;

/// `NiTimeController::DontDoUpdate` (`00a36250`, `thiscall`, one float).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The interpolator's virtual slots: `0xd4` (no arguments, a bool), the
/// bool update `0xa0` `(time, target, &value)` (a bool), and `0xbc` (no
/// arguments) that `LinkObject` runs on an interpolator it made.
const SLOT_INTERPOLATOR_ALWAYS_UPDATE: u32 = 0xd4;
const SLOT_INTERPOLATOR_UPDATE_BOOL: u32 = 0xa0;
const SLOT_INTERPOLATOR_COLLAPSE: u32 = 0xbc;
/// `NiFloatInterpController::LoadBinary` (`00a57c70`; the engine map's
/// name for the base load this class's load runs).
const BASE_LOAD_BINARY: u32 = 0x00a5_7c70;
/// `00a645b0` (unit `nistream.cpp`, `thiscall` on the stream): reads a
/// link ID.
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiPSysModifierCtlr::LinkObject` (`00a57630`; the engine map's name for
/// the base link this class's link runs).
const BASE_LINK_OBJECT: u32 = 0x00a5_7630;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall` on the stream).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a53280` (unit `niboolinterpolator.cpp`, `thiscall`, one word): the
/// `NiBoolInterpolator` constructor taking a bool data pointer.
const BOOL_INTERPOLATOR_CONSTRUCT: u32 = 0x00a5_3280;
/// The size `LinkObject` allocates for that interpolator.
const BOOL_INTERPOLATOR_SIZE: u32 = 0x18;
/// `00a57790` (unplaced, `run_00a57790`): the constructor body
/// `CreateObject` and `CreateClone` run (it stores the
/// `NiBoolInterpController` vtable).
const BOOL_INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a5_7790;
/// `00a57620` (unnamed in the engine map; `thiscall`, `(destination,
/// cloning)`): the member copy `CreateClone` runs.
const COPY_MEMBERS: u32 = 0x00a5_7620;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots) and
/// `NiRefObject::DeleteThis` (virtual slot 1).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SLOT_DELETE_THIS: u32 = 0x04;

// Translated from 00a44d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVisController::GetTargetBoolValue` (Xbox PDB): stores whether bit 0
/// of the target's flags word is clear (1) or set (0) in the byte at `value`.
pub fn ni_vis_controller_get_target_bool_value(e: &mut Engine, this: Ptr, value: Ptr) {
    let target = e.mem.u32(this.addr().wrapping_add(TARGET));
    let flags = e.mem.u8(target.wrapping_add(TARGET_FLAGS));
    e.mem.set_u8(value.addr(), !flags & 1);
}

// Translated from 00a44d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niviscontroller.cpp`; the body after
/// `GetTargetBoolValue` and before `LinkObject`, a stream load): the base
/// load (`00a57c70`), then, in files older than version `0x0a010068`, one
/// more link ID read (`00a645b0`), the bool data that `LinkObject` turns
/// into an interpolator.
pub fn fn_00a44d20(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_VERSION)) < VERSION_WITH_INTERPOLATOR {
        e.call(STREAM_READ_LINK_ID, &args![stream]);
    }
}

// Translated from 00a44d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVisController::LinkObject` (Xbox PDB): the base link (`00a57630`),
/// then, in files older than version `0x0a010068`, the linked bool data
/// (`GetObjectFromLinkID`) wrapped in a new 0x18-byte `NiBoolInterpolator`
/// (null when the allocation fails) that becomes the interpolator (counted;
/// the old one released), and its slot `0xbc` runs.
pub fn ni_vis_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_VERSION)) >= VERSION_WITH_INTERPOLATOR {
        return;
    }
    link_old_bool_data(e, this.addr(), stream.addr());
}

/// The old-file part of the link: the bool data in a new interpolator,
/// stored as the controller's interpolator, then its slot `0xbc`.
fn link_old_bool_data(e: &mut Engine, this: u32, stream: u32) {
    let data = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let block = e.call(NI_ALLOCATE, &args![BOOL_INTERPOLATOR_SIZE]).u32();
    let interpolator = if block == 0 {
        0
    } else {
        e.call(BOOL_INTERPOLATOR_CONSTRUCT, &args![block, data])
            .u32()
    };
    let slot = this.wrapping_add(INTERPOLATOR);
    let old = e.mem.u32(slot);
    if old != interpolator {
        if old != 0
            && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(old, SLOT_DELETE_THIS, &args![]);
        }
        e.mem.set_u32(slot, interpolator);
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator.wrapping_add(4)]);
        }
    }
    let interpolator = e.mem.u32(slot);
    e.vcall(interpolator, SLOT_INTERPOLATOR_COLLAPSE, &args![]);
}

// Translated from 00a44e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVisController::Update` (Xbox PDB). A forced update (flags bit 5)
/// sets the scaled time to the float at `01096324`; otherwise
/// `DontDoUpdate` runs on the time (the first float of `update`) and, when
/// it says to skip, the update only goes on when there is an interpolator
/// whose slot `0xd4` says so. Then the interpolator's bool update (slot
/// `0xa0`) runs with the scaled time, the target and the argument's own
/// stack word as the value; on success bit 0 of the target's flags word is
/// cleared for a true value and set for a false one, and when the bit
/// was set before and the value is true the byte at `update + 8` is set.
pub fn ni_vis_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
    let this = this.addr();
    if (e.mem.u8(this.wrapping_add(FLAGS)) >> 5) & 1 != 0 {
        let forced = e.mem.u32(FORCED_UPDATE_TIME);
        e.mem
            .set_u32(this.wrapping_add(SCALED_TIME), x87_quiet_bits(forced));
    } else {
        let time = x87_quiet_bits(e.mem.u32(update.addr()));
        if e.call(DONT_DO_UPDATE, &args![this, time]).u8() != 0 {
            let interpolator = e.mem.u32(this.wrapping_add(INTERPOLATOR));
            if interpolator == 0 {
                return;
            }
            if e.vcall(interpolator, SLOT_INTERPOLATOR_ALWAYS_UPDATE, &args![])
                .u8()
                == 0
            {
                return;
            }
        }
    }
    let interpolator = e.mem.u32(this.wrapping_add(INTERPOLATOR));
    if interpolator == 0 {
        return;
    }
    // The value goes into the argument's own stack word.
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, update.addr());
        let time = x87_quiet_bits(e.mem.u32(this.wrapping_add(SCALED_TIME)));
        let target = e.mem.u32(this.wrapping_add(TARGET));
        if e.vcall(
            interpolator,
            SLOT_INTERPOLATOR_UPDATE_BOOL,
            &args![time, target, slot],
        )
        .u8()
            == 0
        {
            return;
        }
        let target = e.mem.u32(this.wrapping_add(TARGET));
        let was_set = e.mem.u8(target.wrapping_add(TARGET_FLAGS)) & 1;
        let flags = e.mem.u32(target.wrapping_add(TARGET_FLAGS));
        let value_true = e.mem.u8(slot) != 0;
        if value_true {
            e.mem
                .set_u32(target.wrapping_add(TARGET_FLAGS), flags & 0xffff_fffe);
        } else {
            e.mem.set_u32(target.wrapping_add(TARGET_FLAGS), flags | 1);
        }
        if was_set != 0 && value_true {
            e.mem.set_u8(update.addr().wrapping_add(8), 1);
        }
    });
}

// Translated from 00a44ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVisController::CreateClone` (Xbox PDB): allocates 0x38 bytes and,
/// when that succeeds, runs the constructor body `00a57790` and stores the
/// vtable; then the member copy (`00a57620`) into it, and returns it. When
/// the allocation fails the member copy still runs, into address 0, and 0
/// is returned.
pub fn ni_vis_controller_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![NI_VIS_CONTROLLER_SIZE]).u32();
    if object == 0 {
        e.call(COPY_MEMBERS, &args![this, 0u32, cloning]);
        return Ptr::new(0);
    }
    e.call(BOOL_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_VIS_CONTROLLER_VTABLE);
    e.call(COPY_MEMBERS, &args![this, object, cloning]);
    Ptr::new(object)
}

// Translated from 00a44f10 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiVisController::CreateObject` (registered through
/// `NiStream::RegisterLoader` for `"NiVisController"` at `00a2dc5c`; Ghidra
/// has no function here): allocates 0x38 bytes and, when that succeeds,
/// runs the constructor body `00a57790` and stores the vtable. Returns the
/// object, or 0.
pub fn ni_vis_controller_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![NI_VIS_CONTROLLER_SIZE]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(BOOL_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_VIS_CONTROLLER_VTABLE);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a44eb0, ni_vis_controller_get_rtti() -> Ptr),
        entry!(0x00a44f40, ni_vis_controller_get_viewer_strings(Ptr, Ptr)),
        entry!(
            0x00a4_4d00,
            ni_vis_controller_get_target_bool_value(Ptr, Ptr)
        ),
        entry!(0x00a4_4d20, fn_00a44d20(Ptr, Ptr)),
        entry!(0x00a4_4d50, ni_vis_controller_link_object(Ptr, Ptr)),
        entry!(0x00a4_4e10, ni_vis_controller_update(Ptr, Ptr)),
        entry!(0x00a4_4ec0, ni_vis_controller_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a4_4f10, ni_vis_controller_create_object() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_vis_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a44eb0, &args![]).u32(), NI_VIS_CONTROLLER_RTTI);
    }

    #[test]
    fn ni_vis_controller_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a5_7800;
        const NAME_POINTER: u32 = 0x011f_3f08;
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
            ni_vis_controller_get_viewer_strings(&mut e, this, Ptr::new(strings));
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

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALWAYS_UPDATE: u32 = 0x00ee_0100;
    const UPDATE_BOOL: u32 = 0x00ee_0200;
    const COLLAPSE: u32 = 0x00ee_0300;
    const DELETE_THIS: u32 = 0x00ee_0400;

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

    /// An interpolator object whose slots are the doubles above.
    fn interpolator(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem
            .set_u32(vtable + SLOT_INTERPOLATOR_ALWAYS_UPDATE, ALWAYS_UPDATE);
        e.mem
            .set_u32(vtable + SLOT_INTERPOLATOR_UPDATE_BOOL, UPDATE_BOOL);
        e.mem.set_u32(vtable + SLOT_INTERPOLATOR_COLLAPSE, COLLAPSE);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn get_target_bool_value_reports_bit_0_clear() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        let target = e.mem.alloc(0x40);
        let value = e.mem.alloc(4);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_u32(target + TARGET_FLAGS, 0xfe);
        e.call(0x00a4_4d00, &args![this, value]);
        assert_eq!(e.mem.u8(value), 1);
        e.mem.set_u32(target + TARGET_FLAGS, 1);
        e.call(0x00a4_4d00, &args![this, value]);
        assert_eq!(e.mem.u8(value), 0);
    }

    #[test]
    fn fn_00a44d20_reads_a_link_id_only_in_old_files() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BASE_LOAD_BINARY, 0);
        logged(&mut e, &log, STREAM_READ_LINK_ID, 0);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0067);
        e.call(0x00a4_4d20, &args![0x5000u32, stream]);
        assert_eq!(
            *log.borrow(),
            [
                (BASE_LOAD_BINARY, vec![0x5000, stream]),
                (STREAM_READ_LINK_ID, vec![stream]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0068);
        e.call(0x00a4_4d20, &args![0x5000u32, stream]);
        assert_eq!(*log.borrow(), [(BASE_LOAD_BINARY, vec![0x5000, stream])]);
    }

    #[test]
    fn link_object_wraps_old_bool_data_in_an_interpolator() {
        let mut e = Engine::new();
        interlocked(&mut e);
        let log = Log::default();
        let made = interpolator(&mut e, 0);
        let old = interpolator(&mut e, 1);
        logged(&mut e, &log, BASE_LINK_OBJECT, 0);
        logged(&mut e, &log, GET_OBJECT_FROM_LINK_ID, 0x7700);
        logged(&mut e, &log, NI_ALLOCATE, made);
        logged(&mut e, &log, BOOL_INTERPOLATOR_CONSTRUCT, made);
        logged(&mut e, &log, DELETE_THIS, 0);
        logged(&mut e, &log, COLLAPSE, 0);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0067);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + INTERPOLATOR, old);
        e.call(0x00a4_4d50, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            [
                (BASE_LINK_OBJECT, vec![this, stream]),
                (GET_OBJECT_FROM_LINK_ID, vec![stream]),
                (NI_ALLOCATE, vec![0x18]),
                (BOOL_INTERPOLATOR_CONSTRUCT, vec![made, 0x7700]),
                (DELETE_THIS, vec![old]),
                (COLLAPSE, vec![made]),
            ]
        );
        assert_eq!(e.mem.u32(this + INTERPOLATOR), made);
        assert_eq!(e.mem.u32(made + 4), 1);
        // New files: only the base link.
        log.borrow_mut().clear();
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0068);
        e.call(0x00a4_4d50, &args![this, stream]);
        assert_eq!(*log.borrow(), [(BASE_LINK_OBJECT, vec![this, stream])]);
    }

    /// A controller whose interpolator's bool update writes `value` and
    /// answers `updated`; its slot `0xd4` answers `always`.
    fn controller(always: u32, updated: u32, value: u8) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(FORCED_UPDATE_TIME, 4);
        e.mem.set_f32(FORCED_UPDATE_TIME, -f32::MAX);
        let interp = interpolator(&mut e, 1);
        logged(&mut e, &log, ALWAYS_UPDATE, always);
        let l = log.clone();
        e.register_double(UPDATE_BOOL, move |e, a| {
            l.borrow_mut().push((UPDATE_BOOL, a.to_vec()));
            e.mem.set_u8(a[3], value);
            ret(updated)
        });
        let target = e.mem.alloc(0x40);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + INTERPOLATOR, interp);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_f32(this + SCALED_TIME, 1.5);
        (e, log, this, target)
    }

    #[test]
    fn update_clears_the_bit_for_a_true_value_and_flags_the_change() {
        let (mut e, log, this, target) = controller(0, 1, 1);
        e.mem.set_u8(this + FLAGS, 0x20);
        e.mem.set_u32(target + TARGET_FLAGS, 0x0000_0101);
        let update = e.mem.alloc(0x10);
        e.call(0x00a4_4e10, &args![this, update]);
        assert_eq!(e.mem.f32(this + SCALED_TIME), -f32::MAX);
        assert_eq!(log.borrow()[0].1[1..3], [(-f32::MAX).to_bits(), target]);
        assert_eq!(e.mem.u32(target + TARGET_FLAGS), 0x100);
        assert_eq!(e.mem.u8(update + 8), 1);
    }

    #[test]
    fn update_sets_the_bit_for_a_false_value() {
        let (mut e, log, this, target) = controller(0, 1, 0);
        logged(&mut e, &log, DONT_DO_UPDATE, 0);
        let update = e.mem.alloc(0x10);
        e.mem.set_f32(update, 2.0);
        e.call(0x00a4_4e10, &args![this, update]);
        assert_eq!(
            log.borrow()[0],
            (DONT_DO_UPDATE, vec![this, 2.0f32.to_bits()])
        );
        assert_eq!(e.mem.u32(target + TARGET_FLAGS), 1);
        assert_eq!(e.mem.u8(update + 8), 0);
        // Already set and a true value that was not taken: nothing changes.
        let (mut e, log, this, target) = controller(0, 0, 1);
        logged(&mut e, &log, DONT_DO_UPDATE, 0);
        e.mem.set_u32(target + TARGET_FLAGS, 1);
        e.call(0x00a4_4e10, &args![this, update]);
        assert_eq!(e.mem.u32(target + TARGET_FLAGS), 1);
    }

    #[test]
    fn update_skipped_unless_the_interpolator_insists() {
        let (mut e, log, this, target) = controller(0, 1, 0);
        logged(&mut e, &log, DONT_DO_UPDATE, 1);
        let update = e.mem.alloc(0x10);
        e.call(0x00a4_4e10, &args![this, update]);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(target + TARGET_FLAGS), 0);
        logged(&mut e, &log, ALWAYS_UPDATE, 1);
        e.call(0x00a4_4e10, &args![this, update]);
        assert_eq!(e.mem.u32(target + TARGET_FLAGS), 1);
        e.mem.set_u32(this + INTERPOLATOR, 0);
        log.borrow_mut().clear();
        e.call(0x00a4_4e10, &args![this, update]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn create_object_and_create_clone_build_the_controller() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x38);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, BOOL_INTERP_CONTROLLER_CONSTRUCT, block);
        logged(&mut e, &log, COPY_MEMBERS, 0);
        assert_eq!(e.call(0x00a4_4f10, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_VIS_CONTROLLER_VTABLE);
        let this = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00a4_4ec0, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(log.borrow()[4], (COPY_MEMBERS, vec![this, block, 0x9000]));
        logged(&mut e, &log, NI_ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a4_4f10, &args![]).u32(), 0);
        assert_eq!(e.call(0x00a4_4ec0, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x38]),
                (NI_ALLOCATE, vec![0x38]),
                (COPY_MEMBERS, vec![this, 0, 0x9000]),
            ]
        );
    }
}
