//! `Havok SDK/run_00e00970` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e00c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_8a80`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e00c40(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_8a80, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e00c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to `00e00c40`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e00c00(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e0_0c40, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 00e00c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x14` (`SUB ECX,0x14`) and jumps to `00e00c40`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e00c10(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e0_0c40, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00e00c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00e00c40`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e00c20(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e0_0c40, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00e00c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0xc` (`SUB ECX,0xc`) and jumps to `00e00c40`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e00c30(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e0_0c40, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00e00a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e00a80` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e00a60(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e0_0a80, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e00b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x100-byte stack buffer by calling `00e00ba0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e00b80(e: &mut Engine) -> u32 {
    e.with_stack(0x100, |e, buffer| {
        e.call(0x00e0_0ba0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e00d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x60-byte stack buffer by calling `00d57b20` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e00d70(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, buffer| {
        e.call(0x00d5_7b20, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e00970 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_88b4.
pub fn return_constant_00e00970(_e: &mut Engine) -> u32 {
    0x010e_88b4
}

// Translated from 00e00a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0e18.
pub fn return_constant_00e00a10(_e: &mut Engine) -> u32 {
    0x010d_0e18
}

// Translated from 00e00b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_e4bc.
pub fn return_constant_00e00b30(_e: &mut Engine) -> u32 {
    0x010c_e4bc
}

// Translated from 00e00cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): builds a temporary object in a 0x60-byte frame with
/// the constructor at `00dcd380` (`this` = the frame, one argument 0), does not destroy it and returns
/// the constant 0x010e_8ab0 (the word the constructor stored is not read back).
pub fn fn_00e00cd0(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, record| {
        e.call(0x00dc_d380, &args![record, 0u32]);
    });
    0x010e_8ab0
}

// Translated from 00e00d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_63cc.
pub fn return_constant_00e00d20(_e: &mut Engine) -> u32 {
    0x010c_63cc
}

// Translated from 00e00dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_8d8c.
pub fn return_constant_00e00dc0(_e: &mut Engine) -> u32 {
    0x010e_8d8c
}

// Translated from 00e00a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). It writes the words at `+8`, `+0xc`, `+0x10` and `+0x14` with the
/// base-class values (`010c3b80`, `010c3b98`, `010c3bac`, `010c3b8c`), then the vtable word at
/// `+0` (`010c646c`) and the four words again with the final values (`010c6460`, `010c644c`,
/// `010c6440`, `010c6434`); when `mode` is not zero it sets `+0x20` to 1. Returns `this`
/// (`EAX`). The class is unconfirmed.
pub fn fn_00e00a80(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x8, 0x010c_3b80);
    e.mem.set_u32(base + 0xc, 0x010c_3b98);
    e.mem.set_u32(base + 0x10, 0x010c_3bac);
    e.mem.set_u32(base + 0x14, 0x010c_3b8c);
    e.mem.set_u32(base, 0x010c_646c);
    e.mem.set_u32(base + 0x8, 0x010c_6460);
    e.mem.set_u32(base + 0xc, 0x010c_644c);
    e.mem.set_u32(base + 0x10, 0x010c_6440);
    e.mem.set_u32(base + 0x14, 0x010c_6434);
    if mode != 0 {
        e.mem.set_u32(base + 0x20, 1);
    }
    this
}

// Translated from 00e00ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). It writes the words at `+8`, `+0xc`, `+0x10` and `+0x14` with the
/// base-class values (`010c3b80`, `010c3b98`, `010c3bac`, `010c3b8c`), then the vtable word at
/// `+0` (`010e8a6c`) and the four words again with the final values (`010e8a60`, `010e8a4c`,
/// `010e8a40`, `010e8a34`); when `mode` is not zero it sets `+0x20` to 2. Returns `this`
/// (`EAX`). The class is unconfirmed.
pub fn fn_00e00ba0(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x8, 0x010c_3b80);
    e.mem.set_u32(base + 0xc, 0x010c_3b98);
    e.mem.set_u32(base + 0x10, 0x010c_3bac);
    e.mem.set_u32(base + 0x14, 0x010c_3b8c);
    e.mem.set_u32(base, 0x010e_8a6c);
    e.mem.set_u32(base + 0x8, 0x010e_8a60);
    e.mem.set_u32(base + 0xc, 0x010e_8a4c);
    e.mem.set_u32(base + 0x10, 0x010e_8a40);
    e.mem.set_u32(base + 0x14, 0x010e_8a34);
    if mode != 0 {
        e.mem.set_u32(base + 0x20, 2);
    }
    this
}

// Translated from 00e00c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, runs `00dcd380` on it with the stack
/// word `mode` (the callee pops it), writes the vtable words `010e8ab0` at `+0` and `010e8a90`
/// at `+0x10`, and when `mode` is not zero sets `+0xc` to `0x17`. The function takes two stack
/// words and pops none (`RET`), and returns nothing. The class is unconfirmed.
pub fn fn_00e00c90(e: &mut Engine, object: Ptr, mode: u32) {
    if object.addr() != 0 {
        let base = object.addr();
        e.call(0x00dc_d380, &args![object, mode]);
        e.mem.set_u32(base, 0x010e_8ab0);
        e.mem.set_u32(base + 0x10, 0x010e_8a90);
        if mode != 0 {
            e.mem.set_u32(base + 0xc, 0x17);
        }
    }
}

// Translated from 00e00930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpRemoveTerminalsMoppModifier` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_remove_terminals_mopp_modifier_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpRemoveTerminalsMoppModifier` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores 0x010e_8894 at `+8`, the vtable address 0x010e_88b4 at `+0`, then 0x010e_88a4 at `+8`; the flag (second word) is not read.
pub fn hkp_remove_terminals_mopp_modifier_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        let base = object.addr();
        e.mem.set_u32(base + 8, 0x010e_8894);
        e.mem.set_u32(base, 0x010e_88b4);
        e.mem.set_u32(base + 8, 0x010e_88a4);
    }
}

// Translated from 00e009c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpPlaneShape` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_plane_shape_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e009e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpPlaneShape` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x0103_05a8 at `+0`, then (only with a non-zero flag) 0 at `+0xc`, then the vtable address 0x010d_0e18 at `+0` and 0x1c at `+0xc`.
pub fn hkp_plane_shape_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    let base = object.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if flag != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base, 0x010d_0e18);
    e.mem.set_u32(base + 0xc, 0x1c);
}

