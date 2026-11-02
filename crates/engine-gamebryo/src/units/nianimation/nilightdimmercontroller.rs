//! `NiAnimation/nilightdimmercontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLightDimmerController::ms_RTTI` (the class's `NiRTTI` record).
const NI_LIGHT_DIMMER_CONTROLLER_RTTI: u32 = 0x011f_3fa0;

// Translated from 00a4cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_light_dimmer_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LIGHT_DIMMER_CONTROLLER_RTTI)
}

/// `00a57c70`: the jump target of `00a4cf40`.
const JUMP_TARGET_00A4CF40: u32 = 0x00a5_7c70;

// Translated from 00a4cf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::LoadBinary` (Xbox PDB): a 5-byte jump to `00a57c70` (engine map: `NiFloatInterpController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiLightDimmerController@@UAAXAAVNiStream@@@Z`.
pub fn ni_light_dimmer_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A4CF40, &args![this, stream]);
}

/// `NiLightDimmerController`'s vtable (the address its constructors and destructor store)
/// and size.
const NI_LIGHT_DIMMER_CONTROLLER_VTABLE: u32 = 0x0109_8ccc;
const NI_LIGHT_DIMMER_CONTROLLER_SIZE: u32 = 0x38;
/// Fields as the PC code uses them (Xbox PDB names): `NiTimeController`'s flags byte +0x08,
/// `m_fScaledTime` +0x28, `m_pkTarget` +0x2c and `NiSingleInterpController::m_spInterpolator`
/// +0x34; the light's revision word +0xa8 and dimmer float +0xc4.
const TIME_FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const INTERPOLATOR: u32 = 0x34;
const LIGHT_REVISION: u32 = 0xa8;
const LIGHT_DIMMER: u32 = 0xc4;
/// The base `IsEqual` (`00a57c10`), `CopyMembers` (`00a57620`) and `GetViewerStrings`
/// (`00a576c0`), the `NiFloatInterpController` constructor (`00a57640`) and the
/// `NiTimeController` destructor (`00a6d290`), all thiscall.
const BASE_IS_EQUAL: u32 = 0x00a5_7c10;
const BASE_COPY_MEMBERS: u32 = 0x00a5_7620;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a5_76c0;
const FLOAT_INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a5_7640;
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
/// `NiTimeController::DontDoUpdate` (`00a36250`, thiscall, the float time) and the float a
/// forced controller's scaled time becomes (`01096324`).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
const FORCED_TIME: u32 = 0x0109_6324;
/// `00653290` (thiscall on an object, an `NiRTTI` record): the object when it is of that
/// class, else 0; `NiLight::ms_RTTI` (`011f4a28`).
const DYNAMIC_CAST: u32 = 0x0065_3290;
const NI_LIGHT_RTTI: u32 = 0x011f_4a28;
/// `00aa13e0` (cdecl, one word): allocates from the Ni allocator; the sized delete
/// (`00aa1460`, cdecl).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `InterlockedDecrement` (import slot), `DeleteThis` (virtual slot 1) and the
/// interpolator's slots `0xd4` (always update) and `0x9c` (float update).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_ALWAYS_UPDATE: u32 = 0xd4;
const SLOT_UPDATE_FLOAT: u32 = 0x9c;
/// `NiGetViewerString` (`00a7d330`, cdecl) and the viewer-string array's resize
/// (`0096ad30`) and `SetAt` (`0096ae90`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a4cf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::IsEqual` (Xbox PDB): whether the base's `IsEqual` (`00a57c10`)
/// succeeded.
pub fn ni_light_dimmer_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(BASE_IS_EQUAL, &args![this, other]).bool()
}

// Translated from 00a4cf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::GetTargetFloatValue` (Xbox PDB): with a target, its dimmer
/// (+0xc4) into `value` (through the x87 stack).
pub fn ni_light_dimmer_controller_get_target_float_value(e: &mut Engine, this: Ptr, value: Ptr) {
    let target = e.mem.u32(this.addr().wrapping_add(TARGET));
    if target == 0 {
        return;
    }
    let dimmer = x87_quiet_bits(e.mem.u32(target.wrapping_add(LIGHT_DIMMER)));
    e.mem.set_u32(value.addr(), dimmer);
}

