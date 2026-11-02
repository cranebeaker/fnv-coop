//! `fallout/misc/saveload/bgssaveloadformidmap.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSSaveLoadFormIDMap` maps form IDs to the indices a save file uses and
//! back: the forward map is at `+0`, the reverse map at `+0x10`, the next
//! index at `+0x20` (layout in `run_00846b60.rs`, which also holds the
//! constructor and the two lookups `00846c90`/`00846d20`). The functions
//! here clear the maps, translate a form ID through the load order table,
//! and save and load the reverse map.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_misc::run_00846b60::BGSSaveLoadFormIDMap;

/// `00438af0`: empties a hash map (thiscall on the map).
const CLEAR_MAP: u32 = 0x0043_8af0;
/// `00846c90`: gets or adds the index of a form ID (thiscall on the map; the
/// index 0 is reserved for the null form ID).
const GET_OR_ADD_INDEX: u32 = 0x0084_6c90;
/// `00846d20`: the form ID for an index in the reverse map (thiscall).
const RESOLVE_INDEX: u32 = 0x0084_6d20;
/// `00846d80`'s helper: the byte at `+0x44 + n` of the object at `011ddf38`
/// (thiscall with the byte `n` as the single stack word).
const LOAD_ORDER_LOOKUP: u32 = 0x0084_6de0;
/// The global pointer to the object `00846de0` is called on.
const LOAD_ORDER_TABLE: u32 = 0x011d_df38;
/// `00844700`: stores a value under a key in a hash map (thiscall on the
/// map: key, value).
const MAP_SET: u32 = 0x0084_4700;
/// `00845d00`: writes bytes to the save file (cdecl: file, source, count).
const SAVE_DATA_TO_FILE: u32 = 0x0084_5d00;
/// `00845d20`: reads bytes from the save file (cdecl: file, destination,
/// count).
const LOAD_DATA_FROM_FILE: u32 = 0x0084_5d20;

// Translated from 00846c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the forward map (`+0`) and the reverse map (`+0x10`), sets the
/// next index (`+0x20`) to 0 and reserves index 0 for the null form ID
/// (`00846c90(0)`).
pub fn fn_00846c50(e: &mut Engine, this: Ptr<BGSSaveLoadFormIDMap>) {
    e.call(CLEAR_MAP, &args![this]);
    e.call(CLEAR_MAP, &args![this.addr().wrapping_add(0x10)]);
    e.set(this, BGSSaveLoadFormIDMap::iCurrentIndex, 0);
    e.call(GET_OR_ADD_INDEX, &args![this, 0u32]);
}

// Translated from 00846d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Translates a form ID saved with another load order into the current one:
/// an ID whose top byte is 0xff (a created form) stays; otherwise the top
/// byte is looked up (`00846de0` on the object at `011ddf38`) and replaced
/// by the answer, or the result is 0 when the answer is 0xff.
pub fn fn_00846d80(e: &mut Engine, _this: Ptr<BGSSaveLoadFormIDMap>, form_id: u32) -> u32 {
    let top_byte = (form_id >> 24) as u8;
    if top_byte == 0xff {
        return form_id;
    }
    let table = e.global::<u32>(LOAD_ORDER_TABLE);
    let mapped = e.call(LOAD_ORDER_LOOKUP, &args![table, top_byte]).u8();
    if mapped == 0xff {
        0
    } else {
        (form_id & 0x00ff_ffff) | (u32::from(mapped) << 24)
    }
}

