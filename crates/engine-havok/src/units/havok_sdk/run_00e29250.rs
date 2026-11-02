//! `Havok SDK/run_00e29250` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMatrix3::setTranspose` (Xbox PDB, `00cd4ba0`): `this` = the destination, one stack word:
/// the source.
const MATRIX3_SET_TRANSPOSE: u32 = 0x00cd_4ba0;
/// `hkMatrix3_setMulMat3Mat3` (Xbox PDB, `00cd5530`): cdecl `(out, a, b)`.
const MATRIX3_SET_MUL: u32 = 0x00cd_5530;

/// Four lanes as raw bits, so that NaN payloads travel unchanged.
type Lanes = [u32; 4];

fn is_nan_bits(bits: u32) -> bool {
    bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0
}

/// The NaN an SSE arithmetic lane (`ADDPS`, `SUBPS`, `MULPS`) gives, or `None` when neither
/// operand is a NaN (Intel SDM vol. 1, table 4-7, SSE column): a NaN in the first (destination)
/// operand wins, otherwise the second; the result is quieted.
pub(crate) fn sse_nan(first: u32, second: u32) -> Option<u32> {
    if is_nan_bits(first) {
        Some(x87_quiet_bits(first))
    } else if is_nan_bits(second) {
        Some(x87_quiet_bits(second))
    } else {
        None
    }
}

pub(crate) fn lane_add(first: u32, second: u32) -> u32 {
    sse_nan(first, second)
        .unwrap_or_else(|| (f32::from_bits(first) + f32::from_bits(second)).to_bits())
}

pub(crate) fn lane_sub(first: u32, second: u32) -> u32 {
    sse_nan(first, second)
        .unwrap_or_else(|| (f32::from_bits(first) - f32::from_bits(second)).to_bits())
}

pub(crate) fn lane_mul(first: u32, second: u32) -> u32 {
    sse_nan(first, second)
        .unwrap_or_else(|| (f32::from_bits(first) * f32::from_bits(second)).to_bits())
}

/// `ADDPS first, second`.
fn add_ps(first: Lanes, second: Lanes) -> Lanes {
    std::array::from_fn(|lane| lane_add(first[lane], second[lane]))
}

/// `SUBPS first, second`.
fn sub_ps(first: Lanes, second: Lanes) -> Lanes {
    std::array::from_fn(|lane| lane_sub(first[lane], second[lane]))
}

/// `MULPS first, second`.
fn mul_ps(first: Lanes, second: Lanes) -> Lanes {
    std::array::from_fn(|lane| lane_mul(first[lane], second[lane]))
}

/// `SHUFPS v, v, 0x00` / `0x55` / `0xaa`: lane `lane` in all four.
fn splat(v: Lanes, lane: usize) -> Lanes {
    [v[lane]; 4]
}

fn load_lanes(e: &Engine, at: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.u32(at + 4 * lane as u32))
}

fn store_lanes(e: &mut Engine, at: u32, v: Lanes) {
    for (lane, bits) in v.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * lane as u32, bits);
    }
}

/// `__time64` (`00ecadda`): cdecl, a `__time64_t*` (null here); returns the
/// 64-bit time in EDX:EAX.
const TIME64: u32 = 0x00ec_adda;

// Translated from 00e29910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Returns `__time64(0)`.
pub fn fn_00e29910(e: &mut Engine) -> u64 {
    e.call(TIME64, &args![0u32]).u64()
}

// Translated from 00e29250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: transposes each of the four 3x3 blocks (`0x30` bytes apart) of
/// the 6x6 matrix at `source` into the matrix at `target`, swapping the two off-diagonal
/// blocks: `hkMatrix3::setTranspose` (`00cd4ba0`, `this` = the destination, the source as the
/// stack word) on `(target, source)`, `(target + 0x30, source + 0x60)`, `(target + 0x60, source
/// + 0x30)` and `(target + 0x90, source + 0x90)`. The decompiler hides the first argument; the
/// disassembly takes `target` from the first stack word and `source` from the second. The
/// function takes two stack words and pops none (`RET`) and returns nothing.
pub fn fn_00e29250(e: &mut Engine, target: Ptr, source: Ptr) {
    let (target, source) = (target.addr(), source.addr());
    e.call(MATRIX3_SET_TRANSPOSE, &args![target, source]);
    e.call(MATRIX3_SET_TRANSPOSE, &args![target + 0x30, source + 0x60]);
    e.call(MATRIX3_SET_TRANSPOSE, &args![target + 0x60, source + 0x30]);
    e.call(MATRIX3_SET_TRANSPOSE, &args![target + 0x90, source + 0x90]);
}

