//! `Havok SDK/hkaprocessflyingcolors.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The one function checks the product key string the engine carries in its data (`010c3aa0`).
//! Nothing here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// The product key string (`010c3aa0`): `0x`, a hexadecimal number, `-0x`, a second number,
/// then text with `.`-separated parts.
const KEY_STRING: u32 = 0x010c_3aa0;
/// The two messages the check reports (`010ecf68` invalid, `010ecf00` expired) through the
/// reporting routine `00cb1750` (cdecl `(message)`).
const REPORT: u32 = 0x00cb_1750;
const MESSAGE_INVALID: u32 = 0x010e_cf68;
const MESSAGE_EXPIRED: u32 = 0x010e_cf00;
/// The routine that answers the current date (`00e29910`, no arguments); the check compares the
/// key's date with it shifted right by 8.
const CURRENT_DATE: u32 = 0x00e2_9910;

/// The value of the hexadecimal digit `character` (`0`-`9`, `A`-`F`, `a`-`f`), or `-1`.
fn hex_digit(character: u8) -> i32 {
    match character {
        b'0'..=b'9' => i32::from(character) - 0x30,
        b'A'..=b'F' => i32::from(character) - 0x37,
        b'a'..=b'f' => i32::from(character) - 0x57,
        _ => -1,
    }
}

/// Moves `cursor` to the first NUL or `.` at or after it, one past the end of memory counting as
/// a failure (`None`).
fn find_part_end(e: &Engine, cursor: u32) -> Option<u32> {
    let mut cursor = cursor;
    loop {
        let character = e.mem.u8(cursor);
        if character == 0 || character == b'.' {
            return Some(cursor);
        }
        cursor = cursor.wrapping_add(1);
        if cursor == 0 {
            return None;
        }
    }
}

