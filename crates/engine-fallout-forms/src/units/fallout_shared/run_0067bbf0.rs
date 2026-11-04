//! `fallout shared/run_0067bbf0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESEscortPackageData`'s vtable (RTTI `.?AVTESEscortPackageData@@`).
const TES_ESCORT_PACKAGE_DATA_VTABLE: u32 = 0x0106_8960;

// Translated from 0067bbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEscortPackageData` default constructor (the class of the vtable
/// `01068960` it stores, by its RTTI): the `TESPackageData` constructor
/// (`0067ad20`), the vtable, the word +0x4 = 0 and +0x8 = 200. Returns
/// `this`.
pub fn fn_0067bbf0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0067_ad20, &args![this]);
    e.mem.set_u32(this.addr(), TES_ESCORT_PACKAGE_DATA_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 200);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_bbf0, fn_0067bbf0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_0067bbf0_builds_the_escort_package_data() {
        let mut e = Engine::new();
        e.register(0x0067_ad20, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcc; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_bbf0, &args![this]).u32(), this);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x0067_ad20, vec![this])));
        assert_eq!(e.mem.u32(this), TES_ESCORT_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 200);
        assert_eq!(e.mem.u32(this + 0xc), 0xcccc_cccc);
    }
}