// Translated from 00e29290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: adds the twelve vectors (`0xc0` bytes) at `source` to those at
/// `target` (`ADDPS`, aligned loads). The operand order is the compiler's: vectors 0 and 3 are
/// `target + source`, the other ten `source + target`, which only matters for the NaN a lane
/// gives when both are NaNs. The function takes two stack words and pops none (`RET`) and
/// returns nothing.
pub fn fn_00e29290(e: &mut Engine, target: Ptr, source: Ptr) {
    for vector in 0..12u32 {
        let own = load_lanes(e, target.addr() + 0x10 * vector);
        let other = load_lanes(e, source.addr() + 0x10 * vector);
        let sum = if vector == 0 || vector == 3 {
            add_ps(own, other)
        } else {
            add_ps(other, own)
        };
        store_lanes(e, target.addr() + 0x10 * vector, sum);
    }
}

// Translated from 00e29380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: subtracts the twelve vectors (`0xc0` bytes) at `source` from
/// those at `target` (`SUBPS` with the target as the first operand). The function takes two
/// stack words and pops none (`RET`) and returns nothing.
pub fn fn_00e29380(e: &mut Engine, target: Ptr, source: Ptr) {
    for vector in 0..12u32 {
        let own = load_lanes(e, target.addr() + 0x10 * vector);
        let other = load_lanes(e, source.addr() + 0x10 * vector);
        store_lanes(e, target.addr() + 0x10 * vector, sub_ps(own, other));
    }
}

// Translated from 00e29470 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: multiplies two 6x6 matrices stored as 2x2 blocks of 3x3 matrices
/// (`0x30` bytes each, row-major in blocks): `target(r, c) = a(r, 0) * b(0, c) + a(r, 1) *
/// b(1, c)` for the blocks at `target + 0x30 * (2r + c)`, `a + 0x60 * r + 0x30 * k` and `b + 0x60
/// * k + 0x30 * c`. Each term is `hkMatrix3_setMulMat3Mat3` (`00cd5530`, cdecl `(out, a, b)`): the
/// first into a `0x30`-byte stack temporary, the second straight into the target block; then
/// the temporary is added to the block (`ADDPS`: block + temporary for the first two vectors,
/// temporary + block for the third). The function takes three stack words and pops none
/// (`RET`) and returns nothing.
pub fn fn_00e29470(e: &mut Engine, target: Ptr, a: Ptr, b: Ptr) {
    let mut block = target.addr();
    for row in 0..2u32 {
        let left = a.addr() + 0x60 * row;
        for column in 0..2u32 {
            let right = b.addr() + 0x30 * column;
            e.with_stack(0x30, |e, temporary| {
                e.call(MATRIX3_SET_MUL, &args![temporary, left, right]);
                e.call(MATRIX3_SET_MUL, &args![block, left + 0x30, right + 0x60]);
                for vector in 0..3u32 {
                    let own = load_lanes(e, block + 0x10 * vector);
                    let extra = load_lanes(e, temporary.addr() + 0x10 * vector);
                    let sum = if vector == 2 {
                        add_ps(extra, own)
                    } else {
                        add_ps(own, extra)
                    };
                    store_lanes(e, block + 0x10 * vector, sum);
                }
            });
            block += 0x30;
        }
    }
}

