//! `NiXenonRenderer/nid3dshaderprogramfactory.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a642a0` (`thiscall`, `RET 8`, no unit): stores a creator under a name in a map.
const MAP_SET_AT: u32 = 0x00a6_42a0;
/// `0126fa14`: the creator map object.
const CREATOR_MAP_SLOT: u32 = 0x0126_fa14;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00ec61c0` (`cdecl`, `memset`: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `00ec7690` (`cdecl`, `strchr`: text, character).
const STRCHR: u32 = 0x00ec_7690;
/// `00ec67aa` (`cdecl`, `tolower`: character).
const TOLOWER: u32 = 0x00ec_67aa;

// Translated from 00e93d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::CreateShaderCreatorMap` (Xbox PDB; no arguments): allocates
/// 0x14 bytes from the memory manager and, when that succeeds, sets the vtable
/// `010f0eb0`, the size 0x25 at `+4`, the word at `+0xc` to 0, a zeroed 0x94-byte slot
/// block at `+8` (`00aa1070`, `memset`), the byte at `+0x10` to 1 and the vtable
/// `010f0f50`, keeping the object in `0126fa14` (null when the allocation fails).
pub fn ni_d3d_shader_program_factory_create_shader_creator_map(e: &mut Engine) {
    let map = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if map == 0 {
        e.mem.set_u32(CREATOR_MAP_SLOT, 0);
        return;
    }
    e.mem.set_u32(map, 0x010f_0eb0);
    e.mem.set_u32(map + 4, 0x25);
    e.mem.set_u32(map + 0xc, 0);
    let slots = e.call(ALLOCATE_BLOCK, &args![0x94u32]).u32();
    let bytes = e.mem.u32(map + 4).wrapping_mul(4);
    e.mem.set_u32(map + 8, slots);
    e.call(MEMSET, &args![slots, 0u32, bytes]);
    e.mem.set_u8(map + 0x10, 1);
    e.mem.set_u32(map, 0x010f_0f50);
    e.mem.set_u32(CREATOR_MAP_SLOT, map);
}

// Translated from 00e93df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderprogramfactory.cpp`; no arguments): when the
/// creator map `0126fa14` is null, makes it with `00e93d80` (a jump in the exe).
pub fn fn_00e93df0(e: &mut Engine) {
    if e.mem.u32(CREATOR_MAP_SLOT) == 0 {
        ni_d3d_shader_program_factory_create_shader_creator_map(e);
    }
}

// Translated from 00e93e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e93e70 - /GS stack cookie check (00ec408c) not modelled
// (the 0x100-byte name buffer is a zeroed heap block)
/// `NiD3DShaderProgramFactory::RegisterShaderCreator` (Xbox PDB; `cdecl`, two words):
/// answers `false` for a null `name` or one containing a '.'; otherwise copies the name
/// lower-cased (`tolower` per character, up to 0x100 characters including the terminator)
/// into a local buffer, makes the creator map when it is missing (`00e93d80`) and stores
/// `creator` in it under the copy (`00a642a0`); answers `true`. A name of 0x100 characters
/// or more leaves the copy without terminator (the exe's buffer is followed by its
/// cookie; here by zero bytes).
pub fn ni_d3d_shader_program_factory_register_shader_creator(
    e: &mut Engine,
    name: Ptr,
    creator: u32,
) -> bool {
    if name.is_null() {
        return false;
    }
    if !e.call(STRCHR, &args![name, 0x2eu32]).ptr::<u8>().is_null() {
        return false;
    }
    e.with_stack(0x104, |e, buffer| {
        let mut index = 0u32;
        let mut source = name.addr();
        loop {
            let character = e.mem.u8(source);
            let lower = e.call(TOLOWER, &args![character as i8 as i32]).u8();
            e.mem.set_u8(buffer.addr() + index, lower);
            if character == 0 {
                break;
            }
            index += 1;
            source = source.wrapping_add(1);
            if index >= 0x100 {
                break;
            }
        }
        if e.mem.u32(CREATOR_MAP_SLOT) == 0 {
            ni_d3d_shader_program_factory_create_shader_creator_map(e);
        }
        let map = e.mem.u32(CREATOR_MAP_SLOT);
        e.call(MAP_SET_AT, &args![map, buffer, creator]);
    });
    true
}

/// `NiD3DShaderProgramFactory`'s single instance (`GetInstance`, `00e92eb0`,
/// returns it).
const FACTORY_INSTANCE: u32 = 0x0126_fa10;
/// The Gamebryo `free` (`run_00aa1010.rs`), cdecl (block).
const NI_FREE: u32 = 0x00aa_10f0;
/// The sized Gamebryo free (`run_00aa1380.rs`), cdecl (block, size).
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// The memory manager's free (`00aa4060`), thiscall on `011f6238` (block).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// `_strcpy_s` (engine map), cdecl (destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// The map's `RemoveAll` (`run_00b601a0.rs`), thiscall.
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
/// The map's `GetNext` (thiscall: position, key, value out words), `RET 0xC`.
const MAP_GET_NEXT: u32 = 0x00b5_7f20;
/// The map's `RemoveAt` (`run_00e91fd0.rs`), thiscall (key), `RET 4`.
const MAP_REMOVE_AT: u32 = 0x00e9_1fd0;
/// The list's `RemoveHead` (`run_00b5fd60.rs`), thiscall.
const LIST_REMOVE_HEAD: u32 = 0x00b6_02d0;
/// The list's `RemoveTail` (`run_00e7cd50.rs`), thiscall.
const LIST_REMOVE_TAIL: u32 = 0x00e7_d5b0;
/// The node allocator's deallocate (`run_0045cb90.rs`), thiscall on the
/// allocator (node), `RET 4`.
const NODE_DEALLOCATE: u32 = 0x0045_cee0;
/// Imported `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Vtables (RTTI) of the vertex shader, pixel shader and creator maps:
/// their `NiTStringTemplateMap`, `NiTPointerMap` and `NiTMapBase` levels.
const VERTEX_TEMPLATE_MAP_VTABLE: u32 = 0x010f_0e30;
const VERTEX_POINTER_MAP_VTABLE: u32 = 0x010f_0e10;
const PIXEL_TEMPLATE_MAP_VTABLE: u32 = 0x010f_0e90;
const PIXEL_POINTER_MAP_VTABLE: u32 = 0x010f_0e70;
const CREATOR_TEMPLATE_MAP_VTABLE: u32 = 0x010f_0ef0;
const CREATOR_POINTER_MAP_VTABLE: u32 = 0x010f_0ed0;
const CREATOR_MAP_BASE_VTABLE: u32 = 0x010f_0eb0;

// Translated from 00ae7f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,char *>::RemoveHead`
/// (engine map): unlinks the head node (the next one becomes the head with
/// no previous link, or the tail is cleared when there is none), takes its
/// element and clears it, gives the node back to the allocator (`0045cee0`
/// on +8) and counts one less. Returns the element.
pub fn ni_t_pointer_list_base_remove_head(
    e: &mut Engine,
    this: Ptr<crate::types::NiTPointerList>,
) -> u32 {
    use crate::types::NiTPointerList;
    let node = e.get(this, NiTPointerList::m_pkHead);
    let head = e.get(this, NiTPointerList::m_pkHead);
    let next = e.mem.u32(head);
    e.set(this, NiTPointerList::m_pkHead, next);
    if e.get(this, NiTPointerList::m_pkHead) != 0 {
        let head = e.get(this, NiTPointerList::m_pkHead);
        e.mem.set_u32(head + 4, 0);
    } else {
        e.set(this, NiTPointerList::m_pkTail, 0);
    }
    let element = e.mem.u32(node + 8);
    e.mem.set_u32(node + 8, 0);
    e.call(NODE_DEALLOCATE, &args![this.addr() + 8, node]);
    let count = e.get(this, NiTPointerList::m_uiCount);
    e.set(this, NiTPointerList::m_uiCount, count.wrapping_sub(1));
    element
}

// Translated from 00e92ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: deletes the factory instance (`0126fa10`,
/// vtable slot 0 with 1) when there is one and clears it.
pub fn fn_00e92ec0(e: &mut Engine) {
    let instance = e.global::<u32>(FACTORY_INSTANCE);
    if instance != 0 {
        e.vcall(instance, 0, &args![1u32]);
    }
    e.set_global(FACTORY_INSTANCE, 0u32);
}

// Translated from 00e93000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::GetNextProgramDirectory` (engine map):
/// with a node at `*position`, moves `*position` to the next node and
/// returns the node's element (the directory); else 0.
pub fn ni_d3d_shader_program_factory_get_next_program_directory(
    e: &mut Engine,
    position: u32,
) -> u32 {
    let node = e.mem.u32(position);
    if node == 0 {
        return 0;
    }
    let next = e.mem.u32(node);
    e.mem.set_u32(position, next);
    e.mem.u32(node + 8)
}

// Translated from 00e93260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): removes every entry of the creator map
/// (`0126fa14`) whose value is `creator`: from the first non-empty bucket,
/// `GetNext` (`00b57f20`) gives each key and value and `RemoveAt`
/// (`00e91fd0`) removes the matching keys.
pub fn fn_00e93260(e: &mut Engine, creator: u32) {
    let map = e.mem.u32(CREATOR_MAP_SLOT);
    if map == 0 {
        return;
    }
    let size = e.mem.u32(map + 4);
    let mut first = 0u32;
    let mut index = 0u32;
    while index < size {
        let buckets = e.mem.u32(map + 8);
        let bucket = e.mem.u32(buckets + 4 * index);
        if bucket != 0 {
            first = bucket;
            break;
        }
        index += 1;
    }
    if first == 0 {
        return;
    }
    // The locals: the position, the key and the value.
    e.with_stack(0xc, |e, locals| {
        let value_slot = locals.addr();
        let key_slot = locals.addr() + 4;
        let position = locals.addr() + 8;
        e.mem.set_u32(position, first);
        loop {
            let map = e.mem.u32(CREATOR_MAP_SLOT);
            e.mem.set_u32(key_slot, 0);
            e.mem.set_u32(value_slot, 0);
            e.call(MAP_GET_NEXT, &args![map, position, key_slot, value_slot]);
            if e.mem.u32(value_slot) == creator {
                let key = e.mem.u32(key_slot);
                let map = e.mem.u32(CREATOR_MAP_SLOT);
                e.call(MAP_REMOVE_AT, &args![map, key]);
            }
            if e.mem.u32(position) == 0 {
                break;
            }
        }
    });
}

// Translated from 00e933d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the creator map (`0126fa14`,
/// `00b601a0`), deletes it (vtable slot 0 with 1) when there is one and
/// clears it.
pub fn fn_00e933d0(e: &mut Engine) {
    let map = e.mem.u32(CREATOR_MAP_SLOT);
    e.call(MAP_REMOVE_ALL, &args![map]);
    let map = e.mem.u32(CREATOR_MAP_SLOT);
    if map != 0 {
        e.vcall(map, 0, &args![1u32]);
    }
    e.mem.set_u32(CREATOR_MAP_SLOT, 0);
}

