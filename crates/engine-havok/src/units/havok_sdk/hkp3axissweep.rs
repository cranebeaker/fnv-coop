//! `Havok SDK/hkp3axissweep.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkp3AxisSweep` is Havok's three-axis sweep-and-prune broad phase. The Xbox PDB has the
//! `hkpBroadPhase` base (0x14 bytes, vtable slots 0x0c..0x70) but not this class, so the
//! names below that are not function names are role names read from the code, not from the
//! PDB. What the code shows:
//!
//! * The sweep (`this`) holds the world bounds (+0x20 low, +0x30 high), the float offsets
//!   and scale that quantise a float box into 16-bit endpoint values (+0x40 low offset,
//!   +0x50 high offset, +0x60 scale), the same three for the 32-bit variant (+0x70, +0x80,
//!   +0x90; all `hkVector4`), the node array (+0xa0, `hkArray` of 16-byte [`HkpBpNode`]),
//!   three endpoint arrays (+0xac, +0xb8, +0xc4: x, y, z; each an `hkArray` of 4-byte
//!   endpoints, value `u16` then node index `u16`, bit 0 of the value set for a maximum),
//!   and the markers (+0xd0 count, +0xd4 shift, +0xd8 array of 16-byte [`HkpBpMarker`]).
//! * An `hkArray` is `{ data, size, capacityAndFlags }`; the low 30 bits of the last word are
//!   the capacity and bit 31 says the array does not own its memory.
//! * Node 0 is a sentinel; `getNumObjects` is the node count minus one.
//! * Function-local temporaries that Havok takes from the thread memory router's stack
//!   allocator (`00c85750`, +0xc, vtable slots 0xc and 0x10) are real allocations here too.
//!
//! x87 note: the 16-bit endpoint values are converted back to floats with `FILD`/`FMUL` and
//! stored as `float`; that is a product computed in `f64` and rounded once. SSE code
//! (`MINPS`, `MULPS`, `RCPPS`) computes in `f32`; `RCPPS` is the host CPU's estimate, as in
//! the other Havok units.

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------
// Callees outside this file.
// ---------------------------------------------------------------------------

/// `00c85750` (`hkMemoryRouter.cpp`, no map name): the current thread's memory router.
/// Its +0x10 is the heap allocator, its +0xc the stack allocator and its +0x0 the stack
/// top used by the bump allocations of `calcAabbCache_ov2` and `castRay`.
pub(crate) const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00c90940` (`hkArrayUtil.cpp`, no map name): `(heap allocator, hkArray*, elementSize)`
/// makes room for more elements in a full array.
pub(crate) const ARRAY_GROW: u32 = 0x00c9_0940;
/// `hkArrayUtil::_reserve` `00c908c0` (Xbox PDB): `(heap allocator, hkArray*, capacity,
/// elementSize)`.
pub(crate) const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkStatisticsCollector::getAllocatedSize` (cdecl `(pointer, size)`).
const STATISTICS_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkStatisticsCollector::addNormalChunk` (`this`, name, pointer, size, 0).
const STATISTICS_ADD_NORMAL_CHUNK: u32 = 0x00d0_6fb0;

// ---------------------------------------------------------------------------
// Constants read from the exe's data.
// ---------------------------------------------------------------------------

/// `65532.0` x4: the top of the 16-bit quantisation range (`MINPS` limit).
const QUANTISE_MAX: u32 = 0x011b_3790;
/// `65536.0` x4: added before the shift so the float's bits hold the integer part.
const QUANTISE_ADD: u32 = 0x011b_3770;
/// `2147352576.0` x4: the upper clamp of `addObject`'s 32-bit conversion.
const QUANTISE_MAX_32: u32 = 0x011b_6d70;
/// `0.0` x4 lower clamp of the same conversion.
const QUANTISE_MIN_32: u32 = 0x0126_7e30;
/// `0x80000000` x4: the sign mask `XORPS` negates a vector with.
const SIGN_MASK: u32 = 0x010c_44d0;
/// `{1.0, 1.0, 1.0, 1.0}`: lane 3 of it takes the place of the scale's `w`.
const ONES: u32 = 0x010c_7170;
/// `65532.0` (`float`).
const SCALE_FLOAT: u32 = 0x010c_d5b0;
/// `1.525972e-05` (`float`, 1 / 65532 rounded).
const INVERSE_SCALE_FLOAT: u32 = 0x010c_d5b4;
/// `0.0` (`float`).
const ZERO_FLOAT: u32 = 0x0101_1d78;
/// Table of the node field offsets of an endpoint: `[(isMax) + axis * 2]` is x: 8, 10;
/// y: 0, 4; z: 2, 6.
const NODE_FIELD_TABLE: u32 = 0x011b_37a4;
/// `"Nodes"`, `"Axis"` and `"Markers"`, the names handed to the statistics collector.
const NODES_NAME: u32 = 0x010c_d5c0;
const AXIS_NAME: u32 = 0x0104_4b20;
const MARKERS_NAME: u32 = 0x010c_d5b8;

/// The vtable `hkp3AxisSweep` objects get (scalar deleting destructor first).
const VTABLE: u32 = 0x010c_d5cc;

/// Capacity mask of an `hkArray`'s last word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The last word of an empty array that owns nothing.
const NOT_OWNED: u32 = 0x8000_0000;

/// `00d97010` (`hkpBroadPhase` base constructor, no map name): `(this)`.
const BASE_CONSTRUCTOR: u32 = 0x00d9_7010;
/// `00d970c0` (`hkpBroadPhase` base destructor, no map name): `(this)`.
const BASE_DESTRUCTOR: u32 = 0x00d9_70c0;
/// `00c85690` (no map name): `(heap allocator, size)` cdecl, returns the memory.
const EASY_ALLOC: u32 = 0x00c8_5690;
/// `00c856c0` (no map name): `(heap allocator, pointer)` cdecl.
const EASY_FREE: u32 = 0x00c8_56c0;
/// `10.0` (`float`).
const TEN_FLOAT: u32 = 0x0101_7b78;
/// `11.0` (`float`).
const ELEVEN_FLOAT: u32 = 0x0105_0cb0;
/// `1.0` (`float`).
const UNIT_FLOAT: u32 = 0x0101_2068;
/// `0.5` (`double`).
const HALF_DOUBLE: u32 = 0x0101_1588;
/// `11.0` (`double`).
const ELEVEN_DOUBLE: u32 = 0x0103_5718;

/// Appends `value` to an `hkArray` of 4-byte elements whose room was already made: the slot
/// is written only when its address is not 0, then the size grows.
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
    if slot != 0 {
        e.mem.set_u32(slot, value);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// ---------------------------------------------------------------------------
// Layouts.
// ---------------------------------------------------------------------------

layout! {
    /// `hkp3AxisSweep::hkpBpNode`: one object's six endpoint indices and its handle.
    pub struct HkpBpNode: 0x10 {
        /// Index of the minimum endpoint on the y axis.
        0x00 min_y: u16,
        /// Index of the minimum endpoint on the z axis.
        0x02 min_z: u16,
        /// Index of the maximum endpoint on the y axis.
        0x04 max_y: u16,
        /// Index of the maximum endpoint on the z axis.
        0x06 max_z: u16,
        /// Index of the minimum endpoint on the x axis.
        0x08 min_x: u16,
        /// Index of the maximum endpoint on the x axis.
        0x0a max_x: u16,
        /// The object's handle pointer, or `markerOffset | 1` for a marker node.
        0x0c handle: u32,
    }

    /// `hkp3AxisSweep::hkpBpMarker`: a node with the objects that straddle its x position.
    pub struct HkpBpMarker: 0x10 {
        /// Node index the marker sits on.
        0x00 node: u16,
        /// `hkArray<u16>` of the nodes overlapping the marker: data.
        0x04 overlap_data: u32,
        /// Size of that array.
        0x08 overlap_size: i32,
        /// Capacity and flags of that array.
        0x0c overlap_capacity: u32,
    }

    /// `hkp3AxisSweep` (class not in the Xbox PDB; role names), 0xe0 bytes on PC.
    pub struct Hkp3AxisSweep: 0xe0 {
        /// Node array data (`hkArray<hkpBpNode>` at +0xa0).
        0xa0 nodes: u32,
        /// Node count.
        0xa4 node_count: i32,
        /// Node array capacity and flags.
        0xa8 node_capacity: u32,
        /// X endpoint array (`hkArray<hkpBpEndPoint>` at +0xac): data.
        0xac x_endpoints: u32,
        /// X endpoint count.
        0xb0 x_count: i32,
        /// X endpoint capacity and flags.
        0xb4 x_capacity: u32,
        /// Y endpoint array data.
        0xb8 y_endpoints: u32,
        /// Y endpoint count.
        0xbc y_count: i32,
        /// Y endpoint capacity and flags.
        0xc0 y_capacity: u32,
        /// Z endpoint array data.
        0xc4 z_endpoints: u32,
        /// Z endpoint count.
        0xc8 z_count: i32,
        /// Z endpoint capacity and flags.
        0xcc z_capacity: u32,
        /// Number of markers (`(1 << bits) - 1`, 0 for a sweep built without markers).
        0xd0 marker_count: i32,
        /// Shift count of the marker spacing: `bits`.
        0xd4 marker_bits: i32,
        /// Marker array (`hkpBpMarker`s).
        0xd8 markers: u32,
        /// Float exponent offset found by the constructor's search.
        0xdc exponent_offset: f32,
    }
}

/// `hkVector4` fields of the sweep, by offset (each 16 bytes).
const WORLD_MIN: u32 = 0x20;
const WORLD_MAX: u32 = 0x30;
const OFFSET_LOW: u32 = 0x40;
const OFFSET_HIGH: u32 = 0x50;
const SCALE: u32 = 0x60;
const OFFSET_LOW_32: u32 = 0x70;
const OFFSET_HIGH_32: u32 = 0x80;
const SCALE_32: u32 = 0x90;

/// Byte offsets of the three `hkArray`s of endpoints in the sweep, x, y, z.
const AXIS_ARRAYS: [u32; 3] = [0xac, 0xb8, 0xc4];
/// Node field offsets of the minimum and maximum endpoint index, x, y, z.
const AXIS_NODE_FIELDS: [[u32; 2]; 3] = [[8, 10], [0, 4], [2, 6]];

// ---------------------------------------------------------------------------
// Shared helpers.
// ---------------------------------------------------------------------------

type Vec4 = [f32; 4];

fn load_vec4(e: &Engine, addr: u32) -> Vec4 {
    [
        e.mem.f32(addr),
        e.mem.f32(addr + 4),
        e.mem.f32(addr + 8),
        e.mem.f32(addr + 12),
    ]
}

fn store_vec4(e: &mut Engine, addr: u32, v: Vec4) {
    for (i, lane) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * i as u32, *lane);
    }
}

/// Copies 16 bytes as the bit patterns `MOVAPS` moves.
fn copy_vec4(e: &mut Engine, from: u32, to: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `MINPS a, b`: each lane `a` when `a < b`, else `b` (NaN and equal lanes give `b`).
fn minps(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| if a[i] < b[i] { a[i] } else { b[i] })
}

/// `MAXPS a, b`: each lane `a` when `a > b`, else `b`.
fn maxps(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| if a[i] > b[i] { a[i] } else { b[i] })
}

fn add4(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] + b[i])
}

fn sub4(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] - b[i])
}

fn mul4(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] * b[i])
}

/// `XORPS v, mask` on the bit patterns.
fn xor4(v: Vec4, mask: [u32; 4]) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(v[i].to_bits() ^ mask[i]))
}

fn load_mask(e: &Engine, addr: u32) -> [u32; 4] {
    [
        e.mem.u32(addr),
        e.mem.u32(addr + 4),
        e.mem.u32(addr + 8),
        e.mem.u32(addr + 12),
    ]
}

/// `RCPPS` of one lane (the host CPU's estimate on x86).
#[cfg(target_arch = "x86_64")]
fn rcp_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
fn rcp_estimate(x: f32) -> f32 {
    1.0 / x
}

/// The scale vector with lane 3 replaced by lane 3 of the constant at `ONES`
/// (`UNPCKHPS` / `SHUFPS 0xc4`), then one Newton step on its `RCPPS`:
/// `(r + r) - (r * x) * r`.
fn refined_reciprocal(e: &Engine, scale: u32) -> Vec4 {
    let mut x = load_vec4(e, scale);
    x[3] = e.mem.f32(ONES + 12);
    let r = x.map(rcp_estimate);
    let rx = mul4(r, x);
    sub4(add4(r, r), mul4(rx, r))
}

/// `1.0 / divisor` the x87 way: in `f64`, rounded to the `float` that gets stored.
fn x87_inverse(divisor: f32) -> f32 {
    (1.0f64 / divisor as f64) as f32
}

/// The check every `hkArray::pushBack` of the sweep starts with: `00c85750`, then growing
/// the array through `00c90940` when its size equals its capacity.
fn array_make_room(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let heap = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![heap, array, element_size]);
    }
}

/// `hkArray::reserve(wanted)` as the sweep inlines it: when `capacity < wanted`, grows to
/// `max(2 * capacity, wanted)` through `hkArrayUtil::_reserve`.
fn array_reserve_at_least(e: &mut Engine, array: u32, wanted: i32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_mul(2);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        let heap = e.mem.u32(router + 0x10);
        e.call(
            ARRAY_RESERVE,
            &args![heap, array, new_capacity, element_size],
        );
    }
}

/// A block taken from the router's stack allocator (vtable slot 0xc with the byte count
/// passed by address, slot 0x10 to give it back). `shift` converts the granted bytes to the
/// array's element count; an empty or zero-granting request leaves capacity `NOT_OWNED`.
struct StackBlock {
    data: u32,
    capacity: u32,
}

fn stack_alloc(e: &mut Engine, bytes: i32, shift: u32) -> StackBlock {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if bytes == 0 {
        return StackBlock {
            data: 0,
            capacity: NOT_OWNED,
        };
    }
    let allocator = e.mem.u32(router + 0xc);
    let (data, granted) = e.with_stack(4, |e, size_cell| {
        e.mem.set_i32(size_cell.addr(), bytes);
        let data = e.vcall(allocator, 0xc, &args![size_cell.addr()]).u32();
        (data, e.mem.i32(size_cell.addr()))
    });
    let rounding = if shift == 0 {
        0
    } else {
        (granted >> 31) & ((1 << shift) - 1)
    };
    let elements = granted.wrapping_add(rounding) >> shift;
    StackBlock {
        data,
        capacity: if elements == 0 {
            NOT_OWNED
        } else {
            elements as u32
        },
    }
}

fn stack_free(e: &mut Engine, block: &StackBlock, shift: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if (block.capacity as i32) >= 0 {
        let allocator = e.mem.u32(router + 0xc);
        e.vcall(
            allocator,
            0x10,
            &args![block.data, (block.capacity & CAPACITY_MASK) << shift],
        );
    }
}

/// `bits[value >> 5] ^= 1 << (value & 31)`.
fn toggle_bit(e: &mut Engine, bits: u32, value: u32) {
    let word = bits + (value >> 5) * 4;
    let v = e.mem.u32(word);
    e.mem.set_u32(word, v ^ (1u32 << (value & 31)));
}

/// `bits[value >> 5] &= !(1 << (value & 31))`.
fn clear_bit(e: &mut Engine, bits: u32, value: u32) {
    let word = bits + (value >> 5) * 4;
    let v = e.mem.u32(word);
    e.mem.set_u32(word, v & !(1u32 << (value & 31)));
}

/// `PCMPGTW` for one lane: all ones when `a > b` (signed), else zero.
fn lane_greater(a: i16, b: i16) -> i16 {
    if a > b {
        -1
    } else {
        0
    }
}

/// Appends `(first.handle, second.handle)` to the pair array (the body of `00d11bc0`).
fn push_handle_pair(e: &mut Engine, pairs: u32, first: u32, second: u32) {
    array_make_room(e, pairs, 8);
    let size = e.mem.u32(pairs + 4);
    let slot = e.mem.u32(pairs) + size * 8;
    e.mem.set_u32(pairs + 4, size + 1);
    let first_handle = e.mem.u32(first + 0xc);
    e.mem.set_u32(slot, first_handle);
    let second_handle = e.mem.u32(second + 0xc);
    e.mem.set_u32(slot + 4, second_handle);
}

/// Appends `(first, second)` to the pair array.
fn push_pair(e: &mut Engine, pairs: u32, first: u32, second: u32) {
    array_make_room(e, pairs, 8);
    let size = e.mem.u32(pairs + 4);
    let slot = e.mem.u32(pairs) + size * 8;
    e.mem.set_u32(slot, first);
    e.mem.set_u32(slot + 4, second);
    e.mem.set_u32(pairs + 4, size + 1);
}

/// Removes `value` from the `hkArray<u16>` of a marker (data at +4, size at +8): the last
/// element takes its place. A value that is not there leaves index -1, so the last element
/// lands one element before the data, exactly as `00d11d00` does.
fn remove_swap_u16(e: &mut Engine, marker: u32, value: u16) {
    let size = e.mem.i32(marker + 8);
    let data = e.mem.u32(marker + 4);
    let mut found = -1i32;
    let mut i = 0i32;
    while i < size {
        if e.mem.u16(data + 2 * i as u32) == value {
            found = i;
            break;
        }
        i += 1;
    }
    let new_size = size.wrapping_sub(1);
    e.mem.set_i32(marker + 8, new_size);
    if new_size != found {
        let data = e.mem.u32(marker + 4);
        let last = e
            .mem
            .u16(data.wrapping_add((new_size as u32).wrapping_mul(2)));
        e.mem
            .set_u16(data.wrapping_add((found as u32).wrapping_mul(2)), last);
    }
}

/// `CVTPS2DQ` / `FISTP` with the default rounding (nearest even); out of range gives
/// `0x80000000`, the x86 "integer indefinite".
fn round_to_even(value: f32) -> i32 {
    // `f32::round_ties_even` (Rust 1.77) spelled for the 1.75 MSRV: below
    // 2^23, adding and removing 2^23 rounds to the nearest integer, ties to
    // even; larger magnitudes, infinities and NaN are already integral.
    let rounded = if value.is_nan() || value.abs() >= 8388608.0 {
        value
    } else if value >= 0.0 {
        (value + 8388608.0) - 8388608.0
    } else {
        (value - 8388608.0) + 8388608.0
    };
    if rounded.is_nan() || rounded >= 2147483648.0 || rounded < -2147483648.0 {
        i32::MIN
    } else {
        rounded as i32
    }
}

// ---------------------------------------------------------------------------
// Functions, in address order.
// ---------------------------------------------------------------------------

// Translated from 00d10ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::getType` (Xbox PDB): the broad phase type, 0.
pub fn get_type(_e: &mut Engine, _this: Ptr<Hkp3AxisSweep>) -> u32 {
    0
}

// Translated from 00d10b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. MMX loop run by `addObject` after it has put a node's six endpoints in:
/// each of the first `count` nodes has every endpoint index raised by 1 for each of the new
/// node's endpoints on that axis that sits at or below it (signed 16-bit compares, as
/// `PCMPGTW`). The node `new_node` supplies the six new indices.
pub fn fn_00d10b30(e: &mut Engine, _unused_1: u32, nodes: u32, count: u32, new_node: u32) {
    let field = |e: &Engine, off: u32| e.mem.u16(new_node + off) as i16;
    let low = [
        field(e, 0).wrapping_sub(1),
        field(e, 2).wrapping_sub(1),
        field(e, 0).wrapping_sub(1),
        field(e, 2).wrapping_sub(1),
    ];
    let high = [
        field(e, 4).wrapping_sub(2),
        field(e, 6).wrapping_sub(2),
        field(e, 4).wrapping_sub(2),
        field(e, 6).wrapping_sub(2),
    ];
    let x_low = field(e, 8).wrapping_sub(1);
    let x_high = field(e, 10).wrapping_sub(2);
    let end = nodes.wrapping_add(count << 4);
    let mut node = nodes;
    while node < end {
        for lane in 0..4u32 {
            let a = e.mem.u16(node + 2 * lane) as i16;
            let step = lane_greater(a, low[lane as usize])
                .wrapping_add(lane_greater(a, high[lane as usize]));
            e.mem.set_u16(node + 2 * lane, a.wrapping_sub(step) as u16);
        }
        for lane in 4..6u32 {
            let a = e.mem.u16(node + 2 * lane) as i16;
            let step = lane_greater(a, x_low).wrapping_add(lane_greater(a, x_high));
            e.mem.set_u16(node + 2 * lane, a.wrapping_sub(step) as u16);
        }
        node = node.wrapping_add(16);
    }
}

// Translated from 00d10c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. MMX loop: adds `delta` (its low 16 bits) to every endpoint index of the
/// first `count` nodes that is above the threshold of its axis, the thresholds being one
/// less than the words at +0, +4 and +8 of `limits`: the y index words (node +0, +4) compare
/// against the word at +4, the z words (+2, +6) against +8, the x words (+8, +10) against
/// +0.
pub fn fn_00d10c40(e: &mut Engine, nodes: u32, count: u32, delta: i32, limits: u32) {
    let x_limit = (e.mem.u16(limits) as i16).wrapping_sub(1);
    let y_limit = (e.mem.u16(limits + 4) as i16).wrapping_sub(1);
    let z_limit = (e.mem.u16(limits + 8) as i16).wrapping_sub(1);
    let delta = delta as i16;
    let end = nodes.wrapping_add(count << 4);
    let mut node = nodes;
    while node < end {
        let thresholds = [y_limit, z_limit, y_limit, z_limit, x_limit, x_limit];
        for (lane, limit) in thresholds.iter().enumerate() {
            let at = node + 2 * lane as u32;
            let a = e.mem.u16(at) as i16;
            let step = lane_greater(a, *limit) & delta;
            e.mem.set_u16(at, a.wrapping_add(step) as u16);
        }
        node = node.wrapping_add(16);
    }
}

// Translated from 00d10d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. MMX loop run by `removeObject`: the inverse of `00d10b30`. Each of the
/// first `count` nodes has every endpoint index lowered by 1 for each of the removed node's
/// endpoints on that axis that lies below it (compared without the `- 1` and `- 2`).
pub fn fn_00d10d10(e: &mut Engine, _unused_1: u32, nodes: u32, count: u32, removed: u32) {
    let field = |e: &Engine, off: u32| e.mem.u16(removed + off) as i16;
    let first = [field(e, 0), field(e, 2), field(e, 0), field(e, 2)];
    let second = [field(e, 4), field(e, 6), field(e, 4), field(e, 6)];
    let x_first = field(e, 8);
    let x_second = field(e, 10);
    let end = nodes.wrapping_add(count << 4);
    let mut node = nodes;
    while node != end {
        for lane in 0..4u32 {
            let a = e.mem.u16(node + 2 * lane) as i16;
            let step = lane_greater(a, first[lane as usize])
                .wrapping_add(lane_greater(a, second[lane as usize]));
            e.mem.set_u16(node + 2 * lane, a.wrapping_add(step) as u16);
        }
        for lane in 4..6u32 {
            let a = e.mem.u16(node + 2 * lane) as i16;
            let step = lane_greater(a, x_first).wrapping_add(lane_greater(a, x_second));
            e.mem.set_u16(node + 2 * lane, a.wrapping_add(step) as u16);
        }
        node = node.wrapping_add(16);
    }
}

// Translated from 00d10e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::getExtents` (Xbox PDB): `low` is the negated float offset, `high` is
/// `low + 65532 / scale` (a refined `RCPPS` of the scale, its `w` taken from the constants).
pub fn get_extents(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, low: u32, high: u32) {
    let sign = load_mask(e, SIGN_MASK);
    let offset = load_vec4(e, this.addr() + OFFSET_LOW);
    store_vec4(e, low, xor4(offset, sign));
    let inverse = refined_reciprocal(e, this.addr() + SCALE);
    let scale: f32 = e.global(SCALE_FLOAT);
    let scaled = mul4([scale; 4], inverse);
    let low_now = load_vec4(e, low);
    store_vec4(e, high, add4(low_now, scaled));
}

// Translated from 00d10ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::hkpBpAxis::find` (Xbox PDB): the first endpoint in `[begin, end]` whose
/// value is not below `value` (a binary search while more than 16 endpoints are left, then
/// a linear scan). The caller guarantees a sentinel that stops the scan.
pub fn hkp_bp_axis_find(e: &mut Engine, _unused_1: u32, begin: u32, end: u32, value: u16) -> u32 {
    let mut begin = begin;
    let mut end = end;
    let mut span = end.wrapping_sub(begin);
    while (span & 0xffff_fffc) as i32 > 0x40 {
        let middle = begin.wrapping_add((((span as i32) >> 3) * 4) as u32);
        if e.mem.u16(middle) < value {
            begin = middle;
        } else {
            end = middle;
        }
        span = end.wrapping_sub(begin);
    }
    while e.mem.u16(begin) < value {
        begin += 4;
    }
    begin
}

// Translated from 00d10f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::shiftBroadPhase` (Xbox PDB): shifts every object by `-amount` through
/// `shiftAllObjects` (vtable +0x50), adds the resulting offset to both float offsets,
/// recomputes the high offsets as `low + 1 / scale` and returns the negated applied shift in
/// `result`.
pub fn shift_broad_phase(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    amount: u32,
    result: u32,
    third: u32,
) {
    let me = this.addr();
    let sign = load_mask(e, SIGN_MASK);
    let negated = xor4(load_vec4(e, amount), sign);
    let applied = e.with_stack(0x20, |e, block| {
        let negated_at = block.addr();
        let applied_at = block.addr() + 0x10;
        store_vec4(e, negated_at, negated);
        e.vcall(me, 0x50, &args![negated_at, applied_at, third]);
        load_vec4(e, applied_at)
    });
    let low = add4(load_vec4(e, me + OFFSET_LOW), applied);
    store_vec4(e, me + OFFSET_LOW, low);
    let inverse = refined_reciprocal(e, me + SCALE);
    let low_now = load_vec4(e, me + OFFSET_LOW);
    store_vec4(e, me + OFFSET_HIGH, add4(inverse, low_now));
    let low_32 = add4(load_vec4(e, me + OFFSET_LOW_32), applied);
    store_vec4(e, me + OFFSET_LOW_32, low_32);
    let inverse_32 = refined_reciprocal(e, me + SCALE_32);
    let low_32_now = load_vec4(e, me + OFFSET_LOW_32);
    store_vec4(e, me + OFFSET_HIGH_32, add4(inverse_32, low_32_now));
    store_vec4(e, result, xor4(applied, sign));
}

// Translated from 00d10ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::checkDeterminism` (Xbox PDB): does nothing in this build.
pub fn check_determinism(_e: &mut Engine, _this: Ptr<Hkp3AxisSweep>) {}

// Translated from 00d11020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::set32BitOffsetAndScale` (Xbox PDB): stores the three 32-bit vectors
/// (`offset_low`, `offset_high`, `scale`), then sets the 16-bit quantisation from the world
/// bounds: the float offset is `offset_low`, the high offset adds `1 / 65532` of the world
/// extent, and the scale is `65532 / extent` (the x87 division rounded to a `float`), each
/// with `w` zero.
pub fn set_32_bit_offset_and_scale(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    offset_low: u32,
    offset_high: u32,
    scale: u32,
) {
    let me = this.addr();
    let world_min = load_vec4(e, me + WORLD_MIN);
    copy_vec4(e, offset_low, me + OFFSET_LOW_32);
    copy_vec4(e, offset_high, me + OFFSET_HIGH_32);
    copy_vec4(e, scale, me + SCALE_32);
    let extent = sub4(load_vec4(e, me + WORLD_MAX), world_min);
    let low = load_vec4(e, offset_low);
    copy_vec4(e, offset_low, me + OFFSET_LOW);
    let inverse_scale: f32 = e.global(INVERSE_SCALE_FLOAT);
    store_vec4(
        e,
        me + OFFSET_HIGH,
        add4(mul4([inverse_scale; 4], extent), low),
    );
    let scale_float: f32 = e.global(SCALE_FLOAT);
    let zero: f32 = e.global(ZERO_FLOAT);
    let inverse = [
        x87_inverse(extent[0]),
        x87_inverse(extent[1]),
        x87_inverse(extent[2]),
        zero,
    ];
    store_vec4(e, me + SCALE, mul4(inverse, [scale_float; 4]));
    // The last lane of the three vectors is cleared.
    for base in [SCALE, OFFSET_LOW, OFFSET_HIGH] {
        e.mem.set_u32(me + base + 12, 0);
    }
}

// Translated from 00d112c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::setBitsBasedOnXInterval` (Xbox PDB): builds in `bits` the set of nodes
/// whose x interval overlaps the one of `node` (index `node_index`, which is left out of
/// the marker's list). `count` is the node count, `min_x_value` the minimum endpoint's
/// value. It clears `count / 128 + 1` blocks of 16 bytes, toggles the bit of the marker at
/// or below the interval's start and its overlap list, then walks the endpoints from there.
pub fn set_bits_based_on_x_interval(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    count: i32,
    min_x_value: i32,
    node: Ptr<HkpBpNode>,
    node_index: u16,
    bits: u32,
) {
    let me = this;
    let max_x = e.get(node, HkpBpNode::max_x) as u32;
    let min_x = e.get(node, HkpBpNode::min_x) as u32;
    let mut blocks = count >> 7;
    let mut at = bits;
    while blocks >= 0 {
        for i in 0..4 {
            e.mem.set_u32(at + 4 * i, 0);
        }
        at = at.wrapping_add(16);
        blocks -= 1;
    }
    let mut cursor = e.get(me, Hkp3AxisSweep::x_endpoints) + 4;
    let marker_count = e.get(me, Hkp3AxisSweep::marker_count);
    if marker_count != 0 {
        let shift = 0x10u32.wrapping_sub(e.get(me, Hkp3AxisSweep::marker_bits) as u32) & 0x1f;
        let index = min_x_value >> shift;
        if index > 0 {
            let marker = e.get(me, Hkp3AxisSweep::markers) + (index as u32) * 16 - 16;
            let marker_node = e.mem.u16(marker) as u32;
            toggle_bit(e, bits, marker_node);
            let mut left = e.mem.i32(marker + 8) - 1;
            let mut overlap = e.mem.u32(marker + 4);
            while left >= 0 {
                let other = e.mem.u16(overlap);
                if other != node_index {
                    toggle_bit(e, bits, other as u32);
                }
                overlap += 2;
                left -= 1;
            }
            let nodes = e.get(me, Hkp3AxisSweep::nodes);
            let x = e.get(me, Hkp3AxisSweep::x_endpoints);
            let marker_node_ptr = nodes + marker_node * 16;
            let from = x + (e.mem.u16(marker_node_ptr + 8) as u32) * 4 + 4;
            let to = x + (e.mem.u16(marker_node_ptr + 10) as u32) * 4;
            let mut p = from;
            while p < to {
                if e.mem.u8(p) & 1 == 0 {
                    clear_bit(e, bits, e.mem.u16(p + 2) as u32);
                }
                p += 4;
            }
            let x = e.get(me, Hkp3AxisSweep::x_endpoints);
            cursor = x + (e.mem.u16(marker_node_ptr + 8) as u32) * 4 + 4;
        }
    }
    let x = e.get(me, Hkp3AxisSweep::x_endpoints);
    let min_end = x + min_x * 4;
    while cursor < min_end {
        toggle_bit(e, bits, e.mem.u16(cursor + 2) as u32);
        cursor += 4;
    }
    let x = e.get(me, Hkp3AxisSweep::x_endpoints);
    cursor += 4;
    let max_end = x + max_x * 4;
    while cursor < max_end {
        if e.mem.u8(cursor) & 1 == 0 {
            toggle_bit(e, bits, e.mem.u16(cursor + 2) as u32);
        }
        cursor += 4;
    }
}

// Translated from 00d11470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::areAabbsOverlapping` (Xbox PDB): whether the nodes at the two node
/// indices (read through the pointers) overlap on all three axes: the sign bits of the
/// 16-bit differences, then of the two 32-bit differences, are all clear.
pub fn are_aabbs_overlapping(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    first_index: u32,
    second_index: u32,
) -> bool {
    let nodes = e.get(this, Hkp3AxisSweep::nodes);
    let second = nodes + e.mem.u32(second_index) * 16;
    let first = nodes + e.mem.u32(first_index) * 16;
    let word = |e: &Engine, node: u32, off: u32| e.mem.u16(node + off);
    let mut bits = word(e, second, 10).wrapping_sub(word(e, first, 8));
    bits |= word(e, first, 10).wrapping_sub(word(e, second, 8));
    bits |= word(e, second, 4).wrapping_sub(word(e, first, 0));
    bits |= word(e, first, 4).wrapping_sub(word(e, second, 0));
    if bits & 0x8000 != 0 {
        return false;
    }
    let high = e.mem.u32(second + 4).wrapping_sub(e.mem.u32(first));
    let low = e.mem.u32(first + 4).wrapping_sub(e.mem.u32(second));
    (high | low) & 0x8000_8000 == 0
}

