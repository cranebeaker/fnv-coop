//! The differential test of every typed function of the culling family (`engine_typed::culling`)
//! against its translation: here because the translations live in the part crates, and this crate
//! registers them all. One scene serves every test: a culling process with six frustum planes (the
//! box `|x|, |y|, |z| <= 10`, normals inward), objects with world bounds and transforms, a node with
//! children (one slot empty), a multi-bound node and box, a compound frustum with a short program.
//! The float cases sweep the harness's operand classes (NaN, infinities, zeros, denormals, huge,
//! values that round) through centres, radii, plane constants, scales and matrix entries, under both
//! control words (the family's row is `Threads::Both`).
//!
//! What stands in: virtual functions the walk calls on objects, processes, accumulators and shapes
//! (test functions past `.text` that log what they get), the bound pool, the thread's scrap heap,
//! the skin instance's bound update and the interlocked imports. Everything else runs translated.

use engine as _;
use engine_core::prelude::*;
use engine_core::typed::diff::{Case, Diff, Input};
use engine_typed::culling::{self, bound, compound, planes, process, update, visible};

/// Where the test's virtual functions are registered: past `.text`, one per vtable slot.
const VT_FN: u32 = 0x00ff_2000;
/// The log the stand-ins write (a count, then `(tag, word)` pairs), at a fixed guest address.
const LOG: u32 = 0x0300_0000;
/// The thread's scrap heap the stand-in answers.
const HEAP: u32 = 0x0300_2000;

/// Callees and imports.
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
const SCRAP_HEAP: u32 = 0x00aa_42e0;
const SCRAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_FREE: u32 = 0x00aa_5610;
const SKIN_BOUND: u32 = 0x00a8_6530;
const INCREMENT: u32 = 0x00fd_f1c0;
const DECREMENT: u32 = 0x00fd_f1c4;
const GETPTD_NOEXIT: u32 = 0x00ed_1165;

/// The real functions the scene's vtables point at.
const PROCESS: u32 = 0x00c4_ee90;
const APPEND_VIRTUAL: u32 = 0x00c4_f1d0;
const NODE_ON_VISIBLE: u32 = 0x00a5_dbe0;
const GEOMETRY_ON_VISIBLE: u32 = 0x00a7_fd90;
/// `NiPlane::NiPlane(normal, point)`: the typed `NiFrustumPlanes::Set` does its work in place.
const PLANE_THROUGH: u32 = 0x00a6_9990;

/// The most calls one run makes here is a few hundred: a version that loops stops at this many.
const MAX_CALLS: usize = 5_000;

/// Cases each test adds with every float argument at once a random float with a full mantissa (the
/// harness's float sweep): the ordinary cases vary one at a time against plain others, which misses a
/// rounding that only shows when several operands carry low bits (batch 2's Merge mutant).
const FLOAT_SWEEP: usize = 64;

/// Argument `i` of the case, a float, as its bits.
fn bits(case: &Case, i: usize) -> u32 {
    case.f32(i).to_bits()
}

/// The float bit patterns a scene value takes in the cases that don't sweep all of them: an
/// ordinary value, zero, a NaN.
const FEW_FLOATS: &[u32] = &[0x3f80_0000, 0, 0x7fc0_0000, 0xc120_0000];

fn ret(eax: u32) -> Ret {
    Ret {
        eax,
        ..Ret::default()
    }
}

/// Logs `(tag, word)`.
fn log(e: &mut Engine, tag: u32, word: u32) {
    let n = e.mem.u32(LOG);
    e.mem.set_u32(LOG + 4 + 8 * n, tag);
    e.mem.set_u32(LOG + 8 + 8 * n, word);
    e.mem.set_u32(LOG, n + 1);
}

