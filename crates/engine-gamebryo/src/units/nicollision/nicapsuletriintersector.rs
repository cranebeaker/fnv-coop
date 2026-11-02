//! `NiCollision/nicapsuletriintersector.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiCapsuleTriIntersector` (Xbox PDB, 0x6c bytes): the `NiIntersector` base, then `m_kCapsule`
//! (+0x38, the capsule volume: segment +0x20, radius +0x38), `m_kTri` (+0x3c, an `NiTrigon`: the
//! origin, then the edges at +0x48 and +0x54), `m_fInvRadiusSqr` (+0x60) and the two triangle
//! parameters of the closest point `m_fParameter0` (+0x64) and `m_fParameter1` (+0x68).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSqrDistance::Compute_ov3` (`00eb1d20`, cdecl: segment, triangle, the out segment parameter
/// and the two out triangle parameters), in ST0.
const SQR_DISTANCE_SEGMENT_TRIANGLE: u32 = 0x00eb_1d20;
/// `NiTrigon::GetNormal` (`00eb3830`, thiscall on the triangle: the out point; answers it).
const NI_TRIGON_GET_NORMAL: u32 = 0x00eb_3830;
/// The double `1.0` at `01012070`.
const ONE_01012070: u32 = 0x0101_2070;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

// Translated from 00eb3640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleTriIntersector::Pseudodistance` (Xbox PDB): the capsule's segment moved by
/// `time * capsule_velocity` and the triangle (a copy) by `time * triangle_velocity` (products and
/// sums stored as floats), then the squared segment-triangle distance
/// (`NiSqrDistance::Compute_ov3`, writing both triangle parameters; the segment parameter goes to a
/// stack slot) times `m_fInvRadiusSqr`, minus the double 1.0 (`01012070`), as a float in ST0.
pub fn ni_capsule_tri_intersector_pseudodistance(
    e: &mut Engine,
    this: Ptr,
    capsule_velocity: Ptr,
    triangle_velocity: Ptr,
    time: f32,
) -> f64 {
    let base = this.addr();
    let t = x87_load32(time.to_bits());
    e.with_stack(0x40, |e, locals| {
        let segment = locals.addr();
        let triangle = segment + 0x18;
        let segment_parameter = triangle + 0x24;
        let capsule_segment = e.mem.u32(base + 0x38).wrapping_add(0x20);
        for offset in (0..0x18u32).step_by(4) {
            let word = e.mem.u32(capsule_segment.wrapping_add(offset));
            e.mem.set_u32(segment + offset, word);
        }
        for offset in (0..0x24u32).step_by(4) {
            let word = e.mem.u32(base + 0x3c + offset);
            e.mem.set_u32(triangle + offset, word);
        }
        let v = capsule_velocity.addr();
        let shift = [
            x87_store32(x87_mul(load(e, v), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(4)), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(8)), t)),
        ];
        let origin = [
            x87_store32(x87_add(x87_load32(shift[0]), operand(e, segment))),
            x87_store32(x87_add(load(e, segment + 4), x87_m32(shift[1]))),
            x87_store32(x87_add(load(e, segment + 8), x87_m32(shift[2]))),
        ];
        for (axis, word) in origin.iter().enumerate() {
            e.mem.set_u32(segment + 4 * axis as u32, *word);
        }
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
                SQR_DISTANCE_SEGMENT_TRIANGLE,
                &args![
                    segment,
                    triangle,
                    segment_parameter,
                    base + 0x64,
                    base + 0x68
                ],
            )
            .f64();
        let scaled = x87_mul(squared, operand(e, base + 0x60));
        let one = e.mem.f64(ONE_01012070);
        x87_load32(x87_store32(x87_sub(scaled, one)))
    })
}

