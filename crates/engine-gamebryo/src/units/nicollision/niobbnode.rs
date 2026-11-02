//! `NiCollision/niobbnode.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiOBBNode` (Xbox PDB, 0x8c bytes): `m_box` (+4, an `NiOBBox` whose world center is at box
//! +0x3c and whose root pointer is at box +0x78), `m_pLeft` (+0x80), `m_pRight` (+0x84) and
//! `m_uiChangeStamp` (+0x88). Virtual slots: the deleting destructor +0, `IsLeaf` +4,
//! `TestCollision` +8, `FindCollisions` +0xc, `FindIntersections` +0x10.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{float_operand, load_float};

/// `NiOBBox::Transform` (`00eba480`, thiscall on the box: the world transform).
const NI_OBB_BOX_TRANSFORM: u32 = 0x00eb_a480;
/// `NiOBBox::TestIntersection` (`00eba680`, thiscall on the box: the other box), an `int`.
const NI_OBB_BOX_TEST_INTERSECTION: u32 = 0x00eb_a680;
/// `NiOBBox::TestIntersection_ov2` (`00ebf610`, thiscall on the box: the ray origin and
/// direction), an `int`.
const NI_OBB_BOX_TEST_RAY: u32 = 0x00eb_f610;
/// The moving-box contact (`00ebc2a0`, thiscall on the box: the other box, the out time and the
/// out point), an `int`.
const NI_OBB_BOX_FIND_CONTACT: u32 = 0x00eb_c2a0;
/// The world box's normal at a point (`00ebf4a0`, thiscall on the box: the point and the out
/// normal).
const NI_OBB_BOX_NORMAL_AT: u32 = 0x00eb_f4a0;
/// "Has children" (`00eb6b70`, thiscall on a node).
const HAS_CHILDREN: u32 = 0x00eb_6b70;
/// The three words at `011f426c` (zero in the image) the velocities are compared with.
const ZERO_POINT_011F426C: u32 = 0x011f_426c;
/// The double `0.5` at `01011588`.
const HALF_01011588: u32 = 0x0101_1588;

/// Refreshes a node's world box from `owner`'s world transform (+0x68) when its stamp differs.
fn refresh(e: &mut Engine, node: u32, owner: u32, stamp: u32) {
    if stamp != e.mem.u32(node.wrapping_add(0x88)) {
        e.call(
            NI_OBB_BOX_TRANSFORM,
            &args![node.wrapping_add(4), owner.wrapping_add(0x68)],
        );
        e.mem.set_u32(node.wrapping_add(0x88), stamp);
    }
}

fn is_leaf(e: &mut Engine, node: u32) -> bool {
    e.vcall(node, 4, &args![]).bool()
}

// Translated from 00eb9180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBNode::TestCollision` (Xbox PDB): false for a null `other`. Both world boxes are refreshed
/// from their owners when the stamps differ; false when the boxes do not intersect; true when a
/// depth is 0. A leaf against a node goes into the node's children (with `other_depth - 1`), a
/// node against a leaf into this node's children (with `depth - 1`), two nodes into the four pairs
/// of children (both depths minus one); a missing child or a leaf without children ends with true.
/// True when any pair answers true (`TestCollision`, slot +8), otherwise false.
pub fn ni_obb_node_test_collision(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    this_owner: u32,
    other_owner: u32,
    depth: u32,
    other_depth: u32,
    this_stamp: u32,
    other_stamp: u32,
) -> bool {
    let o = other.addr();
    if o == 0 {
        return false;
    }
    let base = this.addr();
    refresh(e, base, this_owner, this_stamp);
    refresh(e, o, other_owner, other_stamp);
    if e.call(NI_OBB_BOX_TEST_INTERSECTION, &args![base + 4, o + 4])
        .u32()
        == 0
    {
        return false;
    }
    if depth == 0 || other_depth == 0 {
        return true;
    }
    let test = |e: &mut Engine, node: u32, against: u32, d: u32, od: u32| {
        e.vcall(
            node,
            8,
            &args![
                against,
                this_owner,
                other_owner,
                d,
                od,
                this_stamp,
                other_stamp
            ],
        )
        .bool()
    };
    if is_leaf(e, base) {
        if is_leaf(e, o) || !e.call(HAS_CHILDREN, &args![other]).bool() {
            return true;
        }
        let next = other_depth.wrapping_sub(1);
        let left = e.mem.u32(o + 0x80);
        if test(e, base, left, depth, next) {
            return true;
        }
        let right = e.mem.u32(o + 0x84);
        return test(e, base, right, depth, next);
    }
    let other_is_leaf = is_leaf(e, o);
    let left = e.mem.u32(base + 0x80);
    if left == 0 || e.mem.u32(base + 0x84) == 0 {
        return true;
    }
    if other_is_leaf {
        let next = depth.wrapping_sub(1);
        if test(e, left, o, next, other_depth) {
            return true;
        }
        let right = e.mem.u32(base + 0x84);
        return test(e, right, o, next, other_depth);
    }
    let (next, other_next) = (depth.wrapping_sub(1), other_depth.wrapping_sub(1));
    for (mine, theirs) in [(0x80u32, 0x80u32), (0x80, 0x84), (0x84, 0x80), (0x84, 0x84)] {
        let child = e.mem.u32(base + mine);
        let against = e.mem.u32(o + theirs);
        if test(e, child, against, next, other_next) {
            return true;
        }
    }
    false
}

