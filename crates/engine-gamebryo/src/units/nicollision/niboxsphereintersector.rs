//! `NiCollision/niboxsphereintersector.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiBoxSphereIntersector` (Xbox PDB, 0x50 bytes): the `NiIntersector` base, then `m_kBox`
//! (+0x38, the box volume: center +4, axes +0x10/+0x1c/+0x28, extents +0x34..), `m_kSphere`
//! (+0x3c, the sphere volume: center +4, radius +0x10), `m_fInvRadiusSqr` (+0x40) and
//! `m_afParameter` (+0x44, three box parameters of the closest point).

#[allow(unused_imports)]
use crate::prelude::*;

/// The point-box squared distance (`00eb42e0`, cdecl: point, box, three out box parameters), in
/// ST0.
const SQR_DISTANCE_POINT_BOX: u32 = 0x00eb_42e0;
/// Unnamed in the engine map: normalizes the three floats at ECX in place.
const UNITIZE: u32 = 0x004a_0c10;
/// The double `1.0` at `01012070`.
const ONE_01012070: u32 = 0x0101_2070;
/// The double `0.5` at `01011588`.
const HALF_01011588: u32 = 0x0101_1588;
/// The `float`s `-1e-5` (`010d5368`) and `1e-5` (`010718c0`) the difference of the squared speeds
/// is compared with.
const SPEED_LOW_010D5368: u32 = 0x010d_5368;
const SPEED_HIGH_010718C0: u32 = 0x0107_18c0;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

// Translated from 00eb3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxSphereIntersector::Pseudodistance` (Xbox PDB): the box (a copy) moved by
/// `time * box_velocity` and the sphere's center by `time * sphere_velocity` (products and sums
/// stored as floats), then the squared point-box distance (`00eb42e0`, writing the three box
/// parameters) times `m_fInvRadiusSqr`, minus the double 1.0 (`01012070`), as a float in ST0.
pub fn ni_box_sphere_intersector_pseudodistance(
    e: &mut Engine,
    this: Ptr,
    box_velocity: Ptr,
    sphere_velocity: Ptr,
    time: f32,
) -> f64 {
    let base = this.addr();
    let t = x87_load32(time.to_bits());
    e.with_stack(0x48, |e, locals| {
        let point = locals.addr();
        let box_copy = point + 0xc;
        let box_volume = e.mem.u32(base + 0x38).wrapping_add(4);
        for offset in (0..0x3cu32).step_by(4) {
            let word = e.mem.u32(box_volume.wrapping_add(offset));
            e.mem.set_u32(box_copy + offset, word);
        }
        let v = box_velocity.addr();
        let shift = [
            x87_store32(x87_mul(load(e, v), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(4)), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(8)), t)),
        ];
        let center = [
            x87_store32(x87_add(x87_load32(shift[0]), operand(e, box_copy))),
            x87_store32(x87_add(load(e, box_copy + 4), x87_m32(shift[1]))),
            x87_store32(x87_add(load(e, box_copy + 8), x87_m32(shift[2]))),
        ];
        for (axis, word) in center.iter().enumerate() {
            e.mem.set_u32(box_copy + 4 * axis as u32, *word);
        }
        let w = sphere_velocity.addr();
        let shift = [
            x87_store32(x87_mul(load(e, w), t)),
            x87_store32(x87_mul(load(e, w.wrapping_add(4)), t)),
            x87_store32(x87_mul(t, operand(e, w.wrapping_add(8)))),
        ];
        let sphere_center = e.mem.u32(base + 0x3c).wrapping_add(4);
        let y = x87_store32(x87_add(
            load(e, sphere_center.wrapping_add(4)),
            x87_m32(shift[1]),
        ));
        let z = x87_store32(x87_add(
            load(e, sphere_center.wrapping_add(8)),
            x87_m32(shift[2]),
        ));
        let x = x87_store32(x87_add(load(e, sphere_center), x87_m32(shift[0])));
        e.mem.set_u32(point, x);
        e.mem.set_u32(point + 4, x87_quiet_bits(y));
        e.mem.set_u32(point + 8, x87_quiet_bits(z));
        let squared = e
            .call(
                SQR_DISTANCE_POINT_BOX,
                &args![point, box_copy, base + 0x44, base + 0x48, base + 0x4c],
            )
            .f64();
        let scaled = x87_mul(squared, operand(e, base + 0x40));
        let one = e.mem.f64(ONE_01012070);
        x87_load32(x87_store32(x87_sub(scaled, one)))
    })
}

