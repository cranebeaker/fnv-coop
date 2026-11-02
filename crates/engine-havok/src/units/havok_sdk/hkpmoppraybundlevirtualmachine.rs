//! `Havok SDK/hkpmoppraybundlevirtualmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMoppRayBundleVirtualMachine` walks a MOPP code tree with a bundle of four rays at once.
//! The rays are kept structure-of-arrays in a 0x70-byte block `B`: the six vectors `from.x`,
//! `from.y`, `from.z`, `to.x`, `to.y`, `to.z` (offsets `+0x00` ... `+0x50`, one lane per ray, in
//! the code's scaled and offset coordinates) and the active-ray mask at `+0x60`. The walk keeps
//! a small state `A` (`0x38` bytes: an offset vector, a scale exponent, a scale vector and the
//! terminal offset at `+0x30`).
//!
//! SSE code is computed lane by lane in `f32` on bit patterns (`Lanes`), x87 constants are read
//! from the exe's data, `RCPPS` is the host CPU's estimate. The code keeps the stack frame at
//! fixed offsets (`s + 0x..`), so the translation uses the same frame (`with_stack`) and offsets;
//! words the code never wrote before reading them are zero here. Functions that take a register
//! argument (`EAX`) get it as the word after `this` in the uniform form.

#[allow(unused_imports)]
use crate::prelude::*;

type Lanes = [u32; 4];

layout! {
    /// `hkpMoppRayBundleVirtualMachine` (Xbox PDB), 0xe0 bytes; offsets checked against the
    /// code below, the field names are unconfirmed.
    pub struct HkpMoppRayBundleVirtualMachine: 0xe0 {
        /// The code description (`+0x10` the origin vector, `+0x1c` the scale, `+0x20` the
        /// MOPP data).
        0x10 m_info: Ptr,
        /// The ray bundle being cast (six vectors, then the filter info at `+0x60` and the
        /// filter object at `+0x64`).
        0x30 m_rays: Ptr,
        /// The four ray outputs (`0x50` bytes each: hit fraction at `+0x10`, shape keys from
        /// `+0x20`, key count at `+0x40`).
        0xc0 m_output: Ptr,
        /// The shape container whose slot `+0x14` makes the shape of a key.
        0xcc m_container: Ptr,
        /// All ones when the code is not chunked, else zero.
        0xd0 m_chunkMask: u32,
    }
}

/// `1.0f`, the exe's `0.0f`, `255.0f`, `510.0f`, `2.0f`, `4.0f`, `8.0f` and `16.0f` floats.
const ONE_FLOAT: u32 = 0x0101_2068;
const ZERO_FLOAT: u32 = 0x0101_1d78;
const FLOAT_255: u32 = 0x0102_3cd8;
const FLOAT_510: u32 = 0x010d_7ba0;
const FLOAT_2: u32 = 0x0101_62c0;
const FLOAT_4: u32 = 0x0101_e570;
const FLOAT_8: u32 = 0x0102_918c;
const FLOAT_16: u32 = 0x0101_e57c;
/// `2^-16` as a double.
const SCALE_DOUBLE: u32 = 0x010d_5198;
/// Vector constants: all bits set, `1.0f` in every lane, the sign bit of every lane, zero.
const ALL_BITS: u32 = 0x010c_e1c0;
const ONES: u32 = 0x010c_7170;
const SIGN_BITS: u32 = 0x010c_44d0;
const ZERO_VECTOR: u32 = 0x0126_8460;
/// The lane bit of each of the four rays.
const RAY_LANE_MASKS: u32 = 0x010c_92e0;
/// The flag byte of a one-time initialisation and its routine (cdecl, the flag's address).
const INIT_FLAG: u32 = 0x0126_9cbe;
const INIT_ONCE: u32 = 0x00df_22f0;
/// The Havok error object (`01268360` holds a pointer to it), the text stream pieces
/// `00cb1600(stream, buffer, size)`, `operator<<(char*)` `00cc01c0` and the destructor
/// `00cc00e0`, and the report's text, file name and line.
const ERROR_OBJECT: u32 = 0x0126_8360;
const STREAM_CONSTRUCT: u32 = 0x00cb_1600;
const STREAM_WRITE: u32 = 0x00cc_01c0;
const STREAM_DESTRUCT: u32 = 0x00cc_00e0;
const BAD_COMMAND_TEXT: u32 = 0x010d_5100;
const SOURCE_FILE_TEXT: u32 = 0x010d_7b64;
const BAD_COMMAND_LINE: u32 = 0x346;
const BAD_COMMAND_ID: u32 = 0x1298_fedd;
/// This unit's own functions, called by address as the code does.
const RECIPROCAL_PAIR: u32 = 0x00dd_2fd0;
const COUNT_ASCENDING: u32 = 0x00dd_3120;
const QUERY_RAY_ON_TREE: u32 = 0x00dd_3210;
const COPY_STATE: u32 = 0x00dd_4a50;
const COPY_BUNDLE: u32 = 0x00dd_4a90;

fn ld(e: &Engine, address: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.u32(address.wrapping_add(4 * lane as u32)))
}

fn st(e: &mut Engine, address: u32, v: Lanes) {
    for (lane, word) in v.iter().enumerate() {
        e.mem.set_u32(address.wrapping_add(4 * lane as u32), *word);
    }
}

fn lanewise(a: Lanes, b: Lanes, f: impl Fn(f32, f32) -> f32) -> Lanes {
    std::array::from_fn(|i| f(f32::from_bits(a[i]), f32::from_bits(b[i])).to_bits())
}

fn addps(a: Lanes, b: Lanes) -> Lanes {
    lanewise(a, b, |x, y| x + y)
}

fn subps(a: Lanes, b: Lanes) -> Lanes {
    lanewise(a, b, |x, y| x - y)
}

fn mulps(a: Lanes, b: Lanes) -> Lanes {
    lanewise(a, b, |x, y| x * y)
}

/// `CMPLTPS`: all bits where `a < b` (false for NaN).
fn cmpltps(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|i| {
        if f32::from_bits(a[i]) < f32::from_bits(b[i]) {
            u32::MAX
        } else {
            0
        }
    })
}

/// `CMPLEPS`.
fn cmpleps(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|i| {
        if f32::from_bits(a[i]) <= f32::from_bits(b[i]) {
            u32::MAX
        } else {
            0
        }
    })
}

fn andps(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|i| a[i] & b[i])
}

/// `ANDNPS`: `!a & b`.
fn andnps(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|i| !a[i] & b[i])
}

fn orps(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|i| a[i] | b[i])
}

fn xorps(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|i| a[i] ^ b[i])
}

/// `SHUFPS v, v, imm` (both operands the same register).
fn shufps(v: Lanes, imm: u32) -> Lanes {
    std::array::from_fn(|i| v[((imm >> (2 * i)) & 3) as usize])
}

fn splat(word: u32) -> Lanes {
    [word; 4]
}

