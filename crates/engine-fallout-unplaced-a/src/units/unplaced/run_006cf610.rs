//! `(unplaced)/run_006cf610` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSScrapArray<bool, 1024>`'s vtable (`0106c990`; RTTI
/// `.?AV?$BSScrapArray@_N$0EAA@@@`).
const VTABLE_BS_SCRAP_ARRAY_BOOL: u32 = 0x0106_c990;
/// `008454f0`: `thiscall`, one word (1 here): releases the array's storage.
const RELEASE_008454F0: u32 = 0x0084_54f0;
/// `0069b1d0`: the base destructor (`thiscall`).
const BASE_DESTRUCTOR: u32 = 0x0069_b1d0;

// Translated from 006cf610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cf610 - the C++ exception frame (FS:[0]) is not translated
/// The `BSScrapArray<bool, 1024>` destructor (by the vtable it sets):
/// `008454f0(1)`, then the base destructor (`0069b1d0`).
pub fn fn_006cf610(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_BS_SCRAP_ARRAY_BOOL);
    e.call(RELEASE_008454F0, &args![this, 1u32]);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_f610, fn_006cf610(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_releases_then_chains() {
        let mut e = Engine::new();
        e.register(RELEASE_008454F0, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006c_f610, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_BS_SCRAP_ARRAY_BOOL);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (RELEASE_008454F0, vec![this, 1]),
                (BASE_DESTRUCTOR, vec![this])
            ]
        );
    }
}
