//! `Havok SDK/hkpsolvercheck.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The product key text in the exe image (`010c3a50`), a string of the form
/// `0x<hex>-0x<hex>:<date>.<module>.<name>`.
const KEY_TEXT: u32 = 0x010c_3a50;
/// The error reporter (`00cb1750`, cdecl, one message pointer).
const REPORT_ERROR: u32 = 0x00cb_1750;
/// The "key code is invalid" message (`010da1c8`).
const INVALID_MESSAGE: u32 = 0x010d_a1c8;
/// The "key code has expired" message (`010da160`).
const EXPIRED_MESSAGE: u32 = 0x010d_a160;
/// `00e29910` (cdecl, no arguments): returns the value compared with the key's date (shifted
/// right by 8 before the comparison). Its meaning is unconfirmed.
const CLOCK_VALUE: u32 = 0x00e2_9910;
/// The constant the key values are XORed with before the 31-bit mask.
const KEY_MASK_XOR: u32 = 0xf2e6_ef51;

/// The value of one hexadecimal digit, or -1 for any other byte.
fn hex_value(c: u8) -> i32 {
    if c.wrapping_sub(0x30) <= 9 {
        i32::from(c as i8) - 0x30
    } else if c.wrapping_sub(0x41) <= 5 {
        i32::from(c as i8) - 0x37
    } else if c.wrapping_sub(0x61) <= 5 {
        i32::from(c as i8) - 0x57
    } else {
        -1
    }
}

/// Address of the first NUL or `.` at or after `from`, or `None` when the scan wraps the
/// address space (the exe tests the pointer for zero after each step).
fn find_stop(e: &Engine, from: u32) -> Option<u32> {
    let mut p = from;
    loop {
        let c = e.mem.u8(p);
        if c == 0 || c == b'.' {
            return Some(p);
        }
        p = p.wrapping_add(1);
        if p == 0 {
            return None;
        }
    }
}

