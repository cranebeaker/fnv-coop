//! `fallout shared/run_0047bdd0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor `.?AVBaseFormComponent@@`.
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// RTTI type descriptor `.?AVBGSTouchSpellForm@@`.
const TYPE_BGS_TOUCH_SPELL_FORM: u32 = 0x0118_6a8c;
/// `00726070` (`thiscall`): the word at +4.
const GET_WORD_4: u32 = 0x0072_6070;
/// `006ecd40` (`thiscall`, one word): stores it at +4 (the engine map's
/// name, `CombatProcedureAttackMelee::Initialize`, is a folded body).
const SET_WORD_4: u32 = 0x006e_cd40;
/// `004a8ae0` (`thiscall`): the 16-bit word at +8 (folded body; engine map
/// `TESActorBaseData::GetFatigue`).
const GET_SHORT_8: u32 = 0x004a_8ae0;
/// `00483170` (`thiscall`, one word): stores its low 16 bits at +8.
const SET_SHORT_8: u32 = 0x0048_3170;

// Translated from 0047bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `BGSTouchSpellForm` component (`thiscall (this; other)`; no Xbox
/// PDB name): when `other` (a `BaseFormComponent`) casts to
/// `BGSTouchSpellForm`, its word at +4 and its 16-bit word at +8 are stored
/// in `this` through their setters.
pub fn fn_0047bdd0(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_BGS_TOUCH_SPELL_FORM,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    let value = e.call(GET_WORD_4, &args![source]).u32();
    e.call(SET_WORD_4, &args![this, value]);
    let short = e.call(GET_SHORT_8, &args![source]).u16();
    e.call(SET_SHORT_8, &args![this, u32::from(short)]);
}

// Translated from 0047be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTouchSpellForm`'s compare (`thiscall`, `RET 4`; the engine map gives
/// it no name): true (they differ) when `other` is not a
/// `BGSTouchSpellForm` (`dynamic_cast` from `BaseFormComponent`), when the
/// words at +4 (`00726070`) differ or the 16-bit values at +8 (`004a8ae0`)
/// differ.
pub fn fn_0047be30(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_BGS_TOUCH_SPELL_FORM,
                0u32
            ],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let mine = e.call(GET_WORD_4, &args![this]).u32();
    let theirs = e.call(GET_WORD_4, &args![cast]).u32();
    if mine != theirs {
        return true;
    }
    let mine = e.call(GET_SHORT_8, &args![this]).u16();
    let theirs = e.call(GET_SHORT_8, &args![cast]).u16();
    mine != theirs
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_be30, fn_0047be30(Ptr, Ptr) -> bool),
        entry!(0x0047_bdd0, fn_0047bdd0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn copies_both_fields_from_a_touch_spell_form() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| {
            assert_eq!(
                a[1..],
                [0, TYPE_BASE_FORM_COMPONENT, TYPE_BGS_TOUCH_SPELL_FORM, 0]
            );
            word(a[0])
        });
        e.register(GET_WORD_4, |_, _| word(0x1234));
        e.register(GET_SHORT_8, |_, _| word(0xffff_0042));
        e.register(SET_WORD_4, |_, _| Ret::default());
        e.register(SET_SHORT_8, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_bdd0, &args![0x100u32, 0x200u32]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == SET_WORD_4 || c.0 == SET_SHORT_8)
            .collect();
        assert_eq!(
            log,
            vec![
                (SET_WORD_4, vec![0x100, 0x1234]),
                (SET_SHORT_8, vec![0x100, 0x42])
            ]
        );

        e.register(RT_DYNAMIC_CAST, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x0047_bdd0, &args![0x100u32, 0x200u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn at_0047be30_compares_both_fields() {
        let mut e = Engine::new();
        e.mem.map(0x0f00_0000, 4);
        e.register(RT_DYNAMIC_CAST, |e, _| word(e.mem.u32(0x0f00_0000)));
        e.register(GET_WORD_4, |e, a| word(e.mem.u32(a[0] + 4)));
        // The upper half of EAX is not compared (`MOVZX ...,AX`).
        e.register(GET_SHORT_8, |e, a| word(e.mem.u32(a[0] + 8)));
        let this = e.mem.alloc(0xc);
        let other = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_be30, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (
                RT_DYNAMIC_CAST,
                vec![
                    other,
                    0,
                    TYPE_BASE_FORM_COMPONENT,
                    TYPE_BGS_TOUCH_SPELL_FORM,
                    0
                ]
            )
        );
        e.mem.set_u32(0x0f00_0000, other);
        assert!(!e.call(0x0047_be30, &args![this, other]).bool());
        e.mem.set_u32(other + 8, 0x0001_0000);
        assert!(!e.call(0x0047_be30, &args![this, other]).bool());
        e.mem.set_u32(other + 8, 2);
        assert!(e.call(0x0047_be30, &args![this, other]).bool());
        e.mem.set_u32(other + 8, 0);
        e.mem.set_u32(other + 4, 2);
        assert!(e.call(0x0047_be30, &args![this, other]).bool());
    }
}
