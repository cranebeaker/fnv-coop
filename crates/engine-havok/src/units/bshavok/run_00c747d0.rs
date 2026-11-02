//! `BSHavok/run_00c747d0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Six small accessors on one object. The exe data names no class for them (the Xbox PDB
//! names them `range`), so they keep their addresses and raw field offsets.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c747d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c747d0` (unnamed in the Xbox PDB): one argument, a flag (only its low byte is
/// tested). The byte at `+0x222` becomes: unchanged when the byte at `+0x43` is not 0;
/// otherwise 1 when both the byte at `+0x220` and the flag are not 0, else 0. When it was not
/// 0 and is now 0, the byte at `+0x223` is set to 1.
pub fn fn_00c747d0(e: &mut Engine, this: u32, flag: u32) {
    let old = e.mem.u8(this + 0x222);
    let new = if e.mem.u8(this + 0x43) != 0 {
        old
    } else if e.mem.u8(this + 0x220) != 0 && flag as u8 != 0 {
        1
    } else {
        0
    };
    e.mem.set_u8(this + 0x222, new);
    if old != 0 && new == 0 {
        e.mem.set_u8(this + 0x223, 1);
    }
}

// Translated from 00c74820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c74820` (unnamed in the Xbox PDB): takes an index and a flag (low byte used). When
/// the flag is not 0, the byte at `+0x21e` is replaced: by itself when the byte at `+0x43` is
/// not 0, otherwise by whether the byte at `+0x21d` is not 0. Then, when the byte at `+0x21e`
/// is not 0, the flag byte is stored at `this + index * 48 + 0x1bc`.
pub fn fn_00c74820(e: &mut Engine, this: u32, index: u32, flag: u32) {
    let flag = flag as u8;
    if flag != 0 {
        let value = if e.mem.u8(this + 0x43) == 0 {
            (e.mem.u8(this + 0x21d) != 0) as u8
        } else {
            e.mem.u8(this + 0x21e)
        };
        e.mem.set_u8(this + 0x21e, value);
    }
    if e.mem.u8(this + 0x21e) != 0 {
        let slot = this
            .wrapping_add(index.wrapping_mul(48))
            .wrapping_add(0x1bc);
        e.mem.set_u8(slot, flag);
    }
}

// Translated from 00c74870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c74870` (unnamed in the Xbox PDB): stores the float at `this + index * 48 + 0x1b4`
/// (a load and store through the x87 stack, which quiets a signalling NaN).
pub fn fn_00c74870(e: &mut Engine, this: u32, index: u32, value: f32) {
    let slot = this
        .wrapping_add(index.wrapping_mul(48))
        .wrapping_add(0x1b4);
    e.mem.set_u32(slot, x87_quiet_bits(value.to_bits()));
}

// Translated from 00c74890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c74890` (unnamed in the Xbox PDB): stores the word `value` at
/// `this + index * 48 + 0x1b0`.
pub fn fn_00c74890(e: &mut Engine, this: u32, index: u32, value: u32) {
    let slot = this
        .wrapping_add(index.wrapping_mul(48))
        .wrapping_add(0x1b0);
    e.mem.set_u32(slot, value);
}

// Translated from 00c748b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c748b0` (unnamed in the Xbox PDB): when the word at `+0x194` is 0, sets the byte at
/// `+0x181` to 1 and the word at `+0x194` to 1.
pub fn fn_00c748b0(e: &mut Engine, this: u32) {
    if e.mem.u32(this + 0x194) == 0 {
        e.mem.set_u8(this + 0x181, 1);
        e.mem.set_u32(this + 0x194, 1);
    }
}

