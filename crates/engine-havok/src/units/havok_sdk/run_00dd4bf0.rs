//! `Havok SDK/run_00dd4bf0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The two initializers of the sub-records at `this + 8` (`00dd50f0`, `thiscall`, `RET 4`: a
/// mode word) and `this + 0x20` (`00dd5150`, `thiscall`, `RET 4`: a word it ignores).
const INIT_FIRST: u32 = 0x00dd_50f0;
const INIT_SECOND: u32 = 0x00dd_5150;
/// `float`s the constructor stores: 0.5 (`01016248`), the one at `0101df44`, the one at
/// `010181bc` and the exe's zero (`01011d78`).
const HALF: u32 = 0x0101_6248;
const SETTING_A: u32 = 0x0101_df44;
const SETTING_B: u32 = 0x0101_81bc;
const ZERO: u32 = 0x0101_1d78;

// Translated from 00dd4bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `mode`): a constructor that sets
/// defaults. Initializes the sub-records at `this + 8` (`00dd50f0` with `mode`) and `this + 0x20`
/// (`00dd5150`), then stores the `float`s at `+0x40` (0.5 from `01016248`), `+0x44` (the one at
/// `0101df44`) and `+0x48` (1.0), all through the x87 stack; the word at `+0x60` becomes 5; the
/// vector at `+0x50` becomes `[a, a, b, zero]` with `a` the `float` at `0101df44`, `b` the one at
/// `010181bc` and the zero at `01011d78` (raw bits, `MOVSS`/`UNPCKLPS`); the byte at `+0x64`
/// becomes 1 and the word at `+4` 0. Answers `this`.
pub fn fn_00dd4bf0(e: &mut Engine, this: u32, mode: u32) -> u32 {
    e.call(INIT_FIRST, &args![this + 8, mode]);
    e.call(INIT_SECOND, &args![this + 0x20, mode]);
    let half = x87_quiet_bits(e.mem.u32(HALF));
    e.mem.set_u32(this + 0x40, half);
    let setting_a = e.mem.u32(SETTING_A);
    let setting_b = e.mem.u32(SETTING_B);
    let zero = e.mem.u32(ZERO);
    e.mem.set_u32(this + 0x44, x87_quiet_bits(setting_a));
    e.mem.set_u32(this + 0x48, 1.0f32.to_bits());
    e.mem.set_u32(this + 0x60, 5);
    for (i, bits) in [setting_a, setting_a, setting_b, zero]
        .into_iter()
        .enumerate()
    {
        e.mem.set_u32(this + 0x50 + 4 * i as u32, bits);
    }
    e.mem.set_u8(this + 0x64, 1);
    e.mem.set_u32(this + 4, 0);
    this
}

// Translated from 00dd4c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `source`). Copies a settings block:
/// the `float`s at `source + 0`, `+4`, `+8` to `this + 0x40`, `+0x44`, `+0x48` (through the x87
/// stack, so a signalling NaN comes out quiet), the 16-byte vector at `source + 0x10` to `this +
/// 0x50`, the word at `source + 0x20` to `this + 0x60` and the byte at `source + 0x24` to
/// `this + 0x64`.
pub fn fn_00dd4c70(e: &mut Engine, this: u32, source: u32) {
    for i in 0..3 {
        let bits = e.mem.u32(source + 4 * i);
        e.mem.set_u32(this + 0x40 + 4 * i, x87_quiet_bits(bits));
    }
    for i in 0..4 {
        let bits = e.mem.u32(source + 0x10 + 4 * i);
        e.mem.set_u32(this + 0x50 + 4 * i, bits);
    }
    let word = e.mem.u32(source + 0x20);
    e.mem.set_u32(this + 0x60, word);
    let byte = e.mem.u8(source + 0x24);
    e.mem.set_u8(this + 0x64, byte);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd4bf0, fn_00dd4bf0(u32, u32) -> u32),
        entry!(0x00dd4c70, fn_00dd4c70(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn the_constructor_initializes_both_sub_records_and_sets_the_defaults() {
        let mut e = Engine::new();
        for page in [0x0101_6000, 0x0101_d000, 0x0101_8000, 0x0101_1000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_f32(SETTING_A, 0.2);
        e.mem.set_f32(SETTING_B, 0.05);
        e.mem.set_f32(ZERO, 0.0);
        let log: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::default();
        for address in [INIT_FIRST, INIT_SECOND] {
            let seen = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                seen.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 4, 99);
        assert_eq!(e.call(0x00dd_4bf0, &args![this, 7u32]).u32(), this);
        assert_eq!(
            *log.borrow(),
            vec![
                (INIT_FIRST, vec![this + 8, 7]),
                (INIT_SECOND, vec![this + 0x20, 7])
            ]
        );
        assert_eq!(e.mem.f32(this + 0x40), 0.5);
        assert_eq!(e.mem.f32(this + 0x44), 0.2);
        assert_eq!(e.mem.f32(this + 0x48), 1.0);
        assert_eq!(e.mem.u32(this + 0x60), 5);
        let vector: Vec<f32> = (0..4).map(|i| e.mem.f32(this + 0x50 + 4 * i)).collect();
        assert_eq!(vector, vec![0.2, 0.2, 0.05, 0.0]);
        assert_eq!(e.mem.u8(this + 0x64), 1);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn the_settings_block_is_copied_with_the_floats_quieted() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x30);
        e.mem.set_u32(source, 0x7f80_0001);
        e.mem.set_f32(source + 4, 2.0);
        e.mem.set_f32(source + 8, 3.0);
        for i in 0..4 {
            e.mem.set_u32(source + 0x10 + 4 * i, 0x7f80_0100 + i);
        }
        e.mem.set_u32(source + 0x20, 0xabcd);
        e.mem.set_u8(source + 0x24, 9);
        let this = e.mem.alloc(0x70);
        e.call(0x00dd_4c70, &args![this, source]);
        assert_eq!(e.mem.u32(this + 0x40), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x44), 2.0);
        assert_eq!(e.mem.f32(this + 0x48), 3.0);
        // The vector is copied bit for bit.
        assert_eq!(e.mem.u32(this + 0x50), 0x7f80_0100);
        assert_eq!(e.mem.u32(this + 0x5c), 0x7f80_0103);
        assert_eq!(e.mem.u32(this + 0x60), 0xabcd);
        assert_eq!(e.mem.u8(this + 0x64), 9);
    }
}
