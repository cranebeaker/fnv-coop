//! `Havok SDK/run_00e2b070` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions belong to a convex-hull growing routine over two convex
//! shapes (the next unit, `run_00df9840`, drives it); the executable has no
//! names for them and no Xbox PDB entry, so the routines are named by address
//! and the fields below come from this code only.
//!
//! The polytope object (`this` of `00e2d0d0`, `00e2d1b0`, `00e2d440`,
//! `00e2d550`) holds: +0x08 and +0x0c counts, +0x10 the number of faces in
//! use, +0x20 and +0x50 vertex records of 0x40 bytes (the vector at +0, a link
//! at +0x30), +0xde0 a stack of up to 100 recycled face pointers (count at
//! +0x0c) and +0xf70 the faces, 0x50 bytes each: the normal (+0x00), three
//! edge slots of 0x10 bytes (+0x10, +0x20, +0x30), a state word (+0x44) and
//! the plane distance (+0x40). SSE code computes lane by lane in `f32`; x87
//! sums stay unrounded until the `FSTP float`, compares read the status word
//! the way the `TEST AH` pairs do (NaN included).
//!
//! Register conventions: `00e2da40` takes the shape in `ECX`, the matrix in
//! `ESI` and the direction in `EDI` (stack: the second shape and the record,
//! the caller pops them); here those registers are the first three
//! arguments, in that order.

use crate::x87::rsqrt_estimate;

#[allow(unused_imports)]
use crate::prelude::*;

/// Four lanes as raw bits.
type Lanes = [u32; 4];

/// `MOVAPS xmm, [at]`: the 16 bytes at `at` as four words.
fn load_lanes(e: &Engine, at: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.u32(at.wrapping_add(4 * lane as u32)))
}

/// `MOVAPS [at], xmm`.
fn store_lanes(e: &mut Engine, at: u32, v: Lanes) {
    for (lane, word) in v.into_iter().enumerate() {
        e.mem.set_u32(at.wrapping_add(4 * lane as u32), word);
    }
}
use super::run_00e29250::{lane_add, lane_mul, lane_sub};

/// The integer flags the exe's conditional jumps read.
#[derive(Clone, Copy, Default)]
struct Flags {
    zf: bool,
    sf: bool,
    cf: bool,
    of: bool,
    pf: bool,
}

/// The mask of an operand `bits` wide.
fn width_mask(bits: u32) -> u32 {
    if bits == 32 {
        u32::MAX
    } else {
        (1u32 << bits) - 1
    }
}

impl Flags {
    /// `ZF`, `SF` and `PF` of a `bits`-wide result.
    fn result(&mut self, value: u32, bits: u32) {
        let value = value & width_mask(bits);
        self.zf = value == 0;
        self.sf = (value >> (bits - 1)) & 1 == 1;
        self.pf = (value as u8).count_ones() % 2 == 0;
    }

    fn add(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        let mask = width_mask(bits);
        let (a, b) = (a & mask, b & mask);
        let wide = u64::from(a) + u64::from(b);
        let result = (wide as u32) & mask;
        self.cf = wide > u64::from(mask);
        self.of = ((a ^ result) & (b ^ result)) >> (bits - 1) & 1 == 1;
        self.result(result, bits);
        result
    }

    fn sub(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        let mask = width_mask(bits);
        let (a, b) = (a & mask, b & mask);
        let result = a.wrapping_sub(b) & mask;
        self.cf = a < b;
        self.of = ((a ^ b) & (a ^ result)) >> (bits - 1) & 1 == 1;
        self.result(result, bits);
        result
    }

    /// `AND`/`OR`/`XOR`/`TEST`: the carry and overflow flags are cleared.
    fn logic(&mut self, value: u32, bits: u32) -> u32 {
        let value = value & width_mask(bits);
        self.cf = false;
        self.of = false;
        self.result(value, bits);
        value
    }

    fn and(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        self.logic(a & b, bits)
    }

    fn or(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        self.logic(a | b, bits)
    }

    fn xor(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        self.logic(a ^ b, bits)
    }

    /// `INC`: like `ADD 1` but the carry flag stays.
    fn inc(&mut self, a: u32, bits: u32) -> u32 {
        let carry = self.cf;
        let result = self.add(a, 1, bits);
        self.cf = carry;
        result
    }

    /// `DEC`: like `SUB 1` but the carry flag stays.
    fn dec(&mut self, a: u32, bits: u32) -> u32 {
        let carry = self.cf;
        let result = self.sub(a, 1, bits);
        self.cf = carry;
        result
    }

    /// `SHL value, count` (a count of 0 changes nothing).
    fn shift_shl(&mut self, value: u32, count: u32, bits: u32) -> u32 {
        let count = count & 31;
        let value = value & width_mask(bits);
        if count == 0 {
            return value;
        }
        let wide = u64::from(value) << count;
        let result = (wide as u32) & width_mask(bits);
        self.cf = (wide >> bits) & 1 == 1;
        self.of = false;
        self.result(result, bits);
        result
    }

    /// `SAR value, count`.
    fn shift_sar(&mut self, value: u32, count: u32, bits: u32) -> u32 {
        let count = count & 31;
        let value = value & width_mask(bits);
        if count == 0 {
            return value;
        }
        let signed = ((u64::from(value) << (64 - bits)) as i64) >> (64 - bits);
        let result = ((signed >> count) as u32) & width_mask(bits);
        self.cf = (signed >> (count - 1)) & 1 == 1;
        self.of = false;
        self.result(result, bits);
        result
    }
}

/// `SHUFPS dst, src, imm`.
fn shufps(dst: Lanes, src: Lanes, imm: u32) -> Lanes {
    [
        dst[(imm & 3) as usize],
        dst[((imm >> 2) & 3) as usize],
        src[((imm >> 4) & 3) as usize],
        src[((imm >> 6) & 3) as usize],
    ]
}

/// `UNPCKLPS dst, src`.
fn unpcklps(dst: Lanes, src: Lanes) -> Lanes {
    [dst[0], src[0], dst[1], src[1]]
}

/// `MINPS dst, src` per lane: `dst` when it is smaller, else `src` (equal values and NaNs give
/// `src`).
fn minps(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| {
        if f32::from_bits(dst[lane]) < f32::from_bits(src[lane]) {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

/// `MAXPS dst, src` per lane: `dst` when it is larger, else `src`.
fn maxps(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| {
        if f32::from_bits(dst[lane]) > f32::from_bits(src[lane]) {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

/// `CMPLTPS dst, src`: all ones where `dst < src`.
fn cmpltps(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| {
        if f32::from_bits(dst[lane]) < f32::from_bits(src[lane]) {
            u32::MAX
        } else {
            0
        }
    })
}

/// `CMPEQSS` on one lane.
fn lane_cmpeq(a: u32, b: u32) -> u32 {
    if f32::from_bits(a) == f32::from_bits(b) {
        u32::MAX
    } else {
        0
    }
}

fn mulps(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| lane_mul(dst[lane], src[lane]))
}

fn addps(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| lane_add(dst[lane], src[lane]))
}

fn subps(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| lane_sub(dst[lane], src[lane]))
}

fn xorps(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| dst[lane] ^ src[lane])
}

/// `ANDNPS dst, src`: `!dst & src`.
fn andnps(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| !dst[lane] & src[lane])
}

/// `MOVMSKPS`: the four sign bits.
fn movmskps(v: Lanes) -> u32 {
    (0..4).fold(0, |mask, lane| mask | ((v[lane] >> 31) << lane))
}

/// An x87 arithmetic operation; `Sub`/`Div` are `a - b`/`a / b` and the reversed forms `b - a`/`b / a`.
#[derive(Clone, Copy)]
enum Fop {
    Add,
    Sub,
    SubR,
    Mul,
    Div,
    DivR,
}

/// `ST(i)` of the stack (the last element is `ST0`).
fn st_get(st: &[f64], i: usize) -> f64 {
    st[st.len() - 1 - i]
}

fn st_set(st: &mut [f64], i: usize, value: f64) {
    let at = st.len() - 1 - i;
    st[at] = value;
}

/// `ST(dst) = ST(dst) op b`, with the x87 NaN choice, then a pop when `pop` is set.
fn fpu_arith_with(st: &mut Vec<f64>, op: Fop, dst: usize, b: f64, pop: bool) {
    let a = st_get(st, dst);
    let result = match op {
        Fop::Add => x87_add(a, b),
        Fop::Sub => x87_sub(a, b),
        Fop::SubR => x87_sub(b, a),
        Fop::Mul => x87_mul(a, b),
        Fop::Div => x87_div(a, b),
        Fop::DivR => x87_div(b, a),
    };
    st_set(st, dst, result);
    if pop {
        st.pop();
    }
}

/// The register form: the second operand is `ST(src)`.
fn fpu_arith(st: &mut Vec<f64>, op: Fop, dst: usize, src: usize, pop: bool) {
    let b = st_get(st, src);
    fpu_arith_with(st, op, dst, b, pop);
}

/// The status word after `FCOM`: `C0` (0x100) when `a < b`, `C3` (0x4000) when equal, all of
/// `C0`, `C2` (0x400) and `C3` when unordered.
fn fpu_compare(a: f64, b: f64) -> u16 {
    if a.is_nan() || b.is_nan() {
        0x4500
    } else if a < b {
        0x0100
    } else if a == b {
        0x4000
    } else {
        0
    }
}

type Vec4 = [f32; 4];

/// Float constants: 0.0, 3.0 and 0.5 (the `RSQRTSS` refinement), the largest
/// value (`0x7f7fffee`) and the most negative one (`0xff7fffee`).
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const MAX_FLOAT: u32 = 0x0104_da48;
const MIN_FLOAT: u32 = 0x0104_e148;
/// Four sign-bit masks (`0x80000000`).
const SIGN_MASK: u32 = 0x010c_44d0;
/// `double` constants: -1e-10 (as a float widened), 0.9 and 0.99, and 0.5.
const NEG_TINY: u32 = 0x010e_b190;
const NINE_TENTHS: u32 = 0x0106_b9e8;
const SHRINK: u32 = 0x0106_d508;
const HALF_DOUBLE: u32 = 0x0101_1588;
/// The `float` 0.001 the shrinking step of `00e2db70` stops at.
const SMALL_STEP: u32 = 0x0101_7d00;
/// The table of three direction vectors `00e2db70` starts from.
const START_DIRECTIONS: u32 = 0x0126_8370;
/// The table `00e2d440` builds the first four faces from (twelve entries of
/// three words).
const FACE_TABLE: u32 = 0x010e_d0f0;

/// Callees outside this unit, and the functions of this unit that call each
/// other.
const RESET_STATES: u32 = 0x00e6_7fd0;
const FACE_SIDE_TEST: u32 = 0x00e6_3af0;
const NORMALIZE_FACE: u32 = 0x00e2_cfe0;
const SUPPORT_POINT: u32 = 0x00e2_da40;
const WALK_FACES: u32 = 0x00e2_d0d0;

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

fn zip_with(a: Vec4, b: Vec4, f: impl Fn(f32, f32) -> f32) -> Vec4 {
    std::array::from_fn(|lane| f(a[lane], b[lane]))
}

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
fn sum3(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

/// `SHUFPS v, v, 0xc9`: the lanes y, z, x, w.
fn yzx(v: Vec4) -> Vec4 {
    [v[1], v[2], v[0], v[3]]
}

/// `SHUFPS v, v, 0xd2`: the lanes z, x, y, w.
fn zxy(v: Vec4) -> Vec4 {
    [v[2], v[0], v[1], v[3]]
}

/// `first.yzx * second.zxy - first.zxy * second.yzx`, the cross product of
/// the SSE sequence (`MULPS`, `MULPS`, `SUBPS`), lane by lane.
fn cross(first: Vec4, second: Vec4) -> Vec4 {
    let plus = zip_with(yzx(first), zxy(second), |a, b| a * b);
    let minus = zip_with(zxy(first), yzx(second), |a, b| a * b);
    zip_with(plus, minus, |a, b| a - b)
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero
/// guard: `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == 0`.
fn inverse_length(e: &Engine, sum: f32) -> f32 {
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// `vector` with every lane's sign bit flipped (`XORPS` with the sign mask).
fn negated(e: &Engine, vector: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        let mask = e.mem.u32(SIGN_MASK + 4 * lane as u32);
        f32::from_bits(vector[lane].to_bits() ^ mask)
    })
}

/// `FLD float; FCHS; FSTP float`: the negation of a value that went through
/// the x87 stack (a signalling NaN comes back quiet).
fn x87_negated(value: f32) -> f32 {
    f32::from_bits(x87_quiet_bits(value.to_bits()) ^ 0x8000_0000)
}

// Translated from 00e2cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets up the plane of a face (`this` in `ECX`, a face of the polytope):
/// the normal is `(v0 - v1) x (v1 - v2)` of the vectors the three slot
/// pointers at +0x10, +0x20 and +0x30 point to (the SSE cross product, lane
/// by lane), normalised with the refined `RSQRTSS` (zero stays zero) and
/// stored at +0x00; the plane distance, the sum `z + (y + x)` of the unit
/// normal times `v0`, goes to +0x40.
pub fn fn_00e2cfe0(e: &mut Engine, face: Ptr) {
    let v1 = load_vec(e, e.mem.u32(face.addr() + 0x20));
    let v0_address = e.mem.u32(face.addr() + 0x10);
    let v0 = load_vec(e, v0_address);
    let v2 = load_vec(e, e.mem.u32(face.addr() + 0x30));
    let first = zip_with(v0, v1, |a, b| a - b);
    let second = zip_with(v1, v2, |a, b| a - b);
    let normal = cross(first, second);
    let sum = sum3(zip_with(normal, normal, |a, b| a * b));
    let scale = inverse_length(e, sum);
    let unit: Vec4 = normal.map(|lane| scale * lane);
    store_vec(e, face.addr(), unit);
    let v0_again = load_vec(e, v0_address);
    let distance = sum3(zip_with(unit, v0_again, |a, b| a * b));
    e.mem.set_f32(face.addr() + 0x40, x87_quiet(distance));
}

// Translated from 00e2d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the faces around `face` (`this` in `ECX` is the polytope): for each
/// of its three edge slots, whose pointer at +0x18, +0x28, +0x38 leads to the
/// record of the face across it (`record + 0xc`), a neighbour with state 0
/// is classified by `00e63af0` (`point_test` and the polytope's first two
/// words are its arguments) and the state saved: a result of 1 recurses into
/// that neighbour (a result of 2 from the recursion ends this walk), a result
/// of 4 ends the walk with 2; a neighbour in state 2 (just classified or from
/// before) is linked: the record's first word points to a structure whose
/// +0x30 gets the record, `*count` goes up by one and `*last` is set to the
/// record. Afterwards the face's plane distance (+0x40) is set to the largest
/// float and, while the stack of processed faces (+0xde0, count at +0x0c)
/// has fewer than 100 entries, the face is pushed on it. Returns the walk's
/// result, 2 when the stack is full.
pub fn fn_00e2d0d0(
    e: &mut Engine,
    this: Ptr,
    face: Ptr,
    point_test: u32,
    count: Ptr,
    last: Ptr,
) -> i32 {
    let mut result = 0i32;
    let mut slot_pointer = face.addr() + 0x18;
    let mut index = 0;
    loop {
        let record = e.mem.u32(slot_pointer);
        let neighbour = e.mem.u32(record + 0xc);
        let mut state = e.mem.u32(neighbour + 0x44);
        if state == 0 {
            state = e
                .call(
                    FACE_SIDE_TEST,
                    &args![neighbour, point_test, this, this.addr() + 4],
                )
                .u32();
            e.mem.set_u32(neighbour + 0x44, state);
            if state == 1 {
                result = e
                    .call(WALK_FACES, &args![this, neighbour, point_test, count, last])
                    .u32() as i32;
                if result == 2 {
                    break;
                }
            } else if state == 4 {
                result = 2;
                break;
            }
        }
        if state == 2 {
            let target = e.mem.u32(record);
            e.mem.set_u32(target + 0x30, record);
            let counted = e.mem.u32(count.addr());
            e.mem.set_u32(count.addr(), counted.wrapping_add(1));
            e.mem.set_u32(last.addr(), record);
        }
        slot_pointer = slot_pointer.wrapping_add(0x10);
        index += 1;
        if index >= 3 {
            break;
        }
    }
    // The plane distance is reset to the largest float (`FLD`/`FSTP`).
    let largest = x87_quiet(e.global::<f32>(MAX_FLOAT));
    e.mem.set_f32(face.addr() + 0x40, largest);
    let depth = e.mem.i32(this.addr() + 0xc);
    if depth >= 100 {
        return 2;
    }
    e.mem
        .set_u32(this.addr() + 0xde0 + 4 * depth as u32, face.addr());
    e.mem.set_i32(this.addr() + 0xc, depth + 1);
    result
}

// Translated from 00e2d1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the faces that close the hole left when the faces of a visible
/// region were removed (`this` in `ECX` is the polytope): `count` faces are
/// taken, first from the stack of recycled faces (+0xde0, count at +0x0c),
/// then new ones from the end of the face array (+0xf70 + 0x50 * the number
/// in use at +0x10, which grows). The edges of the hole form a linked list
/// starting at `edge_list`: an edge record holds its first vertex pointer
/// (+0x00) and a slot (+0x04) leading to the pointer of its second vertex,
/// whose record's +0x30 gives the next edge (and is cleared). Face `i`,
/// built on the edge `i`, joins that edge's two vertices and `point`; the
/// edge slot, the face's three slots (back pointers, links to the neighbour
/// face `i + 1`, wrapping to the first) and its plane (the unit normal of
/// `(vb - va) x (va - point)` from the SSE cross product with the refined
/// `RSQRTSS`, the distance `z + (y + x)` of the normal times `vb`) are
/// filled in. The call returns 2 early when the list ends too soon or a face
/// has a zero normal. Afterwards the states of all faces (+0x44) and the
/// first +0x08 vertex records (+0x50, 0x40 apart) are cleared; returns 2
/// when the last edge reached is not `edge_list` (the hole did not close),
/// else 0.
pub fn fn_00e2d1b0(e: &mut Engine, this: Ptr, count: i32, edge_list: Ptr, point: Ptr) -> u32 {
    let t = this.addr();
    // A scratch array of face pointers on the stack (up to 100).
    e.with_stack(0x200, |e, scratch| {
        let local = scratch.addr();
        let mut top = e.mem.i32(t + 0xc).wrapping_sub(1);
        let mut filled = 0u32;
        let mut remaining = count;
        if count != 0 {
            let mut slot = t
                .wrapping_add(0xde0)
                .wrapping_add((top as u32).wrapping_mul(4));
            loop {
                if top < 0 {
                    break;
                }
                let face = e.mem.u32(slot);
                let depth = e.mem.i32(t + 0xc);
                e.mem.set_i32(t + 0xc, depth.wrapping_sub(1));
                e.mem.set_u32(local + 4 * filled, face);
                remaining = remaining.wrapping_sub(1);
                filled += 1;
                top = top.wrapping_sub(1);
                slot = slot.wrapping_sub(4);
                if remaining == 0 {
                    break;
                }
            }
        }
        if remaining > 0 {
            let in_use = e.mem.i32(t + 0x10);
            let mut offset = (in_use as u32).wrapping_mul(0x50);
            for k in 0..remaining as u32 {
                e.mem.set_u32(
                    local + 4 * (filled + k),
                    offset.wrapping_add(t).wrapping_add(0xf70),
                );
                offset = offset.wrapping_add(0x50);
            }
            e.mem.set_i32(t + 0x10, in_use.wrapping_add(remaining));
        }

        let mut edge = edge_list.addr();
        let mut index = 0i32;
        if count > 0 {
            loop {
                if edge == 0 {
                    return 2;
                }
                let face = e.mem.u32(local + 4 * index as u32);
                let next_index = index + 1;
                let next_face = if next_index < count {
                    e.mem.u32(local + 4 * next_index as u32)
                } else {
                    e.mem.u32(local)
                };
                e.mem.set_u32(face + 0x1c, face);
                e.mem.set_u32(face + 0x2c, face);
                e.mem.set_u32(face + 0x3c, face);
                let second_slot = face + 0x20;
                e.mem.set_u32(face + 0x14, second_slot);
                let third_slot = face + 0x30;
                e.mem.set_u32(face + 0x24, third_slot);
                let first_slot = face + 0x10;
                e.mem.set_u32(face + 0x34, first_slot);
                let second_pointer = e.mem.u32(e.mem.u32(edge + 4));
                let first_pointer = e.mem.u32(edge);
                e.mem.set_u32(first_slot, second_pointer);
                e.mem.set_u32(third_slot, point.addr());
                e.mem.set_u32(face + 0x20, first_pointer);
                e.mem.set_u32(edge + 8, first_slot);
                e.mem.set_u32(face + 0x18, edge);
                e.mem.set_u32(next_face + 0x28, third_slot);
                e.mem.set_u32(face + 0x38, next_face + 0x20);

                let second = load_vec(e, second_pointer);
                let first = load_vec(e, first_pointer);
                let apex = load_vec(e, point.addr());
                let along = zip_with(second, first, |a, b| a - b);
                let back = zip_with(first, apex, |a, b| a - b);
                let normal = cross(along, back);
                let sum = sum3(zip_with(normal, normal, |a, b| a * b));
                if sum == 0.0 {
                    return 2;
                }
                let scale = inverse_length(e, sum);
                let unit: Vec4 = normal.map(|lane| scale * lane);
                let distance = sum3(zip_with(second, unit, |a, b| a * b));
                e.mem.set_f32(face + 0x40, x87_quiet(distance));
                store_vec(e, face, unit);
                let next_slot = e.mem.u32(edge + 4);
                let vertex = e.mem.u32(next_slot);
                edge = e.mem.u32(vertex + 0x30);
                e.mem.set_u32(vertex + 0x30, 0);
                index = next_index;
                if index >= count {
                    break;
                }
            }
        }
        for face in 0..e.mem.i32(t + 0x10).max(0) as u32 {
            e.mem.set_u32(t + 0xfb4 + 0x50 * face, 0);
        }
        for vertex in 0..e.mem.i32(t + 8).max(0) as u32 {
            e.mem.set_u32(t + 0x50 + 0x40 * vertex, 0);
        }
        if edge_list.addr() != edge {
            2
        } else {
            0
        }
    })
}

// Translated from 00e2d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets up the first four faces of the polytope (`this` in `ECX`): the face
/// count (+0x10) becomes 4 and each of the four faces (+0xf70, 0x50 bytes
/// apart) gets its slot links and, from the twelve-entry table at
/// `010ed0f0`, the vertex pointers of its three slots (`entry[0] + this +
/// 0x20`, and `entry[2] + entry[1] + this + 0xf80` for the pointer in +8);
/// state 0. The last face (+0x1060) is given its plane (`00e2cfe0`); if the
/// first two vertices (+0x20 and +0x60) satisfy `(v20 - v60) . normal > 0`
/// for that normal, the vertex pointers of three slots are redirected to the
/// vertices at +0x60 and +0x20 (+0x1020, +0xfd0, +0xf80 get +0x60; +0x1070,
/// +0xfe0, +0xfa0 get +0x20). Then all four faces get their planes.
pub fn fn_00e2d440(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t + 0x10, 4);
    let mut table = FACE_TABLE;
    let end = t + 0x10b0;
    let mut face = t + 0xf70;
    while face != end {
        e.mem.set_u32(face + 0x14, face + 0x20);
        e.mem.set_u32(face + 0x24, face + 0x30);
        let mut slot = face + 0x10;
        let limit = face + 0x40;
        e.mem.set_u32(face + 0x44, 0);
        e.mem.set_u32(face + 0x34, slot);
        while slot < limit {
            e.mem.set_u32(slot + 0xc, face);
            let first = e.mem.u32(table);
            e.mem
                .set_u32(slot, first.wrapping_add(t).wrapping_add(0x20));
            let third = e.mem.u32(table + 8).wrapping_add(e.mem.u32(table + 4));
            slot += 0x10;
            e.mem
                .set_u32(slot - 8, third.wrapping_add(t).wrapping_add(0xf80));
            table = table.wrapping_add(0xc);
        }
        face += 0x50;
    }
    let last_face = t + 0x1060;
    e.call(NORMALIZE_FACE, &args![last_face]);
    let v60 = t + 0x60;
    let v20 = t + 0x20;
    let difference = zip_with(load_vec(e, v20), load_vec(e, v60), |a, b| a - b);
    let normal = load_vec(e, last_face);
    let side = sum3(zip_with(difference, normal, |a, b| a * b));
    if side > 0.0 {
        e.mem.set_u32(t + 0x1020, v60);
        e.mem.set_u32(t + 0xfd0, v60);
        e.mem.set_u32(t + 0xf80, v60);
        e.mem.set_u32(last_face + 0x10, v20);
        e.mem.set_u32(t + 0xfe0, v20);
        e.mem.set_u32(t + 0xfa0, v20);
    }
    for k in 0..4 {
        e.call(NORMALIZE_FACE, &args![t + 0xf70 + 0x50 * k]);
    }
}

// Translated from 00e2d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hill-climbs over the faces of the polytope (`this` in `ECX`) from `face`:
/// resets the per-face states (`00e67fd0`), then repeatedly marks the
/// current face as state 3 and looks through its three slots, skipping the
/// one it was entered by. For slot `k` with vertices `p` (its pointer at
/// +0x00) and `q` (the pointer in the slot at +0x04), `d = q - p`, `f = (d x
/// normal) . p`, the slot counts as a way out when `-1e-10 * d.d > f * |f|`
/// (the constant is a `double`, the product stays unrounded) and the starting
/// face's plane distance (+0x40) is not above 0 or 0.9 times it is at most the
/// neighbour's. The walk moves to the neighbour across the slot (`slot + 8`
/// leads to its record, `+0xc` to the face), ending after that move when
/// the neighbour already has state 3. Returns the face it stopped at.
pub fn fn_00e2d550(e: &mut Engine, this: Ptr, face: Ptr) -> Ptr {
    e.call(RESET_STATES, &args![this]);
    let tiny: f64 = e.global(NEG_TINY);
    let nine_tenths: f64 = e.global(NINE_TENTHS);
    let mut current = face.addr();
    let mut excluded = 0u32;
    let mut done = false;
    loop {
        if done {
            return Ptr::new(current);
        }
        e.mem.set_u32(current + 0x44, 3);
        let mut slot_index = 0u32;
        let mut slot = current + 0x10;
        while slot_index < 3 {
            if slot != excluded {
                let p = load_vec(e, e.mem.u32(slot));
                let q_address = e.mem.u32(e.mem.u32(slot + 4));
                let q = load_vec(e, q_address);
                let d = zip_with(q, p, |a, b| a - b);
                let d_squared = sum3(zip_with(d, d, |a, b| a * b));
                let normal = load_vec(e, current);
                let crossed = cross(d, normal);
                let f = sum3(zip_with(crossed, p, |a, b| a * b));
                let bound = f as f64 * f.abs() as f64;
                let threshold = (d_squared as f64 * tiny) as f32;
                if threshold as f64 > bound {
                    let neighbour = e.mem.u32(e.mem.u32(slot + 8) + 0xc);
                    let own = e.mem.f32(face.addr() + 0x40);
                    let theirs = e.mem.f32(neighbour + 0x40);
                    let positive = 0.0 < own;
                    let nearer = own as f64 * nine_tenths > theirs as f64;
                    if !positive || !nearer {
                        excluded = e.mem.u32(current + 0x18 + 0x10 * slot_index);
                        done = e.mem.u32(neighbour + 0x44) == 3;
                        current = neighbour;
                        break;
                    }
                }
            }
            slot_index += 1;
            slot += 0x10;
        }
        if slot_index == 3 {
            return Ptr::new(current);
        }
    }
}

// Translated from 00e2da40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Support point of the pair of shapes along a direction (custom
/// convention): `shape_a` is `ECX`, `matrix` `ESI` (four column vectors: the
/// rotation and the translation of `shape_b` relative to `shape_a`) and
/// `direction` `EDI`; the stack arguments are `shape_b` and `record`.
///
/// Asks `shape_a` (virtual call, slot +0x38) for its vertex along `-direction`
/// and `shape_b` for its vertex along the direction turned by the transpose
/// of the rotation, moves that vertex by the transform (`(y * c1 + x * c0) +
/// (z * c2 + c3)` per lane) and takes `s = (va - vb') . direction` (sum `z +
/// (y + x)`). If the record's float at +0x2c is below `s` (ordered), the
/// record is updated: +0x00 the vertex of `shape_a`, +0x10 the vertex of
/// `shape_b` (before the transform), +0x20 the direction, +0x2c `s`.
pub fn fn_00e2da40(
    e: &mut Engine,
    shape_a: Ptr,
    matrix: Ptr,
    direction: Ptr,
    shape_b: Ptr,
    record: Ptr,
) {
    e.with_stack(0x40, |e, frame| {
        let flipped = frame.addr();
        let rotated = frame.addr() + 0x10;
        let vertex_b = frame.addr() + 0x20;
        let vertex_a = frame.addr() + 0x30;
        let direction_now = load_vec(e, direction.addr());
        let backwards = negated(e, direction_now);
        store_vec(e, flipped, backwards);
        e.vcall(shape_a.addr(), 0x38, &args![flipped, vertex_a]);

        let c0 = load_vec(e, matrix.addr());
        let c1 = load_vec(e, matrix.addr() + 0x10);
        let c2 = load_vec(e, matrix.addr() + 0x20);
        let d = load_vec(e, direction.addr());
        // The transposed rows: the x, y, z of each column, with lane 3 taken
        // from c2's y and w.
        let row_x = [c0[0], c1[0], c2[0], c2[1]];
        let row_y = [c0[1], c1[1], c2[1], c2[3]];
        let row_z = [c0[2], c1[2], c2[2], c2[3]];
        let turned: Vec4 = std::array::from_fn(|lane| {
            (d[1] * row_y[lane] + d[0] * row_x[lane]) + d[2] * row_z[lane]
        });
        store_vec(e, rotated, turned);
        e.vcall(shape_b.addr(), 0x38, &args![rotated, vertex_b]);

        let b = load_vec(e, vertex_b);
        let c0 = load_vec(e, matrix.addr());
        let c1 = load_vec(e, matrix.addr() + 0x10);
        let c2 = load_vec(e, matrix.addr() + 0x20);
        let c3 = load_vec(e, matrix.addr() + 0x30);
        let direction_after = load_vec(e, direction.addr());
        let moved: Vec4 = std::array::from_fn(|lane| {
            (b[1] * c1[lane] + b[0] * c0[lane]) + (b[2] * c2[lane] + c3[lane])
        });
        let a = load_vec(e, vertex_a);
        let gap = zip_with(a, moved, |x, y| x - y);
        let score = sum3(zip_with(gap, direction_after, |x, y| x * y));
        // `FCOMP`: updates only when the record's float is below the score.
        if e.mem.f32(record.addr() + 0x2c) < score {
            store_vec(e, record.addr() + 0x20, direction_after);
            store_vec(e, record.addr(), a);
            store_vec(e, record.addr() + 0x10, b);
            e.mem.set_f32(record.addr() + 0x2c, x87_quiet(score));
        }
    });
}

// Translated from 00e2db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refines the pair of closest support points of two shapes (cdecl, six
/// stack arguments: the shapes `shape_a` and `shape_b`, the `matrix` of
/// `00e2da40`, and the outputs `out_a`, `out_b` and `out_c`).
///
/// A record of `00e2da40` starts with the float +0x2c at the lowest value
/// (`0104e148`). The three directions of the table at `01268370` and their
/// negations seed it (two calls each). Then, with a step `s` that starts at
/// 1.0, every round shrinks it (`s = s * 0.99`, the `double` at `0106d508`),
/// takes the record's direction `d`, builds a vector `u` perpendicular to
/// it (two lanes picked from the smallest absolute components, one negated),
/// the unit `un = u / |u|` (refined `RSQRTSS`), the cross product `d x un`,
/// and the four directions `d + s un`, `d - s un`, `s (d x un) + d` and
/// `d - s (d x un)`, each scaled by the same normalising factor (that of
/// the first); each goes through `00e2da40`. The step is then halved and the
/// loop goes on while it exceeds 0.001. Finally `out_a` gets the record's
/// +0x00 vector, `out_b` its +0x10 vector, and `out_c` the direction (+0x00,
/// lane 3 being the score), the +0x00 vector at +0x10 and the score at +0x20.
pub fn fn_00e2db70(
    e: &mut Engine,
    shape_a: Ptr,
    shape_b: Ptr,
    matrix: Ptr,
    out_a: Ptr,
    out_b: Ptr,
    out_c: Ptr,
) {
    e.with_stack(0x100, |e, frame| {
        let base = frame.addr();
        let record = Ptr::<()>::new(base + 0x40);
        let start = x87_quiet(e.global::<f32>(MIN_FLOAT));
        e.mem.set_f32(base + 0x6c, start);
        for k in 0..3u32 {
            copy_vec(e, START_DIRECTIONS + 0x10 * k, base + 0x20);
            e.call(
                SUPPORT_POINT,
                &args![shape_a, matrix, base + 0x20, shape_b, record],
            );
            let flipped = negated(e, load_vec(e, base + 0x20));
            store_vec(e, base + 0x20, flipped);
            e.call(
                SUPPORT_POINT,
                &args![shape_a, matrix, base + 0x20, shape_b, record],
            );
        }

        let shrink: f64 = e.global(SHRINK);
        let half: f64 = e.global(HALF_DOUBLE);
        let mut step = 1.0f32;
        loop {
            step = (step as f64 * shrink) as f32;
            let d = load_vec(e, base + 0x60);
            let abs_x = d[0].abs();
            let abs_y = d[1].abs();
            let abs_z = d[2].abs();
            let mut first = 1usize;
            let mut second = 2usize;
            let mut third = 0usize;
            let mut smallest = abs_x;
            if abs_x > abs_y {
                smallest = abs_y;
                first = 0;
                third = 1;
            }
            if smallest > abs_z {
                second = third;
            }
            let mut u = [0.0f32; 4];
            u[first] = x87_quiet(d[second]);
            u[second] = x87_negated(d[first]);
            let u_sum = sum3(zip_with(u, u, |a, b| a * b));
            let u_scale = inverse_length(e, u_sum);
            let unit_u: Vec4 = u.map(|lane| u_scale * lane);
            let scaled_u: Vec4 = unit_u.map(|lane| step * lane);
            let crossed = cross(d, unit_u);
            let scaled_cross: Vec4 = crossed.map(|lane| lane * step);
            let plus = zip_with(d, scaled_u, |a, b| a + b);
            let plus_sum = sum3(zip_with(plus, plus, |a, b| a * b));
            let scale = inverse_length(e, plus_sum);
            let directions: [Vec4; 4] = [
                plus.map(|lane| scale * lane),
                zip_with(d, scaled_u, |a, b| a - b).map(|lane| lane * scale),
                zip_with(scaled_cross, d, |a, b| a + b).map(|lane| lane * scale),
                zip_with(d, scaled_cross, |a, b| a - b).map(|lane| lane * scale),
            ];
            let slots = [0xb0u32, 0x90, 0xa0, 0xc0];
            for (direction, slot) in directions.iter().zip(slots) {
                store_vec(e, base + slot, *direction);
            }
            for slot in slots {
                e.call(
                    SUPPORT_POINT,
                    &args![shape_a, matrix, base + slot, shape_b, record],
                );
            }
            step = (step as f64 * half) as f32;
            // `FCOM`: the loop goes on only while the step is above the limit.
            let limit: f32 = e.global(SMALL_STEP);
            let go_on = step > limit;
            if !go_on {
                break;
            }
        }
        copy_vec(e, base + 0x40, out_a.addr());
        copy_vec(e, base + 0x50, out_b.addr());
        let best = x87_quiet(e.mem.f32(base + 0x6c));
        e.mem.set_f32(out_c.addr() + 0x20, best);
        copy_vec(e, base + 0x60, out_c.addr());
        copy_vec(e, base + 0x40, out_c.addr() + 0x10);
    });
}

// Translated from 00e2b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: keeps the `store`'s best entries by key. `store` (the third word)
/// is a list of 0x60-byte entries: `+8` the array, `+0xc` the write pointer, `+0x10` the
/// capacity, `+0x14` the number in use, `+0x18` an override pointer. `candidate` has the float
/// key at `+0x10`. While the list has room the entry at the write pointer is used (the pointer
/// advances by 0x60, the count by one). When it is full, the entry with the largest key (the
/// first of equals; compares as `FCOMPP`: a NaN never wins) is the victim, and the function
/// returns when its key is not strictly above the candidate's (a NaN also returns). The entry
/// gets the candidate's key (through the x87 stack, `+0x10`), its first 16 bytes (`+0`), its
/// word at `+0x14` and, at `+0x50`, the override pointer or, when it is zero, the last node of
/// the chain that starts at `chain` (the nodes link through `+0xc`). Finally the chain's length
/// `n` (the number of links after `chain`) is counted, the entry's word `n` at `+0x20` becomes
/// `0xffffffff` and the words `n - 1` down to 0 get `[node + 4]` of the chain's first `n`
/// nodes. The function takes three stack words and pops none (`RET`) and returns nothing.
pub fn fn_00e2b9b0(e: &mut Engine, chain: Ptr, candidate: Ptr, store: Ptr) {
    let (chain, candidate, store) = (chain.addr(), candidate.addr(), store.addr());
    let count = e.mem.i32(store + 0x14);
    let entry = if count < e.mem.i32(store + 0x10) {
        let entry = e.mem.u32(store + 0xc);
        e.mem.set_i32(store + 0x14, count.wrapping_add(1));
        e.mem.set_u32(store + 0xc, entry.wrapping_add(0x60));
        entry
    } else {
        let mut best = e.mem.u32(store + 8);
        if count > 1 {
            let mut other = best.wrapping_add(0x60);
            let mut remaining = count - 1;
            loop {
                let other_key = x87_load32(e.mem.u32(other + 0x10));
                let best_key = x87_load32(e.mem.u32(best + 0x10));
                if best_key < other_key {
                    best = other;
                }
                other = other.wrapping_add(0x60);
                remaining -= 1;
                if remaining == 0 {
                    break;
                }
            }
        }
        let new_key = x87_load32(e.mem.u32(candidate + 0x10));
        let best_key = x87_load32(e.mem.u32(best + 0x10));
        if best_key.partial_cmp(&new_key) != Some(std::cmp::Ordering::Greater) {
            return;
        }
        best
    };
    let key = x87_quiet_bits(e.mem.u32(candidate + 0x10));
    e.mem.set_u32(entry + 0x10, key);
    for lane in 0..4 {
        let word = e.mem.u32(candidate + 4 * lane);
        e.mem.set_u32(entry + 4 * lane, word);
    }
    let word = e.mem.u32(candidate + 0x14);
    e.mem.set_u32(entry + 0x14, word);
    let override_pointer = e.mem.u32(store + 0x18);
    if override_pointer != 0 {
        e.mem.set_u32(entry + 0x50, override_pointer);
    } else {
        let mut last = chain;
        let mut link = e.mem.u32(chain + 0xc);
        if link != 0 {
            loop {
                last = link;
                link = e.mem.u32(last + 0xc);
                if link == 0 {
                    break;
                }
            }
        }
        e.mem.set_u32(entry + 0x50, last);
    }
    let mut length: u32 = 0;
    let mut link = e.mem.u32(chain + 0xc);
    if link != 0 {
        loop {
            link = e.mem.u32(link + 0xc);
            length += 1;
            if link == 0 {
                break;
            }
        }
    }
    e.mem.set_u32(entry + length * 4 + 0x20, 0xffff_ffff);
    let mut index = length as i32 - 1;
    if index >= 0 {
        let mut node = chain;
        let mut slot = entry
            .wrapping_add((index as u32).wrapping_mul(4))
            .wrapping_add(0x20);
        loop {
            let word = e.mem.u32(node + 4);
            e.mem.set_u32(slot, word);
            node = e.mem.u32(node + 0xc);
            index -= 1;
            slot = slot.wrapping_sub(4);
            if index < 0 {
                break;
            }
        }
    }
}

// Translated from 00e2baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: initialises the record `object` from the record `source` and a
/// word `extra`: `1.0f` (through the x87 stack) at `+0x14`, the byte 0 at `+0x18`, `source`
/// itself at `+0x1c` and `+0x20`, `[source + 0x30]` at `+0x24` and `+0x28`, `[source + 0x24]` at
/// `+0x50`, and at `+0x54` the word `extra + 0x10` when the byte at `source + 0x20` is not zero
/// and `extra` is not zero, else 0. The function takes three stack words and pops none (`RET`)
/// and returns nothing.
pub fn fn_00e2baa0(e: &mut Engine, object: Ptr, source: Ptr, extra: u32) {
    let (object, source) = (object.addr(), source.addr());
    e.mem.set_u32(object + 0x14, 0x3f80_0000);
    e.mem.set_u8(object + 0x18, 0);
    let first = e.mem.u32(source + 0x30);
    e.mem.set_u32(object + 0x24, first);
    let second = e.mem.u32(source + 0x30);
    e.mem.set_u32(object + 0x20, source);
    e.mem.set_u32(object + 0x28, second);
    let third = e.mem.u32(source + 0x24);
    e.mem.set_u32(object + 0x1c, source);
    e.mem.set_u32(object + 0x50, third);
    let selected = if e.mem.i8(source + 0x20) != 0 {
        extra
    } else {
        0
    };
    if selected != 0 {
        e.mem.set_u32(object + 0x54, selected.wrapping_add(0x10));
    } else {
        e.mem.set_u32(object + 0x54, 0);
    }
}

// Translated from 00e2baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: transposes eight vectors into six (pure SSE shuffles, nothing is
/// computed). `a` .. `h` are the vectors at the eight pointers (stack words 2 to 9) and `tail`
/// the tenth word: `out[0]` = x of `a`, `c`, `e`, `g`; `out[1]` = their y; `out[2]` = their z;
/// `out[3]` = x of `b`, `d`, `f`, `h`; `out[4]` = their y; `out[5]` = their z; and the word at
/// `out + 0x60` is `tail`. All eight vectors are loaded before anything is stored. The function
/// takes ten stack words and pops none (`RET`) and returns nothing.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e2baf0(
    e: &mut Engine,
    out: Ptr,
    a: Ptr,
    b: Ptr,
    c: Ptr,
    d: Ptr,
    e_vector: Ptr,
    f: Ptr,
    g: Ptr,
    h: Ptr,
    tail: u32,
) {
    let load = |e: &Engine, at: Ptr| -> [u32; 4] {
        std::array::from_fn(|lane| e.mem.u32(at.addr() + 4 * lane as u32))
    };
    let (a, b, c, d) = (load(e, a), load(e, b), load(e, c), load(e, d));
    let (e_lanes, f, g, h) = (load(e, e_vector), load(e, f), load(e, g), load(e, h));
    let rows = [
        [a[0], c[0], e_lanes[0], g[0]],
        [a[1], c[1], e_lanes[1], g[1]],
        [a[2], c[2], e_lanes[2], g[2]],
        [b[0], d[0], f[0], h[0]],
        [b[1], d[1], f[1], h[1]],
        [b[2], d[2], f[2], h[2]],
    ];
    for (row, lanes) in rows.into_iter().enumerate() {
        for (lane, word) in lanes.into_iter().enumerate() {
            e.mem
                .set_u32(out.addr() + 0x10 * row as u32 + 4 * lane as u32, word);
        }
    }
    e.mem.set_u32(out.addr() + 0x60, tail);
}

// Translated from 00e2b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the same function as `00e2b510` with other frame offsets and the
/// callee `00d1d530`). Arguments (cdecl, four stack words, nothing popped): `owner` (nothing
/// happens when it is null), a `source` record, a word `value` and a four-float `vector`. It
/// builds a `0x3a4`-byte stack frame: `00d1bb50` (ten stack words, the last `0xf`) converts eight
/// vectors of `source` (at `+0x00`, `+0x10`, `+0x30`, `+0x40`, `+0x60`, `+0x70`, `+0x90` and
/// `+0xa0`) into the frame; four 0x80-byte local objects get their vtable words (`010dbacc`,
/// `010dbae4`, `010dbad8`, `010dbaf8`), `1.0f`, `value` and records copied from `source`
/// (`+0xc0..+0xd4` and the words of the four sub-records); a four-pointer array refers to
/// them; the vector is copied from `vector`. Then `00d1d530` (cdecl, six words: `owner`,
/// `[owner + 0x10]`, 0, the converted block, the pointer array and the vector copy) runs, four
/// words of the frame go to `source + 0xe0..+0xec` and the vector copy back to `vector`
/// (interleaved by `UNPCKLPS`); the closing loop writes the vtable words of four destroyed
/// locals into the frame that is about to be discarded.
///
/// A register-level translation of straight-line code: each statement ends with the address of
/// the instruction it comes from; `frame` is the address of what the exe's `ESP` points at after
/// its prologue (the saved registers) and the offsets are relative to it, taking the words
/// still pushed for the unpopped cdecl calls into account.
#[allow(unused_assignments)]
pub fn fn_00e2b070(e: &mut Engine, owner: Ptr, source: Ptr, value: u32, vector: Ptr) {
    let (arg0, arg1, arg2, arg3) = (owner.addr(), source.addr(), value, vector.addr());
    if arg0 == 0 {
        return;
    }
    e.with_stack(0x420, |e, buffer| {
        let frame = buffer.addr() + 0x60;
        let (mut eax, mut ecx, mut edx, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32);
        let ebx = 0u32;
        let mut xmm = [[0u32; 4]; 8];
        let mut st0 = 0u32;
        esi = arg1; // 00e2b08a
        let pushed0 = 0xf;
        e.mem.set_u32(frame - 0x4, pushed0); // 00e2b08d
        edx = esi.wrapping_add(0xa0); // 00e2b08f
        let pushed1 = edx;
        e.mem.set_u32(frame - 0x8, pushed1); // 00e2b095
        edi = esi.wrapping_add(0x90); // 00e2b096
        let pushed2 = edi;
        e.mem.set_u32(frame - 0xc, pushed2); // 00e2b09c
        edx = esi.wrapping_add(0x70); // 00e2b09d
        let pushed3 = edx;
        e.mem.set_u32(frame - 0x10, pushed3); // 00e2b0a0
        eax = esi.wrapping_add(0x60); // 00e2b0a1
        let pushed4 = eax;
        e.mem.set_u32(frame - 0x14, pushed4); // 00e2b0a4
        eax = esi.wrapping_add(0x40); // 00e2b0a5
        let pushed5 = eax;
        e.mem.set_u32(frame - 0x18, pushed5); // 00e2b0a8
        ecx = esi.wrapping_add(0x30); // 00e2b0a9
        let pushed6 = ecx;
        e.mem.set_u32(frame - 0x1c, pushed6); // 00e2b0ac
        eax = esi.wrapping_add(0x10); // 00e2b0ad
        let pushed7 = eax;
        e.mem.set_u32(frame - 0x20, pushed7); // 00e2b0b0
        ecx = frame + 0x340; // 00e2b0b1
        let pushed8 = esi;
        e.mem.set_u32(frame - 0x24, pushed8); // 00e2b0b8
        let pushed9 = ecx;
        e.mem.set_u32(frame - 0x28, pushed9); // 00e2b0b9
        e.call(
            0x00d1bb50,
            &args![
                pushed9, pushed8, pushed7, pushed6, pushed5, pushed4, pushed3, pushed2, pushed1,
                pushed0
            ],
        ); // 00e2b0ba
        st0 = 0x3f80_0000; // 00e2b0bf
        xmm[0] = load_lanes(e, esi); // 00e2b0c1
        e.mem.set_u32(frame + 0x144, st0); // 00e2b0c4
        eax = 0x10dbacc; // 00e2b0cb
        e.mem.set_u32(frame + 0x1c4, st0); // 00e2b0d0
        edx = 0x10dbae4; // 00e2b0d7
        e.mem.set_u32(frame + 0x244, st0); // 00e2b0dc
        e.mem.set_u32(frame + 0x13c, eax); // 00e2b0e3
        e.mem.set_u32(frame + 0x2c4, st0); // 00e2b0ea
        e.mem.set_u32(frame + 0x1bc, eax); // 00e2b0f1
        e.mem.set_u32(frame + 0x23c, eax); // 00e2b0f8
        e.mem.set_u32(frame + 0x2bc, eax); // 00e2b0ff
        eax = e.mem.u32(esi.wrapping_add(0xc8)); // 00e2b106
        ecx = 0x10dbad8; // 00e2b10c
        e.mem.set_u32(frame + 0x60, eax); // 00e2b111
        e.mem.set_u32(frame + 0x154, eax); // 00e2b118
        e.mem.set_u32(frame + 0x158, eax); // 00e2b11f
        eax = e.mem.u32(esi.wrapping_add(0x24)); // 00e2b126
        e.mem.set_u32(frame + 0x134, edx); // 00e2b129
        e.mem.set_u32(frame + 0x1b4, edx); // 00e2b130
        e.mem.set_u32(frame + 0x234, edx); // 00e2b137
        e.mem.set_u32(frame + 0x2b4, edx); // 00e2b13e
        edx = u32::from(e.mem.u8(esi.wrapping_add(0x20))); // 00e2b145
        e.mem.set_u32(frame + 0x54, eax); // 00e2b149
        eax = (eax & 0xffff_ff00) | u32::from(e.mem.u8(esi.wrapping_add(0xc4))); // 00e2b14d
        store_lanes(e, frame + 0x30, xmm[0]); // 00e2b153
        xmm[0] = load_lanes(e, esi.wrapping_add(0x10)); // 00e2b158
        e.mem.set_u32(frame + 0x138, ecx); // 00e2b15c
        e.mem.set_u32(frame + 0x1b8, ecx); // 00e2b163
        e.mem.set_u32(frame + 0x238, ecx); // 00e2b16a
        e.mem.set_u32(frame + 0x2b8, ecx); // 00e2b171
        ecx = arg2; // 00e2b178
        e.mem.set_u8(frame + 0x6c, ebx as u8); // 00e2b17b
        e.mem.set_u8(frame + 0xac, ebx as u8); // 00e2b182
        e.mem.set_u8(frame + 0xec, ebx as u8); // 00e2b189
        e.mem.set_u8(frame + 0x12c, ebx as u8); // 00e2b190
        e.mem.set_u32(frame + 0x130, 0x10dbaf8); // 00e2b197
        e.mem.set_u32(frame + 0x180, ebx); // 00e2b1a2
        e.mem.set_u32(frame + 0x184, ebx); // 00e2b1a9
        e.mem.set_u32(frame + 0x190, ebx); // 00e2b1b0
        e.mem.set_u8(frame + 0x148, ebx as u8); // 00e2b1b7
        e.mem.set_u32(frame + 0x1b0, 0x10dbaf8); // 00e2b1be
        e.mem.set_u32(frame + 0x200, ebx); // 00e2b1c9
        e.mem.set_u32(frame + 0x204, ebx); // 00e2b1d0
        e.mem.set_u32(frame + 0x210, ebx); // 00e2b1d7
        e.mem.set_u8(frame + 0x1c8, ebx as u8); // 00e2b1de
        e.mem.set_u32(frame + 0x230, 0x10dbaf8); // 00e2b1e5
        e.mem.set_u32(frame + 0x280, ebx); // 00e2b1f0
        e.mem.set_u32(frame + 0x284, ebx); // 00e2b1f7
        e.mem.set_u32(frame + 0x290, ebx); // 00e2b1fe
        e.mem.set_u8(frame + 0x248, ebx as u8); // 00e2b205
        e.mem.set_u32(frame + 0x2b0, 0x10dbaf8); // 00e2b20c
        e.mem.set_u32(frame + 0x300, ebx); // 00e2b217
        e.mem.set_u32(frame + 0x304, ebx); // 00e2b21e
        e.mem.set_u32(frame + 0x310, ebx); // 00e2b225
        e.mem.set_u8(frame + 0x2c8, ebx as u8); // 00e2b22c
        store_lanes(e, frame + 0x40, xmm[0]); // 00e2b233
        e.mem.set_u8(frame + 0x50, edx as u8); // 00e2b238
        e.mem.set_u8(frame + 0x6d, eax as u8); // 00e2b23c
        e.mem.set_u32(frame + 0x64, 0x1); // 00e2b243
        e.mem.set_u32(frame + 0x68, ebx); // 00e2b24e
        edx = frame + 0x30; // 00e2b255
        e.mem.set_u32(frame + 0x150, edx); // 00e2b259
        e.mem.set_u32(frame + 0x140, ecx); // 00e2b260
        ecx = edx; // 00e2b267
        edx = frame + 0x138; // 00e2b269
        e.mem.set_u32(frame + 0x20, edx); // 00e2b270
        edx = e.mem.u32(esi.wrapping_add(0xcc)); // 00e2b274
        e.mem.set_u32(frame + 0xa0, edx); // 00e2b27a
        e.mem.set_u32(frame + 0x1d4, edx); // 00e2b281
        e.mem.set_u32(frame + 0x1d8, edx); // 00e2b288
        edx = u32::from(e.mem.u8(esi.wrapping_add(0x50))); // 00e2b28f
        e.mem.set_u8(frame + 0x90, edx as u8); // 00e2b293
        edx = e.mem.u32(esi.wrapping_add(0x54)); // 00e2b29a
        e.mem.set_u32(frame + 0x94, edx); // 00e2b29d
        edx = arg2; // 00e2b2a4
        e.mem.set_u32(frame + 0x1c0, edx); // 00e2b2a7
        xmm[0] = load_lanes(e, esi.wrapping_add(0x30)); // 00e2b2ae
        edx = frame + 0x70; // 00e2b2b2
        e.mem.set_u32(frame + 0x1d0, edx); // 00e2b2b9
        e.mem.set_u32(frame + 0x1cc, edx); // 00e2b2c0
        edx = frame + 0x1b8; // 00e2b2c7
        e.mem.set_u32(frame + 0x24, edx); // 00e2b2ce
        edx = e.mem.u32(esi.wrapping_add(0xd0)); // 00e2b2d2
        e.mem.set_u32(frame + 0xe0, edx); // 00e2b2d8
        e.mem.set_u32(frame + 0x254, edx); // 00e2b2df
        e.mem.set_u32(frame + 0x258, edx); // 00e2b2e6
        edx = u32::from(e.mem.u8(esi.wrapping_add(0x80))); // 00e2b2ed
        e.mem.set_u8(frame + 0xd0, edx as u8); // 00e2b2f4
        edx = e.mem.u32(esi.wrapping_add(0x84)); // 00e2b2fb
        e.mem.set_u32(frame + 0xd4, edx); // 00e2b301
        edx = arg2; // 00e2b308
        e.mem.set_u32(frame + 0x240, edx); // 00e2b30b
        edx = frame + 0xb0; // 00e2b312
        e.mem.set_u32(frame + 0x250, edx); // 00e2b319
        e.mem.set_u32(frame + 0x24c, edx); // 00e2b320
        edx = frame + 0x238; // 00e2b327
        e.mem.set_u32(frame + 0x28, edx); // 00e2b32e
        edx = e.mem.u32(esi.wrapping_add(0xd4)); // 00e2b332
        e.mem.set_u32(frame + 0x120, edx); // 00e2b338
        e.mem.set_u32(frame + 0x2d4, edx); // 00e2b33f
        e.mem.set_u32(frame + 0x2d8, edx); // 00e2b346
        edx = u32::from(e.mem.u8(edi.wrapping_add(0x20))); // 00e2b34d
        e.mem.set_u32(frame + 0x14c, ecx); // 00e2b351
        ecx = arg3; // 00e2b358
        store_lanes(e, frame + 0x70, xmm[0]); // 00e2b35b
        xmm[0] = load_lanes(e, esi.wrapping_add(0x40)); // 00e2b363
        st0 = x87_quiet_bits(e.mem.u32(ecx)); // 00e2b367
        store_lanes(e, frame + 0x80, xmm[0]); // 00e2b369
        xmm[0] = load_lanes(e, esi.wrapping_add(0x60)); // 00e2b371
        e.mem.set_u32(frame + 0x10, st0); // 00e2b375
        st0 = x87_quiet_bits(e.mem.u32(ecx.wrapping_add(0x4))); // 00e2b379
        e.mem.set_u8(frame + 0x110, edx as u8); // 00e2b37c
        edx = e.mem.u32(edi.wrapping_add(0x24)); // 00e2b383
        e.mem.set_u32(frame + 0x14, st0); // 00e2b386
        st0 = x87_quiet_bits(e.mem.u32(ecx.wrapping_add(0x8))); // 00e2b38a
        store_lanes(e, frame + 0xb0, xmm[0]); // 00e2b38d
        xmm[0] = load_lanes(e, esi.wrapping_add(0x70)); // 00e2b395
        e.mem.set_u32(frame + 0x18, st0); // 00e2b399
        st0 = x87_quiet_bits(e.mem.u32(ecx.wrapping_add(0xc))); // 00e2b39d
        e.mem.set_u8(frame + 0xad, eax as u8); // 00e2b3a0
        e.mem.set_u8(frame + 0xed, eax as u8); // 00e2b3a7
        e.mem.set_u32(frame + 0x1c, st0); // 00e2b3ae
        e.mem.set_u8(frame + 0x12d, eax as u8); // 00e2b3b2
        eax = arg2; // 00e2b3b9
        store_lanes(e, frame + 0xc0, xmm[0]); // 00e2b3bc
        xmm[0] = load_lanes(e, edi); // 00e2b3c4
        e.mem.set_u32(frame + 0x114, edx); // 00e2b3c7
        edx = frame + 0xf0; // 00e2b3ce
        e.mem.set_u32(frame + 0x2c0, eax); // 00e2b3d5
        store_lanes(e, frame + 0xf0, xmm[0]); // 00e2b3dc
        xmm[0] = load_lanes(e, edi.wrapping_add(0x10)); // 00e2b3e4
        eax = edx; // 00e2b3e8
        e.mem.set_u32(frame + 0xa4, 0x1); // 00e2b3ea
        e.mem.set_u32(frame + 0xa8, ebx); // 00e2b3f5
        e.mem.set_u32(frame + 0xe4, 0x1); // 00e2b3fc
        e.mem.set_u32(frame + 0xe8, ebx); // 00e2b407
        store_lanes(e, frame + 0x100, xmm[0]); // 00e2b40e
        e.mem.set_u32(frame + 0x124, 0x1); // 00e2b416
        e.mem.set_u32(frame + 0x128, ebx); // 00e2b421
        e.mem.set_u32(frame + 0x2d0, edx); // 00e2b428
        e.mem.set_u32(frame + 0x2cc, eax); // 00e2b42f
        edx = e.mem.u32(esi.wrapping_add(0xc0)); // 00e2b436
        eax = e.mem.u32(edx.wrapping_mul(4).wrapping_add(0x11b3830)); // 00e2b43c
        ecx = frame + 0x2b8; // 00e2b443
        e.mem.set_u32(frame + 0x2c, ecx); // 00e2b44a
        ecx = frame + 0x10; // 00e2b44e
        let pushed10 = ecx;
        e.mem.set_u32(frame - 0x2c, pushed10); // 00e2b452
        e.mem.set_u32(frame + 0x3a0, eax); // 00e2b453
        edx = frame + 0x20; // 00e2b45a
        let pushed11 = edx;
        e.mem.set_u32(frame - 0x30, pushed11); // 00e2b45e
        eax = frame + 0x340; // 00e2b45f
        let pushed12 = eax;
        e.mem.set_u32(frame - 0x34, pushed12); // 00e2b466
        eax = arg0; // 00e2b467
        ecx = e.mem.u32(eax.wrapping_add(0x10)); // 00e2b46a
        let pushed13 = ebx;
        e.mem.set_u32(frame - 0x38, pushed13); // 00e2b46d
        let pushed14 = ecx;
        e.mem.set_u32(frame - 0x3c, pushed14); // 00e2b46e
        let pushed15 = eax;
        e.mem.set_u32(frame - 0x40, pushed15); // 00e2b46f
        e.call(
            0x00d1d530,
            &args![pushed15, pushed14, pushed13, pushed12, pushed11, pushed10],
        ); // 00e2b470
        xmm[0] = [e.mem.u32(frame + 0x10), 0, 0, 0]; // 00e2b475
        xmm[1] = [e.mem.u32(frame + 0x14), 0, 0, 0]; // 00e2b47b
        xmm[2] = [e.mem.u32(frame + 0x18), 0, 0, 0]; // 00e2b481
        xmm[3] = [e.mem.u32(frame + 0x1c), 0, 0, 0]; // 00e2b487
        edx = e.mem.u32(frame + 0x68); // 00e2b48d
        eax = e.mem.u32(frame + 0xa8); // 00e2b494
        ecx = e.mem.u32(frame + 0xe8); // 00e2b49b
        e.mem.set_u32(esi.wrapping_add(0xe0), edx); // 00e2b4a2
        edx = e.mem.u32(frame + 0x128); // 00e2b4a8
        e.mem.set_u32(esi.wrapping_add(0xe4), eax); // 00e2b4af
        eax = arg3; // 00e2b4b5
        e.mem.set_u32(esi.wrapping_add(0xe8), ecx); // 00e2b4b8
        e.mem.set_u32(esi.wrapping_add(0xec), edx); // 00e2b4be
        xmm[0] = [xmm[0][0], xmm[2][0], xmm[0][1], xmm[2][1]]; // 00e2b4c4
        xmm[1] = [xmm[1][0], xmm[3][0], xmm[1][1], xmm[3][1]]; // 00e2b4c7
        xmm[0] = [xmm[0][0], xmm[1][0], xmm[0][1], xmm[1][1]]; // 00e2b4ca
        store_lanes(e, eax, xmm[0]); // 00e2b4cd
                                     // ADD ESP,0x40 00e2b4d0
                                     // The exe's closing loop: four stores of vtable words into the dead locals.
        ecx = ebx.wrapping_add(3);
        eax = frame + 0x338;
        edi = 0x010c_cfac;
        esi = 0x010c_cf8c;
        edx = 0x010c_3bb8;
        loop {
            eax = eax.wrapping_sub(0x80);
            ecx = ecx.wrapping_sub(1);
            e.mem.set_u32(eax.wrapping_add(4), edi);
            e.mem.set_u32(eax, 0x010c_cfa0);
            e.mem.set_u32(eax.wrapping_sub(4), esi);
            e.mem.set_u32(eax.wrapping_sub(8), edx);
            if (ecx as i32) < 0 {
                break;
            }
        }
    });
}

// Translated from 00e2b510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the same function as `00e2b070` with other frame offsets and the
/// callee `00d1d490`). Arguments (cdecl, four stack words, nothing popped): `owner` (nothing
/// happens when it is null), a `source` record, a word `value` and a four-float `vector`. It
/// builds a `0x3a4`-byte stack frame: `00d1bb50` (ten stack words, the last `0xf`) converts eight
/// vectors of `source` (at `+0x00`, `+0x10`, `+0x30`, `+0x40`, `+0x60`, `+0x70`, `+0x90` and
/// `+0xa0`) into the frame; four 0x80-byte local objects get their vtable words (`010dbacc`,
/// `010dbae4`, `010dbad8`, `010dbaf8`), `1.0f`, `value` and records copied from `source`
/// (`+0xc0..+0xd4` and the words of the four sub-records); a four-pointer array refers to
/// them; the vector is copied from `vector`. Then `00d1d490` (cdecl, six words: `owner`,
/// `[owner + 0x10]`, 0, the converted block, the pointer array and the vector copy) runs, four
/// words of the frame go to `source + 0xe0..+0xec` and the vector copy back to `vector`
/// (interleaved by `UNPCKLPS`); the closing loop writes the vtable words of four destroyed
/// locals into the frame that is about to be discarded.
///
/// A register-level translation of straight-line code: each statement ends with the address of
/// the instruction it comes from; `frame` is the address of what the exe's `ESP` points at after
/// its prologue (the saved registers) and the offsets are relative to it, taking the words
/// still pushed for the unpopped cdecl calls into account.
#[allow(unused_assignments)]
pub fn fn_00e2b510(e: &mut Engine, owner: Ptr, source: Ptr, value: u32, vector: Ptr) {
    let (arg0, arg1, arg2, arg3) = (owner.addr(), source.addr(), value, vector.addr());
    if arg0 == 0 {
        return;
    }
    e.with_stack(0x420, |e, buffer| {
        let frame = buffer.addr() + 0x60;
        let (mut eax, mut ecx, mut edx, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32);
        let ebx = 0u32;
        let mut xmm = [[0u32; 4]; 8];
        let mut st0 = 0u32;
        esi = arg1; // 00e2b52a
        let pushed0 = 0xf;
        e.mem.set_u32(frame - 0x4, pushed0); // 00e2b52d
        edx = esi.wrapping_add(0xa0); // 00e2b52f
        let pushed1 = edx;
        e.mem.set_u32(frame - 0x8, pushed1); // 00e2b535
        edi = esi.wrapping_add(0x90); // 00e2b536
        let pushed2 = edi;
        e.mem.set_u32(frame - 0xc, pushed2); // 00e2b53c
        edx = esi.wrapping_add(0x70); // 00e2b53d
        let pushed3 = edx;
        e.mem.set_u32(frame - 0x10, pushed3); // 00e2b540
        eax = esi.wrapping_add(0x60); // 00e2b541
        let pushed4 = eax;
        e.mem.set_u32(frame - 0x14, pushed4); // 00e2b544
        eax = esi.wrapping_add(0x40); // 00e2b545
        let pushed5 = eax;
        e.mem.set_u32(frame - 0x18, pushed5); // 00e2b548
        ecx = esi.wrapping_add(0x30); // 00e2b549
        let pushed6 = ecx;
        e.mem.set_u32(frame - 0x1c, pushed6); // 00e2b54c
        eax = esi.wrapping_add(0x10); // 00e2b54d
        let pushed7 = eax;
        e.mem.set_u32(frame - 0x20, pushed7); // 00e2b550
        ecx = frame + 0x340; // 00e2b551
        let pushed8 = esi;
        e.mem.set_u32(frame - 0x24, pushed8); // 00e2b558
        let pushed9 = ecx;
        e.mem.set_u32(frame - 0x28, pushed9); // 00e2b559
        e.call(
            0x00d1bb50,
            &args![
                pushed9, pushed8, pushed7, pushed6, pushed5, pushed4, pushed3, pushed2, pushed1,
                pushed0
            ],
        ); // 00e2b55a
        st0 = 0x3f80_0000; // 00e2b55f
        xmm[0] = load_lanes(e, esi); // 00e2b561
        e.mem.set_u32(frame + 0x144, st0); // 00e2b564
        eax = 0x10dbacc; // 00e2b56b
        e.mem.set_u32(frame + 0x1c4, st0); // 00e2b570
        edx = 0x10dbae4; // 00e2b577
        e.mem.set_u32(frame + 0x244, st0); // 00e2b57c
        e.mem.set_u32(frame + 0x13c, eax); // 00e2b583
        e.mem.set_u32(frame + 0x2c4, st0); // 00e2b58a
        e.mem.set_u32(frame + 0x1bc, eax); // 00e2b591
        e.mem.set_u32(frame + 0x23c, eax); // 00e2b598
        e.mem.set_u32(frame + 0x2bc, eax); // 00e2b59f
        eax = e.mem.u32(esi.wrapping_add(0xc8)); // 00e2b5a6
        ecx = 0x10dbad8; // 00e2b5ac
        e.mem.set_u32(frame + 0x60, eax); // 00e2b5b1
        e.mem.set_u32(frame + 0x154, eax); // 00e2b5b8
        e.mem.set_u32(frame + 0x158, eax); // 00e2b5bf
        eax = e.mem.u32(esi.wrapping_add(0x24)); // 00e2b5c6
        e.mem.set_u32(frame + 0x134, edx); // 00e2b5c9
        e.mem.set_u32(frame + 0x1b4, edx); // 00e2b5d0
        e.mem.set_u32(frame + 0x234, edx); // 00e2b5d7
        e.mem.set_u32(frame + 0x2b4, edx); // 00e2b5de
        edx = u32::from(e.mem.u8(esi.wrapping_add(0x20))); // 00e2b5e5
        e.mem.set_u32(frame + 0x54, eax); // 00e2b5e9
        eax = (eax & 0xffff_ff00) | u32::from(e.mem.u8(esi.wrapping_add(0xc4))); // 00e2b5ed
        store_lanes(e, frame + 0x30, xmm[0]); // 00e2b5f3
        xmm[0] = load_lanes(e, esi.wrapping_add(0x10)); // 00e2b5f8
        e.mem.set_u32(frame + 0x138, ecx); // 00e2b5fc
        e.mem.set_u32(frame + 0x1b8, ecx); // 00e2b603
        e.mem.set_u32(frame + 0x238, ecx); // 00e2b60a
        e.mem.set_u32(frame + 0x2b8, ecx); // 00e2b611
        ecx = arg2; // 00e2b618
        e.mem.set_u8(frame + 0x6c, ebx as u8); // 00e2b61b
        e.mem.set_u8(frame + 0xac, ebx as u8); // 00e2b622
        e.mem.set_u8(frame + 0xec, ebx as u8); // 00e2b629
        e.mem.set_u8(frame + 0x12c, ebx as u8); // 00e2b630
        e.mem.set_u32(frame + 0x130, 0x10dbaf8); // 00e2b637
        e.mem.set_u32(frame + 0x180, ebx); // 00e2b642
        e.mem.set_u32(frame + 0x184, ebx); // 00e2b649
        e.mem.set_u32(frame + 0x190, ebx); // 00e2b650
        e.mem.set_u8(frame + 0x148, ebx as u8); // 00e2b657
        e.mem.set_u32(frame + 0x1b0, 0x10dbaf8); // 00e2b65e
        e.mem.set_u32(frame + 0x200, ebx); // 00e2b669
        e.mem.set_u32(frame + 0x204, ebx); // 00e2b670
        e.mem.set_u32(frame + 0x210, ebx); // 00e2b677
        e.mem.set_u8(frame + 0x1c8, ebx as u8); // 00e2b67e
        e.mem.set_u32(frame + 0x230, 0x10dbaf8); // 00e2b685
        e.mem.set_u32(frame + 0x280, ebx); // 00e2b690
        e.mem.set_u32(frame + 0x284, ebx); // 00e2b697
        e.mem.set_u32(frame + 0x290, ebx); // 00e2b69e
        e.mem.set_u8(frame + 0x248, ebx as u8); // 00e2b6a5
        e.mem.set_u32(frame + 0x2b0, 0x10dbaf8); // 00e2b6ac
        e.mem.set_u32(frame + 0x300, ebx); // 00e2b6b7
        e.mem.set_u32(frame + 0x304, ebx); // 00e2b6be
        e.mem.set_u32(frame + 0x310, ebx); // 00e2b6c5
        e.mem.set_u8(frame + 0x2c8, ebx as u8); // 00e2b6cc
        store_lanes(e, frame + 0x40, xmm[0]); // 00e2b6d3
        e.mem.set_u8(frame + 0x50, edx as u8); // 00e2b6d8
        e.mem.set_u8(frame + 0x6d, eax as u8); // 00e2b6dc
        e.mem.set_u32(frame + 0x64, 0x1); // 00e2b6e3
        e.mem.set_u32(frame + 0x68, ebx); // 00e2b6ee
        edx = frame + 0x30; // 00e2b6f5
        e.mem.set_u32(frame + 0x150, edx); // 00e2b6f9
        e.mem.set_u32(frame + 0x140, ecx); // 00e2b700
        ecx = edx; // 00e2b707
        edx = frame + 0x13c; // 00e2b709
        e.mem.set_u32(frame + 0x20, edx); // 00e2b710
        edx = e.mem.u32(esi.wrapping_add(0xcc)); // 00e2b714
        e.mem.set_u32(frame + 0xa0, edx); // 00e2b71a
        e.mem.set_u32(frame + 0x1d4, edx); // 00e2b721
        e.mem.set_u32(frame + 0x1d8, edx); // 00e2b728
        edx = u32::from(e.mem.u8(esi.wrapping_add(0x50))); // 00e2b72f
        e.mem.set_u8(frame + 0x90, edx as u8); // 00e2b733
        edx = e.mem.u32(esi.wrapping_add(0x54)); // 00e2b73a
        e.mem.set_u32(frame + 0x94, edx); // 00e2b73d
        edx = arg2; // 00e2b744
        e.mem.set_u32(frame + 0x1c0, edx); // 00e2b747
        xmm[0] = load_lanes(e, esi.wrapping_add(0x30)); // 00e2b74e
        edx = frame + 0x70; // 00e2b752
        e.mem.set_u32(frame + 0x1d0, edx); // 00e2b759
        e.mem.set_u32(frame + 0x1cc, edx); // 00e2b760
        edx = frame + 0x1bc; // 00e2b767
        e.mem.set_u32(frame + 0x24, edx); // 00e2b76e
        edx = e.mem.u32(esi.wrapping_add(0xd0)); // 00e2b772
        e.mem.set_u32(frame + 0xe0, edx); // 00e2b778
        e.mem.set_u32(frame + 0x254, edx); // 00e2b77f
        e.mem.set_u32(frame + 0x258, edx); // 00e2b786
        edx = u32::from(e.mem.u8(esi.wrapping_add(0x80))); // 00e2b78d
        e.mem.set_u8(frame + 0xd0, edx as u8); // 00e2b794
        edx = e.mem.u32(esi.wrapping_add(0x84)); // 00e2b79b
        e.mem.set_u32(frame + 0xd4, edx); // 00e2b7a1
        edx = arg2; // 00e2b7a8
        e.mem.set_u32(frame + 0x240, edx); // 00e2b7ab
        edx = frame + 0xb0; // 00e2b7b2
        e.mem.set_u32(frame + 0x250, edx); // 00e2b7b9
        e.mem.set_u32(frame + 0x24c, edx); // 00e2b7c0
        edx = frame + 0x23c; // 00e2b7c7
        e.mem.set_u32(frame + 0x28, edx); // 00e2b7ce
        edx = e.mem.u32(esi.wrapping_add(0xd4)); // 00e2b7d2
        e.mem.set_u32(frame + 0x120, edx); // 00e2b7d8
        e.mem.set_u32(frame + 0x2d4, edx); // 00e2b7df
        e.mem.set_u32(frame + 0x2d8, edx); // 00e2b7e6
        edx = u32::from(e.mem.u8(edi.wrapping_add(0x20))); // 00e2b7ed
        e.mem.set_u32(frame + 0x14c, ecx); // 00e2b7f1
        ecx = arg3; // 00e2b7f8
        store_lanes(e, frame + 0x70, xmm[0]); // 00e2b7fb
        xmm[0] = load_lanes(e, esi.wrapping_add(0x40)); // 00e2b803
        st0 = x87_quiet_bits(e.mem.u32(ecx)); // 00e2b807
        store_lanes(e, frame + 0x80, xmm[0]); // 00e2b809
        xmm[0] = load_lanes(e, esi.wrapping_add(0x60)); // 00e2b811
        e.mem.set_u32(frame + 0x10, st0); // 00e2b815
        st0 = x87_quiet_bits(e.mem.u32(ecx.wrapping_add(0x4))); // 00e2b819
        e.mem.set_u8(frame + 0x110, edx as u8); // 00e2b81c
        edx = e.mem.u32(edi.wrapping_add(0x24)); // 00e2b823
        e.mem.set_u32(frame + 0x14, st0); // 00e2b826
        st0 = x87_quiet_bits(e.mem.u32(ecx.wrapping_add(0x8))); // 00e2b82a
        store_lanes(e, frame + 0xb0, xmm[0]); // 00e2b82d
        xmm[0] = load_lanes(e, esi.wrapping_add(0x70)); // 00e2b835
        e.mem.set_u32(frame + 0x18, st0); // 00e2b839
        st0 = x87_quiet_bits(e.mem.u32(ecx.wrapping_add(0xc))); // 00e2b83d
        e.mem.set_u8(frame + 0xad, eax as u8); // 00e2b840
        e.mem.set_u8(frame + 0xed, eax as u8); // 00e2b847
        e.mem.set_u32(frame + 0x1c, st0); // 00e2b84e
        e.mem.set_u8(frame + 0x12d, eax as u8); // 00e2b852
        eax = arg2; // 00e2b859
        store_lanes(e, frame + 0xc0, xmm[0]); // 00e2b85c
        xmm[0] = load_lanes(e, edi); // 00e2b864
        e.mem.set_u32(frame + 0x114, edx); // 00e2b867
        edx = frame + 0xf0; // 00e2b86e
        e.mem.set_u32(frame + 0x2c0, eax); // 00e2b875
        store_lanes(e, frame + 0xf0, xmm[0]); // 00e2b87c
        xmm[0] = load_lanes(e, edi.wrapping_add(0x10)); // 00e2b884
        eax = edx; // 00e2b888
        e.mem.set_u32(frame + 0xa4, 0x1); // 00e2b88a
        e.mem.set_u32(frame + 0xa8, ebx); // 00e2b895
        e.mem.set_u32(frame + 0xe4, 0x1); // 00e2b89c
        e.mem.set_u32(frame + 0xe8, ebx); // 00e2b8a7
        store_lanes(e, frame + 0x100, xmm[0]); // 00e2b8ae
        e.mem.set_u32(frame + 0x124, 0x1); // 00e2b8b6
        e.mem.set_u32(frame + 0x128, ebx); // 00e2b8c1
        e.mem.set_u32(frame + 0x2d0, edx); // 00e2b8c8
        e.mem.set_u32(frame + 0x2cc, eax); // 00e2b8cf
        edx = e.mem.u32(esi.wrapping_add(0xc0)); // 00e2b8d6
        eax = e.mem.u32(edx.wrapping_mul(4).wrapping_add(0x11b3830)); // 00e2b8dc
        ecx = frame + 0x2bc; // 00e2b8e3
        e.mem.set_u32(frame + 0x2c, ecx); // 00e2b8ea
        ecx = frame + 0x10; // 00e2b8ee
        let pushed10 = ecx;
        e.mem.set_u32(frame - 0x2c, pushed10); // 00e2b8f2
        e.mem.set_u32(frame + 0x3a0, eax); // 00e2b8f3
        edx = frame + 0x20; // 00e2b8fa
        let pushed11 = edx;
        e.mem.set_u32(frame - 0x30, pushed11); // 00e2b8fe
        eax = frame + 0x340; // 00e2b8ff
        let pushed12 = eax;
        e.mem.set_u32(frame - 0x34, pushed12); // 00e2b906
        eax = arg0; // 00e2b907
        ecx = e.mem.u32(eax.wrapping_add(0x10)); // 00e2b90a
        let pushed13 = ebx;
        e.mem.set_u32(frame - 0x38, pushed13); // 00e2b90d
        let pushed14 = ecx;
        e.mem.set_u32(frame - 0x3c, pushed14); // 00e2b90e
        let pushed15 = eax;
        e.mem.set_u32(frame - 0x40, pushed15); // 00e2b90f
        e.call(
            0x00d1d490,
            &args![pushed15, pushed14, pushed13, pushed12, pushed11, pushed10],
        ); // 00e2b910
        xmm[0] = [e.mem.u32(frame + 0x10), 0, 0, 0]; // 00e2b915
        xmm[1] = [e.mem.u32(frame + 0x14), 0, 0, 0]; // 00e2b91b
        xmm[2] = [e.mem.u32(frame + 0x18), 0, 0, 0]; // 00e2b921
        xmm[3] = [e.mem.u32(frame + 0x1c), 0, 0, 0]; // 00e2b927
        edx = e.mem.u32(frame + 0x68); // 00e2b92d
        eax = e.mem.u32(frame + 0xa8); // 00e2b934
        ecx = e.mem.u32(frame + 0xe8); // 00e2b93b
        e.mem.set_u32(esi.wrapping_add(0xe0), edx); // 00e2b942
        edx = e.mem.u32(frame + 0x128); // 00e2b948
        e.mem.set_u32(esi.wrapping_add(0xe4), eax); // 00e2b94f
        eax = arg3; // 00e2b955
        e.mem.set_u32(esi.wrapping_add(0xe8), ecx); // 00e2b958
        e.mem.set_u32(esi.wrapping_add(0xec), edx); // 00e2b95e
        xmm[0] = [xmm[0][0], xmm[2][0], xmm[0][1], xmm[2][1]]; // 00e2b964
        xmm[1] = [xmm[1][0], xmm[3][0], xmm[1][1], xmm[3][1]]; // 00e2b967
        xmm[0] = [xmm[0][0], xmm[1][0], xmm[0][1], xmm[1][1]]; // 00e2b96a
        store_lanes(e, eax, xmm[0]); // 00e2b96d
                                     // ADD ESP,0x40 00e2b970
                                     // The exe's closing loop: four stores of vtable words into the dead locals.
        ecx = ebx.wrapping_add(3);
        eax = frame + 0x338;
        edi = 0x010c_cfac;
        esi = 0x010c_cf8c;
        edx = 0x010c_3bb8;
        loop {
            eax = eax.wrapping_sub(0x80);
            ecx = ecx.wrapping_sub(1);
            e.mem.set_u32(eax.wrapping_add(4), edi);
            e.mem.set_u32(eax, 0x010c_cfa0);
            e.mem.set_u32(eax.wrapping_sub(4), esi);
            e.mem.set_u32(eax.wrapping_sub(8), edx);
            if (ecx as i32) < 0 {
                break;
            }
        }
    });
}

// Translated from 00e2bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the leaf step of a ray cast over a table of boxes (the classes are
/// unconfirmed; `00e2c8a0` drives it). Twenty stack words, cdecl, nothing popped (`arg0` ..
/// `arg19`): `arg1` is the ray block, `arg3` the node table (0x20 bytes per node: a lower
/// corner and, at +0x10, a scale vector), `arg4` the hit collector (an object whose embedded
/// object at `+8` has a virtual method at vtable slot 4), `arg6` the leaf records (0x40 bytes),
/// `arg9` and `arg10` the node index list and its length, `arg14`/`arg15` and `arg16`/`arg17`
/// two (hit list, hit count) outputs, `arg18` and `arg19` two ray buffers. The counts `arg15`
/// and `arg17` point at are cleared first. Three branches: when bit `0x8000` of the 16-bit word at
/// `arg1 + 0x24` is set, each listed node is slab-tested against the ray (the vectors at `arg1`
/// and `arg1 + 0x10`; `(ray - lower) * scale`, `MINPS`/`MAXPS`, the largest entry component and
/// the zero vector `01267e30` against the smallest exit component and `1.0` through
/// `CMPLTPS`/`MOVMSKPS`) and a hit calls `00e2baa0(arg4, (index << 6) + arg6, arg7)` and the
/// collector's virtual method (`this` = `arg4 + 8`, one word: `arg1`); otherwise, when `arg12`
/// and `arg13` are at least the length, `arg11` is at least 2 and the length is above 4,
/// `00d1bb40(arg0 + 0x1c, word)` finds two ray blocks (from the words at `arg1 + 0x22` and
/// `+0x20`), `00d9a4e0` copies them into the buffers `arg18` and `arg19`, and every listed node
/// is tested against both, appending the index to the matching list and counting it;
/// otherwise the nodes go through the leaf solver four at a time (`00e2baa0`, `00e2baf0`,
/// `00d1ca00`), the values it leaves in the output frame (`arg4`, lanes of 0x80 bytes) going to
/// the table `arg8`. Written instruction by instruction: the exe's registers are locals, its stack
/// is a block of the engine's memory (so the addresses handed to the callees are real), its basic
/// blocks are the arms of a `match` on the block's address, and the comment on each statement
/// is the instruction it comes from.
#[allow(
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_labels,
    clippy::all
)]
pub fn fn_00e2bbd0(
    e: &mut Engine,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    arg7: u32,
    arg8: u32,
    arg9: u32,
    arg10: u32,
    arg11: u32,
    arg12: u32,
    arg13: u32,
    arg14: u32,
    arg15: u32,
    arg16: u32,
    arg17: u32,
    arg18: u32,
    arg19: u32,
) {
    e.with_stack(0x1400, |e, frame| {
        // The exe's registers; `esp` points at the return address slot, the arguments above it.
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut esp = frame.addr() + 0x1400 - 0x80;
        e.mem.set_u32(esp, 0);
        e.mem.set_u32(esp + 4, arg0);
        e.mem.set_u32(esp + 8, arg1);
        e.mem.set_u32(esp + 12, arg2);
        e.mem.set_u32(esp + 16, arg3);
        e.mem.set_u32(esp + 20, arg4);
        e.mem.set_u32(esp + 24, arg5);
        e.mem.set_u32(esp + 28, arg6);
        e.mem.set_u32(esp + 32, arg7);
        e.mem.set_u32(esp + 36, arg8);
        e.mem.set_u32(esp + 40, arg9);
        e.mem.set_u32(esp + 44, arg10);
        e.mem.set_u32(esp + 48, arg11);
        e.mem.set_u32(esp + 52, arg12);
        e.mem.set_u32(esp + 56, arg13);
        e.mem.set_u32(esp + 60, arg14);
        e.mem.set_u32(esp + 64, arg15);
        e.mem.set_u32(esp + 68, arg16);
        e.mem.set_u32(esp + 72, arg17);
        e.mem.set_u32(esp + 76, arg18);
        e.mem.set_u32(esp + 80, arg19);
        let mut xmm = [[0u32; 4]; 8];
        let mut fl = Flags::default();
        let mut st: Vec<f64> = Vec::new();
        let mut fsw: u16 = 0;
        let mut pc = 0x00e2_bbd0_u32;
        loop {
            match pc {
                0x00e2_bbd0 => {
                    // 00e2bbd0: PUSH EBP
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp);
                    // 00e2bbd1: MOV EBP,ESP
                    ebp = esp;
                    // 00e2bbd3: AND ESP,0xfffffff0
                    esp = fl.and(esp, 0xfffffff0_u32, 32);
                    // 00e2bbd6: MOV EAX,dword ptr [EBP + 0x4c]
                    eax = e.mem.u32(ebp.wrapping_add(0x4c));
                    // 00e2bbd9: MOV ECX,dword ptr [EBP + 0x44]
                    ecx = e.mem.u32(ebp.wrapping_add(0x44));
                    // 00e2bbdc: SUB ESP,0xd4
                    esp = fl.sub(esp, 0xd4_u32, 32);
                    // 00e2bbe2: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2bbe3: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2bbe4: XOR ESI,ESI
                    esi = fl.xor(esi, esi, 32);
                    // 00e2bbe6: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2bbe7: MOV EDI,dword ptr [EBP + 0xc]
                    edi = e.mem.u32(ebp.wrapping_add(0xc));
                    // 00e2bbea: MOV dword ptr [EAX],ESI
                    e.mem.set_u32(eax, esi);
                    // 00e2bbec: MOV EDX,0x8000
                    edx = 0x8000_u32;
                    // 00e2bbf1: MOV dword ptr [ECX],ESI
                    e.mem.set_u32(ecx, esi);
                    // 00e2bbf3: TEST word ptr [EDI + 0x24],DX
                    fl.and(
                        u32::from(e.mem.u16(edi.wrapping_add(0x24))),
                        edx & 0xffff,
                        16,
                    );
                    // 00e2bbf7: JZ 0x00e2bcd5
                    if fl.zf {
                        pc = 0x00e2_bcd5;
                        continue;
                    }
                    pc = 0x00e2_bbfd;
                }
                0x00e2_bbfd => {
                    // 00e2bbfd: CMP dword ptr [EBP + 0x30],ESI
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x30)), esi, 32);
                    // 00e2bc00: MOVAPS XMM0,xmmword ptr [EDI]
                    xmm[0] = load_lanes(e, edi);
                    // 00e2bc03: MOVAPS xmmword ptr [ESP + 0x50],XMM0
                    store_lanes(e, esp.wrapping_add(0x50), xmm[0]);
                    // 00e2bc08: MOVAPS XMM0,xmmword ptr [EDI + 0x10]
                    xmm[0] = load_lanes(e, edi.wrapping_add(0x10));
                    // 00e2bc0c: MOVAPS xmmword ptr [ESP + 0x60],XMM0
                    store_lanes(e, esp.wrapping_add(0x60), xmm[0]);
                    // 00e2bc11: JLE 0x00e2c0ef
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c0ef;
                        continue;
                    }
                    pc = 0x00e2_bc17;
                }
                0x00e2_bc17 => {
                    // 00e2bc17: MOVSS XMM4,dword ptr [0x01012068]
                    xmm[4] = [e.mem.u32(0x1012068_u32), 0, 0, 0];
                    // 00e2bc1f: MOV EBX,dword ptr [EBP + 0x18]
                    ebx = e.mem.u32(ebp.wrapping_add(0x18));
                    // 00e2bc22: SHUFPS XMM4,XMM4,0x0
                    xmm[4] = shufps(xmm[4], xmm[4], 0);
                    // 00e2bc26: MOVAPS xmmword ptr [ESP + 0x40],XMM4
                    store_lanes(e, esp.wrapping_add(0x40), xmm[4]);
                    pc = 0x00e2_bc2b;
                }
                0x00e2_bc2b => {
                    // 00e2bc2b: MOV EAX,dword ptr [EBP + 0x2c]
                    eax = e.mem.u32(ebp.wrapping_add(0x2c));
                    // 00e2bc2e: MOV EAX,dword ptr [EAX + ESI*0x4]
                    eax = e.mem.u32(eax.wrapping_add(esi.wrapping_mul(4)));
                    // 00e2bc31: MOVAPS XMM1,xmmword ptr [ESP + 0x50]
                    xmm[1] = load_lanes(e, esp.wrapping_add(0x50));
                    // 00e2bc36: MOVAPS XMM2,xmmword ptr [ESP + 0x60]
                    xmm[2] = load_lanes(e, esp.wrapping_add(0x60));
                    // 00e2bc3b: MOV ECX,EAX
                    ecx = eax;
                    // 00e2bc3d: SHL ECX,0x5
                    ecx = fl.shift_shl(ecx, 0x5_u32, 32);
                    // 00e2bc40: ADD ECX,dword ptr [EBP + 0x14]
                    ecx = fl.add(ecx, e.mem.u32(ebp.wrapping_add(0x14)), 32);
                    // 00e2bc43: MOVAPS XMM0,xmmword ptr [ECX]
                    xmm[0] = load_lanes(e, ecx);
                    // 00e2bc46: MOVAPS XMM3,xmmword ptr [ECX + 0x10]
                    xmm[3] = load_lanes(e, ecx.wrapping_add(0x10));
                    // 00e2bc4a: SUBPS XMM1,XMM0
                    xmm[1] = subps(xmm[1], xmm[0]);
                    // 00e2bc4d: MULPS XMM1,XMM3
                    xmm[1] = mulps(xmm[1], xmm[3]);
                    // 00e2bc50: SUBPS XMM2,XMM0
                    xmm[2] = subps(xmm[2], xmm[0]);
                    // 00e2bc53: MULPS XMM2,XMM3
                    xmm[2] = mulps(xmm[2], xmm[3]);
                    // 00e2bc56: MOVAPS XMM0,XMM1
                    xmm[0] = xmm[1];
                    // 00e2bc59: MINPS XMM0,XMM2
                    xmm[0] = minps(xmm[0], xmm[2]);
                    // 00e2bc5c: MAXPS XMM1,XMM2
                    xmm[1] = maxps(xmm[1], xmm[2]);
                    // 00e2bc5f: MOVAPS XMM2,XMM1
                    xmm[2] = xmm[1];
                    // 00e2bc62: SHUFPS XMM2,XMM1,0xaa
                    xmm[2] = shufps(xmm[2], xmm[1], 170);
                    // 00e2bc66: MOVAPS XMM3,XMM1
                    xmm[3] = xmm[1];
                    // 00e2bc69: SHUFPS XMM3,XMM1,0x55
                    xmm[3] = shufps(xmm[3], xmm[1], 85);
                    // 00e2bc6d: MINPS XMM3,XMM2
                    xmm[3] = minps(xmm[3], xmm[2]);
                    // 00e2bc70: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2bc73: MINPS XMM1,XMM4
                    xmm[1] = minps(xmm[1], xmm[4]);
                    // 00e2bc76: MINPS XMM1,XMM3
                    xmm[1] = minps(xmm[1], xmm[3]);
                    // 00e2bc79: MOVAPS XMM3,XMM0
                    xmm[3] = xmm[0];
                    // 00e2bc7c: SHUFPS XMM2,XMM0,0xaa
                    xmm[2] = shufps(xmm[2], xmm[0], 170);
                    // 00e2bc80: SHUFPS XMM3,XMM0,0x55
                    xmm[3] = shufps(xmm[3], xmm[0], 85);
                    // 00e2bc84: MAXPS XMM0,xmmword ptr [0x01267e30]
                    xmm[0] = maxps(xmm[0], load_lanes(e, 0x1267e30_u32));
                    // 00e2bc8b: MAXPS XMM3,XMM2
                    xmm[3] = maxps(xmm[3], xmm[2]);
                    // 00e2bc8e: MAXPS XMM0,XMM3
                    xmm[0] = maxps(xmm[0], xmm[3]);
                    // 00e2bc91: CMPLTPS XMM0,XMM1
                    xmm[0] = cmpltps(xmm[0], xmm[1]);
                    // 00e2bc95: MOVMSKPS ECX,XMM0
                    ecx = movmskps(xmm[0]);
                    // 00e2bc98: TEST CL,0x1
                    fl.and(ecx & 0xff, 0x1_u32, 8);
                    // 00e2bc9b: JZ 0x00e2bcc4
                    if fl.zf {
                        pc = 0x00e2_bcc4;
                        continue;
                    }
                    pc = 0x00e2_bc9d;
                }
                0x00e2_bc9d => {
                    // 00e2bc9d: MOV EDX,dword ptr [EBP + 0x24]
                    edx = e.mem.u32(ebp.wrapping_add(0x24));
                    // 00e2bca0: SHL EAX,0x6
                    eax = fl.shift_shl(eax, 0x6_u32, 32);
                    // 00e2bca3: ADD EAX,dword ptr [EBP + 0x20]
                    eax = fl.add(eax, e.mem.u32(ebp.wrapping_add(0x20)), 32);
                    // 00e2bca6: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2bca7: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2bca8: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2bca9: CALL 0x00e2baa0
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                    ];
                    let result = e.call(0x00e2baa0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2bcae: MOV EAX,dword ptr [EBX + 0x8]
                    eax = e.mem.u32(ebx.wrapping_add(0x8));
                    // 00e2bcb1: MOV EDX,dword ptr [EAX + 0x4]
                    edx = e.mem.u32(eax.wrapping_add(0x4));
                    // 00e2bcb4: LEA ECX,[EBX + 0x8]
                    ecx = ebx.wrapping_add(0x8);
                    // 00e2bcb7: ADD ESP,0xc
                    esp = fl.add(esp, 0xc_u32, 32);
                    // 00e2bcba: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2bcbb: CALL EDX
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(edx, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    st.push(result.st0);
                    // 00e2bcbd: FSTP ST0
                    st.pop();
                    // 00e2bcbf: MOVAPS XMM4,xmmword ptr [ESP + 0x40]
                    xmm[4] = load_lanes(e, esp.wrapping_add(0x40));
                    pc = 0x00e2_bcc4;
                }
                0x00e2_bcc4 => {
                    // 00e2bcc4: INC ESI
                    esi = fl.inc(esi, 32);
                    // 00e2bcc5: CMP ESI,dword ptr [EBP + 0x30]
                    fl.sub(esi, e.mem.u32(ebp.wrapping_add(0x30)), 32);
                    // 00e2bcc8: JL 0x00e2bc2b
                    if fl.sf != fl.of {
                        pc = 0x00e2_bc2b;
                        continue;
                    }
                    pc = 0x00e2_bcce;
                }
                0x00e2_bcce => {
                    // 00e2bcce: POP EDI
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2bccf: POP ESI
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2bcd0: POP EBX
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2bcd1: MOV ESP,EBP
                    esp = ebp;
                    // 00e2bcd3: POP EBP
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2bcd4: RET
                    return;
                }
                0x00e2_bcd5 => {
                    // 00e2bcd5: MOV ECX,dword ptr [EBP + 0x30]
                    ecx = e.mem.u32(ebp.wrapping_add(0x30));
                    // 00e2bcd8: CMP dword ptr [EBP + 0x38],ECX
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x38)), ecx, 32);
                    // 00e2bcdb: JL 0x00e2be5c
                    if fl.sf != fl.of {
                        pc = 0x00e2_be5c;
                        continue;
                    }
                    pc = 0x00e2_bce1;
                }
                0x00e2_bce1 => {
                    // 00e2bce1: CMP dword ptr [EBP + 0x3c],ECX
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x3c)), ecx, 32);
                    // 00e2bce4: JL 0x00e2be5c
                    if fl.sf != fl.of {
                        pc = 0x00e2_be5c;
                        continue;
                    }
                    pc = 0x00e2_bcea;
                }
                0x00e2_bcea => {
                    // 00e2bcea: CMP dword ptr [EBP + 0x34],0x2
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x34)), 0x2_u32, 32);
                    // 00e2bcee: JL 0x00e2be5c
                    if fl.sf != fl.of {
                        pc = 0x00e2_be5c;
                        continue;
                    }
                    pc = 0x00e2_bcf4;
                }
                0x00e2_bcf4 => {
                    // 00e2bcf4: CMP ECX,0x4
                    fl.sub(ecx, 0x4_u32, 32);
                    // 00e2bcf7: JLE 0x00e2be5c
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_be5c;
                        continue;
                    }
                    pc = 0x00e2_bcfd;
                }
                0x00e2_bcfd => {
                    // 00e2bcfd: MOVZX EAX,word ptr [EDI + 0x22]
                    eax = u32::from(e.mem.u16(edi.wrapping_add(0x22)));
                    // 00e2bd01: MOV EBX,dword ptr [EBP + 0x8]
                    ebx = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2bd04: ADD EBX,0x1c
                    ebx = fl.add(ebx, 0x1c_u32, 32);
                    // 00e2bd07: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2bd08: MOV ECX,EBX
                    ecx = ebx;
                    // 00e2bd0a: CALL 0x00d1bb40
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d1bb40, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2bd0f: MOVZX ECX,word ptr [EDI + 0x20]
                    ecx = u32::from(e.mem.u16(edi.wrapping_add(0x20)));
                    // 00e2bd13: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2bd14: MOV ECX,EBX
                    ecx = ebx;
                    // 00e2bd16: MOV ESI,EAX
                    esi = eax;
                    // 00e2bd18: CALL 0x00d1bb40
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d1bb40, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2bd1d: MOV EDI,dword ptr [EBP + 0x50]
                    edi = e.mem.u32(ebp.wrapping_add(0x50));
                    // 00e2bd20: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2bd21: MOV ECX,EDI
                    ecx = edi;
                    // 00e2bd23: CALL 0x00d9a4e0
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d9a4e0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2bd28: MOV EBX,dword ptr [EBP + 0x54]
                    ebx = e.mem.u32(ebp.wrapping_add(0x54));
                    // 00e2bd2b: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2bd2c: MOV ECX,EBX
                    ecx = ebx;
                    // 00e2bd2e: CALL 0x00d9a4e0
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d9a4e0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2bd33: MOVAPS XMM5,xmmword ptr [EDI]
                    xmm[5] = load_lanes(e, edi);
                    // 00e2bd36: MOVAPS XMM6,xmmword ptr [EDI + 0x10]
                    xmm[6] = load_lanes(e, edi.wrapping_add(0x10));
                    // 00e2bd3a: MOVAPS XMM0,xmmword ptr [EBX + 0x10]
                    xmm[0] = load_lanes(e, ebx.wrapping_add(0x10));
                    // 00e2bd3e: MOVSS XMM4,dword ptr [0x01012068]
                    xmm[4] = [e.mem.u32(0x1012068_u32), 0, 0, 0];
                    // 00e2bd46: MOVAPS XMM7,xmmword ptr [EBX]
                    xmm[7] = load_lanes(e, ebx);
                    // 00e2bd49: XOR EDI,EDI
                    edi = fl.xor(edi, edi, 32);
                    // 00e2bd4b: CMP dword ptr [EBP + 0x30],EDI
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x30)), edi, 32);
                    // 00e2bd4e: MOVAPS xmmword ptr [ESP + 0x60],XMM0
                    store_lanes(e, esp.wrapping_add(0x60), xmm[0]);
                    // 00e2bd53: SHUFPS XMM4,XMM4,0x0
                    xmm[4] = shufps(xmm[4], xmm[4], 0);
                    // 00e2bd57: JLE 0x00e2c0ef
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c0ef;
                        continue;
                    }
                    pc = 0x00e2_bd5d;
                }
                0x00e2_bd5d => {
                    // 00e2bd5d: MOV ESI,dword ptr [EBP + 0x48]
                    esi = e.mem.u32(ebp.wrapping_add(0x48));
                    // 00e2bd60: MOV EBX,dword ptr [EBP + 0x40]
                    ebx = e.mem.u32(ebp.wrapping_add(0x40));
                    pc = 0x00e2_bd63;
                }
                0x00e2_bd63 => {
                    // 00e2bd63: MOV EDX,dword ptr [EBP + 0x2c]
                    edx = e.mem.u32(ebp.wrapping_add(0x2c));
                    // 00e2bd66: MOV EAX,dword ptr [EDX + EDI*0x4]
                    eax = e.mem.u32(edx.wrapping_add(edi.wrapping_mul(4)));
                    // 00e2bd69: MOV EDX,dword ptr [EBP + 0x14]
                    edx = e.mem.u32(ebp.wrapping_add(0x14));
                    // 00e2bd6c: MOV ECX,EAX
                    ecx = eax;
                    // 00e2bd6e: SHL ECX,0x5
                    ecx = fl.shift_shl(ecx, 0x5_u32, 32);
                    // 00e2bd71: MOVAPS XMM0,xmmword ptr [ECX + EDX*0x1]
                    xmm[0] = load_lanes(e, ecx.wrapping_add(edx));
                    // 00e2bd75: MOVAPS XMM3,xmmword ptr [ECX + EDX*0x1 + 0x10]
                    xmm[3] = load_lanes(e, ecx.wrapping_add(edx).wrapping_add(0x10));
                    // 00e2bd7a: ADD ECX,EDX
                    ecx = fl.add(ecx, edx, 32);
                    // 00e2bd7c: MOVAPS XMM1,XMM5
                    xmm[1] = xmm[5];
                    // 00e2bd7f: SUBPS XMM1,XMM0
                    xmm[1] = subps(xmm[1], xmm[0]);
                    // 00e2bd82: MULPS XMM1,XMM3
                    xmm[1] = mulps(xmm[1], xmm[3]);
                    // 00e2bd85: MOVAPS XMM2,XMM6
                    xmm[2] = xmm[6];
                    // 00e2bd88: SUBPS XMM2,XMM0
                    xmm[2] = subps(xmm[2], xmm[0]);
                    // 00e2bd8b: MULPS XMM2,XMM3
                    xmm[2] = mulps(xmm[2], xmm[3]);
                    // 00e2bd8e: MOVAPS XMM0,XMM1
                    xmm[0] = xmm[1];
                    // 00e2bd91: MINPS XMM0,XMM2
                    xmm[0] = minps(xmm[0], xmm[2]);
                    // 00e2bd94: MAXPS XMM1,XMM2
                    xmm[1] = maxps(xmm[1], xmm[2]);
                    // 00e2bd97: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2bd9a: SHUFPS XMM2,XMM0,0xaa
                    xmm[2] = shufps(xmm[2], xmm[0], 170);
                    // 00e2bd9e: MOVAPS XMM3,XMM0
                    xmm[3] = xmm[0];
                    // 00e2bda1: SHUFPS XMM3,XMM0,0x55
                    xmm[3] = shufps(xmm[3], xmm[0], 85);
                    // 00e2bda5: MAXPS XMM0,xmmword ptr [0x01267e30]
                    xmm[0] = maxps(xmm[0], load_lanes(e, 0x1267e30_u32));
                    // 00e2bdac: MAXPS XMM3,XMM2
                    xmm[3] = maxps(xmm[3], xmm[2]);
                    // 00e2bdaf: MAXPS XMM0,XMM3
                    xmm[0] = maxps(xmm[0], xmm[3]);
                    // 00e2bdb2: MOVAPS XMM2,XMM1
                    xmm[2] = xmm[1];
                    // 00e2bdb5: MOVAPS XMM3,XMM1
                    xmm[3] = xmm[1];
                    // 00e2bdb8: SHUFPS XMM2,XMM1,0xaa
                    xmm[2] = shufps(xmm[2], xmm[1], 170);
                    // 00e2bdbc: SHUFPS XMM3,XMM1,0x55
                    xmm[3] = shufps(xmm[3], xmm[1], 85);
                    // 00e2bdc0: MINPS XMM3,XMM2
                    xmm[3] = minps(xmm[3], xmm[2]);
                    // 00e2bdc3: MINPS XMM1,XMM4
                    xmm[1] = minps(xmm[1], xmm[4]);
                    // 00e2bdc6: MINPS XMM1,XMM3
                    xmm[1] = minps(xmm[1], xmm[3]);
                    // 00e2bdc9: CMPLTPS XMM0,XMM1
                    xmm[0] = cmpltps(xmm[0], xmm[1]);
                    // 00e2bdcd: MOVMSKPS EDX,XMM0
                    edx = movmskps(xmm[0]);
                    // 00e2bdd0: TEST DL,0x1
                    fl.and(edx & 0xff, 0x1_u32, 8);
                    // 00e2bdd3: JZ 0x00e2bddf
                    if fl.zf {
                        pc = 0x00e2_bddf;
                        continue;
                    }
                    pc = 0x00e2_bdd5;
                }
                0x00e2_bdd5 => {
                    // 00e2bdd5: MOV EDX,dword ptr [EBP + 0x44]
                    edx = e.mem.u32(ebp.wrapping_add(0x44));
                    // 00e2bdd8: MOV dword ptr [EBX],EAX
                    e.mem.set_u32(ebx, eax);
                    // 00e2bdda: ADD EBX,0x4
                    ebx = fl.add(ebx, 0x4_u32, 32);
                    // 00e2bddd: INC dword ptr [EDX]
                    e.mem.set_u32(edx, fl.inc(e.mem.u32(edx), 32));
                    pc = 0x00e2_bddf;
                }
                0x00e2_bddf => {
                    // 00e2bddf: MOVAPS XMM0,xmmword ptr [ECX]
                    xmm[0] = load_lanes(e, ecx);
                    // 00e2bde2: MOVAPS XMM3,xmmword ptr [ECX + 0x10]
                    xmm[3] = load_lanes(e, ecx.wrapping_add(0x10));
                    // 00e2bde6: MOVAPS XMM2,xmmword ptr [ESP + 0x60]
                    xmm[2] = load_lanes(e, esp.wrapping_add(0x60));
                    // 00e2bdeb: SUBPS XMM2,XMM0
                    xmm[2] = subps(xmm[2], xmm[0]);
                    // 00e2bdee: MULPS XMM2,XMM3
                    xmm[2] = mulps(xmm[2], xmm[3]);
                    // 00e2bdf1: MOVAPS XMM1,XMM7
                    xmm[1] = xmm[7];
                    // 00e2bdf4: SUBPS XMM1,XMM0
                    xmm[1] = subps(xmm[1], xmm[0]);
                    // 00e2bdf7: MULPS XMM1,XMM3
                    xmm[1] = mulps(xmm[1], xmm[3]);
                    // 00e2bdfa: MOVAPS XMM0,XMM1
                    xmm[0] = xmm[1];
                    // 00e2bdfd: MINPS XMM0,XMM2
                    xmm[0] = minps(xmm[0], xmm[2]);
                    // 00e2be00: MAXPS XMM1,XMM2
                    xmm[1] = maxps(xmm[1], xmm[2]);
                    // 00e2be03: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2be06: SHUFPS XMM2,XMM0,0xaa
                    xmm[2] = shufps(xmm[2], xmm[0], 170);
                    // 00e2be0a: MOVAPS XMM3,XMM0
                    xmm[3] = xmm[0];
                    // 00e2be0d: SHUFPS XMM3,XMM0,0x55
                    xmm[3] = shufps(xmm[3], xmm[0], 85);
                    // 00e2be11: MAXPS XMM0,xmmword ptr [0x01267e30]
                    xmm[0] = maxps(xmm[0], load_lanes(e, 0x1267e30_u32));
                    // 00e2be18: MAXPS XMM3,XMM2
                    xmm[3] = maxps(xmm[3], xmm[2]);
                    // 00e2be1b: MAXPS XMM0,XMM3
                    xmm[0] = maxps(xmm[0], xmm[3]);
                    // 00e2be1e: MOVAPS XMM2,XMM1
                    xmm[2] = xmm[1];
                    // 00e2be21: MOVAPS XMM3,XMM1
                    xmm[3] = xmm[1];
                    // 00e2be24: SHUFPS XMM2,XMM1,0xaa
                    xmm[2] = shufps(xmm[2], xmm[1], 170);
                    // 00e2be28: SHUFPS XMM3,XMM1,0x55
                    xmm[3] = shufps(xmm[3], xmm[1], 85);
                    // 00e2be2c: MINPS XMM3,XMM2
                    xmm[3] = minps(xmm[3], xmm[2]);
                    // 00e2be2f: MINPS XMM1,XMM4
                    xmm[1] = minps(xmm[1], xmm[4]);
                    // 00e2be32: MINPS XMM1,XMM3
                    xmm[1] = minps(xmm[1], xmm[3]);
                    // 00e2be35: CMPLTPS XMM0,XMM1
                    xmm[0] = cmpltps(xmm[0], xmm[1]);
                    // 00e2be39: MOVMSKPS ECX,XMM0
                    ecx = movmskps(xmm[0]);
                    // 00e2be3c: TEST CL,0x1
                    fl.and(ecx & 0xff, 0x1_u32, 8);
                    // 00e2be3f: JZ 0x00e2be4b
                    if fl.zf {
                        pc = 0x00e2_be4b;
                        continue;
                    }
                    pc = 0x00e2_be41;
                }
                0x00e2_be41 => {
                    // 00e2be41: MOV dword ptr [ESI],EAX
                    e.mem.set_u32(esi, eax);
                    // 00e2be43: MOV EAX,dword ptr [EBP + 0x4c]
                    eax = e.mem.u32(ebp.wrapping_add(0x4c));
                    // 00e2be46: ADD ESI,0x4
                    esi = fl.add(esi, 0x4_u32, 32);
                    // 00e2be49: INC dword ptr [EAX]
                    e.mem.set_u32(eax, fl.inc(e.mem.u32(eax), 32));
                    pc = 0x00e2_be4b;
                }
                0x00e2_be4b => {
                    // 00e2be4b: INC EDI
                    edi = fl.inc(edi, 32);
                    // 00e2be4c: CMP EDI,dword ptr [EBP + 0x30]
                    fl.sub(edi, e.mem.u32(ebp.wrapping_add(0x30)), 32);
                    // 00e2be4f: JL 0x00e2bd63
                    if fl.sf != fl.of {
                        pc = 0x00e2_bd63;
                        continue;
                    }
                    pc = 0x00e2_be55;
                }
                0x00e2_be55 => {
                    // 00e2be55: POP EDI
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2be56: POP ESI
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2be57: POP EBX
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2be58: MOV ESP,EBP
                    esp = ebp;
                    // 00e2be5a: POP EBP
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2be5b: RET
                    return;
                }
                0x00e2_be5c => {
                    // 00e2be5c: LEA EAX,[ECX + 0x3]
                    eax = ecx.wrapping_add(0x3);
                    // 00e2be5f: SAR EAX,0x2
                    eax = fl.shift_sar(eax, 0x2_u32, 32);
                    // 00e2be62: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2be64: MOV dword ptr [ESP + 0x24],ESI
                    e.mem.set_u32(esp.wrapping_add(0x24), esi);
                    // 00e2be68: MOV dword ptr [ESP + 0x28],ESI
                    e.mem.set_u32(esp.wrapping_add(0x28), esi);
                    // 00e2be6c: MOV dword ptr [ESP + 0x2c],ESI
                    e.mem.set_u32(esp.wrapping_add(0x2c), esi);
                    // 00e2be70: MOV dword ptr [ESP + 0x30],ESI
                    e.mem.set_u32(esp.wrapping_add(0x30), esi);
                    // 00e2be74: JLE 0x00e2c0ef
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c0ef;
                        continue;
                    }
                    pc = 0x00e2_be7a;
                }
                0x00e2_be7a => {
                    // 00e2be7a: MOV EDX,dword ptr [EBP + 0x2c]
                    edx = e.mem.u32(ebp.wrapping_add(0x2c));
                    // 00e2be7d: MOV EBX,dword ptr [EBP + 0x18]
                    ebx = e.mem.u32(ebp.wrapping_add(0x18));
                    // 00e2be80: MOV EDI,ECX
                    edi = ecx;
                    // 00e2be82: MOV dword ptr [ESP + 0x20],EDX
                    e.mem.set_u32(esp.wrapping_add(0x20), edx);
                    // 00e2be86: MOV dword ptr [ESP + 0x38],EDI
                    e.mem.set_u32(esp.wrapping_add(0x38), edi);
                    // 00e2be8a: MOV dword ptr [ESP + 0x3c],EAX
                    e.mem.set_u32(esp.wrapping_add(0x3c), eax);
                    // 00e2be8e: JMP 0x00e2be92
                    pc = 0x00e2_be92;
                    continue;
                }
                0x00e2_be90 => {
                    // 00e2be90: XOR ESI,ESI
                    esi = fl.xor(esi, esi, 32);
                    pc = 0x00e2_be92;
                }
                0x00e2_be92 => {
                    // 00e2be92: CMP EDI,0x4
                    fl.sub(edi, 0x4_u32, 32);
                    // 00e2be95: JGE 0x00e2be9d
                    if fl.sf == fl.of {
                        pc = 0x00e2_be9d;
                        continue;
                    }
                    pc = 0x00e2_be97;
                }
                0x00e2_be97 => {
                    // 00e2be97: MOV dword ptr [ESP + 0x18],EDI
                    e.mem.set_u32(esp.wrapping_add(0x18), edi);
                    // 00e2be9b: JMP 0x00e2bea9
                    pc = 0x00e2_bea9;
                    continue;
                }
                0x00e2_be9d => {
                    // 00e2be9d: MOV dword ptr [ESP + 0x18],0x4
                    e.mem.set_u32(esp.wrapping_add(0x18), 0x4_u32);
                    // 00e2bea5: MOV EDI,dword ptr [ESP + 0x18]
                    edi = e.mem.u32(esp.wrapping_add(0x18));
                    pc = 0x00e2_bea9;
                }
                0x00e2_bea9 => {
                    // 00e2bea9: CMP EDI,ESI
                    fl.sub(edi, esi, 32);
                    // 00e2beab: MOV dword ptr [ESP + 0x34],ESI
                    e.mem.set_u32(esp.wrapping_add(0x34), esi);
                    // 00e2beaf: JLE 0x00e2bf7f
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_bf7f;
                        continue;
                    }
                    pc = 0x00e2_beb5;
                }
                0x00e2_beb5 => {
                    // 00e2beb5: MOV ESI,dword ptr [EBP + 0x1c]
                    esi = e.mem.u32(ebp.wrapping_add(0x1c));
                    // 00e2beb8: MOV EAX,dword ptr [ESP + 0x20]
                    eax = e.mem.u32(esp.wrapping_add(0x20));
                    // 00e2bebc: ADD EBX,0x60
                    ebx = fl.add(ebx, 0x60_u32, 32);
                    // 00e2bebf: ADD ESI,0x8
                    esi = fl.add(esi, 0x8_u32, 32);
                    // 00e2bec2: MOV dword ptr [ESP + 0x1c],EAX
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax);
                    pc = 0x00e2_bec6;
                }
                0x00e2_bec6 => {
                    // 00e2bec6: MOV ECX,dword ptr [ESP + 0x1c]
                    ecx = e.mem.u32(esp.wrapping_add(0x1c));
                    // 00e2beca: MOV EAX,dword ptr [ECX]
                    eax = e.mem.u32(ecx);
                    // 00e2becc: MOV EDX,dword ptr [ESP + 0x34]
                    edx = e.mem.u32(esp.wrapping_add(0x34));
                    // 00e2bed0: MOV dword ptr [ESP + EDX*0x4 + 0x24],EAX
                    e.mem.set_u32(
                        esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x24),
                        eax,
                    );
                    // 00e2bed4: SHL EAX,0x6
                    eax = fl.shift_shl(eax, 0x6_u32, 32);
                    // 00e2bed7: ADD EAX,dword ptr [EBP + 0x20]
                    eax = fl.add(eax, e.mem.u32(ebp.wrapping_add(0x20)), 32);
                    // 00e2beda: LEA ECX,[EBX + -0x60]
                    ecx = ebx.wrapping_sub(0x60);
                    // 00e2bedd: MOV EDI,EAX
                    edi = eax;
                    // 00e2bedf: MOV EAX,dword ptr [EBP + 0x24]
                    eax = e.mem.u32(ebp.wrapping_add(0x24));
                    // 00e2bee2: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2bee3: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2bee4: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2bee5: CALL 0x00e2baa0
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                    ];
                    let result = e.call(0x00e2baa0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2beea: MOV ECX,dword ptr [ESP + 0x40]
                    ecx = e.mem.u32(esp.wrapping_add(0x40));
                    // 00e2beee: MOV EDX,dword ptr [ESP + ECX*0x4 + 0x30]
                    edx = e
                        .mem
                        .u32(esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x30));
                    // 00e2bef2: MOV EAX,dword ptr [EBP + 0x28]
                    eax = e.mem.u32(ebp.wrapping_add(0x28));
                    // 00e2bef5: MOVSS XMM0,dword ptr [EAX + EDX*0x4]
                    xmm[0] = [e.mem.u32(eax.wrapping_add(edx.wrapping_mul(4))), 0, 0, 0];
                    // 00e2befa: SHUFPS XMM0,XMM0,0x0
                    xmm[0] = shufps(xmm[0], xmm[0], 0);
                    // 00e2befe: MOVAPS xmmword ptr [EBX + 0x10],XMM0
                    store_lanes(e, ebx.wrapping_add(0x10), xmm[0]);
                    // 00e2bf02: FLD float ptr [EAX + EDX*0x4]
                    st.push(x87_load32(e.mem.u32(eax.wrapping_add(edx.wrapping_mul(4)))));
                    // 00e2bf05: FSTP float ptr [ESP + ECX*0x4 + 0x5c]
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x5c),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2bf09: ADD ESP,0xc
                    esp = fl.add(esp, 0xc_u32, 32);
                    // 00e2bf0c: CMP byte ptr [EDI + 0x3c],0x0
                    fl.sub(u32::from(e.mem.u8(edi.wrapping_add(0x3c))), 0x0_u32, 8);
                    // 00e2bf10: JZ 0x00e2bf5b
                    if fl.zf {
                        pc = 0x00e2_bf5b;
                        continue;
                    }
                    pc = 0x00e2_bf12;
                }
                0x00e2_bf12 => {
                    // 00e2bf12: LEA EAX,[ESI + -0x8]
                    eax = esi.wrapping_sub(0x8);
                    // 00e2bf15: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2bf17: JZ 0x00e2bf4b
                    if fl.zf {
                        pc = 0x00e2_bf4b;
                        continue;
                    }
                    pc = 0x00e2_bf19;
                }
                0x00e2_bf19 => {
                    // 00e2bf19: MOV EDX,dword ptr [EDI + 0x34]
                    edx = e.mem.u32(edi.wrapping_add(0x34));
                    // 00e2bf1c: FLD1
                    st.push(1.0);
                    // 00e2bf1e: MOV EDI,dword ptr [EDI + 0x30]
                    edi = e.mem.u32(edi.wrapping_add(0x30));
                    // 00e2bf21: FSTP float ptr [ESI + -0x4]
                    e.mem
                        .set_u32(esi.wrapping_sub(0x4), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2bf24: MOV dword ptr [ESI + 0x8],EDX
                    e.mem.set_u32(esi.wrapping_add(0x8), edx);
                    // 00e2bf27: LEA EAX,[ESI + -0x8]
                    eax = esi.wrapping_sub(0x8);
                    // 00e2bf2a: XOR EDX,EDX
                    edx = fl.xor(edx, edx, 32);
                    // 00e2bf2c: MOV dword ptr [EAX],0x10dbb04
                    e.mem.set_u32(eax, 0x10dbb04_u32);
                    // 00e2bf32: MOV dword ptr [ESI],EDI
                    e.mem.set_u32(esi, edi);
                    // 00e2bf34: MOV dword ptr [ESI + 0x4],EDI
                    e.mem.set_u32(esi.wrapping_add(0x4), edi);
                    // 00e2bf37: MOV dword ptr [ESI + 0xc],EDX
                    e.mem.set_u32(esi.wrapping_add(0xc), edx);
                    // 00e2bf3a: MOV dword ptr [ESI + 0x10],EDX
                    e.mem.set_u32(esi.wrapping_add(0x10), edx);
                    // 00e2bf3d: MOV dword ptr [EBX],EAX
                    e.mem.set_u32(ebx, eax);
                    // 00e2bf3f: MOV dword ptr [0x0126ec04],0xe2b9b0
                    e.mem.set_u32(0x126ec04_u32, 0xe2b9b0_u32);
                    // 00e2bf49: JMP 0x00e2bf61
                    pc = 0x00e2_bf61;
                    continue;
                }
                0x00e2_bf4b => {
                    // 00e2bf4b: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    // 00e2bf4d: MOV dword ptr [EBX],EAX
                    e.mem.set_u32(ebx, eax);
                    // 00e2bf4f: MOV dword ptr [0x0126ec04],0xe2b9b0
                    e.mem.set_u32(0x126ec04_u32, 0xe2b9b0_u32);
                    // 00e2bf59: JMP 0x00e2bf61
                    pc = 0x00e2_bf61;
                    continue;
                }
                0x00e2_bf5b => {
                    // 00e2bf5b: MOV dword ptr [EBX],0x0
                    e.mem.set_u32(ebx, 0x0_u32);
                    pc = 0x00e2_bf61;
                }
                0x00e2_bf61 => {
                    // 00e2bf61: ADD dword ptr [ESP + 0x1c],0x4
                    e.mem.set_u32(
                        esp.wrapping_add(0x1c),
                        fl.add(e.mem.u32(esp.wrapping_add(0x1c)), 0x4_u32, 32),
                    );
                    // 00e2bf66: INC ECX
                    ecx = fl.inc(ecx, 32);
                    // 00e2bf67: ADD ESI,0x1c
                    esi = fl.add(esi, 0x1c_u32, 32);
                    // 00e2bf6a: SUB EBX,-0x80
                    ebx = fl.sub(ebx, 0xffffff80_u32, 32);
                    // 00e2bf6d: CMP ECX,dword ptr [ESP + 0x18]
                    fl.sub(ecx, e.mem.u32(esp.wrapping_add(0x18)), 32);
                    // 00e2bf71: MOV dword ptr [ESP + 0x34],ECX
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx);
                    // 00e2bf75: JL 0x00e2bec6
                    if fl.sf != fl.of {
                        pc = 0x00e2_bec6;
                        continue;
                    }
                    pc = 0x00e2_bf7b;
                }
                0x00e2_bf7b => {
                    // 00e2bf7b: MOV EDI,dword ptr [ESP + 0x18]
                    edi = e.mem.u32(esp.wrapping_add(0x18));
                    pc = 0x00e2_bf7f;
                }
                0x00e2_bf7f => {
                    // 00e2bf7f: MOV EAX,dword ptr [EBP + 0x14]
                    eax = e.mem.u32(ebp.wrapping_add(0x14));
                    // 00e2bf82: MOV ECX,dword ptr [ESP + 0x30]
                    ecx = e.mem.u32(esp.wrapping_add(0x30));
                    // 00e2bf86: MOV EDX,dword ptr [ESP + 0x2c]
                    edx = e.mem.u32(esp.wrapping_add(0x2c));
                    // 00e2bf8a: MOV ESI,dword ptr [ESP + 0x28]
                    esi = e.mem.u32(esp.wrapping_add(0x28));
                    // 00e2bf8e: MOV EBX,dword ptr [ESP + 0x24]
                    ebx = e.mem.u32(esp.wrapping_add(0x24));
                    // 00e2bf92: SHL ECX,0x5
                    ecx = fl.shift_shl(ecx, 0x5_u32, 32);
                    // 00e2bf95: ADD ECX,EAX
                    ecx = fl.add(ecx, eax, 32);
                    // 00e2bf97: SHL EDX,0x5
                    edx = fl.shift_shl(edx, 0x5_u32, 32);
                    // 00e2bf9a: ADD EDX,EAX
                    edx = fl.add(edx, eax, 32);
                    // 00e2bf9c: SHL EBX,0x5
                    ebx = fl.shift_shl(ebx, 0x5_u32, 32);
                    // 00e2bf9f: SHL ESI,0x5
                    esi = fl.shift_shl(esi, 0x5_u32, 32);
                    // 00e2bfa2: ADD ESI,EAX
                    esi = fl.add(esi, eax, 32);
                    // 00e2bfa4: ADD EAX,EBX
                    eax = fl.add(eax, ebx, 32);
                    // 00e2bfa6: MOV EBX,dword ptr [EDI*0x4 + 0x11b3830]
                    ebx = e.mem.u32(edi.wrapping_mul(4).wrapping_add(0x11b3830));
                    // 00e2bfad: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2bfae: LEA EBX,[ECX + 0x10]
                    ebx = ecx.wrapping_add(0x10);
                    // 00e2bfb1: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2bfb2: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2bfb3: LEA ECX,[EDX + 0x10]
                    ecx = edx.wrapping_add(0x10);
                    // 00e2bfb6: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2bfb7: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2bfb8: LEA EDX,[ESI + 0x10]
                    edx = esi.wrapping_add(0x10);
                    // 00e2bfbb: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2bfbc: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2bfbd: LEA ECX,[EAX + 0x10]
                    ecx = eax.wrapping_add(0x10);
                    // 00e2bfc0: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2bfc1: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2bfc2: LEA EDX,[ESP + 0x94]
                    edx = esp.wrapping_add(0x94);
                    // 00e2bfc9: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2bfca: CALL 0x00e2baf0
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                        e.mem.u32(esp.wrapping_add(12)),
                        e.mem.u32(esp.wrapping_add(16)),
                        e.mem.u32(esp.wrapping_add(20)),
                        e.mem.u32(esp.wrapping_add(24)),
                        e.mem.u32(esp.wrapping_add(28)),
                        e.mem.u32(esp.wrapping_add(32)),
                        e.mem.u32(esp.wrapping_add(36)),
                    ];
                    let result = e.call(0x00e2baf0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2bfcf: MOV EBX,dword ptr [EBP + 0x18]
                    ebx = e.mem.u32(ebp.wrapping_add(0x18));
                    // 00e2bfd2: XOR ESI,ESI
                    esi = fl.xor(esi, esi, 32);
                    // 00e2bfd4: ADD ESP,0x28
                    esp = fl.add(esp, 0x28_u32, 32);
                    // 00e2bfd7: CMP EBX,ESI
                    fl.sub(ebx, esi, 32);
                    // 00e2bfd9: JZ 0x00e2bfe4
                    if fl.zf {
                        pc = 0x00e2_bfe4;
                        continue;
                    }
                    pc = 0x00e2_bfdb;
                }
                0x00e2_bfdb => {
                    // 00e2bfdb: LEA EAX,[EBX + 0x8]
                    eax = ebx.wrapping_add(0x8);
                    // 00e2bfde: MOV dword ptr [ESP + 0x40],EAX
                    e.mem.set_u32(esp.wrapping_add(0x40), eax);
                    // 00e2bfe2: JMP 0x00e2bfe8
                    pc = 0x00e2_bfe8;
                    continue;
                }
                0x00e2_bfe4 => {
                    // 00e2bfe4: MOV dword ptr [ESP + 0x40],ESI
                    e.mem.set_u32(esp.wrapping_add(0x40), esi);
                    pc = 0x00e2_bfe8;
                }
                0x00e2_bfe8 => {
                    // 00e2bfe8: LEA EAX,[EBX + 0x80]
                    eax = ebx.wrapping_add(0x80);
                    // 00e2bfee: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2bff0: JZ 0x00e2bffe
                    if fl.zf {
                        pc = 0x00e2_bffe;
                        continue;
                    }
                    pc = 0x00e2_bff2;
                }
                0x00e2_bff2 => {
                    // 00e2bff2: LEA ECX,[EBX + 0x88]
                    ecx = ebx.wrapping_add(0x88);
                    // 00e2bff8: MOV dword ptr [ESP + 0x44],ECX
                    e.mem.set_u32(esp.wrapping_add(0x44), ecx);
                    // 00e2bffc: JMP 0x00e2c002
                    pc = 0x00e2_c002;
                    continue;
                }
                0x00e2_bffe => {
                    // 00e2bffe: MOV dword ptr [ESP + 0x44],ESI
                    e.mem.set_u32(esp.wrapping_add(0x44), esi);
                    pc = 0x00e2_c002;
                }
                0x00e2_c002 => {
                    // 00e2c002: LEA EAX,[EBX + 0x100]
                    eax = ebx.wrapping_add(0x100);
                    // 00e2c008: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2c00a: JZ 0x00e2c018
                    if fl.zf {
                        pc = 0x00e2_c018;
                        continue;
                    }
                    pc = 0x00e2_c00c;
                }
                0x00e2_c00c => {
                    // 00e2c00c: LEA EDX,[EBX + 0x108]
                    edx = ebx.wrapping_add(0x108);
                    // 00e2c012: MOV dword ptr [ESP + 0x48],EDX
                    e.mem.set_u32(esp.wrapping_add(0x48), edx);
                    // 00e2c016: JMP 0x00e2c01c
                    pc = 0x00e2_c01c;
                    continue;
                }
                0x00e2_c018 => {
                    // 00e2c018: MOV dword ptr [ESP + 0x48],ESI
                    e.mem.set_u32(esp.wrapping_add(0x48), esi);
                    pc = 0x00e2_c01c;
                }
                0x00e2_c01c => {
                    // 00e2c01c: LEA EAX,[EBX + 0x180]
                    eax = ebx.wrapping_add(0x180);
                    // 00e2c022: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2c024: JZ 0x00e2c032
                    if fl.zf {
                        pc = 0x00e2_c032;
                        continue;
                    }
                    pc = 0x00e2_c026;
                }
                0x00e2_c026 => {
                    // 00e2c026: LEA EAX,[EBX + 0x188]
                    eax = ebx.wrapping_add(0x188);
                    // 00e2c02c: MOV dword ptr [ESP + 0x4c],EAX
                    e.mem.set_u32(esp.wrapping_add(0x4c), eax);
                    // 00e2c030: JMP 0x00e2c036
                    pc = 0x00e2_c036;
                    continue;
                }
                0x00e2_c032 => {
                    // 00e2c032: MOV dword ptr [ESP + 0x4c],ESI
                    e.mem.set_u32(esp.wrapping_add(0x4c), esi);
                    pc = 0x00e2_c036;
                }
                0x00e2_c036 => {
                    // 00e2c036: LEA ECX,[ESP + 0x50]
                    ecx = esp.wrapping_add(0x50);
                    // 00e2c03a: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2c03b: MOV ECX,dword ptr [EBP + 0x10]
                    ecx = e.mem.u32(ebp.wrapping_add(0x10));
                    // 00e2c03e: LEA EDX,[ESP + 0x44]
                    edx = esp.wrapping_add(0x44);
                    // 00e2c042: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2c043: MOV EDX,dword ptr [EBP + 0xc]
                    edx = e.mem.u32(ebp.wrapping_add(0xc));
                    // 00e2c046: LEA EAX,[ESP + 0x78]
                    eax = esp.wrapping_add(0x78);
                    // 00e2c04a: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2c04b: MOV EAX,dword ptr [EBP + 0x8]
                    eax = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2c04e: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2c04f: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2c050: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2c051: CALL 0x00d1ca00
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                        e.mem.u32(esp.wrapping_add(12)),
                        e.mem.u32(esp.wrapping_add(16)),
                        e.mem.u32(esp.wrapping_add(20)),
                    ];
                    let result = e.call(0x00d1ca00, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2c056: ADD ESP,0x18
                    esp = fl.add(esp, 0x18_u32, 32);
                    // 00e2c059: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    // 00e2c05b: CMP EDI,0x4
                    fl.sub(edi, 0x4_u32, 32);
                    // 00e2c05e: JL 0x00e2c0b0
                    if fl.sf != fl.of {
                        pc = 0x00e2_c0b0;
                        continue;
                    }
                    pc = 0x00e2_c060;
                }
                0x00e2_c060 => {
                    // 00e2c060: LEA ESI,[EDI + -0x3]
                    esi = edi.wrapping_sub(0x3);
                    // 00e2c063: LEA ECX,[EBX + 0xf0]
                    ecx = ebx.wrapping_add(0xf0);
                    // 00e2c069: LEA ESP,[ESP]
                    // (no operation)
                    pc = 0x00e2_c070;
                }
                0x00e2_c070 => {
                    // 00e2c070: FLD float ptr [ECX + -0x80]
                    st.push(x87_load32(e.mem.u32(ecx.wrapping_sub(0x80))));
                    // 00e2c073: MOV EDX,dword ptr [EBP + 0x28]
                    edx = e.mem.u32(ebp.wrapping_add(0x28));
                    // 00e2c076: MOV EBX,dword ptr [ESP + EAX*0x4 + 0x24]
                    ebx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x24));
                    // 00e2c07a: FSTP float ptr [EDX + EBX*0x4]
                    e.mem.set_u32(
                        edx.wrapping_add(ebx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c07d: MOV EBX,dword ptr [ESP + EAX*0x4 + 0x28]
                    ebx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x28));
                    // 00e2c081: FLD float ptr [ECX]
                    st.push(x87_load32(e.mem.u32(ecx)));
                    // 00e2c083: ADD EAX,0x4
                    eax = fl.add(eax, 0x4_u32, 32);
                    // 00e2c086: FSTP float ptr [EDX + EBX*0x4]
                    e.mem.set_u32(
                        edx.wrapping_add(ebx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c089: MOV EBX,dword ptr [ESP + EAX*0x4 + 0x1c]
                    ebx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x1c));
                    // 00e2c08d: FLD float ptr [ECX + 0x80]
                    st.push(x87_load32(e.mem.u32(ecx.wrapping_add(0x80))));
                    // 00e2c093: ADD ECX,0x200
                    ecx = fl.add(ecx, 0x200_u32, 32);
                    // 00e2c099: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2c09b: FSTP float ptr [EDX + EBX*0x4]
                    e.mem.set_u32(
                        edx.wrapping_add(ebx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c09e: MOV EBX,dword ptr [ESP + EAX*0x4 + 0x20]
                    ebx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x20));
                    // 00e2c0a2: FLD float ptr [ECX + 0xffffff00]
                    st.push(x87_load32(e.mem.u32(ecx.wrapping_add(0xffffff00))));
                    // 00e2c0a8: FSTP float ptr [EDX + EBX*0x4]
                    e.mem.set_u32(
                        edx.wrapping_add(ebx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c0ab: JL 0x00e2c070
                    if fl.sf != fl.of {
                        pc = 0x00e2_c070;
                        continue;
                    }
                    pc = 0x00e2_c0ad;
                }
                0x00e2_c0ad => {
                    // 00e2c0ad: MOV EBX,dword ptr [EBP + 0x18]
                    ebx = e.mem.u32(ebp.wrapping_add(0x18));
                    pc = 0x00e2_c0b0;
                }
                0x00e2_c0b0 => {
                    // 00e2c0b0: CMP EAX,EDI
                    fl.sub(eax, edi, 32);
                    // 00e2c0b2: JGE 0x00e2c0d4
                    if fl.sf == fl.of {
                        pc = 0x00e2_c0d4;
                        continue;
                    }
                    pc = 0x00e2_c0b4;
                }
                0x00e2_c0b4 => {
                    // 00e2c0b4: MOV ECX,EAX
                    ecx = eax;
                    // 00e2c0b6: SHL ECX,0x7
                    ecx = fl.shift_shl(ecx, 0x7_u32, 32);
                    // 00e2c0b9: LEA ECX,[ECX + EBX*0x1 + 0x70]
                    ecx = ecx.wrapping_add(ebx).wrapping_add(0x70);
                    // 00e2c0bd: LEA ECX,[ECX]
                    // (no operation)
                    pc = 0x00e2_c0c0;
                }
                0x00e2_c0c0 => {
                    // 00e2c0c0: MOV EDX,dword ptr [ESP + EAX*0x4 + 0x24]
                    edx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x24));
                    // 00e2c0c4: FLD float ptr [ECX]
                    st.push(x87_load32(e.mem.u32(ecx)));
                    // 00e2c0c6: MOV ESI,dword ptr [EBP + 0x28]
                    esi = e.mem.u32(ebp.wrapping_add(0x28));
                    // 00e2c0c9: INC EAX
                    eax = fl.inc(eax, 32);
                    // 00e2c0ca: FSTP float ptr [ESI + EDX*0x4]
                    e.mem.set_u32(
                        esi.wrapping_add(edx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c0cd: SUB ECX,-0x80
                    ecx = fl.sub(ecx, 0xffffff80_u32, 32);
                    // 00e2c0d0: CMP EAX,EDI
                    fl.sub(eax, edi, 32);
                    // 00e2c0d2: JL 0x00e2c0c0
                    if fl.sf != fl.of {
                        pc = 0x00e2_c0c0;
                        continue;
                    }
                    pc = 0x00e2_c0d4;
                }
                0x00e2_c0d4 => {
                    // 00e2c0d4: MOV EDI,dword ptr [ESP + 0x38]
                    edi = e.mem.u32(esp.wrapping_add(0x38));
                    // 00e2c0d8: ADD dword ptr [ESP + 0x20],0x10
                    e.mem.set_u32(
                        esp.wrapping_add(0x20),
                        fl.add(e.mem.u32(esp.wrapping_add(0x20)), 0x10_u32, 32),
                    );
                    // 00e2c0dd: SUB EDI,0x4
                    edi = fl.sub(edi, 0x4_u32, 32);
                    // 00e2c0e0: SUB dword ptr [ESP + 0x3c],0x1
                    e.mem.set_u32(
                        esp.wrapping_add(0x3c),
                        fl.sub(e.mem.u32(esp.wrapping_add(0x3c)), 0x1_u32, 32),
                    );
                    // 00e2c0e5: MOV dword ptr [ESP + 0x38],EDI
                    e.mem.set_u32(esp.wrapping_add(0x38), edi);
                    // 00e2c0e9: JNZ 0x00e2be90
                    if !fl.zf {
                        pc = 0x00e2_be90;
                        continue;
                    }
                    pc = 0x00e2_c0ef;
                }
                0x00e2_c0ef => {
                    // 00e2c0ef: POP EDI
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c0f0: POP ESI
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c0f1: POP EBX
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c0f2: MOV ESP,EBP
                    esp = ebp;
                    // 00e2c0f4: POP EBP
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c0f5: RET
                    return;
                }
                _ => unreachable!("pc {pc:#x}"),
            }
        }
    });
}

// Translated from 00e2c100 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `00e2bbd0` for rays given as 16-bit coordinates (the classes are
/// unconfirmed; `00e2cc30` drives it). The same twenty stack words (`arg0` is now the
/// transform: `+0x70` a scale vector and `+0x40` an offset vector), cdecl, nothing popped. The
/// flag is bit `0x8000` of the word at `arg1 + 0x10`; the ray block holds six 16-bit values
/// (`+0`, `+2`, `+4` and `+6`, `+8`, `+0xa`) that are shifted up by 15, turned into floats
/// (`FILD`/`FSTP`) and made into two vectors scaled by `[arg0 + 0x70]` less `[arg0 + 0x40]`; the
/// collector's virtual method is at `arg4 + 0xc`; the long-list path asks `00d9a4d0` (not
/// `00d1bb40`) for the ray blocks; the short-list path ends in `00d1bfa0` (not `00d1ca00`).
/// The slab test, the two-ray path and the output counts are as in `00e2bbd0`. Written instruction
/// by instruction like `00e2bbd0`.
#[allow(
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_labels,
    clippy::all
)]
pub fn fn_00e2c100(
    e: &mut Engine,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    arg7: u32,
    arg8: u32,
    arg9: u32,
    arg10: u32,
    arg11: u32,
    arg12: u32,
    arg13: u32,
    arg14: u32,
    arg15: u32,
    arg16: u32,
    arg17: u32,
    arg18: u32,
    arg19: u32,
) {
    e.with_stack(0x1400, |e, frame| {
        // The exe's registers; `esp` points at the return address slot, the arguments above it.
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut esp = frame.addr() + 0x1400 - 0x80;
        e.mem.set_u32(esp, 0);
        e.mem.set_u32(esp + 4, arg0);
        e.mem.set_u32(esp + 8, arg1);
        e.mem.set_u32(esp + 12, arg2);
        e.mem.set_u32(esp + 16, arg3);
        e.mem.set_u32(esp + 20, arg4);
        e.mem.set_u32(esp + 24, arg5);
        e.mem.set_u32(esp + 28, arg6);
        e.mem.set_u32(esp + 32, arg7);
        e.mem.set_u32(esp + 36, arg8);
        e.mem.set_u32(esp + 40, arg9);
        e.mem.set_u32(esp + 44, arg10);
        e.mem.set_u32(esp + 48, arg11);
        e.mem.set_u32(esp + 52, arg12);
        e.mem.set_u32(esp + 56, arg13);
        e.mem.set_u32(esp + 60, arg14);
        e.mem.set_u32(esp + 64, arg15);
        e.mem.set_u32(esp + 68, arg16);
        e.mem.set_u32(esp + 72, arg17);
        e.mem.set_u32(esp + 76, arg18);
        e.mem.set_u32(esp + 80, arg19);
        let mut xmm = [[0u32; 4]; 8];
        let mut fl = Flags::default();
        let mut st: Vec<f64> = Vec::new();
        let mut fsw: u16 = 0;
        let mut pc = 0x00e2_c100_u32;
        loop {
            match pc {
                0x00e2_c100 => {
                    // 00e2c100: PUSH EBP
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp);
                    // 00e2c101: MOV EBP,ESP
                    ebp = esp;
                    // 00e2c103: AND ESP,0xfffffff0
                    esp = fl.and(esp, 0xfffffff0_u32, 32);
                    // 00e2c106: MOV EAX,dword ptr [EBP + 0x4c]
                    eax = e.mem.u32(ebp.wrapping_add(0x4c));
                    // 00e2c109: MOV ECX,dword ptr [EBP + 0x44]
                    ecx = e.mem.u32(ebp.wrapping_add(0x44));
                    // 00e2c10c: SUB ESP,0xd4
                    esp = fl.sub(esp, 0xd4_u32, 32);
                    // 00e2c112: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2c113: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2c114: XOR ESI,ESI
                    esi = fl.xor(esi, esi, 32);
                    // 00e2c116: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2c117: MOV EDI,dword ptr [EBP + 0xc]
                    edi = e.mem.u32(ebp.wrapping_add(0xc));
                    // 00e2c11a: MOV dword ptr [EAX],ESI
                    e.mem.set_u32(eax, esi);
                    // 00e2c11c: MOV EDX,0x8000
                    edx = 0x8000_u32;
                    // 00e2c121: MOV dword ptr [ECX],ESI
                    e.mem.set_u32(ecx, esi);
                    // 00e2c123: TEST word ptr [EDI + 0x10],DX
                    fl.and(
                        u32::from(e.mem.u16(edi.wrapping_add(0x10))),
                        edx & 0xffff,
                        16,
                    );
                    // 00e2c127: JZ 0x00e2c2c8
                    if fl.zf {
                        pc = 0x00e2_c2c8;
                        continue;
                    }
                    pc = 0x00e2_c12d;
                }
                0x00e2_c12d => {
                    // 00e2c12d: MOVZX EAX,word ptr [EDI]
                    eax = u32::from(e.mem.u16(edi));
                    // 00e2c130: MOVZX ECX,word ptr [EDI + 0x2]
                    ecx = u32::from(e.mem.u16(edi.wrapping_add(0x2)));
                    // 00e2c134: MOVZX EDX,word ptr [EDI + 0x4]
                    edx = u32::from(e.mem.u16(edi.wrapping_add(0x4)));
                    // 00e2c138: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c13b: MOVSS XMM3,dword ptr [0x01011d78]
                    xmm[3] = [e.mem.u32(0x1011d78_u32), 0, 0, 0];
                    // 00e2c143: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c147: MOVZX EAX,word ptr [EDI + 0x6]
                    eax = u32::from(e.mem.u16(edi.wrapping_add(0x6)));
                    // 00e2c14b: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c14f: SHL ECX,0xf
                    ecx = fl.shift_shl(ecx, 0xf_u32, 32);
                    // 00e2c152: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx);
                    // 00e2c156: MOVZX ECX,word ptr [EDI + 0x8]
                    ecx = u32::from(e.mem.u16(edi.wrapping_add(0x8)));
                    // 00e2c15a: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c15e: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c162: MOVSS XMM1,dword ptr [ESP + 0x20]
                    xmm[1] = [e.mem.u32(esp.wrapping_add(0x20)), 0, 0, 0];
                    // 00e2c168: SHL EDX,0xf
                    edx = fl.shift_shl(edx, 0xf_u32, 32);
                    // 00e2c16b: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(esp.wrapping_add(0x1c), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c16f: MOVSS XMM0,dword ptr [ESP + 0x1c]
                    xmm[0] = [e.mem.u32(esp.wrapping_add(0x1c)), 0, 0, 0];
                    // 00e2c175: MOV dword ptr [ESP + 0x18],EDX
                    e.mem.set_u32(esp.wrapping_add(0x18), edx);
                    // 00e2c179: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c17d: MOVZX EDX,word ptr [EDI + 0xa]
                    edx = u32::from(e.mem.u16(edi.wrapping_add(0xa)));
                    // 00e2c181: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c184: SHL ECX,0xf
                    ecx = fl.shift_shl(ecx, 0xf_u32, 32);
                    // 00e2c187: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c18b: MOVSS XMM2,dword ptr [ESP + 0x18]
                    xmm[2] = [e.mem.u32(esp.wrapping_add(0x18)), 0, 0, 0];
                    // 00e2c191: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c195: MOV EAX,dword ptr [EBP + 0x8]
                    eax = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2c198: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c19c: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx);
                    // 00e2c1a0: MOVSS XMM4,XMM3
                    xmm[4][0] = xmm[3][0];
                    // 00e2c1a4: SHL EDX,0xf
                    edx = fl.shift_shl(edx, 0xf_u32, 32);
                    // 00e2c1a7: CMP dword ptr [EBP + 0x30],ESI
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x30)), esi, 32);
                    // 00e2c1aa: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c1ae: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c1b2: MOV dword ptr [ESP + 0x18],EDX
                    e.mem.set_u32(esp.wrapping_add(0x18), edx);
                    // 00e2c1b6: UNPCKLPS XMM1,XMM2
                    xmm[1] = unpcklps(xmm[1], xmm[2]);
                    // 00e2c1b9: MOVAPS XMM2,xmmword ptr [EAX + 0x70]
                    xmm[2] = load_lanes(e, eax.wrapping_add(0x70));
                    // 00e2c1bd: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(esp.wrapping_add(0x1c), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c1c1: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c1c5: UNPCKLPS XMM0,XMM4
                    xmm[0] = unpcklps(xmm[0], xmm[4]);
                    // 00e2c1c8: MOVSS XMM4,dword ptr [ESP + 0x1c]
                    xmm[4] = [e.mem.u32(esp.wrapping_add(0x1c)), 0, 0, 0];
                    // 00e2c1ce: UNPCKLPS XMM1,XMM0
                    xmm[1] = unpcklps(xmm[1], xmm[0]);
                    // 00e2c1d1: MOVSS XMM0,dword ptr [ESP + 0x20]
                    xmm[0] = [e.mem.u32(esp.wrapping_add(0x20)), 0, 0, 0];
                    // 00e2c1d7: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c1db: MOVSS XMM5,dword ptr [ESP + 0x18]
                    xmm[5] = [e.mem.u32(esp.wrapping_add(0x18)), 0, 0, 0];
                    // 00e2c1e1: UNPCKLPS XMM4,XMM3
                    xmm[4] = unpcklps(xmm[4], xmm[3]);
                    // 00e2c1e4: UNPCKLPS XMM0,XMM5
                    xmm[0] = unpcklps(xmm[0], xmm[5]);
                    // 00e2c1e7: UNPCKLPS XMM0,XMM4
                    xmm[0] = unpcklps(xmm[0], xmm[4]);
                    // 00e2c1ea: MULPS XMM0,XMM2
                    xmm[0] = mulps(xmm[0], xmm[2]);
                    // 00e2c1ed: MULPS XMM2,XMM1
                    xmm[2] = mulps(xmm[2], xmm[1]);
                    // 00e2c1f0: MOVAPS XMM1,xmmword ptr [EAX + 0x40]
                    xmm[1] = load_lanes(e, eax.wrapping_add(0x40));
                    // 00e2c1f4: SUBPS XMM0,XMM1
                    xmm[0] = subps(xmm[0], xmm[1]);
                    // 00e2c1f7: SUBPS XMM2,XMM1
                    xmm[2] = subps(xmm[2], xmm[1]);
                    // 00e2c1fa: MOVAPS xmmword ptr [ESP + 0x50],XMM0
                    store_lanes(e, esp.wrapping_add(0x50), xmm[0]);
                    // 00e2c1ff: MOVAPS xmmword ptr [ESP + 0x60],XMM2
                    store_lanes(e, esp.wrapping_add(0x60), xmm[2]);
                    // 00e2c204: JLE 0x00e2c88f
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c88f;
                        continue;
                    }
                    pc = 0x00e2_c20a;
                }
                0x00e2_c20a => {
                    // 00e2c20a: MOVSS XMM4,dword ptr [0x01012068]
                    xmm[4] = [e.mem.u32(0x1012068_u32), 0, 0, 0];
                    // 00e2c212: MOV EBX,dword ptr [EBP + 0x18]
                    ebx = e.mem.u32(ebp.wrapping_add(0x18));
                    // 00e2c215: SHUFPS XMM4,XMM4,0x0
                    xmm[4] = shufps(xmm[4], xmm[4], 0);
                    // 00e2c219: MOVAPS xmmword ptr [ESP + 0x40],XMM4
                    store_lanes(e, esp.wrapping_add(0x40), xmm[4]);
                    pc = 0x00e2_c21e;
                }
                0x00e2_c21e => {
                    // 00e2c21e: MOV EAX,dword ptr [EBP + 0x2c]
                    eax = e.mem.u32(ebp.wrapping_add(0x2c));
                    // 00e2c221: MOV EAX,dword ptr [EAX + ESI*0x4]
                    eax = e.mem.u32(eax.wrapping_add(esi.wrapping_mul(4)));
                    // 00e2c224: MOVAPS XMM1,xmmword ptr [ESP + 0x50]
                    xmm[1] = load_lanes(e, esp.wrapping_add(0x50));
                    // 00e2c229: MOVAPS XMM2,xmmword ptr [ESP + 0x60]
                    xmm[2] = load_lanes(e, esp.wrapping_add(0x60));
                    // 00e2c22e: MOV ECX,EAX
                    ecx = eax;
                    // 00e2c230: SHL ECX,0x5
                    ecx = fl.shift_shl(ecx, 0x5_u32, 32);
                    // 00e2c233: ADD ECX,dword ptr [EBP + 0x14]
                    ecx = fl.add(ecx, e.mem.u32(ebp.wrapping_add(0x14)), 32);
                    // 00e2c236: MOVAPS XMM0,xmmword ptr [ECX]
                    xmm[0] = load_lanes(e, ecx);
                    // 00e2c239: MOVAPS XMM3,xmmword ptr [ECX + 0x10]
                    xmm[3] = load_lanes(e, ecx.wrapping_add(0x10));
                    // 00e2c23d: SUBPS XMM1,XMM0
                    xmm[1] = subps(xmm[1], xmm[0]);
                    // 00e2c240: MULPS XMM1,XMM3
                    xmm[1] = mulps(xmm[1], xmm[3]);
                    // 00e2c243: SUBPS XMM2,XMM0
                    xmm[2] = subps(xmm[2], xmm[0]);
                    // 00e2c246: MULPS XMM2,XMM3
                    xmm[2] = mulps(xmm[2], xmm[3]);
                    // 00e2c249: MOVAPS XMM0,XMM1
                    xmm[0] = xmm[1];
                    // 00e2c24c: MINPS XMM0,XMM2
                    xmm[0] = minps(xmm[0], xmm[2]);
                    // 00e2c24f: MAXPS XMM1,XMM2
                    xmm[1] = maxps(xmm[1], xmm[2]);
                    // 00e2c252: MOVAPS XMM2,XMM1
                    xmm[2] = xmm[1];
                    // 00e2c255: SHUFPS XMM2,XMM1,0xaa
                    xmm[2] = shufps(xmm[2], xmm[1], 170);
                    // 00e2c259: MOVAPS XMM3,XMM1
                    xmm[3] = xmm[1];
                    // 00e2c25c: SHUFPS XMM3,XMM1,0x55
                    xmm[3] = shufps(xmm[3], xmm[1], 85);
                    // 00e2c260: MINPS XMM3,XMM2
                    xmm[3] = minps(xmm[3], xmm[2]);
                    // 00e2c263: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2c266: MINPS XMM1,XMM4
                    xmm[1] = minps(xmm[1], xmm[4]);
                    // 00e2c269: MINPS XMM1,XMM3
                    xmm[1] = minps(xmm[1], xmm[3]);
                    // 00e2c26c: MOVAPS XMM3,XMM0
                    xmm[3] = xmm[0];
                    // 00e2c26f: SHUFPS XMM2,XMM0,0xaa
                    xmm[2] = shufps(xmm[2], xmm[0], 170);
                    // 00e2c273: SHUFPS XMM3,XMM0,0x55
                    xmm[3] = shufps(xmm[3], xmm[0], 85);
                    // 00e2c277: MAXPS XMM0,xmmword ptr [0x01267e30]
                    xmm[0] = maxps(xmm[0], load_lanes(e, 0x1267e30_u32));
                    // 00e2c27e: MAXPS XMM3,XMM2
                    xmm[3] = maxps(xmm[3], xmm[2]);
                    // 00e2c281: MAXPS XMM0,XMM3
                    xmm[0] = maxps(xmm[0], xmm[3]);
                    // 00e2c284: CMPLTPS XMM0,XMM1
                    xmm[0] = cmpltps(xmm[0], xmm[1]);
                    // 00e2c288: MOVMSKPS ECX,XMM0
                    ecx = movmskps(xmm[0]);
                    // 00e2c28b: TEST CL,0x1
                    fl.and(ecx & 0xff, 0x1_u32, 8);
                    // 00e2c28e: JZ 0x00e2c2b7
                    if fl.zf {
                        pc = 0x00e2_c2b7;
                        continue;
                    }
                    pc = 0x00e2_c290;
                }
                0x00e2_c290 => {
                    // 00e2c290: MOV EDX,dword ptr [EBP + 0x24]
                    edx = e.mem.u32(ebp.wrapping_add(0x24));
                    // 00e2c293: SHL EAX,0x6
                    eax = fl.shift_shl(eax, 0x6_u32, 32);
                    // 00e2c296: ADD EAX,dword ptr [EBP + 0x20]
                    eax = fl.add(eax, e.mem.u32(ebp.wrapping_add(0x20)), 32);
                    // 00e2c299: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2c29a: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2c29b: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2c29c: CALL 0x00e2baa0
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                    ];
                    let result = e.call(0x00e2baa0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2c2a1: MOV EAX,dword ptr [EBX + 0xc]
                    eax = e.mem.u32(ebx.wrapping_add(0xc));
                    // 00e2c2a4: MOV EDX,dword ptr [EAX + 0x4]
                    edx = e.mem.u32(eax.wrapping_add(0x4));
                    // 00e2c2a7: LEA ECX,[EBX + 0xc]
                    ecx = ebx.wrapping_add(0xc);
                    // 00e2c2aa: ADD ESP,0xc
                    esp = fl.add(esp, 0xc_u32, 32);
                    // 00e2c2ad: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2c2ae: CALL EDX
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(edx, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    st.push(result.st0);
                    // 00e2c2b0: FSTP ST0
                    st.pop();
                    // 00e2c2b2: MOVAPS XMM4,xmmword ptr [ESP + 0x40]
                    xmm[4] = load_lanes(e, esp.wrapping_add(0x40));
                    pc = 0x00e2_c2b7;
                }
                0x00e2_c2b7 => {
                    // 00e2c2b7: INC ESI
                    esi = fl.inc(esi, 32);
                    // 00e2c2b8: CMP ESI,dword ptr [EBP + 0x30]
                    fl.sub(esi, e.mem.u32(ebp.wrapping_add(0x30)), 32);
                    // 00e2c2bb: JL 0x00e2c21e
                    if fl.sf != fl.of {
                        pc = 0x00e2_c21e;
                        continue;
                    }
                    pc = 0x00e2_c2c1;
                }
                0x00e2_c2c1 => {
                    // 00e2c2c1: POP EDI
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c2c2: POP ESI
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c2c3: POP EBX
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c2c4: MOV ESP,EBP
                    esp = ebp;
                    // 00e2c2c6: POP EBP
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c2c7: RET
                    return;
                }
                0x00e2_c2c8 => {
                    // 00e2c2c8: MOV ECX,dword ptr [EBP + 0x30]
                    ecx = e.mem.u32(ebp.wrapping_add(0x30));
                    // 00e2c2cb: CMP dword ptr [EBP + 0x38],ECX
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x38)), ecx, 32);
                    // 00e2c2ce: JL 0x00e2c5f9
                    if fl.sf != fl.of {
                        pc = 0x00e2_c5f9;
                        continue;
                    }
                    pc = 0x00e2_c2d4;
                }
                0x00e2_c2d4 => {
                    // 00e2c2d4: CMP dword ptr [EBP + 0x3c],ECX
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x3c)), ecx, 32);
                    // 00e2c2d7: JL 0x00e2c5f9
                    if fl.sf != fl.of {
                        pc = 0x00e2_c5f9;
                        continue;
                    }
                    pc = 0x00e2_c2dd;
                }
                0x00e2_c2dd => {
                    // 00e2c2dd: CMP dword ptr [EBP + 0x34],0x2
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x34)), 0x2_u32, 32);
                    // 00e2c2e1: JL 0x00e2c5f9
                    if fl.sf != fl.of {
                        pc = 0x00e2_c5f9;
                        continue;
                    }
                    pc = 0x00e2_c2e7;
                }
                0x00e2_c2e7 => {
                    // 00e2c2e7: CMP ECX,0x4
                    fl.sub(ecx, 0x4_u32, 32);
                    // 00e2c2ea: JLE 0x00e2c5f9
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c5f9;
                        continue;
                    }
                    pc = 0x00e2_c2f0;
                }
                0x00e2_c2f0 => {
                    // 00e2c2f0: MOVZX EAX,word ptr [EDI + 0xc]
                    eax = u32::from(e.mem.u16(edi.wrapping_add(0xc)));
                    // 00e2c2f4: MOV EBX,dword ptr [EBP + 0x8]
                    ebx = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2c2f7: ADD EBX,0x1c
                    ebx = fl.add(ebx, 0x1c_u32, 32);
                    // 00e2c2fa: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2c2fb: MOV ECX,EBX
                    ecx = ebx;
                    // 00e2c2fd: CALL 0x00d9a4d0
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d9a4d0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2c302: MOVZX ECX,word ptr [EDI + 0xe]
                    ecx = u32::from(e.mem.u16(edi.wrapping_add(0xe)));
                    // 00e2c306: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2c307: MOV ECX,EBX
                    ecx = ebx;
                    // 00e2c309: MOV ESI,EAX
                    esi = eax;
                    // 00e2c30b: CALL 0x00d9a4d0
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d9a4d0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2c310: MOV EDX,dword ptr [ESI]
                    edx = e.mem.u32(esi);
                    // 00e2c312: MOV ECX,dword ptr [EBP + 0x50]
                    ecx = e.mem.u32(ebp.wrapping_add(0x50));
                    // 00e2c315: MOV dword ptr [ECX],EDX
                    e.mem.set_u32(ecx, edx);
                    // 00e2c317: MOV EDX,dword ptr [ESI + 0x4]
                    edx = e.mem.u32(esi.wrapping_add(0x4));
                    // 00e2c31a: MOV dword ptr [ECX + 0x4],EDX
                    e.mem.set_u32(ecx.wrapping_add(0x4), edx);
                    // 00e2c31d: MOV EDX,dword ptr [ESI + 0x8]
                    edx = e.mem.u32(esi.wrapping_add(0x8));
                    // 00e2c320: MOV dword ptr [ECX + 0x8],EDX
                    e.mem.set_u32(ecx.wrapping_add(0x8), edx);
                    // 00e2c323: MOV EDX,dword ptr [ESI + 0xc]
                    edx = e.mem.u32(esi.wrapping_add(0xc));
                    // 00e2c326: MOV dword ptr [ECX + 0xc],EDX
                    e.mem.set_u32(ecx.wrapping_add(0xc), edx);
                    // 00e2c329: MOV EDX,dword ptr [ESI + 0x10]
                    edx = e.mem.u32(esi.wrapping_add(0x10));
                    // 00e2c32c: MOV dword ptr [ECX + 0x10],EDX
                    e.mem.set_u32(ecx.wrapping_add(0x10), edx);
                    // 00e2c32f: MOV ESI,dword ptr [EAX]
                    esi = e.mem.u32(eax);
                    // 00e2c331: MOV EDX,dword ptr [EBP + 0x54]
                    edx = e.mem.u32(ebp.wrapping_add(0x54));
                    // 00e2c334: MOV dword ptr [EDX],ESI
                    e.mem.set_u32(edx, esi);
                    // 00e2c336: MOV ESI,dword ptr [EAX + 0x4]
                    esi = e.mem.u32(eax.wrapping_add(0x4));
                    // 00e2c339: MOV dword ptr [EDX + 0x4],ESI
                    e.mem.set_u32(edx.wrapping_add(0x4), esi);
                    // 00e2c33c: MOV ESI,dword ptr [EAX + 0x8]
                    esi = e.mem.u32(eax.wrapping_add(0x8));
                    // 00e2c33f: MOV dword ptr [EDX + 0x8],ESI
                    e.mem.set_u32(edx.wrapping_add(0x8), esi);
                    // 00e2c342: MOV ESI,dword ptr [EAX + 0xc]
                    esi = e.mem.u32(eax.wrapping_add(0xc));
                    // 00e2c345: MOV dword ptr [EDX + 0xc],ESI
                    e.mem.set_u32(edx.wrapping_add(0xc), esi);
                    // 00e2c348: MOV EAX,dword ptr [EAX + 0x10]
                    eax = e.mem.u32(eax.wrapping_add(0x10));
                    // 00e2c34b: MOV dword ptr [EDX + 0x10],EAX
                    e.mem.set_u32(edx.wrapping_add(0x10), eax);
                    // 00e2c34e: MOVZX EAX,word ptr [ECX]
                    eax = u32::from(e.mem.u16(ecx));
                    // 00e2c351: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c354: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c358: MOVZX EAX,word ptr [ECX + 0x2]
                    eax = u32::from(e.mem.u16(ecx.wrapping_add(0x2)));
                    // 00e2c35c: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c360: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c363: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c367: MOVZX EAX,word ptr [ECX + 0x4]
                    eax = u32::from(e.mem.u16(ecx.wrapping_add(0x4)));
                    // 00e2c36b: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c36f: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c373: MOVSS XMM1,dword ptr [ESP + 0x20]
                    xmm[1] = [e.mem.u32(esp.wrapping_add(0x20)), 0, 0, 0];
                    // 00e2c379: MOVSS XMM4,dword ptr [0x01011d78]
                    xmm[4] = [e.mem.u32(0x1011d78_u32), 0, 0, 0];
                    // 00e2c381: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c384: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(esp.wrapping_add(0x1c), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c388: MOVSS XMM0,dword ptr [ESP + 0x1c]
                    xmm[0] = [e.mem.u32(esp.wrapping_add(0x1c)), 0, 0, 0];
                    // 00e2c38e: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c392: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c396: MOVZX EAX,word ptr [ECX + 0x6]
                    eax = u32::from(e.mem.u16(ecx.wrapping_add(0x6)));
                    // 00e2c39a: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c39d: MOVSS XMM3,XMM4
                    xmm[3][0] = xmm[4][0];
                    // 00e2c3a1: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c3a5: MOVSS XMM2,dword ptr [ESP + 0x18]
                    xmm[2] = [e.mem.u32(esp.wrapping_add(0x18)), 0, 0, 0];
                    // 00e2c3ab: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c3af: MOVZX EAX,word ptr [ECX + 0x8]
                    eax = u32::from(e.mem.u16(ecx.wrapping_add(0x8)));
                    // 00e2c3b3: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c3b7: MOVZX ECX,word ptr [ECX + 0xa]
                    ecx = u32::from(e.mem.u16(ecx.wrapping_add(0xa)));
                    // 00e2c3bb: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c3bf: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c3c2: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c3c6: MOV EAX,dword ptr [EBP + 0x8]
                    eax = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2c3c9: SHL ECX,0xf
                    ecx = fl.shift_shl(ecx, 0xf_u32, 32);
                    // 00e2c3cc: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c3d0: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx);
                    // 00e2c3d4: UNPCKLPS XMM0,XMM3
                    xmm[0] = unpcklps(xmm[0], xmm[3]);
                    // 00e2c3d7: UNPCKLPS XMM1,XMM2
                    xmm[1] = unpcklps(xmm[1], xmm[2]);
                    // 00e2c3da: MOVAPS XMM2,xmmword ptr [EAX + 0x70]
                    xmm[2] = load_lanes(e, eax.wrapping_add(0x70));
                    // 00e2c3de: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(esp.wrapping_add(0x1c), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c3e2: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c3e6: MOVSS XMM3,dword ptr [ESP + 0x1c]
                    xmm[3] = [e.mem.u32(esp.wrapping_add(0x1c)), 0, 0, 0];
                    // 00e2c3ec: UNPCKLPS XMM1,XMM0
                    xmm[1] = unpcklps(xmm[1], xmm[0]);
                    // 00e2c3ef: MOVSS XMM0,dword ptr [ESP + 0x20]
                    xmm[0] = [e.mem.u32(esp.wrapping_add(0x20)), 0, 0, 0];
                    // 00e2c3f5: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c3f9: MOVSS XMM6,XMM4
                    xmm[6][0] = xmm[4][0];
                    // 00e2c3fd: MOVSS XMM5,dword ptr [ESP + 0x18]
                    xmm[5] = [e.mem.u32(esp.wrapping_add(0x18)), 0, 0, 0];
                    // 00e2c403: UNPCKLPS XMM3,XMM6
                    xmm[3] = unpcklps(xmm[3], xmm[6]);
                    // 00e2c406: UNPCKLPS XMM0,XMM5
                    xmm[0] = unpcklps(xmm[0], xmm[5]);
                    // 00e2c409: UNPCKLPS XMM0,XMM3
                    xmm[0] = unpcklps(xmm[0], xmm[3]);
                    // 00e2c40c: MULPS XMM0,XMM2
                    xmm[0] = mulps(xmm[0], xmm[2]);
                    // 00e2c40f: MOVAPS XMM3,xmmword ptr [EAX + 0x40]
                    xmm[3] = load_lanes(e, eax.wrapping_add(0x40));
                    // 00e2c413: MOVZX EAX,word ptr [EDX]
                    eax = u32::from(e.mem.u16(edx));
                    // 00e2c416: MOVZX ECX,word ptr [EDX + 0x2]
                    ecx = u32::from(e.mem.u16(edx.wrapping_add(0x2)));
                    // 00e2c41a: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c41d: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c421: MOVZX EAX,word ptr [EDX + 0x4]
                    eax = u32::from(e.mem.u16(edx.wrapping_add(0x4)));
                    // 00e2c425: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c429: SHL ECX,0xf
                    ecx = fl.shift_shl(ecx, 0xf_u32, 32);
                    // 00e2c42c: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx);
                    // 00e2c430: MOVZX ECX,word ptr [EDX + 0x6]
                    ecx = u32::from(e.mem.u16(edx.wrapping_add(0x6)));
                    // 00e2c434: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c438: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c43c: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c43f: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c443: MOVZX EAX,word ptr [EDX + 0x8]
                    eax = u32::from(e.mem.u16(edx.wrapping_add(0x8)));
                    // 00e2c447: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(esp.wrapping_add(0x1c), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c44b: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c44f: SHL ECX,0xf
                    ecx = fl.shift_shl(ecx, 0xf_u32, 32);
                    // 00e2c452: MULPS XMM1,XMM2
                    xmm[1] = mulps(xmm[1], xmm[2]);
                    // 00e2c455: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c459: MOVSS XMM6,dword ptr [ESP + 0x18]
                    xmm[6] = [e.mem.u32(esp.wrapping_add(0x18)), 0, 0, 0];
                    // 00e2c45f: SUBPS XMM1,XMM3
                    xmm[1] = subps(xmm[1], xmm[3]);
                    // 00e2c462: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx);
                    // 00e2c466: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c46a: MOVZX ECX,word ptr [EDX + 0xa]
                    ecx = u32::from(e.mem.u16(edx.wrapping_add(0xa)));
                    // 00e2c46e: SUBPS XMM0,XMM3
                    xmm[0] = subps(xmm[0], xmm[3]);
                    // 00e2c471: MOVAPS xmmword ptr [ESP + 0x60],XMM1
                    store_lanes(e, esp.wrapping_add(0x60), xmm[1]);
                    // 00e2c476: MOVSS XMM1,dword ptr [ESP + 0x20]
                    xmm[1] = [e.mem.u32(esp.wrapping_add(0x20)), 0, 0, 0];
                    // 00e2c47c: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c480: MOVAPS XMM5,XMM0
                    xmm[5] = xmm[0];
                    // 00e2c483: MOVSS XMM0,dword ptr [ESP + 0x1c]
                    xmm[0] = [e.mem.u32(esp.wrapping_add(0x1c)), 0, 0, 0];
                    // 00e2c489: SHL EAX,0xf
                    eax = fl.shift_shl(eax, 0xf_u32, 32);
                    // 00e2c48c: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c490: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c494: MOVSS XMM7,XMM4
                    xmm[7][0] = xmm[4][0];
                    // 00e2c498: SHL ECX,0xf
                    ecx = fl.shift_shl(ecx, 0xf_u32, 32);
                    // 00e2c49b: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx);
                    // 00e2c49f: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(esp.wrapping_add(0x1c), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c4a3: UNPCKLPS XMM1,XMM6
                    xmm[1] = unpcklps(xmm[1], xmm[6]);
                    // 00e2c4a6: FILD dword ptr [ESP + 0x18]
                    st.push(f64::from(e.mem.i32(esp.wrapping_add(0x18))));
                    // 00e2c4aa: MOVSS XMM6,dword ptr [ESP + 0x1c]
                    xmm[6] = [e.mem.u32(esp.wrapping_add(0x1c)), 0, 0, 0];
                    // 00e2c4b0: XOR ESI,ESI
                    esi = fl.xor(esi, esi, 32);
                    // 00e2c4b2: CMP dword ptr [EBP + 0x30],ESI
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x30)), esi, 32);
                    // 00e2c4b5: UNPCKLPS XMM0,XMM7
                    xmm[0] = unpcklps(xmm[0], xmm[7]);
                    // 00e2c4b8: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c4bc: MOVSS XMM7,dword ptr [ESP + 0x18]
                    xmm[7] = [e.mem.u32(esp.wrapping_add(0x18)), 0, 0, 0];
                    // 00e2c4c2: UNPCKLPS XMM1,XMM0
                    xmm[1] = unpcklps(xmm[1], xmm[0]);
                    // 00e2c4c5: MOVSS XMM0,dword ptr [ESP + 0x20]
                    xmm[0] = [e.mem.u32(esp.wrapping_add(0x20)), 0, 0, 0];
                    // 00e2c4cb: UNPCKLPS XMM6,XMM4
                    xmm[6] = unpcklps(xmm[6], xmm[4]);
                    // 00e2c4ce: MULPS XMM1,XMM2
                    xmm[1] = mulps(xmm[1], xmm[2]);
                    // 00e2c4d1: SUBPS XMM1,XMM3
                    xmm[1] = subps(xmm[1], xmm[3]);
                    // 00e2c4d4: UNPCKLPS XMM0,XMM7
                    xmm[0] = unpcklps(xmm[0], xmm[7]);
                    // 00e2c4d7: UNPCKLPS XMM0,XMM6
                    xmm[0] = unpcklps(xmm[0], xmm[6]);
                    // 00e2c4da: MULPS XMM0,XMM2
                    xmm[0] = mulps(xmm[0], xmm[2]);
                    // 00e2c4dd: SUBPS XMM0,XMM3
                    xmm[0] = subps(xmm[0], xmm[3]);
                    // 00e2c4e0: MOVSS XMM3,dword ptr [0x01012068]
                    xmm[3] = [e.mem.u32(0x1012068_u32), 0, 0, 0];
                    // 00e2c4e8: MOVAPS XMM6,XMM0
                    xmm[6] = xmm[0];
                    // 00e2c4eb: MOVAPS XMM7,XMM1
                    xmm[7] = xmm[1];
                    // 00e2c4ee: SHUFPS XMM3,XMM3,0x0
                    xmm[3] = shufps(xmm[3], xmm[3], 0);
                    // 00e2c4f2: JLE 0x00e2c88f
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c88f;
                        continue;
                    }
                    pc = 0x00e2_c4f8;
                }
                0x00e2_c4f8 => {
                    // 00e2c4f8: MOV EDI,dword ptr [EBP + 0x48]
                    edi = e.mem.u32(ebp.wrapping_add(0x48));
                    // 00e2c4fb: MOV EBX,dword ptr [EBP + 0x40]
                    ebx = e.mem.u32(ebp.wrapping_add(0x40));
                    // 00e2c4fe: MOV EDI,EDI
                    // (no operation)
                    pc = 0x00e2_c500;
                }
                0x00e2_c500 => {
                    // 00e2c500: MOV EDX,dword ptr [EBP + 0x2c]
                    edx = e.mem.u32(ebp.wrapping_add(0x2c));
                    // 00e2c503: MOV EAX,dword ptr [EDX + ESI*0x4]
                    eax = e.mem.u32(edx.wrapping_add(esi.wrapping_mul(4)));
                    // 00e2c506: MOV EDX,dword ptr [EBP + 0x14]
                    edx = e.mem.u32(ebp.wrapping_add(0x14));
                    // 00e2c509: MOVAPS XMM2,xmmword ptr [ESP + 0x60]
                    xmm[2] = load_lanes(e, esp.wrapping_add(0x60));
                    // 00e2c50e: MOV ECX,EAX
                    ecx = eax;
                    // 00e2c510: SHL ECX,0x5
                    ecx = fl.shift_shl(ecx, 0x5_u32, 32);
                    // 00e2c513: MOVAPS XMM0,xmmword ptr [ECX + EDX*0x1]
                    xmm[0] = load_lanes(e, ecx.wrapping_add(edx));
                    // 00e2c517: MOVAPS XMM4,xmmword ptr [ECX + EDX*0x1 + 0x10]
                    xmm[4] = load_lanes(e, ecx.wrapping_add(edx).wrapping_add(0x10));
                    // 00e2c51c: SUBPS XMM2,XMM0
                    xmm[2] = subps(xmm[2], xmm[0]);
                    // 00e2c51f: MULPS XMM2,XMM4
                    xmm[2] = mulps(xmm[2], xmm[4]);
                    // 00e2c522: ADD ECX,EDX
                    ecx = fl.add(ecx, edx, 32);
                    // 00e2c524: MOVAPS XMM1,XMM5
                    xmm[1] = xmm[5];
                    // 00e2c527: SUBPS XMM1,XMM0
                    xmm[1] = subps(xmm[1], xmm[0]);
                    // 00e2c52a: MULPS XMM1,XMM4
                    xmm[1] = mulps(xmm[1], xmm[4]);
                    // 00e2c52d: MOVAPS XMM0,XMM1
                    xmm[0] = xmm[1];
                    // 00e2c530: MINPS XMM0,XMM2
                    xmm[0] = minps(xmm[0], xmm[2]);
                    // 00e2c533: MAXPS XMM1,XMM2
                    xmm[1] = maxps(xmm[1], xmm[2]);
                    // 00e2c536: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2c539: SHUFPS XMM2,XMM0,0xaa
                    xmm[2] = shufps(xmm[2], xmm[0], 170);
                    // 00e2c53d: MOVAPS XMM4,XMM0
                    xmm[4] = xmm[0];
                    // 00e2c540: SHUFPS XMM4,XMM0,0x55
                    xmm[4] = shufps(xmm[4], xmm[0], 85);
                    // 00e2c544: MAXPS XMM0,xmmword ptr [0x01267e30]
                    xmm[0] = maxps(xmm[0], load_lanes(e, 0x1267e30_u32));
                    // 00e2c54b: MAXPS XMM4,XMM2
                    xmm[4] = maxps(xmm[4], xmm[2]);
                    // 00e2c54e: MAXPS XMM0,XMM4
                    xmm[0] = maxps(xmm[0], xmm[4]);
                    // 00e2c551: MOVAPS XMM2,XMM1
                    xmm[2] = xmm[1];
                    // 00e2c554: MOVAPS XMM4,XMM1
                    xmm[4] = xmm[1];
                    // 00e2c557: SHUFPS XMM2,XMM1,0xaa
                    xmm[2] = shufps(xmm[2], xmm[1], 170);
                    // 00e2c55b: SHUFPS XMM4,XMM1,0x55
                    xmm[4] = shufps(xmm[4], xmm[1], 85);
                    // 00e2c55f: MINPS XMM4,XMM2
                    xmm[4] = minps(xmm[4], xmm[2]);
                    // 00e2c562: MINPS XMM1,XMM3
                    xmm[1] = minps(xmm[1], xmm[3]);
                    // 00e2c565: MINPS XMM1,XMM4
                    xmm[1] = minps(xmm[1], xmm[4]);
                    // 00e2c568: CMPLTPS XMM0,XMM1
                    xmm[0] = cmpltps(xmm[0], xmm[1]);
                    // 00e2c56c: MOVMSKPS EDX,XMM0
                    edx = movmskps(xmm[0]);
                    // 00e2c56f: TEST DL,0x1
                    fl.and(edx & 0xff, 0x1_u32, 8);
                    // 00e2c572: JZ 0x00e2c57e
                    if fl.zf {
                        pc = 0x00e2_c57e;
                        continue;
                    }
                    pc = 0x00e2_c574;
                }
                0x00e2_c574 => {
                    // 00e2c574: MOV EDX,dword ptr [EBP + 0x44]
                    edx = e.mem.u32(ebp.wrapping_add(0x44));
                    // 00e2c577: MOV dword ptr [EBX],EAX
                    e.mem.set_u32(ebx, eax);
                    // 00e2c579: ADD EBX,0x4
                    ebx = fl.add(ebx, 0x4_u32, 32);
                    // 00e2c57c: INC dword ptr [EDX]
                    e.mem.set_u32(edx, fl.inc(e.mem.u32(edx), 32));
                    pc = 0x00e2_c57e;
                }
                0x00e2_c57e => {
                    // 00e2c57e: MOVAPS XMM0,xmmword ptr [ECX]
                    xmm[0] = load_lanes(e, ecx);
                    // 00e2c581: MOVAPS XMM4,xmmword ptr [ECX + 0x10]
                    xmm[4] = load_lanes(e, ecx.wrapping_add(0x10));
                    // 00e2c585: MOVAPS XMM1,XMM6
                    xmm[1] = xmm[6];
                    // 00e2c588: SUBPS XMM1,XMM0
                    xmm[1] = subps(xmm[1], xmm[0]);
                    // 00e2c58b: MULPS XMM1,XMM4
                    xmm[1] = mulps(xmm[1], xmm[4]);
                    // 00e2c58e: MOVAPS XMM2,XMM7
                    xmm[2] = xmm[7];
                    // 00e2c591: SUBPS XMM2,XMM0
                    xmm[2] = subps(xmm[2], xmm[0]);
                    // 00e2c594: MULPS XMM2,XMM4
                    xmm[2] = mulps(xmm[2], xmm[4]);
                    // 00e2c597: MOVAPS XMM0,XMM1
                    xmm[0] = xmm[1];
                    // 00e2c59a: MINPS XMM0,XMM2
                    xmm[0] = minps(xmm[0], xmm[2]);
                    // 00e2c59d: MAXPS XMM1,XMM2
                    xmm[1] = maxps(xmm[1], xmm[2]);
                    // 00e2c5a0: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2c5a3: SHUFPS XMM2,XMM0,0xaa
                    xmm[2] = shufps(xmm[2], xmm[0], 170);
                    // 00e2c5a7: MOVAPS XMM4,XMM0
                    xmm[4] = xmm[0];
                    // 00e2c5aa: SHUFPS XMM4,XMM0,0x55
                    xmm[4] = shufps(xmm[4], xmm[0], 85);
                    // 00e2c5ae: MAXPS XMM0,xmmword ptr [0x01267e30]
                    xmm[0] = maxps(xmm[0], load_lanes(e, 0x1267e30_u32));
                    // 00e2c5b5: MAXPS XMM4,XMM2
                    xmm[4] = maxps(xmm[4], xmm[2]);
                    // 00e2c5b8: MAXPS XMM0,XMM4
                    xmm[0] = maxps(xmm[0], xmm[4]);
                    // 00e2c5bb: MOVAPS XMM2,XMM1
                    xmm[2] = xmm[1];
                    // 00e2c5be: MOVAPS XMM4,XMM1
                    xmm[4] = xmm[1];
                    // 00e2c5c1: SHUFPS XMM2,XMM1,0xaa
                    xmm[2] = shufps(xmm[2], xmm[1], 170);
                    // 00e2c5c5: SHUFPS XMM4,XMM1,0x55
                    xmm[4] = shufps(xmm[4], xmm[1], 85);
                    // 00e2c5c9: MINPS XMM4,XMM2
                    xmm[4] = minps(xmm[4], xmm[2]);
                    // 00e2c5cc: MINPS XMM1,XMM3
                    xmm[1] = minps(xmm[1], xmm[3]);
                    // 00e2c5cf: MINPS XMM1,XMM4
                    xmm[1] = minps(xmm[1], xmm[4]);
                    // 00e2c5d2: CMPLTPS XMM0,XMM1
                    xmm[0] = cmpltps(xmm[0], xmm[1]);
                    // 00e2c5d6: MOVMSKPS ECX,XMM0
                    ecx = movmskps(xmm[0]);
                    // 00e2c5d9: TEST CL,0x1
                    fl.and(ecx & 0xff, 0x1_u32, 8);
                    // 00e2c5dc: JZ 0x00e2c5e8
                    if fl.zf {
                        pc = 0x00e2_c5e8;
                        continue;
                    }
                    pc = 0x00e2_c5de;
                }
                0x00e2_c5de => {
                    // 00e2c5de: MOV dword ptr [EDI],EAX
                    e.mem.set_u32(edi, eax);
                    // 00e2c5e0: MOV EAX,dword ptr [EBP + 0x4c]
                    eax = e.mem.u32(ebp.wrapping_add(0x4c));
                    // 00e2c5e3: ADD EDI,0x4
                    edi = fl.add(edi, 0x4_u32, 32);
                    // 00e2c5e6: INC dword ptr [EAX]
                    e.mem.set_u32(eax, fl.inc(e.mem.u32(eax), 32));
                    pc = 0x00e2_c5e8;
                }
                0x00e2_c5e8 => {
                    // 00e2c5e8: INC ESI
                    esi = fl.inc(esi, 32);
                    // 00e2c5e9: CMP ESI,dword ptr [EBP + 0x30]
                    fl.sub(esi, e.mem.u32(ebp.wrapping_add(0x30)), 32);
                    // 00e2c5ec: JL 0x00e2c500
                    if fl.sf != fl.of {
                        pc = 0x00e2_c500;
                        continue;
                    }
                    pc = 0x00e2_c5f2;
                }
                0x00e2_c5f2 => {
                    // 00e2c5f2: POP EDI
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c5f3: POP ESI
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c5f4: POP EBX
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c5f5: MOV ESP,EBP
                    esp = ebp;
                    // 00e2c5f7: POP EBP
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c5f8: RET
                    return;
                }
                0x00e2_c5f9 => {
                    // 00e2c5f9: LEA EAX,[ECX + 0x3]
                    eax = ecx.wrapping_add(0x3);
                    // 00e2c5fc: SAR EAX,0x2
                    eax = fl.shift_sar(eax, 0x2_u32, 32);
                    // 00e2c5ff: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2c601: MOV dword ptr [ESP + 0x28],ESI
                    e.mem.set_u32(esp.wrapping_add(0x28), esi);
                    // 00e2c605: MOV dword ptr [ESP + 0x2c],ESI
                    e.mem.set_u32(esp.wrapping_add(0x2c), esi);
                    // 00e2c609: MOV dword ptr [ESP + 0x30],ESI
                    e.mem.set_u32(esp.wrapping_add(0x30), esi);
                    // 00e2c60d: MOV dword ptr [ESP + 0x34],ESI
                    e.mem.set_u32(esp.wrapping_add(0x34), esi);
                    // 00e2c611: JLE 0x00e2c88f
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c88f;
                        continue;
                    }
                    pc = 0x00e2_c617;
                }
                0x00e2_c617 => {
                    // 00e2c617: MOV EDX,dword ptr [EBP + 0x2c]
                    edx = e.mem.u32(ebp.wrapping_add(0x2c));
                    // 00e2c61a: MOV EBX,dword ptr [EBP + 0x18]
                    ebx = e.mem.u32(ebp.wrapping_add(0x18));
                    // 00e2c61d: MOV EDI,ECX
                    edi = ecx;
                    // 00e2c61f: MOV dword ptr [ESP + 0x1c],EDX
                    e.mem.set_u32(esp.wrapping_add(0x1c), edx);
                    // 00e2c623: MOV dword ptr [ESP + 0x20],EDI
                    e.mem.set_u32(esp.wrapping_add(0x20), edi);
                    // 00e2c627: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2c62b: JMP 0x00e2c632
                    pc = 0x00e2_c632;
                    continue;
                }
                0x00e2_c630 => {
                    // 00e2c630: XOR ESI,ESI
                    esi = fl.xor(esi, esi, 32);
                    pc = 0x00e2_c632;
                }
                0x00e2_c632 => {
                    // 00e2c632: CMP EDI,0x4
                    fl.sub(edi, 0x4_u32, 32);
                    // 00e2c635: JGE 0x00e2c63d
                    if fl.sf == fl.of {
                        pc = 0x00e2_c63d;
                        continue;
                    }
                    pc = 0x00e2_c637;
                }
                0x00e2_c637 => {
                    // 00e2c637: MOV dword ptr [ESP + 0x24],EDI
                    e.mem.set_u32(esp.wrapping_add(0x24), edi);
                    // 00e2c63b: JMP 0x00e2c649
                    pc = 0x00e2_c649;
                    continue;
                }
                0x00e2_c63d => {
                    // 00e2c63d: MOV dword ptr [ESP + 0x24],0x4
                    e.mem.set_u32(esp.wrapping_add(0x24), 0x4_u32);
                    // 00e2c645: MOV EDI,dword ptr [ESP + 0x24]
                    edi = e.mem.u32(esp.wrapping_add(0x24));
                    pc = 0x00e2_c649;
                }
                0x00e2_c649 => {
                    // 00e2c649: CMP EDI,ESI
                    fl.sub(edi, esi, 32);
                    // 00e2c64b: MOV dword ptr [ESP + 0x3c],ESI
                    e.mem.set_u32(esp.wrapping_add(0x3c), esi);
                    // 00e2c64f: JLE 0x00e2c71f
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c71f;
                        continue;
                    }
                    pc = 0x00e2_c655;
                }
                0x00e2_c655 => {
                    // 00e2c655: MOV ESI,dword ptr [EBP + 0x1c]
                    esi = e.mem.u32(ebp.wrapping_add(0x1c));
                    // 00e2c658: MOV EAX,dword ptr [ESP + 0x1c]
                    eax = e.mem.u32(esp.wrapping_add(0x1c));
                    // 00e2c65c: ADD EBX,0x60
                    ebx = fl.add(ebx, 0x60_u32, 32);
                    // 00e2c65f: ADD ESI,0x8
                    esi = fl.add(esi, 0x8_u32, 32);
                    // 00e2c662: MOV dword ptr [ESP + 0x38],EAX
                    e.mem.set_u32(esp.wrapping_add(0x38), eax);
                    pc = 0x00e2_c666;
                }
                0x00e2_c666 => {
                    // 00e2c666: MOV ECX,dword ptr [ESP + 0x38]
                    ecx = e.mem.u32(esp.wrapping_add(0x38));
                    // 00e2c66a: MOV EAX,dword ptr [ECX]
                    eax = e.mem.u32(ecx);
                    // 00e2c66c: MOV EDX,dword ptr [ESP + 0x3c]
                    edx = e.mem.u32(esp.wrapping_add(0x3c));
                    // 00e2c670: MOV dword ptr [ESP + EDX*0x4 + 0x28],EAX
                    e.mem.set_u32(
                        esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x28),
                        eax,
                    );
                    // 00e2c674: SHL EAX,0x6
                    eax = fl.shift_shl(eax, 0x6_u32, 32);
                    // 00e2c677: ADD EAX,dword ptr [EBP + 0x20]
                    eax = fl.add(eax, e.mem.u32(ebp.wrapping_add(0x20)), 32);
                    // 00e2c67a: LEA ECX,[EBX + -0x60]
                    ecx = ebx.wrapping_sub(0x60);
                    // 00e2c67d: MOV EDI,EAX
                    edi = eax;
                    // 00e2c67f: MOV EAX,dword ptr [EBP + 0x24]
                    eax = e.mem.u32(ebp.wrapping_add(0x24));
                    // 00e2c682: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2c683: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2c684: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2c685: CALL 0x00e2baa0
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                    ];
                    let result = e.call(0x00e2baa0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2c68a: MOV ECX,dword ptr [ESP + 0x48]
                    ecx = e.mem.u32(esp.wrapping_add(0x48));
                    // 00e2c68e: MOV EDX,dword ptr [ESP + ECX*0x4 + 0x34]
                    edx = e
                        .mem
                        .u32(esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x34));
                    // 00e2c692: MOV EAX,dword ptr [EBP + 0x28]
                    eax = e.mem.u32(ebp.wrapping_add(0x28));
                    // 00e2c695: MOVSS XMM0,dword ptr [EAX + EDX*0x4]
                    xmm[0] = [e.mem.u32(eax.wrapping_add(edx.wrapping_mul(4))), 0, 0, 0];
                    // 00e2c69a: SHUFPS XMM0,XMM0,0x0
                    xmm[0] = shufps(xmm[0], xmm[0], 0);
                    // 00e2c69e: MOVAPS xmmword ptr [EBX + 0x10],XMM0
                    store_lanes(e, ebx.wrapping_add(0x10), xmm[0]);
                    // 00e2c6a2: FLD float ptr [EAX + EDX*0x4]
                    st.push(x87_load32(e.mem.u32(eax.wrapping_add(edx.wrapping_mul(4)))));
                    // 00e2c6a5: FSTP float ptr [ESP + ECX*0x4 + 0x5c]
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x5c),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c6a9: ADD ESP,0xc
                    esp = fl.add(esp, 0xc_u32, 32);
                    // 00e2c6ac: CMP byte ptr [EDI + 0x3c],0x0
                    fl.sub(u32::from(e.mem.u8(edi.wrapping_add(0x3c))), 0x0_u32, 8);
                    // 00e2c6b0: JZ 0x00e2c6fb
                    if fl.zf {
                        pc = 0x00e2_c6fb;
                        continue;
                    }
                    pc = 0x00e2_c6b2;
                }
                0x00e2_c6b2 => {
                    // 00e2c6b2: LEA EAX,[ESI + -0x8]
                    eax = esi.wrapping_sub(0x8);
                    // 00e2c6b5: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2c6b7: JZ 0x00e2c6eb
                    if fl.zf {
                        pc = 0x00e2_c6eb;
                        continue;
                    }
                    pc = 0x00e2_c6b9;
                }
                0x00e2_c6b9 => {
                    // 00e2c6b9: MOV EDX,dword ptr [EDI + 0x34]
                    edx = e.mem.u32(edi.wrapping_add(0x34));
                    // 00e2c6bc: FLD1
                    st.push(1.0);
                    // 00e2c6be: MOV EDI,dword ptr [EDI + 0x30]
                    edi = e.mem.u32(edi.wrapping_add(0x30));
                    // 00e2c6c1: FSTP float ptr [ESI + -0x4]
                    e.mem
                        .set_u32(esi.wrapping_sub(0x4), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2c6c4: MOV dword ptr [ESI + 0x8],EDX
                    e.mem.set_u32(esi.wrapping_add(0x8), edx);
                    // 00e2c6c7: LEA EAX,[ESI + -0x8]
                    eax = esi.wrapping_sub(0x8);
                    // 00e2c6ca: XOR EDX,EDX
                    edx = fl.xor(edx, edx, 32);
                    // 00e2c6cc: MOV dword ptr [EAX],0x10dbb04
                    e.mem.set_u32(eax, 0x10dbb04_u32);
                    // 00e2c6d2: MOV dword ptr [ESI],EDI
                    e.mem.set_u32(esi, edi);
                    // 00e2c6d4: MOV dword ptr [ESI + 0x4],EDI
                    e.mem.set_u32(esi.wrapping_add(0x4), edi);
                    // 00e2c6d7: MOV dword ptr [ESI + 0xc],EDX
                    e.mem.set_u32(esi.wrapping_add(0xc), edx);
                    // 00e2c6da: MOV dword ptr [ESI + 0x10],EDX
                    e.mem.set_u32(esi.wrapping_add(0x10), edx);
                    // 00e2c6dd: MOV dword ptr [EBX],EAX
                    e.mem.set_u32(ebx, eax);
                    // 00e2c6df: MOV dword ptr [0x0126ec04],0xe2b9b0
                    e.mem.set_u32(0x126ec04_u32, 0xe2b9b0_u32);
                    // 00e2c6e9: JMP 0x00e2c701
                    pc = 0x00e2_c701;
                    continue;
                }
                0x00e2_c6eb => {
                    // 00e2c6eb: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    // 00e2c6ed: MOV dword ptr [EBX],EAX
                    e.mem.set_u32(ebx, eax);
                    // 00e2c6ef: MOV dword ptr [0x0126ec04],0xe2b9b0
                    e.mem.set_u32(0x126ec04_u32, 0xe2b9b0_u32);
                    // 00e2c6f9: JMP 0x00e2c701
                    pc = 0x00e2_c701;
                    continue;
                }
                0x00e2_c6fb => {
                    // 00e2c6fb: MOV dword ptr [EBX],0x0
                    e.mem.set_u32(ebx, 0x0_u32);
                    pc = 0x00e2_c701;
                }
                0x00e2_c701 => {
                    // 00e2c701: ADD dword ptr [ESP + 0x38],0x4
                    e.mem.set_u32(
                        esp.wrapping_add(0x38),
                        fl.add(e.mem.u32(esp.wrapping_add(0x38)), 0x4_u32, 32),
                    );
                    // 00e2c706: INC ECX
                    ecx = fl.inc(ecx, 32);
                    // 00e2c707: ADD ESI,0x1c
                    esi = fl.add(esi, 0x1c_u32, 32);
                    // 00e2c70a: SUB EBX,-0x80
                    ebx = fl.sub(ebx, 0xffffff80_u32, 32);
                    // 00e2c70d: CMP ECX,dword ptr [ESP + 0x24]
                    fl.sub(ecx, e.mem.u32(esp.wrapping_add(0x24)), 32);
                    // 00e2c711: MOV dword ptr [ESP + 0x3c],ECX
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx);
                    // 00e2c715: JL 0x00e2c666
                    if fl.sf != fl.of {
                        pc = 0x00e2_c666;
                        continue;
                    }
                    pc = 0x00e2_c71b;
                }
                0x00e2_c71b => {
                    // 00e2c71b: MOV EDI,dword ptr [ESP + 0x24]
                    edi = e.mem.u32(esp.wrapping_add(0x24));
                    pc = 0x00e2_c71f;
                }
                0x00e2_c71f => {
                    // 00e2c71f: MOV EAX,dword ptr [EBP + 0x14]
                    eax = e.mem.u32(ebp.wrapping_add(0x14));
                    // 00e2c722: MOV ECX,dword ptr [ESP + 0x34]
                    ecx = e.mem.u32(esp.wrapping_add(0x34));
                    // 00e2c726: MOV EDX,dword ptr [ESP + 0x30]
                    edx = e.mem.u32(esp.wrapping_add(0x30));
                    // 00e2c72a: MOV ESI,dword ptr [ESP + 0x2c]
                    esi = e.mem.u32(esp.wrapping_add(0x2c));
                    // 00e2c72e: MOV EBX,dword ptr [ESP + 0x28]
                    ebx = e.mem.u32(esp.wrapping_add(0x28));
                    // 00e2c732: SHL ECX,0x5
                    ecx = fl.shift_shl(ecx, 0x5_u32, 32);
                    // 00e2c735: ADD ECX,EAX
                    ecx = fl.add(ecx, eax, 32);
                    // 00e2c737: SHL EDX,0x5
                    edx = fl.shift_shl(edx, 0x5_u32, 32);
                    // 00e2c73a: ADD EDX,EAX
                    edx = fl.add(edx, eax, 32);
                    // 00e2c73c: SHL EBX,0x5
                    ebx = fl.shift_shl(ebx, 0x5_u32, 32);
                    // 00e2c73f: SHL ESI,0x5
                    esi = fl.shift_shl(esi, 0x5_u32, 32);
                    // 00e2c742: ADD ESI,EAX
                    esi = fl.add(esi, eax, 32);
                    // 00e2c744: ADD EAX,EBX
                    eax = fl.add(eax, ebx, 32);
                    // 00e2c746: MOV EBX,dword ptr [EDI*0x4 + 0x11b3830]
                    ebx = e.mem.u32(edi.wrapping_mul(4).wrapping_add(0x11b3830));
                    // 00e2c74d: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2c74e: LEA EBX,[ECX + 0x10]
                    ebx = ecx.wrapping_add(0x10);
                    // 00e2c751: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2c752: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2c753: LEA ECX,[EDX + 0x10]
                    ecx = edx.wrapping_add(0x10);
                    // 00e2c756: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2c757: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2c758: LEA EDX,[ESI + 0x10]
                    edx = esi.wrapping_add(0x10);
                    // 00e2c75b: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2c75c: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2c75d: LEA ECX,[EAX + 0x10]
                    ecx = eax.wrapping_add(0x10);
                    // 00e2c760: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2c761: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2c762: LEA EDX,[ESP + 0x94]
                    edx = esp.wrapping_add(0x94);
                    // 00e2c769: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2c76a: CALL 0x00e2baf0
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                        e.mem.u32(esp.wrapping_add(12)),
                        e.mem.u32(esp.wrapping_add(16)),
                        e.mem.u32(esp.wrapping_add(20)),
                        e.mem.u32(esp.wrapping_add(24)),
                        e.mem.u32(esp.wrapping_add(28)),
                        e.mem.u32(esp.wrapping_add(32)),
                        e.mem.u32(esp.wrapping_add(36)),
                    ];
                    let result = e.call(0x00e2baf0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2c76f: MOV EBX,dword ptr [EBP + 0x18]
                    ebx = e.mem.u32(ebp.wrapping_add(0x18));
                    // 00e2c772: XOR ESI,ESI
                    esi = fl.xor(esi, esi, 32);
                    // 00e2c774: ADD ESP,0x28
                    esp = fl.add(esp, 0x28_u32, 32);
                    // 00e2c777: CMP EBX,ESI
                    fl.sub(ebx, esi, 32);
                    // 00e2c779: JZ 0x00e2c784
                    if fl.zf {
                        pc = 0x00e2_c784;
                        continue;
                    }
                    pc = 0x00e2_c77b;
                }
                0x00e2_c77b => {
                    // 00e2c77b: LEA EAX,[EBX + 0xc]
                    eax = ebx.wrapping_add(0xc);
                    // 00e2c77e: MOV dword ptr [ESP + 0x40],EAX
                    e.mem.set_u32(esp.wrapping_add(0x40), eax);
                    // 00e2c782: JMP 0x00e2c788
                    pc = 0x00e2_c788;
                    continue;
                }
                0x00e2_c784 => {
                    // 00e2c784: MOV dword ptr [ESP + 0x40],ESI
                    e.mem.set_u32(esp.wrapping_add(0x40), esi);
                    pc = 0x00e2_c788;
                }
                0x00e2_c788 => {
                    // 00e2c788: LEA EAX,[EBX + 0x80]
                    eax = ebx.wrapping_add(0x80);
                    // 00e2c78e: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2c790: JZ 0x00e2c79e
                    if fl.zf {
                        pc = 0x00e2_c79e;
                        continue;
                    }
                    pc = 0x00e2_c792;
                }
                0x00e2_c792 => {
                    // 00e2c792: LEA ECX,[EBX + 0x8c]
                    ecx = ebx.wrapping_add(0x8c);
                    // 00e2c798: MOV dword ptr [ESP + 0x44],ECX
                    e.mem.set_u32(esp.wrapping_add(0x44), ecx);
                    // 00e2c79c: JMP 0x00e2c7a2
                    pc = 0x00e2_c7a2;
                    continue;
                }
                0x00e2_c79e => {
                    // 00e2c79e: MOV dword ptr [ESP + 0x44],ESI
                    e.mem.set_u32(esp.wrapping_add(0x44), esi);
                    pc = 0x00e2_c7a2;
                }
                0x00e2_c7a2 => {
                    // 00e2c7a2: LEA EAX,[EBX + 0x100]
                    eax = ebx.wrapping_add(0x100);
                    // 00e2c7a8: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2c7aa: JZ 0x00e2c7b8
                    if fl.zf {
                        pc = 0x00e2_c7b8;
                        continue;
                    }
                    pc = 0x00e2_c7ac;
                }
                0x00e2_c7ac => {
                    // 00e2c7ac: LEA EDX,[EBX + 0x10c]
                    edx = ebx.wrapping_add(0x10c);
                    // 00e2c7b2: MOV dword ptr [ESP + 0x48],EDX
                    e.mem.set_u32(esp.wrapping_add(0x48), edx);
                    // 00e2c7b6: JMP 0x00e2c7bc
                    pc = 0x00e2_c7bc;
                    continue;
                }
                0x00e2_c7b8 => {
                    // 00e2c7b8: MOV dword ptr [ESP + 0x48],ESI
                    e.mem.set_u32(esp.wrapping_add(0x48), esi);
                    pc = 0x00e2_c7bc;
                }
                0x00e2_c7bc => {
                    // 00e2c7bc: LEA EAX,[EBX + 0x180]
                    eax = ebx.wrapping_add(0x180);
                    // 00e2c7c2: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2c7c4: JZ 0x00e2c7d2
                    if fl.zf {
                        pc = 0x00e2_c7d2;
                        continue;
                    }
                    pc = 0x00e2_c7c6;
                }
                0x00e2_c7c6 => {
                    // 00e2c7c6: LEA EAX,[EBX + 0x18c]
                    eax = ebx.wrapping_add(0x18c);
                    // 00e2c7cc: MOV dword ptr [ESP + 0x4c],EAX
                    e.mem.set_u32(esp.wrapping_add(0x4c), eax);
                    // 00e2c7d0: JMP 0x00e2c7d6
                    pc = 0x00e2_c7d6;
                    continue;
                }
                0x00e2_c7d2 => {
                    // 00e2c7d2: MOV dword ptr [ESP + 0x4c],ESI
                    e.mem.set_u32(esp.wrapping_add(0x4c), esi);
                    pc = 0x00e2_c7d6;
                }
                0x00e2_c7d6 => {
                    // 00e2c7d6: LEA ECX,[ESP + 0x50]
                    ecx = esp.wrapping_add(0x50);
                    // 00e2c7da: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2c7db: MOV ECX,dword ptr [EBP + 0x10]
                    ecx = e.mem.u32(ebp.wrapping_add(0x10));
                    // 00e2c7de: LEA EDX,[ESP + 0x44]
                    edx = esp.wrapping_add(0x44);
                    // 00e2c7e2: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2c7e3: MOV EDX,dword ptr [EBP + 0xc]
                    edx = e.mem.u32(ebp.wrapping_add(0xc));
                    // 00e2c7e6: LEA EAX,[ESP + 0x78]
                    eax = esp.wrapping_add(0x78);
                    // 00e2c7ea: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2c7eb: MOV EAX,dword ptr [EBP + 0x8]
                    eax = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2c7ee: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2c7ef: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2c7f0: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2c7f1: CALL 0x00d1bfa0
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                        e.mem.u32(esp.wrapping_add(12)),
                        e.mem.u32(esp.wrapping_add(16)),
                        e.mem.u32(esp.wrapping_add(20)),
                    ];
                    let result = e.call(0x00d1bfa0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2c7f6: ADD ESP,0x18
                    esp = fl.add(esp, 0x18_u32, 32);
                    // 00e2c7f9: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    // 00e2c7fb: CMP EDI,0x4
                    fl.sub(edi, 0x4_u32, 32);
                    // 00e2c7fe: JL 0x00e2c850
                    if fl.sf != fl.of {
                        pc = 0x00e2_c850;
                        continue;
                    }
                    pc = 0x00e2_c800;
                }
                0x00e2_c800 => {
                    // 00e2c800: LEA ESI,[EDI + -0x3]
                    esi = edi.wrapping_sub(0x3);
                    // 00e2c803: LEA ECX,[EBX + 0xf0]
                    ecx = ebx.wrapping_add(0xf0);
                    // 00e2c809: LEA ESP,[ESP]
                    // (no operation)
                    pc = 0x00e2_c810;
                }
                0x00e2_c810 => {
                    // 00e2c810: FLD float ptr [ECX + -0x80]
                    st.push(x87_load32(e.mem.u32(ecx.wrapping_sub(0x80))));
                    // 00e2c813: MOV EDX,dword ptr [EBP + 0x28]
                    edx = e.mem.u32(ebp.wrapping_add(0x28));
                    // 00e2c816: MOV EBX,dword ptr [ESP + EAX*0x4 + 0x28]
                    ebx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x28));
                    // 00e2c81a: FSTP float ptr [EDX + EBX*0x4]
                    e.mem.set_u32(
                        edx.wrapping_add(ebx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c81d: MOV EBX,dword ptr [ESP + EAX*0x4 + 0x2c]
                    ebx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x2c));
                    // 00e2c821: FLD float ptr [ECX]
                    st.push(x87_load32(e.mem.u32(ecx)));
                    // 00e2c823: ADD EAX,0x4
                    eax = fl.add(eax, 0x4_u32, 32);
                    // 00e2c826: FSTP float ptr [EDX + EBX*0x4]
                    e.mem.set_u32(
                        edx.wrapping_add(ebx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c829: MOV EBX,dword ptr [ESP + EAX*0x4 + 0x20]
                    ebx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x20));
                    // 00e2c82d: FLD float ptr [ECX + 0x80]
                    st.push(x87_load32(e.mem.u32(ecx.wrapping_add(0x80))));
                    // 00e2c833: ADD ECX,0x200
                    ecx = fl.add(ecx, 0x200_u32, 32);
                    // 00e2c839: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2c83b: FSTP float ptr [EDX + EBX*0x4]
                    e.mem.set_u32(
                        edx.wrapping_add(ebx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c83e: MOV EBX,dword ptr [ESP + EAX*0x4 + 0x24]
                    ebx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x24));
                    // 00e2c842: FLD float ptr [ECX + 0xffffff00]
                    st.push(x87_load32(e.mem.u32(ecx.wrapping_add(0xffffff00))));
                    // 00e2c848: FSTP float ptr [EDX + EBX*0x4]
                    e.mem.set_u32(
                        edx.wrapping_add(ebx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c84b: JL 0x00e2c810
                    if fl.sf != fl.of {
                        pc = 0x00e2_c810;
                        continue;
                    }
                    pc = 0x00e2_c84d;
                }
                0x00e2_c84d => {
                    // 00e2c84d: MOV EBX,dword ptr [EBP + 0x18]
                    ebx = e.mem.u32(ebp.wrapping_add(0x18));
                    pc = 0x00e2_c850;
                }
                0x00e2_c850 => {
                    // 00e2c850: CMP EAX,EDI
                    fl.sub(eax, edi, 32);
                    // 00e2c852: JGE 0x00e2c874
                    if fl.sf == fl.of {
                        pc = 0x00e2_c874;
                        continue;
                    }
                    pc = 0x00e2_c854;
                }
                0x00e2_c854 => {
                    // 00e2c854: MOV ECX,EAX
                    ecx = eax;
                    // 00e2c856: SHL ECX,0x7
                    ecx = fl.shift_shl(ecx, 0x7_u32, 32);
                    // 00e2c859: LEA ECX,[ECX + EBX*0x1 + 0x70]
                    ecx = ecx.wrapping_add(ebx).wrapping_add(0x70);
                    // 00e2c85d: LEA ECX,[ECX]
                    // (no operation)
                    pc = 0x00e2_c860;
                }
                0x00e2_c860 => {
                    // 00e2c860: MOV EDX,dword ptr [ESP + EAX*0x4 + 0x28]
                    edx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x28));
                    // 00e2c864: FLD float ptr [ECX]
                    st.push(x87_load32(e.mem.u32(ecx)));
                    // 00e2c866: MOV ESI,dword ptr [EBP + 0x28]
                    esi = e.mem.u32(ebp.wrapping_add(0x28));
                    // 00e2c869: INC EAX
                    eax = fl.inc(eax, 32);
                    // 00e2c86a: FSTP float ptr [ESI + EDX*0x4]
                    e.mem.set_u32(
                        esi.wrapping_add(edx.wrapping_mul(4)),
                        x87_store32(st_get(&st, 0)),
                    );
                    st.pop();
                    // 00e2c86d: SUB ECX,-0x80
                    ecx = fl.sub(ecx, 0xffffff80_u32, 32);
                    // 00e2c870: CMP EAX,EDI
                    fl.sub(eax, edi, 32);
                    // 00e2c872: JL 0x00e2c860
                    if fl.sf != fl.of {
                        pc = 0x00e2_c860;
                        continue;
                    }
                    pc = 0x00e2_c874;
                }
                0x00e2_c874 => {
                    // 00e2c874: MOV EDI,dword ptr [ESP + 0x20]
                    edi = e.mem.u32(esp.wrapping_add(0x20));
                    // 00e2c878: ADD dword ptr [ESP + 0x1c],0x10
                    e.mem.set_u32(
                        esp.wrapping_add(0x1c),
                        fl.add(e.mem.u32(esp.wrapping_add(0x1c)), 0x10_u32, 32),
                    );
                    // 00e2c87d: SUB EDI,0x4
                    edi = fl.sub(edi, 0x4_u32, 32);
                    // 00e2c880: SUB dword ptr [ESP + 0x18],0x1
                    e.mem.set_u32(
                        esp.wrapping_add(0x18),
                        fl.sub(e.mem.u32(esp.wrapping_add(0x18)), 0x1_u32, 32),
                    );
                    // 00e2c885: MOV dword ptr [ESP + 0x20],EDI
                    e.mem.set_u32(esp.wrapping_add(0x20), edi);
                    // 00e2c889: JNZ 0x00e2c630
                    if !fl.zf {
                        pc = 0x00e2_c630;
                        continue;
                    }
                    pc = 0x00e2_c88f;
                }
                0x00e2_c88f => {
                    // 00e2c88f: POP EDI
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c890: POP ESI
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c891: POP EBX
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c892: MOV ESP,EBP
                    esp = ebp;
                    // 00e2c894: POP EBP
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2c895: RET
                    return;
                }
                _ => unreachable!("pc {pc:#x}"),
            }
        }
    });
}

// Translated from 00e2c8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the driver of `00e2bbd0` (the classes are unconfirmed). Seven stack
/// words, cdecl, nothing popped: `arg0` (the function does nothing when it is zero) is an
/// object whose pointer at `+0x10` is the root record, `arg6` a count. The memory router
/// (`00c85750`) answers a stack allocator cursor in its first word: `arg6 * 4` bytes rounded up
/// to 128 are taken for a list of ones (a capacity word `arg6 | 0x80000000` is kept: "not
/// owned"), then 0x280 bytes for the node records, the first being a copy of the root record
/// with its counters set. Records are popped and handed to `00e2bbd0` with twenty words built from
/// the frame and the driver's own arguments; the children it reports (two counts) are
/// pushed as new records in the 0x40-byte slots, the cursor being restored at the end (and the
/// list given back through the allocator's slot `0x10` when it is owned). Written instruction by
/// instruction like `00e2bbd0`.
#[allow(
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_labels,
    clippy::all
)]
pub fn fn_00e2c8a0(
    e: &mut Engine,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
) {
    e.with_stack(0x1400, |e, frame| {
        // The exe's registers; `esp` points at the return address slot, the arguments above it.
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut esp = frame.addr() + 0x1400 - 0x80;
        e.mem.set_u32(esp, 0);
        e.mem.set_u32(esp + 4, arg0);
        e.mem.set_u32(esp + 8, arg1);
        e.mem.set_u32(esp + 12, arg2);
        e.mem.set_u32(esp + 16, arg3);
        e.mem.set_u32(esp + 20, arg4);
        e.mem.set_u32(esp + 24, arg5);
        e.mem.set_u32(esp + 28, arg6);
        let mut xmm = [[0u32; 4]; 8];
        let mut fl = Flags::default();
        let mut st: Vec<f64> = Vec::new();
        let mut fsw: u16 = 0;
        let mut pc = 0x00e2_c8a0_u32;
        loop {
            match pc {
                0x00e2_c8a0 => {
                    // 00e2c8a0: PUSH EBP
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp);
                    // 00e2c8a1: MOV EBP,ESP
                    ebp = esp;
                    // 00e2c8a3: AND ESP,0xfffffff0
                    esp = fl.and(esp, 0xfffffff0_u32, 32);
                    // 00e2c8a6: SUB ESP,0x8e4
                    esp = fl.sub(esp, 0x8e4_u32, 32);
                    // 00e2c8ac: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2c8ad: XOR EBX,EBX
                    ebx = fl.xor(ebx, ebx, 32);
                    // 00e2c8af: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2c8b0: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2c8b1: CMP dword ptr [EBP + 0x8],EBX
                    fl.sub(e.mem.u32(ebp.wrapping_add(0x8)), ebx, 32);
                    // 00e2c8b4: JZ 0x00e2cc23
                    if fl.zf {
                        pc = 0x00e2_cc23;
                        continue;
                    }
                    pc = 0x00e2_c8ba;
                }
                0x00e2_c8ba => {
                    // 00e2c8ba: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2c8bf: MOV ESI,dword ptr [EBP + 0x20]
                    esi = e.mem.u32(ebp.wrapping_add(0x20));
                    // 00e2c8c2: MOV EDI,dword ptr [EAX]
                    edi = e.mem.u32(eax);
                    // 00e2c8c4: LEA ECX,[ESI*0x4 + 0x7f]
                    ecx = esi.wrapping_mul(4).wrapping_add(0x7f);
                    // 00e2c8cb: AND ECX,0xffffff80
                    ecx = fl.and(ecx, 0xffffff80_u32, 32);
                    // 00e2c8ce: ADD ECX,EDI
                    ecx = fl.add(ecx, edi, 32);
                    // 00e2c8d0: MOV dword ptr [EAX],ECX
                    e.mem.set_u32(eax, ecx);
                    // 00e2c8d2: MOV EAX,ESI
                    eax = esi;
                    // 00e2c8d4: OR EAX,0x80000000
                    eax = fl.or(eax, 0x80000000_u32, 32);
                    // 00e2c8d9: CMP ESI,EBX
                    fl.sub(esi, ebx, 32);
                    // 00e2c8db: MOV dword ptr [ESP + 0x2c],EDI
                    e.mem.set_u32(esp.wrapping_add(0x2c), edi);
                    // 00e2c8df: MOV dword ptr [ESP + 0xa8],EAX
                    e.mem.set_u32(esp.wrapping_add(0xa8), eax);
                    // 00e2c8e6: JLE 0x00e2c8f1
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c8f1;
                        continue;
                    }
                    pc = 0x00e2_c8e8;
                }
                0x00e2_c8e8 => {
                    // 00e2c8e8: MOV ECX,ESI
                    ecx = esi;
                    // 00e2c8ea: MOV EAX,0x3f800000
                    eax = 0x3f800000_u32;
                    // 00e2c8ef: STOSD.REP ES:EDI
                    while ecx != 0 {
                        e.mem.set_u32(edi, eax);
                        edi = edi.wrapping_add(4);
                        ecx = ecx.wrapping_sub(1);
                    }
                    pc = 0x00e2_c8f1;
                }
                0x00e2_c8f1 => {
                    // 00e2c8f1: CALL 0x00e4e3f0
                    let words = [];
                    let result = e.call(0x00e4e3f0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2c8f6: XOR EDI,EDI
                    edi = fl.xor(edi, edi, 32);
                    // 00e2c8f8: MOV dword ptr [ESP + 0x20],EDI
                    e.mem.set_u32(esp.wrapping_add(0x20), edi);
                    // 00e2c8fc: MOV dword ptr [ESP + 0x24],EBX
                    e.mem.set_u32(esp.wrapping_add(0x24), ebx);
                    // 00e2c900: CALL 0x00e4e3f0
                    let words = [];
                    let result = e.call(0x00e4e3f0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2c905: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2c90a: MOV ECX,dword ptr [EAX]
                    ecx = e.mem.u32(eax);
                    // 00e2c90c: LEA EDX,[ECX + 0x280]
                    edx = ecx.wrapping_add(0x280);
                    // 00e2c912: MOV dword ptr [ESP + 0x30],EBX
                    e.mem.set_u32(esp.wrapping_add(0x30), ebx);
                    // 00e2c916: MOV dword ptr [ESP + 0x28],ECX
                    e.mem.set_u32(esp.wrapping_add(0x28), ecx);
                    // 00e2c91a: MOV dword ptr [EAX],EDX
                    e.mem.set_u32(eax, edx);
                    // 00e2c91c: LEA EBX,[EDI + 0x1]
                    ebx = edi.wrapping_add(0x1);
                    // 00e2c91f: TEST ECX,ECX
                    fl.and(ecx, ecx, 32);
                    // 00e2c921: JZ 0x00e2c929
                    if fl.zf {
                        pc = 0x00e2_c929;
                        continue;
                    }
                    pc = 0x00e2_c923;
                }
                0x00e2_c923 => {
                    // 00e2c923: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    // 00e2c925: MOV word ptr [ECX + 0x24],AX
                    e.mem.set_u16(ecx.wrapping_add(0x24), (eax & 0xffff) as u16);
                    pc = 0x00e2_c929;
                }
                0x00e2_c929 => {
                    // 00e2c929: MOV EDX,dword ptr [EBP + 0x8]
                    edx = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2c92c: MOV EAX,dword ptr [EDX + 0x10]
                    eax = e.mem.u32(edx.wrapping_add(0x10));
                    // 00e2c92f: MOVAPS XMM0,xmmword ptr [EAX]
                    xmm[0] = load_lanes(e, eax);
                    // 00e2c932: MOVAPS xmmword ptr [ECX],XMM0
                    store_lanes(e, ecx, xmm[0]);
                    // 00e2c935: MOVAPS XMM0,xmmword ptr [EAX + 0x10]
                    xmm[0] = load_lanes(e, eax.wrapping_add(0x10));
                    // 00e2c939: MOVAPS xmmword ptr [ECX + 0x10],XMM0
                    store_lanes(e, ecx.wrapping_add(0x10), xmm[0]);
                    // 00e2c93d: MOV EDX,dword ptr [EAX + 0x20]
                    edx = e.mem.u32(eax.wrapping_add(0x20));
                    // 00e2c940: MOV dword ptr [ECX + 0x20],EDX
                    e.mem.set_u32(ecx.wrapping_add(0x20), edx);
                    // 00e2c943: MOV EDX,dword ptr [EAX + 0x20]
                    edx = e.mem.u32(eax.wrapping_add(0x20));
                    // 00e2c946: MOV dword ptr [ECX + 0x20],EDX
                    e.mem.set_u32(ecx.wrapping_add(0x20), edx);
                    // 00e2c949: MOV DX,word ptr [EAX + 0x24]
                    edx = (edx & 0xffff_0000)
                        | ((u32::from(e.mem.u16(eax.wrapping_add(0x24)))) & 0xffff);
                    // 00e2c94d: MOV word ptr [ECX + 0x24],DX
                    e.mem.set_u16(ecx.wrapping_add(0x24), (edx & 0xffff) as u16);
                    // 00e2c951: MOV AX,word ptr [EAX + 0x26]
                    eax = (eax & 0xffff_0000)
                        | ((u32::from(e.mem.u16(eax.wrapping_add(0x26)))) & 0xffff);
                    // 00e2c955: MOV word ptr [ECX + 0x26],AX
                    e.mem.set_u16(ecx.wrapping_add(0x26), (eax & 0xffff) as u16);
                    // 00e2c959: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    // 00e2c95b: CMP ESI,EAX
                    fl.sub(esi, eax, 32);
                    // 00e2c95d: MOV dword ptr [ECX + 0x30],EAX
                    e.mem.set_u32(ecx.wrapping_add(0x30), eax);
                    // 00e2c960: MOV dword ptr [ECX + 0x34],EAX
                    e.mem.set_u32(ecx.wrapping_add(0x34), eax);
                    // 00e2c963: MOV dword ptr [ECX + 0x38],ESI
                    e.mem.set_u32(ecx.wrapping_add(0x38), esi);
                    // 00e2c966: JLE 0x00e2c981
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_c981;
                        continue;
                    }
                    pc = 0x00e2_c968;
                }
                0x00e2_c968 => {
                    // 00e2c968: JMP 0x00e2c970
                    pc = 0x00e2_c970;
                    continue;
                }
                0x00e2_c970 => {
                    // 00e2c970: MOV dword ptr [ESP + EDI*0x4 + 0x120],EAX
                    e.mem.set_u32(
                        esp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x120),
                        eax,
                    );
                    // 00e2c977: INC EAX
                    eax = fl.inc(eax, 32);
                    // 00e2c978: INC EDI
                    edi = fl.inc(edi, 32);
                    // 00e2c979: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2c97b: JL 0x00e2c970
                    if fl.sf != fl.of {
                        pc = 0x00e2_c970;
                        continue;
                    }
                    pc = 0x00e2_c97d;
                }
                0x00e2_c97d => {
                    // 00e2c97d: MOV dword ptr [ESP + 0x20],EDI
                    e.mem.set_u32(esp.wrapping_add(0x20), edi);
                    pc = 0x00e2_c981;
                }
                0x00e2_c981 => {
                    // 00e2c981: MOV dword ptr [ESP + 0x18],0x40
                    e.mem.set_u32(esp.wrapping_add(0x18), 0x40_u32);
                    // 00e2c989: MOV dword ptr [ESP + 0x14],0x9
                    e.mem.set_u32(esp.wrapping_add(0x14), 0x9_u32);
                    // 00e2c991: MOV EAX,ECX
                    eax = ecx;
                    // 00e2c993: JMP 0x00e2c999
                    pc = 0x00e2_c999;
                    continue;
                }
                0x00e2_c995 => {
                    // 00e2c995: MOV EAX,dword ptr [ESP + 0x1c]
                    eax = e.mem.u32(esp.wrapping_add(0x1c));
                    pc = 0x00e2_c999;
                }
                0x00e2_c999 => {
                    // 00e2c999: MOV ESI,EAX
                    esi = eax;
                    // 00e2c99b: XOR EDX,EDX
                    edx = fl.xor(edx, edx, 32);
                    // 00e2c99d: MOV word ptr [ESP + 0x94],DX
                    e.mem.set_u16(esp.wrapping_add(0x94), (edx & 0xffff) as u16);
                    // 00e2c9a5: SUB EAX,0x40
                    eax = fl.sub(eax, 0x40_u32, 32);
                    // 00e2c9a8: MOV ECX,0x10
                    ecx = 0x10_u32;
                    // 00e2c9ad: LEA EDI,[ESP + 0xe0]
                    edi = esp.wrapping_add(0xe0);
                    // 00e2c9b4: MOVSD.REP ES:EDI,ESI
                    while ecx != 0 {
                        let word = e.mem.u32(esi);
                        e.mem.set_u32(edi, word);
                        esi = esi.wrapping_add(4);
                        edi = edi.wrapping_add(4);
                        ecx = ecx.wrapping_sub(1);
                    }
                    // 00e2c9b6: MOV dword ptr [ESP + 0x1c],EAX
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax);
                    // 00e2c9ba: MOV EAX,dword ptr [ESP + 0x114]
                    eax = e.mem.u32(esp.wrapping_add(0x114));
                    // 00e2c9c1: MOV ECX,0xc
                    ecx = 0xc_u32;
                    // 00e2c9c6: LEA ESI,[ESP + 0xe0]
                    esi = esp.wrapping_add(0xe0);
                    // 00e2c9cd: LEA EDI,[ESP + 0xb0]
                    edi = esp.wrapping_add(0xb0);
                    // 00e2c9d4: MOVSD.REP ES:EDI,ESI
                    while ecx != 0 {
                        let word = e.mem.u32(esi);
                        e.mem.set_u32(edi, word);
                        esi = esi.wrapping_add(4);
                        edi = edi.wrapping_add(4);
                        ecx = ecx.wrapping_sub(1);
                    }
                    // 00e2c9d6: XOR ECX,ECX
                    ecx = fl.xor(ecx, ecx, 32);
                    // 00e2c9d8: MOV word ptr [ESP + 0x64],CX
                    e.mem.set_u16(esp.wrapping_add(0x64), (ecx & 0xffff) as u16);
                    // 00e2c9dd: MOV ECX,dword ptr [ESP + 0x118]
                    ecx = e.mem.u32(esp.wrapping_add(0x118));
                    // 00e2c9e4: SUB dword ptr [ESP + EAX*0x4 + 0x20],ECX
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x20),
                        fl.sub(
                            e.mem
                                .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x20)),
                            ecx,
                            32,
                        ),
                    );
                    // 00e2c9e8: MOV EDX,dword ptr [ESP + EAX*0x4 + 0x20]
                    edx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x20));
                    // 00e2c9ec: IMUL EAX,EAX,0xfa
                    eax = (eax as i32).wrapping_mul(250_i32) as u32;
                    // 00e2c9f2: MOV ESI,dword ptr [ESP + 0x24]
                    esi = e.mem.u32(esp.wrapping_add(0x24));
                    // 00e2c9f6: MOV EDI,dword ptr [ESP + 0x20]
                    edi = e.mem.u32(esp.wrapping_add(0x20));
                    // 00e2c9fa: ADD EAX,EDX
                    eax = fl.add(eax, edx, 32);
                    // 00e2c9fc: INC dword ptr [ESP + 0x14]
                    e.mem.set_u32(
                        esp.wrapping_add(0x14),
                        fl.inc(e.mem.u32(esp.wrapping_add(0x14)), 32),
                    );
                    // 00e2ca00: LEA EDX,[ESP + 0x70]
                    edx = esp.wrapping_add(0x70);
                    // 00e2ca04: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca05: LEA EDX,[ESP + 0x44]
                    edx = esp.wrapping_add(0x44);
                    // 00e2ca09: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca0a: LEA EDX,[ESP + 0x44]
                    edx = esp.wrapping_add(0x44);
                    // 00e2ca0e: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca0f: LEA EDX,[ESP + ESI*0x4 + 0x514]
                    edx = esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x514);
                    // 00e2ca16: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca17: LEA EDX,[ESP + 0x44]
                    edx = esp.wrapping_add(0x44);
                    // 00e2ca1b: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca1c: LEA EDX,[ESP + EDI*0x4 + 0x134]
                    edx = esp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x134);
                    // 00e2ca23: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca24: MOV EDX,0xfa
                    edx = 0xfa_u32;
                    // 00e2ca29: SUB EDX,ESI
                    edx = fl.sub(edx, esi, 32);
                    // 00e2ca2b: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca2c: SUB dword ptr [ESP + 0x34],0x40
                    e.mem.set_u32(
                        esp.wrapping_add(0x34),
                        fl.sub(e.mem.u32(esp.wrapping_add(0x34)), 0x40_u32, 32),
                    );
                    // 00e2ca31: MOV EDX,0xfa
                    edx = 0xfa_u32;
                    // 00e2ca36: SUB EDX,EDI
                    edx = fl.sub(edx, edi, 32);
                    // 00e2ca38: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca39: MOV EDX,dword ptr [ESP + 0x34]
                    edx = e.mem.u32(esp.wrapping_add(0x34));
                    // 00e2ca3d: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca3e: MOV EDX,dword ptr [EBP + 0x14]
                    edx = e.mem.u32(ebp.wrapping_add(0x14));
                    // 00e2ca41: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2ca42: MOV ECX,dword ptr [ESP + 0x54]
                    ecx = e.mem.u32(esp.wrapping_add(0x54));
                    // 00e2ca46: LEA EAX,[ESP + EAX*0x4 + 0x148]
                    eax = esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x148);
                    // 00e2ca4d: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2ca4e: MOV EAX,dword ptr [EBP + 0x10]
                    eax = e.mem.u32(ebp.wrapping_add(0x10));
                    // 00e2ca51: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2ca52: MOV ECX,dword ptr [EBP + 0x1c]
                    ecx = e.mem.u32(ebp.wrapping_add(0x1c));
                    // 00e2ca55: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca56: MOV EDX,dword ptr [EBP + 0x18]
                    edx = e.mem.u32(ebp.wrapping_add(0x18));
                    // 00e2ca59: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2ca5a: MOV EAX,dword ptr [EBP + 0xc]
                    eax = e.mem.u32(ebp.wrapping_add(0xc));
                    // 00e2ca5d: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2ca5e: MOV ECX,dword ptr [ESP + 0x14c]
                    ecx = e.mem.u32(esp.wrapping_add(0x14c));
                    // 00e2ca65: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca66: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2ca67: MOV EAX,dword ptr [EBP + 0x8]
                    eax = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2ca6a: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2ca6b: LEA EDX,[ESP + 0xf8]
                    edx = esp.wrapping_add(0xf8);
                    // 00e2ca72: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ca73: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2ca74: DEC EBX
                    ebx = fl.dec(ebx, 32);
                    // 00e2ca75: CALL 0x00e2bbd0
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                        e.mem.u32(esp.wrapping_add(12)),
                        e.mem.u32(esp.wrapping_add(16)),
                        e.mem.u32(esp.wrapping_add(20)),
                        e.mem.u32(esp.wrapping_add(24)),
                        e.mem.u32(esp.wrapping_add(28)),
                        e.mem.u32(esp.wrapping_add(32)),
                        e.mem.u32(esp.wrapping_add(36)),
                        e.mem.u32(esp.wrapping_add(40)),
                        e.mem.u32(esp.wrapping_add(44)),
                        e.mem.u32(esp.wrapping_add(48)),
                        e.mem.u32(esp.wrapping_add(52)),
                        e.mem.u32(esp.wrapping_add(56)),
                        e.mem.u32(esp.wrapping_add(60)),
                        e.mem.u32(esp.wrapping_add(64)),
                        e.mem.u32(esp.wrapping_add(68)),
                        e.mem.u32(esp.wrapping_add(72)),
                        e.mem.u32(esp.wrapping_add(76)),
                    ];
                    let result = e.call(0x00e2bbd0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2ca7a: MOV EAX,dword ptr [ESP + 0x84]
                    eax = e.mem.u32(esp.wrapping_add(0x84));
                    // 00e2ca81: ADD EDI,EAX
                    edi = fl.add(edi, eax, 32);
                    // 00e2ca83: MOV dword ptr [ESP + 0x70],EDI
                    e.mem.set_u32(esp.wrapping_add(0x70), edi);
                    // 00e2ca87: MOV EDI,dword ptr [ESP + 0x8c]
                    edi = e.mem.u32(esp.wrapping_add(0x8c));
                    // 00e2ca8e: ADD ESI,EDI
                    esi = fl.add(esi, edi, 32);
                    // 00e2ca90: ADD ESP,0x50
                    esp = fl.add(esp, 0x50_u32, 32);
                    // 00e2ca93: MOV dword ptr [ESP + 0x24],ESI
                    e.mem.set_u32(esp.wrapping_add(0x24), esi);
                    // 00e2ca97: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2ca99: JZ 0x00e2cb29
                    if fl.zf {
                        pc = 0x00e2_cb29;
                        continue;
                    }
                    pc = 0x00e2_ca9f;
                }
                0x00e2_ca9f => {
                    // 00e2ca9f: MOV EAX,dword ptr [ESP + 0x18]
                    eax = e.mem.u32(esp.wrapping_add(0x18));
                    // 00e2caa3: ADD dword ptr [ESP + 0x1c],0x40
                    e.mem.set_u32(
                        esp.wrapping_add(0x1c),
                        fl.add(e.mem.u32(esp.wrapping_add(0x1c)), 0x40_u32, 32),
                    );
                    // 00e2caa8: DEC dword ptr [ESP + 0x14]
                    e.mem.set_u32(
                        esp.wrapping_add(0x14),
                        fl.dec(e.mem.u32(esp.wrapping_add(0x14)), 32),
                    );
                    // 00e2caac: MOV EDX,EAX
                    edx = eax;
                    // 00e2caae: ADD EAX,0x40
                    eax = fl.add(eax, 0x40_u32, 32);
                    // 00e2cab1: MOV ECX,EBX
                    ecx = ebx;
                    // 00e2cab3: INC EBX
                    ebx = fl.inc(ebx, 32);
                    // 00e2cab4: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2cab8: MOV EAX,EBX
                    eax = ebx;
                    // 00e2caba: SUB EAX,ECX
                    eax = fl.sub(eax, ecx, 32);
                    // 00e2cabc: MOV ECX,dword ptr [ESP + 0x28]
                    ecx = e.mem.u32(esp.wrapping_add(0x28));
                    // 00e2cac0: LEA ESI,[EDX + ECX*0x1]
                    esi = edx.wrapping_add(ecx);
                    // 00e2cac3: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2cac5: JLE 0x00e2cadb
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_cadb;
                        continue;
                    }
                    pc = 0x00e2_cac7;
                }
                0x00e2_cac7 => {
                    // 00e2cac7: MOV ECX,ESI
                    ecx = esi;
                    pc = 0x00e2_cac9;
                }
                0x00e2_cac9 => {
                    // 00e2cac9: TEST ECX,ECX
                    fl.and(ecx, ecx, 32);
                    // 00e2cacb: JZ 0x00e2cad3
                    if fl.zf {
                        pc = 0x00e2_cad3;
                        continue;
                    }
                    pc = 0x00e2_cacd;
                }
                0x00e2_cacd => {
                    // 00e2cacd: XOR EDX,EDX
                    edx = fl.xor(edx, edx, 32);
                    // 00e2cacf: MOV word ptr [ECX + 0x24],DX
                    e.mem.set_u16(ecx.wrapping_add(0x24), (edx & 0xffff) as u16);
                    pc = 0x00e2_cad3;
                }
                0x00e2_cad3 => {
                    // 00e2cad3: ADD ECX,0x40
                    ecx = fl.add(ecx, 0x40_u32, 32);
                    // 00e2cad6: SUB EAX,0x1
                    eax = fl.sub(eax, 0x1_u32, 32);
                    // 00e2cad9: JNZ 0x00e2cac9
                    if !fl.zf {
                        pc = 0x00e2_cac9;
                        continue;
                    }
                    pc = 0x00e2_cadb;
                }
                0x00e2_cadb => {
                    // 00e2cadb: MOV EAX,dword ptr [ESP + 0x60]
                    eax = e.mem.u32(esp.wrapping_add(0x60));
                    // 00e2cadf: MOV CX,word ptr [ESP + 0x64]
                    ecx = (ecx & 0xffff_0000)
                        | ((u32::from(e.mem.u16(esp.wrapping_add(0x64)))) & 0xffff);
                    // 00e2cae4: MOVAPS XMM0,xmmword ptr [ESP + 0x40]
                    xmm[0] = load_lanes(e, esp.wrapping_add(0x40));
                    // 00e2cae9: MOV DX,word ptr [ESP + 0x66]
                    edx = (edx & 0xffff_0000)
                        | ((u32::from(e.mem.u16(esp.wrapping_add(0x66)))) & 0xffff);
                    // 00e2caee: MOV word ptr [ESI + 0x24],CX
                    e.mem.set_u16(esi.wrapping_add(0x24), (ecx & 0xffff) as u16);
                    // 00e2caf2: MOV ECX,dword ptr [EBP + 0x8]
                    ecx = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2caf5: MOV dword ptr [ESI + 0x20],EAX
                    e.mem.set_u32(esi.wrapping_add(0x20), eax);
                    // 00e2caf8: MOV EAX,dword ptr [ESP + 0xd2]
                    eax = e.mem.u32(esp.wrapping_add(0xd2));
                    // 00e2caff: MOVAPS xmmword ptr [ESI],XMM0
                    store_lanes(e, esi, xmm[0]);
                    // 00e2cb02: MOVAPS XMM0,xmmword ptr [ESP + 0x50]
                    xmm[0] = load_lanes(e, esp.wrapping_add(0x50));
                    // 00e2cb07: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2cb08: ADD ECX,0x1c
                    ecx = fl.add(ecx, 0x1c_u32, 32);
                    // 00e2cb0b: MOVAPS xmmword ptr [ESI + 0x10],XMM0
                    store_lanes(e, esi.wrapping_add(0x10), xmm[0]);
                    // 00e2cb0f: MOV word ptr [ESI + 0x26],DX
                    e.mem.set_u16(esi.wrapping_add(0x26), (edx & 0xffff) as u16);
                    // 00e2cb13: CALL 0x00d1bb40
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d1bb40, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2cb18: MOV ECX,dword ptr [ESP + 0x34]
                    ecx = e.mem.u32(esp.wrapping_add(0x34));
                    // 00e2cb1c: MOV dword ptr [ESI + 0x30],EAX
                    e.mem.set_u32(esi.wrapping_add(0x30), eax);
                    // 00e2cb1f: MOV dword ptr [ESI + 0x34],0x0
                    e.mem.set_u32(esi.wrapping_add(0x34), 0x0_u32);
                    // 00e2cb26: MOV dword ptr [ESI + 0x38],ECX
                    e.mem.set_u32(esi.wrapping_add(0x38), ecx);
                    pc = 0x00e2_cb29;
                }
                0x00e2_cb29 => {
                    // 00e2cb29: TEST EDI,EDI
                    fl.and(edi, edi, 32);
                    // 00e2cb2b: JZ 0x00e2cbd1
                    if fl.zf {
                        pc = 0x00e2_cbd1;
                        continue;
                    }
                    pc = 0x00e2_cb31;
                }
                0x00e2_cb31 => {
                    // 00e2cb31: MOV EAX,dword ptr [ESP + 0x18]
                    eax = e.mem.u32(esp.wrapping_add(0x18));
                    // 00e2cb35: ADD dword ptr [ESP + 0x1c],0x40
                    e.mem.set_u32(
                        esp.wrapping_add(0x1c),
                        fl.add(e.mem.u32(esp.wrapping_add(0x1c)), 0x40_u32, 32),
                    );
                    // 00e2cb3a: DEC dword ptr [ESP + 0x14]
                    e.mem.set_u32(
                        esp.wrapping_add(0x14),
                        fl.dec(e.mem.u32(esp.wrapping_add(0x14)), 32),
                    );
                    // 00e2cb3e: MOV EDX,EAX
                    edx = eax;
                    // 00e2cb40: ADD EAX,0x40
                    eax = fl.add(eax, 0x40_u32, 32);
                    // 00e2cb43: MOV ECX,EBX
                    ecx = ebx;
                    // 00e2cb45: INC EBX
                    ebx = fl.inc(ebx, 32);
                    // 00e2cb46: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(esp.wrapping_add(0x18), eax);
                    // 00e2cb4a: MOV EAX,EBX
                    eax = ebx;
                    // 00e2cb4c: SUB EAX,ECX
                    eax = fl.sub(eax, ecx, 32);
                    // 00e2cb4e: MOV ECX,dword ptr [ESP + 0x28]
                    ecx = e.mem.u32(esp.wrapping_add(0x28));
                    // 00e2cb52: LEA ESI,[EDX + ECX*0x1]
                    esi = edx.wrapping_add(ecx);
                    // 00e2cb55: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2cb57: JLE 0x00e2cb6d
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_cb6d;
                        continue;
                    }
                    pc = 0x00e2_cb59;
                }
                0x00e2_cb59 => {
                    // 00e2cb59: MOV ECX,ESI
                    ecx = esi;
                    pc = 0x00e2_cb5b;
                }
                0x00e2_cb5b => {
                    // 00e2cb5b: TEST ECX,ECX
                    fl.and(ecx, ecx, 32);
                    // 00e2cb5d: JZ 0x00e2cb65
                    if fl.zf {
                        pc = 0x00e2_cb65;
                        continue;
                    }
                    pc = 0x00e2_cb5f;
                }
                0x00e2_cb5f => {
                    // 00e2cb5f: XOR EDX,EDX
                    edx = fl.xor(edx, edx, 32);
                    // 00e2cb61: MOV word ptr [ECX + 0x24],DX
                    e.mem.set_u16(ecx.wrapping_add(0x24), (edx & 0xffff) as u16);
                    pc = 0x00e2_cb65;
                }
                0x00e2_cb65 => {
                    // 00e2cb65: ADD ECX,0x40
                    ecx = fl.add(ecx, 0x40_u32, 32);
                    // 00e2cb68: SUB EAX,0x1
                    eax = fl.sub(eax, 0x1_u32, 32);
                    // 00e2cb6b: JNZ 0x00e2cb5b
                    if !fl.zf {
                        pc = 0x00e2_cb5b;
                        continue;
                    }
                    pc = 0x00e2_cb6d;
                }
                0x00e2_cb6d => {
                    // 00e2cb6d: MOV EAX,dword ptr [ESP + 0x90]
                    eax = e.mem.u32(esp.wrapping_add(0x90));
                    // 00e2cb74: MOV CX,word ptr [ESP + 0x94]
                    ecx = (ecx & 0xffff_0000)
                        | ((u32::from(e.mem.u16(esp.wrapping_add(0x94)))) & 0xffff);
                    // 00e2cb7c: MOVAPS XMM0,xmmword ptr [ESP + 0x70]
                    xmm[0] = load_lanes(e, esp.wrapping_add(0x70));
                    // 00e2cb81: MOV DX,word ptr [ESP + 0x96]
                    edx = (edx & 0xffff_0000)
                        | ((u32::from(e.mem.u16(esp.wrapping_add(0x96)))) & 0xffff);
                    // 00e2cb89: MOV dword ptr [ESI + 0x20],EAX
                    e.mem.set_u32(esi.wrapping_add(0x20), eax);
                    // 00e2cb8c: MOV EAX,dword ptr [ESP + 0xd6]
                    eax = e.mem.u32(esp.wrapping_add(0xd6));
                    // 00e2cb93: MOV word ptr [ESI + 0x24],CX
                    e.mem.set_u16(esi.wrapping_add(0x24), (ecx & 0xffff) as u16);
                    // 00e2cb97: MOV ECX,0x7fff
                    ecx = 0x7fff_u32;
                    // 00e2cb9c: MOVAPS xmmword ptr [ESI],XMM0
                    store_lanes(e, esi, xmm[0]);
                    // 00e2cb9f: MOVAPS XMM0,xmmword ptr [ESP + 0x80]
                    xmm[0] = load_lanes(e, esp.wrapping_add(0x80));
                    // 00e2cba7: MOVAPS xmmword ptr [ESI + 0x10],XMM0
                    store_lanes(e, esi.wrapping_add(0x10), xmm[0]);
                    // 00e2cbab: MOV word ptr [ESI + 0x26],DX
                    e.mem.set_u16(esi.wrapping_add(0x26), (edx & 0xffff) as u16);
                    // 00e2cbaf: CMP AX,CX
                    fl.sub(eax & 0xffff, ecx & 0xffff, 16);
                    // 00e2cbb2: JZ 0x00e2cbc2
                    if fl.zf {
                        pc = 0x00e2_cbc2;
                        continue;
                    }
                    pc = 0x00e2_cbb4;
                }
                0x00e2_cbb4 => {
                    // 00e2cbb4: MOV ECX,dword ptr [EBP + 0x8]
                    ecx = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2cbb7: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2cbb8: ADD ECX,0x1c
                    ecx = fl.add(ecx, 0x1c_u32, 32);
                    // 00e2cbbb: CALL 0x00d1bb40
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d1bb40, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2cbc0: JMP 0x00e2cbc4
                    pc = 0x00e2_cbc4;
                    continue;
                }
                0x00e2_cbc2 => {
                    // 00e2cbc2: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    pc = 0x00e2_cbc4;
                }
                0x00e2_cbc4 => {
                    // 00e2cbc4: MOV dword ptr [ESI + 0x30],EAX
                    e.mem.set_u32(esi.wrapping_add(0x30), eax);
                    // 00e2cbc7: MOV dword ptr [ESI + 0x34],0x1
                    e.mem.set_u32(esi.wrapping_add(0x34), 0x1_u32);
                    // 00e2cbce: MOV dword ptr [ESI + 0x38],EDI
                    e.mem.set_u32(esi.wrapping_add(0x38), edi);
                    pc = 0x00e2_cbd1;
                }
                0x00e2_cbd1 => {
                    // 00e2cbd1: CMP dword ptr [ESP + 0x30],EBX
                    fl.sub(e.mem.u32(esp.wrapping_add(0x30)), ebx, 32);
                    // 00e2cbd5: JG 0x00e2cbdb
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e2_cbdb;
                        continue;
                    }
                    pc = 0x00e2_cbd7;
                }
                0x00e2_cbd7 => {
                    // 00e2cbd7: MOV dword ptr [ESP + 0x30],EBX
                    e.mem.set_u32(esp.wrapping_add(0x30), ebx);
                    pc = 0x00e2_cbdb;
                }
                0x00e2_cbdb => {
                    // 00e2cbdb: TEST EBX,EBX
                    fl.and(ebx, ebx, 32);
                    // 00e2cbdd: JNZ 0x00e2c995
                    if !fl.zf {
                        pc = 0x00e2_c995;
                        continue;
                    }
                    pc = 0x00e2_cbe3;
                }
                0x00e2_cbe3 => {
                    // 00e2cbe3: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cbe8: MOV EDX,dword ptr [ESP + 0x28]
                    edx = e.mem.u32(esp.wrapping_add(0x28));
                    // 00e2cbec: MOV dword ptr [EAX],EDX
                    e.mem.set_u32(eax, edx);
                    // 00e2cbee: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cbf3: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cbf8: MOV ESI,dword ptr [ESP + 0x2c]
                    esi = e.mem.u32(esp.wrapping_add(0x2c));
                    // 00e2cbfc: MOV dword ptr [EAX],ESI
                    e.mem.set_u32(eax, esi);
                    // 00e2cbfe: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cc03: MOV ECX,dword ptr [EAX + 0x10]
                    ecx = e.mem.u32(eax.wrapping_add(0x10));
                    // 00e2cc06: MOV EAX,dword ptr [ESP + 0xa8]
                    eax = e.mem.u32(esp.wrapping_add(0xa8));
                    // 00e2cc0d: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2cc0f: JS 0x00e2cc23
                    if fl.sf {
                        pc = 0x00e2_cc23;
                        continue;
                    }
                    pc = 0x00e2_cc11;
                }
                0x00e2_cc11 => {
                    // 00e2cc11: MOV EDX,dword ptr [ECX]
                    edx = e.mem.u32(ecx);
                    // 00e2cc13: AND EAX,0x3fffffff
                    eax = fl.and(eax, 0x3fffffff_u32, 32);
                    // 00e2cc18: ADD EAX,EAX
                    eax = fl.add(eax, eax, 32);
                    // 00e2cc1a: ADD EAX,EAX
                    eax = fl.add(eax, eax, 32);
                    // 00e2cc1c: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2cc1d: MOV EAX,dword ptr [EDX + 0x10]
                    eax = e.mem.u32(edx.wrapping_add(0x10));
                    // 00e2cc20: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2cc21: CALL EAX
                    let words = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                    let result = e.call(eax, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(8);
                    st.push(result.st0);
                    pc = 0x00e2_cc23;
                }
                0x00e2_cc23 => {
                    // 00e2cc23: POP EDI
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2cc24: POP ESI
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2cc25: POP EBX
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2cc26: MOV ESP,EBP
                    esp = ebp;
                    // 00e2cc28: POP EBP
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2cc29: RET
                    return;
                }
                _ => unreachable!("pc {pc:#x}"),
            }
        }
    });
}

// Translated from 00e2cc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the driver of `00e2c100`, the same as `00e2c8a0` for 0x20-byte
/// records (the 16-bit-coordinate variant): it does nothing when `arg0` is zero, takes the same
/// blocks from the stack allocator (0x180 bytes for the records) and runs `00e2c100`; the
/// child records are built with `00d9a4d0`. Seven stack words, cdecl, nothing popped. Written
/// instruction by instruction like `00e2bbd0`.
#[allow(
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_labels,
    clippy::all
)]
pub fn fn_00e2cc30(
    e: &mut Engine,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
) {
    e.with_stack(0x1400, |e, frame| {
        // The exe's registers; `esp` points at the return address slot, the arguments above it.
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut esp = frame.addr() + 0x1400 - 0x80;
        e.mem.set_u32(esp, 0);
        e.mem.set_u32(esp + 4, arg0);
        e.mem.set_u32(esp + 8, arg1);
        e.mem.set_u32(esp + 12, arg2);
        e.mem.set_u32(esp + 16, arg3);
        e.mem.set_u32(esp + 20, arg4);
        e.mem.set_u32(esp + 24, arg5);
        e.mem.set_u32(esp + 28, arg6);
        let mut xmm = [[0u32; 4]; 8];
        let mut fl = Flags::default();
        let mut st: Vec<f64> = Vec::new();
        let mut fsw: u16 = 0;
        let mut pc = 0x00e2_cc30_u32;
        loop {
            match pc {
                0x00e2_cc30 => {
                    // 00e2cc30: SUB ESP,0x868
                    esp = fl.sub(esp, 0x868_u32, 32);
                    // 00e2cc36: PUSH EBP
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp);
                    // 00e2cc37: XOR EBP,EBP
                    ebp = fl.xor(ebp, ebp, 32);
                    // 00e2cc39: CMP dword ptr [ESP + 0x870],EBP
                    fl.sub(e.mem.u32(esp.wrapping_add(0x870)), ebp, 32);
                    // 00e2cc40: JZ 0x00e2cfaa
                    if fl.zf {
                        pc = 0x00e2_cfaa;
                        continue;
                    }
                    pc = 0x00e2_cc46;
                }
                0x00e2_cc46 => {
                    // 00e2cc46: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2cc47: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2cc48: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cc4d: MOV ESI,dword ptr [ESP + 0x890]
                    esi = e.mem.u32(esp.wrapping_add(0x890));
                    // 00e2cc54: MOV EDI,dword ptr [EAX]
                    edi = e.mem.u32(eax);
                    // 00e2cc56: LEA ECX,[ESI*0x4 + 0x7f]
                    ecx = esi.wrapping_mul(4).wrapping_add(0x7f);
                    // 00e2cc5d: AND ECX,0xffffff80
                    ecx = fl.and(ecx, 0xffffff80_u32, 32);
                    // 00e2cc60: ADD ECX,EDI
                    ecx = fl.add(ecx, edi, 32);
                    // 00e2cc62: MOV dword ptr [EAX],ECX
                    e.mem.set_u32(eax, ecx);
                    // 00e2cc64: MOV EAX,ESI
                    eax = esi;
                    // 00e2cc66: OR EAX,0x80000000
                    eax = fl.or(eax, 0x80000000_u32, 32);
                    // 00e2cc6b: CMP ESI,EBP
                    fl.sub(esi, ebp, 32);
                    // 00e2cc6d: MOV dword ptr [ESP + 0x28],EDI
                    e.mem.set_u32(esp.wrapping_add(0x28), edi);
                    // 00e2cc71: MOV dword ptr [ESP + 0x9c],EAX
                    e.mem.set_u32(esp.wrapping_add(0x9c), eax);
                    // 00e2cc78: JLE 0x00e2cc83
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_cc83;
                        continue;
                    }
                    pc = 0x00e2_cc7a;
                }
                0x00e2_cc7a => {
                    // 00e2cc7a: MOV ECX,ESI
                    ecx = esi;
                    // 00e2cc7c: MOV EAX,0x3f800000
                    eax = 0x3f800000_u32;
                    // 00e2cc81: STOSD.REP ES:EDI
                    while ecx != 0 {
                        e.mem.set_u32(edi, eax);
                        edi = edi.wrapping_add(4);
                        ecx = ecx.wrapping_sub(1);
                    }
                    pc = 0x00e2_cc83;
                }
                0x00e2_cc83 => {
                    // 00e2cc83: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2cc84: CALL 0x00e4e3f0
                    let words = [];
                    let result = e.call(0x00e4e3f0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cc89: XOR EDI,EDI
                    edi = fl.xor(edi, edi, 32);
                    // 00e2cc8b: MOV dword ptr [ESP + 0x18],EDI
                    e.mem.set_u32(esp.wrapping_add(0x18), edi);
                    // 00e2cc8f: MOV dword ptr [ESP + 0x1c],EBP
                    e.mem.set_u32(esp.wrapping_add(0x1c), ebp);
                    // 00e2cc93: CALL 0x00e4e3f0
                    let words = [];
                    let result = e.call(0x00e4e3f0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cc98: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cc9d: MOV ECX,dword ptr [EAX]
                    ecx = e.mem.u32(eax);
                    // 00e2cc9f: LEA EDX,[ECX + 0x180]
                    edx = ecx.wrapping_add(0x180);
                    // 00e2cca5: MOV dword ptr [ESP + 0x28],ECX
                    e.mem.set_u32(esp.wrapping_add(0x28), ecx);
                    // 00e2cca9: MOV dword ptr [EAX],EDX
                    e.mem.set_u32(eax, edx);
                    // 00e2ccab: MOV dword ptr [ESP + 0x24],EBP
                    e.mem.set_u32(esp.wrapping_add(0x24), ebp);
                    // 00e2ccaf: LEA EBX,[EDI + 0x1]
                    ebx = edi.wrapping_add(0x1);
                    // 00e2ccb2: CMP ECX,EBP
                    fl.sub(ecx, ebp, 32);
                    // 00e2ccb4: JZ 0x00e2ccbc
                    if fl.zf {
                        pc = 0x00e2_ccbc;
                        continue;
                    }
                    pc = 0x00e2_ccb6;
                }
                0x00e2_ccb6 => {
                    // 00e2ccb6: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    // 00e2ccb8: MOV word ptr [ECX + 0x10],AX
                    e.mem.set_u16(ecx.wrapping_add(0x10), (eax & 0xffff) as u16);
                    pc = 0x00e2_ccbc;
                }
                0x00e2_ccbc => {
                    // 00e2ccbc: MOV EDX,dword ptr [ESP + 0x87c]
                    edx = e.mem.u32(esp.wrapping_add(0x87c));
                    // 00e2ccc3: MOV EAX,dword ptr [EDX + 0x10]
                    eax = e.mem.u32(edx.wrapping_add(0x10));
                    // 00e2ccc6: MOV EDX,dword ptr [EAX]
                    edx = e.mem.u32(eax);
                    // 00e2ccc8: MOV dword ptr [ECX],EDX
                    e.mem.set_u32(ecx, edx);
                    // 00e2ccca: MOV EDX,dword ptr [EAX + 0x4]
                    edx = e.mem.u32(eax.wrapping_add(0x4));
                    // 00e2cccd: MOV dword ptr [ECX + 0x4],EDX
                    e.mem.set_u32(ecx.wrapping_add(0x4), edx);
                    // 00e2ccd0: MOV EDX,dword ptr [EAX + 0x8]
                    edx = e.mem.u32(eax.wrapping_add(0x8));
                    // 00e2ccd3: MOV dword ptr [ECX + 0x8],EDX
                    e.mem.set_u32(ecx.wrapping_add(0x8), edx);
                    // 00e2ccd6: MOV EDX,dword ptr [EAX + 0xc]
                    edx = e.mem.u32(eax.wrapping_add(0xc));
                    // 00e2ccd9: MOV dword ptr [ECX + 0xc],EDX
                    e.mem.set_u32(ecx.wrapping_add(0xc), edx);
                    // 00e2ccdc: MOV EAX,dword ptr [EAX + 0x10]
                    eax = e.mem.u32(eax.wrapping_add(0x10));
                    // 00e2ccdf: MOV dword ptr [ECX + 0x10],EAX
                    e.mem.set_u32(ecx.wrapping_add(0x10), eax);
                    // 00e2cce2: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    // 00e2cce4: CMP ESI,EBP
                    fl.sub(esi, ebp, 32);
                    // 00e2cce6: MOV dword ptr [ECX + 0x14],EBP
                    e.mem.set_u32(ecx.wrapping_add(0x14), ebp);
                    // 00e2cce9: MOV dword ptr [ECX + 0x18],EBP
                    e.mem.set_u32(ecx.wrapping_add(0x18), ebp);
                    // 00e2ccec: MOV dword ptr [ECX + 0x1c],ESI
                    e.mem.set_u32(ecx.wrapping_add(0x1c), esi);
                    // 00e2ccef: JLE 0x00e2cd02
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_cd02;
                        continue;
                    }
                    pc = 0x00e2_ccf1;
                }
                0x00e2_ccf1 => {
                    // 00e2ccf1: MOV dword ptr [ESP + EDI*0x4 + 0xa8],EAX
                    e.mem.set_u32(
                        esp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0xa8),
                        eax,
                    );
                    // 00e2ccf8: INC EAX
                    eax = fl.inc(eax, 32);
                    // 00e2ccf9: INC EDI
                    edi = fl.inc(edi, 32);
                    // 00e2ccfa: CMP EAX,ESI
                    fl.sub(eax, esi, 32);
                    // 00e2ccfc: JL 0x00e2ccf1
                    if fl.sf != fl.of {
                        pc = 0x00e2_ccf1;
                        continue;
                    }
                    pc = 0x00e2_ccfe;
                }
                0x00e2_ccfe => {
                    // 00e2ccfe: MOV dword ptr [ESP + 0x18],EDI
                    e.mem.set_u32(esp.wrapping_add(0x18), edi);
                    pc = 0x00e2_cd02;
                }
                0x00e2_cd02 => {
                    // 00e2cd02: MOV EBP,0x20
                    ebp = 0x20_u32;
                    // 00e2cd07: MOV dword ptr [ESP + 0x10],0x9
                    e.mem.set_u32(esp.wrapping_add(0x10), 0x9_u32);
                    // 00e2cd0f: MOV EAX,ECX
                    eax = ecx;
                    // 00e2cd11: JMP 0x00e2cd17
                    pc = 0x00e2_cd17;
                    continue;
                }
                0x00e2_cd13 => {
                    // 00e2cd13: MOV EAX,dword ptr [ESP + 0x14]
                    eax = e.mem.u32(esp.wrapping_add(0x14));
                    pc = 0x00e2_cd17;
                }
                0x00e2_cd17 => {
                    // 00e2cd17: MOV ESI,EAX
                    esi = eax;
                    // 00e2cd19: SUB EAX,0x20
                    eax = fl.sub(eax, 0x20_u32, 32);
                    // 00e2cd1c: MOV dword ptr [ESP + 0x14],EAX
                    e.mem.set_u32(esp.wrapping_add(0x14), eax);
                    // 00e2cd20: MOV ECX,0x8
                    ecx = 0x8_u32;
                    // 00e2cd25: LEA EDI,[ESP + 0x78]
                    edi = esp.wrapping_add(0x78);
                    // 00e2cd29: MOVSD.REP ES:EDI,ESI
                    while ecx != 0 {
                        let word = e.mem.u32(esi);
                        e.mem.set_u32(edi, word);
                        esi = esi.wrapping_add(4);
                        edi = edi.wrapping_add(4);
                        ecx = ecx.wrapping_sub(1);
                    }
                    // 00e2cd2b: MOV ECX,dword ptr [ESP + 0x78]
                    ecx = e.mem.u32(esp.wrapping_add(0x78));
                    // 00e2cd2f: MOV EAX,dword ptr [ESP + 0x80]
                    eax = e.mem.u32(esp.wrapping_add(0x80));
                    // 00e2cd36: MOV EDX,dword ptr [ESP + 0x7c]
                    edx = e.mem.u32(esp.wrapping_add(0x7c));
                    // 00e2cd3a: MOV dword ptr [ESP + 0x38],ECX
                    e.mem.set_u32(esp.wrapping_add(0x38), ecx);
                    // 00e2cd3e: MOV ECX,dword ptr [ESP + 0x84]
                    ecx = e.mem.u32(esp.wrapping_add(0x84));
                    // 00e2cd45: MOV dword ptr [ESP + 0x40],EAX
                    e.mem.set_u32(esp.wrapping_add(0x40), eax);
                    // 00e2cd49: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    // 00e2cd4b: MOV dword ptr [ESP + 0x44],ECX
                    e.mem.set_u32(esp.wrapping_add(0x44), ecx);
                    // 00e2cd4f: INC dword ptr [ESP + 0x10]
                    e.mem.set_u32(
                        esp.wrapping_add(0x10),
                        fl.inc(e.mem.u32(esp.wrapping_add(0x10)), 32),
                    );
                    // 00e2cd53: XOR ECX,ECX
                    ecx = fl.xor(ecx, ecx, 32);
                    // 00e2cd55: MOV word ptr [ESP + 0x60],AX
                    e.mem.set_u16(esp.wrapping_add(0x60), (eax & 0xffff) as u16);
                    // 00e2cd5a: MOV EAX,dword ptr [ESP + 0x90]
                    eax = e.mem.u32(esp.wrapping_add(0x90));
                    // 00e2cd61: LEA ESI,[ESP + 0x64]
                    esi = esp.wrapping_add(0x64);
                    // 00e2cd65: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2cd66: MOV word ptr [ESP + 0x78],CX
                    e.mem.set_u16(esp.wrapping_add(0x78), (ecx & 0xffff) as u16);
                    // 00e2cd6b: MOV ECX,dword ptr [ESP + 0x98]
                    ecx = e.mem.u32(esp.wrapping_add(0x98));
                    // 00e2cd72: SUB dword ptr [ESP + EAX*0x4 + 0x1c],ECX
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x1c),
                        fl.sub(
                            e.mem
                                .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x1c)),
                            ecx,
                            32,
                        ),
                    );
                    // 00e2cd76: LEA ESI,[ESP + 0x54]
                    esi = esp.wrapping_add(0x54);
                    // 00e2cd7a: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2cd7b: LEA ESI,[ESP + 0x28]
                    esi = esp.wrapping_add(0x28);
                    // 00e2cd7f: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2cd80: MOV ESI,dword ptr [ESP + 0x28]
                    esi = e.mem.u32(esp.wrapping_add(0x28));
                    // 00e2cd84: SUB EBP,0x20
                    ebp = fl.sub(ebp, 0x20_u32, 32);
                    // 00e2cd87: LEA EDI,[ESP + ESI*0x4 + 0x49c]
                    edi = esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x49c);
                    // 00e2cd8e: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2cd8f: MOV dword ptr [ESP + 0x4c],EDX
                    e.mem.set_u32(esp.wrapping_add(0x4c), edx);
                    // 00e2cd93: MOV EDX,dword ptr [ESP + 0x98]
                    edx = e.mem.u32(esp.wrapping_add(0x98));
                    // 00e2cd9a: MOV dword ptr [ESP + 0x44],EBP
                    e.mem.set_u32(esp.wrapping_add(0x44), ebp);
                    // 00e2cd9e: LEA EDI,[ESP + 0x40]
                    edi = esp.wrapping_add(0x40);
                    // 00e2cda2: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2cda3: MOV EDI,dword ptr [ESP + 0x2c]
                    edi = e.mem.u32(esp.wrapping_add(0x2c));
                    // 00e2cda7: MOV dword ptr [ESP + 0x5c],EDX
                    e.mem.set_u32(esp.wrapping_add(0x5c), edx);
                    // 00e2cdab: MOV EDX,dword ptr [ESP + EAX*0x4 + 0x2c]
                    edx = e
                        .mem
                        .u32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x2c));
                    // 00e2cdaf: IMUL EAX,EAX,0xfa
                    eax = (eax as i32).wrapping_mul(250_i32) as u32;
                    // 00e2cdb5: LEA EBP,[ESP + EDI*0x4 + 0xbc]
                    ebp = esp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0xbc);
                    // 00e2cdbc: PUSH EBP
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp);
                    // 00e2cdbd: MOV EBP,0xfa
                    ebp = 0xfa_u32;
                    // 00e2cdc2: SUB EBP,ESI
                    ebp = fl.sub(ebp, esi, 32);
                    // 00e2cdc4: PUSH EBP
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp);
                    // 00e2cdc5: MOV EBP,0xfa
                    ebp = 0xfa_u32;
                    // 00e2cdca: ADD EAX,EDX
                    eax = fl.add(eax, edx, 32);
                    // 00e2cdcc: SUB EBP,EDI
                    ebp = fl.sub(ebp, edi, 32);
                    // 00e2cdce: PUSH EBP
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp);
                    // 00e2cdcf: MOV EBP,dword ptr [ESP + 0x30]
                    ebp = e.mem.u32(esp.wrapping_add(0x30));
                    // 00e2cdd3: PUSH EBP
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp);
                    // 00e2cdd4: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2cdd5: MOV ECX,dword ptr [ESP + 0x8b0]
                    ecx = e.mem.u32(esp.wrapping_add(0x8b0));
                    // 00e2cddc: LEA EDX,[ESP + EAX*0x4 + 0xd0]
                    edx = esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0xd0);
                    // 00e2cde3: MOV EAX,dword ptr [ESP + 0x54]
                    eax = e.mem.u32(esp.wrapping_add(0x54));
                    // 00e2cde7: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2cde8: MOV EDX,dword ptr [ESP + 0x8b0]
                    edx = e.mem.u32(esp.wrapping_add(0x8b0));
                    // 00e2cdef: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2cdf0: MOV EAX,dword ptr [ESP + 0x8c0]
                    eax = e.mem.u32(esp.wrapping_add(0x8c0));
                    // 00e2cdf7: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2cdf8: MOV ECX,dword ptr [ESP + 0x8c0]
                    ecx = e.mem.u32(esp.wrapping_add(0x8c0));
                    // 00e2cdff: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ce00: MOV EDX,dword ptr [ESP + 0x8b8]
                    edx = e.mem.u32(esp.wrapping_add(0x8b8));
                    // 00e2ce07: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2ce08: MOV EAX,dword ptr [ESP + 0xc8]
                    eax = e.mem.u32(esp.wrapping_add(0xc8));
                    // 00e2ce0f: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2ce10: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ce11: MOV EDX,dword ptr [ESP + 0x8c0]
                    edx = e.mem.u32(esp.wrapping_add(0x8c0));
                    // 00e2ce18: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2ce19: LEA ECX,[ESP + 0x80]
                    ecx = esp.wrapping_add(0x80);
                    // 00e2ce20: PUSH ECX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx);
                    // 00e2ce21: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ce22: DEC EBX
                    ebx = fl.dec(ebx, 32);
                    // 00e2ce23: CALL 0x00e2c100
                    let words = [
                        e.mem.u32(esp),
                        e.mem.u32(esp.wrapping_add(4)),
                        e.mem.u32(esp.wrapping_add(8)),
                        e.mem.u32(esp.wrapping_add(12)),
                        e.mem.u32(esp.wrapping_add(16)),
                        e.mem.u32(esp.wrapping_add(20)),
                        e.mem.u32(esp.wrapping_add(24)),
                        e.mem.u32(esp.wrapping_add(28)),
                        e.mem.u32(esp.wrapping_add(32)),
                        e.mem.u32(esp.wrapping_add(36)),
                        e.mem.u32(esp.wrapping_add(40)),
                        e.mem.u32(esp.wrapping_add(44)),
                        e.mem.u32(esp.wrapping_add(48)),
                        e.mem.u32(esp.wrapping_add(52)),
                        e.mem.u32(esp.wrapping_add(56)),
                        e.mem.u32(esp.wrapping_add(60)),
                        e.mem.u32(esp.wrapping_add(64)),
                        e.mem.u32(esp.wrapping_add(68)),
                        e.mem.u32(esp.wrapping_add(72)),
                        e.mem.u32(esp.wrapping_add(76)),
                    ];
                    let result = e.call(0x00e2c100, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2ce28: MOV EAX,dword ptr [ESP + 0x80]
                    eax = e.mem.u32(esp.wrapping_add(0x80));
                    // 00e2ce2f: MOV EBP,dword ptr [ESP + 0x84]
                    ebp = e.mem.u32(esp.wrapping_add(0x84));
                    // 00e2ce36: ADD EDI,EAX
                    edi = fl.add(edi, eax, 32);
                    // 00e2ce38: ADD ESP,0x50
                    esp = fl.add(esp, 0x50_u32, 32);
                    // 00e2ce3b: ADD ESI,dword ptr [ESP + 0x20]
                    esi = fl.add(esi, e.mem.u32(esp.wrapping_add(0x20)), 32);
                    // 00e2ce3f: MOV dword ptr [ESP + 0x18],EDI
                    e.mem.set_u32(esp.wrapping_add(0x18), edi);
                    // 00e2ce43: MOV dword ptr [ESP + 0x1c],ESI
                    e.mem.set_u32(esp.wrapping_add(0x1c), esi);
                    // 00e2ce47: MOV EDI,dword ptr [ESP + 0x28]
                    edi = e.mem.u32(esp.wrapping_add(0x28));
                    // 00e2ce4b: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2ce4d: JZ 0x00e2cec9
                    if fl.zf {
                        pc = 0x00e2_cec9;
                        continue;
                    }
                    pc = 0x00e2_ce4f;
                }
                0x00e2_ce4f => {
                    // 00e2ce4f: ADD dword ptr [ESP + 0x14],0x20
                    e.mem.set_u32(
                        esp.wrapping_add(0x14),
                        fl.add(e.mem.u32(esp.wrapping_add(0x14)), 0x20_u32, 32),
                    );
                    // 00e2ce54: DEC dword ptr [ESP + 0x10]
                    e.mem.set_u32(
                        esp.wrapping_add(0x10),
                        fl.dec(e.mem.u32(esp.wrapping_add(0x10)), 32),
                    );
                    // 00e2ce58: MOV ECX,EBX
                    ecx = ebx;
                    // 00e2ce5a: INC EBX
                    ebx = fl.inc(ebx, 32);
                    // 00e2ce5b: MOV EAX,EBX
                    eax = ebx;
                    // 00e2ce5d: MOV EDX,EBP
                    edx = ebp;
                    // 00e2ce5f: SUB EAX,ECX
                    eax = fl.sub(eax, ecx, 32);
                    // 00e2ce61: ADD EBP,0x20
                    ebp = fl.add(ebp, 0x20_u32, 32);
                    // 00e2ce64: LEA ESI,[EDI + EDX*0x1]
                    esi = edi.wrapping_add(edx);
                    // 00e2ce67: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2ce69: JLE 0x00e2ce82
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_ce82;
                        continue;
                    }
                    pc = 0x00e2_ce6b;
                }
                0x00e2_ce6b => {
                    // 00e2ce6b: MOV ECX,ESI
                    ecx = esi;
                    // 00e2ce6d: LEA ECX,[ECX]
                    // (no operation)
                    pc = 0x00e2_ce70;
                }
                0x00e2_ce70 => {
                    // 00e2ce70: TEST ECX,ECX
                    fl.and(ecx, ecx, 32);
                    // 00e2ce72: JZ 0x00e2ce7a
                    if fl.zf {
                        pc = 0x00e2_ce7a;
                        continue;
                    }
                    pc = 0x00e2_ce74;
                }
                0x00e2_ce74 => {
                    // 00e2ce74: XOR EDX,EDX
                    edx = fl.xor(edx, edx, 32);
                    // 00e2ce76: MOV word ptr [ECX + 0x10],DX
                    e.mem.set_u16(ecx.wrapping_add(0x10), (edx & 0xffff) as u16);
                    pc = 0x00e2_ce7a;
                }
                0x00e2_ce7a => {
                    // 00e2ce7a: ADD ECX,0x20
                    ecx = fl.add(ecx, 0x20_u32, 32);
                    // 00e2ce7d: SUB EAX,0x1
                    eax = fl.sub(eax, 0x1_u32, 32);
                    // 00e2ce80: JNZ 0x00e2ce70
                    if !fl.zf {
                        pc = 0x00e2_ce70;
                        continue;
                    }
                    pc = 0x00e2_ce82;
                }
                0x00e2_ce82 => {
                    // 00e2ce82: MOV EAX,dword ptr [ESP + 0x50]
                    eax = e.mem.u32(esp.wrapping_add(0x50));
                    // 00e2ce86: MOV ECX,dword ptr [ESP + 0x54]
                    ecx = e.mem.u32(esp.wrapping_add(0x54));
                    // 00e2ce8a: MOV EDX,dword ptr [ESP + 0x58]
                    edx = e.mem.u32(esp.wrapping_add(0x58));
                    // 00e2ce8e: MOV dword ptr [ESI],EAX
                    e.mem.set_u32(esi, eax);
                    // 00e2ce90: MOV EAX,dword ptr [ESP + 0x5c]
                    eax = e.mem.u32(esp.wrapping_add(0x5c));
                    // 00e2ce94: MOV dword ptr [ESI + 0x4],ECX
                    e.mem.set_u32(esi.wrapping_add(0x4), ecx);
                    // 00e2ce97: MOV ECX,dword ptr [ESP + 0x60]
                    ecx = e.mem.u32(esp.wrapping_add(0x60));
                    // 00e2ce9b: MOV dword ptr [ESI + 0x8],EDX
                    e.mem.set_u32(esi.wrapping_add(0x8), edx);
                    // 00e2ce9e: MOV EDX,dword ptr [ESP + 0x46]
                    edx = e.mem.u32(esp.wrapping_add(0x46));
                    // 00e2cea2: MOV dword ptr [ESI + 0xc],EAX
                    e.mem.set_u32(esi.wrapping_add(0xc), eax);
                    // 00e2cea5: MOV dword ptr [ESI + 0x10],ECX
                    e.mem.set_u32(esi.wrapping_add(0x10), ecx);
                    // 00e2cea8: MOV ECX,dword ptr [ESP + 0x87c]
                    ecx = e.mem.u32(esp.wrapping_add(0x87c));
                    // 00e2ceaf: PUSH EDX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx);
                    // 00e2ceb0: ADD ECX,0x1c
                    ecx = fl.add(ecx, 0x1c_u32, 32);
                    // 00e2ceb3: CALL 0x00d9a4d0
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d9a4d0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2ceb8: MOV dword ptr [ESI + 0x14],EAX
                    e.mem.set_u32(esi.wrapping_add(0x14), eax);
                    // 00e2cebb: MOV EAX,dword ptr [ESP + 0x30]
                    eax = e.mem.u32(esp.wrapping_add(0x30));
                    // 00e2cebf: MOV dword ptr [ESI + 0x18],0x0
                    e.mem.set_u32(esi.wrapping_add(0x18), 0x0_u32);
                    // 00e2cec6: MOV dword ptr [ESI + 0x1c],EAX
                    e.mem.set_u32(esi.wrapping_add(0x1c), eax);
                    pc = 0x00e2_cec9;
                }
                0x00e2_cec9 => {
                    // 00e2cec9: CMP dword ptr [ESP + 0x20],0x0
                    fl.sub(e.mem.u32(esp.wrapping_add(0x20)), 0x0_u32, 32);
                    // 00e2cece: JZ 0x00e2cf59
                    if fl.zf {
                        pc = 0x00e2_cf59;
                        continue;
                    }
                    pc = 0x00e2_ced4;
                }
                0x00e2_ced4 => {
                    // 00e2ced4: ADD dword ptr [ESP + 0x14],0x20
                    e.mem.set_u32(
                        esp.wrapping_add(0x14),
                        fl.add(e.mem.u32(esp.wrapping_add(0x14)), 0x20_u32, 32),
                    );
                    // 00e2ced9: DEC dword ptr [ESP + 0x10]
                    e.mem.set_u32(
                        esp.wrapping_add(0x10),
                        fl.dec(e.mem.u32(esp.wrapping_add(0x10)), 32),
                    );
                    // 00e2cedd: MOV ECX,EBX
                    ecx = ebx;
                    // 00e2cedf: INC EBX
                    ebx = fl.inc(ebx, 32);
                    // 00e2cee0: MOV EAX,EBX
                    eax = ebx;
                    // 00e2cee2: MOV EDX,EBP
                    edx = ebp;
                    // 00e2cee4: SUB EAX,ECX
                    eax = fl.sub(eax, ecx, 32);
                    // 00e2cee6: ADD EBP,0x20
                    ebp = fl.add(ebp, 0x20_u32, 32);
                    // 00e2cee9: LEA ESI,[EDI + EDX*0x1]
                    esi = edi.wrapping_add(edx);
                    // 00e2ceec: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2ceee: JLE 0x00e2cf04
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e2_cf04;
                        continue;
                    }
                    pc = 0x00e2_cef0;
                }
                0x00e2_cef0 => {
                    // 00e2cef0: MOV ECX,ESI
                    ecx = esi;
                    pc = 0x00e2_cef2;
                }
                0x00e2_cef2 => {
                    // 00e2cef2: TEST ECX,ECX
                    fl.and(ecx, ecx, 32);
                    // 00e2cef4: JZ 0x00e2cefc
                    if fl.zf {
                        pc = 0x00e2_cefc;
                        continue;
                    }
                    pc = 0x00e2_cef6;
                }
                0x00e2_cef6 => {
                    // 00e2cef6: XOR EDX,EDX
                    edx = fl.xor(edx, edx, 32);
                    // 00e2cef8: MOV word ptr [ECX + 0x10],DX
                    e.mem.set_u16(ecx.wrapping_add(0x10), (edx & 0xffff) as u16);
                    pc = 0x00e2_cefc;
                }
                0x00e2_cefc => {
                    // 00e2cefc: ADD ECX,0x20
                    ecx = fl.add(ecx, 0x20_u32, 32);
                    // 00e2ceff: SUB EAX,0x1
                    eax = fl.sub(eax, 0x1_u32, 32);
                    // 00e2cf02: JNZ 0x00e2cef2
                    if !fl.zf {
                        pc = 0x00e2_cef2;
                        continue;
                    }
                    pc = 0x00e2_cf04;
                }
                0x00e2_cf04 => {
                    // 00e2cf04: MOV EAX,dword ptr [ESP + 0x64]
                    eax = e.mem.u32(esp.wrapping_add(0x64));
                    // 00e2cf08: MOV ECX,dword ptr [ESP + 0x68]
                    ecx = e.mem.u32(esp.wrapping_add(0x68));
                    // 00e2cf0c: MOV EDX,dword ptr [ESP + 0x6c]
                    edx = e.mem.u32(esp.wrapping_add(0x6c));
                    // 00e2cf10: MOV dword ptr [ESI],EAX
                    e.mem.set_u32(esi, eax);
                    // 00e2cf12: MOV EAX,dword ptr [ESP + 0x70]
                    eax = e.mem.u32(esp.wrapping_add(0x70));
                    // 00e2cf16: MOV dword ptr [ESI + 0x4],ECX
                    e.mem.set_u32(esi.wrapping_add(0x4), ecx);
                    // 00e2cf19: MOV ECX,dword ptr [ESP + 0x74]
                    ecx = e.mem.u32(esp.wrapping_add(0x74));
                    // 00e2cf1d: MOV dword ptr [ESI + 0x8],EDX
                    e.mem.set_u32(esi.wrapping_add(0x8), edx);
                    // 00e2cf20: MOV dword ptr [ESI + 0xc],EAX
                    e.mem.set_u32(esi.wrapping_add(0xc), eax);
                    // 00e2cf23: MOV EAX,dword ptr [ESP + 0x4a]
                    eax = e.mem.u32(esp.wrapping_add(0x4a));
                    // 00e2cf27: MOV EDX,0x7fff
                    edx = 0x7fff_u32;
                    // 00e2cf2c: MOV dword ptr [ESI + 0x10],ECX
                    e.mem.set_u32(esi.wrapping_add(0x10), ecx);
                    // 00e2cf2f: CMP AX,DX
                    fl.sub(eax & 0xffff, edx & 0xffff, 16);
                    // 00e2cf32: JZ 0x00e2cf46
                    if fl.zf {
                        pc = 0x00e2_cf46;
                        continue;
                    }
                    pc = 0x00e2_cf34;
                }
                0x00e2_cf34 => {
                    // 00e2cf34: MOV ECX,dword ptr [ESP + 0x87c]
                    ecx = e.mem.u32(esp.wrapping_add(0x87c));
                    // 00e2cf3b: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2cf3c: ADD ECX,0x1c
                    ecx = fl.add(ecx, 0x1c_u32, 32);
                    // 00e2cf3f: CALL 0x00d9a4d0
                    let words = [ecx, e.mem.u32(esp)];
                    let result = e.call(0x00d9a4d0, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(4);
                    // 00e2cf44: JMP 0x00e2cf48
                    pc = 0x00e2_cf48;
                    continue;
                }
                0x00e2_cf46 => {
                    // 00e2cf46: XOR EAX,EAX
                    eax = fl.xor(eax, eax, 32);
                    pc = 0x00e2_cf48;
                }
                0x00e2_cf48 => {
                    // 00e2cf48: MOV dword ptr [ESI + 0x14],EAX
                    e.mem.set_u32(esi.wrapping_add(0x14), eax);
                    // 00e2cf4b: MOV EAX,dword ptr [ESP + 0x20]
                    eax = e.mem.u32(esp.wrapping_add(0x20));
                    // 00e2cf4f: MOV dword ptr [ESI + 0x18],0x1
                    e.mem.set_u32(esi.wrapping_add(0x18), 0x1_u32);
                    // 00e2cf56: MOV dword ptr [ESI + 0x1c],EAX
                    e.mem.set_u32(esi.wrapping_add(0x1c), eax);
                    pc = 0x00e2_cf59;
                }
                0x00e2_cf59 => {
                    // 00e2cf59: CMP dword ptr [ESP + 0x24],EBX
                    fl.sub(e.mem.u32(esp.wrapping_add(0x24)), ebx, 32);
                    // 00e2cf5d: JG 0x00e2cf63
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e2_cf63;
                        continue;
                    }
                    pc = 0x00e2_cf5f;
                }
                0x00e2_cf5f => {
                    // 00e2cf5f: MOV dword ptr [ESP + 0x24],EBX
                    e.mem.set_u32(esp.wrapping_add(0x24), ebx);
                    pc = 0x00e2_cf63;
                }
                0x00e2_cf63 => {
                    // 00e2cf63: TEST EBX,EBX
                    fl.and(ebx, ebx, 32);
                    // 00e2cf65: JNZ 0x00e2cd13
                    if !fl.zf {
                        pc = 0x00e2_cd13;
                        continue;
                    }
                    pc = 0x00e2_cf6b;
                }
                0x00e2_cf6b => {
                    // 00e2cf6b: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cf70: MOV dword ptr [EAX],EDI
                    e.mem.set_u32(eax, edi);
                    // 00e2cf72: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cf77: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cf7c: MOV ESI,dword ptr [ESP + 0x2c]
                    esi = e.mem.u32(esp.wrapping_add(0x2c));
                    // 00e2cf80: MOV dword ptr [EAX],ESI
                    e.mem.set_u32(eax, esi);
                    // 00e2cf82: CALL 0x00c85750
                    let words = [];
                    let result = e.call(0x00c85750, &words);
                    eax = result.eax;
                    edx = result.edx;
                    // 00e2cf87: MOV ECX,dword ptr [EAX + 0x10]
                    ecx = e.mem.u32(eax.wrapping_add(0x10));
                    // 00e2cf8a: MOV EAX,dword ptr [ESP + 0xa0]
                    eax = e.mem.u32(esp.wrapping_add(0xa0));
                    // 00e2cf91: POP EBX
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2cf92: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e2cf94: JS 0x00e2cfa8
                    if fl.sf {
                        pc = 0x00e2_cfa8;
                        continue;
                    }
                    pc = 0x00e2_cf96;
                }
                0x00e2_cf96 => {
                    // 00e2cf96: MOV EDX,dword ptr [ECX]
                    edx = e.mem.u32(ecx);
                    // 00e2cf98: AND EAX,0x3fffffff
                    eax = fl.and(eax, 0x3fffffff_u32, 32);
                    // 00e2cf9d: ADD EAX,EAX
                    eax = fl.add(eax, eax, 32);
                    // 00e2cf9f: ADD EAX,EAX
                    eax = fl.add(eax, eax, 32);
                    // 00e2cfa1: PUSH EAX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax);
                    // 00e2cfa2: MOV EAX,dword ptr [EDX + 0x10]
                    eax = e.mem.u32(edx.wrapping_add(0x10));
                    // 00e2cfa5: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2cfa6: CALL EAX
                    let words = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                    let result = e.call(eax, &words);
                    eax = result.eax;
                    edx = result.edx;
                    esp = esp.wrapping_add(8);
                    st.push(result.st0);
                    pc = 0x00e2_cfa8;
                }
                0x00e2_cfa8 => {
                    // 00e2cfa8: POP EDI
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2cfa9: POP ESI
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    pc = 0x00e2_cfaa;
                }
                0x00e2_cfaa => {
                    // 00e2cfaa: POP EBP
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2cfab: ADD ESP,0x868
                    esp = fl.add(esp, 0x868_u32, 32);
                    // 00e2cfb1: RET
                    return;
                }
                _ => unreachable!("pc {pc:#x}"),
            }
        }
    });
}

// Translated from 00e2d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: finds the closest pair of faces of two convex hulls (the classes
/// are unconfirmed). Five stack words, cdecl, nothing popped: `arg0` and `arg1` the two hulls (a
/// face count at `+0x10`, faces of 0x50 bytes at `+0xf70`: a normal vector, a w word and, at
/// `+0x40`, a distance), `arg2` the output vector, `arg3` and `arg4` where the chosen faces are
/// stored. Every face of `arg0` (from the last to the first) is paired with every face of `arg1`
/// from the last to the second (the loop stops before index 0): pairs whose squared distance is
/// above the best so far, or whose normals' dot product is below -0.5, are skipped; the nearest
/// point between the two planes is found with the exe's branches (weights `0`/`1`, `0.5`/`0.5`
/// or the general case, with a threshold of `1.1920929e-07`), and the pair with the smallest
/// squared distance is kept. The result is the weighted sum of the two normals, normalised
/// with the `RSQRTSS` estimate (the host's, within an ULP; zero stays zero), its w the weighted
/// sum of the w words, replaced by the square root of the best squared distance (`00ec6040`,
/// through `ST0`). The x87 stack is a vector of `f64` (the prelude's `x87_*` helpers choose the
/// NaN; every float load and store goes through `x87_load32`/`x87_store32`). Written
/// instruction by instruction like `00e2bbd0`.
#[allow(
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_labels,
    clippy::all
)]
pub fn fn_00e2d6c0(e: &mut Engine, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) {
    e.with_stack(0x800, |e, frame| {
        // The exe's registers; `esp` points at the return address slot, the arguments above it.
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut esp = frame.addr() + 0x800 - 0x80;
        e.mem.set_u32(esp, 0);
        e.mem.set_u32(esp + 4, arg0);
        e.mem.set_u32(esp + 8, arg1);
        e.mem.set_u32(esp + 12, arg2);
        e.mem.set_u32(esp + 16, arg3);
        e.mem.set_u32(esp + 20, arg4);
        let mut xmm = [[0u32; 4]; 8];
        let mut fl = Flags::default();
        let mut st: Vec<f64> = Vec::new();
        let mut fsw: u16 = 0;
        let mut pc = 0x00e2_d6c0_u32;
        loop {
            match pc {
                0x00e2_d6c0 => {
                    // 00e2d6c0: PUSH EBP
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp);
                    // 00e2d6c1: MOV EBP,ESP
                    ebp = esp;
                    // 00e2d6c3: AND ESP,0xfffffff0
                    esp = fl.and(esp, 0xfffffff0_u32, 32);
                    // 00e2d6c6: SUB ESP,0x24
                    esp = fl.sub(esp, 0x24_u32, 32);
                    // 00e2d6c9: FLD float ptr [0x0104da48]
                    st.push(x87_load32(e.mem.u32(0x104da48_u32)));
                    // 00e2d6cf: MOV EAX,dword ptr [EBP + 0x8]
                    eax = e.mem.u32(ebp.wrapping_add(0x8));
                    // 00e2d6d2: PUSH EBX
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx);
                    // 00e2d6d3: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(esp.wrapping_add(0x14), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d6d7: FLDZ
                    st.push(0.0);
                    // 00e2d6d9: PUSH ESI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi);
                    // 00e2d6da: PUSH EDI
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi);
                    // 00e2d6db: FST float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(esp.wrapping_add(0x24), x87_store32(st_get(&st, 0)));
                    // 00e2d6df: MOV EDI,dword ptr [EAX + 0x10]
                    edi = e.mem.u32(eax.wrapping_add(0x10));
                    // 00e2d6e2: FST float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(esp.wrapping_add(0x28), x87_store32(st_get(&st, 0)));
                    // 00e2d6e6: SUB EDI,0x1
                    edi = fl.sub(edi, 0x1_u32, 32);
                    // 00e2d6e9: JS 0x00e2d958
                    if fl.sf {
                        pc = 0x00e2_d958;
                        continue;
                    }
                    pc = 0x00e2_d6ef;
                }
                0x00e2_d6ef => {
                    // 00e2d6ef: FLD1
                    st.push(1.0);
                    // 00e2d6f1: LEA ECX,[EDI + EDI*0x4]
                    ecx = edi.wrapping_add(edi.wrapping_mul(4));
                    // 00e2d6f4: SHL ECX,0x4
                    ecx = fl.shift_shl(ecx, 0x4_u32, 32);
                    // 00e2d6f7: LEA EBX,[ECX + EAX*0x1 + 0xfb0]
                    ebx = ecx.wrapping_add(eax).wrapping_add(0xfb0);
                    pc = 0x00e2_d6fe;
                }
                0x00e2_d6fe => {
                    // 00e2d6fe: FLD float ptr [EBX]
                    st.push(x87_load32(e.mem.u32(ebx)));
                    // 00e2d700: LEA EDX,[EBX + -0x40]
                    edx = ebx.wrapping_sub(0x40);
                    // 00e2d703: FMUL ST0
                    fpu_arith(&mut st, Fop::Mul, 0, 0, false);
                    // 00e2d705: FLD float ptr [ESP + 0x1c]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x1c))));
                    // 00e2d709: FCOMPP
                    fsw = fpu_compare(st_get(&st, 0), st_get(&st, 1));
                    st.pop();
                    st.pop();
                    // 00e2d70b: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d70d: TEST AH,0x5
                    fl.and((eax >> 8) & 0xff, 0x5_u32, 8);
                    // 00e2d710: JNP 0x00e2d94a
                    if !fl.pf {
                        pc = 0x00e2_d94a;
                        continue;
                    }
                    pc = 0x00e2_d716;
                }
                0x00e2_d716 => {
                    // 00e2d716: MOV ECX,dword ptr [EBP + 0xc]
                    ecx = e.mem.u32(ebp.wrapping_add(0xc));
                    // 00e2d719: MOV EAX,dword ptr [ECX + 0x10]
                    eax = e.mem.u32(ecx.wrapping_add(0x10));
                    // 00e2d71c: DEC EAX
                    eax = fl.dec(eax, 32);
                    // 00e2d71d: CMP EAX,0x1
                    fl.sub(eax, 0x1_u32, 32);
                    // 00e2d720: JL 0x00e2d94a
                    if fl.sf != fl.of {
                        pc = 0x00e2_d94a;
                        continue;
                    }
                    pc = 0x00e2_d726;
                }
                0x00e2_d726 => {
                    // 00e2d726: LEA ESI,[EAX + EAX*0x4]
                    esi = eax.wrapping_add(eax.wrapping_mul(4));
                    // 00e2d729: SHL ESI,0x4
                    esi = fl.shift_shl(esi, 0x4_u32, 32);
                    // 00e2d72c: LEA ECX,[ESI + ECX*0x1 + 0xf70]
                    ecx = esi.wrapping_add(ecx).wrapping_add(0xf70);
                    // 00e2d733: MOV ESI,EAX
                    esi = eax;
                    pc = 0x00e2_d735;
                }
                0x00e2_d735 => {
                    // 00e2d735: FLD float ptr [ECX + 0x40]
                    st.push(x87_load32(e.mem.u32(ecx.wrapping_add(0x40))));
                    // 00e2d738: FMUL ST0
                    fpu_arith(&mut st, Fop::Mul, 0, 0, false);
                    // 00e2d73a: FLD float ptr [ESP + 0x1c]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x1c))));
                    // 00e2d73e: FCOMPP
                    fsw = fpu_compare(st_get(&st, 0), st_get(&st, 1));
                    st.pop();
                    st.pop();
                    // 00e2d740: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d742: TEST AH,0x5
                    fl.and((eax >> 8) & 0xff, 0x5_u32, 8);
                    // 00e2d745: JNP 0x00e2d93e
                    if !fl.pf {
                        pc = 0x00e2_d93e;
                        continue;
                    }
                    pc = 0x00e2_d74b;
                }
                0x00e2_d74b => {
                    // 00e2d74b: MOVAPS XMM1,xmmword ptr [EDX]
                    xmm[1] = load_lanes(e, edx);
                    // 00e2d74e: FLD float ptr [0x0102295c]
                    st.push(x87_load32(e.mem.u32(0x102295c_u32)));
                    // 00e2d754: MOVAPS XMM0,xmmword ptr [ECX]
                    xmm[0] = load_lanes(e, ecx);
                    // 00e2d757: MULPS XMM0,XMM1
                    xmm[0] = mulps(xmm[0], xmm[1]);
                    // 00e2d75a: MOVAPS XMM1,XMM0
                    xmm[1] = xmm[0];
                    // 00e2d75d: SHUFPS XMM1,XMM0,0x55
                    xmm[1] = shufps(xmm[1], xmm[0], 85);
                    // 00e2d761: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2d764: ADDSS XMM1,XMM0
                    xmm[1][0] = lane_add(xmm[1][0], xmm[0][0]);
                    // 00e2d768: SHUFPS XMM2,XMM0,0xaa
                    xmm[2] = shufps(xmm[2], xmm[0], 170);
                    // 00e2d76c: ADDSS XMM2,XMM1
                    xmm[2][0] = lane_add(xmm[2][0], xmm[1][0]);
                    // 00e2d770: MOVSS dword ptr [ESP + 0x2c],XMM2
                    e.mem.set_u32(esp.wrapping_add(0x2c), xmm[2][0]);
                    // 00e2d776: FLD float ptr [ESP + 0x2c]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x2c))));
                    // 00e2d77a: FCOM
                    fsw = fpu_compare(st_get(&st, 0), st_get(&st, 1));
                    // 00e2d77c: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d77e: FSTP ST1
                    {
                        let value = st_get(&st, 0);
                        st_set(&mut st, 1, value);
                    }
                    st.pop();
                    // 00e2d780: TEST AH,0x5
                    fl.and((eax >> 8) & 0xff, 0x5_u32, 8);
                    // 00e2d783: JNP 0x00e2d936
                    if !fl.pf {
                        pc = 0x00e2_d936;
                        continue;
                    }
                    pc = 0x00e2_d789;
                }
                0x00e2_d789 => {
                    // 00e2d789: FLD float ptr [EBX]
                    st.push(x87_load32(e.mem.u32(ebx)));
                    // 00e2d78b: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d78f: FLD float ptr [ECX + 0x40]
                    st.push(x87_load32(e.mem.u32(ecx.wrapping_add(0x40))));
                    // 00e2d792: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d796: FLD float ptr [ESP + 0x20]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x20))));
                    // 00e2d79a: FLD ST1
                    {
                        let value = st_get(&st, 1);
                        st.push(value);
                    }
                    // 00e2d79c: FLD float ptr [ESP + 0x18]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x18))));
                    // 00e2d7a0: FLD ST0
                    {
                        let value = st_get(&st, 0);
                        st.push(value);
                    }
                    // 00e2d7a2: FMULP ST2
                    fpu_arith(&mut st, Fop::Mul, 2, 0, true);
                    // 00e2d7a4: FXCH
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 1);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 1, top);
                    }
                    // 00e2d7a6: FSUBR ST0,ST2
                    fpu_arith(&mut st, Fop::SubR, 0, 2, false);
                    // 00e2d7a8: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d7ac: FLD ST0
                    {
                        let value = st_get(&st, 0);
                        st.push(value);
                    }
                    // 00e2d7ae: FLD ST3
                    {
                        let value = st_get(&st, 3);
                        st.push(value);
                    }
                    // 00e2d7b0: FMUL ST3
                    fpu_arith(&mut st, Fop::Mul, 0, 3, false);
                    // 00e2d7b2: FSUBP
                    fpu_arith(&mut st, Fop::Sub, 1, 0, true);
                    // 00e2d7b4: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d7b8: FLD float ptr [0x0104da4c]
                    st.push(x87_load32(e.mem.u32(0x104da4c_u32)));
                    // 00e2d7be: FLD float ptr [ESP + 0x20]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x20))));
                    // 00e2d7c2: FCOM
                    fsw = fpu_compare(st_get(&st, 0), st_get(&st, 1));
                    // 00e2d7c4: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d7c6: TEST AH,0x41
                    fl.and((eax >> 8) & 0xff, 0x41_u32, 8);
                    // 00e2d7c9: JNZ 0x00e2d8c6
                    if !fl.zf {
                        pc = 0x00e2_d8c6;
                        continue;
                    }
                    pc = 0x00e2_d7cf;
                }
                0x00e2_d7cf => {
                    // 00e2d7cf: FLD float ptr [ESP + 0x18]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x18))));
                    // 00e2d7d3: FCOM ST2
                    fsw = fpu_compare(st_get(&st, 0), st_get(&st, 2));
                    // 00e2d7d5: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d7d7: FSTP ST2
                    {
                        let value = st_get(&st, 0);
                        st_set(&mut st, 2, value);
                    }
                    st.pop();
                    // 00e2d7d9: TEST AH,0x41
                    fl.and((eax >> 8) & 0xff, 0x41_u32, 8);
                    // 00e2d7dc: JNZ 0x00e2d8a3
                    if !fl.zf {
                        pc = 0x00e2_d8a3;
                        continue;
                    }
                    pc = 0x00e2_d7e2;
                }
                0x00e2_d7e2 => {
                    // 00e2d7e2: FSTP ST6
                    {
                        let value = st_get(&st, 0);
                        st_set(&mut st, 6, value);
                    }
                    st.pop();
                    // 00e2d7e4: FCOM ST5
                    fsw = fpu_compare(st_get(&st, 0), st_get(&st, 5));
                    // 00e2d7e6: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d7e8: TEST AH,0x5
                    fl.and((eax >> 8) & 0xff, 0x5_u32, 8);
                    // 00e2d7eb: JP 0x00e2d831
                    if fl.pf {
                        pc = 0x00e2_d831;
                        continue;
                    }
                    pc = 0x00e2_d7ed;
                }
                0x00e2_d7ed => {
                    // 00e2d7ed: FSTP ST1
                    {
                        let value = st_get(&st, 0);
                        st_set(&mut st, 1, value);
                    }
                    st.pop();
                    // 00e2d7ef: FDIVRP ST4,ST0
                    fpu_arith(&mut st, Fop::DivR, 4, 0, true);
                    // 00e2d7f1: FXCH ST3
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 3);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 3, top);
                    }
                    // 00e2d7f3: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d7f7: FLD ST0
                    {
                        let value = st_get(&st, 0);
                        st.push(value);
                    }
                    // 00e2d7f9: FLD float ptr [ESP + 0x20]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x20))));
                    // 00e2d7fd: FLD ST0
                    {
                        let value = st_get(&st, 0);
                        st.push(value);
                    }
                    // 00e2d7ff: FMULP ST2
                    fpu_arith(&mut st, Fop::Mul, 2, 0, true);
                    // 00e2d801: FXCH
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 1);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 1, top);
                    }
                    // 00e2d803: FADD ST0,ST3
                    fpu_arith(&mut st, Fop::Add, 0, 3, false);
                    // 00e2d805: FLD ST3
                    {
                        let value = st_get(&st, 3);
                        st.push(value);
                    }
                    // 00e2d807: FDIV ST0,ST1
                    fpu_arith(&mut st, Fop::Div, 0, 1, false);
                    // 00e2d809: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(esp.wrapping_add(0x10), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d80d: FLD float ptr [ESP + 0x10]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x10))));
                    // 00e2d811: FMUL ST2
                    fpu_arith(&mut st, Fop::Mul, 0, 2, false);
                    // 00e2d813: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(esp.wrapping_add(0x14), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d817: FDIVP ST4,ST0
                    fpu_arith(&mut st, Fop::Div, 4, 0, true);
                    // 00e2d819: FXCH ST3
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 3);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 3, top);
                    }
                    // 00e2d81b: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d81f: FLD float ptr [ESP + 0x18]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x18))));
                    // 00e2d823: FMULP ST3
                    fpu_arith(&mut st, Fop::Mul, 3, 0, true);
                    // 00e2d825: FXCH ST2
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 2);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 2, top);
                    }
                    // 00e2d827: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d82b: FLD float ptr [ESP + 0x20]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x20))));
                    // 00e2d82f: JMP 0x00e2d873
                    pc = 0x00e2_d873;
                    continue;
                }
                0x00e2_d831 => {
                    // 00e2d831: FSTP ST2
                    {
                        let value = st_get(&st, 0);
                        st_set(&mut st, 2, value);
                    }
                    st.pop();
                    // 00e2d833: FXCH ST4
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 4);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 4, top);
                    }
                    // 00e2d835: FDIVRP
                    fpu_arith(&mut st, Fop::DivR, 1, 0, true);
                    // 00e2d837: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d83b: FLD ST0
                    {
                        let value = st_get(&st, 0);
                        st.push(value);
                    }
                    // 00e2d83d: FLD float ptr [ESP + 0x20]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x20))));
                    // 00e2d841: FLD ST0
                    {
                        let value = st_get(&st, 0);
                        st.push(value);
                    }
                    // 00e2d843: FMULP ST2
                    fpu_arith(&mut st, Fop::Mul, 2, 0, true);
                    // 00e2d845: FXCH
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 1);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 1, top);
                    }
                    // 00e2d847: FADD ST0,ST3
                    fpu_arith(&mut st, Fop::Add, 0, 3, false);
                    // 00e2d849: FLD ST3
                    {
                        let value = st_get(&st, 3);
                        st.push(value);
                    }
                    // 00e2d84b: FDIV ST0,ST1
                    fpu_arith(&mut st, Fop::Div, 0, 1, false);
                    // 00e2d84d: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(esp.wrapping_add(0x14), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d851: FLD ST1
                    {
                        let value = st_get(&st, 1);
                        st.push(value);
                    }
                    // 00e2d853: FMUL float ptr [ESP + 0x14]
                    fpu_arith_with(
                        &mut st,
                        Fop::Mul,
                        0,
                        x87_m32(e.mem.u32(esp.wrapping_add(0x14))),
                        false,
                    );
                    // 00e2d857: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(esp.wrapping_add(0x10), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d85b: FDIVP ST4,ST0
                    fpu_arith(&mut st, Fop::Div, 4, 0, true);
                    // 00e2d85d: FXCH ST3
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 3);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 3, top);
                    }
                    // 00e2d85f: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(esp.wrapping_add(0x20), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d863: FLD float ptr [ESP + 0x20]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x20))));
                    // 00e2d867: FLD ST0
                    {
                        let value = st_get(&st, 0);
                        st.push(value);
                    }
                    // 00e2d869: FMULP ST4
                    fpu_arith(&mut st, Fop::Mul, 4, 0, true);
                    // 00e2d86b: FXCH ST3
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 3);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 3, top);
                    }
                    // 00e2d86d: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d871: FXCH ST2
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 2);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 2, top);
                    }
                    pc = 0x00e2_d873;
                }
                0x00e2_d873 => {
                    // 00e2d873: FLD ST2
                    {
                        let value = st_get(&st, 2);
                        st.push(value);
                    }
                    // 00e2d875: FMUL ST1
                    fpu_arith(&mut st, Fop::Mul, 0, 1, false);
                    // 00e2d877: FADD float ptr [ESP + 0x18]
                    fpu_arith_with(
                        &mut st,
                        Fop::Add,
                        0,
                        x87_m32(e.mem.u32(esp.wrapping_add(0x18))),
                        false,
                    );
                    // 00e2d87b: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d87f: FLD float ptr [ESP + 0x18]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x18))));
                    // 00e2d883: FLD ST3
                    {
                        let value = st_get(&st, 3);
                        st.push(value);
                    }
                    // 00e2d885: FMULP ST4
                    fpu_arith(&mut st, Fop::Mul, 4, 0, true);
                    // 00e2d887: FLD ST2
                    {
                        let value = st_get(&st, 2);
                        st.push(value);
                    }
                    // 00e2d889: FSUBRP ST4,ST0
                    fpu_arith(&mut st, Fop::SubR, 4, 0, true);
                    // 00e2d88b: FLD ST1
                    {
                        let value = st_get(&st, 1);
                        st.push(value);
                    }
                    // 00e2d88d: FMULP ST4
                    fpu_arith(&mut st, Fop::Mul, 4, 0, true);
                    // 00e2d88f: FXCH ST3
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 3);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 3, top);
                    }
                    // 00e2d891: FMULP
                    fpu_arith(&mut st, Fop::Mul, 1, 0, true);
                    // 00e2d893: FLD ST2
                    {
                        let value = st_get(&st, 2);
                        st.push(value);
                    }
                    // 00e2d895: FMULP ST3
                    fpu_arith(&mut st, Fop::Mul, 3, 0, true);
                    // 00e2d897: FADDP ST2,ST0
                    fpu_arith(&mut st, Fop::Add, 2, 0, true);
                    // 00e2d899: FXCH
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 1);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 1, top);
                    }
                    // 00e2d89b: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d89f: FLDZ
                    st.push(0.0);
                    // 00e2d8a1: JMP 0x00e2d905
                    pc = 0x00e2_d905;
                    continue;
                }
                0x00e2_d8a3 => {
                    // 00e2d8a3: FSTP ST0
                    st.pop();
                    // 00e2d8a5: FSTP ST0
                    st.pop();
                    // 00e2d8a7: FCOMP ST4
                    fsw = fpu_compare(st_get(&st, 0), st_get(&st, 4));
                    st.pop();
                    // 00e2d8a9: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d8ab: TEST AH,0x41
                    fl.and((eax >> 8) & 0xff, 0x41_u32, 8);
                    // 00e2d8ae: JNZ 0x00e2d8e9
                    if !fl.zf {
                        pc = 0x00e2_d8e9;
                        continue;
                    }
                    pc = 0x00e2_d8b0;
                }
                0x00e2_d8b0 => {
                    // 00e2d8b0: FSTP ST1
                    {
                        let value = st_get(&st, 0);
                        st_set(&mut st, 1, value);
                    }
                    st.pop();
                    // 00e2d8b2: FLD1
                    st.push(1.0);
                    // 00e2d8b4: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(esp.wrapping_add(0x10), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d8b8: FXCH ST2
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 2);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 2, top);
                    }
                    // 00e2d8ba: FST float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(esp.wrapping_add(0x14), x87_store32(st_get(&st, 0)));
                    // 00e2d8be: FLD ST2
                    {
                        let value = st_get(&st, 2);
                        st.push(value);
                    }
                    // 00e2d8c0: FMULP ST3
                    fpu_arith(&mut st, Fop::Mul, 3, 0, true);
                    // 00e2d8c2: FXCH ST2
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 2);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 2, top);
                    }
                    // 00e2d8c4: JMP 0x00e2d8ff
                    pc = 0x00e2_d8ff;
                    continue;
                }
                0x00e2_d8c6 => {
                    // 00e2d8c6: FSTP ST0
                    st.pop();
                    // 00e2d8c8: FCOMP ST2
                    fsw = fpu_compare(st_get(&st, 0), st_get(&st, 2));
                    st.pop();
                    // 00e2d8ca: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d8cc: FSTP ST1
                    {
                        let value = st_get(&st, 0);
                        st_set(&mut st, 1, value);
                    }
                    st.pop();
                    // 00e2d8ce: TEST AH,0x5
                    fl.and((eax >> 8) & 0xff, 0x5_u32, 8);
                    // 00e2d8d1: JP 0x00e2d8e9
                    if fl.pf {
                        pc = 0x00e2_d8e9;
                        continue;
                    }
                    pc = 0x00e2_d8d3;
                }
                0x00e2_d8d3 => {
                    // 00e2d8d3: FSTP ST1
                    {
                        let value = st_get(&st, 0);
                        st_set(&mut st, 1, value);
                    }
                    st.pop();
                    // 00e2d8d5: FXCH ST2
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 2);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 2, top);
                    }
                    // 00e2d8d7: FST float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(esp.wrapping_add(0x10), x87_store32(st_get(&st, 0)));
                    // 00e2d8db: FLD1
                    st.push(1.0);
                    // 00e2d8dd: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(esp.wrapping_add(0x14), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d8e1: FLD ST2
                    {
                        let value = st_get(&st, 2);
                        st.push(value);
                    }
                    // 00e2d8e3: FMULP ST3
                    fpu_arith(&mut st, Fop::Mul, 3, 0, true);
                    // 00e2d8e5: FXCH ST2
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 2);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 2, top);
                    }
                    // 00e2d8e7: JMP 0x00e2d8ff
                    pc = 0x00e2_d8ff;
                    continue;
                }
                0x00e2_d8e9 => {
                    // 00e2d8e9: FSTP ST0
                    st.pop();
                    // 00e2d8eb: FLD float ptr [0x01016248]
                    st.push(x87_load32(e.mem.u32(0x1016248_u32)));
                    // 00e2d8f1: FST float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(esp.wrapping_add(0x10), x87_store32(st_get(&st, 0)));
                    // 00e2d8f5: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(esp.wrapping_add(0x14), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d8f9: FLD ST0
                    {
                        let value = st_get(&st, 0);
                        st.push(value);
                    }
                    // 00e2d8fb: FCHS
                    {
                        let top = st.pop().unwrap();
                        st.push(f64::from_bits(top.to_bits() ^ (1 << 63)));
                    }
                    // 00e2d8fd: FMULP
                    fpu_arith(&mut st, Fop::Mul, 1, 0, true);
                    pc = 0x00e2_d8ff;
                }
                0x00e2_d8ff => {
                    // 00e2d8ff: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(esp.wrapping_add(0x18), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d903: FXCH
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 1);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 1, top);
                    }
                    pc = 0x00e2_d905;
                }
                0x00e2_d905 => {
                    // 00e2d905: FLD float ptr [ESP + 0x18]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x18))));
                    // 00e2d909: FLD float ptr [ESP + 0x1c]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x1c))));
                    // 00e2d90d: FCOMP
                    fsw = fpu_compare(st_get(&st, 0), st_get(&st, 1));
                    st.pop();
                    // 00e2d90f: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d911: TEST AH,0x41
                    fl.and((eax >> 8) & 0xff, 0x41_u32, 8);
                    // 00e2d914: JNZ 0x00e2d93a
                    if !fl.zf {
                        pc = 0x00e2_d93a;
                        continue;
                    }
                    pc = 0x00e2_d916;
                }
                0x00e2_d916 => {
                    // 00e2d916: FLD float ptr [ESP + 0x10]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x10))));
                    // 00e2d91a: MOV EAX,dword ptr [EBP + 0x14]
                    eax = e.mem.u32(ebp.wrapping_add(0x14));
                    // 00e2d91d: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(esp.wrapping_add(0x24), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d921: MOV dword ptr [EAX],EDX
                    e.mem.set_u32(eax, edx);
                    // 00e2d923: FLD float ptr [ESP + 0x14]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x14))));
                    // 00e2d927: MOV EAX,dword ptr [EBP + 0x18]
                    eax = e.mem.u32(ebp.wrapping_add(0x18));
                    // 00e2d92a: FSTP float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(esp.wrapping_add(0x28), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d92e: MOV dword ptr [EAX],ECX
                    e.mem.set_u32(eax, ecx);
                    // 00e2d930: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(esp.wrapping_add(0x1c), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2d934: JMP 0x00e2d93c
                    pc = 0x00e2_d93c;
                    continue;
                }
                0x00e2_d936 => {
                    // 00e2d936: FSTP ST0
                    st.pop();
                    // 00e2d938: JMP 0x00e2d93e
                    pc = 0x00e2_d93e;
                    continue;
                }
                0x00e2_d93a => {
                    // 00e2d93a: FSTP ST0
                    st.pop();
                    pc = 0x00e2_d93c;
                }
                0x00e2_d93c => {
                    // 00e2d93c: FXCH
                    {
                        let top = st_get(&st, 0);
                        let other = st_get(&st, 1);
                        st_set(&mut st, 0, other);
                        st_set(&mut st, 1, top);
                    }
                    pc = 0x00e2_d93e;
                }
                0x00e2_d93e => {
                    // 00e2d93e: SUB ECX,0x50
                    ecx = fl.sub(ecx, 0x50_u32, 32);
                    // 00e2d941: SUB ESI,0x1
                    esi = fl.sub(esi, 0x1_u32, 32);
                    // 00e2d944: JNZ 0x00e2d735
                    if !fl.zf {
                        pc = 0x00e2_d735;
                        continue;
                    }
                    pc = 0x00e2_d94a;
                }
                0x00e2_d94a => {
                    // 00e2d94a: DEC EDI
                    edi = fl.dec(edi, 32);
                    // 00e2d94b: SUB EBX,0x50
                    ebx = fl.sub(ebx, 0x50_u32, 32);
                    // 00e2d94e: TEST EDI,EDI
                    fl.and(edi, edi, 32);
                    // 00e2d950: JGE 0x00e2d6fe
                    if fl.sf == fl.of {
                        pc = 0x00e2_d6fe;
                        continue;
                    }
                    pc = 0x00e2_d956;
                }
                0x00e2_d956 => {
                    // 00e2d956: FSTP ST0
                    st.pop();
                    pc = 0x00e2_d958;
                }
                0x00e2_d958 => {
                    // 00e2d958: MOV ECX,dword ptr [EBP + 0x14]
                    ecx = e.mem.u32(ebp.wrapping_add(0x14));
                    // 00e2d95b: FLD float ptr [ESP + 0x1c]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x1c))));
                    // 00e2d95f: MOV EDX,dword ptr [ECX]
                    edx = e.mem.u32(ecx);
                    // 00e2d961: FCOM double ptr [0x01012060]
                    fsw = fpu_compare(
                        st_get(&st, 0),
                        f64::from_bits(
                            u64::from(e.mem.u32(0x1012060_u32))
                                | (u64::from(e.mem.u32(0x1012060_u32.wrapping_add(4))) << 32),
                        ),
                    );
                    // 00e2d967: MOVSS XMM0,dword ptr [ESP + 0x24]
                    xmm[0] = [e.mem.u32(esp.wrapping_add(0x24)), 0, 0, 0];
                    // 00e2d96d: MOVAPS XMM1,xmmword ptr [EDX]
                    xmm[1] = load_lanes(e, edx);
                    // 00e2d970: MOV ESI,dword ptr [EBP + 0x10]
                    esi = e.mem.u32(ebp.wrapping_add(0x10));
                    // 00e2d973: MOV EAX,dword ptr [EBP + 0x18]
                    eax = e.mem.u32(ebp.wrapping_add(0x18));
                    // 00e2d976: MOVSS XMM4,dword ptr [0x01011d78]
                    xmm[4] = [e.mem.u32(0x1011d78_u32), 0, 0, 0];
                    // 00e2d97e: SHUFPS XMM0,XMM0,0x0
                    xmm[0] = shufps(xmm[0], xmm[0], 0);
                    // 00e2d982: MULPS XMM0,XMM1
                    xmm[0] = mulps(xmm[0], xmm[1]);
                    // 00e2d985: MOVAPS xmmword ptr [ESI],XMM0
                    store_lanes(e, esi, xmm[0]);
                    // 00e2d988: MOV ECX,dword ptr [EAX]
                    ecx = e.mem.u32(eax);
                    // 00e2d98a: FNSTSW AX
                    eax = (eax & 0xffff_0000) | u32::from(fsw);
                    // 00e2d98c: MOVSS XMM1,dword ptr [ESP + 0x28]
                    xmm[1] = [e.mem.u32(esp.wrapping_add(0x28)), 0, 0, 0];
                    // 00e2d992: MOVAPS XMM2,xmmword ptr [ECX]
                    xmm[2] = load_lanes(e, ecx);
                    // 00e2d995: MOVSS XMM5,dword ptr [0x01017718]
                    xmm[5] = [e.mem.u32(0x1017718_u32), 0, 0, 0];
                    // 00e2d99d: SHUFPS XMM1,XMM1,0x0
                    xmm[1] = shufps(xmm[1], xmm[1], 0);
                    // 00e2d9a1: MULPS XMM1,XMM2
                    xmm[1] = mulps(xmm[1], xmm[2]);
                    // 00e2d9a4: ADDPS XMM1,XMM0
                    xmm[1] = addps(xmm[1], xmm[0]);
                    // 00e2d9a7: MOVAPS XMM0,XMM1
                    xmm[0] = xmm[1];
                    // 00e2d9aa: MULPS XMM0,XMM1
                    xmm[0] = mulps(xmm[0], xmm[1]);
                    // 00e2d9ad: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2d9b0: SHUFPS XMM2,XMM0,0x55
                    xmm[2] = shufps(xmm[2], xmm[0], 85);
                    // 00e2d9b4: ADDSS XMM2,XMM0
                    xmm[2][0] = lane_add(xmm[2][0], xmm[0][0]);
                    // 00e2d9b8: SHUFPS XMM0,XMM0,0xaa
                    xmm[0] = shufps(xmm[0], xmm[0], 170);
                    // 00e2d9bc: ADDSS XMM0,XMM2
                    xmm[0][0] = lane_add(xmm[0][0], xmm[2][0]);
                    // 00e2d9c0: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2d9c3: RSQRTSS XMM2,XMM2
                    xmm[2][0] = rsqrt_estimate(f32::from_bits(xmm[2][0])).to_bits();
                    // 00e2d9c7: MOVAPS XMM3,XMM2
                    xmm[3] = xmm[2];
                    // 00e2d9ca: XORPS XMM2,XMM2
                    xmm[2] = xorps(xmm[2], xmm[2]);
                    // 00e2d9cd: MOVSS XMM2,XMM4
                    xmm[2][0] = xmm[4][0];
                    // 00e2d9d1: MOVAPS XMM4,XMM2
                    xmm[4] = xmm[2];
                    // 00e2d9d4: MOVAPS XMM2,XMM0
                    xmm[2] = xmm[0];
                    // 00e2d9d7: MULSS XMM0,XMM3
                    xmm[0][0] = lane_mul(xmm[0][0], xmm[3][0]);
                    // 00e2d9db: MULSS XMM0,XMM3
                    xmm[0][0] = lane_mul(xmm[0][0], xmm[3][0]);
                    // 00e2d9df: CMPEQSS XMM2,XMM4
                    xmm[2][0] = lane_cmpeq(xmm[2][0], xmm[4][0]);
                    // 00e2d9e4: XORPS XMM4,XMM4
                    xmm[4] = xorps(xmm[4], xmm[4]);
                    // 00e2d9e7: MOVSS XMM4,XMM5
                    xmm[4][0] = xmm[5][0];
                    // 00e2d9eb: MOVSS XMM5,dword ptr [0x01016248]
                    xmm[5] = [e.mem.u32(0x1016248_u32), 0, 0, 0];
                    // 00e2d9f3: SUBSS XMM4,XMM0
                    xmm[4][0] = lane_sub(xmm[4][0], xmm[0][0]);
                    // 00e2d9f7: XORPS XMM0,XMM0
                    xmm[0] = xorps(xmm[0], xmm[0]);
                    // 00e2d9fa: MOVSS XMM0,XMM5
                    xmm[0][0] = xmm[5][0];
                    // 00e2d9fe: MULSS XMM0,XMM3
                    xmm[0][0] = lane_mul(xmm[0][0], xmm[3][0]);
                    // 00e2da02: MULSS XMM0,XMM4
                    xmm[0][0] = lane_mul(xmm[0][0], xmm[4][0]);
                    // 00e2da06: ANDNPS XMM2,XMM0
                    xmm[2] = andnps(xmm[2], xmm[0]);
                    // 00e2da09: MOVAPS XMM0,XMM2
                    xmm[0] = xmm[2];
                    // 00e2da0c: SHUFPS XMM0,XMM2,0x0
                    xmm[0] = shufps(xmm[0], xmm[2], 0);
                    // 00e2da10: MULPS XMM0,XMM1
                    xmm[0] = mulps(xmm[0], xmm[1]);
                    // 00e2da13: MOVAPS xmmword ptr [ESI],XMM0
                    store_lanes(e, esi, xmm[0]);
                    // 00e2da16: TEST AH,0x5
                    fl.and((eax >> 8) & 0xff, 0x5_u32, 8);
                    // 00e2da19: JP 0x00e2da1f
                    if fl.pf {
                        pc = 0x00e2_da1f;
                        continue;
                    }
                    pc = 0x00e2_da1b;
                }
                0x00e2_da1b => {
                    // 00e2da1b: FSTP ST0
                    st.pop();
                    // 00e2da1d: JMP 0x00e2da21
                    pc = 0x00e2_da21;
                    continue;
                }
                0x00e2_da1f => {
                    // 00e2da1f: FSTP ST1
                    {
                        let value = st_get(&st, 0);
                        st_set(&mut st, 1, value);
                    }
                    st.pop();
                    pc = 0x00e2_da21;
                }
                0x00e2_da21 => {
                    // 00e2da21: FSTP float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(esp.wrapping_add(0x28), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2da25: FLD float ptr [ESP + 0x28]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x28))));
                    // 00e2da29: CALL 0x00ec6040
                    let bits = st.pop().unwrap().to_bits();
                    let result = e.call(0x00ec6040, &[bits as u32, (bits >> 32) as u32]);
                    eax = result.eax;
                    edx = result.edx;
                    st.push(result.st0);
                    // 00e2da2e: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(esp.wrapping_add(0x2c), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2da32: FLD float ptr [ESP + 0x2c]
                    st.push(x87_load32(e.mem.u32(esp.wrapping_add(0x2c))));
                    // 00e2da36: POP EDI
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2da37: FSTP float ptr [ESI + 0xc]
                    e.mem
                        .set_u32(esi.wrapping_add(0xc), x87_store32(st_get(&st, 0)));
                    st.pop();
                    // 00e2da3a: POP ESI
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2da3b: POP EBX
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2da3c: MOV ESP,EBP
                    esp = ebp;
                    // 00e2da3e: POP EBP
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4);
                    // 00e2da3f: RET
                    return;
                }
                _ => unreachable!("pc {pc:#x}"),
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e2cfe0, fn_00e2cfe0(Ptr)),
        entry!(0x00e2d0d0, fn_00e2d0d0(Ptr, Ptr, u32, Ptr, Ptr) -> i32),
        entry!(0x00e2d1b0, fn_00e2d1b0(Ptr, i32, Ptr, Ptr) -> u32),
        entry!(0x00e2d440, fn_00e2d440(Ptr)),
        entry!(0x00e2d550, fn_00e2d550(Ptr, Ptr) -> Ptr),
        entry!(0x00e2da40, fn_00e2da40(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e2db70, fn_00e2db70(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e2_b070, fn_00e2b070(Ptr, Ptr, u32, Ptr)),
        entry!(0x00e2_b510, fn_00e2b510(Ptr, Ptr, u32, Ptr)),
        entry!(0x00e2_b9b0, fn_00e2b9b0(Ptr, Ptr, Ptr)),
        entry!(0x00e2_baa0, fn_00e2baa0(Ptr, Ptr, u32)),
        entry!(
            0x00e2_baf0,
            fn_00e2baf0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00e2_bbd0,
            fn_00e2bbd0(
                u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32,
                u32, u32, u32, u32
            )
        ),
        entry!(
            0x00e2_c100,
            fn_00e2c100(
                u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32,
                u32, u32, u32, u32
            )
        ),
        entry!(0x00e2_c8a0, fn_00e2c8a0(u32, u32, u32, u32, u32, u32, u32)),
        entry!(0x00e2_cc30, fn_00e2cc30(u32, u32, u32, u32, u32, u32, u32)),
        entry!(0x00e2_d6c0, fn_00e2d6c0(u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(MAX_FLOAT, f32::from_bits(0x7f7f_ffee));
        e.set_global(MIN_FLOAT, f32::from_bits(0xff7f_ffee));
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.set_global(NEG_TINY, -1e-10f32 as f64);
        e.set_global(NINE_TENTHS, 0.9f32 as f64);
        e.set_global(SHRINK, 0.99f32 as f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(SMALL_STEP, 0.001f32);
        e
    }

    fn vector_at(e: &mut Engine, v: Vec4) -> u32 {
        let address = e.mem.alloc(16);
        store_vec(e, address, v);
        address
    }

    fn assert_close(actual: &[f32], expected: &[f32]) {
        for (a, b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 1e-4, "{actual:?} vs {expected:?}");
        }
    }

    /// A face whose three slots point at the given vertices.
    fn face_on(e: &mut Engine, vertices: [Vec4; 3]) -> u32 {
        let face = e.mem.alloc(0x50);
        for (k, v) in vertices.into_iter().enumerate() {
            let at = vector_at(e, v);
            e.mem.set_u32(face + 0x10 + 0x10 * k as u32, at);
        }
        face
    }

    #[test]
    fn face_plane_is_the_unit_normal_and_its_distance() {
        let mut e = engine();
        let face = face_on(
            &mut e,
            [
                [1.0, 0.0, 5.0, 0.0],
                [0.0, 0.0, 3.0, 0.0],
                [0.0, 1.0, 3.0, 0.0],
            ],
        );
        e.call(NORMALIZE_FACE, &args![face]);
        let root = 5.0f32.sqrt();
        assert_close(&load_vec(&e, face)[..3], &[2.0 / root, 0.0, -1.0 / root]);
        assert_close(&[e.mem.f32(face + 0x40)], &[-3.0 / root]);
    }

    #[test]
    fn face_plane_of_coincident_vertices_is_zero() {
        let mut e = engine();
        let same = [1.0, 2.0, 3.0, 0.0];
        let face = face_on(&mut e, [same, same, same]);
        e.call(NORMALIZE_FACE, &args![face]);
        assert_eq!(load_vec(&e, face), [0.0; 4]);
        assert_eq!(e.mem.f32(face + 0x40), 0.0);
    }

    /// A face with three records leading to three neighbour faces with the
    /// given states; returns (face, records, neighbours).
    fn ring(e: &mut Engine, states: [u32; 3]) -> (u32, [u32; 3], [u32; 3]) {
        let face = e.mem.alloc(0x50);
        let mut records = [0; 3];
        let mut neighbours = [0; 3];
        for k in 0..3 {
            let neighbour = e.mem.alloc(0x50);
            e.mem.set_u32(neighbour + 0x44, states[k]);
            let target = e.mem.alloc(0x40);
            let record = e.mem.alloc(0x10);
            e.mem.set_u32(record, target);
            e.mem.set_u32(record + 0xc, neighbour);
            e.mem.set_u32(face + 0x18 + 0x10 * k as u32, record);
            records[k] = record;
            neighbours[k] = neighbour;
        }
        (face, records, neighbours)
    }

    #[test]
    fn walk_links_neighbours_that_are_in_state_two() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        let (face, records, _) = ring(&mut e, [2, 5, 2]);
        let counter = e.mem.alloc(8);
        let result = e
            .call(WALK_FACES, &args![this, face, 7u32, counter, counter + 4])
            .u32() as i32;
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(counter), 2);
        assert_eq!(e.mem.u32(counter + 4), records[2]);
        for k in [0usize, 2] {
            let target = e.mem.u32(records[k]);
            assert_eq!(e.mem.u32(target + 0x30), records[k]);
        }
        let untouched = e.mem.u32(records[1]);
        assert_eq!(e.mem.u32(untouched + 0x30), 0);
        // The face is reset and pushed on the stack of processed faces.
        assert_eq!(e.mem.f32(face + 0x40), f32::from_bits(0x7f7f_ffee));
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0xde0), face);
    }

    #[test]
    fn walk_classifies_fresh_neighbours_and_stops_on_a_four() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        let (face, records, neighbours) = ring(&mut e, [0, 0, 0]);
        let results = [3u32, 2, 4];
        let seen: std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>> = Default::default();
        let log = seen.clone();
        e.register_double(FACE_SIDE_TEST, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            let index = log.borrow().len() - 1;
            Ret {
                eax: results[index],
                ..Ret::default()
            }
        });
        let counter = e.mem.alloc(8);
        let result = e
            .call(WALK_FACES, &args![this, face, 7u32, counter, counter + 4])
            .u32() as i32;
        assert_eq!(result, 2);
        assert_eq!(seen.borrow()[0], vec![neighbours[0], 7, this, this + 4]);
        assert_eq!(seen.borrow().len(), 3);
        // The result is saved in each neighbour's state.
        assert_eq!(e.mem.u32(neighbours[0] + 0x44), 3);
        assert_eq!(e.mem.u32(neighbours[1] + 0x44), 2);
        assert_eq!(e.mem.u32(neighbours[2] + 0x44), 4);
        // The state-2 neighbour was linked; the one that answered 4 was not.
        assert_eq!(e.mem.u32(counter), 1);
        assert_eq!(e.mem.u32(counter + 4), records[1]);
    }

    #[test]
    fn walk_recurses_into_a_neighbour_that_answers_one() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        let (face, _, neighbours) = ring(&mut e, [0, 5, 5]);
        // The neighbour's own neighbours are all finished (state 5).
        let (inner, _, _) = ring(&mut e, [5, 5, 5]);
        // Make the first record's neighbour the inner ring's face.
        let record = e.mem.u32(face + 0x18);
        e.mem.set_u32(record + 0xc, inner);
        e.register_double(FACE_SIDE_TEST, move |_, a| {
            assert_eq!(a[0], inner);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let counter = e.mem.alloc(8);
        let result = e
            .call(WALK_FACES, &args![this, face, 7u32, counter, counter + 4])
            .u32() as i32;
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(inner + 0x44), 1);
        // The inner face was walked first and pushed first.
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(this + 0xde0), inner);
        assert_eq!(e.mem.u32(this + 0xde4), face);
        let _ = neighbours;
    }

    #[test]
    fn walk_returns_two_when_the_face_stack_is_full() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        e.mem.set_u32(this + 0xc, 100);
        let (face, _, _) = ring(&mut e, [5, 5, 5]);
        let counter = e.mem.alloc(8);
        let result = e
            .call(WALK_FACES, &args![this, face, 7u32, counter, counter + 4])
            .u32() as i32;
        assert_eq!(result, 2);
        assert_eq!(e.mem.u32(this + 0xc), 100);
        assert_eq!(e.mem.f32(face + 0x40), f32::from_bits(0x7f7f_ffee));
    }

    /// A polytope with three vertex records and a closed list of two edges
    /// A->B and B->C around the apex `point`; returns (this, edge0, point).
    fn hole(e: &mut Engine, apex: Vec4) -> (u32, u32, u32, [u32; 3]) {
        let this = e.mem.alloc(0x1200);
        e.mem.set_u32(this + 8, 2);
        let a = e.mem.alloc(0x40);
        let b = e.mem.alloc(0x40);
        let c = e.mem.alloc(0x40);
        store_vec(e, a, [0.0, 0.0, 0.0, 0.0]);
        store_vec(e, b, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, c, [0.0, 1.0, 0.0, 0.0]);
        let mut edges = [0; 2];
        for (k, (from, to)) in [(a, b), (b, c)].into_iter().enumerate() {
            let edge = e.mem.alloc(0x10);
            let cell = e.mem.alloc(4);
            e.mem.set_u32(edge, from);
            e.mem.set_u32(edge + 4, cell);
            e.mem.set_u32(cell, to);
            edges[k] = edge;
        }
        e.mem.set_u32(b + 0x30, edges[1]);
        e.mem.set_u32(c + 0x30, edges[0]);
        let point = vector_at(e, apex);
        (this, edges[0], point, [a, b, c])
    }

    #[test]
    fn closing_a_hole_builds_a_ring_of_faces() {
        let mut e = engine();
        let (this, edge, point, [a, b, c]) = hole(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let result = e.call(0x00e2_d1b0, &args![this, 2i32, edge, point]).u32();
        assert_eq!(result, 0);
        let face0 = this + 0xf70;
        let face1 = this + 0xfc0;
        assert_eq!(e.mem.u32(this + 0x10), 2);
        // Face 0 sits on the edge A->B, face 1 on B->C.
        assert_eq!(e.mem.u32(face0 + 0x10), b);
        assert_eq!(e.mem.u32(face0 + 0x20), a);
        assert_eq!(e.mem.u32(face0 + 0x30), point);
        assert_eq!(e.mem.u32(face0 + 0x14), face0 + 0x20);
        assert_eq!(e.mem.u32(face0 + 0x24), face0 + 0x30);
        assert_eq!(e.mem.u32(face0 + 0x34), face0 + 0x10);
        assert_eq!(e.mem.u32(face0 + 0x1c), face0);
        assert_eq!(e.mem.u32(face0 + 0x18), edge);
        assert_eq!(e.mem.u32(edge + 8), face0 + 0x10);
        assert_eq!(e.mem.u32(face1 + 0x10), c);
        assert_eq!(e.mem.u32(face1 + 0x20), b);
        // Neighbour links, the last face wrapping to the first.
        assert_eq!(e.mem.u32(face0 + 0x38), face1 + 0x20);
        assert_eq!(e.mem.u32(face1 + 0x28), face0 + 0x30);
        assert_eq!(e.mem.u32(face1 + 0x38), face0 + 0x20);
        assert_eq!(e.mem.u32(face0 + 0x28), face1 + 0x30);
        // The planes and the cleared links.
        assert_close(&load_vec(&e, face0)[..3], &[0.0, 1.0, 0.0]);
        assert_close(&[e.mem.f32(face0 + 0x40)], &[0.0]);
        let third = 1.0 / 3.0f32.sqrt();
        assert_close(&load_vec(&e, face1)[..3], &[-third, -third, -third]);
        assert_close(&[e.mem.f32(face1 + 0x40)], &[-third]);
        assert_eq!(e.mem.u32(b + 0x30), 0);
        assert_eq!(e.mem.u32(c + 0x30), 0);
    }

    #[test]
    fn closing_a_hole_takes_recycled_faces_first() {
        let mut e = engine();
        let (this, edge, point, [_, b, _]) = hole(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let recycled = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0xc, 1);
        e.mem.set_u32(this + 0xde0, recycled);
        let result = e.call(0x00e2_d1b0, &args![this, 2i32, edge, point]).u32();
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // One face came off the stack, one is new.
        assert_eq!(e.mem.u32(this + 0x10), 1);
        assert_eq!(e.mem.u32(recycled + 0x10), b);
        assert_eq!(e.mem.u32(edge + 8), recycled + 0x10);
    }

    #[test]
    fn closing_a_hole_stops_at_a_degenerate_face() {
        let mut e = engine();
        // The apex coincides with the first vertex: a zero normal.
        let (this, edge, point, _) = hole(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let result = e.call(0x00e2_d1b0, &args![this, 2i32, edge, point]).u32();
        assert_eq!(result, 2);
        // The edge list running out ends the call with 2 as well.
        let result = e.call(0x00e2_d1b0, &args![this, 1i32, 0u32, point]).u32();
        assert_eq!(result, 2);
    }

    #[test]
    fn closing_a_hole_reports_an_open_ring() {
        let mut e = engine();
        let (this, edge, point, [_, _, c]) = hole(&mut e, [0.0, 0.0, 1.0, 0.0]);
        // Only the first edge is used: the walk ends on edge 1, not edge 0.
        let result = e.call(0x00e2_d1b0, &args![this, 1i32, edge, point]).u32();
        assert_eq!(result, 2);
        let _ = c;
    }

    #[test]
    fn first_faces_are_built_from_the_table() {
        let mut e = engine();
        for j in 0..12u32 {
            e.mem.set_u32(FACE_TABLE + 12 * j, 0x40 * (j % 4));
            e.mem.set_u32(FACE_TABLE + 12 * j + 4, 0x10 * j);
            e.mem.set_u32(FACE_TABLE + 12 * j + 8, 0x20);
        }
        let this = e.mem.alloc(0x1200);
        store_vec(&mut e, this + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, this + 0x60, [0.0; 4]);
        let faces: std::rc::Rc<std::cell::RefCell<Vec<u32>>> = Default::default();
        let log = faces.clone();
        let last = this + 0x1060;
        e.register_double(NORMALIZE_FACE, move |e, a| {
            log.borrow_mut().push(a[0]);
            if a[0] == last {
                store_vec(e, a[0], [0.0, 0.0, 1.0, 0.0]);
            }
            Ret::default()
        });
        e.call(0x00e2_d440, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), 4);
        // Face 1 (+0xfc0), slot 2: entry 5 of the table.
        let face = this + 0xfc0;
        assert_eq!(e.mem.u32(face + 0x20), this + 0x20);
        assert_eq!(e.mem.u32(face + 0x28), 0x20 + 0x10 * 4 + this + 0xf80);
        assert_eq!(e.mem.u32(face + 0x2c), face);
        assert_eq!(e.mem.u32(face + 0x14), face + 0x20);
        assert_eq!(e.mem.u32(face + 0x24), face + 0x30);
        assert_eq!(e.mem.u32(face + 0x34), face + 0x10);
        assert_eq!(e.mem.u32(face + 0x44), 0);
        // The last face's normal faces the first vertex: three slots are redirected.
        assert_eq!(e.mem.u32(this + 0x1020), this + 0x60);
        assert_eq!(e.mem.u32(this + 0xfd0), this + 0x60);
        assert_eq!(e.mem.u32(this + 0xf80), this + 0x60);
        assert_eq!(e.mem.u32(last + 0x10), this + 0x20);
        assert_eq!(e.mem.u32(this + 0xfe0), this + 0x20);
        assert_eq!(e.mem.u32(this + 0xfa0), this + 0x20);
        assert_eq!(
            *faces.borrow(),
            vec![
                last,
                this + 0xf70,
                this + 0xfc0,
                this + 0x1010,
                this + 0x1060
            ]
        );
    }

    #[test]
    fn first_faces_keep_their_vertices_when_the_normal_points_away() {
        let mut e = engine();
        let this = e.mem.alloc(0x1200);
        store_vec(&mut e, this + 0x20, [0.0, 0.0, -1.0, 0.0]);
        store_vec(&mut e, this + 0x60, [0.0; 4]);
        let last = this + 0x1060;
        e.register_double(NORMALIZE_FACE, move |e, a| {
            if a[0] == last {
                store_vec(e, a[0], [0.0, 0.0, 1.0, 0.0]);
            }
            Ret::default()
        });
        e.call(0x00e2_d440, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1020), e.mem.u32(this + 0x1020));
        assert_ne!(e.mem.u32(this + 0x1020), this + 0x60);
        assert_ne!(e.mem.u32(this + 0xfd0), this + 0x60);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A 0x60-byte-entry list: `count` entries in use with the given keys, capacity `capacity`.
    /// Returns (store, entries).
    fn store(e: &mut Engine, keys: &[f32], capacity: i32, count: i32) -> (u32, u32) {
        let store = e.mem.alloc(0x20);
        let entries = e.mem.alloc(0x60 * 4);
        for (index, key) in keys.iter().enumerate() {
            e.mem.set_f32(entries + 0x60 * index as u32 + 0x10, *key);
        }
        e.mem.set_u32(store + 8, entries);
        e.mem.set_u32(store + 0xc, entries + 0x60 * count as u32);
        e.mem.set_i32(store + 0x10, capacity);
        e.mem.set_i32(store + 0x14, count);
        (store, entries)
    }

    /// A chain of three nodes linked through +0xc whose +4 words are 0x111, 0x222, 0x333.
    fn chain(e: &mut Engine) -> [u32; 3] {
        let nodes = [e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10)];
        for (index, node) in nodes.iter().enumerate() {
            e.mem.set_u32(node + 4, 0x111 * (index as u32 + 1));
            e.mem
                .set_u32(node + 0xc, nodes.get(index + 1).copied().unwrap_or(0));
        }
        nodes
    }

    fn candidate(e: &mut Engine, key: f32) -> u32 {
        let candidate = e.mem.alloc(0x20);
        for word in 0..4u32 {
            e.mem.set_u32(candidate + 4 * word, 0xc0de_0000 + word);
        }
        e.mem.set_f32(candidate + 0x10, key);
        e.mem.set_u32(candidate + 0x14, 0xfeed);
        candidate
    }

    /// With room the entry at the write pointer is used; the candidate's words, the last chain
    /// node and the two chain words (plus the terminator) end up in it.
    #[test]
    fn fn_00e2b9b0_fills_the_next_free_entry() {
        let mut e = Engine::new();
        let (store, entries) = store(&mut e, &[], 3, 0);
        let nodes = chain(&mut e);
        let candidate = candidate(&mut e, 2.5);
        e.call(0x00e2_b9b0, &args![nodes[0], candidate, store]);
        assert_eq!(e.mem.i32(store + 0x14), 1);
        assert_eq!(e.mem.u32(store + 0xc), entries + 0x60);
        assert_eq!(e.mem.f32(entries + 0x10), 2.5);
        for word in 0..4u32 {
            assert_eq!(e.mem.u32(entries + 4 * word), 0xc0de_0000 + word);
        }
        assert_eq!(e.mem.u32(entries + 0x14), 0xfeed);
        assert_eq!(e.mem.u32(entries + 0x50), nodes[2]);
        assert_eq!(e.mem.u32(entries + 0x20), 0x222);
        assert_eq!(e.mem.u32(entries + 0x24), 0x111);
        assert_eq!(e.mem.u32(entries + 0x28), 0xffff_ffff);
    }

    /// A non-zero override pointer replaces the last-node search; a chain without links gives
    /// only the terminator at word 0.
    #[test]
    fn fn_00e2b9b0_uses_the_override_pointer() {
        let mut e = Engine::new();
        let (store, entries) = store(&mut e, &[], 3, 0);
        e.mem.set_u32(store + 0x18, 0x4242);
        let lone = e.mem.alloc(0x10);
        let candidate = candidate(&mut e, 1.0);
        e.call(0x00e2_b9b0, &args![lone, candidate, store]);
        assert_eq!(e.mem.u32(entries + 0x50), 0x4242);
        assert_eq!(e.mem.u32(entries + 0x20), 0xffff_ffff);
    }

    /// A chain without links and no override: the entry's last node is the chain's own node.
    #[test]
    fn fn_00e2b9b0_single_node_chain_is_its_own_last_node() {
        let mut e = Engine::new();
        let (store, entries) = store(&mut e, &[], 3, 0);
        let lone = e.mem.alloc(0x10);
        let candidate = candidate(&mut e, 1.0);
        e.call(0x00e2_b9b0, &args![lone, candidate, store]);
        assert_eq!(e.mem.u32(entries + 0x50), lone);
    }

    /// A full list replaces its largest key (here entry 1) when the candidate's key is smaller.
    #[test]
    fn fn_00e2b9b0_replaces_the_largest_key_when_full() {
        let mut e = Engine::new();
        let (store, entries) = store(&mut e, &[1.0, 5.0, 3.0], 3, 3);
        let nodes = chain(&mut e);
        let candidate = candidate(&mut e, 2.0);
        e.call(0x00e2_b9b0, &args![nodes[0], candidate, store]);
        assert_eq!(e.mem.f32(entries + 0x10), 1.0);
        assert_eq!(e.mem.f32(entries + 0x60 + 0x10), 2.0);
        assert_eq!(e.mem.f32(entries + 0xc0 + 0x10), 3.0);
        assert_eq!(e.mem.i32(store + 0x14), 3);
        assert_eq!(e.mem.u32(entries + 0x60 + 0x50), nodes[2]);
    }

    /// A full list is left alone when the candidate's key is not smaller than the largest
    /// (greater, equal or a NaN).
    #[test]
    fn fn_00e2b9b0_keeps_a_full_list_when_the_candidate_is_not_smaller() {
        for key in [9.0f32, 5.0, f32::NAN] {
            let mut e = Engine::new();
            let (store, entries) = store(&mut e, &[1.0, 5.0, 3.0], 3, 3);
            let nodes = chain(&mut e);
            let candidate = candidate(&mut e, key);
            e.call(0x00e2_b9b0, &args![nodes[0], candidate, store]);
            assert_eq!(e.mem.f32(entries + 0x60 + 0x10), 5.0, "key {key}");
            assert_eq!(e.mem.u32(entries + 0x60 + 0x50), 0);
        }
    }

    /// A NaN key in the list never counts as larger: the first entry stays the victim.
    #[test]
    fn fn_00e2b9b0_a_nan_in_the_list_is_not_a_victim() {
        let mut e = Engine::new();
        let (store, entries) = store(&mut e, &[4.0, f32::NAN, 3.0], 3, 3);
        let nodes = chain(&mut e);
        let candidate = candidate(&mut e, 1.0);
        e.call(0x00e2_b9b0, &args![nodes[0], candidate, store]);
        assert_eq!(e.mem.f32(entries + 0x10), 1.0);
    }

    /// The fields are copied, the pointer words depend on the byte at +0x20 and the extra word.
    #[test]
    fn fn_00e2baa0_initialises_the_record() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x60);
        e.mem.write(object, &[0x55; 0x60]);
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source + 0x30, 0x3030);
        e.mem.set_u32(source + 0x24, 0x2424);
        e.mem.set_u8(source + 0x20, 1);
        e.call(0x00e2_baa0, &args![object, source, 0x100u32]);
        assert_eq!(e.mem.f32(object + 0x14), 1.0);
        assert_eq!(e.mem.u8(object + 0x18), 0);
        assert_eq!(e.mem.u32(object + 0x1c), source);
        assert_eq!(e.mem.u32(object + 0x20), source);
        assert_eq!(e.mem.u32(object + 0x24), 0x3030);
        assert_eq!(e.mem.u32(object + 0x28), 0x3030);
        assert_eq!(e.mem.u32(object + 0x50), 0x2424);
        assert_eq!(e.mem.u32(object + 0x54), 0x110);
        assert_eq!(e.mem.u32(object + 0x58), 0x5555_5555);
        // A zero byte, or a zero extra word, gives 0.
        e.mem.set_u8(source + 0x20, 0);
        e.call(0x00e2_baa0, &args![object, source, 0x100u32]);
        assert_eq!(e.mem.u32(object + 0x54), 0);
        e.mem.set_u8(source + 0x20, 0x80);
        e.call(0x00e2_baa0, &args![object, source, 0x100u32]);
        assert_eq!(e.mem.u32(object + 0x54), 0x110);
        e.call(0x00e2_baa0, &args![object, source, 0u32]);
        assert_eq!(e.mem.u32(object + 0x54), 0);
    }

    /// Eight vectors with lane values 10 * index + lane become six rows.
    #[test]
    fn fn_00e2baf0_transposes_eight_vectors_into_six() {
        let mut e = Engine::new();
        let vectors: Vec<u32> = (0..8u32)
            .map(|index| {
                let at = e.mem.alloc(0x10);
                for lane in 0..4u32 {
                    e.mem.set_u32(at + 4 * lane, 10 * index + lane);
                }
                at
            })
            .collect();
        let out = e.mem.alloc(0x80);
        e.mem.write(out, &[0x77; 0x80]);
        let mut words = vec![out];
        words.extend(vectors.iter().copied());
        words.push(0x1234);
        e.call(0x00e2_baf0, &words);
        let rows: Vec<Vec<u32>> = (0..6u32)
            .map(|row| {
                (0..4u32)
                    .map(|lane| e.mem.u32(out + 0x10 * row + 4 * lane))
                    .collect()
            })
            .collect();
        // a, c, e, g are vectors 0, 2, 4, 6; b, d, f, h are 1, 3, 5, 7.
        assert_eq!(rows[0], vec![0, 20, 40, 60]);
        assert_eq!(rows[1], vec![1, 21, 41, 61]);
        assert_eq!(rows[2], vec![2, 22, 42, 62]);
        assert_eq!(rows[3], vec![10, 30, 50, 70]);
        assert_eq!(rows[4], vec![11, 31, 51, 71]);
        assert_eq!(rows[5], vec![12, 32, 52, 72]);
        assert_eq!(e.mem.u32(out + 0x60), 0x1234);
        assert_eq!(e.mem.u32(out + 0x64), 0x7777_7777);
    }

    /// The frame builders: the two big callees are doubles; `00d1d530` (or `00d1d490`) checks
    /// what the frame holds when it is called and changes four frame words and the vector copy,
    /// which the function then writes back.
    fn check_frame_builder(address: u32, final_callee: u32, pointer_shift: u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let first_calls = Rc::clone(&calls);
        e.register_double(0x00d1_bb50, move |_, a| {
            first_calls.borrow_mut().push((0x00d1_bb50, a.to_vec()));
            Ret::default()
        });
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 0x10, 0xabc0);
        let source = e.mem.alloc(0x100);
        for word in 0..0x3d {
            e.mem.set_u32(source + 4 * word, 0x1000 + word);
        }
        // The byte flags and the table index the function reads.
        e.mem.set_u8(source + 0x20, 7);
        e.mem.set_u8(source + 0xc4, 9);
        e.mem.set_u32(source + 0xc0, 2);
        e.set_global(0x011b_3830 + 8, 0x7e57u32);
        let vector = e.mem.alloc(0x10);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(vector + 4 * lane as u32, value);
        }
        let last_calls = Rc::clone(&calls);
        e.register_double(final_callee, move |e, a| {
            last_calls.borrow_mut().push((final_callee, a.to_vec()));
            let frame = a[3] - 0x340;
            // The record vectors copied from the source, the vector copy and the sources of
            // the written-back words.
            for word in 0..4u32 {
                assert_eq!(e.mem.u32(frame + 0x30 + 4 * word), 0x1000 + word);
                assert_eq!(e.mem.u32(frame + 0x40 + 4 * word), 0x1004 + word);
            }
            for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                assert_eq!(e.mem.f32(a[5] + 4 * lane as u32), value);
            }
            assert_eq!(a[4], frame + 0x20);
            assert_eq!(a[5], frame + 0x10);
            let objects: Vec<u32> = (0..4u32).map(|slot| e.mem.u32(a[4] + 4 * slot)).collect();
            let expected: Vec<u32> = [0x138u32, 0x1b8, 0x238, 0x2b8]
                .iter()
                .map(|offset| frame + offset + pointer_shift)
                .collect();
            assert_eq!(objects, expected);
            // The table word at the end of the frame (`011b3830 + 4 * [source + 0xc0]`).
            assert_eq!(e.mem.u32(frame + 0x3a0), 0x7e57);
            for (offset, word) in [
                (0x68u32, 0xa1u32),
                (0xa8, 0xa2),
                (0xe8, 0xa3),
                (0x128, 0xa4),
            ] {
                e.mem.set_u32(frame + offset, word);
            }
            for (lane, value) in [5.0f32, 6.0, 7.0, 8.0].into_iter().enumerate() {
                e.mem.set_f32(a[5] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e.call(address, &args![owner, source, 0x7777u32, vector]);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 2);
        let (first_address, first) = &calls[0];
        assert_eq!(*first_address, 0x00d1_bb50);
        assert_eq!(
            first[1..],
            [
                source,
                source + 0x10,
                source + 0x30,
                source + 0x40,
                source + 0x60,
                source + 0x70,
                source + 0x90,
                source + 0xa0,
                0xf
            ]
        );
        let (_, last) = &calls[1];
        assert_eq!(last[..3], [owner, 0xabc0, 0]);
        assert_eq!(first[0], last[3]);
        for (offset, word) in [(0xe0u32, 0xa1u32), (0xe4, 0xa2), (0xe8, 0xa3), (0xec, 0xa4)] {
            assert_eq!(e.mem.u32(source + offset), word);
        }
        let result: Vec<f32> = (0..4u32).map(|lane| e.mem.f32(vector + 4 * lane)).collect();
        assert_eq!(result, vec![5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn fn_00e2b070_builds_the_frame_and_writes_the_results_back() {
        check_frame_builder(0x00e2_b070, 0x00d1_d530, 0);
    }

    #[test]
    fn fn_00e2b510_builds_the_frame_and_writes_the_results_back() {
        check_frame_builder(0x00e2_b510, 0x00d1_d490, 4);
    }

    /// A null owner does nothing at all.
    #[test]
    fn frame_builders_do_nothing_for_a_null_owner() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00e2_b070, &args![0u32, 0x1000u32, 0u32, 0x2000u32]);
        e.call(0x00e2_b510, &args![0u32, 0x1000u32, 0u32, 0x2000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod tests_a05_machine {
    use super::*;

    fn new_engine() -> Engine {
        Engine::new()
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ONE: u32 = 0x0101_2068;
    const VIRTUAL: u32 = 0x0900_0001;

    /// An engine with the exe's data region mapped and `1.0f` where the routines load it.
    fn engine() -> Engine {
        let mut e = new_engine();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ONE, 0x3f80_0000u32);
        e
    }

    /// A double at `address` that logs its words (`eax` as given).
    fn logging(e: &mut Engine, calls: &Calls, address: u32, eax: u32) {
        let log = Rc::clone(calls);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn put_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
    }

    /// Three nodes of 0x20 bytes (a lower corner, a scale vector of ones): the lower corners
    /// are 0.25, 2.0 and 1.2 in x, y and z.
    fn nodes(e: &mut Engine) -> u32 {
        let table = e.mem.alloc(0x80);
        for (index, lower) in [0.25f32, 2.0, 1.2].into_iter().enumerate() {
            let at = table + 32 * index as u32;
            put_floats(e, at, &[lower, lower, lower, 0.0]);
            put_floats(e, at + 0x10, &[1.0, 1.0, 1.0, 0.0]);
        }
        table
    }

    fn words(e: &mut Engine, values: &[u32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32 + 8);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * index as u32, *value);
        }
        at
    }

    /// A collector whose embedded object at `+offset` has a vtable whose slot 1 is `VIRTUAL`.
    fn collector(e: &mut Engine, offset: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, VIRTUAL);
        e.mem.set_u32(object + offset, vtable);
        object
    }

    // ---- the flags and lane helpers ------------------------------------------------------

    /// The conditions the exe's jumps test agree with the integer comparisons they stand for,
    /// at every width.
    #[test]
    fn flags_after_a_compare_match_the_integer_comparisons() {
        let samples = [
            0u32,
            1,
            2,
            0x7f,
            0x80,
            0xff,
            0x7fff,
            0x8000,
            0xffff,
            0x7fff_ffff,
            0x8000_0000,
            0xffff_ffff,
            5,
            0x1234_5678,
        ];
        for bits in [8u32, 16, 32] {
            let mask = width_mask(bits);
            let signed =
                |v: u32| -> i64 { ((u64::from(v & mask) << (64 - bits)) as i64) >> (64 - bits) };
            for &a in &samples {
                for &b in &samples {
                    let mut fl = Flags::default();
                    let result = fl.sub(a, b, bits);
                    assert_eq!(result, a.wrapping_sub(b) & mask);
                    assert_eq!(fl.zf, a & mask == b & mask);
                    assert_eq!(
                        fl.sf != fl.of,
                        signed(a) < signed(b),
                        "JL {a:#x} {b:#x} {bits}"
                    );
                    assert_eq!(fl.zf || fl.sf != fl.of, signed(a) <= signed(b));
                    assert_eq!(!fl.zf && fl.sf == fl.of, signed(a) > signed(b));
                    assert_eq!(fl.cf, (a & mask) < (b & mask));
                    assert_eq!(fl.cf || fl.zf, (a & mask) <= (b & mask));
                    let mut fl = Flags::default();
                    let sum = fl.add(a, b, bits);
                    assert_eq!(sum, a.wrapping_add(b) & mask);
                    assert_eq!(
                        fl.cf,
                        u64::from(a & mask) + u64::from(b & mask) > u64::from(mask)
                    );
                    assert_eq!(fl.of, signed(a) + signed(b) != signed(sum));
                }
            }
        }
    }

    /// `TEST`/`AND`/`XOR`/`INC`/`DEC`, the parity flag and the shifts.
    #[test]
    fn flags_after_logic_inc_dec_and_shifts() {
        let mut fl = Flags::default();
        assert_eq!(fl.and(0xf0, 0x0f, 8), 0);
        assert!(fl.zf && !fl.sf && fl.pf && !fl.cf && !fl.of);
        assert_eq!(fl.or(0x80, 0x01, 8), 0x81);
        assert!(!fl.zf && fl.sf && fl.pf);
        assert_eq!(fl.xor(0x07, 0x00, 8), 0x07);
        assert!(!fl.pf);
        fl.cf = true;
        assert_eq!(fl.inc(u32::MAX, 32), 0);
        assert!(fl.zf && fl.cf);
        assert_eq!(fl.dec(0, 32), u32::MAX);
        assert!(fl.sf && fl.cf);
        assert_eq!(fl.shift_shl(0x8000_0001, 1, 32), 2);
        assert!(fl.cf);
        assert_eq!(fl.shift_shl(5, 4, 32), 0x50);
        assert!(!fl.cf);
        assert_eq!(fl.shift_shl(0xffff_ffff, 0, 32), 0xffff_ffff);
        assert_eq!(fl.shift_sar(0x8000_0000, 31, 32), 0xffff_ffff);
        assert_eq!(fl.shift_sar(0xfffc, 2, 16), 0xffff);
        assert_eq!(fl.shift_sar(0x7fff_ffff, 30, 32), 1);
        assert!(fl.cf);
    }

    /// The SSE lane helpers: shuffles, min/max picking the second operand for NaNs and equal
    /// values, compares, masks.
    #[test]
    fn lane_helpers() {
        let a = [1u32, 2, 3, 4];
        let b = [5u32, 6, 7, 8];
        assert_eq!(shufps(a, b, 0x44), [1, 2, 5, 6]);
        assert_eq!(shufps(a, b, 0xee), [3, 4, 7, 8]);
        assert_eq!(shufps(a, b, 0x55), [2, 2, 6, 6]);
        assert_eq!(shufps(a, b, 0xaa), [3, 3, 7, 7]);
        assert_eq!(shufps(a, b, 0xc9), [2, 3, 5, 8]);
        assert_eq!(unpcklps(a, b), [1, 5, 2, 6]);
        let f = |x: f32| x.to_bits();
        let low = [f(1.0), f(f32::NAN), f(0.0), f(-0.0)];
        let high = [f(2.0), f(3.0), f(-0.0), f(0.0)];
        assert_eq!(minps(low, high), [f(1.0), f(3.0), f(-0.0), f(0.0)]);
        assert_eq!(maxps(low, high), [f(2.0), f(3.0), f(-0.0), f(0.0)]);
        assert_eq!(cmpltps(low, high), [u32::MAX, 0, 0, 0]);
        assert_eq!(lane_cmpeq(f(1.0), f(1.0)), u32::MAX);
        assert_eq!(lane_cmpeq(f(f32::NAN), f(f32::NAN)), 0);
        assert_eq!(movmskps([0x8000_0000, 0, 0xffff_ffff, 1]), 0b0101);
        assert_eq!(
            xorps([0xf0, 0x0f, 1, 2], [0xff, 0xff, 1, 3]),
            [0x0f, 0xf0, 0, 1]
        );
        assert_eq!(
            andnps([0xf0, 0x0f, 1, 2], [0xff, 0xff, 1, 3]),
            [0x0f, 0xf0, 0, 1]
        );
        assert_eq!(mulps([f(2.0); 4], [f(3.0); 4]), [f(6.0); 4]);
        assert_eq!(addps([f(2.0); 4], [f(3.0); 4]), [f(5.0); 4]);
        assert_eq!(subps([f(2.0); 4], [f(3.0); 4]), [f(-1.0); 4]);
        // The first operand's NaN wins in the arithmetic.
        assert_eq!(addps([0x7fc0_0001; 4], [0x7fc0_0002; 4]), [0x7fc0_0001; 4]);
    }

    /// The x87 helpers: the arithmetic forms, the reversed forms and the compare flags.
    #[test]
    fn x87_helpers() {
        // ST0 = 10, ST1 = 3, ST2 = 2.
        let mut st = vec![2.0, 3.0, 10.0];
        fpu_arith(&mut st, Fop::Sub, 0, 1, false);
        assert_eq!(st_get(&st, 0), 7.0);
        fpu_arith(&mut st, Fop::SubR, 0, 1, false);
        assert_eq!(st_get(&st, 0), -4.0);
        fpu_arith(&mut st, Fop::Div, 1, 0, true);
        assert_eq!(st.len(), 2);
        assert_eq!(st_get(&st, 0), -0.75);
        fpu_arith_with(&mut st, Fop::DivR, 0, 3.0, false);
        assert_eq!(st_get(&st, 0), -4.0);
        assert_eq!(fpu_compare(1.0, 2.0), 0x0100);
        assert_eq!(fpu_compare(2.0, 2.0), 0x4000);
        assert_eq!(fpu_compare(3.0, 2.0), 0);
        assert_eq!(fpu_compare(f64::NAN, 2.0), 0x4500);
    }

    // ---- 00e2bbd0 and 00e2c100: the leaf step ------------------------------------------

    /// Twenty words for `00e2bbd0`/`00e2c100`.
    fn twenty(set: &[(usize, u32)]) -> Vec<u32> {
        let mut words = vec![0u32; 20];
        for (index, value) in set {
            words[*index] = *value;
        }
        words
    }

    /// With bit 0x8000 set, every listed node is slab-tested against the ray (0, 0, 0) to
    /// (1, 1, 1): node 0 (lower corner 0.25) is hit, node 1 (2.0) is missed. The hit goes to
    /// `00e2baa0` and the collector's virtual method; both output counts are cleared first.
    #[test]
    fn fn_00e2bbd0_tests_the_listed_nodes_against_the_ray() {
        let mut e = engine();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        logging(&mut e, &calls, 0x00e2_baa0, 0);
        logging(&mut e, &calls, VIRTUAL, 0);
        let ray = e.mem.alloc(0x30);
        put_floats(&mut e, ray, &[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
        e.mem.set_u16(ray + 0x24, 0x8000);
        let table = nodes(&mut e);
        let hits = collector(&mut e, 8);
        let list = words(&mut e, &[0, 1]);
        let (count_a, count_b) = (words(&mut e, &[99]), words(&mut e, &[99]));
        let args = twenty(&[
            (1, ray),
            (3, table),
            (4, hits),
            (6, 0x5000),
            (7, 0x77),
            (9, list),
            (10, 2),
            (15, count_a),
            (17, count_b),
        ]);
        e.call(0x00e2_bbd0, &args);
        assert_eq!(e.mem.u32(count_a), 0);
        assert_eq!(e.mem.u32(count_b), 0);
        assert_eq!(
            *calls.borrow(),
            vec![
                (0x00e2_baa0, vec![hits, 0x5000, 0x77]),
                (VIRTUAL, vec![hits + 8, ray]),
            ]
        );
    }

    /// No node is hit when the ray misses all of them; nothing is called.
    #[test]
    fn fn_00e2bbd0_calls_nothing_for_a_miss() {
        let mut e = engine();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        logging(&mut e, &calls, 0x00e2_baa0, 0);
        logging(&mut e, &calls, VIRTUAL, 0);
        let ray = e.mem.alloc(0x30);
        put_floats(&mut e, ray, &[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
        e.mem.set_u16(ray + 0x24, 0x8000);
        let table = nodes(&mut e);
        let list = words(&mut e, &[1]);
        let counts = words(&mut e, &[0, 0]);
        let args = twenty(&[
            (1, ray),
            (3, table),
            (4, 0x1000),
            (9, list),
            (10, 1),
            (15, counts),
            (17, counts + 4),
        ]);
        e.call(0x00e2_bbd0, &args);
        assert!(calls.borrow().is_empty());
    }

    /// The long-list path: `00d1bb40` is asked for two ray blocks (called with `arg0 + 0x1c`),
    /// `00d9a4e0` copies them into the two ray buffers, and each of the five listed nodes is
    /// tested against both rays: ray A (0 to 1) hits node 0 only, ray B (0.5 to 1.5) hits
    /// nodes 0 and 2 (lower corner 1.2).
    #[test]
    fn fn_00e2bbd0_tests_two_rays_against_a_long_list() {
        let mut e = engine();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let first = e.mem.alloc(0x40);
        put_floats(&mut e, first, &[0.5, 0.5, 0.5, 0.0, 1.5, 1.5, 1.5, 0.0]);
        let second = e.mem.alloc(0x40);
        put_floats(&mut e, second, &[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
        let results = Rc::new(RefCell::new(vec![first, second]));
        let log = Rc::clone(&calls);
        e.register_double(0x00d1_bb40, move |_, a| {
            log.borrow_mut().push((0x00d1_bb40, a.to_vec()));
            let next = results.borrow_mut().remove(0);
            Ret {
                eax: next,
                ..Ret::default()
            }
        });
        let log = Rc::clone(&calls);
        e.register_double(0x00d9_a4e0, move |e, a| {
            log.borrow_mut().push((0x00d9_a4e0, a.to_vec()));
            for word in 0..8 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        let ray = e.mem.alloc(0x30);
        e.mem.set_u16(ray + 0x22, 7);
        e.mem.set_u16(ray + 0x20, 9);
        let table = nodes(&mut e);
        let list = words(&mut e, &[0, 1, 2, 1, 1]);
        let this = e.mem.alloc(0x40);
        let (count_a, count_b) = (words(&mut e, &[99]), words(&mut e, &[99]));
        let (hits_a, hits_b) = (words(&mut e, &[0; 8]), words(&mut e, &[0; 8]));
        let (buffer_a, buffer_b) = (e.mem.alloc(0x40), e.mem.alloc(0x40));
        let args = twenty(&[
            (0, this),
            (1, ray),
            (3, table),
            (9, list),
            (10, 5),
            (11, 2),
            (12, 5),
            (13, 5),
            (14, hits_a),
            (15, count_a),
            (16, hits_b),
            (17, count_b),
            (18, buffer_a),
            (19, buffer_b),
        ]);
        e.call(0x00e2_bbd0, &args);
        assert_eq!(
            *calls.borrow(),
            vec![
                (0x00d1_bb40, vec![this + 0x1c, 7]),
                (0x00d1_bb40, vec![this + 0x1c, 9]),
                (0x00d9_a4e0, vec![buffer_a, second]),
                (0x00d9_a4e0, vec![buffer_b, first]),
            ]
        );
        assert_eq!(e.mem.u32(count_a), 1);
        assert_eq!(e.mem.u32(hits_a), 0);
        assert_eq!(e.mem.u32(count_b), 2);
        assert_eq!(e.mem.u32(hits_b), 0);
        assert_eq!(e.mem.u32(hits_b + 4), 2);
    }

    /// The short-list path: one leaf goes through `00e2baa0`, `00e2baf0` and `00d1ca00`, and the
    /// value `00d1ca00` leaves at `+0x70` of the output frame is stored in the leaf's slot of
    /// the table.
    #[test]
    fn fn_00e2bbd0_short_list_runs_the_leaf_solver() {
        let mut e = engine();
        e.set_global(0x011b_3834, 0x4444u32);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        logging(&mut e, &calls, 0x00e2_baa0, 0);
        logging(&mut e, &calls, 0x00e2_baf0, 0);
        let frame = e.mem.alloc(0x400);
        let log = Rc::clone(&calls);
        e.register_double(0x00d1_ca00, move |e, a| {
            log.borrow_mut().push((0x00d1_ca00, a.to_vec()));
            e.mem.set_u32(frame + 0x70, 0xabc);
            Ret::default()
        });
        let ray = e.mem.alloc(0x30);
        let table = nodes(&mut e);
        let records = e.mem.alloc(0x80);
        let values = words(&mut e, &[0x1111]);
        let list = words(&mut e, &[0]);
        let counts = words(&mut e, &[0, 0]);
        let args = twenty(&[
            (0, 0xa0),
            (1, ray),
            (2, 0xa2),
            (3, table),
            (4, frame),
            (5, e.mem.alloc(0x200)),
            (6, records),
            (7, 0x77),
            (8, values),
            (9, list),
            (10, 1),
            (15, counts),
            (17, counts + 4),
        ]);
        e.call(0x00e2_bbd0, &args);
        assert_eq!(e.mem.u32(values), 0xabc);
        let log = calls.borrow();
        let addresses: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(addresses, vec![0x00e2_baa0, 0x00e2_baf0, 0x00d1_ca00]);
        assert_eq!(log[0].1, vec![frame, records, 0x77]);
        assert_eq!(
            log[1].1[1..],
            [
                table,
                table + 0x10,
                table,
                table + 0x10,
                table,
                table + 0x10,
                table,
                table + 0x10,
                0x4444
            ]
        );
        assert_eq!(log[2].1[..3], [0xa0, ray, 0xa2]);
    }

    /// Quantized variant: the ray block holds 16-bit coordinates (shifted up by 15 and turned
    /// into floats, then scaled by `[arg0 + 0x70]` less `[arg0 + 0x40]`), the flag is bit 0x8000
    /// of the word at `+0x10`, and the collector's method sits at `+0xc`. The same ray and
    /// nodes as above give the same result.
    #[test]
    fn fn_00e2c100_tests_the_listed_nodes_against_the_quantized_ray() {
        let mut e = engine();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        logging(&mut e, &calls, 0x00e2_baa0, 0);
        logging(&mut e, &calls, VIRTUAL, 0);
        let transform = e.mem.alloc(0x80);
        put_floats(&mut e, transform + 0x70, &[f32::from_bits(0x3800_0000); 4]);
        let ray = e.mem.alloc(0x30);
        for (offset, value) in [(0u32, 0u16), (2, 0), (4, 0), (6, 1), (8, 1), (0xa, 1)] {
            e.mem.set_u16(ray + offset, value);
        }
        e.mem.set_u16(ray + 0x10, 0x8000);
        let table = nodes(&mut e);
        let hits = collector(&mut e, 0xc);
        let list = words(&mut e, &[0, 1]);
        let (count_a, count_b) = (words(&mut e, &[99]), words(&mut e, &[99]));
        let args = twenty(&[
            (0, transform),
            (1, ray),
            (3, table),
            (4, hits),
            (6, 0x5000),
            (7, 0x77),
            (9, list),
            (10, 2),
            (15, count_a),
            (17, count_b),
        ]);
        e.call(0x00e2_c100, &args);
        assert_eq!(e.mem.u32(count_a), 0);
        assert_eq!(e.mem.u32(count_b), 0);
        assert_eq!(
            *calls.borrow(),
            vec![
                (0x00e2_baa0, vec![hits, 0x5000, 0x77]),
                (VIRTUAL, vec![hits + 0xc, ray]),
            ]
        );
    }

    // ---- the drivers -----------------------------------------------------------------------

    /// A router whose first word is the stack allocator's cursor (into a 0x2000-byte arena) and
    /// whose `+0x10` is an allocator whose slot 0x10 logs. Returns (router, arena).
    fn router(e: &mut Engine, calls: &Calls) -> (u32, u32) {
        let arena = e.mem.alloc(0x2000);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, arena);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        logging(e, calls, 0x0900_0010, 0);
        logging(e, calls, 0x00e4_e3f0, 0);
        (router, arena)
    }

    /// The driver for the 0x40-byte nodes: a null object does nothing; otherwise two blocks are
    /// taken from the stack allocator, the root record is copied, `00e2bbd0` runs once with the
    /// driver's arguments (both counts it leaves are zero, so nothing is followed) and the
    /// allocator's cursor is restored.
    #[test]
    fn fn_00e2c8a0_runs_the_leaf_step_over_the_root_record() {
        let mut e = engine();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let (router, arena) = router(&mut e, &calls);
        let object = e.mem.alloc(0x40);
        let root = e.mem.alloc(0x80);
        e.mem.set_u32(object + 0x10, root);
        let log = Rc::clone(&calls);
        e.register_double(0x00e2_bbd0, move |e, a| {
            log.borrow_mut().push((0x00e2_bbd0, a.to_vec()));
            e.mem.set_u32(a[15], 0);
            e.mem.set_u32(a[17], 0);
            Ret::default()
        });
        e.call(
            0x00e2_c8a0,
            &args![0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 3u32],
        );
        assert!(calls.borrow().is_empty());
        e.call(
            0x00e2_c8a0,
            &args![object, 0x11u32, 0x12u32, 0x13u32, 0x14u32, 0x15u32, 3u32],
        );
        let log = calls.borrow();
        let leaf: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00e2_bbd0).collect();
        assert_eq!(leaf.len(), 1);
        let words = &leaf[0].1;
        assert_eq!(words.len(), 20);
        assert_eq!(words[0], object);
        assert_eq!(words[3], 0x11);
        assert_eq!(words[6], 0x12);
        assert_eq!(words[7], 0x13);
        assert_eq!(words[4], 0x14);
        assert_eq!(words[5], 0x15);
        assert_eq!(e.mem.u32(router), arena);
        assert!(log.iter().all(|(a, _)| *a != 0x0900_0010));
    }

    /// The driver for the 0x20-byte nodes does the same with `00e2c100`.
    #[test]
    fn fn_00e2cc30_runs_the_leaf_step_over_the_root_record() {
        let mut e = engine();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let (router, arena) = router(&mut e, &calls);
        let object = e.mem.alloc(0x40);
        let root = e.mem.alloc(0x80);
        e.mem.set_u32(object + 0x10, root);
        let log = Rc::clone(&calls);
        e.register_double(0x00e2_c100, move |e, a| {
            log.borrow_mut().push((0x00e2_c100, a.to_vec()));
            e.mem.set_u32(a[15], 0);
            e.mem.set_u32(a[17], 0);
            Ret::default()
        });
        e.call(
            0x00e2_cc30,
            &args![0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 3u32],
        );
        assert!(calls.borrow().is_empty());
        e.call(
            0x00e2_cc30,
            &args![object, 0x11u32, 0x12u32, 0x13u32, 0x14u32, 0x15u32, 3u32],
        );
        let log = calls.borrow();
        let leaf: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00e2_c100).collect();
        assert_eq!(leaf.len(), 1);
        let words = &leaf[0].1;
        assert_eq!(words.len(), 20);
        assert_eq!(words[0], object);
        assert_eq!(words[3], 0x11);
        assert_eq!(e.mem.u32(router), arena);
    }

    // ---- 00e2d6c0: the closest pair of faces --------------------------------------------

    /// The constants the routine loads, and a square-root helper for `00ec6040` (the value in
    /// `ST0` arrives as two words and the root comes back in `ST0`).
    fn closest_engine() -> Engine {
        let mut e = engine();
        e.set_global(0x0104_da48, 0x7f7f_ffeeu32);
        e.set_global(0x0104_da4c, 0x3400_0000u32);
        e.set_global(0x0102_295c, 0xbf00_0000u32);
        e.set_global(0x0101_6248, 0x3f00_0000u32);
        e.set_global(0x0101_7718, 0x4040_0000u32);
        e.register_double(0x00ec_6040, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        e
    }

    /// Polytope A holds one face (normal `na`, w `wa`, distance `da`), polytope B two (only the
    /// second is examined: the loop stops before index 0). Runs the routine and returns the
    /// output vector and the two face pointers it chose.
    fn closest(
        na: [f32; 3],
        wa: f32,
        da: f32,
        nb: [f32; 3],
        wb: f32,
        db: f32,
    ) -> ([f32; 4], bool, bool) {
        let mut e = closest_engine();
        let a = e.mem.alloc(0x1100);
        let b = e.mem.alloc(0x1100);
        e.mem.set_u32(a + 0x10, 1);
        e.mem.set_u32(b + 0x10, 2);
        put_floats(&mut e, a + 0xf70, &[na[0], na[1], na[2], wa]);
        e.mem.set_f32(a + 0xfb0, da);
        let second = b + 0xf70 + 0x50;
        put_floats(&mut e, second, &[nb[0], nb[1], nb[2], wb]);
        e.mem.set_f32(second + 0x40, db);
        let out = e.mem.alloc(0x10);
        let (face_a, face_b) = (words(&mut e, &[0]), words(&mut e, &[0]));
        e.call(0x00e2_d6c0, &args![a, b, out, face_a, face_b]);
        let result = std::array::from_fn(|lane| e.mem.f32(out + 4 * lane as u32));
        (
            result,
            e.mem.u32(face_a) == a + 0xf70,
            e.mem.u32(face_b) == second,
        )
    }

    fn near(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() <= 1e-6
    }

    /// Parallel faces whose distances differ: the second face's distance is below the first's,
    /// so the nearest point is on face A alone (weights 1 and 0): the output is A's normal
    /// (normalised; the `RSQRTSS` estimate is within an ULP), its w, and the root of the
    /// squared distance, 1.
    #[test]
    fn fn_00e2d6c0_chooses_the_face_pair_and_normalises_the_point() {
        let (out, chose_a, chose_b) =
            closest([1.0, 0.0, 0.0], 0.5, 1.0, [1.0, 0.0, 0.0], 0.25, 0.5);
        assert!(chose_a && chose_b);
        assert!(
            near(out[0], 1.0) && near(out[1], 0.0) && near(out[2], 0.0),
            "{out:?}"
        );
        assert!(near(out[3], 1.0), "{out:?}");
    }

    /// Perpendicular faces at equal distance take the general branch (both weights 1): the point
    /// is the sum of the two normals, normalised; the squared distance is 2.
    #[test]
    fn fn_00e2d6c0_general_branch_for_perpendicular_faces() {
        let (out, chose_a, chose_b) =
            closest([1.0, 0.0, 0.0], 0.5, 1.0, [0.0, 1.0, 0.0], 0.25, 1.0);
        assert!(chose_a && chose_b);
        let half_root = std::f32::consts::FRAC_1_SQRT_2;
        assert!(
            near(out[0], half_root) && near(out[1], half_root) && near(out[2], 0.0),
            "{out:?}"
        );
        assert!(near(out[3], 2.0f32.sqrt()), "{out:?}");
    }

    /// A face of A that is too far (its squared distance exceeds the start limit's) or faces
    /// that point away from each other (dot below -0.5) are skipped: no pair is chosen.
    #[test]
    fn fn_00e2d6c0_skips_faces_pointing_away() {
        let mut e = closest_engine();
        let a = e.mem.alloc(0x1100);
        let b = e.mem.alloc(0x1100);
        e.mem.set_u32(a + 0x10, 1);
        e.mem.set_u32(b + 0x10, 2);
        put_floats(&mut e, a + 0xf70, &[1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(a + 0xfb0, 1.0);
        put_floats(&mut e, b + 0xf70 + 0x50, &[-1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(b + 0xf70 + 0x50 + 0x40, 1.0);
        let out = e.mem.alloc(0x10);
        let (face_a, face_b) = (words(&mut e, &[a]), words(&mut e, &[b]));
        // The routine reads the faces it chose last, here the pointers it was handed.
        let a_face = e.mem.alloc(0x50);
        let b_face = e.mem.alloc(0x50);
        e.mem.set_u32(face_a, a_face);
        e.mem.set_u32(face_b, b_face);
        e.call(0x00e2_d6c0, &args![a, b, out, face_a, face_b]);
        assert_eq!(e.mem.u32(face_a), a_face);
        assert_eq!(e.mem.u32(face_b), b_face);
    }
}
