//! `fallout/ai/run_009a3e90` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CombatTargetMap`'s vtable (`0108e1e8`, named by its RTTI locator).
const COMBAT_TARGET_MAP_VTABLE: u32 = 0x0108_e1e8;
/// `NiTMap<Actor*, CombatTargetData*>` constructor (`009a4290`,
/// `thiscall(hash size)`).
const NI_TMAP_CONSTRUCT: u32 = 0x009a_4290;

// Translated from 009a3e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTargetMap` constructor (by the vtable it sets, `0108e1e8`): the
/// `NiTMap` base with 0x25 buckets (`009a4290`), then the vtable. Returns
/// `this`.
pub fn fn_009a3e90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_TMAP_CONSTRUCT, &args![this, 0x25u32]);
    e.mem.set_u32(this.addr(), COMBAT_TARGET_MAP_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009a_3e90, fn_009a3e90(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_the_map_and_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(NI_TMAP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a_3e90, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), COMBAT_TARGET_MAP_VTABLE);
        let log = e.call_log.unwrap();
        assert!(log.contains(&(NI_TMAP_CONSTRUCT, vec![this, 0x25])));
    }
}
