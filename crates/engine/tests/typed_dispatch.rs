//! The differential test of every typed function of the dispatch family (`engine_typed::dispatch`)
//! against its translation: here because the translations live in the part crates, and this crate
//! registers them all. The tests share one kind of world: a node with a vtable whose slots log, a
//! controller chain, a property list, children with bounds, a reference with loaded 3D, lists,
//! arrays, an inventory and an actor; transforms, matrices and points from the case's floats, swept
//! under both control words (the family's row is `Threads::Both`).
//!
//! What stands in: the callees outside the family that reach far (the list walk, the bound copy's
//! helper, the frustum's planes and stamp, the inventory's container and extra lists, the water and
//! height of an actor, D3DX's inverse, the arrays' element destructors and set-ups, the square root)
//! and the imports (interlocked operations, the thread id, `Sleep`). Everything else runs translated.

use engine as _;
use engine_core::prelude::*;
use engine_core::typed::diff::{Case, Diff, Input};
use engine_typed::dispatch::{self, math, refs, scene};
use engine_typed::{culling, pathing};

/// Where the test's virtual functions are registered: past `.text`, one per vtable slot.
const VT_FN: u32 = 0x00ff_2000;
/// The log the stand-ins write (a count, then `(tag, word)` pairs).
const LOG: u32 = 0x0300_0000;
const MAX_CALLS: usize = 5_000;
const FLOAT_SWEEP: usize = 32;

const ONE: u32 = 0x3f80_0000;
const TWO: u32 = 0x4000_0000;
const HALF: u32 = 0x3f00_0000;
const MINUS_ONE: u32 = 0xbf80_0000;
const THIRD: u32 = 0x3eaa_aaab;

/// The RTTI the cast looks for, and the chain an object's slot 8 answers with.
const RTTI: u32 = 0x0120_43f8;

fn ret(eax: u32) -> Ret {
    Ret {
        eax,
        ..Ret::default()
    }
}

fn ret_f(st0: f64) -> Ret {
    Ret {
        st0,
        ..Ret::default()
    }
}

fn log(e: &mut Engine, tag: u32, word: u32) {
    let n = e.mem.u32(LOG);
    e.mem.set_u32(LOG + 4 + 8 * n, tag);
    e.mem.set_u32(LOG + 8 + 8 * n, word);
    e.mem.set_u32(LOG, n + 1);
}

fn bits(case: &Case, i: usize) -> u32 {
    case.f32(i).to_bits()
}

fn words(e: &mut Engine, at: u32, w: &[u32]) {
    for (i, v) in (0u32..).zip(w) {
        e.mem.set_u32(at + 4 * i, *v);
    }
}

/// A new block holding `w`.
fn block(e: &mut Engine, size: u32, w: &[u32]) -> u32 {
    let a = e.mem.alloc(size.max(4 * w.len() as u32));
    words(e, a, w);
    a
}

/// The exe's data these functions read, and the stand-ins.
fn stand_ins(e: &mut Engine) {
    e.map(LOG, 0x4000);
    for page in [
        0x0101_7000,
        0x0101_d000,
        0x0102_0000,
        0x0102_4000,
        0x0118_9000,
        0x011a_3000,
        0x011a_9000,
        0x011c_1000,
        0x011c_5000,
        0x011c_6000,
        0x011d_e000,
        0x011d_f000,
        0x011e_0000,
        0x011f_4000,
        0x011f_6000,
    ] {
        e.map(page, 0x1000);
    }
    e.mem.set_u64(0x0101_7cf8, 0.001f64.to_bits());
    e.mem.set_u64(0x0102_41b0, f64::from(f32::MIN).to_bits());
    e.mem.set_u64(0x0102_0998, 5.0f64.to_bits());
    e.mem.set_u64(0x0101_de30, 0.75f64.to_bits());
    // The unit axes, the havok scale, the fixed translation.
    words(e, 0x011a_946c, &[ONE, 0, 0, 0, ONE, 0, 0, 0, ONE]);
    e.mem.set_u32(0x011c_582c, 0x3e12_4925);
    words(e, 0x011f_474c, &[ONE, TWO, HALF]);
    // The null bound.
    words(e, 0x011f_4288, &[ONE, TWO, HALF, TWO]);
    // The calendar and the player.
    words(e, 0x011e_0358, &[0x4140_0000, 0x7c2, 0xff]);
    e.mem.set_u32(0x011a_39c4, 7);
    // The imports.
    e.register(0x00fd_f1c0, |e, a| {
        let n = e.mem.u32(a[0]).wrapping_add(1);
        e.mem.set_u32(a[0], n);
        ret(n)
    });
    e.register(0x00fd_f1c4, |e, a| {
        let n = e.mem.u32(a[0]).wrapping_sub(1);
        e.mem.set_u32(a[0], n);
        ret(n)
    });
    e.register(0x00fd_f1bc, |e, a| {
        let old = e.mem.u32(a[0]);
        if old == a[2] {
            e.mem.set_u32(a[0], a[1]);
        }
        ret(old)
    });
    e.register(0x00fd_f1d0, |_, _| ret(0x1234));
    // The C runtime's thread data (the float routines' error path), as the pathing test has it.
    e.register(0x00ed_1165, |_, _| ret(0));
    // The arrays' vtable in the exe's data, each slot the stand-in for it.
    e.map(0x0106_b000, 0x1000);
    for slot in (0..0x20).step_by(4) {
        e.mem.set_u32(0x0106_b1a4 + slot, VT_FN + slot);
    }
    e.register(0x0040_fca0, |e, a| {
        // The other owner gives the lock back after a sleep; with `LOG + 0x3ff4` set, only after a
        // `Sleep(1)`, which comes after 10,000 tries (the calls compare in the harness's own log).
        if e.mem.u32(LOG + 0x3ff4) == 0 || a[0] == 1 {
            log(e, 0xfca0, a[0]);
            let lock = e.mem.u32(LOG + 0x3ff0);
            e.mem.set_u32(lock, 0);
        }
        ret(0)
    });
    // The square root (its tables aren't mapped here).
    e.register(0x00ec_6054, |_, a| {
        ret_f(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt())
    });
    // The list walk: the node at the position, the position moved on.
    e.register(0x0057_cbe0, |e, a| {
        let node = e.mem.u32(a[1]);
        e.mem.set_u32(a[1], e.mem.u32(node + 4));
        log(e, 0xcbe0, node);
        ret(node)
    });
    // The frustum: its constructor, its set-up from a camera, the camera's stamp.
    e.register(0x0045_c620, |e, a| {
        log(e, 0xc620, a[0]);
        ret(a[0])
    });
    e.register(0x00a7_55d0, |e, a| {
        log(e, 0x55d0, a[1]);
        // Six planes whose constant comes from the camera: a bound far on one side of the third.
        for i in 0..6u32 {
            let c = if i == 2 {
                e.mem.u32(a[1] + 4)
            } else {
                0xc2c8_0000
            };
            words(e, a[0] + 16 * i, &[ONE, 0, 0, c]);
        }
        ret(0)
    });
    e.register(0x0082_5c00, |e, a| ret(e.mem.u32(a[0] + 4)));
    // The inventory: a reference's container, a container's count, an extra list's reference.
    e.register(0x0055_d310, |e, a| ret(e.mem.u32(a[0] + 0x20)));
    e.register(
        0x0048_2a90,
        |e, a| ret(e.mem.u32(a[0]) ^ a[1] & 0x8000_0003),
    );
    e.register(0x0041_c8d0, |e, a| ret(e.mem.u32(a[0] + 8)));
    e.register(0x0071_7e50, |e, a| ret(e.mem.u32(a[0] + 0x10)));
    // An actor's water height, parent cell, the cell's water and the actor's height.
    e.register(0x0057_b0a0, |e, a| {
        ret_f(f64::from(f32::from_bits(e.mem.u32(a[0] + 0x150))))
    });
    e.register(0x008d_6f30, |e, a| ret(e.mem.u32(a[0] + 0x40)));
    e.register(0x0045_18e0, |e, a| ret(e.mem.u32(a[0]) | 0x100));
    e.register(0x0088_53a0, |e, a| {
        ret_f(f64::from(f32::from_bits(e.mem.u32(a[0] + 0x154))))
    });
    e.register(0x0045_8b20, |_, _| ret(0x011f_426c));
    e.register(0x00ee_6dda, |e, a| {
        log(e, 0x6dda, e.mem.u32(a[2] + 0x30));
        ret(a[0])
    });
    // The arrays' element destructors (their buffer free and set-up are the pathing family's).
    e.register(0x0072_ba80, |e, a| {
        log(e, 0xba80, a[2]);
        ret(0)
    });
    // A vtable's slots: each logs its slot, `this` and its words, and answers by slot.
    let slots: [(u32, AbiFn); 13] = [
        (4, slot::<4>),
        (8, slot::<8>),
        (0x90, slot::<0x90>),
        (0x94, slot::<0x94>),
        (0xa4, slot::<0xa4>),
        (0xa8, slot::<0xa8>),
        (0xac, slot::<0xac>),
        (0xb8, slot::<0xb8>),
        (0xc0, slot::<0xc0>),
        (0x100, slot::<0x100>),
        (0x1b8, slot::<0x1b8>),
        (0x1f4, slot::<0x1f4>),
        (0x420, slot::<0x420>),
    ];
    for s in (0..0x440).step_by(4) {
        e.register(VT_FN + s, any_slot);
    }
    for (s, f) in slots {
        e.register(VT_FN + s, f);
    }
}

