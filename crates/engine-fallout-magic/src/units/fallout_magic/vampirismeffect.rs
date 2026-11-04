//! `fallout/magic/vampirismeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00804090` (named `ActiveEffect::CopyData` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const COPY_DATA_TARGET_00804090: u32 = 0x0080_4090;

// Translated from 00829b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VampirismEffect::CopyData` (Xbox PDB `?CopyData@VampirismEffect@@MAAXPAVActiveEffect@@@Z`,
/// void, vtable word 0107a110 and others): forwards `this` and the `ActiveEffect` pointer to
/// the base body `00804090` (`ActiveEffect::CopyData`). Returns nothing.
pub fn vampirism_effect_copy_data(e: &mut Engine, this: Ptr, target: Ptr) {
    e.call(COPY_DATA_TARGET_00804090, &args![this, target]);
}

/// The `ActiveEffect` constructor (`00803d30`), thiscall (caster, spell,
/// effect item), `RET 0xc`.
const ACTIVE_EFFECT_CONSTRUCT: u32 = 0x0080_3d30;
/// `VampirismEffect`'s vtable (`0107c7bc`).
const VAMPIRISM_EFFECT_VTABLE: u32 = 0x0107_c7bc;
/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `ActiveEffect`'s virtual slot +0x44 (`CopyData`, Xbox PDB order).
const SLOT_COPY_DATA: u32 = 0x44;
/// `ActiveEffect::InitLoadGame` (engine map), thiscall (word), `RET 4`.
const ACTIVE_EFFECT_INIT_LOAD_GAME: u32 = 0x0080_5670;
/// `__RTDynamicCast` (CRT), cdecl, and the type descriptors of
/// `MagicTarget`, `Actor` and `Character`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_MAGIC_TARGET: u32 = 0x011a_0ef8;
const TYPE_ACTOR: u32 = 0x0118_46d4;
const TYPE_CHARACTER: u32 = 0x011a_0f14;

// Translated from 0082cf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `VampirismEffect` constructor (unnamed in the engine map;
/// `thiscall`, `RET 0xc`): the `ActiveEffect` constructor with the caster,
/// the spell and the effect item, then the class's vtable (`0107c7bc`).
/// Returns `this`.
pub fn fn_0082cf80(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(ACTIVE_EFFECT_CONSTRUCT, &args![this, caster, spell, effect]);
    e.mem.set_u32(this.addr(), VAMPIRISM_EFFECT_VTABLE);
    this
}

// Translated from 0082cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082cfb0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new effect) is not translated.
/// `VampirismEffect::Copy` (engine map; `thiscall`, plain `RET`): a new
/// effect (0x48 bytes, `0082cf80` with this effect's caster (+0x28), spell
/// (+8) and effect item (+0xc); 0 when the allocation failed) that this
/// effect's `CopyData` (virtual slot +0x44) fills. Returns it.
pub fn vampirism_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let copy = if memory == 0 {
        0
    } else {
        let caster = e.mem.u32(this.addr() + 0x28);
        let spell = e.mem.u32(this.addr() + 8);
        let effect = e.mem.u32(this.addr() + 0xc);
        fn_0082cf80(e, Ptr::new(memory), caster, spell, effect).addr()
    };
    e.vcall(this.addr(), SLOT_COPY_DATA, &args![copy]);
    copy
}

// Translated from 0082d060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether `target` (a
/// `MagicTarget`) is a `Character` (`__RTDynamicCast`).
pub fn fn_0082d060(e: &mut Engine, _this: Ptr, target: u32) -> bool {
    e.call(
        RT_DYNAMIC_CAST,
        &args![target, 0u32, TYPE_MAGIC_TARGET, TYPE_CHARACTER, 0u32],
    )
    .u32()
        != 0
}

// Translated from 0082d090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082d090 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new effect) is not translated.
/// Unnamed in the engine map (cdecl, three words): a new `VampirismEffect`
/// (0x48 bytes, `0082cf80` with the caster, the spell and the effect item),
/// or 0 when the allocation failed.
pub fn fn_0082d090(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    if memory == 0 {
        return 0;
    }
    fn_0082cf80(e, Ptr::new(memory), caster, spell, effect).addr()
}

