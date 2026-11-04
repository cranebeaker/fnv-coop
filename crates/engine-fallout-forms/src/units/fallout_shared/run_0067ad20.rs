//! `fallout shared/run_0067ad20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESPackageData`'s vtable (RTTI `.?AVTESPackageData@@`).
pub(crate) const TES_PACKAGE_DATA_VTABLE: u32 = 0x0106_8884;

// Translated from 0067ad20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackageData` constructor (the class of the vtable `01068884` it
/// stores, by its RTTI): stores the vtable and returns `this`.
pub fn fn_0067ad20(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), TES_PACKAGE_DATA_VTABLE);
    this
}

// Translated from 0067ad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackageData` destructor body: stores the vtable `01068884` back.
pub fn fn_0067ad70(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_PACKAGE_DATA_VTABLE);
}

// Translated from 0067ad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackageData` scalar deleting destructor: runs the destructor body
/// (`fn_0067ad70`), then frees `this` with `00401030` when bit 0 of `flags`
/// is set. Returns `this` (`RET 4`).
pub fn fn_0067ad40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0067ad70(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_ad20, fn_0067ad20(Ptr) -> Ptr),
        entry!(0x0067_ad70, fn_0067ad70(Ptr)),
        entry!(0x0067_ad40, fn_0067ad40(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn the_constructor_and_destructor_store_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x1234);
        assert_eq!(e.call(0x0067_ad20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TES_PACKAGE_DATA_VTABLE);
        e.mem.set_u32(this, 0);
        e.call(0x0067_ad70, &args![this]);
        assert_eq!(e.mem.u32(this), TES_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x1234);
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn deleting_destructor_frees_with_flag_1() {
        let mut e = Engine::new();
        e.register(0x0040_1030, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_ad40, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), TES_PACKAGE_DATA_VTABLE);
        e.call(0x0067_ad40, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<_> = log.iter().filter(|c| c.0 == 0x0040_1030).collect();
        assert_eq!(freed, vec![&(0x0040_1030, vec![this])]);
    }
}
