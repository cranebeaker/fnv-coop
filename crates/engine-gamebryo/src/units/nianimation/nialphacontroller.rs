//! `NiAnimation/nialphacontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAlphaController::ms_RTTI` (the class's `NiRTTI` record).
const NI_ALPHA_CONTROLLER_RTTI: u32 = 0x011f_4084;

// Translated from 00a56a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_alpha_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_ALPHA_CONTROLLER_RTTI)
}

// Translated from 00a56b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaController::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a576c0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f4084`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_alpha_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_76c0;
    const CLASS_NAME_POINTER: u32 = 0x011f_4084;
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

/// The vtable of `NiAlphaController` (RTTI `.?AVNiAlphaController@@`).
const NI_ALPHA_CONTROLLER_VTABLE: u32 = 0x0109_a76c;
/// `NiAlphaController` size (the `CreateObject` allocation).
const NI_ALPHA_CONTROLLER_SIZE: u32 = 0x38;

/// `NiTimeController` fields as the PC code uses them: the flags byte +0x08
/// (bit 5 forces the update), the last scaled time +0x28 and the target
/// +0x2c; `NiSingleInterpController::m_spInterpolator` (Xbox PDB) +0x34.
const FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
/// The target's alpha float +0x3c and its revision counter +0x44 (offsets
/// as `Update` writes them; the target class is not named in the code).
const TARGET_ALPHA: u32 = 0x3c;
const TARGET_REVISION: u32 = 0x44;

/// The scaled time a forced update uses (a float in `.rdata`).
const FORCED_UPDATE_TIME: u32 = 0x0109_6324;

/// `NiTimeController::DontDoUpdate` (`00a36250`, `thiscall`, one float).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The interpolator's virtual slots `Update` uses: `0xd4` (no arguments,
/// a bool) and the float update `0x9c` `(time, target, &value)` (a bool).
const SLOT_INTERPOLATOR_ALWAYS_UPDATE: u32 = 0xd4;
const SLOT_INTERPOLATOR_UPDATE_FLOAT: u32 = 0x9c;
/// `00a57640` (unplaced, `run_00a57640`): the constructor body
/// `CreateObject` and `CreateClone` run (it stores the
/// `NiFloatInterpController` vtable).
const FLOAT_INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a5_7640;
/// `00a57620` (unnamed in the engine map; `thiscall`, `(destination,
/// cloning)`): the member copy `CreateClone` runs.
const COPY_MEMBERS: u32 = 0x00a5_7620;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;

// Translated from 00a56a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaController::Update` (Xbox PDB). A forced update (flags bit 5)
/// sets the scaled time to the float at `01096324`; otherwise
/// `DontDoUpdate` runs on the time (the first float of `update`) and, when
/// it says to skip, the update only goes on when there is an interpolator
/// whose slot `0xd4` says so. Then the interpolator's float update (slot
/// `0x9c`) runs with the scaled time, the target and the argument's own
/// stack word as the value; on success the target's revision counter
/// (+0x44) goes up and its alpha (+0x3c) takes the value.
pub fn ni_alpha_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
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
            SLOT_INTERPOLATOR_UPDATE_FLOAT,
            &args![time, target, slot],
        )
        .u8()
            != 0
        {
            let target = e.mem.u32(this.wrapping_add(TARGET));
            let revision = e.mem.u32(target.wrapping_add(TARGET_REVISION));
            e.mem.set_u32(
                target.wrapping_add(TARGET_REVISION),
                revision.wrapping_add(1),
            );
            let value = x87_quiet_bits(e.mem.u32(slot));
            e.mem.set_u32(target.wrapping_add(TARGET_ALPHA), value);
        }
    });
}

// Translated from 00a56ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaController::CreateClone` (Xbox PDB): allocates 0x38 bytes and,
/// when that succeeds, runs the constructor body `00a57640` and stores the
/// vtable; then the member copy (`00a57620`) into it, and returns it. When
/// the allocation fails the member copy still runs, into address 0, and 0
/// is returned.
pub fn ni_alpha_controller_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![NI_ALPHA_CONTROLLER_SIZE]).u32();
    if object == 0 {
        e.call(COPY_MEMBERS, &args![this, 0u32, cloning]);
        return Ptr::new(0);
    }
    e.call(FLOAT_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_ALPHA_CONTROLLER_VTABLE);
    e.call(COPY_MEMBERS, &args![this, object, cloning]);
    Ptr::new(object)
}

