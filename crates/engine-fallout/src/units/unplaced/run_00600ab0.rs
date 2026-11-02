//! `(unplaced)/run_00600ab0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00600ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `this + 0x14`. The engine map has no name
/// for it; the condition item loader (`00681950`) calls it with 1 or 2 on a
/// condition, where `+0x14` is `CONDITION_ITEM_DATA::eObject` (Xbox PDB).
pub fn fn_00600ab0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x14, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00600ab0, fn_00600ab0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_word_at_0x14() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x18);
        e.call(0x0060_0ab0, &[object, 0xabcd_1234]);
        assert_eq!(e.mem.u32(object + 0x14), 0xabcd_1234);
        assert_eq!(e.mem.u32(object + 0x10), 0);
    }
}
