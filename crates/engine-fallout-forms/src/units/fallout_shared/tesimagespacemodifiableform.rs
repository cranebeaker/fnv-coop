//! `fallout shared/tesimagespacemodifiableform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (`00ec43fb`, cdecl: pointer, offset 0, source and target type
/// descriptors, 0).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` type descriptor (`.?AVTESForm@@`) every cast here starts from.
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// The target type descriptor of the cast (`.?AVTESImageSpaceModifier@@`).
const IMAGE_SPACE_MODIFIER_TYPE: u32 = 0x0118_62b8;
/// The warning printer (`005b5e40`, cdecl, a format and its words).
const WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find imagespacemodifier (%08X) on unknown owner."
const MISSING_ON_UNKNOWN_OWNER: u32 = 0x0101_cbe8;
/// "MASTERFILE: Unable to find imagespacemodifier (%08X) on owner object (%08X)."
const MISSING_ON_OWNER_ID: u32 = 0x0101_cc30;
/// "MASTERFILE: Unable to find imagespacemodifier (%08X) on owner object \"%s\"."
const MISSING_ON_OWNER_NAME: u32 = 0x0101_cc80;

// Translated from 00487490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifiableForm::InitItem` (Xbox PDB; `ECX` and one stack word, the owner
/// form, `RET 4`): when the word at +4 (the modifier's form ID as loaded) is not 0, turns it
/// into the modifier. With an owner, the ID is first fixed up in a stack slot through
/// `00485d50` (cdecl, the slot and the owner's file from `00484e60` with -1); the form
/// `004839c0` (cdecl) finds for it is cast to the image-space modifier type with
/// `__RTDynamicCast` and stored at +4. When that is null a warning is printed with the ID:
/// without an owner the "unknown owner" text; with one, its name (vtable slot +0x130) when
/// `00474cb0` on the owner is nonzero, else the word `0084e3a0` gives for the owner.
pub fn tes_image_space_modifiable_form_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    let base = this.addr();
    let loaded = e.mem.u32(base + 4);
    if loaded == 0 {
        return;
    }
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), loaded);
        if !owner.is_null() {
            let file = e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32();
            e.call(0x0048_5d50, &args![slot, file]);
        }
        e.mem.u32(slot.addr())
    });
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let modifier = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TES_FORM_TYPE, IMAGE_SPACE_MODIFIER_TYPE, 0u32],
        )
        .u32();
    e.mem.set_u32(base + 4, modifier);
    if e.mem.u32(base + 4) != 0 {
        return;
    }
    if owner.is_null() {
        e.call(WARNING, &args![MISSING_ON_UNKNOWN_OWNER, id]);
    } else if e.call(0x0047_4cb0, &args![owner]).u32() != 0 {
        let name = e.vcall(owner.addr(), 0x130, &args![]).u32();
        e.call(WARNING, &args![MISSING_ON_OWNER_NAME, id, name]);
    } else {
        let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
        e.call(WARNING, &args![MISSING_ON_OWNER_ID, id, owner_id]);
    }
}

/// The `BaseFormComponent` (`01183040`) and `TESImageSpaceModifiableForm`
/// (`01186ae8`) type descriptors (their names read at +8).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_MODIFIABLE_FORM: u32 = 0x0118_6ae8;
/// The getter of the word at +4 (`00726070`, `thiscall`) and its setter
/// (`006ecd40`, `thiscall`, one word; the engine map's name for it is folded
/// code).
const GET_MODIFIER: u32 = 0x0072_6070;
const SET_MODIFIER: u32 = 0x006e_cd40;

/// `other` cast from `BaseFormComponent` to `TESImageSpaceModifiableForm`.
fn cast_modifiable_form(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_MODIFIABLE_FORM,
            0u32
        ],
    )
    .u32()
}

// Translated from 00487570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifiableForm::CopyComponent` (Xbox PDB): when `other`
/// is a `TESImageSpaceModifiableForm`, takes its modifier (+4, through
/// `00726070` and `006ecd40`).
pub fn tes_image_space_modifiable_form_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = cast_modifiable_form(e, other);
    if source == 0 {
        return;
    }
    let modifier = e.call(GET_MODIFIER, &args![source]).u32();
    e.call(SET_MODIFIER, &args![this, modifier]);
}

