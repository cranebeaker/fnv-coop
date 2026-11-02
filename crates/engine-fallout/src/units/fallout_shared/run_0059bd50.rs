//! `fallout shared/run_0059bd50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `0059bd50` and `0059be50` cast their argument from `TESForm` to
//! `TESCaravanDeck` (type descriptor `.?AVTESCaravanDeck@@` at
//! `0x0118_643c`); the other six are this-adjusting thunks.

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm` type descriptor (`.?AVTESForm@@`).
const TESFORM_TYPE: u32 = 0x0118_3028;
/// `TESCaravanDeck` type descriptor (`.?AVTESCaravanDeck@@`).
const CARAVAN_DECK_TYPE: u32 = 0x0118_643c;
/// The list head of a deck (`0059bb30`, `thiscall`; the engine map's name
/// `D3DTexture_LockRect` is a folded body).
const LIST_HEAD: u32 = 0x0059_bb30;
/// A list node's item slot (`006815c0`, `thiscall`): returns a pointer to
/// the node's item word.
const NODE_ITEM: u32 = 0x0068_15c0;
/// The next list node (`00726070`, `thiscall`).
const NEXT_NODE: u32 = 0x0072_6070;
/// The word read through `004610d0` (`thiscall`): a pointer to the 4 bytes
/// kept at `+0x28`.
const VALUE_AT_28: u32 = 0x0046_10d0;
/// Checks the list kept at `+0x24` (`008256d0`, `thiscall`).
const LIST_CHECK: u32 = 0x0082_56d0;

fn cast_to_caravan_deck(e: &mut Engine, form: u32) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, TESFORM_TYPE, CARAVAN_DECK_TYPE, 0u32],
    )
    .u32()
}

// Translated from 0059bd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `TESCaravanDeck` (`form` cast from `TESForm`; nothing happens
/// when the cast fails): `TESForm::CopyAllComponents` (`004851b0`), then
/// empties the list at `+0x24` (`0063f7b0` until `008256d0` answers true,
/// as long as there is one), adds every item of the source's list to it
/// (`005ae3d0`, or `00726c60` when the list is missing or `008256d0`
/// answers true, each with the item's address), copies the word at
/// `+0x28` (`004610d0`) and finally calls `004f15a0` with `00401170` of
/// the source.
pub fn fn_0059bd50(e: &mut Engine, this: Ptr, form: u32) {
    let base = this.addr();
    let source = cast_to_caravan_deck(e, form);
    if source == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, source]);
    loop {
        let list = e.mem.u32(base + 0x24);
        if list == 0 || e.call(LIST_CHECK, &args![list]).bool() {
            break;
        }
        e.call(0x0063_f7b0, &args![list]);
    }
    e.with_stack(4, |e, item| {
        let mut node = e.call(LIST_HEAD, &args![source]).u32();
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let value = e.mem.u32(slot);
            e.mem.set_u32(item.addr(), value);
            let list = e.mem.u32(base + 0x24);
            if list == 0 || e.call(LIST_CHECK, &args![list]).bool() {
                let list = e.mem.u32(base + 0x24);
                e.call(0x0072_6c60, &args![list, item]);
            } else {
                let list = e.mem.u32(base + 0x24);
                e.call(0x005a_e3d0, &args![list, item]);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
    });
    let value = e.call(VALUE_AT_28, &args![source]).u32();
    let word = e.mem.u32(value);
    e.mem.set_u32(base + 0x28, word);
    let id = e.call(0x0040_1170, &args![source]).u32();
    e.call(0x004f_15a0, &args![this, id]);
}

// Translated from 0059be50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a `TESCaravanDeck` differs from `form`: true when the cast
/// fails, when `TESForm::CompareAllComponents` (`00485270`) answers true,
/// when the two lists' counts (`005ae380`) differ, when `005f65d0` on this
/// deck's list does not find one of the other deck's items (by the item's
/// address), or when the 4 bytes at `+0x28` differ (`memcmp`).
pub fn fn_0059be50(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_caravan_deck(e, form);
    if other == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, other]).bool() {
        return true;
    }
    let own_list = e.call(LIST_HEAD, &args![this]).u32();
    let own_count = e.call(0x005a_e380, &args![own_list]).u32();
    let other_list = e.call(LIST_HEAD, &args![other]).u32();
    let other_count = e.call(0x005a_e380, &args![other_list]).u32();
    if own_count != other_count {
        return true;
    }
    let mut node = e.call(LIST_HEAD, &args![other]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let own_list = e.call(LIST_HEAD, &args![this]).u32();
        if !e.call(0x005f_65d0, &args![own_list, slot]).bool() {
            return true;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    let value = e.call(VALUE_AT_28, &args![other]).u32();
    e.call(0x00ec_4835, &args![this.addr() + 0x28, value, 4u32])
        .u32()
        != 0
}

// Translated from 0059bf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x3c` and jumps to `0059af90` (a scalar
/// deleting destructor, `RET 4`). Returns its `EAX`.
pub fn fn_0059bf40(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0059_af90, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 0059bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x78` and jumps to `00616640` (`RET 8`).
/// Returns its `EAX`.
pub fn fn_0059bf50(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    e.call(
        0x0061_6640,
        &args![this.addr().wrapping_sub(0x78), first, second],
    )
    .u32()
}

// Translated from 0059bf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x78` and jumps to `0059adb0` (`RET 4`).
/// Returns its `EAX`.
pub fn fn_0059bf60(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    e.call(0x0059_adb0, &args![this.addr().wrapping_sub(0x78), value])
        .u32()
}

// Translated from 0059bf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x3c` and jumps to `0059a4c0` (a scalar
/// deleting destructor, `RET 4`). Returns its `EAX`.
pub fn fn_0059bf70(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0059_a4c0, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 0059bf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x78` and jumps to `005a7770` (`RET 4`).
/// Returns its `EAX`.
pub fn fn_0059bf80(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    e.call(0x005a_7770, &args![this.addr().wrapping_sub(0x78), value])
        .u32()
}

// Translated from 0059bf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x78` and jumps to `0059b850` (`RET 4`).
/// Returns its `EAX`.
pub fn fn_0059bf90(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    e.call(0x0059_b850, &args![this.addr().wrapping_sub(0x78), value])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_bd50, fn_0059bd50(Ptr, u32)),
        entry!(0x0059_be50, fn_0059be50(Ptr, u32) -> bool),
        entry!(0x0059_bf40, fn_0059bf40(Ptr, u32) -> u32),
        entry!(0x0059_bf50, fn_0059bf50(Ptr, u32, u32) -> u32),
        entry!(0x0059_bf60, fn_0059bf60(Ptr, u32) -> u32),
        entry!(0x0059_bf70, fn_0059bf70(Ptr, u32) -> u32),
        entry!(0x0059_bf80, fn_0059bf80(Ptr, u32) -> u32),
        entry!(0x0059_bf90, fn_0059bf90(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Lists as chains of 8-byte nodes (item, next); `0059bb30` answers the
    /// word at `deck + 0x40` (a test-only head slot), the item slot is the
    /// node itself.
    fn deck_engine() -> Engine {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(LIST_HEAD, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(VALUE_AT_28, |_, a| (a[0] + 0x28).into_ret());
        e.register(0x0040_1170, |_, a| (a[0] + 1).into_ret());
        for a in [0x0048_51b0, 0x004f_15a0, 0x0072_6c60, 0x005a_e3d0] {
            e.register(a, noop);
        }
        e
    }

    fn chain(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(12);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn fn_0059bd50_copies_the_list_and_the_word() {
        let mut e = deck_engine();
        // The list at +0x24 answers "not empty" twice, then "empty".
        let checks = Rc::new(RefCell::new(0u32));
        let seen = checks.clone();
        e.register_double(LIST_CHECK, move |_, _| {
            *seen.borrow_mut() += 1;
            u32::from(*seen.borrow() > 2).into_ret()
        });
        e.register(0x0063_f7b0, noop);
        let this = e.mem.alloc(0x44);
        let source = e.mem.alloc(0x44);
        e.mem.set_u32(this + 0x24, 0x5000);
        e.mem.set_u32(source + 0x28, 0xabcd);
        let head = chain(&mut e, &[0x11, 0x22, 0]);
        e.mem.set_u32(source + 0x40, head);
        e.call_log = Some(vec![]);
        e.call(0x0059_bd50, &args![this, source]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![source, 0, TESFORM_TYPE, CARAVAN_DECK_TYPE, 0]]
        );
        assert_eq!(calls_to(&e, 0x0048_51b0), vec![vec![this, source]]);
        assert_eq!(calls_to(&e, 0x0063_f7b0), vec![vec![0x5000], vec![0x5000]]);
        // Both items go through 00726c60 (the check now answers true); the
        // third node's empty item ends the walk.
        assert_eq!(calls_to(&e, 0x005a_e3d0).len(), 0);
        assert_eq!(calls_to(&e, 0x0072_6c60).len(), 2);
        assert_eq!(e.mem.u32(this + 0x28), 0xabcd);
        assert_eq!(calls_to(&e, 0x004f_15a0), vec![vec![this, source + 1]]);
    }

    #[test]
    fn fn_0059bd50_uses_the_other_add_while_the_check_fails() {
        let mut e = deck_engine();
        e.register(LIST_CHECK, |_, _| 0u32.into_ret());
        e.register(0x0063_f7b0, |e, a| {
            // Emptying stops once the list is gone.
            let this = e.mem.u32(0x7ff0_0000);
            if a[0] != 0 {
                e.mem.set_u32(this + 0x24, 0);
            }
            Ret::default()
        });
        e.map(0x7ff0_0000, 4);
        let this = e.mem.alloc(0x44);
        e.set_global(0x7ff0_0000, this);
        let source = e.mem.alloc(0x44);
        let head = chain(&mut e, &[0x11]);
        e.mem.set_u32(source + 0x40, head);
        e.mem.set_u32(this + 0x24, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0059_bd50, &args![this, source]);
        // The list pointer was cleared, so the item goes to 00726c60 on null.
        assert_eq!(calls_to(&e, 0x0072_6c60).len(), 1);
        assert_eq!(calls_to(&e, 0x0072_6c60)[0][0], 0);
    }

    #[test]
    fn fn_0059bd50_ignores_other_forms() {
        let mut e = deck_engine();
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0059_bd50, &args![0x4000u32, 0x5000u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    fn compare_engine(components_differ: bool, counts: (u32, u32), found: bool) -> Engine {
        let mut e = deck_engine();
        e.register_double(0x0048_5270, move |_, _| {
            u32::from(components_differ).into_ret()
        });
        e.register_double(0x005a_e380, move |e, a| {
            // The own list head is recognised by the test marker at +8.
            let own = e.mem.u32(a[0] + 8) == 1;
            if own { counts.0 } else { counts.1 }.into_ret()
        });
        e.register_double(0x005f_65d0, move |_, _| u32::from(found).into_ret());
        e.register(0x00ec_4835, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        e
    }

    fn decks(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x44);
        let other = e.mem.alloc(0x44);
        let own_head = e.mem.alloc(12);
        e.mem.set_u32(own_head + 8, 1);
        e.mem.set_u32(this + 0x40, own_head);
        let other_head = chain(e, &[0x11, 0x22]);
        e.mem.set_u32(other + 0x40, other_head);
        (this, other)
    }

    #[test]
    fn fn_0059be50_reports_differences() {
        // Equal: same components, counts, items and word.
        let mut e = compare_engine(false, (2, 2), true);
        let (this, other) = decks(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0059_be50, &args![this, other]).bool());
        assert_eq!(calls_to(&e, 0x005f_65d0).len(), 2);
        assert_eq!(
            calls_to(&e, 0x00ec_4835),
            vec![vec![this + 0x28, other + 0x28, 4]]
        );
        // The word differs.
        e.mem.set_u32(other + 0x28, 9);
        assert!(e.call(0x0059_be50, &args![this, other]).bool());

        let mut e = compare_engine(true, (2, 2), true);
        let (this, other) = decks(&mut e);
        assert!(e.call(0x0059_be50, &args![this, other]).bool());

        let mut e = compare_engine(false, (1, 2), true);
        let (this, other) = decks(&mut e);
        assert!(e.call(0x0059_be50, &args![this, other]).bool());

        let mut e = compare_engine(false, (2, 2), false);
        let (this, other) = decks(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_be50, &args![this, other]).bool());
        assert_eq!(calls_to(&e, 0x005f_65d0).len(), 1);
        assert!(calls_to(&e, 0x00ec_4835).is_empty());

        let mut e = compare_engine(false, (2, 2), true);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0059_be50, &args![this, other]).bool());
    }

    #[test]
    fn thunks_move_this_back_and_pass_the_words_on() {
        let cases: [(u32, u32, u32, usize); 6] = [
            (0x0059_bf40, 0x0059_af90, 0x3c, 1),
            (0x0059_bf50, 0x0061_6640, 0x78, 2),
            (0x0059_bf60, 0x0059_adb0, 0x78, 1),
            (0x0059_bf70, 0x0059_a4c0, 0x3c, 1),
            (0x0059_bf80, 0x005a_7770, 0x78, 1),
            (0x0059_bf90, 0x0059_b850, 0x78, 1),
        ];
        for (thunk, target, adjust, words) in cases {
            let mut e = Engine::new();
            e.register(target, |_, a| (a[0] ^ 0xff).into_ret());
            let mut call = vec![0x1000u32, 5, 6];
            call.truncate(1 + words);
            e.call_log = Some(vec![]);
            let r = e.call(thunk, &call).u32();
            let mut want = call.clone();
            want[0] = 0x1000 - adjust;
            assert_eq!(calls_to(&e, target), vec![want]);
            assert_eq!(r, (0x1000 - adjust) ^ 0xff);
        }
    }
}
