//! `BSHavok/bhktrisampledheightfieldbvtreeshape.obj` (Xbox PDB source unit), subsystem
//! `BSHavok`: its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkTriSampledHeightFieldBvTreeShape` (vtable `0104dac4`, RTTI-confirmed) is a
//! `bhkBvTreeShape` (vtable `0104dbac`) and a `bhkShape` (0x14 bytes): its Havok shape is at
//! `+8` and its construction info at `+0xC`. `Init` builds three Havok objects: the
//! `hkBSHeightFieldShape` (vtable `010c853c`, RTTI-confirmed, 0x70 bytes), the sampled
//! collection around it (0x40 bytes) and the bounding-volume tree shape around that (0x30
//! bytes). The construction info is 0x50 bytes, with the height field's `hkpSampledHeightField`
//! construction values at `+0x10` to `+0x2F` and the extents at `+0x40`/`+0x44`.
//!
//! The functions that allocate or release Havok objects raise the per-thread allocation word
//! at `TLS + 0x2b4` to 0x10 for their duration. The exception-unwinding frames (`FS:[0]`) and
//! `/GS` cookie checks of the functions marked `Translation incomplete` are not modeled. The
//! 16-byte aligned vector moves (`MOVAPS`) are done as plain byte copies; their alignment
//! faults are not modeled. `RSQRTSS` is the host CPU's estimate (`rsqrt_estimate`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{
    add_viewer_string, allocate_havok, enter_scope, leave_scope,
};
use crate::units::bshavok::bhklimitedhingeconstraint::allocator;
use crate::x87::rsqrt_estimate;

