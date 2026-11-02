//! `Havok SDK/run_00dfd200` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::release_array;

// Translated from 00dfd200 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00cf56f0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfd200(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00cf_56f0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfd280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00d59830` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfd280(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00d5_9830, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfd320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0xf0-byte stack buffer by calling `00e3dbd0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfd320(e: &mut Engine) -> u32 {
    e.with_stack(0xf0, |e, buffer| {
        e.call(0x00e3_dbd0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfd630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00e3e380` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfd630(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00e3_e380, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfd800 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x5c-byte frame stores the vtable word 0x010e_311c at
/// +0xC, runs the constructor at `00cb0a70` (argument 0) on the members at frame+0x20, 0x4c in that
/// order and returns the vtable word. Nothing is destroyed.
pub fn fn_00dfd800(e: &mut Engine) -> u32 {
    e.with_stack(0x5c, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base + 0xc, 0x010e_311c);
        e.call(0x00cb_0a70, &args![base + 0x20, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x4c, 0u32]);
        e.mem.u32(base + 0xc)
    })
}

// Translated from 00dfd380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: releases four `hkArray`-shaped triples (data, size, capacity and
/// flags) of 2-byte elements, at `+0x34`, `+0x28`, `+0x1c` and `+0x10` of `this` (`ECX`), in that
/// order. Each one: the size is cleared, the data is returned through the allocator's slot
/// `0x10` with `capacity * 2` bytes unless the capacity word is negative, then data is cleared
/// and the capacity word becomes `0x80000000`. The class is unconfirmed.
pub fn fn_00dfd380(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_array(e, base + 0x34, 2);
    release_array(e, base + 0x28, 2);
    release_array(e, base + 0x1c, 2);
    release_array(e, base + 0x10, 2);
}

// Translated from 00dfd440 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: releases three `hkArray`-shaped triples of `this` (`ECX`) the way
/// `00dfd380` does: at `+0x28` (2-byte elements), `+0x1c` (1-byte elements) and `+0x10` (2-byte
/// elements), in that order. The class is unconfirmed.
pub fn fn_00dfd440(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_array(e, base + 0x28, 2);
    release_array(e, base + 0x1c, 1);
    release_array(e, base + 0x10, 2);
}

// Translated from 00dfd680 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: releases five `hkArray`-shaped triples of `this` (`ECX`) the way
/// `00dfd380` does, in this order: `+0x30` (1-byte elements), `+0x24` (2 bytes), `+0x18`
/// (4 bytes), `+0x0c` (4 bytes) and `+0` (16 bytes). The class is unconfirmed.
pub fn fn_00dfd680(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_array(e, base + 0x30, 1);
    release_array(e, base + 0x24, 2);
    release_array(e, base + 0x18, 4);
    release_array(e, base + 0x0c, 4);
    release_array(e, base, 16);
}

// Translated from 00dfd7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e311c` and
/// runs `00cb0a70` (an `hkStringPtr` function, one stack word `extra`) on `object + 0x14` and
/// then on `object + 0x40`. The function takes two stack words and pops none (`RET`), and
/// returns nothing. The class is unconfirmed.
pub fn fn_00dfd7d0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_311c);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.call(0x00cb_0a70, &args![object.addr() + 0x40, extra]);
    }
}

// Translated from 00dfd1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpListShapeChildInfo` (the record at `011bb49c` of the data table, name pointer `010e2430`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00dfd1a0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00dfd220 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpConvexVerticesShapeFourVectors` (the record at `011bb4dc` of the data table, name pointer `010e2588`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00dfd220(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00dfd340 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpCompressedMeshShapeChunk` (the record at `011bb52c` of the data table, name pointer `010e2948`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00dfd340(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00dfd360 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpCompressedMeshShapeChunk` (the record at `011bb52c` of the data table, name pointer `010e2948`). Loads the object into `ECX` and jumps to `00dfd380` (thiscall, no stack words), so this is that function run on the object.
pub fn fn_00dfd360(e: &mut Engine, object: Ptr) {
    fn_00dfd380(e, object);
}

