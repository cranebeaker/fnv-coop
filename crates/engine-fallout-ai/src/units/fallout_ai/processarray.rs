//! `fallout/ai/processarray.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ProcessArray` (0x40 bytes) is an array of object pointers (the Xbox PDB's
//! `AIProcessArray`, an `NiTPrimitiveArray<MobileObject *>` in the first 0x10
//! bytes) divided into four consecutive process levels. Level `n` owns the
//! indexes from `ProcessLevelHeads[n]` up to (not including)
//! `ProcessLevelTails[n]`; `ProcessLevelNextUpdate[n]` is a cursor inside that
//! range. The three tables of four words sit at `+0x10`, `+0x20` and `+0x30`.
//! Callees outside the file: the array's element address (`00877a30`), its
//! element count (`00658930`), its element setter (`0096ae90`) and its resize
//! (`0096ad30`).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ProcessArray` (Xbox PDB), 0x40 bytes. The three tables of four words
    /// are addressed by level (`heads`, `tails`, `next_update`).
    pub struct ProcessArray: 0x40 {
    }
}

/// `ProcessLevelHeads` (Xbox PDB): first index of each level.
const HEADS: u32 = 0x10;
/// `ProcessLevelTails` (Xbox PDB): one past the last index of each level.
const TAILS: u32 = 0x20;
/// `ProcessLevelNextUpdate` (Xbox PDB): per-level update cursor.
const NEXT_UPDATE: u32 = 0x30;

/// The array's element address: `(this, index)`.
const ELEMENT_ADDRESS: u32 = 0x0087_7a30;
/// The array's element count (`this`).
const ELEMENT_COUNT: u32 = 0x0065_8930;
/// Sets the element at an index from the word at an address:
/// `(this, index, pointer to the value)`.
const SET_ELEMENT: u32 = 0x0096_ae90;
/// Resizes the array to a number of elements: `(this, count)`.
const RESIZE: u32 = 0x0096_ad30;

/// The address of table word `level` (table at `base`); `level` is a signed
/// index and the address wraps like the exe's `[this + level*4 + base]`.
fn slot(this: Ptr<ProcessArray>, base: u32, level: i32) -> u32 {
    this.addr()
        .wrapping_add(base)
        .wrapping_add((level as u32).wrapping_mul(4))
}

fn head(e: &Engine, this: Ptr<ProcessArray>, level: i32) -> u32 {
    e.mem.u32(slot(this, HEADS, level))
}

fn tail(e: &Engine, this: Ptr<ProcessArray>, level: i32) -> u32 {
    e.mem.u32(slot(this, TAILS, level))
}

// Translated from 0096a970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ProcessArray::AddActor` (Xbox PDB): appends `actor` at the end of process
/// level `level` (`-1` adds nothing). When the level below would then run
/// into the next level's start, the first element of the next level is first
/// moved to that level's end (recursively) so that the next level starts one
/// later. The array grows by 100 elements when the level's end reaches the
/// element count. The update cursors are corrected at the end.
pub fn process_array_add_actor(e: &mut Engine, this: Ptr<ProcessArray>, actor: u32, level: i32) {
    if level != -1 {
        let next = level.wrapping_add(1);
        if level < 3 {
            let end_plus_one = tail(e, this, level).wrapping_add(1);
            let next_head = head(e, this, next);
            if end_plus_one > next_head {
                let element = e.call(ELEMENT_ADDRESS, &args![this, next_head]).u32();
                let moved = e.mem.u32(element);
                process_array_add_actor(e, this, moved, next);
                let new_head = tail(e, this, level).wrapping_add(1);
                e.mem.set_u32(slot(this, HEADS, next), new_head);
            }
        }
        let end = tail(e, this, level);
        e.with_stack(4, |e, value| {
            e.mem.set_u32(value.addr(), actor);
            e.call(SET_ELEMENT, &args![this, end, value]);
        });
        let end = tail(e, this, level).wrapping_add(1);
        e.mem.set_u32(slot(this, TAILS, level), end);
        let count = e.call(ELEMENT_COUNT, &args![this]).u32();
        if tail(e, this, level) >= count {
            fn_0096aca0(e, this, 100);
        }
    }
    fn_0096ac30(e, this);
}