/// The stand-ins and the constants the code reads from the exe's data.
fn stand_ins(e: &mut Engine) {
    e.map(LOG, 0x3000);
    // OnVisible, Process, AppendVirtual: logged.
    e.register(VT_FN + 0xd4, |e, a| {
        log(e, 0xd4, a[0]);
        ret(0)
    });
    e.register(VT_FN + 0x44, |e, a| {
        log(e, 0x44, a[1]);
        ret(0)
    });
    e.register(VT_FN + 0x4c, |e, a| {
        log(e, 0x4c, a[1]);
        ret(0)
    });
    // SetWorldBound: the word into +0x20.
    e.register(VT_FN + 0x78, |e, a| {
        e.mem.set_u32(a[0] + 0x20, a[1]);
        ret(0)
    });
    // The collision object's update, the accumulator's camera, a deleting destructor: logged.
    e.register(VT_FN + 0x90, |e, a| {
        log(e, 0x90, a[1]);
        ret(0)
    });
    e.register(VT_FN + 0x8c, |e, a| {
        log(e, 0x8c, a[1]);
        ret(0)
    });
    e.register(VT_FN + 0x04, |e, a| {
        log(e, 0x04, a[0]);
        ret(0)
    });
    // The accumulator's RegisterObject: its word at +0x10 says whether it accepts.
    e.register(VT_FN + 0x98, |e, a| {
        log(e, 0x98, a[1]);
        ret(e.mem.u32(a[0] + 0x10))
    });
    // A reference's fade node: its word at +0x10.
    e.register(VT_FN + 0x10, |e, a| ret(e.mem.u32(a[0] + 0x10)));
    // A shape's WithinFrustum: its word at +0x24 (past a box), garbage in the high bytes.
    e.register(VT_FN + 0x9c, |e, a| {
        log(e, 0x9c, a[1]);
        ret(e.mem.u32(a[0] + 0x24) | 0x5a00)
    });
    e.register(POOL_ALLOCATE, |e, a| ret(e.mem.alloc(a[1])));
    e.register(SCRAP_HEAP, |_, _| ret(HEAP));
    e.register(SCRAP_ALLOCATE, |e, a| {
        let block = e.mem.alloc(a[1].max(4));
        // What the heap leaves in a new block.
        for i in 0..a[1] / 4 {
            e.mem.set_u32(block + 4 * i, 0xcdcd_cdcd);
        }
        ret(block)
    });
    e.register(SCRAP_FREE, |e, a| {
        log(e, 0x5610, a[1]);
        ret(0)
    });
    // The skin's update: logged, and the bound's radius doubled as words so it shows.
    e.register(SKIN_BOUND, |e, a| {
        log(e, 0x6530, a[1]);
        let r = e.mem.u32(a[1] + 0xc);
        e.mem.set_u32(a[1] + 0xc, r ^ 0x0080_0000);
        ret(0)
    });
    e.register(INCREMENT, |e, a| {
        let n = e.mem.u32(a[0]).wrapping_add(1);
        e.mem.set_u32(a[0], n);
        ret(n)
    });
    e.register(DECREMENT, |e, a| {
        let n = e.mem.u32(a[0]).wrapping_sub(1);
        e.mem.set_u32(a[0], n);
        ret(n)
    });
    // The square root's error path (an infinite or NaN operand) sets `errno`: the thread has no
    // CRT data block here, so it goes to the static slot.
    e.register(GETPTD_NOEXIT, |_, _| ret(0));
    e.map(0x011c_1000, 0x1000);
    // The exe's constants: 0.5 and 0.0 as doubles, the merge threshold 1e-6, the scrap alignment;
    // the empty bound and the zero point are zero.
    e.map(0x0101_1000, 0x2000);
    e.mem.set_u64(0x0101_1588, 0.5f64.to_bits());
    e.mem.set_u64(0x0101_2060, 0);
    e.map(0x011a_b000, 0x1000);
    e.mem.set_u32(0x011a_b3c0, 0x3586_37bd);
    // The unit axes the box's corners are made from (`011a946c`, three points).
    e.map(0x011a_9000, 0x1000);
    for (i, w) in [ONE, 0, 0, 0, ONE, 0, 0, 0, ONE].iter().enumerate() {
        e.mem.set_u32(0x011a_946c + 4 * i as u32, *w);
    }
    // The double 1.0 the perspective planes add.
    e.mem.set_u64(0x0101_2070, 1.0f64.to_bits());
    e.map(0x010a_2000, 0x1000);
    e.mem.set_u32(0x010a_2720, 0x10);
    e.map(0x011f_4000, 0x3000);
    e.map(0x0101_f000, 0x1000);
    e.map(0x0109_c000, 0x1000);
}

/// A vtable of 0x100 slots, each the stand-in for its slot unless `real` says otherwise.
fn vtable(e: &mut Engine, real: &[(u32, u32)]) -> u32 {
    let vt = e.mem.alloc(0x100);
    for slot in (0..0x100).step_by(4) {
        let target = real
            .iter()
            .find(|(s, _)| *s == slot)
            .map_or(VT_FN + slot, |(_, a)| *a);
        e.mem.set_u32(vt + slot, target);
    }
    vt
}

/// A bound of four float bit patterns.
fn bound_of(e: &mut Engine, x: u32, y: u32, z: u32, r: u32) -> u32 {
    let b = e.mem.alloc(0x10);
    e.mem.set_u32(b, x);
    e.mem.set_u32(b + 4, y);
    e.mem.set_u32(b + 8, z);
    e.mem.set_u32(b + 0xc, r);
    b
}

/// A transform: rotation `m` (nine float patterns), translation, scale.
fn transform_at(e: &mut Engine, at: u32, m: [u32; 9], t: [u32; 3], s: u32) {
    for (i, w) in m.iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, *w);
    }
    for (i, w) in t.iter().enumerate() {
        e.mem.set_u32(at + 0x24 + 4 * i as u32, *w);
    }
    e.mem.set_u32(at + 0x30, s);
}

/// A rotation of a few degrees about z, as float patterns (cos, sin of 0.1).
const ROTATION: [u32; 9] = [
    0x3f7e_8a6e,
    0xbdcc_6c67,
    0,
    0x3dcc_6c67,
    0x3f7e_8a6e,
    0,
    0,
    0,
    0x3f80_0000,
];
const ONE: u32 = 0x3f80_0000;
const TWO: u32 = 0x4000_0000;
const HALF_F: u32 = 0x3f00_0000;
const MINUS_TEN: u32 = 0xc120_0000;

/// The rotation's entries but `skip`, for the sweep to fill (the ordinary cases keep [`ROTATION`]).
fn swept_rotation(skip: usize) -> impl Iterator<Item = Input> {
    ROTATION
        .into_iter()
        .enumerate()
        .filter(move |(i, _)| *i != skip)
        .map(|(_, b)| Input::F32Swept(b))
}

/// The rotation from the case: entry `skip` from argument `at`, the other eight from argument `from`
/// on ([`swept_rotation`]).
fn rotation_from(case: &Case, skip: usize, at: usize, from: usize) -> [u32; 9] {
    let mut k = from;
    std::array::from_fn(|i| {
        if i == skip {
            bits(case, at)
        } else {
            k += 1;
            bits(case, k - 1)
        }
    })
}

/// The bound update tests' inputs: the centre's x, the radius, the scale, rotation entry `skip`
/// (arguments 0-3); swept: the centre's y and z (4, 5), the other rotation entries (6-13), the
/// translation (14-16).
fn bound_inputs(skip: usize) -> Vec<Input> {
    [Input::F32, Input::F32, Input::F32, Input::F32]
        .into_iter()
        .chain([TWO, MINUS_TEN].map(Input::F32Swept))
        .chain(swept_rotation(skip))
        .chain([ONE, 0x3dcc_cccd, MINUS_TEN].map(Input::F32Swept))
        .collect()
}

