//! `NiCollision/niboxcapsuleintersector.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiBoxCapsuleIntersector` (Xbox PDB, 0x50 bytes): the `NiIntersector` base, then `m_kBox`
//! (+0x38, the box volume: center +4, axes +0x10/+0x1c/+0x28, extents +0x34..), `m_kCapsule`
//! (+0x3c, the capsule volume: segment +0x20, radius +0x38), `m_fInvRadiusSqr` (+0x40) and
//! `m_afParameter` (+0x44, three box parameters of the closest point).

#[allow(unused_imports)]
use crate::prelude::*;

/// The segment-box squared distance (`00eb50f0`, cdecl: segment, box, out segment parameter,
/// three out box parameters), in ST0.
const SQR_DISTANCE_SEGMENT_BOX: u32 = 0x00eb_50f0;
/// `NiBoxBV::GetNormal` (`00ea3a60`, thiscall on the box volume: the out point, the three side
/// flags and the relative velocity by value; answers the out point).
const NI_BOX_BV_GET_NORMAL: u32 = 0x00ea_3a60;
/// The double `1.0` at `01012070`.
const ONE_01012070: u32 = 0x0101_2070;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

// Translated from 00eb38f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxCapsuleIntersector::Pseudodistance` (Xbox PDB): the box (a copy) moved by
/// `time * box_velocity` and the capsule's segment by `time * capsule_velocity` (products and
/// sums stored as floats), then the squared segment-box distance (`00eb50f0`, writing the three
/// box parameters; the segment parameter goes to a stack slot) times `m_fInvRadiusSqr`, minus the
/// double 1.0 (`01012070`), as a float in ST0.
pub fn ni_box_capsule_intersector_pseudodistance(
    e: &mut Engine,
    this: Ptr,
    box_velocity: Ptr,
    capsule_velocity: Ptr,
    time: f32,
) -> f64 {
    let base = this.addr();
    let t = x87_load32(time.to_bits());
    e.with_stack(0x58, |e, locals| {
        let segment = locals.addr();
        let box_copy = segment + 0x18;
        let segment_parameter = box_copy + 0x3c;
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
        let capsule_segment = e.mem.u32(base + 0x3c).wrapping_add(0x20);
        for offset in (0..0x18u32).step_by(4) {
            let word = e.mem.u32(capsule_segment.wrapping_add(offset));
            e.mem.set_u32(segment + offset, word);
        }
        let v = capsule_velocity.addr();
        let shift = [
            x87_store32(x87_mul(load(e, v), t)),
            x87_store32(x87_mul(load(e, v.wrapping_add(4)), t)),
            x87_store32(x87_mul(t, operand(e, v.wrapping_add(8)))),
        ];
        let origin = [
            x87_store32(x87_add(x87_load32(shift[0]), operand(e, segment))),
            x87_store32(x87_add(load(e, segment + 4), x87_m32(shift[1]))),
            x87_store32(x87_add(load(e, segment + 8), x87_m32(shift[2]))),
        ];
        for (axis, word) in origin.iter().enumerate() {
            e.mem.set_u32(segment + 4 * axis as u32, *word);
        }
        let squared = e
            .call(
                SQR_DISTANCE_SEGMENT_BOX,
                &args![
                    segment,
                    box_copy,
                    segment_parameter,
                    base + 0x44,
                    base + 0x48,
                    base + 0x4c
                ],
            )
            .f64();
        let scaled = x87_mul(squared, operand(e, base + 0x40));
        let one = e.mem.f64(ONE_01012070);
        x87_load32(x87_store32(x87_sub(scaled, one)))
    })
}

/// The side flag of one box parameter: -1 when it equals the negated extent, 1 when it equals
/// the extent (`FUCOMPP`, unordered is unequal), otherwise 0.
fn side(e: &Engine, parameter: u32, extent: u32) -> i32 {
    let value = load(e, parameter);
    if -load(e, extent) == value {
        -1
    } else if load(e, extent) == value {
        1
    } else {
        0
    }
}

/// Adds `parameter` times the axis at `axis` to the contact point at `point` (products
/// `axis.x * p`, `axis.y * p`, `p * axis.z`, then `point + product` for each component, all
/// stored as floats).
fn add_along(e: &mut Engine, point: u32, axis: u32, parameter: u32) {
    let p = x87_load32(x87_quiet_bits(e.mem.u32(parameter)));
    let offset = [
        x87_store32(x87_mul(load(e, axis), p)),
        x87_store32(x87_mul(load(e, axis + 4), p)),
        x87_store32(x87_mul(p, operand(e, axis + 8))),
    ];
    for (component, word) in offset.iter().enumerate() {
        let slot = point + 4 * component as u32;
        let sum = x87_store32(x87_add(load(e, slot), x87_m32(*word)));
        e.mem.set_u32(slot, sum);
    }
}

