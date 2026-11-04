//! `fallout/ai/run_009d7c60` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d7c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the byte at `+0xe1` (returned in `AL`).
pub fn fn_009d7c60(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xe1)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009d_7c60, fn_009d7c60(Ptr) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_009d7c60_reads_the_byte_at_0xe1() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xe4);
        e.mem.set_u8(this + 0xe1, 0x5a);
        e.mem.set_u8(this + 0xe0, 0xff);
        assert_eq!(e.call(0x009d_7c60, &args![this]).u8(), 0x5a);
    }
}
