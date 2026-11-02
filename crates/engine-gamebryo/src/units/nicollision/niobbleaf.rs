//! `NiCollision/niobbleaf.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiOBBNode` (Xbox PDB, 0x8c bytes; vtable `010f1b58` by its RTTI): `m_box` (+4, an `NiOBBox`
//! of 0x7c bytes), `m_pLeft` (+0x80), `m_pRight` (+0x84) and `m_uiChangeStamp` (+0x88).
//! `NiOBBLeaf` (0x98 bytes, vtable `010f1b70`) adds `m_pWorldVertex` (+0x8c, three vertex
//! pointers). The node's virtual slots: the deleting destructor +0, `IsLeaf` +4, `TestCollision`
//! +8, `FindCollisions` +0xc, `FindIntersections` +0x10.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::{float_operand, load_float};

/// `NiOBBNode`'s vtable (`010f1b58`).
const NI_OBB_NODE_VTABLE: u32 = 0x010f_1b58;
/// The memory manager instance and its `Deallocate` (`00aa4060`, thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `NiOBBox::Transform` (`00eba480`, thiscall on the box: the world transform).
const NI_OBB_BOX_TRANSFORM: u32 = 0x00eb_a480;
/// `NiOBBox::TestIntersection` (`00eba680`, thiscall on the box: the other box), an `int`.
const NI_OBB_BOX_TEST_INTERSECTION: u32 = 0x00eb_a680;
/// The pick's ray-triangle test (`00eadb90`, cdecl, ten words).
const RAY_HITS_TRIANGLE: u32 = 0x00ea_db90;
/// The pick results pool's next record (`00e99330`, thiscall on the pool: the object).
const NEXT_PICK_RECORD: u32 = 0x00e9_9330;
/// Unnamed in the engine map: normalizes the three floats at ECX in place.
const UNITIZE: u32 = 0x004a_0c10;

/// Refreshes a node's world box from `owner`'s world transform (+0x68, `NiOBBox::Transform`)
/// when its change stamp differs from `stamp`, then keeps `stamp`.
fn refresh(e: &mut Engine, node: u32, owner: u32, stamp: u32) {
    if stamp != e.mem.u32(node.wrapping_add(0x88)) {
        e.call(
            NI_OBB_BOX_TRANSFORM,
            &args![node.wrapping_add(4), owner.wrapping_add(0x68)],
        );
        e.mem.set_u32(node.wrapping_add(0x88), stamp);
    }
}

// Translated from 00eb6b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBLeaf::_scalar_deleting_destructor_` (Xbox PDB; also slot 0 of the `NiOBBNode` vtable):
/// the node destructor (`fn_00eb8ab0`), then the memory manager frees `this` when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_obb_leaf_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00eb8ab0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00eb6b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: 1 when the node has a left (+0x80) or a right (+0x84) child,
/// otherwise 0.
pub fn fn_00eb6b70(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    u32::from(e.mem.u32(base + 0x80) != 0 || e.mem.u32(base + 0x84) != 0)
}

// Translated from 00eb72b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`): the barycentric weights of `point` in the triangle
/// `(a, b, c)`. With `u = a - c`, `v = b - c`, `w = point - c` (stored floats) and the stored dot
/// products `uu`, `uv`, `vv`, `uw`, `vw`: `inv = 1 / (vv uu - uv^2)` (stored), `out[0] =
/// (vv uw - vw uv) inv`, `out[1] = inv (uu vw - uv uw)` and `out[2] = (1 - out[0]) - out[1]`, in
/// the exe's operand order.
pub fn fn_00eb72b0(e: &mut Engine, a: Ptr, b: Ptr, c: Ptr, point: Ptr, out: Ptr) {
    let diff = |e: &Engine, p: u32| {
        [0u32, 4, 8].map(|offset| {
            x87_load32(x87_store32(x87_sub(
                load_float(e, p.wrapping_add(offset)),
                float_operand(e, c.addr().wrapping_add(offset)),
            )))
        })
    };
    let [ux, uy, uz] = diff(e, a.addr());
    let [vx, vy, vz] = diff(e, b.addr());
    let [wx, wy, wz] = diff(e, point.addr());
    let uu = x87_add(x87_mul(uz, uz), x87_add(x87_mul(ux, ux), x87_mul(uy, uy)));
    let uu = x87_load32(x87_store32(uu));
    let uv = x87_add(x87_add(x87_mul(vx, ux), x87_mul(vy, uy)), x87_mul(vz, uz));
    let uv = x87_load32(x87_store32(uv));
    let vv = x87_add(x87_add(x87_mul(vx, vx), x87_mul(vy, vy)), x87_mul(vz, vz));
    let vv = x87_load32(x87_store32(vv));
    let uw = x87_add(x87_mul(uz, wz), x87_add(x87_mul(uy, wy), x87_mul(ux, wx)));
    let uw = x87_load32(x87_store32(uw));
    let vw = x87_add(x87_mul(wz, vz), x87_add(x87_mul(vy, wy), x87_mul(vx, wx)));
    let vw = x87_load32(x87_store32(vw));
    let determinant = x87_sub(x87_mul(vv, uu), x87_mul(uv, uv));
    let inverse = x87_load32(x87_store32(x87_div(1.0, determinant)));
    let first = x87_sub(x87_mul(vv, uw), x87_mul(vw, uv));
    let first = x87_store32(x87_mul(first, inverse));
    let o = out.addr();
    e.mem.set_u32(o, first);
    let first = x87_load32(first);
    let second = x87_sub(x87_mul(uu, vw), x87_mul(uv, uw));
    let second = x87_store32(x87_mul(inverse, second));
    e.mem.set_u32(o.wrapping_add(4), second);
    let third = x87_sub(x87_sub(1.0, first), x87_load32(second));
    e.mem.set_u32(o.wrapping_add(8), x87_store32(third));
}

/// `FLD dword [address]`.
fn load(e: &Engine, address: u32) -> f64 {
    load_float(e, address)
}

