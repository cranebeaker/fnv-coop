//! `Havok SDK/hkpboxboxcollisiondetection.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpBoxBoxCollisionDetection` (the Xbox PDB has no record of the Havok classes, so the field
//! names below are unconfirmed). Two layouts are used. The manifold (`hkpBoxBoxManifold`, see
//! `hkpboxboxmanifold.rs`) has entries of 4 bytes at `+0` (a feature code, a side code and a
//! word), a byte at `+0x20`, the entry count at `+0x21` and the completeness byte at `+0x22`.
//! The detection state is the query record `hkpboxboxagent.rs` builds: the two bodies at `+0` and
//! `+4`, the object whose virtual function `+0xc` creates a contact point at `+0xc`, a pointer to
//! the cursor over the contact point records at `+0x10`, the two body transforms at `+0x14` and
//! `+0x18`, the rotation between the boxes as three vectors at `+0x20`, `+0x30`, `+0x40` (the
//! translation at `+0x50`), the `radius + half extents` of the first box at `+0x60` and of the
//! second at `+0x70`, the tolerance at `+0x80`, `tolerance + extents` at `+0x90` and `+0xa0`
//! and the tolerance at `+0xb0`. `00d40940` fills the translation in the first box's frame at
//! `+0xc0` and in the second's at `+0xd0`; `checkIntersection` computes the separations it finds
//! at `+0xe0`, `+0xf0`, `+0x100`, `+0x110` and `+0x120`. The SSE code computes lane by lane in
//! `f32`; the x87 code is walked instruction by instruction on a model of the register stack in
//! double precision, rounding at every `float` store.

use super::hkpboxmotion::{add, cross, mul, splat, sub};
use super::hkpsphereshape::{
    inverse_length, load_vec, movhlps, movlhps, shufps, store_vec, sum3, unpckhps, unpcklps, Vec4,
    SIGN_MASK, ZERO_F32,
};
#[allow(unused_imports)]
use crate::prelude::*;

/// The 16-byte mask `0x7fffffff` in all four lanes (`ANDPS` takes absolute values).
const ABS_MASK: u32 = 0x0101_7d10;
/// The `double` 1.1920929e-07 (`FLT_EPSILON`) added before a square root.
const EPSILON: u32 = 0x010c_9300;
/// The `float` 999.99994 the inverse lengths are compared with and the `float` (about -1.0e17)
/// `checkIntersection` stores in place of a larger inverse length.
const LENGTH_LIMIT: u32 = 0x010d_5364;
const LENGTH_FILL: u32 = 0x010d_5360;
/// The CRT square root (`00ec6040`, argument in ST0, result in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `hkMatrix3::setTranspose` (engine map; thiscall: `this` the result, one word the matrix).
const SET_TRANSPOSE: u32 = 0x00cd_4ba0;
/// Eight rows of 16 bytes, the sign vectors `(+-1, +-1, +-1, 0)` a feature code picks with its
/// sign bits (`code & 0x70`).
const SIGN_ROWS: u32 = 0x010d_52d0;
/// The `double`s -2.0, 4.0, 0.99999990463 (`0x3fefffffc0000000`), 0.5 and about 0.999
/// (`0x3feff7cee0000000`).
const MINUS_TWO: u32 = 0x0106_d4c8;
const FOUR: u32 = 0x0101_db80;
const NEARLY_ONE: u32 = 0x010d_5370;
const HALF: u32 = 0x0101_1588;
const THRESHOLD_SCALE: u32 = 0x0109_8bf8;
/// The `float`s +1.0e-5 and -1.0e-5 `findClosestPoint` nudges a point by and the large negative
/// `float` (`0xff7fffee`) it writes over a separation it has used up.
const SMALL_POSITIVE: u32 = 0x0107_18c0;
const SMALL_NEGATIVE: u32 = 0x010d_5368;
const USED_UP: u32 = 0x0104_e148;
/// `hkpBoxBoxManifold::removePoint` (`this`, the index), `clear` (`this`), `addPoint` (`this`,
/// two unread words, the key) and `findInManifold` (`this`, the result byte, the key), and
/// `00d40940`, which prepares the query record (`this` only).
const REMOVE_POINT: u32 = 0x00db_0900;
const CLEAR_MANIFOLD: u32 = 0x00db_0920;
const ADD_POINT: u32 = 0x00db_0960;
const FIND_IN_MANIFOLD: u32 = 0x00db_09b0;
const PREPARE_QUERY: u32 = 0x00d4_0940;
/// The functions `calcManifold` calls by address: `004b4ab0` (`__thiscall`, `RET 8`: the result
/// at `this`, a matrix of three rows and a vector), `hkVector4Util::packQuaternionIntoInt32`
/// (`00d975d0`, `cdecl`: a pointer to a vector, returns the packed word) and
/// `hkVector4::setRotatedDir` (`00c7f700`, `RET 8`: the result at `this`, a transform, a vector).
const TRANSFORM_VECTOR: u32 = 0x004b_4ab0;
const PACK_DIRECTION: u32 = 0x00d9_75d0;
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// The `double`s 0.00859375 (the scale of a byte of the packed direction), 0.95 (the dot product
/// a direction must stay above), 0.05 (the share of the tolerance taken off the smallest record
/// distance) and 0.98 (the alignment a neighbour axis needs).
const DIRECTION_SCALE: u32 = 0x010c_e488;
const DIRECTION_LIMIT: u32 = 0x0107_d1b8;
const MINIMUM_SCALE: u32 = 0x0103_c7c8;
const ALIGNED_LIMIT: u32 = 0x010d_5378;
/// The seven `word`s `0x50, 0x30, 0x60, 0x10, 0x20, 0x40, 0x00` (from `010d5350` to `010d535e`):
/// the masks the neighbour loops of `calcManifold` change a feature code by.
const NEIGHBOUR_MASKS: u32 = 0x010d_5350;
const NEIGHBOUR_MASKS_END: u32 = 0x010d_535e;

/// The x87 condition bits `FNSTSW` leaves after a compare of `a` with `b`: `c0` when `a < b`,
/// `c3` when equal, all three when unordered.
#[derive(Clone, Copy)]
struct Compare {
    c0: bool,
    c2: bool,
    c3: bool,
}

fn compare(a: f64, b: f64) -> Compare {
    if a.is_nan() || b.is_nan() {
        Compare {
            c0: true,
            c2: true,
            c3: true,
        }
    } else {
        Compare {
            c0: a < b,
            c2: false,
            c3: a == b,
        }
    }
}

/// The x87 register stack: the last element is `ST0`. Values live in double precision (the
/// x87 control word of the game keeps 53 bits); a `float` load widens, a `float` store rounds.
struct X87 {
    stack: Vec<f64>,
}

impl X87 {
    fn new() -> X87 {
        X87 { stack: Vec::new() }
    }

    fn top(&self, depth: usize) -> usize {
        self.stack.len() - 1 - depth
    }

    fn pop(&mut self) -> f64 {
        self.stack.pop().expect("x87 stack")
    }

    /// `FLD dword ptr [addr]` (a signalling NaN is quieted).
    fn fld(&mut self, e: &Engine, addr: u32) {
        self.fld_value(e.mem.f32(addr));
    }

    /// `FLD` of a `float` that was passed on the stack.
    fn fld_value(&mut self, value: f32) {
        self.stack.push(f64::from(x87_quiet(value)));
    }

    /// `FLD qword ptr [addr]`.
    fn fld64(&mut self, e: &Engine, addr: u32) {
        self.stack.push(e.global::<f64>(addr));
    }

    /// `FLD ST(depth)`.
    fn fld_st(&mut self, depth: usize) {
        let value = self.stack[self.top(depth)];
        self.stack.push(value);
    }

    fn fld1(&mut self) {
        self.stack.push(1.0);
    }

    fn fldz(&mut self) {
        self.stack.push(0.0);
    }

    /// `FMUL dword ptr [addr]`.
    fn fmul(&mut self, e: &Engine, addr: u32) {
        let index = self.top(0);
        self.stack[index] *= f64::from(x87_quiet(e.mem.f32(addr)));
    }

    /// `FMUL qword ptr [addr]`.
    fn fmul64(&mut self, e: &Engine, addr: u32) {
        let index = self.top(0);
        self.stack[index] *= e.global::<f64>(addr);
    }

    /// `FMUL ST0` (`ST0 * ST0`).
    fn fmul_self(&mut self) {
        let index = self.top(0);
        self.stack[index] *= self.stack[index];
    }

    /// `FMUL ST0, ST(depth)`.
    fn fmul_st0(&mut self, depth: usize) {
        let index = self.top(0);
        self.stack[index] *= self.stack[self.top(depth)];
    }

    /// `FADD ST0, ST0`.
    fn fadd_self(&mut self) {
        let index = self.top(0);
        self.stack[index] += self.stack[index];
    }

    /// `FADD ST0, ST(depth)`.
    fn fadd_st0(&mut self, depth: usize) {
        let index = self.top(0);
        self.stack[index] += self.stack[self.top(depth)];
    }

    /// `FADD dword ptr [addr]`.
    fn fadd(&mut self, e: &Engine, addr: u32) {
        let index = self.top(0);
        self.stack[index] += f64::from(x87_quiet(e.mem.f32(addr)));
    }

    /// `FADD qword ptr [addr]`.
    fn fadd64(&mut self, e: &Engine, addr: u32) {
        let index = self.top(0);
        self.stack[index] += e.global::<f64>(addr);
    }

    /// `FSUB dword ptr [addr]`.
    fn fsub(&mut self, e: &Engine, addr: u32) {
        let index = self.top(0);
        self.stack[index] -= f64::from(x87_quiet(e.mem.f32(addr)));
    }

    /// `FADDP ST(depth), ST0`.
    fn faddp(&mut self, depth: usize) {
        let value = self.pop();
        let index = self.stack.len() - depth;
        self.stack[index] += value;
    }

    /// `FMULP ST(depth), ST0`.
    fn fmulp(&mut self, depth: usize) {
        let value = self.pop();
        let index = self.stack.len() - depth;
        self.stack[index] *= value;
    }

    /// `FSUBP ST(depth), ST0`: `ST(depth) = ST(depth) - ST0`, then pop.
    fn fsubp(&mut self, depth: usize) {
        let value = self.pop();
        let index = self.stack.len() - depth;
        self.stack[index] -= value;
    }

    /// `FSUBRP ST(depth), ST0`: `ST(depth) = ST0 - ST(depth)`, then pop.
    fn fsubrp(&mut self, depth: usize) {
        let value = self.pop();
        let index = self.stack.len() - depth;
        self.stack[index] = value - self.stack[index];
    }

    /// `FDIVRP ST(depth), ST0`: `ST(depth) = ST0 / ST(depth)`, then pop.
    fn fdivrp(&mut self, depth: usize) {
        let value = self.pop();
        let index = self.stack.len() - depth;
        self.stack[index] = value / self.stack[index];
    }

    fn fabs(&mut self) {
        let index = self.top(0);
        self.stack[index] = self.stack[index].abs();
    }

    fn fchs(&mut self) {
        let index = self.top(0);
        self.stack[index] = -self.stack[index];
    }

    /// `FXCH ST(depth)`.
    fn fxch(&mut self, depth: usize) {
        let (a, b) = (self.top(0), self.top(depth));
        self.stack.swap(a, b);
    }

    /// `FSTP dword ptr [addr]`.
    fn fstp(&mut self, e: &mut Engine, addr: u32) {
        let value = self.pop();
        e.mem.set_f32(addr, value as f32);
    }

    /// `FSTP ST(depth)`: `ST(depth) = ST0`, then pop.
    fn fstp_st(&mut self, depth: usize) {
        let value = self.pop();
        let index = self.stack.len() - depth;
        self.stack[index] = value;
    }

    /// `FCOMP ST(depth)`: compares `ST0` with `ST(depth)`, then pops.
    fn fcomp(&mut self, depth: usize) -> Compare {
        let result = compare(self.stack[self.top(0)], self.stack[self.top(depth)]);
        self.pop();
        result
    }

    /// `FCOMPP`: compares `ST0` with `ST1`, then pops both.
    fn fcompp(&mut self) -> Compare {
        let result = self.fcomp(1);
        self.pop();
        result
    }

    /// `FCOMP dword ptr [addr]`.
    fn fcomp32(&mut self, e: &Engine, addr: u32) -> Compare {
        let result = compare(self.stack[self.top(0)], f64::from(e.mem.f32(addr)));
        self.pop();
        result
    }

    /// `FILD dword ptr`.
    fn fild(&mut self, value: i32) {
        self.stack.push(f64::from(value));
    }

    /// `FIMUL dword ptr`.
    fn fimul(&mut self, value: i32) {
        let index = self.top(0);
        self.stack[index] *= f64::from(value);
    }

    /// `FMUL ST(depth), ST0`: `ST(depth) = ST(depth) * ST0`.
    fn fmul_into(&mut self, depth: usize) {
        let value = self.stack[self.top(0)];
        let index = self.top(depth);
        self.stack[index] *= value;
    }

    /// `FSUB ST(depth), ST0`: `ST(depth) = ST(depth) - ST0`.
    fn fsub_into(&mut self, depth: usize) {
        let value = self.stack[self.top(0)];
        let index = self.top(depth);
        self.stack[index] -= value;
    }

    /// `FST dword ptr [addr]`: stores `ST0` without popping it.
    fn fst(&self, e: &mut Engine, addr: u32) {
        e.mem.set_f32(addr, self.stack[self.top(0)] as f32);
    }

    /// `FCOM ST(depth)`: compares `ST0` with `ST(depth)`.
    fn fcom(&self, depth: usize) -> Compare {
        compare(self.stack[self.top(0)], self.stack[self.top(depth)])
    }

    /// `FCOMP qword ptr [addr]`.
    fn fcomp64(&mut self, e: &Engine, addr: u32) -> Compare {
        let result = compare(self.stack[self.top(0)], e.global::<f64>(addr));
        self.pop();
        result
    }

    /// `CALL 00ec6040`: the square root of `ST0`, left in `ST0`.
    fn sqrt(&mut self, e: &mut Engine) {
        let value = self.pop();
        self.stack.push(e.call(X87_SQRT, &args![value]).f64());
    }
}

/// `MOVSS xmm, dword ptr [addr]`: the word in the first lane, the other lanes 0.
fn movss(e: &Engine, addr: u32) -> Vec4 {
    [f32::from_bits(e.mem.u32(addr)), 0.0, 0.0, 0.0]
}

fn lane_bits(v: Vec4) -> [u32; 4] {
    v.map(f32::to_bits)
}

/// `ANDPS a, b`.
fn andps(a: Vec4, b: Vec4) -> Vec4 {
    let (a, b) = (lane_bits(a), lane_bits(b));
    std::array::from_fn(|lane| f32::from_bits(a[lane] & b[lane]))
}

/// `ANDNPS a, b`: `!a & b`.
fn andnps(a: Vec4, b: Vec4) -> Vec4 {
    let (a, b) = (lane_bits(a), lane_bits(b));
    std::array::from_fn(|lane| f32::from_bits(!a[lane] & b[lane]))
}

/// `ORPS a, b`.
fn orps(a: Vec4, b: Vec4) -> Vec4 {
    let (a, b) = (lane_bits(a), lane_bits(b));
    std::array::from_fn(|lane| f32::from_bits(a[lane] | b[lane]))
}

/// `XORPS a, b`.
fn xorps(a: Vec4, b: Vec4) -> Vec4 {
    let (a, b) = (lane_bits(a), lane_bits(b));
    std::array::from_fn(|lane| f32::from_bits(a[lane] ^ b[lane]))
}

/// `CMPLTPS a, b`: all ones in a lane where `a < b` (false for NaN).
fn cmpltps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(if a[lane] < b[lane] { u32::MAX } else { 0 }))
}

/// `MOVMSKPS`: the four sign bits.
fn movmskps(v: Vec4) -> u32 {
    lane_bits(v)
        .iter()
        .enumerate()
        .fold(0, |mask, (lane, bits)| mask | (bits >> 31) << lane)
}

/// The point `(v.y * m1 + v.x * m0) + (v.z * m2 + m3)` for the four rows of the transform at
/// `matrix`.
fn transform_point(e: &Engine, matrix: u32, v: Vec4) -> Vec4 {
    let row = |index: u32| load_vec(e, matrix + 0x10 * index);
    let low = add(mul(splat(v, 1), row(1)), mul(splat(v, 0), row(0)));
    add(low, add(mul(splat(v, 2), row(2)), row(3)))
}

/// The vector `(v.y * m1 + v.x * m0) + v.z * m2` (the transform without its translation row).
fn transform_direction(e: &Engine, matrix: u32, v: Vec4) -> Vec4 {
    let row = |index: u32| load_vec(e, matrix + 0x10 * index);
    let low = add(mul(splat(v, 1), row(1)), mul(splat(v, 0), row(0)));
    add(low, mul(splat(v, 2), row(2)))
}

/// `v` times the matrix whose rows are the three vectors at `matrix`, the `UNPCK`/`MOVLHPS`/
/// `SHUFPS` chain: `(v.y * column_y + v.x * column_x) + v.z * column_z`.
fn rows_times_columns(e: &Engine, matrix: u32, v: Vec4) -> Vec4 {
    let (r0, r1, r2) = (
        load_vec(e, matrix),
        load_vec(e, matrix + 0x10),
        load_vec(e, matrix + 0x20),
    );
    let low = unpcklps(r0, r1);
    let high = unpckhps(r0, r1);
    let column_x = movlhps(low, r2);
    let column_y = shufps(movhlps(column_x, low), r2, 0xd4);
    let column_z = shufps(high, r2, 0xe4);
    let part_y = mul(splat(v, 1), column_y);
    let part_x = mul(splat(v, 0), column_x);
    let part_z = mul(column_z, splat(v, 2));
    add(add(part_y, part_x), part_z)
}

/// The feature byte of a side: the sign bits of lanes 0 to 2 in bits 4 to 6 and, in bit 3, the
/// sign bit of `flags` (the float `+-1.0` that says which side of the box).
fn feature_byte(mask: u32, flags: u32) -> u8 {
    (((mask & 7) << 4) | ((flags >> 0x1c) & 8)) as u8
}

/// Whether the manifold, scanned from its last entry down, has an entry whose first two bytes
/// are `first` and `second`.
fn has_entry(e: &Engine, manifold: u32, first: u8, second: u8) -> bool {
    let mut index = i32::from(e.mem.u8(manifold + 0x21)) - 1;
    while index >= 0 {
        let entry = manifold + index as u32 * 4;
        if e.mem.u8(entry) == first && e.mem.u8(entry + 1) == second {
            return true;
        }
        index -= 1;
    }
    false
}

/// Adds the pair-counting bit of `shift` to `mask`: a 2-bit counter at bit `2 * shift` that
/// stops growing once its high bit is set.
fn count_pair(mask: &mut u32, shift: u32) {
    let bit = 1u32 << (shift * 2);
    if *mask & (bit * 2) == 0 {
        *mask += bit;
    }
}

/// The three counters a face code (the high nibble of `code`: one bit per axis, 1 meaning
/// "negative side") adds to `mask`, one for each axis except `skip`.
fn count_face(mask: &mut u32, code: u8, skip: u32) {
    let nibble = u32::from(code) >> 4;
    let (first, second, third) = (!nibble & 1, !(nibble >> 1) & 1, !(nibble >> 2) & 1);
    if skip != 0 {
        count_pair(mask, second + third * 2);
    }
    if skip != 1 {
        count_pair(mask, first + 4 + third * 2);
    }
    if skip != 2 {
        count_pair(mask, first + 8 + second * 2);
    }
}

/// The counter an edge code adds to `mask`: the low nibble of `code` picks the edge axis
/// (0, 1 or any other) and the high nibble the side bits.
fn count_edge(mask: &mut u32, code: u8) {
    let nibble = u32::from(code) >> 4;
    let (first, second, third) = (!nibble & 1, !(nibble >> 1) & 1, !(nibble >> 2) & 1);
    let shift = match code & 0xf {
        0 => second + third * 2,
        1 => first + 4 + third * 2,
        _ => first + 8 + second * 2,
    };
    count_pair(mask, shift);
}

