//! `fallout shared/magic/effectarchetype.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `EffectItem`'s effect setting (`00825c00`, `ECX` = the effect item).
const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
/// The effect setting's archetype index (`00810fd0`, `ECX` = setting).
const SETTING_ARCHETYPE: u32 = 0x0081_0fd0;
/// The archetype table: 16-byte rows, the creation function (cdecl:
/// caster, magic item, effect item) at +0 of row `i` (`01183324 + i *
/// 0x10`).
const ARCHETYPE_CREATE: u32 = 0x0118_3324;
/// The new effect's source setter (`008041c0`, `this`; source).
const SET_SOURCE: u32 = 0x0080_41c0;

// Translated from 00403ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectArchetypes::Activate` (Xbox PDB, cdecl): creates the active
/// effect for `effect` through its setting's archetype row (the setting is
/// null for a null `effect`): a null creation function gives null; a
/// created effect gets `source` (`008041c0`). Returns the effect.
pub fn effect_archetypes_activate(
    e: &mut Engine,
    caster: u32,
    item: u32,
    effect: u32,
    source: u32,
) -> u32 {
    let mut created = 0u32;
    let setting = if effect == 0 {
        0
    } else {
        e.call(EFFECT_ITEM_SETTING, &args![effect]).u32()
    };
    let archetype = e.call(SETTING_ARCHETYPE, &args![setting]).u32();
    let create = e
        .mem
        .u32(ARCHETYPE_CREATE.wrapping_add(archetype.wrapping_shl(4)));
    if create != 0 {
        created = e.call(create, &args![caster, item, effect]).u32();
    }
    if created != 0 {
        e.call(SET_SOURCE, &args![created, source]);
    }
    created
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0040_3ab0,
        effect_archetypes_activate(u32, u32, u32, u32) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn activate_through_the_archetype_row() {
        let mut e = Engine::new();
        e.map(0x0118_3000, 0x1000);
        e.mem.set_u32(ARCHETYPE_CREATE + 2 * 0x10, 0x00b2_0000);
        e.register(EFFECT_ITEM_SETTING, |_, _| Ret {
            eax: 0x700,
            ..Ret::default()
        });
        e.register(SETTING_ARCHETYPE, |_, a| Ret {
            eax: if a[0] == 0x700 { 2 } else { 3 },
            ..Ret::default()
        });
        e.register(0x00b2_0000, |_, _| Ret {
            eax: 0x900,
            ..Ret::default()
        });
        e.register(SET_SOURCE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let r = e.call(0x0040_3ab0, &args![1u32, 2u32, 3u32, 4u32]).u32();
        assert_eq!(r, 0x900);
        assert_eq!(calls_to(&e, 0x00b2_0000), vec![vec![1, 2, 3]]);
        assert_eq!(calls_to(&e, SET_SOURCE), vec![vec![0x900, 4]]);
        // A null effect: setting 0, row 3 has no creation function.
        e.call_log = Some(vec![]);
        assert_eq!(effect_archetypes_activate(&mut e, 1, 2, 0, 4), 0);
        assert_eq!(calls_to(&e, SETTING_ARCHETYPE), vec![vec![0]]);
        assert!(calls_to(&e, EFFECT_ITEM_SETTING).is_empty());
        assert!(calls_to(&e, SET_SOURCE).is_empty());
    }
}