// Translated from 00d114e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::addObject` (Xbox PDB): converts the float box (two `hkVector4`s, min then
/// max, at `aabb`) to integers with the 32-bit offsets and scale — add the offset, multiply
/// by the scale, clamp to `[0, 2147352576]`, round to nearest even — and calls the
/// integer-box `addObject` (vtable +0x14) with `(handle, box, pairs)`.
pub fn add_object(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, handle: u32, aabb: u32, pairs: u32) {
    let me = this.addr();
    let upper = load_vec4(e, QUANTISE_MAX_32);
    let lower = load_vec4(e, QUANTISE_MIN_32);
    let scale = load_vec4(e, me + SCALE_32);
    let mut integer_box = [0i32; 8];
    for half in 0..2usize {
        let offset = load_vec4(e, me + OFFSET_LOW_32 + 0x10 * half as u32);
        let corner = load_vec4(e, aabb + 0x10 * half as u32);
        let converted = maxps(minps(mul4(add4(offset, corner), scale), upper), lower);
        for (lane, value) in converted.iter().enumerate() {
            integer_box[4 * half + lane] = round_to_even(*value);
        }
    }
    e.with_stack(0x20, |e, block| {
        for (i, v) in integer_box.iter().enumerate() {
            e.mem.set_i32(block.addr() + 4 * i as u32, *v);
        }
        e.vcall(me, 0x14, &args![handle, block.addr(), pairs]);
    });
}

// Translated from 00d11620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::getNumObjects` (Xbox PDB): the node count minus the sentinel.
pub fn get_num_objects(e: &mut Engine, this: Ptr<Hkp3AxisSweep>) -> i32 {
    e.get(this, Hkp3AxisSweep::node_count).wrapping_sub(1)
}

// Translated from 00d11630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::getAabbCacheSize` (Xbox PDB): `(nodes - markers) * 24 + 0x24` bytes.
pub fn get_aabb_cache_size(e: &mut Engine, this: Ptr<Hkp3AxisSweep>) -> i32 {
    let nodes = e.get(this, Hkp3AxisSweep::node_count);
    let markers = e.get(this, Hkp3AxisSweep::marker_count);
    nodes
        .wrapping_sub(markers)
        .wrapping_mul(3)
        .wrapping_mul(8)
        .wrapping_add(0x24)
}

// Translated from 00d11650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::getAabbFromNode` (Xbox PDB): the float box of `node` into `out` (two
/// `hkVector4`s): each endpoint value times `1 / scale` (the x87 division rounded to
/// `float`, the product rounded once to `float`) minus the float offset, `w` being
/// `0.0 - offset.w`.
pub fn get_aabb_from_node(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    node: Ptr<HkpBpNode>,
    out: u32,
) {
    let me = this.addr();
    let scale = load_vec4(e, me + SCALE);
    let inverse = [
        x87_inverse(scale[0]),
        x87_inverse(scale[1]),
        x87_inverse(scale[2]),
    ];
    let x = e.get(this, Hkp3AxisSweep::x_endpoints);
    let y = e.get(this, Hkp3AxisSweep::y_endpoints);
    let z = e.get(this, Hkp3AxisSweep::z_endpoints);
    let value = |e: &Engine, array: u32, index: u16| e.mem.u16(array + 4 * index as u32) as f64;
    let min_x = value(e, x, e.get(node, HkpBpNode::min_x));
    let min_y = value(e, y, e.get(node, HkpBpNode::min_y));
    let min_z = value(e, z, e.get(node, HkpBpNode::min_z));
    let max_x = value(e, x, e.get(node, HkpBpNode::max_x));
    let max_y = value(e, y, e.get(node, HkpBpNode::max_y));
    let max_z = value(e, z, e.get(node, HkpBpNode::max_z));
    let zero: f32 = e.global(ZERO_FLOAT);
    let offset = load_vec4(e, me + OFFSET_LOW);
    let product = |v: f64, k: f32| (v * k as f64) as f32;
    let low = [
        product(min_x, inverse[0]),
        product(min_y, inverse[1]),
        product(min_z, inverse[2]),
        zero,
    ];
    let high = [
        product(max_x, inverse[0]),
        product(max_y, inverse[1]),
        product(max_z, inverse[2]),
        zero,
    ];
    store_vec4(e, out, sub4(low, offset));
    store_vec4(e, out + 0x10, sub4(high, offset));
}

// Translated from 00d11790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::_fixDeterministicOrderAfterNodeIdWasDecreased` (Xbox PDB): after node
/// `node` got a smaller index, moves each of its six endpoints down past equal-valued
/// endpoints of nodes with a larger index, so equal values stay ordered by node index, and
/// updates the node fields of the endpoints it passes.
pub fn fix_deterministic_order_after_node_id_was_decreased(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    node: u32,
) {
    let me = this.addr();
    let nodes = e.get(this, Hkp3AxisSweep::nodes);
    let node_ptr = nodes + node * 16;
    for (axis, array) in AXIS_ARRAYS.iter().enumerate() {
        for field in AXIS_NODE_FIELDS[axis] {
            let endpoints = e.mem.u32(me + array);
            let mut index = e.mem.u16(node_ptr + field) as u32;
            let mut at = endpoints + index * 4;
            if e.mem.u16(at) == e.mem.u16(at - 4) {
                loop {
                    let before = e.mem.u16(at - 2) as u32;
                    if before <= node {
                        break;
                    }
                    let moved = e.mem.u32(at - 4);
                    e.mem.set_u32(at, moved);
                    e.mem.set_u16(nodes + before * 16 + field, index as u16);
                    index = index.wrapping_sub(1);
                    at -= 4;
                    if e.mem.u16(at) != e.mem.u16(at - 4) {
                        break;
                    }
                }
            }
            e.mem.set_u16(at + 2, node as u16);
            e.mem.set_u16(node_ptr + field, index as u16);
        }
    }
}

// Translated from 00d11960 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Removes the two elements at index `first` and index `second`
/// (`first < second`) from an `hkArray` of 4-byte elements, keeping the order of the rest,
/// and lowers the size by 2: the elements between them move down one place, the ones after
/// `second` two places.
pub fn fn_00d11960(e: &mut Engine, array: u32, first: u32, second: u32) {
    let data = e.mem.u32(array);
    let mut to = data.wrapping_add(first.wrapping_mul(4));
    let end = data.wrapping_add(second.wrapping_mul(4)).wrapping_sub(4);
    if to < end {
        let count = (end - to - 1) / 4 + 1;
        let mut from = to + 4;
        for _ in 0..count {
            let word = e.mem.u32(from);
            e.mem.set_u32(to, word);
            from += 4;
            to += 4;
        }
    }
    let size = e.mem.i32(array + 4).wrapping_sub(2);
    let data_now = e.mem.u32(array);
    let top = data_now.wrapping_add((size as u32).wrapping_mul(4));
    e.mem.set_i32(array + 4, size);
    if to < top {
        let count = (top - to - 1) / 4 + 1;
        let mut from = to + 8;
        for _ in 0..count {
            let word = e.mem.u32(from);
            e.mem.set_u32(to, word);
            from += 4;
            to += 4;
        }
    }
}

// Translated from 00d119c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::getAabb` (Xbox PDB): `getAabbFromNode` of the node whose index is read
/// through `index`.
pub fn get_aabb(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, index: u32, out: u32) {
    let nodes = e.get(this, Hkp3AxisSweep::nodes);
    let node = nodes.wrapping_add(e.mem.u32(index) << 4);
    get_aabb_from_node(e, this, Ptr::new(node), out);
}

// Translated from 00d119e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::calcContentStatistics` (Xbox PDB): reports to the statistics collector
/// (vtable +0x14: kind 3, name, pointer, bytes used, bytes allocated) the node array, the
/// three endpoint arrays and the markers with each one's overlap array. An array is
/// reported when it owns its memory (capacity word not negative) or the collector's byte at
/// +0x10 is set. The second word the vtable slot takes is not read.
pub fn calc_content_statistics(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    collector: u32,
    _unused_2: u32,
) {
    let me = this.addr();
    let include_all = e.mem.u8(collector + 0x10) != 0;
    let report = |e: &mut Engine, name: u32, data: u32, used: u32, capacity: u32, shift: u32| {
        if (capacity as i32) >= 0 || include_all {
            let size = (capacity & CAPACITY_MASK) << shift;
            let allocated = e.call(STATISTICS_ALLOCATED_SIZE, &args![data, size]).u32();
            e.vcall(collector, 0x14, &args![3u32, name, data, used, allocated]);
        }
    };
    let nodes = e.mem.u32(me + 0xa0);
    let capacity = e.mem.u32(me + 0xa8);
    let used = e.mem.u32(me + 0xa4) << 4;
    report(e, NODES_NAME, nodes, used, capacity, 4);
    for array in AXIS_ARRAYS {
        let data = e.mem.u32(me + array);
        let capacity = e.mem.u32(me + array + 8);
        let used = e.mem.u32(me + array + 4) << 2;
        report(e, AXIS_NAME, data, used, capacity, 2);
    }
    let marker_count = e.mem.i32(me + 0xd0);
    let markers = e.mem.u32(me + 0xd8);
    if marker_count != 0 {
        e.call(
            STATISTICS_ADD_NORMAL_CHUNK,
            &args![
                collector,
                MARKERS_NAME,
                markers,
                (marker_count as u32) << 4,
                0u32
            ],
        );
    }
    let mut marker = 0u32;
    while (marker as i32) < e.mem.i32(me + 0xd0) {
        let at = e.mem.u32(me + 0xd8) + marker * 16;
        let data = e.mem.u32(at + 4);
        let capacity = e.mem.u32(at + 0xc);
        let used = e.mem.u32(at + 8).wrapping_mul(2);
        report(e, MARKERS_NAME, data, used, capacity, 1);
        marker += 1;
    }
}

// Translated from 00d11bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Appends the pair of the two nodes' handles (`node + 0xc`) to the pair
/// array.
pub fn fn_00d11bc0(e: &mut Engine, first: u32, second: u32, pairs: u32) {
    push_handle_pair(e, pairs, first, second);
}

// Translated from 00d11c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name; the same body as `00d11bc0` (appends the pair of the two nodes' handles).
pub fn fn_00d11c10(e: &mut Engine, first: u32, second: u32, pairs: u32) {
    push_handle_pair(e, pairs, first, second);
}

// Translated from 00d11c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. A new overlap between node `first` and `second`: when `second`'s handle
/// is an object (bit 0 clear) the pair `(first.handle, second.handle)` is appended to
/// `pairs`; when it is a marker (`markerOffset | 1`) the id `index` is appended to that
/// marker's overlap array instead. `markers` is the marker array.
pub fn fn_00d11c60(e: &mut Engine, markers: u32, first: u32, index: u16, second: u32, pairs: u32) {
    let second_handle = e.mem.u32(second + 0xc);
    if second_handle & 1 == 0 {
        push_handle_pair(e, pairs, first, second);
        return;
    }
    let overlap = (second_handle & 0xffff_fffe) + 4 + markers;
    array_make_room(e, overlap, 2);
    let size = e.mem.u32(overlap + 4);
    let slot = e.mem.u32(overlap) + size * 2;
    e.mem.set_u16(slot, index);
    e.mem.set_u32(overlap + 4, size + 1);
}

// Translated from 00d11d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. An overlap between node `first` and `second` ends: when `second` is an
/// object the pair `(first.handle, second.handle)` is appended to `pairs`; when it is a
/// marker, `index` is removed from that marker's overlap array (the last element takes its
/// place).
pub fn fn_00d11d00(e: &mut Engine, markers: u32, first: u32, index: u16, second: u32, pairs: u32) {
    let second_handle = e.mem.u32(second + 0xc);
    if second_handle & 1 == 0 {
        push_handle_pair(e, pairs, first, second);
        return;
    }
    remove_swap_u16(e, (second_handle & 0xffff_fffe) + markers, index);
}

// ---------------------------------------------------------------------------
// The endpoint moves shared by updateAabbs and updateAabbsUint32.
// ---------------------------------------------------------------------------

/// What one object update needs while it moves the six endpoints of node `node_id`.
struct EndpointMove {
    /// The node array and the endpoint array of the axis being moved.
    nodes: u32,
    endpoints: u32,
    /// The moving node (its index and its address in the node array).
    node_id: u32,
    node: u32,
    /// The marker array, and the two pair arrays new and ended overlaps are written to.
    markers: u32,
    enter_pairs: u32,
    exit_pairs: u32,
    /// 0, 1, 2 for x, y, z.
    axis: usize,
}

impl EndpointMove {
    fn min_field(&self) -> u32 {
        AXIS_NODE_FIELDS[self.axis][0]
    }

    fn max_field(&self) -> u32 {
        AXIS_NODE_FIELDS[self.axis][1]
    }
}

/// `*dst = *(dst - 4)` or `*(dst + 4)`: one endpoint (4 bytes) moves.
fn copy_endpoint(e: &mut Engine, from: u32, to: u32) {
    let word = e.mem.u32(from);
    e.mem.set_u32(to, word);
}

/// Adds `delta` to the 16-bit node field at `address`.
fn add_u16(e: &mut Engine, address: u32, delta: i16) {
    let v = e.mem.u16(address);
    e.mem.set_u16(address, v.wrapping_add(delta as u16));
}

/// Whether two nodes overlap on the two axes other than `axis`: the sign bits of the
/// differences of their endpoint indices are clear. The x axis tests the y and z words
/// together as 32-bit differences (borrows cross the halves); y and z test four 16-bit
/// differences.
fn overlaps_other_axes(e: &Engine, axis: usize, moving: u32, other: u32) -> bool {
    if axis == 0 {
        let high_other = e.mem.u32(other + 4).wrapping_sub(e.mem.u32(moving));
        let high_moving = e.mem.u32(moving + 4).wrapping_sub(e.mem.u32(other));
        return (high_other | high_moving) & 0x8000_8000 == 0;
    }
    // The other two axes' (min, max) fields: y and z for x is handled above; for y it is
    // z and x, for z it is x and y.
    let pairs: [[u32; 2]; 2] = if axis == 1 {
        [[2, 6], [8, 10]]
    } else {
        [[8, 10], [0, 4]]
    };
    let mut bits = 0u16;
    for [min_field, max_field] in pairs {
        bits |= e
            .mem
            .u16(other + max_field)
            .wrapping_sub(e.mem.u16(moving + min_field));
        bits |= e
            .mem
            .u16(moving + max_field)
            .wrapping_sub(e.mem.u16(other + min_field));
    }
    bits & 0x8000 == 0
}

/// A new overlap with `other`: the x axis goes through `00d11c60` (which can add to a
/// marker), the others append the handle pair (`00d11bc0`).
fn notify_enter(e: &mut Engine, c: &EndpointMove, other: u32) {
    if c.axis == 0 {
        fn_00d11c60(e, c.markers, c.node, c.node_id as u16, other, c.enter_pairs);
    } else {
        fn_00d11bc0(e, c.node, other, c.enter_pairs);
    }
}

/// An overlap with `other` ended: `00d11d00` for x, `00d11c10` for y and z.
fn notify_exit(e: &mut Engine, c: &EndpointMove, other: u32) {
    if c.axis == 0 {
        fn_00d11d00(e, c.markers, c.node, c.node_id as u16, other, c.exit_pairs);
    } else {
        fn_00d11c10(e, c.node, other, c.exit_pairs);
    }
}

/// Moves the node's minimum endpoint of this axis to `value`, down (to lower indices) when
/// the endpoints before it are above it. Passing an other node's maximum endpoint starts an
/// overlap when the other two axes overlap; equal values are ordered by node index.
fn move_min_endpoint_down(e: &mut Engine, c: &EndpointMove, value: u32) {
    let min_field = c.min_field();
    let max_field = c.max_field();
    let mut index = e.mem.u16(c.node + min_field) as u32;
    let mut at = c.endpoints + index * 4;
    let mut previous = e.mem.u16(at - 4) as u32;
    while value < previous {
        let other = c.nodes + (e.mem.u16(at - 2) as u32) * 16;
        copy_endpoint(e, at - 4, at);
        if previous & 1 == 0 {
            e.mem.set_u16(other + min_field, index as u16);
        } else {
            e.mem.set_u16(other + max_field, index as u16);
            if overlaps_other_axes(e, c.axis, c.node, other) {
                notify_enter(e, c, other);
            }
        }
        index = index.wrapping_sub(1);
        at -= 4;
        previous = e.mem.u16(at - 4) as u32;
    }
    while value == previous {
        let before = e.mem.u16(at - 2) as u32;
        if before <= c.node_id {
            break;
        }
        copy_endpoint(e, at - 4, at);
        e.mem
            .set_u16(c.nodes + before * 16 + min_field, index as u16);
        index = index.wrapping_sub(1);
        at -= 4;
        previous = e.mem.u16(at - 4) as u32;
    }
    e.mem.set_u16(at, value as u16);
    e.mem.set_u16(at + 2, c.node_id as u16);
    e.mem.set_u16(c.node + min_field, index as u16);
}

/// Moves the node's maximum endpoint of this axis to `value`: up first (to higher indices,
/// passing an other node's minimum endpoint starts an overlap), then down from where it
/// stands (passing an other node's minimum endpoint ends one).
fn move_max_endpoint(e: &mut Engine, c: &EndpointMove, value: u32) {
    let min_field = c.min_field();
    let max_field = c.max_field();
    let mut index = e.mem.u16(c.node + max_field) as u32;
    let mut at = c.endpoints + index * 4;
    let mut next = e.mem.u16(at + 4) as u32;
    while next < value {
        let other = c.nodes + (e.mem.u16(at + 6) as u32) * 16;
        copy_endpoint(e, at + 4, at);
        index = index.wrapping_add(1);
        if next & 1 == 0 {
            add_u16(e, other + min_field, -1);
            if overlaps_other_axes(e, c.axis, c.node, other) {
                notify_enter(e, c, other);
            }
        } else {
            add_u16(e, other + max_field, -1);
        }
        at += 4;
        next = e.mem.u16(at + 4) as u32;
    }
    while value == next {
        let after = e.mem.u16(at + 6) as u32;
        if !(after < c.node_id && after != 0) {
            break;
        }
        copy_endpoint(e, at + 4, at);
        add_u16(e, c.nodes + after * 16 + max_field, -1);
        next = e.mem.u16(at + 8) as u32;
        index = index.wrapping_add(1);
        at += 4;
    }
    let mut previous = e.mem.u16(at - 4) as u32;
    while value < previous {
        let other = c.nodes + (e.mem.u16(at - 2) as u32) * 16;
        copy_endpoint(e, at - 4, at);
        index = index.wrapping_sub(1);
        if previous & 1 == 0 {
            add_u16(e, other + min_field, 1);
            if overlaps_other_axes(e, c.axis, c.node, other) {
                notify_exit(e, c, other);
            }
        } else {
            add_u16(e, other + max_field, 1);
        }
        at -= 4;
        previous = e.mem.u16(at - 4) as u32;
    }
    while value == previous {
        let before = e.mem.u16(at - 2) as u32;
        if before <= c.node_id {
            break;
        }
        copy_endpoint(e, at - 4, at);
        add_u16(e, c.nodes + before * 16 + max_field, 1);
        index = index.wrapping_sub(1);
        at -= 4;
        previous = e.mem.u16(at - 4) as u32;
    }
    e.mem.set_u16(c.node + max_field, index as u16);
    e.mem.set_u16(at, value as u16);
    e.mem.set_u16(at + 2, c.node_id as u16);
}

/// Moves the node's minimum endpoint of this axis up to `value`: passing an other node's
/// maximum endpoint ends an overlap.
fn move_min_endpoint_up(e: &mut Engine, c: &EndpointMove, value: u32) {
    let min_field = c.min_field();
    let max_field = c.max_field();
    let mut index = e.mem.u16(c.node + min_field) as u32;
    let mut at = c.endpoints + index * 4;
    let mut next = e.mem.u16(at + 4) as u32;
    while next < value {
        let other = c.nodes + (e.mem.u16(at + 6) as u32) * 16;
        copy_endpoint(e, at + 4, at);
        index = index.wrapping_add(1);
        if next & 1 == 0 {
            add_u16(e, other + min_field, -1);
        } else {
            add_u16(e, other + max_field, -1);
            if overlaps_other_axes(e, c.axis, c.node, other) {
                notify_exit(e, c, other);
            }
        }
        at += 4;
        next = e.mem.u16(at + 4) as u32;
    }
    while value == next {
        let after = e.mem.u16(at + 6) as u32;
        if c.node_id <= after {
            break;
        }
        copy_endpoint(e, at + 4, at);
        add_u16(e, c.nodes + after * 16 + min_field, -1);
        index = index.wrapping_add(1);
        at += 4;
        next = e.mem.u16(at + 4) as u32;
    }
    e.mem.set_u16(c.node + min_field, index as u16);
    e.mem.set_u16(at, value as u16);
    e.mem.set_u16(at + 2, c.node_id as u16);
}

/// The new endpoint values of one object, per axis: (minimum, maximum).
type EndpointValues = [(u32, u32); 3];

/// Moves all six endpoints of the object `node_id` to `values`, axis by axis in the order
/// x, y, z; per axis: minimum down, maximum up then down, minimum up.
fn move_object_endpoints(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    node_id: u32,
    values: &EndpointValues,
    enter_pairs: u32,
    exit_pairs: u32,
) {
    let nodes = e.get(this, Hkp3AxisSweep::nodes);
    let markers = e.get(this, Hkp3AxisSweep::markers);
    for axis in 0..3 {
        let endpoints = e.mem.u32(this.addr() + AXIS_ARRAYS[axis]);
        let c = EndpointMove {
            nodes,
            endpoints,
            node_id,
            node: nodes + node_id * 16,
            markers,
            enter_pairs,
            exit_pairs,
            axis,
        };
        let (minimum, maximum) = values[axis];
        move_min_endpoint_down(e, &c, minimum);
        move_max_endpoint(e, &c, maximum);
        move_min_endpoint_up(e, &c, minimum);
    }
}

// Translated from 00d12040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::updateAabbs` (Xbox PDB): for each of `count` objects (`handles` holds
/// pointers to their handles, whose first word is the node index; `boxes` holds two
/// `hkVector4`s per object, min then max) quantises the box with the float offsets and
/// scale — add the offset, multiply by the scale, clamp to `[0, 65532]`, add 65536, take
/// the bits `>> 7` — and moves the node's endpoints there. Minimum values are even
/// (`& 0xfffe`), maximum values odd (`| 1`). New overlaps are appended to `enter_pairs`
/// (or to a marker's list), ended ones to `exit_pairs`.
pub fn update_aabbs(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    handles: u32,
    boxes: u32,
    count: i32,
    enter_pairs: u32,
    exit_pairs: u32,
) {
    let me = this.addr();
    let end = handles.wrapping_add((count as u32).wrapping_mul(4));
    let mut handle = handles;
    let mut aabb = boxes;
    let zero = [0.0f32; 4];
    while handle < end {
        let limit = load_vec4(e, QUANTISE_MAX);
        let add = load_vec4(e, QUANTISE_ADD);
        let scale = load_vec4(e, me + SCALE);
        let quantise = |v: Vec4, offset: Vec4| -> Vec4 {
            add4(maxps(minps(mul4(add4(v, offset), scale), limit), zero), add)
        };
        let low = quantise(load_vec4(e, aabb), load_vec4(e, me + OFFSET_LOW));
        let high = quantise(load_vec4(e, aabb + 0x10), load_vec4(e, me + OFFSET_HIGH));
        let low_bits = low.map(|v| v.to_bits() >> 7);
        let high_bits = high.map(|v| v.to_bits() >> 7);
        let values: EndpointValues =
            [0usize, 1, 2].map(|axis| (low_bits[axis] & 0xfffe, (high_bits[axis] & 0xffff) | 1));
        let node_id = e.mem.u32(e.mem.u32(handle));
        move_object_endpoints(e, this, node_id, &values, enter_pairs, exit_pairs);
        aabb += 0x20;
        handle += 4;
    }
}

// Translated from 00d12b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::updateAabbsUint32` (Xbox PDB): the same as `updateAabbs` for boxes of
/// eight 32-bit integers per object (min x, y, z, w, max x, y, z, w): the endpoint value is
/// `value >> 15`, even (`& 0xfffe`) for a minimum; for a maximum it is one more unless it is
/// `0xffff`, then odd (`| 1`).
pub fn update_aabbs_uint32(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    handles: u32,
    boxes: u32,
    count: i32,
    enter_pairs: u32,
    exit_pairs: u32,
) {
    let end = handles.wrapping_add((count as u32).wrapping_mul(4));
    let mut handle = handles;
    let mut aabb = boxes;
    while handle < end {
        let word = |e: &Engine, i: u32| e.mem.u32(aabb + 4 * i);
        let maximum = |e: &Engine, i: u32| {
            let mut v = word(e, i) >> 15;
            if v != 0xffff {
                v += 1;
            }
            v | 1
        };
        let values: EndpointValues = [
            ((word(e, 0) >> 15) & 0xfffe, maximum(e, 4)),
            ((word(e, 1) >> 15) & 0xfffe, maximum(e, 5)),
            ((word(e, 2) >> 15) & 0xfffe, maximum(e, 6)),
        ];
        let node_id = e.mem.u32(e.mem.u32(handle));
        move_object_endpoints(e, this, node_id, &values, enter_pairs, exit_pairs);
        aabb += 0x20;
        handle += 4;
    }
}

// Translated from 00d11da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::hkpBpAxis::insert` (Xbox PDB): `this` is one axis's endpoint `hkArray`.
/// Makes room for two endpoints, the old last one (the sentinel) moving up two places, and
/// puts the node `node_id`'s maximum endpoint (`max_value`) and then its minimum endpoint
/// (`min_value`) where they sort, by value and then by node index, moving the larger ones up.
/// The indices they land on are stored as 16-bit values at `out_min` and `out_max`. The first
/// word is not read.
pub fn hkp_bp_axis_insert(
    e: &mut Engine,
    this: u32,
    _unused_1: u32,
    node_id: u32,
    min_value: u16,
    max_value: u16,
    out_min: u32,
    out_max: u32,
) {
    let size = e.mem.i32(this + 4).wrapping_add(2);
    array_reserve_at_least(e, this, size, 4);
    let data = e.mem.u32(this);
    let mut at = data
        .wrapping_add((size as u32).wrapping_mul(4))
        .wrapping_sub(0xc);
    e.mem.set_i32(this + 4, size);
    copy_endpoint(e, at, at + 8);
    at = at.wrapping_sub(4);
    while max_value < e.mem.u16(at) {
        copy_endpoint(e, at, at + 8);
        at = at.wrapping_sub(4);
    }
    while max_value == e.mem.u16(at) && node_id < e.mem.u16(at + 2) as u32 {
        copy_endpoint(e, at, at + 8);
        at = at.wrapping_sub(4);
    }
    e.mem.set_u16(at + 8, max_value);
    e.mem.set_u16(at + 0xa, node_id as u16);
    let data = e.mem.u32(this);
    e.mem.set_u16(
        out_max,
        (at.wrapping_sub(data) as i32 >> 2).wrapping_add(2) as u16,
    );
    while min_value < e.mem.u16(at) {
        copy_endpoint(e, at, at + 4);
        at = at.wrapping_sub(4);
    }
    while min_value == e.mem.u16(at) && node_id < e.mem.u16(at + 2) as u32 {
        copy_endpoint(e, at, at + 4);
        at = at.wrapping_sub(4);
    }
    e.mem.set_u16(at + 6, node_id as u16);
    e.mem.set_u16(at + 4, min_value);
    let data = e.mem.u32(this);
    e.mem.set_u16(
        out_min,
        (at.wrapping_sub(data) as i32 >> 2).wrapping_add(1) as u16,
    );
}

// Translated from 00d11e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::hkpBpAxis::insertTail` (Xbox PDB): `this` is one axis's endpoint
/// `hkArray`. Makes room for `count` endpoints in front of the last one (the sentinel, which
/// moves to the end) and shifts the existing endpoints whose value is above the last new
/// endpoint's value (`new_endpoints[count - 1]`) up by `count`, pointing each moved
/// endpoint's node field (looked up in the table `011b37a4` by axis and the endpoint's
/// maximum bit) at its new index. Returns the index just above the first endpoint left in
/// place.
pub fn hkp_bp_axis_insert_tail(
    e: &mut Engine,
    this: u32,
    nodes: u32,
    axis: u32,
    new_endpoints: u32,
    count: i32,
) -> i32 {
    let old_size = e.mem.i32(this + 4);
    let size = old_size.wrapping_add(count);
    array_reserve_at_least(e, this, size, 4);
    e.mem.set_i32(this + 4, size);
    let last_value = e.mem.u16(
        new_endpoints
            .wrapping_add((count as u32).wrapping_mul(4))
            .wrapping_sub(4),
    );
    let data = e.mem.u32(this);
    let mut to = data
        .wrapping_add((size as u32).wrapping_mul(4))
        .wrapping_sub(4);
    let mut from = data
        .wrapping_add((old_size as u32).wrapping_mul(4))
        .wrapping_sub(4);
    copy_endpoint(e, from, to);
    let update_node_field = |e: &mut Engine, from: u32, to: u32| {
        let flag = (e.mem.u16(from) & 1) as u32;
        let node = e.mem.u16(from + 2) as u32;
        let field = e
            .mem
            .u32(NODE_FIELD_TABLE + (flag + axis.wrapping_mul(2)) * 4);
        let data = e.mem.u32(this);
        e.mem.set_u16(
            nodes.wrapping_add(node << 4).wrapping_add(field),
            (to.wrapping_sub(data) as i32 >> 2) as u16,
        );
    };
    update_node_field(e, from, to);
    from = from.wrapping_sub(4);
    to = to.wrapping_sub(4);
    while e.mem.u16(from) > last_value {
        copy_endpoint(e, from, to);
        update_node_field(e, from, to);
        from = from.wrapping_sub(4);
        to = to.wrapping_sub(4);
    }
    let data = e.mem.u32(this);
    (to.wrapping_sub(data) as i32 >> 2).wrapping_add(1)
}

// Translated from 00d11fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::getAllAabbs` (Xbox PDB): fills the `hkArray` of 32-byte boxes `out` with
/// the box of every node that is not a marker (bit 0 of its handle clear), growing it to
/// exactly the needed capacity when it is smaller.
pub fn get_all_aabbs(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, out: u32) {
    let nodes_total = e.get(this, Hkp3AxisSweep::node_count);
    let wanted = nodes_total.wrapping_sub(e.get(this, Hkp3AxisSweep::marker_count));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = (e.mem.u32(out + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let heap = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE, &args![heap, out, wanted, 0x20u32]);
    }
    let count = e.get(this, Hkp3AxisSweep::node_count);
    let size = count.wrapping_sub(e.get(this, Hkp3AxisSweep::marker_count));
    e.mem.set_i32(out + 4, size);
    let mut written = 0u32;
    let mut index = 0i32;
    let mut offset = 0u32;
    while index < e.get(this, Hkp3AxisSweep::node_count) {
        let node = e.get(this, Hkp3AxisSweep::nodes).wrapping_add(offset);
        if e.mem.u8(node + 0xc) & 1 == 0 {
            let target = e.mem.u32(out).wrapping_add(written);
            get_aabb_from_node(e, this, Ptr::new(node), target);
            written = written.wrapping_add(0x20);
        }
        index += 1;
        offset = offset.wrapping_add(0x10);
    }
}

// ---------------------------------------------------------------------------
// The sweeps between two sorted lists of boxes.
// ---------------------------------------------------------------------------

/// Whether the two list records overlap on y and z: the sign bits of
/// `(first.high - second.low) | (second.high - first.low)` for both 16-bit halves are clear.
/// A record is `{ low: u32, high: u32, min_x: u16, max_x: u16, handle: u32 }`.
fn records_overlap(e: &Engine, first: u32, second: u32) -> bool {
    let high_low = e.mem.u32(first + 4).wrapping_sub(e.mem.u32(second));
    let low_high = e.mem.u32(second + 4).wrapping_sub(e.mem.u32(first));
    (high_low | low_high) & 0x8000_8000 == 0
}