/// A transform from [`bound_inputs`]' arguments at `t`.
fn bound_transform_at(e: &mut Engine, t: u32, case: &Case, skip: usize) {
    let m = rotation_from(case, skip, 3, 6);
    let translation = [bits(case, 14), bits(case, 15), bits(case, 16)];
    transform_at(e, t, m, translation, bits(case, 2));
}

/// An object: `size` bytes, vtable `vt`, world bound `bound`, flags; its local transform the
/// rotation, its world transform scaled by 2 and moved.
fn object(e: &mut Engine, size: u32, vt: u32, bound: u32, flags: u32) -> u32 {
    let o = e.mem.alloc(size);
    e.mem.set_u32(o, vt);
    e.mem.set_u32(o + 4, 1);
    e.mem.set_u32(o + 0x20, bound);
    e.mem.set_u32(o + 0x30, flags);
    transform_at(e, o + 0x34, ROTATION, [ONE, TWO, HALF_F], ONE);
    transform_at(e, o + 0x68, ROTATION, [TWO, ONE, MINUS_TEN], TWO);
    o
}

/// Six planes, the box `|x|, |y|, |z| <= 10` with inward normals, and an active mask, at `at`.
fn planes_at(e: &mut Engine, at: u32, active: u32) {
    let normals: [[u32; 3]; 6] = [
        [ONE, 0, 0],
        [0xbf80_0000, 0, 0],
        [0, ONE, 0],
        [0, 0xbf80_0000, 0],
        [0, 0, ONE],
        [0, 0, 0xbf80_0000],
    ];
    for (i, n) in normals.iter().enumerate() {
        let p = at + 0x10 * i as u32;
        e.mem.set_u32(p, n[0]);
        e.mem.set_u32(p + 4, n[1]);
        e.mem.set_u32(p + 8, n[2]);
        e.mem.set_u32(p + 0xc, MINUS_TEN);
    }
    e.mem.set_u32(at + 0x60, active);
}

/// The scene's parts.
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Scene {
    /// A `BSCullingProcess` (0xc8 bytes): mode 0, all planes active, no compound frustum, an
    /// accumulator that accepts.
    process: u32,
    accumulator: u32,
    /// A node (0xac bytes) with four child slots: two geometries, an empty slot, a node.
    node: u32,
    children: [u32; 4],
    /// A geometry (0xc8 bytes) with model data.
    geometry: u32,
    /// The vtables: a node's, a geometry's (both real `OnVisible`), and plain stand-ins.
    node_vt: u32,
    geometry_vt: u32,
    stand_in_vt: u32,
}

fn scene(e: &mut Engine) -> Scene {
    stand_ins(e);
    let stand_in_vt = vtable(e, &[]);
    let node_vt = vtable(e, &[(0xd4, NODE_ON_VISIBLE)]);
    let geometry_vt = vtable(e, &[(0xd4, GEOMETRY_ON_VISIBLE)]);
    let process_vt = vtable(e, &[(0x44, PROCESS), (0x4c, APPEND_VIRTUAL)]);
    let accumulator = e.mem.alloc(0x20);
    e.mem.set_u32(accumulator, stand_in_vt);
    e.mem.set_u32(accumulator + 4, 1);
    e.mem.set_u32(accumulator + 0x10, 1);
    let process = e.mem.alloc(0xc8);
    e.mem.set_u32(process, process_vt);
    planes_at(e, process + 0x2c, 0x3f);
    e.mem.set_u32(process + 0xc4, accumulator);
    let g1_bound = bound_of(e, ONE, ONE, ONE, ONE);
    let g1 = object(e, 0xc8, geometry_vt, g1_bound, 0);
    let g2_bound = bound_of(e, 0x41a0_0000, 0, 0, ONE);
    let g2 = object(e, 0xc8, geometry_vt, g2_bound, 0);
    let inner_bound = bound_of(e, 0, 0, 0, TWO);
    let inner = object(e, 0xac, node_vt, inner_bound, 0);
    let node_bound = bound_of(e, 0, 0, 0, 0x4120_0000);
    let node = object(e, 0xac, node_vt, node_bound, 0);
    let slots = e.mem.alloc(0x10);
    let children = [g1, g2, 0, inner];
    for (i, c) in children.iter().enumerate() {
        e.mem.set_u32(slots + 4 * i as u32, *c);
    }
    e.mem.set_u32(node + 0x9c, stand_in_vt);
    e.mem.set_u32(node + 0xa0, slots);
    e.mem.set_u16(node + 0xa4, 4);
    e.mem.set_u16(node + 0xa6, 4);
    e.mem.set_u16(node + 0xa8, 3);
    for child in [g1, g2, inner] {
        e.mem.set_u32(child + 0x18, node);
    }
    // The geometry's model data: its bound at +0x10.
    let data = e.mem.alloc(0x40);
    e.mem.set_u32(data + 0x10, ONE);
    e.mem.set_u32(data + 0x14, TWO);
    e.mem.set_u32(data + 0x18, MINUS_TEN);
    e.mem.set_u32(data + 0x1c, HALF_F);
    e.mem.set_u32(g1 + 0xb8, data);
    e.mem.set_u32(g2 + 0xb8, data);
    Scene {
        process,
        accumulator,
        node,
        children,
        geometry: g1,
        node_vt,
        geometry_vt,
        stand_in_vt,
    }
}

/// What every test adds to its comparison: the families and the call budget.
trait Common {
    fn common(self) -> Diff;
}

impl Common for Diff {
    fn common(self) -> Diff {
        self.family(&culling::CULLING)
            .family(&leaf::LEAVES)
            // The transform product and the box's corners are the dispatch family's (its own test).
            .goes_by_address(&[0x0062_c250, 0x00c3_9270])
            .max_calls(MAX_CALLS)
            .float_sweep(FLOAT_SWEEP)
    }
}

