//! `fallout/ai/combat/combattimer.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `FLT_MAX` (`0108e360`): times outside `(-FLT_MAX, FLT_MAX)` are saved
/// unchanged.
const FLT_MAX_CONSTANT: u32 = 0x0108_e360;
/// The `float` time `CombatTimer` values are saved relative to
/// (`011f1bf0`; not named in the exe's data).
const TIMER_TIME_BASE: u32 = 0x011f_1bf0;
/// The time `CombatTimeStamp` values are saved relative to (`00435dd0`,
/// `extradataobjects.cpp`; no stack words, the `float` in ST0).
const TIME_STAMP_TIME_BASE: u32 = 0x0043_5dd0;
/// `BGSSaveGameBuffer`'s byte save (`00865e50`, thiscall on the buffer:
/// source, size, a third word).
const SAVE_BYTES: u32 = 0x0086_5e50;

/// Whether the `float` at `bits` is strictly inside `(-FLT_MAX, FLT_MAX)`:
/// the two `FCOMPP`/`TEST AH,0x41` tests (equal and unordered fail).
fn is_finite_time(e: &Engine, bits: u32) -> bool {
    let max = x87_load32(e.global::<u32>(FLT_MAX_CONSTANT));
    let value = x87_load32(bits);
    max > value && value > -max
}

// Translated from 009a5f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTimeStamp::SaveGame` (Xbox PDB; thiscall, `RET 4`): saves the
/// stamp's `float` (4 bytes) into `buffer`, made relative to the time
/// `00435dd0` gives (`local - time`, x87) when it is strictly between
/// `-FLT_MAX` and `FLT_MAX`.
pub fn combat_time_stamp_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let bits = e.mem.u32(this.addr());
    let mut local = x87_quiet_bits(bits);
    if is_finite_time(e, bits) {
        let time = e.call(TIME_STAMP_TIME_BASE, &args![]).f64();
        local = x87_store32(x87_sub(x87_m32(local), time));
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), local);
        e.call(SAVE_BYTES, &args![buffer, slot, 4u32, 0u32]);
    });
}

// Translated from 009a5eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTimer::SaveGame` (Xbox PDB; thiscall, `RET 4`): saves the
/// timer's `float` (made relative to the `float` at `011f1bf0`,
/// `local - base`, when strictly between `-FLT_MAX` and `FLT_MAX`), then
/// the 4 bytes at +4, into `buffer`.
pub fn combat_timer_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let bits = e.mem.u32(this.addr());
    let mut local = x87_quiet_bits(bits);
    if is_finite_time(e, bits) {
        let base = e.global::<u32>(TIMER_TIME_BASE);
        local = x87_store32(x87_sub(x87_load32(local), x87_m32(base)));
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), local);
        e.call(SAVE_BYTES, &args![buffer, slot, 4u32, 0u32]);
    });
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 4, 4u32, 0u32]);
}

/// `BGSLoadGameBuffer`'s byte load (`00864980`, thiscall on the buffer:
/// destination, size).
const LOAD_BYTES: u32 = 0x0086_4980;

// Translated from 009a5f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTimer::LoadGame` (Xbox PDB; thiscall, `RET 4`): loads the
/// timer's `float` and the 4 bytes at +4 from `buffer`, then, when the
/// `float` is strictly between `-FLT_MAX` and `FLT_MAX`, makes it absolute
/// again (`local + base`, the `float` at `011f1bf0`, x87).
pub fn combat_timer_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(LOAD_BYTES, &args![buffer, this, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 4, 4u32]);
    let bits = e.mem.u32(this.addr());
    if is_finite_time(e, bits) {
        let base = e.global::<u32>(TIMER_TIME_BASE);
        let value = x87_store32(x87_add(x87_load32(bits), x87_m32(base)));
        e.mem.set_u32(this.addr(), value);
    }
}

