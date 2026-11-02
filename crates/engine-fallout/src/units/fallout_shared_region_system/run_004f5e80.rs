//! `fallout shared/region system/run_004f5e80` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`thiscall`, no stack word): the address of the list's head item word.
const HEAD_ITEM_SLOT: u32 = 0x0068_15c0;
/// `0063f7b0` (`thiscall`, no stack word): removes the list's head.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;

// Translated from 004f5ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): empties the list at +4. While its
/// head item (`006815c0`) is not null, removes the head (`0063f7b0`) and, when the byte at
/// +0xc is set, calls the item's vtable slot +8 with 1. Then clears the word at +0x10.
pub fn fn_004f5ef0(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 4;
    let slot = e.call(HEAD_ITEM_SLOT, &args![list]).u32();
    let mut item = e.mem.u32(slot);
    while item != 0 {
        e.call(LIST_REMOVE_HEAD, &args![list]);
        if e.mem.u8(this.addr() + 0xc) != 0 && item != 0 {
            e.vcall(item, 8, &args![1u32]);
        }
        let slot = e.call(HEAD_ITEM_SLOT, &args![list]).u32();
        item = e.mem.u32(slot);
    }
    e.mem.set_u32(this.addr() + 0x10, 0);
}

/// `005f65d0` (`thiscall`, `RET 4`: the address of an item word): whether
/// the list already holds the item.
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// `00905820` (`thiscall`, `RET 4`: the address of an item word): adds the
/// item to the list.
const LIST_ADD: u32 = 0x0090_5820;
/// `00726070` (`thiscall`): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0084e3a0` (`thiscall`): the form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// The game's allocator (`00401000`, cdecl: size).
const ALLOCATE: u32 = 0x0040_1000;
/// `004f5db0` (`thiscall`, `RET 4`: the ownership byte): the list object's
/// constructor.
const LIST_CONSTRUCT: u32 = 0x004f_5db0;
/// The `double` constants `004f61d0` compares and divides with (`01012060`
/// 0.0, `01012070` 1.0 and `01017a40` 100.0 in the exe).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;

// Translated from 004f5e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): adds `item` to the list
/// at +4 (`004f5ec0`) unless it is null or already there (`005f65d0` with
/// the address of the argument word). True when added.
pub fn fn_004f5e80(e: &mut Engine, this: Ptr, item: u32) -> bool {
    if item == 0 {
        return false;
    }
    let held = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_CONTAINS, &args![this.addr() + 4, slot]).bool()
    });
    if held {
        return false;
    }
    fn_004f5ec0(e, this, item);
    true
}

// Translated from 004f5ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): adds `item` to the list
/// at +4 (`00905820` with the address of the argument word) and counts it
/// in the word at +0x10.
pub fn fn_004f5ec0(e: &mut Engine, this: Ptr, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_ADD, &args![this.addr() + 4, slot]);
    });
    let count = e.mem.u32(this.addr() + 0x10).wrapping_add(1);
    e.mem.set_u32(this.addr() + 0x10, count);
}