/// Appends `id` to the overlap array of the marker at `marker`.
fn marker_overlap_add(e: &mut Engine, marker: u32, id: u16) {
    let overlap = marker + 4;
    array_make_room(e, overlap, 2);
    let size = e.mem.u32(overlap + 4);
    let slot = e.mem.u32(overlap).wrapping_add(size.wrapping_mul(2));
    e.mem.set_u16(slot, id);
    e.mem.set_u32(overlap + 4, size.wrapping_add(1));
}

/// What `collide1Axis` does once two records overlap on y and z: when `other` is an object
/// the pair `(object.handle, other.handle)` is appended to `pairs`; when it is a marker
/// (bit 0 of its handle set) `mode` 0 appends the node id that `object`'s handle points at to
/// the marker's overlap array, `mode` 1 does nothing and any other mode removes that id.
fn collide_pair(e: &mut Engine, markers: u32, mode: i32, pairs: u32, object: u32, other: u32) {
    let other_handle = e.mem.u32(other + 0xc);
    if other_handle & 1 == 0 {
        let object_handle = e.mem.u32(object + 0xc);
        push_pair(e, pairs, object_handle, other_handle);
        return;
    }
    if mode == 1 {
        return;
    }
    let id = e.mem.u32(e.mem.u32(object + 0xc)) as u16;
    let marker = (other_handle & 0xffff_fffe).wrapping_add(markers);
    if mode == 0 {
        marker_overlap_add(e, marker, id);
    } else {
        remove_swap_u16(e, marker, id);
    }
}

// Translated from 00d13620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::collide1Axis` (Xbox PDB): merges two lists of 16-byte records
/// (`{ low: u32, high: u32, min_x: u16, max_x: u16, handle: u32 }`, sorted by `min_x`) by
/// `min_x`. The record that starts later is tested against every record of the other list
/// that starts before it ends, comparing y and z (the sign bits of the 32-bit differences,
/// mask `0x80008000`); object pairs go to `pairs`, and a marker record takes the other
/// record's node id into its overlap array (`mode` 0) or loses it (`mode` other than 0 and 1;
/// 1 does nothing). It runs at least once, then until the count of the list it advanced
/// reaches 0.
pub fn collide_1_axis(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    list_a: u32,
    count_a: i32,
    list_b: u32,
    count_b: i32,
    mode: i32,
    pairs: u32,
) {
    let markers = e.get(this, Hkp3AxisSweep::markers);
    let mut a = list_a;
    let mut count_a = count_a;
    let mut b = list_b;
    let mut count_b = count_b;
    loop {
        let remaining = if e.mem.u16(a + 8) > e.mem.u16(b + 8) {
            let b_max = e.mem.u16(b + 0xa);
            if e.mem.u16(a + 8) < b_max {
                let mut candidate = a;
                loop {
                    if records_overlap(e, b, candidate) {
                        collide_pair(e, markers, mode, pairs, candidate, b);
                    }
                    let next_min = e.mem.u16(candidate + 0x18);
                    candidate = candidate.wrapping_add(0x10);
                    if next_min >= b_max {
                        break;
                    }
                }
            }
            b = b.wrapping_add(0x10);
            count_b = count_b.wrapping_sub(1);
            count_b
        } else {
            let a_max = e.mem.u16(a + 0xa);
            if e.mem.u16(b + 8) < a_max {
                let mut candidate = b;
                loop {
                    if records_overlap(e, a, candidate) {
                        collide_pair(e, markers, mode, pairs, a, candidate);
                    }
                    let next_min = e.mem.u16(candidate + 0x18);
                    candidate = candidate.wrapping_add(0x10);
                    if next_min >= a_max {
                        break;
                    }
                }
            }
            a = a.wrapping_add(0x10);
            count_a = count_a.wrapping_sub(1);
            count_a
        };
        if remaining <= 0 {
            break;
        }
    }
}

// Translated from 00d138a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::collide1Axis` (the overload that sweeps one list against itself; cdecl,
/// no `this`): for each of the first `count - 1` records, every later record that starts
/// before its `max_x` and overlaps it on y and z, and is not a marker, adds the pair
/// `(record.handle, later.handle)` to `pairs`. The first word is not read.
pub fn collide_1_axis_one_list(e: &mut Engine, _unused_1: u32, list: u32, count: i32, pairs: u32) {
    let mut remaining = count.wrapping_sub(1);
    if remaining <= 0 {
        return;
    }
    let mut current = list;
    loop {
        let max_x = e.mem.u16(current + 0xa);
        let following = current.wrapping_add(0x10);
        let mut candidate = following;
        if e.mem.u16(current + 0x18) < max_x {
            loop {
                if records_overlap(e, current, candidate) && e.mem.u8(candidate + 0xc) & 1 == 0 {
                    let first = e.mem.u32(current + 0xc);
                    let second = e.mem.u32(candidate + 0xc);
                    push_pair(e, pairs, first, second);
                }
                let next_min = e.mem.u16(candidate + 0x18);
                candidate = candidate.wrapping_add(0x10);
                if next_min >= max_x {
                    break;
                }
            }
        }
        current = following;
        remaining -= 1;
        if remaining <= 0 {
            break;
        }
    }
}

// Translated from 00d13950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::calcAabbCacheInternal` (role name; no Xbox PDB entry): `nodes` is an
/// `hkArray` of node pointers, `cache` a block of `getAabbCacheSize` bytes. It sets up three
/// `hkArray`s in the block's first 0x24 bytes (x, y, z; each over its own `2 * count + 2`
/// endpoints in the storage that follows, not owned), puts a zero sentinel endpoint in each,
/// copies the min and max endpoints (the 4-byte value with node index) of every node out of
/// the sweep's arrays, sorts each array after its sentinel with `quickSortRecursive`, and ends
/// each with the `0x0000fffc` sentinel.
pub fn calc_aabb_cache_internal(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, nodes: u32, cache: u32) {
    let me = this.addr();
    let count = e.mem.i32(nodes + 4);
    let capacity = count.wrapping_mul(2).wrapping_add(2) as u32;
    let storage = cache.wrapping_add(0x24);
    let arrays = [cache, cache + 0xc, cache + 0x18];
    if cache != 0 {
        for (index, array) in arrays.iter().enumerate() {
            let data = storage.wrapping_add(capacity.wrapping_mul(4 * index as u32));
            e.mem.set_u32(*array, data);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, capacity | NOT_OWNED);
        }
    }
    for array in arrays {
        push_word(e, array, 0);
    }
    // The stack slot that holds the comparator argument of the sort keeps the address of the
    // last endpoint copied (or `this`); the call passes it with its low byte cleared.
    let mut scratch = me;
    let list = e.mem.u32(nodes);
    let mut index = 0i32;
    while index < count {
        let node = e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
        for (axis, array) in arrays.iter().enumerate() {
            let endpoints = e.mem.u32(me + AXIS_ARRAYS[axis]);
            for field in AXIS_NODE_FIELDS[axis] {
                let source = endpoints.wrapping_add((e.mem.u16(node + field) as u32) * 4);
                scratch = source;
                let value = e.mem.u32(source);
                push_word(e, *array, value);
            }
        }
        index += 1;
    }
    for array in arrays {
        let size = e.mem.i32(array + 4);
        if size.wrapping_sub(1) > 1 {
            let base = e.mem.u32(array).wrapping_add(4);
            quick_sort_recursive(e, base, 0, size.wrapping_sub(2), scratch & 0xffff_ff00);
        }
    }
    for array in arrays {
        push_word(e, array, 0xfffc);
    }
}

// Translated from 00d13bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::hkp3AxisSweep` (Xbox PDB; the constructor, not in the PDB's type list):
/// `this` after the base constructor `00d97010`. Stores the vtable and the world box
/// (`world_min`, `world_max`, `hkVector4`s), starts all arrays empty, and sets
/// `bits = floor(log2(markers))` (1 when `marker_request` is 0, -1 when it is negative).
/// A 23-step bisection finds the float exponent offset (stored at +0xdc) from the
/// quantisation of a test value; the offsets and the scale are cleared. Then it reserves the
/// arrays, adds the sentinel node 0, the endpoint arrays' sentinels (value 0), the
/// `(1 << bits) - 1` marker nodes — each with its own min and max x endpoints at
/// `(index + 1) << (16 - bits)` — the end sentinel endpoints (value `0xfffd`) and fills node
/// 0's maximum endpoint indices. The pointer to node 0 is taken before the marker nodes are
/// added, as the code does; it is only valid when the node array does not move.
pub fn hkp_3_axis_sweep(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    world_min: u32,
    world_max: u32,
    marker_request: i32,
) -> Ptr<Hkp3AxisSweep> {
    let me = this.addr();
    let nodes_array = me + 0xa0;
    let x_array = me + AXIS_ARRAYS[0];
    let y_array = me + AXIS_ARRAYS[1];
    let z_array = me + AXIS_ARRAYS[2];
    e.call(BASE_CONSTRUCTOR, &args![me]);
    e.mem.set_u32(me, VTABLE);
    copy_vec4(e, world_min, me + WORLD_MIN);
    copy_vec4(e, world_max, me + WORLD_MAX);
    for array in [nodes_array, x_array, y_array, z_array] {
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
    }
    e.mem.set_u32(me + 0xd0, 0);
    e.mem.set_u32(me + 0xd4, 0);
    let mut requested = if marker_request == 0 {
        1
    } else {
        marker_request
    };
    let mut bits = -1i32;
    if requested > 0 {
        loop {
            requested >>= 1;
            bits += 1;
            if requested <= 0 {
                break;
            }
        }
    }
    array_reserve_at_least(e, nodes_array, 0xff, 0x10);
    let exponent_offset = search_exponent_offset(e, me);
    store_vec4(e, me + SCALE, [0.0; 4]);
    store_vec4(e, me + OFFSET_LOW, [0.0; 4]);
    store_vec4(e, me + OFFSET_HIGH, [0.0; 4]);
    e.mem.set_f32(me + 0xdc, exponent_offset);
    array_make_room(e, nodes_array, 0x10);
    let node_count = e.mem.u32(me + 0xa4);
    let sentinel = e.mem.u32(me + 0xa0).wrapping_add(node_count << 4);
    e.mem.set_u32(me + 0xa4, node_count.wrapping_add(1));
    e.mem.set_u16(sentinel + 2, 0);
    e.mem.set_u16(sentinel, 0);
    e.mem.set_u16(sentinel + 8, 0);
    e.mem.set_u32(sentinel + 0xc, 0);
    let marker_total = 1u32 << (bits as u32 & 31);
    array_reserve_at_least(
        e,
        x_array,
        marker_total.wrapping_mul(2).wrapping_add(0x1fe) as i32,
        4,
    );
    array_reserve_at_least(e, y_array, 0x200, 4);
    array_reserve_at_least(e, z_array, 0x200, 4);
    for array in [x_array, y_array, z_array] {
        push_word(e, array, 0);
    }
    e.mem.set_i32(me + 0xd4, bits);
    let marker_count = marker_total.wrapping_sub(1) as i32;
    e.mem.set_i32(me + 0xd0, marker_count);
    e.mem.set_u32(me + 0xd8, 0);
    if marker_count != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let heap = e.mem.u32(router + 0x10);
        let markers = e
            .call(EASY_ALLOC, &args![heap, (marker_count as u32) << 4])
            .u32();
        e.mem.set_u32(me + 0xd8, markers);
    }
    if e.mem.i32(me + 0xd0) > 0 {
        let mut index = 0u32;
        let mut offset = 0u32;
        loop {
            let marker = e.mem.u32(me + 0xd8).wrapping_add(offset);
            e.mem.set_u32(marker + 4, 0);
            e.mem.set_u32(marker + 8, 0);
            e.mem.set_u32(marker + 0xc, NOT_OWNED);
            let node_id = e.mem.u16(me + 0xa4);
            let shift = 0x10u32.wrapping_sub(bits as u32) & 31;
            let value = index.wrapping_add(1).wrapping_shl(shift) as u16;
            e.mem.set_u16(marker, node_id);
            e.mem.set_u16(marker + 2, value);
            let endpoint = (node_id as u32) << 16 | value as u32;
            array_make_room(e, nodes_array, 0x10);
            let count = e.mem.u32(me + 0xa4);
            e.mem.set_u32(me + 0xa4, count.wrapping_add(1));
            let node = e.mem.u32(me + 0xa0).wrapping_add(count << 4);
            let x_index = e.mem.u16(me + 0xb0);
            e.mem.set_u16(node + 8, x_index);
            push_word(e, x_array, endpoint);
            let x_index = e.mem.u16(me + 0xb0);
            e.mem.set_u16(node + 0xa, x_index);
            push_word(e, x_array, endpoint | 1);
            e.mem.set_u32(node + 0xc, offset | 1);
            e.mem.set_u16(node + 2, 0);
            e.mem.set_u16(node, 0);
            e.mem.set_u16(node + 6, 1);
            e.mem.set_u16(node + 4, 1);
            index = index.wrapping_add(1);
            offset = offset.wrapping_add(0x10);
            if (index as i32) >= e.mem.i32(me + 0xd0) {
                break;
            }
        }
    }
    let first = e.mem.u32(me + 0xa0);
    copy_vec4(e, first, sentinel);
    let x_max = e.mem.u16(me + 0xb0);
    e.mem.set_u16(sentinel + 0xa, x_max);
    let y_max = e.mem.u16(me + 0xbc);
    e.mem.set_u16(sentinel + 4, y_max);
    let z_max = e.mem.u16(me + 0xc8);
    e.mem.set_u16(sentinel + 6, z_max);
    for array in [x_array, y_array, z_array] {
        push_word(e, array, 0xfffd);
    }
    this
}

/// The constructor's bisection between 11.0 and 10.0 (floats from the exe): 23 times it takes
/// the middle `(high + low) * 0.5`, quantises `middle + 1.0` the way `updateAabbs` does
/// (times a scale of 1.0, clamped to `[0, 65532]`, plus 65536, the bits `>> 7 | 1`) and moves
/// the upper or lower end to it when that value is at or above 12 or below. The result is
/// `(high + low) * 0.5 - 11.0`.
fn search_exponent_offset(e: &mut Engine, me: u32) -> f32 {
    let mut upper: f32 = e.global(ELEVEN_FLOAT);
    let mut lower: f32 = e.global(TEN_FLOAT);
    let unit: f32 = e.global(UNIT_FLOAT);
    store_vec4(e, me + SCALE, [unit; 4]);
    let scale = load_vec4(e, me + SCALE);
    let limit = load_vec4(e, QUANTISE_MAX);
    let add = load_vec4(e, QUANTISE_ADD);
    let half: f64 = e.global(HALF_DOUBLE);
    for _ in 0..23 {
        let middle = ((upper as f64 + lower as f64) * half) as f32;
        let shifted = (middle as f64 + 1.0) as f32;
        let value = add4(
            maxps(
                minps(mul4(add4([shifted; 4], [0.0; 4]), scale), limit),
                [0.0; 4],
            ),
            add,
        );
        if ((value[0].to_bits() >> 7) | 1) as u16 >= 0xc {
            upper = middle;
        } else {
            lower = middle;
        }
    }
    let eleven: f64 = e.global(ELEVEN_DOUBLE);
    ((upper as f64 + lower as f64) * half - eleven) as f32
}

// Translated from 00d19fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpBpEndPoint, less>` (Xbox PDB): sorts the endpoints
/// (4 bytes: `u16` value, `u16` node index) at `base[lo..=hi]` ascending by value, then node
/// index. `comparator` (the empty `less` passed by value) is only handed on.
pub fn quick_sort_recursive(e: &mut Engine, base: u32, lo: i32, hi: i32, comparator: u32) {
    let at = |index: i32| base.wrapping_add((index as u32).wrapping_mul(4));
    let mut lo = lo;
    loop {
        let pivot = e.mem.u32(at(lo.wrapping_add(hi) >> 1));
        let pivot_value = pivot as u16;
        let pivot_id = (pivot >> 16) as u16;
        let mut i = lo;
        let mut j = hi;
        loop {
            loop {
                let value = e.mem.u16(at(i));
                if value > pivot_value || (value == pivot_value && e.mem.u16(at(i) + 2) >= pivot_id)
                {
                    break;
                }
                i = i.wrapping_add(1);
            }
            loop {
                let value = e.mem.u16(at(j));
                if !(pivot_value < value
                    || (pivot_value == value && pivot_id < e.mem.u16(at(j) + 2)))
                {
                    break;
                }
                j = j.wrapping_sub(1);
            }
            if j < i {
                break;
            }
            if j != i {
                let left = e.mem.u32(at(i));
                let right = e.mem.u32(at(j));
                e.mem.set_u32(at(j), left);
                e.mem.set_u32(at(i), right);
            }
            j = j.wrapping_sub(1);
            i = i.wrapping_add(1);
            if i > j {
                break;
            }
        }
        if lo < j {
            quick_sort_recursive(e, base, lo, j, comparator);
        }
        if i >= hi {
            return;
        }
        lo = i;
    }
}

// ---------------------------------------------------------------------------
// Queries and changes that use a temporary bit array of nodes.
// ---------------------------------------------------------------------------

/// The stack-allocated bit array those functions fill with `setBitsBasedOnXInterval`: one bit
/// per node, `(nodeCount >> 5) + 8` words. The block is given back with [`stack_free`].
fn alloc_node_bits(e: &mut Engine, node_count: i32, logical_shift: bool) -> StackBlock {
    let words = if logical_shift {
        ((node_count as u32) >> 5).wrapping_add(8) as i32
    } else {
        (node_count >> 5).wrapping_add(8)
    };
    stack_alloc(e, words.wrapping_mul(4), 2)
}

/// Calls `visit(e, other)` for every node whose bit is set in `bits`, in node order. The loop
/// reads 32 nodes (0x200 bytes) per word and skips eight nodes at a time while the low byte
/// of the word is zero.
fn for_each_marked_node(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    bits: u32,
    mut visit: impl FnMut(&mut Engine, u32),
) {
    let mut cursor = e.get(this, Hkp3AxisSweep::nodes);
    let words = (e.get(this, Hkp3AxisSweep::node_count) >> 5) as u32;
    let end = bits.wrapping_add(words.wrapping_mul(4)).wrapping_add(4);
    let mut at = bits;
    while at < end {
        let mut word = e.mem.u32(at);
        let mut other = cursor;
        while word != 0 {
            if word & 0xff == 0 {
                other = other.wrapping_add(0x80);
                word >>= 8;
                continue;
            }
            if word & 1 != 0 {
                visit(e, other);
            }
            other = other.wrapping_add(0x10);
            word >>= 1;
        }
        cursor = cursor.wrapping_add(0x200);
        at = at.wrapping_add(4);
    }
}

// Translated from 00d14100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::reQuerySingleObject` (Xbox PDB): adds to `pairs` the pair
/// `(object, handle)` for every object whose box overlaps that of the object `object` (a
/// pointer to the handle whose first word is the node index). It marks the nodes whose x
/// interval overlaps with `setBitsBasedOnXInterval` in a temporary bit array and tests the
/// y and z extents of the marked nodes; markers are skipped.
pub fn re_query_single_object(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, object: u32, pairs: u32) {
    let count = e.get(this, Hkp3AxisSweep::node_count);
    let block = alloc_node_bits(e, count, false);
    let id = e.mem.u32(object);
    let node = e.get(this, Hkp3AxisSweep::nodes).wrapping_add(id << 4);
    let x_endpoints = e.get(this, Hkp3AxisSweep::x_endpoints);
    let min_x_value = e.mem.u16(x_endpoints + (e.mem.u16(node + 8) as u32) * 4);
    set_bits_based_on_x_interval(
        e,
        this,
        count,
        min_x_value as i32,
        Ptr::new(node),
        id as u16,
        block.data,
    );
    for_each_marked_node(e, this, block.data, |e, other| {
        if !records_overlap(e, other, node) {
            return;
        }
        let handle = e.mem.u32(other + 0xc);
        if handle & 1 == 0 {
            push_pair(e, pairs, object, handle);
        }
    });
    stack_free(e, &block, 2);
}

// Translated from 00d142b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::addObject` (Xbox PDB, the overload taking an integer box): `aabb` holds
/// eight 32-bit integers (min x, y, z, w, max x, y, z, w). The endpoint values are
/// `value >> 15`; a minimum is masked `& 0xfffe`, a maximum is one more unless it is
/// `0xffff`, then `| 1`. The new node (index = the old node count, handle `handle`, whose
/// first word becomes that index) gets its three pairs of endpoints through
/// `hkpBpAxis::insert`, and `00d10b30` raises the other nodes' indices. Every object whose
/// box overlaps the new one is added to `pairs` as `(handle, other.handle)`; a marker that
/// overlaps it gets the new node's index in its overlap array.
pub fn add_object_ov2(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    handle: u32,
    aabb: u32,
    pairs: u32,
) {
    let me = this.addr();
    let maximum = |e: &Engine, offset: u32| {
        let mut v = e.mem.u32(aabb + offset) >> 15;
        if v != 0xffff {
            v = v.wrapping_add(1);
        }
        v | 1
    };
    let minimum = |e: &Engine, offset: u32| (e.mem.u32(aabb + offset) >> 15) & 0xfffe;
    let max_x = maximum(e, 0x10);
    let max_y = maximum(e, 0x14);
    let max_z = maximum(e, 0x18);
    let min_x = minimum(e, 0);
    let min_y = minimum(e, 4);
    let min_z = minimum(e, 8);
    let node_id = e.get(this, Hkp3AxisSweep::node_count) as u32;
    let nodes_array = me + 0xa0;
    array_make_room(e, nodes_array, 0x10);
    let count = e.mem.u32(nodes_array + 4);
    let node = e.mem.u32(nodes_array).wrapping_add(count << 4);
    e.mem.set_u32(nodes_array + 4, count.wrapping_add(1));
    let nodes = e.mem.u32(nodes_array);
    hkp_bp_axis_insert(
        e,
        me + AXIS_ARRAYS[0],
        nodes,
        node_id,
        min_x as u16,
        max_x as u16,
        node + 8,
        node + 0xa,
    );
    hkp_bp_axis_insert(
        e,
        me + AXIS_ARRAYS[1],
        nodes,
        node_id,
        min_y as u16,
        max_y as u16,
        node,
        node + 4,
    );
    hkp_bp_axis_insert(
        e,
        me + AXIS_ARRAYS[2],
        nodes,
        node_id,
        min_z as u16,
        max_z as u16,
        node + 2,
        node + 6,
    );
    fn_00d10b30(e, me, nodes, node_id, node);
    e.mem.set_u32(node + 0xc, handle);
    e.mem.set_u32(handle, node_id);
    let new_count = e.get(this, Hkp3AxisSweep::node_count);
    let block = alloc_node_bits(e, new_count, false);
    set_bits_based_on_x_interval(
        e,
        this,
        new_count,
        min_x as i32,
        Ptr::new(node),
        node_id as u16,
        block.data,
    );
    for_each_marked_node(e, this, block.data, |e, other| {
        if !records_overlap(e, other, node) {
            return;
        }
        let other_handle = e.mem.u32(other + 0xc);
        if other_handle & 1 == 0 {
            push_pair(e, pairs, handle, other_handle);
        } else {
            let markers = e.get(this, Hkp3AxisSweep::markers);
            let marker = (other_handle & 0xffff_fffe).wrapping_add(markers);
            marker_overlap_add(e, marker, node_id as u16);
        }
    });
    stack_free(e, &block, 2);
}

// Translated from 00d14600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::removeObject` (Xbox PDB): `handle` points at the object's node index.
/// Every object whose box overlaps the node's is added to `pairs` as `(handle,
/// other.handle)`, and a marker that overlaps it loses the node's index from its overlap
/// array. Then the node's six endpoints leave their arrays (`00d11960`), the other nodes'
/// indices drop (`00d10d10`), and when the node was not the last one the last node moves into
/// its place: its endpoints, handle (or marker) and the overlap arrays of the markers its
/// x interval spans are pointed at the new index, and
/// `_fixDeterministicOrderAfterNodeIdWasDecreased` runs. The node array's size becomes the
/// old count minus one (its capacity is only raised, never lowered).
pub fn remove_object(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, handle: u32, pairs: u32) {
    let me = this.addr();
    let count = e.get(this, Hkp3AxisSweep::node_count);
    let id = e.mem.u32(handle);
    let nodes_array = me + 0xa0;
    let node = e.mem.u32(nodes_array).wrapping_add(id << 4);
    let block = alloc_node_bits(e, count, true);
    let x_endpoints = e.get(this, Hkp3AxisSweep::x_endpoints);
    let min_x_value = e.mem.u16(x_endpoints + (e.mem.u16(node + 8) as u32) * 4);
    set_bits_based_on_x_interval(
        e,
        this,
        count,
        min_x_value as i32,
        Ptr::new(node),
        id as u16,
        block.data,
    );
    for_each_marked_node(e, this, block.data, |e, other| {
        if !records_overlap(e, other, node) {
            return;
        }
        let other_handle = e.mem.u32(other + 0xc);
        if other_handle & 1 == 0 {
            push_pair(e, pairs, handle, other_handle);
        } else {
            let markers = e.get(this, Hkp3AxisSweep::markers);
            remove_swap_u16(
                e,
                (other_handle & 0xffff_fffe).wrapping_add(markers),
                id as u16,
            );
        }
    });
    stack_free(e, &block, 2);
    let word = |e: &Engine, offset: u32| e.mem.u16(node + offset) as u32;
    let (min_x, max_x) = (word(e, 8), word(e, 0xa));
    fn_00d11960(e, me + AXIS_ARRAYS[0], min_x, max_x);
    let (min_y, max_y) = (word(e, 0), word(e, 4));
    fn_00d11960(e, me + AXIS_ARRAYS[1], min_y, max_y);
    let (min_z, max_z) = (word(e, 2), word(e, 6));
    fn_00d11960(e, me + AXIS_ARRAYS[2], min_z, max_z);
    let nodes = e.mem.u32(nodes_array);
    fn_00d10d10(e, me, nodes, count as u32, node);
    if id < (count as u32).wrapping_sub(1) {
        let last = e
            .mem
            .u32(nodes_array)
            .wrapping_add((count as u32) << 4)
            .wrapping_sub(0x10);
        copy_vec4(e, last, node);
        let id16 = id as u16;
        let x = e.mem.u32(me + AXIS_ARRAYS[0]);
        let moved_min_x = e.mem.u16(node + 8) as u32;
        e.mem.set_u16(x + moved_min_x * 4 + 2, id16);
        let x = e.mem.u32(me + AXIS_ARRAYS[0]);
        let moved_max_x = e.mem.u16(node + 0xa) as u32;
        e.mem.set_u16(x + moved_max_x * 4 + 2, id16);
        let moved_handle = e.mem.u32(node + 0xc);
        if moved_handle & 1 == 0 {
            for (axis, field) in [(1usize, 0u32), (1, 4), (2, 2), (2, 6)] {
                let endpoints = e.mem.u32(me + AXIS_ARRAYS[axis]);
                let index = e.mem.u16(node + field) as u32;
                e.mem.set_u16(endpoints + index * 4 + 2, id16);
            }
            let moved_handle = e.mem.u32(node + 0xc);
            e.mem.set_u32(moved_handle, id);
        } else {
            let markers = e.get(this, Hkp3AxisSweep::markers);
            e.mem
                .set_u16((moved_handle & 0xffff_fffe).wrapping_add(markers), id16);
        }
        if e.get(this, Hkp3AxisSweep::marker_count) != 0 && e.mem.u8(node + 0xc) & 1 == 0 {
            let moved_min_x = e.mem.u16(node + 8);
            let x = e.mem.u32(me + AXIS_ARRAYS[0]);
            let shift = 0x10u16.wrapping_sub(e.mem.u16(me + 0xd4)) as u32 & 31;
            let mut first = (e.mem.u16(x + moved_min_x as u32 * 4) as u32 >> shift) as i32;
            if first > 0 {
                let markers = e.get(this, Hkp3AxisSweep::markers);
                let marker_node = e
                    .mem
                    .u16(markers.wrapping_add((first as u32) << 4).wrapping_sub(0x10))
                    as u32;
                let nodes = e.mem.u32(nodes_array);
                if e.mem.u16(nodes + (marker_node << 4) + 0xa) > moved_min_x {
                    first -= 1;
                }
            }
            let moved_max_x = e.mem.u16(node + 0xa) as u32;
            let last_marker = (e.mem.u16(x + moved_max_x * 4) as u32 >> shift) as i32 - 1;
            if first <= last_marker {
                let old_id = (count.wrapping_sub(1) as u32) as u16;
                let mut offset = (first as u32) << 4;
                let mut remaining = last_marker.wrapping_sub(first).wrapping_add(1);
                loop {
                    let markers = e.get(this, Hkp3AxisSweep::markers);
                    let marker = markers.wrapping_add(offset);
                    let size = e.mem.i32(marker + 8);
                    let data = e.mem.u32(marker + 4);
                    let mut found = -1i32;
                    let mut index = 0i32;
                    while index < size {
                        if e.mem.u16(data.wrapping_add((index as u32).wrapping_mul(2))) == old_id {
                            found = index;
                            break;
                        }
                        index += 1;
                    }
                    let data = e.mem.u32(marker + 4);
                    e.mem
                        .set_u16(data.wrapping_add((found as u32).wrapping_mul(2)), id16);
                    offset = offset.wrapping_add(0x10);
                    remaining -= 1;
                    if remaining == 0 {
                        break;
                    }
                }
            }
        }
        fix_deterministic_order_after_node_id_was_decreased(e, this, id);
    }
    let wanted = count.wrapping_sub(1);
    array_reserve_at_least(e, nodes_array, wanted, 0x10);
    e.mem.set_i32(nodes_array + 4, wanted);
}

// ---------------------------------------------------------------------------
// Queries with one float box.
// ---------------------------------------------------------------------------

/// `((box + offset) * scale)` clamped to `[0, 65532]` plus 65536, as the bit patterns shifted
/// right by 7: the low 16 bits of each lane are the quantised value.
fn quantise_vec(e: &Engine, this: u32, corner: Vec4, offset: u32) -> [u32; 4] {
    let limit = load_vec4(e, QUANTISE_MAX);
    let add = load_vec4(e, QUANTISE_ADD);
    let scale = load_vec4(e, this + SCALE);
    let sum = add4(load_vec4(e, this + offset), corner);
    let value = add4(maxps(minps(mul4(sum, scale), limit), [0.0; 4]), add);
    value.map(|lane| lane.to_bits() >> 7)
}

/// [`quantise_vec`] of the box corner stored at `corner`.
fn quantise_corner(e: &Engine, this: u32, corner: u32, offset: u32) -> [u32; 4] {
    quantise_vec(e, this, load_vec4(e, corner), offset)
}

