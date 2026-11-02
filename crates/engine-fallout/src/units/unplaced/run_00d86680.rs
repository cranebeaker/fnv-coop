//! `(unplaced)/run_00d86680` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The exe's inlined bit count of one word (the SWAR steps: pairs, nibbles, bytes, halves).
fn bit_count(word: u32) -> u32 {
    let pairs = ((word >> 1) & 0x5555_5555).wrapping_add(word & 0x5555_5555);
    let nibbles = ((pairs >> 2) & 0x3333_3333).wrapping_add(pairs & 0x3333_3333);
    let bytes = ((nibbles >> 4) & 0x0f0f_0f0f).wrapping_add(nibbles & 0x0f0f_0f0f);
    let halves = ((bytes >> 8) & 0x000f_000f).wrapping_add(bytes & 0x000f_000f);
    (halves & 0xff).wrapping_add(halves >> 16)
}

// Translated from 00d86680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: words, count): the number of set bits in `count`
/// 32-bit words. Two words per pass into two sums while `index < count - 1` (with at least
/// 2 words), then the last word when one is left; the three sums added.
pub fn fn_00d86680(e: &mut Engine, words: Ptr, count: i32) -> u32 {
    let words = words.addr();
    let word = |e: &Engine, index: i32| {
        e.mem
            .u32(words.wrapping_add((index as u32).wrapping_mul(4)))
    };
    let mut even = 0u32;
    let mut odd = 0u32;
    let mut last = 0u32;
    let mut index: i32 = 0;
    if count >= 2 {
        loop {
            even = bit_count(word(e, index)).wrapping_add(even);
            odd = bit_count(word(e, index + 1)).wrapping_add(odd);
            index += 2;
            if index >= count.wrapping_sub(1) {
                break;
            }
        }
    }
    if index < count {
        last = bit_count(word(e, index));
    }
    even.wrapping_add(odd).wrapping_add(last)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d8_6680, fn_00d86680(Ptr, i32) -> u32)]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn counts_the_set_bits_of_every_word() {
        let mut e = Engine::new();
        let words = e.mem.alloc(16);
        for (i, w) in [0xffff_ffffu32, 0x8000_0001, 0x0f0f_0000, 0x7]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(words + 4 * i as u32, *w);
        }
        for (count, want) in [(0i32, 0u32), (1, 32), (2, 34), (3, 42), (4, 45), (-1, 0)] {
            assert_eq!(
                e.call(0x00d8_6680, &args![words, count]).u32(),
                want,
                "{count}"
            );
        }
    }
}