/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The base constructor the object constructor starts with (`00c87c30`).
const BASE_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable` methods: `Kill`, the clone lock and unlock.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// The base class's methods (`00ca4xxx`, the `bhkBvTreeShape` family): destructor body
/// (`ECX`), `CopyCinfo` `(cinfo)`, `CopyMembers` `(source, process)`, `LoadBinary`
/// `(stream)`, `SaveBinary` `(stream)` and `GetViewerStrings` `(list)`.
const BASE_DESTRUCT: u32 = 0x00ca_4da0;
const BASE_COPY_CINFO: u32 = 0x00ca_4e10;
const BASE_COPY_MEMBERS: u32 = 0x00ca_4e50;
const BASE_LOAD_BINARY: u32 = 0x00ca_4ec0;
const BASE_SAVE_BINARY: u32 = 0x00ca_4f80;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00ca_5190;
/// `00d26450` (`ECX` = the Havok shape's child at `+0x18`): answers the child's description,
/// which `CopyCinfo` reads its values from.
const CHILD_SOURCE: u32 = 0x00d2_6450;
/// The construction info constructor (`00622490`, `ECX` = the block).
const CINFO_CONSTRUCT: u32 = 0x0062_2490;
/// `hkReferencedObject::removeReference` (`00c905b0`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The Havok constructors `Init` uses, all with `ECX` = the block: the height field
/// `(source, 2)` (`00d248a0`), the sampled collection `(height field, float)` (`00d26700`)
/// and the bounding-volume tree shape `(collection, flag)` (`00d26b50`).
const HK_HEIGHT_FIELD_CONSTRUCT: u32 = 0x00d2_48a0;
const HK_COLLECTION_CONSTRUCT: u32 = 0x00d2_6700;
const HK_TREE_CONSTRUCT: u32 = 0x00d2_6b50;
/// `00ec62c0`: the truncating float to integer helper, the value passed in `ST0`.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// `NiGetViewerString` (`00a7d330`, cdecl).
const VIEWER_STRING: u32 = 0x00a7_d330;
/// The vtables of the classes the constructor passes through (RTTI-confirmed) and their
/// live-object counts.
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_BV_TREE_SHAPE: u32 = 0x0104_dbac;
const VTABLE_TRI_SAMPLED_HEIGHT_FIELD: u32 = 0x0104_dac4;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_BV_TREE_SHAPE: u32 = 0x0126_82cc;
const COUNT_TRI_SAMPLED_HEIGHT_FIELD: u32 = 0x0126_834c;
/// The `hkBSHeightFieldShape` vtable.
const VTABLE_HK_HEIGHT_FIELD: u32 = 0x010c_853c;
/// The global holding the text `GetViewerStrings` starts with (read as a word).
const CLASS_TEXT: u32 = 0x0126_8350;
/// Constants of `Init` and `collideSpheres`: the float the collection is built with (`0.5`
/// in this build), the double `1.0`, the zero, one and large floats of the default result,
/// the float magic bias (a vector of four) the sample coordinates are packed with, and the
/// `3.0` and `0.5` of the reciprocal square root refinement.
const COLLECTION_FLOAT: u32 = 0x0101_6248;
const ONE_DOUBLE: u32 = 0x0101_2070;
const ZERO: u32 = 0x0101_1d78;
const ONE: u32 = 0x0101_2068;
const LARGE: u32 = 0x0104_da48;
const BIAS: u32 = 0x011b_1bd0;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;

type Vec4 = [u32; 4];

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// The construction info of `this`: virtual slot `0xB4` fills a flag byte and answers it.
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32()
}

/// Byte-swaps the eight words at `cinfo + 0x10` in place and clears `cinfo + 0x40`.
fn swap_words(e: &mut Engine, cinfo: u32) {
    for offset in (0x10..0x30).step_by(4) {
        let word = e.mem.u32(cinfo + offset);
        e.mem.set_u32(cinfo + offset, word.swap_bytes());
    }
    e.mem.set_u32(cinfo + 0x40, 0);
}

/// The constructor `00cb05d0` has inline: the base constructor, `+0xC` and `+0x10` cleared,
/// then the `bhkShape`, `bhkBvTreeShape` and `bhkTriSampledHeightFieldBvTreeShape` vtables,
/// each with its live-object count raised.
fn construct(e: &mut Engine, block: u32) {
    e.call(BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, VTABLE_SHAPE);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0);
    increment_count(e, COUNT_SHAPE);
    e.mem.set_u32(block, VTABLE_BV_TREE_SHAPE);
    increment_count(e, COUNT_BV_TREE_SHAPE);
    e.mem.set_u32(block, VTABLE_TRI_SAMPLED_HEIGHT_FIELD);
    increment_count(e, COUNT_TRI_SAMPLED_HEIGHT_FIELD);
}

/// A value stored to a float slot and loaded back.
fn x87_round32(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// One lane of an SSE arithmetic instruction: a NaN in the destination operand wins
/// (quieted), then a NaN in the source operand, otherwise `op`.
fn sse(dest: u32, source: u32, op: impl Fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

fn sse_add(dest: u32, source: u32) -> u32 {
    sse(dest, source, |a, b| a + b)
}

fn sse_sub(dest: u32, source: u32) -> u32 {
    sse(dest, source, |a, b| a - b)
}

fn sse_mul(dest: u32, source: u32) -> u32 {
    sse(dest, source, |a, b| a * b)
}

/// A packed instruction on every lane.
fn lanes(dest: Vec4, source: Vec4, op: fn(u32, u32) -> u32) -> Vec4 {
    [
        op(dest[0], source[0]),
        op(dest[1], source[1]),
        op(dest[2], source[2]),
        op(dest[3], source[3]),
    ]
}

fn read_vec4(e: &Engine, address: u32) -> Vec4 {
    [
        e.mem.u32(address),
        e.mem.u32(address + 4),
        e.mem.u32(address + 8),
        e.mem.u32(address + 12),
    ]
}

fn write_vec4(e: &mut Engine, address: u32, value: Vec4) {
    for (index, lane) in value.into_iter().enumerate() {
        e.mem.set_u32(address + 4 * index as u32, lane);
    }
}

/// The height at the grid point `(x, z)`: virtual slot `0x34`, answering in `ST0`, stored
/// as a float by the caller. Answered as the stored bits.
fn corner(e: &mut Engine, shape: u32, x: i32, z: i32) -> u32 {
    x87_store32(e.vcall(shape, 0x34, &args![x as u32, z as u32]).f64())
}

// Translated from 00caff80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caff80 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriSampledHeightFieldBvTreeShape::~bhkTriSampledHeightFieldBvTreeShape` (Xbox PDB):
/// sets the class vtable, runs `bhkSerializable::Kill`, decrements the live-object count at
/// `0126834c` and runs the base destructor body (`00ca4da0`).
pub fn bhk_tri_sampled_height_field_bv_tree_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_TRI_SAMPLED_HEIGHT_FIELD);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(COUNT_TRI_SAMPLED_HEIGHT_FIELD);
    e.set_global(COUNT_TRI_SAMPLED_HEIGHT_FIELD, count.wrapping_sub(1));
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00cafff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriSampledHeightFieldBvTreeShape::CopyMembers` (Xbox PDB), `(this, source, process)`:
/// asks for the cinfo (virtual slot `0xB4`, which makes one when there is none; the answer is
/// not used), then runs the base `CopyMembers` (`00ca4e50`) with `(source, process)`.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_copy_members(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    process: u32,
) {
    cinfo_of(e, this);
    e.call(BASE_COPY_MEMBERS, &args![this, source, process]);
}

// Translated from 00cb0020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the byte at `flag` (one stack word; `ECX` unused) and answers `flag`
/// (it is still in `EAX` from the load).
/// `this` (`ECX`) is not read, but it sits in vtables and is called with `ECX` set.
pub fn fn_00cb0020(e: &mut Engine, _this: Ptr, flag: Ptr) -> Ptr {
    e.mem.set_u8(flag.addr(), 0);
    flag
}

// Translated from 00cb0030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriSampledHeightFieldBvTreeShape::CopyCinfo` (Xbox PDB): the base `CopyCinfo`
/// (`00ca4e10`), then, when the shape has a Havok shape whose child (`+0x18`) gives a
/// description (`00d26450`): the cinfo's `+0x2C` is that description's `+0x58` plus its
/// `+0x18` and the cinfo's `+0x28` its `+0x18` minus its `+0x58` (through the x87 stack),
/// the 16 bytes at `+0x20` go to the cinfo's `+0x10`, the words at `+0x10` and `+0x14` to the
/// cinfo's `+0x20` and `+0x24`, and the word at `+0x60` to the cinfo's `+0x40`.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_copy_cinfo(
    e: &mut Engine,
    this: Ptr,
    cinfo: u32,
) {
    e.call(BASE_COPY_CINFO, &args![this, cinfo]);
    let shape = e.mem.u32(this.addr() + 8);
    if shape == 0 {
        return;
    }
    let child = e.mem.u32(shape + 0x18);
    if child == 0 {
        return;
    }
    let source = e.call(CHILD_SOURCE, &args![child]).u32();
    if source == 0 {
        return;
    }
    let upper = x87_load32(e.mem.u32(source + 0x58));
    let lower = x87_load32(e.mem.u32(source + 0x18));
    e.mem
        .set_u32(cinfo + 0x2c, x87_store32(x87_add(upper, lower)));
    let lower = x87_load32(e.mem.u32(source + 0x18));
    let upper = x87_load32(e.mem.u32(source + 0x58));
    e.mem
        .set_u32(cinfo + 0x28, x87_store32(x87_sub(lower, upper)));
    let row = e.mem.bytes(source + 0x20, 16);
    e.mem.write(cinfo + 0x10, &row);
    let word = e.mem.u32(source + 0x10);
    e.mem.set_u32(cinfo + 0x20, word);
    let word = e.mem.u32(source + 0x14);
    e.mem.set_u32(cinfo + 0x24, word);
    let word = e.mem.u32(source + 0x60);
    e.mem.set_u32(cinfo + 0x40, word);
}

// Translated from 00cb00a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriSampledHeightFieldBvTreeShape::LoadBinary` (Xbox PDB): the base `LoadBinary`
/// (`00ca4ec0`), then, when the stream's byte at `+0x1E5` is not 1, takes the cinfo,
/// byte-swaps its eight words at `+0x10` to `+0x2F` and clears its `+0x40`.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_load_binary(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let cinfo = cinfo_of(e, this);
        swap_words(e, cinfo);
    }
}

// Translated from 00cb0120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriSampledHeightFieldBvTreeShape::SaveBinary` (Xbox PDB): takes the cinfo, swaps its
/// eight words at `+0x10` and clears its `+0x40` when the stream's byte at `+0x1E4` is not
/// 1, runs the base `SaveBinary` (`00ca4f80`) and swaps and clears again under the same
/// condition.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_save_binary(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
) {
    let cinfo = cinfo_of(e, this);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_words(e, cinfo);
    }
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_words(e, cinfo);
    }
}

