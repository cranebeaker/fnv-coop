//! `BSMain/run_00c3a180` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,Script*>::RemoveAll` (`004ed900`,
/// engine map; `thiscall`).
const SCRIPT_LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// The counted-pointer list's `RemoveAll` (`00c3a2e0`, `thiscall`).
const COUNTED_LIST_REMOVE_ALL: u32 = 0x00c3_a2e0;

// Translated from 00c3a180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3a180 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2daf8, state variable) is not translated
/// A list destructor (unnamed in the engine map; `thiscall`, plain `RET`): empties the
/// `Script*` pointer list at `this` (`004ed900`).
pub fn fn_00c3a180(e: &mut Engine, this: Ptr) {
    e.call(SCRIPT_LIST_REMOVE_ALL, &args![this]);
}

// Translated from 00c3a1d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3a1d0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fb98, state variable) is not translated
/// A list destructor (unnamed in the engine map; `thiscall`, plain `RET`): empties the list
/// of counted pointers at `this` (`00c3a2e0`).
pub fn fn_00c3a1d0(e: &mut Engine, this: Ptr) {
    e.call(COUNTED_LIST_REMOVE_ALL, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_a180, fn_00c3a180(Ptr)),
        entry!(0x00c3_a1d0, fn_00c3a1d0(Ptr)),
    ]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn both_destructors_empty_their_list() {
        let mut e = Engine::new();
        e.register(SCRIPT_LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(COUNTED_LIST_REMOVE_ALL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_a180, &args![0x100u32]);
        e.call(0x00c3_a1d0, &args![0x200u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(SCRIPT_LIST_REMOVE_ALL, vec![0x100])));
        assert!(log.contains(&(COUNTED_LIST_REMOVE_ALL, vec![0x200])));
    }
}
