//! `Havok SDK/run_00da0910` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00da0910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6bec.
pub fn return_constant_00da0910(_e: &mut Engine) -> u32 {
    0x010c_6bec
}

// Translated from 00da0950 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6c64.
pub fn return_constant_00da0950(_e: &mut Engine) -> u32 {
    0x010c_6c64
}

// Translated from 00da0b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6fd0.
pub fn return_constant_00da0b20(_e: &mut Engine) -> u32 {
    0x010c_6fd0
}

// Translated from 00da08a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpKeyframedRigidMotion` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_keyframed_rigid_motion_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00da08c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMaxSizeMotion` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_max_size_motion_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00da08e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpKeyframedRigidMotion` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the base vtable address 0x010c_59f4 at `+0`; when the flag is not zero it stores the high half (`SAR 16`) of the float 1.0 (`0x3f80`) in the 16-bit word at `+0x11e`; then stores the vtable address 0x010c_6bec at `+0`.
pub fn hkp_keyframed_rigid_motion_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
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
    e.mem.set_u32(base, 0x010c_6bec);
}

// Translated from 00da0920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMaxSizeMotion` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the base vtable address 0x010c_59f4 at `+0`; when the flag is not zero it stores the high half (`SAR 16`) of the float 1.0 (`0x3f80`) in the 16-bit word at `+0x11e`; then stores the vtable address 0x010c_6c64 at `+0`.
pub fn hkp_max_size_motion_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
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
    e.mem.set_u32(base, 0x010c_6c64);
}

// Translated from 00da0960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMaterial` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hkp_material_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da0980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMaterial` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// an empty function (a lone `RET`) that reads neither word.
pub fn hkp_material_finish_loaded_object(_e: &mut Engine, _unused_0: u32, _unused_1: u32) {}

// Translated from 00da0990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkRangeRealAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_range_real_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da09b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkRangeInt32Attribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_range_int32_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da09d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkUiAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_ui_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da09f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkGizmoAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_gizmo_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da0a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkModelerNodeTypeAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_modeler_node_type_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da0a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkLinkAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_link_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da0a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkSemanticsAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_semantics_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da0a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkDescriptionAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_description_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da0a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkArrayTypeAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_array_type_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da0ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkDataObjectTypeAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_data_object_type_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkDocumentationAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_documentation_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00da0af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpSingleShapeContainer` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_single_shape_container_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00da0b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpSingleShapeContainer` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010c_6fd0 at `+0`; the flag (second word) is not read.
pub fn hkp_single_shape_container_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010c_6fd0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da0910, return_constant_00da0910() -> u32),
        entry!(0x00da0950, return_constant_00da0950() -> u32),
        entry!(0x00da0b20, return_constant_00da0b20() -> u32),
        entry!(
            0x00da_08a0,
            hkp_keyframed_rigid_motion_cleanup_loaded_object(Ptr)
        ),
        entry!(0x00da_08c0, hkp_max_size_motion_cleanup_loaded_object(Ptr)),
        entry!(
            0x00da_08e0,
            hkp_keyframed_rigid_motion_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00da_0920,
            hkp_max_size_motion_finish_loaded_object(Ptr, u32)
        ),
        entry!(0x00da_0960, hkp_material_cleanup_loaded_object(u32)),
        entry!(0x00da_0980, hkp_material_finish_loaded_object(u32, u32)),
        entry!(
            0x00da_0990,
            hk_range_real_attribute_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00da_09b0,
            hk_range_int32_attribute_cleanup_loaded_object(u32)
        ),
        entry!(0x00da_09d0, hk_ui_attribute_cleanup_loaded_object(u32)),
        entry!(0x00da_09f0, hk_gizmo_attribute_cleanup_loaded_object(u32)),
        entry!(
            0x00da_0a10,
            hk_modeler_node_type_attribute_cleanup_loaded_object(u32)
        ),
        entry!(0x00da_0a30, hk_link_attribute_cleanup_loaded_object(u32)),
        entry!(
            0x00da_0a50,
            hk_semantics_attribute_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00da_0a70,
            hk_description_attribute_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00da_0a90,
            hk_array_type_attribute_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00da_0ab0,
            hk_data_object_type_attribute_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00da_0ad0,
            hk_documentation_attribute_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00da_0af0,
            hkp_single_shape_container_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00da_0b10,
            hkp_single_shape_container_finish_loaded_object(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00da0910_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00da_0910, &args![]).u32(), 0x010c_6bec);
    }

    #[test]
    fn return_constant_00da0950_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00da_0950, &args![]).u32(), 0x010c_6c64);
    }

    #[test]
    fn return_constant_00da0b20_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00da_0b20, &args![]).u32(), 0x010c_6fd0);
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
    fn hkp_keyframed_rigid_motion_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0001, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0001);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00da_08a0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0001), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_max_size_motion_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0002, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0002);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00da_08c0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0002), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_keyframed_rigid_motion_finish_loaded_object_sets_the_half_one_only_with_the_flag() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x130);
        e.call(0x00da_08e0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010c_6bec);
        assert_eq!(e.mem.u16(object + 0x11e), 0);
        e.call(0x00da_08e0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_6bec);
        assert_eq!(e.mem.u16(object + 0x11e), 0x3f80);
        // A null object is left alone.
        e.call(0x00da_08e0, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_max_size_motion_finish_loaded_object_sets_the_half_one_only_with_the_flag() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x130);
        e.call(0x00da_0920, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010c_6c64);
        assert_eq!(e.mem.u16(object + 0x11e), 0);
        e.call(0x00da_0920, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_6c64);
        assert_eq!(e.mem.u16(object + 0x11e), 0x3f80);
        // A null object is left alone.
        e.call(0x00da_0920, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_material_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0960, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hkp_material_finish_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0980, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_range_real_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0990, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_range_int32_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_09b0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_ui_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_09d0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_gizmo_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_09f0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_modeler_node_type_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0a10, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_link_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0a30, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_semantics_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0a50, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_description_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0a70, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_array_type_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0a90, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_data_object_type_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0ab0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_documentation_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0ad0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hkp_single_shape_container_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0003, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0003);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00da_0af0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0003), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_single_shape_container_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00da_0b10, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_6fd0);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00da_0b10, &args![0u32, 1u32]);
    }
}
