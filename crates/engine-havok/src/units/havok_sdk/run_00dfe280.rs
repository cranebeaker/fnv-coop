//! `Havok SDK/run_00dfe280` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfe280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00ce25f0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfe280(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00ce_25f0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfe2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x80-byte frame stores the vtable word 0x010c_b864 at
/// its start, runs the constructor at `00ce5360` on the member at frame+0x10 (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00dfe2f0(e: &mut Engine) -> u32 {
    e.with_stack(0x80, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_b864);
        e.call(0x00ce_5360, &args![base + 0x10, 0u32]);
        e.mem.u32(base)
    })
}

// Translated from 00dfe350 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5474.
pub fn return_constant_00dfe350(_e: &mut Engine) -> u32 {
    0x010e_5474
}

// Translated from 00dfe080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkGeometryTriangle` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_geometry_triangle_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkGeometry` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// an empty function (a lone `RET`) that reads neither word.
pub fn hk_geometry_finish_loaded_object(_e: &mut Engine, _unused_0: u32, _unused_1: u32) {}

// Translated from 00dfe0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkGeometry` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// tail-jumps to `00cce530` (no stack word) with `object` in `ECX`.
pub fn hk_geometry_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.call(0x00cc_e530, &args![object]);
}

// Translated from 00dfe0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkCustomAttributesAttribute` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_custom_attributes_attribute_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkCustomAttributes` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_custom_attributes_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkContactPointMaterial` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_contact_point_material_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkContactPoint` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_contact_point_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkColor` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_color_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkClassMember` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_class_member_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkClassEnumItem` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_class_enum_item_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkClassEnum` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_class_enum_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkClass` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// an empty function (a lone `RET`) that does not read its word.
pub fn hk_class_cleanup_loaded_object(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfe200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkBitField` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// an empty function (a lone `RET`) that reads neither word.
pub fn hk_bit_field_finish_loaded_object(_e: &mut Engine, _unused_0: u32, _unused_1: u32) {}

// Translated from 00dfe210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkBitField` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// gets the memory router from `00c85750`, reads the allocator at `router + 0x10`, clears the size word at `+4`, and when the capacity word at `+8` is not negative (bit 31 clear) calls the allocator's virtual function at vtable offset 0x10 with the buffer pointer (`+0`) and `(capacity & 0x3fff_ffff) * 4` bytes; then stores `0x8000_0000` at `+8` and 0 at `+0`. This is the clear-and-deallocate of an array of 4-byte elements. No null check.
pub fn hk_bit_field_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    let base = object.addr();
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 8);
    e.mem.set_u32(base + 4, 0);
    if (capacity as i32) >= 0 {
        let buffer = e.mem.u32(base);
        let bytes = (capacity & 0x3fff_ffff) * 4;
        e.vcall(allocator, 0x10, &args![buffer, bytes]);
    }
    e.mem.set_u32(base + 8, 0x8000_0000);
    e.mem.set_u32(base, 0);
}

// Translated from 00dfe250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaRagdollInstance` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00ce25f0` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hka_ragdoll_instance_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00ce_25f0, &args![object, flag]);
    }
}

// Translated from 00dfe270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaRagdollInstance` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hka_ragdoll_instance_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfe2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaSkeletonMapper` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hka_skeleton_mapper_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfe2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaSkeletonMapper` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010c_b864 at `+0` and runs `00ce5360` on `object + 0x10` with the flag (thiscall, the callee pops the word).
pub fn hka_skeleton_mapper_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010c_b864);
        e.call(0x00ce_5360, &args![object.addr() + 0x10, flag]);
    }
}

