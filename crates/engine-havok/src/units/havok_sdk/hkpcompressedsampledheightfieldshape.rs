//! `Havok SDK/hkpcompressedsampledheightfieldshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, sum3, zip_with, Vec4};
use crate::x87::rsqrt_estimate;

// Translated from 00e49510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCompressedSampledHeightFieldShape::calcSizeForSpu` (Xbox PDB): a virtual method
/// (`RET 8`: one word it never reads, then the buffer size) that returns `0x80` when the
/// buffer size, as a signed number, is at least `0x80`, and `-1` otherwise. `this` (`ECX`)
/// is not read.
pub fn hkp_compressed_sampled_height_field_shape_calc_size_for_spu(
    e: &mut Engine,
    _unused_this: Ptr,
    _unused_1: u32,
    buffer_size: i32,
) -> u32 {
    let _ = e;
    if buffer_size >= 0x80 {
        0x80
    } else {
        u32::MAX
    }
}

/// Floats, vectors and doubles of the exe the function reads: 0.0, 1.0, the large float, 3.0
/// and 0.5 (the `RSQRTSS` refinement), the vector added to make the grid cell numbers appear
/// in the low bits of a float, and the double 1.0.
const ZERO: u32 = 0x0101_1d78;
const ONE: u32 = 0x0101_2068;
const LARGE: u32 = 0x0104_da48;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const CELL_BIAS: u32 = 0x011b_1bd0;
const ONE_DOUBLE: u32 = 0x0101_2070;
/// Virtual slots of the shape: `0x34` is the height of the sample `(x, z)` (two words, answers
/// a float in ST0) and `0x38` tells which way the diagonal of the cell runs (one word, a byte
/// to write).
const SLOT_HEIGHT: u32 = 0x34;
const SLOT_DIAGONAL: u32 = 0x38;

/// The height of the sample `(x, z)`, stored as a float.
fn sample(e: &mut Engine, this: u32, x: u32, z: u32) -> f32 {
    e.vcall(this, SLOT_HEIGHT, &args![x, z]).f32()
}

/// The slopes along x and z and the height at the position inside a cell.
struct Plane {
    x_slope: f32,
    z_slope: f32,
    height: f32,
    /// The sample the plane runs through: its x, and its height.
    anchor: (u32, f32),
}

/// The heights at the corners of the cell `(x, z)` and the plane over the position `(fx, fz)`
/// in it, as the code forms them: with the diagonal the shape names (`diagonal`), the corner
/// `(x, z)` is the anchor and the cell splits at `fz < fx`; without it the cell splits at
/// `fx + fz > 1` and the lower triangle's anchor is `(x, z)` too, the upper one's is
/// `(x + 1, z)`. The sums are x87 (`f64` here) and the slopes are rounded to floats.
fn cell_plane(
    e: &mut Engine,
    this: u32,
    diagonal: bool,
    (x, z): (u32, u32),
    frac: (f32, f32),
) -> Plane {
    let (fx, fz) = frac;
    let (fx64, fz64) = (f64::from(fx), f64::from(fz));
    if diagonal {
        let h00 = sample(e, this, x, z);
        let h11 = sample(e, this, x + 1, z + 1);
        let (x_slope, z_slope) = if fz < fx {
            let h10 = sample(e, this, x + 1, z);
            (
                (f64::from(h10) - f64::from(h00)) as f32,
                (f64::from(h11) - f64::from(h10)) as f32,
            )
        } else {
            let h01 = sample(e, this, x, z + 1);
            (
                (f64::from(h11) - f64::from(h01)) as f32,
                (f64::from(h01) - f64::from(h00)) as f32,
            )
        };
        let across = f64::from(h00) + f64::from(z_slope) * fz64;
        let height = (across + f64::from(x_slope) * fx64) as f32;
        return Plane {
            x_slope,
            z_slope,
            height,
            anchor: (x, h00),
        };
    }
    let h10 = sample(e, this, x + 1, z);
    let h01 = sample(e, this, x, z + 1);
    let one = e.mem.f64(ONE_DOUBLE);
    if fz64 + fx64 > one {
        let h11 = sample(e, this, x + 1, z + 1);
        let x_slope = (f64::from(h11) - f64::from(h01)) as f32;
        let z_slope = (f64::from(h11) - f64::from(h10)) as f32;
        let across = f64::from(h10) + f64::from(z_slope) * fz64;
        let height = (across + (fx64 - one) * f64::from(x_slope)) as f32;
        Plane {
            x_slope,
            z_slope,
            height,
            anchor: (x + 1, h10),
        }
    } else {
        let h00 = sample(e, this, x, z);
        let x_slope = (f64::from(h10) - f64::from(h00)) as f32;
        let z_slope = (f64::from(h01) - f64::from(h00)) as f32;
        let across = f64::from(h00) + f64::from(z_slope) * fz64;
        let height = (across + f64::from(x_slope) * fx64) as f32;
        Plane {
            x_slope,
            z_slope,
            height,
            anchor: (x + 1, h10),
        }
    }
}