// Translated from 00eb3a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoxCapsuleIntersector::ComputeContactInformation` (Xbox PDB): the contact point is the box
/// center moved by the contact time times `box_velocity`, plus each parameter times its axis; each
/// parameter's side flag is -1 or 1 when it sits on the negative or positive face (equal to the
/// negated or the plain extent) and 0 otherwise; the normal comes from `NiBoxBV::GetNormal` with
/// the three flags and `capsule_velocity - box_velocity` (stored floats).
pub fn ni_box_capsule_intersector_compute_contact_information(
    e: &mut Engine,
    this: Ptr,
    box_velocity: Ptr,
    capsule_velocity: Ptr,
) {
    let base = this.addr();
    let point = base + 0x20;
    let contact = load(e, base + 0x1c);
    let v = box_velocity.addr();
    let shift = [
        x87_store32(x87_mul(load(e, v), contact)),
        x87_store32(x87_mul(load(e, v.wrapping_add(4)), contact)),
        x87_store32(x87_mul(contact, operand(e, v.wrapping_add(8)))),
    ];
    let center = e.mem.u32(base + 0x38).wrapping_add(4);
    let moved = [0u32, 1, 2].map(|axis| {
        x87_store32(x87_add(
            load(e, center.wrapping_add(4 * axis)),
            x87_m32(shift[axis as usize]),
        ))
    });
    for (axis, word) in moved.iter().enumerate() {
        e.mem.set_u32(point + 4 * axis as u32, *word);
    }
    let mut flags = [0i32; 3];
    for (axis, flag) in flags.iter_mut().enumerate() {
        let axis = axis as u32;
        let box_volume = e.mem.u32(base + 0x38);
        add_along(
            e,
            point,
            box_volume.wrapping_add(0x10 + 12 * axis),
            base + 0x44 + 4 * axis,
        );
        let box_volume = e.mem.u32(base + 0x38);
        *flag = side(
            e,
            base + 0x44 + 4 * axis,
            box_volume.wrapping_add(0x34 + 4 * axis),
        );
    }
    let w = capsule_velocity.addr();
    let relative = [0u32, 4, 8].map(|offset| {
        x87_store32(x87_sub(
            load(e, w.wrapping_add(offset)),
            operand(e, v.wrapping_add(offset)),
        ))
    });
    e.with_stack(12, |e, out| {
        let box_volume = e.mem.u32(base + 0x38);
        let normal = e
            .call(
                NI_BOX_BV_GET_NORMAL,
                &args![
                    box_volume,
                    out,
                    flags[0],
                    flags[1],
                    flags[2],
                    relative[0],
                    relative[1],
                    relative[2]
                ],
            )
            .u32();
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(normal.wrapping_add(offset));
            e.mem.set_u32(base + 0x2c + offset, word);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eb_38f0,
            ni_box_capsule_intersector_pseudodistance(Ptr, Ptr, Ptr, f32) -> f64
        ),
        entry!(
            0x00eb_3a00,
            ni_box_capsule_intersector_compute_contact_information(Ptr, Ptr, Ptr)
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

    /// An intersector over a unit box at the origin (extents 1, 2, 3) and a capsule whose
    /// segment starts at (5, 0, 0) and runs along +z by 2.
    fn intersector(e: &mut Engine) -> u32 {
        let mut words = vec![0.0f32, 0.0, 0.0, 0.0];
        words.extend([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0]);
        let box_volume = floats(e, &words);
        let mut capsule = vec![0.0f32; 15];
        capsule[8] = 5.0;
        capsule[13] = 2.0;
        capsule[14] = 1.0;
        let capsule = floats(e, &capsule);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x38, box_volume);
        e.mem.set_u32(this + 0x3c, capsule);
        e.mem.set_f32(this + 0x40, 0.5);
        this
    }

    #[test]
    fn pseudodistance_moves_both_volumes() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ONE_01012070, 1.0);
        let this = intersector(&mut e);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let left = floats(&mut e, &[-1.0, 0.0, 0.0]);
        e.register(SQR_DISTANCE_SEGMENT_BOX, |e, a| {
            assert_eq!(read(e, a[0]), vec![3.0, 0.0, 0.0]);
            assert_eq!(read(e, a[1]), vec![0.0, 0.0, 2.0]);
            assert_eq!(e.mem.f32(a[1] + 0x34), 2.0);
            e.mem.set_f32(a[3], 1.0);
            Ret {
                st0: 6.0,
                ..Ret::default()
            }
        });
        let r = e.call(0x00eb_38f0, &args![this, up, left, 2.0f32]).f64();
        assert_eq!(r, 2.0);
        assert_eq!(e.mem.f32(this + 0x44), 1.0);
    }

    #[test]
    fn contact_point_from_the_box_parameters() {
        let mut e = Engine::new();
        let this = intersector(&mut e);
        e.mem.set_f32(this + 0x1c, 2.0);
        e.mem.set_f32(this + 0x44, 1.0);
        e.mem.set_f32(this + 0x48, -2.0);
        e.mem.set_f32(this + 0x4c, 0.5);
        let up = floats(&mut e, &[0.0, 0.0, 1.0]);
        let left = floats(&mut e, &[-1.0, 0.0, 0.0]);
        e.register(NI_BOX_BV_GET_NORMAL, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            a[1].into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00eb_3a00, &args![this, up, left]);
        assert_eq!(read(&e, this + 0x20), vec![1.0, -2.0, 2.5]);
        let call = &calls_to(&e, NI_BOX_BV_GET_NORMAL)[0];
        assert_eq!(call[0], e.mem.u32(this + 0x38));
        assert_eq!(&call[2..5], &[1, (-1i32) as u32, 0]);
        assert_eq!(
            &call[5..8],
            &[(-1.0f32).to_bits(), 0.0f32.to_bits(), (-1.0f32).to_bits()]
        );
        assert_eq!(read(&e, this + 0x2c), vec![1.0, 0.0, 0.0]);
    }
}
