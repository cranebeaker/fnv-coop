//! `Havok SDK/run_00dfb2b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfb590 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_dc5c.
pub fn return_constant_00dfb590(_e: &mut Engine) -> u32 {
    0x010d_dc5c
}

// Translated from 00dfb2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00dfb2d0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfb2b0(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00df_b2d0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfb400 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x20-byte stack buffer by calling `00e2ea20` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfb400(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, buffer| {
        e.call(0x00e2_ea20, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfb460 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e2ebc0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfb460(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e2_ebc0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfb2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`; the class is unconfirmed). It writes the vtable word at `+0` with four
/// values in turn (`010305a8`, `010ce3c8`, `010d0d70`, `010d0dac`) and the word at `+0x30`
/// (`010c6fd0`); when `mode` is not zero the intermediate stages also write `+0xc` (0, then 8)
/// and the byte at `+0x10` (2, then 0). When `mode` is 1 it sets `+0xc` to `0xa` and copies the
/// 16 bytes at `[this + 0x14] + 0x10` to `this + 0x20` (`MOVAPS`, which faults on an address
/// that is not 16-byte aligned; here a plain copy) and the words at `[this + 0x14] + 0x20` and
/// `+ 0x24` to `this + 0x18` and `this + 0x1c`. Returns `this` (`EAX`).
pub fn fn_00dfb2d0(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base, 0x010c_e3c8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 8);
        e.mem.set_u8(base + 0x10, 2);
    }
    e.mem.set_u32(base, 0x010d_0d70);
    if mode != 0 {
        e.mem.set_u8(base + 0x10, 0);
    }
    e.mem.set_u32(base, 0x010d_0dac);
    e.mem.set_u32(base + 0x30, 0x010c_6fd0);
    if mode == 1 {
        let source = e.mem.u32(base + 0x14);
        e.mem.set_u32(base + 0xc, 0xa);
        for lane in 0..4 {
            let word = e.mem.u32(source + 0x10 + 4 * lane);
            e.mem.set_u32(base + 0x20 + 4 * lane, word);
        }
        let low = e.mem.u32(source + 0x20);
        e.mem.set_u32(base + 0x18, low);
        let high = e.mem.u32(source + 0x24);
        e.mem.set_u32(base + 0x1c, high);
    }
    this
}

// Translated from 00dfb250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpVehicleSuspensionSuspensionWheelParameters` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hkp_vehicle_suspension_suspension_wheel_parameters_cleanup_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
) {
}

// Translated from 00dfb270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMoppBvTreeShape` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_mopp_bv_tree_shape_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfb290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMoppBvTreeShape` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, runs `00dfb2d0` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it); the callee is in this file.
pub fn hkp_mopp_bv_tree_shape_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        fn_00dfb2d0(e, object, flag);
    }
}

// Translated from 00dfb350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMeshSection` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_mesh_section_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfb370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMeshSectionCinfo` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_mesh_section_cinfo_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfb390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMemoryResourceHandleExternalLink` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, runs `00cb0a70` on `object` and then on `object + 4`, each with the flag (thiscall, the callee pops the word).
pub fn hk_memory_resource_handle_external_link_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    flag: u32,
) {
    if !object.is_null() {
        e.call(0x00cb_0a70, &args![object, flag]);
        e.call(0x00cb_0a70, &args![object.addr() + 4, flag]);
    }
}

// Translated from 00dfb3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMemoryResourceHandle` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00e2ea20` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hk_memory_resource_handle_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00e2_ea20, &args![object, flag]);
    }
}

// Translated from 00dfb3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMemoryResourceHandle` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hk_memory_resource_handle_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfb430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMemoryResourceContainer` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00e2ebc0` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hk_memory_resource_container_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00e2_ebc0, &args![object, flag]);
    }
}

// Translated from 00dfb450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMemoryResourceContainer` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hk_memory_resource_container_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfb490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkMemoryResourceHandleExternalLink` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// runs `00cb0c00` (no stack word) on `object + 4` and then on `object` (the second as a tail jump); no null check.
pub fn hk_memory_resource_handle_external_link_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.call(0x00cb_0c00, &args![object.addr() + 4]);
    e.call(0x00cb_0c00, &args![object]);
}

// Translated from 00dfb4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaKeyFrameHierarchyUtilityControlData` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// an empty function (a lone `RET`) that reads neither word.
pub fn hka_key_frame_hierarchy_utility_control_data_finish_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
) {
}

// Translated from 00dfb4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaKeyFrameHierarchyUtilityControlData` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hka_key_frame_hierarchy_utility_control_data_cleanup_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
) {
}