/// The axis (1 or 2, or 0 when bit 4 is set) a feature code names in `checkCompleteness`.
fn feature_axis(code: u32) -> u32 {
    let shifted = (code as i32) >> 4;
    if shifted & 1 != 0 {
        0
    } else if shifted & 2 != 0 {
        1
    } else {
        2
    }
}

// Translated from 00db0b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxCollisionDetection::checkCompleteness` (engine map; `RET 0xc`: the manifold, two
/// feature codes). With fewer than 3 entries (`+0x21`) nothing happens; with 4 or more
/// contact points counted in the byte at `+0x20` the byte at `+0x22` becomes 1. Otherwise it
/// counts, for every entry (4 bytes: a kind and a code), pairs of face and edge bits into two
/// masks: a kind below 3 adds the faces of its code to the second mask (skipping the axis of
/// `second_code`), a kind from 3 to 6 adds them to the first mask (skipping the axis of
/// `first_code`) and a larger kind adds its own edge to the first mask and the edge of its code
/// to the second. The byte at `+0x22` is 1 when neither mask has a bit of `0x555555` set.
pub fn check_completeness(e: &mut Engine, manifold: Ptr, first_code: u32, second_code: u32) {
    let manifold = manifold.addr();
    let count = e.mem.u8(manifold + 0x21);
    if count < 3 {
        return;
    }
    if e.mem.u8(manifold + 0x20) >= 4 {
        e.mem.set_u8(manifold + 0x22, 1);
        return;
    }
    let first_axis = feature_axis(first_code);
    let second_axis = feature_axis(second_code);
    let (mut first_mask, mut second_mask) = (0u32, 0u32);
    for index in 0..u32::from(count) {
        let kind = e.mem.u8(manifold + index * 4);
        let code = e.mem.u8(manifold + index * 4 + 1);
        if kind < 3 {
            count_face(&mut second_mask, code, second_axis);
        } else if kind < 7 {
            count_face(&mut first_mask, code, first_axis);
        } else {
            count_edge(&mut first_mask, kind);
            count_edge(&mut second_mask, code);
        }
    }
    let complete = first_mask & 0x55_5555 == 0 && second_mask & 0x55_5555 == 0;
    e.mem.set_u8(manifold + 0x22, u8::from(complete));
}

// Translated from 00db0dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the two feature bytes of an edge pair (no engine-map name; cdecl). Registers: a
/// vector (EAX) and the two result bytes (EDX); stack words: a second vector and two axis
/// indices. The stack vector gets 1.0 in lane `axis_a`, the register vector 1.0 in lane `axis_b`
/// (the exe does not check the indices; an index above 3 panics here). Result byte 0 is
/// `(sign bits of the stack vector's lanes 0 to 2 | 8) << 4 | axis_a` (a lane has its sign
/// bit set when it is below 0.0; NaN does not) and byte 1 is `(!sign bits of the register
/// vector & 7) << 4` plus `axis_b`.
pub fn fn_00db0dc0(
    e: &mut Engine,
    vector_b: Ptr,
    out: Ptr,
    vector_a: Ptr,
    axis_a: u32,
    axis_b: u32,
) {
    let mut a = load_vec(e, vector_a.addr());
    let mut b = load_vec(e, vector_b.addr());
    a[axis_a as usize] = 1.0;
    let negative_a = movmskps(cmpltps(a, [0.0; 4]));
    b[axis_b as usize] = 1.0;
    let negative_b = movmskps(cmpltps(b, [0.0; 4]));
    let first = (((negative_a as u8) | 0xf8) << 4) | axis_a as u8;
    let second = (((!negative_b) as u8 & 7) << 4).wrapping_add(axis_b as u8);
    e.mem.set_u8(out.addr(), first);
    e.mem.set_u8(out.addr() + 1, second);
}

// Translated from 00db0e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The feature byte of a vector (no engine-map name; cdecl). Register: the vector (EAX); stack
/// word: a flags word. Returns `(sign bits of lanes 0 to 2 (below 0.0) & 7) << 4 |
/// (flags >> 28 & 8)`.
pub fn fn_00db0e50(e: &mut Engine, vector: Ptr, flags: u32) -> u32 {
    let negative = movmskps(cmpltps(load_vec(e, vector.addr()), [0.0; 4]));
    u32::from(feature_byte(negative, flags))
}

// Translated from 00db0ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxCollisionDetection::checkIntersection` (engine map; `__thiscall`, `RET 4`: the
/// separation tolerance, a pointer to four floats). The separating axes of two boxes: the
/// three face axes of each box and the nine edge pairs. For every axis the separation (the
/// distance of the centres minus the extents' projections) is compared with the tolerance
/// (`tolerance < separation` in lanes 0 to 2 means separated: it returns 1 and stops); the
/// separations are stored at `+0xe0` (first box), `+0xf0` (second box) and `+0x100`, `+0x110`,
/// `+0x120` (the edge pairs: each divided by the length of its axis, which a square root with
/// `FLT_EPSILON` added gives; an axis whose inverse length is above 999.99994 stores the float
/// at `010d5360` instead). Returns 0 when no axis separates.
pub fn check_intersection(e: &mut Engine, this: Ptr, tolerance: Ptr) -> u32 {
    let (t, tolerance) = (this.addr(), tolerance.addr());
    e.with_stack(0x110, |e, frame| {
        check_intersection_frame(e, frame.addr(), t, tolerance)
    })
}

/// The body of `check_intersection` with its stack frame at `f` (the offsets are the exe's
/// `ESP + offset`).
fn check_intersection_frame(e: &mut Engine, f: u32, t: u32, tolerance: u32) -> u32 {
    let abs = load_vec(e, ABS_MASK);
    let mut x0 = movss(e, LENGTH_LIMIT);
    let mut x4 = abs;
    let mut x1 = load_vec(e, t + 0x70);
    let mut x3 = load_vec(e, t + 0x30);
    let mut x2 = load_vec(e, t + 0x40);
    store_vec(e, f + 0xa0, x0);
    x0 = movss(e, LENGTH_FILL);
    let mut x5 = splat(x1, 1);
    let mut x6 = splat(x1, 0);
    store_vec(e, f + 0x60, x0);
    x0 = load_vec(e, t + 0x20);
    x0 = andps(x0, x4);
    x3 = andps(x3, x4);
    x6 = mul(x6, x0);
    x5 = mul(x5, x3);
    x5 = add(x5, x6);
    x6 = splat(x1, 2);
    x1 = load_vec(e, t + 0x60);
    x2 = andps(x2, x4);
    x6 = mul(x6, x2);
    x5 = add(x5, x6);
    x5 = add(x5, x1);
    x1 = load_vec(e, t + 0xc0);
    x1 = andps(x1, x4);
    x4 = load_vec(e, tolerance);
    x1 = sub(x1, x5);
    x4 = cmpltps(x4, x1);
    let mask = movmskps(x4);
    store_vec(e, f + 0x30, x0);
    store_vec(e, f + 0x40, x3);
    store_vec(e, f + 0x50, x2);
    if mask & 7 != 0 {
        return 1;
    }
    store_vec(e, t + 0xe0, x1);
    x1 = unpcklps(x0, x3);
    x0 = unpckhps(x0, x3);
    x4 = movlhps(x1, x2);
    x3 = movhlps(x4, x1);
    x1 = load_vec(e, t + 0x60);
    x3 = shufps(x3, x2, 0xd4);
    x5 = splat(x1, 1);
    x5 = mul(x5, x3);
    x3 = splat(x1, 0);
    x3 = mul(x3, x4);
    x5 = add(x5, x3);
    x3 = splat(x1, 2);
    x0 = shufps(x0, x2, 0xe4);
    x1 = load_vec(e, ABS_MASK);
    x3 = mul(x3, x0);
    x0 = load_vec(e, t + 0x70);
    x5 = add(x5, x3);
    x5 = add(x5, x0);
    x0 = load_vec(e, t + 0xd0);
    x0 = andps(x0, x1);
    x1 = load_vec(e, tolerance);
    x0 = sub(x0, x5);
    x1 = cmpltps(x1, x0);
    let mask = movmskps(x1);
    if mask & 7 != 0 {
        return 1;
    }
    store_vec(e, t + 0xf0, x0);
    e.call(SET_TRANSPOSE, &args![f + 0xd0, t + 0x20]);

    // The first group of edge pairs.
    let mut x = X87::new();
    x.fld(e, f + 0x38);
    x.fmul(e, t + 0x64);
    x0 = load_vec(e, f + 0xd0);
    x.fld(e, f + 0x34);
    x3 = load_vec(e, f + 0xe0);
    x.fmul(e, t + 0x68);
    x2 = load_vec(e, f + 0xf0);
    x0 = mul(x0, x0);
    store_vec(e, f + 0xc0, x0);
    x.faddp(1);
    x0 = load_vec(e, t + 0xc0);
    x.fld(e, f + 0x50);
    x0 = splat(x0, 2);
    x.fld_st(0);
    x0 = splat(x0, 0);
    x.fmul(e, t + 0x74);
    x0 = mul(x0, x3);
    store_vec(e, f + 0x70, x0);
    x0 = load_vec(e, t + 0xc0);
    x.faddp(2);
    x0 = splat(x0, 1);
    x.fld(e, f + 0x40);
    x2 = mul(x2, x2);
    x.fld_st(0);
    store_vec(e, f + 0x80, x0);
    x.fmul(e, t + 0x78);
    x1 = x3;
    x1 = mul(x1, x3);
    x0 = x2;
    x.faddp(3);
    x0 = add(x0, x1);
    x.fxch(2);
    store_vec(e, f + 0x10, x0);
    store_vec(e, f + 0x100, x1);
    x.fstp(e, f + 0x2c);
    store_vec(e, f + 0x90, x2);
    x.fld(e, f + 0x48);
    x.fmul(e, t + 0x64);
    x.fld(e, f + 0x44);
    x.fmul(e, t + 0x68);
    x.faddp(1);
    x.fld(e, t + 0x70);
    x.fmulp(2);
    x.faddp(1);
    x.fld(e, f + 0x30);
    x.fld_st(0);
    x.fmul(e, t + 0x78);
    x.faddp(2);
    x.fxch(1);
    x.fstp(e, f + 0x28);
    x.fld(e, f + 0x58);
    x.fmul(e, t + 0x64);
    x.fld(e, f + 0x54);
    x.fmul(e, t + 0x68);
    x.faddp(1);
    x.fld(e, t + 0x70);
    x.fmulp(3);
    x.faddp(2);
    x.fmul(e, t + 0x74);
    x.faddp(1);
    x.fstp(e, f + 0x8);
    normalise_three(e, &mut x, f, 0x10, 0xc);
    x0 = load_vec(e, f + 0x80);
    x.fld1();
    x3 = load_vec(e, f + 0xf0);
    x.fdivrp(1);
    x2 = movss(e, f + 0x28);
    x4 = movss(e, f + 0x8);
    x5 = movss(e, ZERO_F32);
    x1 = splat(x0, 0);
    x0 = load_vec(e, f + 0x70);
    x1 = mul(x1, x3);
    x0 = sub(x0, x1);
    x1 = load_vec(e, ABS_MASK);
    x0 = andps(x0, x1);
    x1 = movss(e, f + 0x2c);
    x2 = unpcklps(x2, x5);
    x1 = unpcklps(x1, x4);
    x1 = unpcklps(x1, x2);
    x0 = sub(x0, x1);
    x.fstp(e, f + 0x18);
    x.fldz();
    x.fstp(e, f + 0x1c);
    x1 = load_vec(e, f + 0x10);
    x0 = mul(x0, x1);
    x2 = x0;
    x0 = load_vec(e, t + 0x80);
    store_vec(e, f + 0xb0, x0);
    x0 = cmpltps(x0, x2);
    let mask = movmskps(x0);
    if mask & 7 != 0 {
        return 1;
    }
    // The second group.
    x.fld(e, f + 0x30);
    x0 = load_vec(e, f + 0xa0);
    x.fmul(e, t + 0x68);
    x0 = splat(x0, 0);
    x.fld(e, f + 0x38);
    store_vec(e, f + 0x70, x0);
    x.fmul(e, t + 0x60);
    x0 = cmpltps(x0, x1);
    x1 = load_vec(e, f + 0x60);
    x4 = x0;
    x.faddp(1);
    x0 = andnps(x0, x2);
    x.fld(e, f + 0x54);
    x1 = splat(x1, 0);
    x.fld_st(0);
    x4 = andps(x4, x1);
    x.fmul(e, t + 0x74);
    x4 = orps(x4, x0);
    store_vec(e, t + 0x100, x4);
    x0 = load_vec(e, t + 0xc0);
    x.faddp(2);
    x0 = splat(x0, 0);
    x.fld(e, f + 0x44);
    x0 = mul(x0, x3);
    x.fld_st(0);
    store_vec(e, f + 0xa0, x0);
    x.fmul(e, t + 0x78);
    x0 = load_vec(e, t + 0xc0);
    x0 = splat(x0, 2);
    store_vec(e, f + 0x60, x0);
    x.faddp(3);
    x0 = load_vec(e, f + 0x90);
    x.fxch(2);
    x0 = add(x0, load_vec(e, f + 0xc0));
    store_vec(e, f + 0x10, x0);
    x.fstp(e, f + 0x28);
    store_vec(e, f + 0x80, x1);
    x.fld(e, f + 0x48);
    x.fmul(e, t + 0x60);
    x.fld(e, f + 0x40);
    x.fmul(e, t + 0x68);
    x.faddp(1);
    x.fld(e, t + 0x70);
    x.fmulp(2);
    x.faddp(1);
    x.fld(e, f + 0x34);
    x.fld_st(0);
    x.fmul(e, t + 0x78);
    x.faddp(2);
    x.fxch(1);
    x.fstp(e, f + 0x2c);
    x.fld(e, f + 0x58);
    x.fmul(e, t + 0x60);
    x.fld(e, f + 0x50);
    x.fmul(e, t + 0x68);
    x.faddp(1);
    x.fld(e, t + 0x70);
    x.fmulp(3);
    x.faddp(2);
    x.fmul(e, t + 0x74);
    x.faddp(1);
    x.fstp(e, f + 0xc);
    normalise_three(e, &mut x, f, 0x10, 0x8);
    x0 = load_vec(e, f + 0x60);
    x.fld1();
    x3 = load_vec(e, f + 0xd0);
    x.fdivrp(1);
    x2 = movss(e, f + 0x2c);
    x4 = movss(e, f + 0xc);
    x5 = movss(e, ZERO_F32);
    x1 = splat(x0, 0);
    x0 = load_vec(e, f + 0xa0);
    x1 = mul(x1, x3);
    x0 = sub(x0, x1);
    x1 = load_vec(e, ABS_MASK);
    x0 = andps(x0, x1);
    x1 = movss(e, f + 0x28);
    x2 = unpcklps(x2, x5);
    x1 = unpcklps(x1, x4);
    x1 = unpcklps(x1, x2);
    x0 = sub(x0, x1);
    x.fstp(e, f + 0x18);
    x.fldz();
    x.fstp(e, f + 0x1c);
    x2 = load_vec(e, f + 0x10);
    x0 = mul(x0, x2);
    x1 = x0;
    x0 = load_vec(e, f + 0xb0);
    x0 = cmpltps(x0, x1);
    let mask = movmskps(x0);
    if mask & 7 != 0 {
        return 1;
    }
    x.fld(e, f + 0x30);
    x0 = load_vec(e, f + 0x70);
    x.fmul(e, t + 0x64);
    x0 = cmpltps(x0, x2);
    x.fld(e, f + 0x34);
    x2 = x0;
    x.fmul(e, t + 0x60);
    x2 = andps(x2, load_vec(e, f + 0x80));
    x0 = andnps(x0, x1);
    x2 = orps(x2, x0);
    x.faddp(1);
    store_vec(e, t + 0x110, x2);
    x.fld(e, f + 0x58);
    x0 = load_vec(e, t + 0xc0);
    x.fld_st(0);
    x0 = splat(x0, 1);
    x.fmul(e, t + 0x74);
    x0 = splat(x0, 0);
    x0 = mul(x0, x3);
    store_vec(e, f + 0x60, x0);
    x.faddp(2);
    x0 = load_vec(e, t + 0xc0);
    x.fld(e, f + 0x48);
    store_vec(e, f + 0x90, x0);
    x.fld_st(0);
    x0 = load_vec(e, f + 0xc0);
    x.fmul(e, t + 0x78);
    x0 = add(x0, load_vec(e, f + 0x100));
    store_vec(e, f + 0x10, x0);
    x.faddp(3);
    x.fxch(2);
    x.fstp(e, f + 0x28);
    x.fld(e, f + 0x44);
    x.fmul(e, t + 0x60);
    x.fld(e, f + 0x40);
    x.fmul(e, t + 0x64);
    x.faddp(1);
    x.fld(e, f + 0x38);
    x.fld_st(0);
    x.fmul(e, t + 0x78);
    x.faddp(2);
    x.fld(e, t + 0x70);
    x.fmulp(3);
    x.fxch(1);
    x.faddp(2);
    x.fxch(1);
    x.fstp(e, f + 0x2c);
    x.fld(e, f + 0x54);
    x.fmul(e, t + 0x60);
    x.fld(e, f + 0x50);
    x.fmul(e, t + 0x64);
    x.faddp(1);
    x.fld(e, t + 0x70);
    x.fmulp(3);
    x.faddp(2);
    x.fmul(e, t + 0x74);
    x.faddp(1);
    x.fstp(e, f + 0xc);
    normalise_three(e, &mut x, f, 0x10, 0x8);
    x0 = load_vec(e, f + 0x90);
    x.fld1();
    x2 = movss(e, f + 0x2c);
    x.fdivrp(1);
    x3 = movss(e, f + 0xc);
    x4 = movss(e, ZERO_F32);
    x1 = splat(x0, 0);
    x1 = mul(x1, load_vec(e, f + 0xe0));
    x0 = load_vec(e, f + 0x60);
    x0 = sub(x0, x1);
    x1 = load_vec(e, ABS_MASK);
    x0 = andps(x0, x1);
    x1 = movss(e, f + 0x28);
    x2 = unpcklps(x2, x4);
    x1 = unpcklps(x1, x3);
    x1 = unpcklps(x1, x2);
    x0 = sub(x0, x1);
    x1 = load_vec(e, f + 0xb0);
    x.fstp(e, f + 0x18);
    x.fldz();
    x.fstp(e, f + 0x1c);
    x2 = load_vec(e, f + 0x10);
    x0 = mul(x0, x2);
    x1 = cmpltps(x1, x0);
    let mask = movmskps(x1);
    if mask & 7 != 0 {
        return 1;
    }
    x1 = load_vec(e, f + 0x70);
    x1 = cmpltps(x1, x2);
    x2 = x1;
    x1 = andps(x1, load_vec(e, f + 0x80));
    x2 = andnps(x2, x0);
    x2 = orps(x2, x1);
    store_vec(e, t + 0x120, x2);
    0
}

/// Three times `sqrt(frame[first + 4 i] + FLT_EPSILON)` for the lane `i` of the vector at
/// `f + first`, stored back as `1 / sqrt` at the same place; the third inverse stays on the
/// x87 stack. `spill` is the frame slot the exe uses for the intermediate `float`.
fn normalise_three(e: &mut Engine, x: &mut X87, f: u32, first: u32, spill: u32) {
    for lane in 0..2 {
        x.fld(e, f + first + 4 * lane);
        x.fadd64(e, EPSILON);
        x.fstp(e, f + spill);
        x.fld(e, f + spill);
        x.sqrt(e);
        x.fstp(e, f + spill);
        x.fld(e, f + spill);
        x.fld1();
        x.fdivrp(1);
        x.fstp(e, f + first + 4 * lane);
    }
    x.fld(e, f + first + 8);
    x.fadd64(e, EPSILON);
    x.fstp(e, f + spill);
    x.fld(e, f + spill);
    x.sqrt(e);
    x.fstp(e, f + spill);
    x.fld(e, f + spill);
}