/// A stored float loaded back (`FSTP` then `FLD`).
fn round(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// The motions the edge-plane crossing writes, `velocity * time`, in the three operand orders the
/// exe uses: `Mixed` is `v.x * t`, `v.y * t`, `t * v.z`; `Late` is `v.x * t`, `t * v.y`,
/// `t * v.z`; `Loaded` is `v.x * t`, `v.y * t`, `v.z * t` (each product stored as a float).
#[derive(Clone, Copy)]
enum Order {
    Mixed,
    Late,
    Loaded,
}

fn motion(e: &Engine, velocity: u32, time: f64, order: Order) -> [u32; 3] {
    let x = x87_mul(load(e, velocity), time);
    let (y, z) = match order {
        Order::Mixed => (
            x87_mul(load(e, velocity + 4), time),
            x87_mul(time, float_operand(e, velocity + 8)),
        ),
        Order::Late => (
            x87_mul(time, float_operand(e, velocity + 4)),
            x87_mul(time, float_operand(e, velocity + 8)),
        ),
        Order::Loaded => (
            x87_mul(load(e, velocity + 4), time),
            x87_mul(load(e, velocity + 8), time),
        ),
    };
    [x87_store32(x), x87_store32(y), x87_store32(z)]
}

/// `point + offset` as the exe adds a stored offset to a vertex (`p.x + o.x`, `p.y + o.y`,
/// `o.z + p.z`, each stored).
fn offset_point(e: &Engine, point: u32, offset: [u32; 3]) -> [u32; 3] {
    [
        x87_store32(x87_add(load(e, point), x87_m32(offset[0]))),
        x87_store32(x87_add(load(e, point + 4), x87_m32(offset[1]))),
        x87_store32(x87_add(x87_load32(offset[2]), float_operand(e, point + 8))),
    ]
}

fn write_point(e: &mut Engine, out: u32, words: [u32; 3]) {
    for (axis, word) in words.iter().enumerate() {
        e.mem.set_u32(out.wrapping_add(4 * axis as u32), *word);
    }
}

/// The point between the two edge ends where the plane distance changes sign: with
/// `f = from / (from - to)` and `g = 1 - f` (both stored), `p0 * g + p1 * f` plus `shift` (each
/// product and sum stored, in the exe's operand order).
fn crossing_point(e: &Engine, p0: u32, p1: u32, f: f64, shift: [u32; 3]) -> [u32; 3] {
    let far = [
        x87_store32(x87_mul(load(e, p1), f)),
        x87_store32(x87_mul(load(e, p1 + 4), f)),
        x87_store32(x87_mul(load(e, p1 + 8), f)),
    ];
    let g = round(x87_sub(1.0, f));
    let near = [
        x87_store32(x87_mul(load(e, p0), g)),
        x87_store32(x87_mul(load(e, p0 + 4), g)),
        x87_store32(x87_mul(g, float_operand(e, p0 + 8))),
    ];
    let sum =
        [0usize, 1, 2].map(|axis| x87_store32(x87_add(x87_load32(near[axis]), x87_m32(far[axis]))));
    [0usize, 1, 2].map(|axis| x87_store32(x87_add(x87_load32(sum[axis]), x87_m32(shift[axis]))))
}

/// The edge-plane outputs: the two times and points.
struct Crossing {
    time_a: u32,
    point_a: u32,
    time_b: u32,
    point_b: u32,
}

impl Crossing {
    /// Records a second contact as `(time_b, point_b)` and swaps the two when it came earlier
    /// (`FCOMPP` of the times, unordered keeps them).
    fn second(&self, e: &mut Engine, time: u32, point: [u32; 3]) {
        e.mem.set_u32(self.time_b, time);
        write_point(e, self.point_b, point);
        let first = load(e, self.time_a);
        let second = load(e, self.time_b);
        if second < first {
            let kept = x87_quiet_bits(e.mem.u32(self.time_a));
            let earlier = x87_quiet_bits(e.mem.u32(self.time_b));
            e.mem.set_u32(self.time_a, earlier);
            e.mem.set_u32(self.time_b, kept);
            for offset in [0u32, 4, 8] {
                let a = e.mem.u32(self.point_a + offset);
                let b = e.mem.u32(self.point_b + offset);
                e.mem.set_u32(self.point_a + offset, b);
                e.mem.set_u32(self.point_b + offset, a);
            }
        }
    }
}

// Translated from 00eb7440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, ten words): where the edge `p0`-`p1`, moving with
/// `velocity` from time 0 to `time`, crosses the plane `normal . x = constant`. With the signed
/// distances (stored floats) `s0`, `s1` of the ends at time 0 and `s2`, `s3` at `time` (the
/// positions `p + velocity * 0` and `p + velocity * time`), each of the four sides of that
/// swept quad is checked in order: the start edge (`s0`, `s1`), the path of `p0` (`s0`, `s2`), the
/// path of `p1` (`s1`, `s3`) and the end edge (`s2`, `s3`). A side whose distances' product is at
/// most 0 holds a contact: when both are exactly 0 the two contacts are its ends (times and
/// points written, answer 1); otherwise its sign change gives a time and a point (the edge
/// interpolation `p0 (1 - f) + p1 f` for an edge, `time * s / (s - s')` along a path), the first
/// found into `(time_a, point_a)`, the second into `(time_b, point_b)` (then ordered by time,
/// answer 1). One contact is copied into both; none answers 0.
pub fn fn_00eb7440(
    e: &mut Engine,
    time: f32,
    normal: Ptr,
    constant: f32,
    velocity: Ptr,
    p0: Ptr,
    p1: Ptr,
    time_a: Ptr,
    point_a: Ptr,
    time_b: Ptr,
    point_b: Ptr,
) -> u32 {
    let n = normal.addr();
    let v = velocity.addr();
    let (p0, p1) = (p0.addr(), p1.addr());
    let out = Crossing {
        time_a: time_a.addr(),
        point_a: point_a.addr(),
        time_b: time_b.addr(),
        point_b: point_b.addr(),
    };
    let t = x87_load32(time.to_bits());
    let c = x87_load32(constant.to_bits());
    let span = x87_store32(x87_sub(t, 0.0));
    let at_start = [0u32, 4, 8].map(|offset| x87_store32(x87_mul(load(e, v + offset), 0.0)));
    let start0 = [
        x87_store32(x87_add(load(e, p0), x87_m32(at_start[0]))),
        x87_store32(x87_add(load(e, p0 + 4), x87_m32(at_start[1]))),
        x87_store32(x87_add(load(e, p0 + 8), x87_m32(at_start[2]))),
    ];
    let dot_yxz = |e: &Engine, p: &[u32]| {
        x87_store32(x87_add(
            x87_add(
                x87_mul(x87_load32(p[1]), float_operand(e, n + 4)),
                x87_mul(x87_load32(p[0]), float_operand(e, n)),
            ),
            x87_mul(x87_load32(p[2]), float_operand(e, n + 8)),
        ))
    };
    let dot_xyz = |e: &Engine, p: &[u32]| {
        x87_store32(x87_add(
            x87_add(
                x87_mul(x87_load32(p[0]), float_operand(e, n)),
                x87_mul(x87_load32(p[1]), float_operand(e, n + 4)),
            ),
            x87_mul(x87_load32(p[2]), float_operand(e, n + 8)),
        ))
    };
    let s0 = round(x87_sub(x87_load32(dot_yxz(e, &start0)), c));
    let start1 = offset_point(e, p1, at_start);
    let s1 = round(x87_sub(x87_load32(dot_xyz(e, &start1)), c));
    let at_end = [0u32, 4, 8].map(|offset| x87_store32(x87_mul(load(e, v + offset), t)));
    let end0 = offset_point(e, p0, at_end);
    let s2 = round(x87_sub(x87_load32(dot_xyz(e, &end0)), c));
    let end1 = offset_point(e, p1, at_end);
    let s3 = round(x87_sub(x87_m32(dot_xyz(e, &end1)), c));

    let mut found = false;
    // The start edge.
    if x87_mul(s1, s0) <= 0.0 {
        if s0 == 0.0 && s1 == 0.0 {
            e.mem.set_u32(out.time_a, 0);
            e.mem.set_u32(out.time_b, 0);
            let at = load(e, out.time_a);
            let point = offset_point(e, p0, motion(e, v, at, Order::Mixed));
            write_point(e, out.point_a, point);
            let at = load(e, out.time_b);
            let point = offset_point(e, p1, motion(e, v, at, Order::Mixed));
            write_point(e, out.point_b, point);
            return 1;
        }
        e.mem.set_u32(out.time_a, 0);
        found = true;
        let f = round(x87_div(s0, x87_sub(s0, s1)));
        let shift = motion(e, v, 0.0, Order::Loaded);
        let point = crossing_point(e, p0, p1, f, shift);
        write_point(e, out.point_a, point);
    }
    // The path of the first end, then of the second.
    for (here, there, vertex, first_order) in
        [(s0, s2, p0, Order::Mixed), (s1, s3, p1, Order::Late)]
    {
        if !(x87_mul(there, here) <= 0.0) {
            continue;
        }
        if here == 0.0 && there == 0.0 {
            e.mem.set_u32(out.time_a, 0);
            e.mem.set_u32(out.time_b, x87_store32(t));
            let at = load(e, out.time_a);
            let point = offset_point(e, vertex, motion(e, v, at, first_order));
            write_point(e, out.point_a, point);
            let at = load(e, out.time_b);
            let point = offset_point(e, vertex, motion(e, v, at, Order::Late));
            write_point(e, out.point_b, point);
            return 1;
        }
        let along = x87_mul(here, x87_m32(span));
        let at = round(x87_add(x87_div(along, x87_sub(here, there)), 0.0));
        let point = offset_point(e, vertex, motion(e, v, at, Order::Loaded));
        if found {
            out.second(e, x87_store32(at), point);
            return 1;
        }
        e.mem.set_u32(out.time_a, x87_store32(at));
        write_point(e, out.point_a, point);
        found = true;
    }
    // The end edge.
    if x87_mul(s3, s2) <= 0.0 {
        if s2 == 0.0 && s3 == 0.0 {
            e.mem.set_u32(out.time_a, x87_store32(t));
            e.mem.set_u32(out.time_b, x87_store32(t));
            let at = load(e, out.time_a);
            let point = offset_point(e, p0, motion(e, v, at, Order::Late));
            write_point(e, out.point_a, point);
            let at = load(e, out.time_b);
            let point = offset_point(e, p1, motion(e, v, at, Order::Late));
            write_point(e, out.point_b, point);
            return 1;
        }
        let f = round(x87_div(s2, x87_sub(s2, s3)));
        let shift = motion(e, v, t, Order::Loaded);
        let point = crossing_point(e, p0, p1, f, shift);
        if found {
            out.second(e, x87_store32(t), point);
            return 1;
        }
        e.mem.set_u32(out.time_a, x87_store32(t));
        write_point(e, out.point_a, point);
        found = true;
    }
    if !found {
        return 0;
    }
    let first = x87_quiet_bits(e.mem.u32(out.time_a));
    e.mem.set_u32(out.time_b, first);
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(out.point_a + offset);
        e.mem.set_u32(out.point_b + offset, word);
    }
    1
}

/// `NiOBBNode::BoxBoxCollision` (`00eb9b00`, thiscall: the other node and five words).
const NI_OBB_NODE_BOX_BOX_COLLISION: u32 = 0x00eb_9b00;
/// The collision report of a node (`00eb9a50`, thiscall: the address of the record).
const NI_OBB_NODE_REPORT: u32 = 0x00eb_9a50;
/// The world box's normal at a point (`00ebf4a0`, thiscall on an `NiOBBox`: the point and the
/// out normal).
const NI_OBB_BOX_NORMAL_AT: u32 = 0x00eb_f4a0;
/// The `float` at `01016970` (`FLT_MAX` in the executable) the earliest contact time starts at.
const NO_CONTACT_01016970: u32 = 0x0101_6970;