#[test]
fn plane_distance() {
    // Normal x, the constant, the point's x; the sweep also fills the normal's y and z and the
    // point's.
    Diff::new(entry!(
        0x004b_6160,
        bound::plane_distance(Ptr<_>, Ptr<_>) -> f32
    ))
    .common()
    .inputs(&[
        Input::F32,
        Input::F32,
        Input::F32,
        Input::F32Swept(HALF_F),
        Input::F32Swept(0xbfc0_0000),
        Input::F32Swept(TWO),
        Input::F32Swept(ONE),
    ])
    .max_cases(256)
    .setup(|e, case| {
        stand_ins(e);
        let plane = bound_of(
            e,
            bits(case, 0),
            bits(case, 3),
            bits(case, 4),
            bits(case, 1),
        );
        let point = bound_of(e, bits(case, 2), bits(case, 5), bits(case, 6), 0);
        vec![plane, point]
    })
    .run()
    .assert_same();
}

#[test]
fn which_side() {
    // The centre's x, the radius, the plane's constant; the sweep also fills the centre's y and z
    // and the plane's normal.
    Diff::new(entry!(
        0x004b_6100,
        bound::which_side(Ptr<_>, Ptr<_>) -> u32
    ))
    .common()
    .inputs(&[
        Input::F32,
        Input::F32,
        Input::F32,
        Input::F32Swept(ONE),
        Input::F32Swept(0),
        Input::F32Swept(ONE),
        Input::F32Swept(0),
        Input::F32Swept(0),
    ])
    .max_cases(256)
    .setup(|e, case| {
        stand_ins(e);
        let b = bound_of(
            e,
            bits(case, 0),
            bits(case, 3),
            bits(case, 4),
            bits(case, 1),
        );
        let plane = bound_of(
            e,
            bits(case, 5),
            bits(case, 6),
            bits(case, 7),
            bits(case, 2),
        );
        vec![b, plane]
    })
    .run()
    .assert_same();
}

#[test]
fn merge() {
    // This centre's x, this radius, the other's centre x, the other's radius; the sweep also fills
    // both centres' y and z, so all three differences carry low bits at once.
    Diff::new(entry!(0x00a7_f3f0, bound::merge(Ptr<_>, Ptr<_>)))
        .common()
        .inputs(&[
            Input::F32,
            Input::F32,
            Input::F32,
            Input::F32,
            Input::F32Swept(ONE),
            Input::F32Swept(0),
            Input::F32Swept(0),
            Input::F32Swept(HALF_F),
        ])
        .max_cases(384)
        .setup(|e, case| {
            stand_ins(e);
            let a = bound_of(
                e,
                bits(case, 0),
                bits(case, 4),
                bits(case, 5),
                bits(case, 1),
            );
            let b = bound_of(
                e,
                bits(case, 2),
                bits(case, 6),
                bits(case, 7),
                bits(case, 3),
            );
            vec![a, b]
        })
        .run()
        .assert_same();
}

#[test]
fn do_culling() {
    // The bound: none, or its centre's x and radius; the active planes.
    Diff::new(entry!(0x00a6_94e0, process::do_culling(Ptr<_>, Ptr<_>)))
        .common()
        .inputs(&[
            Input::Of(&[0, 1]),
            Input::F32,
            Input::F32,
            Input::Of(&[0x3f, 0x2a, 0]),
        ])
        .max_cases(256)
        .setup(|e, case| {
            let s = scene(e);
            let b = if case.word(0) == 0 {
                0
            } else {
                bound_of(e, case.f32(1).to_bits(), ONE, 0, case.f32(2).to_bits())
            };
            let o = object(e, 0xc8, s.stand_in_vt, b, 0);
            e.mem.set_u32(s.process + 0x8c, case.word(3));
            vec![s.process, o]
        })
        .run()
        .assert_same();
}

#[test]
fn bs_process() {
    // Mode; flags; compound frustum (none, in use, no operations); active planes.
    Diff::new(entry!(0x00c4_ee90, process::bs_process(Ptr<_>, Ptr<_>)))
        .common()
        .inputs(&[
            Input::Of(&[0, 1, 2, 3, 4]),
            Input::Of(&[0, 0x1000, 0x0010_1000, 0x800, 0x0010_0000]),
            Input::Of(&[0, 1, 2]),
            Input::Of(&[0x3f, 0]),
        ])
        .setup(|e, case| {
            let s = scene(e);
            e.mem.set_u32(s.process + 0x90, case.word(0));
            let b = bound_of(e, ONE, 0, 0, ONE);
            let o = object(e, 0xc8, s.stand_in_vt, b, case.word(1));
            let frustum = match case.word(2) {
                0 => 0,
                n => compound_frustum(e, if n == 1 { 6 } else { 0 }, 0, 0x3f),
            };
            e.mem.set_u32(s.process + 0xc0, frustum);
            e.mem.set_u32(s.process + 0x8c, case.word(3));
            vec![s.process, o]
        })
        .run()
        .assert_same();
}