// Translated from 00eb5ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleTriIntersector::ComputeContactInformation` (Xbox PDB): the contact point is the
/// triangle point `origin + p0 * edge0 + p1 * edge1` (each product and sum stored as a float), the
/// normal the negated triangle normal (`NiTrigon::GetNormal`), and for a touching contact (type 2)
/// the point is moved by the contact time times `triangle_velocity`.
pub fn ni_capsule_tri_intersector_compute_contact_information(
    e: &mut Engine,
    this: Ptr,
    _capsule_velocity: Ptr,
    triangle_velocity: Ptr,
) {
    let base = this.addr();
    let along = |e: &Engine, edge: u32, parameter: u32| {
        let p = x87_load32(x87_quiet_bits(e.mem.u32(base + parameter)));
        [
            x87_store32(x87_mul(load(e, base + edge), p)),
            x87_store32(x87_mul(load(e, base + edge + 4), p)),
            x87_store32(x87_mul(p, operand(e, base + edge + 8))),
        ]
    };
    let second = along(e, 0x54, 0x68);
    let first = along(e, 0x48, 0x64);
    let corner = [0u32, 1, 2].map(|axis| {
        x87_store32(x87_add(
            load(e, base + 0x3c + 4 * axis),
            x87_m32(first[axis as usize]),
        ))
    });
    let point = [0usize, 1, 2]
        .map(|axis| x87_store32(x87_add(x87_load32(corner[axis]), x87_m32(second[axis]))));
    for (axis, word) in point.iter().enumerate() {
        e.mem.set_u32(base + 0x20 + 4 * axis as u32, *word);
    }
    e.with_stack(12, |e, out| {
        let normal = e.call(NI_TRIGON_GET_NORMAL, &args![base + 0x3c, out]).u32();
        let negated = [0u32, 4, 8]
            .map(|offset| x87_store32(-x87_load32(e.mem.u32(normal.wrapping_add(offset)))));
        for (axis, word) in negated.iter().enumerate() {
            e.mem.set_u32(base + 0x2c + 4 * axis as u32, *word);
        }
    });
    if e.mem.u32(base + 0x18) != 2 {
        return;
    }
    let contact = load(e, base + 0x1c);
    let w = triangle_velocity.addr();
    let shift = [
        x87_store32(x87_mul(load(e, w), contact)),
        x87_store32(x87_mul(load(e, w.wrapping_add(4)), contact)),
        x87_store32(x87_mul(contact, operand(e, w.wrapping_add(8)))),
    ];
    for component in 0..2u32 {
        let slot = base + 0x20 + 4 * component;
        let sum = x87_store32(x87_add(load(e, slot), x87_m32(shift[component as usize])));
        e.mem.set_u32(slot, sum);
    }
    let sum = x87_store32(x87_add(x87_load32(shift[2]), operand(e, base + 0x28)));
    e.mem.set_u32(base + 0x28, sum);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eb_3640,
            ni_capsule_tri_intersector_pseudodistance(Ptr, Ptr, Ptr, f32) -> f64
        ),
        entry!(
            0x00eb_5ce0,
            ni_capsule_tri_intersector_compute_contact_information(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    fn read(e: &Engine, at: u32) -> Vec<f32> {
        (0..3).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// An intersector over a capsule (segment from (0, 0, 3) along +x by 2) and the triangle with
    /// the corner at the origin and the edges (2, 0, 0) and (0, 2, 0).
    fn intersector(e: &mut Engine) -> u32 {
        let mut capsule = vec![0.0f32; 15];
        capsule[10] = 3.0;
        capsule[11] = 2.0;
        capsule[14] = 1.0;
        let capsule = floats(e, &capsule);
        let this = floats(e, &[0.0; 27]);
        e.mem.set_u32(this + 0x38, capsule);
        for (i, v) in [0.0f32, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0, 0.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(this + 0x3c + 4 * i as u32, *v);
        }
        e.mem.set_f32(this + 0x60, 0.25);
        this
    }

    #[test]
    fn pseudodistance_moves_the_segment_and_the_triangle() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ONE_01012070, 1.0);
        let this = intersector(&mut e);
        let down = floats(&mut e, &[0.0, 0.0, -1.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        e.register(SQR_DISTANCE_SEGMENT_TRIANGLE, |e, a| {
            assert_eq!(read(e, a[0]), vec![0.0, 0.0, 2.0]);
            assert_eq!(read(e, a[1]), vec![0.0, 0.0, 1.0]);
            assert_eq!(read(e, a[1] + 0xc), vec![2.0, 0.0, 0.0]);
            e.mem.set_f32(a[3], 0.25);
            e.mem.set_f32(a[4], 0.5);
            Ret {
                st0: 1.0,
                ..Ret::default()
            }
        });
        let r = e.call(0x00eb_3640, &args![this, down, up, 1.0f32]).f64();
        assert_eq!(r, -0.75);
        assert_eq!(e.mem.f32(this + 0x64), 0.25);
        assert_eq!(e.mem.f32(this + 0x68), 0.5);
    }

    #[test]
    fn contact_point_on_the_triangle() {
        let mut e = Engine::new();
        e.register(NI_TRIGON_GET_NORMAL, |e, a| {
            e.mem.set_f32(a[1] + 8, 1.0);
            a[1].into_ret()
        });
        let this = intersector(&mut e);
        e.mem.set_f32(this + 0x64, 0.25);
        e.mem.set_f32(this + 0x68, 0.5);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        e.mem.set_u32(this + 0x18, 3);
        e.call(0x00eb_5ce0, &args![this, 0u32, up]);
        assert_eq!(read(&e, this + 0x20), vec![0.5, 1.0, 0.0]);
        assert_eq!(read(&e, this + 0x2c), vec![-0.0, -0.0, -1.0]);
        // Touching: moved with the triangle by the contact time.
        e.mem.set_u32(this + 0x18, 2);
        e.mem.set_f32(this + 0x1c, 2.0);
        e.call(0x00eb_5ce0, &args![this, 0u32, up]);
        assert_eq!(read(&e, this + 0x20), vec![0.5, 1.0, 2.0]);
    }
}
