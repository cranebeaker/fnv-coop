//! `(unplaced)/run_00ec7ce3` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ec7ce3 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checked CRT movement. Zero length succeeds even for null pointers. Invalid
/// pointers or insufficient capacity set the thread error and call the native
/// invalid-parameter handler; valid moves use the existing memmove boundary.
pub fn checked_move(
    e: &mut Engine,
    destination: Ptr,
    capacity: u32,
    source: Ptr,
    size: u32,
) -> u32 {
    if size == 0 {
        return 0;
    }
    let error = if destination.is_null() || source.is_null() {
        0x16u32
    } else if capacity < size {
        0x22u32
    } else {
        e.call(0x00ec7230, &args![destination, source, size]);
        return 0;
    };
    let thread_error = e.call(0x00ec85e3, &[]).u32();
    e.mem.set_u32(thread_error, error);
    e.call(0x00ec7c30, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
    error
}

// Translated from 00ec7d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the CRT `abs` shape, cdecl, one word): the argument, negated when it is below
/// zero (`NEG`, so `i32::MIN` stays `i32::MIN`).
pub fn fn_00ec7d40(_e: &mut Engine, value: i32) -> i32 {
    if value >= 0 {
        value
    } else {
        value.wrapping_neg()
    }
}

/// The import slot of KERNEL32 `GetSystemTimeAsFileTime` (`00fdf184`, one word).
const GET_SYSTEM_TIME_AS_FILE_TIME: u32 = 0x00fd_f184;
/// `__allmul` (`00ecc300`, four words: the low and high words of each factor, `RET 0x10`).
const ALLMUL: u32 = 0x00ec_c300;
/// The 64-bit start time `___inittime` keeps (`.data`, low word first).
const START_TIME_0126FDA0: u32 = 0x0126_fda0;

// Translated from 00ec7d95 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___inittime` (Ghidra library match; a C initializer of `_initterm_e`'s table, no
/// arguments): reads the system time as a `FILETIME` (8 bytes on the stack), forms
/// `__allmul(dwHighDateTime, 0, 0, 1) + dwLowDateTime` (64-bit add with carry) and stores it
/// at `0126fda0` (low word) and `0126fda4` (high word). Returns 0.
pub fn inittime(e: &mut Engine) -> u32 {
    e.with_stack(8, |e, file_time| {
        e.call(GET_SYSTEM_TIME_AS_FILE_TIME, &args![file_time]);
        let high = e.mem.u32(file_time.addr() + 4);
        let product = e.call(ALLMUL, &args![high, 0u32, 0u32, 1u32]);
        let low = e.mem.u32(file_time.addr());
        let (sum_low, carry) = product.eax.overflowing_add(low);
        let sum_high = product.edx.wrapping_add(carry as u32);
        e.mem.set_u32(START_TIME_0126FDA0, sum_low);
        e.mem.set_u32(START_TIME_0126FDA0 + 4, sum_high);
    });
    0
}

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`, thiscall, one word: the `_locale_t`).
const LOCALE_UPDATE: u32 = 0x00ec_660e;
/// `_errno` (`00ec85e3`): the address of the thread's `errno`.
const ERRNO: u32 = 0x00ec_85e3;
/// `_invalid_parameter` (`00ec7c30`, five words, all zero here).
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
/// `___ascii_strnicmp` (`00ed5bc0`, cdecl: first, second, count).
const ASCII_STRNICMP: u32 = 0x00ed_5bc0;
/// `__tolower_l` (`00ec6695`, cdecl: character, `_locale_t`).
const TOLOWER_L: u32 = 0x00ec_6695;
/// The `_LocaleUpdate` local (0x10 bytes): the `_locale_tstruct` (`locinfo` at +0, `mbcinfo`
/// at +4), the thread data at +8 and the "updated" byte at +0xc.
const LOCALE_UPDATE_SIZE: u32 = 0x10;
const LOCALE_UPDATE_THREAD_DATA: u32 = 0x8;
const LOCALE_UPDATE_UPDATED: u32 = 0xc;

/// The end of `_strnicmp_l`'s paths: when the `_LocaleUpdate` changed the thread's locale
/// (its byte at +0xc), clears bit 1 of the thread data's word at +0x70.
fn locale_update_end(e: &mut Engine, update: u32) {
    if e.mem.u8(update + LOCALE_UPDATE_UPDATED) != 0 {
        let thread_data = e.mem.u32(update + LOCALE_UPDATE_THREAD_DATA);
        let flags = e.mem.u32(thread_data + 0x70);
        e.mem.set_u32(thread_data + 0x70, flags & 0xffff_fffd);
    }
}

// Translated from 00ec7dce (decompiled, FalloutNV.exe 1.4.0.525)
/// `__strnicmp_l` (Ghidra library match; cdecl, four words): 0 when `count` is 0. Otherwise,
/// with a `_LocaleUpdate` for `locale` on the stack: a null string sets `errno` to `0x16`,
/// calls `_invalid_parameter(0, 0, 0, 0, 0)` and returns `0x7fffffff`; so does a `count` above
/// `0x7fffffff`. When the locale's `LC_CTYPE` handle (`locinfo + 0x14`) is 0 it returns
/// `___ascii_strnicmp(first, second, count)`; otherwise it lowers each pair of bytes with
/// `__tolower_l` and stops after `count` pairs, at a 0 in the first string, or at the first
/// difference, returning the difference of the last pair.
pub fn strnicmp_l(e: &mut Engine, first: Ptr, second: Ptr, count: u32, locale: u32) -> i32 {
    if count == 0 {
        return 0;
    }
    e.with_stack(LOCALE_UPDATE_SIZE, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        if first.is_null() || second.is_null() || count > 0x7fff_ffff {
            let error = e.call(ERRNO, &args![]).u32();
            e.mem.set_u32(error, 0x16);
            e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            locale_update_end(e, update);
            return 0x7fff_ffff;
        }
        let locinfo = e.mem.u32(update);
        let result = if e.mem.u32(locinfo + 0x14) == 0 {
            e.call(ASCII_STRNICMP, &args![first, second, count]).i32()
        } else {
            let (mut a, mut b) = (first.addr(), second.addr());
            let mut left = count;
            loop {
                let byte = e.mem.u8(a) as u32;
                let lowered_first = e.call(TOLOWER_L, &args![byte, update]).i32();
                a = a.wrapping_add(1);
                let byte = e.mem.u8(b) as u32;
                let lowered_second = e.call(TOLOWER_L, &args![byte, update]).i32();
                b = b.wrapping_add(1);
                left -= 1;
                if left == 0 || lowered_first == 0 || lowered_first != lowered_second {
                    break lowered_first.wrapping_sub(lowered_second);
                }
            }
        };
        locale_update_end(e, update);
        result
    })
}

/// `__aulldiv` (`00ecc590`, four words: the low and high words of the dividend, then of the
/// divisor; `RET 0x10`; the quotient in `EDX:EAX`).
const AULLDIV: u32 = 0x00ec_c590;
/// Ticks of a `FILETIME` (100 ns) per millisecond.
const TICKS_PER_MILLISECOND: u32 = 10_000;

// Translated from 00ec7d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `clock` (Ghidra library match; cdecl, no arguments): the milliseconds since `___inittime`.
/// Reads the system time as a `FILETIME` (8 bytes on the stack), forms
/// `__allmul(dwHighDateTime, 0, 0, 1)` minus the start time (`0126fda0`, 64-bit with
/// borrow) plus `dwLowDateTime` (64-bit with carry), and answers the low word of
/// `__aulldiv(that, 10000)`.
pub fn clock(e: &mut Engine) -> u32 {
    e.with_stack(8, |e, file_time| {
        e.call(GET_SYSTEM_TIME_AS_FILE_TIME, &args![file_time]);
        let high = e.mem.u32(file_time.addr() + 4);
        let product = e.call(ALLMUL, &args![high, 0u32, 0u32, 1u32]);
        let start_low = e.mem.u32(START_TIME_0126FDA0);
        let (low, borrow) = product.eax.overflowing_sub(start_low);
        let start_high = e.mem.u32(START_TIME_0126FDA0 + 4);
        let high = product
            .edx
            .wrapping_sub(start_high)
            .wrapping_sub(u32::from(borrow));
        let time_low = e.mem.u32(file_time.addr());
        let (low, carry) = low.overflowing_add(time_low);
        let high = high.wrapping_add(u32::from(carry));
        e.call(AULLDIV, &args![low, high, TICKS_PER_MILLISECOND, 0u32])
            .u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_7d50, clock() -> u32),
        entry!(0x00ec7ce3, checked_move(Ptr, u32, Ptr, u32) -> u32),
        entry!(0x00ec_7d40, fn_00ec7d40(i32) -> i32),
        entry!(0x00ec_7d95, inittime() -> u32),
        entry!(0x00ec_7dce, strnicmp_l(Ptr, Ptr, u32, u32) -> i32),
    ]
}

#[cfg(test)]
mod b001_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn abs_negates_only_negative_values() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec_7d40, &args![5i32]).i32(), 5);
        assert_eq!(e.call(0x00ec_7d40, &args![0i32]).i32(), 0);
        assert_eq!(e.call(0x00ec_7d40, &args![-7i32]).i32(), 7);
        assert_eq!(e.call(0x00ec_7d40, &args![i32::MIN]).i32(), i32::MIN);
    }

    #[test]
    fn inittime_stores_the_file_time_as_one_64_bit_value() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(GET_SYSTEM_TIME_AS_FILE_TIME, |e, a| {
            e.mem.set_u32(a[0], 0xffff_fff0);
            e.mem.set_u32(a[0] + 4, 0x0000_0002);
            Ret::default()
        });
        // __allmul(high, 0, 0, 1) = high << 32 with a low word of 0x20, so the add carries.
        e.register(ALLMUL, |_, a| Ret {
            eax: 0x20,
            edx: a[0],
            st0: 0.0,
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_7d95, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, ALLMUL), vec![vec![2, 0, 0, 1]]);
        assert_eq!(e.mem.u32(START_TIME_0126FDA0), 0x10);
        assert_eq!(e.mem.u32(START_TIME_0126FDA0 + 4), 3);
    }

    /// Doubles for `_strnicmp_l`'s callees. The `_LocaleUpdate` double writes a `locinfo` whose
    /// `LC_CTYPE` handle is `handle`, the thread data and the updated byte.
    fn locale_setup(e: &mut Engine, handle: u32, updated: bool) -> (u32, u32) {
        let locinfo = e.mem.alloc(0x20);
        e.mem.set_u32(locinfo + 0x14, handle);
        let thread_data = e.mem.alloc(0x80);
        e.mem.set_u32(thread_data + 0x70, 0xff);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], locinfo);
            e.mem.set_u32(a[0] + 8, thread_data);
            e.mem.set_u8(a[0] + 0xc, updated as u8);
            a[0].into_ret()
        });
        let error = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| error.into_ret());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        e.register(TOLOWER_L, |_, a| (a[0] | 0x20).into_ret());
        e.register(ASCII_STRNICMP, |_, a| (a[2] + 100).into_ret());
        (thread_data, error)
    }

    #[test]
    fn strnicmp_l_zero_count_returns_zero_without_calls() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_7dce, &args![0u32, 0u32, 0u32, 0u32]).i32(), 0);
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn strnicmp_l_bad_arguments_report_and_return_the_maximum() {
        for (null_first, null_second, count) in [
            (true, false, 4u32),
            (false, true, 4),
            (false, false, 0x8000_0000),
        ] {
            let mut e = Engine::new();
            let (thread_data, error) = locale_setup(&mut e, 1, true);
            let text = e.mem.alloc(8);
            e.call_log = Some(vec![]);
            let first = if null_first { 0 } else { text };
            let second = if null_second { 0 } else { text };
            let result = e.call(0x00ec_7dce, &args![first, second, count, 0x55u32]);
            assert_eq!(result.i32(), 0x7fff_ffff);
            assert_eq!(e.mem.u32(error), 0x16);
            assert_eq!(calls_to(&e, INVALID_PARAMETER), vec![vec![0; 5]]);
            assert_eq!(e.mem.u32(thread_data + 0x70), 0xfd, "bit 1 cleared");
            assert_eq!(calls_to(&e, LOCALE_UPDATE)[0][1], 0x55);
        }
    }

    #[test]
    fn strnicmp_l_without_a_ctype_handle_uses_the_ascii_compare() {
        let mut e = Engine::new();
        let (thread_data, _) = locale_setup(&mut e, 0, false);
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_7dce, &args![a, b, 3u32, 0u32]).i32(), 103);
        assert_eq!(calls_to(&e, ASCII_STRNICMP), vec![vec![a, b, 3]]);
        assert_eq!(e.mem.u32(thread_data + 0x70), 0xff, "not updated: kept");
    }

    #[test]
    fn strnicmp_l_lowers_pairs_until_count_zero_or_difference() {
        let mut e = Engine::new();
        locale_setup(&mut e, 1, false);
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.write(a, b"ABCD\0");
        e.mem.write(b, b"abXd\0");
        // The difference at the third pair: 'c' - 'x'.
        assert_eq!(
            e.call(0x00ec_7dce, &args![a, b, 8u32, 0u32]).i32(),
            b'c' as i32 - b'x' as i32
        );
        // Two pairs only: equal.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_7dce, &args![a, b, 2u32, 0u32]).i32(), 0);
        assert_eq!(calls_to(&e, TOLOWER_L).len(), 4);
        // A 0 in the first string stops after its pair (identity lowering here).
        e.mem.write(a, b"ab\0x");
        e.mem.write(b, b"ab\0y");
        e.register(TOLOWER_L, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_7dce, &args![a, b, 8u32, 0u32]).i32(), 0);
        assert_eq!(calls_to(&e, TOLOWER_L).len(), 6);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_move_zero_length_accepts_null_and_nonzero_moves_overlap() {
        // Three generated native vectors: FalloutNV.exe 1.4.0.525,
        // SHA256 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // Ryzen AI 9 HX 370, FCW027F/MXCSR1F80; all returned zero without fault.
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec7ce3, &[0, 0, 0, 0]).u32(), 0);
        let bytes = e.mem.alloc(8);
        e.mem.write(bytes, b"abcdefgh");
        assert_eq!(e.call(0x00ec7ce3, &[bytes + 2, 6, bytes, 6]).u32(), 0);
        assert_eq!(e.mem.bytes(bytes, 8), b"ababcdef");
        e.mem.write(bytes, b"abcdefgh");
        assert_eq!(e.call(0x00ec7ce3, &[bytes, 8, bytes + 2, 6]).u32(), 0);
        assert_eq!(e.mem.bytes(bytes, 8), b"cdefghgh");
    }

    #[test]
    fn checked_move_errors_set_thread_error_before_handler_and_preserve_data() {
        for (null_destination, null_source, capacity, expected) in [
            (true, false, 8u32, 0x16u32),
            (false, true, 0, 0x16),
            (false, false, 3, 0x22),
        ] {
            let mut e = Engine::new();
            let bytes = e.mem.alloc(8);
            e.mem.write(bytes, b"abcdefgh");
            let error = e.mem.alloc(4);
            e.register_double(0x00ec85e3, move |_, a| {
                assert!(a.is_empty());
                error.into_ret()
            });
            e.register_double(0x00ec7c30, move |e, a| {
                assert_eq!(a, &[0; 5]);
                assert_eq!(e.mem.u32(error), expected);
                Ret::default()
            });
            let destination = if null_destination { 0 } else { bytes };
            let source = if null_source { 0 } else { bytes + 4 };
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00ec7ce3, &[destination, capacity, source, 4])
                    .u32(),
                expected
            );
            assert_eq!(e.mem.bytes(bytes, 8), b"abcdefgh");
            assert!(e
                .call_log
                .unwrap()
                .iter()
                .any(|(address, _)| *address == 0x00ec7c30));
        }
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn clock_divides_the_ticks_since_the_start_by_10000() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        // FILETIME 0x0000_0002_0000_0010; start time 0x0000_0001_FFFF_FFF0.
        e.register(GET_SYSTEM_TIME_AS_FILE_TIME, |e, a| {
            e.mem.set_u32(a[0], 0x10);
            e.mem.set_u32(a[0] + 4, 2);
            Ret::default()
        });
        e.register(ALLMUL, |_, a| {
            let product = (u64::from(a[0]) | u64::from(a[1]) << 32)
                .wrapping_mul(u64::from(a[2]) | u64::from(a[3]) << 32);
            product.into_ret()
        });
        e.register(AULLDIV, |_, a| {
            let dividend = u64::from(a[0]) | u64::from(a[1]) << 32;
            let divisor = u64::from(a[2]) | u64::from(a[3]) << 32;
            (dividend / divisor).into_ret()
        });
        e.mem.set_u32(START_TIME_0126FDA0, 0xffff_fff0);
        e.mem.set_u32(START_TIME_0126FDA0 + 4, 1);
        e.call_log = Some(vec![]);
        // (0x2_0000_0000 - 0x1_FFFF_FFF0) + 0x10 = 0x20 ticks: 0 ms.
        assert_eq!(e.call(0x00ec_7d50, &args![]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let multiply = log.iter().find(|(a, _)| *a == ALLMUL).unwrap();
        assert_eq!(multiply.1, vec![2, 0, 0, 1]);
        let divide = log.iter().find(|(a, _)| *a == AULLDIV).unwrap();
        assert_eq!(divide.1, vec![0x20, 0, 10_000, 0]);
        // A start one second (10,000,000 ticks) earlier: 1000 ms.
        let start = 0x2_0000_0010u64 - 10_000_000;
        e.mem.set_u32(START_TIME_0126FDA0, start as u32);
        e.mem.set_u32(START_TIME_0126FDA0 + 4, (start >> 32) as u32);
        assert_eq!(e.call(0x00ec_7d50, &args![]).u32(), 1000);
    }
}
