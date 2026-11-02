//! `BSHavok/bhkshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkShape` (vtable `01030bf4`, RTTI-confirmed) is a `bhkSerializable` whose word at `+0x10`
//! holds the material in its low five bits. The functions marked `Translation incomplete` carry
//! an `FS:[0]` exception-unwinding frame, which is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkSerializable` functions the shape calls (engine map names): `Kill` (`00c876a0`),
/// `~bhkSerializable` (`00c877b0`), `LinkObject` (`00c87670`), `LoadBinary` (`00c87650`),
/// `SaveBinary` (`00c87850`) and `CopyMembers` (`00c87600`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const SERIALIZABLE_DESTRUCT: u32 = 0x00c8_77b0;
const SERIALIZABLE_LINK_OBJECT: u32 = 0x00c8_7670;
const SERIALIZABLE_LOAD_BINARY: u32 = 0x00c8_7650;
const SERIALIZABLE_SAVE_BINARY: u32 = 0x00c8_7850;
const SERIALIZABLE_COPY_MEMBERS: u32 = 0x00c8_7600;
/// `bhkRefObject::Link` (engine map name, `00c87c90`, thiscall: the flag).
const REF_OBJECT_LINK: u32 = 0x00c8_7c90;
/// The base equality test (`00ba87d0`, thiscall: the other object; the engine map does not
/// name it).
const BASE_IS_EQUAL: u32 = 0x00ba_87d0;
/// `hkpInertiaTensorComputer::computeShapeVolumeMassProperties` (engine map name, `00cf0690`,
/// cdecl: the Havok shape, the float at `+4` of the argument, the argument).
const COMPUTE_SHAPE_VOLUME_MASS_PROPERTIES: u32 = 0x00cf_0690;
/// `BuildDisplayFromhkGeometry` callees: the memory manager's allocate (`00aa3e40`, thiscall:
/// size), the heap allocator (`00aa13e0`, cdecl: size), the vertex copy (`00458620`, cdecl:
/// destination, source; unnamed), `NiTriShape::NiTriShape` (`00a74410`, engine map name), the
/// string interning (`00a5b690`, cdecl: text; unnamed), the function taking the interned
/// handle (`00a5b950`, thiscall: the address of the handle; unnamed),
/// `NiGeometryData::CreateNormals` (`00a669f0`, engine map name) and
/// `bhkUtilFunctions::PrepareDebugGeom` (`00c811f0`, engine map name).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
const COPY_VERTEX: u32 = 0x0045_8620;
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
const INTERN_STRING: u32 = 0x00a5_b690;
const SET_INTERNED_NAME: u32 = 0x00a5_b950;
const CREATE_NORMALS: u32 = 0x00a6_69f0;
const PREPARE_DEBUG_GEOM: u32 = 0x00c8_11f0;
/// The interned empty string (`0109b220`): the one handle that is not released, and
/// `InterlockedDecrement` (import slot `00fdf1c4`) that releases the others at `handle - 8`.
const EMPTY_STRING: u32 = 0x0109_b220;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGetViewerString` (`00a7d330`, cdecl: name), its formatting overload (`00a7d4d0`,
/// `NiGetViewerString_ov7` in the engine map; cdecl: format, value) and the string array's
/// resize (`0096ad30`) and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_FORMATTED: u32 = 0x00a7_d4d0;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `bhkSerializable::GetViewerStrings` (engine map name, `00c878c0`).
const SERIALIZABLE_GET_VIEWER_STRINGS: u32 = 0x00c8_78c0;
/// The vtable `~bhkShape` stores and the object counter it lowers.
const SHAPE_VTABLE: u32 = 0x0103_0bf4;
const SHAPE_COUNT: u32 = 0x0126_80f8;
/// The class name word (`01268100`) `GetViewerStrings` passes to `NiGetViewerString` and the
/// format string (`010c59cc`) it formats the material with.
const CLASS_RTTI: u32 = 0x0126_8100;
const MATERIAL_FORMAT: u32 = 0x010c_59cc;
/// The table of 12-byte entries `00c84cd0` copies from (`011b09d8`).
const ENTRY_TABLE: u32 = 0x011b_09d8;
/// The three floats `CanShare` compares the argument's `+0x10`, `+0x14` and `+0x18` with.
const SHARE_CONSTANTS: [u32; 3] = [0x011a_9490, 0x011a_9494, 0x011a_9498];
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

// Virtual slots (byte offsets).
/// Unconfirmed name (`bhkSerializable::PostInit`, `00c876d0`, jumps to this slot).
const VSLOT_POST_INIT: u32 = 0xa4;
/// `CreateCinfo` (out parameter: the address of a flag byte).
const VSLOT_CREATE_CINFO: u32 = 0xb4;

/// The material is the low five bits of the word at `+0x10`.
const MATERIAL_MASK: u32 = 0x1f;

/// Calls `CreateCinfo` (virtual `+0xb4`, with the address of a scratch flag word) and answers
/// the Cinfo and the flag word (the exe reads a whole dword of which only the flag byte is
/// written; the other bytes are zero here).
fn create_cinfo(e: &mut Engine, this: u32) -> (u32, u32) {
    e.with_stack(4, |e, created| {
        let cinfo = e.vcall(this, VSLOT_CREATE_CINFO, &args![created]).u32();
        (cinfo, e.mem.u32(created.addr()))
    })
}

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

/// Stores the string pointer `text` in the string array: grows the array by its grow-by
/// (`0096ad30`) when the size (`+0xa`) has reached the capacity (`+8`), then stores through
/// `0096ae90` (array, index = the old size, address of the pointer).
fn store_text(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
}

/// The `MUL` and `SETO`/`NEG`/`OR` sequence of an allocation size: the product, or all ones
/// when it does not fit in 32 bits.
fn allocation_size(count: u32, unit: u32) -> u32 {
    count.checked_mul(unit).unwrap_or(u32::MAX)
}

// Translated from 00c84c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c84c50 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkShape::~bhkShape` (engine map name): stores the vtable, runs `Kill` (`00c876a0`),
/// lowers the object counter (`012680f8`) and runs `~bhkSerializable` (`00c877b0`).
pub fn bhk_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SHAPE_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(SHAPE_COUNT);
    e.mem.set_u32(SHAPE_COUNT, count.wrapping_sub(1));
    e.call(SERIALIZABLE_DESTRUCT, &args![this]);
}