// Translated from 00c748d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c748d0` (unnamed in the Xbox PDB): stores two floats (through the x87 stack) at
/// `this + index * 80 + 0x134` and `+ 0x138`.
pub fn fn_00c748d0(e: &mut Engine, this: u32, index: u32, first: f32, second: f32) {
    let base = this.wrapping_add(index.wrapping_mul(80));
    e.mem
        .set_u32(base.wrapping_add(0x134), x87_quiet_bits(first.to_bits()));
    e.mem
        .set_u32(base.wrapping_add(0x138), x87_quiet_bits(second.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c747d0, fn_00c747d0(u32, u32)),
        entry!(0x00c74820, fn_00c74820(u32, u32, u32)),
        entry!(0x00c74870, fn_00c74870(u32, u32, f32)),
        entry!(0x00c74890, fn_00c74890(u32, u32, u32)),
        entry!(0x00c748b0, fn_00c748b0(u32)),
        entry!(0x00c748d0, fn_00c748d0(u32, u32, f32, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_update_follows_the_two_guard_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x300);
        // Guard 0x43 clear, 0x220 set, flag set: becomes 1, nothing flagged.
        e.mem.set_u8(this + 0x220, 1);
        e.call(0x00c7_47d0, &args![this, 0x100u32]);
        assert_eq!(e.mem.u8(this + 0x222), 0);
        assert_eq!(e.mem.u8(this + 0x223), 0);
        e.call(0x00c7_47d0, &args![this, 0x101u32]);
        assert_eq!(e.mem.u8(this + 0x222), 1);
        // Now flag 0 turns it off and marks +0x223.
        e.call(0x00c7_47d0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x222), 0);
        assert_eq!(e.mem.u8(this + 0x223), 1);
    }

    #[test]
    fn flag_update_keeps_the_old_value_when_the_override_byte_is_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x300);
        e.mem.set_u8(this + 0x43, 1);
        e.mem.set_u8(this + 0x222, 7);
        e.call(0x00c7_47d0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x222), 7);
        assert_eq!(e.mem.u8(this + 0x223), 0);
    }

    #[test]
    fn slot_flag_copies_the_selector_and_stores_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x300);
        // Flag 0 and +0x21e clear: nothing stored.
        e.call(0x00c7_4820, &args![this, 2u32, 0u32]);
        assert_eq!(e.mem.u8(this + 2 * 48 + 0x1bc), 0);
        // Flag 5 with +0x43 clear and +0x21d set: +0x21e becomes 1, the byte 5 is stored.
        e.mem.set_u8(this + 0x21d, 9);
        e.call(0x00c7_4820, &args![this, 2u32, 5u32]);
        assert_eq!(e.mem.u8(this + 0x21e), 1);
        assert_eq!(e.mem.u8(this + 2 * 48 + 0x1bc), 5);
        // Flag 0 with +0x21e set stores the 0 byte.
        e.call(0x00c7_4820, &args![this, 2u32, 0u32]);
        assert_eq!(e.mem.u8(this + 2 * 48 + 0x1bc), 0);
    }

    #[test]
    fn slot_flag_keeps_the_selector_when_the_override_byte_is_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x300);
        e.mem.set_u8(this + 0x43, 1);
        e.mem.set_u8(this + 0x21d, 1);
        e.call(0x00c7_4820, &args![this, 0u32, 1u32]);
        assert_eq!(e.mem.u8(this + 0x21e), 0);
        assert_eq!(e.mem.u8(this + 0x1bc), 0);
        e.mem.set_u8(this + 0x21e, 3);
        e.call(0x00c7_4820, &args![this, 0u32, 1u32]);
        assert_eq!(e.mem.u8(this + 0x21e), 3);
        assert_eq!(e.mem.u8(this + 0x1bc), 1);
    }

    #[test]
    fn float_store_quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x300);
        e.call(0x00c7_4870, &args![this, 1u32, 2.5f32]);
        assert_eq!(e.mem.f32(this + 48 + 0x1b4), 2.5);
        e.call(0x00c7_4870, &args![this, 1u32, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this + 48 + 0x1b4), 0x7fc0_0001);
    }

    #[test]
    fn word_store_goes_to_the_indexed_slot() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x300);
        e.call(0x00c7_4890, &args![this, 3u32, 0xdead_beefu32]);
        assert_eq!(e.mem.u32(this + 3 * 48 + 0x1b0), 0xdead_beef);
    }

    #[test]
    fn first_use_sets_the_flag_byte_once() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x300);
        e.call(0x00c7_48b0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x181), 1);
        assert_eq!(e.mem.u32(this + 0x194), 1);
        e.mem.set_u8(this + 0x181, 0);
        e.call(0x00c7_48b0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x181), 0);
    }

    #[test]
    fn pair_store_writes_both_floats() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x400);
        e.call(
            0x00c7_48d0,
            &args![this, 2u32, 1.5f32, f32::from_bits(0xff80_0002)],
        );
        assert_eq!(e.mem.f32(this + 160 + 0x134), 1.5);
        assert_eq!(e.mem.u32(this + 160 + 0x138), 0xffc0_0002);
    }
}
