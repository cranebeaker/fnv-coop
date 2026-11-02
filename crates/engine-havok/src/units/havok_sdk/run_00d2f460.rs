//! `Havok SDK/run_00d2f460` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The float `0x7f7fffee` (`0104da48`) and the four sign-bit words (`010c44d0`).
const LARGE_FLOAT: u32 = 0x0104_da48;
const SIGN_MASK: u32 = 0x010c_44d0;

// Translated from 00d2f460 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x14`: five stack words `first`, `second`, `info`, `value_a`,
/// `value_b`; the names are descriptions, not confirmed): the method form of `00d2daf0`. Builds the
/// same copy of the 0x70 bytes at `info` (the four-lane vector at `+0x50` sign-flipped with
/// `010c44d0`) and the same records (vtable word `0x010c_e80c`, the float at `0104da48` at `+4`,
/// the vector at `info + 0x50` at `+0x10`, `value_a` / `value_b` at `+0x20`; the second record
/// only when `value_b` is not 0) in a 0xd8-byte frame, then calls `00d315a0` on `this` with
/// `(second, first, copy, record_a, record_b or 0)`. The words the exe leaves uninitialised are
/// zero here.
pub fn fn_00d2f460(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    info: Ptr,
    value_a: u32,
    value_b: u32,
) {
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
            0x00d3_15a0,
            &args![this, second, first, s + 0x68, s + 8, record_b],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d2_f460,
        fn_00d2f460(Ptr, u32, u32, Ptr, u32, u32)
    )]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00d2f460_builds_the_records_and_calls_00d315a0_on_this() {
        for (value_b, with_second) in [(0x66u32, true), (0, false)] {
            let mut e = Engine::new();
            e.map(0x0100_0000, 0x0030_0000);
            e.mem.set_u32(LARGE_FLOAT, 0x7fc0_0001);
            for lane in 0..4 {
                e.mem.set_u32(SIGN_MASK + lane * 4, 0x8000_0000);
            }
            let info = e.mem.alloc(0x70);
            for word in 0..0x1c {
                e.mem.set_u32(info + word * 4, 0x2000 + word);
            }
            let seen = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
            let log = Rc::clone(&seen);
            e.register_double(0x00d3_15a0, move |e, a| {
                let mut words = a.to_vec();
                for word in 0..0x1c {
                    words.push(e.mem.u32(a[3] + word * 4));
                }
                for word in 0..9 {
                    words.push(e.mem.u32(a[4] + word * 4));
                }
                if a[5] != 0 {
                    for word in 0..9 {
                        words.push(e.mem.u32(a[5] + word * 4));
                    }
                }
                log.borrow_mut().push(words);
                Ret::default()
            });
            let this = e.mem.alloc(0x10);
            e.call(
                0x00d2_f460,
                &args![this, 0x11u32, 0x22u32, info, 0x44u32, value_b],
            );
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            let w = &seen[0];
            assert_eq!((w[0], w[1], w[2]), (this, 0x22, 0x11));
            assert_eq!(w[5] != 0, with_second);
            for word in 0..0x1c {
                let expected = if (0x14..0x18).contains(&word) {
                    (0x2000 + word) ^ 0x8000_0000
                } else {
                    0x2000 + word
                };
                assert_eq!(w[6 + word as usize], expected, "copy word {word}");
            }
            let a_record = &w[6 + 0x1c..6 + 0x1c + 9];
            assert_eq!(a_record[0], 0x010c_e80c);
            assert_eq!(a_record[1], 0x7fc0_0001);
            assert_eq!(&a_record[4..8], &[0x2014, 0x2015, 0x2016, 0x2017]);
            assert_eq!(a_record[8], 0x44);
            if with_second {
                let b_record = &w[6 + 0x1c + 9..];
                assert_eq!(b_record[0], 0x010c_e80c);
                assert_eq!(&b_record[4..8], &[0x2014, 0x2015, 0x2016, 0x2017]);
                assert_eq!(b_record[8], 0x66);
            } else {
                assert_eq!(w.len(), 6 + 0x1c + 9);
            }
        }
    }

    #[test]
    fn fn_00d2f460_copies_the_float_through_the_x87_stack() {
        // A signalling NaN is quieted by the FLD / FSTP pair.
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(LARGE_FLOAT, 0x7f80_0001);
        let info = e.mem.alloc(0x70);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d3_15a0, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[4] + 4));
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call(0x00d2_f460, &args![this, 0u32, 0u32, info, 0u32, 0u32]);
        assert_eq!(*seen.borrow(), vec![0x7fc0_0001]);
    }
}