/// The first half of both single-box queries, after `bits` was allocated: clears the bit
/// array, quantises the box (`aabb`: two `hkVector4`s, min then max), sets in `bits` the nodes
/// whose x interval overlaps the box's (a marker's list and an endpoint walk, as
/// `setBitsBasedOnXInterval` does for a node) and finds the box's y and z endpoint indices.
/// Returns the box as the two `u32`s a node's y and z indices make: `(low, high)`, each
/// `y | z << 16`.
fn mark_nodes_in_box(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, aabb: u32, bits: u32) -> (u32, u32) {
    let me = this.addr();
    let count = e.get(this, Hkp3AxisSweep::node_count);
    let mut blocks = count >> 7;
    let mut at = bits;
    while blocks >= 0 {
        for i in 0..4 {
            e.mem.set_u32(at + 4 * i, 0);
        }
        at = at.wrapping_add(16);
        blocks -= 1;
    }
    let low = quantise_corner(e, me, aabb, OFFSET_LOW);
    let high = quantise_corner(e, me, aabb + 0x10, OFFSET_HIGH);
    let min_values = [0, 1, 2].map(|lane: usize| low[lane] & 0xfffe);
    let max_values = [0, 1, 2].map(|lane: usize| (high[lane] & 0xffff) | 1);
    let (min_x, min_y, min_z) = (min_values[0], min_values[1], min_values[2]);
    let (max_x, max_y, max_z) = (max_values[0], max_values[1], max_values[2]);
    let mut cursor = e.get(this, Hkp3AxisSweep::x_endpoints).wrapping_add(4);
    if e.get(this, Hkp3AxisSweep::marker_count) != 0 {
        let shift = 0x10u32.wrapping_sub(e.get(this, Hkp3AxisSweep::marker_bits) as u32) & 31;
        let index = (min_x as i32) >> shift;
        if index > 0 {
            let marker = e
                .get(this, Hkp3AxisSweep::markers)
                .wrapping_add((index as u32) << 4)
                .wrapping_sub(0x10);
            let marker_node = e.mem.u16(marker) as u32;
            toggle_bit(e, bits, marker_node);
            let mut left = e.mem.i32(marker + 8).wrapping_sub(1);
            let mut overlap = e.mem.u32(marker + 4);
            while left >= 0 {
                let other = e.mem.u16(overlap) as u32;
                overlap = overlap.wrapping_add(2);
                toggle_bit(e, bits, other);
                left -= 1;
            }
            let x = e.get(this, Hkp3AxisSweep::x_endpoints);
            let node = e
                .get(this, Hkp3AxisSweep::nodes)
                .wrapping_add((e.mem.u16(marker) as u32) << 4);
            let mut from = x + (e.mem.u16(node + 8) as u32) * 4 + 4;
            let to = x + (e.mem.u16(node + 0xa) as u32) * 4;
            while from < to {
                if e.mem.u8(from) & 1 == 0 {
                    clear_bit(e, bits, e.mem.u16(from + 2) as u32);
                }
                from += 4;
            }
            let x = e.get(this, Hkp3AxisSweep::x_endpoints);
            cursor = x + (e.mem.u16(node + 8) as u32) * 4 + 4;
        }
    }
    while (e.mem.u16(cursor) as u32) < min_x {
        toggle_bit(e, bits, e.mem.u16(cursor + 2) as u32);
        cursor += 4;
    }
    while (e.mem.u16(cursor) as u32) < max_x {
        if e.mem.u16(cursor) & 1 == 0 {
            toggle_bit(e, bits, e.mem.u16(cursor + 2) as u32);
        }
        cursor += 4;
    }
    let mut indices = [0u16; 4];
    for (slot, (array, min_value, max_value)) in [
        (AXIS_ARRAYS[1], min_y, max_y),
        (AXIS_ARRAYS[2], min_z, max_z),
    ]
    .into_iter()
    .enumerate()
    {
        let axis = me + array;
        let data = e.mem.u32(axis);
        let end = data
            .wrapping_add((e.mem.u32(axis + 4)).wrapping_mul(4))
            .wrapping_sub(8);
        let first = hkp_bp_axis_find(e, axis, data + 4, end, min_value as u16);
        indices[slot] = (first.wrapping_sub(data) as i32 >> 2) as u16;
        let last = hkp_bp_axis_find(e, axis, data + 4, end, max_value as u16);
        indices[2 + slot] = (last.wrapping_sub(4).wrapping_sub(data) as i32 >> 2) as u16;
    }
    let low = indices[0] as u32 | (indices[1] as u32) << 16;
    let high = indices[2] as u32 | (indices[3] as u32) << 16;
    (low, high)
}

/// Calls `visit(e, node, handle)` in node order for every marked node (see
/// [`mark_nodes_in_box`]) that is not a marker and whose y and z extents overlap the box
/// `(low, high)`: the sign bits of `(high - node.low) | (node.high - low)`, mask
/// `0x80008000`, are clear.
fn for_each_node_in_box(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    bits: u32,
    low: u32,
    high: u32,
    mut visit: impl FnMut(&mut Engine, u32, u32),
) {
    for_each_marked_node(e, this, bits, |e, node| {
        let differences =
            high.wrapping_sub(e.mem.u32(node)) | e.mem.u32(node + 4).wrapping_sub(low);
        if differences & 0x8000_8000 != 0 {
            return;
        }
        let handle = e.mem.u32(node + 0xc);
        if handle & 1 == 0 {
            visit(e, node, handle);
        }
    });
}

// Translated from 00d14ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::querySingleAabb` (Xbox PDB): appends to `pairs` the pair `(0, handle)` for
/// every object whose box overlaps the float box at `aabb` (two `hkVector4`s, min then
/// max). The x overlap comes from a bit array of nodes, the y and z overlap from searching
/// the endpoint arrays; markers are skipped.
pub fn query_single_aabb(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, aabb: u32, pairs: u32) {
    let count = e.get(this, Hkp3AxisSweep::node_count);
    let block = alloc_node_bits(e, count, false);
    let (low, high) = mark_nodes_in_box(e, this, aabb, block.data);
    for_each_node_in_box(e, this, block.data, low, high, |e, _node, handle| {
        push_pair(e, pairs, 0, handle);
    });
    stack_free(e, &block, 2);
}

// Translated from 00d154a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::querySingleAabb` (Xbox PDB name of the overload that reports to a
/// collector): the same search as `00d14ef0`, but each overlapping object's handle is given
/// to the collector's vtable slot +4 as `(handle, 0)` (the float it returns is discarded)
/// instead of being appended to an array.
pub fn query_single_aabb_with_collector(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    aabb: u32,
    collector: u32,
) {
    let count = e.get(this, Hkp3AxisSweep::node_count);
    let block = alloc_node_bits(e, count, false);
    let (low, high) = mark_nodes_in_box(e, this, aabb, block.data);
    for_each_node_in_box(e, this, block.data, low, high, |e, _node, handle| {
        e.vcall(collector, 4, &args![handle, 0u32]);
    });
    stack_free(e, &block, 2);
}

// Translated from 00d159a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::calcAabbCache` (Xbox PDB; the overload taking a float box): collects the
/// pointers to the nodes that overlap the float box at `aabb` (the search of
/// `querySingleAabb`, markers skipped) in a local `hkArray` that takes its memory from the
/// router's stack allocator (reserved to exactly the node count), builds the cache for them in
/// `cache` with `calcAabbCacheInternal` and gives the memory back.
pub fn calc_aabb_cache(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, aabb: u32, cache: u32) {
    let count = e.get(this, Hkp3AxisSweep::node_count);
    e.with_stack(0xc, |e, local| {
        let array = local.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
        if capacity < count {
            let allocator = e.mem.u32(router + 0xc);
            e.call(ARRAY_RESERVE, &args![allocator, array, count, 4u32]);
        }
        let node_count = e.get(this, Hkp3AxisSweep::node_count);
        let block = alloc_node_bits(e, node_count, false);
        let (low, high) = mark_nodes_in_box(e, this, aabb, block.data);
        for_each_node_in_box(e, this, block.data, low, high, |e, node, _handle| {
            let size = e.mem.u32(array + 4);
            let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
            e.mem.set_u32(slot, node);
            e.mem.set_u32(array + 4, size.wrapping_add(1));
        });
        stack_free(e, &block, 2);
        calc_aabb_cache_internal(e, this, array, cache);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0xc);
        let capacity = e.mem.u32(array + 8);
        let data = e.mem.u32(array);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK) << 2],
            );
        }
    });
}

// Translated from 00d15f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::calcAabbCache` (Xbox PDB; the overload taking an `hkArray` of collidable
/// pointers, whose dword at +0x14 is the node index): takes `4 * size` bytes (rounded up to a
/// multiple of 128) from the router's bump pointer (its first word), stores the node pointers
/// of the given objects there as a local `hkArray` that does not own its memory, builds the
/// cache with `calcAabbCacheInternal` and puts the bump pointer back. The array has its
/// not-owned bit set, so nothing is freed.
pub fn calc_aabb_cache_ov2(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, objects: u32, cache: u32) {
    let size = e.mem.u32(objects + 4);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    e.mem.set_u32(
        router,
        (size.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(top),
    );
    let nodes = e.get(this, Hkp3AxisSweep::nodes);
    let data = e.mem.u32(objects);
    let mut stored = 0u32;
    while (stored as i32) < size as i32 {
        let object = e.mem.u32(data.wrapping_add(stored.wrapping_mul(4)));
        let node = (e.mem.u32(object + 0x14) << 4).wrapping_add(nodes);
        stored += 1;
        e.mem.set_u32(
            top.wrapping_add(stored.wrapping_mul(4)).wrapping_sub(4),
            node,
        );
    }
    e.with_stack(0xc, |e, local| {
        let array = local.addr();
        e.mem.set_u32(array, top);
        e.mem.set_u32(array + 4, stored);
        e.mem.set_u32(array + 8, size | NOT_OWNED);
        calc_aabb_cache_internal(e, this, array, cache);
    });
    // The router is asked for twice more: its bump pointer is restored, then the heap
    // allocator is fetched for a free that the not-owned bit makes unreachable.
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
    e.call(MEMORY_ROUTER, &args![]);
}

// ---------------------------------------------------------------------------
// Endpoint array helpers used by the batch functions and `defragment`.
// ---------------------------------------------------------------------------

/// The merge of `00d19d90`, `00d19e40` and `00d19ef0`, which differ only in the table of node
/// field offsets (x: `011b37a4`, y: `011b37ac`, z: `011b37b4`; the entry for an endpoint is
/// picked by its maximum bit). `array` is an endpoint `hkArray`; its first
/// `end - count` endpoints are sorted, and `count` sorted new endpoints at `new_endpoints`
/// (already counted in the array's size, but the array's slots from `end - count` on are free)
/// are merged in from the back into the slots up to index `end - 1`. An old endpoint moves
/// when its value is above the next new endpoint's, else the new one is stored. Every stored
/// endpoint sets its node's field (`nodes + id * 16 + table[bit]`) to its new index. The loop
/// ends when the new endpoints are used up; the old ones that are left stay where they are.
fn merge_endpoints_from_back(
    e: &mut Engine,
    table: u32,
    array: u32,
    nodes: u32,
    end: i32,
    new_endpoints: u32,
    count: i32,
) {
    let data = e.mem.u32(array);
    let mut old_at = data
        .wrapping_add((end.wrapping_sub(count) as u32).wrapping_mul(4))
        .wrapping_sub(4);
    let mut new_at = new_endpoints
        .wrapping_add((count as u32).wrapping_mul(4))
        .wrapping_sub(4);
    let mut old_value = e.mem.u16(old_at) as u32;
    let mut new_value = e.mem.u16(new_at) as u32;
    let mut index = end.wrapping_sub(1);
    let mut to = data.wrapping_add((index as u32).wrapping_mul(4));
    loop {
        let from = if old_value > new_value {
            old_at
        } else {
            new_at
        };
        let endpoint = e.mem.u32(from);
        e.mem.set_u32(to, endpoint);
        let field = e.mem.u32(table + (e.mem.u16(from) as u32 & 1) * 4);
        let id = e.mem.u16(from + 2) as u32;
        e.mem.set_u16(
            nodes.wrapping_add(id << 4).wrapping_add(field),
            index as u16,
        );
        to = to.wrapping_sub(4);
        index = index.wrapping_sub(1);
        if old_value > new_value {
            old_at = old_at.wrapping_sub(4);
            old_value = e.mem.u16(old_at) as u32;
            continue;
        }
        new_at = new_at.wrapping_sub(4);
        new_value = e.mem.u16(new_at) as u32;
        if new_at < new_endpoints {
            break;
        }
    }
}

// Translated from 00d19d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` is an x endpoint array): [`merge_endpoints_from_back`] with the node
/// field table `011b37a4` (x: `min_x`, `max_x`).
pub fn fn_00d19d90(
    e: &mut Engine,
    array: u32,
    nodes: u32,
    end: i32,
    new_endpoints: u32,
    count: i32,
) {
    merge_endpoints_from_back(e, NODE_FIELD_TABLE, array, nodes, end, new_endpoints, count);
}

// Translated from 00d19e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` is a y endpoint array): [`merge_endpoints_from_back`] with the node
/// field table `011b37ac` (y: `min_y`, `max_y`).
pub fn fn_00d19e40(
    e: &mut Engine,
    array: u32,
    nodes: u32,
    end: i32,
    new_endpoints: u32,
    count: i32,
) {
    merge_endpoints_from_back(
        e,
        NODE_FIELD_TABLE + 8,
        array,
        nodes,
        end,
        new_endpoints,
        count,
    );
}

// Translated from 00d19ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` is a z endpoint array): [`merge_endpoints_from_back`] with the node
/// field table `011b37b4` (z: `min_z`, `max_z`).
pub fn fn_00d19ef0(
    e: &mut Engine,
    array: u32,
    nodes: u32,
    end: i32,
    new_endpoints: u32,
    count: i32,
) {
    merge_endpoints_from_back(
        e,
        NODE_FIELD_TABLE + 16,
        array,
        nodes,
        end,
        new_endpoints,
        count,
    );
}

/// The compaction of `00d1a040`, `00d1a0e0` and `00d1a180` (tables as in
/// [`merge_endpoints_from_back`]): `mapping` points at a table of node indices (`i32` per old
/// index, negative for a node that goes away). The endpoints of `array` are packed down
/// keeping those whose node survives, with the node index replaced by the mapped one; each
/// kept endpoint sets its node's field to its new index. Afterwards the array is grown to the
/// new size when its capacity is smaller (it never shrinks) and its size is set.
fn compact_endpoints(e: &mut Engine, table: u32, array: u32, nodes: u32, mapping: u32) {
    let data = e.mem.u32(array);
    let end = data.wrapping_add(e.mem.u32(array + 4).wrapping_mul(4));
    let mut cursor = data;
    let mut to = data;
    let mut kept = 0u32;
    while cursor < end {
        let id = e.mem.u16(cursor + 2) as u32;
        let table_base = e.mem.u32(mapping);
        let mapped = e.mem.i32(table_base.wrapping_add(id.wrapping_mul(4)));
        if mapped >= 0 {
            let endpoint = e.mem.u32(cursor);
            e.mem.set_u32(to, endpoint);
            e.mem.set_u16(to + 2, mapped as u16);
            let field = e.mem.u32(table + (e.mem.u16(cursor) as u32 & 1) * 4);
            e.mem.set_u16(
                (mapped as u32)
                    .wrapping_shl(4)
                    .wrapping_add(field)
                    .wrapping_add(nodes),
                kept as u16,
            );
            to += 4;
            kept += 1;
        }
        cursor += 4;
    }
    array_reserve_at_least(e, array, kept as i32, 4);
    e.mem.set_u32(array + 4, kept);
}

// Translated from 00d1a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` is an x endpoint array): [`compact_endpoints`] with the table
/// `011b37a4`.
pub fn fn_00d1a040(e: &mut Engine, array: u32, nodes: u32, mapping: u32) {
    compact_endpoints(e, NODE_FIELD_TABLE, array, nodes, mapping);
}

// Translated from 00d1a0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` is a y endpoint array): [`compact_endpoints`] with the table
/// `011b37ac`.
pub fn fn_00d1a0e0(e: &mut Engine, array: u32, nodes: u32, mapping: u32) {
    compact_endpoints(e, NODE_FIELD_TABLE + 8, array, nodes, mapping);
}

// Translated from 00d1a180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` is a z endpoint array): [`compact_endpoints`] with the table
/// `011b37b4`.
pub fn fn_00d1a180(e: &mut Engine, array: u32, nodes: u32, mapping: u32) {
    compact_endpoints(e, NODE_FIELD_TABLE + 16, array, nodes, mapping);
}

// Translated from 00d1a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: sorts the endpoints (4 bytes: `u16` value, `u16` node index) at
/// `base[lo..=hi]` ascending by value only (equal values may end up in any order); the
/// `comparator` argument is only handed on. The sibling `quickSortRecursive` (`00d19fa0`)
/// also orders equal values by node index.
pub fn fn_00d1a220(e: &mut Engine, base: u32, lo: i32, hi: i32, comparator: u32) {
    let at = |index: i32| base.wrapping_add((index as u32).wrapping_mul(4));
    let mut lo = lo;
    loop {
        let pivot = e.mem.u16(at(lo.wrapping_add(hi) >> 1));
        let mut i = lo;
        let mut j = hi;
        loop {
            while e.mem.u16(at(i)) < pivot {
                i = i.wrapping_add(1);
            }
            while pivot < e.mem.u16(at(j)) {
                j = j.wrapping_sub(1);
            }
            if j < i {
                break;
            }
            if j != i {
                let left = e.mem.u32(at(i));
                let right = e.mem.u32(at(j));
                e.mem.set_u32(at(j), left);
                e.mem.set_u32(at(i), right);
            }
            j = j.wrapping_sub(1);
            i = i.wrapping_add(1);
            if i > j {
                break;
            }
        }
        if lo < j {
            fn_00d1a220(e, base, lo, j, comparator);
        }
        if i >= hi {
            return;
        }
        lo = i;
    }
}

// ---------------------------------------------------------------------------
// defragment, the constructor wrapper, the destructors and small helpers.
// ---------------------------------------------------------------------------

// Translated from 00d15fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::defragment` (Xbox PDB): renumbers the nodes in the order of their minimum
/// x endpoints. With three temporary blocks from the router's stack allocator (a copy of the
/// nodes, `(min_x index, node)` pairs, and the old-to-new index map) it sorts the pairs after
/// the sentinel with `00d1a220`, rewrites the node array in the new order, points every
/// handle (or marker) at its node's new index, rewrites the node indices in the markers'
/// overlap arrays and in all endpoints, and finally runs
/// `_fixDeterministicOrderAfterNodeIdWasDecreased` for each node but the first.
pub fn defragment(e: &mut Engine, this: Ptr<Hkp3AxisSweep>) {
    let me = this.addr();
    let count = e.get(this, Hkp3AxisSweep::node_count);
    let copy = stack_alloc(e, count.wrapping_shl(4), 4);
    let pairs = stack_alloc(e, count.wrapping_mul(4), 2);
    let map = stack_alloc(e, count.wrapping_mul(4), 2);
    let mut index = 0i32;
    let mut offset = 0u32;
    while index < e.get(this, Hkp3AxisSweep::node_count) {
        let node = e.get(this, Hkp3AxisSweep::nodes).wrapping_add(offset);
        copy_vec4(e, node, copy.data.wrapping_add(offset));
        let min_x = e
            .mem
            .u16(e.get(this, Hkp3AxisSweep::nodes).wrapping_add(offset) + 8);
        let pair = pairs.data.wrapping_add((index as u32).wrapping_mul(4));
        e.mem.set_u16(pair, min_x);
        e.mem.set_u16(pair + 2, index as u16);
        index += 1;
        offset = offset.wrapping_add(0x10);
    }
    if count.wrapping_sub(1) > 1 {
        // The comparator argument is stack the exe never initialised beyond its low byte.
        fn_00d1a220(e, pairs.data + 4, 0, count.wrapping_sub(2), 0);
    }
    let mut new_index = 0i32;
    let mut offset = 0u32;
    while new_index < e.get(this, Hkp3AxisSweep::node_count) {
        let old = e
            .mem
            .u16(pairs.data.wrapping_add((new_index as u32).wrapping_mul(4)) + 2)
            as u32;
        e.mem.set_i32(map.data.wrapping_add(old << 2), new_index);
        let to = e.get(this, Hkp3AxisSweep::nodes).wrapping_add(offset);
        copy_vec4(e, copy.data.wrapping_add(old << 4), to);
        new_index += 1;
        offset = offset.wrapping_add(0x10);
    }
    let mut node_index = 1i32;
    let mut offset = 0x10u32;
    while node_index < e.get(this, Hkp3AxisSweep::node_count) {
        let handle = e
            .mem
            .u32(e.get(this, Hkp3AxisSweep::nodes).wrapping_add(offset) + 0xc);
        if handle & 1 == 0 {
            e.mem.set_i32(handle, node_index);
        } else {
            let markers = e.get(this, Hkp3AxisSweep::markers);
            e.mem.set_u16(
                (handle & 0xffff_fffe).wrapping_add(markers),
                node_index as u16,
            );
        }
        node_index += 1;
        offset = offset.wrapping_add(0x10);
    }
    let remap = |e: &Engine, old: u16| e.mem.u16(map.data.wrapping_add((old as u32) << 2));
    let mut marker_index = 0i32;
    let mut offset = 0u32;
    while marker_index < e.get(this, Hkp3AxisSweep::marker_count) {
        let marker = e.get(this, Hkp3AxisSweep::markers).wrapping_add(offset);
        let mut element = e.mem.i32(marker + 8).wrapping_sub(1);
        while element >= 0 {
            let at = e
                .mem
                .u32(marker + 4)
                .wrapping_add((element as u32).wrapping_mul(2));
            let mapped = remap(e, e.mem.u16(at));
            e.mem.set_u16(at, mapped);
            element -= 1;
        }
        marker_index += 1;
        offset = offset.wrapping_add(0x10);
    }
    for array in AXIS_ARRAYS {
        let mut endpoint = 0i32;
        while endpoint < e.mem.i32(me + array + 4) {
            let at = e.mem.u32(me + array) + (endpoint as u32) * 4 + 2;
            let mapped = remap(e, e.mem.u16(at));
            e.mem.set_u16(at, mapped);
            endpoint += 1;
        }
    }
    let mut node_index = 1i32;
    while node_index < e.get(this, Hkp3AxisSweep::node_count) {
        fix_deterministic_order_after_node_id_was_decreased(e, this, node_index as u32);
        node_index += 1;
    }
    stack_free(e, &map, 2);
    stack_free(e, &pairs, 2);
    stack_free(e, &copy, 4);
}

// Translated from 00d182a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (its address is stored in a function table at `011b37a0` that `hkpWorld`
/// calls through, with the world's min and max vectors and the marker count): allocates the
/// 0xe0 bytes of an `hkp3AxisSweep` from the heap allocator (vtable +4), stores 0xe0 in the
/// 16-bit field at +4 (the object's memory size) and runs the constructor.
pub fn fn_00d182a0(
    e: &mut Engine,
    world_min: u32,
    world_max: u32,
    marker_request: i32,
) -> Ptr<Hkp3AxisSweep> {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let heap = e.mem.u32(router + 0x10);
    let object = e.vcall(heap, 4, &args![0xe0u32]).u32();
    e.mem.set_u16(object + 4, 0xe0);
    hkp_3_axis_sweep(e, Ptr::new(object), world_min, world_max, marker_request)
}

/// Empties an `hkArray` and gives its memory back to the heap allocator `heap` through
/// vtable slot 0x10 as `(data, bytes)` when it owns it: `bytes = (capacity & mask) << shift`.
fn release_array(e: &mut Engine, heap: u32, array: u32, shift: u32) {
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            heap,
            0x10,
            &args![data, (capacity & CAPACITY_MASK) << shift],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

// Translated from 00d182e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::~hkp3AxisSweep` (Xbox PDB): sets the vtable, releases each marker's overlap
/// array, frees the marker block (`hkMemoryRouter::easyFree`, `00c856c0`), releases the z, y
/// and x endpoint arrays and the node array, and ends in the base destructor `00d970c0`.
pub fn hkp_3_axis_sweep_destructor(e: &mut Engine, this: Ptr<Hkp3AxisSweep>) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE);
    let mut index = 0i32;
    let mut offset = 0u32;
    while index < e.mem.i32(me + 0xd0) {
        let overlap = e.mem.u32(me + 0xd8).wrapping_add(offset) + 4;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let heap = e.mem.u32(router + 0x10);
        release_array(e, heap, overlap, 1);
        index += 1;
        offset = offset.wrapping_add(0x10);
    }
    let markers = e.mem.u32(me + 0xd8);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let heap = e.mem.u32(router + 0x10);
    e.call(EASY_FREE, &args![heap, markers]);
    for array in AXIS_ARRAYS.iter().rev() {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let heap = e.mem.u32(router + 0x10);
        release_array(e, heap, me + array, 2);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let heap = e.mem.u32(router + 0x10);
    release_array(e, heap, me + 0xa0, 4);
    e.call(BASE_DESTRUCTOR, &args![me]);
}

// Translated from 00d1a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor and, when
/// bit 0 of `flags` is set, frees the object through the heap allocator (vtable +8) with the
/// size in the 16-bit field at +4. Returns `this`.
pub fn hkp_3_axis_sweep_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    flags: u32,
) -> Ptr<Hkp3AxisSweep> {
    hkp_3_axis_sweep_destructor(e, this);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = e.mem.u16(this.addr() + 4) as u32;
        let heap = e.mem.u32(router + 0x10);
        e.vcall(heap, 8, &args![this.addr(), size]);
    }
    this
}

// Translated from 00d1a2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::getOffsetLowHigh32bit` (Xbox PDB): copies the 32-bit low offset (+0x70)
/// to `low` and the high offset (+0x80) to `high`.
pub fn get_offset_low_high_32_bit(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, low: u32, high: u32) {
    copy_vec4(e, this.addr() + OFFSET_LOW_32, low);
    copy_vec4(e, this.addr() + OFFSET_HIGH_32, high);
}

// Translated from 00d19d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (called from `splitTreeBuilding`): the larger of two 16-bit values.
pub fn fn_00d19d70(_e: &mut Engine, first: u16, second: u16) -> u16 {
    if first > second {
        first
    } else {
        second
    }
}

// Translated from 00d19d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (called from `castAabb`): the index (0, 1 or 2) of the smallest of the three
/// floats at `values`, whose value is written to `out`. The comparisons are the `FCOMPP`
/// flag tests, so a NaN counts as not greater. A stored float goes through `FLD`/`FSTP`.
pub fn fn_00d19d10(e: &mut Engine, values: u32, out: u32) -> u32 {
    let first = e.mem.f32(values);
    let second = e.mem.f32(values + 4);
    let third = e.mem.f32(values + 8);
    let (index, source) = if second > first {
        if third > first {
            (0, values)
        } else {
            (2, values + 8)
        }
    } else if third > second {
        (1, values + 4)
    } else {
        (2, values + 8)
    };
    let bits = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(out, bits);
    index
}

// ---------------------------------------------------------------------------
// The batch functions.
// ---------------------------------------------------------------------------

/// `hkRadixSort::sort16` (`00d9fcf0`, no map name): `(data, count rounded up to 4, scratch)`
/// cdecl; sorts 4-byte entries by their low 16 bits.
const RADIX_SORT_16: u32 = 0x00d9_fcf0;

/// Copies one 16-byte node.
fn copy_node(e: &mut Engine, from: u32, to: u32) {
    for word in 0..4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Gives back what `stack_alloc` took and what `ARRAY_RESERVE` put in a local array that
/// takes its memory from the stack allocator: `(capacity & mask) << shift` bytes when the
/// array owns its memory.
fn release_stack_array(e: &mut Engine, array: u32, shift: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8);
    if (capacity as i32) >= 0 {
        let allocator = e.mem.u32(router + 0xc);
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK) << shift],
        );
    }
}

