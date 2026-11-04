//! `fallout/magic/run_00806eb0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00c741e0` (`thiscall`, one word).
const BASE_CONSTRUCT_00C741E0: u32 = 0x00c7_41e0;
/// The three vtables the constructor stores at +0, +0x3E0 and +0x410
/// (`010795e4`, `010795d4`, `010795b8`; RTTI
/// `.?AVbhkCharacterListenerSpell@@`).
const LISTENER_VTABLE: u32 = 0x0107_95e4;
const LISTENER_VTABLE_3E0: u32 = 0x0107_95d4;
const LISTENER_VTABLE_410: u32 = 0x0107_95b8;

// Translated from 00806eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the class whose vtables are `010795e4`, `010795d4`
/// and `010795b8` (RTTI `bhkCharacterListenerSpell`; `thiscall`,
/// `RET 8`): constructs the base (`00c741e0`) with the first word, stores
/// the three vtables (+0, +0x3E0, +0x410) and the second word at +0x650.
/// Returns `this`.
pub fn fn_00806eb0(e: &mut Engine, this: Ptr, base_argument: u32, owner: u32) -> Ptr {
    e.call(BASE_CONSTRUCT_00C741E0, &args![this, base_argument]);
    let base = this.addr();
    e.mem.set_u32(base, LISTENER_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x3e0), LISTENER_VTABLE_3E0);
    e.mem.set_u32(base.wrapping_add(0x410), LISTENER_VTABLE_410);
    e.mem.set_u32(base.wrapping_add(0x650), owner);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_6eb0, fn_00806eb0(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stores_the_vtables_and_the_owner() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_00C741E0, |_, _| Ret::default());
        let this = e.mem.alloc(0x654);
        e.call_log = Some(vec![]);
        let r = e.call(0x0080_6eb0, &args![this, 0x77u32, 0x88u32]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == BASE_CONSTRUCT_00C741E0)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, vec![this, 0x77]);
        assert_eq!(e.mem.u32(this), LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x3e0), LISTENER_VTABLE_3E0);
        assert_eq!(e.mem.u32(this + 0x410), LISTENER_VTABLE_410);
        assert_eq!(e.mem.u32(this + 0x650), 0x88);
    }
}
