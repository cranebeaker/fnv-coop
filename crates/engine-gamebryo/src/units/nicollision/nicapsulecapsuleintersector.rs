//! `NiCollision/nicapsulecapsuleintersector.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiCapsuleCapsuleIntersector` (Xbox PDB, 0x4c bytes): the `NiIntersector` base, then
//! `m_kCapsule0` (+0x38), `m_kCapsule1` (+0x3c), `m_fInvRadiusSumSqr` (+0x40) and the two segment
//! parameters of the closest points `m_fParameter0` (+0x44) and `m_fParameter1` (+0x48). Each
//! capsule volume's segment is at +0x20 (origin, then direction) and its radius at +0x38.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSqrDistance::Compute_ov2` (`00eaf310`, cdecl: segment, segment, two out parameters), in ST0.
const SQR_DISTANCE_SEGMENT_SEGMENT: u32 = 0x00ea_f310;
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
/// The three words at `011a9484` (the z axis in the image): the normal when the two points meet.
const DEFAULT_NORMAL_011A9484: u32 = 0x011a_9484;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// `velocity * time` as three stored floats (`v.x * t`, `v.y * t`, `t * v.z`).
fn motion(e: &Engine, velocity: u32, time: f64) -> [u32; 3] {
    [
        x87_store32(x87_mul(load(e, velocity), time)),
        x87_store32(x87_mul(load(e, velocity.wrapping_add(4)), time)),
        x87_store32(x87_mul(time, operand(e, velocity.wrapping_add(8)))),
    ]
}

/// Adds the stored motion to the three floats at `point` in the exe's operand order
/// (`m.x + p.x`, `p.y + m.y`, `p.z + m.z`), storing each sum.
fn move_point(e: &mut Engine, point: u32, shift: [u32; 3]) {
    let moved = [
        x87_store32(x87_add(x87_load32(shift[0]), operand(e, point))),
        x87_store32(x87_add(load(e, point + 4), x87_m32(shift[1]))),
        x87_store32(x87_add(load(e, point + 8), x87_m32(shift[2]))),
    ];
    for (axis, word) in moved.iter().enumerate() {
        e.mem.set_u32(point + 4 * axis as u32, *word);
    }
}

// Translated from 00eaee50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleCapsuleIntersector::Pseudodistance` (Xbox PDB): both segments' origins moved by
/// `time` times their capsule's velocity (products and sums stored as floats), then the squared
/// distance between the segments (`NiSqrDistance::Compute_ov2`, writing both parameters) times
/// `m_fInvRadiusSumSqr`, minus the double 1.0 (`01012070`), as a float in ST0.
pub fn ni_capsule_capsule_intersector_pseudodistance(
    e: &mut Engine,
    this: Ptr,
    velocity0: Ptr,
    velocity1: Ptr,
    time: f32,
) -> f64 {
    let base = this.addr();
    let t = x87_load32(time.to_bits());
    e.with_stack(0x30, |e, locals| {
        let second = locals.addr();
        let first = second + 0x18;
        let first_segment = e.mem.u32(base + 0x38).wrapping_add(0x20);
        let second_segment = e.mem.u32(base + 0x3c).wrapping_add(0x20);
        for offset in (0..0x18u32).step_by(4) {
            let word = e.mem.u32(first_segment.wrapping_add(offset));
            e.mem.set_u32(first + offset, word);
            let word = e.mem.u32(second_segment.wrapping_add(offset));
            e.mem.set_u32(second + offset, word);
        }
        let v = velocity0.addr();
        let shift = [
            x87_store32(x87_mul(load(e, v), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(4)), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(8)), t)),
        ];
        move_point(e, first, shift);
        let shift = motion(e, velocity1.addr(), t);
        move_point(e, second, shift);
        let squared = e
            .call(
                SQR_DISTANCE_SEGMENT_SEGMENT,
                &args![first, second, base + 0x44, base + 0x48],
            )
            .f64();
        let scaled = x87_mul(squared, operand(e, base + 0x40));
        let one = e.mem.f64(ONE_01012070);
        x87_load32(x87_store32(x87_sub(scaled, one)))
    })
}