/// Adds the box parameter at `parameter` times the axis at `axis` to the contact point (products
/// `axis.x * p`, `axis.y * p`, `p * axis.z`; sums `point.x + k.x`, `point.y + k.y`,
/// `k.z + point.z`, all stored as floats).
fn add_along(e: &mut Engine, point: u32, axis: u32, parameter: u32) {
    let p = x87_load32(x87_quiet_bits(e.mem.u32(parameter)));
    let offset = [
        x87_store32(x87_mul(load(e, axis), p)),
        x87_store32(x87_mul(load(e, axis + 4), p)),
        x87_store32(x87_mul(p, operand(e, axis + 8))),
    ];
    for component in 0..2u32 {
        let slot = point + 4 * component;
        let sum = x87_store32(x87_add(load(e, slot), x87_m32(offset[component as usize])));
        e.mem.set_u32(slot, sum);
    }
    let slot = point + 8;
    let sum = x87_store32(x87_add(x87_load32(offset[2]), operand(e, slot)));
    e.mem.set_u32(slot, sum);
}

/// The point on the box from its three parameters: each parameter times its axis added to the
/// point at `point` (which holds the starting center).
fn add_parameters(e: &mut Engine, base: u32, point: u32) {
    for axis in 0..3u32 {
        let box_volume = e.mem.u32(base + 0x38);
        add_along(
            e,
            point,
            box_volume.wrapping_add(0x10 + 12 * axis),
            base + 0x44 + 4 * axis,
        );
    }
}

/// `velocity * time` as three stored floats (`v.x * t`, `v.y * t`, `t * v.z`).
fn motion(e: &Engine, velocity: u32, time: f64) -> [u32; 3] {
    [
        x87_store32(x87_mul(load(e, velocity), time)),
        x87_store32(x87_mul(load(e, velocity.wrapping_add(4)), time)),
        x87_store32(x87_mul(time, operand(e, velocity.wrapping_add(8)))),
    ]
}

/// `|v|^2` of the three floats at `v` as the exe sums it (`y*y + x*x`, then `z*z +` that), with
/// the values loaded by `FLD`.
fn squared_speed(e: &Engine, v: u32) -> f64 {
    let x = load(e, v);
    let y = load(e, v.wrapping_add(4));
    let z = load(e, v.wrapping_add(8));
    let plane = x87_add(x87_mul(x, x), x87_mul(y, y));
    x87_add(x87_mul(z, z), plane)
}