/// The destructor body the three `NiTStringTemplateMap` instances share:
/// the template vtable; with the copy flag (+0x10) every key string of
/// every bucket's items freed; the pointer-map vtable and `RemoveAll`; the
/// map-base vtable and `RemoveAll` again; the bucket array (+8) freed.
fn string_template_map_destruct(e: &mut Engine, this: Ptr, vtables: [u32; 3]) {
    let base = this.addr();
    e.mem.set_u32(base, vtables[0]);
    if e.mem.u8(base + 0x10) != 0 {
        let mut index = 0u32;
        while index < e.mem.u32(base + 4) {
            let buckets = e.mem.u32(base + 8);
            let mut item = e.mem.u32(buckets + 4 * index);
            while item != 0 {
                let key = e.mem.u32(item + 4);
                item = e.mem.u32(item);
                e.call(NI_FREE, &args![key]);
            }
            index += 1;
        }
    }
    e.mem.set_u32(base, vtables[1]);
    e.call(MAP_REMOVE_ALL, &args![base]);
    e.mem.set_u32(base, vtables[2]);
    e.call(MAP_REMOVE_ALL, &args![base]);
    let buckets = e.mem.u32(base + 8);
    e.call(NI_FREE, &args![buckets]);
}

// Translated from 00e93400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTStringTemplateMap<NiTPointerMap<const char *, NiD3DVertexShader
/// *>, ...>` destructor (RTTI of the vtables it stores): see
/// `string_template_map_destruct`.
pub fn fn_00e93400(e: &mut Engine, this: Ptr) {
    string_template_map_destruct(
        e,
        this,
        [
            VERTEX_TEMPLATE_MAP_VTABLE,
            VERTEX_POINTER_MAP_VTABLE,
            FIRST_MAP_BASE_VTABLE,
        ],
    );
}

// Translated from 00e93470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The string map's `SetValue` (unnamed in the engine map): with the copy
/// flag (+0x10) the item's key (+4) becomes a copy of `key` (`strlen + 1`
/// bytes from `00aa1070`, `_strcpy_s`), else `key` itself; the item's value
/// (+8) becomes `value`.
pub fn fn_00e93470(e: &mut Engine, this: Ptr, item: u32, key: u32, value: u32) {
    if e.mem.u8(this.addr() + 0x10) == 0 {
        e.mem.set_u32(item + 4, key);
        e.mem.set_u32(item + 8, value);
        return;
    }
    let mut length = 0u32;
    while e.mem.u8(key.wrapping_add(length)) != 0 {
        length += 1;
    }
    let size = length + 1;
    let copy = e.call(ALLOCATE_BLOCK, &args![size]).u32();
    e.mem.set_u32(item + 4, copy);
    e.call(STRCPY_S, &args![copy, size, key]);
    e.mem.set_u32(item + 8, value);
}

// Translated from 00e934d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTStringTemplateMap<NiTPointerMap<const char *, NiD3DPixelShader
/// *>, ...>` destructor: see `string_template_map_destruct`.
pub fn fn_00e934d0(e: &mut Engine, this: Ptr) {
    string_template_map_destruct(
        e,
        this,
        [
            PIXEL_TEMPLATE_MAP_VTABLE,
            PIXEL_POINTER_MAP_VTABLE,
            SECOND_MAP_BASE_VTABLE,
        ],
    );
}

// Translated from 00e93540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTStringTemplateMap<NiTPointerMap<const char *,
/// NiD3DShaderProgramCreator *>, ...>` destructor: see
/// `string_template_map_destruct`.
pub fn fn_00e93540(e: &mut Engine, this: Ptr) {
    string_template_map_destruct(
        e,
        this,
        [
            CREATOR_TEMPLATE_MAP_VTABLE,
            CREATOR_POINTER_MAP_VTABLE,
            CREATOR_MAP_BASE_VTABLE,
        ],
    );
}

// Translated from 00e935b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The string map's `ClearValue` (unnamed in the engine map): with the copy
/// flag (+0x10), frees the item's key (+4).
pub fn fn_00e935b0(e: &mut Engine, this: Ptr, item: u32) {
    if e.mem.u8(this.addr() + 0x10) != 0 {
        let key = e.mem.u32(item + 4);
        e.call(NI_FREE, &args![key]);
    }
}