// Translated from 00db15a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxCollisionDetection::isValidEdgeEdge` (engine map; `__thiscall`, `RET 8`: a result
/// byte, a record). The record holds two feature words (`+0x38`, `+0x3c`: the low nibble an
/// axis, `& 0x70` a row of the sign table). It finds where the two edges are closest (a 2x2
/// system solved with the determinant `D`; the edge pair is rejected when a parameter leaves
/// its range or `|D|` is too small), writes the contact point (`+0`), the distance (`+0x34`)
/// and the negated normal (`+0x20`) into the record and sets the byte to 1; every rejection
/// sets it to 0 and leaves the record alone.
/// Answers `valid` (`EAX` at every `RET`; callers test the flag through it, 00db19ed).
pub fn is_valid_edge_edge(e: &mut Engine, this: Ptr, valid: Ptr, info: Ptr) -> Ptr {
    let (t, info) = (this.addr(), info.addr());
    let ok = e.with_stack(0x60, |e, frame| {
        is_valid_edge_edge_frame(e, frame.addr(), t, info)
    });
    e.mem.set_u8(valid.addr(), u8::from(ok));
    valid
}

/// The body of `is_valid_edge_edge` with its frame at `f` (the exe's `ESP + offset` after the
/// two pushes).
fn is_valid_edge_edge_frame(e: &mut Engine, f: u32, t: u32, info: u32) -> bool {
    let word_a = e.mem.u32(info + 0x38);
    let word_b = e.mem.u32(info + 0x3c);
    let row_a = load_vec(e, SIGN_ROWS + (word_a & 0x70));
    let index_b = word_b & 0xf;
    let mut x2 = mul(load_vec(e, t + 0x60), row_a);
    let row_b = load_vec(e, SIGN_ROWS + (word_b & 0x70));
    let index_a = word_a & 0xf;
    let mut x = X87::new();
    x.fld(e, t + index_b * 4 + 0x70);
    x.fadd_self();
    let x5 = load_vec(e, t + 0x20);
    x.fstp(e, f + 0x14);
    store_vec(e, f + 0x50, row_a);
    let ext_b = load_vec(e, t + 0x70);
    x.fld(e, t + index_a * 4 + 0x60);
    x.fmul64(e, MINUS_TWO);
    let x1 = mul(row_b, ext_b);
    let two_b = movss(e, f + 0x14);
    x.fstp(e, f + 0x14);
    let edge_row = load_vec(e, t + (index_b + 2) * 16);
    let x0 = mul(splat(two_b, 0), edge_row);
    let x3 = load_vec(e, t + 0x30);
    store_vec(e, f + 0x30, x0);
    x.fld(e, f + 0x30 + index_a * 4);
    x.fld(e, f + 0x14);
    x.fld_st(0);
    let x4 = splat(x1, 1);
    x.fmulp(2);
    let x4 = mul(x4, x3);
    x.fxch(1);
    let x3 = splat(x1, 0);
    x.fstp(e, f + 0x10);
    let x3 = mul(x3, x5);
    x.fld(e, t + index_b * 4 + 0x70);
    let x4 = add(x4, x3);
    x.fmul_self();
    let x3 = splat(x1, 2);
    let x1_z = load_vec(e, t + 0x40);
    x.fmul64(e, FOUR);
    let x3 = mul(x3, x1_z);
    let x1_w = load_vec(e, t + 0x50);
    let x3 = add(x3, x1_w);
    x.fstp(e, f + 0x18);
    let x4 = add(x4, x3);
    let x4 = sub(x4, x2);
    x.fld_st(0);
    store_vec(e, f + 0x20, x2);
    x.fld(e, f + 0x10);
    let x1 = mul(x4, x0);
    x.fld_st(0);
    x.fld_st(2);
    x.fmulp(3);
    x.fxch(2);
    x.fstp(e, f + 0x10);
    let sum = sum3(x1);
    x.fld(e, f + 0x10);
    x.fld(e, f + 0x18);
    store_vec(e, f + 0x40, x4);
    e.mem.set_u32(f + 0x14, sum.to_bits());
    x.fld_st(0);
    x.fmulp(2);
    x.fld_st(3);
    x.fmulp(4);
    x.fxch(1);
    x.fsubrp(3);
    x.fxch(2);
    x.fstp(e, f + 0x18);
    x.fld(e, f + 0x18);
    x.fabs();
    x.fstp(e, f + 0x18);
    x.fld(e, f + 0x18);
    x.fstp(e, f + 0x1c);
    x.fld(e, f + index_a * 4 + 0x40);
    x.fmul_st0(3);
    x.fstp(e, f + 0x18);
    x.fld(e, f + 0x18);
    x.fmul_st0(2);
    x.fld(e, f + 0x14);
    x.fmul_st0(2);
    x.fsubp(1);
    x.fstp(e, f + 0x10);
    x.fld(e, f + 0x10);
    x.fld(e, f + 0x1c);
    x.fld64(e, EPSILON);
    x.fmul_st0(1);
    x.fstp(e, f + 0x1c);
    x.fld(e, f + 0x1c);
    let result = x.fcomp(2);
    if !result.c0 {
        return false;
    }
    x.fld_st(0);
    x.fld64(e, NEARLY_ONE);
    // 00db172f is `DC C9`, `FMUL ST(1),ST(0)` (Ghidra lists it as `FMUL ST1`, like the `D8`
    // form): the product goes to ST1, 0.99999988 stays on top for the `FMULP` at 00db1777, and
    // the `FCOMP ST3` after the `FXCH` compares the product, not the bare value.
    x.fmul_into(1);
    x.fxch(1);
    let result = x.fcomp(3);
    if result.c0 != result.c3 {
        return false;
    }
    x.fxch(2);
    x.fdivrp(1);
    x.fstp(e, f + 0x10);
    x.fld(e, f + 0x10);
    x.fld_st(0);
    x.fmulp(3);
    x.fld(e, f + 0x14);
    x.fsubp(3);
    x.fxch(2);
    x.fstp(e, f + 0x1c);
    x.fld(e, f + 0x1c);
    x.fld_st(3);
    x.fmul64(e, EPSILON);
    let result = x.fcomp(1);
    if !result.c0 {
        return false;
    }
    x.fxch(3);
    x.fmulp(1);
    x.fstp(e, f + 0x1c);
    x.fld(e, f + 0x1c);
    let result = x.fcomp(2);
    x.fstp_st(1);
    if result.c0 != result.c3 {
        return false;
    }
    x.fmul_st0(1);
    store_vec(e, f + 0x30, [0.0; 4]);
    x.fadd(e, f + 0x20 + index_a * 4);
    let x3 = shufps(x0, x0, 0xd2);
    x.fstp(e, f + 0x20 + index_a * 4);
    let x0 = shufps(x0, x0, 0xc9);
    x.fmul64(e, HALF);
    x.fstp(e, f + 0x30 + index_a * 4);
    let x1 = load_vec(e, f + 0x30);
    x2 = mul(shufps(x1, x1, 0xc9), x3);
    let x3 = mul(shufps(x1, x1, 0xd2), x0);
    x2 = sub(x2, x3);
    let squares = mul(x2, x2);
    let sum = sum3(squares);
    let inverse = inverse_length(e, sum);
    let mut normal = mul(splat([inverse, 0.0, 0.0, 0.0], 0), x2);
    let length = sum3(squares) * inverse;
    e.mem.set_u32(f + 0x1c, length.to_bits());
    x.fld(e, f + 0x1c);
    x.fld(e, t + 0xb4);
    let result = x.fcompp();
    if !result.c0 && !result.c3 {
        return false;
    }
    let row_a_vector = load_vec(e, f + 0x50);
    x.fldz();
    let along = sum3(mul(row_a_vector, normal));
    e.mem.set_u32(f + 0x1c, along.to_bits());
    let result = x.fcomp32(e, f + 0x1c);
    if !result.c0 && !result.c3 {
        normal = xorps(normal, load_vec(e, SIGN_MASK));
    }
    let distance = sum3(mul(normal, x4));
    e.mem.set_u32(f + 0x1c, distance.to_bits());
    x.fld(e, f + 0x1c);
    x.fld(e, t + 0xb0);
    let result = x.fcomp(1);
    if result.c0 != result.c2 {
        return false;
    }
    let point = load_vec(e, f + 0x20);
    store_vec(e, info, point);
    x.fstp(e, info + 0x34);
    store_vec(e, info + 0x20, xorps(normal, load_vec(e, SIGN_MASK)));
    true
}

// Translated from 00db1990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxCollisionDetection::addAdditionalEdgeHelper` (engine map; `__thiscall`, `RET 0xc`:
/// the manifold, an entry of 4 bytes (kind, code, a word) and a distance threshold). Does
/// nothing when `findInManifold` already has the entry. Otherwise it asks `isValidEdgeEdge`
/// for the contact between the two edges (a record on the stack) and goes on only when
/// `threshold * 0.999 <= distance`. For a kind up to 6 it transforms the point of the record
/// and returns when it lies within the squared tolerance (`+0xb0`, plus `FLT_EPSILON`) of a
/// contact point of the same kind already in the manifold. It then needs room (fewer than 8
/// entries) and `addPoint` to succeed, fills the next contact point record (position,
/// normal, distance, the entry's word at `+0x20`) from the transformed point and normal of
/// the matching box, drops the point again when it does not point the same way as the
/// previous one, lets the virtual function `+0xc` of the object at `this + 0xc` create it
/// (`0xffff` drops the point), advances the cursor by `0x30`, stores the created id in the
/// entry and the manifold and counts the point (`+0x20`) when the kind is up to 6.
pub fn add_additional_edge_helper(
    e: &mut Engine,
    this: Ptr,
    manifold: Ptr,
    entry: Ptr,
    threshold: f32,
) {
    let (t, manifold, entry) = (this.addr(), manifold.addr(), entry.addr());
    e.with_stack(0x60, |e, frame| {
        add_additional_edge_helper_frame(e, frame.addr(), t, manifold, entry, threshold)
    });
}

/// The body of `add_additional_edge_helper` with its frame at `f` (the exe's `ESP + offset`
/// after the three pushes).
fn add_additional_edge_helper_frame(
    e: &mut Engine,
    f: u32,
    t: u32,
    manifold: u32,
    entry: u32,
    threshold: f32,
) {
    let found = e
        .call(FIND_IN_MANIFOLD, &args![manifold, f + 0x17, entry])
        .u32();
    if e.mem.u8(found) != 0 {
        return;
    }
    let kind = u32::from(e.mem.u8(entry));
    let code = u32::from(e.mem.u8(entry + 1));
    let mut x = X87::new();
    x.fldz();
    x.fstp(e, f + 0x50);
    e.mem.set_u32(f + 0x58, kind);
    store_vec(e, f + 0x20, [0.0; 4]);
    store_vec(e, f + 0x30, [0.0; 4]);
    e.mem.set_u32(f + 0x5c, code);
    is_valid_edge_edge(e, Ptr::new(t), Ptr::new(f + 0x17), Ptr::new(f + 0x20));
    if e.mem.u8(f + 0x17) == 0 {
        return;
    }
    x.fld(e, f + 0x54);
    x.fld_value(threshold);
    x.fmul64(e, THRESHOLD_SCALE);
    let result = x.fcompp();
    if result.c0 == result.c3 {
        return;
    }
    let kind_byte = e.mem.u8(entry);
    let first_vector = load_vec(e, f + 0x30);
    let second_vector = load_vec(e, f + 0x20);
    if kind_byte <= 6 {
        let point = if kind_byte <= 2 {
            transform_point(e, e.mem.u32(t + 0x18), first_vector)
        } else {
            transform_point(e, e.mem.u32(t + 0x14), second_vector)
        };
        if e.mem.u8(manifold + 0x21) != 0 {
            let mut index = 0u32;
            x.fld64(e, EPSILON);
            while index < u32::from(e.mem.u8(manifold + 0x21)) {
                if e.mem.u8(manifold + index * 4) <= 6 {
                    let records = e.mem.u32(t + 0x10);
                    x.fld(e, t + 0xb0);
                    let other = load_vec(e, records + index * 0x30 + 0x10);
                    let difference = sub(point, other);
                    let squared = sum3(mul(difference, difference));
                    e.mem.set_u32(f + 0x18, squared.to_bits());
                    x.fld(e, f + 0x18);
                    x.fld_st(1);
                    x.fmulp(2);
                    x.fxch(1);
                    x.fadd_st0(2);
                    x.fstp(e, f + 0x1c);
                    x.fld(e, f + 0x1c);
                    let result = x.fcompp();
                    if !result.c0 {
                        return;
                    }
                }
                index += 1;
            }
            x.pop();
        }
    }
    if e.mem.u8(manifold + 0x21) >= 8 {
        return;
    }
    let (body_a, body_b) = (e.mem.u32(t), e.mem.u32(t + 4));
    let point_id = e
        .call(ADD_POINT, &args![manifold, body_a, body_b, entry])
        .u32();
    e.mem.set_u32(f + 0x18, point_id);
    if (point_id as i32) < 0 {
        return;
    }
    let cursor = e.mem.u32(t + 0x10);
    let record = e.mem.u32(cursor);
    let kind_word = e.mem.u32(f + 0x58);
    let (point, normal) = if (kind_word as i32) <= 2 {
        let entry_kind = u32::from(e.mem.u8(entry));
        x.fld(e, f + 0x50);
        x.fchs();
        let entry_word = u32::from(e.mem.u16(entry + 2));
        x.fstp(e, f + 0x1c);
        let vector = load_vec(e, f + 0x30);
        e.mem.set_u32(record + 0x20, entry_word);
        let point = transform_point(e, e.mem.u32(t + 0x18), vector);
        let negated = movss(e, f + 0x1c);
        let axis = load_vec(e, (entry_kind << 4).wrapping_add(e.mem.u32(t + 0x14)));
        (point, mul(splat(negated, 0), axis))
    } else if (kind_word as i32) <= 6 {
        let entry_kind = u32::from(e.mem.u8(entry));
        let vector = load_vec(e, f + 0x20);
        x.fld(e, f + 0x50);
        x.fchs();
        let entry_word = u32::from(e.mem.u16(entry + 2));
        x.fstp(e, f + 0x1c);
        e.mem.set_u32(record + 0x20, entry_word);
        let point = transform_point(e, e.mem.u32(t + 0x14), vector);
        let negated = movss(e, f + 0x1c);
        let axis = load_vec(
            e,
            (entry_kind.wrapping_sub(4) << 4).wrapping_add(e.mem.u32(t + 0x18)),
        );
        (point, mul(splat(negated, 0), axis))
    } else {
        let entry_word = u32::from(e.mem.u16(entry + 2));
        e.mem.set_u32(record + 0x20, entry_word);
        let vector = load_vec(e, f + 0x20);
        let point = transform_point(e, e.mem.u32(t + 0x14), vector);
        let edge = load_vec(e, f + 0x40);
        (point, transform_direction(e, e.mem.u32(t + 0x14), edge))
    };
    store_vec(e, record, point);
    x.fld(e, f + 0x54);
    store_vec(e, record + 0x10, normal);
    x.fstp(e, record + 0x1c);
    if e.mem.u8(manifold + 0x21) > 1 {
        let cursor = e.mem.u32(t + 0x10);
        x.fldz();
        let current = e.mem.u32(cursor);
        let direction = load_vec(e, record + 0x10);
        let previous = load_vec(e, current.wrapping_sub(0x20));
        let along = sum3(mul(previous, direction));
        e.mem.set_u32(f + 0x1c, along.to_bits());
        let result = x.fcomp32(e, f + 0x1c);
        if !result.c0 {
            e.call(REMOVE_POINT, &args![manifold, point_id]);
            return;
        }
    }
    let object = e.mem.u32(t + 0xc);
    let (body_a, body_b, extra, cursor) = (
        e.mem.u32(t),
        e.mem.u32(t + 4),
        e.mem.u32(t + 8),
        e.mem.u32(t + 0x10),
    );
    let created = e
        .vcall(
            object,
            0xc,
            &args![body_a, body_b, extra, cursor, 0u32, record],
        )
        .u16();
    e.mem.set_u16(manifold + point_id * 4 + 2, created);
    if created == 0xffff {
        e.call(REMOVE_POINT, &args![manifold, point_id]);
        return;
    }
    let cursor = e.mem.u32(t + 0x10);
    let advanced = e.mem.u32(cursor).wrapping_add(0x30);
    e.mem.set_u32(cursor, advanced);
    let stored = e.mem.u16(manifold + point_id * 4 + 2);
    e.mem.set_u16(entry + 2, stored);
    e.mem.set_u32(record + 0x20, u32::from(stored));
    if e.mem.u8(entry) <= 6 {
        let counted = e.mem.u8(manifold + 0x20).wrapping_add(1);
        e.mem.set_u8(manifold + 0x20, counted);
    }
}

// Translated from 00db1d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the four additional edge contacts around an edge pair (no engine-map name; `__thiscall`,
/// `RET 0x20`: the manifold, the axes `axis_a` and `axis_b`, two side selectors, two vectors
/// and a threshold). The entry (kind, code) is built like `fn_00db0dc0` does (the first
/// vector gets 1.0 in lane `axis_a`, the second 1.0 in lane `axis_b`; an axis above 3 panics
/// here, the exe writes past the vector), then `add_additional_edge_helper` runs for it, for
/// the entry with the side bit `4 + s_a` of its first byte flipped (`s` is 0 for a selector 0,
/// else 1 for a selector 1 and 2 for any other), with the side bit `4 + s_b` of its second byte
/// flipped as well, and with the first byte restored.
#[allow(clippy::too_many_arguments)]
pub fn fn_00db1d90(
    e: &mut Engine,
    this: Ptr,
    manifold: Ptr,
    axis_a: u32,
    axis_b: u32,
    selector_a: u32,
    selector_b: u32,
    vector_a: Ptr,
    vector_b: Ptr,
    threshold: f32,
) {
    let mut a = load_vec(e, vector_a.addr());
    let mut b = load_vec(e, vector_b.addr());
    a[axis_a as usize] = 1.0;
    b[axis_b as usize] = 1.0;
    let negative_a = movmskps(cmpltps(a, [0.0; 4]));
    let negative_b = movmskps(cmpltps(b, [0.0; 4]));
    let first = (((negative_a as u8) | 0xf8) << 4) | axis_a as u8;
    let second = (((!negative_b) as u8 & 7) << 4).wrapping_add(axis_b as u8);
    let side = |selector: u32| {
        if selector == 0 {
            0
        } else {
            u32::from(selector != 1) + 1
        }
    };
    let flip_a = (1u32 << (side(selector_a) + 4)) as u8;
    let flip_b = (1u32 << (side(selector_b) + 4)) as u8;
    let threshold = x87_quiet(threshold);
    e.with_stack(4, |e, entry| {
        let entry = entry.addr();
        e.mem.set_u32(entry, 0);
        e.mem.set_u8(entry, first);
        e.mem.set_u8(entry + 1, second);
        add_additional_edge_helper(e, this, manifold, Ptr::new(entry), threshold);
        let flipped = e.mem.u8(entry) ^ flip_a;
        e.mem.set_u8(entry, flipped);
        add_additional_edge_helper(e, this, manifold, Ptr::new(entry), threshold);
        let flipped = e.mem.u8(entry + 1) ^ flip_b;
        e.mem.set_u8(entry + 1, flipped);
        add_additional_edge_helper(e, this, manifold, Ptr::new(entry), threshold);
        let flipped = e.mem.u8(entry) ^ flip_a;
        e.mem.set_u8(entry, flipped);
        add_additional_edge_helper(e, this, manifold, Ptr::new(entry), threshold);
    });
}

