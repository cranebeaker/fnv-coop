//! `Havok SDK/hkpchainbuildjacobian.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions here walk the links of a ball and socket chain block by block. A block is
//! three 3-row matrices (`0x30` bytes each, one `0x10`-byte row per vector) in the matrix
//! array (`0x90` bytes per block), and a link names its two bodies by the byte offsets
//! `links[i]` and `links[i + 1]` into the body array (the body's 4-vector at `+0x30` has the
//! factor in its fourth lane). The output rows of a block (`0x90` bytes: the matrix `+0`
//! made from the previous block's carry, `+0x30` the diagonal block, `+0x60` its product with
//! the carry of this block) are what the two functions fill.
//!
//! Both are partial: lane 3 of the rows they build is read from the stack frame before it is
//! written (modelled as 0), and the x87 stack is left one entry deeper by every block (the
//! exe overflows it after about six blocks; not modelled).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, sum3, zip_with, Vec4};

/// `0x00cd5530` (cdecl, `(destination, a, b)`): the product of two 3-row matrices, row `i` of
/// the destination being `b[i].x * a[0] + b[i].y * a[1] + b[i].z * a[2]`.
const MULTIPLY_ROWS: u32 = 0x00cd_5530;
/// `0x00cd55a0` (cdecl, one argument): works on the 3-row matrix it is given.
const PREPARE_ROWS: u32 = 0x00cd_55a0;
/// The float the diagonal of each block is kept above (`2^-23`).
const EPSILON: u32 = 0x0104_da4c;
/// The double added to the first word of the output header (`2^-23`).
const HEADER_BIAS: u32 = 0x010c_9300;

/// Scratch block of the block loop: the carried product (three rows), the result of the
/// first multiplication and the new carry matrix.
const CARRIED_PRODUCT: u32 = 0;
const FIRST_PRODUCT: u32 = 0x30;
const CARRY_MATRIX: u32 = 0x60;

fn load_rows(e: &Engine, at: u32) -> [Vec4; 3] {
    [0, 0x10, 0x20].map(|offset| load_vec(e, at + offset))
}

