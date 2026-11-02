//! `NiParticle/nipsysairfieldairfrictionctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a4fbd0`: the jump target of `00c29e80`.
const JUMP_TARGET_00C29E80: u32 = 0x00a4_fbd0;

/// `00a44cd0`: the jump target of `00c29f80`.
const JUMP_TARGET_00C29F80: u32 = 0x00a4_4cd0;

/// `00c32df0`: the jump target of `00c2a190`.
const JUMP_TARGET_00C2A190: u32 = 0x00c3_2df0;

/// `00c32e10`: the jump target of `00c2b450`.
const JUMP_TARGET_00C2B450: u32 = 0x00c3_2e10;

/// `00a41220`: the jump target of `00c2b8a0`.
const JUMP_TARGET_00C2B8A0: u32 = 0x00a4_1220;

// Translated from 00c29e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::LoadBinary` (Xbox PDB): a 5-byte jump to `00a4fbd0` (engine map: `NiPSysModifierBoolCtlr::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiPSysAirFieldAirFrictionCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C29E80, &args![this, stream]);
}

// Translated from 00c29f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a44cd0` (engine map: `NiPSysModifierBoolCtlr::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysAirFieldAirFrictionCtlr@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C29F80, &args![this, stream]).bool()
}

// Translated from 00c2a190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::IsEqual` (Xbox PDB): a 5-byte jump to `00c32df0` (engine map: `NiPSysModifierBoolCtlr::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiPSysAirFieldAirFrictionCtlr@@UAA_NPAVNiObject@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_is_equal(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C2A190, &args![this, other]).bool()
}

// Translated from 00c2b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::LinkObject` (Xbox PDB): a 5-byte jump to `00c32e10` (engine map: `NiPSysModifierFloatCtlr::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysAirFieldAirFrictionCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C2B450, &args![this, stream]);
}

// Translated from 00c2b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::SaveBinary` (Xbox PDB): a 5-byte jump to `00a41220` (engine map: `NiPSysModifierBoolCtlr::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiPSysAirFieldAirFrictionCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_air_friction_ctlr_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C2B8A0, &args![this, stream]);
}

// Translated from 00c307c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27b8.
pub fn ni_p_sys_air_field_air_friction_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27b8
}

/// The controlled modifier (`NiPSysModifierCtlr::m_pkModifier`, Xbox PDB,
/// +0x3C) and its `m_fAirFriction` (Xbox PDB, `NiPSysAirFieldModifier` +0x58).
const MODIFIER_OFFSET: u32 = 0x3c;
const VALUE_OFFSET: u32 = 0x58;
/// `00aa13e0` (`cdecl`, one word): the object allocation; `00c32d80`: the
/// `NiPSysModifierFloatCtlr` constructor (`thiscall`); `00c32de0`: the base
/// member copy (`thiscall`, `(clone, process)`).
const NI_NEW: u32 = 0x00aa_13e0;
const FLOAT_CTLR_CONSTRUCTOR: u32 = 0x00c3_2d80;
const BASE_COPY_MEMBERS: u32 = 0x00c3_2de0;
/// The vtable the clone gets (`010c06e4`).
const CLONE_VTABLE: u32 = 0x010c_06e4;

// Translated from 00c307d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::GetTargetFloatValue` (Xbox PDB): the modifier's
/// `m_fAirFriction` into `*value`.
pub fn ni_p_sys_air_field_air_friction_ctlr_get_target_float_value(
    e: &mut Engine,
    this: Ptr,
    value: Ptr,
) {
    let modifier = e.mem.u32(this.addr() + MODIFIER_OFFSET);
    let bits = e.mem.u32(modifier + VALUE_OFFSET);
    e.mem.set_u32(value.addr(), x87_quiet_bits(bits));
}

/// The `double` bounds the setter compares with (`01012060`, `01012070`;
/// read from the exe).
const LOWER_BOUND: u32 = 0x0101_2060;
const UPPER_BOUND: u32 = 0x0101_2070;

// Translated from 00c307e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::SetTargetFloatValue` (Xbox PDB): `value` into the
/// modifier's `m_fAirFriction`, made 0.0 when it is below the `double` at
/// `01012060` and then 1.0 when it is above the one at `01012070` (a NaN is
/// kept).
pub fn ni_p_sys_air_field_air_friction_ctlr_set_target_float_value(
    e: &mut Engine,
    this: Ptr,
    value: f32,
) {
    let modifier = e.mem.u32(this.addr() + MODIFIER_OFFSET);
    let mut slot = x87_quiet_bits(value.to_bits());
    if x87_load32(value.to_bits()) < e.global::<f64>(LOWER_BOUND) {
        slot = 0.0f32.to_bits();
    }
    if x87_load32(slot) > e.global::<f64>(UPPER_BOUND) {
        slot = 1.0f32.to_bits();
    }
    e.mem.set_u32(modifier + VALUE_OFFSET, slot);
}

