//! `NiParticle/nipsysemitterspeedctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterSpeedCtlr::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00c32f60`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `01202728`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_p_sys_emitter_speed_ctlr_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00c3_2f60;
    const CLASS_NAME_POINTER: u32 = 0x0120_2728;
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

// Translated from 00c2b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterSpeedCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2728.
pub fn ni_p_sys_emitter_speed_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2728
}

/// `00aa13e0` (`cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00c32d80`: the `NiPSysModifierFloatCtlr` constructor (`thiscall`; it
/// stores that class's vtable `010c0e44`).
const FLOAT_CTLR_CONSTRUCTOR: u32 = 0x00c3_2d80;
/// `00c32de0` (a 5-byte jump to `00c312f0`): the base member copy,
/// `thiscall`, `(clone, process)`.
const BASE_COPY_MEMBERS: u32 = 0x00c3_2de0;
/// The vtable of `NiPSysEmitterSpeedCtlr` (RTTI
/// `.?AVNiPSysEmitterSpeedCtlr@@`).
const VTABLE: u32 = 0x010b_f654;

// Translated from 00c2b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterSpeedCtlr::CreateClone` (Xbox PDB): a 0x40-byte object
/// from `00aa13e0` through the `NiPSysModifierFloatCtlr` constructor and this
/// class's vtable (0 when the allocation fails), then the base member copy
/// into it (which still runs for a null clone). Returns the clone.
pub fn ni_p_sys_emitter_speed_ctlr_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_NEW, &args![0x40u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        e.call(FLOAT_CTLR_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, VTABLE);
        Ptr::new(block)
    };
    e.call(BASE_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

use crate::units::niparticle::nipsysmodifierfloatctlr::{get_target_float, set_target_float};

/// `NiPSysEmitter::m_fSpeed` (Xbox PDB), +0x18 in the controller's
/// modifier.
const SPEED: u32 = 0x18;

// Translated from 00c29fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterSpeedCtlr::SetTargetFloatValue` (Xbox PDB): `value` into
/// the modifier's `float` at +0x18 (the emitter speed).
pub fn ni_p_sys_emitter_speed_ctlr_set_target_float_value(e: &mut Engine, this: Ptr, value: f32) {
    set_target_float(e, this, SPEED, value);
}

// Translated from 00c2b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterSpeedCtlr::GetTargetFloatValue` (Xbox PDB): the modifier's
/// `float` at +0x18 (the emitter speed) into `value`.
pub fn ni_p_sys_emitter_speed_ctlr_get_target_float_value(e: &mut Engine, this: Ptr, value: Ptr) {
    get_target_float(e, this, SPEED, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_9fa0,
            ni_p_sys_emitter_speed_ctlr_set_target_float_value(Ptr, f32)
        ),
        entry!(
            0x00c2_b370,
            ni_p_sys_emitter_speed_ctlr_get_target_float_value(Ptr, Ptr)
        ),
        entry!(0x00c2b360, ni_p_sys_emitter_speed_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00c2b3d0,
            ni_p_sys_emitter_speed_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(
            0x00c2_b380,
            ni_p_sys_emitter_speed_ctlr_create_clone(Ptr, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_p_sys_emitter_speed_ctlr_get_viewer_strings_appends_the_class_name_growing_a_full_array()
    {
        const PARENT: u32 = 0x00c3_2f60;
        const NAME_POINTER: u32 = 0x0120_2728;
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
            ni_p_sys_emitter_speed_ctlr_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
    fn ni_p_sys_emitter_speed_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_b360, &args![]).u32(), 0x0120_2728);
    }

    #[test]
    fn create_clone_constructs_sets_the_vtable_and_copies() {
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FLOAT_CTLR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_b380, &args![0x5000u32, 0x6000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x40]]);
        assert_eq!(calls_to(&e, FLOAT_CTLR_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_b380, &args![0x5000u32, 0x6000u32]).u32(), 0);
        assert!(calls_to(&e, FLOAT_CTLR_CONSTRUCTOR).is_empty());
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![0x5000, 0, 0x6000]]
        );
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifierfloatctlr::subclass_checks;

    #[test]
    fn set_target_float_value_writes_the_speed() {
        subclass_checks::set_value(0x00c2_9fa0, SPEED);
    }

    #[test]
    fn get_target_float_value_reads_the_speed() {
        subclass_checks::get_value(0x00c2_b370, SPEED);
    }
}
