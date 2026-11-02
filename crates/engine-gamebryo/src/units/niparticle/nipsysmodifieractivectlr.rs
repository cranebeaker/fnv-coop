//! `NiParticle/nipsysmodifieractivectlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c32bf0`: the jump target of `00c29d30`.
const JUMP_TARGET_00C29D30: u32 = 0x00c3_2bf0;

// Translated from 00c29d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierActiveCtlr::LinkObject` (Xbox PDB): a 5-byte jump to `00c32bf0` (engine map: `NiPSysModifierBoolCtlr::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysModifierActiveCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_active_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C29D30, &args![this, stream]);
}

// Translated from 00c29d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierActiveCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26d0.
pub fn ni_p_sys_modifier_active_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26d0
}

/// `NiPSysModifierCtlr::m_pkModifier` (Xbox PDB) +0x3C: `NiPSysModifier*`.
const MODIFIER: u32 = 0x3c;
/// `NiPSysModifier::m_bActive` (Xbox PDB) +0x14.
const MODIFIER_ACTIVE: u32 = 0x14;
/// `NiPSysModifier::SetActive` (Xbox PDB): vtable slot +0x98.
const SET_ACTIVE_SLOT: u32 = 0x98;
/// `00aa13e0` (`cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00c32b60`: the `NiPSysModifierBoolCtlr` constructor (`thiscall`).
const BOOL_CTLR_CONSTRUCTOR: u32 = 0x00c3_2b60;
/// `00c32de0` (a 5-byte jump to `00c312f0`): the base member copy,
/// `thiscall`, `(clone, process)`.
const BASE_COPY_MEMBERS: u32 = 0x00c3_2de0;
/// The vtable of `NiPSysModifierActiveCtlr` (RTTI
/// `.?AVNiPSysModifierActiveCtlr@@`).
const VTABLE: u32 = 0x010b_ebbc;

// Translated from 00c29d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierActiveCtlr::SetTargetBoolValue` (Xbox PDB): when the low
/// byte of `value` differs from the modifier's `m_bActive`, a tail jump to
/// the modifier's `SetActive` (slot +0x98) with the whole argument word;
/// nothing otherwise. The modifier pointer is not checked for null.
pub fn ni_p_sys_modifier_active_ctlr_set_target_bool_value(e: &mut Engine, this: Ptr, value: u32) {
    let modifier = e.mem.u32(this.addr() + MODIFIER);
    if e.mem.u8(modifier + MODIFIER_ACTIVE) == value as u8 {
        return;
    }
    e.vcall(modifier, SET_ACTIVE_SLOT, &args![value]);
}