// Translated from 00eb7dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, nine words): whether the moving edge `p0`-`p1` meets the
/// triangle (the three vertex pointers at `triangle`) whose plane is `normal . x = constant` within
/// `time`, keeping the earliest contact. The two plane crossings come from `fn_00eb7440`; each is
/// placed in the triangle by its barycentric weights (`fn_00eb72b0`). When the first crossing's
/// weights are all at least 0 it is the contact; otherwise the crossing segment is cut where the
/// first negative weight changes sign (`k = a / (a - b)`, both other weights then checked to be at
/// least 0; unordered fails). The contact's time (`(1 - k) tA + k tB`) replaces `best_time` and its
/// point `best_point` only when `best_time` is greater. Answers 1 for a contact (kept or not),
/// 0 otherwise.
pub fn fn_00eb7dc0(
    e: &mut Engine,
    p0: Ptr,
    p1: Ptr,
    triangle: Ptr,
    normal: Ptr,
    constant: f32,
    velocity: Ptr,
    time: f32,
    best_time: Ptr,
    best_point: Ptr,
) -> u32 {
    e.with_stack(0x3c, |e, locals| {
        let base = locals.addr();
        let (time_a, time_b) = (base + 4, base + 8);
        let (point_a, point_b) = (base + 0xc, base + 0x18);
        let (weights_b, weights_a) = (base + 0x24, base + 0x30);
        let found = fn_00eb7440(
            e,
            f32::from_bits(x87_quiet_bits(time.to_bits())),
            normal,
            f32::from_bits(x87_quiet_bits(constant.to_bits())),
            velocity,
            p0,
            p1,
            Ptr::new(time_a),
            Ptr::new(point_a),
            Ptr::new(time_b),
            Ptr::new(point_b),
        );
        if found == 0 {
            return 0;
        }
        let t = triangle.addr();
        let vertices = [e.mem.u32(t), e.mem.u32(t + 4), e.mem.u32(t + 8)].map(Ptr::new);
        fn_00eb72b0(
            e,
            vertices[0],
            vertices[1],
            vertices[2],
            Ptr::new(point_a),
            Ptr::new(weights_a),
        );
        fn_00eb72b0(
            e,
            vertices[0],
            vertices[1],
            vertices[2],
            Ptr::new(point_b),
            Ptr::new(weights_b),
        );
        let a = [
            load(e, weights_a),
            load(e, weights_a + 4),
            load(e, weights_a + 8),
        ];
        let b = [weights_b, weights_b + 4, weights_b + 8];
        let best = best_time.addr();
        // The cut where weight `axis` changes sign: `k = a / (a - b)` (stored) and `1 - k`.
        let cut = |axis: usize, b_value: f64| {
            let k = round(x87_div(a[axis], x87_sub(a[axis], b_value)));
            (k, x87_sub(1.0, k))
        };
        // The contact at the cut: the time `k tB + tA (1 - k)` and the point
        // `pA (1 - k) + pB k` (`late_z` when the last product of `pB` loads its z first).
        let keep = |e: &mut Engine, k: f64, rest: f64, late_z: bool| {
            let at = round(x87_add(
                x87_mul(k, float_operand(e, time_b)),
                x87_mul(load(e, time_a), rest),
            ));
            if !(load(e, best) > at) {
                return 1;
            }
            e.mem.set_u32(best, x87_store32(at));
            let z = if late_z {
                x87_mul(k, load(e, point_b + 8))
            } else {
                x87_mul(k, float_operand(e, point_b + 8))
            };
            let far = [
                x87_store32(x87_mul(load(e, point_b), k)),
                x87_store32(x87_mul(load(e, point_b + 4), k)),
                x87_store32(z),
            ];
            let rest = round(rest);
            let near = [
                x87_store32(x87_mul(load(e, point_a), rest)),
                x87_store32(x87_mul(load(e, point_a + 4), rest)),
                x87_store32(x87_mul(rest, float_operand(e, point_a + 8))),
            ];
            let point = [0usize, 1, 2]
                .map(|axis| x87_store32(x87_add(x87_load32(near[axis]), x87_m32(far[axis]))));
            write_point(e, best_point.addr(), point);
            1
        };
        if a[0] < 0.0 {
            let b0 = load(e, b[0]);
            if b0 < 0.0 {
                return 0;
            }
            let (k, rest) = cut(0, b0);
            let second = round(x87_add(x87_mul(load(e, b[1]), k), x87_mul(a[1], rest)));
            if !(second >= 0.0) {
                return 0;
            }
            let third = round(x87_add(x87_mul(load(e, b[2]), k), x87_mul(a[2], rest)));
            if !(third >= 0.0) {
                return 0;
            }
            return keep(e, k, rest, false);
        }
        if a[1] < 0.0 {
            let b1 = load(e, b[1]);
            if b1 < 0.0 {
                return 0;
            }
            let (k, rest) = cut(1, b1);
            let first = round(x87_add(
                x87_mul(k, float_operand(e, b[0])),
                x87_mul(a[0], rest),
            ));
            if !(first >= 0.0) {
                return 0;
            }
            let third = round(x87_add(x87_mul(load(e, b[2]), k), x87_mul(a[2], rest)));
            if !(third >= 0.0) {
                return 0;
            }
            return keep(e, k, rest, true);
        }
        if a[2] < 0.0 {
            let b2 = load(e, b[2]);
            if b2 < 0.0 {
                return 0;
            }
            let (k, rest) = cut(2, b2);
            let first = round(x87_add(
                x87_mul(k, float_operand(e, b[0])),
                x87_mul(a[0], rest),
            ));
            if !(first >= 0.0) {
                return 0;
            }
            let second = round(x87_add(x87_mul(load(e, b[1]), k), x87_mul(a[1], rest)));
            if !(second >= 0.0) {
                return 0;
            }
            return keep(e, k, rest, true);
        }
        let at = load(e, time_a);
        if !(load(e, best) > at) {
            return 1;
        }
        e.mem.set_u32(best, x87_store32(at));
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(point_a + offset);
            e.mem.set_u32(best_point.addr() + offset, word);
        }
        1
    })
}

// Translated from 00eb8250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): the three edges of the first triangle (vertex
/// pointer triples at `first` and `second`) moving with `velocity` against the second triangle's
/// plane: the normal `(v1 - v0) x (v2 - v0)` of the second triangle (stored floats) and the constant
/// `normal . v0` (stored), then `fn_00eb7dc0` for the edges `a0 a1`, `a0 a2` and `a1 a2` (all three
/// run). Answers the three results or-ed.
pub fn fn_00eb8250(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    velocity: Ptr,
    time: f32,
    best_time: Ptr,
    best_point: Ptr,
) -> u32 {
    let f = first.addr();
    let s = second.addr();
    let b0 = e.mem.u32(s);
    let b1 = e.mem.u32(s + 4);
    let b2 = e.mem.u32(s + 8);
    let edge = |e: &Engine, to: u32| {
        [0u32, 4, 8]
            .map(|offset| round(x87_sub(load(e, to + offset), float_operand(e, b0 + offset))))
    };
    let [ax, ay, az] = edge(e, b1);
    let [bx, by, bz] = edge(e, b2);
    let normal = [
        x87_store32(x87_sub(x87_mul(bz, ay), x87_mul(by, az))),
        x87_store32(x87_sub(x87_mul(az, bx), x87_mul(bz, ax))),
        x87_store32(x87_sub(x87_mul(ax, by), x87_mul(bx, ay))),
    ];
    let constant = x87_store32(x87_add(
        x87_add(
            x87_mul(load(e, b0 + 4), x87_m32(normal[1])),
            x87_mul(load(e, b0), x87_m32(normal[0])),
        ),
        x87_mul(load(e, b0 + 8), x87_m32(normal[2])),
    ));
    let time = f32::from_bits(x87_quiet_bits(time.to_bits()));
    let constant = f32::from_bits(x87_quiet_bits(constant));
    let a = [e.mem.u32(f), e.mem.u32(f + 4), e.mem.u32(f + 8)];
    e.with_stack(12, |e, n| {
        for (axis, word) in normal.iter().enumerate() {
            e.mem.set_u32(n.addr() + 4 * axis as u32, *word);
        }
        let mut found = 0u32;
        for (from, to) in [(a[0], a[1]), (a[0], a[2]), (a[1], a[2])] {
            found |= fn_00eb7dc0(
                e,
                Ptr::new(from),
                Ptr::new(to),
                second,
                n,
                constant,
                velocity,
                time,
                best_time,
                best_point,
            );
        }
        found
    })
}