/// The stand-in for any other slot: logs `this`.
fn any_slot(e: &mut Engine, a: &[u32]) -> Ret {
    log(e, 0xffff, a.first().copied().unwrap_or(0));
    ret(0)
}

/// The stand-in for vtable slot `S`.
fn slot<const S: u32>(e: &mut Engine, a: &[u32]) -> Ret {
    log(e, S, a[0]);
    for w in &a[1..] {
        log(e, S | 0x8000, *w);
    }
    match S {
        // The RTTI: a chain in the object's word at +0x40.
        8 => ret(e.mem.u32(a[0] + 0x40)),
        // An actor's position.
        0x1f4 => ret(a[0] + 0x30),
        0x100 => ret(e.mem.u32(a[0] + 0x44) & 1),
        0x420 => ret_f(1.5),
        _ => ret(0),
    }
}

/// A vtable whose every slot is the logging stand-in for it.
fn vtable(e: &mut Engine) -> u32 {
    let vt = e.mem.alloc(0x440);
    for slot in (0..0x440).step_by(4) {
        e.mem.set_u32(vt + slot, VT_FN + slot);
    }
    vt
}

/// An object with a vtable, `size` bytes.
fn object(e: &mut Engine, size: u32) -> u32 {
    let o = e.mem.alloc(size);
    let vt = vtable(e);
    e.mem.set_u32(o, vt);
    o
}

/// A list of `items`, the node of the first.
fn list(e: &mut Engine, items: &[u32]) -> u32 {
    let mut next = 0;
    for &item in items.iter().rev() {
        next = block(e, 8, &[item, next]);
    }
    next
}

/// A node (0xb0 bytes) with flags, `children` children (one of them null) with bounds whose radius
/// comes from `radius`, two controllers and a property list with one property with controllers.
fn node(e: &mut Engine, flags: u32, children: u32, radius: u32) -> u32 {
    let n = object(e, 0xb0);
    e.mem.set_u32(n + 0x30, flags);
    let bound = block(e, 0x10, &[ONE, TWO, HALF, THIRD]);
    e.mem.set_u32(n + 0x20, bound);
    let c2 = object(e, 0x40);
    let c1 = object(e, 0x40);
    e.mem.set_u32(c1 + 0x30, c2);
    e.mem.set_u32(n + 0xc, c1);
    let property = object(e, 0x40);
    let pc = object(e, 0x40);
    e.mem.set_u32(property + 0xc, pc);
    let other = object(e, 0x40);
    let props = list(e, &[property, other, 0]);
    e.mem.set_u32(n + 0x24, props);
    let base = e.mem.alloc(4 * children.max(1));
    for i in 0..children {
        let child = if i == 1 {
            0
        } else {
            let c = object(e, 0xb0);
            e.mem
                .set_u32(c + 0x30, [2, 0x12, 0x16, 0][(i & 3) as usize]);
            let r = if i == 0 { radius } else { TWO };
            let b = block(e, 0x10, &[TWO, ONE, HALF, r]);
            e.mem.set_u32(c + 0x20, b);
            c
        };
        e.mem.set_u32(base + 4 * i, child);
    }
    let vt = vtable(e);
    e.mem.set_u32(n + 0x9c, vt);
    e.mem.set_u32(n + 0xa0, base);
    e.mem.set_u16(n + 0xa4, children as u16);
    e.mem.set_u16(n + 0xa6, children as u16);
    e.mem.set_u16(n + 0xa8, children.saturating_sub(1) as u16);
    n
}

/// Update data: the flags at +4 and +5.
fn data(e: &mut Engine, case: &Case, i: usize) -> u32 {
    block(e, 8, &[ONE, case.word(i) & 0xffff])
}

/// A transform of 13 floats: the case's at `i` (scale) and `i + 1` (first entry), the rest from a
/// table.
fn transform(e: &mut Engine, case: &Case, i: usize) -> u32 {
    let table = [
        ONE, HALF, 0, THIRD, ONE, MINUS_ONE, 0, TWO, ONE, TWO, MINUS_ONE, HALF,
    ];
    let t = e.mem.alloc(0x34);
    words(e, t + 4, &table[1..9]);
    words(e, t + 0x24, &table[9..12]);
    e.mem.set_u32(t, bits(case, i + 1));
    e.mem.set_u32(t + 0x30, bits(case, i));
    t
}

/// A point of three case floats from `i`.
fn point(e: &mut Engine, case: &Case, i: usize) -> u32 {
    block(
        e,
        0xc,
        &[bits(case, i), bits(case, i + 1), bits(case, i + 2)],
    )
}

/// Three floats: one from the operand classes, two swept.
const P3: [Input; 3] = [Input::F32, Input::F32Swept(ONE), Input::F32Swept(MINUS_ONE)];

fn inputs(parts: &[&[Input]]) -> Vec<Input> {
    parts.iter().flat_map(|p| p.iter().copied()).collect()
}

/// The callees the translations a test reaches call plainly (a Rust call, not logged), by the
/// function under test: the typed side's calls to them are left out of its log.
const PLAIN: &[(u32, &[u32])] = &[
    (0x0047_68c0, &[0x0047_6930, 0x0047_6980, 0x0047_69c0]),
    (0x0047_6980, &[0x0047_69c0]),
    (0x004b_4880, &[0x0047_6930, 0x0047_6980, 0x0047_69c0]),
    (0x00c4_c2d0, &[0x0047_6930, 0x0047_6980, 0x0047_69c0]),
    (
        0x0043_b4f0,
        &[
            0x0043_b560,
            0x0043_b540,
            0x0043_b5b0,
            0x0043_b5f0,
            0x0043_b5d0,
        ],
    ),
    (0x0043_b560, &[0x0043_b5b0, 0x0043_b5f0, 0x0043_b5d0]),
    (0x0043_b5b0, &[0x0043_b5d0]),
    (0x0043_b5f0, &[0x0043_b5d0]),
    (0x004e_ff50, &[0x004f_0000]),
    (0x00a5_dd70, &[0x004f_0000]),
    (0x00a5_ded0, &[0x004f_0000]),
    (0x004b_5fc0, &[0x004b_5ff0, 0x004b_6100]),
    (0x004b_5ff0, &[0x004b_6100]),
    (0x004c_8f30, &[0x004b_ffb0, 0x004b_fba0]),
    (0x0065_3270, &[0x0065_3290]),
    (0x0043_b610, &[0x0065_3290]),
    (0x0056_0e10, &[0x0056_0e50]),
    (0x004a_3e00, &[0x004a_3e90]),
];

/// What every test adds to its comparison: the families, the plain callees, the call budget.
trait Common {
    fn common(self, function: u32) -> Diff;
}

impl Common for Diff {
    fn common(self, function: u32) -> Diff {
        let mut callees = vec![0x0040_19d0, 0x0040_8860];
        if let Some((_, more)) = PLAIN.iter().find(|(f, _)| *f == function) {
            callees.extend_from_slice(more);
        }
        callees.retain(|c| *c != function);
        self.family(&dispatch::DISPATCH)
            .family(&pathing::PATHING)
            .family(&culling::CULLING)
            .family(&leaf::LEAVES)
            .translation_calls_plainly(&callees)
            .max_calls(MAX_CALLS)
            .float_sweep(FLOAT_SWEEP)
    }
}

// ---- The arithmetic ----

