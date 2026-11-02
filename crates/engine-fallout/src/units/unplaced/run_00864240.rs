//! `(unplaced)/run_00864240` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiTLargeArray;

/// `0096afc0`: `cdecl(count)`, allocates the storage of `count` pointers.
const ARRAY_ALLOCATE: u32 = 0x0096_afc0;
/// `004ede70`: `cdecl(storage)`, frees storage `ARRAY_ALLOCATE` made.
const ARRAY_FREE: u32 = 0x004e_de70;
/// `0061bd20`: the same array's `SetAt` (`thiscall(index, element pointer)`).
const ARRAY_SET_AT: u32 = 0x0061_bd20;

// Translated from 00864240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SetAtGrow` of an `NiTLargeArray` of plain
/// values (`thiscall`, two stack words, `RET 8`): when `index` is at or past
/// the capacity, grows the array to `index + m_uiGrowBy` ([`fn_00864280`]),
/// then stores the word `*element` at `index` (`0061bd20`). Returns `index`.
pub fn fn_00864240(e: &mut Engine, this: Ptr<NiTLargeArray>, index: u32, element: Ptr) -> u32 {
    if index >= e.get(this, NiTLargeArray::m_uiMaxSize) {
        let grow_by = e.get(this, NiTLargeArray::m_uiGrowBy);
        fn_00864280(e, this, index.wrapping_add(grow_by));
    }
    e.call(ARRAY_SET_AT, &args![this, index, element]);
    index
}