// Translated from 00e49530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the shape (the map gives it no name; `RET 8`, `this` in `ECX`;
/// `RSQRTSS` is the host CPU's estimate): for every point of a list it finds the grid cell
/// the point is over and writes a surface normal and a signed distance. `positions` points
/// at a pointer to the points (16-byte vectors whose fourth lane is a radius) and their
/// count; `output` takes one vector per point, which starts as `(0, 1, 0, large float)` and
/// stays so when the point is outside the grid.
///
/// The point times the shape's scale vector (`this + 0x30`) is `q`; the cell numbers `x`, `z`
/// are the low 16 bits of lanes 0 and 2 of `(point + this[0x40]) * scale + 196608.0`
/// shifted right by 6, and the point is skipped unless `x < this[0x10] - 1` and
/// `z < this[0x14] - 1` (signed). The fractions are `fx = q.x - x` and `fz = q.z - z` (x87,
/// rounded to floats). The shape's slot `0x38` says which way the cell's diagonal runs and
/// slot `0x34` gives sample heights; the plane over the cell (see `cell_plane`) gives the
/// slopes `(P, Q)` and the height `H`. The output holds `(-P, 1, -Q)` times the scale
/// vector, normalised (the `RSQRTSS` estimate refined by one step, zero stays zero), with the
/// distance in lane 3: when the byte `this + 0x1C` is 0, `((q.y - H) * this[0x24]) - radius`;
/// else the sum of the first three lanes of `((q - (x, height of the anchor, z, 0)) *
/// this[0x20 vector]) * normal`, minus the radius.
pub fn fn_00e49530(e: &mut Engine, this: Ptr, positions: Ptr, output: Ptr) {
    let this = this.addr();
    let mut cursor = e.mem.u32(positions.addr());
    let count = e.mem.u32(positions.addr() + 4) as i32;
    let mut out = output.addr();
    let default: Vec4 = [
        e.mem.f32(ZERO),
        e.mem.f32(ONE),
        e.mem.f32(ZERO),
        e.mem.f32(LARGE),
    ];
    e.with_stack(0x10, |e, diagonal_byte| {
        for _ in 0..count.max(0) {
            store_vec(e, out, default);
            let scale = load_vec(e, this + 0x30);
            let point = load_vec(e, cursor);
            let scaled = zip_with(point, scale, |p, s| p * s);
            let moved = zip_with(
                zip_with(point, load_vec(e, this + 0x40), |p, o| p + o),
                scale,
                |m, s| m * s,
            );
            let biased = zip_with(moved, load_vec(e, CELL_BIAS), |m, k| m + k);
            let cell_x = (biased[0].to_bits() >> 6) & 0xffff;
            let cell_z = (biased[2].to_bits() >> 6) & 0xffff;
            let in_x = (cell_x as i32) < (e.mem.u32(this + 0x10) as i32).wrapping_sub(1);
            if in_x && (cell_z as i32) < (e.mem.u32(this + 0x14) as i32).wrapping_sub(1) {
                let frac_x = (f64::from(x87_quiet(scaled[0])) - f64::from(cell_x as i32)) as f32;
                let frac_z = (f64::from(x87_quiet(scaled[2])) - f64::from(cell_z as i32)) as f32;
                e.vcall(this, SLOT_DIAGONAL, &args![diagonal_byte]);
                let diagonal = e.mem.u8(diagonal_byte.addr()) != 0;
                let plane = cell_plane(e, this, diagonal, (cell_x, cell_z), (frac_x, frac_z));
                e.mem.set_f32(out, -x87_quiet(plane.x_slope));
                e.mem.set_f32(out + 8, -x87_quiet(plane.z_slope));
                let raw = zip_with(load_vec(e, this + 0x30), load_vec(e, out), |s, v| s * v);
                store_vec(e, out, raw);
                let squares = zip_with(raw, raw, |a, b| a * b);
                let sum = sum3(squares);
                let estimate = rsqrt_estimate(sum);
                let factor = if sum == e.mem.f32(ZERO) {
                    0.0
                } else {
                    (e.mem.f32(HALF) * estimate) * (e.mem.f32(THREE) - (sum * estimate) * estimate)
                };
                let unit = raw.map(|lane| factor * lane);
                store_vec(e, out, unit);
                let radius = f64::from(x87_quiet(e.mem.f32(cursor + 0xc)));
                let distance = if e.mem.u8(this + 0x1c) == 0 {
                    let height = f64::from(x87_quiet(plane.height));
                    let slope = f64::from(x87_quiet(e.mem.f32(this + 0x24)));
                    ((f64::from(x87_quiet(scaled[1])) - height) * slope) - radius
                } else {
                    let (anchor_x, anchor_height) = plane.anchor;
                    let anchor: Vec4 = [
                        anchor_x as i32 as f32,
                        anchor_height,
                        cell_z as i32 as f32,
                        e.mem.f32(ZERO),
                    ];
                    let offset = zip_with(scaled, anchor, |q, a| q - a);
                    let weighted = zip_with(offset, load_vec(e, this + 0x20), |o, w| o * w);
                    let along = sum3(zip_with(weighted, unit, |w, n| w * n));
                    f64::from(x87_quiet(along)) - radius
                };
                e.mem.set_f32(out + 0xc, x87_quiet(distance as f32));
            }
            cursor += 0x10;
            out += 0x10;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e4_9510,
            hkp_compressed_sampled_height_field_shape_calc_size_for_spu(Ptr, u32, i32) -> u32
        ),
        entry!(0x00e4_9530, fn_00e49530(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn calc_size_for_spu_00e49510() {
        let mut e = Engine::new();
        let cases: [(i32, u32); 6] = [
            (0x7f, u32::MAX),
            (0x80, 0x80),
            (0x81, 0x80),
            (0, u32::MAX),
            (-5, u32::MAX),
            (i32::MAX, 0x80),
        ];
        for (size, expected) in cases {
            let result = e.call(0x00e4_9510, &args![0x1000u32, 0x1111u32, size]);
            assert_eq!(result.u32(), expected, "buffer size {size:#x}");
        }
    }

    const SHAPE_HEIGHT: u32 = 0x0900_0034;
    const SHAPE_DIAGONAL: u32 = 0x0900_0038;
    const LARGE_BITS: u32 = 0x7f7f_ffee;

    type Samples = Rc<RefCell<Vec<(u32, u32)>>>;

    /// The height of the grid sample `(x, z)` the shape's double answers.
    fn height_at(x: u32, z: u32) -> f32 {
        0.5 * x as f32 + 2.0 * z as f32 + 0.25 * (x * z) as f32
    }

    struct Field {
        e: Engine,
        this: u32,
        positions: u32,
        out: u32,
        samples: Samples,
    }

    /// A 10 by 10 grid shape (scale 1, offset 0, vector `+0x20` = (1, 2, 1, 0)) whose slot
    /// `0x38` answers `diagonal` and whose `+0x1C` byte is `flagged`, and a list of `points`.
    fn field(diagonal: u8, flagged: u8, points: &[[f32; 4]]) -> Field {
        let mut e = engine();
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(ONE, 1.0);
        e.mem.set_u64(ONE_DOUBLE, 1.0f64.to_bits());
        e.mem.set_u32(LARGE, LARGE_BITS);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        for lane in 0..4 {
            e.mem.set_u32(CELL_BIAS + 4 * lane, 0x4840_0000);
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + SLOT_HEIGHT, SHAPE_HEIGHT);
        e.mem.set_u32(vtable + SLOT_DIAGONAL, SHAPE_DIAGONAL);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x10, 10);
        e.mem.set_u32(this + 0x14, 10);
        e.mem.set_u8(this + 0x1c, flagged);
        store_vec(&mut e, this + 0x20, [1.0, 2.0, 1.0, 0.0]);
        store_vec(&mut e, this + 0x30, [1.0; 4]);
        store_vec(&mut e, this + 0x40, [0.0; 4]);
        let samples = Samples::default();
        let seen = Rc::clone(&samples);
        e.register_double(SHAPE_HEIGHT, move |_, a| {
            seen.borrow_mut().push((a[1], a[2]));
            Ret {
                st0: f64::from(height_at(a[1], a[2])),
                ..Ret::default()
            }
        });
        e.register_double(SHAPE_DIAGONAL, move |e, a| {
            e.mem.set_u8(a[1], diagonal);
            Ret::default()
        });
        let list = e.mem.alloc(0x10 * points.len() as u32 + 0x10);
        let out = e.mem.alloc(0x10 * points.len() as u32 + 0x10);
        for (n, point) in points.iter().enumerate() {
            store_vec(&mut e, list + 0x10 * n as u32, *point);
            store_vec(&mut e, out + 0x10 * n as u32, [7.0; 4]);
        }
        let positions = e.mem.alloc(8);
        e.mem.set_u32(positions, list);
        e.mem.set_u32(positions + 4, points.len() as u32);
        Field {
            e,
            this,
            positions,
            out,
            samples,
        }
    }

    /// What the point should produce, worked out from the plane through the three samples
    /// of its triangle (independent of the code's slopes): the unit normal of
    /// `(-P, 1, -Q)`, and the distance for the byte `flagged`.
    fn expected(point: [f32; 4], diagonal: bool, flagged: bool) -> [f64; 4] {
        let (x, z) = (point[0] as u32, point[2] as u32);
        let (fx, fz) = (
            f64::from(point[0]) - f64::from(x),
            f64::from(point[2]) - f64::from(z),
        );
        let h = |dx: u32, dz: u32| f64::from(height_at(x + dx, z + dz));
        let (px, pz, anchor) = if diagonal {
            if fz < fx {
                (h(1, 0) - h(0, 0), h(1, 1) - h(1, 0), (0, h(0, 0)))
            } else {
                (h(1, 1) - h(0, 1), h(0, 1) - h(0, 0), (0, h(0, 0)))
            }
        } else if fx + fz > 1.0 {
            (h(1, 1) - h(0, 1), h(1, 1) - h(1, 0), (1, h(1, 0)))
        } else {
            (h(1, 0) - h(0, 0), h(0, 1) - h(0, 0), (1, h(1, 0)))
        };
        let length = (px * px + 1.0 + pz * pz).sqrt();
        let unit = [-px / length, 1.0 / length, -pz / length];
        let on_plane = anchor.1 + px * (f64::from(point[0]) - f64::from(x + anchor.0)) + pz * fz;
        let distance = if flagged {
            (f64::from(point[0]) - f64::from(x + anchor.0)) * unit[0]
                + (f64::from(point[1]) - anchor.1) * 2.0 * unit[1]
                + fz * unit[2]
                - f64::from(point[3])
        } else {
            (f64::from(point[1]) - on_plane) * 2.0 - f64::from(point[3])
        };
        [unit[0], unit[1], unit[2], distance]
    }

    fn out_of(f: &Field, n: u32) -> [f32; 4] {
        load_vec(&f.e, f.out + 0x10 * n)
    }

    #[test]
    fn collide_points_over_cells_00e49530() {
        let points = [
            [2.5, 3.0, 4.25, 0.5],
            [2.25, 3.0, 4.5, 0.5],
            [2.75, 3.0, 4.5, 0.5],
            [2.5, 3.0, 4.5, 0.5],
            [2.25, 3.0, 4.75, 0.25],
        ];
        // (diagonal byte, samples asked for the first four points)
        let orders: [(u8, [Vec<(u32, u32)>; 4]); 2] = [
            (
                1,
                [
                    vec![(2, 4), (3, 5), (3, 4)],
                    vec![(2, 4), (3, 5), (2, 5)],
                    vec![(2, 4), (3, 5), (3, 4)],
                    vec![(2, 4), (3, 5), (2, 5)],
                ],
            ),
            (
                0,
                [
                    vec![(3, 4), (2, 5), (2, 4)],
                    vec![(3, 4), (2, 5), (2, 4)],
                    vec![(3, 4), (2, 5), (3, 5)],
                    vec![(3, 4), (2, 5), (2, 4)],
                ],
            ),
        ];
        for (diagonal, asked) in orders {
            for flagged in [0u8, 1] {
                for (n, point) in points.iter().enumerate() {
                    let mut f = field(diagonal, flagged, &[*point]);
                    f.e.call(0x00e4_9530, &[f.this, f.positions, f.out]);
                    if n < 4 {
                        assert_eq!(
                            *f.samples.borrow(),
                            asked[n],
                            "diagonal {diagonal} point {point:?}"
                        );
                    }
                    let want = expected(*point, diagonal != 0, flagged != 0);
                    let got = out_of(&f, 0);
                    for lane in 0..4 {
                        assert!(
                            (f64::from(got[lane]) - want[lane]).abs() < 1e-4,
                            "diagonal {diagonal} flagged {flagged} point {point:?} lane {lane}: {got:?} vs {want:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn collide_points_outside_the_grid_keep_the_default_vector_00e49530() {
        let points = [
            [9.5, 3.0, 4.5, 0.5],
            [2.5, 3.0, 9.5, 0.5],
            [-1.0, 3.0, 4.5, 0.5],
            [2.5, 3.0, 4.5, 0.5],
        ];
        let mut f = field(1, 0, &points);
        f.e.call(0x00e4_9530, &[f.this, f.positions, f.out]);
        for n in 0..3 {
            let bits: Vec<u32> = out_of(&f, n).map(f32::to_bits).to_vec();
            assert_eq!(bits, vec![0, 1.0f32.to_bits(), 0, LARGE_BITS], "point {n}");
        }
        // The point inside is still done, after the skipped ones.
        assert_eq!(f.samples.borrow().len(), 3);
        assert!(out_of(&f, 3)[3].is_finite() && out_of(&f, 3)[3] != 7.0);
    }

    #[test]
    fn an_empty_list_writes_nothing_00e49530() {
        let mut f = field(1, 0, &[[2.5, 3.0, 4.5, 0.5]]);
        f.e.mem.set_u32(f.positions + 4, 0);
        f.e.call(0x00e4_9530, &[f.this, f.positions, f.out]);
        assert_eq!(out_of(&f, 0), [7.0; 4]);
        assert!(f.samples.borrow().is_empty());
        f.e.mem.set_u32(f.positions + 4, u32::MAX);
        f.e.call(0x00e4_9530, &[f.this, f.positions, f.out]);
        assert_eq!(out_of(&f, 0), [7.0; 4]);
    }
}
