//! `(unplaced)/run_00c90350` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c90350 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. A virtual method (it is only referenced from vtables) that
/// ignores `this` and returns 0.
pub fn fn_00c90350(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00c90330 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No map name and no function in the Ghidra project (bytes disassembled: a lone `RET`,
/// then `INT3` padding); nothing references it. It reads no register and no stack word,
/// so it takes nothing and does nothing.
pub fn fn_00c90330(_e: &mut Engine) {}

// Translated from 00c90340 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No map name and no function in the Ghidra project (bytes disassembled: a lone `RET`,
/// then `INT3` padding); nothing references it. It takes nothing and does nothing.
pub fn fn_00c90340(_e: &mut Engine) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c90350, fn_00c90350(Ptr) -> u32),
        entry!(0x00c9_0330, fn_00c90330()),
        entry!(0x00c9_0340, fn_00c90340()),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn empty_bodies_call_nothing() {
        let mut e = Engine::new();
        for addr in [0x00c9_0330u32, 0x00c9_0340] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![]), Ret::default());
            assert_eq!(e.call_log.take().unwrap().len(), 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c90350_returns_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c9_0350, &args![Ptr::<()>::new(0x10)]).u32(), 0);
    }
}
