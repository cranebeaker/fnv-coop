//! `Havok SDK/run_00dfde50` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfde50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_3cc0.
pub fn return_constant_00dfde50(_e: &mut Engine) -> u32 {
    0x010d_3cc0
}

// Translated from 00dfdfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_3e5c.
pub fn return_constant_00dfdfc0(_e: &mut Engine) -> u32 {
    0x010e_3e5c
}

// Translated from 00dfdd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkxAttributeGroup` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00cb0a70` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hkx_attribute_group_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00cb_0a70, &args![object, flag]);
    }
}

// Translated from 00dfdd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkxAttributeGroup` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// tail-jumps to `00dfd840` (no stack word) with `object` in `ECX`.
pub fn hkx_attribute_group_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.call(0x00df_d840, &args![object]);
}

// Translated from 00dfdd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkxAttribute` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00e3f900` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hkx_attribute_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00e3_f900, &args![object, flag]);
    }
}

// Translated from 00dfdd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkxAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// reads the word at `object + 4`; when it is not null, runs `00c905b0` on it (no stack word); clears that word; then tail-jumps to `00cb0c00` on `object`. No null check on `object`.
pub fn hkx_attribute_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    let base = object.addr();
    let held = e.mem.u32(base + 4);
    if held != 0 {
        e.call(0x00c9_05b0, &args![held]);
    }
    e.mem.set_u32(base + 4, 0);
    e.call(0x00cb_0c00, &args![object]);
}

// Translated from 00dfddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkPackfileSectionHeader` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_packfile_section_header_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfddd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkPackfileHeader` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_packfile_header_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfddf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkTraceStreamTitle` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_trace_stream_title_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfde10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkTraceStream` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hk_trace_stream_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfde30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkTraceStream` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores 1 in the 16-bit word at `+6` and the vtable address 0x010d_3cc0 at `+0`; the flag (second word) is not read.
pub fn hk_trace_stream_finish_loaded_object(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if !object.is_null() {
        e.mem.set_u16(object.addr() + 6, 1);
        e.mem.set_u32(object.addr(), 0x010d_3cc0);
    }
}

// Translated from 00dfde60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkSweptTransform` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_swept_transform_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfde80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkSphere` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_sphere_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfdea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMotionState` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_motion_state_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfdec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMonitorStreamStringMapStringMap` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00e0bf30` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hk_monitor_stream_string_map_string_map_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    flag: u32,
) {
    if !object.is_null() {
        e.call(0x00e0_bf30, &args![object, flag]);
    }
}

// Translated from 00dfdef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMonitorStreamStringMap` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00e0f700` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hk_monitor_stream_string_map_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00e0_f700, &args![object, flag]);
    }
}

// Translated from 00dfdf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMonitorStreamFrameInfo` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_monitor_stream_frame_info_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfdf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMonitorStreamColorTable` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hk_monitor_stream_color_table_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfdf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMonitorStreamColorTableColorPair` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00cb0a70` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hk_monitor_stream_color_table_color_pair_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    flag: u32,
) {
    if !object.is_null() {
        e.call(0x00cb_0a70, &args![object, flag]);
    }
}

// Translated from 00dfdf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMonitorStreamStringMapStringMap` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// tail-jumps to `00cb0c00` (no stack word) with `object + 8` in `ECX`.
pub fn hk_monitor_stream_string_map_string_map_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.call(0x00cb_0c00, &args![object.addr() + 8]);
}

// Translated from 00dfdfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMonitorStreamColorTableColorPair` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// tail-jumps to `00cb0c00` (no stack word) with `object` in `ECX`.
pub fn hk_monitor_stream_color_table_color_pair_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.call(0x00cb_0c00, &args![object]);
}

// Translated from 00dfdfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMonitorStreamColorTable` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010e_3e5c at `+0`; the flag (second word) is not read.
pub fn hk_monitor_stream_color_table_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_3e5c);
    }
}