// Translated from 00c30830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::CreateClone` (Xbox PDB): a 0x40-byte object from `00aa13e0`
/// through the `NiPSysModifierFloatCtlr` constructor and this class's
/// vtable (null when the allocation fails), then the base member copy into
/// it (which still runs for a null clone). Returns the clone.
pub fn ni_p_sys_air_field_air_friction_ctlr_create_clone(
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

use crate::units::niparticle::nipsysdata::add_viewer_string;

/// `NiPSysModifierCtlr::InterpTargetIsCorrectType` (`00c31530`) and
/// `NiPSysModifierFloatCtlr::GetViewerStrings` (`00c32f60`) (Xbox PDB
/// names; `thiscall`, one word); `00c1b0f0` (unnamed; `thiscall` on a
/// particle system, the address of a modifier name; returns a modifier or
/// null).
const MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE: u32 = 0x00c3_1530;
const FLOAT_CTLR_GET_VIEWER_STRINGS: u32 = 0x00c3_2f60;
const FIND_MODIFIER: u32 = 0x00c1_b0f0;
/// The modifier name (`NiPSysModifierCtlr::m_kModifierName`, Xbox PDB,
/// +0x38).
const MODIFIER_NAME_OFFSET: u32 = 0x38;
/// `NiPSysAirFieldModifier::ms_RTTI` (`012027a8`, what its `GetRTTI`
/// returns) and `NiPSysAirFieldAirFrictionCtlr::ms_RTTI` (`012027b8`, its
/// name is the first word).
const AIR_FIELD_MODIFIER_RTTI: u32 = 0x0120_27a8;
const RTTI: u32 = 0x0120_27b8;
/// `NiGetViewerString` (`00a7d330`, Xbox PDB name; `cdecl`, the text) and
/// `NiObject`'s `GetRTTI` slot (`+8`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const GET_RTTI_SLOT: u32 = 0x8;

// Translated from 00c306f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::InterpTargetIsCorrectType` (Xbox PDB):
/// `NiPSysModifierCtlr`'s test, then `target`'s modifier of this
/// controller's name (`00c1b0f0`) must exist and be an
/// `NiPSysAirFieldModifier` (its `NiRTTI` chain, each record's base at +4).
pub fn ni_p_sys_air_field_air_friction_ctlr_interp_target_is_correct_type(
    e: &mut Engine,
    this: Ptr,
    target: Ptr,
) -> bool {
    if !e
        .call(
            MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE,
            &args![this, target],
        )
        .bool()
    {
        return false;
    }
    let name = this.addr() + MODIFIER_NAME_OFFSET;
    let modifier = e.call(FIND_MODIFIER, &args![target, name]).u32();
    if modifier == 0 {
        return false;
    }
    let mut rtti = e.vcall(modifier, GET_RTTI_SLOT, &args![]).u32();
    while rtti != 0 {
        if rtti == AIR_FIELD_MODIFIER_RTTI {
            return true;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    false
}

// Translated from 00c30880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldAirFrictionCtlr::GetViewerStrings` (Xbox PDB):
/// `NiPSysModifierFloatCtlr`'s strings, then the class name.
pub fn ni_p_sys_air_field_air_friction_ctlr_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    e.call(FLOAT_CTLR_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings.addr(), string);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c307c0, ni_p_sys_air_field_air_friction_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_9e80,
            ni_p_sys_air_field_air_friction_ctlr_load_binary(Ptr, Ptr)
        ),
        entry!(0x00c2_9f80, ni_p_sys_air_field_air_friction_ctlr_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c2_a190, ni_p_sys_air_field_air_friction_ctlr_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_b450,
            ni_p_sys_air_field_air_friction_ctlr_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00c2_b8a0,
            ni_p_sys_air_field_air_friction_ctlr_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c3_07d0,
            ni_p_sys_air_field_air_friction_ctlr_get_target_float_value(Ptr, Ptr)
        ),
        entry!(
            0x00c3_07e0,
            ni_p_sys_air_field_air_friction_ctlr_set_target_float_value(Ptr, f32)
        ),
        entry!(0x00c3_0830, ni_p_sys_air_field_air_friction_ctlr_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00c3_06f0,
            ni_p_sys_air_field_air_friction_ctlr_interp_target_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00c3_0880,
            ni_p_sys_air_field_air_friction_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c29e80_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_9e80;
        const TARGET: u32 = 0x00a4_fbd0;
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
    fn thunk_00c29f80_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_9f80;
        const TARGET: u32 = 0x00a4_4cd0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00c2a190_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_a190;
        const TARGET: u32 = 0x00c3_2df0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00c2b450_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_b450;
        const TARGET: u32 = 0x00c3_2e10;
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
    fn thunk_00c2b8a0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_b8a0;
        const TARGET: u32 = 0x00a4_1220;
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
    fn ni_p_sys_air_field_air_friction_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_07c0, &args![]).u32(), 0x0120_27b8);
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{answer, calls_to};

    /// A controller whose modifier is a zeroed 0x80-byte block.
    fn controller(e: &mut Engine) -> (Ptr, u32) {
        let this: Ptr = Ptr::new(e.mem.alloc(0x40));
        let modifier = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0x3c, modifier);
        (this, modifier)
    }

    #[test]
    fn target_float_value_is_read_from_the_modifier() {
        let mut e = Engine::new();
        let (this, modifier) = controller(&mut e);
        e.mem.set_u32(modifier + 0x58, 0x7f80_0001);
        let value = e.mem.alloc(4);
        e.call(0x00c3_07d0, &args![this, value]);
        assert_eq!(e.mem.u32(value), 0x7fc0_0001);
        e.mem.set_u32(modifier + 0x58, 2.5f32.to_bits());
        e.call(0x00c3_07d0, &args![this, value]);
        assert_eq!(e.mem.u32(value), 2.5f32.to_bits());
    }

    #[test]
    fn target_float_value_is_clamped_into_the_modifier() {
        let mut e = Engine::new();
        e.map(LOWER_BOUND, 0x18);
        e.set_global::<f64>(LOWER_BOUND, 0.0);
        e.set_global::<f64>(UPPER_BOUND, 1.0);
        let (this, modifier) = controller(&mut e);
        let mut set = |value: u32| {
            e.call(0x00c3_07e0, &args![this, value]);
            e.mem.u32(modifier + 0x58)
        };
        assert_eq!(set(0.25f32.to_bits()), 0.25f32.to_bits());
        assert_eq!(set((-0.5f32).to_bits()), 0);
        assert_eq!(set(1.5f32.to_bits()), 1.0f32.to_bits());
        assert_eq!(set(1.0f32.to_bits()), 1.0f32.to_bits());
        assert_eq!(set((-0.0f32).to_bits()), (-0.0f32).to_bits());
        assert_eq!(set(0x7f80_0001), 0x7fc0_0001);
        assert_eq!(set(f32::NEG_INFINITY.to_bits()), 0);
    }

    #[test]
    fn create_clone_constructs_and_copies() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| answer(e.mem.alloc(a[0])));
        e.register(FLOAT_CTLR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let (this, _) = controller(&mut e);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c3_0830, &args![this, 0x900u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x40]]);
        assert_eq!(calls_to(&e, FLOAT_CTLR_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(e.mem.u32(clone), CLONE_VTABLE);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x900]]
        );
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c3_0830, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS)[1],
            vec![this.addr(), 0, 0x900]
        );
        assert_eq!(calls_to(&e, FLOAT_CTLR_CONSTRUCTOR).len(), 1);
    }
}

