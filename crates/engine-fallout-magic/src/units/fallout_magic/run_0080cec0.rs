//! `fallout/magic/run_0080cec0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A byte flag at `011dc034` that `0080cec0` clears (its readers are not in this unit).
const FLAG_011DC034: u32 = 0x011d_c034;
/// The word at `011c3588` that `0080d0d0` answers (a shader-like form the hit effect is
/// built from; its type is not confirmed here).
const HIT_SHADER_011C3588: u32 = 0x011c_3588;
/// The player (`PlayerCharacter*` global at `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map name of `008d8520`; the body
/// answers the word at +0x68 of a reference: its process). `thiscall`, no stack words.
const REFERENCE_PROCESS: u32 = 0x008d_8520;
/// `00403e20` (`thiscall`, no stack words): the address of a setting's value (+4), or of a
/// zeroed float at `01202800` for a null setting.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The setting object at `011d15e0` whose float value scales the range.
const RANGE_SETTING: u32 = 0x011d_15e0;
/// The reference vtable slot +0x1F4 (no stack words): the address of its position
/// (three floats).
const SLOT_POSITION: u32 = 0x1f4;
/// `00439ef0` (`thiscall` on a point, two words: out point, other point): the difference,
/// written to the out point, whose address it returns.
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
/// `004a7290` (`thiscall` on a point): its length in `ST0`.
const POINT_LENGTH: u32 = 0x004a_7290;
/// The reference vtable slot +0x22C (one word, here 0): a byte answer that makes the
/// effect fail when set.
const SLOT_22C: u32 = 0x22c;
/// `0045cd60` (`thiscall`): the word at +0x28 of the process.
const PROCESS_WORD_28: u32 = 0x0045_cd60;
/// `00477ba0` (`thiscall`): bit 0x800000 of the form flags (+8) of the reference.
const FORM_FLAG_800000: u32 = 0x0047_7ba0;
/// The allocator `00aa13e0` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x00aa_13e0;
/// The `MagicShaderHitEffect` constructor (by the size and the vtable `0081f580` stores;
/// `thiscall`, three words: target, shader word, float).
const SHADER_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_f580;
/// The float pushed to that constructor (`01012054`).
const HIT_EFFECT_FLOAT: u32 = 0x0101_2054;
/// The new effect's vtable slot +0xC4 (no stack words): a byte saying it can be added.
const SLOT_C4: u32 = 0xc4;
/// The process lists (`ProcessLists` object at `011e0e80`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `ProcessLists::AddTempEffect` (engine map, `00973fd0`; `thiscall`, one word).
const ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
/// `ProcessLists::FinishMagicShaderHitEffect` (engine map, `00974a50`; `thiscall`, two
/// words: reference, shader word).
const FINISH_MAGIC_SHADER_HIT_EFFECT: u32 = 0x0097_4a50;

// Translated from 0080cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): clears the byte at `011dc034`.
pub fn fn_0080cec0(e: &mut Engine) {
    e.set_global::<u8>(FLAG_011DC034, 0);
}

