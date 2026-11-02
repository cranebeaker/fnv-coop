//! `(unplaced)/run_00559490` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00559490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the 16-bit `value` at `this + 0xE` (thiscall, one stack word of
/// which the low half is used).
pub fn fn_00559490(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr().wrapping_add(0xe), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0055_9490, fn_00559490(Ptr, u16))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_low_half_word_at_0xe() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xaaaa_aaaa);
        e.call(0x0055_9490, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x5678_aaaa);
    }
}
