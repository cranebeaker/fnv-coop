//! `fallout shared/tesraceform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The type descriptors the casts here use: `.?AVBaseFormComponent@@`, `.?AVTESRaceForm@@`,
/// `.?AVTESForm@@` and `.?AVTESRace@@`.
const BASE_FORM_COMPONENT_TYPE: u32 = 0x0118_3040;
const RACE_FORM_TYPE: u32 = 0x0118_6bc0;
const TES_FORM_TYPE: u32 = 0x0118_3028;
const RACE_TYPE: u32 = 0x0118_6370;
/// "MASTERFILE: Unable to find race (%08X) on owner object \"%s\"."
const MISSING_ON_OWNER_NAME: u32 = 0x0101_d424;
/// "MASTERFILE: Unable to find race (%08X) on owner object (%08X)."
const MISSING_ON_OWNER_ID: u32 = 0x0101_d3e4;
/// "MASTERFILE: Unable to find race (%08X) on unknown owner."
const MISSING_ON_UNKNOWN_OWNER: u32 = 0x0101_d3a8;

// Translated from 0048b870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRaceForm::CopyComponent` (Xbox PDB; `ECX` and one stack word, the source component,
/// `RET 4`): casts the source from `BaseFormComponent` to `TESRaceForm` with
/// `__RTDynamicCast` (`00ec43fb`); when that is not null, gives its race (the word
/// `00726070` reads at +4) to `006ecd40` on this component.
pub fn tes_race_form_copy_component(e: &mut Engine, this: Ptr, source: Ptr) {
    let race_form = e
        .call(
            0x00ec_43fb,
            &args![source, 0u32, BASE_FORM_COMPONENT_TYPE, RACE_FORM_TYPE, 0u32],
        )
        .u32();
    if race_form == 0 {
        return;
    }
    let race = e.call(0x0072_6070, &args![race_form]).u32();
    e.call(0x006e_cd40, &args![this, race]);
}

// Translated from 0048b920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRaceForm::InitItem` (Xbox PDB; `ECX` and one stack word, the owner form, `RET 4`):
/// when the word at +4 (the race's form ID as loaded) is not 0, fixes it up in a stack slot
/// through `00485d50` (cdecl, the slot and the owner's file from `00484e60` with -1; the
/// owner is used here without a null check), looks it up with `004839c0`, casts it to
/// `TESRace` with `__RTDynamicCast` and stores the result at +4. When that is null a warning
/// is printed with the ID (`005b5e40`): without an owner the "unknown owner" text; with one,
/// its name (vtable slot +0x130) when `00474cb0` on it is nonzero, else the word
/// `0084e3a0` gives for it.
pub fn tes_race_form_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    let base = this.addr();
    let loaded = e.mem.u32(base + 4);
    if loaded == 0 {
        return;
    }
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), loaded);
        let file = e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let race = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, TES_FORM_TYPE, RACE_TYPE, 0u32],
        )
        .u32();
    e.mem.set_u32(base + 4, race);
    if e.mem.u32(base + 4) != 0 {
        return;
    }
    if owner.is_null() {
        e.call(0x005b_5e40, &args![MISSING_ON_UNKNOWN_OWNER, id]);
    } else if e.call(0x0047_4cb0, &args![owner]).u32() != 0 {
        let name = e.vcall(owner.addr(), 0x130, &args![]).u32();
        e.call(0x005b_5e40, &args![MISSING_ON_OWNER_NAME, id, name]);
    } else {
        let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
        e.call(0x005b_5e40, &args![MISSING_ON_OWNER_ID, id, owner_id]);
    }
}

