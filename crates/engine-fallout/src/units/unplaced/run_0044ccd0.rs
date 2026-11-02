//! `(unplaced)/run_0044ccd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_Char_traits_cat<char_traits<char>>()` (cdecl, no arguments): the
/// secure-traits tag byte.
const CHAR_TRAITS_CATEGORY: u32 = 0x0065_e750;
/// `_Traits_helper::copy_s(dest, destsize, src, count, tag)` (cdecl).
const COPY_WITH_TAG: u32 = 0x0044_e420;

// Translated from 0044ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Traits_helper::copy_s<char_traits<char>>(dest, destsize, src,
/// count)` (cdecl; the name Ghidra's library match gives): gets the traits
/// category tag (`0065e750`) and forwards to the tagged `copy_s`
/// (`0044e420`), answering its result.
pub fn traits_helper_copy_s(
    e: &mut Engine,
    destination: u32,
    destination_size: u32,
    source: u32,
    count: u32,
) -> u32 {
    // 0065e750 answers the top byte of ECX, which this cdecl function never sets: the
    // caller's leftover ECX, not modelled (0 here; the tag goes to 0044e420's fifth word,
    // which nothing reads). No words are pushed; the two the translation takes are 0.
    let tag = e.call(CHAR_TRAITS_CATEGORY, &[0, 0, 0]).u8();
    e.call(
        COPY_WITH_TAG,
        &args![destination, destination_size, source, count, tag as u32],
    )
    .u32()
}

// Translated from 0044cd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a five-word record (class unconfirmed): the three
/// arguments at `+0`, `+4`, `+8`, zero at `+0xc` and `+0x10`. Returns
/// `this`.
pub fn fn_0044cd00(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    e.mem.set_u32(this.addr(), first);
    e.mem.set_u32(this.addr() + 4, second);
    e.mem.set_u32(this.addr() + 8, third);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0044ccd0,
            traits_helper_copy_s(u32, u32, u32, u32) -> u32
        ),
        entry!(0x0044cd00, fn_0044cd00(Ptr, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_s_passes_the_category_tag() {
        let mut e = Engine::new();
        e.register(CHAR_TRAITS_CATEGORY, |_, _| 0x1234_5601u32.into_ret());
        e.register(COPY_WITH_TAG, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(0x0044_ccd0, &args![0x10u32, 0x20u32, 0x30u32, 4u32]);
        assert_eq!(r.u32(), 0x10);
        let log = e.call_log.take().unwrap();
        let copy = log.iter().find(|(a, _)| *a == COPY_WITH_TAG).unwrap();
        // The tag is the low byte, zero-extended.
        assert_eq!(copy.1, vec![0x10, 0x20, 0x30, 4, 1]);
    }

    #[test]
    fn record_constructor_stores_three_words_and_two_zeros() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        for i in 0..5 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        let back = e.call(0x0044_cd00, &args![this, 1u32, 2u32, 3u32]).u32();
        assert_eq!(back, this);
        let words: Vec<u32> = (0..5).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 0, 0]);
    }
}
