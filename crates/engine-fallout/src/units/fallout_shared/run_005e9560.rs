//! `fallout shared/run_005e9560` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e9590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BGSPerkEntry` (the RTTI name of the vtable `0x0104_6b48` it stores;
/// unnamed in the engine map; `ECX` only, plain `RET`): stores that vtable, then runs
/// `00483710` on the object.
pub fn fn_005e9590(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_6b48);
    e.call(0x0048_3710, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_9590, fn_005e9590(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_005e9590_stores_the_vtable_and_runs_the_base() {
        let mut e = Engine::new();
        let entry = e.mem.alloc(0x10);
        e.register(0x0048_3710, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005e_9590, &args![entry]);
        assert_eq!(e.mem.u32(entry), 0x0104_6b48);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0048_3710, vec![entry])]);
    }
}
