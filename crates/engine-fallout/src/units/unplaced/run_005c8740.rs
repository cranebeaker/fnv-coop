//! `(unplaced)/run_005c8740` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `Script::ParseParameters` (`005accb0`, cdecl, varargs).
const PARSE_PARAMETERS: u32 = 0x005a_ccb0;
/// A reference test (`00440da0`, `modelloader.cpp`; thiscall, a `bool`):
/// the command gives up on a reference for which it answers true.
const REFERENCE_TEST_00440DA0: u32 = 0x0044_0da0;
/// `PackageLocation::PackageLocation` (`0067f030`), `SetLocReference`
/// (`0067f3c0`, one word) and the destructor (`0067f110`); a location is
/// 12 bytes on the stack.
const PACKAGE_LOCATION_CONSTRUCT: u32 = 0x0067_f030;
const PACKAGE_LOCATION_SET_REFERENCE: u32 = 0x0067_f3c0;
const PACKAGE_LOCATION_DESTRUCT: u32 = 0x0067_f110;
const PACKAGE_LOCATION_SIZE: u32 = 12;
/// `TESObjectREFR::AddTalkingActivatorMobileObjectREFR` (`00578950`).
const TALKING_ACTIVATOR_MOBILE_OBJECT: u32 = 0x0057_8950;
/// `Script::IsPendingFadeDisabledReference` (`005aa6d0`, cdecl, one word).
const IS_PENDING_FADE_DISABLED: u32 = 0x005a_a6d0;
/// `TESObjectREFR::Activate` (`00573170`, thiscall, four words).
const ACTIVATE: u32 = 0x0057_3170;
/// `MiddleHighProcess::GetSavedAcquireObject` (`008d8520`, engine map).
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// `MobileObject::GetCurrentPackage` (`009344a0`, engine map), the
/// package's type (`0041ca90`) and the setter `005c8a30` (engine map
/// `MiddleHighProcess::SetBSBound`, folded) on the player.
const GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
const PACKAGE_TYPE_0041CA90: u32 = 0x0041_ca90;
const PLAYER_SET_005C8A30: u32 = 0x005c_8a30;
/// The player reference (`011dea3c`, the word the command compares with).
const PLAYER: u32 = 0x011d_ea3c;
/// Virtual slots of the reference: +0x100 (a `bool`, whether it is a
/// mobile object by its use), +0x214 (a word compared with 9) and +0x280
/// (nine words, a package start by its use here).
const SLOT_IS_MOBILE: u32 = 0x100;
const SLOT_214: u32 = 0x214;
const SLOT_START_PACKAGE: u32 = 0x280;

