//! `fallout shared/run_0067e3a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `PackageList::FreePackageList` (`0067e400`, engine map; `thiscall`).
const FREE_PACKAGE_LIST: u32 = 0x0067_e400;
/// `0046ffb0` (`thiscall`): runs `00470470` on `this` (a simple list's
/// clean-up).
const LIST_DESTROY_0046FFB0: u32 = 0x0046_ffb0;

// Translated from 0067e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067e3a0 - the C++ exception unwinding frame (FS:[0], handler 00eeaa58) is not translated
/// A package list's destructor shape (`thiscall`; no Xbox PDB name):
/// `PackageList::FreePackageList(this)`, then `0046ffb0(this)`. The
/// exception-unwinding frame is not translated.
pub fn fn_0067e3a0(e: &mut Engine, this: Ptr) {
    e.call(FREE_PACKAGE_LIST, &args![this]);
    e.call(LIST_DESTROY_0046FFB0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_e3a0, fn_0067e3a0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frees_then_destroys_the_list() {
        let mut e = Engine::new();
        e.register(FREE_PACKAGE_LIST, |_, _| Ret::default());
        e.register(LIST_DESTROY_0046FFB0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_e3a0, &args![0x500u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1..],
            [
                (FREE_PACKAGE_LIST, vec![0x500]),
                (LIST_DESTROY_0046FFB0, vec![0x500])
            ]
        );
    }
}
