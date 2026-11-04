//! `(unplaced)/run_00456540` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A constructor of a 0x34-byte record and accessors of one object (classes
//! not confirmed): a float at +0xA0, the flag word at +0x30, and a 16-bit
//! value reached through the member at +0xB8.

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`thiscall`, no stack words): run on the two three-word
/// members of the record before they are overwritten.
const MEMBER_INIT: u32 = 0x0068_15c0;
/// Three words (`011f426c`, `011f4270`, `011f4274`) copied into both
/// members of the record.
const DEFAULT_TRIPLE: u32 = 0x011f_426c;
/// `00559450` (`thiscall`): returns an object for `004a8ae0`.
const MEMBER_OBJECT: u32 = 0x0055_9450;
/// `TESActorBaseData::GetFatigue` (`004a8ae0`, Xbox PDB name; returns the
/// 16-bit word at +8 in `AX`).
const GET_FATIGUE: u32 = 0x004a_8ae0;

// Translated from 00456540 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The record's constructor (`thiscall`): `006815c0` on the members at +8
/// and +0x14, then +0 and +4 = 0, both members = the three words at
/// `011f426c`, the byte at +0x20 = 0 and the words +0x24..+0x30 = 0.
/// Returns `this`.
pub fn fn_00456540(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.call(MEMBER_INIT, &args![t + 8]);
    e.call(MEMBER_INIT, &args![t + 0x14]);
    e.mem.set_u32(t, 0);
    e.mem.set_u32(t + 4, 0);
    for member in [t + 8, t + 0x14] {
        for i in 0..3 {
            let word = e.global::<u32>(DEFAULT_TRIPLE + 4 * i);
            e.mem.set_u32(member + 4 * i, word);
        }
    }
    e.mem.set_u8(t + 0x20, 0);
    e.mem.set_u32(t + 0x24, 0);
    e.mem.set_u32(t + 0x28, 0);
    e.mem.set_u32(t + 0x2c, 0);
    e.mem.set_u32(t + 0x30, 0);
    this
}

// Translated from 004565f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Returns the float at +0xA0 (`FLD`, so a signalling NaN comes back quiet).
pub fn fn_004565f0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0xa0))
}

// Translated from 00456610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests flag bit 0: `00456630` with the mask 1. The result stays in `EAX`
/// (0 or 1) although the decompiler shows no return value.
pub fn fn_00456610(e: &mut Engine, this: Ptr) -> bool {
    fn_00456630(e, this, 1)
}

// Translated from 00456630 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Whether the flag word at +0x30 has any bit of `mask` (`thiscall`, `RET 4`;
/// 0 or 1 in `EAX`).
pub fn fn_00456630(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0x30) & mask != 0
}

// Translated from 00456650 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `GetFatigue` (`004a8ae0`) of the object `00559450` returns for the member
/// at +0xB8; its `AX` is this function's result.
pub fn fn_00456650(e: &mut Engine, this: Ptr) -> u16 {
    let object = e.call(MEMBER_OBJECT, &args![this.addr() + 0xb8]).u32();
    e.call(GET_FATIGUE, &args![object]).u16()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00456540, fn_00456540(Ptr) -> Ptr),
        entry!(0x004565f0, fn_004565f0(Ptr) -> f32),
        entry!(0x00456610, fn_00456610(Ptr) -> bool),
        entry!(0x00456630, fn_00456630(Ptr, u32) -> bool),
        entry!(0x00456650, fn_00456650(Ptr) -> u16),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_record_gets_the_default_triples_and_zeros() {
        let mut e = Engine::new();
        e.register(MEMBER_INIT, |_, a| a[0].into_ret());
        e.map(DEFAULT_TRIPLE, 12);
        for i in 0..3 {
            e.set_global::<u32>(DEFAULT_TRIPLE + 4 * i, 0x100 + i);
        }
        let r = e.mem.alloc(0x34);
        for i in 0..0x34 {
            e.mem.set_u8(r + i, 0xff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0045_6540, &[r]).u32(), r);
        let log = e.call_log.take().unwrap();
        let inits: Vec<_> = log.iter().filter(|c| c.0 == MEMBER_INIT).collect();
        assert_eq!(
            inits,
            [&(MEMBER_INIT, vec![r + 8]), &(MEMBER_INIT, vec![r + 0x14])]
        );
        assert_eq!(e.mem.u32(r), 0);
        assert_eq!(e.mem.u32(r + 4), 0);
        for base in [r + 8, r + 0x14] {
            assert_eq!(e.mem.u32(base), 0x100);
            assert_eq!(e.mem.u32(base + 4), 0x101);
            assert_eq!(e.mem.u32(base + 8), 0x102);
        }
        assert_eq!(e.mem.u8(r + 0x20), 0);
        assert_eq!(e.mem.u8(r + 0x21), 0xff);
        for off in [0x24, 0x28, 0x2c, 0x30] {
            assert_eq!(e.mem.u32(r + off), 0);
        }
    }

    #[test]
    fn the_float_comes_back_quiet() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0xa4);
        e.mem.set_f32(o + 0xa0, 2.5);
        assert_eq!(e.call(0x0045_65f0, &[o]).f32(), 2.5);
        e.mem.set_u32(o + 0xa0, 0x7f80_0001);
        assert_eq!(e.call(0x0045_65f0, &[o]).f32().to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn asks_the_flag_test_for_mask_one() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0x34);
        e.mem.set_u32(o + 0x30, 0xfffe);
        assert!(!e.call(0x0045_6610, &[o]).bool());
        e.mem.set_u32(o + 0x30, 1);
        assert!(e.call(0x0045_6610, &[o]).bool());
    }

    #[test]
    fn the_flag_test_masks_the_word() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0x34);
        e.mem.set_u32(o + 0x30, 0x8000_0000);
        assert_eq!(e.call(0x0045_6630, &[o, 0x8000_0001]).u32(), 1);
        assert_eq!(e.call(0x0045_6630, &[o, 0x7fff_ffff]).u32(), 0);
    }

    #[test]
    fn reads_the_fatigue_of_the_member_object() {
        let mut e = Engine::new();
        e.register(MEMBER_OBJECT, |_, a| (a[0] + 1).into_ret());
        e.register(GET_FATIGUE, |_, a| (a[0] << 16 | 0x0042).into_ret());
        assert_eq!(e.call(0x0045_6650, &[0x1000]).u32() as u16, 0x0042);
        e.call_log = Some(vec![]);
        e.call(0x0045_6650, &[0x1000]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMBER_OBJECT, vec![0x10b8])));
        assert!(log.contains(&(GET_FATIGUE, vec![0x10b9])));
    }
}