// Translated from 00eb9430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBNode::FindIntersections` (Xbox PDB): the world box refreshed from `owner` when the
/// stamp differs; 0 when the ray misses the box (`NiOBBox::TestIntersection_ov2`, its result
/// answered). Otherwise the left and then the right child (when present) are asked
/// (`FindIntersections`, slot +0x10, the same six words): the first result of at least 1 is
/// answered, else 0.
pub fn ni_obb_node_find_intersections(
    e: &mut Engine,
    this: Ptr,
    origin: Ptr,
    direction: Ptr,
    pick: Ptr,
    hit: Ptr,
    owner: u32,
    stamp: u32,
) -> u32 {
    let base = this.addr();
    refresh(e, base, owner, stamp);
    let inside = e
        .call(NI_OBB_BOX_TEST_RAY, &args![base + 4, origin, direction])
        .u32();
    if inside == 0 {
        return inside;
    }
    for side in [0x80u32, 0x84] {
        let child = e.mem.u32(base + side);
        if child == 0 {
            continue;
        }
        let found = e
            .vcall(
                child,
                0x10,
                &args![origin, direction, pick, hit, owner, stamp],
            )
            .u32();
        if found as i32 >= 1 {
            return found;
        }
    }
    0
}

/// The collision report's callback of one side: the collision data at +0x1c of the record's
/// object (or of its owner when the object has none), and its callback at +0x38.
fn callback(e: &Engine, object: u32, owner: u32) -> u32 {
    let mut data = e.mem.u32(object.wrapping_add(0x1c));
    if data == 0 {
        data = e.mem.u32(owner.wrapping_add(0x1c));
        if data == 0 {
            return 0;
        }
    }
    e.mem.u32(data.wrapping_add(0x38))
}

// Translated from 00eb9a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word; `this` is not read): reports a collision record
/// (two objects at +0 and +4, their owners at +8 and +0xc, the time at +0x10, the point at +0x14
/// and each side's point at +0x20 and +0x2c) to the callbacks of both sides' collision data (the
/// object's collision object at +0x1c, else its owner's; the callback at +0x38, cdecl on the
/// record). The second side's callback sees the record with the objects and the two side points
/// swapped (the owners are not). Answers 2 when a callback answered 2, else 1 when one answered
/// non-zero, else 0.
pub fn fn_00eb9a50(e: &mut Engine, _this: Ptr, record: Ptr) -> u32 {
    let r = record.addr();
    let mut first = 0u32;
    let mut second = 0u32;
    let object = e.mem.u32(r);
    if object != 0 {
        let report = callback(e, object, e.mem.u32(r + 8));
        if report != 0 {
            first = e.call(report, &args![record]).u32();
        }
    }
    let other = e.mem.u32(r + 4);
    if other != 0 {
        let report = callback(e, other, e.mem.u32(r + 0xc));
        if report != 0 {
            e.mem.set_u32(r, other);
            e.mem.set_u32(r + 4, object);
            for offset in [0u32, 4, 8] {
                let a = e.mem.u32(r + 0x20 + offset);
                let b = e.mem.u32(r + 0x2c + offset);
                e.mem.set_u32(r + 0x20 + offset, b);
                e.mem.set_u32(r + 0x2c + offset, a);
            }
            second = e.call(report, &args![record]).u32();
        }
    }
    if first == 0 && second == 0 {
        return 0;
    }
    if first == 2 || second == 2 {
        2
    } else {
        1
    }
}

// Translated from 00eb9b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBNode::BoxBoxCollision` (Xbox PDB): builds a 0x40-byte collision record on its stack (the
/// four words, zeros at +0x38 and +0x3c). When both roots' velocities (+0x10 of the root at box
/// +0x78) equal the three words at `011f426c` (`FUCOMP`, unordered is unequal), the time is 0 and
/// the point the middle of the two world centers (box +0x3c, the double `0.5` at `01011588`);
/// otherwise the boxes' moving contact (`00ebc2a0`) fills them, 0 being answered when there is
/// none. Then `*hit` is set, each side's point is the contact point in its box frame
/// (`00ebf4a0`) and the record is reported (`fn_00eb9a50`), whose answer is returned.
pub fn ni_obb_node_box_box_collision(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    first: u32,
    second: u32,
    this_owner: u32,
    other_owner: u32,
    hit: Ptr,
) -> u32 {
    let base = this.addr();
    let o = other.addr();
    e.with_stack(0x40, |e, record| {
        let r = record.addr();
        e.mem.set_u32(r, first);
        e.mem.set_u32(r + 0xc, other_owner);
        e.mem.set_u32(r + 0x38, 0);
        e.mem.set_u32(r + 0x3c, 0);
        e.mem.set_u32(r + 4, second);
        e.mem.set_u32(r + 8, this_owner);
        let mine = e.mem.u32(base + 0x7c).wrapping_add(0x10);
        let theirs = e.mem.u32(o + 0x7c).wrapping_add(0x10);
        let still = |e: &Engine, velocity: u32| {
            (0..3u32).all(|axis| {
                load_float(e, ZERO_POINT_011F426C + 4 * axis)
                    == load_float(e, velocity.wrapping_add(4 * axis))
            })
        };
        let resting = still(e, mine) && {
            // The second root's components are compared the same way, x first.
            (0..3u32).all(|axis| {
                load_float(e, theirs.wrapping_add(4 * axis))
                    == load_float(e, ZERO_POINT_011F426C + 4 * axis)
            })
        };
        if resting {
            e.mem.set_u32(r + 0x10, 0);
            let sum = [0u32, 4, 8].map(|offset| {
                x87_store32(x87_add(
                    load_float(e, o + 0x40 + offset),
                    float_operand(e, base + 0x40 + offset),
                ))
            });
            let half = x87_load64(e.mem.u64(HALF_01011588));
            let point = [
                x87_store32(x87_mul(x87_load32(sum[0]), half)),
                x87_store32(x87_mul(x87_load32(sum[1]), half)),
                x87_store32(x87_mul(half, x87_m32(sum[2]))),
            ];
            for (axis, word) in point.iter().enumerate() {
                e.mem.set_u32(r + 0x14 + 4 * axis as u32, *word);
            }
        } else if e
            .call(
                NI_OBB_BOX_FIND_CONTACT,
                &args![base + 4, o + 4, r + 0x10, r + 0x14],
            )
            .u32()
            == 0
        {
            return 0;
        }
        e.mem.set_u8(hit.addr(), 1);
        e.call(NI_OBB_BOX_NORMAL_AT, &args![base + 4, r + 0x14, r + 0x20]);
        e.call(NI_OBB_BOX_NORMAL_AT, &args![o + 4, r + 0x14, r + 0x2c]);
        fn_00eb9a50(e, this, record)
    })
}

