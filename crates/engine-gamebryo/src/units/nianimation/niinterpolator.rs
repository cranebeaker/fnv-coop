//! `NiAnimation/niinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_INTERPOLATOR_RTTI: u32 = 0x011f_4200;

// Translated from 00a57070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_INTERPOLATOR_RTTI)
}

/// `00a5d610`: the jump target of `00a88a40`.
const JUMP_TARGET_00A88A40: u32 = 0x00a5_d610;

// Translated from 00a88a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a5d610` (engine map: `NiObject::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A88A40, &args![this, stream]);
}

/// The vtable of `NiInterpolator` (RTTI `.?AVNiInterpolator@@`).
const NI_INTERPOLATOR_VTABLE: u32 = 0x0109_a924;
/// The `float` the constructor gives `m_fLastTime`.
const INITIAL_LAST_TIME: u32 = 0x0101_5f5c;
/// `NiInterpolator::m_fLastTime` (Xbox PDB) +0x08.
const LAST_TIME: u32 = 0x08;
/// `NiObject::NiObject` / `~NiObject` (`00a5d3a0` / `00a5d3d0`, `NiMain/niobject.cpp`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;

// Translated from 00a56ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::NiInterpolator` (Xbox PDB): the `NiObject` constructor,
/// `m_fLastTime` from the `float` at `01015f5c` (an x87 `FLD`/`FSTP` copy),
/// then the `NiInterpolator` vtable. Returns `this`.
pub fn ni_interpolator_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    let last_time = e.global::<u32>(INITIAL_LAST_TIME);
    e.mem.set_u32(
        this.addr().wrapping_add(LAST_TIME),
        x87_quiet_bits(last_time),
    );
    e.mem.set_u32(this.addr(), NI_INTERPOLATOR_VTABLE);
    this
}

// Translated from 00a57010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::~NiInterpolator` (unnamed in the engine map): stores the
/// `NiInterpolator` vtable, then the `NiObject` destructor (`00a5d3d0`, a
/// tail jump).
pub fn ni_interpolator_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_INTERPOLATOR_VTABLE);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a57020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niinterpolator.cpp`): a virtual function of three stack
/// words that reads nothing and answers false (the `NiInterpolator::Update` overloads'
/// default body, shared by several vtables' slots).
pub fn fn_00a57020(
    _e: &mut Engine,
    _unused_0: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> bool {
    false
}

// Translated from 00a57030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::GetActiveTimeRange` (Xbox PDB; `this` unread): the range is 0.0 to 0.0.
pub fn ni_interpolator_get_active_time_range(e: &mut Engine, _unused_0: Ptr, start: Ptr, end: Ptr) {
    e.mem.set_f32(start.addr(), 0.0);
    e.mem.set_f32(end.addr(), 0.0);
}

/// `NiObject::Clone` (`00a5d680`, thiscall, no arguments) and the interpolator's
/// `SetUpDependencies` slot (`0xd0`).
const NI_OBJECT_CLONE: u32 = 0x00a5_d680;
const SLOT_SET_UP_DEPENDENCIES: u32 = 0xd0;

// Translated from 00a57050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::GetSequenceInterpolator` (Xbox PDB; the two range floats are unread): a
/// clone (`NiObject::Clone`) whose slot `0xd0` runs; the clone is returned.
pub fn ni_interpolator_get_sequence_interpolator(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) -> u32 {
    let clone = e.call(NI_OBJECT_CLONE, &args![this]).u32();
    e.vcall(clone, SLOT_SET_UP_DEPENDENCIES, &args![]);
    clone
}

/// The sized delete (`00aa1460`, cdecl, `(block, size)`) and `NiInterpolator`'s size.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
const NI_INTERPOLATOR_SIZE: u32 = 0x0c;

// Translated from 00a57080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable, runs the
/// `NiObject` destructor (`00a5d3d0`) and, when bit 0 of `flags` is set, the sized delete of
/// 0xC bytes. Returns `this`.
pub fn ni_interpolator_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_INTERPOLATOR_VTABLE);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, NI_INTERPOLATOR_SIZE]);
    }
    this
}

