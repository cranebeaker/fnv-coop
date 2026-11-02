//! `NiMain/nirtti.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a7b610 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiRTTI::NiRTTI` (Xbox PDB; `thiscall`, `RET 8`): the class name at +0
/// and the base class's `NiRTTI` at +4. Returns `this` (`EAX`).
pub fn ni_rtti_ni_rtti(e: &mut Engine, this: Ptr, name: u32, base: u32) -> Ptr {
    e.mem.set_u32(this.addr(), name);
    e.mem.set_u32(this.addr() + 4, base);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7b610, ni_rtti_ni_rtti(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_name_and_base() {
        let mut e = Engine::new();
        let r = e.mem.alloc(8);
        assert_eq!(e.call(0x00a7b610, &[r, 0x0105_0000, 0x011f_0000]).u32(), r);
        assert_eq!(e.mem.u32(r), 0x0105_0000);
        assert_eq!(e.mem.u32(r + 4), 0x011f_0000);
    }
}