/// The candidate separations `findClosestPoint` looks at: the offset in the detection state and
/// the feature index (the box A faces 0 to 2, the box B faces 4 to 6, the nine edge pairs
/// 8 to 10, 12 to 14 and 16 to 18; the last one, 18, is the final comparison).
const FEATURES: [(u32, u32); 14] = [
    (0xe0, 0),
    (0xe4, 1),
    (0xe8, 2),
    (0xf0, 4),
    (0xf4, 5),
    (0xf8, 6),
    (0x100, 8),
    (0x104, 9),
    (0x108, 10),
    (0x110, 12),
    (0x114, 13),
    (0x118, 14),
    (0x120, 16),
    (0x124, 17),
];

/// The largest separation (a candidate replaces the best one only when the best is below it)
/// and its feature index.
fn largest_feature(e: &Engine, t: u32) -> (f32, u32) {
    let mut best = x87_quiet(e.mem.f32(t + 0xe0));
    let mut index = 0;
    for (offset, feature) in FEATURES {
        let value = x87_quiet(e.mem.f32(t + offset));
        if best < value {
            best = value;
            index = feature;
        }
    }
    (best, index)
}

// Translated from 00db1ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxCollisionDetection::findClosestPoint` (engine map; `__thiscall`, `RET 0xc`: the
/// manifold, two result bytes and a result record of 0x40 bytes). Up to eight times it takes
/// the feature with the largest separation (the faces of the first box, of the second box and
/// the edge pairs, the last of which only counts when it is above the separation at `+0x128`),
/// builds its closest point (record: the point at `+0x00`, the second point or normal at
/// `+0x10`, the side `+-1.0` at `+0x30`, the distance at `+0x34`, the feature words at
/// `+0x38` and `+0x3c`) and its two code bytes, and returns 1 when the manifold already holds
/// them or 2 when the point is within the tolerance extents (`+0x90`, `+0xa0`) and, for an
/// edge pair, `isValidEdgeEdge` accepts it. Otherwise the separation is overwritten with
/// `0xff7fffee` and the next feature is tried; it returns 0 after eight failures.
pub fn find_closest_point(e: &mut Engine, this: Ptr, manifold: Ptr, codes: Ptr, out: Ptr) -> u32 {
    let (t, manifold, codes, out) = (this.addr(), manifold.addr(), codes.addr(), out.addr());
    for _ in 0..8 {
        let (best, index) = largest_feature(e, t);
        let limit = x87_quiet(e.mem.f32(t + 0x128));
        let feature = if best < limit { 18 } else { index };
        let outcome = if feature <= 2 {
            closest_on_first_box(e, t, manifold, codes, out, feature)
        } else if feature <= 6 {
            closest_on_second_box(e, t, manifold, codes, out, feature)
        } else {
            closest_between_edges(e, t, manifold, codes, out, feature)
        };
        if let Some(result) = outcome {
            return result;
        }
        let used = x87_quiet_bits(e.mem.u32(USED_UP));
        e.mem.set_u32(t + 0xe0 + feature * 4, used);
    }
    0
}

/// The face of the first box (`feature` 0 to 2) in `find_closest_point`: `Some(result)` to
/// return it, `None` to try the next feature.
fn closest_on_first_box(
    e: &mut Engine,
    t: u32,
    manifold: u32,
    codes: u32,
    out: u32,
    feature: u32,
) -> Option<u32> {
    e.mem.set_u8(codes, feature as u8);
    e.mem.set_u32(out + 0x38, feature);
    let offset = e.mem.f32(t + 0xc0 + feature * 4);
    let column = [
        e.mem.f32(t + 0x20 + feature * 4),
        e.mem.f32(t + 0x30 + feature * 4),
        e.mem.f32(t + 0x40 + feature * 4),
        e.mem.f32(ZERO_F32),
    ];
    let axis = load_vec(e, t + 0x70);
    let sign_mask = load_vec(e, SIGN_MASK);
    let (side, axis) = if offset < 0.0 {
        (-1.0f32, axis)
    } else {
        (1.0f32, xorps(axis, sign_mask))
    };
    e.mem.set_f32(out + 0x30, side);
    store_vec(e, out + 0x10, axis);
    let positive = cmpltps([e.mem.f32(ZERO_F32); 4], load_vec(e, t + 0xd0));
    let small = orps(
        andps(splat(movss(e, SMALL_POSITIVE), 0), positive),
        andnps(positive, splat(movss(e, SMALL_NEGATIVE), 0)),
    );
    let nudged = add(mul(small, splat(movss(e, out + 0x30), 0)), column);
    let adjusted = xorps(andps(nudged, sign_mask), load_vec(e, out + 0x10));
    store_vec(e, out + 0x10, adjusted);
    let point = transform_point(e, t + 0x20, adjusted);
    store_vec(e, out, point);
    let mask = movmskps(cmpltps(adjusted, [0.0; 4]));
    let code = feature_byte(mask, e.mem.u32(out + 0x30));
    e.mem.set_u8(codes + 1, code);
    if has_entry(e, manifold, e.mem.u8(codes), code) {
        return Some(1);
    }
    let point = load_vec(e, out);
    let inside = movmskps(cmpltps(
        andps(point, load_vec(e, ABS_MASK)),
        load_vec(e, t + 0x90),
    ));
    if inside & 7 != 7 {
        return None;
    }
    let mut x = X87::new();
    x.fld(e, out + feature * 4);
    x.fmul(e, out + 0x30);
    x.fsub(e, t + 0x60 + feature * 4);
    x.fstp(e, out + 0x34);
    Some(2)
}

/// The face of the second box (`feature` 3 to 6) in `find_closest_point`.
fn closest_on_second_box(
    e: &mut Engine,
    t: u32,
    manifold: u32,
    codes: u32,
    out: u32,
    feature: u32,
) -> Option<u32> {
    e.mem.set_u8(codes, feature as u8);
    e.mem.set_u32(out + 0x38, feature);
    let offset = e.mem.f32(t + 0xc0 + feature * 4);
    let axis_row = load_vec(e, t + feature.wrapping_sub(2).wrapping_mul(16));
    let axis = load_vec(e, t + 0x60);
    let sign_mask = load_vec(e, SIGN_MASK);
    let (side, axis) = if offset < 0.0 {
        (-1.0f32, xorps(axis, sign_mask))
    } else {
        (1.0f32, axis)
    };
    e.mem.set_f32(out + 0x30, side);
    store_vec(e, out, axis);
    let positive = cmpltps([e.mem.f32(ZERO_F32); 4], load_vec(e, t + 0xc0));
    let small = orps(
        andps(splat(movss(e, SMALL_NEGATIVE), 0), positive),
        andnps(positive, splat(movss(e, SMALL_POSITIVE), 0)),
    );
    let nudged = add(mul(small, splat([-side, 0.0, 0.0, 0.0], 0)), axis_row);
    let adjusted = xorps(andps(nudged, sign_mask), load_vec(e, out));
    store_vec(e, out, adjusted);
    let relative = sub(adjusted, load_vec(e, t + 0x50));
    let second = rows_times_columns(e, t + 0x20, relative);
    store_vec(e, out + 0x10, second);
    let mask = movmskps(cmpltps(adjusted, [0.0; 4]));
    let code = feature_byte(mask, e.mem.u32(out + 0x30));
    e.mem.set_u8(codes + 1, code);
    if has_entry(e, manifold, e.mem.u8(codes), code) {
        return Some(1);
    }
    let second = load_vec(e, out + 0x10);
    let inside = movmskps(cmpltps(
        andps(second, load_vec(e, ABS_MASK)),
        load_vec(e, t + 0xa0),
    ));
    if inside & 7 != 7 {
        return None;
    }
    let lane = feature.wrapping_sub(4);
    let mut x = X87::new();
    x.fld(e, out + 0x10 + lane * 4);
    x.fld(e, out + 0x30);
    x.fchs();
    x.fmulp(1);
    x.fsub(e, t + 0x70 + lane * 4);
    x.fstp(e, out + 0x34);
    Some(2)
}

/// The edge pair (`feature` 7 and above) in `find_closest_point`.
fn closest_between_edges(
    e: &mut Engine,
    t: u32,
    manifold: u32,
    codes: u32,
    out: u32,
    feature: u32,
) -> Option<u32> {
    let relative = feature.wrapping_sub(8);
    let second_axis = relative & 3;
    let first_axis = relative >> 2;
    e.mem.set_u32(out + 0x38, first_axis);
    e.mem.set_u32(out + 0x3c, second_axis);
    let edge = load_vec(e, t + (second_axis + 2) * 16);
    let mut unit = [0.0f32; 4];
    unit[first_axis as usize] = 1.0;
    let mut normal = cross(unit, edge);
    let along = sum3(mul(load_vec(e, t + 0xc0), normal));
    if along < 0.0 {
        normal = xorps(normal, load_vec(e, SIGN_MASK));
    }
    store_vec(e, out, normal);
    let mut first_unit = normal;
    first_unit[first_axis as usize] = 1.0;
    let second = rows_times_columns(e, t + 0x20, normal);
    store_vec(e, out + 0x10, second);
    let mut second_unit = second;
    second_unit[second_axis as usize] = 1.0;
    let negative_first = movmskps(cmpltps(first_unit, [0.0; 4]));
    let negative_second = movmskps(cmpltps(second_unit, [0.0; 4]));
    let first = (((negative_first as u8) | 0xf8) << 4) | first_axis as u8;
    let second_code = (((!negative_second) as u8 & 7) << 4).wrapping_add(second_axis as u8);
    e.mem.set_u8(codes, first);
    e.mem.set_u8(codes + 1, second_code);
    e.mem.set_u32(out + 0x38, u32::from(first));
    e.mem.set_u32(out + 0x3c, u32::from(second_code));
    if has_entry(e, manifold, first, second_code) {
        return Some(1);
    }
    let valid = e.with_stack(4, |e, slot| {
        is_valid_edge_edge(e, Ptr::new(t), slot, Ptr::new(out));
        e.mem.u8(slot.addr()) != 0
    });
    if valid {
        Some(2)
    } else {
        None
    }
}

// Translated from 00db2750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxCollisionDetection::calculateClosestPoint` (engine map; `__thiscall`, `RET 8`: a
/// result byte and a result record of 8 floats). Clears a manifold on its frame, prepares the
/// query record (`00d40940`), runs `checkIntersection` (tolerance at `+0x80`) and, when
/// nothing separates the boxes, `findClosestPoint`. When that finds a point (2) the byte
/// is 1 and the record gets the point (`+0`) and normal (`+0x10`) in world space, the
/// normal moved along itself by minus the distance for a feature code above 2 (the distance
/// is also stored at `+0x1c`); otherwise (or when a separating axis exists) the byte is 0.
/// Returns the byte's address.
pub fn calculate_closest_point(e: &mut Engine, this: Ptr, result: Ptr, out: Ptr) -> Ptr {
    let (t, result, out_record) = (this.addr(), result.addr(), out.addr());
    e.with_stack(0xc0, |e, frame| {
        let f = frame.addr();
        e.mem.set_u8(result, 0);
        e.call(CLEAR_MANIFOLD, &args![f + 0x80]);
        e.mem.set_u16(f + 0xe, 0);
        e.call(PREPARE_QUERY, &args![t]);
        if check_intersection(e, this, Ptr::new(t + 0x80)) != 0 {
            return;
        }
        let found = find_closest_point(
            e,
            this,
            Ptr::new(f + 0x80),
            Ptr::new(f + 0xc),
            Ptr::new(f + 0x10),
        );
        if found != 2 {
            e.mem.set_u8(result, 0);
            return;
        }
        let kind = e.mem.u32(f + 0x48) as i32;
        let code = e.mem.u8(f + 0xc);
        e.mem.set_u8(result, 1);
        let (point, normal) = if kind <= 2 {
            let vector = load_vec(e, f + 0x20);
            let mut x = X87::new();
            x.fld(e, f + 0x40);
            x.fchs();
            x.fstp(e, f + 0xc);
            let point = transform_point(e, e.mem.u32(t + 0x18), vector);
            let negated = movss(e, f + 0xc);
            let axis = load_vec(e, (u32::from(code) << 4).wrapping_add(e.mem.u32(t + 0x14)));
            (point, mul(splat(negated, 0), axis))
        } else if kind <= 6 {
            let vector = load_vec(e, f + 0x10);
            let mut x = X87::new();
            x.fld(e, f + 0x40);
            x.fchs();
            x.fstp(e, f + 0xc);
            let point = transform_point(e, e.mem.u32(t + 0x14), vector);
            let negated = movss(e, f + 0xc);
            let axis = load_vec(
                e,
                (u32::from(code).wrapping_sub(4) << 4).wrapping_add(e.mem.u32(t + 0x18)),
            );
            (point, mul(splat(negated, 0), axis))
        } else {
            let matrix = e.mem.u32(t + 0x14);
            let vector = load_vec(e, f + 0x10);
            let point = transform_point(e, matrix, vector);
            let edge = load_vec(e, f + 0x30);
            (point, transform_direction(e, matrix, edge))
        };
        let mut x = X87::new();
        x.fld(e, f + 0x44);
        let mut normal = normal;
        normal[3] = x87_quiet(e.mem.f32(f + 0x44));
        let mut point = point;
        if code > 2 {
            x.fld_st(0);
            x.fchs();
            x.fstp(e, f + 0xc);
            point = add(mul(splat(movss(e, f + 0xc), 0), normal), point);
        }
        store_vec(e, out_record + 0x10, normal);
        store_vec(e, out_record, point);
        x.fstp(e, out_record + 0x1c);
    });
    Ptr::new(result)
}

/// The addresses `calc_manifold` works with: its stack frame (the exe's `ESP` after the three
/// pushes; the offsets below are `ESP + offset`), the detection state and the manifold.
#[derive(Clone, Copy)]
struct Calc {
    f: u32,
    t: u32,
    m: u32,
}

/// `value / 0x30` the way the exe divides the distance between two record pointers: the high
/// word of a product with `0x2aaaaaab`, shifted right by 3, plus its sign bit.
fn divide_by_48(value: u32) -> i32 {
    let product = i64::from(value as i32) * 0x2aaa_aaab;
    let high = ((product >> 32) as i32) >> 3;
    high + ((high as u32) >> 31) as i32
}

/// The sign bits of a feature code: bit 3 of `code` shifted to bit 31 (the sign of the float
/// 1.0 the code is or-ed into).
fn sign_bits(code: u32) -> u32 {
    (code & 0xffff_fff8) << 0x1c
}

/// Whether the absolute values of lanes 0 to 2 of `v` are all below the lanes of the vector at
/// `limit` (the mask `7` of `MOVMSKPS`).
fn inside(e: &Engine, v: Vec4, limit: u32) -> bool {
    movmskps(cmpltps(andps(v, load_vec(e, ABS_MASK)), load_vec(e, limit))) & 7 == 7
}

/// A `float` copied through the x87 stack (a signalling NaN is quieted).
fn copy_quieted(e: &mut Engine, from: u32, to: u32) {
    let value = x87_quiet(e.mem.f32(from));
    e.mem.set_f32(to, value);
}

/// Takes the next contact record: the cursor cell at `+0x10` holds the record's address; the
/// cell moves on by `0x30`.
fn take_record(e: &mut Engine, t: u32) -> u32 {
    let cell = e.mem.u32(t + 0x10);
    let record = e.mem.u32(cell);
    e.mem.set_u32(cell, record.wrapping_add(0x30));
    record
}

/// `hkpBoxBoxManifold::addPoint` for the key at `key`, with the two bodies of the detection
/// state.
fn add_point(e: &mut Engine, c: Calc, key: u32) -> u32 {
    let (body_a, body_b) = (e.mem.u32(c.t), e.mem.u32(c.t + 4));
    e.call(ADD_POINT, &args![c.m, body_a, body_b, key]).u32()
}

/// Writes the contact record at `record` for the point `calc_manifold` just found: a feature
/// `kind` up to 2 is a point of the second box (frame `+0x70`, transformed by the first
/// body's transform at `+0x18`) with the normal row `row_kind` of the second transform
/// (`+0x14`) scaled by `negated`; up to 6 a point of the first box (frame `+0x60`, the `+0x14`
/// transform) with the row `row_kind - 4` of the `+0x18` transform; anything else a point of
/// an edge pair (frame `+0x60`) with the direction at frame `+0x80`. The distance at frame
/// `+0x94` goes to `+0x1c` and the word to `+0x20`.
fn write_record(
    e: &mut Engine,
    c: Calc,
    record: u32,
    kind: u32,
    row_kind: u32,
    word: u32,
    negated: f32,
) {
    let Calc { f, t, .. } = c;
    e.mem.set_u32(record + 0x20, word);
    let (point, normal) = if (kind as i32) <= 2 {
        let point = transform_point(e, e.mem.u32(t + 0x18), load_vec(e, f + 0x70));
        let row = load_vec(e, (row_kind << 4).wrapping_add(e.mem.u32(t + 0x14)));
        (point, mul([negated; 4], row))
    } else if (kind as i32) <= 6 {
        let point = transform_point(e, e.mem.u32(t + 0x14), load_vec(e, f + 0x60));
        let row = load_vec(
            e,
            (row_kind.wrapping_sub(4) << 4).wrapping_add(e.mem.u32(t + 0x18)),
        );
        (point, mul([negated; 4], row))
    } else {
        let matrix = e.mem.u32(t + 0x14);
        let point = transform_point(e, matrix, load_vec(e, f + 0x60));
        (point, transform_direction(e, matrix, load_vec(e, f + 0x80)))
    };
    store_vec(e, record, point);
    store_vec(e, record + 0x10, normal);
    copy_quieted(e, f + 0x94, record + 0x1c);
}

/// The test every new point of `calc_manifold` starts with: whether a point of kind up to 6 in
/// the manifold lies within `+0xb0` of `point` (the squared distance, stored at `sum_slot`,
/// against `+0xb0` squared plus the epsilon on ST1, stored at `total_slot`; both rounded to a
/// `float`).
fn near_existing(e: &mut Engine, x: &mut X87, c: Calc, point: Vec4, slots: (u32, u32)) -> bool {
    let (sum_slot, total_slot) = slots;
    let mut index = 0u32;
    while index < u32::from(e.mem.u8(c.m + 0x21)) {
        if e.mem.u8(c.m + index * 4) <= 6 {
            let records = e.mem.u32(c.t + 0x10);
            x.fld(e, c.t + 0xb0);
            let other = load_vec(e, records + index * 0x30 + 0x10);
            let difference = sub(point, other);
            let squared = sum3(mul(difference, difference));
            e.mem.set_u32(sum_slot, squared.to_bits());
            x.fld(e, sum_slot);
            x.fld_st(1);
            x.fmulp(2);
            x.fxch(1);
            x.fadd_st0(2);
            x.fstp(e, total_slot);
            x.fld(e, total_slot);
            if !x.fcompp().c0 {
                return true;
            }
        }
        index += 1;
    }
    false
}

/// A point `calc_manifold` has added to the manifold and written a record for.
struct NewPoint {
    /// The manifold index `addPoint` returned.
    id: u32,
    record: u32,
    /// The frame slot of the key (kind, code, word) the point was added with.
    key: u32,
    /// The frame slot the dot product of the two last normals goes through.
    dot_slot: u32,
    /// Whether the new id is also stored in the key's word at `+0x2e` of the frame.
    update_word: bool,
}