/// A compound frustum: the view frustum (the scene's box, `active` planes), two plane sets (the
/// box moved by 5 along x, and a small occluder box at the origin), and a program: a portal test on
/// set 0 (pass to 2, fail to 5), an occluder test on set 1 (pass to 4, fail to 5), pass, fail.
/// `ops` operations are in use (0: none); `skip` skips the view frustum.
fn compound_frustum(e: &mut Engine, ops: i32, skip: u8, active: u32) -> u32 {
    let f = e.mem.alloc(0xa4);
    let sets = e.mem.alloc(0x64 * 2);
    planes_at(e, sets, 0x3f);
    e.mem.set_u32(sets + 0xc, 0xc0a0_0000);
    planes_at(e, sets + 0x64, 0x3f);
    for i in 0..6 {
        e.mem.set_u32(sets + 0x64 + 0x10 * i + 0xc, 0xbf80_0000);
    }
    e.mem.set_u32(f + 8, sets);
    e.mem.set_u32(f + 0xc, 2);
    e.mem.set_u32(f + 0x10, 2);
    let program: [[u32; 3]; 6] = [
        [7, 2, 5],
        [0, 0, 0],
        [8, 4, 5],
        [1, 0, 0],
        [2, 0, 0],
        [3, 0, 0],
    ];
    let code = e.mem.alloc(0xc * 6);
    for (i, op) in program.iter().enumerate() {
        for (j, w) in op.iter().enumerate() {
            e.mem.set_u32(code + 0xc * i as u32 + 4 * j as u32, *w);
        }
    }
    e.mem.set_u32(f + 0x18, code);
    e.mem.set_u32(f + 0x1c, 6);
    e.mem.set_u32(f + 0x20, 6);
    e.mem.set_u32(f + 0x24, 1);
    e.mem.set_u32(f + 0x28, ops as u32);
    e.mem.set_u32(f + 0x2c, 0);
    planes_at(e, f + 0x30, active);
    e.mem.set_u8(f + 0xa0, skip);
    f
}

#[test]
fn append_virtual() {
    // Accumulator: none, accepts, refuses; the geometry's name: none or one.
    Diff::new(entry!(0x00c4_f1d0, process::append_virtual(Ptr<_>, Ptr<_>)))
        .common()
        .inputs(&[Input::Of(&[0, 1, 2]), Input::Flag])
        .setup(|e, case| {
            let s = scene(e);
            match case.word(0) {
                0 => e.mem.set_u32(s.process + 0xc4, 0),
                2 => e.mem.set_u32(s.accumulator + 0x10, 0),
                _ => {}
            }
            let name = if case.word(1) != 0 { 0x0101_f7b0 } else { 0 };
            e.mem.set_u32(s.geometry + 8, name);
            vec![s.process, s.geometry]
        })
        .run()
        .assert_same();
}

/// A camera: its world transform at +0x68 (the object's), its frustum at +0xdc, its position at
/// +0x8c.
fn camera(e: &mut Engine, s: &Scene, left: u32) -> u32 {
    let c = object(e, 0x120, s.stand_in_vt, 0, 0);
    let frustum = [left, ONE, ONE, 0xbf80_0000, ONE, 0x447a_0000, 0];
    for (i, w) in frustum.iter().enumerate() {
        e.mem.set_u32(c + 0xdc + 4 * i as u32, *w);
    }
    e.mem.set_u32(c + 0x8c, TWO);
    e.mem.set_u32(c + 0x90, ONE);
    e.mem.set_u32(c + 0x94, MINUS_TEN);
    c
}

#[test]
fn set_frustum() {
    // No camera or one; the frustum's left edge.
    Diff::new(entry!(0x00a6_94a0, process::set_frustum(Ptr<_>, Ptr<_>)))
        .common()
        .ignore_calls(&[PLANE_THROUGH])
        .inputs(&[Input::Flag, Input::F32])
        .setup(|e, case| {
            let s = scene(e);
            let c = camera(e, &s, case.f32(1).to_bits());
            e.mem
                .set_u32(s.process + 0xc, if case.word(0) != 0 { c } else { 0 });
            e.mem.set_u32(s.process + 0x8c, 0x5);
            vec![s.process, c + 0xdc]
        })
        .run()
        .assert_same();
}

#[test]
fn bs_process_scene() {
    // Camera; scene; visible set; the accumulator: the process's, none (the default one), none at all.
    Diff::new(entry!(
        0x00c4_f070,
        process::bs_process_scene(Ptr<_>, u32, Ptr<_>, u32)
    ))
    .common()
    .ignore_calls(&[PLANE_THROUGH])
    .inputs(&[Input::Flag, Input::Flag, Input::Flag, Input::Of(&[0, 1, 2])])
    .setup(|e, case| {
        let s = scene(e);
        let c = camera(e, &s, 0xbf80_0000);
        let default = e.mem.alloc(0x10);
        e.mem.set_u32(0x011f_4748, default);
        match case.word(3) {
            1 => {
                e.mem.set_u32(s.process + 0xc4, 0);
                e.mem.set_u32(default + 8, s.accumulator);
            }
            2 => e.mem.set_u32(s.process + 0xc4, 0),
            _ => {}
        }
        let visible_set = e.mem.alloc(0x10);
        e.mem.set_u32(s.process + 8, 0x1234);
        vec![
            s.process,
            if case.word(0) != 0 { c } else { 0 },
            if case.word(1) != 0 { s.node } else { 0 },
            if case.word(2) != 0 { visible_set } else { 0 },
        ]
    })
    .run()
    .assert_same();
}

#[test]
fn fade_culler_process() {
    // A fade node or none; its bound's centre x and radius.
    Diff::new(entry!(
        0x00bc_2b30,
        process::fade_culler_process(Ptr<_>, u32)
    ))
    .common()
    .inputs(&[Input::Flag, Input::F32, Input::F32])
    .max_cases(256)
    .setup(|e, case| {
        let s = scene(e);
        let b = bound_of(e, case.f32(1).to_bits(), 0, ONE, case.f32(2).to_bits());
        let node = object(e, 0xac, s.stand_in_vt, b, 0x0010_0001);
        let reference = e.mem.alloc(0x20);
        e.mem.set_u32(reference, s.stand_in_vt);
        e.mem
            .set_u32(reference + 0x10, if case.word(0) != 0 { node } else { 0 });
        vec![s.process, reference]
    })
    .run()
    .assert_same();
}