// Translated from 00eb9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBNode::FindCollisions` (Xbox PDB), ten words: 0 for a null `other`. Both world boxes are
/// refreshed from their owners when the stamps differ; 0 when they do not intersect; the box
/// collision (`BoxBoxCollision`) when a depth is 0. A leaf against a leaf or a childless node:
/// the box collision (below 1 answers 0). A leaf against a node: its children with
/// `other_depth - 1`. A node against anything: when it lacks a child, the box collision;
/// otherwise its children with `depth - 1` against `other`. The first child result of at least 1
/// (`FindCollisions`, slot +0xc) is answered, otherwise 0.
pub fn ni_obb_node_find_collisions(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    first: u32,
    second: u32,
    this_owner: u32,
    other_owner: u32,
    depth: u32,
    other_depth: u32,
    this_stamp: u32,
    other_stamp: u32,
    hit: Ptr,
) -> u32 {
    let o = other.addr();
    if o == 0 {
        return 0;
    }
    let base = this.addr();
    refresh(e, base, this_owner, this_stamp);
    refresh(e, o, other_owner, other_stamp);
    if e.call(NI_OBB_BOX_TEST_INTERSECTION, &args![base + 4, o + 4])
        .u32()
        == 0
    {
        return 0;
    }
    let box_box = |e: &mut Engine| {
        ni_obb_node_box_box_collision(e, this, other, first, second, this_owner, other_owner, hit)
    };
    if depth == 0 || other_depth == 0 {
        return box_box(e);
    }
    let find = |e: &mut Engine, node: u32, against: u32, d: u32, od: u32| {
        e.vcall(
            node,
            0xc,
            &args![
                against,
                first,
                second,
                this_owner,
                other_owner,
                d,
                od,
                this_stamp,
                other_stamp,
                hit
            ],
        )
        .u32()
    };
    if is_leaf(e, base) {
        if is_leaf(e, o) || !e.call(HAS_CHILDREN, &args![other]).bool() {
            let found = box_box(e);
            return if (found as i32) < 1 { 0 } else { found };
        }
        let next = other_depth.wrapping_sub(1);
        let left = e.mem.u32(o + 0x80);
        let found = find(e, base, left, depth, next);
        if found as i32 >= 1 {
            return found;
        }
        let right = e.mem.u32(o + 0x84);
        let found = find(e, base, right, depth, next);
        return if (found as i32) < 1 { 0 } else { found };
    }
    // The exe asks whether the other node is a leaf; both answers continue the same way.
    is_leaf(e, o);
    let left = e.mem.u32(base + 0x80);
    let next = depth.wrapping_sub(1);
    if left == 0 || e.mem.u32(base + 0x84) == 0 {
        return box_box(e);
    }
    let found = find(e, left, o, next, other_depth);
    if found as i32 >= 1 {
        return found;
    }
    let right = e.mem.u32(base + 0x84);
    let found = find(e, right, o, next, other_depth);
    if (found as i32) < 1 {
        0
    } else {
        found
    }
}

/// The double 0.25 at `010290b0` (`FLD qword`).
const QUARTER_010290B0: u32 = 0x0102_90b0;
/// `00a59980` (thiscall on a symmetric `NiMatrix3`: the out eigenvalues, the out axes;
/// `NiMain/nimatrix3.cpp`, no Xbox name): its eigen decomposition (by its use here).
const MATRIX_EIGEN: u32 = 0x00a5_9980;
/// `NiTransform::MakeIdentity` (`00a86bf0`, Xbox PDB; thiscall, no arguments).
const MAKE_IDENTITY: u32 = 0x00a8_6bf0;