// Translated from 00cb01f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb01f0 - the /GS cookie check is not modeled.
/// `hkBSHeightFieldShape::collideSpheres` (Xbox PDB), `this` = the height field,
/// `(input, output)`: `input` holds a pointer to the spheres (4 floats each) and their count,
/// `output` receives 4 floats per sphere: the contact normal and the signed distance.
///
/// Each result starts as `(0, 1, 0, 3.4e38)`. A sphere's position times the field's scale
/// (`this + 0x30`), plus the field's offset (`+0x40`) times the scale, plus a magic bias,
/// packs the grid cell (`x` from lane 0, `z` from lane 2, bits 6 to 21 of the float); a
/// sphere outside the grid (index not below the width `+0x10` or depth `+0x14` minus one)
/// keeps the default. Otherwise the cell fractions `fx`, `fz` come from the scaled position,
/// virtual slot `0x38` fills a flag byte, and the corner heights come from virtual slot
/// `0x34` `(x, z)` (answered in `ST0`) on the triangle half the flag and fractions select;
/// the two slopes make the plane's normal `(-slope_x, 1, -slope_z)`, scaled by the field's
/// `+0x30` and normalized with the `RSQRTSS` estimate refined once (zero for a zero length),
/// and the fourth lane is the distance: with the byte at `this + 0x1C` zero
/// `(scaled y - height) * this[0x24] - sphere[3]`, otherwise the dot product of the scaled
/// position minus the corner `(x, corner height, z)` with `this[+0x20..]` and the normal,
/// minus `sphere[3]`.
pub fn hk_bs_height_field_shape_collide_spheres(
    e: &mut Engine,
    this: Ptr,
    input: u32,
    output: u32,
) {
    let shape = this.addr();
    let zero = e.mem.u32(ZERO);
    let default = [zero, e.mem.u32(ONE), zero, e.mem.u32(LARGE)];
    let mut sphere = e.mem.u32(input);
    let mut remaining = (e.mem.u32(input + 4) as i32).wrapping_sub(1);
    let mut result = output;
    while remaining >= 0 {
        write_vec4(e, result, default);
        let scale = read_vec4(e, shape + 0x30);
        let position = read_vec4(e, sphere);
        let scaled = lanes(position, scale, sse_mul);
        let offset = read_vec4(e, shape + 0x40);
        let shifted = lanes(lanes(position, offset, sse_add), scale, sse_mul);
        let bias = read_vec4(e, BIAS);
        let packed = lanes(shifted, bias, sse_add);
        let x_index = ((packed[0] >> 6) & 0xffff) as i32;
        let z_index = ((packed[2] >> 6) & 0xffff) as i32;
        let width = (e.mem.u32(shape + 0x10) as i32).wrapping_sub(1);
        let depth = (e.mem.u32(shape + 0x14) as i32).wrapping_sub(1);
        if x_index < width && z_index < depth {
            let fraction_x = x87_store32(x87_sub(x87_load32(scaled[0]), f64::from(x_index)));
            let fraction_z = x87_store32(x87_sub(x87_load32(scaled[2]), f64::from(z_index)));
            let flag = e.with_stack(4, |e, byte| {
                e.vcall(shape, 0x38, &args![byte]);
                e.mem.u8(byte.addr())
            });
            let fx = x87_load32(fraction_x);
            let fz = x87_load32(fraction_z);
            // The slopes along x and z, the plane height, the grid x the plane is taken
            // from and the height word it is taken at.
            let (slope_x, slope_z, height, plane_x, plane_y) = if flag != 0 {
                let h00 = corner(e, shape, x_index, z_index);
                let h11 = corner(e, shape, x_index + 1, z_index + 1);
                let (low, high) = (x87_load32(h00), x87_load32(h11));
                if fz < fx {
                    let h10 = x87_load32(corner(e, shape, x_index + 1, z_index));
                    let slope_x = x87_round32(x87_sub(h10, low));
                    let slope_z = x87_round32(x87_sub(high, h10));
                    let base = x87_add(low, x87_mul(slope_z, fz));
                    (
                        slope_x,
                        slope_z,
                        x87_add(base, x87_mul(slope_x, fx)),
                        x_index,
                        h00,
                    )
                } else {
                    let h01 = x87_load32(corner(e, shape, x_index, z_index + 1));
                    let slope_x = x87_round32(x87_sub(high, h01));
                    let slope_z = x87_round32(x87_sub(h01, low));
                    let base = x87_add(low, x87_mul(slope_z, fz));
                    (
                        slope_x,
                        slope_z,
                        x87_add(base, x87_mul(slope_x, fx)),
                        x_index,
                        h00,
                    )
                }
            } else {
                let h10_bits = corner(e, shape, x_index + 1, z_index);
                let h10 = x87_load32(h10_bits);
                let h01 = x87_load32(corner(e, shape, x_index, z_index + 1));
                let one = e.global::<f64>(ONE_DOUBLE);
                if x87_add(fz, fx) > one {
                    let h11 = x87_load32(corner(e, shape, x_index + 1, z_index + 1));
                    let slope_x = x87_round32(x87_sub(h11, h01));
                    let slope_z = x87_round32(x87_sub(h11, h10));
                    let base = x87_add(h10, x87_mul(slope_z, fz));
                    let along = x87_mul(x87_sub(fx, one), slope_x);
                    (
                        slope_x,
                        slope_z,
                        x87_add(along, base),
                        x_index + 1,
                        h10_bits,
                    )
                } else {
                    let h00 = x87_load32(corner(e, shape, x_index, z_index));
                    let slope_x = x87_round32(x87_sub(h10, h00));
                    let slope_z = x87_round32(x87_sub(h01, h00));
                    let base = x87_add(h00, x87_mul(slope_z, fz));
                    (
                        slope_x,
                        slope_z,
                        x87_add(base, x87_mul(slope_x, fx)),
                        x_index + 1,
                        h10_bits,
                    )
                }
            };
            let height_bits = x87_store32(height);
            e.mem.set_u32(result, ((-slope_x) as f32).to_bits());
            e.mem.set_u32(result + 8, ((-slope_z) as f32).to_bits());
            let slope = read_vec4(e, result);
            let weighted = lanes(scale, slope, sse_mul);
            let squares = lanes(weighted, weighted, sse_mul);
            let length = sse_add(squares[2], sse_add(squares[1], squares[0]));
            let estimate = rsqrt_estimate(f32::from_bits(length)).to_bits();
            let refined = sse_sub(
                e.mem.u32(THREE),
                sse_mul(sse_mul(length, estimate), estimate),
            );
            let inverse = sse_mul(sse_mul(e.mem.u32(HALF), estimate), refined);
            let factor = if f32::from_bits(length) == f32::from_bits(zero) {
                0
            } else {
                inverse
            };
            let normal = lanes([factor; 4], weighted, sse_mul);
            write_vec4(e, result, normal);
            let distance = if e.mem.u8(shape + 0x1c) == 0 {
                let above = x87_sub(x87_load32(scaled[1]), x87_load32(height_bits));
                let along = x87_mul(above, x87_load32(e.mem.u32(shape + 0x24)));
                x87_sub(along, x87_load32(e.mem.u32(sphere + 0xc)))
            } else {
                let point = [
                    (plane_x as f32).to_bits(),
                    plane_y,
                    (z_index as f32).to_bits(),
                    zero,
                ];
                let relative = lanes(scaled, point, sse_sub);
                let weights = read_vec4(e, shape + 0x20);
                let along = lanes(lanes(relative, weights, sse_mul), normal, sse_mul);
                let sum = sse_add(along[2], sse_add(along[1], along[0]));
                x87_sub(x87_load32(sum), x87_load32(e.mem.u32(sphere + 0xc)))
            };
            e.mem.set_u32(result + 0xc, x87_store32(distance));
        }
        sphere = sphere.wrapping_add(0x10);
        result = result.wrapping_add(0x10);
        remaining = remaining.wrapping_sub(1);
    }
}

