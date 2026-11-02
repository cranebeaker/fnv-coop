//! `NiMain/nipoint3_xenon.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `011f4aa8`: the pointer to the square-root mantissa table (256 words, indexed by the top
/// eight mantissa bits with the exponent's low bit folded in) that the fast square root
/// below ORs under its halved exponent.
const SQRT_TABLE_POINTER: u32 = 0x011f_4aa8;

/// One point of [`ni_point3_unitize_vectors`]: the squared length `y*y + x*x + z*z` (x87, in
/// that operand order) stored as a `float`; a scale of 0.0 when its bits are 0 (only +0.0:
/// -0.0 takes the other branch), else `1 / r` stored as a `float`, where `r` is the table's
/// fast square root of the squared length (exponent `(e - 127) / 2 + 127` computed in 16-bit
/// arithmetic as the exe does, mantissa from the table entry of the top mantissa bits, with
/// bit 23 set for an odd exponent); then each component multiplied by the scale.
fn unitize(e: &mut Engine, point: u32) {
    let y = x87_load32(e.mem.u32(point.wrapping_add(4)));
    let x = x87_load32(e.mem.u32(point));
    let z = x87_load32(e.mem.u32(point.wrapping_add(8)));
    let xx = x87_mul(x, x);
    let yy = x87_mul(y, y);
    let sum = x87_add(yy, xx);
    let zz = x87_mul(z, z);
    let length_squared = x87_store32(x87_add(sum, zz));
    let scale = if length_squared == 0 {
        x87_store32(0.0)
    } else {
        let exponent = ((length_squared >> 23).wrapping_sub(0x7f)) as u16;
        let mut mantissa = length_squared & 0x7f_ffff;
        if exponent & 1 != 0 {
            mantissa |= 0x80_0000;
        }
        let table = e.mem.u32(SQRT_TABLE_POINTER);
        let halved = ((exponent as i16) >> 1) as i32 as u32;
        let root_bits = (halved.wrapping_add(0x7f) << 23)
            | e.mem
                .u32(table.wrapping_add((mantissa >> 16).wrapping_mul(4)));
        let root = x87_load32(root_bits);
        x87_store32(x87_div(1.0, root))
    };
    let x = x87_load32(e.mem.u32(point));
    let s = x87_load32(scale);
    e.mem.set_u32(point, x87_store32(x87_mul(x, s)));
    let y = x87_load32(e.mem.u32(point.wrapping_add(4)));
    e.mem
        .set_u32(point.wrapping_add(4), x87_store32(x87_mul(y, s)));
    let z = x87_m32(e.mem.u32(point.wrapping_add(8)));
    e.mem
        .set_u32(point.wrapping_add(8), x87_store32(x87_mul(s, z)));
}

// Translated from 00a7e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3::UnitizeVectors` (Xbox PDB, `cdecl`): normalizes `count` points (three floats
/// each) starting at `points`, `stride` bytes apart, with the table square root (each point
/// as [`unitize`]; a zero-length point becomes zero). The exe unrolls four points per pass
/// while `count >= 4` (signed), then does the rest one by one (`count - 4 * passes`,
/// compared unsigned, so a negative count runs on as a huge unsigned one); every point gets
/// the same arithmetic.
pub fn ni_point3_unitize_vectors(e: &mut Engine, points: Ptr, count: i32, stride: u32) {
    let mut point = points.addr();
    let mut done = 0u32;
    if count >= 4 {
        let passes = ((count as u32).wrapping_sub(4) >> 2).wrapping_add(1);
        done = passes.wrapping_mul(4);
        for _ in 0..passes {
            for _ in 0..4 {
                unitize(e, point);
                point = point.wrapping_add(stride);
            }
        }
    }
    let count = count as u32;
    if done < count {
        for _ in 0..count - done {
            unitize(e, point);
            point = point.wrapping_add(stride);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a7_e960,
        ni_point3_unitize_vectors(Ptr, i32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine whose table pointer holds a table of 256 entries that turn the top
    /// mantissa bits back into a mantissa (`index << 15`), so the root is `sqrt` within the
    /// table's resolution; the tests check against the same arithmetic.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        let table = e.mem.alloc(0x400);
        for i in 0..0x100u32 {
            e.mem.set_u32(table + 4 * i, (i << 15) & 0x7f_ffff);
        }
        e.mem.set_u32(SQRT_TABLE_POINTER, table);
        (e, table)
    }

    fn expected_scale(e: &Engine, table: u32, length_squared: f32) -> f32 {
        let bits = length_squared.to_bits();
        let exponent = ((bits >> 23).wrapping_sub(0x7f)) as u16;
        let mut mantissa = bits & 0x7f_ffff;
        if exponent & 1 != 0 {
            mantissa |= 0x80_0000;
        }
        let halved = ((exponent as i16) >> 1) as i32 as u32;
        let root = (halved.wrapping_add(0x7f) << 23) | e.mem.u32(table + 4 * (mantissa >> 16));
        (1.0f64 / f32::from_bits(root) as f64) as f32
    }

    #[test]
    fn unitizes_every_point_unrolled_and_in_the_tail() {
        let (mut e, table) = engine();
        // Six points (one unrolled pass of four, a tail of two), 16 bytes apart.
        let points = e.mem.alloc(16 * 6);
        let values = [
            [3.0f32, 4.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 2.0, 2.0],
            [0.0, 0.0, 5.0],
            [2.0, 0.0, 0.0],
            [-1.0, 1.0, 1.0],
        ];
        for (i, v) in values.iter().enumerate() {
            for (c, value) in v.iter().enumerate() {
                e.mem.set_f32(points + 16 * i as u32 + 4 * c as u32, *value);
            }
        }
        e.mem.set_f32(points + 12, 99.0);
        e.call(0x00a7_e960, &args![points, 6i32, 16u32]);
        for (i, v) in values.iter().enumerate() {
            let length_squared = (v[1] * v[1] + v[0] * v[0]) + v[2] * v[2];
            let scale = if length_squared.to_bits() == 0 {
                0.0
            } else {
                expected_scale(&e, table, length_squared)
            };
            let base = points + 16 * i as u32;
            assert_eq!(e.mem.f32(base), v[0] * scale, "point {i} x");
            assert_eq!(e.mem.f32(base + 4), v[1] * scale, "point {i} y");
            assert_eq!(e.mem.f32(base + 8), scale * v[2], "point {i} z");
        }
        // The word after each point is not touched.
        assert_eq!(e.mem.f32(points + 12), 99.0);
        // A length of 4 has an even exponent and an exact table root of 2.
        assert_eq!(e.mem.f32(points + 16 * 4), 1.0);
    }

    #[test]
    fn fewer_than_four_points_and_zero_points() {
        let (mut e, _) = engine();
        let points = e.mem.alloc(12 * 2);
        e.mem.set_f32(points, 4.0);
        e.mem.set_f32(points + 12 + 8, 2.0);
        e.call(0x00a7_e960, &args![points, 2i32, 12u32]);
        assert_eq!(e.mem.f32(points), 1.0);
        assert_eq!(e.mem.f32(points + 12 + 8), 1.0);
        // No point at all: nothing is read.
        e.call(0x00a7_e960, &args![0u32, 0i32, 12u32]);
    }
}