// Translated from 00d193e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::removeObjectBatch` (Xbox PDB): removes the objects of the `hkArray` at
/// `objects` (each element points at the dword that holds the object's node index) and
/// appends to `pairs` the pairs that stop overlapping.
///
/// The removed nodes are copied out, in the order of their `min_x` endpoint (radix sort), to
/// one list; their handle becomes the address of a local dword (the tombstone) and their
/// object's node index is cleared. The node array is then packed: from the back, a
/// tombstoned node is dropped and a tombstoned node at the front takes the last live node's
/// place (the object's node index or the marker's node field is updated), building a map
/// from old to new indices (-1 for removed). The nodes that overlap the box of all removed
/// nodes are copied out, sorted by `min_x`, to a second list; `collide1Axis` of the
/// removed list with itself and with that list reports the pairs. The three endpoint arrays
/// are compacted through the map, the markers' overlap arrays are remapped, and the
/// moved nodes get `_fixDeterministicOrderAfterNodeIdWasDecreased`.
pub fn remove_object_batch(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, objects: u32, pairs: u32) {
    let me = this.addr();
    let total = e.get(this, Hkp3AxisSweep::node_count);
    let removed_count = e.mem.i32(objects + 4);
    e.with_stack(0x40, |e, frame| {
        let tombstone = frame.addr();
        let bounds = frame.addr() + 0x10;
        let removed = frame.addr() + 0x20;
        let mapping = frame.addr() + 0x2c;

        // The removed nodes' copies, then the box and the sort keys.
        let copies = stack_alloc(e, removed_count.wrapping_add(1).wrapping_mul(16), 4);
        let first_object = e.mem.u32(e.mem.u32(objects));
        let first_node = e
            .mem
            .u32(first_object)
            .wrapping_shl(4)
            .wrapping_add(e.get(this, Hkp3AxisSweep::nodes));
        copy_node(e, first_node, bounds);
        let keys = stack_alloc(e, removed_count.wrapping_add(4).wrapping_mul(4), 2);
        let mut index = 0u32;
        while (index as i32) < removed_count {
            let element = e
                .mem
                .u32(e.mem.u32(objects).wrapping_add(index.wrapping_mul(4)));
            let id = e.mem.u32(element);
            let node = id
                .wrapping_shl(4)
                .wrapping_add(e.get(this, Hkp3AxisSweep::nodes));
            let key_at = keys.data.wrapping_add(index.wrapping_mul(4));
            e.mem.set_u16(key_at + 2, id as u16);
            let min_x = e.mem.u16(node + 8);
            e.mem.set_u16(key_at, min_x);
            if min_x < e.mem.u16(bounds + 8) {
                e.mem.set_u16(bounds + 8, min_x);
            }
            let min_y = e.mem.u16(node);
            if min_y < e.mem.u16(bounds) {
                e.mem.set_u16(bounds, min_y);
            }
            let min_z = e.mem.u16(node + 2);
            if min_z < e.mem.u16(bounds + 2) {
                e.mem.set_u16(bounds + 2, min_z);
            }
            let max_x = e.mem.u16(node + 0xa);
            if max_x > e.mem.u16(bounds + 0xa) {
                e.mem.set_u16(bounds + 0xa, max_x);
            }
            let max_y = e.mem.u16(node + 4);
            if max_y > e.mem.u16(bounds + 4) {
                e.mem.set_u16(bounds + 4, max_y);
            }
            let max_z = e.mem.u16(node + 6);
            if max_z > e.mem.u16(bounds + 6) {
                e.mem.set_u16(bounds + 6, max_z);
            }
            index += 1;
        }
        let scratch = stack_alloc(e, removed_count.wrapping_add(4).wrapping_mul(4), 2);
        let end_key = keys
            .data
            .wrapping_add((removed_count as u32).wrapping_mul(4));
        e.mem.set_u16(end_key + 4, 0xffff);
        e.mem.set_u16(end_key, 0xffff);
        e.mem.set_u16(end_key + 8, 0xffff);
        e.call(
            RADIX_SORT_16,
            &args![
                keys.data,
                removed_count.wrapping_add(3) as u32 & 0xffff_fffc,
                scratch.data
            ],
        );
        if removed_count > 0 {
            for position in 0..removed_count as u32 {
                let id = e.mem.u16(keys.data + 4 * position + 2) as u32;
                let node = id
                    .wrapping_shl(4)
                    .wrapping_add(e.get(this, Hkp3AxisSweep::nodes));
                copy_node(e, node, copies.data + 16 * position);
                let handle = e.mem.u32(node + 0xc);
                e.mem.set_u32(handle, 0);
                e.mem.set_u32(node + 0xc, tombstone);
            }
        }
        let copies_end = copies
            .data
            .wrapping_add((removed_count as u32).wrapping_shl(4));
        e.mem.set_u16(copies_end + 8, 0xffff);
        stack_free(e, &scratch, 2);
        stack_free(e, &keys, 2);

        // The ids of the nodes that moved, as a local array on the stack allocator.
        let marker_count = e.get(this, Hkp3AxisSweep::marker_count);
        let wanted = marker_count.wrapping_add(removed_count);
        e.mem.set_u32(removed, 0);
        e.mem.set_u32(removed + 4, 0);
        e.mem.set_u32(removed + 8, NOT_OWNED);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        if ((e.mem.u32(removed + 8) & CAPACITY_MASK) as i32) < wanted {
            let allocator = e.mem.u32(router + 0xc);
            e.call(ARRAY_RESERVE, &args![allocator, removed, wanted, 4u32]);
        }

        // The map from old to new node index and the nodes that may overlap the box.
        let map = stack_alloc(e, total.wrapping_mul(4), 2);
        e.mem.set_u32(mapping, map.data);
        e.mem.set_u32(
            mapping + 4,
            if map.capacity == NOT_OWNED {
                0
            } else {
                map.capacity
            },
        );
        e.mem.set_u32(mapping + 8, map.capacity);
        let candidates = stack_alloc(
            e,
            total
                .wrapping_sub(removed_count)
                .wrapping_add(4)
                .wrapping_mul(4),
            2,
        );
        let set_map = |e: &mut Engine, at: i32, value: i32| {
            e.mem
                .set_i32(map.data.wrapping_add((at as u32).wrapping_mul(4)), value);
        };
        let mut last = total.wrapping_sub(1);
        e.mem.set_u32(map.data, 0);
        let mut current = 1i32;
        let mut survivors = 0u32;
        if last >= current {
            loop {
                let nodes = e.get(this, Hkp3AxisSweep::nodes);
                let last_node = nodes.wrapping_add((last as u32).wrapping_shl(4));
                let current_node = nodes.wrapping_add((current as u32).wrapping_shl(4));
                if e.mem.u32(last_node + 0xc) == tombstone {
                    set_map(e, last, -1);
                    last = last.wrapping_sub(1);
                    current = current.wrapping_sub(1);
                } else {
                    let mut test_overlap = true;
                    if e.mem.u32(current_node + 0xc) == tombstone {
                        copy_node(e, last_node, current_node);
                        set_map(e, last, current);
                        last = last.wrapping_sub(1);
                        set_map(e, current, -1);
                        let size = e.mem.u32(removed + 4);
                        let slot = e.mem.u32(removed).wrapping_add(size.wrapping_mul(4));
                        e.mem.set_i32(slot, current);
                        e.mem.set_u32(removed + 4, size.wrapping_add(1));
                        let handle = e.mem.u32(current_node + 0xc);
                        if handle & 1 == 0 {
                            e.mem.set_i32(handle, current);
                        } else {
                            let markers = e.get(this, Hkp3AxisSweep::markers);
                            e.mem.set_u16(
                                markers.wrapping_add(handle & 0xffff_fffe),
                                current as u16,
                            );
                            test_overlap = false;
                        }
                    } else {
                        set_map(e, current, current);
                    }
                    if test_overlap {
                        let high_minus_low =
                            e.mem.u32(current_node + 4).wrapping_sub(e.mem.u32(bounds));
                        let low_minus_high =
                            e.mem.u32(bounds + 4).wrapping_sub(e.mem.u32(current_node));
                        if (high_minus_low | low_minus_high) & 0x8000_8000 == 0 {
                            let node_min_x = e.mem.u16(current_node + 8);
                            let signs = e
                                .mem
                                .u16(current_node + 0xa)
                                .wrapping_sub(e.mem.u16(bounds + 8))
                                | e.mem.u16(current_node + 4).wrapping_sub(e.mem.u16(bounds))
                                | e.mem.u16(bounds + 4).wrapping_sub(e.mem.u16(current_node))
                                | e.mem.u16(bounds + 0xa).wrapping_sub(node_min_x);
                            if signs & 0x8000 == 0 {
                                let at = candidates.data.wrapping_add(survivors.wrapping_mul(4));
                                e.mem.set_u16(at, node_min_x);
                                e.mem.set_u16(at + 2, current as u16);
                                survivors += 1;
                            }
                        }
                    }
                }
                current = current.wrapping_add(1);
                if current > last {
                    break;
                }
            }
        }
        let new_count = last.wrapping_add(1);
        array_reserve_at_least(e, me + 0xa0, new_count, 0x10);
        e.mem.set_i32(me + 0xa4, new_count);

        // The surviving candidates, sorted by `min_x`.
        let neighbours = stack_alloc(e, (survivors as i32).wrapping_add(1).wrapping_mul(16), 4);
        let neighbour_scratch =
            stack_alloc(e, (survivors as i32).wrapping_add(4).wrapping_mul(4), 2);
        let end_candidate = candidates.data.wrapping_add(survivors.wrapping_mul(4));
        e.mem.set_u16(end_candidate, 0xffff);
        e.mem.set_u16(end_candidate + 8, 0xffff);
        e.mem.set_u16(end_candidate + 4, 0xffff);
        e.call(
            RADIX_SORT_16,
            &args![
                candidates.data,
                survivors.wrapping_add(3) & 0xffff_fffc,
                neighbour_scratch.data
            ],
        );
        for position in 0..survivors {
            let id = e.mem.u16(candidates.data + 4 * position + 2) as u32;
            let node = id
                .wrapping_shl(4)
                .wrapping_add(e.get(this, Hkp3AxisSweep::nodes));
            copy_node(e, node, neighbours.data + 16 * position);
        }
        let neighbours_end = neighbours.data.wrapping_add(survivors.wrapping_shl(4));
        e.mem.set_u16(neighbours_end + 8, 0xffff);
        stack_free(e, &neighbour_scratch, 2);

        collide_1_axis_one_list(e, me, copies.data, removed_count, pairs);
        collide_1_axis(
            e,
            this,
            copies.data,
            removed_count,
            neighbours.data,
            survivors as i32,
            1,
            pairs,
        );
        let nodes = e.get(this, Hkp3AxisSweep::nodes);
        fn_00d1a040(e, me + 0xac, nodes, mapping);
        let nodes = e.get(this, Hkp3AxisSweep::nodes);
        fn_00d1a0e0(e, me + 0xb8, nodes, mapping);
        let nodes = e.get(this, Hkp3AxisSweep::nodes);
        fn_00d1a180(e, me + 0xc4, nodes, mapping);

        // The markers' nodes lose the removed endpoints and their overlap lists are remapped.
        if e.get(this, Hkp3AxisSweep::marker_count) != 0 {
            let shrink = removed_count.wrapping_mul(2) as u16;
            let mut marker_index = 0i32;
            while marker_index < e.get(this, Hkp3AxisSweep::marker_count) {
                let marker = e
                    .get(this, Hkp3AxisSweep::markers)
                    .wrapping_add((marker_index as u32).wrapping_shl(4));
                let node = (e.mem.u16(marker) as u32)
                    .wrapping_shl(4)
                    .wrapping_add(e.get(this, Hkp3AxisSweep::nodes));
                let max_y = e.mem.u16(node + 4).wrapping_sub(shrink);
                e.mem.set_u16(node + 4, max_y);
                let max_z = e.mem.u16(node + 6).wrapping_sub(shrink);
                e.mem.set_u16(node + 6, max_z);
                let mut kept = 0u32;
                let mut position = 0i32;
                while position < e.mem.i32(marker + 8) {
                    let overlap = e.mem.u32(marker + 4);
                    let old = e
                        .mem
                        .u16(overlap.wrapping_add((position as u32).wrapping_mul(2)))
                        as u32;
                    let mapped = e.mem.i32(map.data.wrapping_add(old.wrapping_mul(4)));
                    if mapped >= 0 {
                        e.mem
                            .set_u16(overlap.wrapping_add(kept.wrapping_mul(2)), mapped as u16);
                        kept += 1;
                    }
                    position += 1;
                }
                array_reserve_at_least(e, marker + 4, kept as i32, 2);
                e.mem.set_u32(marker + 8, kept);
                marker_index += 1;
            }
        }
        let mut moved = 0u32;
        while (moved as i32) < e.mem.i32(removed + 4) {
            let node = e
                .mem
                .u32(e.mem.u32(removed).wrapping_add(moved.wrapping_mul(4)));
            fix_deterministic_order_after_node_id_was_decreased(e, this, node);
            moved += 1;
        }
        stack_free(e, &neighbours, 4);
        stack_free(e, &candidates, 2);
        stack_free(e, &map, 2);
        e.mem.set_u32(removed + 4, 0);
        release_stack_array(e, removed, 2);
        e.mem.set_u32(removed, 0);
        e.mem.set_u32(removed + 8, NOT_OWNED);
        stack_free(e, &copies, 4);
    });
}

/// The CRT conversion of `ST0` to a 64-bit integer (`00ec62f6`, no map name; the value comes
/// as a leading `f64` and the result in `EDX:EAX`).
const FLOAT_TO_INT64: u32 = 0x00ec_62f6;

