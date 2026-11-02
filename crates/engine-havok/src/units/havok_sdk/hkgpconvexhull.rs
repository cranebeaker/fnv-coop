//! `Havok SDK/hkgpconvexhull.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, maxps, minps, profile_record, rcp_estimate,
    shufps, store_vec, sum3, unpckhps, zip_with, Vec4, PROFILE_END, SIGN_MASK, ZERO_F32,
};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d7f060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d7_6940`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d7f060(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d7_6940, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `FTOL` (`_ftol2_sse`, `00ec62c0`): truncates the `double` in `ST0` (passed as the leading
/// two words).
const FTOL: u32 = 0x00ec_62c0;
/// The `hkArray` reserve (`00c908c0`, cdecl: `(allocator, array, capacity, element size)`);
/// the allocator is the memory router's (`00c85750`) field `+0x10`.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The failed-assertion report: the string builder (`00cb1600`: `this` = builder, `(buffer,
/// size)`; `00cc01c0` appends a text, `00cc02d0` appends a number, both answer `this`;
/// `00cc00e0` destroys it) and the sink at `01268360` whose slot `0xc` takes `(3, id, text,
/// file, line)` and answers non-zero to ask for a debugger break (`INT3`).
const REPORT_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const REPORT_BUILDER_TEXT: u32 = 0x00cc_01c0;
const REPORT_BUILDER_NUMBER: u32 = 0x00cc_02d0;
const REPORT_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
const REPORT_SINK: u32 = 0x0126_8360;
/// `".\GeometryProcessing\ConvexHull\hkgpConvexHull.cpp"`, the closing `")"` and the two
/// messages of the accessors' assertions.
const REPORT_FILE: u32 = 0x010d_3540;
const REPORT_CLOSE: u32 = 0x0103_d078;
const MESSAGE_NO_INDEX: u32 = 0x010d_3574;
const MESSAGE_NO_MASS_PROPERTIES: u32 = 0x010d_358c;

layout! {
    /// `hkgpConvexHull` (the Xbox PDB has no type for it; the field roles are read from the code
    /// and the names are unconfirmed): a reference-counted object whose `+8` is its
    /// implementation.
    pub struct HkgpConvexHull: 0x14 {
        /// `hkReferencedObject` memory size and flags, then the reference count the
        /// constructor sets to 1.
        0x04 memory_size: u16,
        0x06 reference_count: u16,
        /// The `hkgpConvexHullImpl` (`0x1c0` bytes).
        0x08 implementation: Ptr<HkgpConvexHullImpl>,
        /// A word the constructor clears and `clone` copies.
        0x0c extra_word: u32,
        /// An owned object (virtual slot 0 deletes it with 1, slot 4 clones it).
        0x10 owned_object: Ptr,
    }

    /// `hkgpConvexHullImpl` (names unconfirmed, roles read from the code): the vertex list, the
    /// planes, the quantisation grid and the mass properties.
    pub struct HkgpConvexHullImpl: 0x1c0 {
        /// The `float` `buildIndices` stores before it runs.
        0x04 index_argument: f32,
        /// Head of the vertex list (nodes: `+0` next, `+0x10` position, `+0x20..+0x28` the
        /// three grid coordinates) and the number of vertices.
        0x24 vertex_list: Ptr,
        0x28 vertex_count: i32,
        /// Plane equations (16 bytes each) and their number.
        0x54 planes: Ptr,
        0x58 plane_count: i32,
        /// The dimension count (0 to 3).
        0x1a8 dimensions: i32,
        /// The value printed by the accessors' assertions.
        0x1b0 report_value: u32,
        /// Mass properties are built / planes and indices are built.
        0x1b4 mass_properties_valid: bool,
        0x1b5 indices_valid: bool,
    }
}

/// `FLD dword`: the `float` widened to the x87's `double`; a signalling NaN comes out quiet.
fn fld(value: f32) -> f64 {
    f64::from(x87_quiet(value))
}

fn splat(value: f32) -> Vec4 {
    [value; 4]
}

fn add(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

fn sub(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

fn mul(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

/// `FLD dword [from]; FSTP dword [to]`: a copy that quiets a signalling NaN.
fn copy_through_x87(e: &mut Engine, to: u32, from: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// The failed-assertion report of the accessors: a builder over a `0x200`-byte buffer holding
/// `message`, the number at `number_address` and `")"`, sent to the sink with `(3, id, text,
/// file, line)`. A non-zero answer is the `INT3` at `break_address`.
fn report_with_number(
    e: &mut Engine,
    message: u32,
    number_address: u32,
    id: u32,
    line: u32,
    break_address: u32,
) {
    e.with_stack(0x20, |e, builder| {
        e.with_stack(0x200, |e, buffer| {
            let (builder, buffer) = (builder.addr(), buffer.addr());
            e.call(REPORT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
            let number = e.mem.u32(number_address);
            let text = e.call(REPORT_BUILDER_TEXT, &args![builder, message]).u32();
            let text = e.call(REPORT_BUILDER_NUMBER, &args![text, number]).u32();
            e.call(REPORT_BUILDER_TEXT, &args![text, REPORT_CLOSE]);
            let sink = e.global::<u32>(REPORT_SINK);
            let answer = e
                .vcall(sink, 0xc, &args![3u32, id, buffer, REPORT_FILE, line])
                .u32();
            if answer != 0 {
                panic!("INT3 at {break_address:08x} (assertion break)");
            }
            e.call(REPORT_BUILDER_DESTRUCT, &args![builder]);
        });
    });
}

/// The guard of the plane accessors: the planes exist once `indices_valid` is set, else the
/// "No index available" report (id `79f9d886`, line `0x148`). Answers the implementation read
/// from the hull afterwards, as the exe re-reads it.
fn guard_indices(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    break_address: u32,
) -> Ptr<HkgpConvexHullImpl> {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    assert_indices(e, implementation, break_address);
    e.get(this, HkgpConvexHull::implementation)
}

/// The check of `guard_indices` on the implementation itself.
fn assert_indices(e: &mut Engine, implementation: Ptr<HkgpConvexHullImpl>, break_address: u32) {
    if !e.get(implementation, HkgpConvexHullImpl::indices_valid) {
        report_with_number(
            e,
            MESSAGE_NO_INDEX,
            implementation.addr() + 0x1b0,
            0x79f9_d886,
            0x148,
            break_address,
        );
    }
}

/// The guard of the mass-property accessors: the "No mass properties available" report (id
/// `79f9d887`, line `0x14e`) unless `mass_properties_valid` is set.
fn guard_mass_properties(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    break_address: u32,
) -> Ptr<HkgpConvexHullImpl> {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    if !e.get(implementation, HkgpConvexHullImpl::mass_properties_valid) {
        report_with_number(
            e,
            MESSAGE_NO_MASS_PROPERTIES,
            implementation.addr() + 0x1b0,
            0x79f9_d887,
            0x14e,
            break_address,
        );
    }
    e.get(this, HkgpConvexHull::implementation)
}

/// The position of a vertex node on the hull's grid (`CVTSI2SS` of the three coordinates at
/// `+0x20`, the float at `01011d78` as the fourth, `UNPCKLPS` into one vector, times the scale
/// at `+0x110` plus the offset at `+0xf0` of the implementation).
fn grid_position(e: &Engine, implementation: u32, vertex: u32) -> Vec4 {
    let coordinates = [
        e.mem.i32(vertex + 0x20) as f32,
        e.mem.i32(vertex + 0x24) as f32,
        e.mem.i32(vertex + 0x28) as f32,
        e.mem.f32(ZERO_F32),
    ];
    let scale = load_vec(e, implementation + 0x110);
    let offset = load_vec(e, implementation + 0xf0);
    add(mul(coordinates, scale), offset)
}

/// The growth of an `hkArray` the exe inlines before appending `count` elements: when the
/// capacity (low 30 bits of `+8`) is below `size + count`, reserve twice the capacity or the
/// needed size, whichever is greater (`00c908c0`), then add `count` to the size. Answers the
/// old size. The allocator is the router's `+0x10`.
fn array_append(e: &mut Engine, array: u32, count: i32, element_size: u32) -> i32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let size = e.mem.i32(array + 4);
    let needed = count.wrapping_add(size);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < needed {
        let doubled = capacity.wrapping_add(capacity);
        let target = if needed >= doubled { needed } else { doubled };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, target, element_size],
        );
    }
    let size_now = e.mem.i32(array + 4);
    e.mem.set_i32(array + 4, size_now.wrapping_add(count));
    size
}

// Translated from 00d74c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl, `(vector, scale, out)`): the 4x4 matrix `out` (row
/// stride `0x10`) of `I + (scale - 1) * v * v^T` on the first three lanes of `vector`, with
/// the fourth column `(scale - 1) * v * v3` in the last row's lanes `0xc..0xe` and `1.0` at
/// `0xf`. `(scale - 1)` is rounded to a `float` first; the lane products come from `MULPS`
/// and the sums and products that fill the matrix go through the x87, rounded to `float` at
/// each store (so `out` aliasing `vector` gives the exe's result).
pub fn fn_00d74c70(e: &mut Engine, vector: Ptr, scale: f32, out: Ptr) {
    let source = vector.addr();
    let o = out.addr();
    let v = load_vec(e, source);
    // The x87 part keeps the x87 choice of NaN (`x87_add`, `x87_mul`); `FMUL
    // [vector + 0xc]` takes the lane as a memory operand (a signalling NaN stays
    // one there), `FLD` quiets it. The `MULPS` keep the SSE rules.
    let t = f32::from_bits(x87_store32(x87_sub(fld(scale), 1.0)));
    let tv = crate::sse::mulps(splat(t), v);
    let tvv = crate::sse::mulps(splat(t), crate::sse::mulps(v, v));
    let lane = |e: &Engine, index: u32| fld(e.mem.f32(source + 4 * index));
    let memory_lane = |e: &Engine, index: u32| x87_m32(e.mem.u32(source + 4 * index));
    let store = |e: &mut Engine, at: u32, value: f64| e.mem.set_u32(at, x87_store32(value));
    store(e, o, x87_add(fld(tvv[0]), 1.0));
    let product = x87_mul(lane(e, 0), fld(tv[1]));
    store(e, o + 0x10, product);
    let product = x87_mul(lane(e, 0), fld(tv[2]));
    store(e, o + 0x20, product);
    let product = x87_mul(fld(tv[0]), memory_lane(e, 3));
    store(e, o + 0x30, product);
    copy_through_x87(e, o + 4, o + 0x10);
    store(e, o + 0x14, x87_add(fld(tvv[1]), 1.0));
    let product = x87_mul(lane(e, 1), fld(tv[2]));
    store(e, o + 0x24, product);
    let product = x87_mul(fld(tv[1]), lane(e, 3));
    store(e, o + 0x34, product);
    copy_through_x87(e, o + 8, o + 0x20);
    copy_through_x87(e, o + 0x18, o + 0x24);
    store(e, o + 0x28, x87_add(1.0, fld(tvv[2])));
    let product = x87_mul(fld(tv[2]), memory_lane(e, 3));
    store(e, o + 0x38, product);
    e.mem.set_u32(o + 0xc, 0);
    e.mem.set_u32(o + 0x1c, 0);
    e.mem.set_u32(o + 0x2c, 0);
    e.mem.set_f32(o + 0x3c, 1.0);
}

/// `float` the build configuration's tolerance starts from (`0106ea1c`).
const BUILD_CONFIG_DEFAULT_FLOAT: u32 = 0x0106_ea1c;

// Translated from 00d74d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::BuildConfig::BuildConfig` (Xbox PDB): `this` = the configuration. The
/// `float` at `+4` is the one at `0106ea1c` (through the x87); the bytes at `+0`, `+8`, `+0xa`,
/// `+0xc` and `+0xd` are cleared, those at `+9` and `+0xb` set to 1. Returns `this`.
pub fn hkgp_convex_hull_build_config_build_config(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    copy_through_x87(e, at + 4, BUILD_CONFIG_DEFAULT_FLOAT);
    e.mem.set_u8(at, 0);
    e.mem.set_u8(at + 8, 0);
    e.mem.set_u8(at + 9, 1);
    e.mem.set_u8(at + 0xa, 0);
    e.mem.set_u8(at + 0xb, 1);
    e.mem.set_u8(at + 0xc, 0);
    e.mem.set_u8(at + 0xd, 0);
    this
}

/// `float` a second, smaller configuration starts from (`0104da4c`).
const SECOND_CONFIG_DEFAULT_FLOAT: u32 = 0x0104_da4c;

// Translated from 00d74d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an 8-byte configuration (no name in the engine map, class unconfirmed):
/// the `float` at `0104da4c` (through the x87) into `+4` and `1` into `+0`. Returns `this`.
pub fn fn_00d74d90(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    copy_through_x87(e, at + 4, SECOND_CONFIG_DEFAULT_FLOAT);
    e.mem.set_u32(at, 1);
    this
}

// Translated from 00d74dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the owned object of the hull (`this + 0x10`): when it is not null its virtual slot
/// 0 is called with `1`; the field is then cleared.
pub fn fn_00d74dd0(e: &mut Engine, this: Ptr<HkgpConvexHull>) {
    let owned = e.get(this, HkgpConvexHull::owned_object);
    if !owned.is_null() {
        e.vcall(owned.addr(), 0, &args![1u32]);
    }
    e.set(this, HkgpConvexHull::owned_object, Ptr::NULL);
}

// Translated from 00d74e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hull's implementation pointer (`this + 8`).
pub fn fn_00d74e00(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> Ptr<HkgpConvexHullImpl> {
    e.get(this, HkgpConvexHull::implementation)
}

// Translated from 00d74e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::getDimensions` (Xbox PDB): the dimension count of the implementation.
pub fn hkgp_convex_hull_get_dimensions(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> i32 {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    e.get(implementation, HkgpConvexHullImpl::dimensions)
}

// Translated from 00d74e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four words at `+0xe0` of the implementation (a vector, unconfirmed role) to
/// `out` as integers, bit for bit. Returns `out`.
pub fn fn_00d74e50(e: &mut Engine, this: Ptr<HkgpConvexHull>, out: Ptr) -> Ptr {
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    for word in 0..4 {
        let value = e.mem.u32(implementation + 0xe0 + 4 * word);
        e.mem.set_u32(out.addr() + 4 * word, value);
    }
    out
}

// Translated from 00d74e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The position of a vertex node on the hull's grid, written to the vector `out`: the node's
/// three integer coordinates (`+0x20`) converted to `float`, the float at `01011d78` as the
/// fourth lane, times the vector at `+0x110` plus the vector at `+0xf0` of the
/// implementation. Returns `out`.
pub fn fn_00d74e80(e: &mut Engine, this: Ptr<HkgpConvexHull>, out: Ptr, vertex: Ptr) -> Ptr {
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    let position = grid_position(e, implementation, vertex.addr());
    store_vec(e, out.addr(), position);
    out
}

// Translated from 00d74ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The position of a vertex node: its address plus `0x10`.
pub fn fn_00d74ed0(_e: &mut Engine, vertex: Ptr) -> Ptr {
    Ptr::new(vertex.addr().wrapping_add(0x10))
}

// Translated from 00d74f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of plane equations (`+0x58` of the implementation; the role is read from
/// `fetchPlanes`, which sizes its array with it). Asserts that the indices are built (`+0x1b5`):
/// otherwise the "No index available (<`+0x1b0`>)" report (`.\GeometryProcessing\ConvexHull\
/// hkgpConvexHull.cpp`, line `0x148`), whose non-zero answer is the `INT3` at `00d74f88`.
pub fn fn_00d74f10(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> i32 {
    let implementation = guard_indices(e, this, 0x00d7_4f88);
    e.get(implementation, HkgpConvexHullImpl::plane_count)
}

// Translated from 00d74fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plane equation number `index` (16 bytes at `index * 0x10` from the array at `+0x54` of the
/// implementation), under the same assertion as `00d74f10` (the `INT3` here is at
/// `00d75028`).
pub fn fn_00d74fb0(e: &mut Engine, this: Ptr<HkgpConvexHull>, index: i32) -> u32 {
    let implementation = guard_indices(e, this, 0x00d7_5028);
    let planes = e.get(implementation, HkgpConvexHullImpl::planes).addr();
    (index as u32).wrapping_mul(0x10).wrapping_add(planes)
}

// Translated from 00d75050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of vertices (`+0x28` of the implementation).
pub fn fn_00d75050(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> i32 {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    e.get(implementation, HkgpConvexHullImpl::vertex_count)
}

// Translated from 00d75060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The head of the vertex list (`+0x24` of the implementation).
pub fn fn_00d75060(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> Ptr {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    e.get(implementation, HkgpConvexHullImpl::vertex_list)
}

// Translated from 00d75120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The next node of a list node (its first word).
pub fn fn_00d75120(e: &mut Engine, node: Ptr) -> u32 {
    e.mem.u32(node.addr())
}

// Translated from 00d75130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x40` of the implementation (role unconfirmed).
pub fn fn_00d75130(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> u32 {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    e.mem.u32(implementation.addr() + 0x40)
}

// Translated from 00d75140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x3c` of the implementation (role unconfirmed).
pub fn fn_00d75140(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> u32 {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    e.mem.u32(implementation.addr() + 0x3c)
}

// Translated from 00d75200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first word of a node (same body as `00d75120`).
pub fn fn_00d75200(e: &mut Engine, node: Ptr) -> u32 {
    e.mem.u32(node.addr())
}

// Translated from 00d752c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Word number `index` after the first two words of `node`: `*(node + 8 + index * 4)` (the
/// vertex pointers of a triangle, unconfirmed).
pub fn fn_00d752c0(e: &mut Engine, node: Ptr, index: u32) -> u32 {
    e.mem.u32(
        node.addr()
            .wrapping_add(8)
            .wrapping_add(index.wrapping_mul(4)),
    )
}

// Translated from 00d752d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tail call: `00d7a180` with `this` replaced by the hull's implementation (it takes no stack
/// words: its `RET` is plain); answers what it answers.
pub fn fn_00d752d0(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> Ptr {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    fn_00d7a180(e, implementation)
}

// Translated from 00d752e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::getVolume` (Xbox PDB): the `float` at `+0x1a0` of the implementation,
/// through the x87. Asserts that the mass properties are built (`+0x1b4`): otherwise the "No
/// mass properties available (<`+0x1b0`>)" report (line `0x14e`, `INT3` at `00d75358`).
pub fn hkgp_convex_hull_get_volume(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> f32 {
    let implementation = guard_mass_properties(e, this, 0x00d7_5358);
    x87_quiet(e.mem.f32(implementation.addr() + 0x1a0))
}

// Translated from 00d75390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::getSurfaceArea` (Xbox PDB): the `float` at `+0x1a4` of the implementation,
/// under the assertion of `getVolume` (`INT3` at `00d75408`).
pub fn hkgp_convex_hull_get_surface_area(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> f32 {
    let implementation = guard_mass_properties(e, this, 0x00d7_5408);
    x87_quiet(e.mem.f32(implementation.addr() + 0x1a4))
}

/// `pow`'s exponent and the factor of `getSphericity` (two `double`s).
const SPHERICITY_EXPONENT: u32 = 0x010d_35e8;
const SPHERICITY_FACTOR: u32 = 0x010d_35e0;
/// The CRT `pow` (`00ec9b00`): `(x, y)` as two `double`s.
const POW: u32 = 0x00ec_9b00;

// Translated from 00d75440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::getSphericity` (Xbox PDB): `pow(volume, e) * factor / surfaceArea` with
/// the `double`s at `010d35e8` (`e`) and `010d35e0` (`factor`); the power is rounded to a
/// `float` before the product and the quotient is rounded to a `float`.
pub fn hkgp_convex_hull_get_sphericity(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> f32 {
    let volume = hkgp_convex_hull_get_volume(e, this);
    let exponent: f64 = e.global(SPHERICITY_EXPONENT);
    let power = e.call(POW, &args![fld(volume), exponent]).f64() as f32;
    let factor: f64 = e.global(SPHERICITY_FACTOR);
    let numerator = fld(power) * factor;
    let area = hkgp_convex_hull_get_surface_area(e, this);
    x87_quiet((numerator / fld(area)) as f32)
}

// Translated from 00d75490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the mass-property block of the implementation: the four vectors at `+0x120` into
/// `matrix` and the vector at `+0x160` into `vector`, 16 bytes at a time. Asserts that the mass
/// properties are built (as `getVolume`; `INT3` at `00d7550e`).
pub fn fn_00d75490(e: &mut Engine, this: Ptr<HkgpConvexHull>, matrix: Ptr, vector: Ptr) {
    guard_mass_properties(e, this, 0x00d7_550e);
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    for row in 0..4 {
        copy_vec(
            e,
            implementation + 0x120 + 0x10 * row,
            matrix.addr() + 0x10 * row,
        );
    }
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    copy_vec(e, implementation + 0x160, vector.addr());
}

// Translated from 00d75560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the block at `+0x170` of the implementation (role unconfirmed; the
/// mass-property assertion applies, `INT3` at `00d755d8`).
pub fn fn_00d75560(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> u32 {
    guard_mass_properties(e, this, 0x00d7_55d8);
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    implementation.wrapping_add(0x170)
}

// Translated from 00d75600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the block at `+0x150` of the implementation (role unconfirmed; the
/// mass-property assertion applies, `INT3` at `00d75678`).
pub fn fn_00d75600(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> u32 {
    guard_mass_properties(e, this, 0x00d7_5678);
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    implementation.wrapping_add(0x150)
}

/// The sign-clearing mask (four words `0x7fffffff`).
const ABS_MASK: u32 = 0x0101_7d10;
/// Lane 0 of this vector (`010c7160`) is the `0.0` the plane offsets are subtracted from.
const ZERO_VECTOR: u32 = 0x010c_7160;
/// Four `1.0` (`010c7170`).
const ONES_VECTOR: u32 = 0x010c_7170;

// Translated from 00d756a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Fetches the mass-property block (`00d75490` into a matrix of
/// four vectors `m` and a vector `v`), takes the reciprocals of `(v0, v1, v2, 1)` (`RCPPS`,
/// the host's estimate, refined once as `(r + r) - (r * x) * r`) in absolute value, picks the
/// index of the largest of the first three (ties and NaN decided by the x87 compares of the
/// exe: lane 1 must be greater than lane 0 to beat it, lane 2 greater than the best so far),
/// copies row `m[index]` to `out` and sets its fourth lane to `0.0 - dot(out, m[3])` (summed
/// as `z + (y + x)`). The mass-property assertion applies (`INT3` at `00d7571e`).
pub fn fn_00d756a0(e: &mut Engine, this: Ptr<HkgpConvexHull>, out: Ptr) {
    guard_mass_properties(e, this, 0x00d7_571e);
    e.with_stack(0x40, |e, matrix| {
        e.with_stack(0x10, |e, vector| {
            fn_00d75490(e, this, matrix, vector);
            let v = load_vec(e, vector.addr());
            let ones = load_vec(e, ONES_VECTOR);
            let mixed = shufps(v, unpckhps(v, ones), 0xc4);
            let estimate: Vec4 = std::array::from_fn(|lane| rcp_estimate(mixed[lane]));
            let refined = sub(add(estimate, estimate), mul(mul(estimate, mixed), estimate));
            let mask = load_bits(e, ABS_MASK);
            let absolute: Vec4 =
                std::array::from_fn(|lane| f32::from_bits(refined[lane].to_bits() & mask[lane]));
            store_vec(e, vector.addr(), absolute);
            let a = [fld(absolute[0]), fld(absolute[1]), fld(absolute[2])];
            let index = if a[1] > a[0] {
                if a[2] > a[1] {
                    2
                } else {
                    1
                }
            } else if a[2] > a[0] {
                2
            } else {
                0
            };
            let row3 = load_vec(e, matrix.addr() + 0x30);
            for word in 0..4 {
                let value = e.mem.u32(matrix.addr() + 0x10 * index + 4 * word);
                e.mem.set_u32(out.addr() + 4 * word, value);
            }
            let products = mul(load_vec(e, out.addr()), row3);
            let zero = e.mem.f32(ZERO_VECTOR);
            let offset = zero - sum3(products);
            e.mem
                .set_u32(out.addr() + 0xc, x87_quiet_bits(offset.to_bits()));
        });
    });
}

// Translated from 00d75850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the implementation (`hkgpConvexHullImpl::reset` with `0`, `00d7ac40`) and deletes the
/// owned object as `00d74dd0` does.
pub fn fn_00d75850(e: &mut Engine, this: Ptr<HkgpConvexHull>) {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    hkgp_convex_hull_impl_reset(e, implementation, 0);
    fn_00d74dd0(e, this);
}

/// The `float` the plane search starts its maximum from (`0104e148`, `-FLT_MAX`).
const NEGATIVE_FLOAT_MAX: u32 = 0x0104_e148;

// Translated from 00d75880 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The largest signed distance of the point `point` (a vector,
/// `+0` .. `+8` used) from the planes: the maximum over the plane equations (`00d74fb0`, count
/// from `00d74f10`, evaluated again on every pass) of `dot + w` with `dot` summed as
/// `z + (y + x)`. The running maximum starts at the `float` at `0104e148` and is replaced
/// unless the new value is smaller (a NaN replaces it, as the x87 flag test reads). The count
/// and plane assertions apply.
pub fn fn_00d75880(e: &mut Engine, this: Ptr<HkgpConvexHull>, point: Ptr) -> f32 {
    let mut best = x87_quiet(e.global::<f32>(NEGATIVE_FLOAT_MAX));
    let mut index = 0;
    if fn_00d74f10(e, this) > 0 {
        loop {
            let plane = fn_00d74fb0(e, this, index);
            let products = mul(load_vec(e, point.addr()), load_vec(e, plane));
            let distance = x87_quiet(sum3(products) + e.mem.f32(plane + 0xc));
            if !(fld(distance) < fld(best)) {
                best = distance;
            }
            index += 1;
            if index >= fn_00d74f10(e, this) {
                break;
            }
        }
    }
    x87_quiet(best)
}

/// The scale and centre `getBox` blends with: the double `0.5` at `01011588` and the float
/// `0.5` at `01016248`.
const HALF_DOUBLE: u32 = 0x0101_1588;
const HALF_FLOAT: u32 = 0x0101_6248;

// Translated from 00d75920 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The bounding box of the hull's vertices, written to `out` as
/// two vectors (minimum, maximum; 32 bytes). Box number `index` (`0`: the vertex positions,
/// otherwise the grid positions of `00d74e80`) is cached at `+0x90 + index * 0x20` of the
/// implementation; when its maximum is `<=` its minimum in one of the first three lanes (the
/// cache is empty) and the hull has vertices it is rebuilt by `MINPS`/`MAXPS` over the vertex
/// list. With `scale == 1` and not `0 < margin` the cached box is copied out. Otherwise, with
/// the copy `(low, high)`: if `0 < margin` both are moved apart by `margin * 0.5 * |high -
/// low|` (lane 3: the float at `01011d78`); if `scale != 1` the box is rescaled around its
/// centre `0.5 * (high + low)` by `0.5 * scale` times its extent. The scale and margin
/// arithmetic is the x87's (double precision, rounded to `float` where the exe stores one).
pub fn fn_00d75920(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    out: Ptr,
    index: u32,
    scale: f32,
    margin: f32,
) {
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    let cached = implementation + 0x90 + (index << 5);
    let low = load_vec(e, cached);
    let high = load_vec(e, cached + 0x10);
    let empty = (0..3).any(|lane| high[lane] <= low[lane]);
    if empty && fn_00d75050(e, this) > 0 {
        let head = fn_00d75060(e, this).addr();
        if index == 0 {
            let position = fn_00d74ed0(e, Ptr::new(head)).addr();
            copy_vec(e, position, cached);
            copy_vec(e, position, cached + 0x10);
            let mut node = fn_00d75120(e, Ptr::new(head));
            while node != 0 {
                let position = fn_00d74ed0(e, Ptr::new(node)).addr();
                let point = load_vec(e, position);
                let lower = zip_with(load_vec(e, cached), point, minps);
                store_vec(e, cached, lower);
                let point = load_vec(e, position);
                let upper = zip_with(load_vec(e, cached + 0x10), point, maxps);
                store_vec(e, cached + 0x10, upper);
                node = fn_00d75120(e, Ptr::new(node));
            }
        } else {
            e.with_stack(0x10, |e, scratch| {
                let first = fn_00d74e80(e, this, scratch, Ptr::new(head)).addr();
                copy_vec(e, first, cached);
                let second = fn_00d74e80(e, this, scratch, Ptr::new(head)).addr();
                copy_vec(e, second, cached + 0x10);
                let mut node = fn_00d75120(e, Ptr::new(head));
                while node != 0 {
                    let position = fn_00d74e80(e, this, scratch, Ptr::new(node)).addr();
                    let point = load_vec(e, position);
                    let lower = zip_with(load_vec(e, cached), point, minps);
                    store_vec(e, cached, lower);
                    let point = load_vec(e, position);
                    let upper = zip_with(load_vec(e, cached + 0x10), point, maxps);
                    store_vec(e, cached + 0x10, upper);
                    node = fn_00d75120(e, Ptr::new(node));
                }
            });
        }
    }
    if scale == 1.0 && !(0.0 < margin) {
        for word in 0..8 {
            let value = e.mem.u32(cached + 4 * word);
            e.mem.set_u32(out.addr() + 4 * word, value);
        }
        return;
    }
    let mut low = load_vec(e, cached);
    let mut high = load_vec(e, cached + 0x10);
    let half_double: f64 = e.global(HALF_DOUBLE);
    if 0.0 < margin {
        let extent = sub(high, low);
        let squares = mul(extent, extent);
        let length = sum3(squares).sqrt();
        let step = ((fld(margin) * half_double) * fld(length)) as f32;
        let spread = [step, step, step, e.mem.f32(ZERO_F32)];
        low = sub(low, spread);
        high = add(spread, high);
    }
    if scale != 1.0 {
        let half_scale = (half_double * fld(scale)) as f32;
        let extent = sub(high, low);
        let reach = mul(splat(half_scale), extent);
        let centre = mul(splat(e.mem.f32(HALF_FLOAT)), add(high, low));
        low = sub(centre, reach);
        high = add(centre, reach);
    }
    store_vec(e, out.addr(), low);
    store_vec(e, out.addr() + 0x10, high);
}

// Translated from 00d75b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The plane through the three vertices of the triangle `triangle`
/// (vertex pointers at `+8`, `+0xc`, `+0x10`; positions `+0x10` of the vertices), written to
/// `out`: the cross product of the edges `(v1 - v0) x (v2 - v0)` scaled by the refined
/// reciprocal length (`RSQRTSS` estimate, `0` for a zero length), with `0.0 - dot(normal, v0)`
/// (summed as `z + (y + x)`) as the fourth lane. The unnormalised vector is stored first, then
/// overwritten. Returns `out`.
pub fn fn_00d75b80(e: &mut Engine, _this: Ptr<HkgpConvexHull>, out: Ptr, triangle: Ptr) -> Ptr {
    let vertex_position = |e: &mut Engine, index: u32| -> Vec4 {
        let vertex = fn_00d752c0(e, triangle, index);
        let position = fn_00d74ed0(e, Ptr::new(vertex));
        load_vec(e, position.addr())
    };
    let origin = vertex_position(e, 0);
    let first = vertex_position(e, 1);
    let a = sub(first, origin);
    let second = vertex_position(e, 2);
    let b = sub(second, origin);
    let a_zxy = shufps(a, a, 0xd2);
    let a_yzx = shufps(a, a, 0xc9);
    let b_yzx = shufps(b, b, 0xc9);
    let b_zxy = shufps(b, b, 0xd2);
    let normal = sub(mul(b_zxy, a_yzx), mul(b_yzx, a_zxy));
    let length_squared = sum3(mul(normal, normal));
    let scale = inverse_length(e, length_squared);
    let unit = mul(splat(scale), normal);
    store_vec(e, out.addr(), normal);
    store_vec(e, out.addr(), unit);
    let offset = e.mem.f32(ZERO_VECTOR) - sum3(mul(unit, origin));
    e.mem
        .set_u32(out.addr() + 0xc, x87_quiet_bits(offset.to_bits()));
    out
}

/// The float the nearest-vertex search starts from (`0104da48`).
const SEARCH_START_NEAREST: u32 = 0x0104_da48;

/// The squared distance search shared in shape by `00d75cb0` and `00d75e30`: `out` starts as
/// the query; over the vertex list the squared distance (summed `z + (y + x)`) of the query
/// from the vertex position (`mode == 0`: `+0x10` of the node; otherwise the grid position)
/// is compared with the best so far (`better(best, distance)`); when it wins the position
/// becomes `out`.
fn vertex_search(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    mode: u32,
    query: Ptr,
    out: Ptr,
    start: f32,
    better: fn(f64, f64) -> bool,
) {
    let mut best = start;
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    copy_vec(e, query.addr(), out.addr());
    let mut node = e.mem.u32(implementation + 0x24);
    while node != 0 {
        let position = if mode == 0 {
            load_vec(e, node + 0x10)
        } else {
            grid_position(e, implementation, node)
        };
        let difference = sub(load_vec(e, query.addr()), position);
        let distance = sum3(mul(difference, difference));
        if better(fld(best), fld(distance)) {
            best = distance;
            store_vec(e, out.addr(), position);
        }
        node = e.mem.u32(node);
    }
}

// Translated from 00d75cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The vertex nearest to the point `query`: `out` is the query, then
/// each vertex position (`mode == 0`: the stored position, otherwise the grid position) whose
/// squared distance is smaller than the best so far (which starts at the `float` at
/// `0104da48`) replaces it.
pub fn fn_00d75cb0(e: &mut Engine, this: Ptr<HkgpConvexHull>, mode: u32, query: Ptr, out: Ptr) {
    let start = e.mem.f32(SEARCH_START_NEAREST);
    vertex_search(e, this, mode, query, out, start, |best, distance| {
        best > distance
    });
}

// Translated from 00d75e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The vertex farthest from the point `query`: as `00d75cb0`, with
/// the best starting at the `float` at `01011d78` and a larger distance winning.
pub fn fn_00d75e30(e: &mut Engine, this: Ptr<HkgpConvexHull>, mode: u32, query: Ptr, out: Ptr) {
    let start = e.mem.f32(ZERO_F32);
    vertex_search(e, this, mode, query, out, start, |best, distance| {
        best < distance
    });
}

/// The clamp limits and rounding offsets of `00d75fc0`: the doubles `8191.0` (`010d3698`) and
/// `-8192.0` (`010d3688`), the floats `8191.0` (`010d3690`), `-8192.0` (`010d3694`) and
/// `-0.5` (`0102295c`), and the float `0.5` at `01016248`.
const GRID_MAX_DOUBLE: u32 = 0x010d_3698;
const GRID_MAX_FLOAT: u32 = 0x010d_3690;
const GRID_MIN_DOUBLE: u32 = 0x010d_3688;
const GRID_MIN_FLOAT: u32 = 0x010d_3694;
const GRID_ROUND_DOWN: u32 = 0x0102_295c;

// Translated from 00d75fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Whether the hull has a vertex on the grid cell of the point
/// `point`: `q = (point - offset) * cell` (vectors at `+0xf0` and `+0x100` of the
/// implementation), each of the first three lanes clamped and rounded to an integer through the
/// x87 (`m = q`, or the maximum float when `q` is above the maximum double; below the minimum
/// double the sum is `minimum float + (-0.5)`, otherwise `m + (m < 0 ? -0.5 : 0.5)`; truncated
/// by `00ec62c0`), then the vertex list is searched for a node whose first two grid
/// coordinates (`+0x20`, `+0x24`) equal the first two integers (the third is computed but not
/// compared).
pub fn fn_00d75fc0(e: &mut Engine, this: Ptr<HkgpConvexHull>, point: Ptr) -> bool {
    let limits = grid_limits(e);
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    let cell: Vec4 =
        std::array::from_fn(|lane| f32::from_bits(e.mem.u32(point.addr() + 4 * lane as u32)));
    let offset = load_vec(e, implementation + 0xf0);
    let scale = load_vec(e, implementation + 0x100);
    let q = mul(sub(cell, offset), scale);
    let mut integers = [0i32; 3];
    for (lane, integer) in integers.iter_mut().enumerate() {
        *integer = grid_coordinate(e, &limits, q[lane]);
    }
    let mut node = e.mem.u32(implementation + 0x24);
    while node != 0 {
        if integers[0] == e.mem.i32(node + 0x20) && integers[1] == e.mem.i32(node + 0x24) {
            return true;
        }
        node = e.mem.u32(node);
    }
    false
}

// Translated from 00d76100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00d7b330` on the hull's implementation with `(point, flag, tolerance)` (the
/// float goes through the x87 on its way, so a signalling NaN is quieted); answers what it
/// answers.
pub fn fn_00d76100(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    point: Ptr,
    flag: u32,
    tolerance: f32,
) -> bool {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    fn_00d7b330(e, implementation, point, flag as u8, x87_quiet(tolerance))
}

// Translated from 00d76120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::fetchPositions` (Xbox PDB): appends the vertex positions to the `hkArray`
/// `array` of 16-byte vectors, growing it first (`00c908c0`). `mode == 0`: the stored position
/// of each node (`+0x10`); otherwise its grid position.
pub fn hkgp_convex_hull_fetch_positions(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    mode: u32,
    array: Ptr,
) {
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    let count = e.mem.i32(implementation + 0x28);
    let old_size = array_append(e, array.addr(), count, 0x10);
    let data = e.mem.u32(array.addr());
    let mut destination = (old_size as u32).wrapping_shl(4).wrapping_add(data);
    let mut node = e.mem.u32(implementation + 0x24);
    while node != 0 {
        if mode == 0 {
            copy_vec(e, node + 0x10, destination);
        } else {
            let position = grid_position(e, implementation, node);
            store_vec(e, destination, position);
        }
        destination = destination.wrapping_add(0x10);
        node = e.mem.u32(node);
    }
}

// Translated from 00d76250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::fetchPlanes` (Xbox PDB): appends the plane equations (`00d74fb0` for each
/// of `00d74f10` planes) to the `hkArray` `array` of 16-byte vectors, growing it first. The
/// count is asked again for the copy loop. The plane assertions apply.
pub fn hkgp_convex_hull_fetch_planes(e: &mut Engine, this: Ptr<HkgpConvexHull>, array: Ptr) {
    let count = fn_00d74f10(e, this);
    let old_size = array_append(e, array.addr(), count, 0x10);
    let data = e.mem.u32(array.addr());
    let mut destination = (old_size as u32).wrapping_shl(4).wrapping_add(data);
    let total = fn_00d74f10(e, this);
    if total > 0 {
        let mut index = 0;
        loop {
            let plane = fn_00d74fb0(e, this, index);
            copy_vec(e, plane, destination);
            destination = destination.wrapping_add(0x10);
            index += 1;
            if index >= total {
                break;
            }
        }
    }
}

/// The two helpers `00d76320` calls (cdecl): `00dedcf0(points, support)` builds the support
/// structure at `implementation + 0x60` from an array of points, `00dea810(support, a, b)` is
/// the query.
const SUPPORT_BUILD: u32 = 0x00de_dcf0;
const SUPPORT_QUERY: u32 = 0x00de_a810;

// Translated from 00d76320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. A query on the support structure at `+0x60` of the
/// implementation with `(input, out)` (`00dea810`). When the word at `+0x64` is still zero the
/// structure is built first: the vertex positions (`+0x10` of each node) are collected into a
/// temporary `hkArray` of 16-byte vectors that starts non-owning (capacity word
/// `0x80000000`), passed to `00dedcf0` with the structure, and the array's buffer is released
/// through the allocator's slot `0x10` (size = capacity * 16) unless it is still non-owning.
pub fn fn_00d76320(e: &mut Engine, this: Ptr<HkgpConvexHull>, input: Ptr, out: Ptr) {
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    build_support_if_missing(e, implementation);
    e.call(SUPPORT_QUERY, &args![implementation + 0x60, input, out]);
}

// Translated from 00d76840 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. For the plane `plane`: asks `00d76320` for the support point in
/// the direction of the negated plane (all four lanes xored with the sign mask at `010c44d0`)
/// and answers `dot(plane, point) + plane.w` (summed as `(z + (y + x)) + w`).
pub fn fn_00d76840(e: &mut Engine, this: Ptr<HkgpConvexHull>, plane: Ptr) -> f32 {
    let original = load_vec(e, plane.addr());
    let mask = load_bits(e, SIGN_MASK);
    let negated: Vec4 =
        std::array::from_fn(|lane| f32::from_bits(original[lane].to_bits() ^ mask[lane]));
    e.with_stack(0x10, |e, input| {
        e.with_stack(0x10, |e, point| {
            store_vec(e, input.addr(), negated);
            fn_00d76320(e, this, input, point);
            let plane_now = load_vec(e, plane.addr());
            let products = mul(plane_now, load_vec(e, point.addr()));
            x87_quiet(sum3(products) + plane_now[3])
        })
    })
}

// Translated from 00d768b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. As `00d76840` but with the support point in the direction of the
/// plane itself.
pub fn fn_00d768b0(e: &mut Engine, this: Ptr<HkgpConvexHull>, plane: Ptr) -> f32 {
    e.with_stack(0x10, |e, point| {
        fn_00d76320(e, this, plane, point);
        let plane_now = load_vec(e, plane.addr());
        let products = mul(plane_now, load_vec(e, point.addr()));
        x87_quiet(sum3(products) + plane_now[3])
    })
}

// Translated from 00d769c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The extent of the hull along the direction `direction` (three
/// floats read): the plane `(x, y, z, <float at 01011d78>)` is evaluated by `00d768b0` and
/// `00d76840`, and the answer is their difference rounded to a `float`.
pub fn fn_00d769c0(e: &mut Engine, this: Ptr<HkgpConvexHull>, direction: Ptr) -> f32 {
    let x = e.mem.f32(direction.addr());
    let y = e.mem.f32(direction.addr() + 4);
    let z = e.mem.f32(direction.addr() + 8);
    let plane = [x, y, z, e.mem.f32(ZERO_F32)];
    e.with_stack(0x10, |e, local| {
        store_vec(e, local.addr(), plane);
        let positive = fn_00d768b0(e, this, local);
        let negative = fn_00d76840(e, this, local);
        x87_quiet((fld(positive) - fld(negative)) as f32)
    })
}

/// `FLT_MAX` as a `double` (`0104e138`): the "centre not computed" marker the centre cache holds
/// in its fourth lane, and the zero vector (`01267e30`) its sum starts from.
const FLOAT_MAX_DOUBLE: u32 = 0x0104_e138;
const ZERO_VECTOR_GLOBAL: u32 = 0x0126_7e30;

// Translated from 00d7a180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The centre of the vertex positions, cached at `+0x70` of the
/// implementation (`this`) and answered as that address. It is computed when the cache's fourth
/// lane (`+0x7c`) equals the `FLT_MAX` marker: the sum of the positions (`ADDPS`, new position
/// plus the running sum), times `1.0 / count` (the count converted by `FILD`, the reciprocal
/// rounded to a `float`) when the vertex count (`+0x28`) is not zero, with the fourth lane then
/// set to `0.0`.
pub fn fn_00d7a180(e: &mut Engine, this: Ptr<HkgpConvexHullImpl>) -> Ptr {
    let at = this.addr();
    let centre = at + 0x70;
    let marker: f64 = e.global(FLOAT_MAX_DOUBLE);
    if fld(e.mem.f32(at + 0x7c)) == marker {
        copy_vec(e, ZERO_VECTOR_GLOBAL, centre);
        let mut node = e.mem.u32(at + 0x24);
        while node != 0 {
            let sum = add(load_vec(e, node + 0x10), load_vec(e, centre));
            store_vec(e, centre, sum);
            node = e.mem.u32(node);
        }
        let count = e.mem.i32(at + 0x28);
        if count != 0 {
            let inverse = (1.0 / f64::from(count)) as f32;
            let scaled = mul(splat(inverse), load_vec(e, centre));
            store_vec(e, centre, scaled);
        }
        e.mem.set_u32(centre + 0xc, 0);
    }
    Ptr::new(centre)
}

/// The vtable `hkgpConvexHull` carries while it is alive (`010d36e0`) and the base class's
/// (`0102e388`) the destructor leaves behind.
const HULL_VTABLE: u32 = 0x010d_36e0;
const HULL_BASE_VTABLE: u32 = 0x0102_e388;
/// The map operations `generateGeometry` uses on its local `hkMapBase<unsigned long, unsigned
/// long>`: reserve (`00cb1fe0`: `this` = map, `(allocator, count)`), insert (`00cb1870`,
/// `(allocator, key, value)`), get with default (`00cb1930`, `(key, default)`), clear and
/// deallocate (`00cb1a90`, `(allocator)`) and the no-op `00a29680` the code calls on the local
/// afterwards (a bare `RET`).
const MAP_RESERVE: u32 = 0x00cb_1fe0;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const LOCAL_NO_EFFECT: u32 = 0x00a2_9680;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl: `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkVector4::setTransformedPos` (`00c7f640`, `RET 8`: `this` = result, `(transform,
/// position)`) and `hkVector4::setRotatedDir` (`00c7f700`, `RET 8`: `this` = result,
/// `(rotation, direction)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// The CRT square root of the `double` in `ST0` (`00ec6040`).
const X87_SQRT: u32 = 0x00ec_6040;
/// The default extents (four words at `01267e30`) and the default transform (four rows at
/// `01268370`, `01268380`, `01268390`, `012683a0`) `createRandomSamples` starts from.
const DEFAULT_EXTENTS: u32 = 0x0126_7e30;
const DEFAULT_TRANSFORM: u32 = 0x0126_8370;
/// `"Failed to generate samples"` and the report id and line of that failure.
const MESSAGE_SAMPLES_FAILED: u32 = 0x010d_36fc;

// Translated from 00d76410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::generateGeometry` (Xbox PDB): appends the hull as triangles to the geometry
/// `geometry` (`+0` an `hkArray` of 16-byte vertices, `+0xc` an `hkArray` of 16-byte
/// triangles). `mode == 0` uses the stored vertex positions, otherwise the grid positions.
///
/// With two dimensions the vertices are appended, and a fan of `2 * (n - 2)` triangles (both
/// windings, only the three index words written) over a temporary index array that starts
/// non-owning. With three dimensions a local `hkMapBase` from vertex node to index (`base +
/// number of entries so far`, base = the vertex array's old size) is filled while the
/// vertices are appended, then each triangle node of the list at `+0x3c` is written as the
/// mapped indices of its three vertices (default `-1`) and `-1` as the fourth word; the map
/// is cleared and deallocated. Other dimension counts do nothing.
pub fn hkgp_convex_hull_generate_geometry(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    mode: u32,
    geometry: Ptr,
) {
    let plain = mode == 0;
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    let dimensions = e.mem.i32(implementation + 0x1a8);
    let geometry = geometry.addr();
    let write_position = |e: &mut Engine, node: u32, at: u32| {
        if plain {
            copy_vec(e, node + 0x10, at);
        } else {
            let implementation = e.get(this, HkgpConvexHull::implementation).addr();
            let position = grid_position(e, implementation, node);
            store_vec(e, at, position);
        }
    };
    if dimensions == 2 {
        let base = e.mem.u32(geometry + 4);
        e.with_stack(0x10, |e, list| {
            let list = list.addr();
            e.mem.set_u32(list, 0);
            e.mem.set_u32(list + 4, 0);
            e.mem.set_u32(list + 8, 0x8000_0000);
            let count = e.mem.i32(implementation + 0x28);
            let old_vertices = array_append(e, geometry, count, 0x10);
            let mut vertex_out = (old_vertices as u32)
                .wrapping_shl(4)
                .wrapping_add(e.mem.u32(geometry));
            let count = e.mem.i32(implementation + 0x28);
            let fan = count.wrapping_add(count).wrapping_sub(4);
            let old_triangles = array_append(e, geometry + 0xc, fan, 0x10);
            let mut triangle_out = (old_triangles as u32)
                .wrapping_shl(4)
                .wrapping_add(e.mem.u32(geometry + 0xc));
            let count = e.mem.i32(implementation + 0x28);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let capacity = (e.mem.u32(list + 8) & 0x3fff_ffff) as i32;
            if capacity < count {
                let doubled = capacity.wrapping_add(capacity);
                let target = if count < doubled { doubled } else { count };
                e.call(ARRAY_RESERVE, &args![allocator, list, target, 4u32]);
            }
            let mut node = e.mem.u32(implementation + 0x24);
            while node != 0 {
                let size = e.mem.u32(list + 4);
                let indices = e.mem.u32(list);
                e.mem.set_u32(indices + 4 * size, base.wrapping_add(size));
                e.mem.set_u32(list + 4, size.wrapping_add(1));
                write_position(e, node, vertex_out);
                vertex_out = vertex_out.wrapping_add(0x10);
                node = e.mem.u32(node);
            }
            let last = e.mem.i32(list + 4).wrapping_sub(1);
            let mut index = 1;
            while index < last {
                let indices = e.mem.u32(list);
                let first = e.mem.u32(indices);
                let second = e.mem.u32(indices + 4 * index as u32);
                let third = e.mem.u32(indices + 4 * index as u32 + 4);
                e.mem.set_u32(triangle_out, first);
                e.mem.set_u32(triangle_out + 4, second);
                e.mem.set_u32(triangle_out + 8, third);
                triangle_out = triangle_out.wrapping_add(0x10);
                let indices = e.mem.u32(list);
                let first = e.mem.u32(indices);
                let second = e.mem.u32(indices + 4 * index as u32 + 4);
                let third = e.mem.u32(indices + 4 * index as u32);
                e.mem.set_u32(triangle_out, first);
                e.mem.set_u32(triangle_out + 4, second);
                e.mem.set_u32(triangle_out + 8, third);
                triangle_out = triangle_out.wrapping_add(0x10);
                index += 1;
            }
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let capacity = e.mem.u32(list + 8);
            e.mem.set_u32(list + 4, 0);
            if (capacity as i32) >= 0 {
                let data = e.mem.u32(list);
                let bytes = (capacity & 0x3fff_ffff).wrapping_add(capacity & 0x3fff_ffff);
                e.vcall(allocator, 0x10, &args![data, bytes.wrapping_add(bytes)]);
            }
        });
    } else if dimensions == 3 {
        let base = e.mem.u32(geometry + 4);
        e.with_stack(0x10, |e, map| {
            let map = map.addr();
            e.mem.set_u32(map, 0);
            e.mem.set_u32(map + 4, 0);
            e.mem.set_u32(map + 8, 0xffff_ffff);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
            let count = e.mem.i32(implementation + 0x28);
            let old_vertices = array_append(e, geometry, count, 0x10);
            let mut vertex_out = (old_vertices as u32)
                .wrapping_shl(4)
                .wrapping_add(e.mem.u32(geometry));
            let triangle_count = e.mem.i32(implementation + 0x40);
            let old_triangles = array_append(e, geometry + 0xc, triangle_count, 0x10);
            let mut triangle_out = (old_triangles as u32)
                .wrapping_shl(4)
                .wrapping_add(e.mem.u32(geometry + 0xc));
            let count = e.mem.u32(implementation + 0x28);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(MAP_RESERVE, &args![map, allocator, count]);
            let mut node = e.mem.u32(implementation + 0x24);
            while node != 0 {
                let value = (e.mem.u32(map + 4) & 0x7fff_ffff).wrapping_add(base);
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let allocator = e.mem.u32(router + 0x10);
                e.call(MAP_INSERT, &args![map, allocator, node, value]);
                write_position(e, node, vertex_out);
                vertex_out = vertex_out.wrapping_add(0x10);
                node = e.mem.u32(node);
            }
            let mut triangle = e.mem.u32(implementation + 0x3c);
            while triangle != 0 {
                for corner in 0..3u32 {
                    let vertex = e.mem.u32(triangle + 8 + 4 * corner);
                    let mapped = e
                        .call(MAP_GET_WITH_DEFAULT, &args![map, vertex, 0xffff_ffffu32])
                        .u32();
                    e.mem.set_u32(triangle_out + 4 * corner, mapped);
                }
                e.mem.set_u32(triangle_out + 0xc, 0xffff_ffff);
                triangle = e.mem.u32(triangle);
                triangle_out = triangle_out.wrapping_add(0x10);
            }
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
            e.call(LOCAL_NO_EFFECT, &args![map]);
        });
    }
}

// Translated from 00d76940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::~hkgpConvexHull` (Xbox PDB): sets the vtable `010d36e0`; when there is an
/// implementation destroys it (`00d7ce20`) and returns its `0x1c0` bytes to the allocator
/// (slot 8); deletes the owned object (slot 0 with `1`) if there is one; then sets the base
/// class's vtable `0102e388` and clears the owned-object field.
pub fn hkgp_convex_hull_destruct(e: &mut Engine, this: Ptr<HkgpConvexHull>) {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    e.mem.set_u32(this.addr(), HULL_VTABLE);
    if !implementation.is_null() {
        hkgp_convex_hull_impl_destruct(e, implementation);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![implementation, 0x1c0u32]);
    }
    let owned = e.get(this, HkgpConvexHull::owned_object);
    if !owned.is_null() {
        e.vcall(owned.addr(), 0, &args![1u32]);
    }
    e.mem.set_u32(this.addr(), HULL_BASE_VTABLE);
    e.set(this, HkgpConvexHull::owned_object, Ptr::NULL);
}

// Translated from 00d76990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::buildIndices` (Xbox PDB): with a non-zero low byte of `clear_indices` the
/// implementation's `indices_valid` byte is cleared first; `tolerance` goes through the x87 to
/// `+4` of the implementation; then the implementation builds its indices (`00d7d2a0`).
pub fn hkgp_convex_hull_build_indices(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    tolerance: f32,
    clear_indices: u8,
) {
    if clear_indices != 0 {
        let implementation = e.get(this, HkgpConvexHull::implementation);
        e.set(implementation, HkgpConvexHullImpl::indices_valid, false);
    }
    let implementation = e.get(this, HkgpConvexHull::implementation);
    e.set(
        implementation,
        HkgpConvexHullImpl::index_argument,
        x87_quiet(tolerance),
    );
    let implementation = e.get(this, HkgpConvexHull::implementation);
    hkgp_convex_hull_impl_build_indices(e, implementation);
}

// Translated from 00d76a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Writes three planes (16 bytes each) to `out`, one per row of the
/// mass-property matrix `m` (`00d75490`), ordered by the extent of the hull along the row
/// (`00d769c0`, largest first; a bubble sort that restarts after each swap and only swaps when
/// the later extent is strictly greater): the row, normalised by `1 / sqrt` (CRT `00ec6040`,
/// `1.0 / length` rounded to `float`) unless its squared length is exactly zero, with the
/// fourth lane `0.0 - dot(plane, m[3])` (summed as `z + (y + x)`). The mass-property
/// assertion applies (`INT3` at `00d76aa4`).
pub fn fn_00d76a20(e: &mut Engine, this: Ptr<HkgpConvexHull>, out: Ptr) {
    guard_mass_properties(e, this, 0x00d7_6aa4);
    e.with_stack(0x40, |e, matrix| {
        e.with_stack(0x10, |e, vector| {
            fn_00d75490(e, this, matrix, vector);
            let mut order = [0usize, 1, 2];
            let mut extents = [0f32; 3];
            for (row, extent) in extents.iter_mut().enumerate() {
                let direction = Ptr::new(matrix.addr() + 0x10 * row as u32);
                *extent = fn_00d769c0(e, this, direction);
            }
            let mut index = 0;
            while index < 2 {
                if fld(extents[index + 1]) > fld(extents[index]) {
                    extents.swap(index, index + 1);
                    order.swap(index, index + 1);
                    index = 0;
                } else {
                    index += 1;
                }
            }
            let row3 = load_vec(e, matrix.addr() + 0x30);
            for (plane, source) in order.iter().enumerate() {
                let at = out.addr() + 0x10 * plane as u32;
                copy_vec(e, matrix.addr() + 0x10 * *source as u32, at);
                let saved = load_vec(e, at);
                let length_squared = sum3(mul(saved, saved));
                if length_squared != 0.0 {
                    let length = e.call(X87_SQRT, &args![fld(length_squared)]).f64() as f32;
                    let inverse = (1.0 / fld(length)) as f32;
                    store_vec(e, at, mul(splat(inverse), saved));
                }
                let products = mul(load_vec(e, at), row3);
                let offset = e.mem.f32(ZERO_VECTOR) - sum3(products);
                e.mem.set_u32(at + 0xc, x87_quiet_bits(offset.to_bits()));
            }
        });
    });
}

// Translated from 00d76ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::hkgpConvexHull` (Xbox PDB): sets the reference count to 1 and the vtable
/// `010d36e0`, allocates `0x1c0` bytes from the allocator (slot 4) and constructs the
/// implementation in it (`00d7f6a0`; a null block stays null), clears the two fields after it.
/// Returns `this`.
pub fn hkgp_convex_hull_construct(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
) -> Ptr<HkgpConvexHull> {
    e.set(this, HkgpConvexHull::reference_count, 1);
    e.mem.set_u32(this.addr(), HULL_VTABLE);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x1c0u32]).u32();
    let implementation = if block != 0 {
        hkgp_convex_hull_impl_construct(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    e.set(this, HkgpConvexHull::implementation, implementation);
    e.set(this, HkgpConvexHull::extra_word, 0);
    e.set(this, HkgpConvexHull::owned_object, Ptr::NULL);
    this
}

// Translated from 00d76cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::clone` (Xbox PDB): allocates `0x14` bytes (allocator slot 4), sets its
/// size word (`+4`) to `0x14` and constructs a hull in it (`00d76ca0`); the new hull's own
/// implementation is destroyed and freed, replaced by a clone of this one's (`00d7f720`); the
/// extra word is copied and the owned object cloned through its slot 4 (or cleared). Returns
/// the new hull.
pub fn hkgp_convex_hull_clone(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> Ptr<HkgpConvexHull> {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x14u32]).u32();
    let block: Ptr<HkgpConvexHull> = Ptr::new(block);
    e.set(block, HkgpConvexHull::memory_size, 0x14);
    let created = hkgp_convex_hull_construct(e, block);
    let old = e.get(created, HkgpConvexHull::implementation);
    if !old.is_null() {
        hkgp_convex_hull_impl_destruct(e, old);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![old, 0x1c0u32]);
    }
    e.set(created, HkgpConvexHull::implementation, Ptr::NULL);
    let source = e.get(this, HkgpConvexHull::implementation);
    let cloned = hkgp_convex_hull_impl_clone(e, source);
    e.set(created, HkgpConvexHull::implementation, cloned);
    let extra = e.get(this, HkgpConvexHull::extra_word);
    e.set(created, HkgpConvexHull::extra_word, extra);
    let owned = e.get(this, HkgpConvexHull::owned_object);
    if !owned.is_null() {
        let copy = e.vcall(owned.addr(), 4, &args![]).ptr::<()>();
        e.set(created, HkgpConvexHull::owned_object, copy);
    } else {
        e.set(created, HkgpConvexHull::owned_object, Ptr::NULL);
    }
    created
}

// Translated from 00d76d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::getOrientedBoundingBox` (Xbox PDB): a tail call of
/// `getOrientedRectangle` (`00d81340`) on the hull's implementation with the same two stack
/// words.
pub fn hkgp_convex_hull_impl_get_oriented_bounding_box(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    first: Ptr,
    second: Ptr,
) {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    hkgp_convex_hull_impl_get_oriented_rectangle(e, implementation, first, second)
}

// Translated from 00d76d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::getBestProjectionPlane` (Xbox PDB): writes a vector to `out` and returns
/// `out`. It starts as zero. With one dimension: the three absolute values of the vector at
/// `+0xd0` pick two indices `(c, d)` (`c` the larger of the first two, `d` the smaller of
/// them when the third is smaller than both, else 2; compares as the x87 flags read, ties and
/// NaN included) and `out[c] = v[d]`, `out[d] = -v[c]`, normalised by `1 / sqrt` (CRT
/// `00ec6040`) unless its squared length is exactly zero. With two: the vector at `+0xe0`.
/// With three: the oriented bounding box (`00d76d80`) gives extents and a rotation; the axis
/// of the smallest extent is rotated by it (`hkVector4::setRotatedDir`). In every case the
/// fourth lane ends as `0.0`.
pub fn hkgp_convex_hull_get_best_projection_plane(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    out: Ptr,
) -> Ptr {
    let at = out.addr();
    store_vec(e, at, [0.0; 4]);
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    let dimensions = e.mem.i32(implementation + 0x1a8);
    if dimensions == 1 {
        let component =
            |e: &Engine, index: u32| fld(e.mem.f32(implementation + 0xd0 + 4 * index)).abs() as f32;
        let a = [component(e, 0), component(e, 1), component(e, 2)];
        let (larger, smaller, minimum) = if fld(a[0]) > fld(a[1]) {
            (0u32, 1u32, a[1])
        } else {
            (1, 0, a[0])
        };
        let second = if fld(minimum) > fld(a[2]) { smaller } else { 2 };
        store_vec(e, at, [0.0; 4]);
        copy_through_x87(e, at + 4 * larger, implementation + 0xd0 + 4 * second);
        let negated = -fld(e.mem.f32(implementation + 0xd0 + 4 * larger));
        e.mem.set_f32(at + 4 * second, negated as f32);
        let vector = load_vec(e, at);
        let length_squared = sum3(mul(vector, vector));
        if length_squared != 0.0 {
            let length = e.call(X87_SQRT, &args![fld(length_squared)]).f64() as f32;
            let inverse = (1.0 / fld(length)) as f32;
            store_vec(e, at, mul(splat(inverse), vector));
        }
        e.mem.set_u32(at + 0xc, 0);
    } else if dimensions == 2 {
        copy_vec(e, implementation + 0xe0, at);
        e.mem.set_u32(at + 0xc, 0);
    } else if dimensions == 3 {
        e.with_stack(0x10, |e, extents| {
            e.with_stack(0x40, |e, transform| {
                hkgp_convex_hull_impl_get_oriented_bounding_box(e, this, extents, transform);
                let (a, b, c) = (
                    fld(e.mem.f32(extents.addr())),
                    fld(e.mem.f32(extents.addr() + 4)),
                    fld(e.mem.f32(extents.addr() + 8)),
                );
                let smallest = if b > a {
                    if c > a {
                        0
                    } else {
                        2
                    }
                } else if c > b {
                    1
                } else {
                    2
                };
                e.with_stack(0x10, |e, axis| {
                    store_vec(e, axis.addr(), [0.0; 4]);
                    e.mem.set_f32(axis.addr() + 4 * smallest, 1.0);
                    e.call(SET_ROTATED_DIR, &args![out, transform, axis]);
                });
                e.mem.set_u32(at + 0xc, 0);
            });
        });
    } else {
        e.mem.set_u32(at + 0xc, 0);
    }
    out
}

// Translated from 00d76fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::createRandomSamples` (Xbox PDB; cdecl `(hull, generator, count, array)`):
/// appends `count` random points inside the hull to the `hkArray` `array` of 16-byte vectors.
/// The oriented bounding box (`00d76d80`) is fetched into an extents vector and a transform
/// that start as the default words at `01267e30` and `01268370..`; each sample is
/// `extents * random` (`00d7a4b0` fills a vector of the generator), transformed by
/// `hkVector4::setTransformedPos` and tested with `00d76100(sample, indices_valid, 0.0)`, up to
/// `0x400` tries; a sample that passes is appended (growing the array with `00c90940` when it
/// is full). After `0x400` failed tries the report "Failed to generate samples" (severity 1, id
/// `0x1559e`, line `0xe22`; the sink's answer is ignored) is written and the function ends.
pub fn hkgp_convex_hull_create_random_samples(
    e: &mut Engine,
    hull: Ptr<HkgpConvexHull>,
    generator: Ptr,
    count: i32,
    array: Ptr,
) {
    e.with_stack(0x10, |e, extents| {
        e.with_stack(0x40, |e, transform| {
            for word in 0..4 {
                let value = e.mem.u32(DEFAULT_EXTENTS + 4 * word);
                e.mem.set_u32(extents.addr() + 4 * word, value);
            }
            for word in 0..16 {
                let value = e.mem.u32(DEFAULT_TRANSFORM + 4 * word);
                e.mem.set_u32(transform.addr() + 4 * word, value);
            }
            hkgp_convex_hull_impl_get_oriented_bounding_box(e, hull, extents, transform);
            if count <= 0 {
                return;
            }
            e.with_stack(0x10, |e, random| {
                e.with_stack(0x10, |e, sample| {
                    let array = array.addr();
                    let mut produced = 0;
                    loop {
                        let mut tries = 0;
                        loop {
                            hk_pseudo_random_generator_get_random_vector11(e, generator, random);
                            let scaled =
                                mul(load_vec(e, extents.addr()), load_vec(e, random.addr()));
                            store_vec(e, sample.addr(), scaled);
                            e.call(SET_TRANSFORMED_POS, &args![sample, transform, sample]);
                            let implementation = e.get(hull, HkgpConvexHull::implementation);
                            let indexed =
                                u32::from(e.get(implementation, HkgpConvexHullImpl::indices_valid));
                            let answer = fn_00d76100(e, hull, sample, indexed, 0.0);
                            if answer {
                                break;
                            }
                            tries += 1;
                            if tries >= 0x400 {
                                report_samples_failed(e);
                                return;
                            }
                        }
                        let router = e.call(MEMORY_ROUTER, &args![]).u32();
                        let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
                        if e.mem.u32(array + 4) == capacity {
                            let allocator = e.mem.u32(router + 0x10);
                            e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x10u32]);
                        }
                        let size = e.mem.u32(array + 4);
                        let data = e.mem.u32(array);
                        copy_vec(e, sample.addr(), size.wrapping_shl(4).wrapping_add(data));
                        produced += 1;
                        let size = e.mem.u32(array + 4);
                        e.mem.set_u32(array + 4, size.wrapping_add(1));
                        if produced >= count {
                            break;
                        }
                    }
                });
            });
        });
    });
}

/// The report `createRandomSamples` writes when it gives up (no break request is checked).
fn report_samples_failed(e: &mut Engine) {
    e.with_stack(0x20, |e, builder| {
        e.with_stack(0x200, |e, buffer| {
            let (builder, buffer) = (builder.addr(), buffer.addr());
            e.call(REPORT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
            e.call(REPORT_BUILDER_TEXT, &args![builder, MESSAGE_SAMPLES_FAILED]);
            let sink = e.global::<u32>(REPORT_SINK);
            e.vcall(
                sink,
                0xc,
                &args![1u32, 0x1559eu32, buffer, REPORT_FILE, 0xe22u32],
            );
            e.call(REPORT_BUILDER_DESTRUCT, &args![builder]);
        });
    });
}

// Translated from 00d771c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::build` (Xbox PDB): copies the 16 bytes at `vector` into the first four
/// words of the implementation, then builds from `(points, count)` with the implementation's
/// `build` (`00d81510`, flags `0, 0`) and answers its result.
pub fn hkgp_convex_hull_build(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    points: Ptr,
    count: i32,
    vector: Ptr,
) -> i32 {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    for word in 0..4 {
        let value = e.mem.u32(vector.addr() + 4 * word);
        e.mem.set_u32(implementation.addr() + 4 * word, value);
    }
    let implementation = e.get(this, HkgpConvexHull::implementation);
    hkgp_convex_hull_impl_build(e, implementation, points, count, 0, 0)
}

/// Copies `count` positions of `stride` bytes (three floats read, the fourth lane the float at
/// `01011d78`) from `data` into a temporary `hkArray` of 16-byte vectors that starts
/// non-owning, runs `body(array data or 0 when empty, count)`, then releases the array's block
/// through the allocator (slot `0x10`) unless it is still non-owning. The shape both
/// `build_ov2` and `00d77380` share.
fn with_padded_points(
    e: &mut Engine,
    source: u32,
    body: impl FnOnce(&mut Engine, u32, i32) -> i32,
) -> i32 {
    let data = e.mem.u32(source);
    let count = e.mem.i32(source + 4);
    e.with_stack(0x10, |e, list| {
        let list = list.addr();
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(list + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let target = if count < doubled { doubled } else { count };
            e.call(ARRAY_RESERVE, &args![allocator, list, target, 0x10u32]);
        }
        e.mem.set_i32(list + 4, count);
        let mut cursor = data;
        let mut index = 0;
        while index < e.mem.i32(source + 4) {
            let x = e.mem.f32(cursor);
            let y = e.mem.f32(cursor + 4);
            let z = e.mem.f32(cursor + 8);
            let vector = [x, y, z, e.mem.f32(ZERO_F32)];
            let destination = e.mem.u32(list).wrapping_add(0x10 * index as u32);
            store_vec(e, destination, vector);
            cursor = cursor.wrapping_add(e.mem.u32(source + 8));
            index += 1;
        }
        let size = e.mem.i32(list + 4);
        let points = if size != 0 { e.mem.u32(list) } else { 0 };
        let result = body(e, points, size);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(list + 8);
        e.mem.set_u32(list + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(list);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 4]);
        }
        result
    })
}

// Translated from 00d77200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::build` (second overload, Xbox PDB): `source` is a strided array of
/// positions (`+0` data, `+4` count, `+8` stride in bytes) and `vector` is passed on to
/// `build`. With a stride of 16 the data is used as it is. Otherwise the positions are copied
/// as vectors into a temporary non-owning `hkArray` (fourth lane the float at `01011d78`),
/// built from, and the array released through the allocator unless it stayed non-owning.
/// Answers the result of `build`.
pub fn hkgp_convex_hull_build_ov2(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    source: Ptr,
    vector: Ptr,
) -> i32 {
    if e.mem.u32(source.addr() + 8) == 0x10 {
        let data = e.mem.u32(source.addr());
        let count = e.mem.i32(source.addr() + 4);
        return hkgp_convex_hull_build(e, this, Ptr::new(data), count, vector);
    }
    with_padded_points(e, source.addr(), |e, points, count| {
        hkgp_convex_hull_build(e, this, Ptr::new(points), count, vector)
    })
}

// Translated from 00d77320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. As `00d771c0`, but also copies the vector `second` (16 bytes) to
/// `+0xe0` of the implementation and builds with the flags `1, 0`.
pub fn fn_00d77320(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    points: Ptr,
    count: i32,
    second: Ptr,
    first: Ptr,
) -> i32 {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    for word in 0..4 {
        let value = e.mem.u32(first.addr() + 4 * word);
        e.mem.set_u32(implementation.addr() + 4 * word, value);
    }
    let implementation = e.get(this, HkgpConvexHull::implementation);
    copy_vec(e, second.addr(), implementation.addr() + 0xe0);
    let implementation = e.get(this, HkgpConvexHull::implementation);
    hkgp_convex_hull_impl_build(e, implementation, points, count, 1, 0)
}

// Translated from 00d77380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. `00d77200`'s shape over `00d77320`: `source` is a strided array
/// of positions, `second` and `first` are the two vectors `00d77320` takes.
pub fn fn_00d77380(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    source: Ptr,
    second: Ptr,
    first: Ptr,
) -> i32 {
    if e.mem.u32(source.addr() + 8) == 0x10 {
        let data = e.mem.u32(source.addr());
        let count = e.mem.i32(source.addr() + 4);
        return fn_00d77320(e, this, Ptr::new(data), count, second, first);
    }
    with_padded_points(e, source.addr(), |e, points, count| {
        fn_00d77320(e, this, Ptr::new(points), count, second, first)
    })
}

/// `hkMatrix3::invert` (`00cd4f10`, `RET 4`: `this` = the matrix of three column vectors,
/// `(epsilon)`; answers 0 for success, 1 when it could not invert) and the `float` epsilon the
/// plane code passes it and compares distances with (`0104da4c`).
const MATRIX3_INVERT: u32 = 0x00cd_4f10;
const PLANE_EPSILON: u32 = 0x0104_da4c;
/// The profile label `buildFromPlanes` opens its timed section with (`"TtVertexEnumeration"`).
const PROFILE_VERTEX_ENUMERATION: u32 = 0x010d_3734;

/// The array release the exe inlines for a local `hkArray` of `element_size`-byte elements:
/// the size is cleared and, unless the capacity word has its top bit (non-owning) set, the
/// buffer is returned to the allocator (slot `0x10` of the router's `+0x10` object) with
/// `(capacity & 0x3fffffff) * element_size` bytes; the allocator is the router's field at `allocator_offset`.
fn release_array(e: &mut Engine, array: u32, element_size: u32, allocator_offset: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + allocator_offset);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// Appends the vector to the `hkArray` `array` of 16-byte vectors (allocator at the router's
/// `allocator_offset`): when the size equals the
/// capacity (low 30 bits of `+8`) `hkArrayUtil::_reserveMore` (`00c90940`, cdecl: `(allocator,
/// array, 0x10)`) makes room first.
fn array_push_vector(e: &mut Engine, array: u32, vector: Vec4, allocator_offset: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    let size = e.mem.u32(array + 4);
    if size == capacity {
        let allocator = e.mem.u32(router + allocator_offset);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x10u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    store_vec(e, size.wrapping_shl(4).wrapping_add(data), vector);
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// The vertex where three planes meet: the 3x3 matrix whose columns are `(a.x, b.x, c.x)`,
/// `(a.y, b.y, c.y)` and `(a.z, b.z, c.z)` (fourth lane the float at `01011d78`) is inverted
/// in place (`00cd4f10` with the epsilon float; `None` when it answers non-zero) and the
/// answer is `v.y * column1 + v.x * column0 + v.z * column2` summed in that order, `v` being
/// the negated plane offsets.
fn three_plane_vertex(e: &mut Engine, planes: [u32; 3], offsets: [f32; 3]) -> Option<Vec4> {
    let zero = e.mem.f32(ZERO_F32);
    let column = |e: &Engine, lane: u32| -> Vec4 {
        [
            e.mem.f32(planes[0] + 4 * lane),
            e.mem.f32(planes[1] + 4 * lane),
            e.mem.f32(planes[2] + 4 * lane),
            zero,
        ]
    };
    let columns = [column(e, 0), column(e, 1), column(e, 2)];
    let epsilon = x87_quiet_bits(e.mem.u32(PLANE_EPSILON));
    e.with_stack(0x30, |e, matrix| {
        for (index, vector) in columns.iter().enumerate() {
            store_vec(e, matrix.addr() + 0x10 * index as u32, *vector);
        }
        if e.call(MATRIX3_INVERT, &args![matrix, epsilon]).u32() != 0 {
            return None;
        }
        let first = load_vec(e, matrix.addr());
        let second = load_vec(e, matrix.addr() + 0x10);
        let third = load_vec(e, matrix.addr() + 0x20);
        let sum = add(
            mul(splat(offsets[1]), second),
            mul(splat(offsets[0]), first),
        );
        Some(add(sum, mul(splat(offsets[2]), third)))
    })
}

/// Whether the vertex lies on or inside every plane but the ones at `skipped`: `false` as soon
/// as a plane gives `epsilon < ((r.z * p.z + p.w) + (r.y * p.y + r.x * p.x))` (the exe's add
/// order; a NaN does not reject).
fn vertex_inside_planes(
    e: &mut Engine,
    vertex: Vec4,
    planes: u32,
    count: i32,
    skipped: &[i32],
) -> bool {
    let epsilon = fld(e.mem.f32(PLANE_EPSILON));
    let mut plane = planes;
    let mut index = 0;
    while index < count {
        if !skipped.contains(&index) {
            let products = mul(vertex, load_vec(e, plane));
            let distance = (products[2] + e.mem.f32(plane + 0xc)) + (products[1] + products[0]);
            if epsilon < fld(distance) {
                return false;
            }
        }
        index += 1;
        plane = plane.wrapping_add(0x10);
    }
    true
}

/// `0.0 - plane.w` through the x87 store (`SUBSS`, `FLD`, `FSTP`), the zero being lane 0 of the
/// vector at `010c7160`.
fn negated_offset(e: &mut Engine, plane: u32) -> f32 {
    x87_quiet(e.mem.f32(ZERO_VECTOR) - e.mem.f32(plane + 0xc))
}

// Translated from 00d774a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::buildFromPlanes` (Xbox PDB): builds the hull from `count` plane equations
/// (16 bytes each, at `planes`) and answers `build`'s result, or `-1` when fewer than four
/// vertices come out. The timed section `"TtVertexEnumeration"` (profile record `010d3734`,
/// closed with `010c41f8`) encloses the enumeration. Every triple of planes `i < j < k` is
/// intersected (`three_plane_vertex`, skipped when the matrix is singular) and a vertex that
/// is not outside any other plane is collected into a local `hkArray` of vectors (reserved for
/// `3 * count` entries first, starting non-owning); the collected vertices are built with
/// `00d771c0(vertices, count, config)` and the array released through the allocator.
pub fn hkgp_convex_hull_build_from_planes(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    planes: Ptr,
    count: i32,
    config: Ptr,
) -> i32 {
    e.with_stack(0x10, |e, list| {
        let list = list.addr();
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
        profile_record(e, PROFILE_VERTEX_ENUMERATION);
        let wanted = count.wrapping_mul(3);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(list + 8) & 0x3fff_ffff) as i32;
        if capacity < wanted {
            let doubled = capacity.wrapping_add(capacity);
            let target = if wanted < doubled { doubled } else { wanted };
            e.call(ARRAY_RESERVE, &args![allocator, list, target, 0x10u32]);
        }
        let mut i = 0;
        while i < count {
            let plane_i = planes.addr().wrapping_add(0x10 * i as u32);
            let first = negated_offset(e, plane_i);
            let mut j = i + 1;
            while j < count {
                let plane_j = planes.addr().wrapping_add(0x10 * j as u32);
                let second = negated_offset(e, plane_j);
                let mut k = j + 1;
                while k < count {
                    let plane_k = planes.addr().wrapping_add(0x10 * k as u32);
                    let third = negated_offset(e, plane_k);
                    if let Some(vertex) =
                        three_plane_vertex(e, [plane_i, plane_j, plane_k], [first, second, third])
                    {
                        if vertex_inside_planes(e, vertex, planes.addr(), count, &[i, j, k]) {
                            array_push_vector(e, list, vertex, 0x10);
                        }
                    }
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
        profile_record(e, PROFILE_END);
        let size = e.mem.i32(list + 4);
        if size > 3 {
            let data = e.mem.u32(list);
            let result = hkgp_convex_hull_build(e, this, Ptr::new(data), size, config);
            release_array(e, list, 0x10, 0x10);
            result
        } else {
            release_array(e, list, 0x10, 0x10);
            -1
        }
    })
}

// Translated from 00d77840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::buildFromPlanes` (second overload, Xbox PDB): as the first, for a hull in
/// the plane `third` (a 16-byte plane equation): every pair of planes `i < j` is intersected
/// with `third`, the skipped planes of the inside test being `i` and `j` only. With at least
/// three vertices they are built as a strided array `{data, size, 16}` by `00d77380(array,
/// third, config)`, otherwise the answer is `-1`.
pub fn hkgp_convex_hull_build_from_planes_ov2(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    planes: Ptr,
    count: i32,
    third: Ptr,
    config: Ptr,
) -> i32 {
    e.with_stack(0x10, |e, list| {
        let list = list.addr();
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
        profile_record(e, PROFILE_VERTEX_ENUMERATION);
        let wanted = count.wrapping_mul(3);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(list + 8) & 0x3fff_ffff) as i32;
        if capacity < wanted {
            let doubled = capacity.wrapping_add(capacity);
            let target = if wanted < doubled { doubled } else { wanted };
            e.call(ARRAY_RESERVE, &args![allocator, list, target, 0x10u32]);
        }
        let mut i = 0;
        while i < count {
            let plane_i = planes.addr().wrapping_add(0x10 * i as u32);
            let first = negated_offset(e, plane_i);
            let mut j = i + 1;
            while j < count {
                let plane_j = planes.addr().wrapping_add(0x10 * j as u32);
                let second = negated_offset(e, plane_j);
                let last = negated_offset(e, third.addr());
                if let Some(vertex) =
                    three_plane_vertex(e, [plane_i, plane_j, third.addr()], [first, second, last])
                {
                    if vertex_inside_planes(e, vertex, planes.addr(), count, &[i, j]) {
                        array_push_vector(e, list, vertex, 0x10);
                    }
                }
                j += 1;
            }
            i += 1;
        }
        profile_record(e, PROFILE_END);
        let size = e.mem.i32(list + 4);
        if size >= 3 {
            let data = e.mem.u32(list);
            let result = e.with_stack(0x10, |e, source| {
                e.mem.set_u32(source.addr(), data);
                e.mem.set_i32(source.addr() + 4, size);
                e.mem.set_u32(source.addr() + 8, 0x10);
                fn_00d77380(e, this, source, third, config)
            });
            release_array(e, list, 0x10, 0x10);
            result
        } else {
            release_array(e, list, 0x10, 0x10);
            -1
        }
    })
}

// Translated from 00d762f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::buildMassProperties` (Xbox PDB): a tail call of the implementation's
/// `buildMassProperties` (`00d7b600`) with `this` replaced by the hull's implementation; answers
/// what it answers (`true` when the mass properties are not valid afterwards: a failure).
pub fn hkgp_convex_hull_build_mass_properties(e: &mut Engine, this: Ptr<HkgpConvexHull>) -> bool {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    hkgp_convex_hull_impl_build_mass_properties(e, implementation)
}

/// The `float` `buildIndices` is asked to use after a rebuild (`0106ea1c`, the build
/// configuration's default).
const REBUILD_INDEX_FLOAT: u32 = 0x0106_ea1c;

// Translated from 00d78720 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Moves the hull's vertices or planes by `margin` and rebuilds the
/// hull. `params` is `{ mode: i32, limit: float }`. The index assertion applies (`INT3` at
/// `00d7879f`). The centre `c` is the block at `+0x150` (`00d75600`) when the mass properties
/// are built, else the centroid (`00d752d0`).
///
/// Mode 0, vertices: each vertex is kept when its distance from `c` is not above `limit`
/// (a NaN distance keeps it too); otherwise it moves along the ray from `c` to
/// `max(distance + margin, limit) / distance` times the offset (`scale * diff + c`; the
/// maximum is the x87's: `limit` replaces the sum only when it is strictly greater). The
/// moved vertices go to a temporary non-owning `hkArray`. Mode 1, planes: each plane keeps its
/// normal and its offset becomes `w - r`, `r = (margin < t) ? t : margin` with `t = limit + d`
/// (`d` the plane's signed distance of `c`, summed `(z + w) + (y + x)`). The array is built with
/// the implementation's own configuration (the implementation pointer `00d74e00` answers is
/// passed as the configuration, whose first 16 bytes it copies onto themselves): two
/// dimensions use the `third` plane `00d74e50` fetches (`00d77380` / `00d77840`), others
/// `build_ov2` / `buildFromPlanes`. Other modes do nothing. In every case the mass properties
/// (`00d762f0`) and indices (`buildIndices(float at 0106ea1c, 0)`) are rebuilt at the end.
pub fn fn_00d78720(e: &mut Engine, this: Ptr<HkgpConvexHull>, margin: f32, params: Ptr) {
    guard_indices(e, this, 0x00d7_879f);
    let implementation = e.get(this, HkgpConvexHull::implementation);
    let source = if e.get(implementation, HkgpConvexHullImpl::mass_properties_valid) {
        fn_00d75600(e, this)
    } else {
        fn_00d752d0(e, this).addr()
    };
    let centre: Vec4 =
        std::array::from_fn(|lane| f32::from_bits(e.mem.u32(source + 4 * lane as u32)));
    let mode = e.mem.i32(params.addr());
    if mode == 0 {
        e.with_stack(0x10, |e, list| {
            let list = list.addr();
            e.mem.set_u32(list, 0);
            e.mem.set_u32(list + 4, 0);
            e.mem.set_u32(list + 8, 0x8000_0000);
            let count = fn_00d75050(e, this);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            if ((e.mem.u32(list + 8) & 0x3fff_ffff) as i32) < count {
                e.call(ARRAY_RESERVE, &args![allocator, list, count, 0x10u32]);
            }
            let mut vertex = fn_00d75060(e, this).addr();
            while vertex != 0 {
                let position = fn_00d74ed0(e, Ptr::new(vertex)).addr();
                let point = load_vec(e, position);
                let difference = sub(point, centre);
                let squares = mul(difference, difference);
                let length = sum3(squares).sqrt();
                let limit = e.mem.f32(params.addr() + 4);
                if fld(limit) < fld(length) {
                    let mut target = (fld(length) + fld(margin)) as f32;
                    let limit = e.mem.f32(params.addr() + 4);
                    if fld(limit) > fld(target) {
                        target = x87_quiet(limit);
                    }
                    let scale = (fld(target) / fld(length)) as f32;
                    let moved = add(mul(splat(scale), difference), centre);
                    array_push_vector(e, list, moved, 0x10);
                } else {
                    array_push_vector(e, list, point, 0x10);
                }
                vertex = fn_00d75120(e, Ptr::new(vertex));
            }
            let dimensions = hkgp_convex_hull_get_dimensions(e, this);
            e.with_stack(0x10, |e, array| {
                let array = array.addr();
                e.mem.set_u32(array, e.mem.u32(list));
                e.mem.set_u32(array + 4, e.mem.u32(list + 4));
                e.mem.set_u32(array + 8, 0x10);
                let configuration = fn_00d74e00(e, this);
                if dimensions == 2 {
                    e.with_stack(0x10, |e, third| {
                        let third = fn_00d74e50(e, this, third);
                        fn_00d77380(e, this, Ptr::new(array), third, configuration.cast());
                    });
                } else {
                    hkgp_convex_hull_build_ov2(e, this, Ptr::new(array), configuration.cast());
                }
            });
            release_array(e, list, 0x10, 0x10);
        });
    } else if mode == 1 {
        e.with_stack(0x10, |e, list| {
            let list = list.addr();
            e.mem.set_u32(list, 0);
            e.mem.set_u32(list + 4, 0);
            e.mem.set_u32(list + 8, 0x8000_0000);
            let count = fn_00d74f10(e, this);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            if ((e.mem.u32(list + 8) & 0x3fff_ffff) as i32) < count {
                e.call(ARRAY_RESERVE, &args![allocator, list, count, 0x10u32]);
            }
            let mut index = 0;
            if fn_00d74f10(e, this) > 0 {
                loop {
                    let plane = fn_00d74fb0(e, this, index);
                    let limit = e.mem.f32(params.addr() + 4);
                    let mut moved: Vec4 = std::array::from_fn(|lane| {
                        f32::from_bits(e.mem.u32(plane + 4 * lane as u32))
                    });
                    let products = mul(moved, centre);
                    let distance = (products[2] + moved[3]) + (products[1] + products[0]);
                    let bound = (fld(limit) + fld(distance)) as f32;
                    let chosen = if fld(margin) < fld(bound) {
                        bound
                    } else {
                        margin
                    };
                    moved[3] = (fld(moved[3]) - fld(x87_quiet(chosen))) as f32;
                    array_push_vector(e, list, moved, 0x10);
                    index += 1;
                    if index >= fn_00d74f10(e, this) {
                        break;
                    }
                }
            }
            let dimensions = hkgp_convex_hull_get_dimensions(e, this);
            let data = e.mem.u32(list);
            let size = e.mem.i32(list + 4);
            let configuration = fn_00d74e00(e, this);
            if dimensions == 2 {
                e.with_stack(0x10, |e, third| {
                    let third = fn_00d74e50(e, this, third);
                    hkgp_convex_hull_build_from_planes_ov2(
                        e,
                        this,
                        Ptr::new(data),
                        size,
                        third,
                        configuration.cast(),
                    );
                });
            } else {
                hkgp_convex_hull_build_from_planes(
                    e,
                    this,
                    Ptr::new(data),
                    size,
                    configuration.cast(),
                );
            }
            release_array(e, list, 0x10, 0x10);
        });
    }
    hkgp_convex_hull_build_mass_properties(e, this);
    let tolerance = x87_quiet(e.mem.f32(REBUILD_INDEX_FLOAT));
    hkgp_convex_hull_build_indices(e, this, tolerance, 0);
}

/// Deletes an object through its virtual slot 0 with `1` (the deleting destructor) when it is
/// not null.
fn delete_object(e: &mut Engine, object: u32) {
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

/// Allocates a `0x14`-byte hull from the allocator (slot 4 of the router's `+0x10` object),
/// stores its size word `0x14` at `+4` and constructs it (`00d76ca0`).
fn new_hull(e: &mut Engine) -> Ptr<HkgpConvexHull> {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x14u32]).u32();
    e.mem.set_u16(block + 4, 0x14);
    hkgp_convex_hull_construct(e, Ptr::new(block))
}

// Translated from 00d77bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. A tail call of the implementation's `decimateVertices`
/// (`00d82ad0`) with `this` replaced by the hull's implementation and the two stack words
/// passed on; answers its result.
pub fn fn_00d77bd0(e: &mut Engine, this: Ptr<HkgpConvexHull>, count: i32, options: u32) -> bool {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    hkgp_convex_hull_impl_decimate_vertices(e, implementation, count, options)
}

// Translated from 00d78b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(hull, tolerance, options)`). Looks for a simplified copy
/// of a three-dimensional hull whose volume is within `tolerance` (relative) of the original's:
/// the answer starts as a clone; the vertex count is bisected between 4 and the hull's vertex
/// count, each probe a clone whose mass properties are built (`00d762f0`) and which is
/// decimated to the middle count (`00d77bd0(count, options)`). A probe that fails to decimate
/// ends the search with the best so far; one whose relative volume change `|(v - v0) / v0|`
/// (rounded to a `float`) is below the tolerance becomes the answer (the former answer is
/// deleted) and the upper bound, otherwise it is deleted and the lower bound moves. Answers
/// null for a hull that is not three-dimensional.
pub fn fn_00d78b90(
    e: &mut Engine,
    hull: Ptr<HkgpConvexHull>,
    tolerance: f32,
    options: u32,
) -> Ptr<HkgpConvexHull> {
    if hkgp_convex_hull_get_dimensions(e, hull) != 3 {
        return Ptr::NULL;
    }
    let mut best = hkgp_convex_hull_clone(e, hull);
    let reference = hkgp_convex_hull_get_volume(e, best);
    let mut low = 4i32;
    let mut high = fn_00d75050(e, hull);
    if (high.wrapping_sub(4) as u32) & 0xffff_fffe == 0 {
        return best;
    }
    loop {
        let middle = high.wrapping_add(low) >> 1;
        let candidate = hkgp_convex_hull_clone(e, hull);
        hkgp_convex_hull_build_mass_properties(e, candidate);
        if !fn_00d77bd0(e, candidate, middle, options) {
            delete_object(e, candidate.addr());
            return best;
        }
        let volume = hkgp_convex_hull_get_volume(e, candidate);
        let change = ((fld(volume) - fld(reference)) / fld(reference)) as f32;
        let magnitude = f32::from_bits(change.to_bits() & 0x7fff_ffff);
        let discarded;
        if fld(tolerance) > fld(magnitude) {
            discarded = best;
            best = candidate;
            high = middle;
        } else {
            discarded = candidate;
            low = middle;
        }
        delete_object(e, discarded.addr());
        if (high.wrapping_sub(low) as u32) & 0xffff_fffe == 0 {
            return best;
        }
    }
}

/// One element of a strided array of positions as a vector: three floats from the element and
/// the fourth lane from `+0xc` through the x87 when the stride is at least 16, else `0.0`.
fn strided_vector(e: &Engine, array: u32, index: i32) -> Vec4 {
    let stride = e.mem.i32(array + 8);
    let element = (stride.wrapping_mul(index) as u32).wrapping_add(e.mem.u32(array));
    let fourth = if stride >= 0x10 {
        x87_quiet(e.mem.f32(element + 0xc))
    } else {
        0.0
    };
    [
        e.mem.f32(element),
        e.mem.f32(element + 4),
        e.mem.f32(element + 8),
        fourth,
    ]
}

// Translated from 00d78ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(first, second, scale)`). A new hull of the points `a +
/// s * b` for every `a` of the strided array `first` and `b` of the strided array `second`
/// (`{data, count, stride}`; a vector's fourth lane is the element's `+0xc` when the stride is
/// at least 16, else `0.0`; `s` is the first float at `scale`; each result is `s * b + a`).
/// The points go to a temporary non-owning `hkArray` whose allocator is the router's `+0xc`
/// object (not `+0x10` as elsewhere), reserved for `count * count` entries. The hull is
/// constructed (`0x14` bytes through the allocator), the build configuration (default
/// constructor, then its first byte set to 1) is passed to `build_ov2` over the array, and the
/// hull is deleted and the answer null when that answers `-1`. The configuration's padding
/// bytes are stack contents in the exe and are zero here.
pub fn fn_00d78ca0(e: &mut Engine, first: Ptr, second: Ptr, scale: Ptr) -> Ptr<HkgpConvexHull> {
    e.with_stack(0x10, |e, list| {
        let list = list.addr();
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
        let wanted = e
            .mem
            .i32(second.addr() + 4)
            .wrapping_mul(e.mem.i32(first.addr() + 4));
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0xc);
        if ((e.mem.u32(list + 8) & 0x3fff_ffff) as i32) < wanted {
            e.call(ARRAY_RESERVE, &args![allocator, list, wanted, 0x10u32]);
        }
        let mut a_index = 0;
        while a_index < e.mem.i32(first.addr() + 4) {
            let a = strided_vector(e, first.addr(), a_index);
            let mut b_index = 0;
            while b_index < e.mem.i32(second.addr() + 4) {
                let b = strided_vector(e, second.addr(), b_index);
                let factor = e.mem.f32(scale.addr());
                array_push_vector(e, list, add(mul(splat(factor), b), a), 0xc);
                b_index += 1;
            }
            a_index += 1;
        }
        e.with_stack(0x10, |e, config| {
            hkgp_convex_hull_build_config_build_config(e, config);
            e.mem.set_u8(config.addr(), 1);
            let hull = new_hull(e);
            let result = e.with_stack(0x10, |e, source| {
                e.mem.set_u32(source.addr(), e.mem.u32(list));
                e.mem.set_u32(source.addr() + 4, e.mem.u32(list + 4));
                e.mem.set_u32(source.addr() + 8, 0x10);
                hkgp_convex_hull_build_ov2(e, hull, source, config)
            });
            let hull = if result == -1 {
                delete_object(e, hull.addr());
                Ptr::NULL
            } else {
                hull
            };
            release_array(e, list, 0x10, 0xc);
            hull
        })
    })
}

// Translated from 00d78ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(mode, hulls, count)`). A new hull of the union of the
/// positions of the `count` hulls in the array `hulls` (`fetchPositions(mode, array)` of each
/// into a temporary non-owning `hkArray`, reserved for the total vertex count). Answers null
/// for `count <= 0`. The configuration is the first 16 bytes of the first hull's implementation
/// with the first byte set to 1. The first non-null owned object (`+0x10`) found among the
/// hulls is cloned (slot 4) and every other hull's owned object is added to the clone (slot
/// 8); the new hull takes the clone as its owned object. It is built with `00d771c0`; when
/// its dimension count is `-1` the hull is deleted and the answer is null.
pub fn fn_00d78ea0(e: &mut Engine, mode: u32, hulls: Ptr, count: i32) -> Ptr<HkgpConvexHull> {
    if count <= 0 {
        return Ptr::NULL;
    }
    let hull_at = |e: &Engine, index: i32| -> Ptr<HkgpConvexHull> {
        Ptr::new(e.mem.u32(hulls.addr().wrapping_add(4 * index as u32)))
    };
    e.with_stack(0x10, |e, list| {
        let list = list.addr();
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
        let first = hull_at(e, 0);
        let implementation = fn_00d74e00(e, first).addr();
        e.with_stack(0x10, |e, config| {
            let config = config.addr();
            for word in 0..4 {
                let value = e.mem.u32(implementation + 4 * word);
                e.mem.set_u32(config + 4 * word, value);
            }
            e.mem.set_u8(config, 1);
            let mut total = 0i32;
            let mut shared = 0u32;
            let mut index = 0;
            while index < count {
                let hull = hull_at(e, index);
                total = total.wrapping_add(fn_00d75050(e, hull));
                if shared == 0 {
                    let owned = e.get(hull, HkgpConvexHull::owned_object).addr();
                    if owned != 0 {
                        shared = owned;
                    }
                }
                index += 1;
            }
            let mut clone = 0;
            if shared != 0 {
                clone = e.vcall(shared, 4, &args![]).u32();
            }
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let capacity = (e.mem.u32(list + 8) & 0x3fff_ffff) as i32;
            if capacity < total {
                let doubled = capacity.wrapping_add(capacity);
                let target = if total < doubled { doubled } else { total };
                e.call(ARRAY_RESERVE, &args![allocator, list, target, 0x10u32]);
            }
            let mut index = 0;
            while index < count {
                let hull = hull_at(e, index);
                if clone != 0 {
                    let owned = e.get(hull, HkgpConvexHull::owned_object).addr();
                    if owned != 0 && owned != shared {
                        e.vcall(clone, 8, &args![owned]);
                    }
                }
                hkgp_convex_hull_fetch_positions(e, hull, mode, Ptr::new(list));
                index += 1;
            }
            let hull = new_hull(e);
            e.set(hull, HkgpConvexHull::owned_object, Ptr::new(clone));
            let data = e.mem.u32(list);
            let size = e.mem.i32(list + 4);
            hkgp_convex_hull_build(e, hull, Ptr::new(data), size, Ptr::new(config));
            let dimensions = hkgp_convex_hull_get_dimensions(e, hull);
            if dimensions != -1 {
                release_array(e, list, 0x10, 0x10);
                hull
            } else {
                delete_object(e, hull.addr());
                release_array(e, list, 0x10, 0x10);
                Ptr::NULL
            }
        })
    })
}

// Translated from 00d79060 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(mode, first, second)`): `00d78ea0(mode, [first, second],
/// 2)` over a two-element array of hull pointers on the stack.
pub fn fn_00d79060(e: &mut Engine, mode: u32, first: Ptr, second: Ptr) -> Ptr<HkgpConvexHull> {
    e.with_stack(0x8, |e, hulls| {
        e.mem.set_u32(hulls.addr(), first.addr());
        e.mem.set_u32(hulls.addr() + 4, second.addr());
        fn_00d78ea0(e, mode, hulls, 2)
    })
}

// Translated from 00d790a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Forwards to the implementation's `splitByPlane<PositionAsSource>`
/// (`00d83430`) with `(first, second, third, fourth)`, the float going through the x87 on its
/// way; answers what it answers.
pub fn fn_00d790a0(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    first: u32,
    second: u32,
    third: u32,
    fourth: f32,
) -> u32 {
    let implementation = e.get(this, HkgpConvexHull::implementation);
    hkgp_convex_hull_impl_split_by_plane(e, implementation, first, second, third, x87_quiet(fourth))
}

/// Makes room for one more 4-byte element in an `hkArray` of words: when the size equals the
/// capacity (low 30 bits of `+8`) `hkArrayUtil::_reserveMore` (`00c90940`, `(allocator, array,
/// 4)`) is called first. The size is not changed.
fn array_prepare_word(e: &mut Engine, array: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
}

/// `array[size] = value; size += 1` after [`array_prepare_word`].
fn array_push_word(e: &mut Engine, array: u32, value: u32) {
    array_prepare_word(e, array);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00d790d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (thiscall with `this` = the hull, stack `(rounds, array,
/// threshold)`). Splits the hull into pieces. `array` (an `hkArray` of hull pointers) is
/// emptied and gets a clone of the hull. In each of `rounds` rounds every piece that was in
/// the array when the round began is processed: its mass properties are built (`00d762f0`), a
/// plane is chosen (`00d756a0`) and the piece is split by it (`00d790a0(plane, &first,
/// &second, threshold)`). When the split answers `3`: with both halves present the first is
/// appended to the array, the second takes the slot of the piece and the piece is deleted;
/// with only one half present that half is deleted and the piece stays. Other answers leave
/// the piece alone.
pub fn fn_00d790d0(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    rounds: i32,
    array: Ptr,
    threshold: f32,
) {
    let array = array.addr();
    e.mem.set_u32(array + 4, 0);
    let clone = hkgp_convex_hull_clone(e, this);
    array_push_word(e, array, clone.addr());
    if rounds <= 0 {
        return;
    }
    let mut remaining = rounds;
    loop {
        let size_at_start = e.mem.i32(array + 4);
        let mut index = 0;
        while index < size_at_start {
            let data = e.mem.u32(array);
            let piece = e.mem.u32(data.wrapping_add(4 * index as u32));
            hkgp_convex_hull_build_mass_properties(e, Ptr::new(piece));
            let (answer, first, second) = e.with_stack(0x10, |e, plane| {
                e.with_stack(0x8, |e, halves| {
                    fn_00d756a0(e, Ptr::new(piece), plane);
                    let answer = fn_00d790a0(
                        e,
                        Ptr::new(piece),
                        plane.addr(),
                        halves.addr(),
                        halves.addr() + 4,
                        threshold,
                    );
                    (
                        answer,
                        e.mem.u32(halves.addr()),
                        e.mem.u32(halves.addr() + 4),
                    )
                })
            });
            if answer == 3 {
                if first == 0 {
                    delete_object(e, second);
                } else if second == 0 {
                    delete_object(e, first);
                } else {
                    array_push_word(e, array, first);
                    array_prepare_word(e, array);
                    let size = e.mem.i32(array + 4);
                    let data = e.mem.u32(array);
                    e.mem.set_u32(data.wrapping_add(4 * size as u32), second);
                    if size != index {
                        let moved = e.mem.u32(data.wrapping_add(4 * size as u32));
                        e.mem.set_u32(data.wrapping_add(4 * index as u32), moved);
                    }
                    delete_object(e, piece);
                }
            }
            index += 1;
        }
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
}

// Translated from 00d79250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(hull, cutter, out_slot, out_array, threshold)`). Cuts a
/// clone of `hull` by the planes of the hull `cutter`. The planes' indices are listed in a
/// temporary non-owning `hkArray` of words; each round picks the plane with the greatest
/// `00d76840` value (the first of equals; a NaN never wins), removes it from the list by
/// moving the last index into its place, and splits the current piece with it
/// (`00d790a0(plane, &first, &second, threshold)`). Answer `3`: the current piece is deleted
/// and replaced by the second half, the first half is appended to `out_array` (if there is
/// one, else deleted). Answer `1`: the piece goes to `out_array` (or is deleted) and the cut
/// ends. Any other answer keeps the piece. At the end the piece is stored through `out_slot`
/// (or deleted when that is null). `*out_slot` starts as null and the size of `out_array` as 0.
pub fn fn_00d79250(
    e: &mut Engine,
    hull: Ptr<HkgpConvexHull>,
    cutter: Ptr<HkgpConvexHull>,
    out_slot: Ptr,
    out_array: Ptr,
    threshold: f32,
) {
    let mut current = hkgp_convex_hull_clone(e, hull).addr();
    let cutter_impl = e.get(cutter, HkgpConvexHull::implementation).addr();
    e.with_stack(0x10, |e, list| {
        let list = list.addr();
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
        if !out_slot.is_null() {
            e.mem.set_u32(out_slot.addr(), 0);
        }
        if !out_array.is_null() {
            e.mem.set_u32(out_array.addr() + 4, 0);
        }
        let count = fn_00d74f10(e, cutter);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(list + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let target = if count < doubled { doubled } else { count };
            e.call(ARRAY_RESERVE, &args![allocator, list, target, 4u32]);
        }
        e.mem.set_i32(list + 4, count);
        let total = fn_00d74f10(e, cutter);
        let mut index = 0;
        while index < total {
            let data = e.mem.u32(list);
            e.mem
                .set_u32(data.wrapping_add(4 * index as u32), index as u32);
            index += 1;
        }
        let plane_at = |e: &Engine, entry: i32| {
            let data = e.mem.u32(list);
            let planes = e.mem.u32(cutter_impl + 0x54);
            e.mem
                .u32(data.wrapping_add(4 * entry as u32))
                .wrapping_shl(4)
                .wrapping_add(planes)
        };
        while current != 0 && e.mem.i32(list + 4) > 0 {
            let first_plane = plane_at(e, 0);
            let mut best = fn_00d76840(e, cutter, Ptr::new(first_plane));
            let mut best_index = 0;
            let mut probe = 1;
            while probe < e.mem.i32(list + 4) {
                let probe_plane = plane_at(e, probe);
                let value = fn_00d76840(e, cutter, Ptr::new(probe_plane));
                if fld(best) < fld(value) {
                    best = value;
                    best_index = probe;
                }
                probe += 1;
            }
            let plane = plane_at(e, best_index);
            let remaining = e.mem.i32(list + 4).wrapping_sub(1);
            e.mem.set_i32(list + 4, remaining);
            if remaining != best_index {
                let data = e.mem.u32(list);
                let last = e.mem.u32(data.wrapping_add(4 * remaining as u32));
                e.mem
                    .set_u32(data.wrapping_add(4 * best_index as u32), last);
            }
            let piece = current;
            let (answer, first, second) = e.with_stack(0x8, |e, halves| {
                let answer = fn_00d790a0(
                    e,
                    Ptr::new(piece),
                    plane,
                    halves.addr(),
                    halves.addr() + 4,
                    threshold,
                );
                (
                    answer,
                    e.mem.u32(halves.addr()),
                    e.mem.u32(halves.addr() + 4),
                )
            });
            if answer == 1 {
                if out_array.is_null() {
                    delete_object(e, piece);
                } else {
                    array_push_word(e, out_array.addr(), piece);
                }
                current = 0;
            } else if answer == 3 {
                delete_object(e, piece);
                current = second;
                if out_array.is_null() {
                    delete_object(e, first);
                } else if first != 0 {
                    array_push_word(e, out_array.addr(), first);
                }
            }
        }
        if out_slot.is_null() {
            delete_object(e, current);
        } else {
            e.mem.set_u32(out_slot.addr(), current);
        }
        release_array(e, list, 4, 0x10);
    });
}

/// The three-word swap of two `hkArray` headers (`00d9ec10`, `RET 4`: `this` and `(other)`).
const ARRAY_SWAP: u32 = 0x00d9_ec10;

// Translated from 00d79510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(hull, cutter, array, threshold)`). Runs the cut
/// `00d79250(hull, cutter, &kept, array, threshold)` and answers `true` when it kept a piece
/// (the kept piece is deleted again). Otherwise the pieces the cut appended to `array` after
/// the size it had on entry are deleted and the array is emptied; the answer is `false`.
pub fn fn_00d79510(
    e: &mut Engine,
    hull: Ptr<HkgpConvexHull>,
    cutter: Ptr<HkgpConvexHull>,
    array: Ptr,
    threshold: f32,
) -> bool {
    let initial_size = e.mem.i32(array.addr() + 4);
    e.with_stack(8, |e, kept| {
        fn_00d79250(e, hull, cutter, kept, array, x87_quiet(threshold));
        let piece = e.mem.u32(kept.addr());
        if piece != 0 {
            delete_object(e, piece);
            return true;
        }
        let mut index = initial_size;
        while index < e.mem.i32(array.addr() + 4) {
            let data = e.mem.u32(array.addr());
            let appended = e.mem.u32(data.wrapping_add(4 * index as u32));
            delete_object(e, appended);
            index += 1;
        }
        e.mem.set_u32(array.addr() + 4, 0);
        false
    })
}

// Translated from 00d79580 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(hull, cutters, out, threshold)`; `cutters` an `hkArray`
/// of hull pointers, `out` an `hkArray` of words). Starts with a clone of `hull` as the only
/// piece and cuts every piece by each cutter in turn (`00d79510(piece, cutter, halves,
/// threshold)`): a piece the cut did not keep stays in the next list, a piece it kept is
/// deleted and the pieces it produced (`halves`) join the next list with their indices rebuilt
/// (`buildIndices(float at 0106ea1c, 0)`). After each cutter the two lists are swapped
/// (`00d9ec10`). The final pieces are appended to `out`; the answer is whether there are any.
/// All three working arrays are non-owning at first and released through the allocator.
pub fn fn_00d79580(
    e: &mut Engine,
    hull: Ptr<HkgpConvexHull>,
    cutters: Ptr,
    out: Ptr,
    threshold: f32,
) -> bool {
    e.with_stack(0x30, |e, block| {
        let pieces = block.addr();
        let next = block.addr() + 0x10;
        let halves = block.addr() + 0x20;
        for array in [pieces, next, halves] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
        let clone = hkgp_convex_hull_clone(e, hull);
        array_push_word(e, pieces, clone.addr());
        let mut cutter_index = 0;
        if e.mem.i32(cutters.addr() + 4) > 0 {
            loop {
                let size_at_start = e.mem.i32(pieces + 4);
                let mut piece_index = 0;
                while piece_index < size_at_start {
                    let cutter_data = e.mem.u32(cutters.addr());
                    let cutter = e.mem.u32(cutter_data.wrapping_add(4 * cutter_index as u32));
                    e.mem.set_u32(halves + 4, 0);
                    let piece_data = e.mem.u32(pieces);
                    let piece = e.mem.u32(piece_data.wrapping_add(4 * piece_index as u32));
                    let cut = fn_00d79510(
                        e,
                        Ptr::new(piece),
                        Ptr::new(cutter),
                        Ptr::new(halves),
                        threshold,
                    );
                    let piece_data = e.mem.u32(pieces);
                    let piece_slot = piece_data.wrapping_add(4 * piece_index as u32);
                    if !cut {
                        let piece = e.mem.u32(piece_slot);
                        array_push_word(e, next, piece);
                    } else {
                        let piece = e.mem.u32(piece_slot);
                        delete_object(e, piece);
                        let mut half_index = 0;
                        while half_index < e.mem.i32(halves + 4) {
                            let data = e.mem.u32(halves);
                            let half = e.mem.u32(data.wrapping_add(4 * half_index as u32));
                            array_push_word(e, next, half);
                            let tolerance = x87_quiet(e.mem.f32(REBUILD_INDEX_FLOAT));
                            let data = e.mem.u32(halves);
                            let half = e.mem.u32(data.wrapping_add(4 * half_index as u32));
                            hkgp_convex_hull_build_indices(e, Ptr::new(half), tolerance, 0);
                            half_index += 1;
                        }
                    }
                    piece_index += 1;
                }
                e.call(ARRAY_SWAP, &args![pieces, next]);
                e.mem.set_u32(next + 4, 0);
                cutter_index += 1;
                if cutter_index >= e.mem.i32(cutters.addr() + 4) {
                    break;
                }
            }
        }
        let mut index = 0;
        while index < e.mem.i32(pieces + 4) {
            let data = e.mem.u32(pieces);
            let piece = e.mem.u32(data.wrapping_add(4 * index as u32));
            array_push_word(e, out.addr(), piece);
            index += 1;
        }
        let any = e.mem.i32(pieces + 4) > 0;
        release_array(e, halves, 4, 0x10);
        release_array(e, next, 4, 0x10);
        release_array(e, pieces, 4, 0x10);
        any
    })
}

/// The `float` the best volume starts from (`01012054`) and the tolerance `fn_00d79850` splits
/// with (`0106c288`).
const BEST_VOLUME_START: u32 = 0x0101_2054;
const SPLIT_TOLERANCE: u32 = 0x0106_c288;

// Translated from 00d79850 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(hull, other, check, out_merged, out_part)`). Both `*out_*`
/// start as null and the index assertion applies to `hull` (`INT3` at `00d798ec`). Each plane
/// of `hull` (`00d74fb0`) is copied and `other` is split by it (`00d790a0(plane, &first,
/// &second, <float at 0106c288>)`). Only when the split answers `3` with both halves
/// three-dimensional, the second half's mass properties built (`00d762f0`; a non-zero answer
/// fails), its volume strictly above the best so far, the first half's mass properties built
/// and `00d79060(0, hull, second)` giving a three-dimensional hull whose mass properties are
/// built, and the object `check` answering true through its slot 4 (`(&flag, [merged, first],
/// 2)`; the answer is the pointer to the flag byte) does the split count: the previous
/// `*out_merged` and `*out_part` are deleted, the best becomes the second half's volume,
/// `*out_merged` the merged hull and `*out_part` the first half. Every other half and a
/// failed merged hull are deleted. Answers the best volume (`-1.0` when nothing counted).
pub fn fn_00d79850(
    e: &mut Engine,
    hull: Ptr<HkgpConvexHull>,
    other: Ptr<HkgpConvexHull>,
    check: Ptr,
    out_merged: Ptr,
    out_part: Ptr,
) -> f32 {
    let mut best = x87_quiet(e.mem.f32(BEST_VOLUME_START));
    e.mem.set_u32(out_merged.addr(), 0);
    e.mem.set_u32(out_part.addr(), 0);
    guard_indices(e, hull, 0x00d7_98ec);
    let mut index = 0;
    if fn_00d74f10(e, hull) > 0 {
        loop {
            let plane = fn_00d74fb0(e, hull, index);
            let (answer, first, second) = e.with_stack(0x10, |e, copy| {
                e.with_stack(0x8, |e, halves| {
                    copy_vec(e, plane, copy.addr());
                    let tolerance = x87_quiet(e.mem.f32(SPLIT_TOLERANCE));
                    let answer = fn_00d790a0(
                        e,
                        other,
                        copy.addr(),
                        halves.addr(),
                        halves.addr() + 4,
                        tolerance,
                    );
                    (
                        answer,
                        e.mem.u32(halves.addr()),
                        e.mem.u32(halves.addr() + 4),
                    )
                })
            });
            if answer == 3 && first == 0 {
                // Nothing to delete for a missing first half; the second is deleted below.
            } else if answer != 3
                || !split_counts(
                    e, hull, check, out_merged, out_part, first, second, &mut best,
                )
            {
                delete_object(e, first);
            }
            delete_object(e, second);
            index += 1;
            if index >= fn_00d74f10(e, hull) {
                break;
            }
        }
    }
    x87_quiet(best)
}

/// The tests of one split of `fn_00d79850` (see there): answers whether the split counted, in
/// which case the outputs are set and `first` now belongs to `out_part`; on failure the merged hull, if
/// one was made, has been deleted.
#[allow(clippy::too_many_arguments)]
fn split_counts(
    e: &mut Engine,
    hull: Ptr<HkgpConvexHull>,
    check: Ptr,
    out_merged: Ptr,
    out_part: Ptr,
    first: u32,
    second: u32,
    best: &mut f32,
) -> bool {
    if second == 0 {
        return false;
    }
    if hkgp_convex_hull_get_dimensions(e, Ptr::new(first)) != 3 {
        return false;
    }
    if hkgp_convex_hull_get_dimensions(e, Ptr::new(second)) != 3 {
        return false;
    }
    if hkgp_convex_hull_build_mass_properties(e, Ptr::new(second)) {
        return false;
    }
    let volume = hkgp_convex_hull_get_volume(e, Ptr::new(second));
    if !(fld(*best) < fld(volume)) {
        return false;
    }
    if hkgp_convex_hull_build_mass_properties(e, Ptr::new(first)) {
        return false;
    }
    let merged = fn_00d79060(e, 0, hull.cast(), Ptr::new(second));
    if merged.is_null() {
        return false;
    }
    if hkgp_convex_hull_get_dimensions(e, merged) != 3
        || hkgp_convex_hull_build_mass_properties(e, merged)
    {
        delete_object(e, merged.addr());
        return false;
    }
    let verdict = e.with_stack(0x10, |e, flag| {
        e.with_stack(0x8, |e, pair| {
            e.mem.set_u32(pair.addr(), merged.addr());
            e.mem.set_u32(pair.addr() + 4, first);
            let answer = e.vcall(check.addr(), 4, &args![flag, pair, 2u32]).u32();
            e.mem.u8(answer) != 0
        })
    });
    if !verdict {
        delete_object(e, merged.addr());
        return false;
    }
    let previous = e.mem.u32(out_merged.addr());
    delete_object(e, previous);
    let previous = e.mem.u32(out_part.addr());
    delete_object(e, previous);
    *best = hkgp_convex_hull_get_volume(e, Ptr::new(second));
    e.mem.set_u32(out_merged.addr(), merged.addr());
    e.mem.set_u32(out_part.addr(), first);
    true
}

/// The smallest width `fn_00d79ab0` accepts: the double `1.1920929e-06` at `010d37d8` that is
/// added to twice the radius and the float of the same value at `010d1018` that replaces a
/// smaller width; and the double `4.0` at `0101db80` the reference distance is scaled by.
const WIDTH_MARGIN_DOUBLE: u32 = 0x010d_37d8;
const WIDTH_MINIMUM_FLOAT: u32 = 0x010d_1018;
const REFERENCE_SCALE_DOUBLE: u32 = 0x0101_db80;

// Translated from 00d79ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(hull, points, out, radius)`; `points` an array of 16-byte
/// vectors `{data, count}`, `out` an `hkArray` of hull pointers). Splits the hull into clusters
/// of the points. The width `w` is `2 * radius + 1.1920929e-06` (a smaller value than the double
/// at `010d37d8` is replaced by the float at `010d1018`), its square `s` and `h = -w * 0.5` are
/// rounded to `float`. Pass 1 collects centres: a point is a new centre unless the squared
/// distance to some centre already collected is `<= s` (the centres go to two parallel
/// temporary arrays reserved for the point count). Pass 2 makes, for each centre `c`, a clone of
/// the hull with the configuration bytes `+9` and `+0xa` cleared, sorts the second array by
/// distance from `c` (`00d7aaf0(data, 0, count - 1, &c)` when it has more than one entry) and
/// walks it: for each point `b` with squared distance `d2 > s` from `c` the plane `(n, -0.5 *
/// (dot(n, b) + dot(n, c) + h))`, `n` = the unit direction from `c` to `b` (`RSQRTSS` refined,
/// `0` for a zero length), splits the piece (`00d790a0(plane, null, &half, 1.1920929e-06)`):
/// answer `1` deletes the piece and ends the walk; `3` replaces it by the half (and resets the
/// reference distance to `-1.0`); `2` ends the walk when the reference distance (taken from
/// `00d7a350` of the piece's implementation and `c` when it is negative) times `4.0` is below
/// `d2`. A piece that survives the walk gets its indices (`buildIndices(float at 0106ea1c, 0)`)
/// and mass properties rebuilt when the hull has them, and is appended to `out` (growing it
/// by `00c90940`). The temporary arrays are released through the allocator.
pub fn fn_00d79ab0(e: &mut Engine, hull: Ptr<HkgpConvexHull>, points: Ptr, out: Ptr, radius: f32) {
    let margin: f64 = e.global(WIDTH_MARGIN_DOUBLE);
    let sum = (fld(radius) + fld(radius) + margin) as f32;
    let width = if fld(sum) < margin {
        x87_quiet(e.global::<f32>(WIDTH_MINIMUM_FLOAT))
    } else {
        x87_quiet(sum)
    };
    let squared = (fld(width) * fld(width)) as f32;
    let half_double: f64 = e.global(HALF_DOUBLE);
    let extent = ((-fld(width)) * half_double) as f32;
    let half_float = e.mem.f32(HALF_FLOAT);
    e.with_stack(0x20, |e, block| {
        let centres = block.addr();
        let candidates = block.addr() + 0x10;
        for array in [centres, candidates] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
        for array in [centres, candidates] {
            let count = e.mem.i32(points.addr() + 4);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            if ((e.mem.u32(array + 8) & 0x3fff_ffff) as i32) < count {
                e.call(ARRAY_RESERVE, &args![allocator, array, count, 0x10u32]);
            }
        }
        let mut index = 0;
        while index < e.mem.i32(points.addr() + 4) {
            let point_at = e.mem.u32(points.addr()).wrapping_add(0x10 * index as u32);
            let point = load_vec(e, point_at);
            let size = e.mem.i32(centres + 4);
            let mut near = false;
            let mut scan = 0;
            while scan < size {
                let centre = load_vec(e, e.mem.u32(centres).wrapping_add(0x10 * scan as u32));
                let difference = sub(centre, point);
                let distance = sum3(mul(difference, difference));
                if fld(distance) <= fld(squared) {
                    near = true;
                    break;
                }
                scan += 1;
            }
            if !near {
                for array in [centres, candidates] {
                    let size = e.mem.u32(array + 4);
                    let destination = e.mem.u32(array).wrapping_add(size.wrapping_shl(4));
                    copy_vec(e, point_at, destination);
                    e.mem.set_u32(array + 4, size.wrapping_add(1));
                }
            }
            index += 1;
        }
        let mut reference = x87_quiet(e.global::<f32>(BEST_VOLUME_START));
        let mut cluster = 0;
        if e.mem.i32(centres + 4) > 0 {
            loop {
                let mut piece = hkgp_convex_hull_clone(e, hull).addr();
                let completed = e.with_stack(0x10, |e, centre_block| {
                    e.with_stack(0x10, |e, plane| {
                        e.with_stack(0x10, |e, scratch| {
                            e.with_stack(0x8, |e, slots| {
                                let centre_at = centre_block.addr();
                                copy_vec(
                                    e,
                                    e.mem.u32(centres).wrapping_add(0x10 * cluster as u32),
                                    centre_at,
                                );
                                let centre = load_vec(e, centre_at);
                                let count = e.mem.i32(candidates + 4);
                                if count > 1 {
                                    let data = e.mem.u32(candidates);
                                    fn_00d7aaf0(e, data, 0, count - 1, centre_block);
                                }
                                let implementation = e.mem.u32(piece + 8);
                                e.mem.set_u8(implementation + 9, 0);
                                let implementation = e.mem.u32(piece + 8);
                                e.mem.set_u8(implementation + 0xa, 0);
                                let mut walk = 0;
                                while walk < e.mem.i32(candidates + 4) {
                                    let candidate = load_vec(
                                        e,
                                        e.mem.u32(candidates).wrapping_add(0x10 * walk as u32),
                                    );
                                    let difference = sub(candidate, centre);
                                    let distance = sum3(mul(difference, difference));
                                    if fld(squared) < fld(distance) {
                                        let scale = inverse_length(e, distance);
                                        let direction = mul(splat(scale), difference);
                                        let along_candidate = sum3(mul(candidate, direction));
                                        let along_centre = sum3(mul(direction, centre));
                                        let total = (along_candidate + along_centre) + extent;
                                        let offset = (e.mem.f32(ZERO_VECTOR) - half_float) * total;
                                        store_vec(e, plane.addr(), direction);
                                        e.mem.set_u32(
                                            plane.addr() + 0xc,
                                            x87_quiet_bits(offset.to_bits()),
                                        );
                                        e.mem.set_u32(slots.addr(), 0);
                                        e.mem.set_u32(slots.addr() + 4, 0);
                                        let tolerance =
                                            x87_quiet(e.global::<f32>(WIDTH_MINIMUM_FLOAT));
                                        let answer = fn_00d790a0(
                                            e,
                                            Ptr::new(piece),
                                            plane.addr(),
                                            0,
                                            slots.addr() + 4,
                                            tolerance,
                                        );
                                        if answer == 1 {
                                            delete_object(e, piece);
                                            piece = 0;
                                        } else if answer == 3 {
                                            delete_object(e, piece);
                                            reference =
                                                x87_quiet(e.global::<f32>(BEST_VOLUME_START));
                                            piece = e.mem.u32(slots.addr() + 4);
                                        } else if answer == 2 {
                                            if reference < 0.0 {
                                                let implementation = e.mem.u32(piece + 8);
                                                let found = fn_00d7a350(
                                                    e,
                                                    Ptr::new(implementation),
                                                    scratch,
                                                    centre_block,
                                                );
                                                reference = x87_quiet(e.mem.f32(found.addr()));
                                            }
                                            let scaled = fld(reference)
                                                * e.global::<f64>(REFERENCE_SCALE_DOUBLE);
                                            if scaled < fld(distance) {
                                                walk = e.mem.i32(candidates + 4);
                                            }
                                        }
                                    }
                                    walk += 1;
                                    if piece == 0 {
                                        return false;
                                    }
                                }
                                true
                            })
                        })
                    })
                });
                if completed && piece != 0 {
                    let hull_impl = e.get(hull, HkgpConvexHull::implementation);
                    if e.get(hull_impl, HkgpConvexHullImpl::indices_valid) {
                        let tolerance = x87_quiet(e.mem.f32(REBUILD_INDEX_FLOAT));
                        hkgp_convex_hull_build_indices(e, Ptr::new(piece), tolerance, 0);
                    }
                    let hull_impl = e.get(hull, HkgpConvexHull::implementation);
                    if e.get(hull_impl, HkgpConvexHullImpl::mass_properties_valid) {
                        hkgp_convex_hull_build_mass_properties(e, Ptr::new(piece));
                    }
                    array_push_word(e, out.addr(), piece);
                }
                cluster += 1;
                if cluster >= e.mem.i32(centres + 4) {
                    break;
                }
            }
        }
        release_array(e, candidates, 0x10, 0x10);
        release_array(e, centres, 0x10, 0x10);
    });
}

// Translated from 00d79f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(hull, points, clusters, out, radius)`). Clusters the points
/// (`00d79ab0(hull, points, clusters, radius)`, which fills the `hkArray` `clusters` with hulls),
/// then makes a clone of the hull, shrinks it (`00d78720(-radius, config)` with a configuration
/// made by `00d74d90`, mode 1: planes) and cuts it by the cluster hulls (`00d79580(clone,
/// clusters, out, <float at 0106c288>)`); the clone is deleted at the end.
pub fn fn_00d79f90(
    e: &mut Engine,
    hull: Ptr<HkgpConvexHull>,
    points: Ptr,
    clusters: Ptr,
    out: Ptr,
    radius: f32,
) {
    fn_00d79ab0(e, hull, points, clusters, x87_quiet(radius));
    let clone = hkgp_convex_hull_clone(e, hull);
    e.with_stack(0x8, |e, config| {
        fn_00d74d90(e, config);
        let margin = -x87_quiet(radius);
        fn_00d78720(e, clone, margin, config);
    });
    let tolerance = x87_quiet(e.mem.f32(SPLIT_TOLERANCE));
    fn_00d79580(e, clone, clusters, out, tolerance);
    delete_object(e, clone.addr());
}

// Translated from 00d7a010 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(hull, points, out, radius)`). Runs `00d79f90` with a
/// temporary non-owning `hkArray` for the clusters, then deletes every hull left in it and
/// releases it through the allocator.
pub fn fn_00d7a010(e: &mut Engine, hull: Ptr<HkgpConvexHull>, points: Ptr, out: Ptr, radius: f32) {
    e.with_stack(0x10, |e, list| {
        let list = list.addr();
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
        fn_00d79f90(e, hull, points, Ptr::new(list), out, x87_quiet(radius));
        let mut index = 0;
        while index < e.mem.i32(list + 4) {
            let data = e.mem.u32(list);
            let cluster = e.mem.u32(data.wrapping_add(4 * index as u32));
            delete_object(e, cluster);
            index += 1;
        }
        release_array(e, list, 4, 0x10);
    });
}

// Translated from 00d7a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, stack `(transform, direction,
/// out)`). Over the vertex list: each position `p` is transformed by the four-vector matrix
/// `m` at `transform` as `((p.y * m1 + p.x * m0) + p.z * m2) + m3` (four lanes, each product
/// splatted), and the transformed vector with the greatest `dot(direction, t)` (summed `z + (y +
/// x)`; the first of equals, and a NaN replaces nothing but a first vertex) is written to
/// `out`.
pub fn fn_00d7a0b0(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    transform: Ptr,
    direction: Ptr,
    out: Ptr,
) {
    let mut node = e.mem.u32(this.addr() + 0x24);
    let mut best = 0.0f32;
    let mut found = false;
    while node != 0 {
        let position = load_vec(e, node + 0x10);
        let m0 = load_vec(e, transform.addr());
        let m1 = load_vec(e, transform.addr() + 0x10);
        let m2 = load_vec(e, transform.addr() + 0x20);
        let m3 = load_vec(e, transform.addr() + 0x30);
        let first = mul(splat(position[1]), m1);
        let second = mul(splat(position[0]), m0);
        let third = mul(splat(position[2]), m2);
        let transformed = add(add(add(first, second), third), m3);
        let distance = sum3(mul(load_vec(e, direction.addr()), transformed));
        if !found || fld(best) < fld(distance) {
            best = x87_quiet(distance);
            store_vec(e, out.addr(), transformed);
            found = true;
        }
        node = e.mem.u32(node);
    }
}

// Translated from 00d7a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, stack `(plane, epsilon, counts,
/// range)`). Classifies the vertices against the plane: for each vertex `d = ((p.z * n.z + w) +
/// (p.y * n.y + p.x * n.x))` (four-lane `ADDPS`, lane 0). `range` (when null, a temporary) is
/// set to `(min, max)` of the distances (the first vertex sets both; a NaN replaces neither).
/// `counts` is `(positive, non-positive)`: a vertex with `|d| < epsilon` gets `0.0` in its field
/// `+0x2c` and is not counted, otherwise the field gets `d` and `counts[0]` grows when `d > 0`,
/// else `counts[1]`. Answers `3` when both counts are non-zero, `2` when only the second is
/// non-zero, `1` when only the first is, else `0`.
pub fn fn_00d7a200(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    plane: Ptr,
    epsilon: f32,
    counts: Ptr,
    range: Ptr,
) -> u32 {
    e.with_stack(0x8, |e, scratch| {
        let range = if range.is_null() { scratch } else { range };
        let counts = counts.addr();
        let range = range.addr();
        e.mem.set_u32(counts + 4, 0);
        e.mem.set_u32(counts, 0);
        e.mem.set_u32(range + 4, 0);
        e.mem.set_u32(range, 0);
        let mut node = e.mem.u32(this.addr() + 0x24);
        while node != 0 {
            let first = e.mem.u32(node + 4) == 0;
            let plane_vector = load_vec(e, plane.addr());
            let products = mul(load_vec(e, node + 0x10), plane_vector);
            let distance = (products[2] + plane_vector[3]) + (products[1] + products[0]);
            if first {
                let bits = x87_quiet_bits(distance.to_bits());
                e.mem.set_u32(range + 4, bits);
                e.mem.set_u32(range, bits);
            } else {
                let mut lowest = x87_quiet(e.mem.f32(range));
                if fld(lowest) > fld(distance) {
                    lowest = x87_quiet(distance);
                }
                e.mem.set_f32(range, lowest);
                let mut highest = x87_quiet(e.mem.f32(range + 4));
                if fld(highest) < fld(distance) {
                    highest = x87_quiet(distance);
                }
                e.mem.set_f32(range + 4, highest);
            }
            let magnitude = f32::from_bits(distance.to_bits() & 0x7fff_ffff);
            if fld(magnitude) < fld(epsilon) {
                e.mem.set_u32(node + 0x2c, 0);
            } else {
                e.mem
                    .set_u32(node + 0x2c, x87_quiet_bits(distance.to_bits()));
                if fld(distance) > 0.0 {
                    let positive = e.mem.u32(counts);
                    e.mem.set_u32(counts, positive.wrapping_add(1));
                } else {
                    let negative = e.mem.u32(counts + 4);
                    e.mem.set_u32(counts + 4, negative.wrapping_add(1));
                }
            }
            node = e.mem.u32(node);
        }
        let positive = e.mem.i32(counts);
        let negative = e.mem.i32(counts + 4);
        if positive != 0 && negative != 0 {
            3
        } else if negative > positive {
            2
        } else {
            u32::from(positive != 0)
        }
    })
}

// Translated from 00d7a350 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, stack `(out, point)`). The vertex
/// farthest from `point` (squared distance summed `z + (y + x)`; one that is strictly
/// greater than the best so far, starting at the float at `01011d78` with `point` itself as the
/// best position): `out` is the squared distance `z + (y + x)` of `point - best` in lane 0 and
/// the square of its `z` in lanes 1..3. Returns `out`.
pub fn fn_00d7a350(e: &mut Engine, this: Ptr<HkgpConvexHullImpl>, out: Ptr, point: Ptr) -> Ptr {
    let origin = load_vec(e, point.addr());
    let mut best_distance = e.mem.f32(ZERO_F32);
    let mut best_position = origin;
    let mut node = e.mem.u32(this.addr() + 0x24);
    while node != 0 {
        let position = load_vec(e, node + 0x10);
        let difference = sub(origin, position);
        let squares = mul(difference, difference);
        let distance = sum3(squares);
        if fld(best_distance) < fld(distance) {
            best_distance = distance;
            best_position = position;
        }
        node = e.mem.u32(node);
    }
    let difference = sub(origin, best_position);
    let squares = mul(difference, difference);
    let mut result = [squares[2]; 4];
    result[0] = sum3(squares);
    store_vec(e, out.addr(), result);
    out
}

// Translated from 00d7a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(data, low, high, centre)`). Quicksort of the node
/// pointers `data[low..=high]` by the signed integer at `+0x38` of each node (Hoare's scheme
/// with the middle element as pivot, recursing on the left part and looping on the right);
/// `centre` is only passed on to the recursion.
pub fn fn_00d7a420(e: &mut Engine, data: u32, low: i32, high: i32, centre: Ptr) {
    let key = |e: &Engine, index: i32| -> i32 {
        let node = e.mem.u32(data.wrapping_add(4 * index as u32));
        e.mem.i32(node + 0x38)
    };
    let mut low = low;
    loop {
        let middle = (low.wrapping_add(high)) >> 1;
        let pivot = e.mem.u32(data.wrapping_add(4 * middle as u32));
        let mut left = low;
        let mut right = high;
        loop {
            let pivot_key = e.mem.i32(pivot + 0x38);
            while key(e, left) < pivot_key {
                left += 1;
            }
            while pivot_key < key(e, right) {
                right -= 1;
            }
            if right < left {
                break;
            }
            if right != left {
                let at_left = data.wrapping_add(4 * left as u32);
                let at_right = data.wrapping_add(4 * right as u32);
                let a = e.mem.u32(at_left);
                let b = e.mem.u32(at_right);
                e.mem.set_u32(at_right, a);
                e.mem.set_u32(at_left, b);
            }
            right -= 1;
            left += 1;
            if left > right {
                break;
            }
        }
        if low < right {
            fn_00d7a420(e, data, low, right, centre);
        }
        if left >= high {
            return;
        }
        low = left;
    }
}

/// The unsigned correction `4294967296.0` (a `float` at `01099378`) and the scale `2^-32`
/// (a `double` at `010229c0`) `getRandomVector11` converts the generator's state with, and the
/// `2.0` (a `double` at `01011590`).
const RANDOM_UNSIGNED_FIX: u32 = 0x0109_9378;
const RANDOM_SCALE: u32 = 0x0102_29c0;
const RANDOM_TWO: u32 = 0x0101_1590;

// Translated from 00d7a4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPseudoRandomGenerator::getRandomVector11` (Xbox PDB): four draws of the linear
/// congruential generator whose state is the word at `+4` (`state * 0x19660d + 0x3c6ef35f`,
/// 32-bit). Each state is converted as a signed integer (`FILD`) plus `4294967296.0` when
/// negative, times `2^-32` (rounded to `float`), times `2.0` minus `1.0` (double arithmetic,
/// rounded to `float`): the four floats go to `out` (lanes 0 to 3).
pub fn hk_pseudo_random_generator_get_random_vector11(e: &mut Engine, this: Ptr, out: Ptr) {
    let scale: f64 = e.global(RANDOM_SCALE);
    let two: f64 = e.global(RANDOM_TWO);
    for lane in 0..4u32 {
        let state = e
            .mem
            .u32(this.addr() + 4)
            .wrapping_mul(0x0019_660d)
            .wrapping_add(0x3c6e_f35f);
        e.mem.set_u32(this.addr() + 4, state);
        let mut value = f64::from(state as i32);
        if (state as i32) < 0 {
            value += fld(e.global::<f32>(RANDOM_UNSIGNED_FIX));
        }
        let unit = (value * scale) as f32;
        let drawn = (fld(unit) * two - 1.0) as f32;
        e.mem.set_f32(out.addr() + 4 * lane, drawn);
    }
}

/// Four `1.0` (`010ec7f0`), the vector the clipped segment's parameters are subtracted from.
const SEGMENT_ONES: u32 = 0x010e_c7f0;

// Translated from 00d7a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, stack `(start, end, t_in, t_out,
/// limit)`; the index assertion applies, `INT3` at `00d7a61c`). Clips the segment `start`..`end`
/// (two vectors, copied) against the planes: `*t_in` starts as `0.0` and `*t_out` as `1.0`. For
/// each plane (the count is read again after each) the signed distances `d0`, `d1` of the two
/// current endpoints are `(p.z * n.z + w) + (p.y * n.y + p.x * n.x)` (`MULPS`, `ADDSS`,
/// `ADDPS`), stored through the x87. An endpoint beyond `limit` (`d > limit`) sets its bit; both
/// bits end the function with `false`. A distance smaller than the epsilon float at `0104da4c`
/// in magnitude becomes `0.0`. When `d1 * d0 < 0` (the segment crosses) `t = d0 / (d0 - d1)`
/// (rounded to a `float`) goes to `*t_in` when `d0 < 0`, else to `*t_out`, and the current
/// endpoints become `(1 - t_in) * start + end * t_in` and `(1 - t_out) * start + end * t_out`
/// from the original (unclipped) vectors in memory. Answers `true` at the end.
pub fn fn_00d7a5a0(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    start: Ptr,
    end: Ptr,
    t_in: Ptr,
    t_out: Ptr,
    limit: f32,
) -> bool {
    assert_indices(e, this, 0x00d7_a61c);
    let load_words = |e: &Engine, at: u32| -> Vec4 {
        std::array::from_fn(|lane| f32::from_bits(e.mem.u32(at + 4 * lane as u32)))
    };
    let mut first = load_words(e, start.addr());
    let mut second = load_words(e, end.addr());
    e.mem.set_u32(t_in.addr(), 0);
    e.mem.set_f32(t_out.addr(), 1.0);
    if e.mem.i32(this.addr() + 0x58) <= 0 {
        return true;
    }
    let limit_value = fld(limit);
    let epsilon = fld(e.mem.f32(PLANE_EPSILON));
    let mut index = 0;
    let mut offset = 0u32;
    loop {
        let plane_at = e.mem.u32(this.addr() + 0x54).wrapping_add(offset);
        let plane = load_vec(e, plane_at);
        let at_first = mul(plane, first);
        let at_second = mul(plane, second);
        let mut distances = [
            x87_quiet((at_first[2] + plane[3]) + (at_first[1] + at_first[0])),
            x87_quiet((at_second[2] + plane[3]) + (at_second[1] + at_second[0])),
        ];
        let mut mask = 0;
        for (side, distance) in distances.iter_mut().enumerate() {
            if fld(*distance) > limit_value {
                mask += 1 << side;
            }
            let magnitude = f32::from_bits(distance.to_bits() & 0x7fff_ffff);
            if fld(magnitude) < epsilon {
                *distance = 0.0;
            }
        }
        if mask == 3 {
            return false;
        }
        if fld(distances[1]) * fld(distances[0]) < 0.0 {
            let parameter = (fld(distances[0]) / (fld(distances[0]) - fld(distances[1]))) as f32;
            if fld(distances[0]) < 0.0 {
                e.mem.set_f32(t_in.addr(), parameter);
            } else {
                e.mem.set_f32(t_out.addr(), parameter);
            }
            let enter = splat(e.mem.f32(t_in.addr()));
            let ones = load_vec(e, SEGMENT_ONES);
            let leave = splat(e.mem.f32(t_out.addr()));
            let original_start = load_vec(e, start.addr());
            let original_end = load_vec(e, end.addr());
            first = add(
                mul(sub(ones, enter), original_start),
                mul(original_end, enter),
            );
            second = add(
                mul(sub(ones, leave), original_start),
                mul(original_end, leave),
            );
        }
        offset = offset.wrapping_add(0x10);
        index += 1;
        if index >= e.mem.i32(this.addr() + 0x58) {
            break;
        }
    }
    true
}

// Translated from 00d7a810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, stack `(points, matrix, out)`;
/// `points` `{data, count}` of 16-byte vectors, `matrix` four 16-byte vectors). For each point
/// and each triangle node of the list at `+0x3c`: the three vertex positions are transformed
/// by the matrix (`((p.x * m0 + p.y * m1) + p.z * m2) + m3`), the plane through them built
/// (cross product of the edges `t1 - t0` and `t2 - t0`, normalised by the refined `RSQRTSS`,
/// fourth lane `0.0 - dot(normal, t0)` through the x87) and the point's signed distance
/// `(dot(p, n) summed z + (y + x)) + w` taken; a distance greater than the best so far (which
/// starts at the epsilon float at `0104da4c`) becomes the best and the plane is written to
/// `out`. Answers the best distance.
pub fn fn_00d7a810(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    points: Ptr,
    matrix: Ptr,
    out: Ptr,
) -> f32 {
    let mut best = x87_quiet(e.mem.f32(PLANE_EPSILON));
    let mut index = 0;
    if e.mem.i32(points.addr() + 4) > 0 {
        loop {
            let point_at = e.mem.u32(points.addr()).wrapping_add(0x10 * index as u32);
            let mut triangle = e.mem.u32(this.addr() + 0x3c);
            while triangle != 0 {
                let m0 = load_vec(e, matrix.addr());
                let m1 = load_vec(e, matrix.addr() + 0x10);
                let m2 = load_vec(e, matrix.addr() + 0x20);
                let m3 = load_vec(e, matrix.addr() + 0x30);
                let corners: [Vec4; 3] = std::array::from_fn(|corner| {
                    let vertex = e.mem.u32(triangle + 8 + 4 * corner as u32);
                    let position = load_vec(e, vertex + 0x10);
                    let sum = add(mul(splat(position[0]), m0), mul(splat(position[1]), m1));
                    add(add(sum, mul(splat(position[2]), m2)), m3)
                });
                let a = sub(corners[1], corners[0]);
                let b = sub(corners[2], corners[0]);
                let normal = sub(
                    mul(shufps(b, b, 0xd2), shufps(a, a, 0xc9)),
                    mul(shufps(b, b, 0xc9), shufps(a, a, 0xd2)),
                );
                let scale = inverse_length(e, sum3(mul(normal, normal)));
                let unit = mul(splat(scale), normal);
                let offset = x87_quiet(e.mem.f32(ZERO_VECTOR) - sum3(mul(unit, corners[0])));
                let plane = [unit[0], unit[1], unit[2], offset];
                let at_point = mul(load_vec(e, point_at), plane);
                let distance = x87_quiet(sum3(at_point) + plane[3]);
                if fld(best) < fld(distance) {
                    best = distance;
                    store_vec(e, out.addr(), plane);
                }
                triangle = e.mem.u32(triangle);
            }
            index += 1;
            if index >= e.mem.i32(points.addr() + 4) {
                break;
            }
        }
    }
    best
}

// Translated from 00d7aa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::getCentroid<hkgpConvexHullImpl::PositionAsTransform>` (Xbox PDB;
/// `this` = the implementation): the centre of the vertices' grid positions, cached at `+0x80`
/// and answered as that address. Computed as `00d7a180` does for the stored positions, with the
/// grid position of each vertex (as `00d74e80`) added to the running sum, when the cache's
/// fourth lane (`+0x8c`) equals the `FLT_MAX` marker.
pub fn hkgp_convex_hull_impl_get_centroid(e: &mut Engine, this: Ptr<HkgpConvexHullImpl>) -> Ptr {
    let at = this.addr();
    let centre = at + 0x80;
    let marker: f64 = e.global(FLOAT_MAX_DOUBLE);
    if fld(e.mem.f32(at + 0x8c)) == marker {
        copy_vec(e, ZERO_VECTOR_GLOBAL, centre);
        let mut node = e.mem.u32(at + 0x24);
        while node != 0 {
            let position = grid_position(e, at, node);
            let sum = add(position, load_vec(e, centre));
            store_vec(e, centre, sum);
            node = e.mem.u32(node);
        }
        let count = e.mem.i32(at + 0x28);
        if count != 0 {
            let inverse = (1.0 / f64::from(count)) as f32;
            let scaled = mul(splat(inverse), load_vec(e, centre));
            store_vec(e, centre, scaled);
        }
        e.mem.set_u32(centre + 0xc, 0);
    }
    Ptr::new(centre)
}

/// The squared distance of a vector from `centre`, summed `z + (y + x)`.
fn squared_distance(vector: Vec4, centre: Vec4) -> f32 {
    let difference = sub(vector, centre);
    sum3(mul(difference, difference))
}

// Translated from 00d7aaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(data, low, high, centre)`). Quicksort of the 16-byte
/// vectors `data[low..=high]` by their squared distance from the vector at `centre` (summed
/// `z + (y + x)`), ascending, Hoare's scheme with the middle element as pivot (its vector and
/// distance fixed for the round, the centre reloaded at each step), recursing on the left part
/// and looping on the right. The scans stop at an element that is not strictly on the near side
/// of the pivot's distance (a NaN stops them); swapped elements are exchanged 16 bytes at a
/// time.
pub fn fn_00d7aaf0(e: &mut Engine, data: u32, low: i32, high: i32, centre: Ptr) {
    let element = |index: i32| data.wrapping_add((index as u32).wrapping_shl(4));
    let mut low = low;
    loop {
        let middle = low.wrapping_add(high) >> 1;
        let pivot = load_vec(e, element(middle));
        let mut left = low;
        let mut right = high;
        loop {
            let centre_vector = load_vec(e, centre.addr());
            let pivot_distance = squared_distance(pivot, centre_vector);
            while fld(pivot_distance)
                > fld(squared_distance(load_vec(e, element(left)), centre_vector))
            {
                left += 1;
            }
            while fld(squared_distance(load_vec(e, element(right)), centre_vector))
                > fld(pivot_distance)
            {
                right -= 1;
            }
            if right < left {
                break;
            }
            if right != left {
                let from_right = load_bits(e, element(right));
                let from_left = load_bits(e, element(left));
                store_bits_at(e, element(right), from_left);
                store_bits_at(e, element(left), from_right);
            }
            right -= 1;
            left += 1;
            if left > right {
                break;
            }
        }
        if low < right {
            fn_00d7aaf0(e, data, low, right, centre);
        }
        if left >= high {
            return;
        }
        low = left;
    }
}

/// A 16-byte store of four words (`MOVAPS` of a register that holds them).
fn store_bits_at(e: &mut Engine, at: u32, words: [u32; 4]) {
    for (lane, word) in words.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * lane as u32, word);
    }
}

/// The CRT 64-bit multiply (`_allmul`, `00ecc300`, `RET 0x10`: `(low1, high1, low2, high2)`,
/// the product in `EDX:EAX`).
const ALLMUL: u32 = 0x00ec_c300;
/// The vtable slot of the report sink and the messages of the pool allocators' failure:
/// `"Item does not belong to the pool allocator"`, the id and line, and the source file
/// `"d:\build\20091222_000100_AiAnBeClDePh\Source\Common/Internal/GeometryProcessing/
/// hkGeometryProcessing.inl"`.
const MESSAGE_POOL_ITEM: u32 = 0x010d_365c;
const POOL_REPORT_FILE: u32 = 0x010d_35f0;

/// A text-only report: a builder over a `0x200`-byte buffer holding `message`, sent to the sink
/// with `(severity, id, text, file, line)`. When `break_address` is given a non-zero answer
/// is the `INT3` there.
fn report_text(
    e: &mut Engine,
    message: u32,
    severity: u32,
    id: u32,
    file: u32,
    line: u32,
    break_address: Option<u32>,
) {
    e.with_stack(0x20, |e, builder| {
        e.with_stack(0x200, |e, buffer| {
            let (builder, buffer) = (builder.addr(), buffer.addr());
            e.call(REPORT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
            e.call(REPORT_BUILDER_TEXT, &args![builder, message]);
            let sink = e.global::<u32>(REPORT_SINK);
            let answer = e
                .vcall(sink, 0xc, &args![severity, id, buffer, file, line])
                .u32();
            if let Some(address) = break_address {
                if answer != 0 {
                    panic!("INT3 at {address:08x} (assertion break)");
                }
            }
            e.call(REPORT_BUILDER_DESTRUCT, &args![builder]);
        });
    });
}

/// The release of an item to a pool allocator (`this` = `{blocks, count}`): the block whose
/// `[start, start + span]` contains the item (unsigned compares) has its use count at
/// `block + count_offset` decremented; when it reaches zero the block (`block_size` bytes) is
/// returned to the allocator (slot 8 of the router's `+0x10` object), removed from the list and
/// the last block moved into its place. An item that is in no block is the "Item does not
/// belong to the pool allocator" report (id `90355365`, line `0xaf`; `INT3` at
/// `break_address` on a non-zero answer). A null item does nothing.
fn pool_release(
    e: &mut Engine,
    pool: u32,
    item: u32,
    span: u32,
    count_offset: u32,
    block_size: u32,
    break_address: u32,
) {
    if item == 0 {
        return;
    }
    let mut index = 0;
    while index < e.mem.i32(pool + 4) {
        let blocks = e.mem.u32(pool);
        let block = e.mem.u32(blocks.wrapping_add(4 * index as u32));
        if item >= block && item <= block.wrapping_add(span) {
            let remaining = e.mem.i32(block + count_offset).wrapping_sub(1);
            e.mem.set_i32(block + count_offset, remaining);
            if remaining != 0 {
                return;
            }
            let blocks = e.mem.u32(pool);
            let block = e.mem.u32(blocks.wrapping_add(4 * index as u32));
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(allocator, 8, &args![block, block_size]);
            let count = e.mem.i32(pool + 4).wrapping_sub(1);
            e.mem.set_i32(pool + 4, count);
            if count != index {
                let blocks = e.mem.u32(pool);
                let last = e.mem.u32(blocks.wrapping_add(4 * count as u32));
                e.mem.set_u32(blocks.wrapping_add(4 * index as u32), last);
            }
            return;
        }
        index += 1;
    }
    report_text(
        e,
        MESSAGE_POOL_ITEM,
        3,
        0x9035_5365,
        POOL_REPORT_FILE,
        0xaf,
        Some(break_address),
    );
}

// Translated from 00d7adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::PoolAllocator<hkgpConvexHullImpl::Vertex, 32, hkContainerHeapAllocator>
/// ::release` (Xbox PDB): `this` = the pool (`{blocks, count}`). Releases the vertex `item` to
/// its block (`0x7c0` bytes of items, use count at `+0x804`, `0x808` bytes allocated); see
/// `pool_release` (`INT3` at `00d7ae32`).
pub fn hk_geometry_processing_pool_allocator_vertex_release(e: &mut Engine, this: Ptr, item: u32) {
    pool_release(e, this.addr(), item, 0x7c0, 0x804, 0x808, 0x00d7_ae32);
}

// Translated from 00d7aea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same for the triangle pool (the engine map has no name for it): blocks of `0x8b8`
/// bytes of items, use count at `+0x904`, `0x908` bytes allocated (`INT3` at `00d7af22`).
pub fn fn_00d7aea0(e: &mut Engine, this: Ptr, item: u32) {
    pool_release(e, this.addr(), item, 0x8b8, 0x904, 0x908, 0x00d7_af22);
}

// Translated from 00d7ac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::reset` (Xbox PDB; `this` = the implementation, one byte): calls the
/// embedded mesh's slot `0xc` (the object at `+0x10`) and returns the implementation to its
/// initial state: `+0x58`, `+0x4c`, `+0x64` and `+0x1ac` are cleared, the two flag bytes
/// `+0x1b4` and `+0x1b5` too, the centre caches `+0x70` and `+0x80` become `(0, 0, 0, FLT_MAX)`
/// with the zero the float at `01011d78` and `FLT_MAX` the float at `0104da48`, the boxes at
/// `+0x90..+0xd0` the maximum and minimum floats (`0104da48`, `0104e148`) in all lanes, the
/// matrix at `+0x120` the four vectors at `01268370`, `+0x160` the vector at `01267e30`,
/// `+0x170` the identity (`hkMatrix3::getIdentity`, `00cd4b50`, three columns), `+0x1a0` and
/// `+0x1a4` zero, `+0xd0`, `+0x100`, `+0x110` and `+0xf0` the vector at `01267e30`, `+0x1a8`
/// `-1`, and `+0xe0` the vector at `01267e30` when `keep_normal` is zero.
pub fn hkgp_convex_hull_impl_reset(e: &mut Engine, this: Ptr<HkgpConvexHullImpl>, keep_normal: u8) {
    let at = this.addr();
    e.vcall(at + 0x10, 0xc, &args![]);
    let zero = e.mem.u32(ZERO_F32);
    let largest = e.mem.u32(RESET_LARGEST);
    e.mem.set_u32(at + 0x58, 0);
    e.mem.set_u32(at + 0x4c, 0);
    e.mem.set_u32(at + 0x64, 0);
    e.mem.set_u8(at + 0x1b4, 0);
    e.mem.set_u8(at + 0x1b5, 0);
    e.mem.set_u32(at + 0x1ac, 0);
    store_bits_at(e, at + 0x70, [zero, zero, zero, largest]);
    store_bits_at(e, at + 0x80, [zero, zero, zero, largest]);
    let most_negative = e.mem.u32(NEGATIVE_FLOAT_MAX);
    store_bits_at(e, at + 0x90, [largest; 4]);
    store_bits_at(e, at + 0xa0, [most_negative; 4]);
    store_bits_at(e, at + 0xb0, [largest; 4]);
    store_bits_at(e, at + 0xc0, [most_negative; 4]);
    for row in 0..4u32 {
        copy_vec(e, RESET_MATRIX + 0x10 * row, at + 0x120 + 0x10 * row);
    }
    copy_vec(e, ZERO_VECTOR_GLOBAL, at + 0x160);
    let identity = e.call(MATRIX3_GET_IDENTITY, &args![]).u32();
    for column in 0..3u32 {
        copy_vec(e, identity + 0x10 * column, at + 0x170 + 0x10 * column);
    }
    e.mem.set_u32(at + 0x1a0, 0);
    e.mem.set_u32(at + 0x1a4, 0);
    copy_vec(e, ZERO_VECTOR_GLOBAL, at + 0xd0);
    e.mem.set_u32(at + 0x1a8, 0xffff_ffff);
    copy_vec(e, ZERO_VECTOR_GLOBAL, at + 0x100);
    copy_vec(e, ZERO_VECTOR_GLOBAL, at + 0x110);
    copy_vec(e, ZERO_VECTOR_GLOBAL, at + 0xf0);
    if keep_normal == 0 {
        copy_vec(e, ZERO_VECTOR_GLOBAL, at + 0xe0);
    }
}

/// The largest float (`0104da48`) `reset` fills the minimum caches with, the four default matrix
/// rows (`01268370`) and `hkMatrix3::getIdentity` (`00cd4b50`, answers the address of the
/// identity matrix's three columns).
const RESET_LARGEST: u32 = 0x0104_da48;
const RESET_MATRIX: u32 = 0x0126_8370;
const MATRIX3_GET_IDENTITY: u32 = 0x00cd_4b50;

// Translated from 00d7af90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(data, low, high, extra)`; `extra` is only passed on).
/// Quicksort of the `0x40`-byte records `data[low..=high]` by the three signed words at `+0x20`,
/// `+0x24` and `+0x28` compared lexicographically (ascending), Hoare's scheme with the middle
/// record's key copied as the pivot, recursing on the left part and looping on the right. A
/// swap exchanges the `0x28` bytes from `+0x10` of the two records (the float word at `+0x2c`
/// through the x87).
pub fn fn_00d7af90(e: &mut Engine, data: u32, low: i32, high: i32, extra: u32) {
    let record = |index: i32| data.wrapping_add((index as u32).wrapping_shl(6));
    let key = |e: &Engine, index: i32| -> [i32; 3] {
        let at = record(index);
        [
            e.mem.i32(at + 0x20),
            e.mem.i32(at + 0x24),
            e.mem.i32(at + 0x28),
        ]
    };
    let mut low = low;
    loop {
        let middle = low.wrapping_add(high) >> 1;
        let pivot = key(e, middle);
        let mut left = low;
        let mut right = high;
        loop {
            while key(e, left) < pivot {
                left += 1;
            }
            while pivot < key(e, right) {
                right -= 1;
            }
            if right < left {
                break;
            }
            if right != left {
                let (from_right, from_left) = (record(right), record(left));
                let mut saved = [0u32; 10];
                for (word, slot) in saved.iter_mut().enumerate() {
                    *slot = e.mem.u32(from_right + 0x10 + 4 * word as u32);
                }
                saved[7] = x87_quiet_bits(saved[7]);
                for word in 0..10u32 {
                    let value = e.mem.u32(from_left + 0x10 + 4 * word);
                    let value = if word == 7 {
                        x87_quiet_bits(value)
                    } else {
                        value
                    };
                    e.mem.set_u32(from_right + 0x10 + 4 * word, value);
                }
                for (word, value) in saved.iter().enumerate() {
                    e.mem.set_u32(from_left + 0x10 + 4 * word as u32, *value);
                }
            }
            right -= 1;
            left += 1;
            if left > right {
                break;
            }
        }
        if low < right {
            fn_00d7af90(e, data, low, right, extra);
        }
        if left >= high {
            return;
        }
        low = left;
    }
}

/// The double `-1.0` (`0101a6b0`) `fn_00d7b160` multiplies its differences by.
const SORT_NEGATIVE_ONE: u32 = 0x0101_a6b0;

// Translated from 00d7b160 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(data, low, high, extra)`). Quicksort of the 12-byte
/// records `data[low..=high]` by the float at `+8`, ascending, Hoare's scheme with the middle
/// record's key as the pivot (through the x87). The left scan runs while `(key - pivot) * -1.0`
/// (the double at `0101a6b0`) is above zero, the right scan while `(pivot - key) * -1.0` is;
/// whole 12-byte records are swapped.
pub fn fn_00d7b160(e: &mut Engine, data: u32, low: i32, high: i32, extra: u32) {
    let minus_one: f64 = e.global(SORT_NEGATIVE_ONE);
    let record = |index: i32| data.wrapping_add((index as u32).wrapping_mul(12));
    let mut low = low;
    loop {
        let middle = low.wrapping_add(high) >> 1;
        let pivot = fld(e.mem.f32(record(middle) + 8));
        let mut left = low;
        let mut right = high;
        loop {
            while (fld(e.mem.f32(record(left) + 8)) - pivot) * minus_one > 0.0 {
                left += 1;
            }
            while (pivot - fld(e.mem.f32(record(right) + 8))) * minus_one > 0.0 {
                right -= 1;
            }
            if right < left {
                break;
            }
            if right != left {
                for word in 0..3u32 {
                    let a = e.mem.u32(record(right) + 4 * word);
                    let b = e.mem.u32(record(left) + 4 * word);
                    e.mem.set_u32(record(right) + 4 * word, b);
                    e.mem.set_u32(record(left) + 4 * word, a);
                }
            }
            right -= 1;
            left += 1;
            if left > right {
                break;
            }
        }
        if low < right {
            fn_00d7b160(e, data, low, right, extra);
        }
        if left >= high {
            return;
        }
        low = left;
    }
}

// Translated from 00d7b270 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl `(data, low, high, extra)`). Quicksort of the 8-byte
/// records `data[low..=high]` by the float at `+4`, descending (the left scan runs while `key -
/// pivot > 0`, the right scan while `pivot - key > 0`, all in the x87's double precision),
/// Hoare's scheme with the middle record's key as the pivot; whole records are swapped.
pub fn fn_00d7b270(e: &mut Engine, data: u32, low: i32, high: i32, extra: u32) {
    let record = |index: i32| data.wrapping_add((index as u32).wrapping_shl(3));
    let mut low = low;
    loop {
        let middle = low.wrapping_add(high) >> 1;
        let pivot = fld(e.mem.f32(record(middle) + 4));
        let mut left = low;
        let mut right = high;
        loop {
            while fld(e.mem.f32(record(left) + 4)) - pivot > 0.0 {
                left += 1;
            }
            while pivot - fld(e.mem.f32(record(right) + 4)) > 0.0 {
                right -= 1;
            }
            if right < left {
                break;
            }
            if right != left {
                for word in 0..2u32 {
                    let a = e.mem.u32(record(right) + 4 * word);
                    let b = e.mem.u32(record(left) + 4 * word);
                    e.mem.set_u32(record(right) + 4 * word, b);
                    e.mem.set_u32(record(left) + 4 * word, a);
                }
            }
            right -= 1;
            left += 1;
            if left > right {
                break;
            }
        }
        if low < right {
            fn_00d7b270(e, data, low, right, extra);
        }
        if left >= high {
            return;
        }
        low = left;
    }
}

/// The clamp limits and rounding offsets of the grid coordinates (see `fn_00d75fc0`).
struct GridLimits {
    top_double: f64,
    top_float: f32,
    bottom_double: f64,
    bottom_float: f32,
    round_down: f32,
}

fn grid_limits(e: &Engine) -> GridLimits {
    GridLimits {
        top_double: e.global(GRID_MAX_DOUBLE),
        top_float: e.global(GRID_MAX_FLOAT),
        bottom_double: e.global(GRID_MIN_DOUBLE),
        bottom_float: e.global(GRID_MIN_FLOAT),
        round_down: e.global(GRID_ROUND_DOWN),
    }
}

/// The x87 clamp-and-round of one grid coordinate: `m = q` (a value above the top double gives
/// the top float); below the bottom double the sum is `bottom float + round_down`, else `m +
/// round_down` for a negative `m` and `m + 0.5` otherwise; truncated by `00ec62c0`.
fn grid_coordinate(e: &mut Engine, limits: &GridLimits, value: f32) -> i32 {
    let quiet = x87_quiet(value);
    let m = if fld(quiet) > limits.top_double {
        x87_quiet(limits.top_float)
    } else {
        quiet
    };
    let (rounding, base) = if fld(m) < limits.bottom_double {
        (x87_quiet(limits.round_down), x87_quiet(limits.bottom_float))
    } else if fld(m) < 0.0 {
        (x87_quiet(limits.round_down), m)
    } else {
        (x87_quiet(e.mem.f32(HALF_FLOAT)), m)
    };
    let sum = fld(rounding) + fld(base);
    e.call(FTOL, &args![sum]).i32()
}

/// A signed 32-bit product as the 64-bit `_allmul` of the sign-extended operands.
fn product_64(e: &mut Engine, first: i32, second: i32) -> i64 {
    e.call(
        ALLMUL,
        &args![
            first as u32,
            (first >> 31) as u32,
            second as u32,
            (second >> 31) as u32
        ],
    )
    .u64() as i64
}

// Translated from 00d7b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, stack `(point, exact, tolerance)`).
/// Whether the point lies inside the hull. With `exact == 0`: the point's grid coordinates
/// `q = (point - offset) * cell` (vectors at `+0xf0` and `+0x100`) are clamped and rounded as
/// `fn_00d75fc0` does, all three through `00ec62c0`, and the answer is `false` when a lane was
/// outside the clamp range (above the top double or below the bottom float); otherwise every
/// triangle node of the list at `+0x3c` is tested exactly: `n2 * q2 + n1 * q1 + n0 * q0 + c`
/// (`n` the words at `+0x20`, `+0x24`, `+0x28`, `c` the 64-bit word at `+0x30`; the products by
/// `_allmul`, summed as 64-bit integers) must not be positive. With `exact != 0`: the index
/// assertion applies (`INT3` at `00d7b57d`) and every plane must give `d = ((p.z * n.z + p.y *
/// n.y + p.x * n.x) ...)` that is not above `tolerance` (a plane's distance summed `(z + (y +
/// x)) + w`; NaN does not reject).
pub fn fn_00d7b330(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    point: Ptr,
    exact: u8,
    tolerance: f32,
) -> bool {
    let at = this.addr();
    if exact == 0 {
        let limits = grid_limits(e);
        let cell: Vec4 =
            std::array::from_fn(|lane| f32::from_bits(e.mem.u32(point.addr() + 4 * lane as u32)));
        let offset = load_vec(e, at + 0xf0);
        let scale = load_vec(e, at + 0x100);
        let q = mul(sub(cell, offset), scale);
        let mut outside = false;
        let mut integers = [0i32; 3];
        for (lane, integer) in integers.iter_mut().enumerate() {
            let high = fld(q[lane]) > limits.top_double;
            let low = !high && fld(limits.bottom_float) > fld(q[lane]);
            outside |= high || low;
            *integer = grid_coordinate(e, &limits, q[lane]);
        }
        if outside {
            return false;
        }
        let mut node = e.mem.u32(at + 0x3c);
        while node != 0 {
            let normal_z = e.mem.i32(node + 0x28);
            let product_2 = product_64(e, normal_z, integers[2]);
            let normal_y = e.mem.i32(node + 0x24);
            let product_1 = product_64(e, normal_y, integers[1]);
            let sum = product_2.wrapping_add(product_1);
            let normal_x = e.mem.i32(node + 0x20);
            let product_0 = product_64(e, normal_x, integers[0]);
            let constant =
                i64::from(e.mem.u32(node + 0x30)) | i64::from(e.mem.i32(node + 0x34)) << 32;
            let total = sum.wrapping_add(product_0).wrapping_add(constant);
            if total > 0 {
                return false;
            }
            node = e.mem.u32(node);
        }
        true
    } else {
        assert_indices(e, this, 0x00d7_b57d);
        let count = e.mem.i32(at + 0x58);
        if count <= 0 {
            return true;
        }
        let limit = fld(tolerance);
        let planes = e.mem.u32(at + 0x54);
        let vector = load_vec(e, point.addr());
        let mut plane = planes;
        let mut index = 0;
        loop {
            let values = load_vec(e, plane);
            let products = mul(values, vector);
            let distance = x87_quiet(sum3(products) + values[3]);
            if fld(distance) > limit {
                return false;
            }
            index += 1;
            plane = plane.wrapping_add(0x10);
            if index >= count {
                break;
            }
        }
        true
    }
}

/// The pools the implementation embeds (`hkGeometryProcessing::PoolAllocator`): a header `{blocks
/// (+0), number of blocks (+4), capacity word (+8), head of the list of used items (+0xc),
/// number of items (+0x10)}`. The vertex pool is at `+0x18` of the implementation (blocks of
/// `0x808` bytes: 32 items of `0x40` bytes, the free-list head at `+0x800` and the use count at
/// `+0x804`), the triangle pool at `+0x30` (blocks of `0x908` bytes: items of `0x48` bytes, free
/// list at `+0x900`, use count at `+0x904`). A pool's blocks are returned to the allocator with
/// slot 8 of the router's `+0x10` object.
fn pool_free_blocks(e: &mut Engine, pool: u32, block_size: u32) {
    let mut index = 0;
    while index < e.mem.i32(pool + 4) {
        let blocks = e.mem.u32(pool);
        let block = e.mem.u32(blocks.wrapping_add(4 * index as u32));
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![block, block_size]);
        index += 1;
    }
}

/// The first half of a pool's destruction after its blocks are freed: the block count is
/// cleared, the array of block pointers released when the capacity word does not have its top
/// bit (`(capacity & 0x3fffffff) * 4` bytes) and reset to non-owning.
fn pool_release_blocks_array(e: &mut Engine, pool: u32) {
    e.mem.set_u32(pool + 4, 0);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(pool + 8);
    e.mem.set_u32(pool + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(pool);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(pool + 8, 0x8000_0000);
    e.mem.set_u32(pool, 0);
}

// Translated from 00d7c200 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (fastcall, `this` = a vertex pool): frees every block (`0x808`
/// bytes) and clears the block count, the list head (`+0xc`) and the item count (`+0x10`).
pub fn fn_00d7c200(e: &mut Engine, this: Ptr) {
    pool_free_blocks(e, this.addr(), 0x808);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
}

// Translated from 00d7c250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same for a triangle pool (blocks of `0x908` bytes).
pub fn fn_00d7c250(e: &mut Engine, this: Ptr) {
    pool_free_blocks(e, this.addr(), 0x908);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
}

// Translated from 00d7c850 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = a vertex pool): frees every block and the array of block
/// pointers (see `pool_release_blocks_array`).
pub fn fn_00d7c850(e: &mut Engine, this: Ptr) {
    pool_free_blocks(e, this.addr(), 0x808);
    pool_release_blocks_array(e, this.addr());
}

// Translated from 00d7c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same for a triangle pool (blocks of `0x908` bytes).
pub fn fn_00d7c8d0(e: &mut Engine, this: Ptr) {
    pool_free_blocks(e, this.addr(), 0x908);
    pool_release_blocks_array(e, this.addr());
}

/// A new block for a pool: allocated through slot 4 of the allocator with `size` bytes, its 32
/// items (each `stride` bytes, the last at `last_item`) chained through their first words from
/// the highest to the lowest (the first item is the head, at `head_offset`), the use count at
/// `count_offset` set to 0, then appended to the pool's array of blocks and swapped with the
/// first block so it is first. Answers the block.
fn pool_new_block(
    e: &mut Engine,
    pool: u32,
    size: u32,
    last_item: u32,
    stride: u32,
    head_offset: u32,
    count_offset: u32,
) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    let mut previous = 0;
    let mut at = block.wrapping_add(last_item);
    let mut head = 0;
    for _ in 0..32 {
        e.mem.set_u32(at, previous);
        head = at;
        previous = at;
        at = at.wrapping_sub(stride);
    }
    e.mem.set_u32(block + count_offset, 0);
    e.mem.set_u32(block + head_offset, head);
    array_push_word(e, pool, block);
    let count = e.mem.i32(pool + 4);
    if count > 1 {
        let data = e.mem.u32(pool);
        let last = e.mem.u32(data.wrapping_add(4 * (count - 1) as u32));
        let first = e.mem.u32(data);
        e.mem.set_u32(data, last);
        let count = e.mem.i32(pool + 4);
        let data = e.mem.u32(pool);
        e.mem
            .set_u32(data.wrapping_add(4 * (count - 1) as u32), first);
    }
    block
}

// Translated from 00d7c2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::PoolAllocator<hkgpConvexHullImpl::Vertex, 32,
/// hkContainerHeapAllocator>::allocateNewPool` (Xbox PDB; `this` = the pool): a new `0x808`-byte
/// block of 32 vertex items (`0x40` bytes, the last at `+0x7c0`), see `pool_new_block`.
pub fn hk_geometry_processing_pool_allocator_vertex_allocate_new_pool(
    e: &mut Engine,
    this: Ptr,
) -> u32 {
    pool_new_block(e, this.addr(), 0x808, 0x7c0, 0x40, 0x800, 0x804)
}

// Translated from 00d7c340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::PoolAllocator<hkgpConvexHullImpl::Triangle, 32,
/// hkContainerHeapAllocator>::allocateNewPool` (Xbox PDB; `this` = the pool): a new `0x908`-byte
/// block of 32 triangle items (`0x48` bytes, the last at `+0x8b8`).
pub fn hk_geometry_processing_pool_allocator_triangle_allocate_new_pool(
    e: &mut Engine,
    this: Ptr,
) -> u32 {
    pool_new_block(e, this.addr(), 0x908, 0x8b8, 0x48, 0x900, 0x904)
}

// Translated from 00d7c950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpAbstractMesh<Edge, Vertex, Triangle, hkContainerHeapAllocator>::reset` (Xbox PDB; `this`
/// = the mesh): clears its vertex pool (`+8`, `00d7c200`) and its triangle pool (`+0x20`,
/// `00d7c250`).
pub fn hkgp_abstract_mesh_reset(e: &mut Engine, this: Ptr) {
    fn_00d7c200(e, Ptr::new(this.addr() + 8));
    fn_00d7c250(e, Ptr::new(this.addr() + 0x20));
}

/// The vtables the mesh and the implementation carry while alive (`010d36cc`) and the base's
/// (`0102e388`).
const MESH_VTABLE: u32 = 0x010d_36cc;

// Translated from 00d7d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpAbstractMesh<...>::scalar deleting destructor` (Xbox PDB; `this` = the mesh, one flag
/// byte): sets the mesh vtable `010d36cc`, clears and destroys the triangle pool (`+0x20`) and
/// the vertex pool (`+8`), sets the base vtable `0102e388` and, with bit 0 of the flag, returns
/// the block to the allocator (slot 8, size = the 16-bit word at `+4`). Returns `this`.
pub fn hkgp_abstract_mesh_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, MESH_VTABLE);
    fn_00d7c250(e, Ptr::new(at + 0x20));
    fn_00d7c8d0(e, Ptr::new(at + 0x20));
    fn_00d7c200(e, Ptr::new(at + 8));
    fn_00d7c850(e, Ptr::new(at + 8));
    e.mem.set_u32(at, HULL_BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = u32::from(e.mem.u16(at + 4));
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The vertex of a triangle at `index` (the pointer at `+8 + 4 * index`).
fn triangle_vertex(e: &Engine, triangle: u32, index: u32) -> u32 {
    e.mem.u32(triangle + 8 + 4 * index)
}

/// The three signed grid coordinates (`+0x20`) of a vertex, ordered lexicographically.
fn vertex_key(e: &Engine, vertex: u32) -> [i32; 3] {
    [
        e.mem.i32(vertex + 0x20),
        e.mem.i32(vertex + 0x24),
        e.mem.i32(vertex + 0x28),
    ]
}

/// The next index of a triangle's three corners (`(9 >> (2 * index)) & 3`: 0 to 1, 1 to 2, 2
/// to 0).
fn next_corner(index: u32) -> u32 {
    (9u32 >> ((2 * index) & 0x1f)) & 3
}

/// Whether the neighbour link of the triangle at `index` (at `+0x14 + 4 * index`, low two bits
/// are tags) is non-null.
fn has_neighbour(e: &Engine, triangle: u32, index: u32) -> bool {
    e.mem.u32(triangle + 0x14 + 4 * index) & 0xffff_fffc != 0
}

/// How the grid coordinates of corner `index` of the triangle compare with those of the next
/// corner.
fn compare_corners(e: &Engine, triangle: u32, index: u32) -> std::cmp::Ordering {
    let this_corner = vertex_key(e, triangle_vertex(e, triangle, index));
    let next = vertex_key(e, triangle_vertex(e, triangle, next_corner(index)));
    this_corner.cmp(&next)
}

// Translated from 00d7c970 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (fastcall, `this` = an edge cursor `{triangle (+0), corner
/// (+4)}`). Moves the cursor on until it stands on a canonical edge. Triangles are nodes with
/// the next triangle at `+0`, the corner vertices at `+8`, `+0xc`, `+0x10` and the neighbour
/// links at `+0x14`.. (tag bits in the low two bits). A corner is advanced to the next corner
/// (`(9 >> 2 * corner) & 3`) while the edge from it to the following corner goes up in the
/// lexicographic order of the vertices' signed grid coordinates (`+0x20`) and has a neighbour; the
/// cursor stops on an edge that does not go up or has no neighbour, otherwise moves to the next
/// triangle, starting at corner 0. A null triangle ends the walk.
pub fn fn_00d7c970(e: &mut Engine, this: Ptr) {
    let cursor = this.addr();
    let mut triangle = e.mem.u32(cursor);
    if triangle == 0 {
        return;
    }
    'restart: loop {
        let mut corner = e.mem.u32(cursor + 4);
        let advanced = loop {
            e.mem.set_u32(cursor, triangle);
            let candidate = next_corner(corner);
            e.mem.set_u32(cursor + 4, candidate);
            if candidate == 0 {
                break 0;
            }
            if compare_corners(e, triangle, candidate) == std::cmp::Ordering::Greater
                && has_neighbour(e, triangle, candidate)
            {
                corner = candidate;
                continue;
            }
            break candidate;
        };
        if advanced != 0 {
            let current = e.mem.u32(cursor);
            match compare_corners(e, current, advanced) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => return,
                std::cmp::Ordering::Greater => {
                    let stored = e.mem.u32(cursor + 4);
                    if !has_neighbour(e, current, stored) {
                        return;
                    }
                }
            }
        }
        let current = e.mem.u32(cursor);
        triangle = e.mem.u32(current);
        e.mem.set_u32(cursor, triangle);
        e.mem.set_u32(cursor + 4, 0);
        if triangle == 0 {
            return;
        }
        match compare_corners(e, triangle, 0) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => return,
            std::cmp::Ordering::Greater => {
                if !has_neighbour(e, triangle, 0) {
                    return;
                }
                continue 'restart;
            }
        }
    }
}

// Translated from 00d7ce20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::~hkgpConvexHullImpl` (Xbox PDB; `this` = the implementation): resets it
/// (`00d7ac40` with 0), releases the three arrays through the allocator (slot `0x10`, each
/// unless its capacity word has the top bit) and sets them non-owning: the support array at
/// `+0x60` (size at `+0x64`, `0x30`-byte elements), the plane array at `+0x54` (size `+0x58`,
/// capacity `+0x5c`, 16-byte elements) and the array at `+0x48` (size `+0x4c`, capacity
/// `+0x50`, `0x40`-byte elements), then destroys the triangle pool (`+0x30`) and the vertex pool
/// (`+0x18`) with the mesh vtable `010d36cc` at `+0x10` in between and the base vtable `0102e388`
/// at the end.
pub fn hkgp_convex_hull_impl_destruct(e: &mut Engine, this: Ptr<HkgpConvexHullImpl>) {
    let at = this.addr();
    hkgp_convex_hull_impl_reset(e, this, 0);
    for (data_offset, size_offset, capacity_offset, element_size) in [
        (0x60u32, 0x64u32, 0x68u32, 0x30u32),
        (0x54, 0x58, 0x5c, 0x10),
        (0x48, 0x4c, 0x50, 0x40),
    ] {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(at + capacity_offset);
        e.mem.set_u32(at + size_offset, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(at + data_offset);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_size);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(at + data_offset, 0);
        e.mem.set_u32(at + capacity_offset, 0x8000_0000);
    }
    e.mem.set_u32(at + 0x10, MESH_VTABLE);
    fn_00d7c250(e, Ptr::new(at + 0x30));
    fn_00d7c8d0(e, Ptr::new(at + 0x30));
    fn_00d7c200(e, Ptr::new(at + 0x18));
    fn_00d7c850(e, Ptr::new(at + 0x18));
    e.mem.set_u32(at + 0x10, HULL_BASE_VTABLE);
}

// Translated from 00d7d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = an edge cursor, stack `(triangle, corner)`). Sets the
/// cursor and, for a non-null triangle whose edge from `corner` to the next corner goes up in
/// the lexicographic order of the vertices' grid coordinates and has a neighbour, advances it
/// (`00d7c970`). Returns the cursor.
pub fn fn_00d7d220(e: &mut Engine, this: Ptr, triangle: u32, corner: u32) -> Ptr {
    e.mem.set_u32(this.addr(), triangle);
    e.mem.set_u32(this.addr() + 4, corner);
    if triangle != 0
        && compare_corners(e, triangle, corner) == std::cmp::Ordering::Greater
        && has_neighbour(e, triangle, corner)
    {
        fn_00d7c970(e, this);
    }
    this
}

/// The first free item of a list's pool: a new block when the list has none (`allocate_new`)
/// or its first block has no free item (head at `head_offset` is zero); the item is taken off the
/// block's free list and the block's use count at `count_offset` grows. Answers the item.
fn list_take_item(
    e: &mut Engine,
    list: u32,
    head_offset: u32,
    count_offset: u32,
    allocate_new: fn(&mut Engine, u32) -> u32,
) -> u32 {
    let mut block;
    if e.mem.u32(list + 4) == 0 {
        block = allocate_new(e, list);
    } else {
        let blocks = e.mem.u32(list);
        block = e.mem.u32(blocks);
        if e.mem.u32(block + head_offset) == 0 {
            block = allocate_new(e, list);
        }
    }
    let item = e.mem.u32(block + head_offset);
    let next = e.mem.u32(item);
    e.mem.set_u32(block + head_offset, next);
    let used = e.mem.u32(block + count_offset);
    e.mem.set_u32(block + count_offset, used.wrapping_add(1));
    item
}

/// Links a new item in front of the list: `item.prev = 0`, `item.next = head`, the old head's
/// `prev = item`, the count (`+0x10`) grows and the head (`+0xc`) is the item.
fn list_link_front(e: &mut Engine, list: u32, item: u32) {
    e.mem.set_u32(item + 4, 0);
    let head = e.mem.u32(list + 0xc);
    e.mem.set_u32(item, head);
    let head = e.mem.u32(list + 0xc);
    if head != 0 {
        e.mem.set_u32(head + 4, item);
    }
    let count = e.mem.u32(list + 0x10);
    e.mem.set_u32(list + 0x10, count.wrapping_add(1));
    e.mem.set_u32(list + 0xc, item);
}

// Translated from 00d7ef20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpAbstractMeshDefinitions::List<hkgpConvexHullImpl::Vertex, ..., PoolAllocator<Vertex,
/// 32, hkContainerHeapAllocator>>::allocate` (Xbox PDB; `this` = the vertex pool/list, one
/// stack word: the vertex to copy). Takes a free item (`list_take_item`, free list at `+0x800`
/// and use count `+0x804` of the first block; a new block through `00d7c2a0` when there is none),
/// copies the words `+0x10..+0x37` of the vertex into it (the float at `+0x2c` through the x87)
/// and links it in front of the list; answers the new item.
pub fn fn_00d7ef20(e: &mut Engine, this: Ptr, vertex: Ptr) -> u32 {
    let list = this.addr();
    let item = list_take_item(e, list, 0x800, 0x804, |e, pool| {
        hk_geometry_processing_pool_allocator_vertex_allocate_new_pool(e, Ptr::new(pool))
    });
    let source = vertex.addr();
    for word in 0..10u32 {
        let value = e.mem.u32(source + 0x10 + 4 * word);
        let value = if word == 7 {
            x87_quiet_bits(value)
        } else {
            value
        };
        e.mem.set_u32(item + 0x10 + 4 * word, value);
    }
    list_link_front(e, list, item);
    item
}

// Translated from 00d7efc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same for the triangle list (`hkgpAbstractMeshDefinitions::List<hkgpConvexHullImpl::
/// Triangle, ...>::allocate`, Xbox PDB): free list at `+0x900`, use count `+0x904`, blocks from
/// `00d7c340`; copies the 16 words `+8..+0x47` of the triangle (plain `dword` copies).
pub fn fn_00d7efc0(e: &mut Engine, this: Ptr, triangle: Ptr) -> u32 {
    let list = this.addr();
    let item = list_take_item(e, list, 0x900, 0x904, |e, pool| {
        hk_geometry_processing_pool_allocator_triangle_allocate_new_pool(e, Ptr::new(pool))
    });
    let source = triangle.addr();
    for word in 0..16u32 {
        let value = e.mem.u32(source + 8 + 4 * word);
        e.mem.set_u32(item + 8 + 4 * word, value);
    }
    list_link_front(e, list, item);
    item
}

/// Builds the support structure at `implementation + 0x60` when the word at `+0x64` is zero:
/// the vertex positions (`+0x10` of each node) are collected into a temporary `hkArray` of
/// 16-byte vectors that starts non-owning (capacity word `0x80000000`), passed to `00dedcf0`
/// with the structure, and the array buffer is released through slot `0x10` of the allocator
/// (size = capacity * 16) unless it is still non-owning.
fn build_support_if_missing(e: &mut Engine, implementation: u32) {
    if e.mem.u32(implementation + 0x64) != 0 {
        return;
    }
    let count = e.mem.i32(implementation + 0x28);
    e.with_stack(0x10, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let old_size = array_append(e, array, count, 0x10);
        let data = e.mem.u32(array);
        let mut destination = (old_size as u32).wrapping_shl(4).wrapping_add(data);
        let mut node = e.mem.u32(implementation + 0x24);
        while node != 0 {
            copy_vec(e, node + 0x10, destination);
            destination = destination.wrapping_add(0x10);
            node = e.mem.u32(node);
        }
        e.call(SUPPORT_BUILD, &args![array, implementation + 0x60]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 4]);
        }
    });
}

// Translated from 00d7c610 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, stack `(direction, min_out,
/// max_out)`). The extent of the hull along `direction`: the support structure (built first
/// when missing, as `00d76320` does, and checked again before the second query) answers the
/// point farthest along the direction (`00dea810(support, direction, out)`) and along its
/// negation (the direction xored with the sign mask at `010c44d0`). `*max_out` is
/// `dot(direction, first)`, `*min_out` `dot(direction, second)` (both summed `z + (y + x)` and
/// stored through the x87) and the answer is `*max_out - *min_out` rounded to a `float`.
pub fn fn_00d7c610(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    direction: Ptr,
    min_out: Ptr,
    max_out: Ptr,
) -> f32 {
    let implementation = this.addr();
    let original = load_vec(e, direction.addr());
    let mask = load_bits(e, SIGN_MASK);
    let negated: Vec4 =
        std::array::from_fn(|lane| f32::from_bits(original[lane].to_bits() ^ mask[lane]));
    e.with_stack(0x10, |e, opposite| {
        e.with_stack(0x10, |e, first| {
            e.with_stack(0x10, |e, second| {
                store_vec(e, opposite.addr(), negated);
                build_support_if_missing(e, implementation);
                e.call(
                    SUPPORT_QUERY,
                    &args![implementation + 0x60, direction, first],
                );
                build_support_if_missing(e, implementation);
                e.call(
                    SUPPORT_QUERY,
                    &args![implementation + 0x60, opposite, second],
                );
                let high = sum3(mul(
                    load_vec(e, direction.addr()),
                    load_vec(e, first.addr()),
                ));
                e.mem
                    .set_u32(max_out.addr(), x87_quiet_bits(high.to_bits()));
                let low = sum3(mul(
                    load_vec(e, direction.addr()),
                    load_vec(e, second.addr()),
                ));
                e.mem.set_u32(min_out.addr(), x87_quiet_bits(low.to_bits()));
                let high = e.mem.f32(max_out.addr());
                x87_quiet((fld(high) - fld(x87_quiet(low))) as f32)
            })
        })
    })
}

// Translated from 00d7c3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::clearHorizon` (Xbox PDB; `this` = the implementation, stack `(point,
/// edge, out)`; `point` three signed grid coordinates at `+0x20`, `edge` a pair `{triangle,
/// corner}`, `out` a pair to fill). Walks the triangles the point can see. A null triangle, or
/// one already stamped with the word at `+0x1ac` of the implementation (`+0x40` of the
/// triangle), ends the call; otherwise it is stamped and the exact 64-bit plane test `n2 * p2 +
/// n1 * p1 + p0 * n0 + c` (`n` the words at `+0x20..+0x28` of the triangle, `c` its 64-bit word
/// at `+0x30`) decides: negative (the point is outside) sets `out` to the edge and ends;
/// otherwise the walk recurses over the neighbours across the corner and the next two corners
/// (links at `+0x14 + 4 * corner`, tag in the low two bits), the three links are cleared on both
/// sides, the three vertices of the triangle lose a use (a vertex whose count at `+0x30`
/// reaches zero is unlinked from the vertex list at `+0x18` and released to the vertex pool,
/// `00d7adb0`) and the triangle itself is unlinked from the list at `+0x30` and released
/// (`00d7aea0`).
pub fn hkgp_convex_hull_impl_clear_horizon(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    point: Ptr,
    edge: Ptr,
    out: Ptr,
) {
    let implementation = this.addr();
    let triangle = e.mem.u32(edge.addr());
    if triangle == 0 {
        return;
    }
    let stamp = e.mem.u32(implementation + 0x1ac);
    if e.mem.u32(triangle + 0x40) == stamp {
        return;
    }
    e.mem.set_u32(triangle + 0x40, stamp);
    let triangle = e.mem.u32(edge.addr());
    let p = point.addr();
    let mut total = i64::from(e.mem.i32(triangle + 0x28)) * i64::from(e.mem.i32(p + 0x28));
    total =
        total.wrapping_add(i64::from(e.mem.i32(triangle + 0x24)) * i64::from(e.mem.i32(p + 0x24)));
    total =
        total.wrapping_add(i64::from(e.mem.i32(p + 0x20)) * i64::from(e.mem.i32(triangle + 0x20)));
    let constant =
        i64::from(e.mem.u32(triangle + 0x30)) | i64::from(e.mem.i32(triangle + 0x34)) << 32;
    total = total.wrapping_add(constant);
    if total < 0 {
        e.mem.set_u32(out.addr(), triangle);
        let corner = e.mem.u32(edge.addr() + 4);
        e.mem.set_u32(out.addr() + 4, corner);
        return;
    }
    // The links across the corner, the next corner (table 9) and the one after (table 0x12).
    let link_at = |e: &Engine, table: Option<u32>| -> u32 {
        let triangle = e.mem.u32(edge.addr());
        let corner = e.mem.u32(edge.addr() + 4);
        let index = match table {
            None => corner,
            Some(table) => (table >> ((2 * corner) & 0x1f)) & 3,
        };
        e.mem.u32(triangle + 0x14 + 4 * index)
    };
    e.with_stack(0x8, |e, pair| {
        for table in [None, Some(9u32), Some(0x12)] {
            let link = link_at(e, table);
            e.mem.set_u32(pair.addr() + 4, link & 3);
            e.mem.set_u32(pair.addr(), link & 0xffff_fffc);
            hkgp_convex_hull_impl_clear_horizon(e, this, point, pair, out);
        }
    });
    // Unlink the three neighbours on both sides.
    let triangle_now = e.mem.u32(edge.addr());
    let corner = e.mem.u32(edge.addr() + 4);
    for table in [None, Some(9u32), Some(0x12)] {
        let index = match table {
            None => corner,
            Some(table) => (table >> ((2 * corner) & 0x1f)) & 3,
        };
        let link = e.mem.u32(triangle_now + 0x14 + 4 * index);
        if link & 0xffff_fffc != 0 {
            let other = link & 0xffff_fffc;
            e.mem.set_u32(other + 0x14 + 4 * (link & 3), 0);
        }
        e.mem.set_u32(triangle_now + 0x14 + 4 * index, 0);
    }
    let triangle = e.mem.u32(edge.addr());
    let mut at = triangle + 8;
    for _ in 0..3 {
        let vertex = e.mem.u32(at);
        let uses = e.mem.u32(vertex + 0x30).wrapping_sub(1);
        e.mem.set_u32(vertex + 0x30, uses);
        if uses == 0 {
            let vertex = e.mem.u32(at);
            let next = e.mem.u32(vertex);
            let previous = e.mem.u32(vertex + 4);
            let list = implementation + 0x18;
            if next != 0 {
                e.mem.set_u32(next + 4, previous);
            }
            if previous != 0 {
                e.mem.set_u32(previous, next);
            } else {
                e.mem.set_u32(list + 0xc, next);
            }
            let count = e.mem.u32(list + 0x10);
            e.mem.set_u32(list + 0x10, count.wrapping_sub(1));
            hk_geometry_processing_pool_allocator_vertex_release(e, Ptr::new(list), vertex);
        }
        at += 4;
    }
    let next = e.mem.u32(triangle);
    let previous = e.mem.u32(triangle + 4);
    let list = implementation + 0x30;
    if next != 0 {
        e.mem.set_u32(next + 4, previous);
    }
    if previous != 0 {
        e.mem.set_u32(previous, next);
        let count = e.mem.u32(list + 0x10);
        e.mem.set_u32(list + 0x10, count.wrapping_sub(1));
    } else {
        let count = e.mem.u32(list + 0x10);
        e.mem.set_u32(list + 0x10, count.wrapping_sub(1));
        e.mem.set_u32(list + 0xc, next);
    }
    fn_00d7aea0(e, Ptr::new(list), triangle);
}

/// The length of the vector from `from` to `to` as the exe computes it: the difference
/// squared, summed `z + (y + x)`, `SQRTSS`.
fn edge_length(e: &Engine, to: u32, from: u32) -> f32 {
    let difference = sub(load_vec(e, to), load_vec(e, from));
    sum3(mul(difference, difference)).sqrt()
}

/// `total + length` through the x87 (`FLD length; FADD [total]; FSTP [total]`).
fn add_length(total: f32, length: f32) -> f32 {
    x87_quiet((fld(length) + fld(total)) as f32)
}

// Translated from 00d7cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, answers a `float` in `ST0`). The total
/// edge length of the hull by dimension: 0 (and anything above 3) answers `0.0`; 1 the distance
/// between the first two vertices (`+0x10` positions of the first node and its successor); 2 the
/// perimeter of the polygon, the positions copied into a temporary `hkArray` of 16-byte vectors
/// (non-owning at first, released through the allocator afterwards) and the lengths from each
/// element to its predecessor (the first to the last) summed; 3 the lengths of the edges of the
/// triangle list at `+0x3c`, each counted from the corner whose grid coordinates (`+0x20..`)
/// compare lexicographically below those of the next corner (or when the neighbour across the
/// edge has no link in its upper bits), skipping the edges whose triangle's word at `+0x38` is
/// not `-1` and equals the neighbour's.
pub fn fn_00d7cf00(e: &mut Engine, this: Ptr<HkgpConvexHullImpl>) -> f32 {
    let implementation = this.addr();
    let mut total = 0.0f32;
    match e.mem.u32(implementation + 0x1a8) {
        1 => {
            let node = e.mem.u32(implementation + 0x24);
            let next = e.mem.u32(node);
            x87_quiet(edge_length(e, node + 0x10, next + 0x10))
        }
        2 => {
            let count = e.mem.i32(implementation + 0x28);
            e.with_stack(0x10, |e, array| {
                let array = array.addr();
                e.mem.set_u32(array, 0);
                e.mem.set_u32(array + 4, 0);
                e.mem.set_u32(array + 8, 0x8000_0000);
                let old_size = array_append(e, array, count, 0x10);
                let data = e.mem.u32(array);
                let mut destination = (old_size as u32).wrapping_shl(4).wrapping_add(data);
                let mut node = e.mem.u32(implementation + 0x24);
                while node != 0 {
                    copy_vec(e, node + 0x10, destination);
                    destination = destination.wrapping_add(0x10);
                    node = e.mem.u32(node);
                }
                let size = e.mem.i32(array + 4);
                if size > 0 {
                    let mut current = 0u32;
                    let mut previous = (size as u32).wrapping_sub(1).wrapping_shl(4);
                    for _ in 0..size {
                        let length =
                            edge_length(e, data.wrapping_add(previous), data.wrapping_add(current));
                        total = add_length(total, length);
                        previous = current;
                        current = current.wrapping_add(0x10);
                    }
                }
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let allocator = e.mem.u32(router + 0x10);
                let capacity = e.mem.u32(array + 8);
                e.mem.set_u32(array + 4, 0);
                if (capacity as i32) >= 0 {
                    let data = e.mem.u32(array);
                    e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 4]);
                }
            });
            x87_quiet(total)
        }
        3 => {
            let mut triangle = e.mem.u32(implementation + 0x3c);
            while triangle != 0 {
                for corner in 0..3u32 {
                    let slot = triangle + 8 + 4 * corner;
                    let next_corner = (9u32 >> (2 * corner)) & 3;
                    let next_slot = triangle + 8 + 4 * next_corner;
                    let vertex = e.mem.u32(slot);
                    let next_vertex = e.mem.u32(next_slot);
                    let link = e.mem.u32(slot + 0xc);
                    // Lexicographic comparison of the grid coordinates of the two corners.
                    let mut skip = false;
                    for axis in 0..3u32 {
                        let own = e.mem.i32(vertex + 0x20 + 4 * axis);
                        let other = e.mem.i32(next_vertex + 0x20 + 4 * axis);
                        if own < other {
                            break;
                        }
                        if own > other {
                            skip = link & 0xffff_fffc != 0;
                            break;
                        }
                    }
                    if skip {
                        continue;
                    }
                    let tag = e.mem.u32(triangle + 0x38);
                    if tag != u32::MAX {
                        let neighbour = e.mem.u32(slot + 0xc) & 0xffff_fffc;
                        if tag == e.mem.u32(neighbour + 0x38) {
                            continue;
                        }
                    }
                    let vertex = e.mem.u32(slot);
                    let next_vertex = e.mem.u32(next_slot);
                    let length = edge_length(e, vertex + 0x10, next_vertex + 0x10);
                    total = add_length(total, length);
                }
                triangle = e.mem.u32(triangle);
            }
            x87_quiet(total)
        }
        _ => x87_quiet(total),
    }
}

// Translated from 00d7caa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, stack `(threshold, out)`; `out` an
/// `hkArray` of 16-byte planes). Only for three dimensions, once the indices exist (the "No
/// index available" report, `INT3` at `00d7cb26`). For each edge of each triangle of the list at
/// `+0x3c`, taken from the corner whose grid coordinates compare below the next corner's (or
/// whenever the neighbour across the edge has no link), when the plane words at `+0x38` of the
/// two triangles differ and the dot product of the two stored planes (`+0x54`, summed `z + (y +
/// x)`) is below `threshold`: the sum of the two planes is normalised (skipped when its squared
/// length is zero; `1.0 / 00ec6040(length squared)`), its offset is the largest dot product of
/// the normal with the grid positions of the vertices (the first one, and the others when the
/// byte at `+0xd` is set; a later one replaces the maximum unless it is below it) negated, and
/// the plane is appended to `out`.
pub fn fn_00d7caa0(e: &mut Engine, this: Ptr<HkgpConvexHullImpl>, threshold: f32, out: Ptr) {
    let implementation = this.addr();
    assert_indices(e, this, 0x00d7_cb26);
    if e.mem.i32(implementation + 0x1a8) != 3 {
        return;
    }
    let mut triangle = e.mem.u32(implementation + 0x3c);
    while triangle != 0 {
        for corner in 0..3u32 {
            let link = e.mem.u32(triangle + 0x14 + 4 * corner);
            let neighbour = link & 0xffff_fffc;
            let vertex = e.mem.u32(triangle + 8 + 4 * corner);
            let next_corner = (9u32 >> (2 * corner)) & 3;
            let next_vertex = e.mem.u32(triangle + 8 + 4 * next_corner);
            let mut skip = false;
            for axis in 0..3u32 {
                let own = e.mem.i32(vertex + 0x20 + 4 * axis);
                let other = e.mem.i32(next_vertex + 0x20 + 4 * axis);
                if own < other {
                    break;
                }
                if own > other {
                    skip = neighbour != 0;
                    break;
                }
            }
            if skip {
                continue;
            }
            let plane_index = e.mem.u32(triangle + 0x38);
            let neighbour_index = e.mem.u32(neighbour + 0x38);
            if plane_index == neighbour_index {
                continue;
            }
            let planes = e.mem.u32(implementation + 0x54);
            let first_plane = load_vec(e, planes.wrapping_add(plane_index.wrapping_shl(4)));
            let second_plane = load_vec(e, planes.wrapping_add(neighbour_index.wrapping_shl(4)));
            let dot = sum3(mul(first_plane, second_plane));
            if !(fld(dot) < fld(threshold)) {
                continue;
            }
            let sum = add(first_plane, second_plane);
            let length_squared = sum3(mul(sum, sum));
            if length_squared == 0.0 {
                continue;
            }
            let length = e.call(X87_SQRT, &args![fld(length_squared)]).f64() as f32;
            let inverse = (1.0 / fld(length)) as f32;
            let first = e.mem.u32(implementation + 0x24);
            let unit = mul(splat(inverse), sum);
            let position = grid_position(e, implementation, first);
            let mut extent = x87_quiet(sum3(mul(position, unit)));
            if e.mem.u8(implementation + 0xd) != 0 {
                let mut node = e.mem.u32(first);
                while node != 0 {
                    let position = grid_position(e, implementation, node);
                    let distance = sum3(mul(position, [unit[0], unit[1], unit[2], extent]));
                    if !(fld(distance) < fld(extent)) {
                        extent = x87_quiet(distance);
                    }
                    node = e.mem.u32(node);
                }
            }
            let plane = [unit[0], unit[1], unit[2], -x87_quiet(extent)];
            array_push_vector(e, out.addr(), plane, 0x10);
        }
        triangle = e.mem.u32(triangle);
    }
}

/// The hkMatrix3 and hkRotation functions the mass properties call: determinant (`this` =
/// matrix, `(out vector)`, answers `out`), cross product matrix (`this` = matrix, `(vector)`),
/// product (`this` = result, `(a, b)`), transpose and rotation renormalize (in place), `sub`
/// (`this -= argument`), an unnamed function (`this`, `(scale vector, matrix)`), the diagonalizer
/// (`this`, `(rotation, diagonal, iterations, epsilon)`, answers 1 on failure), `changeBasis`
/// and the rotation setter (`this`, `(quaternion)`).
const MATRIX3_GET_DETERMINANT: u32 = 0x00cd_51c0;
const MATRIX3_SKEW_SYMMETRIC: u32 = 0x00cd_4cc0;
const MATRIX3_SET_MUL: u32 = 0x00cd_55b0;
const MATRIX3_TRANSPOSE: u32 = 0x00cd_4b60;
const MATRIX3_SUB: u32 = 0x00cd_5180;
const MATRIX3_ADD_SCALED: u32 = 0x00cd_4d90;
const MATRIX3_DIAGONALIZE: u32 = 0x00cd_56c0;
const MATRIX3_CHANGE_BASIS: u32 = 0x00cd_5880;
const ROTATION_RENORMALIZE: u32 = 0x00cb_2fc0;
const ROTATION_SET: u32 = 0x00cb_2d90;
/// The rotation (quaternion) the degenerate volume resets the basis with.
const DEGENERATE_ROTATION: u32 = 0x0126_83b0;
/// The identity vector pair the point case copies: `0x01268370`, `0x01268380`, `0x01268390`
/// and `0x012683a0` are the rows of `RESET_MATRIX` and its translation.
const RESET_MATRIX_ROW_ONE: u32 = 0x0126_8380;
const RESET_MATRIX_ROW_TWO: u32 = 0x0126_8390;
const RESET_MATRIX_ROW_THREE: u32 = 0x0126_83a0;
/// `"Failed to diagonalize inertia matrix ("`.
const MESSAGE_DIAGONALIZE: u32 = 0x010d_36a0;
/// Doubles and floats of the mass property sums: 2.0, 6.0, 10.0, 20.0, about 1/3 (a `float`
/// widened), the float 1.0, `-0.0` and the tolerance `1e-4` (the diagonalizer epsilon, also the
/// tolerance of the normalised plane test).
const MOMENT_TWO: u32 = 0x0101_1590;
const VOLUME_DIVISOR: u32 = 0x0103_4208;
const MOMENT_TEN: u32 = 0x0102_0758;
const MOMENT_TWENTY: u32 = 0x0102_fc70;
const THIRD_DOUBLE: u32 = 0x0109_9ee0;
const ONE_FLOAT: u32 = 0x0101_2068;
const NEGATIVE_ZERO: u32 = 0x010c_4990;
const TOLERANCE_FLOAT: u32 = 0x0101_41cc;

/// The three dimensional case of `buildMassProperties` (`frame` a `0x400` byte block for the
/// matrices the hkMatrix3 calls take). Each triangle `(v0, v1, v2)` of the list at `+0x3c`
/// (grid positions less the centroid, as the rows `a, b, c` of a matrix at `frame + 0x40`)
/// adds, with `d` the determinant `00cd51c0` answers: `|(b - a) x (c - a)|` to the area sum, `d`
/// to the volume sum, `(a + b + c) * d` to the centre sum and the second moment integrals (the
/// products of the components of `a`, `b` and `c`, summed in the exe's x87 order, times `d`) to
/// the six moment sums. The centre becomes `centroid + sum / (4 * volume sum)`, the area half
/// the area sum and the volume the volume sum over 6; a volume at or below the epsilon float
/// resets the rotation and the moments. Otherwise the inertia tensor (`sum1 = m88 + m84` etc.
/// of the sums scaled by `1 / (10 * v)` and `1 / (20 * v)`, the off-diagonals negated) gets the
/// parallel axis term for the centre offset, is diagonalized (`00cd56c0`, the report "Failed to
/// diagonalize inertia matrix" with id `45465`, line `0x6d9` and `INT3` at `00d7bdc7` when it
/// answers 1), and the rotation, the diagonal and the basis changed tensor are stored.
fn mass_properties_solid(e: &mut Engine, implementation: u32, frame: u32) {
    let zero = e.mem.f32(ZERO_F32);
    let centroid_pointer = hkgp_convex_hull_impl_get_centroid(e, Ptr::new(implementation)).addr();
    let centroid = load_vec(e, centroid_pointer);
    let mut total_area = 0.0f32;
    let mut total_volume = 0.0f32;
    let seed = load_vec(e, ZERO_VECTOR_GLOBAL);
    let mut centre_sum = seed;
    let mut first_moments = seed;
    let mut second_moments = seed;
    let two = e.global::<f64>(MOMENT_TWO);
    let mut triangle = e.mem.u32(implementation + 0x3c);
    while triangle != 0 {
        let vertices = [
            e.mem.u32(triangle + 8),
            e.mem.u32(triangle + 0xc),
            e.mem.u32(triangle + 0x10),
        ];
        let rows = vertices.map(|vertex| sub(grid_position(e, implementation, vertex), centroid));
        for (index, row) in rows.iter().enumerate() {
            store_vec(e, frame + 0x40 + 0x10 * index as u32, *row);
        }
        let determinant_at = e
            .call(MATRIX3_GET_DETERMINANT, &args![frame + 0x40, frame + 0x1e0])
            .u32();
        let determinant = x87_quiet(e.mem.f32(determinant_at));
        let a = load_vec(e, frame + 0x40);
        let b = load_vec(e, frame + 0x50);
        let c = load_vec(e, frame + 0x60);
        // The edge cross product, its length and the area sum.
        let (edge_b, edge_c) = (sub(b, rows[0]), sub(c, rows[0]));
        let cross = sub(
            mul(shufps(edge_c, edge_c, 0xd2), shufps(edge_b, edge_b, 0xc9)),
            mul(shufps(edge_c, edge_c, 0xc9), shufps(edge_b, edge_b, 0xd2)),
        );
        let length = sum3(mul(cross, cross)).sqrt();
        total_area = add_length(total_area, length);
        let weighted = mul(add(add(rows[0], b), c), splat(determinant));
        centre_sum = add(weighted, centre_sum);
        total_volume = x87_quiet((fld(determinant) + fld(total_volume)) as f32);
        // The moment sums (x87, instruction order; every operand a `float` widened).
        let (ax, ay, az) = (fld(a[0]), fld(a[1]), fld(a[2]));
        let (bx, by, bz) = (fld(b[0]), fld(b[1]), fld(b[2]));
        let (cx, cy, cz) = (fld(c[0]), fld(c[1]), fld(c[2]));
        let d = fld(determinant);
        let twice = |value: f64| value + value;
        // 00d7b8b4: the sum kept at `frame + 0x80`.
        let t = cx * bx + bx * ax;
        let t = t + cx * ax;
        let t = ax * ax + t;
        let t = t + bx * bx;
        let t = t + cx * cx;
        first_moments[0] = (d * t + fld(first_moments[0])) as f32;
        // 00d7b90c: the sum at `frame + 0x90`.
        let t = cz * by + bz * ay;
        let t = t + az * cy;
        let t = t + cz * ay;
        let t = t + az * by;
        let t = t + cy * bz;
        let t = t + twice(az * ay);
        let t = t + twice(by * bz);
        let t = t + twice(cy * cz);
        second_moments[0] = (t * d + fld(second_moments[0])) as f32;
        // 00d7b980: the sum at `frame + 0x84`.
        let t = ay * by + cy * by;
        let t = t + cy * ay;
        let t = t + ay * ay;
        let t = t + by * by;
        let t = t + cy * cy;
        first_moments[1] = (t * d + fld(first_moments[1])) as f32;
        // 00d7b9de: the sum at `frame + 0x94`.
        let t = az * bx + bz * cx;
        let t = t + cz * ax;
        let t = t + az * cx;
        let t = t + bz * ax;
        let t = t + cz * bx;
        let t = t + twice(az * ax);
        let t = t + twice(bz * bx);
        let t = t + twice(cz * cx);
        second_moments[1] = (t * d + fld(second_moments[1])) as f32;
        // 00d7ba36: the sum at `frame + 0x88`.
        let t = bz * cz + az * bz;
        let t = t + cz * az;
        let t = t + az * az;
        let t = t + bz * bz;
        let t = t + cz * cz;
        first_moments[2] = (t * d + fld(first_moments[2])) as f32;
        // 00d7ba9e: the sum at `frame + 0x98`.
        let t = cy * bx + by * ax;
        let t = t + ay * cx;
        let t = t + cy * ax;
        let t = t + ay * bx;
        let t = t + by * cx;
        let t = two * (ax * ay) + t;
        let t = two * (bx * by) + t;
        let t = t + two * (cx * cy);
        second_moments[2] = (t * d + fld(second_moments[2])) as f32;
        triangle = e.mem.u32(triangle);
    }
    let volume_sum = fld(total_volume);
    let reciprocal = (1.0 / (e.global::<f64>(REFERENCE_SCALE_DOUBLE) * volume_sum)) as f32;
    let half_area = fld(total_area) * e.global::<f64>(HALF_DOUBLE);
    let new_centre = add(mul(splat(reciprocal), centre_sum), centroid);
    e.mem.set_f32(implementation + 0x1a4, half_area as f32);
    store_vec(e, implementation + 0x150, new_centre);
    let volume = (volume_sum / e.global::<f64>(VOLUME_DIVISOR)) as f32;
    e.mem.set_f32(implementation + 0x1a0, volume);
    if !(fld(volume) > f64::from(e.mem.f32(PLANE_EPSILON))) {
        e.call(
            ROTATION_SET,
            &args![implementation + 0x120, DEGENERATE_ROTATION],
        );
        e.mem.set_f32(implementation + 0x1a0, 0.0);
        copy_vec(e, ZERO_VECTOR_GLOBAL, implementation + 0x170);
        copy_vec(e, ZERO_VECTOR_GLOBAL, implementation + 0x180);
        copy_vec(e, ZERO_VECTOR_GLOBAL, implementation + 0x190);
        e.mem.set_u8(implementation + 0x1b4, 1);
        copy_vec(e, ZERO_VECTOR_GLOBAL, implementation + 0x160);
        return;
    }
    let delta = sub(new_centre, centroid);
    let tenth = (1.0 / (volume_sum * e.global::<f64>(MOMENT_TEN))) as f32;
    let twentieth = (1.0 / (volume_sum * e.global::<f64>(MOMENT_TWENTY))) as f32;
    let first = mul(splat(tenth), first_moments);
    let (m88, m84, m80) = (fld(first[2]), fld(first[1]), fld(first[0]));
    let second = mul(splat(twentieth), second_moments);
    let negative_zero = e.mem.f32(NEGATIVE_ZERO);
    let product_xy = negative_zero - second[2];
    let sum_one = (m88 + m84) as f32;
    let product_xz = negative_zero - second[1];
    let sum_two = (m88 + m80) as f32;
    let sum_three = (m80 + m84) as f32;
    let product_yz = negative_zero - second[0];
    // The inertia tensor, a symmetric matrix at `frame + 0x40`.
    store_vec(e, frame + 0x40, [sum_one, product_xy, product_xz, zero]);
    store_vec(e, frame + 0x50, [product_xy, sum_two, product_yz, zero]);
    store_vec(e, frame + 0x60, [product_xz, product_yz, sum_three, zero]);
    store_vec(e, frame + 0xc0, delta);
    e.call(MATRIX3_SKEW_SYMMETRIC, &args![frame + 0x180, frame + 0xc0]);
    e.call(
        MATRIX3_SKEW_SYMMETRIC,
        &args![frame + 0x120, ZERO_VECTOR_GLOBAL],
    );
    e.call(
        MATRIX3_SET_MUL,
        &args![frame + 0x1b0, frame + 0x180, frame + 0x180],
    );
    e.call(
        MATRIX3_SET_MUL,
        &args![frame + 0x150, frame + 0x120, frame + 0x120],
    );
    e.call(MATRIX3_TRANSPOSE, &args![frame + 0x150]);
    e.call(MATRIX3_SUB, &args![frame + 0x1b0, frame + 0x150]);
    let one = e.mem.f32(ONE_FLOAT);
    store_vec(e, frame + 0xa0, [one, 0.0, 0.0, 0.0]);
    e.call(
        MATRIX3_ADD_SCALED,
        &args![frame + 0x40, frame + 0xa0, frame + 0x1b0],
    );
    let epsilon = x87_quiet_bits(e.mem.u32(TOLERANCE_FLOAT));
    let status = e
        .call(
            MATRIX3_DIAGONALIZE,
            &args![frame + 0x40, frame + 0xf0, frame + 0xd0, 0x80u32, epsilon],
        )
        .u32();
    if status == 1 {
        report_with_number(
            e,
            MESSAGE_DIAGONALIZE,
            implementation + 0x1b0,
            0x45465,
            0x6d9,
            0x00d7_bdc7,
        );
    }
    e.call(MATRIX3_TRANSPOSE, &args![frame + 0xf0]);
    e.call(ROTATION_RENORMALIZE, &args![frame + 0xf0]);
    copy_vec(e, frame + 0x40, implementation + 0x170);
    copy_vec(e, frame + 0x50, implementation + 0x180);
    copy_vec(e, frame + 0x60, implementation + 0x190);
    e.call(MATRIX3_CHANGE_BASIS, &args![frame + 0x40, frame + 0xf0]);
    let diagonal = [
        e.mem.f32(frame + 0x40),
        e.mem.f32(frame + 0x54),
        e.mem.f32(frame + 0x68),
        zero,
    ];
    store_vec(e, implementation + 0x160, diagonal);
    e.call(MATRIX3_TRANSPOSE, &args![frame + 0xf0]);
    copy_vec(e, frame + 0xf0, implementation + 0x120);
    copy_vec(e, frame + 0x100, implementation + 0x130);
    copy_vec(e, frame + 0x110, implementation + 0x140);
    e.mem.set_u8(implementation + 0x1b4, 1);
}

/// The two dimensional case of `buildMassProperties`: the grid positions of the vertices are
/// collected into a temporary non-owning `hkArray` of vectors, and for each triangle fan `(p0,
/// p[i + 1], p[i + 2])` the area `|((p[i + 1] - p0) x (p[i + 2] - p0)) . normal|` (the normal
/// at `+0xe0`) is added to the sum and `area / 3` (the float third widened) times the sum of the
/// two edge vectors to the centre sum. The area is half the sum, `+0x170` becomes `1.0f`,
/// `+0x160` `(1.0, 0, 0, 0)` and the centre (`+0x150`) `p0 + sum / area sum`.
fn mass_properties_polygon(e: &mut Engine, implementation: u32) {
    let count = e.mem.i32(implementation + 0x28);
    let seed = load_vec(e, ZERO_VECTOR_GLOBAL);
    e.with_stack(0x10, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let old_size = array_append(e, array, count, 0x10);
        let data = e.mem.u32(array);
        let mut destination = (old_size as u32).wrapping_shl(4).wrapping_add(data);
        let mut node = e.mem.u32(implementation + 0x24);
        while node != 0 {
            let position = grid_position(e, implementation, node);
            store_vec(e, destination, position);
            destination = destination.wrapping_add(0x10);
            node = e.mem.u32(node);
        }
        let zero = e.mem.f32(ZERO_F32);
        let size = e.mem.i32(array + 4);
        let mut total = 0.0f32;
        let mut sum = seed;
        if size.wrapping_sub(1) > 1 {
            let origin = load_vec(e, data);
            let normal = load_vec(e, implementation + 0xe0);
            let third = e.global::<f64>(THIRD_DOUBLE);
            for index in 1..size.wrapping_sub(1) {
                let at = data.wrapping_add((index as u32).wrapping_shl(4));
                let first = sub(load_vec(e, at), origin);
                let second = sub(load_vec(e, at.wrapping_add(0x10)), origin);
                let cross = sub(
                    mul(shufps(second, second, 0xd2), shufps(first, first, 0xc9)),
                    mul(shufps(second, second, 0xc9), shufps(first, first, 0xd2)),
                );
                let area = fld(sum3(mul(cross, normal))).abs() as f32;
                let both = add(second, first);
                total = x87_quiet((fld(area) + fld(total)) as f32);
                let weight = (fld(area) * third) as f32;
                sum = add(mul(splat(weight), both), sum);
            }
        }
        e.mem.set_u8(implementation + 0x1b4, 1);
        let half = fld(total) * e.global::<f64>(HALF_DOUBLE);
        e.mem.set_f32(implementation + 0x1a4, half as f32);
        e.mem.set_u32(implementation + 0x170, 0x3f80_0000);
        let reciprocal = (1.0 / fld(total)) as f32;
        let one = e.mem.f32(ONE_FLOAT);
        store_vec(e, implementation + 0x160, [one, zero, zero, zero]);
        let origin = load_vec(e, data);
        store_vec(
            e,
            implementation + 0x150,
            add(mul(splat(reciprocal), sum), origin),
        );
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 4]);
        }
    });
}

/// The remaining dimensions (0, 1 and anything above 3) of `buildMassProperties`: zero area and
/// volume, the identity matrix (`hkMatrix3::getIdentity`) as the three rows at `+0x170`, the zero
/// vector at `+0x160` and the reset matrix (four vectors at `01268370`) at `+0x120`.
fn mass_properties_point(e: &mut Engine, implementation: u32) {
    e.mem.set_f32(implementation + 0x1a4, 0.0);
    e.mem.set_u8(implementation + 0x1b4, 1);
    e.mem.set_f32(implementation + 0x1a0, 0.0);
    let identity = e.call(MATRIX3_GET_IDENTITY, &args![]).u32();
    copy_vec(e, identity, implementation + 0x170);
    copy_vec(e, identity + 0x10, implementation + 0x180);
    copy_vec(e, identity + 0x20, implementation + 0x190);
    copy_vec(e, ZERO_VECTOR_GLOBAL, implementation + 0x160);
    copy_vec(e, RESET_MATRIX, implementation + 0x120);
    copy_vec(e, RESET_MATRIX_ROW_ONE, implementation + 0x130);
    copy_vec(e, RESET_MATRIX_ROW_TWO, implementation + 0x140);
    copy_vec(e, RESET_MATRIX_ROW_THREE, implementation + 0x150);
}

// Translated from 00d7b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::buildMassProperties<hkgpConvexHullImpl::PositionAsTransform>` (Xbox PDB;
/// `this` = the implementation). Does nothing when the mass properties exist (`+0x1b4`);
/// otherwise builds them by dimension (`+0x1a8`: 3 `mass_properties_solid`, 2
/// `mass_properties_polygon`, else `mass_properties_point`), sets the flag, and answers whether
/// the flag is still clear (always `false`).
pub fn hkgp_convex_hull_impl_build_mass_properties(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
) -> bool {
    let implementation = this.addr();
    if e.mem.u8(implementation + 0x1b4) == 0 {
        match e.mem.i32(implementation + 0x1a8) {
            3 => e.with_stack(0x400, |e, frame| {
                mass_properties_solid(e, implementation, frame.addr())
            }),
            2 => mass_properties_polygon(e, implementation),
            _ => mass_properties_point(e, implementation),
        }
    }
    e.mem.u8(implementation + 0x1b4) == 0
}

/// `InterlockedExchangeAdd` (the import at `00fdf1ec`: `(address, value)`, answers the old
/// value) and the counter at `0126914c` that numbers the implementations.
const INTERLOCKED_EXCHANGE_ADD: u32 = 0x00fd_f1ec;
const IMPLEMENTATION_COUNTER: u32 = 0x0126_914c;
/// `hkMapBase<unsigned long, unsigned long>::findKey` followed by `remove` (`00cb1fb0`: `this` =
/// map, `(key)`; answers 0 when the key was removed, 1 when it was not found).
const MAP_REMOVE_KEY: u32 = 0x00cb_1fb0;

// Translated from 00d7f6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::hkgpConvexHullImpl` (Xbox PDB; `this` = the `0x1c0` byte block, answers
/// it). Builds the build configuration (`00d74d40`), sets the embedded mesh's vtable
/// (`010d36cc`, `+0x10`) and its word `+0x16` to 1, clears the vertex pool (`+0x18`, the list
/// head `+0x24` and count `+0x28`), the triangle pool (`+0x30`, `+0x3c`, `+0x40`), the item
/// array (`+0x48`), the plane array (`+0x54`) and the support array (`+0x60`) with their
/// capacity words set to `0x80000000` (non-owning), takes the next number of the counter at
/// `0126914c` for `+0x1b0` and resets the hull (`00d7ac40`, with 0).
pub fn hkgp_convex_hull_impl_construct(e: &mut Engine, this: Ptr) -> Ptr<HkgpConvexHullImpl> {
    let at = this.addr();
    hkgp_convex_hull_build_config_build_config(e, this);
    e.mem.set_u32(at + 0x10, MESH_VTABLE);
    e.mem.set_u16(at + 0x16, 1);
    for (offset, value) in [
        (0x18, 0),
        (0x1c, 0),
        (0x24, 0),
        (0x28, 0),
        (0x20, 0x8000_0000),
        (0x30, 0),
        (0x34, 0),
        (0x38, 0x8000_0000),
        (0x3c, 0),
        (0x40, 0),
        (0x48, 0),
        (0x4c, 0),
        (0x50, 0x8000_0000),
        (0x54, 0),
        (0x58, 0),
        (0x5c, 0x8000_0000),
        (0x60, 0),
        (0x64, 0),
        (0x68, 0x8000_0000),
    ] {
        e.mem.set_u32(at + offset, value);
    }
    let number = e
        .call(
            INTERLOCKED_EXCHANGE_ADD,
            &args![IMPLEMENTATION_COUNTER, 1u32],
        )
        .u32();
    e.mem.set_u32(at + 0x1b0, number);
    hkgp_convex_hull_impl_reset(e, Ptr::new(at), 0);
    Ptr::new(at)
}

/// Reserves the `hkArray` `array` (cdecl `00c908c0`) as the exe inlines it before a copy: when
/// the capacity is below `count` it grows to twice the capacity or `count`, whichever is
/// greater. The allocator is the router's `+0x10`.
fn array_reserve_for(e: &mut Engine, array: u32, count: i32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let target = if count >= doubled { count } else { doubled };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, target, element_size],
        );
    }
}

/// The last node of the list that starts at `first` (following the `+0` links), or 0.
fn list_last(e: &Engine, first: u32) -> u32 {
    let mut node = first;
    if node == 0 {
        return 0;
    }
    loop {
        let next = e.mem.u32(node);
        if next == 0 {
            return node;
        }
        node = next;
    }
}

/// An empty `hkMap` (`{0, 0, 0xffffffff}`) on the stack, reserved for no entries as the exe does
/// first.
fn map_start(e: &mut Engine, map: u32) {
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
}

/// Reserves room for `count` entries (the allocator read from the router again).
fn map_reserve(e: &mut Engine, map: u32, count: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_RESERVE, &args![map, allocator, count]);
}

/// `clearAndDeallocate` of the map and the call `00a29680` that follows it (no effect).
fn map_finish(e: &mut Engine, map: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(LOCAL_NO_EFFECT, &args![map]);
}

// Translated from 00d7f720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::clone` (Xbox PDB; `this` = the source, answers the copy). A `0x1c0`
/// byte block comes from allocator slot 4 and is constructed (`00d7f6a0`; a null block is not
/// checked afterwards, as in the exe). Copied: the build configuration words `+0..+0xc`, the
/// vectors `+0xd0..+0x160`, the floats `+0x1a0` and `+0x1a4` (through the x87), the words
/// `+0x1a8` and `+0x1ac` and the bytes `+0x1b4` and `+0x1b5`; the array of `0x40` byte items
/// (`+0x48`: the vector at `+0x10`, three words, a float at `+0x2c`, two words; items the array
/// grows by start with `0.0` at `+0x2c` and `-1` at `+0x34`) and the array of planes (`+0x54`).
/// The vertices are then copied from the last to the first (`00d7ef20` into the pool at `+0x18`,
/// recorded in a map from source to copy) and the triangles likewise (`00d7efc0` into the pool at
/// `+0x30`): their vertex pointers are mapped (default 0) and each of the three links is looked up
/// by its value in a second map: a found entry (the neighbour's copy, `{pointer, corner}` as
/// pointer plus corner) is removed (`00cb1fb0`) and linked both ways, otherwise this triangle's
/// edge (source plus corner to copy plus corner) is recorded for the neighbour.
pub fn hkgp_convex_hull_impl_clone(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
) -> Ptr<HkgpConvexHullImpl> {
    let source = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x1c0u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        hkgp_convex_hull_impl_construct(e, Ptr::new(block)).addr()
    };
    for offset in [0, 4, 8, 0xc] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(copy + offset, word);
    }
    for offset in (0xd0..=0x160).step_by(0x10) {
        copy_vec(e, source + offset, copy + offset);
    }
    copy_through_x87(e, copy + 0x1a0, source + 0x1a0);
    copy_through_x87(e, copy + 0x1a4, source + 0x1a4);
    let dimensions = e.mem.u32(source + 0x1a8);
    e.mem.set_u32(copy + 0x1a8, dimensions);
    let stamp = e.mem.u32(source + 0x1ac);
    e.mem.set_u32(copy + 0x1ac, stamp);
    let valid = e.mem.u8(source + 0x1b4);
    e.mem.set_u8(copy + 0x1b4, valid);
    let valid = e.mem.u8(source + 0x1b5);
    e.mem.set_u8(copy + 0x1b5, valid);
    // The array of 0x40 byte items.
    let count = e.mem.i32(source + 0x4c);
    array_reserve_for(e, copy + 0x48, count, 0x40);
    let size = e.mem.i32(copy + 0x4c);
    let mut item = (size as u32)
        .wrapping_shl(6)
        .wrapping_add(e.mem.u32(copy + 0x48));
    for _ in 0..count.wrapping_sub(size).max(0) {
        if item != 0 {
            e.mem.set_u32(item + 0x2c, 0);
            e.mem.set_u32(item + 0x34, 0xffff_ffff);
        }
        item = item.wrapping_add(0x40);
    }
    e.mem.set_i32(copy + 0x4c, count);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(source + 0x4c) {
        let from = e.mem.u32(source + 0x48).wrapping_add(index << 6);
        let to = e.mem.u32(copy + 0x48).wrapping_add(index << 6);
        copy_vec(e, from + 0x10, to + 0x10);
        for offset in [0x20, 0x24, 0x28] {
            let word = e.mem.u32(from + offset);
            e.mem.set_u32(to + offset, word);
        }
        copy_through_x87(e, to + 0x2c, from + 0x2c);
        for offset in [0x30, 0x34] {
            let word = e.mem.u32(from + offset);
            e.mem.set_u32(to + offset, word);
        }
        index += 1;
    }
    // The array of planes.
    let count = e.mem.i32(source + 0x58);
    array_reserve_for(e, copy + 0x54, count, 0x10);
    e.mem.set_i32(copy + 0x58, count);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(source + 0x58) {
        let from = e.mem.u32(source + 0x54).wrapping_add(index << 4);
        let to = e.mem.u32(copy + 0x54).wrapping_add(index << 4);
        copy_vec(e, from, to);
        index += 1;
    }
    e.with_stack(0x10, |e, vertex_map| {
        let vertex_map = vertex_map.addr();
        map_start(e, vertex_map);
        let vertex_count = e.mem.u32(source + 0x28);
        if vertex_count != 0 {
            map_reserve(e, vertex_map, vertex_count);
            let mut vertex = list_last(e, e.mem.u32(source + 0x24));
            while vertex != 0 {
                let new_vertex = fn_00d7ef20(e, Ptr::new(copy + 0x18), Ptr::new(vertex));
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let allocator = e.mem.u32(router + 0x10);
                e.call(
                    MAP_INSERT,
                    &args![vertex_map, allocator, vertex, new_vertex],
                );
                vertex = e.mem.u32(vertex + 4);
            }
        }
        if e.mem.u32(source + 0x40) != 0 {
            e.with_stack(0x10, |e, edge_map| {
                let edge_map = edge_map.addr();
                map_start(e, edge_map);
                let triangle_count = e.mem.u32(source + 0x40);
                map_reserve(e, edge_map, triangle_count);
                let mut triangle = list_last(e, e.mem.u32(source + 0x3c));
                while triangle != 0 {
                    let new_triangle = fn_00d7efc0(e, Ptr::new(copy + 0x30), Ptr::new(triangle));
                    for corner in 0..3u32 {
                        let old_vertex = e.mem.u32(new_triangle + 8 + 4 * corner);
                        let mapped = e
                            .call(MAP_GET_WITH_DEFAULT, &args![vertex_map, old_vertex, 0u32])
                            .u32();
                        e.mem.set_u32(new_triangle + 8 + 4 * corner, mapped);
                    }
                    let source_offset = triangle.wrapping_sub(new_triangle);
                    for corner in 0..3u32 {
                        let link_at = new_triangle + 0x14 + 4 * corner;
                        let link = e.mem.u32(link_at);
                        let key = (link & 3).wrapping_add(link & 0xffff_fffc);
                        let found = e
                            .call(MAP_GET_WITH_DEFAULT, &args![edge_map, key, 0u32])
                            .u32();
                        let tag = found & 3;
                        let neighbour = found & 0xffff_fffc;
                        let new_edge = new_triangle.wrapping_add(corner);
                        if neighbour != 0 {
                            let key = (e.mem.u32(link_at) & 3)
                                .wrapping_add(e.mem.u32(link_at) & 0xffff_fffc);
                            e.call(MAP_REMOVE_KEY, &args![edge_map, key]);
                            e.mem.set_u32(link_at, tag.wrapping_add(neighbour));
                            e.mem.set_u32(neighbour + 0x14 + 4 * tag, new_edge);
                        } else {
                            let router = e.call(MEMORY_ROUTER, &args![]).u32();
                            let allocator = e.mem.u32(router + 0x10);
                            let key = source_offset.wrapping_add(new_edge);
                            e.call(MAP_INSERT, &args![edge_map, allocator, key, new_edge]);
                        }
                    }
                    triangle = e.mem.u32(triangle + 4);
                }
                map_finish(e, edge_map);
            });
        }
        map_finish(e, vertex_map);
    });
    Ptr::new(copy)
}

/// The cross product the exe computes with two `SHUFPS` pairs: `a.yzx * b.zxy - a.zxy * b.yzx`
/// (the cross product `a x b`, the fourth lane `a.w * b.w - a.w * b.w`).
fn cross_yzx(a: Vec4, b: Vec4) -> Vec4 {
    sub(
        mul(shufps(a, a, 0xc9), shufps(b, b, 0xd2)),
        mul(shufps(a, a, 0xd2), shufps(b, b, 0xc9)),
    )
}

/// The normalising step the rectangle search repeats for a vector whose squared length
/// `length_squared` is not exactly zero: the CRT root (`00ec6040`), `1.0 / length` rounded to a
/// `float`, and the scaled vector.
fn normalise_with_root(e: &mut Engine, vector: Vec4, length_squared: f32) -> Vec4 {
    let length = e.call(X87_SQRT, &args![fld(length_squared)]).f64() as f32;
    let inverse = (1.0 / fld(length)) as f32;
    mul(splat(inverse), vector)
}

// Translated from 00d7f090 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the implementation, stack `(direction, size_out,
/// frame_out)`; answers the best product in `ST0`). Searches the oriented rectangle that bounds
/// the hull seen along `direction` (the "No index available" report, `INT3` at `00d7f112`,
/// applies first). In three dimensions every edge of the hull (the edge cursor `00d7d220` /
/// `00d7c970`) whose two planes (`+0x54`) have dot products with `direction` whose product is not
/// above zero gives the axes `e = normalise(direction x (plane1 x plane2))` and `f =
/// normalise(e x direction)`; in two dimensions every plane `p` gives the axes `p` and `g =
/// normalise(direction x p)`. A candidate whose extents along its two axes (`00d7c610`, the width
/// of the hull along each) multiply to less than the best so far (which starts at the float at
/// `0104da48`) becomes the best, together with the axes, the extents and the centres of the two
/// ranges. The answer fills `frame_out` with the two axes, `direction` and `centre1 * axis1 +
/// centre2 * axis2 - direction.w * direction`, and `size_out` with half of each extent and two
/// zeros.
pub fn fn_00d7f090(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    direction: Ptr,
    size_out: Ptr,
    frame_out: Ptr,
) -> f32 {
    let implementation = this.addr();
    assert_indices(e, this, 0x00d7_f112);
    let start = x87_quiet(e.mem.f32(SEARCH_START_NEAREST));
    let seed = load_vec(e, ZERO_VECTOR_GLOBAL);
    let mut best = start;
    let mut extent_one = start;
    let mut extent_two = start;
    let mut centre_one = start;
    let mut centre_two = start;
    let mut axis_one = seed;
    let mut axis_two = seed;
    let dimensions = e.mem.i32(implementation + 0x1a8);
    // The scratch block of the exe: the floats of the second range at `+0x1c` and `+0x20`, those
    // of the first range at `+0x38` and `+0x3c`, the cursor (or plane) at `+0x40`, the first
    // axis at `+0x50` and the second at `+0x90`.
    e.with_stack(0x100, |e, frame| {
        let frame = frame.addr();
        let (low_two, high_two) = (frame + 0x1c, frame + 0x20);
        let (low_one, high_one) = (frame + 0x38, frame + 0x3c);
        let cursor = frame + 0x40;
        let first_axis = frame + 0x50;
        let second_axis = frame + 0x90;
        let half: f64 = e.global(HALF_DOUBLE);
        if dimensions == 3 {
            let first_triangle = e.mem.u32(implementation + 0x3c);
            fn_00d7d220(e, Ptr::new(cursor), first_triangle, 0);
            while e.mem.u32(cursor) != 0 {
                let triangle = e.mem.u32(cursor);
                let corner = e.mem.u32(cursor + 4);
                let link = e
                    .mem
                    .u32(triangle + corner.wrapping_mul(4).wrapping_add(0x14));
                let plane_index = e.mem.u32(triangle + 0x38);
                let neighbour_index = e.mem.u32((link & 0xffff_fffc).wrapping_add(0x38));
                if plane_index != neighbour_index {
                    let planes = e.mem.u32(implementation + 0x54);
                    let plane_n = load_vec(e, planes.wrapping_add(neighbour_index.wrapping_shl(4)));
                    let direction_vector = load_vec(e, direction.addr());
                    let plane_t = load_vec(e, planes.wrapping_add(plane_index.wrapping_shl(4)));
                    let dot_n = sum3(mul(direction_vector, plane_n));
                    let dot_t = sum3(mul(plane_t, direction_vector));
                    let product = dot_t * dot_n;
                    if product <= 0.0 {
                        let across = cross_yzx(plane_t, plane_n);
                        let first = cross_yzx(direction_vector, across);
                        let length_squared = sum3(mul(first, first));
                        if length_squared != 0.0 {
                            let first = normalise_with_root(e, first, length_squared);
                            let second = cross_yzx(first, direction_vector);
                            store_vec(e, first_axis, first);
                            let length_squared = sum3(mul(second, second));
                            if length_squared != 0.0 {
                                let second = normalise_with_root(e, second, length_squared);
                                store_vec(e, second_axis, second);
                                let width_one = fn_00d7c610(
                                    e,
                                    this,
                                    Ptr::new(first_axis),
                                    Ptr::new(low_one),
                                    Ptr::new(high_one),
                                );
                                let width_two = fn_00d7c610(
                                    e,
                                    this,
                                    Ptr::new(second_axis),
                                    Ptr::new(low_two),
                                    Ptr::new(high_two),
                                );
                                let area = (fld(width_two) * fld(width_one)) as f32;
                                if fld(area) < fld(best) {
                                    best = area;
                                    axis_one = load_vec(e, first_axis);
                                    extent_one = width_one;
                                    axis_two = load_vec(e, second_axis);
                                    extent_two = width_two;
                                    let sum = fld(e.mem.f32(high_one)) + fld(e.mem.f32(low_one));
                                    centre_one = (half * sum) as f32;
                                    let sum = fld(e.mem.f32(high_two)) + fld(e.mem.f32(low_two));
                                    centre_two = (half * sum) as f32;
                                }
                            }
                        }
                    }
                }
                fn_00d7c970(e, Ptr::new(cursor));
            }
        } else if dimensions == 2 && e.mem.i32(implementation + 0x58) > 0 {
            let mut index = 0;
            while index < e.mem.i32(implementation + 0x58) {
                let plane_at = e
                    .mem
                    .u32(implementation + 0x54)
                    .wrapping_add((index as u32) << 4);
                copy_vec(e, plane_at, cursor);
                let direction_vector = load_vec(e, direction.addr());
                let plane = load_vec(e, cursor);
                let across = cross_yzx(direction_vector, plane);
                let length_squared = sum3(mul(across, across));
                if length_squared != 0.0 {
                    let across = normalise_with_root(e, across, length_squared);
                    store_vec(e, first_axis, across);
                    let width_plane = fn_00d7c610(
                        e,
                        this,
                        Ptr::new(cursor),
                        Ptr::new(low_two),
                        Ptr::new(high_two),
                    );
                    let width_across = fn_00d7c610(
                        e,
                        this,
                        Ptr::new(first_axis),
                        Ptr::new(low_one),
                        Ptr::new(high_one),
                    );
                    let area = (fld(width_across) * fld(width_plane)) as f32;
                    if fld(area) < fld(best) {
                        best = area;
                        axis_one = load_vec(e, cursor);
                        extent_one = width_plane;
                        axis_two = load_vec(e, first_axis);
                        extent_two = width_across;
                        let sum = fld(e.mem.f32(high_two)) + fld(e.mem.f32(low_two));
                        centre_one = (half * sum) as f32;
                        let sum = fld(e.mem.f32(high_one)) + fld(e.mem.f32(low_one));
                        centre_two = (half * sum) as f32;
                    }
                }
                index += 1;
            }
        }
    });
    let half: f64 = e.global(HALF_DOUBLE);
    let half_one = (half * fld(extent_one)) as f32;
    let half_two = (half * fld(extent_two)) as f32;
    let direction_vector = load_vec(e, direction.addr());
    let frame = frame_out.addr();
    let offset = add(
        mul(splat(centre_two), axis_two),
        mul(splat(centre_one), axis_one),
    );
    let offset = sub(offset, mul(splat(direction_vector[3]), direction_vector));
    store_vec(e, frame + 0x30, offset);
    store_vec(e, frame + 0x10, axis_two);
    store_vec(e, frame, axis_one);
    store_vec(e, frame + 0x20, direction_vector);
    let zero = e.mem.f32(ZERO_F32);
    store_vec(e, size_out.addr(), [half_one, half_two, zero, zero]);
    x87_quiet(best)
}

// Translated from 00d81340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::getOrientedRectangle` (Xbox PDB; `this` = the implementation, stack
/// `(size_out, frame_out)`: the sizes (a vector) and the frame (four vectors) of the rectangle
/// or box). Fewer than three dimensions: the search of `00d7f090` along the normal at `+0xe0`.
/// Otherwise (the "No index available" report, `INT3` at `00d813ec`, applies first) every plane
/// (`+0x54`) is tried as the direction of the third axis: the width of the hull along it
/// (`00d7c610`) must be below the best so far (starting at the float at `0104da48`), and then the
/// rectangle search along the plane (`00d7f090`) gives a volume `(max - min) * product`, which
/// must be below the best volume, and a candidate that passes replaces the answer (the sizes
/// with their third value set to the width, the four frame vectors). At the end the third size is
/// halved and the frame origin moves by that half width along the third axis (`origin = origin
/// - half * axis`).
pub fn hkgp_convex_hull_impl_get_oriented_rectangle(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    size_out: Ptr,
    frame_out: Ptr,
) {
    let implementation = this.addr();
    if e.mem.i32(implementation + 0x1a8) < 3 {
        fn_00d7f090(
            e,
            this,
            Ptr::new(implementation + 0xe0),
            size_out,
            frame_out,
        );
        return;
    }
    assert_indices(e, this, 0x00d8_13ec);
    let start = x87_quiet(e.mem.f32(SEARCH_START_NEAREST));
    let mut best_width = start;
    let mut best_volume = start;
    // The scratch block: the range at `+0x18` and `+0x1c`, the sizes found at `+0x30` and the
    // frame found at `+0x40`.
    e.with_stack(0x100, |e, scratch| {
        let scratch = scratch.addr();
        let (low, high) = (scratch + 0x18, scratch + 0x1c);
        let (sizes, frame) = (scratch + 0x30, scratch + 0x40);
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(implementation + 0x58) {
            let plane = e.mem.u32(implementation + 0x54).wrapping_add(index << 4);
            let width = fn_00d7c610(e, this, Ptr::new(plane), Ptr::new(low), Ptr::new(high));
            if fld(width) < fld(best_width) {
                let plane = e.mem.u32(implementation + 0x54).wrapping_add(index << 4);
                let product =
                    fn_00d7f090(e, this, Ptr::new(plane), Ptr::new(sizes), Ptr::new(frame));
                let volume = ((fld(e.mem.f32(high)) - fld(e.mem.f32(low))) * fld(product)) as f32;
                if fld(volume) < fld(best_volume) {
                    best_width = width;
                    copy_vec(e, sizes, size_out.addr());
                    best_volume = volume;
                    e.mem.set_f32(size_out.addr() + 8, width);
                    for offset in [0u32, 0x10, 0x20, 0x30] {
                        copy_vec(e, frame + offset, frame_out.addr() + offset);
                    }
                }
            }
            index += 1;
        }
    });
    let half: f64 = e.global(HALF_DOUBLE);
    let third = (fld(e.mem.f32(size_out.addr() + 8)) * half) as f32;
    e.mem.set_f32(size_out.addr() + 8, third);
    let negated = -x87_quiet(e.mem.f32(size_out.addr() + 8));
    let axis = load_vec(e, frame_out.addr() + 0x20);
    let origin = load_vec(e, frame_out.addr() + 0x30);
    store_vec(
        e,
        frame_out.addr() + 0x30,
        add(mul(splat(negated), axis), origin),
    );
}

/// `hkVector4::isNormalized3` (`00c7f520`: `this` = the vector, `(byte out, float tolerance)`,
/// answers the address of the byte it wrote) and the text `"Invalid dimemsions for that
/// operation"`.
const VECTOR4_IS_NORMALIZED3: u32 = 0x00c7_f520;
const MESSAGE_INVALID_DIMENSIONS: u32 = 0x010d_3748;

/// Builds the hull from the vectors in the `hkArray` `array` (the strided source `{data, count,
/// stride 0x10}` is made in `source`) with the configuration copy `config`; answers what the
/// hull's second `build` (`00d77200`) answers.
fn build_from_array(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    array: u32,
    source: u32,
    config: u32,
) -> i32 {
    let data = e.mem.u32(array);
    let count = e.mem.u32(array + 4);
    e.mem.set_u32(source, data);
    e.mem.set_u32(source + 4, count);
    e.mem.set_u32(source + 8, 0x10);
    hkgp_convex_hull_build_ov2(e, this, Ptr::new(source), Ptr::new(config))
}

// Translated from 00d77be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHull::expandByPlanarMargin` (Xbox PDB; `this` = the hull, stack `(margin)`,
/// answers a byte: whether the hull is three dimensional afterwards). Rebuilds the hull from
/// points moved out by the margin along its planes, by dimension (`+0x1a8`): 0 six points
/// `centre +- margin / 2` along the axes; 1 the two end points and eight more `p +- w`, `p +- h`
/// (`w` and `h` perpendicular to the segment and to each other, normalised by the refined
/// `RSQRTSS` and scaled by `margin / 2`); 2 for each vertex `p +- margin * normal` (when
/// that does not make a solid hull the hull is rebuilt from its centre point and the call
/// repeats; the answer is the repeat's); 3 for each triangle whose plane is normalised (`1e-4`) the
/// three vertices moved by `margin * plane`. Other dimension counts report "Invalid dimemsions
/// for that operation" (id `172cdf48`, line `0xa77`, `INT3` at `00d786b5`). The configuration
/// words (`+0..+0xc`) are the `build` parameters; afterwards, for a solid hull, the indices
/// (`00d76990` with the float at `0106ea1c` and 0) and the mass properties (`00d762f0`) are
/// rebuilt when they existed before.
pub fn hkgp_convex_hull_expand_by_planar_margin(
    e: &mut Engine,
    this: Ptr<HkgpConvexHull>,
    margin: f32,
) -> bool {
    let implementation = e.get(this, HkgpConvexHull::implementation).addr();
    let half = (fld(margin) * e.global::<f64>(HALF_DOUBLE)) as f32;
    let mass_valid = e.mem.u8(implementation + 0x1b4);
    let indices_valid = e.mem.u8(implementation + 0x1b5);
    let dimensions = hkgp_convex_hull_get_dimensions(e, this);
    // The scratch block of the exe: the temporary array at `+0x18`, the strided source at
    // `+0x30` and the copy of the build configuration at `+0x90`.
    let finished = e.with_stack(0x100, |e, frame| {
        let frame = frame.addr();
        let (array, source, config) = (frame + 0x18, frame + 0x30, frame + 0x90);
        for word in 0..4 {
            let value = e.mem.u32(implementation + 4 * word);
            e.mem.set_u32(config + 4 * word, value);
        }
        let fresh_array = |e: &mut Engine| {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        };
        match dimensions as u32 {
            0 => {
                fresh_array(e);
                let node = e.mem.u32(implementation + 0x24);
                let point = load_vec(e, node + 0x10);
                array_reserve_for(e, array, 6, 0x10);
                let data = e.mem.u32(array);
                let size = e.mem.i32(array + 4);
                let mut at = (size as u32).wrapping_shl(4).wrapping_add(data);
                for _ in 0..(6i32.wrapping_sub(size)).max(0) {
                    store_vec(e, at, point);
                    at = at.wrapping_add(0x10);
                }
                e.mem.set_i32(array + 4, 6);
                let half = fld(half);
                let adjust = |e: &mut Engine, offset: u32, plus: bool| {
                    let at = e.mem.u32(array) + offset;
                    let value = fld(e.mem.f32(at));
                    let value = if plus { value + half } else { value - half };
                    e.mem.set_f32(at, value as f32);
                };
                adjust(e, 0, true);
                adjust(e, 0x10, false);
                adjust(e, 0x24, true);
                adjust(e, 0x34, false);
                adjust(e, 0x48, true);
                adjust(e, 0x58, false);
                build_from_array(e, this, array, source, config);
                release_array(e, array, 0x10, 0x10);
                None
            }
            1 => {
                fresh_array(e);
                array_reserve_for(e, array, 10, 0x10);
                hkgp_convex_hull_fetch_positions(e, this, 0, Ptr::new(array));
                let data = e.mem.u32(array);
                let first = load_vec(e, data);
                let second = load_vec(e, data + 0x10);
                let direction = sub(second, first);
                let absolute = [0usize, 1, 2].map(|lane| fld(direction[lane]).abs() as f32);
                let mut least = absolute[0];
                let (mut high_index, mut low_index, mut third_index) = (1usize, 0usize, 2usize);
                if fld(absolute[0]) > fld(absolute[1]) {
                    least = absolute[1];
                    high_index = 0;
                    low_index = third_index - 1;
                }
                if fld(absolute[2]) < fld(least) {
                    third_index = low_index;
                }
                let mut across = [0.0f32; 4];
                across[high_index] = x87_quiet(direction[third_index]);
                across[third_index] = -x87_quiet(direction[high_index]);
                let up = cross_yzx(direction, across);
                let across_length = sum3(mul(across, across));
                let inverse = inverse_length(e, across_length);
                let across = mul(mul(splat(inverse), across), splat(half));
                let up_length = sum3(mul(up, up));
                let inverse = inverse_length(e, up_length);
                let up = mul(mul(splat(inverse), up), splat(half));
                for point in [first, second] {
                    for vector in [across, up] {
                        array_push_vector(e, array, add(vector, point), 0x10);
                        array_push_vector(e, array, sub(point, vector), 0x10);
                    }
                }
                build_from_array(e, this, array, source, config);
                release_array(e, array, 0x10, 0x10);
                None
            }
            2 => {
                fresh_array(e);
                let centre = fn_00d752d0(e, this).addr();
                let centre = load_vec(e, centre);
                let normal_at = frame + 0x60;
                fn_00d74e50(e, this, Ptr::new(normal_at));
                let count = fn_00d75050(e, this);
                array_reserve_for(e, array, count.wrapping_mul(3), 0x10);
                hkgp_convex_hull_fetch_positions(e, this, 0, Ptr::new(array));
                let normal = load_vec(e, normal_at);
                let mut vertex = fn_00d75060(e, this).addr();
                let negative = -x87_quiet(margin);
                while vertex != 0 {
                    let position = fn_00d74ed0(e, Ptr::new(vertex)).addr();
                    let at = load_vec(e, position);
                    array_push_vector(e, array, add(mul(splat(margin), normal), at), 0x10);
                    let at = load_vec(e, position);
                    array_push_vector(e, array, add(mul(splat(negative), normal), at), 0x10);
                    vertex = fn_00d75120(e, Ptr::new(vertex));
                }
                let status = build_from_array(e, this, array, source, config);
                if status != 3 {
                    e.mem.set_u32(array + 4, 0);
                    array_push_vector(e, array, centre, 0x10);
                    build_from_array(e, this, array, source, config);
                    let result = hkgp_convex_hull_expand_by_planar_margin(e, this, margin);
                    release_array(e, array, 0x10, 0x10);
                    return Some(result);
                }
                release_array(e, array, 0x10, 0x10);
                None
            }
            3 => {
                fresh_array(e);
                let triangles = fn_00d75130(e, this) as i32;
                array_reserve_for(e, array, triangles.wrapping_mul(3).wrapping_mul(2), 0x10);
                hkgp_convex_hull_fetch_positions(e, this, 0, Ptr::new(array));
                let mut triangle = fn_00d75140(e, this);
                while triangle != 0 {
                    let plane_at = frame + 0x40;
                    fn_00d75b80(e, this, Ptr::new(plane_at), Ptr::new(triangle));
                    let tolerance = x87_quiet_bits(e.mem.u32(TOLERANCE_FLOAT));
                    let flag_at = frame + 0x27;
                    let answer = e
                        .call(VECTOR4_IS_NORMALIZED3, &args![plane_at, flag_at, tolerance])
                        .u32();
                    if e.mem.u8(answer) != 0 {
                        for corner in 0..3u32 {
                            let vertex = fn_00d752c0(e, Ptr::new(triangle), corner);
                            let position = fn_00d74ed0(e, Ptr::new(vertex)).addr();
                            let plane = load_vec(e, plane_at);
                            let at = load_vec(e, position);
                            array_push_vector(e, array, add(mul(splat(margin), plane), at), 0x10);
                        }
                    }
                    triangle = fn_00d75200(e, Ptr::new(triangle));
                }
                build_from_array(e, this, array, source, config);
                release_array(e, array, 0x10, 0x10);
                None
            }
            _ => {
                report_text(
                    e,
                    MESSAGE_INVALID_DIMENSIONS,
                    3,
                    0x172c_df48,
                    REPORT_FILE,
                    0xa77,
                    Some(0x00d7_86b5),
                );
                None
            }
        }
    });
    if let Some(result) = finished {
        return result;
    }
    if hkgp_convex_hull_get_dimensions(e, this) == 3 {
        if indices_valid != 0 {
            let tolerance = x87_quiet(e.mem.f32(BUILD_CONFIG_DEFAULT_FLOAT));
            hkgp_convex_hull_build_indices(e, this, tolerance, 0);
        }
        if mass_valid != 0 {
            hkgp_convex_hull_build_mass_properties(e, this);
        }
        return true;
    }
    false
}

/// The matrix `ensureContainment` starts from (four vectors at `0126a3e0`, set at start-up), the
/// doubles `1e-4` (`01032980`) and `1e-5` (`0101dae8`) it compares with, and the unnamed
/// `00de8e50` (`this` = a matrix, `(matrix)`: it multiplies through `00de8da0` into a temporary
/// and copies the 64 bytes back into `this`).
const CONTAINMENT_MATRIX: u32 = 0x0126_a3e0;
const CONTAINMENT_DISTANCE_LIMIT: u32 = 0x0103_2980;
const CONTAINMENT_OFFSET_LIMIT: u32 = 0x0101_dae8;
const MATRIX4_MUL_ASSIGN: u32 = 0x00de_8e50;

// Translated from 00d827b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::ensureContainment` (Xbox PDB; `this` = the implementation, stack
/// `(points)` = `{data, count}` of 16-byte vectors). Keeps a transform (four vectors, starting as
/// the matrix at `0126a3e0`) that grows until the hull contains the points: at most `count +
/// 0x40` times the worst point (`00d7a810`, answers the greatest plane distance and the plane,
/// which must be above `1e-4`) is moved onto its plane: the plane is negated, the hull's
/// extreme point along the negated plane (`00d7a0b0`) gives the offset `cc = -(plane.w + dot(extreme,
/// plane))` (must have `cc * cc` above `1e-5`), the plane's `w` becomes `-dot`, a scale `(cc +
/// distance) / cc` along the plane (`00d74c70`) is multiplied into the transform (`00de8e50`).
/// When the transform changed, the vertex positions are transformed (`((v.y * m1 + v.x * m0) +
/// v.z * m2) + m3`) into a temporary array and the hull rebuilt from them (`00d81510` with
/// zeros), then the indices (`00d7d2a0`) and the mass properties (`00d7b600`) are rebuilt when
/// they existed before.
pub fn hkgp_convex_hull_impl_ensure_containment(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    points: Ptr,
) {
    let implementation = this.addr();
    let indices_valid = e.mem.u8(implementation + 0x1b5);
    let mass_valid = e.mem.u8(implementation + 0x1b4);
    let limit = e.mem.i32(points.addr() + 4).wrapping_add(0x40);
    // The scratch block of the exe: the vector the extreme point goes to at `+0x10`, the plane
    // found at `+0x30`, the matrix at `+0x50`, the negated plane at `+0x90` and the saved copy
    // of the matrix at `+0xa0`.
    e.with_stack(0x100, |e, frame| {
        let frame = frame.addr();
        let (zero_vector, plane, matrix, saved, negated) = (
            frame + 0x10,
            frame + 0x30,
            frame + 0x50,
            frame + 0xa0,
            frame + 0x90,
        );
        for word in 0..16 {
            let value = e.mem.u32(CONTAINMENT_MATRIX + 4 * word);
            e.mem.set_u32(matrix + 4 * word, value);
        }
        let mut changed = false;
        let mut round = 0;
        while round < limit {
            let distance = fn_00d7a810(e, this, points, Ptr::new(matrix), Ptr::new(plane));
            if !(fld(distance) > e.global::<f64>(CONTAINMENT_DISTANCE_LIMIT)) {
                break;
            }
            let mask = load_bits(e, SIGN_MASK);
            let plane_bits = load_bits(e, plane);
            let flipped: [u32; 4] = std::array::from_fn(|lane| mask[lane] ^ plane_bits[lane]);
            for word in 0..4 {
                let value = e.mem.u32(ZERO_VECTOR_GLOBAL + 4 * word);
                e.mem.set_u32(zero_vector + 4 * word, value);
            }
            store_bits_at(e, negated, flipped);
            fn_00d7a0b0(
                e,
                this,
                Ptr::new(matrix),
                Ptr::new(negated),
                Ptr::new(zero_vector),
            );
            let plane_w = fld(e.mem.f32(plane + 0xc));
            let dot = sum3(mul(load_vec(e, zero_vector), load_vec(e, plane)));
            let offset = (-(plane_w + fld(dot))) as f32;
            let squared = fld(offset) * fld(offset);
            if !(squared > e.global::<f64>(CONTAINMENT_OFFSET_LIMIT)) {
                break;
            }
            e.mem.set_f32(plane + 0xc, (-fld(dot)) as f32);
            for word in 0..16 {
                let value = e.mem.u32(matrix + 4 * word);
                e.mem.set_u32(saved + 4 * word, value);
            }
            let scale = ((fld(offset) + fld(distance)) / fld(offset)) as f32;
            fn_00d74c70(e, Ptr::new(plane), scale, Ptr::new(matrix));
            e.call(MATRIX4_MUL_ASSIGN, &args![matrix, saved]);
            round += 1;
            changed = true;
        }
        if !changed {
            return;
        }
        e.with_stack(0x10, |e, array| {
            let array = array.addr();
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
            let count = e.mem.i32(implementation + 0x28);
            let old_size = array_append(e, array, count, 0x10);
            let data = e.mem.u32(array);
            let mut destination = (old_size as u32).wrapping_shl(4).wrapping_add(data);
            let mut node = e.mem.u32(implementation + 0x24);
            while node != 0 {
                copy_vec(e, node + 0x10, destination);
                destination = destination.wrapping_add(0x10);
                node = e.mem.u32(node);
            }
            let size = e.mem.i32(array + 4);
            let (row0, row1) = (load_vec(e, matrix), load_vec(e, matrix + 0x10));
            let (row2, row3) = (load_vec(e, matrix + 0x20), load_vec(e, matrix + 0x30));
            let mut index = 0u32;
            while (index as i32) < size {
                let at = data.wrapping_add(index << 4);
                let vertex = load_vec(e, at);
                let sum = add(mul(splat(vertex[1]), row1), mul(splat(vertex[0]), row0));
                let sum = add(sum, mul(splat(vertex[2]), row2));
                store_vec(e, at, add(sum, row3));
                index += 1;
            }
            hkgp_convex_hull_impl_build(e, this, Ptr::new(data), size, 0, 0);
            if indices_valid != 0 {
                hkgp_convex_hull_impl_build_indices(e, this);
            }
            if mass_valid != 0 {
                hkgp_convex_hull_impl_build_mass_properties(e, this);
            }
            release_array(e, array, 0x10, 0x10);
        });
    });
}

/// `hkArrayUtil::_reduce` (`00c909d0`, cdecl `(allocator, array, element size, 0, size)`: shrinks
/// the capacity of the array to its size) and the float `-1.0` (`01012054`).
const ARRAY_REDUCE: u32 = 0x00c9_09d0;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;

/// Reserves the `hkArray` `array` for `count` elements as the exe inlines it when the router's
/// field `allocator_offset` is the allocator: reserve twice the capacity or `count`, whichever is
/// greater, when the capacity is below `count` (the router is asked first).
fn array_reserve_with(
    e: &mut Engine,
    array: u32,
    count: i32,
    element_size: u32,
    allocator_offset: u32,
) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + allocator_offset);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let target = if count >= doubled { count } else { doubled };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, target, element_size],
        );
    }
}

/// The room for one more element of `element_size` bytes in the `hkArray` `array` (allocator at
/// the router's `allocator_offset`): `_reserveMore` when the size equals the capacity; answers the
/// address of the new element and adds one to the size.
fn array_take_slot(e: &mut Engine, array: u32, element_size: u32, allocator_offset: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    let size = e.mem.u32(array + 4);
    if size == capacity {
        let allocator = e.mem.u32(router + allocator_offset);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element_size]);
    }
    let size = e.mem.u32(array + 4);
    let slot = size
        .wrapping_mul(element_size)
        .wrapping_add(e.mem.u32(array));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    slot
}

/// Pushes the pair `(pointer, tag)` on the stack `array` of 8-byte entries (allocator at router
/// `+0xc`); a null slot is skipped as in the exe.
fn push_pair(e: &mut Engine, array: u32, pointer: u32, tag: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0xc);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 8u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
    if slot != 0 {
        e.mem.set_u32(slot, pointer);
        e.mem.set_u32(slot + 4, tag);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// The release of an `hkArray` through allocator slot `0x10` as the exe inlines it (the size is
/// cleared first; `element_shift` is the base two logarithm of the element size; a non-owning
/// buffer is not freed; the allocator is the router's field at `allocator_offset`).
fn release_array_shifted(e: &mut Engine, array: u32, element_shift: u32, allocator_offset: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + allocator_offset);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_shl(element_shift)],
        );
    }
}

/// The plane through the grid positions `p0`, `p1` and `p2` of a triangle (normal `e1 x e2`
/// with `e1 = p1 - p0`, `e2 = p2 - p0`, normalised through the CRT root unless exactly zero,
/// fourth lane `0.0 - dot(normal, p0)` through the x87).
fn triangle_plane_normalised(e: &mut Engine, p0: Vec4, p1: Vec4, p2: Vec4) -> Vec4 {
    let (edge_one, edge_two) = (sub(p1, p0), sub(p2, p0));
    let normal = cross_yzx(edge_one, edge_two);
    let length_squared = sum3(mul(normal, normal));
    let normal = if length_squared != 0.0 {
        normalise_with_root(e, normal, length_squared)
    } else {
        normal
    };
    let offset = x87_quiet(e.mem.f32(ZERO_VECTOR) - sum3(mul(normal, p0)));
    [normal[0], normal[1], normal[2], offset]
}

// Translated from 00d7d2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::buildIndices<hkgpConvexHullImpl::PositionAsTransform>` (Xbox PDB; `this`
/// = the implementation). Does nothing when the indices exist (`+0x1b5`). The planes of the hull
/// (`+0x54`, count `+0x58`) are built by dimension (`+0x1a8`):
///
/// * 3: every triangle (list `+0x3c`) gets its plane from its grid positions (normal `(p1 - p0) x
///   (p2 - p0)`, `1 / sqrt` through `00ec6040` unless zero, `w = 0.0 - dot`), kept in a temporary
///   array with the triangle's `+0x44` set to its number and `+0x38` to `-1`, and a map from
///   triangle to number; the triangles are also put in a second array with the length of that
///   cross product, sorted by it (descending, `00d7b270`). Each vertex (list `+0x24`) numbers
///   itself at `+0x34`. Then, taking the triangles in the sorted order, one that has no plane yet
///   (`+0x38 == -1`) takes the next plane number, its plane is appended to the planes and a flood
///   fill over its neighbours (the three links `+0x14`, `+0x18`, `+0x1c`, pairs `(triangle, tag)`
///   on a stack) gives every triangle that has no plane yet and whose plane is not normalised or
///   has `dot(plane, first plane) >= +4` of the implementation the same plane number, pushing its
///   other two links (the corners `(9 >> 2 * tag) & 3` and `(0x12 >> 2 * tag) & 3`);
/// * 2: for each vertex and the previous one (the last one for the first) the plane
///   `normalise(normal(+0xe0) x (v - previous))` through the previous grid position, turned
///   around when the centroid is on its positive side;
/// * 1: the two planes of the unit direction between the first and the last vertex, one through
///   each end.
///
/// With the byte at `+0xd` set every plane is then moved out by the distance of each vertex that
/// is in front of it; the planes array is reduced to its size (`00c909d0`); `+0x1b5` is set and
/// the temporary arrays and the map are released.
pub fn hkgp_convex_hull_impl_build_indices(e: &mut Engine, this: Ptr<HkgpConvexHullImpl>) {
    let implementation = this.addr();
    if e.mem.u8(implementation + 0x1b5) != 0 {
        return;
    }
    // The scratch block: the triangle list array at `+0`, the plane array at `+0xc`, the map at
    // `+0x18`, the stack of pairs at `+0x24` and a byte at `+0x30`.
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let (triangles, planes, map, stack, flag) =
            (frame, frame + 0xc, frame + 0x18, frame + 0x24, frame + 0x30);
        for array in [triangles, stack] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
        let triangle_count = e.mem.i32(implementation + 0x40);
        e.mem.set_u32(planes, 0);
        e.mem.set_u32(planes + 4, 0);
        e.mem.set_u32(planes + 8, 0x8000_0000);
        let mut last_bits = 0u32;
        if triangle_count != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = (e.mem.u32(triangles + 8) & 0x3fff_ffff) as i32;
            if capacity < triangle_count {
                let allocator = e.mem.u32(router + 0xc);
                e.call(
                    ARRAY_RESERVE,
                    &args![allocator, triangles, triangle_count, 8u32],
                );
            }
            let mut triangle = e.mem.u32(implementation + 0x3c);
            while triangle != 0 {
                let index = e.mem.u32(triangles + 4);
                e.mem.set_u32(triangles + 4, index.wrapping_add(1));
                let at = e.mem.u32(triangles).wrapping_add(index.wrapping_mul(8));
                e.mem.set_u32(at, triangle);
                let vertices = [
                    e.mem.u32(triangle + 8),
                    e.mem.u32(triangle + 0xc),
                    e.mem.u32(triangle + 0x10),
                ];
                let [p0, p1, p2] = vertices.map(|vertex| grid_position(e, implementation, vertex));
                let cross = cross_yzx(sub(p1, p0), sub(p2, p0));
                let length = sum3(mul(cross, cross)).sqrt();
                last_bits = length.to_bits();
                e.mem.set_u32(at + 4, x87_quiet_bits(length.to_bits()));
                triangle = e.mem.u32(triangle);
            }
            let size = e.mem.i32(triangles + 4);
            if size > 1 {
                let data = e.mem.u32(triangles);
                fn_00d7b270(e, data, 0, size - 1, last_bits & 0xffff_ff00);
            }
            let count = e.mem.i32(implementation + 0x40);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(MAP_RESERVE, &args![map, allocator, count]);
            let count = e.mem.i32(implementation + 0x40);
            array_reserve_with(e, planes, count, 0x10, 0xc);
            e.mem.set_i32(planes + 4, count);
            let mut triangle = e.mem.u32(implementation + 0x3c);
            let mut index = 0u32;
            let mut offset = 0u32;
            while triangle != 0 {
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let allocator = e.mem.u32(router + 0x10);
                e.call(MAP_INSERT, &args![map, allocator, triangle, index]);
                let vertices = [
                    e.mem.u32(triangle + 8),
                    e.mem.u32(triangle + 0xc),
                    e.mem.u32(triangle + 0x10),
                ];
                let [p0, p1, p2] = vertices.map(|vertex| grid_position(e, implementation, vertex));
                let plane = triangle_plane_normalised(e, p0, p1, p2);
                let at = e.mem.u32(planes).wrapping_add(offset);
                store_vec(e, at, plane);
                e.mem.set_u32(triangle + 0x44, index);
                e.mem.set_u32(triangle + 0x38, 0xffff_ffff);
                triangle = e.mem.u32(triangle);
                offset = offset.wrapping_add(0x10);
                index = index.wrapping_add(1);
            }
        }
        let mut vertex = e.mem.u32(implementation + 0x24);
        let mut number = 0u32;
        while vertex != 0 {
            e.mem.set_u32(vertex + 0x34, number);
            vertex = e.mem.u32(vertex);
            number += 1;
        }
        let dimensions = e.mem.i32(implementation + 0x1a8);
        e.mem.set_u32(implementation + 0x58, 0);
        let hull_planes = implementation + 0x54;
        match dimensions {
            3 => {
                let count = e.mem.i32(triangles + 4);
                let mut index = 0;
                while index < count {
                    let seed = e
                        .mem
                        .u32(e.mem.u32(triangles).wrapping_add(8 * index as u32));
                    if e.mem.u32(seed + 0x38) == 0xffff_ffff {
                        let found = e
                            .call(MAP_GET_WITH_DEFAULT, &args![map, seed, 0xffff_ffffu32])
                            .u32();
                        let seed_plane = found.wrapping_shl(4).wrapping_add(e.mem.u32(planes));
                        let plane_number = e.mem.u32(implementation + 0x58);
                        e.mem.set_u32(seed + 0x38, plane_number);
                        let slot = array_take_slot(e, hull_planes, 0x10, 0x10);
                        copy_vec(e, seed_plane, slot);
                        e.mem.set_u32(stack + 4, 0);
                        for link_offset in [0x14u32, 0x18, 0x1c] {
                            let link = e.mem.u32(seed + link_offset);
                            push_pair(e, stack, link & 0xffff_fffc, link & 3);
                        }
                        while e.mem.u32(stack + 4) != 0 {
                            let size = e.mem.u32(stack + 4);
                            let entry = e.mem.u32(stack).wrapping_add(size.wrapping_mul(8));
                            let triangle = e.mem.u32(entry.wrapping_sub(8));
                            let tag = e.mem.u32(entry.wrapping_sub(4));
                            e.mem.set_u32(stack + 4, size.wrapping_sub(1));
                            if e.mem.u32(triangle + 0x38) != 0xffff_ffff {
                                continue;
                            }
                            let found = e
                                .call(MAP_GET_WITH_DEFAULT, &args![map, triangle, 0xffff_ffffu32])
                                .u32();
                            let plane = found.wrapping_shl(4).wrapping_add(e.mem.u32(planes));
                            let tolerance = x87_quiet_bits(e.mem.u32(TOLERANCE_FLOAT));
                            let answer = e
                                .call(VECTOR4_IS_NORMALIZED3, &args![plane, flag, tolerance])
                                .u32();
                            let merge = if e.mem.u8(answer) == 0 {
                                true
                            } else {
                                let first = load_vec(e, seed_plane);
                                let second = load_vec(e, plane);
                                let dot = sum3(mul(second, first));
                                let threshold = e.mem.f32(implementation + 4);
                                fld(threshold) <= fld(dot)
                            };
                            if !merge {
                                continue;
                            }
                            let plane_id = e.mem.u32(seed + 0x38);
                            e.mem.set_u32(triangle + 0x38, plane_id);
                            let shift = tag.wrapping_mul(2);
                            let corner = (9u32 >> (shift & 0x1f)) & 3;
                            let link = e.mem.u32(triangle + 0x14 + corner * 4);
                            push_pair(e, stack, link & 0xffff_fffc, link & 3);
                            let corner = (0x12u32 >> (shift & 0x1f)) & 3;
                            let link = e.mem.u32(triangle + 0x14 + corner * 4);
                            push_pair(e, stack, link & 0xffff_fffc, link & 3);
                        }
                    }
                    index += 1;
                }
                release_array_shifted(e, stack, 3, 0xc);
            }
            2 => {
                let centroid_at = hkgp_convex_hull_impl_get_centroid(e, this).addr();
                let centroid = load_vec(e, centroid_at);
                let first = e.mem.u32(implementation + 0x24);
                let mut previous = list_last(e, first);
                let mut vertex = first;
                while vertex != 0 {
                    let previous_position = grid_position(e, implementation, previous);
                    let direction =
                        sub(grid_position(e, implementation, vertex), previous_position);
                    let slot = array_take_slot(e, hull_planes, 0x10, 0x10);
                    let normal = load_vec(e, implementation + 0xe0);
                    let side = cross_yzx(normal, direction);
                    store_vec(e, slot, side);
                    let length_squared = sum3(mul(side, side));
                    let plane = if length_squared != 0.0 {
                        normalise_with_root(e, side, length_squared)
                    } else {
                        side
                    };
                    store_vec(e, slot, plane);
                    let offset =
                        x87_quiet(e.mem.f32(ZERO_VECTOR) - sum3(mul(previous_position, plane)));
                    e.mem.set_f32(slot + 0xc, offset);
                    let plane = load_vec(e, slot);
                    let at_centroid = sum3(mul(plane, centroid)) + plane[3];
                    if 0.0 < at_centroid {
                        let mask = load_bits(e, SIGN_MASK);
                        let bits = load_bits(e, slot);
                        store_bits_at(e, slot, std::array::from_fn(|lane| bits[lane] ^ mask[lane]));
                    }
                    previous = vertex;
                    vertex = e.mem.u32(vertex);
                }
            }
            1 => {
                let first = e.mem.u32(implementation + 0x24);
                let start = grid_position(e, implementation, first);
                let last = list_last(e, first);
                let end = grid_position(e, implementation, last);
                let direction = sub(end, start);
                let length_squared = sum3(mul(direction, direction));
                let inverse = inverse_length(e, length_squared);
                let unit = mul(splat(inverse), direction);
                let offset = x87_quiet(e.mem.f32(ZERO_VECTOR) - sum3(mul(end, unit)));
                let slot = array_take_slot(e, hull_planes, 0x10, 0x10);
                store_vec(e, slot, [unit[0], unit[1], unit[2], offset]);
                let flipped = mul(
                    splat(e.mem.f32(MINUS_ONE_FLOAT)),
                    [unit[0], unit[1], unit[2], offset],
                );
                let offset = x87_quiet(e.mem.f32(ZERO_VECTOR) - sum3(mul(start, flipped)));
                let slot = array_take_slot(e, hull_planes, 0x10, 0x10);
                store_vec(e, slot, [flipped[0], flipped[1], flipped[2], offset]);
            }
            _ => {}
        }
        if e.mem.u8(implementation + 0xd) != 0 {
            let mut vertex = e.mem.u32(implementation + 0x24);
            while vertex != 0 {
                let mut index = 0u32;
                while (index as i32) < e.mem.i32(implementation + 0x58) {
                    let plane_at = e.mem.u32(hull_planes).wrapping_add(index << 4);
                    let plane = load_vec(e, plane_at);
                    let position = load_vec(e, vertex + 0x10);
                    let distance = sum3(mul(plane, position)) + plane[3];
                    if fld(distance) > 0.0 {
                        let offset = (fld(e.mem.f32(plane_at + 0xc)) - fld(distance)) as f32;
                        e.mem.set_f32(plane_at + 0xc, offset);
                    }
                    index += 1;
                }
                vertex = e.mem.u32(vertex);
            }
        }
        if e.mem.u32(implementation + 0x58) != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = (e.mem.u32(hull_planes + 8) & 0x3fff_ffff) as i32;
            let size = e.mem.i32(hull_planes + 4);
            if size < capacity {
                let allocator = e.mem.u32(router + 0x10);
                e.call(
                    ARRAY_REDUCE,
                    &args![allocator, hull_planes, 0x10u32, 0u32, size],
                );
            }
        }
        e.mem.set_u8(implementation + 0x1b5, 1);
        release_array_shifted(e, planes, 4, 0xc);
        e.mem.set_u32(planes, 0);
        e.mem.set_u32(planes + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
        e.call(LOCAL_NO_EFFECT, &args![map]);
        release_array_shifted(e, triangles, 3, 0xc);
    });
}

/// The function-local static of `generateIndexedFaces` (`01269158` guard word, `01269150` and
/// `01269154` the pair, set to 0 on first use and never written again): the start pair a
/// triangle gets when none of its neighbours is in another plane.
const FACES_STATIC_GUARD: u32 = 0x0126_9158;
const FACES_STATIC_TRIANGLE: u32 = 0x0126_9150;
const FACES_STATIC_CORNER: u32 = 0x0126_9154;

// Translated from 00d7df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::generateIndexedFaces<int, int>` (Xbox PDB; `this` = the implementation,
/// stack `(faces, indices, vertex_ids, reorder)`; `faces` and `indices` are `hkArray`s of `int`s
/// that are emptied first and reserved for `plane count` and `3 * vertex count` entries;
/// answers the number of faces). The "No index available" report (`INT3` at `00d7dfe7`) applies
/// first.
///
/// Three dimensions: the triangles (list `+0x3c`) go into a temporary array sorted by their word
/// at `+0x38` (`00d7a420`). The faces are numbered from 0; a triangle whose plane word equals the
/// number of the next face and which has a neighbour (links `+0x14`, `+0x18`, `+0x1c`) in another
/// plane starts the outline of that face: the vertices (`+0x34` of the vertex, or its word at
/// `+0x1c` with the bits `0x3f000000` cleared when `vertex_ids` is set) are written to `indices`
/// walking the edge `(9 >> 2 * corner) & 3` of the triangle and turning around the vertex through
/// the neighbours that are in the same plane (the link's low two bits give the corner on the
/// neighbour), until the start vertex comes round; the count of vertices goes to `faces`. A
/// triangle with no such neighbour starts nothing. Two dimensions: one face of all the vertices
/// (their numbers `0..n`, or the words at `+0x1c` masked as above).
///
/// When `reorder` is set and there are faces, each face of at least three indices is rotated so
/// that it starts at the vertex whose corner has the greatest squared cross product of the grid
/// edges (`(b - a) x (c - a)` for the vertex `a` and the next two, 32-bit products, the squares
/// summed as 64-bit through `_allmul`; the first of equals wins; nothing moves when the best is
/// the first).
pub fn hkgp_convex_hull_impl_generate_indexed_faces(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    faces: Ptr,
    indices: Ptr,
    vertex_ids: u8,
    reorder: u8,
) -> i32 {
    let implementation = this.addr();
    let (faces, indices) = (faces.addr(), indices.addr());
    let mut face_count = 0i32;
    assert_indices(e, this, 0x00d7_dfe7);
    e.mem.set_u32(faces + 4, 0);
    let plane_count = e.mem.i32(implementation + 0x58);
    array_reserve_for(e, faces, plane_count, 4);
    e.mem.set_u32(indices + 4, 0);
    let vertex_count = e.mem.i32(implementation + 0x28);
    array_reserve_for(e, indices, vertex_count.wrapping_mul(3), 4);
    let push_word = |e: &mut Engine, array: u32, value: u32| {
        let slot = array_take_slot(e, array, 4, 0x10);
        e.mem.set_u32(slot, value);
    };
    let dimensions = e.mem.i32(implementation + 0x1a8);
    if dimensions == 3 {
        let triangle_count = e.mem.i32(implementation + 0x40);
        e.with_stack(0x10, |e, list| {
            let list = list.addr();
            e.mem.set_u32(list, 0);
            e.mem.set_u32(list + 4, 0);
            e.mem.set_u32(list + 8, 0x8000_0000);
            array_reserve_for(e, list, triangle_count, 4);
            let mut node = e.mem.u32(implementation + 0x3c);
            while node != 0 {
                let size = e.mem.u32(list + 4);
                let data = e.mem.u32(list);
                e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), node);
                e.mem.set_u32(list + 4, size.wrapping_add(1));
                node = e.mem.u32(node);
            }
            let size = e.mem.i32(list + 4);
            if size > 1 {
                let data = e.mem.u32(list);
                fn_00d7a420(e, data, 0, size - 1, Ptr::new(0));
            }
            let mut index = 0i32;
            while index < size {
                let triangle = e.mem.u32(e.mem.u32(list).wrapping_add(4 * index as u32));
                if e.mem.u32(triangle + 0x38) == face_count as u32 {
                    if e.mem.u8(FACES_STATIC_GUARD) & 1 == 0 {
                        let guard = e.mem.u32(FACES_STATIC_GUARD);
                        e.mem.set_u32(FACES_STATIC_GUARD, guard | 1);
                        e.mem.set_u32(FACES_STATIC_TRIANGLE, 0);
                        e.mem.set_u32(FACES_STATIC_CORNER, 0);
                    }
                    let mut start_triangle = e.mem.u32(FACES_STATIC_TRIANGLE);
                    let mut start_corner = e.mem.u32(FACES_STATIC_CORNER);
                    for corner in 0..3u32 {
                        let neighbour = e.mem.u32(triangle + 0x14 + 4 * corner) & 0xffff_fffc;
                        if e.mem.u32(neighbour + 0x38) != face_count as u32 {
                            start_triangle = triangle;
                            start_corner = corner;
                            break;
                        }
                    }
                    if start_triangle != 0 {
                        let mut current = start_triangle;
                        let mut corner = start_corner;
                        let start_vertex = e.mem.u32(current + 8 + corner * 4);
                        let mut count = 0u32;
                        loop {
                            count += 1;
                            let vertex = e.mem.u32(current + 8 + corner * 4);
                            let value = if vertex_ids == 0 {
                                e.mem.u32(vertex + 0x34)
                            } else {
                                e.mem.u32(vertex + 0x1c) & 0xc0ff_ffff
                            };
                            push_word(e, indices, value);
                            corner = (9u32 >> ((corner * 2) & 0x1f)) & 3;
                            loop {
                                let across = e.mem.u32(current + 0x14 + corner * 4) & 0xffff_fffc;
                                if e.mem.u32(across + 0x38) != face_count as u32 {
                                    break;
                                }
                                let link = e.mem.u32(current + 0x14 + corner * 4);
                                current = link & 0xffff_fffc;
                                corner = (9u32 >> (((link & 3) * 2) & 0x1f)) & 3;
                            }
                            if e.mem.u32(current + 8 + corner * 4) == start_vertex {
                                break;
                            }
                        }
                        push_word(e, faces, count);
                        face_count += 1;
                    }
                }
                index += 1;
            }
            release_array(e, list, 4, 0x10);
        });
    } else if dimensions == 2 {
        let count = e.mem.u32(implementation + 0x28);
        face_count = 1;
        push_word(e, faces, count);
        if vertex_ids == 0 {
            let mut index = 0u32;
            while (index as i32) < e.mem.i32(implementation + 0x28) {
                push_word(e, indices, index);
                index += 1;
            }
        } else {
            let mut vertex = e.mem.u32(implementation + 0x24);
            while vertex != 0 {
                let value = e.mem.u32(vertex + 0x1c) & 0xc0ff_ffff;
                push_word(e, indices, value);
                vertex = e.mem.u32(vertex);
            }
        }
    }
    if reorder != 0 && face_count != 0 {
        let mut polygon = e.mem.u32(indices);
        e.with_stack(0x10, |e, vertices| {
            let vertices = vertices.addr();
            e.mem.set_u32(vertices, 0);
            e.mem.set_u32(vertices + 4, 0);
            e.mem.set_u32(vertices + 8, 0x8000_0000);
            let count = e.mem.i32(implementation + 0x28);
            array_reserve_for(e, vertices, count, 4);
            let mut vertex = e.mem.u32(implementation + 0x24);
            while vertex != 0 {
                push_word(e, vertices, vertex);
                vertex = e.mem.u32(vertex);
            }
            let vertex_at = |e: &Engine, id: u32| -> u32 {
                e.mem
                    .u32(e.mem.u32(vertices).wrapping_add(id.wrapping_mul(4)))
            };
            let mut face = 0i32;
            while face < e.mem.i32(faces + 4) {
                let size = e.mem.i32(e.mem.u32(faces).wrapping_add(4 * face as u32));
                if size >= 3 {
                    let mut best = -1i64;
                    let mut best_corner = -1i32;
                    let mut corner = 0i32;
                    while corner < size {
                        let id = |e: &Engine, offset: i32| -> u32 {
                            let at = polygon.wrapping_add(4 * ((corner + offset) % size) as u32);
                            e.mem.u32(at)
                        };
                        let third = vertex_at(e, id(e, 2));
                        let second = vertex_at(e, id(e, 1));
                        let first = vertex_at(e, id(e, 0));
                        let coordinate =
                            |e: &Engine, vertex: u32, offset: u32| e.mem.i32(vertex + offset);
                        let dy1 =
                            coordinate(e, second, 0x24).wrapping_sub(coordinate(e, first, 0x24));
                        let dy2 =
                            coordinate(e, third, 0x24).wrapping_sub(coordinate(e, first, 0x24));
                        let dz1 =
                            coordinate(e, second, 0x28).wrapping_sub(coordinate(e, first, 0x28));
                        let dx1 =
                            coordinate(e, second, 0x20).wrapping_sub(coordinate(e, first, 0x20));
                        let dx2 =
                            coordinate(e, third, 0x20).wrapping_sub(coordinate(e, first, 0x20));
                        let dz2 =
                            coordinate(e, third, 0x28).wrapping_sub(coordinate(e, first, 0x28));
                        let x = dz2.wrapping_mul(dy1).wrapping_sub(dy2.wrapping_mul(dz1));
                        let z = dy2.wrapping_mul(dx1).wrapping_sub(dx2.wrapping_mul(dy1));
                        let y = dx2.wrapping_mul(dz1).wrapping_sub(dz2.wrapping_mul(dx1));
                        let mut sum = product_64(e, x, x);
                        sum = sum.wrapping_add(product_64(e, y, y));
                        sum = sum.wrapping_add(product_64(e, z, z));
                        if sum > best {
                            best = sum;
                            best_corner = corner;
                        }
                        corner += 1;
                    }
                    if best_corner > 0 {
                        e.with_stack(0x90, |e, buffer| {
                            let temporary = buffer.addr();
                            let inline = temporary + 0xc;
                            e.mem.set_u32(temporary, inline);
                            e.mem.set_u32(temporary + 4, 0);
                            e.mem.set_u32(temporary + 8, 0x8000_0020);
                            array_reserve_for(e, temporary, best_corner, 4);
                            e.mem.set_i32(temporary + 4, best_corner);
                            for k in 0..best_corner as u32 {
                                let value = e.mem.u32(polygon + 4 * k);
                                let at = e.mem.u32(temporary) + 4 * k;
                                e.mem.set_u32(at, value);
                            }
                            let mut destination = polygon;
                            for k in best_corner..size {
                                let value = e.mem.u32(polygon + 4 * k as u32);
                                e.mem.set_u32(destination, value);
                                destination += 4;
                            }
                            for k in 0..best_corner as u32 {
                                let value = e.mem.u32(e.mem.u32(temporary) + 4 * k);
                                e.mem.set_u32(destination, value);
                                destination += 4;
                            }
                            release_array(e, temporary, 4, 0x10);
                        });
                    }
                }
                polygon = polygon.wrapping_add(4 * size as u32);
                face += 1;
            }
            release_array(e, vertices, 4, 0x10);
        });
    }
    face_count
}

/// `"Internal error"` and the grid coordinates of a pending item or vertex (`+0x20`, `+0x24`,
/// `+0x28`).
const MESSAGE_INTERNAL_ERROR: u32 = 0x010d_36ec;

fn grid_coordinates(e: &Engine, item: u32) -> [i32; 3] {
    [
        e.mem.i32(item + 0x20),
        e.mem.i32(item + 0x24),
        e.mem.i32(item + 0x28),
    ]
}

/// The cross product the exe forms in 32-bit integers for the triangle `(p, q, r)`: with `a = q
/// - p` and `b = r - p` it is `(b.z * a.y - b.y * a.z, b.x * a.z - b.z * a.x, b.y * a.x - b.x *
/// a.y)`.
fn integer_normal(p: [i32; 3], q: [i32; 3], r: [i32; 3]) -> [i32; 3] {
    let a = [
        q[0].wrapping_sub(p[0]),
        q[1].wrapping_sub(p[1]),
        q[2].wrapping_sub(p[2]),
    ];
    let b = [
        r[0].wrapping_sub(p[0]),
        r[1].wrapping_sub(p[1]),
        r[2].wrapping_sub(p[2]),
    ];
    [
        b[2].wrapping_mul(a[1])
            .wrapping_sub(b[1].wrapping_mul(a[2])),
        b[0].wrapping_mul(a[2])
            .wrapping_sub(b[2].wrapping_mul(a[0])),
        b[1].wrapping_mul(a[0])
            .wrapping_sub(b[0].wrapping_mul(a[1])),
    ]
}

/// The plane constant `-(n . p)` of the exact planes, a 64-bit sum of the products of the 32-bit
/// values (`n.y * p.y + n.z * p.z + n.x * p.x`), negated.
fn plane_constant(p: [i32; 3], n: [i32; 3]) -> i64 {
    let sum = i64::from(n[1])
        .wrapping_mul(i64::from(p[1]))
        .wrapping_add(i64::from(n[2]).wrapping_mul(i64::from(p[2])))
        .wrapping_add(i64::from(n[0]).wrapping_mul(i64::from(p[0])));
    sum.wrapping_neg()
}

/// Gives the triangle `triangle` its three vertices, clears the stamp (`+0x40`), sets `+0x44` to
/// `-1`, counts it as a use of each vertex (`+0x30`), and computes its exact plane from the
/// first vertex `p`: the integer normal at `+0x20..+0x28`, the constant at `+0x30` (low) and
/// `+0x34` (high), both 0 when the normal is.
fn init_triangle(e: &mut Engine, triangle: u32, vertices: [u32; 3]) {
    e.mem.set_u32(triangle + 8, vertices[0]);
    e.mem.set_u32(triangle + 0xc, vertices[1]);
    e.mem.set_u32(triangle + 0x10, vertices[2]);
    e.mem.set_u32(triangle + 0x40, 0);
    e.mem.set_u32(triangle + 0x44, 0xffff_ffff);
    for vertex in vertices {
        let uses = e.mem.u32(vertex + 0x30);
        e.mem.set_u32(vertex + 0x30, uses.wrapping_add(1));
    }
    let points = vertices.map(|vertex| grid_coordinates(e, vertex));
    let normal = integer_normal(points[0], points[1], points[2]);
    for (index, value) in normal.into_iter().enumerate() {
        e.mem.set_i32(triangle + 0x20 + 4 * index as u32, value);
    }
    e.mem.set_u32(triangle + 0x30, 0);
    e.mem.set_u32(triangle + 0x34, 0);
    if normal != [0, 0, 0] {
        let constant = plane_constant(points[0], normal);
        e.mem.set_u32(triangle + 0x30, constant as u32);
        e.mem.set_u32(triangle + 0x34, (constant >> 32) as u32);
    }
}

/// A new vertex in the vertex pool (`00d7ef20` on the list at `+0x18`, with a template that has
/// `0.0` at `+0x2c` and `-1` at `+0x34`) filled from the pending item `item`: the position
/// vector, the three grid coordinates, the float at `+0x2c` (through the x87) and the word at
/// `+0x34`; the use count `+0x30` is zero.
fn new_vertex_from_item(e: &mut Engine, implementation: u32, template: u32, item: u32) -> u32 {
    e.mem.set_u32(template + 0x2c, 0);
    e.mem.set_u32(template + 0x34, 0xffff_ffff);
    let vertex = fn_00d7ef20(e, Ptr::new(implementation + 0x18), Ptr::new(template));
    copy_vec(e, item + 0x10, vertex + 0x10);
    for offset in [0x20u32, 0x24, 0x28] {
        let word = e.mem.u32(item + offset);
        e.mem.set_u32(vertex + offset, word);
    }
    copy_through_x87(e, vertex + 0x2c, item + 0x2c);
    e.mem.set_u32(vertex + 0x30, 0);
    let word = e.mem.u32(item + 0x34);
    e.mem.set_u32(vertex + 0x34, word);
    vertex
}

/// A new triangle in the triangle pool (`00d7efc0` on the list at `+0x30`, with a template that
/// has `-1` at `+0x38` and `+0x44`).
fn new_triangle(e: &mut Engine, implementation: u32, template: u32) -> u32 {
    e.mem.set_u32(template + 0x38, 0xffff_ffff);
    e.mem.set_u32(template + 0x44, 0xffff_ffff);
    fn_00d7efc0(e, Ptr::new(implementation + 0x30), Ptr::new(template))
}

/// The grid position of integer coordinates as `00d74e80` computes it (`float` conversion,
/// the float at `01011d78` as fourth lane, times the scale at `+0x110` plus the offset at
/// `+0xf0`).
fn grid_from_integers(e: &Engine, implementation: u32, p: [i32; 3]) -> Vec4 {
    let coordinates = [p[0] as f32, p[1] as f32, p[2] as f32, e.mem.f32(ZERO_F32)];
    let scale = load_vec(e, implementation + 0x110);
    let offset = load_vec(e, implementation + 0xf0);
    add(mul(coordinates, scale), offset)
}

/// Removes the pending item at `index` from the array at `+0x48` by moving the last item into
/// its place (when it is not the last).
fn remove_pending_item(e: &mut Engine, implementation: u32, index: i32) {
    let count = e.mem.i32(implementation + 0x4c).wrapping_sub(1);
    e.mem.set_i32(implementation + 0x4c, count);
    if count != index {
        let data = e.mem.u32(implementation + 0x48);
        let source = data.wrapping_add((count as u32) << 6);
        let destination = data.wrapping_add((index as u32) << 6);
        for word in 0..16u32 {
            let value = e.mem.u32(source + 4 * word);
            e.mem.set_u32(destination + 4 * word, value);
        }
    }
}

// Translated from 00d7fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::flushPending` (Xbox PDB; `this` = the implementation, answers a byte:
/// whether a first solid was found). The pending points are the items (`0x40` bytes: position at
/// `+0x10`, grid coordinates `+0x20..+0x28`) of the array at `+0x48` (count `+0x4c`); fewer
/// than four is the report "Internal error" (id `642cf968`, line `0x2f4`, `INT3` at
/// `00d7fbfb`). The normal at `+0xe0` starts as the vector at `01267e30`.
///
/// First every set of four items `i < j < k < l` is tried in this order for the first that is
/// not coplanar (exact 32-bit integer cross product `n` of the triangle `i, j, k`, not zero,
/// and `n . l - n . i` not zero in 64-bit); for every triangle `(i, j, k)` with a normal the
/// float plane (when the byte at `+0` is set) replaces the normal at `+0xe0` when its squared
/// length is greater. The four items become vertices (pool `+0x18`) and the four faces of the
/// tetrahedron triangles (pool `+0x30`, linked through `+0x14..+0x1c`, orientation chosen by the
/// side of `l`), and are removed from the pending array. When none is found the answer is 0 and
/// nothing else happens.
///
/// Then the pending points are inserted one by one, last first: the first triangle of the list
/// the point is in front of (the 64-bit `n . p + c` positive) is the start: the point becomes a
/// vertex (stamp `+0x1ac` incremented), the triangles it sees are removed (`clearHorizon`,
/// `00d7c3e0`) and a fan of new triangles over the horizon edges, each linked to the old
/// triangle behind the edge and to the previous and next of the fan, closes the hole. A point no
/// triangle sees is dropped. At the end the pending array is released and the answer is 1.
pub fn hkgp_convex_hull_impl_flush_pending(e: &mut Engine, this: Ptr<HkgpConvexHullImpl>) -> bool {
    let implementation = this.addr();
    let count = e.mem.i32(implementation + 0x4c);
    if count < 4 {
        report_text(
            e,
            MESSAGE_INTERNAL_ERROR,
            3,
            0x642c_f968,
            REPORT_FILE,
            0x2f4,
            Some(0x00d7_fbfb),
        );
    }
    copy_vec(e, ZERO_VECTOR_GLOBAL, implementation + 0xe0);
    // The scratch block: the template item for the pools at `+0`, the edge pair at `+0x50` and
    // the horizon pair at `+0x58`.
    e.with_stack(0x80, |e, scratch| {
        let scratch = scratch.addr();
        let (template, edge, horizon) = (scratch, scratch + 0x50, scratch + 0x58);
        let item = |e: &Engine, index: i32| -> u32 {
            e.mem
                .u32(implementation + 0x48)
                .wrapping_add((index as u32) << 6)
        };
        let mut found = false;
        'search: for i in 0..count {
            let point_i = grid_coordinates(e, item(e, i));
            for j in (i + 1)..count {
                let point_j = grid_coordinates(e, item(e, j));
                for k in (j + 1)..count {
                    let point_k = grid_coordinates(e, item(e, k));
                    let normal = integer_normal(point_i, point_j, point_k);
                    if normal == [0, 0, 0] {
                        continue;
                    }
                    let mut constant = 0i64;
                    for axis in 0..3 {
                        constant =
                            constant.wrapping_add(product_64(e, point_i[axis], normal[axis]));
                    }
                    constant = constant.wrapping_neg();
                    if e.mem.u8(implementation) != 0 {
                        let zero = e.mem.f32(ZERO_VECTOR);
                        let p_i = grid_from_integers(e, implementation, point_i);
                        let p_j = grid_from_integers(e, implementation, point_j);
                        let p_k = grid_from_integers(e, implementation, point_k);
                        let across = cross_yzx(sub(p_j, p_i), sub(p_k, p_i));
                        let length_squared = sum3(mul(across, across));
                        let inverse = inverse_length(e, length_squared);
                        let unit = mul(splat(inverse), across);
                        let offset = x87_quiet(zero - sum3(mul(unit, p_i)));
                        let plane = [unit[0], unit[1], unit[2], offset];
                        let new_length = sum3(mul(plane, plane));
                        let current = load_vec(e, implementation + 0xe0);
                        let current_length = sum3(mul(current, current));
                        if fld(current_length) < fld(new_length) {
                            store_vec(e, implementation + 0xe0, plane);
                        }
                    }
                    for l in (k + 1)..count {
                        let point_l = grid_coordinates(e, item(e, l));
                        let mut sum = 0i64;
                        for axis in 0..3 {
                            sum = sum.wrapping_add(product_64(e, point_l[axis], normal[axis]));
                        }
                        sum = sum.wrapping_add(constant);
                        if sum == 0 {
                            continue;
                        }
                        let vertex_i =
                            new_vertex_from_item(e, implementation, template, item(e, i));
                        let vertex_j =
                            new_vertex_from_item(e, implementation, template, item(e, j));
                        let vertex_k =
                            new_vertex_from_item(e, implementation, template, item(e, k));
                        let vertex_l =
                            new_vertex_from_item(e, implementation, template, item(e, l));
                        let (a, b) = if sum > 0 {
                            (vertex_j, vertex_i)
                        } else {
                            (vertex_i, vertex_j)
                        };
                        let first = new_triangle(e, implementation, template);
                        let second = new_triangle(e, implementation, template);
                        let third = new_triangle(e, implementation, template);
                        let fourth = new_triangle(e, implementation, template);
                        init_triangle(e, first, [a, b, vertex_k]);
                        init_triangle(e, second, [a, vertex_l, b]);
                        init_triangle(e, third, [a, vertex_k, vertex_l]);
                        init_triangle(e, fourth, [b, vertex_l, vertex_k]);
                        e.mem.set_u32(first + 0x14, second + 2);
                        e.mem.set_u32(second + 0x1c, first);
                        e.mem.set_u32(first + 0x18, fourth + 2);
                        e.mem.set_u32(fourth + 0x1c, first + 1);
                        e.mem.set_u32(first + 0x1c, third);
                        e.mem.set_u32(third + 0x14, first + 2);
                        e.mem.set_u32(second + 0x14, third + 2);
                        e.mem.set_u32(third + 0x1c, second);
                        e.mem.set_u32(third + 0x18, fourth + 1);
                        e.mem.set_u32(fourth + 0x18, third + 1);
                        e.mem.set_u32(fourth + 0x14, second + 1);
                        e.mem.set_u32(second + 0x18, fourth);
                        remove_pending_item(e, implementation, l);
                        remove_pending_item(e, implementation, k);
                        remove_pending_item(e, implementation, j);
                        remove_pending_item(e, implementation, i);
                        found = true;
                        break 'search;
                    }
                }
            }
        }
        if !found {
            return false;
        }
        while e.mem.i32(implementation + 0x4c) > 0 {
            let remaining = e.mem.i32(implementation + 0x4c);
            let last = e
                .mem
                .u32(implementation + 0x48)
                .wrapping_add(((remaining - 1) as u32) << 6);
            let mut triangle = e.mem.u32(implementation + 0x3c);
            let point = grid_coordinates(e, last);
            while triangle != 0 {
                let mut sum = 0i64;
                for axis in [1usize, 2, 0] {
                    let normal = e.mem.i32(triangle + 0x20 + 4 * axis as u32);
                    sum = sum.wrapping_add(product_64(e, normal, point[axis]));
                }
                let constant = i64::from(e.mem.u32(triangle + 0x30))
                    | (i64::from(e.mem.i32(triangle + 0x34)) << 32);
                sum = sum.wrapping_add(constant);
                if sum > 0 {
                    let vertex = new_vertex_from_item(e, implementation, template, last);
                    e.mem.set_u32(edge, triangle);
                    e.mem.set_u32(edge + 4, 0);
                    e.mem.set_u32(horizon, 0);
                    e.mem.set_u32(horizon + 4, 0);
                    let stamp = e.mem.u32(implementation + 0x1ac);
                    e.mem.set_u32(implementation + 0x1ac, stamp.wrapping_add(1));
                    hkgp_convex_hull_impl_clear_horizon(
                        e,
                        this,
                        Ptr::new(vertex),
                        Ptr::new(edge),
                        Ptr::new(horizon),
                    );
                    let mut around = e.mem.u32(horizon);
                    let mut corner = e.mem.u32(horizon + 4);
                    while e.mem.u32(around + 0x14 + 4 * corner) & 0xffff_fffc != 0 {
                        corner = (9u32 >> ((corner * 2) & 0x1f)) & 3;
                        e.mem.set_u32(horizon + 4, corner);
                    }
                    let start = around.wrapping_add(corner);
                    let mut previous = 0u32;
                    let mut first_new = 0u32;
                    loop {
                        corner = (9u32 >> ((corner * 2) & 0x1f)) & 3;
                        while e.mem.u32(around + 0x14 + 4 * corner) & 0xffff_fffc != 0 {
                            let link = e.mem.u32(around + 0x14 + 4 * corner);
                            around = link & 0xffff_fffc;
                            corner = (9u32 >> (((link & 3) * 2) & 0x1f)) & 3;
                        }
                        e.mem.set_u32(horizon, around);
                        e.mem.set_u32(horizon + 4, corner);
                        let closing = around.wrapping_add(corner) == start;
                        let created = new_triangle(e, implementation, template);
                        let near = e.mem.u32(around + 8 + 4 * corner);
                        let next = (9u32 >> ((corner * 2) & 0x1f)) & 3;
                        let far = e.mem.u32(around + 8 + 4 * next);
                        init_triangle(e, created, [vertex, far, near]);
                        e.mem.set_u32(around + 0x14 + 4 * corner, created + 1);
                        e.mem.set_u32(created + 0x18, around.wrapping_add(corner));
                        if closing {
                            e.mem.set_u32(previous + 0x14, created + 2);
                            e.mem.set_u32(created + 0x1c, previous);
                            e.mem.set_u32(created + 0x14, first_new.wrapping_add(2));
                            if first_new != 0 {
                                e.mem.set_u32(first_new + 0x1c, created);
                            }
                            break;
                        }
                        if previous != 0 {
                            e.mem.set_u32(previous + 0x14, created + 2);
                            e.mem.set_u32(created + 0x1c, previous);
                        } else {
                            first_new = created;
                        }
                        previous = created;
                    }
                    break;
                }
                triangle = e.mem.u32(triangle);
            }
            let remaining = e.mem.i32(implementation + 0x4c).wrapping_sub(1);
            e.mem.set_i32(implementation + 0x4c, remaining);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(implementation + 0x50);
        e.mem.set_u32(implementation + 0x4c, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(implementation + 0x48);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 6]);
        }
        e.mem.set_u32(implementation + 0x48, 0);
        e.mem.set_u32(implementation + 0x50, 0x8000_0000);
        true
    })
}

/// `"Input domain out of range"`, and the doubles `-FLT_MAX` (`0104e140`) and `1e-5`
/// (`0101dae8`, a `float` widened) the domain of the points is checked against.
const MESSAGE_DOMAIN_OUT_OF_RANGE: u32 = 0x010d_3718;
const NEGATIVE_FLOAT_MAX_DOUBLE: u32 = 0x0104_e140;
const DOMAIN_MINIMUM_WIDTH: u32 = 0x0101_dae8;
/// The squared length (a `double`, `1e-4` as a `float` widened, `01032980`) above which a plane
/// normal found by `build_find_normal` ends the search.
const NORMAL_LENGTH_LIMIT: u32 = 0x0103_2980;

/// The refined reciprocal `RCPPS` plus one Newton step per lane, `(r + r) - (r * x) * r`.
fn refined_reciprocals(x: Vec4) -> Vec4 {
    let estimate: Vec4 = std::array::from_fn(|lane| rcp_estimate(x[lane]));
    sub(add(estimate, estimate), mul(mul(estimate, x), estimate))
}

/// The three lane vector `v` with the first three lanes of `v` and `1.0` (the fourth lane of
/// the ones vector at `010c7170`) as fourth lane: `SHUFPS v, UNPCKHPS(v, ones), 0xc4`.
fn with_one(e: &Engine, v: Vec4) -> Vec4 {
    shufps(v, unpckhps(v, load_vec(e, ONES_VECTOR)), 0xc4)
}

/// The axis along which the vector `normal` is largest in absolute value (the first three lanes
/// masked with the vector at `01017d10` and compared as the exe's x87 sequence does: `y` must be
/// greater than `x` to beat it, and `z` greater than the best so far).
fn dominant_axis(e: &Engine, normal: Vec4) -> u32 {
    let mask = load_bits(e, ABS_MASK);
    let absolute: Vec4 =
        std::array::from_fn(|lane| f32::from_bits(normal[lane].to_bits() & mask[lane]));
    let (x, y, z) = (fld(absolute[0]), fld(absolute[1]), fld(absolute[2]));
    if y > x {
        if z > y {
            2
        } else {
            1
        }
    } else if z > x {
        2
    } else {
        0
    }
}

/// Exchanges two grid coordinates of each of the `count` items of the array at `+0x48` (axis 0:
/// `+0x20` and `+0x28`, axis 1: `+0x24` and `+0x28`; axis 2 does nothing).
fn swap_item_coordinates(e: &mut Engine, implementation: u32, axis: u32, count: i32) {
    let (first, second) = match axis {
        0 => (0x20u32, 0x28u32),
        1 => (0x24, 0x28),
        _ => return,
    };
    for index in 0..count.max(0) as u32 {
        let item = e.mem.u32(implementation + 0x48).wrapping_add(index << 6);
        let a = e.mem.u32(item + first);
        let b = e.mem.u32(item + second);
        e.mem.set_u32(item + first, b);
        e.mem.set_u32(item + second, a);
    }
}

/// Gives the freshly reserved elements `old_size..new_size` of the array of `0x40` byte items
/// at `array` their start values: `0.0` at `+0x2c` and `-1` at `+0x34`.
fn initialise_new_items(e: &mut Engine, array: u32, old_size: i32, new_size: i32) {
    let mut item = (old_size as u32)
        .wrapping_shl(6)
        .wrapping_add(e.mem.u32(array));
    for _ in 0..new_size.wrapping_sub(old_size).max(0) {
        if item != 0 {
            e.mem.set_u32(item + 0x2c, 0);
            e.mem.set_u32(item + 0x34, 0xffff_ffff);
        }
        item = item.wrapping_add(0x40);
    }
}

/// Copies an item (`0x40` bytes) as the exe does: the vector at `+0x10`, the three grid
/// coordinates, the float at `+0x2c` through the x87 and the words at `+0x30` and `+0x34`.
fn copy_item(e: &mut Engine, from: u32, to: u32) {
    copy_vec(e, from + 0x10, to + 0x10);
    for offset in [0x20u32, 0x24, 0x28] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    copy_through_x87(e, to + 0x2c, from + 0x2c);
    for offset in [0x30u32, 0x34] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
}

/// The grid of the points when none is given and the configuration byte (`+0xc`) is clear: the
/// box of the points (`MINPS` / `MAXPS`; the report "Input domain out of range", id
/// `405a2174`, line `0x228`, `INT3` at `00d817a8`, when a lane is beyond `+-FLT_MAX`) widened
/// to at least `1e-5` in each of the three lanes around its middle, the offset (`+0xf0`) its
/// centre, the cell (`+0x100`) `8191 / half extent` and the scale (`+0x110`) the refined
/// reciprocal of that.
fn build_domain_grid(e: &mut Engine, implementation: u32, points: u32, count: i32) {
    let mut low = load_vec(e, points);
    let mut high = low;
    for index in 1..count {
        let point = load_vec(e, points.wrapping_add((index as u32) << 4));
        low = zip_with(low, point, minps);
        high = zip_with(high, point, maxps);
    }
    let minimum_width: f64 = e.global(DOMAIN_MINIMUM_WIDTH);
    let half: f64 = e.global(HALF_DOUBLE);
    let flt_max: f64 = e.global(FLOAT_MAX_DOUBLE);
    let negative_flt_max: f64 = e.global(NEGATIVE_FLOAT_MAX_DOUBLE);
    for lane in 0..3 {
        if fld(low[lane]) < negative_flt_max || fld(high[lane]) > flt_max {
            report_text(
                e,
                MESSAGE_DOMAIN_OUT_OF_RANGE,
                3,
                0x405a_2174,
                REPORT_FILE,
                0x228,
                Some(0x00d8_17a8),
            );
        }
        if fld(high[lane]) - fld(low[lane]) < minimum_width {
            let middle = ((fld(high[lane]) + fld(low[lane])) * half) as f32;
            let lowest = x87_quiet(low[lane]);
            let below = (fld(middle) - minimum_width) as f32;
            let new_low = if fld(below) > fld(lowest) {
                lowest
            } else {
                below
            };
            low[lane] = x87_quiet(new_low);
            let highest = x87_quiet(high[lane]);
            let above = (fld(middle) + minimum_width) as f32;
            let new_high = if fld(above) < fld(highest) {
                highest
            } else {
                above
            };
            high[lane] = x87_quiet(new_high);
        }
    }
    let half_extent = mul(splat(e.mem.f32(HALF_FLOAT)), sub(high, low));
    store_vec(e, implementation + 0xf0, add(half_extent, low));
    let cell = refined_reciprocals(with_one(e, half_extent));
    store_vec(e, implementation + 0x100, cell);
    e.mem.set_u32(implementation + 0xfc, 0);
    e.mem.set_u32(implementation + 0x10c, 0);
    let cell = mul(
        splat(e.mem.f32(GRID_MAX_FLOAT)),
        load_vec(e, implementation + 0x100),
    );
    store_vec(e, implementation + 0x100, cell);
    let scale = refined_reciprocals(with_one(e, load_vec(e, implementation + 0x100)));
    store_vec(e, implementation + 0x110, scale);
}

/// The plane normal of the points: the cross product `(p[j] - p[i]) x (p[k] - p[i])` with the
/// greatest squared length found so far (a candidate replaces the normal at `+0xe0` when its
/// squared length is greater, and the search stops at the first above `1e-4`), normalised through
/// the CRT root (the vector `(1, 0, 0, 0)` when its squared length is exactly zero), with the
/// fourth lane `0.0 - dot(normal, p[0])` through the x87.
fn build_find_normal(e: &mut Engine, implementation: u32, points: u32, count: i32) {
    let threshold: f64 = e.global(NORMAL_LENGTH_LIMIT);
    let point = |e: &Engine, index: i32| load_vec(e, points.wrapping_add((index as u32) << 4));
    'search: for i in 0..count {
        let origin = point(e, i);
        for j in (i + 1)..count {
            let a = sub(point(e, j), origin);
            for k in (j + 1)..count {
                let b = sub(point(e, k), origin);
                let normal = cross_yzx(a, b);
                let length_squared = sum3(mul(normal, normal));
                let current = load_vec(e, implementation + 0xe0);
                let current_squared = sum3(mul(current, current));
                if fld(current_squared) < fld(length_squared) {
                    store_vec(e, implementation + 0xe0, normal);
                    if fld(length_squared) > threshold {
                        break 'search;
                    }
                }
            }
        }
    }
    let found = load_vec(e, implementation + 0xe0);
    let length_squared = sum3(mul(found, found));
    if length_squared != 0.0 {
        let normal = normalise_with_root(e, found, length_squared);
        store_vec(e, implementation + 0xe0, normal);
    } else {
        let zero = e.mem.f32(ZERO_F32);
        store_vec(
            e,
            implementation + 0xe0,
            [e.mem.f32(ONE_FLOAT), zero, zero, zero],
        );
    }
    let normal = load_vec(e, implementation + 0xe0);
    let offset = x87_quiet(e.mem.f32(ZERO_VECTOR) - sum3(mul(point(e, 0), normal)));
    e.mem.set_f32(implementation + 0xec, offset);
}

/// Creates the pending items from the points: for each point (`index`) the item at `+0x48` gets
/// the point as its vector, the grid coordinates `((p - offset) * cell)` clamped and rounded as
/// the exe's x87 sequence does (`fn grid_coordinate`), `0.0` at `+0x2c`, `0` at `+0x30` (stack
/// garbage in the exe), `-1` at `+0x34`; with the byte at `+8` set the fourth lane of the vector
/// is `index | 0x3f000000`.
fn build_store_items(e: &mut Engine, implementation: u32, points: u32, count: i32) {
    let limits = grid_limits(e);
    for index in 0..count {
        let at = points.wrapping_add((index as u32) << 4);
        let words = load_bits(e, at);
        let point: Vec4 = words.map(f32::from_bits);
        let offset = load_vec(e, implementation + 0xf0);
        let cell = load_vec(e, implementation + 0x100);
        let quantised = mul(sub(point, offset), cell);
        let integers = [0usize, 1, 2].map(|lane| grid_coordinate(e, &limits, quantised[lane]));
        let item = e
            .mem
            .u32(implementation + 0x48)
            .wrapping_add((index as u32) << 6);
        store_bits_at(e, item + 0x10, words);
        for (lane, value) in integers.into_iter().enumerate() {
            e.mem.set_i32(item + 0x20 + 4 * lane as u32, value);
        }
        e.mem.set_u32(item + 0x2c, 0);
        e.mem.set_u32(item + 0x30, 0);
        e.mem.set_u32(item + 0x34, 0xffff_ffff);
        if e.mem.u8(implementation + 8) != 0 {
            e.mem.set_u32(item + 0x1c, (index as u32) | 0x3f00_0000);
        }
    }
}

/// The planar case of the build: the items' grid coordinates are exchanged so that the largest
/// axis of the normal becomes the third, the items sorted (`00d7af90`) and the 2D convex hull of
/// the first two coordinates taken (Andrew's monotone chain; a vertex is dropped while the turn
/// is not strictly positive: `(b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x) <= 0`) into a
/// temporary array of `2 * count` items. With at most three hull points (the last repeats the
/// first) the coordinates are exchanged back and the dimension count becomes 1 (answers `true`
/// to try again). Otherwise the hull becomes the item array (`00d9ec10`), its last (repeated)
/// point is dropped, the coordinates are exchanged back and a vertex is created from each item,
/// which are then dropped (answers `false`).
fn build_planar_hull(e: &mut Engine, implementation: u32, count: i32) -> bool {
    let normal = load_vec(e, implementation + 0xe0);
    let axis = dominant_axis(e, normal);
    let item_count = e.mem.i32(implementation + 0x4c);
    swap_item_coordinates(e, implementation, axis, item_count);
    let item_count = e.mem.i32(implementation + 0x4c);
    if item_count > 1 {
        let data = e.mem.u32(implementation + 0x48);
        fn_00d7af90(e, data, 0, item_count - 1, 0);
    }
    e.with_stack(0x10, |e, hull| {
        let hull = hull.addr();
        e.mem.set_u32(hull, 0);
        e.mem.set_u32(hull + 4, 0);
        e.mem.set_u32(hull + 8, 0x8000_0000);
        array_reserve_for(e, hull, count.wrapping_mul(2), 0x40);
        let old_size = e.mem.i32(hull + 4);
        initialise_new_items(e, hull, old_size, count.wrapping_mul(2));
        e.mem.set_i32(hull + 4, count.wrapping_mul(2));
        let turn = |e: &Engine, size: i32, point: u32| -> i32 {
            let top = e.mem.u32(hull).wrapping_add((size as u32) << 6);
            let a = top.wrapping_sub(0x80);
            let b = top.wrapping_sub(0x40);
            let (ax, ay) = (e.mem.i32(a + 0x20), e.mem.i32(a + 0x24));
            let (bx, by) = (e.mem.i32(b + 0x20), e.mem.i32(b + 0x24));
            let (px, py) = (e.mem.i32(point + 0x20), e.mem.i32(point + 0x24));
            bx.wrapping_sub(ax)
                .wrapping_mul(py.wrapping_sub(ay))
                .wrapping_sub(by.wrapping_sub(ay).wrapping_mul(px.wrapping_sub(ax)))
        };
        let mut size = 0i32;
        for index in 0..count {
            let point = e
                .mem
                .u32(implementation + 0x48)
                .wrapping_add((index as u32) << 6);
            while size >= 2 {
                if turn(e, size, point) > 0 {
                    break;
                }
                size -= 1;
            }
            let slot = e.mem.u32(hull).wrapping_add((size as u32) << 6);
            copy_item(e, point, slot);
            size += 1;
        }
        let lower = size + 1;
        let mut index = count.wrapping_sub(2);
        while index >= 0 {
            let point = e
                .mem
                .u32(implementation + 0x48)
                .wrapping_add((index as u32) << 6);
            while size >= lower {
                if turn(e, size, point) > 0 {
                    break;
                }
                size -= 1;
            }
            let slot = e.mem.u32(hull).wrapping_add((size as u32) << 6);
            copy_item(e, point, slot);
            size += 1;
            index -= 1;
        }
        let again;
        if size <= 3 {
            let normal = load_vec(e, implementation + 0xe0);
            let axis = dominant_axis(e, normal);
            let item_count = e.mem.i32(implementation + 0x4c);
            swap_item_coordinates(e, implementation, axis, item_count);
            e.mem.set_u32(implementation + 0x1a8, 1);
            again = true;
        } else {
            e.call(ARRAY_SWAP, &args![implementation + 0x48, hull]);
            let kept = size - 1;
            array_reserve_for(e, implementation + 0x48, kept, 0x40);
            let old_size = e.mem.i32(implementation + 0x4c);
            initialise_new_items(e, implementation + 0x48, old_size, kept);
            e.mem.set_i32(implementation + 0x4c, kept);
            let normal = load_vec(e, implementation + 0xe0);
            let axis = dominant_axis(e, normal);
            swap_item_coordinates(e, implementation, axis, kept);
            let template = e.mem.alloc(0x40);
            let vertex_count = e.mem.i32(implementation + 0x4c);
            for index in 0..vertex_count {
                let item = e
                    .mem
                    .u32(implementation + 0x48)
                    .wrapping_add((index as u32) << 6);
                new_vertex_from_item(e, implementation, template, item);
            }
            e.mem.free(template);
            e.mem.set_u32(implementation + 0x4c, 0);
            again = false;
        }
        release_array(e, hull, 0x40, 0x10);
        again
    })
}

// Translated from 00d81510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::build` (Xbox PDB; `this` = the implementation, stack `(points, count,
/// keep_normal, grid_source)`; answers the dimension count, `-1` when the build failed). Points
/// that are not 16-byte aligned are first copied into an aligned temporary array and built from
/// there. The hull is reset (`00d7ac40` with `keep_normal`); with `count <= 0` nothing else
/// happens. The grid (`+0xf0`, `+0x100`, `+0x110`) is copied from the implementation
/// `grid_source`, or is the unit grid when the configuration byte at `+0xc` is set
/// (`(0, 0, 0, 0)`, `(1, 1, 1, 0)`, `(1, 1, 1, 0)`), or comes from the box of the points
/// (`build_domain_grid`). The dimension count (`+0x1a8`) is `min(count - 1, 3)` (2 with
/// `keep_normal`), `+0xd0` the first edge, `+0xe0` the plane normal (`build_find_normal`; the
/// zero vector when `count <= 2`), and the array of pending items at `+0x48` is filled from
/// the points (`build_store_items`) and, when `keep_normal` is clear and the byte at `+0xb` is
/// set, sorted (`00d7af90`). Then by dimension: 0 makes a vertex of the first item; 1 the two
/// items with the least and the greatest projection on the edge (a vertex each; if they have
/// the same first two grid coordinates the dimension count becomes 0 and it starts again); 2
/// the planar hull (`build_planar_hull`, which may lower the dimension count to 1 and start
/// again); 3 `flushPending` (`00d7fb80`): when it fails the dimension count becomes 2 (if the
/// byte at `+0` is set; started again) or `-1`. Unless the dimension count is `-1` the indices
/// (`00d7d2a0`, byte `+9`) and the mass properties (`00d7b600`, byte `+0xa`) are built.
pub fn hkgp_convex_hull_impl_build(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    points: Ptr,
    count: i32,
    keep_normal: u32,
    grid_source: u32,
) -> i32 {
    let implementation = this.addr();
    let points = points.addr();
    if count != 0 && points & 0xf != 0 {
        return e.with_stack(0x10, |e, array| {
            let array = array.addr();
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
            array_reserve_for(e, array, count, 0x10);
            e.mem.set_i32(array + 4, count);
            for index in 0..count.max(0) as u32 {
                let words = load_bits(e, points + (index << 4));
                let at = e.mem.u32(array).wrapping_add(index << 4);
                store_bits_at(e, at, words);
            }
            let data = e.mem.u32(array);
            let result = hkgp_convex_hull_impl_build(
                e,
                this,
                Ptr::new(data),
                count,
                keep_normal,
                grid_source,
            );
            release_array(e, array, 0x10, 0x10);
            result
        });
    }
    hkgp_convex_hull_impl_reset(e, this, keep_normal as u8);
    if count > 0 {
        let zero = e.mem.f32(ZERO_F32);
        let one = e.mem.f32(ONE_FLOAT);
        if grid_source != 0 {
            for offset in [0xf0u32, 0x100, 0x110] {
                copy_vec(e, grid_source + offset, implementation + offset);
            }
        } else if e.mem.u8(implementation + 0xc) != 0 {
            store_vec(e, implementation + 0xf0, [zero; 4]);
            store_vec(e, implementation + 0x100, [one, one, one, zero]);
            store_vec(e, implementation + 0x110, [one, one, one, zero]);
        } else {
            build_domain_grid(e, implementation, points, count);
        }
        let dimensions = if count - 1 < 3 { count - 1 } else { 3 };
        e.mem.set_i32(implementation + 0x1a8, dimensions);
        copy_vec(e, ZERO_VECTOR_GLOBAL, implementation + 0xd0);
        if keep_normal as u8 != 0 {
            e.mem.set_u32(implementation + 0x1a8, 2);
        } else {
            copy_vec(e, ZERO_VECTOR_GLOBAL, implementation + 0xe0);
            if count > 1 {
                let edge = sub(load_vec(e, points + 0x10), load_vec(e, points));
                store_vec(e, implementation + 0xd0, edge);
            }
            if count > 2 {
                build_find_normal(e, implementation, points, count);
            }
        }
        e.mem.set_u32(implementation + 0x4c, 0);
        array_reserve_for(e, implementation + 0x48, count, 0x40);
        let old_size = e.mem.i32(implementation + 0x4c);
        initialise_new_items(e, implementation + 0x48, old_size, count);
        e.mem.set_i32(implementation + 0x4c, count);
        build_store_items(e, implementation, points, count);
        if keep_normal as u8 == 0 && e.mem.u8(implementation + 0xb) != 0 {
            let item_count = e.mem.i32(implementation + 0x4c);
            if item_count > 1 {
                let data = e.mem.u32(implementation + 0x48);
                let extra = ((count as u32) << 6) & 0xffff_ff00;
                fn_00d7af90(e, data, 0, item_count - 1, extra);
            }
        }
        let template = e.mem.alloc(0x40);
        loop {
            let dimensions = e.mem.u32(implementation + 0x1a8);
            match dimensions {
                0 => {
                    let item = e.mem.u32(implementation + 0x48);
                    new_vertex_from_item(e, implementation, template, item);
                    break;
                }
                1 => {
                    let direction = load_vec(e, implementation + 0xd0);
                    let item_at = |e: &Engine, index: i32| {
                        e.mem
                            .u32(implementation + 0x48)
                            .wrapping_add((index as u32) << 6)
                    };
                    let projection =
                        |e: &Engine, item: u32| sum3(mul(load_vec(e, item + 0x10), direction));
                    let (mut least, mut greatest) = (0i32, 0i32);
                    for index in 1..count {
                        let current = projection(e, item_at(e, index));
                        if fld(projection(e, item_at(e, least))) > fld(current) {
                            least = index;
                        }
                        if fld(projection(e, item_at(e, greatest))) < fld(current) {
                            greatest = index;
                        }
                    }
                    let (first, second) = (item_at(e, least), item_at(e, greatest));
                    let same = e.mem.i32(first + 0x20) == e.mem.i32(second + 0x20)
                        && e.mem.i32(first + 0x24) == e.mem.i32(second + 0x24);
                    if same {
                        e.mem.set_u32(implementation + 0x1a8, 0);
                        continue;
                    }
                    new_vertex_from_item(e, implementation, template, first);
                    new_vertex_from_item(e, implementation, template, second);
                    break;
                }
                2 => {
                    if build_planar_hull(e, implementation, count) {
                        continue;
                    }
                    break;
                }
                3 => {
                    if hkgp_convex_hull_impl_flush_pending(e, this) {
                        break;
                    }
                    if e.mem.u8(implementation) != 0 {
                        e.mem.set_u32(implementation + 0x1a8, 2);
                        continue;
                    }
                    e.mem.set_u32(implementation + 0x1a8, 0xffff_ffff);
                    break;
                }
                _ => break,
            }
        }
        e.mem.free(template);
    }
    if e.mem.u32(implementation + 0x1a8) != 0xffff_ffff {
        if e.mem.u8(implementation + 9) != 0 {
            hkgp_convex_hull_impl_build_indices(e, this);
        }
        if e.mem.u8(implementation + 0xa) != 0 {
            hkgp_convex_hull_impl_build_mass_properties(e, this);
        }
    }
    e.mem.i32(implementation + 0x1a8)
}

/// `hkOstream::operator<<(int)` (`00cc0280`: `this` = the builder, `(value)`, answers the
/// builder), `"Invalid dimension for this operation ("` and `" required)"`.
const REPORT_BUILDER_SIGNED_NUMBER: u32 = 0x00cc_0280;
const MESSAGE_INVALID_DIMENSION: u32 = 0x010d_35ac;
const MESSAGE_REQUIRED: u32 = 0x010d_35d4;

/// The sum of the determinants of the fan of triangles around a vertex: starting with the
/// triangle `start` and its corner `start_corner` (the vertex), each triangle `(tri, corner)` of
/// the fan gives the determinant (`00cd51c0`) of the three vectors from `Q`, the grid position of
/// the vertex at index `(9 >> 2 * corner) & 3`, to the vertex at `corner`, to the vertex at
/// `(0x12 >> 2 * corner) & 3` and to the vertex of the neighbour across the link at `corner`
/// that is at `(0x12 >> 2 * its tag) & 3`; the sum is a `float` (each determinant added to the
/// running `float` through the x87). The fan is walked through the links at `(0x12 >> 2 *
/// corner) & 3` back to the start; when it runs into an open edge, the other direction is
/// walked from the neighbour across the start's link at `start_corner` (each triangle `(u,
/// corner)` taken with the corner `(9 >> 2 * tag) & 3` of the link it was reached through).
fn fan_determinant_sum(
    e: &mut Engine,
    implementation: u32,
    scratch: u32,
    start: u32,
    start_corner: u32,
) -> f32 {
    let mut sum = 0.0f32;
    let rows = scratch;
    let out = scratch + 0x40;
    let determinant = |e: &mut Engine, tri: u32, corner: u32, sum: f32| -> f32 {
        let vertex_at = |e: &Engine, index: u32| e.mem.u32(tri + 8 + 4 * index);
        let next = (9u32 >> ((corner * 2) & 0x1f)) & 3;
        let after = (0x12u32 >> ((corner * 2) & 0x1f)) & 3;
        let reference = grid_position(e, implementation, vertex_at(e, next));
        let a = sub(
            grid_position(e, implementation, vertex_at(e, corner)),
            reference,
        );
        let b = sub(
            grid_position(e, implementation, vertex_at(e, after)),
            reference,
        );
        let link = e.mem.u32(tri + 0x14 + 4 * corner);
        let neighbour = link & 0xffff_fffc;
        let neighbour_index = (0x12u32 >> (((link & 3) * 2) & 0x1f)) & 3;
        let far = e.mem.u32(neighbour + 8 + 4 * neighbour_index);
        let c = sub(grid_position(e, implementation, far), reference);
        store_vec(e, rows, a);
        store_vec(e, rows + 0x10, b);
        store_vec(e, rows + 0x20, c);
        let result = e.call(MATRIX3_GET_DETERMINANT, &args![rows, out]).u32();
        let value = e.mem.f32(result);
        (fld(value) + fld(sum)) as f32
    };
    let (mut tri, mut corner) = (start, start_corner);
    loop {
        sum = determinant(e, tri, corner, sum);
        let after = (0x12u32 >> ((corner * 2) & 0x1f)) & 3;
        let link = e.mem.u32(tri + 0x14 + 4 * after);
        let tag = link & 3;
        let next_triangle = link & 0xffff_fffc;
        if next_triangle == 0 {
            // An open edge: the other direction.
            let first = e.mem.u32(start + 0x14 + 4 * start_corner);
            let mut tri = first & 0xffff_fffc;
            let mut corner = (9u32 >> (((first & 3) * 2) & 0x1f)) & 3;
            while tri != 0 {
                sum = determinant(e, tri, corner, sum);
                let link = e.mem.u32(tri + 0x14 + 4 * corner);
                tri = link & 0xffff_fffc;
                corner = (9u32 >> (((link & 3) * 2) & 0x1f)) & 3;
            }
            return sum;
        }
        tri = next_triangle;
        corner = tag;
        if next_triangle.wrapping_add(tag) == start.wrapping_add(start_corner) {
            return sum;
        }
    }
}

/// Adds `1.0` to the marks (`+0x2c`) of the vertex at `corner` of `start` and of the vertices
/// around it (the vertex at `(9 >> 2 * corner) & 3` of each triangle of the fan, walked as
/// `fan_determinant_sum` does through the links at `(0x12 >> 2 * corner) & 3`, and, at an open
/// edge, in the other direction through the neighbour across the start's link at `corner`);
/// answers whether a vertex around (not the first) reached a mark above 1.0.
fn mark_fan(e: &mut Engine, start: u32, start_corner: u32) -> bool {
    let bump = |e: &mut Engine, vertex: u32| -> f32 {
        let mark = (fld(e.mem.f32(vertex + 0x2c)) + 1.0) as f32;
        e.mem.set_f32(vertex + 0x2c, mark);
        mark
    };
    let first = e.mem.u32(start + 8 + 4 * start_corner);
    bump(e, first);
    let mut conflict = false;
    let (mut tri, mut corner) = (start, start_corner);
    loop {
        let next = (9u32 >> ((corner * 2) & 0x1f)) & 3;
        let vertex = e.mem.u32(tri + 8 + 4 * next);
        if bump(e, vertex) > 1.0 {
            conflict = true;
        }
        let after = (0x12u32 >> ((corner * 2) & 0x1f)) & 3;
        let link = e.mem.u32(tri + 0x14 + 4 * after);
        let next_triangle = link & 0xffff_fffc;
        if next_triangle == 0 {
            let first = e.mem.u32(start + 0x14 + 4 * start_corner);
            let mut tri = first & 0xffff_fffc;
            let mut corner = (9u32 >> (((first & 3) * 2) & 0x1f)) & 3;
            while tri != 0 {
                let next = (9u32 >> ((corner * 2) & 0x1f)) & 3;
                let vertex = e.mem.u32(tri + 8 + 4 * next);
                if bump(e, vertex) > 1.0 {
                    conflict = true;
                }
                let link = e.mem.u32(tri + 0x14 + 4 * corner);
                tri = link & 0xffff_fffc;
                corner = (9u32 >> (((link & 3) * 2) & 0x1f)) & 3;
            }
            return conflict;
        }
        tri = next_triangle;
        corner = link & 3;
        if next_triangle.wrapping_add(corner) == start.wrapping_add(start_corner) {
            return conflict;
        }
    }
}

// Translated from 00d82ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::decimateVertices` (Xbox PDB; `this` = the implementation, stack
/// `(target, contain)`, `contain` a byte; answers whether it worked). The target is at least 4;
/// with no more vertices than that nothing happens (answers `true`). A hull that is not three
/// dimensional is the report "Invalid dimension for this operation (3 required)" (id `79f9d888`,
/// line `0x154`, `INT3` at `00d82b9d`). With `contain` the vertex positions are saved first.
///
/// While there are more vertices than the target: every vertex (marks at `+0x2c` clear) gets as
/// weight the sum of the determinants of its fan of triangles (`fan_determinant_sum`) in an array
/// of `(triangle, corner, weight)`; the array is sorted by weight (`00d7b160`) and the lightest
/// `vertices - target` are taken, in order, until one whose fan shares a vertex with a fan taken
/// before (`mark_fan`: a mark above 1.0); those vertices are unlinked from the list at `+0x24`
/// and released to the pool (`00d7adb0`); the remaining vertex positions rebuild the hull
/// (`00d81510` with zeros); a failed rebuild (`-1`) releases everything and answers `false`.
/// Afterwards, with `contain` the hull is made to contain the saved positions again
/// (`00d827b0`) and the indices (`00d7d2a0`) and mass properties (`00d7b600`) are rebuilt when
/// they existed before.
pub fn hkgp_convex_hull_impl_decimate_vertices(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    target: i32,
    contain: u32,
) -> bool {
    let implementation = this.addr();
    let target = target.max(4);
    if e.mem.i32(implementation + 0x28) <= target {
        return true;
    }
    let indices_valid = e.mem.u8(implementation + 0x1b5);
    let mass_valid = e.mem.u8(implementation + 0x1b4);
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        // The saved positions (`+0x00`), the weights (`+0x10`) and the scratch for the
        // determinants (`+0x20`..).
        let (saved, weights, scratch) = (frame, frame + 0x10, frame + 0x20);
        for array in [saved, weights] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
        if e.mem.i32(implementation + 0x1a8) != 3 {
            e.with_stack(0x20, |e, builder| {
                e.with_stack(0x200, |e, buffer| {
                    let (builder, buffer) = (builder.addr(), buffer.addr());
                    e.call(REPORT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
                    let text = e
                        .call(
                            REPORT_BUILDER_TEXT,
                            &args![builder, MESSAGE_INVALID_DIMENSION],
                        )
                        .u32();
                    let text = e
                        .call(REPORT_BUILDER_SIGNED_NUMBER, &args![text, 3u32])
                        .u32();
                    e.call(REPORT_BUILDER_TEXT, &args![text, MESSAGE_REQUIRED]);
                    let sink = e.global::<u32>(REPORT_SINK);
                    let answer = e
                        .vcall(
                            sink,
                            0xc,
                            &args![3u32, 0x79f9_d888u32, buffer, REPORT_FILE, 0x154u32],
                        )
                        .u32();
                    if answer != 0 {
                        panic!("INT3 at 00d82b9d (assertion break)");
                    }
                    e.call(REPORT_BUILDER_DESTRUCT, &args![builder]);
                });
            });
        }
        if contain as u8 != 0 {
            let count = e.mem.i32(implementation + 0x28);
            let old_size = array_append(e, saved, count, 0x10);
            let mut destination = (old_size as u32)
                .wrapping_shl(4)
                .wrapping_add(e.mem.u32(saved));
            let mut vertex = e.mem.u32(implementation + 0x24);
            while vertex != 0 {
                copy_vec(e, vertex + 0x10, destination);
                destination = destination.wrapping_add(0x10);
                vertex = e.mem.u32(vertex);
            }
        }
        // The weights array is reserved for exactly the vertex count.
        let count = e.mem.i32(implementation + 0x28);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = (e.mem.u32(weights + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE, &args![allocator, weights, count, 0xcu32]);
        }
        let release_all = |e: &mut Engine| {
            release_array(e, weights, 0xc, 0x10);
            e.mem.set_u32(weights, 0);
            e.mem.set_u32(weights + 8, 0x8000_0000);
            release_array(e, saved, 0x10, 0x10);
        };
        while e.mem.i32(implementation + 0x28) > target {
            let mut vertex = e.mem.u32(implementation + 0x24);
            e.mem.set_u32(weights + 4, 0);
            while vertex != 0 {
                e.mem.set_u32(vertex + 0x2c, 0);
                vertex = e.mem.u32(vertex);
            }
            let mut triangle = e.mem.u32(implementation + 0x3c);
            while triangle != 0 {
                for corner in 0..3u32 {
                    let vertex = e.mem.u32(triangle + 8 + 4 * corner);
                    if e.mem.f32(vertex + 0x2c) == 0.0 {
                        let sum = fan_determinant_sum(e, implementation, scratch, triangle, corner);
                        let slot = array_take_slot(e, weights, 0xc, 0x10);
                        e.mem.set_f32(slot + 8, sum);
                        e.mem.set_u32(slot, triangle);
                        e.mem.set_u32(slot + 4, corner);
                        e.mem.set_u32(vertex + 0x2c, 0x3f80_0000);
                    }
                }
                triangle = e.mem.u32(triangle);
            }
            let mut vertex = e.mem.u32(implementation + 0x24);
            while vertex != 0 {
                e.mem.set_u32(vertex + 0x2c, 0);
                vertex = e.mem.u32(vertex);
            }
            let size = e.mem.i32(weights + 4);
            if size > 1 {
                let data = e.mem.u32(weights);
                fn_00d7b160(e, data, 0, size - 1, 0);
            }
            let removals = e.mem.i32(implementation + 0x28).wrapping_sub(target);
            let mut taken = 0i32;
            while taken < removals {
                let record = e.mem.u32(weights).wrapping_add(12 * taken as u32);
                let triangle = e.mem.u32(record);
                let corner = e.mem.u32(record + 4);
                if mark_fan(e, triangle, corner) {
                    break;
                }
                taken += 1;
            }
            for index in 0..taken {
                let record = e.mem.u32(weights).wrapping_add(12 * index as u32);
                let triangle = e.mem.u32(record);
                let corner = e.mem.u32(record + 4);
                let vertex = e.mem.u32(triangle + 8 + 4 * corner);
                let next = e.mem.u32(vertex);
                let previous = e.mem.u32(vertex + 4);
                if next != 0 {
                    e.mem.set_u32(next + 4, previous);
                }
                if previous != 0 {
                    e.mem.set_u32(previous, next);
                } else {
                    e.mem.set_u32(implementation + 0x24, next);
                }
                let remaining = e.mem.u32(implementation + 0x28).wrapping_sub(1);
                e.mem.set_u32(implementation + 0x28, remaining);
                hk_geometry_processing_pool_allocator_vertex_release(
                    e,
                    Ptr::new(implementation + 0x18),
                    vertex,
                );
            }
            let status = e.with_stack(0x10, |e, positions| {
                let positions = positions.addr();
                e.mem.set_u32(positions, 0);
                e.mem.set_u32(positions + 4, 0);
                e.mem.set_u32(positions + 8, 0x8000_0000);
                let count = e.mem.i32(implementation + 0x28);
                let old_size = array_append(e, positions, count, 0x10);
                let mut destination = (old_size as u32)
                    .wrapping_shl(4)
                    .wrapping_add(e.mem.u32(positions));
                let mut vertex = e.mem.u32(implementation + 0x24);
                while vertex != 0 {
                    copy_vec(e, vertex + 0x10, destination);
                    destination = destination.wrapping_add(0x10);
                    vertex = e.mem.u32(vertex);
                }
                let data = e.mem.u32(positions);
                let size = e.mem.i32(positions + 4);
                let result = hkgp_convex_hull_impl_build(e, this, Ptr::new(data), size, 0, 0);
                if result != -1 {
                    e.mem.set_u32(positions + 4, 0);
                }
                release_array(e, positions, 0x10, 0x10);
                e.mem.set_u32(positions, 0);
                e.mem.set_u32(positions + 8, 0x8000_0000);
                result
            });
            if status == -1 {
                release_all(e);
                return false;
            }
        }
        if contain as u8 != 0 {
            hkgp_convex_hull_impl_ensure_containment(e, this, Ptr::new(saved));
        }
        if indices_valid != 0 {
            hkgp_convex_hull_impl_build_indices(e, this);
        }
        if mass_valid != 0 {
            hkgp_convex_hull_impl_build_mass_properties(e, this);
        }
        release_all(e);
        true
    })
}

/// `"Current dimension not implemented for that operation"` and the vector of ones at
/// `010ec7f0` the crossing points are interpolated with.
const MESSAGE_SPLIT_UNSUPPORTED: u32 = 0x010d_3770;

/// Allocates a hull (`new_hull`), stores it in `*out` and gives its implementation the build
/// configuration (the four words at `+0`) of `implementation` and, with `copy_normal`, its
/// normal (the vector at `+0xe0`).
fn new_split_hull(
    e: &mut Engine,
    implementation: u32,
    out: u32,
    copy_normal: bool,
) -> Ptr<HkgpConvexHull> {
    let hull = new_hull(e);
    e.mem.set_u32(out, hull.addr());
    let target = e.get(hull, HkgpConvexHull::implementation).addr();
    for word in 0..4u32 {
        let value = e.mem.u32(implementation + 4 * word);
        e.mem.set_u32(target + 4 * word, value);
    }
    if copy_normal {
        let target = e
            .get(
                Ptr::<HkgpConvexHull>::new(e.mem.u32(out)),
                HkgpConvexHull::implementation,
            )
            .addr();
        copy_vec(e, implementation + 0xe0, target + 0xe0);
    }
    hull
}

/// Builds the hull `*out` from the points in `array` (`00d81510`, `keep_normal`, the grid of
/// `implementation`) and deletes it (virtual slot 0 with `1`) and clears `*out` when the
/// dimension count of the result differs from that of `implementation`; answers whether it
/// was kept.
fn build_split_hull(
    e: &mut Engine,
    implementation: u32,
    out: u32,
    array: u32,
    keep_normal: u32,
) -> bool {
    let hull = Ptr::<HkgpConvexHull>::new(e.mem.u32(out));
    let target = e.get(hull, HkgpConvexHull::implementation);
    let data = e.mem.u32(array);
    let size = e.mem.i32(array + 4);
    let dimensions =
        hkgp_convex_hull_impl_build(e, target, Ptr::new(data), size, keep_normal, implementation);
    dimensions == e.mem.i32(implementation + 0x1a8)
}

/// Deletes the hull `*out` when it is not null and clears `*out`.
fn drop_split_hull(e: &mut Engine, out: u32) {
    let hull = e.mem.u32(out);
    delete_object(e, hull);
    e.mem.set_u32(out, 0);
}

/// The polygon case of `splitByPlane` (two dimensions): each vertex (list `+0x24`) gets the
/// distance `d = (z * n.z + w) + (y * n.y + x * n.x)` to the plane in `+0x2c`, `0.0` when `|d| <=
/// tolerance`; those with `d < 0` and the rest (NaN included) are counted. No negative: answers
/// 1 when there is a positive one, else 0; no positive: 2. Otherwise the polygon is cut: each
/// edge from the previous vertex (the last for the first) whose distances have a negative
/// product adds the point `(1 - t) * previous + t * current`, `t = d_prev / (d_prev - d_cur)`, to
/// both arrays; a vertex with `d <= 0` goes to the first array and one with `d >= 0` to the
/// second. The first array (non-positive side) builds `*second_out` and the second (non-negative)
/// `*first_out` (`00d81510` with `keep_normal` and the grid of this hull; a result with another
/// dimension count deletes the new hull). Answers 3.
fn split_polygon(
    e: &mut Engine,
    implementation: u32,
    plane: u32,
    first_out: u32,
    second_out: u32,
    tolerance: f32,
) -> u32 {
    let plane_vector = load_vec(e, plane);
    let (mut negative, mut positive) = (0i32, 0i32);
    let head = e.mem.u32(implementation + 0x24);
    if head == 0 {
        return 0;
    }
    let mut vertex = head;
    while vertex != 0 {
        let product = mul(load_vec(e, vertex + 0x10), plane_vector);
        let distance = (product[2] + plane_vector[3]) + (product[1] + product[0]);
        e.mem.set_f32(vertex + 0x2c, x87_quiet(distance));
        let magnitude = fld(distance).abs() as f32;
        if fld(magnitude) <= fld(tolerance) {
            e.mem.set_u32(vertex + 0x2c, 0);
        } else if fld(distance) < 0.0 {
            negative += 1;
        } else {
            positive += 1;
        }
        vertex = e.mem.u32(vertex);
    }
    if negative == 0 {
        return u32::from(positive != 0);
    }
    if positive == 0 {
        return 2;
    }
    e.with_stack(0x10, |e, first| {
        e.with_stack(0x10, |e, second| {
            let (first, second) = (first.addr(), second.addr());
            for array in [first, second] {
                e.mem.set_u32(array, 0);
                e.mem.set_u32(array + 4, 0);
                e.mem.set_u32(array + 8, 0x8000_0000);
            }
            let ones = load_vec(e, SEGMENT_ONES);
            let last = list_last(e, head);
            let mut previous = last;
            let mut vertex = head;
            while vertex != 0 {
                let current = load_vec(e, vertex + 0x10);
                let current_distance = e.mem.f32(vertex + 0x2c);
                let previous_distance = e.mem.f32(previous + 0x2c);
                if fld(current_distance) * fld(previous_distance) < 0.0 {
                    let t = (fld(previous_distance)
                        / (fld(previous_distance) - fld(current_distance)))
                        as f32;
                    let before = load_vec(e, previous + 0x10);
                    let point = add(mul(sub(ones, splat(t)), before), mul(current, splat(t)));
                    array_push_vector(e, first, point, 0x10);
                    array_push_vector(e, second, point, 0x10);
                }
                if fld(current_distance) <= 0.0 {
                    array_push_vector(e, first, current, 0x10);
                }
                if fld(current_distance) >= 0.0 {
                    array_push_vector(e, second, current, 0x10);
                }
                previous = vertex;
                vertex = e.mem.u32(vertex);
            }
            if second_out != 0 {
                new_split_hull(e, implementation, second_out, true);
                if !build_split_hull(e, implementation, second_out, first, 1) {
                    drop_split_hull(e, second_out);
                }
            }
            if first_out != 0 {
                new_split_hull(e, implementation, first_out, true);
                if !build_split_hull(e, implementation, first_out, second, 1) {
                    drop_split_hull(e, first_out);
                }
            }
            release_array(e, second, 0x10, 0x10);
            e.mem.set_u32(second, 0);
            e.mem.set_u32(second + 8, 0x8000_0000);
            release_array(e, first, 0x10, 0x10);
            e.mem.set_u32(first, 0);
            e.mem.set_u32(first + 8, 0x8000_0000);
        })
    });
    3
}

/// The solid case of `splitByPlane` (three dimensions): `00d7a200` classifies the vertices
/// (distances in `+0x2c`, counts of positive and non-positive ones, the range of the distances).
/// Not both sides: answers 2 when there are more non-positive than positive vertices, else 1
/// when there is a positive one, else 0. Both sides: the range `(a, b)` gives the width `b - a`
/// and `m`, the smaller of `|a|` and `|b|` (`|b|` when equal); when the width is not above the
/// tolerance or `m <= tolerance` the whole hull is on one side (answers 1 when the middle of the
/// range is not negative, else 2). Otherwise the arrays of the positive and of the negative side
/// are filled: every vertex with `d > 0` (both with `d == 0`, the negative side with `d <= 0` or
/// NaN) and for each edge `V -> W` of a triangle (list `+0x3c`) with `d(V) > 0` and `d(V) *
/// d(W) < 0` the crossing point `(1 - t) * V + t * W`, `t = d(V) / (d(V) - d(W))`, in both. The
/// hulls are created (configuration words copied) and built from the arrays (`00d81510`, not
/// keeping the normal, the grid of this hull): the positive side into `*first_out`, the negative
/// into `*second_out`; one whose build gives another dimension count is deleted. Answers 3.
fn split_solid(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    plane: u32,
    first_out: u32,
    second_out: u32,
    tolerance: f32,
) -> u32 {
    let implementation = this.addr();
    e.with_stack(0x20, |e, scratch| {
        let scratch = scratch.addr();
        let (counts, range) = (scratch, scratch + 8);
        e.mem.set_u32(counts, 0);
        e.mem.set_u32(counts + 4, 0);
        let status = fn_00d7a200(
            e,
            this,
            Ptr::new(plane),
            tolerance,
            Ptr::new(counts),
            Ptr::new(range),
        );
        let (positive, non_positive) = (e.mem.i32(counts), e.mem.i32(counts + 4));
        if status != 3 {
            if non_positive > positive {
                return 2;
            }
            return u32::from(positive != 0);
        }
        let (low, high) = (e.mem.f32(range), e.mem.f32(range + 4));
        let width = (fld(high) - fld(low)) as f32;
        let (magnitude_high, magnitude_low) = (fld(high).abs() as f32, fld(low).abs() as f32);
        let smallest = if fld(magnitude_high) > fld(magnitude_low) {
            magnitude_low
        } else {
            magnitude_high
        };
        let split = fld(tolerance) < fld(width) && !(fld(smallest) <= fld(tolerance));
        e.with_stack(0x10, |e, positive_side| {
            e.with_stack(0x10, |e, negative_side| {
                let (positive_side, negative_side) = (positive_side.addr(), negative_side.addr());
                for array in [positive_side, negative_side] {
                    e.mem.set_u32(array, 0);
                    e.mem.set_u32(array + 4, 0);
                    e.mem.set_u32(array + 8, 0x8000_0000);
                }
                let release_both = |e: &mut Engine| {
                    release_array(e, negative_side, 0x10, 0x10);
                    e.mem.set_u32(negative_side, 0);
                    e.mem.set_u32(negative_side + 8, 0x8000_0000);
                    release_array(e, positive_side, 0x10, 0x10);
                    e.mem.set_u32(positive_side, 0);
                    e.mem.set_u32(positive_side + 8, 0x8000_0000);
                };
                if !split {
                    let middle = (fld(low) + fld(high)) * e.global::<f64>(HALF_DOUBLE);
                    release_both(e);
                    return if middle >= 0.0 { 1 } else { 2 };
                }
                let count = e.mem.i32(implementation + 0x28);
                let quarter = count.wrapping_add((count >> 31) & 3) >> 2;
                array_reserve_for(e, positive_side, quarter.wrapping_add(positive), 0x10);
                array_reserve_for(e, negative_side, quarter.wrapping_add(non_positive), 0x10);
                let ones = load_vec(e, SEGMENT_ONES);
                let mut vertex = e.mem.u32(implementation + 0x24);
                while vertex != 0 {
                    let distance = e.mem.f32(vertex + 0x2c);
                    let position = load_vec(e, vertex + 0x10);
                    if fld(distance) == 0.0 {
                        array_push_vector(e, positive_side, position, 0x10);
                        array_push_vector(e, negative_side, position, 0x10);
                    } else if fld(distance) > 0.0 {
                        array_push_vector(e, positive_side, position, 0x10);
                    } else {
                        array_push_vector(e, negative_side, position, 0x10);
                    }
                    vertex = e.mem.u32(vertex);
                }
                let mut triangle = e.mem.u32(implementation + 0x3c);
                while triangle != 0 {
                    for corner in 0..3u32 {
                        let start = e.mem.u32(triangle + 8 + 4 * corner);
                        let start_distance = e.mem.f32(start + 0x2c);
                        if !(fld(start_distance) > 0.0) {
                            continue;
                        }
                        let next = (9u32 >> ((corner * 2) & 0x1f)) & 3;
                        let end = e.mem.u32(triangle + 8 + 4 * next);
                        let end_distance = e.mem.f32(end + 0x2c);
                        if fld(end_distance) * fld(start_distance) < 0.0 {
                            let t = (fld(start_distance)
                                / (fld(start_distance) - fld(end_distance)))
                                as f32;
                            let from = load_vec(e, start + 0x10);
                            let to = load_vec(e, end + 0x10);
                            let point = add(mul(sub(ones, splat(t)), from), mul(to, splat(t)));
                            array_push_vector(e, positive_side, point, 0x10);
                            array_push_vector(e, negative_side, point, 0x10);
                        }
                    }
                    triangle = e.mem.u32(triangle);
                }
                if first_out != 0 {
                    new_split_hull(e, implementation, first_out, false);
                }
                if second_out != 0 {
                    new_split_hull(e, implementation, second_out, false);
                }
                let first_kept = first_out != 0
                    && build_split_hull(e, implementation, first_out, positive_side, 0);
                let second_kept = second_out != 0
                    && build_split_hull(e, implementation, second_out, negative_side, 0);
                if !first_kept && first_out != 0 {
                    drop_split_hull(e, first_out);
                }
                if !second_kept && second_out != 0 {
                    drop_split_hull(e, second_out);
                }
                release_both(e);
                3
            })
        })
    })
}

// Translated from 00d83430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::splitByPlane<hkgpConvexHullImpl::PositionAsSource>` (Xbox PDB; `this` =
/// the implementation, stack `(plane, first_out, second_out, tolerance)`: `plane` four floats,
/// the outs addresses of hull pointers or 0; answers 0 (nothing on the positive side), 1
/// (all positive), 2 (all non-positive), 3 (split), 4 (dimension not handled)). Both outs are
/// cleared first. By dimension (`+0x1a8`): 2 `split_polygon`, 3 `split_solid`, otherwise the report
/// "Current dimension not implemented for that operation" (id `3a86e3ef`, line `0x854`, `INT3` at
/// `00d834b7`) and 4.
pub fn hkgp_convex_hull_impl_split_by_plane(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    plane: u32,
    first_out: u32,
    second_out: u32,
    tolerance: f32,
) -> u32 {
    let implementation = this.addr();
    if first_out != 0 {
        e.mem.set_u32(first_out, 0);
    }
    if second_out != 0 {
        e.mem.set_u32(second_out, 0);
    }
    match e.mem.i32(implementation + 0x1a8) {
        2 => split_polygon(e, implementation, plane, first_out, second_out, tolerance),
        3 => split_solid(e, this, plane, first_out, second_out, tolerance),
        _ => {
            report_text(
                e,
                MESSAGE_SPLIT_UNSUPPORTED,
                3,
                0x3a86_e3ef,
                REPORT_FILE,
                0x854,
                Some(0x00d8_34b7),
            );
            4
        }
    }
}

/// `"Operation not implemented for that dimension"`.
const MESSAGE_OPERATION_NOT_IMPLEMENTED: u32 = 0x010d_37a8;

/// Appends the position vectors (`+0x10`) of the vertices (list `+0x24`) to the `hkArray`
/// `array` of vectors, growing it first.
fn append_vertex_positions(e: &mut Engine, implementation: u32, array: u32) {
    let count = e.mem.i32(implementation + 0x28);
    let old_size = array_append(e, array, count, 0x10);
    let mut destination = (old_size as u32)
        .wrapping_shl(4)
        .wrapping_add(e.mem.u32(array));
    let mut vertex = e.mem.u32(implementation + 0x24);
    while vertex != 0 {
        copy_vec(e, vertex + 0x10, destination);
        destination = destination.wrapping_add(0x10);
        vertex = e.mem.u32(vertex);
    }
}

/// Resets the `hkArray` header at `array` to an empty non-owning array.
fn reset_array_header(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// The squared distance between the positions of two vertices as the exe sums it (`z + (y +
/// x)`).
fn vertex_distance_squared(e: &Engine, a: u32, b: u32) -> f32 {
    let difference = sub(load_vec(e, a + 0x10), load_vec(e, b + 0x10));
    sum3(mul(difference, difference))
}

// Translated from 00d83f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpConvexHullImpl::simplify` (Xbox PDB; `this` = the implementation, stack `(parameters)`:
/// `+0` target vertex count, `+4` byte (`contain`), `+8` float area tolerance, `+0x10` byte
/// (merge faces), `+0x11` byte (already changed)). Fewer than three dimensions does nothing; other
/// than three (or a negative count) is the report "Operation not implemented for that dimension"
/// (id `3e0abbab`, line `0x57b`, `INT3` at `00d84745`). Three dimensions:
///
/// * with a target of at least 4 the vertices are decimated to it (`00d82ad0`; `changed` is
///   whether their number changed);
/// * with a positive tolerance, repeatedly (while the hull stays three dimensional) the
///   triangle of the least area (the cross product of two edges, its length, below twice the
///   tolerance) has the vertex opposite to its longest edge (the first of equal) marked (`+0x34` =
///   `-1`) and the hull is rebuilt (`00d81510`) from the unmarked vertex positions (and
///   `buildIndices` when it is solid); `changed` is set; no such triangle ends it;
/// * with the merge byte set, the indices must exist (the "No index available" report, `INT3`
///   at `00d84313`) and, repeatedly, `generateIndexedFaces` (`00d7df60`, no ids, no reorder)
///   gives the outlines of the faces; the distinct vertex numbers in them are collected and,
///   when fewer than the hull's vertices, the hull is rebuilt from the positions of those
///   vertices (`changed` set);
/// * when `changed` and the hull is still three dimensional, the hull is rebuilt once more from
///   all vertex positions and the indices and mass properties rebuilt when they existed before.
pub fn hkgp_convex_hull_impl_simplify(
    e: &mut Engine,
    this: Ptr<HkgpConvexHullImpl>,
    parameters: Ptr,
) {
    let implementation = this.addr();
    let parameters = parameters.addr();
    let mass_valid = e.mem.u8(implementation + 0x1b4);
    let indices_valid = e.mem.u8(implementation + 0x1b5);
    let mut changed = e.mem.u8(parameters + 0x11) != 0;
    let dimensions = e.mem.i32(implementation + 0x1a8);
    if dimensions >= 0 {
        if dimensions < 3 {
            return;
        }
        if dimensions == 3 {
            let target = e.mem.i32(parameters);
            if target >= 4 {
                let before = e.mem.u32(implementation + 0x28);
                let contain = u32::from(e.mem.u8(parameters + 4) != 0);
                hkgp_convex_hull_impl_decimate_vertices(e, this, target, contain);
                changed = e.mem.u32(implementation + 0x28) != before;
            }
            let tolerance = e.mem.f32(parameters + 8);
            if 0.0 < fld(tolerance) {
                let limit = (fld(tolerance) + fld(tolerance)) as f32;
                loop {
                    let mut best_area = limit;
                    let mut best = 0u32;
                    let mut triangle = e.mem.u32(implementation + 0x3c);
                    while triangle != 0 {
                        let positions = [0u32, 1, 2].map(|corner| {
                            let vertex = e.mem.u32(triangle + 8 + 4 * corner);
                            load_vec(e, vertex + 0x10)
                        });
                        let across = cross_yzx(
                            sub(positions[1], positions[0]),
                            sub(positions[2], positions[0]),
                        );
                        let area = sum3(mul(across, across)).sqrt();
                        if fld(area) < fld(best_area) {
                            best_area = area;
                            best = triangle;
                        }
                        triangle = e.mem.u32(triangle);
                    }
                    if best == 0 {
                        break;
                    }
                    let vertices = [0u32, 1, 2].map(|corner| e.mem.u32(best + 8 + 4 * corner));
                    let mut longest = vertex_distance_squared(e, vertices[0], vertices[1]);
                    let mut longest_edge = 0usize;
                    for edge in 1..3usize {
                        let other = vertices[(edge + 1) % 3];
                        let length = vertex_distance_squared(e, vertices[edge], other);
                        if fld(longest) < fld(length) {
                            longest = length;
                            longest_edge = edge;
                        }
                    }
                    let opposite = match longest_edge {
                        0 => vertices[2],
                        1 => vertices[0],
                        _ => vertices[1],
                    };
                    e.mem.set_u32(opposite + 0x34, 0xffff_ffff);
                    e.with_stack(0x10, |e, positions| {
                        let positions = positions.addr();
                        reset_array_header(e, positions);
                        let count = e.mem.i32(implementation + 0x28);
                        array_reserve_for(e, positions, count, 0x10);
                        let mut vertex = e.mem.u32(implementation + 0x24);
                        while vertex != 0 {
                            if e.mem.u32(vertex + 0x34) != 0xffff_ffff {
                                let slot = array_take_slot(e, positions, 0x10, 0x10);
                                copy_vec(e, vertex + 0x10, slot);
                            }
                            vertex = e.mem.u32(vertex);
                        }
                        let data = e.mem.u32(positions);
                        let size = e.mem.i32(positions + 4);
                        let result =
                            hkgp_convex_hull_impl_build(e, this, Ptr::new(data), size, 0, 0);
                        if result == 3 {
                            hkgp_convex_hull_impl_build_indices(e, this);
                        }
                        changed = true;
                        release_array(e, positions, 0x10, 0x10);
                    });
                    if e.mem.i32(implementation + 0x1a8) != 3 {
                        break;
                    }
                }
            }
            if e.mem.u8(parameters + 0x10) != 0 && e.mem.i32(implementation + 0x1a8) == 3 {
                assert_indices(e, this, 0x00d8_4313);
                e.with_stack(0x30, |e, frame| {
                    let frame = frame.addr();
                    let (faces, indices, unique) = (frame, frame + 0x10, frame + 0x20);
                    for array in [faces, indices, unique] {
                        reset_array_header(e, array);
                    }
                    loop {
                        e.mem.set_u32(faces + 4, 0);
                        e.mem.set_u32(indices + 4, 0);
                        e.mem.set_u32(unique + 4, 0);
                        hkgp_convex_hull_impl_generate_indexed_faces(
                            e,
                            this,
                            Ptr::new(faces),
                            Ptr::new(indices),
                            0,
                            0,
                        );
                        let index_count = e.mem.i32(indices + 4);
                        for position in 0..index_count {
                            let value = e
                                .mem
                                .u32(e.mem.u32(indices).wrapping_add(4 * position as u32));
                            let known = (0..e.mem.i32(unique + 4)).any(|slot| {
                                e.mem.u32(e.mem.u32(unique).wrapping_add(4 * slot as u32)) == value
                            });
                            if !known {
                                let slot = array_take_slot(e, unique, 4, 0x10);
                                e.mem.set_u32(slot, value);
                            }
                        }
                        let unique_count = e.mem.i32(unique + 4);
                        if unique_count >= e.mem.i32(implementation + 0x28) {
                            break;
                        }
                        changed = true;
                        e.with_stack(0x20, |e, scratch| {
                            let (all, kept) = (scratch.addr(), scratch.addr() + 0x10);
                            reset_array_header(e, all);
                            reset_array_header(e, kept);
                            append_vertex_positions(e, implementation, all);
                            array_reserve_for(e, kept, unique_count, 0x10);
                            e.mem.set_i32(kept + 4, unique_count);
                            for slot in 0..unique_count {
                                let source =
                                    e.mem.u32(e.mem.u32(unique).wrapping_add(4 * slot as u32));
                                let from = e.mem.u32(all).wrapping_add(source.wrapping_shl(4));
                                let to = e.mem.u32(kept).wrapping_add((slot as u32) << 4);
                                copy_vec(e, from, to);
                            }
                            let data = e.mem.u32(kept);
                            let result = hkgp_convex_hull_impl_build(
                                e,
                                this,
                                Ptr::new(data),
                                unique_count,
                                0,
                                0,
                            );
                            if result == 3 {
                                hkgp_convex_hull_impl_build_indices(e, this);
                            }
                            release_array(e, kept, 0x10, 0x10);
                            release_array(e, all, 0x10, 0x10);
                        });
                        if e.mem.i32(implementation + 0x1a8) != 3 {
                            break;
                        }
                    }
                    release_array(e, unique, 4, 0x10);
                    release_array(e, indices, 4, 0x10);
                    release_array(e, faces, 4, 0x10);
                });
            }
            if !changed || e.mem.i32(implementation + 0x1a8) != 3 {
                return;
            }
            e.with_stack(0x10, |e, positions| {
                let positions = positions.addr();
                reset_array_header(e, positions);
                append_vertex_positions(e, implementation, positions);
                let data = e.mem.u32(positions);
                let size = e.mem.i32(positions + 4);
                hkgp_convex_hull_impl_build(e, this, Ptr::new(data), size, 0, 0);
                if indices_valid != 0 {
                    hkgp_convex_hull_impl_build_indices(e, this);
                }
                if mass_valid != 0 {
                    hkgp_convex_hull_impl_build_mass_properties(e, this);
                }
                release_array(e, positions, 0x10, 0x10);
            });
            return;
        }
    }
    report_text(
        e,
        MESSAGE_OPERATION_NOT_IMPLEMENTED,
        3,
        0x3e0a_bbab,
        REPORT_FILE,
        0x57b,
        Some(0x00d8_4745),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d7_f060, deleting_destructor_00d7f060(Ptr, u8) -> Ptr),
        entry!(0x00d7_4c70, fn_00d74c70(Ptr, f32, Ptr)),
        entry!(0x00d7_4d40, hkgp_convex_hull_build_config_build_config(Ptr) -> Ptr),
        entry!(0x00d7_4d90, fn_00d74d90(Ptr) -> Ptr),
        entry!(0x00d7_4dd0, fn_00d74dd0(Ptr<HkgpConvexHull>)),
        entry!(
            0x00d7_4e00,
            fn_00d74e00(Ptr<HkgpConvexHull>) -> Ptr<HkgpConvexHullImpl>
        ),
        entry!(
            0x00d7_4e10,
            hkgp_convex_hull_get_dimensions(Ptr<HkgpConvexHull>) -> i32
        ),
        entry!(0x00d7_4e50, fn_00d74e50(Ptr<HkgpConvexHull>, Ptr) -> Ptr),
        entry!(
            0x00d7_4e80,
            fn_00d74e80(Ptr<HkgpConvexHull>, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00d7_4ed0, fn_00d74ed0(Ptr) -> Ptr),
        entry!(0x00d7_4f10, fn_00d74f10(Ptr<HkgpConvexHull>) -> i32),
        entry!(0x00d7_4fb0, fn_00d74fb0(Ptr<HkgpConvexHull>, i32) -> u32),
        entry!(0x00d7_5050, fn_00d75050(Ptr<HkgpConvexHull>) -> i32),
        entry!(0x00d7_5060, fn_00d75060(Ptr<HkgpConvexHull>) -> Ptr),
        entry!(0x00d7_5120, fn_00d75120(Ptr) -> u32),
        entry!(0x00d7_5130, fn_00d75130(Ptr<HkgpConvexHull>) -> u32),
        entry!(0x00d7_5140, fn_00d75140(Ptr<HkgpConvexHull>) -> u32),
        entry!(0x00d7_5200, fn_00d75200(Ptr) -> u32),
        entry!(0x00d7_52c0, fn_00d752c0(Ptr, u32) -> u32),
        entry!(
            0x00d7_52e0,
            hkgp_convex_hull_get_volume(Ptr<HkgpConvexHull>) -> f32
        ),
        entry!(
            0x00d7_5390,
            hkgp_convex_hull_get_surface_area(Ptr<HkgpConvexHull>) -> f32
        ),
        entry!(
            0x00d7_5440,
            hkgp_convex_hull_get_sphericity(Ptr<HkgpConvexHull>) -> f32
        ),
        entry!(0x00d7_5490, fn_00d75490(Ptr<HkgpConvexHull>, Ptr, Ptr)),
        entry!(0x00d7_5560, fn_00d75560(Ptr<HkgpConvexHull>) -> u32),
        entry!(0x00d7_5600, fn_00d75600(Ptr<HkgpConvexHull>) -> u32),
        entry!(0x00d7_56a0, fn_00d756a0(Ptr<HkgpConvexHull>, Ptr)),
        entry!(0x00d7_5850, fn_00d75850(Ptr<HkgpConvexHull>)),
        entry!(0x00d7_5880, fn_00d75880(Ptr<HkgpConvexHull>, Ptr) -> f32),
        entry!(
            0x00d7_5920,
            fn_00d75920(Ptr<HkgpConvexHull>, Ptr, u32, f32, f32)
        ),
        entry!(
            0x00d7_5b80,
            fn_00d75b80(Ptr<HkgpConvexHull>, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00d7_5cb0, fn_00d75cb0(Ptr<HkgpConvexHull>, u32, Ptr, Ptr)),
        entry!(0x00d7_5e30, fn_00d75e30(Ptr<HkgpConvexHull>, u32, Ptr, Ptr)),
        entry!(0x00d7_5fc0, fn_00d75fc0(Ptr<HkgpConvexHull>, Ptr) -> bool),
        entry!(
            0x00d7_6100,
            fn_00d76100(Ptr<HkgpConvexHull>, Ptr, u32, f32) -> bool
        ),
        entry!(
            0x00d7_6120,
            hkgp_convex_hull_fetch_positions(Ptr<HkgpConvexHull>, u32, Ptr)
        ),
        entry!(
            0x00d7_6250,
            hkgp_convex_hull_fetch_planes(Ptr<HkgpConvexHull>, Ptr)
        ),
        entry!(0x00d7_6320, fn_00d76320(Ptr<HkgpConvexHull>, Ptr, Ptr)),
        entry!(0x00d7_6840, fn_00d76840(Ptr<HkgpConvexHull>, Ptr) -> f32),
        entry!(0x00d7_68b0, fn_00d768b0(Ptr<HkgpConvexHull>, Ptr) -> f32),
        entry!(0x00d7_69c0, fn_00d769c0(Ptr<HkgpConvexHull>, Ptr) -> f32),
        entry!(0x00d7_a180, fn_00d7a180(Ptr<HkgpConvexHullImpl>) -> Ptr),
        entry!(0x00d7_52d0, fn_00d752d0(Ptr<HkgpConvexHull>) -> Ptr),
        entry!(
            0x00d7_6410,
            hkgp_convex_hull_generate_geometry(Ptr<HkgpConvexHull>, u32, Ptr)
        ),
        entry!(0x00d7_6940, hkgp_convex_hull_destruct(Ptr<HkgpConvexHull>)),
        entry!(
            0x00d7_6990,
            hkgp_convex_hull_build_indices(Ptr<HkgpConvexHull>, f32, u8)
        ),
        entry!(0x00d7_6a20, fn_00d76a20(Ptr<HkgpConvexHull>, Ptr)),
        entry!(
            0x00d7_6ca0,
            hkgp_convex_hull_construct(Ptr<HkgpConvexHull>) -> Ptr<HkgpConvexHull>
        ),
        entry!(
            0x00d7_6cf0,
            hkgp_convex_hull_clone(Ptr<HkgpConvexHull>) -> Ptr<HkgpConvexHull>
        ),
        entry!(
            0x00d7_6d80,
            hkgp_convex_hull_impl_get_oriented_bounding_box(Ptr<HkgpConvexHull>, Ptr, Ptr)
        ),
        entry!(
            0x00d7_6d90,
            hkgp_convex_hull_get_best_projection_plane(Ptr<HkgpConvexHull>, Ptr) -> Ptr
        ),
        entry!(
            0x00d7_6fa0,
            hkgp_convex_hull_create_random_samples(Ptr<HkgpConvexHull>, Ptr, i32, Ptr)
        ),
        entry!(
            0x00d7_71c0,
            hkgp_convex_hull_build(Ptr<HkgpConvexHull>, Ptr, i32, Ptr) -> i32
        ),
        entry!(
            0x00d7_7200,
            hkgp_convex_hull_build_ov2(Ptr<HkgpConvexHull>, Ptr, Ptr) -> i32
        ),
        entry!(
            0x00d7_7320,
            fn_00d77320(Ptr<HkgpConvexHull>, Ptr, i32, Ptr, Ptr) -> i32
        ),
        entry!(
            0x00d7_7380,
            fn_00d77380(Ptr<HkgpConvexHull>, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(
            0x00d7_74a0,
            hkgp_convex_hull_build_from_planes(Ptr<HkgpConvexHull>, Ptr, i32, Ptr) -> i32
        ),
        entry!(
            0x00d7_7840,
            hkgp_convex_hull_build_from_planes_ov2(Ptr<HkgpConvexHull>, Ptr, i32, Ptr, Ptr) -> i32
        ),
        entry!(
            0x00d7_62f0,
            hkgp_convex_hull_build_mass_properties(Ptr<HkgpConvexHull>) -> bool
        ),
        entry!(0x00d7_8720, fn_00d78720(Ptr<HkgpConvexHull>, f32, Ptr)),
        entry!(
            0x00d7_7bd0,
            fn_00d77bd0(Ptr<HkgpConvexHull>, i32, u32) -> bool
        ),
        entry!(
            0x00d7_8b90,
            fn_00d78b90(Ptr<HkgpConvexHull>, f32, u32) -> Ptr<HkgpConvexHull>
        ),
        entry!(0x00d7_8ca0, fn_00d78ca0(Ptr, Ptr, Ptr) -> Ptr<HkgpConvexHull>),
        entry!(0x00d7_8ea0, fn_00d78ea0(u32, Ptr, i32) -> Ptr<HkgpConvexHull>),
        entry!(0x00d7_9060, fn_00d79060(u32, Ptr, Ptr) -> Ptr<HkgpConvexHull>),
        entry!(
            0x00d7_90a0,
            fn_00d790a0(Ptr<HkgpConvexHull>, u32, u32, u32, f32) -> u32
        ),
        entry!(0x00d7_90d0, fn_00d790d0(Ptr<HkgpConvexHull>, i32, Ptr, f32)),
        entry!(
            0x00d7_9250,
            fn_00d79250(Ptr<HkgpConvexHull>, Ptr<HkgpConvexHull>, Ptr, Ptr, f32)
        ),
        entry!(
            0x00d7_9510,
            fn_00d79510(Ptr<HkgpConvexHull>, Ptr<HkgpConvexHull>, Ptr, f32) -> bool
        ),
        entry!(
            0x00d7_9580,
            fn_00d79580(Ptr<HkgpConvexHull>, Ptr, Ptr, f32) -> bool
        ),
        entry!(
            0x00d7_9850,
            fn_00d79850(Ptr<HkgpConvexHull>, Ptr<HkgpConvexHull>, Ptr, Ptr, Ptr) -> f32
        ),
        entry!(0x00d7_9ab0, fn_00d79ab0(Ptr<HkgpConvexHull>, Ptr, Ptr, f32)),
        entry!(
            0x00d7_9f90,
            fn_00d79f90(Ptr<HkgpConvexHull>, Ptr, Ptr, Ptr, f32)
        ),
        entry!(0x00d7_a010, fn_00d7a010(Ptr<HkgpConvexHull>, Ptr, Ptr, f32)),
        entry!(
            0x00d7_a0b0,
            fn_00d7a0b0(Ptr<HkgpConvexHullImpl>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d7_a200,
            fn_00d7a200(Ptr<HkgpConvexHullImpl>, Ptr, f32, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00d7_a350,
            fn_00d7a350(Ptr<HkgpConvexHullImpl>, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00d7_a420, fn_00d7a420(u32, i32, i32, Ptr)),
        entry!(
            0x00d7_a4b0,
            hk_pseudo_random_generator_get_random_vector11(Ptr, Ptr)
        ),
        entry!(
            0x00d7_a5a0,
            fn_00d7a5a0(Ptr<HkgpConvexHullImpl>, Ptr, Ptr, Ptr, Ptr, f32) -> bool
        ),
        entry!(
            0x00d7_a810,
            fn_00d7a810(Ptr<HkgpConvexHullImpl>, Ptr, Ptr, Ptr) -> f32
        ),
        entry!(
            0x00d7_aa30,
            hkgp_convex_hull_impl_get_centroid(Ptr<HkgpConvexHullImpl>) -> Ptr
        ),
        entry!(0x00d7_aaf0, fn_00d7aaf0(u32, i32, i32, Ptr)),
        entry!(
            0x00d7_adb0,
            hk_geometry_processing_pool_allocator_vertex_release(Ptr, u32)
        ),
        entry!(0x00d7_aea0, fn_00d7aea0(Ptr, u32)),
        entry!(
            0x00d7_ac40,
            hkgp_convex_hull_impl_reset(Ptr<HkgpConvexHullImpl>, u8)
        ),
        entry!(0x00d7_af90, fn_00d7af90(u32, i32, i32, u32)),
        entry!(0x00d7_b160, fn_00d7b160(u32, i32, i32, u32)),
        entry!(0x00d7_b270, fn_00d7b270(u32, i32, i32, u32)),
        entry!(
            0x00d7_b330,
            fn_00d7b330(Ptr<HkgpConvexHullImpl>, Ptr, u8, f32) -> bool
        ),
        entry!(0x00d7_c200, fn_00d7c200(Ptr)),
        entry!(0x00d7_c250, fn_00d7c250(Ptr)),
        entry!(0x00d7_c850, fn_00d7c850(Ptr)),
        entry!(0x00d7_c8d0, fn_00d7c8d0(Ptr)),
        entry!(0x00d7_c2a0, hk_geometry_processing_pool_allocator_vertex_allocate_new_pool(Ptr) -> u32),
        entry!(0x00d7_c340, hk_geometry_processing_pool_allocator_triangle_allocate_new_pool(Ptr) -> u32),
        entry!(0x00d7_c950, hkgp_abstract_mesh_reset(Ptr)),
        entry!(0x00d7_d1c0, hkgp_abstract_mesh_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00d7_c970, fn_00d7c970(Ptr)),
        entry!(
            0x00d7_ce20,
            hkgp_convex_hull_impl_destruct(Ptr<HkgpConvexHullImpl>)
        ),
        entry!(0x00d7_d220, fn_00d7d220(Ptr, u32, u32) -> Ptr),
        entry!(0x00d7_ef20, fn_00d7ef20(Ptr, Ptr) -> u32),
        entry!(0x00d7_efc0, fn_00d7efc0(Ptr, Ptr) -> u32),
        entry!(
            0x00d7_c610,
            fn_00d7c610(Ptr<HkgpConvexHullImpl>, Ptr, Ptr, Ptr) -> f32
        ),
        entry!(
            0x00d7_c3e0,
            hkgp_convex_hull_impl_clear_horizon(Ptr<HkgpConvexHullImpl>, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d7_cf00, fn_00d7cf00(Ptr<HkgpConvexHullImpl>) -> f32),
        entry!(0x00d7_caa0, fn_00d7caa0(Ptr<HkgpConvexHullImpl>, f32, Ptr)),
        entry!(
            0x00d7_b600,
            hkgp_convex_hull_impl_build_mass_properties(Ptr<HkgpConvexHullImpl>) -> bool
        ),
        entry!(0x00d7_f6a0, hkgp_convex_hull_impl_construct(Ptr) -> Ptr<HkgpConvexHullImpl>),
        entry!(
            0x00d7_f720,
            hkgp_convex_hull_impl_clone(Ptr<HkgpConvexHullImpl>) -> Ptr<HkgpConvexHullImpl>
        ),
        entry!(
            0x00d7_f090,
            fn_00d7f090(Ptr<HkgpConvexHullImpl>, Ptr, Ptr, Ptr) -> f32
        ),
        entry!(
            0x00d8_1340,
            hkgp_convex_hull_impl_get_oriented_rectangle(Ptr<HkgpConvexHullImpl>, Ptr, Ptr)
        ),
        entry!(
            0x00d7_7be0,
            hkgp_convex_hull_expand_by_planar_margin(Ptr<HkgpConvexHull>, f32) -> bool
        ),
        entry!(
            0x00d8_27b0,
            hkgp_convex_hull_impl_ensure_containment(Ptr<HkgpConvexHullImpl>, Ptr)
        ),
        entry!(
            0x00d7_d2a0,
            hkgp_convex_hull_impl_build_indices(Ptr<HkgpConvexHullImpl>)
        ),
        entry!(
            0x00d7_df60,
            hkgp_convex_hull_impl_generate_indexed_faces(
                Ptr<HkgpConvexHullImpl>,
                Ptr,
                Ptr,
                u8,
                u8,
            ) -> i32
        ),
        entry!(
            0x00d7_fb80,
            hkgp_convex_hull_impl_flush_pending(Ptr<HkgpConvexHullImpl>) -> bool
        ),
        entry!(
            0x00d8_1510,
            hkgp_convex_hull_impl_build(Ptr<HkgpConvexHullImpl>, Ptr, i32, u32, u32) -> i32
        ),
        entry!(
            0x00d8_2ad0,
            hkgp_convex_hull_impl_decimate_vertices(Ptr<HkgpConvexHullImpl>, i32, u32) -> bool
        ),
        entry!(
            0x00d8_3430,
            hkgp_convex_hull_impl_split_by_plane(
                Ptr<HkgpConvexHullImpl>,
                u32,
                u32,
                u32,
                f32,
            ) -> u32
        ),
        entry!(
            0x00d8_3f30,
            hkgp_convex_hull_impl_simplify(Ptr<HkgpConvexHullImpl>, Ptr)
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
    fn deleting_destructor_00d7f060_frees_only_when_bit_zero_is_set() {
        check(0x00d7_f060, 0x00d7_6940);
    }

    use crate::units::havok_sdk::hkpsphereshape::{HALF_F32, THREE_F32};
    // ---- the convex hull translations ----

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const REPORT_SINK_DOUBLE: u32 = 0x0900_0020;
    const OBJECT_SLOT_DOUBLE: u32 = 0x0900_0030;

    /// A hull (`0x14` bytes) over an implementation block (`0x1c0` bytes); answers both.
    fn hull_with_impl(e: &mut Engine) -> (u32, u32) {
        let implementation = e.mem.alloc(0x1c0);
        let hull = e.mem.alloc(0x14);
        e.mem.set_u32(hull + 8, implementation);
        (hull, implementation)
    }

    fn put_vec(e: &mut Engine, at: u32, v: [f32; 4]) {
        store_vec(e, at, v);
    }

    fn get_vec(e: &Engine, at: u32) -> [f32; 4] {
        load_vec(e, at)
    }

    /// The vertex list `positions` on the implementation `imp`: nodes of `0x30` bytes (next at
    /// `+0`, position at `+0x10`, grid coordinates at `+0x20`), linked in order; answers them.
    fn vertex_list(e: &mut Engine, imp: u32, positions: &[[f32; 4]]) -> Vec<u32> {
        let nodes: Vec<u32> = positions.iter().map(|_| e.mem.alloc(0x30)).collect();
        for (index, position) in positions.iter().enumerate() {
            put_vec(e, nodes[index] + 0x10, *position);
            let next = nodes.get(index + 1).copied().unwrap_or(0);
            e.mem.set_u32(nodes[index], next);
        }
        e.mem
            .set_u32(imp + 0x24, nodes.first().copied().unwrap_or(0));
        e.mem.set_i32(imp + 0x28, positions.len() as i32);
        nodes
    }

    /// Doubles for the failed-assertion report (`REPORT_BUILDER_*` and the sink) that log
    /// every call as (address, argument words); the sink answers `answer`.
    fn install_report(e: &mut Engine, answer: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for address in [
            REPORT_BUILDER_CONSTRUCT,
            REPORT_BUILDER_TEXT,
            REPORT_BUILDER_NUMBER,
            REPORT_BUILDER_SIGNED_NUMBER,
            REPORT_BUILDER_DESTRUCT,
        ] {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, REPORT_SINK_DOUBLE);
        let sink = e.mem.alloc(0x10);
        e.mem.set_u32(sink, vtable);
        e.mem.set_u32(REPORT_SINK, sink);
        let sink_log = Rc::clone(&log);
        e.register_double(REPORT_SINK_DOUBLE, move |_, a| {
            sink_log.borrow_mut().push((REPORT_SINK_DOUBLE, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    /// The report the accessors write: construct, text, number, closing text, sink, destroy.
    fn assert_report(log: &Log, message: u32, number: u32, id: u32, line: u32) {
        let log = log.borrow();
        let addresses: Vec<u32> = log.iter().map(|(address, _)| *address).collect();
        assert_eq!(
            addresses,
            vec![
                REPORT_BUILDER_CONSTRUCT,
                REPORT_BUILDER_TEXT,
                REPORT_BUILDER_NUMBER,
                REPORT_BUILDER_TEXT,
                REPORT_SINK_DOUBLE,
                REPORT_BUILDER_DESTRUCT
            ]
        );
        let builder = log[0].1[0];
        let buffer = log[0].1[1];
        assert_eq!(log[0].1[2], 0x200);
        assert_eq!(log[1].1, vec![builder, message]);
        assert_eq!(log[2].1, vec![builder, number]);
        assert_eq!(log[3].1, vec![builder, REPORT_CLOSE]);
        assert_eq!(log[4].1[1..], [3, id, buffer, REPORT_FILE, line]);
        assert_eq!(log[5].1, vec![builder]);
    }

    fn assert_close(got: f32, want: f32) {
        assert!(
            (got - want).abs() <= 1e-5 * want.abs().max(1.0),
            "got {got}, want {want}"
        );
    }

    fn assert_vec_close(got: [f32; 4], want: [f32; 4]) {
        for lane in 0..4 {
            assert_close(got[lane], want[lane]);
        }
    }

    /// Sets the constants of the grid: the offset (`+0xf0`), scale (`+0x110`) and the zero
    /// float the fourth lane comes from.
    fn grid(e: &mut Engine, imp: u32, offset: [f32; 4], scale: [f32; 4]) {
        put_vec(e, imp + 0xf0, offset);
        put_vec(e, imp + 0x110, scale);
        e.mem.set_f32(ZERO_F32, 0.0);
    }

    #[test]
    fn fn_00d74c70_builds_the_scaling_matrix_along_a_direction() {
        let mut e = engine();
        let vector = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x40);
        put_vec(&mut e, vector, [0.6, 0.0, 0.8, 2.0]);
        for word in 0..16 {
            e.mem.set_u32(out + 4 * word, 0xdead_beef);
        }
        e.call(0x00d7_4c70, &args![vector, 3.0f32, out]);
        let matrix: Vec<f32> = (0..16).map(|i| e.mem.f32(out + 4 * i)).collect();
        // t = scale - 1 = 2: I + t * v * v^T on the first three lanes.
        assert_close(matrix[0], 1.0 + 2.0 * 0.36);
        assert_close(matrix[5], 1.0);
        assert_close(matrix[10], 1.0 + 2.0 * 0.64);
        assert_close(matrix[4], 0.0);
        assert_close(matrix[8], 2.0 * 0.6 * 0.8);
        assert_close(matrix[9], 0.0);
        assert_eq!(matrix[1], matrix[4]);
        assert_eq!(matrix[2], matrix[8]);
        assert_eq!(matrix[6], matrix[9]);
        assert_close(matrix[0xc], 2.0 * 0.6 * 2.0);
        assert_close(matrix[0xd], 0.0);
        assert_close(matrix[0xe], 2.0 * 0.8 * 2.0);
        assert_eq!([matrix[3], matrix[7], matrix[0xb]], [0.0; 3]);
        assert_eq!(matrix[0xf], 1.0);
    }

    #[test]
    fn build_config_defaults_set_only_their_own_bytes() {
        let mut e = engine();
        e.mem.set_u32(BUILD_CONFIG_DEFAULT_FLOAT, 0x3f7f_f972);
        let config = e.mem.alloc(0x10);
        for at in 0..0x10 {
            e.mem.set_u8(config + at, 0xaa);
        }
        let got = e.call(0x00d7_4d40, &args![config]).u32();
        assert_eq!(got, config);
        let bytes: Vec<u8> = (0..0x10).map(|at| e.mem.u8(config + at)).collect();
        assert_eq!(
            bytes,
            [0, 0xaa, 0xaa, 0xaa, 0x72, 0xf9, 0x7f, 0x3f, 0, 1, 0, 1, 0, 0, 0xaa, 0xaa]
        );
    }

    #[test]
    fn fn_00d74d90_sets_a_flag_and_a_float() {
        let mut e = engine();
        e.mem.set_u32(SECOND_CONFIG_DEFAULT_FLOAT, 0x3400_0000);
        let config = e.mem.alloc(0x10);
        let got = e.call(0x00d7_4d90, &args![config]).u32();
        assert_eq!(got, config);
        assert_eq!(e.mem.u32(config), 1);
        assert_eq!(e.mem.u32(config + 4), 0x3400_0000);
        // A signalling NaN is quieted by the x87 copy.
        e.mem.set_u32(SECOND_CONFIG_DEFAULT_FLOAT, 0x7f80_0001);
        e.call(0x00d7_4d90, &args![config]);
        assert_eq!(e.mem.u32(config + 4), 0x7fc0_0001);
    }

    #[test]
    fn fn_00d74dd0_deletes_the_owned_object_when_there_is_one() {
        let mut e = engine();
        let (hull, _) = hull_with_impl(&mut e);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let object_log = Rc::clone(&log);
        e.register_double(OBJECT_SLOT_DOUBLE, move |_, a| {
            object_log
                .borrow_mut()
                .push((OBJECT_SLOT_DOUBLE, a.to_vec()));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, OBJECT_SLOT_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(hull + 0x10, object);
        e.call(0x00d7_4dd0, &args![hull]);
        assert_eq!(*log.borrow(), vec![(OBJECT_SLOT_DOUBLE, vec![object, 1])]);
        assert_eq!(e.mem.u32(hull + 0x10), 0);
        e.call(0x00d7_4dd0, &args![hull]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(e.mem.u32(hull + 0x10), 0);
    }

    #[test]
    fn simple_hull_getters_read_their_implementation_fields() {
        let mut e = engine();
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_i32(imp + 0x1a8, 3);
        e.mem.set_i32(imp + 0x28, 11);
        e.mem.set_u32(imp + 0x24, 0x1234);
        e.mem.set_u32(imp + 0x40, 0x4040);
        e.mem.set_u32(imp + 0x3c, 0x3c3c);
        assert_eq!(e.call(0x00d7_4e00, &args![hull]).u32(), imp);
        assert_eq!(e.call(0x00d7_4e10, &args![hull]).i32(), 3);
        assert_eq!(e.call(0x00d7_5050, &args![hull]).i32(), 11);
        assert_eq!(e.call(0x00d7_5060, &args![hull]).u32(), 0x1234);
        assert_eq!(e.call(0x00d7_5130, &args![hull]).u32(), 0x4040);
        assert_eq!(e.call(0x00d7_5140, &args![hull]).u32(), 0x3c3c);
    }

    #[test]
    fn fn_00d74e50_copies_four_words() {
        let mut e = engine();
        let (hull, imp) = hull_with_impl(&mut e);
        let out = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_u32(imp + 0xe0 + 4 * word, 0x7f80_0001 + word);
        }
        let got = e.call(0x00d7_4e50, &args![hull, out]).u32();
        assert_eq!(got, out);
        for word in 0..4 {
            assert_eq!(e.mem.u32(out + 4 * word), 0x7f80_0001 + word);
        }
    }

    #[test]
    fn fn_00d74e80_maps_grid_coordinates_to_a_position() {
        let mut e = engine();
        let (hull, imp) = hull_with_impl(&mut e);
        grid(&mut e, imp, [10.0, 20.0, 30.0, 40.0], [0.5, 0.25, 2.0, 3.0]);
        let vertex = e.mem.alloc(0x30);
        e.mem.set_i32(vertex + 0x20, 4);
        e.mem.set_i32(vertex + 0x24, -8);
        e.mem.set_i32(vertex + 0x28, 5);
        let out = e.mem.alloc(0x10);
        let got = e.call(0x00d7_4e80, &args![hull, out, vertex]).u32();
        assert_eq!(got, out);
        assert_eq!(get_vec(&e, out), [12.0, 18.0, 40.0, 40.0]);
    }

    #[test]
    fn fn_00d74ed0_is_the_position_inside_a_node() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00d7_4ed0, &args![0x2000_0100u32]).u32(),
            0x2000_0110
        );
    }

    #[test]
    fn plane_accessors_report_missing_indices() {
        let mut e = engine();
        let log = install_report(&mut e, 0);
        let (hull, imp) = hull_with_impl(&mut e);
        let planes = e.mem.alloc(0x40);
        e.mem.set_u32(imp + 0x54, planes);
        e.mem.set_i32(imp + 0x58, 7);
        e.mem.set_u8(imp + 0x1b5, 1);
        assert_eq!(e.call(0x00d7_4f10, &args![hull]).i32(), 7);
        assert_eq!(e.call(0x00d7_4fb0, &args![hull, 2i32]).u32(), planes + 0x20);
        assert!(log.borrow().is_empty());
        e.mem.set_u8(imp + 0x1b5, 0);
        e.mem.set_u32(imp + 0x1b0, 0x55);
        assert_eq!(e.call(0x00d7_4f10, &args![hull]).i32(), 7);
        assert_report(&log, MESSAGE_NO_INDEX, 0x55, 0x79f9_d886, 0x148);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00d7_4fb0, &args![hull, 3i32]).u32(), planes + 0x30);
        assert_report(&log, MESSAGE_NO_INDEX, 0x55, 0x79f9_d886, 0x148);
    }

    #[test]
    fn accessor_assertions_break_at_their_own_address() {
        // (function, arguments built from the hull, address of its INT3)
        type Case = (u32, fn(u32, u32) -> Vec<u32>, u32);
        let cases: [Case; 8] = [
            (0x00d7_4f10, |h, _| args![h].to_vec(), 0x00d7_4f88),
            (0x00d7_4fb0, |h, _| args![h, 1i32].to_vec(), 0x00d7_5028),
            (0x00d7_52e0, |h, _| args![h].to_vec(), 0x00d7_5358),
            (0x00d7_5390, |h, _| args![h].to_vec(), 0x00d7_5408),
            (0x00d7_5490, |h, s| args![h, s, s].to_vec(), 0x00d7_550e),
            (0x00d7_5560, |h, _| args![h].to_vec(), 0x00d7_55d8),
            (0x00d7_5600, |h, _| args![h].to_vec(), 0x00d7_5678),
            (0x00d7_56a0, |h, s| args![h, s].to_vec(), 0x00d7_571e),
        ];
        for (function, build, address) in cases {
            let mut e = engine();
            install_report(&mut e, 1);
            let (hull, _) = hull_with_impl(&mut e);
            let scratch = e.mem.alloc(0x80);
            let words = build(hull, scratch);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(function, &words);
            }));
            let message = *result.unwrap_err().downcast::<String>().unwrap();
            assert!(
                message.contains(&format!("INT3 at {address:08x}")),
                "{function:08x}: {message}"
            );
        }
    }

    #[test]
    fn node_accessors_read_their_words() {
        let mut e = engine();
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(node, 0x1111);
        e.mem.set_u32(node + 8, 0xa);
        e.mem.set_u32(node + 0xc, 0xb);
        e.mem.set_u32(node + 0x10, 0xc);
        assert_eq!(e.call(0x00d7_5120, &args![node]).u32(), 0x1111);
        assert_eq!(e.call(0x00d7_5200, &args![node]).u32(), 0x1111);
        assert_eq!(e.call(0x00d7_52c0, &args![node, 0u32]).u32(), 0xa);
        assert_eq!(e.call(0x00d7_52c0, &args![node, 2u32]).u32(), 0xc);
    }

    #[test]
    fn fn_00d752d0_runs_the_centre_on_the_implementation() {
        let mut e = engine();
        e.mem.set_f64(FLOAT_MAX_DOUBLE, f64::from(f32::MAX));
        let (hull, imp) = hull_with_impl(&mut e);
        // A cached centre (fourth lane not the marker) is answered as it is.
        e.mem.set_f32(imp + 0x7c, 1.0);
        assert_eq!(e.call(0x00d7_52d0, &args![hull]).u32(), imp + 0x70);
    }

    #[test]
    fn mass_property_getters_read_floats_or_report() {
        let mut e = engine();
        let log = install_report(&mut e, 0);
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_f32(imp + 0x1a0, 2.5);
        e.mem.set_f32(imp + 0x1a4, 7.5);
        e.mem.set_u8(imp + 0x1b4, 1);
        assert_eq!(e.call(0x00d7_52e0, &args![hull]).f32(), 2.5);
        assert_eq!(e.call(0x00d7_5390, &args![hull]).f32(), 7.5);
        assert!(log.borrow().is_empty());
        e.mem.set_u8(imp + 0x1b4, 0);
        e.mem.set_u32(imp + 0x1b0, 9);
        assert_eq!(e.call(0x00d7_52e0, &args![hull]).f32(), 2.5);
        assert_report(&log, MESSAGE_NO_MASS_PROPERTIES, 9, 0x79f9_d887, 0x14e);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00d7_5390, &args![hull]).f32(), 7.5);
        assert_report(&log, MESSAGE_NO_MASS_PROPERTIES, 9, 0x79f9_d887, 0x14e);
        // A signalling NaN is quieted by the load.
        e.mem.set_u32(imp + 0x1a0, 0x7f80_0001);
        assert_eq!(
            e.call(0x00d7_52e0, &args![hull]).f32().to_bits(),
            0x7fc0_0001
        );
    }

    #[test]
    fn sphericity_divides_the_power_of_the_volume_by_the_area() {
        let mut e = engine();
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_u8(imp + 0x1b4, 1);
        e.mem.set_f32(imp + 0x1a0, 8.0);
        e.mem.set_f32(imp + 0x1a4, 4.0);
        e.mem.set_f64(SPHERICITY_EXPONENT, 2.0);
        e.mem.set_f64(SPHERICITY_FACTOR, 3.0);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let pow_log = Rc::clone(&log);
        e.register_double(POW, move |_, a| {
            pow_log.borrow_mut().push((POW, a.to_vec()));
            Ret {
                st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32)
                    .powf(f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32)),
                ..Ret::default()
            }
        });
        // pow(8, 2) = 64; 64 * 3 / 4 = 48.
        assert_eq!(e.call(0x00d7_5440, &args![hull]).f32(), 48.0);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        let base = f64::from_bits(u64::from(calls[0].1[0]) | u64::from(calls[0].1[1]) << 32);
        assert_eq!(base, 8.0);
    }

    /// The allocator the hull code reaches through the router (`00c85750`, `+0x10`): a double
    /// router, an allocator object whose slot `0x10` (free) logs `(data, bytes)` under
    /// `FREE_LOG`, and an `hkArray` reserve double (`00c908c0`) that moves the data to a new
    /// block of `capacity * element size` bytes and sets the capacity. Every reserve is logged
    /// as (address, words).
    fn install_memory(e: &mut Engine) -> (Log, Log) {
        let reserves: Log = Rc::new(RefCell::new(Vec::new()));
        let frees: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&frees);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().push((FREE_SLOT_DOUBLE, a.to_vec()));
            Ret::default()
        });
        let reserve_log = Rc::clone(&reserves);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            reserve_log.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let (array, capacity, element) = (a[1], a[2], a[3]);
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            let block = e.mem.alloc(capacity * element);
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(block + byte, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        (reserves, frees)
    }

    #[test]
    fn fn_00d75490_copies_the_mass_property_block() {
        let mut e = engine();
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_u8(imp + 0x1b4, 1);
        for row in 0..5u32 {
            let at = imp + 0x120 + 0x10 * row;
            put_vec(&mut e, at, [row as f32, 1.0, 2.0, 3.0]);
        }
        let matrix = e.mem.alloc(0x40);
        let vector = e.mem.alloc(0x10);
        e.call(0x00d7_5490, &args![hull, matrix, vector]);
        for row in 0..4u32 {
            assert_eq!(
                get_vec(&e, matrix + 0x10 * row),
                [row as f32, 1.0, 2.0, 3.0]
            );
        }
        assert_eq!(get_vec(&e, vector), [4.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn fn_00d75560_and_00d75600_answer_block_addresses() {
        let mut e = engine();
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_u8(imp + 0x1b4, 1);
        assert_eq!(e.call(0x00d7_5560, &args![hull]).u32(), imp + 0x170);
        assert_eq!(e.call(0x00d7_5600, &args![hull]).u32(), imp + 0x150);
    }

    #[test]
    fn fn_00d756a0_picks_the_row_of_the_largest_reciprocal() {
        let mut e = engine();
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_u8(imp + 0x1b4, 1);
        put_vec(&mut e, ONES_VECTOR, [1.0; 4]);
        e.mem.set_u32(ZERO_VECTOR, 0);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        for row in 0..3u32 {
            put_vec(
                &mut e,
                imp + 0x120 + 0x10 * row,
                [row as f32 + 1.0, 0.5, 0.25, 9.0],
            );
        }
        put_vec(&mut e, imp + 0x150, [1.0, 2.0, 3.0, 4.0]);
        let out = e.mem.alloc(0x10);
        // Reciprocals of (v0, v1, v2): the smallest component has the largest reciprocal.
        for (vector, row) in [
            ([1.0, 4.0, 2.0, 0.0], 0u32),
            ([4.0, 1.0, 2.0, 0.0], 1),
            ([2.0, 4.0, 1.0, 0.0], 2),
        ] {
            put_vec(&mut e, imp + 0x160, vector);
            e.call(0x00d7_56a0, &args![hull, out]);
            let want = [row as f32 + 1.0, 0.5, 0.25];
            let got = get_vec(&e, out);
            assert_eq!(got[..3], want);
            assert_eq!(
                got[3],
                0.0 - (want[0] * 1.0 + want[1] * 2.0 + want[2] * 3.0)
            );
        }
    }

    #[test]
    fn fn_00d75880_is_the_largest_signed_distance_to_the_planes() {
        let mut e = engine();
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_u32(NEGATIVE_FLOAT_MAX, 0xff7f_ffee);
        let planes = e.mem.alloc(0x20);
        put_vec(&mut e, planes, [0.0, 1.0, 0.0, -2.0]);
        put_vec(&mut e, planes + 0x10, [1.0, 0.0, 0.0, -1.0]);
        e.mem.set_u32(imp + 0x54, planes);
        e.mem.set_u8(imp + 0x1b5, 1);
        let point = e.mem.alloc(0x10);
        put_vec(&mut e, point, [3.0, 5.0, 0.0, 7.0]);
        e.mem.set_i32(imp + 0x58, 2);
        assert_eq!(e.call(0x00d7_5880, &args![hull, point]).f32(), 3.0);
        // No planes: the starting value.
        e.mem.set_i32(imp + 0x58, 0);
        assert_eq!(
            e.call(0x00d7_5880, &args![hull, point]).f32(),
            f32::from_bits(0xff7f_ffee)
        );
        // A NaN distance replaces the maximum (the x87 flag test only skips a smaller one).
        e.mem.set_i32(imp + 0x58, 2);
        put_vec(&mut e, point, [f32::NAN, 5.0, 0.0, 7.0]);
        assert!(e.call(0x00d7_5880, &args![hull, point]).f32().is_nan());
    }

    /// Constants of the box function: the doubles and floats it reads.
    fn box_constants(e: &mut Engine) {
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f32(HALF_FLOAT, 0.5);
        e.mem.set_f32(ZERO_F32, 0.0);
    }

    fn read_box(e: &Engine, at: u32) -> ([f32; 4], [f32; 4]) {
        (get_vec(e, at), get_vec(e, at + 0x10))
    }

    #[test]
    fn fn_00d75920_builds_and_caches_the_box_of_the_positions() {
        let mut e = engine();
        box_constants(&mut e);
        let (hull, imp) = hull_with_impl(&mut e);
        vertex_list(
            &mut e,
            imp,
            &[
                [0.0, 0.0, 0.0, 0.0],
                [1.0, 2.0, 3.0, 0.0],
                [-1.0, 5.0, 0.0, 0.0],
            ],
        );
        let out = e.mem.alloc(0x20);
        e.call(0x00d7_5920, &args![hull, out, 0u32, 1.0f32, 0.0f32]);
        let want = ([-1.0, 0.0, 0.0, 0.0], [1.0, 5.0, 3.0, 0.0]);
        assert_eq!(read_box(&e, out), want);
        assert_eq!(read_box(&e, imp + 0x90), want);
        // A box that is not empty is not rebuilt.
        put_vec(&mut e, imp + 0x90, [0.0; 4]);
        put_vec(&mut e, imp + 0xa0, [1.0, 1.0, 1.0, 1.0]);
        e.call(0x00d7_5920, &args![hull, out, 0u32, 1.0f32, -1.0f32]);
        assert_eq!(read_box(&e, out), ([0.0; 4], [1.0; 4]));
    }

    #[test]
    fn fn_00d75920_rescales_and_pads_the_box() {
        let mut e = engine();
        box_constants(&mut e);
        let (hull, imp) = hull_with_impl(&mut e);
        // No vertices: the cached box stays as it is.
        put_vec(&mut e, imp + 0x90, [0.0, 0.0, 0.0, 0.0]);
        put_vec(&mut e, imp + 0xa0, [2.0, 4.0, 6.0, 0.0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00d7_5920, &args![hull, out, 0u32, 2.0f32, 0.0f32]);
        assert_eq!(
            read_box(&e, out),
            ([-1.0, -2.0, -3.0, 0.0], [3.0, 6.0, 9.0, 0.0])
        );
        // Margin: 0.5 * margin * diagonal on each of the first three lanes.
        e.call(0x00d7_5920, &args![hull, out, 0u32, 1.0f32, 1.0f32]);
        let step = 0.5 * (4.0f32 + 16.0 + 36.0).sqrt();
        let (low, high) = read_box(&e, out);
        assert_vec_close(low, [-step, -step, -step, 0.0]);
        assert_vec_close(high, [2.0 + step, 4.0 + step, 6.0 + step, 0.0]);
        // Margin and scale together: padded first, then rescaled.
        e.call(0x00d7_5920, &args![hull, out, 0u32, 0.5f32, 1.0f32]);
        let (low, high) = read_box(&e, out);
        let (padded_low, padded_high) = (-step, 2.0 + step);
        let centre = 0.5 * (padded_high + padded_low);
        let reach = 0.25 * (padded_high - padded_low);
        assert_close(low[0], centre - reach);
        assert_close(high[0], centre + reach);
        // The cache itself is untouched by the adjustments.
        assert_eq!(read_box(&e, imp + 0x90), ([0.0; 4], [2.0, 4.0, 6.0, 0.0]));
    }

    #[test]
    fn fn_00d75920_builds_the_box_of_the_grid_positions() {
        let mut e = engine();
        box_constants(&mut e);
        let (hull, imp) = hull_with_impl(&mut e);
        grid(&mut e, imp, [100.0, 0.0, 0.0, 0.0], [2.0, 1.0, 1.0, 1.0]);
        let nodes = vertex_list(&mut e, imp, &[[0.0; 4]; 3]);
        for (node, coordinates) in nodes.iter().zip([[1, 1, 1], [5, -2, 3], [3, 0, -4]]) {
            for (lane, value) in coordinates.iter().enumerate() {
                e.mem.set_i32(node + 0x20 + 4 * lane as u32, *value);
            }
        }
        let out = e.mem.alloc(0x20);
        e.call(0x00d7_5920, &args![hull, out, 1u32, 1.0f32, 0.0f32]);
        assert_eq!(
            read_box(&e, out),
            ([102.0, -2.0, -4.0, 0.0], [110.0, 1.0, 3.0, 0.0])
        );
        assert_eq!(read_box(&e, imp + 0xb0), read_box(&e, out));
    }

    #[test]
    fn fn_00d75b80_builds_the_plane_of_a_triangle() {
        let mut e = engine();
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.mem.set_u32(ZERO_VECTOR, 0);
        let (hull, _) = hull_with_impl(&mut e);
        let triangle = e.mem.alloc(0x20);
        let vertices: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x30)).collect();
        for (index, vertex) in vertices.iter().enumerate() {
            e.mem.set_u32(triangle + 8 + 4 * index as u32, *vertex);
        }
        let out = e.mem.alloc(0x10);
        put_vec(&mut e, vertices[0] + 0x10, [0.0, 0.0, 2.0, 0.0]);
        put_vec(&mut e, vertices[1] + 0x10, [3.0, 0.0, 2.0, 0.0]);
        put_vec(&mut e, vertices[2] + 0x10, [0.0, 4.0, 2.0, 0.0]);
        let got = e.call(0x00d7_5b80, &args![hull, out, triangle]).u32();
        assert_eq!(got, out);
        assert_vec_close(get_vec(&e, out), [0.0, 0.0, 1.0, -2.0]);
        // A degenerate triangle has no direction: the scale is zero.
        put_vec(&mut e, vertices[1] + 0x10, [0.0, 0.0, 2.0, 0.0]);
        e.call(0x00d7_5b80, &args![hull, out, triangle]);
        assert_eq!(get_vec(&e, out), [0.0; 4]);
    }

    #[test]
    fn nearest_and_farthest_vertex_searches() {
        let mut e = engine();
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_u32(SEARCH_START_NEAREST, 0x7f7f_ffff);
        let (hull, imp) = hull_with_impl(&mut e);
        vertex_list(
            &mut e,
            imp,
            &[
                [10.0, 0.0, 0.0, 1.0],
                [1.0, 1.0, 0.0, 2.0],
                [-5.0, 0.0, 0.0, 3.0],
            ],
        );
        let query = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        put_vec(&mut e, query, [0.0, 0.0, 0.0, 9.0]);
        e.call(0x00d7_5cb0, &args![hull, 0u32, query, out]);
        assert_eq!(get_vec(&e, out), [1.0, 1.0, 0.0, 2.0]);
        e.call(0x00d7_5e30, &args![hull, 0u32, query, out]);
        assert_eq!(get_vec(&e, out), [10.0, 0.0, 0.0, 1.0]);
        // An empty list leaves the query.
        e.mem.set_u32(imp + 0x24, 0);
        e.call(0x00d7_5cb0, &args![hull, 0u32, query, out]);
        assert_eq!(get_vec(&e, out), [0.0, 0.0, 0.0, 9.0]);
        // Grid positions (mode 1): the vertex integers through the grid.
        grid(&mut e, imp, [0.0; 4], [1.0, 1.0, 1.0, 1.0]);
        let nodes = vertex_list(&mut e, imp, &[[0.0; 4]; 3]);
        for (node, x) in nodes.iter().zip([7, 2, -4]) {
            e.mem.set_i32(node + 0x20, x);
        }
        put_vec(&mut e, query, [1.0, 0.0, 0.0, 9.0]);
        e.call(0x00d7_5cb0, &args![hull, 1u32, query, out]);
        assert_eq!(get_vec(&e, out), [2.0, 0.0, 0.0, 0.0]);
        e.call(0x00d7_5e30, &args![hull, 1u32, query, out]);
        assert_eq!(get_vec(&e, out), [7.0, 0.0, 0.0, 0.0]);
        // NaN distances never win.
        put_vec(&mut e, query, [f32::NAN, 0.0, 0.0, 9.0]);
        e.call(0x00d7_5cb0, &args![hull, 1u32, query, out]);
        assert!(get_vec(&e, out)[0].is_nan());
    }

    #[test]
    fn fn_00d75fc0_finds_a_vertex_in_the_grid_cell() {
        let mut e = engine();
        e.mem.set_f64(GRID_MAX_DOUBLE, 8191.0);
        e.mem.set_f32(GRID_MAX_FLOAT, 8191.0);
        e.mem.set_f64(GRID_MIN_DOUBLE, -8192.0);
        e.mem.set_f32(GRID_MIN_FLOAT, -8192.0);
        e.mem.set_f32(GRID_ROUND_DOWN, -0.5);
        e.mem.set_f32(HALF_FLOAT, 0.5);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let ftol_log = Rc::clone(&log);
        e.register_double(FTOL, move |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            ftol_log.borrow_mut().push((FTOL, a.to_vec()));
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
        let (hull, imp) = hull_with_impl(&mut e);
        put_vec(&mut e, imp + 0xf0, [0.0; 4]);
        put_vec(&mut e, imp + 0x100, [1.0; 4]);
        let nodes = vertex_list(&mut e, imp, &[[0.0; 4]; 2]);
        for (node, (x, y)) in nodes.iter().zip([(8, 8), (3, 5)]) {
            e.mem.set_i32(node + 0x20, x);
            e.mem.set_i32(node + 0x24, y);
        }
        let point = e.mem.alloc(0x10);
        put_vec(&mut e, point, [3.2, 4.7, 9.9, 0.0]);
        assert!(e.call(0x00d7_5fc0, &args![hull, point]).bool());
        // All three lanes were converted, the third is not compared.
        let converted: Vec<f64> = log
            .borrow()
            .iter()
            .map(|(_, a)| f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32))
            .collect();
        assert_eq!(converted.len(), 3);
        assert_close(converted[0] as f32, 3.7);
        assert_close(converted[1] as f32, 5.2);
        assert_close(converted[2] as f32, 10.4);
        // Rounding is away from zero, and far values are clamped.
        put_vec(&mut e, point, [3.2, 6.0, 0.0, 0.0]);
        assert!(!e.call(0x00d7_5fc0, &args![hull, point]).bool());
        e.mem.set_i32(nodes[0] + 0x20, -2);
        e.mem.set_i32(nodes[0] + 0x24, 8191);
        put_vec(&mut e, point, [-2.4, 9000.0, 0.0, 0.0]);
        assert!(e.call(0x00d7_5fc0, &args![hull, point]).bool());
        e.mem.set_i32(nodes[0] + 0x20, -8192);
        e.mem.set_i32(nodes[0] + 0x24, 4);
        put_vec(&mut e, point, [-9000.0, 3.5, 0.0, 0.0]);
        assert!(e.call(0x00d7_5fc0, &args![hull, point]).bool());
        // No vertices.
        e.mem.set_u32(imp + 0x24, 0);
        assert!(!e.call(0x00d7_5fc0, &args![hull, point]).bool());
    }

    #[test]
    fn fetch_positions_appends_to_the_array_and_grows_it() {
        let mut e = engine();
        let (reserves, _) = install_memory(&mut e);
        let (hull, imp) = hull_with_impl(&mut e);
        grid(&mut e, imp, [1.0, 2.0, 3.0, 4.0], [10.0, 10.0, 10.0, 10.0]);
        let nodes = vertex_list(&mut e, imp, &[[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]]);
        e.mem.set_i32(nodes[1] + 0x20, 1);
        e.mem.set_i32(nodes[1] + 0x24, 2);
        e.mem.set_i32(nodes[1] + 0x28, 3);
        // An array holding one vector with room for it only.
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x10);
        put_vec(&mut e, data, [9.0; 4]);
        e.mem.set_u32(array, data);
        e.mem.set_i32(array + 4, 1);
        e.mem.set_u32(array + 8, 0x8000_0001);
        e.call(0x00d7_6120, &args![hull, 0u32, array]);
        assert_eq!(e.mem.i32(array + 4), 3);
        let words = reserves.borrow()[0].1.clone();
        // Capacity 1 is below 3: twice the capacity (2) is less than needed (3).
        assert_eq!(words[1..], [array, 3, 0x10]);
        let data = e.mem.u32(array);
        assert_eq!(get_vec(&e, data), [9.0; 4]);
        assert_eq!(get_vec(&e, data + 0x10), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(get_vec(&e, data + 0x20), [5.0, 6.0, 7.0, 8.0]);
        // Grid positions go through the offset and scale; no growth when it fits.
        let roomy = e.mem.alloc(0x10);
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(roomy, block);
        e.mem.set_u32(roomy + 8, 4);
        reserves.borrow_mut().clear();
        e.call(0x00d7_6120, &args![hull, 1u32, roomy]);
        assert!(reserves.borrow().is_empty());
        assert_eq!(e.mem.i32(roomy + 4), 2);
        assert_eq!(get_vec(&e, block), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(get_vec(&e, block + 0x10), [11.0, 22.0, 33.0, 4.0]);
    }

    #[test]
    fn fetch_planes_appends_every_plane() {
        let mut e = engine();
        let (reserves, _) = install_memory(&mut e);
        install_report(&mut e, 0);
        let (hull, imp) = hull_with_impl(&mut e);
        let planes = e.mem.alloc(0x30);
        for plane in 0..3u32 {
            put_vec(&mut e, planes + 0x10 * plane, [plane as f32, 1.0, 2.0, 3.0]);
        }
        e.mem.set_u32(imp + 0x54, planes);
        e.mem.set_i32(imp + 0x58, 3);
        e.mem.set_u8(imp + 0x1b5, 1);
        let array = e.mem.alloc(0x10);
        e.call(0x00d7_6250, &args![hull, array]);
        assert_eq!(e.mem.i32(array + 4), 3);
        assert_eq!(reserves.borrow()[0].1[1..], [array, 3, 0x10]);
        let data = e.mem.u32(array);
        for plane in 0..3u32 {
            assert_eq!(
                get_vec(&e, data + 0x10 * plane),
                [plane as f32, 1.0, 2.0, 3.0]
            );
        }
    }

    /// Doubles for the support structure of `00d76320`: the build (`00dedcf0`) logs the array
    /// it is given (size and the vectors) and the query (`00dea810`) answers the input vector
    /// doubled; both log their words.
    fn install_support(e: &mut Engine) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let build_log = Rc::clone(&log);
        e.register_double(SUPPORT_BUILD, move |e, a| {
            let (array, _) = (a[0], a[1]);
            let size = e.mem.u32(array + 4);
            let data = e.mem.u32(array);
            let mut words = vec![size];
            for word in 0..size * 4 {
                words.push(e.mem.u32(data + 4 * word));
            }
            build_log.borrow_mut().push((SUPPORT_BUILD, words));
            Ret::default()
        });
        let query_log = Rc::clone(&log);
        e.register_double(SUPPORT_QUERY, move |e, a| {
            let input = load_vec(e, a[1]);
            query_log.borrow_mut().push((
                SUPPORT_QUERY,
                [a.to_vec(), input.map(f32::to_bits).to_vec()].concat(),
            ));
            store_vec(e, a[2], input.map(|lane| lane * 2.0));
            Ret::default()
        });
        log
    }

    #[test]
    fn fn_00d76320_builds_the_support_structure_once() {
        let mut e = engine();
        let (reserves, frees) = install_memory(&mut e);
        let log = install_support(&mut e);
        let (hull, imp) = hull_with_impl(&mut e);
        vertex_list(&mut e, imp, &[[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]]);
        let input = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        put_vec(&mut e, input, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x00d7_6320, &args![hull, input, out]);
        assert_eq!(get_vec(&e, out), [2.0, 4.0, 6.0, 8.0]);
        {
            let calls = log.borrow();
            assert_eq!(calls[0].0, SUPPORT_BUILD);
            assert_eq!(calls[0].1[0], 2);
            let positions: Vec<f32> = calls[0].1[1..]
                .iter()
                .map(|bits| f32::from_bits(*bits))
                .collect();
            assert_eq!(positions, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
            assert_eq!(calls[1].1[..3], [imp + 0x60, input, out]);
        }
        // The array started non-owning (capacity word 0x80000000): reserve with the count,
        // and release its block (capacity 2, 0x20 bytes) through the allocator.
        assert_eq!(reserves.borrow().len(), 1);
        assert_eq!(reserves.borrow()[0].1[2..], [2, 0x10]);
        assert_eq!(frees.borrow().len(), 1);
        assert_eq!(frees.borrow()[0].1[2], 0x20);
        // With the structure's second word set it is not built again.
        e.mem.set_u32(imp + 0x64, 1);
        e.call(0x00d7_6320, &args![hull, input, out]);
        assert_eq!(log.borrow().len(), 3);
        assert_eq!(log.borrow()[2].0, SUPPORT_QUERY);
        assert_eq!(reserves.borrow().len(), 1);
    }

    #[test]
    fn fn_00d76320_keeps_a_non_owning_empty_array() {
        let mut e = engine();
        let (reserves, frees) = install_memory(&mut e);
        let log = install_support(&mut e);
        let (hull, _) = hull_with_impl(&mut e);
        let input = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        e.call(0x00d7_6320, &args![hull, input, out]);
        // No vertices: nothing reserved, so the array is still non-owning and nothing is freed.
        assert!(reserves.borrow().is_empty());
        assert!(frees.borrow().is_empty());
        assert_eq!(log.borrow()[0].1, vec![0]);
    }

    #[test]
    fn support_functions_evaluate_the_plane_at_the_support_point() {
        let mut e = engine();
        install_memory(&mut e);
        let log = install_support(&mut e);
        e.mem.set_f32(ZERO_F32, 0.0);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_u32(imp + 0x64, 1);
        let plane = e.mem.alloc(0x10);
        put_vec(&mut e, plane, [1.0, 2.0, 3.0, 5.0]);
        // The support point is twice the query: dot(plane, 2 * query) + w.
        assert_eq!(e.call(0x00d7_68b0, &args![hull, plane]).f32(), 28.0 + 5.0);
        assert_eq!(log.borrow()[0].1[1], plane);
        assert_eq!(e.call(0x00d7_6840, &args![hull, plane]).f32(), -28.0 + 5.0);
        let negated: Vec<f32> = log.borrow()[1].1[3..]
            .iter()
            .map(|b| f32::from_bits(*b))
            .collect();
        assert_eq!(negated, [-1.0, -2.0, -3.0, -5.0]);
        // The extent along a direction: the difference of the two, w taken as zero.
        let direction = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [1.0, 2.0, 3.0, 99.0]);
        assert_eq!(e.call(0x00d7_69c0, &args![hull, direction]).f32(), 56.0);
        let queried: Vec<f32> = log.borrow()[2].1[3..]
            .iter()
            .map(|b| f32::from_bits(*b))
            .collect();
        assert_eq!(queried, [1.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn fn_00d7a180_averages_the_vertex_positions_once() {
        let mut e = engine();
        e.mem.set_f64(FLOAT_MAX_DOUBLE, f64::from(f32::MAX));
        let (_, imp) = hull_with_impl(&mut e);
        vertex_list(
            &mut e,
            imp,
            &[
                [1.0, 2.0, 3.0, 9.0],
                [3.0, 4.0, 5.0, 9.0],
                [5.0, 6.0, 7.0, 9.0],
                [7.0, 8.0, 9.0, 9.0],
            ],
        );
        e.mem.set_f32(imp + 0x7c, f32::MAX);
        assert_eq!(e.call(0x00d7_a180, &args![imp]).u32(), imp + 0x70);
        assert_eq!(get_vec(&e, imp + 0x70), [4.0, 5.0, 6.0, 0.0]);
        // The cached centre (fourth lane no longer the marker) is kept.
        put_vec(&mut e, imp + 0x70, [1.0, 1.0, 1.0, 0.0]);
        e.call(0x00d7_a180, &args![imp]);
        assert_eq!(get_vec(&e, imp + 0x70), [1.0, 1.0, 1.0, 0.0]);
        // No vertices: the sum is the zero vector and is not divided.
        e.mem.set_u32(imp + 0x24, 0);
        e.mem.set_i32(imp + 0x28, 0);
        put_vec(&mut e, imp + 0x70, [7.0, 7.0, 7.0, f32::MAX]);
        e.call(0x00d7_a180, &args![imp]);
        assert_eq!(get_vec(&e, imp + 0x70), [0.0; 4]);
    }

    #[test]
    fn fn_00d76a20_orders_the_rows_by_extent_and_normalises_them() {
        let mut e = engine();
        install_memory(&mut e);
        install_support(&mut e);
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_u32(ZERO_VECTOR, 0);
        e.register(X87_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_u8(imp + 0x1b4, 1);
        e.mem.set_u32(imp + 0x64, 1);
        put_vec(&mut e, imp + 0x120, [1.0, 0.0, 0.0, 7.0]);
        put_vec(&mut e, imp + 0x130, [0.0, 3.0, 0.0, 7.0]);
        put_vec(&mut e, imp + 0x140, [0.0, 0.0, 2.0, 7.0]);
        put_vec(&mut e, imp + 0x150, [1.0, 1.0, 1.0, 5.0]);
        let out = e.mem.alloc(0x30);
        e.call(0x00d7_6a20, &args![hull, out]);
        // The support point is twice the query, so the extent is 4 * |row|^2: 36, 16, 4.
        let first = get_vec(&e, out);
        assert_vec_close(first, [0.0, 1.0, 0.0, -1.0]);
        assert_vec_close(get_vec(&e, out + 0x10), [0.0, 0.0, 1.0, -1.0]);
        assert_vec_close(get_vec(&e, out + 0x20), [1.0, 0.0, 0.0, -1.0]);
    }

    #[test]
    fn best_projection_plane_by_dimension() {
        let mut e = engine();
        e.register(X87_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        let (hull, imp) = hull_with_impl(&mut e);
        let out = e.mem.alloc(0x10);
        put_vec(&mut e, out, [9.0; 4]);
        // Unknown dimension count: zero vector (the fourth lane cleared).
        e.mem.set_i32(imp + 0x1a8, 0);
        assert_eq!(e.call(0x00d7_6d90, &args![hull, out]).u32(), out);
        assert_eq!(get_vec(&e, out), [0.0; 4]);
        // Two dimensions: the vector at +0xe0 with the fourth lane cleared.
        e.mem.set_i32(imp + 0x1a8, 2);
        put_vec(&mut e, imp + 0xe0, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x00d7_6d90, &args![hull, out]);
        assert_eq!(get_vec(&e, out), [1.0, 2.0, 3.0, 0.0]);
        // One dimension. (0, 3, 4): the first is not larger than the second, so the pair is
        // (1, 2) as the third is not smaller than the first: out[1] = v[2], out[2] = -v[1].
        e.mem.set_i32(imp + 0x1a8, 1);
        put_vec(&mut e, imp + 0xd0, [0.0, 3.0, 4.0, 0.0]);
        e.call(0x00d7_6d90, &args![hull, out]);
        assert_vec_close(get_vec(&e, out), [0.0, 0.8, -0.6, 0.0]);
        // (5, 3, 1): the first is larger and the third is smaller than both: pair (0, 1).
        put_vec(&mut e, imp + 0xd0, [5.0, 3.0, 1.0, 0.0]);
        e.call(0x00d7_6d90, &args![hull, out]);
        let norm = 34.0f32.sqrt();
        assert_vec_close(get_vec(&e, out), [3.0 / norm, -5.0 / norm, 0.0, 0.0]);
        // (5, 3, 9): the first is larger, the third is not smaller: pair (0, 2).
        put_vec(&mut e, imp + 0xd0, [5.0, 3.0, 9.0, 0.0]);
        e.call(0x00d7_6d90, &args![hull, out]);
        let norm = 106.0f32.sqrt();
        assert_vec_close(get_vec(&e, out), [9.0 / norm, 0.0, -5.0 / norm, 0.0]);
        // (2, 4, 6): pair (1, 2).
        put_vec(&mut e, imp + 0xd0, [2.0, 4.0, 6.0, 0.0]);
        e.call(0x00d7_6d90, &args![hull, out]);
        let norm = 52.0f32.sqrt();
        assert_vec_close(get_vec(&e, out), [0.0, 6.0 / norm, -4.0 / norm, 0.0]);
    }

    /// Doubles for the `hkMapBase` a geometry build fills: reserve and the end-of-use calls log
    /// their words; insert stores the pair and counts the entry in the map's second word; the
    /// lookup answers the pair or its default.
    fn install_map(e: &mut Engine) -> Rc<RefCell<std::collections::HashMap<u32, u32>>> {
        let pairs = Rc::new(RefCell::new(std::collections::HashMap::new()));
        e.register_double(MAP_RESERVE, |_, _| Ret::default());
        e.register_double(MAP_CLEAR_AND_DEALLOCATE, |_, _| Ret::default());
        e.register_double(LOCAL_NO_EFFECT, |_, _| Ret::default());
        let insert_pairs = Rc::clone(&pairs);
        e.register_double(MAP_INSERT, move |e, a| {
            insert_pairs.borrow_mut().insert(a[2], a[3]);
            let count = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, count + 1);
            Ret::default()
        });
        let get_pairs = Rc::clone(&pairs);
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| Ret {
            eax: get_pairs.borrow().get(&a[1]).copied().unwrap_or(a[2]),
            ..Ret::default()
        });
        pairs
    }

    #[test]
    fn generate_geometry_of_a_polygon_is_a_fan() {
        let mut e = engine();
        let (reserves, frees) = install_memory(&mut e);
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_i32(imp + 0x1a8, 2);
        grid(&mut e, imp, [1.0, 0.0, 0.0, 0.0], [2.0, 1.0, 1.0, 1.0]);
        let nodes = vertex_list(
            &mut e,
            imp,
            &[
                [0.0, 0.0, 0.0, 1.0],
                [4.0, 0.0, 0.0, 1.0],
                [4.0, 4.0, 0.0, 1.0],
            ],
        );
        e.mem.set_i32(nodes[1] + 0x20, 3);
        let geometry = e.mem.alloc(0x18);
        // Two vertices already in the geometry (capacity exactly 2), no triangles.
        let old = e.mem.alloc(0x20);
        e.mem.set_u32(geometry, old);
        e.mem.set_i32(geometry + 4, 2);
        e.mem.set_u32(geometry + 8, 2);
        e.mem.set_u32(geometry + 0x14, 0x8000_0000);
        e.call(0x00d7_6410, &args![hull, 0u32, geometry]);
        assert_eq!(e.mem.i32(geometry + 4), 5);
        let vertices = e.mem.u32(geometry);
        assert_eq!(get_vec(&e, vertices + 0x20), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(get_vec(&e, vertices + 0x40), [4.0, 4.0, 0.0, 1.0]);
        // 2 * (3 - 2) = 2 triangles over the indices 2, 3, 4, both windings.
        assert_eq!(e.mem.i32(geometry + 0x10), 2);
        let triangles = e.mem.u32(geometry + 0xc);
        let words: Vec<u32> = [0, 1, 2, 4, 5, 6]
            .iter()
            .map(|w| e.mem.u32(triangles + 4 * w))
            .collect();
        assert_eq!(words, [2, 3, 4, 2, 4, 3]);
        // The temporary index array (element size 4) was released with its capacity in bytes.
        assert!(reserves
            .borrow()
            .iter()
            .any(|(_, a)| a[3] == 4 && a[2] == 3));
        assert_eq!(frees.borrow().len(), 1);
        assert_eq!(frees.borrow()[0].1[2], 12);
        // Grid positions: the second vertex has x = 3 -> 1 + 2 * 3 = 7.
        let again = e.mem.alloc(0x18);
        e.mem.set_u32(again + 0x14, 0x8000_0000);
        e.mem.set_u32(again + 8, 0x8000_0000);
        e.call(0x00d7_6410, &args![hull, 1u32, again]);
        let vertices = e.mem.u32(again);
        assert_eq!(get_vec(&e, vertices), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(get_vec(&e, vertices + 0x10), [7.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn generate_geometry_of_a_solid_maps_vertices_to_indices() {
        let mut e = engine();
        install_memory(&mut e);
        let pairs = install_map(&mut e);
        let (hull, imp) = hull_with_impl(&mut e);
        e.mem.set_i32(imp + 0x1a8, 3);
        let nodes = vertex_list(
            &mut e,
            imp,
            &[
                [0.0, 0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
            ],
        );
        // One triangle node (+8, +0xc, +0x10 are the vertex nodes) in the list at +0x3c.
        let triangle = e.mem.alloc(0x20);
        e.mem.set_u32(triangle + 8, nodes[2]);
        e.mem.set_u32(triangle + 0xc, nodes[0]);
        e.mem.set_u32(triangle + 0x10, nodes[1]);
        e.mem.set_u32(imp + 0x3c, triangle);
        e.mem.set_i32(imp + 0x40, 1);
        let geometry = e.mem.alloc(0x18);
        let old = e.mem.alloc(0x10);
        e.mem.set_u32(geometry, old);
        e.mem.set_i32(geometry + 4, 1);
        e.mem.set_u32(geometry + 8, 1);
        e.mem.set_u32(geometry + 0x14, 0x8000_0000);
        e.call(0x00d7_6410, &args![hull, 0u32, geometry]);
        // The vertices were appended after the old one and mapped to base + number so far.
        assert_eq!(pairs.borrow().get(&nodes[0]), Some(&1));
        assert_eq!(pairs.borrow().get(&nodes[1]), Some(&2));
        assert_eq!(pairs.borrow().get(&nodes[2]), Some(&3));
        let vertices = e.mem.u32(geometry);
        assert_eq!(get_vec(&e, vertices + 0x20), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(e.mem.i32(geometry + 4), 4);
        let triangles = e.mem.u32(geometry + 0xc);
        let words: Vec<u32> = (0..4).map(|w| e.mem.u32(triangles + 4 * w)).collect();
        assert_eq!(words, [3, 1, 2, 0xffff_ffff]);
        // An unmapped vertex gets the default -1.
        pairs.borrow_mut().remove(&nodes[0]);
        let second = e.mem.alloc(0x18);
        e.mem.set_u32(second + 0x14, 0x8000_0000);
        e.mem.set_u32(second + 8, 0x8000_0000);
        // The doubles keep the pairs, so run the lookup against a fresh map only.
        e.mem.set_u32(imp + 0x24, 0);
        e.mem.set_i32(imp + 0x28, 0);
        pairs.borrow_mut().clear();
        e.call(0x00d7_6410, &args![hull, 0u32, second]);
        let triangles = e.mem.u32(second + 0xc);
        let words: Vec<u32> = (0..4).map(|w| e.mem.u32(triangles + 4 * w)).collect();
        assert_eq!(words, [0xffff_ffff; 4]);
        // Other dimension counts do nothing.
        e.mem.set_i32(imp + 0x1a8, 1);
        let untouched = e.mem.alloc(0x18);
        e.call(0x00d7_6410, &args![hull, 0u32, untouched]);
        assert_eq!(e.mem.i32(untouched + 4), 0);
    }

    // ---- the workbench for the larger translations ----

    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0012;
    const FREE_ARRAY_SLOT_DOUBLE: u32 = 0x0900_0013;

    /// An engine with the bench installed; the log takes the frees as `(double, words)`.
    fn bench() -> (Engine, Log) {
        let mut e = engine();
        let log = install_bench(&mut e);
        e.mem.set_f32(ZERO_F32, 0.0);
        (e, log)
    }

    /// What the exe holds at the constants the hull code reads (words copied from the exe's
    /// data; a vector is four words, a `double` two, a `float` one) and the two vtables the
    /// hull and its mesh point at.
    fn install_constants(e: &mut Engine) {
        const WORDS: &[(u32, &[u32])] = &[
            (0x0101_1588, &[0x0000_0000, 0x3fe0_0000]),
            (0x0101_1590, &[0x0000_0000, 0x4000_0000]),
            (0x0101_1d78, &[0]),
            (0x0101_2054, &[0xbf80_0000]),
            (0x0101_2068, &[0x3f80_0000]),
            (0x0101_41cc, &[0x38d1_b717]),
            (0x0101_6248, &[0x3f00_0000]),
            (0x0101_7718, &[0x4040_0000]),
            (0x0101_7d10, &[0x7fff_ffff; 4]),
            (0x0101_a6b0, &[0x0000_0000, 0xbff0_0000]),
            (0x0101_dae8, &[0x8000_0000, 0x3ee4_f8b5]),
            (0x0101_db80, &[0x0000_0000, 0x4010_0000]),
            (0x0102_0758, &[0x0000_0000, 0x4024_0000]),
            (0x0102_295c, &[0xbf00_0000]),
            (0x0102_29c0, &[0x0000_0000, 0x3df0_0000]),
            (0x0102_fc70, &[0x0000_0000, 0x4034_0000]),
            (0x0103_2980, &[0xe000_0000, 0x3f1a_36e2]),
            (0x0103_4208, &[0x0000_0000, 0x4018_0000]),
            (0x0104_da48, &[0x7f7f_ffee, 0x3400_0000]),
            (
                0x0104_e138,
                &[0xc000_0000, 0x47ef_fffd, 0xc000_0000, 0xc7ef_fffd],
            ),
            (0x0104_e148, &[0xff7f_ffee]),
            (0x0106_c288, &[0x33d6_bf95]),
            (0x0106_ea1c, &[0x3f7f_f972]),
            (0x0109_9378, &[0x4f80_0000]),
            (0x0109_9ee0, &[0x6000_0000, 0x3fd5_5555]),
            (0x010c_44d0, &[0x8000_0000; 4]),
            (0x010c_4990, &[0x8000_0000]),
            (0x010c_7160, &[0; 4]),
            (0x010c_7170, &[0x3f80_0000; 4]),
            (0x010c_92e0, &[1, 2, 4, 8]),
            (0x010d_1018, &[0x35a0_0000]),
            (
                0x010d_35e0,
                &[0x0000_0000, 0x4013_580a, 0x6000_0000, 0x3fe5_5555],
            ),
            (
                0x010d_3688,
                &[0x0000_0000, 0xc0c0_0000, 0x45ff_f800, 0xc600_0000],
            ),
            (0x010d_3698, &[0x0000_0000, 0x40bf_ff00]),
            (
                0x010d_36cc,
                &[0x00d7_d1c0, 0x00c9_0350, 0x00c9_0360, 0x00d7_c950],
            ),
            (0x010d_36e0, &[0x00d7_f060, 0x00c9_0350, 0x00c9_0360]),
            (0x010d_37d8, &[0x0000_0000, 0x3eb4_0000]),
            (0x010e_c7f0, &[0x3f80_0000; 4]),
        ];
        for (address, words) in WORDS {
            for (index, word) in words.iter().enumerate() {
                e.mem.set_u32(address + 4 * index as u32, *word);
            }
        }
        // The identity the start-up code leaves for the containment transform.
        for diagonal in 0..4u32 {
            e.mem
                .set_u32(CONTAINMENT_MATRIX + 0x14 * diagonal, 0x3f80_0000);
        }
    }

    /// The identity `hkMatrix3` (`00cd4b50` answers its address, three columns of a vector).
    fn install_identity(e: &mut Engine) {
        let identity = e.mem.alloc(0x30);
        for column in 0..3u32 {
            put_vec(e, identity + 0x10 * column, {
                let mut v = [0.0; 4];
                v[column as usize] = 1.0;
                v
            });
        }
        e.register_double(MATRIX3_GET_IDENTITY, move |_, _| Ret {
            eax: identity,
            ..Ret::default()
        });
    }

    /// Everything a build runs through: the constants, a memory router whose allocator hands
    /// out fresh blocks (slot 4) and takes them back without effect (slots 8 and `0x10`, the
    /// latter logged as `(data, bytes)` in the answer), the `hkArray` reserve doubles that move
    /// the data to a new 16-byte aligned block (reserve, reserve more: twice the size, at
    /// least 4; reduce; header swap), the CRT conversions and root, the 64-bit product, the
    /// report builder and the identity matrix.
    fn install_bench(e: &mut Engine) -> Log {
        install_constants(e);
        install_identity(e);
        install_maps(e);
        install_math(e);
        install_counter(e);
        install_support_points(e);
        let frees: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, allocator);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATE_SLOT_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let block_log = Rc::clone(&frees);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            block_log.borrow_mut().push((FREE_SLOT_DOUBLE, a.to_vec()));
            Ret::default()
        });
        let free_log = Rc::clone(&frees);
        e.register_double(FREE_ARRAY_SLOT_DOUBLE, move |_, a| {
            free_log
                .borrow_mut()
                .push((FREE_ARRAY_SLOT_DOUBLE, a.to_vec()));
            Ret::default()
        });
        fn move_array(e: &mut Engine, array: u32, capacity: u32, element: u32) {
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4).min(capacity);
            let raw = e.mem.alloc(capacity * element + 16);
            let block = (raw + 15) & !15;
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(block + byte, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
        }
        e.register_double(ARRAY_RESERVE, |e, a| {
            move_array(e, a[1], a[2], a[3]);
            Ret::default()
        });
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let size = e.mem.u32(a[1] + 4);
            move_array(e, a[1], (size * 2).max(4), a[2]);
            Ret::default()
        });
        e.register_double(ARRAY_REDUCE, |e, a| {
            move_array(e, a[1], a[4], a[2]);
            Ret::default()
        });
        e.register_double(ARRAY_SWAP, |e, a| {
            for word in 0..3 {
                let mine = e.mem.u32(a[0] + 4 * word);
                let theirs = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, theirs);
                e.mem.set_u32(a[1] + 4 * word, mine);
            }
            Ret::default()
        });
        e.register(VECTOR4_IS_NORMALIZED3, |e, a| {
            let v = load_vec(e, a[0]);
            let squared = f64::from(v[0]) * f64::from(v[0])
                + f64::from(v[1]) * f64::from(v[1])
                + f64::from(v[2]) * f64::from(v[2]);
            let normalised = v.iter().all(|lane| lane.is_finite())
                && (squared - 1.0).abs() < f64::from(f32::from_bits(a[2]));
            e.mem.set_u8(a[1], u8::from(normalised));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32,
            ..Ret::default()
        });
        e.register(X87_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e.register(ALLMUL, |_, a| {
            let product = (u64::from(a[0]) | u64::from(a[1]) << 32)
                .wrapping_mul(u64::from(a[2]) | u64::from(a[3]) << 32);
            Ret {
                eax: product as u32,
                edx: (product >> 32) as u32,
                ..Ret::default()
            }
        });
        frees
    }

    /// Runs the call, which must end in an `INT3` (the report sink asked for a break), and
    /// answers the panic message.
    fn int3_message(e: &mut Engine, function: u32, words: &[u32]) -> String {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(function, words);
        }));
        let payload = result.unwrap_err();
        match payload.downcast::<String>() {
            Ok(text) => *text,
            Err(payload) => (*payload.downcast::<&str>().unwrap()).to_string(),
        }
    }

    /// The entry `index` of the block list of the pool (header `{blocks, count}`).
    fn pool_block(e: &Engine, pool: u32, index: u32) -> u32 {
        e.mem.u32(e.mem.u32(pool) + 4 * index)
    }

    /// A pool header (`0x14` bytes, non-owning empty block list).
    fn new_pool(e: &mut Engine) -> u32 {
        let pool = e.mem.alloc(0x14);
        e.mem.set_u32(pool + 8, 0x8000_0000);
        pool
    }

    /// Doubles for the `hkMap` the clone and the geometry build fill, keyed by (map, key):
    /// reserve and the call after the end of use do nothing, insert stores the pair, the lookup
    /// answers the value or its default, remove answers 0 when it removed the key and 1
    /// otherwise, and clear drops the entries of the map.
    fn install_maps(e: &mut Engine) {
        type Pairs = Rc<RefCell<std::collections::HashMap<(u32, u32), u32>>>;
        let pairs: Pairs = Rc::new(RefCell::new(std::collections::HashMap::new()));
        e.register_double(MAP_RESERVE, |_, _| Ret::default());
        e.register_double(LOCAL_NO_EFFECT, |_, _| Ret::default());
        let insert = Rc::clone(&pairs);
        e.register_double(MAP_INSERT, move |_, a| {
            insert.borrow_mut().insert((a[0], a[2]), a[3]);
            Ret::default()
        });
        let get = Rc::clone(&pairs);
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| Ret {
            eax: get.borrow().get(&(a[0], a[1])).copied().unwrap_or(a[2]),
            ..Ret::default()
        });
        let remove = Rc::clone(&pairs);
        e.register_double(MAP_REMOVE_KEY, move |_, a| Ret {
            eax: u32::from(remove.borrow_mut().remove(&(a[0], a[1])).is_none()),
            ..Ret::default()
        });
        e.register_double(MAP_CLEAR_AND_DEALLOCATE, move |_, a| {
            pairs.borrow_mut().retain(|(map, _), _| *map != a[0]);
            Ret::default()
        });
    }

    /// Checks a new pool block: the items chained from the first to the last (`stride` bytes
    /// apart, ending in zero), its head and use count.
    fn check_block(e: &Engine, block: u32, head_offset: u32, count_offset: u32, stride: u32) {
        assert_eq!(e.mem.u32(block + head_offset), block);
        assert_eq!(e.mem.u32(block + count_offset), 0);
        for item in 0..31 {
            assert_eq!(
                e.mem.u32(block + stride * item),
                block + stride * (item + 1)
            );
        }
        assert_eq!(e.mem.u32(block + stride * 31), 0);
    }

    #[test]
    fn vertex_pool_allocates_blocks_of_chained_items() {
        let (mut e, _) = bench();
        let pool = new_pool(&mut e);
        let first = e.call(0x00d7_c2a0, &args![pool]).u32();
        check_block(&e, first, 0x800, 0x804, 0x40);
        assert_eq!(e.mem.i32(pool + 4), 1);
        assert_eq!(pool_block(&e, pool, 0), first);
        // A second block becomes the first of the list.
        let second = e.call(0x00d7_c2a0, &args![pool]).u32();
        assert_eq!(e.mem.i32(pool + 4), 2);
        assert_eq!(pool_block(&e, pool, 0), second);
        assert_eq!(pool_block(&e, pool, 1), first);
        assert_ne!(first, second);
    }

    #[test]
    fn triangle_pool_allocates_blocks_of_chained_items() {
        let (mut e, _) = bench();
        let pool = new_pool(&mut e);
        let first = e.call(0x00d7_c340, &args![pool]).u32();
        check_block(&e, first, 0x900, 0x904, 0x48);
        assert_eq!(pool_block(&e, pool, 0), first);
        let second = e.call(0x00d7_c340, &args![pool]).u32();
        assert_eq!(pool_block(&e, pool, 0), second);
        assert_eq!(pool_block(&e, pool, 1), first);
    }

    /// A pool with two blocks and some items in use: answers the pool and the blocks.
    fn pool_with_two_blocks(e: &mut Engine, vertices: bool) -> (u32, u32, u32) {
        let pool = new_pool(e);
        let address = if vertices { 0x00d7_c2a0 } else { 0x00d7_c340 };
        let first = e.call(address, &args![pool]).u32();
        let second = e.call(address, &args![pool]).u32();
        e.mem.set_u32(pool + 0xc, 0x1234);
        e.mem.set_u32(pool + 0x10, 5);
        (pool, first, second)
    }

    #[test]
    fn pool_clearing_frees_the_blocks_and_forgets_the_items() {
        for (function, vertices, size) in [
            (0x00d7_c200u32, true, 0x808u32),
            (0x00d7_c250, false, 0x908),
        ] {
            let (mut e, frees) = bench();
            let (pool, first, second) = pool_with_two_blocks(&mut e, vertices);
            e.call(function, &args![pool]);
            let frees = frees.borrow();
            let freed: Vec<(u32, u32)> = frees.iter().map(|(_, a)| (a[1], a[2])).collect();
            assert_eq!(freed, vec![(second, size), (first, size)]);
            assert_eq!(frees[0].0, FREE_SLOT_DOUBLE);
            assert_eq!(e.mem.u32(pool + 4), 0);
            assert_eq!(e.mem.u32(pool + 0xc), 0);
            assert_eq!(e.mem.u32(pool + 0x10), 0);
        }
    }

    #[test]
    fn pool_destruction_also_frees_the_block_list() {
        for (function, vertices, size) in [
            (0x00d7_c850u32, true, 0x808u32),
            (0x00d7_c8d0, false, 0x908),
        ] {
            let (mut e, frees) = bench();
            let (pool, first, second) = pool_with_two_blocks(&mut e, vertices);
            let blocks = e.mem.u32(pool);
            e.call(function, &args![pool]);
            let frees = frees.borrow();
            let freed: Vec<(u32, u32, u32)> = frees.iter().map(|(d, a)| (*d, a[1], a[2])).collect();
            // The capacity after two blocks is the 4 the reserve double settles on.
            assert_eq!(
                freed,
                vec![
                    (FREE_SLOT_DOUBLE, second, size),
                    (FREE_SLOT_DOUBLE, first, size),
                    (FREE_ARRAY_SLOT_DOUBLE, blocks, 16)
                ]
            );
            assert_eq!(e.mem.u32(pool), 0);
            assert_eq!(e.mem.u32(pool + 4), 0);
            assert_eq!(e.mem.u32(pool + 8), 0x8000_0000);
        }
    }

    #[test]
    fn pool_release_counts_down_and_frees_an_empty_block() {
        for (function, vertices, span, count_offset, size, address) in [
            (
                0x00d7_adb0u32,
                true,
                0x7c0u32,
                0x804u32,
                0x808u32,
                0x00d7_ae32u32,
            ),
            (0x00d7_aea0, false, 0x8b8, 0x904, 0x908, 0x00d7_af22),
        ] {
            let (mut e, frees) = bench();
            let (pool, first, second) = pool_with_two_blocks(&mut e, vertices);
            // The list holds [second, first] (the newest block leads).
            e.mem.set_u32(first + count_offset, 2);
            e.mem.set_u32(second + count_offset, 1);
            // A null item does nothing; one of two uses leaves the block in place.
            e.call(function, &args![pool, 0u32]);
            e.call(function, &args![pool, first + 0x10]);
            assert_eq!(e.mem.u32(first + count_offset), 1);
            assert!(frees.borrow().is_empty());
            // The last use of the second block frees it; the last block takes its place.
            e.call(function, &args![pool, second + span]);
            assert_eq!(e.mem.u32(second + count_offset), 0);
            assert_eq!(frees.borrow().len(), 1);
            assert_eq!(frees.borrow()[0].1[1..], [second, size]);
            assert_eq!(e.mem.i32(pool + 4), 1);
            assert_eq!(pool_block(&e, pool, 0), first);
            // An item in no block is the report; a non-zero answer breaks.
            let log = install_report(&mut e, 0);
            e.call(function, &args![pool, 0x1000u32]);
            assert_eq!(log.borrow()[1].1[1], MESSAGE_POOL_ITEM);
            assert_eq!(log.borrow()[2].1[1], 3);
            assert_eq!(log.borrow()[2].1[2], 0x9035_5365);
            assert_eq!(log.borrow()[2].1[4..], [POOL_REPORT_FILE, 0xaf]);
            install_report(&mut e, 1);
            let message = int3_message(&mut e, function, &args![pool, 0x1000u32]);
            assert!(
                message.contains(&format!("INT3 at {address:08x}")),
                "{message}"
            );
        }
    }

    /// A mesh (vtable word, pools at `+8` and `+0x20`) holding copies of the two pools.
    fn mesh_with_pools(e: &mut Engine) -> u32 {
        let mesh = e.mem.alloc(0x40);
        e.mem.set_u16(mesh + 4, 0x40);
        let (vertex_pool, _, _) = pool_with_two_blocks(e, true);
        let (triangle_pool, _, _) = pool_with_two_blocks(e, false);
        for word in 0..5 {
            let a = e.mem.u32(vertex_pool + 4 * word);
            e.mem.set_u32(mesh + 8 + 4 * word, a);
            let b = e.mem.u32(triangle_pool + 4 * word);
            e.mem.set_u32(mesh + 0x20 + 4 * word, b);
        }
        mesh
    }

    #[test]
    fn mesh_reset_clears_both_pools() {
        let (mut e, frees) = bench();
        let mesh = mesh_with_pools(&mut e);
        e.call(0x00d7_c950, &args![mesh]);
        assert_eq!(e.mem.u32(mesh + 0xc), 0);
        assert_eq!(e.mem.u32(mesh + 0x14), 0);
        assert_eq!(e.mem.u32(mesh + 0x24), 0);
        assert_eq!(e.mem.u32(mesh + 0x2c), 0);
        // The blocks of the vertex pool go back first, then those of the triangle pool.
        let sizes: Vec<u32> = frees.borrow().iter().map(|(_, a)| a[2]).collect();
        assert_eq!(sizes, vec![0x808, 0x808, 0x908, 0x908]);
    }

    #[test]
    fn mesh_destructor_frees_the_pools_and_optionally_itself() {
        let (mut e, frees) = bench();
        let mesh = mesh_with_pools(&mut e);
        let got = e.call(0x00d7_d1c0, &args![mesh, 0u32]).u32();
        assert_eq!(got, mesh);
        assert_eq!(e.mem.u32(mesh), HULL_BASE_VTABLE);
        assert_eq!(e.mem.u32(mesh + 0x8), 0);
        assert_eq!(e.mem.u32(mesh + 0x20), 0);
        // The triangle pool goes first (blocks, then its block list), then the vertex pool.
        let log: Vec<(u32, u32)> = frees.borrow().iter().map(|(d, a)| (*d, a[2])).collect();
        assert_eq!(
            log,
            vec![
                (FREE_SLOT_DOUBLE, 0x908),
                (FREE_SLOT_DOUBLE, 0x908),
                (FREE_ARRAY_SLOT_DOUBLE, 16),
                (FREE_SLOT_DOUBLE, 0x808),
                (FREE_SLOT_DOUBLE, 0x808),
                (FREE_ARRAY_SLOT_DOUBLE, 16)
            ]
        );
        frees.borrow_mut().clear();
        e.call(0x00d7_d1c0, &args![mesh, 1u32]);
        let last = frees.borrow().last().cloned().unwrap();
        assert_eq!(
            (last.0, last.1[1], last.1[2]),
            (FREE_SLOT_DOUBLE, mesh, 0x40)
        );
    }

    /// A triangle node with the corner vertices' grid coordinates `keys`, the next triangle
    /// and the neighbour links.
    fn triangle_node(e: &mut Engine, keys: [[i32; 3]; 3], next: u32, links: [u32; 3]) -> u32 {
        let triangle = e.mem.alloc(0x50);
        e.mem.set_u32(triangle, next);
        for (corner, key) in keys.iter().enumerate() {
            let vertex = e.mem.alloc(0x40);
            for (axis, value) in key.iter().enumerate() {
                e.mem.set_i32(vertex + 0x20 + 4 * axis as u32, *value);
            }
            e.mem.set_u32(triangle + 8 + 4 * corner as u32, vertex);
            e.mem
                .set_u32(triangle + 0x14 + 4 * corner as u32, links[corner]);
        }
        triangle
    }

    fn cursor(e: &mut Engine, triangle: u32, corner: u32) -> u32 {
        let at = e.mem.alloc(8);
        e.mem.set_u32(at, triangle);
        e.mem.set_u32(at + 4, corner);
        at
    }

    #[test]
    fn edge_cursor_stops_on_a_canonical_edge() {
        let (mut e, _) = bench();
        // The edge from corner 1 does not go up: the cursor stands on corner 1.
        let flat = triangle_node(&mut e, [[0, 0, 0], [1, 0, 0], [2, 0, 0]], 0, [0; 3]);
        let at = cursor(&mut e, flat, 0);
        e.call(0x00d7_c970, &args![at]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (flat, 1));
        // Corner 1 goes up and has a neighbour, corner 2 goes up without one: stops on 2.
        let stairs = triangle_node(&mut e, [[0, 0, 0], [2, 0, 0], [1, 0, 0]], 0, [0, 0x100, 0]);
        let at = cursor(&mut e, stairs, 0);
        e.call(0x00d7_c970, &args![at]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (stairs, 2));
        // The tag bits of a link do not count as a neighbour.
        let tagged = triangle_node(&mut e, [[0, 0, 0], [2, 0, 0], [1, 0, 0]], 0, [0, 3, 0]);
        let at = cursor(&mut e, tagged, 0);
        e.call(0x00d7_c970, &args![at]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (tagged, 1));
        // A null triangle stays as it is.
        let at = cursor(&mut e, 0, 2);
        e.call(0x00d7_c970, &args![at]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (0, 2));
    }

    #[test]
    fn edge_cursor_moves_on_to_the_next_triangle() {
        let (mut e, _) = bench();
        // Every edge of the first triangle goes up and has a neighbour: on to the second,
        // whose first edge does not go up.
        let second = triangle_node(&mut e, [[0, 0, 0], [1, 0, 0], [2, 0, 0]], 0, [0; 3]);
        let first = triangle_node(
            &mut e,
            [[1, 0, 0], [3, 0, 0], [2, 0, 0]],
            second,
            [0x100, 0x100, 0x100],
        );
        let at = cursor(&mut e, first, 0);
        e.call(0x00d7_c970, &args![at]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (second, 0));
        // With no further triangle the cursor ends at null.
        let last = triangle_node(
            &mut e,
            [[1, 0, 0], [3, 0, 0], [2, 0, 0]],
            0,
            [0x100, 0x100, 0x100],
        );
        let at = cursor(&mut e, last, 0);
        e.call(0x00d7_c970, &args![at]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (0, 0));
    }

    #[test]
    fn fn_00d7d220_sets_the_cursor_and_advances_it_only_off_a_rising_edge() {
        let (mut e, _) = bench();
        let at = e.mem.alloc(8);
        // Not rising: kept at the corner asked for.
        let flat = triangle_node(&mut e, [[0, 0, 0], [1, 0, 0], [2, 0, 0]], 0, [0x100; 3]);
        let got = e.call(0x00d7_d220, &args![at, flat, 0u32]).u32();
        assert_eq!(got, at);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (flat, 0));
        // Rising with a neighbour: the walk runs on to corner 1, whose edge does not rise.
        let rising = triangle_node(&mut e, [[3, 0, 0], [1, 0, 0], [2, 0, 0]], 0, [0x100; 3]);
        e.call(0x00d7_d220, &args![at, rising, 0u32]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (rising, 1));
        // Rising without a neighbour: not advanced.
        let free = triangle_node(&mut e, [[3, 0, 0], [1, 0, 0], [2, 0, 0]], 0, [0; 3]);
        e.call(0x00d7_d220, &args![at, free, 0u32]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (free, 0));
        // A null triangle is only stored.
        e.call(0x00d7_d220, &args![at, 0u32, 1u32]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (0, 1));
    }

    #[test]
    fn list_allocation_copies_and_links_vertices_and_triangles() {
        let (mut e, _) = bench();
        // The vertex list is a pool header with the list head at +0xc and the count at +0x10.
        let list = new_pool(&mut e);
        let vertex = e.mem.alloc(0x40);
        for word in 0..10u32 {
            e.mem.set_u32(vertex + 0x10 + 4 * word, 0x100 + word);
        }
        // A signalling NaN in the float word comes back quieted.
        e.mem.set_u32(vertex + 0x2c, 0x7f80_0001);
        let first = e.call(0x00d7_ef20, &args![list, vertex]).u32();
        let block = pool_block(&e, list, 0);
        assert_eq!(first, block);
        assert_eq!(e.mem.u32(block + 0x804), 1);
        assert_eq!(e.mem.u32(block + 0x800), block + 0x40);
        assert_eq!(e.mem.u32(first + 0x10), 0x100);
        assert_eq!(e.mem.u32(first + 0x2c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(first + 0x34), 0x109);
        assert_eq!((e.mem.u32(list + 0xc), e.mem.u32(list + 0x10)), (first, 1));
        assert_eq!((e.mem.u32(first), e.mem.u32(first + 4)), (0, 0));
        let second = e.call(0x00d7_ef20, &args![list, vertex]).u32();
        assert_eq!(second, block + 0x40);
        assert_eq!((e.mem.u32(list + 0xc), e.mem.u32(list + 0x10)), (second, 2));
        assert_eq!((e.mem.u32(second), e.mem.u32(second + 4)), (first, 0));
        assert_eq!(e.mem.u32(first + 4), second);
        // The triangle list copies sixteen plain words from +8.
        let list = new_pool(&mut e);
        let triangle = e.mem.alloc(0x50);
        for word in 0..16u32 {
            e.mem.set_u32(triangle + 8 + 4 * word, 0x200 + word);
        }
        let item = e.call(0x00d7_efc0, &args![list, triangle]).u32();
        let block = pool_block(&e, list, 0);
        assert_eq!(item, block);
        assert_eq!(e.mem.u32(block + 0x904), 1);
        assert_eq!(e.mem.u32(block + 0x900), block + 0x48);
        assert_eq!(e.mem.u32(item + 8), 0x200);
        assert_eq!(e.mem.u32(item + 0x44), 0x20f);
        assert_eq!((e.mem.u32(list + 0xc), e.mem.u32(list + 0x10)), (item, 1));
    }

    /// The doubles for the counter the implementations are numbered with
    /// (`InterlockedExchangeAdd`: adds, answers the old value).
    fn install_counter(e: &mut Engine) {
        e.register_double(INTERLOCKED_EXCHANGE_ADD, |e, a| {
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], old.wrapping_add(a[1]));
            Ret {
                eax: old,
                ..Ret::default()
            }
        });
    }

    /// Points in a 16-byte aligned block (fourth lane 1).
    fn aligned_points(e: &mut Engine, points: &[[f32; 3]]) -> u32 {
        let raw = e.mem.alloc(16 * points.len() as u32 + 16);
        let at = (raw + 15) & !15;
        for (index, p) in points.iter().enumerate() {
            put_vec(e, at + 16 * index as u32, [p[0], p[1], p[2], 1.0]);
        }
        at
    }

    const CUBE: [[f32; 3]; 8] = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
    ];

    /// A hull constructed through the exe's constructor and built over `points`; answers
    /// `(hull, implementation)`.
    fn built_hull(e: &mut Engine, points: &[[f32; 3]]) -> (u32, u32) {
        install_counter(e);
        let block = e.mem.alloc(0x14);
        let hull = e.call(0x00d7_6ca0, &args![block]).u32();
        let implementation = e.mem.u32(hull + 8);
        let at = aligned_points(e, points);
        e.call(
            0x00d8_1510,
            &args![implementation, at, points.len() as i32, 0u32, 0u32],
        );
        (hull, implementation)
    }

    #[test]
    fn build_of_a_cube_makes_a_solid_with_indices() {
        let (mut e, _) = bench();
        install_counter(&mut e);
        let block = e.mem.alloc(0x14);
        let hull = e.call(0x00d7_6ca0, &args![block]).u32();
        let imp = e.mem.u32(hull + 8);
        let points = aligned_points(&mut e, &CUBE);
        let dims = e
            .call(0x00d8_1510, &args![imp, points, 8i32, 0u32, 0u32])
            .i32();
        assert_eq!(dims, 3);
        assert_eq!(e.mem.i32(imp + 0x1a8), 3);
        assert_eq!(e.mem.i32(imp + 0x28), 8);
        assert_eq!(e.mem.i32(imp + 0x40), 12);
        assert_eq!(e.mem.u8(imp + 0x1b5), 1);
        assert_eq!(e.mem.i32(imp + 0x58), 6);
    }

    /// The nodes of the vertex list of the implementation.
    fn vertex_nodes(e: &Engine, imp: u32) -> Vec<u32> {
        let mut nodes = Vec::new();
        let mut node = e.mem.u32(imp + 0x24);
        while node != 0 {
            nodes.push(node);
            node = e.mem.u32(node);
        }
        nodes
    }

    /// The nodes of the triangle list of the implementation.
    fn triangle_nodes(e: &Engine, imp: u32) -> Vec<u32> {
        let mut nodes = Vec::new();
        let mut node = e.mem.u32(imp + 0x3c);
        while node != 0 {
            nodes.push(node);
            node = e.mem.u32(node);
        }
        nodes
    }

    /// The positions (`+0x10`, first three lanes) of the vertices, sorted.
    fn vertex_positions(e: &Engine, imp: u32) -> Vec<[f32; 3]> {
        let positions: Vec<[f32; 3]> = vertex_nodes(e, imp)
            .iter()
            .map(|node| {
                let v = get_vec(e, node + 0x10);
                [v[0], v[1], v[2]]
            })
            .collect();
        sorted(&positions)
    }

    /// The points sorted lexicographically.
    fn sorted(points: &[[f32; 3]]) -> Vec<[f32; 3]> {
        let mut points = points.to_vec();
        points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        points
    }

    /// A hull constructed through the exe constructor, without a build: `(hull, implementation)`.
    fn fresh_hull(e: &mut Engine) -> (u32, u32) {
        install_counter(e);
        let block = e.mem.alloc(0x14);
        let hull = e.call(0x00d7_6ca0, &args![block]).u32();
        (hull, e.mem.u32(hull + 8))
    }

    #[test]
    fn build_of_a_cube_has_its_vertices_and_six_unit_planes() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        assert_eq!(vertex_positions(&e, imp), sorted(&CUBE));
        let planes = e.mem.u32(imp + 0x54);
        let positions = vertex_positions(&e, imp);
        let mut faces = Vec::new();
        for index in 0..6 {
            let plane = get_vec(&e, planes + 16 * index);
            let length = (plane[0] * plane[0] + plane[1] * plane[1] + plane[2] * plane[2]).sqrt();
            assert_close(length, 1.0);
            // Every vertex is on or behind the plane and four are on it.
            let distances: Vec<f32> = positions
                .iter()
                .map(|p| plane[0] * p[0] + plane[1] * p[1] + plane[2] * p[2] + plane[3])
                .collect();
            assert!(distances.iter().all(|d| *d < 1e-3), "{plane:?}");
            assert_eq!(distances.iter().filter(|d| d.abs() < 1e-3).count(), 4);
            faces.push((
                plane[0].round() as i32,
                plane[1].round() as i32,
                plane[2].round() as i32,
            ));
        }
        faces.sort_unstable();
        assert_eq!(
            faces,
            vec![
                (-1, 0, 0),
                (0, -1, 0),
                (0, 0, -1),
                (0, 0, 1),
                (0, 1, 0),
                (1, 0, 0)
            ]
        );
    }

    #[test]
    fn build_of_a_tetrahedron_with_the_unit_grid() {
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        // The configuration byte at +0xc makes the grid the unit grid.
        e.mem.set_u8(imp + 0xc, 1);
        let points: [[f32; 3]; 4] = [
            [0.0, 0.0, 0.0],
            [10.0, 0.0, 0.0],
            [0.0, 10.0, 0.0],
            [0.0, 0.0, 10.0],
        ];
        let at = aligned_points(&mut e, &points);
        let dims = e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]).i32();
        assert_eq!(dims, 3);
        assert_eq!(e.mem.i32(imp + 0x28), 4);
        assert_eq!(e.mem.i32(imp + 0x40), 4);
        assert_eq!(get_vec(&e, imp + 0xf0), [0.0; 4]);
        assert_eq!(get_vec(&e, imp + 0x100), [1.0, 1.0, 1.0, 0.0]);
        assert_eq!(get_vec(&e, imp + 0x110), [1.0, 1.0, 1.0, 0.0]);
        let mut grid: Vec<[i32; 3]> = vertex_nodes(&e, imp)
            .iter()
            .map(|n| {
                [
                    e.mem.i32(n + 0x20),
                    e.mem.i32(n + 0x24),
                    e.mem.i32(n + 0x28),
                ]
            })
            .collect();
        grid.sort_unstable();
        assert_eq!(grid, vec![[0, 0, 0], [0, 0, 10], [0, 10, 0], [10, 0, 0]]);
    }

    #[test]
    fn build_of_a_square_is_a_polygon_without_its_inner_point() {
        let (mut e, _) = bench();
        let square = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.5, 0.5, 0.0],
        ];
        // Five points ask for three dimensions; the retry byte at +0 lets the hull fall to two.
        let (_, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let at = aligned_points(&mut e, &square);
        e.call(0x00d8_1510, &args![imp, at, 5i32, 0u32, 0u32]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 2);
        assert_eq!(e.mem.i32(imp + 0x28), 4);
        assert_eq!(e.mem.i32(imp + 0x40), 0);
        assert_eq!(vertex_positions(&e, imp), sorted(&square[..4]));
        // The normal of the plane of the points (the cross product of the first two edges).
        let normal = get_vec(&e, imp + 0xe0);
        assert_vec_close(normal, [0.0, 0.0, -1.0, 0.0]);
    }

    #[test]
    fn build_of_collinear_points_is_a_segment_between_the_extremes() {
        let (mut e, _) = bench();
        let line = [[1.0, 1.0, 1.0], [3.0, 3.0, 3.0], [2.0, 2.0, 2.0]];
        let (_, imp) = built_hull(&mut e, &line);
        assert_eq!(e.mem.i32(imp + 0x1a8), 1);
        assert_eq!(
            vertex_positions(&e, imp),
            vec![[1.0, 1.0, 1.0], [3.0, 3.0, 3.0]]
        );
    }

    #[test]
    fn build_of_one_point_or_equal_points_is_a_single_vertex() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &[[2.0, 3.0, 4.0]]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 0);
        assert_eq!(vertex_positions(&e, imp), vec![[2.0, 3.0, 4.0]]);
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &[[2.0, 3.0, 4.0], [2.0, 3.0, 4.0]]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 0);
        assert_eq!(e.mem.i32(imp + 0x28), 1);
    }

    #[test]
    fn build_of_no_points_only_resets() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let points = aligned_points(&mut e, &CUBE);
        let dims = e
            .call(0x00d8_1510, &args![imp, points, 0i32, 0u32, 0u32])
            .i32();
        assert_eq!(dims, -1);
        assert_eq!(e.mem.i32(imp + 0x28), 0);
        assert_eq!(e.mem.u32(imp + 0x24), 0);
    }

    #[test]
    fn build_copies_unaligned_points_to_an_aligned_array() {
        let (mut e, frees) = bench();
        let (_, imp) = fresh_hull(&mut e);
        let aligned = aligned_points(&mut e, &CUBE);
        // The same eight points, 8 bytes off a 16-byte boundary.
        let raw = e.mem.alloc(16 * 8 + 32);
        let unaligned = ((raw + 15) & !15) + 8;
        for word in 0..32 {
            let value = e.mem.u32(aligned + 4 * word);
            e.mem.set_u32(unaligned + 4 * word, value);
        }
        let dims = e
            .call(0x00d8_1510, &args![imp, unaligned, 8i32, 0u32, 0u32])
            .i32();
        assert_eq!(dims, 3);
        assert_eq!(vertex_positions(&e, imp), sorted(&CUBE));
        // The temporary array (8 vectors) went back through the allocator.
        assert!(frees.borrow().iter().any(|(_, a)| a[2] == 8 * 16));
    }

    #[test]
    fn build_takes_the_grid_of_another_implementation() {
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        let source = e.mem.alloc(0x1c0);
        put_vec(&mut e, source + 0xf0, [0.5, 0.5, 0.5, 0.0]);
        put_vec(&mut e, source + 0x100, [100.0, 100.0, 100.0, 0.0]);
        put_vec(&mut e, source + 0x110, [0.01, 0.01, 0.01, 0.0]);
        let points = aligned_points(&mut e, &CUBE);
        let dims = e
            .call(0x00d8_1510, &args![imp, points, 8i32, 0u32, source])
            .i32();
        assert_eq!(dims, 3);
        assert_eq!(get_vec(&e, imp + 0xf0), [0.5, 0.5, 0.5, 0.0]);
        assert_eq!(get_vec(&e, imp + 0x100), [100.0, 100.0, 100.0, 0.0]);
        assert_eq!(get_vec(&e, imp + 0x110), [0.01, 0.01, 0.01, 0.0]);
        // The grid coordinates are (p - 0.5) * 100 rounded: -50 and 50.
        let node = vertex_nodes(&e, imp)[0];
        let coordinate = e.mem.i32(node + 0x20);
        assert!(coordinate == 50 || coordinate == -50, "{coordinate}");
    }

    #[test]
    fn build_keeping_the_normal_makes_a_polygon_in_that_plane() {
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        put_vec(&mut e, imp + 0xe0, [0.0, 0.0, 1.0, 0.0]);
        let points = aligned_points(&mut e, &CUBE[..4]);
        let dims = e
            .call(0x00d8_1510, &args![imp, points, 4i32, 1u32, 0u32])
            .i32();
        assert_eq!(dims, 2);
        assert_eq!(get_vec(&e, imp + 0xe0), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.mem.i32(imp + 0x28), 4);
    }

    #[test]
    fn build_of_coplanar_points_falls_back_to_a_polygon_or_fails() {
        let square = [
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [0.0, 2.0, 0.0],
        ];
        // With the retry byte at +0 set the solid fails and a polygon is built.
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let at = aligned_points(&mut e, &square);
        // Four points ask for three dimensions; these have no volume.
        let dims = e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]).i32();
        assert_eq!(dims, 2);
        assert_eq!(e.mem.i32(imp + 0x28), 4);
        // Without it the build fails.
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &square);
        assert_eq!(e.mem.i32(imp + 0x1a8), -1);
    }

    #[test]
    fn build_of_points_beyond_the_float_range_is_reported() {
        let (mut e, _) = bench();
        install_report(&mut e, 1);
        let (_, imp) = fresh_hull(&mut e);
        let at = aligned_points(&mut e, &[[f32::INFINITY, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        let message = int3_message(&mut e, 0x00d8_1510, &args![imp, at, 2i32, 0u32, 0u32]);
        assert!(message.contains("INT3 at 00d817a8"), "{message}");
    }

    #[test]
    fn build_of_a_segment_along_the_third_axis_collapses_to_a_point() {
        // The exe compares only the first two grid coordinates of the two end items to decide
        // that a segment is a single point, so a segment along z is one vertex.
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &[[0.0, 0.0, 0.0], [0.0, 0.0, 4.0]]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 0);
        assert_eq!(e.mem.i32(imp + 0x28), 1);
    }

    const OBJECT_CLONE_DOUBLE: u32 = 0x0900_0031;

    /// An owned object: slot 0 (delete) logs `(OBJECT_SLOT_DOUBLE, words)` and slot 4 (clone)
    /// logs `(OBJECT_CLONE_DOUBLE, words)` and answers `copy`.
    fn owned_object(e: &mut Engine, copy: u32) -> (u32, Log) {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let delete_log = Rc::clone(&log);
        e.register_double(OBJECT_SLOT_DOUBLE, move |_, a| {
            delete_log
                .borrow_mut()
                .push((OBJECT_SLOT_DOUBLE, a.to_vec()));
            Ret::default()
        });
        let clone_log = Rc::clone(&log);
        e.register_double(OBJECT_CLONE_DOUBLE, move |_, a| {
            clone_log
                .borrow_mut()
                .push((OBJECT_CLONE_DOUBLE, a.to_vec()));
            Ret {
                eax: copy,
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, OBJECT_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 4, OBJECT_CLONE_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        (object, log)
    }

    #[test]
    fn hull_constructor_makes_an_empty_implementation() {
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        assert_ne!(imp, 0);
        assert_eq!(e.mem.u32(hull), HULL_VTABLE);
        assert_eq!(e.mem.u16(hull + 6), 1);
        assert_eq!(e.mem.u32(hull + 0xc), 0);
        assert_eq!(e.mem.u32(hull + 0x10), 0);
        assert_eq!(e.mem.i32(imp + 0x1a8), -1);
        assert_eq!(e.mem.u32(imp + 0x1b0), 0);
        // A constructor given a block that already holds a hull answers it.
        let again = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00d7_6ca0, &args![again]).u32(), again);
        let second = e.mem.u32(again + 8);
        assert_eq!(e.mem.u32(second + 0x1b0), 1);
    }

    #[test]
    fn implementation_constructor_sets_up_the_arrays_and_the_defaults() {
        let (mut e, _) = bench();
        install_counter(&mut e);
        let block = e.mem.alloc(0x1c0);
        for offset in (0..0x1c0).step_by(4) {
            e.mem.set_u32(block + offset, 0xdead_beef);
        }
        let got = e.call(0x00d7_f6a0, &args![block]).u32();
        assert_eq!(got, block);
        // The build configuration (the tolerance float, the flag bytes).
        assert_eq!(e.mem.u32(block + 4), 0x3f7f_f972);
        assert_eq!(e.mem.u8(block), 0);
        assert_eq!(e.mem.u8(block + 8), 0);
        assert_eq!(e.mem.u8(block + 9), 1);
        assert_eq!(e.mem.u8(block + 0xa), 0);
        assert_eq!(e.mem.u8(block + 0xb), 1);
        assert_eq!(e.mem.u32(block + 0x10), MESH_VTABLE);
        assert_eq!(e.mem.u16(block + 0x16), 1);
        for capacity in [0x20, 0x38, 0x50, 0x5c, 0x68] {
            assert_eq!(e.mem.u32(block + capacity), 0x8000_0000, "{capacity:x}");
        }
        for zeroed in [
            0x18, 0x1c, 0x24, 0x28, 0x30, 0x34, 0x3c, 0x40, 0x48, 0x4c, 0x54, 0x58, 0x60, 0x64,
        ] {
            assert_eq!(e.mem.u32(block + zeroed), 0, "{zeroed:x}");
        }
        assert_eq!(e.mem.u32(block + 0x1b0), 0);
        assert_eq!(e.mem.i32(block + 0x1a8), -1);
        // The counter numbers the next one.
        let next = e.mem.alloc(0x1c0);
        e.call(0x00d7_f6a0, &args![next]);
        assert_eq!(e.mem.u32(next + 0x1b0), 1);
        assert_eq!(e.mem.u32(IMPLEMENTATION_COUNTER), 2);
    }

    #[test]
    fn implementation_reset_returns_to_the_initial_state() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        put_vec(&mut e, imp + 0xe0, [0.0, 0.0, 1.0, 5.0]);
        e.mem.set_u8(imp + 0x1b4, 1);
        e.mem.set_u32(imp + 0x1ac, 9);
        e.mem.set_u32(imp + 0x1a0, 0x3f80_0000);
        // Rows the reset copies from the exe data.
        for row in 0..4u32 {
            put_vec(&mut e, 0x0126_8370 + 0x10 * row, [row as f32 + 1.0; 4]);
        }
        e.call(0x00d7_ac40, &args![imp, 1u32]);
        assert_eq!(e.mem.u32(imp + 0x24), 0);
        assert_eq!(e.mem.u32(imp + 0x28), 0);
        assert_eq!(e.mem.u32(imp + 0x3c), 0);
        assert_eq!(e.mem.u32(imp + 0x40), 0);
        for offset in [0x58, 0x4c, 0x64, 0x1ac, 0x1a0, 0x1a4] {
            assert_eq!(e.mem.u32(imp + offset), 0, "{offset:x}");
        }
        assert_eq!((e.mem.u8(imp + 0x1b4), e.mem.u8(imp + 0x1b5)), (0, 0));
        assert_eq!(e.mem.i32(imp + 0x1a8), -1);
        let largest = f32::from_bits(0x7f7f_ffee);
        let smallest = f32::from_bits(0xff7f_ffee);
        assert_eq!(get_vec(&e, imp + 0x70), [0.0, 0.0, 0.0, largest]);
        assert_eq!(get_vec(&e, imp + 0x80), [0.0, 0.0, 0.0, largest]);
        assert_eq!(get_vec(&e, imp + 0x90), [largest; 4]);
        assert_eq!(get_vec(&e, imp + 0xa0), [smallest; 4]);
        assert_eq!(get_vec(&e, imp + 0xb0), [largest; 4]);
        assert_eq!(get_vec(&e, imp + 0xc0), [smallest; 4]);
        for row in 0..4u32 {
            assert_eq!(get_vec(&e, imp + 0x120 + 0x10 * row), [row as f32 + 1.0; 4]);
        }
        assert_eq!(get_vec(&e, imp + 0x170), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(get_vec(&e, imp + 0x180), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(get_vec(&e, imp + 0x190), [0.0, 0.0, 1.0, 0.0]);
        for offset in [0xd0, 0xf0, 0x100, 0x110, 0x160] {
            assert_eq!(get_vec(&e, imp + offset), [0.0; 4], "{offset:x}");
        }
        // The normal is kept when asked for, else cleared.
        assert_eq!(get_vec(&e, imp + 0xe0), [0.0, 0.0, 1.0, 5.0]);
        e.call(0x00d7_ac40, &args![imp, 0u32]);
        assert_eq!(get_vec(&e, imp + 0xe0), [0.0; 4]);
    }

    /// Whether two triangle lists and vertex lists hold the same shapes: the vertex positions
    /// of every triangle (in list order) and the number of linked edges.
    fn triangle_shapes(e: &Engine, imp: u32) -> Vec<[[f32; 3]; 3]> {
        triangle_nodes(e, imp)
            .iter()
            .map(|t| {
                [0u32, 1, 2].map(|corner| {
                    let v = get_vec(e, e.mem.u32(t + 8 + 4 * corner) + 0x10);
                    [v[0], v[1], v[2]]
                })
            })
            .collect()
    }

    /// Every link of every triangle of the implementation names a triangle of the same list and
    /// is answered by the neighbour's link back.
    fn assert_links_are_symmetric(e: &Engine, imp: u32) {
        let triangles = triangle_nodes(e, imp);
        for triangle in &triangles {
            for corner in 0..3u32 {
                let link = e.mem.u32(triangle + 0x14 + 4 * corner);
                let (neighbour, back) = (link & !3, link & 3);
                assert!(triangles.contains(&neighbour), "{link:x}");
                assert_eq!(
                    e.mem.u32(neighbour + 0x14 + 4 * back),
                    triangle + corner,
                    "triangle {triangle:x} corner {corner}"
                );
            }
        }
    }

    #[test]
    fn implementation_clone_copies_the_hull_and_relinks_the_triangles() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        e.mem.set_u32(imp, 0x11);
        e.mem.set_u32(imp + 8, 0x22);
        put_vec(&mut e, imp + 0xd0, [1.0, 2.0, 3.0, 4.0]);
        e.mem.set_u8(imp + 0x1b4, 1);
        e.mem.set_u32(imp + 0x1a0, 0x7fc0_0000);
        assert_links_are_symmetric(&e, imp);
        let copy = e.call(0x00d7_f720, &args![imp]).u32();
        assert_ne!(copy, imp);
        for offset in [0x0u32, 0x8] {
            assert_eq!(e.mem.u32(copy + offset), e.mem.u32(imp + offset));
        }
        assert_eq!(get_vec(&e, copy + 0xd0), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u32(copy + 0x1a0), 0x7fc0_0000);
        assert_eq!(e.mem.i32(copy + 0x1a8), 3);
        assert_eq!(e.mem.u8(copy + 0x1b4), 1);
        assert_eq!(e.mem.u8(copy + 0x1b5), 1);
        assert_eq!(e.mem.i32(copy + 0x28), 8);
        assert_eq!(e.mem.i32(copy + 0x40), 12);
        assert_eq!(vertex_positions(&e, copy), vertex_positions(&e, imp));
        // The copy owns its own nodes: none is shared.
        for node in vertex_nodes(&e, copy) {
            assert!(!vertex_nodes(&e, imp).contains(&node));
        }
        assert_eq!(triangle_shapes(&e, copy), triangle_shapes(&e, imp));
        assert_links_are_symmetric(&e, copy);
        // The planes and the items were copied.
        assert_eq!(e.mem.i32(copy + 0x58), 6);
        for plane in 0..6u32 {
            let from = e.mem.u32(imp + 0x54) + 16 * plane;
            let to = e.mem.u32(copy + 0x54) + 16 * plane;
            assert_eq!(get_vec(&e, from), get_vec(&e, to));
        }
        assert_eq!(e.mem.i32(copy + 0x4c), e.mem.i32(imp + 0x4c));
        // The copy has its own number.
        assert_ne!(e.mem.u32(copy + 0x1b0), e.mem.u32(imp + 0x1b0));
    }

    #[test]
    fn hull_clone_copies_the_implementation_the_word_and_the_owned_object() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &CUBE);
        let (object, log) = owned_object(&mut e, 0x5555);
        e.mem.set_u32(hull + 0x10, object);
        e.mem.set_u32(hull + 0xc, 0x77);
        let copy = e.call(0x00d7_6cf0, &args![hull]).u32();
        assert_ne!(copy, hull);
        assert_eq!(e.mem.u32(copy), HULL_VTABLE);
        assert_eq!(e.mem.u16(copy + 4), 0x14);
        assert_eq!(e.mem.u32(copy + 0xc), 0x77);
        assert_eq!(e.mem.u32(copy + 0x10), 0x5555);
        assert_eq!(*log.borrow(), vec![(OBJECT_CLONE_DOUBLE, vec![object])]);
        let copied = e.mem.u32(copy + 8);
        assert_ne!(copied, imp);
        assert_eq!(vertex_positions(&e, copied), vertex_positions(&e, imp));
        assert_eq!(e.mem.i32(copied + 0x1a8), 3);
        // The implementation the new hull made for itself was destroyed and freed.
        // Without an owned object the copy has none either.
        e.mem.set_u32(hull + 0x10, 0);
        let plain = e.call(0x00d7_6cf0, &args![hull]).u32();
        assert_eq!(e.mem.u32(plain + 0x10), 0);
    }

    #[test]
    fn hull_destructor_destroys_the_implementation_and_the_owned_object() {
        let (mut e, frees) = bench();
        let (hull, imp) = built_hull(&mut e, &CUBE);
        let (object, log) = owned_object(&mut e, 0);
        e.mem.set_u32(hull + 0x10, object);
        frees.borrow_mut().clear();
        e.call(0x00d7_6940, &args![hull]);
        assert_eq!(e.mem.u32(hull), HULL_BASE_VTABLE);
        assert_eq!(e.mem.u32(hull + 0x10), 0);
        assert_eq!(*log.borrow(), vec![(OBJECT_SLOT_DOUBLE, vec![object, 1])]);
        // The implementation block went back with its size, after its arrays.
        let frees = frees.borrow();
        let last = frees.last().unwrap();
        assert_eq!(
            (last.0, last.1[1], last.1[2]),
            (FREE_SLOT_DOUBLE, imp, 0x1c0)
        );
        assert!(frees.iter().any(|(d, _)| *d == FREE_ARRAY_SLOT_DOUBLE));
        // No implementation and no owned object: nothing is freed.
        let (mut e, frees) = bench();
        let hull = e.mem.alloc(0x14);
        e.call(0x00d7_6940, &args![hull]);
        assert!(frees.borrow().is_empty());
        assert_eq!(e.mem.u32(hull), HULL_BASE_VTABLE);
    }

    #[test]
    fn implementation_destructor_frees_the_arrays_and_the_pools() {
        let (mut e, frees) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let planes = e.mem.u32(imp + 0x54);
        let plane_bytes = (e.mem.u32(imp + 0x5c) & 0x3fff_ffff) * 0x10;
        frees.borrow_mut().clear();
        e.call(0x00d7_ce20, &args![imp]);
        for (data, capacity) in [(0x48, 0x50), (0x54, 0x5c), (0x60, 0x68)] {
            assert_eq!(e.mem.u32(imp + data), 0);
            assert_eq!(e.mem.u32(imp + capacity), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(imp + 0x10), HULL_BASE_VTABLE);
        let frees = frees.borrow();
        let array_frees: Vec<(u32, u32)> = frees
            .iter()
            .filter(|(d, _)| *d == FREE_ARRAY_SLOT_DOUBLE)
            .map(|(_, a)| (a[1], a[2]))
            .collect();
        // The item array was released by the build (non-owning), so only the planes and the two
        // block lists of the pools (four block pointers each) go back.
        assert_eq!(array_frees.len(), 3, "{array_frees:?}");
        assert_eq!(array_frees[0], (planes, plane_bytes));
        assert_eq!(array_frees[1].1, 16);
        assert_eq!(array_frees[2].1, 16);
        assert_eq!(plane_bytes, 6 * 16);
        // The pools gave their blocks back (the reset at the start frees the vertex pool first): eight
        // vertices and twelve triangles fit one block each.
        let sizes: Vec<u32> = frees
            .iter()
            .filter(|(d, _)| *d == FREE_SLOT_DOUBLE)
            .map(|(_, a)| a[2])
            .collect();
        assert_eq!(sizes, vec![0x808, 0x908]);
    }

    #[test]
    fn fn_00d75850_resets_the_implementation_and_deletes_the_owned_object() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &CUBE);
        let (object, log) = owned_object(&mut e, 0);
        e.mem.set_u32(hull + 0x10, object);
        e.call(0x00d7_5850, &args![hull]);
        assert_eq!(e.mem.i32(imp + 0x1a8), -1);
        assert_eq!(e.mem.u32(imp + 0x28), 0);
        assert_eq!(e.mem.u32(hull + 0x10), 0);
        assert_eq!(*log.borrow(), vec![(OBJECT_SLOT_DOUBLE, vec![object, 1])]);
    }

    #[test]
    fn hull_build_indices_rebuilds_the_planes_with_the_given_tolerance() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &CUBE);
        assert_eq!(e.mem.u8(imp + 0x1b5), 1);
        // Without the clear byte the valid indices stay and only the float is stored.
        e.call(0x00d7_6990, &args![hull, 0.75f32.to_bits(), 0u32]);
        assert_eq!(e.mem.f32(imp + 4), 0.75);
        assert_eq!(e.mem.u8(imp + 0x1b5), 1);
        // With it they are rebuilt from the triangles.
        let planes = e.mem.u32(imp + 0x54);
        e.mem.set_u32(planes, 0xdead_beef);
        e.call(0x00d7_6990, &args![hull, 0.5f32.to_bits(), 1u32]);
        assert_eq!(e.mem.f32(imp + 4), 0.5);
        assert_eq!(e.mem.u8(imp + 0x1b5), 1);
        assert_eq!(e.mem.i32(imp + 0x58), 6);
        let plane = get_vec(&e, e.mem.u32(imp + 0x54));
        assert_ne!(plane[0].to_bits(), 0xdead_beef);
    }

    use crate::units::havok_sdk::hkpsphereshape::TLS_GET_VALUE;

    /// Where the profiling record stream of the thread is (`TlsGetValue` answers it): the
    /// cursor at `+4` and the limit at `+0xc`, room for eight 12-byte records.
    const PROFILE_STREAM_AT: u32 = 0x0129_0000;

    /// The labels of the profiling records written so far.
    fn profile_labels(e: &Engine) -> Vec<u32> {
        let first = PROFILE_STREAM_AT + 0x10;
        let cursor = e.mem.u32(PROFILE_STREAM_AT + 4);
        (first..cursor)
            .step_by(12)
            .map(|at| e.mem.u32(at))
            .collect()
    }

    /// The vector and matrix doubles: `setTransformedPos` and `setRotatedDir` as the exe
    /// computes them, `hkMatrix3` determinant, inverse, cross matrix, product, transpose,
    /// subtraction and scaled addition, a diagonalizer that takes the matrix to be diagonal
    /// already (identity rotation, the diagonal as the answer), the basis change and the rotation
    /// functions (no change for the identity), and the thread's profile stream.
    fn install_math(e: &mut Engine) {
        e.mem
            .set_u32(PROFILE_STREAM_AT + 4, PROFILE_STREAM_AT + 0x10);
        e.mem
            .set_u32(PROFILE_STREAM_AT + 0xc, PROFILE_STREAM_AT + 0x10 + 12 * 8);
        e.register(TLS_GET_VALUE, |_, _| Ret {
            eax: PROFILE_STREAM_AT,
            ..Ret::default()
        });
        fn transformed(e: &mut Engine, a: &[u32], translate: bool) -> Ret {
            let position = load_vec(e, a[2]);
            let columns = [0u32, 0x10, 0x20, 0x30].map(|offset| load_vec(e, a[1] + offset));
            let by_y = mul(splat(position[1]), columns[1]);
            let by_x = mul(splat(position[0]), columns[0]);
            let by_z = mul(splat(position[2]), columns[2]);
            let tail = if translate {
                add(by_z, columns[3])
            } else {
                by_z
            };
            let sum = add(add(by_y, by_x), tail);
            store_vec(e, a[0], sum);
            Ret::default()
        }
        e.register(SET_TRANSFORMED_POS, |e, a| transformed(e, a, true));
        e.register(SET_ROTATED_DIR, |e, a| transformed(e, a, false));
        e.register(MATRIX3_INVERT, |e, a| {
            let columns = [0u32, 0x10, 0x20].map(|offset| get_vec(e, a[0] + offset));
            let m = |row: usize, column: usize| f64::from(columns[column][row]);
            let determinant = m(0, 0) * (m(1, 1) * m(2, 2) - m(1, 2) * m(2, 1))
                - m(0, 1) * (m(1, 0) * m(2, 2) - m(1, 2) * m(2, 0))
                + m(0, 2) * (m(1, 0) * m(2, 1) - m(1, 1) * m(2, 0));
            if determinant.abs() < f64::from(f32::from_bits(a[1])) {
                return Ret {
                    eax: 1,
                    ..Ret::default()
                };
            }
            let cofactor = |row: usize, column: usize| {
                let (r1, r2) = ((row + 1) % 3, (row + 2) % 3);
                let (c1, c2) = ((column + 1) % 3, (column + 2) % 3);
                m(r1, c1) * m(r2, c2) - m(r1, c2) * m(r2, c1)
            };
            for column in 0..3 {
                let mut inverse = [0.0f32; 4];
                for (row, value) in inverse.iter_mut().take(3).enumerate() {
                    *value = (cofactor(column, row) / determinant) as f32;
                }
                put_vec(e, a[0] + 0x10 * column as u32, inverse);
            }
            Ret::default()
        });
        e.register(MATRIX3_GET_DETERMINANT, |e, a| {
            let columns = [0u32, 0x10, 0x20].map(|offset| get_vec(e, a[0] + offset));
            let c = |column: usize, row: usize| columns[column][row];
            let determinant = (c(1, 2) * c(0, 1) - c(0, 2) * c(1, 1)) * c(2, 0)
                + (c(0, 2) * c(2, 1) - c(2, 2) * c(0, 1)) * c(1, 0)
                + (c(2, 2) * c(1, 1) - c(1, 2) * c(2, 1)) * c(0, 0);
            put_vec(e, a[1], [determinant, 0.0, 0.0, 0.0]);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(MATRIX3_SKEW_SYMMETRIC, |e, a| {
            let v = get_vec(e, a[1]);
            put_vec(e, a[0], [0.0, v[2], -v[1], 0.0]);
            put_vec(e, a[0] + 0x10, [-v[2], 0.0, v[0], 0.0]);
            put_vec(e, a[0] + 0x20, [v[1], -v[0], 0.0, 0.0]);
            Ret::default()
        });
        e.register(MATRIX3_SET_MUL, |e, a| {
            let first = [0u32, 0x10, 0x20].map(|offset| get_vec(e, a[1] + offset));
            let second = [0u32, 0x10, 0x20].map(|offset| get_vec(e, a[2] + offset));
            for (column, b) in second.iter().enumerate() {
                let sum = add(
                    add(mul(splat(b[0]), first[0]), mul(splat(b[1]), first[1])),
                    mul(splat(b[2]), first[2]),
                );
                put_vec(e, a[0] + 0x10 * column as u32, sum);
            }
            Ret::default()
        });
        e.register(MATRIX3_TRANSPOSE, |e, a| {
            for (low, high) in [(1u32, 4u32), (2, 8), (6, 9)] {
                let (first, second) = (a[0] + 4 * low, a[0] + 4 * high);
                let (x, y) = (e.mem.u32(first), e.mem.u32(second));
                e.mem.set_u32(first, y);
                e.mem.set_u32(second, x);
            }
            Ret::default()
        });
        e.register(MATRIX3_SUB, |e, a| {
            for column in 0..3u32 {
                let difference = sub(
                    get_vec(e, a[0] + 0x10 * column),
                    get_vec(e, a[1] + 0x10 * column),
                );
                put_vec(e, a[0] + 0x10 * column, difference);
            }
            Ret::default()
        });
        e.register(MATRIX3_ADD_SCALED, |e, a| {
            let scale = e.mem.f32(a[1]);
            for column in 0..3u32 {
                let sum = add(
                    mul(splat(scale), get_vec(e, a[2] + 0x10 * column)),
                    get_vec(e, a[0] + 0x10 * column),
                );
                put_vec(e, a[0] + 0x10 * column, sum);
            }
            Ret::default()
        });
        e.register(MATRIX3_DIAGONALIZE, |e, a| {
            let diagonal = [
                e.mem.f32(a[0]),
                e.mem.f32(a[0] + 0x14),
                e.mem.f32(a[0] + 0x28),
                0.0,
            ];
            put_vec(e, a[2], diagonal);
            for column in 0..3u32 {
                let mut identity = [0.0; 4];
                identity[column as usize] = 1.0;
                put_vec(e, a[1] + 0x10 * column, identity);
            }
            Ret::default()
        });
        // `this = other * this` for 4x4 matrices of four row vectors (`00de8e50`).
        e.register(MATRIX4_MUL_ASSIGN, |e, a| {
            let this = [0u32, 0x10, 0x20, 0x30].map(|offset| get_vec(e, a[0] + offset));
            let other = [0u32, 0x10, 0x20, 0x30].map(|offset| get_vec(e, a[1] + offset));
            for row in 0..4 {
                let mut product = [0.0f32; 4];
                for (column, value) in product.iter_mut().enumerate() {
                    *value = (0..4).map(|k| other[row][k] * this[k][column]).sum();
                }
                put_vec(e, a[0] + 0x10 * row as u32, product);
            }
            Ret::default()
        });
        e.register(MATRIX3_CHANGE_BASIS, |_, _| Ret::default());
        e.register(ROTATION_RENORMALIZE, |_, _| Ret::default());
        e.register_double(ROTATION_SET, |e, a| {
            for column in 0..3u32 {
                let mut identity = [0.0; 4];
                identity[column as usize] = 1.0;
                put_vec(e, a[0] + 0x10 * column, identity);
            }
            Ret::default()
        });
    }

    #[test]
    fn mass_properties_of_a_cube() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        assert_eq!(e.mem.u8(imp + 0x1b4), 0);
        let failed = e.call(0x00d7_b600, &args![imp]).bool();
        assert!(!failed);
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        assert_close(e.mem.f32(imp + 0x1a0), 1.0);
        assert_close(e.mem.f32(imp + 0x1a4), 6.0);
        assert_vec_close(get_vec(&e, imp + 0x150), [0.5, 0.5, 0.5, 0.0]);
        // The inertia of a unit cube per unit volume about its centre is 1/6 on the diagonal.
        let diagonal = get_vec(&e, imp + 0x160);
        assert_vec_close(diagonal, [1.0 / 6.0, 1.0 / 6.0, 1.0 / 6.0, 0.0]);
        // The tensor itself is stored as three rows at +0x170.
        let first = get_vec(&e, imp + 0x170);
        assert_close(first[0], 1.0 / 6.0);
        assert!(first[1].abs() < 1e-4 && first[2].abs() < 1e-4);
        // The rotation is the identity (the diagonalizer double takes the tensor as diagonal).
        assert_vec_close(get_vec(&e, imp + 0x120), [1.0, 0.0, 0.0, 0.0]);
        assert_vec_close(get_vec(&e, imp + 0x140), [0.0, 0.0, 1.0, 0.0]);
        // A second call finds the properties valid and changes nothing.
        e.mem.set_f32(imp + 0x1a0, 7.0);
        e.call(0x00d7_b600, &args![imp]);
        assert_eq!(e.mem.f32(imp + 0x1a0), 7.0);
    }

    #[test]
    fn mass_properties_of_a_box_scale_with_its_extents() {
        let (mut e, _) = bench();
        let slab: Vec<[f32; 3]> = CUBE
            .iter()
            .map(|p| [p[0] * 2.0, p[1], p[2] * 4.0])
            .collect();
        let (_, imp) = built_hull(&mut e, &slab);
        e.call(0x00d7_b600, &args![imp]);
        assert_close(e.mem.f32(imp + 0x1a0), 8.0);
        // Surface area 2 * (2 * 1 + 2 * 4 + 1 * 4) = 28, halved by the exe's sum: 28 / 2 * 2.
        assert_close(e.mem.f32(imp + 0x1a4), 28.0);
        assert_vec_close(get_vec(&e, imp + 0x150), [1.0, 0.5, 2.0, 0.0]);
        // Per unit volume: I_xx = (b^2 + c^2) / 12 with b = 1, c = 4 and so on.
        let diagonal = get_vec(&e, imp + 0x160);
        assert_vec_close(diagonal, [17.0 / 12.0, 20.0 / 12.0, 5.0 / 12.0, 0.0]);
    }

    #[test]
    fn mass_properties_of_a_polygon_use_its_area_and_centre() {
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let square = [
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [0.0, 2.0, 0.0],
        ];
        let at = aligned_points(&mut e, &square);
        e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 2);
        e.call(0x00d7_b600, &args![imp]);
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        assert_close(e.mem.f32(imp + 0x1a4), 4.0);
        assert_eq!(e.mem.f32(imp + 0x1a0), 0.0);
        let centre = get_vec(&e, imp + 0x150);
        assert_vec_close(centre, [1.0, 1.0, 0.0, 0.0]);
        assert_eq!(get_vec(&e, imp + 0x160), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(imp + 0x170), 0x3f80_0000);
    }

    #[test]
    fn mass_properties_of_a_point_or_segment_are_empty() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &[[1.0, 2.0, 3.0]]);
        // The rows the exe copies out of its data: the matrix rows and the translation.
        for row in 0..4u32 {
            put_vec(&mut e, 0x0126_8370 + 0x10 * row, [row as f32 + 1.0; 4]);
        }
        e.call(0x00d7_b600, &args![imp]);
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        assert_eq!(e.mem.f32(imp + 0x1a0), 0.0);
        assert_eq!(e.mem.f32(imp + 0x1a4), 0.0);
        assert_eq!(get_vec(&e, imp + 0x170), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(get_vec(&e, imp + 0x190), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(get_vec(&e, imp + 0x160), [0.0; 4]);
        for row in 0..4u32 {
            assert_eq!(get_vec(&e, imp + 0x120 + 0x10 * row), [row as f32 + 1.0; 4]);
        }
    }

    #[test]
    fn mass_properties_of_a_flat_solid_reset_the_rotation() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        // Shrinking the grid scale squeezes the hull to a point: its volume is below the epsilon.
        put_vec(&mut e, imp + 0x110, [1e-9, 1e-9, 1e-9, 0.0]);
        e.call_log = Some(Vec::new());
        e.call(0x00d7_b600, &args![imp]);
        let log = e.call_log.take().unwrap();
        let sets: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(address, _)| *address == ROTATION_SET)
            .map(|(_, words)| words)
            .collect();
        assert_eq!(*sets, vec![&vec![imp + 0x120, DEGENERATE_ROTATION]]);
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        assert_eq!(e.mem.f32(imp + 0x1a0), 0.0);
        for offset in [0x160, 0x170, 0x180, 0x190] {
            assert_eq!(get_vec(&e, imp + offset), [0.0; 4]);
        }
    }

    #[test]
    fn mass_properties_report_a_failed_diagonalization() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        e.register(MATRIX3_DIAGONALIZE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let log = install_report(&mut e, 0);
        e.mem.set_u32(imp + 0x1b0, 0x31);
        e.call(0x00d7_b600, &args![imp]);
        assert_report(&log, MESSAGE_DIAGONALIZE, 0x31, 0x45465, 0x6d9);
        // A break request makes it the INT3 at 00d7bdc7.
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        e.register(MATRIX3_DIAGONALIZE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        install_report(&mut e, 1);
        let message = int3_message(&mut e, 0x00d7_b600, &args![imp]);
        assert!(message.contains("INT3 at 00d7bdc7"), "{message}");
    }

    #[test]
    fn hull_build_mass_properties_runs_on_the_implementation() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &CUBE);
        let failed = e.call(0x00d7_62f0, &args![hull]).bool();
        assert!(!failed);
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        assert_close(e.mem.f32(imp + 0x1a0), 1.0);
    }

    /// A support mapping over the points it is built from: the build (`00dedcf0`, `(array,
    /// structure)`) keeps the vectors of the `hkArray` and marks the structure as built (its
    /// second word), the query (`00dea810`, `(structure, direction, out)`) answers the first
    /// point with the greatest dot product with the direction.
    fn install_support_points(e: &mut Engine) {
        type Points = Rc<RefCell<Vec<[f32; 4]>>>;
        let points: Points = Rc::new(RefCell::new(Vec::new()));
        let built = Rc::clone(&points);
        e.register_double(SUPPORT_BUILD, move |e, a| {
            let size = e.mem.u32(a[0] + 4);
            let data = e.mem.u32(a[0]);
            *built.borrow_mut() = (0..size)
                .map(|index| get_vec(e, data + 16 * index))
                .collect();
            e.mem.set_u32(a[1] + 4, 1);
            Ret::default()
        });
        e.register_double(SUPPORT_QUERY, move |e, a| {
            let direction = get_vec(e, a[1]);
            let dot =
                |p: &[f32; 4]| p[0] * direction[0] + p[1] * direction[1] + p[2] * direction[2];
            let mut best = points.borrow()[0];
            for point in points.borrow().iter() {
                if dot(point) > dot(&best) {
                    best = *point;
                }
            }
            put_vec(e, a[2], best);
            Ret::default()
        });
    }

    #[test]
    fn fn_00d7c610_is_the_width_of_the_hull_along_a_direction() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let direction = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [1.0, 1.0, 0.0, 0.0]);
        let (low, high) = (e.mem.alloc(8), e.mem.alloc(8));
        let width = e.call(0x00d7_c610, &args![imp, direction, low, high]).f32();
        assert_close(width, 2.0);
        assert_close(e.mem.f32(high), 2.0);
        assert_close(e.mem.f32(low), 0.0);
        // The support structure was built once and is kept.
        assert_eq!(e.mem.u32(imp + 0x64), 1);
        put_vec(&mut e, direction, [0.0, 0.0, 2.0, 0.0]);
        let width = e.call(0x00d7_c610, &args![imp, direction, low, high]).f32();
        assert_close(width, 2.0);
        assert_close(e.mem.f32(high), 2.0);
    }

    /// The vectors of a frame (four vectors) and sizes written by an oriented box search.
    fn assert_unit_cube_box(e: &Engine, sizes: u32, frame: u32) {
        let size = get_vec(e, sizes);
        assert_vec_close([size[0], size[1], size[2], 0.0], [0.5, 0.5, 0.5, 0.0]);
        let axes: Vec<[f32; 4]> = (0..3)
            .map(|index| get_vec(e, frame + 0x10 * index))
            .collect();
        // Each axis is one of the coordinate axes, no two the same.
        let mut used = [false; 3];
        for axis in &axes {
            let direction = (0..3)
                .find(|lane| (axis[*lane].abs() - 1.0).abs() < 1e-4)
                .unwrap();
            assert!(!used[direction], "{axes:?}");
            used[direction] = true;
        }
        assert_vec_close(get_vec(e, frame + 0x30), [0.5, 0.5, 0.5, 0.0]);
    }

    #[test]
    fn oriented_rectangle_of_a_cube_is_the_cube() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let sizes = e.mem.alloc(0x10);
        let frame = e.mem.alloc(0x40);
        e.call(0x00d8_1340, &args![imp, sizes, frame]);
        assert_unit_cube_box(&e, sizes, frame);
    }

    #[test]
    fn oriented_bounding_box_forwards_to_the_rectangle() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let sizes = e.mem.alloc(0x10);
        let frame = e.mem.alloc(0x40);
        e.call(0x00d7_6d80, &args![hull, sizes, frame]);
        assert_unit_cube_box(&e, sizes, frame);
    }

    #[test]
    fn oriented_rectangle_of_a_polygon_searches_along_the_normal() {
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let rectangle = [
            [0.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 2.0, 0.0],
        ];
        let at = aligned_points(&mut e, &rectangle);
        e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 2);
        let sizes = e.mem.alloc(0x10);
        let frame = e.mem.alloc(0x40);
        e.call(0x00d8_1340, &args![imp, sizes, frame]);
        let size = get_vec(&e, sizes);
        assert_vec_close([size[0], size[1], size[2], size[3]], [2.0, 1.0, 0.0, 0.0]);
        // The third frame vector is the normal of the polygon, the origin its middle.
        let normal = get_vec(&e, frame + 0x20);
        assert_close(normal[2].abs(), 1.0);
        let origin = get_vec(&e, frame + 0x30);
        assert_vec_close([origin[0], origin[1], origin[2], 0.0], [2.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn oriented_rectangle_assertion_is_the_int3_of_each_search() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        e.mem.set_u8(imp + 0x1b5, 0);
        install_report(&mut e, 1);
        let sizes = e.mem.alloc(0x10);
        let frame = e.mem.alloc(0x40);
        let message = int3_message(&mut e, 0x00d8_1340, &args![imp, sizes, frame]);
        assert!(message.contains("INT3 at 00d813ec"), "{message}");
        // With fewer than three dimensions the search runs on its own and breaks at 00d7f112.
        e.mem.set_i32(imp + 0x1a8, 2);
        let message = int3_message(&mut e, 0x00d8_1340, &args![imp, sizes, frame]);
        assert!(message.contains("INT3 at 00d7f112"), "{message}");
    }

    #[test]
    fn fn_00d7f090_finds_the_smallest_rectangle_along_a_direction() {
        let (mut e, _) = bench();
        let slab: Vec<[f32; 3]> = CUBE
            .iter()
            .map(|p| [p[0] * 4.0, p[1], p[2] * 2.0])
            .collect();
        let (_, imp) = built_hull(&mut e, &slab);
        let direction = e.mem.alloc(0x10);
        // Looking down the y axis: a 4 by 2 rectangle.
        put_vec(&mut e, direction, [0.0, 1.0, 0.0, 0.0]);
        let sizes = e.mem.alloc(0x10);
        let frame = e.mem.alloc(0x40);
        let area = e
            .call(0x00d7_f090, &args![imp, direction, sizes, frame])
            .f32();
        assert_close(area, 8.0);
        let size = get_vec(&e, sizes);
        let mut halves = [size[0], size[1]];
        halves.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_vec_close(
            [halves[0], halves[1], size[2], size[3]],
            [1.0, 2.0, 0.0, 0.0],
        );
        assert_eq!(get_vec(&e, frame + 0x20), [0.0, 1.0, 0.0, 0.0]);
        let origin = get_vec(&e, frame + 0x30);
        assert_vec_close([origin[0], origin[1], origin[2], 0.0], [2.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn random_vector_follows_the_congruential_generator() {
        let (mut e, _) = bench();
        let generator = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        e.call(0x00d7_a4b0, &args![generator, out]);
        let mut state = 0u32;
        let mut want = [0.0f32; 4];
        for lane in want.iter_mut() {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            *lane = (f64::from(state) / 4_294_967_296.0 * 2.0 - 1.0) as f32;
        }
        assert_eq!(e.mem.u32(generator + 4), state);
        // The first draw is the golden-ratio constant 0x3c6ef35f.
        assert_close(want[0], -0.527_864);
        let got = get_vec(&e, out);
        for lane in 0..4 {
            assert!((got[lane] - want[lane]).abs() < 1e-6, "{got:?} {want:?}");
            assert!((-1.0..=1.0).contains(&got[lane]));
        }
    }

    #[test]
    fn random_samples_lie_inside_the_hull() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let generator = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 8, 0x8000_0000);
        e.call(0x00d7_6fa0, &args![hull, generator, 6i32, array]);
        assert_eq!(e.mem.i32(array + 4), 6);
        let data = e.mem.u32(array);
        for sample in 0..6u32 {
            let p = get_vec(&e, data + 16 * sample);
            for lane in 0..3 {
                assert!(p[lane] > -1e-3 && p[lane] < 1.001, "{p:?}");
            }
        }
        // A count of zero (or less) adds nothing.
        e.call(0x00d7_6fa0, &args![hull, generator, 0i32, array]);
        assert_eq!(e.mem.i32(array + 4), 6);
    }

    #[test]
    fn random_samples_give_up_with_a_report_when_nothing_fits() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &CUBE);
        // No plane accepts anything: every tolerance test fails.
        for plane in 0..6u32 {
            let at = e.mem.u32(imp + 0x54) + 16 * plane;
            put_vec(&mut e, at, [0.0, 0.0, 0.0, 1.0]);
        }
        let log = install_report(&mut e, 0);
        let generator = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 8, 0x8000_0000);
        e.call(0x00d7_6fa0, &args![hull, generator, 2i32, array]);
        assert_eq!(e.mem.i32(array + 4), 0);
        let log = log.borrow();
        assert_eq!(log[1].1[1], MESSAGE_SAMPLES_FAILED);
        assert_eq!(log[2].1[1..], [1, 0x1559e, log[0].1[1], REPORT_FILE, 0xe22]);
    }

    /// Sixteen bytes holding the configuration words `(retry, tolerance, ids, indices, mass, sort,
    /// unit grid)` the hull wrappers copy over the implementation.
    fn config_words(e: &mut Engine, mass: u8, indices: u8) -> u32 {
        let at = e.mem.alloc(0x10);
        e.mem.set_f32(at + 4, 0.25);
        e.mem.set_u8(at + 9, indices);
        e.mem.set_u8(at + 0xa, mass);
        e.mem.set_u8(at + 0xb, 1);
        at
    }

    #[test]
    fn hull_build_applies_the_configuration_words_then_builds() {
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        let config = config_words(&mut e, 1, 0);
        let points = aligned_points(&mut e, &CUBE);
        let dims = e
            .call(0x00d7_71c0, &args![hull, points, 8i32, config])
            .i32();
        assert_eq!(dims, 3);
        assert_eq!(e.mem.f32(imp + 4), 0.25);
        // Mass properties were asked for, indices not.
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        assert_eq!(e.mem.u8(imp + 0x1b5), 0);
        assert_close(e.mem.f32(imp + 0x1a0), 1.0);
    }

    /// A strided array `{data, count, stride}` over the points, three floats each.
    fn strided_points(e: &mut Engine, points: &[[f32; 3]], stride: u32) -> u32 {
        let data = e.mem.alloc(stride * points.len() as u32 + 16);
        for (index, p) in points.iter().enumerate() {
            let at = data + stride * index as u32;
            for (lane, value) in p.iter().enumerate() {
                e.mem.set_f32(at + 4 * lane as u32, *value);
            }
        }
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, data);
        e.mem.set_i32(source + 4, points.len() as i32);
        e.mem.set_u32(source + 8, stride);
        source
    }

    #[test]
    fn hull_build_of_a_strided_array_copies_unpacked_points() {
        // Stride 16 builds from the data as it is (it must be aligned).
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        let config = config_words(&mut e, 0, 1);
        let aligned = aligned_points(&mut e, &CUBE);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, aligned);
        e.mem.set_i32(source + 4, 8);
        e.mem.set_u32(source + 8, 0x10);
        let dims = e.call(0x00d7_7200, &args![hull, source, config]).i32();
        assert_eq!(dims, 3);
        assert_eq!(vertex_positions(&e, imp), sorted(&CUBE));
        // Stride 12: the points are copied into an array of vectors that is released after.
        let (mut e, frees) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        let config = config_words(&mut e, 0, 1);
        let source = strided_points(&mut e, &CUBE, 12);
        let dims = e.call(0x00d7_7200, &args![hull, source, config]).i32();
        assert_eq!(dims, 3);
        assert_eq!(vertex_positions(&e, imp), sorted(&CUBE));
        assert!(frees.borrow().iter().any(|(_, a)| a[2] == 8 * 16));
    }

    #[test]
    fn fn_00d77320_builds_in_the_given_plane() {
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        let config = config_words(&mut e, 0, 1);
        let normal = e.mem.alloc(0x10);
        put_vec(&mut e, normal, [0.0, 0.0, 1.0, 0.0]);
        let points = aligned_points(&mut e, &CUBE[..4]);
        let dims = e
            .call(0x00d7_7320, &args![hull, points, 4i32, normal, config])
            .i32();
        assert_eq!(dims, 2);
        assert_eq!(get_vec(&e, imp + 0xe0), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.mem.f32(imp + 4), 0.25);
        assert_eq!(e.mem.i32(imp + 0x28), 4);
    }

    #[test]
    fn fn_00d77380_builds_a_strided_array_in_the_given_plane() {
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        let config = config_words(&mut e, 0, 1);
        let normal = e.mem.alloc(0x10);
        put_vec(&mut e, normal, [0.0, 0.0, 1.0, 0.0]);
        let source = strided_points(&mut e, &CUBE[..4], 12);
        let dims = e
            .call(0x00d7_7380, &args![hull, source, normal, config])
            .i32();
        assert_eq!(dims, 2);
        assert_eq!(vertex_positions(&e, imp), sorted(&CUBE[..4]));
        // A stride of 16 goes straight to the plane build.
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        let config = config_words(&mut e, 0, 1);
        let normal = e.mem.alloc(0x10);
        put_vec(&mut e, normal, [0.0, 0.0, 1.0, 0.0]);
        let aligned = aligned_points(&mut e, &CUBE[..4]);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, aligned);
        e.mem.set_i32(source + 4, 4);
        e.mem.set_u32(source + 8, 0x10);
        let dims = e
            .call(0x00d7_7380, &args![hull, source, normal, config])
            .i32();
        assert_eq!(dims, 2);
        assert_eq!(e.mem.i32(imp + 0x28), 4);
    }

    /// Plane equations `n . p + w <= 0` of the box `[0, 1]^3` (the first four are the sides of
    /// the unit square, the others the ends).
    const BOX_PLANES: [[f32; 4]; 6] = [
        [-1.0, 0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, -1.0],
        [0.0, -1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, -1.0],
        [0.0, 0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0, -1.0],
    ];

    fn plane_block(e: &mut Engine, planes: &[[f32; 4]]) -> u32 {
        let at = e.mem.alloc(16 * planes.len() as u32 + 16);
        for (index, plane) in planes.iter().enumerate() {
            put_vec(e, at + 16 * index as u32, *plane);
        }
        at
    }

    #[test]
    fn build_from_planes_intersects_every_triple() {
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        let config = config_words(&mut e, 0, 1);
        let planes = plane_block(&mut e, &BOX_PLANES);
        let dims = e
            .call(0x00d7_74a0, &args![hull, planes, 6i32, config])
            .i32();
        assert_eq!(dims, 3);
        assert_eq!(vertex_positions(&e, imp).len(), 8);
        for corner in CUBE {
            assert!(vertex_positions(&e, imp)
                .iter()
                .any(|p| (0..3).all(|lane| (p[lane] - corner[lane]).abs() < 1e-4)));
        }
        // The timed section of the enumeration was opened and closed.
        assert_eq!(
            profile_labels(&e),
            vec![PROFILE_VERTEX_ENUMERATION, PROFILE_END]
        );
    }

    #[test]
    fn build_from_planes_with_too_few_vertices_fails() {
        let (mut e, _) = bench();
        let (hull, _) = fresh_hull(&mut e);
        let config = config_words(&mut e, 0, 1);
        // Three planes meet in one point.
        let planes = plane_block(&mut e, &[BOX_PLANES[0], BOX_PLANES[2], BOX_PLANES[4]]);
        assert_eq!(
            e.call(0x00d7_74a0, &args![hull, planes, 3i32, config])
                .i32(),
            -1
        );
        // Parallel planes make no vertices at all.
        let planes = plane_block(&mut e, &BOX_PLANES[..2]);
        assert_eq!(
            e.call(0x00d7_74a0, &args![hull, planes, 2i32, config])
                .i32(),
            -1
        );
    }

    #[test]
    fn build_from_planes_in_a_plane_makes_a_polygon() {
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        let config = config_words(&mut e, 0, 1);
        let planes = plane_block(&mut e, &BOX_PLANES[..4]);
        let third = e.mem.alloc(0x10);
        put_vec(&mut e, third, [0.0, 0.0, 1.0, 0.0]);
        let dims = e
            .call(0x00d7_7840, &args![hull, planes, 4i32, third, config])
            .i32();
        assert_eq!(dims, 2);
        assert_eq!(
            vertex_positions(&e, imp),
            vec![
                [0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 1.0, 0.0]
            ]
        );
        assert_eq!(get_vec(&e, imp + 0xe0), [0.0, 0.0, 1.0, 0.0]);
        // Two planes only meet the third in one point: too few.
        let two = plane_block(&mut e, &BOX_PLANES[..2]);
        assert_eq!(
            e.call(0x00d7_7840, &args![hull, two, 2i32, third, config])
                .i32(),
            -1
        );
    }

    #[test]
    fn contains_tests_the_planes_exactly_or_the_grid_triangles() {
        // Exact: every plane of the unit box (indices valid after the build).
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let inside = e.mem.alloc(0x10);
        put_vec(&mut e, inside, [0.5, 0.5, 0.5, 1.0]);
        assert!(e
            .call(0x00d7_6100, &args![hull, inside, 1u32, 0.0f32.to_bits()])
            .bool());
        let outside = e.mem.alloc(0x10);
        put_vec(&mut e, outside, [1.5, 0.5, 0.5, 1.0]);
        assert!(!e
            .call(0x00d7_6100, &args![hull, outside, 1u32, 0.0f32.to_bits()])
            .bool());
        // A tolerance of one lets it in.
        assert!(e
            .call(0x00d7_6100, &args![hull, outside, 1u32, 1.0f32.to_bits()])
            .bool());
        // Not exact: the point is quantised and tested against the integer planes of the triangles.
        assert!(e
            .call(0x00d7_6100, &args![hull, inside, 0u32, 0.0f32.to_bits()])
            .bool());
        assert!(!e
            .call(0x00d7_6100, &args![hull, outside, 0u32, 0.0f32.to_bits()])
            .bool());
        // Far outside the grid range it is rejected before the planes.
        let far = e.mem.alloc(0x10);
        put_vec(&mut e, far, [1.0e6, 0.5, 0.5, 1.0]);
        assert!(!e
            .call(0x00d7_6100, &args![hull, far, 0u32, 0.0f32.to_bits()])
            .bool());
    }

    #[test]
    fn contains_breaks_without_indices() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        e.mem.set_u8(imp + 0x1b5, 0);
        install_report(&mut e, 1);
        let point = e.mem.alloc(0x10);
        let message = int3_message(
            &mut e,
            0x00d7_b330,
            &args![imp, point, 1u32, 0.0f32.to_bits()],
        );
        assert!(message.contains("INT3 at 00d7b57d"), "{message}");
    }

    /// The vertex positions of a hull object, sorted.
    fn hull_positions(e: &Engine, hull: u32) -> Vec<[f32; 3]> {
        vertex_positions(e, e.mem.u32(hull + 8))
    }

    /// The box `(low, high)` of the vertex positions of a hull object.
    fn hull_box(e: &Engine, hull: u32) -> ([f32; 3], [f32; 3]) {
        let positions = hull_positions(e, hull);
        let mut low = [f32::MAX; 3];
        let mut high = [f32::MIN; 3];
        for p in positions {
            for lane in 0..3 {
                low[lane] = low[lane].min(p[lane]);
                high[lane] = high[lane].max(p[lane]);
            }
        }
        (low, high)
    }

    fn assert_box(e: &Engine, hull: u32, low: [f32; 3], high: [f32; 3]) {
        let (got_low, got_high) = hull_box(e, hull);
        for lane in 0..3 {
            assert!(
                (got_low[lane] - low[lane]).abs() < 2e-3,
                "{got_low:?} {low:?}"
            );
            assert!(
                (got_high[lane] - high[lane]).abs() < 2e-3,
                "{got_high:?} {high:?}"
            );
        }
    }

    /// A built box hull `[low, high]` (a hull object and its implementation).
    fn box_hull(e: &mut Engine, low: [f32; 3], high: [f32; 3]) -> (u32, u32) {
        let corners: Vec<[f32; 3]> = CUBE
            .iter()
            .map(|c| {
                [
                    low[0] + c[0] * (high[0] - low[0]),
                    low[1] + c[1] * (high[1] - low[1]),
                    low[2] + c[2] * (high[2] - low[2]),
                ]
            })
            .collect();
        built_hull(e, &corners)
    }

    fn plane_at(e: &mut Engine, plane: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        put_vec(e, at, plane);
        at
    }

    #[test]
    fn split_of_a_cube_makes_two_halves() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let plane = plane_at(&mut e, [1.0, 0.0, 0.0, -0.5]);
        let outs = e.mem.alloc(8);
        let answer = e
            .call(
                0x00d8_3430,
                &args![imp, plane, outs, outs + 4, 1e-4f32.to_bits()],
            )
            .u32();
        assert_eq!(answer, 3);
        let (positive, negative) = (e.mem.u32(outs), e.mem.u32(outs + 4));
        assert_box(&e, positive, [0.5, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert_box(&e, negative, [0.0, 0.0, 0.0], [0.5, 1.0, 1.0]);
        for hull in [positive, negative] {
            assert_eq!(e.mem.i32(e.mem.u32(hull + 8) + 0x1a8), 3);
            assert_eq!(e.mem.i32(e.mem.u32(hull + 8) + 0x28), 8);
        }
        // Either side may be left out.
        let negative_only = e.mem.alloc(4);
        let answer = e
            .call(
                0x00d8_3430,
                &args![imp, plane, 0u32, negative_only, 1e-4f32.to_bits()],
            )
            .u32();
        assert_eq!(answer, 3);
        assert_box(
            &e,
            e.mem.u32(negative_only),
            [0.0, 0.0, 0.0],
            [0.5, 1.0, 1.0],
        );
    }

    #[test]
    fn split_of_a_cube_by_a_plane_that_misses_it_or_touches_it() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let outs = e.mem.alloc(8);
        let tolerance = 1e-4f32.to_bits();
        // Everything behind the plane: 2. Everything in front of it: 1.
        let behind = plane_at(&mut e, [1.0, 0.0, 0.0, -5.0]);
        assert_eq!(
            e.call(0x00d8_3430, &args![imp, behind, outs, outs + 4, tolerance])
                .u32(),
            2
        );
        assert_eq!((e.mem.u32(outs), e.mem.u32(outs + 4)), (0, 0));
        let front = plane_at(&mut e, [1.0, 0.0, 0.0, 5.0]);
        assert_eq!(
            e.call(0x00d8_3430, &args![imp, front, outs, outs + 4, tolerance])
                .u32(),
            1
        );
        // A plane along a face: the face vertices count as neither side.
        let face = plane_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            e.call(0x00d8_3430, &args![imp, face, outs, outs + 4, tolerance])
                .u32(),
            1
        );
        let face = plane_at(&mut e, [1.0, 0.0, 0.0, -1.0]);
        assert_eq!(
            e.call(0x00d8_3430, &args![imp, face, outs, outs + 4, tolerance])
                .u32(),
            2
        );
    }

    #[test]
    fn split_of_a_polygon_cuts_the_edges() {
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let rectangle = [
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [0.0, 2.0, 0.0],
        ];
        let at = aligned_points(&mut e, &rectangle);
        e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 2);
        let plane = plane_at(&mut e, [1.0, 0.0, 0.0, -1.0]);
        let outs = e.mem.alloc(8);
        let answer = e
            .call(
                0x00d8_3430,
                &args![imp, plane, outs, outs + 4, 1e-4f32.to_bits()],
            )
            .u32();
        assert_eq!(answer, 3);
        let (first, second) = (e.mem.u32(outs), e.mem.u32(outs + 4));
        // The non-negative side is built into the first hull, the non-positive into the second.
        assert_box(&e, first, [1.0, 0.0, 0.0], [2.0, 2.0, 0.0]);
        assert_box(&e, second, [0.0, 0.0, 0.0], [1.0, 2.0, 0.0]);
        assert_eq!(e.mem.i32(e.mem.u32(first + 8) + 0x28), 4);
        // Nothing on one side: 2 for all behind, 1 for all in front.
        let behind = plane_at(&mut e, [1.0, 0.0, 0.0, -5.0]);
        assert_eq!(
            e.call(
                0x00d8_3430,
                &args![imp, behind, outs, outs + 4, 1e-4f32.to_bits()]
            )
            .u32(),
            2
        );
        let front = plane_at(&mut e, [1.0, 0.0, 0.0, 5.0]);
        assert_eq!(
            e.call(
                0x00d8_3430,
                &args![imp, front, outs, outs + 4, 1e-4f32.to_bits()]
            )
            .u32(),
            1
        );
    }

    #[test]
    fn split_of_other_dimensions_is_reported() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &[[1.0, 2.0, 3.0]]);
        let log = install_report(&mut e, 0);
        let plane = plane_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let outs = e.mem.alloc(8);
        e.mem.set_u32(outs, 0x1234);
        let answer = e
            .call(
                0x00d8_3430,
                &args![imp, plane, outs, outs + 4, 1e-4f32.to_bits()],
            )
            .u32();
        assert_eq!(answer, 4);
        assert_eq!(e.mem.u32(outs), 0);
        assert_eq!(log.borrow()[1].1[1], MESSAGE_SPLIT_UNSUPPORTED);
        assert_eq!(log.borrow()[2].1[1..3], [3, 0x3a86_e3ef]);
        assert_eq!(log.borrow()[2].1[4..], [REPORT_FILE, 0x854]);
        install_report(&mut e, 1);
        let message = int3_message(
            &mut e,
            0x00d8_3430,
            &args![imp, plane, outs, outs + 4, 0u32],
        );
        assert!(message.contains("INT3 at 00d834b7"), "{message}");
    }

    #[test]
    fn fn_00d790a0_splits_the_implementation_of_the_hull() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let plane = plane_at(&mut e, [0.0, 1.0, 0.0, -0.25]);
        let outs = e.mem.alloc(8);
        let answer = e
            .call(
                0x00d7_90a0,
                &args![hull, plane, outs, outs + 4, 1e-4f32.to_bits()],
            )
            .u32();
        assert_eq!(answer, 3);
        assert_box(&e, e.mem.u32(outs), [0.0, 0.25, 0.0], [1.0, 1.0, 1.0]);
        assert_box(&e, e.mem.u32(outs + 4), [0.0, 0.0, 0.0], [1.0, 0.25, 1.0]);
    }

    #[test]
    fn splitting_into_pieces_halves_each_piece_per_round() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 8, 0x8000_0000);
        e.call(0x00d7_90d0, &args![hull, 1i32, array, 1e-4f32.to_bits()]);
        assert_eq!(e.mem.i32(array + 4), 2);
        let pieces: Vec<u32> = (0..2)
            .map(|i| e.mem.u32(e.mem.u32(array) + 4 * i))
            .collect();
        // Two halves of the cube, each of half the volume.
        let mut volumes = Vec::new();
        for piece in &pieces {
            e.call(0x00d7_62f0, &args![*piece]);
            volumes.push(e.mem.f32(e.mem.u32(piece + 8) + 0x1a0));
        }
        assert_close(volumes[0] + volumes[1], 1.0);
        assert_close(volumes[0], 0.5);
        // A second round halves both: four pieces.
        e.call(0x00d7_90d0, &args![hull, 2i32, array, 1e-4f32.to_bits()]);
        assert_eq!(e.mem.i32(array + 4), 4);
        // No rounds: only the clone of the hull.
        e.call(0x00d7_90d0, &args![hull, 0i32, array, 1e-4f32.to_bits()]);
        assert_eq!(e.mem.i32(array + 4), 1);
    }

    /// The pieces in an `hkArray` of hull pointers.
    fn array_hulls(e: &Engine, array: u32) -> Vec<u32> {
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(e.mem.u32(array) + 4 * index))
            .collect()
    }

    /// An empty non-owning `hkArray` header.
    fn empty_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 8, 0x8000_0000);
        array
    }

    #[test]
    fn cutting_a_hull_by_the_planes_of_another_keeps_the_inside_apart() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let (cutter, _) = box_hull(&mut e, [0.25; 3], [0.75; 3]);
        let slot = e.mem.alloc(4);
        let array = empty_array(&mut e);
        e.mem.set_u32(slot, 0x77);
        e.call(
            0x00d7_9250,
            &args![hull, cutter, slot, array, 1e-4f32.to_bits()],
        );
        // The inside is the box of the cutter, the six slabs around it are the pieces.
        assert_box(&e, e.mem.u32(slot), [0.25; 3], [0.75; 3]);
        let outside = array_hulls(&e, array);
        assert_eq!(outside.len(), 6);
        for piece in &outside {
            let (low, high) = hull_box(&e, *piece);
            assert!(low.iter().all(|v| *v > -1e-3) && high.iter().all(|v| *v < 1.001));
        }
        // Without an array or a slot the pieces are just deleted.
        e.call(
            0x00d7_9250,
            &args![hull, cutter, 0u32, 0u32, 1e-4f32.to_bits()],
        );
        // A cutter that does not touch the hull leaves the whole piece in the array.
        let (far, _) = box_hull(&mut e, [5.0; 3], [6.0; 3]);
        e.call(
            0x00d7_9250,
            &args![hull, far, slot, array, 1e-4f32.to_bits()],
        );
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(array_hulls(&e, array).len(), 1);
        assert_box(&e, array_hulls(&e, array)[0], [0.0; 3], [1.0; 3]);
    }

    #[test]
    fn cut_answers_whether_any_piece_lies_inside_the_cutter() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let (cutter, _) = box_hull(&mut e, [0.25; 3], [0.75; 3]);
        let array = empty_array(&mut e);
        let kept = e
            .call(0x00d7_9510, &args![hull, cutter, array, 1e-4f32.to_bits()])
            .bool();
        assert!(kept);
        assert_eq!(array_hulls(&e, array).len(), 6);
        // No overlap: nothing is kept, and the pieces the cut appended are dropped again.
        let (far, _) = box_hull(&mut e, [5.0; 3], [6.0; 3]);
        let kept = e
            .call(0x00d7_9510, &args![hull, far, array, 1e-4f32.to_bits()])
            .bool();
        assert!(!kept);
        assert_eq!(e.mem.i32(array + 4), 0);
    }

    #[test]
    fn cutting_by_several_cutters_leaves_what_is_outside_all_of_them() {
        let (mut e, _) = bench();
        let (hull, _) = box_hull(&mut e, [0.0; 3], [3.0, 1.0, 1.0]);
        // Two boxes that take the ends of the hull; what is left is the middle third.
        let (first, _) = box_hull(&mut e, [-1.0, -1.0, -1.0], [1.0, 2.0, 2.0]);
        let (second, _) = box_hull(&mut e, [2.0, -1.0, -1.0], [4.0, 2.0, 2.0]);
        let cutters = e.mem.alloc(0x10);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, first);
        e.mem.set_u32(data + 4, second);
        e.mem.set_u32(cutters, data);
        e.mem.set_u32(cutters + 4, 2);
        e.mem.set_u32(cutters + 8, 2);
        let out = empty_array(&mut e);
        let any = e
            .call(0x00d7_9580, &args![hull, cutters, out, 1e-4f32.to_bits()])
            .bool();
        assert!(any);
        let pieces = array_hulls(&e, out);
        assert_eq!(pieces.len(), 1);
        assert_box(&e, pieces[0], [1.0, 0.0, 0.0], [2.0, 1.0, 1.0]);
        // A cutter that swallows the hull leaves nothing.
        let (big, _) = box_hull(&mut e, [-5.0; 3], [5.0; 3]);
        e.mem.set_u32(data, big);
        e.mem.set_u32(cutters + 4, 1);
        let out = empty_array(&mut e);
        assert!(!e
            .call(0x00d7_9580, &args![hull, cutters, out, 1e-4f32.to_bits()])
            .bool());
        assert_eq!(e.mem.i32(out + 4), 0);
    }

    const CHECK_SLOT_DOUBLE: u32 = 0x0900_0040;

    /// An object whose slot 4 logs `(flag, pair, count)` words and answers `verdict` through the
    /// flag byte it is given, answering the flag's address.
    fn check_object(e: &mut Engine, verdict: u8) -> (u32, Log) {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let calls = Rc::clone(&log);
        e.register_double(CHECK_SLOT_DOUBLE, move |e, a| {
            let pair = [e.mem.u32(a[2]), e.mem.u32(a[2] + 4)];
            calls
                .borrow_mut()
                .push((CHECK_SLOT_DOUBLE, vec![a[3], pair[0], pair[1]]));
            e.mem.set_u8(a[1], verdict);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, CHECK_SLOT_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        (object, log)
    }

    #[test]
    fn fn_00d79850_finds_the_split_with_the_largest_inner_part() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let (other, _) = box_hull(&mut e, [0.5, 0.0, 0.0], [1.5, 1.0, 1.0]);
        let (check, log) = check_object(&mut e, 1);
        let (merged, part) = (e.mem.alloc(4), e.mem.alloc(4));
        let best = e
            .call(0x00d7_9850, &args![hull, other, check, merged, part])
            .f32();
        // The part of the other box behind the hull's x = 1 plane is 0.5 of volume.
        assert_close(best, 0.5);
        assert_box(&e, e.mem.u32(part), [1.0, 0.0, 0.0], [1.5, 1.0, 1.0]);
        assert_box(&e, e.mem.u32(merged), [0.0; 3], [1.0; 3]);
        // The check object was asked about (merged hull, outer part) for two hulls.
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(
            log.borrow()[0].1,
            vec![2, e.mem.u32(merged), e.mem.u32(part)]
        );
    }

    #[test]
    fn fn_00d79850_without_a_counting_split_answers_minus_one() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let (other, _) = box_hull(&mut e, [0.5, 0.0, 0.0], [1.5, 1.0, 1.0]);
        // The check object refuses every candidate.
        let (check, log) = check_object(&mut e, 0);
        let (merged, part) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(merged, 0x99);
        e.mem.set_u32(part, 0x99);
        let best = e
            .call(0x00d7_9850, &args![hull, other, check, merged, part])
            .f32();
        assert_eq!(best, -1.0);
        assert_eq!((e.mem.u32(merged), e.mem.u32(part)), (0, 0));
        assert_eq!(log.borrow().len(), 1);
        // A hull without indices breaks at 00d798ec.
        let (hull, imp) = built_hull(&mut e, &CUBE);
        e.mem.set_u8(imp + 0x1b5, 0);
        install_report(&mut e, 1);
        let message = int3_message(
            &mut e,
            0x00d7_9850,
            &args![hull, other, check, merged, part],
        );
        assert!(message.contains("INT3 at 00d798ec"), "{message}");
    }

    /// Pseudo-random keys with duplicates and negative values, from a fixed congruential
    /// sequence.
    fn sort_keys(count: usize, seed: u32) -> Vec<i32> {
        let mut state = seed;
        (0..count)
            .map(|_| {
                state = state.wrapping_mul(1_103_515_245).wrapping_add(12345);
                ((state >> 16) % 17) as i32 - 8
            })
            .collect()
    }

    /// The orders the sorts are run on: random, sorted, reversed, equal and tiny.
    fn sort_cases() -> Vec<Vec<i32>> {
        let mut cases = vec![
            sort_keys(40, 1),
            sort_keys(33, 7),
            vec![5],
            vec![2, 1],
            vec![3, 3, 3, 3],
        ];
        let mut ascending = sort_keys(20, 3);
        ascending.sort_unstable();
        cases.push(ascending.clone());
        ascending.reverse();
        cases.push(ascending);
        cases
    }

    #[test]
    fn grid_item_sort_orders_by_the_three_coordinates() {
        for keys in sort_cases() {
            let (mut e, _) = bench();
            let data = e.mem.alloc(0x40 * keys.len() as u32);
            // The coordinates come from the key, the second from its reverse, and the tag word
            // tells the records apart.
            for (index, key) in keys.iter().enumerate() {
                let at = data + 0x40 * index as u32;
                e.mem.set_i32(at + 0x20, key / 3);
                e.mem.set_i32(at + 0x24, key % 3);
                e.mem.set_i32(at + 0x28, index as i32);
                e.mem.set_u32(at + 0x10, index as u32);
                e.mem.set_u32(at + 0x34, 0x1000 + index as u32);
                e.mem.set_u32(at + 0x2c, x87_quiet_bits(0x7f80_0001));
            }
            e.call(0x00d7_af90, &args![data, 0i32, keys.len() as i32 - 1, 0u32]);
            let mut previous: Option<[i32; 3]> = None;
            let mut seen = vec![false; keys.len()];
            for index in 0..keys.len() as u32 {
                let at = data + 0x40 * index;
                let key = [
                    e.mem.i32(at + 0x20),
                    e.mem.i32(at + 0x24),
                    e.mem.i32(at + 0x28),
                ];
                if let Some(before) = previous {
                    assert!(before <= key, "{keys:?}");
                }
                previous = Some(key);
                // Whole records travel: the tag words still belong to their record.
                let tag = e.mem.u32(at + 0x10) as usize;
                assert_eq!(e.mem.u32(at + 0x34), 0x1000 + tag as u32);
                assert_eq!(key[2], tag as i32);
                assert!(!seen[tag]);
                seen[tag] = true;
            }
        }
    }

    /// Sorts `keys` as records of `stride` bytes whose float key is at `key_offset` and checks
    /// the records come back in `order` (ascending or descending), each with its payload.
    fn check_float_sort(function: u32, stride: u32, key_offset: u32, descending: bool) {
        for keys in sort_cases() {
            let (mut e, _) = bench();
            let data = e.mem.alloc(stride * keys.len() as u32);
            for (index, key) in keys.iter().enumerate() {
                let at = data + stride * index as u32;
                e.mem.set_f32(at + key_offset, *key as f32 * 0.5);
                e.mem.set_u32(at, 0x100 + index as u32);
            }
            e.call(function, &args![data, 0i32, keys.len() as i32 - 1, 0u32]);
            let mut seen = vec![false; keys.len()];
            let mut previous: Option<f32> = None;
            for index in 0..keys.len() as u32 {
                let at = data + stride * index;
                let key = e.mem.f32(at + key_offset);
                if let Some(before) = previous {
                    assert!(
                        if descending {
                            before >= key
                        } else {
                            before <= key
                        },
                        "{keys:?}"
                    );
                }
                previous = Some(key);
                let tag = (e.mem.u32(at) - 0x100) as usize;
                assert_eq!(key, keys[tag] as f32 * 0.5);
                assert!(!seen[tag]);
                seen[tag] = true;
            }
        }
    }

    #[test]
    fn weight_record_sort_orders_ascending_by_the_float() {
        check_float_sort(0x00d7_b160, 12, 8, false);
    }

    #[test]
    fn triangle_area_record_sort_orders_descending_by_the_float() {
        check_float_sort(0x00d7_b270, 8, 4, true);
    }

    #[test]
    fn node_sort_orders_by_the_plane_number() {
        for keys in sort_cases() {
            let (mut e, _) = bench();
            let data = e.mem.alloc(4 * keys.len() as u32);
            let nodes: Vec<u32> = keys
                .iter()
                .map(|key| {
                    let node = e.mem.alloc(0x50);
                    e.mem.set_i32(node + 0x38, *key);
                    node
                })
                .collect();
            for (index, node) in nodes.iter().enumerate() {
                e.mem.set_u32(data + 4 * index as u32, *node);
            }
            let centre = e.mem.alloc(0x10);
            e.call(
                0x00d7_a420,
                &args![data, 0i32, keys.len() as i32 - 1, centre],
            );
            let mut sorted_nodes: Vec<u32> = (0..keys.len() as u32)
                .map(|i| e.mem.u32(data + 4 * i))
                .collect();
            let order: Vec<i32> = sorted_nodes.iter().map(|n| e.mem.i32(n + 0x38)).collect();
            let mut want = keys.clone();
            want.sort_unstable();
            assert_eq!(order, want);
            sorted_nodes.sort_unstable();
            let mut all = nodes.clone();
            all.sort_unstable();
            assert_eq!(sorted_nodes, all);
        }
    }

    #[test]
    fn vector_sort_orders_by_the_distance_from_the_centre() {
        for keys in sort_cases() {
            let (mut e, _) = bench();
            let data = e.mem.alloc(16 * keys.len() as u32 + 16);
            for (index, key) in keys.iter().enumerate() {
                put_vec(
                    &mut e,
                    data + 16 * index as u32,
                    [*key as f32, index as f32, 0.0, 0.0],
                );
            }
            let centre = e.mem.alloc(0x10);
            put_vec(&mut e, centre, [1.0, 0.0, 0.0, 0.0]);
            // Distances to the centre (1, 0): only the x offset matters at equal index, so use
            // the pair as the distance.
            e.call(
                0x00d7_aaf0,
                &args![data, 0i32, keys.len() as i32 - 1, centre],
            );
            let distance = |e: &Engine, index: u32| {
                let v = get_vec(e, data + 16 * index);
                (v[0] - 1.0) * (v[0] - 1.0) + v[1] * v[1]
            };
            let mut seen = vec![false; keys.len()];
            for index in 0..keys.len() as u32 {
                if index > 0 {
                    assert!(distance(&e, index - 1) <= distance(&e, index), "{keys:?}");
                }
                let v = get_vec(&e, data + 16 * index);
                assert_eq!(v[0], keys[v[1] as usize] as f32);
                assert!(!seen[v[1] as usize]);
                seen[v[1] as usize] = true;
            }
        }
    }

    #[test]
    fn fn_00d7a200_classifies_the_vertices_against_a_plane() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let plane = plane_at(&mut e, [1.0, 0.0, 0.0, -0.25]);
        let counts = e.mem.alloc(8);
        let range = e.mem.alloc(8);
        let answer = e
            .call(
                0x00d7_a200,
                &args![imp, plane, 1e-4f32.to_bits(), counts, range],
            )
            .u32();
        assert_eq!(answer, 3);
        assert_eq!((e.mem.i32(counts), e.mem.i32(counts + 4)), (4, 4));
        assert_close(e.mem.f32(range), -0.25);
        assert_close(e.mem.f32(range + 4), 0.75);
        // Each vertex holds its distance.
        for node in vertex_nodes(&e, imp) {
            let p = get_vec(&e, node + 0x10);
            assert_close(e.mem.f32(node + 0x2c), p[0] - 0.25);
        }
        // Without a range block, and with the plane touching the x = 0 vertices.
        let touching = plane_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let answer = e
            .call(
                0x00d7_a200,
                &args![imp, touching, 1e-4f32.to_bits(), counts, 0u32],
            )
            .u32();
        assert_eq!(answer, 1);
        assert_eq!((e.mem.i32(counts), e.mem.i32(counts + 4)), (4, 0));
        let nodes = vertex_nodes(&e, imp);
        let zero_fields = nodes.iter().filter(|n| e.mem.u32(*n + 0x2c) == 0).count();
        assert_eq!(zero_fields, 4);
        // All behind: 2. Nothing counted (a huge epsilon): 0.
        let behind = plane_at(&mut e, [1.0, 0.0, 0.0, -5.0]);
        assert_eq!(
            e.call(
                0x00d7_a200,
                &args![imp, behind, 1e-4f32.to_bits(), counts, range]
            )
            .u32(),
            2
        );
        assert_eq!(
            e.call(
                0x00d7_a200,
                &args![imp, behind, 100.0f32.to_bits(), counts, range]
            )
            .u32(),
            0
        );
    }

    #[test]
    fn fn_00d7a350_finds_the_vertex_farthest_from_a_point() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let point = plane_at(&mut e, [0.0, 0.0, 0.0, 1.0]);
        let out = e.mem.alloc(0x10);
        let got = e.call(0x00d7_a350, &args![imp, out, point]).u32();
        assert_eq!(got, out);
        let result = get_vec(&e, out);
        assert_close(result[0], 3.0);
        assert_close(result[1], 1.0);
        assert_eq!(result[1].to_bits(), result[3].to_bits());
        // A point beyond the far corner: the near corner is the farthest.
        put_vec(&mut e, point, [2.0, 2.0, 2.0, 0.0]);
        e.call(0x00d7_a350, &args![imp, out, point]);
        assert_close(get_vec(&e, out)[0], 12.0);
    }

    #[test]
    fn fn_00d7a0b0_finds_the_transformed_vertex_farthest_along_a_direction() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let matrix = e.mem.alloc(0x40);
        // Scale x by 2 and shift y by one.
        put_vec(&mut e, matrix, [2.0, 0.0, 0.0, 0.0]);
        put_vec(&mut e, matrix + 0x10, [0.0, 1.0, 0.0, 0.0]);
        put_vec(&mut e, matrix + 0x20, [0.0, 0.0, 1.0, 0.0]);
        put_vec(&mut e, matrix + 0x30, [0.0, 1.0, 0.0, 1.0]);
        let direction = plane_at(&mut e, [1.0, 1.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00d7_a0b0, &args![imp, matrix, direction, out]);
        let got = get_vec(&e, out);
        assert_vec_close([got[0], got[1], got[3], 0.0], [2.0, 2.0, 1.0, 0.0]);
        // An empty hull leaves the output alone.
        let (_, empty) = fresh_hull(&mut e);
        put_vec(&mut e, out, [7.0; 4]);
        e.call(0x00d7_a0b0, &args![empty, matrix, direction, out]);
        assert_eq!(get_vec(&e, out), [7.0; 4]);
    }

    #[test]
    fn fn_00d7a810_finds_the_triangle_plane_the_point_is_furthest_in_front_of() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let matrix = e.mem.alloc(0x40);
        for (row, vector) in [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .iter()
        .enumerate()
        {
            put_vec(&mut e, matrix + 0x10 * row as u32, *vector);
        }
        let points = e.mem.alloc(8);
        let data = e.mem.alloc(0x20);
        put_vec(&mut e, data, [0.5, 0.5, 0.5, 1.0]);
        put_vec(&mut e, data + 0x10, [2.0, 0.5, 0.5, 1.0]);
        e.mem.set_u32(points, data);
        e.mem.set_u32(points + 4, 2);
        let out = e.mem.alloc(0x10);
        let distance = e.call(0x00d7_a810, &args![imp, points, matrix, out]).f32();
        assert!((distance - 1.0).abs() < 1e-2, "{distance}");
        let plane = get_vec(&e, out);
        assert_vec_close([plane[0], plane[1], plane[2], 0.0], [1.0, 0.0, 0.0, 0.0]);
        assert!((plane[3] + 1.0).abs() < 1e-2);
        // Only the inside point: nothing is in front of any plane, the answer stays the epsilon.
        e.mem.set_i32(points + 4, 1);
        put_vec(&mut e, out, [9.0; 4]);
        let distance = e.call(0x00d7_a810, &args![imp, points, matrix, out]).f32();
        assert_eq!(distance, f32::from_bits(0x3400_0000));
        assert_eq!(get_vec(&e, out), [9.0; 4]);
    }

    #[test]
    fn centroid_averages_the_grid_positions_once() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        // The cache is marked empty by the largest float in its fourth lane (the reset puts it).
        assert_eq!(e.mem.u32(imp + 0x8c), 0x7f7f_ffee);
        let got = e.call(0x00d7_aa30, &args![imp]).u32();
        assert_eq!(got, imp + 0x80);
        let centre = get_vec(&e, imp + 0x80);
        assert_vec_close(
            [centre[0], centre[1], centre[2], centre[3]],
            [0.5, 0.5, 0.5, 0.0],
        );
        // A cached value is kept.
        put_vec(&mut e, imp + 0x80, [9.0, 9.0, 9.0, 0.0]);
        e.call(0x00d7_aa30, &args![imp]);
        assert_eq!(get_vec(&e, imp + 0x80), [9.0, 9.0, 9.0, 0.0]);
        // Without vertices the sum is not divided.
        let (_, empty) = fresh_hull(&mut e);
        assert_eq!(e.mem.u32(empty + 0x8c), 0x7f7f_ffee);
        e.call(0x00d7_aa30, &args![empty]);
        assert_eq!(get_vec(&e, empty + 0x80), [0.0; 4]);
    }

    /// An implementation block holding only the planes (indices valid).
    fn implementation_with_planes(e: &mut Engine, planes: &[[f32; 4]]) -> u32 {
        let imp = e.mem.alloc(0x1c0);
        let block = plane_block(e, planes);
        e.mem.set_u32(imp + 0x54, block);
        e.mem.set_i32(imp + 0x58, planes.len() as i32);
        e.mem.set_u8(imp + 0x1b5, 1);
        imp
    }

    #[test]
    fn segment_clip_finds_the_parameters_where_it_crosses_the_planes() {
        let (mut e, _) = bench();
        let (first, second) = (e.mem.alloc(4), e.mem.alloc(4));
        let limit = 100.0f32.to_bits();
        let (start, end) = (
            plane_at(&mut e, [-1.0, 0.5, 0.5, 1.0]),
            plane_at(&mut e, [3.0, 0.5, 0.5, 1.0]),
        );
        // The plane x <= 1: the start is behind it (distance -2), the end in front (2). The
        // crossing at t = 1/2 goes to the first word when the start distance is negative.
        let imp = implementation_with_planes(&mut e, &[[1.0, 0.0, 0.0, -1.0]]);
        assert!(e
            .call(0x00d7_a5a0, &args![imp, start, end, first, second, limit])
            .bool());
        assert_eq!((e.mem.f32(first), e.mem.f32(second)), (0.5, 1.0));
        // The plane -x <= 0: the start is in front (1), the end behind (-3): t = 1/4 goes to the second.
        let imp = implementation_with_planes(&mut e, &[[-1.0, 0.0, 0.0, 0.0]]);
        assert!(e
            .call(0x00d7_a5a0, &args![imp, start, end, first, second, limit])
            .bool());
        assert_eq!((e.mem.f32(first), e.mem.f32(second)), (0.0, 0.25));
        // Both ends beyond the limit of the same plane: rejected.
        let (start, end) = (
            plane_at(&mut e, [3.0, 0.5, 0.5, 1.0]),
            plane_at(&mut e, [4.0, 0.5, 0.5, 1.0]),
        );
        let imp = implementation_with_planes(&mut e, &[[1.0, 0.0, 0.0, -1.0]]);
        assert!(!e
            .call(
                0x00d7_a5a0,
                &args![imp, start, end, first, second, 0.5f32.to_bits()]
            )
            .bool());
        // A segment behind every plane does not change the parameters; no planes: true as well.
        let (start, end) = (
            plane_at(&mut e, [0.25, 0.5, 0.5, 1.0]),
            plane_at(&mut e, [0.75, 0.5, 0.5, 1.0]),
        );
        assert!(e
            .call(0x00d7_a5a0, &args![imp, start, end, first, second, limit])
            .bool());
        assert_eq!((e.mem.f32(first), e.mem.f32(second)), (0.0, 1.0));
        let none = implementation_with_planes(&mut e, &[]);
        assert!(e
            .call(0x00d7_a5a0, &args![none, start, end, first, second, limit])
            .bool());
    }

    #[test]
    fn segment_clip_without_indices_breaks_at_its_address() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        e.mem.set_u8(imp + 0x1b5, 0);
        install_report(&mut e, 1);
        let (start, end) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        let words = e.mem.alloc(8);
        let message = int3_message(
            &mut e,
            0x00d7_a5a0,
            &args![imp, start, end, words, words + 4, 0.0f32.to_bits()],
        );
        assert!(message.contains("INT3 at 00d7a61c"), "{message}");
    }

    /// The twelve vertices of an icosahedron (the golden ratio `1.618034`).
    const ICOSAHEDRON: [[f32; 3]; 12] = [
        [0.0, 1.0, 1.618_034],
        [0.0, 1.0, -1.618_034],
        [0.0, -1.0, 1.618_034],
        [0.0, -1.0, -1.618_034],
        [1.0, 1.618_034, 0.0],
        [1.0, -1.618_034, 0.0],
        [-1.0, 1.618_034, 0.0],
        [-1.0, -1.618_034, 0.0],
        [1.618_034, 0.0, 1.0],
        [1.618_034, 0.0, -1.0],
        [-1.618_034, 0.0, 1.0],
        [-1.618_034, 0.0, -1.0],
    ];

    #[test]
    fn decimating_the_hull_forwards_to_the_implementation() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &ICOSAHEDRON);
        assert_eq!(e.mem.i32(imp + 0x28), 12);
        let worked = e.call(0x00d7_7bd0, &args![hull, 8i32, 0u32]).bool();
        assert!(worked);
        assert_eq!(e.mem.i32(imp + 0x28), 8);
    }

    #[test]
    fn decimate_removes_the_lightest_vertices_down_to_the_target() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &ICOSAHEDRON);
        let worked = e.call(0x00d8_2ad0, &args![imp, 9i32, 0u32]).bool();
        assert!(worked);
        assert_eq!(e.mem.i32(imp + 0x28), 9);
        assert_eq!(e.mem.i32(imp + 0x1a8), 3);
        // The indices were valid before and are valid again.
        assert_eq!(e.mem.u8(imp + 0x1b5), 1);
        // Every remaining vertex is one of the original ones.
        for position in vertex_positions(&e, imp) {
            assert!(ICOSAHEDRON
                .iter()
                .any(|p| (0..3).all(|lane| (p[lane] - position[lane]).abs() < 1e-3)));
        }
        // A target at or above the vertex count does nothing; a target below 4 means 4.
        assert!(e.call(0x00d8_2ad0, &args![imp, 20i32, 0u32]).bool());
        assert_eq!(e.mem.i32(imp + 0x28), 9);
        assert!(e.call(0x00d8_2ad0, &args![imp, 1i32, 0u32]).bool());
        assert_eq!(e.mem.i32(imp + 0x28), 4);
    }

    #[test]
    fn decimate_with_containment_keeps_the_original_points_inside() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &ICOSAHEDRON);
        e.call(0x00d7_b600, &args![imp]);
        let worked = e.call(0x00d8_2ad0, &args![imp, 8i32, 1u32]).bool();
        assert!(worked);
        // Mass properties existed and are rebuilt.
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        for point in ICOSAHEDRON {
            let at = aligned_points(&mut e, &[point]);
            let inside = e
                .call(0x00d7_6100, &args![hull, at, 1u32, 0.01f32.to_bits()])
                .bool();
            assert!(inside, "{point:?}");
        }
    }

    #[test]
    fn decimate_reports_a_hull_that_is_not_a_solid() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let log = install_report(&mut e, 0);
        // The count of dimensions is checked first; the weighing that follows uses the triangles.
        e.mem.set_i32(imp + 0x1a8, 2);
        e.call(0x00d8_2ad0, &args![imp, 7i32, 0u32]);
        let log = log.borrow();
        assert_eq!(log[1].1[1], MESSAGE_INVALID_DIMENSION);
        assert_eq!(log[2].1[1], 3);
        assert_eq!(log[3].1[1], MESSAGE_REQUIRED);
        assert_eq!(log[4].1[1..3], [3, 0x79f9_d888]);
        assert_eq!(log[4].1[4..], [REPORT_FILE, 0x154]);
    }

    #[test]
    fn decimate_of_a_planar_hull_breaks_at_its_address() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        e.mem.set_i32(imp + 0x1a8, 2);
        install_report(&mut e, 1);
        let message = int3_message(&mut e, 0x00d8_2ad0, &args![imp, 4i32, 0u32]);
        assert!(message.contains("INT3 at 00d82b9d"), "{message}");
    }

    #[test]
    fn fn_00d78b90_finds_a_simplified_copy_within_the_tolerance() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &ICOSAHEDRON);
        e.call(0x00d7_62f0, &args![hull]);
        let volume = e.mem.f32(imp + 0x1a0);
        let copy = e
            .call(0x00d7_8b90, &args![hull, 0.3f32.to_bits(), 0u32])
            .u32();
        assert_ne!(copy, 0);
        assert_ne!(copy, hull);
        let copied = e.mem.u32(copy + 8);
        assert_eq!(e.mem.i32(copied + 0x1a8), 3);
        let vertices = e.mem.i32(copied + 0x28);
        assert!((4..=12).contains(&vertices), "{vertices}");
        e.call(0x00d7_62f0, &args![copy]);
        let relative = (e.mem.f32(copied + 0x1a0) - volume).abs() / volume;
        assert!(relative < 0.3, "{relative}");
        // A hull that is not three-dimensional has no simplified copy.
        let (flat, _) = built_hull(&mut e, &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        assert_eq!(
            e.call(0x00d7_8b90, &args![flat, 0.3f32.to_bits(), 0u32])
                .u32(),
            0
        );
    }

    #[test]
    fn fn_00d78ca0_builds_the_hull_of_sums_of_points() {
        let (mut e, _) = bench();
        let first = strided_points(&mut e, &CUBE, 12);
        let second = strided_points(&mut e, &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], 12);
        let scale = e.mem.alloc(4);
        e.mem.set_f32(scale, 2.0);
        let hull = e.call(0x00d7_8ca0, &args![first, second, scale]).u32();
        assert_ne!(hull, 0);
        // The cube and a copy two further along x: the box [0, 3] x [0, 1] x [0, 1].
        assert_box(&e, hull, [0.0; 3], [3.0, 1.0, 1.0]);
        assert_eq!(e.mem.i32(e.mem.u32(hull + 8) + 0x1a8), 3);
        // The retry byte of the configuration is set, so coplanar points still make a polygon.
        let flat = strided_points(&mut e, &CUBE[..4], 12);
        let point = strided_points(&mut e, &[[0.0, 0.0, 0.0]], 12);
        let polygon = e.call(0x00d7_8ca0, &args![flat, point, scale]).u32();
        assert_ne!(polygon, 0);
        assert_eq!(e.mem.i32(e.mem.u32(polygon + 8) + 0x1a8), 2);
        // No points at all: the build fails and nothing is answered.
        let none = strided_points(&mut e, &[], 12);
        assert_eq!(e.call(0x00d7_8ca0, &args![none, point, scale]).u32(), 0);
    }

    #[test]
    fn fn_00d78ea0_builds_the_hull_of_several_hulls_and_merges_their_owned_objects() {
        let (mut e, _) = bench();
        let (first, _) = box_hull(&mut e, [0.0; 3], [1.0; 3]);
        let (second, _) = box_hull(&mut e, [2.0, 0.0, 0.0], [3.0, 1.0, 1.0]);
        let hulls = e.mem.alloc(8);
        e.mem.set_u32(hulls, first);
        e.mem.set_u32(hulls + 4, second);
        let union = e.call(0x00d7_8ea0, &args![0u32, hulls, 2i32]).u32();
        assert_ne!(union, 0);
        assert_box(&e, union, [0.0; 3], [3.0, 1.0, 1.0]);
        assert_eq!(e.mem.u32(union + 0x10), 0);
        // The first owned object is cloned and the other hull's is added to the clone.
        let (object_one, log_one) = owned_object(&mut e, 0x6000);
        e.mem.set_u32(first + 0x10, object_one);
        let added = e.mem.alloc(0x10);
        e.mem.set_u32(added, e.mem.u32(object_one));
        e.mem.set_u32(second + 0x10, added);
        // The clone's own slot 8 must exist: give the clone object a vtable whose slot 8 logs.
        let clone = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        const ADD_SLOT_DOUBLE: u32 = 0x0900_0050;
        let adds: Log = Rc::new(RefCell::new(Vec::new()));
        let add_log = Rc::clone(&adds);
        e.register_double(ADD_SLOT_DOUBLE, move |_, a| {
            add_log.borrow_mut().push((ADD_SLOT_DOUBLE, a.to_vec()));
            Ret::default()
        });
        e.mem.set_u32(vtable + 8, ADD_SLOT_DOUBLE);
        e.mem.set_u32(clone, vtable);
        let clone_log = Rc::clone(&log_one);
        e.register_double(OBJECT_CLONE_DOUBLE, move |_, a| {
            clone_log
                .borrow_mut()
                .push((OBJECT_CLONE_DOUBLE, a.to_vec()));
            Ret {
                eax: clone,
                ..Ret::default()
            }
        });
        let union = e.call(0x00d7_8ea0, &args![0u32, hulls, 2i32]).u32();
        assert_eq!(e.mem.u32(union + 0x10), clone);
        assert_eq!(*adds.borrow(), vec![(ADD_SLOT_DOUBLE, vec![clone, added])]);
        // No hulls: nothing.
        assert_eq!(e.call(0x00d7_8ea0, &args![0u32, hulls, 0i32]).u32(), 0);
        // Hulls without vertices: the build fails and the new hull is deleted.
        let (mut e, _) = bench();
        let (empty, _) = fresh_hull(&mut e);
        let hulls = e.mem.alloc(4);
        e.mem.set_u32(hulls, empty);
        assert_eq!(e.call(0x00d7_8ea0, &args![0u32, hulls, 1i32]).u32(), 0);
    }

    #[test]
    fn fn_00d79060_unites_two_hulls() {
        let (mut e, _) = bench();
        let (first, _) = box_hull(&mut e, [0.0; 3], [1.0; 3]);
        let (second, _) = box_hull(&mut e, [0.0, 2.0, 0.0], [1.0, 3.0, 1.0]);
        let union = e.call(0x00d7_9060, &args![0u32, first, second]).u32();
        assert_ne!(union, 0);
        assert_box(&e, union, [0.0; 3], [1.0, 3.0, 1.0]);
    }

    /// A parameter block `{mode, limit}`.
    fn shrink_parameters(e: &mut Engine, mode: i32, limit: f32) -> u32 {
        let at = e.mem.alloc(8);
        e.mem.set_i32(at, mode);
        e.mem.set_f32(at + 4, limit);
        at
    }

    #[test]
    fn shrinking_the_planes_moves_every_face_inwards() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &CUBE);
        let params = shrink_parameters(&mut e, 1, 0.0);
        e.call(0x00d7_8720, &args![hull, (-0.1f32).to_bits(), params]);
        assert_box(&e, hull, [0.1; 3], [0.9; 3]);
        // The mass properties and the indices were rebuilt afterwards.
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        assert_eq!(e.mem.u8(imp + 0x1b5), 1);
        assert_eq!(e.mem.i32(imp + 0x58), 6);
        assert_close(e.mem.f32(imp + 0x1a0), 0.512);
    }

    #[test]
    fn shrinking_the_planes_of_a_polygon_uses_the_third_plane() {
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let square = [
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [0.0, 2.0, 0.0],
        ];
        let at = aligned_points(&mut e, &square);
        e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 2);
        let params = shrink_parameters(&mut e, 1, 0.0);
        e.call(0x00d7_8720, &args![hull, (-0.5f32).to_bits(), params]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 2);
        assert_box(&e, hull, [0.5, 0.5, 0.0], [1.5, 1.5, 0.0]);
    }

    #[test]
    fn shrinking_the_vertices_pulls_the_far_ones_towards_the_centre() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        // The corners are 0.866 from the centre; those beyond the limit 0.5 move in by 0.2.
        let params = shrink_parameters(&mut e, 0, 0.5);
        e.call(0x00d7_8720, &args![hull, (-0.2f32).to_bits(), params]);
        let half = 0.5 * (0.866_025_4 - 0.2) / 0.866_025_4;
        assert_box(&e, hull, [0.5 - half; 3], [0.5 + half; 3]);
        // With a limit above their distance nothing moves.
        let (hull, _) = built_hull(&mut e, &CUBE);
        let params = shrink_parameters(&mut e, 0, 2.0);
        e.call(0x00d7_8720, &args![hull, (-0.2f32).to_bits(), params]);
        assert_box(&e, hull, [0.0; 3], [1.0; 3]);
        // Another mode only rebuilds the properties and the indices.
        let params = shrink_parameters(&mut e, 5, 0.0);
        e.call(0x00d7_8720, &args![hull, 0.0f32.to_bits(), params]);
        assert_box(&e, hull, [0.0; 3], [1.0; 3]);
    }

    #[test]
    fn shrinking_without_indices_breaks_at_its_address() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &CUBE);
        e.mem.set_u8(imp + 0x1b5, 0);
        install_report(&mut e, 1);
        let params = shrink_parameters(&mut e, 1, 0.0);
        let message = int3_message(&mut e, 0x00d7_8720, &args![hull, 0.0f32.to_bits(), params]);
        assert!(message.contains("INT3 at 00d7879f"), "{message}");
    }

    /// The points in an array `{data, count}` of 16-byte vectors.
    fn point_array(e: &mut Engine, points: &[[f32; 3]]) -> u32 {
        let data = aligned_points(e, points);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, data);
        e.mem.set_i32(array + 4, points.len() as i32);
        array
    }

    #[test]
    fn clustering_cuts_the_hull_into_cells_around_far_apart_points() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let points = point_array(
            &mut e,
            &[[0.1, 0.5, 0.5], [0.9, 0.5, 0.5], [0.12, 0.5, 0.5]],
        );
        let out = empty_array(&mut e);
        e.call(0x00d7_9ab0, &args![hull, points, out, 0.2f32.to_bits()]);
        // The third point lies within the radius of the first and makes no cluster of its own.
        let cells = array_hulls(&e, out);
        assert_eq!(cells.len(), 2);
        // Each cell keeps its side of the planes half way between the centres, pulled back by the
        // radius: x up to 0.4 and from 0.6.
        assert_box(&e, cells[0], [0.0; 3], [0.4, 1.0, 1.0]);
        assert_box(&e, cells[1], [0.6, 0.0, 0.0], [1.0; 3]);
        // The configuration bytes for the indices and the mass properties were cleared on the
        // clones, and the cells are indexed only when the hull is.
        assert_eq!(e.mem.u8(e.mem.u32(cells[0] + 8) + 0x1b5), 1);
        assert_eq!(e.mem.u8(e.mem.u32(cells[0] + 8) + 0x1b4), 0);
    }

    #[test]
    fn clustering_with_one_centre_keeps_the_whole_hull() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let points = point_array(&mut e, &[[0.5, 0.5, 0.5]]);
        let out = empty_array(&mut e);
        e.call(0x00d7_9ab0, &args![hull, points, out, 0.1f32.to_bits()]);
        let cells = array_hulls(&e, out);
        assert_eq!(cells.len(), 1);
        assert_box(&e, cells[0], [0.0; 3], [1.0; 3]);
        // No points: no cells.
        let none = point_array(&mut e, &[]);
        let out = empty_array(&mut e);
        e.call(0x00d7_9ab0, &args![hull, none, out, 0.1f32.to_bits()]);
        assert_eq!(e.mem.i32(out + 4), 0);
    }

    #[test]
    fn fn_00d79f90_cuts_the_shrunk_hull_by_the_cells() {
        let (mut e, _) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let points = point_array(&mut e, &[[0.1, 0.5, 0.5], [0.9, 0.5, 0.5]]);
        let clusters = empty_array(&mut e);
        let out = empty_array(&mut e);
        e.call(
            0x00d7_9f90,
            &args![hull, points, clusters, out, 0.2f32.to_bits()],
        );
        // The cells are left in the cluster array; what the cells leave of the hull shrunk by the
        // radius (0.2 to 0.8 in each direction) is the slab between them.
        assert_eq!(array_hulls(&e, clusters).len(), 2);
        let pieces = array_hulls(&e, out);
        assert_eq!(pieces.len(), 1);
        assert_box(&e, pieces[0], [0.4, 0.2, 0.2], [0.6, 0.8, 0.8]);
    }

    #[test]
    fn fn_00d7a010_deletes_its_clusters_and_leaves_the_pieces() {
        let (mut e, frees) = bench();
        let (hull, _) = built_hull(&mut e, &CUBE);
        let points = point_array(&mut e, &[[0.1, 0.5, 0.5], [0.9, 0.5, 0.5]]);
        let out = empty_array(&mut e);
        e.call(0x00d7_a010, &args![hull, points, out, 0.2f32.to_bits()]);
        let pieces = array_hulls(&e, out);
        assert_eq!(pieces.len(), 1);
        assert_box(&e, pieces[0], [0.4, 0.2, 0.2], [0.6, 0.8, 0.8]);
        // The temporary array of clusters went back through the allocator.
        assert!(frees
            .borrow()
            .iter()
            .any(|(d, a)| *d == FREE_ARRAY_SLOT_DOUBLE && a[2] == 16));
    }

    /// The cube with the centre of every face pushed out a little: fourteen vertices whose
    /// extra triangles are nearly in the planes of the faces.
    fn bumped_cube() -> Vec<[f32; 3]> {
        let mut points = CUBE.to_vec();
        for (axis, value) in [(0, 0.0), (0, 1.0), (1, 0.0), (1, 1.0), (2, 0.0), (2, 1.0)] {
            let mut p = [0.5, 0.5, 0.5];
            p[axis] = if value == 0.0 { -0.002 } else { 1.002 };
            points.push(p);
        }
        points
    }

    #[test]
    fn edge_length_total_by_dimension() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        // Without plane numbers every triangle edge counts once: the 12 edges and the 6 diagonals.
        let triangles = triangle_nodes(&e, imp);
        let saved: Vec<u32> = triangles.iter().map(|t| e.mem.u32(t + 0x38)).collect();
        for triangle in &triangles {
            e.mem.set_u32(triangle + 0x38, 0xffff_ffff);
        }
        let all = e.call(0x00d7_cf00, &args![imp]).f32();
        assert_close(all, 12.0 + 6.0 * 2.0f32.sqrt());
        // With the planes the diagonals inside a face are skipped.
        for (triangle, number) in triangles.iter().zip(&saved) {
            e.mem.set_u32(triangle + 0x38, *number);
        }
        assert_close(e.call(0x00d7_cf00, &args![imp]).f32(), 12.0);
        // A polygon: its perimeter. A segment: its length. A point: nothing.
        let (_, square) = fresh_hull(&mut e);
        e.mem.set_u8(square, 1);
        let at = aligned_points(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [2.0, 3.0, 0.0],
                [0.0, 3.0, 0.0],
            ],
        );
        e.call(0x00d8_1510, &args![square, at, 4i32, 0u32, 0u32]);
        assert_close(e.call(0x00d7_cf00, &args![square]).f32(), 10.0);
        let (_, segment) = built_hull(&mut e, &[[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]]);
        assert_close(e.call(0x00d7_cf00, &args![segment]).f32(), 5.0);
        let (_, point) = built_hull(&mut e, &[[1.0, 1.0, 1.0]]);
        assert_eq!(e.call(0x00d7_cf00, &args![point]).f32(), 0.0);
    }

    #[test]
    fn plane_merging_adds_a_plane_for_every_sharp_edge() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let out = empty_array(&mut e);
        // The faces of the cube meet at right angles: their normals have a dot product of zero.
        e.call(0x00d7_caa0, &args![imp, 0.5f32.to_bits(), out]);
        assert_eq!(e.mem.i32(out + 4), 12);
        let planes = e.mem.u32(out);
        let positions = vertex_positions(&e, imp);
        for index in 0..12 {
            let plane = get_vec(&e, planes + 16 * index);
            let length = (plane[0] * plane[0] + plane[1] * plane[1] + plane[2] * plane[2]).sqrt();
            assert_close(length, 1.0);
            // A diagonal normal: two non-zero components.
            assert_eq!(
                plane[..3].iter().filter(|v| v.abs() > 0.5).count(),
                2,
                "{plane:?}"
            );
            // The offset is taken at the first vertex only.
            let first = e.mem.u32(imp + 0x24);
            let at = get_vec(&e, first + 0x10);
            let distance = plane[0] * at[0] + plane[1] * at[1] + plane[2] * at[2] + plane[3];
            assert!(distance.abs() < 1e-3, "{distance}");
            let _ = &positions;
        }
        // With the byte at +0xd set the largest dot product over all vertices is used: no vertex
        // is in front of the plane.
        e.mem.set_u8(imp + 0xd, 1);
        let tight = empty_array(&mut e);
        e.call(0x00d7_caa0, &args![imp, 0.5f32.to_bits(), tight]);
        assert_eq!(e.mem.i32(tight + 4), 12);
        for index in 0..12 {
            let plane = get_vec(&e, e.mem.u32(tight) + 16 * index);
            let distances: Vec<f32> = positions
                .iter()
                .map(|p| plane[0] * p[0] + plane[1] * p[1] + plane[2] * p[2] + plane[3])
                .collect();
            assert!(distances.iter().all(|d| *d < 1e-3), "{plane:?}");
            assert_eq!(distances.iter().filter(|d| d.abs() < 1e-3).count(), 2);
        }
        // A threshold below every dot product adds nothing.
        let none = empty_array(&mut e);
        e.call(0x00d7_caa0, &args![imp, (-0.5f32).to_bits(), none]);
        assert_eq!(e.mem.i32(none + 4), 0);
    }

    #[test]
    fn plane_merging_needs_indices_and_a_solid() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let out = empty_array(&mut e);
        // Not three dimensional: nothing is added.
        e.mem.set_i32(imp + 0x1a8, 2);
        e.call(0x00d7_caa0, &args![imp, 0.5f32.to_bits(), out]);
        assert_eq!(e.mem.i32(out + 4), 0);
        // Without indices the report breaks.
        e.mem.set_u8(imp + 0x1b5, 0);
        install_report(&mut e, 1);
        let message = int3_message(&mut e, 0x00d7_caa0, &args![imp, 0.5f32.to_bits(), out]);
        assert!(message.contains("INT3 at 00d7cb26"), "{message}");
    }

    /// Checks the planes the indices built for the hull: every vertex is behind or on every plane
    /// and every triangle's plane word names a plane its three vertices lie on.
    fn assert_planes_cover_the_hull(e: &Engine, imp: u32, plane_count: i32) {
        assert_eq!(e.mem.i32(imp + 0x58), plane_count);
        let planes = e.mem.u32(imp + 0x54);
        let positions = vertex_positions(e, imp);
        for index in 0..plane_count as u32 {
            let plane = get_vec(e, planes + 16 * index);
            for p in &positions {
                let distance = plane[0] * p[0] + plane[1] * p[1] + plane[2] * p[2] + plane[3];
                assert!(distance < 2e-3, "{plane:?} {p:?}");
            }
        }
        for triangle in triangle_nodes(e, imp) {
            let number = e.mem.u32(triangle + 0x38);
            assert!((number as i32) >= 0 && (number as i32) < plane_count);
            let plane = get_vec(e, planes + 16 * number);
            for corner in 0..3u32 {
                let p = get_vec(e, e.mem.u32(triangle + 8 + 4 * corner) + 0x10);
                let distance = plane[0] * p[0] + plane[1] * p[1] + plane[2] * p[2] + plane[3];
                assert!(distance.abs() < 2e-3, "{plane:?} {p:?}");
            }
        }
    }

    #[test]
    fn build_indices_makes_the_planes_of_a_solid() {
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        // The indices are left to this call.
        e.mem.set_u8(imp + 9, 0);
        let at = aligned_points(&mut e, &CUBE);
        e.call(0x00d8_1510, &args![imp, at, 8i32, 0u32, 0u32]);
        assert_eq!(e.mem.u8(imp + 0x1b5), 0);
        e.call(0x00d7_d2a0, &args![imp]);
        assert_eq!(e.mem.u8(imp + 0x1b5), 1);
        assert_planes_cover_the_hull(&e, imp, 6);
        // The vertices are numbered in list order.
        let numbers: Vec<u32> = vertex_nodes(&e, imp)
            .iter()
            .map(|n| e.mem.u32(n + 0x34))
            .collect();
        assert_eq!(numbers, (0..8).collect::<Vec<u32>>());
        // Called again with valid indices it changes nothing.
        let planes = e.mem.u32(imp + 0x54);
        e.call(0x00d7_d2a0, &args![imp]);
        assert_eq!(e.mem.u32(imp + 0x54), planes);
    }

    #[test]
    fn build_indices_merges_nearly_coplanar_triangles() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &bumped_cube());
        assert_eq!(e.mem.i32(imp + 0x28), 14);
        assert_eq!(e.mem.i32(imp + 0x40), 24);
        // The 24 triangles still make only six planes.
        assert_eq!(e.mem.i32(imp + 0x58), 6);
        // A tolerance above every dot product keeps every triangle apart (the planes of
        // the pyramids are not exactly the plane of the face).
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        e.mem.set_f32(imp + 4, 1.5);
        let at = aligned_points(&mut e, &bumped_cube());
        e.call(0x00d8_1510, &args![imp, at, 14i32, 0u32, 0u32]);
        assert_eq!(e.mem.i32(imp + 0x58), 24);
    }

    #[test]
    fn build_indices_of_a_polygon_makes_one_plane_per_edge() {
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let at = aligned_points(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [2.0, 2.0, 0.0],
                [0.0, 2.0, 0.0],
            ],
        );
        e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 2);
        assert_eq!(e.mem.i32(imp + 0x58), 4);
        let planes = e.mem.u32(imp + 0x54);
        let positions = vertex_positions(&e, imp);
        let centre = [1.0f32, 1.0, 0.0];
        for index in 0..4u32 {
            let plane = get_vec(&e, planes + 16 * index);
            let distances: Vec<f32> = positions
                .iter()
                .map(|p| plane[0] * p[0] + plane[1] * p[1] + plane[2] * p[2] + plane[3])
                .collect();
            assert!(distances.iter().all(|d| *d < 1e-3), "{plane:?}");
            assert_eq!(distances.iter().filter(|d| d.abs() < 1e-3).count(), 2);
            let at_centre =
                plane[0] * centre[0] + plane[1] * centre[1] + plane[2] * centre[2] + plane[3];
            assert!(at_centre < -0.5);
        }
    }

    #[test]
    fn build_indices_of_a_segment_makes_its_two_end_planes() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &[[0.0, 0.0, 0.0], [4.0, 0.0, 0.0]]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 1);
        assert_eq!(e.mem.i32(imp + 0x58), 2);
        let planes = e.mem.u32(imp + 0x54);
        let mut both = [get_vec(&e, planes), get_vec(&e, planes + 16)];
        // One plane through each end, facing away from the segment (the order follows the list).
        both.sort_by(|a, b| b[0].partial_cmp(&a[0]).unwrap());
        let (first, second) = (both[0], both[1]);
        assert_vec_close([first[0], first[1], first[2], 0.0], [1.0, 0.0, 0.0, 0.0]);
        assert_vec_close(
            [second[0], second[1], second[2], 0.0],
            [-1.0, 0.0, 0.0, 0.0],
        );
        // The first plane passes through the end at x = 4, the second through the start.
        assert!((first[3] + 4.0).abs() < 1e-3, "{first:?}");
        assert!(second[3].abs() < 1e-3, "{second:?}");
    }

    /// The vertex positions of the faces `generate_indexed_faces` made, by vertex number.
    fn faces_of(e: &Engine, imp: u32, faces: u32, indices: u32) -> Vec<Vec<[f32; 3]>> {
        let by_number = |number: u32| -> [f32; 3] {
            let node = vertex_nodes(e, imp)
                .into_iter()
                .find(|n| e.mem.u32(n + 0x34) == number)
                .unwrap();
            let v = get_vec(e, node + 0x10);
            [v[0], v[1], v[2]]
        };
        let mut at = 0u32;
        let mut result = Vec::new();
        for face in 0..e.mem.u32(faces + 4) {
            let length = e.mem.u32(e.mem.u32(faces) + 4 * face);
            result.push(
                (0..length)
                    .map(|k| by_number(e.mem.u32(e.mem.u32(indices) + 4 * (at + k))))
                    .collect(),
            );
            at += length;
        }
        result
    }

    #[test]
    fn indexed_faces_of_a_cube_are_six_quadrilaterals() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let (faces, indices) = (empty_array(&mut e), empty_array(&mut e));
        let count = e
            .call(0x00d7_df60, &args![imp, faces, indices, 0u32, 0u32])
            .i32();
        assert_eq!(count, 6);
        assert_eq!(e.mem.i32(indices + 4), 24);
        let outlines = faces_of(&e, imp, faces, indices);
        assert_eq!(outlines.len(), 6);
        for outline in &outlines {
            assert_eq!(outline.len(), 4);
            // Four distinct corners with one coordinate in common.
            let constant = (0..3).any(|axis| outline.iter().all(|p| p[axis] == outline[0][axis]));
            assert!(constant, "{outline:?}");
            let mut distinct = outline.clone();
            distinct.sort_by(|a, b| a.partial_cmp(b).unwrap());
            distinct.dedup();
            assert_eq!(distinct.len(), 4);
        }
        // Reordering rotates each face to start at its best corner and keeps the same corners.
        let (reordered_faces, reordered_indices) = (empty_array(&mut e), empty_array(&mut e));
        let count = e
            .call(
                0x00d7_df60,
                &args![imp, reordered_faces, reordered_indices, 0u32, 1u32],
            )
            .i32();
        assert_eq!(count, 6);
        let sorted_faces = |faces: &Vec<Vec<[f32; 3]>>| {
            let mut corners: Vec<Vec<[f32; 3]>> = faces.iter().map(|f| sorted(f)).collect();
            corners.sort_by(|a, b| a.partial_cmp(b).unwrap());
            corners
        };
        let rotated = faces_of(&e, imp, reordered_faces, reordered_indices);
        assert_eq!(sorted_faces(&rotated), sorted_faces(&outlines));
    }

    #[test]
    fn indexed_faces_ignore_the_vertices_inside_a_face() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &bumped_cube());
        let (faces, indices) = (empty_array(&mut e), empty_array(&mut e));
        let count = e
            .call(0x00d7_df60, &args![imp, faces, indices, 0u32, 0u32])
            .i32();
        assert_eq!(count, 6);
        for outline in faces_of(&e, imp, faces, indices) {
            assert_eq!(outline.len(), 4);
            assert!(outline
                .iter()
                .all(|p| (0..3).any(|axis| p[axis] == 0.0 || p[axis] == 1.0)));
        }
    }

    #[test]
    fn indexed_faces_can_answer_the_ids_stored_in_the_vertices() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        for (number, node) in vertex_nodes(&e, imp).iter().enumerate() {
            e.mem
                .set_u32(node + 0x1c, 0x3f00_0000 | (100 + number as u32));
        }
        let (faces, indices) = (empty_array(&mut e), empty_array(&mut e));
        e.call(0x00d7_df60, &args![imp, faces, indices, 1u32, 0u32]);
        for slot in 0..24 {
            let id = e.mem.u32(e.mem.u32(indices) + 4 * slot);
            assert!((100..108).contains(&id), "{id}");
        }
    }

    #[test]
    fn indexed_faces_of_a_polygon_are_one_face_of_all_the_vertices() {
        let (mut e, _) = bench();
        let (_, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let at = aligned_points(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [2.0, 2.0, 0.0],
                [0.0, 2.0, 0.0],
            ],
        );
        e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]);
        let (faces, indices) = (empty_array(&mut e), empty_array(&mut e));
        let count = e
            .call(0x00d7_df60, &args![imp, faces, indices, 0u32, 0u32])
            .i32();
        assert_eq!(count, 1);
        assert_eq!(e.mem.u32(e.mem.u32(faces)), 4);
        let numbers: Vec<u32> = (0..4)
            .map(|k| e.mem.u32(e.mem.u32(indices) + 4 * k))
            .collect();
        assert_eq!(numbers, vec![0, 1, 2, 3]);
        // Without indices it breaks at 00d7dfe7.
        e.mem.set_u8(imp + 0x1b5, 0);
        install_report(&mut e, 1);
        let message = int3_message(&mut e, 0x00d7_df60, &args![imp, faces, indices, 0u32, 0u32]);
        assert!(message.contains("INT3 at 00d7dfe7"), "{message}");
    }

    /// An implementation whose pending items (array `+0x48`) are the grid points `points`, on the
    /// unit grid, ready for `flushPending`.
    fn implementation_with_pending(e: &mut Engine, points: &[[i32; 3]]) -> u32 {
        let (_, imp) = fresh_hull(e);
        e.mem.set_u8(imp + 0xc, 1);
        put_vec(e, imp + 0x100, [1.0, 1.0, 1.0, 0.0]);
        put_vec(e, imp + 0x110, [1.0, 1.0, 1.0, 0.0]);
        let items = e.mem.alloc(0x40 * points.len() as u32);
        for (index, p) in points.iter().enumerate() {
            let item = items + 0x40 * index as u32;
            put_vec(e, item + 0x10, [p[0] as f32, p[1] as f32, p[2] as f32, 1.0]);
            for axis in 0..3 {
                e.mem.set_i32(item + 0x20 + 4 * axis as u32, p[axis]);
            }
            e.mem.set_u32(item + 0x34, 0xffff_ffff);
        }
        e.mem.set_u32(imp + 0x48, items);
        e.mem.set_i32(imp + 0x4c, points.len() as i32);
        e.mem.set_u32(imp + 0x50, 0x8000_0000 | points.len() as u32);
        e.mem.set_i32(imp + 0x1a8, 3);
        imp
    }

    #[test]
    fn flush_pending_builds_the_solid_from_the_pending_points() {
        let (mut e, _) = bench();
        let imp = implementation_with_pending(
            &mut e,
            &[
                [0, 0, 0],
                [100, 0, 0],
                [0, 100, 0],
                [0, 0, 100],
                // Inside the tetrahedron: dropped.
                [10, 10, 10],
                // Outside it: a fifth vertex.
                [100, 100, 100],
            ],
        );
        let built = e.call(0x00d7_fb80, &args![imp]).bool();
        assert!(built);
        assert_eq!(e.mem.i32(imp + 0x28), 5);
        assert_eq!(e.mem.i32(imp + 0x40), 6);
        assert_links_are_symmetric(&e, imp);
        let mut grid: Vec<[i32; 3]> = vertex_nodes(&e, imp)
            .iter()
            .map(|n| {
                [
                    e.mem.i32(n + 0x20),
                    e.mem.i32(n + 0x24),
                    e.mem.i32(n + 0x28),
                ]
            })
            .collect();
        grid.sort_unstable();
        assert_eq!(
            grid,
            vec![
                [0, 0, 0],
                [0, 0, 100],
                [0, 100, 0],
                [100, 0, 0],
                [100, 100, 100]
            ]
        );
        // The pending array is empty afterwards.
        assert_eq!(e.mem.i32(imp + 0x4c), 0);
    }

    #[test]
    fn flush_pending_of_coplanar_points_finds_no_solid() {
        let (mut e, _) = bench();
        let imp = implementation_with_pending(
            &mut e,
            &[
                [0, 0, 0],
                [100, 0, 0],
                [0, 100, 0],
                [100, 100, 0],
                [50, 50, 0],
            ],
        );
        let built = e.call(0x00d7_fb80, &args![imp]).bool();
        assert!(!built);
        assert_eq!(e.mem.i32(imp + 0x28), 0);
        assert_eq!(e.mem.i32(imp + 0x40), 0);
    }

    #[test]
    fn flush_pending_with_fewer_than_four_points_is_an_internal_error() {
        let (mut e, _) = bench();
        let imp = implementation_with_pending(&mut e, &[[0, 0, 0], [100, 0, 0], [0, 100, 0]]);
        let log = install_report(&mut e, 0);
        e.call(0x00d7_fb80, &args![imp]);
        assert_eq!(log.borrow()[1].1[1], MESSAGE_INTERNAL_ERROR);
        assert_eq!(log.borrow()[2].1[1..3], [3, 0x642c_f968]);
        assert_eq!(log.borrow()[2].1[4..], [REPORT_FILE, 0x2f4]);
        install_report(&mut e, 1);
        let message = int3_message(&mut e, 0x00d7_fb80, &args![imp]);
        assert!(message.contains("INT3 at 00d7fbfb"), "{message}");
    }

    #[test]
    fn clear_horizon_removes_the_triangles_a_point_sees() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        // The grid coordinates of the corners are plus or minus 8191; a point far beyond the x
        // face sees the two triangles of that face only.
        let point = e.mem.alloc(0x30);
        e.mem.set_i32(point + 0x20, 20000);
        e.mem.set_i32(point + 0x24, 0);
        e.mem.set_i32(point + 0x28, 0);
        e.mem.set_u32(imp + 0x1ac, 1);
        let visible: Vec<u32> = triangle_nodes(&e, imp)
            .into_iter()
            .filter(|t| {
                (0..3).all(|c| {
                    let v = e.mem.u32(t + 8 + 4 * c);
                    e.mem.i32(v + 0x20) == 8191
                })
            })
            .collect();
        assert_eq!(visible.len(), 2);
        let edge = e.mem.alloc(8);
        e.mem.set_u32(edge, visible[0]);
        e.mem.set_u32(edge + 4, 0);
        let out = e.mem.alloc(8);
        e.call(0x00d7_c3e0, &args![imp, point, edge, out]);
        // Both triangles of the face are gone, and the horizon edge is on a remaining triangle.
        let remaining = triangle_nodes(&e, imp);
        assert_eq!(remaining.len(), 10);
        assert_eq!(e.mem.i32(imp + 0x40), 10);
        assert!(!remaining.contains(&visible[0]) && !remaining.contains(&visible[1]));
        assert!(remaining.contains(&e.mem.u32(out)));
        // The links of the remaining triangles to the removed ones were cleared.
        let open_links = remaining
            .iter()
            .map(|t| (0..3).filter(|c| e.mem.u32(t + 0x14 + 4 * c) == 0).count())
            .sum::<usize>();
        assert_eq!(open_links, 4);
        // A triangle already stamped (or none) ends the call at once.
        let before = e.mem.i32(imp + 0x40);
        e.mem.set_u32(edge, 0);
        e.call(0x00d7_c3e0, &args![imp, point, edge, out]);
        assert_eq!(e.mem.i32(imp + 0x40), before);
    }

    #[test]
    fn clear_horizon_stops_at_a_triangle_the_point_does_not_see() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let point = e.mem.alloc(0x30);
        // The centre of the cube is behind every triangle.
        e.mem.set_u32(imp + 0x1ac, 1);
        let first = triangle_nodes(&e, imp)[0];
        let edge = e.mem.alloc(8);
        e.mem.set_u32(edge, first);
        e.mem.set_u32(edge + 4, 2);
        let out = e.mem.alloc(8);
        e.call(0x00d7_c3e0, &args![imp, point, edge, out]);
        assert_eq!(e.mem.u32(out), first);
        assert_eq!(e.mem.u32(out + 4), 2);
        assert_eq!(e.mem.i32(imp + 0x40), 12);
        // The triangle is stamped as visited.
        assert_eq!(e.mem.u32(first + 0x40), 1);
    }

    #[test]
    fn ensure_containment_grows_the_hull_to_hold_the_points() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        e.call(0x00d7_b600, &args![imp]);
        let points = point_array(&mut e, &[[2.0, 0.5, 0.5], [0.5, 0.5, 0.5]]);
        e.call(0x00d8_27b0, &args![imp, points]);
        // The face away from the pivot moved out to the point: the box is [0, 2] along x.
        let mut low = [f32::MAX; 3];
        let mut high = [f32::MIN; 3];
        for p in vertex_positions(&e, imp) {
            for lane in 0..3 {
                low[lane] = low[lane].min(p[lane]);
                high[lane] = high[lane].max(p[lane]);
            }
        }
        assert!(
            (high[0] - 2.0).abs() < 2e-3 && low[0].abs() < 2e-3,
            "{low:?} {high:?}"
        );
        assert!((high[1] - 1.0).abs() < 2e-3 && (high[2] - 1.0).abs() < 2e-3);
        // The indices and the mass properties were rebuilt.
        assert_eq!(e.mem.u8(imp + 0x1b5), 1);
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        assert_close(e.mem.f32(imp + 0x1a0), 2.0);
        // Points that are inside change nothing.
        let (_, imp) = built_hull(&mut e, &CUBE);
        let inside = point_array(&mut e, &[[0.5, 0.5, 0.5]]);
        let nodes = vertex_nodes(&e, imp);
        e.call(0x00d8_27b0, &args![imp, inside]);
        assert_eq!(vertex_nodes(&e, imp), nodes);
    }

    #[test]
    fn simplify_decimates_removes_thin_triangles_and_merges_faces() {
        // Fewer than three dimensions: nothing happens.
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        let parameters = e.mem.alloc(0x20);
        e.mem.set_i32(parameters, 4);
        e.call(0x00d8_3f30, &args![imp, parameters]);
        assert_eq!(e.mem.i32(imp + 0x28), 2);
        // The target vertex count.
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &ICOSAHEDRON);
        let parameters = e.mem.alloc(0x20);
        e.mem.set_i32(parameters, 8);
        e.call(0x00d8_3f30, &args![imp, parameters]);
        assert_eq!(e.mem.i32(imp + 0x28), 8);
        // Thin triangles: a tolerance above the area of the smallest removes vertices.
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &ICOSAHEDRON);
        let parameters = e.mem.alloc(0x20);
        e.mem.set_f32(parameters + 8, 1.8);
        e.call(0x00d8_3f30, &args![imp, parameters]);
        let remaining = e.mem.i32(imp + 0x28);
        assert!(remaining < 12, "{remaining}");
        // Merging faces drops the vertices inside the faces.
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &bumped_cube());
        assert_eq!(e.mem.i32(imp + 0x28), 14);
        let parameters = e.mem.alloc(0x20);
        e.mem.set_u8(parameters + 0x10, 1);
        e.call(0x00d8_3f30, &args![imp, parameters]);
        assert_eq!(e.mem.i32(imp + 0x28), 8);
        assert_eq!(e.mem.i32(imp + 0x1a8), 3);
        assert_eq!(e.mem.i32(imp + 0x58), 6);
    }

    #[test]
    fn simplify_reports_a_missing_index_and_other_dimension_counts() {
        let (mut e, _) = bench();
        let (_, imp) = built_hull(&mut e, &CUBE);
        let parameters = e.mem.alloc(0x20);
        e.mem.set_u8(parameters + 0x10, 1);
        e.mem.set_u8(imp + 0x1b5, 0);
        install_report(&mut e, 1);
        let message = int3_message(&mut e, 0x00d8_3f30, &args![imp, parameters]);
        assert!(message.contains("INT3 at 00d84313"), "{message}");
        // No dimensions at all is the "not implemented" report.
        e.mem.set_i32(imp + 0x1a8, -1);
        let message = int3_message(&mut e, 0x00d8_3f30, &args![imp, parameters]);
        assert!(message.contains("INT3 at 00d84745"), "{message}");
    }

    #[test]
    fn expanding_a_solid_moves_its_vertices_out_along_the_planes() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &CUBE);
        e.call(0x00d7_62f0, &args![hull]);
        let solid = e.call(0x00d7_7be0, &args![hull, 0.2f32.to_bits()]).bool();
        assert!(solid);
        assert_box(&e, hull, [-0.2; 3], [1.2; 3]);
        assert_eq!(e.mem.i32(imp + 0x1a8), 3);
        // The indices and the mass properties existed and are rebuilt.
        assert_eq!(e.mem.u8(imp + 0x1b5), 1);
        assert_eq!(e.mem.u8(imp + 0x1b4), 1);
        assert!(e.mem.f32(imp + 0x1a0) > 1.5);
    }

    #[test]
    fn expanding_a_polygon_makes_a_prism() {
        let (mut e, _) = bench();
        let (hull, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let at = aligned_points(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [2.0, 2.0, 0.0],
                [0.0, 2.0, 0.0],
            ],
        );
        e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]);
        let solid = e.call(0x00d7_7be0, &args![hull, 0.5f32.to_bits()]).bool();
        assert!(solid);
        assert_eq!(e.mem.i32(imp + 0x1a8), 3);
        assert_box(&e, hull, [0.0, 0.0, -0.5], [2.0, 2.0, 0.5]);
        // A margin of nothing leaves the points in their plane: the hull falls back to its
        // centre point and expands that, which is no solid either.
        let (hull, imp) = fresh_hull(&mut e);
        e.mem.set_u8(imp, 1);
        let at = aligned_points(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [2.0, 2.0, 0.0],
                [0.0, 2.0, 0.0],
            ],
        );
        e.call(0x00d8_1510, &args![imp, at, 4i32, 0u32, 0u32]);
        assert!(!e.call(0x00d7_7be0, &args![hull, 0.0f32.to_bits()]).bool());
    }

    #[test]
    fn expanding_a_segment_or_a_point_makes_a_small_solid() {
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &[[0.0, 0.0, 0.0], [4.0, 0.0, 0.0]]);
        assert!(e.call(0x00d7_7be0, &args![hull, 1.0f32.to_bits()]).bool());
        assert_eq!(e.mem.i32(imp + 0x1a8), 3);
        // The ends are kept and the sides reach half the margin out.
        let (low, high) = hull_box(&e, hull);
        assert!(
            low[0].abs() < 2e-3 && (high[0] - 4.0).abs() < 2e-3,
            "{low:?} {high:?}"
        );
        assert!(high[1] > 0.4 && high[1] < 0.51, "{high:?}");
        assert!(high[2] > 0.4 && high[2] < 0.51, "{high:?}");
        let (mut e, _) = bench();
        let (hull, imp) = built_hull(&mut e, &[[1.0, 2.0, 3.0]]);
        assert!(e.call(0x00d7_7be0, &args![hull, 1.0f32.to_bits()]).bool());
        assert_eq!(e.mem.i32(imp + 0x1a8), 3);
        assert_box(&e, hull, [0.5, 1.5, 2.5], [1.5, 2.5, 3.5]);
        assert_eq!(e.mem.i32(imp + 0x28), 6);
    }

    #[test]
    fn expanding_an_empty_hull_reports_the_dimension() {
        let (mut e, _) = bench();
        let (hull, _) = fresh_hull(&mut e);
        let log = install_report(&mut e, 0);
        e.call(0x00d7_7be0, &args![hull, 1.0f32.to_bits()]);
        assert_eq!(log.borrow()[1].1[1], MESSAGE_INVALID_DIMENSIONS);
        assert_eq!(log.borrow()[2].1[1..3], [3, 0x172c_df48]);
        assert_eq!(log.borrow()[2].1[4..], [REPORT_FILE, 0xa77]);
        install_report(&mut e, 1);
        let message = int3_message(&mut e, 0x00d7_7be0, &args![hull, 1.0f32.to_bits()]);
        assert!(message.contains("INT3 at 00d786b5"), "{message}");
    }
}