#[cfg(test)]
mod n064_friction_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to};

    #[test]
    fn interp_target_needs_an_air_field_modifier() {
        const GET_RTTI: u32 = 0x0000_a008;
        let mut e = Engine::new();
        e.register(MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE, |_, _| {
            answer(1)
        });
        e.map(0x0f70_0000, 0x20);
        e.register(GET_RTTI, |e, _| answer(e.mem.u32(0x0f70_0000)));
        let modifier = viewer_test::object(&mut e, &[(0x8, GET_RTTI)]);
        e.mem.set_u32(0x0f70_0004, modifier);
        e.register(FIND_MODIFIER, |e, _| answer(e.mem.u32(0x0f70_0004)));
        // Chain: 0f700010 -> the air field modifier's record.
        e.map(AIR_FIELD_MODIFIER_RTTI, 8);
        e.mem.set_u32(0x0f70_0014, AIR_FIELD_MODIFIER_RTTI);
        e.mem.set_u32(0x0f70_0000, 0x0f70_0010);
        let this: Ptr = Ptr::new(e.mem.alloc(0x40));
        let target: Ptr = Ptr::new(e.mem.alloc(0x10));
        let check = |e: &mut Engine| e.call(0x00c3_06f0, &args![this, target]).bool();
        e.call_log = Some(vec![]);
        assert!(check(&mut e));
        assert_eq!(
            calls_to(&e, MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE),
            vec![vec![this.addr(), target.addr()]]
        );
        assert_eq!(
            calls_to(&e, FIND_MODIFIER),
            vec![vec![target.addr(), this.addr() + 0x38]]
        );
        e.mem.set_u32(0x0f70_0014, 0);
        assert!(!check(&mut e));
        e.mem.set_u32(0x0f70_0004, 0);
        assert!(!check(&mut e));
        e.register(MODIFIER_CTLR_INTERP_TARGET_IS_CORRECT_TYPE, |_, _| {
            answer(0)
        });
        assert!(!check(&mut e));
        assert_eq!(calls_to(&e, FIND_MODIFIER).len(), 3);
    }

    #[test]
    fn viewer_strings_add_the_class_name() {
        let mut e = Engine::new();
        e.register(FLOAT_CTLR_GET_VIEWER_STRINGS, |_, _| Ret::default());
        viewer_test::counting(&mut e, &[VIEWER_STRING]);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_0080);
        let strings = viewer_test::strings(&mut e, 0);
        let this: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.call_log = Some(vec![]);
        e.call(0x00c3_0880, &args![this, strings]);
        assert_eq!(viewer_test::collected(&e, strings), vec![1]);
        assert_eq!(
            calls_to(&e, FLOAT_CTLR_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x0100_0080]]);
    }
}
