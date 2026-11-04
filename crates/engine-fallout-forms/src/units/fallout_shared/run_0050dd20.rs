//! `fallout shared/run_0050dd20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004bc2c0` (`thiscall`, unnamed in the engine map): run after either
/// setter below changes the object.
const AFTER_CHANGE: u32 = 0x004b_c2c0;

// Translated from 0050dd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores `value` as the
/// float at `this + 0xc4` (an `FLD`/`FSTP` copy, so a signalling NaN comes out
/// quiet) and calls `004bc2c0` on `this`.
pub fn fn_0050dd20(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0xc4),
        x87_quiet_bits(value.to_bits()),
    );
    e.call(AFTER_CHANGE, &args![this]);
}

// Translated from 0050dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): copies the three words
/// at `color` to `this + 0xe0` and calls `004bc2c0` on `this`.
pub fn fn_0050dd50(e: &mut Engine, this: Ptr, color: Ptr) {
    let to = this.addr().wrapping_add(0xe0);
    for i in 0..3 {
        let word = e.mem.u32(color.addr().wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
    e.call(AFTER_CHANGE, &args![this]);
}

// Translated from 0050dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): whether bit 0 of the
/// word at `this + 0xa8` is set.
pub fn fn_0050dd90(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0xa8)) & 1 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_dd20, fn_0050dd20(Ptr, f32)),
        entry!(0x0050_dd50, fn_0050dd50(Ptr, Ptr)),
        entry!(0x0050_dd90, fn_0050dd90(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn float_setter_stores_a_quiet_copy_then_notifies() {
        let mut e = Engine::new();
        e.register(AFTER_CHANGE, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x0050_dd20, &[this, 0.75f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0xc4), 0.75);
        e.call(0x0050_dd20, &[this, 0x7f80_0001]);
        assert_eq!(e.mem.u32(this + 0xc4), 0x7fc0_0001);
        assert_eq!(calls_to(&e, AFTER_CHANGE), vec![vec![this], vec![this]]);
    }

    #[test]
    fn color_setter_copies_three_words_then_notifies() {
        let mut e = Engine::new();
        e.register(AFTER_CHANGE, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        let color = e.mem.alloc(0xc);
        for (i, w) in [0x7f80_0001u32, 2, 3].iter().enumerate() {
            e.mem.set_u32(color + 4 * i as u32, *w);
        }
        e.call_log = Some(vec![]);
        e.call(0x0050_dd50, &[this, color]);
        assert_eq!(e.mem.u32(this + 0xe0), 0x7f80_0001);
        assert_eq!(e.mem.u32(this + 0xe4), 2);
        assert_eq!(e.mem.u32(this + 0xe8), 3);
        assert_eq!(calls_to(&e, AFTER_CHANGE), vec![vec![this]]);
    }

    #[test]
    fn bit_0_test_reads_the_word_at_a8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0xa8, 0xffff_fffe);
        assert!(!e.call(0x0050_dd90, &[this]).bool());
        e.mem.set_u32(this + 0xa8, 0x8000_0001);
        assert!(e.call(0x0050_dd90, &[this]).bool());
    }
}