// Translated from 00846e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFormIDMap::Save` (Xbox PDB): writes the number of indices
/// minus one, then the form ID of every index from 1 on (`00846d20`), each
/// as four bytes through `00845d00`.
pub fn bgssaveloadformidmap_save(e: &mut Engine, this: Ptr<BGSSaveLoadFormIDMap>, file: Ptr) {
    let count = e.get(this, BGSSaveLoadFormIDMap::iCurrentIndex);
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), count.wrapping_sub(1));
        e.call(SAVE_DATA_TO_FILE, &args![file, word, 4u32]);
        let mut index = 1u32;
        while index < e.get(this, BGSSaveLoadFormIDMap::iCurrentIndex) {
            let form_id = e.call(RESOLVE_INDEX, &args![this, index]).u32();
            e.mem.set_u32(word.addr(), form_id);
            e.call(SAVE_DATA_TO_FILE, &args![file, word, 4u32]);
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00846e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFormIDMap::Load` (Xbox PDB): clears the maps (`00846c50`),
/// reads the number of saved entries, then for each reads a form ID,
/// translates it (`00846d80`), enters it in the forward map under the next
/// index when the translation is not 0 and always enters the next index in
/// the reverse map (as the translation, so 0 for a lost form), advancing the
/// index.
pub fn bgssaveloadformidmap_load(e: &mut Engine, this: Ptr<BGSSaveLoadFormIDMap>, file: Ptr) {
    fn_00846c50(e, this);
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(LOAD_DATA_FROM_FILE, &args![file, word, 4u32]);
        let count = e.mem.u32(word.addr());
        let mut entry = 0u32;
        while entry < count {
            e.mem.set_u32(word.addr(), 0);
            e.call(LOAD_DATA_FROM_FILE, &args![file, word, 4u32]);
            let saved = e.mem.u32(word.addr());
            let form_id = fn_00846d80(e, this, saved);
            let index = e.get(this, BGSSaveLoadFormIDMap::iCurrentIndex);
            if form_id != 0 {
                e.call(MAP_SET, &args![this, form_id, index]);
            }
            e.call(
                MAP_SET,
                &args![this.addr().wrapping_add(0x10), index, form_id],
            );
            e.set(
                this,
                BGSSaveLoadFormIDMap::iCurrentIndex,
                index.wrapping_add(1),
            );
            entry = entry.wrapping_add(1);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00846c50, fn_00846c50(Ptr<BGSSaveLoadFormIDMap>)),
        entry!(
            0x00846d80,
            fn_00846d80(Ptr<BGSSaveLoadFormIDMap>, u32) -> u32
        ),
        entry!(
            0x00846e00,
            bgssaveloadformidmap_save(Ptr<BGSSaveLoadFormIDMap>, Ptr)
        ),
        entry!(
            0x00846e70,
            bgssaveloadformidmap_load(Ptr<BGSSaveLoadFormIDMap>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn word_ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn reset_clears_both_maps_and_reserves_index_zero() {
        let mut e = Engine::new();
        e.register(CLEAR_MAP, |_, _| Ret::default());
        e.register(GET_OR_ADD_INDEX, |_, _| word_ret(0));
        let map = e.new_object::<BGSSaveLoadFormIDMap>();
        e.set(map, BGSSaveLoadFormIDMap::iCurrentIndex, 9);
        e.call_log = Some(vec![]);
        e.call(0x0084_6c50, &[map.addr()]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (CLEAR_MAP, vec![map.addr()]),
                (CLEAR_MAP, vec![map.addr() + 0x10]),
                (GET_OR_ADD_INDEX, vec![map.addr(), 0]),
            ]
        );
        assert_eq!(e.get(map, BGSSaveLoadFormIDMap::iCurrentIndex), 0);
    }

    fn translation_engine() -> Engine {
        let mut e = Engine::new();
        e.map(LOAD_ORDER_TABLE, 4);
        e.set_global(LOAD_ORDER_TABLE, 0x0c00_0000u32);
        e.register(LOAD_ORDER_LOOKUP, |_, a| {
            word_ret(match a[1] {
                0x00 => 0x00,
                0x01 => 0x05,
                0x02 => 0xff,
                other => other + 0x10,
            })
        });
        e
    }

    #[test]
    fn form_ids_are_translated_through_the_load_order() {
        let mut e = translation_engine();
        let map = e.new_object::<BGSSaveLoadFormIDMap>();
        let call = |e: &mut Engine, id: u32| e.call(0x0084_6d80, &[map.addr(), id]).u32();
        // Created forms stay.
        assert_eq!(call(&mut e, 0xff12_3456), 0xff12_3456);
        // The top byte is replaced, the rest kept.
        assert_eq!(call(&mut e, 0x0100_1234), 0x0500_1234);
        assert_eq!(call(&mut e, 0x0000_0007), 0x0000_0007);
        assert_eq!(call(&mut e, 0x0300_00aa), 0x1300_00aa);
        // An unknown plugin gives 0.
        assert_eq!(call(&mut e, 0x0200_1234), 0);
    }

    #[test]
    fn save_writes_the_count_then_each_resolved_form_id() {
        let mut e = Engine::new();
        let map = e.new_object::<BGSSaveLoadFormIDMap>();
        e.set(map, BGSSaveLoadFormIDMap::iCurrentIndex, 4);
        e.register(RESOLVE_INDEX, |_, a| word_ret(0x0100_0000 + a[1]));
        let written = Rc::new(RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(SAVE_DATA_TO_FILE, move |e, a| {
            assert_eq!(a[2], 4);
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x0084_6e00, &[map.addr(), 0xf11e]);
        assert_eq!(
            *written.borrow(),
            vec![
                (0xf11e, 3),
                (0xf11e, 0x0100_0001),
                (0xf11e, 0x0100_0002),
                (0xf11e, 0x0100_0003)
            ]
        );
    }

    #[test]
    fn save_of_an_empty_map_writes_only_the_count() {
        let mut e = Engine::new();
        let map = e.new_object::<BGSSaveLoadFormIDMap>();
        e.set(map, BGSSaveLoadFormIDMap::iCurrentIndex, 1);
        let written = Rc::new(RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(SAVE_DATA_TO_FILE, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call(0x0084_6e00, &[map.addr(), 1]);
        assert_eq!(*written.borrow(), vec![0]);
    }

    #[test]
    fn load_enters_each_translated_form_id_and_every_index() {
        let mut e = translation_engine();
        let map = e.new_object::<BGSSaveLoadFormIDMap>();
        e.register(CLEAR_MAP, |_, _| Ret::default());
        e.register(GET_OR_ADD_INDEX, |e, a| {
            // The real one reserves index 0; the double only advances it.
            e.mem.set_u32(a[0] + 0x20, 1);
            Ret::default()
        });
        // The file holds the count 3, then the form IDs below.
        let words = Rc::new(RefCell::new(
            vec![3u32, 0x0100_0010, 0x0200_0020, 0xff00_0030].into_iter(),
        ));
        e.register_double(LOAD_DATA_FROM_FILE, move |e, a| {
            assert_eq!(a[2], 4);
            let word = words.borrow_mut().next().unwrap();
            e.mem.set_u32(a[1], word);
            Ret::default()
        });
        let sets = Rc::new(RefCell::new(Vec::new()));
        let sink = sets.clone();
        e.register_double(MAP_SET, move |_, a| {
            sink.borrow_mut().push((a[0], a[1], a[2]));
            Ret::default()
        });
        e.call(0x0084_6e70, &[map.addr(), 0xf11e]);
        let reverse = map.addr() + 0x10;
        assert_eq!(
            *sets.borrow(),
            vec![
                (map.addr(), 0x0500_0010, 1),
                (reverse, 1, 0x0500_0010),
                // The unknown plugin: no forward entry, a reverse entry of 0.
                (reverse, 2, 0),
                (map.addr(), 0xff00_0030, 3),
                (reverse, 3, 0xff00_0030),
            ]
        );
        assert_eq!(e.get(map, BGSSaveLoadFormIDMap::iCurrentIndex), 4);
    }
}