// Translated from 00dfb4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaKeyFrameHierarchyUtility` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hka_key_frame_hierarchy_utility_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfb500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaWaveletCompressedAnimationCompressionParams` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hka_wavelet_compressed_animation_compression_params_cleanup_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
) {
}

// Translated from 00dfb520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaWaveletCompressedAnimationQuantizationFormat` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// an empty function (a lone `RET`) that reads neither word.
pub fn hka_wavelet_compressed_animation_quantization_format_finish_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
) {
}

// Translated from 00dfb530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaWaveletCompressedAnimationQuantizationFormat` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hka_wavelet_compressed_animation_quantization_format_cleanup_loaded_object(
    _e: &mut Engine,
    _unused_0: u32,
) {
}

// Translated from 00dfb550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaWaveletCompressedAnimation` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hka_wavelet_compressed_animation_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfb570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaWaveletCompressedAnimation` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010d_dc5c at `+0`, and when the flag is not zero runs `00e2fc10` on `object` (a tail jump, no stack word).
pub fn hka_wavelet_compressed_animation_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    flag: u32,
) {
    if object.is_null() {
        return;
    }
    e.mem.set_u32(object.addr(), 0x010d_dc5c);
    if flag != 0 {
        e.call(0x00e2_fc10, &args![object]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_b2b0, fn_00dfb2b0() -> u32),
        entry!(0x00df_b400, fn_00dfb400() -> u32),
        entry!(0x00df_b460, fn_00dfb460() -> u32),
        entry!(0x00dfb590, return_constant_00dfb590() -> u32),
        entry!(0x00df_b2d0, fn_00dfb2d0(Ptr, u32) -> Ptr),
        entry!(
            0x00df_b250,
            hkp_vehicle_suspension_suspension_wheel_parameters_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_b270,
            hkp_mopp_bv_tree_shape_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_b290,
            hkp_mopp_bv_tree_shape_finish_loaded_object(Ptr, u32)
        ),
        entry!(0x00df_b350, hk_mesh_section_cleanup_loaded_object(u32)),
        entry!(
            0x00df_b370,
            hk_mesh_section_cinfo_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_b390,
            hk_memory_resource_handle_external_link_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_b3d0,
            hk_memory_resource_handle_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_b3f0,
            hk_memory_resource_handle_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_b430,
            hk_memory_resource_container_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_b450,
            hk_memory_resource_container_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_b490,
            hk_memory_resource_handle_external_link_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_b4b0,
            hka_key_frame_hierarchy_utility_control_data_finish_loaded_object(u32, u32)
        ),
        entry!(
            0x00df_b4c0,
            hka_key_frame_hierarchy_utility_control_data_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_b4e0,
            hka_key_frame_hierarchy_utility_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_b500,
            hka_wavelet_compressed_animation_compression_params_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_b520,
            hka_wavelet_compressed_animation_quantization_format_finish_loaded_object(u32, u32)
        ),
        entry!(
            0x00df_b530,
            hka_wavelet_compressed_animation_quantization_format_cleanup_loaded_object(u32)
        ),
        entry!(
            0x00df_b550,
            hka_wavelet_compressed_animation_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_b570,
            hka_wavelet_compressed_animation_finish_loaded_object(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfb590_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_b590, &args![]).u32(), 0x010d_dc5c);
    }
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
        fn_00dfb2b0_returns_first_word_of_built_object,
        0x00df_b2b0,
        0x00df_b2d0
    );
    first_word_test!(
        fn_00dfb400_returns_first_word_of_built_object,
        0x00df_b400,
        0x00e2_ea20
    );
    first_word_test!(
        fn_00dfb460_returns_first_word_of_built_object,
        0x00df_b460,
        0x00e2_ebc0
    );
}