// Translated from 00d175f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::shiftAllObjects` (Xbox PDB): shifts every endpoint of the three axes by
/// the float vector at `shift`. Per axis the shift is `shift * scale` truncated to a 64-bit
/// integer with the low bit cleared (an even number of endpoint units); `applied` (four
/// floats, `w` copied) gets that integer divided by the scale, as a `float`.
///
/// Each endpoint (except the first and last of the array, which are sentinels; indices run
/// up when the shift is negative and down otherwise) whose value is in `2..=0xfffb` gets
/// `(value + shift) & !1 | (value & 1)`, below 0 clamped to its bit and from `0xfffc` up to
/// `bit | 0xfffc`. An endpoint that ends up at 0 or `0xfffd` belongs to an object whose box
/// hit the edge: that object's six endpoints are then moved to their current values the way
/// `updateAabbs` moves them (minimum down, maximum up then down, minimum up, per axis x, y,
/// z), new overlaps going to `pairs` and ended ones to a local array that is thrown away.
pub fn shift_all_objects(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    shift: u32,
    applied: u32,
    pairs: u32,
) {
    let me = this.addr();
    let mut lanes = load_vec4(e, shift);
    let mut shifts = [0i32; 3];
    for axis in 0..3 {
        let scale = e.mem.f32(me + SCALE + 4 * axis as u32);
        let product = scale as f64 * lanes[axis] as f64;
        let raw = e.call(FLOAT_TO_INT64, &args![product]).u64();
        let even = raw & 0xffff_ffff_ffff_fffe;
        shifts[axis] = even as u32 as i32;
        lanes[axis] = ((even as i64 as f64) / scale as f64) as f32;
    }
    store_vec4(e, applied, lanes);
    e.with_stack(0xc, |e, exits| {
        let exits = exits.addr();
        e.mem.set_u32(exits, 0);
        e.mem.set_u32(exits + 4, 0);
        e.mem.set_u32(exits + 8, NOT_OWNED);
        for axis in 0..3usize {
            let array = me + AXIS_ARRAYS[axis];
            let amount = shifts[axis];
            let size = e.mem.i32(array + 4);
            let (mut index, end, step) = if amount < 0 {
                (1i32, size.wrapping_sub(1), 1i32)
            } else {
                (size.wrapping_sub(2), 0i32, -1i32)
            };
            if index == end {
                continue;
            }
            loop {
                let at = e
                    .mem
                    .u32(array)
                    .wrapping_add((index as u32).wrapping_mul(4));
                let value = e.mem.u16(at) as u32;
                if value.wrapping_sub(2) <= 0xfff9 {
                    let bit = value & 1;
                    let sum = amount.wrapping_add(value as i32);
                    let moved = ((sum as u32) & 0xffff_fffe) | bit;
                    let new_value = if (moved as i32) < 0 {
                        bit
                    } else if (moved as i32) < 0xfffc {
                        moved
                    } else {
                        bit | 0xfffc
                    };
                    e.mem.set_u16(at, new_value as u16);
                    if new_value == 0 || new_value == 0xfffd {
                        let endpoint_node = e
                            .get(this, Hkp3AxisSweep::nodes)
                            .wrapping_add((e.mem.u16(at + 2) as u32) << 4);
                        let mut values = [(0u32, 0u32); 3];
                        for (slot, array_offset) in AXIS_ARRAYS.iter().enumerate() {
                            let data = e.mem.u32(me + array_offset);
                            let [min_field, max_field] = AXIS_NODE_FIELDS[slot];
                            let min_index = e.mem.u16(endpoint_node + min_field) as u32;
                            let max_index = e.mem.u16(endpoint_node + max_field) as u32;
                            values[slot] = (
                                e.mem.u16(data.wrapping_add(min_index * 4)) as u32,
                                e.mem.u16(data.wrapping_add(max_index * 4)) as u32,
                            );
                        }
                        let handle = e.mem.u32(endpoint_node + 0xc);
                        let node_id = e.mem.u32(handle);
                        move_object_endpoints(e, this, node_id, &values, pairs, exits);
                    }
                }
                index = index.wrapping_add(step);
                if index == end {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(exits + 4, 0);
        let capacity = e.mem.u32(exits + 8);
        if (capacity as i32) >= 0 {
            let heap = e.mem.u32(router + 0x10);
            let data = e.mem.u32(exits);
            e.vcall(heap, 0x10, &args![data, (capacity & CAPACITY_MASK) << 3]);
        }
        e.mem.set_u32(exits, 0);
        e.mem.set_u32(exits + 8, NOT_OWNED);
        e.call(MEMORY_ROUTER, &args![]);
    });
}

/// `3.4028232e38` (`0x7f7fffee`, `float`): where the running minimum of the new boxes starts.
const RUNNING_MIN_START: u32 = 0x0104_da48;
/// `-3.4028232e38` (`0xff7fffee`, `float`): where the running maximum starts.
const RUNNING_MAX_START: u32 = 0x0104_e148;

// Translated from 00d18400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::addObjectBatch` (Xbox PDB): adds the objects of the `hkArray` `handles`
/// (each element points at the dword that receives the object's node index) with the
/// float boxes of the `hkArray` `boxes` (32 bytes each, min then max `hkVector4`), and
/// appends to `pairs` the pairs of new objects among themselves and with the old ones.
///
/// The boxes are quantised like `updateAabbs` (min even, max odd) into three local arrays of
/// `2 * count` endpoints (value, node index), each sorted by `quickSortRecursive`. The nodes
/// that overlap the union of the new boxes are collected with the search of
/// `querySingleAabb` (before the new endpoints exist), the new endpoints are merged into the
/// sweep's arrays (`insertTail` then the merge `00d19d90`, `00d19e40` or `00d19ef0`), the
/// markers' nodes move up by `2 * count`, and the new nodes and the collected ones are copied
/// out, sorted by `min_x` (radix sort) and swept with `collide1Axis`.
pub fn add_object_batch(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    handles: u32,
    boxes: u32,
    pairs: u32,
) {
    let me = this.addr();
    if e.mem.i32(boxes + 4) < 1 {
        return;
    }
    let count = e.mem.i32(handles + 4);
    let old_count = e.get(this, Hkp3AxisSweep::node_count);
    let doubled = count.wrapping_mul(2);
    let new_x = stack_alloc(e, doubled.wrapping_mul(4), 2);
    let new_y = stack_alloc(e, doubled.wrapping_mul(4), 2);
    let new_z = stack_alloc(e, doubled.wrapping_mul(4), 2);
    let new_arrays = [new_x.data, new_y.data, new_z.data];
    e.with_stack(0x30, |e, frame| {
        let candidates = frame.addr();
        let query_box = frame.addr() + 0x10;
        e.mem.set_u32(candidates, 0);
        e.mem.set_u32(candidates + 4, 0);
        e.mem.set_u32(candidates + 8, NOT_OWNED);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        if ((e.mem.u32(candidates + 8) & CAPACITY_MASK) as i32) < old_count {
            let allocator = e.mem.u32(router + 0xc);
            e.call(
                ARRAY_RESERVE,
                &args![allocator, candidates, old_count, 4u32],
            );
        }
        let wanted = e.get(this, Hkp3AxisSweep::node_count).wrapping_add(count);
        array_reserve_at_least(e, me + 0xa0, wanted, 0x10);
        e.mem.set_i32(me + 0xa4, wanted);

        // The new endpoints, per object, and the union of the boxes.
        let start_min = e.mem.f32(RUNNING_MIN_START);
        let start_max = e.mem.f32(RUNNING_MAX_START);
        let mut running_min = [start_min; 4];
        let mut running_max = [start_max; 4];
        let mut index = 0i32;
        while index < count {
            let box_at = e
                .mem
                .u32(boxes)
                .wrapping_add((index as u32).wrapping_mul(0x20));
            running_min = minps(running_min, load_vec4(e, box_at));
            running_max = maxps(running_max, load_vec4(e, box_at + 0x10));
            let low = quantise_corner(e, me, box_at, OFFSET_LOW);
            let high = quantise_corner(e, me, box_at + 0x10, OFFSET_HIGH);
            let handle = e.mem.u32(
                e.mem
                    .u32(handles)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let id = (old_count as u32).wrapping_add(index as u32);
            let nodes = e.get(this, Hkp3AxisSweep::nodes);
            e.mem.set_u32(
                nodes.wrapping_add(id.wrapping_mul(16)).wrapping_add(0xc),
                handle,
            );
            e.mem.set_u32(handle, id);
            for axis in 0..3 {
                let at = new_arrays[axis].wrapping_add((index as u32).wrapping_mul(8));
                e.mem.set_u16(at, (low[axis] & 0xfffe) as u16);
                e.mem.set_u16(at + 2, id as u16);
                e.mem.set_u16(at + 4, ((high[axis] & 0xffff) | 1) as u16);
                e.mem.set_u16(at + 6, id as u16);
            }
            index += 1;
        }
        e.mem.set_i32(me + 0xa4, old_count);

        // The old nodes that overlap the union of the new boxes.
        let bits = alloc_node_bits(e, old_count, false);
        store_vec4(e, query_box, running_min);
        store_vec4(e, query_box + 0x10, running_max);
        let (low, high) = mark_nodes_in_box(e, this, query_box, bits.data);
        for_each_node_in_box(e, this, bits.data, low, high, |e, node, _handle| {
            let size = e.mem.u32(candidates + 4);
            let slot = e.mem.u32(candidates).wrapping_add(size.wrapping_mul(4));
            e.mem.set_u32(slot, node);
            e.mem.set_u32(candidates + 4, size.wrapping_add(1));
        });
        let bits_end = bits
            .data
            .wrapping_add(((old_count >> 5) as u32).wrapping_mul(4))
            .wrapping_add(4);
        stack_free(e, &bits, 2);

        // The nodes join the sweep: sort the new endpoints, merge them into the arrays.
        let wanted = e.get(this, Hkp3AxisSweep::node_count).wrapping_add(count);
        array_reserve_at_least(e, me + 0xa0, wanted, 0x10);
        e.mem.set_i32(me + 0xa4, wanted);
        let nodes = e.get(this, Hkp3AxisSweep::nodes);
        for new in new_arrays {
            if doubled > 1 {
                quick_sort_recursive(e, new, 0, doubled.wrapping_sub(1), bits_end & 0xffff_ff00);
            }
        }
        let mut ends = [0i32; 3];
        for axis in 0..3 {
            ends[axis] = hkp_bp_axis_insert_tail(
                e,
                me + AXIS_ARRAYS[axis],
                nodes,
                axis as u32,
                new_arrays[axis],
                doubled,
            );
        }
        fn_00d19d90(e, me + 0xac, nodes, ends[0], new_x.data, doubled);
        fn_00d19e40(e, me + 0xb8, nodes, ends[1], new_y.data, doubled);
        fn_00d19ef0(e, me + 0xc4, nodes, ends[2], new_z.data, doubled);
        if e.get(this, Hkp3AxisSweep::marker_count) != 0 {
            let mut marker_index = 0i32;
            while marker_index < e.get(this, Hkp3AxisSweep::marker_count) {
                let marker = e
                    .get(this, Hkp3AxisSweep::markers)
                    .wrapping_add((marker_index as u32).wrapping_shl(4));
                let node = (e.mem.u16(marker) as u32)
                    .wrapping_shl(4)
                    .wrapping_add(e.get(this, Hkp3AxisSweep::nodes));
                let max_y = e.mem.u16(node + 4).wrapping_add(doubled as u16);
                e.mem.set_u16(node + 4, max_y);
                let max_z = e.mem.u16(node + 6).wrapping_add(doubled as u16);
                e.mem.set_u16(node + 6, max_z);
                marker_index += 1;
            }
        }

        // The new nodes, sorted by `min_x`.
        let new_nodes = stack_alloc(e, count.wrapping_add(1).wrapping_mul(16), 4);
        let new_keys = stack_alloc(e, count.wrapping_add(4).wrapping_mul(4), 2);
        let new_scratch = stack_alloc(e, count.wrapping_add(4).wrapping_mul(4), 2);
        let mut position = 0u32;
        while (position as i32) < count {
            let id = (old_count as u32).wrapping_add(position);
            let node = e
                .get(this, Hkp3AxisSweep::nodes)
                .wrapping_add(id.wrapping_shl(4));
            let key_at = new_keys.data.wrapping_add(position.wrapping_mul(4));
            e.mem.set_u16(key_at + 2, id as u16);
            let min_x = e.mem.u16(node + 8);
            e.mem.set_u16(key_at, min_x);
            position += 1;
        }
        let end_key = new_keys.data.wrapping_add((count as u32).wrapping_mul(4));
        e.mem.set_u16(end_key + 4, 0xffff);
        e.mem.set_u16(end_key, 0xffff);
        e.mem.set_u16(end_key + 8, 0xffff);
        e.call(
            RADIX_SORT_16,
            &args![
                new_keys.data,
                count.wrapping_add(3) as u32 & 0xffff_fffc,
                new_scratch.data
            ],
        );
        for position in 0..count.max(0) as u32 {
            let id = e.mem.u16(new_keys.data + 4 * position + 2) as u32;
            let node = id
                .wrapping_shl(4)
                .wrapping_add(e.get(this, Hkp3AxisSweep::nodes));
            copy_node(e, node, new_nodes.data + 16 * position);
        }
        let new_nodes_end = new_nodes.data.wrapping_add((count as u32).wrapping_shl(4));
        e.mem.set_u16(new_nodes_end + 8, 0xffff);
        stack_free(e, &new_scratch, 2);
        stack_free(e, &new_keys, 2);

        // The collected old nodes, sorted by `min_x`.
        let found = e.mem.i32(candidates + 4);
        let old_nodes = stack_alloc(e, found.wrapping_add(1).wrapping_mul(16), 4);
        let old_keys = stack_alloc(e, found.wrapping_add(4).wrapping_mul(4), 2);
        let old_scratch = stack_alloc(e, found.wrapping_add(4).wrapping_mul(4), 2);
        let nodes = e.get(this, Hkp3AxisSweep::nodes);
        let mut position = 0i32;
        while position < found {
            let node = e.mem.u32(
                e.mem
                    .u32(candidates)
                    .wrapping_add((position as u32).wrapping_mul(4)),
            );
            let key_at = old_keys
                .data
                .wrapping_add((position as u32).wrapping_mul(4));
            e.mem
                .set_u16(key_at + 2, (node.wrapping_sub(nodes) as i32 >> 4) as u16);
            let min_x = e.mem.u16(node + 8);
            e.mem.set_u16(key_at, min_x);
            position += 1;
        }
        let end_key = old_keys.data.wrapping_add((found as u32).wrapping_mul(4));
        e.mem.set_u16(end_key + 4, 0xffff);
        e.mem.set_u16(end_key, 0xffff);
        e.mem.set_u16(end_key + 8, 0xffff);
        e.call(
            RADIX_SORT_16,
            &args![
                old_keys.data,
                found.wrapping_add(3) as u32 & 0xffff_fffc,
                old_scratch.data
            ],
        );
        for position in 0..found.max(0) as u32 {
            let id = e.mem.u16(old_keys.data + 4 * position + 2) as u32;
            let node = id
                .wrapping_shl(4)
                .wrapping_add(e.get(this, Hkp3AxisSweep::nodes));
            copy_node(e, node, old_nodes.data + 16 * position);
        }
        let old_nodes_end = old_nodes.data.wrapping_add((found as u32).wrapping_shl(4));
        e.mem.set_u16(old_nodes_end + 8, 0xffff);
        stack_free(e, &old_scratch, 2);
        stack_free(e, &old_keys, 2);

        collide_1_axis_one_list(e, me, new_nodes.data, count, pairs);
        collide_1_axis(
            e,
            this,
            new_nodes.data,
            count,
            old_nodes.data,
            found,
            0,
            pairs,
        );
        stack_free(e, &old_nodes, 4);
        stack_free(e, &new_nodes, 4);
        e.mem.set_u32(candidates + 4, 0);
        release_stack_array(e, candidates, 2);
        e.mem.set_u32(candidates, 0);
        e.mem.set_u32(candidates + 8, NOT_OWNED);
    });
    stack_free(e, &new_z, 2);
    stack_free(e, &new_y, 2);
    stack_free(e, &new_x, 2);
}

// ---------------------------------------------------------------------------
// Ray and box casts.
// ---------------------------------------------------------------------------

/// `-2.0` (`float`): the "no crossing" time castAabb gives an axis that does not move.
const NEG_TWO_FLOAT: u32 = 0x0104_a494;
/// `2.0` (`float`): the time given to an endpoint list that ran out of endpoints.
const TWO_FLOAT: u32 = 0x0101_62c0;
/// `2^-23` (`double`, 1.1920929e-07): how small a movement per axis counts as none.
const TINY_DOUBLE: u32 = 0x010c_9300;

/// The index (0, 1 or 2) of the smallest of three floats, by the `FCOM` flag tests the
/// casts use: a value that is not greater (NaN included) loses to the earlier one.
fn pick_smallest(values: [f32; 3]) -> usize {
    if values[1] > values[0] {
        if values[2] > values[0] {
            0
        } else {
            2
        }
    } else if values[2] > values[1] {
        1
    } else {
        2
    }
}

/// What the cast does with the float a collector's slot `+4` returns for `(handle, 0)`:
/// `FSTP` to a `float`, then it replaces the best fraction unless it is greater (a NaN does
/// replace it).
fn collect_fraction(e: &mut Engine, collector: u32, handle: u32, second: u32, best: &mut f32) {
    let returned = e.vcall(collector, 4, &args![handle, second]).st0 as f32;
    if returned <= *best || returned.is_nan() || best.is_nan() {
        *best = returned;
    }
}

/// `byte[id] ^= mask`.
fn xor_byte(e: &mut Engine, bytes: u32, id: u32, mask: u8) {
    let at = bytes.wrapping_add(id);
    let value = e.mem.u8(at);
    e.mem.set_u8(at, value ^ mask);
}

// Translated from 00d16c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::castAabb` (Xbox PDB): sweeps a box along a segment and reports to the
/// collector (vtable slot `+4`, `(handle, 0)`, returns the hit fraction as a float) every
/// object whose box the swept box meets, narrowing the fraction to the smallest returned.
///
/// `input`: `from` (`hkVector4` at +0), `to` (+0x10), the box's half extents (+0x20) and an
/// optional pointer (+0x30) to three `hkArray`s that stand for the x, y and z endpoint arrays
/// (0: the sweep's own).
///
/// A byte per node holds one bit per axis: the node's interval on that axis overlaps the box
/// at the start. Per axis two cursors then walk the endpoint array in the direction of the
/// movement, one at the box's far side and one at its near side, each remembering the
/// time (`value * inverse - start`) its next endpoint is met; the smaller time moves first.
/// A node whose byte reaches 7 when the far cursor passes it is reported.
pub fn cast_aabb(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, input: u32, collector: u32) {
    let me = this.addr();
    let center = load_vec4(e, input);
    let extent = load_vec4(e, input + 0x20);
    let upper = add4(center, extent);
    let lower = sub4(center, extent);
    let quantised_low = quantise_vec(e, me, lower, OFFSET_LOW);
    let quantised_high = quantise_vec(e, me, upper, OFFSET_LOW);
    let mut first_value = [0u32; 3];
    let mut last_value = [0u32; 3];
    for axis in 0..3 {
        first_value[axis] = quantised_low[axis] & 0xffff;
        last_value[axis] = quantised_high[axis] & 0xffff;
    }
    let count = e.get(this, Hkp3AxisSweep::node_count);
    let bytes = stack_alloc(e, count.wrapping_add(0x10), 0);
    let mut blocks = count >> 4;
    let mut at = bytes.data;
    while blocks >= 0 {
        for i in 0..4 {
            e.mem.set_u32(at + 4 * i, 0);
        }
        at = at.wrapping_add(16);
        blocks -= 1;
    }
    let override_arrays = e.mem.u32(input + 0x30);
    let arrays = if override_arrays != 0 {
        override_arrays
    } else {
        me + 0xac
    };

    // The bits of the nodes that overlap the box at the start, and the two cursors per axis.
    let mut near_cursor = [0u32; 3];
    let mut far_cursor = [0u32; 3];
    for axis in 0..3usize {
        let array = arrays + 12 * axis as u32;
        let data = e.mem.u32(array);
        let size = e.mem.i32(array + 4);
        let coordinate = first_value[axis];
        let end = last_value[axis];
        let mask = 1u8 << axis;
        let middle =
            e.mem
                .u16(data.wrapping_add(((size >> 1) as u32).wrapping_mul(4))) as u32;
        let id_at = |e: &Engine, at: u32, offset: u32| e.mem.u16(at.wrapping_add(offset)) as u32;
        if coordinate < middle {
            let limit = data
                .wrapping_add((size as u32).wrapping_mul(4))
                .wrapping_sub(0x10);
            let mut at = data.wrapping_add(4);
            if at < limit {
                loop {
                    if id_at(e, at, 0xc) > coordinate {
                        break;
                    }
                    for offset in [2, 6, 0xa, 0xe] {
                        let id = id_at(e, at, offset);
                        xor_byte(e, bytes.data, id, mask);
                    }
                    at = at.wrapping_add(0x10);
                    if at >= limit {
                        break;
                    }
                }
            }
            while id_at(e, at, 0) <= coordinate {
                let id = id_at(e, at, 2);
                xor_byte(e, bytes.data, id, mask);
                at = at.wrapping_add(4);
            }
            near_cursor[axis] = at;
            while id_at(e, at, 0) <= end {
                let toggle = (e.mem.u8(at) & 1).wrapping_sub(1) & mask;
                let id = id_at(e, at, 2);
                xor_byte(e, bytes.data, id, toggle);
                at = at.wrapping_add(4);
            }
            far_cursor[axis] = at;
        } else {
            let first = data.wrapping_add(0x10);
            let mut at = data
                .wrapping_add((size as u32).wrapping_mul(4))
                .wrapping_sub(8);
            if at >= first {
                loop {
                    if id_at(e, at, 0u32.wrapping_sub(0xc)) <= end {
                        break;
                    }
                    for offset in [
                        2u32,
                        0u32.wrapping_sub(2),
                        0u32.wrapping_sub(6),
                        0u32.wrapping_sub(0xa),
                    ] {
                        let id = id_at(e, at, offset);
                        xor_byte(e, bytes.data, id, mask);
                    }
                    at = at.wrapping_sub(0x10);
                    if at < first {
                        break;
                    }
                }
            }
            while id_at(e, at, 0) > end {
                let id = id_at(e, at, 2);
                xor_byte(e, bytes.data, id, mask);
                at = at.wrapping_sub(4);
            }
            far_cursor[axis] = at.wrapping_add(4);
            while id_at(e, at, 0) > coordinate {
                let toggle = 0u8.wrapping_sub(e.mem.u8(at) & 1) & mask;
                let id = id_at(e, at, 2);
                xor_byte(e, bytes.data, id, toggle);
                at = at.wrapping_sub(4);
            }
            near_cursor[axis] = at.wrapping_add(4);
        }
    }

    // The nodes inside the box at the start are hit at fraction 0.
    let mut best = 1.0f32;
    let nodes = e.get(this, Hkp3AxisSweep::nodes);
    let words = (e.get(this, Hkp3AxisSweep::node_count) >> 2) as u32;
    let limit = bytes
        .data
        .wrapping_add(words.wrapping_mul(4))
        .wrapping_add(4);
    let mut at = bytes.data;
    let mut node = nodes;
    while at < limit {
        let word = e.mem.u32(at);
        if word.wrapping_add(0x0101_0101) & 0x0808_0808 != 0 {
            for position in 0..4u32 {
                if e.mem.u8(at + position) == 7 {
                    let handle = e
                        .mem
                        .u32(node.wrapping_add(position * 16).wrapping_add(0xc));
                    if handle & 1 == 0 {
                        collect_fraction(e, collector, handle, 0, &mut best);
                    }
                }
            }
        }
        node = node.wrapping_add(0x40);
        at = at.wrapping_add(4);
    }

    // The movement per axis: the step direction, the inverse of the scaled movement, and the
    // times the near and far sides of the box meet the first endpoint.
    let to = load_vec4(e, input + 0x10);
    let movement = sub4(to, center);
    let negative_two = x87_quiet(e.mem.f32(NEG_TWO_FLOAT));
    let tiny = e.mem.f64(TINY_DOUBLE);
    let exponent_offset = e.get(this, Hkp3AxisSweep::exponent_offset);
    let mut low_side = lower;
    let mut high_side = upper;
    let mut step = [0i32; 3];
    let mut near_flag = [0u32; 3];
    let mut far_flag = [0u32; 3];
    let mut inverse = [0f32; 3];
    let mut start_time = [0f32; 3];
    let mut end_time = [0f32; 3];
    for axis in 0..3usize {
        let scale = e.mem.f32(me + SCALE + 4 * axis as u32);
        let offset = e.mem.f32(me + OFFSET_LOW + 4 * axis as u32);
        let scaled = (movement[axis] as f64 * scale as f64) as f32;
        let magnitude = scaled.abs();
        if scaled > 0.0 {
            step[axis] = 4;
            near_flag[axis] = 0;
            far_flag[axis] = 1;
        } else {
            let low = x87_quiet(low_side[axis]);
            let high = x87_quiet(high_side[axis]);
            low_side[axis] = high;
            high_side[axis] = low;
            step[axis] = -4;
            std::mem::swap(&mut near_cursor[axis], &mut far_cursor[axis]);
            near_cursor[axis] = near_cursor[axis].wrapping_sub(4);
            far_cursor[axis] = far_cursor[axis].wrapping_sub(4);
            near_flag[axis] = 1;
            far_flag[axis] = 0;
        }
        let low_scaled = ((low_side[axis] as f64 + offset as f64) * scale as f64) as f32;
        let end_scaled = ((offset as f64 + to[axis] as f64) * scale as f64) as f32;
        if low_scaled as f64 * tiny > magnitude as f64
            || end_scaled as f64 * tiny > magnitude as f64
        {
            inverse[axis] = 0.0;
            start_time[axis] = negative_two;
            end_time[axis] = negative_two;
        } else {
            let reciprocal = (1.0f64 / scaled as f64) as f32;
            inverse[axis] = reciprocal;
            let near_distance = (low_scaled as f64 - exponent_offset as f64) as f32;
            start_time[axis] = (near_distance as f64 * reciprocal as f64) as f32;
            let high_scaled = ((high_side[axis] as f64 + offset as f64) * scale as f64) as f32;
            let far_distance = (high_scaled as f64 - exponent_offset as f64) as f32;
            end_time[axis] = (reciprocal as f64 * far_distance as f64) as f32;
        }
    }

    // The sweep: `times` is the next time of each near cursor (`[0..3]`) and far cursor
    // (`[4..7]`) in memory, since `00d19d10` picks from them by address.
    e.with_stack(0x30, |e, frame| {
        let near_times = frame.addr();
        let far_times = frame.addr() + 0x10;
        let scratch = frame.addr() + 0x20;
        for axis in 0..3usize {
            let near_value = e.mem.u16(near_cursor[axis]) as f64;
            let far_value = e.mem.u16(far_cursor[axis]) as f64;
            let near = (near_value * inverse[axis] as f64 - start_time[axis] as f64) as f32;
            let far = (far_value * inverse[axis] as f64 - end_time[axis] as f64) as f32;
            e.mem.set_f32(near_times + 4 * axis as u32, near);
            e.mem.set_f32(far_times + 4 * axis as u32, far);
        }
        e.mem.set_u8(bytes.data, 8);
        let times = |e: &Engine, base: u32| [0u32, 4, 8].map(|o| e.mem.f32(base + o));
        let mut near_axis = pick_smallest(times(e, near_times));
        let mut far_axis = pick_smallest(times(e, far_times));
        loop {
            let near_time = e.mem.f32(near_times + 4 * near_axis as u32);
            let far_time = e.mem.f32(far_times + 4 * far_axis as u32);
            if near_time > far_time {
                if best < far_time {
                    break;
                }
                // The far cursor of `far_axis` moves.
                let axis = far_axis;
                loop {
                    let cursor = far_cursor[axis];
                    let value = e.mem.u16(cursor) as u32;
                    let id = e.mem.u16(cursor + 2) as u32;
                    let toggle = ((value & 1) ^ far_flag[axis]) << axis;
                    xor_byte(e, bytes.data, id, toggle as u8);
                    if e.mem.u8(bytes.data + id) >= 7 {
                        if id == 0 {
                            let two = x87_quiet(e.mem.f32(TWO_FLOAT));
                            e.mem.set_f32(far_times + 4 * axis as u32, two);
                            far_axis = fn_00d19d10(e, far_times, scratch) as usize;
                            break;
                        }
                        let node = e
                            .get(this, Hkp3AxisSweep::nodes)
                            .wrapping_add(id.wrapping_shl(4));
                        if toggle != 0 {
                            let handle = e.mem.u32(node + 0xc);
                            if handle & 1 == 0 {
                                collect_fraction(e, collector, handle, 0, &mut best);
                            }
                        }
                    }
                    let next = cursor.wrapping_add(step[axis] as u32);
                    far_cursor[axis] = next;
                    if e.mem.u16(cursor) != e.mem.u16(next) {
                        let value = e.mem.u16(next) as f64;
                        let time = (inverse[axis] as f64 * value - end_time[axis] as f64) as f32;
                        e.mem.set_f32(far_times + 4 * axis as u32, time);
                        far_axis = fn_00d19d10(e, far_times, scratch) as usize;
                        break;
                    }
                }
            } else {
                if best < near_time {
                    break;
                }
                // The near cursor of `near_axis` moves.
                let axis = near_axis;
                loop {
                    let cursor = near_cursor[axis];
                    let value = e.mem.u8(cursor);
                    let id = e.mem.u16(cursor + 2) as u32;
                    let toggle = ((value & 1) ^ near_flag[axis] as u8) << axis;
                    xor_byte(e, bytes.data, id, toggle);
                    if e.mem.u8(bytes.data + id) > 8 {
                        let two = x87_quiet(e.mem.f32(TWO_FLOAT));
                        e.mem.set_f32(near_times + 4 * axis as u32, two);
                        near_axis = fn_00d19d10(e, near_times, scratch) as usize;
                        break;
                    }
                    let next = cursor.wrapping_add(step[axis] as u32);
                    near_cursor[axis] = next;
                    if e.mem.u16(cursor) != e.mem.u16(next) {
                        let value = e.mem.u16(next) as f64;
                        let time = (inverse[axis] as f64 * value - start_time[axis] as f64) as f32;
                        e.mem.set_f32(near_times + 4 * axis as u32, time);
                        near_axis = fn_00d19d10(e, near_times, scratch) as usize;
                        break;
                    }
                }
            }
        }
    });
    stack_free(e, &bytes, 0);
}

// Translated from 00d16300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkp3AxisSweep::castRay` (Xbox PDB): casts the rays of `input` through the sweep and
/// reports to the collectors every object a ray enters, narrowing the fraction per ray.
///
/// `input`: the origin (`hkVector4` at +0), the number of rays (+0x10), a pointer (+0x14) to
/// the first ray's end point and the distance between two (+0x18), and an optional pointer
/// (+0x1c) to three `hkArray`s that stand for the x, y and z endpoint arrays (0: the
/// sweep's own). `collectors` is the first collector object and `stride` the distance to the
/// next: each is called through vtable slot `+4` with `(handle, ray index)` and returns the
/// hit fraction as a float.
///
/// A byte per node holds the axis bits twice (`0x11 << axis`): which of its intervals contain
/// the origin. A node whose byte is `0x77` contains the origin on all axes and is reported to
/// every ray at once. Per ray one cursor per axis then walks the endpoints from the origin
/// in the direction of the ray, always the axis whose next endpoint is met first, toggling the
/// high bit (`0x10 << axis`) of the node; a byte of `0x70` or more is a hit. Before the next ray the low
/// nibbles are copied over the high ones again. The per-ray fractions start in an array
/// taken from the router's bump pointer (put back at the end).
pub fn cast_ray(
    e: &mut Engine,
    this: Ptr<Hkp3AxisSweep>,
    input: u32,
    collectors: u32,
    stride: u32,
) {
    let me = this.addr();
    let origin = load_vec4(e, input);
    let quantised = quantise_vec(e, me, origin, OFFSET_LOW);
    let origin_values = [0, 1, 2].map(|lane: usize| quantised[lane] & 0xffff);
    let count = e.get(this, Hkp3AxisSweep::node_count);
    let bytes = stack_alloc(e, count.wrapping_add(0x10), 0);
    let mut blocks = count >> 4;
    let mut at = bytes.data;
    while blocks >= 0 {
        for i in 0..4 {
            e.mem.set_u32(at + 4 * i, 0);
        }
        at = at.wrapping_add(16);
        blocks -= 1;
    }
    let override_arrays = e.mem.u32(input + 0x1c);
    let arrays = if override_arrays != 0 {
        override_arrays
    } else {
        me + 0xac
    };
    let mut start_cursor = [0u32; 3];
    let mut mask = 0x11u8;
    for axis in 0..3usize {
        let array = arrays + 12 * axis as u32;
        let data = e.mem.u32(array);
        let size = e.mem.i32(array + 4);
        let coordinate = origin_values[axis];
        let value_at = |e: &Engine, at: u32, offset: u32| e.mem.u16(at.wrapping_add(offset)) as u32;
        let middle =
            e.mem
                .u16(data.wrapping_add(((size >> 1) as u32).wrapping_mul(4))) as u32;
        if coordinate < middle {
            let limit = data
                .wrapping_add((size as u32).wrapping_mul(4))
                .wrapping_sub(0x10);
            let mut at = data.wrapping_add(4);
            if at < limit {
                loop {
                    if value_at(e, at, 0xc) > coordinate {
                        break;
                    }
                    for offset in [2, 6, 0xa, 0xe] {
                        let id = value_at(e, at, offset);
                        xor_byte(e, bytes.data, id, mask);
                    }
                    at = at.wrapping_add(0x10);
                    if at >= limit {
                        break;
                    }
                }
            }
            while value_at(e, at, 0) <= coordinate {
                let id = value_at(e, at, 2);
                xor_byte(e, bytes.data, id, mask);
                at = at.wrapping_add(4);
            }
            start_cursor[axis] = at;
        } else {
            let first = data.wrapping_add(0x10);
            let mut at = data
                .wrapping_add((size as u32).wrapping_mul(4))
                .wrapping_sub(8);
            if at >= first {
                loop {
                    if value_at(e, at, 0u32.wrapping_sub(0xc)) <= coordinate {
                        break;
                    }
                    for offset in [
                        2u32,
                        0u32.wrapping_sub(2),
                        0u32.wrapping_sub(6),
                        0u32.wrapping_sub(0xa),
                    ] {
                        let id = value_at(e, at, offset);
                        xor_byte(e, bytes.data, id, mask);
                    }
                    at = at.wrapping_sub(0x10);
                    if at < first {
                        break;
                    }
                }
            }
            while value_at(e, at, 0) > coordinate {
                let id = value_at(e, at, 2);
                xor_byte(e, bytes.data, id, mask);
                at = at.wrapping_sub(4);
            }
            start_cursor[axis] = at.wrapping_add(4);
        }
        mask = mask.wrapping_add(mask);
    }

    // The fractions, taken from the router's bump pointer, start at 1.
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let saved_top = e.mem.u32(router);
    let ray_total = e.mem.i32(input + 0x10);
    e.mem.set_u32(
        router,
        ((ray_total as u32).wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80)
            .wrapping_add(saved_top),
    );
    let results = saved_top;
    let mut ray = 0u32;
    while (e.mem.i32(input + 0x10) as i64) > ray as i64 {
        e.mem.set_f32(results + 4 * ray, 1.0);
        ray += 1;
    }

    // The nodes that contain the origin on all axes are hit by every ray.
    let nodes = e.get(this, Hkp3AxisSweep::nodes);
    let words = (e.get(this, Hkp3AxisSweep::node_count) >> 2) as u32;
    let limit = bytes
        .data
        .wrapping_add(words.wrapping_mul(4))
        .wrapping_add(4);
    let marks_something =
        |e: &Engine, at: u32| e.mem.u32(at).wrapping_add(0x0101_0101) & 0x0808_0808 != 0;
    let mut at = bytes.data;
    while at < limit {
        if marks_something(e, at) {
            let first_node = nodes.wrapping_add(at.wrapping_sub(bytes.data).wrapping_mul(16));
            for position in 0..4u32 {
                if e.mem.u8(at + position) != 0x77 {
                    continue;
                }
                let handle_at = first_node.wrapping_add(position * 16).wrapping_add(0xc);
                if e.mem.u8(handle_at) & 1 != 0 {
                    continue;
                }
                let handle = e.mem.u32(handle_at);
                let mut collector = collectors;
                let mut ray = 0u32;
                while (e.mem.i32(input + 0x10) as i64) > ray as i64 {
                    let mut fraction = x87_quiet(e.mem.f32(results + 4 * ray));
                    collect_fraction(e, collector, handle, ray, &mut fraction);
                    e.mem.set_f32(results + 4 * ray, fraction);
                    collector = collector.wrapping_add(stride);
                    ray += 1;
                }
            }
            at = at.wrapping_add(4);
        } else if marks_something(e, at + 4) {
            at = at.wrapping_add(4);
        } else if marks_something(e, at + 8) {
            at = at.wrapping_add(8);
        } else {
            at = at.wrapping_add(12);
        }
    }
    e.mem.set_u8(bytes.data, 0x88);

    // Each ray, from the origin.
    let tiny = e.mem.f64(TINY_DOUBLE);
    let exponent_offset = e.get(this, Hkp3AxisSweep::exponent_offset);
    let mut step = [0i32; 3];
    let mut collector = collectors;
    let mut ray = 0u32;
    while (ray as i32) < e.mem.i32(input + 0x10) {
        let mut cursor = start_cursor;
        let mut best = x87_quiet(e.mem.f32(results + 4 * ray));
        let end_at = e
            .mem
            .u32(input + 0x18)
            .wrapping_mul(ray)
            .wrapping_add(e.mem.u32(input + 0x14));
        let end = load_vec4(e, end_at);
        let delta = sub4(end, origin);
        let mut inverse = [0f32; 3];
        let mut offset_time = [0f32; 3];
        for axis in 0..3usize {
            let scale = e.mem.f32(me + SCALE + 4 * axis as u32);
            let offset = e.mem.f32(me + OFFSET_LOW + 4 * axis as u32);
            let scaled = (delta[axis] as f64 * scale as f64) as f32;
            let magnitude = scaled.abs();
            let origin_scaled = ((offset as f64 + origin[axis] as f64) * scale as f64) as f32;
            let degenerate = origin_scaled as f64 * tiny > magnitude as f64 || {
                let end_scaled = ((end[axis] as f64 + offset as f64) * scale as f64) as f32;
                end_scaled as f64 * tiny > magnitude as f64
            };
            if degenerate {
                inverse[axis] = 0.0;
                offset_time[axis] = x87_quiet(e.mem.f32(NEG_TWO_FLOAT));
            } else {
                step[axis] = 4;
                let reciprocal = (1.0f64 / scaled as f64) as f32;
                if scaled < 0.0 {
                    step[axis] = -4;
                    cursor[axis] = cursor[axis].wrapping_sub(4);
                }
                inverse[axis] = reciprocal;
                let distance = (origin_scaled as f64 - exponent_offset as f64) as f32;
                offset_time[axis] = (reciprocal as f64 * distance as f64) as f32;
            }
        }
        let time_at = |e: &Engine, cursor: u32, axis: usize| {
            let value = e.mem.u16(cursor) as f64;
            (inverse[axis] as f64 * value - offset_time[axis] as f64) as f32
        };
        let mut next = [0f32; 3];
        for axis in 0..3usize {
            next[axis] = time_at(e, cursor[axis], axis);
        }
        let mut axis = pick_smallest(next);
        let mut soonest = x87_quiet(next[axis]);
        while !(best < soonest) {
            let toggle = 1u8 << (axis + 4);
            loop {
                let at = cursor[axis];
                let id = e.mem.u16(at + 2) as u32;
                xor_byte(e, bytes.data, id, toggle);
                if e.mem.u8(bytes.data + id) >= 0x70 {
                    if id == 0 {
                        next[axis] = x87_quiet(e.mem.f32(TWO_FLOAT));
                        break;
                    }
                    let node = e
                        .get(this, Hkp3AxisSweep::nodes)
                        .wrapping_add(id.wrapping_shl(4));
                    let handle = e.mem.u32(node + 0xc);
                    if handle & 1 == 0 {
                        collect_fraction(e, collector, handle, ray, &mut best);
                    }
                }
                let following = at.wrapping_add(step[axis] as u32);
                cursor[axis] = following;
                if e.mem.u16(at) != e.mem.u16(following) {
                    next[axis] = time_at(e, following, axis);
                    break;
                }
            }
            axis = pick_smallest(next);
            soonest = x87_quiet(next[axis]);
        }
        if (ray as i32) < e.mem.i32(input + 0x10).wrapping_sub(1) {
            // The low nibbles go over the high ones again, two words at a time.
            let end_of_bytes = bytes
                .data
                .wrapping_add(((count >> 2) as u32).wrapping_mul(4))
                .wrapping_add(4);
            let mut at = bytes.data;
            if at < end_of_bytes {
                loop {
                    for word_at in [at, at + 4] {
                        let low = e.mem.u32(word_at) & 0x0f0f_0f0f;
                        e.mem.set_u32(word_at, (low << 4) | low);
                    }
                    at = at.wrapping_add(8);
                    if at >= end_of_bytes {
                        break;
                    }
                }
            }
        }
        collector = collector.wrapping_add(stride);
        ray += 1;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, results);
    stack_free(e, &bytes, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d10ac0, get_type(Ptr<Hkp3AxisSweep>) -> u32),
        entry!(0x00d10b30, fn_00d10b30(u32, u32, u32, u32)),
        entry!(0x00d10c40, fn_00d10c40(u32, u32, i32, u32)),
        entry!(0x00d10d10, fn_00d10d10(u32, u32, u32, u32)),
        entry!(0x00d10e10, get_extents(Ptr<Hkp3AxisSweep>, u32, u32)),
        entry!(0x00d10ea0, hkp_bp_axis_find(u32, u32, u32, u16) -> u32),
        entry!(
            0x00d10f00,
            shift_broad_phase(Ptr<Hkp3AxisSweep>, u32, u32, u32)
        ),
        entry!(0x00d10ff0, check_determinism(Ptr<Hkp3AxisSweep>)),
        entry!(
            0x00d11020,
            set_32_bit_offset_and_scale(Ptr<Hkp3AxisSweep>, u32, u32, u32)
        ),
        entry!(
            0x00d112c0,
            set_bits_based_on_x_interval(Ptr<Hkp3AxisSweep>, i32, i32, Ptr<HkpBpNode>, u16, u32)
        ),
        entry!(
            0x00d11470,
            are_aabbs_overlapping(Ptr<Hkp3AxisSweep>, u32, u32) -> bool
        ),
        entry!(0x00d114e0, add_object(Ptr<Hkp3AxisSweep>, u32, u32, u32)),
        entry!(0x00d11620, get_num_objects(Ptr<Hkp3AxisSweep>) -> i32),
        entry!(0x00d11630, get_aabb_cache_size(Ptr<Hkp3AxisSweep>) -> i32),
        entry!(
            0x00d11650,
            get_aabb_from_node(Ptr<Hkp3AxisSweep>, Ptr<HkpBpNode>, u32)
        ),
        entry!(
            0x00d11790,
            fix_deterministic_order_after_node_id_was_decreased(Ptr<Hkp3AxisSweep>, u32)
        ),
        entry!(0x00d11960, fn_00d11960(u32, u32, u32)),
        entry!(0x00d119c0, get_aabb(Ptr<Hkp3AxisSweep>, u32, u32)),
        entry!(
            0x00d119e0,
            calc_content_statistics(Ptr<Hkp3AxisSweep>, u32, u32)
        ),
        entry!(0x00d11bc0, fn_00d11bc0(u32, u32, u32)),
        entry!(0x00d11c10, fn_00d11c10(u32, u32, u32)),
        entry!(0x00d11c60, fn_00d11c60(u32, u32, u16, u32, u32)),
        entry!(0x00d11d00, fn_00d11d00(u32, u32, u16, u32, u32)),
        entry!(
            0x00d11da0,
            hkp_bp_axis_insert(u32, u32, u32, u16, u16, u32, u32)
        ),
        entry!(0x00d11e80, hkp_bp_axis_insert_tail(u32, u32, u32, u32, i32) -> i32),
        entry!(0x00d11fa0, get_all_aabbs(Ptr<Hkp3AxisSweep>, u32)),
        entry!(
            0x00d12040,
            update_aabbs(Ptr<Hkp3AxisSweep>, u32, u32, i32, u32, u32)
        ),
        entry!(
            0x00d12b80,
            update_aabbs_uint32(Ptr<Hkp3AxisSweep>, u32, u32, i32, u32, u32)
        ),
        entry!(
            0x00d13620,
            collide_1_axis(Ptr<Hkp3AxisSweep>, u32, i32, u32, i32, i32, u32)
        ),
        entry!(0x00d138a0, collide_1_axis_one_list(u32, u32, i32, u32)),
        entry!(
            0x00d13950,
            calc_aabb_cache_internal(Ptr<Hkp3AxisSweep>, u32, u32)
        ),
        entry!(
            0x00d13bd0,
            hkp_3_axis_sweep(Ptr<Hkp3AxisSweep>, u32, u32, i32) -> Ptr<Hkp3AxisSweep>
        ),
        entry!(
            0x00d14100,
            re_query_single_object(Ptr<Hkp3AxisSweep>, u32, u32)
        ),
        entry!(
            0x00d142b0,
            add_object_ov2(Ptr<Hkp3AxisSweep>, u32, u32, u32)
        ),
        entry!(0x00d14600, remove_object(Ptr<Hkp3AxisSweep>, u32, u32)),
        entry!(0x00d14ef0, query_single_aabb(Ptr<Hkp3AxisSweep>, u32, u32)),
        entry!(
            0x00d154a0,
            query_single_aabb_with_collector(Ptr<Hkp3AxisSweep>, u32, u32)
        ),
        entry!(0x00d159a0, calc_aabb_cache(Ptr<Hkp3AxisSweep>, u32, u32)),
        entry!(
            0x00d15f10,
            calc_aabb_cache_ov2(Ptr<Hkp3AxisSweep>, u32, u32)
        ),
        entry!(0x00d15fc0, defragment(Ptr<Hkp3AxisSweep>)),
        entry!(0x00d182a0, fn_00d182a0(u32, u32, i32) -> Ptr<Hkp3AxisSweep>),
        entry!(0x00d182e0, hkp_3_axis_sweep_destructor(Ptr<Hkp3AxisSweep>)),
        entry!(0x00d19d10, fn_00d19d10(u32, u32) -> u32),
        entry!(0x00d19d70, fn_00d19d70(u16, u16) -> u16),
        entry!(0x00d19d90, fn_00d19d90(u32, u32, i32, u32, i32)),
        entry!(0x00d19e40, fn_00d19e40(u32, u32, i32, u32, i32)),
        entry!(0x00d19ef0, fn_00d19ef0(u32, u32, i32, u32, i32)),
        entry!(0x00d19fa0, quick_sort_recursive(u32, i32, i32, u32)),
        entry!(0x00d1a040, fn_00d1a040(u32, u32, u32)),
        entry!(0x00d1a0e0, fn_00d1a0e0(u32, u32, u32)),
        entry!(0x00d1a180, fn_00d1a180(u32, u32, u32)),
        entry!(0x00d1a220, fn_00d1a220(u32, i32, i32, u32)),
        entry!(
            0x00d1a2a0,
            get_offset_low_high_32_bit(Ptr<Hkp3AxisSweep>, u32, u32)
        ),
        entry!(
            0x00d1a2d0,
            hkp_3_axis_sweep_scalar_deleting_destructor(
                Ptr<Hkp3AxisSweep>,
                u32,
            ) -> Ptr<Hkp3AxisSweep>
        ),
        entry!(0x00d16300, cast_ray(Ptr<Hkp3AxisSweep>, u32, u32, u32)),
        entry!(0x00d16c50, cast_aabb(Ptr<Hkp3AxisSweep>, u32, u32)),
        entry!(
            0x00d175f0,
            shift_all_objects(Ptr<Hkp3AxisSweep>, u32, u32, u32)
        ),
        entry!(
            0x00d18400,
            add_object_batch(Ptr<Hkp3AxisSweep>, u32, u32, u32)
        ),
        entry!(
            0x00d193e0,
            remove_object_batch(Ptr<Hkp3AxisSweep>, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    const ROUTER: u32 = 0x0030_0000;
    const STACK_ALLOCATOR: u32 = 0x0030_1000;
    const HEAP_ALLOCATOR: u32 = 0x0030_2000;
    const ALLOCATOR_VTABLES: u32 = 0x0030_3000;
    const BUMP_REGION: u32 = 0x0034_0000;
    /// Addresses the fake allocators are registered at (not exe addresses).
    const FAKE_STACK_ALLOC: u32 = 0x7000_0001;
    const FAKE_STACK_FREE: u32 = 0x7000_0002;
    const FAKE_HEAP_NOOP: u32 = 0x7000_0003;
    /// A collector object: vtable at +0, the float its slot +4 returns at +8.
    const COLLECTOR_VTABLE: u32 = 0x0030_4000;
    const FAKE_COLLECT: u32 = 0x7000_0004;
    const FAKE_STATISTICS: u32 = 0x7000_0005;
    const HEAP_ALLOC_FN: u32 = 0x7000_0006;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn put_f32s(e: &mut Engine, addr: u32, values: &[f32]) {
        e.map(addr, 4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(addr + 4 * i as u32, *v);
        }
    }

    fn put_u32s(e: &mut Engine, addr: u32, values: &[u32]) {
        e.map(addr, 4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(addr + 4 * i as u32, *v);
        }
    }

    /// Makes the array hold `capacity` elements of `element` bytes, keeping its contents.
    fn grow_to(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let old = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        let data = e.mem.alloc(capacity.max(1) * element);
        if old != 0 {
            let bytes = e.mem.bytes(old, size * element);
            e.mem.write(data, &bytes);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
    }

    fn fake_grow(e: &mut Engine, a: &[u32]) -> Ret {
        let capacity = e.mem.u32(a[1] + 8) & CAPACITY_MASK;
        grow_to(e, a[1], (capacity * 2).max(4), a[2]);
        Ret::default()
    }

    fn fake_reserve(e: &mut Engine, a: &[u32]) -> Ret {
        grow_to(e, a[1], a[2], a[3]);
        Ret::default()
    }

    fn fake_easy_alloc(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.alloc(a[1]).into_ret()
    }

    fn fake_router(_: &mut Engine, _: &[u32]) -> Ret {
        ROUTER.into_ret()
    }

    fn fake_stack_alloc(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.u32(a[1]);
        e.mem.alloc(bytes).into_ret()
    }

    fn fake_collect(e: &mut Engine, a: &[u32]) -> Ret {
        // a = [collector, handle, second word]; the float to return sits at collector + 8.
        Ret {
            st0: e.mem.f32(a[0] + 8) as f64,
            ..Ret::default()
        }
    }

    /// Reports the size it is asked about as the allocated size.
    fn fake_allocated_size(_: &mut Engine, a: &[u32]) -> Ret {
        a[1].into_ret()
    }

    /// Stable sort of `a[1]` four-byte entries by their low halfword (`hkRadixSort::sort16`).
    fn fake_radix_sort(e: &mut Engine, a: &[u32]) -> Ret {
        let mut entries: Vec<u32> = (0..a[1]).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
        entries.sort_by_key(|entry| entry & 0xffff);
        for (i, entry) in entries.iter().enumerate() {
            e.mem.set_u32(a[0] + 4 * i as u32, *entry);
        }
        Ret::default()
    }

    /// The CRT `ST0` to 64-bit integer conversion (truncation).
    fn fake_float_to_int64(_: &mut Engine, a: &[u32]) -> Ret {
        let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
        (value as i64 as u64).into_ret()
    }

    /// An engine with the exe constants the sweep reads and fakes for everything it calls
    /// outside this file.
    fn setup() -> Engine {
        let mut e = Engine::new();
        put_f32s(&mut e, QUANTISE_MAX, &[65532.0; 4]);
        put_f32s(&mut e, QUANTISE_ADD, &[65536.0; 4]);
        put_f32s(&mut e, QUANTISE_MAX_32, &[2147352576.0; 4]);
        put_f32s(&mut e, QUANTISE_MIN_32, &[0.0; 4]);
        put_u32s(&mut e, SIGN_MASK, &[0x8000_0000; 4]);
        put_f32s(&mut e, ONES, &[1.0; 4]);
        put_f32s(&mut e, SCALE_FLOAT, &[65532.0, 1.0 / 65532.0]);
        put_f32s(&mut e, ZERO_FLOAT, &[0.0]);
        put_u32s(&mut e, NODE_FIELD_TABLE, &[8, 10, 0, 4, 2, 6]);
        put_f32s(&mut e, TEN_FLOAT, &[10.0]);
        put_f32s(&mut e, ELEVEN_FLOAT, &[11.0]);
        put_f32s(&mut e, UNIT_FLOAT, &[1.0]);
        put_f32s(&mut e, NEG_TWO_FLOAT, &[-2.0]);
        put_f32s(&mut e, TWO_FLOAT, &[2.0]);
        put_u32s(&mut e, RUNNING_MIN_START, &[0x7f7f_ffee]);
        put_u32s(&mut e, RUNNING_MAX_START, &[0xff7f_ffee]);
        e.map(HALF_DOUBLE, 8);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.map(ELEVEN_DOUBLE, 8);
        e.mem.set_f64(ELEVEN_DOUBLE, 11.0);
        e.map(TINY_DOUBLE, 8);
        e.mem.set_f64(TINY_DOUBLE, 1.1920928955078125e-07);
        let slots: [u32; 30] = [
            0x00d1_a2d0,
            0x00c9_0350,
            0x00d1_19e0,
            0x00d1_0ac0,
            0x00d1_14e0,
            0x00d1_42b0,
            0x00d1_8400,
            0x00d1_4600,
            0x00d1_93e0,
            0x00d1_1620,
            0x00d1_2040,
            0x00d1_2b80,
            0x00d1_5fc0,
            0x00d1_0ff0,
            0x00d1_1fa0,
            0x00d1_19c0,
            0x00d1_0e10,
            0x00d1_4ef0,
            0x00d1_4100,
            0x00d1_1470,
            0x00d1_75f0,
            0x00d1_0f00,
            0x00d1_a2a0,
            0x00d1_6300,
            0x00d1_1630,
            0x00d1_5f10,
            0x00d1_59a0,
            0x00d1_6c50,
            0x00d1_1020,
            0x00d1_54a0,
        ];
        e.put_vtable(VTABLE, &slots);
        // The router: bump pointer, stack allocator and heap allocator.
        e.map(ROUTER, 0x20);
        e.map(BUMP_REGION, 0x2_0000);
        e.mem.set_u32(ROUTER, BUMP_REGION);
        e.mem.set_u32(ROUTER + 0xc, STACK_ALLOCATOR);
        e.mem.set_u32(ROUTER + 0x10, HEAP_ALLOCATOR);
        e.put_vtable(
            ALLOCATOR_VTABLES,
            &[
                FAKE_HEAP_NOOP,
                FAKE_HEAP_NOOP,
                FAKE_HEAP_NOOP,
                FAKE_STACK_ALLOC,
                FAKE_STACK_FREE,
            ],
        );
        // Stack allocator: vtable with slots 0xc (alloc) and 0x10 (free); heap allocator: any.
        e.map(STACK_ALLOCATOR, 8);
        e.map(HEAP_ALLOCATOR, 8);
        e.mem.set_u32(STACK_ALLOCATOR, ALLOCATOR_VTABLES);
        e.mem.set_u32(HEAP_ALLOCATOR, ALLOCATOR_VTABLES);
        e.put_vtable(COLLECTOR_VTABLE, &[0, FAKE_COLLECT]);
        e.register(MEMORY_ROUTER, fake_router);
        e.register(ARRAY_GROW, fake_grow);
        e.register(ARRAY_RESERVE, fake_reserve);
        e.register(EASY_ALLOC, fake_easy_alloc);
        e.register(EASY_FREE, noop);
        e.register(BASE_CONSTRUCTOR, noop);
        e.register(BASE_DESTRUCTOR, noop);
        e.register(STATISTICS_ALLOCATED_SIZE, fake_allocated_size);
        e.register(STATISTICS_ADD_NORMAL_CHUNK, noop);
        e.register(RADIX_SORT_16, fake_radix_sort);
        e.register(FLOAT_TO_INT64, fake_float_to_int64);
        e.register(FAKE_STACK_ALLOC, fake_stack_alloc);
        e.register(FAKE_STACK_FREE, noop);
        e.register(FAKE_HEAP_NOOP, noop);
        e.register(FAKE_COLLECT, fake_collect);
        e.register(FAKE_STATISTICS, noop);
        e
    }

    fn vec_at(e: &mut Engine, v: Vec4) -> u32 {
        let at = e.mem.alloc(16);
        store_vec4(e, at, v);
        at
    }

    /// An empty `hkArray` that owns nothing.
    fn empty_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(12);
        e.mem.set_u32(array + 8, NOT_OWNED);
        array
    }

    /// An `hkArray` of 4-byte elements filled with `words` and room for 16 more.
    fn word_array(e: &mut Engine, words: &[u32]) -> u32 {
        let array = e.mem.alloc(12);
        let capacity = words.len() as u32 + 16;
        let data = e.mem.alloc(capacity * 4);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *w);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, words.len() as u32);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    fn words_of(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    /// The `(first, second)` pairs of a pair array.
    fn pairs_of(e: &Engine, array: u32) -> Vec<(u32, u32)> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| (e.mem.u32(data + 8 * i), e.mem.u32(data + 8 * i + 4)))
            .collect()
    }

    /// 16-bit endpoint: `(value, node index)`.
    fn endpoints_of(e: &Engine, this: Ptr<Hkp3AxisSweep>, axis: usize) -> Vec<(u16, u16)> {
        let array = this.addr() + AXIS_ARRAYS[axis];
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| (e.mem.u16(data + 4 * i), e.mem.u16(data + 4 * i + 2)))
            .collect()
    }

    fn node_at(e: &Engine, this: Ptr<Hkp3AxisSweep>, id: u32) -> u32 {
        e.mem.u32(this.addr() + 0xa0) + 16 * id
    }

    /// Every endpoint array is sorted and every node's six endpoint indices point back at it
    /// with the matching value parity.
    fn assert_consistent(e: &Engine, this: Ptr<Hkp3AxisSweep>) {
        for (axis, fields) in AXIS_NODE_FIELDS.iter().enumerate() {
            let list = endpoints_of(e, this, axis);
            assert!(
                list.windows(2).all(|w| w[0].0 <= w[1].0),
                "axis {axis} unsorted: {list:x?}"
            );
            for (index, (value, id)) in list.iter().enumerate() {
                let is_max = (value & 1) as usize;
                let at = node_at(e, this, *id as u32) + fields[is_max];
                if *id != 0 || index != 0 {
                    assert_eq!(e.mem.u16(at) as usize, index, "axis {axis} index {index}");
                }
            }
        }
    }

    /// `(min, max)` endpoint values of node `id` per axis.
    fn box_of(e: &Engine, this: Ptr<Hkp3AxisSweep>, id: u32) -> [(u16, u16); 3] {
        let node = node_at(e, this, id);
        [0usize, 1, 2].map(|axis| {
            let list = endpoints_of(e, this, axis);
            let [min_field, max_field] = AXIS_NODE_FIELDS[axis];
            (
                list[e.mem.u16(node + min_field) as usize].0,
                list[e.mem.u16(node + max_field) as usize].0,
            )
        })
    }

    /// A sweep over the world `-100..100` on each axis with four markers (three nodes) and
    /// the quantisation set from it.
    fn fresh() -> (Engine, Ptr<Hkp3AxisSweep>) {
        let mut e = setup();
        let this = e.new_object::<Hkp3AxisSweep>();
        let min = vec_at(&mut e, [-100.0, -100.0, -100.0, 0.0]);
        let max = vec_at(&mut e, [100.0, 100.0, 100.0, 0.0]);
        hkp_3_axis_sweep(&mut e, this, min, max, 4);
        let low = vec_at(&mut e, [100.0, 100.0, 100.0, 0.0]);
        let high = vec_at(&mut e, [100.0, 100.0, 100.0, 0.0]);
        let scale = vec_at(&mut e, [1.0e7, 1.0e7, 1.0e7, 0.0]);
        set_32_bit_offset_and_scale(&mut e, this, low, high, scale);
        (e, this)
    }

    /// The x, y, z `(min, max)` endpoint values of the four test objects.
    const BOXES: [[(u32, u32); 3]; 4] = [
        [(0x1000, 0x2001), (0x1000, 0x2001), (0x1000, 0x2001)],
        [(0x1800, 0x4801), (0x1800, 0x3001), (0x1800, 0x3001)],
        [(0x5000, 0x6001), (0x1000, 0x2001), (0x1000, 0x2001)],
        [(0x1400, 0x1c01), (0x7000, 0x7801), (0x1000, 0x2001)],
    ];

    /// The integer box `addObject` takes for the endpoint values `b`.
    fn int_box(e: &mut Engine, b: [(u32, u32); 3]) -> u32 {
        let at = e.mem.alloc(32);
        for (axis, (min, max)) in b.iter().enumerate() {
            e.mem.set_u32(at + 4 * axis as u32, min << 15);
            e.mem.set_u32(at + 0x10 + 4 * axis as u32, (max - 1) << 15);
        }
        at
    }

    /// Adds the object with endpoint values `b` through `addObject` (integer box); returns
    /// its handle (a word that receives the node index).
    fn add_box(e: &mut Engine, this: Ptr<Hkp3AxisSweep>, b: [(u32, u32); 3], pairs: u32) -> u32 {
        let handle = e.mem.alloc(8);
        let at = int_box(e, b);
        add_object_ov2(e, this, handle, at, pairs);
        handle
    }

    struct World {
        e: Engine,
        this: Ptr<Hkp3AxisSweep>,
        handles: [u32; 4],
        /// Pairs reported while adding.
        pairs: u32,
    }

    /// The sweep with the four test objects (node 0 sentinel, 1..=3 markers, 4..=7 objects).
    fn populated() -> World {
        let (mut e, this) = fresh();
        let pairs = empty_array(&mut e);
        let mut handles = [0; 4];
        for (i, b) in BOXES.iter().enumerate() {
            handles[i] = add_box(&mut e, this, *b, pairs);
        }
        World {
            e,
            this,
            handles,
            pairs,
        }
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= 1e-3 * (1.0 + b.abs())
    }

    /// An endpoint word: the value in the low halfword, the node index in the high one.
    fn ep(value: u32, id: u32) -> u32 {
        value | (id << 16)
    }

    /// A block of the size `getAabbCacheSize` asks for.
    fn cache_block(e: &mut Engine, this: Ptr<Hkp3AxisSweep>) -> u32 {
        let size = get_aabb_cache_size(e, this);
        e.mem.alloc(size as u32)
    }

    /// Nodes in memory: `[y_min, z_min, y_max, z_max, x_min, x_max]` per node, then the handle.
    fn put_nodes(e: &mut Engine, nodes: &[[u16; 6]]) -> u32 {
        let at = e.mem.alloc(16 * nodes.len() as u32 + 16);
        for (i, node) in nodes.iter().enumerate() {
            for (field, value) in node.iter().enumerate() {
                e.mem.set_u16(at + 16 * i as u32 + 2 * field as u32, *value);
            }
        }
        at
    }

    fn node_fields(e: &Engine, at: u32) -> [u16; 6] {
        [0, 1, 2, 3, 4, 5].map(|field| e.mem.u16(at + 2 * field))
    }

    #[test]
    fn get_type_is_zero() {
        let (mut e, this) = fresh();
        assert_eq!(get_type(&mut e, this), 0);
    }

    #[test]
    fn new_node_raises_the_indices_above_its_endpoints() {
        let mut e = setup();
        let new_node = put_nodes(&mut e, &[[2, 3, 5, 6, 4, 7]]);
        let nodes = put_nodes(&mut e, &[[1, 2, 6, 7, 3, 9], [4, 3, 4, 5, 4, 6]]);
        fn_00d10b30(&mut e, 0, nodes, 2, new_node);
        assert_eq!(node_fields(&e, nodes), [1, 2, 8, 9, 3, 11]);
        assert_eq!(node_fields(&e, nodes + 16), [6, 4, 6, 7, 5, 8]);
    }

    #[test]
    fn indices_above_a_limit_move_by_delta() {
        let mut e = setup();
        let nodes = put_nodes(&mut e, &[[1, 4, 3, 2, 4, 5]]);
        let limits = e.mem.alloc(16);
        e.mem.set_u16(limits, 5);
        e.mem.set_u16(limits + 4, 3);
        e.mem.set_u16(limits + 8, 4);
        fn_00d10c40(&mut e, nodes, 1, 10, limits);
        assert_eq!(node_fields(&e, nodes), [1, 14, 13, 2, 4, 15]);
    }

    #[test]
    fn removed_node_lowers_the_indices_above_its_endpoints() {
        let mut e = setup();
        let removed = put_nodes(&mut e, &[[2, 3, 5, 6, 4, 7]]);
        let nodes = put_nodes(&mut e, &[[1, 4, 8, 7, 5, 9]]);
        fn_00d10d10(&mut e, 0, nodes, 1, removed);
        assert_eq!(node_fields(&e, nodes), [1, 3, 6, 5, 4, 7]);
    }

    #[test]
    fn extents_are_the_negated_offset_and_the_far_corner() {
        let mut e = setup();
        let this = e.new_object::<Hkp3AxisSweep>();
        let me = this.addr();
        store_vec4(&mut e, me + OFFSET_LOW, [100.0, 50.0, -20.0, 0.0]);
        store_vec4(&mut e, me + SCALE, [2.0, 4.0, 8.0, 1.0]);
        let low = e.mem.alloc(16);
        let high = e.mem.alloc(16);
        get_extents(&mut e, this, low, high);
        let low = load_vec4(&e, low);
        let high = load_vec4(&e, high);
        assert_eq!(&low[..3], &[-100.0, -50.0, 20.0]);
        assert!(close(high[0], -100.0 + 65532.0 / 2.0));
        assert!(close(high[1], -50.0 + 65532.0 / 4.0));
        assert!(close(high[2], 20.0 + 65532.0 / 8.0));
    }

    #[test]
    fn axis_find_returns_the_first_endpoint_not_below() {
        let mut e = setup();
        let words: Vec<u32> = (0..100u32).map(|i| 2 * i).collect();
        let array = word_array(&mut e, &words);
        let data = e.mem.u32(array);
        let end = data + 99 * 4;
        assert_eq!(hkp_bp_axis_find(&mut e, 0, data, end, 101), data + 51 * 4);
        assert_eq!(hkp_bp_axis_find(&mut e, 0, data, end, 100), data + 50 * 4);
        assert_eq!(hkp_bp_axis_find(&mut e, 0, data, end, 0), data);
        let short = word_array(&mut e, &[10, 20, 30, 0xffff]);
        let data = e.mem.u32(short);
        assert_eq!(hkp_bp_axis_find(&mut e, 0, data, data + 12, 25), data + 8);
        assert_eq!(hkp_bp_axis_find(&mut e, 0, data, data + 12, 20), data + 4);
    }

    #[test]
    fn shift_broad_phase_applies_what_shift_all_objects_reports() {
        let mut e = setup();
        let this = e.new_object::<Hkp3AxisSweep>();
        let me = this.addr();
        e.mem.set_u32(me, VTABLE);
        e.register(0x00d1_75f0, |e, a| {
            // a = [this, negated shift, applied (out), third]: keep the shift, report 3, 4, 5.
            let seen = load_vec4(e, a[1]);
            e.map(0x0030_5000, 0x20);
            store_vec4(e, 0x0030_5000, seen);
            store_vec4(e, a[2], [3.0, 4.0, 5.0, 0.0]);
            e.mem.set_u32(0x0030_5010, a[3]);
            Ret::default()
        });
        store_vec4(&mut e, me + OFFSET_LOW, [10.0, 20.0, 30.0, 0.0]);
        store_vec4(&mut e, me + SCALE, [2.0, 2.0, 2.0, 1.0]);
        store_vec4(&mut e, me + OFFSET_LOW_32, [1.0, 2.0, 3.0, 0.0]);
        store_vec4(&mut e, me + SCALE_32, [4.0, 4.0, 4.0, 1.0]);
        let amount = vec_at(&mut e, [1.0, -2.0, 0.5, 0.0]);
        let result = e.mem.alloc(16);
        shift_broad_phase(&mut e, this, amount, result, 0x1234);
        let seen = load_vec4(&e, 0x0030_5000);
        assert_eq!(&seen[..3], &[-1.0, 2.0, -0.5]);
        assert_eq!(e.mem.u32(0x0030_5010), 0x1234);
        assert_eq!(&load_vec4(&e, me + OFFSET_LOW)[..3], &[13.0, 24.0, 35.0]);
        let high = load_vec4(&e, me + OFFSET_HIGH);
        assert!(close(high[0], 13.5) && close(high[1], 24.5) && close(high[2], 35.5));
        assert_eq!(&load_vec4(&e, me + OFFSET_LOW_32)[..3], &[4.0, 6.0, 8.0]);
        let high_32 = load_vec4(&e, me + OFFSET_HIGH_32);
        assert!(close(high_32[0], 4.25) && close(high_32[1], 6.25) && close(high_32[2], 8.25));
        assert_eq!(&load_vec4(&e, result)[..3], &[-3.0, -4.0, -5.0]);
    }

    #[test]
    fn check_determinism_changes_nothing() {
        let (mut e, this) = fresh();
        let before = e.mem.bytes(this.addr(), 0xe0);
        check_determinism(&mut e, this);
        assert_eq!(e.mem.bytes(this.addr(), 0xe0), before);
    }

    #[test]
    fn offset_and_scale_come_from_the_world_box() {
        let mut e = setup();
        let this = e.new_object::<Hkp3AxisSweep>();
        let me = this.addr();
        store_vec4(&mut e, me + WORLD_MIN, [-100.0, -50.0, 0.0, 0.0]);
        store_vec4(&mut e, me + WORLD_MAX, [100.0, 50.0, 8.0, 0.0]);
        let low = vec_at(&mut e, [100.0, 50.0, 0.0, 5.0]);
        let high = vec_at(&mut e, [101.0, 51.0, 1.0, 6.0]);
        let scale = vec_at(&mut e, [7.0, 8.0, 9.0, 3.0]);
        set_32_bit_offset_and_scale(&mut e, this, low, high, scale);
        assert_eq!(load_vec4(&e, me + OFFSET_LOW_32), [100.0, 50.0, 0.0, 5.0]);
        assert_eq!(load_vec4(&e, me + OFFSET_HIGH_32), [101.0, 51.0, 1.0, 6.0]);
        assert_eq!(load_vec4(&e, me + SCALE_32), [7.0, 8.0, 9.0, 3.0]);
        let low_now = load_vec4(&e, me + OFFSET_LOW);
        assert_eq!(low_now, [100.0, 50.0, 0.0, 0.0]);
        let high_now = load_vec4(&e, me + OFFSET_HIGH);
        assert!(close(high_now[0], 100.0 + 200.0 / 65532.0));
        assert!(close(high_now[1], 50.0 + 100.0 / 65532.0));
        assert!(close(high_now[2], 8.0 / 65532.0));
        assert_eq!(high_now[3], 0.0);
        let scale_now = load_vec4(&e, me + SCALE);
        assert!(close(scale_now[0], 65532.0 / 200.0));
        assert!(close(scale_now[1], 65532.0 / 100.0));
        assert!(close(scale_now[2], 65532.0 / 8.0));
        assert_eq!(scale_now[3], 0.0);
    }

    #[test]
    fn x_interval_bits_are_the_nodes_that_overlap_in_x() {
        let mut w = populated();
        let e = &mut w.e;
        let count = e.get(w.this, Hkp3AxisSweep::node_count);
        assert_eq!(count, 8);
        let bits = e.mem.alloc(64);
        e.mem.set_u32(bits, 0xffff_ffff);
        // Object 4 spans 0x1000..0x2001: objects 5 (0x1800) and 7 (0x1400) start inside it.
        let a = Ptr::new(node_at(e, w.this, 4));
        set_bits_based_on_x_interval(e, w.this, count, 0x1000, a, 4, bits);
        assert_eq!(e.mem.u32(bits), (1 << 5) | (1 << 7));
        // Object 6 starts after marker 1: object 5 is in its list but ends before.
        let c = Ptr::new(node_at(e, w.this, 6));
        set_bits_based_on_x_interval(e, w.this, count, 0x5000, c, 6, bits);
        assert_eq!(e.mem.u32(bits), 0);
    }

    #[test]
    fn aabbs_overlap_when_all_three_axes_do() {
        let mut w = populated();
        let e = &mut w.e;
        let ids = e.mem.alloc(16);
        let overlapping = |e: &mut Engine, this, first: u32, second: u32| {
            e.mem.set_u32(ids, first);
            e.mem.set_u32(ids + 4, second);
            are_aabbs_overlapping(e, this, ids, ids + 4)
        };
        let this = w.this;
        assert!(overlapping(e, this, 4, 5));
        assert!(overlapping(e, this, 5, 4));
        assert!(!overlapping(e, this, 4, 6));
        assert!(!overlapping(e, this, 4, 7));
        assert!(!overlapping(e, this, 5, 7));
    }

    #[test]
    fn add_object_hands_the_integer_box_to_the_overload() {
        let (mut e, this) = fresh();
        let me = this.addr();
        store_vec4(&mut e, me + OFFSET_LOW_32, [100.0, 100.0, 100.0, 0.0]);
        store_vec4(&mut e, me + OFFSET_HIGH_32, [100.0, 100.0, 100.0, 0.0]);
        store_vec4(&mut e, me + SCALE_32, [1.5, 1.5, 1.5, 1.5]);
        e.register(0x00d1_42b0, |e, a| {
            e.map(0x0030_5000, 0x40);
            let bytes = e.mem.bytes(a[2], 32);
            e.mem.write(0x0030_5000, &bytes);
            Ret::default()
        });
        let aabb = e.mem.alloc(32);
        store_vec4(&mut e, aabb, [1.0, 0.0, -200.0, 0.0]);
        store_vec4(&mut e, aabb + 0x10, [1.0e30, 1.0, 0.5, 0.0]);
        e.call_log = Some(vec![]);
        add_object(&mut e, this, 0xaaa0, aabb, 0xbbb0);
        let log: Log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == 0x00d1_42b0).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], me);
        assert_eq!(calls[0].1[1], 0xaaa0);
        assert_eq!(calls[0].1[3], 0xbbb0);
        let ints: Vec<i32> = (0..8).map(|i| e.mem.i32(0x0030_5000 + 4 * i)).collect();
        assert_eq!(ints, [152, 150, 0, 0, 2147352576, 152, 151, 0]);
    }

    #[test]
    fn object_count_leaves_out_the_sentinel() {
        let w = populated();
        let mut e = w.e;
        assert_eq!(get_num_objects(&mut e, w.this), 7);
    }

    #[test]
    fn cache_size_is_24_bytes_per_object_node_and_a_header() {
        let w = populated();
        let mut e = w.e;
        assert_eq!(get_aabb_cache_size(&mut e, w.this), (8 - 3) * 24 + 0x24);
    }

    #[test]
    fn aabb_of_a_node_is_its_endpoints_in_floats() {
        let mut w = populated();
        let e = &mut w.e;
        let out = e.mem.alloc(32);
        let node = Ptr::new(node_at(e, w.this, 4));
        get_aabb_from_node(e, w.this, node, out);
        let low = load_vec4(e, out);
        let high = load_vec4(e, out + 0x10);
        let scale = 65532.0 / 200.0;
        for axis in 0..3 {
            assert!(close(low[axis], 0x1000 as f32 / scale - 100.0), "{low:?}");
            assert!(close(high[axis], 0x2001 as f32 / scale - 100.0), "{high:?}");
        }
        assert!(close(low[3], 0.0) && close(high[3], 0.0));
    }

    #[test]
    fn node_that_got_a_smaller_index_moves_below_equal_valued_endpoints() {
        let mut e = setup();
        let this = e.new_object::<Hkp3AxisSweep>();
        let me = this.addr();
        // Node 3 has the same x minimum as node 9 and now sorts after it.
        let nodes = put_nodes(
            &mut e,
            &[
                [0; 6],
                [0; 6],
                [0; 6],
                [1, 1, 2, 2, 2, 3],
                [0; 6],
                [0; 6],
                [0; 6],
                [0; 6],
                [0; 6],
                [1, 1, 2, 2, 1, 4],
            ],
        );
        e.mem.set_u32(me + 0xa0, nodes);
        e.mem.set_u32(me + 0xa4, 10);
        let x = word_array(&mut e, &[0, ep(10, 9), ep(10, 3), ep(11, 3), ep(12, 9)]);
        let y = word_array(&mut e, &[0, ep(5, 3), ep(6, 3)]);
        let z = word_array(&mut e, &[0, ep(5, 3), ep(6, 3)]);
        for (array, source) in [(0xac, x), (0xb8, y), (0xc4, z)] {
            for word in 0..3 {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(me + array + 4 * word, value);
            }
        }
        fix_deterministic_order_after_node_id_was_decreased(&mut e, this, 3);
        assert_eq!(
            endpoints_of(&e, this, 0)[..4],
            [(0, 0), (10, 3), (10, 9), (11, 3)]
        );
        assert_eq!(e.mem.u16(node_at(&e, this, 3) + 8), 1);
        assert_eq!(e.mem.u16(node_at(&e, this, 9) + 8), 2);
    }

    #[test]
    fn two_elements_are_removed_from_an_array() {
        let mut e = setup();
        let words: Vec<u32> = (0..10).collect();
        let array = word_array(&mut e, &words);
        fn_00d11960(&mut e, array, 2, 5);
        assert_eq!(words_of(&e, array), [0, 1, 3, 4, 6, 7, 8, 9]);
    }

    #[test]
    fn aabb_by_index_reads_the_node_the_index_names() {
        let mut w = populated();
        let e = &mut w.e;
        let index = e.mem.alloc(4);
        e.mem.set_u32(index, 5);
        let out = e.mem.alloc(32);
        get_aabb(e, w.this, index, out);
        let direct = e.mem.alloc(32);
        let node = Ptr::new(node_at(e, w.this, 5));
        get_aabb_from_node(e, w.this, node, direct);
        assert_eq!(e.mem.bytes(out, 32), e.mem.bytes(direct, 32));
        let low = load_vec4(e, out);
        assert!(close(low[0], 0x1800 as f32 / (65532.0 / 200.0) - 100.0));
    }

    #[test]
    fn statistics_report_each_owned_array() {
        let mut w = populated();
        let e = &mut w.e;
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, COLLECTOR_VTABLE + 0x100);
        e.put_vtable(COLLECTOR_VTABLE + 0x100, &[0, 0, 0, 0, 0, FAKE_STATISTICS]);
        let nodes = e.get(w.this, Hkp3AxisSweep::nodes);
        let capacity = e.get(w.this, Hkp3AxisSweep::node_capacity);
        e.call_log = Some(vec![]);
        calc_content_statistics(e, w.this, collector, 0);
        let log: Log = e.call_log.take().unwrap();
        let reports: Vec<_> = log.iter().filter(|c| c.0 == FAKE_STATISTICS).collect();
        // The node array and the three endpoint arrays; the marker overlap arrays of markers
        // 2 and 3 own nothing, marker 1 holds one entry.
        assert_eq!(reports.len(), 5);
        assert_eq!(
            reports[0].1,
            args![
                collector,
                3u32,
                NODES_NAME,
                nodes,
                8u32 * 16,
                (capacity & CAPACITY_MASK) << 4
            ]
            .to_vec()
        );
        assert_eq!(reports[1].1[2], AXIS_NAME);
        let chunks: Vec<_> = log
            .iter()
            .filter(|c| c.0 == STATISTICS_ADD_NORMAL_CHUNK)
            .collect();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].1[1], MARKERS_NAME);
        assert_eq!(chunks[0].1[3], 3 * 16);
    }

    #[test]
    fn pair_of_handles_is_appended() {
        let mut e = setup();
        let pairs = empty_array(&mut e);
        let nodes = put_nodes(&mut e, &[[0; 6], [0; 6]]);
        e.mem.set_u32(nodes + 0xc, 0x111);
        e.mem.set_u32(nodes + 0x1c, 0x222);
        fn_00d11bc0(&mut e, nodes, nodes + 16, pairs);
        fn_00d11bc0(&mut e, nodes + 16, nodes, pairs);
        assert_eq!(pairs_of(&e, pairs), [(0x111, 0x222), (0x222, 0x111)]);
    }

    #[test]
    fn identical_twin_appends_the_same_pair() {
        let mut e = setup();
        let pairs = empty_array(&mut e);
        let nodes = put_nodes(&mut e, &[[0; 6], [0; 6]]);
        e.mem.set_u32(nodes + 0xc, 0x111);
        e.mem.set_u32(nodes + 0x1c, 0x222);
        fn_00d11c10(&mut e, nodes, nodes + 16, pairs);
        fn_00d11c10(&mut e, nodes + 16, nodes, pairs);
        assert_eq!(pairs_of(&e, pairs), [(0x111, 0x222), (0x222, 0x111)]);
    }

    #[test]
    fn new_overlap_is_a_pair_or_a_marker_entry() {
        let mut e = setup();
        let pairs = empty_array(&mut e);
        let markers = e.mem.alloc(0x40);
        let nodes = put_nodes(&mut e, &[[0; 6], [0; 6]]);
        e.mem.set_u32(nodes + 0xc, 0x111);
        e.mem.set_u32(nodes + 0x1c, 0x222);
        fn_00d11c60(&mut e, markers, nodes, 9, nodes + 16, pairs);
        assert_eq!(pairs_of(&e, pairs), [(0x111, 0x222)]);
        // The second node is the marker at offset 0x10.
        e.mem.set_u32(nodes + 0x1c, 0x11);
        fn_00d11c60(&mut e, markers, nodes, 9, nodes + 16, pairs);
        assert_eq!(pairs_of(&e, pairs).len(), 1);
        assert_eq!(words_of(&e, markers + 0x14), [9]);
        fn_00d11c60(&mut e, markers, nodes, 11, nodes + 16, pairs);
        assert_eq!(e.mem.u32(markers + 0x18), 2);
        let data = e.mem.u32(markers + 0x14);
        assert_eq!((e.mem.u16(data), e.mem.u16(data + 2)), (9, 11));
    }

    #[test]
    fn ended_overlap_is_a_pair_or_leaves_the_marker() {
        let mut e = setup();
        let pairs = empty_array(&mut e);
        let markers = e.mem.alloc(0x40);
        let nodes = put_nodes(&mut e, &[[0; 6], [0; 6]]);
        e.mem.set_u32(nodes + 0xc, 0x111);
        e.mem.set_u32(nodes + 0x1c, 0x222);
        fn_00d11d00(&mut e, markers, nodes, 9, nodes + 16, pairs);
        assert_eq!(pairs_of(&e, pairs), [(0x111, 0x222)]);
        e.mem.set_u32(nodes + 0x1c, 0x11);
        // The marker at offset 0x10 lists the ids 7, 9 and 12.
        let list = e.mem.alloc(8);
        for (i, id) in [7u16, 9, 12].iter().enumerate() {
            e.mem.set_u16(list + 2 * i as u32, *id);
        }
        e.mem.set_u32(markers + 0x14, list);
        e.mem.set_u32(markers + 0x18, 3);
        e.mem.set_u32(markers + 0x1c, 4);
        fn_00d11d00(&mut e, markers, nodes, 9, nodes + 16, pairs);
        assert_eq!(pairs_of(&e, pairs).len(), 1);
        assert_eq!(e.mem.u32(markers + 0x18), 2);
        assert_eq!((e.mem.u16(list), e.mem.u16(list + 2)), (7, 12));
    }

    #[test]
    fn axis_insert_puts_the_maximum_then_the_minimum_in_order() {
        let mut e = setup();
        let array = word_array(&mut e, &[0, ep(0x20, 1), ep(0x40, 2), 0xfffd]);
        let out = e.mem.alloc(8);
        hkp_bp_axis_insert(&mut e, array, 0, 9, 0x30, 0x50, out, out + 4);
        assert_eq!(
            words_of(&e, array),
            [
                0,
                ep(0x20, 1),
                ep(0x30, 9),
                ep(0x40, 2),
                ep(0x50, 9),
                0xfffd
            ]
        );
        assert_eq!(e.mem.u16(out), 2);
        assert_eq!(e.mem.u16(out + 4), 4);
    }

    #[test]
    fn axis_insert_tail_makes_room_and_repoints_the_moved_endpoints() {
        let mut e = setup();
        let nodes = put_nodes(&mut e, &[[0; 6]; 4]);
        let array = word_array(&mut e, &[0, ep(0x10, 1), ep(0x20, 2), ep(0x30, 3), 0xfffd]);
        let new = word_array(&mut e, &[ep(0x15, 7), ep(0x25, 7)]);
        let new_data = e.mem.u32(new);
        let end = hkp_bp_axis_insert_tail(&mut e, array, nodes, 0, new_data, 2);
        assert_eq!(end, 5);
        let words = words_of(&e, array);
        assert_eq!(words.len(), 7);
        assert_eq!(words[5], ep(0x30, 3));
        assert_eq!(words[6], 0xfffd);
        // Node 3's x minimum and the sentinel's x maximum follow the endpoints.
        assert_eq!(e.mem.u16(nodes + 3 * 16 + 8), 5);
        assert_eq!(e.mem.u16(nodes + 10), 6);
    }

    #[test]
    fn every_non_marker_node_gets_a_box() {
        let mut w = populated();
        let e = &mut w.e;
        let out = empty_array(e);
        get_all_aabbs(e, w.this, out);
        assert_eq!(e.mem.u32(out + 4), 5);
        assert_eq!(e.mem.u32(out + 8) & CAPACITY_MASK, 5);
        let data = e.mem.u32(out);
        let scale = 65532.0 / 200.0;
        // Nodes 4..7 come after the sentinel's box, in node order.
        for (object, b) in BOXES.iter().enumerate() {
            let low = load_vec4(e, data + 0x20 * (object as u32 + 1));
            let high = load_vec4(e, data + 0x20 * (object as u32 + 1) + 0x10);
            for axis in 0..3 {
                assert!(close(low[axis], b[axis].0 as f32 / scale - 100.0));
                assert!(close(high[axis], b[axis].1 as f32 / scale - 100.0));
            }
        }
    }

    /// Makes the quantisation the identity so that float boxes are endpoint values.
    fn identity_quantisation(e: &mut Engine, this: Ptr<Hkp3AxisSweep>) {
        let me = this.addr();
        store_vec4(e, me + SCALE, [1.0; 4]);
        store_vec4(e, me + OFFSET_LOW, [0.0; 4]);
        store_vec4(e, me + OFFSET_HIGH, [0.0; 4]);
    }

    #[test]
    fn float_box_update_moves_the_endpoints_and_reports_overlaps() {
        let mut w = populated();
        let e = &mut w.e;
        identity_quantisation(e, w.this);
        let handles = e.mem.alloc(4);
        e.mem.set_u32(handles, w.handles[0]);
        let boxes = e.mem.alloc(32);
        store_vec4(e, boxes, [4096.0, 28672.0, 4096.0, 0.0]);
        store_vec4(e, boxes + 0x10, [8192.0, 29696.0, 8192.0, 0.0]);
        let entered = empty_array(e);
        let exited = empty_array(e);
        update_aabbs(e, w.this, handles, boxes, 1, entered, exited);
        assert_consistent(e, w.this);
        assert_eq!(
            box_of(e, w.this, 4),
            [(0x1000, 0x2001), (0x7000, 0x7401), (0x1000, 0x2001)]
        );
        // The moved object now overlaps the one at y 0x7000 and no longer the one at 0x1800.
        assert_eq!(pairs_of(e, entered), [(w.handles[0], w.handles[3])]);
        assert_eq!(pairs_of(e, exited), [(w.handles[0], w.handles[1])]);
    }

    #[test]
    fn integer_box_update_moves_the_endpoints_and_reports_overlaps() {
        let mut w = populated();
        let e = &mut w.e;
        let handles = e.mem.alloc(4);
        e.mem.set_u32(handles, w.handles[0]);
        let boxes = int_box(e, [(0x1000, 0x2001), (0x7000, 0x7401), (0x1000, 0x2001)]);
        let entered = empty_array(e);
        let exited = empty_array(e);
        update_aabbs_uint32(e, w.this, handles, boxes, 1, entered, exited);
        assert_consistent(e, w.this);
        assert_eq!(
            box_of(e, w.this, 4),
            [(0x1000, 0x2001), (0x7000, 0x7401), (0x1000, 0x2001)]
        );
        assert_eq!(pairs_of(e, entered), [(w.handles[0], w.handles[3])]);
        assert_eq!(pairs_of(e, exited), [(w.handles[0], w.handles[1])]);
    }

    /// A record of the lists `collide1Axis` sweeps.
    fn record(e: &mut Engine, at: u32, low: u32, high: u32, min_x: u16, max_x: u16, handle: u32) {
        e.mem.set_u32(at, low);
        e.mem.set_u32(at + 4, high);
        e.mem.set_u16(at + 8, min_x);
        e.mem.set_u16(at + 10, max_x);
        e.mem.set_u32(at + 12, handle);
    }

    /// Records whose y and z overlap, each ending with a record that starts at 0xffff.
    fn records(e: &mut Engine, spans: &[(u16, u16, u32)]) -> u32 {
        let list = e.mem.alloc(16 * spans.len() as u32 + 16);
        for (i, (min_x, max_x, handle)) in spans.iter().enumerate() {
            record(
                e,
                list + 16 * i as u32,
                0x0001_0001,
                0x0005_0005,
                *min_x,
                *max_x,
                *handle,
            );
        }
        record(e, list + 16 * spans.len() as u32, 0, 0, 0xffff, 0xffff, 0);
        list
    }

    /// A handle that points at the node index `id`.
    fn handle_of(e: &mut Engine, id: u32) -> u32 {
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, id);
        handle
    }

    #[test]
    fn two_lists_are_swept_against_each_other() {
        let (mut e, this) = fresh();
        let markers = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0xd8, markers);
        let (h1, h2, h3) = (
            handle_of(&mut e, 4),
            handle_of(&mut e, 5),
            handle_of(&mut e, 6),
        );
        let a = records(&mut e, &[(10, 30, h1)]);
        let b = records(&mut e, &[(20, 40, h2), (50, 60, h3)]);
        let pairs = empty_array(&mut e);
        collide_1_axis(&mut e, this, a, 1, b, 2, 0, pairs);
        assert_eq!(pairs_of(&e, pairs), [(h1, h2)]);
        // A marker in the second list takes the first list's node index (mode 0), mode 1
        // leaves it alone and any other mode removes it again.
        e.mem.set_u32(b + 12, 0x11);
        let pairs = empty_array(&mut e);
        collide_1_axis(&mut e, this, a, 1, b, 2, 1, pairs);
        assert_eq!(e.mem.u32(markers + 0x18), 0);
        collide_1_axis(&mut e, this, a, 1, b, 2, 0, pairs);
        assert_eq!(e.mem.u32(markers + 0x18), 1);
        assert_eq!(e.mem.u16(e.mem.u32(markers + 0x14)), 4);
        collide_1_axis(&mut e, this, a, 1, b, 2, 2, pairs);
        assert_eq!(e.mem.u32(markers + 0x18), 0);
        assert!(pairs_of(&e, pairs).is_empty());
    }

    #[test]
    fn one_list_is_swept_against_itself() {
        let mut e = setup();
        let (h1, h2, h3) = (
            handle_of(&mut e, 4),
            handle_of(&mut e, 5),
            handle_of(&mut e, 6),
        );
        let list = records(&mut e, &[(10, 30, h1), (20, 25, h2), (35, 40, h3)]);
        let pairs = empty_array(&mut e);
        collide_1_axis_one_list(&mut e, 0, list, 3, pairs);
        assert_eq!(pairs_of(&e, pairs), [(h1, h2)]);
        // A marker is skipped, and a record that misses on y adds nothing.
        e.mem.set_u32(list + 16 + 12, 0x11);
        let pairs = empty_array(&mut e);
        collide_1_axis_one_list(&mut e, 0, list, 3, pairs);
        assert!(pairs_of(&e, pairs).is_empty());
        e.mem.set_u32(list + 16 + 12, h2);
        e.mem.set_u32(list + 16, 0x0007_0001);
        e.mem.set_u32(list + 16 + 4, 0x0009_0005);
        let pairs = empty_array(&mut e);
        collide_1_axis_one_list(&mut e, 0, list, 3, pairs);
        assert!(pairs_of(&e, pairs).is_empty());
    }

    /// The `(value, node index)` words of the cache array `index` (0 x, 1 y, 2 z).
    fn cache_words(e: &Engine, cache: u32, index: u32) -> Vec<u32> {
        words_of(e, cache + 0xc * index)
    }

    /// The cache of the nodes `a` and `b` (4 and 5 in the test sweep), per axis.
    fn expected_cache() -> [Vec<u32>; 3] {
        let x = vec![
            0,
            ep(0x1000, 4),
            ep(0x1800, 5),
            ep(0x2001, 4),
            ep(0x4801, 5),
            0xfffc,
        ];
        let y = vec![
            0,
            ep(0x1000, 4),
            ep(0x1800, 5),
            ep(0x2001, 4),
            ep(0x3001, 5),
            0xfffc,
        ];
        [x, y.clone(), y]
    }

    #[test]
    fn cache_holds_the_sorted_endpoints_of_the_given_nodes() {
        let mut w = populated();
        let e = &mut w.e;
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, node_at(e, w.this, 4));
        e.mem.set_u32(list + 4, node_at(e, w.this, 5));
        let nodes = e.mem.alloc(12);
        e.mem.set_u32(nodes, list);
        e.mem.set_u32(nodes + 4, 2);
        e.mem.set_u32(nodes + 8, 2);
        let cache = cache_block(e, w.this);
        calc_aabb_cache_internal(e, w.this, nodes, cache);
        let expected = expected_cache();
        for (axis, words) in expected.iter().enumerate() {
            assert_eq!(cache_words(e, cache, axis as u32), *words, "axis {axis}");
        }
        assert_eq!(e.mem.u32(cache + 8), 6 | NOT_OWNED);
        assert_eq!(e.mem.u32(cache), cache + 0x24);
        assert_eq!(e.mem.u32(cache + 0xc), cache + 0x24 + 6 * 4);
    }

    #[test]
    fn constructor_builds_the_sentinels_and_markers() {
        let mut e = setup();
        let this = e.new_object::<Hkp3AxisSweep>();
        let min = vec_at(&mut e, [-100.0, -50.0, -25.0, 0.0]);
        let max = vec_at(&mut e, [100.0, 50.0, 25.0, 0.0]);
        e.call_log = Some(vec![]);
        let result = hkp_3_axis_sweep(&mut e, this, min, max, 4);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(result, this);
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == BASE_CONSTRUCTOR)
                .collect::<Vec<_>>(),
            vec![&(BASE_CONSTRUCTOR, args![this.addr()].to_vec())]
        );
        let me = this.addr();
        assert_eq!(e.mem.u32(me), VTABLE);
        assert_eq!(load_vec4(&e, me + WORLD_MIN), [-100.0, -50.0, -25.0, 0.0]);
        assert_eq!(load_vec4(&e, me + WORLD_MAX), [100.0, 50.0, 25.0, 0.0]);
        assert_eq!(e.get(this, Hkp3AxisSweep::marker_bits), 2);
        assert_eq!(e.get(this, Hkp3AxisSweep::marker_count), 3);
        assert_eq!(e.get(this, Hkp3AxisSweep::node_count), 4);
        assert_eq!(
            endpoints_of(&e, this, 0),
            [
                (0, 0),
                (0x4000, 1),
                (0x4001, 1),
                (0x8000, 2),
                (0x8001, 2),
                (0xc000, 3),
                (0xc001, 3),
                (0xfffd, 0)
            ]
        );
        assert_eq!(endpoints_of(&e, this, 1), [(0, 0), (0xfffd, 0)]);
        assert_eq!(endpoints_of(&e, this, 2), [(0, 0), (0xfffd, 0)]);
        // The sentinel's maxima are the end sentinels; marker k sits on x endpoints 2k - 1, 2k.
        assert_eq!(node_fields(&e, node_at(&e, this, 0)), [0, 0, 1, 1, 0, 7]);
        for k in 1..4u16 {
            let node = node_at(&e, this, k as u32);
            assert_eq!(node_fields(&e, node), [0, 0, 1, 1, 2 * k - 1, 2 * k]);
            assert_eq!(e.mem.u32(node + 0xc), (0x10 * (k as u32 - 1)) | 1);
        }
        let offset = e.get(this, Hkp3AxisSweep::exponent_offset);
        // Quantising to the nearest 1 / 128 puts the threshold 1 / 256 below 11.
        assert!((offset + 1.0 / 256.0).abs() < 1e-5, "{offset}");
        assert_eq!(load_vec4(&e, me + SCALE), [0.0; 4]);
        assert_eq!(load_vec4(&e, me + OFFSET_LOW), [0.0; 4]);
    }

    #[test]
    fn requery_reports_the_overlapping_objects_once_each() {
        let mut w = populated();
        let e = &mut w.e;
        let pairs = empty_array(e);
        re_query_single_object(e, w.this, w.handles[0], pairs);
        assert_eq!(pairs_of(e, pairs), [(w.handles[0], w.handles[1])]);
        let pairs = empty_array(e);
        re_query_single_object(e, w.this, w.handles[3], pairs);
        assert!(pairs_of(e, pairs).is_empty());
    }

    #[test]
    fn adding_an_object_orders_the_endpoints_and_reports_overlaps() {
        let w = populated();
        let mut e = w.e;
        assert_eq!(e.get(w.this, Hkp3AxisSweep::node_count), 8);
        assert_consistent(&e, w.this);
        for (i, b) in BOXES.iter().enumerate() {
            let want = [0, 1, 2].map(|axis| (b[axis].0 as u16, b[axis].1 as u16));
            assert_eq!(box_of(&e, w.this, 4 + i as u32), want);
            assert_eq!(e.mem.u32(w.handles[i]), 4 + i as u32);
        }
        // Only the second object overlaps an earlier one (the first); the second also spans
        // marker 1's position, so that marker lists it.
        assert_eq!(pairs_of(&e, w.pairs), [(w.handles[1], w.handles[0])]);
        let marker = e.get(w.this, Hkp3AxisSweep::markers);
        assert_eq!(e.mem.i32(marker + 8), 1);
        assert_eq!(e.mem.u16(e.mem.u32(marker + 4)), 5);
        assert_eq!(e.mem.i32(marker + 0x18), 0);
        let again = add_box(&mut e, w.this, BOXES[2], w.pairs);
        assert_eq!(e.mem.u32(again), 8);
    }

    #[test]
    fn removing_an_object_moves_the_last_node_into_its_place() {
        let mut w = populated();
        let e = &mut w.e;
        let pairs = empty_array(e);
        remove_object(e, w.this, w.handles[1], pairs);
        assert_eq!(e.get(w.this, Hkp3AxisSweep::node_count), 7);
        assert_consistent(e, w.this);
        assert_eq!(pairs_of(e, pairs), [(w.handles[1], w.handles[0])]);
        // The last object (7) now has index 5 and marker 1 no longer lists the removed one.
        assert_eq!(e.mem.u32(w.handles[3]), 5);
        assert_eq!(
            box_of(e, w.this, 5),
            [(0x1400, 0x1c01), (0x7000, 0x7801), (0x1000, 0x2001)]
        );
        assert_eq!(
            box_of(e, w.this, 4),
            [(0x1000, 0x2001), (0x1000, 0x2001), (0x1000, 0x2001)]
        );
        let marker = e.get(w.this, Hkp3AxisSweep::markers);
        assert_eq!(e.mem.i32(marker + 8), 0);
        assert_eq!(e.mem.u32(w.handles[2]), 6);
        // Removing the last node needs no move.
        remove_object(e, w.this, w.handles[2], pairs);
        assert_eq!(e.get(w.this, Hkp3AxisSweep::node_count), 6);
        assert_consistent(e, w.this);
    }

    /// A float box (two vectors) for the identity quantisation.
    fn float_box(e: &mut Engine, low: [f32; 3], high: [f32; 3]) -> u32 {
        let at = e.mem.alloc(32);
        store_vec4(e, at, [low[0], low[1], low[2], 0.0]);
        store_vec4(e, at + 0x10, [high[0], high[1], high[2], 0.0]);
        at
    }

    #[test]
    fn single_box_query_reports_the_objects_it_overlaps() {
        let mut w = populated();
        let e = &mut w.e;
        identity_quantisation(e, w.this);
        let pairs = empty_array(e);
        let query = float_box(e, [6400.0; 3], [6656.0; 3]);
        query_single_aabb(e, w.this, query, pairs);
        assert_eq!(pairs_of(e, pairs), [(0, w.handles[0]), (0, w.handles[1])]);
        let pairs = empty_array(e);
        let query = float_box(e, [4500.0, 4500.0, 4500.0], [4600.0, 4600.0, 4600.0]);
        query_single_aabb(e, w.this, query, pairs);
        assert_eq!(pairs_of(e, pairs), [(0, w.handles[0])]);
    }

    #[test]
    fn single_box_query_with_collector_calls_it_per_object() {
        let mut w = populated();
        let e = &mut w.e;
        identity_quantisation(e, w.this);
        let collector = e.mem.alloc(16);
        e.mem.set_u32(collector, COLLECTOR_VTABLE);
        let query = float_box(e, [6400.0; 3], [6656.0; 3]);
        e.call_log = Some(vec![]);
        query_single_aabb_with_collector(e, w.this, query, collector);
        let log: Log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 == FAKE_COLLECT).collect();
        assert_eq!(
            calls,
            vec![
                (FAKE_COLLECT, args![collector, w.handles[0], 0u32].to_vec()),
                (FAKE_COLLECT, args![collector, w.handles[1], 0u32].to_vec())
            ]
        );
    }

    #[test]
    fn cache_for_a_float_box_holds_the_overlapping_nodes() {
        let mut w = populated();
        let e = &mut w.e;
        identity_quantisation(e, w.this);
        let query = float_box(e, [6400.0; 3], [6656.0; 3]);
        let cache = cache_block(e, w.this);
        calc_aabb_cache(e, w.this, query, cache);
        let expected = expected_cache();
        for (axis, words) in expected.iter().enumerate() {
            assert_eq!(cache_words(e, cache, axis as u32), *words, "axis {axis}");
        }
    }

    #[test]
    fn cache_for_collidables_takes_their_node_indices() {
        let mut w = populated();
        let e = &mut w.e;
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x14, 4);
        e.mem.set_u32(second + 0x14, 5);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, first);
        e.mem.set_u32(data + 4, second);
        let objects = e.mem.alloc(12);
        e.mem.set_u32(objects, data);
        e.mem.set_u32(objects + 4, 2);
        e.mem.set_u32(objects + 8, 2);
        let cache = cache_block(e, w.this);
        calc_aabb_cache_ov2(e, w.this, objects, cache);
        let expected = expected_cache();
        for (axis, words) in expected.iter().enumerate() {
            assert_eq!(cache_words(e, cache, axis as u32), *words, "axis {axis}");
        }
        // The bump pointer is back where it was.
        assert_eq!(e.mem.u32(ROUTER), BUMP_REGION);
    }

    #[test]
    fn defragment_numbers_the_nodes_in_x_order() {
        let mut w = populated();
        let e = &mut w.e;
        defragment(e, w.this);
        assert_consistent(e, w.this);
        assert_eq!(e.get(w.this, Hkp3AxisSweep::node_count), 8);
        // Node order follows the x minima: sentinel, A (0x1000), D (0x1400), B (0x1800),
        // marker 1 (0x4000), C (0x5000), marker 2, marker 3.
        let order = [
            (w.handles[0], 1),
            (w.handles[3], 2),
            (w.handles[1], 3),
            (w.handles[2], 5),
        ];
        for (handle, id) in order {
            assert_eq!(e.mem.u32(handle), id);
        }
        assert_eq!(box_of(e, w.this, 2), BOXES_AS_U16[3]);
        // The marker list that held node 5 (B) now holds node 3.
        let marker = e.get(w.this, Hkp3AxisSweep::markers);
        assert_eq!(e.mem.i32(marker + 8), 1);
        assert_eq!(e.mem.u16(e.mem.u32(marker + 4)), 3);
    }

    /// [`BOXES`] with 16-bit values.
    const BOXES_AS_U16: [[(u16, u16); 3]; 4] = {
        let mut out = [[(0, 0); 3]; 4];
        let mut i = 0;
        while i < 4 {
            let mut axis = 0;
            while axis < 3 {
                out[i][axis] = (BOXES[i][axis].0 as u16, BOXES[i][axis].1 as u16);
                axis += 1;
            }
            i += 1;
        }
        out
    };

    #[test]
    fn creation_function_allocates_and_constructs() {
        let mut e = setup();
        let min = vec_at(&mut e, [-10.0, -10.0, -10.0, 0.0]);
        let max = vec_at(&mut e, [10.0, 10.0, 10.0, 0.0]);
        e.register(HEAP_ALLOC_FN, |e, a| {
            assert_eq!(a[1], 0xe0);
            e.mem.alloc(0xe0).into_ret()
        });
        e.put_vtable(
            ALLOCATOR_VTABLES + 0x100,
            &[
                FAKE_HEAP_NOOP,
                HEAP_ALLOC_FN,
                FAKE_HEAP_NOOP,
                FAKE_STACK_ALLOC,
                FAKE_STACK_FREE,
            ],
        );
        e.mem.set_u32(HEAP_ALLOCATOR, ALLOCATOR_VTABLES + 0x100);
        let object = fn_00d182a0(&mut e, min, max, 2);
        assert_eq!(e.mem.u32(object.addr()), VTABLE);
        assert_eq!(e.mem.u16(object.addr() + 4), 0xe0);
        assert_eq!(e.get(object, Hkp3AxisSweep::marker_bits), 1);
        assert_eq!(e.get(object, Hkp3AxisSweep::marker_count), 1);
        assert_eq!(load_vec4(&e, object.addr() + WORLD_MAX)[0], 10.0);
    }

    #[test]
    fn destructor_releases_the_arrays_and_the_marker_block() {
        let mut w = populated();
        let e = &mut w.e;
        let me = w.this.addr();
        let markers = e.get(w.this, Hkp3AxisSweep::markers);
        e.mem.set_u32(me, 0);
        e.call_log = Some(vec![]);
        hkp_3_axis_sweep_destructor(e, w.this);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(me), VTABLE);
        let freed: Vec<_> = log.iter().filter(|c| c.0 == EASY_FREE).collect();
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0].1, args![HEAP_ALLOCATOR, markers].to_vec());
        assert_eq!(log.iter().filter(|c| c.0 == BASE_DESTRUCTOR).count(), 1);
        assert_eq!(log.last().unwrap(), &(BASE_DESTRUCTOR, args![me].to_vec()));
        for array in [0xa0u32, 0xac, 0xb8, 0xc4] {
            assert_eq!(e.mem.u32(me + array), 0);
            assert_eq!(e.mem.u32(me + array + 4), 0);
            assert_eq!(e.mem.u32(me + array + 8), NOT_OWNED);
        }
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_object_when_asked() {
        let mut w = populated();
        let e = &mut w.e;
        let me = w.this.addr();
        e.mem.set_u16(me + 4, 0xe0);
        e.call_log = Some(vec![]);
        let result = hkp_3_axis_sweep_scalar_deleting_destructor(e, w.this, 0);
        assert_eq!(result, w.this);
        let log: Log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != FAKE_HEAP_NOOP));
        let (mut e, this) = fresh();
        e.mem.set_u16(this.addr() + 4, 0xe0);
        e.call_log = Some(vec![]);
        hkp_3_axis_sweep_scalar_deleting_destructor(&mut e, this, 1);
        let log: Log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == FAKE_HEAP_NOOP).collect();
        assert_eq!(
            frees.last().unwrap().1,
            args![HEAP_ALLOCATOR, this.addr(), 0xe0u32].to_vec()
        );
    }

    #[test]
    fn low_and_high_32_bit_offsets_are_copied_out() {
        let mut e = setup();
        let this = e.new_object::<Hkp3AxisSweep>();
        store_vec4(&mut e, this.addr() + OFFSET_LOW_32, [1.0, 2.0, 3.0, 4.0]);
        store_vec4(&mut e, this.addr() + OFFSET_HIGH_32, [5.0, 6.0, 7.0, 8.0]);
        let low = e.mem.alloc(16);
        let high = e.mem.alloc(16);
        get_offset_low_high_32_bit(&mut e, this, low, high);
        assert_eq!(load_vec4(&e, low), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec4(&e, high), [5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn larger_of_two_halfwords() {
        let mut e = setup();
        assert_eq!(fn_00d19d70(&mut e, 3, 9), 9);
        assert_eq!(fn_00d19d70(&mut e, 9, 3), 9);
        assert_eq!(fn_00d19d70(&mut e, 0xffff, 0), 0xffff);
        assert_eq!(fn_00d19d70(&mut e, 5, 5), 5);
    }

    #[test]
    fn index_of_the_smallest_of_three_floats() {
        let mut e = setup();
        let values = e.mem.alloc(12);
        let out = e.mem.alloc(4);
        for (triple, index) in [
            ([1.0f32, 2.0, 3.0], 0),
            ([2.0, 1.0, 3.0], 1),
            ([3.0, 2.0, 1.0], 2),
            ([2.0, 2.0, 2.0], 2),
        ] {
            for (i, v) in triple.iter().enumerate() {
                e.mem.set_f32(values + 4 * i as u32, *v);
            }
            assert_eq!(fn_00d19d10(&mut e, values, out), index, "{triple:?}");
            assert_eq!(e.mem.f32(out), triple[index as usize]);
        }
        // A signalling NaN is quieted on its way out, and counts as not greater.
        e.mem.set_f32(values, 1.0);
        e.mem.set_f32(values + 4, 2.0);
        e.mem.set_u32(values + 8, 0x7fa0_0000);
        assert_eq!(fn_00d19d10(&mut e, values, out), 2);
        assert_eq!(e.mem.u32(out), 0x7fe0_0000);
        e.mem.set_f32(values, 3.0);
        e.mem.set_u32(values + 4, 0x7fc0_0000);
        e.mem.set_f32(values + 8, 1.0);
        assert_eq!(fn_00d19d10(&mut e, values, out), 2);
        assert_eq!(e.mem.f32(out), 1.0);
    }

    type MergeFn = fn(&mut Engine, u32, u32, i32, u32, i32);

    /// Merges two new endpoints into an array whose last two slots are free; `fields` are
    /// the node fields (minimum, maximum) of the axis.
    fn check_merge(merge: MergeFn, fields: [u32; 2]) {
        let mut e = setup();
        let nodes = put_nodes(&mut e, &[[0; 6]; 8]);
        let array = word_array(&mut e, &[0, ep(0x10, 1), ep(0x30, 2), 0, 0]);
        let new = word_array(&mut e, &[ep(0x20, 7), ep(0x41, 7)]);
        let new_data = e.mem.u32(new);
        merge(&mut e, array, nodes, 5, new_data, 2);
        let data = e.mem.u32(array);
        let at = |index: u32| (e.mem.u16(data + 4 * index), e.mem.u16(data + 4 * index + 2));
        assert_eq!(at(1), (0x10, 1));
        assert_eq!(at(2), (0x20, 7));
        assert_eq!(at(3), (0x30, 2));
        assert_eq!(at(4), (0x41, 7));
        assert_eq!(e.mem.u16(nodes + 16 * 7 + fields[0]), 2);
        assert_eq!(e.mem.u16(nodes + 16 * 7 + fields[1]), 4);
        assert_eq!(e.mem.u16(nodes + 16 * 2 + fields[0]), 3);
    }

    #[test]
    fn x_endpoints_merge_from_the_back() {
        check_merge(fn_00d19d90, [8, 10]);
    }

    #[test]
    fn y_endpoints_merge_from_the_back() {
        check_merge(fn_00d19e40, [0, 4]);
    }

    #[test]
    fn z_endpoints_merge_from_the_back() {
        check_merge(fn_00d19ef0, [2, 6]);
    }

    #[test]
    fn quick_sort_orders_endpoints_by_value_then_node() {
        let mut e = setup();
        let array = word_array(
            &mut e,
            &[ep(5, 3), ep(2, 9), ep(5, 1), ep(2, 4), ep(9, 0), ep(1, 7)],
        );
        let data = e.mem.u32(array);
        quick_sort_recursive(&mut e, data, 1, 4, 0);
        assert_eq!(
            words_of(&e, array),
            [ep(5, 3), ep(2, 4), ep(2, 9), ep(5, 1), ep(9, 0), ep(1, 7)]
        );
        quick_sort_recursive(&mut e, data, 0, 5, 0);
        assert_eq!(
            words_of(&e, array),
            [ep(1, 7), ep(2, 4), ep(2, 9), ep(5, 1), ep(5, 3), ep(9, 0)]
        );
    }

    #[test]
    fn value_only_sort_orders_endpoints_by_value() {
        let mut e = setup();
        let words = [
            ep(5, 3),
            ep(2, 9),
            ep(5, 1),
            ep(2, 4),
            ep(9, 0),
            ep(1, 7),
            ep(0xfffc, 2),
        ];
        let array = word_array(&mut e, &words);
        let data = e.mem.u32(array);
        fn_00d1a220(&mut e, data, 0, 6, 0);
        let sorted = words_of(&e, array);
        let values: Vec<u32> = sorted.iter().map(|w| w & 0xffff).collect();
        assert_eq!(values, [1, 2, 2, 5, 5, 9, 0xfffc]);
        let mut again = sorted.clone();
        again.sort_unstable();
        let mut original = words.to_vec();
        original.sort_unstable();
        assert_eq!(again, original);
    }

    type CompactFn = fn(&mut Engine, u32, u32, u32);

    /// Compacts an endpoint array through a mapping where node 2 goes away.
    fn check_compact(compact: CompactFn, fields: [u32; 2]) {
        let mut e = setup();
        let nodes = put_nodes(&mut e, &[[0; 6]; 4]);
        let array = word_array(
            &mut e,
            &[
                ep(0x10, 1),
                ep(0x20, 2),
                ep(0x31, 1),
                ep(0x40, 3),
                ep(0x51, 3),
            ],
        );
        let table = e.mem.alloc(16);
        for (i, v) in [0i32, 2, -1, 1].iter().enumerate() {
            e.mem.set_i32(table + 4 * i as u32, *v);
        }
        let mapping = e.mem.alloc(12);
        e.mem.set_u32(mapping, table);
        compact(&mut e, array, nodes, mapping);
        assert_eq!(
            words_of(&e, array),
            [ep(0x10, 2), ep(0x31, 2), ep(0x40, 1), ep(0x51, 1)]
        );
        // Node 2 (was 1): endpoints 0 and 1; node 1 (was 3): endpoints 2 and 3.
        assert_eq!(e.mem.u16(nodes + 16 * 2 + fields[0]), 0);
        assert_eq!(e.mem.u16(nodes + 16 * 2 + fields[1]), 1);
        assert_eq!(e.mem.u16(nodes + 16 + fields[0]), 2);
        assert_eq!(e.mem.u16(nodes + 16 + fields[1]), 3);
    }

    #[test]
    fn x_endpoints_compact_through_a_mapping() {
        check_compact(fn_00d1a040, [8, 10]);
    }

    #[test]
    fn y_endpoints_compact_through_a_mapping() {
        check_compact(fn_00d1a0e0, [0, 4]);
    }

    #[test]
    fn z_endpoints_compact_through_a_mapping() {
        check_compact(fn_00d1a180, [2, 6]);
    }

    #[test]
    fn shifting_everything_moves_the_endpoints_by_the_scaled_shift() {
        let mut w = populated();
        let e = &mut w.e;
        identity_quantisation(e, w.this);
        let shift = vec_at(e, [8.0, -6.0, 0.0, 0.0]);
        let applied = e.mem.alloc(16);
        let pairs = empty_array(e);
        shift_all_objects(e, w.this, shift, applied, pairs);
        assert_eq!(load_vec4(e, applied), [8.0, -6.0, 0.0, 0.0]);
        assert_consistent(e, w.this);
        assert_eq!(
            box_of(e, w.this, 4),
            [(0x1008, 0x2009), (0x0ffa, 0x1ffb), (0x1000, 0x2001)]
        );
        assert_eq!(
            box_of(e, w.this, 5),
            [(0x1808, 0x4809), (0x17fa, 0x2ffb), (0x1800, 0x3001)]
        );
        assert!(pairs_of(e, pairs).is_empty());
        // The sentinels stay where they are.
        assert_eq!(endpoints_of(e, w.this, 0)[0], (0, 0));
        assert_eq!(endpoints_of(e, w.this, 0).last().unwrap(), &(0xfffd, 0));
    }

    #[test]
    fn batch_add_inserts_all_objects_and_reports_their_overlaps() {
        let (mut e, this) = fresh();
        identity_quantisation(&mut e, this);
        let handles_data = e.mem.alloc(8);
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(handles_data, first);
        e.mem.set_u32(handles_data + 4, second);
        let handles = e.mem.alloc(12);
        e.mem.set_u32(handles, handles_data);
        e.mem.set_u32(handles + 4, 2);
        e.mem.set_u32(handles + 8, 2);
        let boxes_data = e.mem.alloc(64);
        store_vec4(&mut e, boxes_data, [4096.0; 4]);
        store_vec4(&mut e, boxes_data + 0x10, [8192.0; 4]);
        store_vec4(&mut e, boxes_data + 0x20, [6144.0; 4]);
        store_vec4(
            &mut e,
            boxes_data + 0x30,
            [18432.0, 12288.0, 12288.0, 12288.0],
        );
        let boxes = e.mem.alloc(12);
        e.mem.set_u32(boxes, boxes_data);
        e.mem.set_u32(boxes + 4, 2);
        e.mem.set_u32(boxes + 8, 2);
        let pairs = empty_array(&mut e);
        add_object_batch(&mut e, this, handles, boxes, pairs);
        assert_eq!(e.get(this, Hkp3AxisSweep::node_count), 6);
        assert_consistent(&e, this);
        assert_eq!(e.mem.u32(first), 4);
        assert_eq!(e.mem.u32(second), 5);
        assert_eq!(box_of(&e, this, 4), BOXES_AS_U16[0]);
        assert_eq!(box_of(&e, this, 5), BOXES_AS_U16[1]);
        assert_eq!(pairs_of(&e, pairs), [(first, second)]);
        // An empty box list adds nothing.
        e.mem.set_u32(boxes + 4, 0);
        add_object_batch(&mut e, this, handles, boxes, pairs);
        assert_eq!(e.get(this, Hkp3AxisSweep::node_count), 6);
    }

    #[test]
    fn batch_remove_drops_the_objects_and_reports_the_ended_overlaps() {
        let mut w = populated();
        let e = &mut w.e;
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, w.handles[1]);
        e.mem.set_u32(data + 4, w.handles[3]);
        let objects = e.mem.alloc(12);
        e.mem.set_u32(objects, data);
        e.mem.set_u32(objects + 4, 2);
        e.mem.set_u32(objects + 8, 2);
        let pairs = empty_array(e);
        remove_object_batch(e, w.this, objects, pairs);
        assert_eq!(e.get(w.this, Hkp3AxisSweep::node_count), 6);
        assert_consistent(e, w.this);
        assert_eq!(pairs_of(e, pairs), [(w.handles[1], w.handles[0])]);
        // The last node (the third object) takes the place of the first removed one.
        assert_eq!(e.mem.u32(w.handles[0]), 4);
        assert_eq!(e.mem.u32(w.handles[2]), 5);
        assert_eq!(box_of(e, w.this, 4), BOXES_AS_U16[0]);
        assert_eq!(box_of(e, w.this, 5), BOXES_AS_U16[2]);
        let marker = e.get(w.this, Hkp3AxisSweep::markers);
        assert_eq!(e.mem.i32(marker + 8), 0);
    }

    /// A collector object whose slot `+4` answers `fraction`.
    fn collector_with(e: &mut Engine, fraction: f32) -> u32 {
        let collector = e.mem.alloc(16);
        e.mem.set_u32(collector, COLLECTOR_VTABLE);
        e.mem.set_f32(collector + 8, fraction);
        collector
    }

    #[test]
    fn ray_reports_the_objects_it_enters_until_the_fraction_is_passed() {
        let mut w = populated();
        let e = &mut w.e;
        identity_quantisation(e, w.this);
        let collector = collector_with(e, 0.25);
        let end = vec_at(e, [30000.0, 7000.0, 7000.0, 0.0]);
        let input = e.mem.alloc(0x20);
        store_vec4(e, input, [4500.0, 7000.0, 7000.0, 0.0]);
        e.mem.set_u32(input + 0x10, 1);
        e.mem.set_u32(input + 0x14, end);
        e.mem.set_u32(input + 0x18, 16);
        e.call_log = Some(vec![]);
        cast_ray(e, w.this, input, collector, 16);
        let log: Log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 == FAKE_COLLECT).collect();
        // The object around the origin first, then the one the ray runs into at x 6144.
        assert_eq!(
            calls,
            vec![
                (FAKE_COLLECT, args![collector, w.handles[0], 0u32].to_vec()),
                (FAKE_COLLECT, args![collector, w.handles[1], 0u32].to_vec())
            ]
        );
        assert_eq!(e.mem.u32(ROUTER), BUMP_REGION);
    }

    #[test]
    fn box_sweep_reports_the_objects_it_meets_until_the_fraction_is_passed() {
        let mut w = populated();
        let e = &mut w.e;
        identity_quantisation(e, w.this);
        let collector = collector_with(e, 0.25);
        let input = e.mem.alloc(0x40);
        store_vec4(e, input, [4500.0, 7000.0, 7000.0, 0.0]);
        store_vec4(e, input + 0x10, [30000.0, 7000.0, 7000.0, 0.0]);
        store_vec4(e, input + 0x20, [100.0, 100.0, 100.0, 0.0]);
        e.call_log = Some(vec![]);
        cast_aabb(e, w.this, input, collector);
        let log: Log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 == FAKE_COLLECT).collect();
        assert_eq!(
            calls,
            vec![
                (FAKE_COLLECT, args![collector, w.handles[0], 0u32].to_vec()),
                (FAKE_COLLECT, args![collector, w.handles[1], 0u32].to_vec())
            ]
        );
    }
}
