//! `NiParticle/nipsysemitterdeclinationvarctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2b920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterDeclinationVarCtlr::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00c32f60`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `01202750`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_p_sys_emitter_declination_var_ctlr_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00c3_2f60;
    const CLASS_NAME_POINTER: u32 = 0x0120_2750;
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

// Translated from 00c2b8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterDeclinationVarCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2750.
pub fn ni_p_sys_emitter_declination_var_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2750
}

/// The controlled modifier (`NiPSysModifierCtlr::m_pkModifier`, Xbox PDB,
/// +0x3C) and its `m_fDeclinationVar` (Xbox PDB, `NiPSysEmitter` +0x24).
const MODIFIER_OFFSET: u32 = 0x3c;
const VALUE_OFFSET: u32 = 0x24;
/// `00aa13e0` (`cdecl`, one word): the object allocation; `00c32d80`: the
/// `NiPSysModifierFloatCtlr` constructor (`thiscall`); `00c32de0`: the base
/// member copy (`thiscall`, `(clone, process)`).
const NI_NEW: u32 = 0x00aa_13e0;
const FLOAT_CTLR_CONSTRUCTOR: u32 = 0x00c3_2d80;
const BASE_COPY_MEMBERS: u32 = 0x00c3_2de0;
/// The vtable the clone gets (`010bfb54`).
const CLONE_VTABLE: u32 = 0x010b_fb54;

// Translated from 00c2b8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterDeclinationVarCtlr::GetTargetFloatValue` (Xbox PDB): the modifier's
/// `m_fDeclinationVar` into `*value`.
pub fn ni_p_sys_emitter_declination_var_ctlr_get_target_float_value(
    e: &mut Engine,
    this: Ptr,
    value: Ptr,
) {
    let modifier = e.mem.u32(this.addr() + MODIFIER_OFFSET);
    let bits = e.mem.u32(modifier + VALUE_OFFSET);
    e.mem.set_u32(value.addr(), x87_quiet_bits(bits));
}

// Translated from 00c2a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterDeclinationVarCtlr::SetTargetFloatValue` (Xbox PDB): `value` into the
/// modifier's `m_fDeclinationVar`.
pub fn ni_p_sys_emitter_declination_var_ctlr_set_target_float_value(
    e: &mut Engine,
    this: Ptr,
    value: f32,
) {
    let modifier = e.mem.u32(this.addr() + MODIFIER_OFFSET);
    e.mem
        .set_u32(modifier + VALUE_OFFSET, x87_quiet_bits(value.to_bits()));
}

// Translated from 00c2b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterDeclinationVarCtlr::CreateClone` (Xbox PDB): a 0x40-byte object from `00aa13e0`
/// through the `NiPSysModifierFloatCtlr` constructor and this class's
/// vtable (null when the allocation fails), then the base member copy into
/// it (which still runs for a null clone). Returns the clone.
pub fn ni_p_sys_emitter_declination_var_ctlr_create_clone(
    e: &mut Engine,
    this: Ptr,
    process: Ptr,
) -> Ptr {
    let block = e.call(NI_NEW, &args![0x40u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        e.call(FLOAT_CTLR_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, CLONE_VTABLE);
        Ptr::new(block)
    };
    e.call(BASE_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2b8b0, ni_p_sys_emitter_declination_var_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00c2b920,
            ni_p_sys_emitter_declination_var_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(
            0x00c2_b8c0,
            ni_p_sys_emitter_declination_var_ctlr_get_target_float_value(Ptr, Ptr)
        ),
        entry!(
            0x00c2_a0b0,
            ni_p_sys_emitter_declination_var_ctlr_set_target_float_value(Ptr, f32)
        ),
        entry!(0x00c2_b8d0, ni_p_sys_emitter_declination_var_ctlr_create_clone(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_p_sys_emitter_declination_var_ctlr_get_viewer_strings_appends_the_class_name_growing_a_full_array(
    ) {
        const PARENT: u32 = 0x00c3_2f60;
        const NAME_POINTER: u32 = 0x0120_2750;
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
            ni_p_sys_emitter_declination_var_ctlr_get_viewer_strings(
                &mut e,
                this,
                Ptr::new(strings),
            );
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
    fn ni_p_sys_emitter_declination_var_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_b8b0, &args![]).u32(), 0x0120_2750);
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{answer, calls_to};

    /// A controller whose modifier is a zeroed 0x80-byte block.
    fn controller(e: &mut Engine) -> (Ptr, u32) {
        let this = Ptr::new(e.mem.alloc(0x40));
        let modifier = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0x3c, modifier);
        (this, modifier)
    }

    #[test]
    fn target_float_value_is_read_from_the_modifier() {
        let mut e = Engine::new();
        let (this, modifier) = controller(&mut e);
        e.mem.set_u32(modifier + 0x24, 0x7f80_0001);
        let value = e.mem.alloc(4);
        e.call(0x00c2_b8c0, &args![this, value]);
        assert_eq!(e.mem.u32(value), 0x7fc0_0001);
        e.mem.set_u32(modifier + 0x24, 2.5f32.to_bits());
        e.call(0x00c2_b8c0, &args![this, value]);
        assert_eq!(e.mem.u32(value), 2.5f32.to_bits());
    }

    #[test]
    fn target_float_value_is_stored_in_the_modifier() {
        let mut e = Engine::new();
        let (this, modifier) = controller(&mut e);
        e.call(0x00c2_a0b0, &args![this, 0xff80_0001u32]);
        assert_eq!(e.mem.u32(modifier + 0x24), 0xffc0_0001);
        e.call(0x00c2_a0b0, &args![this, -3.5f32]);
        assert_eq!(e.mem.u32(modifier + 0x24), (-3.5f32).to_bits());
    }

    #[test]
    fn create_clone_constructs_and_copies() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| answer(e.mem.alloc(a[0])));
        e.register(FLOAT_CTLR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let (this, _) = controller(&mut e);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_b8d0, &args![this, 0x900u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x40]]);
        assert_eq!(calls_to(&e, FLOAT_CTLR_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(e.mem.u32(clone), CLONE_VTABLE);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x900]]
        );
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_b8d0, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS)[1],
            vec![this.addr(), 0, 0x900]
        );
        assert_eq!(calls_to(&e, FLOAT_CTLR_CONSTRUCTOR).len(), 1);
    }
}