// Translated from 00c29d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierActiveCtlr::CreateClone` (Xbox PDB): a 0x40-byte object
/// from `00aa13e0` through the `NiPSysModifierBoolCtlr` constructor and this
/// class's vtable (0 when the allocation fails), then the base member copy
/// into it (which still runs for a null clone). Returns the clone.
pub fn ni_p_sys_modifier_active_ctlr_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_NEW, &args![0x40u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        e.call(BOOL_CTLR_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, VTABLE);
        Ptr::new(block)
    };
    e.call(BASE_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

use crate::units::niparticle::nipsysmodifier::add_class_name;
use crate::units::niparticle::nipsysmodifierctlr::{
    INTERP_TARGET_IS_CORRECT_TYPE, SYSTEM_FIND_MODIFIER,
};

/// The `NiRTTI` record of `NiPSysModifierActiveCtlr` (`012026d0`).
const RTTI: u32 = 0x0120_26d0;
/// `NiPSysModifierBoolCtlr::GetViewerStrings` (`00c32d30`, `thiscall`, one
/// word).
const BOOL_CTLR_GET_VIEWER_STRINGS: u32 = 0x00c3_2d30;
/// `NiPSysModifierCtlr::m_kModifierName` (Xbox PDB), +0x38.
const MODIFIER_NAME: u32 = 0x38;

// Translated from 00c29d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierActiveCtlr::GetTargetBoolValue` (Xbox PDB): the
/// modifier's active byte (`NiPSysModifier::m_bActive`, +0x14) into the byte
/// at `value`.
pub fn ni_p_sys_modifier_active_ctlr_get_target_bool_value(e: &mut Engine, this: Ptr, value: Ptr) {
    let modifier = e.mem.u32(this.addr() + MODIFIER);
    let active = e.mem.u8(modifier.wrapping_add(MODIFIER_ACTIVE));
    e.mem.set_u8(value.addr(), active);
}

// Translated from 00c29dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierActiveCtlr::InterpTargetIsCorrectType` (Xbox PDB):
/// `NiPSysModifierCtlr::InterpTargetIsCorrectType` (`00c31530`) on
/// `target`; then whether `target` has a modifier with this controller's
/// modifier name (`00c1b0f0`).
pub fn ni_p_sys_modifier_active_ctlr_interp_target_is_correct_type(
    e: &mut Engine,
    this: Ptr,
    target: Ptr,
) -> bool {
    if !e
        .call(INTERP_TARGET_IS_CORRECT_TYPE, &args![this, target])
        .bool()
    {
        return false;
    }
    let name = this.addr() + MODIFIER_NAME;
    e.call(SYSTEM_FIND_MODIFIER, &args![target, name]).u32() != 0
}

// Translated from 00c29e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierActiveCtlr::GetViewerStrings` (Xbox PDB):
/// `NiPSysModifierBoolCtlr::GetViewerStrings` (`00c32d30`), then the class
/// name (`NiRTTI` `012026d0`) appended.
pub fn ni_p_sys_modifier_active_ctlr_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(BOOL_CTLR_GET_VIEWER_STRINGS, &args![this, strings]);
    add_class_name(e, strings.addr(), RTTI);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_9d50,
            ni_p_sys_modifier_active_ctlr_get_target_bool_value(Ptr, Ptr)
        ),
        entry!(
            0x00c2_9dd0,
            ni_p_sys_modifier_active_ctlr_interp_target_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00c2_9e00,
            ni_p_sys_modifier_active_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c29d40, ni_p_sys_modifier_active_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_9d30,
            ni_p_sys_modifier_active_ctlr_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00c2_9d60,
            ni_p_sys_modifier_active_ctlr_set_target_bool_value(Ptr, u32)
        ),
        entry!(
            0x00c2_9d80,
            ni_p_sys_modifier_active_ctlr_create_clone(Ptr, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c29d30_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_9d30;
        const TARGET: u32 = 0x00c3_2bf0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn ni_p_sys_modifier_active_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_9d40, &args![]).u32(), 0x0120_26d0);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn set_target_bool_value_calls_set_active_only_on_a_change() {
        const SET_ACTIVE: u32 = 0x0040_1230;
        let mut e = Engine::new();
        e.register(SET_ACTIVE, |_, _| Ret::default());
        e.put_vtable(0x0090_0000, &[0; 0x9c / 4]);
        e.mem.set_u32(0x0090_0000 + SET_ACTIVE_SLOT, SET_ACTIVE);
        let modifier = e.mem.alloc(0x18);
        e.mem.set_u32(modifier, 0x0090_0000);
        e.mem.set_u8(modifier + MODIFIER_ACTIVE, 1);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + MODIFIER, modifier);
        e.call_log = Some(vec![]);
        // Same low byte (upper bytes ignored): no call.
        e.call(0x00c2_9d60, &args![this, 0x7700_0001u32]);
        assert!(calls_to(&e, SET_ACTIVE).is_empty());
        // A change: the whole word goes to SetActive.
        e.call(0x00c2_9d60, &args![this, 0x7700_0000u32]);
        assert_eq!(calls_to(&e, SET_ACTIVE), vec![vec![modifier, 0x7700_0000]]);
    }

    #[test]
    fn create_clone_constructs_sets_the_vtable_and_copies() {
        let mut e = Engine::new();
        e.register_double(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(BOOL_CTLR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_9d80, &args![0x5000u32, 0x6000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x40]]);
        assert_eq!(calls_to(&e, BOOL_CTLR_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );

        // A failed allocation: no constructor, the copy gets a null clone.
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_9d80, &args![0x5000u32, 0x6000u32]).u32(), 0);
        assert!(calls_to(&e, BOOL_CTLR_CONSTRUCTOR).is_empty());
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![0x5000, 0, 0x6000]]
        );
    }
}

#[cfg(test)]
mod active_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    #[test]
    fn get_target_bool_value_copies_the_active_byte() {
        let mut e = Engine::new();
        let modifier = e.mem.alloc(0x18);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + MODIFIER, modifier);
        e.mem.set_u8(modifier + MODIFIER_ACTIVE, 7);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0xaabb_cc00);
        e.call(0x00c2_9d50, &args![this, value]);
        assert_eq!(e.mem.u32(value), 0xaabb_cc07);
    }

    #[test]
    fn interp_target_needs_a_named_modifier() {
        let mut e = Engine::new();
        e.register(INTERP_TARGET_IS_CORRECT_TYPE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(SYSTEM_FIND_MODIFIER, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c2_9dd0, &args![0x5000u32, 0x7000u32]).bool());
        assert_eq!(
            calls_to(&e, INTERP_TARGET_IS_CORRECT_TYPE),
            vec![vec![0x5000, 0x7000]]
        );
        assert_eq!(
            calls_to(&e, SYSTEM_FIND_MODIFIER),
            vec![vec![0x7000, 0x5038]]
        );
        e.register(SYSTEM_FIND_MODIFIER, |_, _| Ret::default());
        assert!(!e.call(0x00c2_9dd0, &args![0x5000u32, 0x7000u32]).bool());
        e.register(INTERP_TARGET_IS_CORRECT_TYPE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c2_9dd0, &args![0x5000u32, 0x7000u32]).bool());
        assert!(calls_to(&e, SYSTEM_FIND_MODIFIER).is_empty());
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0004);
        e.register(BOOL_CTLR_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_9e00, &args![0x5000u32, strings]);
        assert_eq!(
            calls_to(&e, BOOL_CTLR_GET_VIEWER_STRINGS),
            vec![vec![0x5000, strings]]
        );
        assert_eq!(*stored.borrow(), vec![0x0abc_0004]);
    }
}
