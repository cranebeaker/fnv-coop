//! `fallout/ai/run_008d10e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d1160 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00c705c0` of unit bhkcharactercontroller (the
/// character controller's destructor body), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `00c705c0`'s translated signature `fn_00c705c0(Ptr<Ctl>)` returns nothing.
pub fn fn_008d1160(e: &mut Engine, this: Ptr) {
    e.call(0x00c7_05c0, &args![this]);
}

// Translated from 008d1550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x410` (`SUB ECX,0x410`) and jumps to `008d1130`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_008d1550(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x008d_1130, &args![this.addr().wrapping_sub(0x410), flags])
        .u32()
}

/// The process list the two functions below walk and change (a global
/// `BSSimpleList`-shaped list at `011dfe14`).
const PROCESS_LIST: u32 = 0x011d_fe14;
/// `008256d0` (`thiscall`, no stack words): whether the list is empty (`AL`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`thiscall`, no stack words): the list's first item address.
const LIST_ITEM: u32 = 0x0068_15c0;
/// `0063f7b0` (`thiscall`, no stack words): removes the list's head.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `00905330` (`thiscall`, `RET 4`): takes the address of an item word.
const LIST_ADD: u32 = 0x0090_5330;

// Translated from 008d1560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list at `011dfe14` (no parameters: `ECX` is not read, plain
/// `RET`): while `008256d0` says it is not empty, calls the virtual slot
/// +0x1c4 of the first item and removes the head (`0063f7b0`).
pub fn fn_008d1560(e: &mut Engine) {
    while !e.call(LIST_IS_EMPTY, &args![PROCESS_LIST]).bool() {
        let item_address = e.call(LIST_ITEM, &args![PROCESS_LIST]).u32();
        let item = e.mem.u32(item_address);
        e.vcall(item, 0x1c4, &[]);
        e.call(LIST_REMOVE_HEAD, &args![PROCESS_LIST]);
    }
}

// Translated from 008d15b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl`, one word: when `item` is not null, hands the address of its
/// stack word to `00905330` on the list at `011dfe14`.
pub fn fn_008d15b0(e: &mut Engine, item: u32) {
    if item != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(LIST_ADD, &args![PROCESS_LIST, slot]);
        });
    }
}

// ---- n028: `bhkCharacterListenerArrow` (vtables `01086684`, `01086674`,
// `01086658`; RTTI read at the locators) and `BSPlayerDistanceCheckController`
// (`010867fc`) ----

/// The listener's three vtables (at +0, +0x3E0 and +0x410) and the word
/// holding its arrow (+0x650).
const LISTENER_VTABLE: u32 = 0x0108_6684;
const LISTENER_VTABLE_2: u32 = 0x0108_6674;
const LISTENER_VTABLE_3: u32 = 0x0108_6658;
const LISTENER_ARROW: u32 = 0x650;
/// The player (`PlayerCharacter*`, read from this word).
const PLAYER: u32 = 0x011d_ea3c;
/// `009c69e0` (cdecl `(body)`): the reference that owns a body.
const BODY_REFERENCE: u32 = 0x009c_69e0;
/// `0089f4e0` (`thiscall` on the player): a reference the code compares the
/// body's owner with.
const PLAYER_REFERENCE: u32 = 0x0089_f4e0;
/// `0043b540` (`thiscall` on a body): its filter word; `008c71b0`
/// (`thiscall(word)`): stores it at `this`; `0043b4d0` / `004a3a20`
/// (`thiscall`): its low 7 bits / upper 16 bits.
const BODY_FILTER_WORD: u32 = 0x0043_b540;
const STORE_WORD: u32 = 0x008c_71b0;
const FILTER_LOW_BITS: u32 = 0x0043_b4d0;
const FILTER_HIGH_HALF: u32 = 0x004a_3a20;
/// `004a3f10` (`thiscall(source)`): copies the 16-byte vector at the source
/// to `this`; `005532a0` (`thiscall`, four floats): fills the vector at
/// `this`, returns `this`.
const VECTOR_COPY: u32 = 0x004a_3f10;
const VECTOR_SET: u32 = 0x0055_32a0;
/// `bhkCharacterProxy::GethkLinearVelocity` (engine map name of `00807080`).
const LINEAR_VELOCITY: u32 = 0x0080_7080;
/// The no-op constructor of an `NiPoint3` (`006815c0`).
const CONSTRUCT_IN_PLACE: u32 = 0x0068_15c0;

