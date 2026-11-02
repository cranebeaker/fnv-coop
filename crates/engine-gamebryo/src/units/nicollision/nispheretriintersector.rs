//! `NiCollision/nispheretriintersector.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiSphereTriIntersector` (Xbox PDB, 0x6c bytes): the `NiIntersector` base, then `m_kSphere`
//! (+0x38, the sphere volume: center +4, radius +0x10), `m_kTri` (+0x3c, an `NiTrigon`),
//! `m_fInvRadiusSqr` (+0x60) and the two triangle parameters `m_fParameter0` (+0x64) and
//! `m_fParameter1` (+0x68).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSqrDistance::Compute_ov4` (`00eb5450`, cdecl: point, triangle, two out parameters), in ST0.
const SQR_DISTANCE_POINT_TRIANGLE: u32 = 0x00eb_5450;
/// The double `1.0` at `01012070`.
const ONE_01012070: u32 = 0x0101_2070;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

// Translated from 00eb5bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphereTriIntersector::Pseudodistance` (Xbox PDB): the sphere's center moved by
/// `time * sphere_velocity` and the triangle (a copy) by `time * triangle_velocity` (products and
/// sums stored as floats), then the squared point-triangle distance (`NiSqrDistance::Compute_ov4`,
/// writing both parameters) times `m_fInvRadiusSqr`, minus the double 1.0 (`01012070`), as a
/// float in ST0.
pub fn ni_sphere_tri_intersector_pseudodistance(
    e: &mut Engine,
    this: Ptr,
    sphere_velocity: Ptr,
    triangle_velocity: Ptr,
    time: f32,
) -> f64 {
    let base = this.addr();
    let t = x87_load32(time.to_bits());
    e.with_stack(0x30, |e, locals| {
        let point = locals.addr();
        let triangle = point + 0xc;
        let v = sphere_velocity.addr();
        let shift = [
            x87_store32(x87_mul(load(e, v), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(4)), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(8)), t)),
        ];
        let center = e.mem.u32(base + 0x38).wrapping_add(4);
        let y = x87_store32(x87_add(load(e, center.wrapping_add(4)), x87_m32(shift[1])));
        let z = x87_store32(x87_add(load(e, center.wrapping_add(8)), x87_m32(shift[2])));
        let x = x87_store32(x87_add(load(e, center), x87_m32(shift[0])));
        for offset in (0..0x24u32).step_by(4) {
            let word = e.mem.u32(base + 0x3c + offset);
            e.mem.set_u32(triangle + offset, word);
        }
        e.mem.set_u32(point, x);
        e.mem.set_u32(point + 4, x87_quiet_bits(y));
        e.mem.set_u32(point + 8, x87_quiet_bits(z));
        let w = triangle_velocity.addr();
        let shift = [
            x87_store32(x87_mul(load(e, w), t)),
            x87_store32(x87_mul(load(e, w.wrapping_add(4)), t)),
            x87_store32(x87_mul(t, operand(e, w.wrapping_add(8)))),
        ];
        let corner = [
            x87_store32(x87_add(x87_load32(shift[0]), operand(e, triangle))),
            x87_store32(x87_add(load(e, triangle + 4), x87_m32(shift[1]))),
            x87_store32(x87_add(load(e, triangle + 8), x87_m32(shift[2]))),
        ];
        for (axis, word) in corner.iter().enumerate() {
            e.mem.set_u32(triangle + 4 * axis as u32, *word);
        }
        let squared = e
            .call(
                SQR_DISTANCE_POINT_TRIANGLE,
                &args![point, triangle, base + 0x64, base + 0x68],
            )
            .f64();
        let scaled = x87_mul(squared, operand(e, base + 0x60));
        let one = e.mem.f64(ONE_01012070);
        x87_load32(x87_store32(x87_sub(scaled, one)))
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00eb_5bf0,
        ni_sphere_tri_intersector_pseudodistance(Ptr, Ptr, Ptr, f32) -> f64
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(e: &Engine, at: u32) -> Vec<f32> {
        (0..3).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn pseudodistance_moves_the_center_and_the_triangle() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ONE_01012070, 1.0);
        let sphere = e.mem.alloc(0x14);
        e.mem.set_f32(sphere + 0xc, 4.0);
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + 0x38, sphere);
        e.mem.set_f32(this + 0x48, 2.0);
        e.mem.set_f32(this + 0x60, 0.5);
        let down = e.mem.alloc(12);
        e.mem.set_f32(down + 8, -1.0);
        let up = e.mem.alloc(12);
        e.mem.set_f32(up + 8, 1.0);
        e.register(SQR_DISTANCE_POINT_TRIANGLE, |e, a| {
            assert_eq!(read(e, a[0]), vec![0.0, 0.0, 3.0]);
            assert_eq!(read(e, a[1]), vec![0.0, 0.0, 1.0]);
            assert_eq!(read(e, a[1] + 0xc), vec![2.0, 0.0, 0.0]);
            e.mem.set_f32(a[2], 0.25);
            Ret {
                st0: 4.0,
                ..Ret::default()
            }
        });
        let r = e.call(0x00eb_5bf0, &args![this, down, up, 1.0f32]).f64();
        assert_eq!(r, 1.0);
        assert_eq!(e.mem.f32(this + 0x64), 0.25);
    }
}