/// A scalar deleting destructor's tail: with bit 0 of `flags` the object
/// freed through the memory manager.
fn free_if_flagged(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

// Translated from 00e93650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<..., NiD3DVertexShader *>::scalar deleting
/// destructor` (engine map): `00e93400`, then the object freed with bit 0
/// of `flags`. Returns `this`.
pub fn fn_00e93650(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e93400(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 00e936f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<..., NiD3DPixelShader *>::scalar deleting
/// destructor` (engine map): `00e934d0`, then the object freed with bit 0
/// of `flags`. Returns `this`.
pub fn fn_00e936f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e934d0(e, this);
    free_if_flagged(e, this, flags)
}

/// `00e93790` and `00e937c0`: with a factory instance and a non-null
/// `shader`, removes the shader's name (its vtable +0x50) from the
/// factory's map at `map_offset` when both exist.
fn remove_shader_name(e: &mut Engine, shader: u32, map_offset: u32) {
    if e.global::<u32>(FACTORY_INSTANCE) == 0 || shader == 0 {
        return;
    }
    let name = e.vcall(shader, 0x50, &args![]).u32();
    if name == 0 {
        return;
    }
    let instance = e.global::<u32>(FACTORY_INSTANCE);
    let map = e.mem.u32(instance + map_offset);
    if map != 0 {
        e.call(MAP_REMOVE_AT, &args![map, name]);
    }
}

// Translated from 00e93790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): removes the vertex shader's name from
/// the factory's vertex shader map (+0x14).
pub fn fn_00e93790(e: &mut Engine, shader: u32) {
    remove_shader_name(e, shader, 0x14);
}

// Translated from 00e937c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): removes the pixel shader's name from
/// the factory's pixel shader map (+0x18).
pub fn fn_00e937c0(e: &mut Engine, shader: u32) {
    remove_shader_name(e, shader, 0x18);
}

// Translated from 00e93ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<..., NiD3DShaderProgramCreator *>::scalar deleting
/// destructor` (engine map): `00e93540`, then the object freed with bit 0
/// of `flags`. Returns `this`.
pub fn fn_00e93ab0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e93540(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 00e93cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::RemoveAllProgramDirectories` (engine map):
/// for each node of the directory list (+8), takes its directory; a non-null
/// one is looked up in the list from the head and its node removed (the
/// head through `RemoveHead`, the tail through `RemoveTail`, any other one
/// unlinked here and pushed back on the shared node pool under its lock,
/// the count dropping by one), then the directory string is freed.
pub fn ni_d3d_shader_program_factory_remove_all_program_directories(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 8;
    let mut node = e.mem.u32(list);
    while node != 0 {
        let directory = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if directory == 0 {
            continue;
        }
        let head = e.mem.u32(list);
        let mut cursor = head;
        let mut found = 0u32;
        while cursor != 0 {
            let candidate = cursor;
            cursor = e.mem.u32(cursor);
            if directory == e.mem.u32(candidate + 8) {
                found = candidate;
                break;
            }
        }
        if found != 0 {
            if found == head {
                e.call(LIST_REMOVE_HEAD, &args![list]);
            } else if found == e.mem.u32(list + 4) {
                e.call(LIST_REMOVE_TAIL, &args![list]);
            } else {
                let previous = e.mem.u32(found + 4);
                let next = e.mem.u32(found);
                if previous != 0 {
                    e.mem.set_u32(previous, next);
                }
                if next != 0 {
                    e.mem.set_u32(next + 4, previous);
                }
                e.mem.set_u32(found + 8, 0);
                e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
                e.mem.set_u32(found + 4, 0);
                let free = e.mem.u32(NODE_POOL_FREE_LIST);
                e.mem.set_u32(found, free);
                let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
                e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
                e.mem.set_u32(NODE_POOL_FREE_LIST, found);
                if count == 0 {
                    e.mem.set_u32(NODE_POOL_LOCK, 0);
                }
                let remaining = e.mem.u32(list + 8);
                e.mem.set_u32(list + 8, remaining.wrapping_sub(1));
            }
        }
        e.call(NI_FREE, &args![directory]);
    }
}

// Translated from 00e93e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::~NiD3DShaderProgramFactory` (engine map):
/// the vertex and pixel shader maps (+0x14, +0x18) emptied and deleted
/// (vtable slot 0 with 1) when they exist, the program directories removed
/// and the list emptied (`00b71300`), then the `NiRefObject` vtable and one
/// live object less (`011f4400`).
pub fn ni_d3d_shader_program_factory_destruct(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, PROGRAM_FACTORY_VTABLE);
    for offset in [0x14, 0x18] {
        let map = e.mem.u32(base + offset);
        if map != 0 {
            e.call(MAP_REMOVE_ALL, &args![map]);
        }
        let map = e.mem.u32(base + offset);
        if map != 0 {
            e.vcall(map, 0, &args![1u32]);
        }
    }
    ni_d3d_shader_program_factory_remove_all_program_directories(e, this);
    ni_t_pointer_list_base_remove_all(e, Ptr::new(base + 8));
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00e93f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::scalar deleting destructor` (engine map): the
/// destructor, then with bit 0 of `flags` the 0x1C bytes freed
/// (`00aa1460`). Returns `this`.
pub fn ni_d3d_shader_program_factory_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    ni_d3d_shader_program_factory_destruct(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, 0x1cu32]);
    }
    this
}

/// The program factory's renderer globals (`0126fa1c` the renderer, `0126fa18` its device,
/// counted with the device's `AddRef`/`Release`, slots 4 and 8, `0126fa20` its shader
/// library word, the renderer's +0x8b8).
const FACTORY_RENDERER_0126FA1C: u32 = 0x0126_fa1c;
const FACTORY_DEVICE_0126FA18: u32 = 0x0126_fa18;
const FACTORY_LIBRARY_0126FA20: u32 = 0x0126_fa20;

// Translated from 00e92f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderprogramfactory.cpp`), `cdecl`, the renderer: keeps
/// it at `0126fa1c`. With a renderer, the device held at `0126fa18` is released (slot 8) and
/// replaced by the renderer's device (+0x288; counted, slot 4, when not null), and `0126fa20`
/// gets the renderer's +0x8b8. Without one, `0126fa20` is cleared, the held device released
/// and `0126fa18` cleared.
pub fn fn_00e92f70(e: &mut Engine, renderer: u32) {
    e.mem.set_u32(FACTORY_RENDERER_0126FA1C, renderer);
    if renderer != 0 {
        let device = e.mem.u32(renderer.wrapping_add(0x288));
        let held = e.mem.u32(FACTORY_DEVICE_0126FA18);
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(FACTORY_DEVICE_0126FA18, device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
        let renderer = e.mem.u32(FACTORY_RENDERER_0126FA1C);
        let library = e.mem.u32(renderer.wrapping_add(0x8b8));
        e.mem.set_u32(FACTORY_LIBRARY_0126FA20, library);
    } else {
        let held = e.mem.u32(FACTORY_DEVICE_0126FA18);
        e.mem.set_u32(FACTORY_LIBRARY_0126FA20, 0);
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(FACTORY_DEVICE_0126FA18, 0);
    }
}

/// The `NiRefObject` table and the global object count (`011f4400`, counted up with
/// `InterlockedIncrement`), the factory's table (`010f0f70`) and its two maps' base and
/// final tables (`010f0df0` then `010f0f10`, `010f0e50` then `010f0f30`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const PROGRAM_FACTORY_VTABLE: u32 = 0x010f_0f70;
const FIRST_MAP_BASE_VTABLE: u32 = 0x010f_0df0;
const FIRST_MAP_VTABLE: u32 = 0x010f_0f10;
const SECOND_MAP_BASE_VTABLE: u32 = 0x010f_0e50;
const SECOND_MAP_VTABLE: u32 = 0x010f_0f30;

/// One of the factory's maps (0x14 bytes from the memory manager): base table, 0x3b buckets
/// (0xec bytes of `00aa1070`, cleared), count 0, byte +0x10 cleared, final table; null when
/// the allocation fails.
fn program_map(e: &mut Engine, base_vtable: u32, vtable: u32) -> u32 {
    let map = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if map == 0 {
        return 0;
    }
    e.mem.set_u32(map, base_vtable);
    e.mem.set_u32(map + 4, 0x3b);
    e.mem.set_u32(map + 0xc, 0);
    let buckets = e.call(ALLOCATE_BLOCK, &args![0xecu32]).u32();
    e.mem.set_u32(map + 8, buckets);
    let size = e.mem.u32(map + 4).wrapping_mul(4);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    e.mem.set_u8(map + 0x10, 0);
    e.mem.set_u32(map, vtable);
    map
}

// Translated from 00e93ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderprogramfactory.cpp`): the shader program factory
/// constructor, `thiscall`, no stack words. The `NiRefObject` start, the factory's table, +8,
/// +0xc and +0x10 cleared, and its two maps at +0x14 and +0x18. Answers `this`.
pub fn fn_00e93ad0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
    e.mem.set_u32(base, PROGRAM_FACTORY_VTABLE);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let first = program_map(e, FIRST_MAP_BASE_VTABLE, FIRST_MAP_VTABLE);
    e.mem.set_u32(base + 0x14, first);
    let second = program_map(e, SECOND_MAP_BASE_VTABLE, SECOND_MAP_VTABLE);
    e.mem.set_u32(base + 0x18, second);
    this
}

/// The map lookup (`00a53c30`, `thiscall`: key, address of the value; answers in `AL`).
const MAP_GET_AT: u32 = 0x00a5_3c30;
/// `NiShaderFactory::ReportError` (`00aa0020`, engine map; `cdecl`: 1, the recoverable flag,
/// the format, the format's words).
const REPORT_ERROR: u32 = 0x00aa_0020;
/// `NiFile::Access` (`00aa1a30`, engine map; `cdecl`: path, mode; answers in `AL`).
const NI_FILE_ACCESS: u32 = 0x00aa_1a30;
/// `NiSprintf` (`00a305b0`, engine map; `cdecl`: buffer, size, format, words).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `NiStricmp` (`00aa1360`, engine map; `cdecl`: two strings).
const NI_STRICMP: u32 = 0x00aa_1360;
/// `00e983f0` (`cdecl`, five words: path, drive, directory, file name, extension; unnamed in
/// the engine map): splits a path into the four buffers.
const SPLIT_PATH_00E983F0: u32 = 0x00e9_83f0;
/// `NiD3DUtility::GetFileExtension` (`00e98430`, `nid3dutility.cpp`; `cdecl`: path, buffer,
/// size).
const GET_FILE_EXTENSION: u32 = 0x00e9_8430;
/// The directory list's node allocation (`0043a010`, `thiscall` on the list's allocator, no
/// stack words; the engine map's `NiTPointerListBase<..>::AddHead`, whose body takes a cleared
/// node from the shared pool) and its removal of an element (`00e7db70`, `thiscall` on the
/// list, one word: the address of the element; unnamed in the engine map).
const LIST_NEW_NODE_0043A010: u32 = 0x0043_a010;
const LIST_REMOVE_00E7DB70: u32 = 0x00e7_db70;
/// The empty string (`01011584`) the directory functions compare with.
const EMPTY_STRING: u32 = 0x0101_1584;
/// Formats: `"No valid shader program factory\n"` (`010f0dc8`), `"Invalid or no shader
/// program directory\n"` (`010f0d64`), `"Shader program file not found %s in directory %s\n"`
/// (`010f0d8c`), `"%s\\%s%s"` (`010f0dc0`) and `"%s%s%s"` (`0104f208`).
const NO_VALID_FACTORY: u32 = 0x010f_0dc8;
const INVALID_DIRECTORY: u32 = 0x010f_0d64;
const FILE_NOT_FOUND: u32 = 0x010f_0d8c;
const JOIN_WITH_SEPARATOR: u32 = 0x010f_0dc0;
const JOIN: u32 = 0x0104_f208;

// Translated from 00e92eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::GetInstance` (Xbox PDB, engine map): answers the single
/// instance (`0126fa10`).
pub fn ni_d3d_shader_program_factory_get_instance(e: &mut Engine) -> u32 {
    e.global::<u32>(FACTORY_INSTANCE)
}

/// `00e92ee0` and `00e92f30`: with an instance and a shader, 1 when the shader's handle slot
/// (`handle_slot`) answers one; otherwise the shader's creator (slot +0x6c) recreates it
/// (its slot `recreate_slot`, with the shader; the answer is the creator's), 0 without a
/// creator.
fn recreate_shader(e: &mut Engine, shader: u32, handle_slot: u32, recreate_slot: u32) -> bool {
    if e.global::<u32>(FACTORY_INSTANCE) == 0 || shader == 0 {
        return false;
    }
    if e.vcall(shader, handle_slot, &[]).u32() != 0 {
        return true;
    }
    let creator = e.vcall(shader, 0x6c, &[]).u32();
    if creator == 0 {
        return false;
    }
    e.vcall(creator, recreate_slot, &[shader]).bool()
}

// Translated from 00e92ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::RecreateVertexShader` (Xbox PDB, engine map), `cdecl`, one
/// word: answers 1 for a vertex shader that has its handle (slot +0x84), else its creator's
/// `RecreateVertexShader` (creator slot +0x1c); 0 without an instance, a shader or a creator.
pub fn ni_d3d_shader_program_factory_recreate_vertex_shader(e: &mut Engine, shader: u32) -> bool {
    recreate_shader(e, shader, 0x84, 0x1c)
}

// Translated from 00e92f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::RecreatePixelShader` (Xbox PDB, engine map), `cdecl`, one word:
/// as the vertex one with the pixel shader's handle (slot +0x7c) and the creator's
/// `RecreatePixelShader` (slot +0x20).
pub fn ni_d3d_shader_program_factory_recreate_pixel_shader(e: &mut Engine, shader: u32) -> bool {
    recreate_shader(e, shader, 0x7c, 0x20)
}

// Translated from 00e92fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::GetFirstProgramDirectory` (Xbox PDB, engine map), `thiscall`,
/// one word (the position out): the position becomes the directory list's head (+0x08) and,
/// with a head, then the head's next node; answers the head's directory (+0x08), or 0.
pub fn ni_d3d_shader_program_factory_get_first_program_directory(
    e: &mut Engine,
    this: Ptr,
    position: u32,
) -> u32 {
    let head = e.mem.u32(this.addr() + 8);
    e.mem.set_u32(position, head);
    if head == 0 {
        return 0;
    }
    let next = e.mem.u32(head);
    e.mem.set_u32(position, next);
    e.mem.u32(head + 8)
}

/// `00e93020` and `00e93060`: the shader the map at `map_offset` holds under `name` (a local
/// cleared before the lookup), or 0 without a map or without the key.
fn find_shader(e: &mut Engine, this: Ptr, map_offset: u32, name: u32) -> u32 {
    let map = e.mem.u32(this.addr() + map_offset);
    if map == 0 {
        return 0;
    }
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), 0);
        let found = e.call(MAP_GET_AT, &[map, name, value.addr()]).bool();
        if found {
            e.mem.u32(value.addr())
        } else {
            0
        }
    })
}

// Translated from 00e93020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::GetVertexShader` (Xbox PDB, engine map), `thiscall`, one word:
/// the vertex shader the map at +0x14 holds under the name, or 0.
pub fn ni_d3d_shader_program_factory_get_vertex_shader(
    e: &mut Engine,
    this: Ptr,
    name: u32,
) -> u32 {
    find_shader(e, this, 0x14, name)
}

// Translated from 00e93060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::GetPixelShader` (Xbox PDB, engine map), `thiscall`, one word:
/// the pixel shader the map at +0x18 holds under the name, or 0.
pub fn ni_d3d_shader_program_factory_get_pixel_shader(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    find_shader(e, this, 0x18, name)
}

// Translated from 00e930a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e930a0 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiD3DShaderProgramFactory::ResolveShaderFileName` (Xbox PDB, engine map), `cdecl`, three
/// words (file, path buffer, buffer size). A file `NiFile::Access` finds (mode 0) is copied
/// into the buffer (`_strcpy_s`): 1. Without an instance: reports `"No valid shader program
/// factory\n"` (recoverable 0) and answers 0. Otherwise the file is split (`00e983f0` into
/// four locals: drive, directory, name, extension) and each program directory of the list
/// (+0x08) is tried in order: an empty one is reported (`"Invalid or no shader program
/// directory\n"`), else the buffer gets directory, name and extension (`NiSprintf` with
/// `"%s%s%s"` when the directory ends in `/` or `\`, else `"%s\\%s%s"`); a path that exists
/// answers 1, one that does not is reported (`"Shader program file not found %s in directory
/// %s\n"`, recoverable 1, with the file and the directory). The walk stops at the list's end
/// or at a node without a directory: 0.
pub fn ni_d3d_shader_program_factory_resolve_shader_file_name(
    e: &mut Engine,
    file: u32,
    path: u32,
    size: u32,
) -> bool {
    if e.call(NI_FILE_ACCESS, &[file, 0]).bool() {
        e.call(STRCPY_S, &[path, size, file]);
        return true;
    }
    if e.global::<u32>(FACTORY_INSTANCE) == 0 {
        e.call(REPORT_ERROR, &[1, 0, NO_VALID_FACTORY]);
        return false;
    }
    // The locals: drive (4 bytes), extension (0x100), name (0x104), directory (0x104).
    e.with_stack(0x310, |e, locals| {
        let drive = locals.addr();
        let extension = drive + 4;
        let name = drive + 0x104;
        let directory_part = drive + 0x208;
        e.call(
            SPLIT_PATH_00E983F0,
            &[file, drive, directory_part, name, extension],
        );
        let instance = e.global::<u32>(FACTORY_INSTANCE);
        let head = e.mem.u32(instance + 8);
        if head == 0 {
            return false;
        }
        let mut directory = e.mem.u32(head + 8);
        let mut next = e.mem.u32(head);
        while directory != 0 {
            if e.mem.u8(directory) == 0 {
                e.call(REPORT_ERROR, &[1, 0, INVALID_DIRECTORY]);
            } else {
                let text = e.mem.cstr(directory);
                let last = text[text.len() - 1];
                let format = if last == b'/' || last == b'\\' {
                    JOIN
                } else {
                    JOIN_WITH_SEPARATOR
                };
                e.call(
                    NI_SPRINTF,
                    &[path, size, format, directory, name, extension],
                );
                if e.call(NI_FILE_ACCESS, &[path, 0]).bool() {
                    return true;
                }
                e.call(REPORT_ERROR, &[1, 1, FILE_NOT_FOUND, file, directory]);
            }
            if next == 0 {
                return false;
            }
            directory = e.mem.u32(next + 8);
            next = e.mem.u32(next);
        }
        false
    })
}

