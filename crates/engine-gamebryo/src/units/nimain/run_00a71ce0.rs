//! `NiMain/run_00a71ce0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The game's `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The base constructor `00a89740` (`thiscall`, no engine-map name).
const BASE_CONSTRUCT: u32 = 0x00a8_9740;
/// The `NiTriStrips` vtable (RTTI).
const TRI_STRIPS_VTABLE: u32 = 0x0109_cd44;

// Translated from 00a71ce0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function, no engine-map name; by its body the creation function of
/// `NiTriStrips`, the RTTI of the vtable it stores; no arguments): a new 0xc4-byte block
/// from `operator new`; null when that fails. Otherwise the base constructor `00a89740`
/// and the `NiTriStrips` vtable. Answers the block.
pub fn fn_00a71ce0(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, TRI_STRIPS_VTABLE);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_1ce0, fn_00a71ce0() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a71ce0_builds_tri_strips() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let block = e.call(0x00a7_1ce0, &[]).u32();
        assert_ne!(block, 0);
        assert_eq!(e.mem.u32(block), TRI_STRIPS_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(OPERATOR_NEW, vec![0xc4])));
        assert!(log.contains(&(BASE_CONSTRUCT, vec![block])));

        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_1ce0, &[]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}
