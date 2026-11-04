//! `fallout/ai/run_008c1d20` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00409700` (unnamed, engine map unit `enchantmentitem.cpp`, `thiscall` on the item, no
/// stack words; a `bool` in `AL`).
const ITEM_TEST_00409700: u32 = 0x0040_9700;
/// `00726070` (unnamed, engine map unit `menumanager.cpp`, `thiscall`, no stack words), here on
/// the item's word list `+0x74`.
const LIST_FIRST_00726070: u32 = 0x0072_6070;
/// The actor's vtable slot `+0x17c` (ten stack words).
const ACTOR_SLOT_17C: u32 = 0x17c;
/// `MagicCaster::CastSpell` (engine map, `00815480`; `thiscall` on the actor's `MagicCaster`
/// `+0x88`, three stack words).
const MAGIC_CASTER_CAST_SPELL: u32 = 0x0081_5480;
/// The player (`PlayerCharacter*`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `0094c950` (unnamed, engine map unit `playercharacter.cpp`, `thiscall`, one stack word).
const PLAYER_0094C950: u32 = 0x0094_c950;

// Translated from 008c1d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall` on an actor, two stack words: an item and a word passed on to the
/// cast, `RET 8`; also in three vtables). Does nothing for a null item, when `00409700` says
/// no for it, or when `00726070` on its list `+0x74` answers 0. Otherwise calls the actor's
/// slot `+0x17c` with (item, 0, 1, 0, 0, 0, 0, 0, 1, 0), then `MagicCaster::CastSpell` on
/// `+0x88` with (`00726070`'s answer + 0x18, or 0 when it answers 0, `word`, 0); for the
/// player then `0094c950(player, 0)`.
pub fn fn_008c1d20(e: &mut Engine, this: Ptr, item: u32, word: u32) {
    let actor = this.addr();
    if item == 0 {
        return;
    }
    if !e.call(ITEM_TEST_00409700, &args![item]).bool() {
        return;
    }
    if e.call(LIST_FIRST_00726070, &args![item + 0x74]).u32() == 0 {
        return;
    }
    e.vcall(
        actor,
        ACTOR_SLOT_17C,
        &args![item, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
    let first = e.call(LIST_FIRST_00726070, &args![item + 0x74]).u32();
    let spell = if first != 0 { first + 0x18 } else { 0 };
    e.call(
        MAGIC_CASTER_CAST_SPELL,
        &args![actor + 0x88, spell, word, 0u32],
    );
    if actor == e.mem.u32(PLAYER) {
        let player = e.mem.u32(PLAYER);
        e.call(PLAYER_0094C950, &args![player, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008c_1d20, fn_008c1d20(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An actor with slot `+0x17c` at `00701000`, doubles for the callees: the item test
    /// answers `usable`, the list call `first`.
    fn engine(usable: bool, first: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let actor = e.mem.alloc(0x100);
        let mut slots = vec![0u32; 0x60];
        slots[0x17c / 4] = 0x0070_1000;
        e.put_vtable(0x0070_0000, &slots);
        e.mem.set_u32(actor, 0x0070_0000);
        e.register(0x0070_1000, |_, _| Ret::default());
        e.register_double(ITEM_TEST_00409700, move |_, _| usable.into_ret());
        e.register_double(LIST_FIRST_00726070, move |_, _| first.into_ret());
        e.register(MAGIC_CASTER_CAST_SPELL, |_, _| Ret::default());
        e.register(PLAYER_0094C950, |_, _| Ret::default());
        (e, actor)
    }

    #[test]
    fn fn_008c1d20_casts_the_item_for_an_actor() {
        let (mut e, actor) = engine(true, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x008c_1d20, &args![actor, 0x4000u32, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ITEM_TEST_00409700), vec![vec![0x4000]]);
        assert_eq!(
            calls(&log, LIST_FIRST_00726070),
            vec![vec![0x4074], vec![0x4074]]
        );
        assert_eq!(
            calls(&log, 0x0070_1000),
            vec![vec![actor, 0x4000, 0, 1, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls(&log, MAGIC_CASTER_CAST_SPELL),
            vec![vec![actor + 0x88, 0x5018, 0x77, 0]]
        );
        assert!(calls(&log, PLAYER_0094C950).is_empty());
    }

    #[test]
    fn fn_008c1d20_tells_the_player() {
        let (mut e, actor) = engine(true, 0x5000);
        e.set_global(PLAYER, actor);
        e.call_log = Some(vec![]);
        e.call(0x008c_1d20, &args![actor, 0x4000u32, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, PLAYER_0094C950), vec![vec![actor, 0]]);
    }

    #[test]
    fn fn_008c1d20_stops_early() {
        // No item, an item the test refuses, an empty list.
        for (item, usable, first) in [
            (0u32, true, 0x5000u32),
            (0x4000, false, 0x5000),
            (0x4000, true, 0),
        ] {
            let (mut e, actor) = engine(usable, first);
            e.call_log = Some(vec![]);
            e.call(0x008c_1d20, &args![actor, item, 0x77u32]);
            let log = e.call_log.take().unwrap();
            assert!(calls(&log, 0x0070_1000).is_empty());
            assert!(calls(&log, MAGIC_CASTER_CAST_SPELL).is_empty());
        }
    }
}
