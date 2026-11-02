//! `Havok SDK/hkpstorageextendedmeshshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d4ad20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_ac70`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4ad20(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_ac70, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d4aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_9cb0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4aec0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_9cb0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d4b120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_af60`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4b120(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_af60, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d4ad50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `00d4aec0`. `00d4aec0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d4ad50(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d4_aec0, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

use super::hkpsphereshape::{copy_vec, ZERO_F32};

// The Xbox PDB has no record of the Havok classes, so the field names below describe what the
// code does and are unconfirmed. `hkpStorageExtendedMeshShape` (the engine map names the class)
// is an `hkpExtendedMeshShape` that owns the data its subparts point at: the vtable at `+0` and
// `+0x10`, the array of triangle subparts at `+0xb4` (data) / `+0xb8` (size), 0x60 bytes each,
// the array of shape subparts at `+0xc0` / `+0xc4`, 0x40 bytes each, a 16-bit array at `+0xcc` /
// `+0xd0` / `+0xd4`, the running count of child shapes at `+0xe0`, and then the storage
// objects: an array of pointers to the triangle subparts' storage (0x68 bytes each) at `+0xf0`
// / `+0xf4` / `+0xf8` and one to the shape subparts' storage (0x2c bytes each) at `+0xfc` /
// `+0x100` / `+0x104`. Every array is an `hkArray`: data pointer, size, capacity word (the low
// 30 bits the capacity, bit 31 set when the storage is not owned).
//
// A triangle subpart's storage holds, as `hkArray`s from `+8`: the vertex positions (16 bytes
// each), the 8-bit (`+0x14`), 16-bit (`+0x20`) and 32-bit (`+0x2c`) triangle indices, the 8-bit
// (`+0x38`) and 16-bit (`+0x5c`) material indices, the vertex data (12 bytes each, `+0x44`) and
// the named vertex data (8 bytes each, `+0x50`). A shape subpart's storage holds the 8-bit
// (`+8`) and 16-bit (`+0x20`) material indices and the vertex data (12 bytes each, `+0x14`).

/// The vtables the constructors install: the shape's two, the shape-subpart storage's, the
/// triangle-subpart storage's and the base class's (`0102e388`).
const SHAPE_VTABLE: u32 = 0x010c_fd44;
const SHAPE_SECONDARY_VTABLE: u32 = 0x010c_fd20;
const SHAPES_STORAGE_VTABLE: u32 = 0x010c_fd10;
const TRIANGLES_STORAGE_VTABLE: u32 = 0x010c_fd84;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The array growth helpers: `00c90940` (cdecl: allocator, array, element size) adds room for one
/// element and `00c908c0` (cdecl: allocator, array, wanted capacity, element size) reserves.
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkReferencedObject::addReference` and `removeReference` (`this` = the object).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `memset` (`00cb1280`, cdecl: destination, byte, count).
const MEMORY_SET: u32 = 0x00cb_1280;
/// The `hkStringPtr` functions: the constructor from a C string (`00cb0b80`, `this` and the
/// string), the assignment from another `hkStringPtr` (`00cb0c30`, `this` and the source) and
/// the destructor (`00cb0c00`).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0b80;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c30;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
/// The text the name entries are built from (`010cfd04`, "Default").
const DEFAULT_NAME: u32 = 0x010c_fd04;
/// The base class (`hkpExtendedMeshShape`) functions: the constructor (`00d569c0`, `this`, a
/// `float` radius and a word), the loading constructor (`00d56f10`, `this` and the flag), the
/// destructor (`00d56e00`), the recalculation of the AABB extents (`00d55950`), the two child
/// shape counts (`00d56160` for a triangle subpart with its index, `00d55520` for a shape
/// subpart) and the functions that open one more subpart (`00d562f0` for triangles, `00d56880`
/// for shapes; each returns the new subpart).
const BASE_CONSTRUCT: u32 = 0x00d5_69c0;
const BASE_LOAD_CONSTRUCT: u32 = 0x00d5_6f10;
const BASE_DESTRUCT: u32 = 0x00d5_6e00;
const RECALC_AABB_EXTENTS: u32 = 0x00d5_5950;
const TRIANGLES_CHILD_COUNT: u32 = 0x00d5_6160;
const SHAPES_CHILD_COUNT: u32 = 0x00d5_5520;
const OPEN_TRIANGLES_SUBPART: u32 = 0x00d5_62f0;
const OPEN_SHAPES_SUBPART: u32 = 0x00d5_6880;
/// The shape subpart's own destructor (`00d56800`, `this` = the subpart) and the fix-up of the
/// shape subpart storage when loaded (`00d49c40`, `this` and the flag).
const SHAPES_SUBPART_DESTRUCT: u32 = 0x00d5_6800;
const SHAPES_STORAGE_LOADED: u32 = 0x00d4_9c40;
/// The vertex data layouts a subpart can name at `+0x10` (addresses of records in the exe's
/// data; the first has three full `float`s per vertex, the second names its vertices).
const FLOAT_VERTEX_FORMAT: u32 = 0x0126_8c8c;
const NAMED_VERTEX_FORMAT: u32 = 0x0126_9bb4;
/// `0x3f80`, the 16-bit pattern stored where a vertex of another layout has its second word.
const HALF_ONE: u16 = 0x3f80;

/// The allocator of the memory router (`+0x10` of the result of `00c85750`).
fn current_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Makes room for one more element of an `hkArray`: when the size equals the capacity the array
/// grows through `00c90940`.
fn make_room_for_one(e: &mut Engine, array: u32, element: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element]);
    }
}

/// Makes room for `count` more elements of an `hkArray`: when the capacity (signed) is below
/// size plus `count`, `00c908c0` reserves the larger of that and twice the capacity. Returns the
/// size before the call; the caller adds `count` to the size afterwards.
fn make_room_for(e: &mut Engine, array: u32, count: u32, element: u32) -> u32 {
    let allocator = current_allocator(e);
    let size = e.mem.u32(array + 4);
    let wanted = size.wrapping_add(count);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if (capacity as i32) < (wanted as i32) {
        let doubled = capacity.wrapping_add(capacity);
        let target = if (wanted as i32) < (doubled as i32) {
            doubled
        } else {
            wanted
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, target, element]);
    }
    size
}

/// Empties an `hkArray` the way the destructors do: the size is 0, the storage is freed through
/// the allocator (slot `0x10`: data, `capacity * scale` bytes) when bit 31 of the capacity word
/// is clear, the data pointer is 0 and the capacity word `0x80000000`.
fn release_storage(e: &mut Engine, allocator: u32, array: u32, scale: u32) {
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(scale);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Calls `removeReference` (`00c905b0`) on every pointer of an `hkArray` of references, the
/// size being read again after each call.
fn release_references(e: &mut Engine, array: u32) {
    if (e.mem.u32(array + 4) as i32) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let data = e.mem.u32(array);
        let element = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
        e.call(REMOVE_REFERENCE, &args![element]);
        index = index.wrapping_add(1);
        if index as i32 >= e.mem.u32(array + 4) as i32 {
            break;
        }
    }
}

// Translated from 00d49c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Number of index slots a triangle subpart's index storage needs (no engine-map name); the
/// inputs are in registers, `ECX` = `count` (the number of triangles) first, then `EAX` = `units`
/// (the source index stride in elements): `count + 2` for 1, `count * 2 + 1` for 2, `count * 4`
/// otherwise.
pub fn fn_00d49c20(_e: &mut Engine, count: u32, units: u32) -> u32 {
    match units.wrapping_sub(1) {
        0 => count.wrapping_add(2),
        1 => count.wrapping_mul(2).wrapping_add(1),
        _ => count.wrapping_mul(4),
    }
}

// Translated from 00d49cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `hkpStorageExtendedMeshShape` (no engine-map name): installs the two
/// vtables, calls `removeReference` (`00c905b0`) on every triangle-subpart storage pointer
/// (array `+0xf0`), runs the shape subparts' destructor (`00d56800`) on each of the `+0xc4`
/// subparts (0x40 bytes each, from `+0xc0`) and sets that size to 0, releases the shape-subpart
/// storage pointers (array `+0xfc`) the same way as the first array, frees the storage of both
/// pointer arrays (allocator slot `0x10`, 4 bytes per element when bit 31 of the capacity word
/// is clear) and finally runs the base class's destructor (`00d56e00`).
pub fn fn_00d49cb0(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, SHAPE_VTABLE);
    e.mem.set_u32(object + 0x10, SHAPE_SECONDARY_VTABLE);
    release_references(e, object + 0xf0);
    let count = e.mem.u32(object + 0xc4);
    let mut subpart = e.mem.u32(object + 0xc0);
    if count as i32 > 0 {
        for _ in 0..count {
            e.call(SHAPES_SUBPART_DESTRUCT, &args![subpart]);
            subpart = subpart.wrapping_add(0x40);
        }
    }
    e.mem.set_u32(object + 0xc4, 0);
    release_references(e, object + 0xfc);
    let storage_allocator = current_allocator(e);
    release_storage(e, storage_allocator, object + 0xfc, 4);
    let pointer_allocator = current_allocator(e);
    release_storage(e, pointer_allocator, object + 0xf0, 4);
    e.call(BASE_DESTRUCT, &args![this]);
}

/// Copies the material indices of a subpart `src` into the storage and points `dest` at the
/// copy; the shared part of `addTrianglesSubpart` and `addShapesSubpart`. `dest + 1` takes the
/// kind byte of `src + 1` (1 = 8-bit indices, otherwise 16-bit). A null index pointer (`src +
/// 4`) leaves a null pointer and a 0 stride in `dest`. A stride (`src + 8`) of 0 means one
/// index for the whole subpart, which is appended to `bytes` or `words` (the storage arrays) and
/// pointed at; otherwise `count` (the word at `src + count_offset`) indices are appended, read
/// with that stride, and `dest + 4` points at the first. `dest + 8` becomes the stride, but 1
/// for the 8-bit and 2 for the 16-bit kind when the stride is not 0.
fn copy_material_indices(
    e: &mut Engine,
    src: u32,
    dest: u32,
    bytes: u32,
    words: u32,
    count_offset: u32,
) {
    let kind = e.mem.u8(src + 1);
    e.mem.set_u8(dest + 1, kind);
    let material = e.mem.u32(src + 4);
    if material == 0 {
        e.mem.set_u32(dest + 4, 0);
        e.mem.set_u16(dest + 8, 0);
        return;
    }
    let (array, element) = if kind == 1 {
        (bytes, 1u32)
    } else {
        (words, 2u32)
    };
    if e.mem.u16(src + 8) == 0 {
        make_room_for_one(e, array, element);
        let data = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        let slot = data.wrapping_add(size.wrapping_mul(element));
        if kind == 1 {
            let value = e.mem.u8(material);
            e.mem.set_u8(slot, value);
        } else {
            let value = e.mem.u16(material);
            e.mem.set_u16(slot, value);
        }
        let size = size.wrapping_add(1);
        e.mem.set_u32(array + 4, size);
        let data = e.mem.u32(array);
        let first = data
            .wrapping_add(size.wrapping_mul(element))
            .wrapping_sub(element);
        e.mem.set_u32(dest + 4, first);
    } else {
        let count = e.mem.u32(src + count_offset);
        let old_size = make_room_for(e, array, count, element);
        let size = e.mem.u32(array + 4);
        e.mem.set_u32(array + 4, size.wrapping_add(count));
        let first = e
            .mem
            .u32(array)
            .wrapping_add(old_size.wrapping_mul(element));
        e.mem.set_u32(dest + 4, first);
        if count as i32 > 0 {
            let mut cursor = e.mem.u32(src + 4);
            for index in 0..count {
                if kind == 1 {
                    let value = e.mem.u8(cursor);
                    e.mem.set_u8(first.wrapping_add(index), value);
                } else {
                    let value = e.mem.u16(cursor);
                    e.mem
                        .set_u16(first.wrapping_add(index.wrapping_mul(2)), value);
                }
                cursor = cursor.wrapping_add(u32::from(e.mem.u16(src + 8)));
            }
        }
    }
    let stride = e.mem.u16(src + 8);
    e.mem.set_u16(dest + 8, stride);
    if stride != 0 {
        match e.mem.u8(dest + 1) as i8 {
            1 => e.mem.set_u16(dest + 8, 1),
            2 => e.mem.set_u16(dest + 8, 2),
            _ => {}
        }
    }
}

