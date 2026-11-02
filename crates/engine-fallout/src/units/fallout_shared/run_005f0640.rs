//! `fallout shared/run_005f0640` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_shared::creaturesounds::{CreatureSound, CreatureSounds};

/// Number of sound types (`CreatureSounds::pSoundListArray`, Xbox PDB).
const SOUND_TYPE_COUNT: i32 = 0x16;
/// `008256d0` (`fastcall`): whether a list node is empty (item and next null),
/// a `bool` in `AL`.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`fastcall`): answers the node itself (the item's address).
const LIST_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// `00726070` (`fastcall`): the node's next (`+4`).
const LIST_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (`fastcall`): removes the list's head item.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `004702f0` (`thiscall`, `RET 4`): the list's deleting destructor (flags).
const LIST_DELETE: u32 = 0x0047_02f0;
/// `0096a2d0` (`thiscall`): the list's constructor (item and next 0).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `005ae3d0` (`thiscall`, `RET 4`, the item's address): inserts the item
/// before the node's.
const LIST_INSERT_BEFORE: u32 = 0x005a_e3d0;
/// `00905820` (`thiscall`, `RET 4`, the item's address): appends the item
/// at the end of the list from the node.
const LIST_APPEND: u32 = 0x0090_5820;
/// `00401000` / `00401030`: allocate / free (`cdecl`).
const ALLOCATE_BLOCK: u32 = 0x0040_1000;
const FREE_BLOCK: u32 = 0x0040_1030;
/// `00487f50`: a random number (another unit's translation calls it that).
const RANDOM: u32 = 0x0048_7f50;

/// The address of type `kind`'s list pointer (no bounds check, as the exe
/// indexes it).
fn list_slot(this: Ptr<CreatureSounds>, kind: i32) -> u32 {
    this.addr().wrapping_add((kind as u32).wrapping_mul(4))
}

// Translated from 005f0ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the list of sound type `kind` (`thiscall`,
/// `RET 4`), or null when `kind` is outside `0..0x16` (signed).
pub fn fn_005f0ac0(e: &mut Engine, this: Ptr<CreatureSounds>, kind: i32) -> u32 {
    let mut list = 0;
    if (0..SOUND_TYPE_COUNT).contains(&kind) {
        list = e.mem.u32(list_slot(this, kind));
    }
    list
}

// Translated from 005f0860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties sound type `kind` (`thiscall`, `RET 4`).
/// While its list (`005f0ac0`, read once) is not empty, frees the head
/// item's `CREATURE_SOUND` (`00401030`) and removes the head (`0063f7b0`).
/// Then deletes the list itself when there is one (`004702f0` with 1; the
/// slot is read without a bounds check) and clears the slot.
pub fn fn_005f0860(e: &mut Engine, this: Ptr<CreatureSounds>, kind: i32) {
    let list = fn_005f0ac0(e, this, kind);
    while list != 0 && !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let item = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
        let sound = e.mem.u32(item);
        e.call(FREE_BLOCK, &args![sound]);
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
    let slot = list_slot(this, kind);
    let owned = e.mem.u32(slot);
    if owned != 0 {
        e.call(LIST_DELETE, &args![owned, 1u32]);
    }
    e.mem.set_u32(slot, 0);
}

// Translated from 005f0820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties every sound type, 0 to 0x15
/// (`005f0860`; the counter is a signed byte).
pub fn fn_005f0820(e: &mut Engine, this: Ptr<CreatureSounds>) {
    let mut kind: i8 = 0;
    while i32::from(kind) < SOUND_TYPE_COUNT {
        fn_005f0860(e, this, i32::from(kind));
        kind = kind.wrapping_add(1);
    }
}

