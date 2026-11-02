//! `Havok SDK/hkpmoppaabbcastvirtualmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMoppAabbCastVirtualMachine` (Xbox PDB) walks the bytecode of a MOPP tree with a ray (two
//! 16-byte points). The machine object (`this`) holds: the MOPP code object at `+0x10` (the
//! origin vector at `+0x10`, the scale at `+0x1C`, the code bytes at `+0x20`), the inverse
//! scale at `+0x14`, a word at `+0x18`, the nearest hit fraction found so far at `+0x1C` and
//! the best one at `+0x20`, the output at `+0x24`, a second argument at `+0x28`, the cast
//! record at `+0x2C` and a mask at `+0x30`. The "query" record the walk reads (0x34 bytes) is
//! two vectors (`+0` and `+0x10`), a float at `+0x20`, a word at `+0x24`, the scale at `+0x28`
//! and the words `+0x2C` and `+0x30`. The member names are guesses and not confirmed in the
//! exe, except the ones of the Xbox PDB.

#[allow(unused_imports)]
use crate::prelude::*;

/// A byte in the exe image (`01269b0c`): non-zero once the product key check has passed.
const LICENSE_FLAG: u32 = 0x0126_9b0c;
/// `hkpProcessFlyingColors` (Xbox PDB, `00df22f0`, cdecl, one byte pointer), translated in
/// another unit and called by address.
const LICENSE_CHECK: u32 = 0x00df_22f0;
/// Constructor of `hkErrStream` (`00cb1600`, `(this, buffer, size)`).
const ERR_STREAM_CONSTRUCT: u32 = 0x00cb_1600;
/// Writes text into the stream (`00cc01c0`, `(this, text)`) and destroys it (`00cc00e0`).
const OSTREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The global pointer to the Havok error reporter (`01268360`); its vtable offset 0xc reports
/// a message `(level, id, text, file, line)` and returns non-zero to request a break.
const ERROR_REPORTER: u32 = 0x0126_8360;
/// The message `Unknown command - This mopp data has been corrupted (check for m...` (`010d5100`)
/// and the source file string (`010d50c0`).
const CORRUPTED_TEXT: u32 = 0x010d_5100;
const ERROR_FILE: u32 = 0x010d_50c0;
/// The double constants of the exe: 2^-16 (`010d5198`) and 3.0 (`01021928`).
const SCALE_SHIFT: f64 = 1.52587890625e-05;
const THREE: f64 = 3.0;

/// The four floats of a 16-byte vector.
fn vec4(e: &Engine, at: u32) -> [f32; 4] {
    [
        e.mem.f32(at),
        e.mem.f32(at + 4),
        e.mem.f32(at + 8),
        e.mem.f32(at + 12),
    ]
}

fn set_vec4(e: &mut Engine, at: u32, v: [f32; 4]) {
    for (i, lane) in v.iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, *lane);
    }
}

/// `(1 - t) * a + b * t` lane by lane, the single-precision SSE sequence of the exe.
fn lerp(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    let mut r = [0.0f32; 4];
    for i in 0..4 {
        r[i] = (1.0 - t) * a[i] + b[i] * t;
    }
    r
}

/// `FLD` of a single float: the value as the FPU holds it (a signalling NaN is quieted).
fn ld(e: &Engine, at: u32) -> f64 {
    f64::from(x87_quiet(e.mem.f32(at)))
}

fn byte(e: &Engine, at: u32) -> u32 {
    u32::from(e.mem.u8(at))
}

/// `count` bytes at `at` as a big-endian number.
fn big_endian(e: &Engine, at: u32, count: u32) -> u32 {
    let mut value = 0u32;
    for i in 0..count {
        value = (value << 8) | byte(e, at + i);
    }
    value
}

// Translated from 00da7180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The query record copy `00da7180` makes, `RET 4` with the source: `this` gets the 0x20 bytes
/// of the two vectors, the floats at `+0x20` and `+0x28` (through the FPU) and the words
/// `+0x24`, `+0x2C` and `+0x30`.
pub fn fn_00da7180(e: &mut Engine, this: Ptr, source: Ptr) {
    let to = this.addr();
    let from = source.addr();
    set_vec4(e, to, vec4(e, from));
    set_vec4(e, to + 0x10, vec4(e, from + 0x10));
    let value = x87_quiet(e.mem.f32(from + 0x20));
    e.mem.set_f32(to + 0x20, value);
    let word = e.mem.u32(from + 0x24);
    e.mem.set_u32(to + 0x24, word);
    let value = x87_quiet(e.mem.f32(from + 0x28));
    e.mem.set_f32(to + 0x28, value);
    let word = e.mem.u32(from + 0x2c);
    e.mem.set_u32(to + 0x2c, word);
    let word = e.mem.u32(from + 0x30);
    e.mem.set_u32(to + 0x30, word);
}

/// What the interval commands leave for the common part of the walk: the bounds `lo` and `hi`
/// of the node, the ray's coordinates `r0` and `r1` along the node's axis, and the lengths of
/// the two child blocks.
struct Clip {
    lo: f32,
    hi: f32,
    r0: f32,
    r1: f32,
    near: u32,
    far: u32,
}

/// The update of the best hit the walk does after each subtree: when the current fraction
/// (`this + 0x1C`) is below the best one (`this + 0x20`) it becomes the best one and the far
/// point of the ray (`ray + 0x10`) is rebuilt from the cast record as the point at that
/// fraction, scaled by the query's scale (`query + 0x28`) and moved to its origin.
fn update_best(e: &mut Engine, me: u32, query: u32, ray: u32) {
    let fraction = x87_quiet(e.mem.f32(me + 0x1c));
    e.mem.set_f32(me + 0x20, fraction);
    let top = e.mem.u32(me + 0x2c);
    let start = vec4(e, top);
    let end = vec4(e, top + 0x10);
    let code = e.mem.u32(me + 0x10);
    let origin = vec4(e, code + 0x10);
    let mut point = [0.0f32; 4];
    for i in 0..4 {
        point[i] = ((1.0 - fraction) * start[i] + end[i] * fraction) - origin[i];
    }
    let scale = e.mem.f32(query + 0x28);
    let offset = vec4(e, query);
    for i in 0..4 {
        point[i] = scale * point[i];
        point[i] -= offset[i];
    }
    set_vec4(e, ray + 0x10, point);
}

/// Makes `query` the local record at `local` (copying it) unless it is already.
fn own_query(e: &mut Engine, query: u32, local: u32) -> u32 {
    if query != local {
        fn_00da7180(e, Ptr::new(local), Ptr::new(query));
    }
    local
}