// Translated from 009a5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTimeStamp::LoadGame` (Xbox PDB; thiscall, `RET 4`): loads the
/// stamp's `float` from `buffer`, then, when it is strictly between
/// `-FLT_MAX` and `FLT_MAX`, makes it absolute again (`time + local`, the
/// time from `00435dd0` in ST0, x87).
pub fn combat_time_stamp_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(LOAD_BYTES, &args![buffer, this, 4u32]);
    let bits = e.mem.u32(this.addr());
    if is_finite_time(e, bits) {
        let time = e.call(TIME_STAMP_TIME_BASE, &args![]).f64();
        let bits = e.mem.u32(this.addr());
        let value = x87_store32(x87_add(time, x87_m32(bits)));
        e.mem.set_u32(this.addr(), value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_5f20, combat_timer_load_game(Ptr, Ptr)),
        entry!(0x009a_5ff0, combat_time_stamp_load_game(Ptr, Ptr)),
        entry!(0x009a_5f90, combat_time_stamp_save_game(Ptr, Ptr)),
        entry!(0x009a_5eb0, combat_timer_save_game(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The byte-save double records each saved block's first word and
    /// size here.
    const RECORD: u32 = 0x0330_0000;

    fn setup(e: &mut Engine) {
        e.map(FLT_MAX_CONSTANT, 4);
        e.mem.set_u32(FLT_MAX_CONSTANT, f32::MAX.to_bits());
        e.map(TIMER_TIME_BASE, 4);
        e.mem.set_u32(TIMER_TIME_BASE, 2.5f32.to_bits());
        e.register(TIME_STAMP_TIME_BASE, |_, _| Ret {
            st0: 1.25,
            ..Ret::default()
        });
        e.map(RECORD, 0x40);
        e.register(SAVE_BYTES, |e, a| {
            assert_eq!(a[0], 0x0abc_0000);
            assert_eq!(a[2], 4);
            let used = e.mem.u32(RECORD);
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(RECORD + 4 + used * 4, word);
            e.mem.set_u32(RECORD, used + 1);
            Ret::default()
        });
    }

    fn saved(e: &Engine) -> Vec<u32> {
        (0..e.mem.u32(RECORD))
            .map(|i| e.mem.u32(RECORD + 4 + i * 4))
            .collect()
    }

    fn stamp_saves(value: u32) -> u32 {
        let mut e = Engine::new();
        setup(&mut e);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, value);
        e.call(0x009a_5f90, &args![this, 0x0abc_0000u32]);
        let words = saved(&e);
        assert_eq!(words.len(), 1);
        words[0]
    }

    #[test]
    fn time_stamp_saves_relative_finite_times_and_others_unchanged() {
        assert_eq!(stamp_saves(10.0f32.to_bits()), 8.75f32.to_bits());
        assert_eq!(stamp_saves(f32::MAX.to_bits()), f32::MAX.to_bits());
        assert_eq!(stamp_saves((-f32::MAX).to_bits()), (-f32::MAX).to_bits());
        assert_eq!(
            stamp_saves(f32::INFINITY.to_bits()),
            f32::INFINITY.to_bits()
        );
        // A signalling NaN is unordered (kept) and quieted by FLD/FSTP.
        assert_eq!(stamp_saves(0x7f80_0001), 0x7fc0_0001);
        // Just inside the range.
        let below = f32::from_bits(f32::MAX.to_bits() - 1);
        assert_eq!(
            stamp_saves(below.to_bits()),
            x87_store32(x87_sub(f64::from(below), 1.25))
        );
    }

    fn timer_saves(value: u32) -> Vec<u32> {
        let mut e = Engine::new();
        setup(&mut e);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, value);
        e.mem.set_u32(this + 4, 0x0102_0304);
        e.call(0x009a_5eb0, &args![this, 0x0abc_0000u32]);
        saved(&e)
    }

    #[test]
    fn timer_saves_its_relative_time_then_the_second_word() {
        assert_eq!(
            timer_saves(10.0f32.to_bits()),
            vec![7.5f32.to_bits(), 0x0102_0304]
        );
        assert_eq!(
            timer_saves(f32::NEG_INFINITY.to_bits()),
            vec![f32::NEG_INFINITY.to_bits(), 0x0102_0304]
        );
        assert_eq!(timer_saves(0xffa0_0000), vec![0xffe0_0000, 0x0102_0304]);
        assert_eq!(
            timer_saves(f32::MAX.to_bits()),
            vec![f32::MAX.to_bits(), 0x0102_0304]
        );
    }
}

