//! `NiCollision/nisegment.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00eb6090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSegment::GetPoint` (Xbox PDB): writes `m_kOrigin + parameter * m_kDirection` to `out`
/// (`this` is the segment: origin at +0, direction at +0xc). Each product is stored as a float
/// first (`direction.x * t`, `direction.y * t`, `t * direction.z`), then each sum `origin + product`
/// is stored into `out`, x first. Answers `out`.
pub fn ni_segment_get_point(e: &mut Engine, this: Ptr, out: Ptr, parameter: f32) -> Ptr {
    let s = this.addr();
    let t = x87_load32(parameter.to_bits());
    let load = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
    let operand = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
    let along = [
        x87_store32(x87_mul(load(e, s.wrapping_add(0xc)), t)),
        x87_store32(x87_mul(load(e, s.wrapping_add(0x10)), t)),
        x87_store32(x87_mul(t, operand(e, s.wrapping_add(0x14)))),
    ];
    for (axis, word) in along.iter().enumerate() {
        let offset = 4 * axis as u32;
        let sum = x87_store32(x87_add(load(e, s.wrapping_add(offset)), x87_m32(*word)));
        e.mem.set_u32(out.addr().wrapping_add(offset), sum);
    }
    out
}

// Translated from 00eb60e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSegment::operator==` (Xbox PDB): whether the six floats (origin, then direction) of both
/// segments are equal, compared in order with `FUCOMPP` (unordered is unequal).
pub fn ni_segment_operator_(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    (0..6u32).all(|index| {
        let offset = 4 * index;
        let mine = x87_load32(e.mem.u32(this.addr().wrapping_add(offset)));
        let theirs = x87_load32(e.mem.u32(other.addr().wrapping_add(offset)));
        theirs == mine
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00eb_6090, ni_segment_get_point(Ptr, Ptr, f32) -> Ptr),
        entry!(0x00eb_60e0, ni_segment_operator_(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    #[test]
    fn get_point_moves_along_the_direction() {
        let mut e = Engine::new();
        let segment = floats(&mut e, &[1.0, 2.0, 3.0, 2.0, -4.0, 0.5]);
        let out = e.mem.alloc(12);
        let r = e.call(0x00eb_6090, &args![segment, out, 0.5f32]).u32();
        assert_eq!(r, out);
        assert_eq!(
            (0..3).map(|i| e.mem.f32(out + 4 * i)).collect::<Vec<_>>(),
            vec![2.0, 0.0, 3.25]
        );
    }

    #[test]
    fn equality_compares_all_six_floats() {
        let mut e = Engine::new();
        let a = floats(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = floats(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert!(e.call(0x00eb_60e0, &args![a, b]).bool());
        e.mem.set_f32(b + 0x14, 6.5);
        assert!(!e.call(0x00eb_60e0, &args![a, b]).bool());
        e.mem.set_f32(b + 0x14, 6.0);
        e.mem.set_f32(a, f32::NAN);
        e.mem.set_f32(b, f32::NAN);
        assert!(!e.call(0x00eb_60e0, &args![a, b]).bool());
        // Negative and positive zero are equal.
        e.mem.set_f32(a, -0.0);
        e.mem.set_f32(b, 0.0);
        assert!(e.call(0x00eb_60e0, &args![a, b]).bool());
    }
}