// Translated from 00864280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SetSize` of an `NiTLargeArray` of plain values (layout
/// [`NiTLargeArray`]: 32-bit capacity, used size and non-zero count; `thiscall`,
/// one stack word). Does nothing when `new_max` is the current capacity. When it
/// is below the used size, the elements from `new_max` on are set to zero (the
/// non-zero count goes down for each non-zero one) and the used size becomes
/// `new_max`. Then new storage of `new_max` elements (`0096afc0`) takes the used
/// elements and its other elements are zeroed, the old storage is freed
/// (`004ede70`), and the capacity is `new_max` (the pointer is null for 0).
/// No element is reference counted.
pub fn fn_00864280(e: &mut Engine, this: Ptr<NiTLargeArray>, new_max: u32) {
    if new_max == e.get(this, NiTLargeArray::m_uiMaxSize) {
        return;
    }
    if new_max < e.get(this, NiTLargeArray::m_uiSize) {
        let mut index = new_max;
        while index < e.get(this, NiTLargeArray::m_uiSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            if e.mem.u32(slot) != 0 {
                e.mem.set_u32(slot, 0);
                let present = e.get(this, NiTLargeArray::m_uiESize);
                e.set(this, NiTLargeArray::m_uiESize, present.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.set(this, NiTLargeArray::m_uiSize, new_max);
    }
    let old = e.get(this, NiTLargeArray::m_pBase);
    e.set(this, NiTLargeArray::m_uiMaxSize, new_max);
    if new_max == 0 {
        e.set(this, NiTLargeArray::m_pBase, 0);
    } else {
        let max = e.get(this, NiTLargeArray::m_uiMaxSize);
        let storage = e.call(ARRAY_ALLOCATE, &args![max]).u32();
        e.set(this, NiTLargeArray::m_pBase, storage);
        let mut index = 0u32;
        while index < e.get(this, NiTLargeArray::m_uiSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let value = e.mem.u32(old.wrapping_add(index.wrapping_mul(4)));
            e.mem
                .set_u32(base.wrapping_add(index.wrapping_mul(4)), value);
            index = index.wrapping_add(1);
        }
        let mut index = e.get(this, NiTLargeArray::m_uiSize);
        while index < e.get(this, NiTLargeArray::m_uiMaxSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            e.mem.set_u32(base.wrapping_add(index.wrapping_mul(4)), 0);
            index = index.wrapping_add(1);
        }
    }
    e.call(ARRAY_FREE, &args![old]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086_4280, fn_00864280(Ptr<NiTLargeArray>, u32)),
        entry!(
            0x0086_4240,
            fn_00864240(Ptr<NiTLargeArray>, u32, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(max: u32, items: &[u32]) -> (Engine, Ptr<NiTLargeArray>, u32) {
        let mut e = Engine::new();
        e.register(ARRAY_ALLOCATE, |e, a| e.mem.alloc(a[0] * 4 + 4).into_ret());
        e.register(ARRAY_FREE, |_, _| Ret::default());
        let array = e.new_object::<NiTLargeArray>();
        let base = e.mem.alloc(max * 4 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(array, NiTLargeArray::m_pBase, base);
        e.set(array, NiTLargeArray::m_uiMaxSize, max);
        e.set(array, NiTLargeArray::m_uiSize, items.len() as u32);
        let present = items.iter().filter(|item| **item != 0).count();
        e.set(array, NiTLargeArray::m_uiESize, present as u32);
        (e, array, base)
    }

    fn words(e: &Engine, array: Ptr<NiTLargeArray>, count: u32) -> Vec<u32> {
        let base = e.get(array, NiTLargeArray::m_pBase);
        (0..count).map(|i| e.mem.u32(base + i * 4)).collect()
    }

    #[test]
    fn the_current_capacity_changes_nothing() {
        let (mut e, array, base) = fixture(3, &[5]);
        e.call_log = Some(vec![]);
        e.call(0x0086_4280, &args![array, 3u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.get(array, NiTLargeArray::m_pBase), base);
    }

    #[test]
    fn growing_copies_the_used_elements_and_zeroes_the_rest() {
        let (mut e, array, base) = fixture(3, &[7, 0, 9]);
        e.call_log = Some(vec![]);
        e.call(0x0086_4280, &args![array, 5u32]);
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![5u32]);
        let frees: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(words(&e, array, 5), vec![7, 0, 9, 0, 0]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiMaxSize), 5);
        assert_eq!(e.get(array, NiTLargeArray::m_uiSize), 3);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 2);
    }

    #[test]
    fn shrinking_below_the_used_size_zeroes_the_cut_elements() {
        let (mut e, array, base) = fixture(4, &[7, 8, 0, 9]);
        e.call(0x0086_4280, &args![array, 2u32]);
        assert_eq!(words(&e, array, 2), vec![7, 8]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiMaxSize), 2);
        assert_eq!(e.get(array, NiTLargeArray::m_uiSize), 2);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 2);
        assert_eq!(e.mem.u32(base + 12), 0);
    }

    #[test]
    fn capacity_zero_leaves_a_null_pointer() {
        let (mut e, array, base) = fixture(2, &[7]);
        e.call_log = Some(vec![]);
        e.call(0x0086_4280, &args![array, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != ARRAY_ALLOCATE));
        let frees: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_FREE).collect();
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(array, NiTLargeArray::m_pBase), 0);
        assert_eq!(e.get(array, NiTLargeArray::m_uiSize), 0);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 0);
    }

    #[test]
    fn set_at_grow_grows_by_the_step_only_past_the_capacity() {
        let (mut e, array, base) = fixture(2, &[7]);
        e.set(array, NiTLargeArray::m_uiGrowBy, 3);
        e.register(ARRAY_SET_AT, |_, _| Ret::default());
        let element = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        // Inside the capacity: no growth, the store gets the same words.
        let back = e.call(0x0086_4240, &args![array, 1u32, element]).u32();
        assert_eq!(back, 1);
        assert_eq!(e.get(array, NiTLargeArray::m_pBase), base);
        // At the capacity: grows to index + grow-by before the store.
        let back = e.call(0x0086_4240, &args![array, 2u32, element]).u32();
        assert_eq!(back, 2);
        let log = e.call_log.take().unwrap();
        let stores: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_SET_AT).collect();
        assert_eq!(stores.len(), 2);
        assert_eq!(stores[0].1, args![array, 1u32, element]);
        assert_eq!(stores[1].1, args![array, 2u32, element]);
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![5u32]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiMaxSize), 5);
    }
}
