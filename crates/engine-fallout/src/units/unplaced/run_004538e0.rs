//! `(unplaced)/run_004538e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Members of one object (its class is not confirmed here; its callers
//! are in `tes.cpp` by the engine map): a critical section enter, two
//! helpers on the word at +8, a global's first word, and a reset-like
//! sequence that ends by setting the byte at +0xB4.

#[allow(unused_imports)]
use crate::prelude::*;

/// The import slot of `EnterCriticalSection` (`KERNEL32`, `0x00fdf05c`);
/// takes the section's address.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// Returns the word at +0x34 of its `this` (`005f36f0`, `thiscall`; named
/// `ActorMover::GetPreferredMoveMode` by the map, a folded body).
const WORD_AT_34: u32 = 0x005f_36f0;
/// `00552840` (`thiscall`, no stack words), on the word at +0x34.
const ON_WORD_AT_34: u32 = 0x0055_2840;
/// `004baab0` (`thiscall`, no stack words), on the word at +8.
const ON_WORD_AT_8: u32 = 0x004b_aab0;
/// A global object pointer (`011c3f2c`); `00462290`
/// (`TESDataHandler::UnloadCell` in the Xbox PDB) is called on it.
const GLOBAL_OBJECT: u32 = 0x011c_3f2c;
/// `00451570` (`cdecl`, one word).
const SET_FIRST: u32 = 0x0045_1570;
/// `00451520` (`cdecl`, one byte word): stores the byte at `01189184`.
const SET_FLAG_BYTE: u32 = 0x0045_1520;
/// `0043d4d0` (`thiscall`): `this + 4`, or `01202800` (zeroed) for a null
/// `this`.
const ADDRESS_AFTER_FIRST: u32 = 0x0043_d4d0;
/// The global object [`fn_00453980`] passes to [`ADDRESS_AFTER_FIRST`].
const GLOBAL_STRUCT: u32 = 0x011c_3c90;
/// `0045bb80` (`thiscall`): returns an object for `00470470`.
const GET_FIRST_OBJECT: u32 = 0x0045_bb80;
/// `00470470` (`thiscall`, no stack words).
const ON_FIRST_OBJECT: u32 = 0x0047_0470;
/// `BSAudio::QInstance` (`00453a70`, Xbox PDB): the audio singleton.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `00483710` (`thiscall`, an empty body), on the audio singleton.
const ON_AUDIO: u32 = 0x0048_3710;
/// `00455200` (`thiscall`, `RET 8`).
const STEP_A: u32 = 0x0045_5200;
/// `00454bd0` (`thiscall`, `RET 4`): a byte test on the word at +0x34.
const TEST_WORD_AT_34: u32 = 0x0045_4bd0;
/// `00462290` (`TESDataHandler::UnloadCell` in the Xbox PDB; `thiscall`,
/// `RET 4`).
const UNLOAD: u32 = 0x0046_2290;
/// `00454d50` (`thiscall`, `RET 4`).
const STEP_B: u32 = 0x0045_4d50;
/// `008d7dc0` (`LowProcess::SetItemBeingUsed` in the map, a folded body;
/// `thiscall`, `RET 4`), on `this`.
const STEP_C: u32 = 0x008d_7dc0;

// Translated from 004538e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Enters the critical section at `this` (`EnterCriticalSection`). The
/// function ends with `RET 4`, so it takes one stack word it never reads.
pub fn fn_004538e0(e: &mut Engine, this: Ptr, _unused_0: u32) {
    e.call(ENTER_CRITICAL_SECTION, &args![this]);
}

// Translated from 00453900 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`thiscall`.) With a non-null word at +0x34 (`005f36f0`), calls
/// `00552840` on it; otherwise `004baab0` on the word at +8.
pub fn fn_00453900(e: &mut Engine, this: Ptr) {
    let word = e.call(WORD_AT_34, &args![this]).u32();
    if word != 0 {
        e.call(ON_WORD_AT_34, &args![word]);
    } else {
        let object = e.mem.u32(this.addr() + 8);
        e.call(ON_WORD_AT_8, &args![object]);
    }
}

