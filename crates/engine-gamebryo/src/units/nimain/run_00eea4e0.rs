//! `NiMain/run_00eea4e0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00eea4e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall`, no stack
/// words): the 16-bit word at +0x18, in `AX` (the upper half of `EAX` is left as the caller
/// had it, so only the 16 bits are the answer).
pub fn fn_00eea4e0(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x18)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ee_a4e0, fn_00eea4e0(Ptr) -> u16)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_word_at_0x18() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 0x1234_abcd);
        assert_eq!(e.call(0x00ee_a4e0, &args![this]).u16(), 0xabcd);
    }
}