// Translated from 0048b8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRaceForm::CompareComponent` (engine map; `RET 4`): true when `other`
/// is no `TESRaceForm` (`__RTDynamicCast` from `BaseFormComponent`) or when
/// the races (the word `00726070` reads at +4, this one's first) differ.
pub fn tes_race_form_compare_component(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let race_form = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, BASE_FORM_COMPONENT_TYPE, RACE_FORM_TYPE, 0u32],
        )
        .u32();
    if race_form == 0 {
        return true;
    }
    let mine = e.call(0x0072_6070, &args![this]).u32();
    let theirs = e.call(0x0072_6070, &args![race_form]).u32();
    mine != theirs
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_b870, tes_race_form_copy_component(Ptr, Ptr)),
        entry!(0x0048_b920, tes_race_form_init_item(Ptr, Ptr)),
        entry!(
            0x0048_b8c0,
            tes_race_form_compare_component(Ptr, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn copy_component_copies_the_race_of_a_race_form() {
        for (cast, copies) in [(0u32, false), (0x6000, true)] {
            let mut e = Engine::new();
            e.register_double(0x00ec_43fb, move |_, _| Ret {
                eax: cast,
                ..Ret::default()
            });
            e.register(0x0072_6070, |_, a| Ret {
                eax: a[0] + 4,
                ..Ret::default()
            });
            e.register(0x006e_cd40, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0048_b870, &args![0x5000u32, 0x7000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, 0x00ec_43fb),
                vec![vec![0x7000, 0, BASE_FORM_COMPONENT_TYPE, RACE_FORM_TYPE, 0]]
            );
            if copies {
                assert_eq!(calls_to(&log, 0x006e_cd40), vec![vec![0x5000, 0x6004]]);
            } else {
                assert_eq!(log.len(), 2);
            }
        }
    }

    /// Doubles for `InitItem`: IDs below 0x2000 are found, the fix-up adds 0x100, the owner
    /// check answers `named`, the name is 0x7777 and the owner's word 0xbeef.
    fn race_case(e: &mut Engine, named: u32) -> u32 {
        let owner = e.mem.alloc(0x20);
        e.put_vtable(0x0300_0000, &[0; 0x50]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0300_1130);
        e.mem.set_u32(owner, 0x0300_0000);
        e.register(0x0300_1130, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x100);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: if a[0] < 0x2000 { a[0] + 1 } else { 0 },
            ..Ret::default()
        });
        e.register_double(0x0047_4cb0, move |_, _| Ret {
            eax: named,
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, _| Ret {
            eax: 0xbeef,
            ..Ret::default()
        });
        e.register(0x005b_5e40, |_, _| Ret::default());
        owner
    }

    #[test]
    fn race_init_item_resolves_the_race() {
        let mut e = Engine::new();
        let owner = race_case(&mut e, 0);
        let component = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0048_b920, &args![component, owner]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);

        e.mem.set_u32(component + 4, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0048_b920, &args![component, owner]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(component + 4), 0x111);
        assert_eq!(calls_to(&log, 0x0048_4e60), vec![vec![owner, 0xffff_ffff]]);
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![vec![0x110, 0, TES_FORM_TYPE, RACE_TYPE, 0]]
        );
        assert!(calls_to(&log, 0x005b_5e40).is_empty());
    }

    #[test]
    fn race_init_item_warns_about_a_missing_race() {
        for (named, with_owner, expected) in [
            (0u32, false, vec![MISSING_ON_UNKNOWN_OWNER, 0x3100]),
            (1, true, vec![MISSING_ON_OWNER_NAME, 0x3100, 0x7777]),
            (0, true, vec![MISSING_ON_OWNER_ID, 0x3100, 0xbeef]),
        ] {
            let mut e = Engine::new();
            let owner = race_case(&mut e, named);
            let owner = if with_owner { owner } else { 0 };
            let component = e.mem.alloc(8);
            e.mem.set_u32(component + 4, 0x3000);
            e.call_log = Some(vec![]);
            e.call(0x0048_b920, &args![component, owner]);
            let log = e.call_log.take().unwrap();
            // The owner's file is asked for even without an owner.
            assert_eq!(calls_to(&log, 0x0048_4e60), vec![vec![owner, 0xffff_ffff]]);
            assert_eq!(calls_to(&log, 0x005b_5e40), vec![expected]);
            assert_eq!(e.mem.u32(component + 4), 0);
        }
    }
}

#[cfg(test)]
mod compare_tests {
    use super::*;

    #[test]
    fn compare_component_reports_other_races_and_non_race_forms() {
        let mut e = Engine::new();
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let this = e.mem.alloc(8);
        let same = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x6000);
        e.mem.set_u32(same + 4, 0x6000);
        e.mem.set_u32(other + 4, 0x6001);
        assert!(!e.call(0x0048_b8c0, &args![this, same]).bool());
        assert!(e.call(0x0048_b8c0, &args![this, other]).bool());
        e.register(0x00ec_43fb, |_, _| 0u32.into_ret());
        assert!(e.call(0x0048_b8c0, &args![this, same]).bool());
    }
}