fn product(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

/// The block loop both functions share, for `count > 0` blocks. `rows` is the output (`0x90`
/// bytes a block), `diagonal` the float added to the diagonal of every block's `Q`.
///
/// For block `i` with `A` the 4-vector of body `links[i]` and `B` that of `links[i + 1]`, and
/// `M[j]` the matrices of the block: `Q[j][k]` (symmetric, `k >= j`) is the sum of the first
/// three lanes of `(A.w * T + B.w * T) + ((N1 * M1) * A + (N2 * M2) * B)` where `N = M[k]`,
/// `M = M[j]`, `T = N0 * M0` (row-wise, lane-wise products) and the diagonal gets
/// `diagonal` added in x87. Unless it is the last block, `R[k][r]` is the sum of the first
/// three lanes of `(N'[k]1 * M[r]2) * B - B.w * (N'[k]0 * M[r]0)` where `N'` are the matrices
/// of the next block; else `R` is 0. The block's matrix `+0` is made from the carried `R` of
/// the previous block, transposed (lane 3 of its rows is the last word of the carried matrix);
/// the function at `0x00cd5530` multiplies it with the carried product, `Q` minus that
/// product goes to `+0x30` with its diagonal clamped from below to the epsilon (`NaN`
/// included), `0x00cd55a0` is run on it, and `+0x60` becomes its product with `R`. `R` and
/// that product are carried to the next block.
fn build_blocks(
    e: &mut Engine,
    rows: u32,
    count: i32,
    diagonal: f32,
    matrices: u32,
    links: u32,
    bodies: u32,
) {
    let epsilon = e.mem.f32(EPSILON);
    let diagonal = f64::from(x87_quiet(diagonal));
    e.with_stack(0x90, |e, scratch| {
        let scratch = scratch.addr();
        let (carried_product, first_product, carry_matrix) = (
            scratch + CARRIED_PRODUCT,
            scratch + FIRST_PRODUCT,
            scratch + CARRY_MATRIX,
        );
        let mut carry = [[0.0f32; 4]; 3];
        let mut block = 0i32;
        while block < count {
            let index = block as u32;
            let output = rows + 0x90 * index;
            let matrix_at =
                |block: u32, which: usize| matrices + 0x90 * block + 0x30 * which as u32;
            let first = e.mem.u32(links + 4 * index);
            let second = e.mem.u32(links + 4 * index + 4);
            let body_a = load_vec(e, bodies.wrapping_add(first).wrapping_add(0x30));
            let body_b = load_vec(e, bodies.wrapping_add(second).wrapping_add(0x30));
            let mut q = [[0.0f32; 4]; 3];
            for j in 0..3 {
                let m = load_rows(e, matrix_at(index, j));
                for k in j..3 {
                    let n = load_rows(e, matrix_at(index, k));
                    let t = product(n[0], m[0]);
                    let weighted = zip_with(
                        product([body_a[3]; 4], t),
                        product([body_b[3]; 4], t),
                        |a, b| a + b,
                    );
                    let along = zip_with(
                        product(product(n[1], m[1]), body_a),
                        product(product(n[2], m[2]), body_b),
                        |a, b| a + b,
                    );
                    let sum = sum3(zip_with(weighted, along, |a, b| a + b));
                    q[k][j] = sum;
                    q[j][k] = sum;
                }
                q[j][j] = (f64::from(q[j][j]) + diagonal) as f32;
            }
            let mut r = [[0.0f32; 4]; 3];
            if block < count.wrapping_sub(1) {
                for which in 0..3 {
                    let m = load_rows(e, matrix_at(index, which));
                    for k in 0..3 {
                        let n = load_rows(e, matrix_at(index + 1, k));
                        let weighted = product([body_b[3]; 4], product(n[0], m[0]));
                        let crossed = product(product(n[1], m[2]), body_b);
                        r[k][which] = sum3(zip_with(crossed, weighted, |a, b| a - b));
                    }
                }
            }
            for (row, lanes) in r.iter().enumerate() {
                store_vec(e, carry_matrix + 0x10 * row as u32, *lanes);
            }
            let last = x87_quiet(carry[2][2]);
            for row in 0..3 {
                let lanes = [
                    x87_quiet(carry[0][row]),
                    x87_quiet(carry[1][row]),
                    x87_quiet(carry[2][row]),
                    last,
                ];
                store_vec(e, output + 0x10 * row as u32, lanes);
            }
            e.call(MULTIPLY_ROWS, &[first_product, output, carried_product]);
            for (row, lanes) in q.iter().enumerate() {
                let offset = 0x10 * row as u32;
                let difference =
                    zip_with(*lanes, load_vec(e, first_product + offset), |a, b| a - b);
                store_vec(e, output + 0x30 + offset, difference);
            }
            for at in [0x30, 0x44, 0x58] {
                let value = x87_quiet(e.mem.f32(output + at));
                e.mem
                    .set_f32(output + at, if value > epsilon { value } else { epsilon });
            }
            e.call(PREPARE_ROWS, &[output + 0x30]);
            e.call(MULTIPLY_ROWS, &[output + 0x60, output + 0x30, carry_matrix]);
            carry = [0, 0x10, 0x20].map(|offset| load_vec(e, carry_matrix + offset));
            for offset in [0, 0x10, 0x20] {
                let lanes = load_vec(e, output + 0x60 + offset);
                store_vec(e, carried_product + offset, lanes);
            }
            block += 1;
        }
    });
}

// Translated from 00dd0660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00dd0660 - lane 3 of the Q rows is read from the stack frame before any write (modelled as 0) and the x87 stack gains one entry per block (overflow from about the seventh block not modelled).
/// `buildConstraintMatrixLuDecompositionForBallAndSocketChain` (name from the engine map,
/// unconfirmed; cdecl, `rows` in `ECX`, so the register comes first): for `count` blocks (none
/// when `count <= 0`) of the `matrices` array it fills the output `rows` (`0x90` bytes a
/// block) as described at `build_blocks`: `diagonal` is added to the diagonal of each
/// block's matrix, `links` has the byte offsets of the bodies in `bodies`. Calls `0x00cd5530`
/// and `0x00cd55a0` for the matrix products.
pub fn build_constraint_matrix_lu_decomposition_for_ball_and_socket_chain(
    e: &mut Engine,
    rows: Ptr,
    count: i32,
    diagonal: f32,
    matrices: Ptr,
    links: Ptr,
    bodies: Ptr,
) {
    if count <= 0 {
        return;
    }
    build_blocks(
        e,
        rows.addr(),
        count,
        diagonal,
        matrices.addr(),
        links.addr(),
        bodies.addr(),
    );
}

// Translated from 00dd0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00dd0ad0 - lane 3 of the Q rows is read from the stack frame before any write (modelled as 0) and the x87 stack gains one entry per block (overflow from about the seventh block not modelled).
/// `hkBallSocketChainBuildJacobian` (name from the engine map, unconfirmed; cdecl): writes the
/// header of a Jacobian record and its blocks to the buffer `*out` points at, then moves
/// `*out` on by the record size. With `n16` the low 16 bits of `count`: the header word `+0`
/// becomes the float it holds plus `2^-23` (x87, rounded to a float) with its bytes `+0` and
/// `+2`/`+3` then replaced by `0x1E` and `n16`, `+8` and `+0xC` get the two floats (x87
/// copies), `+4` the size `(n16 * 0x134 + 0x33) & !0xF`. The `count + 1` words of `links`
/// (when `count >= 0`) are copied to `+0x20 + n16 * 0x130`. For `count > 0` the matrices
/// (the buffer's `+0x10`, `0x90` bytes a block) are already in the buffer: the output rows
/// follow them (`+0x10 + n16 * 0x90`) and are filled with the copy of the links as described
/// at `build_blocks` (`diagonal` is added to each block's diagonal). The two words after
/// `bodies` are never read.
pub fn hk_ball_socket_chain_build_jacobian(
    e: &mut Engine,
    count: i32,
    word_8: f32,
    word_c: f32,
    diagonal: f32,
    links: Ptr,
    bodies: Ptr,
    _unused_7: u32,
    _unused_8: u32,
    out: Ptr,
) {
    let out = out.addr();
    let header = e.mem.u32(out);
    let biased = f64::from(x87_quiet(e.mem.f32(header))) + e.mem.f64(HEADER_BIAS);
    e.mem.set_f32(header, biased as f32);
    e.mem.set_f32(header + 8, x87_quiet(word_8));
    e.mem.set_f32(header + 0xc, x87_quiet(word_c));
    let n16 = count as u32 & 0xffff;
    e.mem.set_u32(header + 4, (n16 * 0x134 + 0x33) & !0xf);
    e.mem.set_u8(header, 0x1e);
    e.mem.set_u16(header + 2, n16 as u16);
    let copy = header + 0x20 + n16 * 0x130;
    for word in 0..count.wrapping_add(1).max(0) as u32 {
        let value = e.mem.u32(links.addr() + 4 * word);
        e.mem.set_u32(copy + 4 * word, value);
    }
    if count > 0 {
        let matrices = e.mem.u32(out) + 0x10;
        build_blocks(
            e,
            matrices + 0x90 * n16,
            count,
            diagonal,
            matrices,
            copy,
            bodies.addr(),
        );
    }
    let size = e.mem.u32(header + 4);
    let moved = e.mem.u32(out).wrapping_add(size);
    e.mem.set_u32(out, moved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dd_0660,
            build_constraint_matrix_lu_decomposition_for_ball_and_socket_chain(
                Ptr, i32, f32, Ptr, Ptr, Ptr
            )
        ),
        entry!(
            0x00dd_0ad0,
            hk_ball_socket_chain_build_jacobian(i32, f32, f32, f32, Ptr, Ptr, u32, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const EPSILON_VALUE: f32 = 1.192_092_9e-7;

    /// One call of a helper: its address, its words, and the matrices at its pointers when
    /// it was called.
    struct Call {
        at: u32,
        args: Vec<u32>,
        first: [Vec4; 3],
        second: [Vec4; 3],
    }

    type Calls = Rc<RefCell<Vec<Call>>>;

    /// Maps the exe constants and registers doubles of the two helpers. The multiplication
    /// writes row `t` of its destination as `scale * (t + 1)` in every lane, with `scale`
    /// the given one for the first multiplication of a block and 10 for the second.
    fn scene(first_scale: f32) -> (Engine, Calls) {
        let mut e = Engine::new();
        e.map(EPSILON, 4);
        e.map(HEADER_BIAS, 8);
        e.mem.set_f32(EPSILON, EPSILON_VALUE);
        e.mem.set_f64(HEADER_BIAS, f64::from(EPSILON_VALUE));
        let calls = Calls::default();
        let seen = Rc::clone(&calls);
        e.register_double(MULTIPLY_ROWS, move |e, a| {
            let earlier = seen
                .borrow()
                .iter()
                .filter(|call| call.at == MULTIPLY_ROWS)
                .count();
            let scale = if earlier % 2 == 0 { first_scale } else { 10.0 };
            seen.borrow_mut().push(Call {
                at: MULTIPLY_ROWS,
                args: a.to_vec(),
                first: load_rows(e, a[1]),
                second: load_rows(e, a[2]),
            });
            for row in 0..3u32 {
                store_vec(e, a[0] + 0x10 * row, [scale * (row + 1) as f32; 4]);
            }
            Ret::default()
        });
        let seen = Rc::clone(&calls);
        e.register_double(PREPARE_ROWS, move |e, a| {
            seen.borrow_mut().push(Call {
                at: PREPARE_ROWS,
                args: a.to_vec(),
                first: load_rows(e, a[0]),
                second: [[0.0; 4]; 3],
            });
            Ret::default()
        });
        (e, calls)
    }

    #[derive(Clone, Copy)]
    struct Layout {
        rows: u32,
        matrices: u32,
        links: u32,
        bodies: u32,
    }

    fn element(block: u32, matrix: u32, row: u32, lane: u32) -> f32 {
        ((block * 5 + matrix * 3 + row * 2 + lane * 7) % 4) as f32 - 1.0
    }

    fn body_vector(body: u32) -> Vec4 {
        [
            0.5 * (body + 1) as f32,
            1.0 + body as f32,
            2.0 - 0.25 * body as f32,
            4.0 * (body + 1) as f32,
        ]
    }

    /// Fills the matrices of `blocks` blocks (entries in -1..=2), the bodies (`0x40` bytes
    /// each, the vector at `+0x30`) and the link offsets of the bodies.
    fn fill(e: &mut Engine, blocks: u32, layout: Layout) {
        for block in 0..blocks {
            for matrix in 0..3 {
                for row in 0..3 {
                    let lanes = [0, 1, 2, 3].map(|lane| element(block, matrix, row, lane));
                    let at = layout.matrices + 0x90 * block + 0x30 * matrix + 0x10 * row;
                    store_vec(e, at, lanes);
                }
            }
        }
        for body in 0..=blocks {
            store_vec(e, layout.bodies + 0x40 * body + 0x30, body_vector(body));
            e.mem.set_u32(layout.links + 4 * body, 0x40 * body);
        }
    }

    fn first_lanes(v: Vec4) -> [f64; 3] {
        [f64::from(v[0]), f64::from(v[1]), f64::from(v[2])]
    }

    fn rows_of(m: [Vec4; 3]) -> [[f64; 3]; 3] {
        m.map(first_lanes)
    }

    /// Checks the calls and the rows of `blocks` blocks against the lane by lane sums of
    /// the algorithm (all the numbers are small multiples of 0.25, so the sums are exact).
    fn check_blocks(
        e: &Engine,
        calls: &Calls,
        layout: Layout,
        blocks: u32,
        diagonal: f32,
        first_scale: f32,
    ) {
        let calls = calls.borrow();
        assert_eq!(calls.len(), 3 * blocks as usize);
        let epsilon = f64::from(EPSILON_VALUE);
        let matrix =
            |block: u32, which: u32, row: u32, lane: usize| {
                f64::from(e.mem.f32(
                    layout.matrices + 0x90 * block + 0x30 * which + 0x10 * row + 4 * lane as u32,
                ))
            };
        let body = |block: u32, lane: usize| {
            let offset = e.mem.u32(layout.links + 4 * block);
            f64::from(e.mem.f32(layout.bodies + offset + 0x30 + 4 * lane as u32))
        };
        let mut carry = [[0.0f64; 3]; 3];
        let mut carried_product = [[0.0f64; 4]; 3];
        let mut shared: Option<(u32, u32, u32)> = None;
        for block in 0..blocks {
            let output = layout.rows + 0x90 * block;
            let (a, b) = (
                |lane: usize| body(block, lane),
                |lane: usize| body(block + 1, lane),
            );
            let q = |j: u32, k: u32| {
                let mut total = 0.0;
                for lane in 0..3 {
                    let t = matrix(block, k, 0, lane) * matrix(block, j, 0, lane);
                    total += (a(3) + b(3)) * t
                        + (matrix(block, k, 1, lane) * matrix(block, j, 1, lane)) * a(lane)
                        + (matrix(block, k, 2, lane) * matrix(block, j, 2, lane)) * b(lane);
                }
                if j == k {
                    total += f64::from(diagonal);
                }
                total
            };
            let mut r = [[0.0f64; 4]; 3];
            if block + 1 < blocks {
                for which in 0..3u32 {
                    for k in 0..3u32 {
                        let mut total = 0.0;
                        for lane in 0..3 {
                            total += (matrix(block + 1, k, 1, lane)
                                * matrix(block, which, 2, lane))
                                * b(lane)
                                - b(3)
                                    * (matrix(block + 1, k, 0, lane)
                                        * matrix(block, which, 0, lane));
                        }
                        r[k as usize][which as usize] = total;
                    }
                }
            }
            // The multiplication of the block's first matrix with the carried product.
            let first = &calls[3 * block as usize];
            assert_eq!(first.at, MULTIPLY_ROWS);
            assert_eq!(first.args[1], output);
            let (first_product, carried_at, carry_matrix) =
                *shared.get_or_insert((first.args[0], first.args[2], calls[2].args[2]));
            assert_eq!(first.args[0], first_product);
            assert_eq!(first.args[2], carried_at);
            let transposed =
                [0, 1, 2].map(|row| [carry[0][row], carry[1][row], carry[2][row], carry[2][2]]);
            for (seen, expected) in first.first.iter().zip(transposed) {
                assert_eq!(first_lanes(*seen), [expected[0], expected[1], expected[2]]);
                assert_eq!(f64::from(seen[3]), expected[3]);
            }
            assert_eq!(first.second.map(|row| row.map(f64::from)), carried_product);
            // The diagonal block, Q minus the first product, its diagonal kept above epsilon.
            let mut expected_block = [[0.0f64; 3]; 3];
            for (row, lanes) in expected_block.iter_mut().enumerate() {
                for (lane, value) in lanes.iter_mut().enumerate() {
                    let (j, k) = (row.min(lane) as u32, row.max(lane) as u32);
                    let difference = q(j, k) - f64::from(first_scale) * (row + 1) as f64;
                    *value = if row == lane && difference <= epsilon {
                        epsilon
                    } else {
                        difference
                    };
                }
            }
            let prepare = &calls[3 * block as usize + 1];
            assert_eq!(prepare.at, PREPARE_ROWS);
            assert_eq!(prepare.args, vec![output + 0x30]);
            assert_eq!(rows_of(prepare.first), expected_block);
            let second = &calls[3 * block as usize + 2];
            assert_eq!(second.at, MULTIPLY_ROWS);
            assert_eq!(
                second.args,
                vec![output + 0x60, output + 0x30, carry_matrix]
            );
            assert_eq!(rows_of(second.first), expected_block);
            assert_eq!(second.second.map(|row| row.map(f64::from)), r);
            // What is left in the output.
            assert_eq!(
                rows_of(load_rows(e, output)),
                transposed.map(|row| [row[0], row[1], row[2]])
            );
            assert_eq!(f64::from(e.mem.f32(output + 0xc)), transposed[0][3]);
            assert_eq!(rows_of(load_rows(e, output + 0x30)), expected_block);
            carried_product = [10.0, 20.0, 30.0].map(|value| [value; 4]);
            for (row, expected) in carried_product.iter().enumerate() {
                let found = load_vec(e, output + 0x60 + 0x10 * row as u32);
                assert_eq!(found.map(f64::from), *expected);
            }
            carry = r.map(|row| [row[0], row[1], row[2]]);
        }
        assert_ne!(shared.map(|s| s.1), shared.map(|s| s.2));
    }

    fn chain_layout(e: &mut Engine, blocks: u32) -> Layout {
        let layout = Layout {
            rows: e.mem.alloc(0x90 * blocks.max(1)),
            matrices: e.mem.alloc(0x90 * blocks.max(1)),
            links: e.mem.alloc(4 * blocks + 4),
            bodies: e.mem.alloc(0x40 * blocks + 0x40),
        };
        fill(e, blocks, layout);
        layout
    }

    fn run_0660(e: &mut Engine, layout: Layout, count: i32, diagonal: f32) {
        e.call(
            0x00dd_0660,
            &args![
                layout.rows,
                count,
                diagonal,
                layout.matrices,
                layout.links,
                layout.bodies
            ],
        );
    }

    #[test]
    fn two_blocks_carry_the_r_matrix_and_the_product_00dd0660() {
        let (mut e, calls) = scene(0.5);
        let layout = chain_layout(&mut e, 2);
        run_0660(&mut e, layout, 2, 0.5);
        check_blocks(&e, &calls, layout, 2, 0.5, 0.5);
    }

    #[test]
    fn a_large_first_product_clamps_the_diagonal_to_epsilon_00dd0660() {
        let (mut e, calls) = scene(1000.0);
        let layout = chain_layout(&mut e, 3);
        run_0660(&mut e, layout, 3, 0.25);
        check_blocks(&e, &calls, layout, 3, 0.25, 1000.0);
        // All three diagonal words of the block are the epsilon, the others are not.
        let block = load_rows(&e, layout.rows + 0x30);
        assert_eq!([block[0][0], block[1][1], block[2][2]], [EPSILON_VALUE; 3]);
        assert!(block[0][1] < -900.0);
    }

    #[test]
    fn a_nan_diagonal_becomes_epsilon_and_one_block_has_no_r_00dd0660() {
        let (mut e, calls) = scene(0.5);
        let layout = chain_layout(&mut e, 1);
        // The factor of the first body is NaN: every Q word is.
        e.mem.set_f32(layout.bodies + 0x30 + 0xc, f32::NAN);
        run_0660(&mut e, layout, 1, 0.5);
        let block = load_rows(&e, layout.rows + 0x30);
        assert_eq!([block[0][0], block[1][1], block[2][2]], [EPSILON_VALUE; 3]);
        assert!(block[0][1].is_nan() && block[2][0].is_nan() && block[1][2].is_nan());
        let calls = calls.borrow();
        // Only one block: R is 0 and the carried words are 0.
        assert_eq!(calls[2].second, [[0.0; 4]; 3]);
        assert_eq!(calls[0].second, [[0.0; 4]; 3]);
    }

    #[test]
    fn a_signalling_nan_diagonal_word_is_quieted_00dd0660() {
        let (mut e, _calls) = scene(0.5);
        let layout = chain_layout(&mut e, 1);
        run_0660(&mut e, layout, 1, f32::from_bits(0x7f80_0001));
        let block = load_rows(&e, layout.rows + 0x30);
        assert_eq!([block[0][0], block[1][1], block[2][2]], [EPSILON_VALUE; 3]);
    }

    #[test]
    fn no_blocks_do_nothing_00dd0660() {
        for count in [0, -1] {
            let (mut e, calls) = scene(0.5);
            let layout = chain_layout(&mut e, 2);
            e.mem.set_u32(layout.rows, 0x1234_5678);
            run_0660(&mut e, layout, count, 0.5);
            assert!(calls.borrow().is_empty());
            assert_eq!(e.mem.u32(layout.rows), 0x1234_5678);
        }
    }

    /// A buffer for a record of `blocks` blocks, its pointer cell and the original links.
    struct Record {
        buffer: u32,
        cell: u32,
        links: u32,
        bodies: u32,
    }

    fn record(e: &mut Engine, blocks: u32) -> Record {
        let buffer = e.mem.alloc(0x400);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, buffer);
        let links = e.mem.alloc(4 * blocks + 4);
        let bodies = e.mem.alloc(0x40 * blocks + 0x40);
        let layout = Layout {
            rows: 0,
            matrices: buffer + 0x10,
            links,
            bodies,
        };
        fill(e, blocks, layout);
        // The first header word before the call: a float whose second byte is not 0.
        e.mem.set_u32(buffer, 0x3f80_0200);
        Record {
            buffer,
            cell,
            links,
            bodies,
        }
    }

    fn run_0ad0(e: &mut Engine, r: &Record, count: i32) {
        e.call(
            0x00dd_0ad0,
            &args![
                count,
                f32::from_bits(0x7f80_0001),
                3.5f32,
                0.5f32,
                r.links,
                r.bodies,
                0x1111u32,
                0x2222u32,
                r.cell
            ],
        );
    }

    #[test]
    fn the_record_gets_its_header_the_links_and_the_blocks_00dd0ad0() {
        let (mut e, calls) = scene(0.5);
        let r = record(&mut e, 2);
        run_0ad0(&mut e, &r, 2);
        // 1.0 + 2^-23 + 2^-9 with the bytes 0 and 2/3 replaced by 0x1E and the count.
        assert_eq!(e.mem.u32(r.buffer), 0x0002_021e);
        assert_eq!(e.mem.u32(r.buffer + 4), 0x290);
        assert_eq!(e.mem.u32(r.buffer + 8), 0x7fc0_0001);
        assert_eq!(e.mem.f32(r.buffer + 0xc), 3.5);
        // The links are copied behind the blocks; the pointer moves on by the size.
        for (word, offset) in [0u32, 0x40, 0x80].into_iter().enumerate() {
            assert_eq!(
                e.mem.u32(r.buffer + 0x20 + 2 * 0x130 + 4 * word as u32),
                offset
            );
        }
        assert_eq!(e.mem.u32(r.cell), r.buffer + 0x290);
        let layout = Layout {
            rows: r.buffer + 0x10 + 2 * 0x90,
            matrices: r.buffer + 0x10,
            links: r.buffer + 0x20 + 2 * 0x130,
            bodies: r.bodies,
        };
        check_blocks(&e, &calls, layout, 2, 0.5, 0.5);
    }

    #[test]
    fn no_blocks_write_the_header_and_one_link_word_only_00dd0ad0() {
        let (mut e, calls) = scene(0.5);
        let r = record(&mut e, 0);
        e.mem.set_u32(r.links, 0x77);
        run_0ad0(&mut e, &r, 0);
        assert_eq!(e.mem.u32(r.buffer), 0x0000_021e);
        assert_eq!(e.mem.u32(r.buffer + 4), 0x30);
        assert_eq!(e.mem.u32(r.buffer + 0x20), 0x77);
        assert_eq!(e.mem.u32(r.cell), r.buffer + 0x30);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn a_negative_count_copies_no_link_and_sizes_by_its_low_16_bits_00dd0ad0() {
        let (mut e, calls) = scene(0.5);
        let r = record(&mut e, 0);
        e.mem.set_u32(r.links, 0x77);
        run_0ad0(&mut e, &r, -1);
        let size = (0xffff * 0x134 + 0x33) & !0xf;
        assert_eq!(e.mem.u16(r.buffer + 2), 0xffff);
        assert_eq!(e.mem.u32(r.buffer + 4), size);
        assert_eq!(e.mem.u32(r.cell), r.buffer + size);
        assert_eq!(e.mem.u32(r.buffer + 0x20), 0);
        assert!(calls.borrow().is_empty());
    }
}