/// The end every added point of `calc_manifold` shares: with more than one point the new
/// normal must point the same way as the previous record's, or `removePoint` drops it again
/// (it returns false); then the virtual function `+0xc` of the object at `+0xc` creates the
/// contact point (`0xffff` drops it), the cursor moves on by `0x30`, the created id goes into
/// the record and the points of kind up to 6 are counted in `+0x20`.
fn complete_point(e: &mut Engine, x: &mut X87, c: Calc, p: NewPoint) -> bool {
    let Calc { f, t, m } = c;
    if e.mem.u8(m + 0x21) > 1 {
        x.fldz();
        let cell = e.mem.u32(t + 0x10);
        let current = e.mem.u32(cell);
        let normal = load_vec(e, p.record + 0x10);
        let previous = load_vec(e, current.wrapping_sub(0x20));
        let along = sum3(mul(previous, normal));
        e.mem.set_u32(p.dot_slot, along.to_bits());
        if !x.fcomp32(e, p.dot_slot).c0 {
            e.call(REMOVE_POINT, &args![m, p.id]);
            return false;
        }
    }
    let object = e.mem.u32(t + 0xc);
    let (body_a, body_b, extra, cell) = (
        e.mem.u32(t),
        e.mem.u32(t + 4),
        e.mem.u32(t + 8),
        e.mem.u32(t + 0x10),
    );
    let created = e
        .vcall(
            object,
            0xc,
            &args![body_a, body_b, extra, cell, 0u32, p.record],
        )
        .u16();
    e.mem.set_u16(m + p.id * 4 + 2, created);
    if created == 0xffff {
        e.call(REMOVE_POINT, &args![m, p.id]);
        return true;
    }
    let cell = e.mem.u32(t + 0x10);
    let advanced = e.mem.u32(cell).wrapping_add(0x30);
    e.mem.set_u32(cell, advanced);
    let stored = e.mem.u16(m + p.id * 4 + 2);
    if p.update_word {
        e.mem.set_u16(f + 0x2e, stored);
    }
    e.mem.set_u32(p.record + 0x20, u32::from(stored));
    if e.mem.u8(p.key) <= 6 {
        let counted = e.mem.u8(m + 0x20).wrapping_add(1);
        e.mem.set_u8(m + 0x20, counted);
    }
    true
}

// Translated from 00db2960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxBoxCollisionDetection::calcManifold` (engine map; `__thiscall`, `RET 4`: the
/// manifold). Rebuilds the contact points of a box pair after `00d40940` has prepared the
/// query:
///
/// 1. Every manifold entry is checked again: a face of one box (kind up to 2: a corner of the
///    second box in the first box's frame, kind 3 to 6 the other way round) must still lie
///    within the tolerance extents (`+0x90`, `+0xa0`), an edge pair (kind 7 and up) must still
///    be valid (`isValidEdgeEdge`). A good entry gets a record (point, normal, distance, id)
///    at the cursor `+0x10`; a bad one is removed (virtual function `+0x14` of the object at
///    `+0xc`, then `removePoint`).
/// 2. With fewer than 2 entries the packed direction (`+0x23` flag, `+0x24` word) is cleared;
///    otherwise it is made from the normal at `+0x30` (`004b4ab0` and `00d975d0`) or, when it
///    exists, unpacked and compared with the normal (the manifold counts as moved when the dot
///    product is below 0.95; if not and `+0x22` is set, nothing more happens).
/// 3. `checkIntersection` runs with the tolerance extents lowered by the smallest record
///    distance minus 0.05 of `+0x80` (or `+0x80` itself without entries); a separation ends it.
/// 4. `findClosestPoint` picks the feature; for a moved manifold the normal at `+0x30` is set
///    from it, and the records whose normal is no longer within 0.95 of the rotated normal are
///    removed again (the last record takes their place).
/// 5. A closest point of result 2 is added (unless it lies within `+0xb0` of a point or the
///    manifold holds 8 entries) with its record; a normal that points away from the last
///    one's drops it and ends the function.
/// 6. The two axes best aligned with the new normal give two more features; with the masks of
///    the seven words at `010d5350` each is tried as a neighbour (two loops) and added the
///    same way. With 4 or more points in `+0x20` the manifold counts as complete (`+0x22`),
///    otherwise `checkCompleteness` decides.
pub fn calc_manifold(e: &mut Engine, this: Ptr, manifold: Ptr) {
    let (t, m) = (this.addr(), manifold.addr());
    e.with_stack(0x100, |e, frame| {
        calc_manifold_frame(
            e,
            Calc {
                f: frame.addr(),
                t,
                m,
            },
        );
    });
}

/// The body of `calc_manifold`.
fn calc_manifold_frame(e: &mut Engine, c: Calc) {
    let Calc { f, t, .. } = c;
    e.call(PREPARE_QUERY, &args![t]);
    let cell = e.mem.u32(t + 0x10);
    let records = e.mem.u32(cell);
    e.mem.set_u32(f + 0x10, records);
    let mut x = X87::new();
    check_entries(e, &mut x, c);
    if !check_direction(e, &mut x, c) {
        return;
    }
    if !separate(e, &mut x, c) {
        return;
    }
    let result = find_closest(e, &mut x, c);
    if result != 2 {
        return;
    }
    if !add_closest_point(e, &mut x, c) {
        return;
    }
    add_neighbours(e, &mut x, c);
}

/// Step 1 of `calc_manifold` (`002976`-`002d12`).
fn check_entries(e: &mut Engine, x: &mut X87, c: Calc) {
    let Calc { f, t, m } = c;
    if e.mem.u8(m + 0x21) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        x.fld1();
        let entry = m + index * 4;
        let kind = u32::from(e.mem.u8(entry));
        e.mem.set_u32(f + 0x98, kind);
        let code = u32::from(e.mem.u8(entry + 1));
        if recheck_entry(e, x, c, entry, code) {
            index += 1;
        } else {
            if e.mem.u8(entry) <= 6 {
                let counted = e.mem.u8(m + 0x20).wrapping_sub(1);
                e.mem.set_u8(m + 0x20, counted);
            }
            let cell = e.mem.u32(t + 0x10);
            let other = e.mem.u32(cell + 4);
            let object = e.mem.u32(t + 0xc);
            let word = u32::from(e.mem.u16(entry + 2));
            e.vcall(object, 0x14, &args![word, other]);
            e.call(REMOVE_POINT, &args![m, index]);
        }
        if index >= u32::from(e.mem.u8(m + 0x21)) {
            break;
        }
    }
}

/// One entry of step 1: true (and a record written) when it is still good.
fn recheck_entry(e: &mut Engine, x: &mut X87, c: Calc, entry: u32, code: u32) -> bool {
    let Calc { f, t, .. } = c;
    let kind = e.mem.u32(f + 0x98);
    let sign = sign_bits(code);
    if kind <= 2 {
        x.fstp(e, f + 0x24);
        let extents = mul(
            load_vec(e, SIGN_ROWS + (code & 0x70)),
            load_vec(e, t + 0x70),
        );
        let point = transform_point(e, t + 0x20, extents);
        store_vec(e, f + 0x70, extents);
        store_vec(e, f + 0x60, point);
        let bits = e.mem.u32(f + 0x24) | sign;
        e.mem.set_u32(f + 0x24, bits);
        x.fld(e, f + 0x24);
        let good = inside(e, point, t + 0x90);
        x.fst(e, f + 0x90);
        if !good {
            x.pop();
            return false;
        }
        x.fld(e, f + 0x60 + kind * 4);
        x.fmul_st0(1);
        x.fsub(e, t + 0x60 + kind * 4);
        let record = take_record(e, t);
        let word = u32::from(e.mem.u16(entry + 2));
        x.fstp(e, f + 0x94);
        x.fchs();
        x.fstp(e, f + 0x24);
        let negated = e.mem.f32(f + 0x24);
        let row_kind = u32::from(e.mem.u8(entry));
        write_record(e, c, record, kind, row_kind, word, negated);
        true
    } else if kind <= 6 {
        x.fstp(e, f + 0x24);
        let extents = mul(
            load_vec(e, SIGN_ROWS + (code & 0x70)),
            load_vec(e, t + 0x60),
        );
        let offset = sub(extents, load_vec(e, t + 0x50));
        let rotated = rows_times_columns(e, t + 0x20, offset);
        store_vec(e, f + 0x70, rotated);
        let bits = e.mem.u32(f + 0x24) | sign;
        e.mem.set_u32(f + 0x24, bits);
        x.fld(e, f + 0x24);
        let good = inside(e, rotated, t + 0xa0);
        x.fst(e, f + 0x90);
        store_vec(e, f + 0x60, extents);
        if !good {
            x.pop();
            return false;
        }
        x.fchs();
        x.fstp(e, f + 0x24);
        x.fld(e, f + 0x60 + kind * 4);
        x.fld(e, f + 0x24);
        x.fld_st(0);
        x.fmulp(2);
        x.fld(e, t + 0x60 + kind * 4);
        let record = take_record(e, t);
        x.fsubp(2);
        let word = u32::from(e.mem.u16(entry + 2));
        x.fxch(1);
        x.fstp(e, f + 0x94);
        x.fstp(e, f + 0x24);
        let negated = e.mem.f32(f + 0x24);
        let row_kind = u32::from(e.mem.u8(entry));
        write_record(e, c, record, kind, row_kind, word, negated);
        true
    } else {
        x.pop();
        e.mem.set_u32(f + 0x9c, code);
        is_valid_edge_edge(e, Ptr::new(t), Ptr::new(f + 0x33), Ptr::new(f + 0x60));
        if e.mem.u8(f + 0x33) == 0 {
            return false;
        }
        let record = take_record(e, t);
        let word = u32::from(e.mem.u16(entry + 2));
        let row_kind = u32::from(e.mem.u8(entry));
        write_record(e, c, record, kind, row_kind, word, 0.0);
        true
    }
}

/// Step 2 of `calc_manifold` (`002d17`-`002e3a`): false when nothing more is to be done. The
/// byte at `f + 0x33` says whether the manifold counts as moved.
fn check_direction(e: &mut Engine, x: &mut X87, c: Calc) -> bool {
    let Calc { f, t, m } = c;
    if e.mem.u8(m + 0x21) < 2 {
        e.mem.set_u8(m + 0x23, 0);
        e.mem.set_u8(f + 0x33, 1);
        return true;
    }
    if e.mem.u8(m + 0x23) == 0 {
        e.call(TRANSFORM_VECTOR, &args![f + 0xc0, t + 0x20, m + 0x30]);
        e.mem.set_u8(m + 0x23, 1);
        let packed = e.call(PACK_DIRECTION, &args![f + 0xc0]).u32();
        e.mem.set_u32(m + 0x24, packed);
        e.mem.set_u8(f + 0x33, 1);
        return true;
    }
    let packed = e.mem.u32(m + 0x24);
    let bytes = [
        (packed & 0xff) as i32 - 0x80,
        ((packed >> 8) & 0xff) as i32 - 0x80,
        ((packed >> 0x10) & 0xff) as i32 - 0x80,
        (packed >> 0x18) as i32 - 0x80,
    ];
    x.fild(bytes[0]);
    x.fld64(e, DIRECTION_SCALE);
    x.fmul_into(1);
    x.fxch(1);
    x.fstp(e, f + 0x50);
    x.fild(bytes[1]);
    x.fmul_st0(1);
    x.fstp(e, f + 0x54);
    x.fild(bytes[2]);
    x.fmul_st0(1);
    x.fstp(e, f + 0x58);
    x.fimul(bytes[3]);
    x.fstp(e, f + 0x5c);
    let direction = transform_direction(e, t + 0x20, load_vec(e, f + 0x50));
    let along = sum3(mul(direction, load_vec(e, m + 0x30)));
    e.mem.set_f32(f + 0x24, along);
    x.fld(e, f + 0x24);
    let result = x.fcomp64(e, DIRECTION_LIMIT);
    let moved = result.c0 != result.c2;
    e.mem.set_u8(f + 0x33, u8::from(moved));
    if !moved && e.mem.u8(m + 0x22) != 0 {
        return false;
    }
    true
}

/// Step 3 of `calc_manifold` (`002e40`-`002f99`): false when `checkIntersection` finds the
/// boxes separated.
fn separate(e: &mut Engine, x: &mut X87, c: Calc) -> bool {
    let Calc { f, t, m } = c;
    let count = u32::from(e.mem.u8(m + 0x21));
    let tolerance = if count == 0 {
        t + 0x80
    } else {
        let records = e.mem.u32(f + 0x10);
        x.fld(e, records + 0x1c);
        x.fstp(e, f + 0x2c);
        for index in (1..count).rev() {
            x.fld(e, records + index * 0x30 + 0x1c);
            x.fstp(e, f + 0x28);
            x.fld(e, f + 0x2c);
            x.fld(e, f + 0x28);
            let result = x.fcom(1);
            x.fstp_st(1);
            if result.c0 || result.c3 {
                x.fstp(e, f + 0x2c);
            } else {
                x.pop();
            }
        }
        x.fld(e, f + 0x2c);
        x.fld(e, t + 0x80);
        x.fmul64(e, MINIMUM_SCALE);
        x.fsubp(1);
        x.fstp(e, f + 0x2c);
        let lowered = movss(e, f + 0x2c);
        store_vec(e, f + 0xc0, splat(lowered, 0));
        f + 0xc0
    };
    check_intersection(e, Ptr::new(t), Ptr::new(tolerance)) == 0
}

/// Step 4 of `calc_manifold` (`002fa1`-`0031dc`): returns the result of `findClosestPoint`
/// (the key is at `f + 0x2c`, its record at `f + 0x60`).
fn find_closest(e: &mut Engine, x: &mut X87, c: Calc) -> u32 {
    let Calc { f, t, m } = c;
    e.mem.set_u16(f + 0x2e, 0);
    store_vec(e, f + 0xc0, [0.0; 4]);
    store_vec(e, f + 0x80, [0.0; 4]);
    let result = find_closest_point(
        e,
        Ptr::new(t),
        Ptr::new(m),
        Ptr::new(f + 0x2c),
        Ptr::new(f + 0x60),
    );
    x.fld(e, f + 0x90);
    let kind = e.mem.u32(f + 0x98) as i32;
    e.mem.set_u32(f + 0x24, result);
    if e.mem.u8(f + 0x33) == 0 || result == 0 {
        x.pop();
        return result;
    }
    let normal = m + 0x30;
    let axis = u32::from(e.mem.u8(f + 0x2c));
    if kind <= 2 {
        x.fchs();
        store_vec(e, normal, load_vec(e, f + 0xc0));
        x.fstp(e, normal + axis * 4);
    } else if kind <= 6 {
        x.fchs();
        x.fstp(e, f + 0x10);
        let negated = movss(e, f + 0x10);
        let row = load_vec(e, t.wrapping_add(axis.wrapping_sub(2) << 4));
        store_vec(e, normal, mul(splat(negated, 0), row));
    } else {
        x.pop();
        if result == 2 {
            store_vec(e, normal, load_vec(e, f + 0x80));
        }
    }
    if e.mem.u8(m + 0x23) != 0 {
        update_direction(e, x, c);
    }
    result
}

/// The part of step 4 that runs when the manifold has a direction (`003055`-`0031d6`): the
/// normal is rotated into the first transform's frame (`00c7f700`), the records that no longer
/// point within 0.95 of it are replaced by the last record, and the direction is made again.
fn update_direction(e: &mut Engine, x: &mut X87, c: Calc) {
    let Calc { f, t, m } = c;
    let matrix = e.mem.u32(t + 0x14);
    e.call(SET_ROTATED_DIR, &args![f + 0x50, matrix, m + 0x30]);
    let cell = e.mem.u32(t + 0x10);
    let kept = u32::from(e.mem.u8(m + 0x21));
    let current = e.mem.u32(cell);
    let slots =
        divide_by_48(current.wrapping_sub(cell).wrapping_sub(0x10)).wrapping_sub(kept as i32);
    let mut offset = (slots as u32).wrapping_mul(3) << 4;
    let mut index = 0u32;
    if kept > 0 {
        e.mem.set_u32(f + 0x10, offset);
        loop {
            let cell = e.mem.u32(t + 0x10);
            let normal = load_vec(e, offset.wrapping_add(cell).wrapping_add(0x20));
            let along = sum3(mul(normal, load_vec(e, f + 0x50)));
            e.mem.set_f32(f + 0x48, along);
            x.fld(e, f + 0x48);
            let result = x.fcomp64(e, DIRECTION_LIMIT);
            if result.c0 == result.c2 {
                index += 1;
                offset = offset.wrapping_add(0x30);
                e.mem.set_u32(f + 0x10, offset);
            } else {
                let cell = e.mem.u32(t + 0x10);
                let last = e.mem.u32(cell).wrapping_sub(0x30);
                let slot = cell.wrapping_add(offset);
                store_vec(e, slot + 0x10, load_vec(e, last));
                store_vec(e, slot + 0x20, load_vec(e, last + 0x10));
                let word = e.mem.u32(last + 0x20);
                e.mem.set_u32(slot + 0x30, word);
                if e.mem.u8(m + index * 4) <= 6 {
                    let counted = e.mem.u8(m + 0x20).wrapping_sub(1);
                    e.mem.set_u8(m + 0x20, counted);
                }
                let other = e.mem.u32(e.mem.u32(t + 0x10) + 4);
                let object = e.mem.u32(t + 0xc);
                let id = u32::from(e.mem.u16(m + index * 4 + 2));
                e.vcall(object, 0x14, &args![id, other]);
                e.call(REMOVE_POINT, &args![m, index]);
                let cell = e.mem.u32(t + 0x10);
                let moved_back = e.mem.u32(cell).wrapping_sub(0x30);
                e.mem.set_u32(cell, moved_back);
                offset = e.mem.u32(f + 0x10);
            }
            if index >= u32::from(e.mem.u8(m + 0x21)) {
                break;
            }
        }
    }
    if e.mem.u8(m + 0x21) < 2 {
        e.mem.set_u8(m + 0x23, 0);
    } else {
        let rotated = rows_times_columns(e, t + 0x20, load_vec(e, m + 0x30));
        store_vec(e, f + 0xb0, rotated);
        let packed = e.call(PACK_DIRECTION, &args![f + 0xb0]).u32();
        e.mem.set_u32(m + 0x24, packed);
        e.mem.set_u8(m + 0x23, 1);
    }
}

/// Step 5 of `calc_manifold` (`0031de`-`003558`): adds the closest point (key at `f + 0x2c`);
/// false when the function ends there.
fn add_closest_point(e: &mut Engine, x: &mut X87, c: Calc) -> bool {
    let Calc { f, t, m } = c;
    let kind_byte = e.mem.u8(f + 0x2c);
    x.fld64(e, EPSILON);
    if kind_byte <= 6 {
        let point = if kind_byte <= 2 {
            transform_point(e, e.mem.u32(t + 0x18), load_vec(e, f + 0x70))
        } else {
            transform_point(e, e.mem.u32(t + 0x14), load_vec(e, f + 0x60))
        };
        if near_existing(e, x, c, point, (f + 0x10, f + 0x24)) {
            return false;
        }
    }
    x.pop();
    if e.mem.u8(m + 0x21) >= 8 {
        return false;
    }
    let id = add_point(e, c, f + 0x2c);
    e.mem.set_u32(f + 0x34, id);
    if (id as i32) < 0 {
        return true;
    }
    let cell = e.mem.u32(t + 0x10);
    let record = e.mem.u32(cell);
    let kind = e.mem.u32(f + 0x98);
    let mut negated = 0.0;
    if (kind as i32) <= 6 {
        x.fld(e, f + 0x90);
        x.fchs();
        x.fstp(e, f + 0x10);
        negated = e.mem.f32(f + 0x10);
    }
    let row_kind = u32::from(e.mem.u8(f + 0x2c));
    let word = u32::from(e.mem.u16(f + 0x2e));
    write_record(e, c, record, kind, row_kind, word, negated);
    let point = NewPoint {
        id,
        record,
        key: f + 0x2c,
        dot_slot: f + 0x10,
        update_word: true,
    };
    complete_point(e, x, c, point)
}

