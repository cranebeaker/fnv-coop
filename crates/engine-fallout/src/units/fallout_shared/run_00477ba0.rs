//! `fallout shared/run_00477ba0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00477ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 0x00800000 of the flag word at
/// `this + 8` is set.
pub fn fn_00477ba0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(8)) & 0x0080_0000 != 0
}

/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `100.0` (`double`).
const HUNDRED: u32 = 0x0101_7a40;
/// `BGSSaveFormBuffer::GetForm` (engine map, `007af430`, `thiscall`): the
/// reference's base form (the word at +0x20 by the body).
const BASE_FORM: u32 = 0x007a_f430;
/// `BGSDestructibleObjectForm::GetDestructionForm` (engine map, `00475400`,
/// `cdecl`, one word).
const DESTRUCTION_FORM: u32 = 0x0047_5400;
/// `00726070` (`thiscall`, no stack word): the word at +4 of `this` (here
/// the destruction data).
const WORD_AT_4: u32 = 0x0072_6070;
/// `00475ab0` (`thiscall`, `RET 4`; unit `bgsdestructibleobjectform.cpp`):
/// the reference's health, as an unsigned word.
const HEALTH: u32 = 0x0047_5ab0;
/// `00476b20` (`cdecl`, a `float`; answer in `ST0`; same unit): the
/// percentage the stage is chosen by.
const STAGE_PERCENT: u32 = 0x0047_6b20;

// Translated from 00477bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destruction stage's word for `damage` on `reference` (`cdecl`, two
/// words; the engine map gives it no name). 0 when there is no reference,
/// when `damage` equals the `0.0` at `01012060` (a NaN goes on), when the
/// base form (`007af430`) has no destruction form (`00475400`) or that has
/// no data (`00726070`). Otherwise the percentage `damage * 100 / health`
/// (`00475ab0`; x87, rounded to a `float`) goes through `00476b20` and is
/// truncated to a byte; the data's stages (count byte at +4, pointer array
/// at +8) are walked from the first while a stage's byte at +1 is at least
/// that byte, and the answer is the word at +4 of the last such stage (0
/// when none is).
pub fn fn_00477bc0(e: &mut Engine, reference: u32, damage: u32) -> u32 {
    if reference == 0 {
        return 0;
    }
    let zero: f64 = e.global(ZERO);
    if x87_load32(damage) == zero {
        return 0;
    }
    let base = e.call(BASE_FORM, &args![reference]).u32();
    let destructible = e.call(DESTRUCTION_FORM, &args![base]).u32();
    if destructible == 0 {
        return 0;
    }
    let data = e.call(WORD_AT_4, &args![destructible]).u32();
    if data == 0 {
        return 0;
    }
    let health = e.call(HEALTH, &args![destructible, reference]).u32();
    let hundred: f64 = e.global(HUNDRED);
    let ratio = x87_div(x87_mul(x87_load32(damage), hundred), f64::from(health));
    let ratio = x87_store32(ratio);
    let percent = e.call(STAGE_PERCENT, &args![ratio]).f64();
    let threshold = crate::x87::fistp_chop(percent) as u8;
    let mut result = 0u32;
    let mut index = 0u32;
    while index < u32::from(e.mem.u8(data + 4)) {
        let stages = e.mem.u32(data + 8);
        let stage = e.mem.u32(stages.wrapping_add(index.wrapping_mul(4)));
        if e.mem.u8(stage + 1) < threshold {
            break;
        }
        result = e.mem.u32(stage + 4);
        index += 1;
    }
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_7bc0, fn_00477bc0(u32, u32) -> u32),
        entry!(0x0047_7ba0, fn_00477ba0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00477ba0_tests_bit_23_of_the_word_at_8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0xff7f_ffff);
        assert!(!e.call(0x0047_7ba0, &args![this]).bool());
        e.mem.set_u32(this + 8, 0x0080_0000);
        assert!(e.call(0x0047_7ba0, &args![this]).bool());
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    /// A reference with a destruction form whose data has three stages at
    /// thresholds 75, 50 and 25 (words 0x71, 0x72, 0x73); the stage
    /// percentage double answers its argument.
    fn setup() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.mem.map(0x0101_2000, 0x6000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(HUNDRED, 100.0f64);
        let data = e.mem.alloc(0xc);
        let stages = e.mem.alloc(0xc);
        e.mem.set_u8(data + 4, 3);
        e.mem.set_u32(data + 8, stages);
        for (i, (threshold, word)) in [(75u8, 0x71u32), (50, 0x72), (25, 0x73)]
            .into_iter()
            .enumerate()
        {
            let stage = e.mem.alloc(8);
            e.mem.set_u8(stage + 1, threshold);
            e.mem.set_u32(stage + 4, word);
            e.mem.set_u32(stages + 4 * i as u32, stage);
        }
        let destructible = e.mem.alloc(8);
        e.mem.set_u32(destructible + 4, data);
        e.mem.map(0x0f00_0000, 4);
        e.mem.set_u32(0x0f00_0000, destructible);
        e.register(BASE_FORM, |_, _| Ret {
            eax: 0x4000,
            ..Ret::default()
        });
        e.register(DESTRUCTION_FORM, |e, _| Ret {
            eax: e.mem.u32(0x0f00_0000),
            ..Ret::default()
        });
        e.register(WORD_AT_4, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(HEALTH, |_, _| Ret {
            eax: 200,
            ..Ret::default()
        });
        e.register(STAGE_PERCENT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        (e, destructible, data)
    }

    fn stage(e: &mut Engine, reference: u32, damage: f32) -> u32 {
        e.call(0x0047_7bc0, &args![reference, damage.to_bits()])
            .u32()
    }

    #[test]
    fn at_00477bc0_picks_the_last_stage_reached() {
        let (mut e, _, data) = setup();
        // 120 of 200 health: 60 %, past the first stage (75) only.
        assert_eq!(stage(&mut e, 0x3000, 120.0), 0x71);
        // 60 of 200: 30 %, past 75 and 50.
        assert_eq!(stage(&mut e, 0x3000, 60.0), 0x72);
        // 199.8 of 200: 99.9 % truncates to 99, below every stage.
        assert_eq!(stage(&mut e, 0x3000, 199.8), 0);
        // 10 of 200: 5 %, every stage.
        assert_eq!(stage(&mut e, 0x3000, 10.0), 0x73);
        // A threshold equal to the stage's counts as reached.
        assert_eq!(stage(&mut e, 0x3000, 150.0), 0x71);
        // No stages.
        e.mem.set_u8(data + 4, 0);
        assert_eq!(stage(&mut e, 0x3000, 10.0), 0);
    }

    #[test]
    fn at_00477bc0_answers_zero_early() {
        let (mut e, destructible, _) = setup();
        assert_eq!(stage(&mut e, 0, 10.0), 0);
        assert_eq!(stage(&mut e, 0x3000, 0.0), 0);
        assert_eq!(stage(&mut e, 0x3000, -0.0), 0);
        e.mem.set_u32(destructible + 4, 0);
        assert_eq!(stage(&mut e, 0x3000, 10.0), 0);
        e.mem.set_u32(0x0f00_0000, 0);
        e.call_log = Some(vec![]);
        assert_eq!(stage(&mut e, 0x3000, 10.0), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == WORD_AT_4));
    }
}
