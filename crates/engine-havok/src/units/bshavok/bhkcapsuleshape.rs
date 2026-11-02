//! `BSHavok/bhkcapsuleshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkCapsuleShape` (0x14 bytes, vtable `01066e2c`) wraps the Havok `hkpCapsuleShape`
//! (0x40 bytes) at `+8`: its two end points sit at `+0x20` and `+0x30` and its radius at
//! `+0x10`. The creation info (`hkCapsuleShapeCinfo`, 0x30 bytes) keeps the radius at `+4` and
//! the end points at `+0x10` and `+0x20`.
//!
//! Floats: x87 `FLD`/`FSTP` copies go through [`x87_quiet`]; the x87 arithmetic is done in
//! `f64` and rounded to `f32` where the exe stores to memory; the SSE code runs in `f32`.
//! `RSQRTSS` is the host CPU's estimate (`rsqrt_estimate`). The exception-unwinding frames
//! (`FS:[0]`) and the `/GS` cookie checks (`00ec408c`) of the functions marked `Translation
//! incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `bhkSerializable::Kill` (engine map name, `00c876a0`) and the base constructor of the
/// 0x10-byte serializable part (`00c87c30`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkConvexShape` base functions (engine map names): destructor (`00c9fb00`), `CopyCinfo`
/// (`00c9fb70`), `LoadBinary` (`00c9fbc0`) and `GetViewerStrings` (`00c9fc50`).
const CONVEX_DESTRUCT: u32 = 0x00c9_fb00;
const CONVEX_COPY_CINFO: u32 = 0x00c9_fb70;
const CONVEX_LOAD_BINARY: u32 = 0x00c9_fbc0;
const CONVEX_GET_VIEWER_STRINGS: u32 = 0x00c9_fc50;
/// `bhkShape::CopyMembers` (engine map name, `00c84df0`).
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
/// `bhkSerializable::CloneCritLock` / `CloneCritUnlock` (engine map names).
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `operator new` (`00aa13e0`, cdecl size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The memory manager (`011f6238`), allocating through `00aa3e40` (thiscall, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The memory router (`00c85750`): its field `+0x10` is the allocator (virtual slot 4 allocates).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Constructor of the Havok capsule (`00d22590`, thiscall on the new block, three stack words:
/// the two end points and the radius).
const HAVOK_CAPSULE_CONSTRUCT: u32 = 0x00d2_2590;
/// Constructor of the capsule's display geometry (`00d22dc0`, thiscall on the new block, five
/// stack words: the two end points, the radius, 6 and 1; the engine map does not name it).
const DISPLAY_CAPSULE_CONSTRUCT: u32 = 0x00d2_2dc0;
/// Reference count decrement of a Havok object (`00c905b0`; the engine map does not name it).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The vector copy (`004b4db0`, thiscall on the destination, one stack word: the source) the
/// Cinfo copy uses for the end points.
const COPY_VECTOR: u32 = 0x004b_4db0;
/// The shared zero vector the Cinfo copy takes when there is no Havok capsule.
const ZERO_VECTOR: u32 = 0x0126_7e30;
/// The Havok closest point on a line segment (`00d230e0`, cdecl: the point, the two end
/// points, the result vector).
const CLOSEST_POINT_ON_SEGMENT: u32 = 0x00d2_30e0;
/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`)
/// and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// Vtables of the chain of base classes the constructor steps through, ending in
/// `bhkCapsuleShape`'s, and the object counters it raises.
const BASE_VTABLE_A: u32 = 0x0103_0bf4;
const BASE_VTABLE_B: u32 = 0x0103_0b0c;
const BASE_VTABLE_C: u32 = 0x0103_0a24;
const CAPSULE_SHAPE_VTABLE: u32 = 0x0106_6e2c;
const BASE_COUNT_A: u32 = 0x0126_80f8;
const BASE_COUNT_B: u32 = 0x0126_8274;
const BASE_COUNT_C: u32 = 0x0126_8250;
const CAPSULE_SHAPE_COUNT: u32 = 0x0126_8324;
/// `bhkCapsuleShape::ms_RTTI` (its first word is the class name).
const CAPSULE_SHAPE_RTTI: u32 = 0x0126_832c;
/// The Havok scale (`float`, `01268328`) the point constructor multiplies its inputs by.
const HAVOK_SCALE: u32 = 0x0126_8328;
/// `float` default radius of a Cinfo (`010c72bc`), and the constants 0.0, 3.0 and 0.5 (the
/// `RSQRTSS` refinement).
const DEFAULT_RADIUS: u32 = 0x010c_72bc;
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// The name the display geometry is built under (`"bhkCapsuleShape"`).
const DISPLAY_NAME: u32 = 0x010c_3f3c;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

// `hkpCapsuleShape` (Xbox PDB) offsets used here: `m_radius`, `m_vertexA`, `m_vertexB`.
const HAVOK_RADIUS: u32 = 0x10;
const HAVOK_VERTEX_A: u32 = 0x20;
const HAVOK_VERTEX_B: u32 = 0x30;
// `hkCapsuleShapeCinfo` (Xbox PDB) offsets: `m_vertexA`, `m_vertexB`.
const CINFO_VERTEX_A: u32 = 0x10;
const CINFO_VERTEX_B: u32 = 0x20;

layout! {
    /// `bhkCapsuleShape` (Xbox PDB, 0x14 bytes): a `bhkShape`.
    pub struct BhkCapsuleShape: 0x14 {
        /// `phkObject` (Xbox PDB): the `hkpCapsuleShape`.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB): the Cinfo, as `CreateCinfo` (virtual `+0xb4`) answers it.
        0x0C pInfo: Ptr,
        /// `eMaterial` (Xbox PDB).
        0x10 eMaterial: u32,
    }
}

layout! {
    /// `hkCapsuleShapeCinfo` (Xbox PDB, 0x30 bytes) as far as the unit reads it; the two end
    /// points are 16-byte vectors at `+0x10` and `+0x20`.
    pub struct HkCapsuleShapeCinfo: 0x30 {
        /// `m_userData` (Xbox PDB).
        0x00 m_userData: i32,
        /// `m_radius` (Xbox PDB).
        0x04 m_radius: f32,
    }
}

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, address: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(address + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, address: u32, lanes: Vec4) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, value);
    }
}

/// A bit-for-bit copy of `words` dwords.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// An x87 `FLD dword` of the float at `address`.
fn load_quiet(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.mem.f32(address))
}

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
fn sum3(lanes: Vec4) -> f32 {
    lanes[2] + (lanes[1] + lanes[0])
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero guard:
/// `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == zero`.
fn inverse_length(e: &Engine, sum: f32) -> f32 {
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// The vector scaled by the inverse of its length (all four lanes).
fn normalized(e: &Engine, vector: Vec4) -> Vec4 {
    let squares = vector.map(|lane| lane * lane);
    let inverse = inverse_length(e, sum3(squares));
    vector.map(|lane| inverse * lane)
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

/// Calls `CreateCinfo` (virtual `+0xb4`, with the address of a scratch bool) and answers the
/// Cinfo.
fn create_cinfo(e: &mut Engine, this: u32) -> u32 {
    e.with_stack(4, |e, created| e.vcall(this, 0xb4, &args![created]).u32())
}

/// The end point of the Havok capsule at `offset` for the Cinfo copy: the point itself, or
/// the shared zero vector when there is no shape or no Havok capsule.
fn end_point_source(e: &Engine, this: u32, offset: u32) -> u32 {
    if this != 0 {
        let capsule = e.mem.u32(this + 8);
        if capsule != 0 {
            return capsule + offset;
        }
    }
    ZERO_VECTOR
}

/// The base constructors' steps, common to the two constructors: the base constructor
/// (`00c87c30`), then each class's vtable stored and its object counter raised; `+0xc` and
/// `+0x10` are cleared.
fn construct(e: &mut Engine, this: u32) {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    for (vtable, counter) in [
        (BASE_VTABLE_A, BASE_COUNT_A),
        (BASE_VTABLE_B, BASE_COUNT_B),
        (BASE_VTABLE_C, BASE_COUNT_C),
        (CAPSULE_SHAPE_VTABLE, CAPSULE_SHAPE_COUNT),
    ] {
        e.mem.set_u32(this, vtable);
        let count = e.mem.u32(counter);
        e.mem.set_u32(counter, count.wrapping_add(1));
    }
}

// Translated from 00caacf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caacf0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCapsuleShape::~bhkCapsuleShape` (Xbox PDB): sets the vtable, runs `Kill` (`00c876a0`),
/// lowers the object counter (`01268324`) and runs `~bhkConvexShape` (`00c9fb00`).
pub fn bhk_capsule_shape_destructor(e: &mut Engine, this: Ptr<BhkCapsuleShape>) {
    e.mem.set_u32(this.addr(), CAPSULE_SHAPE_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(CAPSULE_SHAPE_COUNT);
    e.mem.set_u32(CAPSULE_SHAPE_COUNT, count.wrapping_sub(1));
    e.call(CONVEX_DESTRUCT, &args![this]);
}

// Translated from 00caad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the first three lanes of the vector at `vector` into the vector at `this + 0x10`
/// (unconfirmed name; the fourth lane stays).
pub fn fn_00caad60(e: &mut Engine, this: Ptr, vector: Ptr) {
    copy_words(e, vector.addr(), this.addr() + 0x10, 3);
}

// Translated from 00caad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCapsuleShape::bhkCapsuleShape()` (Xbox PDB): constructs the shape in place through its
/// base classes. Answers `this`.
pub fn bhk_capsule_shape_constructor(e: &mut Engine, this: Ptr<BhkCapsuleShape>) -> Ptr {
    construct(e, this.addr());
    this.cast()
}

// Translated from 00caade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCapsuleShape::CopyCinfo` (Xbox PDB), `(cinfo)`: the `bhkConvexShape` copy (`00c9fb70`),
/// then the capsule's end points into the Cinfo's `m_vertexA` and `m_vertexB` (`004b4db0`; the
/// shared zero vector `01267e30` when there is no Havok capsule).
pub fn bhk_capsule_shape_copy_cinfo(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    cinfo: Ptr<HkCapsuleShapeCinfo>,
) {
    e.call(CONVEX_COPY_CINFO, &args![this, cinfo]);
    let source = end_point_source(e, this.addr(), HAVOK_VERTEX_A);
    e.call(COPY_VECTOR, &args![cinfo.addr() + CINFO_VERTEX_A, source]);
    let source = end_point_source(e, this.addr(), HAVOK_VERTEX_B);
    e.call(COPY_VECTOR, &args![cinfo.addr() + CINFO_VERTEX_B, source]);
}

// Translated from 00caae30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caae30 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCapsuleShape::CreateObject` (Xbox PDB): allocates (0x14 bytes, `operator new`) and
/// constructs a `bhkCapsuleShape`. Answers the object, or 0 when the allocation failed.
pub fn bhk_capsule_shape_create_object(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct(e, block);
    Ptr::new(block)
}

// Translated from 00caaea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCapsuleShape::CopyMembers` (Xbox PDB), `(clone, process)`: takes the shape's Cinfo
/// (`CreateCinfo`, virtual `+0xb4`) and, when the float at `process + 0x10` is not 1.0 (NaN
/// included), multiplies the Cinfo's end points (`+0x10` .. `+0x2c`) and then its radius
/// (`+4`) by it in place; then runs `bhkShape::CopyMembers` (`00c84df0`).
pub fn bhk_capsule_shape_copy_members(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    clone: Ptr,
    process: Ptr,
) {
    let cinfo = create_cinfo(e, this.addr());
    let scale = load_quiet(e, process.addr() + 0x10);
    if scale != 1.0 {
        let offsets = [0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x4];
        for offset in offsets {
            let value = load_quiet(e, cinfo + offset);
            e.mem
                .set_f32(cinfo + offset, (f64::from(value) * f64::from(scale)) as f32);
        }
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, clone, process]);
}

// Translated from 00caaf40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caaf40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCapsuleShape::CreateClone` (Xbox PDB), `(process)`: inside the
/// `TLS + 0x2b4 = 0x10` scope and the clone lock (`00c876f0`, `00c87700`), constructs a new
/// shape (0 when the allocation fails) and runs `CopyMembers` (virtual `+0xc4`, with the
/// clone and the process) on `this`. Answers the clone.
pub fn bhk_capsule_shape_create_clone(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    process: Ptr,
) -> Ptr {
    let scope = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = bhk_capsule_shape_create_object(e);
    e.vcall(this.addr(), 0xc4, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, scope);
    clone
}

// Translated from 00cab000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cab000 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCapsuleShape::CreateCinfo` (Xbox PDB), `(created)`: inside the `TLS + 0x2b4 = 0x10`
/// scope, writes whether there was no Cinfo yet to the byte at `created`; if so, allocates a
/// 0x30-byte Cinfo block from the memory manager (`00aa3e40`) holding the default radius
/// (`010c72bc`), the end points (0, 0, 0, 0) and (1, 1, 1, 1) and zero user data, stores it at
/// `+0xc`, and, when the shape has its Havok capsule (`+8`), runs `CopyCinfo` (`00caade0`) on
/// it. Answers the stored Cinfo.
pub fn bhk_capsule_shape_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    created: Ptr,
) -> Ptr {
    let scope = enter_scope(e);
    let is_new = e.get(this, BhkCapsuleShape::pInfo).addr() == 0;
    e.mem.set_u8(created.addr(), is_new as u8);
    if is_new {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x30u32])
            .u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
            let radius = load_quiet(e, DEFAULT_RADIUS);
            e.mem.set_f32(block + 4, radius);
            store_vec(e, block + CINFO_VERTEX_A, [0.0; 4]);
            store_vec(e, block + CINFO_VERTEX_B, [1.0; 4]);
        }
        e.set(this, BhkCapsuleShape::pInfo, Ptr::new(block));
        if e.get(this, BhkCapsuleShape::phkObject).addr() != 0 {
            bhk_capsule_shape_copy_cinfo(e, this, Ptr::new(block));
        }
    }
    let cinfo = e.get(this, BhkCapsuleShape::pInfo);
    leave_scope(e, scope);
    cinfo
}

