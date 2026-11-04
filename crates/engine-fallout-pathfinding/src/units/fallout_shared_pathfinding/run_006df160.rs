//! `fallout shared/pathfinding/run_006df160` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006df160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of an array's first element (no name in the engine map):
/// `00877a30` on `this` with index 0, whose `EAX` the callers read.
pub fn fn_006df160(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0087_7a30, &args![this, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006d_f160, fn_006df160(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006df160_asks_for_element_zero() {
        let mut e = Engine::new();
        e.register(0x0087_7a30, |_, a| Ret {
            eax: a[0] + 0x100 + a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006d_f160, &args![0x5000u32]).u32(), 0x5100);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0087_7a30).collect();
        assert_eq!(calls[0].1, vec![0x5000, 0]);
    }
}