// Translated from 008d10e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkCharacterListenerArrow` constructor: the base (`00c741e0` with
/// `base_argument`), the three vtables and the arrow. Returns `this`.
pub fn fn_008d10e0(e: &mut Engine, this: Ptr, base_argument: u32, arrow: u32) -> Ptr {
    let at = this.addr();
    e.call(0x00c7_41e0, &args![this, base_argument]);
    e.mem.set_u32(at, LISTENER_VTABLE);
    e.mem.set_u32(at + 0x3e0, LISTENER_VTABLE_2);
    e.mem.set_u32(at + 0x410, LISTENER_VTABLE_3);
    e.mem.set_u32(at + LISTENER_ARROW, arrow);
    this
}

// Translated from 008d1130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The listener's scalar deleting destructor: the destructor body
/// (`fn_008d1160`), then `0087e9b0(this)` (cdecl) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_008d1130(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008d1160(e, this);
    if flags & 1 != 0 {
        e.call(0x0087_e9b0, &args![this]);
    }
    this
}

/// The velocity reset of `fn_008d1180` for one contact entry: the entry's
/// second vector (+0x10) takes the listener's linear velocity, the first
/// the vector `(0, 0, 0, 1)` built in `vector`.
fn reset_contact(e: &mut Engine, this: Ptr, entry: u32, vector: u32) {
    let velocity = e.call(LINEAR_VELOCITY, &args![this]).u32();
    e.call(VECTOR_COPY, &args![entry + 0x10, velocity]);
    let set = e
        .call(VECTOR_SET, &args![vector, 0.0f32, 0.0f32, 0.0f32, 1.0f32])
        .u32();
    e.call(VECTOR_COPY, &args![entry, set]);
}

// Translated from 008d1180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d1180 - the /GS cookie check (00ec408c) is not translated
/// Walks the contacts of `list` (count at +0x4C, 0x40-byte entries from
/// +0x48; the bodies through `00807770` on `source`). A body without an
/// owning object (`004b59f0`) whose filter's low bits are not 0x1E has its
/// entry reset (see `reset_contact`); a body owned by the reference
/// `0089f4e0` gives for the player has its entry reset and ends the walk.
pub fn fn_008d1180(e: &mut Engine, this: Ptr, source: u32, list: u32) {
    let count = e.mem.i32(list + 0x4c);
    e.with_stack(0x44, |e, frame| {
        // The filter word (EBP-0x20) and the two vectors (EBP-0x40, -0x60).
        let filter = frame.addr();
        let vector = frame.addr() + 4;
        let mut index = 0i32;
        while index < count {
            let entry = (index as u32)
                .wrapping_mul(0x40)
                .wrapping_add(e.mem.u32(list + 0x48));
            let contact = e.call(0x0080_7770, &args![source, index]).u32();
            let body = e.mem.u32(contact + 0x48);
            let owner = e.call(0x004b_59f0, &args![body]).u32();
            let body = e.mem.u32(contact + 0x48);
            let word = e.call(BODY_FILTER_WORD, &args![body]).u32();
            e.call(STORE_WORD, &args![filter, word]);
            let low = e.call(FILTER_LOW_BITS, &args![filter]).u32();
            if owner == 0 {
                if low != 0x1e {
                    reset_contact(e, this, entry, vector);
                }
            } else {
                let body = e.mem.u32(contact + 0x48);
                let reference = e.call(BODY_REFERENCE, &args![body]).u32();
                if reference != 0 {
                    let player = e.mem.u32(PLAYER);
                    if reference == e.call(PLAYER_REFERENCE, &args![player]).u32() {
                        reset_contact(e, this, entry, vector);
                        return;
                    }
                }
            }
            index += 1;
        }
    });
}