// Translated from 00c84cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::LinkObject` (engine map name), `(stream)`: jumps to `bhkSerializable::LinkObject`
/// (`00c87670`).
pub fn bhk_shape_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SERIALIZABLE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00c84cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkshape.obj`). Cdecl, `(index, out)`: copies the three
/// words of the entry `index` of the table of 12-byte entries at `011b09d8` to `out`.
pub fn fn_00c84cd0(e: &mut Engine, index: u32, out: Ptr) {
    let entry = ENTRY_TABLE.wrapping_add(index.wrapping_mul(3).wrapping_mul(4));
    for word in 0..3 {
        let value = e.mem.u32(entry.wrapping_add(4 * word));
        e.mem.set_u32(out.addr() + 4 * word, value);
    }
}

// Translated from 00c84d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::LoadBinary` (engine map name), `(stream)`: the `bhkSerializable` load
/// (`00c87650`); then, with the Cinfo (`CreateCinfo`), byte-swaps its first word unless the
/// byte at `stream + 0x1e5` is 1, replaces it by 0 if it is 0x24 or more (signed compare),
/// stores it as the material word (`+0x10`) and clears it in the Cinfo.
pub fn bhk_shape_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SERIALIZABLE_LOAD_BINARY, &args![this, stream]);
    let (cinfo, _) = create_cinfo(e, this.addr());
    let mut word = e.mem.u32(cinfo);
    if e.mem.u8(stream.addr() + 0x1e5) != 1 {
        word = word.swap_bytes();
    }
    if word as i32 >= 0x24 {
        word = 0;
    }
    e.mem.set_u32(this.addr() + 0x10, word);
    e.mem.set_u32(cinfo, 0);
}

// Translated from 00c84d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::SaveBinary` (engine map name), `(stream)`: with the Cinfo (`CreateCinfo`), puts
/// the material (`+0x10` masked with 0x1f) in its first word, byte-swapped unless the byte at
/// `stream + 0x1e4` is 1; runs `bhkSerializable::SaveBinary` (`00c87850`); swaps the word back
/// (the byte is read again) and runs the virtual at `+0xa4` with the flag word `CreateCinfo`
/// left.
pub fn bhk_shape_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (cinfo, flag) = create_cinfo(e, this.addr());
    let material = e.mem.u32(this.addr() + 0x10) & MATERIAL_MASK;
    e.mem.set_u32(cinfo, material);
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        e.mem.set_u32(cinfo, material.swap_bytes());
    }
    e.call(SERIALIZABLE_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        let word = e.mem.u32(cinfo);
        e.mem.set_u32(cinfo, word.swap_bytes());
    }
    e.vcall(this.addr(), VSLOT_POST_INIT, &args![flag]);
}

// Translated from 00c84dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::IsEqual` (engine map name), `(other)`: the base test (`00ba87d0`) and, if it
/// holds, whether the materials (the low five bits of `+0x10`) are the same.
pub fn bhk_shape_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let equal = e.call(BASE_IS_EQUAL, &args![this, other]).bool();
    if !equal {
        return false;
    }
    let difference = e.mem.u32(other.addr() + 0x10) ^ e.mem.u32(this.addr() + 0x10);
    difference & MATERIAL_MASK == 0
}

// Translated from 00c84df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::CopyMembers` (engine map name), `(clone, process)`: the `bhkSerializable` copy
/// (`00c87600`), then the material (`+0x10` masked with 0x1f) into the clone's `+0x10`.
pub fn bhk_shape_copy_members(e: &mut Engine, this: Ptr, clone: Ptr, process: Ptr) {
    e.call(SERIALIZABLE_COPY_MEMBERS, &args![this, clone, process]);
    let material = e.mem.u32(this.addr() + 0x10) & MATERIAL_MASK;
    e.mem.set_u32(clone.addr() + 0x10, material);
}

// Translated from 00c84e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::CopyCinfo` (engine map name), `(out)`: stores the word at `+8` of the Havok
/// object (`this + 8`) in `out`, or 0 when `this` or the Havok object is null.
pub fn bhk_shape_copy_cinfo(e: &mut Engine, this: Ptr, out: Ptr) {
    let mut value = 0;
    if this.addr() != 0 {
        let object = e.mem.u32(this.addr() + 8);
        if object != 0 {
            value = e.mem.u32(object + 8);
        }
    }
    e.mem.set_u32(out.addr(), value);
}

