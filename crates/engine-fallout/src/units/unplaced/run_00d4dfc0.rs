//! `(unplaced)/run_00d4dfc0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpBreakableConstraintData` vtable (RTTI).
const VTABLE_BREAKABLE_CONSTRAINT_DATA: u32 = 0x010d_0a1c;
/// `hkpBridgeConstraintAtom::init` (`00d21ce0`, engine map; thiscall, `RET 4`).
const BRIDGE_ATOM_INIT: u32 = 0x00d2_1ce0;

// Translated from 00d4dfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor taking one word (the class by the vtable's RTTI is
/// `hkpBreakableConstraintData`; the word's meaning is unconfirmed): stores the vtable; when
/// the word is not 0, runs `hkpBridgeConstraintAtom::init` on the atom at +0xc with the word
/// at +0x14 (the atom's +8), then, the word tested again, once more with `this`. Returns
/// `this`.
pub fn fn_00d4dfc0(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_BREAKABLE_CONSTRAINT_DATA);
    let atom = base + 0xc;
    if flag != 0 {
        let data = e.mem.u32(atom + 8);
        e.call(BRIDGE_ATOM_INIT, &args![atom, data]);
        if flag != 0 {
            e.call(BRIDGE_ATOM_INIT, &args![atom, base]);
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_dfc0, fn_00d4dfc0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    #[test]
    fn constructor_initializes_the_atom_twice_when_the_word_is_set() {
        for flag in [0u32, 1] {
            let mut e = Engine::new();
            e.register(BRIDGE_ATOM_INIT, |_, _| Ret::default());
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 0x14, 0x5555);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00d4_dfc0, &args![this, flag]).u32(), this);
            assert_eq!(e.mem.u32(this), VTABLE_BREAKABLE_CONSTRAINT_DATA);
            let calls: Vec<_> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a == BRIDGE_ATOM_INIT)
                .map(|(_, w)| w)
                .collect();
            let want = if flag != 0 {
                vec![vec![this + 0xc, 0x5555], vec![this + 0xc, this]]
            } else {
                vec![]
            };
            assert_eq!(calls, want);
        }
    }
}