// Translated from 0082d110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VampirismEffect::InitLoadGame` (engine map; `thiscall`, `RET 4`):
/// `ActiveEffect::InitLoadGame(word)`, then the word cast from `Actor` to
/// `Character` (`__RTDynamicCast`); the cast's result is not used.
pub fn vampirism_effect_init_load_game(e: &mut Engine, this: Ptr, word: u32) {
    e.call(ACTIVE_EFFECT_INIT_LOAD_GAME, &args![this, word]);
    e.call(
        RT_DYNAMIC_CAST,
        &args![word, 0u32, TYPE_ACTOR, TYPE_CHARACTER, 0u32],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_9b90, vampirism_effect_copy_data(Ptr, Ptr)),
        entry!(0x0082_cf80, fn_0082cf80(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0082_cfb0, vampirism_effect_copy(Ptr) -> u32),
        entry!(0x0082_d060, fn_0082d060(Ptr, u32) -> bool),
        entry!(0x0082_d090, fn_0082d090(u32, u32, u32) -> u32),
        entry!(0x0082_d110, vampirism_effect_init_load_game(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_vampirism_effect_copy_data_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(COPY_DATA_TARGET_00804090, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_9b90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == COPY_DATA_TARGET_00804090)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod vampirism_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0082_cf80, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCT),
            vec![vec![this, 1, 2, 3]]
        );
        assert_eq!(e.mem.u32(this), VAMPIRISM_EFFECT_VTABLE);
    }

    #[test]
    fn copy_makes_a_new_effect_and_fills_it() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x48);
        let vtable = e.mem.alloc(0x48);
        e.mem.set_u32(vtable + 0x44, 0x00f5_0000);
        e.register(0x00f5_0000, |_, _| Ret::default());
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x28, 0xa);
        e.mem.set_u32(this + 8, 0xb);
        e.mem.set_u32(this + 0xc, 0xc);
        e.call_log = Some(vec![]);
        let copy = e.call(0x0082_cfb0, &args![this]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x48]]);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCT),
            vec![vec![copy, 0xa, 0xb, 0xc]]
        );
        assert_eq!(calls(&e, 0x00f5_0000), vec![vec![this, copy]]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(vampirism_effect_copy(&mut e, Ptr::new(this)), 0);
        assert_eq!(calls(&e, 0x00f5_0000), vec![vec![this, 0]]);
    }

    #[test]
    fn target_must_be_a_character() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| {
            if a[0] == 0x9000 { 0 } else { a[0] + 4 }.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0082_d060, &args![0x100u32, 0x5000u32]).bool());
        assert!(!fn_0082d060(&mut e, Ptr::new(0x100), 0x9000));
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST)[0],
            vec![0x5000, 0, TYPE_MAGIC_TARGET, TYPE_CHARACTER, 0]
        );
    }

    #[test]
    fn create_or_zero() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let made = e.call(0x0082_d090, &args![1u32, 2u32, 3u32]).u32();
        assert_eq!(e.mem.u32(made), VAMPIRISM_EFFECT_VTABLE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(fn_0082d090(&mut e, 1, 2, 3), 0);
    }

    #[test]
    fn init_load_game_then_the_cast() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_INIT_LOAD_GAME, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_d110, &args![0x100u32, 0x700u32]);
        let log: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|(a, _)| a)
            .filter(|a| *a != 0x0082_d110)
            .collect();
        assert_eq!(log, vec![ACTIVE_EFFECT_INIT_LOAD_GAME, RT_DYNAMIC_CAST]);
        e.call_log = Some(vec![]);
        vampirism_effect_init_load_game(&mut e, Ptr::new(0x100), 0x700);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![0x700, 0, TYPE_ACTOR, TYPE_CHARACTER, 0]]
        );
    }
}