#[cfg(test)]
mod n026_tests {
    use super::*;

    /// The byte-load double writes the next word of `LOADED` (the count at
    /// +0, the words from +4) into the destination.
    const LOADED: u32 = 0x0331_0000;

    fn setup(e: &mut Engine, words: &[u32]) {
        e.map(FLT_MAX_CONSTANT, 4);
        e.mem.set_u32(FLT_MAX_CONSTANT, f32::MAX.to_bits());
        e.map(TIMER_TIME_BASE, 4);
        e.mem.set_u32(TIMER_TIME_BASE, 2.5f32.to_bits());
        e.register(TIME_STAMP_TIME_BASE, |_, _| Ret {
            st0: 1.25,
            ..Ret::default()
        });
        e.map(LOADED, 0x40);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(LOADED + 4 + 4 * i as u32, *word);
        }
        e.register(LOAD_BYTES, |e, a| {
            assert_eq!(a[0], 0x0abc_0000);
            assert_eq!(a[2], 4);
            let used = e.mem.u32(LOADED);
            let word = e.mem.u32(LOADED + 4 + used * 4);
            e.mem.set_u32(a[1], word);
            e.mem.set_u32(LOADED, used + 1);
            Ret::default()
        });
    }

    fn timer_loads(value: u32) -> (u32, u32) {
        let mut e = Engine::new();
        setup(&mut e, &[value, 0x0102_0304]);
        let this = e.mem.alloc(8);
        e.call(0x009a_5f20, &args![this, 0x0abc_0000u32]);
        assert_eq!(e.mem.u32(LOADED), 2);
        (e.mem.u32(this), e.mem.u32(this + 4))
    }

    #[test]
    fn timer_loads_both_words_and_adds_the_base() {
        assert_eq!(
            timer_loads(7.5f32.to_bits()),
            (10.0f32.to_bits(), 0x0102_0304)
        );
        assert_eq!(
            timer_loads(f32::MAX.to_bits()),
            (f32::MAX.to_bits(), 0x0102_0304)
        );
        assert_eq!(timer_loads((-f32::MAX).to_bits()).0, (-f32::MAX).to_bits());
        // A signalling NaN is unordered and stays as loaded (no FSTP).
        assert_eq!(timer_loads(0x7f80_0001).0, 0x7f80_0001);
    }

    fn stamp_loads(value: u32) -> u32 {
        let mut e = Engine::new();
        setup(&mut e, &[value]);
        let this = e.mem.alloc(8);
        e.call(0x009a_5ff0, &args![this, 0x0abc_0000u32]);
        assert_eq!(e.mem.u32(LOADED), 1);
        e.mem.u32(this)
    }

    #[test]
    fn time_stamp_loads_and_adds_the_time() {
        assert_eq!(stamp_loads(8.75f32.to_bits()), 10.0f32.to_bits());
        assert_eq!(
            stamp_loads(f32::INFINITY.to_bits()),
            f32::INFINITY.to_bits()
        );
        assert_eq!(stamp_loads(0xffa0_0000), 0xffa0_0000);
        let below = f32::from_bits(f32::MAX.to_bits() - 1);
        assert_eq!(
            stamp_loads(below.to_bits()),
            x87_store32(x87_add(1.25, f64::from(below)))
        );
    }
}