// Translated from 00cb05d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb05d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriSampledHeightFieldBvTreeShape::CreateClone` (Xbox PDB): in the Havok allocation
/// scope and between the clone lock and unlock (`00c876f0`, `00c87700`), makes a new
/// 0x14-byte shape (`operator new` and the inline constructor; null stays null) and calls
/// this shape's virtual slot `0xC4` with `(clone, process)`. Answers the clone.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_create_clone(
    e: &mut Engine,
    this: Ptr,
    process: u32,
) -> Ptr {
    let old = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block != 0 {
        construct(e, block);
    }
    e.vcall(this.addr(), 0xc4, &args![block, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, old);
    Ptr::new(block)
}

// Translated from 00cb06c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb06c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriSampledHeightFieldBvTreeShape::CreateCinfo` (Xbox PDB), `(this, created)`: in the
/// Havok allocation scope stores into the byte at `created` whether the shape had no cinfo
/// (`+0xC` null). In that case it takes 0x50 bytes from the memory router's allocator (virtual
/// slot 4; no size word), constructs the cinfo in it (`00622490`; a failed allocation leaves
/// null), stores it at `+0xC` and, when the shape has a Havok shape, fills it with
/// `CopyCinfo` (`00cb0030`). Answers the cinfo at `+0xC`.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_create_cinfo(
    e: &mut Engine,
    this: Ptr,
    created: Ptr,
) -> u32 {
    let old = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let heap = allocator(e);
        let block = e.vcall(heap, 4, &args![0x50u32]).u32();
        let cinfo = if block != 0 {
            e.call(CINFO_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        let has_shape = e.mem.u32(this.addr() + 8) != 0;
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if has_shape {
            bhk_tri_sampled_height_field_bv_tree_shape_copy_cinfo(e, this, cinfo);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    leave_scope(e, old);
    cinfo
}

// Translated from 00cb0780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb0780 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriSampledHeightFieldBvTreeShape::KillCinfo` (Xbox PDB), `(this, flag)` with only the
/// low byte of `flag` used: in the Havok allocation scope, when that byte is not 0 gives the
/// cinfo at `+0xC` (when there is one) back to the memory router's allocator (virtual slot
/// 8, `(pointer, 0x30)`: the code frees 0x30 bytes of the 0x50 it allocated) and clears
/// `+0xC`.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_kill_cinfo(e: &mut Engine, this: Ptr, flag: u32) {
    let old = enter_scope(e);
    if flag as u8 != 0 {
        let cinfo = e.mem.u32(this.addr() + 0xc);
        if cinfo != 0 {
            let heap = allocator(e);
            e.vcall(heap, 8, &args![cinfo, 0x30u32]);
        }
        e.mem.set_u32(this.addr() + 0xc, 0);
    }
    leave_scope(e, old);
}

// Translated from 00cb0820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb0820 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriSampledHeightFieldBvTreeShape::Init` (Xbox PDB): in the Havok allocation scope
/// builds the `hkBSHeightFieldShape` in a 0x70-byte block (`00d248a0` with the cinfo's
/// `+0x10` address, 0 for no cinfo, and 2), puts its vtable in, copies the cinfo's word at
/// `+0x40` to `+0x60`, stores the truncation (`00ec62c0`) of its `+0x14` minus 1.0 at `+0x64`
/// and the cinfo's float at `+0x44` at `+0x68`; then builds the sampled collection in a
/// 0x40-byte block (`00d26700` with the height field and the float at `01016248`) and the
/// tree shape in a 0x30-byte block (`00d26b50` with the collection and 0). Hands the tree
/// shape to virtual slot `0x8C`, releases all three (`00c905b0`: tree shape, collection,
/// height field) and calls virtual slot `0xBC` with the cinfo.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_init(e: &mut Engine, this: Ptr, cinfo: u32) {
    let old = enter_scope(e);
    let field = allocate_havok(e, 0x70);
    let source = if cinfo != 0 { cinfo + 0x10 } else { 0 };
    e.call(HK_HEIGHT_FIELD_CONSTRUCT, &args![field, source, 2u32]);
    let rows = f64::from(e.mem.i32(field + 0x14));
    e.mem.set_u32(field, VTABLE_HK_HEIGHT_FIELD);
    let word = e.mem.u32(source + 0x30);
    let rows_less_one = rows - e.global::<f64>(ONE_DOUBLE);
    e.mem.set_u32(field + 0x60, word);
    let truncated = e.call(FLOAT_TO_INT, &args![rows_less_one]).u32();
    e.mem.set_u32(field + 0x64, truncated);
    let extent = x87_quiet_bits(e.mem.u32(source + 0x34));
    e.mem.set_u32(field + 0x68, extent);
    let collection_block = allocate_havok(e, 0x40);
    let collection_float = x87_quiet_bits(e.mem.u32(COLLECTION_FLOAT));
    let collection = e
        .call(
            HK_COLLECTION_CONSTRUCT,
            &args![collection_block, field, collection_float],
        )
        .u32();
    let tree_block = allocate_havok(e, 0x30);
    let tree = e
        .call(HK_TREE_CONSTRUCT, &args![tree_block, collection, 0u32])
        .u32();
    e.vcall(this.addr(), 0x8c, &args![tree]);
    e.call(REMOVE_REFERENCE, &args![tree]);
    e.call(REMOVE_REFERENCE, &args![collection]);
    e.call(REMOVE_REFERENCE, &args![field]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    leave_scope(e, old);
}

// Translated from 00cb09b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb09b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriSampledHeightFieldBvTreeShape::GetViewerStrings` (Xbox PDB): in the Havok
/// allocation scope runs the base `GetViewerStrings` (`00ca5190`) and appends the viewer
/// string of the text in the global at `01268350`.
pub fn bhk_tri_sampled_height_field_bv_tree_shape_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    list: u32,
) {
    let old = enter_scope(e);
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, list]);
    let class_text = e.global::<u32>(CLASS_TEXT);
    let text = e.call(VIEWER_STRING, &args![class_text]).u32();
    add_viewer_string(e, list, text);
    leave_scope(e, old);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00caff80,
            bhk_tri_sampled_height_field_bv_tree_shape_destructor(Ptr)
        ),
        entry!(
            0x00cafff0,
            bhk_tri_sampled_height_field_bv_tree_shape_copy_members(Ptr, u32, u32)
        ),
        entry!(0x00cb0020, fn_00cb0020(Ptr, Ptr) -> Ptr),
        entry!(
            0x00cb0030,
            bhk_tri_sampled_height_field_bv_tree_shape_copy_cinfo(Ptr, u32)
        ),
        entry!(
            0x00cb00a0,
            bhk_tri_sampled_height_field_bv_tree_shape_load_binary(Ptr, u32)
        ),
        entry!(
            0x00cb0120,
            bhk_tri_sampled_height_field_bv_tree_shape_save_binary(Ptr, u32)
        ),
        entry!(
            0x00cb01f0,
            hk_bs_height_field_shape_collide_spheres(Ptr, u32, u32)
        ),
        entry!(
            0x00cb05d0,
            bhk_tri_sampled_height_field_bv_tree_shape_create_clone(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cb06c0,
            bhk_tri_sampled_height_field_bv_tree_shape_create_cinfo(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00cb0780,
            bhk_tri_sampled_height_field_bv_tree_shape_kill_cinfo(Ptr, u32)
        ),
        entry!(
            0x00cb0820,
            bhk_tri_sampled_height_field_bv_tree_shape_init(Ptr, u32)
        ),
        entry!(
            0x00cb09b0,
            bhk_tri_sampled_height_field_bv_tree_shape_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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

    /// A shape (slot `0xB4` answers its `+0xC`, slot `0xC4` records) with a Havok shape block.
    /// Returns `(this, havok shape)`.
    fn shape(e: &mut Engine) -> (u32, u32) {
        let this = object(e, &[(0xb4, 0x0900_0100), (0xc4, 0x0900_0101)]);
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        quiet(e, &[0x0900_0101]);
        let havok = e.mem.alloc(0x80);
        e.mem.set_u32(this + 8, havok);
        (this, havok)
    }

    /// Maps the pages the tests read constants and counters from.
    fn map_globals(e: &mut Engine) {
        e.map(0x0126_8000, 0x1000);
        e.map(0x0101_1000, 0x7000);
        e.map(0x0104_d000, 0x1000);
        e.map(0x011b_1000, 0x1000);
    }

    #[test]
    fn destructor_sets_the_vtable_kills_counts_down_and_destructs() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_TRI_SAMPLED_HEIGHT_FIELD, 9u32);
        quiet(&mut e, &[SERIALIZABLE_KILL, BASE_DESTRUCT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00ca_ff80, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_TRI_SAMPLED_HEIGHT_FIELD);
        assert_eq!(e.global::<u32>(COUNT_TRI_SAMPLED_HEIGHT_FIELD), 8);
        assert_eq!(
            order(&e),
            vec![0x00ca_ff80, SERIALIZABLE_KILL, BASE_DESTRUCT]
        );
    }

    #[test]
    fn copy_members_asks_for_the_cinfo_then_runs_the_base() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_COPY_MEMBERS]);
        let (this, _) = shape(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00ca_fff0, &args![this, 0x9999u32, 0x7777u32]);
        assert_eq!(order(&e), vec![0x00ca_fff0, 0x0900_0100, BASE_COPY_MEMBERS]);
        assert_eq!(calls_to(&e, 0x0900_0100).len(), 1);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this, 0x9999, 0x7777]]
        );
    }

    #[test]
    fn the_flag_function_clears_the_byte() {
        let mut e = Engine::new();
        let byte = e.mem.alloc(4);
        e.mem.set_u8(byte, 5);
        assert_eq!(e.call(0x00cb_0020, &args![0x5555u32, byte]).u32(), byte);
        assert_eq!(e.mem.u8(byte), 0);
    }

    #[test]
    fn copy_cinfo_takes_the_values_of_the_child_description() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_COPY_CINFO]);
        let source = e.mem.alloc(0x80);
        e.mem.set_f32(source + 0x58, 2.0);
        e.mem.set_f32(source + 0x18, 5.0);
        for i in 0..4u32 {
            e.mem.set_u32(source + 0x20 + 4 * i, 0x500 + i);
        }
        e.mem.set_u32(source + 0x10, 0x611);
        e.mem.set_u32(source + 0x14, 0x612);
        e.mem.set_u32(source + 0x60, 0x613);
        e.register_double(CHILD_SOURCE, move |_, _| Ret {
            eax: source,
            ..Ret::default()
        });
        let (this, havok) = shape(&mut e);
        e.mem.set_u32(havok + 0x18, 0xc41d);
        let cinfo = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x00cb_0030, &args![this, cinfo]);
        assert_eq!(calls_to(&e, BASE_COPY_CINFO), vec![vec![this, cinfo]]);
        assert_eq!(calls_to(&e, CHILD_SOURCE), vec![vec![0xc41d]]);
        assert_eq!(e.mem.f32(cinfo + 0x2c), 7.0);
        assert_eq!(e.mem.f32(cinfo + 0x28), 3.0);
        for i in 0..4u32 {
            assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * i), 0x500 + i);
        }
        assert_eq!(e.mem.u32(cinfo + 0x20), 0x611);
        assert_eq!(e.mem.u32(cinfo + 0x24), 0x612);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x613);
        // A NaN in the description comes out quiet, the sum's NaN winning.
        e.mem.set_u32(source + 0x58, 0x7f80_0001);
        e.call(0x00cb_0030, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 0x2c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(cinfo + 0x28), 0x7fc0_0001);
        // Without a description, a child or a Havok shape only the base runs.
        let untouched = e.mem.alloc(0x60);
        e.register(CHILD_SOURCE, |_, _| Ret::default());
        e.call(0x00cb_0030, &args![this, untouched]);
        e.mem.set_u32(havok + 0x18, 0);
        e.call(0x00cb_0030, &args![this, untouched]);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00cb_0030, &args![this, untouched]);
        assert_eq!(e.mem.u32(untouched + 0x2c), 0);
        assert_eq!(calls_to(&e, BASE_COPY_CINFO).len(), 5);
    }

    fn cinfo_with_words(e: &mut Engine, this: u32) -> u32 {
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0xc, cinfo);
        for i in 0..8 {
            e.mem.set_u32(cinfo + 0x10 + 4 * i, 0x0102_0300 + i);
        }
        e.mem.set_u32(cinfo + 0x40, 0x55);
        cinfo
    }

    #[test]
    fn load_binary_swaps_the_words_unless_the_stream_byte_is_one() {
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_LOAD_BINARY]);
        let (this, _) = shape(&mut e);
        let cinfo = cinfo_with_words(&mut e, this);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cb_00a0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x0102_0300);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x55);
        assert_eq!(calls_to(&e, BASE_LOAD_BINARY), vec![vec![this, stream]]);
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cb_00a0, &args![this, stream]);
        for i in 0..8 {
            assert_eq!(
                e.mem.u32(cinfo + 0x10 + 4 * i),
                (0x0102_0300 + i).swap_bytes()
            );
        }
        assert_eq!(e.mem.u32(cinfo + 0x40), 0);
        // The ninth word is not part of it.
        assert_eq!(e.mem.u32(cinfo + 0x30), 0);
    }

    #[test]
    fn save_binary_swaps_around_the_base_save() {
        let mut e = Engine::new();
        // The base save records the first word it sees.
        e.register(BASE_SAVE_BINARY, |e, a| {
            let cinfo = e.mem.u32(a[0] + 0xc);
            let seen = e.mem.u32(cinfo + 0x10);
            e.mem.set_u32(a[0] + 0x14, seen);
            Ret::default()
        });
        let (this, _) = shape(&mut e);
        let cinfo = cinfo_with_words(&mut e, this);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cb_0120, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x14), 0x0102_0300);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x55);
        e.mem.set_u8(stream + 0x1e4, 0);
        e.call(0x00cb_0120, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x14), 0x0003_0201);
        // Swapped back, with the +0x40 word cleared.
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x0102_0300);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0);
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![this, stream]; 2]);
    }

    #[test]
    fn create_clone_brackets_the_new_shape_with_the_clone_lock() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        let (this, _) = shape(&mut e);
        quiet(&mut e, &[CLONE_LOCK, CLONE_UNLOCK, BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let clone = e.call(0x00cb_05d0, &args![this, 0x4242u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_TRI_SAMPLED_HEIGHT_FIELD);
        for counter in [
            COUNT_SHAPE,
            COUNT_BV_TREE_SHAPE,
            COUNT_TRI_SAMPLED_HEIGHT_FIELD,
        ] {
            assert_eq!(e.global::<u32>(counter), 1);
        }
        assert_eq!(
            order(&e),
            vec![
                0x00cb_05d0,
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
        assert_eq!(e.call(0x00cb_05d0, &args![this, 1u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, 0, 1]]);
    }

    /// The memory router with an allocator whose slot 4 hands out blocks of the asked size
    /// and whose slot 8 records `(pointer, size)`.
    fn router(e: &mut Engine) {
        let allocator = object(e, &[(4, 0x0900_0200), (8, 0x0900_0201)]);
        e.register(0x0900_0200, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        quiet(e, &[0x0900_0201]);
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        e.map(0x0200_3000, 0x10);
        e.mem.set_u32(0x0200_3000, router);
        e.register(0x00c8_5750, |e, _| Ret {
            eax: e.mem.u32(0x0200_3000),
            ..Ret::default()
        });
    }

    #[test]
    fn create_cinfo_builds_a_cinfo_once() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        router(&mut e);
        quiet(&mut e, &[BASE_COPY_CINFO]);
        e.register(CINFO_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CHILD_SOURCE, |_, _| Ret::default());
        let (this, _) = shape(&mut e);
        e.mem.set_u32(this + 8, 0);
        let created = e.mem.alloc(8);
        e.mem.set_u8(created, 9);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00cb_06c0, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&e, 0x0900_0200).len(), 1);
        assert_eq!(calls_to(&e, 0x0900_0200)[0][1], 0x50);
        assert_eq!(calls_to(&e, CINFO_CONSTRUCT), vec![vec![cinfo]]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert!(calls_to(&e, BASE_COPY_CINFO).is_empty());
        assert_eq!(scope(&mut e), 0x77);
        // The second call finds it.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cb_06c0, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert!(calls_to(&e, 0x0900_0200).is_empty());
        // With a Havok shape, CopyCinfo (the shape's own) fills it in.
        let (other, _) = shape(&mut e);
        e.call_log = Some(vec![]);
        let filled = e.call(0x00cb_06c0, &args![other, created]).u32();
        assert_eq!(e.mem.u32(other + 0xc), filled);
        assert_eq!(calls_to(&e, BASE_COPY_CINFO), vec![vec![other, filled]]);
        // A failed allocation stores null and does not construct.
        e.register(0x0900_0200, |_, _| Ret::default());
        let (third, _) = shape(&mut e);
        e.mem.set_u32(third + 8, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cb_06c0, &args![third, created]).u32(), 0);
        assert!(calls_to(&e, CINFO_CONSTRUCT).is_empty());
    }

    #[test]
    fn kill_cinfo_frees_the_0x30_bytes_only_when_asked() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        router(&mut e);
        let (this, _) = shape(&mut e);
        e.mem.set_u32(this + 0xc, 0xc1f0);
        e.call_log = Some(vec![]);
        e.call(0x00cb_0780, &args![this, 0x100u32]);
        assert!(calls_to(&e, 0x0900_0201).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0xc1f0);
        assert_eq!(scope(&mut e), 0x77);
        e.call(0x00cb_0780, &args![this, 1u32]);
        let freed = calls_to(&e, 0x0900_0201);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1..], [0xc1f0, 0x30]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // No cinfo: nothing is freed, +0xC stays clear.
        e.call(0x00cb_0780, &args![this, 1u32]);
        assert_eq!(calls_to(&e, 0x0900_0201).len(), 1);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn init_builds_the_height_field_collection_and_tree() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        router(&mut e);
        map_globals(&mut e);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(COLLECTION_FLOAT, 0x7f80_0001u32);
        // The height field constructor sets its row count.
        e.register(HK_HEIGHT_FIELD_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 9);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(HK_COLLECTION_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(HK_TREE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(FLOAT_TO_INT, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
        quiet(&mut e, &[REMOVE_REFERENCE, 0x0900_0103, 0x0900_0104]);
        let this = object(&mut e, &[(0x8c, 0x0900_0103), (0xbc, 0x0900_0104)]);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(cinfo + 0x40, 0x4141);
        e.mem.set_u32(cinfo + 0x44, 0x7f80_0007);
        e.call_log = Some(vec![]);
        e.call(0x00cb_0820, &args![this, cinfo]);
        let field_calls = calls_to(&e, HK_HEIGHT_FIELD_CONSTRUCT);
        let field = field_calls[0][0];
        assert_eq!(field_calls, vec![vec![field, cinfo + 0x10, 2]]);
        assert_eq!(e.mem.u16(field + 4), 0x70);
        assert_eq!(e.mem.u32(field), VTABLE_HK_HEIGHT_FIELD);
        assert_eq!(e.mem.u32(field + 0x60), 0x4141);
        assert_eq!(e.mem.u32(field + 0x64), 8);
        assert_eq!(e.mem.u32(field + 0x68), 0x7fc0_0007);
        let collection_calls = calls_to(&e, HK_COLLECTION_CONSTRUCT);
        let collection = collection_calls[0][0];
        assert_eq!(collection_calls, vec![vec![collection, field, 0x7fc0_0001]]);
        assert_eq!(e.mem.u16(collection + 4), 0x40);
        let tree_calls = calls_to(&e, HK_TREE_CONSTRUCT);
        let tree = tree_calls[0][0];
        assert_eq!(tree_calls, vec![vec![tree, collection, 0]]);
        assert_eq!(e.mem.u16(tree + 4), 0x30);
        assert_eq!(calls_to(&e, 0x0900_0103), vec![vec![this, tree]]);
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![tree], vec![collection], vec![field]]
        );
        assert_eq!(calls_to(&e, 0x0900_0104), vec![vec![this, cinfo]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_text() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        e.set_global(CLASS_TEXT, 0x1111u32);
        quiet(&mut e, &[BASE_GET_VIEWER_STRINGS, 0x0096_ad30]);
        e.register(VIEWER_STRING, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        // The list stores each added text at its index.
        e.register(0x0096_ae90, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], text);
            let count = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, count + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let list = e.mem.alloc(0x200);
        e.mem.set_u16(list + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x00cb_09b0, &args![this, list]);
        assert_eq!(e.mem.u32(list + 0x100), 0x1112);
        assert_eq!(
            calls_to(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![this, list]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x1111]]);
        assert!(calls_to(&e, 0x0096_ad30).is_empty());
        assert_eq!(scope(&mut e), 0x77);
        // A full list grows first, by its grow-by plus its count.
        e.mem.set_u16(list + 8, 1);
        e.mem.set_u16(list + 0xa, 1);
        e.mem.set_u16(list + 0xe, 4);
        e.call(0x00cb_09b0, &args![this, list]);
        assert_eq!(calls_to(&e, 0x0096_ad30), vec![vec![list, 5]]);
    }

    /// A height field: width and depth 10, scale 1 in every lane, offset 0; slot `0x34`
    /// answers a height from the table `(x - 3, z - 5)` -> `1, 4, 2, 8` (zero elsewhere),
    /// slot `0x38` stores the flag held at `+0x60`.
    fn height_field(e: &mut Engine, flag: u8) -> u32 {
        map_globals(e);
        e.set_global(ZERO, 0u32);
        e.set_global(ONE, 1.0f32);
        e.set_global(LARGE, f32::from_bits(0x7f7f_ffff));
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(ONE_DOUBLE, 1.0f64);
        for lane in 0..4u32 {
            e.set_global(BIAS + 4 * lane, 196_608.0f32);
        }
        let field = object(e, &[(0x34, 0x0900_0300), (0x38, 0x0900_0301)]);
        e.register(0x0900_0300, |_, a| {
            let st0 = match (a[1] as i32 - 3, a[2] as i32 - 5) {
                (0, 0) => 1.0,
                (1, 0) => 4.0,
                (0, 1) => 2.0,
                (1, 1) => 8.0,
                _ => 0.0,
            };
            Ret {
                st0,
                ..Ret::default()
            }
        });
        e.register(0x0900_0301, |e, a| {
            let flag = e.mem.u8(a[0] + 0x60);
            e.mem.set_u8(a[1], flag);
            Ret::default()
        });
        for lane in 0..4u32 {
            e.mem.set_f32(field + 0x30 + 4 * lane, 1.0);
            e.mem.set_f32(field + 0x40 + 4 * lane, 0.0);
        }
        e.mem.set_u32(field + 0x10, 10);
        e.mem.set_u32(field + 0x14, 10);
        e.mem.set_f32(field + 0x24, 2.0);
        e.mem.set_u8(field + 0x60, flag);
        field
    }

    /// Runs one sphere at `(x, 0.5, z, 0.25)` through `collideSpheres`, answering the result.
    fn collide_one(e: &mut Engine, field: u32, x: f32, z: f32) -> [f32; 4] {
        let sphere = e.mem.alloc(0x10);
        for (lane, value) in [x, 0.5, z, 0.25].into_iter().enumerate() {
            e.mem.set_f32(sphere + 4 * lane as u32, value);
        }
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, sphere);
        e.mem.set_u32(input + 4, 1);
        let result = e.mem.alloc(0x10);
        e.call(0x00cb_01f0, &args![field, input, result]);
        [
            e.mem.f32(result),
            e.mem.f32(result + 4),
            e.mem.f32(result + 8),
            e.mem.f32(result + 12),
        ]
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() <= 1e-4 * expected.abs().max(1.0),
            "{actual} != {expected}"
        );
    }

    /// The normal and the distance (for the field's `+0x1C` byte zero) the code gives for the
    /// slopes `(slope_x, slope_z)` and plane height `height` at a sphere with scaled `y` 0.5.
    fn expect_plane(result: [f32; 4], slope_x: f32, slope_z: f32, height: f32) {
        let length = (slope_x * slope_x + 1.0 + slope_z * slope_z).sqrt();
        assert_close(result[0], -slope_x / length);
        assert_close(result[1], 1.0 / length);
        assert_close(result[2], -slope_z / length);
        assert_close(result[3], (0.5 - height) * 2.0 - 0.25);
    }

    #[test]
    fn collide_spheres_keeps_the_default_outside_the_grid() {
        let mut e = Engine::new();
        let field = height_field(&mut e, 1);
        e.call_log = Some(vec![]);
        // Cell index 9 is not below width - 1.
        let result = collide_one(&mut e, field, 9.5, 5.5);
        assert_eq!(result, [0.0, 1.0, 0.0, f32::from_bits(0x7f7f_ffff)]);
        let result = collide_one(&mut e, field, 3.5, 9.5);
        assert_eq!(result[1], 1.0);
        assert!(calls_to(&e, 0x0900_0300).is_empty());
        // Two spheres: the count and the 0x10 strides.
        let spheres = e.mem.alloc(0x20);
        for i in 0..2u32 {
            e.mem.set_f32(spheres + 0x10 * i, 20.0);
            e.mem.set_f32(spheres + 0x10 * i + 8, 20.0);
        }
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, spheres);
        e.mem.set_u32(input + 4, 2);
        let output = e.mem.alloc(0x20);
        for offset in 0..0x20 {
            e.mem.set_u8(output + offset, 0xaa);
        }
        e.call(0x00cb_01f0, &args![field, input, output]);
        assert_eq!(e.mem.f32(output + 0x14), 1.0);
        assert_eq!(e.mem.f32(output + 4), 1.0);
        // No spheres: nothing is written.
        e.mem.set_u32(input + 4, 0);
        e.mem.set_u32(output, 0xdead_beef);
        e.call(0x00cb_01f0, &args![field, input, output]);
        assert_eq!(e.mem.u32(output), 0xdead_beef);
    }

    #[test]
    fn collide_spheres_picks_the_triangle_half_from_the_flag_and_fractions() {
        let mut e = Engine::new();
        let field = height_field(&mut e, 1);
        // Flag set, fz < fx (fx 0.75, fz 0.25): corners (3,5) 1, (4,5) 4, (4,6) 8.
        let lower = collide_one(&mut e, field, 3.75, 5.25);
        expect_plane(lower, 3.0, 4.0, 1.0 + 4.0 * 0.25 + 3.0 * 0.75);
        // Flag set, fz >= fx (fx 0.25, fz 0.75): corners (3,5) 1, (3,6) 2, (4,6) 8.
        let upper = collide_one(&mut e, field, 3.25, 5.75);
        expect_plane(upper, 6.0, 1.0, 1.0 + 1.0 * 0.75 + 6.0 * 0.25);
        // Flag clear, fx + fz > 1 (fx 0.75, fz 0.5): corners (4,5) 4, (3,6) 2, (4,6) 8.
        e.mem.set_u8(field + 0x60, 0);
        let far = collide_one(&mut e, field, 3.75, 5.5);
        expect_plane(far, 6.0, 4.0, (0.75 - 1.0) * 6.0 + (4.0 + 4.0 * 0.5));
        // Flag clear, fx + fz <= 1 (fx 0.25, fz 0.5): corners (3,5) 1, (4,5) 4, (3,6) 2.
        let near = collide_one(&mut e, field, 3.25, 5.5);
        expect_plane(near, 3.0, 1.0, 1.0 + 1.0 * 0.5 + 3.0 * 0.25);
    }

    #[test]
    fn collide_spheres_measures_the_distance_from_the_corner_plane() {
        let mut e = Engine::new();
        let field = height_field(&mut e, 1);
        // Byte at +0x1C set: the dot product of (scaled - (x, corner height, z)) * weights
        // with the normal. Weights 1, 1, 1; the lower half has the corner (3, 1.0, 5).
        e.mem.set_u8(field + 0x1c, 1);
        for lane in 0..3u32 {
            e.mem.set_f32(field + 0x20 + 4 * lane, 1.0);
        }
        let result = collide_one(&mut e, field, 3.75, 5.25);
        let length = (9.0f32 + 1.0 + 16.0).sqrt();
        let normal = [-3.0 / length, 1.0 / length, -4.0 / length];
        let expected =
            (3.75 - 3.0) * normal[0] + (0.5 - 1.0) * normal[1] + (5.25 - 5.0) * normal[2] - 0.25;
        assert_close(result[3], expected);
        // The flag-clear half uses the corner (4, h10 = 4.0, 5).
        e.mem.set_u8(field + 0x60, 0);
        let result = collide_one(&mut e, field, 3.25, 5.5);
        let length = (9.0f32 + 1.0 + 1.0).sqrt();
        let normal = [-3.0 / length, 1.0 / length, -1.0 / length];
        let expected =
            (3.25 - 4.0) * normal[0] + (0.5 - 4.0) * normal[1] + (5.5 - 5.0) * normal[2] - 0.25;
        assert_close(result[3], expected);
    }

    #[test]
    fn collide_spheres_gives_a_zero_normal_for_a_zero_length() {
        let mut e = Engine::new();
        let field = height_field(&mut e, 1);
        // A zero y scale and flat slopes leave a zero length: the guard answers zero.
        for lane in 0..4u32 {
            e.mem.set_f32(field + 0x30 + 4 * lane, 0.0);
        }
        e.mem.set_u32(field + 0x10, 100);
        e.mem.set_u32(field + 0x14, 100);
        // Scale 0 puts every cell at index 0 (the bias alone), the corner table is all zero.
        let result = collide_one(&mut e, field, 3.5, 5.5);
        assert_eq!(result[0].to_bits() & 0x7fff_ffff, 0);
        assert_eq!(result[1], 0.0);
        assert_eq!(result[2].to_bits() & 0x7fff_ffff, 0);
    }
}