// Translated from 008d12e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The arrow's contact with `body` at `point` (normal `normal`), cdecl.
/// For a body that `004b5950` marks, the owner must be an `Actor`
/// (`__RTDynamicCast`) whose slot 0x22C (with 0) is false, whose process'
/// slot 0x40C is 0 or 6, and whose slot 0x214 is neither 4 nor 9. Bodies of
/// `fn_008cddb0`'s group and those owned by the player's reference
/// (`0089f4e0`) are ignored. The point and normal are copied
/// (`00458620`, `004a3970`); group 1 makes a ground hit (`fn_008cc740`), an
/// `Actor` that `008ace90` does not exclude an actor hit (`fn_008cba30`), and
/// another owner a reference hit (`fn_008cc730` with the owner and the
/// object `004b5a80` gives for the body's `004b59f0`).
pub fn fn_008d12e0(e: &mut Engine, arrow: u32, body: u32, point: u32, normal: u32) {
    let reference = e.call(BODY_REFERENCE, &args![body]).u32();
    let actor = e
        .call(
            0x00ec_43fb,
            &args![reference, 0u32, 0x0118_41ccu32, 0x0118_46d4u32, 0u32],
        )
        .u32();
    if e.call(0x004b_5950, &args![body]).u32() != 0 {
        if actor == 0 || e.vcall(actor, 0x22c, &args![0u32]).bool() {
            return;
        }
        if e.call(0x008d_8520, &args![actor]).u32() != 0 {
            let process = e.call(0x008d_8520, &args![actor]).u32();
            if e.vcall(process, 0x40c, &[]).u32() != 0 {
                let process = e.call(0x008d_8520, &args![actor]).u32();
                if e.vcall(process, 0x40c, &[]).u32() != 6 {
                    return;
                }
            }
        }
        if e.vcall(actor, 0x214, &[]).u32() == 4 || e.vcall(actor, 0x214, &[]).u32() == 9 {
            return;
        }
    }
    e.with_stack(0x28, |e, frame| {
        // The filter word (EBP-0x28), the normal (-0x24) and the point (-0xC).
        let filter = frame.addr();
        let direction = frame.addr() + 4;
        let place = frame.addr() + 0x1c;
        let word = e.call(BODY_FILTER_WORD, &args![body]).u32();
        e.call(STORE_WORD, &args![filter, word]);
        let group = e.call(FILTER_HIGH_HALF, &args![filter]).u32();
        if group == e.call(0x008c_ddb0, &[]).u32() {
            return;
        }
        if reference != 0 {
            let player = e.mem.u32(PLAYER);
            if reference == e.call(PLAYER_REFERENCE, &args![player]).u32() {
                return;
            }
        }
        e.call(CONSTRUCT_IN_PLACE, &args![place]);
        e.call(0x0045_8620, &args![place, point]);
        e.call(CONSTRUCT_IN_PLACE, &args![direction]);
        e.call(0x004a_3970, &args![direction, normal]);
        if e.call(FILTER_HIGH_HALF, &args![filter]).u32() == 1 {
            e.call(0x008c_c740, &args![arrow, place, direction]);
            return;
        }
        if reference == 0 {
            return;
        }
        if actor != 0 && !e.call(0x008a_ce90, &args![actor]).bool() {
            e.call(0x008c_ba30, &args![arrow, place, direction, actor]);
            return;
        }
        if actor == 0 {
            let owner = e.call(0x004b_59f0, &args![body]).u32();
            let object = e.call(0x004b_5a80, &args![owner]).u32();
            e.call(
                0x008c_c730,
                &args![arrow, place, direction, reference, object],
            );
        }
    });
}

// Translated from 008d14e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The listener's contact callback: while its arrow is flying (state 0,
/// `008ce390`) and not tested out (`00440d80`), the contact goes to
/// `fn_008d12e0` with its body (+0x48), point (`006815c0` on it) and normal
/// (`00460140`).
pub fn fn_008d14e0(e: &mut Engine, this: Ptr, contact: u32) {
    let arrow = e.mem.u32(this.addr() + LISTENER_ARROW);
    if arrow == 0
        || e.call(0x008c_e390, &args![arrow]).u32() != 0
        || e.call(0x0044_0d80, &args![arrow]).bool()
    {
        return;
    }
    let normal = e.call(0x0046_0140, &args![contact]).u32();
    let point = e.call(CONSTRUCT_IN_PLACE, &args![contact]).u32();
    let body = e.mem.u32(contact + 0x48);
    fn_008d12e0(e, arrow, body, point, normal);
}

