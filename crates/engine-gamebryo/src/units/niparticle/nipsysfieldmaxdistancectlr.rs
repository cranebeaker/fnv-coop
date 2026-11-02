//! `NiParticle/nipsysfieldmaxdistancectlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2b0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldMaxDistanceCtlr::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00c32f60`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `01202710`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_p_sys_field_max_distance_ctlr_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00c3_2f60;
    const CLASS_NAME_POINTER: u32 = 0x0120_2710;
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

// Translated from 00c2b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldMaxDistanceCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2710.
pub fn ni_p_sys_field_max_distance_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2710
}

use crate::units::niparticle::nipsysmodifierfloatctlr::create_clone;

/// `NiPSysFieldModifier::m_fMaxDistance` (+0x28) and `m_fMaxDistanceSqr`
/// (+0x2c) (Xbox PDB), in the controller's modifier (`m_pkModifier`,
/// +0x3c).
const MODIFIER: u32 = 0x3c;
const MAX_DISTANCE: u32 = 0x28;
const MAX_DISTANCE_SQUARED: u32 = 0x2c;
/// The vtable of `NiPSysFieldMaxDistanceCtlr` (RTTI
/// `.?AVNiPSysFieldMaxDistanceCtlr@@`).
const VTABLE: u32 = 0x010b_f354;

// Translated from 00c2b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldMaxDistanceCtlr::SetTargetFloatValue` (Xbox PDB): `value`
/// into the modifier's maximum distance (+0x28, `FST`) and its square
/// (`FMUL ST0`) into +0x2c.
pub fn ni_p_sys_field_max_distance_ctlr_set_target_float_value(
    e: &mut Engine,
    this: Ptr,
    value: f32,
) {
    let modifier = e.mem.u32(this.addr() + MODIFIER);
    let distance = x87_load32(value.to_bits());
    e.mem
        .set_u32(modifier.wrapping_add(MAX_DISTANCE), x87_store32(distance));
    e.mem.set_u32(
        modifier.wrapping_add(MAX_DISTANCE_SQUARED),
        x87_store32(x87_mul(distance, distance)),
    );
}

// Translated from 00c2b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldMaxDistanceCtlr::CreateClone` (Xbox PDB): a 0x40-byte object
/// through the `NiPSysModifierFloatCtlr` constructor and this class's vtable
/// (0 when the allocation fails), then the member copy into it (which still
/// runs for a null clone). Returns the clone.
pub fn ni_p_sys_field_max_distance_ctlr_create_clone(
    e: &mut Engine,
    this: Ptr,
    process: Ptr,
) -> Ptr {
    create_clone(e, this, process, VTABLE)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_b050,
            ni_p_sys_field_max_distance_ctlr_set_target_float_value(Ptr, f32)
        ),
        entry!(
            0x00c2_b070,
            ni_p_sys_field_max_distance_ctlr_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00c2b040, ni_p_sys_field_max_distance_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00c2b0c0,
            ni_p_sys_field_max_distance_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_p_sys_field_max_distance_ctlr_get_viewer_strings_appends_the_class_name_growing_a_full_array(
    ) {
        const PARENT: u32 = 0x00c3_2f60;
        const NAME_POINTER: u32 = 0x0120_2710;
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
            ni_p_sys_field_max_distance_ctlr_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
    fn ni_p_sys_field_max_distance_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_b040, &args![]).u32(), 0x0120_2710);
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifierfloatctlr::subclass_checks;

    #[test]
    fn set_target_float_value_writes_the_distance_and_its_square() {
        let mut e = Engine::new();
        let modifier = e.mem.alloc(0x40);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + MODIFIER, modifier);
        e.call(0x00c2_b050, &args![this, 3.0f32]);
        assert_eq!(e.mem.u32(modifier + MAX_DISTANCE), 3.0f32.to_bits());
        assert_eq!(e.mem.u32(modifier + MAX_DISTANCE_SQUARED), 9.0f32.to_bits());
        // The square rounds to a float; a signalling NaN is quieted.
        e.call(0x00c2_b050, &args![this, 1.000_000_1f32]);
        let x = 1.000_000_1f32 as f64;
        assert_eq!(
            e.mem.u32(modifier + MAX_DISTANCE_SQUARED),
            ((x * x) as f32).to_bits()
        );
        e.call(0x00c2_b050, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(modifier + MAX_DISTANCE), 0x7fc0_0001);
        assert_eq!(e.mem.u32(modifier + MAX_DISTANCE_SQUARED), 0x7fc0_0001);
    }

    #[test]
    fn create_clone_uses_this_class_vtable() {
        subclass_checks::create_clone(0x00c2_b070, VTABLE);
    }
}