// Translated from 00e63c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaProcessFlyingColors` (Xbox PDB), cdecl `(valid)`: clears the byte at `valid` and checks
/// the key string. Two hexadecimal numbers are read from it (`high` after the leading `0x`, `low`
/// after the following `-0x`). The text after its second `.` is looked at next (anything else
/// reports the key invalid and returns `1`). With the top bit of `high` clear, a hash of that text
/// (each character from the second one on and a final zero: `hash * 23 + character`, then
/// `^ 0xd4efe34a`, `& 0x7fffffff`) must equal `low`, else the key is invalid; the byte is set
/// and `0` returned. With the top bit set `high` is a date (`^ 0xd4efe34a`, `& 0x7fffffff`)
/// compared with the current date shifted right by 8: before it or more than `0x96c99` days
/// after reports the key expired and returns `2`; less than `0xd2f` days after sets the byte and
/// returns `3` (the key is about to expire); otherwise the byte is set and `0` returned.
pub fn hka_process_flying_colors(e: &mut Engine, valid: Ptr) -> u32 {
    e.mem.set_u8(valid.addr(), 0);
    let mut high = 0xffff_ffffu32;
    let mut low = 0xffff_ffffu32;
    let mut digit = hex_digit(e.mem.u8(KEY_STRING + 2));
    let mut cursor = KEY_STRING + 3;
    if digit >= 0 {
        loop {
            high = (high << 4).wrapping_add(digit as u32);
            digit = hex_digit(e.mem.u8(cursor));
            cursor += 1;
            if digit < 0 {
                break;
            }
        }
    }
    cursor += 2;
    digit = hex_digit(e.mem.u8(cursor));
    cursor += 1;
    if digit >= 0 {
        loop {
            low = (low << 4).wrapping_add(digit as u32);
            digit = hex_digit(e.mem.u8(cursor));
            cursor += 1;
            if digit < 0 {
                break;
            }
        }
    }
    let dated = high >> 31 != 0;
    let invalid = |e: &mut Engine| {
        e.call(REPORT, &args![MESSAGE_INVALID]);
        1
    };
    let Some(first) = find_part_end(e, KEY_STRING) else {
        return invalid(e);
    };
    let start = first.wrapping_add(1);
    if start == 0 {
        return invalid(e);
    }
    let Some(second) = find_part_end(e, start) else {
        return invalid(e);
    };
    let text = second + 1;
    if dated {
        let date = (high ^ 0xd4ef_e34a) & 0x7fff_ffff;
        let today = (e.call(CURRENT_DATE, &args![]).u32() as i32) >> 8;
        let left = (date as i32).wrapping_sub(today);
        if (date as i32) < today || left > 0x96c99 {
            e.call(REPORT, &args![MESSAGE_EXPIRED]);
            return 2;
        }
        if left < 0xd2f {
            e.mem.set_u8(valid.addr(), 1);
            return 3;
        }
    } else {
        let mut hash = 0u32;
        let mut at = text;
        if e.mem.u8(at) != 0 {
            loop {
                let character = e.mem.u8(at + 1);
                hash = hash
                    .wrapping_mul(0x17)
                    .wrapping_add(i32::from(character as i8) as u32);
                at += 1;
                if character == 0 {
                    break;
                }
            }
        }
        if low != ((hash ^ 0xd4ef_e34a) & 0x7fff_ffff) {
            return invalid(e);
        }
    }
    e.mem.set_u8(valid.addr(), 1);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e6_3c60, hka_process_flying_colors(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<u32>>>;

    /// An engine with the key `key` in memory, a reporting double that logs the message address
    /// and a date routine that answers `date << 8`.
    fn fixture(key: &str, date: u32) -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_cstr(KEY_STRING, key.as_bytes());
        let log: Log = Rc::default();
        let l = Rc::clone(&log);
        e.register_double(REPORT, move |_, a| {
            l.borrow_mut().push(a[0]);
            Ret::default()
        });
        e.register_double(CURRENT_DATE, move |_, _| Ret {
            eax: date << 8,
            ..Ret::default()
        });
        (e, log)
    }

    /// Runs the check, with the answer and the byte it sets.
    fn check(e: &mut Engine) -> (u32, u8) {
        let valid = e.mem.alloc(4);
        e.mem.set_u8(valid, 0xee);
        let got = e.call(0x00e6_3c60, &args![valid]).u32();
        (got, e.mem.u8(valid))
    }

    /// The key whose second number is the hash of "AB": `'B' * 23 + 0` is 1518.
    const HASHED: &str = "0x12345678-0x54efe6a4:2011-10-29.Animation.AB";

    #[test]
    fn a_key_with_the_right_hash_is_accepted() {
        let (mut e, log) = fixture(HASHED, 0);
        assert_eq!(check(&mut e), (0, 1));
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn a_key_with_the_wrong_hash_is_invalid() {
        let (mut e, log) = fixture("0x12345678-0x54efe6a5:2011-10-29.Animation.AB", 0);
        assert_eq!(check(&mut e), (1, 0));
        assert_eq!(*log.borrow(), [MESSAGE_INVALID]);
    }

    #[test]
    fn a_key_without_two_dots_hashes_what_follows_the_terminator() {
        // One part only: the text starts after the terminator, where memory is zero, so the hash
        // is that of the empty text (0 ^ 0xd4efe34a).
        let (mut e, _log) = fixture("0x12345678-0x54efe34a:none", 0);
        assert_eq!(check(&mut e), (0, 1));
    }

    #[test]
    fn a_dated_key_is_checked_against_the_current_date() {
        let key =
            |wanted: u32| format!("0x{:08x}-0x1:2011-10-29.Animation.AB", 0xd4ef_e34a ^ wanted);
        // 5000 days to run: fine.
        let (mut e, log) = fixture(&key(1000 + 5000), 1000);
        assert_eq!(check(&mut e), (0, 1));
        assert!(log.borrow().is_empty());
        // 100 days: valid, but flagged as about to expire (3).
        let (mut e, log) = fixture(&key(1000 + 100), 1000);
        assert_eq!(check(&mut e), (3, 1));
        assert!(log.borrow().is_empty());
        // The boundaries: 0xd2f is still fine, 0xd2e is not.
        let (mut e, _) = fixture(&key(1000 + 0xd2f), 1000);
        assert_eq!(check(&mut e), (0, 1));
        let (mut e, _) = fixture(&key(1000 + 0xd2e), 1000);
        assert_eq!(check(&mut e), (3, 1));
        // Already past: expired; the day itself is not (it is about to expire).
        let (mut e, log) = fixture(&key(999), 1000);
        assert_eq!(check(&mut e), (2, 0));
        assert_eq!(*log.borrow(), [MESSAGE_EXPIRED]);
        let (mut e, _) = fixture(&key(1000), 1000);
        assert_eq!(check(&mut e), (3, 1));
        // Too far ahead: expired as well, but 0x96c99 days is still allowed.
        let (mut e, _) = fixture(&key(1000 + 0x96c99), 1000);
        assert_eq!(check(&mut e), (0, 1));
        let (mut e, log) = fixture(&key(1000 + 0x96c9a), 1000);
        assert_eq!(check(&mut e), (2, 0));
        assert_eq!(*log.borrow(), [MESSAGE_EXPIRED]);
    }

    #[test]
    fn the_key_digits_accept_both_cases() {
        // 0xAbCdEf01 has the top bit set, so it is read as a date: (0xAbCdEf01 ^ 0xd4efe34a)
        // & 0x7fffffff is just a large date, reported as expired.
        let (mut e, log) = fixture("0xAbCdEf01-0x1:2011-10-29.Animation.AB", 0);
        assert_eq!(check(&mut e), (2, 0));
        assert_eq!(*log.borrow(), [MESSAGE_EXPIRED]);
    }
}