// Translated from 00eb83b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`): the earliest contact of two moving leaf triangles during
/// the first root's `m_fDeltaTime` (+4 of the root at box +0x78). The first triangle's edges run
/// against the second's plane with the relative velocity `vA - vB` (`fn_00eb8250`) and the
/// second's against the first's with `vB - vA`, each starting from `FLT_MAX` (`01016970`). The
/// earlier contact found wins (the first on a tie); its time goes to `out_time` and its point,
/// moved by the other root's velocity times that time, to `out_point`. Answers 1 for a contact, 0
/// otherwise.
pub fn fn_00eb83b0(e: &mut Engine, first: Ptr, second: Ptr, out_time: Ptr, out_point: Ptr) -> u32 {
    let a_root = e.mem.u32(first.addr() + 0x7c);
    let b_root = e.mem.u32(second.addr() + 0x7c);
    let va = a_root.wrapping_add(0x10);
    let vb = b_root.wrapping_add(0x10);
    let span = x87_quiet_bits(e.mem.u32(a_root.wrapping_add(4)));
    let no_contact = x87_quiet_bits(e.mem.u32(NO_CONTACT_01016970));
    e.with_stack(0x2c, |e, locals| {
        let base = locals.addr();
        let (best_a, best_b) = (base, base + 4);
        let (point_a, point_b) = (base + 8, base + 0x14);
        let relative = base + 0x20;
        let verts_a = first.addr() + 0x8c;
        let verts_b = second.addr() + 0x8c;
        e.mem.set_u32(best_a, no_contact);
        let rel = difference3(e, va, vb);
        write_point(e, relative, rel);
        let found_a = fn_00eb8250(
            e,
            Ptr::new(verts_a),
            Ptr::new(verts_b),
            Ptr::new(relative),
            f32::from_bits(span),
            Ptr::new(best_a),
            Ptr::new(point_a),
        );
        e.mem.set_u32(best_b, no_contact);
        let rel = difference3(e, vb, va);
        write_point(e, relative, rel);
        let found_b = fn_00eb8250(
            e,
            Ptr::new(verts_b),
            Ptr::new(verts_a),
            Ptr::new(relative),
            f32::from_bits(span),
            Ptr::new(best_b),
            Ptr::new(point_b),
        );
        let point = if found_a != 0 {
            let a_time = load(e, best_a);
            if found_b != 0 {
                let b_time = load(e, best_b);
                if b_time >= a_time {
                    e.mem.set_u32(out_time.addr(), x87_store32(a_time));
                    let shift = motion(e, vb, a_time, Order::Mixed);
                    [0usize, 1, 2].map(|axis| {
                        x87_store32(x87_add(
                            load(e, point_a + 4 * axis as u32),
                            x87_m32(shift[axis]),
                        ))
                    })
                } else {
                    e.mem.set_u32(out_time.addr(), x87_store32(b_time));
                    let shift = motion(e, va, b_time, Order::Mixed);
                    [0usize, 1, 2].map(|axis| {
                        x87_store32(x87_add(
                            load(e, point_b + 4 * axis as u32),
                            x87_m32(shift[axis]),
                        ))
                    })
                }
            } else {
                e.mem.set_u32(out_time.addr(), x87_store32(a_time));
                let shift = motion(e, vb, a_time, Order::Mixed);
                [0usize, 1, 2].map(|axis| {
                    x87_store32(x87_add(
                        x87_load32(shift[axis]),
                        float_operand(e, point_a + 4 * axis as u32),
                    ))
                })
            }
        } else if found_b != 0 {
            let b_time = load(e, best_b);
            e.mem.set_u32(out_time.addr(), x87_store32(b_time));
            let shift = motion(e, va, b_time, Order::Mixed);
            [0usize, 1, 2].map(|axis| {
                x87_store32(x87_add(
                    x87_load32(shift[axis]),
                    float_operand(e, point_b + 4 * axis as u32),
                ))
            })
        } else {
            return 0;
        };
        write_point(e, out_point.addr(), point);
        1
    })
}

/// `a - b` for three floats (each `FLD`, `FSUB dword`, `FSTP`).
fn difference3(e: &Engine, a: u32, b: u32) -> [u32; 3] {
    [0u32, 4, 8].map(|offset| {
        x87_store32(x87_sub(
            load(e, a.wrapping_add(offset)),
            float_operand(e, b.wrapping_add(offset)),
        ))
    })
}

// Translated from 00eb85d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a leaf, six words): the triangle-triangle collision of
/// two leaves. On a contact (`fn_00eb83b0`) it sets `*hit`, builds a 0x40-byte record on its stack
/// (the four words, the contact time and point, the point in each leaf's world box frame through
/// `00ebf4a0`, and both leaves' vertex arrays at +0x8c) and answers the node report (`00eb9a50`) on
/// it; otherwise 0.
pub fn fn_00eb85d0(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    hit: Ptr,
) -> u32 {
    e.with_stack(0x40, |e, record| {
        let r = record.addr();
        e.mem.set_u32(r + 4, second);
        e.mem.set_u32(r + 8, third);
        e.mem.set_u32(r + 0x38, 0);
        e.mem.set_u32(r + 0x3c, 0);
        e.mem.set_u32(r, first);
        e.mem.set_u32(r + 0xc, fourth);
        let found = fn_00eb83b0(e, this, other, Ptr::new(r + 0x10), Ptr::new(r + 0x14));
        if found == 0 {
            return 0;
        }
        e.mem.set_u8(hit.addr(), 1);
        e.mem.set_u32(r + 0x38, this.addr() + 0x8c);
        e.mem.set_u32(r + 0x3c, other.addr() + 0x8c);
        e.call(
            NI_OBB_BOX_NORMAL_AT,
            &args![this.addr() + 4, r + 0x14, r + 0x20],
        );
        e.call(
            NI_OBB_BOX_NORMAL_AT,
            &args![other.addr() + 4, r + 0x14, r + 0x2c],
        );
        e.call(NI_OBB_NODE_REPORT, &args![this, r]).u32()
    })
}

// Translated from 00eb8790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBLeaf::FindCollisions` (Xbox PDB), ten words: both world boxes refreshed from their owners
/// when the stamps differ; 0 when they do not intersect. Against a leaf: the triangle collision
/// (`fn_00eb85d0`) when both `limit` words are negative, the box collision
/// (`NiOBBNode::BoxBoxCollision`) otherwise. Against a node with `depth` 0 or no children: the box
/// collision. Otherwise this leaf against the node's left child (`FindCollisions`, slot +0xc, depth
/// minus one), its result when at least 1, else against the right child, whose result below 1 is 0.
pub fn ni_obb_leaf_find_collisions(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    first: u32,
    second: u32,
    this_owner: u32,
    other_owner: u32,
    limit: u32,
    depth: u32,
    this_stamp: u32,
    other_stamp: u32,
    hit: Ptr,
) -> u32 {
    let base = this.addr();
    let o = other.addr();
    refresh(e, base, this_owner, this_stamp);
    refresh(e, o, other_owner, other_stamp);
    let overlap = e
        .call(NI_OBB_BOX_TEST_INTERSECTION, &args![base + 4, o + 4])
        .u32();
    if overlap == 0 {
        return overlap;
    }
    let box_box = |e: &mut Engine| {
        e.call(
            NI_OBB_NODE_BOX_BOX_COLLISION,
            &args![this, other, first, second, this_owner, other_owner, hit],
        )
        .u32()
    };
    if e.vcall(o, 4, &args![]).bool() {
        if (limit as i32) < 0 && (depth as i32) < 0 {
            return fn_00eb85d0(e, this, other, first, second, this_owner, other_owner, hit);
        }
        return box_box(e);
    }
    let left = e.mem.u32(o + 0x80);
    if depth == 0 || (left == 0 && e.mem.u32(o + 0x84) == 0) {
        return box_box(e);
    }
    let next = depth.wrapping_sub(1);
    let words = |child: u32| {
        args![
            child,
            first,
            second,
            this_owner,
            other_owner,
            limit,
            next,
            this_stamp,
            other_stamp,
            hit
        ]
    };
    let found = e.vcall(base, 0xc, &words(left)).u32();
    if found as i32 >= 1 {
        return found;
    }
    let right = e.mem.u32(o + 0x84);
    let found = e.vcall(base, 0xc, &words(right)).u32();
    if (found as i32) < 1 {
        0
    } else {
        found
    }
}

