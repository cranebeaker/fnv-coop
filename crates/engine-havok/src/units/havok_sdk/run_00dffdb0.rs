//! `Havok SDK/run_00dffdb0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x20-byte stack buffer by calling `00cdf760` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfffa0(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, buffer| {
        e.call(0x00cd_f760, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dffdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_c0d8.
pub fn return_constant_00dffdb0(_e: &mut Engine) -> u32 {
    0x010c_c0d8
}

// Translated from 00dffe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6e34.
pub fn return_constant_00dffe10(_e: &mut Engine) -> u32 {
    0x010c_6e34
}

// Translated from 00dffe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_3838.
pub fn return_constant_00dffe50(_e: &mut Engine) -> u32 {
    0x010d_3838
}

// Translated from 00dffeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6d54.
pub fn return_constant_00dffeb0(_e: &mut Engine) -> u32 {
    0x010c_6d54
}

// Translated from 00dffef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_b798.
pub fn return_constant_00dffef0(_e: &mut Engine) -> u32 {
    0x010c_b798
}

// Translated from 00dfff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_a004.
pub fn return_constant_00dfff50(_e: &mut Engine) -> u32 {
    0x010d_a004
}

// Translated from 00dffff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_80b4.
pub fn return_constant_00dffff0(_e: &mut Engine) -> u32 {
    0x010e_80b4
}

// Translated from 00dffd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVelocityConstraintMotor` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_velocity_constraint_motor_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dffda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVelocityConstraintMotor` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010c_c0d8 at `+0`; the flag (second word) is not read.
pub fn hkp_velocity_constraint_motor_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010c_c0d8);
    }
}

// Translated from 00dffdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpThinBoxMotion` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_thin_box_motion_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dffde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpThinBoxMotion` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the base vtable address 0x010c_59f4 at `+0`; when the flag is not zero it stores the high half (`SAR 16`) of the float 1.0 (`0x3f80`) in the 16-bit word at `+0x11e`; then stores the vtable address 0x010c_6e34 at `+0`.
pub fn hkp_thin_box_motion_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    let base = object.addr();
    e.mem.set_u32(base, 0x010c_59f4);
    if flag != 0 {
        // FLD1; FSTP: the float 1.0 (exact), then its bits shifted right 16, arithmetic.
        let one = 1.0f32.to_bits() as i32;
        e.mem.set_u16(base + 0x11e, (one >> 16) as u16);
    }
    e.mem.set_u32(base, 0x010c_6e34);
}

// Translated from 00dffe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpSpringDamperConstraintMotor` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_spring_damper_constraint_motor_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dffe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpSpringDamperConstraintMotor` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010d_3838 at `+0`; the flag (second word) is not read.
pub fn hkp_spring_damper_constraint_motor_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010d_3838);
    }
}

// Translated from 00dffe60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpSphereMotion` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_sphere_motion_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dffe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpSphereMotion` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the base vtable address 0x010c_59f4 at `+0`; when the flag is not zero it stores the high half (`SAR 16`) of the float 1.0 (`0x3f80`) in the 16-bit word at `+0x11e`; then stores the vtable address 0x010c_6d54 at `+0`.
pub fn hkp_sphere_motion_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    let base = object.addr();
    e.mem.set_u32(base, 0x010c_59f4);
    if flag != 0 {
        // FLD1; FSTP: the float 1.0 (exact), then its bits shifted right 16, arithmetic.
        let one = 1.0f32.to_bits() as i32;
        e.mem.set_u16(base + 0x11e, (one >> 16) as u16);
    }
    e.mem.set_u32(base, 0x010c_6d54);
}

// Translated from 00dffec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpPositionConstraintMotor` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_position_constraint_motor_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dffee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpPositionConstraintMotor` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010c_b798 at `+0`; the flag (second word) is not read.
pub fn hkp_position_constraint_motor_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010c_b798);
    }
}

