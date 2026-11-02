//! `fallout/misc/run_00847d00` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The flag word the function changes (+0x244 of `this`).
const FLAGS: u32 = 0x244;
/// The bit it sets or clears.
const FLAG_BIT: u32 = 4;

// Translated from 00847d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (when `on` is non-zero) or clears bit 2 (value 4) of the flag word
/// at +0x244 of `this` (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_00847d00(e: &mut Engine, this: Ptr, on: u8) {
    let address = this.addr() + FLAGS;
    let flags = e.mem.u32(address);
    if on != 0 {
        e.mem.set_u32(address, flags | FLAG_BIT);
    } else {
        e.mem.set_u32(address, flags & !FLAG_BIT);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_7d00, fn_00847d00(Ptr, u8))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_or_clears_bit_two_of_the_flag_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x248);
        e.mem.set_u32(this + FLAGS, 0x8000_0001);
        // Only the low byte of the argument word counts.
        e.call(0x0084_7d00, &args![this, 0x0000_0101u32]);
        assert_eq!(e.mem.u32(this + FLAGS), 0x8000_0005);
        e.call(0x0084_7d00, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + FLAGS), 0x8000_0005);
        e.call(0x0084_7d00, &args![this, 0x0000_0100u32]);
        assert_eq!(e.mem.u32(this + FLAGS), 0x8000_0001);
        e.mem.set_u32(this + FLAGS, 0xffff_ffff);
        e.call(0x0084_7d00, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + FLAGS), 0xffff_fffb);
    }
}