// Translated from 008d15d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSPlayerDistanceCheckController` constructor (RTTI of its vtable
/// `010867fc`): the base constructor `00a6d220` (in `nitimecontroller.cpp` by the engine map), the vtable, 0.0 at
/// +0x34 and 0 at +0x38. Returns `this`.
pub fn fn_008d15d0(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.call(0x00a6_d220, &args![this]);
    e.mem.set_u32(at, 0x0108_67fc);
    e.mem.set_u32(at + 0x34, 0.0f32.to_bits());
    e.mem.set_u32(at + 0x38, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_10e0, fn_008d10e0(Ptr, u32, u32) -> Ptr),
        entry!(0x008d_1130, fn_008d1130(Ptr, u32) -> Ptr),
        entry!(0x008d_1180, fn_008d1180(Ptr, u32, u32)),
        entry!(0x008d_12e0, fn_008d12e0(u32, u32, u32, u32)),
        entry!(0x008d_14e0, fn_008d14e0(Ptr, u32)),
        entry!(0x008d_15d0, fn_008d15d0(Ptr) -> Ptr),
        entry!(0x008d_1160, fn_008d1160(Ptr)),
        entry!(0x008d_1550, fn_008d1550(Ptr, u32) -> u32),
        entry!(0x008d_1560, fn_008d1560()),
        entry!(0x008d_15b0, fn_008d15b0(u32)),
    ]
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_008d1160_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00c7_05c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_1160, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00c7_05c0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_008d1550() {
        assert_eq!(check(0x008d_1550, 0x008d_1130, 0x410), 0xdead_beef);
    }
}

