//! `Havok SDK/run_00e0ab60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e0ac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_9bd0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e0ac80(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_9bd0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The address of record `index` (`0x14` bytes each; the index is signed and the address
/// arithmetic wraps) of the array at `array`.
fn record_address(array: u32, index: i32) -> u32 {
    array.wrapping_add((index as u32).wrapping_mul(0x14))
}

/// Exchanges the five words of two records.
fn swap_records(e: &mut Engine, first: u32, second: u32) {
    let first_words: [u32; 5] = std::array::from_fn(|word| e.mem.u32(first + 4 * word as u32));
    let second_words: [u32; 5] = std::array::from_fn(|word| e.mem.u32(second + 4 * word as u32));
    for (word, (from_first, from_second)) in first_words.into_iter().zip(second_words).enumerate() {
        let at = 4 * word as u32;
        e.mem.set_u32(first + at, from_second);
        e.mem.set_u32(second + at, from_first);
    }
}

// Translated from 00e0ab60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a recursive quicksort over `0x14`-byte records, ascending by the
/// float key at `+8`, for the index range `lo..=hi` of the array at `array` (the fourth stack
/// word `extra` is only passed on to the recursive calls). The pivot is the key of the middle
/// record (`(lo + hi) >> 1`, signed) loaded onto the x87 stack once per pass; the scans compare
/// with `FCOMP`: the left scan goes on while the key is below the pivot, the right scan while
/// the pivot is below the key (a NaN stops both). The two scans' records are exchanged whole
/// when they differ; then the left part is sorted by a recursive call when `lo < j`, and the
/// loop goes on with `lo = i` while `i < hi`. The function takes four stack words and pops none
/// (`RET`) and returns nothing.
pub fn fn_00e0ab60(e: &mut Engine, array: u32, lo: i32, hi: i32, extra: u32) {
    let mut lo = lo;
    loop {
        let middle = lo.wrapping_add(hi) >> 1;
        let pivot = x87_load32(e.mem.u32(record_address(array, middle) + 8));
        let mut i = lo;
        let mut j = hi;
        loop {
            while x87_load32(e.mem.u32(record_address(array, i) + 8)) < pivot {
                i = i.wrapping_add(1);
            }
            while pivot < x87_load32(e.mem.u32(record_address(array, j) + 8)) {
                j = j.wrapping_sub(1);
            }
            if j < i {
                break;
            }
            if j != i {
                swap_records(e, record_address(array, j), record_address(array, i));
            }
            j = j.wrapping_sub(1);
            i = i.wrapping_add(1);
            if i > j {
                break;
            }
        }
        if lo < j {
            fn_00e0ab60(e, array, lo, j, extra);
        }
        lo = i;
        if i >= hi {
            return;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_ac80, deleting_destructor_00e0ac80(Ptr, u8) -> Ptr),
        entry!(0x00e0_ab60, fn_00e0ab60(u32, i32, i32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00e0ac80_frees_only_when_bit_zero_is_set() {
        check(0x00e0_ac80, 0x00e0_9bd0);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// Writes records `(id, key)` as five words: id at +0, 0xaa at +4, the float key at +8, the
    /// id negated at +0xc and 0xbb at +0x10; returns the array.
    fn records(e: &mut Engine, keys: &[f32]) -> u32 {
        let array = e.mem.alloc(0x14 * keys.len() as u32 + 8);
        for (index, key) in keys.iter().enumerate() {
            let at = array + 0x14 * index as u32;
            e.mem.set_u32(at, index as u32);
            e.mem.set_u32(at + 4, 0xaa);
            e.mem.set_f32(at + 8, *key);
            e.mem.set_u32(at + 0xc, !(index as u32));
            e.mem.set_u32(at + 0x10, 0xbb);
        }
        array
    }

    /// The (id, key) pairs now in the array; every record must still be whole.
    fn contents(e: &Engine, array: u32, count: u32) -> Vec<(u32, f32)> {
        (0..count)
            .map(|index| {
                let at = array + 0x14 * index;
                let id = e.mem.u32(at);
                assert_eq!(e.mem.u32(at + 4), 0xaa);
                assert_eq!(e.mem.u32(at + 0xc), !id);
                assert_eq!(e.mem.u32(at + 0x10), 0xbb);
                (id, e.mem.f32(at + 8))
            })
            .collect()
    }

    fn sorted_keys(keys: &[f32]) -> Vec<f32> {
        let mut e = Engine::new();
        let array = records(&mut e, keys);
        e.call(
            0x00e0_ab60,
            &args![array, 0i32, keys.len() as i32 - 1, 0x1234u32],
        );
        contents(&e, array, keys.len() as u32)
            .into_iter()
            .map(|(_, key)| key)
            .collect()
    }

    /// A shuffled list comes out ascending with its records whole.
    #[test]
    fn fn_00e0ab60_sorts_ascending() {
        assert_eq!(
            sorted_keys(&[5.0, -1.0, 3.5, 9.0, 0.0, 2.0, 7.0, -4.0, 6.0]),
            vec![-4.0, -1.0, 0.0, 2.0, 3.5, 5.0, 6.0, 7.0, 9.0]
        );
    }

    /// Equal keys, a sorted list and a reversed list.
    #[test]
    fn fn_00e0ab60_handles_duplicates_and_ordered_input() {
        assert_eq!(
            sorted_keys(&[2.0, 1.0, 2.0, 1.0, 2.0, 1.0]),
            vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0]
        );
        assert_eq!(sorted_keys(&[1.0, 2.0, 3.0, 4.0]), vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(sorted_keys(&[4.0, 3.0, 2.0, 1.0]), vec![1.0, 2.0, 3.0, 4.0]);
    }

    /// A range of one record, and a sub-range that leaves the rest alone.
    #[test]
    fn fn_00e0ab60_sorts_only_its_range() {
        let mut e = Engine::new();
        let keys = [9.0, 8.0, 7.0, 6.0, 5.0, 4.0];
        let array = records(&mut e, &keys);
        e.call(0x00e0_ab60, &args![array, 2i32, 2i32, 0u32]);
        assert_eq!(
            contents(&e, array, 6)
                .iter()
                .map(|r| r.1)
                .collect::<Vec<_>>(),
            keys.to_vec()
        );
        e.call(0x00e0_ab60, &args![array, 1i32, 4i32, 0u32]);
        assert_eq!(
            contents(&e, array, 6)
                .iter()
                .map(|r| r.1)
                .collect::<Vec<_>>(),
            vec![9.0, 5.0, 6.0, 7.0, 8.0, 4.0]
        );
    }

    /// Whole records move with their keys.
    #[test]
    fn fn_00e0ab60_moves_whole_records() {
        let mut e = Engine::new();
        let array = records(&mut e, &[3.0, 1.0, 2.0]);
        e.call(0x00e0_ab60, &args![array, 0i32, 2i32, 0u32]);
        assert_eq!(contents(&e, array, 3), vec![(1, 1.0), (2, 2.0), (0, 3.0)]);
    }

    /// A NaN key stops both scans; the sort still ends and keeps every record.
    #[test]
    fn fn_00e0ab60_terminates_with_a_nan_key() {
        let mut e = Engine::new();
        let array = records(&mut e, &[2.0, f32::NAN, 1.0, 3.0]);
        e.call(0x00e0_ab60, &args![array, 0i32, 3i32, 0u32]);
        let mut ids: Vec<u32> = contents(&e, array, 4).iter().map(|r| r.0).collect();
        ids.sort_unstable();
        assert_eq!(ids, vec![0, 1, 2, 3]);
    }
}