// Translated from 00cab0e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cab0e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCapsuleShape::Init` (Xbox PDB, the overload taking the two end point vectors and the
/// radius): inside the `TLS + 0x2b4 = 0x10` scope, allocates the 0x40-byte Havok capsule from
/// the memory router's allocator (virtual slot 4) with its 16-bit size stored at `+4`,
/// constructs it with `00d22590` on `(vertex_a, vertex_b, radius)`, hands it to `SethkObject`
/// (virtual `+0x8c`) and drops the construction's reference (`00c905b0`).
pub fn bhk_capsule_shape_init(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    vertex_a: Ptr,
    vertex_b: Ptr,
    radius: f32,
) {
    let scope = enter_scope(e);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x40u32]).u32();
    e.mem.set_u16(block + 4, 0x40);
    let radius = x87_quiet_bits(radius.to_bits());
    let capsule = e
        .call(
            HAVOK_CAPSULE_CONSTRUCT,
            &args![block, vertex_a, vertex_b, radius],
        )
        .u32();
    e.vcall(this.addr(), 0x8c, &args![capsule]);
    e.call(REMOVE_REFERENCE, &args![capsule]);
    leave_scope(e, scope);
}

// Translated from 00cab1b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cab1b0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkCapsuleShape::BuildDisplayGeometry` (Xbox PDB), `(node, color)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, fills a local Cinfo (the default radius `010c72bc`, end points
/// (0, 0, 0, 0) and (1, 1, 1, 1)) with `CopyCinfo` (`00c9fb70`) and the capsule's end points
/// (`004b4db0`), allocates 0x90 bytes from the memory router's allocator (size stored at `+4`),
/// constructs the display capsule with `00d22dc0` on `(vertex A, vertex B, radius, 6, 1)` and
/// runs its virtual `+0x10` (unconfirmed). When its geometry (`+8`) exists, builds the display
/// from it with `BuildDisplayFromhkGeometry` (virtual `+0xdc`: node, geometry, color,
/// `"bhkCapsuleShape"`).
pub fn bhk_capsule_shape_build_display_geometry(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    node: Ptr,
    color: Ptr,
) {
    let scope = enter_scope(e);
    // The local Cinfo (0x30 bytes) and the two end point copies passed to the constructor.
    let geometry = e.with_stack(0x50, |e, cinfo| {
        let base = cinfo.addr();
        let radius = load_quiet(e, DEFAULT_RADIUS);
        e.mem.set_f32(base + 4, radius);
        store_vec(e, base + CINFO_VERTEX_A, [0.0; 4]);
        store_vec(e, base + CINFO_VERTEX_B, [1.0; 4]);
        e.call(CONVEX_COPY_CINFO, &args![this, cinfo]);
        let source = end_point_source(e, this.addr(), HAVOK_VERTEX_A);
        e.call(COPY_VECTOR, &args![base + CINFO_VERTEX_A, source]);
        let source = end_point_source(e, this.addr(), HAVOK_VERTEX_B);
        e.call(COPY_VECTOR, &args![base + CINFO_VERTEX_B, source]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let block = e.vcall(allocator, 4, &args![0x90u32]).u32();
        e.mem.set_u16(block + 4, 0x90);
        let radius = x87_quiet_bits(e.mem.u32(base + 4));
        let copy_a = base + 0x30;
        let copy_b = base + 0x40;
        copy_words(e, base + CINFO_VERTEX_A, copy_a, 4);
        copy_words(e, base + CINFO_VERTEX_B, copy_b, 4);
        let display = e
            .call(
                DISPLAY_CAPSULE_CONSTRUCT,
                &args![block, copy_a, copy_b, radius, 6u32, 1u32],
            )
            .u32();
        e.vcall(display, 0x10, &args![]);
        e.mem.u32(display + 8)
    });
    if geometry != 0 {
        e.vcall(
            this.addr(),
            0xdc,
            &args![node, geometry, color, DISPLAY_NAME],
        );
    }
    leave_scope(e, scope);
}

// Translated from 00cab3c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cab3c0 - the /GS cookie check is not modeled; RSQRTSS is the host estimate.
/// `bhkCapsuleShape::GetClosestPoints` (Xbox PDB), `(sphere, transform, contact)`: takes the
/// capsule's end points into the transform's space (the SSE code of the exe: the transform's
/// four rows are rows 0 to 2 and the translation), finds the closest point of that segment to
/// the sphere's centre (`00d230e0`) and compares the squared distance with the squared sum of
/// the sphere's radius (`sphere + 0xc`) and the capsule's. Answers false when the squared
/// radius is not above the squared distance (a NaN counts as a hit). Otherwise the contact is
/// written: the normal (the centre minus the closest point, normalised; for a centre on the
/// segment the unit vector perpendicular to the segment built from its two smallest
/// components) at `contact + 0x10` with its fourth lane kept, the position
/// `centre + normal * (radius - distance)` at `contact`, and the penetration
/// `distance - radius sum` in the fourth lane of the normal. The square root is the CRT's
/// `00ec6040`.
pub fn bhk_capsule_shape_get_closest_points(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    sphere: Ptr,
    transform: Ptr,
    contact: Ptr,
) -> bool {
    let shape = if this.addr() != 0 {
        e.get(this, BhkCapsuleShape::phkObject).addr()
    } else {
        0
    };
    let columns: [Vec4; 4] =
        std::array::from_fn(|column| load_vec(e, transform.addr() + 16 * column as u32));
    let into_space = |vertex: Vec4| -> Vec4 {
        std::array::from_fn(|lane| {
            let first = vertex[1] * columns[1][lane] + vertex[0] * columns[0][lane];
            let second = vertex[2] * columns[2][lane] + columns[3][lane];
            first + second
        })
    };
    let vertex_b = into_space(load_vec(e, shape + HAVOK_VERTEX_B));
    let vertex_a = into_space(load_vec(e, shape + HAVOK_VERTEX_A));
    let closest = e.with_stack(0x30, |e, scratch| {
        let base = scratch.addr();
        store_vec(e, base, vertex_a);
        store_vec(e, base + 0x10, vertex_b);
        e.call(
            CLOSEST_POINT_ON_SEGMENT,
            &args![sphere, base, base + 0x10, base + 0x20],
        );
        load_vec(e, base + 0x20)
    });
    let radius = load_quiet(e, shape + HAVOK_RADIUS);
    let sphere_radius = load_quiet(e, sphere.addr() + 0xc);
    let radius_sum = (f64::from(sphere_radius) + f64::from(radius)) as f32;
    let centre = load_vec(e, sphere.addr());
    let difference: Vec4 = std::array::from_fn(|lane| centre[lane] - closest[lane]);
    let distance_squared = sum3(difference.map(|lane| lane * lane));
    let radius_squared = (f64::from(radius_sum) * f64::from(radius_sum)) as f32;
    if radius_squared <= distance_squared {
        return false;
    }
    let (normal, distance) = if distance_squared > 0.0 {
        let root = e.call(X87_SQRT, &args![f64::from(distance_squared)]).f64() as f32;
        (normalized(e, difference), root)
    } else {
        // The centre is on the segment: the normal is perpendicular to the segment, built
        // from the two smallest of its components.
        let axis: Vec4 = std::array::from_fn(|lane| vertex_b[lane] - vertex_a[lane]);
        let magnitude = |lane: usize| x87_quiet(axis[lane]).abs();
        let (x, y, z) = (magnitude(0), magnitude(1), magnitude(2));
        let (mut kept, mut other, mut smallest) = (0usize, 1usize, 2usize);
        let mut least = x;
        if x > y {
            least = y;
            other = 0;
            kept = 1;
        }
        if least > z {
            smallest = kept;
        }
        let mut perpendicular = [0.0f32; 4];
        perpendicular[other] = x87_quiet(axis[smallest]);
        perpendicular[smallest] = -x87_quiet(axis[other]);
        (normalized(e, perpendicular), 0.0)
    };
    // The normal replaces the first three lanes at `contact + 0x10`.
    for (lane, value) in normal.into_iter().take(3).enumerate() {
        e.mem
            .set_f32(contact.addr() + 0x10 + 4 * lane as u32, value);
    }
    let depth = (f64::from(radius) - f64::from(distance)) as f32;
    let stored_normal = load_vec(e, contact.addr() + 0x10);
    let centre = load_vec(e, sphere.addr());
    let position: Vec4 = std::array::from_fn(|lane| depth * stored_normal[lane] + centre[lane]);
    store_vec(e, contact.addr(), position);
    let penetration = (f64::from(distance) - f64::from(radius_sum)) as f32;
    e.mem.set_f32(contact.addr() + 0x1c, penetration);
    true
}

// Translated from 00cab710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cab710 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkCapsuleShape::bhkCapsuleShape(NiPoint3 const&, NiPoint3 const&, float)` (Xbox PDB):
/// constructs the shape as `00caad90` does, scales the two points and the radius by the Havok
/// scale (`01268328`; the points become vectors with a fourth lane of 0) and runs `Init`
/// (`00cab0e0`) on them. Answers `this`.
pub fn bhk_capsule_shape_constructor_with_points(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    point_a: Ptr,
    point_b: Ptr,
    radius: f32,
) -> Ptr {
    construct(e, this.addr());
    let scale = load_quiet(e, HAVOK_SCALE);
    let scaled = |value: f32| (f64::from(x87_quiet(value)) * f64::from(scale)) as f32;
    let vertex_a: Vec4 = [
        scaled(e.mem.f32(point_a.addr())),
        scaled(e.mem.f32(point_a.addr() + 4)),
        scaled(e.mem.f32(point_a.addr() + 8)),
        0.0,
    ];
    let vertex_b: Vec4 = [
        scaled(e.mem.f32(point_b.addr())),
        scaled(e.mem.f32(point_b.addr() + 4)),
        scaled(e.mem.f32(point_b.addr() + 8)),
        0.0,
    ];
    let scaled_radius = x87_quiet((f64::from(scale) * f64::from(x87_quiet(radius))) as f32);
    e.with_stack(0x20, |e, vertices| {
        store_vec(e, vertices.addr(), vertex_a);
        store_vec(e, vertices.addr() + 0x10, vertex_b);
        bhk_capsule_shape_init(
            e,
            this,
            Ptr::new(vertices.addr()),
            Ptr::new(vertices.addr() + 0x10),
            scaled_radius,
        );
    });
    this.cast()
}

// Translated from 00cab820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cab820 - the /GS cookie check is not modeled.
/// `bhkCapsuleShape::Init` (Xbox PDB, the overload taking the Cinfo), `(cinfo)`: copies the
/// Cinfo's two end point vectors (bit for bit) and its radius (`+4`) and runs the other
/// `Init` (`00cab0e0`) on them, then `PostInit` (virtual `+0xbc`) with the Cinfo.
pub fn bhk_capsule_shape_init_from_cinfo(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    cinfo: Ptr<HkCapsuleShapeCinfo>,
) {
    let radius = x87_quiet(e.mem.f32(cinfo.addr() + 4));
    e.with_stack(0x20, |e, vertices| {
        copy_words(e, cinfo.addr() + CINFO_VERTEX_A, vertices.addr(), 4);
        copy_words(e, cinfo.addr() + CINFO_VERTEX_B, vertices.addr() + 0x10, 4);
        bhk_capsule_shape_init(
            e,
            this,
            Ptr::new(vertices.addr()),
            Ptr::new(vertices.addr() + 0x10),
            radius,
        );
    });
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
}

// Translated from 00cab8c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cab8c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCapsuleShape::GetViewerStrings` (Xbox PDB), `(strings)`: inside the `TLS + 0x2b4 = 0x10`
/// scope, the base strings (`00c9fc50`) and the class name (`NiGetViewerString` of the first
/// word of the RTTI at `0126832c`) stored in the string array.
pub fn bhk_capsule_shape_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkCapsuleShape>,
    strings: Ptr,
) {
    let scope = enter_scope(e);
    e.call(CONVEX_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(CAPSULE_SHAPE_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    store_text(e, strings.addr(), text);
    leave_scope(e, scope);
}

// Translated from 00ccd330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCapsuleShape::LoadBinary` (Xbox PDB), `(stream)`: the `bhkConvexShape` load
/// (`00c9fbc0`), then, unless the byte at `stream + 0x1e5` is 1, byte-swaps the eight words of
/// the Cinfo's end points (`+0x10` .. `+0x2c`).
pub fn bhk_capsule_shape_load_binary(e: &mut Engine, this: Ptr<BhkCapsuleShape>, stream: Ptr) {
    e.call(CONVEX_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e5) != 1 {
        let cinfo = create_cinfo(e, this.addr());
        for offset in (0x10..=0x2c).step_by(4) {
            let word = e.mem.u32(cinfo + offset);
            e.mem.set_u32(cinfo + offset, word.swap_bytes());
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00caacf0,
            bhk_capsule_shape_destructor(Ptr<BhkCapsuleShape>)
        ),
        entry!(0x00caad60, fn_00caad60(Ptr, Ptr)),
        entry!(
            0x00caad90,
            bhk_capsule_shape_constructor(Ptr<BhkCapsuleShape>) -> Ptr
        ),
        entry!(
            0x00caade0,
            bhk_capsule_shape_copy_cinfo(Ptr<BhkCapsuleShape>, Ptr<HkCapsuleShapeCinfo>)
        ),
        entry!(0x00caae30, bhk_capsule_shape_create_object() -> Ptr),
        entry!(
            0x00caaea0,
            bhk_capsule_shape_copy_members(Ptr<BhkCapsuleShape>, Ptr, Ptr)
        ),
        entry!(
            0x00caaf40,
            bhk_capsule_shape_create_clone(Ptr<BhkCapsuleShape>, Ptr) -> Ptr
        ),
        entry!(
            0x00cab000,
            bhk_capsule_shape_create_cinfo(Ptr<BhkCapsuleShape>, Ptr) -> Ptr
        ),
        entry!(
            0x00cab0e0,
            bhk_capsule_shape_init(Ptr<BhkCapsuleShape>, Ptr, Ptr, f32)
        ),
        entry!(
            0x00cab1b0,
            bhk_capsule_shape_build_display_geometry(Ptr<BhkCapsuleShape>, Ptr, Ptr)
        ),
        entry!(
            0x00cab3c0,
            bhk_capsule_shape_get_closest_points(Ptr<BhkCapsuleShape>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00cab710,
            bhk_capsule_shape_constructor_with_points(Ptr<BhkCapsuleShape>, Ptr, Ptr, f32) -> Ptr
        ),
        entry!(
            0x00cab820,
            bhk_capsule_shape_init_from_cinfo(Ptr<BhkCapsuleShape>, Ptr<HkCapsuleShapeCinfo>)
        ),
        entry!(
            0x00cab8c0,
            bhk_capsule_shape_get_viewer_strings(Ptr<BhkCapsuleShape>, Ptr)
        ),
        entry!(
            0x00ccd330,
            bhk_capsule_shape_load_binary(Ptr<BhkCapsuleShape>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOC_SLOT: u32 = 0x7000_0004;
    const COPY_MEMBERS_SLOT: u32 = 0x7000_00c4;
    const CREATE_CINFO_SLOT: u32 = 0x7000_00b4;
    const SET_HK_OBJECT_SLOT: u32 = 0x7000_008c;
    const POST_INIT_SLOT: u32 = 0x7000_00bc;
    const BUILD_DISPLAY_SLOT: u32 = 0x7000_00dc;
    const DISPLAY_SLOT: u32 = 0x7000_0010;
    const SHAPE_VTABLE: u32 = 0x7001_1000;
    const ALLOCATOR_VTABLE: u32 = 0x7001_2000;
    const DISPLAY_VTABLE: u32 = 0x7001_3000;
    const ROUTER_OBJECT: u32 = 0x7100_0000;

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

    /// Writes a vtable at `address` whose slots (byte offsets) point at the targets.
    fn put_vtable_at(e: &mut Engine, address: u32, entries: &[(u32, u32)]) {
        let words = entries
            .iter()
            .map(|(offset, _)| offset / 4 + 1)
            .max()
            .unwrap();
        let mut table = vec![0u32; words as usize];
        for (offset, target) in entries {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(address, &table);
    }

    /// A capsule shape with the vtable that has the slots the unit calls, answering `cinfo`
    /// from `CreateCinfo`.
    fn shape(e: &mut Engine) -> u32 {
        // The same table under the vtable the constructors store.
        for address in [SHAPE_VTABLE, CAPSULE_SHAPE_VTABLE] {
            put_vtable_at(
                e,
                address,
                &[
                    (0x8c, SET_HK_OBJECT_SLOT),
                    (0xb4, CREATE_CINFO_SLOT),
                    (0xbc, POST_INIT_SLOT),
                    (0xc4, COPY_MEMBERS_SLOT),
                    (0xdc, BUILD_DISPLAY_SLOT),
                ],
            );
        }
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, SHAPE_VTABLE);
        this
    }

    /// The memory router whose allocator (virtual slot 4) hands out zeroed blocks.
    fn install_allocator(e: &mut Engine) {
        put_vtable_at(e, ALLOCATOR_VTABLE, &[(4, ALLOC_SLOT)]);
        e.map(ROUTER_OBJECT, 0x40);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, ROUTER_OBJECT + 0x20);
        e.mem.set_u32(ROUTER_OBJECT + 0x20, ALLOCATOR_VTABLE);
        e.register(MEMORY_ROUTER, |_, _| ROUTER_OBJECT.into_ret());
        e.register(ALLOC_SLOT, |e, a| e.mem.alloc(a[1]).into_ret());
    }

    fn set_vec(e: &mut Engine, address: u32, lanes: Vec4) {
        store_vec(e, address, lanes);
    }

    fn close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-4,
            "{actual} is not close to {expected}"
        );
    }

    #[test]
    fn bhk_capsule_shape_destructor_runs_kill_then_the_convex_destructor() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(CONVEX_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(CAPSULE_SHAPE_COUNT, 3);
        e.call(0x00ca_acf0, &args![this]);
        assert_eq!(e.mem.u32(this), CAPSULE_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(CAPSULE_SHAPE_COUNT), 2);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, CONVEX_DESTRUCT), vec![vec![this]]);
        assert_eq!(
            order_of(&e, &[SERIALIZABLE_KILL, CONVEX_DESTRUCT]),
            vec![SERIALIZABLE_KILL, CONVEX_DESTRUCT]
        );
    }

    #[test]
    fn fn_00caad60_replaces_three_lanes_and_keeps_the_fourth() {
        let mut e = logged_engine();
        let this = e.mem.alloc(0x20);
        let vector = e.mem.alloc(0x10);
        set_vec(&mut e, this + 0x10, [1.0, 2.0, 3.0, 4.0]);
        set_vec(&mut e, vector, [5.0, 6.0, 7.0, 8.0]);
        e.call(0x00ca_ad60, &args![this, vector]);
        assert_eq!(load_vec(&e, this + 0x10), [5.0, 6.0, 7.0, 4.0]);
        assert_eq!(load_vec(&e, vector), [5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn bhk_capsule_shape_constructor_steps_through_the_base_classes() {
        let mut e = logged_engine();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 0xffff_ffff);
        e.mem.set_u32(this + 0x10, 0xffff_ffff);
        for (address, value) in [
            (BASE_COUNT_A, 1),
            (BASE_COUNT_B, 2),
            (BASE_COUNT_C, 3),
            (CAPSULE_SHAPE_COUNT, 4),
        ] {
            e.mem.set_u32(address, value);
        }
        assert_eq!(e.call(0x00ca_ad90, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), CAPSULE_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(BASE_COUNT_A), 2);
        assert_eq!(e.mem.u32(BASE_COUNT_B), 3);
        assert_eq!(e.mem.u32(BASE_COUNT_C), 4);
        assert_eq!(e.mem.u32(CAPSULE_SHAPE_COUNT), 5);
    }

    #[test]
    fn bhk_capsule_shape_copy_cinfo_copies_the_end_points_or_the_zero_vector() {
        let mut e = logged_engine();
        e.register(CONVEX_COPY_CINFO, |_, _| Ret::default());
        e.register(COPY_VECTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        let capsule = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, capsule);
        let cinfo = e.mem.alloc(0x30);
        e.call(0x00ca_ade0, &args![this, cinfo]);
        assert_eq!(calls_to(&e, CONVEX_COPY_CINFO), vec![vec![this, cinfo]]);
        assert_eq!(
            calls_to(&e, COPY_VECTOR),
            vec![
                vec![cinfo + 0x10, capsule + 0x20],
                vec![cinfo + 0x20, capsule + 0x30]
            ]
        );
        assert_eq!(
            order_of(&e, &[CONVEX_COPY_CINFO, COPY_VECTOR, COPY_VECTOR]),
            vec![CONVEX_COPY_CINFO, COPY_VECTOR, COPY_VECTOR]
        );
        // Without the Havok capsule, and without a shape, the zero vector is copied.
        let bare = e.mem.alloc(0x14);
        e.call(0x00ca_ade0, &args![bare, cinfo]);
        e.call(0x00ca_ade0, &args![0u32, cinfo]);
        let copies = calls_to(&e, COPY_VECTOR);
        assert_eq!(copies.len(), 6);
        for copy in &copies[2..] {
            assert_eq!(copy[1], ZERO_VECTOR);
        }
        assert_eq!(copies[4][0], cinfo + 0x10);
        assert_eq!(copies[5][0], cinfo + 0x20);
    }

    #[test]
    fn bhk_capsule_shape_create_object_allocates_0x14_bytes_and_constructs() {
        let mut e = logged_engine();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let object = e.call(0x00ca_ae30, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), CAPSULE_SHAPE_VTABLE);
        // A failed allocation answers 0.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00ca_ae30, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn bhk_capsule_shape_copy_members_scales_the_cinfo_unless_the_scale_is_one() {
        let mut e = logged_engine();
        e.register(SHAPE_COPY_MEMBERS, |_, _| Ret::default());
        let this = shape(&mut e);
        let cinfo = e.mem.alloc(0x30);
        e.register_double(CREATE_CINFO_SLOT, move |_, _| cinfo.into_ret());
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 2.0);
        e.mem.set_f32(cinfo + 4, 0.5);
        for index in 0..8u32 {
            e.mem.set_f32(cinfo + 0x10 + 4 * index, index as f32 + 1.0);
        }
        e.call(0x00ca_aea0, &args![this, 0x5000u32, process]);
        let scaled: Vec<f32> = (0..8).map(|i| e.mem.f32(cinfo + 0x10 + 4 * i)).collect();
        assert_eq!(scaled, vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]);
        assert_eq!(e.mem.f32(cinfo + 4), 1.0);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this, 0x5000, process]]
        );
        // A scale of 1.0 leaves the Cinfo alone; a NaN scale does not.
        e.mem.set_f32(process + 0x10, 1.0);
        e.call(0x00ca_aea0, &args![this, 0x5000u32, process]);
        assert_eq!(e.mem.f32(cinfo + 4), 1.0);
        assert_eq!(e.mem.f32(cinfo + 0x10), 2.0);
        e.mem.set_u32(process + 0x10, 0x7fc0_0000);
        e.call(0x00ca_aea0, &args![this, 0x5000u32, process]);
        assert!(e.mem.f32(cinfo + 4).is_nan());
        assert!(e.mem.f32(cinfo + 0x2c).is_nan());
        assert_eq!(calls_to(&e, SHAPE_COPY_MEMBERS).len(), 3);
    }

    #[test]
    fn bhk_capsule_shape_create_clone_copies_the_members_under_the_clone_lock() {
        let mut e = logged_engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(COPY_MEMBERS_SLOT, |_, _| Ret::default());
        let this = shape(&mut e);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let clone = e.call(0x00ca_af40, &args![this, 0x3000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), CAPSULE_SHAPE_VTABLE);
        assert_eq!(
            calls_to(&e, COPY_MEMBERS_SLOT),
            vec![vec![this, clone, 0x3000]]
        );
        let expected = vec![
            CLONE_LOCK,
            OPERATOR_NEW,
            OBJECT_CONSTRUCT,
            COPY_MEMBERS_SLOT,
            CLONE_UNLOCK,
        ];
        assert_eq!(order_of(&e, &expected), expected);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // A failed allocation still copies, onto a null clone.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00ca_af40, &args![this, 0x3000u32]).u32(), 0);
        assert_eq!(calls_to(&e, COPY_MEMBERS_SLOT)[1], vec![this, 0, 0x3000]);
    }

    #[test]
    fn bhk_capsule_shape_create_cinfo_builds_the_cinfo_block_once() {
        let mut e = logged_engine();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(CONVEX_COPY_CINFO, |_, _| Ret::default());
        e.register(COPY_VECTOR, |_, _| Ret::default());
        e.set_global(DEFAULT_RADIUS, 0.25f32);
        let this = shape(&mut e);
        e.mem.set_u32(this + 8, 0x5000);
        let created = e.mem.alloc(4);
        e.mem.set_u8(created, 9);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let cinfo = e.call(0x00ca_b000, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x30]]
        );
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.u32(cinfo), 0);
        assert_eq!(e.mem.f32(cinfo + 4), 0.25);
        assert_eq!(load_vec(&e, cinfo + 0x10), [0.0; 4]);
        assert_eq!(load_vec(&e, cinfo + 0x20), [1.0; 4]);
        assert_eq!(calls_to(&e, CONVEX_COPY_CINFO), vec![vec![this, cinfo]]);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // The second call finds the Cinfo.
        assert_eq!(e.call(0x00ca_b000, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE).len(), 1);
        assert_eq!(calls_to(&e, CONVEX_COPY_CINFO).len(), 1);
        // Without the Havok capsule no copy is made; a failed allocation stores 0.
        let bare = shape(&mut e);
        e.call(0x00ca_b000, &args![bare, created]);
        assert_eq!(calls_to(&e, CONVEX_COPY_CINFO).len(), 1);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        let failing = shape(&mut e);
        e.mem.set_u32(failing + 8, 0x5000);
        assert_eq!(e.call(0x00ca_b000, &args![failing, created]).u32(), 0);
        assert_eq!(calls_to(&e, CONVEX_COPY_CINFO)[1], vec![failing, 0]);
    }

    #[test]
    fn bhk_capsule_shape_init_builds_the_havok_capsule() {
        let mut e = logged_engine();
        install_allocator(&mut e);
        e.register(HAVOK_CAPSULE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(SET_HK_OBJECT_SLOT, |_, _| Ret::default());
        let this = shape(&mut e);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00ca_b0e0, &args![this, 0x6000u32, 0x6010u32, 0.75f32]);
        let constructs = calls_to(&e, HAVOK_CAPSULE_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        let block = constructs[0][0];
        assert_eq!(constructs[0][1..], [0x6000, 0x6010, 0.75f32.to_bits()]);
        assert_eq!(e.mem.u16(block + 4), 0x40);
        assert_eq!(
            calls_to(&e, ALLOC_SLOT),
            vec![vec![ROUTER_OBJECT + 0x20, 0x40]]
        );
        assert_eq!(calls_to(&e, SET_HK_OBJECT_SLOT), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        let order = vec![
            HAVOK_CAPSULE_CONSTRUCT,
            SET_HK_OBJECT_SLOT,
            REMOVE_REFERENCE,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // A signalling NaN radius is passed quieted, as the FLD/FSTP pair leaves it.
        e.call(
            0x00ca_b0e0,
            &args![this, 1u32, 2u32, f32::from_bits(0x7f80_0001)],
        );
        assert_eq!(calls_to(&e, HAVOK_CAPSULE_CONSTRUCT)[1][3], 0x7fc0_0001);
    }

    #[test]
    fn bhk_capsule_shape_build_display_geometry_builds_the_display_capsule() {
        let mut e = logged_engine();
        install_allocator(&mut e);
        e.set_global(DEFAULT_RADIUS, 0.25f32);
        // Base copy: sets the radius, as the shape's own would.
        e.register(CONVEX_COPY_CINFO, |e, a| {
            e.mem.set_f32(a[1] + 4, 2.5);
            Ret::default()
        });
        // Vector copy: three lanes.
        e.register(COPY_VECTOR, |e, a| {
            copy_words(e, a[1], a[0], 3);
            Ret::default()
        });
        // The display capsule keeps copies of both vectors; its geometry is word 0x5555.
        e.register(DISPLAY_CAPSULE_CONSTRUCT, |e, a| {
            let first = e.mem.bytes(a[1], 16);
            let second = e.mem.bytes(a[2], 16);
            e.mem.write(a[0] + 0x10, &first);
            e.mem.write(a[0] + 0x20, &second);
            e.mem.set_u32(a[0], DISPLAY_VTABLE);
            e.mem.set_u32(a[0] + 8, 0x5555);
            a[0].into_ret()
        });
        put_vtable_at(&mut e, DISPLAY_VTABLE, &[(0x10, DISPLAY_SLOT)]);
        e.register(DISPLAY_SLOT, |_, _| Ret::default());
        e.register(BUILD_DISPLAY_SLOT, |_, _| Ret::default());
        let this = shape(&mut e);
        let capsule = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, capsule);
        set_vec(&mut e, capsule + 0x20, [1.0, 2.0, 3.0, 9.0]);
        set_vec(&mut e, capsule + 0x30, [4.0, 5.0, 6.0, 9.0]);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00ca_b1b0, &args![this, 0x7000u32, 0x7010u32]);
        let constructs = calls_to(&e, DISPLAY_CAPSULE_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        let block = constructs[0][0];
        assert_eq!(constructs[0][3..], [2.5f32.to_bits(), 6, 1]);
        assert_eq!(e.mem.u16(block + 4), 0x90);
        assert_eq!(
            calls_to(&e, ALLOC_SLOT),
            vec![vec![ROUTER_OBJECT + 0x20, 0x90]]
        );
        // The end points: the capsule's three lanes and the Cinfo's default fourth lanes.
        assert_eq!(load_vec(&e, block + 0x10), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x20), [4.0, 5.0, 6.0, 1.0]);
        assert_eq!(calls_to(&e, DISPLAY_SLOT), vec![vec![block]]);
        assert_eq!(
            calls_to(&e, BUILD_DISPLAY_SLOT),
            vec![vec![this, 0x7000, 0x5555, 0x7010, DISPLAY_NAME]]
        );
        let order = vec![
            CONVEX_COPY_CINFO,
            COPY_VECTOR,
            COPY_VECTOR,
            DISPLAY_CAPSULE_CONSTRUCT,
            DISPLAY_SLOT,
            BUILD_DISPLAY_SLOT,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // Without a display geometry nothing is built; the scope is still restored.
        e.register(DISPLAY_CAPSULE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], DISPLAY_VTABLE);
            a[0].into_ret()
        });
        e.call(0x00ca_b1b0, &args![this, 0x7000u32, 0x7010u32]);
        assert_eq!(calls_to(&e, BUILD_DISPLAY_SLOT).len(), 1);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    /// The closest-point double answers (10, 0, 1, 1) and a square root double answers the
    /// root of its argument.
    fn closest_point_engine() -> (Engine, u32, u32, u32) {
        let mut e = logged_engine();
        e.register(CLOSEST_POINT_ON_SEGMENT, |e, a| {
            set_vec(e, a[3], [10.0, 0.0, 1.0, 1.0]);
            Ret::default()
        });
        e.register(X87_SQRT, |_, a| {
            let value = f64::from_bits((u64::from(a[1]) << 32) | u64::from(a[0]));
            value.sqrt().into_ret()
        });
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        let this = shape(&mut e);
        let capsule = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, capsule);
        e.mem.set_f32(capsule + 0x10, 0.5);
        set_vec(&mut e, capsule + 0x20, [0.0, 0.0, 0.0, 0.0]);
        set_vec(&mut e, capsule + 0x30, [0.0, 0.0, 2.0, 0.0]);
        // The transform: identity rows, translation (10, 0, 0, 1).
        let transform = e.mem.alloc(0x40);
        set_vec(&mut e, transform, [1.0, 0.0, 0.0, 0.0]);
        set_vec(&mut e, transform + 0x10, [0.0, 1.0, 0.0, 0.0]);
        set_vec(&mut e, transform + 0x20, [0.0, 0.0, 1.0, 0.0]);
        set_vec(&mut e, transform + 0x30, [10.0, 0.0, 0.0, 1.0]);
        let contact = e.mem.alloc(0x20);
        (e, this, transform, contact)
    }

    #[test]
    fn bhk_capsule_shape_get_closest_points_writes_the_contact_of_a_touching_sphere() {
        let (mut e, this, transform, contact) = closest_point_engine();
        let sphere = e.mem.alloc(0x10);
        set_vec(&mut e, sphere, [11.0, 0.0, 1.0, 0.75]);
        set_vec(&mut e, contact + 0x10, [7.0, 7.0, 7.0, 7.0]);
        let hit = e
            .call(0x00ca_b3c0, &args![this, sphere, transform, contact])
            .bool();
        assert!(hit);
        // The segment is transformed before the closest point is taken.
        let calls = calls_to(&e, CLOSEST_POINT_ON_SEGMENT);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], sphere);
        // The distance is 1 and the radius sum 1.25: normal (1, 0, 0), penetration -0.25 in
        // its fourth lane, position centre + normal * (0.5 - 1).
        let normal = load_vec(&e, contact + 0x10);
        close(normal[0], 1.0);
        close(normal[1], 0.0);
        close(normal[2], 0.0);
        assert_eq!(normal[3], -0.25);
        let position = load_vec(&e, contact);
        close(position[0], 10.5);
        close(position[1], 0.0);
        close(position[2], 1.0);
        // The fourth lane used the normal's old fourth lane (7).
        close(position[3], -0.5 * 7.0 + 0.75);
        assert_eq!(calls_to(&e, X87_SQRT).len(), 1);
    }

    #[test]
    fn bhk_capsule_shape_get_closest_points_answers_false_for_a_distant_sphere() {
        let (mut e, this, transform, contact) = closest_point_engine();
        let sphere = e.mem.alloc(0x10);
        set_vec(&mut e, sphere, [13.0, 0.0, 1.0, 0.1]);
        set_vec(&mut e, contact, [7.0, 7.0, 7.0, 7.0]);
        set_vec(&mut e, contact + 0x10, [7.0, 7.0, 7.0, 7.0]);
        assert!(!e
            .call(0x00ca_b3c0, &args![this, sphere, transform, contact])
            .bool());
        assert_eq!(load_vec(&e, contact), [7.0; 4]);
        assert_eq!(load_vec(&e, contact + 0x10), [7.0; 4]);
        // The radius squared must be above the distance squared: touching exactly is a miss.
        set_vec(&mut e, sphere, [11.0, 0.0, 1.0, 0.5]);
        assert!(!e
            .call(0x00ca_b3c0, &args![this, sphere, transform, contact])
            .bool());
        // A NaN centre counts as a hit.
        set_vec(&mut e, sphere, [f32::NAN, 0.0, 1.0, 0.5]);
        assert!(e
            .call(0x00ca_b3c0, &args![this, sphere, transform, contact])
            .bool());
    }

    #[test]
    fn bhk_capsule_shape_get_closest_points_builds_a_perpendicular_for_a_centre_on_the_segment() {
        let (mut e, this, transform, contact) = closest_point_engine();
        let sphere = e.mem.alloc(0x10);
        set_vec(&mut e, sphere, [10.0, 0.0, 1.0, 0.0]);
        set_vec(&mut e, contact + 0x10, [7.0, 7.0, 7.0, 7.0]);
        assert!(e
            .call(0x00ca_b3c0, &args![this, sphere, transform, contact])
            .bool());
        // The segment runs along z (length 2): the smallest components are x and y, so the
        // perpendicular is (0, 2, -0) normalised.
        let normal = load_vec(&e, contact + 0x10);
        close(normal[0], 0.0);
        close(normal[1], 1.0);
        close(normal[2], 0.0);
        assert!(normal[2].is_sign_negative());
        // The distance is 0: position centre + normal * radius, penetration -radius sum.
        let position = load_vec(&e, contact);
        close(position[0], 10.0);
        close(position[1], 0.5);
        close(position[2], 1.0);
        assert_eq!(normal[3], -0.5);
        assert_eq!(calls_to(&e, X87_SQRT).len(), 0);
    }

    #[test]
    fn bhk_capsule_shape_constructor_with_points_scales_the_points_and_initialises() {
        let mut e = logged_engine();
        install_allocator(&mut e);
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(SET_HK_OBJECT_SLOT, |_, _| Ret::default());
        // The Havok capsule keeps the two vectors and the radius it is given.
        e.register(HAVOK_CAPSULE_CONSTRUCT, |e, a| {
            let first = e.mem.bytes(a[1], 16);
            let second = e.mem.bytes(a[2], 16);
            e.mem.write(a[0] + 0x20, &first);
            e.mem.write(a[0] + 0x30, &second);
            e.mem.set_u32(a[0] + 0x10, a[3]);
            a[0].into_ret()
        });
        e.set_global(HAVOK_SCALE, 0.5f32);
        let this = shape(&mut e);
        let point_a = e.mem.alloc(0x10);
        let point_b = e.mem.alloc(0x10);
        set_vec(&mut e, point_a, [2.0, 4.0, 6.0, 99.0]);
        set_vec(&mut e, point_b, [8.0, 10.0, 12.0, 99.0]);
        let result = e
            .call(0x00ca_b710, &args![this, point_a, point_b, 4.0f32])
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), CAPSULE_SHAPE_VTABLE);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![this]]);
        let constructs = calls_to(&e, HAVOK_CAPSULE_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        let block = constructs[0][0];
        assert_eq!(load_vec(&e, block + 0x20), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(load_vec(&e, block + 0x30), [4.0, 5.0, 6.0, 0.0]);
        assert_eq!(e.mem.f32(block + 0x10), 2.0);
        assert_eq!(calls_to(&e, SET_HK_OBJECT_SLOT), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
    }

    #[test]
    fn bhk_capsule_shape_init_from_cinfo_forwards_the_end_points_and_posts_the_init() {
        let mut e = logged_engine();
        install_allocator(&mut e);
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(SET_HK_OBJECT_SLOT, |_, _| Ret::default());
        e.register(POST_INIT_SLOT, |_, _| Ret::default());
        e.register(HAVOK_CAPSULE_CONSTRUCT, |e, a| {
            let first = e.mem.bytes(a[1], 16);
            let second = e.mem.bytes(a[2], 16);
            e.mem.write(a[0] + 0x20, &first);
            e.mem.write(a[0] + 0x30, &second);
            e.mem.set_u32(a[0] + 0x10, a[3]);
            a[0].into_ret()
        });
        let this = shape(&mut e);
        let cinfo = e.mem.alloc(0x30);
        e.mem.set_f32(cinfo + 4, 1.5);
        set_vec(&mut e, cinfo + 0x10, [1.0, 2.0, 3.0, 4.0]);
        set_vec(&mut e, cinfo + 0x20, [5.0, 6.0, 7.0, 8.0]);
        e.call(0x00ca_b820, &args![this, cinfo]);
        let constructs = calls_to(&e, HAVOK_CAPSULE_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        let block = constructs[0][0];
        assert_eq!(load_vec(&e, block + 0x20), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, block + 0x30), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.f32(block + 0x10), 1.5);
        assert_eq!(calls_to(&e, SET_HK_OBJECT_SLOT), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, cinfo]]);
        let order = vec![HAVOK_CAPSULE_CONSTRUCT, SET_HK_OBJECT_SLOT, POST_INIT_SLOT];
        assert_eq!(order_of(&e, &order), order);
    }

    #[test]
    fn bhk_capsule_shape_get_viewer_strings_stores_the_class_name() {
        let mut e = logged_engine();
        e.register(CONVEX_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        e.mem.set_u32(CAPSULE_SHAPE_RTTI, 0x4444);
        let this = e.mem.alloc(0x14);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00ca_b8c0, &args![this, array]);
        assert_eq!(
            calls_to(&e, CONVEX_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4444]]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_capsule_shape_load_binary_swaps_the_end_points_unless_the_stream_is_big_endian() {
        let mut e = logged_engine();
        e.register(CONVEX_LOAD_BINARY, |_, _| Ret::default());
        let this = shape(&mut e);
        let cinfo = e.mem.alloc(0x30);
        e.register_double(CREATE_CINFO_SLOT, move |_, _| cinfo.into_ret());
        for index in 0..12u32 {
            e.mem.set_u32(cinfo + 4 * index, 0x0102_0304 + index);
        }
        let stream = e.mem.alloc(0x200);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00cc_d330, &args![this, stream]);
        assert_eq!(calls_to(&e, CONVEX_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x0102_0308);
        assert_eq!(calls_to(&e, CREATE_CINFO_SLOT).len(), 0);
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cc_d330, &args![this, stream]);
        for index in 0..12u32 {
            let original = 0x0102_0304 + index;
            let expected = if (4..12).contains(&index) {
                original.swap_bytes()
            } else {
                original
            };
            assert_eq!(e.mem.u32(cinfo + 4 * index), expected, "word {index}");
        }
        assert_eq!(calls_to(&e, CREATE_CINFO_SLOT).len(), 1);
    }
}
