//! `Havok SDK/run_00cde880` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl: allocator, array, capacity, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkMemUtil::memMove` (`00d2cc70`, cdecl: destination, source, byte count).
const MEM_MOVE: u32 = 0x00d2_cc70;
/// The mask of the capacity bits of an `hkArray`'s capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00cde880 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `hkArray` of 16-bit words (`this`: data `+0`, size `+4`, capacity word `+8`) replaces
/// `remove` words at `index` with `insert` words read from `source`: the new size is `size -
/// remove + insert`; when the capacity is below it the array grows to the larger of twice the
/// capacity and the new size through `hkArrayUtil::_reserve(allocator, this, capacity, 2)`;
/// the tail after the removed words (`(size - index - remove) * 2` bytes) is moved with
/// `memMove` to follow the inserted ones, the new words are copied one by one (nothing for
/// `insert <= 0`), and the size is stored. Stack words: `(allocator, index, remove, source,
/// insert)`; the function is the exe's `00cde880` (the class is unconfirmed).
pub fn fn_00cde880(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    index: i32,
    remove: i32,
    source: u32,
    insert: i32,
) {
    let base = this.addr();
    let size = e.mem.i32(base + 4);
    let tail = size.wrapping_sub(index).wrapping_sub(remove);
    let new_size = size.wrapping_sub(remove).wrapping_add(insert);
    let capacity = (e.mem.u32(base + 8) & CAPACITY_MASK) as i32;
    if capacity < new_size {
        let mut wanted = capacity.wrapping_add(capacity);
        if new_size >= wanted {
            wanted = new_size;
        }
        e.call(ARRAY_RESERVE, &args![allocator, this, wanted, 2u32]);
    }
    let data = e.mem.u32(base);
    let after_insert = data.wrapping_add(index.wrapping_add(insert).wrapping_mul(2) as u32);
    let after_remove = data.wrapping_add(index.wrapping_add(remove).wrapping_mul(2) as u32);
    e.call(
        MEM_MOVE,
        &args![after_insert, after_remove, tail.wrapping_mul(2)],
    );
    let mut destination = e.mem.u32(base).wrapping_add((index as u32).wrapping_mul(2));
    let mut from = source;
    let mut remaining = insert;
    if remaining > 0 {
        loop {
            let word = e.mem.u16(from);
            e.mem.set_u16(destination, word);
            destination = destination.wrapping_add(2);
            from = from.wrapping_add(2);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    e.mem.set_i32(base + 4, new_size);
}

// Translated from 00cde920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts `count` words from `source` at `index` into the array `this`: the function above
/// with nothing removed (`00cde880(allocator, index, 0, source, count)`). Stack words:
/// `(allocator, index, source, count)`.
pub fn fn_00cde920(e: &mut Engine, this: Ptr, allocator: u32, index: i32, source: u32, count: i32) {
    fn_00cde880(e, this, allocator, index, 0, source, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_e880, fn_00cde880(Ptr, u32, i32, i32, u32, i32)),
        entry!(0x00cd_e920, fn_00cde920(Ptr, u32, i32, u32, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A word array with the given words, capacity `capacity`, and doubles for the two
    /// helpers: `_reserve` moves the data to a block of the new capacity, `memMove` moves
    /// bytes (overlap-safe).
    fn rig(words: &[u16], capacity: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register_double(ARRAY_RESERVE, |e, a| {
            let (array, capacity) = (a[1], a[2]);
            let new = e.mem.alloc(capacity * 2);
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            let bytes = e.mem.bytes(old, size * 2);
            e.mem.write(new, &bytes);
            e.mem.set_u32(array, new);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        e.register_double(MEM_MOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(capacity * 2);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u16(data + 2 * i as u32, *word);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, words.len() as u32);
        e.mem.set_u32(array + 8, capacity);
        (e, array)
    }

    fn contents(e: &Engine, array: u32) -> Vec<u16> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u16(data + 2 * i))
            .collect()
    }

    fn source(e: &mut Engine, words: &[u16]) -> u32 {
        let at = e.mem.alloc(2 * words.len().max(1) as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u16(at + 2 * i as u32, *word);
        }
        at
    }

    /// Replacing words in the middle shifts the tail and copies the new words; a capacity
    /// that is large enough is not touched.
    #[test]
    fn splice_replaces_words_in_place_when_there_is_room() {
        let (mut e, array) = rig(&[1, 2, 3, 4, 5], 8);
        let new = source(&mut e, &[10, 11, 12]);
        e.call(0x00cd_e880, &args![array, 0x77u32, 1i32, 2i32, new, 3i32]);
        assert_eq!(contents(&e, array), vec![1, 10, 11, 12, 4, 5]);
        assert_eq!(e.mem.u32(array + 8), 8);
        // Removing without inserting closes the gap.
        e.call(0x00cd_e880, &args![array, 0x77u32, 0i32, 2i32, new, 0i32]);
        assert_eq!(contents(&e, array), vec![11, 12, 4, 5]);
    }

    /// Without room the array is reserved at the larger of twice its capacity and the new
    /// size, through `_reserve` with the allocator and element size 2.
    #[test]
    fn splice_reserves_the_larger_of_double_and_needed() {
        let (mut e, array) = rig(&[1, 2, 3, 4], 4);
        let new = source(&mut e, &[9, 9, 9]);
        e.call_log = Some(vec![]);
        e.call(0x00cd_e880, &args![array, 0x77u32, 4i32, 0i32, new, 3i32]);
        assert_eq!(contents(&e, array), vec![1, 2, 3, 4, 9, 9, 9]);
        let log = e.call_log.take().unwrap();
        let reserve: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(at, _)| *at == ARRAY_RESERVE)
            .map(|(_, a)| a)
            .collect();
        assert_eq!(*reserve[0], vec![0x77, array, 8, 2]);
        // Needing more than double: exactly what is needed.
        let (mut e, array) = rig(&[1], 1);
        let new = source(&mut e, &[5, 6, 7, 8]);
        e.call_log = Some(vec![]);
        e.call(0x00cd_e880, &args![array, 0x77u32, 1i32, 0i32, new, 4i32]);
        let log = e.call_log.take().unwrap();
        let request = log.iter().find(|(at, _)| *at == ARRAY_RESERVE).unwrap();
        assert_eq!(request.1[2], 5);
        assert_eq!(contents(&e, array), vec![1, 5, 6, 7, 8]);
    }

    /// The insertion wrapper removes nothing.
    #[test]
    fn insert_wrapper_splices_with_nothing_removed() {
        let (mut e, array) = rig(&[1, 2, 3], 8);
        let new = source(&mut e, &[7, 8]);
        e.call(0x00cd_e920, &args![array, 0x77u32, 1i32, new, 2i32]);
        assert_eq!(contents(&e, array), vec![1, 7, 8, 2, 3]);
    }
}
