//! `fallout shared/run_0047ea30` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Seven helpers of `TESActorBaseData` (Xbox PDB; the component of a
//! `TESActorBase` at +0x30) and of the factions it lists, which sit after
//! the `tesactorbasedata.cpp` code and have no names in the Xbox PDB map.
//! What identifies them: `0047ea30` / `0047ea70` are called by
//! `TESActorBase`'s save-game (`005f1f30`) and load-game (`005f1fd0`)
//! functions with `ECX` = the form + 0x30 and move the 0x18 bytes at +4 (the
//! PDB's `actorData`, `ACTOR_BASE_DATA`); `0047ead0` and `0047ebf0` walk the
//! list at +0x2c (the PDB's `listFactions`, a `BSSimpleList<FACTION_RANK *>`
//! whose `FACTION_RANK` starts with `pFaction`); the three tiny wrappers set
//! or clear a bit of the faction's flag word at +0x34 (the PDB's
//! `FACTION_DATA` `iFlags`). The Xbox names of the class members are not
//! known, so the functions keep `fn_<address>` names.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_shared::tesactorbasedata::TESActorBaseData;

/// `ExtraDataList`-unit helper (`ECX` = save/load buffer, one argument: the
/// out pointer): stores the buffer's current save kind word (at +0x17) in
/// the out pointer and returns it.
const BUFFER_SAVE_KIND: u32 = 0x0042_8110;
/// Tests the word at `ECX` against a mask (`ECX` = pointer to the word, one
/// argument: the mask); true when any bit of the mask is set.
const KIND_HAS_BITS: u32 = 0x0042_80f0;
/// `BGSSaveGameBuffer` raw write: `this` = buffer, data, size, 0.
const SAVE_RAW: u32 = 0x0086_5e50;
/// `BGSLoadGameBuffer` raw read: `this` = buffer, destination, size.
const LOAD_RAW: u32 = 0x0086_4980;
/// Tests the flag word at +4 of the object in `ECX` against a mask (one
/// argument); true when any bit is set.
const FLAGS_HAVE_BITS: u32 = 0x0046_1580;
/// `TESActorBaseData::SetFlagBit` (Xbox PDB): `this`, mask, value, mark
/// changed.
const SET_FLAG_BIT: u32 = 0x0047_dd50;
/// `TESActorBaseData::GetFactionRank` (Xbox PDB): `this`, faction, check the
/// faction flag; -1 when the actor base is not in the faction.
const GET_FACTION_RANK: u32 = 0x0047_d680;
/// Returns `ECX + 0x2c`: the `listFactions` head node of the actor base data.
const FACTION_LIST_HEAD: u32 = 0x005d_8a70;
/// Returns `ECX`: the item slot of a `BSSimpleList` node.
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// Returns the next node of a `BSSimpleList` node (`ECX` = node).
const NODE_NEXT: u32 = 0x0072_6070;
/// Returns the base form of the reference in `ECX` (`MOV EAX,[ECX+0x20]`).
const REFERENCE_BASE_FORM: u32 = 0x0041_81e0;
/// The faction's flag-bit setter: `this`, mask, value (a byte). It sets or
/// clears the bits of the flag word at +0x34 and then calls virtual slot
/// `0x48` with 2.
const FACTION_SET_FLAG_BITS: u32 = 0x005f_c970;

/// The global holding the player reference (`011dea3c`).
const PLAYER_REFERENCE: u32 = 0x011d_ea3c;
/// The save kind bit that makes the actor data part of the save (the mask
/// the code tests).
const SAVE_KIND_ACTOR_DATA: u32 = 2;
/// Size of the actor data block at +4 (`ACTOR_BASE_DATA`).
const ACTOR_DATA_SIZE: u32 = 0x18;
/// The bit of `iActorBaseFlags` that `0047ea70` re-applies after loading
/// (`SetFlagBit` mask and the mask tested before the load).
const ACTOR_FLAG_BIT: u32 = 1;
/// Offset from the actor base data to the `TESActorBaseData` component in
/// the player's base form (`TESActorBase` +0x30).
const COMPONENT_OFFSET: u32 = 0x30;
/// The three bits the wrappers set or clear in the faction's flag word.
const FACTION_BIT_FIRST: u32 = 0x10;
const FACTION_BIT_SECOND: u32 = 0x20;
const FACTION_BIT_THIRD: u32 = 0x40;