/// Copies the vertex data of a subpart `src` (12 bytes per vertex) into the `storage` array and
/// points `dest` at it; the shared part of both `add...Subpart` functions. `src + 0xa` vertices
/// are read from `src + 0xc` with the signed stride at `src + 2`: the first word is copied; the
/// other two words too when `src + 0x10` names the float layout, else the 16-bit values 0 (at
/// `+4`) and `0x3f80` (at `+6`) are stored. `dest + 0xc` points at the storage, `dest + 2`
/// becomes 12 and `dest + 0xa` the vertex count, or 0 and 1 when the stride is 0.
fn copy_vertex_data(e: &mut Engine, src: u32, dest: u32, storage: u32) {
    let count = u32::from(e.mem.u16(src + 0xa));
    let old_size = make_room_for(e, storage, count, 0xc);
    let size = e.mem.u32(storage + 4);
    e.mem.set_u32(storage + 4, size.wrapping_add(count));
    let mut out = e.mem.u32(storage).wrapping_add(old_size.wrapping_mul(0xc));
    let mut cursor = e.mem.u32(src + 0xc);
    for _ in 0..count {
        let first = e.mem.u32(cursor);
        e.mem.set_u32(out, first);
        if e.mem.u32(src + 0x10) == FLOAT_VERTEX_FORMAT {
            let second = e.mem.u32(cursor + 4);
            e.mem.set_u32(out + 4, second);
            let third = e.mem.u32(cursor + 8);
            e.mem.set_u32(out + 8, third);
        } else {
            e.mem.set_u16(out + 6, HALF_ONE);
            e.mem.set_u16(out + 4, 0);
        }
        cursor = cursor.wrapping_add(i32::from(e.mem.u16(src + 2) as i16) as u32);
        out = out.wrapping_add(0xc);
    }
    let data = e.mem.u32(storage);
    e.mem.set_u32(dest + 0xc, data);
    if e.mem.u16(src + 2) != 0 {
        e.mem.set_u16(dest + 2, 0xc);
        let vertices = e.mem.u16(src + 0xa);
        e.mem.set_u16(dest + 0xa, vertices);
    } else {
        e.mem.set_u16(dest + 2, 0);
        e.mem.set_u16(dest + 0xa, 1);
    }
}

// Translated from 00d49dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStorageExtendedMeshShape::addShapesSubpart` (engine map), `(this; subpart)`: opens one
/// more shape subpart (`00d56880`, called with `this`), takes 0x2c bytes from the memory
/// router's allocator (slot 4) for its storage (the word at `+4` the size, the word at `+6` the
/// count 1, three empty arrays from `+8`, `+0x14` and `+0x20`, the vtable `010cfd10`), appends it
/// to the storage array at `+0xfc` (growing through `00c90940`), lets `00d4ad60` copy the
/// common part of `subpart` into the new one and then moves the material indices
/// ([`copy_material_indices`], the count at `subpart + 0x18`) and, when the new subpart has a
/// material pointer, the vertex data ([`copy_vertex_data`]) into the storage. Finally the AABB
/// extents are recalculated (`00d55950`), the new subpart's child count (`00d55520`) is added to
/// the word at `+0xe0` and the index of the new subpart (the size at `+0xc4` less one) returned.
pub fn hkp_storage_extended_mesh_shape_add_shapes_subpart(
    e: &mut Engine,
    this: Ptr,
    subpart: Ptr,
) -> u32 {
    let object = this.addr();
    let src = subpart.addr();
    let dest = e.call(OPEN_SHAPES_SUBPART, &args![this]).u32();
    let allocator = current_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x2cu32]).u32();
    e.mem.set_u32(block, SHAPES_STORAGE_VTABLE);
    e.mem.set_u16(block + 4, 0x2c);
    e.mem.set_u16(block + 6, 1);
    for (data, size, capacity) in [(8, 0xc, 0x10), (0x14, 0x18, 0x1c), (0x20, 0x24, 0x28)] {
        e.mem.set_u32(block + data, 0);
        e.mem.set_u32(block + size, 0);
        e.mem.set_u32(block + capacity, 0x8000_0000);
    }
    let pointers = object + 0xfc;
    make_room_for_one(e, pointers, 4);
    let size = e.mem.u32(pointers + 4);
    let data = e.mem.u32(pointers);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), block);
    e.mem.set_u32(pointers + 4, size.wrapping_add(1));
    let size = e.mem.u32(pointers + 4);
    let data = e.mem.u32(pointers);
    let storage = e
        .mem
        .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
    e.call(0x00d4_ad60, &args![dest, src]);
    copy_material_indices(e, src, dest, storage + 8, storage + 0x20, 0x18);
    if e.mem.u32(dest + 4) != 0 {
        copy_vertex_data(e, src, dest, storage + 0x14);
    }
    e.call(RECALC_AABB_EXTENTS, &args![this]);
    // 00d4a149: `00d55520` is stdcall (RET 4) and does not read the ECX (this) the caller
    // sets.
    let children = e.call(SHAPES_CHILD_COUNT, &args![dest]).u32();
    let total = e.mem.u32(object + 0xe0).wrapping_add(children);
    e.mem.set_u32(object + 0xe0, total);
    e.mem.u32(object + 0xc4).wrapping_sub(1)
}

// Translated from 00d4a170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the vtable of a triangle subpart's storage (`010cfd84`; no engine-map name; `this` and
/// one stack word that is not used). Returns `this`.
pub fn fn_00d4a170(e: &mut Engine, this: Ptr, _flag: u32) -> Ptr {
    e.mem.set_u32(this.addr(), TRIANGLES_STORAGE_VTABLE);
    this
}

// Translated from 00d4a180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor for a loaded object (no engine-map name), `(this; flag)`: runs the base class's
/// loading constructor (`00d56f10`, `this` and `flag`) and installs the two vtables. With `flag`
/// 0 nothing else happens. Otherwise each triangle subpart (0x60 bytes from `+0xb4`, count at
/// `+0xb8`) is paired with its storage (the pointer array at `+0xf0`): when the storage pointer
/// is not null `00d4a170` sets its vtable, then the subpart's `+0x18` takes the storage's
/// vertex positions (`+8`), `+0x40` the triangle indices of the kind in `subpart + 0x48` (1:
/// storage `+0x14`, 2: `+0x20`, else `+0x2c`), `+4` the material indices (storage `+0x38` when
/// `subpart + 1` is 1, else `+0x5c`) and `+0xc` the named vertex data (storage `+0x50`) when
/// that array is not empty, else the vertex data (`+0x44`). Each shape subpart (0x40 bytes from
/// `+0xc0`, count at `+0xc4`) is paired with its storage the same way (`00d49c40` for a non-null
/// pointer): `+4` takes the material indices (storage `+8` when `subpart + 1` is 1, else
/// `+0x20`) and `+0xc` the vertex data (`+0x14`). Returns `this`.
pub fn fn_00d4a180(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let object = this.addr();
    e.call(BASE_LOAD_CONSTRUCT, &args![this, flag]);
    e.mem.set_u32(object, SHAPE_VTABLE);
    e.mem.set_u32(object + 0x10, SHAPE_SECONDARY_VTABLE);
    if flag == 0 {
        return this;
    }
    if e.mem.u32(object + 0xb8) as i32 > 0 {
        let mut index = 0u32;
        let mut offset = 0u32;
        loop {
            let storage = e
                .mem
                .u32(e.mem.u32(object + 0xf0).wrapping_add(index.wrapping_mul(4)));
            let subpart = e.mem.u32(object + 0xb4).wrapping_add(offset);
            if storage != 0 {
                e.call(0x00d4_a170, &args![storage, flag]);
            }
            let storage = e
                .mem
                .u32(e.mem.u32(object + 0xf0).wrapping_add(index.wrapping_mul(4)));
            let positions = e.mem.u32(storage + 8);
            e.mem.set_u32(subpart + 0x18, positions);
            let indices = match e.mem.u8(subpart + 0x48) {
                1 => e.mem.u32(storage + 0x14),
                2 => e.mem.u32(storage + 0x20),
                _ => e.mem.u32(storage + 0x2c),
            };
            e.mem.set_u32(subpart + 0x40, indices);
            let materials = if e.mem.u8(subpart + 1) == 1 {
                e.mem.u32(storage + 0x38)
            } else {
                e.mem.u32(storage + 0x5c)
            };
            e.mem.set_u32(subpart + 4, materials);
            let vertices = if e.mem.u32(storage + 0x54) != 0 {
                e.mem.u32(storage + 0x50)
            } else {
                e.mem.u32(storage + 0x44)
            };
            e.mem.set_u32(subpart + 0xc, vertices);
            index = index.wrapping_add(1);
            offset = offset.wrapping_add(0x60);
            if index as i32 >= e.mem.u32(object + 0xb8) as i32 {
                break;
            }
        }
    }
    if e.mem.u32(object + 0xc4) as i32 > 0 {
        let mut index = 0u32;
        let mut offset = 0u32;
        loop {
            let storage = e
                .mem
                .u32(e.mem.u32(object + 0xfc).wrapping_add(index.wrapping_mul(4)));
            let subpart = e.mem.u32(object + 0xc0).wrapping_add(offset);
            if storage != 0 {
                e.call(SHAPES_STORAGE_LOADED, &args![storage, flag]);
            }
            let storage = e
                .mem
                .u32(e.mem.u32(object + 0xfc).wrapping_add(index.wrapping_mul(4)));
            let materials = if e.mem.u8(subpart + 1) == 1 {
                e.mem.u32(storage + 8)
            } else {
                e.mem.u32(storage + 0x20)
            };
            e.mem.set_u32(subpart + 4, materials);
            let vertices = e.mem.u32(storage + 0x14);
            e.mem.set_u32(subpart + 0xc, vertices);
            index = index.wrapping_add(1);
            offset = offset.wrapping_add(0x40);
            if index as i32 >= e.mem.u32(object + 0xc4) as i32 {
                break;
            }
        }
    }
    this
}

/// Appends the triangle indices of the source subpart `src` to the storage array `array` of the
/// kind in `src + 0x48` (1: 8-bit, 2: 16-bit, else 32-bit) and returns the address of the
/// appended block, with `dest + 0x44` set to the destination stride. The source stride
/// (`src + 0x44`, in bytes) is divided by the element size and limited to 4 elements when it
/// is above 2 (signed); the block holds `00d49c20(src + 0x14, source stride in elements)`
/// elements, set to 0 through `memset`, and for each of the `src + 0x14` triangles the three
/// elements are copied, the destination advancing by `dest + 0x44` bytes and the source by
/// `src + 0x44` bytes.
fn copy_triangle_indices(e: &mut Engine, src: u32, dest: u32, storage: u32) -> u32 {
    let kind = e.mem.u8(src + 0x48);
    let (shift, array): (u32, u32) = match kind {
        1 => (0, storage + 0x14),
        2 => (1, storage + 0x20),
        _ => (2, storage + 0x2c),
    };
    let element = 1u32 << shift;
    let units = e.mem.u32(src + 0x44) >> shift;
    let clamped = if units as i32 > 2 { 4 } else { units };
    e.mem.set_u32(dest + 0x44, clamped << shift);
    let triangles = e.mem.u32(src + 0x14);
    let slots = e.call(0x00d4_9c20, &args![triangles, units]).u32();
    let old_size = make_room_for(e, array, slots, element);
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(slots));
    let start = e
        .mem
        .u32(array)
        .wrapping_add(old_size.wrapping_mul(element));
    e.call(MEMORY_SET, &args![start, 0u32, slots.wrapping_mul(element)]);
    let mut from = e.mem.u32(src + 0x40);
    let mut to = start;
    if e.mem.u32(src + 0x14) as i32 > 0 {
        for _ in 0..e.mem.u32(src + 0x14) {
            for lane in 0..3 {
                match element {
                    1 => {
                        let value = e.mem.u8(from + lane);
                        e.mem.set_u8(to + lane, value);
                    }
                    2 => {
                        let value = e.mem.u16(from + 2 * lane);
                        e.mem.set_u16(to + 2 * lane, value);
                    }
                    _ => {
                        let value = e.mem.u32(from + 4 * lane);
                        e.mem.set_u32(to + 4 * lane, value);
                    }
                }
            }
            to = to.wrapping_add(e.mem.u32(dest + 0x44));
            from = from.wrapping_add(e.mem.u32(src + 0x44));
        }
    }
    start
}