// Translated from 00eb8680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBLeaf::TestCollision` (Xbox PDB): with `this` a leaf and `other` a node of the other tree.
/// Both world boxes are refreshed from their owners (`this_owner`, `other_owner`) when the stamps
/// differ; false when the boxes do not intersect (`NiOBBox::TestIntersection`); true when the
/// other node is a leaf (`IsLeaf`, slot +4), when `depth` is 0 or when it has no children;
/// otherwise this leaf is tested (`TestCollision`, slot +8) against the other's left child and then
/// its right child with `depth - 1`, true when either answers true.
pub fn ni_obb_leaf_test_collision(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    this_owner: u32,
    other_owner: u32,
    context: u32,
    depth: u32,
    this_stamp: u32,
    other_stamp: u32,
) -> bool {
    let base = this.addr();
    let o = other.addr();
    refresh(e, base, this_owner, this_stamp);
    refresh(e, o, other_owner, other_stamp);
    if e.call(NI_OBB_BOX_TEST_INTERSECTION, &args![base + 4, o + 4])
        .u32()
        == 0
    {
        return false;
    }
    if e.vcall(o, 4, &args![]).bool() || depth == 0 {
        return true;
    }
    let left = e.mem.u32(o + 0x80);
    if left == 0 && e.mem.u32(o + 0x84) == 0 {
        return true;
    }
    let next = depth.wrapping_sub(1);
    let words = |child: u32| {
        args![
            child,
            this_owner,
            other_owner,
            context,
            next,
            this_stamp,
            other_stamp
        ]
    };
    if e.vcall(base, 8, &words(left)).bool() {
        return true;
    }
    let right = e.mem.u32(o + 0x84);
    e.vcall(base, 8, &words(right)).bool()
}

// Translated from 00eb8930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBLeaf::FindIntersections` (Xbox PDB): the pick ray `(origin, direction)` against this
/// leaf's triangle (its three vertex pointers at +0x8c). The world box is refreshed from `owner`
/// when the stamp differs. The ray test (`00eadb90`, culling by the pick's `m_bFrontOnly` at
/// +0x10, passed in the low byte of the stamp's word as the exe does) writes its result to `hit`;
/// on a hit a record of the pick's results (`00e99330` on pick +0x18, with `owner`) gets the point
/// (+8) and the distance (+0x14), and with `m_bReturnNormal` (+0x31) the normalized
/// `(v1 - v0) x (v2 - v0)` as its normal (+0x28). Answers 0.
pub fn ni_obb_leaf_find_intersections(
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
    let p = pick.addr();
    let cull = (stamp & 0xffff_ff00) | u32::from(e.mem.u8(p + 0x10));
    e.with_stack(0x24, |e, locals| {
        let v_out = locals.addr();
        let u_out = v_out + 4;
        let t_out = v_out + 8;
        let point = v_out + 0xc;
        let normal = v_out + 0x18;
        let found = e
            .call(
                RAY_HITS_TRIANGLE,
                &args![
                    origin,
                    direction,
                    e.mem.u32(base + 0x8c),
                    e.mem.u32(base + 0x90),
                    e.mem.u32(base + 0x94),
                    cull,
                    point,
                    t_out,
                    u_out,
                    v_out
                ],
            )
            .u8();
        e.mem.set_u8(hit.addr(), found);
        if found == 0 {
            return;
        }
        let record = e.call(NEXT_PICK_RECORD, &args![p + 0x18, owner]).u32();
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(point + offset);
            e.mem.set_u32(record.wrapping_add(8 + offset), word);
        }
        let distance = x87_quiet_bits(e.mem.u32(t_out));
        e.mem.set_u32(record.wrapping_add(0x14), distance);
        if e.mem.u8(p + 0x31) == 0 {
            return;
        }
        let v0 = e.mem.u32(base + 0x8c);
        let v1 = e.mem.u32(base + 0x90);
        let v2 = e.mem.u32(base + 0x94);
        let edge = |e: &Engine, to: u32| {
            [0u32, 4, 8].map(|offset| {
                x87_load32(x87_store32(x87_sub(
                    load_float(e, to.wrapping_add(offset)),
                    float_operand(e, v0.wrapping_add(offset)),
                )))
            })
        };
        let [ax, ay, az] = edge(e, v1);
        let [bx, by, bz] = edge(e, v2);
        let x = x87_store32(x87_sub(x87_mul(bz, ay), x87_mul(az, by)));
        let y = x87_store32(x87_sub(x87_mul(az, bx), x87_mul(bz, ax)));
        let z = x87_store32(x87_sub(x87_mul(by, ax), x87_mul(bx, ay)));
        e.mem.set_u32(normal, x);
        e.mem.set_u32(normal + 4, y);
        e.mem.set_u32(normal + 8, z);
        e.call(UNITIZE, &args![normal]);
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(normal + offset);
            e.mem.set_u32(record.wrapping_add(0x28 + offset), word);
        }
    });
    0
}

// Translated from 00eb8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niobbleaf.cpp`; the `NiOBBNode` destructor body by its use):
/// sets the `NiOBBNode` vtable and, when there is a left child, deletes it (its slot +0 with 1) and
/// then the right child when there is one. A right child without a left one is not deleted.
pub fn fn_00eb8ab0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let left = e.mem.u32(base + 0x80);
    e.mem.set_u32(base, NI_OBB_NODE_VTABLE);
    if left == 0 {
        return;
    }
    e.vcall(left, 0, &args![1u32]);
    let right = e.mem.u32(base + 0x84);
    if right != 0 {
        e.vcall(right, 0, &args![1u32]);
    }
}

/// `NiOBBLeaf`'s vtable (`010f1b70`).
const NI_OBB_LEAF_VTABLE: u32 = 0x010f_1b70;
/// The double at `01099ee0` (the float 1/3 widened).
const THIRD_01099EE0: u32 = 0x0109_9ee0;
/// The double 0.5 at `01011588`.
const HALF_01011588: u32 = 0x0101_1588;
/// `NiTransform::MakeIdentity` (`00a86bf0`, Xbox PDB; thiscall, no arguments).
const MAKE_IDENTITY: u32 = 0x00a8_6bf0;

/// A corner's coordinate widening a range: below the minimum it becomes the minimum, otherwise
/// above the maximum it becomes the maximum (a NaN changes neither).
fn widen(minimum: &mut f64, maximum: &mut f64, value: f64) {
    if *minimum > value {
        *minimum = value;
    } else if *maximum < value {
        *maximum = value;
    }
}

