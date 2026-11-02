//! `NiAnimation/niboolinterpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBoolInterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOL_INTERP_CONTROLLER_RTTI: u32 = 0x011f_4228;

// Translated from 00a577b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bool_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOL_INTERP_CONTROLLER_RTTI)
}

/// `00a39010`: the jump target of `00a57c00`.
const JUMP_TARGET_00A57C00: u32 = 0x00a3_9010;

/// `00a39030`: the jump target of `00a57c10`.
const JUMP_TARGET_00A57C10: u32 = 0x00a3_9030;

// Translated from 00a57c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::SaveBinary` (Xbox PDB): a 5-byte jump to `00a39010` (engine map: `NiSingleInterpController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiBoolInterpController@@UAAXAAVNiStream@@@Z`.
pub fn ni_bool_interp_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A57C00, &args![this, stream]);
}

// Translated from 00a57c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::IsEqual` (Xbox PDB): a 5-byte jump to `00a39030` (engine map: `NiSingleInterpController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiBoolInterpController@@UAA_NPAVNiObject@@@Z`.
pub fn ni_bool_interp_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A57C10, &args![this, other]).bool()
}

/// `NiBoolInterpolator::ms_RTTI` (`011f404c`).
const NI_BOOL_INTERPOLATOR_RTTI: u32 = 0x011f_404c;
/// `00653290` (`thiscall (object; rtti)`): the object when it is of the
/// class (its check `006532c0` answers true), else null.
const DYNAMIC_CAST: u32 = 0x0065_3290;
/// `NiBoolInterpolator::SetPoseValue` (`00a531a0`, engine map; `thiscall`,
/// one word).
const BOOL_INTERPOLATOR_SET_POSE_VALUE: u32 = 0x00a5_31a0;
/// `NiSingleInterpController::GetViewerStrings` (`00a390a0`, engine map;
/// `thiscall`, one word: the strings array).
const SINGLE_INTERP_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a3_90a0;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`); the viewer-string
/// array's grow (`0096ad30`, `thiscall (array; new capacity)`) and store
/// (`0096ae90`, `thiscall (array; index, &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a577c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::SynchronizePoseInterpolator` (Xbox PDB,
/// matched by the engine map; `thiscall`, two words, `RET 8`: the
/// interpolator and a word it does not read): the interpolator as a
/// `NiBoolInterpolator` (`00653290` with its `NiRTTI`; null stays null),
/// the target's value from `this` (vtable +0xEC, given the address of the
/// interpolator's own argument word), and that whole word handed to the
/// cast interpolator's `SetPoseValue` (`00a531a0`).
pub fn ni_bool_interp_controller_synchronize_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: u32,
    _unused_2: u32,
) {
    let cast = if interpolator == 0 {
        0
    } else {
        e.call(
            DYNAMIC_CAST,
            &args![interpolator, NI_BOOL_INTERPOLATOR_RTTI],
        )
        .u32()
    };
    let value = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), interpolator);
        e.vcall(this.addr(), 0xec, &args![slot]);
        e.mem.u32(slot.addr())
    });
    e.call(BOOL_INTERPOLATOR_SET_POSE_VALUE, &args![cast, value]);
}

// Translated from 00a57800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::GetViewerStrings` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the strings array):
/// `NiSingleInterpController::GetViewerStrings` (`00a390a0`), then the
/// class name (the first word of the `NiRTTI` record) appended: when the
/// 16-bit size (+0xA) has reached the capacity (+0x8) the array grows to
/// the step (+0xE) plus the size first, then the string is stored at the
/// old size.
pub fn ni_bool_interp_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(
        SINGLE_INTERP_CONTROLLER_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    let class_name: u32 = e.global(NI_BOOL_INTERP_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_77c0,
            ni_bool_interp_controller_synchronize_pose_interpolator(Ptr, u32, u32)
        ),
        entry!(
            0x00a5_7800,
            ni_bool_interp_controller_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a577b0, ni_bool_interp_controller_get_rtti() -> Ptr),
        entry!(0x00a5_7c00, ni_bool_interp_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a5_7c10, ni_bool_interp_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a3_8e00, fn_00a38e00(u32) -> u32),
        entry!(
            0x00a5_7710,
            ni_bool_interp_controller_create_pose_interpolator(u32, u32) -> u32
        ),
    ]
}

/// `00a2f8c0` (`thiscall`, no stack words): the `NiInterpController` base constructor, by
/// its use here.
const INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a2_f8c0;
/// The `NiSingleInterpController` vtable (RTTI).
const SINGLE_INTERP_CONTROLLER_VTABLE: u32 = 0x0109_6bd4;
/// The game's `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiBoolInterpolator::NiBoolInterpolator` (engine map, `00a532d0`; `thiscall`, one word).
const BOOL_INTERPOLATOR_CONSTRUCT: u32 = 0x00a5_32d0;