// Translated from 00d4a2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStorageExtendedMeshShape::addTrianglesSubpart` (engine map), `(this; subpart)`: opens
/// one more triangle subpart (`00d562f0`, called with `this`; it returns the new subpart `dest`),
/// copies the header of `subpart` into it (`00d4abf0`), takes 0x68 bytes from the memory router's
/// allocator (slot 4) for its storage (the word at `+4` the size; `00d4aef0` builds it) and
/// appends it to the storage array at `+0xf0` (growing through `00c90940`). Into the storage the
/// following are copied: the vertex positions (the `subpart + 0x20` positions from
/// `subpart + 0x18` with the stride at `+0x1c`, each three words and the `float` at `01011d78`
/// as the fourth lane; `dest + 0x18` takes the storage, `+0x1c` 16 and `+0x20` the count), the
/// triangle indices ([`copy_triangle_indices`]; `dest + 0x40` points at them and `dest + 0x48`,
/// `+0x14` and `+0x49` take the kind, the triangle count and the byte of `subpart`), the
/// material indices ([`copy_material_indices`], the count at `subpart + 0x14`) and, when `dest`
/// has a material pointer, the vertex data: when `subpart + 0x10` is the named layout `01269bb4` the
/// named entries (8 bytes each: a word and an `hkStringPtr` built from "Default" and assigned
/// from the source; `dest + 2` becomes 8 and `+0xa` the count, or 0 and 1 for stride 0), else
/// the 12-byte vertices ([`copy_vertex_data`]). Finally the AABB extents are recalculated
/// (`00d55950`) and the child count of the new triangle subpart (`00d56160`, `dest` and the
/// size at `+0xb8` less one) is added to the word at `+0xe0`.
pub fn hkp_storage_extended_mesh_shape_add_triangles_subpart(
    e: &mut Engine,
    this: Ptr,
    subpart: Ptr,
) {
    let object = this.addr();
    let src = subpart.addr();
    let dest = e.call(OPEN_TRIANGLES_SUBPART, &args![this]).u32();
    e.call(0x00d4_abf0, &args![dest, src]);
    let allocator = current_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x68u32]).u32();
    e.mem.set_u16(block + 4, 0x68);
    let storage = e.call(0x00d4_aef0, &args![block]).u32();
    let pointers = object + 0xf0;
    make_room_for_one(e, pointers, 4);
    let size = e.mem.u32(pointers + 4);
    let data = e.mem.u32(pointers);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), storage);
    e.mem.set_u32(pointers + 4, size.wrapping_add(1));
    let size = e.mem.u32(pointers + 4);
    let data = e.mem.u32(pointers);
    let storage = e
        .mem
        .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));

    // The vertex positions: three words of the source and the float at `01011d78`.
    let positions = e.mem.u32(src + 0x20);
    let array = storage + 8;
    let old_size = make_room_for(e, array, positions, 0x10);
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(positions));
    let mut cursor = e.mem.u32(src + 0x18);
    let mut out = old_size.wrapping_shl(4).wrapping_add(e.mem.u32(array));
    if positions as i32 > 0 {
        let fourth = e.mem.u32(ZERO_F32);
        for _ in 0..positions {
            for (lane, word) in [
                e.mem.u32(cursor),
                e.mem.u32(cursor + 4),
                e.mem.u32(cursor + 8),
                fourth,
            ]
            .into_iter()
            .enumerate()
            {
                e.mem.set_u32(out + 4 * lane as u32, word);
            }
            out = out.wrapping_add(0x10);
            cursor = cursor.wrapping_add(e.mem.u32(src + 0x1c));
        }
    }
    let data = e.mem.u32(array);
    e.mem.set_u32(dest + 0x18, data);
    e.mem.set_u32(dest + 0x1c, 0x10);
    let count = e.mem.u32(src + 0x20);
    e.mem.set_u32(dest + 0x20, count);

    let start = copy_triangle_indices(e, src, dest, storage);
    let kind = e.mem.u8(src + 0x48);
    e.mem.set_u8(dest + 0x48, kind);
    let triangles = e.mem.u32(src + 0x14);
    e.mem.set_u32(dest + 0x14, triangles);
    let flag = e.mem.u8(src + 0x49);
    e.mem.set_u8(dest + 0x49, flag);
    e.mem.set_u32(dest + 0x40, start);
    copy_material_indices(e, src, dest, storage + 0x38, storage + 0x5c, 0x14);

    if e.mem.u32(dest + 4) != 0 {
        if e.mem.u32(src + 0x10) == NAMED_VERTEX_FORMAT {
            copy_named_vertices(e, src, dest, storage);
        } else {
            copy_vertex_data(e, src, dest, storage + 0x44);
        }
    }
    e.call(RECALC_AABB_EXTENTS, &args![this]);
    let last = e.mem.u32(object + 0xb8).wrapping_sub(1);
    let children = e
        .call(TRIANGLES_CHILD_COUNT, &args![this, dest, last])
        .u32();
    let total = e.mem.u32(object + 0xe0).wrapping_add(children);
    e.mem.set_u32(object + 0xe0, total);
}

/// The named vertex entries of `addTrianglesSubpart`: `src + 0xa` entries of 8 bytes (a word
/// and an `hkStringPtr`) are appended to the storage array at `storage + 0x50`. Each new entry
/// is first built as a "Default" string (`00cb0b80`, on the entry's `+4`) with its word 0,
/// the entries are then built again, their word taken from the source and the string assigned
/// from the source's (`00cb0c30`); the source advances by the signed stride at `src + 2`.
/// `dest + 0xc` points at the storage, `dest + 2` becomes 8 and `dest + 0xa` the count, or 0
/// and 1 when the stride is 0. A null entry pointer skips the construction but, as in the
/// exe, not the stores through it (it cannot occur: the array was reserved).
fn copy_named_vertices(e: &mut Engine, src: u32, dest: u32, storage: u32) {
    let count = u32::from(e.mem.u16(src + 0xa));
    let array = storage + 0x50;
    let old_size = make_room_for(e, array, count, 8);
    let first_new = e
        .mem
        .u32(array)
        .wrapping_add(e.mem.u32(array + 4).wrapping_mul(8));
    if count as i32 > 0 {
        let mut entry = first_new;
        for _ in 0..count {
            if entry != 0 {
                e.call(STRING_PTR_CONSTRUCT, &args![entry + 4, DEFAULT_NAME]);
                e.mem.set_u32(entry, 0);
            }
            entry = entry.wrapping_add(8);
        }
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(count));
    let mut entry = e.mem.u32(array).wrapping_add(old_size.wrapping_mul(8));
    let mut cursor = e.mem.u32(src + 0xc);
    if 0 < e.mem.u16(src + 0xa) {
        let mut done = 0u32;
        loop {
            let target = if entry != 0 {
                e.call(STRING_PTR_CONSTRUCT, &args![entry + 4, DEFAULT_NAME]);
                e.mem.set_u32(entry, 0);
                entry
            } else {
                0
            };
            let word = e.mem.u32(cursor);
            e.mem.set_u32(target, word);
            e.call(STRING_PTR_ASSIGN, &args![target + 4, cursor + 4]);
            cursor = cursor.wrapping_add(i32::from(e.mem.u16(src + 2) as i16) as u32);
            done = done.wrapping_add(1);
            entry = entry.wrapping_add(8);
            if done as i32 >= i32::from(e.mem.u16(src + 0xa)) {
                break;
            }
        }
    }
    let data = e.mem.u32(array);
    e.mem.set_u32(dest + 0xc, data);
    if e.mem.u16(src + 2) != 0 {
        e.mem.set_u16(dest + 2, 8);
        let vertices = e.mem.u16(src + 0xa);
        e.mem.set_u16(dest + 0xa, vertices);
    } else {
        e.mem.set_u16(dest + 2, 0);
        e.mem.set_u16(dest + 0xa, 1);
    }
}

// Translated from 00d4aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStorageExtendedMeshShape::hkpStorageExtendedMeshShape` (engine map), the copy
/// constructor, `(this; source)`: runs the base class's constructor (`00d569c0`) with the
/// `float` at `source + 0xe4` (an x87 copy) and the word at `source + 0xb0`, installs the two
/// vtables, empties the two storage pointer arrays (`+0xf0`, `+0xfc`), copies the 16 bytes at
/// `+0x80`, the word at `+8` and the byte at `+0x14`, adds every triangle subpart of `source`
/// (0x60 bytes from `source + 0xb4`, count at `+0xb8`) through `addTrianglesSubpart` (`00d4a2b0`)
/// and every shape subpart (0x40 bytes from `+0xc0`, count at `+0xc4`) through `addShapesSubpart`
/// (`00d49dc0`). The 16-bit array at `+0xcc` / `+0xd0` / `+0xd4` is then made as large as the
/// source's: when its capacity is below the source's size the storage is freed (allocator slot
/// `0x10`, when owned) and a new block taken (slot `0xc`, which takes a pointer to the byte count
/// and rounds it; the capacity word becomes half the returned count); the size and the 16-bit
/// values are copied and the byte at `+0xd8`. The AABB extents are recalculated (`00d55950`).
/// Returns `this`.
pub fn hkp_storage_extended_mesh_shape_construct_copy(
    e: &mut Engine,
    this: Ptr,
    source: Ptr,
) -> Ptr {
    let object = this.addr();
    let src = source.addr();
    let radius = x87_quiet_bits(e.mem.u32(src + 0xe4));
    let extra = e.mem.u32(src + 0xb0);
    e.call(BASE_CONSTRUCT, &args![this, radius, extra]);
    e.mem.set_u32(object, SHAPE_VTABLE);
    e.mem.set_u32(object + 0x10, SHAPE_SECONDARY_VTABLE);
    for array in [object + 0xf0, object + 0xfc] {
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
    }
    copy_vec(e, src + 0x80, object + 0x80);
    let word = e.mem.u32(src + 8);
    e.mem.set_u32(object + 8, word);
    let byte = e.mem.u8(src + 0x14);
    e.mem.set_u8(object + 0x14, byte);
    if e.mem.u32(src + 0xb8) as i32 > 0 {
        let mut index = 0u32;
        let mut offset = 0u32;
        loop {
            let triangles = e.mem.u32(src + 0xb4).wrapping_add(offset);
            e.call(0x00d4_a2b0, &args![this, triangles]);
            offset = offset.wrapping_add(0x60);
            index = index.wrapping_add(1);
            if index as i32 >= e.mem.u32(src + 0xb8) as i32 {
                break;
            }
        }
    }
    if e.mem.u32(src + 0xc4) as i32 > 0 {
        let mut index = 0u32;
        let mut offset = 0u32;
        loop {
            let shapes = e.mem.u32(src + 0xc0).wrapping_add(offset);
            e.call(0x00d4_9dc0, &args![this, shapes]);
            offset = offset.wrapping_add(0x40);
            index = index.wrapping_add(1);
            if index as i32 >= e.mem.u32(src + 0xc4) as i32 {
                break;
            }
        }
    }
    let allocator = current_allocator(e);
    let capacity_word = e.mem.u32(object + 0xd4);
    let capacity = capacity_word & 0x3fff_ffff;
    if (capacity as i32) < e.mem.u32(src + 0xd0) as i32 {
        if capacity_word as i32 >= 0 {
            let data = e.mem.u32(object + 0xcc);
            e.vcall(
                allocator,
                0x10,
                &args![data, capacity.wrapping_add(capacity)],
            );
        }
        let wanted = e.mem.u32(src + 0xd0).wrapping_add(e.mem.u32(src + 0xd0));
        let (block, granted) = e.with_stack(4, |e, request| {
            let request = request.addr();
            e.mem.set_u32(request, wanted);
            let block = e.vcall(allocator, 0xc, &args![request]).u32();
            (block, e.mem.u32(request))
        });
        e.mem.set_u32(object + 0xcc, block);
        e.mem.set_u32(object + 0xd4, ((granted as i32) / 2) as u32);
    }
    let size = e.mem.u32(src + 0xd0);
    let data = e.mem.u32(object + 0xcc);
    e.mem.set_u32(object + 0xd0, size);
    if size as i32 > 0 {
        let from = e.mem.u32(src + 0xcc);
        for index in 0..size {
            let value = e.mem.u16(from.wrapping_add(2 * index));
            e.mem.set_u16(data.wrapping_add(2 * index), value);
        }
    }
    let byte = e.mem.u8(src + 0xd8);
    e.mem.set_u8(object + 0xd8, byte);
    e.call(RECALC_AABB_EXTENTS, &args![this]);
    this
}