// Translated from 00eb6b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, five words: the root, the triangle indices, the
/// vertices, the vertices the leaf keeps and the triangle; `00eb9fa0` and `00eb5e30` call it):
/// the `NiOBBLeaf` constructor. The children, root (+0x7c) and stamp (+0x88) are cleared, the
/// vtable `010f1b70` stored and the three vertex pointers (+0x8c) set into the kept vertices by
/// the triangle's `u16` indices (positions `3 t`, `3 t + 1`, `3 t + 2` taken as `u16`). From the
/// corners in `vertices`: the box center is their mean (times the double at `01099ee0`), the
/// first axis the normalized first edge (`004a0c10`), the second the second edge minus its
/// projection on the first, normalized, the third their cross product; each corner's projection
/// on the axes (relative to the center) widens ranges that start at 0.0, the extents become half
/// the ranges and the center moves to the middle of each range (the 0.5 a double at
/// `01011588`); each value stored as a float in the exe's order. The box is transformed by the
/// identity (`00a86bf0`, `00eba480`) and the root stored. Answers `this`.
pub fn fn_00eb6b90(
    e: &mut Engine,
    this: Ptr,
    root: u32,
    triangles: Ptr,
    vertices: Ptr,
    leaf_vertices: Ptr,
    triangle: u32,
) -> Ptr {
    let n = this.addr();
    for offset in [0x7cu32, 0x88, 0x80, 0x84] {
        e.mem.set_u32(n + offset, 0);
    }
    e.mem.set_u32(n, NI_OBB_LEAF_VTABLE);
    let first = triangle.wrapping_mul(3) & 0xffff;
    let index = |e: &Engine, k: u32| -> u32 {
        let position = first.wrapping_add(k) & 0xffff;
        u32::from(e.mem.u16(triangles.addr().wrapping_add(position * 2)))
    };
    let indices = [index(e, 0), index(e, 1), index(e, 2)];
    for (k, i) in (0u32..).zip(indices) {
        let kept = leaf_vertices.addr().wrapping_add(i.wrapping_mul(12));
        e.mem.set_u32(n + 0x8c + 4 * k, kept);
    }
    let [v0, v1, v2] = indices.map(|i| vertices.addr().wrapping_add(i.wrapping_mul(12)));
    let center = n + 4;
    let a0 = n + 0x10;
    let a1 = n + 0x1c;
    let a2 = n + 0x28;
    let op = |e: &Engine, address: u32| float_operand(e, address);
    // The center: the mean of the corners.
    let s01 = [0u32, 4, 8].map(|i| round(x87_add(load(e, v1 + i), op(e, v0 + i))));
    let sum = [
        round(x87_add(load(e, v2), x87_m32(x87_store32(s01[0])))),
        round(x87_add(load(e, v2 + 4), x87_m32(x87_store32(s01[1])))),
        round(x87_add(s01[2], op(e, v2 + 8))),
    ];
    let third = x87_load64(e.mem.u64(THIRD_01099EE0));
    let middle = [
        x87_store32(x87_mul(sum[0], third)),
        x87_store32(x87_mul(sum[1], third)),
        x87_store32(x87_mul(third, sum[2])),
    ];
    write_point(e, center, middle);
    // The first axis: the first edge, normalized.
    let edge = [0u32, 4, 8].map(|i| x87_store32(x87_sub(load(e, v1 + i), op(e, v0 + i))));
    write_point(e, a0, edge);
    e.call(UNITIZE, &args![a0]);
    // The second: the second edge without its part along the first, normalized.
    let edge = [0u32, 4, 8].map(|i| x87_store32(x87_sub(load(e, v2 + i), op(e, v0 + i))));
    write_point(e, a1, edge);
    let dot = x87_add(
        x87_mul(load(e, a1 + 4), op(e, a0 + 4)),
        x87_mul(load(e, a1), op(e, a0)),
    );
    let dot = round(x87_add(dot, x87_mul(load(e, a1 + 8), op(e, a0 + 8))));
    let along = [
        x87_store32(x87_mul(load(e, a0), dot)),
        x87_store32(x87_mul(load(e, a0 + 4), dot)),
        x87_store32(x87_mul(dot, op(e, a0 + 8))),
    ];
    for (i, part) in (0u32..).zip(along) {
        let value = x87_sub(load(e, a1 + 4 * i), x87_m32(part));
        e.mem.set_u32(a1 + 4 * i, x87_store32(value));
    }
    e.call(UNITIZE, &args![a1]);
    // The third: their cross product.
    let x = x87_sub(
        x87_mul(load(e, a0 + 4), op(e, a1 + 8)),
        x87_mul(load(e, a0 + 8), op(e, a1 + 4)),
    );
    let y = x87_sub(
        x87_mul(load(e, a1), op(e, a0 + 8)),
        x87_mul(load(e, a0), op(e, a1 + 8)),
    );
    let z = x87_sub(
        x87_mul(load(e, a0), op(e, a1 + 4)),
        x87_mul(load(e, a1), op(e, a0 + 4)),
    );
    write_point(e, a2, [x87_store32(x), x87_store32(y), x87_store32(z)]);
    // The ranges of the corners' projections.
    let mut low = [0.0f64; 3];
    let mut high = [0.0f64; 3];
    for (k, corner) in [v0, v1, v2].into_iter().enumerate() {
        let [dx, dy, dz] =
            [0u32, 4, 8].map(|i| round(x87_sub(load(e, corner + i), op(e, center + i))));
        let s = x87_add(x87_mul(load(e, a0), dx), x87_mul(load(e, a0 + 4), dy));
        let p0 = round(x87_add(x87_mul(load(e, a0 + 8), dz), s));
        let s = if k == 1 {
            x87_add(x87_mul(load(e, a1 + 4), dy), x87_mul(load(e, a1), dx))
        } else {
            x87_add(x87_mul(load(e, a1 + 4), dy), x87_mul(dx, op(e, a1)))
        };
        let p1 = round(x87_add(s, x87_mul(load(e, a1 + 8), dz)));
        let s = x87_add(x87_mul(dx, load(e, a2)), x87_mul(dy, load(e, a2 + 4)));
        let p2 = round(x87_add(s, x87_mul(dz, op(e, a2 + 8))));
        for (axis, value) in [p0, p1, p2].into_iter().enumerate() {
            widen(&mut low[axis], &mut high[axis], value);
        }
    }
    let half = x87_load64(e.mem.u64(HALF_01011588));
    let size = [0usize, 1, 2].map(|axis| round(x87_sub(high[axis], low[axis])));
    let extents = size.map(|s| x87_store32(x87_mul(s, half)));
    write_point(e, n + 0x34, extents);
    let s0 = round(x87_add(low[0], high[0]));
    let s1 = round(x87_add(low[1], high[1]));
    let s2 = round(x87_add(high[2], low[2]));
    let m0 = round(x87_mul(s0, half));
    let m1 = round(x87_mul(s1, half));
    let m2 = round(x87_mul(half, s2));
    let c2 = [
        round(x87_mul(m2, op(e, a2))),
        round(x87_mul(load(e, a2 + 4), m2)),
        round(x87_mul(m2, op(e, a2 + 8))),
    ];
    let c1 = [
        round(x87_mul(load(e, a1), m1)),
        round(x87_mul(load(e, a1 + 4), m1)),
        round(x87_mul(m1, op(e, a1 + 8))),
    ];
    let c0 = [
        round(x87_mul(load(e, a0), m0)),
        round(x87_mul(load(e, a0 + 4), m0)),
        round(x87_mul(m0, op(e, a0 + 8))),
    ];
    let shift = [0usize, 1, 2].map(|i| round(x87_add(round(x87_add(c0[i], c1[i])), c2[i])));
    let x = x87_add(load(e, center), shift[0]);
    e.mem.set_u32(center, x87_store32(x));
    let y = x87_add(load(e, center + 4), shift[1]);
    e.mem.set_u32(center + 4, x87_store32(y));
    let z = x87_add(shift[2], op(e, center + 8));
    e.mem.set_u32(center + 8, x87_store32(z));
    e.with_stack(0x34, |e, transform| {
        e.call(MAKE_IDENTITY, &args![transform]);
        e.call(NI_OBB_BOX_TRANSFORM, &args![center, transform]);
    });
    e.mem.set_u32(n + 0x7c, root);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eb_6b90,
            fn_00eb6b90(Ptr, u32, Ptr, Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00eb_6b50,
            ni_obb_leaf_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00eb_6b70, fn_00eb6b70(Ptr) -> u32),
        entry!(0x00eb_72b0, fn_00eb72b0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00eb_7440,
            fn_00eb7440(f32, Ptr, f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00eb_7dc0,
            fn_00eb7dc0(Ptr, Ptr, Ptr, Ptr, f32, Ptr, f32, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00eb_8250,
            fn_00eb8250(Ptr, Ptr, Ptr, f32, Ptr, Ptr) -> u32
        ),
        entry!(0x00eb_83b0, fn_00eb83b0(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00eb_85d0,
            fn_00eb85d0(Ptr, Ptr, u32, u32, u32, u32, Ptr) -> u32
        ),
        entry!(
            0x00eb_8790,
            ni_obb_leaf_find_collisions(Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32, u32, Ptr) -> u32
        ),
        entry!(
            0x00eb_8680,
            ni_obb_leaf_test_collision(Ptr, Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00eb_8930,
            ni_obb_leaf_find_intersections(Ptr, Ptr, Ptr, Ptr, Ptr, u32, u32) -> u32
        ),
        entry!(0x00eb_8ab0, fn_00eb8ab0(Ptr)),
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

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    fn read(e: &Engine, at: u32) -> Vec<f32> {
        (0..3).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// A node with a vtable whose slots are `slots`.
    fn node(e: &mut Engine, slots: &[u32]) -> u32 {
        let vtable = e.mem.alloc(0x14);
        for (i, s) in slots.iter().enumerate() {
            e.mem.set_u32(vtable + 4 * i as u32, *s);
        }
        let n = e.mem.alloc(0x98);
        e.mem.set_u32(n, vtable);
        n
    }

    #[test]
    fn destructors_delete_the_children() {
        let mut e = Engine::new();
        e.register(0x00ab_4000, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let left = node(&mut e, &[0x00ab_4000]);
        let right = node(&mut e, &[0x00ab_4000]);
        let this = node(&mut e, &[0]);
        e.mem.set_u32(this + 0x80, left);
        e.mem.set_u32(this + 0x84, right);
        assert_eq!(e.call(0x00eb_6b70, &args![this]).u32(), 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00eb_6b50, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_OBB_NODE_VTABLE);
        assert_eq!(
            calls_to(&e, 0x00ab_4000),
            vec![vec![left, 1], vec![right, 1]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
        // Without a left child the right one is kept.
        e.mem.set_u32(this + 0x80, 0);
        e.call_log = Some(vec![]);
        e.call(0x00eb_8ab0, &args![this]);
        assert!(calls_to(&e, 0x00ab_4000).is_empty());
        e.mem.set_u32(this + 0x84, 0);
        assert_eq!(e.call(0x00eb_6b70, &args![this]).u32(), 0);
    }

    #[test]
    fn barycentric_weights() {
        let mut e = Engine::new();
        let a = floats(&mut e, &[1.0, 0.0, 0.0]);
        let b = floats(&mut e, &[0.0, 1.0, 0.0]);
        let c = floats(&mut e, &[0.0, 0.0, 0.0]);
        let p = floats(&mut e, &[0.25, 0.5, 0.0]);
        let out = e.mem.alloc(12);
        e.call(0x00eb_72b0, &args![a, b, c, p, out]);
        assert_eq!(read(&e, out), vec![0.25, 0.5, 0.25]);
    }

    #[test]
    fn edge_plane_crossings() {
        let mut e = Engine::new();
        let normal = floats(&mut e, &[0.0, 0.0, 1.0]);
        let p0 = floats(&mut e, &[0.0, 0.0, 1.0]);
        let p1 = floats(&mut e, &[2.0, 0.0, 1.0]);
        let down = floats(&mut e, &[0.0, 0.0, -1.0]);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let outs = floats(&mut e, &[9.0; 8]);
        let (ta, pa, tb, pb) = (outs, outs + 4, outs + 0x10, outs + 0x14);
        let cross = |e: &mut Engine, a: u32, b: u32, v: u32| {
            e.call(
                0x00eb_7440,
                &args![2.0f32, normal, 0.0f32, v, a, b, ta, pa, tb, pb],
            )
            .u32()
        };
        // Both ends reach the plane at t = 1.
        assert_eq!(cross(&mut e, p0, p1, down), 1);
        assert_eq!(e.mem.f32(ta), 1.0);
        assert_eq!(read(&e, pa), vec![0.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(tb), 1.0);
        assert_eq!(read(&e, pb), vec![2.0, 0.0, 0.0]);
        // Never reaching it.
        assert_eq!(cross(&mut e, p0, p1, still), 0);
        // An edge lying in the plane: its two ends at time 0.
        let q0 = floats(&mut e, &[0.0, 0.0, 0.0]);
        let q1 = floats(&mut e, &[3.0, 0.0, 0.0]);
        assert_eq!(cross(&mut e, q0, q1, down), 1);
        assert_eq!((e.mem.f32(ta), e.mem.f32(tb)), (0.0, 0.0));
        assert_eq!(read(&e, pb), vec![3.0, 0.0, 0.0]);
        // One end crossing on its path, the other side on the end edge at the full time.
        let r1 = floats(&mut e, &[2.0, 0.0, 4.0]);
        assert_eq!(cross(&mut e, p0, r1, down), 1);
        assert_eq!((e.mem.f32(ta), e.mem.f32(tb)), (1.0, 2.0));
        assert_eq!(read(&e, pa), vec![0.0, 0.0, 0.0]);
        assert!(e.mem.f32(pb + 8).abs() < 1e-6);
    }

    /// The triangle (0, 0, 0), (4, 0, 0), (0, 4, 0) as vertex pointers.
    fn big_triangle(e: &mut Engine) -> u32 {
        let v0 = floats(e, &[0.0, 0.0, 0.0]);
        let v1 = floats(e, &[4.0, 0.0, 0.0]);
        let v2 = floats(e, &[0.0, 4.0, 0.0]);
        let array = e.mem.alloc(12);
        for (i, v) in [v0, v1, v2].iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *v);
        }
        array
    }

    /// The small triangle (1, 1, z), (2, 1, z), (1, 2, z) as vertex pointers.
    fn small_triangle(e: &mut Engine, z: f32) -> u32 {
        let v0 = floats(e, &[1.0, 1.0, z]);
        let v1 = floats(e, &[2.0, 1.0, z]);
        let v2 = floats(e, &[1.0, 2.0, z]);
        let array = e.mem.alloc(12);
        for (i, v) in [v0, v1, v2].iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *v);
        }
        array
    }

    #[test]
    fn edge_against_a_triangle_keeps_the_earliest() {
        let mut e = Engine::new();
        let triangle = big_triangle(&mut e);
        let normal = floats(&mut e, &[0.0, 0.0, 1.0]);
        let p0 = floats(&mut e, &[1.0, 1.0, 1.0]);
        let p1 = floats(&mut e, &[2.0, 1.0, 1.0]);
        let down = floats(&mut e, &[0.0, 0.0, -1.0]);
        let best = floats(&mut e, &[f32::MAX, 9.0, 9.0, 9.0]);
        let point = best + 4;
        let hit = e.call(
            0x00eb_7dc0,
            &args![p0, p1, triangle, normal, 0.0f32, down, 2.0f32, best, point],
        );
        assert_eq!(hit.u32(), 1);
        assert_eq!(e.mem.f32(best), 1.0);
        assert_eq!(read(&e, point), vec![1.0, 1.0, 0.0]);
        // An earlier contact already kept: still a hit, nothing replaced.
        e.mem.set_f32(best, 0.5);
        e.mem.set_f32(point, 7.0);
        let hit = e.call(
            0x00eb_7dc0,
            &args![p0, p1, triangle, normal, 0.0f32, down, 2.0f32, best, point],
        );
        assert_eq!(hit.u32(), 1);
        assert_eq!(e.mem.f32(point), 7.0);
        // Outside the triangle.
        let q0 = floats(&mut e, &[5.0, 5.0, 1.0]);
        let q1 = floats(&mut e, &[6.0, 5.0, 1.0]);
        let hit = e.call(
            0x00eb_7dc0,
            &args![q0, q1, triangle, normal, 0.0f32, down, 2.0f32, best, point],
        );
        assert_eq!(hit.u32(), 0);
        // An edge entering the triangle from outside is cut where it enters.
        let r0 = floats(&mut e, &[-1.0, 1.0, 1.0]);
        let r1 = floats(&mut e, &[1.0, 1.0, 1.0]);
        e.mem.set_f32(best, f32::MAX);
        let hit = e.call(
            0x00eb_7dc0,
            &args![r0, r1, triangle, normal, 0.0f32, down, 2.0f32, best, point],
        );
        assert_eq!(hit.u32(), 1);
        assert_eq!(e.mem.f32(best), 1.0);
        assert_eq!(read(&e, point), vec![0.0, 1.0, 0.0]);
    }

    #[test]
    fn triangle_edges_against_a_plane_and_two_leaves() {
        let mut e = Engine::new();
        let small = small_triangle(&mut e, 1.0);
        let big = big_triangle(&mut e);
        let down = floats(&mut e, &[0.0, 0.0, -1.0]);
        let best = floats(&mut e, &[f32::MAX, 9.0, 9.0, 9.0]);
        let r = e.call(
            0x00eb_8250,
            &args![small, big, down, 2.0f32, best, best + 4],
        );
        assert_eq!(r.u32(), 1);
        assert_eq!(e.mem.f32(best), 1.0);
        assert_eq!(read(&e, best + 4), vec![1.0, 1.0, 0.0]);

        // Two leaves whose roots move: the small one falls onto the big one.
        e.map(0x0101_6000, 0x1000);
        e.mem.set_u32(NO_CONTACT_01016970, f32::MAX.to_bits());
        let root_a = floats(&mut e, &[0.0, 2.0, 0.0, 0.0, 0.0, 0.0, -1.0]);
        let root_b = floats(&mut e, &[0.0; 8]);
        let leaf_a = e.mem.alloc(0x98);
        let leaf_b = e.mem.alloc(0x98);
        e.mem.set_u32(leaf_a + 0x7c, root_a);
        e.mem.set_u32(leaf_b + 0x7c, root_b);
        for i in 0..3 {
            let a = e.mem.u32(small + 4 * i);
            e.mem.set_u32(leaf_a + 0x8c + 4 * i, a);
            let b = e.mem.u32(big + 4 * i);
            e.mem.set_u32(leaf_b + 0x8c + 4 * i, b);
        }
        let out = floats(&mut e, &[9.0; 4]);
        let r = e.call(0x00eb_83b0, &args![leaf_a, leaf_b, out, out + 4]);
        assert_eq!(r.u32(), 1);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(read(&e, out + 4), vec![1.0, 1.0, 0.0]);

        // The leaf collision reports a record of the contact.
        e.register(NI_OBB_BOX_NORMAL_AT, |e, a| {
            for i in 0..3 {
                let v = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[2] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(NI_OBB_NODE_REPORT, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x11);
            assert_eq!(e.mem.u32(a[1] + 0xc), 0x44);
            assert_eq!(e.mem.f32(a[1] + 0x10), 1.0);
            assert_eq!(e.mem.f32(a[1] + 0x2c), 1.0);
            3u32.into_ret()
        });
        let hit = e.mem.alloc(4);
        let r = e.call(
            0x00eb_85d0,
            &args![leaf_a, leaf_b, 0x11u32, 0x22u32, 0x33u32, 0x44u32, hit],
        );
        assert_eq!(r.u32(), 3);
        assert_eq!(e.mem.u8(hit), 1);
        // Not moving: no contact, no report.
        e.mem.set_f32(root_a + 0x18, 0.0);
        e.mem.set_u8(hit, 0);
        let r = e.call(
            0x00eb_85d0,
            &args![leaf_a, leaf_b, 0x11u32, 0x22u32, 0x33u32, 0x44u32, hit],
        );
        assert_eq!(r.u32(), 0);
        assert_eq!(e.mem.u8(hit), 0);
    }

    const IS_LEAF: u32 = 0x00ab_4010;
    const IS_NODE: u32 = 0x00ab_4011;
    const TEST: u32 = 0x00ab_4012;
    const FIND: u32 = 0x00ab_4013;

    fn tree_doubles(e: &mut Engine) {
        e.register(IS_LEAF, |_, _| 1u32.into_ret());
        e.register(IS_NODE, |_, _| 0u32.into_ret());
        e.register(NI_OBB_BOX_TRANSFORM, |_, _| Ret::default());
        e.register(NI_OBB_BOX_TEST_INTERSECTION, |_, _| 1u32.into_ret());
    }

    #[test]
    fn leaf_test_collision_walks_the_other_tree() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        e.register_double(TEST, |_, a| (a[1] & 1).into_ret());
        let this = node(&mut e, &[0, IS_LEAF, TEST]);
        let other_leaf = node(&mut e, &[0, IS_LEAF]);
        e.call_log = Some(vec![]);
        let words = |o: u32| args![this, o, 0x100u32, 0x200u32, 0u32, 5u32, 7u32, 8u32];
        assert!(e.call(0x00eb_8680, &words(other_leaf)).bool());
        // Both boxes refreshed from their owners' transforms and the stamps kept.
        assert_eq!(
            calls_to(&e, NI_OBB_BOX_TRANSFORM),
            vec![vec![this + 4, 0x168], vec![other_leaf + 4, 0x268]]
        );
        assert_eq!(e.mem.u32(this + 0x88), 7);
        assert_eq!(e.mem.u32(other_leaf + 0x88), 8);
        // A node: its children with depth - 1; the odd child answers true.
        let parent = node(&mut e, &[0, IS_NODE]);
        let even = e.mem.alloc(0x98) & !1;
        let odd = e.mem.alloc(0x98) | 1;
        e.mem.set_u32(parent + 0x80, even);
        e.mem.set_u32(parent + 0x84, odd);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00eb_8680, &words(parent)).bool());
        let tests = calls_to(&e, TEST);
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[0][1], even);
        assert_eq!(tests[1][5], 4);
        // Boxes apart: false; depth 0: true without the children.
        e.register(NI_OBB_BOX_TEST_INTERSECTION, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00eb_8680, &words(parent)).bool());
        e.register(NI_OBB_BOX_TEST_INTERSECTION, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let shallow = args![this, parent, 0x100u32, 0x200u32, 0u32, 0u32, 7u32, 8u32];
        assert!(e.call(0x00eb_8680, &shallow).bool());
        assert!(calls_to(&e, TEST).is_empty());
    }

    #[test]
    fn leaf_find_collisions_chooses_the_collision() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        e.register(NI_OBB_NODE_BOX_BOX_COLLISION, |_, _| 6u32.into_ret());
        e.register_double(FIND, |_, a| (a[1] & 1).into_ret());
        let this = node(&mut e, &[0, IS_LEAF, 0, FIND]);
        let other_leaf = node(&mut e, &[0, IS_LEAF]);
        let words = |o: u32, limit: i32, depth: i32| {
            args![this, o, 1u32, 2u32, 0x100u32, 0x200u32, limit, depth, 7u32, 8u32, 0x99u32]
        };
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00eb_8790, &words(other_leaf, 0, 3)).u32(), 6);
        assert_eq!(
            calls_to(&e, NI_OBB_NODE_BOX_BOX_COLLISION),
            vec![vec![this, other_leaf, 1, 2, 0x100, 0x200, 0x99]]
        );
        let parent = node(&mut e, &[0, IS_NODE]);
        let even = e.mem.alloc(0x98) & !1;
        let odd = e.mem.alloc(0x98) | 1;
        e.mem.set_u32(parent + 0x80, odd);
        e.mem.set_u32(parent + 0x84, even);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00eb_8790, &words(parent, 0, 3)).u32(), 1);
        let finds = calls_to(&e, FIND);
        assert_eq!(finds.len(), 1);
        assert_eq!(&finds[0][1..], &[odd, 1, 2, 0x100, 0x200, 0, 2, 7, 8, 0x99]);
        e.mem.set_u32(parent + 0x80, even);
        assert_eq!(e.call(0x00eb_8790, &words(parent, 0, 3)).u32(), 0);
        // No overlap answers the test's 0.
        e.register(NI_OBB_BOX_TEST_INTERSECTION, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00eb_8790, &words(parent, 0, 3)).u32(), 0);
    }

    #[test]
    fn leaf_find_intersections_records_the_hit() {
        let mut e = Engine::new();
        e.register(NI_OBB_BOX_TRANSFORM, |_, _| Ret::default());
        e.register(RAY_HITS_TRIANGLE, |e, a| {
            assert_eq!(a[5] & 0xff, 1);
            e.mem.set_f32(a[6] + 8, -2.0);
            e.mem.set_f32(a[7], 3.5);
            1u32.into_ret()
        });
        let record = e.mem.alloc(0x44);
        e.register_double(NEXT_PICK_RECORD, move |_, _| record.into_ret());
        e.register(UNITIZE, |e, a| {
            let z = e.mem.f32(a[0] + 8);
            e.mem.set_f32(a[0] + 8, z.signum());
            Ret::default()
        });
        let this = node(&mut e, &[0]);
        let v0 = floats(&mut e, &[0.0, 0.0, 0.0]);
        let v1 = floats(&mut e, &[1.0, 0.0, 0.0]);
        let v2 = floats(&mut e, &[0.0, 1.0, 0.0]);
        e.mem.set_u32(this + 0x8c, v0);
        e.mem.set_u32(this + 0x90, v1);
        e.mem.set_u32(this + 0x94, v2);
        let pick = e.mem.alloc(0x40);
        e.mem.set_u8(pick + 0x10, 1);
        e.mem.set_u8(pick + 0x31, 1);
        let hit = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00eb_8930,
            &args![this, 0x10u32, 0x20u32, pick, hit, 0x300u32, 0x0055_0004u32],
        );
        assert_eq!(r.u32(), 0);
        assert_eq!(e.mem.u8(hit), 1);
        assert_eq!(
            calls_to(&e, NEXT_PICK_RECORD),
            vec![vec![pick + 0x18, 0x300]]
        );
        assert_eq!(e.mem.f32(record + 0x10), -2.0);
        assert_eq!(e.mem.f32(record + 0x14), 3.5);
        assert_eq!(read(&e, record + 0x28), vec![0.0, 0.0, 1.0]);
        let ray = &calls_to(&e, RAY_HITS_TRIANGLE)[0];
        assert_eq!(ray[5], 0x0055_0001);
        assert_eq!(e.mem.u32(this + 0x88), 0x0055_0004);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn leaf_box_fits_its_triangle() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0109_9000, 0x1000);
        e.mem.set_f64(HALF_01011588, 0.5);
        e.mem.set_u64(THIRD_01099EE0, 0x3fd5_5555_6000_0000);
        e.register(UNITIZE, |e, a| {
            let v = read(e, a[0], 3);
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, x) in (0u32..).zip(v) {
                e.mem.set_f32(a[0] + 4 * i, x / length);
            }
            Ret::default()
        });
        e.register(MAKE_IDENTITY, |_, _| Ret::default());
        e.register(NI_OBB_BOX_TRANSFORM, |_, _| Ret::default());
        // Triangle 1 uses indices 3, 1, 2 (positions 3..6).
        let triangles = e.mem.alloc(12);
        for (i, v) in (0u32..).zip([0u16, 0, 0, 3, 1, 2]) {
            e.mem.set_u16(triangles + 2 * i, v);
        }
        let vertices = e.mem.alloc(48);
        for (i, v) in (0u32..).zip([
            9.0f32, 9.0, 9.0, 3.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0,
        ]) {
            e.mem.set_f32(vertices + 4 * i, v);
        }
        let kept = 0x0004_0000u32;
        let this = e.mem.alloc(0x98);
        e.call_log = Some(vec![]);
        let r = e
            .call(
                0x00eb_6b90,
                &args![this, 0x77u32, triangles, vertices, kept, 1u32],
            )
            .u32();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this), NI_OBB_LEAF_VTABLE);
        assert_eq!(e.mem.u32(this + 0x7c), 0x77);
        assert_eq!(e.mem.u32(this + 0x8c), kept + 36);
        assert_eq!(e.mem.u32(this + 0x90), kept + 12);
        assert_eq!(e.mem.u32(this + 0x94), kept + 24);
        assert_eq!(
            read(&e, this + 0x10, 9),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(read(&e, this + 4, 3), vec![1.5, 1.5, 0.0]);
        assert_eq!(read(&e, this + 0x34, 3), vec![1.5, 1.5, 0.0]);
        let log = e.call_log.clone().unwrap();
        let transform: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == NI_OBB_BOX_TRANSFORM)
            .collect();
        assert_eq!(transform.len(), 1);
        assert_eq!(transform[0].1[0], this + 4);
    }
}