/// Step 6 of `calc_manifold` (`003558`-`004165`).
fn add_neighbours(e: &mut Engine, x: &mut X87, c: Calc) {
    let Calc { f, t, m } = c;
    let key = e.mem.u32(f + 0x2c);
    e.mem.set_u32(f + 0x3c, key);
    let count = u32::from(e.mem.u8(m + 0x21));
    if count < 1 {
        return;
    }
    let cell = e.mem.u32(t + 0x10);
    e.mem.set_u32(f + 0x38, count);
    let current = e.mem.u32(cell);
    let slots =
        divide_by_48(current.wrapping_sub(cell).wrapping_sub(0x10)).wrapping_sub(count as i32);
    let offset = (slots as u32).wrapping_mul(3) << 4;
    let normal = load_vec(e, cell.wrapping_add(offset).wrapping_add(0x20));
    let rotated_first = rows_times_columns(e, e.mem.u32(t + 0x14), normal);
    let rotated_second = rows_times_columns(e, e.mem.u32(t + 0x18), normal);
    let flipped = xorps(rotated_first, load_vec(e, SIGN_MASK));
    let abs = load_vec(e, ABS_MASK);
    store_vec(e, f + 0xb0, andps(flipped, abs));
    x.fld(e, f + 0xb0);
    x.fld(e, f + 0xb4);
    store_vec(e, f + 0x50, andps(rotated_second, abs));
    let result = x.fcompp();
    let (mut best_first, first_mask) = if result.c0 != result.c2 {
        (0, 0x10)
    } else {
        (1, 0x20)
    };
    e.mem.set_u32(f + 0x24, first_mask);
    x.fld(e, f + 0xb8);
    e.mem.set_u32(f + 0x40, best_first);
    x.fld(e, f + 0xb0 + best_first * 4);
    let result = x.fcompp();
    if result.c0 != result.c2 {
        best_first = 2;
        e.mem.set_u32(f + 0x40, best_first);
        e.mem.set_u32(f + 0x24, 0x40);
    }
    x.fld(e, f + 0x50);
    x.fld(e, f + 0x54);
    let result = x.fcompp();
    let (mut best_second, second_mask) = if result.c0 != result.c2 {
        (0, 0x10)
    } else {
        (1, 0x20)
    };
    e.mem.set_u32(f + 0x48, second_mask);
    x.fld(e, f + 0x58);
    x.fld(e, f + 0x50 + best_second * 4);
    let result = x.fcompp();
    if result.c0 != result.c2 {
        best_second = 2;
        e.mem.set_u32(f + 0x48, 0x40);
    }
    x.fld(e, f + 0xb0 + best_first * 4);
    x.fld64(e, ALIGNED_LIMIT);
    let result = x.fcom(1);
    x.fstp_st(1);
    if !result.c0 && !result.c3 {
        return;
    }
    x.fld(e, f + 0x50 + best_second * 4);
    let result = x.fcompp();
    if result.c0 != result.c2 {
        return;
    }
    x.fld(e, t + 0xc0 + best_first * 4);
    let tolerance = load_vec(e, f + 0xc0);
    x.fstp(e, f + 0x10);
    let bits_first = e.mem.u32(f + 0x10);
    x.fld(e, t + 0xd0 + best_second * 4);
    e.mem.set_u16(f + 0x4e, 0);
    x.fstp(e, f + 0x10);
    x.fld1();
    let first_code = feature_byte(movmskps(cmpltps(rotated_second, tolerance)), bits_first);
    let bits_second = e.mem.u32(f + 0x10);
    e.mem.set_u8(f + 0x3d, first_code);
    let second_code = feature_byte(movmskps(cmpltps(flipped, tolerance)), bits_second);
    e.mem.set_u8(f + 0x4d, second_code);
    let first_kind = e.mem.u8(f + 0x40);
    x.fstp(e, f + 0x40);
    e.mem.set_u8(f + 0x3c, first_kind);
    e.mem.set_u8(f + 0x4c, best_second as u8 + 4);
    let mut address = cell.wrapping_add(offset).wrapping_add(0x2c);
    for _ in 0..e.mem.u8(m + 0x21) {
        x.fld(e, address);
        x.fstp(e, f + 0x10);
        x.fld(e, f + 0x10);
        x.fld(e, f + 0x40);
        let result = x.fcomp(1);
        if result.c0 || result.c3 {
            x.pop();
        } else {
            x.fstp(e, f + 0x40);
        }
        address = address.wrapping_add(0x30);
    }
    let mut word_address = NEIGHBOUR_MASKS;
    while word_address < NEIGHBOUR_MASKS_END {
        let word = u32::from(e.mem.u16(word_address));
        add_first_neighbour(e, x, c, word);
        word_address += 2;
    }
    let mut word_address = NEIGHBOUR_MASKS;
    while word_address < NEIGHBOUR_MASKS_END {
        let word = u32::from(e.mem.u16(word_address));
        add_second_neighbour(e, x, c, word);
        word_address += 2;
    }
    if e.mem.u8(m + 0x20) >= 4 {
        e.mem.set_u8(m + 0x22, 1);
        return;
    }
    let (first, second) = (e.mem.u32(f + 0x24), e.mem.u32(f + 0x48));
    check_completeness(e, Ptr::new(m), first, second);
}

/// One word of the first neighbour loop of step 6 (`003818`-`003c83`): the first feature
/// (`f + 0x3c`), changed by the word, as a point of the second box.
fn add_first_neighbour(e: &mut Engine, x: &mut X87, c: Calc, word: u32) {
    let Calc { f, t, m } = c;
    if (e.mem.u32(f + 0x48) & word) as u16 != 0 {
        return;
    }
    let entry = e.mem.u32(f + 0x3c);
    let mixed = word as u8 ^ e.mem.u8(f + 0x3d);
    e.mem.set_u32(f + 0x28, entry);
    let kind = entry as u8;
    let code = (((entry >> 8) as u8) & 0xf) | mixed;
    let count = u32::from(e.mem.u8(m + 0x21));
    e.mem.set_u32(f + 0x38, count);
    e.mem.set_u8(f + 0x29, code);
    if has_entry(e, m, kind, code) {
        return;
    }
    x.fld1();
    x.fstp(e, f + 0x34);
    let extents = mul(
        load_vec(e, SIGN_ROWS + (u32::from(code) & 0x70)),
        load_vec(e, t + 0x70),
    );
    let point = transform_point(e, t + 0x20, extents);
    store_vec(e, f + 0x70, extents);
    let bits = e.mem.u32(f + 0x34) | sign_bits(u32::from(code));
    e.mem.set_u32(f + 0x34, bits);
    x.fld(e, f + 0x34);
    let good = inside(e, point, t + 0x90);
    x.fst(e, f + 0x90);
    e.mem.set_u32(f + 0x98, u32::from(kind));
    store_vec(e, f + 0xd0, splat(extents, 1));
    store_vec(e, f + 0xc0, splat(extents, 0));
    store_vec(e, f + 0x50, splat(extents, 2));
    store_vec(e, f + 0x60, point);
    if !good {
        x.pop();
        return;
    }
    x.fmul(e, f + u32::from(kind) * 4 + 0x60);
    x.fsub(e, t + u32::from(kind) * 4 + 0x60);
    x.fstp(e, f + 0x94);
    x.fld(e, f + 0x94);
    x.fld(e, f + 0x40);
    x.fld64(e, EPSILON);
    x.fsub_into(1);
    x.fxch(2);
    if x.fcompp().c0 {
        x.pop();
        return;
    }
    let kind_byte = e.mem.u8(f + 0x28);
    if kind_byte <= 6 {
        let other = if kind_byte <= 2 {
            transform_point(e, e.mem.u32(t + 0x18), extents)
        } else {
            transform_point(e, e.mem.u32(t + 0x14), point)
        };
        if near_existing(e, x, c, other, (f + 0x44, f + 0x38)) {
            x.pop();
            return;
        }
    }
    x.pop();
    if e.mem.u8(m + 0x21) >= 8 {
        return;
    }
    let id = add_point(e, c, f + 0x28);
    e.mem.set_u32(f + 0x38, id);
    if (id as i32) < 0 {
        return;
    }
    let cell = e.mem.u32(t + 0x10);
    let record = e.mem.u32(cell);
    let kind = e.mem.u32(f + 0x98);
    let mut negated = 0.0;
    if (kind as i32) <= 6 {
        x.fld(e, f + 0x34);
        x.fchs();
        x.fstp(e, f + 0x34);
        negated = e.mem.f32(f + 0x34);
    }
    let row_kind = u32::from(e.mem.u8(f + 0x28));
    let word = u32::from(e.mem.u16(f + 0x2a));
    write_record(e, c, record, kind, row_kind, word, negated);
    let point = NewPoint {
        id,
        record,
        key: f + 0x28,
        dot_slot: f + 0xac,
        update_word: false,
    };
    complete_point(e, x, c, point);
}

