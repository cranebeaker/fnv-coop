//! `Havok SDK/hkpclosestcdpointcollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The collector (`hkpClosestCdPointCollector`, Xbox PDB) keeps the closest contact point it
//! was offered: the point's 32 bytes at `+0x10` (position, then the separating normal whose
//! `w` lane, at `+0x1C`, is the distance), the distance again at `+0x2C`, and for each of
//! the two bodies the root body of its parent chain, the shape key of the body itself and the
//! chain of shape keys, terminated by `0xFFFFFFFF`: body A at `+0x30` (root), `+0x34` (key),
//! `+0x38` (keys), body B at `+0x58`, `+0x5C`, `+0x60`. The early-out distance for callers is
//! at `+4`; a non-null root at `+0x30` means a point is stored.

#[allow(unused_imports)]
use crate::prelude::*;

/// The root of the parent chain (`+0xC` links) starting at `body`.
fn chain_root(e: &Engine, body: u32) -> u32 {
    let mut root = body;
    loop {
        let parent = e.mem.u32(root + 0xc);
        if parent == 0 {
            return root;
        }
        root = parent;
    }
}

/// The number of parents above `body`.
fn chain_depth(e: &Engine, body: u32) -> u32 {
    let mut depth = 0;
    let mut parent = e.mem.u32(body + 0xc);
    while parent != 0 {
        depth += 1;
        parent = e.mem.u32(parent + 0xc);
    }
    depth
}

/// Stores the root, the shape key and the shape key chain of `body` at `this + root_field`,
/// `+ 4` and `+ 8`: the chain holds the keys of the body and of its parents, but not of the
/// root, the farthest ancestor first, then the terminator `0xFFFFFFFF`.
fn record_body(e: &mut Engine, this: u32, root_field: u32, body: u32) {
    let root = chain_root(e, body);
    e.mem.set_u32(this + root_field, root);
    let key = e.mem.u32(body + 4);
    e.mem.set_u32(this + root_field + 4, key);
    let depth = chain_depth(e, body);
    let keys = this + root_field + 8;
    e.mem.set_u32(keys + depth.wrapping_mul(4), 0xffff_ffff);
    let mut walker = body;
    for index in (0..depth).rev() {
        let key = e.mem.u32(walker + 4);
        e.mem.set_u32(keys + index * 4, key);
        walker = e.mem.u32(walker + 0xc);
    }
}