// Translated from 00dfff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpPointToPathConstraintData` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_point_to_path_constraint_data_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpPointToPathConstraintData` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010d_a004 at `+0`; when the flag is not zero it runs `00d21ce0` on `object + 0xc` with the word at `object + 0x14` (thiscall, the callee pops the word).
pub fn hkp_point_to_path_constraint_data_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    flag: u32,
) {
    if object.is_null() {
        return;
    }
    e.mem.set_u32(object.addr(), 0x010d_a004);
    if flag != 0 {
        let member = object.addr() + 0xc;
        let word = e.mem.u32(member + 8);
        e.call(0x00d2_1ce0, &args![member, word]);
    }
}

// Translated from 00dfff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMalleableConstraintData` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_malleable_constraint_data_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMalleableConstraintData` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00cdf760` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hkp_malleable_constraint_data_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00cd_f760, &args![object, flag]);
    }
}

// Translated from 00dfffc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpLinearParametricCurve` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_linear_parametric_curve_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpLinearParametricCurve` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010e_80b4 at `+0`; the flag (second word) is not read.
pub fn hkp_linear_parametric_curve_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_80b4);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_ffa0, fn_00dfffa0() -> u32),
        entry!(0x00dffdb0, return_constant_00dffdb0() -> u32),
        entry!(0x00dffe10, return_constant_00dffe10() -> u32),
        entry!(0x00dffe50, return_constant_00dffe50() -> u32),
        entry!(0x00dffeb0, return_constant_00dffeb0() -> u32),
        entry!(0x00dffef0, return_constant_00dffef0() -> u32),
        entry!(0x00dfff50, return_constant_00dfff50() -> u32),
        entry!(0x00dffff0, return_constant_00dffff0() -> u32),
        entry!(
            0x00df_fd80,
            hkp_velocity_constraint_motor_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_fda0,
            hkp_velocity_constraint_motor_finish_loaded_object(Ptr, u32)
        ),
        entry!(0x00df_fdc0, hkp_thin_box_motion_cleanup_loaded_object(Ptr)),
        entry!(
            0x00df_fde0,
            hkp_thin_box_motion_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_fe20,
            hkp_spring_damper_constraint_motor_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_fe40,
            hkp_spring_damper_constraint_motor_finish_loaded_object(Ptr, u32)
        ),
        entry!(0x00df_fe60, hkp_sphere_motion_cleanup_loaded_object(Ptr)),
        entry!(
            0x00df_fe80,
            hkp_sphere_motion_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_fec0,
            hkp_position_constraint_motor_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_fee0,
            hkp_position_constraint_motor_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_ff00,
            hkp_point_to_path_constraint_data_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_ff20,
            hkp_point_to_path_constraint_data_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_ff60,
            hkp_malleable_constraint_data_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_ff80,
            hkp_malleable_constraint_data_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_ffc0,
            hkp_linear_parametric_curve_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_ffe0,
            hkp_linear_parametric_curve_finish_loaded_object(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod a07_first_word_tests {
    use crate::prelude::*;

    const BUILT_WORD: u32 = 0x0abc_def0;

    /// A double over `callee` writes `BUILT_WORD` at the buffer it is given; `function`
    /// must call it once with that buffer and a zero flag word, and return the word.
    fn check(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |e, a| {
            e.mem.set_u32(a[0], BUILT_WORD);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(function, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0].1[0], 0);
        assert_eq!(calls[0].1[1], 0);
        assert_eq!(result, BUILT_WORD);
    }

    macro_rules! first_word_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check($function, $callee);
            }
        };
    }

    first_word_test!(
        fn_00dfffa0_returns_first_word_of_built_object,
        0x00df_ffa0,
        0x00cd_f760
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type ConstructorLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Double for a constructor: logs its argument words and writes `vtable` at the start of `this`.
    fn constructor_double(e: &mut Engine, address: u32, vtable: u32) -> ConstructorLog {
        let log: ConstructorLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn return_constant_00dffdb0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fdb0, &args![]).u32(), 0x010c_c0d8);
    }

    #[test]
    fn return_constant_00dffe10_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fe10, &args![]).u32(), 0x010c_6e34);
    }

    #[test]
    fn return_constant_00dffe50_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fe50, &args![]).u32(), 0x010d_3838);
    }

    #[test]
    fn return_constant_00dffeb0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_feb0, &args![]).u32(), 0x010c_6d54);
    }

    #[test]
    fn return_constant_00dffef0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fef0, &args![]).u32(), 0x010c_b798);
    }

    #[test]
    fn return_constant_00dfff50_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ff50, &args![]).u32(), 0x010d_a004);
    }

    #[test]
    fn fn_00dfffa0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00cd_f760, 0x0a0b_0c0d);
        let result = e.call(0x00df_ffa0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dffff0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fff0, &args![]).u32(), 0x010e_80b4);
    }
}