/// A value stored as a float and loaded back.
fn stored(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// The three corners' addresses of triangle `t`: the `u16` indices at `triangles + 6 t` into the
/// 12-byte vertices.
fn triangle_corners(e: &Engine, triangles: u32, vertices: u32, t: u32) -> [u32; 3] {
    let base = triangles.wrapping_add(t.wrapping_mul(6));
    [0u32, 2, 4].map(|i| {
        let index = u32::from(e.mem.u16(base.wrapping_add(i)));
        vertices.wrapping_add(index.wrapping_mul(12))
    })
}

/// `x0*x0 + ... ` as `fn_00eb8ae0` sums one diagonal term: `((c^2 + p0^2) + p1^2) + p2^2` of the
/// floats at `offset` of the centroid and the three corners.
fn diagonal_term(e: &Engine, c: u32, corners: [u32; 3], offset: u32) -> f64 {
    let [p0, p1, p2] = corners.map(|p| load_float(e, p + offset));
    let c = load_float(e, c + offset);
    let p0p0 = x87_mul(p0, p0);
    let cc = x87_mul(c, c);
    let s = x87_add(cc, p0p0);
    let s = x87_add(s, x87_mul(p1, p1));
    x87_add(s, x87_mul(p2, p2))
}

/// An off-diagonal term of `fn_00eb8ae0`: `c.a*c.b + p0.a*p0.b + p1.a*p1.b + p2.a*p2.b` with the
/// exe's operand orders (`c`, `p1` and `p2` as `[p+first] * [p+second]`, `p0` as the loaded and
/// the memory operand's offsets `p0_order`).
fn cross_term(
    e: &Engine,
    c: u32,
    corners: [u32; 3],
    first: u32,
    second: u32,
    p0_order: (u32, u32),
) -> f64 {
    let [p0, p1, p2] = corners;
    let a = x87_mul(load_float(e, c + first), float_operand(e, c + second));
    let b = x87_mul(
        load_float(e, p0 + p0_order.0),
        float_operand(e, p0 + p0_order.1),
    );
    let s = x87_add(a, b);
    let s = x87_add(
        s,
        x87_mul(load_float(e, p1 + first), float_operand(e, p1 + second)),
    );
    x87_add(
        s,
        x87_mul(load_float(e, p2 + first), float_operand(e, p2 + second)),
    )
}

// Translated from 00eb8ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiOBBNode`, eight words; `fn_00eb94e0` calls
/// it): the area-weighted mean and covariance of the triangles `order[first..=last]` (`first`
/// taken as a `u16`, the loop index compared as a signed number with `last`). For each
/// triangle `t` (its indices at `triangles + 6 t`, its corners in the 12-byte `vertices`, its
/// area `areas[t]` and its centroid `centroids[t]`, 12 bytes) the area and the sums `area * c`
/// and `area * (c c^T + p0 p0^T + p1 p1^T + p2 p2^T)` are accumulated (each stored as a float).
/// The box center (+4) becomes `sum(area c) / sum(area)` and `out` (a 3x3 matrix) gets
/// `0.25 / sum(area) * sum(...) - center center^T` (the 0.25 a double at `010290b0`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00eb8ae0(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    triangles: Ptr,
    vertices: Ptr,
    areas: Ptr,
    centroids: Ptr,
    first: u32,
    last: u32,
    order: Ptr,
) {
    let n = this.addr();
    // weight, x, y, z, xx, xy, xz, yy, yz, zz: stored floats starting at 0.0.
    let mut sums = [0u32; 10];
    let mut counter = first & 0xffff;
    let mut index = counter;
    if index as i32 <= last as i32 {
        loop {
            let t = e.mem.u32(order.addr().wrapping_add(index.wrapping_mul(4)));
            let corners = triangle_corners(e, triangles.addr(), vertices.addr(), t);
            let c = centroids.addr().wrapping_add(t.wrapping_mul(12));
            let area = stored(load_float(e, areas.addr().wrapping_add(t.wrapping_mul(4))));
            let add_to = |sums: &mut [u32; 10], slot: usize, value: f64| {
                sums[slot] = x87_store32(x87_add(value, x87_m32(sums[slot])));
            };
            add_to(&mut sums, 0, area);
            let px = stored(x87_mul(load_float(e, c), area));
            let py = stored(x87_mul(area, float_operand(e, c + 4)));
            let pz = stored(x87_mul(load_float(e, c + 8), area));
            add_to(&mut sums, 1, px);
            add_to(&mut sums, 2, py);
            add_to(&mut sums, 3, pz);
            let xx = x87_mul(diagonal_term(e, c, corners, 0), area);
            add_to(&mut sums, 4, xx);
            let xy = x87_mul(cross_term(e, c, corners, 0, 4, (4, 0)), area);
            add_to(&mut sums, 5, xy);
            let xz = x87_mul(cross_term(e, c, corners, 0, 8, (8, 0)), area);
            add_to(&mut sums, 6, xz);
            let yy = x87_mul(diagonal_term(e, c, corners, 4), area);
            add_to(&mut sums, 7, yy);
            let yz = x87_mul(cross_term(e, c, corners, 8, 4, (8, 4)), area);
            add_to(&mut sums, 8, yz);
            let zz = x87_mul(area, diagonal_term(e, c, corners, 8));
            add_to(&mut sums, 9, zz);
            counter = counter.wrapping_add(1);
            index = counter & 0xffff;
            if index as i32 > last as i32 {
                break;
            }
        }
    }
    let inverse = stored(x87_div(1.0, x87_load32(sums[0])));
    let quarter = x87_load64(e.mem.u64(QUARTER_010290B0));
    let k = stored(x87_mul(quarter, inverse));
    e.mem
        .set_u32(n + 4, x87_store32(x87_mul(x87_load32(sums[1]), inverse)));
    e.mem
        .set_u32(n + 8, x87_store32(x87_mul(x87_load32(sums[2]), inverse)));
    e.mem
        .set_u32(n + 0xc, x87_store32(x87_mul(inverse, x87_m32(sums[3]))));
    let (mx, my, mz) = (n + 4, n + 8, n + 0xc);
    let o = out.addr();
    let x = load_float(e, mx);
    let r0 = x87_sub(x87_mul(k, x87_m32(sums[4])), x87_mul(x, x));
    e.mem.set_u32(o, x87_store32(r0));
    let r1 = x87_mul(k, x87_m32(sums[5]));
    let r1 = x87_store32(x87_sub(
        r1,
        x87_mul(load_float(e, my), float_operand(e, mx)),
    ));
    e.mem.set_u32(o + 4, r1);
    let r2 = x87_mul(k, x87_m32(sums[6]));
    let r2 = x87_store32(x87_sub(
        r2,
        x87_mul(load_float(e, mx), float_operand(e, mz)),
    ));
    e.mem.set_u32(o + 8, r2);
    let y = load_float(e, my);
    let r4 = x87_sub(x87_mul(k, x87_m32(sums[7])), x87_mul(y, y));
    e.mem.set_u32(o + 0x10, x87_store32(r4));
    let r5 = x87_mul(k, x87_m32(sums[8]));
    let r5 = x87_store32(x87_sub(
        r5,
        x87_mul(load_float(e, my), float_operand(e, mz)),
    ));
    e.mem.set_u32(o + 0x14, r5);
    let z = load_float(e, mz);
    let kzz = x87_mul(k, x87_load32(sums[9]));
    let r8 = x87_sub(kzz, x87_mul(z, z));
    e.mem.set_u32(o + 0x20, x87_store32(r8));
    e.mem.set_u32(o + 0xc, r1);
    e.mem.set_u32(o + 0x18, r2);
    e.mem.set_u32(o + 0x1c, r5);
}