// Translated from 005f0900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds a `CREATURE_SOUND` to type `kind`'s list,
/// kept in ascending chance order (`thiscall`, `RET 8`: the sound, the type).
/// Nothing for a null sound or a type outside `0..0x16`. A missing list is
/// made (8 bytes, `00401000`, constructed by `0096a2d0` unless the
/// allocation failed). The walk stops at the first node that is empty or
/// whose item's chance is above the new one's; the sound (from its stack
/// slot) is inserted before that node (`005ae3d0`), or, when the walk ran off
/// the end, appended after the last node (`00905820`) if there was one.
// Translation incomplete: 005f0900 - the SEH frame (FS:[0], unwind of the list allocation) is not translated.
pub fn fn_005f0900(
    e: &mut Engine,
    this: Ptr<CreatureSounds>,
    sound: Ptr<CreatureSound>,
    kind: i32,
) {
    if sound.is_null() || !(0..SOUND_TYPE_COUNT).contains(&kind) {
        return;
    }
    if e.mem.u32(list_slot(this, kind)) == 0 {
        let block = e.call(ALLOCATE_BLOCK, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(list_slot(this, kind), list);
    }
    let mut node = fn_005f0ac0(e, this, kind);
    let mut previous = 0;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        let chance = e.get(
            Ptr::<CreatureSound>::new(e.mem.u32(item)),
            CreatureSound::iChance,
        );
        if chance > e.get(sound, CreatureSound::iChance) {
            break;
        }
        previous = node;
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), sound.addr());
        if node != 0 {
            e.call(LIST_INSERT_BEFORE, &args![node, argument]);
        } else if previous != 0 {
            e.call(LIST_APPEND, &args![previous, argument]);
        }
    });
}

