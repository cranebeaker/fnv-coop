//! `fallout shared/havok/viewcaster.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `ViewCaster` layout is declared in `run_006319e0.rs` (the rest of the
//! class's functions).

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_006319e0::ViewCaster;

/// `*this` of a `NiPointer` (`00559450`).
const POINTER_GET: u32 = 0x0055_9450;
/// `bhkWorldObject::GethkWorld` (engine map name of `00c87a40`): the
/// world pointer of the object's world entry, or 0.
const GET_HK_WORLD: u32 = 0x00c8_7a40;
/// `00621480(hkWorld)` (cdecl, one argument): `006214b0` on a non-null
/// world, else 0.
const WORLD_FROM_HK_WORLD: u32 = 0x0062_1480;

// Translated from 00621440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ViewCaster::GetWorld` (Xbox PDB name, from the engine map): 0 when the
/// caster has no phantom, else `00621480` applied to the phantom's
/// `GethkWorld`.
pub fn view_caster_get_world(e: &mut Engine, this: Ptr<ViewCaster>) -> u32 {
    if e.call(POINTER_GET, &args![this]).u32() == 0 {
        return 0;
    }
    let phantom = e.call(POINTER_GET, &args![this]).u32();
    let hk_world = e.call(GET_HK_WORLD, &args![phantom]).u32();
    e.call(WORLD_FROM_HK_WORLD, &args![hk_world]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0062_1440,
        view_caster_get_world(Ptr<ViewCaster>) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine(phantom: u32) -> (Engine, Ptr<ViewCaster>) {
        let mut e = Engine::new();
        let caster = e.new_object::<ViewCaster>();
        e.set(caster, ViewCaster::spShapePhantom, Ptr::new(phantom));
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GET_HK_WORLD, |_, a| (a[0] + 0x100).into_ret());
        e.register(WORLD_FROM_HK_WORLD, |_, a| (a[0] + 0x10).into_ret());
        e.call_log = Some(vec![]);
        (e, caster)
    }

    #[test]
    fn world_comes_from_the_phantoms_hk_world() {
        let (mut e, caster) = engine(0x5000);
        assert_eq!(e.call(0x0062_1440, &args![caster]).u32(), 0x5110);
        assert_eq!(logged(&e, POINTER_GET), vec![vec![caster.addr()]; 2]);
        assert_eq!(logged(&e, GET_HK_WORLD), vec![vec![0x5000]]);
        assert_eq!(logged(&e, WORLD_FROM_HK_WORLD), vec![vec![0x5100]]);
    }

    #[test]
    fn no_phantom_gives_null_without_touching_the_world() {
        let (mut e, caster) = engine(0);
        assert_eq!(e.call(0x0062_1440, &args![caster]).u32(), 0);
        assert_eq!(logged(&e, POINTER_GET).len(), 1);
        assert!(logged(&e, GET_HK_WORLD).is_empty());
        assert!(logged(&e, WORLD_FROM_HK_WORLD).is_empty());
    }
}