// Translated from 00cd3b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpClosestCdPointCollector::addCdPoint` (Xbox PDB): takes the point (32 bytes at
/// `point`, its distance at `+0x1C`, the two bodies at `+0x20` and `+0x24`) when nothing is
/// stored yet (`+0x30` is null) or when the stored distance (`+0x2C`) is greater than the
/// new one; an unordered compare (NaN) keeps the stored point. Taking it copies the 32 bytes
/// to `+0x10`, records the roots, keys and key chains of the two bodies, and sets the
/// early-out distance `+4` to the point's distance (a floating point load and store, so a
/// signalling NaN is quieted).
pub fn hkp_closest_cd_point_collector_add_cd_point(e: &mut Engine, this: Ptr, point: Ptr) {
    let collector = this.addr();
    let new = point.addr();
    if e.mem.u32(collector + 0x30) != 0 {
        let stored = e.mem.f32(collector + 0x2c);
        let distance = e.mem.f32(new + 0x1c);
        if !(stored > distance) {
            return;
        }
    }
    for word in 0..8 {
        let value = e.mem.u32(new + word * 4);
        e.mem.set_u32(collector + 0x10 + word * 4, value);
    }
    let body_a = e.mem.u32(new + 0x20);
    record_body(e, collector, 0x30, body_a);
    let body_b = e.mem.u32(new + 0x24);
    record_body(e, collector, 0x58, body_b);
    let distance = x87_quiet_bits(e.mem.u32(new + 0x1c));
    e.mem.set_u32(collector + 4, distance);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cd_3b20,
        hkp_closest_cd_point_collector_add_cd_point(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADD_CD_POINT: u32 = 0x00cd_3b20;

    /// A body with a shape key and a parent.
    fn body(e: &mut Engine, key: u32, parent: u32) -> u32 {
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body + 4, key);
        e.mem.set_u32(body + 0xc, parent);
        body
    }

    /// A point with the given distance bits and bodies.
    fn point(e: &mut Engine, distance: u32, body_a: u32, body_b: u32) -> u32 {
        let point = e.mem.alloc(0x30);
        for word in 0..8 {
            e.mem.set_u32(point + word * 4, 0x100 + word);
        }
        e.mem.set_u32(point + 0x1c, distance);
        e.mem.set_u32(point + 0x20, body_a);
        e.mem.set_u32(point + 0x24, body_b);
        point
    }

    fn keys(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(address + i * 4)).collect()
    }

    #[test]
    fn first_point_is_taken_with_its_body_chains_00cd3b20() {
        let mut e = Engine::new();
        let root_a = body(&mut e, 12, 0);
        let parent_a = body(&mut e, 11, root_a);
        let body_a = body(&mut e, 10, parent_a);
        let body_b = body(&mut e, 20, 0);
        let new = point(&mut e, 2.5f32.to_bits(), body_a, body_b);
        let collector = e.mem.alloc(0x80);

        e.call(ADD_CD_POINT, &[collector, new]);

        for word in 0..8 {
            let expected = if word == 7 {
                2.5f32.to_bits()
            } else {
                0x100 + word
            };
            assert_eq!(e.mem.u32(collector + 0x10 + word * 4), expected);
        }
        assert_eq!(e.mem.u32(collector + 0x30), root_a);
        assert_eq!(e.mem.u32(collector + 0x34), 10);
        assert_eq!(keys(&e, collector + 0x38, 3), vec![11, 10, 0xffff_ffff]);
        assert_eq!(e.mem.u32(collector + 0x58), body_b);
        assert_eq!(e.mem.u32(collector + 0x5c), 20);
        assert_eq!(keys(&e, collector + 0x60, 1), vec![0xffff_ffff]);
        assert_eq!(e.mem.f32(collector + 4), 2.5);
    }

    #[test]
    fn closer_point_replaces_the_stored_one_00cd3b20() {
        let mut e = Engine::new();
        let body_a = body(&mut e, 1, 0);
        let body_b = body(&mut e, 2, 0);
        let collector = e.mem.alloc(0x80);
        let far = point(&mut e, 5.0f32.to_bits(), body_a, body_b);
        e.call(ADD_CD_POINT, &[collector, far]);
        assert_eq!(e.mem.f32(collector + 0x2c), 5.0);

        let other_a = body(&mut e, 3, 0);
        let near = point(&mut e, 4.0f32.to_bits(), other_a, body_b);
        e.call(ADD_CD_POINT, &[collector, near]);

        assert_eq!(e.mem.f32(collector + 0x2c), 4.0);
        assert_eq!(e.mem.u32(collector + 0x30), other_a);
        assert_eq!(e.mem.f32(collector + 4), 4.0);
    }

    #[test]
    fn equal_farther_or_unordered_points_are_ignored_00cd3b20() {
        let mut e = Engine::new();
        let body_a = body(&mut e, 1, 0);
        let body_b = body(&mut e, 2, 0);
        let collector = e.mem.alloc(0x80);
        let first = point(&mut e, 5.0f32.to_bits(), body_a, body_b);
        e.call(ADD_CD_POINT, &[collector, first]);

        let other_a = body(&mut e, 3, 0);
        for distance in [5.0f32.to_bits(), 6.0f32.to_bits(), f32::NAN.to_bits()] {
            let other = point(&mut e, distance, other_a, body_b);
            e.call(ADD_CD_POINT, &[collector, other]);
            assert_eq!(e.mem.u32(collector + 0x30), body_a);
            assert_eq!(e.mem.f32(collector + 0x2c), 5.0);
        }
    }

    #[test]
    fn signalling_nan_distance_is_quieted_only_in_the_early_out_00cd3b20() {
        let mut e = Engine::new();
        let body_a = body(&mut e, 1, 0);
        let body_b = body(&mut e, 2, 0);
        let signalling = 0x7f80_0001;
        let new = point(&mut e, signalling, body_a, body_b);
        let collector = e.mem.alloc(0x80);

        e.call(ADD_CD_POINT, &[collector, new]);

        assert_eq!(e.mem.u32(collector + 0x2c), signalling);
        assert_eq!(e.mem.u32(collector + 4), 0x7fc0_0001);
    }
}
