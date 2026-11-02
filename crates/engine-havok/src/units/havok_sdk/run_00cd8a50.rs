//! `Havok SDK/run_00cd8a50` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here belong to a foot placement solver and to the quaternion helper it uses
//! (the class is unconfirmed; `DoFootIK` calls `00cd8d90`). They mix SSE vectors and x87
//! scalars. SSE lanes are computed in `f32` one instruction at a time; `RCPPS`/`RSQRTPS`/
//! `RSQRTSS` are the host's estimates (which differ between Intel and AMD parts in the last
//! bits). The x87 stack is modelled by [`Fpu`] (values as `f64`, every NaN choice through the
//! prelude's `x87_*` helpers, every `FLD`/`FSTP` of a float through `x87_load32`/`x87_store32`).
//!
//! `00cd8d90` is translated instruction by instruction: it keeps the exe's eight vector
//! registers (`x0`..`x7`), its integer registers, its stack frame (a block of the engine's
//! memory, so the addresses it hands to its callees are real) and its basic blocks (the arms of
//! a `match` on the block's address), because its x87 stack juggling and its register reuse
//! cross block boundaries.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkaabbtree::rcp_estimate;
use crate::units::havok_sdk::hkpconvexradiusbuilder::{
    addps, mulps, shufps, splat, subps, sum_of_three,
};
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, Vec4};
use crate::x87::rsqrt_estimate;

type V = Vec4;

/// The float `3.4028235e38` (`0104da48`) the casts' fractions start at, the double `0.0`
/// (`01012060`), the float epsilon (`01017d00`) and the lane mask that takes magnitudes
/// (`01017d10`).
const FRACTION_START: u32 = 0x0104_da48;
const TOLERANCE: u32 = 0x0101_7d00;
const ABSOLUTE_MASK: u32 = 0x0101_7d10;
/// The zero vector (`010c7160`) the second cast's plane distance is taken from.
const ZERO_VECTOR: u32 = 0x010c_7160;
/// `hkQuaternion::setAxisAngle`-like helper (`00cb2700`, `this` = the result, one word: the
/// vector whose perpendicular axis it turns a half circle about; unnamed).
const HALF_TURN: u32 = 0x00cb_2700;
/// Vector constants of the quaternion helper: the sign mask (`010c44d0`), `0.99999`
/// (`010cb620`), `-0.999` (`010cb630`), `(0, 0, 0, 1)` (`010c71b0`), `3.0` (`010c71c0`) and `0.5`
/// (`010c71d0`) in every lane, and the scalars `0.0` (`01011d78`), `3.0` (`01017718`) and `0.5`
/// (`01016248`).
const SIGN_MASK: u32 = 0x010c_44d0;
const NEAR_ONE: u32 = 0x010c_b620;
const NEAR_MINUS_ONE: u32 = 0x010c_b630;
const IDENTITY_QUATERNION: u32 = 0x010c_71b0;
const THREE_VECTOR: u32 = 0x010c_71c0;
const HALF_VECTOR: u32 = 0x010c_71d0;
const ZERO_SCALAR: u32 = 0x0101_1d78;
const THREE_SCALAR: u32 = 0x0101_7718;
const HALF_SCALAR: u32 = 0x0101_6248;

fn ld(e: &Engine, at: u32) -> V {
    load_vec(e, at)
}

fn st(e: &mut Engine, at: u32, v: V) {
    store_vec(e, at, v);
}

/// `ADDSS dst, src`: lane 0 summed, the other lanes of `dst` kept.
fn addss(dst: V, src: V) -> V {
    [dst[0] + src[0], dst[1], dst[2], dst[3]]
}

/// `MOVSS xmm, m32`: lane 0 loaded, the others zeroed (the float's bits are kept).
fn movss_load(bits: u32) -> V {
    [f32::from_bits(bits), 0.0, 0.0, 0.0]
}

/// `XORPS` and `ANDPS`: lane-wise on the bit patterns.
fn xorps(a: V, b: V) -> V {
    std::array::from_fn(|lane| f32::from_bits(a[lane].to_bits() ^ b[lane].to_bits()))
}

fn andps(a: V, b: V) -> V {
    std::array::from_fn(|lane| f32::from_bits(a[lane].to_bits() & b[lane].to_bits()))
}

/// `UNPCKHPS dst, src`.
fn unpckhps(dst: V, src: V) -> V {
    [dst[2], src[2], dst[3], src[3]]
}

/// `RCPPS`: the host's estimate in every lane.
fn rcpps(src: V) -> V {
    src.map(rcp_estimate)
}

/// `RSQRTPS`: the host's estimate in every lane.
fn rsqrtps(src: V) -> V {
    src.map(rsqrt_estimate)
}

/// The status-word bits the x87 compares leave in `AH`: `C0` (`0x01`), `C2` (`0x04`) and `C3`
/// (`0x40`): clear when `a > b`, `C0` when `a < b`, `C3` when equal, all three when unordered.
fn compare_status(a: f64, b: f64) -> u8 {
    if a.is_nan() || b.is_nan() {
        0x45
    } else if a > b {
        0
    } else if a < b {
        0x01
    } else {
        0x40
    }
}

/// The second operand of an x87 compare.
#[derive(Clone, Copy)]
enum Operand {
    /// A float in memory (its bits), compared as the x87 sees a float memory operand.
    Float(u32),
    /// A double in memory.
    Double(f64),
    /// `ST(i)`.
    Register(usize),
}

/// The x87 register stack: `ST(0)` is the last element. Values are `f64`; floats are loaded
/// and stored through `x87_load32`/`x87_store32`, arithmetic through `x87_add` and friends.
struct Fpu {
    stack: Vec<f64>,
    /// The `AH` bits of the status word after the last compare.
    status: u8,
}

impl Fpu {
    fn new() -> Self {
        Fpu {
            stack: Vec::new(),
            status: 0,
        }
    }

    fn push(&mut self, value: f64) {
        assert!(self.stack.len() < 8, "x87 stack overflow");
        self.stack.push(value);
    }

    fn pop(&mut self) -> f64 {
        self.stack.pop().expect("x87 stack underflow")
    }

    fn st(&self, i: usize) -> f64 {
        self.stack[self.stack.len() - 1 - i]
    }

    fn set_st(&mut self, i: usize, value: f64) {
        let top = self.stack.len() - 1;
        self.stack[top - i] = value;
    }

    /// `FLD dword`.
    fn fld32(&mut self, bits: u32) {
        self.push(x87_load32(bits));
    }

    /// `FLD ST(i)`.
    fn fld_st(&mut self, i: usize) {
        let value = self.st(i);
        self.push(value);
    }

    /// `FST dword`: the bits of `ST(0)` as a float.
    fn fst32(&self) -> u32 {
        x87_store32(self.st(0))
    }

    /// `FSTP dword`.
    fn fstp32(&mut self) -> u32 {
        let bits = self.fst32();
        self.pop();
        bits
    }

    /// `FSTP ST(i)`: `ST(i)` takes `ST(0)`, then the stack pops.
    fn fstp_st(&mut self, i: usize) {
        let value = self.st(0);
        self.set_st(i, value);
        self.pop();
    }

    fn fxch(&mut self, i: usize) {
        let top = self.stack.len() - 1;
        self.stack.swap(top, top - i);
    }

    fn fchs(&mut self) {
        let value = self.st(0);
        self.set_st(0, -value);
    }

    /// `FADD`/`FSUB`/`FMUL dword ptr`.
    fn fadd_m32(&mut self, bits: u32) {
        let value = x87_add(self.st(0), x87_m32(bits));
        self.set_st(0, value);
    }

    fn fsub_m32(&mut self, bits: u32) {
        let value = x87_sub(self.st(0), x87_m32(bits));
        self.set_st(0, value);
    }

    fn fmul_m32(&mut self, bits: u32) {
        let value = x87_mul(self.st(0), x87_m32(bits));
        self.set_st(0, value);
    }

    /// `FSUB ST(dst), ST(src)`: `ST(dst) = ST(dst) - ST(src)`.
    fn fsub_st(&mut self, dst: usize, src: usize) {
        let value = x87_sub(self.st(dst), self.st(src));
        self.set_st(dst, value);
    }

    /// `FSUBR ST(dst), ST(src)`: `ST(dst) = ST(src) - ST(dst)`.
    fn fsubr_st(&mut self, dst: usize, src: usize) {
        let value = x87_sub(self.st(src), self.st(dst));
        self.set_st(dst, value);
    }

    /// `FADDP ST(i), ST(0)`: `ST(i) = ST(i) + ST(0)`, then pop.
    fn faddp(&mut self, i: usize) {
        let value = x87_add(self.st(i), self.st(0));
        self.set_st(i, value);
        self.pop();
    }

    fn fmulp(&mut self, i: usize) {
        let value = x87_mul(self.st(i), self.st(0));
        self.set_st(i, value);
        self.pop();
    }

    /// `FSUBP ST(i), ST(0)`: `ST(i) = ST(i) - ST(0)`, then pop.
    fn fsubp(&mut self, i: usize) {
        let value = x87_sub(self.st(i), self.st(0));
        self.set_st(i, value);
        self.pop();
    }

    /// `FSUBRP ST(i), ST(0)`: `ST(i) = ST(0) - ST(i)`, then pop.
    fn fsubrp(&mut self, i: usize) {
        let value = x87_sub(self.st(0), self.st(i));
        self.set_st(i, value);
        self.pop();
    }

    /// `FDIVP ST(i), ST(0)`: `ST(i) = ST(i) / ST(0)`, then pop.
    fn fdivp(&mut self, i: usize) {
        let value = x87_div(self.st(i), self.st(0));
        self.set_st(i, value);
        self.pop();
    }

    /// `FCOM`: compares `ST(0)` with the operand (`ST(1)` when there is none).
    fn fcom(&mut self, operand: Option<Operand>) {
        let other = match operand {
            None => self.st(1),
            Some(Operand::Float(bits)) => x87_m32(bits),
            Some(Operand::Double(value)) => value,
            Some(Operand::Register(i)) => self.st(i),
        };
        self.status = compare_status(self.st(0), other);
    }

    fn fcomp(&mut self, operand: Option<Operand>) {
        self.fcom(operand);
        self.pop();
    }

    fn fcompp(&mut self, operand: Option<Operand>) {
        self.fcom(operand);
        self.pop();
        self.pop();
    }
}

/// The words a `CALL` receives from the pushes since the last call, in the callee's order (the
/// last word pushed is the first argument).
fn take_args(pushed: &mut Vec<u32>) -> Vec<u32> {
    pushed.drain(..).rev().collect()
}

/// A `thiscall` with `this` followed by the pushed words.
fn thiscall(e: &mut Engine, address: u32, this: u32, pushed: &mut Vec<u32>) -> Ret {
    let mut words = vec![this];
    words.extend(take_args(pushed));
    e.call(address, &words)
}

// `Translated from 00cd9d10`'s sibling helper: the dot-and-cross rotation of a vector `v` by
// the quaternion `q` the exe expands inline many times is not shared here: `00cd8d90` keeps
// each expansion instruction by instruction.

