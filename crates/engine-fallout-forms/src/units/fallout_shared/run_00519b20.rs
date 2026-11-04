//! `fallout shared/run_00519b20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00519b80` (`tesobjectdoor.cpp`, `thiscall`, one
/// word) and the vtable this constructor stores (`0102b3d0`, RTTI
/// `.?AV?$NiTPointerMap@H_N@@`).
const BASE_CONSTRUCT: u32 = 0x0051_9b80;
const VTABLE: u32 = 0x0102_b3d0;

// Translated from 00519b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPointerMap<int,bool>` constructor (by the RTTI of the vtable it
/// stores; `thiscall`, one word, `RET 4`): runs the base constructor
/// `00519b80` with `size`, stores the vtable `0102b3d0` and returns `this`.
pub fn fn_00519b20(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this, size]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_9b20, fn_00519b20(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_00519b20_runs_the_base_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0102_b3f0);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0051_9b20, &[this, 0x25]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x25);
    }
}
