//! `(unplaced)/run_00a6e960` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The palette map's `RemoveAll` (`00aa01f0`, `thiscall` on `this + 8`)
/// and `RecurseAndAddObjectsToPalette` (`00a6e870`, `cdecl (scene,
/// palette)`).
const MAP_REMOVE_ALL: u32 = 0x00aa_01f0;
const RECURSE_AND_ADD_OBJECTS: u32 = 0x00a6_e870;

// Translated from 00a6e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refills an object palette from its scene (the engine map has no name;
/// the callees are `NiDefaultAVObjectPalette`'s): when the scene at +0x18
/// is set, the name map at +8 is emptied and every object under the scene
/// added again.
pub fn fn_00a6e960(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x18) == 0 {
        return;
    }
    e.call(MAP_REMOVE_ALL, &args![this.addr() + 8]);
    let scene = e.mem.u32(this.addr() + 0x18);
    e.call(RECURSE_AND_ADD_OBJECTS, &args![scene, this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_e960, fn_00a6e960(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_is_refilled_only_with_a_scene() {
        let mut e = Engine::new();
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(RECURSE_AND_ADD_OBJECTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x00a6_e960, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_u32(this + 0x18, 0x4000);
        e.call(0x00a6_e960, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (MAP_REMOVE_ALL, vec![this + 8]));
        assert_eq!(log[3], (RECURSE_AND_ADD_OBJECTS, vec![0x4000, this]));
    }
}
