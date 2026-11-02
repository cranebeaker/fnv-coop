//! `NiParticle/nipsysinitialrotanglectlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2a260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysInitialRotAngleCtlr::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00c32f60`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `012026f0`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_p_sys_initial_rot_angle_ctlr_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00c3_2f60;
    const CLASS_NAME_POINTER: u32 = 0x0120_26f0;
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

// Translated from 00c2a1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysInitialRotAngleCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26f0.
pub fn ni_p_sys_initial_rot_angle_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26f0
}

use crate::units::niparticle::nipsysmodifierctlr::interp_target_modifier_is;
use crate::units::niparticle::nipsysmodifierfloatctlr::create_clone;

/// The vtable of `NiPSysInitialRotAngleCtlr` (RTTI
/// `.?AVNiPSysInitialRotAngleCtlr@@`).
const VTABLE: u32 = 0x010b_efbc;
/// The `NiRTTI` record of `NiPSysRotationModifier` (`01202528`, the class
/// name its static initializer gives it).
const ROTATION_MODIFIER_RTTI: u32 = 0x0120_2528;

// Translated from 00c2a1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysInitialRotAngleCtlr::CreateClone` (Xbox PDB): a 0x40-byte object
/// through the `NiPSysModifierFloatCtlr` constructor and this class's vtable
/// (0 when the allocation fails), then the member copy into it (which still
/// runs for a null clone). Returns the clone.
pub fn ni_p_sys_initial_rot_angle_ctlr_create_clone(
    e: &mut Engine,
    this: Ptr,
    process: Ptr,
) -> Ptr {
    create_clone(e, this, process, VTABLE)
}

// Translated from 00c2a210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysInitialRotAngleCtlr::InterpTargetIsCorrectType` (Xbox PDB):
/// `NiPSysModifierCtlr::InterpTargetIsCorrectType` (`00c31530`) on
/// `target`, then `target`'s modifier with this controller's modifier name
/// (`00c1b0f0`) must exist and be of kind `NiPSysRotationModifier`.
pub fn ni_p_sys_initial_rot_angle_ctlr_interp_target_is_correct_type(
    e: &mut Engine,
    this: Ptr,
    target: Ptr,
) -> bool {
    interp_target_modifier_is(e, this.addr(), target.addr(), ROTATION_MODIFIER_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_a1c0,
            ni_p_sys_initial_rot_angle_ctlr_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00c2_a210,
            ni_p_sys_initial_rot_angle_ctlr_interp_target_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(0x00c2a1a0, ni_p_sys_initial_rot_angle_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00c2a260,
            ni_p_sys_initial_rot_angle_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_p_sys_initial_rot_angle_ctlr_get_viewer_strings_appends_the_class_name_growing_a_full_array(
    ) {
        const PARENT: u32 = 0x00c3_2f60;
        const NAME_POINTER: u32 = 0x0120_26f0;
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
            ni_p_sys_initial_rot_angle_ctlr_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
    fn ni_p_sys_initial_rot_angle_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_a1a0, &args![]).u32(), 0x0120_26f0);
    }
}

#[cfg(test)]
mod clone_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifierfloatctlr::subclass_checks;

    #[test]
    fn create_clone_uses_this_class_vtable() {
        subclass_checks::create_clone(0x00c2_a1c0, VTABLE);
    }

    #[test]
    fn interp_target_needs_a_rotation_modifier() {
        subclass_checks::interp_target(0x00c2_a210, ROTATION_MODIFIER_RTTI);
    }
}