// Translated from 00e00a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpNullCollisionFilter` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_null_collision_filter_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpNullCollisionFilter` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, runs `00e00a80` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it); the callee is in this file.
pub fn hkp_null_collision_filter_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        fn_00e00a80(e, object, flag);
    }
}

// Translated from 00e00ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMultiSphereShape` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_multi_sphere_shape_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpMultiSphereShape` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x0103_05a8 at `+0`, then (only with a non-zero flag) 0 at `+0xc`, then the vtable address 0x010c_e4bc at `+0` and 0x15 at `+0xc`.
pub fn hkp_multi_sphere_shape_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    let base = object.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if flag != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base, 0x010c_e4bc);
    e.mem.set_u32(base + 0xc, 0x15);
}

// Translated from 00e00b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpGroupFilter` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_group_filter_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpGroupFilter` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, runs `00e00ba0` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it); the callee is in this file.
pub fn hkp_group_filter_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        fn_00e00ba0(e, object, flag);
    }
}

// Translated from 00e00c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpFastMeshShape` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_fast_mesh_shape_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpDefaultConvexListFilter` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_default_convex_list_filter_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpDefaultConvexListFilter` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010c_63cc at `+0`; the flag (second word) is not read.
pub fn hkp_default_convex_list_filter_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010c_63cc);
    }
}

// Translated from 00e00d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpCylinderShape` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_cylinder_shape_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpCylinderShape` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, calls `00d57b20` on it (thiscall, `this` = `object`, one word = the flag, the callee pops it).
pub fn hkp_cylinder_shape_finish_loaded_object(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00d5_7b20, &args![object, flag]);
    }
}