/// `NiObject::GetViewerStrings` (`00a5d6e0`, thiscall, the array), `NiGetViewerString`
/// (`00a7d330`, cdecl) and the viewer-string array's resize (`0096ad30`) and `SetAt`
/// (`0096ae90`).
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a570b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpolator::GetViewerStrings` (Xbox PDB): `NiObject::GetViewerStrings`, then the class
/// name (the first word of the `NiRTTI` record at `011f4200`, through `NiGetViewerString`)
/// appended to the array (grown by its step when full), held in the argument's own stack
/// word.
pub fn ni_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    let class_name = e.mem.u32(NI_INTERPOLATOR_RTTI);
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
        entry!(0x00a57070, ni_interpolator_get_rtti() -> Ptr),
        entry!(0x00a8_8a40, ni_interpolator_load_binary(Ptr, Ptr)),
        entry!(0x00a5_6ff0, ni_interpolator_construct(Ptr) -> Ptr),
        entry!(0x00a5_7010, ni_interpolator_destruct(Ptr)),
        entry!(0x00a5_7020, fn_00a57020(Ptr, u32, u32, u32) -> bool),
        entry!(
            0x00a5_7030,
            ni_interpolator_get_active_time_range(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a5_7050,
            ni_interpolator_get_sequence_interpolator(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00a5_7080,
            ni_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a5_70b0, ni_interpolator_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a57070, &args![]).u32(), NI_INTERPOLATOR_RTTI);
    }

    #[test]
    fn thunk_00a88a40_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_8a40;
        const TARGET: u32 = 0x00a5_d610;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    #[test]
    fn constructor_and_destructor_set_the_vtable_and_last_time() {
        let mut e = Engine::new();
        let log = Log::default();
        for address in [NI_OBJECT_CONSTRUCT, NI_OBJECT_DESTRUCT] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        e.map(INITIAL_LAST_TIME, 4);
        // A signalling NaN shows the x87 copy quieting it.
        e.mem.set_u32(INITIAL_LAST_TIME, 0x7fa0_0001);
        let this = e.mem.alloc(0x0c);
        assert_eq!(e.call(0x00a5_6ff0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x7fe0_0001);
        e.mem.set_u32(INITIAL_LAST_TIME, (-f32::MAX).to_bits());
        e.call(0x00a5_6ff0, &args![this]);
        assert_eq!(e.mem.f32(this + 8), -f32::MAX);
        e.mem.set_u32(this, 0);
        e.call(0x00a5_7010, &args![this]);
        assert_eq!(e.mem.u32(this), NI_INTERPOLATOR_VTABLE);
        assert_eq!(
            *log.borrow(),
            [
                (NI_OBJECT_CONSTRUCT, vec![this]),
                (NI_OBJECT_CONSTRUCT, vec![this]),
                (NI_OBJECT_DESTRUCT, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

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

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00a57020_answers_false() {
        let mut e = Engine::new();
        let result = e.call(0x00a5_7020, &args![0x1000u32, 1u32, 2u32, 3u32]);
        assert_eq!(result.u8(), 0);
    }

    #[test]
    fn get_active_time_range_is_empty() {
        let mut e = Engine::new();
        let range = e.mem.alloc(8);
        e.mem.set_f32(range, 3.0);
        e.mem.set_f32(range + 4, -0.0);
        e.call(0x00a5_7030, &args![0u32, range, range + 4]);
        assert_eq!(e.mem.u32(range), 0);
        assert_eq!(e.mem.u32(range + 4), 0);
    }

    #[test]
    fn get_sequence_interpolator_sets_up_a_clone() {
        let mut e = Engine::new();
        let log = Log::default();
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + SLOT_SET_UP_DEPENDENCIES, 0x00ee_c000);
        let clone = e.mem.alloc(0x0c);
        e.mem.set_u32(clone, vtable);
        logged(&mut e, &log, NI_OBJECT_CLONE, clone);
        logged(&mut e, &log, 0x00ee_c000, 0);
        let this = e.mem.alloc(0x0c);
        let result = e.call(0x00a5_7050, &args![this, 1.0f32, 2.0f32]).u32();
        assert_eq!(result, clone);
        assert_eq!(calls(&log, NI_OBJECT_CLONE), [vec![this]]);
        assert_eq!(calls(&log, 0x00ee_c000), [vec![clone]]);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_OBJECT_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        let this = e.mem.alloc(0x0c);
        assert_eq!(e.call(0x00a5_7080, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_INTERPOLATOR_VTABLE);
        assert!(calls(&log, NI_SIZED_DELETE).is_empty());
        e.call(0x00a5_7080, &args![this, 1u32]);
        assert_eq!(calls(&log, NI_SIZED_DELETE), [vec![this, 0x0c]]);
        assert_eq!(calls(&log, NI_OBJECT_DESTRUCT), [vec![this], vec![this]]);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(NI_INTERPOLATOR_RTTI, 4);
        e.mem.set_u32(NI_INTERPOLATOR_RTTI, 0x0155_0000);
        logged(&mut e, &log, NI_OBJECT_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x0166_0000);
        logged(&mut e, &log, VIEWER_STRINGS_GROW, 0);
        e.register(VIEWER_STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0166_0000);
            Ret::default()
        });
        let this = e.mem.alloc(0x0c);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 4);
        e.call(0x00a5_70b0, &args![this, strings]);
        assert_eq!(
            calls(&log, NI_OBJECT_GET_VIEWER_STRINGS),
            [vec![this, strings]]
        );
        assert_eq!(calls(&log, NI_GET_VIEWER_STRING), [vec![0x0155_0000]]);
        assert_eq!(calls(&log, VIEWER_STRINGS_GROW), [vec![strings, 5]]);
    }
}