/// A multi-bound box: centre `(cx, 0, 0)`, half extents `(1, 1, hz)`; `answer` is what its
/// `WithinFrustum` stand-in says.
fn aabb(e: &mut Engine, s: &Scene, cx: u32, hz: u32, result: u32, answer: u32) -> u32 {
    let b = e.mem.alloc(0x28);
    e.mem.set_u32(b, s.stand_in_vt);
    e.mem.set_u32(b + 4, 1);
    e.mem.set_u32(b + 8, result);
    e.mem.set_u32(b + 0xc, cx);
    e.mem.set_u32(b + 0x18, ONE);
    e.mem.set_u32(b + 0x1c, ONE);
    e.mem.set_u32(b + 0x20, hz);
    e.mem.set_u32(b + 0x24, answer);
    b
}

#[test]
fn within_frustum() {
    // The box's centre x, its third half extent, the active planes.
    Diff::new(entry!(
        0x00c3_87f0,
        process::within_frustum(Ptr<_>, Ptr<_>) -> bool
    ))
    .common()
    .inputs(&[Input::F32, Input::F32, Input::Of(&[0x3f, 0x2, 0])])
    .max_cases(192)
    .setup(|e, case| {
        let s = scene(e);
        let b = aabb(e, &s, case.f32(0).to_bits(), case.f32(1).to_bits(), 0, 0);
        e.mem.set_u32(s.process + 0x8c, case.word(2));
        vec![b, s.process + 0x2c]
    })
    .run()
    .assert_same();
}

#[test]
fn cull() {
    // Flags: app-culled or not; the process: the real one, or a stand-in.
    Diff::new(entry!(0x00a5_9e00, visible::cull(Ptr<_>, Ptr<_>)))
        .common()
        .inputs(&[Input::Of(&[0, 1, 0x1001, 0x800]), Input::Flag])
        .setup(|e, case| {
            let s = scene(e);
            e.mem.set_u32(s.geometry + 0x30, case.word(0));
            let process = if case.word(1) != 0 {
                s.process
            } else {
                let p = e.mem.alloc(0xc8);
                e.mem.set_u32(p, s.stand_in_vt);
                p
            };
            vec![s.geometry, process]
        })
        .run()
        .assert_same();
}

#[test]
fn node_on_visible() {
    // The node's bound: none or with a radius; the real process or a stand-in.
    Diff::new(entry!(
        0x00a5_dbe0,
        visible::node_on_visible(Ptr<_>, Ptr<_>)
    ))
    .common()
    .inputs(&[Input::Of(&[0, 1]), Input::F32, Input::Flag])
    .setup(|e, case| {
        let s = scene(e);
        let b = if case.word(0) != 0 {
            bound_of(e, 0, 0, 0, case.f32(1).to_bits())
        } else {
            0
        };
        e.mem.set_u32(s.node + 0x20, b);
        let process = if case.word(2) != 0 {
            s.process
        } else {
            let p = e.mem.alloc(0xc8);
            e.mem.set_u32(p, s.stand_in_vt);
            p
        };
        vec![s.node, process]
    })
    .run()
    .assert_same();
}

#[test]
fn geometry_on_visible() {
    Diff::new(entry!(
        0x00a7_fd90,
        visible::geometry_on_visible(Ptr<_>, Ptr<_>)
    ))
    .common()
    .inputs(&[Input::Flag])
    .setup(|e, case| {
        let s = scene(e);
        e.mem.set_u32(s.accumulator + 0x10, case.word(0));
        vec![s.geometry, s.process]
    })
    .run()
    .assert_same();
}

#[test]
fn multibound_node_on_visible() {
    // Process mode; non-null children; a multi-bound; the shape's last verdict; its answer.
    Diff::new(entry!(
        0x00c4_6f60,
        visible::multibound_node_on_visible(Ptr<_>, Ptr<_>)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1, 3, 4, 5]),
        Input::Flag,
        Input::Flag,
        Input::Of(&[0, 1, 2, 7]),
        Input::Flag,
    ])
    .setup(|e, case| {
        let s = scene(e);
        let node = e.mem.alloc(0xb4);
        let bytes = e.mem.bytes(s.node, 0xac).to_vec();
        for (i, b) in bytes.iter().enumerate() {
            e.mem.set_u8(node + i as u32, *b);
        }
        e.mem
            .set_u16(node + 0xa8, if case.word(1) != 0 { 3 } else { 0 });
        let shape = aabb(e, &s, 0, ONE, case.word(3), case.word(4));
        let multibound = e.mem.alloc(0x10);
        e.mem.set_u32(multibound + 0xc, shape);
        e.mem
            .set_u32(node + 0xac, if case.word(2) != 0 { multibound } else { 0 });
        e.mem.set_u32(node + 0xb0, 2);
        e.mem.set_u32(s.process + 0x90, case.word(0));
        vec![node, s.process]
    })
    .run()
    .assert_same();
}

#[test]
fn bound_update() {
    // The source centre's x, its radius, the scale, a matrix entry; the sweep fills the rest of the
    // centre, the matrix and the translation.
    Diff::new(entry!(
        0x00a7_fdd0,
        update::bound_update(Ptr<_>, Ptr<_>, Ptr<_>)
    ))
    .common()
    .inputs(&bound_inputs(4))
    .max_cases(384)
    .setup(|e, case| {
        stand_ins(e);
        let from = bound_of(
            e,
            bits(case, 0),
            bits(case, 4),
            bits(case, 5),
            bits(case, 1),
        );
        let t = e.mem.alloc(0x34);
        bound_transform_at(e, t, case, 4);
        let out = bound_of(e, 0, 0, 0, 0);
        vec![out, from, t]
    })
    .run()
    .assert_same();
}

