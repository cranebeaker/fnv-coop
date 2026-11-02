//! `BSHavok/bhkconvexverticesshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkConvexVerticesShape` is a `bhkShape` (0x14 bytes): its Havok shape is at `+8` and its
//! construction info at `+0xC`. The construction info this class makes (`CreateCinfo`) is a
//! 0x20-byte block: radius at `+4` and two `hkArray`s of 16-byte entries, the vertices at
//! `+8` (data), `+0xC` (size), `+0x10` (capacity) and the plane equations at `+0x14`, `+0x18`,
//! `+0x1C`. In an `hkArray` the capacity word holds the capacity in its low 30 bits and
//! has its top bit set when the storage is not owned (`0x80000000` is the empty array).
//!
//! The functions that allocate or release Havok objects raise the per-thread allocation
//! word at `TLS + 0x2b4` to 0x10 for their duration. The exception-unwinding frames
//! (`FS:[0]`) of the functions marked `Translation incomplete` are not modeled. The
//! 16-byte aligned vector moves (`MOVAPS`) are done as plain byte copies; their alignment
//! faults are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{
    add_viewer_string, allocate_havok, enter_scope, leave_scope,
};
use crate::units::bshavok::bhkconvextransformshape::{copy_bytes, mulps_lane};

/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The pooled allocator object at `011f6238` and its allocate `(size)` (`00aa3e40`) and
/// release `(block)` (`00aa4060`) methods.
const POOL: u32 = 0x011f_6238;
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
const POOL_RELEASE: u32 = 0x00aa_4060;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `bhkStream::SwapHKArray` (`00cf0930`, cdecl `(cinfo, size, array)`), `bhkStream::LoadBinary`
/// (`00cf0f40`, cdecl `(stream, array)`) and `bhkStream::SaveBinary` (`00cf0cf0`, cdecl
/// `(stream, array)`).
const SWAP_HK_ARRAY: u32 = 0x00cf_0930;
const STREAM_LOAD_ARRAY: u32 = 0x00cf_0f40;
const STREAM_SAVE_ARRAY: u32 = 0x00cf_0cf0;
/// The base constructor the object constructor starts with (`00c87c30`).
const BASE_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable` methods: `Kill`, the clone lock and unlock.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkConvexShape` methods: destructor body, `CopyCinfo`, `LoadBinary`, `SaveBinary`,
/// `GetViewerStrings`.
const CONVEX_SHAPE_DESTRUCT: u32 = 0x00c9_fb00;
const CONVEX_SHAPE_COPY_CINFO: u32 = 0x00c9_fb70;
const CONVEX_SHAPE_LOAD_BINARY: u32 = 0x00c9_fbc0;
const CONVEX_SHAPE_SAVE_BINARY: u32 = 0x00c9_fc00;
const CONVEX_SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c9_fc50;
/// `bhkShape::CopyMembers` `(source, process)` (`00c84df0`).
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
/// The Havok convex vertices shape: constructor `(vertices, planes, radius)` (`00d59bd0`,
/// `RET 0xC`), `getOriginalVertices` `(out)` (`00d59200`) and `getPlaneEquations`
/// (`00d58d50`, answers the array), and `hkGeometryUtility::createConvexGeometry`
/// `(vertices, geometry)` (`00d5a5e0`, cdecl). `hkReferencedObject::removeReference` is
/// `00c905b0`.
const HK_VERTICES_CONSTRUCT: u32 = 0x00d5_9bd0;
const GET_ORIGINAL_VERTICES: u32 = 0x00d5_9200;
const GET_PLANE_EQUATIONS: u32 = 0x00d5_8d50;
const CREATE_CONVEX_GEOMETRY: u32 = 0x00d5_a5e0;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `NiGetViewerString` for a text (`00a7d330`, cdecl).
const VIEWER_STRING: u32 = 0x00a7_d330;
/// The vtables of the classes the constructor passes through (RTTI-confirmed), and their
/// live-object counts.
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_SPHERE_REP_SHAPE: u32 = 0x0103_0b0c;
const VTABLE_CONVEX_SHAPE: u32 = 0x0103_0a24;
const VTABLE_CONVEX_VERTICES_SHAPE: u32 = 0x010c_a924;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_SPHERE_REP_SHAPE: u32 = 0x0126_8274;
const COUNT_CONVEX_SHAPE: u32 = 0x0126_8250;
const COUNT_CONVEX_VERTICES_SHAPE: u32 = 0x0126_869c;
/// The global holding the text `GetViewerStrings` starts with (read as a word).
const CLASS_TEXT: u32 = 0x0126_86a0;
/// The default radius float, and the name the display geometry is made under.
const DEFAULT_RADIUS: u32 = 0x010c_72bc;
const GEOMETRY_NAME: u32 = 0x010c_405c;
/// Bits of an `hkArray`'s capacity word: the capacity, and the flag that the storage is not
/// owned.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// The construction info of `this`: virtual slot `0xB4` fills a flag byte and answers it.
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32()
}

/// The hkArray allocator: the memory router's allocator (`router + 0x10`).
fn array_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Gives the array an empty state: no data, no size, not owned.
fn init_empty_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// Releases an `hkArray` of 16-byte entries the way the code does: the size is cleared,
/// the allocator's virtual slot `0x10` takes `(data, capacity * 16)` when the capacity
/// word's top bit is clear, and the array becomes empty and not owned.
fn release_array(e: &mut Engine, array: u32) {
    let allocator = array_allocator(e);
    let capacity_word = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity_word as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity_word & CAPACITY_MASK) << 4],
        );
    }
    e.mem.set_u32(array + 8, NOT_OWNED);
    e.mem.set_u32(array, 0);
}

/// Reserves room in the `hkArray` of 16-byte entries for `wanted` entries when its capacity
/// is smaller: the larger of twice the capacity and `wanted` (signed compares).
fn reserve_array(e: &mut Engine, array: u32, wanted: u32) {
    let allocator = array_allocator(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted as i32 {
        let doubled = capacity.wrapping_add(capacity);
        let grow_to = if (wanted as i32) < doubled {
            doubled
        } else {
            wanted as i32
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, grow_to, 0x10u32]);
    }
}

/// `FLD`/`FMUL`/`FSTP` of two floats through the x87 stack: both operands are quieted when
/// loaded, a NaN operand wins (the larger significand, then the positive
/// one, when both are NaN), otherwise the product rounded to a float.
fn x87_mul_bits(first: u32, second: u32) -> u32 {
    // Both operands loaded (quiet), the x87 NaN choice (#199); the product of two floats is
    // exact in f64, so it is rounded once.
    x87_store32(x87_mul(x87_load32(first), x87_load32(second)))
}