#[cfg(test)]
mod process_list_b039_tests {
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
    fn fn_008d1560_runs_slot_0x1c4_on_each_item_until_empty() {
        let mut e = Engine::new();
        let items = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0; 0x72]);
        e.mem.set_u32(0x0300_0000 + 0x1c4, 0x0300_1000);
        e.mem.set_u32(items, 0x0300_0000);
        e.mem.set_u32(items + 8, 0x0300_0000);
        // A list of two items: the head index lives in a counter.
        let slot = e.mem.alloc(8);
        e.mem.set_u32(slot, items);
        e.mem.set_u32(slot + 4, 2);
        e.register_double(LIST_IS_EMPTY, move |e, _| Ret {
            eax: u32::from(e.mem.u32(slot + 4) == 0),
            ..Ret::default()
        });
        e.register_double(LIST_ITEM, move |_, _| Ret {
            eax: slot,
            ..Ret::default()
        });
        e.register_double(LIST_REMOVE_HEAD, move |e, _| {
            let head = e.mem.u32(slot);
            e.mem.set_u32(slot, head + 8);
            let left = e.mem.u32(slot + 4);
            e.mem.set_u32(slot + 4, left - 1);
            Ret::default()
        });
        e.register(0x0300_1000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_1560, &[]);
        assert_eq!(
            calls_to(&e, 0x0300_1000),
            vec![vec![items], vec![items + 8]]
        );
        assert_eq!(calls_to(&e, LIST_REMOVE_HEAD).len(), 2);
        assert!(calls_to(&e, LIST_REMOVE_HEAD)
            .iter()
            .all(|w| w == &vec![PROCESS_LIST]));
    }

    #[test]
    fn fn_008d15b0_adds_only_a_non_null_item() {
        let mut e = Engine::new();
        e.register_double(LIST_ADD, |e, a| Ret {
            eax: e.mem.u32(a[1]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x008d_15b0, &args![0u32]);
        assert!(calls_to(&e, LIST_ADD).is_empty());
        e.call(0x008d_15b0, &args![0x1234u32]);
        let calls = calls_to(&e, LIST_ADD);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], PROCESS_LIST);
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, callee: u32, eax: u32) {
        e.register_double(callee, move |_, _| ret(eax));
    }

    fn stub_all(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    fn object(e: &mut Engine, size: u32, base: u32) -> u32 {
        let this = e.mem.alloc(size);
        let table = e.mem.alloc(0x800);
        for slot in (0..0x800).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
        }
        e.mem.set_u32(this, table);
        this
    }

    #[test]
    fn fn_008d10e0_builds_the_listener() {
        let mut e = Engine::new();
        stub_all(&mut e, &[0x00c7_41e0]);
        let this = e.mem.alloc(0x654);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008d_10e0, &[this, 0x77, 0xa770]).u32(), this);
        assert_eq!(calls_to(&e, 0x00c7_41e0), vec![vec![this, 0x77]]);
        assert_eq!(e.mem.u32(this), LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x3e0), LISTENER_VTABLE_2);
        assert_eq!(e.mem.u32(this + 0x410), LISTENER_VTABLE_3);
        assert_eq!(e.mem.u32(this + LISTENER_ARROW), 0xa770);
    }

    #[test]
    fn fn_008d1130_frees_only_with_bit_0() {
        let mut e = Engine::new();
        stub_all(&mut e, &[0x00c7_05c0, 0x0087_e9b0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008d_1130, &[0x9000, 0]).u32(), 0x9000);
        assert!(calls_to(&e, 0x0087_e9b0).is_empty());
        e.call(0x008d_1130, &[0x9000, 1]);
        assert_eq!(calls_to(&e, 0x0087_e9b0), vec![vec![0x9000]]);
        assert_eq!(calls_to(&e, 0x00c7_05c0).len(), 2);
    }

    #[test]
    fn fn_008d1180_resets_the_contacts_that_qualify() {
        let mut e = Engine::new();
        e.map(PLAYER, 4);
        e.mem.set_u32(PLAYER, 0x9a7e);
        let list = e.mem.alloc(0x50);
        let entries = e.mem.alloc(0x40 * 3);
        e.mem.set_u32(list + 0x48, entries);
        e.mem.set_u32(list + 0x4c, 3);
        let contacts: Vec<u32> = (0..3)
            .map(|i| {
                let c = e.mem.alloc(0x4c);
                e.mem.set_u32(c + 0x48, 0xb0d0 + i);
                c
            })
            .collect();
        e.register_double(0x0080_7770, move |_, a| ret(contacts[a[1] as usize]));
        // Body 0: no owner, low bits 5: reset. Body 1: no owner, 0x1e: kept.
        // Body 2: owned by the player's reference: reset and stop.
        e.register(0x004b_59f0, |_, a| ret(u32::from(a[0] == 0xb0d2)));
        e.register(BODY_FILTER_WORD, |_, a| {
            ret(if a[0] == 0xb0d1 { 0x1e } else { 5 })
        });
        e.register(STORE_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(FILTER_LOW_BITS, |e, a| ret(e.mem.u32(a[0]) & 0x7f));
        stub(&mut e, BODY_REFERENCE, 0x5e5e);
        stub(&mut e, PLAYER_REFERENCE, 0x5e5e);
        stub(&mut e, LINEAR_VELOCITY, 0x7e10);
        stub_all(&mut e, &[VECTOR_COPY]);
        e.register(VECTOR_SET, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x008d_1180, &[0x4000, 0x5000, list]);
        let copies = calls_to(&e, VECTOR_COPY);
        assert_eq!(copies.len(), 4);
        assert_eq!(copies[0][..2], [entries + 0x10, 0x7e10]);
        assert_eq!(copies[1][0], entries);
        assert_eq!(copies[2][..2], [entries + 0x90, 0x7e10]);
        assert_eq!(copies[3][0], entries + 0x80);
        let set = calls_to(&e, VECTOR_SET);
        assert_eq!(set[0][1..], [0, 0, 0, 1.0f32.to_bits()]);
        assert_eq!(calls_to(&e, BODY_REFERENCE), vec![vec![0xb0d2]]);
    }

    /// The doubles of the contact handler with a body owned by `owner`.
    fn contact_setup(e: &mut Engine, owner: u32, actor: u32, group: u32) {
        e.map(PLAYER, 4);
        e.mem.set_u32(PLAYER, 0x9a7e);
        stub(e, BODY_REFERENCE, owner);
        stub(e, 0x00ec_43fb, actor);
        stub(e, 0x004b_5950, 0);
        stub(e, BODY_FILTER_WORD, group << 16);
        e.register(STORE_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(FILTER_HIGH_HALF, |e, a| ret(e.mem.u32(a[0]) >> 16));
        stub(e, 0x008c_ddb0, 0x42);
        stub(e, PLAYER_REFERENCE, 0x5e5e);
        stub_all(
            e,
            &[
                CONSTRUCT_IN_PLACE,
                0x0045_8620,
                0x004a_3970,
                0x008c_c740,
                0x008c_ba30,
                0x008c_c730,
            ],
        );
        stub(e, 0x008a_ce90, 0);
        stub(e, 0x004b_59f0, 0x0b0b);
        stub(e, 0x004b_5a80, 0x0c0c);
    }

    #[test]
    fn fn_008d12e0_sends_each_contact_to_its_handler() {
        let mut e = Engine::new();
        contact_setup(&mut e, 0, 0, 1);
        e.call_log = Some(vec![]);
        e.call(0x008d_12e0, &[0xa770, 0xb0d0, 0x1111, 0x2222]);
        let ground = calls_to(&e, 0x008c_c740);
        assert_eq!(ground.len(), 1);
        assert_eq!(ground[0][0], 0xa770);
        assert_eq!(calls_to(&e, 0x0045_8620), vec![vec![ground[0][1], 0x1111]]);
        assert_eq!(calls_to(&e, 0x004a_3970), vec![vec![ground[0][2], 0x2222]]);
        // An actor owner (group 3): the actor hit.
        let mut e = Engine::new();
        let actor = object(&mut e, 0x100, 0x7200_0000);
        contact_setup(&mut e, 0x5e00, actor, 3);
        e.call_log = Some(vec![]);
        e.call(0x008d_12e0, &[0xa770, 0xb0d0, 0x1111, 0x2222]);
        assert_eq!(calls_to(&e, 0x008c_ba30)[0][3], actor);
        // Another owner: the reference hit with the body's object.
        let mut e = Engine::new();
        contact_setup(&mut e, 0x5e00, 0, 3);
        e.call_log = Some(vec![]);
        e.call(0x008d_12e0, &[0xa770, 0xb0d0, 0x1111, 0x2222]);
        assert_eq!(calls_to(&e, 0x008c_c730)[0][3..], [0x5e00, 0x0c0c]);
        // The arrow's own group and the player's reference are ignored.
        let mut e = Engine::new();
        contact_setup(&mut e, 0x5e00, 0, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x008d_12e0, &[0xa770, 0xb0d0, 0x1111, 0x2222]);
        assert!(calls_to(&e, 0x008c_c730).is_empty());
        let mut e = Engine::new();
        contact_setup(&mut e, 0x5e5e, 0, 3);
        e.call_log = Some(vec![]);
        e.call(0x008d_12e0, &[0xa770, 0xb0d0, 0x1111, 0x2222]);
        assert!(calls_to(&e, 0x008c_c730).is_empty());
    }

    #[test]
    fn fn_008d12e0_checks_a_marked_body_owner() {
        let mut e = Engine::new();
        let actor = object(&mut e, 0x100, 0x7200_0000);
        contact_setup(&mut e, 0x5e00, actor, 3);
        stub(&mut e, 0x004b_5950, 1);
        stub(&mut e, 0x7200_022c, 0);
        stub(&mut e, 0x008d_8520, 0);
        stub(&mut e, 0x7200_0214, 4);
        e.call_log = Some(vec![]);
        e.call(0x008d_12e0, &[0xa770, 0xb0d0, 0x1111, 0x2222]);
        assert!(calls_to(&e, 0x008c_ba30).is_empty());
        stub(&mut e, 0x7200_0214, 2);
        e.call_log = Some(vec![]);
        e.call(0x008d_12e0, &[0xa770, 0xb0d0, 0x1111, 0x2222]);
        assert_eq!(calls_to(&e, 0x008c_ba30).len(), 1);
        // No actor at all: nothing.
        stub(&mut e, 0x00ec_43fb, 0);
        e.call_log = Some(vec![]);
        e.call(0x008d_12e0, &[0xa770, 0xb0d0, 0x1111, 0x2222]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn fn_008d14e0_passes_a_flying_arrows_contact() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x654);
        e.mem.set_u32(this + LISTENER_ARROW, 0xa770);
        stub(&mut e, 0x008c_e390, 0);
        stub(&mut e, 0x0044_0d80, 0);
        stub(&mut e, 0x0046_0140, 0x2222);
        e.register(CONSTRUCT_IN_PLACE, |_, a| ret(a[0]));
        e.register(BODY_REFERENCE, |_, _| ret(0));
        e.register(0x00ec_43fb, |_, _| ret(0));
        stub(&mut e, 0x004b_5950, 1);
        let contact = e.mem.alloc(0x4c);
        e.mem.set_u32(contact + 0x48, 0xb0d0);
        e.call_log = Some(vec![]);
        e.call(0x008d_14e0, &[this, contact]);
        assert_eq!(calls_to(&e, BODY_REFERENCE), vec![vec![0xb0d0]]);
        // A stuck arrow (state not 0): nothing.
        stub(&mut e, 0x008c_e390, 2);
        e.call_log = Some(vec![]);
        e.call(0x008d_14e0, &[this, contact]);
        assert!(calls_to(&e, BODY_REFERENCE).is_empty());
    }

    #[test]
    fn fn_008d15d0_builds_the_controller() {
        let mut e = Engine::new();
        stub_all(&mut e, &[0x00a6_d220]);
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this + 0x34, 9);
        e.mem.set_u32(this + 0x38, 9);
        assert_eq!(e.call(0x008d_15d0, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_67fc);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
    }
}
