//! `fallout shared/run_004fd3a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fd3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): the word at +0x74.
pub fn fn_004fd3a0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x74)
}

// Translated from 004fd3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): the word at +0x84.
pub fn fn_004fd3c0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x84)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_d3a0, fn_004fd3a0(Ptr) -> u32),
        entry!(0x004f_d3c0, fn_004fd3c0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004fd3a0_and_fn_004fd3c0_read_their_words() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x90);
        e.mem.set_u32(object + 0x74, 0x1111_2222);
        e.mem.set_u32(object + 0x84, 0x3333_4444);
        assert_eq!(e.call(0x004f_d3a0, &args![object]).u32(), 0x1111_2222);
        assert_eq!(e.call(0x004f_d3c0, &args![object]).u32(), 0x3333_4444);
    }
}
