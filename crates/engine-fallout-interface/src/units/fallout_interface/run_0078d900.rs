//! `fallout/interface/run_0078d900` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0078d900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a constructor that clears a 0xc-byte record: word `+0`, the
/// float `0.0` at `+4` and word `+8`. Returns `this`.
pub fn fn_0078d900(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_f32(base + 4, 0.0);
    e.mem.set_u32(base + 8, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0078_d900, fn_0078d900(Ptr) -> Ptr)]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    #[test]
    fn clears_the_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for i in 0..4u32 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0078_d900, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0xffff_ffff);
    }
}
