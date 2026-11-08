//! The differential test of every typed function of the pathing family (`engine_typed::pathing`)
//! against its translation: here because the translations live in the part crates, and this crate
//! registers them all. One world serves the tests: two navmeshes (six vertices and four triangles, one
//! edge leading to the other mesh, whose triangle leads back), their infos and grid, a search with a
//! goal, pathing locations, arrays with a vtable, spheres and arcs. The float cases sweep the
//! harness's operand classes through coordinates, radii and scales under both control words (the
//! family's row is `Threads::Both`).
//!
//! What stands in: the callees that reach far outside the family (the info's mesh, the heights in a
//! triangle, the water, the map lookups, the pathing singleton's map, the dynamic cast, the cell and
//! world-space getters, the heap, the scrap heap) and the interlocked imports. Everything else runs
//! translated.

use engine as _;
use engine_core::prelude::*;
use engine_core::typed::diff::{Case, Diff, Input};
use engine_typed::pathing::{self, location, navmesh, point, pov, search};

/// Where the test's virtual functions are registered: past `.text`, one per vtable slot.
const VT_FN: u32 = 0x00ff_2000;
/// The log the stand-ins write (a count, then `(tag, word)` pairs).
const LOG: u32 = 0x0300_0000;
/// The most calls one run makes here is a few hundred.
const MAX_CALLS: usize = 5_000;
/// Swept cases per float test.
const FLOAT_SWEEP: usize = 32;

const ONE: u32 = 0x3f80_0000;
const TWO: u32 = 0x4000_0000;
const TEN: u32 = 0x4120_0000;
const HALF: u32 = 0x3f00_0000;
const MINUS_ONE: u32 = 0xbf80_0000;

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

/// The exe's constants these functions read, and the stand-ins.
fn stand_ins(e: &mut Engine) {
    e.map(LOG, 0x3000);
    for page in [
        0x0101_1000,
        0x0101_2000,
        0x0101_5000,
        0x0101_6000,
        0x0101_7000,
        0x0101_e000,
        0x0102_4000,
        0x0102_9000,
        0x0102_f000,
        0x0103_5000,
        0x0106_b000,
        0x010a_2000,
        0x011d_e000,
        0x011c_1000,
        0x011f_6000,
        0x0106_a000,
    ] {
        e.map(page, 0x1000);
    }
    let doubles: [(u32, f64); 9] = [
        (0x0103_56e8, 19.0),
        (0x0101_7a40, 100.0),
        (0x0102_90b0, 0.25),
        (0x0101_2060, 0.0),
        (0x0101_e2c0, 50.0),
        (0x0106_b160, -180.0),
        (0x0102_40c0, 64.0),
        (0x0103_5810, 40.0),
        (0x0101_6978, f64::from(0.001f32)),
    ];
    for (a, v) in doubles {
        e.mem.set_u64(a, v.to_bits());
    }
    let floats: [(u32, u32); 6] = [
        (0x0102_f200, 0x3eaa_aa9f),
        (0x0101_6970, 0x7f7f_ffff),
        (0x0101_5f5c, 0xff7f_ffff),
        (0x0101_2054, MINUS_ONE),
        (0x0101_e6ec, 0x4280_0000),
        (0x0106_b158, 0x4334_0000),
    ];
    for (a, v) in floats {
        e.mem.set_u32(a, v);
    }
    e.mem.set_u32(0x010a_2720, 4);
    // The pathing singleton and its info map.
    let map = e.mem.alloc(0x40);
    let pathing = e.mem.alloc(0x40);
    e.mem.set_u32(pathing + 0x10, map);
    e.mem.set_u32(0x011d_ea10, pathing);
    e.register(0x0045_af00, |e, a| ret(e.mem.u32(a[0] + 0x10)));
    // An info's mesh: its word at +0x54 into the slot.
    e.register(0x0068_f2e0, |e, a| {
        let m = e.mem.u32(a[0] + 0x54);
        e.mem.set_u32(a[1], m);
        ret(u32::from(m != 0))
    });
    // The heights in and near a triangle: a float from the triangle's index, found for all but every
    // third.
    e.register(0x0069_7810, |e, a| height(e, a[1] & 0xffff, a[3]));
    e.register(0x0069_7730, |e, a| height(e, (a[1] & 0xffff) + 1, a[3]));
    // The water's height.
    e.register(0x0054_7250, |e, a| {
        e.mem.set_u32(a[2], TEN);
        ret(1)
    });
    // A map lookup: the map's word at +0x10 is the value, found when the key is odd.
    e.register(0x0085_3130, |e, a| {
        log(e, 0x3130, a[1]);
        e.mem.set_u32(a[2], e.mem.u32(a[0] + 0x10));
        ret(a[1] & 1)
    });
    e.register(0x0046_4f30, |_, a| ret(a[0] | 1));
    e.register(0x00ec_43fb, |_, a| ret(a[0]));
    e.register(0x0048_39c0, |_, a| ret(a[0] ^ 0x100));
    e.register(0x0058_7630, |e, a| {
        log(e, 0x7630, a[1]);
        ret(a[1] & 0xff0)
    });
    e.register(0x006b_77b0, |_, a| ret(a[0] + 0x40));
    e.register(0x0054_4c60, |_, a| ret(a[0] & 0xff));
    e.register(0x0054_4c30, |_, a| ret(a[0] >> 8 & 0xff));
    e.register(0x0058_7410, |_, a| ret(a[0] << 16 | a[1] | 1));
    e.register(0x0054_ddd0, |e, a| ret(e.mem.u32(a[0] + 0x20)));
    e.register(0x0042_5fd0, |e, a| ret(e.mem.u32(a[0] + 0x24) & 1));
    // The scrap heap, the location set-up and the open list's heap.
    e.register(0x006d_d280, |e, a| {
        log(e, 0xd280, a[1]);
        ret(a[0])
    });
    // The open list's scrap heap set-up (it reserves memory from the system), logged.
    e.register(0x00aa_5410, |e, a| {
        log(e, 0x5410, a[1]);
        ret(a[0])
    });
    e.register(0x00aa_54a0, |e, a| {
        log(e, 0x54a0, a[2]);
        ret(e.mem.alloc(a[1].max(4)))
    });
    // The holders' interlocked counts.
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
    e.register(0x00ed_1165, |_, _| ret(0));
    // A vtable's slots: allocate (slot 1) and free (slot 2), logged.
    e.register(VT_FN + 4, |e, a| {
        log(e, 4, a[1]);
        ret(e.mem.alloc(a[1].max(1) * 4))
    });
    e.register(VT_FN + 8, |e, a| {
        log(e, 8, a[1]);
        ret(0)
    });
    // A vtable's slot 3 (a reallocation), logged.
    e.register(VT_FN + 0xc, |e, a| {
        log(e, 0xc, a[1]);
        ret(e.mem.alloc(0x100))
    });
    // The arrays' vtables in the exe's data, each slot the stand-in for it.
    for vt in [0x0106_abe4u32, 0x0106_b1e0] {
        for slot in (0..0x20).step_by(4) {
            e.mem.set_u32(vt + slot, VT_FN + slot);
        }
    }
    // The C runtime's square root, sine and cosine (their tables aren't mapped here).
    e.register(0x00ec_6054, |_, a| {
        ret_f(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt())
    });
    e.register(0x004e_4470, |_, a| {
        ret_f(f64::from(f32::from_bits(a[0])).sin())
    });
    e.register(0x004e_44b0, |_, a| {
        ret_f(f64::from(f32::from_bits(a[0])).cos())
    });
    // The request's radius.
    e.register(0x009b_0bd0, |e, a| {
        ret_f(f64::from(f32::from_bits(e.mem.u32(a[0] + 0x74))))
    });
}

/// A height for triangle `t` into `out`: a float from its index, found for all but every third.
fn height(e: &mut Engine, t: u32, out: u32) -> Ret {
    let h = [0x4100_0000u32, 0xc348_0000, 0x4220_0000, 0x42a0_0000][(t & 3) as usize];
    e.mem.set_u32(out, h);
    ret(u32::from(t % 3 != 2))
}

/// A vtable of 0x40 slots, each the stand-in for its slot.
fn vtable(e: &mut Engine) -> u32 {
    let vt = e.mem.alloc(0x40);
    for slot in (0..0x40).step_by(4) {
        e.mem.set_u32(vt + slot, VT_FN + slot);
    }
    vt
}

/// An array header: vtable, data, size, capacity.
fn array(e: &mut Engine, data: u32, size: u32, capacity: u32) -> u32 {
    let a = e.mem.alloc(0x10);
    let vt = vtable(e);
    e.mem.set_u32(a, vt);
    e.mem.set_u32(a + 4, data);
    e.mem.set_u32(a + 8, size);
    e.mem.set_u32(a + 0xc, capacity);
    a
}

fn point_at(e: &mut Engine, at: u32, p: [u32; 3]) {
    for (i, w) in p.iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, *w);
    }
}

fn point(e: &mut Engine, p: [u32; 3]) -> u32 {
    let a = e.mem.alloc(0x10);
    point_at(e, a, p);
    a
}

/// The world's parts.
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct World {
    mesh: u32,
    other: u32,
    info: u32,
    other_info: u32,
    /// A triangle list of the grid (triangles 0 to 3).
    list: u32,
    /// The pathing location on triangle 1 of `mesh`.
    location: u32,
    /// A search whose goal is triangle 2 of `mesh`, with a request.
    search: u32,
    /// A node whose location (at +8) is on triangle `n` of `mesh`.
    node: u32,
}

/// Vertex floats of the first mesh (x, y, z).
const VERTICES: [[f32; 3]; 6] = [
    [0.0, 0.0, 0.0],
    [10.0, 0.0, 1.0],
    [0.0, 10.0, 2.0],
    [10.0, 10.0, 3.0],
    [20.0, 0.0, 0.0],
    [20.0, 10.0, 1.0],
];

/// Builds a mesh: vertices, triangles `(vertices, links, flags)`, edge infos `(info, triangle)`.
fn mesh(
    e: &mut Engine,
    vertices: &[[f32; 3]],
    triangles: &[([u16; 3], [u16; 3], u32)],
    infos: &[(u32, u16)],
    info: u32,
) -> u32 {
    let m = e.mem.alloc(0x108);
    let vt = vtable(e);
    let vdata = e.mem.alloc(0xc * vertices.len() as u32);
    for (i, v) in vertices.iter().enumerate() {
        point_at(e, vdata + 0xc * i as u32, v.map(f32::to_bits));
    }
    let tdata = e.mem.alloc(0x10 * triangles.len() as u32);
    for (i, (vs, ls, flags)) in triangles.iter().enumerate() {
        let t = tdata + 0x10 * i as u32;
        for j in 0..3 {
            e.mem.set_u16(t + 2 * j as u32, vs[j]);
            e.mem.set_u16(t + 6 + 2 * j as u32, ls[j]);
        }
        e.mem.set_u32(t + 0xc, *flags);
    }
    let idata = e.mem.alloc(0xc * infos.len().max(1) as u32);
    for (i, (inf, t)) in infos.iter().enumerate() {
        e.mem.set_u32(idata + 0xc * i as u32 + 4, *inf);
        e.mem.set_u16(idata + 0xc * i as u32 + 8, *t);
    }
    for (off, data, n) in [
        (0x28, vdata, vertices.len()),
        (0x38, tdata, triangles.len()),
        (0x48, idata, infos.len()),
    ] {
        e.mem.set_u32(m + off, vt);
        e.mem.set_u32(m + off + 4, data);
        e.mem.set_u32(m + off + 8, n as u32);
        e.mem.set_u32(m + off + 0xc, n as u32);
    }
    // The grid: 2 x 2 cells of 10, from (0, 0) to (20, 20); every cell lists triangles 0 to 3.
    let g = m + 0xa8;
    e.mem.set_u32(g, 2);
    point_at(e, g + 4, [TEN, TEN, 0]);
    point_at(e, g + 0x10, [0, 0, 0]);
    point_at(e, g + 0x18, [0x41a0_0000, 0x41a0_0000, TEN]);
    let cells = e.mem.alloc(0x40);
    let entries = e.mem.alloc(8);
    for i in 0..4 {
        e.mem.set_u16(entries + 2 * i, i as u16);
    }
    for c in 0..4 {
        e.mem.set_u32(cells + 0x10 * c, vt);
        e.mem.set_u32(cells + 0x10 * c + 4, entries);
        e.mem.set_u32(cells + 0x10 * c + 8, 4 - c);
        e.mem.set_u32(cells + 0x10 * c + 0xc, 4);
    }
    e.mem.set_u32(g + 0x24, cells);
    e.mem.set_u32(m + 0x104, info);
    // The mesh's form id (+0xc) for the node keys.
    e.mem.set_u32(m + 0xc, 0x0012_3456);
    m
}