// Translated from 00e29510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the product of a 6x6 matrix at `a` (twelve vectors: four 3x3
/// blocks of three vectors, `0x30` bytes each) with the 6-vector at `b` (two vectors, the
/// components x, y, z of each used), as two result vectors at `target`:
/// `target[0] = (b0 * A[0..3]) + (b1 * A[3..6])` and `target[1] = (b1 * A[9..12]) + (b0 *
/// A[6..9])` where `v * A[k..k + 3]` is `((v.y * A[k + 1]) + (v.x * A[k])) + (v.z * A[k + 2])`.
/// The exe reads `b[0]`, `b[1]` and the blocks again after storing `target[0]`, so the loads here
/// happen in the same order. The function takes three stack words and pops none (`RET`) and
/// returns nothing.
pub fn fn_00e29510(e: &mut Engine, target: Ptr, a: Ptr, b: Ptr) {
    let (target, a, b) = (target.addr(), a.addr(), b.addr());
    // Dot of the vector `v` with the three vectors at `a + 0x10 * k`.
    let combine = |e: &Engine, v: Lanes, k: u32| {
        let first = mul_ps(splat(v, 1), load_lanes(e, a + 0x10 * (k + 1)));
        let second = mul_ps(splat(v, 0), load_lanes(e, a + 0x10 * k));
        let third = mul_ps(splat(v, 2), load_lanes(e, a + 0x10 * (k + 2)));
        add_ps(add_ps(first, second), third)
    };
    let b_first = load_lanes(e, b);
    let sum_a = combine(e, b_first, 0);
    let b_second = load_lanes(e, b + 0x10);
    let sum_b = combine(e, b_second, 3);
    store_lanes(e, target, sum_b);
    let b_first = load_lanes(e, b);
    let sum_c = combine(e, b_first, 6);
    let b_second = load_lanes(e, b + 0x10);
    let sum_d = combine(e, b_second, 9);
    let first = load_lanes(e, target);
    store_lanes(e, target, add_ps(first, sum_a));
    store_lanes(e, target + 0x10, sum_d);
    let second = load_lanes(e, target + 0x10);
    store_lanes(e, target + 0x10, add_ps(second, sum_c));
}

// Translated from 00e29620 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: inverts the 6x6 matrix at `source` (`this` = `target` receives
/// the inverse; one stack word, `RET 4`) by Gauss-Jordan elimination without pivoting. The
/// matrix is twelve vectors (three components of each used): row `i < 3` is vector `i` (columns
/// 0-2) and vector `6 + i` (columns 3-5), row `3 + i` is vector `3 + i` and vector `9 + i`.
/// A working copy of the source lives on the stack; the target starts as the identity (ones at
/// vector 0 x, 1 y, 2 z, 9 x, 10 y and 11 z, everything else zero). For each pivot `(b, c)`
/// (`b` in 0..2, `c` in 0..3, the diagonal element `3b + c`, read from vector `9b + c`
/// component `c`) the four vectors of its row (`work[u]`, `work[6 + u]`, `target[u]` and
/// `target[6 + u]`, `u = 3b + c`) are multiplied by `1 / pivot` (x87 division of `1.0`, stored as
/// a `float`); then for every other row `(e, r)` the factor `-work[r + 3 * (e + 2b)][c]` times
/// the pivot row is added to row `3e + r` (`MULPS`, then `ADDPS` with the product first). The
/// reciprocal and the factor go through the x87 stack (`FLD`, `FDIVR`, `FCHS`, `FSTP`), so
/// signalling NaNs come out quiet.
pub fn fn_00e29620(e: &mut Engine, this: Ptr, source: Ptr) {
    let inverse = this.addr();
    let mut work: [Lanes; 12] =
        std::array::from_fn(|vector| load_lanes(e, source.addr() + 16 * vector as u32));
    for word in 0..48u32 {
        e.mem.set_u32(inverse + 4 * word, 0);
    }
    for (vector, component) in [(0u32, 0u32), (1, 1), (2, 2), (9, 0), (10, 1), (11, 2)] {
        e.mem
            .set_u32(inverse + 16 * vector + 4 * component, 0x3f80_0000);
    }
    for b in 0..2usize {
        for c in 0..3usize {
            let u = 3 * b + c;
            let reciprocal = x87_store32(x87_div(1.0, x87_load32(work[9 * b + c][c])));
            let scale = [reciprocal; 4];
            work[u] = mul_ps(scale, work[u]);
            work[6 + u] = mul_ps(scale, work[6 + u]);
            for vector in [u as u32, 6 + u as u32] {
                let row = load_lanes(e, inverse + 16 * vector);
                store_lanes(e, inverse + 16 * vector, mul_ps(scale, row));
            }
            for block_row in 0..2usize {
                for r in 0..3usize {
                    if block_row == b && r == c {
                        continue;
                    }
                    let factor = x87_quiet_bits(work[r + 3 * (block_row + 2 * b)][c]) ^ 0x8000_0000;
                    let factor = [factor; 4];
                    let target = 3 * block_row + r;
                    work[target] = add_ps(mul_ps(factor, work[u]), work[target]);
                    work[6 + target] = add_ps(mul_ps(factor, work[6 + u]), work[6 + target]);
                    for (from, to) in [(u, target), (6 + u, 6 + target)] {
                        let pivot_row = load_lanes(e, inverse + 16 * from as u32);
                        let old = load_lanes(e, inverse + 16 * to as u32);
                        store_lanes(
                            e,
                            inverse + 16 * to as u32,
                            add_ps(mul_ps(factor, pivot_row), old),
                        );
                    }
                }
            }
        }
    }
}

