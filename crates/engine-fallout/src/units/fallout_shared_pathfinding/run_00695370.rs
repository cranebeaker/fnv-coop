//! `fallout shared/pathfinding/run_00695370` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00695370` calls on the member (`0044ddc0`).
const CALLEE_0044DDC0: u32 = 0x0044_ddc0;

// Translated from 00695370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044ddc0` with `ECX` = `this + 0x48` and passes on its result.
pub fn fn_00695370(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0044DDC0, &args![this.byte_add(0x48)]).u32()
}

/// `00403d30` (cdecl: three words): passes them on to `_memset` (`00ec61c0`)
/// and returns its result.
const MEMSET_FORWARD: u32 = 0x0040_3d30;

// Translated from 00695390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes its three words (cdecl) on to `00403d30`, which hands them to
/// `_memset`, and returns that result. (Ghidra's library signature names
/// both functions `std::_Fiopen`; the bodies are the `memset` forwarders.)
pub fn fn_00695390(e: &mut Engine, destination: u32, value: u32, size: u32) -> u32 {
    e.call(MEMSET_FORWARD, &args![destination, value, size])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069_5370, fn_00695370(Ptr) -> u32),
        entry!(0x0069_5390, fn_00695390(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00695370_passes_this_plus_48_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044DDC0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_5370, &args![0x4100u32]).u32(), 0x141d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044DDC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x48]);
    }

    #[test]
    fn at_00695390_forwards_three_words() {
        let mut e = Engine::new();
        e.register(MEMSET_FORWARD, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0069_5390, &args![0x5000u32, 7u32, 0x20u32]).u32(),
            0x5000
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == MEMSET_FORWARD).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x5000, 7, 0x20]);
    }
}
