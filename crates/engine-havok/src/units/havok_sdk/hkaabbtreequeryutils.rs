//! `Havok SDK/hkaabbtreequeryutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{
    add_ps, all_three_le, allocator, load_vec, max_ps, min_ps, mul_ps, narrow_bounds, rcp_estimate,
    release_array, reserve_at_least, sub_ps, Vec4, ARRAY_RESERVE_MORE, CAPACITY_MASK, FLOAT_ONE,
    FLOAT_ZERO, LEAF_NARROW, LEAF_WIDE, NODE_AT_NARROW, NODE_AT_WIDE, NONE_NARROW, NONE_WIDE,
    ONES_VECTOR, ZERO_VECTOR,
};

/// The node accessor of the third node manager kind (`00d1bb40`, `RET 4`, `this` = manager, one
/// word: index): data (the manager's first word) + the low word of the index * 0x30.
const NODE_AT_FLOAT16: u32 = 0x00d1_bb40;
/// Vectors in the exe image: the large value a ray direction lane that is exactly zero gets as
/// its inverse (`010cd670`, four floats), the per-ray bits 1, 2, 4, 8 (`010c92e0`, four words)
/// and the table of lane masks for each set of active rays (`01268460`, 16 bytes per set).
const BIG_VECTOR: u32 = 0x010c_d670;
const RAY_BITS: u32 = 0x010c_92e0;
const ACTIVE_RAY_LANES: u32 = 0x0126_8460;

/// The three node layouts the query functions are written for. All three trees keep the root
/// node at +0x10 and the node manager (whose first word is the node storage) at +0x1c.
///
/// * `Narrow`: nodes of 0x14 bytes with six 16-bit grid bounds (words 0..2 the upper corner,
///   3..5 the lower), child words at +0xC and +0xE, leaf bit 15 of the word at +0x10, the
///   next-node link word at +0x12 (`0x7fff` = none), leaf data dword at +0xC. The grid is
///   scaled by the vector at tree +0x70 and offset by the vector at tree +0x40.
/// * `Float16`: nodes of 0x30 bytes with float bounds (lower corner at +0, upper at +0x10),
///   child words at +0x20 / +0x22, flags word at +0x24 with the leaf bit 15, link word at
///   +0x26, leaf data dword at +0x20; nodes are found by 16-bit index.
/// * `Wide`: nodes of 0x30 bytes with float bounds, child dwords at +0x20 / +0x24, flags dword
///   at +0x28 with the leaf bit 31, link dword at +0x2C (`0x7fffffff` = none), leaf data dword
///   at +0x20; nodes are found by 32-bit index.
#[derive(Clone, Copy)]
enum Kind {
    Narrow,
    Float16,
    Wide,
}

impl Kind {
    /// The node of `index` (calls the manager's accessor with the manager at tree +0x1c).
    fn node_at(self, e: &mut Engine, tree: u32, index: u32) -> u32 {
        let accessor = match self {
            Kind::Narrow => NODE_AT_NARROW,
            Kind::Float16 => NODE_AT_FLOAT16,
            Kind::Wide => NODE_AT_WIDE,
        };
        e.call(accessor, &args![tree + 0x1c, index]).u32()
    }

    /// The node the traversal continues with after `node` (its subtree skipped), or 0.
    fn next(self, e: &mut Engine, tree: u32, node: u32) -> u32 {
        match self {
            Kind::Narrow | Kind::Float16 => {
                let link_at = if matches!(self, Kind::Narrow) {
                    0x12
                } else {
                    0x26
                };
                let link = e.mem.u16(node + link_at);
                if link == NONE_NARROW {
                    0
                } else {
                    self.node_at(e, tree, u32::from(link))
                }
            }
            Kind::Wide => {
                let link = e.mem.u32(node + 0x2c);
                if link == NONE_WIDE {
                    0
                } else {
                    self.node_at(e, tree, link)
                }
            }
        }
    }

    fn is_leaf(self, e: &Engine, node: u32) -> bool {
        match self {
            Kind::Narrow => e.mem.u16(node + 0x10) & LEAF_NARROW != 0,
            Kind::Float16 => e.mem.u16(node + 0x24) & LEAF_NARROW != 0,
            Kind::Wide => e.mem.u32(node + 0x28) & LEAF_WIDE != 0,
        }
    }

    /// The first child of an inner node.
    fn first_child(self, e: &mut Engine, tree: u32, node: u32) -> u32 {
        let index = match self {
            Kind::Narrow => u32::from(e.mem.u16(node + 0xc)),
            Kind::Float16 => u32::from(e.mem.u16(node + 0x20)),
            Kind::Wide => e.mem.u32(node + 0x20),
        };
        self.node_at(e, tree, index)
    }

    /// The data dword of a leaf.
    fn leaf_data(self, e: &Engine, node: u32) -> u32 {
        match self {
            Kind::Narrow => e.mem.u32(node + 0xc),
            Kind::Float16 | Kind::Wide => e.mem.u32(node + 0x20),
        }
    }

    /// The (lower, upper) corners of the node's bounds in tree space.
    fn bounds(self, e: &Engine, tree: u32, node: u32) -> (Vec4, Vec4) {
        match self {
            Kind::Narrow => narrow_world_bounds(e, tree, node),
            Kind::Float16 | Kind::Wide => (load_vec(e, node), load_vec(e, node + 0x10)),
        }
    }
}

/// `FILD` of a grid word shifted left by 15 (exact in a float).
fn grid_lane(word: u16) -> f32 {
    (u32::from(word) << 15) as i32 as f32
}

/// The bounds of a narrow node in tree space: the lanes (w3, w4, w5, zero) and (w0, w1, w2,
/// zero) as floats of `word << 15`, `lower = lanes * scale - offset` and
/// `upper = scale * lanes - offset` with the scale at tree +0x70 and the offset at +0x40.
fn narrow_world_bounds(e: &Engine, tree: u32, node: u32) -> (Vec4, Vec4) {
    let zero = e.mem.f32(FLOAT_ZERO);
    let words = narrow_bounds(e, node);
    let upper_lanes = [
        grid_lane(words[0]),
        grid_lane(words[1]),
        grid_lane(words[2]),
        zero,
    ];
    let lower_lanes = [
        grid_lane(words[3]),
        grid_lane(words[4]),
        grid_lane(words[5]),
        zero,
    ];
    let scale = load_vec(e, tree + 0x70);
    let offset = load_vec(e, tree + 0x40);
    let lower = sub_ps(mul_ps(lower_lanes, scale), offset);
    let upper = sub_ps(mul_ps(scale, upper_lanes), offset);
    (lower, upper)
}

fn splat(value: f32) -> Vec4 {
    [value; 4]
}

/// The box test of the AABB queries: `lower <= query max` and `query min <= upper` on the
/// three low lanes.
fn boxes_overlap(lower: Vec4, upper: Vec4, query_min: Vec4, query_max: Vec4) -> bool {
    all_three_le(lower, query_max) && all_three_le(query_min, upper)
}

/// The inverse ray direction the ray queries share: `target - origin` with the lanes that are
/// exactly zero replaced by one, lane 3 taken from the ones vector, the reciprocal estimate
/// refined by one Newton step `(r + r) - (r * v) * r`, and the lanes that were zero replaced
/// by the big vector's lanes.
fn ray_inverse_direction(e: &Engine, origin: Vec4, target: Vec4) -> Vec4 {
    let direction = sub_ps(target, origin);
    let zero = load_vec(e, ZERO_VECTOR);
    let ones = load_vec(e, ONES_VECTOR);
    let big = load_vec(e, BIG_VECTOR);
    let was_zero: [bool; 4] = std::array::from_fn(|lane| zero[lane] == direction[lane]);
    let safe: Vec4 = std::array::from_fn(|lane| {
        if was_zero[lane] {
            ones[lane]
        } else {
            direction[lane]
        }
    });
    let v = [safe[0], safe[1], safe[2], ones[3]];
    let estimate: Vec4 = std::array::from_fn(|lane| rcp_estimate(v[lane]));
    let refined = sub_ps(
        add_ps(estimate, estimate),
        mul_ps(mul_ps(estimate, v), estimate),
    );
    std::array::from_fn(|lane| {
        if was_zero[lane] {
            big[lane]
        } else {
            refined[lane]
        }
    })
}

/// The slab test of the ray queries on the entry/exit distances at both box corners: entry is
/// the largest of the per-axis minima and zero, exit the smallest of the per-axis maxima and
/// the current limit; the ray hits when lane 0 of entry is below lane 0 of exit.
fn ray_enters(e: &Engine, t_low: Vec4, t_high: Vec4, limit: Vec4) -> bool {
    let zero = load_vec(e, ZERO_VECTOR);
    let t_min = min_ps(t_low, t_high);
    let t_max = max_ps(t_low, t_high);
    let exit = min_ps(
        min_ps(t_max, limit),
        min_ps(splat(t_max[1]), splat(t_max[2])),
    );
    let entry = max_ps(
        max_ps(t_min, zero),
        max_ps(splat(t_min[1]), splat(t_min[2])),
    );
    entry[0] < exit[0]
}

/// The address at which an array of `element_size`-byte elements stores its next element:
/// first makes room (`00c90940`) when the size equals the capacity, via the allocator found
/// through the memory router (which is called in either case).
fn next_slot(e: &mut Engine, array: u32, element_size: u32) -> u32 {
    let alloc = allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![alloc, array, element_size]);
    }
    let size = e.mem.u32(array + 4);
    e.mem
        .u32(array)
        .wrapping_add(size.wrapping_mul(element_size))
}

