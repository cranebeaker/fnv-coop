//! `fallout/misc/run_00841120` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (`thiscall`, no stack words): the word at `this`.
const WORD_AT: u32 = 0x0055_9450;
/// `00562210` (`thiscall` on a flag word, one word: clears those bits; `RET 4`).
const CLEAR_BITS: u32 = 0x0056_2210;

// Translated from 00841120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five words: the result's address, a flag word and
/// a word passed by value, a form type and a byte flag; answers the result's address). In
/// its own argument slot the flag word loses the bits of the second word (read through
/// `00559450` on its slot), then the upper 16 bits (`0xFFFF0000`), and, with `flag` set
/// and a type of 0x3B or 0x3C (signed compare), bit `0x400` (each through `00562210`); the
/// final word is stored at `result`.
pub fn fn_00841120(
    e: &mut Engine,
    result: Ptr,
    flags: u32,
    mask: u32,
    form_type: i32,
    flag: u8,
) -> Ptr {
    e.with_stack(8, |e, slots| {
        let flags_slot = slots.addr();
        let mask_slot = flags_slot + 4;
        e.mem.set_u32(flags_slot, flags);
        e.mem.set_u32(mask_slot, mask);
        let bits = e.call(WORD_AT, &args![mask_slot]).u32();
        e.call(CLEAR_BITS, &args![flags_slot, bits]);
        e.call(CLEAR_BITS, &args![flags_slot, 0xffff_0000u32]);
        if flag != 0 && (0x3b..=0x3c).contains(&form_type) {
            e.call(CLEAR_BITS, &args![flags_slot, 0x400u32]);
        }
        let value = e.mem.u32(flags_slot);
        e.mem.set_u32(result.addr(), value);
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0084_1120,
        fn_00841120(Ptr, u32, u32, i32, u8) -> Ptr
    )]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CLEAR_BITS, |e, a| {
            let value = e.mem.u32(a[0]) & !a[1];
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e
    }

    #[test]
    fn fn_00841120_clears_the_mask_the_upper_half_and_maybe_0x400() {
        let mut e = engine();
        let result = e.mem.alloc(4);
        let back = e
            .call(
                0x0084_1120,
                &args![result, 0xffff_ffffu32, 0x11u32, 0x3bu32, 1u32],
            )
            .u32();
        assert_eq!(back, result);
        assert_eq!(e.mem.u32(result), 0x0000_fbee);
        // Type 0x3C with the flag: 0x400 too; 0x3D or no flag: kept.
        e.call(0x0084_1120, &args![result, 0xffffu32, 0u32, 0x3cu32, 1u32]);
        assert_eq!(e.mem.u32(result), 0xfbff);
        e.call(0x0084_1120, &args![result, 0xffffu32, 0u32, 0x3du32, 1u32]);
        assert_eq!(e.mem.u32(result), 0xffff);
        e.call(
            0x0084_1120,
            &args![result, 0xffffu32, 0u32, 0x3bu32, 0x100u32],
        );
        assert_eq!(e.mem.u32(result), 0xffff);
        // The type compares signed: a negative one is below 0x3B.
        e.call(
            0x0084_1120,
            &args![result, 0xffffu32, 0u32, 0xffff_ffffu32, 1u32],
        );
        assert_eq!(e.mem.u32(result), 0xffff);
    }
}