// Translated from 00cd8a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A two-cast probe along a segment (the classes are unconfirmed). `this` holds the segment's
/// direction `V` at `+0x40` (vector), the radius `r` at `+0x7c` and the length `x` at `+0x80`;
/// `query` (the `00cd8d90` input block) the collidable at `+0x64` (virtual methods `+4`, taking one
/// more word, the word at `query + 0x80`, when the byte at `query + 0x84` is not 0, and `+8`
/// otherwise), and `point` and `second` are two points. The exe's frame, as offsets from its
/// start `F`: `+0x0c` `r`, `+0x10` the first cast's fraction (starting at the float at
/// `0104da48`), `+0x14` the second cast's fraction (preset to `-x`; the cast writes its hit fraction there), `+0x18` the second answer, `+0x1c` `x + r`, `+0x24` and `+0x34` the
/// segment ends handed to the casts, `+0x44` and `+0x54` the casts' output blocks.
///
/// First cast: the segment from `B = point + r * V` to `A = point - x * V`; the byte it reports
/// is `hit_first`. The second cast runs unless `|second - point| <= tolerance` in lanes 0 to 2
/// (`01017d00`): the segment from `second + r * V` to `second - x * V`, with the fraction slot
/// at `F + 0x14` and its output at `F + 0x44`.
/// It counts as a hit when the cast reports one and the dot of `V` with the output's first
/// vector is positive (or a NaN); its fraction `t2` is then `(s_R - s_P) / (-dot * (x + r))` with
/// `s_R` and `s_P` the output vector's dot with the point `B2 + f * (A2 - B2)` (`f` the fraction the cast wrote) and with
/// `point + r * V`. With no hit at all the answer is false. Otherwise the output vector
/// (`a4`) and fraction are the first cast's unless both hit and the first fraction exceeds the
/// second (or only the second hit). The position written to `a3` is `point + splat(-(t * (x
/// + r) - r)) * V`. Returns whether anything hit.
pub fn fn_00cd8a50(
    e: &mut Engine,
    this: Ptr,
    query: Ptr,
    point: Ptr,
    second: Ptr,
    out_position: Ptr,
    out_normal: Ptr,
) -> bool {
    let me = this.addr();
    let query = query.addr();
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let r_bits = x87_quiet_bits(e.mem.u32(me + 0x7c));
        e.mem.set_u32(f + 0xc, r_bits);
        let direction = ld(e, me + 0x40);
        let origin = ld(e, point.addr());
        let x_bits = x87_quiet_bits(e.mem.u32(me + 0x80));
        e.mem.set_u32(f + 0x1c, x_bits);
        let one_sided = e.mem.u8(query + 0x84) == 0;
        let x = x87_load32(x_bits);
        let collidable = e.mem.u32(query + 0x64);
        let reach_bits = x87_store32(x87_add(x, x87_m32(r_bits)));
        e.mem.set_u32(f + 0x1c, reach_bits);
        e.mem
            .set_u32(f + 0x10, x87_quiet_bits(e.mem.u32(FRACTION_START)));
        let minus_x_bits = x87_store32(-x);
        e.mem.set_u32(f + 0x14, minus_x_bits);
        e.mem.set_u32(f + 0x18, x87_quiet_bits(minus_x_bits));
        let minus_x = f32::from_bits(e.mem.u32(f + 0x18));
        let radius = f32::from_bits(e.mem.u32(f + 0xc));
        let end = addps(mulps(splat(minus_x), direction), origin);
        st(e, f + 0x34, end);
        let start = addps(mulps(splat(radius), direction), origin);
        st(e, f + 0x24, start);
        let reply = if one_sided {
            e.vcall(
                collidable,
                8,
                &args![f + 0xb, f + 0x24, f + 0x34, f + 0x10, f + 0x54],
            )
        } else {
            let extra = e.mem.u32(query + 0x80);
            e.vcall(
                collidable,
                4,
                &args![f + 0xa, f + 0x24, f + 0x34, extra, f + 0x10, f + 0x54],
            )
        };
        let hit_first = e.mem.u8(reply.u32()) != 0;
        e.mem
            .set_u32(f + 0x18, x87_quiet_bits(e.mem.u32(FRACTION_START)));
        let other = ld(e, second.addr());
        let again = ld(e, point.addr());
        let mask = ABSOLUTE_MASK;
        let distance = andps(subps(other, again), ld(e, mask));
        let tolerance = e.mem.f32(TOLERANCE);
        let near = (0..3).all(|lane| distance[lane] <= tolerance);
        let mut hit_second = false;
        if !near {
            let nx_bits = x87_quiet_bits(e.mem.u32(f + 0x14));
            e.mem.set_u32(f + 0x14, nx_bits);
            let direction = ld(e, me + 0x40);
            let collidable = e.mem.u32(query + 0x64);
            let nx = f32::from_bits(e.mem.u32(f + 0x14));
            let end = addps(mulps(splat(nx), direction), other);
            st(e, f + 0x34, end);
            let radius = f32::from_bits(e.mem.u32(f + 0xc));
            let start = addps(mulps(splat(radius), direction), other);
            st(e, f + 0x24, start);
            let reply = if one_sided {
                e.vcall(
                    collidable,
                    8,
                    &args![f + 0xa, f + 0x24, f + 0x34, f + 0x14, f + 0x44],
                )
            } else {
                let extra = e.mem.u32(query + 0x80);
                e.vcall(
                    collidable,
                    4,
                    &args![f + 0xb, f + 0x24, f + 0x34, extra, f + 0x14, f + 0x44],
                )
            };
            hit_second = e.mem.u8(reply.u32()) != 0;
            if hit_second {
                let direction = ld(e, me + 0x40);
                let normal = ld(e, f + 0x44);
                let slope = sum_of_three(mulps(direction, normal));
                e.mem.set_u32(f + 0x20, slope.to_bits());
                let slope = x87_load32(slope.to_bits());
                // `FCOM` of the slope with 0.0: only a slope above zero (or a NaN) goes on.
                if slope <= 0.0 {
                    hit_second = false;
                } else {
                    let start = ld(e, f + 0x24);
                    let nx = f32::from_bits(e.mem.u32(f + 0x14));
                    let end = ld(e, f + 0x34);
                    let along = addps(mulps(splat(nx), subps(end, start)), start);
                    let s_r = sum_of_three(mulps(along, normal));
                    let neg_s_r = ld(e, ZERO_VECTOR)[0] - s_r;
                    e.mem.set_u32(f + 0x18, neg_s_r.to_bits());
                    let origin = ld(e, point.addr());
                    let radius = f32::from_bits(e.mem.u32(f + 0xc));
                    let moved = addps(mulps(splat(radius), direction), origin);
                    let s_p = sum_of_three(mulps(moved, normal));
                    e.mem.set_u32(f + 0x20, s_p.to_bits());
                    let sum = x87_add(x87_load32(s_p.to_bits()), x87_m32(neg_s_r.to_bits()));
                    let numerator_bits = x87_store32(-sum);
                    e.mem.set_u32(f + 0x20, numerator_bits);
                    let numerator = x87_load32(numerator_bits);
                    let reach = -x87_load32(e.mem.u32(f + 0x1c));
                    let denominator_bits = x87_store32(x87_mul(slope, reach));
                    e.mem.set_u32(f + 0x20, denominator_bits);
                    let t2 = x87_div(numerator, x87_m32(denominator_bits));
                    e.mem.set_u32(f + 0x18, x87_store32(t2));
                }
            }
        }
        let second_fraction = x87_load32(e.mem.u32(f + 0x18));
        let fraction;
        if !hit_first {
            if !hit_second {
                return false;
            }
            copy_vector(e, f + 0x44, out_normal.addr());
            fraction = second_fraction;
        } else {
            let first_fraction = x87_load32(e.mem.u32(f + 0x10));
            if first_fraction > second_fraction {
                copy_vector(e, f + 0x44, out_normal.addr());
                fraction = second_fraction;
            } else {
                copy_vector(e, f + 0x54, out_normal.addr());
                fraction = first_fraction;
            }
        }
        e.mem.set_u32(f + 0x18, x87_store32(fraction));
        let t = x87_load32(e.mem.u32(f + 0x18));
        let scaled = x87_mul(t, x87_m32(e.mem.u32(f + 0x1c)));
        let moved = x87_sub(scaled, x87_m32(e.mem.u32(f + 0xc)));
        e.mem.set_u32(f + 0x20, x87_store32(moved));
        let negated = -x87_load32(e.mem.u32(f + 0x20));
        e.mem.set_u32(f + 0x20, x87_store32(negated));
        let scale = f32::from_bits(e.mem.u32(f + 0x20));
        let direction = ld(e, me + 0x40);
        let origin = ld(e, point.addr());
        st(
            e,
            out_position.addr(),
            addps(mulps(splat(scale), direction), origin),
        );
        true
    })
}

/// Copies one 16-byte vector (`MOVAPS` load and store), bit for bit.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00cd9d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Quaternion between two unit vectors, `RET 8` with `this` = the result: with `p` and `q` the
/// vectors at `a0` and `a1` and `d = q . p` (three lanes, summed `z + (y + x)`): when `d >
/// 0.99999` the result is `(0, 0, 0, 1)`; when `d < -0.99999` it is made by `00cb2700(this,
/// a0)`; otherwise, with `s = 0.5 * d + 0.5`, `n = (3 - (r * s) * r) * (r * 0.5)` (`r` the
/// reciprocal square root estimate of `s`) and the cross product `c = q.zxy * p.yzx - q.yzx *
/// p.zxy`, the result is `(c * k, s * n)` where the first three lanes use `k = n * 0.5` and
/// the last lane `s * n`; for `d < -0.999` the scale is refined: `k = g * f` with `f = (0.5 *
/// r2) * (3 - (l * r2) * r2)` for the squared length `l` of `c` (0 when `l` is 0) and `g =
/// (3 - ((r3 * e) * r3)) * (r3 * 0.5) * e` for `e = s - d`.
pub fn fn_00cd9d10(e: &mut Engine, this: Ptr, a0: Ptr, a1: Ptr) {
    let p = ld(e, a0.addr());
    let q = ld(e, a1.addr());
    let dot = sum_of_three(mulps(q, p));
    let dot_splat = splat(dot);
    let limit = ld(e, NEAR_ONE);
    let mirrored = xorps(ld(e, SIGN_MASK), limit);
    let above = (0..4).any(|lane| limit[lane] < dot_splat[lane]);
    let below = (0..4).any(|lane| dot_splat[lane] < mirrored[lane]);
    if above {
        let identity = ld(e, IDENTITY_QUATERNION);
        st(e, this.addr(), identity);
        return;
    }
    if below {
        e.call(HALF_TURN, &args![this, a0]);
        return;
    }
    let half = ld(e, HALF_VECTOR);
    let three = ld(e, THREE_VECTOR);
    let s = addps(mulps(half, dot_splat), half);
    let r = rsqrtps(s);
    let r_s_r = mulps(mulps(r, s), r);
    let r_half = mulps(r, ld(e, HALF_VECTOR));
    let n = mulps(subps(three, r_s_r), r_half);
    let cross = subps(
        mulps(shufps(q, q, 0xd2), shufps(p, p, 0xc9)),
        mulps(shufps(q, q, 0xc9), shufps(p, p, 0xd2)),
    );
    let near_minus_one = ld(e, NEAR_MINUS_ONE);
    let refine = (0..4).any(|lane| dot_splat[lane] < near_minus_one[lane]);
    let mut scale = mulps(n, ld(e, HALF_VECTOR));
    if refine {
        let zero = e.mem.f32(ZERO_SCALAR);
        let squares = mulps(cross, cross);
        let length_squared = sum_of_three(squares);
        let r2 = rsqrt_estimate(length_squared);
        let difference = subps(s, dot_splat);
        let refined_r2 = (length_squared * r2) * r2;
        let three_scalar = e.mem.f32(THREE_SCALAR);
        let half_scalar = e.mem.f32(HALF_SCALAR);
        let inverse_length = (half_scalar * r2) * (three_scalar - refined_r2);
        let factor = if length_squared == zero {
            0.0
        } else {
            inverse_length
        };
        let r3 = rsqrtps(difference);
        let r3_diff_r3 = mulps(mulps(r3, difference), r3);
        let r3_half = mulps(r3, ld(e, HALF_VECTOR));
        let root = mulps(mulps(subps(three, r3_diff_r3), r3_half), difference);
        scale = mulps(root, splat(factor));
    }
    let axis = mulps(scale, cross);
    let w = mulps(s, n);
    st(e, this.addr(), [axis[0], axis[1], axis[2], w[3]]);
}