// Translated from 00453940 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`thiscall`.) Only when the global at `011c3f2c` is set: slot 0x08 of the
/// object at +8 (no stack words), then `00451570(0)` and `00451520(1)`.
pub fn fn_00453940(e: &mut Engine, this: Ptr) {
    if e.global::<u32>(GLOBAL_OBJECT) == 0 {
        return;
    }
    let object = e.mem.u32(this.addr() + 8);
    e.vcall(object, 0x08, &[]);
    e.call(SET_FIRST, &args![0u32]);
    e.call(SET_FLAG_BYTE, &args![1u32]);
}

// Translated from 00453980 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`thiscall`, `this` not read.) The word at the address `0043d4d0`
/// returns for the global `011c3c90`.
pub fn fn_00453980(e: &mut Engine, _this: Ptr) -> u32 {
    let p = e.call(ADDRESS_AFTER_FIRST, &args![GLOBAL_STRUCT]).u32();
    e.mem.u32(p)
}

// Translated from 004539a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`thiscall`, `RET 8`; both arguments are bytes.) `00470470` on what
/// `0045bb80` returns; [`fn_00453940`] unless `first` is set and the word at
/// +0x34 is null; `00483710` on the audio singleton; `00455200(first, 0)`.
/// When the word at +0x34 is set and `first` is not, and `00454bd0` with
/// that word says no, `00462290` on the global at `011c3f2c` with it. Then
/// `00454d50(first)`; with neither `first` nor `second`, `008d7dc0(0)`;
/// finally the byte at +0xB4 = 1.
pub fn fn_004539a0(e: &mut Engine, this: Ptr, first: u8, second: u8) {
    let object = e.call(GET_FIRST_OBJECT, &args![this]).u32();
    e.call(ON_FIRST_OBJECT, &args![object]);
    if first == 0 || e.call(WORD_AT_34, &args![this]).u32() != 0 {
        fn_00453940(e, this);
    }
    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
    e.call(ON_AUDIO, &args![audio]);
    e.call(STEP_A, &args![this, u32::from(first), 0u32]);
    if e.call(WORD_AT_34, &args![this]).u32() != 0 && first == 0 {
        let word = e.call(WORD_AT_34, &args![this]).u32();
        if e.call(TEST_WORD_AT_34, &args![this, word]).u8() == 0 {
            let word = e.call(WORD_AT_34, &args![this]).u32();
            let global = e.global::<u32>(GLOBAL_OBJECT);
            e.call(UNLOAD, &args![global, word]);
        }
    }
    e.call(STEP_B, &args![this, u32::from(first)]);
    if first == 0 && second == 0 {
        e.call(STEP_C, &args![this, 0u32]);
    }
    e.mem.set_u8(this.addr() + 0xb4, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004538e0, fn_004538e0(Ptr, u32)),
        entry!(0x00453900, fn_00453900(Ptr)),
        entry!(0x00453940, fn_00453940(Ptr)),
        entry!(0x00453980, fn_00453980(Ptr) -> u32),
        entry!(0x004539a0, fn_004539a0(Ptr, u8, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn enters_the_section_at_this() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0045_38e0, &args![0x2000u32, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ENTER_CRITICAL_SECTION, vec![0x2000]));
    }

    #[test]
    fn picks_the_word_at_34_or_the_word_at_8() {
        let mut e = Engine::new();
        e.register(WORD_AT_34, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(ON_WORD_AT_34, |_, _| Ret::default());
        e.register(ON_WORD_AT_8, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, 0x888);
        e.mem.set_u32(this + 0x34, 0x343);
        e.call_log = Some(vec![]);
        e.call(0x0045_3900, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ON_WORD_AT_34), vec![vec![0x343]]);
        assert!(calls_to(&log, ON_WORD_AT_8).is_empty());

        e.mem.set_u32(this + 0x34, 0);
        e.call_log = Some(vec![]);
        e.call(0x0045_3900, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ON_WORD_AT_8), vec![vec![0x888]]);
        assert!(calls_to(&log, ON_WORD_AT_34).is_empty());
    }

    #[test]
    fn runs_its_sequence_only_with_the_global_set() {
        let mut e = Engine::new();
        e.register(SET_FIRST, |_, _| Ret::default());
        e.register(SET_FLAG_BYTE, |_, _| Ret::default());
        e.register(0x0300_1000, |_, _| Ret::default());
        e.put_vtable(0x0300_0000, &[0, 0, 0x0300_1000]);
        e.map(GLOBAL_OBJECT, 4);
        let this = e.mem.alloc(0x40);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0300_0000);
        e.mem.set_u32(this + 8, object);
        e.call_log = Some(vec![]);
        e.call(0x0045_3940, &[this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);

        e.set_global::<u32>(GLOBAL_OBJECT, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x0045_3940, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0300_1000, vec![object]),
                (SET_FIRST, vec![0]),
                (SET_FLAG_BYTE, vec![1]),
            ]
        );
    }

    #[test]
    fn reads_the_word_the_helper_points_at() {
        let mut e = Engine::new();
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0xfeed);
        e.register_double(ADDRESS_AFTER_FIRST, move |_, a| {
            assert_eq!(a[0], GLOBAL_STRUCT);
            cell.into_ret()
        });
        assert_eq!(e.call(0x0045_3980, &[0x1234]).u32(), 0xfeed);
    }

    fn sequence_doubles(e: &mut Engine, test_result: u32) {
        e.register(WORD_AT_34, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(GET_FIRST_OBJECT, |_, _| 0x0bbu32.into_ret());
        e.register(AUDIO_INSTANCE, |_, _| 0x0aau32.into_ret());
        for a in [
            ON_FIRST_OBJECT,
            ON_AUDIO,
            STEP_A,
            UNLOAD,
            STEP_B,
            STEP_C,
            SET_FIRST,
            SET_FLAG_BYTE,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.register_double(TEST_WORD_AT_34, move |_, _| Ret {
            eax: 0x1234_5600 | test_result,
            ..Ret::default()
        });
        e.map(GLOBAL_OBJECT, 4);
        e.set_global::<u32>(GLOBAL_OBJECT, 0);
    }

    #[test]
    fn the_sequence_without_flags_unloads_and_clears() {
        let mut e = Engine::new();
        sequence_doubles(&mut e, 0);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0x34, 0x343);
        e.call_log = Some(vec![]);
        e.call(0x0045_39a0, &[this, 0, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ON_FIRST_OBJECT), vec![vec![0x0bb]]);
        assert_eq!(calls_to(&log, ON_AUDIO), vec![vec![0x0aa]]);
        assert_eq!(calls_to(&log, STEP_A), vec![vec![this, 0, 0]]);
        assert_eq!(calls_to(&log, TEST_WORD_AT_34), vec![vec![this, 0x343]]);
        assert_eq!(calls_to(&log, UNLOAD), vec![vec![0, 0x343]]);
        assert_eq!(calls_to(&log, STEP_B), vec![vec![this, 0]]);
        assert_eq!(calls_to(&log, STEP_C), vec![vec![this, 0]]);
        assert_eq!(e.mem.u8(this + 0xb4), 1);
    }

    #[test]
    fn the_sequence_keeps_the_word_when_the_test_says_yes() {
        let mut e = Engine::new();
        sequence_doubles(&mut e, 1);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0x34, 0x343);
        e.call_log = Some(vec![]);
        e.call(0x0045_39a0, &[this, 0, 1]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, UNLOAD).is_empty());
        assert!(calls_to(&log, STEP_C).is_empty());
    }

    #[test]
    fn the_sequence_with_first_set_skips_the_null_word_branch() {
        let mut e = Engine::new();
        sequence_doubles(&mut e, 0);
        e.set_global::<u32>(GLOBAL_OBJECT, 0x99);
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        e.call(0x0045_39a0, &[this, 1, 0]);
        let log = e.call_log.take().unwrap();
        // With `first` set and a null word, `00453940` doesn't run although
        // the global is set.
        assert!(calls_to(&log, SET_FIRST).is_empty());
        assert_eq!(calls_to(&log, STEP_A), vec![vec![this, 1, 0]]);
        assert!(calls_to(&log, TEST_WORD_AT_34).is_empty());
        assert_eq!(calls_to(&log, STEP_B), vec![vec![this, 1]]);
        assert!(calls_to(&log, STEP_C).is_empty());
        assert_eq!(e.mem.u8(this + 0xb4), 1);
    }
}