// Translated from 005f0a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: picks a sound of type `kind` (`thiscall`,
/// `RET 4`). Walks the list (`005f0ac0`) until a node is empty or a sound is
/// picked; for each item with a sound, a random number (`00487f50`) modulo
/// 100 below the item's chance picks that sound. Answers the `TESSound`
/// pointer or null.
pub fn fn_005f0a30(e: &mut Engine, this: Ptr<CreatureSounds>, kind: i32) -> u32 {
    let mut picked = 0;
    let mut node = fn_005f0ac0(e, this, kind);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() && picked == 0 {
        let item = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        let creature_sound = Ptr::<CreatureSound>::new(e.mem.u32(item));
        if e.get(creature_sound, CreatureSound::pSound) != 0 {
            let roll = e.call(RANDOM, &args![]).u32();
            if roll % 100 < u32::from(e.get(creature_sound, CreatureSound::iChance)) {
                picked = e.get(creature_sound, CreatureSound::pSound);
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    picked
}

// Translated from 005f0640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies another `CreatureSounds` into this
/// one (`thiscall`, `RET 4`: the other): empties every type
/// ([`fn_005f0820`]), then, with another, for each type 0 to 0x15 (a signed
/// byte counter) each of its items up to an empty node: a new 8-byte block
/// (`00401000`) filled from the item ([`fn_005f07f0`]) and added to this
/// one's list ([`fn_005f0900`]).
pub fn fn_005f0640(e: &mut Engine, this: Ptr<CreatureSounds>, other: u32) {
    fn_005f0820(e, this);
    if other == 0 {
        return;
    }
    let mut kind: i8 = 0;
    while i32::from(kind) < SOUND_TYPE_COUNT {
        let mut node = fn_005f0ac0(e, Ptr::new(other), i32::from(kind));
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let item = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
            let sound = e.mem.u32(item);
            let block = e.call(ALLOCATE_BLOCK, &args![8u32]).u32();
            fn_005f07f0(e, Ptr::new(block), Ptr::new(sound));
            fn_005f0900(e, this, Ptr::new(block), i32::from(kind));
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        kind = kind.wrapping_add(1);
    }
}

// Translated from 005f06f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether another `CreatureSounds` differs
/// (`thiscall`, `RET 4`: the other): true without one; then for each type
/// 0 to 0x15 the two lists side by side while both nodes are set and not
/// empty: true when the items' chance bytes (+4) or sounds (+0) differ;
/// true when either walk stopped on a node that is still set. False when
/// every type matches.
pub fn fn_005f06f0(e: &mut Engine, this: Ptr<CreatureSounds>, other: u32) -> bool {
    if other == 0 {
        return true;
    }
    let mut kind: i8 = 0;
    while i32::from(kind) < SOUND_TYPE_COUNT {
        let mut mine = fn_005f0ac0(e, this, i32::from(kind));
        let mut theirs = fn_005f0ac0(e, Ptr::new(other), i32::from(kind));
        while mine != 0
            && !e.call(LIST_IS_EMPTY, &args![mine]).bool()
            && theirs != 0
            && !e.call(LIST_IS_EMPTY, &args![theirs]).bool()
        {
            let slot = e.call(LIST_ITEM_ADDRESS, &args![mine]).u32();
            let my_sound = Ptr::<CreatureSound>::new(e.mem.u32(slot));
            let slot = e.call(LIST_ITEM_ADDRESS, &args![theirs]).u32();
            let their_sound = Ptr::<CreatureSound>::new(e.mem.u32(slot));
            if e.get(my_sound, CreatureSound::iChance) != e.get(their_sound, CreatureSound::iChance)
            {
                return true;
            }
            if e.get(my_sound, CreatureSound::pSound) != e.get(their_sound, CreatureSound::pSound) {
                return true;
            }
            mine = e.call(LIST_NEXT, &args![mine]).u32();
            theirs = e.call(LIST_NEXT, &args![theirs]).u32();
        }
        if mine != 0 || theirs != 0 {
            return true;
        }
        kind = kind.wrapping_add(1);
    }
    false
}

// Translated from 005f07f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies a `CREATURE_SOUND` (cdecl:
/// destination, source): the sound (+0) and the chance byte (+4), only
/// when both are set.
pub fn fn_005f07f0(e: &mut Engine, destination: Ptr<CreatureSound>, source: Ptr<CreatureSound>) {
    if destination.is_null() || source.is_null() {
        return;
    }
    let sound = e.get(source, CreatureSound::pSound);
    e.set(destination, CreatureSound::pSound, sound);
    let chance = e.get(source, CreatureSound::iChance);
    e.set(destination, CreatureSound::iChance, chance);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_0640, fn_005f0640(Ptr<CreatureSounds>, u32)),
        entry!(0x005f_06f0, fn_005f06f0(Ptr<CreatureSounds>, u32) -> bool),
        entry!(
            0x005f_07f0,
            fn_005f07f0(Ptr<CreatureSound>, Ptr<CreatureSound>)
        ),
        entry!(0x005f_0ac0, fn_005f0ac0(Ptr<CreatureSounds>, i32) -> u32),
        entry!(0x005f_0860, fn_005f0860(Ptr<CreatureSounds>, i32)),
        entry!(0x005f_0820, fn_005f0820(Ptr<CreatureSounds>)),
        entry!(
            0x005f_0900,
            fn_005f0900(Ptr<CreatureSounds>, Ptr<CreatureSound>, i32)
        ),
        entry!(0x005f_0a30, fn_005f0a30(Ptr<CreatureSounds>, i32) -> u32),
    ]
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// List doubles that act on two-word nodes (item, next) as the exe's
    /// bodies do; inserts record (callee, node, the item behind the address).
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(LIST_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(LIST_DELETE, |_, _| Ret::default());
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let log: Log = Rc::new(RefCell::new(vec![]));
        for callee in [LIST_INSERT_BEFORE, LIST_APPEND] {
            let l = log.clone();
            e.register_double(callee, move |e, a| {
                l.borrow_mut().push((callee, a[0], e.mem.u32(a[1])));
                Ret::default()
            });
        }
        (e, log)
    }

    fn sound(e: &mut Engine, form: u32, chance: u8) -> u32 {
        let s = e.mem.alloc(8);
        e.mem.set_u32(s, form);
        e.mem.set_u8(s + 4, chance);
        s
    }

    /// A list of nodes holding `items`; answers the head.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        let mut head = 0;
        for item in items.iter().rev() {
            head = e.mem.alloc(8);
            e.mem.set_u32(head, *item);
            e.mem.set_u32(head + 4, next);
            next = head;
        }
        head
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_005f0ac0_answers_the_list_inside_the_range_only() {
        let (mut e, _) = engine();
        let this = e.new_object::<CreatureSounds>();
        e.mem.set_u32(this.addr() + 4 * 0x15, 0x1234);
        e.mem.set_u32(this.addr(), 0x99);
        assert_eq!(e.call(0x005f_0ac0, &args![this, 0x15u32]).u32(), 0x1234);
        assert_eq!(e.call(0x005f_0ac0, &args![this, 0u32]).u32(), 0x99);
        assert_eq!(e.call(0x005f_0ac0, &args![this, 0x16u32]).u32(), 0);
        assert_eq!(e.call(0x005f_0ac0, &args![this, -1i32]).u32(), 0);
    }

    #[test]
    fn fn_005f0860_frees_every_sound_and_the_list() {
        let (mut e, _) = engine();
        let this = e.new_object::<CreatureSounds>();
        let l = list(&mut e, &[0x500, 0x600]);
        e.mem.set_u32(this.addr() + 8, l);
        e.call_log = Some(vec![]);
        e.call(0x005f_0860, &args![this, 2u32]);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![0x500], vec![0x600]]);
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![l, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
        // An empty slot: nothing to delete, the slot is still cleared.
        e.call_log = Some(vec![]);
        e.call(0x005f_0860, &args![this, 2u32]);
        assert!(calls_to(&e, LIST_DELETE).is_empty());
    }

    #[test]
    fn fn_005f0820_empties_all_22_types() {
        let (mut e, _) = engine();
        let this = e.new_object::<CreatureSounds>();
        for kind in 0..0x16u32 {
            let l = list(&mut e, &[]);
            e.mem.set_u32(this.addr() + 4 * kind, l.max(0x10 + kind));
        }
        e.call_log = Some(vec![]);
        e.mem.map(0x10, 0x100);
        e.call(0x005f_0820, &args![this]);
        assert_eq!(calls_to(&e, LIST_DELETE).len(), 0x16);
        for kind in 0..0x16u32 {
            assert_eq!(e.mem.u32(this.addr() + 4 * kind), 0);
        }
    }

    #[test]
    fn fn_005f0900_keeps_ascending_chance_order() {
        let (mut e, log) = engine();
        let this = e.new_object::<CreatureSounds>();
        // Null sound or bad type: nothing.
        let s50 = sound(&mut e, 0x50, 50);
        e.call_log = Some(vec![]);
        e.call(0x005f_0900, &args![this, 0u32, 1u32]);
        e.call(0x005f_0900, &args![this, s50, 0x16u32]);
        assert!(calls_to(&e, ALLOCATE_BLOCK).is_empty());
        // A new list is made; its empty head takes the insert.
        e.call(0x005f_0900, &args![this, s50, 1u32]);
        let l = e.mem.u32(this.addr() + 4);
        assert_ne!(l, 0);
        assert_eq!(log.borrow().as_slice(), [(LIST_INSERT_BEFORE, l, s50)]);
        // Before the first item with a higher chance.
        let s10 = sound(&mut e, 0x10, 10);
        let s90 = sound(&mut e, 0x90, 90);
        let l = list(&mut e, &[s10, s90]);
        e.mem.set_u32(this.addr() + 4, l);
        log.borrow_mut().clear();
        e.call(0x005f_0900, &args![this, s50, 1u32]);
        let second = e.mem.u32(l + 4);
        assert_eq!(log.borrow().as_slice(), [(LIST_INSERT_BEFORE, second, s50)]);
        // Higher than all: appended after the last node.
        let s99 = sound(&mut e, 0x99, 99);
        log.borrow_mut().clear();
        e.call(0x005f_0900, &args![this, s99, 1u32]);
        assert_eq!(log.borrow().as_slice(), [(LIST_APPEND, second, s99)]);
    }

    #[test]
    fn fn_005f0a30_picks_by_chance() {
        let (mut e, _) = engine();
        e.register(RANDOM, |_, _| 1234u32.into_ret());
        let this = e.new_object::<CreatureSounds>();
        // 1234 % 100 = 34: chance 34 misses, 35 hits; no sound is skipped.
        let s34 = sound(&mut e, 0x34, 34);
        let none = sound(&mut e, 0, 100);
        let s35 = sound(&mut e, 0x35, 35);
        let s99 = sound(&mut e, 0x99, 99);
        let l = list(&mut e, &[s34, none, s35, s99]);
        e.mem.set_u32(this.addr() + 12, l);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_0a30, &args![this, 3u32]).u32(), 0x35);
        assert_eq!(calls_to(&e, RANDOM).len(), 2);
        assert_eq!(e.call(0x005f_0a30, &args![this, 4u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Lists as nodes `{item, next}`, empty when the item is 0.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_ITEM_ADDRESS, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
    }

    /// A list of `CREATURE_SOUND`s `(sound, chance)`.
    fn sound_list(e: &mut Engine, sounds: &[(u32, u8)]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (i, &(sound, chance)) in sounds.iter().enumerate() {
            let item = e.mem.alloc(8);
            e.mem.set_u32(item, sound);
            e.mem.set_u8(item + 4, chance);
            e.mem.set_u32(node, item);
            if i + 1 < sounds.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    #[test]
    fn fn_005f07f0_copies_only_between_two_sounds() {
        let mut e = Engine::new();
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x77);
        e.mem.set_u32(source + 4, 0xffff_ff33);
        let destination = e.mem.alloc(8);
        e.call(0x005f_07f0, &args![destination, source]);
        assert_eq!(e.mem.u32(destination), 0x77);
        assert_eq!(e.mem.u32(destination + 4), 0x33);
        e.call(0x005f_07f0, &args![0u32, source]);
        let other = e.mem.alloc(8);
        e.call(0x005f_07f0, &args![other, 0u32]);
        assert_eq!(e.mem.u32(other), 0);
    }

    #[test]
    fn fn_005f0640_copies_each_type() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(ALLOCATE_BLOCK, |e, a| ret(e.mem.alloc(a[0])));
        e.register(LIST_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LIST_INSERT_BEFORE, |e, a| {
            // Puts the sound into the empty node.
            let sound = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], sound);
            Ret::default()
        });
        let this = e.mem.alloc(0x58);
        let other = e.mem.alloc(0x58);
        let list = sound_list(&mut e, &[(0x51, 25)]);
        e.mem.set_u32(other + 8, list);
        e.call_log = Some(vec![]);
        e.call(0x005f_0640, &args![this, other]);
        let copy = e.mem.u32(e.mem.u32(this + 8));
        assert_ne!(copy, e.mem.u32(list));
        assert_eq!((e.mem.u32(copy), e.mem.u8(copy + 4)), (0x51, 25));
        assert_eq!(calls(&e, LIST_INSERT_BEFORE).len(), 1);
        // Without another: only emptied.
        e.call_log = Some(vec![]);
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(LIST_DELETE, |_, _| Ret::default());
        e.call(0x005f_0640, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(calls(&e, ALLOCATE_BLOCK).len(), 0);
    }

    #[test]
    fn fn_005f06f0_compares_the_lists_side_by_side() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = e.mem.alloc(0x58);
        let other = e.mem.alloc(0x58);
        let mine = sound_list(&mut e, &[(1, 10), (2, 20)]);
        let theirs = sound_list(&mut e, &[(1, 10), (2, 20)]);
        e.mem.set_u32(this + 0x54, mine);
        e.mem.set_u32(other + 0x54, theirs);
        assert!(!e.call(0x005f_06f0, &args![this, other]).bool());
        assert!(e.call(0x005f_06f0, &args![this, 0u32]).bool());
        for sounds in [
            vec![(1u32, 10u8), (2, 21)],
            vec![(1, 10), (3, 20)],
            vec![(1, 10)],
            vec![(1, 10), (2, 20), (3, 30)],
        ] {
            let theirs = sound_list(&mut e, &sounds);
            e.mem.set_u32(other + 0x54, theirs);
            assert!(
                e.call(0x005f_06f0, &args![this, other]).bool(),
                "{sounds:?}"
            );
        }
        // An empty list against none: the empty node is still set.
        let empty = e.mem.alloc(8);
        e.mem.set_u32(other + 0x54, empty);
        e.mem.set_u32(this + 0x54, 0);
        assert!(e.call(0x005f_06f0, &args![this, other]).bool());
    }
}
