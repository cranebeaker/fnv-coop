//! `BSMain/run_00c5bb10` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004a4650` (`thiscall`): the body this destructor runs on `this`.
const DESTRUCTOR_BODY: u32 = 0x004a_4650;

// Translated from 00c5bb10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5bb10 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fb98, state variable) is not translated
/// A destructor (unnamed in the engine map; `thiscall`, plain `RET`): runs `004a4650` on
/// `this`.
pub fn fn_00c5bb10(e: &mut Engine, this: Ptr) {
    e.call(DESTRUCTOR_BODY, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c5_bb10, fn_00c5bb10(Ptr))]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn runs_the_body_on_this() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_BODY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_bb10, &args![0x100u32]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(DESTRUCTOR_BODY, vec![0x100])));
    }
}