// Translated from 00dfe320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaInterleavedUncompressedAnimation` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hka_interleaved_uncompressed_animation_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfe340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkaInterleavedUncompressedAnimation` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010e_5474 at `+0`; the flag (second word) is not read.
pub fn hka_interleaved_uncompressed_animation_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_5474);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_e280, fn_00dfe280() -> u32),
        entry!(0x00dfe2f0, fn_00dfe2f0() -> u32),
        entry!(0x00dfe350, return_constant_00dfe350() -> u32),
        entry!(0x00df_e080, hk_geometry_triangle_cleanup_loaded_object(u32)),
        entry!(0x00df_e0b0, hk_geometry_finish_loaded_object(u32, u32)),
        entry!(0x00df_e0c0, hk_geometry_cleanup_loaded_object(Ptr)),
        entry!(
            0x00df_e0d0,
            hk_custom_attributes_attribute_cleanup_loaded_object(u32)
        ),
        entry!(0x00df_e0f0, hk_custom_attributes_cleanup_loaded_object(u32)),
        entry!(
            0x00df_e110,
            hk_contact_point_material_cleanup_loaded_object(u32)
        ),
        entry!(0x00df_e130, hk_contact_point_cleanup_loaded_object(u32)),
        entry!(0x00df_e150, hk_color_cleanup_loaded_object(u32)),
        entry!(0x00df_e170, hk_class_member_cleanup_loaded_object(u32)),
        entry!(0x00df_e190, hk_class_enum_item_cleanup_loaded_object(u32)),
        entry!(0x00df_e1b0, hk_class_enum_cleanup_loaded_object(u32)),
        entry!(0x00df_e1d0, hk_class_cleanup_loaded_object(u32)),
        entry!(0x00df_e200, hk_bit_field_finish_loaded_object(u32, u32)),
        entry!(0x00df_e210, hk_bit_field_cleanup_loaded_object(Ptr)),
        entry!(
            0x00df_e250,
            hka_ragdoll_instance_finish_loaded_object(Ptr, u32)
        ),
        entry!(0x00df_e270, hka_ragdoll_instance_cleanup_loaded_object(Ptr)),
        entry!(0x00df_e2b0, hka_skeleton_mapper_cleanup_loaded_object(Ptr)),
        entry!(
            0x00df_e2d0,
            hka_skeleton_mapper_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00df_e320,
            hka_interleaved_uncompressed_animation_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00df_e340,
            hka_interleaved_uncompressed_animation_finish_loaded_object(Ptr, u32)
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
        fn_00dfe280_returns_first_word_of_built_object,
        0x00df_e280,
        0x00ce_25f0
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

    /// What the probe saw on one call: `this`, the argument, the word `behind` bytes before `this`
    /// (the frame's vtable slot) and, when asked for, the `u32` at +0x20, `u16` at +0x24 and `u16`
    /// at +0x26 of the frame.
    type ProbeCall = (u32, u32, u32, [u32; 3]);
    type ProbeLog = Rc<RefCell<Vec<ProbeCall>>>;

    /// Double for the member constructor: logs, per call, what `ProbeCall` describes, with the
    /// vtable slot `behind[call]` bytes before `this`.
    fn frame_probe(e: &mut Engine, address: u32, behind: &[u32], with_tail: bool) -> ProbeLog {
        let log: ProbeLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let behind = behind.to_vec();
        e.register_double(address, move |e, a| {
            let call = seen.borrow().len();
            let slot = a[0] - behind[call];
            let mut tail = [0; 3];
            if with_tail {
                tail = [
                    e.mem.u32(slot + 0x20),
                    u32::from(e.mem.u16(slot + 0x24)),
                    u32::from(e.mem.u16(slot + 0x26)),
                ];
            }
            seen.borrow_mut().push((a[0], a[1], e.mem.u32(slot), tail));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00dfe280_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00ce_25f0, 0x0a0b_0c0d);
        let result = e.call(0x00df_e280, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfe2f0_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00ce_5360, &[0x10], false);
        let result = e.call(0x00df_e2f0, &args![]).u32();
        assert_eq!(result, 0x010c_b864);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_b864);
        }
    }

    #[test]
    fn return_constant_00dfe350_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_e350, &args![]).u32(), 0x010e_5474);
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
    fn hk_geometry_triangle_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e080, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_geometry_finish_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e0b0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_geometry_cleanup_loaded_object_runs_the_helper() {
        let mut e = Engine::new();
        e.register(0x00cc_e530, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_e0c0, &args![0x5000u32]);
        assert_eq!(calls_to(&e, 0x00cc_e530), vec![vec![0x5000]]);
    }

    #[test]
    fn hk_custom_attributes_attribute_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e0d0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_custom_attributes_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e0f0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_contact_point_material_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e110, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_contact_point_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e130, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_color_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e150, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_class_member_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e170, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_class_enum_item_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e190, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_class_enum_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e1b0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_class_cleanup_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e1d0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_bit_field_finish_loaded_object_does_nothing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x1234_5678);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e200, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x1234_5678);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hk_bit_field_cleanup_loaded_object_frees_the_buffer_unless_the_capacity_is_negative() {
        let mut e = Engine::new();
        e.register(0x0900_0100, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0100);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x7000);
        e.mem.set_u32(array + 4, 5);
        // Bit 30 is masked off: 6 elements of 4 bytes.
        e.mem.set_u32(array + 8, 0x4000_0006);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e210, &args![array]);
        assert_eq!(calls_to(&e, 0x0900_0100), vec![vec![allocator, 0x7000, 24]]);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        // Bit 31 set: the buffer is not owned, so nothing is freed.
        e.mem.set_u32(array, 0x7000);
        e.mem.set_u32(array + 4, 5);
        e.mem.set_u32(array + 8, 0x8000_0006);
        e.call(0x00df_e210, &args![array]);
        assert_eq!(calls_to(&e, 0x0900_0100).len(), 1);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(array), 0);
    }

    #[test]
    fn hka_ragdoll_instance_finish_loaded_object_calls_the_helper_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00ce_25f0, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00df_e250, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00ce_25f0), vec![vec![0x5000, 7]]);
        e.call(0x00df_e250, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00ce_25f0).len(), 1);
    }

    #[test]
    fn hka_ragdoll_instance_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_000b, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_000b);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e270, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_000b), vec![vec![object, 0]]);
    }

    #[test]
    fn hka_skeleton_mapper_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_000c, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_000c);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e2b0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_000c), vec![vec![object, 0]]);
    }

    #[test]
    fn hka_skeleton_mapper_finish_loaded_object_stores_the_vtable_and_runs_the_helper_on_the_member(
    ) {
        let mut e = Engine::new();
        e.register(0x00ce_5360, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e2d0, &args![object, 3u32]);
        assert_eq!(e.mem.u32(object), 0x010c_b864);
        assert_eq!(calls_to(&e, 0x00ce_5360), vec![vec![object + 0x10, 3]]);
        // A null object is left alone.
        e.call(0x00df_e2d0, &args![0u32, 3u32]);
        assert_eq!(calls_to(&e, 0x00ce_5360).len(), 1);
    }

    #[test]
    fn hka_interleaved_uncompressed_animation_cleanup_loaded_object_calls_vtable_slot_zero_with_zero(
    ) {
        let mut e = Engine::new();
        e.register(0x0900_000d, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_000d);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00df_e320, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_000d), vec![vec![object, 0]]);
    }

    #[test]
    fn hka_interleaved_uncompressed_animation_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00df_e340, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_5474);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00df_e340, &args![0u32, 1u32]);
    }
}