/// One word of the second neighbour loop of step 6 (`003ca1`-`00412a`): the second feature
/// (`f + 0x4c`), changed by the word, as a point of the first box.
fn add_second_neighbour(e: &mut Engine, x: &mut X87, c: Calc, word: u32) {
    let Calc { f, t, m } = c;
    if (e.mem.u32(f + 0x24) & word) as u16 != 0 {
        return;
    }
    let entry = e.mem.u32(f + 0x4c);
    let mixed = word as u8 ^ e.mem.u8(f + 0x4d);
    e.mem.set_u32(f + 0x28, entry);
    let code = (((entry >> 8) as u8) & 0xf) | mixed;
    e.mem.set_u8(f + 0x29, code);
    let found = e
        .call(FIND_IN_MANIFOLD, &args![m, f + 0x33, f + 0x28])
        .u32();
    if e.mem.u8(found) != 0 {
        return;
    }
    x.fld1();
    x.fstp(e, f + 0x10);
    let kind = u32::from(e.mem.u8(f + 0x28));
    let extents = mul(
        load_vec(e, SIGN_ROWS + (u32::from(code) & 0x70)),
        load_vec(e, t + 0x60),
    );
    let offset = sub(extents, load_vec(e, t + 0x50));
    let rotated = rows_times_columns(e, t + 0x20, offset);
    let bits = e.mem.u32(f + 0x10) | sign_bits(u32::from(code));
    e.mem.set_u32(f + 0x10, bits);
    x.fld(e, f + 0x10);
    let good = inside(e, rotated, t + 0xa0);
    x.fst(e, f + 0x90);
    e.mem.set_u32(f + 0x98, kind);
    store_vec(e, f + 0x60, extents);
    store_vec(e, f + 0x70, rotated);
    if !good {
        x.pop();
        return;
    }
    x.fchs();
    x.fstp(e, f + 0x10);
    x.fld(e, f + kind * 4 + 0x60);
    x.fmul(e, f + 0x10);
    x.fsub(e, t + kind * 4 + 0x60);
    x.fstp(e, f + 0x94);
    x.fld(e, f + 0x94);
    x.fld(e, f + 0x40);
    x.fld64(e, EPSILON);
    x.fsub_into(1);
    x.fxch(2);
    if x.fcompp().c0 {
        x.pop();
        return;
    }
    let kind_byte = e.mem.u8(f + 0x28);
    if kind_byte <= 6 {
        let other = if kind_byte <= 2 {
            transform_point(e, e.mem.u32(t + 0x18), rotated)
        } else {
            transform_point(e, e.mem.u32(t + 0x14), extents)
        };
        if near_existing(e, x, c, other, (f + 0xac, f + 0x44)) {
            x.pop();
            return;
        }
    }
    x.pop();
    if e.mem.u8(m + 0x21) >= 8 {
        return;
    }
    let id = add_point(e, c, f + 0x28);
    e.mem.set_u32(f + 0x38, id);
    if (id as i32) < 0 {
        return;
    }
    let cell = e.mem.u32(t + 0x10);
    let record = e.mem.u32(cell);
    let kind = e.mem.u32(f + 0x98);
    let mut negated = 0.0;
    if (kind as i32) <= 6 {
        x.fld(e, f + 0x10);
        x.fstp(e, f + 0x44);
        negated = e.mem.f32(f + 0x44);
    }
    let row_kind = u32::from(e.mem.u8(f + 0x28));
    let word = u32::from(e.mem.u16(f + 0x2a));
    write_record(e, c, record, kind, row_kind, word, negated);
    let point = NewPoint {
        id,
        record,
        key: f + 0x28,
        dot_slot: f + 0x34,
        update_word: false,
    };
    complete_point(e, x, c, point);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db_0b00, check_completeness(Ptr, u32, u32)),
        entry!(0x00db_0dc0, fn_00db0dc0(Ptr, Ptr, Ptr, u32, u32)),
        entry!(0x00db_0e50, fn_00db0e50(Ptr, u32) -> u32),
        entry!(0x00db_0ec0, check_intersection(Ptr, Ptr) -> u32),
        entry!(0x00db_15a0, is_valid_edge_edge(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00db_1990, add_additional_edge_helper(Ptr, Ptr, Ptr, f32)),
        entry!(
            0x00db_1d90,
            fn_00db1d90(Ptr, Ptr, u32, u32, u32, u32, Ptr, Ptr, f32)
        ),
        entry!(0x00db_1ed0, find_closest_point(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00db_2750, calculate_closest_point(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00db_2960, calc_manifold(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::hkpsphereshape::{HALF_F32, THREE_F32};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The argument words of every call a double saw.
    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    /// The virtual function `+0xc` of the object the tests hang on the detection state.
    const CREATE_POINT: u32 = 0x00ee_0000;
    const USED_UP_BITS: u32 = 0xff7f_ffee;
    const FILLER_BITS: u32 = 0xdbb1_a2bc;

    /// An engine with the exe's data range, the constants the functions read and the CRT
    /// square root.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, f64::from(f32::EPSILON));
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(LENGTH_LIMIT, f32::from_bits(0x4479_ffff));
        e.mem.set_u32(LENGTH_FILL, FILLER_BITS);
        e.set_global(MINUS_TWO, -2.0f64);
        e.set_global(FOUR, 4.0f64);
        e.set_global(NEARLY_ONE, f64::from_bits(0x3fef_ffff_c000_0000));
        e.set_global(HALF, 0.5f64);
        e.set_global(DIRECTION_SCALE, f64::from_bits(0x3f81_9999_a000_0000));
        e.set_global(DIRECTION_LIMIT, f64::from_bits(0x3fee_6666_6000_0000));
        e.set_global(MINIMUM_SCALE, f64::from_bits(0x3fa9_9999_a000_0000));
        e.set_global(ALIGNED_LIMIT, f64::from_bits(0x3fef_5c29_0000_0000));
        let masks: [u16; 7] = [0x50, 0x30, 0x60, 0x10, 0x20, 0x40, 0x00];
        for (index, word) in masks.into_iter().enumerate() {
            e.mem.set_u16(NEIGHBOUR_MASKS + 2 * index as u32, word);
        }
        e.set_global(THRESHOLD_SCALE, f64::from_bits(0x3fef_f7ce_e000_0000));
        e.mem.set_u32(SMALL_POSITIVE, 0x3727_c5ac);
        e.mem.set_u32(SMALL_NEGATIVE, 0xb727_c5ac);
        e.mem.set_u32(USED_UP, USED_UP_BITS);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        for row in 0..8u32 {
            let sign = |bit: u32| if row & bit != 0 { -1.0 } else { 1.0 };
            store_vec(
                &mut e,
                SIGN_ROWS + 0x10 * row,
                [sign(1), sign(2), sign(4), 0.0],
            );
        }
        e.register_double(X87_SQRT, |_, a| {
            let mut next = 0;
            Ret {
                st0: f64::take(a, &mut next).sqrt(),
                ..Ret::default()
            }
        });
        e
    }

    /// A detection state whose rotation between the boxes is the identity.
    fn state(e: &mut Engine) -> u32 {
        let t = e.mem.alloc(0x180);
        for axis in 0..3u32 {
            let mut row = [0.0f32; 4];
            row[axis as usize] = 1.0;
            store_vec(e, t + 0x20 + 0x10 * axis, row);
        }
        t
    }

    /// The scene most tests use: both boxes with half extents 1, the second box at
    /// (0.2, 0.3, 3) in the first box's frame, wide tolerance extents and the thresholds
    /// 2.0 (`+0xb0`) and 0.01 (`+0xb4`).
    fn scene(e: &mut Engine) -> u32 {
        let t = state(e);
        store_vec(e, t + 0x50, [0.2, 0.3, 3.0, 0.0]);
        store_vec(e, t + 0x60, [1.0, 1.0, 1.0, 0.0]);
        store_vec(e, t + 0x70, [1.0, 1.0, 1.0, 0.0]);
        store_vec(e, t + 0x90, [10.0; 4]);
        store_vec(e, t + 0xa0, [10.0; 4]);
        e.mem.set_f32(t + 0xb0, 2.0);
        e.mem.set_f32(t + 0xb4, 0.01);
        store_vec(e, t + 0xc0, [0.5, -1.0, 1.5, 0.0]);
        store_vec(e, t + 0xd0, [0.1, 0.2, 0.3, 0.0]);
        t
    }

    /// A transform: the identity rotation and `translation` as the fourth row.
    fn transform(e: &mut Engine, translation: Vec4) -> u32 {
        let matrix = e.mem.alloc(0x40);
        for axis in 0..3u32 {
            let mut row = [0.0f32; 4];
            row[axis as usize] = 1.0;
            store_vec(e, matrix + 0x10 * axis, row);
        }
        store_vec(e, matrix + 0x30, translation);
        matrix
    }

    /// A manifold block holding `entries` (kind, code), with its completeness byte at 0xaa.
    fn new_manifold(e: &mut Engine, entries: &[[u8; 2]]) -> u32 {
        let manifold = e.mem.alloc(0x30);
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u8(manifold + 4 * index as u32, entry[0]);
            e.mem.set_u8(manifold + 4 * index as u32 + 1, entry[1]);
        }
        e.mem.set_u8(manifold + 0x21, entries.len() as u8);
        e.mem.set_u8(manifold + 0x22, 0xaa);
        manifold
    }

    fn expect_vec(e: &Engine, address: u32, expected: Vec4) {
        let actual = load_vec(e, address);
        for (lane, (a, x)) in actual.iter().zip(expected).enumerate() {
            assert!(
                (a - x).abs() < 1e-4,
                "lane {lane} at {address:#x}: {actual:?} != {expected:?}"
            );
        }
    }

    fn expect_float(e: &Engine, address: u32, expected: f32) {
        let actual = e.mem.f32(address);
        assert!(
            (actual - expected).abs() < 1e-4,
            "{address:#x}: {actual} != {expected}"
        );
    }

    /// Sets every candidate separation to -5 (the limit at `+0x128` to -9), then the given ones.
    fn separations(e: &mut Engine, t: u32, values: &[(u32, f32)]) {
        for (offset, _) in FEATURES {
            e.mem.set_f32(t + offset, -5.0);
        }
        e.mem.set_f32(t + 0x128, -9.0);
        for (offset, value) in values {
            e.mem.set_f32(t + offset, *value);
        }
    }

    #[test]
    fn check_completeness_counts_edges_and_faces_in_pairs() {
        let mut e = engine();
        let run = |e: &mut Engine, entries: &[[u8; 2]], points: u8, first: u32| -> u8 {
            let m = new_manifold(e, entries);
            e.mem.set_u8(m + 0x20, points);
            check_completeness(e, Ptr::new(m), first, 0);
            e.mem.u8(m + 0x22)
        };
        // Fewer than three entries leave the byte alone.
        assert_eq!(run(&mut e, &[[0, 0], [0, 0]], 0, 0), 0xaa);
        // Four counted contact points complete it.
        assert_eq!(run(&mut e, &[[0, 0], [0, 0], [0, 0]], 4, 0), 1);
        // Faces counted twice by kinds below 3 complete it ...
        assert_eq!(run(&mut e, &[[0, 0], [0, 0], [0, 0]], 0, 0), 1);
        // ... a third face with another side does not ...
        assert_eq!(run(&mut e, &[[0, 0], [0, 0], [0, 0x10]], 0, 0), 0);
        // ... and a kind from 3 to 6 counts into the other mask, so one of them is not a pair.
        assert_eq!(run(&mut e, &[[0, 0], [0, 0], [3, 0]], 0, 0), 0);
        assert_eq!(run(&mut e, &[[3, 0], [3, 0], [3, 0]], 0, 0), 1);
        // Edges (kinds above 6) count their own edge and the edge of their code.
        assert_eq!(run(&mut e, &[[7, 0], [7, 0], [7, 0]], 0, 0), 1);
        assert_eq!(run(&mut e, &[[7, 0], [7, 0], [7, 1]], 0, 0), 0);
    }

    #[test]
    fn fn_00db0dc0_packs_signs_and_axes_of_both_vectors() {
        let mut e = engine();
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        store_vec(&mut e, a, [-1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, b, [1.0, -2.0, 5.0, 0.0]);
        fn_00db0dc0(&mut e, Ptr::new(b), Ptr::new(out), Ptr::new(a), 1, 2);
        // a becomes (-1, 1, 3): lane 0 negative -> ((1 | 0xf8) << 4) | 1.
        assert_eq!(e.mem.u8(out), 0x91);
        // b becomes (1, -2, 1): lane 1 negative -> (!2 & 7) << 4, plus 2.
        assert_eq!(e.mem.u8(out + 1), 0x52);
        // The axis 1.0 overrides a negative lane.
        store_vec(&mut e, a, [0.5, -2.0, 3.0, -1.0]);
        fn_00db0dc0(&mut e, Ptr::new(b), Ptr::new(out), Ptr::new(a), 1, 1);
        assert_eq!(e.mem.u8(out), 0x81);
        assert_eq!(e.mem.u8(out + 1), 0x71);
    }

    #[test]
    fn fn_00db0e50_packs_sign_bits_and_the_side_flag() {
        let mut e = engine();
        let v = e.mem.alloc(0x10);
        store_vec(&mut e, v, [-1.0, 1.0, -1.0, -5.0]);
        assert_eq!(fn_00db0e50(&mut e, Ptr::new(v), 0), 0x50);
        assert_eq!(fn_00db0e50(&mut e, Ptr::new(v), 0xbf80_0000), 0x58);
        assert_eq!(fn_00db0e50(&mut e, Ptr::new(v), 0x3f80_0000), 0x50);
        // NaN is not below 0.0.
        store_vec(&mut e, v, [f32::NAN, -0.0, -3.0, 0.0]);
        assert_eq!(fn_00db0e50(&mut e, Ptr::new(v), 0), 0x40);
    }

    /// The overlap scene for `check_intersection`: identity rotation, half extents (1, 2, 3)
    /// and (0.25, 0.5, 0.75).
    fn overlap(e: &mut Engine) -> (u32, u32) {
        let t = state(e);
        store_vec(e, t + 0x60, [1.0, 2.0, 3.0, 0.0]);
        store_vec(e, t + 0x70, [0.25, 0.5, 0.75, 0.0]);
        store_vec(e, t + 0xc0, [0.5, -1.0, 1.5, 0.0]);
        store_vec(e, t + 0xd0, [0.1, 0.2, 0.3, 0.0]);
        let tolerance = e.mem.alloc(0x10);
        (t, tolerance)
    }

    #[test]
    fn check_intersection_stores_the_separations_of_overlapping_boxes() {
        let mut e = engine();
        let (t, tolerance) = overlap(&mut e);
        assert_eq!(
            check_intersection(&mut e, Ptr::new(t), Ptr::new(tolerance)),
            0
        );
        expect_vec(&e, t + 0xe0, [-0.75, -1.5, -2.25, 0.0]);
        expect_vec(&e, t + 0xf0, [-1.15, -2.3, -3.45, 0.0]);
        // With the identity rotation each edge pair is an axis (or parallel, which stores the
        // fill value): the pair of axes i and j separates like the third axis.
        let (fill, x, y, z) = (f32::from_bits(FILLER_BITS), -0.75, -1.5, -2.25);
        for (offset, expected) in [
            (0x100, fill),
            (0x104, z),
            (0x108, y),
            (0x110, z),
            (0x114, fill),
            (0x118, x),
            (0x120, y),
            (0x124, x),
            (0x128, fill),
        ] {
            let actual = e.mem.f32(t + offset);
            if expected == fill {
                assert_eq!(actual.to_bits(), FILLER_BITS, "{offset:#x}");
            } else {
                assert!((actual - expected).abs() < 1e-4, "{offset:#x}: {actual}");
            }
        }
        for offset in [0x10c, 0x11c, 0x12c] {
            assert_eq!(e.mem.f32(t + offset), 0.0, "{offset:#x}");
        }
    }

    #[test]
    fn check_intersection_stops_at_the_first_separating_axis() {
        // The box faces use the tolerance argument, the edge pairs the vector at `+0x80`.
        let outcome = |tune: &dyn Fn(&mut Engine, u32, u32)| {
            let mut e = engine();
            let (t, tolerance) = overlap(&mut e);
            tune(&mut e, t, tolerance);
            let result = check_intersection(&mut e, Ptr::new(t), Ptr::new(tolerance));
            (result, e, t)
        };
        // First box: the x separation is above the tolerance; nothing is stored.
        let (result, e, t) = outcome(&|e, t, _| store_vec(e, t + 0xc0, [5.0, -1.0, 1.5, 0.0]));
        assert_eq!(result, 1);
        assert_eq!(e.mem.f32(t + 0xe0), 0.0);
        // Second box: the first box's separations are stored, the second's are not.
        let (result, e, t) = outcome(&|e, t, _| store_vec(e, t + 0xd0, [5.0, 0.2, 0.3, 0.0]));
        assert_eq!(result, 1);
        expect_vec(&e, t + 0xe0, [-0.75, -1.5, -2.25, 0.0]);
        assert_eq!(e.mem.f32(t + 0xf0), 0.0);
        // First group of edge pairs: lane 1 of `+0x80` is below the -2.25 of the pair.
        let (result, e, t) = outcome(&|e, t, _| store_vec(e, t + 0x80, [0.0, -2.3, 0.0, 0.0]));
        assert_eq!(result, 1);
        expect_vec(&e, t + 0xf0, [-1.15, -2.3, -3.45, 0.0]);
        assert_eq!(e.mem.f32(t + 0x100), 0.0);
        // Second group: lane 2 is below -0.75 but not below the first group's -1.5.
        let (result, e, t) = outcome(&|e, t, _| store_vec(e, t + 0x80, [0.0, 0.0, -0.8, 0.0]));
        assert_eq!(result, 1);
        assert_eq!(e.mem.u32(t + 0x100), FILLER_BITS);
        assert_eq!(e.mem.f32(t + 0x110), 0.0);
        // Third group: a parallel pair compares as 0 (only its stored value is the fill), so
        // lane 2 at -0.5 is below it but not below the pairs of the first two groups.
        let (result, e, t) = outcome(&|e, t, _| store_vec(e, t + 0x80, [0.0, 0.0, -0.5, 0.0]));
        assert_eq!(result, 1);
        expect_float(&e, t + 0x110, -2.25);
        assert_eq!(e.mem.f32(t + 0x120), 0.0);
    }

    /// The edge pair of the scene: the x edge of the first box (corner (1, 1, 1)) and the y
    /// edge of the second (corner (-1, -1, -1) of its box), which cross at
    /// (-0.8, 1, 1) / (-0.8, 1, 2).
    fn edge_info(e: &mut Engine) -> u32 {
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x38, 0x00);
        e.mem.set_u32(info + 0x3c, 0x71);
        info
    }

    #[test]
    fn is_valid_edge_edge_accepts_crossing_edges_only() {
        let mut e = engine();
        let t = scene(&mut e);
        let info = edge_info(&mut e);
        let valid = e.mem.alloc(4);
        e.mem.set_u8(valid, 0xaa);
        is_valid_edge_edge(&mut e, Ptr::new(t), Ptr::new(valid), Ptr::new(info));
        assert_eq!(e.mem.u8(valid), 1);
        // The point on the first edge, the distance and the negated normal.
        expect_vec(&e, info, [-0.8, 1.0, 1.0, 0.0]);
        expect_float(&e, info + 0x34, 1.0);
        expect_vec(&e, info + 0x20, [0.0, 0.0, -1.0, 0.0]);
        // The distance above the limit at `+0xb0` rejects it and leaves the record alone.
        e.mem.set_f32(t + 0xb0, 0.5);
        let info = edge_info(&mut e);
        is_valid_edge_edge(&mut e, Ptr::new(t), Ptr::new(valid), Ptr::new(info));
        assert_eq!(e.mem.u8(valid), 0);
        assert_eq!(e.mem.f32(info + 0x34), 0.0);
        // Parallel edges (both along x) have a zero determinant.
        e.mem.set_f32(t + 0xb0, 2.0);
        e.mem.set_u32(info + 0x3c, 0x70);
        is_valid_edge_edge(&mut e, Ptr::new(t), Ptr::new(valid), Ptr::new(info));
        assert_eq!(e.mem.u8(valid), 0);
        // A second box far to the side leaves the edge parameter outside its range.
        let far = scene(&mut e);
        store_vec(&mut e, far + 0x50, [3.2, 0.3, 3.0, 0.0]);
        let info = edge_info(&mut e);
        is_valid_edge_edge(&mut e, Ptr::new(far), Ptr::new(valid), Ptr::new(info));
        assert_eq!(e.mem.u8(valid), 0);
    }

    /// 00db172f is `DC C9` (`FMUL ST(1),ST(0)`): the numerator of the parameter along the first
    /// edge is compared with `0.99999988 * determinant`, not with the determinant. Derived from
    /// the exe: the scene's edges cross at `x = -1 + dx` (`dx` the second box's x translation),
    /// so the parameter along the first edge (from the corner (1, 1, 1), length 2) is
    /// `(2 - dx) / 2 = 1 - dx / 2`, its numerator `16 (1 - dx / 2)` against the determinant 16;
    /// the edge is valid only when `numerator < 0.99999988 * 16`, that is `dx / 2 > 2^-23`.
    #[test]
    fn is_valid_edge_edge_compares_the_parameter_with_nearly_one() {
        let verdict = |dx: f32| {
            let mut e = engine();
            let t = scene(&mut e);
            store_vec(&mut e, t + 0x50, [dx, 0.3, 3.0, 0.0]);
            let info = edge_info(&mut e);
            let valid = e.mem.alloc(4);
            e.mem.set_u8(valid, 0xaa);
            is_valid_edge_edge(&mut e, Ptr::new(t), Ptr::new(valid), Ptr::new(info));
            e.mem.u8(valid)
        };
        // dx = 2^-21: the parameter is 1 - 2^-22, below 0.99999988 (1 - 2^-23).
        assert_eq!(verdict(f32::EPSILON * 4.0), 1);
        // dx = 2^-22: the parameter is exactly 1 - 2^-23: not below it.
        assert_eq!(verdict(f32::EPSILON * 2.0), 0);
        // dx = 2^-23: the parameter 1 - 2^-24 is above it but below 1 (the value the bare
        // determinant would have accepted).
        assert_eq!(verdict(f32::EPSILON), 0);
    }

    #[test]
    fn is_valid_edge_edge_answers_its_flag_pointer() {
        let mut e = engine();
        let t = scene(&mut e);
        let info = edge_info(&mut e);
        let valid = e.mem.alloc(4);
        // EAX is `valid` at every RET; the callers test the flag through it (00db19ed).
        assert_eq!(e.call(0x00db_15a0, &args![t, valid, info]).u32(), valid);
    }

    /// A scene for `add_additional_edge_helper`: transforms at `+0x14` and `+0x18` (identity
    /// with the translations (10, 20, 30) and (1, 2, 3)), the bodies 0x1111, 0x2222 and the word
    /// 0x3333, an object whose virtual function `+0xc` returns `created`, a record array with a
    /// cursor cell at `+0x10` and an empty manifold.
    struct Helper {
        t: u32,
        manifold: u32,
        cell: u32,
        record: u32,
        calls: Words,
    }

    fn helper_scene(e: &mut Engine, created: u32) -> Helper {
        let t = scene(e);
        let (m14, m18) = (
            transform(e, [10.0, 20.0, 30.0, 0.0]),
            transform(e, [1.0, 2.0, 3.0, 0.0]),
        );
        e.mem.set_u32(t + 0x14, m14);
        e.mem.set_u32(t + 0x18, m18);
        e.mem.set_u32(t, 0x1111);
        e.mem.set_u32(t + 4, 0x2222);
        e.mem.set_u32(t + 8, 0x3333);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, CREATE_POINT);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(t + 0xc, object);
        let records = e.mem.alloc(0x200);
        let record = records + 0x80;
        let cell = e.mem.alloc(0x100);
        e.mem.set_u32(cell, record);
        e.mem.set_u32(t + 0x10, cell);
        let calls = Words::default();
        let sink = Rc::clone(&calls);
        e.register_double(CREATE_POINT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: created,
                ..Ret::default()
            }
        });
        let manifold = new_manifold(e, &[]);
        Helper {
            t,
            manifold,
            cell,
            record,
            calls,
        }
    }

    fn run_helper(e: &mut Engine, h: &Helper, entry: [u8; 2], threshold: f32) -> u32 {
        let address = e.mem.alloc(4);
        e.mem.set_u8(address, entry[0]);
        e.mem.set_u8(address + 1, entry[1]);
        add_additional_edge_helper(
            e,
            Ptr::new(h.t),
            Ptr::new(h.manifold),
            Ptr::new(address),
            threshold,
        );
        address
    }

    #[test]
    fn add_additional_edge_helper_creates_the_contact_point_of_an_edge_entry() {
        let mut e = engine();
        let h = helper_scene(&mut e, 0x123);
        let entry = run_helper(&mut e, &h, [0x80, 0x71], 0.5);
        // The record: the point and normal in the world, the distance, the created id.
        expect_vec(&e, h.record, [9.2, 21.0, 31.0, 0.0]);
        expect_vec(&e, h.record + 0x10, [0.0, 0.0, -1.0, 1.0]);
        assert_eq!(e.mem.u32(h.record + 0x20), 0x123);
        // The manifold holds the entry with the id; the cursor moved on by 0x30.
        assert_eq!(e.mem.u8(h.manifold + 0x21), 1);
        assert_eq!(e.mem.u8(h.manifold), 0x80);
        assert_eq!(e.mem.u8(h.manifold + 1), 0x71);
        assert_eq!(e.mem.u16(h.manifold + 2), 0x123);
        assert_eq!(e.mem.u16(entry + 2), 0x123);
        assert_eq!(e.mem.u32(h.cell), h.record + 0x30);
        // A kind above 6 is not counted in `+0x20`.
        assert_eq!(e.mem.u8(h.manifold + 0x20), 0);
        assert_eq!(
            *h.calls.borrow(),
            vec![vec![
                e.mem.u32(h.t + 0xc),
                0x1111,
                0x2222,
                0x3333,
                h.cell,
                0,
                h.record
            ]]
        );
    }

    #[test]
    fn add_additional_edge_helper_leaves_the_manifold_alone_when_it_should() {
        // The key is in the manifold already.
        let mut e = engine();
        let h = helper_scene(&mut e, 0x123);
        let existing = new_manifold(&mut e, &[[0x80, 0x71]]);
        let h = Helper {
            manifold: existing,
            ..h
        };
        run_helper(&mut e, &h, [0x80, 0x71], 0.5);
        assert_eq!(e.mem.u8(existing + 0x21), 1);
        assert_eq!(e.mem.u32(h.cell), h.record);
        // The edges are not a valid pair (the distance is above the limit at `+0xb0`).
        let mut e = engine();
        let h = helper_scene(&mut e, 0x123);
        e.mem.set_f32(h.t + 0xb0, 0.5);
        run_helper(&mut e, &h, [0x80, 0x71], 0.5);
        assert_eq!(e.mem.u8(h.manifold + 0x21), 0);
        // A threshold above the distance: 2.0 * 0.999 > 1.0.
        let mut e = engine();
        let h = helper_scene(&mut e, 0x123);
        run_helper(&mut e, &h, [0x80, 0x71], 2.0);
        assert_eq!(e.mem.u8(h.manifold + 0x21), 0);
        // The virtual function refuses (0xffff): the point is taken out again.
        let mut e = engine();
        let h = helper_scene(&mut e, 0xffff);
        run_helper(&mut e, &h, [0x80, 0x71], 0.5);
        assert_eq!(e.mem.u8(h.manifold + 0x21), 0);
        assert_eq!(e.mem.u32(h.cell), h.record);
        assert_eq!(h.calls.borrow().len(), 1);
    }

    #[test]
    fn add_additional_edge_helper_checks_kinds_up_to_six_against_the_manifold() {
        // Kind 0 (a point of the first box): a point (1, 2, 3) of its own transform's
        // translation row. A manifold point of kind 1 within the tolerance rejects it.
        let mut e = engine();
        let h = helper_scene(&mut e, 0x123);
        let existing = new_manifold(&mut e, &[[1, 5]]);
        let h = Helper {
            manifold: existing,
            ..h
        };
        store_vec(&mut e, h.cell + 0x10, [1.0, 2.0, 3.0, 0.0]);
        run_helper(&mut e, &h, [0x00, 0x71], 0.5);
        assert_eq!(e.mem.u8(existing + 0x21), 1);
        assert!(h.calls.borrow().is_empty());
        // A far point lets it through to `addPoint`, but a normal that does not point the
        // way of the previous record takes the point out again.
        let mut e = engine();
        let h = helper_scene(&mut e, 0x123);
        let existing = new_manifold(&mut e, &[[1, 5]]);
        let h = Helper {
            manifold: existing,
            ..h
        };
        store_vec(&mut e, h.cell + 0x10, [100.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, h.record - 0x20, [0.0, 0.0, 1.0, 0.0]);
        run_helper(&mut e, &h, [0x00, 0x71], 0.5);
        assert_eq!(e.mem.u8(existing + 0x21), 1);
        assert!(h.calls.borrow().is_empty());
        expect_vec(&e, h.record, [1.0, 2.0, 3.0, 0.0]);
        // With a previous normal that agrees, the point is created and counted in `+0x20`.
        let mut e = engine();
        let h = helper_scene(&mut e, 0x123);
        let existing = new_manifold(&mut e, &[[1, 5]]);
        let h = Helper {
            manifold: existing,
            ..h
        };
        store_vec(&mut e, h.cell + 0x10, [100.0, 0.0, 0.0, 0.0]);
        // The normal of a kind 0 point is -0 times the first axis: a dot product of -0.0 or 0.0
        // is not above 0.0, so use the unordered case to get past it.
        store_vec(&mut e, h.record - 0x20, [f32::NAN, 0.0, 0.0, 0.0]);
        run_helper(&mut e, &h, [0x00, 0x71], 0.5);
        assert_eq!(e.mem.u8(existing + 0x21), 2);
        assert_eq!(e.mem.u8(existing + 0x20), 1);
        assert_eq!(h.calls.borrow().len(), 1);
        assert_eq!(e.mem.u32(h.cell), h.record + 0x30);
    }

    #[test]
    fn fn_00db1d90_runs_the_helper_for_the_four_side_combinations() {
        let mut e = engine();
        let keys = Words::default();
        let sink = Rc::clone(&keys);
        e.register_double(FIND_IN_MANIFOLD, move |e, a| {
            sink.borrow_mut().push(vec![
                u32::from(e.mem.u8(a[2])),
                u32::from(e.mem.u8(a[2] + 1)),
            ]);
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        store_vec(&mut e, a, [-1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, b, [1.0, -2.0, 5.0, 0.0]);
        let manifold = new_manifold(&mut e, &[]);
        // Entry (0x91, 0x52) as `fn_00db0dc0` builds it; the selectors 0 and 2 flip bits 4 and 6.
        fn_00db1d90(
            &mut e,
            Ptr::new(0x1000),
            Ptr::new(manifold),
            1,
            2,
            0,
            2,
            Ptr::new(a),
            Ptr::new(b),
            0.5,
        );
        assert_eq!(
            *keys.borrow(),
            vec![
                vec![0x91, 0x52],
                vec![0x81, 0x52],
                vec![0x81, 0x12],
                vec![0x91, 0x12]
            ]
        );
        // Selector 1 flips bit 5, any other selector bit 6.
        keys.borrow_mut().clear();
        fn_00db1d90(
            &mut e,
            Ptr::new(0x1000),
            Ptr::new(manifold),
            1,
            2,
            1,
            7,
            Ptr::new(a),
            Ptr::new(b),
            0.5,
        );
        assert_eq!(
            *keys.borrow(),
            vec![
                vec![0x91, 0x52],
                vec![0xb1, 0x52],
                vec![0xb1, 0x12],
                vec![0x91, 0x12]
            ]
        );
    }

    /// A scene for `find_closest_point`: the codes, the result record and an empty manifold.
    fn closest_scene(e: &mut Engine) -> (u32, u32, u32, u32) {
        let t = scene(e);
        let codes = e.mem.alloc(4);
        let out = e.mem.alloc(0x40);
        let manifold = new_manifold(e, &[]);
        (t, codes, out, manifold)
    }

    fn find(e: &mut Engine, t: u32, manifold: u32, codes: u32, out: u32) -> u32 {
        find_closest_point(
            e,
            Ptr::new(t),
            Ptr::new(manifold),
            Ptr::new(codes),
            Ptr::new(out),
        )
    }

    #[test]
    fn find_closest_point_on_a_face_of_the_first_box() {
        let mut e = engine();
        let (t, codes, out, manifold) = closest_scene(&mut e);
        separations(&mut e, t, &[(0xe0, -3.0), (0xe4, -1.0), (0xe8, -2.0)]);
        assert_eq!(find(&mut e, t, manifold, codes, out), 2);
        assert_eq!(e.mem.u8(codes), 1);
        assert_eq!(e.mem.u8(codes + 1), 0x58);
        assert_eq!(e.mem.u32(out + 0x38), 1);
        expect_float(&e, out + 0x30, -1.0);
        expect_vec(&e, out, [-0.8, 1.3, 2.0, 0.0]);
        expect_vec(&e, out + 0x10, [-1.0, 1.0, -1.0, 0.0]);
        expect_float(&e, out + 0x34, -2.3);
        // The manifold already has the key.
        let known = new_manifold(&mut e, &[[1, 0x58]]);
        assert_eq!(find(&mut e, t, known, codes, out), 1);
        // A point outside the tolerance extents (its y is 1.3) uses up the separation and the
        // next best, feature 2, is taken.
        store_vec(&mut e, t + 0x90, [10.0, 1.0, 10.0, 10.0]);
        assert_eq!(find(&mut e, t, manifold, codes, out), 2);
        assert_eq!(e.mem.u32(t + 0xe4), USED_UP_BITS);
        assert_eq!(e.mem.u8(codes), 2);
        assert_eq!(e.mem.u8(codes + 1), 0x70);
        expect_vec(&e, out, [-0.8, -0.7, 2.0, 0.0]);
        expect_float(&e, out + 0x34, 1.0);
    }

    #[test]
    fn find_closest_point_on_a_face_of_the_second_box() {
        let mut e = engine();
        let (t, codes, out, manifold) = closest_scene(&mut e);
        separations(&mut e, t, &[(0xf4, -1.0)]);
        assert_eq!(find(&mut e, t, manifold, codes, out), 2);
        assert_eq!(e.mem.u8(codes), 5);
        assert_eq!(e.mem.u8(codes + 1), 0x00);
        assert_eq!(e.mem.u32(out + 0x38), 5);
        expect_vec(&e, out, [1.0, 1.0, 1.0, 0.0]);
        expect_vec(&e, out + 0x10, [0.8, 0.7, -2.0, 0.0]);
        expect_float(&e, out + 0x34, -1.7);
    }

    #[test]
    fn find_closest_point_between_two_edges() {
        let mut e = engine();
        let (t, codes, out, manifold) = closest_scene(&mut e);
        separations(&mut e, t, &[(0x104, -0.5)]);
        assert_eq!(find(&mut e, t, manifold, codes, out), 2);
        // Feature 9: the x edge of the first box with the y edge of the second.
        assert_eq!(e.mem.u8(codes), 0x80);
        assert_eq!(e.mem.u8(codes + 1), 0x71);
        assert_eq!(e.mem.u32(out + 0x38), 0x80);
        assert_eq!(e.mem.u32(out + 0x3c), 0x71);
        expect_vec(&e, out, [-0.8, 1.0, 1.0, 0.0]);
        expect_float(&e, out + 0x34, 1.0);
        // The pair is not valid (limit `+0xb0`): its separation is used up and the next best,
        // the face at feature 0, is taken.
        let mut e = engine();
        let (t, codes, out, manifold) = closest_scene(&mut e);
        e.mem.set_f32(t + 0xb0, 0.5);
        separations(&mut e, t, &[(0x104, -0.5), (0xe0, -2.0)]);
        assert_eq!(find(&mut e, t, manifold, codes, out), 2);
        assert_eq!(e.mem.u32(t + 0x104), USED_UP_BITS);
        assert_eq!(e.mem.u8(codes), 0);
        assert_eq!(e.mem.u8(codes + 1), 0x70);
        expect_vec(&e, out, [-0.8, -0.7, 2.0, 0.0]);
        expect_float(&e, out + 0x34, -1.8);
        // The known key of an edge pair is reported as 1.
        let mut e = engine();
        let (t, codes, out, _) = closest_scene(&mut e);
        separations(&mut e, t, &[(0x104, -0.5)]);
        let known = new_manifold(&mut e, &[[0x80, 0x71]]);
        assert_eq!(find(&mut e, t, known, codes, out), 1);
    }

    /// A scene for `calculate_closest_point`: both boxes at (0.5, -1, 1.5), a world transform
    /// for each (identity with the translations (10, 20, 30) and (1, 2, 3)) and the query
    /// preparation (`00d40940`) replaced by a function that does nothing.
    fn calculate_scene(e: &mut Engine) -> (u32, u32, u32) {
        let t = scene(e);
        store_vec(e, t + 0x50, [0.5, -1.0, 1.5, 0.0]);
        store_vec(e, t + 0xd0, [0.5, -1.0, 1.5, 0.0]);
        let (m14, m18) = (
            transform(e, [10.0, 20.0, 30.0, 0.0]),
            transform(e, [1.0, 2.0, 3.0, 0.0]),
        );
        e.mem.set_u32(t + 0x14, m14);
        e.mem.set_u32(t + 0x18, m18);
        e.register_double(PREPARE_QUERY, |_, _| Ret::default());
        let result = e.mem.alloc(4);
        let out = e.mem.alloc(0x20);
        (t, result, out)
    }

    #[test]
    fn calculate_closest_point_reports_the_point_in_world_space() {
        // The first box's z face is the closest feature.
        let mut e = engine();
        let (t, result, out) = calculate_scene(&mut e);
        let returned =
            calculate_closest_point(&mut e, Ptr::new(t), Ptr::new(result), Ptr::new(out));
        assert_eq!(returned.addr(), result);
        assert_eq!(e.mem.u8(result), 1);
        expect_vec(&e, out, [0.0, 3.0, 2.0, 0.0]);
        expect_vec(&e, out + 0x10, [0.0, 0.0, -1.0, -0.5]);
        // A face of the second box (a feature code above 2 moves the point along the normal).
        let mut e = engine();
        let (t, result, out) = calculate_scene(&mut e);
        store_vec(&mut e, t + 0xd0, [1.9, 1.5, 1.2, 0.0]);
        calculate_closest_point(&mut e, Ptr::new(t), Ptr::new(result), Ptr::new(out));
        assert_eq!(e.mem.u8(result), 1);
        expect_vec(&e, out, [9.5, 19.0, 31.0, -2.25]);
        expect_vec(&e, out + 0x10, [-1.0, 0.0, 0.0, -1.5]);
        // Separated boxes: the byte is 0 and the record is left alone.
        let mut e = engine();
        let (t, result, out) = calculate_scene(&mut e);
        e.mem.set_u8(result, 0x55);
        store_vec(&mut e, t + 0xc0, [9.0, -1.0, 1.5, 0.0]);
        calculate_closest_point(&mut e, Ptr::new(t), Ptr::new(result), Ptr::new(out));
        assert_eq!(e.mem.u8(result), 0);
        assert_eq!(e.mem.u32(out), 0);
    }

    /// The virtual function `+0x14` of the object (it drops a contact point) and the word
    /// the packing function returns.
    const DROP_POINT: u32 = 0x00ee_0010;
    const PACKED: u32 = 0x1234_5678;

    /// A scene for `calc_manifold`: transforms at `+0x14` and `+0x18` (identity with the
    /// translations (10, 20, 30) and (1, 2, 3)), an object whose virtual function `+0xc`
    /// creates the ids 0x101, 0x102 and so on and whose function `+0x14` drops a point, a
    /// record array with the cursor cell at `+0x10` (the cell holds the first record's address
    /// at `+0` and the word 0x7777 at `+4`), a manifold with `entries` (the ids 0x500 and on)
    /// and doubles for the functions the exe calls from elsewhere.
    struct Manifolds {
        t: u32,
        manifold: u32,
        cell: u32,
        records: u32,
        object: u32,
        created: Words,
        dropped: Words,
        transformed: Words,
        packed: Words,
    }

    fn manifold_scene(e: &mut Engine, entries: &[[u8; 2]]) -> Manifolds {
        let t = scene(e);
        let (m14, m18) = (
            transform(e, [10.0, 20.0, 30.0, 0.0]),
            transform(e, [1.0, 2.0, 3.0, 0.0]),
        );
        e.mem.set_u32(t + 0x14, m14);
        e.mem.set_u32(t + 0x18, m18);
        e.mem.set_u32(t, 0x1111);
        e.mem.set_u32(t + 4, 0x2222);
        e.mem.set_u32(t + 8, 0x3333);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, CREATE_POINT);
        e.mem.set_u32(vtable + 0x14, DROP_POINT);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(t + 0xc, object);
        let cell = e.mem.alloc(0x10 + 0x30 * 16);
        let records = cell + 0x10;
        e.mem.set_u32(cell, records);
        e.mem.set_u32(cell + 4, 0x7777);
        e.mem.set_u32(t + 0x10, cell);
        let manifold = e.mem.alloc(0x40);
        for (index, entry) in entries.iter().enumerate() {
            let at = manifold + 4 * index as u32;
            e.mem.set_u8(at, entry[0]);
            e.mem.set_u8(at + 1, entry[1]);
            e.mem.set_u16(at + 2, 0x500 + index as u16);
        }
        let counted = entries.iter().filter(|entry| entry[0] <= 6).count();
        e.mem.set_u8(manifold + 0x20, counted as u8);
        e.mem.set_u8(manifold + 0x21, entries.len() as u8);
        e.mem.set_u8(manifold + 0x22, 0xaa);
        store_vec(e, manifold + 0x30, [1.0, 0.0, 0.0, 0.0]);
        e.register_double(PREPARE_QUERY, |_, _| Ret::default());
        let created = Words::default();
        let sink = Rc::clone(&created);
        let mut next = 0x100;
        e.register_double(CREATE_POINT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            next += 1;
            Ret {
                eax: next,
                ..Ret::default()
            }
        });
        let dropped = Words::default();
        let sink = Rc::clone(&dropped);
        e.register_double(DROP_POINT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let transformed = Words::default();
        let sink = Rc::clone(&transformed);
        e.register_double(TRANSFORM_VECTOR, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            store_vec(e, a[0], [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        let packed = Words::default();
        let sink = Rc::clone(&packed);
        e.register_double(PACK_DIRECTION, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: PACKED,
                ..Ret::default()
            }
        });
        e.register_double(SET_ROTATED_DIR, |e, a| {
            let rotated = transform_direction(e, a[1], load_vec(e, a[2]));
            store_vec(e, a[0], rotated);
            Ret::default()
        });
        Manifolds {
            t,
            manifold,
            cell,
            records,
            object,
            created,
            dropped,
            transformed,
            packed,
        }
    }

    fn run_calc(e: &mut Engine, s: &Manifolds) {
        calc_manifold(e, Ptr::new(s.t), Ptr::new(s.manifold));
    }

    /// The number of records between the first one and the cursor.
    fn records_in_use(e: &Engine, s: &Manifolds) -> u32 {
        (e.mem.u32(s.cell) - s.records) / 0x30
    }

    #[test]
    fn calc_manifold_rewrites_the_records_of_every_kind_of_entry() {
        let mut e = engine();
        // A face of the second box, a face of the first box and an edge pair.
        let s = manifold_scene(&mut e, &[[0x00, 0x00], [0x04, 0x08], [0x80, 0x71]]);
        // Separated along x, so nothing happens after the records and the direction.
        store_vec(&mut e, s.t + 0xc0, [5.0, -1.0, 1.5, 0.0]);
        run_calc(&mut e, &s);
        assert_eq!(e.mem.u8(s.manifold + 0x21), 3);
        assert_eq!(records_in_use(&e, &s), 3);
        // Kind 0: the corner (1, 1, 1) of the second box, the normal -x, the distance 1.2 - 1.
        expect_vec(&e, s.records, [2.0, 3.0, 4.0, 0.0]);
        expect_vec(&e, s.records + 0x10, [-1.0, 0.0, 0.0, 0.2]);
        assert_eq!(e.mem.u32(s.records + 0x20), 0x500);
        // Kind 4: the corner (1, 1, 1) of the first box, the normal +x (the sign bit of the
        // code is set), the distance 0.8 - 1.
        expect_vec(&e, s.records + 0x30, [11.0, 21.0, 31.0, 0.0]);
        expect_vec(&e, s.records + 0x40, [1.0, 0.0, 0.0, -0.2]);
        assert_eq!(e.mem.u32(s.records + 0x50), 0x501);
        // The edge pair: its crossing point and normal as `isValidEdgeEdge` finds them.
        expect_vec(&e, s.records + 0x60, [9.2, 21.0, 31.0, 0.0]);
        expect_vec(&e, s.records + 0x70, [0.0, 0.0, -1.0, 1.0]);
        assert_eq!(e.mem.u32(s.records + 0x80), 0x502);
        // The kinds up to 6 are still counted.
        assert_eq!(e.mem.u8(s.manifold + 0x20), 2);
        // The direction of three points is made from the normal and packed.
        let transformed = s.transformed.borrow().clone();
        assert_eq!(transformed.len(), 1);
        assert_eq!(transformed[0][1..], [s.t + 0x20, s.manifold + 0x30]);
        assert_eq!(*s.packed.borrow(), vec![vec![transformed[0][0]]]);
        assert_eq!(e.mem.u8(s.manifold + 0x23), 1);
        assert_eq!(e.mem.u32(s.manifold + 0x24), PACKED);
        assert!(s.created.borrow().is_empty());
        assert!(s.dropped.borrow().is_empty());
    }

    #[test]
    fn calc_manifold_removes_entries_outside_the_tolerance_extents() {
        let mut e = engine();
        let s = manifold_scene(&mut e, &[[0x00, 0x00]]);
        store_vec(&mut e, s.t + 0xc0, [5.0, -1.0, 1.5, 0.0]);
        // The corner is at 1.2 along x, the extents allow 0.5.
        store_vec(&mut e, s.t + 0x90, [0.5; 4]);
        run_calc(&mut e, &s);
        assert_eq!(e.mem.u8(s.manifold + 0x21), 0);
        assert_eq!(e.mem.u8(s.manifold + 0x20), 0);
        assert_eq!(records_in_use(&e, &s), 0);
        assert_eq!(*s.dropped.borrow(), vec![vec![s.object, 0x500, 0x7777]]);
        // An edge pair that is not valid any more is removed as well.
        let mut e = engine();
        let s = manifold_scene(&mut e, &[[0x80, 0x71]]);
        store_vec(&mut e, s.t + 0xc0, [5.0, -1.0, 1.5, 0.0]);
        store_vec(&mut e, s.t + 0x50, [3.2, 0.3, 3.0, 0.0]);
        run_calc(&mut e, &s);
        assert_eq!(e.mem.u8(s.manifold + 0x21), 0);
        assert_eq!(e.mem.u8(s.manifold + 0x20), 0);
        assert_eq!(*s.dropped.borrow(), vec![vec![s.object, 0x500, 0x7777]]);
    }

    #[test]
    fn calc_manifold_stops_when_the_direction_has_not_moved() {
        let mut e = engine();
        let s = manifold_scene(&mut e, &[[0x00, 0x00], [0x00, 0x20]]);
        // The direction is +x (0x80 + 116 in the first byte): the dot product with the normal
        // at `+0x30` (also +x) is above 0.95 and the completeness byte is set.
        e.mem.set_u8(s.manifold + 0x23, 1);
        e.mem.set_u32(s.manifold + 0x24, 0x8080_80f4);
        run_calc(&mut e, &s);
        assert_eq!(records_in_use(&e, &s), 2);
        assert_eq!(e.mem.u8(s.manifold + 0x22), 0xaa);
        // The boxes overlap, so `checkIntersection` would have stored its separations.
        assert_eq!(e.mem.f32(s.t + 0xe0), 0.0);
        assert!(s.transformed.borrow().is_empty());
    }

    /// The overlapping boxes of `calculate_closest_point`: the z face of the first box is the
    /// closest feature.
    fn overlapping(e: &mut Engine, s: &Manifolds) {
        store_vec(e, s.t + 0x50, [0.5, -1.0, 1.5, 0.0]);
        store_vec(e, s.t + 0xd0, [0.5, -1.0, 1.5, 0.0]);
    }

    #[test]
    fn calc_manifold_adds_the_closest_point_of_overlapping_boxes() {
        let mut e = engine();
        let s = manifold_scene(&mut e, &[]);
        overlapping(&mut e, &s);
        run_calc(&mut e, &s);
        // The separations of `checkIntersection` were stored.
        expect_vec(&e, s.t + 0xe0, [-1.5, -1.0, -0.5, 0.0]);
        let count = u32::from(e.mem.u8(s.manifold + 0x21));
        assert!(count >= 1);
        // The z face of the first box is the first point: kind 2, id 0x101, the record of
        // `calculate_closest_point`.
        assert_eq!(e.mem.u8(s.manifold), 2);
        assert_eq!(e.mem.u16(s.manifold + 2), 0x101);
        expect_vec(&e, s.records, [0.0, 3.0, 2.0, 0.0]);
        expect_vec(&e, s.records + 0x10, [0.0, 0.0, -1.0, -0.5]);
        assert_eq!(e.mem.u32(s.records + 0x20), 0x101);
        // Every point has one record and one created id, in order.
        assert_eq!(records_in_use(&e, &s), count);
        let created = s.created.borrow();
        assert!(created.len() as u32 >= count);
        for (index, call) in created.iter().enumerate() {
            assert_eq!(call[..5], [s.object, 0x1111, 0x2222, 0x3333, s.cell]);
            assert_eq!(call[5], 0);
            assert_eq!(call[6], s.records + 0x30 * index as u32);
        }
        // The normal of the manifold follows the face.
        expect_vec(&e, s.manifold + 0x30, [0.0, 0.0, -1.0, 0.0]);
        assert!(s.dropped.borrow().is_empty());
    }

    #[test]
    fn calc_manifold_drops_records_that_leave_the_direction() {
        let mut e = engine();
        let s = manifold_scene(&mut e, &[[0x00, 0x00], [0x00, 0x20]]);
        overlapping(&mut e, &s);
        // The direction is +y (0x80 + 116 in the second byte), the normal +x: the manifold
        // has moved, so the closest feature sets a new normal (-z) and both old records, whose
        // normals (-x) point elsewhere, go.
        e.mem.set_u8(s.manifold + 0x23, 1);
        e.mem.set_u32(s.manifold + 0x24, 0x8080_f480);
        run_calc(&mut e, &s);
        assert_eq!(
            *s.dropped.borrow(),
            vec![vec![s.object, 0x500, 0x7777], vec![s.object, 0x501, 0x7777]]
        );
        let count = u32::from(e.mem.u8(s.manifold + 0x21));
        assert!(count >= 1);
        assert_eq!(records_in_use(&e, &s), count);
        expect_vec(&e, s.manifold + 0x30, [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(e.mem.f32(s.t + 0xe0), -1.5);
    }
}
