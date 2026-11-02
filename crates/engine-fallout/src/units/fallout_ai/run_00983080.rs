//! `fallout/ai/run_00983080` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CombatCoverMap`'s vtable (`0108d0e4`, named by its RTTI locator).
const COMBAT_COVER_MAP_VTABLE: u32 = 0x0108_d0e4;
/// `NiTMap<unsigned int, ReservedCoverLocation*>` constructor (`00983670`,
/// `thiscall(hash size)`).
const NI_TMAP_CONSTRUCT: u32 = 0x0098_3670;
/// The pointer constructor (`00633c90`, `thiscall(object)`).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// A two-word constructor (`00aa5fc0`, `thiscall`).
const PAIR_CONSTRUCT: u32 = 0x00aa_5fc0;

// Translated from 00983080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00983080 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatCoverMap` constructor (by the vtable it sets, `0108d0e4`): the
/// `NiTMap` base with 0x25 buckets (`00983670`), the vtable, a null pointer
/// at +0x10 (`00633c90(0)`) and the two-word member at +0x14 (`00aa5fc0`).
/// Returns `this`.
pub fn fn_00983080(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_TMAP_CONSTRUCT, &args![this, 0x25u32]);
    e.mem.set_u32(this.addr(), COMBAT_COVER_MAP_VTABLE);
    e.call(POINTER_CONSTRUCT, &args![this.addr() + 0x10, 0u32]);
    e.call(PAIR_CONSTRUCT, &args![this.addr() + 0x14]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0098_3080, fn_00983080(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_the_map_pointer_and_pair() {
        let mut e = Engine::new();
        for addr in [NI_TMAP_CONSTRUCT, POINTER_CONSTRUCT, PAIR_CONSTRUCT] {
            e.register(addr, |_, a| a[0].into_ret());
        }
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0098_3080, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), COMBAT_COVER_MAP_VTABLE);
        let log: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x0098_3080)
            .collect();
        assert_eq!(
            log,
            vec![
                (NI_TMAP_CONSTRUCT, vec![this, 0x25]),
                (POINTER_CONSTRUCT, vec![this + 0x10, 0]),
                (PAIR_CONSTRUCT, vec![this + 0x14]),
            ]
        );
    }
}
