//! `NiCollision/nicapsulesphereintersector.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiCapsuleSphereIntersector` (Xbox PDB, 0x48 bytes): the `NiIntersector` base, then
//! `m_kCapsule` (+0x38, the capsule volume), `m_kSphere` (+0x3c, the sphere volume),
//! `m_fInvRadiusSumSqr` (+0x40) and `m_fParameter` (+0x44, the segment parameter of the closest
//! point). In the volumes the capsule's segment is at +0x20 (origin, then direction) and its
//! radius at +0x38; the sphere's center at +4.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSqrDistance::Compute` (`00eaf1f0`, cdecl: point, segment, out parameter), in ST0.
const SQR_DISTANCE_POINT_SEGMENT: u32 = 0x00ea_f1f0;
/// `NiSegment::GetPoint` (`00eb6090`, thiscall on the segment: the out point, the parameter).
const NI_SEGMENT_GET_POINT: u32 = 0x00eb_6090;
/// `00439090` (thiscall on a point, the other point): whether they differ.
const POINTS_DIFFER: u32 = 0x0043_9090;
/// Unnamed in the engine map: normalizes the three floats at ECX in place.
const UNITIZE: u32 = 0x004a_0c10;
/// The double `1.0` at `01012070`.
const ONE_01012070: u32 = 0x0101_2070;
/// The double `0.5` at `01011588`.
const HALF_01011588: u32 = 0x0101_1588;
/// The three words at `011a9484` (the z axis in the image): the normal when the two centers meet.
const DEFAULT_NORMAL_011A9484: u32 = 0x011a_9484;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// `velocity * time` as three stored floats, in the exe's operand order (`v.x * t`, `v.y * t`,
/// `t * v.z`).
fn motion(e: &Engine, velocity: u32, time: f64) -> [u32; 3] {
    [
        x87_store32(x87_mul(load(e, velocity), time)),
        x87_store32(x87_mul(load(e, velocity.wrapping_add(4)), time)),
        x87_store32(x87_mul(time, operand(e, velocity.wrapping_add(8)))),
    ]
}

// Translated from 00eaea50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleSphereIntersector::Pseudodistance` (Xbox PDB): with the capsule's segment moved by
/// `time * capsule_velocity` and the sphere's center moved by `time * sphere_velocity` (products
/// and sums stored as floats), the squared distance of the center from the segment
/// (`NiSqrDistance::Compute`, writing `m_fParameter`) times `m_fInvRadiusSumSqr`, minus the double
/// 1.0 (`01012070`), as a float in ST0.
pub fn ni_capsule_sphere_intersector_pseudodistance(
    e: &mut Engine,
    this: Ptr,
    capsule_velocity: Ptr,
    sphere_velocity: Ptr,
    time: f32,
) -> f64 {
    let base = this.addr();
    let segment = e.mem.u32(base + 0x38).wrapping_add(0x20);
    let t = x87_load32(time.to_bits());
    e.with_stack(0x24, |e, locals| {
        let point = locals.addr();
        let moved = point + 0xc;
        for offset in (0..0x18u32).step_by(4) {
            let word = e.mem.u32(segment.wrapping_add(offset));
            e.mem.set_u32(moved + offset, word);
        }
        let v = capsule_velocity.addr();
        let shift = [
            x87_store32(x87_mul(load(e, v), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(4)), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(8)), t)),
        ];
        let origin = [
            x87_store32(x87_add(x87_load32(shift[0]), operand(e, moved))),
            x87_store32(x87_add(load(e, moved + 4), x87_m32(shift[1]))),
            x87_store32(x87_add(load(e, moved + 8), x87_m32(shift[2]))),
        ];
        for (axis, word) in origin.iter().enumerate() {
            e.mem.set_u32(moved + 4 * axis as u32, *word);
        }
        let shift = motion(e, sphere_velocity.addr(), t);
        let center = e.mem.u32(base + 0x3c).wrapping_add(4);
        let y = x87_store32(x87_add(load(e, center.wrapping_add(4)), x87_m32(shift[1])));
        let z = x87_store32(x87_add(load(e, center.wrapping_add(8)), x87_m32(shift[2])));
        let x = x87_store32(x87_add(load(e, center), x87_m32(shift[0])));
        e.mem.set_u32(point, x);
        e.mem.set_u32(point + 4, x87_quiet_bits(y));
        e.mem.set_u32(point + 8, x87_quiet_bits(z));
        let squared = e
            .call(
                SQR_DISTANCE_POINT_SEGMENT,
                &args![point, moved, base + 0x44],
            )
            .f64();
        let scaled = x87_mul(squared, operand(e, base + 0x40));
        let one = e.mem.f64(ONE_01012070);
        x87_load32(x87_store32(x87_sub(scaled, one)))
    })
}