// Translated from 00a38e00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed by its body (the engine map gives `NiBoolInterpController::CopyMembers`, folded
/// code; by the vtable it stores the `NiSingleInterpController` constructor; `thiscall`, no
/// stack words): the base constructor `00a2f8c0`, the `NiSingleInterpController` vtable,
/// no interpolator (+0x34). Answers `this`.
pub fn fn_00a38e00(e: &mut Engine, this: u32) -> u32 {
    e.call(INTERP_CONTROLLER_CONSTRUCT, &args![this]);
    e.mem.set_u32(this, SINGLE_INTERP_CONTROLLER_VTABLE);
    e.mem.set_u32(this + 0x34, 0);
    this
}

// Translated from 00a57710 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::CreatePoseInterpolator` (Xbox PDB; `thiscall`, one word it does
/// not read, `RET 4`): asks `this` for the target value (vtable slot +0xEC with the address
/// of a stack word, which starts out holding `this` from the `PUSH ECX`), then answers a
/// 0x18-byte `NiBoolInterpolator` built with that whole word, or null when `operator new`
/// fails.
pub fn ni_bool_interp_controller_create_pose_interpolator(
    e: &mut Engine,
    this: u32,
    _unused_1: u32,
) -> u32 {
    e.with_stack(4, |e, value| {
        let value = value.addr();
        e.mem.set_u32(value, this);
        e.vcall(this, 0xec, &args![value]);
        let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
        if block == 0 {
            return 0;
        }
        let word = e.mem.u32(value);
        e.call(BOOL_INTERPOLATOR_CONSTRUCT, &args![block, word])
            .u32()
    })
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    #[test]
    fn fn_00a38e00_builds_the_single_interp_controller() {
        let mut e = Engine::new();
        e.register(INTERP_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0x34, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_8e00, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), SINGLE_INTERP_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(INTERP_CONTROLLER_CONSTRUCT, vec![this])));
    }

    #[test]
    fn create_pose_interpolator_passes_the_target_value() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(table + 0xec, 0x0098_0000);
        e.mem.set_u32(this, table);
        // The slot writes only the low byte, as a bool.
        e.register(0x0098_0000, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |_, _| 0x6000u32.into_ret());
        e.register(BOOL_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_7710, &[this, 0]).u32(), 0x6000);
        let built: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == BOOL_INTERPOLATOR_CONSTRUCT)
            .collect();
        assert_eq!(
            built,
            vec![(
                BOOL_INTERPOLATOR_CONSTRUCT,
                vec![0x6000, (this & !0xff) | 1]
            )]
        );
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_7710, &[this, 0]).u32(), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bool_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a577b0, &args![]).u32(),
            NI_BOOL_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a57c00_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_7c00;
        const TARGET: u32 = 0x00a3_9010;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a57c10_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_7c10;
        const TARGET: u32 = 0x00a3_9030;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    const VALUE_SLOT: u32 = 0x0001_0100;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn synchronize_pose_interpolator_sets_the_target_value() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| word(a[0] + 0x10));
        e.register(BOOL_INTERPOLATOR_SET_POSE_VALUE, |_, _| Ret::default());
        e.register(VALUE_SLOT, |e, a| {
            // The slot holds the interpolator's word; the value goes in
            // its low byte.
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], (word & 0xffff_ff00) | 1);
            Ret::default()
        });
        let mut slots = vec![0u32; 0x3c];
        slots[0xec / 4] = VALUE_SLOT;
        e.put_vtable(0x0002_0000, &slots);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x0002_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a5_77c0, &args![this, 0x5000u32, 0x77u32]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x5000, NI_BOOL_INTERPOLATOR_RTTI]]
        );
        assert_eq!(calls(&e, VALUE_SLOT)[0][0], this);
        assert_eq!(
            calls(&e, BOOL_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![0x5010, 0x5001]]
        );
        // A null interpolator is not cast.
        e.call_log = Some(vec![]);
        e.call(0x00a5_77c0, &args![this, 0u32, 0u32]);
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
        assert_eq!(
            calls(&e, BOOL_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![0, 1]]
        );
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        for (capacity, grows) in [(4u16, false), (2, true)] {
            let mut e = Engine::new();
            e.map(NI_BOOL_INTERP_CONTROLLER_RTTI, 4);
            e.set_global(NI_BOOL_INTERP_CONTROLLER_RTTI, 0x0109_4444u32);
            e.register(SINGLE_INTERP_CONTROLLER_GET_VIEWER_STRINGS, |_, _| {
                Ret::default()
            });
            e.register(NI_GET_VIEWER_STRING, |_, _| word(0xd1));
            e.register(STRINGS_GROW, |_, _| Ret::default());
            e.register(STRINGS_SET_AT, |e, a| {
                assert_eq!(e.mem.u32(a[2]), 0xd1);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            let this = e.mem.alloc(0x40);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, 2);
            e.mem.set_u16(strings + 0xe, 3);
            e.call(0x00a5_7800, &args![this, strings]);
            assert_eq!(
                calls(&e, SINGLE_INTERP_CONTROLLER_GET_VIEWER_STRINGS),
                vec![vec![this, strings]]
            );
            assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_4444]]);
            let grow = calls(&e, STRINGS_GROW);
            if grows {
                assert_eq!(grow, vec![vec![strings, 5]]);
            } else {
                assert!(grow.is_empty());
            }
            assert_eq!(calls(&e, STRINGS_SET_AT)[0][..2], [strings, 2]);
        }
    }
}