// Translated from 00e932f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e932f0 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiD3DShaderProgramFactory::GetShaderCreator` (Xbox PDB, engine map), `cdecl`, one word
/// (the file or type name): 0 without the creator map (`0126fa14`) or for a null or empty
/// name. Otherwise takes the name's extension (`GetFileExtension` into a 0x100-byte local);
/// the extension without its first character (the dot), or the whole name when that is
/// empty, is lower-cased into the local (`tolower` per character, up to 0x100 characters with
/// the terminator), and the creator the map holds under it is answered (0 when none).
pub fn ni_d3d_shader_program_factory_get_shader_creator(e: &mut Engine, name: u32) -> u32 {
    let map = e.mem.u32(CREATOR_MAP_SLOT);
    if map == 0 || name == 0 || e.mem.u8(name) == 0 {
        return 0;
    }
    e.with_stack(0x104, |e, locals| {
        let value = locals.addr();
        let buffer = locals.addr() + 4;
        e.call(GET_FILE_EXTENSION, &[name, buffer, 0x100]);
        let source = if e.mem.u8(buffer + 1) != 0 {
            buffer + 1
        } else {
            name
        };
        let mut index = 0u32;
        loop {
            let character = e.mem.u8(source.wrapping_add(index));
            let lower = e.call(TOLOWER, &args![character as i8 as i32]).u8();
            e.mem.set_u8(buffer + index, lower);
            if lower == 0 {
                break;
            }
            index += 1;
            if index >= 0x100 {
                break;
            }
        }
        e.mem.set_u32(value, 0);
        let map = e.mem.u32(CREATOR_MAP_SLOT);
        let found = e.call(MAP_GET_AT, &[map, buffer, value]).bool();
        if found {
            e.mem.u32(value)
        } else {
            0
        }
    })
}

/// The six map scalar deleting destructors: each vtable of `vtables` installed in turn with
/// the map emptied under it (`00b601a0`), the bucket array (+0x08) freed (`00aa10f0`), and
/// with bit 0 of `flags` the map freed through the memory manager. Answers `this`.
fn delete_pointer_map(e: &mut Engine, this: Ptr, flags: u32, vtables: &[u32]) -> Ptr {
    let map = this.addr();
    for vtable in vtables {
        e.mem.set_u32(map, *vtable);
        e.call(MAP_REMOVE_ALL, &args![map]);
    }
    let buckets = e.mem.u32(map + 8);
    e.call(NI_FREE, &args![buckets]);
    free_if_flagged(e, this, flags)
}

// Translated from 00e935d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NiTPointerAllocator<unsigned int>,const char*,NiD3DVertexShader*>` scalar
/// deleting destructor (vtable `010f0df0`, by its RTTI; unnamed in the engine map), `thiscall`
/// with one stack word (the flags).
pub fn fn_00e935d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_pointer_map(e, this, flags, &[FIRST_MAP_BASE_VTABLE])
}

// Translated from 00e93610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPointerMap<const char*,NiD3DVertexShader*>` scalar deleting destructor (vtable
/// `010f0e10`, then its base's `010f0df0`; unnamed in the engine map).
pub fn fn_00e93610(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_pointer_map(
        e,
        this,
        flags,
        &[VERTEX_POINTER_MAP_VTABLE, FIRST_MAP_BASE_VTABLE],
    )
}

// Translated from 00e93670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NiTPointerAllocator<unsigned int>,const char*,NiD3DPixelShader*>` scalar
/// deleting destructor (vtable `010f0e50`; unnamed in the engine map).
pub fn fn_00e93670(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_pointer_map(e, this, flags, &[SECOND_MAP_BASE_VTABLE])
}

// Translated from 00e936b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPointerMap<const char*,NiD3DPixelShader*>` scalar deleting destructor (vtable
/// `010f0e70`, then `010f0e50`; unnamed in the engine map).
pub fn fn_00e936b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_pointer_map(
        e,
        this,
        flags,
        &[PIXEL_POINTER_MAP_VTABLE, SECOND_MAP_BASE_VTABLE],
    )
}

// Translated from 00e93710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NiTPointerAllocator<unsigned int>,const char*,NiD3DShaderProgramCreator*>`
/// scalar deleting destructor (vtable `010f0eb0`; unnamed in the engine map).
pub fn fn_00e93710(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_pointer_map(e, this, flags, &[CREATOR_MAP_BASE_VTABLE])
}

// Translated from 00e93750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPointerMap<const char*,NiD3DShaderProgramCreator*>` scalar deleting destructor
/// (vtable `010f0ed0`, then `010f0eb0`; unnamed in the engine map).
pub fn fn_00e93750(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_pointer_map(
        e,
        this,
        flags,
        &[CREATOR_POINTER_MAP_VTABLE, CREATOR_MAP_BASE_VTABLE],
    )
}

/// `00e937f0` and `00e93820`: a non-null shader goes into the map at `map_offset` (when there
/// is one) under its name (slot +0x50; the exe pushes the shader before that call and the
/// map's `00a642a0` takes it as its value).
fn insert_shader(e: &mut Engine, this: Ptr, map_offset: u32, shader: u32) {
    if shader == 0 || e.mem.u32(this.addr() + map_offset) == 0 {
        return;
    }
    let name = e.vcall(shader, 0x50, &[]).u32();
    let map = e.mem.u32(this.addr() + map_offset);
    e.call(MAP_SET_AT, &[map, name, shader]);
}

// Translated from 00e937f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::InsertVertexShaderIntoMap` (Xbox PDB, engine map), `thiscall`,
/// one word: stores the shader under its name in the vertex shader map (+0x14).
pub fn ni_d3d_shader_program_factory_insert_vertex_shader_into_map(
    e: &mut Engine,
    this: Ptr,
    shader: u32,
) {
    insert_shader(e, this, 0x14, shader);
}

// Translated from 00e93820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::InsertPixelShaderIntoMap` (Xbox PDB, engine map), `thiscall`,
/// one word: stores the shader under its name in the pixel shader map (+0x18).
pub fn ni_d3d_shader_program_factory_insert_pixel_shader_into_map(
    e: &mut Engine,
    this: Ptr,
    shader: u32,
) {
    insert_shader(e, this, 0x18, shader);
}

// Translated from 00e93850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::CreateVertexShaderFromBuffer` (Xbox PDB, engine map), `cdecl`,
/// ten words (code, size, type, name, entry point, target, declaration, usage, software
/// processing, recoverable): 0 without an instance or without code or size. A vertex shader
/// already held under the name is answered; otherwise the type's creator
/// (`GetShaderCreator`; 0 without one) makes it (its slot +0x10 with code, size, name, entry
/// point, target, declaration, usage, software processing, recoverable), it goes into the
/// instance's map and is answered.
pub fn ni_d3d_shader_program_factory_create_vertex_shader_from_buffer(
    e: &mut Engine,
    code: u32,
    size: u32,
    kind: u32,
    name: u32,
    entry_point: u32,
    shader_target: u32,
    declaration: u32,
    usage: u32,
    software_vp: u32,
    recoverable: u32,
) -> u32 {
    let instance = e.global::<u32>(FACTORY_INSTANCE);
    if instance == 0 || code == 0 || size == 0 {
        return 0;
    }
    let held = ni_d3d_shader_program_factory_get_vertex_shader(e, Ptr::new(instance), name);
    if held != 0 {
        return held;
    }
    let creator = ni_d3d_shader_program_factory_get_shader_creator(e, kind);
    if creator == 0 {
        return 0;
    }
    let shader = e
        .vcall(
            creator,
            0x10,
            &[
                code,
                size,
                name,
                entry_point,
                shader_target,
                declaration,
                usage,
                software_vp,
                recoverable,
            ],
        )
        .u32();
    let instance = e.global::<u32>(FACTORY_INSTANCE);
    ni_d3d_shader_program_factory_insert_vertex_shader_into_map(e, Ptr::new(instance), shader);
    shader
}

// Translated from 00e938e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::CreatePixelShaderFromBuffer` (Xbox PDB, engine map), `cdecl`,
/// seven words (code, size, type, name, entry point, target, recoverable): as the vertex one
/// with the pixel shader map and the creator's slot +0x18 (code, size, name, entry point,
/// target, recoverable).
pub fn ni_d3d_shader_program_factory_create_pixel_shader_from_buffer(
    e: &mut Engine,
    code: u32,
    size: u32,
    kind: u32,
    name: u32,
    entry_point: u32,
    shader_target: u32,
    recoverable: u32,
) -> u32 {
    let instance = e.global::<u32>(FACTORY_INSTANCE);
    if instance == 0 || code == 0 || size == 0 {
        return 0;
    }
    let held = ni_d3d_shader_program_factory_get_pixel_shader(e, Ptr::new(instance), name);
    if held != 0 {
        return held;
    }
    let creator = ni_d3d_shader_program_factory_get_shader_creator(e, kind);
    if creator == 0 {
        return 0;
    }
    let shader = e
        .vcall(
            creator,
            0x18,
            &[code, size, name, entry_point, shader_target, recoverable],
        )
        .u32();
    let instance = e.global::<u32>(FACTORY_INSTANCE);
    ni_d3d_shader_program_factory_insert_pixel_shader_into_map(e, Ptr::new(instance), shader);
    shader
}

/// The directory functions' common start, inlined in both: a copy of `directory` in the
/// 0x104-byte `buffer` (`_strcpy_s`) with every `/` turned into `\` (the length measured
/// again after each character).
fn copy_directory(e: &mut Engine, buffer: u32, directory: u32) {
    e.call(STRCPY_S, &[buffer, 0x104, directory]);
    if e.mem.cstr(buffer).is_empty() {
        return;
    }
    let mut index = 0u32;
    loop {
        if e.mem.u8(buffer + index) == b'/' {
            e.mem.set_u8(buffer + index, b'\\');
        }
        index += 1;
        if index >= e.mem.cstr(buffer).len() as u32 {
            break;
        }
    }
}

// Translated from 00e93960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e93960 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiD3DShaderProgramFactory::AddProgramDirectory` (Xbox PDB, engine map), `thiscall`, one
/// word: nothing for a null directory or one equal to the empty string (`01011584`).
/// Otherwise the directory is copied with `\` separators; when no directory of the list
/// (+0x08) matches it (`NiStricmp`), a copy of it (`00aa1070`, `_strcpy_s`) goes into a new
/// node (`0043a010` on the list's allocator, +0x10) put at the head of the list (the old head's
/// previous link, or the tail +0x0C for an empty list, and one more in the count +0x10).
pub fn ni_d3d_shader_program_factory_add_program_directory(
    e: &mut Engine,
    this: Ptr,
    directory: u32,
) {
    if directory == 0 || e.mem.cstr(directory) == e.mem.cstr(EMPTY_STRING) {
        return;
    }
    let factory = this.addr();
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        copy_directory(e, buffer, directory);
        let mut node = e.mem.u32(factory + 8);
        while node != 0 {
            let held = e.mem.u32(node + 8);
            node = e.mem.u32(node);
            if held != 0 && e.call(NI_STRICMP, &[held, buffer]).u32() == 0 {
                return;
            }
        }
        let size = e.mem.cstr(buffer).len() as u32 + 1;
        let copy = e.call(ALLOCATE_BLOCK, &[size]).u32();
        e.call(STRCPY_S, &[copy, size, buffer]);
        let count = factory + 0x10;
        let node = e.call(LIST_NEW_NODE_0043A010, &[count]).u32();
        e.mem.set_u32(node + 8, copy);
        e.mem.set_u32(node + 4, 0);
        let head = e.mem.u32(factory + 8);
        e.mem.set_u32(node, head);
        let head = e.mem.u32(factory + 8);
        if head != 0 {
            e.mem.set_u32(head + 4, node);
        } else {
            e.mem.set_u32(factory + 0xc, node);
        }
        let items = e.mem.u32(count).wrapping_add(1);
        e.mem.set_u32(count, items);
        e.mem.set_u32(factory + 8, node);
    });
}