// Translated from 0047ea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the actor data block (`this + 4`, 0x18 bytes) into the save game
/// buffer when the buffer's save kind has bit 2.
pub fn fn_0047ea30(e: &mut Engine, this: Ptr<TESActorBaseData>, buffer: Ptr) {
    let wanted = e.with_stack(4, |e, kind| {
        let kind = e.call(BUFFER_SAVE_KIND, &args![buffer, kind]).ptr::<()>();
        e.call(KIND_HAS_BITS, &args![kind, SAVE_KIND_ACTOR_DATA])
            .bool()
    });
    if wanted {
        e.call(
            SAVE_RAW,
            &args![buffer, this.addr() + 4, ACTOR_DATA_SIZE, 0],
        );
    }
}

// Translated from 0047ea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the actor data block (`this + 4`, 0x18 bytes) from the load game
/// buffer when the buffer's save kind has bit 2. The state of flag bit 1 of
/// the old flags is read first; after the load it is written back with
/// `TESActorBaseData::SetFlagBit` (mark changed set), so the change
/// bookkeeping sees the old state's difference.
pub fn fn_0047ea70(e: &mut Engine, this: Ptr<TESActorBaseData>, buffer: Ptr) {
    let wanted = e.with_stack(4, |e, kind| {
        let kind = e.call(BUFFER_SAVE_KIND, &args![buffer, kind]).ptr::<()>();
        e.call(KIND_HAS_BITS, &args![kind, SAVE_KIND_ACTOR_DATA])
            .bool()
    });
    if wanted {
        let old_flag = e.call(FLAGS_HAVE_BITS, &args![this, ACTOR_FLAG_BIT]).u8();
        e.call(LOAD_RAW, &args![buffer, this.addr() + 4, ACTOR_DATA_SIZE]);
        e.call(
            SET_FLAG_BIT,
            &args![this, ACTOR_FLAG_BIT, old_flag as u32, 1],
        );
    }
}

