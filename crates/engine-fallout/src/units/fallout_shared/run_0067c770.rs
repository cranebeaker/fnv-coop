//! `fallout shared/run_0067c770` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESUseItemPackageData`'s vtable (RTTI `.?AVTESUseItemPackageData@@`).
const TES_USE_ITEM_PACKAGE_DATA_VTABLE: u32 = 0x0106_8a0c;

// Translated from 0067c770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseItemPackageData` constructor (the class of the vtable `01068a0c`
/// it stores, by its RTTI): the `TESPackageData` constructor (`0067ad20`),
/// the vtable and the word +0x4 = 0. Returns `this`.
pub fn fn_0067c770(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0067_ad20, &args![this]);
    e.mem.set_u32(this.addr(), TES_USE_ITEM_PACKAGE_DATA_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_c770, fn_0067c770(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_0067c770_builds_the_use_item_package_data() {
        let mut e = Engine::new();
        e.register(0x0067_ad20, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.write(this, &[0xcc; 0xc]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_c770, &args![this]).u32(), this);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x0067_ad20, vec![this])));
        assert_eq!(e.mem.u32(this), TES_USE_ITEM_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0xcccc_cccc);
    }
}