// Translated from 00d4abf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the header of a triangle subpart (no engine-map name), `(this; source)`: the words at
/// `+0` to `+0x20` (nine words), the 16 bytes at `+0x30`, the words at `+0x40` and
/// `+0x44`, the bytes at `+0x48` and `+0x49` and the words at `+0x4c` and `+0x50`; the words at
/// `+0x24` to `+0x2c` are not touched. Returns `this`.
pub fn fn_00d4abf0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (dest, src) = (this.addr(), source.addr());
    for offset in [
        0u32, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x40, 0x44, 0x4c, 0x50,
    ] {
        let word = e.mem.u32(src + offset);
        e.mem.set_u32(dest + offset, word);
    }
    copy_vec(e, src + 0x30, dest + 0x30);
    for offset in [0x48, 0x49] {
        let byte = e.mem.u8(src + offset);
        e.mem.set_u8(dest + offset, byte);
    }
    this
}

// Translated from 00d4ac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of a shape subpart's storage (no engine-map name; the 0x2c-byte object):
/// sets the vtable `010cfd10`, then empties its three arrays in turn (`+0x20` of 2-byte
/// elements, `+0x14` of 12-byte elements and `+8` of bytes; [`release_storage`]) and installs
/// the base class's vtable (`0102e388`).
pub fn fn_00d4ac70(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, SHAPES_STORAGE_VTABLE);
    for (array, scale) in [(0x20, 2), (0x14, 12), (8, 1)] {
        let allocator = current_allocator(e);
        release_storage(e, allocator, object + array, scale);
    }
    e.mem.set_u32(object, BASE_VTABLE);
}

// Translated from 00d4ad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the common part of a shape subpart (no engine-map name), `(this; source)`: the five
/// words at `+0` to `+0x10`, then the array of references at `+0x14` / `+0x18` / `+0x1c`
/// is made equal to the source's: the array is reserved to the source's size when its capacity
/// (signed) is lower (`00c908c0` with the larger of that size and twice the capacity, element
/// size 4); entries beyond the source's size are released (`removeReference`, `00c905b0`) and
/// cleared; the entries both have get the source's reference added (`addReference`, `00c90510`),
/// the old one released and the pointer stored; the entries only the source has are added and
/// stored; the size is set to the source's. The 16-byte values at `+0x20` and `+0x30` are
/// copied. Returns `this`.
pub fn fn_00d4ad60(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (dest, src) = (this.addr(), source.addr());
    for offset in [0u32, 4, 8, 0xc, 0x10] {
        let word = e.mem.u32(src + offset);
        e.mem.set_u32(dest + offset, word);
    }
    let array = dest + 0x14;
    let allocator = current_allocator(e);
    let dest_size = e.mem.u32(array + 4) as i32;
    let source_count = e.mem.u32(src + 0x18) as i32;
    let common = if source_count > dest_size {
        dest_size
    } else {
        source_count
    };
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < source_count {
        let doubled = capacity.wrapping_add(capacity);
        let target = if source_count < doubled {
            doubled
        } else {
            source_count
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, target as u32, 4u32]);
    }
    let tail = e
        .mem
        .u32(array)
        .wrapping_add((source_count as u32).wrapping_mul(4));
    let surplus = dest_size.wrapping_sub(source_count);
    if surplus > 0 {
        for index in 0..surplus as u32 {
            let slot = tail.wrapping_add(index.wrapping_mul(4));
            let element = e.mem.u32(slot);
            if element != 0 {
                e.call(REMOVE_REFERENCE, &args![element]);
            }
            e.mem.set_u32(slot, 0);
        }
    }
    let from = e.mem.u32(src + 0x14);
    let to = e.mem.u32(dest + 0x14);
    if common > 0 {
        for index in 0..common as u32 {
            let source_slot = from.wrapping_add(index.wrapping_mul(4));
            let dest_slot = to.wrapping_add(index.wrapping_mul(4));
            let element = e.mem.u32(source_slot);
            if element != 0 {
                e.call(ADD_REFERENCE, &args![element]);
            }
            let old = e.mem.u32(dest_slot);
            if old != 0 {
                e.call(REMOVE_REFERENCE, &args![old]);
            }
            let element = e.mem.u32(source_slot);
            e.mem.set_u32(dest_slot, element);
        }
    }
    let from = e
        .mem
        .u32(src + 0x14)
        .wrapping_add((common as u32).wrapping_mul(4));
    let to = e
        .mem
        .u32(dest + 0x14)
        .wrapping_add((common as u32).wrapping_mul(4));
    let rest = source_count.wrapping_sub(common);
    if rest > 0 {
        for index in 0..rest as u32 {
            let source_slot = from.wrapping_add(index.wrapping_mul(4));
            let dest_slot = to.wrapping_add(index.wrapping_mul(4));
            if dest_slot != 0 {
                let element = e.mem.u32(source_slot);
                if element != 0 {
                    e.call(ADD_REFERENCE, &args![element]);
                }
                let element = e.mem.u32(source_slot);
                e.mem.set_u32(dest_slot, element);
            }
        }
    }
    e.mem.set_u32(dest + 0x18, source_count as u32);
    copy_vec(e, src + 0x20, dest + 0x20);
    copy_vec(e, src + 0x30, dest + 0x30);
    this
}

// Translated from 00d4aef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a triangle subpart's storage (no engine-map name; the 0x68-byte object): the
/// vtable `010cfd84`, the 16-bit word at `+6` set to 1 and the eight `hkArray`s from `+8`,
/// `+0x14`, `+0x20`, `+0x2c`, `+0x38`, `+0x44`, `+0x50` and `+0x5c` made empty (data 0, size 0,
/// capacity word `0x80000000`). The word at `+4` is left alone. Returns `this`.
pub fn fn_00d4aef0(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    e.mem.set_u32(object, TRIANGLES_STORAGE_VTABLE);
    e.mem.set_u16(object + 6, 1);
    for array in [8, 0x14, 0x20, 0x2c, 0x38, 0x44, 0x50, 0x5c] {
        e.mem.set_u32(object + array, 0);
        e.mem.set_u32(object + array + 4, 0);
        e.mem.set_u32(object + array + 8, 0x8000_0000);
    }
    this
}