// Translated from 00a4cfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::Update` (Xbox PDB): a forced controller (flag bit 5 of the byte
/// at +8) sets its scaled time to the float at `01096324`; otherwise, when `DontDoUpdate`
/// (with the update's time) says so, it stops unless the interpolator's slot `0xd4` answers
/// true. The interpolator's float update (slot `0x9c`: scaled time, target, and the
/// argument's own stack word as the out float, which still holds the update's address when
/// the interpolator writes nothing) gives the dimmer; with a target, the light (`00653290`
/// with `NiLight`'s record) counts a revision (+0xa8) and takes it (+0xc4).
pub fn ni_light_dimmer_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
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
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, update.addr());
        let time = x87_quiet_bits(e.mem.u32(object.wrapping_add(SCALED_TIME)));
        let target = e.mem.u32(object.wrapping_add(TARGET));
        if !e
            .vcall(interpolator, SLOT_UPDATE_FLOAT, &args![time, target, slot])
            .bool()
        {
            return;
        }
        let target = e.mem.u32(object.wrapping_add(TARGET));
        if target == 0 {
            return;
        }
        let light = e.call(DYNAMIC_CAST, &args![target, NI_LIGHT_RTTI]).u32();
        let dimmer = x87_quiet_bits(e.mem.u32(slot));
        let revision = e.mem.u32(light.wrapping_add(LIGHT_REVISION));
        e.mem
            .set_u32(light.wrapping_add(LIGHT_REVISION), revision.wrapping_add(1));
        e.mem.set_u32(light.wrapping_add(LIGHT_DIMMER), dimmer);
    });
}

// Translated from 00a4d030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable,
/// releases the interpolator, runs the `NiTimeController` destructor (`00a6d290`) and frees
/// the 0x38 bytes when bit 0 of `flags` is set. Returns `this`.
pub fn ni_light_dimmer_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem
        .set_u32(this.addr(), NI_LIGHT_DIMMER_CONTROLLER_VTABLE);
    let interpolator = e.mem.u32(this.addr().wrapping_add(INTERPOLATOR));
    if interpolator != 0
        && e.call(INTERLOCKED_DECREMENT, &args![interpolator.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(interpolator, SLOT_DELETE_THIS, &args![]);
    }
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(
            NI_SIZED_DELETE,
            &args![this, NI_LIGHT_DIMMER_CONTROLLER_SIZE],
        );
    }
    this
}

// Translated from 00a4d080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::CreateClone` (Xbox PDB): a new controller (0x38 bytes; the
/// `NiFloatInterpController` constructor `00a57640`, then the vtable) and the base's
/// `CopyMembers` (`00a57620`) into it; when the allocation fails the member copy runs into
/// address 0 and 0 is returned.
pub fn ni_light_dimmer_controller_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> u32 {
    let object = e
        .call(NI_ALLOCATE, &args![NI_LIGHT_DIMMER_CONTROLLER_SIZE])
        .u32();
    if object == 0 {
        e.call(BASE_COPY_MEMBERS, &args![this, 0u32, cloning]);
        return 0;
    }
    e.call(FLOAT_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_LIGHT_DIMMER_CONTROLLER_VTABLE);
    e.call(BASE_COPY_MEMBERS, &args![this, object, cloning]);
    object
}

// Translated from 00a4d0d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::CreateObject` (registered as the `"NiLightDimmerController"`
/// loader at `00a2bde0`; Ghidra has no function here): allocates 0x38 bytes and, when that
/// succeeds, runs the `NiFloatInterpController` constructor (`00a57640`) and stores the
/// vtable. Returns it, or 0.
pub fn ni_light_dimmer_controller_create_object(e: &mut Engine) -> u32 {
    let object = e
        .call(NI_ALLOCATE, &args![NI_LIGHT_DIMMER_CONTROLLER_SIZE])
        .u32();
    if object == 0 {
        return 0;
    }
    e.call(FLOAT_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_LIGHT_DIMMER_CONTROLLER_VTABLE);
    object
}