#[cfg(test)]
mod record_tests {
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
    fn fn_00dfb2b0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00df_b2d0, 0x0a0b_0c0d);
        let result = e.call(0x00df_b2b0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfb400_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e2_ea20, 0x0a0b_0c0d);
        let result = e.call(0x00df_b400, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfb460_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e2_ebc0, 0x0a0b_0c0d);
        let result = e.call(0x00df_b460, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// Mode 0 writes only the vtable (final value) and the word at +0x30.
    #[test]
    fn fn_00dfb2d0_mode_zero_writes_the_vtable_only() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0x77; 0x40]);
        assert_eq!(e.call(0x00df_b2d0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_0dac);
        assert_eq!(e.mem.u32(this + 0x30), 0x010c_6fd0);
        assert_eq!(e.mem.u32(this + 0xc), 0x7777_7777);
        assert_eq!(e.mem.u8(this + 0x10), 0x77);
        assert_eq!(e.mem.u32(this + 0x20), 0x7777_7777);
    }

    /// Mode 2 (not 0, not 1): the stages clear +0xc... to 8 and the byte to 0, no copy.
    #[test]
    fn fn_00dfb2d0_other_modes_leave_the_staged_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0x77; 0x40]);
        e.call(0x00df_b2d0, &args![this, 2u32]);
        assert_eq!(e.mem.u32(this), 0x010d_0dac);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x7777_7777);
        assert_eq!(e.mem.u32(this + 0x18), 0x7777_7777);
    }

    /// Mode 1 also copies the source's vector and words and sets +0xc to 0xa.
    #[test]
    fn fn_00dfb2d0_mode_one_copies_from_the_source() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let source = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x14, source);
        for word in 0..12u32 {
            e.mem.set_u32(source + 4 * word, 0x100 + word);
        }
        e.call(0x00df_b2d0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0xa);
        for lane in 0..4u32 {
            assert_eq!(e.mem.u32(this + 0x20 + 4 * lane), 0x104 + lane);
        }
        assert_eq!(e.mem.u32(this + 0x18), 0x108);
        assert_eq!(e.mem.u32(this + 0x1c), 0x109);
        assert_eq!(e.mem.u8(this + 0x10), 0);
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
    fn hkp_vehicle_suspension_suspension_wheel_parameters_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b250, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hkp_mopp_bv_tree_shape_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0004, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0004);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b270, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_mopp_bv_tree_shape_finish_loaded_object_runs_the_constructor_chain_on_a_non_null_object()
    {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00df_b290, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010d_0dac);
        assert_eq!(e.mem.u32(object + 0x30), 0x010c_6fd0);
        // A null object is left alone (nothing is written or read).
        e.call(0x00df_b290, &args![0u32, 1u32]);
    }

    #[test]
    fn hk_mesh_section_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b350, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_mesh_section_cinfo_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b370, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_memory_resource_handle_external_link_finish_loaded_object_runs_the_helper_on_both_members(
    ) {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_b390, &args![0x5000u32, 9u32]);
        assert_eq!(
            calls_to(&e, 0x00cb_0a70),
            vec![vec![0x5000, 9], vec![0x5004, 9]]
        );
        e.call(0x00df_b390, &args![0u32, 9u32]);
        assert_eq!(calls_to(&e, 0x00cb_0a70).len(), 2);
    }

    #[test]
    fn hk_memory_resource_handle_finish_loaded_object_calls_the_helper_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00e2_ea20, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_b3d0, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e2_ea20), vec![vec![0x5000, 7]]);
        e.call(0x00df_b3d0, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e2_ea20).len(), 1);
    }

    #[test]
    fn hk_memory_resource_handle_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0005, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0005);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b3f0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0005), vec![vec![object, 0]]);
    }

    #[test]
    fn hk_memory_resource_container_finish_loaded_object_calls_the_helper_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00e2_ebc0, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_b430, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e2_ebc0), vec![vec![0x5000, 7]]);
        e.call(0x00df_b430, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00e2_ebc0).len(), 1);
    }

    #[test]
    fn hk_memory_resource_container_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0006, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0006);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b450, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0006), vec![vec![object, 0]]);
    }

    #[test]
    fn hk_memory_resource_handle_external_link_cleanup_loaded_object_runs_the_helper_on_both_members(
    ) {
        let mut e = Engine::new();
        e.register(0x00cb_0c00, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_b490, &args![0x5000u32]);
        assert_eq!(calls_to(&e, 0x00cb_0c00), vec![vec![0x5004], vec![0x5000]]);
    }

    #[test]
    fn hka_key_frame_hierarchy_utility_control_data_finish_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b4b0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hka_key_frame_hierarchy_utility_control_data_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b4c0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hka_key_frame_hierarchy_utility_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b4e0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hka_wavelet_compressed_animation_compression_params_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b500, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hka_wavelet_compressed_animation_quantization_format_finish_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b520, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hka_wavelet_compressed_animation_quantization_format_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b530, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hka_wavelet_compressed_animation_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0007, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0007);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b550, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0007), vec![vec![object, 0]]);
    }

    #[test]
    fn hka_wavelet_compressed_animation_finish_loaded_object_stores_the_vtable_and_runs_the_helper_with_the_flag(
    ) {
        let mut e = Engine::new();
        e.register(0x00e2_fc10, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.call_log = Some(Vec::new());
        e.call(0x00df_b570, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010d_dc5c);
        assert!(calls_to(&e, 0x00e2_fc10).is_empty());
        e.call(0x00df_b570, &args![object, 1u32]);
        assert_eq!(calls_to(&e, 0x00e2_fc10), vec![vec![object]]);
        // A null object is left alone.
        e.call(0x00df_b570, &args![0u32, 1u32]);
        assert_eq!(calls_to(&e, 0x00e2_fc10).len(), 1);
    }
}
