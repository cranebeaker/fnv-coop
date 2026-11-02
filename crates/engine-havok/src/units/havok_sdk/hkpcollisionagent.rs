//! `Havok SDK/hkpcollisionagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions of this unit are the empty default implementations of `hkpCollisionAgent`'s
//! virtual methods; each is a bare `RET n` that reads neither `this` (`ECX`) nor its stack words.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d9fca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionAgent::invalidateTim` (Xbox PDB): does nothing (`RET 4`).
pub fn hkp_collision_agent_invalidate_tim(e: &mut Engine, _unused_this: Ptr, _unused_1: u32) {
    let _ = e;
}

// Translated from 00d9fcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionAgent::warpTime` (Xbox PDB): does nothing (`RET 0xC`).
pub fn hkp_collision_agent_warp_time(
    e: &mut Engine,
    _unused_this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
    let _ = e;
}

// Translated from 00d9fcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionAgent::removePoint` (Xbox PDB): does nothing (`RET 4`).
pub fn hkp_collision_agent_remove_point(e: &mut Engine, _unused_this: Ptr, _unused_1: u32) {
    let _ = e;
}

// Translated from 00d9fcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionAgent::commitPotential` (Xbox PDB): does nothing (`RET 4`).
pub fn hkp_collision_agent_commit_potential(e: &mut Engine, _unused_this: Ptr, _unused_1: u32) {
    let _ = e;
}

// Translated from 00d9fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionAgent::createZombie` (Xbox PDB): does nothing (`RET 4`).
pub fn hkp_collision_agent_create_zombie(e: &mut Engine, _unused_this: Ptr, _unused_1: u32) {
    let _ = e;
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9_fca0, hkp_collision_agent_invalidate_tim(Ptr, u32)),
        entry!(
            0x00d9_fcb0,
            hkp_collision_agent_warp_time(Ptr, u32, u32, u32)
        ),
        entry!(0x00d9_fcc0, hkp_collision_agent_remove_point(Ptr, u32)),
        entry!(0x00d9_fcd0, hkp_collision_agent_commit_potential(Ptr, u32)),
        entry!(0x00d9_fce0, hkp_collision_agent_create_zombie(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calls the stub at `address` with the given stack words through the engine and checks
    /// that it returns without touching memory or calling anything.
    fn check(address: u32, words: &[u32]) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let ret = e.call(address, words);
        assert_eq!(ret.eax, 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a == address));
    }

    #[test]
    fn invalidate_tim_00d9fca0() {
        check(0x00d9_fca0, &[0x1000, 7]);
    }

    #[test]
    fn warp_time_00d9fcb0() {
        check(0x00d9_fcb0, &[0x1000, 1, 2, 3]);
    }

    #[test]
    fn remove_point_00d9fcc0() {
        check(0x00d9_fcc0, &[0x1000, 7]);
    }

    #[test]
    fn commit_potential_00d9fcd0() {
        check(0x00d9_fcd0, &[0x1000, 7]);
    }

    #[test]
    fn create_zombie_00d9fce0() {
        check(0x00d9_fce0, &[0x1000, 7]);
    }
}