// Translated from 00ccbb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QCinfoSize` of the convex vertices shape (the address is shared with other classes'
/// identical code, so the name is unconfirmed): takes no arguments and answers 0x20, the
/// size of the construction info this class builds.
pub fn fn_00ccbb40(_e: &mut Engine) -> u32 {
    0x20
}

// Translated from 00ccdbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccdbc0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexVerticesShape::~bhkConvexVerticesShape` (Ghidra name): sets the class vtable,
/// runs `bhkSerializable::Kill`, decrements the live-object count at `0126869c` and runs the
/// `bhkConvexShape` destructor body (`00c9fb00`).
pub fn bhk_convex_vertices_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CONVEX_VERTICES_SHAPE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(COUNT_CONVEX_VERTICES_SHAPE);
    e.set_global(COUNT_CONVEX_VERTICES_SHAPE, count.wrapping_sub(1));
    e.call(CONVEX_SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00ccdc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexVerticesShape` constructor (unnamed in the engine map): the base constructor
/// (`00c87c30`), `+0xC` and `+0x10` cleared, then the vtable of each class on the way down
/// (`bhkShape`, `bhkSphereRepShape`, `bhkConvexShape`, `bhkConvexVerticesShape`), each with
/// its live-object count raised. Answers `this`.
pub fn fn_00ccdc40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    for (vtable, counter) in [
        (VTABLE_SHAPE, COUNT_SHAPE),
        (VTABLE_SPHERE_REP_SHAPE, COUNT_SPHERE_REP_SHAPE),
        (VTABLE_CONVEX_SHAPE, COUNT_CONVEX_SHAPE),
        (VTABLE_CONVEX_VERTICES_SHAPE, COUNT_CONVEX_VERTICES_SHAPE),
    ] {
        e.mem.set_u32(this.addr(), vtable);
        increment_count(e, counter);
    }
    this
}

// Translated from 00ccdc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales a construction info (unnamed in the engine map), `(cinfo, scale)`: every float of
/// every 16-byte vertex entry (array at `+8`, size at `+0xC`, signed) is multiplied by
/// `scale` (`MULPS`, the scale being the destination operand), then the fourth float (`+0xC`)
/// of every 16-byte plane equation (array at `+0x14`, size at `+0x18`, signed) is multiplied
/// by it through the x87 stack. Data pointers and sizes are read again on every iteration.
pub fn fn_00ccdc90(e: &mut Engine, this: Ptr, scale: f32) {
    let scale = scale.to_bits();
    let mut index = 0u32;
    if (e.mem.u32(this.addr() + 0xc) as i32) > 0 {
        loop {
            let entry = e.mem.u32(this.addr() + 8).wrapping_add(16 * index);
            for lane in [0, 4, 8, 12] {
                let value = e.mem.u32(entry + lane);
                e.mem.set_u32(entry + lane, mulps_lane(scale, value));
            }
            index += 1;
            if (index as i32) >= (e.mem.u32(this.addr() + 0xc) as i32) {
                break;
            }
        }
    }
    let mut index = 0u32;
    if (e.mem.u32(this.addr() + 0x18) as i32) > 0 {
        loop {
            let at = e.mem.u32(this.addr() + 0x14).wrapping_add(16 * index + 0xc);
            let value = e.mem.u32(at);
            e.mem.set_u32(at, x87_mul_bits(value, scale));
            index += 1;
            if (index as i32) >= (e.mem.u32(this.addr() + 0x18) as i32) {
                break;
            }
        }
    }
}

// Translated from 00ccdd00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccdd00 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates a 0x14-byte block (`operator new`) and constructs a `bhkConvexVerticesShape` in
/// it with `00ccdc40` (unnamed in the engine map). Answers the object, or 0 when the
/// allocation fails.
pub fn fn_00ccdd00(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00ccdc40(e, Ptr::new(block))
}

// Translated from 00ccdd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexVerticesShape::CopyMembers` (Ghidra name), `(this, source, process)`: takes the
/// cinfo (virtual slot `0xB4`); when the float at `process + 0x10` (through the x87 stack)
/// is not 1.0 (a NaN counts as not equal) scales the cinfo by it (`00ccdc90`); then
/// `bhkShape::CopyMembers`.
pub fn bhk_convex_vertices_shape_copy_members(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    process: u32,
) {
    let cinfo = cinfo_of(e, this);
    let scale = f32::from_bits(x87_quiet_bits(e.mem.u32(process + 0x10)));
    if scale != 1.0 {
        fn_00ccdc90(e, Ptr::new(cinfo), scale);
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, source, process]);
}

// Translated from 00ccdde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexVerticesShape::LoadBinary` (Ghidra name): `bhkConvexShape::LoadBinary`, takes
/// the cinfo, sets its radius (`+4`) to the default float at `010c72bc` (through the x87
/// stack), swaps both of its arrays (`bhkStream::SwapHKArray` with the size from virtual
/// slot `0xA8`) when the stream's byte at `+0x1E5` is not 1, then loads both arrays from the
/// stream (`bhkStream::LoadBinary`).
pub fn bhk_convex_vertices_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CONVEX_SHAPE_LOAD_BINARY, &args![this, stream]);
    let cinfo = cinfo_of(e, this);
    let radius = x87_quiet_bits(e.mem.u32(DEFAULT_RADIUS));
    e.mem.set_u32(cinfo + 4, radius);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let size = e.vcall(this.addr(), 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![cinfo, size, cinfo + 8]);
        e.call(SWAP_HK_ARRAY, &args![cinfo, size, cinfo + 0x14]);
    }
    e.call(STREAM_LOAD_ARRAY, &args![stream, cinfo + 8]);
    e.call(STREAM_LOAD_ARRAY, &args![stream, cinfo + 0x14]);
}

// Translated from 00ccde60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccde60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexVerticesShape::CreateClone` (Ghidra name): in the Havok allocation scope and
/// between the clone lock and unlock (`00c876f0`, `00c87700`), makes a new 0x14-byte shape
/// (`operator new` and the constructor `00ccdc40`; null stays null) and calls this shape's
/// virtual slot `0xC4` with `(clone, process)`. Answers the clone.
pub fn bhk_convex_vertices_shape_create_clone(e: &mut Engine, this: Ptr, process: u32) -> Ptr {
    let old = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = fn_00ccdd00(e);
    e.vcall(this.addr(), 0xc4, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, old);
    clone
}