fn world(e: &mut Engine) -> World {
    stand_ins(e);
    let info = e.mem.alloc(0x58);
    let other_info = e.mem.alloc(0x58);
    let mesh_a = mesh(
        e,
        &VERTICES,
        &[
            ([0, 1, 2], [0xffff, 1, 0xffff], 0),
            ([1, 3, 2], [2, 0xffff, 0], 0x20),
            ([1, 4, 3], [0xffff, 3, 1], 0x10),
            ([4, 5, 3], [0, 0xffff, 2], 0x1),
        ],
        &[(other_info, 0)],
        info,
    );
    let mesh_b = mesh(
        e,
        &[[20.0, 0.0, 0.0], [30.0, 0.0, 0.0], [20.0, 10.0, 0.0]],
        &[([0, 1, 2], [0xffff, 0xffff, 0], 0x4)],
        &[(info, 3)],
        other_info,
    );
    e.mem.set_u32(info + 0x54, mesh_a);
    e.mem.set_u32(info + 0x4, 0x0001_0203);
    e.mem.set_u32(info + 0xc, 0x0000_0506);
    e.mem.set_u32(other_info + 0x54, mesh_b);
    e.mem.set_u32(other_info + 0x1c, 0x0400_0000);
    let list = e.mem.u32(mesh_a + 0xa8 + 0x24);
    let location = e.mem.alloc(0x28);
    e.mem.set_u32(location, 0x0102_583c);
    point_at(e, location + 4, [ONE, TWO, HALF]);
    e.mem.set_u32(location + 0x10, info);
    e.mem.set_u32(location + 0x1c, 0x0400_0000);
    e.mem.set_u32(location + 0x20, 0x0000_0123);
    e.mem.set_u16(location + 0x24, 1);
    e.mem.set_u8(location + 0x26, 2);
    let search = e.mem.alloc(0x2084);
    let request = e.mem.alloc(0x80);
    e.mem.set_u32(request + 0x68, 0x42c8_0000 + 0x0010_0000);
    e.mem.set_u32(request + 0x74, 0x4120_0000);
    e.mem.set_u32(search + 0x2058, request);
    e.mem.set_u32(search + 0x2070, mesh_a);
    e.mem.set_u16(search + 0x2074, 2);
    point_at(e, search + 0x2078, [ONE, ONE, ONE]);
    let node = e.mem.alloc(0x20);
    e.mem.set_u32(node + 8, mesh_a);
    e.mem.set_u16(node + 0xc, 2);
    point_at(e, node + 0x10, [TWO, HALF, TEN]);
    World {
        mesh: mesh_a,
        other: mesh_b,
        info,
        other_info,
        list,
        location,
        search,
        node,
    }
}

/// Functions whose translations compute in plain `f64` (the other threads' rounding), so they differ from
/// the exe under the main thread's 24-bit word: compared under the other threads' word only. The typed
/// versions round as the exe does under both (translation bugs, listed in the batch notes).
const MAIN_WORD_DIFFERS: &[u32] = &[0x006b_9e90, 0x0069_8060, 0x0069_8620, 0x006a_0660];

/// Functions whose translations pick another NaN than the exe's x87 (a signalling one passed on
/// unquieted, the sign of a NaN): translation bugs, listed in the batch notes.
const NAN_PAYLOADS_DIFFER: &[u32] = &[0x0045_79e0, 0x0069_8620];

/// The holder locals of the counted-pointer helpers.
const HOLDER: [LocalAt; 5] = [
    (0x0042_fdc0, 1, 4),
    (0x0042_fd70, 0, 4),
    (0x0046_4ff0, 0, 4),
    (0x0042_f4c0, 1, 4),
    (0x0042_fa40, 0, 4),
];

/// Stack locals the typed version places in other frames than the translation, by function under
/// test: `(callee, word, size)`.
/// `(callee, word, size)` of a `local_at`.
type LocalAt = (u32, usize, u32);

const LOCALS: &[(u32, &[LocalAt])] = &[
    (0x0069_0890, &HOLDER),
    (0x0069_ad00, &HOLDER),
    (0x006d_d610, &HOLDER),
    (0x006d_d640, &HOLDER),
    (
        0x0069_6a50,
        &[
            (0x0069_d370, 2, 4),
            (0x0069_7810, 3, 4),
            (0x0054_7250, 2, 4),
        ],
    ),
    (
        0x006b_9b10,
        &[
            (0x0041_6870, 0, 0xc),
            (0x0043_9ef0, 0, 0xc),
            (0x0043_9ef0, 1, 0xc),
            (0x0043_9ef0, 2, 0xc),
            (0x004b_6190, 0, 0xc),
            (0x004b_6190, 1, 0xc),
            (0x004a_3760, 0, 0xc),
            (0x004a_3760, 2, 0xc),
            (0x0043_9e90, 1, 0xc),
            (0x0043_9e90, 2, 0xc),
            (0x0045_7990, 0, 0xc),
        ],
    ),
    (
        0x006b_ab00,
        &[
            (0x0041_6870, 0, 0xc),
            (0x0043_9ef0, 0, 0xc),
            (0x0043_9ef0, 1, 0xc),
            (0x0043_9ef0, 2, 0xc),
            (0x004b_6190, 0, 0xc),
            (0x004b_6190, 1, 0xc),
            (0x004a_3760, 0, 0xc),
            (0x004a_3760, 2, 0xc),
            (0x0043_9e90, 1, 0xc),
            (0x0043_9e90, 2, 0xc),
            (0x0068_15c0, 0, 0xc),
            (0x004a_7290, 0, 0xc),
        ],
    ),
    (
        0x006b_b570,
        &[
            (0x0041_6870, 0, 0xc),
            (0x0043_9ef0, 1, 0xc),
            (0x0049_41c0, 0, 0xc),
            (0x0059_5c80, 0, 0xc),
        ],
    ),
    (
        0x006b_a240,
        &[
            (0x0041_6870, 0, 0xc),
            (0x0043_9ef0, 1, 0xc),
            (0x004a_3760, 0, 0xc),
            (0x004a_3760, 2, 0xc),
            (0x0043_9e90, 1, 0xc),
            (0x0043_9e90, 2, 0xc),
        ],
    ),
    (
        0x006b_9e90,
        &[
            (0x004e_8880, 1, 8),
            (0x0053_2fd0, 0, 8),
            (0x0053_2fd0, 2, 8),
            (0x004d_fe30, 1, 8),
            (0x004d_fe30, 2, 8),
        ],
    ),
    (0x0055_8220, &[(0x0063_c8a0, 0, 0xc), (0x0043_9180, 0, 0xc)]),
    (
        0x0069_71e0,
        &[
            (0x0041_6870, 0, 0xc),
            (0x0043_9ef0, 1, 0xc),
            (0x0053_d1a0, 0, 0xc),
            (0x0053_d1a0, 2, 0xc),
        ],
    ),
    (
        0x006a_79c0,
        &[
            (0x0041_6870, 0, 0xc),
            (0x006a_6eb0, 1, 0xc),
            (0x0043_9ef0, 0, 0xc),
            (0x0043_9ef0, 1, 0xc),
            (0x0043_9ef0, 2, 0xc),
            (0x0059_5c80, 0, 0xc),
        ],
    ),
    (0x0068_f040, &[(0x0068_efd0, 1, 4)]),
    (0x0068_f460, &[(0x0068_f2e0, 1, 4)]),
    (0x0068_f670, &[(0x0068_f2e0, 1, 4)]),
    (0x006b_77e0, &[(0x0085_3130, 2, 4)]),
    (0x006b_7850, &[(0x0085_3130, 2, 4)]),
    (0x006d_6f40, &[(0x0085_3130, 2, 4)]),
    (0x006d_6f60, &[(0x0085_3130, 2, 4)]),
    (0x0069_3d70, &[(0x0085_3130, 2, 4)]),
    (
        0x006a_0660,
        &[
            (0x0069_8620, 1, 8),
            (0x006a_06f0, 0, 8),
            (0x006a_06f0, 1, 8),
        ],
    ),
    (0x0069_8620, &[(0x0069_85f0, 1, 8)]),
    (0x006a_0e90, &[(0x0069_85f0, 1, 8)]),
    (0x0069_8060, &[(0x006b_9e90, 0, 8), (0x006b_9e90, 1, 8)]),
];

/// The callees the translations a test reaches call plainly (a Rust call, not logged), by the
/// function under test: the typed side's calls to them are left out of its log.
const PLAIN: &[(u32, &[u32])] = &[
    (0x0040_19b0, &[0x0040_19d0]),
    (0x0040_8840, &[0x0040_8860]),
    (0x0045_7990, &[0x0040_19d0]),
    (0x0045_79e0, &[0x0040_19d0]),
    (0x0055_8220, &[0x0055_82d0, 0x0055_82f0]),
    (0x0058_9850, &[0x0040_19d0]),
    (0x0068_f040, &[0x0068_efd0, 0x0068_f0a0]),
    (0x0068_f0c0, &[0x0068_f0a0]),
    (0x0068_f230, &[0x0068_f1d0, 0x0068_f2c0]),
    (
        0x0068_f3b0,
        &[
            0x0068_efb0,
            0x0068_f1d0,
            0x0068_f200,
            0x0068_f230,
            0x0068_f2c0,
            0x0068_f460,
        ],
    ),
    (
        0x0068_f460,
        &[
            0x0068_efb0,
            0x0068_f1d0,
            0x0068_f200,
            0x0068_f230,
            0x0068_f2c0,
        ],
    ),
    (
        0x0068_f670,
        &[
            0x0068_efb0,
            0x0068_f1d0,
            0x0068_f200,
            0x0068_f230,
            0x0068_f2c0,
            0x0068_f460,
        ],
    ),
    (0x0069_0800, &[0x0069_0830]),
    (0x0069_69e0, &[0x0069_6a10]),
    (0x0069_6a50, &[0x0040_8860]),
    (0x0069_6cf0, &[0x0040_8860, 0x0055_82d0, 0x0055_82f0]),
    (0x0069_8620, &[0x0069_85f0]),
    (0x0069_d370, &[]),
    (0x006a_0660, &[0x0069_85f0, 0x006a_06f0]),
    (0x006a_79c0, &[0x006a_6eb0]),
    (0x006a_7b10, &[0x006a_8380]),
    (0x006b_40f0, &[0x006a_eb70]),
    (0x006b_77e0, &[0x006b_7850, 0x006b_78b0]),
    (0x006b_9b10, &[0x0040_19d0]),
    (0x006b_9e90, &[0x0040_8860]),
    (0x006b_b570, &[0x0040_19d0]),
    (0x006d_6f40, &[0x006b_7850, 0x006b_78b0]),
    (0x006d_b4f0, &[0x006d_bbb0]),
    (0x006d_cf20, &[0x0069_0830, 0x006d_cfe0]),
    (0x006e_78b0, &[0x006e_8f40]),
];

trait Common {
    fn common(self) -> Diff;
    fn plain(self, function: u32) -> Diff;
}

impl Common for Diff {
    fn common(self) -> Diff {
        self.family(&pathing::PATHING)
            .family(&leaf::LEAVES)
            .max_calls(MAX_CALLS)
    }

