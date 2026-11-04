//! `fallout shared/bgstouchspellform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `.?AVTESForm@@` and `.?AVMagicItemForm@@`: the type descriptors of the cast.
const TES_FORM_TYPE: u32 = 0x0118_3028;
const MAGIC_ITEM_FORM_TYPE: u32 = 0x0118_395c;
/// "MASTERFILE: Unable to find Actor Effect (%08X) on owner object \"%s\"."
const MISSING_ON_OWNER_NAME: u32 = 0x0101_aff8;
/// "MASTERFILE: Unable to find Actor Effect (%08X) on owner object (%08X)."
const MISSING_ON_OWNER_ID: u32 = 0x0101_afb0;
/// "MASTERFILE: Unable to find Actor Effect (%08X) on unknown owner."
const MISSING_ON_UNKNOWN_OWNER: u32 = 0x0101_af68;
/// "MASTERFILE: Enchanting (%08X) '%s' has invalid effect trigger type or effect for object
/// (%08X) '%s'."
const INVALID_ENCHANTMENT: u32 = 0x0101_af00;
/// The warning printer (`005b5e40`, cdecl, a format and its words).
const WARNING: u32 = 0x005b_5e40;

// Translated from 0047beb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTouchSpellForm::InitItem` (Xbox PDB; `ECX` and one stack word, the owner form,
/// `RET 4`): when the word at +4 (the spell's form ID as loaded) is not 0:
/// - fixes the ID up (with an owner only) in a stack slot through `00485d50` (cdecl, the
///   slot and the owner's file from `00484e60` with -1), looks it up with `004839c0`, casts
///   it to `MagicItemForm` with `__RTDynamicCast` and stores the result at +4; a null result
///   prints a warning with the ID (without an owner the "unknown owner" text; with one, its
///   name, vtable slot +0x130, when `00474cb0` on it is nonzero, else the word `0084e3a0`
///   gives);
/// - when the spell is set and the type its magic item (at +0x18) answers through vtable
///   slot +0x18 is 6, and there is an owner, `004094f0` checks the spell against the owner;
///   when that answers false the "Enchanting ... invalid effect trigger type" warning is
///   printed (the spell's `0084e3a0` word and name, then the owner's) and +4 is cleared.
pub fn bgs_touch_spell_form_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
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
    let spell = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, TES_FORM_TYPE, MAGIC_ITEM_FORM_TYPE, 0u32],
        )
        .u32();
    e.mem.set_u32(base + 4, spell);
    if e.mem.u32(base + 4) == 0 {
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
    let spell = e.mem.u32(base + 4);
    let enchantment = if spell != 0 {
        let kind = e.vcall(spell + 0x18, 0x18, &args![]).u32();
        if kind == 6 {
            e.mem.u32(base + 4)
        } else {
            0
        }
    } else {
        0
    };
    if enchantment == 0 || owner.is_null() {
        return;
    }
    if e.call(0x0040_94f0, &args![enchantment, owner]).bool() {
        return;
    }
    let owner_name = e.vcall(owner.addr(), 0x130, &args![]).u32();
    let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
    let spell = e.mem.u32(base + 4);
    let spell_name = e.vcall(spell, 0x130, &args![]).u32();
    let spell = e.mem.u32(base + 4);
    let spell_id = e.call(0x0084_e3a0, &args![spell]).u32();
    e.call(
        WARNING,
        &args![
            INVALID_ENCHANTMENT,
            spell_id,
            spell_name,
            owner_id,
            owner_name
        ],
    );
    e.mem.set_u32(base + 4, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0047_beb0,
        bgs_touch_spell_form_init_item(Ptr, Ptr)
    )]
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

    /// A form whose vtable slot +0x130 answers `name`, with its magic item at +0x18 answering
    /// `kind` through slot +0x18.
    fn named_form(e: &mut Engine, name: u32, kind: u32) -> u32 {
        let form = e.mem.alloc(0x20);
        let table = e.mem.alloc(0x140);
        let item_table = e.mem.alloc(0x20);
        let name_fn = 0x0310_0000 + name;
        let kind_fn = 0x0320_0000 + kind;
        e.mem.set_u32(table + 0x130, name_fn);
        e.mem.set_u32(item_table + 0x18, kind_fn);
        e.mem.set_u32(form, table);
        e.mem.set_u32(form + 0x18, item_table);
        e.register_double(name_fn, move |_, _| Ret {
            eax: name,
            ..Ret::default()
        });
        e.register_double(kind_fn, move |_, _| Ret {
            eax: kind,
            ..Ret::default()
        });
        form
    }

    /// Doubles: the lookup gives the spell `spell` for ID 0x110 (after the owner fix-up) and
    /// for 0x10 (without one), 0 otherwise; the owner check answers `named`, the
    /// enchantment check `valid`; `0084e3a0` answers the form + 1.
    fn touch_case(e: &mut Engine, spell: u32, named: u32, valid: bool) {
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
        e.register_double(0x00ec_43fb, move |_, a| Ret {
            eax: if a[0] == 0x110 || a[0] == 0x10 {
                spell
            } else {
                0
            },
            ..Ret::default()
        });
        e.register_double(0x0047_4cb0, move |_, _| Ret {
            eax: named,
            ..Ret::default()
        });
        e.register_double(0x0040_94f0, move |_, _| Ret {
            eax: u32::from(valid),
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(WARNING, |_, _| Ret::default());
    }

    #[test]
    fn touch_spell_init_item_does_nothing_without_an_id() {
        let mut e = Engine::new();
        let component = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0047_beb0, &args![component, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn touch_spell_init_item_warns_about_a_missing_spell() {
        for (named, with_owner, expected) in [
            (0u32, false, vec![MISSING_ON_UNKNOWN_OWNER, 0x3000]),
            (1, true, vec![MISSING_ON_OWNER_NAME, 0x3100, 0x0abc]),
            (0, true, vec![MISSING_ON_OWNER_ID, 0x3100, 0]),
        ] {
            let mut e = Engine::new();
            touch_case(&mut e, 0x1234, named, true);
            let owner = if with_owner {
                named_form(&mut e, 0x0abc, 0)
            } else {
                0
            };
            let component = e.mem.alloc(8);
            e.mem.set_u32(component + 4, 0x3000);
            e.call_log = Some(vec![]);
            e.call(0x0047_beb0, &args![component, owner]);
            let log = e.call_log.take().unwrap();
            let mut expected = expected;
            if expected[0] == MISSING_ON_OWNER_ID {
                expected[2] = owner + 1;
            }
            assert_eq!(calls_to(&log, WARNING), vec![expected]);
            assert_eq!(e.mem.u32(component + 4), 0);
            assert!(calls_to(&log, 0x0040_94f0).is_empty());
        }
    }

    #[test]
    fn touch_spell_init_item_checks_an_enchantment_against_the_owner() {
        // (kind the spell answers, owner present, the check's answer, warns)
        for (kind, with_owner, valid, warns) in [
            (5u32, true, false, false),
            (6, false, false, false),
            (6, true, true, false),
            (6, true, false, true),
        ] {
            let mut e = Engine::new();
            let spell = named_form(&mut e, 0x05e1, kind);
            touch_case(&mut e, spell, 0, valid);
            let owner = if with_owner {
                named_form(&mut e, 0x0abc, 0)
            } else {
                0
            };
            let component = e.mem.alloc(8);
            e.mem.set_u32(component + 4, 0x10);
            e.call_log = Some(vec![]);
            e.call(0x0047_beb0, &args![component, owner]);
            let log = e.call_log.take().unwrap();
            let checks = calls_to(&log, 0x0040_94f0);
            if kind == 6 && with_owner {
                assert_eq!(checks, vec![vec![spell, owner]]);
            } else {
                assert!(checks.is_empty());
            }
            if warns {
                assert_eq!(
                    calls_to(&log, WARNING),
                    vec![vec![
                        INVALID_ENCHANTMENT,
                        spell + 1,
                        0x05e1,
                        owner + 1,
                        0x0abc
                    ]]
                );
                assert_eq!(e.mem.u32(component + 4), 0);
            } else {
                assert!(calls_to(&log, WARNING).is_empty());
                assert_eq!(e.mem.u32(component + 4), spell);
            }
        }
    }
}