// Translated from 00ccdf20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccdf20 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexVerticesShape::Init` (Ghidra name): in the Havok allocation scope builds a
/// 0x70-byte Havok convex vertices shape (`00d59bd0`) from a vertex array view made of the
/// cinfo's vertex data (`+8`) and size (`+0xC`) with capacity 0x10, the cinfo's plane
/// equations (`+0x14`) and the default radius float (`010c72bc`, through the x87 stack),
/// hands it to virtual slot `0x8C`, releases it, then calls virtual slot `0xBC` with the
/// cinfo.
pub fn bhk_convex_vertices_shape_init(e: &mut Engine, this: Ptr, cinfo: u32) {
    let old = enter_scope(e);
    e.with_stack(0xc, |e, view| {
        let data = e.mem.u32(cinfo + 8);
        let size = e.mem.u32(cinfo + 0xc);
        e.mem.set_u32(view.addr(), data);
        e.mem.set_u32(view.addr() + 4, size);
        e.mem.set_u32(view.addr() + 8, 0x10);
        let block = allocate_havok(e, 0x70);
        let radius = x87_quiet_bits(e.mem.u32(DEFAULT_RADIUS));
        let shape = e
            .call(
                HK_VERTICES_CONSTRUCT,
                &args![block, view, cinfo + 0x14, radius],
            )
            .u32();
        e.vcall(this.addr(), 0x8c, &args![shape]);
        e.call(REMOVE_REFERENCE, &args![shape]);
        e.vcall(this.addr(), 0xbc, &args![cinfo]);
    });
    leave_scope(e, old);
}

// Translated from 00cce010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexVerticesShape::CopyCinfo` (Ghidra name): `bhkConvexShape::CopyCinfo`, then,
/// when `this` and its Havok shape exist, the Havok shape's original vertices go into the
/// cinfo's vertex array (`getOriginalVertices` with `cinfo + 8`) and its plane equations
/// (`getPlaneEquations`) are copied into the cinfo's second array: when that array's
/// capacity is smaller than the planes' count (signed) its storage is freed (virtual slot
/// `0x10` of the allocator with `(data, capacity * 16)`, when the capacity word's top bit is
/// clear) and replaced by the allocator's virtual slot `0xC` block for `count * 16` bytes
/// (the size it grants, divided by 16 with the signed rounding the compiler uses, becomes the
/// capacity word); then the size (`+0x18`) is set and the entries are copied.
pub fn bhk_convex_vertices_shape_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: u32) {
    e.call(CONVEX_SHAPE_COPY_CINFO, &args![this, cinfo]);
    if this.is_null() {
        return;
    }
    let shape = e.mem.u32(this.addr() + 8);
    if shape == 0 {
        return;
    }
    e.call(GET_ORIGINAL_VERTICES, &args![shape, cinfo + 8]);
    let planes = e.call(GET_PLANE_EQUATIONS, &args![shape]).u32();
    let allocator = array_allocator(e);
    let capacity_word = e.mem.u32(cinfo + 0x1c);
    let capacity = capacity_word & CAPACITY_MASK;
    if (capacity as i32) < (e.mem.u32(planes + 4) as i32) {
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(cinfo + 0x14);
            e.vcall(allocator, 0x10, &args![data, capacity << 4]);
        }
        let wanted = e.mem.u32(planes + 4) << 4;
        let (block, granted) = e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), wanted);
            let block = e.vcall(allocator, 0xc, &args![size]).u32();
            (block, e.mem.u32(size.addr()) as i32)
        });
        e.mem.set_u32(cinfo + 0x14, block);
        let capacity = (granted + ((granted >> 31) & 0xf)) >> 4;
        e.mem.set_u32(cinfo + 0x1c, capacity as u32);
    }
    let count = e.mem.u32(planes + 4);
    let target = e.mem.u32(cinfo + 0x14);
    e.mem.set_u32(cinfo + 0x18, count);
    let source = e.mem.u32(planes);
    if (count as i32) > 0 {
        copy_bytes(e, target, source, count << 4);
    }
}

// Translated from 00cce0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases an `hkArray` of 16-byte entries (unnamed in the engine map): clears its size at
/// `+4`, frees its storage through the allocator's virtual slot `0x10` with `(data,
/// capacity * 16)` when the capacity word's top bit is clear, and leaves it empty and not
/// owned.
pub fn fn_00cce0d0(e: &mut Engine, array: Ptr) {
    release_array(e, array.addr());
}

// Translated from 00cce110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cce110 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexVerticesShape::GetViewerStrings` (Ghidra name): in the Havok allocation scope
/// runs `bhkConvexShape::GetViewerStrings` and appends the viewer string of the text in the
/// global at `012686a0`.
pub fn bhk_convex_vertices_shape_get_viewer_strings(e: &mut Engine, this: Ptr, list: u32) {
    let old = enter_scope(e);
    e.call(CONVEX_SHAPE_GET_VIEWER_STRINGS, &args![this, list]);
    let class_text = e.global::<u32>(CLASS_TEXT);
    let text = e.call(VIEWER_STRING, &args![class_text]).u32();
    add_viewer_string(e, list, text);
    leave_scope(e, old);
}

// Translated from 00cce1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cce1c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexVerticesShape::CreateCinfo` (Ghidra name), `(this, created)`: in the Havok
/// allocation scope stores into the byte at `created` whether the shape had no cinfo
/// (`+0xC` null); in that case allocates a 0x20-byte block from the pool at `011f6238`
/// (null stays null), fills it (`+0` zero, `+4` the float at `010c72bc` through the x87
/// stack, both arrays empty and not owned), stores it at `+0xC` and, when the shape has a
/// Havok shape, fills it with `CopyCinfo` (`00cce010`; also with a null cinfo). Answers the
/// cinfo at `+0xC`.
pub fn bhk_convex_vertices_shape_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let old = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let cinfo = e.call(POOL_ALLOCATE, &args![POOL, 0x20u32]).u32();
        if cinfo != 0 {
            e.mem.set_u32(cinfo, 0);
            let radius = x87_quiet_bits(e.mem.u32(DEFAULT_RADIUS));
            e.mem.set_u32(cinfo + 4, radius);
            init_empty_array(e, cinfo + 8);
            init_empty_array(e, cinfo + 0x14);
        }
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if e.mem.u32(this.addr() + 8) != 0 {
            bhk_convex_vertices_shape_copy_cinfo(e, this, cinfo);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    leave_scope(e, old);
    cinfo
}

// Translated from 00cce290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cce290 - the FS:[0] exception-unwinding frame is not modeled.
/// Releases both arrays of a construction info (unnamed in the engine map): the plane
/// equations (`+0x14`) first, then the vertices (`+8`), each as `00cce0d0` does.
pub fn fn_00cce290(e: &mut Engine, cinfo: Ptr) {
    release_array(e, cinfo.addr() + 0x14);
    release_array(e, cinfo.addr() + 8);
}

