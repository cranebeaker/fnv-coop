//! `Havok SDK/run_00e05100` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e05100 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: stores `value` at `this + 8`. The function takes two stack words
/// (`RET 8`); the second is never read. The class is unconfirmed.
pub fn fn_00e05100(e: &mut Engine, this: Ptr, value: u32, _unused_1: u32) {
    e.mem.set_u32(this.addr() + 8, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e0_5100, fn_00e05100(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The first stack word goes to +8, the second is ignored.
    #[test]
    fn fn_00e05100_stores_the_first_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0x77; 0x10]);
        e.call(0x00e0_5100, &args![this, 0x1234u32, 0x5678u32]);
        assert_eq!(e.mem.u32(this + 8), 0x1234);
        assert_eq!(e.mem.u32(this + 4), 0x7777_7777);
        assert_eq!(e.mem.u32(this + 12), 0x7777_7777);
    }
}