// Translated from 0096aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the element at `index` (unnamed in the Xbox PDB): finds its level
/// (`fn_0096ab20`; nothing happens for no level), shortens the level by one
/// when it is not empty, moves the level's last element into `index` unless
/// `index` is that last slot, clears the last slot and corrects the cursors.
pub fn fn_0096aa40(e: &mut Engine, this: Ptr<ProcessArray>, index: u32) {
    let level = fn_0096ab20(e, this, index);
    if level == -1 {
        return;
    }
    if tail(e, this, level) > head(e, this, level) {
        let shortened = tail(e, this, level).wrapping_sub(1);
        e.mem.set_u32(slot(this, TAILS, level), shortened);
    }
    if index != tail(e, this, level) {
        let last = tail(e, this, level);
        let element = e.call(ELEMENT_ADDRESS, &args![this, last]).u32();
        e.call(SET_ELEMENT, &args![this, index, element]);
    }
    let last = tail(e, this, level);
    e.with_stack(4, |e, zero| {
        e.call(SET_ELEMENT, &args![this, last, zero]);
    });
    fn_0096ac30(e, this);
}

// Translated from 0096aaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ProcessArray::RemoveActor` (Xbox PDB): finds `actor` from the start of
/// process level `level` and removes the element found.
pub fn process_array_remove_actor(e: &mut Engine, this: Ptr<ProcessArray>, actor: u32, level: i32) {
    let index = fn_0096abc0(e, this, actor, level);
    fn_0096aa40(e, this, index);
}

// Translated from 0096ab20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process level that owns `index` (unnamed in the Xbox PDB): the first
/// level `n` in `0..4` with `heads[n] <= index < tails[n]`, else `-1`.
pub fn fn_0096ab20(e: &mut Engine, this: Ptr<ProcessArray>, index: u32) -> i32 {
    for level in 0..4 {
        if index >= head(e, this, level) && index < tail(e, this, level) {
            return level;
        }
    }
    -1
}

// Translated from 0096abc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds an actor (unnamed in the Xbox PDB): the index of the first element
/// equal to `actor`, searching from the start of process level `level` to the
/// end of the last level; `-1` (`0xffffffff`) for a null actor or no match.
pub fn fn_0096abc0(e: &mut Engine, this: Ptr<ProcessArray>, actor: u32, level: i32) -> u32 {
    if actor != 0 {
        let end = e.mem.u32(this.addr() + 0x2c);
        let mut index = head(e, this, level);
        while index < end {
            let element = e.call(ELEMENT_ADDRESS, &args![this, index]).u32();
            if actor == e.mem.u32(element) {
                return index;
            }
            index += 1;
        }
    }
    u32::MAX
}

// Translated from 0096ac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Corrects the update cursors (unnamed in the Xbox PDB): a cursor outside
/// its level's range `heads[n] <= cursor < tails[n]` goes back to the level's
/// start.
pub fn fn_0096ac30(e: &mut Engine, this: Ptr<ProcessArray>) {
    for level in 0..4 {
        let cursor = e.mem.u32(slot(this, NEXT_UPDATE, level));
        if cursor < head(e, this, level) || cursor >= tail(e, this, level) {
            let start = head(e, this, level);
            e.mem.set_u32(slot(this, NEXT_UPDATE, level), start);
        }
    }
}

