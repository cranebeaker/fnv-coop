//! `Havok SDK/hkpsimpleclosestcontactcollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dafc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleClosestContactCollector::addCdPoint` (Xbox PDB): keeps the closest contact point.
/// When no point is held yet (byte at +8 clear) or the held distance (`float` at +0x2c) is
/// greater than the new point's (`float` at `point + 0x1c`; the `FCOMPP` test skips equal and
/// unordered values), sets the byte, copies the 32-byte point to +0x10 (two 16-byte moves) and
/// stores its distance at +4 through the x87 stack.
pub fn hkp_simple_closest_contact_collector_add_cd_point(e: &mut Engine, this: Ptr, point: Ptr) {
    let base = this.addr();
    let source = point.addr();
    if e.mem.u8(base + 8) != 0 {
        let held = e.mem.f32(base + 0x2c);
        let new = e.mem.f32(source + 0x1c);
        if held.partial_cmp(&new) != Some(std::cmp::Ordering::Greater) {
            return;
        }
    }
    e.mem.set_u8(base + 8, 1);
    for offset in (0..0x20).step_by(4) {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(base + 0x10 + offset, word);
    }
    let distance = e.mem.u32(source + 0x1c);
    e.mem.set_u32(base + 4, x87_quiet_bits(distance));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00da_fc90,
        hkp_simple_closest_contact_collector_add_cd_point(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(e: &mut Engine, distance_bits: u32) -> u32 {
        let point = e.mem.alloc(0x20);
        for lane in 0..8 {
            e.mem.set_u32(point + 4 * lane, 0x100 + lane);
        }
        e.mem.set_u32(point + 0x1c, distance_bits);
        point
    }

    #[test]
    fn the_first_point_is_always_kept() {
        let mut e = Engine::new();
        let collector = e.mem.alloc(0x40);
        e.mem.set_f32(collector + 0x2c, -1.0);
        let point = point(&mut e, 0x7f80_0001);
        e.call(0x00da_fc90, &args![collector, point]);
        assert_eq!(e.mem.u8(collector + 8), 1);
        for lane in 0..7 {
            assert_eq!(e.mem.u32(collector + 0x10 + 4 * lane), 0x100 + lane);
        }
        // The distance word passes through the x87 stack: the signalling NaN is quieted.
        assert_eq!(e.mem.u32(collector + 0x2c), 0x7f80_0001);
        assert_eq!(e.mem.u32(collector + 4), 0x7fc0_0001);
    }

    #[test]
    fn a_closer_point_replaces_the_held_one_and_others_are_ignored() {
        let mut e = Engine::new();
        let collector = e.mem.alloc(0x40);
        e.mem.set_u8(collector + 8, 1);
        e.mem.set_f32(collector + 0x2c, 2.0);
        e.mem.set_f32(collector + 4, 9.0);
        let farther = point(&mut e, 3.0f32.to_bits());
        e.call(0x00da_fc90, &args![collector, farther]);
        let equal = point(&mut e, 2.0f32.to_bits());
        e.call(0x00da_fc90, &args![collector, equal]);
        let unordered = point(&mut e, 0x7fc0_0000);
        e.call(0x00da_fc90, &args![collector, unordered]);
        assert_eq!(e.mem.f32(collector + 4), 9.0);
        assert_eq!(e.mem.u32(collector + 0x10), 0);
        let closer = point(&mut e, 1.5f32.to_bits());
        e.call(0x00da_fc90, &args![collector, closer]);
        assert_eq!(e.mem.f32(collector + 4), 1.5);
        assert_eq!(e.mem.f32(collector + 0x2c), 1.5);
        assert_eq!(e.mem.u32(collector + 0x10), 0x100);
    }

    #[test]
    fn a_held_nan_distance_keeps_the_point() {
        let mut e = Engine::new();
        let collector = e.mem.alloc(0x40);
        e.mem.set_u8(collector + 8, 1);
        e.mem.set_u32(collector + 0x2c, 0x7fc0_0000);
        let closer = point(&mut e, 1.0f32.to_bits());
        e.call(0x00da_fc90, &args![collector, closer]);
        assert_eq!(e.mem.u32(collector + 0x10), 0);
    }
}