/// `MOVMSKPS`: the sign bits of the four lanes.
fn movmskps(v: Lanes) -> u32 {
    (0..4).fold(0, |mask, i| mask | ((v[i] >> 31) << i))
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

fn rcpps(v: Lanes) -> Lanes {
    std::array::from_fn(|i| rcp_estimate(f32::from_bits(v[i])).to_bits())
}

/// One Newton-Raphson step on the reciprocal estimate, the way the code spells it:
/// `(r + r) - ((r * d) * r)`.
fn refined_reciprocal(d: Lanes) -> Lanes {
    let r = rcpps(d);
    let correction = mulps(mulps(r, d), r);
    subps(addps(r, r), correction)
}

// Translated from 00dd2fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the vectors `a` (in `ECX`) and `b` (stack word 1), forms `r = 1 / (b - a)` (refined
/// `RCPPS`) and stores `b * r` to `result` (stack word 2) and `-(a * r)` to `out` (in `EAX`).
/// With `a` and `b` the positions of a segment's two ends relative to a plane, `b * r` is the
/// fraction of the segment from the `b` end to the plane and `-(a * r)` its complement. The
/// function is unnamed; in the uniform form the words are `(a, out, b, result)`.
pub fn fn_00dd2fd0(e: &mut Engine, a: u32, out: u32, b: u32, result: u32) {
    let b_vec = ld(e, b);
    let a_vec = ld(e, a);
    let r = refined_reciprocal(subps(b_vec, a_vec));
    st(e, result, mulps(b_vec, r));
    let a_again = ld(e, a);
    let product = mulps(a_again, r);
    st(e, out, product);
    let sign = ld(e, SIGN_BITS);
    st(e, out, xorps(product, sign));
}

// Translated from 00dd3120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the vector `starts` (in `ECX`), the active-ray mask `mask` (in `EAX`) and the vector
/// `ends` (stack word 1): counts the active rays (mask lanes ANDed with `1.0f`, summed) and
/// the active rays with `start < end`, and returns `0xf` when the active count is at most twice
/// the ascending count (at least half of the active rays ascend), else 0. The function is
/// unnamed; in the uniform form the words are `(starts, mask, ends)`.
pub fn fn_00dd3120(e: &mut Engine, starts: u32, mask: u32, ends: u32) -> u32 {
    let m = ld(e, mask);
    let ends_vec = ld(e, ends);
    let one = ld(e, ONES);
    let starts_vec = ld(e, starts);
    let below = andps(cmpltps(starts_vec, ends_vec), m);
    let zero = [0u32; 4];
    let weight = |v: Lanes| mulps(orps(andnps(v, zero), andps(v, one)), one);
    // Sum the four lanes with the two shuffle-and-add steps of the code.
    let sum_all = {
        let w = weight(m);
        let pairs = addps(shufps(w, 0x4e), w);
        addps(shufps(pairs, 0xb1), pairs)
    };
    let sum_below = {
        let w = weight(below);
        let pairs = addps(shufps(w, 0x4e), w);
        addps(shufps(pairs, 0xb1), pairs)
    };
    let all = shufps(sum_all, 0);
    let twice_below = {
        let v = shufps(sum_below, 0);
        addps(v, v)
    };
    movmskps(cmpleps(all, twice_below))
}

// Translated from 00dd4a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a walk state `A` (the unnamed 0x38-byte structure): the vectors at `+0x00` and `+0x20`
/// and the words at `+0x10`, `+0x30` and `+0x34`.
pub fn fn_00dd4a50(e: &mut Engine, dest: u32, source: u32) {
    let first = ld(e, source);
    st(e, dest, first);
    let word = e.mem.u32(source + 0x10);
    e.mem.set_u32(dest + 0x10, word);
    let scale = ld(e, source + 0x20);
    st(e, dest + 0x20, scale);
    let word = e.mem.u32(source + 0x30);
    e.mem.set_u32(dest + 0x30, word);
    let word = e.mem.u32(source + 0x34);
    e.mem.set_u32(dest + 0x34, word);
}

// Translated from 00dd4a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a ray bundle block `B` (the unnamed 0x70-byte structure): six vectors, then four words.
pub fn fn_00dd4a90(e: &mut Engine, dest: u32, source: u32) {
    for vector in 0..6 {
        let v = ld(e, source + 0x10 * vector);
        st(e, dest + 0x10 * vector, v);
    }
    for word in 0..4 {
        let value = e.mem.u32(source + 0x60 + 4 * word);
        e.mem.set_u32(dest + 0x60 + 4 * word, value);
    }
}

// Translated from 00dd4870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppRayBundleVirtualMachine::queryRayBundle` (Xbox PDB; hidden result pointer `result`,
/// which is returned): sets the machine up for the ray bundle `rays` against the MOPP code
/// description `info` (origin at `+0x10`, scale at `+0x1C`, data at `+0x20`) with the shape
/// `container`, the ray outputs `output` and the initial active-ray mask at `mask`; builds the
/// scaled ray block on the stack, walks the tree (`queryRayOnTree`) and returns the machine's
/// hit mask (`this + 0xa0`) in `result`.
#[allow(clippy::too_many_arguments)]
pub fn query_ray_bundle(
    e: &mut Engine,
    this: Ptr<HkpMoppRayBundleVirtualMachine>,
    result: Ptr,
    container: Ptr,
    info: Ptr,
    rays: Ptr,
    output: Ptr,
    mask: Ptr,
) -> Ptr {
    let this_addr = this.addr();
    let one = e.mem.u32(ONE_FLOAT);
    st(e, this_addr + 0xb0, splat(one));
    e.set(this, HkpMoppRayBundleVirtualMachine::m_container, container);
    e.set(this, HkpMoppRayBundleVirtualMachine::m_output, output);
    // The ray bundle is copied into the machine: the from vectors to +0x70, the to vectors
    // to +0x40.
    for (source, dest) in [
        (0x00, 0x70),
        (0x10, 0x80),
        (0x20, 0x90),
        (0x30, 0x40),
        (0x40, 0x50),
        (0x50, 0x60),
    ] {
        let v = ld(e, rays.addr() + source);
        st(e, this_addr + dest, v);
    }
    e.set(this, HkpMoppRayBundleVirtualMachine::m_info, info);
    let code = e.mem.u32(info.addr() + 0x20);
    let scale = e.mem.f32(info.addr() + 0x1c);
    // 1 / scale through the x87 stack (a double, then stored as a float).
    let inverse = (1.0f64 / f64::from(scale)) as f32;
    st(e, this_addr + 0x20, splat(inverse.to_bits()));
    let step = (f64::from(scale) * e.mem.f64(SCALE_DOUBLE)) as f32;
    let step_vec = splat(step.to_bits());
    let origin = ld(e, info.addr() + 0x10);
    let origin_x = shufps(origin, 0x00);
    let origin_y = shufps(origin, 0x55);
    let origin_z = shufps(origin, 0xaa);
    e.with_stack(0xc0, |e, frame| {
        let t = frame.addr();
        let from_x = ld(e, this_addr + 0x70);
        let to_x = ld(e, this_addr + 0x40);
        let from_y = ld(e, this_addr + 0x80);
        let to_y = ld(e, this_addr + 0x50);
        let from_z = ld(e, this_addr + 0x90);
        let to_z = ld(e, this_addr + 0x60);
        st(e, t + 0x10, mulps(subps(from_x, origin_x), step_vec));
        st(e, t + 0x40, mulps(step_vec, subps(to_x, origin_x)));
        st(e, t + 0x20, mulps(step_vec, subps(from_y, origin_y)));
        st(e, t + 0x50, mulps(step_vec, subps(to_y, origin_y)));
        st(e, t + 0x30, mulps(step_vec, subps(from_z, origin_z)));
        st(e, t + 0x60, mulps(subps(to_z, origin_z), step_vec));
        // The active-ray mask follows the six vectors; a zero vector, the walk state A
        // (zero offset, scale exponent and terminal offset, scale vector `step`) follows it.
        let mask_vec = ld(e, mask.addr());
        st(e, t + 0x70, mask_vec);
        st(e, t + 0x80, [0; 4]);
        e.mem.set_u32(t + 0x90, 0);
        st(e, t + 0xa0, step_vec);
        e.mem.set_u32(t + 0xb0, 0);
        e.mem.set_u32(t + 0xb4, 0);
        e.set(this, HkpMoppRayBundleVirtualMachine::m_rays, rays);
        let zero = ld(e, ZERO_VECTOR);
        st(e, this_addr + 0xa0, zero);
        let first_command = e.mem.u8(code);
        let chunk_mask = u32::from(first_command != 0x0d).wrapping_sub(1);
        e.set(
            this,
            HkpMoppRayBundleVirtualMachine::m_chunkMask,
            chunk_mask,
        );
        e.call(
            QUERY_RAY_ON_TREE,
            &args![this, t + 0x80, code, t + 0x10, 0u32],
        );
    });
    let hit = ld(e, this_addr + 0xa0);
    st(e, result.addr(), hit);
    result
}

/// The frame and machine of one `queryRayOnTree` call.
struct Walk {
    /// The stack frame base (the code's `ESP` after its prologue; the offsets below are its).
    s: u32,
    /// `this`.
    machine: u32,
    /// The ray block `B` (`EBX`).
    block: u32,
}

/// The walk state pointer and the chunk number, which commands change.
#[derive(Clone, Copy)]
struct Ctx {
    state: u32,
    chunk: u32,
}

/// `UNPCKLPS`: `[a0, b0, a1, b1]`.
fn unpcklps(a: Lanes, b: Lanes) -> Lanes {
    [a[0], b[0], a[1], b[1]]
}

/// `FILD` of a small integer, stored as a float.
fn int_float_bits(value: u32) -> u32 {
    (value as i32 as f32).to_bits()
}

/// `(a & mask) | (!mask & b)`.
fn select(mask: Lanes, a: Lanes, b: Lanes) -> Lanes {
    orps(andps(a, mask), andnps(mask, b))
}

fn add_word(e: &mut Engine, address: u32, value: u32) {
    let sum = e.mem.u32(address).wrapping_add(value);
    e.mem.set_u32(address, sum);
}

// Translated from 00dd3210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppRayBundleVirtualMachine::queryRayOnTree` (Xbox PDB; `this` in `ECX`, the walk state
/// `state`, the code pointer `code`, the ray block `block` and the chunk number `chunk` on the
/// stack, `RET 0x10`): interprets MOPP commands at `code` for the bundle of four rays in `block`
/// until the code returns or no ray is left active. Commands (by first byte):
///
/// - 0: return. 1..4: scale the block by `2^n` about an offset given in the next three bytes
///   (the state gets a new offset, exponent and scale). 5..7: jump by 8, 16 or 24 bits.
///   9..11: add to (or, for 11, set) the terminal offset (8, 16 or 32 bits). 12: jump to a code
///   chunk (sets the chunk number). 13: skip five bytes.
/// - 16..18 and 19..28: cut by a plane (an axis, or a sum or difference of axes): rays wholly
///   on one side go to one child, rays that straddle are clipped to both planes and the two
///   sub-bundles are walked (the nearer first; the hit distance of the first walk then clips
///   the second). 32..34 and 35..37: the same with a pair of planes. 38..43: keep only the
///   slab between two planes (8-bit or 24-bit plane offsets), clipping the rays to it.
/// - 48..83: terminal (a 5-bit value, or one of 8, 16, 24 or 32 bits): the shape of the key is
///   made through the container and cast against, hits recorded in the ray outputs.
/// - 96..107: set a property word (8, 16 or 32 bits). Other commands report an error.
///
/// No exception frame. The initialisation guard (`01269cbe`) is run first. The class layout
/// names are unconfirmed.
pub fn query_ray_on_tree(
    e: &mut Engine,
    this: Ptr<HkpMoppRayBundleVirtualMachine>,
    state: u32,
    code: u32,
    block: u32,
    chunk: u32,
) {
    if e.mem.u8(INIT_FLAG) == 0 {
        e.call(INIT_ONCE, &args![INIT_FLAG]);
        if e.mem.u8(INIT_FLAG) == 0 {
            return;
        }
    }
    e.with_stack(0x700, |e, frame| {
        let walk = Walk {
            s: (frame.addr() + 0x40 + 15) & !15,
            machine: this.addr(),
            block,
        };
        run(e, &walk, Ctx { state, chunk }, code);
    });
}

/// The command loop.
fn run(e: &mut Engine, w: &Walk, mut ctx: Ctx, mut pc: u32) {
    let s = w.s;
    e.mem.set_u32(s + 0x24, w.machine);
    loop {
        let edi = pc;
        let byte = |e: &Engine, n: u32| u32::from(e.mem.u8(edi.wrapping_add(n)));
        let (b0, b1, b2, b3) = (byte(e, 0), byte(e, 1), byte(e, 2), byte(e, 3));
        let b4 = byte(e, 4);
        e.mem.set_u32(s + 0x18, b2);
        let v30: Lanes = [
            int_float_bits(b0),
            int_float_bits(b1),
            int_float_bits(b2),
            int_float_bits(b3),
        ];
        st(e, s + 0x30, v30);
        e.mem.set_u32(s + 0x14, v30[3]);
        e.mem.set_u32(s + 0x20, 0x3e7);
        let word16 = (b1 << 8).wrapping_add(b2);
        let word24 = (word16 << 8).wrapping_add(b3);
        let word32 = (word24 << 8).wrapping_add(b4);
        let next: Option<u32> = match b0 {
            0 => None,
            1..=4 => {
                ctx.state = scale_command(e, w, ctx.state, b0, v30);
                Some(edi.wrapping_add(4))
            }
            5 => Some(edi.wrapping_add(b1).wrapping_add(2)),
            6 => Some(edi.wrapping_add(word16).wrapping_add(3)),
            7 => Some(edi.wrapping_add(word24).wrapping_add(4)),
            9 => {
                ctx.state = own_state(e, s, ctx.state);
                add_word(e, s + 0xa0, b1);
                Some(edi.wrapping_add(2))
            }
            10 => {
                ctx.state = own_state(e, s, ctx.state);
                add_word(e, s + 0xa0, word16);
                Some(edi.wrapping_add(3))
            }
            11 => {
                ctx.state = own_state(e, s, ctx.state);
                e.mem.set_u32(s + 0xa0, word32);
                Some(edi.wrapping_add(5))
            }
            12 => {
                ctx.chunk = word16;
                let info = e.mem.u32(w.machine + 0x10);
                let data = e.mem.u32(info + 0x20);
                Some((ctx.chunk << 9).wrapping_add(data))
            }
            13 => Some(edi.wrapping_add(5)),
            16..=18 => {
                let axis = b0 - 0x10;
                e.mem.set_u32(s + 0x20, axis);
                let x0 = ld(e, w.block + (axis << 4));
                let x1 = ld(e, w.block + ((axis + 3) << 4));
                cut_tail(e, w, ctx, edi, x0, x1, v30)
            }
            19..=28 => {
                let (planes, x0, x1) = diagonal_cut(e, w.block, b0, v30);
                cut_tail(e, w, ctx, edi, x0, x1, planes)
            }
            32..=34 => {
                let axis = b0 - 0x20;
                e.mem.set_u32(s + 0x20, axis);
                let x0 = ld(e, w.block + (axis << 4));
                let x1 = ld(e, w.block + ((axis + 3) << 4));
                let low = shufps(v30, 0x55);
                let high = addps(low, ld(e, ONES));
                let planes = [x0, x1, high, low];
                slab(e, w, ctx, edi.wrapping_add(3), planes, (0, b2))
            }
            35..=37 => {
                let axis = b0 - 0x23;
                e.mem.set_u32(s + 0x20, axis);
                let x1 = ld(e, w.block + ((axis + 3) << 4));
                let first = b4.wrapping_add(b3 << 8);
                let second = (byte(e, 5) << 8).wrapping_add(byte(e, 6));
                let x0 = ld(e, w.block + (axis << 4));
                let planes = [x0, x1, shufps(v30, 0x55), shufps(v30, 0xaa)];
                slab(e, w, ctx, edi.wrapping_add(7), planes, (first, second))
            }
            38..=40 => {
                let axis = b0 - 0x26;
                let lo = shufps(v30, 0x55);
                let hi = shufps(v30, 0xaa);
                double_cut(e, w, edi.wrapping_add(3), axis, lo, hi)
            }
            41..=43 => {
                let axis = b0 - 0x29;
                let (lo, hi) = wide_planes(e, w, ctx.state, edi, axis);
                double_cut(e, w, edi.wrapping_add(7), axis, lo, hi)
            }
            48..=79 => {
                terminal(e, w, ctx, b0 - 0x30);
                None
            }
            80 => {
                terminal(e, w, ctx, b1);
                None
            }
            81 => {
                terminal(e, w, ctx, word16);
                None
            }
            82 => {
                terminal(e, w, ctx, word24);
                None
            }
            83 => {
                terminal(e, w, ctx, word32);
                None
            }
            96..=107 => {
                let (value, length, base) = match b0 {
                    96..=99 => (b1, 2, 0xdc),
                    100..=103 => (word16, 3, 0xec),
                    _ => (word32, 5, 0xfc),
                };
                // The property words live at `s + 0xa4 + 4 * (op & 3)`.
                e.mem
                    .set_u32(s.wrapping_add(4 * b0).wrapping_sub(base), value);
                // The first one is the state's `+0x34` word, which a copy of the state
                // overwrites; keep it across the copy.
                let kept = e.mem.u32(s + 0xa4);
                ctx.state = own_state(e, s, ctx.state);
                e.mem.set_u32(s + 0xa4, kept);
                Some(edi.wrapping_add(length))
            }
            _ => {
                bad_command(e, s);
                Some(edi)
            }
        };
        match next {
            Some(p) => pc = p,
            None => return,
        }
    }
}

/// The walk state is edited in the frame's own copy (`s + 0x70`): copies `state` there unless
/// that already is the state, and returns the copy.
fn own_state(e: &mut Engine, s: u32, state: u32) -> u32 {
    let copy = s + 0x70;
    if state != copy {
        e.call(COPY_STATE, &args![copy, state]);
    }
    copy
}

/// Commands 1..4: scales the block's six vectors by `2^op` about the offset
/// `(b1, b2, b3)` (the bytes after the command, as floats) and builds the new walk state in the
/// frame (`s + 0x70`: the old offset plus the new one, times the scale; the exponent plus `op`;
/// the scale vector times the scale; the words at `+0x30` and `+0x34` as they were). Returns
/// the new state pointer.
fn scale_command(e: &mut Engine, w: &Walk, state: u32, op: u32, v30: Lanes) -> u32 {
    let (s, b) = (w.s, w.block);
    for (slot, constant) in [
        (0x220, FLOAT_2),
        (0x230, FLOAT_4),
        (0x240, FLOAT_8),
        (0x250, FLOAT_16),
    ] {
        let bits = x87_quiet_bits(e.mem.u32(constant));
        st(e, s + slot, splat(bits));
    }
    let zero = [e.mem.u32(ZERO_FLOAT), 0, 0, 0];
    let first = [v30[1], 0, 0, 0];
    let second = [v30[2], 0, 0, 0];
    let third = [v30[3], 0, 0, 0];
    let offsets = unpcklps(unpcklps(first, third), unpcklps(second, zero));
    let (ox, oy, oz) = (
        shufps(offsets, 0x00),
        shufps(offsets, 0x55),
        shufps(offsets, 0xaa),
    );
    let scale = ld(e, s + 0x210 + (op << 4));
    for (index, offset) in [(0u32, ox), (1, oy), (2, oz), (3, ox), (4, oy), (5, oz)] {
        let v = ld(e, b + 0x10 * index);
        st(e, b + 0x10 * index, mulps(scale, subps(v, offset)));
    }
    let moved = mulps(addps(ld(e, state), offsets), scale);
    st(e, s + 0x70, moved);
    let exponent = e.mem.u32(state + 0x10).wrapping_add(op);
    e.mem.set_u32(s + 0x80, exponent);
    let scaled = mulps(ld(e, state + 0x20), scale);
    st(e, s + 0x90, scaled);
    let word = e.mem.u32(state + 0x34);
    e.mem.set_u32(s + 0xa4, word);
    let word = e.mem.u32(state + 0x30);
    e.mem.set_u32(s + 0xa0, word);
    s + 0x70
}

/// Commands 19..28: the plane vector (`2 * v - c` or `3 * v - c`; lanes 1 and 2 hold the two
/// plane values) and the projections of the ray starts and ends on the diagonal axis. Returns
/// `(planes, start, end)`.
fn diagonal_cut(e: &mut Engine, b: u32, op: u32, v30: Lanes) -> (Lanes, Lanes, Lanes) {
    let c255 = splat(x87_quiet_bits(e.mem.u32(FLOAT_255)));
    let c510 = splat(x87_quiet_bits(e.mem.u32(FLOAT_510)));
    let zero = splat(0);
    let v: [Lanes; 6] = std::array::from_fn(|i| ld(e, b + 0x10 * i as u32));
    let double = addps(v30, v30);
    let triple = addps(double, v30);
    match op {
        19 => (subps(double, zero), addps(v[2], v[1]), addps(v[5], v[4])),
        20 => (subps(double, c255), subps(v[1], v[2]), subps(v[4], v[5])),
        21 => (subps(double, zero), addps(v[2], v[0]), addps(v[5], v[3])),
        22 => (subps(double, c255), subps(v[0], v[2]), subps(v[3], v[5])),
        23 => (subps(double, zero), addps(v[1], v[0]), addps(v[4], v[3])),
        24 => (subps(double, c255), subps(v[0], v[1]), subps(v[3], v[4])),
        25 => (
            subps(triple, zero),
            addps(addps(v[1], v[0]), v[2]),
            addps(addps(v[4], v[3]), v[5]),
        ),
        26 => (
            subps(triple, c255),
            subps(addps(v[1], v[0]), v[2]),
            subps(addps(v[4], v[3]), v[5]),
        ),
        27 => (
            subps(triple, c255),
            addps(subps(v[0], v[1]), v[2]),
            addps(subps(v[3], v[4]), v[5]),
        ),
        _ => (
            subps(triple, c510),
            subps(subps(v[0], v[1]), v[2]),
            subps(subps(v[3], v[4]), v[5]),
        ),
    }
}

/// The common end of the single-plane commands: reads the skip byte after the four command
/// bytes and takes the planes from lanes 1 and 2 of `planes`.
fn cut_tail(
    e: &mut Engine,
    w: &Walk,
    ctx: Ctx,
    edi: u32,
    x0: Lanes,
    x1: Lanes,
    planes: Lanes,
) -> Option<u32> {
    let skip = u32::from(e.mem.u8(edi.wrapping_add(3)));
    let high = shufps(planes, 0x55);
    let low = shufps(planes, 0xaa);
    slab(
        e,
        w,
        ctx,
        edi.wrapping_add(4),
        [x0, x1, high, low],
        (0, skip),
    )
}

/// Tests the rays against the planes `high` (`planes[2]`) and `low` (`planes[3]`) with start
/// projection `planes[0]` and end projection `planes[1]`. When no active ray reaches past
/// `low` the walk continues `first` bytes on, when none reaches below `high` it continues
/// `second` bytes on; otherwise the straddling rays are clipped into two sub-bundles, which are
/// walked in turn, the nearer first. Returns the next code pointer, or `None` to return.
fn slab(
    e: &mut Engine,
    w: &Walk,
    ctx: Ctx,
    pc: u32,
    planes: [Lanes; 4],
    skips: (u32, u32),
) -> Option<u32> {
    let (s, b) = (w.s, w.block);
    let [x0, x1, high, low] = planes;
    let (first, second) = skips;
    e.mem.set_u32(s + 0x28, first);
    e.mem.set_u32(s + 0x18, second);
    st(e, s + 0x60, high);
    st(e, s + 0x40, low);
    let all = ld(e, ALL_BITS);
    let mask = ld(e, b + 0x60);
    let below = andps(andps(cmpltps(x1, low), cmpltps(x0, low)), mask);
    let inactive = andnps(mask, all);
    st(e, s + 0xd0, inactive);
    if movmskps(orps(inactive, below)) == 0xf {
        return Some(pc.wrapping_add(first));
    }
    let above = andps(andps(cmpltps(high, x1), cmpltps(high, x0)), mask);
    st(e, s + 0x2f0, below);
    st(e, s + 0x2d0, above);
    if movmskps(orps(inactive, above)) == 0xf {
        return Some(pc.wrapping_add(second));
    }
    // Both sub-bundles are built in the frame: the first at `s + 0x220` (rays that do not lie
    // above), the second at `s + 0x3e0` (rays that do not lie below).
    let from = [ld(e, b), ld(e, b + 0x10), ld(e, b + 0x20)];
    let to = [ld(e, b + 0x30), ld(e, b + 0x40), ld(e, b + 0x50)];
    let start_low = subps(x0, low);
    let end_low = subps(x1, low);
    st(e, s + 0x390, start_low);
    st(e, s + 0x2c0, end_low);
    let start_high = subps(x0, high);
    let end_high = subps(x1, high);
    st(e, s + 0x280, andps(andnps(above, all), mask));
    st(e, s + 0x440, andps(andnps(below, all), mask));
    let zero = [0u32; 4];
    st(e, s + 0xd0, zero);
    let sign = ld(e, SIGN_BITS);
    // The plane `high` crossing.
    let crosses_high = cmpltps(mulps(end_high, start_high), zero);
    let r = refined_reciprocal(subps(start_high, end_high));
    let w1 = xorps(mulps(r, end_high), sign);
    let w2 = mulps(r, start_high);
    let rises = cmpltps(start_high, end_high);
    let rising_cross = andps(rises, crosses_high);
    let falling_cross = andps(andnps(rises, all), crosses_high);
    for axis in 0..3 {
        let position = addps(mulps(from[axis], w1), mulps(to[axis], w2));
        let bundle = s + 0x220 + 0x10 * axis as u32;
        st(e, bundle, select(falling_cross, position, from[axis]));
        st(e, bundle + 0x30, select(rising_cross, position, to[axis]));
    }
    st(e, s + 0x330, start_high);
    st(e, s + 0x370, start_high);
    st(e, s + 0x3b0, end_high);
    st(e, s + 0x350, end_high);
    // The plane `low` crossing.
    let falls = cmpltps(end_high, start_high);
    let crosses_low = cmpltps(mulps(end_low, start_low), zero);
    let r2 = refined_reciprocal(subps(start_low, end_low));
    let u1 = xorps(mulps(r2, end_low), sign);
    let u2 = mulps(r2, start_low);
    let from_mask = andps(andnps(falls, all), crosses_low);
    let to_mask = andps(falls, crosses_low);
    for axis in 0..3 {
        let position = addps(mulps(from[axis], u1), mulps(to[axis], u2));
        let bundle = s + 0x3e0 + 0x10 * axis as u32;
        st(e, bundle, select(from_mask, position, from[axis]));
        st(e, bundle + 0x30, select(to_mask, position, to[axis]));
    }
    let ascending = e
        .call(COUNT_ASCENDING, &args![s + 0x370, b + 0x60, s + 0x350])
        .eax;
    let (bundle_one, bundle_two) = (s + 0x220, s + 0x3e0);
    let (head, head_skip, tail, tail_skip) = if ascending != 0 {
        (bundle_one, first, bundle_two, second)
    } else {
        (bundle_two, second, bundle_one, first)
    };
    e.mem.set_u32(s + 0x14, tail_skip);
    let this = w.machine;
    e.call(
        QUERY_RAY_ON_TREE,
        &args![this, ctx.state, pc.wrapping_add(head_skip), head, ctx.chunk],
    );
    // The first walk left the nearest hit fractions at `this + 0xb0`; the second bundle's end
    // points are pulled back to the hit position.
    let hit = ld(e, this + 0xb0);
    let ones = ld(e, ONES);
    let remaining = subps(ones, hit);
    let info = e.mem.u32(this + 0x10);
    let origin = ld(e, info + 0x10);
    let offset = ld(e, ctx.state);
    let scale = ld(e, ctx.state + 0x20);
    let before_one = cmpltps(hit, ones);
    for axis in 0..3u32 {
        let lane = |v: Lanes| splat(v[axis as usize]);
        let start = ld(e, this + 0x70 + 0x10 * axis);
        let end = ld(e, this + 0x40 + 0x10 * axis);
        let blended = addps(mulps(start, remaining), mulps(end, hit));
        let moved = subps(
            mulps(subps(blended, lane(origin)), lane(scale)),
            lane(offset),
        );
        let slot = tail + 0x30 + 0x10 * axis;
        let old = ld(e, slot);
        st(e, slot, select(before_one, moved, old));
    }
    let axis = e.mem.u32(s + 0x20);
    if (axis as i32) < 3 {
        let ascending_high = cmpltps(ld(e, s + 0x330), ld(e, s + 0x3b0));
        let end_new = ld(e, tail + (axis.wrapping_add(3) << 4));
        let above_high = cmpltps(ld(e, s + 0x60), end_new);
        let below_low = andps(cmpltps(end_new, ld(e, s + 0x40)), ascending_high);
        let beyond = orps(andps(andnps(ascending_high, all), above_high), below_low);
        let not_above = andnps(ld(e, s + 0x2d0), all);
        let not_below = andnps(ld(e, s + 0x2f0), all);
        let drop = andps(andps(beyond, andps(not_above, not_below)), before_one);
        let old = ld(e, tail + 0x60);
        st(e, tail + 0x60, andps(andnps(drop, all), old));
    }
    if movmskps(ld(e, tail + 0x60)) == 0 {
        return None;
    }
    let next = pc.wrapping_add(tail_skip);
    e.call(COPY_BUNDLE, &args![b, tail]);
    Some(next)
}

/// Commands 41..43: the two plane offsets as 24-bit values at `edi + 1` and `edi + 4`,
/// scaled by the machine's inverse scale and the state's scale, less the state's offset along
/// `axis`. Returns `(low, high)`.
fn wide_planes(e: &mut Engine, w: &Walk, state: u32, edi: u32, axis: u32) -> (Lanes, Lanes) {
    let byte = |e: &Engine, n: u32| u32::from(e.mem.u8(edi.wrapping_add(n)));
    let low24 = (((byte(e, 1) << 8).wrapping_add(byte(e, 2))) << 8).wrapping_add(byte(e, 3));
    let high24 = (((byte(e, 4) << 8).wrapping_add(byte(e, 5))) << 8).wrapping_add(byte(e, 6));
    let inverse = ld(e, w.machine + 0x20);
    let scale = ld(e, state + 0x20);
    let low = mulps(mulps(splat(int_float_bits(low24)), inverse), scale);
    let high = mulps(mulps(splat(int_float_bits(high24)), inverse), scale);
    let offsets = ld(e, state);
    let offset = match axis {
        0 => shufps(offsets, 0x00),
        1 => shufps(offsets, 0x55),
        2 => shufps(offsets, 0xaa),
        3 => shufps(offsets, 0xff),
        _ => ld(e, w.s + 0x130),
    };
    st(e, w.s + 0x130, offset);
    (subps(low, offset), subps(high, offset))
}

/// Commands 38..43: drops the rays that miss the slab `low..high` along `axis` and clips the
/// others to it (clipping fractions come from `00dd2fd0`). Returns the next code pointer, or
/// `None` when no ray is left.
fn double_cut(
    e: &mut Engine,
    w: &Walk,
    pc: u32,
    axis: u32,
    low: Lanes,
    high: Lanes,
) -> Option<u32> {
    let (s, b) = (w.s, w.block);
    let all = ld(e, ALL_BITS);
    let from = ld(e, b + (axis << 4));
    let to = ld(e, b + ((axis + 3) << 4));
    let descending = cmpleps(to, from);
    st(e, s + 0xc0, descending);
    let ascending = cmpltps(from, to);
    let outside_a = orps(cmpltps(high, to), cmpltps(from, low));
    let outside_b = andps(orps(cmpltps(to, low), cmpltps(high, from)), ascending);
    let miss = orps(andnps(ascending, outside_a), outside_b);
    st(e, s + 0x40, low);
    let mask = andps(andnps(miss, all), ld(e, b + 0x60));
    st(e, s + 0x3c0, ascending);
    st(e, b + 0x60, mask);
    if movmskps(mask) == 0 {
        return None;
    }
    // The clipping works from copies of the unclipped end points.
    for k in 0..12 {
        let word = e.mem.u32(b + 4 * k);
        e.mem.set_u32(s + 0x300 + 4 * k, word);
        let word = e.mem.u32(b + 0x30 + 4 * k);
        e.mem.set_u32(s + 0x290 + 4 * k, word);
    }
    let zero = [0u32; 4];
    // Clip against `high`.
    let to_rel = subps(to, high);
    st(e, s + 0x360, to_rel);
    let from_rel = subps(from, high);
    let crossing = cmpltps(mulps(to_rel, from_rel), zero);
    st(e, s + 0x3a0, from_rel);
    st(e, s + 0xd0, zero);
    e.call(
        RECIPROCAL_PAIR,
        &args![s + 0x360, s + 0x1f0, s + 0x3a0, s + 0x1d0],
    );
    let result = ld(e, s + 0x1d0);
    let out = ld(e, s + 0x1f0);
    let from_mask = andps(crossing, ld(e, s + 0xc0));
    let to_mask = andps(crossing, ascending);
    clip_block(e, w, result, out, from_mask, to_mask);
    // Clip against `low`.
    let low_now = ld(e, s + 0x40);
    let to_rel = subps(to, low_now);
    st(e, s + 0x380, to_rel);
    let from_rel = subps(from, low_now);
    st(e, s + 0x3d0, from_rel);
    let crossing = cmpltps(mulps(to_rel, from_rel), ld(e, s + 0xd0));
    e.call(
        RECIPROCAL_PAIR,
        &args![s + 0x380, s + 0x1b0, s + 0x3d0, s + 0x340],
    );
    let result = ld(e, s + 0x340);
    let out = ld(e, s + 0x1b0);
    let from_mask = andps(crossing, ld(e, s + 0x3c0));
    let to_mask = andps(crossing, ld(e, s + 0xc0));
    clip_block(e, w, result, out, from_mask, to_mask);
    Some(pc)
}

/// Replaces the block's from points where `from_mask` is set and its to points where `to_mask`
/// is set by `result * to + out * from` (taken from the copies of the unclipped points).
fn clip_block(
    e: &mut Engine,
    w: &Walk,
    result: Lanes,
    out: Lanes,
    from_mask: Lanes,
    to_mask: Lanes,
) {
    let (s, b) = (w.s, w.block);
    for axis in 0..3u32 {
        let copy_from = ld(e, s + 0x300 + 0x10 * axis);
        let copy_to = ld(e, s + 0x290 + 0x10 * axis);
        let position = addps(mulps(copy_to, result), mulps(copy_from, out));
        let old_from = ld(e, b + 0x10 * axis);
        st(e, b + 0x10 * axis, select(from_mask, position, old_from));
        let old_to = ld(e, b + 0x30 + 0x10 * axis);
        st(e, b + 0x30 + 0x10 * axis, select(to_mask, position, old_to));
    }
}

/// Commands 48..83: the terminal. The shape key is the chunk bits (when chunked) and the state's
/// terminal offset plus `value`; the shape is made through the container's slot `+0x14`, the
/// ray bundle's filter (when it has one) may reject it, and the shape's `+0x28` slot casts the
/// bundle at it. The ray outputs count the shape and record the key for each ray that hit.
fn terminal(e: &mut Engine, w: &Walk, ctx: Ctx, value: u32) {
    let (s, b, this) = (w.s, w.block, w.machine);
    let chunk_bits = e.mem.u32(this + 0xd0) & (ctx.chunk << 8);
    let key = chunk_bits | e.mem.u32(ctx.state + 0x30).wrapping_add(value);
    let container = e.mem.u32(this + 0xcc);
    let shape = e.vcall(container, 0x14, &args![key, s + 0x450]).eax;
    let rays = e.mem.u32(this + 0x30);
    let filter = e.mem.u32(rays + 0x64);
    if filter != 0 {
        e.mem.set_u32(s + 0x2b0, 0);
        e.mem.set_u32(s + 0x2b4, 0);
        let word = e.mem.u32(rays + 0x60);
        e.mem.set_u32(s + 0x2b0, word);
        let verdict = e
            .vcall(
                filter,
                0,
                &args![s + 0x5f, s + 0x290, shape, container, key],
            )
            .eax;
        if e.mem.u8(verdict) == 0 {
            return;
        }
    }
    let out = e.mem.u32(this + 0xc0);
    for lane in 0..4 {
        add_word(e, out + 0x40 + 0x50 * lane, 1);
    }
    e.vcall(shape, 0x28, &args![s + 0xc0, rays, out, b + 0x60]);
    for lane in 0..4u32 {
        let out = e.mem.u32(this + 0xc0);
        let count = out + 0x50 * lane + 0x40;
        let decremented = e.mem.u32(count).wrapping_sub(1);
        e.mem.set_u32(count, decremented);
        let hit_bits = movmskps(ld(e, s + 0xc0));
        if e.mem.u32(RAY_LANE_MASKS + 4 * lane) & hit_bits != 0 {
            let fraction = e.mem.u32(out + 0x50 * lane + 0x10);
            e.mem
                .set_u32(this + 0xb0 + 4 * lane, x87_quiet_bits(fraction));
            let at = e.mem.u32(count);
            e.mem.set_u32(out + 0x50 * lane + 0x20 + 4 * at, key);
        }
    }
    let hits = orps(ld(e, s + 0xc0), ld(e, this + 0xa0));
    st(e, this + 0xa0, hits);
}

/// Unknown command: reports the corrupted MOPP data through the Havok error object (a debug
/// break when the handler asks for one, which is modelled as a panic).
fn bad_command(e: &mut Engine, s: u32) {
    e.call(STREAM_CONSTRUCT, &args![s + 0xb4, s + 0x450, 0x200u32]);
    e.call(STREAM_WRITE, &args![s + 0xb4, BAD_COMMAND_TEXT]);
    let reporter = e.mem.u32(ERROR_OBJECT);
    let asked = e
        .vcall(
            reporter,
            0xc,
            &args![
                3u32,
                BAD_COMMAND_ID,
                s + 0x450,
                SOURCE_FILE_TEXT,
                BAD_COMMAND_LINE
            ],
        )
        .eax;
    if asked != 0 {
        panic!("INT3 in queryRayOnTree: unknown MOPP command (00dd4580)");
    }
    e.call(STREAM_DESTRUCT, &args![s + 0xb4]);
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_2fd0, fn_00dd2fd0(u32, u32, u32, u32)),
        entry!(0x00dd_3120, fn_00dd3120(u32, u32, u32) -> u32),
        entry!(
            0x00dd_3210,
            query_ray_on_tree(Ptr<HkpMoppRayBundleVirtualMachine>, u32, u32, u32, u32)
        ),
        entry!(
            0x00dd_4870,
            query_ray_bundle(
                Ptr<HkpMoppRayBundleVirtualMachine>,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            ) -> Ptr
        ),
        entry!(0x00dd_4a50, fn_00dd4a50(u32, u32)),
        entry!(0x00dd_4a90, fn_00dd4a90(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const CONTAINER_DOUBLE: u32 = 0x0900_0001;
    const SHAPE_DOUBLE: u32 = 0x0900_0002;
    const FILTER_DOUBLE: u32 = 0x0900_0003;
    const REPORT_DOUBLE: u32 = 0x0900_0004;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        for (address, value) in [
            (ONE_FLOAT, 1.0),
            (FLOAT_255, 255.0),
            (FLOAT_510, 510.0),
            (FLOAT_2, 2.0),
            (FLOAT_4, 4.0),
            (FLOAT_8, 8.0),
            (FLOAT_16, 16.0),
        ] {
            e.mem.set_f32(address, value);
        }
        e.mem.set_f64(SCALE_DOUBLE, 1.0 / 65536.0);
        st(&mut e, ALL_BITS, splat(u32::MAX));
        st(&mut e, ONES, splat(1.0f32.to_bits()));
        st(&mut e, SIGN_BITS, splat(0x8000_0000));
        for (lane, bit) in [1u32, 2, 4, 8].into_iter().enumerate() {
            e.mem.set_u32(RAY_LANE_MASKS + 4 * lane as u32, bit);
        }
        e.mem.set_u8(INIT_FLAG, 1);
        e
    }

    fn floats(e: &Engine, address: u32) -> [f32; 4] {
        std::array::from_fn(|i| e.mem.f32(address + 4 * i as u32))
    }

    fn put(e: &mut Engine, address: u32, v: [f32; 4]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(address + 4 * i as u32, *x);
        }
    }

    fn close(a: [f32; 4], b: [f32; 4]) {
        for i in 0..4 {
            assert!((a[i] - b[i]).abs() < 1e-2, "{a:?} != {b:?}");
        }
    }

    /// A machine with a container double that records the keys it is asked for, a shape double
    /// that reports the hits in `hits` (a lane bit set) and counts its casts.
    struct Rig {
        e: Engine,
        this: u32,
        state: u32,
        block: u32,
        code: u32,
        out: u32,
        keys: Rc<RefCell<Vec<u32>>>,
        casts: Rc<Cell<u32>>,
        hits: Rc<Cell<u32>>,
    }

    fn rig(code: &[u8]) -> Rig {
        let mut e = engine();
        let this = e.mem.alloc(0xe0);
        let info = e.mem.alloc(0x40);
        let rays = e.mem.alloc(0x70);
        let out = e.mem.alloc(0x140);
        let state = e.mem.alloc(0x40);
        let block = e.mem.alloc(0x70);
        let data = e.mem.alloc(0x400);
        e.mem.write(data, &[0; 0x400]);
        e.mem.write(data, code);
        e.mem.set_u32(this + 0x10, info);
        st(&mut e, this + 0x20, splat(1.0f32.to_bits()));
        e.mem.set_u32(this + 0x30, rays);
        e.mem.set_u32(this + 0xc0, out);
        e.mem.set_u32(info + 0x20, data);
        st(&mut e, state + 0x20, splat(1.0f32.to_bits()));
        st(&mut e, block + 0x60, splat(u32::MAX));
        let container = e.mem.alloc(0x10);
        let container_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(container, container_vtable);
        e.mem.set_u32(container_vtable + 0x14, CONTAINER_DOUBLE);
        e.mem.set_u32(this + 0xcc, container);
        let shape = e.mem.alloc(0x10);
        let shape_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(shape, shape_vtable);
        e.mem.set_u32(shape_vtable + 0x28, SHAPE_DOUBLE);
        let keys = Rc::new(RefCell::new(Vec::new()));
        let casts = Rc::new(Cell::new(0));
        let hits = Rc::new(Cell::new(0));
        let key_log = Rc::clone(&keys);
        e.register_double(CONTAINER_DOUBLE, move |_, a| {
            key_log.borrow_mut().push(a[1]);
            Ret {
                eax: shape,
                ..Ret::default()
            }
        });
        let cast_count = Rc::clone(&casts);
        let hit_lanes = Rc::clone(&hits);
        e.register_double(SHAPE_DOUBLE, move |e, a| {
            cast_count.set(cast_count.get() + 1);
            for lane in 0..4u32 {
                let hit = if hit_lanes.get() & (1 << lane) != 0 {
                    u32::MAX
                } else {
                    0
                };
                e.mem.set_u32(a[1] + 4 * lane, hit);
            }
            Ret::default()
        });
        Rig {
            e,
            this,
            state,
            block,
            code: data,
            out,
            keys,
            casts,
            hits,
        }
    }

    impl Rig {
        fn walk(&mut self) {
            let this = Ptr::new(self.this);
            query_ray_on_tree(&mut self.e, this, self.state, self.code, self.block, 0);
        }

        fn set_block(&mut self, from: [[f32; 4]; 3], to: [[f32; 4]; 3]) {
            for axis in 0..3 {
                put(&mut self.e, self.block + 0x10 * axis as u32, from[axis]);
                put(
                    &mut self.e,
                    self.block + 0x30 + 0x10 * axis as u32,
                    to[axis],
                );
            }
        }
    }

    #[test]
    fn reciprocal_pair_gives_the_fractions() {
        let mut e = engine();
        let (a, b, out, result) = (0x0100_1000, 0x0100_1010, 0x0100_1020, 0x0100_1030);
        put(&mut e, a, [1.0, 2.0, 3.0, 4.0]);
        put(&mut e, b, [3.0, 6.0, 9.0, 12.0]);
        e.call(0x00dd_2fd0, &args![a, out, b, result]);
        close(floats(&e, result), [1.5; 4]);
        close(floats(&e, out), [-0.5; 4]);
    }

    #[test]
    fn ascending_count_needs_half_of_the_active_rays() {
        let mut e = engine();
        let (starts, ends, mask) = (0x0100_1000, 0x0100_1010, 0x0100_1020);
        put(&mut e, ends, [10.0; 4]);
        st(&mut e, mask, splat(u32::MAX));
        for (values, expected) in [
            ([1.0, 2.0, 3.0, 4.0], 0xf),
            ([20.0, 2.0, 30.0, 40.0], 0),
            ([20.0, 2.0, 3.0, 40.0], 0xf),
        ] {
            put(&mut e, starts, values);
            assert_eq!(
                e.call(0x00dd_3120, &args![starts, mask, ends]).eax,
                expected
            );
        }
        // Inactive rays do not count.
        put(&mut e, starts, [20.0, 2.0, 30.0, 40.0]);
        st(&mut e, mask, [0, u32::MAX, 0, 0]);
        assert_eq!(e.call(0x00dd_3120, &args![starts, mask, ends]).eax, 0xf);
    }

    #[test]
    fn copies_of_the_state_and_the_block() {
        let mut e = engine();
        let (source, dest) = (0x0100_2000, 0x0100_2100);
        for i in 0..0x70 {
            e.mem.set_u8(source + i, i as u8 + 1);
        }
        e.call(0x00dd_4a50, &args![dest, source]);
        for i in 0..0x38 {
            // The words at +0x14..0x20 are not part of the state.
            let expected = if (0x14..0x20).contains(&i) {
                0
            } else {
                i as u8 + 1
            };
            assert_eq!(e.mem.u8(dest + i), expected, "state byte {i:#x}");
        }
        let block_dest = 0x0100_2200;
        e.call(0x00dd_4a90, &args![block_dest, source]);
        for i in 0..0x70 {
            assert_eq!(e.mem.u8(block_dest + i), i as u8 + 1, "block byte {i:#x}");
        }
    }

    #[test]
    fn ray_bundle_sets_up_the_scaled_block_and_walks() {
        let mut e = engine();
        let this = e.mem.alloc(0xe0);
        let (result, container, info, rays, output, mask) = (
            e.mem.alloc(0x10),
            e.mem.alloc(0x10),
            e.mem.alloc(0x40),
            e.mem.alloc(0x70),
            e.mem.alloc(0x140),
            e.mem.alloc(0x10),
        );
        let code = e.mem.alloc(0x10);
        e.mem.set_u8(code, 0x0d);
        e.mem.set_u32(info + 0x20, code);
        put(&mut e, info + 0x10, [1.0, 2.0, 3.0, 2.0]);
        for vector in 0..6 {
            put(&mut e, rays + 0x10 * vector, [10.0 + vector as f32; 4]);
        }
        st(&mut e, mask, [u32::MAX, 0, u32::MAX, 0]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00dd_3210, move |e, a| {
            let block = a[3];
            let words: Vec<u32> = (0..28).map(|i| e.mem.u32(block + 4 * i)).collect();
            let state: Vec<u32> = (0..14).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push((a.to_vec(), words, state));
            // The walk found a hit in lane 1.
            let this = a[0];
            e.mem.set_u32(this + 0xa4, u32::MAX);
            Ret::default()
        });
        let returned = query_ray_bundle(
            &mut e,
            Ptr::new(this),
            Ptr::new(result),
            Ptr::new(container),
            Ptr::new(info),
            Ptr::new(rays),
            Ptr::new(output),
            Ptr::new(mask),
        );
        assert_eq!(returned.addr(), result);
        let seen = seen.borrow();
        let (args, block, state) = &seen[0];
        assert_eq!((args[0], args[2], args[4]), (this, code, 0));
        let step = 2.0f32 / 65536.0;
        let expect_from_x = (10.0 - 1.0) * step;
        assert!((f32::from_bits(block[0]) - expect_from_x).abs() < 1e-9);
        let expect_to_z = (15.0 - 3.0) * step;
        assert!((f32::from_bits(block[20]) - expect_to_z).abs() < 1e-9);
        assert_eq!(&block[24..28], &[u32::MAX, 0, u32::MAX, 0]);
        assert_eq!(f32::from_bits(state[8]), step);
        assert_eq!(e.mem.u32(this + 0xd0), 0xffff_ffff);
        assert_eq!(e.mem.f32(this + 0x20), 0.5);
        assert_eq!(e.mem.f32(this + 0xb0), 1.0);
        assert_eq!(e.mem.u32(result + 4), u32::MAX);
        assert_eq!(e.mem.u32(this + 0xcc), container);
    }

    #[test]
    fn init_guard_stops_the_walk_when_the_flag_stays_clear() {
        let mut r = rig(&[0x30, 0]);
        r.e.mem.set_u8(INIT_FLAG, 0);
        let asked = Rc::new(Cell::new(0));
        let seen = Rc::clone(&asked);
        r.e.register_double(INIT_ONCE, move |_, a| {
            seen.set(a[0]);
            Ret::default()
        });
        r.walk();
        assert_eq!(asked.get(), INIT_FLAG);
        assert!(r.keys.borrow().is_empty());
        // When the routine sets the flag, the walk goes on.
        r.e.register_double(INIT_ONCE, |e, _| {
            e.mem.set_u8(INIT_FLAG, 1);
            Ret::default()
        });
        r.walk();
        assert_eq!(*r.keys.borrow(), vec![0]);
    }

    #[test]
    fn jumps_move_the_code_pointer() {
        // 8-bit jump over a return, 16-bit jump over a return, then a terminal.
        let mut r = rig(&[5, 1, 0x00, 6, 0, 1, 0x00, 0x35]);
        r.walk();
        assert_eq!(*r.keys.borrow(), vec![5]);
        // A 24-bit jump and the skip of five bytes.
        let mut r = rig(&[7, 0, 0, 5, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0x31]);
        r.walk();
        assert_eq!(*r.keys.borrow(), vec![1]);
    }

    #[test]
    fn terminal_records_the_key_for_the_rays_that_hit() {
        let mut r = rig(&[0x35]);
        e_set_terminal_offset(&mut r, 0x100);
        r.hits.set(0b0101);
        r.e.mem.set_f32(r.out + 0x10, 0.25);
        r.e.mem.set_f32(r.out + 2 * 0x50 + 0x10, 0.75);
        r.walk();
        assert_eq!(*r.keys.borrow(), vec![0x105]);
        assert_eq!(r.casts.get(), 1);
        // Lane 0 and 2 hit: their fractions go to the machine, their keys to the output.
        assert_eq!(r.e.mem.f32(r.this + 0xb0), 0.25);
        assert_eq!(r.e.mem.f32(r.this + 0xb8), 0.75);
        assert_eq!(r.e.mem.u32(r.out + 0x20), 0x105);
        assert_eq!(r.e.mem.u32(r.out + 2 * 0x50 + 0x20), 0x105);
        // The counts are back to zero and the hit mask has the hit lanes.
        for lane in 0..4 {
            assert_eq!(r.e.mem.u32(r.out + 0x50 * lane + 0x40), 0);
        }
        assert_eq!(ld(&r.e, r.this + 0xa0), [u32::MAX, 0, u32::MAX, 0]);
    }

    fn e_set_terminal_offset(r: &mut Rig, value: u32) {
        r.e.mem.set_u32(r.state + 0x30, value);
    }

    #[test]
    fn terminal_values_of_each_width() {
        for (code, key) in [
            (vec![80u8, 0x12], 0x12),
            (vec![81, 0x12, 0x34], 0x1234),
            (vec![82, 0x12, 0x34, 0x56], 0x12_3456),
            (vec![83, 0x12, 0x34, 0x56, 0x78], 0x1234_5678),
        ] {
            let mut r = rig(&code);
            r.walk();
            assert_eq!(*r.keys.borrow(), vec![key]);
        }
    }

    #[test]
    fn the_filter_can_reject_a_shape() {
        for (verdict, casts) in [(0u8, 0), (1, 1)] {
            let mut r = rig(&[0x30]);
            let rays = r.e.mem.u32(r.this + 0x30);
            let filter = r.e.mem.alloc(0x10);
            let filter_vtable = r.e.mem.alloc(0x10);
            let answer = r.e.mem.alloc(4);
            r.e.mem.set_u8(answer, verdict);
            r.e.mem.set_u32(filter, filter_vtable);
            r.e.mem.set_u32(filter_vtable, FILTER_DOUBLE);
            r.e.mem.set_u32(rays + 0x64, filter);
            r.e.mem.set_u32(rays + 0x60, 0xabcd);
            let seen = Rc::new(RefCell::new(Vec::new()));
            let log = Rc::clone(&seen);
            r.e.register_double(FILTER_DOUBLE, move |_, a| {
                log.borrow_mut().push((a[0], a[5]));
                Ret {
                    eax: answer,
                    ..Ret::default()
                }
            });
            r.walk();
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            assert_eq!(seen[0], (filter, 0));
            assert_eq!(r.casts.get(), casts);
        }
    }

    #[test]
    fn scale_command_scales_the_block_about_the_offset() {
        let mut r = rig(&[2, 1, 2, 3, 0]);
        r.set_block(
            [[1.0, 2.0, 3.0, 4.0], [2.0; 4], [3.0; 4]],
            [[5.0, 6.0, 7.0, 8.0], [4.0; 4], [9.0; 4]],
        );
        r.walk();
        let b = r.block;
        close(floats(&r.e, b), [0.0, 4.0, 8.0, 12.0]);
        close(floats(&r.e, b + 0x10), [0.0; 4]);
        close(floats(&r.e, b + 0x20), [0.0; 4]);
        close(floats(&r.e, b + 0x30), [16.0, 20.0, 24.0, 28.0]);
        close(floats(&r.e, b + 0x40), [8.0; 4]);
        close(floats(&r.e, b + 0x50), [24.0; 4]);
    }

    #[test]
    fn terminal_offset_commands_change_the_key() {
        // Add 8 bits, add 16 bits, then a terminal.
        let mut r = rig(&[9, 5, 10, 1, 0, 0x30]);
        e_set_terminal_offset(&mut r, 0x1000);
        r.walk();
        assert_eq!(*r.keys.borrow(), vec![0x1000 + 5 + 0x100]);
        // Set 32 bits.
        let mut r = rig(&[11, 0, 1, 2, 3, 0x31]);
        r.walk();
        assert_eq!(*r.keys.borrow(), vec![0x0001_0203 + 1]);
        // A property word keeps the terminal offset.
        let mut r = rig(&[96, 9, 100, 1, 2, 0x30]);
        e_set_terminal_offset(&mut r, 0x40);
        r.walk();
        assert_eq!(*r.keys.borrow(), vec![0x40]);
    }

    #[test]
    fn chunk_jump_sets_the_chunk_bits_of_the_key() {
        let mut code = vec![12u8, 0, 1];
        code.resize(0x200, 0);
        code.push(0x32);
        let mut r = rig(&code);
        r.e.mem.set_u32(r.this + 0xd0, u32::MAX);
        r.walk();
        assert_eq!(*r.keys.borrow(), vec![0x102]);
    }

    #[test]
    fn plane_cut_continues_at_the_side_the_rays_are_on() {
        // Axis cut: high plane 10, low plane 100, skip 2 for rays above the high plane.
        let code = [16u8, 10, 100, 2, 0x30, 0, 0];
        for (x, reaches_terminal) in [(50.0, true), (200.0, false)] {
            let mut r = rig(&code);
            r.set_block([[x; 4], [0.0; 4], [0.0; 4]], [[x; 4], [0.0; 4], [0.0; 4]]);
            r.walk();
            // Below the low plane the first child (code + 4) is walked; above the high plane
            // the rays skip two bytes and return.
            assert_eq!(
                r.keys.borrow().len(),
                usize::from(reaches_terminal),
                "x = {x}"
            );
        }
    }

    #[test]
    fn diagonal_cut_uses_the_summed_axes() {
        // 2*v planes: high 10, low 60; the projection is z + y = 20, below the low plane.
        let mut r = rig(&[19, 5, 30, 0, 0x30]);
        r.set_block(
            [[0.0; 4], [10.0; 4], [10.0; 4]],
            [[0.0; 4], [10.0; 4], [10.0; 4]],
        );
        r.walk();
        assert_eq!(r.keys.borrow().len(), 1);
        // The same rays above both planes (high 4, low 10) skip three bytes and return.
        let mut r = rig(&[19, 2, 5, 3, 0x30, 0, 0, 0]);
        r.set_block(
            [[0.0; 4], [10.0; 4], [10.0; 4]],
            [[0.0; 4], [10.0; 4], [10.0; 4]],
        );
        r.walk();
        assert!(r.keys.borrow().is_empty());
    }

    #[test]
    fn straddling_rays_are_clipped_and_both_children_walked() {
        // Rays from 5 to 200 along x cross the planes 10 and 100.
        let code = [16u8, 10, 100, 4, 0, 0, 0, 0, 0x30];
        let mut r = rig(&code);
        st(&mut r.e, r.this + 0xb0, splat(1.0f32.to_bits()));
        r.set_block(
            [[5.0; 4], [0.0; 4], [0.0; 4]],
            [[200.0; 4], [0.0; 4], [0.0; 4]],
        );
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        r.e.register_double(0x00dd_3210, move |e, a| {
            let bundle = a[3];
            log.borrow_mut()
                .push((a[1], a[2], floats(e, bundle), floats(e, bundle + 0x30)));
            Ret::default()
        });
        r.walk();
        let seen = seen.borrow();
        // The ascending rays walk the part below the high plane first, at the first offset.
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, r.state);
        assert_eq!(seen[0].1, r.code + 4);
        close(seen[0].2, [5.0; 4]);
        close(seen[0].3, [10.0; 4]);
        // The second part runs from the low plane and continues at the second offset.
        close(floats(&r.e, r.block), [100.0; 4]);
        close(floats(&r.e, r.block + 0x30), [200.0; 4]);
        assert_eq!(*r.keys.borrow(), vec![0]);
    }

    #[test]
    fn slab_commands_clip_the_rays_to_the_slab() {
        // A slab from 40 to 80 along x, as bytes (38) and as 24-bit values (41).
        for code in [vec![38u8, 40, 80, 0x30], vec![41, 0, 0, 40, 0, 0, 80, 0x30]] {
            let mut r = rig(&code);
            r.set_block(
                [[0.0, 50.0, 20.0, 200.0], [0.0; 4], [0.0; 4]],
                [[100.0, 60.0, 30.0, 300.0], [0.0; 4], [0.0; 4]],
            );
            r.walk();
            let b = r.block;
            assert_eq!(ld(&r.e, b + 0x60), [u32::MAX, u32::MAX, 0, 0]);
            let from = floats(&r.e, b);
            let to = floats(&r.e, b + 0x30);
            assert!((from[0] - 40.0).abs() < 0.1 && (to[0] - 80.0).abs() < 0.1);
            close([from[1], to[1], 0.0, 0.0], [50.0, 60.0, 0.0, 0.0]);
            assert_eq!(r.keys.borrow().len(), 1);
        }
        // No ray is left: the walk returns.
        let mut r = rig(&[38, 40, 80, 0x30]);
        r.set_block(
            [[0.0; 4], [0.0; 4], [0.0; 4]],
            [[10.0; 4], [0.0; 4], [0.0; 4]],
        );
        r.walk();
        assert!(r.keys.borrow().is_empty());
    }

    #[test]
    #[should_panic(expected = "unknown MOPP command")]
    fn an_unknown_command_reports_and_breaks() {
        let mut r = rig(&[14]);
        for address in [STREAM_CONSTRUCT, STREAM_WRITE, STREAM_DESTRUCT] {
            r.e.register_double(address, |_, _| Ret::default());
        }
        let reporter = r.e.mem.alloc(0x10);
        let vtable = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(reporter, vtable);
        r.e.mem.set_u32(vtable + 0xc, REPORT_DOUBLE);
        r.e.mem.set_u32(ERROR_OBJECT, reporter);
        r.e.register_double(REPORT_DOUBLE, |_, a| {
            assert_eq!((a[1], a[2]), (3, BAD_COMMAND_ID));
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        r.walk();
    }
}