// Translated from 00e298f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk with two stack words: `p1` goes in `ECX` and `p2` on the stack to `00e29620` (the
/// 6x6 inverse: `this` = `p1`, source = `p2`). The function pops none (`RET`); the result in
/// `EAX` is the callee's leftover and not meaningful.
pub fn fn_00e298f0(e: &mut Engine, this: Ptr, source: Ptr) {
    fn_00e29620(e, this, source);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e29910, fn_00e29910() -> u64),
        entry!(0x00e2_9250, fn_00e29250(Ptr, Ptr)),
        entry!(0x00e2_9290, fn_00e29290(Ptr, Ptr)),
        entry!(0x00e2_9380, fn_00e29380(Ptr, Ptr)),
        entry!(0x00e2_9470, fn_00e29470(Ptr, Ptr, Ptr)),
        entry!(0x00e2_9510, fn_00e29510(Ptr, Ptr, Ptr)),
        entry!(0x00e2_9620, fn_00e29620(Ptr, Ptr)),
        entry!(0x00e2_98f0, fn_00e298f0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e29910_returns_the_time_of_a_null_pointer_call() {
        let mut e = Engine::new();
        e.register(TIME64, |_, a| {
            assert_eq!(a, [0]);
            Ret {
                eax: 0x1234_5678,
                edx: 0x9,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00e2_9910, &args![]).u64(), 0x9_1234_5678);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SET_TRANSPOSE: u32 = 0x00cd_4ba0;
    const SET_MUL: u32 = 0x00cd_5530;

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|index| e.mem.f32(at + 4 * index)).collect()
    }

    /// Four calls of the 3x3 transpose, the blocks 1 and 2 swapped.
    #[test]
    fn fn_00e29250_transposes_the_four_blocks() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let transpose_log = Rc::clone(&log);
        e.register_double(SET_TRANSPOSE, move |_, a| {
            transpose_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00e2_9250, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x1000, 0x2000],
                vec![0x1030, 0x2060],
                vec![0x1060, 0x2030],
                vec![0x1090, 0x2090]
            ]
        );
    }

    /// Sums of the twelve vectors, with the NaN of the first operand when both lanes are NaNs:
    /// vector 0 is target + source, vector 1 is source + target.
    #[test]
    fn fn_00e29290_adds_and_orders_nans() {
        let mut e = Engine::new();
        let target = e.mem.alloc(0xc0);
        let source = e.mem.alloc(0xc0);
        for word in 0..48u32 {
            e.mem.set_f32(target + 4 * word, word as f32);
            e.mem.set_f32(source + 4 * word, 100.0 + 2.0 * word as f32);
        }
        // Lane 0 of vectors 0 and 1: two different NaNs in both operands.
        e.mem.set_u32(target, 0x7fc0_0001);
        e.mem.set_u32(source, 0x7fc0_0002);
        e.mem.set_u32(target + 0x10, 0x7fc0_0003);
        e.mem.set_u32(source + 0x10, 0x7fc0_0004);
        // Lane 1 of vector 4: a signalling NaN in the source only.
        e.mem.set_u32(source + 0x44, 0x7f80_0005);
        e.call(0x00e2_9290, &args![target, source]);
        assert_eq!(e.mem.u32(target), 0x7fc0_0001);
        assert_eq!(e.mem.u32(target + 0x10), 0x7fc0_0004);
        assert_eq!(e.mem.u32(target + 0x44), 0x7fc0_0005);
        for word in [1u32, 2, 3, 5, 47] {
            let expected = word as f32 + 100.0 + 2.0 * word as f32;
            assert_eq!(e.mem.f32(target + 4 * word), expected);
        }
        // The source is unchanged.
        assert_eq!(e.mem.f32(source + 4), 102.0);
    }

    /// Differences, target minus source, with the target's NaN winning.
    #[test]
    fn fn_00e29380_subtracts_and_orders_nans() {
        let mut e = Engine::new();
        let target = e.mem.alloc(0xc0);
        let source = e.mem.alloc(0xc0);
        for word in 0..48u32 {
            e.mem.set_f32(target + 4 * word, 10.0 * word as f32);
            e.mem.set_f32(source + 4 * word, word as f32);
        }
        e.mem.set_u32(target + 0x10, 0x7fc0_0003);
        e.mem.set_u32(source + 0x10, 0x7fc0_0004);
        e.mem.set_u32(source + 0x14, 0xff80_0006);
        e.call(0x00e2_9380, &args![target, source]);
        assert_eq!(e.mem.u32(target + 0x10), 0x7fc0_0003);
        assert_eq!(e.mem.u32(target + 0x14), 0xffc0_0006);
        for word in [0u32, 2, 3, 6, 47] {
            assert_eq!(e.mem.f32(target + 4 * word), 9.0 * word as f32);
        }
    }

    /// Eight block products (the doubles write a pattern that names their operands) and the
    /// sums into the four target blocks.
    #[test]
    fn fn_00e29470_adds_the_two_block_products() {
        let mut e = Engine::new();
        let target = e.mem.alloc(0xc0);
        let a = e.mem.alloc(0xc0);
        let b = e.mem.alloc(0xc0);
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let mul_log = Rc::clone(&log);
        e.register_double(SET_MUL, move |e, words| {
            let (out, left, right) = (words[0], words[1], words[2]);
            mul_log.borrow_mut().push(words.to_vec());
            let left_block = (left - a) / 0x30;
            let right_block = (right - b) / 0x30;
            for word in 0..12u32 {
                let value = (10 * left_block + right_block) as f32 + word as f32 / 64.0;
                e.mem.set_f32(out + 4 * word, value);
            }
            Ret::default()
        });
        for word in 0..48u32 {
            e.mem.set_f32(target + 4 * word, 1000.0);
        }
        e.call(0x00e2_9470, &args![target, a, b]);
        let log = log.borrow();
        assert_eq!(log.len(), 8);
        let mut call = 0;
        for row in 0..2u32 {
            for column in 0..2u32 {
                let first = &log[call];
                let second = &log[call + 1];
                assert_eq!(first[1], a + 0x60 * row);
                assert_eq!(first[2], b + 0x30 * column);
                assert_eq!(second[0], target + 0x30 * (2 * row + column));
                assert_eq!(second[1], a + 0x60 * row + 0x30);
                assert_eq!(second[2], b + 0x30 * column + 0x60);
                call += 2;
            }
        }
        for row in 0..2u32 {
            for column in 0..2u32 {
                let block = target + 0x30 * (2 * row + column);
                for word in 0..12u32 {
                    let first = (10 * (2 * row) + column) as f32 + word as f32 / 64.0;
                    let second = (10 * (2 * row + 1) + 2 + column) as f32 + word as f32 / 64.0;
                    assert_eq!(e.mem.f32(block + 4 * word), first + second);
                }
            }
        }
    }

    /// With b0 = (1, 2, 3), b1 = (4, 5, 6) and matrix vectors that hold their index in lane 0,
    /// the two result vectors are the weighted sums of the matrix vectors.
    #[test]
    fn fn_00e29510_multiplies_the_vector_by_the_blocks() {
        let mut e = Engine::new();
        let target = e.mem.alloc(0x20);
        let a = e.mem.alloc(0xc0);
        let b = e.mem.alloc(0x20);
        for vector in 0..12u32 {
            let k = vector as f32;
            set_floats(&mut e, a + 0x10 * vector, &[k + 1.0, 100.0 + k, -k, 0.5]);
        }
        set_floats(&mut e, b, &[1.0, 2.0, 3.0, 0.0]);
        set_floats(&mut e, b + 0x10, &[4.0, 5.0, 6.0, 0.0]);
        e.call(0x00e2_9510, &args![target, a, b]);
        let vector = |k: u32| {
            let k = k as f32;
            [k + 1.0, 100.0 + k, -k, 0.5]
        };
        let weighted = |weights: [f32; 3], first: u32| {
            let mut sum = [0.0f32; 4];
            for (index, weight) in weights.into_iter().enumerate() {
                for (lane, value) in vector(first + index as u32).into_iter().enumerate() {
                    sum[lane] += weight * value;
                }
            }
            sum
        };
        let low = weighted([1.0, 2.0, 3.0], 0);
        let low_b = weighted([4.0, 5.0, 6.0], 3);
        let high = weighted([1.0, 2.0, 3.0], 6);
        let high_b = weighted([4.0, 5.0, 6.0], 9);
        let expected_first: Vec<f32> = (0..4).map(|lane| low_b[lane] + low[lane]).collect();
        let expected_second: Vec<f32> = (0..4).map(|lane| high_b[lane] + high[lane]).collect();
        assert_eq!(floats(&e, target, 4), expected_first);
        assert_eq!(floats(&e, target + 0x10, 4), expected_second);
    }

    /// The 6x6 matrix `rows[i][j]` as twelve vectors: row `i` is vector `i` (columns 0-2) and
    /// vector `6 + i` (columns 3-5), the same for rows 3-5 with vectors 3-5 and 9-11.
    fn put_matrix(e: &mut Engine, at: u32, rows: &[[f32; 6]; 6]) {
        for (i, row) in rows.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                let vector = if j < 3 { i } else { 6 + i };
                e.mem
                    .set_f32(at + 16 * vector as u32 + 4 * (j % 3) as u32, *value);
            }
        }
    }

    fn get_matrix(e: &Engine, at: u32) -> [[f32; 6]; 6] {
        std::array::from_fn(|i| {
            std::array::from_fn(|j| {
                let vector = if j < 3 { i } else { 6 + i };
                e.mem.f32(at + 16 * vector as u32 + 4 * (j % 3) as u32)
            })
        })
    }

    /// An upper-triangular matrix with power-of-two diagonal (so every step is exact): the
    /// result times the matrix is the identity, the matrix is not changed, and the padding
    /// lane of the target is zero.
    #[test]
    fn fn_00e29620_inverts_a_matrix() {
        let rows = [
            [2.0, 1.0, 0.0, 3.0, 0.0, 0.0],
            [0.0, 4.0, 1.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 2.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 2.0, 1.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 4.0],
        ];
        let mut e = Engine::new();
        let source = e.mem.alloc(0xc0);
        let target = e.mem.alloc(0xc0);
        put_matrix(&mut e, source, &rows);
        e.mem.write(target, &[0x5a; 0xc0]);
        e.call(0x00e2_9620, &args![target, source]);
        assert_eq!(get_matrix(&e, source), rows);
        let inverse = get_matrix(&e, target);
        for (i, row) in rows.iter().enumerate() {
            for j in 0..6 {
                let product: f32 = row
                    .iter()
                    .zip(inverse.iter())
                    .map(|(value, inverse_row)| value * inverse_row[j])
                    .sum();
                assert_eq!(product, if i == j { 1.0 } else { 0.0 }, "({i}, {j})");
            }
        }
        for vector in 0..12u32 {
            assert_eq!(e.mem.u32(target + 16 * vector + 12), 0);
        }
    }

    /// The inverse of a diagonal matrix is the diagonal of reciprocals.
    #[test]
    fn fn_00e29620_inverts_a_diagonal_matrix() {
        let mut rows = [[0.0f32; 6]; 6];
        for (i, diagonal) in [2.0f32, 4.0, 8.0, 0.5, 0.25, 16.0].into_iter().enumerate() {
            rows[i][i] = diagonal;
        }
        let mut e = Engine::new();
        let source = e.mem.alloc(0xc0);
        let target = e.mem.alloc(0xc0);
        put_matrix(&mut e, source, &rows);
        e.call(0x00e2_9620, &args![target, source]);
        let inverse = get_matrix(&e, target);
        for (i, row) in inverse.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                let expected = if i == j { 1.0 / rows[i][i] } else { 0.0 };
                assert_eq!(*value, expected, "({i}, {j})");
            }
        }
    }

    /// The thunk passes its two words on to the inverse.
    #[test]
    fn fn_00e298f0_runs_the_inverse() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0xc0);
        let target = e.mem.alloc(0xc0);
        let mut rows = [[0.0f32; 6]; 6];
        for (i, row) in rows.iter_mut().enumerate() {
            row[i] = 2.0;
        }
        put_matrix(&mut e, source, &rows);
        e.call(0x00e2_98f0, &args![target, source]);
        assert_eq!(get_matrix(&e, target)[3][3], 0.5);
    }
}