// Translated from 0080ced0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080ced0 - the FS:[0] exception frame (freeing the hit effect when its constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, two words: a reference and a byte). Fails (false)
/// when `0080d0d0` answers null, the reference is null or the player, or it has no process
/// (`008d8520`). Otherwise it compares the distance between the player's and the
/// reference's positions (vtable +0x1F4, `00439ef0`, `004a7290`) with the square of
/// `0.0 * value of the setting at 011d15e0` (x87, rounded to `float` at each store). When
/// that square is greater than the distance and the reference's slot +0x22C (with 0), its
/// process's word at +0x28 and its form flag 0x800000 are all clear, it succeeds (true),
/// first building a 0x6c-byte hit effect (`00aa13e0`, `0081f580` with the reference, the
/// word and the float at `01012054`) when `flag` is clear: added to the process lists when
/// its slot +0xC4 answers true, deleted (slot 0 with 1) otherwise. In every other case it
/// fails, first finishing the reference's shader hit effect (`00974a50`) when `flag` is
/// set.
pub fn fn_0080ced0(e: &mut Engine, reference: u32, flag: u8) -> bool {
    let shader = fn_0080d0d0(e);
    let player: u32 = e.global(PLAYER);
    if shader == 0
        || reference == 0
        || reference == player
        || e.call(REFERENCE_PROCESS, &args![reference]).u32() == 0
    {
        return false;
    }
    let zero = x87_store32(0.0);
    let value = e.call(SETTING_VALUE, &args![RANGE_SETTING]).u32();
    let range = x87_store32(x87_mul(x87_load32(zero), x87_m32(e.mem.u32(value))));
    let square = x87_store32(x87_mul(x87_load32(range), x87_m32(range)));
    let distance = e.with_stack(0x20, |e, frame| {
        let player_position = frame.addr();
        let difference = frame.addr() + 0x10;
        let player: u32 = e.global(PLAYER);
        let position = e.vcall(player, SLOT_POSITION, &args![]).u32();
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(player_position + 4 * i, word);
        }
        let other = e.vcall(reference, SLOT_POSITION, &args![]).u32();
        let point = e
            .call(POINT_DIFFERENCE, &args![player_position, difference, other])
            .u32();
        e.call(POINT_LENGTH, &args![point]).f64()
    });
    // FLD square; FCOMPP with the distance below; TEST AH,0x41; JNZ: on to the success
    // path only when the square is greater (ordered).
    let close_enough = x87_load32(square) > distance;
    if close_enough
        && !e.vcall(reference, SLOT_22C, &args![0u32]).bool()
        && {
            let process = e.call(REFERENCE_PROCESS, &args![reference]).u32();
            e.call(PROCESS_WORD_28, &args![process]).u32() == 0
        }
        && !e.call(FORM_FLAG_800000, &args![reference]).bool()
    {
        if flag == 0 {
            let block = e.call(ALLOCATE, &args![0x6cu32]).u32();
            let effect = if block == 0 {
                0
            } else {
                let float = x87_quiet_bits(e.global::<u32>(HIT_EFFECT_FLOAT));
                e.call(
                    SHADER_HIT_EFFECT_CONSTRUCTOR,
                    &args![block, reference, shader, float],
                )
                .u32()
            };
            if e.vcall(effect, SLOT_C4, &args![]).bool() {
                e.call(ADD_TEMP_EFFECT, &args![PROCESS_LISTS, effect]);
            } else if effect != 0 {
                e.vcall(effect, 0, &args![1u32]);
            }
        }
        return true;
    }
    if flag != 0 {
        e.call(
            FINISH_MAGIC_SHADER_HIT_EFFECT,
            &args![PROCESS_LISTS, reference, shader],
        );
    }
    false
}

