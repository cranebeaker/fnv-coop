//! `Havok SDK/run_00dd50f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `float`s the defaults come from: `0101e2bc` (the default value), `0101df44` (mode 0) and
/// `010181bc` (mode 1), and 0.5 (`01016248`).
const DEFAULT_VALUE: u32 = 0x0101_e2bc;
const MODE_0_VALUE: u32 = 0x0101_df44;
const MODE_1_VALUE: u32 = 0x0101_81bc;
const HALF: u32 = 0x0101_6248;

// Translated from 00dd50f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `mode`). Sets a block of defaults: the
/// `float` at `+0` from `0101e2bc` (through the x87 stack), the words `+0x10` and `+0xc` to 5,
/// `+4` to 1000 and `+8` to 10, and the byte `+0x14` to 1. Mode 0 then replaces the `float` at
/// `+0` by the one at `0101df44`; mode 1 sets `+8` to 0x1e and the `float` at `+0` to the one at
/// `010181bc`; any other mode leaves the defaults.
pub fn fn_00dd50f0(e: &mut Engine, this: u32, mode: u32) {
    let default = x87_quiet_bits(e.mem.u32(DEFAULT_VALUE));
    e.mem.set_u32(this, default);
    e.mem.set_u32(this + 0x10, 5);
    e.mem.set_u32(this + 0xc, 5);
    e.mem.set_u32(this + 4, 0x3e8);
    e.mem.set_u32(this + 8, 0xa);
    e.mem.set_u8(this + 0x14, 1);
    if mode == 0 {
        let value = x87_quiet_bits(e.mem.u32(MODE_0_VALUE));
        e.mem.set_u32(this, value);
    } else if mode == 1 {
        let value = x87_quiet_bits(e.mem.u32(MODE_1_VALUE));
        e.mem.set_u32(this + 8, 0x1e);
        e.mem.set_u32(this, value);
    }
}

// Translated from 00dd5150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this` and a stack word the code never reads,
/// `_unused_0`). Sets five `float`s at `+0` to `+0x10` to 1.0 and the one at `+0x14` to the
/// `float` at `01016248` (through the x87 stack).
pub fn fn_00dd5150(e: &mut Engine, this: u32, _unused_0: u32) {
    for i in 0..5 {
        e.mem.set_u32(this + 4 * i, 1.0f32.to_bits());
    }
    let half = x87_quiet_bits(e.mem.u32(HALF));
    e.mem.set_u32(this + 0x14, half);
}

// Translated from 00dd5170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `source`). Copies the six words at
/// `source` to `this + 8` through `this + 0x1c`.
pub fn fn_00dd5170(e: &mut Engine, this: u32, source: u32) {
    for i in 0..6 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(this + 8 + 4 * i, word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd50f0, fn_00dd50f0(u32, u32)),
        entry!(0x00dd5150, fn_00dd5150(u32, u32)),
        entry!(0x00dd5170, fn_00dd5170(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_e000, 0x0101_d000, 0x0101_8000, 0x0101_6000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(DEFAULT_VALUE, 0.1);
        e.mem.set_f32(MODE_0_VALUE, 0.2);
        e.mem.set_f32(MODE_1_VALUE, 0.05);
        e.mem.set_f32(HALF, 0.5);
        e
    }

    fn defaults(e: &mut Engine, mode: u32) -> (f32, Vec<u32>, u8) {
        let this = e.mem.alloc(0x20);
        e.call(0x00dd_50f0, &args![this, mode]);
        let words = [4u32, 8, 0xc, 0x10].map(|o| e.mem.u32(this + o)).to_vec();
        (e.mem.f32(this), words, e.mem.u8(this + 0x14))
    }

    #[test]
    fn the_mode_picks_the_first_float_and_mode_one_the_third_word() {
        let mut e = engine();
        assert_eq!(defaults(&mut e, 0), (0.2, vec![1000, 10, 5, 5], 1));
        assert_eq!(defaults(&mut e, 1), (0.05, vec![1000, 0x1e, 5, 5], 1));
        assert_eq!(defaults(&mut e, 2), (0.1, vec![1000, 10, 5, 5], 1));
    }

    #[test]
    fn the_unit_defaults_and_the_word_copy() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.call(0x00dd_5150, &args![this, 0xdeadu32]);
        let floats: Vec<f32> = (0..6).map(|i| e.mem.f32(this + 4 * i)).collect();
        assert_eq!(floats, vec![1.0, 1.0, 1.0, 1.0, 1.0, 0.5]);
        let source = e.mem.alloc(0x20);
        for i in 0..6 {
            e.mem.set_u32(source + 4 * i, 0x30 + i);
        }
        e.call(0x00dd_5170, &args![this, source]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 8 + 4 * i)).collect();
        assert_eq!(words, vec![0x30, 0x31, 0x32, 0x33, 0x34, 0x35]);
    }
}