// Translated from 005c8740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005c8740 - the function's C++ exception frame (FS:[0], unwind states of the two PackageLocations) is not translated
/// A script command body (the eight script-command words, cdecl; unnamed
/// in the engine map). It parses a target reference, a word, two location
/// references and two flags (defaults 0, 0, 0, 0, 1, 0); false when that
/// fails, otherwise true. With a reference `this_obj` and a target, neither
/// refused by `00440da0`, it builds two `PackageLocation`s (set from the
/// location references given) and, when both sides are mobile objects
/// (slot +0x100; the target may be given one through
/// `AddTalkingActivatorMobileObjectREFR`) that pass the checks (`00440da0`,
/// slot +0x214 not 9, not pending a fade-disable), starts the package
/// through the reference's slot +0x280 (when both have a saved acquire
/// object), afterwards handing the reference's current package to the
/// player's setter when its type is 0x1c; a reference that is the player
/// is activated by the target instead (`Activate(player, 0, 0, 1)`), and
/// a failed check activates the target from the reference
/// (`Activate(target, 0, 0, 1)` on `this_obj`). `result` is not used.
pub fn fn_005c8740(
    e: &mut Engine,
    param_info: u32,
    script_data: u32,
    this_obj: u32,
    containing_obj: u32,
    script_obj: u32,
    event_list: u32,
    _result: u32,
    opcode_offset: u32,
) -> bool {
    // The six parsed values: target, word, two location references and
    // the two flags.
    let parsed = e.with_stack(24, |e, block| {
        let outs: Vec<u32> = (0..6).map(|i| block.addr() + 4 * i).collect();
        for (out, value) in outs.iter().zip([0u32, 0, 0, 0, 1, 0]) {
            e.mem.set_u32(*out, value);
        }
        let mut words = args![
            param_info,
            script_data,
            opcode_offset,
            this_obj,
            containing_obj,
            script_obj,
            event_list
        ];
        words.extend_from_slice(&outs);
        let ok = e.call(PARSE_PARAMETERS, &words).u32() & 0xff != 0;
        let values: Vec<u32> = outs.iter().map(|out| e.mem.u32(*out)).collect();
        ok.then_some(values)
    });
    let Some(values) = parsed else {
        return false;
    };
    let (target, word, location_1, location_2, flag_1, flag_2) = (
        values[0], values[1], values[2], values[3], values[4], values[5],
    );
    if this_obj == 0
        || target == 0
        || e.call(REFERENCE_TEST_00440DA0, &args![this_obj]).u32() & 0xff != 0
        || e.call(REFERENCE_TEST_00440DA0, &args![target]).u32() & 0xff != 0
    {
        return true;
    }
    e.with_stack(2 * PACKAGE_LOCATION_SIZE, |e, locations| {
        let first = locations.addr();
        let second = locations.addr() + PACKAGE_LOCATION_SIZE;
        let mut first_used = 0u32;
        let mut second_used = 0u32;
        e.call(PACKAGE_LOCATION_CONSTRUCT, &args![first]);
        e.call(PACKAGE_LOCATION_CONSTRUCT, &args![second]);
        if location_1 != 0 {
            first_used = first;
            e.call(PACKAGE_LOCATION_SET_REFERENCE, &args![first, location_1]);
        }
        if location_2 != 0 {
            second_used = second;
            e.call(PACKAGE_LOCATION_SET_REFERENCE, &args![second, location_2]);
        }
        let mut actor = 0u32;
        if e.vcall(this_obj, SLOT_IS_MOBILE, &args![]).u32() & 0xff != 0 {
            actor = this_obj;
        }
        let other = if e.vcall(target, SLOT_IS_MOBILE, &args![]).u32() & 0xff != 0 {
            target
        } else {
            e.call(TALKING_ACTIVATOR_MOBILE_OBJECT, &args![target])
                .u32()
        };
        let refused = actor == 0
            || other == 0
            || e.call(REFERENCE_TEST_00440DA0, &args![actor]).u32() & 0xff != 0
            || e.call(REFERENCE_TEST_00440DA0, &args![other]).u32() & 0xff != 0
            || e.vcall(actor, SLOT_214, &args![]).u32() == 9
            || e.call(IS_PENDING_FADE_DISABLED, &args![actor]).u32() & 0xff != 0
            || e.call(IS_PENDING_FADE_DISABLED, &args![other]).u32() & 0xff != 0;
        if refused {
            e.call(ACTIVATE, &args![this_obj, target, 0u32, 0u32, 1u32]);
        } else if actor == e.global::<u32>(PLAYER) {
            let player = e.global::<u32>(PLAYER);
            e.call(ACTIVATE, &args![other, player, 0u32, 0u32, 1u32]);
        } else if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32() != 0
            && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![other]).u32() != 0
        {
            e.vcall(
                actor,
                SLOT_START_PACKAGE,
                &args![
                    other,
                    first_used,
                    second_used,
                    u32::from(flag_1 == 1),
                    u32::from(flag_2 == 1),
                    0u32,
                    word,
                    0u32,
                    0u32
                ],
            );
            let package = e.call(GET_CURRENT_PACKAGE, &args![actor]).u32();
            if package != 0 && e.call(PACKAGE_TYPE_0041CA90, &args![package]).u32() == 0x1c {
                let player = e.global::<u32>(PLAYER);
                e.call(PLAYER_SET_005C8A30, &args![player, package]);
            }
        }
        e.call(PACKAGE_LOCATION_DESTRUCT, &args![second]);
        e.call(PACKAGE_LOCATION_DESTRUCT, &args![first]);
    });
    true
}