// Translated from 00eaeb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleSphereIntersector::ComputeContactInformation` (Xbox PDB): with `q` the segment point
/// at `m_fParameter` (`NiSegment::GetPoint`) and `c` the sphere's center:
/// - a touching contact (type 2): both moved by the contact time times their velocities (stored
///   floats), the normal `m_kContactNormal` is `c - q` normalized (`004a0c10`) and the contact
///   point `q` plus the capsule's radius times the normal.
/// - otherwise the contact point is `(c + q) * 0.5` (the double at `01011588`) and the normal
///   `c - q` normalized, or the three words at `011a9484` when the points do not differ
///   (`00439090`).
pub fn ni_capsule_sphere_intersector_compute_contact_information(
    e: &mut Engine,
    this: Ptr,
    capsule_velocity: Ptr,
    sphere_velocity: Ptr,
) {
    let base = this.addr();
    let normal = base + 0x2c;
    let parameter = x87_quiet_bits(e.mem.u32(base + 0x44));
    e.with_stack(0x18, |e, locals| {
        let near = locals.addr();
        let center = near + 0xc;
        let segment = e.mem.u32(base + 0x38).wrapping_add(0x20);
        e.call(NI_SEGMENT_GET_POINT, &args![segment, near, parameter]);
        let sphere_center = e.mem.u32(base + 0x3c).wrapping_add(4);
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(sphere_center.wrapping_add(offset));
            e.mem.set_u32(center + offset, word);
        }
        if e.mem.u32(base + 0x18) == 2 {
            let contact = load(e, base + 0x1c);
            let shift = motion(e, capsule_velocity.addr(), contact);
            let moved = [
                x87_store32(x87_add(x87_load32(shift[0]), operand(e, near))),
                x87_store32(x87_add(load(e, near + 4), x87_m32(shift[1]))),
                x87_store32(x87_add(load(e, near + 8), x87_m32(shift[2]))),
            ];
            for (axis, word) in moved.iter().enumerate() {
                e.mem.set_u32(near + 4 * axis as u32, *word);
            }
            let contact = load(e, base + 0x1c);
            let shift = motion(e, sphere_velocity.addr(), contact);
            let moved = [
                x87_store32(x87_add(x87_load32(shift[0]), operand(e, center))),
                x87_store32(x87_add(load(e, center + 4), x87_m32(shift[1]))),
                x87_store32(x87_add(load(e, center + 8), x87_m32(shift[2]))),
            ];
            for (axis, word) in moved.iter().enumerate() {
                e.mem.set_u32(center + 4 * axis as u32, *word);
            }
            for offset in [0u32, 4, 8] {
                let word =
                    x87_store32(x87_sub(load(e, center + offset), operand(e, near + offset)));
                e.mem.set_u32(normal + offset, word);
            }
            e.call(UNITIZE, &args![normal]);
            let radius = load(e, e.mem.u32(base + 0x38).wrapping_add(0x38));
            let reach = [
                x87_store32(x87_mul(load(e, normal), radius)),
                x87_store32(x87_mul(load(e, normal + 4), radius)),
                x87_store32(x87_mul(radius, operand(e, normal + 8))),
            ];
            for (axis, word) in reach.iter().enumerate() {
                let offset = 4 * axis as u32;
                let point = x87_store32(x87_add(x87_load32(*word), operand(e, near + offset)));
                e.mem.set_u32(base + 0x20 + offset, point);
            }
            return;
        }
        let sum = [0u32, 4, 8].map(|offset| {
            x87_store32(x87_add(load(e, center + offset), operand(e, near + offset)))
        });
        let half = x87_load64(e.mem.u64(HALF_01011588));
        let point = [
            x87_store32(x87_mul(x87_load32(sum[0]), half)),
            x87_store32(x87_mul(x87_load32(sum[1]), half)),
            x87_store32(x87_mul(half, x87_m32(sum[2]))),
        ];
        for (axis, word) in point.iter().enumerate() {
            e.mem.set_u32(base + 0x20 + 4 * axis as u32, *word);
        }
        if e.call(POINTS_DIFFER, &args![near, center]).bool() {
            for offset in [0u32, 4, 8] {
                let word =
                    x87_store32(x87_sub(load(e, center + offset), operand(e, near + offset)));
                e.mem.set_u32(normal + offset, word);
            }
            e.call(UNITIZE, &args![normal]);
        } else {
            for offset in [0u32, 4, 8] {
                let word = e.mem.u32(DEFAULT_NORMAL_011A9484 + offset);
                e.mem.set_u32(normal + offset, word);
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ea_ea50,
            ni_capsule_sphere_intersector_pseudodistance(Ptr, Ptr, Ptr, f32) -> f64
        ),
        entry!(
            0x00ea_eb50,
            ni_capsule_sphere_intersector_compute_contact_information(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

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

    /// An intersector over a capsule (segment from the origin along +z by 2, radius 0.5) and a
    /// sphere at `center`.
    fn intersector(e: &mut Engine, center: [f32; 3]) -> u32 {
        let mut capsule = vec![0.0f32; 15];
        capsule[13] = 2.0;
        capsule[14] = 0.5;
        let capsule = floats(e, &capsule);
        let sphere = floats(e, &[0.0, center[0], center[1], center[2], 1.0]);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x38, capsule);
        e.mem.set_u32(this + 0x3c, sphere);
        e.mem.set_f32(this + 0x40, 0.25);
        this
    }

    fn unitize(e: &mut Engine, a: &[u32]) -> Ret {
        let v = read(e, a[0]);
        let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        for (i, c) in v.iter().enumerate() {
            e.mem.set_f32(a[0] + 4 * i as u32, c / length);
        }
        Ret::default()
    }

    #[test]
    fn pseudodistance_moves_both_and_scales_the_squared_distance() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ONE_01012070, 1.0);
        let this = intersector(&mut e, [3.0, 0.0, 1.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let left = floats(&mut e, &[-1.0, 0.0, 0.0]);
        e.register(SQR_DISTANCE_POINT_SEGMENT, |e, a| {
            assert_eq!(read(e, a[0]), vec![2.0, 0.0, 1.0]);
            assert_eq!(read(e, a[1]), vec![0.0, 0.0, 1.0]);
            assert_eq!(read(e, a[1] + 0xc), vec![0.0, 0.0, 2.0]);
            e.mem.set_f32(a[2], 0.5);
            Ret {
                st0: 4.0,
                ..Ret::default()
            }
        });
        let r = e.call(0x00ea_ea50, &args![this, up, left, 1.0f32]).f64();
        assert_eq!(r, 0.0);
        assert_eq!(e.mem.f32(this + 0x44), 0.5);
    }

    #[test]
    fn touching_contact_moves_to_the_contact_time() {
        let mut e = Engine::new();
        e.register(UNITIZE, unitize);
        e.register(NI_SEGMENT_GET_POINT, |e, a| {
            let t = f32::from_bits(a[2]);
            e.mem.set_f32(a[1], 0.0);
            e.mem.set_f32(a[1] + 4, 0.0);
            e.mem.set_f32(a[1] + 8, 2.0 * t);
            a[1].into_ret()
        });
        let this = intersector(&mut e, [4.0, 0.0, 1.0]);
        e.mem.set_f32(this + 0x44, 0.5);
        e.mem.set_u32(this + 0x18, 2);
        e.mem.set_f32(this + 0x1c, 2.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let left = floats(&mut e, &[-1.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x00ea_eb50, &args![this, still, left]);
        assert_eq!(calls_to(&e, NI_SEGMENT_GET_POINT)[0][2], 0.5f32.to_bits());
        assert_eq!(read(&e, this + 0x2c), vec![1.0, 0.0, 0.0]);
        assert_eq!(read(&e, this + 0x20), vec![0.5, 0.0, 1.0]);
    }

    #[test]
    fn overlapping_contact_takes_the_middle() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.mem.set_f64(HALF_01011588, 0.5);
        e.map(0x011a_9000, 0x1000);
        e.mem.set_f32(DEFAULT_NORMAL_011A9484 + 8, 1.0);
        e.register(UNITIZE, unitize);
        e.register(NI_SEGMENT_GET_POINT, |e, a| {
            e.mem.set_f32(a[1] + 8, 1.0);
            a[1].into_ret()
        });
        e.register(POINTS_DIFFER, |e, a| {
            (read(e, a[0]) != read(e, a[1])).into_ret()
        });
        let this = intersector(&mut e, [0.0, 3.0, 1.0]);
        e.mem.set_u32(this + 0x18, 3);
        e.call(0x00ea_eb50, &args![this, 0u32, 0u32]);
        assert_eq!(read(&e, this + 0x20), vec![0.0, 1.5, 1.0]);
        assert_eq!(read(&e, this + 0x2c), vec![0.0, 1.0, 0.0]);
        // The same point: the default normal.
        let same = intersector(&mut e, [0.0, 0.0, 1.0]);
        e.mem.set_u32(same + 0x18, 3);
        e.call(0x00ea_eb50, &args![same, 0u32, 0u32]);
        assert_eq!(read(&e, same + 0x2c), vec![0.0, 0.0, 1.0]);
    }
}