/// The report of a command byte no case knows (`hkErrStream`, the reporter's virtual
/// function at 0xc, then `INT3`, which this translation panics on: it is a breakpoint trap
/// in the exe, not a call).
fn report_corrupted(e: &mut Engine, stream: u32, buffer: u32) {
    e.call(ERR_STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
    e.call(OSTREAM_WRITE_TEXT, &args![stream, CORRUPTED_TEXT]);
    let reporter: u32 = e.global(ERROR_REPORTER);
    let wanted_break = e
        .vcall(
            reporter,
            0xc,
            &args![3u32, 0x1298_feddu32, buffer, ERROR_FILE, 0x209u32],
        )
        .u32();
    if wanted_break != 0 {
        panic!("00da7f1f: the exe's INT3 trap (the error reporter asked for a break)");
    }
    e.call(OSTREAM_DESTRUCT, &args![stream]);
}

// Translated from 00da71c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppAabbCastVirtualMachine::queryRayOnTree` (Xbox PDB), `RET 0x10` with `(query, code,
/// ray, bits)`: interprets the MOPP bytecode at `code` for the ray `ray` (the start vector at
/// `+0`, the end vector at `+0x10`) in the frame of `query` (see the unit comment). `bits` is
/// the offset of the current code block (set by the jump commands, `>> 9` it is the chunk
/// number the leaf ids carry).
///
/// The commands move the origin and scale of the frame (1 to 4), skip or jump (5 to 7, 0xC,
/// 0x70), add to the frame's id word (9 to 0xB, 0x60 to 0x6B), test the ray against an interval
/// on an axis or a combination of axes (0x10 to 0x25, which split the ray at the interval and
/// walk the two child blocks, recursively), clip the ray (0x26 to 0x2B), or end at a leaf
/// (0x30 to 0x53), where the mediator's shape key function is called for the hit. The word at
/// `query + 0x30` is kept by commands 0x60 to 0x6B, whose writes to the other three slots of
/// the group land in unused stack words (modelled as such).
pub fn hkp_mopp_aabb_cast_virtual_machine_query_ray_on_tree(
    e: &mut Engine,
    this: Ptr,
    query: Ptr,
    code: Ptr,
    ray: Ptr,
    bits: u32,
) {
    let me = this.addr();
    let ray = ray.addr();
    // The frame is laid out like the exe's: the local query record at +0x50, scratch words
    // at +0x80, the leaf call's structure at +0x40, the error stream at +0x94 and its buffer
    // at +0xD0, the copies of the ray at +0xB0 and +0xC0.
    e.with_stack(0x2d0, |e, frame| {
        let f = frame.addr();
        let local = f + 0x50;
        let mut query = query.addr();
        let mut pc = code.addr();
        let mut bits = bits;
        loop {
            let op = byte(e, pc);
            let mut axis: i32 = 999;
            let clip = match op {
                0 => return,
                1..=4 => {
                    let offset = [
                        byte(e, pc + 1) as f32,
                        byte(e, pc + 2) as f32,
                        byte(e, pc + 3) as f32,
                        0.0,
                    ];
                    let scale = (1u32 << op) as f32;
                    let mut start = vec4(e, ray);
                    let mut end = vec4(e, ray + 0x10);
                    for i in 0..4 {
                        start[i] = scale * (start[i] - offset[i]);
                        end[i] = scale * (end[i] - offset[i]);
                    }
                    set_vec4(e, ray, start);
                    set_vec4(e, ray + 0x10, end);
                    let origin = vec4(e, query);
                    let mut corner = [0.0f32; 4];
                    for i in 0..4 {
                        corner[i] = (offset[i] + origin[i]) * scale;
                    }
                    let end_vector = vec4(e, query + 0x10);
                    let mut far_corner = [0.0f32; 4];
                    for i in 0..4 {
                        far_corner[i] = end_vector[i] * scale;
                    }
                    let word = e.mem.u32(query + 0x24).wrapping_add(op);
                    let width = (f64::from(e.mem.f32(query + 0x28)) * f64::from(scale)) as f32;
                    let depth = (f64::from(scale) * f64::from(e.mem.f32(query + 0x20))) as f32;
                    let last = e.mem.u32(query + 0x30);
                    let second = e.mem.u32(query + 0x2c);
                    set_vec4(e, local, corner);
                    e.mem.set_u32(local + 0x24, word);
                    e.mem.set_f32(local + 0x28, width);
                    set_vec4(e, local + 0x10, far_corner);
                    e.mem.set_f32(local + 0x20, depth);
                    e.mem.set_u32(local + 0x30, last);
                    e.mem.set_u32(local + 0x2c, second);
                    query = local;
                    pc = pc.wrapping_add(4);
                    continue;
                }
                5 => {
                    pc = pc.wrapping_add(byte(e, pc + 1)).wrapping_add(2);
                    continue;
                }
                6 => {
                    pc = pc.wrapping_add(big_endian(e, pc + 1, 2)).wrapping_add(3);
                    continue;
                }
                7 => {
                    pc = pc.wrapping_add(big_endian(e, pc + 1, 3)).wrapping_add(4);
                    continue;
                }
                9 => {
                    let add = byte(e, pc + 1);
                    query = own_query(e, query, local);
                    let value = e.mem.u32(local + 0x2c).wrapping_add(add);
                    e.mem.set_u32(local + 0x2c, value);
                    pc = pc.wrapping_add(2);
                    continue;
                }
                0xa => {
                    let add = big_endian(e, pc + 1, 2);
                    query = own_query(e, query, local);
                    let value = e.mem.u32(local + 0x2c).wrapping_add(add);
                    e.mem.set_u32(local + 0x2c, value);
                    pc = pc.wrapping_add(3);
                    continue;
                }
                0xb => {
                    let value = big_endian(e, pc + 1, 4);
                    query = own_query(e, query, local);
                    e.mem.set_u32(local + 0x2c, value);
                    pc = pc.wrapping_add(5);
                    continue;
                }
                0xc => {
                    bits = big_endian(e, pc + 1, 2) << 9;
                    let base = e.mem.u32(e.mem.u32(me + 0x10) + 0x20);
                    pc = base.wrapping_add(bits);
                    continue;
                }
                0xd => {
                    pc = pc.wrapping_add(5);
                    continue;
                }
                0x10..=0x12 => {
                    let a = op - 0x10;
                    let bound = ld(e, query + 0x10 + 4 * a);
                    axis = a as i32;
                    let clip = Clip {
                        lo: (f64::from(byte(e, pc + 2)) - bound) as f32,
                        hi: (f64::from(byte(e, pc + 1)) + bound) as f32,
                        r0: x87_quiet(e.mem.f32(ray + 4 * a)),
                        r1: x87_quiet(e.mem.f32(ray + 0x10 + 4 * a)),
                        near: byte(e, pc + 3),
                        far: 0,
                    };
                    pc = pc.wrapping_add(4);
                    clip
                }
                0x13..=0x1c => {
                    let b1 = byte(e, pc + 1) as i32;
                    let b2 = byte(e, pc + 2) as i32;
                    let p = |i: u32| ld(e, ray + 4 * i);
                    let (center, high, low, r0, r1) = match op {
                        0x13..=0x18 => {
                            let (x, y) = match op {
                                0x13 | 0x14 => (ld(e, query + 0x18), ld(e, query + 0x14)),
                                0x15 | 0x16 => (ld(e, query + 0x18), ld(e, query + 0x10)),
                                _ => (ld(e, query + 0x14), ld(e, query + 0x10)),
                            };
                            let center = ((x + y) * 2.0) as f32;
                            let (i1, i2) = if op & 1 == 1 {
                                (2 * b1, 2 * b2)
                            } else {
                                (2 * b1 - 255, 2 * b2 - 255)
                            };
                            let (r0, r1) = match op {
                                0x13 => (p(2) + p(1), p(6) + p(5)),
                                0x14 => (p(1) - p(2), p(5) - p(6)),
                                0x15 => (p(2) + p(0), p(6) + p(4)),
                                0x16 => (p(0) - p(2), p(4) - p(6)),
                                0x17 => (p(1) + p(0), p(5) + p(4)),
                                _ => (p(0) - p(1), p(4) - p(5)),
                            };
                            (center, i1, i2, r0, r1)
                        }
                        _ => {
                            let center = x87_quiet(e.mem.f32(query + 0x20));
                            let (m1, m2) = match op {
                                0x19 => (3 * b1, 3 * b2),
                                0x1a | 0x1b => (3 * (b1 - 0x55), 3 * (b2 - 0x55)),
                                _ => (3 * (b1 - 0xaa), 3 * (b2 - 0xaa)),
                            };
                            let (r0, r1) = match op {
                                0x19 => ((p(1) + p(0)) + p(2), (p(5) + p(4)) + p(6)),
                                0x1a => ((p(1) + p(0)) - p(2), (p(5) + p(4)) - p(6)),
                                0x1b => ((p(0) - p(1)) + p(2), (p(4) - p(5)) + p(6)),
                                _ => ((p(0) - p(1)) - p(2), (p(4) - p(5)) - p(6)),
                            };
                            (center, m1, m2, r0, r1)
                        }
                    };
                    let clip = Clip {
                        lo: (f64::from(low) - f64::from(center)) as f32,
                        hi: (f64::from(center) + f64::from(high)) as f32,
                        r0: r0 as f32,
                        r1: r1 as f32,
                        near: byte(e, pc + 3),
                        far: 0,
                    };
                    pc = pc.wrapping_add(4);
                    clip
                }
                0x20..=0x22 => {
                    let a = op - 0x20;
                    let b1 = f64::from(byte(e, pc + 1));
                    let bound = ld(e, query + 0x10 + 4 * a);
                    let widened = (1.0 + b1) as f32;
                    axis = a as i32;
                    let clip = Clip {
                        lo: (b1 - bound) as f32,
                        hi: (bound + f64::from(widened)) as f32,
                        r0: x87_quiet(e.mem.f32(ray + 4 * a)),
                        r1: x87_quiet(e.mem.f32(ray + 0x10 + 4 * a)),
                        near: byte(e, pc + 2),
                        far: 0,
                    };
                    pc = pc.wrapping_add(3);
                    clip
                }
                0x23..=0x25 => {
                    let a = op - 0x23;
                    let bound = ld(e, query + 0x10 + 4 * a);
                    axis = a as i32;
                    let clip = Clip {
                        lo: (f64::from(byte(e, pc + 2)) - bound) as f32,
                        hi: (f64::from(byte(e, pc + 1)) + bound) as f32,
                        r0: x87_quiet(e.mem.f32(ray + 4 * a)),
                        r1: x87_quiet(e.mem.f32(ray + 0x10 + 4 * a)),
                        near: big_endian(e, pc + 5, 2),
                        far: big_endian(e, pc + 3, 2),
                    };
                    pc = pc.wrapping_add(7);
                    clip
                }
                0x26..=0x2b => {
                    let (a, lo, hi);
                    if op <= 0x28 {
                        a = op - 0x26;
                        let bound = ld(e, query + 0x10 + 4 * a);
                        lo = (f64::from(byte(e, pc + 1)) - bound) as f32;
                        hi = (f64::from(byte(e, pc + 2)) + bound) as f32;
                        pc = pc.wrapping_add(3);
                    } else {
                        a = op - 0x29;
                        let inverse = f64::from(e.mem.f32(me + 0x14));
                        let scale = f64::from(e.mem.f32(query + 0x28));
                        let first = f64::from(big_endian(e, pc + 1, 3));
                        let second = f64::from(big_endian(e, pc + 4, 3));
                        let low_bound = ld(e, query + 4 * a);
                        let high_bound = ld(e, query + 0x10 + 4 * a);
                        let t1 = (first * inverse) as f32;
                        let t2 = (f64::from(t1) * scale) as f32;
                        let t3 = (f64::from(t2) - low_bound) as f32;
                        lo = (f64::from(t3) - high_bound) as f32;
                        let u1 = (second * inverse) as f32;
                        let u2 = (f64::from(u1) * scale) as f32;
                        let u3 = (f64::from(u2) - low_bound) as f32;
                        hi = (f64::from(u3) + high_bound) as f32;
                        pc = pc.wrapping_add(7);
                    }
                    let r0 = x87_quiet(e.mem.f32(ray + 4 * a));
                    let r1 = x87_quiet(e.mem.f32(ray + 0x10 + 4 * a));
                    let side;
                    if r1 > r0 {
                        if lo > r1 || hi < r0 {
                            return;
                        }
                        side = 0u32;
                    } else {
                        if lo > r0 || hi < r1 {
                            return;
                        }
                        side = 1;
                    }
                    let start = vec4(e, ray);
                    let end = vec4(e, ray + 0x10);
                    let d0 = (f64::from(r0) - f64::from(hi)) as f32;
                    let d1 = (f64::from(r1) - f64::from(hi)) as f32;
                    if f64::from(d1) * f64::from(d0) < 0.0 {
                        let t = (f64::from(d0) / (f64::from(d0) - f64::from(d1))) as f32;
                        // The slot of the side that lies beyond the upper bound.
                        let at = if side == 0 { ray + 0x10 } else { ray };
                        set_vec4(e, at, lerp(start, end, t));
                    }
                    let e0 = (f64::from(r0) - f64::from(lo)) as f32;
                    let e1 = (f64::from(r1) - f64::from(lo)) as f32;
                    if f64::from(e1) * f64::from(e0) < 0.0 {
                        let t = (f64::from(e0) / (f64::from(e0) - f64::from(e1))) as f32;
                        let at = if side == 0 { ray } else { ray + 0x10 };
                        set_vec4(e, at, lerp(start, end, t));
                    }
                    continue;
                }
                0x30..=0x53 => {
                    let value = match op {
                        0x30..=0x4f => op - 0x30,
                        0x50 => byte(e, pc + 1),
                        0x51 => big_endian(e, pc + 1, 2),
                        0x52 => big_endian(e, pc + 1, 3),
                        _ => big_endian(e, pc + 1, 4),
                    };
                    let mask = e.mem.u32(me + 0x30);
                    let chunk = (((bits as i32) >> 9) << 8) as u32 & mask;
                    let id = chunk | e.mem.u32(query + 0x2c).wrapping_add(value);
                    let top = e.mem.u32(me + 0x2c);
                    let holder = e.mem.u32(e.mem.u32(top + 0x38));
                    let table = e.mem.u32(holder + 0x34);
                    let object = if table == 0 { 0 } else { table + 0x10 };
                    let buffer = f + 0xd0;
                    let shape = e.vcall(object, 0x14, &args![id, buffer]).u32();
                    let top = e.mem.u32(me + 0x2c);
                    let words = e.mem.u32(top + 0x30);
                    let tail = e.mem.u32(words + 0xc) + 0xc;
                    let (word34, word38) = (e.mem.u32(top + 0x34), e.mem.u32(top + 0x38));
                    let flag = e
                        .vcall(tail, 4, &args![f + 0x3f, words, word34, word38, object, id])
                        .u32();
                    if e.mem.u8(flag) == 0 {
                        return;
                    }
                    let top = e.mem.u32(me + 0x2c);
                    let holder = e.mem.u32(top + 0x38);
                    let list = e.mem.u32(top + 0x30);
                    let table = e.mem.u32(list);
                    let key = byte(
                        e,
                        e.mem
                            .u32(me + 0x18)
                            .wrapping_add(0xd)
                            .wrapping_shl(5)
                            .wrapping_add(e.mem.u32(shape + 0xc))
                            .wrapping_add(table),
                    );
                    e.mem.set_u32(f + 0x40, shape);
                    e.mem.set_u32(f + 0x44, id);
                    let extra = e.mem.u32(holder + 8);
                    e.mem.set_u32(f + 0x48, extra);
                    e.mem.set_u32(f + 0x4c, holder);
                    let routine = e
                        .mem
                        .u32(table.wrapping_add(key * 0x14).wrapping_add(0x9ac));
                    let (out, context) = (e.mem.u32(me + 0x24), e.mem.u32(me + 0x28));
                    let word34 = e.mem.u32(top + 0x34);
                    e.call(routine, &args![word34, f + 0x40, list, out, context]);
                    let fraction = x87_quiet(e.mem.f32(e.mem.u32(me + 0x24) + 4));
                    e.mem.set_f32(me + 0x1c, fraction);
                    return;
                }
                0x60..=0x6b => {
                    let (value, length) = match op {
                        0x60..=0x63 => (byte(e, pc + 1), 2),
                        0x64..=0x67 => (big_endian(e, pc + 1, 2), 3),
                        _ => (big_endian(e, pc + 1, 4), 5),
                    };
                    pc = pc.wrapping_add(length);
                    e.mem.set_u32(f + 0x80 + 4 * (op & 3), value);
                    // The query's word at +0x30 survives the copy.
                    let kept = e.mem.u32(local + 0x30);
                    query = own_query(e, query, local);
                    e.mem.set_u32(local + 0x30, kept);
                    continue;
                }
                0x70 => {
                    bits = big_endian(e, pc + 1, 4);
                    let base = e.mem.u32(e.mem.u32(me + 0x10) + 0x20);
                    pc = base.wrapping_add(bits);
                    continue;
                }
                _ => {
                    report_corrupted(e, f + 0x94, f + 0xd0);
                    continue;
                }
            };
            let Clip {
                lo,
                hi,
                r0,
                r1,
                near,
                far,
            } = clip;
            // The ray lies outside the interval: only the far block is left.
            if lo > r1 && r0 < lo {
                pc = pc.wrapping_add(far);
                continue;
            }
            pc = pc.wrapping_add(near);
            if hi < r0 && hi < r1 {
                continue;
            }
            let start = vec4(e, ray);
            let end = vec4(e, ray + 0x10);
            let d0 = (f64::from(r0) - f64::from(hi)) as f32;
            let d1 = (f64::from(r1) - f64::from(hi)) as f32;
            let c0 = (f64::from(r0) - f64::from(lo)) as f32;
            let c1 = (f64::from(r1) - f64::from(lo)) as f32;
            let copy = f + 0xb0;
            if d1 > d0 {
                // The far block first, with the ray clipped at the upper bound.
                let mut clipped = end;
                if f64::from(d1) * f64::from(d0) < 0.0 {
                    let t = (f64::from(d0) / (f64::from(d0) - f64::from(d1))) as f32;
                    clipped = lerp(start, end, t);
                }
                set_vec4(e, copy, start);
                set_vec4(e, copy + 0x10, clipped);
                let other = pc.wrapping_add(far).wrapping_sub(near);
                hkp_mopp_aabb_cast_virtual_machine_query_ray_on_tree(
                    e,
                    this,
                    Ptr::new(query),
                    Ptr::new(other),
                    Ptr::new(copy),
                    bits,
                );
                if f64::from(c1) * f64::from(c0) < 0.0 {
                    let t = (f64::from(c0) / (f64::from(c0) - f64::from(c1))) as f32;
                    let new_start = lerp(vec4(e, ray), vec4(e, ray + 0x10), t);
                    set_vec4(e, ray, new_start);
                }
                if e.mem.f32(me + 0x1c) < e.mem.f32(me + 0x20) {
                    update_best(e, me, query, ray);
                    if axis < 3 && lo > ld(e, ray + 0x10 + 4 * axis as u32) as f32 {
                        return;
                    }
                }
            } else {
                // The near block first, with the ray clipped at the lower bound.
                let mut clipped = end;
                if f64::from(c1) * f64::from(c0) < 0.0 {
                    let t = (f64::from(c0) / (f64::from(c0) - f64::from(c1))) as f32;
                    clipped = lerp(start, end, t);
                }
                set_vec4(e, copy, start);
                set_vec4(e, copy + 0x10, clipped);
                hkp_mopp_aabb_cast_virtual_machine_query_ray_on_tree(
                    e,
                    this,
                    Ptr::new(query),
                    Ptr::new(pc),
                    Ptr::new(copy),
                    bits,
                );
                if f64::from(d1) * f64::from(d0) < 0.0 {
                    let t = (f64::from(d0) / (f64::from(d0) - f64::from(d1))) as f32;
                    let new_start = lerp(vec4(e, ray), vec4(e, ray + 0x10), t);
                    set_vec4(e, ray, new_start);
                }
                if e.mem.f32(me + 0x1c) < e.mem.f32(me + 0x20) {
                    update_best(e, me, query, ray);
                    if axis < 3 && (ld(e, ray + 0x10 + 4 * axis as u32) as f32) > hi {
                        return;
                    }
                }
                pc = pc.wrapping_add(far).wrapping_sub(near);
            }
        }
    });
}

// Translated from 00da8180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppAabbCastVirtualMachine::aabbCast` (Xbox PDB), `RET 0xC` with `(record, output,
/// context)`: after the one-time product check (`00df22f0` on the byte `01269b0c`, nothing
/// happens if it still reads 0), stores the record, the code object (`[[record + 0x38]] +
/// 0x14`), the word `[[[record + 0x34]] + 0xC]` at `+0x18`, the arguments at `+0x24` and
/// `+0x28`, 1.0 at `+0x1C` and `+0x20` and the inverse of the code's scale at `+0x14`; builds the
/// query record from the record's vector at `+0x20` (scaled by the code's scale times 2^-16)
/// and the ray from the record's two vectors (moved by the code's origin and scaled the
/// same way); sets the mask at `+0x30` (0 when the first code byte is 0x0D, all ones
/// otherwise) and walks the tree (`queryRayOnTree`).
pub fn hkp_mopp_aabb_cast_virtual_machine_aabb_cast(
    e: &mut Engine,
    this: Ptr,
    record: Ptr,
    output: u32,
    context: u32,
) {
    let me = this.addr();
    if e.mem.u8(LICENSE_FLAG) == 0 {
        e.call(LICENSE_CHECK, &args![LICENSE_FLAG]);
        if e.mem.u8(LICENSE_FLAG) == 0 {
            return;
        }
    }
    let record = record.addr();
    e.mem.set_u32(me + 0x2c, record);
    let code_object = e.mem.u32(e.mem.u32(e.mem.u32(record + 0x38)) + 0x14);
    e.mem.set_u32(me + 0x10, code_object);
    e.mem.set_u32(me + 0x28, context);
    e.mem.set_u32(me + 0x24, output);
    let word = e.mem.u32(e.mem.u32(e.mem.u32(record + 0x34)) + 0xc);
    e.mem.set_f32(me + 0x1c, 1.0);
    e.mem.set_f32(me + 0x20, 1.0);
    e.mem.set_u32(me + 0x18, word);
    let code_bytes = e.mem.u32(code_object + 0x20);
    let unit = ld(e, code_object + 0x1c);
    e.mem.set_f32(me + 0x14, (1.0 / unit) as f32);
    let scale = (unit * SCALE_SHIFT) as f32;
    e.with_stack(0x6c, |e, frame| {
        let b = frame.addr();
        // The query record at +0x2C of the frame, the ray at +0xC.
        let query = b + 0x2c;
        let ray = b + 0xc;
        set_vec4(e, query, [0.0; 4]);
        let direction = vec4(e, record + 0x20);
        let mut scaled = [0.0f32; 4];
        for i in 0..4 {
            scaled[i] = scale * direction[i];
        }
        set_vec4(e, query + 0x10, scaled);
        let sum = scaled[2] + (scaled[1] + scaled[0]);
        e.mem.set_f32(query + 0x20, (f64::from(sum) * THREE) as f32);
        e.mem.set_u32(query + 0x24, 0);
        e.mem.set_f32(query + 0x28, scale);
        e.mem.set_u32(query + 0x2c, 0);
        e.mem.set_u32(query + 0x30, 0);
        let origin = vec4(e, code_object + 0x10);
        let start = vec4(e, record);
        let end = vec4(e, record + 0x10);
        let mut moved_start = [0.0f32; 4];
        let mut moved_end = [0.0f32; 4];
        for i in 0..4 {
            moved_start[i] = scale * (start[i] - origin[i]);
            moved_end[i] = scale * (end[i] - origin[i]);
        }
        set_vec4(e, ray, moved_start);
        set_vec4(e, ray + 0x10, moved_end);
        let mask = if e.mem.u8(code_bytes) != 0x0d {
            0
        } else {
            0xffff_ffffu32
        };
        e.mem.set_u32(me + 0x30, mask);
        hkp_mopp_aabb_cast_virtual_machine_query_ray_on_tree(
            e,
            this,
            Ptr::new(query),
            Ptr::new(code_bytes),
            Ptr::new(ray),
            0,
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_7180, fn_00da7180(Ptr, Ptr)),
        entry!(
            0x00da_71c0,
            hkp_mopp_aabb_cast_virtual_machine_query_ray_on_tree(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00da_8180,
            hkp_mopp_aabb_cast_virtual_machine_aabb_cast(Ptr, Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const SHAPE_SLOT: u32 = 0x0900_0201;
    const TAIL_SLOT: u32 = 0x0900_0202;
    const ROUTINE: u32 = 0x0900_0203;
    const REPORT_SLOT: u32 = 0x0900_0204;
    /// A chain word that the key lookup of the leaf turns into the offset 0x1a0 (the shift
    /// drops the high bits).
    const CHAIN_WORD: u32 = 0x0800_0000;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= 1e-5 * b.abs().max(1e-3)
    }

    /// A machine with a one-leaf shape database behind it: the shape lookup, the leaf test
    /// and the routine the leaf finally calls are doubles.
    struct Rig {
        e: Engine,
        this: u32,
        top: u32,
        rec: u32,
        ray: u32,
        base: u32,
        out: u32,
        code_object: u32,
        shape: u32,
        holder: u32,
        list: u32,
        tail_flag: u32,
        ids: Rc<RefCell<Vec<u32>>>,
        calls: Rc<RefCell<Vec<Vec<u32>>>>,
        fraction: Rc<Cell<f32>>,
    }

    fn rig(code: &[u8]) -> Rig {
        let mut e = Engine::new();
        // The tests run without the exe, so its data range is mapped.
        e.map(0x0100_0000, 0x0030_0000);
        let ids: Rc<RefCell<Vec<u32>>> = Rc::default();
        let calls: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let fraction = Rc::new(Cell::new(1.0f32));
        let shape = e.mem.alloc(0x20);
        let sink = Rc::clone(&ids);
        e.register_double(SHAPE_SLOT, move |_, a| {
            sink.borrow_mut().push(a[1]);
            Ret {
                eax: shape,
                ..Ret::default()
            }
        });
        let tail_flag = e.mem.alloc(4);
        e.mem.set_u8(tail_flag, 1);
        e.register_double(TAIL_SLOT, move |_, _| Ret {
            eax: tail_flag,
            ..Ret::default()
        });
        let sink = Rc::clone(&calls);
        let given = Rc::clone(&fraction);
        e.register_double(ROUTINE, move |e, a| {
            // The five arguments and then the four words of the structure the second one
            // points to.
            let mut row = a.to_vec();
            for i in 0..4 {
                row.push(e.mem.u32(a[1] + 4 * i));
            }
            sink.borrow_mut().push(row);
            e.mem.set_f32(a[3] + 4, given.get());
            Ret::default()
        });
        let shape_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(shape_vtable + 0x14, SHAPE_SLOT);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x10, shape_vtable);
        let inner = e.mem.alloc(0x40);
        e.mem.set_u32(inner + 0x34, table);
        let code_object = e.mem.alloc(0x40);
        e.mem.set_u32(inner + 0x14, code_object);
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(holder, inner);
        e.mem.set_u32(holder + 8, 0x7777);
        let base = e.mem.alloc(0x1100);
        e.mem.write(base, code);
        e.mem.set_u32(code_object + 0x20, base);
        e.mem.set_f32(code_object + 0x1c, 2.0);
        let tail_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(tail_vtable + 4, TAIL_SLOT);
        let tail_object = e.mem.alloc(0x20);
        e.mem.set_u32(tail_object, tail_vtable);
        let keys = e.mem.alloc(0x1000);
        e.mem.set_u8(keys + 0x1a0, 2);
        e.mem.set_u32(keys + 2 * 0x14 + 0x9ac, ROUTINE);
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(list, keys);
        e.mem.set_u32(list + 0xc, tail_object - 0xc);
        let chain = e.mem.alloc(0x20);
        let chain_end = e.mem.alloc(0x20);
        e.mem.set_u32(chain, chain_end);
        e.mem.set_u32(chain_end + 0xc, CHAIN_WORD);
        let top = e.mem.alloc(0x60);
        e.mem.set_u32(top + 0x30, list);
        e.mem.set_u32(top + 0x34, chain);
        e.mem.set_u32(top + 0x38, holder);
        let out = e.mem.alloc(0x10);
        e.mem.set_f32(out + 4, 1.0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, code_object);
        e.mem.set_f32(this + 0x14, 1.0);
        e.mem.set_f32(this + 0x1c, 1.0);
        e.mem.set_f32(this + 0x20, 1.0);
        e.mem.set_u32(this + 0x24, out);
        e.mem.set_u32(this + 0x28, 0x5555);
        e.mem.set_u32(this + 0x2c, top);
        let rec = e.mem.alloc(0x40);
        e.mem.set_f32(rec + 0x28, 1.0);
        let ray = e.mem.alloc(0x20);
        Rig {
            e,
            this,
            top,
            rec,
            ray,
            base,
            out,
            code_object,
            shape,
            holder,
            list,
            tail_flag,
            ids,
            calls,
            fraction,
        }
    }

    impl Rig {
        fn run(&mut self, bits: u32) {
            let (this, rec, base, ray) = (self.this, self.rec, self.base, self.ray);
            self.e.call(0x00da71c0, &args![this, rec, base, ray, bits]);
        }

        fn set_ray_x(&mut self, start: f32, end: f32) {
            set_vec4(&mut self.e, self.ray, [start, 0.0, 0.0, 0.0]);
            set_vec4(&mut self.e, self.ray + 0x10, [end, 0.0, 0.0, 0.0]);
        }

        fn ray_x(&self) -> (f32, f32) {
            (self.e.mem.f32(self.ray), self.e.mem.f32(self.ray + 0x10))
        }

        fn visited(&self) -> Vec<u32> {
            self.ids.borrow().clone()
        }
    }

    /// The code of a split on axis 0 with the interval [5, 6] (the half extent is 0): the first
    /// block is the leaf 1, the second the leaf 2.
    const SPLIT: [u8; 5] = [0x20, 5, 1, 0x31, 0x32];

    #[test]
    fn the_record_copy_moves_vectors_floats_and_words() {
        let mut e = Engine::new();
        let from = e.mem.alloc(0x40);
        let to = e.mem.alloc(0x40);
        for i in 0..8 {
            e.mem.set_f32(from + 4 * i, i as f32 + 0.5);
        }
        e.mem.set_u32(from + 0x24, 0x1111);
        e.mem.set_u32(from + 0x2c, 0x2222);
        e.mem.set_u32(from + 0x30, 0x3333);
        e.mem.set_u32(from + 0x34, 0x4444);
        // A signalling NaN comes out quiet where the exe copies through the FPU.
        e.mem.set_u32(from + 0x20, 0x7f80_0001);
        e.mem.set_u32(from + 0x28, 0x7fa0_0000);
        e.call(0x00da7180, &args![to, from]);
        for i in 0..8 {
            assert_eq!(e.mem.f32(to + 4 * i), i as f32 + 0.5);
        }
        assert_eq!(e.mem.u32(to + 0x20), 0x7fc0_0001);
        assert_eq!(e.mem.u32(to + 0x24), 0x1111);
        assert_eq!(e.mem.u32(to + 0x28), 0x7fe0_0000);
        assert_eq!(e.mem.u32(to + 0x2c), 0x2222);
        assert_eq!(e.mem.u32(to + 0x30), 0x3333);
        assert_eq!(e.mem.u32(to + 0x34), 0);
    }

    #[test]
    fn leaves_report_the_value_bytes_as_the_id() {
        for (code, id) in [
            (vec![0x31u8], 1u32),
            (vec![0x4f], 0x1f),
            (vec![0x50, 9], 9),
            (vec![0x51, 1, 2], 0x102),
            (vec![0x52, 1, 2, 3], 0x01_0203),
            (vec![0x53, 1, 2, 3, 4], 0x0102_0304),
        ] {
            let mut r = rig(&code);
            r.run(0);
            assert_eq!(r.visited(), vec![id]);
        }
    }

    #[test]
    fn a_leaf_calls_the_routine_with_the_shape_and_stores_the_fraction() {
        let mut r = rig(&[0x31]);
        r.fraction.set(0.75);
        r.run(0);
        let calls = r.calls.borrow();
        assert_eq!(calls.len(), 1);
        let row = &calls[0];
        // The chain word of the cast record, the stack structure, the list, the output and
        // the second argument; then the structure (shape, id, extra word, holder).
        assert_eq!(row[0], r.e.mem.u32(r.top + 0x34));
        assert_eq!(row[2], r.list);
        assert_eq!(row[3], r.out);
        assert_eq!(row[4], 0x5555);
        assert_eq!(row[5..], [r.shape, 1, 0x7777, r.holder][..]);
        assert_eq!(r.e.mem.f32(r.this + 0x1c), 0.75);
    }

    #[test]
    fn the_chunk_and_the_frame_word_are_or_ed_into_the_id() {
        let mut r = rig(&[0x31]);
        r.e.mem.set_u32(r.this + 0x30, 0xffff_ffff);
        r.e.mem.set_u32(r.rec + 0x2c, 0x40);
        // bits 0x600 is chunk 3: (3 << 8) | (0x40 + 1).
        r.run(0x600);
        assert_eq!(r.visited(), vec![0x341]);
        // The mask clears the chunk.
        let mut r = rig(&[0x31]);
        r.e.mem.set_u32(r.rec + 0x2c, 0x40);
        r.run(0x600);
        assert_eq!(r.visited(), vec![0x41]);
    }

    #[test]
    fn a_zero_flag_from_the_leaf_test_ends_the_walk_before_the_routine() {
        let mut r = rig(&[0x31]);
        r.e.mem.set_u8(r.tail_flag, 0);
        r.e.mem.set_f32(r.this + 0x1c, 0.5);
        r.run(0);
        assert_eq!(r.visited(), vec![1]);
        assert!(r.calls.borrow().is_empty());
        assert_eq!(r.e.mem.f32(r.this + 0x1c), 0.5);
    }

    #[test]
    fn skips_and_jumps_move_the_code_pointer() {
        for code in [
            vec![5u8, 2, 0xff, 0xff, 0x31],
            vec![6, 0, 1, 0xff, 0x31],
            vec![7, 0, 0, 1, 0xff, 0x31],
            vec![0x0d, 0, 0, 0, 0, 0x31],
        ] {
            let mut r = rig(&code);
            r.run(0);
            assert_eq!(r.visited(), vec![1], "{code:?}");
        }
        // 0x0c jumps to the code base plus (value << 9) and sets the chunk number.
        let mut r = rig(&[0x0c, 0, 2]);
        r.e.mem.write(r.base + 0x400, &[0x31]);
        r.e.mem.set_u32(r.this + 0x30, 0xffff_ffff);
        r.run(0);
        assert_eq!(r.visited(), vec![0x201]);
        // 0x70 does the same with a 32-bit offset.
        let mut r = rig(&[0x70, 0, 0, 6, 0]);
        r.e.mem.write(r.base + 0x600, &[0x31]);
        r.e.mem.set_u32(r.this + 0x30, 0xffff_ffff);
        r.run(0);
        assert_eq!(r.visited(), vec![0x301]);
    }

    #[test]
    fn the_frame_commands_work_on_a_copy_of_the_record() {
        let mut r = rig(&[0x09, 5, 0x31]);
        r.e.mem.set_u32(r.rec + 0x2c, 4);
        r.run(0);
        assert_eq!(r.visited(), vec![10]);
        assert_eq!(r.e.mem.u32(r.rec + 0x2c), 4);
        let mut r = rig(&[0x0a, 1, 2, 0x31]);
        r.e.mem.set_u32(r.rec + 0x2c, 4);
        r.run(0);
        assert_eq!(r.visited(), vec![0x107]);
        let mut r = rig(&[0x0b, 0, 0, 1, 0, 0x31]);
        r.e.mem.set_u32(r.rec + 0x2c, 4);
        r.run(0);
        assert_eq!(r.visited(), vec![0x101]);
        // The commands that store a value in the scratch words carry on with a copy.
        for code in [
            vec![0x60u8, 7, 0x31],
            vec![0x61, 7, 0x31],
            vec![0x64, 0, 7, 0x31],
            vec![0x68, 0, 0, 0, 7, 0x31],
        ] {
            let mut r = rig(&code);
            r.e.mem.set_u32(r.rec + 0x2c, 4);
            r.run(0);
            assert_eq!(r.visited(), vec![5], "{code:?}");
            assert_eq!(r.e.mem.u32(r.rec + 0x2c), 4);
        }
    }

    #[test]
    fn the_scale_commands_move_and_scale_the_ray_and_the_record() {
        // The record's bound on axis 0 is 3, scaled by 2 it is 6, so the interval of 0x20 is
        // [5 - 6, 6 + 6] = [-1, 12]. The ray (7.4 on axis 0 at both ends) becomes
        // 2 * (7.4 - 1) = 12.8, above the interval: only the second block is walked.
        let mut code = vec![1, 1, 2, 3];
        code.extend(SPLIT);
        let mut r = rig(&code);
        r.e.mem.set_f32(r.rec + 0x10, 3.0);
        set_vec4(&mut r.e, r.ray, [7.4, 4.0, 5.0, 0.5]);
        set_vec4(&mut r.e, r.ray + 0x10, [7.4, 6.0, 7.0, 0.25]);
        r.run(0);
        assert_eq!(r.visited(), vec![2]);
        let expected = |value: f32, offset: f32| 2.0 * (value - offset);
        assert_eq!(r.e.mem.f32(r.ray), expected(7.4, 1.0));
        assert_eq!(r.e.mem.f32(r.ray + 4), expected(4.0, 2.0));
        assert_eq!(r.e.mem.f32(r.ray + 8), expected(5.0, 3.0));
        assert_eq!(r.e.mem.f32(r.ray + 12), expected(0.5, 0.0));
        assert_eq!(r.e.mem.f32(r.ray + 0x14), expected(6.0, 2.0));
        assert_eq!(r.e.mem.f32(r.rec + 0x10), 3.0);
        // Without the command the same ray is inside [2, 9], so both blocks are walked,
        // the second first (the ray does not rise).
        let mut r = rig(&SPLIT);
        r.e.mem.set_f32(r.rec + 0x10, 3.0);
        set_vec4(&mut r.e, r.ray, [7.4, 4.0, 5.0, 0.5]);
        set_vec4(&mut r.e, r.ray + 0x10, [7.4, 6.0, 7.0, 0.25]);
        r.run(0);
        assert_eq!(r.visited(), vec![2, 1]);
    }

    #[test]
    fn an_interval_command_walks_the_blocks_in_the_order_of_the_ray() {
        // The interval is [5, 6]. A rising ray (4 to 7) crosses it: the first block is walked
        // first with the ray clipped at the upper bound, then the second with the start of
        // the ray moved to the lower bound.
        let mut r = rig(&SPLIT);
        r.set_ray_x(4.0, 7.0);
        r.run(0);
        assert_eq!(r.visited(), vec![1, 2]);
        assert!(close(r.ray_x().0, 5.0));
        assert_eq!(r.ray_x().1, 7.0);
        // A falling ray (7 to 4) walks the second block first.
        let mut r = rig(&SPLIT);
        r.set_ray_x(7.0, 4.0);
        r.run(0);
        assert_eq!(r.visited(), vec![2, 1]);
        assert!(close(r.ray_x().0, 6.0));
        // Below the interval only the first block is left, above it only the second.
        let mut r = rig(&SPLIT);
        r.set_ray_x(-10.0, -10.0);
        r.run(0);
        assert_eq!(r.visited(), vec![1]);
        let mut r = rig(&SPLIT);
        r.set_ray_x(100.0, 100.0);
        r.run(0);
        assert_eq!(r.visited(), vec![2]);
    }

    #[test]
    fn a_hit_pulls_the_end_of_the_ray_in_and_can_end_the_walk() {
        // The first leaf reports the fraction 0.25; the point at that fraction of the cast
        // record (0 to 8 on axis 0) is 2, and the end of the ray becomes 2 * 2 - 1 = 3 (scale
        // 2, offset 1), below the lower bound 5: nothing else is walked.
        let mut r = rig(&SPLIT);
        r.fraction.set(0.25);
        r.e.mem.set_f32(r.top + 0x10, 8.0);
        r.e.mem.set_f32(r.rec, 1.0);
        r.e.mem.set_f32(r.rec + 0x28, 2.0);
        r.set_ray_x(4.0, 7.0);
        r.run(0);
        assert_eq!(r.visited(), vec![1]);
        assert_eq!(r.ray_x().1, 3.0);
        assert_eq!(r.e.mem.f32(r.this + 0x20), 0.25);
        // With the cast record reaching to 40 the point is 10: the second block is walked.
        let mut r = rig(&SPLIT);
        r.fraction.set(0.25);
        r.e.mem.set_f32(r.top + 0x10, 40.0);
        r.set_ray_x(4.0, 7.0);
        r.run(0);
        assert_eq!(r.visited(), vec![1, 2]);
        assert_eq!(r.ray_x().1, 10.0);
        // A falling ray hits the second block first; the end point 10 is above the upper
        // bound 6, which ends the walk, while 2 does not.
        let mut r = rig(&SPLIT);
        r.fraction.set(0.25);
        r.e.mem.set_f32(r.top + 0x10, 40.0);
        r.set_ray_x(7.0, 4.0);
        r.run(0);
        assert_eq!(r.visited(), vec![2]);
        let mut r = rig(&SPLIT);
        r.fraction.set(0.25);
        r.e.mem.set_f32(r.top + 0x10, 8.0);
        r.set_ray_x(7.0, 4.0);
        r.run(0);
        assert_eq!(r.visited(), vec![2, 1]);
    }

    #[test]
    fn the_other_interval_commands_choose_their_axes() {
        // 0x10 reads the interval from the bytes (high, low, length of the first block).
        let mut r = rig(&[0x10, 6, 5, 1, 0x31, 0x32]);
        r.set_ray_x(4.0, 7.0);
        r.run(0);
        assert_eq!(r.visited(), vec![1, 2]);
        // 0x11 works on axis 1.
        let mut r = rig(&[0x11, 6, 5, 1, 0x31, 0x32]);
        set_vec4(&mut r.e, r.ray, [0.0, 7.0, 0.0, 0.0]);
        set_vec4(&mut r.e, r.ray + 0x10, [0.0, 4.0, 0.0, 0.0]);
        r.run(0);
        assert_eq!(r.visited(), vec![2, 1]);
        // 0x23 has explicit lengths for both blocks: the second (near) and the third (far).
        let code = [0x23, 6, 5, 0, 2, 0, 1, 0x31, 0x32, 0x33];
        let mut r = rig(&code);
        r.set_ray_x(4.0, 7.0);
        r.run(0);
        assert_eq!(r.visited(), vec![3, 2]);
        let mut r = rig(&code);
        r.set_ray_x(7.0, 4.0);
        r.run(0);
        assert_eq!(r.visited(), vec![2, 3]);
        let mut r = rig(&code);
        r.set_ray_x(-10.0, -10.0);
        r.run(0);
        assert_eq!(r.visited(), vec![3]);
    }

    #[test]
    fn the_sum_commands_test_a_combination_of_axes() {
        // 0x19: the sum of the axes, against [3 * 1 - 2, 2 + 3 * 1] = [1, 5].
        for (point, expected) in [(1.0f32, vec![2u32, 1]), (2.0, vec![2]), (0.0, vec![1])] {
            let mut r = rig(&[0x19, 1, 1, 1, 0x31, 0x32]);
            r.e.mem.set_f32(r.rec + 0x20, 2.0);
            set_vec4(&mut r.e, r.ray, [point; 4]);
            set_vec4(&mut r.e, r.ray + 0x10, [point; 4]);
            r.run(0);
            assert_eq!(r.visited(), expected, "{point}");
        }
        // 0x13: twice the sum of two bounds is 6, the interval [2 * 1 - 6, 6 + 2 * 2] =
        // [-4, 10], the sums of the ray 5 or 22.
        for (z, expected) in [(3.0f32, vec![2u32, 1]), (20.0, vec![2])] {
            let mut r = rig(&[0x13, 2, 1, 1, 0x31, 0x32]);
            r.e.mem.set_f32(r.rec + 0x14, 1.0);
            r.e.mem.set_f32(r.rec + 0x18, 2.0);
            set_vec4(&mut r.e, r.ray, [0.0, 2.0, z, 0.0]);
            set_vec4(&mut r.e, r.ray + 0x10, [0.0, 2.0, z, 0.0]);
            r.run(0);
            assert_eq!(r.visited(), expected, "{z}");
        }
    }

    #[test]
    fn a_clip_command_cuts_the_ray_to_its_interval() {
        // [2, 3] on axis 0; the rising ray 0 to 10 is cut to 2 to 3.
        let mut r = rig(&[0x26, 2, 3, 0x31]);
        r.set_ray_x(0.0, 10.0);
        r.run(0);
        assert_eq!(r.visited(), vec![1]);
        assert!(close(r.ray_x().0, 2.0));
        assert!(close(r.ray_x().1, 3.0));
        // The falling ray 10 to 0 is cut the other way round.
        let mut r = rig(&[0x26, 2, 3, 0x31]);
        r.set_ray_x(10.0, 0.0);
        r.run(0);
        assert!(close(r.ray_x().0, 3.0));
        assert!(close(r.ray_x().1, 2.0));
        // A ray beyond the interval ends the walk.
        let mut r = rig(&[0x26, 2, 3, 0x31]);
        r.set_ray_x(5.0, 5.0);
        r.run(0);
        assert!(r.visited().is_empty());
        let mut r = rig(&[0x26, 2, 3, 0x31]);
        r.set_ray_x(-5.0, -4.0);
        r.run(0);
        assert!(r.visited().is_empty());
    }

    #[test]
    fn the_wide_clip_command_scales_its_bounds_by_the_inverse_unit() {
        // Inverse 0.5 and scale 2: the 24-bit values 8 and 16 become 8 and 16, the record
        // origin 1 gives the interval [7, 15]; the ray 0 to 20 is cut to 7 to 15.
        let mut r = rig(&[0x29, 0, 0, 8, 0, 0, 16, 0x31]);
        r.e.mem.set_f32(r.this + 0x14, 0.5);
        r.e.mem.set_f32(r.rec + 0x28, 2.0);
        r.e.mem.set_f32(r.rec, 1.0);
        r.set_ray_x(0.0, 20.0);
        r.run(0);
        assert_eq!(r.visited(), vec![1]);
        assert!(close(r.ray_x().0, 7.0));
        assert!(close(r.ray_x().1, 15.0));
    }

    #[test]
    fn a_command_byte_nobody_knows_is_reported_and_read_again() {
        unknown_command(0);
    }

    #[test]
    #[should_panic(expected = "00da7f1f: the exe's INT3 trap")]
    fn an_unknown_command_traps_with_int3_when_the_reporter_asks() {
        unknown_command(1);
    }

    fn unknown_command(wanted: u32) {
        let mut r = rig(&[0x08]);
        let log: Log = Rc::default();
        for target in [ERR_STREAM_CONSTRUCT, OSTREAM_WRITE_TEXT, OSTREAM_DESTRUCT] {
            let sink = Rc::clone(&log);
            r.e.register_double(target, move |_, a| {
                sink.borrow_mut().push((target, a.to_vec()));
                Ret::default()
            });
        }
        let sink = Rc::clone(&log);
        let base = r.base;
        r.e.register_double(REPORT_SLOT, move |e, a| {
            sink.borrow_mut().push((REPORT_SLOT, a.to_vec()));
            // The handler "repairs" the byte so the next pass ends the query.
            e.mem.set_u8(base, 0);
            Ret {
                eax: wanted,
                ..Ret::default()
            }
        });
        let vtable = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(vtable + 0xc, REPORT_SLOT);
        let reporter = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(reporter, vtable);
        r.e.set_global(ERROR_REPORTER, reporter);
        r.run(0);
        let log = log.borrow();
        let stream = log[0].1[0];
        assert_eq!(
            log[0],
            (ERR_STREAM_CONSTRUCT, vec![stream, stream + 0x3c, 0x200])
        );
        assert_eq!(log[1], (OSTREAM_WRITE_TEXT, vec![stream, CORRUPTED_TEXT]));
        assert_eq!(
            log[2],
            (
                REPORT_SLOT,
                vec![reporter, 3, 0x1298_fedd, stream + 0x3c, ERROR_FILE, 0x209]
            )
        );
        assert_eq!(log[3], (OSTREAM_DESTRUCT, vec![stream]));
        assert_eq!(log.len(), 4);
    }

    fn cast(r: &mut Rig) {
        let (this, top, out) = (r.this, r.top, r.out);
        r.e.call(0x00da8180, &args![this, top, out, 0x6666u32]);
    }

    #[test]
    fn the_cast_does_nothing_until_the_product_check_passes() {
        let mut r = rig(&[0x00]);
        r.e.mem.set_u8(LICENSE_FLAG, 0);
        r.e.mem.set_u32(r.this + 0x2c, 0);
        let asked = Rc::new(Cell::new(0u32));
        let seen = Rc::clone(&asked);
        r.e.register_double(LICENSE_CHECK, move |_, a| {
            seen.set(a[0]);
            Ret::default()
        });
        cast(&mut r);
        assert_eq!(asked.get(), LICENSE_FLAG);
        assert_eq!(r.e.mem.u32(r.this + 0x2c), 0);
        // A check that sets the flag lets the cast go on.
        let mut r = rig(&[0x00]);
        r.e.mem.set_u8(LICENSE_FLAG, 0);
        r.e.mem.set_u32(r.this + 0x2c, 0);
        r.e.register_double(LICENSE_CHECK, |e, a| {
            e.mem.set_u8(a[0], 1);
            Ret::default()
        });
        cast(&mut r);
        assert_eq!(r.e.mem.u32(r.this + 0x2c), r.top);
    }

    #[test]
    fn the_cast_fills_in_the_machine_and_walks_the_code() {
        let mut r = rig(&[0x31]);
        r.e.mem.set_u8(LICENSE_FLAG, 1);
        cast(&mut r);
        assert_eq!(r.e.mem.u32(r.this + 0x2c), r.top);
        assert_eq!(r.e.mem.u32(r.this + 0x10), r.code_object);
        assert_eq!(r.e.mem.u32(r.this + 0x28), 0x6666);
        assert_eq!(r.e.mem.u32(r.this + 0x24), r.out);
        assert_eq!(r.e.mem.u32(r.this + 0x18), CHAIN_WORD);
        assert_eq!(r.e.mem.f32(r.this + 0x14), 0.5);
        assert_eq!(r.e.mem.f32(r.this + 0x1c), 1.0);
        assert_eq!(r.e.mem.f32(r.this + 0x20), 1.0);
        assert_eq!(r.e.mem.u32(r.this + 0x30), 0);
        assert_eq!(r.visited(), vec![1]);
        assert_eq!(r.calls.borrow().len(), 1);
        // The first code byte 0x0d makes the mask all ones.
        let mut r = rig(&[0x0d, 0, 0, 0, 0, 0x00]);
        r.e.mem.set_u8(LICENSE_FLAG, 1);
        cast(&mut r);
        assert_eq!(r.e.mem.u32(r.this + 0x30), 0xffff_ffff);
        assert!(r.visited().is_empty());
    }

    /// A cast of the ray 10 to 11 on axis 0 with the half extent 2 against the code of
    /// [`SPLIT`]-like shape (first interval byte 1) at the given scale and origin.
    fn scaled_cast(unit: f32, origin_x: f32) -> Vec<u32> {
        let mut r = rig(&[0x20, 1, 1, 0x31, 0x32]);
        r.e.mem.set_u8(LICENSE_FLAG, 1);
        set_vec4(&mut r.e, r.code_object + 0x10, [origin_x, 0.0, 0.0, unit]);
        set_vec4(&mut r.e, r.top, [10.0, 0.0, 0.0, 0.0]);
        set_vec4(&mut r.e, r.top + 0x10, [11.0, 0.0, 0.0, 0.0]);
        set_vec4(&mut r.e, r.top + 0x20, [2.0, 0.0, 0.0, 0.0]);
        cast(&mut r);
        r.visited()
    }

    #[test]
    fn the_cast_moves_the_ray_to_the_code_origin_and_scales_it() {
        // The code's scale 65536 (times 2^-16) is 1: the ray 10 to 11 less the origin 8.5 is
        // 1.5 to 2.5 against the interval [1 - 2, 2 + 2], so both blocks are walked, the first
        // first (the ray rises).
        assert_eq!(scaled_cast(65536.0, 8.5), vec![1, 2]);
        // At scale 2 the same ray is 3 to 5 and the half extent 4: the interval [-3, 6].
        assert_eq!(scaled_cast(131072.0, 8.5), vec![1, 2]);
        // A ray that is not moved by the origin lies above the interval [-1, 4].
        assert_eq!(scaled_cast(65536.0, 0.0), vec![2]);
    }
}
