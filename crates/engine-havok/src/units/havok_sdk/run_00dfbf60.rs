//! `Havok SDK/run_00dfbf60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfbf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_f89c.
pub fn return_constant_00dfbf60(_e: &mut Engine) -> u32 {
    0x010d_f89c
}

// Translated from 00dfbe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVehicleFrictionDescriptionAxisDescription` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hkp_vehicle_friction_description_axis_description_cleanup_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
) {
}

// Translated from 00dfbea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVehicleFrictionDescription` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hkp_vehicle_friction_description_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfbec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVehicleFrictionStatusAxisStatus` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hkp_vehicle_friction_status_axis_status_cleanup_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
) {
}

// Translated from 00dfbee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVehicleFrictionStatus` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hkp_vehicle_friction_status_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfbf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVehicleFrictionStatus` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// an empty function (a lone `RET`) that reads neither word.
pub fn hkp_vehicle_friction_status_finish_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
) {
}

// Translated from 00dfbf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVehicleDefaultSuspensionWheelSpringSuspension` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hkp_vehicle_default_suspension_wheel_spring_suspension_cleanup_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
) {
}

// Translated from 00dfbf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVehicleDefaultSuspension` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_vehicle_default_suspension_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfbf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVehicleDefaultSuspension` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010d_f89c at `+0`; the flag (second word) is not read.
pub fn hkp_vehicle_default_suspension_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010d_f89c);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfbf60, return_constant_00dfbf60() -> u32),
        entry!(
            0x00df_be80,
            hkp_vehicle_friction_description_axis_description_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_bea0,
            hkp_vehicle_friction_description_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_bec0,
            hkp_vehicle_friction_status_axis_status_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_bee0,
            hkp_vehicle_friction_status_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_bf00,
            hkp_vehicle_friction_status_finish_loaded_object(u32, u32)
        ),
        entry!(
            0x00df_bf10,
            hkp_vehicle_default_suspension_wheel_spring_suspension_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_bf30,
            hkp_vehicle_default_suspension_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_bf50,
            hkp_vehicle_default_suspension_finish_loaded_object(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfbf60_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_bf60, &args![]).u32(), 0x010d_f89c);
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
    fn hkp_vehicle_friction_description_axis_description_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_be80, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hkp_vehicle_friction_description_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_bea0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hkp_vehicle_friction_status_axis_status_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_bec0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hkp_vehicle_friction_status_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_bee0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hkp_vehicle_friction_status_finish_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_bf00, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hkp_vehicle_default_suspension_wheel_spring_suspension_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_bf10, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hkp_vehicle_default_suspension_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0008, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0008);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_bf30, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0008), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_vehicle_default_suspension_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00df_bf50, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010d_f89c);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00df_bf50, &args![0u32, 1u32]);
    }
}