// Translated from 00a56b30 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiAlphaController::CreateObject` (registered through
/// `NiStream::RegisterLoader` for `"NiAlphaController"` at `00a2d8ad`;
/// Ghidra has no function here): allocates 0x38 bytes and, when that
/// succeeds, runs the constructor body `00a57640` and stores the vtable.
/// Returns the object, or 0.
pub fn ni_alpha_controller_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![NI_ALPHA_CONTROLLER_SIZE]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(FLOAT_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_ALPHA_CONTROLLER_VTABLE);
    Ptr::new(object)
}

/// `NiMaterialProperty::ms_RTTI` (`011f4a5c`; `NiMaterialProperty::GetRTTI`
/// `00a756c0` answers it).
const NI_MATERIAL_PROPERTY_RTTI: u32 = 0x011f_4a5c;

// Translated from 00a569f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaController::GetTargetFloatValue` (Xbox PDB): the target's alpha
/// (+0x3C, through `FLD`/`FSTP`) stored to `value`.
pub fn ni_alpha_controller_get_target_float_value(e: &mut Engine, this: Ptr, value: Ptr) {
    let target = e.mem.u32(this.addr().wrapping_add(TARGET));
    let alpha = x87_quiet_bits(e.mem.u32(target.wrapping_add(TARGET_ALPHA)));
    e.mem.set_u32(value.addr(), alpha);
}

// Translated from 00bc3050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaController::InterpTargetIsCorrectType` (Xbox PDB; `thiscall`,
/// one stack word): whether the target is an `NiMaterialProperty` (its
/// `GetRTTI`, vtable +8, and then each base, `NiRTTI` +4, compared with
/// `NiMaterialProperty::ms_RTTI`); false for a null target. `this` is not
/// read.
pub fn ni_alpha_controller_interp_target_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    target: Ptr,
) -> bool {
    if target.is_null() {
        return false;
    }
    let mut class = e.vcall(target.addr(), 8, &args![]).u32();
    while class != 0 {
        if class == NI_MATERIAL_PROPERTY_RTTI {
            return true;
        }
        class = e.mem.u32(class.wrapping_add(4));
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a56a80, ni_alpha_controller_get_rtti() -> Ptr),
        entry!(0x00a56b60, ni_alpha_controller_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a5_6a00, ni_alpha_controller_update(Ptr, Ptr)),
        entry!(0x00a5_6ae0, ni_alpha_controller_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a5_6b30, ni_alpha_controller_create_object() -> Ptr),
        entry!(
            0x00a5_69f0,
            ni_alpha_controller_get_target_float_value(Ptr, Ptr)
        ),
        entry!(
            0x00bc_3050,
            ni_alpha_controller_interp_target_is_correct_type(Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_alpha_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a56a80, &args![]).u32(), NI_ALPHA_CONTROLLER_RTTI);
    }

    #[test]
    fn ni_alpha_controller_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a5_76c0;
        const NAME_POINTER: u32 = 0x011f_4084;
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
            ni_alpha_controller_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
    const UPDATE_FLOAT: u32 = 0x00ee_0200;

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

    /// A controller with an interpolator (slot `0xd4` answering `always`,
    /// slot `0x9c` writing 0.25 and answering `updated`) and a target.
    fn controller(always: u32, updated: u32) -> (Engine, Log, u32, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(FORCED_UPDATE_TIME, 4);
        e.mem.set_f32(FORCED_UPDATE_TIME, -f32::MAX);
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + SLOT_INTERPOLATOR_ALWAYS_UPDATE, ALWAYS_UPDATE);
        e.mem
            .set_u32(vtable + SLOT_INTERPOLATOR_UPDATE_FLOAT, UPDATE_FLOAT);
        let interpolator = e.mem.alloc(0x18);
        e.mem.set_u32(interpolator, vtable);
        logged(&mut e, &log, ALWAYS_UPDATE, always);
        let l = log.clone();
        e.register_double(UPDATE_FLOAT, move |e, a| {
            l.borrow_mut().push((UPDATE_FLOAT, a.to_vec()));
            e.mem.set_f32(a[3], 0.25);
            ret(updated)
        });
        let target = e.mem.alloc(0x48);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_f32(this + SCALED_TIME, 1.5);
        (e, log, this, interpolator, target)
    }

    #[test]
    fn update_forced_sets_the_time_and_writes_the_alpha() {
        let (mut e, log, this, interpolator, target) = controller(0, 1);
        e.mem.set_u8(this + FLAGS, 0x20);
        let update = e.mem.alloc(4);
        e.call(0x00a5_6a00, &args![this, update]);
        assert_eq!(e.mem.f32(this + SCALED_TIME), -f32::MAX);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0].1[..3],
            [interpolator, (-f32::MAX).to_bits(), target]
        );
        assert_eq!(e.mem.f32(target + TARGET_ALPHA), 0.25);
        assert_eq!(e.mem.u32(target + TARGET_REVISION), 1);
    }

    #[test]
    fn update_skipped_unless_the_interpolator_insists() {
        let (mut e, log, this, interpolator, target) = controller(0, 1);
        logged(&mut e, &log, DONT_DO_UPDATE, 1);
        let update = e.mem.alloc(4);
        e.mem.set_f32(update, 2.0);
        e.call(0x00a5_6a00, &args![this, update]);
        assert_eq!(
            *log.borrow(),
            [
                (DONT_DO_UPDATE, vec![this, 2.0f32.to_bits()]),
                (ALWAYS_UPDATE, vec![interpolator]),
            ]
        );
        assert_eq!(e.mem.u32(target + TARGET_REVISION), 0);
        // Slot 0xd4 says yes: the update runs.
        logged(&mut e, &log, ALWAYS_UPDATE, 1);
        e.call(0x00a5_6a00, &args![this, update]);
        assert_eq!(e.mem.u32(target + TARGET_REVISION), 1);
        // No interpolator: nothing after DontDoUpdate.
        e.mem.set_u32(this + INTERPOLATOR, 0);
        log.borrow_mut().clear();
        e.call(0x00a5_6a00, &args![this, update]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn update_runs_when_not_skipped_and_keeps_a_failed_value_out() {
        let (mut e, log, this, _, target) = controller(0, 0);
        logged(&mut e, &log, DONT_DO_UPDATE, 0);
        let update = e.mem.alloc(4);
        e.call(0x00a5_6a00, &args![this, update]);
        assert_eq!(log.borrow()[1].0, UPDATE_FLOAT);
        assert_eq!(log.borrow()[1].1[1], 1.5f32.to_bits());
        assert_eq!(e.mem.u32(target + TARGET_REVISION), 0);
        assert_eq!(e.mem.f32(target + TARGET_ALPHA), 0.0);
    }

    #[test]
    fn create_object_and_create_clone_build_the_controller() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x38);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, FLOAT_INTERP_CONTROLLER_CONSTRUCT, block);
        logged(&mut e, &log, COPY_MEMBERS, 0);
        assert_eq!(e.call(0x00a5_6b30, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_ALPHA_CONTROLLER_VTABLE);
        let this = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00a5_6ae0, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x38]),
                (FLOAT_INTERP_CONTROLLER_CONSTRUCT, vec![block]),
                (NI_ALLOCATE, vec![0x38]),
                (FLOAT_INTERP_CONTROLLER_CONSTRUCT, vec![block]),
                (COPY_MEMBERS, vec![this, block, 0x9000]),
            ]
        );
        logged(&mut e, &log, NI_ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a5_6b30, &args![]).u32(), 0);
        assert_eq!(e.call(0x00a5_6ae0, &args![this, 0x9000u32]).u32(), 0);
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