// Translated from 00dfd2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpCompressedMeshShapeBigTriangle` (the record at `011bb544` of the data table, name pointer `010e2964`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00dfd2b0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00dfd350 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpCompressedMeshShapeConvexPiece` (the record at `011bb55c` of the data table, name pointer `010e2988`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00dfd350(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00dfd370 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpCompressedMeshShapeConvexPiece` (the record at `011bb55c` of the data table, name pointer `010e2988`). Loads the object into `ECX` and jumps to `00dfd440` (thiscall, no stack words), so this is that function run on the object.
pub fn fn_00dfd370(e: &mut Engine, object: Ptr) {
    fn_00dfd440(e, object);
}

// Translated from 00dfd1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpListShape` (the record at `0126c140`, name pointer `010e2448`, that the static initializer `00fc2fd0` fills). When `object` is not null, runs `00cf56f0` (thiscall, one stack word, `RET 4`) with `this` = the object and the second stack word `finishing`; a null object does nothing.
pub fn fn_00dfd1e0(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00cf_56f0, &args![object.addr(), finishing]);
    }
}

// Translated from 00dfd1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpListShape` (the record at `0126c140`, name pointer `010e2448`, that the static initializer `00fc2fd0` fills). Calls the object's virtual function in vtable slot `0` (`this` = the object) with the one stack word `0`. There is no null test: the object's vtable is read at once. The slot has the shape of a scalar deleting destructor called with flag 0 (unconfirmed).
pub fn fn_00dfd1c0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfd260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpConvexVerticesShape` (the record at `0126c1b8`, name pointer `010e25ac`, that the static initializer `00fc3070` fills). When `object` is not null, runs `00d59830` (thiscall, one stack word, `RET 4`) with `this` = the object and the second stack word `finishing`; a null object does nothing.
pub fn fn_00dfd260(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00d5_9830, &args![object.addr(), finishing]);
    }
}

// Translated from 00dfd240 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpConvexVerticesShape` (the record at `0126c1b8`, name pointer `010e25ac`, that the static initializer `00fc3070` fills). Calls the object's virtual function in vtable slot `0` (`this` = the object) with the one stack word `0`. There is no null test: the object's vtable is read at once. The slot has the shape of a scalar deleting destructor called with flag 0 (unconfirmed).
pub fn fn_00dfd240(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfd300 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpCompressedMeshShape` (the record at `0126c290`, name pointer `010e2a20`, that the static initializer `00fc31a0` fills). When `object` is not null, runs `00e3dbd0` (thiscall, one stack word, `RET 4`) with `this` = the object and the second stack word `finishing`; a null object does nothing.
pub fn fn_00dfd300(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00e3_dbd0, &args![object.addr(), finishing]);
    }
}

// Translated from 00dfd2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpCompressedMeshShape` (the record at `0126c290`, name pointer `010e2a20`, that the static initializer `00fc31a0` fills). Calls the object's virtual function in vtable slot `0` (`this` = the object) with the one stack word `0`. There is no null test: the object's vtable is read at once. The slot has the shape of a scalar deleting destructor called with flag 0 (unconfirmed).
pub fn fn_00dfd2e0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfd4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpCdBody` (the record at `011bb82c` of the data table, name pointer `010e2aa8`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00dfd4d0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00dfd510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpBroadPhaseHandle` (the record at `011bb85c` of the data table, name pointer `010e2acc`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00dfd510(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00dfd4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpBroadPhaseHandle` (the record at `011bb85c` of the data table, name pointer `010e2acc`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00dfd4f0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00dfd520 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpAgent1nSector` (the record at `011bb898` of the data table, name pointer `010e2b80`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00dfd520(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00dfd530 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpAgent1nSector` (the record at `011bb898` of the data table, name pointer `010e2b80`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00dfd530(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00dfd550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkxVertexDescriptionElementDecl` (the record at `011bb8d8` of the data table, name pointer `010e2dc4`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00dfd550(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00dfd570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkxVertexDescription` (the record at `011bb8f0` of the data table, name pointer `010e2de4`). When `object` is not null, runs `00e3df50` (thiscall, one stack word, `RET 4`) with `this` = the object and the second stack word `finishing`; a null object does nothing.
pub fn fn_00dfd570(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00e3_df50, &args![object.addr(), finishing]);
    }
}

// Translated from 00dfd5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkxVertexDescription` (the record at `011bb8f0` of the data table, name pointer `010e2de4`). Releases the `hkArray`-shaped triple (data, size, capacity word) at the start of the object, 12-byte elements: the size is cleared, the data goes back through the memory router's allocator (slot `0x10`, `capacity * 12` bytes) unless the capacity word is negative, then data is cleared and the capacity word becomes `0x80000000`.
pub fn fn_00dfd5a0(e: &mut Engine, object: Ptr) {
    release_array(e, object.addr(), 12);
}

// Translated from 00dfd600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkxVertexBuffer` (the record at `0126c3f8`, name pointer `010e2fec`, that the static initializer `00fc3350` fills). When `object` is not null, runs `00e3e380` (thiscall, one stack word, `RET 4`) with `this` = the object and the second stack word `finishing`; a null object does nothing.
pub fn fn_00dfd600(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00e3_e380, &args![object.addr(), finishing]);
    }
}

// Translated from 00dfd620 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkxVertexBuffer` (the record at `0126c3f8`, name pointer `010e2fec`, that the static initializer `00fc3350` fills). Calls the object's virtual function in vtable slot `0` (`this` = the object) with the one stack word `0`. There is no null test: the object's vtable is read at once. The slot has the shape of a scalar deleting destructor called with flag 0 (unconfirmed).
pub fn fn_00dfd620(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfd660 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkxVertexBufferVertexData` (the record at `011bb9bc` of the data table, name pointer `010e2fd0`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00dfd660(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00dfd670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkxVertexBufferVertexData` (the record at `011bb9bc` of the data table, name pointer `010e2fd0`). Loads the object into `ECX` and jumps to `00dfd680` (thiscall, no stack words), so this is that function run on the object.
pub fn fn_00dfd670(e: &mut Engine, object: Ptr) {
    fn_00dfd680(e, object);
}

// Translated from 00dfd770 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkxNodeAnnotationData` (the record at `011bba5c` of the data table, name pointer `010e3128`). When `object` is not null, runs `00cb0a70` (an `hkStringPtr` function, thiscall, one stack word, `RET 4`) on `object + 4` with the second stack word `finishing`; a null object does nothing.
pub fn fn_00dfd770(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00cb_0a70, &args![object.addr() + 4, finishing]);
    }
}

// Translated from 00dfd7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkxNodeAnnotationData` (the record at `011bba5c` of the data table, name pointer `010e3128`). Adds 4 to the object and jumps to `00cb0c00` (`hkStringPtr::~hkStringPtr` in the engine map; thiscall, no stack words), so that function runs on `object + 4`.
pub fn fn_00dfd7c0(e: &mut Engine, object: Ptr) {
    e.call(0x00cb_0c00, &args![object.addr().wrapping_add(4)]);
}

// Translated from 00dfd7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkxNode` (the record at `0126c470`, name pointer `010e3140`, that the static initializer `00fc33f0` fills). Calls the object's virtual function in vtable slot `0` (`this` = the object) with the one stack word `0`. There is no null test: the object's vtable is read at once. The slot has the shape of a scalar deleting destructor called with flag 0 (unconfirmed).
pub fn fn_00dfd7a0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_d200, fn_00dfd200() -> u32),
        entry!(0x00df_d280, fn_00dfd280() -> u32),
        entry!(0x00df_d320, fn_00dfd320() -> u32),
        entry!(0x00df_d630, fn_00dfd630() -> u32),
        entry!(0x00dfd800, fn_00dfd800() -> u32),
        entry!(0x00df_d380, fn_00dfd380(Ptr)),
        entry!(0x00df_d440, fn_00dfd440(Ptr)),
        entry!(0x00df_d680, fn_00dfd680(Ptr)),
        entry!(0x00df_d7d0, fn_00dfd7d0(Ptr, u32)),
        entry!(0x00df_d1a0, fn_00dfd1a0(Ptr)),
        entry!(0x00df_d220, fn_00dfd220(Ptr)),
        entry!(0x00df_d340, fn_00dfd340(Ptr, u32)),
        entry!(0x00df_d360, fn_00dfd360(Ptr)),
        entry!(0x00df_d2b0, fn_00dfd2b0(Ptr)),
        entry!(0x00df_d350, fn_00dfd350(Ptr, u32)),
        entry!(0x00df_d370, fn_00dfd370(Ptr)),
        entry!(0x00df_d1e0, fn_00dfd1e0(Ptr, u32)),
        entry!(0x00df_d1c0, fn_00dfd1c0(Ptr)),
        entry!(0x00df_d260, fn_00dfd260(Ptr, u32)),
        entry!(0x00df_d240, fn_00dfd240(Ptr)),
        entry!(0x00df_d300, fn_00dfd300(Ptr, u32)),
        entry!(0x00df_d2e0, fn_00dfd2e0(Ptr)),
        entry!(0x00df_d4d0, fn_00dfd4d0(Ptr)),
        entry!(0x00df_d510, fn_00dfd510(Ptr, u32)),
        entry!(0x00df_d4f0, fn_00dfd4f0(Ptr)),
        entry!(0x00df_d520, fn_00dfd520(Ptr, u32)),
        entry!(0x00df_d530, fn_00dfd530(Ptr)),
        entry!(0x00df_d550, fn_00dfd550(Ptr)),
        entry!(0x00df_d570, fn_00dfd570(Ptr, u32)),
        entry!(0x00df_d5a0, fn_00dfd5a0(Ptr)),
        entry!(0x00df_d600, fn_00dfd600(Ptr, u32)),
        entry!(0x00df_d620, fn_00dfd620(Ptr)),
        entry!(0x00df_d660, fn_00dfd660(Ptr, u32)),
        entry!(0x00df_d670, fn_00dfd670(Ptr)),
        entry!(0x00df_d770, fn_00dfd770(Ptr, u32)),
        entry!(0x00df_d7c0, fn_00dfd7c0(Ptr)),
        entry!(0x00df_d7a0, fn_00dfd7a0(Ptr)),
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
        fn_00dfd200_returns_first_word_of_built_object,
        0x00df_d200,
        0x00cf_56f0
    );
    first_word_test!(
        fn_00dfd280_returns_first_word_of_built_object,
        0x00df_d280,
        0x00d5_9830
    );
    first_word_test!(
        fn_00dfd320_returns_first_word_of_built_object,
        0x00df_d320,
        0x00e3_dbd0
    );
    first_word_test!(
        fn_00dfd630_returns_first_word_of_built_object,
        0x00df_d630,
        0x00e3_e380
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
    fn fn_00dfd200_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00cf_56f0, 0x0a0b_0c0d);
        let result = e.call(0x00df_d200, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfd280_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d5_9830, 0x0a0b_0c0d);
        let result = e.call(0x00df_d280, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfd320_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_dbd0, 0x0a0b_0c0d);
        let result = e.call(0x00df_d320, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfd630_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_e380, 0x0a0b_0c0d);
        let result = e.call(0x00df_d630, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfd800_stores_the_vtable_and_constructs_its_members() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14, 0x40], false);
        let result = e.call(0x00df_d800, &args![]).u32();
        assert_eq!(result, 0x010e_311c);
        let calls = log.borrow();
        assert_eq!(calls.len(), 2);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_311c);
        }
        assert_eq!(calls[1].0 - calls[0].0, 0x2c);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A memory router whose allocator's slot 0x10 is a double that logs its words.
    fn engine_with_allocator() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (e, log, allocator)
    }

    /// Sets the array triple at `array` to `data`, size 3 and the capacity word `capacity`.
    fn set_array(e: &mut Engine, array: u32, data: u32, capacity: u32) {
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, capacity);
    }

    fn assert_cleared(e: &Engine, array: u32) {
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }

    /// Four owned arrays of 2-byte elements are freed in the order +0x34, +0x28, +0x1c, +0x10.
    #[test]
    fn fn_00dfd380_releases_four_arrays_of_words() {
        let (mut e, log, allocator) = engine_with_allocator();
        let this = e.mem.alloc(0x50);
        set_array(&mut e, this + 0x34, 0xa1, 5);
        set_array(&mut e, this + 0x28, 0xa2, 6);
        set_array(&mut e, this + 0x1c, 0xa3, 0x8000_0007);
        set_array(&mut e, this + 0x10, 0xa4, 0x4000_0008);
        e.call(0x00df_d380, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![allocator, 0xa1, 10],
                vec![allocator, 0xa2, 12],
                vec![allocator, 0xa4, 16]
            ]
        );
        for offset in [0x34, 0x28, 0x1c, 0x10] {
            assert_cleared(&e, this + offset);
        }
    }

    /// The middle array holds single bytes.
    #[test]
    fn fn_00dfd440_releases_three_arrays() {
        let (mut e, log, allocator) = engine_with_allocator();
        let this = e.mem.alloc(0x50);
        set_array(&mut e, this + 0x28, 0xb1, 5);
        set_array(&mut e, this + 0x1c, 0xb2, 6);
        set_array(&mut e, this + 0x10, 0xb3, 7);
        e.call(0x00df_d440, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![allocator, 0xb1, 10],
                vec![allocator, 0xb2, 6],
                vec![allocator, 0xb3, 14]
            ]
        );
        for offset in [0x28, 0x1c, 0x10] {
            assert_cleared(&e, this + offset);
        }
    }

    /// Five arrays with element sizes 1, 2, 4, 4 and 16; the one at +0x24 is not owned.
    #[test]
    fn fn_00dfd680_releases_five_arrays() {
        let (mut e, log, allocator) = engine_with_allocator();
        let this = e.mem.alloc(0x50);
        set_array(&mut e, this + 0x30, 0xc1, 3);
        set_array(&mut e, this + 0x24, 0xc2, 0x8000_0004);
        set_array(&mut e, this + 0x18, 0xc3, 5);
        set_array(&mut e, this + 0x0c, 0xc4, 6);
        set_array(&mut e, this, 0xc5, 7);
        e.call(0x00df_d680, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![allocator, 0xc1, 3],
                vec![allocator, 0xc3, 20],
                vec![allocator, 0xc4, 24],
                vec![allocator, 0xc5, 112]
            ]
        );
        for offset in [0x30, 0x24, 0x18, 0x0c, 0] {
            assert_cleared(&e, this + offset);
        }
    }

    /// A null object does nothing; otherwise the string function runs on both members.
    #[test]
    fn fn_00dfd7d0_runs_the_string_function_twice() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let string_log = Rc::clone(&log);
        e.register_double(0x00cb_0a70, move |_, a| {
            string_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00df_d7d0, &args![0u32, 4u32]);
        assert!(log.borrow().is_empty());
        let object = e.mem.alloc(0x60);
        e.call(0x00df_d7d0, &args![object, 4u32]);
        assert_eq!(e.mem.u32(object), 0x010e_311c);
        assert_eq!(
            *log.borrow(),
            vec![vec![object + 0x14, 4], vec![object + 0x40, 4]]
        );
    }
}