/// The projections of `point - center` on the three axes (the 3x3 floats at `axes`), each
/// stored, in `fn_00eb94e0`'s operand order.
fn project(e: &Engine, point: u32, center: u32, axes: u32) -> [f64; 3] {
    let [dx, dy, dz] = [0u32, 4, 8].map(|i| {
        stored(x87_sub(
            load_float(e, point.wrapping_add(i)),
            float_operand(e, center + i),
        ))
    });
    let a = |i: u32| load_float(e, axes + 4 * i);
    let m = |i: u32| float_operand(e, axes + 4 * i);
    let s = x87_add(x87_mul(a(0), dx), x87_mul(dy, m(1)));
    let p0 = stored(x87_add(s, x87_mul(dz, m(2))));
    let t = x87_add(x87_mul(a(4), dy), x87_mul(a(3), dx));
    let p1 = stored(x87_add(t, x87_mul(a(5), dz)));
    let b = x87_mul(dy, m(7));
    let c = x87_mul(dx, a(6));
    let u = x87_add(c, b);
    let p2 = stored(x87_add(x87_mul(dz, a(8)), u));
    [p0, p1, p2]
}

// Translated from 00eb94e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiOBBNode`, eight words: the root, the triangle
/// indices, the vertices, the areas, the centroids, the first and last positions and the order
/// array; `00eb9fa0` calls it): fits the node's box to the triangles `order[first..=last]`. The
/// mean goes to the box center and the covariance to a matrix (`fn_00eb8ae0`), whose eigen
/// decomposition (`00a59980`) gives the box axes (+0x10). Every corner's projection on the
/// axes (relative to the center) widens the minimum and maximum per axis, both starting at 0.0
/// (a smaller value replaces the minimum, otherwise a larger one the maximum; a NaN neither).
/// The extents (+0x34) become `(max - min) * 0.5` and the center moves by the axes times
/// `(min + max) * 0.5` (the 0.5 a double at `01011588`); the box is transformed by the identity
/// (`00a86bf0`, `00eba480`) and the root stored at +0x7c.
#[allow(clippy::too_many_arguments)]
pub fn fn_00eb94e0(
    e: &mut Engine,
    this: Ptr,
    root: u32,
    triangles: Ptr,
    vertices: Ptr,
    areas: Ptr,
    centroids: Ptr,
    first: u32,
    last: u32,
    order: Ptr,
) {
    let n = this.addr();
    let center = n + 4;
    let axes = n + 0x10;
    // The exe's frame: offsets from ESP after its four register pushes.
    e.with_stack(0xc8, |e, frame| {
        let f = frame.addr();
        let covariance = f + 0x70;
        fn_00eb8ae0(
            e,
            this,
            Ptr::new(covariance),
            triangles,
            vertices,
            areas,
            centroids,
            first,
            last,
            order,
        );
        e.call(MATRIX_EIGEN, &args![covariance, f + 0x64, axes]);
        let mut low = [0u32; 3];
        let mut high = [0u32; 3];
        let mut counter = first & 0xffff;
        let mut index = counter;
        if index as i32 <= last as i32 {
            loop {
                let t = e.mem.u32(order.addr().wrapping_add(index.wrapping_mul(4)));
                for corner in triangle_corners(e, triangles.addr(), vertices.addr(), t) {
                    let projection = project(e, corner, center, axes);
                    for axis in 0..3 {
                        let p = projection[axis];
                        if x87_load32(low[axis]) > p {
                            low[axis] = x87_store32(p);
                        } else if x87_load32(high[axis]) < p {
                            high[axis] = x87_store32(p);
                        }
                    }
                }
                counter = counter.wrapping_add(1);
                index = counter & 0xffff;
                if index as i32 > last as i32 {
                    break;
                }
            }
        }
        let half = x87_load64(e.mem.u64(HALF_01011588));
        for axis in 0..3 {
            let size = stored(x87_sub(x87_load32(high[axis]), x87_load32(low[axis])));
            let extent = x87_store32(x87_mul(size, half));
            e.mem.set_u32(n + 0x34 + 4 * axis as u32, extent);
        }
        let middle = [0usize, 1, 2]
            .map(|axis| stored(x87_add(x87_load32(low[axis]), x87_load32(high[axis]))));
        let m0 = stored(x87_mul(middle[0], half));
        let m1 = stored(x87_mul(middle[1], half));
        let m2 = stored(x87_mul(half, middle[2]));
        let scaled = |e: &Engine, row: u32, m: f64| -> [f64; 3] {
            let at = axes + row;
            [
                stored(x87_mul(load_float(e, at), m)),
                stored(x87_mul(load_float(e, at + 4), m)),
                stored(x87_mul(m, float_operand(e, at + 8))),
            ]
        };
        let c2 = scaled(e, 0x18, m2);
        let c1 = scaled(e, 0xc, m1);
        let c0 = scaled(e, 0, m0);
        let shift = [0usize, 1, 2].map(|i| stored(x87_add(stored(x87_add(c0[i], c1[i])), c2[i])));
        let x = x87_add(shift[0], float_operand(e, center));
        e.mem.set_u32(center, x87_store32(x));
        let y = x87_add(shift[1], float_operand(e, center + 4));
        e.mem.set_u32(center + 4, x87_store32(y));
        let z = x87_add(load_float(e, center + 8), shift[2]);
        e.mem.set_u32(center + 8, x87_store32(z));
        let transform = f + 0x94;
        e.call(MAKE_IDENTITY, &args![transform]);
        e.call(NI_OBB_BOX_TRANSFORM, &args![center, transform]);
        e.mem.set_u32(n + 0x7c, root);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eb_8ae0,
            fn_00eb8ae0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32, u32, Ptr)
        ),
        entry!(
            0x00eb_94e0,
            fn_00eb94e0(Ptr, u32, Ptr, Ptr, Ptr, Ptr, u32, u32, Ptr)
        ),
        entry!(
            0x00eb_9180,
            ni_obb_node_test_collision(Ptr, Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00eb_9430,
            ni_obb_node_find_intersections(Ptr, Ptr, Ptr, Ptr, Ptr, u32, u32) -> u32
        ),
        entry!(0x00eb_9a50, fn_00eb9a50(Ptr, Ptr) -> u32),
        entry!(
            0x00eb_9b00,
            ni_obb_node_box_box_collision(Ptr, Ptr, u32, u32, u32, u32, Ptr) -> u32
        ),
        entry!(
            0x00eb_9cc0,
            ni_obb_node_find_collisions(Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32, u32, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    const IS_LEAF: u32 = 0x00ab_5000;
    const IS_NODE: u32 = 0x00ab_5001;
    const TEST: u32 = 0x00ab_5002;
    const FIND: u32 = 0x00ab_5003;
    const FIND_RAY: u32 = 0x00ab_5004;

    fn doubles(e: &mut Engine) {
        e.register(IS_LEAF, |_, _| 1u32.into_ret());
        e.register(IS_NODE, |_, _| 0u32.into_ret());
        e.register(NI_OBB_BOX_TRANSFORM, |_, _| Ret::default());
        e.register(NI_OBB_BOX_TEST_INTERSECTION, |_, _| 1u32.into_ret());
        e.register(HAS_CHILDREN, |e, a| {
            (e.mem.u32(a[0] + 0x80) != 0 || e.mem.u32(a[0] + 0x84) != 0).into_ret()
        });
    }

    /// A node whose leaf test is `leaf_fn` and whose other slots are the test functions, with a
    /// root of zero velocity at box +0x78.
    fn node(e: &mut Engine, leaf_fn: u32) -> u32 {
        let vtable = e.mem.alloc(0x14);
        for (i, s) in [0, leaf_fn, TEST, FIND, FIND_RAY].iter().enumerate() {
            e.mem.set_u32(vtable + 4 * i as u32, *s);
        }
        let n = e.mem.alloc(0x8c);
        e.mem.set_u32(n, vtable);
        let root = e.mem.alloc(0x54);
        e.mem.set_u32(n + 0x7c, root);
        n
    }

    #[test]
    fn test_collision_walks_both_trees() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(TEST, |_, _| Ret::default());
        let this = node(&mut e, IS_NODE);
        let other = node(&mut e, IS_NODE);
        let words = |o: u32, d: u32, od: u32| args![this, o, 0x100u32, 0x200u32, d, od, 1u32, 2u32];
        assert!(!e.call(0x00eb_9180, &words(0, 3, 3)).bool());
        assert!(e.call(0x00eb_9180, &words(other, 0, 3)).bool());
        // A node without children ends with true.
        assert!(e.call(0x00eb_9180, &words(other, 3, 3)).bool());
        let [a, b, c, d] = [0; 4].map(|_: u32| node(&mut e, IS_NODE));
        e.mem.set_u32(this + 0x80, a);
        e.mem.set_u32(this + 0x84, b);
        e.mem.set_u32(other + 0x80, c);
        e.mem.set_u32(other + 0x84, d);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00eb_9180, &words(other, 3, 5)).bool());
        let pairs: Vec<_> = calls_to(&e, TEST)
            .iter()
            .map(|w| (w[0], w[1], w[4], w[5]))
            .collect();
        assert_eq!(
            pairs,
            vec![(a, c, 2, 4), (a, d, 2, 4), (b, c, 2, 4), (b, d, 2, 4)]
        );
        // A leaf against the node: the node's children with the other depth reduced.
        let leaf = node(&mut e, IS_LEAF);
        e.call_log = Some(vec![]);
        let leaf_words = args![leaf, other, 0x100u32, 0x200u32, 3u32, 5u32, 1u32, 2u32];
        assert!(!e.call(0x00eb_9180, &leaf_words).bool());
        let pairs: Vec<_> = calls_to(&e, TEST)
            .iter()
            .map(|w| (w[0], w[1], w[4], w[5]))
            .collect();
        assert_eq!(pairs, vec![(leaf, c, 3, 4), (leaf, d, 3, 4)]);
        // Boxes apart.
        e.register(NI_OBB_BOX_TEST_INTERSECTION, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00eb_9180, &words(other, 3, 3)).bool());
    }

    #[test]
    fn find_intersections_asks_the_children() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(NI_OBB_BOX_TEST_RAY, |_, _| 1u32.into_ret());
        let this = node(&mut e, IS_NODE);
        let left = node(&mut e, IS_NODE);
        let right = node(&mut e, IS_NODE);
        e.register_double(FIND_RAY, move |_, a| {
            u32::from(a[0] == right).wrapping_mul(2).into_ret()
        });
        e.mem.set_u32(this + 0x84, right);
        e.call_log = Some(vec![]);
        let words = args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32];
        assert_eq!(e.call(0x00eb_9430, &words).u32(), 2);
        assert_eq!(calls_to(&e, FIND_RAY), vec![vec![right, 1, 2, 3, 4, 5, 6]]);
        assert_eq!(e.mem.u32(this + 0x88), 6);
        e.mem.set_u32(this + 0x80, left);
        e.mem.set_u32(this + 0x84, 0);
        assert_eq!(e.call(0x00eb_9430, &words).u32(), 0);
        e.register(NI_OBB_BOX_TEST_RAY, |_, _| Ret::default());
        assert_eq!(e.call(0x00eb_9430, &words).u32(), 0);
    }

    /// An object whose collision data's callback (+0x38) is `report`.
    fn object(e: &mut Engine, report: u32) -> u32 {
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 0x38, report);
        let o = e.mem.alloc(0x20);
        e.mem.set_u32(o + 0x1c, data);
        o
    }

    #[test]
    fn report_calls_both_sides_with_the_record_swapped() {
        let mut e = Engine::new();
        e.register(0x00ab_5010, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 0x20), 7);
            1u32.into_ret()
        });
        e.register(0x00ab_5011, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 0x20), 8);
            2u32.into_ret()
        });
        let first = object(&mut e, 0x00ab_5010);
        let second = object(&mut e, 0x00ab_5011);
        let record = e.mem.alloc(0x40);
        e.mem.set_u32(record, first);
        e.mem.set_u32(record + 4, second);
        e.mem.set_u32(record + 0x20, 7);
        e.mem.set_u32(record + 0x2c, 8);
        assert_eq!(e.call(0x00eb_9a50, &args![0u32, record]).u32(), 2);
        assert_eq!(e.mem.u32(record), second);
        assert_eq!(e.mem.u32(record + 4), first);
        // The first side only: 1. No callbacks: 0.
        let lone = e.mem.alloc(0x40);
        e.mem.set_u32(lone, first);
        e.mem.set_u32(lone + 0x20, 7);
        assert_eq!(e.call(0x00eb_9a50, &args![0u32, lone]).u32(), 1);
        e.mem.set_u32(lone, 0);
        assert_eq!(e.call(0x00eb_9a50, &args![0u32, lone]).u32(), 0);
    }

    #[test]
    fn box_box_collision_at_rest_takes_the_middle() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.mem.set_f64(HALF_01011588, 0.5);
        e.register(NI_OBB_BOX_NORMAL_AT, |_, _| Ret::default());
        e.register(0x00ab_5012, |e, a| {
            assert_eq!(e.mem.f32(a[0] + 0x14), 1.0);
            assert_eq!(e.mem.f32(a[0] + 0x1c), 3.0);
            assert_eq!(e.mem.u32(a[0] + 8), 0x300);
            1u32.into_ret()
        });
        let this = node(&mut e, IS_NODE);
        let other = node(&mut e, IS_NODE);
        e.mem.set_f32(this + 0x40, 2.0);
        e.mem.set_f32(other + 0x48, 6.0);
        let reporter = object(&mut e, 0x00ab_5012);
        let hit = e.mem.alloc(4);
        let words = args![this, other, reporter, 0u32, 0x300u32, 0x400u32, hit];
        assert_eq!(e.call(0x00eb_9b00, &words).u32(), 1);
        assert_eq!(e.mem.u8(hit), 1);
        // Moving: the boxes' contact decides; none answers 0.
        let root = e.mem.u32(this + 0x7c);
        e.mem.set_f32(root + 0x10, 1.0);
        e.register(NI_OBB_BOX_FIND_CONTACT, |_, _| Ret::default());
        e.mem.set_u8(hit, 0);
        assert_eq!(e.call(0x00eb_9b00, &words).u32(), 0);
        assert_eq!(e.mem.u8(hit), 0);
    }

    #[test]
    fn find_collisions_descends_this_tree() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        doubles(&mut e);
        e.register(NI_OBB_BOX_FIND_CONTACT, |_, _| Ret::default());
        e.register(FIND, |_, _| Ret::default());
        let this = node(&mut e, IS_NODE);
        let other = node(&mut e, IS_NODE);
        let root = e.mem.u32(this + 0x7c);
        e.mem.set_f32(root + 0x10, 1.0);
        let words = |o: u32, d: u32, od: u32| {
            args![this, o, 1u32, 2u32, 0x100u32, 0x200u32, d, od, 7u32, 8u32, 0x99u32]
        };
        assert_eq!(e.call(0x00eb_9cc0, &words(0, 3, 3)).u32(), 0);
        // No children: the box collision (no contact here).
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00eb_9cc0, &words(other, 3, 3)).u32(), 0);
        assert_eq!(calls_to(&e, NI_OBB_BOX_FIND_CONTACT).len(), 1);
        let left = node(&mut e, IS_NODE);
        let right = node(&mut e, IS_NODE);
        e.mem.set_u32(this + 0x80, left);
        e.mem.set_u32(this + 0x84, right);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00eb_9cc0, &words(other, 3, 3)).u32(), 0);
        let calls: Vec<_> = calls_to(&e, FIND)
            .iter()
            .map(|w| (w[0], w[1], w[6], w[7]))
            .collect();
        assert_eq!(calls, vec![(left, other, 2, 3), (right, other, 2, 3)]);
    }
}