// Translated from 00eb3d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxSphereIntersector::ComputeContactInformation` (Xbox PDB).
/// - A touching contact (type 2): the point is the box center moved by the contact time times
///   `box_velocity` plus each parameter times its axis; the normal is from it to the sphere's
///   center moved the same way with `sphere_velocity`, normalized (`004a0c10`).
/// - Otherwise, with `d = |sphere_velocity|^2 - |box_velocity|^2` (each sum stored):
///   - `d < -1e-5` (`010d5368`): with `n` the box velocity normalized and `q` the box center
///     minus twice the sphere's radius times `n`, `w` is `q` minus the sphere's center,
///     normalized; the point is the sphere's center plus the radius times `w` and the normal `-w`.
///   - `d > 1e-5` (`010718c0`): the point is the box center plus the parameters along the axes.
///   - otherwise the point is the middle of the sphere's and the box's centers (the double 0.5
///     at `01011588`).
///
///   In the last two cases the normal is the sphere's center minus the point, normalized.
pub fn ni_box_sphere_intersector_compute_contact_information(
    e: &mut Engine,
    this: Ptr,
    box_velocity: Ptr,
    sphere_velocity: Ptr,
) {
    let base = this.addr();
    let point = base + 0x20;
    let normal = base + 0x2c;
    if e.mem.u32(base + 0x18) == 2 {
        let contact = load(e, base + 0x1c);
        let shift = motion(e, box_velocity.addr(), contact);
        let center = e.mem.u32(base + 0x38).wrapping_add(4);
        let moved = [
            x87_store32(x87_add(x87_load32(shift[0]), operand(e, center))),
            x87_store32(x87_add(load(e, center.wrapping_add(4)), x87_m32(shift[1]))),
            x87_store32(x87_add(load(e, center.wrapping_add(8)), x87_m32(shift[2]))),
        ];
        for (axis, word) in moved.iter().enumerate() {
            e.mem.set_u32(point + 4 * axis as u32, *word);
        }
        add_parameters(e, base, point);
        let contact = load(e, base + 0x1c);
        let shift = motion(e, sphere_velocity.addr(), contact);
        let sphere_center = e.mem.u32(base + 0x3c).wrapping_add(4);
        let target = [0u32, 1, 2].map(|axis| {
            x87_store32(x87_add(
                load(e, sphere_center.wrapping_add(4 * axis)),
                x87_m32(shift[axis as usize]),
            ))
        });
        for (axis, word) in target.iter().enumerate() {
            let offset = 4 * axis as u32;
            let value = x87_store32(x87_sub(x87_load32(*word), operand(e, point + offset)));
            e.mem.set_u32(normal + offset, value);
        }
        e.call(UNITIZE, &args![normal]);
        return;
    }
    let v = box_velocity.addr();
    let box_speed = squared_speed(e, v);
    let sphere_speed = x87_store32(squared_speed(e, sphere_velocity.addr()));
    let box_speed = x87_store32(box_speed);
    let difference = x87_load32(x87_store32(x87_sub(
        x87_load32(sphere_speed),
        x87_m32(box_speed),
    )));
    if difference < load(e, SPEED_LOW_010D5368) {
        e.with_stack(0x24, |e, locals| {
            let direction = locals.addr();
            let away = direction + 0xc;
            let reach = direction + 0x18;
            for offset in [0u32, 4, 8] {
                let word = e.mem.u32(v.wrapping_add(offset));
                e.mem.set_u32(direction + offset, word);
            }
            e.call(UNITIZE, &args![direction]);
            let sphere = e.mem.u32(base + 0x3c);
            let radius = load(e, sphere.wrapping_add(0x10));
            let twice = x87_load32(x87_store32(x87_add(radius, radius)));
            let scaled = [
                x87_store32(x87_mul(load(e, direction), twice)),
                x87_store32(x87_mul(load(e, direction + 4), twice)),
                x87_store32(x87_mul(twice, operand(e, direction + 8))),
            ];
            let center = e.mem.u32(base + 0x38).wrapping_add(4);
            let behind = [0u32, 1, 2].map(|axis| {
                x87_store32(x87_sub(
                    load(e, center.wrapping_add(4 * axis)),
                    x87_m32(scaled[axis as usize]),
                ))
            });
            for (axis, word) in behind.iter().enumerate() {
                let offset = 4 * axis as u32;
                let value = x87_store32(x87_sub(
                    x87_load32(*word),
                    operand(e, sphere.wrapping_add(4 + offset)),
                ));
                e.mem.set_u32(away + offset, value);
            }
            e.call(UNITIZE, &args![away]);
            let radius = x87_load32(x87_quiet_bits(e.mem.u32(sphere.wrapping_add(0x10))));
            let unit = [load(e, away), load(e, away + 4), load(e, away + 8)];
            let step = [
                x87_store32(x87_mul(unit[0], radius)),
                x87_store32(x87_mul(unit[1], radius)),
                x87_store32(x87_mul(radius, unit[2])),
            ];
            for (axis, word) in step.iter().enumerate() {
                e.mem.set_u32(reach + 4 * axis as u32, *word);
            }
            let contact = [0u32, 1, 2].map(|axis| {
                x87_store32(x87_add(
                    load(e, sphere.wrapping_add(4 + 4 * axis)),
                    operand(e, reach + 4 * axis),
                ))
            });
            for (axis, word) in contact.iter().enumerate() {
                e.mem.set_u32(point + 4 * axis as u32, *word);
            }
            for (axis, value) in unit.iter().enumerate() {
                e.mem
                    .set_u32(normal + 4 * axis as u32, x87_store32(-*value));
            }
        });
        return;
    }
    if difference > operand(e, SPEED_HIGH_010718C0) {
        let center = e.mem.u32(base + 0x38).wrapping_add(4);
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(center.wrapping_add(offset));
            e.mem.set_u32(point + offset, word);
        }
        add_parameters(e, base, point);
    } else {
        let sphere_center = e.mem.u32(base + 0x3c).wrapping_add(4);
        let center = e.mem.u32(base + 0x38).wrapping_add(4);
        let sum = [0u32, 4, 8].map(|offset| {
            x87_store32(x87_add(
                load(e, sphere_center.wrapping_add(offset)),
                operand(e, center.wrapping_add(offset)),
            ))
        });
        let half = x87_load64(e.mem.u64(HALF_01011588));
        let middle = [
            x87_store32(x87_mul(x87_load32(sum[0]), half)),
            x87_store32(x87_mul(x87_load32(sum[1]), half)),
            x87_store32(x87_mul(half, x87_m32(sum[2]))),
        ];
        for (axis, word) in middle.iter().enumerate() {
            e.mem.set_u32(point + 4 * axis as u32, *word);
        }
    }
    let sphere_center = e.mem.u32(base + 0x3c).wrapping_add(4);
    let toward = [0u32, 4, 8].map(|offset| {
        x87_store32(x87_sub(
            load(e, sphere_center.wrapping_add(offset)),
            operand(e, point + offset),
        ))
    });
    for (axis, word) in toward.iter().enumerate() {
        e.mem.set_u32(normal + 4 * axis as u32, *word);
    }
    e.call(UNITIZE, &args![normal]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eb_3ca0,
            ni_box_sphere_intersector_pseudodistance(Ptr, Ptr, Ptr, f32) -> f64
        ),
        entry!(
            0x00eb_3d90,
            ni_box_sphere_intersector_compute_contact_information(Ptr, Ptr, Ptr)
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

    fn unitize(e: &mut Engine, a: &[u32]) -> Ret {
        let v = read(e, a[0]);
        let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        for (i, c) in v.iter().enumerate() {
            e.mem.set_f32(a[0] + 4 * i as u32, c / length);
        }
        Ret::default()
    }

    /// An intersector over a box at the origin (identity axes, extents 1, 2, 3) and a sphere of
    /// radius 1 at `center`.
    fn intersector(e: &mut Engine, center: [f32; 3]) -> u32 {
        let mut words = vec![0.0f32, 0.0, 0.0, 0.0];
        words.extend([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0]);
        let box_volume = floats(e, &words);
        let sphere = floats(e, &[0.0, center[0], center[1], center[2], 1.0]);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x38, box_volume);
        e.mem.set_u32(this + 0x3c, sphere);
        e.mem.set_f32(this + 0x40, 1.0);
        this
    }

    #[test]
    fn pseudodistance_moves_both_volumes() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ONE_01012070, 1.0);
        let this = intersector(&mut e, [4.0, 0.0, 0.0]);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let left = floats(&mut e, &[-1.0, 0.0, 0.0]);
        e.register(SQR_DISTANCE_POINT_BOX, |e, a| {
            assert_eq!(read(e, a[0]), vec![3.0, 0.0, 0.0]);
            assert_eq!(read(e, a[1]), vec![0.0, 0.0, 1.0]);
            e.mem.set_f32(a[2], 1.0);
            Ret {
                st0: 4.0,
                ..Ret::default()
            }
        });
        let r = e.call(0x00eb_3ca0, &args![this, up, left, 1.0f32]).f64();
        assert_eq!(r, 3.0);
        assert_eq!(e.mem.f32(this + 0x44), 1.0);
    }

    #[test]
    fn touching_contact_uses_the_parameters() {
        let mut e = Engine::new();
        e.register(UNITIZE, unitize);
        let this = intersector(&mut e, [4.0, 0.0, 0.0]);
        e.mem.set_u32(this + 0x18, 2);
        e.mem.set_f32(this + 0x1c, 2.0);
        e.mem.set_f32(this + 0x44, 1.0);
        let still = floats(&mut e, &[0.0, 0.0, 0.0]);
        let left = floats(&mut e, &[-1.0, 0.0, 0.0]);
        e.call(0x00eb_3d90, &args![this, still, left]);
        assert_eq!(read(&e, this + 0x20), vec![1.0, 0.0, 0.0]);
        assert_eq!(read(&e, this + 0x2c), vec![1.0, 0.0, 0.0]);
    }

    #[test]
    fn overlapping_contact_by_the_speeds() {
        let mut e = Engine::new();
        e.map(0x010d_5000, 0x1000);
        e.map(0x0107_1000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.mem.set_f32(SPEED_LOW_010D5368, -1e-5);
        e.mem.set_f32(SPEED_HIGH_010718C0, 1e-5);
        e.mem.set_f64(HALF_01011588, 0.5);
        e.register(UNITIZE, unitize);
        let fast = floats(&mut e, &[2.0, 0.0, 0.0]);
        let slow = floats(&mut e, &[1.0, 0.0, 0.0]);
        // The box is faster: the point on the sphere facing back along the box's motion.
        let this = intersector(&mut e, [0.0, 4.0, 0.0]);
        e.mem.set_u32(this + 0x18, 3);
        e.call(0x00eb_3d90, &args![this, fast, slow]);
        let normal = read(&e, this + 0x2c);
        let point = read(&e, this + 0x20);
        let w = [-2.0f32 / 20f32.sqrt(), -4.0 / 20f32.sqrt(), 0.0];
        for axis in 0..3 {
            assert!((normal[axis] + w[axis]).abs() < 1e-6, "{normal:?}");
        }
        assert!((point[1] - (4.0 + w[1])).abs() < 1e-6, "{point:?}");
        // The sphere is faster: the point from the box parameters.
        e.mem.set_f32(this + 0x44, 0.5);
        e.mem.set_f32(this + 0x48, 2.0);
        e.call(0x00eb_3d90, &args![this, slow, fast]);
        assert_eq!(read(&e, this + 0x20), vec![0.5, 2.0, 0.0]);
        assert_eq!(
            read(&e, this + 0x2c),
            vec![-0.5 / 4.25f32.sqrt(), 2.0 / 4.25f32.sqrt(), 0.0]
        );
        // Equal speeds: the middle of the centers.
        e.call(0x00eb_3d90, &args![this, slow, slow]);
        assert_eq!(read(&e, this + 0x20), vec![0.0, 2.0, 0.0]);
        assert_eq!(read(&e, this + 0x2c), vec![0.0, 1.0, 0.0]);
    }
}