// Translated from 00cce340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cce340 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexVerticesShape::SaveBinary` (Ghidra name): takes the cinfo (virtual slot `0xB4`,
/// which also fills a flag saying whether it was just created) and moves both of its
/// arrays into locals, leaving the cinfo's arrays empty and not owned. When the stream's
/// byte at `+0x1E4` is not 1 it swaps the (now empty) arrays (`bhkStream::SwapHKArray`
/// with the size from virtual slot `0xA8`); then `bhkConvexShape::SaveBinary`, both local
/// arrays written to the stream (`bhkStream::SaveBinary`) and virtual slot `0xA4` called
/// with the flag (its upper three bytes are stack leftovers in the exe; here 0). When the
/// flag's low byte is clear and the stream's byte is not 1 the empty arrays are swapped
/// again. Finally both local arrays are released (second first).
pub fn bhk_convex_vertices_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.with_stack(0x20, |e, block| {
        let first = block.addr();
        let second = first + 0xc;
        let flag = first + 0x18;
        let cinfo = e.vcall(this.addr(), 0xb4, &args![flag]).u32();
        for (from, to) in [(cinfo + 8, first), (cinfo + 0x14, second)] {
            for offset in [0, 4, 8] {
                let word = e.mem.u32(from + offset);
                e.mem.set_u32(to + offset, word);
            }
            init_empty_array(e, from);
        }
        if e.mem.u8(stream + 0x1e4) != 1 {
            let size = e.vcall(this.addr(), 0xa8, &args![]).u32();
            e.call(SWAP_HK_ARRAY, &args![cinfo, size, cinfo + 8]);
            e.call(SWAP_HK_ARRAY, &args![cinfo, size, cinfo + 0x14]);
        }
        e.call(CONVEX_SHAPE_SAVE_BINARY, &args![this, stream]);
        e.call(STREAM_SAVE_ARRAY, &args![stream, first]);
        e.call(STREAM_SAVE_ARRAY, &args![stream, second]);
        let created = e.mem.u32(flag);
        e.vcall(this.addr(), 0xa4, &args![created]);
        if e.mem.u8(flag) == 0 && e.mem.u8(stream + 0x1e4) != 1 {
            let size = e.vcall(this.addr(), 0xa8, &args![]).u32();
            e.call(SWAP_HK_ARRAY, &args![cinfo, size, cinfo + 8]);
            e.call(SWAP_HK_ARRAY, &args![cinfo, size, cinfo + 0x14]);
        }
        release_array(e, second);
        release_array(e, first);
    });
}

// Translated from 00cce530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cce530 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkGeometry::~hkGeometry` (Ghidra name): releases the geometry's two arrays of 16-byte
/// entries, the one at `+0xC` first, then the one at `+0`, each as `00cce0d0` does.
pub fn hk_geometry_destructor(e: &mut Engine, geometry: Ptr) {
    release_array(e, geometry.addr() + 0xc);
    release_array(e, geometry.addr());
}

// Translated from 00cce5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the geometry that `00cce530` destroys (unnamed in the engine map): both
/// arrays of 16-byte entries (`+0` and `+0xC`) become empty and not owned.
/// Answers `geometry` (`MOV EAX,ECX` first; callers keep it, 00ddadfc).
pub fn fn_00cce5e0(e: &mut Engine, geometry: Ptr) -> Ptr {
    init_empty_array(e, geometry.addr());
    init_empty_array(e, geometry.addr() + 0xc);
    geometry
}

// Translated from 00cce600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cce600 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexVerticesShape::BuildDisplayGeometry` (Ghidra name), `(this, parent, extra)`: in
/// the Havok allocation scope asks the Havok shape (a null shape reads through address 0)
/// for its vertex count (virtual slot `0x2C`), reserves room for that many 16-byte entries
/// in a scratch array, has the shape fill it (virtual slot `0x30` with the array's data,
/// answering the vertices' address), copies the vertices into a second array of the same
/// size, builds a convex geometry from it (`hkGeometryUtility::createConvexGeometry`, with
/// the array viewed with capacity 0x10) and calls this shape's virtual slot `0xDC` with
/// `(parent, geometry, extra, name)` (the name is the text at `010c405c`). Then the geometry
/// and both arrays are released.
pub fn bhk_convex_vertices_shape_build_display_geometry(
    e: &mut Engine,
    this: Ptr,
    parent: u32,
    extra: u32,
) {
    let old = enter_scope(e);
    let shape = if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    let count = e.vcall(shape, 0x2c, &args![]).u32();
    e.with_stack(0x3c, |e, block| {
        let scratch = block.addr();
        let copy = scratch + 0xc;
        let view = scratch + 0x18;
        let geometry = scratch + 0x24;
        init_empty_array(e, scratch);
        reserve_array(e, scratch, count);
        let data = e.mem.u32(scratch);
        e.mem.set_u32(scratch + 4, count);
        let vertices = e.vcall(shape, 0x30, &args![data]).u32();
        init_empty_array(e, copy);
        reserve_array(e, copy, count);
        e.mem.set_u32(copy + 4, count);
        if (count as i32) > 0 {
            let target = e.mem.u32(copy);
            copy_bytes(e, target, vertices, count << 4);
        }
        let target = e.mem.u32(copy);
        let size = e.mem.u32(copy + 4);
        e.mem.set_u32(view, target);
        e.mem.set_u32(view + 4, size);
        e.mem.set_u32(view + 8, 0x10);
        fn_00cce5e0(e, Ptr::new(geometry));
        e.call(CREATE_CONVEX_GEOMETRY, &args![view, geometry]);
        e.vcall(
            this.addr(),
            0xdc,
            &args![parent, geometry, extra, GEOMETRY_NAME],
        );
        hk_geometry_destructor(e, Ptr::new(geometry));
        release_array(e, copy);
        release_array(e, scratch);
    });
    leave_scope(e, old);
}

// Translated from 00cce860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cce860 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexVerticesShape::KillCinfo` (Ghidra name): when the low byte of `kill` is set,
/// in the Havok allocation scope releases the cinfo's arrays (`00cce290`) and the cinfo block
/// to the pool (`00aa4060`) if there is one, and clears `+0xC`.
pub fn bhk_convex_vertices_shape_kill_cinfo(e: &mut Engine, this: Ptr, kill: u32) {
    let old = enter_scope(e);
    if kill as u8 != 0 {
        let cinfo = e.mem.u32(this.addr() + 0xc);
        if cinfo != 0 {
            fn_00cce290(e, Ptr::new(cinfo));
            e.call(POOL_RELEASE, &args![POOL, cinfo]);
        }
        e.mem.set_u32(this.addr() + 0xc, 0);
    }
    leave_scope(e, old);
}