// Translated from 00e93bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e93bb0 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiD3DShaderProgramFactory::RemoveProgramDirectory` (Xbox PDB, engine map), `thiscall`, one
/// word: nothing for a null directory or the empty string. Otherwise the directory is copied
/// with `\` separators and the first directory of the list (+0x08) that matches it
/// (`NiStricmp`) is removed from the list (`00e7db70` with the address of a local holding the
/// list's string; the string itself is not freed).
pub fn ni_d3d_shader_program_factory_remove_program_directory(
    e: &mut Engine,
    this: Ptr,
    directory: u32,
) {
    if directory == 0 || e.mem.cstr(directory) == e.mem.cstr(EMPTY_STRING) {
        return;
    }
    let list = this.addr() + 8;
    e.with_stack(0x108, |e, locals| {
        let held_slot = locals.addr();
        let buffer = locals.addr() + 4;
        copy_directory(e, buffer, directory);
        let mut node = e.mem.u32(list);
        while node != 0 {
            let held = e.mem.u32(node + 8);
            node = e.mem.u32(node);
            e.mem.set_u32(held_slot, held);
            if held != 0 && e.call(NI_STRICMP, &[held, buffer]).u32() == 0 {
                e.call(LIST_REMOVE_00E7DB70, &[list, held_slot]);
                return;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_2eb0,
            ni_d3d_shader_program_factory_get_instance() -> u32
        ),
        entry!(
            0x00e9_2ee0,
            ni_d3d_shader_program_factory_recreate_vertex_shader(u32) -> bool
        ),
        entry!(
            0x00e9_2f30,
            ni_d3d_shader_program_factory_recreate_pixel_shader(u32) -> bool
        ),
        entry!(
            0x00e9_2fe0,
            ni_d3d_shader_program_factory_get_first_program_directory(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_3020,
            ni_d3d_shader_program_factory_get_vertex_shader(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_3060,
            ni_d3d_shader_program_factory_get_pixel_shader(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_30a0,
            ni_d3d_shader_program_factory_resolve_shader_file_name(u32, u32, u32) -> bool
        ),
        entry!(
            0x00e9_32f0,
            ni_d3d_shader_program_factory_get_shader_creator(u32) -> u32
        ),
        entry!(0x00e9_35d0, fn_00e935d0(Ptr, u32) -> Ptr),
        entry!(0x00e9_3610, fn_00e93610(Ptr, u32) -> Ptr),
        entry!(0x00e9_3670, fn_00e93670(Ptr, u32) -> Ptr),
        entry!(0x00e9_36b0, fn_00e936b0(Ptr, u32) -> Ptr),
        entry!(0x00e9_3710, fn_00e93710(Ptr, u32) -> Ptr),
        entry!(0x00e9_3750, fn_00e93750(Ptr, u32) -> Ptr),
        entry!(
            0x00e9_37f0,
            ni_d3d_shader_program_factory_insert_vertex_shader_into_map(Ptr, u32)
        ),
        entry!(
            0x00e9_3820,
            ni_d3d_shader_program_factory_insert_pixel_shader_into_map(Ptr, u32)
        ),
        entry!(
            0x00e9_3850,
            ni_d3d_shader_program_factory_create_vertex_shader_from_buffer(
                u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e9_38e0,
            ni_d3d_shader_program_factory_create_pixel_shader_from_buffer(
                u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e9_3960,
            ni_d3d_shader_program_factory_add_program_directory(Ptr, u32)
        ),
        entry!(
            0x00e9_3bb0,
            ni_d3d_shader_program_factory_remove_program_directory(Ptr, u32)
        ),
        entry!(
            0x00e9_3d80,
            ni_d3d_shader_program_factory_create_shader_creator_map()
        ),
        entry!(0x00e9_3df0, fn_00e93df0()),
        entry!(0x00e9_3e70, ni_d3d_shader_program_factory_register_shader_creator(Ptr, u32) -> bool),
        entry!(
            0x00b7_1300,
            ni_t_pointer_list_base_remove_all(Ptr<crate::types::NiTPointerList>)
        ),
        entry!(
            0x00ae_7f90,
            ni_t_pointer_list_base_remove_head(Ptr<crate::types::NiTPointerList>) -> u32
        ),
        entry!(0x00e9_2ec0, fn_00e92ec0()),
        entry!(0x00e9_2f70, fn_00e92f70(u32)),
        entry!(0x00e9_3000, ni_d3d_shader_program_factory_get_next_program_directory(u32) -> u32),
        entry!(0x00e9_3260, fn_00e93260(u32)),
        entry!(0x00e9_33d0, fn_00e933d0()),
        entry!(0x00e9_3400, fn_00e93400(Ptr)),
        entry!(0x00e9_3470, fn_00e93470(Ptr, u32, u32, u32)),
        entry!(0x00e9_34d0, fn_00e934d0(Ptr)),
        entry!(0x00e9_3540, fn_00e93540(Ptr)),
        entry!(0x00e9_35b0, fn_00e935b0(Ptr, u32)),
        entry!(0x00e9_3650, fn_00e93650(Ptr, u32) -> Ptr),
        entry!(0x00e9_36f0, fn_00e936f0(Ptr, u32) -> Ptr),
        entry!(0x00e9_3790, fn_00e93790(u32)),
        entry!(0x00e9_37c0, fn_00e937c0(u32)),
        entry!(0x00e9_3ab0, fn_00e93ab0(Ptr, u32) -> Ptr),
        entry!(0x00e9_3ad0, fn_00e93ad0(Ptr) -> Ptr),
        entry!(
            0x00e9_3cc0,
            ni_d3d_shader_program_factory_remove_all_program_directories(Ptr)
        ),
        entry!(0x00e9_3e00, ni_d3d_shader_program_factory_destruct(Ptr)),
        entry!(0x00e9_3f30, ni_d3d_shader_program_factory_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

/// The node pool's lock object (`this` of the lock enter `0040fbf0`, thiscall,
/// one word); its owner word.
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
/// The lock's entry count, which the inlined leave decrements.
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
/// The lock enter (`0040fbf0`; thiscall, one word).
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// Head of the node pool's free list.
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;

// Translated from 00b71300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,char *>::RemoveAll`
/// (engine map): returns every node of the list to the shared node pool:
/// for each node clears its element (+0x08), enters the pool's lock
/// (`0040fbf0(0)`), clears its previous link (+0x04), pushes it on the free
/// list at `011c5f58` and leaves the lock (the count at `011c5f84` drops by
/// one; at zero the owner word `011c5f80` is cleared). Then clears the
/// count, head and tail.
pub fn ni_t_pointer_list_base_remove_all(e: &mut Engine, this: Ptr<crate::types::NiTPointerList>) {
    use crate::types::NiTPointerList;
    let mut next = e.get(this, NiTPointerList::m_pkHead);
    while next != 0 {
        let node = next;
        next = e.mem.u32(node);
        e.mem.set_u32(node + 8, 0);
        e.call(LOCK_ENTER, &[NODE_POOL_LOCK, 0]);
        e.mem.set_u32(node + 4, 0);
        let free = e.mem.u32(NODE_POOL_FREE_LIST);
        e.mem.set_u32(node, free);
        let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
        e.mem.set_u32(NODE_POOL_FREE_LIST, node);
        if count == 0 {
            e.mem.set_u32(NODE_POOL_LOCK, 0);
        }
    }
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
}

#[cfg(test)]
mod remove_all_tests {
    use super::*;

    #[test]
    fn remove_all_returns_each_node_to_the_pool_and_empties_the_list() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        // The lock enter double takes the lock: owner 1, count + 1.
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            0u32.into_ret()
        });
        // Already held once by the caller: the owner stays after each leave.
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 1);
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0099_0000);
        let list = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 4, first);
        e.mem.set_u32(second + 8, 0x22);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1300, &[list]);
        let locks: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == LOCK_ENTER)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(locks, vec![vec![NODE_POOL_LOCK, 0]; 2]);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), second);
        assert_eq!(e.mem.u32(second), first);
        assert_eq!(e.mem.u32(first), 0x0099_0000);
        for node in [first, second] {
            assert_eq!(e.mem.u32(node + 4), 0);
            assert_eq!(e.mem.u32(node + 8), 0);
        }
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 1);
        assert_eq!(e.mem.bytes(list, 0xc), vec![0; 0xc]);
    }

    #[test]
    fn remove_all_clears_the_owner_when_the_lock_count_reaches_zero() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            0u32.into_ret()
        });
        let list = e.mem.alloc(0xc);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(list, node);
        e.mem.set_u32(list + 4, node);
        e.mem.set_u32(list + 8, 1);
        e.call(0x00b7_1300, &[list]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), node);
        // An empty list only clears its words.
        e.call_log = Some(vec![]);
        e.call(0x00b7_1300, &[list]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn creator_map_is_made_once_with_a_zeroed_slot_block() {
        let mut e = engine();
        e.call(0x00e9_3df0, &args![]);
        let map = e.mem.u32(CREATOR_MAP_SLOT);
        assert_ne!(map, 0);
        assert_eq!(e.mem.u32(map), 0x010f_0f50);
        assert_eq!(e.mem.u32(map + 4), 0x25);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        assert_eq!(e.mem.u8(map + 0x10), 1);
        let slots = e.mem.u32(map + 8);
        assert_eq!(e.mem.bytes(slots, 0x94), vec![0; 0x94]);
        e.call(0x00e9_3df0, &args![]);
        assert_eq!(e.mem.u32(CREATOR_MAP_SLOT), map);
    }

    #[test]
    fn failed_allocation_stores_null() {
        let mut e = engine();
        e.mem.set_u32(CREATOR_MAP_SLOT, 0);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e9_3d80, &args![]);
        assert_eq!(e.mem.u32(CREATOR_MAP_SLOT), 0);
    }

    fn register(name: &[u8]) -> (bool, Vec<(Vec<u8>, u32)>) {
        let mut e = engine();
        e.register(STRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().position(|c| u32::from(*c) == a[1]) {
                Some(at) => (a[0] + at as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(TOLOWER, |_, a| {
            u32::from((a[0] as u8).to_ascii_lowercase()).into_ret()
        });
        let stored: Rc<RefCell<Vec<(Vec<u8>, u32)>>> = Rc::default();
        let log = stored.clone();
        e.register_double(MAP_SET_AT, move |e, a| {
            log.borrow_mut().push((e.mem.cstr(a[1]), a[2]));
            Ret::default()
        });
        let text = e.mem.alloc(0x40);
        e.mem.set_cstr(text, name);
        let accepted = e.call(0x00e9_3e70, &args![text, 0x1234u32]).bool();
        let entries = stored.borrow().clone();
        (accepted, entries)
    }

    #[test]
    fn registers_the_lower_case_copy() {
        assert_eq!(
            register(b"NiD3DShader"),
            (true, vec![(b"nid3dshader".to_vec(), 0x1234)])
        );
    }

    #[test]
    fn refuses_names_with_a_dot() {
        assert_eq!(register(b"Shader.vsh"), (false, vec![]));
    }

    #[test]
    fn refuses_a_null_name() {
        let mut e = engine();
        assert!(!e.call(0x00e9_3e70, &args![0u32, 1u32]).bool());
    }
}

#[cfg(test)]
mod factory_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011c_5000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e
    }

    /// An object whose vtable has the test functions `0x7000 + slot`.
    fn object(e: &mut Engine) -> u32 {
        let slots: Vec<u32> = (0..0x18).map(|slot| 0x0000_7000 + 4 * slot).collect();
        e.put_vtable(0x0f00_0000, &slots);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0f00_0000);
        object
    }

    #[test]
    fn remove_head_unlinks_the_first_node() {
        let mut e = engine();
        stub(&mut e, &[NODE_DEALLOCATE]);
        let list = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(second + 4, first);
        e.mem.set_u32(second + 8, 0x22);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_7f90, &args![list]).u32(), 0x11);
        assert_eq!(e.mem.u32(list), second);
        assert_eq!(e.mem.u32(second + 4), 0);
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(calls_to(&e, NODE_DEALLOCATE), vec![vec![list + 8, first]]);
        assert_eq!(e.call(0x00ae_7f90, &args![list]).u32(), 0x22);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (0, 0));
        assert_eq!(e.mem.u32(list + 8), 0);
    }

    #[test]
    fn fn_00e92ec0_deletes_the_instance() {
        let mut e = engine();
        let instance = object(&mut e);
        stub(&mut e, &[0x0000_7000]);
        e.set_global(FACTORY_INSTANCE, instance);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2ec0, &args![]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![instance, 1]]);
        assert_eq!(e.global::<u32>(FACTORY_INSTANCE), 0);
        e.call(0x00e9_2ec0, &args![]);
        assert_eq!(calls_to(&e, 0x0000_7000).len(), 1);
    }

    #[test]
    fn fn_00e92f70_keeps_the_renderer_and_its_device() {
        let mut e = engine();
        let old = object(&mut e);
        let device = object(&mut e);
        stub(&mut e, &[0x0000_7004, 0x0000_7008]);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x55);
        e.set_global(FACTORY_DEVICE_0126FA18, old);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2f70, &args![renderer]);
        assert_eq!(e.global::<u32>(FACTORY_RENDERER_0126FA1C), renderer);
        assert_eq!(e.global::<u32>(FACTORY_DEVICE_0126FA18), device);
        assert_eq!(e.global::<u32>(FACTORY_LIBRARY_0126FA20), 0x55);
        assert_eq!(calls_to(&e, 0x0000_7008), vec![vec![old]]);
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![device]]);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2f70, &args![0u32]);
        assert_eq!(e.global::<u32>(FACTORY_RENDERER_0126FA1C), 0);
        assert_eq!(e.global::<u32>(FACTORY_DEVICE_0126FA18), 0);
        assert_eq!(e.global::<u32>(FACTORY_LIBRARY_0126FA20), 0);
        assert_eq!(calls_to(&e, 0x0000_7008), vec![vec![device]]);
    }

    #[test]
    fn get_next_program_directory_steps_the_position() {
        let mut e = engine();
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node, 0x4444);
        e.mem.set_u32(node + 8, 0x5555);
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, node);
        assert_eq!(e.call(0x00e9_3000, &args![position]).u32(), 0x5555);
        assert_eq!(e.mem.u32(position), 0x4444);
        e.mem.set_u32(position, 0);
        assert_eq!(e.call(0x00e9_3000, &args![position]).u32(), 0);
    }

    #[test]
    fn fn_00e93260_removes_the_creator_entries() {
        let mut e = engine();
        stub(&mut e, &[MAP_REMOVE_AT]);
        // Three entries in turn: (1, creator), (2, other), (3, creator).
        let mut step = 0u32;
        e.register_double(MAP_GET_NEXT, move |e, a| {
            step += 1;
            e.mem.set_u32(a[2], step);
            e.mem.set_u32(a[3], if step == 2 { 0x99 } else { 0x77 });
            e.mem
                .set_u32(a[1], if step == 3 { 0 } else { 0x1000 + step });
            Ret::default()
        });
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(0xc);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(buckets + 8, 0x1000);
        e.mem.set_u32(CREATOR_MAP_SLOT, map);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3260, &args![0x77u32]);
        assert_eq!(calls_to(&e, MAP_GET_NEXT).len(), 3);
        assert_eq!(
            calls_to(&e, MAP_REMOVE_AT),
            vec![vec![map, 1], vec![map, 3]]
        );
        // An empty map asks nothing.
        e.mem.set_u32(buckets + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3260, &args![0x77u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e933d0_destroys_the_creator_map() {
        let mut e = engine();
        let map = object(&mut e);
        stub(&mut e, &[MAP_REMOVE_ALL, 0x0000_7000]);
        e.mem.set_u32(CREATOR_MAP_SLOT, map);
        e.call_log = Some(vec![]);
        e.call(0x00e9_33d0, &args![]);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map]]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![map, 1]]);
        assert_eq!(e.mem.u32(CREATOR_MAP_SLOT), 0);
    }

    fn string_map(e: &mut Engine, copies: bool) -> (u32, u32) {
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x6001);
        e.mem.set_u32(second + 4, 0x6002);
        e.mem.set_u32(buckets + 4, first);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u8(map + 0x10, u8::from(copies));
        (map, buckets)
    }

    #[test]
    fn string_map_destructors_free_the_copied_keys() {
        for (address, deleting, vtable) in [
            (0x00e9_3400u32, 0x00e9_3650u32, FIRST_MAP_BASE_VTABLE),
            (0x00e9_34d0, 0x00e9_36f0, SECOND_MAP_BASE_VTABLE),
            (0x00e9_3540, 0x00e9_3ab0, CREATOR_MAP_BASE_VTABLE),
        ] {
            let mut e = engine();
            stub(&mut e, &[NI_FREE, MAP_REMOVE_ALL, MEMORY_MANAGER_FREE]);
            let (map, buckets) = string_map(&mut e, true);
            e.call_log = Some(vec![]);
            e.call(address, &args![map]);
            assert_eq!(e.mem.u32(map), vtable);
            assert_eq!(
                calls_to(&e, NI_FREE),
                vec![vec![0x6001], vec![0x6002], vec![buckets]]
            );
            assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map], vec![map]]);
            let (map, buckets) = string_map(&mut e, false);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(deleting, &args![map, 1u32]).u32(), map);
            assert_eq!(calls_to(&e, NI_FREE), vec![vec![buckets]]);
            assert_eq!(
                calls_to(&e, MEMORY_MANAGER_FREE),
                vec![vec![MEMORY_MANAGER_OBJECT, map]]
            );
            e.call_log = Some(vec![]);
            e.call(deleting, &args![map, 0u32]);
            assert!(calls_to(&e, MEMORY_MANAGER_FREE).is_empty());
        }
    }

    #[test]
    fn set_and_clear_value_copy_keys_with_the_flag() {
        let mut e = engine();
        e.register(ALLOCATE_BLOCK, |_, _| eax(0x8800));
        stub(&mut e, &[STRCPY_S, NI_FREE]);
        let map = e.mem.alloc(0x14);
        let item = e.mem.alloc(0xc);
        let key = e.mem.alloc(8);
        e.mem.set_cstr(key, b"abc");
        e.call_log = Some(vec![]);
        e.call(0x00e9_3470, &args![map, item, key, 0x99u32]);
        assert_eq!((e.mem.u32(item + 4), e.mem.u32(item + 8)), (key, 0x99));
        e.call(0x00e9_35b0, &args![map, item]);
        assert!(e.call_log.as_ref().unwrap().len() == 2);
        e.mem.set_u8(map + 0x10, 1);
        e.call(0x00e9_3470, &args![map, item, key, 0x98u32]);
        assert_eq!(calls_to(&e, ALLOCATE_BLOCK), vec![vec![4]]);
        assert_eq!(calls_to(&e, STRCPY_S), vec![vec![0x8800, 4, key]]);
        assert_eq!((e.mem.u32(item + 4), e.mem.u32(item + 8)), (0x8800, 0x98));
        e.call(0x00e9_35b0, &args![map, item]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x8800]]);
    }

    #[test]
    fn shader_names_leave_their_maps() {
        let mut e = engine();
        let shader = object(&mut e);
        e.register(0x0000_7050, |_, _| eax(0x6100));
        stub(&mut e, &[MAP_REMOVE_AT]);
        let instance = e.mem.alloc(0x1c);
        e.mem.set_u32(instance + 0x14, 0x4100);
        e.mem.set_u32(instance + 0x18, 0x4200);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3790, &args![shader]);
        assert!(calls_to(&e, 0x0000_7050).is_empty());
        e.set_global(FACTORY_INSTANCE, instance);
        e.call(0x00e9_3790, &args![shader]);
        e.call(0x00e9_37c0, &args![shader]);
        e.call(0x00e9_37c0, &args![0u32]);
        assert_eq!(
            calls_to(&e, MAP_REMOVE_AT),
            vec![vec![0x4100, 0x6100], vec![0x4200, 0x6100]]
        );
        e.mem.set_u32(instance + 0x18, 0);
        e.call(0x00e9_37c0, &args![shader]);
        assert_eq!(calls_to(&e, MAP_REMOVE_AT).len(), 2);
    }

    #[test]
    fn constructor_makes_both_maps() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| eax(e.mem.alloc(a[1])));
        e.register(ALLOCATE_BLOCK, |_, _| eax(0x9900));
        stub(&mut e, &[MEMSET, IMPORT_INTERLOCKED_INCREMENT]);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_3ad0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PROGRAM_FACTORY_VTABLE);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![OBJECT_COUNT_011F4400]]
        );
        let vertex = e.mem.u32(this + 0x14);
        let pixel = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u32(vertex), FIRST_MAP_VTABLE);
        assert_eq!(e.mem.u32(pixel), SECOND_MAP_VTABLE);
        assert_eq!(e.mem.u32(vertex + 4), 0x3b);
        assert_eq!(e.mem.u32(vertex + 8), 0x9900);
        assert_eq!(e.mem.u8(pixel + 0x10), 0);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![0x9900, 0, 0xec]; 2]);
        // Failed allocations leave null maps.
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| eax(0));
        e.call(0x00e9_3ad0, &args![this]);
        assert_eq!((e.mem.u32(this + 0x14), e.mem.u32(this + 0x18)), (0, 0));
    }

    #[test]
    fn remove_all_program_directories_frees_each_directory() {
        let mut e = engine();
        stub(
            &mut e,
            &[LIST_REMOVE_HEAD, LIST_REMOVE_TAIL, NI_FREE, LOCK_ENTER],
        );
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 2);
        let this = e.mem.alloc(0x1c);
        let nodes: Vec<u32> = (0..4).map(|_| e.mem.alloc(0xc)).collect();
        for (index, &node) in nodes.iter().enumerate() {
            e.mem
                .set_u32(node, nodes.get(index + 1).copied().unwrap_or(0));
            e.mem
                .set_u32(node + 4, if index == 0 { 0 } else { nodes[index - 1] });
            e.mem
                .set_u32(node + 8, if index == 2 { 0 } else { 0x6000 + index as u32 });
        }
        e.mem.set_u32(this + 8, nodes[0]);
        e.mem.set_u32(this + 0xc, nodes[3]);
        e.mem.set_u32(this + 0x10, 4);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3cc0, &args![this]);
        assert_eq!(calls_to(&e, LIST_REMOVE_HEAD), vec![vec![this + 8]]);
        // The second node is in the middle: unlinked here.
        assert_eq!(calls_to(&e, LOCK_ENTER), vec![vec![NODE_POOL_LOCK, 0]]);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), nodes[1]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(this + 0x10), 3);
        assert_eq!(calls_to(&e, LIST_REMOVE_TAIL), vec![vec![this + 8]]);
        assert_eq!(
            calls_to(&e, NI_FREE),
            vec![vec![0x6000], vec![0x6001], vec![0x6003]]
        );
    }

    #[test]
    fn destructor_and_deleting_destructor() {
        let mut e = engine();
        let map = object(&mut e);
        stub(
            &mut e,
            &[
                MAP_REMOVE_ALL,
                0x0000_7000,
                INTERLOCKED_DECREMENT,
                NI_FREE_SIZED,
            ],
        );
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, map);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_3f30, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map]]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![map, 1]]);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![OBJECT_COUNT_011F4400]]
        );
        assert_eq!(calls_to(&e, NI_FREE_SIZED), vec![vec![this, 0x1c]]);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3f30, &args![this, 0u32]);
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
        e.call(0x00e9_3e00, &args![this]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 2);
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e92f70_swaps_the_held_device() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.put_vtable(0x6000, &[0, 0x6104, 0x6108]);
        e.register(0x6104, |_, _| Ret::default());
        e.register(0x6108, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x66);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2f70, &args![renderer]);
        assert_eq!(e.mem.u32(FACTORY_DEVICE_0126FA18), device);
        assert_eq!(e.mem.u32(FACTORY_LIBRARY_0126FA20), 0x66);
        e.call(0x00e9_2f70, &args![0u32]);
        assert_eq!(e.mem.u32(FACTORY_DEVICE_0126FA18), 0);
        assert_eq!(e.mem.u32(FACTORY_LIBRARY_0126FA20), 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x6104, vec![device])));
        assert!(log.contains(&(0x6108, vec![device])));
    }

    #[test]
    fn fn_00e93ad0_makes_the_two_maps() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        assert_eq!(e.call(0x00e9_3ad0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PROGRAM_FACTORY_VTABLE);
        let first = e.mem.u32(this + 0x14);
        let second = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u32(first), FIRST_MAP_VTABLE);
        assert_eq!(e.mem.u32(second), SECOND_MAP_VTABLE);
        assert_eq!(e.mem.u32(first + 4), 0x3b);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e9_3ad0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
    }
}