#[test]
fn compose() {
    Diff::new(entry!(
        0x0062_c250,
        math::compose(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common(0x0062_c250)
    .inputs(&inputs(&[&P3, &P3]))
    .max_cases(128)
    .setup(|e, case| {
        stand_ins(e);
        let a = transform(e, case, 0);
        let b = transform(e, case, 3);
        e.mem.set_u32(a + 0x24, bits(case, 2));
        e.mem.set_u32(b + 0x28, bits(case, 5));
        vec![a, e.mem.alloc(0x34), b]
    })
    .run()
    .assert_same();
}

#[test]
fn invert_non_uniform() {
    Diff::new(entry!(
        0x004b_4880,
        math::invert_non_uniform(Ptr<_>, Ptr<_>)
    ))
    .common(0x004b_4880)
    .inputs(&P3)
    .setup(|e, case| {
        stand_ins(e);
        let t = transform(e, case, 0);
        e.mem.set_u32(t + 0x28, bits(case, 2));
        vec![t, e.mem.alloc(0x34)]
    })
    .run()
    .assert_same();
}

#[test]
fn rotate() {
    Diff::new(entry!(
        0x004b_4500,
        math::rotate(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common(0x004b_4500)
    .inputs(&inputs(&[&P3, &P3]))
    .max_cases(128)
    .setup(|e, case| {
        stand_ins(e);
        let m = transform(e, case, 0);
        e.mem.set_u32(m + 0x14, bits(case, 2));
        vec![m, e.mem.alloc(0xc), point(e, case, 3)]
    })
    .run()
    .assert_same();
}

#[test]
fn multiply() {
    Diff::new(entry!(
        0x0043_f8d0,
        math::multiply(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common(0x0043_f8d0)
    .inputs(&inputs(&[&P3, &P3]))
    .max_cases(128)
    .setup(|e, case| {
        stand_ins(e);
        let a = transform(e, case, 0);
        let b = transform(e, case, 3);
        e.mem.set_u32(a + 0x20, bits(case, 2));
        e.mem.set_u32(b + 0x10, bits(case, 5));
        vec![a, e.mem.alloc(0x24), b]
    })
    .run()
    .assert_same();
}

#[test]
fn transpose() {
    Diff::new(entry!(
        0x0047_68c0,
        math::transpose(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common(0x0047_68c0)
    .inputs(&P3)
    .setup(|e, case| {
        stand_ins(e);
        let m = transform(e, case, 0);
        e.mem.set_u32(m + 0x1c, bits(case, 2));
        vec![m, e.mem.alloc(0x24)]
    })
    .run()
    .assert_same();
}

#[test]
fn row() {
    Diff::new(entry!(0x0047_6930, math::row(Ptr<_>, u32, Ptr<_>)))
        .common(0x0047_6930)
        .inputs(&inputs(&[&P3, &[Input::Of(&[0, 1, 2])]]))
        .setup(|e, case| {
            stand_ins(e);
            let m = transform(e, case, 0);
            e.mem.set_u32(m + 0x18, bits(case, 2));
            vec![m, case.word(3), e.mem.alloc(0xc)]
        })
        .run()
        .assert_same();
}

#[test]
fn set_columns() {
    Diff::new(entry!(
        0x0047_6980,
        math::set_columns(Ptr<_>, Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common(0x0047_6980)
    .inputs(&P3)
    .setup(|e, case| {
        stand_ins(e);
        let a = point(e, case, 0);
        let b = block(e, 0xc, &[bits(case, 2), ONE, bits(case, 0)]);
        vec![e.mem.alloc(0x24), a, b, a]
    })
    .run()
    .assert_same();
}

#[test]
fn set_column() {
    Diff::new(entry!(0x0047_69c0, math::set_column(Ptr<_>, u32, Ptr<_>)))
        .common(0x0047_69c0)
        .inputs(&inputs(&[&P3, &[Input::Of(&[0, 1, 2])]]))
        .setup(|e, case| {
            stand_ins(e);
            let p = point(e, case, 0);
            vec![e.mem.alloc(0x24), case.word(3), p]
        })
        .run()
        .assert_same();
}

#[test]
fn transform_new() {
    Diff::new(entry!(0x0047_6a80, math::transform_new(Ptr<_>) -> Ptr<_>))
        .common(0x0047_6a80)
        .inputs(&[Input::Flag])
        .setup(|e, _| {
            stand_ins(e);
            vec![e.mem.alloc(0x34)]
        })
        .run()
        .assert_same();
}

#[test]
fn times() {
    Diff::new(entry!(
        0x0045_bb20,
        math::times(Ptr<_>, Ptr<_>, f32) -> Ptr<_>
    ))
    .common(0x0045_bb20)
    .inputs(&inputs(&[&P3, &[Input::F32]]))
    .max_cases(128)
    .setup(|e, case| {
        stand_ins(e);
        let p = point(e, case, 0);
        vec![p, e.mem.alloc(0xc), bits(case, 3)]
    })
    .run()
    .assert_same();
}

#[test]
fn normalize() {
    Diff::new(entry!(0x004a_0c10, math::normalize(Ptr<_>)))
        .common(0x004a_0c10)
        .inputs(&P3)
        .setup(|e, case| {
            stand_ins(e);
            vec![point(e, case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn box_corners() {
    Diff::new(entry!(0x00c3_9270, math::box_corners(Ptr<_>, Ptr<_>)))
        .common(0x00c3_9270)
        .inputs(&inputs(&[&P3, &P3]))
        .max_cases(128)
        .setup(|e, case| {
            stand_ins(e);
            let b = e.mem.alloc(0x24);
            words(e, b + 0xc, &[bits(case, 0), bits(case, 1), bits(case, 2)]);
            words(e, b + 0x18, &[bits(case, 3), bits(case, 4), bits(case, 5)]);
            vec![b, e.mem.alloc(0x60)]
        })
        .run()
        .assert_same();
}

#[test]
fn inverse_4x4() {
    Diff::new(entry!(0x00c4_c2d0, math::inverse_4x4(Ptr<_>, u32, Ptr<_>)))
        .common(0x00c4_c2d0)
        .inputs(&inputs(&[&P3, &[Input::Of(&[0, 1])]]))
        .setup(|e, case| {
            stand_ins(e);
            let t = transform(e, case, 0);
            e.mem.set_u32(t + 0x2c, bits(case, 2));
            vec![t, case.word(3), e.mem.alloc(0x40)]
        })
        .run()
        .assert_same();
}

#[test]
fn havok_vector() {
    Diff::new(entry!(
        0x004a_3e00,
        math::havok_vector(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common(0x004a_3e00)
    .inputs(&P3)
    .setup(|e, case| {
        stand_ins(e);
        let p = point(e, case, 0);
        vec![e.mem.alloc(0x10), p]
    })
    .run()
    .assert_same();
}

#[test]
fn havok_scale() {
    Diff::new(entry!(0x004a_3e90, math::havok_scale(f32) -> f32))
        .common(0x004a_3e90)
        .inputs(&[Input::F32, Input::F32Swept(0x3e12_4925)])
        .setup(|e, case| {
            stand_ins(e);
            e.mem.set_u32(0x011c_582c, bits(case, 1));
            vec![bits(case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn lane() {
    Diff::new(entry!(0x0056_0d30, math::lane(Ptr<_>, u32) -> Ptr<_>))
        .common(0x0056_0d30)
        .inputs(&[Input::Word, Input::Word])
        .run()
        .assert_same();
}

#[test]
fn copy_16() {
    Diff::new(entry!(0x004a_3c90, math::copy_16(Ptr<_>, Ptr<_>) -> Ptr<_>))
        .common(0x004a_3c90)
        .inputs(&[Input::F32, Input::Word])
        .setup(|e, case| {
            let s = block(e, 0x10, &[bits(case, 0), case.word(1), 0xffff_ffff, 7]);
            vec![e.mem.alloc(0x10), s]
        })
        .run()
        .assert_same();
}

#[test]
fn point2_new() {
    Diff::new(entry!(
        0x0045_2dc0,
        math::point2_new(Ptr<_>, f32, f32) -> Ptr<_>
    ))
    .common(0x0045_2dc0)
    .inputs(&[Input::F32, Input::F32])
    .setup(|e, case| vec![e.mem.alloc(8), bits(case, 0), bits(case, 1)])
    .run()
    .assert_same();
}

#[test]
fn points_close() {
    Diff::new(entry!(
        0x0049_e2f0,
        math::points_close(Ptr<_>, Ptr<_>, f32) -> bool
    ))
    .common(0x0049_e2f0)
    .inputs(&inputs(&[&P3, &[Input::F32, Input::F32Swept(HALF)]]))
    .max_cases(128)
    .setup(|e, case| {
        stand_ins(e);
        let a = point(e, case, 0);
        let b = block(e, 0xc, &[bits(case, 3), bits(case, 1), MINUS_ONE]);
        vec![a, b, bits(case, 4)]
    })
    .run()
    .assert_same();
}

#[test]
fn floats_close() {
    Diff::new(entry!(0x0049_e390, math::floats_close(f32, f32, f32) -> bool))
        .common(0x0049_e390)
        .inputs(&[Input::F32, Input::F32Swept(ONE), Input::F32])
        .max_cases(128)
        .setup(|e, case| {
            stand_ins(e);
            vec![bits(case, 0), bits(case, 1), bits(case, 2)]
        })
        .run()
        .assert_same();
}

// ---- The scene graph ----

#[test]
fn first_controller() {
    Diff::new(entry!(
        0x0043_b230,
        scene::first_controller(Ptr<_>) -> Ptr<_>
    ))
    .common(0x0043_b230)
    .inputs(&[Input::Word])
    .setup(|e, case| vec![block(e, 0x40, &[0, 0, 0, case.word(0)])])
    .run()
    .assert_same();
}

#[test]
fn next_controller() {
    Diff::new(entry!(0x004a_8a90, scene::next_controller(Ptr<_>) -> u32))
        .common(0x004a_8a90)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            let c = e.mem.alloc(0x40);
            e.mem.set_u32(c + 0x30, case.word(0));
            vec![c]
        })
        .run()
        .assert_same();
}

/// Flags for the controller and child tests: with and without 0x40000, bits 0 to 4, 0xd and 0x13.
const NODE_FLAGS: &[u32] = &[
    0x0004_0000,
    0,
    0x0008_0004,
    0x0008_2005,
    0x0004_001f,
    0x0008_0002,
];

#[test]
fn update_object_controllers() {
    Diff::new(entry!(
        0x004e_ff50,
        scene::update_object_controllers(Ptr<_>, Ptr<_>, u8)
    ))
    .common(0x004e_ff50)
    .inputs(&[Input::Of(NODE_FLAGS), Input::Flag])
    .setup(|e, case| {
        stand_ins(e);
        let n = node(e, case.word(0), 0, ONE);
        let d = block(e, 8, &[ONE, 0]);
        vec![n, d, case.word(1)]
    })
    .run()
    .assert_same();
}

#[test]
fn has_property_controllers() {
    Diff::new(entry!(
        0x004f_0000,
        scene::has_property_controllers(Ptr<_>) -> bool
    ))
    .common(0x004f_0000)
    .inputs(&[Input::Of(NODE_FLAGS)])
    .setup(|e, case| {
        let n = block(e, 0x40, &[0; 16]);
        e.mem.set_u32(n + 0x30, case.word(0));
        vec![n]
    })
    .run()
    .assert_same();
}

#[test]
fn child_count() {
    Diff::new(entry!(0x0043_b480, scene::child_count(Ptr<_>) -> u16))
        .common(0x0043_b480)
        .inputs(&[Input::Of(&[0, 1, 3, 0xffff])])
        .setup(|e, case| {
            stand_ins(e);
            let n = node(e, 0, 0, ONE);
            e.mem.set_u16(n + 0xa6, case.word(0) as u16);
            vec![n]
        })
        .run()
        .assert_same();
}

#[test]
fn child() {
    Diff::new(entry!(0x0043_b4a0, scene::child(Ptr<_>, u32) -> Ptr<_>))
        .common(0x0043_b4a0)
        .inputs(&[Input::Of(&[0, 1, 2])])
        .setup(|e, case| {
            stand_ins(e);
            vec![node(e, 0, 3, ONE), case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn array_size() {
    Diff::new(entry!(0x0065_8930, scene::array_size(Ptr<_>) -> u16))
        .common(0x0065_8930)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![block(e, 0x10, &[0, 0, case.word(0) << 16])])
        .run()
        .assert_same();
}

/// The children and radius inputs of the downward passes.
const PASS: [Input; 4] = [
    Input::Of(NODE_FLAGS),
    Input::Of(&[3, 0, 1, 4]),
    Input::Of(&[ONE, 0, 0x7fc0_0000, 0x8000_0000]),
    Input::Of(&[0x0001, 0, 0x0100, 0x0101]),
];

#[test]
fn update_downward() {
    Diff::new(entry!(
        0x00a5_dd70,
        scene::update_downward(Ptr<_>, u32, u32)
    ))
    .common(0x00a5_dd70)
    .inputs(&inputs(&[&PASS, &[Input::Of(&[0, 1, 6])]]))
    .setup(|e, case| {
        stand_ins(e);
        let n = node(e, case.word(0), case.word(1), case.word(2));
        vec![n, data(e, case, 3), case.word(4)]
    })
    .run()
    .assert_same();
}

#[test]
fn update_selected_downward() {
    Diff::new(entry!(
        0x00a5_ded0,
        scene::update_selected_downward(Ptr<_>, u32, u32)
    ))
    .common(0x00a5_ded0)
    .inputs(&inputs(&[&PASS, &[Input::Of(&[0, 1, 6])]]))
    .setup(|e, case| {
        stand_ins(e);
        let n = node(e, case.word(0) | 8, case.word(1), case.word(2));
        vec![n, data(e, case, 3), case.word(4)]
    })
    .run()
    .assert_same();
}

#[test]
fn update_transform_and_bounds() {
    Diff::new(entry!(
        0x00a5_e0b0,
        scene::update_transform_and_bounds(Ptr<_>, u32)
    ))
    .common(0x00a5_e0b0)
    .inputs(&PASS)
    .setup(|e, case| {
        stand_ins(e);
        let n = node(e, case.word(0), case.word(1), case.word(2));
        vec![n, data(e, case, 3)]
    })
    .run()
    .assert_same();
}

/// A renderer: mode at +0x200, immediate flag at +0x208.
fn renderer(e: &mut Engine, case: &Case) -> u32 {
    let r = object(e, 0x210);
    e.mem.set_u32(r + 0x200, case.word(0));
    e.mem.set_u32(r + 0x208, case.word(1));
    r
}

#[test]
fn geometry_render_immediate() {
    Diff::new(entry!(
        0x00a8_0610,
        scene::geometry_render_immediate(Ptr<_>, Ptr<_>)
    ))
    .common(0x00a8_0610)
    .inputs(&[Input::Of(&[1, 2, 0]), Input::Flag])
    .setup(|e, case| {
        stand_ins(e);
        let n = node(e, 0, 0, ONE);
        vec![n, renderer(e, case)]
    })
    .run()
    .assert_same();
}

#[test]
fn tri_strips_render_immediate() {
    Diff::new(entry!(
        0x00a7_1d70,
        scene::tri_strips_render_immediate(Ptr<_>, Ptr<_>)
    ))
    .common(0x00a7_1d70)
    .inputs(&[Input::Of(&[1, 2, 0, 3]), Input::Of(&[1, 0, 0x101])])
    .setup(|e, case| {
        stand_ins(e);
        let n = node(e, 0, 0, ONE);
        vec![n, renderer(e, case)]
    })
    .run()
    .assert_same();
}

#[test]
fn reference_3d() {
    Diff::new(entry!(0x0043_fcd0, scene::reference_3d(Ptr<_>) -> Ptr<_>))
        .common(0x0043_fcd0)
        .inputs(&[Input::Of(&[1, 0, 2])])
        .setup(|e, case| {
            let r = e.mem.alloc(0x70);
            if case.word(0) == 1 {
                let loaded = block(e, 0x18, &[0, 0, 0, 0, 0, 0x0abc_0000]);
                e.mem.set_u32(r + 0x64, loaded);
            }
            let tls = e.tls();
            e.mem.set_u32(tls + 0x260, 0x0def_0000);
            if case.word(0) == 2 {
                e.mem.set_u32(tls + 0x264, r);
            }
            vec![r]
        })
        .run()
        .assert_same();
}

/// A model handle: a holder whose entry (+8) holds its value at +0x10, the value's node at +0x1c.
fn handle(e: &mut Engine, case: &Case) -> u32 {
    let entry = block(e, 0x80, &[0; 32]);
    e.mem.set_u32(entry + 0x2c, 0x0aaa_0000);
    block(e, 0x10, &[0, 0, if case.word(0) == 0 { 0 } else { entry }])
}

#[test]
fn loaded_3d() {
    Diff::new(entry!(
        0x0043_b4f0,
        scene::loaded_3d(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common(0x0043_b4f0)
    .inputs(&[Input::Flag])
    .setup(|e, case| {
        let h = handle(e, case);
        vec![h, e.mem.alloc(4)]
    })
    .run()
    .assert_same();
}

#[test]
fn handle_3d() {
    Diff::new(entry!(0x0043_b540, scene::handle_3d(Ptr<_>) -> u32))
        .common(0x0043_b540)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![block(e, 0x20, &[0, 0, 0, 0, 0, 0, 0, case.word(0)])])
        .run()
        .assert_same();
}

#[test]
fn loaded_handle() {
    Diff::new(entry!(0x0043_b560, scene::loaded_handle(Ptr<_>) -> u32))
        .common(0x0043_b560)
        .inputs(&[Input::Flag])
        .setup(|e, case| vec![handle(e, case)])
        .run()
        .assert_same();
}

#[test]
fn lock_loaded() {
    Diff::new(entry!(0x0043_b5b0, scene::lock_loaded(Ptr<_>)))
        .common(0x0043_b5b0)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

#[test]
fn loaded_lock() {
    Diff::new(entry!(0x0043_b5d0, scene::loaded_lock(Ptr<_>) -> Ptr<_>))
        .common(0x0043_b5d0)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

#[test]
fn unlock_loaded() {
    Diff::new(entry!(0x0043_b5f0, scene::unlock_loaded(Ptr<_>)))
        .common(0x0043_b5f0)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

/// An object whose RTTI chain (slot 8, its word at +0x40) holds `rtti` or not, with a collision
/// object at +0x1c of the same kind.
fn kinded(e: &mut Engine, case: &Case, i: usize) -> u32 {
    let base = block(e, 8, &[0, if case.word(i) == 1 { RTTI } else { 0 }]);
    let own = block(e, 8, &[0, base]);
    let o = object(e, 0x48);
    e.mem
        .set_u32(o + 0x40, if case.word(i) == 2 { RTTI } else { own });
    o
}

#[test]
fn collision_object() {
    Diff::new(entry!(
        0x0043_b610,
        scene::collision_object(Ptr<_>) -> Ptr<_>
    ))
    .common(0x0043_b610)
    .inputs(&[Input::Of(&[1, 0, 2, 3])])
    .setup(|e, case| {
        stand_ins(e);
        let o = e.mem.alloc(0x20);
        let c = if case.word(0) == 3 {
            0
        } else {
            kinded(e, case, 0)
        };
        e.mem.set_u32(o + 0x1c, c);
        vec![o]
    })
    .run()
    .assert_same();
}

#[test]
fn collision_word() {
    Diff::new(entry!(0x0068_38b0, scene::collision_word(Ptr<_>) -> u32))
        .common(0x0068_38b0)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![block(e, 0x20, &[0, 0, 0, 0, 0, 0, 0, case.word(0)])])
        .run()
        .assert_same();
}

#[test]
fn cast_if() {
    Diff::new(entry!(0x0065_3270, scene::cast_if(u32, u32) -> u32))
        .common(0x0065_3270)
        .inputs(&[Input::Of(&[1, 0, 2, 3])])
        .setup(|e, case| {
            stand_ins(e);
            let o = if case.word(0) == 3 {
                0
            } else {
                kinded(e, case, 0)
            };
            vec![RTTI, o]
        })
        .run()
        .assert_same();
}

#[test]
fn cast() {
    Diff::new(entry!(0x0065_3290, scene::cast(Ptr<_>, u32) -> Ptr<_>))
        .common(0x0065_3290)
        .inputs(&[Input::Of(&[1, 0, 2])])
        .setup(|e, case| {
            stand_ins(e);
            vec![kinded(e, case, 0), RTTI]
        })
        .run()
        .assert_same();
}

#[test]
fn is_kind_of() {
    Diff::new(entry!(0x0065_32c0, scene::is_kind_of(Ptr<_>, u32) -> bool))
        .common(0x0065_32c0)
        .inputs(&[Input::Of(&[1, 0, 2])])
        .setup(|e, case| {
            stand_ins(e);
            vec![kinded(e, case, 0), RTTI]
        })
        .run()
        .assert_same();
}

#[test]
fn esize_of_children() {
    Diff::new(entry!(0x0045_3470, scene::esize_of_children(Ptr<_>) -> u32))
        .common(0x0045_3470)
        .inputs(&[Input::Of(&[0, 1, 3, 4])])
        .setup(|e, case| {
            stand_ins(e);
            vec![node(e, 0, case.word(0), ONE)]
        })
        .run()
        .assert_same();
}

#[test]
fn esize() {
    Diff::new(entry!(0x0099_38b0, scene::esize(Ptr<_>) -> u16))
        .common(0x0099_38b0)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![block(e, 0x10, &[0, 0, 0, case.word(0)])])
        .run()
        .assert_same();
}

/// A camera at a given stamp; the frustum kept for it or another.
fn camera(e: &mut Engine, case: &Case) -> u32 {
    let c = block(e, 0x10, &[0, case.f32(0).to_bits()]);
    let owner = 0x011f_6394;
    e.mem.set_u32(owner + 4, 5);
    e.mem.set_u32(0x011c_6394, case.word(1));
    if case.word(1) & 1 != 0 {
        e.mem
            .set_u32(0x011c_632c, if case.word(2) == 0 { c } else { 0x1234 });
        e.mem.set_u32(0x0118_90e0, case.word(2) & 5);
        for i in 0..6u32 {
            words(e, 0x011c_6330 + 16 * i, &[ONE, 0, 0, MINUS_ONE]);
        }
    }
    c
}

/// A bound from case floats: centre x and radius.
fn bound(e: &mut Engine, case: &Case) -> u32 {
    block(e, 0x10, &[bits(case, 3), ONE, HALF, bits(case, 4)])
}

/// The frustum inputs: the camera's constant, the guard, the stamp, the bound's x and radius.
const FRUSTUM: [Input; 5] = [
    Input::F32Swept(0xc120_0000),
    Input::Of(&[1, 0]),
    Input::Of(&[0, 5, 1]),
    Input::F32,
    Input::F32Swept(TWO),
];

#[test]
fn in_frustum() {
    Diff::new(entry!(
        0x004b_5ff0,
        scene::in_frustum(Ptr<_>, Ptr<_>) -> bool
    ))
    .common(0x004b_5ff0)
    .inputs(&inputs(&[&FRUSTUM, &[Input::Flag]]))
    .setup(|e, case| {
        stand_ins(e);
        let c = if case.word(5) == 1 {
            camera(e, case)
        } else {
            0
        };
        vec![bound(e, case), c]
    })
    .run()
    .assert_same();
}

#[test]
fn bound_in_frustum() {
    Diff::new(entry!(
        0x004b_5fc0,
        scene::bound_in_frustum(Ptr<_>, Ptr<_>) -> bool
    ))
    .common(0x004b_5fc0)
    .inputs(&inputs(&[&FRUSTUM, &[Input::Flag]]))
    .setup(|e, case| {
        stand_ins(e);
        let o = if case.word(5) == 1 {
            let n = node(e, 0, 0, ONE);
            let b = bound(e, case);
            e.mem.set_u32(n + 0x20, b);
            n
        } else {
            0
        };
        vec![o, camera(e, case)]
    })
    .run()
    .assert_same();
}

#[test]
fn plane() {
    Diff::new(entry!(0x004b_4cf0, scene::plane(Ptr<_>, u32) -> Ptr<_>))
        .common(0x004b_4cf0)
        .inputs(&[Input::Word, Input::Word])
        .run()
        .assert_same();
}

#[test]
fn word_8() {
    Diff::new(entry!(0x0062_0b80, scene::word_8(u32) -> u32))
        .common(0x0062_0b80)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![block(e, 0xc, &[0, 0, case.word(0)])])
        .run()
        .assert_same();
}

#[test]
fn character_proxy() {
    Diff::new(entry!(0x004a_e750, scene::character_proxy(Ptr<_>) -> u32))
        .common(0x004a_e750)
        .inputs(&[Input::Word, Input::Flag])
        .setup(|e, case| {
            let o = block(e, 0xc, &[0, 0, case.word(0)]);
            vec![if case.word(1) == 1 { o } else { 0 }]
        })
        .run()
        .assert_same();
}

#[test]
fn body_vector() {
    Diff::new(entry!(0x0056_0e10, scene::body_vector(Ptr<_>) -> Ptr<_>))
        .common(0x0056_0e10)
        .inputs(&[Input::Flag])
        .setup(|e, case| {
            stand_ins(e);
            let entity = e.mem.alloc(0x200);
            vec![block(
                e,
                0xc,
                &[0, 0, if case.word(0) == 1 { entity } else { 0 }],
            )]
        })
        .run()
        .assert_same();
}

#[test]
fn body_vector_of() {
    Diff::new(entry!(0x0056_0e50, scene::body_vector_of(Ptr<_>) -> Ptr<_>))
        .common(0x0056_0e50)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

#[test]
fn part_e0() {
    Diff::new(entry!(0x009d_9f40, scene::part_e0(Ptr<_>) -> Ptr<_>))
        .common(0x009d_9f40)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

// ---- References, locks, lists and getters ----

#[test]
fn increment() {
    Diff::new(entry!(0x0040_b460, refs::increment(Ptr<_>) -> i32))
        .common(0x0040_b460)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            vec![block(e, 4, &[case.word(0)])]
        })
        .run()
        .assert_same();
}

#[test]
fn decrement() {
    Diff::new(entry!(0x0040_19a0, refs::decrement(Ptr<_>) -> i32))
        .common(0x0040_19a0)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            vec![block(e, 4, &[case.word(0)])]
        })
        .run()
        .assert_same();
}

#[test]
fn add_reference() {
    Diff::new(entry!(0x0040_f6e0, refs::add_reference(Ptr<_>)))
        .common(0x0040_f6e0)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let o = object(e, 8);
            e.mem.set_u32(o + 4, case.word(0));
            vec![o]
        })
        .run()
        .assert_same();
}

#[test]
fn release() {
    Diff::new(entry!(0x0040_1970, refs::release(Ptr<_>)))
        .common(0x0040_1970)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let o = object(e, 8);
            e.mem.set_u32(o + 4, case.word(0));
            vec![o]
        })
        .run()
        .assert_same();
}

#[test]
fn release_held() {
    Diff::new(entry!(0x0045_cec0, refs::release_held(Ptr<_>)))
        .common(0x0045_cec0)
        .inputs(&[Input::Of(&[1, 2, 0])])
        .setup(|e, case| {
            stand_ins(e);
            let o = object(e, 8);
            e.mem.set_u32(o + 4, case.word(0));
            vec![block(e, 4, &[if case.word(0) == 0 { 0 } else { o }])]
        })
        .run()
        .assert_same();
}

#[test]
fn ref_object_new() {
    Diff::new(entry!(0x0049_68b0, refs::ref_object_new(Ptr<_>) -> Ptr<_>))
        .common(0x0049_68b0)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            e.mem.set_u32(0x011f_4400, case.word(0));
            vec![block(e, 8, &[0xdead, 0xbeef])]
        })
        .run()
        .assert_same();
}

#[test]
fn ref_object_drop() {
    Diff::new(entry!(0x0049_6910, refs::ref_object_drop(Ptr<_>)))
        .common(0x0049_6910)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            e.mem.set_u32(0x011f_4400, case.word(0));
            vec![block(e, 8, &[0xdead, 0xbeef])]
        })
        .run()
        .assert_same();
}

#[test]
fn current_thread() {
    Diff::new(entry!(0x0040_fc90, refs::current_thread() -> u32))
        .common(0x0040_fc90)
        .inputs(&[])
        .setup(|e, _| {
            stand_ins(e);
            vec![]
        })
        .run()
        .assert_same();
}

#[test]
fn compare_exchange() {
    Diff::new(entry!(
        0x0043_b460,
        refs::compare_exchange(Ptr<_>, i32, i32) -> i32
    ))
    .common(0x0043_b460)
    .inputs(&[Input::Of(&[0, 5]), Input::Of(&[0, 5]), Input::Word])
    .setup(|e, case| {
        stand_ins(e);
        vec![block(e, 4, &[case.word(0)]), case.word(1), case.word(2)]
    })
    .run()
    .assert_same();
}

#[test]
fn lock() {
    Diff::new(entry!(0x0040_fbf0, refs::lock(Ptr<_>, u32)))
        .common(0x0040_fbf0)
        .inputs(&[Input::Of(&[0x1234, 0, 0x99]), Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let l = block(e, 8, &[case.word(0), case.word(1)]);
            // `Sleep` lets the other owner go at once (the long wait: `lock_sleeps_a_millisecond_after_ten_thousand_tries`).
            e.mem.set_u32(LOG + 0x3ff0, l);
            vec![l, 0]
        })
        .run()
        .assert_same();
}

#[test]
fn unlock() {
    Diff::new(entry!(0x0040_fba0, refs::unlock(Ptr<_>)))
        .common(0x0040_fba0)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![block(e, 8, &[0x1234, case.word(0)])])
        .run()
        .assert_same();
}

#[test]
fn nothing_2() {
    Diff::new(entry!(0x0045_34f0, refs::nothing_2(u32, u32)))
        .common(0x0045_34f0)
        .inputs(&[Input::Word, Input::Word])
        .run()
        .assert_same();
}

#[test]
fn nothing_1() {
    Diff::new(entry!(0x0048_3710, refs::nothing_1(u32)))
        .common(0x0048_3710)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

#[test]
fn at_10() {
    Diff::new(entry!(0x0046_0140, refs::at_10(Ptr<_>) -> Ptr<_>))
        .common(0x0046_0140)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

#[test]
fn store_word() {
    Diff::new(entry!(0x008c_71b0, refs::store_word(Ptr<_>, u32) -> Ptr<_>))
        .common(0x008c_71b0)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![e.mem.alloc(4), case.word(0)])
        .run()
        .assert_same();
}

/// A list of three entries whose first words are 7, `form` and 9 (the middle one null when `hole`),
/// each with a word at +4 and +8.
fn entries(e: &mut Engine, form: u32, hole: bool) -> u32 {
    let a = block(e, 0xc, &[7, 0xffff_fffe, 0x70]);
    let b = block(e, 0xc, &[form, 3, 0x80]);
    let c = block(e, 0xc, &[9, 0, 0x90]);
    list(e, &[a, if hole { 0 } else { b }, c])
}

#[test]
fn reaction() {
    Diff::new(entry!(0x0048_c1b0, refs::reaction(Ptr<_>, u32) -> u32))
        .common(0x0048_c1b0)
        .inputs(&[Input::Of(&[0x55, 0, 9, 0x66]), Input::Flag])
        .setup(|e, case| {
            stand_ins(e);
            let l = entries(e, 0x55, case.word(1) == 0);
            vec![block(e, 0x14, &[0, 0, 0, 0, l]), case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn detection_state() {
    Diff::new(entry!(
        0x008f_6650,
        refs::detection_state(Ptr<_>, u32, i32) -> Ptr<_>
    ))
    .common(0x008f_6650)
    .inputs(&[
        Input::Of(&[0x55, 7, 9, 0x66]),
        Input::Of(&[0, 1]),
        Input::Flag,
    ])
    .setup(|e, case| {
        stand_ins(e);
        let p = e.mem.alloc(0x264);
        let l = entries(e, 0x55, case.word(2) == 0);
        let m = entries(e, 0x66, false);
        e.mem.set_u32(p + 0x25c, l);
        e.mem.set_u32(p + 0x260, m);
        vec![p, case.word(0), case.word(1)]
    })
    .run()
    .assert_same();
}

#[test]
fn array_item() {
    Diff::new(entry!(0x0096_8670, refs::array_item(Ptr<_>, u32) -> u32))
        .common(0x0096_8670)
        .inputs(&[Input::Of(&[0, 1, 2])])
        .setup(|e, case| {
            let data = block(e, 0xc, &[0x11, 0x22, 0x33]);
            vec![block(e, 0x10, &[0, data, 3, 3]), case.word(0)]
        })
        .run()
        .assert_same();
}

/// Inventory changes: their list of entries (one for `0x55` with a count and extra lists whose
/// references have ids), their reference with a container whose count depends on the form.
fn inventory(e: &mut Engine, case: &Case) -> u32 {
    let reference = block(e, 0x10, &[0, 0, 0, 0x4242]);
    let other = block(e, 0x10, &[0, 0, 0, 0x4343]);
    let x1 = block(e, 0xc, &[0, 0, other]);
    let x2 = block(e, 0xc, &[0, 0, reference]);
    let x0 = block(e, 0xc, &[0, 0, 0]);
    let extras = list(e, &[x0, x1, 0, x2]);
    let entry = block(e, 0xc, &[extras, case.word(1), 0x55]);
    let first = block(e, 0xc, &[0, 2, 0x66]);
    let changes = list(e, &[first, 0, entry]);
    let container = block(e, 4, &[case.word(2)]);
    let owner = block(e, 0x24, &[0; 9]);
    e.mem.set_u32(owner + 0x20, container);
    block(
        e,
        0x14,
        &[changes, if case.word(3) == 1 { owner } else { 0 }],
    )
}

const INVENTORY: [Input; 4] = [
    Input::Of(&[0x55, 0x66, 0x77]),
    Input::Of(&[0, 3, 0xffff_fffe]),
    Input::Of(&[0, 0x55, 0xffff_ffaa]),
    Input::Flag,
];

#[test]
fn item_count() {
    Diff::new(entry!(0x004c_8f30, refs::item_count(Ptr<_>, u32) -> i32))
        .common(0x004c_8f30)
        .inputs(&INVENTORY)
        .setup(|e, case| {
            stand_ins(e);
            vec![inventory(e, case), case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn container_of() {
    Diff::new(entry!(0x004b_ffb0, refs::container_of(Ptr<_>) -> u32))
        .common(0x004b_ffb0)
        .inputs(&INVENTORY)
        .setup(|e, case| {
            stand_ins(e);
            vec![inventory(e, case)]
        })
        .run()
        .assert_same();
}

#[test]
fn entry_for() {
    Diff::new(entry!(
        0x004b_fba0,
        refs::entry_for(Ptr<_>, u32, u32, u32) -> u32
    ))
    .common(0x004b_fba0)
    .inputs(&inputs(&[
        &INVENTORY,
        &[Input::Of(&[0, 0x4242, 0x4343, 0x9999])],
    ]))
    .setup(|e, case| {
        stand_ins(e);
        vec![inventory(e, case), case.word(0), 1, case.word(4)]
    })
    .run()
    .assert_same();
}

/// An actor: the flag at +0x14d, the water height (+0x150) and its height (+0x154), its position,
/// its cell (+0x40) and whether the cell has water.
fn actor(e: &mut Engine, case: &Case) -> u32 {
    let a = object(e, 0x160);
    e.mem.set_u8(a + 0x14d, case.word(0) as u8);
    e.mem.set_u32(a + 0x150, bits(case, 1));
    e.mem.set_u32(a + 0x154, bits(case, 2));
    words(e, a + 0x30, &[ONE, TWO, bits(case, 3)]);
    let cell = block(e, 4, &[case.word(4)]);
    e.mem
        .set_u32(a + 0x40, if case.word(4) == 2 { 0 } else { cell });
    e.mem.set_u32(a + 0x108, case.word(5));
    e.mem.set_u32(0x011d_f880, case.word(6));
    a
}

const ACTOR: [Input; 7] = [
    Input::Of(&[1, 0, 0x100]),
    Input::F32,
    Input::F32Swept(0x42c8_0000),
    Input::F32Swept(0x4248_0000),
    Input::Of(&[1, 0, 2]),
    Input::Of(&[2, 1, 6, 0]),
    Input::Of(&[0, 3, 4, 0xffff_ffff]),
];

#[test]
fn location_on_reference() {
    Diff::new(entry!(
        0x008a_e4c0,
        refs::location_on_reference(Ptr<_>) -> u32
    ))
    .common(0x008a_e4c0)
    .inputs(&ACTOR)
    .max_cases(128)
    .setup(|e, case| {
        stand_ins(e);
        vec![actor(e, case)]
    })
    .run()
    .assert_same();
}

#[test]
fn position() {
    Diff::new(entry!(0x0043_6aa0, refs::position(Ptr<_>) -> Ptr<_>))
        .common(0x0043_6aa0)
        .inputs(&[Input::Word])
        .run()
        .assert_same();
}

#[test]
fn byte_14d() {
    Diff::new(entry!(0x005a_2030, refs::byte_14d(Ptr<_>) -> u8))
        .common(0x005a_2030)
        .inputs(&ACTOR)
        .max_cases(16)
        .setup(|e, case| {
            stand_ins(e);
            vec![actor(e, case)]
        })
        .run()
        .assert_same();
}

#[test]
fn in_state() {
    Diff::new(entry!(0x0088_44f0, refs::in_state(Ptr<_>, bool) -> bool))
        .common(0x0088_44f0)
        .inputs(&inputs(&[&ACTOR, &[Input::Flag]]))
        .max_cases(32)
        .setup(|e, case| {
            stand_ins(e);
            vec![actor(e, case), case.word(7)]
        })
        .run()
        .assert_same();
}

#[test]
fn word_108() {
    Diff::new(entry!(0x004f_8960, refs::word_108(Ptr<_>) -> u32))
        .common(0x004f_8960)
        .inputs(&ACTOR)
        .max_cases(16)
        .setup(|e, case| {
            stand_ins(e);
            vec![actor(e, case)]
        })
        .run()
        .assert_same();
}

#[test]
fn compute_last_time_processed() {
    Diff::new(entry!(
        0x0090_7650,
        refs::compute_last_time_processed(Ptr<_>)
    ))
    .common(0x0090_7650)
    .inputs(&[Input::F32, Input::Of(&[0x7f, 0x80, 0xff, 1])])
    .setup(|e, case| {
        stand_ins(e);
        e.mem.set_u32(0x011e_0358, bits(case, 0));
        e.mem.set_u8(0x011e_0360, case.word(1) as u8);
        vec![e.mem.alloc(0x28)]
    })
    .run()
    .assert_same();
}

#[test]
fn set_hour() {
    Diff::new(entry!(0x0069_3d50, refs::set_hour(Ptr<_>, f32)))
        .common(0x0069_3d50)
        .inputs(&[Input::F32])
        .setup(|e, case| vec![e.mem.alloc(0x28), bits(case, 0)])
        .run()
        .assert_same();
}

#[test]
fn set_date() {
    Diff::new(entry!(0x0070_37c0, refs::set_date(Ptr<_>, u32)))
        .common(0x0070_37c0)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![e.mem.alloc(0x28), case.word(0)])
        .run()
        .assert_same();
}

#[test]
fn compass_order() {
    Diff::new(entry!(
        0x0077_b300,
        refs::compass_order(Ptr<_>, Ptr<_>) -> i32
    ))
    .common(0x0077_b300)
    .inputs(&inputs(&[&P3, &[Input::F32, Input::Of(&[1, 0, 2, 3])]]))
    .max_cases(128)
    .setup(|e, case| {
        stand_ins(e);
        let actor_at = |e: &mut Engine, p: [u32; 3]| {
            let a = object(e, 0x40);
            words(e, a + 0x30, &p);
            a
        };
        let player = actor_at(e, [0, 0, 0]);
        e.mem.set_u32(0x011d_ea3c, player);
        let first = actor_at(e, [bits(case, 0), bits(case, 1), bits(case, 2)]);
        let second = actor_at(e, [bits(case, 3), bits(case, 1), MINUS_ONE]);
        let s1 = block(e, 4, &[if case.word(4) == 2 { 0 } else { first }]);
        let s2 = block(e, 4, &[second]);
        match case.word(4) {
            0 => vec![0, s2],
            3 => vec![s1, 0],
            _ => vec![s1, s2],
        }
    })
    .run()
    .assert_same();
}

#[test]
fn object_factor() {
    Diff::new(entry!(0x0086_a050, refs::object_factor(u32) -> f64))
        .common(0x0086_a050)
        .inputs(&[Input::Of(&[1, 0, 2])])
        .setup(|e, case| {
            stand_ins(e);
            let o = object(e, 0x48);
            e.mem.set_u32(o + 0x44, case.word(0));
            let s = e.mem.alloc(0xd0);
            e.mem
                .set_u32(s + 0xcc, if case.word(0) == 2 { 0 } else { o });
            vec![s]
        })
        .run()
        .assert_same();
}

#[test]
fn word_cc() {
    Diff::new(entry!(0x009a_d610, refs::word_cc(Ptr<_>) -> Ptr<_>))
        .common(0x009a_d610)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            let s = e.mem.alloc(0xd0);
            e.mem.set_u32(s + 0xcc, case.word(0));
            vec![s]
        })
        .run()
        .assert_same();
}

#[test]
fn array_clear() {
    Diff::new(entry!(0x0084_54f0, refs::array_clear(Ptr<_>, u8)))
        .common(0x0084_54f0)
        .inputs(&[Input::Of(&[0x40, 0]), Input::Of(&[1, 0, 0x100])])
        .setup(|e, case| {
            stand_ins(e);
            vec![
                block(e, 0x10, &[0x0106_b1a4, case.word(0), 3, 8]),
                case.word(1),
            ]
        })
        .run()
        .assert_same();
}

#[test]
fn order_array_new() {
    Diff::new(entry!(0x0069_aee0, refs::order_array_new(Ptr<_>) -> Ptr<_>))
        .common(0x0069_aee0)
        .inputs(&[Input::Flag])
        .setup(|e, _| {
            stand_ins(e);
            vec![e.mem.alloc(0x10)]
        })
        .run()
        .assert_same();
}

#[test]
fn info_array_drop() {
    Diff::new(entry!(0x0069_af10, refs::info_array_drop(Ptr<_>) -> Ptr<_>))
        .common(0x0069_af10)
        .inputs(&[Input::Of(&[0x40, 0])])
        .setup(|e, case| {
            stand_ins(e);
            vec![block(e, 0x10, &[0, case.word(0), 3, 8])]
        })
        .run()
        .assert_same();
}

#[test]
fn extra_kind() {
    Diff::new(entry!(0x004f_1540, refs::extra_kind(Ptr<_>) -> u8))
        .common(0x004f_1540)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![block(e, 8, &[0, case.word(0)])])
        .run()
        .assert_same();
}

// ---- Fixed values at the main thread's 24-bit precision ----
//
// The differential tests compare the typed versions with their translations, so a reading of the exe
// both share would pass them. These check the three sums the batch's translation fixes are about
// against values worked out by hand from the x87's rules: under 24-bit precision each `FADD` and `FMUL`
// rounds to 24 bits (ties to even) before the next, under 53-bit to 53 bits, and `FSTP` to a float.

/// 2^-24 as a float.
const TWO_POW_M24: u32 = 0x3380_0000;

/// Runs `f` under the control word `word` (the thread's, as the CRT keeps it), then back to 53 bits.
fn under(e: &mut Engine, word: u16, f: impl FnOnce(&mut Engine) -> u32) -> u32 {
    engine_core::units::crt::set_x87_control_word(e, word);
    let r = f(e);
    engine_core::units::crt::set_x87_control_word(e, 0x027f);
    r
}

/// `(1 + 2^-24) + 2^-24`: at 24 bits each sum is a tie that goes to the even 1.0; at 53 bits the sum
/// is exact, 1 + 2^-23, the float after 1.0.
const TIE_SUM: [(u16, u32); 2] = [(0x007f, ONE), (0x027f, ONE + 1)];

#[test]
fn multiply_at_24_bits_rounds_each_sum() {
    let mut e = Engine::new();
    stand_ins(&mut e);
    let a = block(
        &mut e,
        0x24,
        &[ONE, TWO_POW_M24, TWO_POW_M24, 0, 0, 0, 0, 0, 0],
    );
    let b = block(&mut e, 0x24, &[ONE, 0, 0, ONE, 0, 0, ONE, 0, 0]);
    for (word, expected) in TIE_SUM {
        let out = e.mem.alloc(0x24);
        let got = under(&mut e, word, |e| {
            e.call(0x0043_f8d0, &engine_core::args![a, out, b]);
            e.mem.u32(out)
        });
        assert_eq!(got, expected, "translation, control word {word:04x}");
        let out = e.mem.alloc(0x24);
        let got = under(&mut e, word, |e| {
            math::multiply(e, Ptr::new(a), Ptr::new(out), Ptr::new(b));
            e.mem.u32(out)
        });
        assert_eq!(got, expected, "typed, control word {word:04x}");
    }
}

#[test]
fn rotate_at_24_bits_rounds_each_sum() {
    let mut e = Engine::new();
    stand_ins(&mut e);
    let m = block(
        &mut e,
        0x24,
        &[ONE, TWO_POW_M24, TWO_POW_M24, 0, 0, 0, 0, 0, 0],
    );
    let v = block(&mut e, 0xc, &[ONE, ONE, ONE]);
    for (word, expected) in TIE_SUM {
        let out = e.mem.alloc(0xc);
        let got = under(&mut e, word, |e| {
            e.call(0x004b_4500, &engine_core::args![m, out, v]);
            e.mem.u32(out)
        });
        assert_eq!(got, expected, "translation, control word {word:04x}");
        let out = e.mem.alloc(0xc);
        let got = under(&mut e, word, |e| {
            math::rotate(e, Ptr::new(m), Ptr::new(out), Ptr::new(v));
            e.mem.u32(out)
        });
        assert_eq!(got, expected, "typed, control word {word:04x}");
    }
}

/// An actor's height `h = k * 2^-52` with `k = (2^54 + 2^30 + 1) / 3`, so `0.75 * h` is exactly
/// `1 + 2^-24 + 2^-54`. Under 24 bits `FMUL` rounds that up (above the tie) to 1 + 2^-23; under
/// 53 bits it rounds to 1 + 2^-24 (a quarter of a unit is dropped) and `FSTP` then breaks the tie to
/// the even 1.0.
#[test]
fn location_on_reference_at_24_bits_rounds_the_rise_once() {
    let mut e = Engine::new();
    stand_ins(&mut e);
    e.register(0x0057_b0a0, |_, _| ret_f(1000.0));
    e.register(0x0088_53a0, |_, _| {
        let k = ((1u64 << 54) + (1 << 30) + 1) / 3;
        ret_f(k as f64 * f64::powi(2.0, -52))
    });
    e.register(0x0045_18e0, |_, _| ret(1));
    let actor = e.mem.alloc(0x160);
    e.mem.set_u8(actor + 0x14d, 1);
    let cell = e.mem.alloc(4);
    e.mem.set_u32(actor + 0x40, cell);
    for (word, expected) in [(0x007f, ONE + 1), (0x027f, ONE)] {
        let got = under(&mut e, word, |e| {
            let slot = e.call(0x008a_e4c0, &engine_core::args![actor]).u32();
            e.mem.u32(slot + 8)
        });
        assert_eq!(got, expected, "translation, control word {word:04x}");
        let got = under(&mut e, word, |e| {
            let slot = refs::location_on_reference(e, Ptr::new(actor));
            e.mem.u32(slot + 8)
        });
        assert_eq!(got, expected, "typed, control word {word:04x}");
    }
}

/// The lock's long wait, which the differential test leaves out (10,000 tries, each call compared
/// with a hash of memory, is too slow there): an owner that lets go only at a `Sleep(1)` gets 10,000
/// `Sleep(0)`s, then one `Sleep(1)`, then the lock is this thread's with a count of 1. The translation
/// and the typed version, counted the same way.
#[test]
fn lock_sleeps_a_millisecond_after_ten_thousand_tries() {
    fn run(typed: bool) -> (u32, u32, u32, u32) {
        let mut e = Engine::new();
        stand_ins(&mut e);
        // `Sleep`: counts by its argument; the owner lets go at a `Sleep(1)`.
        e.register(0x0040_fca0, |e, a| {
            let at = LOG + 0x3f00 + 4 * a[0].min(1);
            e.mem.set_u32(at, e.mem.u32(at) + 1);
            if a[0] == 1 {
                let lock = e.mem.u32(LOG + 0x3ff0);
                e.mem.set_u32(lock, 0);
            }
            ret(0)
        });
        let l = block(&mut e, 8, &[0x98, 5]);
        e.mem.set_u32(LOG + 0x3ff0, l);
        if typed {
            refs::lock(&mut e, Ptr::new(l), 0);
        } else {
            e.call(0x0040_fbf0, &engine_core::args![l, 0u32]);
        }
        let sleeps = (e.mem.u32(LOG + 0x3f00), e.mem.u32(LOG + 0x3f04));
        (sleeps.0, sleeps.1, e.mem.u32(l), e.mem.u32(l + 4))
    }
    let expected = (10_000, 1, 0x1234, 1);
    assert_eq!(run(false), expected, "translation");
    assert_eq!(run(true), expected, "typed");
}