// Translated from 00dfdfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMonitorStreamStringMap` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// tail-jumps to `00ddc870` (no stack word) with `object` in `ECX`.
pub fn hk_monitor_stream_string_map_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.call(0x00dd_c870, &args![object]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfde50, return_constant_00dfde50() -> u32),
        entry!(0x00dfdfc0, return_constant_00dfdfc0() -> u32),
        entry!(
            0x00df_dd30,
            hkx_attribute_group_finish_loaded_object(Ptr, u32)
        ),
        entry!(0x00df_dd50, hkx_attribute_group_cleanup_loaded_object(Ptr)),
        entry!(0x00df_dd60, hkx_attribute_finish_loaded_object(Ptr, u32)),
        entry!(0x00df_dd90, hkx_attribute_cleanup_loaded_object(Ptr)),
        entry!(
            0x00df_ddb0,
            hk_packfile_section_header_cleanup_loaded_object(u32)
        ),
        entry!(0x00df_ddd0, hk_packfile_header_cleanup_loaded_object(u32)),
        entry!(
            0x00df_ddf0,
            hk_trace_stream_title_cleanup_loaded_object(u32)
        ),
        entry!(0x00df_de10, hk_trace_stream_cleanup_loaded_object(Ptr)),
        entry!(0x00df_de30, hk_trace_stream_finish_loaded_object(Ptr, u32)),
        entry!(0x00df_de60, hk_swept_transform_cleanup_loaded_object(u32)),
        entry!(0x00df_de80, hk_sphere_cleanup_loaded_object(u32)),
        entry!(0x00df_dea0, hk_motion_state_cleanup_loaded_object(u32)),
        entry!(
            0x00df_dec0,
            hk_monitor_stream_string_map_string_map_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_def0,
            hk_monitor_stream_string_map_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_df20,
            hk_monitor_stream_frame_info_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_df50,
            hk_monitor_stream_color_table_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_df70,
            hk_monitor_stream_color_table_color_pair_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_df90,
            hk_monitor_stream_string_map_string_map_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_dfa0,
            hk_monitor_stream_color_table_color_pair_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_dfb0,
            hk_monitor_stream_color_table_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_dfd0,
            hk_monitor_stream_string_map_cleanup_loaded_object(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfde50_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_de50, &args![]).u32(), 0x010d_3cc0);
    }

    #[test]
    fn return_constant_00dfdfc0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_dfc0, &args![]).u32(), 0x010e_3e5c);
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
    fn hkx_attribute_group_finish_loaded_object_calls_the_helper_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_dd30, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00cb_0a70), vec![vec![0x5000, 7]]);
        e.call(0x00df_dd30, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00cb_0a70).len(), 1);
    }

    #[test]
    fn hkx_attribute_group_cleanup_loaded_object_runs_the_helper() {
        let mut e = Engine::new();
        e.register(0x00df_d840, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_dd50, &args![0x5000u32]);
        assert_eq!(calls_to(&e, 0x00df_d840), vec![vec![0x5000]]);
    }

    #[test]
    fn hkx_attribute_finish_loaded_object_calls_the_helper_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00e3_f900, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_dd60, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e3_f900), vec![vec![0x5000, 7]]);
        e.call(0x00df_dd60, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e3_f900).len(), 1);
    }

    #[test]
    fn hkx_attribute_cleanup_loaded_object_releases_the_held_object_and_clears_the_word() {
        let mut e = Engine::new();
        e.register(0x00c9_05b0, |_, _| Ret::default());
        e.register(0x00cb_0c00, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 4, 0x7000);
        e.call_log = Some(Vec::new());
        e.call(0x00df_dd90, &args![object]);
        assert_eq!(calls_to(&e, 0x00c9_05b0), vec![vec![0x7000]]);
        assert_eq!(calls_to(&e, 0x00cb_0c00), vec![vec![object]]);
        assert_eq!(e.mem.u32(object + 4), 0);
        // Nothing held: the release is skipped.
        e.call(0x00df_dd90, &args![object]);
        assert_eq!(calls_to(&e, 0x00c9_05b0).len(), 1);
        assert_eq!(calls_to(&e, 0x00cb_0c00).len(), 2);
    }

    #[test]
    fn hk_packfile_section_header_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_ddb0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_packfile_header_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_ddd0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_trace_stream_title_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_ddf0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_trace_stream_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0009, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0009);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_de10, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0009), vec![vec![object, 0]]);
    }

    #[test]
    fn hk_trace_stream_finish_loaded_object_sets_the_word_and_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00df_de30, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010d_3cc0);
        assert_eq!(e.mem.u16(object + 6), 1);
        assert_eq!(e.mem.u16(object + 4), 0);
        // A null object is left alone.
        e.call(0x00df_de30, &args![0u32, 1u32]);
    }

    #[test]
    fn hk_swept_transform_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_de60, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_sphere_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_de80, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_motion_state_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_dea0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_monitor_stream_string_map_string_map_finish_loaded_object_calls_the_helper_for_a_non_null_object(
    ) {
        let mut e = Engine::new();
        e.register(0x00e0_bf30, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_dec0, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e0_bf30), vec![vec![0x5000, 7]]);
        e.call(0x00df_dec0, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e0_bf30).len(), 1);
    }

    #[test]
    fn hk_monitor_stream_string_map_finish_loaded_object_calls_the_helper_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00e0_f700, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_def0, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e0_f700), vec![vec![0x5000, 7]]);
        e.call(0x00df_def0, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e0_f700).len(), 1);
    }

    #[test]
    fn hk_monitor_stream_frame_info_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_df20, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_monitor_stream_color_table_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_000a, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_000a);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_df50, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_000a), vec![vec![object, 0]]);
    }

    #[test]
    fn hk_monitor_stream_color_table_color_pair_finish_loaded_object_calls_the_helper_for_a_non_null_object(
    ) {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_df70, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00cb_0a70), vec![vec![0x5000, 7]]);
        e.call(0x00df_df70, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00cb_0a70).len(), 1);
    }

    #[test]
    fn hk_monitor_stream_string_map_string_map_cleanup_loaded_object_runs_the_helper() {
        let mut e = Engine::new();
        e.register(0x00cb_0c00, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_df90, &args![0x5000u32]);
        assert_eq!(calls_to(&e, 0x00cb_0c00), vec![vec![0x5008]]);
    }

    #[test]
    fn hk_monitor_stream_color_table_color_pair_cleanup_loaded_object_runs_the_helper() {
        let mut e = Engine::new();
        e.register(0x00cb_0c00, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_dfa0, &args![0x5000u32]);
        assert_eq!(calls_to(&e, 0x00cb_0c00), vec![vec![0x5000]]);
    }

    #[test]
    fn hk_monitor_stream_color_table_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00df_dfb0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_3e5c);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00df_dfb0, &args![0u32, 1u32]);
    }

    #[test]
    fn hk_monitor_stream_string_map_cleanup_loaded_object_runs_the_helper() {
        let mut e = Engine::new();
        e.register(0x00dd_c870, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_dfd0, &args![0x5000u32]);
        assert_eq!(calls_to(&e, 0x00dd_c870), vec![vec![0x5000]]);
    }
}