// Translated from 00cd7f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexVerticesShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ccdbc0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_convex_vertices_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_dbc0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00ccdc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexVerticesShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_86a0.
pub fn bhk_convex_vertices_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_86a0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ccbb40, fn_00ccbb40() -> u32),
        entry!(0x00ccdbc0, bhk_convex_vertices_shape_destructor(Ptr)),
        entry!(0x00ccdc30, bhk_convex_vertices_shape_get_r_t_t_i() -> u32),
        entry!(0x00ccdc40, fn_00ccdc40(Ptr) -> Ptr),
        entry!(0x00ccdc90, fn_00ccdc90(Ptr, f32)),
        entry!(0x00ccdd00, fn_00ccdd00() -> Ptr),
        entry!(
            0x00ccdd70,
            bhk_convex_vertices_shape_copy_members(Ptr, u32, u32)
        ),
        entry!(0x00ccdde0, bhk_convex_vertices_shape_load_binary(Ptr, u32)),
        entry!(0x00ccde60, bhk_convex_vertices_shape_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00ccdf20, bhk_convex_vertices_shape_init(Ptr, u32)),
        entry!(0x00cce010, bhk_convex_vertices_shape_copy_cinfo(Ptr, u32)),
        entry!(0x00cce0d0, fn_00cce0d0(Ptr)),
        entry!(
            0x00cce110,
            bhk_convex_vertices_shape_get_viewer_strings(Ptr, u32)
        ),
        entry!(0x00cce1c0, bhk_convex_vertices_shape_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00cce290, fn_00cce290(Ptr)),
        entry!(0x00cce340, bhk_convex_vertices_shape_save_binary(Ptr, u32)),
        entry!(0x00cce530, hk_geometry_destructor(Ptr)),
        entry!(0x00cce5e0, fn_00cce5e0(Ptr) -> Ptr),
        entry!(
            0x00cce600,
            bhk_convex_vertices_shape_build_display_geometry(Ptr, u32, u32)
        ),
        entry!(0x00cce860, bhk_convex_vertices_shape_kill_cinfo(Ptr, u32)),
        entry!(0x00cd7f30, bhk_convex_vertices_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_convex_vertices_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_dbc0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cd7f30, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cd7f30, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x14]]);
        assert_eq!(calls_to(&e, 0x00cc_dbc0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_convex_vertices_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_dc30, &args![]).u32(), 0x0126_86a0);
    }

    #[test]
    fn empty_geometry_initialiser_answers_its_geometry() {
        let mut e = Engine::new();
        let geometry = e.mem.alloc(0x18);
        // `MOV EAX,ECX` first; the callers keep the answer (00ddadfc).
        assert_eq!(e.call(0x00cce5e0, &args![geometry]).u32(), geometry);
        assert_eq!(e.mem.u32(geometry + 8), 0x8000_0000);
    }

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// An object whose vtable (a fresh block) has the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    fn scope(e: &mut Engine) -> u32 {
        let word = e.tls() + 0x2b4;
        e.mem.u32(word)
    }

    /// Starts the allocation scope word at a recognisable value, so its restore is visible.
    fn mark_scope(e: &mut Engine) {
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
    }

    /// The memory router and its allocator: slot `0xC` grants the asked size plus 16 bytes
    /// (and writes the granted size back), slot `0x10` records a free (`0x09000201`).
    fn install_allocator(e: &mut Engine) {
        let allocator = object(e, &[(0xc, 0x0900_0200), (0x10, 0x0900_0201)]);
        e.register(0x0900_0200, |e, a| {
            let granted = e.mem.u32(a[1]) + 16;
            e.mem.set_u32(a[1], granted);
            Ret {
                eax: e.mem.alloc(granted),
                ..Ret::default()
            }
        });
        quiet(e, &[0x0900_0201]);
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        e.map(0x0200_3000, 0x10);
        e.mem.set_u32(0x0200_3000, router);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(0x0200_3000),
            ..Ret::default()
        });
    }

    /// A shape (slot `0xB4` answers its `+0xC`, slot `0xC4` records, `0xA8` answers 9).
    fn shape(e: &mut Engine) -> u32 {
        let this = object(
            e,
            &[
                (0xb4, 0x0900_0100),
                (0xc4, 0x0900_0101),
                (0xa8, 0x0900_0102),
            ],
        );
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        e.register(0x0900_0102, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        quiet(e, &[0x0900_0101]);
        this
    }

    #[test]
    fn fn_00ccbb40_answers_0x20() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ccbb40, &args![]).u32(), 0x20);
    }

    #[test]
    fn destructor_sets_the_vtable_kills_counts_down_and_destructs() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_CONVEX_VERTICES_SHAPE, 9u32);
        quiet(&mut e, &[SERIALIZABLE_KILL, CONVEX_SHAPE_DESTRUCT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00ccdbc0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_CONVEX_VERTICES_SHAPE);
        assert_eq!(e.global::<u32>(COUNT_CONVEX_VERTICES_SHAPE), 8);
        assert_eq!(
            order(&e),
            vec![0x00ccdbc0, SERIALIZABLE_KILL, CONVEX_SHAPE_DESTRUCT]
        );
    }

    #[test]
    fn constructor_counts_every_class_on_the_way_down() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        quiet(&mut e, &[BASE_CONSTRUCT]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 5);
        e.mem.set_u32(this + 0x10, 6);
        assert_eq!(e.call(0x00ccdc40, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CONVEX_VERTICES_SHAPE);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (0, 0));
        for counter in [
            COUNT_SHAPE,
            COUNT_SPHERE_REP_SHAPE,
            COUNT_CONVEX_SHAPE,
            COUNT_CONVEX_VERTICES_SHAPE,
        ] {
            assert_eq!(e.global::<u32>(counter), 1);
        }
    }

    /// A construction info with two vertices and two plane equations.
    fn cinfo_with_data(e: &mut Engine) -> (u32, u32, u32) {
        let cinfo = e.mem.alloc(0x40);
        let vertices = e.mem.alloc(0x20);
        let planes = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_f32(vertices + 4 * i, 1.0 + i as f32);
            e.mem.set_f32(planes + 4 * i, 10.0 + i as f32);
        }
        e.mem.set_u32(cinfo + 8, vertices);
        e.mem.set_u32(cinfo + 0xc, 2);
        e.mem.set_u32(cinfo + 0x14, planes);
        e.mem.set_u32(cinfo + 0x18, 2);
        (cinfo, vertices, planes)
    }

    #[test]
    fn scaling_multiplies_every_vertex_float_and_the_plane_distances() {
        let mut e = Engine::new();
        let (cinfo, vertices, planes) = cinfo_with_data(&mut e);
        e.call(0x00ccdc90, &args![cinfo, 2.0f32]);
        for i in 0..8u32 {
            assert_eq!(e.mem.f32(vertices + 4 * i), 2.0 * (1.0 + i as f32));
        }
        // Only the fourth float of each plane equation.
        for i in 0..8u32 {
            let original = 10.0 + i as f32;
            let expected = if i % 4 == 3 { 2.0 * original } else { original };
            assert_eq!(e.mem.f32(planes + 4 * i), expected);
        }
        // Counts of zero or below leave the data alone.
        e.mem.set_u32(cinfo + 0xc, 0);
        e.mem.set_u32(cinfo + 0x18, 0xffff_ffff);
        e.call(0x00ccdc90, &args![cinfo, 3.0f32]);
        assert_eq!(e.mem.f32(vertices), 2.0);
        assert_eq!(e.mem.f32(planes + 12), 26.0);
    }

    #[test]
    fn scaling_by_a_nan_gives_quiet_nans() {
        let mut e = Engine::new();
        let (cinfo, vertices, planes) = cinfo_with_data(&mut e);
        e.call(0x00ccdc90, &args![cinfo, f32::from_bits(0x7f80_0001)]);
        for i in 0..8u32 {
            assert_eq!(e.mem.u32(vertices + 4 * i), 0x7fc0_0001);
        }
        assert_eq!(e.mem.u32(planes + 12), 0x7fc0_0001);
        assert_eq!(e.mem.f32(planes), 10.0);
    }

    #[test]
    fn allocate_and_construct_makes_a_0x14_byte_shape() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        quiet(&mut e, &[BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00ccdd00, &args![]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(e.mem.u32(made), VTABLE_CONVEX_VERTICES_SHAPE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00ccdd00, &args![]).u32(), 0);
    }

    #[test]
    fn copy_members_scales_the_cinfo_unless_the_scale_is_one() {
        let mut e = Engine::new();
        quiet(&mut e, &[SHAPE_COPY_MEMBERS]);
        let this = shape(&mut e);
        let (cinfo, vertices, _) = cinfo_with_data(&mut e);
        e.mem.set_u32(this + 0xc, cinfo);
        let process = e.mem.alloc(0x40);
        e.mem.set_f32(process + 0x10, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00ccdd70, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.f32(vertices), 1.0);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this, 0x9999, process]]
        );
        e.mem.set_f32(process + 0x10, 0.5);
        e.call(0x00ccdd70, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.f32(vertices), 0.5);
        assert_eq!(e.mem.f32(vertices + 4), 1.0);
        // A NaN scale is not 1.0: it is applied (quieted).
        e.mem.set_u32(process + 0x10, 0x7f80_0001);
        e.call(0x00ccdd70, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(vertices), 0x7fc0_0001);
    }

    #[test]
    fn load_binary_sets_the_default_radius_swaps_and_loads_both_arrays() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[CONVEX_SHAPE_LOAD_BINARY, SWAP_HK_ARRAY, STREAM_LOAD_ARRAY],
        );
        e.map(DEFAULT_RADIUS, 4);
        e.mem.set_u32(DEFAULT_RADIUS, 0x7f80_0001);
        let this = shape(&mut e);
        let cinfo = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, cinfo);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ccdde0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x7fc0_0001);
        assert!(calls_to(&e, SWAP_HK_ARRAY).is_empty());
        assert_eq!(
            calls_to(&e, STREAM_LOAD_ARRAY),
            vec![vec![stream, cinfo + 8], vec![stream, cinfo + 0x14]]
        );
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ccdde0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, SWAP_HK_ARRAY),
            vec![vec![cinfo, 9, cinfo + 8], vec![cinfo, 9, cinfo + 0x14]]
        );
        // The swaps come before the array loads.
        let order = order(&e);
        let swap = order.iter().position(|a| *a == SWAP_HK_ARRAY).unwrap();
        let load = order.iter().position(|a| *a == STREAM_LOAD_ARRAY).unwrap();
        assert!(swap < load);
        assert_eq!(order[..2], [0x00ccdde0, CONVEX_SHAPE_LOAD_BINARY]);
    }

    #[test]
    fn create_clone_brackets_the_new_shape_with_the_clone_lock() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        let this = shape(&mut e);
        quiet(&mut e, &[CLONE_LOCK, CLONE_UNLOCK, BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let clone = e.call(0x00ccde60, &args![this, 0x4242u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_CONVEX_VERTICES_SHAPE);
        assert_eq!(
            order(&e),
            vec![
                0x00ccde60,
                CLONE_LOCK,
                OPERATOR_NEW,
                BASE_CONSTRUCT,
                0x0900_0101,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, clone, 0x4242]]);
        assert_eq!(scope(&mut e), 0x77);
        // A failed allocation clones into null.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ccde60, &args![this, 1u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, 0, 1]]);
    }

    #[test]
    fn init_builds_the_havok_shape_from_the_cinfo() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        // The memory router and its allocator (slot 4 allocates).
        let allocator = object(&mut e, &[(4, 0x0900_0200)]);
        e.register(0x0900_0200, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        e.map(0x0200_3000, 0x10);
        e.mem.set_u32(0x0200_3000, router);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(0x0200_3000),
            ..Ret::default()
        });
        e.map(DEFAULT_RADIUS, 4);
        e.mem.set_u32(DEFAULT_RADIUS, 0x3f80_0000);
        e.register_double(HK_VERTICES_CONSTRUCT, |e, a| {
            // The view of the vertices is read while the call runs.
            let view = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8));
            assert_eq!(view, (0xda7a, 3, 0x10));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        quiet(&mut e, &[REMOVE_REFERENCE, 0x0900_0103, 0x0900_0104]);
        let this = object(&mut e, &[(0x8c, 0x0900_0103), (0xbc, 0x0900_0104)]);
        let cinfo = e.mem.alloc(0x40);
        e.mem.set_u32(cinfo + 8, 0xda7a);
        e.mem.set_u32(cinfo + 0xc, 3);
        e.call_log = Some(vec![]);
        e.call(0x00ccdf20, &args![this, cinfo]);
        let built = calls_to(&e, HK_VERTICES_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][2..], [cinfo + 0x14, 0x3f80_0000]);
        let block = built[0][0];
        assert_eq!(e.mem.u16(block + 4), 0x70);
        assert_eq!(calls_to(&e, 0x0900_0103), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, 0x0900_0104), vec![vec![this, cinfo]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn copy_cinfo_fills_the_arrays_from_the_havok_shape() {
        let mut e = Engine::new();
        install_allocator(&mut e);
        quiet(&mut e, &[CONVEX_SHAPE_COPY_CINFO, GET_ORIGINAL_VERTICES]);
        // The plane equations: an array of two 16-byte entries.
        let planes = e.mem.alloc(0x10);
        let entries = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(entries + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(planes, entries);
        e.mem.set_u32(planes + 4, 2);
        e.register_double(GET_PLANE_EQUATIONS, move |_, _| Ret {
            eax: planes,
            ..Ret::default()
        });
        let this = shape(&mut e);
        let havok = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, havok);
        let cinfo = e.mem.alloc(0x40);
        // The array has room for one entry of an owned block: it is freed and replaced.
        let old = e.mem.alloc(16);
        e.mem.set_u32(cinfo + 0x14, old);
        e.mem.set_u32(cinfo + 0x1c, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cce010, &args![this, cinfo]);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_COPY_CINFO),
            vec![vec![this, cinfo]]
        );
        assert_eq!(
            calls_to(&e, GET_ORIGINAL_VERTICES),
            vec![vec![havok, cinfo + 8]]
        );
        assert_eq!(calls_to(&e, 0x0900_0201)[0][1..], [old, 16]);
        // The block asked for 32 bytes and was granted 48: capacity 3.
        assert_eq!(e.mem.u32(cinfo + 0x1c), 3);
        assert_eq!(e.mem.u32(cinfo + 0x18), 2);
        let data = e.mem.u32(cinfo + 0x14);
        assert_ne!(data, old);
        for i in 0..8u32 {
            assert_eq!(e.mem.u32(data + 4 * i), 0x100 + i);
        }
        // Enough capacity: nothing is freed or allocated.
        e.call_log = Some(vec![]);
        e.call(0x00cce010, &args![this, cinfo]);
        assert!(calls_to(&e, 0x0900_0201).is_empty());
        assert_eq!(e.mem.u32(cinfo + 0x14), data);
        // Not owned storage is not freed when it is replaced.
        e.mem.set_u32(cinfo + 0x1c, 0x8000_0000);
        e.call_log = Some(vec![]);
        e.call(0x00cce010, &args![this, cinfo]);
        assert!(calls_to(&e, 0x0900_0201).is_empty());
        assert_eq!(e.mem.u32(cinfo + 0x1c), 3);
        // No Havok shape: only the base copy.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cce010, &args![this, cinfo]);
        assert!(calls_to(&e, GET_ORIGINAL_VERTICES).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00cce010, &args![0u32, cinfo]);
        assert!(calls_to(&e, GET_ORIGINAL_VERTICES).is_empty());
    }

    #[test]
    fn releasing_an_array_frees_only_owned_storage() {
        let mut e = Engine::new();
        install_allocator(&mut e);
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, 4);
        e.call_log = Some(vec![]);
        e.call(0x00cce0d0, &args![array]);
        assert_eq!(calls_to(&e, 0x0900_0201)[0][1..], [data, 64]);
        assert_eq!(
            (e.mem.u32(array), e.mem.u32(array + 4), e.mem.u32(array + 8)),
            (0, 0, 0x8000_0000)
        );
        // Now not owned: no free.
        e.mem.set_u32(array, data);
        e.call_log = Some(vec![]);
        e.call(0x00cce0d0, &args![array]);
        assert!(calls_to(&e, 0x0900_0201).is_empty());
        assert_eq!(e.mem.u32(array), 0);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_text() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        e.set_global(CLASS_TEXT, 0x1111u32);
        quiet(&mut e, &[CONVEX_SHAPE_GET_VIEWER_STRINGS, 0x0096_ad30]);
        e.register(VIEWER_STRING, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(0x0096_ae90, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], text);
            let count = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, count + 1);
            Ret::default()
        });
        let list = e.mem.alloc(0x200);
        e.mem.set_u16(list + 8, 8);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00cce110, &args![this, list]);
        assert_eq!(e.mem.u32(list + 0x100), 0x1112);
        assert_eq!(e.mem.u16(list + 0xa), 1);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_GET_VIEWER_STRINGS),
            vec![vec![this, list]]
        );
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn create_cinfo_builds_an_empty_cinfo_once() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        quiet(&mut e, &[CONVEX_SHAPE_COPY_CINFO, GET_ORIGINAL_VERTICES]);
        e.map(DEFAULT_RADIUS, 4);
        e.mem.set_u32(DEFAULT_RADIUS, 0x7f80_0001);
        e.register(POOL_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let this = shape(&mut e);
        let created = e.mem.alloc(8);
        e.mem.set_u8(created, 9);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00cce1c0, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&e, POOL_ALLOCATE), vec![vec![POOL, 0x20]]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(cinfo + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0, 0x7fc0_0001, 0, 0, 0x8000_0000, 0, 0, 0x8000_0000]
        );
        // No Havok shape: no CopyCinfo.
        assert!(calls_to(&e, GET_ORIGINAL_VERTICES).is_empty());
        assert_eq!(scope(&mut e), 0x77);
        // The second call finds it.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cce1c0, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert!(calls_to(&e, POOL_ALLOCATE).is_empty());
        // With a Havok shape the base copy runs for the new cinfo (the plane lookup is
        // stopped by a double).
        let other = shape(&mut e);
        let havok = e.mem.alloc(0x40);
        e.mem.set_u32(other + 8, havok);
        let planes = e.mem.alloc(0x10);
        e.register_double(GET_PLANE_EQUATIONS, move |_, _| Ret {
            eax: planes,
            ..Ret::default()
        });
        install_allocator(&mut e);
        e.call_log = Some(vec![]);
        let filled = e.call(0x00cce1c0, &args![other, created]).u32();
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_COPY_CINFO),
            vec![vec![other, filled]]
        );
        assert_eq!(
            calls_to(&e, GET_ORIGINAL_VERTICES),
            vec![vec![havok, filled + 8]]
        );
    }

    #[test]
    fn releasing_a_cinfo_frees_the_planes_then_the_vertices() {
        let mut e = Engine::new();
        install_allocator(&mut e);
        let cinfo = e.mem.alloc(0x40);
        let vertices = e.mem.alloc(0x20);
        let planes = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 8, vertices);
        e.mem.set_u32(cinfo + 0xc, 2);
        e.mem.set_u32(cinfo + 0x10, 2);
        e.mem.set_u32(cinfo + 0x14, planes);
        e.mem.set_u32(cinfo + 0x18, 1);
        e.mem.set_u32(cinfo + 0x1c, 4);
        e.call_log = Some(vec![]);
        e.call(0x00cce290, &args![cinfo]);
        let frees = calls_to(&e, 0x0900_0201);
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0][1..], [planes, 64]);
        assert_eq!(frees[1][1..], [vertices, 32]);
        assert_eq!(e.mem.u32(cinfo + 0x1c), 0x8000_0000);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
        assert_eq!(e.mem.u32(cinfo + 0x14), 0);
    }

    #[test]
    fn save_binary_writes_the_arrays_through_locals_and_swaps_when_asked() {
        let mut e = Engine::new();
        install_allocator(&mut e);
        quiet(&mut e, &[CONVEX_SHAPE_SAVE_BINARY, SWAP_HK_ARRAY]);
        // The stream writer records the arrays it is given.
        e.register(STREAM_SAVE_ARRAY, |e, a| {
            let words = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8));
            e.mem.set_u32(a[0] + 0x100, words.0);
            e.mem.set_u32(a[0] + 0x104, words.1);
            e.mem.set_u32(a[0] + 0x108, words.2);
            Ret::default()
        });
        let this = object(
            &mut e,
            &[
                (0xb4, 0x0900_0100),
                (0xa4, 0x0900_0103),
                (0xa8, 0x0900_0102),
            ],
        );
        // Slot 0xB4: answers +0xC and fills the created flag from +0x10.
        e.register(0x0900_0100, |e, a| {
            let flag = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[1], flag);
            Ret {
                eax: e.mem.u32(a[0] + 0xc),
                ..Ret::default()
            }
        });
        e.register(0x0900_0102, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        quiet(&mut e, &[0x0900_0103]);
        let cinfo = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, cinfo);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 8, data);
        e.mem.set_u32(cinfo + 0xc, 2);
        e.mem.set_u32(cinfo + 0x10, 2);
        e.mem.set_u32(cinfo + 0x14, 0x5000);
        e.mem.set_u32(cinfo + 0x18, 1);
        e.mem.set_u32(cinfo + 0x1c, 0x8000_0001);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e4, 1);
        // The created flag at this + 0x10 is 0 here.
        e.mem.set_u32(this + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cce340, &args![this, stream]);
        // The writer saw the moved arrays; the first one was freed at the end (2 * 16).
        let saves = calls_to(&e, STREAM_SAVE_ARRAY);
        assert_eq!(saves.len(), 2);
        assert_eq!(saves[0][0], stream);
        assert_eq!(
            (e.mem.u32(stream + 0x100), e.mem.u32(stream + 0x104)),
            (0x5000, 1)
        );
        // The cinfo's arrays were left empty and not owned.
        for offset in [8, 0x14] {
            assert_eq!(
                (
                    e.mem.u32(cinfo + offset),
                    e.mem.u32(cinfo + offset + 4),
                    e.mem.u32(cinfo + offset + 8)
                ),
                (0, 0, 0x8000_0000)
            );
        }
        let frees = calls_to(&e, 0x0900_0201);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1..], [data, 32]);
        assert!(calls_to(&e, SWAP_HK_ARRAY).is_empty());
        assert_eq!(calls_to(&e, 0x0900_0103), vec![vec![this, 0]]);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        // The other endianness: swaps before the base save, and again afterwards when
        // the cinfo was not just created.
        e.mem.set_u8(stream + 0x1e4, 0);
        e.mem.set_u32(cinfo + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cce340, &args![this, stream]);
        assert_eq!(calls_to(&e, SWAP_HK_ARRAY).len(), 4);
        // A just created cinfo is swapped only once, and its flag reaches slot 0xA4.
        e.mem.set_u32(this + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cce340, &args![this, stream]);
        assert_eq!(calls_to(&e, SWAP_HK_ARRAY).len(), 2);
        assert_eq!(calls_to(&e, 0x0900_0103), vec![vec![this, 1]]);
    }

    #[test]
    fn the_geometry_has_two_arrays_that_are_released_second_first() {
        let mut e = Engine::new();
        install_allocator(&mut e);
        let geometry = e.mem.alloc(0x20);
        e.call(0x00cce5e0, &args![geometry]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(geometry + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0x8000_0000, 0, 0, 0x8000_0000]);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(geometry, first);
        e.mem.set_u32(geometry + 4, 1);
        e.mem.set_u32(geometry + 8, 2);
        e.mem.set_u32(geometry + 0xc, second);
        e.mem.set_u32(geometry + 0x10, 1);
        e.mem.set_u32(geometry + 0x14, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cce530, &args![geometry]);
        let frees = calls_to(&e, 0x0900_0201);
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0][1..], [second, 16]);
        assert_eq!(frees[1][1..], [first, 32]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(geometry + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0x8000_0000, 0, 0, 0x8000_0000]);
    }

    #[test]
    fn build_display_geometry_makes_a_convex_geometry_from_the_vertices() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        install_allocator(&mut e);
        // The array reserve gives the array room for the asked entries.
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(16 * a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        // The Havok shape: slot 0x2C answers the count 2, slot 0x30 fills and answers the
        // vertices.
        let havok = object(&mut e, &[(0x2c, 0x0900_0300), (0x30, 0x0900_0301)]);
        e.register(0x0900_0300, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        let vertices = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(vertices + 4 * i, 0x500 + i);
        }
        e.mem.set_u32(havok + 0x40, vertices);
        e.register(0x0900_0301, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        // The geometry utility checks the vertex view it is given: two copied vertices with
        // capacity 0x10, and an empty geometry.
        e.register(CREATE_CONVEX_GEOMETRY, |e, a| {
            let data = e.mem.u32(a[0]);
            assert_eq!((e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8)), (2, 0x10));
            for i in 0..8u32 {
                assert_eq!(e.mem.u32(data + 4 * i), 0x500 + i);
            }
            assert_eq!((e.mem.u32(a[1]), e.mem.u32(a[1] + 8)), (0, 0x8000_0000));
            Ret::default()
        });
        let this = object(&mut e, &[(0xdc, 0x0900_0302)]);
        quiet(&mut e, &[0x0900_0302]);
        e.mem.set_u32(this + 8, havok);
        let parent = 0x7000u32;
        e.call_log = Some(vec![]);
        e.call(0x00cce600, &args![this, parent, 0x3131u32]);
        let view_call = calls_to(&e, CREATE_CONVEX_GEOMETRY);
        assert_eq!(view_call.len(), 1);
        let geometry = view_call[0][1];
        assert_eq!(view_call[0][0] + 0x0c, geometry);
        assert_eq!(
            calls_to(&e, 0x0900_0302),
            vec![vec![this, parent, geometry, 0x3131, GEOMETRY_NAME]]
        );
        assert_eq!(calls_to(&e, 0x0900_0301).len(), 1);
        // The two arrays of 2 entries are freed; the geometry's are empty and not owned.
        let frees = calls_to(&e, 0x0900_0201);
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0][2], 32);
        assert_eq!(frees[1][2], 32);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn kill_cinfo_releases_the_cinfo_only_when_asked() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        install_allocator(&mut e);
        quiet(&mut e, &[POOL_RELEASE]);
        let this = e.mem.alloc(0x20);
        let cinfo = e.mem.alloc(0x40);
        let vertices = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 8, vertices);
        e.mem.set_u32(cinfo + 0x10, 2);
        e.mem.set_u32(this + 0xc, cinfo);
        e.call_log = Some(vec![]);
        e.call(0x00cce860, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert!(calls_to(&e, POOL_RELEASE).is_empty());
        e.call(0x00cce860, &args![this, 0x0100u32]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        e.call(0x00cce860, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&e, POOL_RELEASE), vec![vec![POOL, cinfo]]);
        // The plane array (empty) is released first, then the vertices.
        assert_eq!(calls_to(&e, 0x0900_0201)[1][1..], [vertices, 32]);
        // No cinfo: still clears +0xC.
        e.mem.set_u32(this + 0xc, 0);
        e.call(0x00cce860, &args![this, 1u32]);
        assert_eq!(calls_to(&e, POOL_RELEASE).len(), 1);
        assert_eq!(scope(&mut e), 0x77);
    }
}