// Translated from 004f5f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): the first item of the
/// list at +4 (none when `this` is null; the walk stops at an empty item)
/// whose form at vtable `+4` has the id `first`, and either, for
/// `second >= 0`, a form at `+0xc` with the id `second`, or, for
/// `second < 0`, no form at `+0xc`. Null when none matches.
pub fn fn_004f5f80(e: &mut Engine, this: Ptr, first: u32, second: i32) -> u32 {
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while node != 0 {
        let item_address = e.call(HEAD_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            return 0;
        }
        let item_address = e.call(HEAD_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(item_address);
        let form = e.vcall(item, 4, &args![]).u32();
        if e.call(FORM_ID, &args![form]).u32() == first {
            if second >= 0 && e.vcall(item, 0xc, &args![]).u32() != 0 {
                let other = e.vcall(item, 0xc, &args![]).u32();
                if e.call(FORM_ID, &args![other]).i32() == second {
                    return item;
                }
            }
            if second < 0 && e.vcall(item, 0xc, &args![]).u32() == 0 {
                return item;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 004f6060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): copies the list at +4
/// of `this` into `target` and answers the target. Without a target a new
/// 0x14-byte list object is made (`00401000`, then `004f5db0` with this
/// one's ownership byte at +0xc; null when the allocation fails). Until an
/// empty item: when the target owns its items (+0xc) each item is
/// duplicated (vtable `+0x14` with 0) and added (`004f5e80`), a duplicate
/// that was not added being deleted (vtable `+8` with 1); otherwise each
/// item is added as it is.
// Translation incomplete: 004f6060 - the C++ exception frame (FS:[0] chain, unwind state 0 that frees the new list) and the /GS cookie word are not translated.
pub fn fn_004f6060(e: &mut Engine, this: Ptr, target: u32) -> u32 {
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    let target = if target != 0 {
        target
    } else {
        let block = e.call(ALLOCATE, &args![0x14u32]).u32();
        if block == 0 {
            0
        } else {
            let owns = u32::from(e.mem.u8(this.addr() + 0xc));
            e.call(LIST_CONSTRUCT, &args![block, owns]).u32()
        }
    };
    while node != 0 {
        let item_address = e.call(HEAD_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        if e.mem.u8(target.wrapping_add(0xc)) != 0 {
            let item_address = e.call(HEAD_ITEM_SLOT, &args![node]).u32();
            let item = e.mem.u32(item_address);
            let copy = e.vcall(item, 0x14, &args![0u32]).u32();
            if !fn_004f5e80(e, Ptr::new(target), copy) && copy != 0 {
                e.vcall(copy, 8, &args![1u32]);
            }
        } else {
            let item_address = e.call(HEAD_ITEM_SLOT, &args![node]).u32();
            let item = e.mem.u32(item_address);
            fn_004f5e80(e, Ptr::new(target), item);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    target
}

// Translated from 004f61d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: a float, a list
/// object, a float): each float above 1.0 is taken as a percentage
/// (divided by 100.0, stored as a float). With a list and both values not
/// below 0.0 and not above 1.0 (NaN passes, as the exe's flag tests let
/// it), each item of the source's list at +4 (until an empty item) that
/// has a form at vtable `+4` is duplicated (`+0x14` with 0) and added to
/// `this` (`004f5e80`). The two values are only checked, never used
/// otherwise.
pub fn fn_004f61d0(e: &mut Engine, this: Ptr, first: f32, source: u32, second: f32) {
    let one: f64 = e.global(ONE_DOUBLE);
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let percent = |value: f32| {
        let loaded = x87_load32(value.to_bits());
        if loaded > one {
            x87_store32(x87_div(loaded, hundred))
        } else {
            x87_quiet_bits(value.to_bits())
        }
    };
    let first = percent(first);
    let second = percent(second);
    if source == 0 {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    // `FCOMP` with `TEST AH,5`/`JNP` and `TEST AH,0x41`/`JZ`: only a value
    // below 0.0 or above 1.0 fails.
    let in_range = |bits: u32| {
        let value = x87_load32(bits);
        !(value < zero) && !(value > one)
    };
    if !in_range(first) || !in_range(second) {
        return;
    }
    let mut node = source.wrapping_add(4);
    while node != 0 {
        let item_address = e.call(HEAD_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            return;
        }
        let item_address = e.call(HEAD_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if e.vcall(item, 4, &args![]).u32() != 0 {
            let item_address = e.call(HEAD_ITEM_SLOT, &args![node]).u32();
            let item = e.mem.u32(item_address);
            let copy = e.vcall(item, 0x14, &args![0u32]).u32();
            fn_004f5e80(e, this, copy);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_5ef0, fn_004f5ef0(Ptr)),
        entry!(0x004f_5e80, fn_004f5e80(Ptr, u32) -> bool),
        entry!(0x004f_5ec0, fn_004f5ec0(Ptr, u32)),
        entry!(0x004f_5f80, fn_004f5f80(Ptr, u32, i32) -> u32),
        entry!(0x004f_6060, fn_004f6060(Ptr, u32) -> u32),
        entry!(0x004f_61d0, fn_004f61d0(Ptr, f32, u32, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn run(owns: u8) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0, 0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let items = [e.mem.alloc(4), e.mem.alloc(4)];
        for item in items {
            e.mem.set_u32(item, 0x0150_0000);
        }
        let slot = e.mem.alloc(4);
        let left = Rc::new(Cell::new(2usize));
        let (peek, remove) = (left.clone(), left.clone());
        e.register_double(HEAD_ITEM_SLOT, move |e, _| {
            let item = if peek.get() == 0 {
                0
            } else {
                items[2 - peek.get()]
            };
            e.mem.set_u32(slot, item);
            slot.into_ret()
        });
        e.register_double(LIST_REMOVE_HEAD, move |_, _| {
            remove.set(remove.get() - 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u8(this + 0xc, owns);
        e.mem.set_u32(this + 0x10, 9);
        e.call_log = Some(vec![]);
        e.call(0x004f_5ef0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(left.get(), 0);
        let log = e.call_log.take().unwrap();
        let mut deleted: Vec<_> = log.into_iter().filter(|c| c.0 == 0x0150_1000).collect();
        for call in &mut deleted {
            let item = call.1[0];
            assert!(items.contains(&item));
            call.1[0] = 0;
        }
        deleted
    }

    #[test]
    fn empties_the_list_deleting_items_only_when_owned() {
        assert_eq!(
            run(1),
            vec![(0x0150_1000, vec![0, 1]), (0x0150_1000, vec![0, 1])]
        );
        assert!(run(0).is_empty());
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Nodes are (item, next); the list at +4 of a list object is its first
    /// node. Items: vtable `0x0300_0000`, slot `+4` the word at +4, `+0xc`
    /// the word at +8, `+0x14` the word at +0xc, `+8` deletes.
    fn setup(e: &mut Engine) -> Rc<RefCell<Vec<u32>>> {
        e.put_vtable(
            0x0300_0000,
            &[0, 0x0300_1004, 0x0300_1008, 0x0300_100c, 0, 0x0300_1014],
        );
        e.register(0x0300_1004, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0300_1008, |_, _| Ret::default());
        e.register(0x0300_100c, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0300_1014, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(HEAD_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FORM_ID, |e, a| e.mem.u32(a[0]).into_ret());
        let added: Rc<RefCell<Vec<u32>>> = Rc::default();
        let contains = added.clone();
        e.register_double(LIST_CONTAINS, move |e, a| {
            let item = e.mem.u32(a[1]);
            u32::from(contains.borrow().contains(&item)).into_ret()
        });
        let add = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            add.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        added
    }

    fn item(e: &mut Engine, form: u32, other: u32, copy: u32) -> u32 {
        let i = e.mem.alloc(0x10);
        e.mem.set_u32(i, 0x0300_0000);
        e.mem.set_u32(i + 4, form);
        e.mem.set_u32(i + 8, other);
        e.mem.set_u32(i + 0xc, copy);
        i
    }

    /// A list object (0x14 bytes) holding `items` from +4.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let object = e.mem.alloc(0x14);
        let mut node = object + 4;
        for (i, it) in items.iter().enumerate() {
            e.mem.set_u32(node, *it);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        object
    }

    fn form(e: &mut Engine, id: u32) -> u32 {
        let f = e.mem.alloc(4);
        e.mem.set_u32(f, id);
        f
    }

    #[test]
    fn fn_004f5e80_adds_new_items_and_counts_them() {
        let mut e = Engine::new();
        let added = setup(&mut e);
        let this = e.mem.alloc(0x14);
        assert!(!e.call(0x004f_5e80, &args![this, 0u32]).bool());
        assert!(e.call(0x004f_5e80, &args![this, 0x77u32]).bool());
        assert!(!e.call(0x004f_5e80, &args![this, 0x77u32]).bool());
        assert_eq!(*added.borrow(), vec![0x77]);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        assert_eq!(calls(&e, LIST_ADD)[0][0], this + 4);
    }

    #[test]
    fn fn_004f5ec0_adds_without_checking() {
        let mut e = Engine::new();
        let added = setup(&mut e);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0xffff_ffff);
        e.call(0x004f_5ec0, &args![this, 5u32]);
        e.call(0x004f_5ec0, &args![this, 5u32]);
        assert_eq!(*added.borrow(), vec![5, 5]);
        assert_eq!(e.mem.u32(this + 0x10), 1);
    }

    #[test]
    fn fn_004f5f80_matches_both_forms() {
        let mut e = Engine::new();
        setup(&mut e);
        let a = form(&mut e, 10);
        let b = form(&mut e, 20);
        let c = form(&mut e, 30);
        let lone = item(&mut e, a, 0, 0);
        let paired = item(&mut e, a, b, 0);
        let other = item(&mut e, c, 0, 0);
        let this = list(&mut e, &[other, paired, lone]);
        let find = |e: &mut Engine, first: u32, second: i32| {
            e.call(0x004f_5f80, &args![this, first, second]).u32()
        };
        assert_eq!(find(&mut e, 10, 20), paired);
        assert_eq!(find(&mut e, 10, -1), lone);
        assert_eq!(find(&mut e, 10, 21), 0);
        assert_eq!(find(&mut e, 30, -5), other);
        assert_eq!(e.call(0x004f_5f80, &args![0u32, 10u32, 20u32]).u32(), 0);
    }

    #[test]
    fn fn_004f6060_copies_into_an_owning_target() {
        let mut e = Engine::new();
        let added = setup(&mut e);
        let copy = item(&mut e, 1, 0, 0);
        let source_item = item(&mut e, 1, 0, copy);
        let null_copy_item = item(&mut e, 1, 0, 0);
        let source = list(&mut e, &[source_item, source_item, null_copy_item]);
        let target = e.mem.alloc(0x14);
        e.mem.set_u8(target + 0xc, 1);
        assert_eq!(e.call(0x004f_6060, &args![source, target]).u32(), target);
        assert_eq!(*added.borrow(), vec![copy]);
        // The second duplicate is the same item, already held: deleted.
        assert_eq!(calls(&e, 0x0300_1008), vec![vec![copy, 1]]);
        assert_eq!(calls(&e, 0x0300_1014)[0], vec![source_item, 0]);
    }

    #[test]
    fn fn_004f6060_makes_a_target() {
        let mut e = Engine::new();
        let added = setup(&mut e);
        let made = e.mem.alloc(0x14);
        e.register_double(ALLOCATE, move |_, _| made.into_ret());
        e.register(LIST_CONSTRUCT, |_, a| a[0].into_ret());
        let it = item(&mut e, 1, 0, 0);
        let source = list(&mut e, &[it]);
        e.mem.set_u8(source + 0xc, 1);
        assert_eq!(e.call(0x004f_6060, &args![source, 0u32]).u32(), made);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x14]]);
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![made, 1]]);
        assert_eq!(*added.borrow(), vec![it]);
    }

    fn constants(e: &mut Engine) {
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.set_global::<f64>(ZERO_DOUBLE, 0.0);
        e.set_global::<f64>(ONE_DOUBLE, 1.0);
        e.set_global::<f64>(HUNDRED_DOUBLE, 100.0);
    }

    #[test]
    fn fn_004f61d0_adds_duplicates_of_items_with_forms() {
        let mut e = Engine::new();
        let added = setup(&mut e);
        constants(&mut e);
        let copy = item(&mut e, 1, 0, 0);
        let with_form = item(&mut e, 1, 0, copy);
        let without = item(&mut e, 0, 0, 0x99);
        let source = list(&mut e, &[without, with_form]);
        let this = e.mem.alloc(0x14);
        // 50 is 50%, 0.25 stays; NaN passes the range test.
        e.call(0x004f_61d0, &args![this, 50.0f32, source, 0.25f32]);
        assert_eq!(*added.borrow(), vec![copy]);
        added.borrow_mut().clear();
        e.call(0x004f_61d0, &args![this, f32::NAN, source, 1.0f32]);
        assert_eq!(*added.borrow(), vec![copy]);
    }

    #[test]
    fn fn_004f61d0_refuses_values_out_of_range() {
        let mut e = Engine::new();
        let added = setup(&mut e);
        constants(&mut e);
        let copy = item(&mut e, 1, 0, 0);
        let with_form = item(&mut e, 1, 0, copy);
        let source = list(&mut e, &[with_form]);
        let this = e.mem.alloc(0x14);
        // 150 becomes 1.5: above 1.0.
        e.call(0x004f_61d0, &args![this, 150.0f32, source, 0.5f32]);
        e.call(0x004f_61d0, &args![this, 0.5f32, source, -0.5f32]);
        e.call(0x004f_61d0, &args![this, 0.5f32, 0u32, 0.5f32]);
        assert!(added.borrow().is_empty());
    }
}
