//! `fallout shared/run_0067b950` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESEatPackageData`'s vtable (RTTI `.?AVTESEatPackageData@@`).
const TES_EAT_PACKAGE_DATA_VTABLE: u32 = 0x0106_8938;

// Translated from 0067b950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEatPackageData` constructor (the class of the vtable `01068938` it
/// stores, by its RTTI): the `TESPackageData` constructor (`0067ad20`), the
/// vtable and the word +0x4 = 0. Returns `this`.
pub fn fn_0067b950(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0067_ad20, &args![this]);
    e.mem.set_u32(this.addr(), TES_EAT_PACKAGE_DATA_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_b950, fn_0067b950(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_0067b950_builds_the_eat_package_data() {
        let mut e = Engine::new();
        e.register(0x0067_ad20, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.write(this, &[0xcc; 0xc]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_b950, &args![this]).u32(), this);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x0067_ad20, vec![this])));
        assert_eq!(e.mem.u32(this), TES_EAT_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0xcccc_cccc);
    }
}
