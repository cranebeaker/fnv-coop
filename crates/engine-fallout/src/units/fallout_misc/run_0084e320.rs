//! `fallout/misc/run_0084e320` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006a7ad0` (`thiscall`, `RET 4`; engine-map unit `navmeshsearch.cpp`):
/// answers the address of an element.
const ELEMENT_ADDRESS_006A7AD0: u32 = 0x006a_7ad0;

// Translated from 0084e320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the word at the
/// address `006a7ad0` answers for `index`.
pub fn fn_0084e320(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let address = e.call(ELEMENT_ADDRESS_006A7AD0, &args![this, index]).u32();
    e.mem.u32(address)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_e320, fn_0084e320(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn fn_0084e320_reads_the_element() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x1234);
        e.register(ELEMENT_ADDRESS_006A7AD0, |_, a| {
            (a[0] + 4 * a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_e320, &args![this, 2u32]).u32(), 0x1234);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ELEMENT_ADDRESS_006A7AD0, vec![this, 2])));
    }
}
