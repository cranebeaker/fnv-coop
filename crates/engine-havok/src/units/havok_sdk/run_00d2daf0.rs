//! `Havok SDK/run_00d2daf0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The float `0x7f7fffee` (`0104da48`) and the four sign-bit words (`010c44d0`).
const LARGE_FLOAT: u32 = 0x0104_da48;
const SIGN_MASK: u32 = 0x010c_44d0;

// Translated from 00d2daf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, five words `first`, `second`, `info`, `value_a`, `value_b`; the names are
/// descriptions, not confirmed): builds two records and a copy in a 0xd8-byte frame and calls
/// `00d2f0d0(second, first, copy, record_a, record_b)` (cdecl). `copy` is the 0x70 bytes at `info`
/// with the four-lane vector at `+0x50` sign-flipped (`XORPS` with `010c44d0`). A record is a
/// vtable word `0x010c_e80c`, the float at `0104da48` (copied through `FLD`/`FSTP`) at `+4`, the
/// vector at `info + 0x50` at `+0x10`, and a word at `+0x20`: `value_a` for `record_a` (frame
/// `+8`), `value_b` for `record_b` (frame `+0x38`), which is only built, and passed, when `value_b`
/// is not 0 (else the fifth argument is 0). The words the exe leaves uninitialised (`+8` and
/// `+0xc` of each record and the rest of the frame) are zero here.
pub fn fn_00d2daf0(e: &mut Engine, first: u32, second: u32, info: Ptr, value_a: u32, value_b: u32) {
    e.with_stack(0xd8, |e, frame| {
        let s = frame.addr();
        let at = info.addr();
        let large = x87_quiet_bits(e.mem.u32(LARGE_FLOAT));
        e.mem.set_u32(s + 0xc, large);
        let vector = [0u32, 1, 2, 3].map(|lane| e.mem.u32(at + 0x50 + lane * 4));
        let mask = [0u32, 1, 2, 3].map(|lane| e.mem.u32(SIGN_MASK + lane * 4));
        for word in 0..0x1c {
            let value = e.mem.u32(at + word * 4);
            e.mem.set_u32(s + 0x68 + word * 4, value);
        }
        for lane in 0..4 {
            let value = e.mem.u32(at + 0x50 + lane * 4);
            e.mem.set_u32(s + 0x18 + lane * 4, value);
        }
        e.mem.set_u32(s + 0x28, value_a);
        for lane in 0..4 {
            e.mem.set_u32(
                s + 0xb8 + lane * 4,
                vector[lane as usize] ^ mask[lane as usize],
            );
        }
        e.mem.set_u32(s + 8, 0x010c_e80c);
        let record_b = if value_b != 0 {
            e.mem.set_u32(s + 0x38, 0x010c_e80c);
            e.mem.set_u32(s + 0x3c, large);
            for lane in 0..3 {
                let value = e.mem.u32(at + 0x50 + lane * 4);
                e.mem.set_u32(s + 0x48 + lane * 4, value);
            }
            e.mem.set_u32(s + 0x58, value_b);
            let last = e.mem.u32(at + 0x5c);
            e.mem.set_u32(s + 0x54, last);
            s + 0x38
        } else {
            0
        };
        e.call(
            0x00d2_f0d0,
            &args![second, first, s + 0x68, s + 8, record_b],
        );
    });
}

// Translated from 00d2dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of a class with the vtable word `0x0104_da3c` (thiscall, one stack
/// word `flags`; no map name): writes that vtable word at `+0` and, when bit 0 of `flags` is set,
/// frees the 8-byte block through slot 8 of the memory router's allocator. Returns `this`.
pub fn fn_00d2dbe0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0104_da3c);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, 8u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_daf0, fn_00d2daf0(u32, u32, Ptr, u32, u32)),
        entry!(0x00d2_dbe0, fn_00d2dbe0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00d2daf0_builds_the_records_and_calls_00d2f0d0() {
        for (value_b, with_second) in [(0x66u32, true), (0, false)] {
            let mut e = Engine::new();
            e.map(0x0100_0000, 0x0030_0000);
            e.mem.set_u32(LARGE_FLOAT, 0x7f7f_ffee);
            for lane in 0..4 {
                e.mem.set_u32(SIGN_MASK + lane * 4, 0x8000_0000);
            }
            let info = e.mem.alloc(0x70);
            for word in 0..0x1c {
                e.mem.set_u32(info + word * 4, 0x1000 + word);
            }
            let seen = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
            let log = Rc::clone(&seen);
            e.register_double(0x00d2_f0d0, move |e, a| {
                let mut words = a.to_vec();
                // The copy: 28 words; record A: 9 words; record B: 9 words (when passed).
                for word in 0..0x1c {
                    words.push(e.mem.u32(a[2] + word * 4));
                }
                for word in 0..9 {
                    words.push(e.mem.u32(a[3] + word * 4));
                }
                if a[4] != 0 {
                    for word in 0..9 {
                        words.push(e.mem.u32(a[4] + word * 4));
                    }
                }
                log.borrow_mut().push(words);
                Ret::default()
            });
            e.call(
                0x00d2_daf0,
                &args![0x11u32, 0x22u32, info, 0x44u32, value_b],
            );
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            let w = &seen[0];
            assert_eq!((w[0], w[1]), (0x22, 0x11));
            assert_eq!(w[4] != 0, with_second);
            // The copy is the info with the vector at +0x50 sign-flipped.
            for word in 0..0x1c {
                let expected = if (0x14..0x18).contains(&word) {
                    (0x1000 + word) ^ 0x8000_0000
                } else {
                    0x1000 + word
                };
                assert_eq!(w[5 + word as usize], expected, "copy word {word}");
            }
            // Record A: vtable, float, (two uninitialised), vector, value_a.
            let a_record = &w[5 + 0x1c..5 + 0x1c + 9];
            assert_eq!(a_record[0], 0x010c_e80c);
            assert_eq!(a_record[1], 0x7f7f_ffee);
            assert_eq!(&a_record[2..4], &[0, 0]);
            assert_eq!(&a_record[4..8], &[0x1014, 0x1015, 0x1016, 0x1017]);
            assert_eq!(a_record[8], 0x44);
            if with_second {
                let b_record = &w[5 + 0x1c + 9..];
                assert_eq!(b_record[0], 0x010c_e80c);
                assert_eq!(b_record[1], 0x7f7f_ffee);
                assert_eq!(&b_record[4..8], &[0x1014, 0x1015, 0x1016, 0x1017]);
                assert_eq!(b_record[8], 0x66);
            } else {
                assert_eq!(w.len(), 5 + 0x1c + 9);
            }
        }
    }

    #[test]
    fn fn_00d2dbe0_frees_the_block_only_when_bit_zero_is_set() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, 0x0900_0081);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x0900_0081, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let result = e.call(0x00d2_dbe0, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x0104_da3c);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let result = e.call(0x00d2_dbe0, &args![this, 1u32]).u32();
        assert_eq!(result, this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[2..],
            [
                (MEMORY_ROUTER, vec![]),
                (0x0900_0081, vec![allocator, this, 8])
            ]
        );
    }
}
