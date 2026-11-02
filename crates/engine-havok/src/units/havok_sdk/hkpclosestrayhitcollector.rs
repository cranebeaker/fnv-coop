//! `Havok SDK/hkpclosestrayhitcollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The collector (`hkpClosestRayHitCollector`, Xbox PDB) keeps the closest ray hit offered:
//! the hit's 32 bytes at `+0x10` (normal, then the hit fraction at `+0x20` and the rest of
//! the hit), the shape keys of the hit body at `+0x30` (an `hkpShapeKey` array of 8 slots
//! filled by `00cf0860`) and the root of the body's parent chain at `+0x60`. The early-out
//! fraction for callers is at `+4`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpRayHitCollector::shapeKeysFromCdBody` (cdecl: key array, capacity, body).
const SHAPE_KEYS_FROM_CD_BODY: u32 = 0x00cf_0860;

// Translated from 00ca9440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpClosestRayHitCollector::addRayHit` (Xbox PDB): when the stored fraction (`+0x20`) is
/// greater than the hit's fraction (`+0x10`) (an unordered compare, NaN, keeps the stored
/// hit), copies the hit's 16 bytes to `+0x10`, its fraction (a floating point load and
/// store, so a signalling NaN is quieted) to `+0x20` and its words `+0x14`, `+0x18`, `+0x1C`
/// to `+0x24`, `+0x28`, `+0x2C`; fills the shape key array at `+0x30` from `body` with
/// capacity 8 (`00cf0860`); stores the root of the body's parent chain at `+0x60`; and sets
/// the early-out fraction `+4` to the hit's fraction (quieted too).
pub fn hkp_closest_ray_hit_collector_add_ray_hit(e: &mut Engine, this: Ptr, body: Ptr, hit: Ptr) {
    let collector = this.addr();
    let new = hit.addr();
    if !(e.mem.f32(collector + 0x20) > e.mem.f32(new + 0x10)) {
        return;
    }
    for word in 0..4 {
        let value = e.mem.u32(new + word * 4);
        e.mem.set_u32(collector + 0x10 + word * 4, value);
    }
    let fraction = x87_quiet_bits(e.mem.u32(new + 0x10));
    e.mem.set_u32(collector + 0x20, fraction);
    for offset in [0x14, 0x18, 0x1c] {
        let value = e.mem.u32(new + offset);
        e.mem.set_u32(collector + 0x10 + offset, value);
    }
    e.call(
        SHAPE_KEYS_FROM_CD_BODY,
        &args![collector + 0x30, 8u32, body],
    );
    let mut root = body.addr();
    loop {
        let parent = e.mem.u32(root + 0xc);
        if parent == 0 {
            break;
        }
        root = parent;
    }
    e.mem.set_u32(collector + 0x60, root);
    e.mem.set_u32(collector + 4, fraction);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ca_9440,
        hkp_closest_ray_hit_collector_add_ray_hit(Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double, new_log, Log};

    const ADD_RAY_HIT: u32 = 0x00ca_9440;

    /// A collector holding a hit with fraction `stored`, and a double of `00cf0860`.
    fn collector(e: &mut Engine, stored: f32) -> (u32, Log) {
        let collector = e.mem.alloc(0x80);
        e.mem.set_f32(collector + 0x20, stored);
        let log = new_log();
        log_double(e, &log, SHAPE_KEYS_FROM_CD_BODY, 0);
        (collector, log)
    }

    /// A hit whose words are `0x200 + index` except the fraction at `+0x10`.
    fn hit(e: &mut Engine, fraction: u32) -> u32 {
        let hit = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem.set_u32(hit + word * 4, 0x200 + word);
        }
        e.mem.set_u32(hit + 0x10, fraction);
        hit
    }

    #[test]
    fn nearer_hit_is_taken_00ca9440() {
        let mut e = Engine::new();
        let (collector, log) = collector(&mut e, 0.75);
        let hit = hit(&mut e, 0.25f32.to_bits());
        let root = e.mem.alloc(0x10);
        let middle = e.mem.alloc(0x10);
        e.mem.set_u32(middle + 0xc, root);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body + 0xc, middle);

        e.call(ADD_RAY_HIT, &[collector, body, hit]);

        let words: Vec<u32> = (0..4)
            .map(|i| e.mem.u32(collector + 0x10 + i * 4))
            .collect();
        assert_eq!(words, vec![0x200, 0x201, 0x202, 0x203]);
        assert_eq!(e.mem.f32(collector + 0x20), 0.25);
        assert_eq!(e.mem.u32(collector + 0x24), 0x205);
        assert_eq!(e.mem.u32(collector + 0x28), 0x206);
        assert_eq!(e.mem.u32(collector + 0x2c), 0x207);
        assert_eq!(
            calls(&log, SHAPE_KEYS_FROM_CD_BODY),
            vec![vec![collector + 0x30, 8, body]]
        );
        assert_eq!(e.mem.u32(collector + 0x60), root);
        assert_eq!(e.mem.f32(collector + 4), 0.25);
    }

    #[test]
    fn body_without_parent_is_its_own_root_00ca9440() {
        let mut e = Engine::new();
        let (collector, _) = collector(&mut e, 1.0);
        let hit = hit(&mut e, 0.5f32.to_bits());
        let body = e.mem.alloc(0x10);
        e.call(ADD_RAY_HIT, &[collector, body, hit]);
        assert_eq!(e.mem.u32(collector + 0x60), body);
    }

    #[test]
    fn equal_farther_or_unordered_hits_are_ignored_00ca9440() {
        let mut e = Engine::new();
        let (collector, log) = collector(&mut e, 0.5);
        let body = e.mem.alloc(0x10);
        for fraction in [0.5f32.to_bits(), 0.75f32.to_bits(), f32::NAN.to_bits()] {
            let hit = hit(&mut e, fraction);
            e.call(ADD_RAY_HIT, &[collector, body, hit]);
        }
        assert!(calls(&log, SHAPE_KEYS_FROM_CD_BODY).is_empty());
        assert_eq!(e.mem.u32(collector + 4), 0);
        assert_eq!(e.mem.u32(collector + 0x10), 0);
    }

    #[test]
    fn nan_on_either_side_is_unordered_and_ignored_00ca9440() {
        let mut e = Engine::new();
        let (collector, _) = collector(&mut e, 2.0);
        let body = e.mem.alloc(0x10);
        // A signalling NaN fraction never passes the compare, so it is never stored.
        let signalling = hit(&mut e, 0x7f80_0001);
        e.call(ADD_RAY_HIT, &[collector, body, signalling]);
        assert_eq!(e.mem.f32(collector + 0x20), 2.0);
        // A NaN already stored is not greater than anything.
        e.mem.set_u32(collector + 0x20, 0x7f80_0001);
        let nearer = hit(&mut e, 0.5f32.to_bits());
        e.call(ADD_RAY_HIT, &[collector, body, nearer]);
        assert_eq!(e.mem.u32(collector + 0x20), 0x7f80_0001);
    }
}