#[cfg(test)]
mod n009_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const SLOTS: u32 = 0x0e70_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// An engine with the factory's globals mapped, an instance at `0126fa10` (with a
    /// directory list at +8 and two maps at +0x14 and +0x18) and the empty string.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.mem.set_u8(EMPTY_STRING, 0);
        let instance = e.mem.alloc(0x1c);
        e.mem.set_u32(instance + 0x14, 0x5140);
        e.mem.set_u32(instance + 0x18, 0x5180);
        e.set_global(FACTORY_INSTANCE, instance);
        e.call_log = Some(vec![]);
        (e, instance)
    }

    /// An object whose vtable's slots are `SLOTS + slot` (each a no-op double).
    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for slot in (0..0x100u32).step_by(4) {
            e.mem.set_u32(vtable + slot, SLOTS + slot);
            e.register(SLOTS + slot, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x48);
        e.mem.set_u32(object, vtable);
        object
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let block = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(block, s);
        block
    }

    /// A list of directory nodes (next +0, previous +4, element +8) holding `directories`.
    fn directory_list(e: &mut Engine, list: u32, directories: &[u32]) -> Vec<u32> {
        let nodes: Vec<u32> = directories.iter().map(|_| e.mem.alloc(0xc)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(node + 8, directories[i]);
            if i + 1 < nodes.len() {
                e.mem.set_u32(*node, nodes[i + 1]);
                e.mem.set_u32(nodes[i + 1] + 4, *node);
            }
        }
        e.mem.set_u32(list, nodes.first().copied().unwrap_or(0));
        e.mem.set_u32(list + 4, nodes.last().copied().unwrap_or(0));
        e.mem.set_u32(list + 8, nodes.len() as u32);
        nodes
    }

    #[test]
    fn get_instance_answers_the_global() {
        let (mut e, instance) = engine();
        assert_eq!(e.call(0x00e9_2eb0, &[]).u32(), instance);
    }

    fn recreates(address: u32, handle_slot: u32, recreate_slot: u32) {
        let (mut e, _) = engine();
        let shader = object(&mut e);
        let creator = object(&mut e);
        // With a handle: 1 at once.
        e.register(SLOTS + handle_slot, |_, _| 0x55u32.into_ret());
        assert!(e.call(address, &[shader]).bool());
        assert!(calls(&e, SLOTS + 0x6c).is_empty());
        // Without: the creator answers.
        e.register(SLOTS + handle_slot, |_, _| Ret::default());
        e.register_double(SLOTS + 0x6c, move |_, _| creator.into_ret());
        e.register(SLOTS + recreate_slot, |_, _| 1u32.into_ret());
        assert!(e.call(address, &[shader]).bool());
        assert_eq!(
            calls(&e, SLOTS + recreate_slot),
            vec![vec![creator, shader]]
        );
        e.register(SLOTS + recreate_slot, |_, _| 0x100u32.into_ret());
        assert!(!e.call(address, &[shader]).bool());
        // No creator, no shader, no instance: 0.
        e.register(SLOTS + 0x6c, |_, _| Ret::default());
        assert!(!e.call(address, &[shader]).bool());
        assert!(!e.call(address, &[0]).bool());
        e.set_global(FACTORY_INSTANCE, 0u32);
        e.register(SLOTS + handle_slot, |_, _| 1u32.into_ret());
        assert!(!e.call(address, &[shader]).bool());
    }

    #[test]
    fn recreate_vertex_shader_uses_its_handle_or_creator() {
        recreates(0x00e9_2ee0, 0x84, 0x1c);
    }

    #[test]
    fn recreate_pixel_shader_uses_its_handle_or_creator() {
        recreates(0x00e9_2f30, 0x7c, 0x20);
    }

    #[test]
    fn get_first_program_directory_starts_the_walk() {
        let (mut e, instance) = engine();
        let nodes = directory_list(&mut e, instance + 8, &[0x7100, 0x7200]);
        let position = e.mem.alloc(4);
        assert_eq!(e.call(0x00e9_2fe0, &[instance, position]).u32(), 0x7100);
        assert_eq!(e.mem.u32(position), nodes[1]);
        e.mem.set_u32(instance + 8, 0);
        assert_eq!(e.call(0x00e9_2fe0, &[instance, position]).u32(), 0);
        assert_eq!(e.mem.u32(position), 0);
    }

    fn finds(address: u32, offset: u32, map: u32) {
        let (mut e, instance) = engine();
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 0x7777);
            u32::from(a[1] == 0x1111).into_ret()
        });
        assert_eq!(e.call(address, &[instance, 0x1111]).u32(), 0x7777);
        assert_eq!(calls(&e, MAP_GET_AT)[0][..2], [map, 0x1111]);
        assert_eq!(e.call(address, &[instance, 0x2222]).u32(), 0);
        e.mem.set_u32(instance + offset, 0);
        assert_eq!(e.call(address, &[instance, 0x1111]).u32(), 0);
        assert_eq!(calls(&e, MAP_GET_AT).len(), 2);
    }

    #[test]
    fn get_vertex_shader_looks_in_its_map() {
        finds(0x00e9_3020, 0x14, 0x5140);
    }

    #[test]
    fn get_pixel_shader_looks_in_its_map() {
        finds(0x00e9_3060, 0x18, 0x5180);
    }

    #[test]
    fn resolve_shader_file_name_tries_each_directory() {
        let (mut e, instance) = engine();
        quiet(
            &mut e,
            &[STRCPY_S, REPORT_ERROR, SPLIT_PATH_00E983F0, NI_SPRINTF],
        );
        let file = text(&mut e, b"water.vsh");
        let path = e.mem.alloc(0x104);
        // The file itself exists: copied.
        e.register_double(NI_FILE_ACCESS, move |_, a| {
            u32::from(a[0] == file).into_ret()
        });
        assert!(e.call(0x00e9_30a0, &[file, path, 0x104]).bool());
        assert_eq!(calls(&e, STRCPY_S), vec![vec![path, 0x104, file]]);
        // Directories: an empty one, one ending in a slash, one without a separator.
        let empty = text(&mut e, b"");
        let slash = text(&mut e, b"shaders/");
        let plain = text(&mut e, b"data\\shaders");
        directory_list(&mut e, instance + 8, &[empty, slash, plain]);
        e.register(NI_FILE_ACCESS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_30a0, &[file, path, 0x104]).bool());
        let split = &calls(&e, SPLIT_PATH_00E983F0)[0];
        assert_eq!(split[0], file);
        let printed = calls(&e, NI_SPRINTF);
        assert_eq!(printed.len(), 2);
        assert_eq!(printed[0][..4], [path, 0x104, JOIN, slash]);
        assert_eq!(printed[0][4..], [split[3], split[4]]);
        assert_eq!(printed[1][2..4], [JOIN_WITH_SEPARATOR, plain]);
        assert_eq!(
            calls(&e, REPORT_ERROR),
            vec![
                vec![1, 0, INVALID_DIRECTORY],
                vec![1, 1, FILE_NOT_FOUND, file, slash],
                vec![1, 1, FILE_NOT_FOUND, file, plain],
            ]
        );
        // The second directory tried has the file.
        let tries = Rc::new(Cell::new(0u32));
        let count = tries.clone();
        e.register_double(NI_FILE_ACCESS, move |_, _| {
            count.set(count.get() + 1);
            u32::from(count.get() == 3).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_30a0, &[file, path, 0x104]).bool());
        assert_eq!(tries.get(), 3);
        // No instance: reported.
        e.set_global(FACTORY_INSTANCE, 0u32);
        e.register(NI_FILE_ACCESS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_30a0, &[file, path, 0x104]).bool());
        assert_eq!(calls(&e, REPORT_ERROR), vec![vec![1, 0, NO_VALID_FACTORY]]);
    }

    #[test]
    fn get_shader_creator_looks_up_the_lowered_extension() {
        let (mut e, _) = engine();
        e.mem.set_u32(CREATOR_MAP_SLOT, 0x5300);
        e.register(TOLOWER, |_, a| {
            u32::from((a[0] as u8).to_ascii_lowercase()).into_ret()
        });
        e.register(GET_FILE_EXTENSION, |e, a| {
            let name = e.mem.cstr(a[0]);
            let dot = name.iter().position(|c| *c == b'.').unwrap_or(name.len());
            e.mem.set_cstr(a[1], &name[dot..]);
            1u32.into_ret()
        });
        let keys = Rc::new(RefCell::new(vec![]));
        let seen = keys.clone();
        e.register_double(MAP_GET_AT, move |e, a| {
            seen.borrow_mut().push(e.mem.cstr(a[1]));
            e.mem.set_u32(a[2], 0x6600);
            u32::from(e.mem.cstr(a[1]) == b"hlsl").into_ret()
        });
        let file = text(&mut e, b"Water.HLSL");
        assert_eq!(e.call(0x00e9_32f0, &[file]).u32(), 0x6600);
        // Without an extension the whole name is the key.
        let bare = text(&mut e, b"Obj");
        assert_eq!(e.call(0x00e9_32f0, &[bare]).u32(), 0);
        assert_eq!(*keys.borrow(), vec![b"hlsl".to_vec(), b"obj".to_vec()]);
        assert_eq!(calls(&e, MAP_GET_AT)[0][0], 0x5300);
        // Null or empty names, or no map: 0 before any lookup.
        let empty = text(&mut e, b"");
        assert_eq!(e.call(0x00e9_32f0, &[empty]).u32(), 0);
        assert_eq!(e.call(0x00e9_32f0, &[0]).u32(), 0);
        e.mem.set_u32(CREATOR_MAP_SLOT, 0);
        assert_eq!(e.call(0x00e9_32f0, &[file]).u32(), 0);
        assert_eq!(calls(&e, GET_FILE_EXTENSION).len(), 2);
    }

    fn deletes(address: u32, vtables: &[u32]) {
        let (mut e, _) = engine();
        quiet(&mut e, &[NI_FREE, MEMORY_MANAGER_FREE]);
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 8, 0x7700);
        let seen = Rc::new(RefCell::new(vec![]));
        let tables = seen.clone();
        e.register_double(MAP_REMOVE_ALL, move |e, a| {
            tables.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        assert_eq!(e.call(address, &[map, 1]).u32(), map);
        assert_eq!(*seen.borrow(), vtables);
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x7700]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, map]]
        );
        e.call(address, &[map, 0]);
        assert_eq!(calls(&e, MEMORY_MANAGER_FREE).len(), 1);
    }

    #[test]
    fn vertex_map_base_deleting_destructor() {
        deletes(0x00e9_35d0, &[FIRST_MAP_BASE_VTABLE]);
    }

    #[test]
    fn vertex_map_deleting_destructor() {
        deletes(
            0x00e9_3610,
            &[VERTEX_POINTER_MAP_VTABLE, FIRST_MAP_BASE_VTABLE],
        );
    }

    #[test]
    fn pixel_map_base_deleting_destructor() {
        deletes(0x00e9_3670, &[SECOND_MAP_BASE_VTABLE]);
    }

    #[test]
    fn pixel_map_deleting_destructor() {
        deletes(
            0x00e9_36b0,
            &[PIXEL_POINTER_MAP_VTABLE, SECOND_MAP_BASE_VTABLE],
        );
    }

    #[test]
    fn creator_map_base_deleting_destructor() {
        deletes(0x00e9_3710, &[CREATOR_MAP_BASE_VTABLE]);
    }

    #[test]
    fn creator_map_deleting_destructor() {
        deletes(
            0x00e9_3750,
            &[CREATOR_POINTER_MAP_VTABLE, CREATOR_MAP_BASE_VTABLE],
        );
    }

    fn inserts(address: u32, offset: u32, map: u32) {
        let (mut e, instance) = engine();
        quiet(&mut e, &[MAP_SET_AT]);
        let shader = object(&mut e);
        e.register(SLOTS + 0x50, |_, _| 0x1234u32.into_ret());
        e.call(address, &[instance, shader]);
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![map, 0x1234, shader]]);
        e.call(address, &[instance, 0]);
        e.mem.set_u32(instance + offset, 0);
        e.call(address, &[instance, shader]);
        assert_eq!(calls(&e, MAP_SET_AT).len(), 1);
    }

    #[test]
    fn insert_vertex_shader_uses_its_name() {
        inserts(0x00e9_37f0, 0x14, 0x5140);
    }

    #[test]
    fn insert_pixel_shader_uses_its_name() {
        inserts(0x00e9_3820, 0x18, 0x5180);
    }

    /// The creator map (`0126fa14` = 0x5300) holds `creator` for every key, the shader maps
    /// hold nothing; the new shaders answer the name 0x4321.
    fn creating_engine() -> (Engine, u32, u32) {
        let (mut e, instance) = engine();
        e.mem.set_u32(CREATOR_MAP_SLOT, 0x5300);
        let creator = object(&mut e);
        e.register(TOLOWER, |_, a| (a[0] & 0xff).into_ret());
        e.register(GET_FILE_EXTENSION, |e, a| {
            e.mem.set_cstr(a[1], b".hlsl");
            1u32.into_ret()
        });
        e.register_double(MAP_GET_AT, move |e, a| {
            if a[0] == 0x5300 {
                e.mem.set_u32(a[2], creator);
                return 1u32.into_ret();
            }
            Ret::default()
        });
        quiet(&mut e, &[MAP_SET_AT]);
        let shader = object(&mut e);
        e.register(SLOTS + 0x50, |_, _| 0x4321u32.into_ret());
        e.register_double(SLOTS + 0x10, move |_, _| shader.into_ret());
        e.register_double(SLOTS + 0x18, move |_, _| shader.into_ret());
        (e, instance, creator)
    }

    #[test]
    fn create_vertex_shader_from_buffer_asks_the_creator() {
        let (mut e, _, creator) = creating_engine();
        let kind = text(&mut e, b"a.hlsl");
        let words = [0x6000, 0x40, kind, 0x7100, 0x7200, 0x7300, 0x7400, 5, 1, 1];
        let shader = e.call(0x00e9_3850, &words).u32();
        assert_eq!(
            calls(&e, SLOTS + 0x10),
            vec![vec![
                creator, 0x6000, 0x40, 0x7100, 0x7200, 0x7300, 0x7400, 5, 1, 1
            ]]
        );
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![0x5140, 0x4321, shader]]);
        // Held already: answered without the creator.
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 0x9999);
            1u32.into_ret()
        });
        assert_eq!(e.call(0x00e9_3850, &words).u32(), 0x9999);
        assert_eq!(calls(&e, SLOTS + 0x10).len(), 1);
        // No code: 0.
        let mut no_code = words;
        no_code[0] = 0;
        assert_eq!(e.call(0x00e9_3850, &no_code).u32(), 0);
        assert_eq!(calls(&e, MAP_GET_AT).len(), 3);
    }

    #[test]
    fn create_pixel_shader_from_buffer_asks_the_creator() {
        let (mut e, _, creator) = creating_engine();
        let kind = text(&mut e, b"a.hlsl");
        let words = [0x6000, 0x40, kind, 0x7100, 0x7200, 0x7300, 0];
        let shader = e.call(0x00e9_38e0, &words).u32();
        assert_eq!(
            calls(&e, SLOTS + 0x18),
            vec![vec![creator, 0x6000, 0x40, 0x7100, 0x7200, 0x7300, 0]]
        );
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![0x5180, 0x4321, shader]]);
        // No creator for the type: 0.
        e.register(MAP_GET_AT, |_, _| Ret::default());
        assert_eq!(e.call(0x00e9_38e0, &words).u32(), 0);
        assert_eq!(calls(&e, SLOTS + 0x18).len(), 1);
        // No instance: 0.
        e.set_global(FACTORY_INSTANCE, 0u32);
        assert_eq!(e.call(0x00e9_38e0, &words).u32(), 0);
    }

    /// Doubles for the directory functions: a real `_strcpy_s`, a case-insensitive
    /// `NiStricmp`, an allocator and a node pool.
    fn directory_engine() -> (Engine, u32) {
        let (mut e, instance) = engine();
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(NI_STRICMP, |e, a| {
            let left = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right = e.mem.cstr(a[1]).to_ascii_lowercase();
            u32::from(left != right).into_ret()
        });
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_NEW_NODE_0043A010, |e, _| e.mem.alloc(0xc).into_ret());
        quiet(&mut e, &[LIST_REMOVE_00E7DB70]);
        (e, instance)
    }

    #[test]
    fn add_program_directory_puts_a_new_copy_at_the_head() {
        let (mut e, instance) = directory_engine();
        let first = text(&mut e, b"data/shaders/");
        e.call(0x00e9_3960, &[instance, first]);
        let head = e.mem.u32(instance + 8);
        assert_eq!(e.mem.cstr(e.mem.u32(head + 8)), b"data\\shaders\\");
        assert_eq!(e.mem.u32(instance + 0xc), head);
        assert_eq!(e.mem.u32(instance + 0x10), 1);
        assert_eq!(
            calls(&e, LIST_NEW_NODE_0043A010),
            vec![vec![instance + 0x10]]
        );
        // A second one goes in front.
        let second = text(&mut e, b"other");
        e.call(0x00e9_3960, &[instance, second]);
        let new_head = e.mem.u32(instance + 8);
        assert_eq!(e.mem.u32(new_head), head);
        assert_eq!(e.mem.u32(head + 4), new_head);
        assert_eq!(e.mem.u32(instance + 0x10), 2);
        // The same directory in another case, an empty one and null: nothing.
        let again = text(&mut e, b"DATA\\Shaders/");
        e.call(0x00e9_3960, &[instance, again]);
        let empty = text(&mut e, b"");
        e.call(0x00e9_3960, &[instance, empty]);
        e.call(0x00e9_3960, &[instance, 0]);
        assert_eq!(e.mem.u32(instance + 0x10), 2);
        assert_eq!(calls(&e, LIST_NEW_NODE_0043A010).len(), 2);
    }

    #[test]
    fn remove_program_directory_removes_the_match() {
        let (mut e, instance) = directory_engine();
        let a = text(&mut e, b"one\\");
        let b = text(&mut e, b"two\\");
        directory_list(&mut e, instance + 8, &[a, b]);
        let held = Rc::new(Cell::new(0u32));
        let seen = held.clone();
        e.register_double(LIST_REMOVE_00E7DB70, move |e, words| {
            seen.set(e.mem.u32(words[1]));
            Ret::default()
        });
        let name = text(&mut e, b"TWO/");
        e.call(0x00e9_3bb0, &[instance, name]);
        assert_eq!(calls(&e, LIST_REMOVE_00E7DB70)[0][0], instance + 8);
        assert_eq!(held.get(), b);
        // No match, an empty or null name: nothing.
        let other = text(&mut e, b"three");
        e.call(0x00e9_3bb0, &[instance, other]);
        let empty = text(&mut e, b"");
        e.call(0x00e9_3bb0, &[instance, empty]);
        e.call(0x00e9_3bb0, &[instance, 0]);
        assert_eq!(calls(&e, LIST_REMOVE_00E7DB70).len(), 1);
    }
}
