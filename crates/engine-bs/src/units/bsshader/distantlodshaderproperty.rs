//! `BSShader/distantlodshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The cache word (`011f91c0`): 0 turns the cached geometry off; below 2 (signed) a cached
/// geometry holds at most 0x50 copies of the source, else at most 0xe4.
const CACHE_LEVEL: u32 = 0x011f_91c0;
/// The local stream: `NiStream`'s constructor (`00a66150`, `thiscall`, no words), the
/// `BSStream` vtable (`01016904`), `BSStream::~BSStream` (`0043d090`) and `BSStream::Load`
/// (`00c3a8a0`, `thiscall`, the file name and 0, a byte).
const STREAM_CONSTRUCT: u32 = 0x00a6_6150;
const BS_STREAM_VTABLE: u32 = 0x0101_6904;
const BS_STREAM_DESTRUCT: u32 = 0x0043_d090;
const BS_STREAM_LOAD: u32 = 0x00c3_a8a0;
/// The geometry cache: `00b58670` (no words) answers it; `00e92760` (`thiscall`, the key and
/// the address of a smart-pointer word, a byte) looks a key up; `00b582b0` (`thiscall`, the
/// key and a smart pointer by value) stores one.
const CACHE: u32 = 0x00b5_8670;
const CACHE_FIND: u32 = 0x00e9_2760;
const CACHE_INSERT: u32 = 0x00b5_82b0;
/// The kind cast (`00653290`, `thiscall` on the object, the RTTI object, the object or 0)
/// and the RTTI objects it is asked about: the strips geometry (`011f4a20`), the shape data
/// (`011f4a88`) and the strips data (`011f4a70`); the root kind that marks the cached
/// geometry (`011f4aa0`).
const KIND_CAST: u32 = 0x0065_3290;
const RTTI_STRIPS: u32 = 0x011f_4a20;
const RTTI_SHAPE_DATA: u32 = 0x011f_4a88;
const RTTI_STRIPS_DATA: u32 = 0x011f_4a70;
const RTTI_MARKED_ROOT: u32 = 0x011f_4aa0;
/// `NiTriShapeData::GetTriList` (`00a7b850`) and `NiTriStripsData::GetStripLists`
/// (`00a75f10`), both `thiscall`, no words.
const GET_TRI_LIST: u32 = 0x00a7_b850;
const GET_STRIP_LISTS: u32 = 0x00a7_5f10;
/// The memory manager's allocation (`00aa3e40`, `thiscall` on `011f6238`, the size), the
/// array constructor walk (`00401050`: block, element size, count, constructor) with the
/// colour constructor (`004a7800`), `memcpy` (`00ec44d0`, `cdecl`) and the object
/// allocation (`00aa13e0`, `cdecl`, the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const ARRAY_CONSTRUCT: u32 = 0x0040_1050;
const COLOR_CONSTRUCT: u32 = 0x004a_7800;
const MEMCPY: u32 = 0x00ec_44d0;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiTriShapeDynamicData::NiTriShapeDynamicData` (`00a816a0`, eleven words, an object of
/// 0x5c bytes) and the strips data constructor (`00a92340`, thirteen words, 0x54 bytes).
const SHAPE_DATA_CONSTRUCT: u32 = 0x00a8_16a0;
const STRIPS_DATA_CONSTRUCT: u32 = 0x00a9_2340;
/// `NiAVObject::GetProperty` (`00a59d30`, the property kind) and
/// `NiAVObject::RemoveProperty` (`00a5b230`, the property kind).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const REMOVE_PROPERTY: u32 = 0x00a5_b230;
/// The alpha property: the property base constructor (`00a5c200`, `thiscall`, no words),
/// its vtable (`010162dc`) and default flags (0xec).
const PROPERTY_CONSTRUCT: u32 = 0x00a5_c200;
const ALPHA_PROPERTY_VTABLE: u32 = 0x0101_62dc;
const ALPHA_DEFAULT_FLAGS: u16 = 0xec;
/// The cached geometry (0x34 bytes): the reference-counted base vtable (`0101dce4`), the
/// object count it bumps (`011f4400`) and its own vtable (`010adbc8`).
const CACHED_BASE_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT: u32 = 0x011f_4400;
const CACHED_VTABLE: u32 = 0x010a_dbc8;
/// `InterlockedIncrement` and `InterlockedDecrement` (imports).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The bounded formatter (`00406d00`, `cdecl`: buffer, size, format, the arguments) and the
/// format the missing-geometry text is made with (`010adbd0`).
const FORMAT: u32 = 0x0040_6d00;
const MISSING_FORMAT: u32 = 0x010a_dbd0;
/// The function's local frame (`SUB ESP,0x730` below the exception record).
const FRAME_SIZE: u32 = 0x730;

/// A reference-counted object released: `InterlockedDecrement` on its count at +4, and
/// deleted through its vtable slot +4 when that answers 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// A smart-pointer word at `field` given `object`: nothing when it already holds it; else
/// the old object released, the new one stored and its count bumped (only when it is not
/// null, if `null_checked`; the code bumps unconditionally for the alpha properties).
fn assign(e: &mut Engine, field: u32, object: u32, null_checked: bool) {
    let old = e.mem.u32(field);
    if old == object {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(field, object);
    if !null_checked || object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// An array of `count` elements of `size` bytes from the memory manager: the size is all
/// ones when the product overflows 32 bits.
fn allocate(e: &mut Engine, count: u32, size: u32) -> u32 {
    let bytes = u32::try_from(u64::from(count) * u64::from(size)).unwrap_or(u32::MAX);
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32()
}

/// A new alpha property: allocated (0x1c bytes), constructed with the default flags and a
/// zero test reference when the allocation did not fail.
fn new_alpha_property(e: &mut Engine) -> u32 {
    let property = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    if property == 0 {
        return 0;
    }
    e.call(PROPERTY_CONSTRUCT, &args![property]);
    e.mem.set_u32(property, ALPHA_PROPERTY_VTABLE);
    e.mem.set_u16(property + 0x18, ALPHA_DEFAULT_FLAGS);
    e.mem.set_u8(property + 0x1a, 0);
    property
}

/// What the copying made: the new geometry data (0 when its allocation failed), whether
/// it is strips, the per-vertex copy numbers, the vertex count of one copy, the number of
/// copies and the word kept at +0x32 (the triangles of one copy for a shape, one copy's
/// strip length plus its join for strips).
struct Copies {
    data: u32,
    strips: bool,
    copy_numbers: u32,
    vertex_count: u32,
    copies: u32,
    word: u16,
}

/// The source's vertices, normals, colours (16 bytes each) and texture coordinates as the
/// geometry data holds them (+0x20 to +0x2c).
struct Source {
    vertices: u32,
    normals: u32,
    colors: u32,
    texture: u32,
}

/// The number of copies `limit` allows, cut to what keeps `per_copy` times the copies
/// within 0xffff (`IDIV`, compared as words).
fn cut(limit: u32, per_copy: u32) -> u32 {
    let most = 0xffff / per_copy;
    if limit as u16 >= most as u16 {
        most & 0xffff
    } else {
        limit
    }
}

/// The shape copy: the shape data's vertices, normals, colours and texture coordinates
/// copied once per copy, each copy's vertices numbered with the copy (as a `float`), and the
/// triangle list repeated with each copy's indices moved past the previous copies'; then a
/// `NiTriShapeDynamicData` over the copies.
fn copy_shape(e: &mut Engine, data: u32, limit: u32) -> Copies {
    let shape = if data == 0 {
        0
    } else {
        e.call(KIND_CAST, &args![data, RTTI_SHAPE_DATA]).u32()
    };
    let source = Source {
        vertices: e.mem.u32(shape.wrapping_add(0x20)),
        normals: e.mem.u32(shape.wrapping_add(0x24)),
        colors: e.mem.u32(shape.wrapping_add(0x28)),
        texture: e.mem.u32(shape.wrapping_add(0x2c)),
    };
    let vertex_count = u32::from(e.mem.u16(shape.wrapping_add(8)));
    let list_length = e.mem.u32(shape.wrapping_add(0x44));
    let triangle_list = e.call(GET_TRI_LIST, &args![shape]).u32();
    let limit = cut(limit, vertex_count);
    let index_count = list_length & 0xffff;
    let limit = cut(limit, index_count);
    let copies = limit & 0xffff;
    let total = copies.wrapping_mul(vertex_count);
    let vertices = allocate(e, total, 0xc);
    let normals = allocate(e, total, 0xc);
    let block = allocate(e, total, 0x10);
    let colors = if block != 0 {
        e.call(
            ARRAY_CONSTRUCT,
            &args![block, 0x10u32, total, COLOR_CONSTRUCT],
        );
        block
    } else {
        0
    };
    let texture = allocate(e, total, 8);
    let copy_numbers = allocate(e, total, 4);
    let indices = allocate(e, index_count.wrapping_mul(copies), 2);
    let mut written = 0u32;
    for copy in 0..copies {
        let first = copy.wrapping_mul(vertex_count);
        if vertex_count > 0 {
            let number = x87_store32(f64::from(copy));
            for k in 0..vertex_count {
                let at = first.wrapping_add(k);
                for w in 0..3u32 {
                    let v = e.mem.u32(source.vertices.wrapping_add(k * 0xc + 4 * w));
                    e.mem
                        .set_u32(vertices.wrapping_add(at.wrapping_mul(0xc) + 4 * w), v);
                }
                for w in 0..3u32 {
                    let v = e.mem.u32(source.normals.wrapping_add(k * 0xc + 4 * w));
                    e.mem
                        .set_u32(normals.wrapping_add(at.wrapping_mul(0xc) + 4 * w), v);
                }
                for w in 0..4u32 {
                    let v = e.mem.u32(source.colors.wrapping_add(k * 0x10 + 4 * w));
                    e.mem
                        .set_u32(colors.wrapping_add(at.wrapping_mul(0x10) + 4 * w), v);
                }
                for w in 0..2u32 {
                    let v = e.mem.u32(source.texture.wrapping_add(k * 8 + 4 * w));
                    e.mem
                        .set_u32(texture.wrapping_add(at.wrapping_mul(8) + 4 * w), v);
                }
                e.mem
                    .set_u32(copy_numbers.wrapping_add(at.wrapping_mul(4)), number);
            }
        }
        for j in 0..index_count {
            let index = e
                .mem
                .u16(triangle_list.wrapping_add(j * 2))
                .wrapping_add(first as u16);
            e.mem
                .set_u16(indices.wrapping_add(written.wrapping_mul(2)), index);
            written = written.wrapping_add(1);
        }
    }
    let triangles = index_count / 3;
    let object = e.call(NI_ALLOCATE, &args![0x5cu32]).u32();
    let made = if object == 0 {
        0
    } else {
        let active = copies.wrapping_mul(vertex_count);
        e.call(
            SHAPE_DATA_CONSTRUCT,
            &args![
                object,
                active,
                vertices,
                normals,
                colors,
                texture,
                1u32,
                0u32,
                triangles.wrapping_mul(copies),
                indices,
                active,
                0u32
            ],
        )
        .u32()
    };
    Copies {
        data: made,
        strips: false,
        copy_numbers,
        vertex_count,
        copies,
        word: triangles as u16,
    }
}

/// The strips copy: the strips data's arrays copied once per copy (`memcpy`), each copy's
/// vertices numbered with the copy (as a `float`), and the first strip repeated with each
/// copy's indices moved past the previous copies', joined to the next copy by repeating
/// its last index and the next copy's first (twice for an odd strip length); then a strips
/// data with one strip over the copies.
fn copy_strips(e: &mut Engine, data: u32, limit: u32) -> Copies {
    let strips = if data == 0 {
        0
    } else {
        e.call(KIND_CAST, &args![data, RTTI_STRIPS_DATA]).u32()
    };
    let source = Source {
        vertices: e.mem.u32(strips.wrapping_add(0x20)),
        normals: e.mem.u32(strips.wrapping_add(0x24)),
        colors: e.mem.u32(strips.wrapping_add(0x28)),
        texture: e.mem.u32(strips.wrapping_add(0x2c)),
    };
    let vertex_count = u32::from(e.mem.u16(strips.wrapping_add(8)));
    let lengths = e.mem.u32(strips.wrapping_add(0x48));
    let strip_list = e.call(GET_STRIP_LISTS, &args![strips]).u32();
    let limit = cut(limit, vertex_count);
    let length = u32::from(e.mem.u16(lengths));
    let limit = cut(limit, length + 2 + (length & 1));
    let copies = limit & 0xffff;
    let total = copies.wrapping_mul(vertex_count);
    let vertices = allocate(e, total, 0xc);
    let normals = allocate(e, total, 0xc);
    let block = allocate(e, total, 0x10);
    let colors = if block != 0 {
        e.call(
            ARRAY_CONSTRUCT,
            &args![block, 0x10u32, total, COLOR_CONSTRUCT],
        );
        block
    } else {
        0
    };
    let texture = allocate(e, total, 8);
    let copy_numbers = allocate(e, total, 4);
    let length = e.mem.u16(lengths);
    // The stride is a word sum (`ADD DX,AX`).
    let stride = u32::from(length.wrapping_add(2 + (length & 1)));
    let index_total = copies
        .wrapping_sub(1)
        .wrapping_mul(stride)
        .wrapping_add(u32::from(length));
    let indices = allocate(e, index_total, 2);
    let mut written = 0u32;
    let last = copies.wrapping_sub(1);
    for copy in 0..copies {
        let first = copy.wrapping_mul(vertex_count);
        e.call(
            MEMCPY,
            &args![
                vertices.wrapping_add(first.wrapping_mul(0xc)),
                source.vertices,
                vertex_count * 0xc
            ],
        );
        e.call(
            MEMCPY,
            &args![
                normals.wrapping_add(first.wrapping_mul(0xc)),
                source.normals,
                vertex_count * 0xc
            ],
        );
        e.call(
            MEMCPY,
            &args![
                colors.wrapping_add(first.wrapping_mul(0x10)),
                source.colors,
                vertex_count * 0x10
            ],
        );
        e.call(
            MEMCPY,
            &args![
                texture.wrapping_add(first.wrapping_mul(8)),
                source.texture,
                vertex_count * 8
            ],
        );
        if vertex_count > 0 {
            let number = x87_store32(f64::from(copy));
            for k in 0..vertex_count {
                let at = first.wrapping_add(k);
                e.mem
                    .set_u32(copy_numbers.wrapping_add(at.wrapping_mul(4)), number);
            }
        }
        if e.mem.u16(lengths) != 0 {
            let shift = copy.wrapping_mul(vertex_count) as u16;
            let mut j = 0u32;
            loop {
                let index = e
                    .mem
                    .u16(strip_list.wrapping_add(j * 2))
                    .wrapping_add(shift);
                e.mem
                    .set_u16(indices.wrapping_add(written.wrapping_mul(2)), index);
                j += 1;
                written = written.wrapping_add(1);
                if j >= u32::from(e.mem.u16(lengths)) {
                    break;
                }
            }
        }
        if copy < last {
            let length = u32::from(e.mem.u16(lengths));
            let this_shift = (copy & 0xffff).wrapping_mul(vertex_count);
            let tail = e
                .mem
                .u16(strip_list.wrapping_add(length * 2).wrapping_sub(2));
            e.mem.set_u16(
                indices.wrapping_add(written.wrapping_mul(2)),
                (this_shift as u16).wrapping_add(tail),
            );
            written = written.wrapping_add(1);
            let next_shift = ((copy & 0xffff) + 1).wrapping_mul(vertex_count) as u16;
            let head = e.mem.u16(strip_list).wrapping_add(next_shift);
            e.mem
                .set_u16(indices.wrapping_add(written.wrapping_mul(2)), head);
            written = written.wrapping_add(1);
            if e.mem.u16(lengths) & 1 == 1 {
                let head = e.mem.u16(strip_list).wrapping_add(next_shift);
                e.mem
                    .set_u16(indices.wrapping_add(written.wrapping_mul(2)), head);
                written = written.wrapping_add(1);
            }
        }
    }
    let strip_lengths = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 2u32]).u32();
    e.mem.set_u16(strip_lengths, written as u16);
    let length = e.mem.u16(lengths);
    let word = if length & 1 == 1 {
        length.wrapping_add(3)
    } else {
        length.wrapping_add(2)
    };
    let object = e.call(NI_ALLOCATE, &args![0x54u32]).u32();
    let made = if object == 0 {
        0
    } else {
        let active = copies.wrapping_mul(vertex_count);
        e.call(
            STRIPS_DATA_CONSTRUCT,
            &args![
                object,
                active,
                vertices,
                normals,
                colors,
                texture,
                1u32,
                0u32,
                written.wrapping_sub(2),
                1u32,
                strip_lengths,
                indices,
                active,
                0u32
            ],
        )
        .u32()
    };
    Copies {
        data: made,
        strips: true,
        copy_numbers,
        vertex_count,
        copies,
        word,
    }
}

/// The cached geometry made over the copies: the geometry's shader property cloned (slot
/// 0xf4) when it is one of kind 0xb, the geometry's property of kind 0 removed, two new
/// alpha properties (flags `0x1201` and `0x1200` over the defaults), the texturing property
/// (kind 4), the data's bound radius (+0x1c of the `+0xb8` data), and the root's kind
/// (marked at +0x30); held in the smart-pointer word `held_slot`, then stored in the cache
/// under the request's key. Answers the held object.
fn make_cached(
    e: &mut Engine,
    request: u32,
    geometry: u32,
    copies: &Copies,
    held_slot: u32,
    bound_copy: u32,
) -> u32 {
    let mut clone = 0u32;
    if e.call(GET_PROPERTY, &args![geometry, 3u32]).u32() != 0 {
        let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
        if e.mem.u32(property.wrapping_add(0x1c)) == 0xb {
            let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
            if property != 0 {
                let made = e.vcall(property, 0xf4, &args![0u32, 0u32]).u32();
                if made != 0 {
                    clone = made;
                    e.call(INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
                }
            }
        }
    }
    e.call(REMOVE_PROPERTY, &args![geometry, 0u32]);
    let first_alpha = new_alpha_property(e);
    let flags = e.mem.u16(first_alpha.wrapping_add(0x18)) & 0xf3ff | 0x1201;
    e.mem.set_u8(first_alpha.wrapping_add(0x1a), 0);
    e.mem.set_u16(first_alpha.wrapping_add(0x18), flags);
    let second_alpha = new_alpha_property(e);
    let flags = e.mem.u16(second_alpha.wrapping_add(0x18)) & 0xf3fe | 0x1200;
    e.mem.set_u16(second_alpha.wrapping_add(0x18), flags);
    e.mem.set_u8(second_alpha.wrapping_add(0x1a), 0);
    let texturing = e.call(GET_PROPERTY, &args![geometry, 4u32]).u32();
    let bound = e.mem.u32(geometry.wrapping_add(0xb8));
    for k in 0..4u32 {
        let v = e.mem.u32(bound.wrapping_add(0x10 + 4 * k));
        e.mem.set_u32(bound_copy + 4 * k, v);
    }
    let object = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    let mut cached = 0u32;
    if object != 0 {
        e.mem.set_u32(object, CACHED_BASE_VTABLE);
        e.mem.set_u32(object + 4, 0);
        e.call(INTERLOCKED_INCREMENT, &args![OBJECT_COUNT]);
        e.mem.set_u32(object, CACHED_VTABLE);
        for field in [0x8u32, 0x18, 0x1c, 0x20, 0x24] {
            e.mem.set_u32(object + field, 0);
        }
        cached = object;
    }
    assign(e, held_slot, cached, true);
    let held = e.mem.u32(held_slot);
    e.mem
        .set_u8(held.wrapping_add(0xc), u8::from(copies.strips));
    assign(e, held.wrapping_add(8), copies.data, true);
    e.mem.set_u32(held.wrapping_add(0x10), copies.copy_numbers);
    e.mem.set_u32(held.wrapping_add(0x14), copies.vertex_count);
    assign(e, held.wrapping_add(0x18), clone, true);
    assign(e, held.wrapping_add(0x20), second_alpha, false);
    assign(e, held.wrapping_add(0x1c), first_alpha, false);
    assign(e, held.wrapping_add(0x24), texturing, true);
    let radius = x87_store32(x87_load32(e.mem.u32(bound_copy + 0xc)));
    e.mem.set_u32(held.wrapping_add(0x28), radius);
    e.mem.set_u32(held.wrapping_add(0x2c), copies.copies);
    e.mem.set_u16(held.wrapping_add(0x32), copies.word);
    e.mem.set_u8(held.wrapping_add(0x30), 0);
    let root = e.mem.u32(request + 4);
    if root != 0 {
        let mut rtti = e.vcall(root, 8, &args![]).u32();
        let mut marked = false;
        while rtti != 0 {
            if rtti == RTTI_MARKED_ROOT {
                marked = true;
                break;
            }
            rtti = e.mem.u32(rtti + 4);
        }
        if marked {
            e.mem.set_u8(held.wrapping_add(0x30), 1);
        }
    }
    // The smart pointer handed to the cache by value.
    e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_add(4)]);
    let key = e.mem.u32(request + 8);
    let cache = e.call(CACHE, &args![]).u32();
    e.call(CACHE_INSERT, &args![cache, key, held]);
    if clone != 0 {
        release(e, clone);
    }
    held
}

// Translated from 00b587c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b587c0 - the FS:[0] exception frame (unwinding on a throw) and the /GS stack cookie check (00ec408c) are not translated
/// `DistantLODShaderProperty::GetCachedGeometry` (engine map): `cdecl`, one word, a request
/// (+0 the file name, +4 the loaded root, +8 the cache key). With the cache off
/// (`011f91c0` is 0) the answer is 0. Otherwise, around a local `BSStream`: no key answers
/// 0; a key the cache holds answers its geometry. Else the root is loaded from the file
/// when the request has none (its first top object's slot 0xc), and the root's children are
/// asked (slot 0x18) for a geometry until one answers (the code asks the same root again
/// for as long as none does). The geometry's data is copied as many times as the cache
/// word and the 16-bit index range allow (strips or shape, by the geometry's kind), a
/// cached geometry is made over the copies and stored in the cache. When there is none in
/// the end the missing text is formatted (into a local buffer, unused). The local smart
/// pointer is released and the cached geometry (or 0) is the answer.
pub fn distant_lod_shader_property_get_cached_geometry(e: &mut Engine, request: Ptr) -> u32 {
    let request = request.addr();
    if e.mem.u32(CACHE_LEVEL) == 0 {
        return 0;
    }
    e.with_stack(FRAME_SIZE, |e, frame| {
        // The frame below the saved registers (`F - 0x73c` is `frame`).
        let at = |offset: u32| frame.addr() + FRAME_SIZE + 0xc - offset;
        let stream = at(0x6a4);
        let held_slot = at(0x6f0);
        let text = at(0xd8);
        e.call(STREAM_CONSTRUCT, &args![stream]);
        e.mem.set_u32(stream, BS_STREAM_VTABLE);
        e.mem.set_u32(at(0xdc), 0);
        e.mem.set_u32(at(0xe0), 0);
        let key = e.mem.u32(request + 8);
        if key == 0 {
            e.call(BS_STREAM_DESTRUCT, &args![stream]);
            return 0;
        }
        e.mem.set_u32(held_slot, 0);
        let cache = e.call(CACHE, &args![]).u32();
        let found = e.call(CACHE_FIND, &args![cache, key, held_slot]).u32() as u8;
        let mut result = e.mem.u32(held_slot);
        if found == 0 {
            if e.mem.u32(request + 4) == 0 {
                let name = e.mem.u32(request);
                if name != 0 {
                    let loaded = e.call(BS_STREAM_LOAD, &args![stream, name, 0u32]).u32() as u8;
                    if loaded != 0 {
                        let top = e.mem.u32(e.mem.u32(at(0x484)));
                        if top != 0 {
                            let root = e.vcall(top, 0xc, &args![]).u32();
                            e.mem.set_u32(request + 4, root);
                        }
                    }
                }
            }
            let root = e.mem.u32(request + 4);
            result = e.mem.u32(held_slot);
            if root != 0 {
                let mut geometry = 0u32;
                let kind = loop {
                    if root == 0 {
                        if geometry != 0 {
                            break e.call(KIND_CAST, &args![geometry, RTTI_STRIPS]).u32();
                        }
                        break 0;
                    }
                    let mut index = 0u32;
                    if e.mem.u16(root + 0xa6) != 0 {
                        loop {
                            let children = e.mem.u32(root + 0xa0);
                            let child = e.mem.u32(children.wrapping_add(index * 4));
                            if child != 0 {
                                geometry = e.vcall(child, 0x18, &args![]).u32();
                                if geometry != 0 {
                                    break;
                                }
                            }
                            index += 1;
                            if index >= u32::from(e.mem.u16(root + 0xa6)) {
                                break;
                            }
                        }
                    }
                    if geometry != 0 {
                        break e.call(KIND_CAST, &args![geometry, RTTI_STRIPS]).u32();
                    }
                };
                let data = e.mem.u32(geometry.wrapping_add(0xb8));
                let limit = if (e.mem.u32(CACHE_LEVEL) as i32) < 2 {
                    0x50
                } else {
                    0xe4
                };
                let copies = if kind == 0 {
                    copy_shape(e, data, limit)
                } else {
                    copy_strips(e, data, limit)
                };
                result = make_cached(e, request, geometry, &copies, held_slot, at(0x6b4));
            }
        }
        if result == 0 {
            let name = e.mem.u32(request);
            e.call(FORMAT, &args![text, 0xc8u32, MISSING_FORMAT, name]);
        }
        if result != 0 {
            release(e, result);
        }
        e.call(BS_STREAM_DESTRUCT, &args![stream]);
        result
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b5_87c0,
        distant_lod_shader_property_get_cached_geometry(Ptr) -> u32
    )]
}

#[cfg(test)]
mod n012_tests {
    use super::*;

    const GET: u32 = 0x00b5_87c0;
    const CACHE_OBJECT: u32 = 0x0055_0000;
    const CHILD_GEOMETRY: u32 = 0x0060_0018;
    const ROOT_RTTI: u32 = 0x0060_0008;
    const TOP_ROOT: u32 = 0x0060_000c;
    const DELETE: u32 = 0x0060_0004;
    const CLONE: u32 = 0x0060_00f4;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object with a vtable whose `slot` is `target` (the delete slot too).
    fn object(e: &mut Engine, size: u32, slot: u32, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, DELETE);
        e.mem.set_u32(vtable + slot, target);
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn rig(level: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.mem.set_u32(CACHE_LEVEL, level);
        e.register(STREAM_CONSTRUCT, |_, _| Ret::default());
        e.register(BS_STREAM_DESTRUCT, |_, _| Ret::default());
        e.register(CACHE, |_, _| CACHE_OBJECT.into_ret());
        e.register(CACHE_FIND, |_, _| Ret::default());
        e.register(CACHE_INSERT, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let n = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let n = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.register(DELETE, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ARRAY_CONSTRUCT, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            for k in 0..a[2] {
                let b = e.mem.u8(a[1] + k);
                e.mem.set_u8(a[0] + k, b);
            }
            a[0].into_ret()
        });
        e.register(SHAPE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(STRIPS_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(REMOVE_PROPERTY, |_, _| Ret::default());
        e.register(PROPERTY_CONSTRUCT, |_, _| Ret::default());
        e.register(FORMAT, |_, _| Ret::default());
        e.register(ROOT_RTTI, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn request(e: &mut Engine, name: u32, root: u32, key: u32) -> u32 {
        let request = e.mem.alloc(0xc);
        e.mem.set_u32(request, name);
        e.mem.set_u32(request + 4, root);
        e.mem.set_u32(request + 8, key);
        request
    }

    /// A root with one empty child slot and a child answering a geometry; the geometry data
    /// two vertices with distinct words, a bound radius 12.5; `strips` picks the kind.
    fn scene(e: &mut Engine, strips: bool) -> (u32, u32, u32) {
        let root = object(e, 0xb0, 8, ROOT_RTTI);
        let children = e.mem.alloc(8);
        let child = object(e, 0x10, 0x18, CHILD_GEOMETRY);
        e.mem.set_u32(children + 4, child);
        e.mem.set_u32(root + 0xa0, children);
        e.mem.set_u16(root + 0xa6, 2);
        let geometry = e.mem.alloc(0xc0);
        e.register_double(CHILD_GEOMETRY, move |_, _| geometry.into_ret());
        let data = e.mem.alloc(0x50);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_f32(data + 0x1c, 12.5);
        e.mem.set_u16(data + 8, 2);
        for (field, size) in [(0x20u32, 0xcu32), (0x24, 0xc), (0x28, 0x10), (0x2c, 8)] {
            let array = e.mem.alloc(2 * size);
            for w in 0..(2 * size / 4) {
                e.mem.set_u32(array + 4 * w, field << 8 | w);
            }
            e.mem.set_u32(data + field, array);
        }
        let list = e.mem.alloc(6);
        e.mem.set_u16(list, 0);
        e.mem.set_u16(list + 2, 1);
        e.mem.set_u16(list + 4, 1);
        if strips {
            let lengths = e.mem.alloc(2);
            e.mem.set_u16(lengths, 3);
            e.mem.set_u32(data + 0x48, lengths);
            e.register_double(GET_STRIP_LISTS, move |_, _| list.into_ret());
        } else {
            e.mem.set_u32(data + 0x44, 3);
            e.register_double(GET_TRI_LIST, move |_, _| list.into_ret());
        }
        e.register_double(KIND_CAST, move |_, a| {
            if a[1] == RTTI_STRIPS && !strips {
                Ret::default()
            } else {
                a[0].into_ret()
            }
        });
        (root, geometry, data)
    }

    #[test]
    fn the_cache_off_or_no_key_answers_nothing() {
        let mut e = rig(0);
        let request = request(&mut e, 0, 0, 7);
        assert_eq!(e.call(GET, &args![request]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_u32(CACHE_LEVEL, 1);
        e.mem.set_u32(request + 8, 0);
        assert_eq!(e.call(GET, &args![request]).u32(), 0);
        assert_eq!(calls(&e, STREAM_CONSTRUCT).len(), 1);
        assert_eq!(calls(&e, BS_STREAM_DESTRUCT), calls(&e, STREAM_CONSTRUCT));
        assert!(calls(&e, CACHE).is_empty());
    }

    #[test]
    fn a_cached_key_answers_the_cache_and_a_missing_root_formats_the_text() {
        let mut e = rig(1);
        let cached = object(&mut e, 0x34, 4, DELETE);
        e.mem.set_u32(cached + 4, 2);
        e.register_double(CACHE_FIND, move |e, a| {
            if a[1] == 7 {
                e.mem.set_u32(a[2], cached);
                1u32.into_ret()
            } else {
                Ret::default()
            }
        });
        let found = request(&mut e, 0x1234, 0, 7);
        assert_eq!(e.call(GET, &args![found]).u32(), cached);
        assert_eq!(e.mem.u32(cached + 4), 1);
        assert!(calls(&e, FORMAT).is_empty());
        assert!(calls(&e, BS_STREAM_LOAD).is_empty());
        // Not cached, no root and no name: nothing, and the missing text is formatted.
        let missing = request(&mut e, 0, 0, 8);
        assert_eq!(e.call(GET, &args![missing]).u32(), 0);
        let format = calls(&e, FORMAT);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][1..], [0xc8, MISSING_FORMAT, 0]);
        assert_eq!(calls(&e, BS_STREAM_DESTRUCT).len(), 2);
    }

    #[test]
    fn a_loaded_shape_is_copied_and_cached() {
        let mut e = rig(1);
        let (root, geometry, data) = scene(&mut e, false);
        let top = object(&mut e, 0x10, 0xc, TOP_ROOT);
        e.register_double(TOP_ROOT, move |_, _| root.into_ret());
        let tops = e.mem.alloc(4);
        e.mem.set_u32(tops, top);
        e.register_double(BS_STREAM_LOAD, move |e, a| {
            e.mem.set_u32(a[0] + 0x220, tops);
            1u32.into_ret()
        });
        let texturing = object(&mut e, 0x20, 4, DELETE);
        e.register_double(GET_PROPERTY, move |_, a| {
            if a[1] == 4 {
                texturing.into_ret()
            } else {
                Ret::default()
            }
        });
        let request = request(&mut e, 0x4444, 0, 9);
        let cached = e.call(GET, &args![request]).u32();
        assert_eq!(e.mem.u32(request + 4), root);
        assert_eq!(calls(&e, BS_STREAM_LOAD)[0][1..], [0x4444, 0]);
        assert_eq!(e.mem.u32(cached), CACHED_VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 1);
        // Held by the cache only.
        assert_eq!(e.mem.u32(cached + 4), 1);
        assert_eq!(calls(&e, CACHE_INSERT), vec![vec![CACHE_OBJECT, 9, cached]]);
        assert_eq!(calls(&e, KIND_CAST)[0], vec![geometry, RTTI_STRIPS]);
        assert_eq!(calls(&e, KIND_CAST)[1], vec![data, RTTI_SHAPE_DATA]);
        // 0x50 copies of two vertices; three indices each, moved by two per copy.
        let shape = calls(&e, SHAPE_DATA_CONSTRUCT);
        assert_eq!(shape.len(), 1);
        let a = &shape[0];
        assert_eq!(a[1], 0xa0);
        assert_eq!(a[6..9], [1, 0, 0x50]);
        assert_eq!(a[10..], [0xa0, 0]);
        let vertices = a[2];
        assert_eq!(e.mem.u32(vertices + 0xc * 3 + 4), 0x2004);
        let colors = a[4];
        assert_eq!(e.mem.u32(colors + 0x10 * 0x9f + 0xc), 0x2807);
        let indices = a[9];
        let first: Vec<u16> = (0..6).map(|k| e.mem.u16(indices + 2 * k)).collect();
        assert_eq!(first, [0, 1, 1, 2, 3, 3]);
        assert_eq!(e.mem.u16(indices + 2 * 0xef), 0x9f);
        assert_eq!(e.mem.u8(cached + 0xc), 0);
        assert_eq!(e.mem.u32(cached + 8), a[0]);
        assert_eq!(e.mem.u32(a[0] + 4), 1);
        let numbers = e.mem.u32(cached + 0x10);
        assert_eq!(e.mem.f32(numbers + 4 * 5), 2.0);
        assert_eq!(e.mem.u32(cached + 0x14), 2);
        assert_eq!(e.mem.u32(cached + 0x18), 0);
        let first_alpha = e.mem.u32(cached + 0x1c);
        let second_alpha = e.mem.u32(cached + 0x20);
        assert_eq!(e.mem.u32(first_alpha), ALPHA_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(first_alpha + 0x18), 0x12ed);
        assert_eq!(e.mem.u16(second_alpha + 0x18), 0x12ec);
        assert_eq!(e.mem.u32(second_alpha + 4), 1);
        assert_eq!(e.mem.u32(cached + 0x24), texturing);
        assert_eq!(e.mem.u32(texturing + 4), 1);
        assert_eq!(e.mem.f32(cached + 0x28), 12.5);
        assert_eq!(e.mem.u32(cached + 0x2c), 0x50);
        assert_eq!(e.mem.u16(cached + 0x32), 1);
        assert_eq!(e.mem.u8(cached + 0x30), 0);
        assert_eq!(calls(&e, REMOVE_PROPERTY), vec![vec![geometry, 0]]);
        assert!(calls(&e, FORMAT).is_empty());
    }

    #[test]
    fn strips_are_joined_and_the_shader_cloned() {
        let mut e = rig(2);
        let (root, _, _) = scene(&mut e, true);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, RTTI_MARKED_ROOT);
        e.register_double(ROOT_RTTI, move |_, _| rtti.into_ret());
        let clone = object(&mut e, 0x20, 4, DELETE);
        let shader = object(&mut e, 0x20, 0xf4, CLONE);
        e.mem.set_u32(shader + 0x1c, 0xb);
        e.register_double(CLONE, move |_, a| {
            assert_eq!(a[1..], [0, 0]);
            clone.into_ret()
        });
        e.register_double(GET_PROPERTY, move |_, a| {
            if a[1] == 3 {
                shader.into_ret()
            } else {
                Ret::default()
            }
        });
        let request = request(&mut e, 0, root, 3);
        let cached = e.call(GET, &args![request]).u32();
        let strips = calls(&e, STRIPS_DATA_CONSTRUCT);
        assert_eq!(strips.len(), 1);
        let a = &strips[0];
        // 0xe4 copies: three indices each, three between copies.
        let written = 0xe4 * 3 + 0xe3 * 3;
        assert_eq!(a[1], 2 * 0xe4);
        assert_eq!(a[6..10], [1, 0, written - 2, 1]);
        assert_eq!(e.mem.u16(a[10]), written as u16);
        let indices = a[11];
        let first: Vec<u16> = (0..9).map(|k| e.mem.u16(indices + 2 * k)).collect();
        assert_eq!(first, [0, 1, 1, 1, 2, 2, 2, 3, 3]);
        assert_eq!(a[12..], [2 * 0xe4, 0]);
        assert_eq!(e.mem.u32(a[2] + 0xc * 2), 0x2000);
        assert_eq!(e.mem.u32(a[5] + 8 * 0x1c7 + 4), 0x2c03);
        assert_eq!(calls(&e, MEMCPY).len(), 4 * 0xe4);
        assert_eq!(e.mem.u8(cached + 0xc), 1);
        assert_eq!(e.mem.u32(cached + 0x18), clone);
        assert_eq!(e.mem.u32(clone + 4), 1);
        assert_eq!(e.mem.u16(cached + 0x32), 6);
        assert_eq!(e.mem.u8(cached + 0x30), 1);
        let numbers = e.mem.u32(cached + 0x10);
        assert_eq!(e.mem.f32(numbers + 4 * 0x1c7), 227.0);
    }
}