// Translated from 004875c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifiableForm::CompareComponent` (Xbox PDB): true
/// (different) when `other` is no `TESImageSpaceModifiableForm` or the
/// modifiers (`00726070`) differ.
pub fn tes_image_space_modifiable_form_compare_component(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    let source = cast_modifiable_form(e, other);
    if source == 0 {
        return true;
    }
    let own = e.call(GET_MODIFIER, &args![this]).u32();
    own != e.call(GET_MODIFIER, &args![source]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0048_7490,
            tes_image_space_modifiable_form_init_item(Ptr, Ptr)
        ),
        entry!(
            0x0048_7570,
            tes_image_space_modifiable_form_copy_component(Ptr, Ptr)
        ),
        entry!(0x0048_75c0, tes_image_space_modifiable_form_compare_component(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles: the file of the owner is 0x9000, the fix-up adds 0x0100_0000 to the ID, the
    /// lookup gives `id + 0x10`, the cast gives `cast_result`, and the owner's name check
    /// answers `named`.
    fn init_case(e: &mut Engine, cast_result: u32, named: u32) {
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        e.register_double(DYNAMIC_CAST, move |_, _| Ret {
            eax: cast_result,
            ..Ret::default()
        });
        e.register_double(0x0047_4cb0, move |_, _| Ret {
            eax: named,
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, _| Ret {
            eax: 0x0000_beef,
            ..Ret::default()
        });
        e.register(WARNING, |_, _| Ret::default());
    }

    /// An owner form with a vtable whose slot +0x130 returns 0x7777.
    fn owner(e: &mut Engine) -> u32 {
        let owner = e.mem.alloc(0x20);
        e.put_vtable(0x0300_0000, &[0; 0x50]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0300_1130);
        e.mem.set_u32(owner, 0x0300_0000);
        e.register(0x0300_1130, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        owner
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn init_item_does_nothing_without_a_loaded_id() {
        let mut e = Engine::new();
        let form = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0048_7490, &args![form, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn init_item_resolves_the_modifier_with_the_owner_file() {
        let mut e = Engine::new();
        init_case(&mut e, 0x5555, 0);
        let form = e.mem.alloc(8);
        e.mem.set_u32(form + 4, 0x0000_1234);
        let owner = owner(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0048_7490, &args![form, owner]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(form + 4), 0x5555);
        assert_eq!(calls_to(&log, 0x0048_4e60), vec![vec![owner, 0xffff_ffff]]);
        assert_eq!(calls_to(&log, 0x0048_5d50)[0][1], 0x9000);
        assert_eq!(calls_to(&log, 0x0048_39c0), vec![vec![0x0100_1234]]);
        assert_eq!(
            calls_to(&log, DYNAMIC_CAST),
            vec![vec![
                0x0100_1244,
                0,
                TES_FORM_TYPE,
                IMAGE_SPACE_MODIFIER_TYPE,
                0
            ]]
        );
        assert!(calls_to(&log, WARNING).is_empty());
    }

    #[test]
    fn init_item_warns_about_a_missing_modifier() {
        // Without an owner: no fix-up, the unknown-owner text.
        let mut e = Engine::new();
        init_case(&mut e, 0, 0);
        let form = e.mem.alloc(8);
        e.mem.set_u32(form + 4, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0048_7490, &args![form, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0048_5d50).is_empty());
        assert_eq!(calls_to(&log, 0x0048_39c0), vec![vec![0x1234]]);
        assert_eq!(
            calls_to(&log, WARNING),
            vec![vec![MISSING_ON_UNKNOWN_OWNER, 0x1234]]
        );
        assert_eq!(e.mem.u32(form + 4), 0);

        // With a named owner: its name; with an unnamed one: its word.
        for (named, expected) in [
            (1u32, vec![MISSING_ON_OWNER_NAME, 0x0100_1234, 0x7777]),
            (0, vec![MISSING_ON_OWNER_ID, 0x0100_1234, 0xbeef]),
        ] {
            let mut e = Engine::new();
            init_case(&mut e, 0, named);
            let form = e.mem.alloc(8);
            e.mem.set_u32(form + 4, 0x1234);
            let owner = owner(&mut e);
            e.call_log = Some(vec![]);
            e.call(0x0048_7490, &args![form, owner]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, WARNING), vec![expected]);
        }
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(GET_MODIFIER, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SET_MODIFIER, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e
    }

    #[test]
    fn copy_component_takes_the_modifier() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other + 4, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0048_7570, &args![this, other]);
        assert_eq!(e.mem.u32(this + 4), 0x4444);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (
                DYNAMIC_CAST,
                vec![other, 0, TYPE_BASE_FORM_COMPONENT, TYPE_MODIFIABLE_FORM, 0]
            )
        );
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.mem.set_u32(other + 4, 0x5555);
        e.call(0x0048_7570, &args![this, other]);
        assert_eq!(e.mem.u32(this + 4), 0x4444);
    }

    #[test]
    fn compare_component_compares_the_modifier() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x4444);
        e.mem.set_u32(other + 4, 0x4444);
        assert!(!e.call(0x0048_75c0, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 0x4445);
        assert!(e.call(0x0048_75c0, &args![this, other]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.mem.set_u32(other + 4, 0x4444);
        assert!(e.call(0x0048_75c0, &args![this, other]).bool());
    }
}