// Translated from 0047ead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the faction list (`this + 0x2c`) and, for each faction the player's
/// base form is in (`GetFactionRank` with the faction-flag check is not
/// -1), sets bit `0x40` of the faction's flag word when `mode` is 0, bit
/// `0x20` when it is 1 and bit `0x10` when it is 2 (any other mode does
/// nothing). The walk ends at the first node whose item is null. The player
/// is read from the global at each step.
pub fn fn_0047ead0(e: &mut Engine, this: Ptr<TESActorBaseData>, mode: u32) {
    let mut node = e.call(FACTION_LIST_HEAD, &args![this]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let rank_entry = e.mem.u32(slot);
        // `pFaction` is the first word of the `FACTION_RANK`.
        let faction = e.mem.u32(rank_entry);
        if rank_entry != 0 {
            let player = e.global::<u32>(PLAYER_REFERENCE);
            let base_form = e.call(REFERENCE_BASE_FORM, &args![player]).u32();
            let rank = e
                .call(
                    GET_FACTION_RANK,
                    &args![base_form + COMPONENT_OFFSET, faction, 1],
                )
                .i32();
            if rank != -1 {
                match mode {
                    0 => fn_0047ebb0(e, Ptr::new(faction), 1),
                    1 => fn_0047ebd0(e, Ptr::new(faction), 1),
                    2 => fn_0047eb90(e, Ptr::new(faction), 1),
                    _ => {}
                }
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0047eb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` not zero) or clears bit `0x10` of the faction's flag word.
pub fn fn_0047eb90(e: &mut Engine, this: Ptr, value: u8) {
    e.call(
        FACTION_SET_FLAG_BITS,
        &args![this, FACTION_BIT_FIRST, value],
    );
}

// Translated from 0047ebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` not zero) or clears bit `0x40` of the faction's flag word.
pub fn fn_0047ebb0(e: &mut Engine, this: Ptr, value: u8) {
    e.call(
        FACTION_SET_FLAG_BITS,
        &args![this, FACTION_BIT_THIRD, value],
    );
}

// Translated from 0047ebd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` not zero) or clears bit `0x20` of the faction's flag word.
pub fn fn_0047ebd0(e: &mut Engine, this: Ptr, value: u8) {
    e.call(
        FACTION_SET_FLAG_BITS,
        &args![this, FACTION_BIT_SECOND, value],
    );
}

// Translated from 0047ebf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `faction` is in the faction list (`this + 0x2c`): some node's
/// item is non-null and its first word (`pFaction`) equals `faction`.
pub fn fn_0047ebf0(e: &mut Engine, this: Ptr<TESActorBaseData>, faction: u32) -> bool {
    let mut node = e.call(FACTION_LIST_HEAD, &args![this]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let rank_entry = e.mem.u32(slot);
        if rank_entry != 0 && e.mem.u32(rank_entry) == faction {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047ea30, fn_0047ea30(Ptr<TESActorBaseData>, Ptr)),
        entry!(0x0047ea70, fn_0047ea70(Ptr<TESActorBaseData>, Ptr)),
        entry!(0x0047ead0, fn_0047ead0(Ptr<TESActorBaseData>, u32)),
        entry!(0x0047eb90, fn_0047eb90(Ptr, u8)),
        entry!(0x0047ebb0, fn_0047ebb0(Ptr, u8)),
        entry!(0x0047ebd0, fn_0047ebd0(Ptr, u8)),
        entry!(0x0047ebf0, fn_0047ebf0(Ptr<TESActorBaseData>, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine whose save-kind helpers read the word at +0x17 of the buffer
    /// and whose list helpers are the plain ones (head at +0x2c, the node is
    /// its own item slot, the next node at +4).
    fn engine_with_helpers() -> Engine {
        let mut e = Engine::new();
        e.register(BUFFER_SAVE_KIND, |e, a| {
            let word = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], word);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(KIND_HAS_BITS, |e, a| Ret {
            eax: (e.mem.u32(a[0]) & a[1] != 0) as u32,
            ..Ret::default()
        });
        e.register(FACTION_LIST_HEAD, |_, a| Ret {
            eax: a[0] + 0x2c,
            ..Ret::default()
        });
        e.register(NODE_ITEM_SLOT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e
    }

    fn buffer_with_kind(e: &mut Engine, kind: u32) -> Ptr {
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(buffer + 0x17, kind);
        Ptr::new(buffer)
    }

    fn actor_base_data(e: &mut Engine) -> Ptr<TESActorBaseData> {
        e.new_object()
    }

    #[test]
    fn save_writes_the_actor_data_only_for_save_kind_bit_two() {
        let mut e = engine_with_helpers();
        let data = actor_base_data(&mut e);
        e.register(SAVE_RAW, |_, _| Ret::default());
        let skipped = buffer_with_kind(&mut e, 0x1d);
        e.call_log = Some(vec![]);
        e.call(0x0047_ea30, &args![data, skipped]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != SAVE_RAW));

        let wanted = buffer_with_kind(&mut e, 0x02);
        e.call_log = Some(vec![]);
        e.call(0x0047_ea30, &args![data, wanted]);
        let log = e.call_log.take().unwrap();
        let raw = log.iter().find(|(addr, _)| *addr == SAVE_RAW).unwrap();
        assert_eq!(raw.1[..], [wanted.addr(), data.addr() + 4, 0x18, 0]);
    }

    #[test]
    fn load_reads_the_actor_data_and_reapplies_the_old_flag_bit() {
        let mut e = engine_with_helpers();
        let data = actor_base_data(&mut e);
        e.register(FLAGS_HAVE_BITS, |e, a| Ret {
            eax: (e.mem.u32(a[0] + 4) & a[1] != 0) as u32,
            ..Ret::default()
        });
        e.register(LOAD_RAW, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 0);
            }
            Ret::default()
        });
        e.register(SET_FLAG_BIT, |_, _| Ret::default());
        // Not the wanted kind: nothing happens.
        let skipped = buffer_with_kind(&mut e, 0x05);
        e.call_log = Some(vec![]);
        e.call(0x0047_ea70, &args![data, skipped]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .all(|(addr, _)| *addr != LOAD_RAW && *addr != SET_FLAG_BIT));

        // Flag bit 1 was set before the load (the load clears the block).
        e.mem.set_u32(data.addr() + 4, 0x81);
        let wanted = buffer_with_kind(&mut e, 0x02);
        e.call_log = Some(vec![]);
        e.call(0x0047_ea70, &args![data, wanted]);
        let log = e.call_log.take().unwrap();
        let load = log.iter().position(|(addr, _)| *addr == LOAD_RAW).unwrap();
        let set = log
            .iter()
            .position(|(addr, _)| *addr == SET_FLAG_BIT)
            .unwrap();
        assert!(load < set);
        assert_eq!(log[load].1[..], [wanted.addr(), data.addr() + 4, 0x18]);
        assert_eq!(log[set].1[..], [data.addr(), 1, 1, 1]);
        assert_eq!(e.mem.u32(data.addr() + 4), 0);

        // Flag bit 1 clear before the load: the value passed is 0.
        e.mem.set_u32(data.addr() + 4, 0x80);
        e.call_log = Some(vec![]);
        e.call(0x0047_ea70, &args![data, wanted]);
        let log = e.call_log.take().unwrap();
        let set = log.iter().find(|(addr, _)| *addr == SET_FLAG_BIT).unwrap();
        assert_eq!(set.1[..], [data.addr(), 1, 0, 1]);
    }

    /// A faction list of the given factions (each a `FACTION_RANK` whose first
    /// word is the faction), hanging off `data`'s list head; a null entry in
    /// `entries` makes a node with a null item.
    fn put_factions(e: &mut Engine, data: Ptr<TESActorBaseData>, entries: &[u32]) {
        let mut slot = data.addr() + 0x2c;
        for (i, faction) in entries.iter().enumerate() {
            let rank_entry = if *faction == 0 {
                0
            } else {
                let block = e.mem.alloc(8);
                e.mem.set_u32(block, *faction);
                block
            };
            e.mem.set_u32(slot, rank_entry);
            if i + 1 < entries.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(slot + 4, next);
                slot = next;
            }
        }
    }

    #[test]
    fn faction_walk_sets_the_bit_for_the_mode_on_player_factions_only() {
        for (mode, mask) in [(0u32, 0x40u32), (1, 0x20), (2, 0x10)] {
            let mut e = engine_with_helpers();
            e.map(0x011d_e000, 0x1000);
            e.set_global::<u32>(PLAYER_REFERENCE, 0x7000);
            let data = actor_base_data(&mut e);
            put_factions(&mut e, data, &[0xfac1, 0xfac2, 0xfac3]);
            let base = e.mem.alloc(0x40);
            e.register_double(REFERENCE_BASE_FORM, move |_, a| {
                assert_eq!(a[0], 0x7000);
                Ret {
                    eax: base,
                    ..Ret::default()
                }
            });
            // The player is in factions 1 and 3 only.
            e.register_double(GET_FACTION_RANK, move |_, a| {
                assert_eq!((a[0], a[2]), (base + 0x30, 1));
                Ret {
                    eax: if a[1] == 0xfac2 { u32::MAX } else { 2 },
                    ..Ret::default()
                }
            });
            e.register(FACTION_SET_FLAG_BITS, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0047_ead0, &args![data, mode]);
            let log = e.call_log.take().unwrap();
            let sets: Vec<_> = log
                .iter()
                .filter(|(addr, _)| *addr == FACTION_SET_FLAG_BITS)
                .map(|(_, a)| a.clone())
                .collect();
            assert_eq!(sets, [vec![0xfac1, mask, 1], vec![0xfac3, mask, 1]]);
        }
    }

    #[test]
    fn faction_walk_ignores_other_modes_and_stops_at_a_null_item() {
        let mut e = engine_with_helpers();
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(PLAYER_REFERENCE, 0x7000);
        let data = actor_base_data(&mut e);
        put_factions(&mut e, data, &[0xfac1, 0, 0xfac3]);
        e.register(REFERENCE_BASE_FORM, |_, _| Ret::default());
        e.register(GET_FACTION_RANK, |_, _| Ret::default());
        e.register(FACTION_SET_FLAG_BITS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_ead0, &args![data, 3]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != FACTION_SET_FLAG_BITS));
        // The walk stopped at the null item: the third faction never reached
        // `GetFactionRank`.
        let asked = log
            .iter()
            .filter(|(addr, _)| *addr == GET_FACTION_RANK)
            .count();
        assert_eq!(asked, 1);
    }

    #[test]
    fn the_three_wrappers_pass_their_mask_and_value() {
        let mut e = Engine::new();
        let faction = Ptr::<()>::new(0x5000);
        e.register(FACTION_SET_FLAG_BITS, |_, _| Ret::default());
        for (addr, mask) in [
            (0x0047_eb90u32, 0x10u32),
            (0x0047_ebb0, 0x40),
            (0x0047_ebd0, 0x20),
        ] {
            for value in [0u32, 1] {
                e.call_log = Some(vec![]);
                e.call(addr, &args![faction, value]);
                let log = e.call_log.take().unwrap();
                let call = log
                    .iter()
                    .find(|(callee, _)| *callee == FACTION_SET_FLAG_BITS)
                    .unwrap();
                assert_eq!(call.1[..], [0x5000, mask, value]);
            }
        }
    }

    #[test]
    fn faction_membership_search_matches_the_first_word_of_a_non_null_item() {
        let mut e = engine_with_helpers();
        let data = actor_base_data(&mut e);
        put_factions(&mut e, data, &[0xfac1, 0, 0xfac3]);
        assert!(e.call(0x0047_ebf0, &args![data, 0xfac1]).bool());
        // A null item is skipped, the search goes on.
        assert!(e.call(0x0047_ebf0, &args![data, 0xfac3]).bool());
        assert!(!e.call(0x0047_ebf0, &args![data, 0xfac2]).bool());
        // An empty list head has a null item and no next node.
        let empty = actor_base_data(&mut e);
        assert!(!e.call(0x0047_ebf0, &args![empty, 0xfac1]).bool());
    }
}