// Translated from 00df22f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpProcessFlyingColors` (Xbox PDB), cdecl with `out` (a byte pointer, set to 0 first and to
/// 1 on success). It parses the product key text at `010c3a50`: the hexadecimal digits that
/// follow the first two characters (`first`), then the digits after the next two characters
/// (`second`). It finds the text after the second `.`; if `first` has its top bit set, `first`
/// XOR `f2e6ef51` masked to 31 bits is a date compared with `00e29910() >> 8`: past the date or
/// more than 0x96c99 before it reports the expired message and returns 2, within 0xd2f of it
/// returns 3, otherwise 0; with the bit clear the characters of the final text (from its
/// second character, times 0x17 each) are hashed, XORed and masked, and must equal `second`.
/// A malformed or mismatching key reports the invalid message and returns 1.
pub fn hkp_process_flying_colors(e: &mut Engine, out: Ptr) -> u32 {
    e.mem.set_u8(out.addr(), 0);
    let mut first = u32::MAX;
    let mut second = u32::MAX;
    let mut value = hex_value(e.mem.u8(KEY_TEXT + 2));
    let mut at = KEY_TEXT + 3;
    while value >= 0 {
        first = (first << 4).wrapping_add(value as u32);
        value = hex_value(e.mem.u8(at));
        at += 1;
    }
    value = hex_value(e.mem.u8(at + 2));
    at += 3;
    while value >= 0 {
        second = (second << 4).wrapping_add(value as u32);
        value = hex_value(e.mem.u8(at));
        at += 1;
    }
    let negative = first >> 31 != 0;
    let name = find_stop(e, KEY_TEXT)
        .and_then(|p| find_stop(e, p.wrapping_add(1)))
        .map(|p| p + 1);
    let Some(name) = name else {
        e.call(REPORT_ERROR, &args![INVALID_MESSAGE]);
        return 1;
    };
    if negative {
        let date = (first ^ KEY_MASK_XOR) & 0x7fff_ffff;
        let now = (e.call(CLOCK_VALUE, &args![]).u32() as i32) >> 8;
        let difference = (date as i32).wrapping_sub(now);
        if (date as i32) < now || difference > 0x96c99 {
            e.call(REPORT_ERROR, &args![EXPIRED_MESSAGE]);
            return 2;
        }
        e.mem.set_u8(out.addr(), 1);
        return if difference < 0xd2f { 3 } else { 0 };
    }
    let mut hash = 0u32;
    if e.mem.u8(name) != 0 {
        let mut q = name;
        loop {
            let c = e.mem.u8(q + 1);
            hash = hash.wrapping_mul(0x17);
            q += 1;
            hash = hash.wrapping_add(i32::from(c as i8) as u32);
            if c == 0 {
                break;
            }
        }
    }
    if second != ((hash ^ KEY_MASK_XOR) & 0x7fff_ffff) {
        e.call(REPORT_ERROR, &args![INVALID_MESSAGE]);
        return 1;
    }
    e.mem.set_u8(out.addr(), 1);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00df_22f0, hkp_process_flying_colors(Ptr) -> u32)]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn rig(text: &str, clock: u32) -> (Engine, Rc<RefCell<Vec<u32>>>, u32) {
        let mut e = test_engine();
        let mut bytes = text.as_bytes().to_vec();
        bytes.push(0);
        e.mem.write(KEY_TEXT, &bytes);
        let reports = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&reports);
        e.register_double(REPORT_ERROR, move |_, a| {
            sink.borrow_mut().push(a[0]);
            Ret::default()
        });
        e.register_double(CLOCK_VALUE, move |_, _| Ret {
            eax: clock,
            ..Ret::default()
        });
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0x55);
        (e, reports, out)
    }

    #[test]
    fn a_matching_hash_accepts_the_key() {
        // The text "AB" hashes to 'B' * 0x17 = 0x5ee, XOR f2e6ef51 = 72e6eabf after the mask.
        let (mut e, reports, out) = rig("0x00000001-0x72e6eabf:2011-10-29.Physics.AB", 0);
        assert_eq!(e.call(0x00df_22f0, &args![out]).u32(), 0);
        assert_eq!(e.mem.u8(out), 1);
        assert!(reports.borrow().is_empty());
        // A different hash is an invalid key.
        let (mut e, reports, out) = rig("0x00000001-0x72e6eabe:2011-10-29.Physics.AB", 0);
        assert_eq!(e.call(0x00df_22f0, &args![out]).u32(), 1);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(*reports.borrow(), vec![INVALID_MESSAGE]);
    }

    #[test]
    fn a_text_without_two_dots_is_invalid() {
        let (mut e, reports, out) = rig("0x00000001-0x72e6eabf:2011-10-29.Physics", 0);
        // The second scan stops at the terminator, so the name starts past it (zero bytes
        // here): its hash is zero and does not match.
        assert_eq!(e.call(0x00df_22f0, &args![out]).u32(), 1);
        assert_eq!(*reports.borrow(), vec![INVALID_MESSAGE]);
    }

    #[test]
    fn a_dated_key_is_checked_against_the_clock() {
        // 0xf2e6cf51 XOR f2e6ef51 is 0x2000 = 8192 once masked.
        let text = "0xf2e6cf51-0x0:2011-10-29.Physics.AB";
        let (mut e, reports, out) = rig(text, 8092 << 8);
        assert_eq!(e.call(0x00df_22f0, &args![out]).u32(), 3);
        assert_eq!(e.mem.u8(out), 1);
        let (mut e, _, out) = rig(text, 3000 << 8);
        assert_eq!(e.call(0x00df_22f0, &args![out]).u32(), 0);
        assert_eq!(e.mem.u8(out), 1);
        assert!(reports.borrow().is_empty());
        let (mut e, reports, out) = rig(text, 8193 << 8);
        assert_eq!(e.call(0x00df_22f0, &args![out]).u32(), 2);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(*reports.borrow(), vec![EXPIRED_MESSAGE]);
    }
}
