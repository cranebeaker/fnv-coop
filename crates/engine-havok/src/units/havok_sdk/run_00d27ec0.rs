//! `Havok SDK/run_00d27ec0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00d06c30` (cdecl `(array, count, elementSize)`): grows a small array (16-bit size and
/// capacity) to `count` elements.
const SMALL_ARRAY_GROW: u32 = 0x00d0_6c30;
/// `00d2cb70` (cdecl `(destination, source, bytes)`): the block copy of `hkMemUtil`.
const BLOCK_COPY: u32 = 0x00d2_cb70;

// Translated from 00d27ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts `count` elements of 0x30 bytes copied from `source` at `index` of a small array
/// (data at +0, 16-bit size at +4, 16-bit capacity and flags at +6). When the new size
/// exceeds the capacity (low 14 bits) the array is grown to the larger of the new size and
/// twice the capacity with `00d06c30`; the tail is moved up and the new elements copied in
/// with `00d2cb70`. The class is unconfirmed.
pub fn fn_00d27ec0(e: &mut Engine, this: Ptr, index: u32, source: Ptr, count: u32) {
    let size = u32::from(e.mem.u16(this.addr() + 4));
    let new_size = size.wrapping_add(count);
    let tail = size.wrapping_sub(index);
    let capacity = u32::from(e.mem.u16(this.addr() + 6) & 0x3fff);
    if (capacity as i32) < (new_size as i32) {
        let doubled = capacity.wrapping_add(capacity);
        let target = if (new_size as i32) < (doubled as i32) {
            doubled
        } else {
            new_size
        };
        e.call(SMALL_ARRAY_GROW, &args![this, target, 0x30u32]);
    }
    let data = e.mem.u32(this.addr());
    let offset = index.wrapping_mul(0x30);
    let moved_to = data.wrapping_add(index.wrapping_add(count).wrapping_mul(0x30));
    e.call(
        BLOCK_COPY,
        &args![moved_to, data.wrapping_add(offset), tail.wrapping_mul(0x30)],
    );
    let data = e.mem.u32(this.addr());
    e.call(
        BLOCK_COPY,
        &args![data.wrapping_add(offset), source, count.wrapping_mul(0x30)],
    );
    e.mem.set_u16(this.addr() + 4, new_size as u16);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d2_7ec0, fn_00d27ec0(Ptr, u32, Ptr, u32))]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A small array of `size` records of 0x30 bytes (record `i` filled with byte `i + 1`)
    /// with the given capacity, a growing double and a copying double.
    fn rig(size: u16, capacity: u16) -> (Engine, u32, Log) {
        let mut e = test_engine();
        let array = e.mem.alloc(8);
        let data = e.mem.alloc(u32::from(capacity) * 0x30);
        for i in 0..u32::from(size) {
            e.mem.write(data + i * 0x30, &[i as u8 + 1; 0x30]);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u16(array + 4, size);
        e.mem.set_u16(array + 6, 0x8000 | capacity);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let grows = Rc::clone(&log);
        e.register_double(SMALL_ARRAY_GROW, move |e, a| {
            grows.borrow_mut().push((SMALL_ARRAY_GROW, a.to_vec()));
            let old = e.mem.u32(a[0]);
            let used = u32::from(e.mem.u16(a[0] + 4)) * 0x30;
            let block = e.mem.alloc(a[1] * 0x30);
            let bytes = e.mem.bytes(old, used);
            e.mem.write(block, &bytes);
            e.mem.set_u32(a[0], block);
            e.mem.set_u16(a[0] + 6, a[1] as u16);
            Ret::default()
        });
        let copies = Rc::clone(&log);
        e.register_double(BLOCK_COPY, move |e, a| {
            copies.borrow_mut().push((BLOCK_COPY, a.to_vec()));
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        (e, array, log)
    }

    fn record_bytes(e: &Engine, array: u32) -> Vec<u8> {
        let data = e.mem.u32(array);
        (0..u32::from(e.mem.u16(array + 4)))
            .map(|i| e.mem.u8(data + i * 0x30))
            .collect()
    }

    #[test]
    fn inserts_in_the_middle_without_growing() {
        let (mut e, array, log) = rig(3, 8);
        let source = e.mem.alloc(0x30);
        e.mem.write(source, &[9; 0x30]);
        e.call(0x00d2_7ec0, &args![array, 1u32, source, 1u32]);
        assert_eq!(record_bytes(&e, array), vec![1, 9, 2, 3]);
        assert!(!log.borrow().iter().any(|(a, _)| *a == SMALL_ARRAY_GROW));
    }

    #[test]
    fn grows_to_the_larger_of_the_new_size_and_twice_the_capacity() {
        let (mut e, array, log) = rig(2, 2);
        let source = e.mem.alloc(0x30);
        e.mem.write(source, &[9; 0x30]);
        e.call(0x00d2_7ec0, &args![array, 2u32, source, 1u32]);
        assert_eq!(record_bytes(&e, array), vec![1, 2, 9]);
        // Twice the capacity (4) beats the new size (3).
        assert_eq!(log.borrow()[0], (SMALL_ARRAY_GROW, vec![array, 4, 0x30]));
        let (mut e, array, log) = rig(1, 1);
        let source = e.mem.alloc(0x90);
        e.mem.write(source, &[7; 0x90]);
        e.call(0x00d2_7ec0, &args![array, 0u32, source, 3u32]);
        assert_eq!(record_bytes(&e, array), vec![7, 7, 7, 1]);
        // The new size (4) beats twice the capacity (2).
        assert_eq!(log.borrow()[0], (SMALL_ARRAY_GROW, vec![array, 4, 0x30]));
    }
}