// Translated from 00eaef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsuleCapsuleIntersector::ComputeContactInformation` (Xbox PDB): with `q0` and `q1` the
/// segment points at the two parameters (`NiSegment::GetPoint`):
/// - a touching contact (type 2): both moved by the contact time times their velocities (stored
///   floats), the normal `m_kContactNormal` is `q1 - q0` normalized (`004a0c10`) and the contact
///   point `q0` plus the first capsule's radius times the normal.
/// - otherwise the contact point is `(q1 + q0) * 0.5` (the double at `01011588`) and the normal
///   `q1 - q0` normalized, or the three words at `011a9484` when the points do not differ
///   (`00439090` on `q1`).
pub fn ni_capsule_capsule_intersector_compute_contact_information(
    e: &mut Engine,
    this: Ptr,
    velocity0: Ptr,
    velocity1: Ptr,
) {
    let base = this.addr();
    let normal = base + 0x2c;
    e.with_stack(0x18, |e, locals| {
        let first = locals.addr();
        let second = first + 0xc;
        let parameter = x87_quiet_bits(e.mem.u32(base + 0x44));
        let segment = e.mem.u32(base + 0x38).wrapping_add(0x20);
        e.call(NI_SEGMENT_GET_POINT, &args![segment, first, parameter]);
        let parameter = x87_quiet_bits(e.mem.u32(base + 0x48));
        let segment = e.mem.u32(base + 0x3c).wrapping_add(0x20);
        e.call(NI_SEGMENT_GET_POINT, &args![segment, second, parameter]);
        if e.mem.u32(base + 0x18) == 2 {
            let contact = load(e, base + 0x1c);
            let shift = motion(e, velocity0.addr(), contact);
            move_point(e, first, shift);
            let contact = load(e, base + 0x1c);
            let shift = motion(e, velocity1.addr(), contact);
            move_point(e, second, shift);
            for offset in [0u32, 4, 8] {
                let word = x87_store32(x87_sub(
                    load(e, second + offset),
                    operand(e, first + offset),
                ));
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
                let point = x87_store32(x87_add(x87_load32(*word), operand(e, first + offset)));
                e.mem.set_u32(base + 0x20 + offset, point);
            }
            return;
        }
        let sum = [0u32, 4, 8].map(|offset| {
            x87_store32(x87_add(
                load(e, second + offset),
                operand(e, first + offset),
            ))
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
        if e.call(POINTS_DIFFER, &args![second, first]).bool() {
            for offset in [0u32, 4, 8] {
                let word = x87_store32(x87_sub(
                    load(e, second + offset),
                    operand(e, first + offset),
                ));
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
            0x00ea_ee50,
            ni_capsule_capsule_intersector_pseudodistance(Ptr, Ptr, Ptr, f32) -> f64
        ),
        entry!(
            0x00ea_ef80,
            ni_capsule_capsule_intersector_compute_contact_information(Ptr, Ptr, Ptr)
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

    /// A capsule volume whose segment starts at `origin` and runs along +z by 2, radius 0.5.
    fn capsule(e: &mut Engine, origin: [f32; 3]) -> u32 {
        let mut words = vec![0.0f32; 15];
        words[8..11].copy_from_slice(&origin);
        words[13] = 2.0;
        words[14] = 0.5;
        floats(e, &words)
    }

    fn intersector(e: &mut Engine, second_origin: [f32; 3]) -> u32 {
        let first = capsule(e, [0.0; 3]);
        let second = capsule(e, second_origin);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 0x38, first);
        e.mem.set_u32(this + 0x3c, second);
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

    /// `GetPoint` for the tests: origin plus the parameter times the direction.
    fn get_point(e: &mut Engine, a: &[u32]) -> Ret {
        let t = f32::from_bits(a[2]);
        for i in 0..3 {
            let v = e.mem.f32(a[0] + 4 * i) + t * e.mem.f32(a[0] + 0xc + 4 * i);
            e.mem.set_f32(a[1] + 4 * i, v);
        }
        a[1].into_ret()
    }

    #[test]
    fn pseudodistance_moves_both_segments() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ONE_01012070, 1.0);
        let this = intersector(&mut e, [3.0, 0.0, 0.0]);
        let right = floats(&mut e, &[1.0, 0.0, 0.0]);
        let left = floats(&mut e, &[-1.0, 0.0, 0.0]);
        e.register(SQR_DISTANCE_SEGMENT_SEGMENT, |e, a| {
            assert_eq!(read(e, a[0]), vec![0.5, 0.0, 0.0]);
            assert_eq!(read(e, a[1]), vec![2.5, 0.0, 0.0]);
            assert_eq!(read(e, a[1] + 0xc), vec![0.0, 0.0, 2.0]);
            e.mem.set_f32(a[2], 0.25);
            e.mem.set_f32(a[3], 0.75);
            Ret {
                st0: 4.0,
                ..Ret::default()
            }
        });
        let r = e.call(0x00ea_ee50, &args![this, right, left, 0.5f32]).f64();
        assert_eq!(r, 0.0);
        assert_eq!(e.mem.f32(this + 0x44), 0.25);
        assert_eq!(e.mem.f32(this + 0x48), 0.75);
    }

    #[test]
    fn contact_information_touching_and_overlapping() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.mem.set_f64(HALF_01011588, 0.5);
        e.map(0x011a_9000, 0x1000);
        e.mem.set_f32(DEFAULT_NORMAL_011A9484 + 8, 1.0);
        e.register(UNITIZE, unitize);
        e.register(NI_SEGMENT_GET_POINT, get_point);
        e.register(POINTS_DIFFER, |e, a| {
            (read(e, a[0]) != read(e, a[1])).into_ret()
        });
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let left = floats(&mut e, &[-1.0, 0.0, 0.0]);
        // Touching at t = 1: the second capsule has come from x = 2 to x = 1.
        let this = intersector(&mut e, [2.0, 0.0, 0.0]);
        e.mem.set_f32(this + 0x44, 0.5);
        e.mem.set_f32(this + 0x48, 0.5);
        e.mem.set_u32(this + 0x18, 2);
        e.mem.set_f32(this + 0x1c, 1.0);
        e.call(0x00ea_ef80, &args![this, still, left]);
        assert_eq!(read(&e, this + 0x2c), vec![1.0, 0.0, 0.0]);
        assert_eq!(read(&e, this + 0x20), vec![0.5, 0.0, 1.0]);
        // Overlapping: the middle.
        e.mem.set_u32(this + 0x18, 3);
        e.call(0x00ea_ef80, &args![this, still, left]);
        assert_eq!(read(&e, this + 0x20), vec![1.0, 0.0, 1.0]);
        assert_eq!(read(&e, this + 0x2c), vec![1.0, 0.0, 0.0]);
        let same = intersector(&mut e, [0.0, 0.0, 0.0]);
        e.mem.set_u32(same + 0x18, 3);
        e.call(0x00ea_ef80, &args![same, still, left]);
        assert_eq!(read(&e, same + 0x2c), vec![0.0, 0.0, 1.0]);
    }
}