// Translated from 0096aca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Grows the array by `count` elements (unnamed in the Xbox PDB): resizes it
/// to the old count plus `count` and clears every new slot.
pub fn fn_0096aca0(e: &mut Engine, this: Ptr<ProcessArray>, count: i32) {
    let old_count = e.call(ELEMENT_COUNT, &args![this]).u32();
    e.call(RESIZE, &args![this, old_count.wrapping_add(count as u32)]);
    let mut index = old_count;
    while index < e.call(ELEMENT_COUNT, &args![this]).u32() {
        e.with_stack(4, |e, zero| {
            e.call(SET_ELEMENT, &args![this, index, zero]);
        });
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0096a970,
            process_array_add_actor(Ptr<ProcessArray>, u32, i32)
        ),
        entry!(0x0096aa40, fn_0096aa40(Ptr<ProcessArray>, u32)),
        entry!(
            0x0096aaf0,
            process_array_remove_actor(Ptr<ProcessArray>, u32, i32)
        ),
        entry!(0x0096ab20, fn_0096ab20(Ptr<ProcessArray>, u32) -> i32),
        entry!(0x0096abc0, fn_0096abc0(Ptr<ProcessArray>, u32, i32) -> u32),
        entry!(0x0096ac30, fn_0096ac30(Ptr<ProcessArray>)),
        entry!(0x0096aca0, fn_0096aca0(Ptr<ProcessArray>, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// A model of the element array: `ELEMENT_ADDRESS` gives the address of a
    /// slot in a flat buffer, `SET_ELEMENT` copies the word the pointer
    /// designates, `ELEMENT_COUNT` is the capacity, `RESIZE` raises it.
    struct Rig {
        e: Engine,
        array: Ptr<ProcessArray>,
        buffer: u32,
        capacity: Rc<Cell<u32>>,
        sets: Rc<RefCell<Vec<(u32, u32)>>>,
        resizes: Rc<RefCell<Vec<u32>>>,
    }

    fn rig(capacity: u32) -> Rig {
        let mut e = Engine::new();
        let buffer = e.mem.alloc(4 * 512);
        let capacity = Rc::new(Cell::new(capacity));
        let sets = Rc::new(RefCell::new(Vec::new()));
        let resizes = Rc::new(RefCell::new(Vec::new()));
        e.register_double(ELEMENT_ADDRESS, move |_, a| Ret {
            eax: buffer + 4 * a[1],
            ..Ret::default()
        });
        let c = capacity.clone();
        e.register_double(ELEMENT_COUNT, move |_, _| Ret {
            eax: c.get(),
            ..Ret::default()
        });
        let s = sets.clone();
        e.register_double(SET_ELEMENT, move |e, a| {
            let value = e.mem.u32(a[2]);
            s.borrow_mut().push((a[1], value));
            e.mem.set_u32(buffer + 4 * a[1], value);
            Ret::default()
        });
        let c = capacity.clone();
        let r = resizes.clone();
        e.register_double(RESIZE, move |_, a| {
            r.borrow_mut().push(a[1]);
            c.set(a[1]);
            Ret::default()
        });
        let array = e.new_object::<ProcessArray>();
        Rig {
            e,
            array,
            buffer,
            capacity,
            sets,
            resizes,
        }
    }

    impl Rig {
        fn set_table(&mut self, base: u32, values: [u32; 4]) {
            for (i, v) in values.iter().enumerate() {
                self.e
                    .mem
                    .set_u32(self.array.addr() + base + 4 * i as u32, *v);
            }
        }
        fn table(&self, base: u32) -> [u32; 4] {
            let mut out = [0; 4];
            for (i, v) in out.iter_mut().enumerate() {
                *v = self.e.mem.u32(self.array.addr() + base + 4 * i as u32);
            }
            out
        }
        fn element(&self, index: u32) -> u32 {
            self.e.mem.u32(self.buffer + 4 * index)
        }
        fn put(&mut self, index: u32, value: u32) {
            self.e.mem.set_u32(self.buffer + 4 * index, value);
        }
    }

    #[test]
    fn level_of_an_index_is_the_range_that_contains_it() {
        let mut r = rig(100);
        r.set_table(HEADS, [0, 2, 5, 5]);
        r.set_table(TAILS, [2, 5, 5, 8]);
        let a = r.array;
        assert_eq!(r.e.call(0x0096ab20, &args![a, 0u32]).i32(), 0);
        assert_eq!(r.e.call(0x0096ab20, &args![a, 1u32]).i32(), 0);
        assert_eq!(r.e.call(0x0096ab20, &args![a, 2u32]).i32(), 1);
        assert_eq!(r.e.call(0x0096ab20, &args![a, 4u32]).i32(), 1);
        assert_eq!(
            r.e.call(0x0096ab20, &args![a, 5u32]).i32(),
            3,
            "level 2 is empty"
        );
        assert_eq!(r.e.call(0x0096ab20, &args![a, 8u32]).i32(), -1);
        assert_eq!(r.e.call(0x0096ab20, &args![a, u32::MAX]).i32(), -1);
    }

    #[test]
    fn find_actor_searches_from_the_level_start_to_the_last_tail() {
        let mut r = rig(100);
        r.set_table(HEADS, [0, 2, 4, 6]);
        r.set_table(TAILS, [2, 4, 6, 8]);
        for (i, v) in [11u32, 12, 13, 14, 15, 16, 17, 18].iter().enumerate() {
            r.put(i as u32, *v);
        }
        let a = r.array;
        assert_eq!(r.e.call(0x0096abc0, &args![a, 13u32, 0u32]).u32(), 2);
        assert_eq!(
            r.e.call(0x0096abc0, &args![a, 12u32, 2u32]).u32(),
            u32::MAX,
            "level 2 starts after index 12"
        );
        assert_eq!(r.e.call(0x0096abc0, &args![a, 18u32, 3u32]).u32(), 7);
        assert_eq!(r.e.call(0x0096abc0, &args![a, 99u32, 0u32]).u32(), u32::MAX);
        assert_eq!(r.e.call(0x0096abc0, &args![a, 0u32, 0u32]).u32(), u32::MAX);
    }

    #[test]
    fn cursor_fix_resets_cursors_outside_their_range() {
        let mut r = rig(100);
        r.set_table(HEADS, [0, 2, 4, 6]);
        r.set_table(TAILS, [2, 4, 6, 8]);
        r.set_table(NEXT_UPDATE, [1, 4, 3, 8]);
        let a = r.array;
        r.e.call(0x0096ac30, &args![a]);
        assert_eq!(
            r.table(NEXT_UPDATE),
            [1, 2, 4, 6],
            "inside stays; at the tail, below the head and at the tail reset"
        );
    }

    #[test]
    fn grow_resizes_then_clears_the_new_slots() {
        let mut r = rig(3);
        for i in 0..8 {
            r.put(i, 0xaa);
        }
        let a = r.array;
        r.e.call(0x0096aca0, &args![a, 4u32]);
        assert_eq!(*r.resizes.borrow(), vec![7]);
        assert_eq!(r.capacity.get(), 7);
        assert_eq!(*r.sets.borrow(), vec![(3, 0), (4, 0), (5, 0), (6, 0)]);
        assert_eq!(r.element(2), 0xaa);
        assert_eq!(r.element(3), 0);
    }

    #[test]
    fn remove_at_moves_the_last_element_into_the_hole() {
        let mut r = rig(100);
        r.set_table(HEADS, [0, 3, 3, 3]);
        r.set_table(TAILS, [3, 3, 3, 3]);
        r.set_table(NEXT_UPDATE, [2, 3, 3, 3]);
        for (i, v) in [21u32, 22, 23].iter().enumerate() {
            r.put(i as u32, *v);
        }
        let a = r.array;
        r.e.call(0x0096aa40, &args![a, 0u32]);
        assert_eq!(r.table(TAILS), [2, 3, 3, 3]);
        assert_eq!(r.element(0), 23, "the last element fills index 0");
        assert_eq!(r.element(2), 0, "the old last slot is cleared");
        assert_eq!(r.table(NEXT_UPDATE)[0], 0, "cursor 2 left the range 0..2");
    }

    #[test]
    fn remove_the_last_slot_only_clears_it() {
        let mut r = rig(100);
        r.set_table(HEADS, [0, 3, 3, 3]);
        r.set_table(TAILS, [3, 3, 3, 3]);
        for (i, v) in [21u32, 22, 23].iter().enumerate() {
            r.put(i as u32, *v);
        }
        let a = r.array;
        r.e.call(0x0096aa40, &args![a, 2u32]);
        assert_eq!(*r.sets.borrow(), vec![(2, 0)], "no copy, one clear");
        assert_eq!(r.table(TAILS)[0], 2);
    }

    #[test]
    fn remove_of_an_index_outside_every_level_does_nothing() {
        let mut r = rig(100);
        r.set_table(HEADS, [0, 1, 1, 1]);
        r.set_table(TAILS, [1, 1, 1, 1]);
        let a = r.array;
        r.e.call(0x0096aa40, &args![a, 40u32]);
        assert!(r.sets.borrow().is_empty());
        assert_eq!(r.table(TAILS), [1, 1, 1, 1]);
    }

    #[test]
    fn remove_actor_finds_then_removes() {
        let mut r = rig(100);
        r.set_table(HEADS, [0, 3, 3, 3]);
        r.set_table(TAILS, [3, 3, 3, 3]);
        for (i, v) in [21u32, 22, 23].iter().enumerate() {
            r.put(i as u32, *v);
        }
        let a = r.array;
        r.e.call(0x0096aaf0, &args![a, 22u32, 0u32]);
        assert_eq!(r.element(1), 23);
        assert_eq!(r.element(2), 0);
        assert_eq!(r.table(TAILS)[0], 2);
        // An actor that is not there changes nothing.
        r.sets.borrow_mut().clear();
        r.e.call(0x0096aaf0, &args![a, 77u32, 0u32]);
        assert!(r.sets.borrow().is_empty());
    }

    #[test]
    fn add_actor_to_the_last_level_appends() {
        let mut r = rig(100);
        r.set_table(HEADS, [0, 0, 0, 0]);
        r.set_table(TAILS, [0, 0, 0, 0]);
        let a = r.array;
        r.e.call(0x0096a970, &args![a, 31u32, 3i32]);
        assert_eq!(r.element(0), 31);
        assert_eq!(r.table(TAILS), [0, 0, 0, 1]);
        assert!(r.resizes.borrow().is_empty(), "capacity 100 is not reached");
    }

    #[test]
    fn add_actor_with_minus_one_only_fixes_cursors() {
        let mut r = rig(100);
        r.set_table(HEADS, [2, 2, 2, 2]);
        r.set_table(TAILS, [4, 4, 4, 4]);
        let a = r.array;
        r.e.call(0x0096a970, &args![a, 31u32, -1i32]);
        assert!(r.sets.borrow().is_empty());
        assert_eq!(r.table(NEXT_UPDATE), [2, 2, 2, 2]);
    }

    #[test]
    fn add_actor_pushes_the_next_levels_first_element_to_its_end() {
        let mut r = rig(100);
        // Level 0: [0,1) holding 41; level 1: [1,3) holding 51, 52.
        r.set_table(HEADS, [0, 1, 3, 3]);
        r.set_table(TAILS, [1, 3, 3, 3]);
        r.put(0, 41);
        r.put(1, 51);
        r.put(2, 52);
        let a = r.array;
        r.e.call(0x0096a970, &args![a, 42u32, 0i32]);
        // 51 moved to the end of level 1 (index 3); 42 took index 1.
        assert_eq!(r.element(3), 51);
        assert_eq!(r.element(1), 42);
        assert_eq!(r.table(HEADS), [0, 2, 4, 4]);
        assert_eq!(r.table(TAILS), [2, 4, 4, 4]);
    }

    #[test]
    fn add_actor_grows_the_array_when_the_end_reaches_the_count() {
        let mut r = rig(2);
        r.set_table(HEADS, [0, 0, 0, 0]);
        r.set_table(TAILS, [0, 0, 0, 1]);
        let a = r.array;
        r.e.call(0x0096a970, &args![a, 31u32, 3i32]);
        assert_eq!(r.table(TAILS)[3], 2);
        assert_eq!(*r.resizes.borrow(), vec![102], "2 + 100 elements");
    }
}