// Translated from 00c84e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::Link` (engine map name), `(flag)`: if `this` and its Havok object (`+8`) are set,
/// stores `this` (when the low byte of `flag` is not 0) or 0 in the word at `+8` of the Havok
/// object; then jumps to `bhkRefObject::Link` (`00c87c90`) with the same `flag`.
pub fn bhk_shape_link(e: &mut Engine, this: Ptr, flag: u32) {
    if this.addr() != 0 {
        let object = e.mem.u32(this.addr() + 8);
        if object != 0 {
            let owner = if flag & 0xff != 0 { this.addr() } else { 0 };
            e.mem.set_u32(object + 8, owner);
        }
    }
    e.call(REF_OBJECT_LINK, &args![this, flag]);
}

// Translated from 00c84e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::CalcMass` (engine map name), `(properties)`: answers true without doing
/// anything when `this` or its Havok object (`+8`) is null; otherwise runs
/// `computeShapeVolumeMassProperties` (`00cf0690`) with the Havok object, the float at
/// `properties + 4` (quieted by the `FLD`/`FSTP` copy) and `properties`, and answers false.
pub fn bhk_shape_calc_mass(e: &mut Engine, this: Ptr, properties: Ptr) -> bool {
    if this.addr() == 0 {
        return true;
    }
    let object = e.mem.u32(this.addr() + 8);
    if object == 0 {
        return true;
    }
    let volume = x87_quiet(e.mem.f32(properties.addr() + 4));
    e.call(
        COMPUTE_SHAPE_VOLUME_MASS_PROPERTIES,
        &args![object, volume.to_bits(), properties],
    );
    false
}

// Translated from 00c84ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkShape::CanShare` (engine map name), `(other)`: ignores `this`; true when the three
/// floats at `other + 0x10`, `+0x14` and `+0x18` equal the floats at `011a9490`, `011a9494` and
/// `011a9498` (the `FUCOMPP` test passes on "equal" only, so a NaN makes it false).
pub fn bhk_shape_can_share(e: &mut Engine, _this: Ptr, other: Ptr) -> bool {
    let mut offset = 0x10;
    for constant in SHARE_CONSTANTS {
        if e.mem.f32(other.addr() + offset) != e.mem.f32(constant) {
            return false;
        }
        offset += 4;
    }
    true
}

// Translated from 00c84f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkshape.obj`). Thiscall, one word
/// (`RET 4`). Unless `index` is `-1`, calls virtual slot `+0xcc` of `this`
/// and, when that returns an object, returns that object's virtual slot
/// `+0xe0` called with `index`. Otherwise returns `[this + 0x10] & 0x1f`.
pub fn fn_00c84f10(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    if index != -1 {
        let object = e.vcall(this.addr(), 0xcc, &args![]).u32();
        if object != 0 {
            return e.vcall(object, 0xe0, &args![index]).u32();
        }
    }
    e.mem.u32(this.addr() + 0x10) & 0x1f
}