#[cfg(test)]
mod a07_tests {
    use super::*;

    #[test]
    fn get_target_float_value_reads_the_alpha() {
        let mut e = Engine::new();
        let target = e.mem.alloc(0x48);
        e.mem.set_u32(target + TARGET_ALPHA, 0x7f80_0002);
        let this = e.mem.alloc(NI_ALPHA_CONTROLLER_SIZE);
        e.mem.set_u32(this + TARGET, target);
        let value = e.mem.alloc(4);
        e.call(0x00a5_69f0, &args![this, value]);
        assert_eq!(e.mem.u32(value), 0x7fc0_0002, "quieted by FLD");
        e.mem.set_f32(target + TARGET_ALPHA, 0.5);
        e.call(0x00a5_69f0, &args![this, value]);
        assert_eq!(e.mem.f32(value), 0.5);
    }

    #[test]
    fn interp_target_is_correct_type_walks_the_rtti_chain() {
        const VTABLE: u32 = 0x7400_0000;
        const GET_RTTI: u32 = 0x7500_0008;
        let mut e = Engine::new();
        e.put_vtable(VTABLE, &[0, 0, GET_RTTI]);
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent + 4, NI_MATERIAL_PROPERTY_RTTI);
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, VTABLE);
        e.mem.set_u32(target + 4, parent);
        assert!(e.call(0x00bc_3050, &args![0u32, target]).bool());
        e.mem.set_u32(parent + 4, 0);
        assert!(!e.call(0x00bc_3050, &args![0u32, target]).bool());
        assert!(!e.call(0x00bc_3050, &args![0u32, 0u32]).bool());
    }
}