#[allow(unused_assignments, unused_mut, unused_variables, unused_labels)]
// Translated from 00cd8d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Foot placement step (the classes are unconfirmed; see the module note for the method),
/// `RET 0xc`. `this` is the solver state, `input` the per-foot input block, `out` an output of
/// a byte at `+4` and a float at `+0`, `pose` the pose whose bone transforms are read (and
/// recalculated through `00cda180` when the bone's flag has bit 1 set).
pub fn fn_00cd8d90(e: &mut Engine, this: Ptr, input: Ptr, out: Ptr, pose: Ptr) {
    e.with_stack(0x280, |e, frame| {
        let base = frame.addr();
        // The exe's registers. The three stack arguments are written where `[EBP + 8]` reads
        // them; `sp` is `ESP` after the prologue's three register pushes.
        let ebp = base + 0x200;
        e.mem.set_u32(ebp + 8, input.addr());
        e.mem.set_u32(ebp + 0xc, out.addr());
        e.mem.set_u32(ebp + 0x10, pose.addr());
        let mut sp = base + 0x40;
        let (mut eax, mut ebx, mut ecx, mut edx, mut esi, mut edi) =
            (0u32, 0u32, this.addr(), 0u32, 0u32, 0u32);
        let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5, mut x6, mut x7) = (
            [0f32; 4], [0f32; 4], [0f32; 4], [0f32; 4], [0f32; 4], [0f32; 4], [0f32; 4], [0f32; 4],
        );
        let mut fpu = Fpu::new();
        let mut pushed: Vec<u32> = Vec::new();
        let mut test = 0u32;
        let mut cmp = (0u32, 0u32);
        let mut ah = 0u8;
        let mut block = 0x00cd_8d9fu32;
        'run: loop {
            match block {
                0x00cd8d9f => {
                    esi = ecx; // 00cd8d9f
                    edi = e.mem.u32(ebp.wrapping_add(0x8)); // 00cd8da1
                    x0 = ld(e, edi.wrapping_add(0x40)); // 00cd8da4
                    x2 = ld(e, edi.wrapping_add(0x30)); // 00cd8da8
                    x1 = x2; // 00cd8dac
                    x1 = mulps(x1, x0); // 00cd8daf
                    x5 = x1; // 00cd8db2
                    x5 = shufps(x5, x1, 0x55); // 00cd8db5
                    x5 = addss(x5, x1); // 00cd8db9
                    x4 = x1; // 00cd8dbd
                    x4 = shufps(x4, x1, 0xaa); // 00cd8dc0
                    x4 = addss(x4, x5); // 00cd8dc4
                    x1 = x2; // 00cd8dc8
                    x1 = shufps(x1, x2, 0xc9); // 00cd8dcb
                    x5 = x0; // 00cd8dcf
                    x5 = shufps(x5, x0, 0xd2); // 00cd8dd2
                    x1 = mulps(x1, x5); // 00cd8dd6
                    x5 = x2; // 00cd8dd9
                    x5 = shufps(x5, x2, 0xd2); // 00cd8ddc
                    x6 = x0; // 00cd8de0
                    x6 = shufps(x6, x0, 0xc9); // 00cd8de3
                    x5 = mulps(x5, x6); // 00cd8de7
                    x1 = subps(x1, x5); // 00cd8dea
                    x3 = x0; // 00cd8ded
                    x3 = shufps(x3, x0, 0xff); // 00cd8df0
                    x1 = mulps(x1, x3); // 00cd8df4
                    x5 = x3; // 00cd8df7
                    x5 = mulps(x5, x3); // 00cd8dfa
                    x5 = addps(x5, ld(e, 0x010c71e0u32)); // 00cd8dfd
                    x3 = ld(e, 0x010c7170u32); // 00cd8e04
                    x5 = mulps(x5, x2); // 00cd8e0b
                    x2 = x4; // 00cd8e0e
                    x2 = shufps(x2, x4, 0x0); // 00cd8e11
                    x2 = mulps(x2, x0); // 00cd8e15
                    x5 = addps(x5, x2); // 00cd8e18
                    x1 = addps(x1, x5); // 00cd8e1b
                    x0 = x1; // 00cd8e1e
                    x0 = addps(x0, x1); // 00cd8e21
                    x1 = ld(e, 0x010c44d0u32); // 00cd8e24
                    x0 = xorps(x0, x1); // 00cd8e2b
                    x1 = ld(e, 0x010c4c00u32); // 00cd8e2e
                    st(e, sp.wrapping_add(0xb0), x0); // 00cd8e35
                    x0 = ld(e, edi.wrapping_add(0x40)); // 00cd8e3d
                    x0 = xorps(x0, x1); // 00cd8e41
                    x1 = ld(e, edi.wrapping_add(0x50)); // 00cd8e44
                    x2 = x1; // 00cd8e48
                    x1 = unpckhps(x1, x3); // 00cd8e4b
                    x2 = shufps(x2, x1, 0xc4); // 00cd8e4e
                    x5 = rcpps(x2); // 00cd8e52
                    x1 = x5; // 00cd8e55
                    x1 = mulps(x1, x2); // 00cd8e58
                    x1 = mulps(x1, x5); // 00cd8e5b
                    x5 = addps(x5, x5); // 00cd8e5e
                    x5 = subps(x5, x1); // 00cd8e61
                    x1 = ld(e, 0x010c4c10u32); // 00cd8e64
                    x5 = andps(x5, x1); // 00cd8e6b
                    x3 = ld(e, esi.wrapping_add(0x40)); // 00cd8e6e
                    x1 = x0; // 00cd8e72
                    x1 = mulps(x1, x3); // 00cd8e75
                    x6 = x1; // 00cd8e78
                    x6 = shufps(x6, x1, 0x55); // 00cd8e7b
                    x6 = addss(x6, x1); // 00cd8e7f
                    x4 = x1; // 00cd8e83
                    x4 = shufps(x4, x1, 0xaa); // 00cd8e86
                    x4 = addss(x4, x6); // 00cd8e8a
                    x2 = x0; // 00cd8e8e
                    x6 = x0; // 00cd8e91
                    x1 = x0; // 00cd8e94
                    x2 = shufps(x2, x0, 0xff); // 00cd8e97
                    x6 = shufps(x6, x0, 0xc9); // 00cd8e9b
                    x1 = shufps(x1, x0, 0xd2); // 00cd8e9f
                    x7 = x3; // 00cd8ea3
                    st(e, sp.wrapping_add(0xc0), x0); // 00cd8ea6
                    st(e, sp.wrapping_add(0xd0), x5); // 00cd8eae
                    st(e, sp.wrapping_add(0x70), x2); // 00cd8eb6
                    st(e, sp.wrapping_add(0xa0), x6); // 00cd8ebb
                    st(e, sp.wrapping_add(0x80), x1); // 00cd8ec3
                    x7 = shufps(x7, x3, 0xc9); // 00cd8ecb
                    ebx = e.mem.u32(ebp.wrapping_add(0xc)); // 00cd8ecf
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00cd8ed2
                    x7 = mulps(x7, x1); // 00cd8ed5
                    x1 = x3; // 00cd8ed8
                    x1 = shufps(x1, x3, 0xd2); // 00cd8edb
                    x1 = mulps(x1, x6); // 00cd8edf
                    x6 = x2; // 00cd8ee2
                    x6 = mulps(x6, x2); // 00cd8ee5
                    x6 = addps(x6, ld(e, 0x010c71e0u32)); // 00cd8ee8
                    x6 = mulps(x6, x3); // 00cd8eef
                    x3 = x4; // 00cd8ef2
                    x3 = shufps(x3, x4, 0x0); // 00cd8ef5
                    x1 = subps(x1, x7); // 00cd8ef9
                    x3 = mulps(x3, x0); // 00cd8efc
                    x6 = addps(x6, x3); // 00cd8eff
                    x1 = mulps(x1, x2); // 00cd8f02
                    x1 = addps(x1, x6); // 00cd8f05
                    e.mem.set_u8(ebx.wrapping_add(0x4), (0x1u32) as u8); // 00cd8f08
                    x3 = ld(e, esi.wrapping_add(0x50)); // 00cd8f0c
                    edx = e.mem.u32(ecx.wrapping_add(0x1c)); // 00cd8f10
                    x1 = addps(x1, x1); // 00cd8f13
                    eax = edi.wrapping_add(0x10); // 00cd8f16
                    st(e, sp.wrapping_add(0x110), x1); // 00cd8f19
                    x1 = ld(e, edi); // 00cd8f21
                    x1 = mulps(x1, x3); // 00cd8f24
                    e.mem.set_u32(sp.wrapping_add(0x5c), eax); // 00cd8f27
                    eax = e.mem.u16(esi.wrapping_add(0x18)) as i16 as i32 as u32; // 00cd8f2b
                    test = (e.mem.u8(edx.wrapping_add(eax.wrapping_mul(0x4))) as u32) & 0x2u32; // 00cd8f2f
                    x3 = x1; // 00cd8f33
                    x3 = shufps(x3, x1, 0x55); // 00cd8f36
                    x4 = x1; // 00cd8f3a
                    x3 = addss(x3, x1); // 00cd8f3d
                    x4 = shufps(x4, x1, 0xaa); // 00cd8f41
                    x4 = addss(x4, x3); // 00cd8f45
                    e.mem.set_u32(sp.wrapping_add(0xc), x4[0].to_bits()); // 00cd8f49
                    if test == 0 {
                        block = 0x00cd8f6f;
                        continue;
                    } // 00cd8f4f
                    pushed.push(eax);
                    sp = sp.wrapping_sub(4); // 00cd8f51
                    eax = thiscall(e, 0x00cda180, ecx, &mut pushed).u32();
                    sp = sp.wrapping_add(4); // 00cd8f52
                    x2 = ld(e, sp.wrapping_add(0x70)); // 00cd8f57
                    x5 = ld(e, sp.wrapping_add(0xd0)); // 00cd8f5c
                    x0 = ld(e, sp.wrapping_add(0xc0)); // 00cd8f64
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00cd8f6c
                    block = 0x00cd8f6f;
                }
                0x00cd8f6f => {
                    eax = e.mem.u16(esi.wrapping_add(0x18)) as i16 as i32 as u32; // 00cd8f6f
                    edx = e.mem.u32(ecx.wrapping_add(0x1c)); // 00cd8f73
                    test = (e.mem.u8(edx.wrapping_add(eax.wrapping_mul(0x4))) as u32) & 0x2u32; // 00cd8f76
                    if test != 0 {
                        block = 0x00cd8f87;
                        continue;
                    } // 00cd8f7a
                    eax = eax.wrapping_add(eax.wrapping_mul(0x2)); // 00cd8f7c
                    eax = eax << 0x4u32; // 00cd8f7f
                    eax = eax.wrapping_add(e.mem.u32(ecx.wrapping_add(0x10))); // 00cd8f82
                    block = 0x00cd8fa2;
                    continue; // 00cd8f85
                }
                0x00cd8f87 => {
                    pushed.push(eax);
                    sp = sp.wrapping_sub(4); // 00cd8f87
                    eax = thiscall(e, 0x00cda180, ecx, &mut pushed).u32();
                    sp = sp.wrapping_add(4); // 00cd8f88
                    x2 = ld(e, sp.wrapping_add(0x70)); // 00cd8f8d
                    x5 = ld(e, sp.wrapping_add(0xd0)); // 00cd8f92
                    x0 = ld(e, sp.wrapping_add(0xc0)); // 00cd8f9a
                    block = 0x00cd8fa2;
                }
                0x00cd8fa2 => {
                    x4 = ld(e, eax.wrapping_add(0x20)); // 00cd8fa2
                    x3 = ld(e, eax); // 00cd8fa6
                    st(e, sp.wrapping_add(0x100), x4); // 00cd8fa9
                    x4 = ld(e, edi.wrapping_add(0x50)); // 00cd8fb1
                    x1 = ld(e, eax.wrapping_add(0x10)); // 00cd8fb5
                    eax = (eax & !0xffu32) | ((e.mem.u8(edi.wrapping_add(0x60)) as u32) & 0xff); // 00cd8fb9
                    st(e, sp.wrapping_add(0xe0), x3); // 00cd8fbc
                    x3 = mulps(x3, x4); // 00cd8fc4
                    x4 = ld(e, edi.wrapping_add(0x40)); // 00cd8fc7
                    st(e, sp.wrapping_add(0x20), x4); // 00cd8fcb
                    x6 = x4; // 00cd8fd0
                    x6 = shufps(x6, x4, 0xff); // 00cd8fd3
                    x4 = mulps(x4, x3); // 00cd8fd7
                    st(e, sp.wrapping_add(0x40), x6); // 00cd8fda
                    x6 = x4; // 00cd8fdf
                    x6 = shufps(x6, x4, 0x55); // 00cd8fe2
                    x6 = addss(x6, x4); // 00cd8fe6
                    x4 = shufps(x4, x4, 0xaa); // 00cd8fea
                    x4 = addss(x4, x6); // 00cd8fee
                    x6 = ld(e, sp.wrapping_add(0x20)); // 00cd8ff2
                    st(e, sp.wrapping_add(0x30), x4); // 00cd8ff7
                    x4 = x6; // 00cd8ffc
                    x4 = shufps(x4, x6, 0xc9); // 00cd8fff
                    x7 = x3; // 00cd9003
                    x7 = shufps(x7, x3, 0xd2); // 00cd9006
                    x4 = mulps(x4, x7); // 00cd900a
                    x7 = x6; // 00cd900d
                    x7 = shufps(x7, x6, 0xd2); // 00cd9010
                    x6 = x3; // 00cd9014
                    x6 = shufps(x6, x3, 0xc9); // 00cd9017
                    x7 = mulps(x7, x6); // 00cd901b
                    x6 = ld(e, sp.wrapping_add(0x40)); // 00cd901e
                    x4 = subps(x4, x7); // 00cd9023
                    x7 = x6; // 00cd9026
                    x7 = mulps(x7, x6); // 00cd9029
                    x7 = addps(x7, ld(e, 0x010c71e0u32)); // 00cd902c
                    x4 = mulps(x4, x6); // 00cd9033
                    x7 = mulps(x7, x3); // 00cd9036
                    x3 = ld(e, sp.wrapping_add(0x30)); // 00cd9039
                    x6 = x3; // 00cd903e
                    x6 = shufps(x6, x3, 0x0); // 00cd9041
                    x6 = mulps(x6, ld(e, sp.wrapping_add(0x20))); // 00cd9045
                    x3 = ld(e, edi.wrapping_add(0x30)); // 00cd904a
                    x7 = addps(x7, x6); // 00cd904e
                    x4 = addps(x4, x7); // 00cd9051
                    x4 = addps(x4, x4); // 00cd9054
                    x4 = addps(x4, x3); // 00cd9057
                    st(e, sp.wrapping_add(0xf0), x1); // 00cd905a
                    st(e, sp.wrapping_add(0x30), x4); // 00cd9062
                    e.mem.set_u8(sp.wrapping_add(0x1a), (eax & 0xff) as u8); // 00cd9067
                    test = (eax & 0xff) & (eax & 0xff); // 00cd906b
                    if test != 0 {
                        block = 0x00cd90d1;
                        continue;
                    } // 00cd906d
                    e.mem.set_u8(ebx.wrapping_add(0x4), (eax & 0xff) as u8); // 00cd906f
                    x0 = ld(e, esi.wrapping_add(0xa0)); // 00cd9072
                    st(e, sp.wrapping_add(0x60), x0); // 00cd9079
                    x0 = movss_load(e.mem.u32(esi.wrapping_add(0xb0))); // 00cd907e
                    x0 = shufps(x0, x0, 0x0); // 00cd9086
                    x0 = mulps(x0, ld(e, sp.wrapping_add(0x110))); // 00cd908a
                    x0 = addps(x0, ld(e, sp.wrapping_add(0xe0))); // 00cd9092
                    st(e, sp.wrapping_add(0x20), x0); // 00cd909a
                    x0 = ld(e, esi.wrapping_add(0x40)); // 00cd909f
                    x0 = mulps(x0, x4); // 00cd90a3
                    x1 = x0; // 00cd90a6
                    x1 = shufps(x1, x0, 0x55); // 00cd90a9
                    x2 = x0; // 00cd90ad
                    x1 = addss(x1, x0); // 00cd90b0
                    x2 = shufps(x2, x0, 0xaa); // 00cd90b4
                    x2 = addss(x2, x1); // 00cd90b8
                    e.mem.set_u32(sp.wrapping_add(0x10), x2[0].to_bits()); // 00cd90bc
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x10))); // 00cd90c2
                    fpu.fadd_m32(e.mem.u32(esi.wrapping_add(0xb0))); // 00cd90c6
                    block = 0x00cd94c6;
                    continue; // 00cd90cc
                }
                0x00cd90d1 => {
                    cmp = ((e.mem.u8(esi.wrapping_add(0xb4)) as u32), 0x0u32); // 00cd90d1
                    ecx = (ecx & !0xffu32) | u32::from(cmp.0 != cmp.1); // 00cd90d8
                    cmp = ((e.mem.u8(esi.wrapping_add(0x84)) as u32), 0x0u32); // 00cd90db
                    e.mem.set_u8(sp.wrapping_add(0x1b), (ecx & 0xff) as u8); // 00cd90e2
                    if cmp.0 == cmp.1 {
                        block = 0x00cd92eb;
                        continue;
                    } // 00cd90e6
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0xc))); // 00cd90ec
                    e.mem.set_u8(esi.wrapping_add(0xb4), (0x0u32) as u8); // 00cd90f0
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x64))); // 00cd90f7
                    fpu.fcompp(None); // 00cd90fa
                    ah = fpu.status; // 00cd90fc
                    test = (ah & (0x41)) as u32; // 00cd90fe
                    if test != 0 {
                        block = 0x00cd92eb;
                        continue;
                    } // 00cd9101
                    test = (ecx & 0xff) & (ecx & 0xff); // 00cd9107
                    if test == 0 {
                        block = 0x00cd91a3;
                        continue;
                    } // 00cd9109
                    x3 = ld(e, esi.wrapping_add(0xc0)); // 00cd910f
                    x5 = mulps(x5, x3); // 00cd9116
                    st(e, sp.wrapping_add(0x30), x3); // 00cd9119
                    x3 = x5; // 00cd911e
                    x3 = mulps(x3, x0); // 00cd9121
                    x6 = x3; // 00cd9124
                    x6 = shufps(x6, x3, 0x55); // 00cd9127
                    x6 = addss(x6, x3); // 00cd912b
                    x4 = x3; // 00cd912f
                    x4 = shufps(x4, x3, 0xaa); // 00cd9132
                    x4 = addss(x4, x6); // 00cd9136
                    x6 = x5; // 00cd913a
                    x6 = shufps(x6, x5, 0xc9); // 00cd913d
                    x6 = mulps(x6, ld(e, sp.wrapping_add(0x80))); // 00cd9141
                    x3 = x5; // 00cd9149
                    x3 = shufps(x3, x5, 0xd2); // 00cd914c
                    x3 = mulps(x3, ld(e, sp.wrapping_add(0xa0))); // 00cd9150
                    x3 = subps(x3, x6); // 00cd9158
                    x6 = x2; // 00cd915b
                    x6 = mulps(x6, x2); // 00cd915e
                    x3 = mulps(x3, x2); // 00cd9161
                    x2 = ld(e, 0x010c71e0u32); // 00cd9164
                    x6 = addps(x6, x2); // 00cd916b
                    x6 = mulps(x6, x5); // 00cd916e
                    x2 = x4; // 00cd9171
                    x2 = shufps(x2, x4, 0x0); // 00cd9174
                    x2 = mulps(x2, x0); // 00cd9178
                    x6 = addps(x6, x2); // 00cd917b
                    x3 = addps(x3, x6); // 00cd917e
                    x5 = x3; // 00cd9181
                    x5 = addps(x5, x3); // 00cd9184
                    x5 = addps(x5, ld(e, sp.wrapping_add(0xb0))); // 00cd9187
                    st(e, sp.wrapping_add(0xe0), x5); // 00cd918f
                    e.mem.set_u8(esi.wrapping_add(0xb4), (0x1u32) as u8); // 00cd9197
                    block = 0x00cd92f3;
                    continue; // 00cd919e
                }
                0x00cd91a3 => {
                    eax = e.mem.u32(esi.wrapping_add(0x30)); // 00cd91a3
                    ecx = e.mem.u32(esi.wrapping_add(0x34)); // 00cd91a6
                    edx = e.mem.u32(esi.wrapping_add(0x38)); // 00cd91a9
                    st(e, esi.wrapping_add(0xc0), x4); // 00cd91ac
                    e.mem.set_u32(sp.wrapping_add(0x20), eax); // 00cd91b3
                    eax = e.mem.u32(esi.wrapping_add(0x3c)); // 00cd91b7
                    e.mem.set_u32(sp.wrapping_add(0x24), ecx); // 00cd91ba
                    e.mem.set_u32(sp.wrapping_add(0x28), edx); // 00cd91be
                    e.mem.set_u32(sp.wrapping_add(0x2c), eax); // 00cd91c2
                    x0 = ld(e, sp.wrapping_add(0x20)); // 00cd91c6
                    x0 = mulps(x0, ld(e, sp.wrapping_add(0x100))); // 00cd91cb
                    x2 = x0; // 00cd91d3
                    x2 = mulps(x2, x1); // 00cd91d6
                    x5 = x2; // 00cd91d9
                    x5 = shufps(x5, x2, 0x55); // 00cd91dc
                    x5 = addss(x5, x2); // 00cd91e0
                    x4 = x2; // 00cd91e4
                    x4 = shufps(x4, x2, 0xaa); // 00cd91e7
                    x4 = addss(x4, x5); // 00cd91eb
                    x5 = x0; // 00cd91ef
                    x5 = shufps(x5, x0, 0xc9); // 00cd91f2
                    x2 = x1; // 00cd91f6
                    x2 = shufps(x2, x1, 0xd2); // 00cd91f9
                    x5 = mulps(x5, x2); // 00cd91fd
                    x2 = x0; // 00cd9200
                    x2 = shufps(x2, x0, 0xd2); // 00cd9203
                    x6 = x1; // 00cd9207
                    x6 = shufps(x6, x1, 0xc9); // 00cd920a
                    x2 = mulps(x2, x6); // 00cd920e
                    x2 = subps(x2, x5); // 00cd9211
                    x3 = x1; // 00cd9214
                    x3 = shufps(x3, x1, 0xff); // 00cd9217
                    x5 = x3; // 00cd921b
                    x5 = mulps(x5, x3); // 00cd921e
                    x5 = addps(x5, ld(e, 0x010c71e0u32)); // 00cd9221
                    x5 = mulps(x5, x0); // 00cd9228
                    x2 = mulps(x2, x3); // 00cd922b
                    x0 = x4; // 00cd922e
                    x0 = shufps(x0, x4, 0x0); // 00cd9231
                    x0 = mulps(x0, x1); // 00cd9235
                    x5 = addps(x5, x0); // 00cd9238
                    x2 = addps(x2, x5); // 00cd923b
                    x0 = x2; // 00cd923e
                    x0 = addps(x0, x2); // 00cd9241
                    x0 = addps(x0, ld(e, sp.wrapping_add(0xe0))); // 00cd9244
                    x2 = ld(e, edi.wrapping_add(0x50)); // 00cd924c
                    x0 = mulps(x0, x2); // 00cd9250
                    x2 = ld(e, edi.wrapping_add(0x40)); // 00cd9253
                    x3 = x0; // 00cd9257
                    x0 = mulps(x0, x2); // 00cd925a
                    x6 = x0; // 00cd925d
                    x6 = shufps(x6, x0, 0x55); // 00cd9260
                    x6 = addss(x6, x0); // 00cd9264
                    x5 = x0; // 00cd9268
                    x5 = shufps(x5, x0, 0xaa); // 00cd926b
                    x5 = addss(x5, x6); // 00cd926f
                    x6 = x3; // 00cd9273
                    x6 = shufps(x6, x3, 0xc9); // 00cd9276
                    x0 = x2; // 00cd927a
                    x0 = shufps(x0, x2, 0xd2); // 00cd927d
                    x6 = mulps(x6, x0); // 00cd9281
                    x0 = x3; // 00cd9284
                    x0 = shufps(x0, x3, 0xd2); // 00cd9287
                    x7 = x2; // 00cd928b
                    x7 = shufps(x7, x2, 0xc9); // 00cd928e
                    x0 = mulps(x0, x7); // 00cd9292
                    x0 = subps(x0, x6); // 00cd9295
                    x4 = x2; // 00cd9298
                    x4 = shufps(x4, x2, 0xff); // 00cd929b
                    x6 = x4; // 00cd929f
                    x6 = mulps(x6, x4); // 00cd92a2
                    x6 = addps(x6, ld(e, 0x010c71e0u32)); // 00cd92a5
                    x6 = mulps(x6, x3); // 00cd92ac
                    x3 = x5; // 00cd92af
                    x3 = shufps(x3, x5, 0x0); // 00cd92b2
                    x0 = mulps(x0, x4); // 00cd92b6
                    x3 = mulps(x3, x2); // 00cd92b9
                    x6 = addps(x6, x3); // 00cd92bc
                    x0 = addps(x0, x6); // 00cd92bf
                    x0 = addps(x0, x0); // 00cd92c2
                    st(e, esi.wrapping_add(0xd0), x0); // 00cd92c5
                    x2 = ld(e, edi.wrapping_add(0x30)); // 00cd92cc
                    x5 = ld(e, sp.wrapping_add(0xe0)); // 00cd92d0
                    x0 = addps(x0, x2); // 00cd92d8
                    st(e, esi.wrapping_add(0xd0), x0); // 00cd92db
                    e.mem.set_u8(esi.wrapping_add(0xb4), (0x1u32) as u8); // 00cd92e2
                    block = 0x00cd92f3;
                    continue; // 00cd92e9
                }
                0x00cd92eb => {
                    x5 = ld(e, sp.wrapping_add(0xe0)); // 00cd92eb
                    block = 0x00cd92f3;
                }
                0x00cd92f3 => {
                    cmp = ((e.mem.u8(esi.wrapping_add(0x84)) as u32), 0x0u32); // 00cd92f3
                    ecx = e.mem.u32(esi.wrapping_add(0x30)); // 00cd92fa
                    edx = e.mem.u32(esi.wrapping_add(0x34)); // 00cd92fd
                    eax = e.mem.u32(esi.wrapping_add(0x38)); // 00cd9300
                    e.mem.set_u32(sp.wrapping_add(0x20), ecx); // 00cd9303
                    ecx = e.mem.u32(esi.wrapping_add(0x3c)); // 00cd9307
                    e.mem.set_u32(sp.wrapping_add(0x24), edx); // 00cd930a
                    e.mem.set_u32(sp.wrapping_add(0x28), eax); // 00cd930e
                    e.mem.set_u32(sp.wrapping_add(0x2c), ecx); // 00cd9312
                    x0 = ld(e, sp.wrapping_add(0x20)); // 00cd9316
                    x0 = mulps(x0, ld(e, sp.wrapping_add(0x100))); // 00cd931b
                    x2 = x0; // 00cd9323
                    x2 = mulps(x2, x1); // 00cd9326
                    x6 = x2; // 00cd9329
                    x6 = shufps(x6, x2, 0x55); // 00cd932c
                    x6 = addss(x6, x2); // 00cd9330
                    x4 = x2; // 00cd9334
                    x4 = shufps(x4, x2, 0xaa); // 00cd9337
                    x7 = x0; // 00cd933b
                    x7 = shufps(x7, x0, 0xc9); // 00cd933e
                    x4 = addss(x4, x6); // 00cd9342
                    x2 = x1; // 00cd9346
                    x2 = shufps(x2, x1, 0xd2); // 00cd9349
                    x7 = mulps(x7, x2); // 00cd934d
                    st(e, sp.wrapping_add(0x120), x2); // 00cd9350
                    x2 = x0; // 00cd9358
                    x2 = shufps(x2, x0, 0xd2); // 00cd935b
                    x6 = x1; // 00cd935f
                    x6 = shufps(x6, x1, 0xc9); // 00cd9362
                    x2 = mulps(x2, x6); // 00cd9366
                    x2 = subps(x2, x7); // 00cd9369
                    x3 = x1; // 00cd936c
                    x3 = shufps(x3, x1, 0xff); // 00cd936f
                    st(e, sp.wrapping_add(0x130), x6); // 00cd9373
                    x6 = ld(e, 0x010c71e0u32); // 00cd937b
                    x7 = x3; // 00cd9382
                    x7 = mulps(x7, x3); // 00cd9385
                    x7 = addps(x7, x6); // 00cd9388
                    x7 = mulps(x7, x0); // 00cd938b
                    x0 = x4; // 00cd938e
                    x2 = mulps(x2, x3); // 00cd9391
                    x0 = shufps(x0, x4, 0x0); // 00cd9394
                    x0 = mulps(x0, x1); // 00cd9398
                    x7 = addps(x7, x0); // 00cd939b
                    x2 = addps(x2, x7); // 00cd939e
                    x2 = addps(x2, x2); // 00cd93a1
                    st(e, sp.wrapping_add(0x90), x3); // 00cd93a4
                    x2 = addps(x2, x5); // 00cd93ac
                    if cmp.0 == cmp.1 {
                        block = 0x00cd93c3;
                        continue;
                    } // 00cd93af
                    cmp = ((e.mem.u8(esi.wrapping_add(0xb4)) as u32), 0x0u32); // 00cd93b1
                    if cmp.0 == cmp.1 {
                        block = 0x00cd93c3;
                        continue;
                    } // 00cd93b8
                    x0 = ld(e, esi.wrapping_add(0xd0)); // 00cd93ba
                    block = 0x00cd9442;
                    continue; // 00cd93c1
                }
                0x00cd93c3 => {
                    x0 = ld(e, edi.wrapping_add(0x50)); // 00cd93c3
                    x2 = mulps(x2, x0); // 00cd93c7
                    x0 = ld(e, edi.wrapping_add(0x40)); // 00cd93ca
                    x1 = x2; // 00cd93ce
                    x1 = mulps(x1, x0); // 00cd93d1
                    x5 = x1; // 00cd93d4
                    x5 = shufps(x5, x1, 0x55); // 00cd93d7
                    x5 = addss(x5, x1); // 00cd93db
                    x4 = x1; // 00cd93df
                    x4 = shufps(x4, x1, 0xaa); // 00cd93e2
                    x4 = addss(x4, x5); // 00cd93e6
                    x5 = x2; // 00cd93ea
                    x5 = shufps(x5, x2, 0xc9); // 00cd93ed
                    x1 = x0; // 00cd93f1
                    x1 = shufps(x1, x0, 0xd2); // 00cd93f4
                    x5 = mulps(x5, x1); // 00cd93f8
                    x1 = x2; // 00cd93fb
                    x1 = shufps(x1, x2, 0xd2); // 00cd93fe
                    x7 = x0; // 00cd9402
                    x7 = shufps(x7, x0, 0xc9); // 00cd9405
                    x3 = x0; // 00cd9409
                    x3 = shufps(x3, x0, 0xff); // 00cd940c
                    x1 = mulps(x1, x7); // 00cd9410
                    x1 = subps(x1, x5); // 00cd9413
                    x5 = x3; // 00cd9416
                    x5 = mulps(x5, x3); // 00cd9419
                    x5 = addps(x5, x6); // 00cd941c
                    x5 = mulps(x5, x2); // 00cd941f
                    x2 = x4; // 00cd9422
                    x2 = shufps(x2, x4, 0x0); // 00cd9425
                    x2 = mulps(x2, x0); // 00cd9429
                    x1 = mulps(x1, x3); // 00cd942c
                    x5 = addps(x5, x2); // 00cd942f
                    x1 = addps(x1, x5); // 00cd9432
                    x0 = x1; // 00cd9435
                    x0 = addps(x0, x1); // 00cd9438
                    x1 = ld(e, edi.wrapping_add(0x30)); // 00cd943b
                    x0 = addps(x0, x1); // 00cd943f
                    block = 0x00cd9442;
                }
                0x00cd9442 => {
                    edx = sp.wrapping_add(0x60); // 00cd9442
                    pushed.push(edx);
                    sp = sp.wrapping_sub(4); // 00cd9446
                    eax = sp.wrapping_add(0x44); // 00cd9447
                    pushed.push(eax);
                    sp = sp.wrapping_sub(4); // 00cd944b
                    ecx = sp.wrapping_add(0x28); // 00cd944c
                    pushed.push(ecx);
                    sp = sp.wrapping_sub(4); // 00cd9450
                    edx = sp.wrapping_add(0x3c); // 00cd9451
                    pushed.push(edx);
                    sp = sp.wrapping_sub(4); // 00cd9455
                    pushed.push(edi);
                    sp = sp.wrapping_sub(4); // 00cd9456
                    ecx = esi; // 00cd9457
                    st(e, sp.wrapping_add(0x34), x0); // 00cd9459
                    {
                        let a = take_args(&mut pushed);
                        let hit = fn_00cd8a50(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(a[0]),
                            Ptr::new(a[1]),
                            Ptr::new(a[2]),
                            Ptr::new(a[3]),
                            Ptr::new(a[4]),
                        );
                        eax = (eax & !0xffu32) | u32::from(hit);
                        sp = sp.wrapping_add(0x14);
                    } // 00cd945e
                    test = (eax & 0xff) & (eax & 0xff); // 00cd9463
                    if test != 0 {
                        block = 0x00cd94de;
                        continue;
                    } // 00cd9465
                    e.mem.set_u8(ebx.wrapping_add(0x4), (eax & 0xff) as u8); // 00cd9467
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0xb0))); // 00cd946a
                    x0 = ld(e, esi.wrapping_add(0xa0)); // 00cd9470
                    st(e, sp.wrapping_add(0x60), x0); // 00cd9477
                    x0 = movss_load(e.mem.u32(esi.wrapping_add(0xb0))); // 00cd947c
                    x0 = shufps(x0, x0, 0x0); // 00cd9484
                    x0 = mulps(x0, ld(e, sp.wrapping_add(0x110))); // 00cd9488
                    x0 = addps(x0, ld(e, sp.wrapping_add(0xe0))); // 00cd9490
                    st(e, sp.wrapping_add(0x20), x0); // 00cd9498
                    x0 = ld(e, esi.wrapping_add(0x40)); // 00cd949d
                    x0 = mulps(x0, ld(e, sp.wrapping_add(0x30))); // 00cd94a1
                    x1 = x0; // 00cd94a6
                    x1 = shufps(x1, x0, 0x55); // 00cd94a9
                    x2 = x0; // 00cd94ad
                    x1 = addss(x1, x0); // 00cd94b0
                    x2 = shufps(x2, x0, 0xaa); // 00cd94b4
                    x2 = addss(x2, x1); // 00cd94b8
                    e.mem.set_u32(sp.wrapping_add(0x10), x2[0].to_bits()); // 00cd94bc
                    fpu.fadd_m32(e.mem.u32(sp.wrapping_add(0x10))); // 00cd94c2
                    block = 0x00cd94c6;
                }
                0x00cd94c6 => {
                    fpu.fsub_m32(e.mem.u32(sp.wrapping_add(0xc))); // 00cd94c6
                    e.mem.set_u32(sp.wrapping_add(0xc), fpu.fstp32()); // 00cd94ca
                    fpu.push(0.0); // 00cd94ce
                    e.mem.set_u32(ebx, fpu.fst32()); // 00cd94d0
                    e.mem.set_u8(esi.wrapping_add(0xb4), (0x0u32) as u8); // 00cd94d2
                    block = 0x00cd9a8f;
                    continue; // 00cd94d9
                }
                0x00cd94de => {
                    cmp = ((e.mem.u8(esi.wrapping_add(0xb4)) as u32), 0x0u32); // 00cd94de
                    x1 = ld(e, sp.wrapping_add(0x40)); // 00cd94e5
                    if cmp.0 == cmp.1 {
                        block = 0x00cd96bb;
                        continue;
                    } // 00cd94ea
                    x5 = ld(e, sp.wrapping_add(0xc0)); // 00cd94f0
                    x0 = ld(e, sp.wrapping_add(0xd0)); // 00cd94f8
                    x0 = mulps(x0, x1); // 00cd9500
                    eax = e.mem.u32(esi.wrapping_add(0x30)); // 00cd9503
                    st(e, esi.wrapping_add(0xc0), x1); // 00cd9506
                    x4 = ld(e, 0x010c71e0u32); // 00cd950d
                    st(e, sp.wrapping_add(0x30), x1); // 00cd9514
                    ecx = e.mem.u32(esi.wrapping_add(0x34)); // 00cd9519
                    edx = e.mem.u32(esi.wrapping_add(0x38)); // 00cd951c
                    x1 = x5; // 00cd951f
                    x1 = mulps(x1, x0); // 00cd9522
                    x3 = x1; // 00cd9525
                    x3 = shufps(x3, x1, 0x55); // 00cd9528
                    x3 = addss(x3, x1); // 00cd952c
                    x2 = x1; // 00cd9530
                    x2 = shufps(x2, x1, 0xaa); // 00cd9533
                    x2 = addss(x2, x3); // 00cd9537
                    x3 = x0; // 00cd953b
                    x3 = shufps(x3, x0, 0xc9); // 00cd953e
                    x3 = mulps(x3, ld(e, sp.wrapping_add(0x80))); // 00cd9542
                    x1 = x0; // 00cd954a
                    x1 = shufps(x1, x0, 0xd2); // 00cd954d
                    x1 = mulps(x1, ld(e, sp.wrapping_add(0xa0))); // 00cd9551
                    x1 = subps(x1, x3); // 00cd9559
                    x3 = ld(e, sp.wrapping_add(0x70)); // 00cd955c
                    x1 = mulps(x1, x3); // 00cd9561
                    x6 = x3; // 00cd9564
                    x6 = mulps(x6, x3); // 00cd9567
                    x6 = addps(x6, x4); // 00cd956a
                    x6 = mulps(x6, x0); // 00cd956d
                    x0 = x2; // 00cd9570
                    x0 = shufps(x0, x2, 0x0); // 00cd9573
                    x0 = mulps(x0, x5); // 00cd9577
                    x6 = addps(x6, x0); // 00cd957a
                    x1 = addps(x1, x6); // 00cd957d
                    x6 = ld(e, sp.wrapping_add(0xf0)); // 00cd9580
                    x5 = x1; // 00cd9588
                    x5 = addps(x5, x1); // 00cd958b
                    x1 = ld(e, sp.wrapping_add(0x100)); // 00cd958e
                    x5 = addps(x5, ld(e, sp.wrapping_add(0xb0))); // 00cd9596
                    e.mem.set_u32(sp.wrapping_add(0x20), eax); // 00cd959e
                    eax = e.mem.u32(esi.wrapping_add(0x3c)); // 00cd95a2
                    e.mem.set_u32(sp.wrapping_add(0x24), ecx); // 00cd95a5
                    e.mem.set_u32(sp.wrapping_add(0x28), edx); // 00cd95a9
                    e.mem.set_u32(sp.wrapping_add(0x2c), eax); // 00cd95ad
                    x0 = ld(e, sp.wrapping_add(0x20)); // 00cd95b1
                    x1 = mulps(x1, x0); // 00cd95b6
                    x0 = x6; // 00cd95b9
                    x0 = mulps(x0, x1); // 00cd95bc
                    x3 = x0; // 00cd95bf
                    x3 = shufps(x3, x0, 0x55); // 00cd95c2
                    x3 = addss(x3, x0); // 00cd95c6
                    x2 = x0; // 00cd95ca
                    x2 = shufps(x2, x0, 0xaa); // 00cd95cd
                    x2 = addss(x2, x3); // 00cd95d1
                    x3 = x1; // 00cd95d5
                    x3 = shufps(x3, x1, 0xc9); // 00cd95d8
                    x3 = mulps(x3, ld(e, sp.wrapping_add(0x120))); // 00cd95dc
                    x0 = x1; // 00cd95e4
                    x0 = shufps(x0, x1, 0xd2); // 00cd95e7
                    x0 = mulps(x0, ld(e, sp.wrapping_add(0x130))); // 00cd95eb
                    x0 = subps(x0, x3); // 00cd95f3
                    x3 = ld(e, sp.wrapping_add(0x90)); // 00cd95f6
                    x7 = x3; // 00cd95fe
                    x7 = mulps(x7, x3); // 00cd9601
                    x7 = addps(x7, x4); // 00cd9604
                    x7 = mulps(x7, x1); // 00cd9607
                    x0 = mulps(x0, x3); // 00cd960a
                    x1 = x2; // 00cd960d
                    x1 = shufps(x1, x2, 0x0); // 00cd9610
                    x1 = mulps(x1, x6); // 00cd9614
                    x7 = addps(x7, x1); // 00cd9617
                    x1 = ld(e, edi.wrapping_add(0x50)); // 00cd961a
                    x0 = addps(x0, x7); // 00cd961e
                    x0 = addps(x0, x0); // 00cd9621
                    x0 = addps(x0, x5); // 00cd9624
                    x0 = mulps(x0, x1); // 00cd9627
                    x1 = ld(e, edi.wrapping_add(0x40)); // 00cd962a
                    x3 = x1; // 00cd962e
                    x2 = x0; // 00cd9631
                    x3 = shufps(x3, x1, 0xff); // 00cd9634
                    x0 = x1; // 00cd9638
                    x0 = mulps(x0, x2); // 00cd963b
                    x6 = x0; // 00cd963e
                    x6 = shufps(x6, x0, 0x55); // 00cd9641
                    x6 = addss(x6, x0); // 00cd9645
                    x4 = x0; // 00cd9649
                    x4 = shufps(x4, x0, 0xaa); // 00cd964c
                    x4 = addss(x4, x6); // 00cd9650
                    x6 = x2; // 00cd9654
                    x6 = shufps(x6, x2, 0xc9); // 00cd9657
                    x0 = x1; // 00cd965b
                    x0 = shufps(x0, x1, 0xd2); // 00cd965e
                    x6 = mulps(x6, x0); // 00cd9662
                    x0 = x2; // 00cd9665
                    x0 = shufps(x0, x2, 0xd2); // 00cd9668
                    x7 = x1; // 00cd966c
                    x7 = shufps(x7, x1, 0xc9); // 00cd966f
                    x0 = mulps(x0, x7); // 00cd9673
                    x0 = subps(x0, x6); // 00cd9676
                    x6 = x3; // 00cd9679
                    x6 = mulps(x6, x3); // 00cd967c
                    x6 = addps(x6, ld(e, 0x010c71e0u32)); // 00cd967f
                    x6 = mulps(x6, x2); // 00cd9686
                    x2 = x4; // 00cd9689
                    x0 = mulps(x0, x3); // 00cd968c
                    x2 = shufps(x2, x4, 0x0); // 00cd968f
                    x2 = mulps(x2, x1); // 00cd9693
                    x6 = addps(x6, x2); // 00cd9696
                    x0 = addps(x0, x6); // 00cd9699
                    x0 = addps(x0, x0); // 00cd969c
                    st(e, esi.wrapping_add(0xd0), x0); // 00cd969f
                    x1 = ld(e, edi.wrapping_add(0x30)); // 00cd96a6
                    x0 = addps(x0, x1); // 00cd96aa
                    x1 = ld(e, sp.wrapping_add(0x40)); // 00cd96ad
                    st(e, esi.wrapping_add(0xd0), x0); // 00cd96b2
                    block = 0x00cd96c3;
                    continue; // 00cd96b9
                }
                0x00cd96bb => {
                    x5 = ld(e, sp.wrapping_add(0xe0)); // 00cd96bb
                    block = 0x00cd96c3;
                }
                0x00cd96c3 => {
                    e.mem.set_u8(ebx.wrapping_add(0x4), (0x1u32) as u8); // 00cd96c3
                    x2 = ld(e, esi.wrapping_add(0x40)); // 00cd96c7
                    x0 = x2; // 00cd96cb
                    x0 = mulps(x0, x1); // 00cd96ce
                    x4 = x0; // 00cd96d1
                    x4 = shufps(x4, x0, 0x55); // 00cd96d4
                    x4 = addss(x4, x0); // 00cd96d8
                    x6 = x0; // 00cd96dc
                    x6 = shufps(x6, x0, 0xaa); // 00cd96df
                    x0 = ld(e, edi.wrapping_add(0x30)); // 00cd96e3
                    x2 = mulps(x2, x0); // 00cd96e7
                    x0 = x2; // 00cd96ea
                    x6 = addss(x6, x4); // 00cd96ed
                    x2 = shufps(x2, x0, 0x55); // 00cd96f1
                    x2 = addss(x2, x0); // 00cd96f5
                    e.mem.set_u32(sp.wrapping_add(0x10), x6[0].to_bits()); // 00cd96f9
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x10))); // 00cd96ff
                    fpu.fld_st(0); // 00cd9703
                    x4 = x0; // 00cd9705
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x60))); // 00cd9708
                    x4 = shufps(x4, x0, 0xaa); // 00cd970b
                    x4 = addss(x4, x2); // 00cd970f
                    e.mem.set_u32(sp.wrapping_add(0x14), x4[0].to_bits()); // 00cd9713
                    fpu.fadd_m32(e.mem.u32(sp.wrapping_add(0x14))); // 00cd9719
                    x3 = ld(e, sp.wrapping_add(0x70)); // 00cd971d
                    fpu.fsubp(1); // 00cd9722
                    e.mem.set_u32(ebx, fpu.fstp32()); // 00cd9724
                    x4 = ld(e, esi.wrapping_add(0x40)); // 00cd9726
                    fpu.push(0.0); // 00cd972a
                    e.mem.set_u32(sp.wrapping_add(0x6c), fpu.fst32()); // 00cd972c
                    fpu.fld_st(1); // 00cd9730
                    fpu.fchs(); // 00cd9732
                    e.mem.set_u32(sp.wrapping_add(0x14), fpu.fstp32()); // 00cd9734
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0xc))); // 00cd9738
                    x0 = movss_load(e.mem.u32(sp.wrapping_add(0x14))); // 00cd973c
                    fpu.fld_st(0); // 00cd9742
                    x2 = x0; // 00cd9744
                    fpu.fsub_m32(e.mem.u32(esi.wrapping_add(0x64))); // 00cd9747
                    x2 = shufps(x2, x2, 0x0); // 00cd974a
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x68))); // 00cd974e
                    x2 = mulps(x2, x4); // 00cd9751
                    fpu.fsub_m32(e.mem.u32(esi.wrapping_add(0x64))); // 00cd9754
                    x2 = addps(x2, x1); // 00cd9757
                    fpu.fdivp(1); // 00cd975a
                    fpu.push(1.0); // 00cd975c
                    fpu.fld_st(0); // 00cd975e
                    fpu.fsubrp(2); // 00cd9760
                    fpu.fxch(1); // 00cd9762
                    e.mem.set_u32(sp.wrapping_add(0xc), fpu.fstp32()); // 00cd9764
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0xc))); // 00cd9768
                    fpu.fcom(Some(Operand::Double(e.global::<f64>(0x01012060u32)))); // 00cd976c
                    ah = fpu.status; // 00cd9772
                    test = (ah & (0x5)) as u32; // 00cd9774
                    if (test as u8).count_ones() % 2 == 0 {
                        block = 0x00cd978b;
                        continue;
                    } // 00cd9777
                    fpu.pop(); // 00cd9779
                    fpu.fxch(2); // 00cd977b
                    e.mem.set_u32(sp.wrapping_add(0xc), fpu.fst32()); // 00cd977d
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0xc))); // 00cd9781
                    fpu.fxch(1); // 00cd9785
                    fpu.fxch(3); // 00cd9787
                    fpu.fxch(1); // 00cd9789
                    block = 0x00cd978b;
                }
                0x00cd978b => {
                    fpu.fcom(None); // 00cd978b
                    ah = fpu.status; // 00cd978d
                    fpu.push(1.0); // 00cd978f
                    test = (ah & (0x41)) as u32; // 00cd9791
                    if test != 0 {
                        block = 0x00cd97a2;
                        continue;
                    } // 00cd9794
                    fpu.fstp_st(1); // 00cd9796
                    e.mem.set_u32(sp.wrapping_add(0xc), fpu.fst32()); // 00cd9798
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0xc))); // 00cd979c
                    fpu.fxch(1); // 00cd97a0
                    block = 0x00cd97a2;
                }
                0x00cd97a2 => {
                    fpu.fld_st(1); // 00cd97a2
                    fpu.fsubr_st(0, 3); // 00cd97a4
                    fpu.fmul_m32(e.mem.u32(edi.wrapping_add(0x78))); // 00cd97a6
                    fpu.fld32(e.mem.u32(edi.wrapping_add(0x74))); // 00cd97a9
                    fpu.fmulp(3); // 00cd97ac
                    fpu.faddp(2); // 00cd97ae
                    fpu.fxch(1); // 00cd97b0
                    e.mem.set_u32(sp.wrapping_add(0xc), fpu.fstp32()); // 00cd97b2
                    fpu.fld_st(3); // 00cd97b6
                    fpu.fcomp(Some(Operand::Float(e.mem.u32(esi.wrapping_add(0x90))))); // 00cd97b8
                    ah = fpu.status; // 00cd97be
                    test = (ah & (0x44)) as u32; // 00cd97c0
                    if (test as u8).count_ones() % 2 == 1 {
                        block = 0x00cd9802;
                        continue;
                    } // 00cd97c3
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x94))); // 00cd97c5
                    fpu.fcomp(Some(Operand::Register(5))); // 00cd97cb
                    ah = fpu.status; // 00cd97cd
                    test = (ah & (0x41)) as u32; // 00cd97cf
                    if test != 0 {
                        block = 0x00cd97d9;
                        continue;
                    } // 00cd97d2
                    fpu.fld32(e.mem.u32(edi.wrapping_add(0x70))); // 00cd97d4
                    block = 0x00cd97dc;
                    continue; // 00cd97d7
                }
                0x00cd97d9 => {
                    fpu.fld32(e.mem.u32(edi.wrapping_add(0x6c))); // 00cd97d9
                    block = 0x00cd97dc;
                }
                0x00cd97dc => {
                    e.mem.set_u32(sp.wrapping_add(0x10), fpu.fstp32()); // 00cd97dc
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x10))); // 00cd97e0
                    fpu.fmul_m32(e.mem.u32(sp.wrapping_add(0xc))); // 00cd97e4
                    e.mem.set_u32(sp.wrapping_add(0x14), fpu.fstp32()); // 00cd97e8
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x14))); // 00cd97ec
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x94))); // 00cd97f0
                    fpu.fsubp(6); // 00cd97f6
                    fpu.fmulp(5); // 00cd97f8
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x94))); // 00cd97fa
                    fpu.faddp(5); // 00cd9800
                    block = 0x00cd9802;
                }
                0x00cd9802 => {
                    cmp = ((e.mem.u8(esi.wrapping_add(0x84)) as u32), 0x0u32); // 00cd9802
                    fpu.fxch(4); // 00cd9809
                    e.mem.set_u32(sp.wrapping_add(0xc), fpu.fstp32()); // 00cd980b
                    if cmp.0 == cmp.1 {
                        block = 0x00cd9828;
                        continue;
                    } // 00cd980f
                    cmp = ((e.mem.u8(esi.wrapping_add(0xb4)) as u32), 0x0u32); // 00cd9811
                    if cmp.0 == cmp.1 {
                        block = 0x00cd9954;
                        continue;
                    } // 00cd9818
                    x0 = xorps(x0, x0); // 00cd981e
                    st(e, esi.wrapping_add(0xe0), x0); // 00cd9821
                    block = 0x00cd9828;
                }
                0x00cd9828 => {
                    fpu.pop(); // 00cd9828
                    block = 0x00cd982a;
                }
                0x00cd982a => {
                    x0 = movss_load(e.mem.u32(sp.wrapping_add(0xc))); // 00cd982a
                    fpu.fld_st(0); // 00cd9830
                    x1 = ld(e, esi.wrapping_add(0x40)); // 00cd9832
                    fpu.fsub_m32(e.mem.u32(esi.wrapping_add(0x60))); // 00cd9836
                    x4 = ld(e, sp.wrapping_add(0xc0)); // 00cd9839
                    x0 = shufps(x0, x0, 0x0); // 00cd9841
                    x0 = mulps(x0, x1); // 00cd9845
                    e.mem.set_u32(sp.wrapping_add(0x14), fpu.fstp32()); // 00cd9848
                    x0 = addps(x0, x2); // 00cd984c
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x6c))); // 00cd984f
                    x0 = mulps(x0, ld(e, sp.wrapping_add(0xd0))); // 00cd9852
                    e.mem.set_u32(sp.wrapping_add(0x10), fpu.fstp32()); // 00cd985a
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x70))); // 00cd985e
                    x1 = x4; // 00cd9861
                    x1 = mulps(x1, x0); // 00cd9864
                    e.mem.set_u32(sp.wrapping_add(0x1c), fpu.fstp32()); // 00cd9867
                    x6 = x1; // 00cd986b
                    x6 = shufps(x6, x1, 0x55); // 00cd986e
                    x6 = addss(x6, x1); // 00cd9872
                    x2 = x1; // 00cd9876
                    x2 = shufps(x2, x1, 0xaa); // 00cd9879
                    x2 = addss(x2, x6); // 00cd987d
                    x6 = x0; // 00cd9881
                    x6 = shufps(x6, x0, 0xc9); // 00cd9884
                    x6 = mulps(x6, ld(e, sp.wrapping_add(0x80))); // 00cd9888
                    x1 = x0; // 00cd9890
                    x1 = shufps(x1, x0, 0xd2); // 00cd9893
                    x1 = mulps(x1, ld(e, sp.wrapping_add(0xa0))); // 00cd9897
                    x1 = subps(x1, x6); // 00cd989f
                    x1 = mulps(x1, x3); // 00cd98a2
                    x6 = x3; // 00cd98a5
                    x6 = mulps(x6, x3); // 00cd98a8
                    x3 = ld(e, 0x010c71e0u32); // 00cd98ab
                    x6 = addps(x6, x3); // 00cd98b2
                    x3 = ld(e, sp.wrapping_add(0x110)); // 00cd98b5
                    x6 = mulps(x6, x0); // 00cd98bd
                    x0 = x2; // 00cd98c0
                    x0 = shufps(x0, x2, 0x0); // 00cd98c3
                    x0 = mulps(x0, x4); // 00cd98c7
                    x6 = addps(x6, x0); // 00cd98ca
                    x0 = movss_load(e.mem.u32(sp.wrapping_add(0x14))); // 00cd98cd
                    x1 = addps(x1, x6); // 00cd98d3
                    x2 = x1; // 00cd98d6
                    x2 = addps(x2, x1); // 00cd98d9
                    x2 = addps(x2, ld(e, sp.wrapping_add(0xb0))); // 00cd98dc
                    x0 = shufps(x0, x0, 0x0); // 00cd98e4
                    x0 = mulps(x0, x3); // 00cd98e8
                    x0 = addps(x0, x2); // 00cd98eb
                    x2 = ld(e, esi.wrapping_add(0x50)); // 00cd98ee
                    x1 = x0; // 00cd98f2
                    x1 = mulps(x1, x2); // 00cd98f5
                    x4 = x1; // 00cd98f8
                    x4 = shufps(x4, x1, 0x55); // 00cd98fb
                    x6 = x1; // 00cd98ff
                    x4 = addss(x4, x1); // 00cd9902
                    x6 = shufps(x6, x1, 0xaa); // 00cd9906
                    x6 = addss(x6, x4); // 00cd990a
                    e.mem.set_u32(sp.wrapping_add(0x14), x6[0].to_bits()); // 00cd990e
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x14))); // 00cd9914
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x10))); // 00cd9918
                    st(e, sp.wrapping_add(0x20), x0); // 00cd991c
                    fpu.fcom(None); // 00cd9921
                    ah = fpu.status; // 00cd9923
                    test = (ah & (0x5)) as u32; // 00cd9925
                    if (test as u8).count_ones() % 2 == 0 {
                        block = 0x00cd99da;
                        continue;
                    } // 00cd9928
                    fpu.fsub_st(0, 1); // 00cd992e
                    e.mem.set_u32(sp.wrapping_add(0x14), fpu.fstp32()); // 00cd9930
                    x1 = movss_load(e.mem.u32(sp.wrapping_add(0x14))); // 00cd9934
                    x4 = x1; // 00cd993a
                    x4 = shufps(x4, x1, 0x0); // 00cd993d
                    x4 = mulps(x4, x2); // 00cd9941
                    x4 = addps(x4, x0); // 00cd9944
                    x0 = x4; // 00cd9947
                    st(e, sp.wrapping_add(0x20), x0); // 00cd994a
                    block = 0x00cd99dc;
                    continue; // 00cd994f
                }
                0x00cd9954 => {
                    cmp = ((e.mem.u8(sp.wrapping_add(0x1b)) as u32), 0x0u32); // 00cd9954
                    if cmp.0 == cmp.1 {
                        block = 0x00cd99a0;
                        continue;
                    } // 00cd9959
                    x1 = ld(e, esi.wrapping_add(0xc0)); // 00cd995b
                    x1 = subps(x1, x2); // 00cd9962
                    st(e, esi.wrapping_add(0xe0), x1); // 00cd9965
                    x0 = x4; // 00cd996c
                    x0 = mulps(x0, x1); // 00cd996f
                    x1 = x0; // 00cd9972
                    x1 = shufps(x1, x0, 0x55); // 00cd9975
                    x1 = addss(x1, x0); // 00cd9979
                    x0 = shufps(x0, x0, 0xaa); // 00cd997d
                    x0 = addss(x0, x1); // 00cd9981
                    x1 = x0; // 00cd9985
                    x1 = shufps(x1, x0, 0x0); // 00cd9988
                    x0 = ld(e, esi.wrapping_add(0xe0)); // 00cd998c
                    x1 = mulps(x1, x4); // 00cd9993
                    x0 = subps(x0, x1); // 00cd9996
                    st(e, esi.wrapping_add(0xe0), x0); // 00cd9999
                    block = 0x00cd99a0;
                }
                0x00cd99a0 => {
                    fpu.fsub_m32(e.mem.u32(edi.wrapping_add(0x7c))); // 00cd99a0
                    x1 = ld(e, esi.wrapping_add(0xe0)); // 00cd99a3
                    x0 = x2; // 00cd99aa
                    x0 = addps(x0, x1); // 00cd99ad
                    x2 = x0; // 00cd99b0
                    e.mem.set_u32(sp.wrapping_add(0x14), fpu.fstp32()); // 00cd99b3
                    x0 = movss_load(e.mem.u32(sp.wrapping_add(0x14))); // 00cd99b7
                    x1 = x0; // 00cd99bd
                    x1 = shufps(x1, x0, 0x0); // 00cd99c0
                    x0 = ld(e, esi.wrapping_add(0xe0)); // 00cd99c4
                    x1 = mulps(x1, x0); // 00cd99cb
                    st(e, esi.wrapping_add(0xe0), x1); // 00cd99ce
                    block = 0x00cd982a;
                    continue; // 00cd99d5
                }
                0x00cd99da => {
                    fpu.pop(); // 00cd99da
                    block = 0x00cd99dc;
                }
                0x00cd99dc => {
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x1c))); // 00cd99dc
                    fpu.fcompp(None); // 00cd99e0
                    ah = fpu.status; // 00cd99e2
                    test = (ah & (0x41)) as u32; // 00cd99e4
                    if test != 0 {
                        block = 0x00cd9a4b;
                        continue;
                    } // 00cd99e7
                    e.mem.set_u8(ebx.wrapping_add(0x4), (0x0u32) as u8); // 00cd99e9
                    x0 = ld(e, esi.wrapping_add(0xa0)); // 00cd99ed
                    x1 = ld(e, esi.wrapping_add(0x40)); // 00cd99f4
                    x1 = mulps(x1, ld(e, sp.wrapping_add(0x30))); // 00cd99f8
                    st(e, sp.wrapping_add(0x60), x0); // 00cd99fd
                    x0 = movss_load(e.mem.u32(esi.wrapping_add(0xb0))); // 00cd9a02
                    x0 = shufps(x0, x0, 0x0); // 00cd9a0a
                    x0 = mulps(x0, x3); // 00cd9a0e
                    x2 = x1; // 00cd9a11
                    x2 = shufps(x2, x1, 0x55); // 00cd9a14
                    x3 = x1; // 00cd9a18
                    x2 = addss(x2, x1); // 00cd9a1b
                    x3 = shufps(x3, x1, 0xaa); // 00cd9a1f
                    x3 = addss(x3, x2); // 00cd9a23
                    e.mem.set_u32(sp.wrapping_add(0x1c), x3[0].to_bits()); // 00cd9a27
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x1c))); // 00cd9a2d
                    fpu.fadd_m32(e.mem.u32(esi.wrapping_add(0xb0))); // 00cd9a31
                    x0 = addps(x0, x5); // 00cd9a37
                    eax &= !0xffu32; // 00cd9a3a
                    st(e, sp.wrapping_add(0x20), x0); // 00cd9a3c
                    fpu.fsubrp(1); // 00cd9a41
                    e.mem.set_u32(sp.wrapping_add(0xc), fpu.fstp32()); // 00cd9a43
                    e.mem.set_u32(ebx, fpu.fst32()); // 00cd9a47
                    block = 0x00cd9a51;
                    continue; // 00cd9a49
                }
                0x00cd9a4b => {
                    eax = (eax & !0xffu32) | ((e.mem.u8(sp.wrapping_add(0x1a)) as u32) & 0xff); // 00cd9a4b
                    fpu.pop(); // 00cd9a4f
                    block = 0x00cd9a51;
                }
                0x00cd9a51 => {
                    x1 = ld(e, esi.wrapping_add(0x50)); // 00cd9a51
                    x0 = subps(x0, x5); // 00cd9a55
                    x0 = mulps(x0, x1); // 00cd9a58
                    x1 = x0; // 00cd9a5b
                    x1 = shufps(x1, x0, 0x55); // 00cd9a5e
                    x2 = x0; // 00cd9a62
                    x1 = addss(x1, x0); // 00cd9a65
                    x2 = shufps(x2, x0, 0xaa); // 00cd9a69
                    x2 = addss(x2, x1); // 00cd9a6d
                    e.mem.set_u32(sp.wrapping_add(0x1c), x2[0].to_bits()); // 00cd9a71
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x1c))); // 00cd9a77
                    e.mem.set_u32(esi.wrapping_add(0xb0), fpu.fstp32()); // 00cd9a7b
                    test = (eax & 0xff) & (eax & 0xff); // 00cd9a81
                    if test == 0 {
                        block = 0x00cd9a8d;
                        continue;
                    } // 00cd9a83
                    fpu.fxch(1); // 00cd9a85
                    e.mem.set_u32(sp.wrapping_add(0x10), fpu.fstp32()); // 00cd9a87
                    block = 0x00cd9a93;
                    continue; // 00cd9a8b
                }
                0x00cd9a8d => {
                    fpu.fstp_st(1); // 00cd9a8d
                    block = 0x00cd9a8f;
                }
                0x00cd9a8f => {
                    e.mem.set_u32(sp.wrapping_add(0x10), fpu.fst32()); // 00cd9a8f
                    block = 0x00cd9a93;
                }
                0x00cd9a93 => {
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x10))); // 00cd9a93
                    fpu.fsub_m32(e.mem.u32(esi.wrapping_add(0x90))); // 00cd9a97
                    fpu.fmul_m32(e.mem.u32(edi.wrapping_add(0x68))); // 00cd9a9d
                    fpu.fadd_m32(e.mem.u32(esi.wrapping_add(0x90))); // 00cd9aa0
                    e.mem.set_u32(sp.wrapping_add(0x1c), fpu.fstp32()); // 00cd9aa6
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0x1c))); // 00cd9aaa
                    e.mem.set_u32(esi.wrapping_add(0x90), fpu.fst32()); // 00cd9aae
                    fpu.fcomp(Some(Operand::Float(e.mem.u32(0x01013ea4u32)))); // 00cd9ab4
                    ah = fpu.status; // 00cd9aba
                    test = (ah & (0x5)) as u32; // 00cd9abc
                    if (test as u8).count_ones() % 2 == 0 {
                        block = 0x00cd9ac7;
                        continue;
                    } // 00cd9abf
                    e.mem.set_u32(esi.wrapping_add(0x90), fpu.fst32()); // 00cd9ac1
                    block = 0x00cd9ac7;
                }
                0x00cd9ac7 => {
                    fpu.fcom(Some(Operand::Float(e.mem.u32(esi.wrapping_add(0x90))))); // 00cd9ac7
                    ah = fpu.status; // 00cd9acd
                    test = (ah & (0x5)) as u32; // 00cd9acf
                    if (test as u8).count_ones() % 2 == 0 {
                        block = 0x00cd9ce6;
                        continue;
                    } // 00cd9ad2
                    ecx = sp.wrapping_add(0x140); // 00cd9ad8
                    fpu.pop(); // 00cd9adf
                    e.call(0x00c74ed0, &args![ecx]); // 00cd9ae1
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x78))); // 00cd9ae6
                    ecx =
                        (ecx & !0xffffu32) | ((e.mem.u16(esi.wrapping_add(0x14)) as u32) & 0xffff); // 00cd9ae9
                    e.mem.set_u32(sp.wrapping_add(0x164), fpu.fstp32()); // 00cd9aed
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x74))); // 00cd9af4
                    edx =
                        (edx & !0xffffu32) | ((e.mem.u16(esi.wrapping_add(0x16)) as u32) & 0xffff); // 00cd9af7
                    ebx = e.mem.u32(ebp.wrapping_add(0x10)); // 00cd9afb
                    e.mem.set_u32(sp.wrapping_add(0x160), fpu.fstp32()); // 00cd9afe
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x90))); // 00cd9b05
                    x0 = ld(e, esi.wrapping_add(0x20)); // 00cd9b0b
                    eax =
                        (eax & !0xffffu32) | ((e.mem.u16(esi.wrapping_add(0x18)) as u32) & 0xffff); // 00cd9b0f
                    e.mem.set_u32(sp.wrapping_add(0x168), fpu.fstp32()); // 00cd9b13
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x90))); // 00cd9b1a
                    e.mem.set_u16(sp.wrapping_add(0x140), (ecx & 0xffff) as u16); // 00cd9b20
                    pushed.push(ebx);
                    sp = sp.wrapping_sub(4); // 00cd9b28
                    e.mem.set_u32(sp.wrapping_add(0x170), fpu.fstp32()); // 00cd9b29
                    fpu.fld32(e.mem.u32(esi.wrapping_add(0x90))); // 00cd9b30
                    ecx = sp.wrapping_add(0x144); // 00cd9b36
                    e.mem.set_u16(sp.wrapping_add(0x146), (edx & 0xffff) as u16); // 00cd9b3d
                    e.mem.set_u32(sp.wrapping_add(0x174), fpu.fstp32()); // 00cd9b45
                    pushed.push(ecx);
                    sp = sp.wrapping_sub(4); // 00cd9b4c
                    edx = sp.wrapping_add(0x22); // 00cd9b4d
                    st(e, sp.wrapping_add(0x158), x0); // 00cd9b51
                    x0 = ld(e, sp.wrapping_add(0x28)); // 00cd9b59
                    pushed.push(edx);
                    sp = sp.wrapping_sub(4); // 00cd9b5e
                    e.mem.set_u16(sp.wrapping_add(0x150), (eax & 0xffff) as u16); // 00cd9b5f
                    st(e, sp.wrapping_add(0x18c), x0); // 00cd9b67
                    {
                        let a = take_args(&mut pushed);
                        e.call(0x00cdbae0, &a);
                    } // 00cd9b6f
                    x1 = ld(e, esi.wrapping_add(0x40)); // 00cd9b74
                    x4 = ld(e, sp.wrapping_add(0xcc)); // 00cd9b78
                    x6 = ld(e, sp.wrapping_add(0xac)); // 00cd9b80
                    x5 = ld(e, 0x010c71e0u32); // 00cd9b88
                    x0 = x4; // 00cd9b8f
                    x0 = mulps(x0, x1); // 00cd9b92
                    x3 = x0; // 00cd9b95
                    x3 = shufps(x3, x0, 0x55); // 00cd9b98
                    x3 = addss(x3, x0); // 00cd9b9c
                    x2 = x0; // 00cd9ba0
                    x2 = shufps(x2, x0, 0xaa); // 00cd9ba3
                    x2 = addss(x2, x3); // 00cd9ba7
                    x3 = x1; // 00cd9bab
                    x3 = shufps(x3, x1, 0xc9); // 00cd9bae
                    x3 = mulps(x3, ld(e, sp.wrapping_add(0x8c))); // 00cd9bb2
                    x0 = x1; // 00cd9bba
                    x0 = shufps(x0, x1, 0xd2); // 00cd9bbd
                    x0 = mulps(x0, x6); // 00cd9bc1
                    x0 = subps(x0, x3); // 00cd9bc4
                    x3 = ld(e, sp.wrapping_add(0x7c)); // 00cd9bc7
                    x0 = mulps(x0, x3); // 00cd9bcc
                    x7 = x3; // 00cd9bcf
                    x7 = mulps(x7, x3); // 00cd9bd2
                    x7 = addps(x7, x5); // 00cd9bd5
                    x7 = mulps(x7, x1); // 00cd9bd8
                    x1 = x2; // 00cd9bdb
                    x1 = shufps(x1, x2, 0x0); // 00cd9bde
                    x2 = ld(e, sp.wrapping_add(0x6c)); // 00cd9be2
                    x1 = mulps(x1, x4); // 00cd9be7
                    x7 = addps(x7, x1); // 00cd9bea
                    x0 = addps(x0, x7); // 00cd9bed
                    x0 = addps(x0, x0); // 00cd9bf0
                    st(e, sp.wrapping_add(0x13c), x0); // 00cd9bf3
                    x0 = x4; // 00cd9bfb
                    x0 = mulps(x0, x2); // 00cd9bfe
                    x7 = x0; // 00cd9c01
                    x7 = shufps(x7, x0, 0x55); // 00cd9c04
                    x7 = addss(x7, x0); // 00cd9c08
                    x1 = x0; // 00cd9c0c
                    x1 = shufps(x1, x0, 0xaa); // 00cd9c0f
                    x1 = addss(x1, x7); // 00cd9c13
                    x7 = x2; // 00cd9c17
                    x0 = x2; // 00cd9c1a
                    x0 = shufps(x0, x2, 0xd2); // 00cd9c1d
                    x0 = mulps(x0, x6); // 00cd9c21
                    x7 = shufps(x7, x2, 0xc9); // 00cd9c24
                    x7 = mulps(x7, ld(e, sp.wrapping_add(0x8c))); // 00cd9c28
                    x0 = subps(x0, x7); // 00cd9c30
                    sp = sp.wrapping_add(0xcu32); // 00cd9c33
                    x0 = mulps(x0, x3); // 00cd9c36
                    x6 = x3; // 00cd9c39
                    x6 = mulps(x6, x3); // 00cd9c3c
                    x6 = addps(x6, x5); // 00cd9c3f
                    x6 = mulps(x6, x2); // 00cd9c42
                    x2 = x1; // 00cd9c45
                    eax = sp.wrapping_add(0x90); // 00cd9c48
                    x2 = shufps(x2, x1, 0x0); // 00cd9c4f
                    x2 = mulps(x2, x4); // 00cd9c53
                    pushed.push(eax);
                    sp = sp.wrapping_sub(4); // 00cd9c56
                    ecx = sp.wrapping_add(0x134); // 00cd9c57
                    x6 = addps(x6, x2); // 00cd9c5e
                    x0 = addps(x0, x6); // 00cd9c61
                    pushed.push(ecx);
                    sp = sp.wrapping_sub(4); // 00cd9c64
                    x0 = addps(x0, x0); // 00cd9c65
                    ecx = sp.wrapping_add(0x128); // 00cd9c68
                    st(e, sp.wrapping_add(0x98), x0); // 00cd9c6f
                    {
                        let a = take_args(&mut pushed);
                        fn_00cd9d10(e, Ptr::new(ecx), Ptr::new(a[0]), Ptr::new(a[1]));
                        sp = sp.wrapping_add(8);
                    } // 00cd9c77
                    edi = e.mem.u32(sp.wrapping_add(0x5c)); // 00cd9c7c
                    pushed.push(edi);
                    sp = sp.wrapping_sub(4); // 00cd9c80
                    edx = sp.wrapping_add(0x124); // 00cd9c81
                    pushed.push(edx);
                    sp = sp.wrapping_sub(4); // 00cd9c88
                    ecx = sp.wrapping_add(0x98); // 00cd9c89
                    {
                        let a = take_args(&mut pushed);
                        e.call(0x00c66320, &[ecx, a[0], a[1]]);
                        sp = sp.wrapping_add(8);
                    } // 00cd9c90
                    x0 = movss_load(e.mem.u32(esi.wrapping_add(0x90))); // 00cd9c95
                    x1 = ld(e, 0x010ec7f0u32); // 00cd9c9d
                    x2 = ld(e, edi); // 00cd9ca4
                    x0 = shufps(x0, x0, 0x0); // 00cd9ca7
                    x1 = subps(x1, x0); // 00cd9cab
                    x1 = mulps(x1, x2); // 00cd9cae
                    x2 = ld(e, sp.wrapping_add(0x90)); // 00cd9cb1
                    x2 = mulps(x2, x0); // 00cd9cb9
                    x1 = addps(x1, x2); // 00cd9cbc
                    ecx = sp.wrapping_add(0x40); // 00cd9cbf
                    st(e, sp.wrapping_add(0x40), x1); // 00cd9cc3
                    e.call(0x005611c0, &args![ecx]); // 00cd9cc8
                    eax = e.mem.u16(esi.wrapping_add(0x18)) as i16 as i32 as u32; // 00cd9ccd
                    pushed.push(0x1u32);
                    sp = sp.wrapping_sub(4); // 00cd9cd1
                    pushed.push(eax);
                    sp = sp.wrapping_sub(4); // 00cd9cd3
                    ecx = ebx; // 00cd9cd4
                    {
                        let a = take_args(&mut pushed);
                        eax = e.call(0x00cdad80, &[ecx, a[0], a[1]]).u32();
                        sp = sp.wrapping_add(8);
                    } // 00cd9cd6
                    x0 = ld(e, sp.wrapping_add(0x40)); // 00cd9cdb
                    st(e, eax.wrapping_add(0x10), x0); // 00cd9ce0
                    block = 0x00cd9ce8;
                    continue; // 00cd9ce4
                }
                0x00cd9ce6 => {
                    e.mem.set_u32(ebx, fpu.fstp32()); // 00cd9ce6
                    block = 0x00cd9ce8;
                }
                0x00cd9ce8 => {
                    fpu.fld32(e.mem.u32(sp.wrapping_add(0xc))); // 00cd9ce8
                    x0 = ld(e, sp.wrapping_add(0x60)); // 00cd9cec
                                                       // pop EDI // 00cd9cf1
                    e.mem.set_u32(esi.wrapping_add(0x94), fpu.fstp32()); // 00cd9cf2
                    st(e, esi.wrapping_add(0xa0), x0); // 00cd9cf8
                                                       // pop ESI // 00cd9cff
                                                       // pop EBX // 00cd9d00
                                                       // pop EBP // 00cd9d03
                    break 'run; // 00cd9d04
                }
                _ => unreachable!("block {block:08x}"),
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cd_8a50,
            fn_00cd8a50(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00cd_8d90, fn_00cd8d90(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00cd_9d10, fn_00cd9d10(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    const SLOT_FOUR_DOUBLE: u32 = 0x0900_0701;
    const SLOT_EIGHT_DOUBLE: u32 = 0x0900_0711;

    /// What a cast of the double collidable reports.
    #[derive(Clone, Copy)]
    struct Reply {
        hit: bool,
        fraction: f32,
        normal: V,
    }

    const MISS: Reply = Reply {
        hit: false,
        fraction: 0.0,
        normal: [0.0; 4],
    };

    /// One call of a cast: the slot (4 or 8), the argument words, the segment's start and end
    /// as the cast saw them.
    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>, V, V)>>>;

    fn put(e: &mut Engine, at: u32, v: V) {
        st(e, at, v);
    }

    fn put_bits(e: &mut Engine, at: u32, words: [u32; 4]) {
        for (lane, word) in words.into_iter().enumerate() {
            e.mem.set_u32(at + 4 * lane as u32, word);
        }
    }

    /// The exe constants these functions read.
    fn constants(e: &mut Engine) {
        e.map(0x0100_0000, 0x0030_0000);
        put_bits(e, SIGN_MASK, [0x8000_0000; 4]);
        put(e, NEAR_ONE, [0.99999; 4]);
        put(e, NEAR_MINUS_ONE, [-0.999; 4]);
        put(e, IDENTITY_QUATERNION, [0.0, 0.0, 0.0, 1.0]);
        put(e, THREE_VECTOR, [3.0; 4]);
        put(e, HALF_VECTOR, [0.5; 4]);
        e.mem.set_f32(ZERO_SCALAR, 0.0);
        e.mem.set_f32(THREE_SCALAR, 3.0);
        e.mem.set_f32(HALF_SCALAR, 0.5);
        e.mem.set_f32(FRACTION_START, 3.4028235e38);
        e.mem.set_f32(TOLERANCE, 0.001);
        put_bits(e, ABSOLUTE_MASK, [0x7fff_ffff; 4]);
        put(e, ZERO_VECTOR, [0.0; 4]);
        put(e, 0x010c_71e0, [-0.5; 4]);
        put(e, 0x010c_7170, [0.0, 0.0, 0.0, 1.0]);
        put_bits(e, 0x010c_4c00, [0x8000_0000, 0x8000_0000, 0x8000_0000, 0]);
        put_bits(e, 0x010c_4c10, [u32::MAX, u32::MAX, u32::MAX, 0]);
        e.mem.set_f32(0x0101_3ea4, 0.01);
        put(e, 0x010e_c7f0, [1.0; 4]);
    }

    /// A collidable whose casts (slot 4 with the extra word, slot 8 without) log themselves and
    /// answer from `replies`, in order. Returns the collidable's address.
    fn collidable(e: &mut Engine, replies: &[Reply], calls: &Calls) -> u32 {
        let queue = Rc::new(RefCell::new(VecDeque::from(replies.to_vec())));
        for (address, slot) in [(SLOT_FOUR_DOUBLE, 4u32), (SLOT_EIGHT_DOUBLE, 8)] {
            let queue = Rc::clone(&queue);
            let calls = Rc::clone(calls);
            e.register_double(address, move |e, a| {
                let (fraction_at, out_at) = if slot == 4 {
                    (a[5], a[6])
                } else {
                    (a[4], a[5])
                };
                let start = load_vec(e, a[2]);
                let end = load_vec(e, a[3]);
                calls.borrow_mut().push((slot, a.to_vec(), start, end));
                let reply = queue.borrow_mut().pop_front().expect("unexpected cast");
                e.mem.set_u8(a[1], u8::from(reply.hit));
                if reply.hit {
                    e.mem.set_f32(fraction_at, reply.fraction);
                    store_vec(e, out_at, reply.normal);
                }
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
        }
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, SLOT_FOUR_DOUBLE);
        e.mem.set_u32(vtable + 8, SLOT_EIGHT_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    fn near(a: V, b: V) {
        for lane in 0..4 {
            assert!(
                (a[lane] - b[lane]).abs() <= 1e-5,
                "lane {lane}: {a:?} vs {b:?}"
            );
        }
    }

    /// The probe's `this` (direction up the z axis, radius 0.5, length 2.0), the query block
    /// holding the collidable, and the two points.
    fn probe(e: &mut Engine, collidable: u32, second: V) -> (u32, u32, u32, u32, u32, u32) {
        let this = e.mem.alloc(0x100);
        put(e, this + 0x40, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_f32(this + 0x7c, 0.5);
        e.mem.set_f32(this + 0x80, 2.0);
        let query = e.mem.alloc(0x100);
        e.mem.set_u32(query + 0x64, collidable);
        e.mem.set_u32(query + 0x80, 0x1234);
        let point = e.mem.alloc(0x10);
        put(e, point, [1.0, 2.0, 3.0, 0.0]);
        let other = e.mem.alloc(0x10);
        put(e, other, second);
        let position = e.mem.alloc(0x10);
        let normal = e.mem.alloc(0x10);
        (this, query, point, other, position, normal)
    }

    /// One cast when the second point is within the tolerance of the first: its segment runs from
    /// the point plus the radius to the point minus the length, and the position is found from
    /// the fraction.
    #[test]
    fn single_cast_places_the_position() {
        let mut e = Engine::new();
        constants(&mut e);
        let calls: Calls = Rc::default();
        let normal = [0.0, 0.0, 1.0, 0.0];
        let hit = Reply {
            hit: true,
            fraction: 0.25,
            normal,
        };
        let object = collidable(&mut e, &[hit], &calls);
        let (this, query, point, other, position, out_normal) =
            probe(&mut e, object, [1.0004, 2.0, 3.0, 0.0]);
        let found = e
            .call(
                0x00cd_8a50,
                &args![this, query, point, other, position, out_normal],
            )
            .u32()
            & 0xff;
        assert_eq!(found, 1);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 1);
        let (slot, words, start, end) = &calls[0];
        assert_eq!(*slot, 8);
        assert_eq!(words.len(), 6);
        assert_eq!(*start, [1.0, 2.0, 3.5, 0.0]);
        assert_eq!(*end, [1.0, 2.0, 1.0, 0.0]);
        // 3.0 - (0.25 * 2.5 - 0.5)
        near(load_vec(&e, position), [1.0, 2.0, 2.875, 0.0]);
        assert_eq!(load_vec(&e, out_normal), normal);
    }

    /// A miss answers false and writes nothing; a set flag byte selects the cast that takes the
    /// extra word.
    #[test]
    fn miss_and_the_two_sided_cast() {
        let mut e = Engine::new();
        constants(&mut e);
        let calls: Calls = Rc::default();
        let object = collidable(&mut e, &[MISS], &calls);
        let (this, query, point, other, position, out_normal) =
            probe(&mut e, object, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_u8(query + 0x84, 1);
        let found = e
            .call(
                0x00cd_8a50,
                &args![this, query, point, other, position, out_normal],
            )
            .u32()
            & 0xff;
        assert_eq!(found, 0);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, 4);
        assert_eq!(calls[0].1.len(), 7);
        assert_eq!(calls[0].1[4], 0x1234);
        assert_eq!(load_vec(&e, position), [0.0; 4]);
        assert_eq!(load_vec(&e, out_normal), [0.0; 4]);
    }

    /// Two casts when the points differ. The second hit counts only with a normal facing along
    /// the direction, and its fraction is `(s_R - s_P) / (-slope * (x + r))`; the smaller
    /// fraction wins and brings its own normal.
    #[test]
    fn two_casts_the_smaller_fraction_wins() {
        let first_normal = [0.0, 0.0, 1.0, 0.0];
        let second_normal = [0.0, 0.5, 0.5, 0.0];
        for (first_fraction, wins_first) in [(0.5f32, true), (0.9, false)] {
            let mut e = Engine::new();
            constants(&mut e);
            let calls: Calls = Rc::default();
            let first = Reply {
                hit: true,
                fraction: first_fraction,
                normal: first_normal,
            };
            let second = Reply {
                hit: true,
                fraction: 0.5,
                normal: second_normal,
            };
            let object = collidable(&mut e, &[first, second], &calls);
            let (this, query, point, other, position, out_normal) =
                probe(&mut e, object, [1.0, 2.0, 2.25, 0.0]);
            let found = e
                .call(
                    0x00cd_8a50,
                    &args![this, query, point, other, position, out_normal],
                )
                .u32()
                & 0xff;
            assert_eq!(found, 1);
            let calls = calls.borrow();
            assert_eq!(calls.len(), 2);
            // The second cast's segment: from (1, 2, 2.75) to (1, 2, 0.25); the point at its fraction
            // 0.5 is (1, 2, 1.5), so s_R = 1.75 and s_P = 2.75, and the second fraction is
            // (1.75 - 2.75) / (0.5 * -2.5) = 0.8.
            assert_eq!(calls[1].2, [1.0, 2.0, 2.75, 0.0]);
            assert_eq!(calls[1].3, [1.0, 2.0, 0.25, 0.0]);
            let (fraction, normal) = if wins_first {
                (first_fraction, first_normal)
            } else {
                (0.8, second_normal)
            };
            assert_eq!(load_vec(&e, out_normal), normal);
            let scale = -(fraction * 2.5 - 0.5);
            near(load_vec(&e, position), [1.0, 2.0, 3.0 + scale, 0.0]);
        }
    }

    /// The second hit is dropped when its normal faces away from the direction.
    #[test]
    fn a_second_hit_facing_away_is_dropped() {
        let mut e = Engine::new();
        constants(&mut e);
        let calls: Calls = Rc::default();
        let normal = [0.0, 0.0, 1.0, 0.0];
        let first = Reply {
            hit: false,
            fraction: 0.0,
            normal,
        };
        let second = Reply {
            hit: true,
            fraction: 0.0,
            normal: [0.0, 0.0, -1.0, 0.0],
        };
        let object = collidable(&mut e, &[first, second], &calls);
        let (this, query, point, other, position, out_normal) =
            probe(&mut e, object, [1.0, 2.0, -4.0, 0.0]);
        let found = e
            .call(
                0x00cd_8a50,
                &args![this, query, point, other, position, out_normal],
            )
            .u32()
            & 0xff;
        assert_eq!(found, 0);
        assert_eq!(calls.borrow().len(), 2);
    }

    fn quaternion(e: &mut Engine, p: V, q: V) -> V {
        let this = e.mem.alloc(0x10);
        let a0 = e.mem.alloc(0x10);
        let a1 = e.mem.alloc(0x10);
        put(e, a0, p);
        put(e, a1, q);
        e.call(0x00cd_9d10, &args![this, a0, a1]);
        load_vec(e, this)
    }

    fn near_to(a: V, b: V, tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (a[lane] - b[lane]).abs() <= tolerance,
                "lane {lane}: {a:?} vs {b:?}"
            );
        }
    }

    /// Equal vectors give the identity quaternion; opposite ones go to the half turn helper
    /// with the first vector.
    #[test]
    fn quaternion_identity_and_half_turn() {
        let mut e = Engine::new();
        constants(&mut e);
        let p = [0.6, 0.8, 0.0, 0.0];
        assert_eq!(quaternion(&mut e, p, p), [0.0, 0.0, 0.0, 1.0]);
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(HALF_TURN, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        let a0 = e.mem.alloc(0x10);
        let a1 = e.mem.alloc(0x10);
        put(&mut e, a0, [1.0, 0.0, 0.0, 0.0]);
        put(&mut e, a1, [-1.0, 0.0, 0.0, 0.0]);
        e.call(0x00cd_9d10, &args![this, a0, a1]);
        assert_eq!(*log.borrow(), vec![vec![this, a0]]);
        assert_eq!(load_vec(&e, this), [0.0; 4]);
    }

    /// A quarter turn and a 53 degree turn about the z axis give `(axis * sin(a/2), cos(a/2))`.
    #[test]
    fn quaternion_of_a_turn() {
        let mut e = Engine::new();
        constants(&mut e);
        let half = std::f32::consts::FRAC_1_SQRT_2;
        near_to(
            quaternion(&mut e, [1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]),
            [0.0, 0.0, half, half],
            1e-4,
        );
        near_to(
            quaternion(&mut e, [1.0, 0.0, 0.0, 0.0], [0.6, 0.8, 0.0, 0.0]),
            [0.0, 0.0, 0.447_213_6, 0.894_427_2],
            1e-4,
        );
    }

    /// Just short of opposite (the refined scale): the same formula holds.
    #[test]
    fn quaternion_of_a_near_half_turn() {
        let mut e = Engine::new();
        constants(&mut e);
        let d = -0.9995f32;
        let s = (1.0 - d * d).sqrt();
        let result = quaternion(&mut e, [1.0, 0.0, 0.0, 0.0], [d, s, 0.0, 0.0]);
        near_to(
            result,
            [0.0, 0.0, ((1.0 - d) / 2.0).sqrt(), ((1.0 + d) / 2.0).sqrt()],
            1e-3,
        );
    }

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// The foot placement step's callees outside this file, as logging doubles. `00cdad80`
    /// returns `track`.
    fn callees(e: &mut Engine, track: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for (address, name) in [
            (0x00cd_a180u32, "bone model space"),
            (0x00c7_4ed0, "reset solver"),
            (0x00cd_bae0, "solve"),
            (0x00c6_6320, "set mul"),
            (0x0056_11c0, "normalize"),
            (0x00cd_ad80, "track"),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((name, a.to_vec()));
                Ret {
                    eax: if name == "track" { track } else { 0 },
                    ..Ret::default()
                }
            });
        }
        log
    }

    struct Step {
        this: u32,
        input: u32,
        out: u32,
        pose: u32,
    }

    /// A solver state, an input block (a point at `(1, 2, 3)`, identity rotation and unit scale,
    /// the foot planted or not), an output block and a pose with one bone.
    fn step(e: &mut Engine, planted: bool, blend: f32, object: u32) -> Step {
        let this = e.mem.alloc(0x100);
        e.mem.set_u16(this + 0x14, 7);
        e.mem.set_u16(this + 0x16, 8);
        e.mem.set_u16(this + 0x18, 1);
        put(e, this + 0x30, [0.0, 0.5, 0.0, 0.0]);
        put(e, this + 0x40, [0.0, 0.0, 1.0, 0.0]);
        put(e, this + 0x50, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_f32(this + 0x60, 0.25);
        e.mem.set_f32(this + 0x64, 0.0);
        e.mem.set_f32(this + 0x68, 3.0);
        e.mem.set_f32(this + 0x6c, 100.0);
        e.mem.set_f32(this + 0x70, -100.0);
        e.mem.set_f32(this + 0x74, 0.125);
        e.mem.set_f32(this + 0x78, 0.375);
        e.mem.set_f32(this + 0x7c, 0.5);
        e.mem.set_f32(this + 0x80, 2.0);
        put(e, this + 0xa0, [9.0, 8.0, 7.0, 6.0]);
        e.mem.set_f32(this + 0xb0, 0.5);
        let input = e.mem.alloc(0x100);
        put(e, input, [0.0, 0.0, 1.5, 0.0]);
        put(e, input + 0x30, [1.0, 2.0, 3.0, 0.0]);
        put(e, input + 0x10, [0.0, 0.0, 0.0, 1.0]);
        put(e, input + 0x40, [0.0, 0.0, 0.0, 1.0]);
        put(e, input + 0x50, [1.0; 4]);
        e.mem.set_u8(input + 0x60, u8::from(planted));
        e.mem.set_f32(input + 0x68, blend);
        e.mem.set_u32(input + 0x64, object);
        let out = e.mem.alloc(0x10);
        let pose = e.mem.alloc(0x40);
        let bones = e.mem.alloc(0x100);
        let flags = e.mem.alloc(0x40);
        e.mem.set_u32(pose + 0x10, bones);
        e.mem.set_u32(pose + 0x1c, flags);
        put(e, bones + 0x30, [1.0, 1.0, 1.0, 0.0]);
        put(e, bones + 0x40, [0.0, 0.0, 0.0, 1.0]);
        put(e, bones + 0x50, [1.0; 4]);
        Step {
            this,
            input,
            out,
            pose,
        }
    }

    fn run(e: &mut Engine, s: &Step) {
        e.call(0x00cd_8d90, &args![s.this, s.input, s.out, s.pose]);
    }

    /// A foot that is not planted: the output is cleared, the solver's remembered height is the
    /// probe point's height plus the offset minus the plane distance, and no blend remains.
    #[test]
    fn unplanted_foot_clears_the_output() {
        let mut e = Engine::new();
        constants(&mut e);
        let log = callees(&mut e, 0);
        let s = step(&mut e, false, 0.5, 0);
        e.mem.set_f32(s.out, 5.0);
        e.mem.set_u8(s.this + 0xb4, 1);
        run(&mut e, &s);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.f32(s.out), 0.0);
        assert_eq!(e.mem.u8(s.out + 4), 0);
        assert_eq!(e.mem.u8(s.this + 0xb4), 0);
        assert_eq!(e.mem.f32(s.this + 0x90), 0.0);
        // (4 + 0.5) - 1.5
        assert_eq!(e.mem.f32(s.this + 0x94), 3.0);
        assert_eq!(load_vec(&e, s.this + 0xa0), [9.0, 8.0, 7.0, 6.0]);
    }

    /// A planted foot whose casts miss behaves like the unplanted one.
    #[test]
    fn planted_foot_without_ground() {
        let mut e = Engine::new();
        constants(&mut e);
        let _log = callees(&mut e, 0);
        let calls: Calls = Rc::default();
        let object = collidable(&mut e, &[MISS, MISS], &calls);
        let s = step(&mut e, true, 0.5, object);
        e.mem.set_u8(s.this + 0xb4, 1);
        run(&mut e, &s);
        assert_eq!(calls.borrow().len(), 2);
        assert_eq!(e.mem.f32(s.out), 0.0);
        assert_eq!(e.mem.u8(s.out + 4), 0);
        assert_eq!(e.mem.f32(s.this + 0x90), 0.0);
        assert_eq!(e.mem.f32(s.this + 0x94), 3.0);
        assert_eq!(e.mem.u8(s.this + 0xb4), 0);
        assert_eq!(load_vec(&e, s.this + 0xa0), [9.0, 8.0, 7.0, 6.0]);
    }

    /// A planted foot over ground with a zero blend rate: the hit is found, the result is
    /// remembered, but the blend stays zero so the output float is cleared again.
    #[test]
    fn planted_foot_with_zero_blend() {
        let mut e = Engine::new();
        constants(&mut e);
        let log = callees(&mut e, 0);
        let calls: Calls = Rc::default();
        let ground = Reply {
            hit: true,
            fraction: 0.4,
            normal: [0.0, 0.0, 1.0, 0.0],
        };
        let object = collidable(&mut e, &[ground, MISS], &calls);
        let s = step(&mut e, true, 0.0, object);
        run(&mut e, &s);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 2);
        // The first cast's segment: from the point (2, 3, 4) plus the radius to minus the length.
        assert_eq!(calls[0].2, [2.0, 3.0, 4.5, 0.0]);
        assert_eq!(calls[0].3, [2.0, 3.0, 2.0, 0.0]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u8(s.out + 4), 1);
        assert_eq!(e.mem.f32(s.out), 0.0);
        assert_eq!(e.mem.f32(s.this + 0x90), 0.0);
        // The ground height under the point: 4 - (0.4 * 2.5 - 0.5).
        near(
            [e.mem.f32(s.this + 0x94), 0.0, 0.0, 0.0],
            [3.5, 0.0, 0.0, 0.0],
        );
        assert_eq!(load_vec(&e, s.this + 0xa0), [0.0, 0.0, 1.0, 0.0]);
    }

    /// A planted foot over ground with a blend rate of one half: the output float is the ground
    /// offset, the blend is a half, and the two-joint solver runs, then its result is composed
    /// and written to the track the exe looks up.
    #[test]
    fn planted_foot_solves_the_leg() {
        let mut e = Engine::new();
        constants(&mut e);
        let track = e.mem.alloc(0x40);
        let log = callees(&mut e, track);
        let calls: Calls = Rc::default();
        let ground = Reply {
            hit: true,
            fraction: 0.4,
            normal: [0.0, 0.0, 1.0, 0.0],
        };
        let object = collidable(&mut e, &[ground, MISS], &calls);
        let s = step(&mut e, true, 0.5, object);
        run(&mut e, &s);
        assert_eq!(e.mem.u8(s.out + 4), 1);
        // 3.5 - (0.25 + 3).
        near([e.mem.f32(s.out), 0.0, 0.0, 0.0], [0.25, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(s.this + 0x90), 0.5);
        let log = log.borrow();
        let names: Vec<&str> = log.iter().map(|(name, _)| *name).collect();
        assert_eq!(
            names,
            vec!["reset solver", "solve", "set mul", "normalize", "track"]
        );
        // The solver's argument block carries the bones and the blend.
        let solve = &log[1].1;
        let block = solve[1];
        assert_eq!(e.mem.u16(block), 7);
        assert_eq!(e.mem.u16(block + 2), 8);
        assert_eq!(e.mem.u16(block + 4), 1);
        assert_eq!(e.mem.f32(block + 0x20), 0.125);
        assert_eq!(e.mem.f32(block + 0x24), 0.375);
        assert_eq!(e.mem.f32(block + 0x28), 0.5);
        assert_eq!(e.mem.f32(block + 0x2c), 0.5);
        assert_eq!(e.mem.f32(block + 0x30), 0.5);
        // The track call takes the bone and 1; the track gets half the pose's rotation and half
        // the identity step's (0, 0, 1, 0).
        assert_eq!(log[4].1[1..], [1, 1]);
        near(load_vec(&e, track + 0x10), [0.0, 0.0, 0.5, 0.5]);
        assert_eq!(e.mem.f32(s.this + 0x94), 3.5);
    }
}