#[test]
fn bound_transform() {
    // The centre's x, the radius, the scale, a matrix entry; the sweep fills the rest of the
    // centre, the matrix and the translation.
    Diff::new(entry!(0x00a7_fea0, update::bound_transform(Ptr<_>, Ptr<_>)))
        .common()
        .inputs(&bound_inputs(1))
        .max_cases(384)
        .setup(|e, case| {
            stand_ins(e);
            let b = bound_of(
                e,
                bits(case, 0),
                bits(case, 4),
                bits(case, 5),
                bits(case, 1),
            );
            let t = e.mem.alloc(0x34);
            bound_transform_at(e, t, case, 1);
            vec![b, t]
        })
        .run()
        .assert_same();
}

#[test]
fn geometry_update_world_bound() {
    // A world bound or none; flags; the bound's radius; a skin.
    Diff::new(entry!(
        0x00a8_0710,
        update::geometry_update_world_bound(Ptr<_>)
    ))
    .common()
    .inputs(&[
        Input::Flag,
        Input::Of(&[0, 0x0008_0000, 0x0008_2000, 0x0040_0000, 0x0048_0000]),
        Input::Of(FEW_FLOATS),
        Input::Flag,
    ])
    .setup(|e, case| {
        let s = scene(e);
        let g = s.geometry;
        let b = if case.word(0) != 0 {
            bound_of(e, ONE, ONE, ONE, case.word(2))
        } else {
            0
        };
        e.mem.set_u32(g + 0x20, b);
        e.mem.set_u32(g + 0x30, case.word(1));
        e.mem
            .set_u32(g + 0xbc, if case.word(3) != 0 { 0x0300_1000 } else { 0 });
        vec![g]
    })
    .run()
    .assert_same();
}

#[test]
fn set_center_and_radius() {
    // The radius, the centre's x.
    Diff::new(entry!(
        0x004a_51d0,
        update::set_center_and_radius(Ptr<_>, Ptr<_>, f32)
    ))
    .common()
    .inputs(&[
        Input::F32,
        Input::F32,
        Input::F32Swept(ONE),
        Input::F32Swept(TWO),
    ])
    .setup(|e, case| {
        stand_ins(e);
        let b = bound_of(e, 0, 0, 0, 0);
        let c = bound_of(e, bits(case, 1), bits(case, 2), bits(case, 3), 0);
        vec![b, c, case.f32(0).to_bits()]
    })
    .run()
    .assert_same();
}

#[test]
fn ensure_world_bound() {
    Diff::new(entry!(0x0054_67e0, update::ensure_world_bound(Ptr<_>)))
        .common()
        .inputs(&[Input::Flag])
        .setup(|e, case| {
            let s = scene(e);
            if case.word(0) == 0 {
                e.mem.set_u32(s.geometry + 0x20, 0);
            }
            vec![s.geometry]
        })
        .run()
        .assert_same();
}

#[test]
fn node_update_world_bound() {
    // Flags; the first child's radius; the last child's radius; the node's own bound or none.
    Diff::new(entry!(0x00a5_e2d0, update::node_update_world_bound(Ptr<_>)))
        .common()
        .inputs(&[
            Input::Of(&[0x0008_0000, 0, 0x0008_2000]),
            Input::F32,
            Input::Of(FEW_FLOATS),
            Input::Flag,
        ])
        .max_cases(256)
        .setup(|e, case| {
            let s = scene(e);
            e.mem.set_u32(s.node + 0x30, case.word(0));
            let first = e.mem.u32(s.children[0] + 0x20);
            e.mem.set_u32(first + 0xc, case.f32(1).to_bits());
            let last = e.mem.u32(s.children[3] + 0x20);
            e.mem.set_u32(last + 0xc, case.word(2));
            if case.word(3) == 0 {
                e.mem.set_u32(s.node + 0x20, 0);
            }
            vec![s.node]
        })
        .run()
        .assert_same();
}

#[test]
fn update_world_transform() {
    // A parent or none; flags; a parent's matrix entry.
    Diff::new(entry!(0x00a6_8bf0, update::update_world_transform(Ptr<_>)))
        .common()
        .inputs(&[Input::Flag, Input::Of(&[0, 0x200]), Input::F32])
        .setup(|e, case| {
            let s = scene(e);
            let g = s.geometry;
            e.mem.set_u32(s.node + 0x68 + 4, case.f32(2).to_bits());
            if case.word(0) == 0 {
                e.mem.set_u32(g + 0x18, 0);
            }
            e.mem.set_u32(g + 0x30, case.word(1));
            vec![g]
        })
        .run()
        .assert_same();
}

#[test]
fn update_world_data() {
    // A collision object or none.
    Diff::new(entry!(0x00a6_8c60, update::update_world_data(Ptr<_>, u32)))
        .common()
        .inputs(&[Input::Flag])
        .setup(|e, case| {
            let s = scene(e);
            if case.word(0) != 0 {
                let collision = e.mem.alloc(0x10);
                e.mem.set_u32(collision, s.stand_in_vt);
                e.mem.set_u32(s.geometry + 0x1c, collision);
            }
            vec![s.geometry, 0x77]
        })
        .run()
        .assert_same();
}