// Translated from 0080d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the word at `011c3588`.
pub fn fn_0080d0d0(e: &mut Engine) -> u32 {
    e.global(HIT_SHADER_011C3588)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_cec0, fn_0080cec0()),
        entry!(0x0080_ced0, fn_0080ced0(u32, u8) -> bool),
        entry!(0x0080_d0d0, fn_0080d0d0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const POSITION_DOUBLE: u32 = 0x0e30_0000;
    const SLOT_22C_DOUBLE: u32 = 0x0e30_0100;
    const SLOT_C4_DOUBLE: u32 = 0x0e30_0200;
    const DELETE_DOUBLE: u32 = 0x0e30_0300;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_0080cec0_and_0080d0d0_touch_their_globals() {
        let mut e = Engine::new();
        e.map(0x011d_c000, 0x100);
        e.map(0x011c_3000, 0x1000);
        e.set_global::<u32>(FLAG_011DC034, 0xffff_ffff);
        e.call(0x0080_cec0, &[]);
        assert_eq!(e.global::<u32>(FLAG_011DC034), 0xffff_ff00);
        e.set_global::<u32>(HIT_SHADER_011C3588, 0x4321);
        assert_eq!(e.call(0x0080_d0d0, &[]).u32(), 0x4321);
    }

    struct World {
        e: Engine,
        reference: u32,
        position: u32,
        block: u32,
    }

    /// A player and a reference sharing one vtable (position slot, slot +0x22C answering
    /// false), the shader word set, the setting value `setting`, a hit effect block whose
    /// slot +0xC4 answers true, and every callee doubled; the distance comes back as
    /// `distance`.
    fn world(distance: f64, setting: f32) -> World {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_2000, 0x100);
        e.map(0x011e_0000, 0x1000);
        e.set_global::<u32>(HIT_SHADER_011C3588, 0x6600);
        e.set_global::<f32>(HIT_EFFECT_FLOAT, -1.0);
        let vtable = e.mem.alloc(0x300);
        e.mem.set_u32(vtable + SLOT_POSITION, POSITION_DOUBLE);
        e.mem.set_u32(vtable + SLOT_22C, SLOT_22C_DOUBLE);
        let player = e.mem.alloc(0x20);
        e.mem.set_u32(player, vtable);
        e.set_global::<u32>(PLAYER, player);
        let reference = e.mem.alloc(0x20);
        e.mem.set_u32(reference, vtable);
        let effect_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(effect_vtable, DELETE_DOUBLE);
        e.mem.set_u32(effect_vtable + SLOT_C4, SLOT_C4_DOUBLE);
        let block = e.mem.alloc(0x6c);
        e.mem.set_u32(block, effect_vtable);
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, setting);
        e.register(REFERENCE_PROCESS, |_, _| 0x7000u32.into_ret());
        e.register_double(SETTING_VALUE, move |_, _| value.into_ret());
        e.register_double(POSITION_DOUBLE, move |_, _| position.into_ret());
        e.register(SLOT_22C_DOUBLE, |_, _| 0u32.into_ret());
        e.register(SLOT_C4_DOUBLE, |_, _| 1u32.into_ret());
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(POINT_DIFFERENCE, |_, a| a[1].into_ret());
        e.register_double(POINT_LENGTH, move |_, _| distance.into_ret());
        e.register(PROCESS_WORD_28, |_, _| 0u32.into_ret());
        e.register(FORM_FLAG_800000, |_, _| 0u32.into_ret());
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        e.register(SHADER_HIT_EFFECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(ADD_TEMP_EFFECT, |_, _| Ret::default());
        e.register(FINISH_MAGIC_SHADER_HIT_EFFECT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        World {
            e,
            reference,
            position,
            block,
        }
    }

    #[test]
    fn fn_0080ced0_fails_early_without_shader_reference_or_process() {
        let mut w = world(1.0, 2.0);
        let player: u32 = w.e.global(PLAYER);
        assert!(!w.e.call(0x0080_ced0, &[0, 1]).bool());
        assert!(!w.e.call(0x0080_ced0, &[player, 1]).bool());
        w.e.register(REFERENCE_PROCESS, |_, _| 0u32.into_ret());
        assert!(!w.e.call(0x0080_ced0, &[w.reference, 1]).bool());
        w.e.set_global::<u32>(HIT_SHADER_011C3588, 0);
        w.e.register(REFERENCE_PROCESS, |_, _| 0x7000u32.into_ret());
        assert!(!w.e.call(0x0080_ced0, &[w.reference, 1]).bool());
        // None of these reached the distance or finished a hit effect.
        assert!(calls(&w.e, POINT_LENGTH).is_empty());
        assert!(calls(&w.e, FINISH_MAGIC_SHADER_HIT_EFFECT).is_empty());
    }

    #[test]
    fn fn_0080ced0_out_of_range_finishes_the_hit_effect_when_flagged() {
        // The square of 0.0 * setting is 0.0, not greater than the distance 1.0.
        let mut w = world(1.0, 2.0);
        assert!(!w.e.call(0x0080_ced0, &[w.reference, 1]).bool());
        let difference = calls(&w.e, POINT_DIFFERENCE);
        assert_eq!(difference.len(), 1);
        assert_eq!(difference[0][2], w.position);
        assert_eq!(
            calls(&w.e, FINISH_MAGIC_SHADER_HIT_EFFECT),
            vec![vec![PROCESS_LISTS, w.reference, 0x6600]]
        );
        assert!(!w.e.call(0x0080_ced0, &[w.reference, 0]).bool());
        assert_eq!(calls(&w.e, FINISH_MAGIC_SHADER_HIT_EFFECT).len(), 1);
        assert!(calls(&w.e, SLOT_22C_DOUBLE).is_empty());
    }

    #[test]
    fn fn_0080ced0_in_range_builds_and_adds_the_hit_effect() {
        // A negative distance is below the square 0.0.
        let mut w = world(-1.0, 2.0);
        assert!(w.e.call(0x0080_ced0, &[w.reference, 0]).bool());
        assert_eq!(calls(&w.e, SLOT_22C_DOUBLE), vec![vec![w.reference, 0]]);
        assert_eq!(calls(&w.e, ALLOCATE), vec![vec![0x6c]]);
        assert_eq!(
            calls(&w.e, SHADER_HIT_EFFECT_CONSTRUCTOR),
            vec![vec![w.block, w.reference, 0x6600, (-1.0f32).to_bits()]]
        );
        assert_eq!(
            calls(&w.e, ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, w.block]]
        );
        assert!(calls(&w.e, DELETE_DOUBLE).is_empty());
        // With the flag set it succeeds without building anything.
        assert!(w.e.call(0x0080_ced0, &[w.reference, 1]).bool());
        assert_eq!(calls(&w.e, ALLOCATE).len(), 1);
    }

    #[test]
    fn fn_0080ced0_deletes_an_effect_it_cannot_add() {
        let mut w = world(-1.0, 2.0);
        w.e.register(SLOT_C4_DOUBLE, |_, _| 0x100u32.into_ret());
        assert!(w.e.call(0x0080_ced0, &[w.reference, 0]).bool());
        assert!(calls(&w.e, ADD_TEMP_EFFECT).is_empty());
        assert_eq!(calls(&w.e, DELETE_DOUBLE), vec![vec![w.block, 1]]);
    }

    #[test]
    fn fn_0080ced0_in_range_but_blocked_fails() {
        let mut w = world(-1.0, 2.0);
        w.e.register(PROCESS_WORD_28, |_, _| 1u32.into_ret());
        assert!(!w.e.call(0x0080_ced0, &[w.reference, 1]).bool());
        assert_eq!(calls(&w.e, FINISH_MAGIC_SHADER_HIT_EFFECT).len(), 1);
        w.e.register(PROCESS_WORD_28, |_, _| 0u32.into_ret());
        w.e.register(FORM_FLAG_800000, |_, _| 1u32.into_ret());
        assert!(!w.e.call(0x0080_ced0, &[w.reference, 0]).bool());
        w.e.register(FORM_FLAG_800000, |_, _| 0u32.into_ret());
        w.e.register(SLOT_22C_DOUBLE, |_, _| 1u32.into_ret());
        assert!(!w.e.call(0x0080_ced0, &[w.reference, 0]).bool());
        assert!(calls(&w.e, ALLOCATE).is_empty());
    }

    #[test]
    fn fn_0080ced0_nan_setting_never_succeeds() {
        // 0.0 * NaN is NaN; an unordered compare takes the failure path.
        let mut w = world(-1.0, f32::NAN);
        assert!(!w.e.call(0x0080_ced0, &[w.reference, 1]).bool());
    }
}