// Translated from 00d1bcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTreeQueryUtils<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::TreeTreeNodePairCollector::addHit`
/// (Xbox PDB), thiscall, `RET 8`; `this` (register input) holds at +4 the pointer to an array
/// of 8-byte pairs. Appends the pair (`first`, `second`) (growing the array when it is full;
/// nothing is stored when the slot address is 0) and returns false so the query goes on.
pub fn hk_aabb_tree_query_utils_pair_collector_add_hit(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
) -> bool {
    let array = e.mem.u32(this.addr() + 4);
    let slot = next_slot(e, array, 8);
    if slot != 0 {
        e.mem.set_u32(slot, first);
        e.mem.set_u32(slot + 4, second);
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    false
}

/// Casts one ray through a tree: nodes whose box the ray enters before the limit are
/// descended; each leaf entered calls the collector's function at +4 with the leaf and the
/// result becomes the new limit. `limit` starts as `max_fraction`.
fn cast_ray(
    e: &mut Engine,
    kind: Kind,
    tree: u32,
    origin: u32,
    target: u32,
    collector: u32,
    max_fraction: f32,
) {
    if tree == 0 {
        return;
    }
    let mut node = e.mem.u32(tree + 0x10);
    if node == 0 {
        return;
    }
    let origin_v = load_vec(e, origin);
    let target_v = load_vec(e, target);
    let inverse = ray_inverse_direction(e, origin_v, target_v);
    let mut limit = splat(max_fraction);
    while node != 0 {
        let next = kind.next(e, tree, node);
        let (lower, upper) = kind.bounds(e, tree, node);
        let t_low = mul_ps(sub_ps(lower, origin_v), inverse);
        let t_high = mul_ps(sub_ps(upper, origin_v), inverse);
        if ray_enters(e, t_low, t_high, limit) {
            if !kind.is_leaf(e, node) {
                node = kind.first_child(e, tree, node);
                continue;
            }
            limit = splat(e.vcall(collector, 4, &args![node]).f32());
        }
        node = next;
    }
}

// Translated from 00d1bd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Single ray through a narrow tree (cdecl; void): `origin` and `target` are vectors, the
/// collector's function at +4 gets each leaf the ray enters and returns the new maximum
/// fraction. Returns at once when the tree or its root is null.
pub fn fn_00d1bd00(
    e: &mut Engine,
    tree: Ptr,
    origin: Ptr,
    target: Ptr,
    collector: Ptr,
    max_fraction: f32,
) {
    cast_ray(
        e,
        Kind::Narrow,
        tree.addr(),
        origin.addr(),
        target.addr(),
        collector.addr(),
        max_fraction,
    );
}

/// Four rays at once through a tree from `start` until the traversal reaches `stop`. `rays`
/// holds the origins (3 vectors of 4 rays, one per axis, at +0), the inverse directions (at
/// +0x30) and the active-ray set (a word at +0x60); `collectors` points at 4 collector
/// objects; the 4 maximum fractions are read from `fractions` (a local copy is updated, the
/// caller's stays).
fn cast_four_rays(
    e: &mut Engine,
    kind: Kind,
    tree: u32,
    start: u32,
    stop: u32,
    rays: u32,
    collectors: u32,
    fractions: u32,
) {
    let mut limit = load_vec(e, fractions);
    let mut node = start;
    loop {
        let next = kind.next(e, tree, node);
        let (lower, upper) = kind.bounds(e, tree, node);
        let slabs: [(Vec4, Vec4); 3] = std::array::from_fn(|axis| {
            let origin = load_vec(e, rays + 16 * axis as u32);
            let inverse = load_vec(e, rays + 0x30 + 16 * axis as u32);
            let t0 = mul_ps(sub_ps(splat(lower[axis]), origin), inverse);
            let t1 = mul_ps(sub_ps(splat(upper[axis]), origin), inverse);
            (min_ps(t0, t1), max_ps(t0, t1))
        });
        let zero = load_vec(e, ZERO_VECTOR);
        let entry = max_ps(max_ps(max_ps(slabs[0].0, slabs[1].0), slabs[2].0), zero);
        let exit = min_ps(min_ps(min_ps(slabs[0].1, slabs[1].1), slabs[2].1), limit);
        let active = e.mem.u32(rays + 0x60);
        let lanes_at = ACTIVE_RAY_LANES.wrapping_add(active.wrapping_shl(4));
        let mut hit = 0u32;
        for lane in 0..4u32 {
            let below = u32::from(entry[lane as usize] < exit[lane as usize]);
            let allowed = e.mem.u32(lanes_at + 4 * lane) >> 31;
            hit |= (below & allowed) << lane;
        }
        let mut descended = false;
        if active & hit != 0 {
            if !kind.is_leaf(e, node) {
                node = kind.first_child(e, tree, node);
                descended = true;
            } else {
                for (lane, fraction) in limit.iter_mut().enumerate() {
                    if e.mem.u32(RAY_BITS + 4 * lane as u32) & hit != 0 {
                        let collector = e.mem.u32(collectors + 4 * lane as u32);
                        *fraction = e.vcall(collector, 4, &args![node]).f32();
                    }
                }
            }
        }
        if descended {
            continue;
        }
        node = next;
        if node == stop {
            return;
        }
    }
}

// Translated from 00d1bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Four rays through a narrow tree (cdecl; void), from `start` to `stop`.
pub fn fn_00d1bfa0(
    e: &mut Engine,
    tree: Ptr,
    start: Ptr,
    stop: Ptr,
    rays: Ptr,
    collectors: Ptr,
    fractions: Ptr,
) {
    cast_four_rays(
        e,
        Kind::Narrow,
        tree.addr(),
        start.addr(),
        stop.addr(),
        rays.addr(),
        collectors.addr(),
        fractions.addr(),
    );
}

// Translated from 00d1ca00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Four rays through a tree with 16-bit indices and float bounds (cdecl; void), from `start`
/// to `stop`.
pub fn fn_00d1ca00(
    e: &mut Engine,
    tree: Ptr,
    start: Ptr,
    stop: Ptr,
    rays: Ptr,
    collectors: Ptr,
    fractions: Ptr,
) {
    cast_four_rays(
        e,
        Kind::Float16,
        tree.addr(),
        start.addr(),
        stop.addr(),
        rays.addr(),
        collectors.addr(),
        fractions.addr(),
    );
}

// Translated from 00d1d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump to `00d1bfa0` (same arguments).
pub fn fn_00d1d490(
    e: &mut Engine,
    tree: Ptr,
    start: Ptr,
    stop: Ptr,
    rays: Ptr,
    collectors: Ptr,
    fractions: Ptr,
) {
    fn_00d1bfa0(e, tree, start, stop, rays, collectors, fractions);
}

// Translated from 00d1d530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump to `00d1ca00` (same arguments).
pub fn fn_00d1d530(
    e: &mut Engine,
    tree: Ptr,
    start: Ptr,
    stop: Ptr,
    rays: Ptr,
    collectors: Ptr,
    fractions: Ptr,
) {
    fn_00d1ca00(e, tree, start, stop, rays, collectors, fractions);
}

/// A ray widened by `padding` (plus a per-lane padding vector at ray +0x20) through a tree:
/// `ray` holds the origin at +0 and the end point at +0x10; the limit starts at 1.0. Every leaf
/// entered goes to the collector (its function at +4 returns the new limit) when there is one,
/// else its data dword is stored at `cursor` (which advances by 4; stops after `max_count`
/// leaves), else it is only counted. Returns the number of leaves counted.
fn cast_padded_ray(
    e: &mut Engine,
    kind: Kind,
    tree: u32,
    ray: u32,
    collector: u32,
    padding: f32,
    cursor: u32,
    max_count: i32,
) -> u32 {
    if tree == 0 {
        return 0;
    }
    let mut node = e.mem.u32(tree + 0x10);
    if node == 0 {
        return 0;
    }
    let origin = load_vec(e, ray);
    let target = load_vec(e, ray + 0x10);
    let inverse = ray_inverse_direction(e, origin, target);
    let pad = add_ps(splat(padding), load_vec(e, ray + 0x20));
    let mut limit = splat(e.mem.f32(FLOAT_ONE));
    let mut cursor = cursor;
    let mut count = 0i32;
    while node != 0 {
        let next = kind.next(e, tree, node);
        let (lower, upper) = kind.bounds(e, tree, node);
        let lower = sub_ps(lower, pad);
        let upper = add_ps(upper, pad);
        let t_low = mul_ps(sub_ps(lower, origin), inverse);
        let t_high = mul_ps(sub_ps(upper, origin), inverse);
        if ray_enters(e, t_low, t_high, limit) {
            if !kind.is_leaf(e, node) {
                node = kind.first_child(e, tree, node);
                continue;
            }
            if collector != 0 {
                limit = splat(e.vcall(collector, 4, &args![node]).f32());
            } else if cursor != 0 {
                let data = kind.leaf_data(e, node);
                e.mem.set_u32(cursor, data);
                cursor = cursor.wrapping_add(4);
                if count >= max_count {
                    return count as u32;
                }
            }
            count += 1;
        }
        node = next;
    }
    count as u32
}

// Translated from 00d1c2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Padded ray through a narrow tree (cdecl); the second word is not read. Returns the leaf
/// count.
pub fn fn_00d1c2b0(
    e: &mut Engine,
    tree: Ptr,
    _unused_1: u32,
    ray: Ptr,
    collector: Ptr,
    padding: f32,
    cursor: Ptr,
    max_count: i32,
) -> u32 {
    cast_padded_ray(
        e,
        Kind::Narrow,
        tree.addr(),
        ray.addr(),
        collector.addr(),
        padding,
        cursor.addr(),
        max_count,
    )
}

// Translated from 00d1cc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Padded ray through a tree with 16-bit indices and float bounds (cdecl); the second word is
/// not read. Returns the leaf count.
pub fn fn_00d1cc50(
    e: &mut Engine,
    tree: Ptr,
    _unused_1: u32,
    ray: Ptr,
    collector: Ptr,
    padding: f32,
    cursor: Ptr,
    max_count: i32,
) -> u32 {
    cast_padded_ray(
        e,
        Kind::Float16,
        tree.addr(),
        ray.addr(),
        collector.addr(),
        padding,
        cursor.addr(),
        max_count,
    )
}

// Translated from 00d1d0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Single ray through a tree of the wide kind (cdecl; void); see `00d1bd00`.
pub fn fn_00d1d0b0(
    e: &mut Engine,
    tree: Ptr,
    origin: Ptr,
    target: Ptr,
    collector: Ptr,
    max_fraction: f32,
) {
    cast_ray(
        e,
        Kind::Wide,
        tree.addr(),
        origin.addr(),
        target.addr(),
        collector.addr(),
        max_fraction,
    );
}

/// Collects the leaves whose box overlaps the query box (lower corner at `query`, upper at
/// +0x10). In order of preference: with a `collector` each leaf goes to its function at +4
/// and a result of exactly 0.0 ends the query, anything else counts the leaf; else with a
/// `cursor` (the address of a write pointer) the leaf's data dword is stored there and the
/// pointer advances by 4 (the query ends when the count has reached `max_count`, before
/// counting); else with an `array` the data dword is appended. Returns the count; 0 when the
/// root is null or none of the three outputs is given.
fn query_aabb_internal(
    e: &mut Engine,
    kind: Kind,
    tree: u32,
    query: u32,
    collector: u32,
    array: u32,
    cursor: u32,
    max_count: i32,
) -> u32 {
    let mut node = e.mem.u32(tree + 0x10);
    if node == 0 || (collector == 0 && cursor == 0 && array == 0) {
        return 0;
    }
    let mut count = 0i32;
    while node != 0 {
        let next = kind.next(e, tree, node);
        let (lower, upper) = kind.bounds(e, tree, node);
        let query_min = load_vec(e, query);
        let query_max = load_vec(e, query + 0x10);
        if boxes_overlap(lower, upper, query_min, query_max) {
            if !kind.is_leaf(e, node) {
                node = kind.first_child(e, tree, node);
                continue;
            }
            if collector != 0 {
                let result = e.vcall(collector, 4, &args![node]);
                let zero = f64::from(e.mem.f32(FLOAT_ZERO));
                if result.st0 == zero {
                    return count as u32;
                }
                count += 1;
            } else if cursor != 0 {
                let at = e.mem.u32(cursor);
                let data = kind.leaf_data(e, node);
                e.mem.set_u32(at, data);
                e.mem.set_u32(cursor, at.wrapping_add(4));
                if count >= max_count {
                    return count as u32;
                }
                count += 1;
            } else {
                let data = kind.leaf_data(e, node);
                let slot = next_slot(e, array, 4);
                e.mem.set_u32(slot, data);
                let size = e.mem.u32(array + 4);
                e.mem.set_u32(array + 4, size.wrapping_add(1));
                count += 1;
            }
        }
        node = next;
    }
    count as u32
}

// Translated from 00d1c560 (decompiled, FalloutNV.exe 1.4.0.525)
/// AABB query of a narrow tree (cdecl): see `query_aabb_internal`. Returns 0 when the root is
/// null (no null check of the tree itself).
pub fn fn_00d1c560(
    e: &mut Engine,
    tree: Ptr,
    query: Ptr,
    collector: Ptr,
    array: Ptr,
    cursor: Ptr,
    max_count: i32,
) -> u32 {
    query_aabb_internal(
        e,
        Kind::Narrow,
        tree.addr(),
        query.addr(),
        collector.addr(),
        array.addr(),
        cursor.addr(),
        max_count,
    )
}

// Translated from 00d1ce50 (decompiled, FalloutNV.exe 1.4.0.525)
/// AABB query of a tree with 16-bit indices and float bounds (cdecl); see
/// `query_aabb_internal`.
pub fn fn_00d1ce50(
    e: &mut Engine,
    tree: Ptr,
    query: Ptr,
    collector: Ptr,
    array: Ptr,
    cursor: Ptr,
    max_count: i32,
) -> u32 {
    query_aabb_internal(
        e,
        Kind::Float16,
        tree.addr(),
        query.addr(),
        collector.addr(),
        array.addr(),
        cursor.addr(),
        max_count,
    )
}

// Translated from 00d1d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTreeQueryUtils<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::queryAabbInternal`
/// (Xbox PDB), cdecl: AABB query of a tree of the wide kind; see `query_aabb_internal`.
pub fn hk_aabb_tree_query_utils_query_aabb_internal_uint(
    e: &mut Engine,
    tree: Ptr,
    query: Ptr,
    collector: Ptr,
    array: Ptr,
    cursor: Ptr,
    max_count: i32,
) -> u32 {
    query_aabb_internal(
        e,
        Kind::Wide,
        tree.addr(),
        query.addr(),
        collector.addr(),
        array.addr(),
        cursor.addr(),
        max_count,
    )
}

// Translated from 00d1d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// AABB query of a narrow tree into an array (cdecl): `00d1c560` with only the array given;
/// returns 0 for a null tree.
pub fn fn_00d1d4a0(e: &mut Engine, tree: Ptr, query: Ptr, array: Ptr) -> u32 {
    if tree.addr() == 0 {
        return 0;
    }
    fn_00d1c560(e, tree, query, Ptr::new(0), array, Ptr::new(0), 0)
}

// Translated from 00d1d4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// AABB query of a narrow tree into a raw buffer (cdecl): `00d1c560` with only the cursor and
/// the maximum given; returns 0 for a null tree.
pub fn fn_00d1d4d0(e: &mut Engine, tree: Ptr, query: Ptr, cursor: Ptr, max_count: i32) -> u32 {
    if tree.addr() == 0 {
        return 0;
    }
    fn_00d1c560(e, tree, query, Ptr::new(0), Ptr::new(0), cursor, max_count)
}

// Translated from 00d1d500 (decompiled, FalloutNV.exe 1.4.0.525)
/// AABB query of a narrow tree with a collector (cdecl): `00d1c560` with only the collector
/// given; returns 0 for a null tree.
pub fn fn_00d1d500(e: &mut Engine, tree: Ptr, query: Ptr, collector: Ptr) -> u32 {
    if tree.addr() == 0 {
        return 0;
    }
    fn_00d1c560(e, tree, query, collector, Ptr::new(0), Ptr::new(0), 0)
}

// Translated from 00d1d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// AABB query of a 16-bit-index float tree into a raw buffer (cdecl): `00d1ce50` with only the
/// cursor and the maximum given; returns 0 for a null tree.
pub fn fn_00d1d540(e: &mut Engine, tree: Ptr, query: Ptr, cursor: Ptr, max_count: i32) -> u32 {
    if tree.addr() == 0 {
        return 0;
    }
    fn_00d1ce50(e, tree, query, Ptr::new(0), Ptr::new(0), cursor, max_count)
}

// Translated from 00d1d570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTreeQueryUtils<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::queryAabb` (Xbox PDB),
/// cdecl: `00d1d240` with only the collector given; returns 0 for a null tree.
pub fn hk_aabb_tree_query_utils_query_aabb_uint(
    e: &mut Engine,
    tree: Ptr,
    query: Ptr,
    collector: Ptr,
) -> u32 {
    if tree.addr() == 0 {
        return 0;
    }
    hk_aabb_tree_query_utils_query_aabb_internal_uint(
        e,
        tree,
        query,
        collector,
        Ptr::new(0),
        Ptr::new(0),
        0,
    )
}

/// The work stack of the tree-collision queries: a local array of 8-byte pairs with room for
/// 64 (its header, 12 bytes, then the storage at +0xC, capacity word `0x80000040` = storage not
/// owned), `depth` entries in use, and `capacity` the stack's own capacity variable. The
/// header's size word is kept at the capacity (or at 0 at the end), the way the exe leaves it.
struct PairStack {
    header: u32,
    depth: i32,
    capacity: i32,
}

impl PairStack {
    /// Sets up the header, makes room for 64 pairs (the capacity already is 64, so the
    /// reserve call is never made) and pushes the first pair.
    fn start(e: &mut Engine, header: u32, first: (u32, u32)) -> Self {
        e.mem.set_u32(header, header + 0xc);
        e.mem.set_u32(header + 4, 0);
        e.mem.set_u32(header + 8, 0x8000_0040);
        reserve_at_least(e, header, 0x40, 8);
        e.mem.set_u32(header + 4, 0x40);
        let data = e.mem.u32(header);
        e.mem.set_u32(data, first.0);
        e.mem.set_u32(data + 4, first.1);
        PairStack {
            header,
            depth: 1,
            capacity: 0x40,
        }
    }

    /// Pops the top pair; when fewer than 4 free slots remain the stack's capacity doubles
    /// (reserving storage when the array is smaller) and the size word is set to it.
    fn pop(&mut self, e: &mut Engine) -> (u32, u32) {
        let data = e.mem.u32(self.header);
        let at = data
            .wrapping_add((self.depth as u32).wrapping_mul(8))
            .wrapping_sub(8);
        let pair = (e.mem.u32(at), e.mem.u32(at + 4));
        self.depth -= 1;
        if self.depth > self.capacity.wrapping_sub(4) {
            self.capacity = self.capacity.wrapping_add(self.capacity);
            reserve_at_least(e, self.header, self.capacity, 8);
            e.mem.set_u32(self.header + 4, self.capacity as u32);
        }
        pair
    }

    fn push(&mut self, e: &mut Engine, first: u32, second: u32) {
        let data = e.mem.u32(self.header);
        let at = data.wrapping_add((self.depth as u32).wrapping_mul(8));
        e.mem.set_u32(at, first);
        e.mem.set_u32(at + 4, second);
        self.depth += 1;
    }

    /// Gives the storage back when the array owns it (the allocator's function at +0x10).
    fn finish(self, e: &mut Engine) {
        release_array(e, self.header, 8);
    }
}

/// The overlap test of two nodes of the wide kind: each one's lower corner against the
/// other's upper corner on the three low lanes.
fn wide_nodes_overlap(e: &Engine, node_a: u32, node_b: u32) -> bool {
    all_three_le(load_vec(e, node_b), load_vec(e, node_a + 0x10))
        && all_three_le(load_vec(e, node_a), load_vec(e, node_b + 0x10))
}

fn wide_is_leaf(e: &Engine, node: u32) -> bool {
    e.mem.u32(node + 0x28) & LEAF_WIDE != 0
}

// Translated from 00d1d5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTreeQueryUtils<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::queryTreeCollision`
/// (Xbox PDB), cdecl: walks two wide trees together with a work stack of node index pairs
/// starting at the two root indices (tree +0x14) and appends every pair of overlapping leaves
/// as (index in A, index in B) to the array `pairs` (8-byte pairs). A pair of the same inner
/// node walks its three child pairs, a pair of the same leaf is skipped. Returns the number of
/// pairs appended; 0 when either root is null.
pub fn hk_aabb_tree_query_utils_query_tree_collision_uint(
    e: &mut Engine,
    tree_a: Ptr,
    tree_b: Ptr,
    pairs: Ptr,
) -> u32 {
    let (tree_a, tree_b, pairs) = (tree_a.addr(), tree_b.addr(), pairs.addr());
    if e.mem.u32(tree_a + 0x10) == 0 || e.mem.u32(tree_b + 0x10) == 0 {
        return 0;
    }
    let mut count = 0u32;
    e.with_stack(0xc + 0x40 * 8, |e, block| {
        let first = (e.mem.u32(tree_a + 0x14), e.mem.u32(tree_b + 0x14));
        let mut stack = PairStack::start(e, block.addr(), first);
        while stack.depth != 0 {
            let (a, b) = stack.pop(e);
            let node_a = Kind::Wide.node_at(e, tree_a, a);
            let node_b = Kind::Wide.node_at(e, tree_b, b);
            let leaf_a = wide_is_leaf(e, node_a);
            let leaf_b = wide_is_leaf(e, node_b);
            if node_a == node_b {
                if !leaf_a {
                    let child0 = e.mem.u32(node_a + 0x20);
                    let child1 = e.mem.u32(node_a + 0x24);
                    stack.push(e, child0, child0);
                    stack.push(e, child0, child1);
                    stack.push(e, child1, child1);
                }
                continue;
            }
            if !wide_nodes_overlap(e, node_a, node_b) {
                continue;
            }
            match (leaf_a, leaf_b) {
                (true, true) => {
                    let slot = next_slot(e, pairs, 8);
                    if slot != 0 {
                        e.mem.set_u32(slot, a);
                        e.mem.set_u32(slot + 4, b);
                    }
                    let size = e.mem.u32(pairs + 4);
                    e.mem.set_u32(pairs + 4, size.wrapping_add(1));
                    count += 1;
                }
                (true, false) => {
                    let child0 = e.mem.u32(node_b + 0x20);
                    let child1 = e.mem.u32(node_b + 0x24);
                    stack.push(e, a, child0);
                    stack.push(e, a, child1);
                }
                (false, true) => {
                    let child0 = e.mem.u32(node_a + 0x20);
                    let child1 = e.mem.u32(node_a + 0x24);
                    stack.push(e, child0, b);
                    stack.push(e, child1, b);
                }
                (false, false) => {
                    let a0 = e.mem.u32(node_a + 0x20);
                    let a1 = e.mem.u32(node_a + 0x24);
                    let b0 = e.mem.u32(node_b + 0x20);
                    let b1 = e.mem.u32(node_b + 0x24);
                    stack.push(e, a0, b0);
                    stack.push(e, a0, b1);
                    stack.push(e, a1, b0);
                    stack.push(e, a1, b1);
                }
            }
        }
        stack.finish(e);
    });
    count
}

// Translated from 00d1d890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbTreeQueryUtils<hkAabbTreeData::hkAabbTreeAabb_unsigned_int>::queryTreeCollision_ov2`
/// (Xbox PDB), cdecl: the same walk of two wide trees as `00d1d5a0` but with node pointers on
/// the work stack (starting at the two root nodes, tree +0x10); every pair of overlapping
/// leaves is handed to the collector's function at +4 (its low byte true ends the query).
/// Always returns 0 (also when either root is null).
pub fn hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(
    e: &mut Engine,
    tree_a: Ptr,
    tree_b: Ptr,
    collector: Ptr,
) -> u32 {
    let (tree_a, tree_b, collector) = (tree_a.addr(), tree_b.addr(), collector.addr());
    let root_a = e.mem.u32(tree_a + 0x10);
    let root_b = e.mem.u32(tree_b + 0x10);
    if root_a == 0 || root_b == 0 {
        return 0;
    }
    e.with_stack(0xc + 0x40 * 8, |e, block| {
        let mut stack = PairStack::start(e, block.addr(), (root_a, root_b));
        while stack.depth != 0 {
            let (node_a, node_b) = stack.pop(e);
            let leaf_a = wide_is_leaf(e, node_a);
            let leaf_b = wide_is_leaf(e, node_b);
            if node_a == node_b {
                if !leaf_a {
                    let index0 = e.mem.u32(node_a + 0x20);
                    let child0 = Kind::Wide.node_at(e, tree_a, index0);
                    let index1 = e.mem.u32(node_a + 0x24);
                    let child1 = Kind::Wide.node_at(e, tree_a, index1);
                    stack.push(e, child0, child0);
                    stack.push(e, child0, child1);
                    stack.push(e, child1, child1);
                }
                continue;
            }
            if !wide_nodes_overlap(e, node_a, node_b) {
                continue;
            }
            match (leaf_a, leaf_b) {
                (true, true) => {
                    if e.vcall(collector, 4, &args![node_a, node_b]).bool() {
                        break;
                    }
                }
                (true, false) => {
                    let index0 = e.mem.u32(node_b + 0x20);
                    let child0 = Kind::Wide.node_at(e, tree_b, index0);
                    let index1 = e.mem.u32(node_b + 0x24);
                    let child1 = Kind::Wide.node_at(e, tree_b, index1);
                    stack.push(e, node_a, child0);
                    stack.push(e, node_a, child1);
                }
                (false, true) => {
                    let index0 = e.mem.u32(node_a + 0x20);
                    let child0 = Kind::Wide.node_at(e, tree_a, index0);
                    let index1 = e.mem.u32(node_a + 0x24);
                    let child1 = Kind::Wide.node_at(e, tree_a, index1);
                    stack.push(e, child0, node_b);
                    stack.push(e, child1, node_b);
                }
                (false, false) => {
                    let index_a0 = e.mem.u32(node_a + 0x20);
                    let a0 = Kind::Wide.node_at(e, tree_a, index_a0);
                    let index_a1 = e.mem.u32(node_a + 0x24);
                    let a1 = Kind::Wide.node_at(e, tree_a, index_a1);
                    let index_b0 = e.mem.u32(node_b + 0x20);
                    let b0 = Kind::Wide.node_at(e, tree_b, index_b0);
                    let index_b1 = e.mem.u32(node_b + 0x24);
                    let b1 = Kind::Wide.node_at(e, tree_b, index_b1);
                    stack.push(e, a0, b0);
                    stack.push(e, a0, b1);
                    stack.push(e, a1, b0);
                    stack.push(e, a1, b1);
                }
            }
        }
        stack.finish(e);
    });
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_bcb0, hk_aabb_tree_query_utils_pair_collector_add_hit(Ptr, u32, u32) -> bool),
        entry!(0x00d1_bd00, fn_00d1bd00(Ptr, Ptr, Ptr, Ptr, f32)),
        entry!(0x00d1_bfa0, fn_00d1bfa0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d1_c2b0, fn_00d1c2b0(Ptr, u32, Ptr, Ptr, f32, Ptr, i32) -> u32),
        entry!(0x00d1_c560, fn_00d1c560(Ptr, Ptr, Ptr, Ptr, Ptr, i32) -> u32),
        entry!(0x00d1_ca00, fn_00d1ca00(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d1_cc50, fn_00d1cc50(Ptr, u32, Ptr, Ptr, f32, Ptr, i32) -> u32),
        entry!(0x00d1_ce50, fn_00d1ce50(Ptr, Ptr, Ptr, Ptr, Ptr, i32) -> u32),
        entry!(0x00d1_d0b0, fn_00d1d0b0(Ptr, Ptr, Ptr, Ptr, f32)),
        entry!(0x00d1_d240, hk_aabb_tree_query_utils_query_aabb_internal_uint(Ptr, Ptr, Ptr, Ptr, Ptr, i32) -> u32),
        entry!(0x00d1_d490, fn_00d1d490(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d1_d4a0, fn_00d1d4a0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d1_d4d0, fn_00d1d4d0(Ptr, Ptr, Ptr, i32) -> u32),
        entry!(0x00d1_d500, fn_00d1d500(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d1_d530, fn_00d1d530(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d1_d540, fn_00d1d540(Ptr, Ptr, Ptr, i32) -> u32),
        entry!(0x00d1_d570, hk_aabb_tree_query_utils_query_aabb_uint(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d1_d5a0, hk_aabb_tree_query_utils_query_tree_collision_uint(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d1_d890, hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(Ptr, Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Leaf = ([f32; 3], [f32; 3], u32);
    type QueryFn = fn(&mut Engine, Ptr, Ptr, Ptr, Ptr, Ptr, i32) -> u32;
    type RayFn = fn(&mut Engine, Ptr, Ptr, Ptr, Ptr, f32);
    type PaddedRayFn = fn(&mut Engine, Ptr, u32, Ptr, Ptr, f32, Ptr, i32) -> u32;
    type FourRayFn = fn(&mut Engine, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr);

    /// The exe's memory router and `_reserve` (the doubles' addresses), the allocator object's
    /// function that frees an array, and the collector's function at vtable slot 4.
    const MEMORY_ROUTER: u32 = 0x00c8_5750;
    const ARRAY_RESERVE: u32 = 0x00c9_08c0;
    const FREE_ARRAY: u32 = 0x00cc_0010;
    const HIT_FUNCTION: u32 = 0x00cc_0204;
    /// What the exe's big vector holds in the doubles' image.
    const BIG: f32 = 1.0e30;

    enum Shape {
        Inner {
            children: [u32; 2],
            next: Option<u32>,
        },
        Leaf {
            data: u32,
            next: Option<u32>,
        },
    }

    struct Setup {
        e: Engine,
        log: Log,
        /// The values the collector's function answers with, by call number (the last one
        /// repeats; none = 0.0).
        answers: Rc<RefCell<Vec<f64>>>,
        alloc: u32,
    }

    fn p(address: u32) -> Ptr {
        Ptr::new(address)
    }

    /// What the `_reserve` double does: a new storage of `capacity` elements that takes over the
    /// array's elements; the capacity word becomes `capacity`.
    fn grow_array(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let bytes = capacity * element;
        let data = e.mem.alloc(bytes.max(4));
        let old = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        if old != 0 {
            for at in 0..size * element {
                let byte = e.mem.u8(old + at);
                e.mem.set_u8(data + at, byte);
            }
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
    }

    fn setup() -> Setup {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(FLOAT_ZERO, 0.0);
        e.mem.set_f32(FLOAT_ONE, 1.0);
        for lane in 0..4u32 {
            e.mem.set_f32(ZERO_VECTOR + 4 * lane, 0.0);
            e.mem.set_f32(ONES_VECTOR + 4 * lane, 1.0);
            e.mem.set_f32(BIG_VECTOR + 4 * lane, BIG);
            e.mem.set_u32(RAY_BITS + 4 * lane, 1 << lane);
        }
        for set in 0..16u32 {
            for lane in 0..4u32 {
                let word = if (set >> lane) & 1 != 0 {
                    0xffff_ffff
                } else {
                    0
                };
                e.mem.set_u32(ACTIVE_RAY_LANES + 16 * set + 4 * lane, word);
            }
        }
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x20);
        let alloc = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(alloc, vtable);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY);
        e.mem.set_u32(router + 0x10, alloc);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_ARRAY, move |_, a| {
            free_log.borrow_mut().push((FREE_ARRAY, a.to_vec()));
            Ret::default()
        });
        let reserve_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            reserve_log.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            grow_array(e, a[1], a[2], a[3]);
            Ret::default()
        });
        let more_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            more_log.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let capacity = ((e.mem.u32(a[1] + 8) & CAPACITY_MASK) * 2).max(4);
            grow_array(e, a[1], capacity, a[2]);
            Ret::default()
        });
        for (address, stride, narrow_index) in [
            (NODE_AT_NARROW, 0x14, true),
            (NODE_AT_FLOAT16, 0x30, true),
            (NODE_AT_WIDE, 0x30, false),
        ] {
            let node_log = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                node_log.borrow_mut().push((address, a.to_vec()));
                let index = if narrow_index {
                    u32::from(a[1] as u16)
                } else {
                    a[1]
                };
                Ret {
                    eax: e.mem.u32(a[0]).wrapping_add(index.wrapping_mul(stride)),
                    ..Ret::default()
                }
            });
        }
        let answers: Rc<RefCell<Vec<f64>>> = Rc::default();
        let hit_log = Rc::clone(&log);
        let hit_answers = Rc::clone(&answers);
        e.register_double(HIT_FUNCTION, move |_, a| {
            let call = hit_log
                .borrow()
                .iter()
                .filter(|(at, _)| *at == HIT_FUNCTION)
                .count();
            hit_log.borrow_mut().push((HIT_FUNCTION, a.to_vec()));
            let list = hit_answers.borrow();
            let answer = list.get(call).or(list.last()).copied().unwrap_or(0.0);
            Ret {
                eax: u32::from(answer == 1.0),
                st0: answer,
                ..Ret::default()
            }
        });
        Setup {
            e,
            log,
            answers,
            alloc,
        }
    }

    /// The argument lists of every logged call to `address`.
    fn calls_to(s: &Setup, address: u32) -> Vec<Vec<u32>> {
        s.log
            .borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn node_calls(s: &Setup) -> usize {
        [NODE_AT_NARROW, NODE_AT_FLOAT16, NODE_AT_WIDE]
            .into_iter()
            .map(|address| calls_to(s, address).len())
            .sum()
    }

    fn stride(kind: Kind) -> u32 {
        match kind {
            Kind::Narrow => 0x14,
            Kind::Float16 | Kind::Wide => 0x30,
        }
    }

    fn node_of(e: &Engine, tree: u32, kind: Kind, index: u32) -> u32 {
        e.mem.u32(tree + 0x1c) + stride(kind) * index
    }

    fn vector(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let at = e.mem.alloc(16);
        for (lane, value) in lanes.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
        at
    }

    /// A query box: lower corner at +0, upper at +0x10.
    fn query_box(e: &mut Engine, min: [f32; 3], max: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0x20);
        for (lane, (low, high)) in min.iter().zip(max.iter()).enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *low);
            e.mem.set_f32(at + 0x10 + 4 * lane as u32, *high);
        }
        at
    }

    fn set_bounds(e: &mut Engine, kind: Kind, node: u32, min: [f32; 3], max: [f32; 3]) {
        for (lane, (low, high)) in min.iter().zip(max.iter()).enumerate() {
            let lane = lane as u32;
            match kind {
                Kind::Narrow => {
                    e.mem.set_u16(node + 2 * lane, *high as u16);
                    e.mem.set_u16(node + 6 + 2 * lane, *low as u16);
                }
                Kind::Float16 | Kind::Wide => {
                    e.mem.set_f32(node + 4 * lane, *low);
                    e.mem.set_f32(node + 0x10 + 4 * lane, *high);
                }
            }
        }
    }

    fn write_node(
        e: &mut Engine,
        kind: Kind,
        node: u32,
        min: [f32; 3],
        max: [f32; 3],
        shape: Shape,
    ) {
        set_bounds(e, kind, node, min, max);
        let (next, leaf) = match shape {
            Shape::Inner { next, .. } => (next, false),
            Shape::Leaf { next, .. } => (next, true),
        };
        match kind {
            Kind::Narrow => {
                e.mem
                    .set_u16(node + 0x12, next.map_or(0x7fff, |n| n as u16));
                e.mem.set_u16(node + 0x10, if leaf { 0x8000 } else { 0 });
                match shape {
                    Shape::Inner { children, .. } => {
                        e.mem.set_u16(node + 0xc, children[0] as u16);
                        e.mem.set_u16(node + 0xe, children[1] as u16);
                    }
                    Shape::Leaf { data, .. } => e.mem.set_u32(node + 0xc, data),
                }
            }
            Kind::Float16 => {
                e.mem
                    .set_u16(node + 0x26, next.map_or(0x7fff, |n| n as u16));
                e.mem.set_u16(node + 0x24, if leaf { 0x8000 } else { 0 });
                match shape {
                    Shape::Inner { children, .. } => {
                        e.mem.set_u16(node + 0x20, children[0] as u16);
                        e.mem.set_u16(node + 0x22, children[1] as u16);
                    }
                    Shape::Leaf { data, .. } => e.mem.set_u32(node + 0x20, data),
                }
            }
            Kind::Wide => {
                e.mem.set_u32(node + 0x2c, next.unwrap_or(0x7fff_ffff));
                e.mem
                    .set_u32(node + 0x28, if leaf { 0x8000_0000 } else { 0 });
                match shape {
                    Shape::Inner { children, .. } => {
                        e.mem.set_u32(node + 0x20, children[0]);
                        e.mem.set_u32(node + 0x24, children[1]);
                    }
                    Shape::Leaf { data, .. } => e.mem.set_u32(node + 0x20, data),
                }
            }
        }
    }

    /// An empty tree of `nodes` nodes of `kind`: node storage in the manager (+0x1c), the root
    /// node (+0x10) and root index (+0x14) pointing at node 0; the narrow kind gets the scale
    /// 2^-15 (so a grid word is its own coordinate) and a zero offset.
    fn new_tree(e: &mut Engine, kind: Kind, nodes: u32) -> u32 {
        let tree = e.mem.alloc(0x80);
        let storage = e.mem.alloc(stride(kind) * nodes);
        e.mem.set_u32(tree + 0x1c, storage);
        e.mem.set_u32(tree + 0x10, storage);
        e.mem.set_u32(tree + 0x14, 0);
        for lane in 0..4u32 {
            e.mem.set_f32(tree + 0x70 + 4 * lane, 1.0 / 32768.0);
        }
        tree
    }

    /// A root with two leaves (nodes 1 and 2, in that traversal order) whose bounds the root
    /// encloses.
    fn three_node_tree(e: &mut Engine, kind: Kind, leaves: [Leaf; 2]) -> u32 {
        let tree = new_tree(e, kind, 3);
        let min: [f32; 3] = std::array::from_fn(|lane| leaves[0].0[lane].min(leaves[1].0[lane]));
        let max: [f32; 3] = std::array::from_fn(|lane| leaves[0].1[lane].max(leaves[1].1[lane]));
        let root = node_of(e, tree, kind, 0);
        let shape = Shape::Inner {
            children: [1, 2],
            next: None,
        };
        write_node(e, kind, root, min, max, shape);
        for (index, leaf) in leaves.iter().enumerate() {
            let node = node_of(e, tree, kind, 1 + index as u32);
            let next = if index == 0 { Some(2) } else { None };
            let shape = Shape::Leaf { data: leaf.2, next };
            write_node(e, kind, node, leaf.0, leaf.1, shape);
        }
        tree
    }

    fn single_leaf_tree(
        e: &mut Engine,
        kind: Kind,
        min: [f32; 3],
        max: [f32; 3],
        data: u32,
    ) -> u32 {
        let tree = new_tree(e, kind, 1);
        let root = node_of(e, tree, kind, 0);
        write_node(e, kind, root, min, max, Shape::Leaf { data, next: None });
        tree
    }

    fn new_collector(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, HIT_FUNCTION);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    /// An array header (data, size, capacity) over fresh storage for `capacity` words.
    fn new_array(e: &mut Engine, capacity: u32, element: u32) -> u32 {
        let header = e.mem.alloc(12);
        let data = e.mem.alloc(capacity * element);
        e.mem.set_u32(header, data);
        e.mem.set_u32(header + 8, capacity);
        header
    }

    fn array_words(e: &Engine, header: u32) -> Vec<u32> {
        let data = e.mem.u32(header);
        (0..e.mem.u32(header + 4) * 2)
            .map(|word| e.mem.u32(data + 4 * word))
            .collect()
    }

    const LEAVES: [Leaf; 2] = [([0.0; 3], [2.0; 3], 111), ([10.0; 3], [12.0; 3], 222)];
    /// Leaves a ray along x at y = z = 1 passes through both of (the second is shifted in x only).
    const RAY_LEAVES: [Leaf; 2] = [
        ([0.0; 3], [2.0; 3], 111),
        ([10.0, 0.0, 0.0], [12.0, 2.0, 2.0], 222),
    ];

    // ------------------------------------------------------------------ pair collector

    #[test]
    fn add_hit_appends_the_pair_and_returns_false() {
        let mut s = setup();
        let array = new_array(&mut s.e, 4, 8);
        let this = s.e.mem.alloc(8);
        s.e.mem.set_u32(this + 4, array);
        let added = hk_aabb_tree_query_utils_pair_collector_add_hit(&mut s.e, p(this), 0x11, 0x22);
        assert!(!added);
        assert_eq!(s.e.mem.u32(array + 4), 1);
        assert_eq!(array_words(&s.e, array), vec![0x11, 0x22]);
        assert!(calls_to(&s, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn add_hit_makes_room_when_the_array_is_full() {
        let mut s = setup();
        let array = new_array(&mut s.e, 1, 8);
        let this = s.e.mem.alloc(8);
        s.e.mem.set_u32(this + 4, array);
        hk_aabb_tree_query_utils_pair_collector_add_hit(&mut s.e, p(this), 1, 2);
        hk_aabb_tree_query_utils_pair_collector_add_hit(&mut s.e, p(this), 3, 4);
        assert_eq!(array_words(&s.e, array), vec![1, 2, 3, 4]);
        assert_eq!(
            calls_to(&s, ARRAY_RESERVE_MORE),
            vec![vec![s.alloc, array, 8]]
        );
    }

    // ------------------------------------------------------------------ AABB queries

    fn check_query_modes(kind: Kind, query: QueryFn) {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, kind, LEAVES);
        let leaf1 = node_of(&s.e, tree, kind, 1);
        let leaf2 = node_of(&s.e, tree, kind, 2);
        let both = query_box(&mut s.e, [1.0; 3], [11.0; 3]);
        let first = query_box(&mut s.e, [1.0; 3], [3.0; 3]);
        let away = query_box(&mut s.e, [20.0; 3], [30.0; 3]);
        let collector = new_collector(&mut s.e);
        let null = p(0);

        // None of the three outputs: nothing is visited.
        assert_eq!(query(&mut s.e, p(tree), p(both), null, null, null, 9), 0);
        assert_eq!(node_calls(&s), 0);

        // A box that misses the root: nothing is visited below the root.
        s.answers.replace(vec![1.0]);
        assert_eq!(
            query(&mut s.e, p(tree), p(away), p(collector), null, null, 0),
            0
        );
        assert!(calls_to(&s, HIT_FUNCTION).is_empty());

        // Collector: every leaf with an answer other than 0.0 counts.
        s.answers.replace(vec![1.0, 1.0]);
        assert_eq!(
            query(&mut s.e, p(tree), p(both), p(collector), null, null, 0),
            2
        );
        assert_eq!(
            calls_to(&s, HIT_FUNCTION),
            vec![vec![collector, leaf1], vec![collector, leaf2]]
        );

        // An answer of exactly 0.0 ends the query before the leaf is counted.
        s.log.borrow_mut().clear();
        s.answers.replace(vec![1.0, 0.0]);
        assert_eq!(
            query(&mut s.e, p(tree), p(both), p(collector), null, null, 0),
            1
        );
        assert_eq!(calls_to(&s, HIT_FUNCTION).len(), 2);
        s.answers.replace(vec![0.0]);
        assert_eq!(
            query(&mut s.e, p(tree), p(first), p(collector), null, null, 0),
            0
        );

        // A NaN answer is not equal to 0.0: the leaf counts.
        s.answers.replace(vec![f64::NAN]);
        assert_eq!(
            query(&mut s.e, p(tree), p(first), p(collector), null, null, 0),
            1
        );

        // The collector wins over the cursor and the array.
        let buffer = s.e.mem.alloc(16);
        let cursor = s.e.mem.alloc(4);
        s.e.mem.set_u32(cursor, buffer);
        let array = new_array(&mut s.e, 4, 4);
        s.answers.replace(vec![1.0]);
        assert_eq!(
            query(
                &mut s.e,
                p(tree),
                p(first),
                p(collector),
                p(array),
                p(cursor),
                9
            ),
            1
        );
        assert_eq!(s.e.mem.u32(cursor), buffer);
        assert_eq!(s.e.mem.u32(array + 4), 0);

        // Cursor: the leaf data is stored and the pointer advances; the count must stay below
        // the maximum before a leaf is counted, so the leaf that reaches it is stored but not
        // counted.
        assert_eq!(
            query(&mut s.e, p(tree), p(both), null, null, p(cursor), 1),
            1
        );
        assert_eq!(s.e.mem.u32(buffer), 111);
        assert_eq!(s.e.mem.u32(buffer + 4), 222);
        assert_eq!(s.e.mem.u32(cursor), buffer + 8);
        s.e.mem.set_u32(cursor, buffer);
        assert_eq!(
            query(&mut s.e, p(tree), p(both), null, p(array), p(cursor), 5),
            2
        );
        assert_eq!(s.e.mem.u32(cursor), buffer + 8);
        assert_eq!(s.e.mem.u32(array + 4), 0);

        // Array: the leaf data is appended; a full array is made larger first.
        let small = new_array(&mut s.e, 1, 4);
        s.log.borrow_mut().clear();
        assert_eq!(
            query(&mut s.e, p(tree), p(both), null, p(small), null, 0),
            2
        );
        assert_eq!(s.e.mem.u32(small + 4), 2);
        let data = s.e.mem.u32(small);
        assert_eq!((s.e.mem.u32(data), s.e.mem.u32(data + 4)), (111, 222));
        assert_eq!(
            calls_to(&s, ARRAY_RESERVE_MORE),
            vec![vec![s.alloc, small, 4]]
        );

        // A tree without a root has nothing to visit.
        s.e.mem.set_u32(tree + 0x10, 0);
        s.log.borrow_mut().clear();
        assert_eq!(
            query(&mut s.e, p(tree), p(both), p(collector), null, null, 0),
            0
        );
        assert_eq!(node_calls(&s), 0);
    }

    #[test]
    fn query_aabb_narrow_modes() {
        check_query_modes(Kind::Narrow, fn_00d1c560);
    }

    #[test]
    fn query_aabb_narrow_applies_scale_and_offset() {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, Kind::Narrow, LEAVES);
        s.e.mem.set_f32(tree + 0x40, 100.0);
        // World x of the leaves: [-100, -98] and [-90, -88].
        let query = query_box(&mut s.e, [-99.0, 1.0, 1.0], [-98.5, 3.0, 3.0]);
        let buffer = s.e.mem.alloc(16);
        let cursor = s.e.mem.alloc(4);
        s.e.mem.set_u32(cursor, buffer);
        let count = fn_00d1c560(&mut s.e, p(tree), p(query), p(0), p(0), p(cursor), 9);
        assert_eq!(count, 1);
        assert_eq!(s.e.mem.u32(buffer), 111);
        assert_eq!(s.e.mem.u32(cursor), buffer + 4);
    }

    #[test]
    fn query_aabb_float16_modes() {
        check_query_modes(Kind::Float16, fn_00d1ce50);
    }

    #[test]
    fn query_aabb_wide_modes() {
        check_query_modes(
            Kind::Wide,
            hk_aabb_tree_query_utils_query_aabb_internal_uint,
        );
    }

    #[test]
    fn query_aabb_into_array_wrapper() {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, Kind::Narrow, LEAVES);
        let both = query_box(&mut s.e, [1.0; 3], [11.0; 3]);
        let array = new_array(&mut s.e, 4, 4);
        assert_eq!(fn_00d1d4a0(&mut s.e, p(tree), p(both), p(array)), 2);
        assert_eq!(s.e.mem.u32(array + 4), 2);
        let data = s.e.mem.u32(array);
        assert_eq!((s.e.mem.u32(data), s.e.mem.u32(data + 4)), (111, 222));
        assert_eq!(fn_00d1d4a0(&mut s.e, p(0), p(both), p(array)), 0);
    }

    #[test]
    fn query_aabb_into_buffer_wrapper() {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, Kind::Narrow, LEAVES);
        let both = query_box(&mut s.e, [1.0; 3], [11.0; 3]);
        let buffer = s.e.mem.alloc(16);
        let cursor = s.e.mem.alloc(4);
        s.e.mem.set_u32(cursor, buffer);
        assert_eq!(fn_00d1d4d0(&mut s.e, p(tree), p(both), p(cursor), 5), 2);
        assert_eq!((s.e.mem.u32(buffer), s.e.mem.u32(buffer + 4)), (111, 222));
        assert_eq!(fn_00d1d4d0(&mut s.e, p(0), p(both), p(cursor), 5), 0);
    }

    #[test]
    fn query_aabb_with_collector_wrapper() {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, Kind::Narrow, LEAVES);
        let both = query_box(&mut s.e, [1.0; 3], [11.0; 3]);
        let collector = new_collector(&mut s.e);
        s.answers.replace(vec![1.0]);
        assert_eq!(fn_00d1d500(&mut s.e, p(tree), p(both), p(collector)), 2);
        assert_eq!(calls_to(&s, HIT_FUNCTION).len(), 2);
        assert_eq!(fn_00d1d500(&mut s.e, p(0), p(both), p(collector)), 0);
    }

    #[test]
    fn query_aabb_float16_into_buffer_wrapper() {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, Kind::Float16, LEAVES);
        let both = query_box(&mut s.e, [1.0; 3], [11.0; 3]);
        let buffer = s.e.mem.alloc(16);
        let cursor = s.e.mem.alloc(4);
        s.e.mem.set_u32(cursor, buffer);
        assert_eq!(fn_00d1d540(&mut s.e, p(tree), p(both), p(cursor), 5), 2);
        assert_eq!((s.e.mem.u32(buffer), s.e.mem.u32(buffer + 4)), (111, 222));
        assert_eq!(fn_00d1d540(&mut s.e, p(0), p(both), p(cursor), 5), 0);
    }

    #[test]
    fn query_aabb_wide_with_collector_wrapper() {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, Kind::Wide, LEAVES);
        let both = query_box(&mut s.e, [1.0; 3], [11.0; 3]);
        let collector = new_collector(&mut s.e);
        s.answers.replace(vec![1.0]);
        let count =
            hk_aabb_tree_query_utils_query_aabb_uint(&mut s.e, p(tree), p(both), p(collector));
        assert_eq!(count, 2);
        let count = hk_aabb_tree_query_utils_query_aabb_uint(&mut s.e, p(0), p(both), p(collector));
        assert_eq!(count, 0);
    }

    // ------------------------------------------------------------------ rays

    #[test]
    fn inverse_direction_replaces_zero_lanes() {
        let mut s = setup();
        let origin = [0.0; 4];
        let target = [4.0, 0.0, -8.0, 0.0];
        let inverse = ray_inverse_direction(&s.e, origin, target);
        assert!((inverse[0] - 0.25).abs() < 1.0e-5);
        assert!((inverse[2] + 0.125).abs() < 1.0e-5);
        assert_eq!((inverse[1], inverse[3]), (BIG, BIG));
        // The same lanes, with an exactly-zero lane left alone when it is not equal to zero.
        s.e.mem.set_f32(ZERO_VECTOR, 1.0);
        let inverse = ray_inverse_direction(&s.e, origin, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(inverse[0], BIG);
    }

    fn ray_vectors(e: &mut Engine, y: f32) -> (u32, u32) {
        (
            vector(e, [-5.0, y, 1.0, 0.0]),
            vector(e, [20.0, y, 1.0, 0.0]),
        )
    }

    fn check_single_ray(kind: Kind, cast: RayFn) {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, kind, RAY_LEAVES);
        let leaf1 = node_of(&s.e, tree, kind, 1);
        let leaf2 = node_of(&s.e, tree, kind, 2);
        let (origin, target) = ray_vectors(&mut s.e, 1.0);
        let (origin_high, target_high) = ray_vectors(&mut s.e, 5.0);
        let collector = new_collector(&mut s.e);

        // The first answer shortens the ray so that the second leaf lies beyond it.
        s.answers.replace(vec![0.5]);
        cast(&mut s.e, p(tree), p(origin), p(target), p(collector), 1.0);
        assert_eq!(calls_to(&s, HIT_FUNCTION), vec![vec![collector, leaf1]]);

        // A longer answer lets the ray reach both leaves.
        s.log.borrow_mut().clear();
        s.answers.replace(vec![2.0]);
        cast(&mut s.e, p(tree), p(origin), p(target), p(collector), 1.0);
        assert_eq!(
            calls_to(&s, HIT_FUNCTION),
            vec![vec![collector, leaf1], vec![collector, leaf2]]
        );

        // A ray that passes beside the leaves, and one that is too short.
        s.log.borrow_mut().clear();
        cast(
            &mut s.e,
            p(tree),
            p(origin_high),
            p(target_high),
            p(collector),
            1.0,
        );
        cast(&mut s.e, p(tree), p(origin), p(target), p(collector), 0.1);
        assert!(calls_to(&s, HIT_FUNCTION).is_empty());

        // No tree, or no root.
        cast(&mut s.e, p(0), p(origin), p(target), p(collector), 1.0);
        s.e.mem.set_u32(tree + 0x10, 0);
        cast(&mut s.e, p(tree), p(origin), p(target), p(collector), 1.0);
        assert!(calls_to(&s, HIT_FUNCTION).is_empty());
    }

    #[test]
    fn cast_ray_narrow() {
        check_single_ray(Kind::Narrow, fn_00d1bd00);
    }

    #[test]
    fn cast_ray_wide() {
        check_single_ray(Kind::Wide, fn_00d1d0b0);
    }

    fn padded_ray_block(e: &mut Engine, y: f32, extra: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x30);
        for (lane, value) in [-5.0, y, 1.0, 0.0].iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
        for (lane, value) in [20.0, y, 1.0, 0.0].iter().enumerate() {
            e.mem.set_f32(at + 0x10 + 4 * lane as u32, *value);
        }
        for (lane, value) in extra.iter().enumerate() {
            e.mem.set_f32(at + 0x20 + 4 * lane as u32, *value);
        }
        at
    }

    fn check_padded_ray(kind: Kind, cast: PaddedRayFn) {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, kind, RAY_LEAVES);
        let ray = padded_ray_block(&mut s.e, 1.0, [0.0; 4]);
        let beside = padded_ray_block(&mut s.e, 5.0, [0.0; 4]);
        let beside_padded = padded_ray_block(&mut s.e, 5.0, [0.0, 3.5, 0.0, 0.0]);
        let collector = new_collector(&mut s.e);
        let null = p(0);

        // Neither a collector nor a cursor: the leaves are only counted.
        assert_eq!(cast(&mut s.e, p(tree), 0, p(ray), null, 0.0, null, 0), 2);
        assert!(calls_to(&s, HIT_FUNCTION).is_empty());

        // A collector: its answer is the new limit, here too short to reach the second leaf.
        s.answers.replace(vec![0.5]);
        assert_eq!(
            cast(&mut s.e, p(tree), 0, p(ray), p(collector), 0.0, null, 0),
            1
        );
        assert_eq!(calls_to(&s, HIT_FUNCTION).len(), 1);

        // The padding widens the boxes: the ray beside the leaves misses without it, hits
        // both with the scalar padding or the padding vector.
        assert_eq!(cast(&mut s.e, p(tree), 0, p(beside), null, 0.0, null, 0), 0);
        assert_eq!(cast(&mut s.e, p(tree), 0, p(beside), null, 3.5, null, 0), 2);
        assert_eq!(
            cast(&mut s.e, p(tree), 0, p(beside_padded), null, 0.0, null, 0),
            2
        );

        // A cursor: leaf data is stored and the cursor advances (by value, so the caller's
        // pointer stays); the leaf that reaches the maximum is stored but not counted.
        let buffer = s.e.mem.alloc(16);
        assert_eq!(
            cast(&mut s.e, p(tree), 0, p(ray), null, 0.0, p(buffer), 1),
            1
        );
        assert_eq!((s.e.mem.u32(buffer), s.e.mem.u32(buffer + 4)), (111, 222));
        assert_eq!(
            cast(&mut s.e, p(tree), 0, p(ray), null, 0.0, p(buffer), 5),
            2
        );

        // No tree, or no root.
        assert_eq!(cast(&mut s.e, null, 0, p(ray), null, 0.0, null, 0), 0);
        s.e.mem.set_u32(tree + 0x10, 0);
        assert_eq!(cast(&mut s.e, p(tree), 0, p(ray), null, 0.0, null, 0), 0);
    }

    #[test]
    fn cast_padded_ray_narrow() {
        check_padded_ray(Kind::Narrow, fn_00d1c2b0);
    }

    #[test]
    fn cast_padded_ray_float16() {
        check_padded_ray(Kind::Float16, fn_00d1cc50);
    }

    /// Four rays along +x at the heights `y`, active as the set `active`.
    fn four_rays(e: &mut Engine, y: [f32; 4], active: u32) -> u32 {
        let at = e.mem.alloc(0x70);
        let lanes = [([-5.0; 4], [0.04; 4]), (y, [BIG; 4]), ([1.0; 4], [BIG; 4])];
        for (axis, (origin, inverse)) in lanes.iter().enumerate() {
            for (lane, (o, inv)) in origin.iter().zip(inverse.iter()).enumerate() {
                let slot = 16 * axis as u32 + 4 * lane as u32;
                e.mem.set_f32(at + slot, *o);
                e.mem.set_f32(at + 0x30 + slot, *inv);
            }
        }
        e.mem.set_u32(at + 0x60, active);
        at
    }

    fn check_four_rays(kind: Kind, cast: FourRayFn) {
        let mut s = setup();
        let tree = three_node_tree(&mut s.e, kind, RAY_LEAVES);
        let root = node_of(&s.e, tree, kind, 0);
        let leaf1 = node_of(&s.e, tree, kind, 1);
        let leaf2 = node_of(&s.e, tree, kind, 2);
        let rays = four_rays(&mut s.e, [1.0, 1.0, 5.0, 1.0], 0b1111);
        let collectors = s.e.mem.alloc(16);
        let mut objects = [0u32; 4];
        for (lane, object) in objects.iter_mut().enumerate() {
            *object = new_collector(&mut s.e);
            s.e.mem.set_u32(collectors + 4 * lane as u32, *object);
        }
        let fractions = vector(&mut s.e, [1.0; 4]);

        // Rays 0, 1 and 3 pass through both leaves (ray 2 only through the root); each answer
        // raises that ray's local limit, the caller's fractions stay.
        s.answers.replace(vec![2.0]);
        cast(
            &mut s.e,
            p(tree),
            p(root),
            p(0),
            p(rays),
            p(collectors),
            p(fractions),
        );
        let expected: Vec<Vec<u32>> = [leaf1, leaf2]
            .iter()
            .flat_map(|leaf| [0usize, 1, 3].map(|lane| vec![objects[lane], *leaf]))
            .collect();
        assert_eq!(calls_to(&s, HIT_FUNCTION), expected);
        assert_eq!(load_vec(&s.e, fractions), [1.0; 4]);

        // A short answer keeps those rays from the second leaf.
        s.log.borrow_mut().clear();
        s.answers.replace(vec![0.5]);
        cast(
            &mut s.e,
            p(tree),
            p(root),
            p(0),
            p(rays),
            p(collectors),
            p(fractions),
        );
        assert_eq!(calls_to(&s, HIT_FUNCTION).len(), 3);

        // The traversal stops when it reaches the stop node.
        s.log.borrow_mut().clear();
        s.answers.replace(vec![2.0]);
        cast(
            &mut s.e,
            p(tree),
            p(leaf1),
            p(leaf2),
            p(rays),
            p(collectors),
            p(fractions),
        );
        assert_eq!(calls_to(&s, HIT_FUNCTION).len(), 3);

        // Only the active rays count.
        s.log.borrow_mut().clear();
        let first_only = four_rays(&mut s.e, [1.0, 1.0, 5.0, 1.0], 0b0001);
        cast(
            &mut s.e,
            p(tree),
            p(root),
            p(0),
            p(first_only),
            p(collectors),
            p(fractions),
        );
        assert_eq!(
            calls_to(&s, HIT_FUNCTION),
            vec![vec![objects[0], leaf1], vec![objects[0], leaf2]]
        );
        s.log.borrow_mut().clear();
        let third_only = four_rays(&mut s.e, [1.0, 1.0, 5.0, 1.0], 0b0100);
        cast(
            &mut s.e,
            p(tree),
            p(root),
            p(0),
            p(third_only),
            p(collectors),
            p(fractions),
        );
        assert!(calls_to(&s, HIT_FUNCTION).is_empty());
    }

    #[test]
    fn cast_four_rays_narrow() {
        check_four_rays(Kind::Narrow, fn_00d1bfa0);
    }

    #[test]
    fn cast_four_rays_narrow_through_the_thunk() {
        check_four_rays(Kind::Narrow, fn_00d1d490);
    }

    #[test]
    fn cast_four_rays_float16() {
        check_four_rays(Kind::Float16, fn_00d1ca00);
    }

    #[test]
    fn cast_four_rays_float16_through_the_thunk() {
        check_four_rays(Kind::Float16, fn_00d1d530);
    }

    // ------------------------------------------------------------------ tree against tree

    /// A chain of `depth` inner nodes (node i holds a leaf and the next inner node, the last
    /// holds two leaves) whose boxes all are the unit cube; leaf data is the node's index.
    fn chain_tree(e: &mut Engine, depth: u32) -> u32 {
        let tree = new_tree(e, Kind::Wide, 2 * depth + 1);
        let unit = [1.0; 3];
        for i in 0..depth {
            let children = if i + 1 < depth {
                [depth + i, i + 1]
            } else {
                [depth + i, 2 * depth]
            };
            let node = node_of(e, tree, Kind::Wide, i);
            write_node(
                e,
                Kind::Wide,
                node,
                [0.0; 3],
                unit,
                Shape::Inner {
                    children,
                    next: None,
                },
            );
        }
        for i in depth..=2 * depth {
            let node = node_of(e, tree, Kind::Wide, i);
            write_node(
                e,
                Kind::Wide,
                node,
                [0.0; 3],
                unit,
                Shape::Leaf {
                    data: i,
                    next: None,
                },
            );
        }
        tree
    }

    fn pairs_of(e: &Engine, header: u32) -> Vec<(u32, u32)> {
        let words = array_words(e, header);
        words.chunks(2).map(|pair| (pair[0], pair[1])).collect()
    }

    #[test]
    fn tree_collision_reports_overlapping_leaf_pairs() {
        let mut s = setup();
        // A: leaves at [0,2] and [10,12]; B: leaves at [1,3] and [20,22].
        let a = three_node_tree(&mut s.e, Kind::Wide, LEAVES);
        let b = three_node_tree(
            &mut s.e,
            Kind::Wide,
            [([1.0; 3], [3.0; 3], 5), ([20.0; 3], [22.0; 3], 6)],
        );
        let pairs = new_array(&mut s.e, 4, 8);
        let count =
            hk_aabb_tree_query_utils_query_tree_collision_uint(&mut s.e, p(a), p(b), p(pairs));
        assert_eq!(count, 1);
        // Node indices: leaf 1 of A against leaf 1 of B.
        assert_eq!(pairs_of(&s.e, pairs), vec![(1, 1)]);
        // 64 pairs of room are enough: the stack never grew and its storage is not given back.
        assert!(calls_to(&s, ARRAY_RESERVE).is_empty());
        assert!(calls_to(&s, FREE_ARRAY).is_empty());
    }

    #[test]
    fn tree_collision_of_a_tree_with_itself_skips_same_node_pairs() {
        let mut s = setup();
        let a = three_node_tree(&mut s.e, Kind::Wide, LEAVES);
        let pairs = new_array(&mut s.e, 4, 8);
        // Root with itself walks the three child pairs; (leaf, same leaf) is skipped and the two
        // distinct leaves are disjoint.
        let count =
            hk_aabb_tree_query_utils_query_tree_collision_uint(&mut s.e, p(a), p(a), p(pairs));
        assert_eq!(count, 0);
        assert_eq!(s.e.mem.u32(pairs + 4), 0);
    }

    #[test]
    fn tree_collision_leaf_against_inner_and_inner_against_leaf() {
        let mut s = setup();
        let single = single_leaf_tree(&mut s.e, Kind::Wide, [1.0; 3], [11.0; 3], 9);
        let three = three_node_tree(&mut s.e, Kind::Wide, LEAVES);
        let pairs = new_array(&mut s.e, 4, 8);
        // A single leaf against the three-node tree: pushed (a, child 0) and (a, child 1); the
        // child pairs are popped last first.
        let count = hk_aabb_tree_query_utils_query_tree_collision_uint(
            &mut s.e,
            p(single),
            p(three),
            p(pairs),
        );
        assert_eq!(count, 2);
        assert_eq!(pairs_of(&s.e, pairs), vec![(0, 2), (0, 1)]);
        // And the other way round.
        let pairs = new_array(&mut s.e, 4, 8);
        let count = hk_aabb_tree_query_utils_query_tree_collision_uint(
            &mut s.e,
            p(three),
            p(single),
            p(pairs),
        );
        assert_eq!(count, 2);
        assert_eq!(pairs_of(&s.e, pairs), vec![(2, 0), (1, 0)]);
        // Two single leaves.
        let other = single_leaf_tree(&mut s.e, Kind::Wide, [5.0; 3], [6.0; 3], 8);
        let pairs = new_array(&mut s.e, 4, 8);
        let count = hk_aabb_tree_query_utils_query_tree_collision_uint(
            &mut s.e,
            p(single),
            p(other),
            p(pairs),
        );
        assert_eq!((count, pairs_of(&s.e, pairs)), (1, vec![(0, 0)]));
    }

    #[test]
    fn tree_collision_needs_both_roots() {
        let mut s = setup();
        let a = three_node_tree(&mut s.e, Kind::Wide, LEAVES);
        let b = three_node_tree(&mut s.e, Kind::Wide, LEAVES);
        let pairs = new_array(&mut s.e, 4, 8);
        s.e.mem.set_u32(b + 0x10, 0);
        assert_eq!(
            hk_aabb_tree_query_utils_query_tree_collision_uint(&mut s.e, p(a), p(b), p(pairs)),
            0
        );
        assert_eq!(
            hk_aabb_tree_query_utils_query_tree_collision_uint(&mut s.e, p(b), p(a), p(pairs)),
            0
        );
        assert_eq!(node_calls(&s), 0);
    }

    #[test]
    fn tree_collision_grows_its_work_stack_and_the_output() {
        let mut s = setup();
        let depth = 24;
        let a = chain_tree(&mut s.e, depth);
        let b = chain_tree(&mut s.e, depth);
        let pairs = new_array(&mut s.e, 2, 8);
        let count =
            hk_aabb_tree_query_utils_query_tree_collision_uint(&mut s.e, p(a), p(b), p(pairs));
        // 25 leaves in each chain, all overlapping.
        assert_eq!(count, 25 * 25);
        assert_eq!(s.e.mem.u32(pairs + 4), 25 * 25);
        // The stack's capacity doubled once: the storage was 64 pairs, now 128 (owned, so it
        // is freed at the end), and the output array was grown several times.
        let reserves = calls_to(&s, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!((reserves[0][2], reserves[0][3]), (128, 8));
        let freed = calls_to(&s, FREE_ARRAY);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 128 * 8);
        assert!(!calls_to(&s, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn tree_collision_ov2_hands_leaf_pairs_to_the_collector() {
        let mut s = setup();
        let a = three_node_tree(&mut s.e, Kind::Wide, LEAVES);
        let b = three_node_tree(
            &mut s.e,
            Kind::Wide,
            [([1.0; 3], [3.0; 3], 5), ([20.0; 3], [22.0; 3], 6)],
        );
        let collector = new_collector(&mut s.e);
        let a_leaf1 = node_of(&s.e, a, Kind::Wide, 1);
        let b_leaf1 = node_of(&s.e, b, Kind::Wide, 1);
        let result = hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(
            &mut s.e,
            p(a),
            p(b),
            p(collector),
        );
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&s, HIT_FUNCTION),
            vec![vec![collector, a_leaf1, b_leaf1]]
        );
        assert!(calls_to(&s, ARRAY_RESERVE).is_empty());
        assert!(calls_to(&s, FREE_ARRAY).is_empty());
    }

    #[test]
    fn tree_collision_ov2_stops_when_the_collector_answers_true() {
        let mut s = setup();
        let a = chain_tree(&mut s.e, 3);
        let b = chain_tree(&mut s.e, 3);
        let collector = new_collector(&mut s.e);
        s.answers.replace(vec![0.0, 1.0]);
        let result = hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(
            &mut s.e,
            p(a),
            p(b),
            p(collector),
        );
        assert_eq!(result, 0);
        assert_eq!(calls_to(&s, HIT_FUNCTION).len(), 2);
    }

    #[test]
    fn tree_collision_ov2_leaf_and_inner_combinations() {
        let mut s = setup();
        let single = single_leaf_tree(&mut s.e, Kind::Wide, [1.0; 3], [11.0; 3], 9);
        let three = three_node_tree(&mut s.e, Kind::Wide, LEAVES);
        let collector = new_collector(&mut s.e);
        let single_root = node_of(&s.e, single, Kind::Wide, 0);
        let leaf1 = node_of(&s.e, three, Kind::Wide, 1);
        let leaf2 = node_of(&s.e, three, Kind::Wide, 2);
        hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(
            &mut s.e,
            p(single),
            p(three),
            p(collector),
        );
        assert_eq!(
            calls_to(&s, HIT_FUNCTION),
            vec![
                vec![collector, single_root, leaf2],
                vec![collector, single_root, leaf1]
            ]
        );
        s.log.borrow_mut().clear();
        hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(
            &mut s.e,
            p(three),
            p(single),
            p(collector),
        );
        assert_eq!(
            calls_to(&s, HIT_FUNCTION),
            vec![
                vec![collector, leaf2, single_root],
                vec![collector, leaf1, single_root]
            ]
        );
        // Both inner: the trees against themselves and against each other.
        s.log.borrow_mut().clear();
        let other = three_node_tree(&mut s.e, Kind::Wide, LEAVES);
        hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(
            &mut s.e,
            p(three),
            p(other),
            p(collector),
        );
        assert_eq!(calls_to(&s, HIT_FUNCTION).len(), 2);
        s.log.borrow_mut().clear();
        hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(
            &mut s.e,
            p(three),
            p(three),
            p(collector),
        );
        assert!(calls_to(&s, HIT_FUNCTION).is_empty());
    }

    #[test]
    fn tree_collision_ov2_needs_both_roots_and_grows_its_stack() {
        let mut s = setup();
        let a = chain_tree(&mut s.e, 24);
        let b = chain_tree(&mut s.e, 24);
        let collector = new_collector(&mut s.e);
        s.answers.replace(vec![0.0]);
        hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(&mut s.e, p(a), p(b), p(collector));
        assert_eq!(calls_to(&s, HIT_FUNCTION).len(), 25 * 25);
        let reserves = calls_to(&s, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!((reserves[0][2], reserves[0][3]), (128, 8));
        let freed = calls_to(&s, FREE_ARRAY);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 128 * 8);
        s.log.borrow_mut().clear();
        s.e.mem.set_u32(b + 0x10, 0);
        let result = hk_aabb_tree_query_utils_query_tree_collision_ov2_uint(
            &mut s.e,
            p(a),
            p(b),
            p(collector),
        );
        assert_eq!(result, 0);
        assert_eq!(node_calls(&s), 0);
    }
}