/// The registered form: the eight script-command words.
fn fn_005c8740_entry(e: &mut Engine, words: &[u32]) -> Ret {
    u32::from(fn_005c8740(
        e, words[0], words[1], words[2], words[3], words[4], words[5], words[6], words[7],
    ))
    .into_ret()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x005c_8740, fn_005c8740_entry as AbiFn)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0713_0000;
    const IS_MOBILE: u32 = 0x0056_0000;
    const SLOT_214_TARGET: u32 = 0x0057_0000;
    const START_PACKAGE: u32 = 0x0058_0000;

    /// References: +0 vtable, +4 mobile byte, +8 slot 0x214 answer, +0xc
    /// refused by `00440da0`, +0x10 pending fade, +0x14 saved acquire
    /// object.
    fn reference(e: &mut Engine) -> u32 {
        let reference = e.mem.alloc(0x18);
        e.mem.set_u32(reference, VTABLE);
        e.mem.set_u32(reference + 4, 1);
        e.mem.set_u32(reference + 0x14, 0x0999_0000);
        reference
    }

    fn setup(e: &mut Engine, target: u32, parse_ok: bool) {
        let mut slots = vec![0u32; 0xa1];
        slots[0x40] = IS_MOBILE;
        slots[0x85] = SLOT_214_TARGET;
        slots[0xa0] = START_PACKAGE;
        e.put_vtable(VTABLE, &slots);
        e.register(IS_MOBILE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SLOT_214_TARGET, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(START_PACKAGE, |_, _| Ret::default());
        e.register(REFERENCE_TEST_00440DA0, |e, a| {
            e.mem.u32(a[0] + 0xc).into_ret()
        });
        e.register(IS_PENDING_FADE_DISABLED, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(GET_SAVED_ACQUIRE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(TALKING_ACTIVATOR_MOBILE_OBJECT, |_, _| 0u32.into_ret());
        e.register(GET_CURRENT_PACKAGE, |_, _| 0x0aaa_0000u32.into_ret());
        e.register(PACKAGE_TYPE_0041CA90, |_, _| 0x1cu32.into_ret());
        for address in [
            PACKAGE_LOCATION_CONSTRUCT,
            PACKAGE_LOCATION_SET_REFERENCE,
            PACKAGE_LOCATION_DESTRUCT,
            ACTIVATE,
            PLAYER_SET_005C8A30,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.map(PLAYER, 4);
        e.mem.set_u32(PLAYER, 0x0bbb_0000);
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            assert_eq!(&a[..7], &[1, 2, 8, a[3], 4, 5, 6]);
            // Defaults as the command sets them.
            let defaults: Vec<u32> = a[7..].iter().map(|out| e.mem.u32(*out)).collect();
            assert_eq!(defaults, vec![0, 0, 0, 0, 1, 0]);
            for (out, value) in a[7..].iter().zip([target, 0x77, 0x0c00_0001, 0, 1, 2]) {
                e.mem.set_u32(*out, value);
            }
            u32::from(parse_ok).into_ret()
        });
    }

    fn run(e: &mut Engine, this_obj: u32) -> bool {
        e.call_log = Some(vec![]);
        e.call(0x005c_8740, &[1, 2, this_obj, 4, 5, 6, 7, 8]).u32() & 0xff != 0
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn starts_the_package_between_two_mobile_references() {
        let mut e = Engine::new();
        let actor = reference(&mut e);
        let target = reference(&mut e);
        setup(&mut e, target, true);
        assert!(run(&mut e, actor));
        let constructed = calls_to(&e, PACKAGE_LOCATION_CONSTRUCT);
        let (first, second) = (constructed[0][0], constructed[1][0]);
        assert_eq!(
            calls_to(&e, PACKAGE_LOCATION_SET_REFERENCE),
            vec![vec![first, 0x0c00_0001]]
        );
        assert_eq!(
            calls_to(&e, START_PACKAGE),
            vec![vec![actor, target, first, 0, 1, 0, 0, 0x77, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, PLAYER_SET_005C8A30),
            vec![vec![0x0bbb_0000, 0x0aaa_0000]]
        );
        assert_eq!(
            calls_to(&e, PACKAGE_LOCATION_DESTRUCT),
            vec![vec![second], vec![first]]
        );
        assert!(calls_to(&e, ACTIVATE).is_empty());
    }

    #[test]
    fn a_refused_reference_is_activated_instead() {
        let mut e = Engine::new();
        let actor = reference(&mut e);
        let target = reference(&mut e);
        setup(&mut e, target, true);
        e.mem.set_u32(actor + 8, 9);
        assert!(run(&mut e, actor));
        assert_eq!(calls_to(&e, ACTIVATE), vec![vec![actor, target, 0, 0, 1]]);
        // A target that is not mobile and gets no mobile object.
        e.mem.set_u32(actor + 8, 0);
        e.mem.set_u32(target + 4, 0);
        assert!(run(&mut e, actor));
        assert_eq!(calls_to(&e, ACTIVATE), vec![vec![actor, target, 0, 0, 1]]);
        assert_eq!(
            calls_to(&e, TALKING_ACTIVATOR_MOBILE_OBJECT),
            vec![vec![target]]
        );
    }

    #[test]
    fn the_player_is_activated_by_the_target() {
        let mut e = Engine::new();
        let target = reference(&mut e);
        setup(&mut e, target, true);
        e.map(0x0bbb_0000, 0x18);
        e.mem.set_u32(0x0bbb_0000, VTABLE);
        e.mem.set_u32(0x0bbb_0004, 1);
        assert!(run(&mut e, 0x0bbb_0000));
        assert_eq!(
            calls_to(&e, ACTIVATE),
            vec![vec![target, 0x0bbb_0000, 0, 0, 1]]
        );
        assert!(calls_to(&e, START_PACKAGE).is_empty());
    }

    #[test]
    fn parse_failure_and_missing_references() {
        let mut e = Engine::new();
        let actor = reference(&mut e);
        setup(&mut e, actor, false);
        assert!(!run(&mut e, actor));
        // No reference: true, nothing built.
        let mut e = Engine::new();
        let target = reference(&mut e);
        setup(&mut e, target, true);
        assert!(run(&mut e, 0));
        assert!(calls_to(&e, PACKAGE_LOCATION_CONSTRUCT).is_empty());
        // A refused target: true, nothing built.
        let actor = reference(&mut e);
        e.mem.set_u32(target + 0xc, 1);
        assert!(run(&mut e, actor));
        assert!(calls_to(&e, PACKAGE_LOCATION_CONSTRUCT).is_empty());
    }
}