#[cfg(test)]
mod tests_a03_type_info {
    use crate::prelude::*;

    /// Calls `function` with an object holding a pattern (and, for two words, a finishing word):
    /// nothing else is called and the object is unchanged.
    fn check_nothing(function: u32, words: usize) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_u32(object + 4 * i, 0x1000_0000 + i);
        }
        e.call_log = Some(vec![]);
        if words == 2 {
            e.call(function, &args![object, 1u32]);
        } else {
            e.call(function, &args![object]);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        for i in 0..16 {
            assert_eq!(e.mem.u32(object + 4 * i), 0x1000_0000 + i);
        }
    }

    macro_rules! nothing_test {
        ($name:ident, $function:expr, $words:expr) => {
            #[test]
            fn $name() {
                check_nothing($function, $words);
            }
        };
    }

    const SLOT_ZERO_TARGET: u32 = 0x0900_0021;

    /// The object's vtable slot 0 is a double: it must run once with `this` = the object and the
    /// word 0.
    fn check_virtual_call(function: u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable, SLOT_ZERO_TARGET);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.register(SLOT_ZERO_TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(function, &args![object]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == SLOT_ZERO_TARGET).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![object, 0]);
    }

    macro_rules! virtual_call_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_virtual_call($function);
            }
        };
    }

    /// A non-null object: the callee double runs once with `this` = the object and the finishing
    /// word; a null object calls nothing.
    fn check_calls(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(function, &args![0u32, 0x55u32]);
        e.call(function, &args![object, 0x66u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![object, 0x66]);
    }

    macro_rules! calls_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check_calls($function, $callee);
            }
        };
    }

    const FREE_SLOT_TARGET: u32 = 0x0900_0031;

    /// A memory router (`00c85750`) whose allocator has a double in slot 0x10; returns the engine
    /// and the allocator object.
    fn engine_with_allocator() -> (Engine, u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_TARGET);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(FREE_SLOT_TARGET, |_, _| Ret::default());
        (e, allocator)
    }

    /// Every array of the jump target (offset, element size) is owned, with its own data word and
    /// capacity: each is released in order, `capacity * element size` bytes, and cleared.
    fn check_jump(function: u32, arrays: &[(u32, u32)]) {
        let (mut e, allocator) = engine_with_allocator();
        let object = e.mem.alloc(0x50);
        let mut expected = Vec::new();
        for (i, (offset, size)) in arrays.iter().enumerate() {
            let data = 0xe0 + i as u32;
            let capacity = 3 + i as u32;
            e.mem.set_u32(object + offset, data);
            e.mem.set_u32(object + offset + 4, 2);
            e.mem.set_u32(object + offset + 8, capacity);
            expected.push(vec![allocator, data, capacity * size]);
        }
        e.call_log = Some(vec![]);
        e.call(function, &args![object]);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == FREE_SLOT_TARGET)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(frees, expected);
        for (offset, _) in arrays {
            assert_eq!(e.mem.u32(object + offset), 0);
            assert_eq!(e.mem.u32(object + offset + 4), 0);
            assert_eq!(e.mem.u32(object + offset + 8), 0x8000_0000);
        }
    }

    macro_rules! jump_test {
        ($name:ident, $function:expr, $arrays:expr) => {
            #[test]
            fn $name() {
                check_jump($function, $arrays);
            }
        };
    }

    /// `00cb0a70` runs on `object + 4` with the finishing word; a null object calls nothing.
    fn check_annotation_finish(function: u32) {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(function, &args![0u32, 3u32]);
        e.call(function, &args![object, 3u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00cb_0a70).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![object + 4, 3]);
    }

    macro_rules! annotation_finish_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_annotation_finish($function);
            }
        };
    }

    /// `00cb0c00` runs on `object + 4`.
    fn check_annotation_cleanup(function: u32) {
        let mut e = Engine::new();
        e.register(0x00cb_0c00, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(function, &args![object]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00cb_0c00).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![object + 4]);
    }

    macro_rules! annotation_cleanup_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_annotation_cleanup($function);
            }
        };
    }

    /// The array at the start of the object is released with 12-byte elements, only when it owns
    /// its storage.
    fn check_array(function: u32) {
        let (mut e, allocator) = engine_with_allocator();
        let owned = e.mem.alloc(0x10);
        e.mem.set_u32(owned, 0xd1);
        e.mem.set_u32(owned + 4, 3);
        e.mem.set_u32(owned + 8, 5);
        let borrowed = e.mem.alloc(0x10);
        e.mem.set_u32(borrowed, 0xd2);
        e.mem.set_u32(borrowed + 4, 3);
        e.mem.set_u32(borrowed + 8, 0x8000_0005);
        e.call_log = Some(vec![]);
        e.call(function, &args![owned]);
        e.call(function, &args![borrowed]);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|(a, _)| *a == FREE_SLOT_TARGET).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, vec![allocator, 0xd1, 60]);
        for array in [owned, borrowed] {
            assert_eq!(e.mem.u32(array), 0);
            assert_eq!(e.mem.u32(array + 4), 0);
            assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        }
    }

    macro_rules! array_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_array($function);
            }
        };
    }

    nothing_test!(fn_00dfd1a0_does_nothing, 0x00df_d1a0, 1);
    nothing_test!(fn_00dfd220_does_nothing, 0x00df_d220, 1);
    nothing_test!(fn_00dfd340_does_nothing, 0x00df_d340, 2);
    jump_test!(
        fn_00dfd360_releases_the_arrays_of_its_target,
        0x00df_d360,
        &[(0x34, 2), (0x28, 2), (0x1c, 2), (0x10, 2)]
    );
    nothing_test!(fn_00dfd2b0_does_nothing, 0x00df_d2b0, 1);
    nothing_test!(fn_00dfd350_does_nothing, 0x00df_d350, 2);
    jump_test!(
        fn_00dfd370_releases_the_arrays_of_its_target,
        0x00df_d370,
        &[(0x28, 2), (0x1c, 1), (0x10, 2)]
    );
    calls_test!(
        fn_00dfd1e0_runs_its_callee_on_a_non_null_object,
        0x00df_d1e0,
        0x00cf_56f0
    );
    virtual_call_test!(fn_00dfd1c0_calls_vtable_slot_0_with_flag_0, 0x00df_d1c0);
    calls_test!(
        fn_00dfd260_runs_its_callee_on_a_non_null_object,
        0x00df_d260,
        0x00d5_9830
    );
    virtual_call_test!(fn_00dfd240_calls_vtable_slot_0_with_flag_0, 0x00df_d240);
    calls_test!(
        fn_00dfd300_runs_its_callee_on_a_non_null_object,
        0x00df_d300,
        0x00e3_dbd0
    );
    virtual_call_test!(fn_00dfd2e0_calls_vtable_slot_0_with_flag_0, 0x00df_d2e0);
    nothing_test!(fn_00dfd4d0_does_nothing, 0x00df_d4d0, 1);
    nothing_test!(fn_00dfd510_does_nothing, 0x00df_d510, 2);
    nothing_test!(fn_00dfd4f0_does_nothing, 0x00df_d4f0, 1);
    nothing_test!(fn_00dfd520_does_nothing, 0x00df_d520, 2);
    nothing_test!(fn_00dfd530_does_nothing, 0x00df_d530, 1);
    nothing_test!(fn_00dfd550_does_nothing, 0x00df_d550, 1);
    calls_test!(
        fn_00dfd570_runs_its_callee_on_a_non_null_object,
        0x00df_d570,
        0x00e3_df50
    );
    array_test!(
        fn_00dfd5a0_releases_the_array_of_12_byte_elements,
        0x00df_d5a0
    );
    calls_test!(
        fn_00dfd600_runs_its_callee_on_a_non_null_object,
        0x00df_d600,
        0x00e3_e380
    );
    virtual_call_test!(fn_00dfd620_calls_vtable_slot_0_with_flag_0, 0x00df_d620);
    nothing_test!(fn_00dfd660_does_nothing, 0x00df_d660, 2);
    jump_test!(
        fn_00dfd670_releases_the_arrays_of_its_target,
        0x00df_d670,
        &[(0x30, 1), (0x24, 2), (0x18, 4), (0x0c, 4), (0, 16)]
    );
    annotation_finish_test!(fn_00dfd770_runs_the_string_function_at_plus_4, 0x00df_d770);
    annotation_cleanup_test!(
        fn_00dfd7c0_runs_the_string_destructor_at_plus_4,
        0x00df_d7c0
    );
    virtual_call_test!(fn_00dfd7a0_calls_vtable_slot_0_with_flag_0, 0x00df_d7a0);
}