// Translated from 00e00d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpConvexVerticesConnectivity` (reached only through that record). A cdecl function of one word: the loaded object.
///
/// What it does:
/// calls the object's virtual function at vtable offset 0 with one word, 0 (no null check).
pub fn hkp_convex_vertices_connectivity_cleanup_loaded_object(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB) of the type info record for `hkpConvexVerticesConnectivity` (reached only through that record). A cdecl function of two words: the loaded object and the finish flag the registry passes.
///
/// What it does:
/// when `object` is not null, stores the vtable address 0x010e_8d8c at `+0`; the flag (second word) is not read.
pub fn hkp_convex_vertices_connectivity_finish_loaded_object(
    e: &mut Engine,
    object: Ptr,
    _unused_1: u32,
) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_8d8c);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_0a60, fn_00e00a60() -> u32),
        entry!(0x00e0_0b80, fn_00e00b80() -> u32),
        entry!(0x00e0_0d70, fn_00e00d70() -> u32),
        entry!(0x00e0_0c40, deleting_destructor_00e00c40(Ptr, u8) -> Ptr),
        entry!(0x00e0_0c00, fn_00e00c00(Ptr, u32) -> u32),
        entry!(0x00e0_0c10, fn_00e00c10(Ptr, u32) -> u32),
        entry!(0x00e0_0c20, fn_00e00c20(Ptr, u32) -> u32),
        entry!(0x00e0_0c30, fn_00e00c30(Ptr, u32) -> u32),
        entry!(0x00e00970, return_constant_00e00970() -> u32),
        entry!(0x00e00a10, return_constant_00e00a10() -> u32),
        entry!(0x00e00b30, return_constant_00e00b30() -> u32),
        entry!(0x00e00cd0, fn_00e00cd0() -> u32),
        entry!(0x00e00d20, return_constant_00e00d20() -> u32),
        entry!(0x00e00dc0, return_constant_00e00dc0() -> u32),
        entry!(0x00e0_0a80, fn_00e00a80(Ptr, u32) -> Ptr),
        entry!(0x00e0_0ba0, fn_00e00ba0(Ptr, u32) -> Ptr),
        entry!(0x00e0_0c90, fn_00e00c90(Ptr, u32)),
        entry!(
            0x00e0_0930,
            hkp_remove_terminals_mopp_modifier_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00e0_0950,
            hkp_remove_terminals_mopp_modifier_finish_loaded_object(Ptr, u32)
        ),
        entry!(0x00e0_09c0, hkp_plane_shape_cleanup_loaded_object(Ptr)),
        entry!(0x00e0_09e0, hkp_plane_shape_finish_loaded_object(Ptr, u32)),
        entry!(
            0x00e0_0a20,
            hkp_null_collision_filter_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00e0_0a40,
            hkp_null_collision_filter_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00e0_0ae0,
            hkp_multi_sphere_shape_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00e0_0b00,
            hkp_multi_sphere_shape_finish_loaded_object(Ptr, u32)
        ),
        entry!(0x00e0_0b40, hkp_group_filter_cleanup_loaded_object(Ptr)),
        entry!(0x00e0_0b60, hkp_group_filter_finish_loaded_object(Ptr, u32)),
        entry!(0x00e0_0c70, hkp_fast_mesh_shape_cleanup_loaded_object(Ptr)),
        entry!(
            0x00e0_0cf0,
            hkp_default_convex_list_filter_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00e0_0d10,
            hkp_default_convex_list_filter_finish_loaded_object(Ptr, u32)
        ),
        entry!(0x00e0_0d30, hkp_cylinder_shape_cleanup_loaded_object(Ptr)),
        entry!(
            0x00e0_0d50,
            hkp_cylinder_shape_finish_loaded_object(Ptr, u32)
        ),
        entry!(
            0x00e0_0d90,
            hkp_convex_vertices_connectivity_cleanup_loaded_object(Ptr)
        ),
        entry!(
            0x00e0_0db0,
            hkp_convex_vertices_connectivity_finish_loaded_object(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00e00c40_frees_only_when_bit_zero_is_set() {
        check(0x00e0_0c40, 0x00e4_8a80);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00e00c00() {
        assert_eq!(check(0x00e0_0c00, 0x00e0_0c40, 0x10), 0xdead_beef);
    }

    #[test]
    fn thunk_00e00c10() {
        assert_eq!(check(0x00e0_0c10, 0x00e0_0c40, 0x14), 0xdead_beef);
    }

    #[test]
    fn thunk_00e00c20() {
        assert_eq!(check(0x00e0_0c20, 0x00e0_0c40, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00e00c30() {
        assert_eq!(check(0x00e0_0c30, 0x00e0_0c40, 0xc), 0xdead_beef);
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
        fn_00e00a60_returns_first_word_of_built_object,
        0x00e0_0a60,
        0x00e0_0a80
    );
    first_word_test!(
        fn_00e00b80_returns_first_word_of_built_object,
        0x00e0_0b80,
        0x00e0_0ba0
    );
    first_word_test!(
        fn_00e00d70_returns_first_word_of_built_object,
        0x00e0_0d70,
        0x00d5_7b20
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
    fn return_constant_00e00970_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0970, &args![]).u32(), 0x010e_88b4);
    }

    #[test]
    fn return_constant_00e00a10_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0a10, &args![]).u32(), 0x010d_0e18);
    }

    #[test]
    fn fn_00e00a60_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_0a80, 0x0a0b_0c0d);
        let result = e.call(0x00e0_0a60, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e00b30_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0b30, &args![]).u32(), 0x010c_e4bc);
    }

    #[test]
    fn fn_00e00b80_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_0ba0, 0x0a0b_0c0d);
        let result = e.call(0x00e0_0b80, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00e00cd0_constructs_a_temporary_and_returns_the_constant() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00dc_d380, 0x0a0b_0c0d);
        let result = e.call(0x00e0_0cd0, &args![]).u32();
        assert_eq!(result, 0x010e_8ab0);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e00d20_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0d20, &args![]).u32(), 0x010c_63cc);
    }

    #[test]
    fn fn_00e00d70_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d5_7b20, 0x0a0b_0c0d);
        let result = e.call(0x00e0_0d70, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e00dc0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0dc0, &args![]).u32(), 0x010e_8d8c);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// Runs the constructor with mode 0 and mode 5 on blocks of 0x55 bytes; the final words must
    /// be `words` (at +0, +8, +0xc, +0x10, +0x14) and +0x20 is `tag` only with a mode.
    fn check(address: u32, words: [u32; 5], tag: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.mem.write(this, &[0x55; 0x30]);
        assert_eq!(e.call(address, &args![this, 0u32]).u32(), this);
        for (offset, word) in [0u32, 8, 0xc, 0x10, 0x14].into_iter().zip(words) {
            assert_eq!(e.mem.u32(this + offset), word);
        }
        assert_eq!(e.mem.u32(this + 4), 0x5555_5555);
        assert_eq!(e.mem.u32(this + 0x20), 0x5555_5555);
        e.call(address, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0x20), tag);
        assert_eq!(e.mem.u32(this + 0x24), 0x5555_5555);
    }

    #[test]
    fn fn_00e00a80_writes_the_final_vtable_words() {
        check(
            0x00e0_0a80,
            [
                0x010c_646c,
                0x010c_6460,
                0x010c_644c,
                0x010c_6440,
                0x010c_6434,
            ],
            1,
        );
    }

    #[test]
    fn fn_00e00ba0_writes_the_final_vtable_words() {
        check(
            0x00e0_0ba0,
            [
                0x010e_8a6c,
                0x010e_8a60,
                0x010e_8a4c,
                0x010e_8a40,
                0x010e_8a34,
            ],
            2,
        );
    }

    /// A null object does nothing; otherwise the base function `00dcd380` runs first with the
    /// mode word, then the vtable words are written, and +0xc only with a mode.
    #[test]
    fn fn_00e00c90_calls_the_base_then_writes_its_words() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.mem.write(object, &[0x66; 0x20]);
        e.register_double(0x00dc_d380, move |e, a| {
            assert_eq!(a, [object, 0]);
            assert_eq!(e.mem.u32(object), 0x6666_6666);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e0_0c90, &args![0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call(0x00e0_0c90, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010e_8ab0);
        assert_eq!(e.mem.u32(object + 0x10), 0x010e_8a90);
        assert_eq!(e.mem.u32(object + 0xc), 0x6666_6666);
        let mut e = Engine::new();
        e.register(0x00dc_d380, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.call(0x00e0_0c90, &args![object, 3u32]);
        assert_eq!(e.mem.u32(object + 0xc), 0x17);
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
    fn hkp_remove_terminals_mopp_modifier_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0016, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0016);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00e0_0930, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0016), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_remove_terminals_mopp_modifier_finish_loaded_object_stores_the_vtable_and_the_final_word(
    ) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00e0_0950, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_88b4);
        assert_eq!(e.mem.u32(object + 8), 0x010e_88a4);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00e0_0950, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_plane_shape_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0017, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0017);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00e0_09c0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0017), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_plane_shape_finish_loaded_object_stores_the_vtable_and_the_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, 0x77);
        e.call(0x00e0_09e0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010d_0e18);
        assert_eq!(e.mem.u32(object + 0xc), 0x1c);
        e.call(0x00e0_09e0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010d_0e18);
        assert_eq!(e.mem.u32(object + 0xc), 0x1c);
        // A null object is left alone.
        e.call(0x00e0_09e0, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_null_collision_filter_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0018, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0018);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00e0_0a20, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0018), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_null_collision_filter_finish_loaded_object_runs_the_constructor_chain_on_a_non_null_object(
    ) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00e0_0a40, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_646c);
        assert_eq!(e.mem.u32(object + 0x20), 1);
        // A null object is left alone (nothing is written or read).
        e.call(0x00e0_0a40, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_multi_sphere_shape_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_0019, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0019);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00e0_0ae0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_0019), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_multi_sphere_shape_finish_loaded_object_stores_the_vtable_and_the_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, 0x77);
        e.call(0x00e0_0b00, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010c_e4bc);
        assert_eq!(e.mem.u32(object + 0xc), 0x15);
        e.call(0x00e0_0b00, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_e4bc);
        assert_eq!(e.mem.u32(object + 0xc), 0x15);
        // A null object is left alone.
        e.call(0x00e0_0b00, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_group_filter_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_001a, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_001a);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00e0_0b40, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_001a), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_group_filter_finish_loaded_object_runs_the_constructor_chain_on_a_non_null_object() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00e0_0b60, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_8a6c);
        assert_eq!(e.mem.u32(object + 0x20), 2);
        // A null object is left alone (nothing is written or read).
        e.call(0x00e0_0b60, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_fast_mesh_shape_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_001b, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_001b);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00e0_0c70, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_001b), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_default_convex_list_filter_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_001c, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_001c);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00e0_0cf0, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_001c), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_default_convex_list_filter_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00e0_0d10, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_63cc);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00e0_0d10, &args![0u32, 1u32]);
    }

    #[test]
    fn hkp_cylinder_shape_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_001d, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_001d);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00e0_0d30, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_001d), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_cylinder_shape_finish_loaded_object_calls_the_helper_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00d5_7b20, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00e0_0d50, &args![0x5000u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00d5_7b20), vec![vec![0x5000, 7]]);
        e.call(0x00e0_0d50, &args![0u32, 7u32]);
        assert_eq!(calls_to(&e, 0x00d5_7b20).len(), 1);
    }

    #[test]
    fn hkp_convex_vertices_connectivity_cleanup_loaded_object_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.register(0x0900_001e, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_001e);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.call_log = Some(Vec::new());
        e.call(0x00e0_0d90, &args![object]);
        assert_eq!(calls_to(&e, 0x0900_001e), vec![vec![object, 0]]);
    }

    #[test]
    fn hkp_convex_vertices_connectivity_finish_loaded_object_stores_the_vtable() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00e0_0db0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_8d8c);
        assert_eq!(e.mem.u32(object + 4), 0);
        // A null object is left alone.
        e.call(0x00e0_0db0, &args![0u32, 1u32]);
    }
}