#[test]
fn plane_through() {
    // The normal's y, the point's x.
    Diff::new(entry!(
        0x00a6_9990,
        update::plane_through(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr
    ))
    .common()
    .inputs(&[
        Input::F32,
        Input::F32,
        Input::F32Swept(HALF_F),
        Input::F32Swept(0xbf80_0000),
        Input::F32Swept(TWO),
        Input::F32Swept(MINUS_TEN),
    ])
    .setup(|e, case| {
        stand_ins(e);
        let plane = bound_of(e, 0, 0, 0, 0);
        let n = bound_of(e, bits(case, 2), bits(case, 0), bits(case, 3), 0);
        let p = bound_of(e, bits(case, 1), bits(case, 4), bits(case, 5), 0);
        vec![plane, n, p]
    })
    .run()
    .assert_same();
}

#[test]
fn active_state() {
    // The last plane set in use; the reserved size.
    Diff::new(entry!(0x00c4_9050, compound::active_state(Ptr<_>) -> u32))
        .common()
        .inputs(&[Input::Of(&[0xffff_ffff, 0, 1, 5]), Input::Of(&[0, 1, 2, 8])])
        .setup(|e, case| {
            stand_ins(e);
            let f = compound_frustum(e, 6, 0, 0x2b);
            e.mem.set_u32(f + 0x24, case.word(0));
            e.mem.set_u32(f + 0x10, case.word(1));
            e.mem.set_u32(e.mem.u32(f + 8) + 0x60, 0x11);
            vec![f]
        })
        .run()
        .assert_same();
}

#[test]
fn set_active_state() {
    Diff::new(entry!(0x00c4_9100, compound::set_active_state(Ptr<_>, u32)))
        .common()
        .inputs(&[Input::Of(&[0xffff_ffff, 0, 1, 5]), Input::Of(&[0, 1, 2, 8])])
        .setup(|e, case| {
            stand_ins(e);
            let f = compound_frustum(e, 6, 0, 0x2b);
            e.mem.set_u32(f + 0x24, case.word(0));
            e.mem.set_u32(f + 0x10, case.word(1));
            let state = e.mem.alloc(0x20);
            for i in 0..8 {
                e.mem.set_u32(state + 4 * i, 0x100 + i);
            }
            vec![f, state]
        })
        .run()
        .assert_same();
}

#[test]
fn compound_process() {
    // The object's radius (or no bound); skipping the view frustum; operations in use; the
    // object's centre x.
    Diff::new(entry!(
        0x00c4_91a0,
        compound::process(Ptr<_>, Ptr<_>) -> bool
    ))
    .common()
    .inputs(&[
        Input::Of(&[0x3f80_0000, 0, 0x7fc0_0000, 0xffff_ffff]),
        Input::Flag,
        Input::Of(&[0, 6, 4]),
        Input::F32,
    ])
    .max_cases(256)
    .setup(|e, case| {
        let s = scene(e);
        let b = if case.word(0) == 0xffff_ffff {
            0
        } else {
            bound_of(e, case.f32(3).to_bits(), 0, 0, case.word(0))
        };
        let o = object(e, 0xc8, s.stand_in_vt, b, 0);
        let ops = i32::try_from(case.word(2)).unwrap_or(0);
        let f = compound_frustum(e, ops, u8::from(case.word(1) != 0), 0x3f);
        if ops == 4 {
            // The program starts at the occluder test.
            e.mem.set_u32(f + 0x2c, 2);
        }
        vec![f, o]
    })
    .run()
    .assert_same();
}

#[test]
fn set_camera() {
    Diff::new(entry!(0x00c4_7980, compound::set_camera(Ptr<_>, u32)))
        .common()
        .ignore_calls(&[PLANE_THROUGH])
        .inputs(&[Input::Flag, Input::F32])
        .setup(|e, case| {
            let s = scene(e);
            let f = compound_frustum(e, 6, 1, 0x3f);
            e.mem.set_u8(f, 1);
            let c = camera(e, &s, case.f32(1).to_bits());
            vec![f, if case.word(0) != 0 { c } else { 0 }]
        })
        .run()
        .assert_same();
}

#[test]
fn walk_of_the_scene() {
    // The whole walk from the scene form of Process: every function of the path at once, the
    // node's children inside, outside and on the frustum, with the left edge swept.
    Diff::new(entry!(
        0x00c4_f070,
        process::bs_process_scene(Ptr<_>, u32, Ptr<_>, u32)
    ))
    .common()
    .ignore_calls(&[PLANE_THROUGH])
    .inputs(&[Input::F32, Input::Of(FEW_FLOATS)])
    .max_cases(64)
    .setup(|e, case| {
        let s = scene(e);
        let c = camera(e, &s, case.f32(0).to_bits());
        let inner = s.children[3];
        let b = e.mem.u32(inner + 0x20);
        e.mem.set_u32(b + 0xc, case.word(1));
        vec![s.process, c, s.node, 0]
    })
    .run()
    .assert_same();
}

#[test]
fn frustum_planes_set() {
    // Orthographic or not; one edge (left, also the near distance); a rotation entry; the
    // translation's x. The plane constructor is done in place. The sweep fills the other edges
    // (arguments 4-7), the other rotation entries (8-15) and the translation's y and z (16, 17).
    Diff::new(entry!(
        0x00a7_4e10,
        planes::frustum_planes_set(Ptr<_>, Ptr<_>, Ptr<_>)
    ))
    .common()
    .ignore_calls(&[PLANE_THROUGH])
    .inputs(
        &[Input::Flag, Input::F32, Input::F32, Input::F32]
            .into_iter()
            .chain([ONE, 0x3f40_0000, 0xbf40_0000, 0x447a_0000].map(Input::F32Swept))
            .chain(swept_rotation(0))
            .chain([TWO, MINUS_TEN].map(Input::F32Swept))
            .collect::<Vec<_>>(),
    )
    .max_cases(320)
    .setup(|e, case| {
        stand_ins(e);
        let planes = e.mem.alloc(0x64);
        let frustum = e.mem.alloc(0x1c);
        let edges = [
            bits(case, 1),
            bits(case, 4),
            bits(case, 5),
            bits(case, 6),
            bits(case, 1),
            bits(case, 7),
        ];
        for (i, w) in edges.iter().enumerate() {
            e.mem.set_u32(frustum + 4 * i as u32, *w);
        }
        e.mem.set_u8(frustum + 0x18, u8::from(case.word(0) != 0));
        let t = e.mem.alloc(0x34);
        let m = rotation_from(case, 0, 2, 8);
        transform_at(
            e,
            t,
            m,
            [bits(case, 3), bits(case, 16), bits(case, 17)],
            ONE,
        );
        vec![planes, frustum, t]
    })
    .run()
    .assert_same();
}