// Translated from 00d4af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of a triangle subpart's storage (no engine-map name; the 0x68-byte object):
/// sets the vtable `010cfd84`, then empties its arrays in turn ([`release_storage`]): `+0x5c`
/// (2-byte elements), `+0x50` (8-byte entries, an `hkStringPtr` destructor `00cb0c00` first on
/// the `+4` of each), `+0x44` (12 bytes), `+0x38` (1), `+0x2c` (4), `+0x20` (2), `+0x14` (1)
/// and `+8` (16), and installs the base class's vtable (`0102e388`).
pub fn fn_00d4af60(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, TRIANGLES_STORAGE_VTABLE);
    for (array, scale) in [
        (0x5c, 2),
        (0x50, 8),
        (0x44, 12),
        (0x38, 1),
        (0x2c, 4),
        (0x20, 2),
        (0x14, 1),
        (8, 16),
    ] {
        let allocator = current_allocator(e);
        if array == 0x50 {
            let count = e.mem.u32(object + 0x54);
            let data = e.mem.u32(object + 0x50);
            if count as i32 > 0 {
                let mut name = data.wrapping_add(4);
                for _ in 0..count {
                    e.call(STRING_PTR_DESTRUCT, &args![name]);
                    name = name.wrapping_add(8);
                }
            }
        }
        release_storage(e, allocator, object + array, scale);
    }
    e.mem.set_u32(object, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_ad20, deleting_destructor_00d4ad20(Ptr, u8) -> Ptr),
        entry!(0x00d4_aec0, deleting_destructor_00d4aec0(Ptr, u8) -> Ptr),
        entry!(0x00d4_b120, deleting_destructor_00d4b120(Ptr, u8) -> Ptr),
        entry!(0x00d4_ad50, fn_00d4ad50(Ptr, u32) -> u32),
        entry!(0x00d4_9c20, fn_00d49c20(u32, u32) -> u32),
        entry!(0x00d4_9cb0, fn_00d49cb0(Ptr)),
        entry!(
            0x00d4_9dc0,
            hkp_storage_extended_mesh_shape_add_shapes_subpart(Ptr, Ptr) -> u32
        ),
        entry!(0x00d4_a170, fn_00d4a170(Ptr, u32) -> Ptr),
        entry!(0x00d4_a180, fn_00d4a180(Ptr, u32) -> Ptr),
        entry!(
            0x00d4_a2b0,
            hkp_storage_extended_mesh_shape_add_triangles_subpart(Ptr, Ptr)
        ),
        entry!(
            0x00d4_aa70,
            hkp_storage_extended_mesh_shape_construct_copy(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00d4_abf0, fn_00d4abf0(Ptr, Ptr) -> Ptr),
        entry!(0x00d4_ac70, fn_00d4ac70(Ptr)),
        entry!(0x00d4_ad60, fn_00d4ad60(Ptr, Ptr) -> Ptr),
        entry!(0x00d4_aef0, fn_00d4aef0(Ptr) -> Ptr),
        entry!(0x00d4_af60, fn_00d4af60(Ptr)),
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
    fn deleting_destructor_00d4ad20_frees_only_when_bit_zero_is_set() {
        check(0x00d4_ad20, 0x00d4_ac70);
    }

    #[test]
    fn deleting_destructor_00d4aec0_frees_only_when_bit_zero_is_set() {
        check(0x00d4_aec0, 0x00d4_9cb0);
    }

    #[test]
    fn deleting_destructor_00d4b120_frees_only_when_bit_zero_is_set() {
        check(0x00d4_b120, 0x00d4_af60);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00d4ad50() {
        assert_eq!(
            check(0x00d4_ad50, 0x00d4_aec0, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod storage_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALLOCATE_DOUBLE: u32 = 0x0900_0021;
    const ALLOCATE_SIZED_DOUBLE: u32 = 0x0900_0022;
    const FREE_DOUBLE: u32 = 0x0900_0023;
    const LOADED_PATTERN: u32 = 0x1234_5678;

    /// Grows an `hkArray` to `capacity` elements of `element` bytes: a new block with the old
    /// elements, a capacity word without flag bits.
    fn grow(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let size = e.mem.u32(array + 4);
        let old = e.mem.u32(array);
        let block = e.mem.alloc(capacity.wrapping_mul(element).max(4));
        if size > 0 {
            let bytes = e.mem.bytes(old, size * element);
            e.mem.write(block, &bytes);
        }
        e.mem.set_u32(array, block);
        e.mem.set_u32(array + 8, capacity);
    }

    /// An engine with the data range mapped, a memory router whose allocator (slot 4: a block,
    /// slot 0xc: a block 8 bytes larger than asked with the size updated, slot 0x10: free) and
    /// the leaf callees log `(address, arguments)`, and array growth doubles that really grow
    /// the array. Returns the allocator's address too.
    fn setup() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(ZERO_F32, LOADED_PATTERN);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 0xc, ALLOCATE_SIZED_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [
            FREE_DOUBLE,
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            STRING_PTR_CONSTRUCT,
            STRING_PTR_ASSIGN,
            STRING_PTR_DESTRUCT,
            RECALC_AABB_EXTENTS,
            BASE_DESTRUCT,
            SHAPES_SUBPART_DESTRUCT,
            SHAPES_STORAGE_LOADED,
            BASE_LOAD_CONSTRUCT,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let sink = Rc::clone(&log);
        e.register_double(ALLOCATE_DOUBLE, move |e, a| {
            sink.borrow_mut().push((ALLOCATE_DOUBLE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(ALLOCATE_SIZED_DOUBLE, move |e, a| {
            sink.borrow_mut().push((ALLOCATE_SIZED_DOUBLE, a.to_vec()));
            let granted = e.mem.u32(a[1]) + 8;
            e.mem.set_u32(a[1], granted);
            Ret {
                eax: e.mem.alloc(granted),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let capacity = ((e.mem.u32(a[1] + 8) & 0x3fff_ffff) * 2).max(4);
            grow(e, a[1], capacity, a[2]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            grow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        (e, log, allocator)
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The logged calls to any of `addresses`, in order.
    fn ordered(log: &Log, addresses: &[u32]) -> Vec<(u32, Vec<u32>)> {
        log.borrow()
            .iter()
            .filter(|(a, _)| addresses.contains(a))
            .cloned()
            .collect()
    }

    /// Makes the `hkArray` at `at` (data, size, capacity word).
    fn array(e: &mut Engine, at: u32, data: u32, size: u32, capacity_word: u32) {
        e.mem.set_u32(at, data);
        e.mem.set_u32(at + 4, size);
        e.mem.set_u32(at + 8, capacity_word);
    }

    /// An empty `hkArray` that does not own storage.
    fn empty(e: &mut Engine, at: u32) {
        array(e, at, 0, 0, 0x8000_0000);
    }

    fn words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    #[test]
    fn the_index_slot_count_follows_the_source_stride_in_elements() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_9c20, &args![10u32, 1u32]).u32(), 12);
        assert_eq!(e.call(0x00d4_9c20, &args![10u32, 2u32]).u32(), 21);
        assert_eq!(e.call(0x00d4_9c20, &args![10u32, 3u32]).u32(), 40);
        assert_eq!(e.call(0x00d4_9c20, &args![10u32, 0u32]).u32(), 40);
    }

    fn shape_with_subparts(e: &mut Engine, owned: bool) -> (u32, u32, u32) {
        let object = e.mem.alloc(0x120);
        let triangles = e.mem.alloc(8);
        e.mem.set_u32(triangles, 0x100);
        e.mem.set_u32(triangles + 4, 0x200);
        let shapes = e.mem.alloc(4);
        e.mem.set_u32(shapes, 0x300);
        let flag = if owned { 0 } else { 0x8000_0000 };
        array(e, object + 0xf0, triangles, 2, flag | 2);
        array(e, object + 0xfc, shapes, 1, flag | 1);
        e.mem.set_u32(object + 0xc0, 0x5000);
        e.mem.set_u32(object + 0xc4, 2);
        (object, triangles, shapes)
    }

    #[test]
    fn the_shape_destructor_releases_the_pointers_and_subparts_and_frees_the_arrays() {
        let (mut e, log, allocator) = setup();
        let (object, triangles, shapes) = shape_with_subparts(&mut e, true);
        e.call(0x00d4_9cb0, &args![object]);
        assert_eq!(
            ordered(
                &log,
                &[
                    REMOVE_REFERENCE,
                    SHAPES_SUBPART_DESTRUCT,
                    FREE_DOUBLE,
                    BASE_DESTRUCT
                ]
            ),
            vec![
                (REMOVE_REFERENCE, vec![0x100]),
                (REMOVE_REFERENCE, vec![0x200]),
                (SHAPES_SUBPART_DESTRUCT, vec![0x5000]),
                (SHAPES_SUBPART_DESTRUCT, vec![0x5040]),
                (REMOVE_REFERENCE, vec![0x300]),
                (FREE_DOUBLE, vec![allocator, shapes, 4]),
                (FREE_DOUBLE, vec![allocator, triangles, 8]),
                (BASE_DESTRUCT, vec![object]),
            ]
        );
        assert_eq!(e.mem.u32(object), SHAPE_VTABLE);
        assert_eq!(e.mem.u32(object + 0x10), SHAPE_SECONDARY_VTABLE);
        assert_eq!(words(&e, object + 0xc4, 1), vec![0]);
        assert_eq!(words(&e, object + 0xf0, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(words(&e, object + 0xfc, 3), vec![0, 0, 0x8000_0000]);
    }

    #[test]
    fn the_shape_destructor_leaves_storage_it_does_not_own() {
        let (mut e, log, _) = setup();
        let (object, _, _) = shape_with_subparts(&mut e, false);
        e.call(0x00d4_9cb0, &args![object]);
        assert!(calls_to(&log, FREE_DOUBLE).is_empty());
        assert_eq!(calls_to(&log, REMOVE_REFERENCE).len(), 3);
        assert_eq!(words(&e, object + 0xf0, 3), vec![0, 0, 0x8000_0000]);
    }

    #[test]
    fn the_shape_destructor_skips_empty_arrays() {
        let (mut e, log, _) = setup();
        let object = e.mem.alloc(0x120);
        empty(&mut e, object + 0xf0);
        empty(&mut e, object + 0xfc);
        e.call(0x00d4_9cb0, &args![object]);
        assert!(calls_to(&log, REMOVE_REFERENCE).is_empty());
        assert!(calls_to(&log, SHAPES_SUBPART_DESTRUCT).is_empty());
        assert_eq!(calls_to(&log, BASE_DESTRUCT), vec![vec![object]]);
    }

    /// A source shape subpart: kind 1 materials `[9]` (stride 0), two float vertices.
    fn shapes_source(e: &mut Engine) -> u32 {
        let src = e.mem.alloc(0x40);
        let materials = e.mem.alloc(8);
        e.mem.write(materials, &[9, 8, 7, 6]);
        e.mem.set_u8(src + 1, 1);
        e.mem.set_u32(src + 4, materials);
        e.mem.set_u16(src + 8, 0);
        e.mem.set_u32(src + 0x18, 3);
        let vertices = e.mem.alloc(0x18);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(vertices + 4 * i as u32, *v);
        }
        e.mem.set_u16(src + 2, 12);
        e.mem.set_u16(src + 0xa, 2);
        e.mem.set_u32(src + 0xc, vertices);
        e.mem.set_u32(src + 0x10, FLOAT_VERTEX_FORMAT);
        src
    }

    /// A shape with the new subpart opened (the double returns `dest`) and the double over the
    /// common-part copy and the two counts; returns `(object, dest)`.
    fn shapes_scene(e: &mut Engine, log: &Log) -> (u32, u32) {
        let object = e.mem.alloc(0x120);
        e.mem.set_u32(object + 0xc4, 3);
        e.mem.set_u32(object + 0xe0, 10);
        empty(e, object + 0xfc);
        let dest = e.mem.alloc(0x40);
        e.register_double(OPEN_SHAPES_SUBPART, move |_, _| Ret {
            eax: dest,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(0x00d4_ad60, move |_, a| {
            sink.borrow_mut().push((0x00d4_ad60, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(SHAPES_CHILD_COUNT, move |_, a| {
            sink.borrow_mut().push((SHAPES_CHILD_COUNT, a.to_vec()));
            Ret {
                eax: 6,
                ..Ret::default()
            }
        });
        (object, dest)
    }

    #[test]
    fn adding_a_shapes_subpart_stores_one_material_and_the_float_vertices() {
        let (mut e, log, _) = setup();
        let (object, dest) = shapes_scene(&mut e, &log);
        let src = shapes_source(&mut e);
        let index = e.call(0x00d4_9dc0, &args![object, src]).u32();
        assert_eq!(index, 2);
        // The storage block: size, count, vtable and the pointer in the array at +0xfc.
        assert_eq!(calls_to(&log, ALLOCATE_DOUBLE).len(), 1);
        assert_eq!(calls_to(&log, ALLOCATE_DOUBLE)[0][1], 0x2c);
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE_MORE)[0][1..],
            [object + 0xfc, 4]
        );
        assert_eq!(e.mem.u32(object + 0x100), 1);
        let storage = e.mem.u32(e.mem.u32(object + 0xfc));
        assert_eq!(e.mem.u32(storage), SHAPES_STORAGE_VTABLE);
        assert_eq!(e.mem.u16(storage + 4), 0x2c);
        assert_eq!(e.mem.u16(storage + 6), 1);
        assert_eq!(calls_to(&log, 0x00d4_ad60), vec![vec![dest, src]]);
        // One 8-bit material index, appended and pointed at.
        assert_eq!(e.mem.u8(dest + 1), 1);
        assert_eq!(words(&e, storage + 0xc, 1), vec![1]);
        let bytes = e.mem.u32(storage + 8);
        assert_eq!(e.mem.u8(bytes), 9);
        assert_eq!(e.mem.u32(dest + 4), bytes);
        assert_eq!(e.mem.u16(dest + 8), 0);
        // The two vertices (12 bytes each) are copied whole.
        assert_eq!(e.mem.u32(storage + 0x18), 2);
        let vertices = e.mem.u32(storage + 0x14);
        assert_eq!(
            words(&e, vertices, 6),
            [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].map(f32::to_bits)
        );
        assert_eq!(e.mem.u32(dest + 0xc), vertices);
        assert_eq!(e.mem.u16(dest + 2), 0xc);
        assert_eq!(e.mem.u16(dest + 0xa), 2);
        // The extents are recalculated, the child count of the new subpart added.
        assert_eq!(calls_to(&log, RECALC_AABB_EXTENTS), vec![vec![object]]);
        assert_eq!(calls_to(&log, SHAPES_CHILD_COUNT), vec![vec![dest]]);
        assert_eq!(e.mem.u32(object + 0xe0), 16);
    }

    #[test]
    fn adding_a_shapes_subpart_copies_strided_sixteen_bit_materials() {
        let (mut e, log, _) = setup();
        let (object, dest) = shapes_scene(&mut e, &log);
        let src = shapes_source(&mut e);
        // Three 16-bit indices 1, 2, 3 four bytes apart; kind 2, stride 4.
        let materials = e.mem.alloc(16);
        e.mem
            .write(materials, &[1, 0, 0xee, 0xee, 2, 0, 0xee, 0xee, 3, 0]);
        e.mem.set_u8(src + 1, 2);
        e.mem.set_u32(src + 4, materials);
        e.mem.set_u16(src + 8, 4);
        e.call(0x00d4_9dc0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xfc));
        assert_eq!(e.mem.u32(storage + 0x24), 3);
        let first = e.mem.u32(storage + 0x20);
        assert_eq!(
            (0..3).map(|i| e.mem.u16(first + 2 * i)).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert_eq!(e.mem.u32(dest + 4), first);
        // The stride becomes the element size.
        assert_eq!(e.mem.u16(dest + 8), 2);
        let reserve = calls_to(&log, ARRAY_RESERVE);
        assert_eq!(reserve.len(), 2);
        assert_eq!(reserve[0][1..], [storage + 0x20, 3, 2]);
        assert_eq!(reserve[1][1..], [storage + 0x14, 2, 0xc]);
    }

    #[test]
    fn adding_a_shapes_subpart_without_materials_keeps_the_vertices_out() {
        let (mut e, log, _) = setup();
        let (object, dest) = shapes_scene(&mut e, &log);
        let src = shapes_source(&mut e);
        e.mem.set_u32(src + 4, 0);
        e.mem.set_u16(dest + 8, 0x55);
        e.mem.set_u32(dest + 0xc, 0xabcd);
        e.call(0x00d4_9dc0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xfc));
        assert_eq!(e.mem.u32(dest + 4), 0);
        assert_eq!(e.mem.u16(dest + 8), 0);
        assert_eq!(e.mem.u32(storage + 0x18), 0);
        assert_eq!(e.mem.u32(dest + 0xc), 0xabcd);
        assert_eq!(calls_to(&log, RECALC_AABB_EXTENTS).len(), 1);
    }

    #[test]
    fn adding_a_shapes_subpart_with_another_vertex_layout_stores_the_half_pattern() {
        let (mut e, log, _) = setup();
        let (object, dest) = shapes_scene(&mut e, &log);
        let src = shapes_source(&mut e);
        e.mem.set_u32(src + 0x10, 0);
        e.mem.set_u16(src + 2, 0);
        e.call(0x00d4_9dc0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xfc));
        let vertices = e.mem.u32(storage + 0x14);
        // Only the first word is copied; the second is the pair of halves (0, 0x3f80); the
        // third is not written. A stride of 0 repeats the first vertex.
        assert_eq!(
            words(&e, vertices, 6),
            vec![
                1.0f32.to_bits(),
                0x3f80_0000,
                0,
                1.0f32.to_bits(),
                0x3f80_0000,
                0
            ]
        );
        assert_eq!(e.mem.u16(dest + 2), 0);
        assert_eq!(e.mem.u16(dest + 0xa), 1);
    }

    #[test]
    fn the_triangle_storage_vtable_setter_returns_this() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d4_a170, &args![this, 7u32]).u32(), this);
        assert_eq!(e.mem.u32(this), TRIANGLES_STORAGE_VTABLE);
    }

    /// A shape with two triangle subparts, each with a storage, and one shape subpart.
    fn loaded_shape(e: &mut Engine) -> (u32, [u32; 2], u32, u32) {
        let object = e.mem.alloc(0x120);
        let subparts = e.mem.alloc(0xc0);
        e.mem.set_u32(object + 0xb4, subparts);
        e.mem.set_u32(object + 0xb8, 2);
        let storages = [e.mem.alloc(0x68), e.mem.alloc(0x68)];
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, storages[0]);
        e.mem.set_u32(list + 4, storages[1]);
        e.mem.set_u32(object + 0xf0, list);
        for (i, storage) in storages.iter().enumerate() {
            for (offset, value) in [
                (8, 0x1111),
                (0x14, 0x2222),
                (0x20, 0x3333),
                (0x2c, 0x4444),
                (0x38, 0x5555),
                (0x5c, 0x6666),
                (0x44, 0x7777),
                (0x50, 0x8888),
            ] {
                e.mem.set_u32(storage + offset, value + i as u32);
            }
        }
        // The second storage has named vertices (a non-empty array at +0x54).
        e.mem.set_u32(storages[1] + 0x54, 1);
        // Subpart 0: 8-bit indices and materials; subpart 1: 32-bit indices, 16-bit materials.
        e.mem.set_u8(subparts + 0x48, 1);
        e.mem.set_u8(subparts + 1, 1);
        e.mem.set_u8(subparts + 0x60 + 0x48, 3);
        e.mem.set_u8(subparts + 0x60 + 1, 2);
        let shape_subparts = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0xc0, shape_subparts);
        e.mem.set_u32(object + 0xc4, 1);
        let shape_storage = e.mem.alloc(0x2c);
        let shape_list = e.mem.alloc(4);
        e.mem.set_u32(shape_list, shape_storage);
        e.mem.set_u32(object + 0xfc, shape_list);
        for (offset, value) in [(8, 0xaaa), (0x20, 0xbbb), (0x14, 0xccc)] {
            e.mem.set_u32(shape_storage + offset, value);
        }
        e.mem.set_u8(shape_subparts + 1, 1);
        (object, storages, subparts, shape_subparts)
    }

    #[test]
    fn the_loading_constructor_without_the_flag_only_sets_the_vtables() {
        let (mut e, log, _) = setup();
        let (object, _, subparts, _) = loaded_shape(&mut e);
        assert_eq!(e.call(0x00d4_a180, &args![object, 0u32]).u32(), object);
        assert_eq!(calls_to(&log, BASE_LOAD_CONSTRUCT), vec![vec![object, 0]]);
        assert_eq!(e.mem.u32(object), SHAPE_VTABLE);
        assert_eq!(e.mem.u32(object + 0x10), SHAPE_SECONDARY_VTABLE);
        assert_eq!(e.mem.u32(subparts + 0x18), 0);
    }

    #[test]
    fn the_loading_constructor_points_the_subparts_at_their_storage() {
        let (mut e, log, _) = setup();
        let (object, storages, subparts, shape_subparts) = loaded_shape(&mut e);
        e.register_double(0x00d4_a170, {
            let sink = Rc::clone(&log);
            move |_, a| {
                sink.borrow_mut().push((0x00d4_a170, a.to_vec()));
                Ret::default()
            }
        });
        assert_eq!(e.call(0x00d4_a180, &args![object, 1u32]).u32(), object);
        assert_eq!(
            calls_to(&log, 0x00d4_a170),
            vec![vec![storages[0], 1], vec![storages[1], 1]]
        );
        // Subpart 0: positions, 8-bit indices, 8-bit materials, vertices (no names).
        assert_eq!(e.mem.u32(subparts + 0x18), 0x1111);
        assert_eq!(e.mem.u32(subparts + 0x40), 0x2222);
        assert_eq!(e.mem.u32(subparts + 4), 0x5555);
        assert_eq!(e.mem.u32(subparts + 0xc), 0x7777);
        // Subpart 1: 32-bit indices, 16-bit materials, the named vertices.
        let second = subparts + 0x60;
        assert_eq!(e.mem.u32(second + 0x18), 0x1112);
        assert_eq!(e.mem.u32(second + 0x40), 0x4445);
        assert_eq!(e.mem.u32(second + 4), 0x6667);
        assert_eq!(e.mem.u32(second + 0xc), 0x8889);
        // The shape subpart takes its 8-bit materials and its vertices.
        assert_eq!(e.mem.u32(shape_subparts + 4), 0xaaa);
        assert_eq!(e.mem.u32(shape_subparts + 0xc), 0xccc);
        assert_eq!(calls_to(&log, SHAPES_STORAGE_LOADED).len(), 1);
        assert_eq!(calls_to(&log, SHAPES_STORAGE_LOADED)[0][1], 1);
    }

    #[test]
    fn the_loading_constructor_takes_sixteen_bit_materials_of_a_shape_subpart() {
        let (mut e, _, _) = setup();
        let (object, _, _, shape_subparts) = loaded_shape(&mut e);
        e.register_double(0x00d4_a170, |_, _| Ret::default());
        e.mem.set_u8(shape_subparts + 1, 2);
        e.call(0x00d4_a180, &args![object, 1u32]);
        assert_eq!(e.mem.u32(shape_subparts + 4), 0xbbb);
    }

    #[test]
    fn the_loading_constructor_does_not_call_the_storage_setter_for_a_null_pointer() {
        let (mut e, log, _) = setup();
        // The exe reads the (absent) storage through the null pointer anyway; map the zero page.
        e.map(0, 0x1000);
        let (object, storages, _, _) = loaded_shape(&mut e);
        e.register_double(0x00d4_a170, {
            let sink = Rc::clone(&log);
            move |_, a| {
                sink.borrow_mut().push((0x00d4_a170, a.to_vec()));
                Ret::default()
            }
        });
        let list = e.mem.u32(object + 0xf0);
        e.mem.set_u32(list, 0);
        e.call(0x00d4_a180, &args![object, 5u32]);
        assert_eq!(calls_to(&log, 0x00d4_a170), vec![vec![storages[1], 5]]);
    }

    /// A source triangle subpart: two positions, two triangles (`indices`, of `kind` 1, 2 or 3
    /// with the byte `stride`), one 8-bit material index 5 and two float vertices.
    fn triangle_source(e: &mut Engine, kind: u8, stride: u32, indices: &[u8]) -> u32 {
        let src = e.mem.alloc(0x60);
        let positions = e.mem.alloc(0x18);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(positions + 4 * i as u32, *v);
        }
        e.mem.set_u32(src + 0x18, positions);
        e.mem.set_u32(src + 0x1c, 12);
        e.mem.set_u32(src + 0x20, 2);
        let index_data = e.mem.alloc(indices.len() as u32 + 4);
        e.mem.write(index_data, indices);
        e.mem.set_u32(src + 0x40, index_data);
        e.mem.set_u32(src + 0x44, stride);
        e.mem.set_u32(src + 0x14, 2);
        e.mem.set_u8(src + 0x48, kind);
        e.mem.set_u8(src + 0x49, 0x77);
        let materials = e.mem.alloc(8);
        e.mem.write(materials, &[5, 6, 7, 8]);
        e.mem.set_u8(src + 1, 1);
        e.mem.set_u32(src + 4, materials);
        e.mem.set_u16(src + 8, 0);
        let vertices = e.mem.alloc(0x18);
        for (i, v) in [7.0f32, 8.0, 9.0, 10.0, 11.0, 12.0].iter().enumerate() {
            e.mem.set_f32(vertices + 4 * i as u32, *v);
        }
        e.mem.set_u16(src + 2, 12);
        e.mem.set_u16(src + 0xa, 2);
        e.mem.set_u32(src + 0xc, vertices);
        e.mem.set_u32(src + 0x10, FLOAT_VERTEX_FORMAT);
        src
    }

    /// A shape with the new triangle subpart opened (the double returns `dest`, a block of
    /// 0x60 bytes) and doubles over `memset` (which really clears) and the child count.
    fn triangles_scene(e: &mut Engine, log: &Log) -> (u32, u32) {
        let object = e.mem.alloc(0x120);
        e.mem.set_u32(object + 0xb8, 3);
        e.mem.set_u32(object + 0xe0, 10);
        empty(e, object + 0xf0);
        let dest = e.mem.alloc(0x60);
        e.register_double(OPEN_TRIANGLES_SUBPART, move |_, _| Ret {
            eax: dest,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(MEMORY_SET, move |e, a| {
            sink.borrow_mut().push((MEMORY_SET, a.to_vec()));
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(TRIANGLES_CHILD_COUNT, move |_, a| {
            sink.borrow_mut().push((TRIANGLES_CHILD_COUNT, a.to_vec()));
            Ret {
                eax: 6,
                ..Ret::default()
            }
        });
        (object, dest)
    }

    #[test]
    fn adding_a_triangles_subpart_copies_positions_eight_bit_indices_materials_and_vertices() {
        let (mut e, log, _) = setup();
        let (object, dest) = triangles_scene(&mut e, &log);
        let src = triangle_source(&mut e, 1, 3, &[0, 1, 2, 3, 4, 5]);
        e.mem.set_u32(src + 0x30, 0x3030);
        e.call(0x00d4_a2b0, &args![object, src]);
        // The header copy (00d4abf0) ran for real.
        assert_eq!(e.mem.u32(dest + 0x30), 0x3030);
        // The storage block and its entry in the pointer array.
        assert_eq!(calls_to(&log, ALLOCATE_DOUBLE)[0][1], 0x68);
        assert_eq!(e.mem.u32(object + 0xf4), 1);
        let storage = e.mem.u32(e.mem.u32(object + 0xf0));
        assert_eq!(e.mem.u32(storage), TRIANGLES_STORAGE_VTABLE);
        assert_eq!(e.mem.u16(storage + 4), 0x68);
        // Positions: three words and the float at 01011d78 as the fourth lane.
        assert_eq!(e.mem.u32(storage + 0xc), 2);
        let positions = e.mem.u32(storage + 8);
        assert_eq!(
            words(&e, positions, 8),
            vec![
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                LOADED_PATTERN,
                4.0f32.to_bits(),
                5.0f32.to_bits(),
                6.0f32.to_bits(),
                LOADED_PATTERN
            ]
        );
        assert_eq!(e.mem.u32(dest + 0x18), positions);
        assert_eq!(e.mem.u32(dest + 0x1c), 0x10);
        assert_eq!(e.mem.u32(dest + 0x20), 2);
        // Indices: stride 3 is above 2 elements, so the destination stride is 4; the block
        // holds 2 * 4 slots, cleared, and each triangle's three indices are copied.
        assert_eq!(e.mem.u32(dest + 0x44), 4);
        let indices = e.mem.u32(storage + 0x14);
        assert_eq!(e.mem.u32(storage + 0x18), 8);
        assert_eq!(e.mem.bytes(indices, 8), vec![0, 1, 2, 0, 3, 4, 5, 0]);
        assert_eq!(calls_to(&log, MEMORY_SET), vec![vec![indices, 0, 8]]);
        assert_eq!(e.mem.u32(dest + 0x40), indices);
        assert_eq!(e.mem.u8(dest + 0x48), 1);
        assert_eq!(e.mem.u32(dest + 0x14), 2);
        assert_eq!(e.mem.u8(dest + 0x49), 0x77);
        // The single material index.
        let materials = e.mem.u32(storage + 0x38);
        assert_eq!(e.mem.u8(materials), 5);
        assert_eq!(e.mem.u32(dest + 4), materials);
        assert_eq!(e.mem.u16(dest + 8), 0);
        // The vertices.
        let vertices = e.mem.u32(storage + 0x44);
        assert_eq!(e.mem.u32(storage + 0x48), 2);
        assert_eq!(
            words(&e, vertices, 6),
            [7.0f32, 8.0, 9.0, 10.0, 11.0, 12.0].map(f32::to_bits)
        );
        assert_eq!(e.mem.u32(dest + 0xc), vertices);
        assert_eq!(e.mem.u16(dest + 2), 0xc);
        assert_eq!(e.mem.u16(dest + 0xa), 2);
        // Extents, child count (subpart index = size - 1).
        assert_eq!(calls_to(&log, RECALC_AABB_EXTENTS), vec![vec![object]]);
        assert_eq!(
            calls_to(&log, TRIANGLES_CHILD_COUNT),
            vec![vec![object, dest, 2]]
        );
        assert_eq!(e.mem.u32(object + 0xe0), 16);
    }

    #[test]
    fn adding_a_triangles_subpart_with_sixteen_bit_indices() {
        let (mut e, log, _) = setup();
        let (object, dest) = triangles_scene(&mut e, &log);
        let bytes: Vec<u8> = (1u16..=6).flat_map(u16::to_le_bytes).collect();
        let src = triangle_source(&mut e, 2, 6, &bytes);
        e.call(0x00d4_a2b0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xf0));
        // 6 bytes is 3 elements, limited to 4: the destination stride is 8 bytes.
        assert_eq!(e.mem.u32(dest + 0x44), 8);
        let indices = e.mem.u32(storage + 0x20);
        assert_eq!(e.mem.u32(storage + 0x24), 8);
        assert_eq!(
            (0..8)
                .map(|i| e.mem.u16(indices + 2 * i))
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 0, 4, 5, 6, 0]
        );
        assert_eq!(calls_to(&log, MEMORY_SET), vec![vec![indices, 0, 16]]);
        assert_eq!(e.mem.u32(dest + 0x40), indices);
        assert_eq!(e.mem.u8(dest + 0x48), 2);
    }

    #[test]
    fn adding_a_triangles_subpart_with_thirty_two_bit_indices() {
        let (mut e, log, _) = setup();
        let (object, dest) = triangles_scene(&mut e, &log);
        let bytes: Vec<u8> = (1u32..=6).flat_map(u32::to_le_bytes).collect();
        let src = triangle_source(&mut e, 3, 12, &bytes);
        e.call(0x00d4_a2b0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xf0));
        assert_eq!(e.mem.u32(dest + 0x44), 16);
        let indices = e.mem.u32(storage + 0x2c);
        assert_eq!(e.mem.u32(storage + 0x30), 8);
        assert_eq!(words(&e, indices, 8), vec![1, 2, 3, 0, 4, 5, 6, 0]);
        assert_eq!(calls_to(&log, MEMORY_SET), vec![vec![indices, 0, 32]]);
        assert_eq!(e.mem.u8(dest + 0x48), 3);
    }

    #[test]
    fn adding_a_triangles_subpart_with_a_one_element_stride_shares_the_indices() {
        let (mut e, log, _) = setup();
        let (object, dest) = triangles_scene(&mut e, &log);
        let src = triangle_source(&mut e, 1, 1, &[10, 11, 12, 13]);
        e.call(0x00d4_a2b0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xf0));
        // A strip: count + 2 slots, each triangle starting one element after the last.
        assert_eq!(e.mem.u32(dest + 0x44), 1);
        let indices = e.mem.u32(storage + 0x14);
        assert_eq!(e.mem.u32(storage + 0x18), 4);
        assert_eq!(e.mem.bytes(indices, 4), vec![10, 11, 12, 13]);
    }

    #[test]
    fn adding_a_triangles_subpart_with_strided_materials() {
        let (mut e, log, _) = setup();
        let (object, dest) = triangles_scene(&mut e, &log);
        let src = triangle_source(&mut e, 1, 3, &[0, 1, 2, 3, 4, 5]);
        let materials = e.mem.alloc(8);
        e.mem.write(materials, &[5, 0xee, 6, 0xee]);
        e.mem.set_u32(src + 4, materials);
        e.mem.set_u16(src + 8, 2);
        e.call(0x00d4_a2b0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xf0));
        // One index per triangle, two bytes apart, appended; the stride becomes 1.
        assert_eq!(e.mem.u32(storage + 0x3c), 2);
        let first = e.mem.u32(storage + 0x38);
        assert_eq!(e.mem.bytes(first, 2), vec![5, 6]);
        assert_eq!(e.mem.u32(dest + 4), first);
        assert_eq!(e.mem.u16(dest + 8), 1);
    }

    #[test]
    fn adding_a_triangles_subpart_with_sixteen_bit_materials_and_no_materials() {
        let (mut e, log, _) = setup();
        let (object, dest) = triangles_scene(&mut e, &log);
        let src = triangle_source(&mut e, 1, 3, &[0, 1, 2, 3, 4, 5]);
        let materials = e.mem.alloc(8);
        e.mem.write(materials, &[0x34, 0x12]);
        e.mem.set_u8(src + 1, 2);
        e.mem.set_u32(src + 4, materials);
        e.call(0x00d4_a2b0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xf0));
        assert_eq!(e.mem.u32(storage + 0x60), 1);
        let first = e.mem.u32(storage + 0x5c);
        assert_eq!(e.mem.u16(first), 0x1234);
        assert_eq!(e.mem.u32(dest + 4), first);
        // Without a material pointer there is nothing to copy and no vertex data either.
        let (object, dest) = triangles_scene(&mut e, &log);
        let src = triangle_source(&mut e, 1, 3, &[0, 1, 2, 3, 4, 5]);
        e.mem.set_u32(src + 4, 0);
        e.call(0x00d4_a2b0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xf0));
        assert_eq!(e.mem.u32(dest + 4), 0);
        assert_eq!(e.mem.u16(dest + 8), 0);
        assert_eq!(e.mem.u32(storage + 0x48), 0);
    }

    #[test]
    fn adding_a_triangles_subpart_with_named_vertices() {
        let (mut e, log, _) = setup();
        let (object, dest) = triangles_scene(&mut e, &log);
        let src = triangle_source(&mut e, 1, 3, &[0, 1, 2, 3, 4, 5]);
        let entries = e.mem.alloc(0x10);
        e.mem.set_u32(entries, 0x11);
        e.mem.set_u32(entries + 4, 0xaa);
        e.mem.set_u32(entries + 8, 0x22);
        e.mem.set_u32(entries + 0xc, 0xbb);
        e.mem.set_u32(src + 0x10, NAMED_VERTEX_FORMAT);
        e.mem.set_u16(src + 2, 8);
        e.mem.set_u32(src + 0xc, entries);
        e.call(0x00d4_a2b0, &args![object, src]);
        let storage = e.mem.u32(e.mem.u32(object + 0xf0));
        assert_eq!(e.mem.u32(storage + 0x54), 2);
        let names = e.mem.u32(storage + 0x50);
        // The new entries are built once as "Default" strings, then again before assignment.
        assert_eq!(
            calls_to(&log, STRING_PTR_CONSTRUCT),
            vec![
                vec![names + 4, DEFAULT_NAME],
                vec![names + 0xc, DEFAULT_NAME],
                vec![names + 4, DEFAULT_NAME],
                vec![names + 0xc, DEFAULT_NAME],
            ]
        );
        assert_eq!(
            calls_to(&log, STRING_PTR_ASSIGN),
            vec![
                vec![names + 4, entries + 4],
                vec![names + 0xc, entries + 0xc]
            ]
        );
        assert_eq!(e.mem.u32(names), 0x11);
        assert_eq!(e.mem.u32(names + 8), 0x22);
        assert_eq!(e.mem.u32(dest + 0xc), names);
        assert_eq!(e.mem.u16(dest + 2), 8);
        assert_eq!(e.mem.u16(dest + 0xa), 2);
        // The plain vertex array is untouched.
        assert_eq!(e.mem.u32(storage + 0x48), 0);
        // A zero stride gives one entry's worth of description.
        let (object, dest) = triangles_scene(&mut e, &log);
        let src = triangle_source(&mut e, 1, 3, &[0, 1, 2, 3, 4, 5]);
        e.mem.set_u32(src + 0x10, NAMED_VERTEX_FORMAT);
        e.mem.set_u16(src + 2, 0);
        e.mem.set_u32(src + 0xc, entries);
        e.call(0x00d4_a2b0, &args![object, src]);
        assert_eq!(e.mem.u16(dest + 2), 0);
        assert_eq!(e.mem.u16(dest + 0xa), 1);
    }

    #[test]
    fn the_copy_constructor_adds_every_subpart_and_copies_the_sixteen_bit_array() {
        let (mut e, log, allocator) = setup();
        let src = e.mem.alloc(0x120);
        let object = e.mem.alloc(0x120);
        e.register_double(BASE_CONSTRUCT, {
            let sink = Rc::clone(&log);
            move |_, a| {
                sink.borrow_mut().push((BASE_CONSTRUCT, a.to_vec()));
                Ret::default()
            }
        });
        for address in [0x00d4_a2b0, 0x00d4_9dc0] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        e.mem.set_u32(src + 0xe4, 0x7f80_0001);
        e.mem.set_u32(src + 0xb0, 0x4321);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(src + 0x80 + 4 * i as u32, *v);
        }
        e.mem.set_u32(src + 8, 0x88);
        e.mem.set_u8(src + 0x14, 0x14);
        e.mem.set_u32(src + 0xb4, 0x6000);
        e.mem.set_u32(src + 0xb8, 2);
        e.mem.set_u32(src + 0xc0, 0x7000);
        e.mem.set_u32(src + 0xc4, 1);
        let values = e.mem.alloc(8);
        e.mem.write(values, &[1, 0, 2, 0, 3, 0]);
        array(&mut e, src + 0xcc, values, 3, 3);
        e.mem.set_u8(src + 0xd8, 0xd8);
        // The destination starts with an owned 16-bit array of capacity 2.
        let old = e.mem.alloc(8);
        array(&mut e, object + 0xcc, old, 0, 2);
        assert_eq!(e.call(0x00d4_aa70, &args![object, src]).u32(), object);
        assert_eq!(
            calls_to(&log, BASE_CONSTRUCT),
            vec![vec![object, 0x7fc0_0001, 0x4321]]
        );
        assert_eq!(e.mem.u32(object), SHAPE_VTABLE);
        assert_eq!(e.mem.u32(object + 0x10), SHAPE_SECONDARY_VTABLE);
        assert_eq!(words(&e, object + 0xf0, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(words(&e, object + 0xfc, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(
            words(&e, object + 0x80, 4),
            [1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits)
        );
        assert_eq!(e.mem.u32(object + 8), 0x88);
        assert_eq!(e.mem.u8(object + 0x14), 0x14);
        assert_eq!(
            ordered(&log, &[0x00d4_a2b0, 0x00d4_9dc0]),
            vec![
                (0x00d4_a2b0, vec![object, 0x6000]),
                (0x00d4_a2b0, vec![object, 0x6060]),
                (0x00d4_9dc0, vec![object, 0x7000]),
            ]
        );
        // The old storage is freed, a new block asked for (3 values = 6 bytes, granted 14).
        assert_eq!(calls_to(&log, FREE_DOUBLE), vec![vec![allocator, old, 4]]);
        assert_eq!(calls_to(&log, ALLOCATE_SIZED_DOUBLE).len(), 1);
        assert_eq!(e.mem.u32(object + 0xd4), 7);
        assert_eq!(e.mem.u32(object + 0xd0), 3);
        let data = e.mem.u32(object + 0xcc);
        assert_eq!(
            (0..3).map(|i| e.mem.u16(data + 2 * i)).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert_eq!(e.mem.u8(object + 0xd8), 0xd8);
        assert_eq!(calls_to(&log, RECALC_AABB_EXTENTS), vec![vec![object]]);
    }

    #[test]
    fn the_copy_constructor_keeps_a_sixteen_bit_array_that_is_large_enough() {
        let (mut e, log, _) = setup();
        let src = e.mem.alloc(0x120);
        let object = e.mem.alloc(0x120);
        e.register_double(BASE_CONSTRUCT, |_, _| Ret::default());
        let values = e.mem.alloc(8);
        e.mem.write(values, &[7, 0, 8, 0]);
        array(&mut e, src + 0xcc, values, 2, 2);
        let own = e.mem.alloc(8);
        array(&mut e, object + 0xcc, own, 0, 0x8000_0003);
        e.call(0x00d4_aa70, &args![object, src]);
        assert!(calls_to(&log, FREE_DOUBLE).is_empty());
        assert!(calls_to(&log, ALLOCATE_SIZED_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(object + 0xcc), own);
        assert_eq!(e.mem.u32(object + 0xd0), 2);
        assert_eq!(e.mem.u16(own), 7);
        assert_eq!(e.mem.u16(own + 2), 8);
    }

    #[test]
    fn the_copy_constructor_frees_nothing_when_the_old_storage_is_not_owned() {
        let (mut e, log, _) = setup();
        let src = e.mem.alloc(0x120);
        let object = e.mem.alloc(0x120);
        e.register_double(BASE_CONSTRUCT, |_, _| Ret::default());
        let values = e.mem.alloc(8);
        array(&mut e, src + 0xcc, values, 2, 2);
        array(&mut e, object + 0xcc, 0x1000, 0, 0x8000_0001);
        e.call(0x00d4_aa70, &args![object, src]);
        assert!(calls_to(&log, FREE_DOUBLE).is_empty());
        assert_eq!(calls_to(&log, ALLOCATE_SIZED_DOUBLE).len(), 1);
        assert_eq!(e.mem.u32(object + 0xd4), 6);
    }

    #[test]
    fn the_subpart_header_copy_skips_the_middle_words() {
        let mut e = Engine::new();
        let src = e.mem.alloc(0x60);
        let dest = e.mem.alloc(0x60);
        for i in 0..0x18u32 {
            e.mem.set_u32(src + 4 * i, 0x100 + i);
            e.mem.set_u32(dest + 4 * i, 0xdead_0000);
        }
        assert_eq!(e.call(0x00d4_abf0, &args![dest, src]).u32(), dest);
        for offset in [
            0u32, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x30, 0x34, 0x38, 0x3c,
        ] {
            assert_eq!(e.mem.u32(dest + offset), 0x100 + offset / 4, "{offset:#x}");
        }
        for offset in [0x40u32, 0x44, 0x4c, 0x50] {
            assert_eq!(e.mem.u32(dest + offset), 0x100 + offset / 4, "{offset:#x}");
        }
        // Only the bytes at +0x48 and +0x49 are copied.
        assert_eq!(e.mem.u32(dest + 0x48), 0xdead_0112);
        for offset in [0x24u32, 0x28, 0x2c, 0x54, 0x58, 0x5c] {
            assert_eq!(e.mem.u32(dest + offset), 0xdead_0000, "{offset:#x}");
        }
    }

    #[test]
    fn the_shapes_storage_destructor_empties_its_three_arrays() {
        let (mut e, log, allocator) = setup();
        let object = e.mem.alloc(0x30);
        array(&mut e, object + 8, 0x1000, 3, 4);
        array(&mut e, object + 0x14, 0x2000, 3, 5);
        array(&mut e, object + 0x20, 0x3000, 3, 0x8000_0006);
        e.call(0x00d4_ac70, &args![object]);
        assert_eq!(
            calls_to(&log, FREE_DOUBLE),
            vec![vec![allocator, 0x2000, 60], vec![allocator, 0x1000, 4]]
        );
        for at in [8, 0x14, 0x20] {
            assert_eq!(words(&e, object + at, 3), vec![0, 0, 0x8000_0000]);
        }
        assert_eq!(e.mem.u32(object), BASE_VTABLE);
    }

    #[test]
    fn the_common_part_copy_makes_the_reference_array_equal_to_the_sources() {
        let (mut e, log, _) = setup();
        let src = e.mem.alloc(0x40);
        let dest = e.mem.alloc(0x40);
        for i in 0..0x10u32 {
            e.mem.set_u32(src + 4 * i, 0x100 + i);
        }
        let source_list = e.mem.alloc(8);
        e.mem.set_u32(source_list, 0xa1);
        e.mem.set_u32(source_list + 4, 0xa2);
        array(&mut e, src + 0x14, source_list, 2, 2);
        let dest_list = e.mem.alloc(12);
        for (i, p) in [0xb1u32, 0xb2, 0xb3].iter().enumerate() {
            e.mem.set_u32(dest_list + 4 * i as u32, *p);
        }
        array(&mut e, dest + 0x14, dest_list, 3, 3);
        assert_eq!(e.call(0x00d4_ad60, &args![dest, src]).u32(), dest);
        assert_eq!(
            ordered(&log, &[ADD_REFERENCE, REMOVE_REFERENCE]),
            vec![
                (REMOVE_REFERENCE, vec![0xb3]),
                (ADD_REFERENCE, vec![0xa1]),
                (REMOVE_REFERENCE, vec![0xb1]),
                (ADD_REFERENCE, vec![0xa2]),
                (REMOVE_REFERENCE, vec![0xb2]),
            ]
        );
        assert!(calls_to(&log, ARRAY_RESERVE).is_empty());
        assert_eq!(words(&e, dest_list, 3), vec![0xa1, 0xa2, 0]);
        assert_eq!(e.mem.u32(dest + 0x18), 2);
        assert_eq!(words(&e, dest, 5), vec![0x100, 0x101, 0x102, 0x103, 0x104]);
        assert_eq!(words(&e, dest + 0x20, 4), words(&e, src + 0x20, 4));
        assert_eq!(words(&e, dest + 0x30, 4), words(&e, src + 0x30, 4));
    }

    #[test]
    fn the_common_part_copy_grows_the_array_and_adds_the_extra_references() {
        let (mut e, log, allocator) = setup();
        let src = e.mem.alloc(0x40);
        let dest = e.mem.alloc(0x40);
        let source_list = e.mem.alloc(12);
        for (i, p) in [0xa1u32, 0, 0xa3].iter().enumerate() {
            e.mem.set_u32(source_list + 4 * i as u32, *p);
        }
        array(&mut e, src + 0x14, source_list, 3, 3);
        let dest_list = e.mem.alloc(4);
        e.mem.set_u32(dest_list, 0xb1);
        array(&mut e, dest + 0x14, dest_list, 1, 1);
        e.call(0x00d4_ad60, &args![dest, src]);
        // The array is reserved to the source's size (3 above twice the capacity, 2).
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE),
            vec![vec![allocator, dest + 0x14, 3, 4]]
        );
        let data = e.mem.u32(dest + 0x14);
        assert_eq!(words(&e, data, 3), vec![0xa1, 0, 0xa3]);
        assert_eq!(e.mem.u32(dest + 0x18), 3);
        // The common entry adds and releases; the extra ones add when not null.
        assert_eq!(
            ordered(&log, &[ADD_REFERENCE, REMOVE_REFERENCE]),
            vec![
                (ADD_REFERENCE, vec![0xa1]),
                (REMOVE_REFERENCE, vec![0xb1]),
                (ADD_REFERENCE, vec![0xa3]),
            ]
        );
    }

    #[test]
    fn the_triangle_storage_constructor_empties_all_eight_arrays() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x68);
        for i in 0..0x1a {
            e.mem.set_u32(this + 4 * i, 0xdead_beef);
        }
        assert_eq!(e.call(0x00d4_aef0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TRIANGLES_STORAGE_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u16(this + 4), 0xbeef);
        for at in [8, 0x14, 0x20, 0x2c, 0x38, 0x44, 0x50, 0x5c] {
            assert_eq!(words(&e, this + at, 3), vec![0, 0, 0x8000_0000], "{at:#x}");
        }
    }

    #[test]
    fn the_triangle_storage_destructor_frees_the_arrays_and_destroys_the_names() {
        let (mut e, log, allocator) = setup();
        let this = e.mem.alloc(0x68);
        let arrays: [(u32, u32, u32); 8] = [
            (0x5c, 2, 2),
            (0x50, 3, 8),
            (0x44, 4, 12),
            (0x38, 5, 1),
            (0x2c, 6, 4),
            (0x20, 7, 2),
            (0x14, 8, 1),
            (8, 9, 16),
        ];
        for (at, capacity, _) in arrays {
            array(&mut e, this + at, 0x1000 * at, 2, capacity);
        }
        e.call(0x00d4_af60, &args![this]);
        let mut expected: Vec<(u32, Vec<u32>)> = Vec::new();
        for (at, capacity, scale) in arrays {
            if at == 0x50 {
                expected.push((STRING_PTR_DESTRUCT, vec![0x1000 * at + 4]));
                expected.push((STRING_PTR_DESTRUCT, vec![0x1000 * at + 0xc]));
            }
            expected.push((FREE_DOUBLE, vec![allocator, 0x1000 * at, capacity * scale]));
        }
        assert_eq!(ordered(&log, &[FREE_DOUBLE, STRING_PTR_DESTRUCT]), expected);
        for (at, _, _) in arrays {
            assert_eq!(words(&e, this + at, 3), vec![0, 0, 0x8000_0000]);
        }
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }
}
