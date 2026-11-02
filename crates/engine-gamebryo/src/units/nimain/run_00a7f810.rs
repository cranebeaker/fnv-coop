//! `NiMain/run_00a7f810` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The virtual table `00a7f810` stores (`0109df2c`) and the global it keeps the object in
/// (`011f4ab4`).
const VTABLE_0109DF2C: u32 = 0x0109_df2c;
const INSTANCE_011F4AB4: u32 = 0x011f_4ab4;

// Translated from 00a7f810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a7f810`): a base constructor, `thiscall`, no stack words:
/// stores the virtual table `0109df2c` and keeps `this` at `011f4ab4`. Answers `this`.
pub fn fn_00a7f810(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_0109DF2C);
    e.mem.set_u32(INSTANCE_011F4AB4, this.addr());
    this
}

// Translated from 00a7f820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the matching destructor (`thiscall`, no stack
/// words): stores the vtable `0109df2c` again and clears `011f4ab4`.
pub fn fn_00a7f820(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_0109DF2C);
    e.set_global::<u32>(INSTANCE_011F4AB4, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_f810, fn_00a7f810(Ptr) -> Ptr),
        entry!(0x00a7_f820, fn_00a7f820(Ptr)),
    ]
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00a7f810_stores_the_table_and_keeps_the_object() {
        let mut e = Engine::new();
        e.map(INSTANCE_011F4AB4, 4);
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00a7_f810, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_0109DF2C);
        assert_eq!(e.mem.u32(INSTANCE_011F4AB4), this);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a7f810_stores_the_vtable_and_the_instance() {
        let mut e = Engine::new();
        e.map(INSTANCE_011F4AB4, 4);
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00a7_f810, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_0109DF2C);
        assert_eq!(e.global::<u32>(INSTANCE_011F4AB4), this);
    }

    #[test]
    fn fn_00a7f820_stores_the_vtable_and_clears_the_instance() {
        let mut e = Engine::new();
        e.map(INSTANCE_011F4AB4, 4);
        e.set_global::<u32>(INSTANCE_011F4AB4, 0x1234);
        let this = e.mem.alloc(8);
        e.call(0x00a7_f820, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_0109DF2C);
        assert_eq!(e.global::<u32>(INSTANCE_011F4AB4), 0);
    }
}