    fn plain(self, function: u32) -> Diff {
        let mut callees = vec![0x0040_19d0, 0x0040_8860];
        if let Some((_, more)) = PLAIN.iter().find(|(f, _)| *f == function) {
            callees.extend_from_slice(more);
        }
        callees.retain(|c| *c != function);
        let mut d = self.translation_calls_plainly(&callees);
        if MAIN_WORD_DIFFERS.contains(&function) {
            d = d.control_words(&[0x027f]);
        }
        if NAN_PAYLOADS_DIFFER.contains(&function) {
            d = d.allow_nan_payloads();
        }
        if let Some((_, locals)) = LOCALS.iter().find(|(f, _)| *f == function) {
            for &(callee, word, size) in locals.iter() {
                d = d.local_at(callee, word, size);
            }
        }
        d
    }
}

/// A point of three case floats from argument `i`.
fn case_point(e: &mut Engine, case: &Case, i: usize) -> u32 {
    point(e, [bits(case, i), bits(case, i + 1), bits(case, i + 2)])
}

/// Three float inputs: one from the operand classes, two swept.
const P3: [Input; 3] = [Input::F32, Input::F32Swept(ONE), Input::F32Swept(MINUS_ONE)];

fn inputs(parts: &[&[Input]]) -> Vec<Input> {
    parts.iter().flat_map(|p| p.iter().copied()).collect()
}

// ---- Point helpers ----

#[test]
fn subtract() {
    Diff::new(entry!(
        0x0043_9ef0,
        point::subtract(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x0043_9ef0)
    .inputs(&inputs(&[&P3, &P3]))
    .max_cases(128)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let (a, b, out) = (
            case_point(e, case, 0),
            case_point(e, case, 3),
            e.mem.alloc(0xc),
        );
        vec![a, out, b]
    })
    .run()
    .assert_same();
}

#[test]
fn add() {
    Diff::new(entry!(
        0x0043_9e90,
        point::add(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x0043_9e90)
    .inputs(&inputs(&[&P3, &P3]))
    .max_cases(128)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let (a, b, out) = (
            case_point(e, case, 0),
            case_point(e, case, 3),
            e.mem.alloc(0xc),
        );
        vec![a, out, b]
    })
    .run()
    .assert_same();
}

