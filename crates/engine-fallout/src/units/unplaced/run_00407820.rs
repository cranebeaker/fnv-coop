//! `(unplaced)/run_00407820` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Five field accessors of one object (its class is not confirmed): two
//! getters of the words at +0x78 and +0x7C, two float setters at +0x90 and
//! +0x94, and a word setter at +0x98.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00407820 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Returns the word at +0x78 of `this` (`thiscall`, no stack words).
pub fn fn_00407820(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x78)
}

// Translated from 00407840 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Returns the word at +0x7C of `this` (`thiscall`, no stack words).
pub fn fn_00407840(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x7c)
}

// Translated from 00407860 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores `value` as the float at +0x90 of `this` (`thiscall`, `RET 4`;
/// `FLD`/`FSTP`, so a signalling NaN is stored quiet).
pub fn fn_00407860(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x90, x87_quiet_bits(value.to_bits()));
}

// Translated from 00407880 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores `value` as the float at +0x94 of `this` (`thiscall`, `RET 4`;
/// `FLD`/`FSTP`, so a signalling NaN is stored quiet).
pub fn fn_00407880(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x94, x87_quiet_bits(value.to_bits()));
}

// Translated from 004078a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores the word `value` at +0x98 of `this` (`thiscall`, `RET 4`).
pub fn fn_004078a0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x98, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00407820, fn_00407820(Ptr) -> u32),
        entry!(0x00407840, fn_00407840(Ptr) -> u32),
        entry!(0x00407860, fn_00407860(Ptr, f32)),
        entry!(0x00407880, fn_00407880(Ptr, f32)),
        entry!(0x004078a0, fn_004078a0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getters_read_their_words() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0xa0);
        e.mem.set_u32(o + 0x78, 11);
        e.mem.set_u32(o + 0x7c, 22);
        assert_eq!(e.call(0x0040_7820, &[o]).u32(), 11);
        assert_eq!(e.call(0x0040_7840, &[o]).u32(), 22);
    }

    #[test]
    fn float_setters_store_quiet_floats() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0xa0);
        e.call(0x0040_7860, &args![o, 1.5f32]);
        assert_eq!(e.mem.u32(o + 0x90), 1.5f32.to_bits());
        e.call(0x0040_7860, &[o, 0x7f80_0001]);
        assert_eq!(e.mem.u32(o + 0x90), 0x7fc0_0001);
        e.call(0x0040_7880, &args![o, -2.0f32]);
        assert_eq!(e.mem.u32(o + 0x94), (-2.0f32).to_bits());
        e.call(0x0040_7880, &[o, 0xff80_0002]);
        assert_eq!(e.mem.u32(o + 0x94), 0xffc0_0002);
    }

    #[test]
    fn word_setter_stores_the_word() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0xa0);
        e.call(0x0040_78a0, &[o, 0x7f80_0001]);
        assert_eq!(e.mem.u32(o + 0x98), 0x7f80_0001);
    }
}