#[cfg(test)]
mod a04_type_info_callback_tests {
    use crate::prelude::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn hkp_velocity_constraint_motor_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_000e, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_000e);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_fd80, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_000e), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_velocity_constraint_motor_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00df_fda0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_c0d8);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00df_fda0, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_thin_box_motion_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_000f, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_000f);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_fdc0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_000f), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_thin_box_motion_finish_loaded_object_sets_the_half_one_only_with_the_flag() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x130);
        e.call(0x00df_fde0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010c_6e34);
        assert_eq!(e.mem.u16(object + 0x11e), 0);
        e.call(0x00df_fde0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_6e34);
        assert_eq!(e.mem.u16(object + 0x11e), 0x3f80);
        // A null object is left alone.
        e.call(0x00df_fde0, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_spring_damper_constraint_motor_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0010, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0010);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_fe20, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0010), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_spring_damper_constraint_motor_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00df_fe40, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010d_3838);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00df_fe40, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_sphere_motion_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0011, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0011);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_fe60, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0011), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_sphere_motion_finish_loaded_object_sets_the_half_one_only_with_the_flag() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x130);
        e.call(0x00df_fe80, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010c_6d54);
        assert_eq!(e.mem.u16(object + 0x11e), 0);
        e.call(0x00df_fe80, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_6d54);
        assert_eq!(e.mem.u16(object + 0x11e), 0x3f80);
        // A null object is left alone.
        e.call(0x00df_fe80, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_position_constraint_motor_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0012, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0012);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_fec0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0012), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_position_constraint_motor_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00df_fee0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_b798);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00df_fee0, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_point_to_path_constraint_data_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0013, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0013);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_ff00, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0013), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_point_to_path_constraint_data_finish_loaded_object_runs_the_helper_on_the_member_only_with_the_flag(
    ) {
        let mut e = Engine::new();
        e.register(0x00d2_1ce0, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x14, 0xabcd);
        e.call_log = Some(Vec::new());
        e.call(0x00df_ff20, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010d_a004);
        assert!(calls_to(&e, 0x00d2_1ce0).is_empty());
        e.call(0x00df_ff20, &args![object, 1u32]);
        assert_eq!(calls_to(&e, 0x00d2_1ce0), vec![vec![object + 0xc, 0xabcd]]);
        // A null object is left alone.
        e.call(0x00df_ff20, &args![0u32, 1u32]);
        assert_eq!(calls_to(&e, 0x00d2_1ce0).len(), 1);
    }

    #[test]
    fn hkp_malleable_constraint_data_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0014, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0014);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_ff60, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0014), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_malleable_constraint_data_finish_loaded_object_calls_the_helper_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00cd_f760, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_ff80, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00cd_f760), vec![vec![0x5000, 7]]);
        e.call(0x00df_ff80, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00cd_f760).len(), 1);
    }

    #[test]
    fn hkp_linear_parametric_curve_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0015, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0015);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_ffc0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0015), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_linear_parametric_curve_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00df_ffe0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_80b4);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00df_ffe0, &args![0u32, 1u32]);
    }
}
