//! `(unplaced)/run_00c852c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The size of one element.
const ELEMENT_SIZE: u32 = 0x60;
/// The offset of an element's float key.
const KEY: u32 = 0x10;

/// The address of element `index` (signed index, wrapping arithmetic).
fn element(base: u32, index: i32) -> u32 {
    (index as u32).wrapping_mul(ELEMENT_SIZE).wrapping_add(base)
}

/// Swaps elements `j` (`a`) and `i` (`b`) the way the code does: the words
/// at `+0` to `+0x40` and `+0x50` are exchanged (the keys at `+0x10`
/// through the FPU); the words at `+0x44`, `+0x48`, `+0x4c` and `+0x54` to
/// `+0x5c` stay where they are.
fn swap(e: &mut Engine, a: u32, b: u32) {
    let a_key = x87_quiet_bits(e.mem.u32(a + KEY));
    let a_words: Vec<u32> = (0..0x11u32)
        .filter(|w| *w != 4)
        .map(|w| e.mem.u32(a + 4 * w))
        .collect();
    let a_tail = e.mem.u32(a + 0x50);
    for w in 0..4u32 {
        let word = e.mem.u32(b + 4 * w);
        e.mem.set_u32(a + 4 * w, word);
    }
    let b_key = x87_quiet_bits(e.mem.u32(b + KEY));
    e.mem.set_u32(a + KEY, b_key);
    for w in 5..0x11u32 {
        let word = e.mem.u32(b + 4 * w);
        e.mem.set_u32(a + 4 * w, word);
    }
    let word = e.mem.u32(b + 0x50);
    e.mem.set_u32(a + 0x50, word);
    e.mem.set_u32(b + KEY, a_key);
    for (w, word) in (0..0x11u32).filter(|w| *w != 4).zip(a_words) {
        e.mem.set_u32(b + 4 * w, word);
    }
    e.mem.set_u32(b + 0x50, a_tail);
}

// Translated from 00c852c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: sorts elements `low..=high` (0x60 bytes each)
/// of the array at `base` by the float at `+0x10`, ascending (a quicksort
/// on the middle element's key: scans while below / above it, NaN stopping
/// both scans; recursion on the lower part, a loop on the upper). The swap
/// leaves the words at `+0x44..+0x4c` and `+0x54..+0x5c` in place. The
/// pivot element's other words, which the code copies to a local it never
/// reads, are not copied. `extra` is passed on to the recursion only.
pub fn fn_00c852c0(e: &mut Engine, base: u32, low: i32, high: i32, extra: u32) {
    let mut low = low;
    loop {
        let middle = low.wrapping_add(high) >> 1;
        let pivot = x87_load32(x87_quiet_bits(e.mem.u32(element(base, middle) + KEY)));
        let mut i = low;
        let mut j = high;
        loop {
            // `TEST AH,5; JP`: on while below the pivot.
            while x87_load32(e.mem.u32(element(base, i) + KEY)) < pivot {
                i = i.wrapping_add(1);
            }
            // `TEST AH,0x41; JNZ`: on while above it.
            while x87_load32(e.mem.u32(element(base, j) + KEY)) > pivot {
                j = j.wrapping_sub(1);
            }
            if j < i {
                break;
            }
            if j != i {
                swap(e, element(base, j), element(base, i));
            }
            j = j.wrapping_sub(1);
            i = i.wrapping_add(1);
            if i > j {
                break;
            }
        }
        if low < j {
            fn_00c852c0(e, base, low, j, extra);
        }
        if i >= high {
            return;
        }
        low = i;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c8_52c0, fn_00c852c0(u32, i32, i32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn array(e: &mut Engine, keys: &[f32]) -> u32 {
        let base = e.mem.alloc(ELEMENT_SIZE * keys.len() as u32);
        for (i, key) in keys.iter().enumerate() {
            let at = base + ELEMENT_SIZE * i as u32;
            for w in 0..0x18u32 {
                e.mem.set_u32(at + 4 * w, (i as u32) << 8 | w);
            }
            e.mem.set_f32(at + KEY, *key);
        }
        base
    }

    fn keys(e: &Engine, base: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| e.mem.f32(base + ELEMENT_SIZE * i + KEY))
            .collect()
    }

    #[test]
    fn sorts_by_key() {
        let mut e = Engine::new();
        let input = [5.0f32, -1.0, 3.5, 3.5, 0.0, 9.0, 2.0];
        let base = array(&mut e, &input);
        e.call(0x00c8_52c0, &args![base, 0i32, 6i32, 0u32]);
        let mut sorted = input;
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(keys(&e, base, 7), sorted);
        // Each moved element carries its own words, except the ones the
        // swap leaves in place.
        for i in 0..7u32 {
            let at = base + ELEMENT_SIZE * i;
            let owner = e.mem.u32(at) >> 8;
            assert_eq!(e.mem.u32(at + 0x50), owner << 8 | 0x14);
            assert_eq!(e.mem.u32(at + 0x18), owner << 8 | 6);
            assert_eq!(e.mem.u32(at + 0x44), i << 8 | 0x11);
            assert_eq!(e.mem.u32(at + 0x5c), i << 8 | 0x17);
        }
    }

    #[test]
    fn single_and_sorted_ranges_stay() {
        let mut e = Engine::new();
        let base = array(&mut e, &[1.0, 2.0, 3.0]);
        e.call(0x00c8_52c0, &args![base, 0i32, 2i32, 0u32]);
        assert_eq!(keys(&e, base, 3), vec![1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(base), 0);
        e.call(0x00c8_52c0, &args![base, 1i32, 1i32, 0u32]);
        assert_eq!(keys(&e, base, 3), vec![1.0, 2.0, 3.0]);
    }
}