#[cfg(test)]
mod n034b_tests {
    use super::*;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_f32(at + 4 * i, *v);
        }
        at
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// Two triangles; the order array picks triangle 1 (indices 1 2 3: (0,0,0), (3,0,0),
    /// (0,6,9); area 2, centroid (1,2,3)). Answers the eight words after the root.
    fn triangles(e: &mut Engine) -> [u32; 7] {
        e.map(0x0101_1000, 0x1000);
        e.map(0x0102_9000, 0x1000);
        e.mem.set_f64(HALF_01011588, 0.5);
        e.mem.set_f64(QUARTER_010290B0, 0.25);
        let list = e.mem.alloc(12);
        for (i, v) in (0u32..).zip([0u16, 0, 0, 1, 2, 3]) {
            e.mem.set_u16(list + 2 * i, v);
        }
        let vertices = floats(
            e,
            &[9.0, 9.0, 9.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 6.0, 9.0],
        );
        let areas = floats(e, &[5.0, 2.0]);
        let centroids = floats(e, &[7.0, 7.0, 7.0, 1.0, 2.0, 3.0]);
        let order = e.mem.alloc(8);
        e.mem.set_u32(order, 1);
        e.mem.set_u32(order + 4, 0);
        [list, vertices, areas, centroids, 0x0005_0000, 0, order]
    }

    #[test]
    fn covariance_of_the_chosen_triangles() {
        let mut e = Engine::new();
        let [list, vertices, areas, centroids, first, last, order] = triangles(&mut e);
        let node = e.mem.alloc(0x8c);
        let out = e.mem.alloc(0x24);
        // `first` is taken as a u16: 0x0005_0000 is position 0.
        e.call(
            0x00eb_8ae0,
            &args![node, out, list, vertices, areas, centroids, first, last, order],
        );
        assert_eq!(read(&e, node + 4, 3), vec![1.0, 2.0, 3.0]);
        assert_eq!(
            read(&e, out, 9),
            vec![1.5, -1.5, -2.25, -1.5, 6.0, 9.0, -2.25, 9.0, 13.5]
        );
    }

    #[test]
    fn box_fits_the_corners_along_the_axes() {
        let mut e = Engine::new();
        let [list, vertices, areas, centroids, first, last, order] = triangles(&mut e);
        e.register(MATRIX_EIGEN, |e, a| {
            for i in 0..9 {
                let v = if i % 4 == 0 { 1.0 } else { 0.0 };
                e.mem.set_f32(a[2] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(MAKE_IDENTITY, |_, _| Ret::default());
        e.register(NI_OBB_BOX_TRANSFORM, |_, _| Ret::default());
        let node = e.mem.alloc(0x8c);
        e.call_log = Some(vec![]);
        e.call(
            0x00eb_94e0,
            &args![node, 0x77u32, list, vertices, areas, centroids, first, last, order],
        );
        assert_eq!(read(&e, node + 0x34, 3), vec![1.5, 3.0, 4.5]);
        assert_eq!(read(&e, node + 4, 3), vec![1.5, 3.0, 4.5]);
        assert_eq!(e.mem.u32(node + 0x7c), 0x77);
        let log = e.call_log.clone().unwrap();
        let transform = log
            .iter()
            .find(|(a, _)| *a == NI_OBB_BOX_TRANSFORM)
            .map(|(_, w)| w.clone())
            .unwrap();
        assert_eq!(transform[0], node + 4);
        let eigen = log
            .iter()
            .find(|(a, _)| *a == MATRIX_EIGEN)
            .map(|(_, w)| w.clone())
            .unwrap();
        assert_eq!(eigen[2], node + 0x10);
    }
}