// Translated from 00c84fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c84fa0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkShape::BuildDisplayFromhkGeometry` (engine map name), `(first_argument, geometry,
/// third_argument, name)`: inside the `TLS + 0x2b4 = 0x10` scope, and only when `geometry` is
/// set and has vertices (`+4` counts the 16-byte vectors at `+0`) and triangles (`+0x10`
/// counts the 16-byte records of three indices at `+0xc`): copies the vertices (`00458620`)
/// into a block of 12 bytes each and the triangle indices as 16-bit values into a block of
/// 6 bytes each (both from the memory manager, an overflowing size becoming all ones; the
/// fourth word of a triangle record is read and not used), makes a `NiTriShape` of them
/// (0xc4 bytes from the heap, `00a74410`), gives it the name (interned by `00a5b690`, handed
/// to `00a5b950` and released with `InterlockedDecrement` at `handle - 8` unless it is the
/// empty string `0109b220`) when `name` is set, creates the normals of its geometry data (the
/// word at `+0xb8`) and runs `PrepareDebugGeom` (`00c811f0`) with the shape, `first_argument`,
/// the material and `third_argument`. A failed allocation of the shape is not modeled: the
/// exe then reads through a null pointer.
pub fn bhk_shape_build_display_from_hk_geometry(
    e: &mut Engine,
    this: Ptr,
    first_argument: u32,
    geometry: Ptr,
    third_argument: u32,
    name: u32,
) {
    let scope = enter_scope(e);
    let geometry = geometry.addr();
    if geometry != 0 {
        let vertex_count = e.mem.u32(geometry + 4);
        let triangle_count = e.mem.u32(geometry + 0x10);
        if vertex_count != 0 && triangle_count != 0 {
            let vertices = e
                .call(
                    MEMORY_ALLOCATE,
                    &args![MEMORY_MANAGER, allocation_size(vertex_count, 12)],
                )
                .u32();
            let index_count = triangle_count.wrapping_mul(3);
            let triangles = e
                .call(
                    MEMORY_ALLOCATE,
                    &args![MEMORY_MANAGER, allocation_size(index_count, 2)],
                )
                .u32();
            for index in 0..vertex_count {
                let source = e.mem.u32(geometry).wrapping_add(index.wrapping_mul(16));
                let destination = vertices.wrapping_add(index.wrapping_mul(12));
                e.call(COPY_VERTEX, &args![destination, source]);
            }
            for index in 0..triangle_count {
                let record = e
                    .mem
                    .u32(geometry + 0xc)
                    .wrapping_add(index.wrapping_mul(16));
                let target = triangles.wrapping_add(index.wrapping_mul(6));
                for corner in 0..3 {
                    let value = e.mem.u32(record + 4 * corner);
                    e.mem.set_u16(target + 2 * corner, value as u16);
                }
            }
            let block = e.call(HEAP_ALLOCATE, &args![0xc4u32]).u32();
            let shape = e
                .call(
                    TRI_SHAPE_CONSTRUCT,
                    &args![
                        block,
                        vertex_count,
                        vertices,
                        0u32,
                        0u32,
                        0u32,
                        0u32,
                        0u32,
                        triangle_count,
                        triangles
                    ],
                )
                .u32();
            if name != 0 {
                let interned = e.call(INTERN_STRING, &args![name]).u32();
                let handle = e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), interned);
                    e.call(SET_INTERNED_NAME, &args![shape, slot]);
                    e.mem.u32(slot.addr())
                });
                if handle != e.mem.u32(EMPTY_STRING) {
                    e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
                }
            }
            let geometry_data = e.mem.u32(shape + 0xb8);
            e.call(CREATE_NORMALS, &args![geometry_data, 1u32]);
            let material = e.mem.u32(this.addr() + 0x10) & MATERIAL_MASK;
            e.call(
                PREPARE_DEBUG_GEOM,
                &args![shape, first_argument, material, third_argument],
            );
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c851c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c851c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkShape::GetViewerStrings` (engine map name), `(strings)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, the base strings (`00c878c0`), then the text
/// `NiGetViewerString` makes of the word at `01268100` and the text `00a7d4d0` makes of the
/// format `010c59cc` and the material (`+0x10` masked with 0x1f), each stored in the string
/// array (grown by `0096ad30` when its size, `+0xa`, has reached its capacity, `+8`).
pub fn bhk_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let scope = enter_scope(e);
    e.call(SERIALIZABLE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(CLASS_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    store_text(e, strings.addr(), text);
    let material = e.mem.u32(this.addr() + 0x10) & MATERIAL_MASK;
    let text = e
        .call(
            GET_VIEWER_STRING_FORMATTED,
            &args![MATERIAL_FORMAT, material],
        )
        .u32();
    store_text(e, strings.addr(), text);
    leave_scope(e, scope);
}

// Translated from 00c84f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the class is unconfirmed): `this` in `ECX`, two stack words
/// (`RET 8`): `selector` and `mask`. Unless `selector` is `0xffffffff` it follows the chain the
/// virtual function at +0xcc gives (each answer is the next object; null or the same object ends
/// it). Then it answers whether the word at `+0x10` of the last object, ANDed with `mask`, is
/// non-zero, compared as the value 0 or 1 with `mask` itself. Only `AL` is set (`MOV AL, DL`):
/// the upper bytes of `EAX` stay those of `mask`. Ghidra made no function here; it is called from
/// 005dbc1c.
pub fn fn_00c84f50(e: &mut Engine, this: u32, selector: u32, mask: u32) -> u32 {
    let mut object = this;
    if selector != 0xffff_ffff {
        loop {
            let next = e.vcall(object, 0xcc, &args![]).u32();
            if next == 0 || next == object {
                break;
            }
            object = next;
        }
    }
    let masked_non_zero = u32::from(e.mem.u32(object + 0x10) & mask != 0);
    (mask & 0xffff_ff00) | u32::from(masked_non_zero == mask)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c84c50, bhk_shape_destructor(Ptr)),
        entry!(0x00c84cc0, bhk_shape_link_object(Ptr, Ptr)),
        entry!(0x00c84cd0, fn_00c84cd0(u32, Ptr)),
        entry!(0x00c84d00, bhk_shape_load_binary(Ptr, Ptr)),
        entry!(0x00c84d50, bhk_shape_save_binary(Ptr, Ptr)),
        entry!(0x00c84dc0, bhk_shape_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c84df0, bhk_shape_copy_members(Ptr, Ptr, Ptr)),
        entry!(0x00c84e20, bhk_shape_copy_cinfo(Ptr, Ptr)),
        entry!(0x00c84e50, bhk_shape_link(Ptr, u32)),
        entry!(0x00c84e80, bhk_shape_calc_mass(Ptr, Ptr) -> bool),
        entry!(0x00c84ec0, bhk_shape_can_share(Ptr, Ptr) -> bool),
        entry!(0x00c84f10, fn_00c84f10(Ptr, i32) -> u32),
        entry!(
            0x00c84fa0,
            bhk_shape_build_display_from_hk_geometry(Ptr, u32, Ptr, u32, u32)
        ),
        entry!(0x00c851c0, bhk_shape_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c8_4f50, fn_00c84f50(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const POST_INIT_SLOT: u32 = 0x7000_00a4;
    const CREATE_CINFO_SLOT: u32 = 0x7000_00b4;
    const OBJECT_VTABLE: u32 = 0x7001_1000;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// A shape whose vtable has the slots the unit calls: `CreateCinfo` answers `cinfo` and
    /// sets the flag byte it is given the address of to `flag`.
    fn shape(e: &mut Engine, cinfo: u32, flag: u8) -> u32 {
        let mut slots = vec![0u32; 0x30];
        slots[0xa4 / 4] = POST_INIT_SLOT;
        slots[0xb4 / 4] = CREATE_CINFO_SLOT;
        e.put_vtable(OBJECT_VTABLE, &slots);
        e.register_double(CREATE_CINFO_SLOT, move |e, a| {
            e.mem.set_u8(a[1], flag);
            cinfo.into_ret()
        });
        e.register(POST_INIT_SLOT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, OBJECT_VTABLE);
        this
    }

    #[test]
    fn fn_00c84f10_asks_the_object_unless_the_index_is_minus_one() {
        let mut e = Engine::new();
        // Vtable of the shape: slot 0xcc returns the object.
        let mut slots = vec![0u32; 0x34];
        slots[0xcc / 4] = 0x0090_5000;
        e.put_vtable(0x0090_2000, &slots);
        let mut object_slots = vec![0u32; 0x39];
        object_slots[0xe0 / 4] = 0x0090_5100;
        e.put_vtable(0x0090_3000, &object_slots);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0090_3000);
        e.register_double(0x0090_5000, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        e.register(0x0090_5100, |_, a| Ret {
            eax: a[1] + 100,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x0090_2000);
        e.mem.set_u32(this + 0x10, 0xffff_ffe7);
        assert_eq!(e.call(0x00c8_4f10, &args![this, 5i32]).u32(), 105);
        assert_eq!(e.call(0x00c8_4f10, &args![this, -1i32]).u32(), 0x07);
    }

    #[test]
    fn fn_00c84f10_falls_back_to_the_low_five_bits_without_an_object() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x34];
        slots[0xcc / 4] = 0x0090_5000;
        e.put_vtable(0x0090_2000, &slots);
        e.register(0x0090_5000, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x0090_2000);
        e.mem.set_u32(this + 0x10, 0x1234_5678);
        assert_eq!(e.call(0x00c8_4f10, &args![this, 3i32]).u32(), 0x18);
    }

    #[test]
    fn bhk_shape_destructor_runs_kill_lowers_the_counter_and_runs_the_base() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(SERIALIZABLE_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(SHAPE_COUNT, 5);
        e.call(0x00c8_4c50, &args![this]);
        assert_eq!(e.mem.u32(this), SHAPE_VTABLE);
        assert_eq!(e.mem.u32(SHAPE_COUNT), 4);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, SERIALIZABLE_DESTRUCT), vec![vec![this]]);
        assert_eq!(
            order_of(&e, &[SERIALIZABLE_KILL, SERIALIZABLE_DESTRUCT]),
            vec![SERIALIZABLE_KILL, SERIALIZABLE_DESTRUCT]
        );
    }

    #[test]
    fn bhk_shape_link_object_forwards_to_the_serializable_link() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_LINK_OBJECT, |_, _| Ret::default());
        e.call(0x00c8_4cc0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_LINK_OBJECT),
            vec![vec![0x1000, 0x2000]]
        );
    }

    #[test]
    fn fn_00c84cd0_copies_the_indexed_twelve_byte_entry() {
        let mut e = logged_engine();
        for (index, words) in [(0u32, [1u32, 2, 3]), (2, [7, 8, 9])] {
            for (n, word) in words.into_iter().enumerate() {
                e.mem.set_u32(ENTRY_TABLE + index * 12 + 4 * n as u32, word);
            }
        }
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 0xc, 0x55);
        e.call(0x00c8_4cd0, &args![2u32, out]);
        assert_eq!(
            [e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)],
            [7, 8, 9]
        );
        assert_eq!(e.mem.u32(out + 0xc), 0x55);
        e.call(0x00c8_4cd0, &args![0u32, out]);
        assert_eq!(
            [e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)],
            [1, 2, 3]
        );
    }

    #[test]
    fn bhk_shape_load_binary_takes_the_material_from_the_cinfo() {
        // (stream byte at +0x1e5, word in the cinfo, material stored)
        let cases = [
            (1u8, 0x0000_0005u32, 5u32),
            (0, 0x0500_0000, 5),
            (1, 0x24, 0),
            (1, 0x23, 0x23),
            (1, 0xffff_fffe, 0xffff_fffe),
            (0, 0x2400_0000, 0),
        ];
        for (byte, word, stored) in cases {
            let mut e = logged_engine();
            e.register(SERIALIZABLE_LOAD_BINARY, |_, _| Ret::default());
            let cinfo = e.mem.alloc(0x10);
            e.mem.set_u32(cinfo, word);
            let this = shape(&mut e, cinfo, 0);
            e.mem.set_u32(this + 0x10, 0x77);
            let stream = e.mem.alloc(0x200);
            e.mem.set_u8(stream + 0x1e5, byte);
            e.call(0x00c8_4d00, &args![this, stream]);
            assert_eq!(
                calls_to(&e, SERIALIZABLE_LOAD_BINARY),
                vec![vec![this, stream]]
            );
            assert_eq!(e.mem.u32(this + 0x10), stored, "{byte} {word:x}");
            assert_eq!(e.mem.u32(cinfo), 0);
        }
    }

    #[test]
    fn bhk_shape_save_binary_writes_the_material_around_the_base_save() {
        for (byte, during) in [(1u8, 0x0000_0007u32), (0, 0x0700_0000)] {
            let mut e = logged_engine();
            let cinfo = e.mem.alloc(0x10);
            e.register_double(SERIALIZABLE_SAVE_BINARY, move |e, _| {
                assert_eq!(e.mem.u32(cinfo), during);
                Ret::default()
            });
            let this = shape(&mut e, cinfo, 1);
            e.mem.set_u32(this + 0x10, 0xffff_ffe7);
            let stream = e.mem.alloc(0x200);
            e.mem.set_u8(stream + 0x1e4, byte);
            e.call(0x00c8_4d50, &args![this, stream]);
            assert_eq!(
                calls_to(&e, SERIALIZABLE_SAVE_BINARY),
                vec![vec![this, stream]]
            );
            assert_eq!(e.mem.u32(cinfo), 7);
            assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, 1]]);
            assert_eq!(
                order_of(&e, &[SERIALIZABLE_SAVE_BINARY, POST_INIT_SLOT]),
                vec![SERIALIZABLE_SAVE_BINARY, POST_INIT_SLOT]
            );
        }
    }

    #[test]
    fn bhk_shape_is_equal_compares_the_materials_when_the_base_test_holds() {
        // (base answer, this +0x10, other +0x10, result)
        let cases = [
            (true, 0x25u32, 0x05u32, true),
            (true, 0x05, 0x06, false),
            (false, 0x05, 0x05, false),
        ];
        for (base, mine, theirs, expected) in cases {
            let mut e = logged_engine();
            e.register_double(BASE_IS_EQUAL, move |_, _| u32::from(base).into_ret());
            let this = e.mem.alloc(0x20);
            let other = e.mem.alloc(0x20);
            e.mem.set_u32(this + 0x10, mine);
            e.mem.set_u32(other + 0x10, theirs);
            let result = e.call(0x00c8_4dc0, &args![this, other]).bool();
            assert_eq!(result, expected);
            assert_eq!(calls_to(&e, BASE_IS_EQUAL), vec![vec![this, other]]);
        }
    }

    #[test]
    fn bhk_shape_copy_members_copies_the_material_to_the_clone() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let clone = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x1234_5679);
        e.mem.set_u32(clone + 0x10, 0xffff_ffff);
        e.call(0x00c8_4df0, &args![this, clone, 0x9000u32]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_COPY_MEMBERS),
            vec![vec![this, clone, 0x9000]]
        );
        assert_eq!(e.mem.u32(clone + 0x10), 0x19);
    }

    #[test]
    fn bhk_shape_copy_cinfo_answers_the_word_of_the_havok_object_or_zero() {
        let mut e = logged_engine();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, 0xabcd);
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x11);
        e.call(0x00c8_4e20, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0);
        e.mem.set_u32(this + 8, object);
        e.call(0x00c8_4e20, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0xabcd);
        e.call(0x00c8_4e20, &args![0u32, out]);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn bhk_shape_link_marks_the_havok_object_then_jumps_to_the_base_link() {
        let mut e = logged_engine();
        e.register(REF_OBJECT_LINK, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, object);
        e.mem.set_u32(object + 8, 0x99);
        e.call(0x00c8_4e50, &args![this, 1u32]);
        assert_eq!(e.mem.u32(object + 8), this);
        e.call(0x00c8_4e50, &args![this, 0x100u32]);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(
            calls_to(&e, REF_OBJECT_LINK),
            vec![vec![this, 1], vec![this, 0x100]]
        );
    }

    #[test]
    fn bhk_shape_link_skips_the_store_without_a_havok_object() {
        let mut e = logged_engine();
        e.register(REF_OBJECT_LINK, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call(0x00c8_4e50, &args![this, 1u32]);
        e.call(0x00c8_4e50, &args![0u32, 1u32]);
        assert_eq!(
            calls_to(&e, REF_OBJECT_LINK),
            vec![vec![this, 1], vec![0, 1]]
        );
    }

    #[test]
    fn bhk_shape_calc_mass_computes_the_properties_of_the_havok_shape() {
        let mut e = logged_engine();
        e.register(COMPUTE_SHAPE_VOLUME_MASS_PROPERTIES, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x20);
        let properties = e.mem.alloc(0x10);
        e.mem.set_f32(properties + 4, 2.5);
        e.mem.set_u32(this + 8, object);
        assert!(!e.call(0x00c8_4e80, &args![this, properties]).bool());
        assert_eq!(
            calls_to(&e, COMPUTE_SHAPE_VOLUME_MASS_PROPERTIES),
            vec![vec![object, 2.5f32.to_bits(), properties]]
        );
    }

    #[test]
    fn bhk_shape_calc_mass_quiets_a_signalling_float() {
        let mut e = logged_engine();
        e.register(COMPUTE_SHAPE_VOLUME_MASS_PROPERTIES, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x20);
        let properties = e.mem.alloc(0x10);
        e.mem.set_u32(properties + 4, 0x7f80_0001);
        e.mem.set_u32(this + 8, object);
        e.call(0x00c8_4e80, &args![this, properties]);
        assert_eq!(
            calls_to(&e, COMPUTE_SHAPE_VOLUME_MASS_PROPERTIES),
            vec![vec![object, 0x7fc0_0001, properties]]
        );
    }

    #[test]
    fn bhk_shape_calc_mass_answers_true_without_a_shape() {
        let mut e = logged_engine();
        e.register(COMPUTE_SHAPE_VOLUME_MASS_PROPERTIES, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let properties = e.mem.alloc(0x10);
        assert!(e.call(0x00c8_4e80, &args![this, properties]).bool());
        assert!(e.call(0x00c8_4e80, &args![0u32, properties]).bool());
        assert!(calls_to(&e, COMPUTE_SHAPE_VOLUME_MASS_PROPERTIES).is_empty());
    }

    #[test]
    fn bhk_shape_can_share_needs_all_three_floats_equal() {
        let mut e = logged_engine();
        let constants = [1.0f32, 2.0, 3.0];
        for (address, value) in SHARE_CONSTANTS.into_iter().zip(constants) {
            e.set_global(address, value);
        }
        let other = e.mem.alloc(0x20);
        for (n, value) in constants.into_iter().enumerate() {
            e.mem.set_f32(other + 0x10 + 4 * n as u32, value);
        }
        assert!(e.call(0x00c8_4ec0, &args![0u32, other]).bool());
        for n in 0..3u32 {
            let saved = e.mem.u32(other + 0x10 + 4 * n);
            e.mem.set_f32(other + 0x10 + 4 * n, 9.0);
            assert!(!e.call(0x00c8_4ec0, &args![0u32, other]).bool(), "{n}");
            e.mem.set_f32(other + 0x10 + 4 * n, f32::NAN);
            assert!(!e.call(0x00c8_4ec0, &args![0u32, other]).bool(), "{n}");
            e.mem.set_u32(other + 0x10 + 4 * n, saved);
        }
        assert!(e.call(0x00c8_4ec0, &args![0u32, other]).bool());
    }

    #[test]
    fn allocation_size_saturates_on_overflow() {
        assert_eq!(allocation_size(3, 12), 36);
        assert_eq!(allocation_size(0x2000_0000, 12), u32::MAX);
        assert_eq!(allocation_size(0x8000_0000, 2), u32::MAX);
        assert_eq!(allocation_size(0x7fff_ffff, 2), 0xffff_fffe);
    }

    const NAME_TEXT: u32 = 0x7002_0000;

    /// Doubles for everything `BuildDisplayFromhkGeometry` calls. The allocations are real
    /// blocks, the shape's geometry data pointer (`+0xb8`) is `0x7777`, the interned handle is
    /// `0x5000_0100`.
    fn display_engine() -> Engine {
        let mut e = logged_engine();
        e.register_double(MEMORY_ALLOCATE, |e, a| {
            let thread = e.tls();
            assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x10);
            e.mem.alloc(a[1]).into_ret()
        });
        e.register(COPY_VERTEX, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        e.register(HEAP_ALLOCATE, |e, _| e.mem.alloc(0xc4).into_ret());
        e.register(TRI_SHAPE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xb8, 0x7777);
            a[0].into_ret()
        });
        e.register(INTERN_STRING, |_, _| 0x5000_0100u32.into_ret());
        e.register(SET_INTERNED_NAME, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(CREATE_NORMALS, |_, _| Ret::default());
        e.register(PREPARE_DEBUG_GEOM, |_, _| Ret::default());
        e
    }

    /// A geometry of three vertices (16 bytes each, x = 1, 2, 3) and two triangles.
    fn geometry(e: &mut Engine) -> u32 {
        let vertices = e.mem.alloc(0x30);
        for n in 0..3u32 {
            e.mem.set_f32(vertices + 16 * n, (n + 1) as f32);
            e.mem.set_f32(vertices + 16 * n + 4, 10.0);
            e.mem.set_f32(vertices + 16 * n + 8, 20.0);
        }
        let triangles = e.mem.alloc(0x20);
        for (n, indices) in [[0u32, 1, 2], [2, 1, 0x1_0003]].into_iter().enumerate() {
            for (corner, index) in indices.into_iter().enumerate() {
                e.mem
                    .set_u32(triangles + 16 * n as u32 + 4 * corner as u32, index);
            }
        }
        let geometry = e.mem.alloc(0x20);
        e.mem.set_u32(geometry, vertices);
        e.mem.set_u32(geometry + 4, 3);
        e.mem.set_u32(geometry + 0xc, triangles);
        e.mem.set_u32(geometry + 0x10, 2);
        geometry
    }

    #[test]
    fn bhk_shape_build_display_builds_and_prepares_the_tri_shape() {
        let mut e = display_engine();
        let geometry = geometry(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0xffff_ffe9);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(
            0x00c8_4fa0,
            &args![this, 0x1111u32, geometry, 0x3333u32, 0u32],
        );
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 36], vec![MEMORY_MANAGER, 12]]
        );
        assert_eq!(calls_to(&e, COPY_VERTEX).len(), 3);
        let constructed = calls_to(&e, TRI_SHAPE_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        let args = &constructed[0];
        let (shape, vertices, triangles) = (args[0], args[2], args[9]);
        assert_eq!(args[1], 3);
        assert_eq!(args[3..8], [0, 0, 0, 0, 0]);
        assert_eq!(args[8], 2);
        for (n, x) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            let at = vertices + 12 * n as u32;
            assert_eq!(e.mem.f32(at), x);
            assert_eq!(e.mem.f32(at + 4), 10.0);
            assert_eq!(e.mem.f32(at + 8), 20.0);
        }
        let indices: Vec<u16> = (0..6).map(|n| e.mem.u16(triangles + 2 * n)).collect();
        assert_eq!(indices, vec![0, 1, 2, 2, 1, 3]);
        assert!(calls_to(&e, INTERN_STRING).is_empty());
        assert_eq!(calls_to(&e, CREATE_NORMALS), vec![vec![0x7777, 1]]);
        assert_eq!(
            calls_to(&e, PREPARE_DEBUG_GEOM),
            vec![vec![shape, 0x1111, 9, 0x3333]]
        );
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_shape_build_display_names_the_shape_and_releases_the_handle() {
        let mut e = display_engine();
        let geometry = geometry(&mut e);
        let this = e.mem.alloc(0x20);
        e.register_double(SET_INTERNED_NAME, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x5000_0100);
            Ret::default()
        });
        e.call(0x00c8_4fa0, &args![this, 1u32, geometry, 2u32, NAME_TEXT]);
        assert_eq!(calls_to(&e, INTERN_STRING), vec![vec![NAME_TEXT]]);
        let named = calls_to(&e, SET_INTERNED_NAME);
        assert_eq!(named.len(), 1);
        let shape = calls_to(&e, TRI_SHAPE_CONSTRUCT)[0][0];
        assert_eq!(named[0][0], shape);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![0x5000_0100 - 8]]
        );
        assert_eq!(
            order_of(
                &e,
                &[
                    TRI_SHAPE_CONSTRUCT,
                    INTERN_STRING,
                    SET_INTERNED_NAME,
                    INTERLOCKED_DECREMENT,
                    CREATE_NORMALS,
                    PREPARE_DEBUG_GEOM
                ]
            ),
            vec![
                TRI_SHAPE_CONSTRUCT,
                INTERN_STRING,
                SET_INTERNED_NAME,
                INTERLOCKED_DECREMENT,
                CREATE_NORMALS,
                PREPARE_DEBUG_GEOM
            ]
        );
    }

    #[test]
    fn bhk_shape_build_display_keeps_the_empty_string_handle() {
        let mut e = display_engine();
        let geometry = geometry(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(EMPTY_STRING, 0x5000_0100);
        e.call(0x00c8_4fa0, &args![this, 1u32, geometry, 2u32, NAME_TEXT]);
        assert_eq!(calls_to(&e, SET_INTERNED_NAME).len(), 1);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn bhk_shape_build_display_does_nothing_without_vertices_or_triangles() {
        let mut e = display_engine();
        let this = e.mem.alloc(0x20);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00c8_4fa0, &args![this, 1u32, 0u32, 2u32, 0u32]);
        let empty_vertices = geometry(&mut e);
        e.mem.set_u32(empty_vertices + 4, 0);
        e.call(0x00c8_4fa0, &args![this, 1u32, empty_vertices, 2u32, 0u32]);
        let empty_triangles = geometry(&mut e);
        e.mem.set_u32(empty_triangles + 0x10, 0);
        e.call(0x00c8_4fa0, &args![this, 1u32, empty_triangles, 2u32, 0u32]);
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
        assert!(calls_to(&e, PREPARE_DEBUG_GEOM).is_empty());
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_shape_get_viewer_strings_stores_the_class_name_and_the_material() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(GET_VIEWER_STRING_FORMATTED, |_, _| 0x2222u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        e.mem.set_u32(CLASS_RTTI, 0x4242);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x25);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 1);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00c8_51c0, &args![this, array]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4242]]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_FORMATTED),
            vec![vec![MATERIAL_FORMAT, 5]]
        );
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 5]]);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(base + 4), 0x2222);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    /// Objects whose virtual function at +0xcc answers the word at +0x20 (the next link).
    fn linked_objects(e: &mut Engine, flags: &[u32]) -> Vec<u32> {
        let vtable = e.mem.alloc(0xd0);
        e.mem.set_u32(vtable + 0xcc, 0x0900_0004);
        e.register_double(0x0900_0004, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x20),
            ..Ret::default()
        });
        let objects: Vec<u32> = flags.iter().map(|_| e.mem.alloc(0x40)).collect();
        for (i, object) in objects.iter().enumerate() {
            e.mem.set_u32(*object, vtable);
            e.mem.set_u32(*object + 0x10, flags[i]);
            if i + 1 < objects.len() {
                e.mem.set_u32(*object + 0x20, objects[i + 1]);
            }
        }
        objects
    }

    /// The last object of the chain decides, unless the selector is `0xffffffff`.
    #[test]
    fn fn_00c84f50_tests_the_end_of_the_chain() {
        let mut e = Engine::new();
        let objects = linked_objects(&mut e, &[0, 0, 1]);
        assert_eq!(e.call(0x00c8_4f50, &args![objects[0], 0u32, 1u32]).u32(), 1);
        assert_eq!(
            e.call(0x00c8_4f50, &args![objects[0], 0xffff_ffffu32, 1u32])
                .u32(),
            0
        );
    }

    /// A mask other than 0 or 1 never equals the 0/1 answer; the upper bytes of `EAX` are the mask's.
    #[test]
    fn fn_00c84f50_keeps_the_upper_bytes_of_the_mask() {
        let mut e = Engine::new();
        let objects = linked_objects(&mut e, &[0x1ff]);
        assert_eq!(
            e.call(0x00c8_4f50, &args![objects[0], 0u32, 0x100u32])
                .u32(),
            0x100
        );
        assert_eq!(
            e.call(0x00c8_4f50, &args![objects[0], 0u32, 0x2u32]).u32(),
            0
        );
        // A mask of 0 gives "no bits set" == 0.
        assert_eq!(e.call(0x00c8_4f50, &args![objects[0], 0u32, 0u32]).u32(), 1);
    }

    /// A link that points back at the object itself ends the chain.
    #[test]
    fn fn_00c84f50_stops_when_the_link_is_the_object_itself() {
        let mut e = Engine::new();
        let objects = linked_objects(&mut e, &[1, 0]);
        e.mem.set_u32(objects[0] + 0x20, objects[0]);
        assert_eq!(e.call(0x00c8_4f50, &args![objects[0], 0u32, 1u32]).u32(), 1);
    }
}