// Translated from 00a4d100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::GetViewerStrings` (Xbox PDB): the base's `GetViewerStrings`
/// (`00a576c0`), then the class name (the first word of the `NiRTTI` record at `011f3fa0`,
/// through `NiGetViewerString`) appended to the array (grown by its step when full), held in
/// the argument's own stack word.
pub fn ni_light_dimmer_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    let class_name = e.mem.u32(NI_LIGHT_DIMMER_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4cf90, ni_light_dimmer_controller_get_rtti() -> Ptr),
        entry!(
            0x00a4_cf40,
            ni_light_dimmer_controller_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_cf50, ni_light_dimmer_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_cf70,
            ni_light_dimmer_controller_get_target_float_value(Ptr, Ptr)
        ),
        entry!(0x00a4_cfa0, ni_light_dimmer_controller_update(Ptr, Ptr)),
        entry!(
            0x00a4_d030,
            ni_light_dimmer_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a4_d080,
            ni_light_dimmer_controller_create_clone(Ptr, Ptr) -> u32
        ),
        entry!(0x00a4_d0d0, ni_light_dimmer_controller_create_object() -> u32),
        entry!(
            0x00a4_d100,
            ni_light_dimmer_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_light_dimmer_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4cf90, &args![]).u32(),
            NI_LIGHT_DIMMER_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a4cf40_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_cf40;
        const TARGET: u32 = 0x00a5_7c70;
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

    #[test]
    fn is_equal_answers_the_base() {
        let mut e = Engine::new();
        let base = Rc::new(Cell::new(0x100u32));
        answers(&mut e, BASE_IS_EQUAL, &base);
        assert!(!e.call(0x00a4_cf50, &args![0x1000u32, 0x2000u32]).bool());
        base.set(1);
        assert!(e.call(0x00a4_cf50, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn get_target_float_value_reads_the_dimmer() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 9.0);
        e.call(0x00a4_cf70, &args![this, value]);
        assert_eq!(e.mem.f32(value), 9.0);
        let light = e.mem.alloc(0xc8);
        e.mem.set_u32(light + LIGHT_DIMMER, 0x7f80_0004);
        e.mem.set_u32(this + TARGET, light);
        e.call(0x00a4_cf70, &args![this, value]);
        assert_eq!(e.mem.u32(value), 0x7fc0_0004);
    }

    #[test]
    fn update_sets_the_light_dimmer() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(FORCED_TIME, 4);
        e.mem.set_f32(FORCED_TIME, 9.0);
        let skip = Rc::new(Cell::new(1u32));
        answers(&mut e, DONT_DO_UPDATE, &skip);
        let always = Rc::new(Cell::new(0u32));
        answers(&mut e, 0x00ee_8000, &always);
        let writes = Rc::new(Cell::new(true));
        let write = writes.clone();
        let updated = Rc::new(Cell::new(1u32));
        let answer = updated.clone();
        let seen = Log::default();
        let record = seen.clone();
        e.register_double(0x00ee_8010, move |e, a| {
            record.borrow_mut().push((0x00ee_8010, a.to_vec()));
            if write.get() {
                e.mem.set_f32(a[3], 0.25);
            }
            ret(answer.get())
        });
        let light = e.mem.alloc(0xc8);
        logged(&mut e, &log, DYNAMIC_CAST, light);
        let interpolator = object(
            &mut e,
            8,
            &[
                (SLOT_ALWAYS_UPDATE, 0x00ee_8000),
                (SLOT_UPDATE_FLOAT, 0x00ee_8010),
            ],
        );
        let this = e.mem.alloc(0x38);
        let update = e.mem.alloc(8);
        e.mem.set_f32(update, 3.0);
        e.mem.set_f32(this + SCALED_TIME, 4.0);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_u32(this + TARGET, 0x7000);
        // DontDoUpdate says skip and the interpolator does not always update.
        e.call(0x00a4_cfa0, &args![this, update]);
        assert!(seen.borrow().is_empty());
        always.set(1);
        e.call(0x00a4_cfa0, &args![this, update]);
        assert_eq!(e.mem.f32(light + LIGHT_DIMMER), 0.25);
        assert_eq!(e.mem.u32(light + LIGHT_REVISION), 1);
        assert_eq!(calls(&log, DYNAMIC_CAST), [vec![0x7000, NI_LIGHT_RTTI]]);
        let first = seen.borrow()[0].1.clone();
        assert_eq!(&first[..3], [interpolator, 4.0f32.to_bits(), 0x7000]);
        // Forced, and an interpolator that writes nothing: the stack word still holds the
        // update's address.
        e.mem.set_u8(this + TIME_FLAGS, 0x20);
        writes.set(false);
        e.call(0x00a4_cfa0, &args![this, update]);
        assert_eq!(e.mem.f32(this + SCALED_TIME), 9.0);
        assert_eq!(e.mem.u32(light + LIGHT_DIMMER), update);
        assert_eq!(e.mem.u32(light + LIGHT_REVISION), 2);
        // A failed update, no target or no interpolator change nothing.
        updated.set(0);
        e.call(0x00a4_cfa0, &args![this, update]);
        updated.set(1);
        e.mem.set_u32(this + TARGET, 0);
        e.call(0x00a4_cfa0, &args![this, update]);
        e.mem.set_u32(this + INTERPOLATOR, 0);
        e.call(0x00a4_cfa0, &args![this, update]);
        assert_eq!(e.mem.u32(light + LIGHT_REVISION), 2);
        assert_eq!(seen.borrow().len(), 4);
    }

    #[test]
    fn scalar_deleting_destructor_releases_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, INTERLOCKED_DECREMENT, 0);
        logged(&mut e, &log, TIME_CONTROLLER_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        logged(&mut e, &log, 0x00ee_8100, 0);
        let interpolator = object(&mut e, 8, &[(SLOT_DELETE_THIS, 0x00ee_8100)]);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        assert_eq!(e.call(0x00a4_d030, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_LIGHT_DIMMER_CONTROLLER_VTABLE);
        assert_eq!(calls(&log, 0x00ee_8100), [vec![interpolator]]);
        assert!(calls(&log, NI_SIZED_DELETE).is_empty());
        e.mem.set_u32(this + INTERPOLATOR, 0);
        e.call(0x00a4_d030, &args![this, 1u32]);
        assert_eq!(calls(&log, NI_SIZED_DELETE), [vec![this, 0x38]]);
        assert_eq!(calls(&log, TIME_CONTROLLER_DESTRUCT).len(), 2);
    }

    #[test]
    fn create_clone_and_create_object_build_the_controller() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, FLOAT_INTERP_CONTROLLER_CONSTRUCT, 0);
        logged(&mut e, &log, BASE_COPY_MEMBERS, 0);
        let block = Rc::new(Cell::new(0u32));
        answers(&mut e, NI_ALLOCATE, &block);
        assert_eq!(e.call(0x00a4_d0d0, &args![]).u32(), 0);
        assert_eq!(e.call(0x00a4_d080, &args![0x1000u32, 0x2000u32]).u32(), 0);
        let object = e.mem.alloc(0x38);
        block.set(object);
        assert_eq!(e.call(0x00a4_d0d0, &args![]).u32(), object);
        assert_eq!(e.mem.u32(object), NI_LIGHT_DIMMER_CONTROLLER_VTABLE);
        e.mem.set_u32(object, 0);
        assert_eq!(
            e.call(0x00a4_d080, &args![0x1000u32, 0x2000u32]).u32(),
            object
        );
        assert_eq!(e.mem.u32(object), NI_LIGHT_DIMMER_CONTROLLER_VTABLE);
        assert_eq!(
            calls(&log, BASE_COPY_MEMBERS),
            [vec![0x1000, 0, 0x2000], vec![0x1000, object, 0x2000]]
        );
        assert_eq!(
            calls(&log, FLOAT_INTERP_CONTROLLER_CONSTRUCT),
            [vec![object], vec![object]]
        );
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(NI_LIGHT_DIMMER_CONTROLLER_RTTI, 4);
        e.mem.set_u32(NI_LIGHT_DIMMER_CONTROLLER_RTTI, 0x0155_0000);
        logged(&mut e, &log, BASE_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x0166_0000);
        logged(&mut e, &log, VIEWER_STRINGS_GROW, 0);
        e.register(VIEWER_STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0166_0000);
            Ret::default()
        });
        let this = e.mem.alloc(0x38);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xa, 1);
        e.call(0x00a4_d100, &args![this, strings]);
        assert_eq!(calls(&log, BASE_GET_VIEWER_STRINGS), [vec![this, strings]]);
        assert_eq!(calls(&log, NI_GET_VIEWER_STRING), [vec![0x0155_0000]]);
        assert!(calls(&log, VIEWER_STRINGS_GROW).is_empty());
    }
}