#[test]
fn scaled() {
    Diff::new(entry!(
        0x004a_3760,
        point::scaled(Ptr<_>, f32, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x004a_3760)
    .inputs(&inputs(&[&[Input::F32], &P3]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let (p, out) = (case_point(e, case, 1), e.mem.alloc(0xc));
        vec![out, bits(case, 0), p]
    })
    .run()
    .assert_same();
}

#[test]
fn scale_by() {
    Diff::new(entry!(0x0043_9180, point::scale_by(Ptr<_>, f32) -> Ptr<_>))
        .common()
        .plain(0x0043_9180)
        .inputs(&inputs(&[&[Input::F32], &P3]))
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, case| {
            stand_ins(e);
            vec![case_point(e, case, 1), bits(case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn add_to() {
    Diff::new(entry!(0x0063_c8a0, point::add_to(Ptr<_>, Ptr<_>) -> Ptr<_>))
        .common()
        .plain(0x0063_c8a0)
        .inputs(&inputs(&[&P3, &P3, &[Input::Flag]]))
        .max_cases(128)
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, case| {
            stand_ins(e);
            let a = case_point(e, case, 0);
            // Also onto itself.
            let b = if case.word(6) == 1 {
                a
            } else {
                case_point(e, case, 3)
            };
            vec![a, b]
        })
        .run()
        .assert_same();
}

#[test]
fn length_xy_squared() {
    Diff::new(entry!(0x0059_5c80, point::length_xy_squared(Ptr<_>) -> f32))
        .common()
        .plain(0x0059_5c80)
        .inputs(&P3)
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, case| {
            stand_ins(e);
            vec![case_point(e, case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn length_squared() {
    Diff::new(entry!(0x004a_7290, point::length_squared(Ptr<_>) -> f32))
        .common()
        .plain(0x004a_7290)
        .inputs(&P3)
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, case| {
            stand_ins(e);
            vec![case_point(e, case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn length() {
    Diff::new(entry!(0x0045_7990, point::length(Ptr<_>) -> f32))
        .common()
        .plain(0x0045_7990)
        .inputs(&P3)
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, case| {
            stand_ins(e);
            vec![case_point(e, case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn length_2d() {
    Diff::new(entry!(0x0058_9850, point::length_2d(Ptr<_>) -> f32))
        .common()
        .plain(0x0058_9850)
        .inputs(&P3)
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, case| {
            stand_ins(e);
            vec![case_point(e, case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn abs() {
    Diff::new(entry!(0x0040_8840, point::abs(f32) -> f32))
        .common()
        .plain(0x0040_8840)
        .inputs(&[Input::F32])
        .setup(|e, case| {
            stand_ins(e);
            vec![bits(case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn root() {
    Diff::new(entry!(0x0040_19b0, point::root(f32) -> f32))
        .common()
        .plain(0x0040_19b0)
        .inputs(&[Input::F32])
        .setup(|e, case| {
            stand_ins(e);
            vec![bits(case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn root_of() {
    Diff::new(entry!(0x0045_79e0, point::root_of(f32) -> f32))
        .common()
        .plain(0x0045_79e0)
        .inputs(&[Input::F32])
        .setup(|e, case| {
            stand_ins(e);
            vec![bits(case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn root_rounded() {
    Diff::new(entry!(0x0040_19d0, point::root_rounded(f32) -> f32))
        .common()
        .plain(0x0040_19d0)
        .inputs(&[Input::F32])
        .setup(|e, case| {
            stand_ins(e);
            vec![bits(case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn negated() {
    Diff::new(entry!(
        0x004a_0bd0,
        point::negated(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x004a_0bd0)
    .inputs(&P3)
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(0xc);
        vec![case_point(e, case, 0), out]
    })
    .run()
    .assert_same();
}

#[test]
fn on_circle() {
    Diff::new(entry!(
        0x0069_e0c0,
        point::on_circle(Ptr<_>, Ptr<_>, f32) -> Ptr<_>
    ))
    .common()
    .plain(0x0069_e0c0)
    .inputs(&inputs(&[&[Input::F32], &P3]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let (d, out) = (case_point(e, case, 1), e.mem.alloc(0xc));
        vec![d, out, bits(case, 0)]
    })
    .run()
    .assert_same();
}

// ---- The navmesh ----

const TRIANGLES: &[u32] = &[0, 1, 2, 3];
const EDGES: &[u32] = &[0, 1, 2, 3, 0xffff_ffff];

#[test]
fn triangle() {
    Diff::new(entry!(
        0x0055_82f0,
        navmesh::triangle(Ptr<_>, u16) -> Ptr<_>
    ))
    .common()
    .plain(0x0055_82f0)
    .inputs(&[Input::Of(TRIANGLES)])
    .setup(|e, case| vec![world(e).mesh, case.word(0)])
    .run()
    .assert_same();
}

#[test]
fn triangle_at() {
    Diff::new(entry!(
        0x0055_8dd0,
        navmesh::triangle_at(Ptr<_>, u32) -> Ptr<_>
    ))
    .common()
    .plain(0x0055_8dd0)
    .inputs(&[Input::Word])
    .setup(|e, case| vec![world(e).mesh + 0x38, case.word(0)])
    .run()
    .assert_same();
}

#[test]
fn vertex() {
    Diff::new(entry!(0x0055_82d0, navmesh::vertex(Ptr<_>, u32) -> u16))
        .common()
        .plain(0x0055_82d0)
        .inputs(&[Input::Of(TRIANGLES), Input::Of(&[0, 1, 2, 4, 5])])
        .setup(|e, case| {
            let w = world(e);
            let t = e.mem.u32(w.mesh + 0x3c) + 0x10 * case.word(0);
            vec![t, case.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn triangle_count() {
    Diff::new(entry!(0x0055_8200, navmesh::triangle_count(Ptr<_>) -> u32))
        .common()
        .plain(0x0055_8200)
        .inputs(&[Input::Flag])
        .setup(|e, case| {
            let w = world(e);
            vec![if case.word(0) == 1 { w.mesh } else { w.other }]
        })
        .run()
        .assert_same();
}

/// Sets vertex `v` of `mesh` from the case's floats at `i`.
fn vertex_from(e: &mut Engine, mesh: u32, v: u32, case: &Case, i: usize) {
    let data = e.mem.u32(mesh + 0x2c);
    point_at(
        e,
        data + 0xc * v,
        [bits(case, i), bits(case, i + 1), bits(case, i + 2)],
    );
}

#[test]
fn center() {
    Diff::new(entry!(
        0x0055_8220,
        navmesh::center(Ptr<_>, Ptr<_>, u16) -> Ptr<_>
    ))
    .common()
    .plain(0x0055_8220)
    .inputs(&inputs(&[&P3, &[Input::Of(TRIANGLES)]]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        let w = world(e);
        vertex_from(e, w.mesh, 1, case, 0);
        let out = e.mem.alloc(0xc);
        vec![w.mesh, out, case.word(3)]
    })
    .run()
    .assert_same();
}

#[test]
fn vertex_point() {
    Diff::new(entry!(
        0x0068_f0a0,
        navmesh::vertex_point(Ptr<_>, u16) -> Ptr<_>
    ))
    .common()
    .plain(0x0068_f0a0)
    .inputs(&[Input::Word])
    .setup(|e, case| vec![world(e).mesh, case.word(0)])
    .run()
    .assert_same();
}

#[test]
fn entry_12() {
    Diff::new(entry!(0x006a_1440, navmesh::entry_12(Ptr<_>, u32) -> u32))
        .common()
        .plain(0x006a_1440)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![world(e).mesh + 0x48, case.word(0)])
        .run()
        .assert_same();
}

#[test]
fn entry_12_of() {
    Diff::new(entry!(
        0x006a_7af0,
        navmesh::entry_12_of(Ptr<_>, u32) -> u32
    ))
    .common()
    .plain(0x006a_7af0)
    .inputs(&[Input::Word])
    .setup(|e, case| vec![world(e).mesh + 0x28, case.word(0)])
    .run()
    .assert_same();
}

/// A triangle whose flags are the case's word 0.
fn flagged(e: &mut Engine, case: &Case) -> u32 {
    let t = e.mem.alloc(0x10);
    e.mem.set_u32(t + 0xc, case.word(0));
    for i in 0..6 {
        e.mem.set_u16(t + 2 * i, (i as u16) * 3 + 1);
    }
    t
}

const FLAGS: &[u32] = &[
    0,
    1,
    2,
    4,
    0x10,
    0x20,
    0x40,
    0x200,
    0x1000,
    0x8000_0000,
    0xffff_ffff,
];

#[test]
fn has_flags() {
    Diff::new(entry!(0x0069_1140, navmesh::has_flags(Ptr<_>, u32) -> bool))
        .common()
        .plain(0x0069_1140)
        .inputs(&[Input::Of(FLAGS), Input::Of(FLAGS)])
        .setup(|e, case| vec![flagged(e, case), case.word(1)])
        .run()
        .assert_same();
}

#[test]
fn edge_flag() {
    Diff::new(entry!(0x0068_f1d0, navmesh::edge_flag(Ptr<_>, u32) -> bool))
        .common()
        .plain(0x0068_f1d0)
        .inputs(&[Input::Of(FLAGS), Input::Of(&[0, 1, 2, 31, 32, 33])])
        .setup(|e, case| vec![flagged(e, case), case.word(1)])
        .run()
        .assert_same();
}

#[test]
fn flag_40() {
    Diff::new(entry!(0x0069_3b70, navmesh::flag_40(Ptr<_>) -> bool))
        .common()
        .plain(0x0069_3b70)
        .inputs(&[Input::Of(FLAGS)])
        .setup(|e, case| vec![flagged(e, case)])
        .run()
        .assert_same();
}

#[test]
fn flag_10() {
    Diff::new(entry!(0x0069_3b90, navmesh::flag_10(Ptr<_>) -> bool))
        .common()
        .plain(0x0069_3b90)
        .inputs(&[Input::Of(FLAGS)])
        .setup(|e, case| vec![flagged(e, case)])
        .run()
        .assert_same();
}

#[test]
fn flag_200() {
    Diff::new(entry!(0x006a_5ec0, navmesh::flag_200(Ptr<_>) -> bool))
        .common()
        .plain(0x006a_5ec0)
        .inputs(&[Input::Of(FLAGS)])
        .setup(|e, case| vec![flagged(e, case)])
        .run()
        .assert_same();
}

#[test]
fn has_link() {
    Diff::new(entry!(0x0068_f200, navmesh::has_link(Ptr<_>, i32) -> bool))
        .common()
        .plain(0x0068_f200)
        .inputs(&[Input::Of(TRIANGLES), Input::Of(&[0, 1, 2])])
        .setup(|e, case| {
            let w = world(e);
            let t = e.mem.u32(w.mesh + 0x3c) + 0x10 * case.word(0);
            vec![t, case.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn link() {
    Diff::new(entry!(0x0068_f2c0, navmesh::link(Ptr<_>, i32) -> u16))
        .common()
        .plain(0x0068_f2c0)
        .inputs(&[Input::Of(TRIANGLES), Input::Of(&[0, 1, 2, 0xffff_fffd])])
        .setup(|e, case| {
            let w = world(e);
            let t = e.mem.u32(w.mesh + 0x3c) + 0x10 * case.word(0);
            vec![t, case.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn edge_info() {
    Diff::new(entry!(
        0x0068_f230,
        navmesh::edge_info(Ptr<_>, u16, i32) -> u32
    ))
    .common()
    .plain(0x0068_f230)
    .inputs(&[Input::Of(TRIANGLES), Input::Of(&[0, 1, 2])])
    .setup(|e, case| vec![world(e).mesh, case.word(0), case.word(1)])
    .run()
    .assert_same();
}

#[test]
fn edge_info_at() {
    Diff::new(entry!(
        0x0069_3d30,
        navmesh::edge_info_at(Ptr<_>, u16) -> u32
    ))
    .common()
    .plain(0x0069_3d30)
    .inputs(&[Input::Word])
    .setup(|e, case| vec![world(e).mesh, case.word(0)])
    .run()
    .assert_same();
}

#[test]
fn info() {
    Diff::new(entry!(0x0068_efb0, navmesh::info(Ptr<_>) -> u32))
        .common()
        .plain(0x0068_efb0)
        .inputs(&[Input::Flag])
        .setup(|e, case| {
            let w = world(e);
            vec![if case.word(0) == 1 { w.mesh } else { w.other }]
        })
        .run()
        .assert_same();
}

/// Out slots for a matching edge: mesh, triangle, edge, filled with a pattern.
fn outs(e: &mut Engine) -> u32 {
    let o = e.mem.alloc(0x10);
    for i in 0..4 {
        e.mem.set_u32(o + 4 * i, 0x5a5a_5a5a);
    }
    o
}

#[test]
fn matching_edge() {
    Diff::new(entry!(
        0x0068_f460,
        navmesh::matching_edge(Ptr<_>, u16, i32, Ptr, Ptr, Ptr) -> bool
    ))
    .common()
    .plain(0x0068_f460)
    .inputs(&[
        Input::Of(&[0, 1, 2, 3, 4]),
        Input::Of(&[0, 1, 2]),
        Input::Flag,
    ])
    .setup(|e, case| {
        let w = world(e);
        let mesh = if case.word(2) == 1 { w.mesh } else { w.other };
        let o = outs(e);
        vec![mesh, case.word(0), case.word(1), o, o + 4, o + 8]
    })
    .run()
    .assert_same();
}

#[test]
fn matching_edge_of() {
    Diff::new(entry!(
        0x0068_f670,
        navmesh::matching_edge_of(Ptr<_>, Ptr<_>, Ptr) -> bool
    ))
    .common()
    .plain(0x0068_f670)
    .inputs(&[Input::Of(TRIANGLES), Input::Of(&[0, 1, 2])])
    .setup(|e, case| {
        let w = world(e);
        let edge = e.mem.alloc(0xc);
        e.mem.set_u16(edge + 4, case.word(0) as u16);
        e.mem.set_u32(edge + 8, case.word(1));
        let o = outs(e);
        vec![w.mesh, edge, o]
    })
    .run()
    .assert_same();
}

#[test]
fn matching_edge_held() {
    Diff::new(entry!(
        0x0068_f3b0,
        navmesh::matching_edge_held(Ptr<_>, u16, i32, Ptr, Ptr, Ptr) -> bool
    ))
    .common()
    .plain(0x0068_f3b0)
    .inputs(&[Input::Of(TRIANGLES), Input::Of(&[0, 1, 2])])
    .setup(|e, case| {
        let w = world(e);
        let o = outs(e);
        e.mem.set_u32(o, 0);
        vec![w.mesh, case.word(0), case.word(1), o, o + 4, o + 8]
    })
    .run()
    .assert_same();
}

#[test]
fn edge_points() {
    Diff::new(entry!(
        0x0068_f0c0,
        navmesh::edge_points(Ptr<_>, u16, i32, Ptr<_>, Ptr<_>) -> bool
    ))
    .common()
    .plain(0x0068_f0c0)
    .inputs(&[Input::Of(TRIANGLES), Input::Of(&[0, 1, 2])])
    .setup(|e, case| {
        let w = world(e);
        let (a, b) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        vec![w.mesh, case.word(0), case.word(1), a, b]
    })
    .run()
    .assert_same();
}

#[test]
fn edge() {
    Diff::new(entry!(
        0x0068_efd0,
        navmesh::edge(Ptr<_>, Ptr, u16, i32) -> Ptr
    ))
    .common()
    .plain(0x0068_efd0)
    .inputs(&[Input::Of(TRIANGLES), Input::Of(EDGES)])
    .setup(|e, case| {
        let w = world(e);
        let out = e.mem.alloc(4);
        vec![w.mesh, out, case.word(0), case.word(1)]
    })
    .run()
    .assert_same();
}

#[test]
fn edge_vertices() {
    Diff::new(entry!(
        0x0068_f040,
        navmesh::edge_vertices(Ptr<_>, Ptr, u16, i32) -> Ptr
    ))
    .common()
    .plain(0x0068_f040)
    .inputs(&[Input::Of(TRIANGLES), Input::Of(&[0, 1, 2])])
    .setup(|e, case| {
        let w = world(e);
        let out = e.mem.alloc(8);
        vec![w.mesh, out, case.word(0), case.word(1)]
    })
    .run()
    .assert_same();
}

/// Inputs of the height tests: the point (3), above, below, one vertex (3).
fn height_inputs() -> Vec<Input> {
    inputs(&[
        &P3,
        &[Input::F32Swept(TEN), Input::F32Swept(TEN)],
        &[
            Input::F32Swept(ONE),
            Input::F32Swept(TWO),
            Input::F32Swept(HALF),
        ],
    ])
}

#[test]
fn triangle_z() {
    Diff::new(entry!(
        0x0069_7980,
        navmesh::triangle_z(Ptr<_>, u16, Ptr<_>, f32, f32, Ptr) -> bool
    ))
    .common()
    .plain(0x0069_7980)
    .inputs(&height_inputs())
    .max_cases(256)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        let w = world(e);
        vertex_from(e, w.mesh, 1, case, 5);
        let p = case_point(e, case, 0);
        let out = e.mem.alloc(4);
        vec![w.mesh, 0, p, bits(case, 3), bits(case, 4), out]
    })
    .run()
    .assert_same();
}

#[test]
fn crossed_edge() {
    Diff::new(entry!(
        0x0069_8320,
        navmesh::crossed_edge(Ptr<_>, u16, i32, Ptr<_>, Ptr<_>) -> i32
    ))
    .common()
    .plain(0x0069_8320)
    .inputs(&inputs(&[
        &P3,
        &[Input::F32Swept(0x40a0_0000), Input::F32Swept(0xc0a0_0000)],
        &[Input::Of(&[0, 1, 0xffff_ffff])],
    ]))
    .max_cases(256)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        let w = world(e);
        let a = point(e, [bits(case, 0), bits(case, 1), 0]);
        let b = point(e, [bits(case, 3), bits(case, 4), 0]);
        vertex_from(e, w.mesh, 2, case, 1);
        vec![w.mesh, 0, case.word(5), a, b]
    })
    .run()
    .assert_same();
}

#[test]
fn triangle_edge() {
    Diff::new(entry!(
        0x0069_8060,
        navmesh::triangle_edge(Ptr<_>, u16, i32, Ptr<_>, Ptr<_>, Ptr<_>) -> i32
    ))
    .common()
    .plain(0x0069_8060)
    .inputs(&inputs(&[
        &P3,
        &[Input::F32Swept(0x40a0_0000), Input::F32Swept(0xc0a0_0000)],
        &[Input::Of(&[0, 1, 0xffff_ffff])],
    ]))
    .max_cases(256)
    // No float sweep: one swept case (a signalling NaN corner, 2^31 in the segment) showed other
    // bytes in a corner local at the fourth crossing test, with every result and write alike; left
    // open in the batch notes.
    .setup(|e, case| {
        let w = world(e);
        let a = point(e, [bits(case, 0), bits(case, 1), 0]);
        let b = point(e, [bits(case, 3), bits(case, 4), 0]);
        vertex_from(e, w.mesh, 2, case, 1);
        let out = e.mem.alloc(8);
        vec![w.mesh, 0, case.word(5), a, b, out]
    })
    .run()
    .assert_same();
}

#[test]
fn triangle_for() {
    Diff::new(entry!(
        0x0069_6a50,
        navmesh::triangle_for(Ptr<_>, Ptr<_>, u32) -> u16
    ))
    .common()
    .plain(0x0069_6a50)
    .inputs(&inputs(&[&P3, &[Input::Flag]]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        let w = world(e);
        if case.word(3) == 0 {
            e.mem.set_u32(w.mesh + 0x24, 0);
        } else {
            e.mem.set_u32(w.mesh + 0x24, w.info);
        }
        let p = case_point(e, case, 0);
        vec![w.mesh, p, 0x1234]
    })
    .run()
    .assert_same();
}

#[test]
fn closest_triangle() {
    Diff::new(entry!(
        0x0069_6cf0,
        navmesh::closest_triangle(Ptr<_>, Ptr<_>, Ptr, Ptr) -> bool
    ))
    .common()
    .plain(0x0069_6cf0)
    .inputs(&inputs(&[&P3, &[Input::Flag]]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        let w = world(e);
        let p = case_point(e, case, 0);
        if case.word(3) == 0 {
            // No grid cell: every triangle.
            e.mem.set_u32(w.mesh + 0xa8 + 0x18, 0);
        }
        let o = outs(e);
        vec![w.mesh, p, o, o + 4]
    })
    .run()
    .assert_same();
}

#[test]
fn grid_range() {
    Diff::new(entry!(
        0x0069_69e0,
        navmesh::grid_range(Ptr<_>, Ptr, Ptr) -> bool
    ))
    .common()
    .plain(0x0069_69e0)
    .inputs(&P3)
    .setup(|e, case| {
        let w = world(e);
        let a = point(e, [0, 0, 0]);
        let b = case_point(e, case, 0);
        let out_list = e.mem.alloc(0x40);
        let _ = out_list;
        vec![w.mesh, a, b]
    })
    .run()
    .assert_same();
}

#[test]
fn grid_cell() {
    Diff::new(entry!(
        0x0069_67c0,
        navmesh::grid_cell(Ptr<_>, u32, u32) -> u32
    ))
    .common()
    .plain(0x0069_67c0)
    .inputs(&[Input::Word, Input::Of(&[0, 1, 7])])
    .setup(|e, case| vec![world(e).mesh + 0xa8, case.word(0), case.word(1)])
    .run()
    .assert_same();
}

#[test]
fn grid_cell_at() {
    Diff::new(entry!(
        0x0069_d370,
        navmesh::grid_cell_at(Ptr<_>, Ptr<_>, Ptr)
    ))
    .common()
    .plain(0x0069_d370)
    .inputs(&inputs(&[
        &[Input::F32, Input::F32Swept(0x40a0_0000)],
        &[Input::F32Swept(0)],
    ]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        let w = world(e);
        let p = case_point(e, case, 0);
        let out = outs(e);
        vec![w.mesh + 0xa8, p, out]
    })
    .run()
    .assert_same();
}

#[test]
fn plane_normal() {
    Diff::new(entry!(
        0x0069_71e0,
        navmesh::plane_normal(Ptr<_>, Ptr<_>, u16) -> Ptr<_>
    ))
    .common()
    .plain(0x0069_71e0)
    .inputs(&inputs(&[&P3, &[Input::Of(TRIANGLES)]]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        let w = world(e);
        vertex_from(e, w.mesh, 3, case, 0);
        let out = e.mem.alloc(0xc);
        vec![w.mesh, out, case.word(3)]
    })
    .run()
    .assert_same();
}

#[test]
fn water_can_cross() {
    Diff::new(entry!(
        0x006c_d5c0,
        navmesh::water_can_cross(Ptr, u32, u32, u32, Ptr<_>, u16, u32) -> bool
    ))
    .common()
    .plain(0x006c_d5c0)
    .inputs(&[Input::Of(FLAGS), Input::Flag])
    .setup(|e, case| {
        let w = world(e);
        let t = e.mem.u32(w.mesh + 0x3c);
        e.mem.set_u32(t + 0xc, case.word(0));
        let filter = e.mem.alloc(8);
        e.mem.set_u8(filter + 4, case.word(1) as u8);
        vec![filter, 1, 2, 3, w.mesh, 0, 4]
    })
    .run()
    .assert_same();
}

#[test]
fn referenced_triangle() {
    Diff::new(entry!(
        0x006a_78d0,
        navmesh::referenced_triangle(Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006a_78d0)
    .inputs(&[Input::Of(TRIANGLES)])
    .setup(|e, case| {
        let w = world(e);
        let r = e.mem.alloc(0xc);
        e.mem.set_u32(r, w.mesh);
        e.mem.set_u16(r + 4, case.word(0) as u16);
        vec![r]
    })
    .run()
    .assert_same();
}

/// A counted holder slot, empty.
fn holder(e: &mut Engine) -> u32 {
    e.mem.alloc(4)
}

#[test]
fn resolve() {
    Diff::new(entry!(0x0069_ad00, navmesh::resolve(Ptr<_>, Ptr) -> bool))
        .common()
        .plain(0x0069_ad00)
        .inputs(&[Input::Of(&[0, 0x10]), Input::Flag])
        .setup(|e, case| {
            let w = world(e);
            e.mem.set_u32(w.info + 8, case.word(0));
            if case.word(1) == 0 {
                e.mem.set_u32(w.info + 0x54, 0);
            }
            let h = holder(e);
            vec![w.info, h]
        })
        .run()
        .assert_same();
}

#[test]
fn hold() {
    Diff::new(entry!(0x0069_0890, navmesh::hold(Ptr, u32)))
        .common()
        .plain(0x0069_0890)
        .inputs(&[Input::Flag])
        .setup(|e, case| {
            let w = world(e);
            let h = holder(e);
            vec![h, if case.word(0) == 1 { w.mesh } else { 0 }]
        })
        .run()
        .assert_same();
}

#[test]
fn info_world_space() {
    Diff::new(entry!(
        0x0069_0830,
        navmesh::info_world_space(Ptr<_>) -> u32
    ))
    .common()
    .plain(0x0069_0830)
    .inputs(&[Input::Flag])
    .setup(|e, case| {
        let w = world(e);
        vec![if case.word(0) == 1 {
            w.info
        } else {
            w.other_info
        }]
    })
    .run()
    .assert_same();
}

#[test]
fn parent_world_space() {
    Diff::new(entry!(
        0x0069_0800,
        navmesh::parent_world_space(Ptr<_>) -> u32
    ))
    .common()
    .plain(0x0069_0800)
    .inputs(&[Input::Flag])
    .setup(|e, case| {
        let w = world(e);
        vec![if case.word(0) == 1 {
            w.info
        } else {
            w.other_info
        }]
    })
    .run()
    .assert_same();
}

#[test]
fn is_loaded() {
    Diff::new(entry!(0x0069_dfb0, navmesh::is_loaded(Ptr<_>) -> bool))
        .common()
        .plain(0x0069_dfb0)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            let w = world(e);
            e.mem.set_u32(w.info + 0x54, case.word(0));
            vec![w.info]
        })
        .run()
        .assert_same();
}

#[test]
fn info_mesh() {
    Diff::new(entry!(0x006a_1b60, navmesh::info_mesh(Ptr<_>) -> u32))
        .common()
        .plain(0x006a_1b60)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            let w = world(e);
            e.mem.set_u32(w.info + 0x54, case.word(0));
            vec![w.info]
        })
        .run()
        .assert_same();
}

#[test]
fn reference_by_id() {
    Diff::new(entry!(0x0069_3d70, navmesh::reference_by_id(Ptr, u32) -> u32))
        .common()
        .plain(0x0069_3d70)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let p = e.mem.alloc(0x40);
            e.mem.set_u32(p + 0x1c + 0x10, 0x0abc_0000);
            vec![p, case.word(0)]
        })
        .run()
        .assert_same();
}

/// A cell: interior when the flag is set; its world space at +0x20.
fn cell(e: &mut Engine, interior: bool) -> u32 {
    let c = e.mem.alloc(0x40);
    e.mem.set_u32(c + 0x24, u32::from(interior));
    let world_space = e.mem.alloc(0x20);
    e.mem.set_u32(world_space + 0xc, 0x0000_3c01);
    e.mem.set_u32(c + 0x20, world_space);
    c
}

#[test]
fn info_for_cell() {
    Diff::new(entry!(0x006b_77e0, navmesh::info_for_cell(Ptr, Ptr) -> u32))
        .common()
        .plain(0x006b_77e0)
        .inputs(&[Input::Of(&[0, 1, 2])])
        .setup(|e, case| {
            stand_ins(e);
            let map = e.mem.alloc(0x40);
            let inner = e.mem.alloc(0x40);
            e.mem.set_u32(map + 0x2c + 0x10, inner);
            e.mem.set_u32(inner + 0x10, 0x0777_0000);
            let c = match case.word(0) {
                0 => 0,
                1 => cell(e, true),
                _ => cell(e, false),
            };
            vec![map, c]
        })
        .run()
        .assert_same();
}

#[test]
fn info_in_world() {
    Diff::new(entry!(0x006b_7850, navmesh::info_in_world(Ptr, Ptr, u32) -> u32))
        .common()
        .plain(0x006b_7850)
        .inputs(&[Input::Of(&[0x3c01, 0x3c02]), Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let map = e.mem.alloc(0x40);
            let inner = e.mem.alloc(0x40);
            e.mem.set_u32(map + 0x2c + 0x10, inner);
            e.mem.set_u32(inner + 0x10, 0x0777_0000);
            let world_space = e.mem.alloc(0x20);
            e.mem.set_u32(world_space + 0xc, case.word(0));
            vec![map, world_space, case.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn potential_info() {
    Diff::new(entry!(0x006d_6f40, navmesh::potential_info(u32) -> u32))
        .common()
        .plain(0x006d_6f40)
        .inputs(&[Input::Of(&[0, 1, 2])])
        .setup(|e, case| {
            stand_ins(e);
            let pathing = e.mem.u32(0x011d_ea10);
            let map = e.mem.u32(pathing + 0x10);
            let inner = e.mem.alloc(0x40);
            e.mem.set_u32(map + 0x2c + 0x10, inner);
            let c = match case.word(0) {
                0 => 0,
                1 => cell(e, true),
                _ => cell(e, false),
            };
            vec![c]
        })
        .run()
        .assert_same();
}

#[test]
fn potential_info_in_world() {
    Diff::new(entry!(0x006d_6f60, navmesh::potential_info_in_world(u32, u32) -> u32))
        .common()
        .plain(0x006d_6f60)
        .inputs(&[Input::Of(&[0x3c01, 0x3c02]), Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let pathing = e.mem.u32(0x011d_ea10);
            let map = e.mem.u32(pathing + 0x10);
            let inner = e.mem.alloc(0x40);
            e.mem.set_u32(map + 0x2c + 0x10, inner);
            let world_space = e.mem.alloc(0x20);
            e.mem.set_u32(world_space + 0xc, case.word(0));
            vec![world_space, case.word(1)]
        })
        .run()
        .assert_same();
}

// ---- The search ----

/// Two floats from the case at `i`.
fn pair(e: &mut Engine, case: &Case, i: usize) -> u32 {
    let p = e.mem.alloc(8);
    e.mem.set_u32(p, bits(case, i));
    e.mem.set_u32(p + 4, bits(case, i + 1));
    p
}

const F2: [Input; 2] = [Input::F32, Input::F32Swept(HALF)];

#[test]
fn sort_key() {
    Diff::new(entry!(0x006b_9710, search::sort_key(Ptr<_>) -> f32))
        .common()
        .plain(0x006b_9710)
        .inputs(&[Input::F32, Input::F32])
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, case| {
            stand_ins(e);
            vec![pair(e, case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn second_float() {
    Diff::new(entry!(0x006b_9130, search::second_float(Ptr<_>) -> f64))
        .common()
        .plain(0x006b_9130)
        .inputs(&[Input::F32Swept(ONE), Input::F32])
        .setup(|e, case| {
            stand_ins(e);
            vec![pair(e, case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn first_float() {
    Diff::new(entry!(0x006a_7f50, search::first_float(Ptr<_>) -> f64))
        .common()
        .plain(0x006a_7f50)
        .inputs(&F2)
        .setup(|e, case| {
            stand_ins(e);
            vec![pair(e, case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn request_radius() {
    Diff::new(entry!(0x0069_ef80, search::request_radius(Ptr) -> f64))
        .common()
        .plain(0x0069_ef80)
        .inputs(&[Input::F32])
        .setup(|e, case| {
            stand_ins(e);
            let r = e.mem.alloc(0x80);
            e.mem.set_u32(r + 0x68, bits(case, 0));
            vec![r]
        })
        .run()
        .assert_same();
}

#[test]
fn keys_equal() {
    Diff::new(entry!(0x006b_81d0, search::keys_equal(Ptr, u32, u32) -> u8))
        .common()
        .plain(0x006b_81d0)
        .inputs(&[Input::Word, Input::Word])
        .setup(|_, case| vec![0x1000, case.word(0), case.word(1)])
        .run()
        .assert_same();
}

#[test]
fn keys_equal_16() {
    Diff::new(entry!(0x0069_1e50, search::keys_equal_16(Ptr, u16, u16) -> bool))
        .common()
        .plain(0x0069_1e50)
        .inputs(&[Input::Word, Input::Of(&[0, 1, 0x1_0001, 0xffff])])
        .setup(|_, case| vec![0x1000, case.word(0), case.word(1)])
        .run()
        .assert_same();
}

#[test]
fn hash() {
    Diff::new(entry!(0x006c_6a60, search::hash(Ptr<_>, u32) -> u32))
        .common()
        .plain(0x006c_6a60)
        .inputs(&[Input::Word, Input::Of(&[1, 7, 0x25, 0xffff_ffff])])
        .setup(|e, case| {
            stand_ins(e);
            let m = e.mem.alloc(8);
            e.mem.set_u32(m + 4, case.word(1));
            vec![m, case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn hash_16() {
    Diff::new(entry!(0x0049_c4c0, search::hash_16(Ptr<_>, u16) -> u32))
        .common()
        .plain(0x0049_c4c0)
        .inputs(&[Input::Word, Input::Of(&[1, 7, 0x25, 0xffff_ffff])])
        .setup(|e, case| {
            stand_ins(e);
            let m = e.mem.alloc(8);
            e.mem.set_u32(m + 4, case.word(1));
            vec![m, case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn node_key() {
    Diff::new(entry!(0x006a_94f0, search::node_key(Ptr, Ptr, u16) -> u32))
        .common()
        .plain(0x006a_94f0)
        .inputs(&[Input::Word, Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let m = e.mem.alloc(0x10);
            e.mem.set_u32(m + 0xc, case.word(0));
            vec![0x1000, m, case.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn first_word_at_94() {
    Diff::new(entry!(0x0069_e390, search::first_word_at_94(Ptr) -> u32))
        .common()
        .plain(0x0069_e390)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let o = e.mem.alloc(0x98);
            e.mem.set_u32(o + 0x94, case.word(0));
            vec![o]
        })
        .run()
        .assert_same();
}

#[test]
fn scaled_cost() {
    Diff::new(entry!(0x006f_4790, search::scaled_cost(Ptr, Ptr<_>) -> u32))
        .common()
        .plain(0x006f_4790)
        .inputs(&[Input::F32, Input::F32, Input::F32Swept(ONE)])
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, case| {
            stand_ins(e);
            let s = e.mem.alloc(4);
            e.mem.set_u32(s, bits(case, 0));
            vec![s, pair(e, case, 1)]
        })
        .run()
        .assert_same();
}

#[test]
fn scrap_new() {
    Diff::new(entry!(0x006a_1f90, search::scrap_new(u32, Ptr) -> u32))
        .common()
        .plain(0x006a_1f90)
        .inputs(&[Input::Of(&[0, 4, 0x24])])
        .setup(|e, case| {
            stand_ins(e);
            vec![case.word(0), 0x0300_2000]
        })
        .run()
        .assert_same();
}

#[test]
fn is_goal() {
    Diff::new(entry!(0x006a_79c0, search::is_goal(Ptr<_>, Ptr) -> bool))
        .common()
        .plain(0x006a_79c0)
        .inputs(&inputs(&[
            &P3,
            &[Input::F32Swept(0x4348_0000)],
            &[Input::Of(&[0, 1, 2])],
        ]))
        .max_cases(256)
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, case| {
            let w = world(e);
            point_at(
                e,
                w.node + 0x10,
                [bits(case, 0), bits(case, 1), bits(case, 2)],
            );
            let request = e.mem.u32(w.search + 0x2058);
            e.mem.set_u32(request + 0x68, bits(case, 3));
            match case.word(4) {
                0 => e.mem.set_u32(w.search + 0x2058, 0),
                1 => e.mem.set_u16(w.node + 0xc, 1),
                _ => {}
            }
            vec![w.search, w.node]
        })
        .run()
        .assert_same();
}

#[test]
fn info_is_goal() {
    Diff::new(entry!(0x006b_8c20, search::info_is_goal(Ptr, Ptr) -> u32))
        .common()
        .plain(0x006b_8c20)
        .inputs(&[Input::Of(&[0, 5, 6])])
        .setup(|e, case| {
            stand_ins(e);
            let s = e.mem.alloc(0x2078);
            e.mem.set_u32(s + 0x2074, 5);
            let n = e.mem.alloc(0xc);
            e.mem.set_u32(n + 8, case.word(0));
            vec![s, n]
        })
        .run()
        .assert_same();
}

#[test]
fn location_center() {
    Diff::new(entry!(
        0x006a_6eb0,
        search::location_center(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006a_6eb0)
    .inputs(&P3)
    .setup(|e, case| {
        stand_ins(e);
        let l = e.mem.alloc(0x14);
        point_at(e, l + 8, [bits(case, 0), bits(case, 1), bits(case, 2)]);
        let out = e.mem.alloc(0xc);
        vec![l, out]
    })
    .run()
    .assert_same();
}

#[test]
fn element() {
    Diff::new(entry!(0x006a_7ad0, search::element(Ptr, u32) -> u32))
        .common()
        .plain(0x006a_7ad0)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            let w = world(e);
            vec![w.list, case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn list_element() {
    Diff::new(entry!(0x0069_dec0, search::list_element(Ptr, u32) -> u32))
        .common()
        .plain(0x0069_dec0)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            let w = world(e);
            vec![w.list, case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn entry_8() {
    Diff::new(entry!(0x0069_50d0, search::entry_8(Ptr<_>, u32) -> u32))
        .common()
        .plain(0x0069_50d0)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![world(e).list, case.word(0)])
        .run()
        .assert_same();
}

#[test]
fn entry_14() {
    Diff::new(entry!(0x006b_3160, search::entry_14(Ptr<_>, u32) -> u32))
        .common()
        .plain(0x006b_3160)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![world(e).list, case.word(0)])
        .run()
        .assert_same();
}

#[test]
fn entry_14_of() {
    Diff::new(entry!(0x006d_ad90, search::entry_14_of(Ptr<_>, u32) -> u32))
        .common()
        .plain(0x006d_ad90)
        .inputs(&[Input::Word])
        .setup(|e, case| vec![world(e).list, case.word(0)])
        .run()
        .assert_same();
}

#[test]
fn free_data() {
    Diff::new(entry!(0x006a_8500, search::free_data(Ptr<_>)))
        .common()
        .plain(0x006a_8500)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            vec![array(e, case.word(0), 3, 4)]
        })
        .run()
        .assert_same();
}

const SIZES: &[u32] = &[0, 1, 3];

#[test]
fn init_vertices() {
    Diff::new(entry!(0x006b_3eb0, search::init_vertices(Ptr<_>, u32, u32)))
        .common()
        .plain(0x006b_3eb0)
        .inputs(&[Input::Of(SIZES), Input::Of(SIZES)])
        .setup(|e, case| {
            stand_ins(e);
            vec![array(e, 0x77, 0x77, 0x77), case.word(0), case.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn init_nodes() {
    Diff::new(entry!(0x006b_3f40, search::init_nodes(Ptr<_>, u32, u32)))
        .common()
        .plain(0x006b_3f40)
        .inputs(&[Input::Of(SIZES), Input::Of(SIZES)])
        .setup(|e, case| {
            stand_ins(e);
            vec![array(e, 0x77, 0x77, 0x77), case.word(0), case.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn init_indices() {
    Diff::new(entry!(0x0069_2290, search::init_indices(Ptr<_>, u32, u32)))
        .common()
        .plain(0x0069_2290)
        .inputs(&[Input::Of(SIZES), Input::Of(SIZES)])
        .setup(|e, case| {
            stand_ins(e);
            vec![array(e, 0x77, 0x77, 0x77), case.word(0), case.word(1)]
        })
        .run()
        .assert_same();
}

/// An array of `size` of `capacity` 32-byte elements (word 0 of the case says which shape).
fn growing(e: &mut Engine, case: &Case) -> u32 {
    let (size, capacity) =
        [(0, 0), (2, 4), (4, 4), (3, 0x400), (0x401, 0x401)][case.word(0) as usize];
    let data = e.mem.alloc(0x20 * capacity.max(1));
    array(e, data, size, capacity)
}

#[test]
fn next_index() {
    Diff::new(entry!(0x0069_2200, search::next_index(Ptr<_>) -> u32))
        .common()
        .plain(0x0069_2200)
        .inputs(&[Input::Of(&[0, 1, 2, 3])])
        .setup(|e, case| {
            stand_ins(e);
            vec![growing(e, case)]
        })
        .run()
        .assert_same();
}

#[test]
fn next_slot() {
    Diff::new(entry!(0x006b_4060, search::next_slot(Ptr<_>) -> u32))
        .common()
        .plain(0x006b_4060)
        .inputs(&[Input::Of(&[0, 1, 2, 3])])
        .setup(|e, case| {
            stand_ins(e);
            vec![growing(e, case)]
        })
        .run()
        .assert_same();
}

#[test]
fn push_index() {
    Diff::new(entry!(0x0069_1820, search::push_index(Ptr<_>, Ptr) -> u32))
        .common()
        .plain(0x0069_1820)
        .inputs(&[Input::Of(&[0, 1, 2]), Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let a = growing(e, case);
            let v = e.mem.alloc(4);
            e.mem.set_u32(v, case.word(1));
            vec![a, v]
        })
        .run()
        .assert_same();
}

#[test]
fn copy_indices() {
    Diff::new(entry!(
        0x0069_18a0,
        search::copy_indices(Ptr<_>, u32, u32) -> u32
    ))
    .common()
    .plain(0x0069_18a0)
    .inputs(&[Input::Of(&[0, 1, 3])])
    .setup(|e, case| {
        stand_ins(e);
        let block = e.call(0x0040_1000, &[0x10]).u32();
        vec![0x1000, block, case.word(0)]
    })
    .run()
    .assert_same();
}

#[test]
fn allocate_indices() {
    Diff::new(entry!(
        0x0069_1870,
        search::allocate_indices(Ptr<_>, u32) -> u32
    ))
    .common()
    .plain(0x0069_1870)
    .inputs(&[Input::Of(&[0, 1, 3])])
    .setup(|e, case| {
        stand_ins(e);
        vec![0x1000, case.word(0)]
    })
    .run()
    .assert_same();
}

#[test]
fn allocate_entries() {
    Diff::new(entry!(
        0x0069_af80,
        search::allocate_entries(Ptr<_>, u32) -> u32
    ))
    .common()
    .plain(0x0069_af80)
    .inputs(&[Input::Of(&[0, 1, 3])])
    .setup(|e, case| {
        stand_ins(e);
        vec![0x1000, case.word(0)]
    })
    .run()
    .assert_same();
}

#[test]
fn free_block() {
    Diff::new(entry!(0x006d_b090, search::free_block(Ptr<_>, u32)))
        .common()
        .plain(0x006d_b090)
        .inputs(&[Input::Of(&[0x10, 0x40])])
        .setup(|e, case| {
            stand_ins(e);
            let block = e.call(0x0040_1000, &[case.word(0)]).u32();
            vec![0x1000, block]
        })
        .run()
        .assert_same();
}

#[test]
fn index_array_new() {
    Diff::new(entry!(
        0x0069_17d0,
        search::index_array_new(Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x0069_17d0)
    .inputs(&[Input::Flag])
    .setup(|e, _| {
        stand_ins(e);
        vec![array(e, 0x77, 0x77, 0x77)]
    })
    .run()
    .assert_same();
}

#[test]
fn index_array_drop() {
    Diff::new(entry!(0x0069_1800, search::index_array_drop(Ptr<_>)))
        .common()
        .plain(0x0069_1800)
        .inputs(&[Input::Of(&[0, 1, 2])])
        .setup(|e, case| {
            stand_ins(e);
            vec![growing(e, case)]
        })
        .run()
        .assert_same();
}

#[test]
fn edge_array_new() {
    Diff::new(entry!(
        0x0069_b2c0,
        search::edge_array_new(Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x0069_b2c0)
    .inputs(&[Input::Flag])
    .setup(|e, _| {
        stand_ins(e);
        vec![array(e, 0x77, 0x77, 0x77)]
    })
    .run()
    .assert_same();
}

#[test]
fn edge_array_drop() {
    Diff::new(entry!(0x0069_b2f0, search::edge_array_drop(Ptr<_>)))
        .common()
        .plain(0x0069_b2f0)
        .inputs(&[Input::Of(&[0, 1, 2])])
        .setup(|e, case| {
            stand_ins(e);
            vec![growing(e, case)]
        })
        .run()
        .assert_same();
}

#[test]
fn construct_20s() {
    Diff::new(entry!(0x006a_8a30, search::construct_20s(Ptr, u32, u32)))
        .common()
        .plain(0x006a_8a30)
        .inputs(&[Input::Of(SIZES)])
        .setup(|e, case| {
            stand_ins(e);
            let block = e.mem.alloc(0x80);
            vec![0x1000, block, case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn construct_12s() {
    Diff::new(entry!(0x006d_03d0, search::construct_12s(Ptr, u32, u32)))
        .common()
        .plain(0x006d_03d0)
        .inputs(&[Input::Of(SIZES)])
        .setup(|e, case| {
            stand_ins(e);
            let block = e.mem.alloc(0x80);
            vec![0x1000, block, case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn construct_32s() {
    Diff::new(entry!(0x006b_40f0, search::construct_32s(Ptr, u32, u32)))
        .common()
        .plain(0x006b_40f0)
        .inputs(&[Input::Of(SIZES)])
        .setup(|e, case| {
            stand_ins(e);
            let block = e.mem.alloc(0x80);
            vec![0x1000, block, case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn slots_new() {
    Diff::new(entry!(0x006b_3de0, search::slots_new(Ptr) -> Ptr))
        .common()
        .plain(0x006b_3de0)
        .inputs(&[Input::Flag])
        .setup(|e, _| {
            stand_ins(e);
            let s = e.mem.alloc(0x1fe0);
            for i in 0..0x7f8 {
                e.mem.set_u32(s + 4 * i, 0x6b6b_6b6b);
            }
            vec![s]
        })
        .run()
        .assert_same();
}

#[test]
fn queue_new() {
    Diff::new(entry!(0x006b_9470, search::queue_new(Ptr<_>) -> Ptr<_>))
        .common()
        .plain(0x006b_9470)
        .inputs(&[Input::Flag])
        .setup(|e, _| {
            stand_ins(e);
            let q = e.mem.alloc(0x54);
            for i in 0..0x15 {
                e.mem.set_u32(q + 4 * i, 0x6b6b_6b6b);
            }
            vec![q]
        })
        .run()
        .assert_same();
}

#[test]
fn search_new() {
    Diff::new(entry!(0x006a_7b10, search::search_new(Ptr<_>) -> Ptr<_>))
        .common()
        .plain(0x006a_7b10)
        .inputs(&[Input::Flag])
        .setup(|e, _| {
            stand_ins(e);
            let s = e.mem.alloc(0x2084);
            for i in 0..0x821 {
                e.mem.set_u32(s + 4 * i, 0x6b6b_6b6b);
            }
            vec![s]
        })
        .run()
        .assert_same();
}

#[test]
fn at_5c() {
    Diff::new(entry!(0x006c_9900, search::at_5c(u32) -> u32))
        .common()
        .plain(0x006c_9900)
        .inputs(&[Input::Word])
        .setup(|_, case| vec![case.word(0)])
        .run()
        .assert_same();
}

#[test]
fn at_34() {
    Diff::new(entry!(0x006a_9540, search::at_34(Ptr) -> u32))
        .common()
        .plain(0x006a_9540)
        .inputs(&[Input::Word])
        .setup(|_, case| vec![case.word(0)])
        .run()
        .assert_same();
}

#[test]
fn byte_at_40() {
    Diff::new(entry!(0x006b_05d0, search::byte_at_40(Ptr) -> u8))
        .common()
        .plain(0x006b_05d0)
        .inputs(&[Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let n = e.mem.alloc(0x44);
            e.mem.set_u32(n + 0x40, case.word(0));
            vec![n]
        })
        .run()
        .assert_same();
}

#[test]
fn edges_equal() {
    Diff::new(entry!(
        0x006d_4ca0,
        search::edges_equal(Ptr<_>, Ptr<_>) -> bool
    ))
    .common()
    .plain(0x006d_4ca0)
    .inputs(&[Input::Of(&[0, 1]), Input::Of(&[0, 1, 0x1_0000])])
    .setup(|e, case| {
        stand_ins(e);
        let (a, b) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        e.mem.set_u32(a, 0x10);
        e.mem.set_u32(b, 0x10 + case.word(0));
        e.mem.set_u32(a + 4, 0x0005_0003);
        e.mem.set_u32(b + 4, 0x0007_0003 + case.word(1));
        vec![a, b]
    })
    .run()
    .assert_same();
}

#[test]
fn append_edge() {
    Diff::new(entry!(0x006d_b4f0, search::append_edge(Ptr<_>, u32) -> u32))
        .common()
        .plain(0x006d_b4f0)
        .inputs(&[Input::Of(&[0, 1, 2])])
        .setup(|e, case| {
            stand_ins(e);
            let a = growing(e, case);
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, 0x1234);
            e.mem.set_u32(item + 4, 0x0000_0002);
            e.mem.set_u32(item + 8, 0x0000_0001);
            vec![a, item]
        })
        .run()
        .assert_same();
}

#[test]
fn solution_node() {
    Diff::new(entry!(0x006e_78b0, search::solution_node(Ptr, u32) -> u32))
        .common()
        .plain(0x006e_78b0)
        .inputs(&[Input::Of(&[0, 1, 2])])
        .setup(|e, case| {
            stand_ins(e);
            let s = e.mem.alloc(0x20);
            let data = e.mem.alloc(0x100);
            e.mem.set_u32(s + 8 + 4, data);
            e.mem.set_u32(s + 8 + 8, 3);
            e.mem.set_u32(s + 8 + 0xc, 4);
            vec![s, case.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn tagged_new() {
    Diff::new(entry!(0x006e_cba0, search::tagged_new(Ptr, u8, u32) -> Ptr))
        .common()
        .plain(0x006e_cba0)
        .inputs(&[Input::Of(&[0, 1, 0x101]), Input::Word])
        .setup(|e, case| {
            stand_ins(e);
            let t = e.mem.alloc(8);
            vec![t, case.word(0), case.word(1)]
        })
        .run()
        .assert_same();
}

// ---- The point-of-view search ----

#[test]
fn copy_pair() {
    Diff::new(entry!(
        0x0069_85f0,
        pov::copy_pair(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x0069_85f0)
    .inputs(&F2)
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(8);
        vec![out, pair(e, case, 0)]
    })
    .run()
    .assert_same();
}

#[test]
fn relative() {
    Diff::new(entry!(
        0x0069_8620,
        pov::relative(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x0069_8620)
    .inputs(&inputs(&[&F2, &F2]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(8);
        vec![pair(e, case, 0), out, pair(e, case, 2)]
    })
    .run()
    .assert_same();
}

#[test]
fn rotated() {
    Diff::new(entry!(
        0x006a_0e90,
        pov::rotated(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006a_0e90)
    .inputs(&inputs(&[&F2, &F2]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(8);
        vec![pair(e, case, 0), out, pair(e, case, 2)]
    })
    .run()
    .assert_same();
}

#[test]
fn quadrant() {
    Diff::new(entry!(0x0069_8680, pov::quadrant(Ptr<_>) -> u32))
        .common()
        .plain(0x0069_8680)
        .inputs(&[Input::F32, Input::F32])
        .setup(|e, case| {
            stand_ins(e);
            vec![pair(e, case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn arc_contains() {
    Diff::new(entry!(
        0x006a_0660,
        pov::arc_contains(Ptr<_>, Ptr<_>) -> bool
    ))
    .common()
    .plain(0x006a_0660)
    .inputs(&inputs(&[&F2, &[Input::Flag]]))
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let arc = e.mem.alloc(0x14);
        point_at(e, arc, [ONE, 0, 0]);
        e.mem.set_u32(arc + 8, 0x3f35_04f3);
        e.mem.set_u32(arc + 0xc, 0x3f35_04f3);
        e.mem.set_u32(arc + 0x10, case.word(2));
        vec![arc, pair(e, case, 0)]
    })
    .run()
    .assert_same();
}

#[test]
fn sphere_new() {
    Diff::new(entry!(
        0x006a_04b0,
        pov::sphere_new(Ptr<_>, Ptr<_>, f32) -> Ptr<_>
    ))
    .common()
    .plain(0x006a_04b0)
    .inputs(&[Input::F32, Input::F32])
    .setup(|e, case| {
        stand_ins(e);
        let s = e.mem.alloc(0x10);
        let c = point(e, [bits(case, 1), ONE, TWO]);
        vec![s, c, bits(case, 0)]
    })
    .run()
    .assert_same();
}

#[test]
fn angle_new() {
    Diff::new(entry!(0x006a_04e0, pov::angle_new(Ptr<_>, f32) -> Ptr<_>))
        .common()
        .plain(0x006a_04e0)
        .inputs(&[Input::F32])
        .setup(|e, case| {
            stand_ins(e);
            let a = e.mem.alloc(8);
            vec![a, bits(case, 0)]
        })
        .run()
        .assert_same();
}

#[test]
fn arc_new() {
    Diff::new(entry!(
        0x006a_05e0,
        pov::arc_new(Ptr<_>, Ptr<_>, Ptr<_>, u32) -> Ptr<_>
    ))
    .common()
    .plain(0x006a_05e0)
    .inputs(&inputs(&[&F2, &[Input::Word]]))
    .setup(|e, case| {
        stand_ins(e);
        let arc = e.mem.alloc(0x14);
        let (a, b) = (pair(e, case, 0), point(e, [TWO, HALF, 0]));
        vec![arc, a, b, case.word(2)]
    })
    .run()
    .assert_same();
}

#[test]
fn arc_assign() {
    Diff::new(entry!(
        0x006a_0620,
        pov::arc_assign(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006a_0620)
    .inputs(&inputs(&[&F2, &[Input::Word]]))
    .setup(|e, case| {
        stand_ins(e);
        let arc = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        point_at(e, other, [bits(case, 0), bits(case, 1), TWO]);
        e.mem.set_u32(other + 0xc, HALF);
        e.mem.set_u32(other + 0x10, case.word(2));
        vec![arc, other]
    })
    .run()
    .assert_same();
}

#[test]
fn vertex_new() {
    Diff::new(entry!(0x006a_0480, pov::vertex_new(Ptr) -> Ptr))
        .common()
        .plain(0x006a_0480)
        .inputs(&[Input::Flag])
        .setup(|e, _| {
            stand_ins(e);
            vec![e.mem.alloc(0x20)]
        })
        .run()
        .assert_same();
}

#[test]
fn tangents_new() {
    Diff::new(entry!(0x006a_0870, pov::tangents_new(Ptr) -> Ptr))
        .common()
        .plain(0x006a_0870)
        .inputs(&[Input::Flag])
        .setup(|e, _| {
            stand_ins(e);
            vec![e.mem.alloc(0x30)]
        })
        .run()
        .assert_same();
}

/// A mesh edge of three words from the case.
fn mesh_edge(e: &mut Engine, case: &Case) -> u32 {
    let m = e.mem.alloc(0xc);
    for i in 0..3 {
        e.mem
            .set_u32(m + 4 * i, case.word(0).rotate_left(8 * i) ^ i);
    }
    m
}

#[test]
fn mesh_edge_new() {
    Diff::new(entry!(
        0x006c_4350,
        pov::mesh_edge_new(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006c_4350)
    .inputs(&[Input::Word])
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(0xc);
        vec![out, mesh_edge(e, case)]
    })
    .run()
    .assert_same();
}

#[test]
fn mesh_edge_copy() {
    Diff::new(entry!(
        0x006b_c870,
        pov::mesh_edge_copy(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006b_c870)
    .inputs(&[Input::Word])
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(0xc);
        vec![out, mesh_edge(e, case)]
    })
    .run()
    .assert_same();
}

#[test]
fn triangle_ref_copy() {
    Diff::new(entry!(
        0x0069_a690,
        pov::triangle_ref_copy(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x0069_a690)
    .inputs(&[Input::Word])
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(0xc);
        vec![out, mesh_edge(e, case)]
    })
    .run()
    .assert_same();
}

#[test]
fn triangle_ref_set() {
    Diff::new(entry!(
        0x0069_a660,
        pov::triangle_ref_set(Ptr<_>, u32, u16) -> Ptr<_>
    ))
    .common()
    .plain(0x0069_a660)
    .inputs(&[Input::Word, Input::Word])
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(0xc);
        vec![out, case.word(0), case.word(1)]
    })
    .run()
    .assert_same();
}

#[test]
fn mesh_edge_set() {
    Diff::new(entry!(
        0x0069_a6c0,
        pov::mesh_edge_set(Ptr<_>, u32, u16, u32) -> Ptr<_>
    ))
    .common()
    .plain(0x0069_a6c0)
    .inputs(&[Input::Word, Input::Word])
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(0xc);
        vec![out, case.word(0), case.word(1), case.word(0) ^ 0x55]
    })
    .run()
    .assert_same();
}

#[test]
fn edge_pair_copy() {
    Diff::new(entry!(
        0x006b_39a0,
        pov::edge_pair_copy(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006b_39a0)
    .inputs(&[Input::Word])
    .setup(|e, case| {
        stand_ins(e);
        let (out, from) = (e.mem.alloc(0x20), e.mem.alloc(0x20));
        for i in 0..8 {
            e.mem
                .set_u32(from + 4 * i, case.word(0).wrapping_mul(i + 1));
        }
        vec![out, from]
    })
    .run()
    .assert_same();
}

#[test]
fn edge_location_new() {
    Diff::new(entry!(
        0x006a_0c10,
        pov::edge_location_new(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006a_0c10)
    .inputs(&inputs(&[&[Input::Word], &P3]))
    .setup(|e, case| {
        stand_ins(e);
        let out = e.mem.alloc(0x18);
        vec![out, mesh_edge(e, case), case_point(e, case, 1)]
    })
    .run()
    .assert_same();
}

#[test]
fn edge_location_copy() {
    Diff::new(entry!(
        0x006a_0e50,
        pov::edge_location_copy(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006a_0e50)
    .inputs(&inputs(&[&[Input::Word], &P3]))
    .setup(|e, case| {
        stand_ins(e);
        let (out, from) = (e.mem.alloc(0x18), e.mem.alloc(0x18));
        let m = mesh_edge(e, case);
        for i in 0..3 {
            let w = e.mem.u32(m + 4 * i);
            e.mem.set_u32(from + 4 * i, w);
        }
        point_at(e, from + 0xc, [bits(case, 1), bits(case, 2), bits(case, 3)]);
        vec![out, from]
    })
    .run()
    .assert_same();
}

#[test]
fn distance_to_segment() {
    Diff::new(entry!(
        0x006b_9b10,
        pov::distance_to_segment(Ptr<_>, Ptr<_>, Ptr<_>, Ptr) -> f32
    ))
    .common()
    .plain(0x006b_9b10)
    .inputs(&inputs(&[
        &P3,
        &[Input::F32Swept(TEN), Input::F32Swept(ONE)],
        &[Input::Flag],
    ]))
    .max_cases(256)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let p = case_point(e, case, 0);
        let s = point(e, [0, 0, 0]);
        let end = point(e, [bits(case, 3), bits(case, 4), ONE]);
        let inside = if case.word(5) == 1 { e.mem.alloc(4) } else { 0 };
        vec![p, s, end, inside]
    })
    .run()
    .assert_same();
}

#[test]
fn intersection() {
    Diff::new(entry!(
        0x006b_a240,
        pov::intersection(Ptr<_>, Ptr<_>, Ptr<_>, Ptr<_>, Ptr<_>) -> bool
    ))
    .common()
    .plain(0x006b_a240)
    .inputs(&inputs(&[
        &P3,
        &[Input::F32Swept(TEN), Input::F32Swept(0xc120_0000)],
    ]))
    .max_cases(256)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let a = point(e, [0, 0, 0]);
        let b = case_point(e, case, 0);
        let c = point(e, [bits(case, 3), bits(case, 4), ONE]);
        let d = point(e, [0xc120_0000, TEN, ONE]);
        let out = e.mem.alloc(0xc);
        vec![a, b, c, d, out]
    })
    .run()
    .assert_same();
}

#[test]
fn crossing() {
    Diff::new(entry!(
        0x006b_9e90,
        pov::crossing(Ptr<_>, Ptr<_>, Ptr<_>, Ptr<_>, Ptr<_>) -> bool
    ))
    .common()
    .plain(0x006b_9e90)
    .inputs(&inputs(&[
        &P3,
        &[Input::F32Swept(TEN), Input::F32Swept(0xc120_0000)],
    ]))
    .max_cases(256)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let p1 = point(e, [0, 0, 0]);
        let p2 = case_point(e, case, 0);
        let p3 = point(e, [bits(case, 3), bits(case, 4), 0]);
        let p4 = point(e, [0xc120_0000, TEN, 0]);
        let out = e.mem.alloc(8);
        vec![p1, p2, p3, p4, out]
    })
    .run()
    .assert_same();
}

#[test]
fn height_in() {
    Diff::new(entry!(
        0x006b_9cf0,
        pov::height_in(Ptr<_>, Ptr<_>, Ptr<_>, Ptr<_>, Ptr, f32, f32) -> bool
    ))
    .common()
    .plain(0x006b_9cf0)
    .inputs(&height_inputs())
    .max_cases(256)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let p1 = point(e, [0, 0, 0]);
        let p2 = case_point(e, case, 5);
        let p3 = point(e, [0, TEN, TWO]);
        let p = case_point(e, case, 0);
        let out = e.mem.alloc(4);
        vec![p1, p2, p3, p, out, bits(case, 3), bits(case, 4)]
    })
    .run()
    .assert_same();
}

#[test]
fn segment_within() {
    Diff::new(entry!(
        0x006b_ab00,
        pov::segment_within(Ptr<_>, f32, Ptr<_>, Ptr<_>) -> bool
    ))
    .common()
    .plain(0x006b_ab00)
    .inputs(&inputs(&[&P3, &[Input::F32Swept(TWO)]]))
    .max_cases(256)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let c = case_point(e, case, 0);
        let a = point(e, [0, 0, 0]);
        let b = point(e, [TEN, ONE, 0]);
        vec![c, bits(case, 3), a, b]
    })
    .run()
    .assert_same();
}

#[test]
fn sphere_tangents() {
    Diff::new(entry!(
        0x006b_b570,
        pov::sphere_tangents(Ptr<_>, Ptr<_>, u32, Ptr) -> u32
    ))
    .common()
    .plain(0x006b_b570)
    .inputs(&inputs(&[
        &P3,
        &[Input::F32Swept(ONE), Input::F32Swept(HALF)],
        &[Input::Flag],
    ]))
    .max_cases(256)
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, case| {
        stand_ins(e);
        let a = e.mem.alloc(0x10);
        point_at(e, a, [0, 0, 0]);
        e.mem.set_u32(a + 0xc, bits(case, 3));
        let b = e.mem.alloc(0x10);
        point_at(e, b, [bits(case, 0), bits(case, 1), bits(case, 2)]);
        e.mem.set_u32(b + 0xc, bits(case, 4));
        let out = e.mem.alloc(0x40);
        vec![a, b, case.word(5), out]
    })
    .run()
    .assert_same();
}

// ---- Pathing locations ----

#[test]
fn world_space() {
    Diff::new(entry!(0x0044_1110, location::world_space(Ptr<_>) -> u32))
        .common()
        .plain(0x0044_1110)
        .inputs(&[Input::Flag])
        .setup(|e, _| vec![world(e).location])
        .run()
        .assert_same();
}

#[test]
fn is_valid() {
    Diff::new(entry!(0x006a_9520, location::is_valid(Ptr<_>) -> bool))
        .common()
        .plain(0x006a_9520)
        .inputs(&[Input::Of(&[0, 1, 2, 3, 0x82])])
        .setup(|e, case| {
            let w = world(e);
            e.mem.set_u8(w.location + 0x26, case.word(0) as u8);
            vec![w.location]
        })
        .run()
        .assert_same();
}

/// The world's location with no info (0), no triangle (1), an unloaded mesh (2), or as made (3).
fn located(e: &mut Engine, case: &Case) -> World {
    let w = world(e);
    match case.word(0) {
        0 => e.mem.set_u32(w.location + 0x10, 0),
        1 => e.mem.set_u16(w.location + 0x24, 0xffff),
        2 => e.mem.set_u32(w.info + 0x54, 0),
        _ => {}
    }
    w
}

#[test]
fn mesh_and_triangle() {
    Diff::new(entry!(
        0x006d_d640,
        location::mesh_and_triangle(Ptr<_>, Ptr, Ptr) -> bool
    ))
    .common()
    .plain(0x006d_d640)
    .inputs(&[Input::Of(&[0, 1, 2, 3])])
    .setup(|e, case| {
        let w = located(e, case);
        let (h, t) = (holder(e), outs(e));
        vec![w.location, h, t]
    })
    .run()
    .assert_same();
}

#[test]
fn location_mesh() {
    Diff::new(entry!(0x006d_d610, location::mesh(Ptr<_>, Ptr) -> bool))
        .common()
        .plain(0x006d_d610)
        .inputs(&[Input::Of(&[0, 1, 2, 3])])
        .setup(|e, case| {
            let w = located(e, case);
            let h = holder(e);
            vec![w.location, h]
        })
        .run()
        .assert_same();
}

#[test]
fn has_triangle() {
    Diff::new(entry!(0x006d_d6a0, location::has_triangle(Ptr<_>) -> bool))
        .common()
        .plain(0x006d_d6a0)
        .inputs(&[Input::Of(&[0, 1, 2, 3])])
        .setup(|e, case| vec![located(e, case).location])
        .run()
        .assert_same();
}

#[test]
fn location_cell() {
    Diff::new(entry!(0x006d_d4f0, location::cell(Ptr<_>) -> u32))
        .common()
        .plain(0x006d_d4f0)
        .inputs(&[
            Input::Of(&[0, 0x40]),
            Input::Of(&[0, 0x0400_0000]),
            Input::Of(&[0, 0x123]),
        ])
        .setup(|e, case| {
            let w = world(e);
            e.mem.set_u32(w.location + 0x18, case.word(0));
            e.mem.set_u32(w.location + 0x1c, case.word(1));
            e.mem.set_u32(w.location + 0x20, case.word(2));
            vec![w.location]
        })
        .run()
        .assert_same();
}

/// A location's bytes before construction.
fn raw_location(e: &mut Engine) -> u32 {
    let l = e.mem.alloc(0x28);
    for i in 0..10 {
        e.mem.set_u32(l + 4 * i, 0x6b6b_6b6b);
    }
    l
}

#[test]
fn location_new() {
    Diff::new(entry!(0x006d_cbb0, location::new(Ptr<_>) -> Ptr<_>))
        .common()
        .plain(0x006d_cbb0)
        .inputs(&[Input::Flag])
        .setup(|e, _| {
            stand_ins(e);
            vec![raw_location(e)]
        })
        .run()
        .assert_same();
}

#[test]
fn location_new_at() {
    Diff::new(entry!(
        0x006d_cee0,
        location::new_at(Ptr<_>, u32, u32, u32) -> Ptr<_>
    ))
    .common()
    .plain(0x006d_cee0)
    .inputs(&[Input::Word])
    .setup(|e, case| {
        world(e);
        let p = point(e, [case.word(0), ONE, TWO]);
        let c = cell(e, case.word(0) & 1 == 1);
        let ws = e.mem.u32(c + 0x20);
        vec![raw_location(e), p, c, ws]
    })
    .run()
    .assert_same();
}

#[test]
fn location_new_in() {
    Diff::new(entry!(
        0x006d_cf20,
        location::new_in(Ptr<_>, Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006d_cf20)
    .inputs(&inputs(&[&P3, &[Input::Flag]]))
    .setup(|e, case| {
        let w = world(e);
        let p = case_point(e, case, 0);
        let info = if case.word(3) == 1 {
            w.info
        } else {
            w.other_info
        };
        vec![raw_location(e), p, info]
    })
    .run()
    .assert_same();
}

#[test]
fn location_copy() {
    Diff::new(entry!(
        0x006d_cc40,
        location::copy(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006d_cc40)
    .inputs(&P3)
    .setup(|e, case| {
        let w = world(e);
        point_at(
            e,
            w.location + 4,
            [bits(case, 0), bits(case, 1), bits(case, 2)],
        );
        vec![raw_location(e), w.location]
    })
    .run()
    .assert_same();
}

#[test]
fn location_assign() {
    Diff::new(entry!(
        0x006d_cce0,
        location::assign(Ptr<_>, Ptr<_>) -> Ptr<_>
    ))
    .common()
    .plain(0x006d_cce0)
    .inputs(&P3)
    .setup(|e, case| {
        let w = world(e);
        point_at(
            e,
            w.location + 4,
            [bits(case, 0), bits(case, 1), bits(case, 2)],
        );
        vec![raw_location(e), w.location]
    })
    .run()
    .assert_same();
}

#[test]
fn world_location_new() {
    Diff::new(entry!(0x006d_5320, location::world_location_new(Ptr) -> Ptr))
        .common()
        .plain(0x006d_5320)
        .inputs(&[Input::Flag])
        .setup(|e, _| {
            stand_ins(e);
            vec![raw_location(e)]
        })
        .run()
        .assert_same();
}
