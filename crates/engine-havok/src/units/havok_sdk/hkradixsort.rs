//! `Havok SDK/hkradixsort.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d9fcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRadixSort::sort16` (Xbox PDB), cdecl `(data, count, scratch)`: a stable two-pass radix
/// sort of 32-bit words by their low 16 bits (the stack frame of 0x1004 bytes is the exe's
/// `__alloca_probe`). Two 256-entry histograms count the low byte and the second byte of every
/// word; prefix sums turn them into destination pointers (`scratch` for the first pass, `data`
/// for the second); the first pass moves the words of `data` to `scratch` ordered by the low
/// byte, the second moves them back ordered by the second byte. The loops are unrolled by four
/// and run `(count - 1) / 4 + 1` times when `count` is above 0 (signed), so they always handle
/// whole groups of four words: when `count` is not a multiple of 4 they read, count and move the
/// words past the end of `data` as well (up to three), and write as many past the sorted words
/// into `scratch` and then `data`. Nothing happens for a `count` of 0 or less.
pub fn hk_radix_sort_sort16(e: &mut Engine, data: Ptr, count: i32, scratch: Ptr) {
    let (data, scratch) = (data.addr(), scratch.addr());
    let mut low_counts = [0u32; 256];
    let mut high_counts = [0u32; 256];
    let groups = if count > 0 {
        ((count as u32 - 1) >> 2) + 1
    } else {
        0
    };
    for group in 0..groups {
        for word in 0..4u32 {
            let at = data.wrapping_add(16 * group + 4 * word);
            let low = e.mem.u8(at);
            low_counts[usize::from(low)] = low_counts[usize::from(low)].wrapping_add(1);
            let high = e.mem.u8(at + 1);
            high_counts[usize::from(high)] = high_counts[usize::from(high)].wrapping_add(1);
        }
    }
    // The destinations: 4 bytes for each word counted before.
    let mut low_next = [0u32; 256];
    let mut high_next = [0u32; 256];
    low_next[0] = scratch;
    high_next[0] = data;
    for index in 0..255 {
        low_next[index + 1] = low_next[index].wrapping_add(low_counts[index].wrapping_mul(4));
        high_next[index + 1] = high_next[index].wrapping_add(high_counts[index].wrapping_mul(4));
    }
    for group in 0..groups {
        for word in 0..4u32 {
            let at = data.wrapping_add(16 * group + 4 * word);
            let key = usize::from(e.mem.u8(at));
            let moved = e.mem.u32(at);
            let destination = low_next[key];
            e.mem.set_u32(destination, moved);
            low_next[key] = destination.wrapping_add(4);
        }
    }
    for group in 0..groups {
        for word in 0..4u32 {
            let at = scratch.wrapping_add(16 * group + 4 * word);
            let moved = e.mem.u32(at);
            let key = usize::from(e.mem.u8(at + 1));
            let destination = high_next[key];
            e.mem.set_u32(destination, moved);
            high_next[key] = destination.wrapping_add(4);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9fcf0, hk_radix_sort_sort16(Ptr, i32, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(address + 4 * i)).collect()
    }

    fn array(e: &mut Engine, values: &[u32]) -> u32 {
        let address = e.mem.alloc(4 * values.len() as u32);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(address + 4 * i as u32, *value);
        }
        address
    }

    #[test]
    fn a_multiple_of_four_words_is_sorted_stably_by_the_low_sixteen_bits() {
        let mut e = Engine::new();
        // keys 0x0102, 0x0001, 0x0102, 0x00ff, 0x0100, 0x0001, 0xffff, 0x0000; the upper bits tag
        // the words so that the stability shows
        let input = [
            0xa000_0102,
            0xb000_0001,
            0xc000_0102,
            0xd000_00ff,
            0xe000_0100,
            0xf000_0001,
            0x1000_ffff,
            0x2000_0000,
        ];
        let data = array(&mut e, &input);
        let scratch = array(&mut e, &[0xdead_beef; 8]);
        e.call(0x00d9_fcf0, &args![data, 8i32, scratch]);
        assert_eq!(
            words(&e, data, 8),
            vec![
                0x2000_0000,
                0xb000_0001,
                0xf000_0001,
                0xd000_00ff,
                0xe000_0100,
                0xa000_0102,
                0xc000_0102,
                0x1000_ffff
            ]
        );
    }

    #[test]
    fn a_count_that_is_not_a_multiple_of_four_sorts_the_words_past_the_end_too() {
        let mut e = Engine::new();
        // six words, then two more in the buffer that the loops count and move as well
        let input = [5u32, 4, 3, 2, 1, 6, 0, 0xffff];
        let data = array(&mut e, &input);
        let scratch = array(&mut e, &[0xdead_beef; 8]);
        e.call(0x00d9_fcf0, &args![data, 6i32, scratch]);
        // all eight are sorted: the 0 past the end comes first
        assert_eq!(words(&e, data, 8), vec![0, 1, 2, 3, 4, 5, 6, 0xffff]);
        // a word past the end that sorts last leaves the first six sorted
        let data = array(&mut e, &[5u32, 4, 3, 2, 1, 6, 0xffff, 0xffff]);
        e.call(0x00d9_fcf0, &args![data, 6i32, scratch]);
        assert_eq!(words(&e, data, 6), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn the_scratch_holds_the_first_pass_and_nothing_happens_for_no_words() {
        let mut e = Engine::new();
        // the first pass orders by the low byte only
        let data = array(&mut e, &[0x0201, 0x0102, 0x0300, 0x0004]);
        let scratch = array(&mut e, &[0; 4]);
        e.call(0x00d9_fcf0, &args![data, 4i32, scratch]);
        assert_eq!(words(&e, scratch, 4), vec![0x0300, 0x0201, 0x0102, 0x0004]);
        assert_eq!(words(&e, data, 4), vec![0x0004, 0x0102, 0x0201, 0x0300]);
        // zero and negative counts touch nothing (no memory is even read)
        let unmapped = 0x0c00_0000u32;
        e.call(0x00d9_fcf0, &args![unmapped, 0i32, unmapped]);
        e.call(0x00d9_fcf0, &args![unmapped, -3i32, unmapped]);
    }
}
