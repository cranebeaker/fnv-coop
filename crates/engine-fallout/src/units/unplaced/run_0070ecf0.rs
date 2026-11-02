//! `(unplaced)/run_0070ecf0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte global `0070ed10` answers.
const FLAG_011DEA2F: u32 = 0x011d_ea2f;

// Translated from 0070ed10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the byte at `011dea2f` (in
/// `AL`).
pub fn fn_0070ed10(e: &mut Engine) -> u8 {
    e.mem.u8(FLAG_011DEA2F)
}

// Translated from 0070ed20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): stores its low byte in `011dea2f` (the byte
/// `0070ed10` answers).
pub fn fn_0070ed20(e: &mut Engine, value: u8) {
    e.mem.set_u8(FLAG_011DEA2F, value);
}

// Translated from 0070ecf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): whether bit 2 (0x4) of the word at `this + 0x14c`
/// is set (`NEG`/`SBB`/`NEG`: `EAX` is exactly 0 or 1).
pub fn fn_0070ecf0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0x14c)) & 4 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070ed10, fn_0070ed10() -> u8),
        entry!(0x0070_ed20, fn_0070ed20(u8)),
        entry!(0x0070_ecf0, fn_0070ecf0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_byte() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u8(FLAG_011DEA2F, 0x42);
        assert_eq!(e.call(0x0070_ed10, &[]).u8(), 0x42);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_0070ed20_stores_the_low_byte() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(FLAG_011DEA2F - 1, 0);
        e.call(0x0070_ed20, &args![0x1234_5601u32]);
        assert_eq!(e.mem.u8(FLAG_011DEA2F), 1);
        // Only the one byte is written.
        assert_eq!(e.mem.u8(FLAG_011DEA2F - 1), 0);
        assert_eq!(e.mem.u8(FLAG_011DEA2F + 1), 0);
        e.call(0x0070_ed20, &args![0u32]);
        assert_eq!(e.mem.u8(FLAG_011DEA2F), 0);
    }

    #[test]
    fn fn_0070ecf0_tests_bit_2_at_0x14c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x150);
        e.mem.set_u32(this + 0x14c, !4u32);
        assert_eq!(e.call(0x0070_ecf0, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x14c, 4);
        assert_eq!(e.call(0x0070_ecf0, &args![this]).u32(), 1);
    }
}
