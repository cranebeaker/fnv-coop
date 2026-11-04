//! `(unplaced)/run_00854ea0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tessaveloadgame::ChangesMap;

/// The `NiTPointerMap<unsigned int, ChangeData *>` constructor (`00863360`,
/// `thiscall(hash size)`).
const POINTER_MAP_CONSTRUCT: u32 = 0x0086_3360;
/// The hash size passed to it.
const HASH_SIZE: u32 = 0x13af;
/// The vtable stored after it (`010804a8`; RTTI name `.?AVChangesMap@@`, read
/// through its complete object locator).
const CHANGES_MAP_VTABLE: u32 = 0x0108_04a8;

// Translated from 00854ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable the `ChangesMap` constructor: the
/// `NiTPointerMap` constructor `00863360` with hash size 0x13AF, then the
/// `ChangesMap` vtable. Returns `this`.
pub fn fn_00854ea0(e: &mut Engine, this: Ptr<ChangesMap>) -> Ptr<ChangesMap> {
    e.call(POINTER_MAP_CONSTRUCT, &args![this, HASH_SIZE]);
    e.mem.set_u32(this.addr(), CHANGES_MAP_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00854ea0,
        fn_00854ea0(Ptr<ChangesMap>) -> Ptr<ChangesMap>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_map_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(POINTER_MAP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111_1111);
            e.mem.set_u32(a[0] + 4, a[1]);
            a[0].into_ret()
        });
        let map: Ptr<ChangesMap> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0085_4ea0, &args![map]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, map.addr());
        let constructs: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == POINTER_MAP_CONSTRUCT)
            .collect();
        assert_eq!(constructs.len(), 1);
        assert_eq!(constructs[0].1, args![map, 0x13afu32]);
        assert_eq!(e.mem.u32(map.addr()), CHANGES_MAP_VTABLE);
        assert_eq!(e.get(map, ChangesMap::m_uiHashSize), 0x13af);
    }
}
