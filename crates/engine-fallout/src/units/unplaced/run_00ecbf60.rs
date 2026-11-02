//! `(unplaced)/run_00ecbf60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`, thiscall: the locale or null): fills the
/// 0x10-byte local with the locale info (+0), the thread data (+8) and an "updated" byte
/// (+0xc).
const LOCALE_UPDATE_CTOR: u32 = 0x00ec_660e;
/// `_errno` (`00ec85e3`) and `_invalid_parameter` (`00ec7c30`, cdecl, five words).
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
/// `_towlower_l` (`00edd3f0`, cdecl: the character, the locale update local).
const TOWLOWER_L: u32 = 0x00ed_d3f0;
/// `__locale_changed` (`0126fdf4`).
const LOCALE_CHANGED: u32 = 0x0126_fdf4;
/// `EINVAL`.
const EINVAL: u32 = 0x16;

/// `'A'..='Z'` lowered by ASCII, as the inlined loops do (`ADD EAX,0x20` on the word).
fn ascii_lower(c: u16) -> u16 {
    if (0x41..=0x5a).contains(&c) {
        c + 0x20
    } else {
        c
    }
}

/// The invalid-argument exit: `errno = EINVAL`, `_invalid_parameter(0, 0, 0, 0, 0)`.
fn invalid_argument(e: &mut Engine) {
    let errno = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(errno, EINVAL);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

/// The plain-ASCII comparison loop of both functions: characters lowered and compared until
/// the first string's character is 0 or they differ; answers their difference.
fn compare_ascii(e: &Engine, first: u32, second: u32) -> i32 {
    let (mut a, mut b) = (first, second);
    loop {
        let x = ascii_lower(e.mem.u16(a));
        let y = ascii_lower(e.mem.u16(b));
        a = a.wrapping_add(2);
        b = b.wrapping_add(2);
        if x == 0 || x != y {
            return i32::from(x) - i32::from(y);
        }
    }
}

// Translated from 00ecbf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_wcsicmp_l` (Ghidra library match; cdecl): with a `_LocaleUpdate` local for `locale`.
/// A null string sets `errno` to `EINVAL`, calls `_invalid_parameter` and answers
/// `0x7fffffff`. Without an LC_CTYPE handle (locale info +0x14) the characters are lowered
/// by ASCII, otherwise by `_towlower_l`; they are compared until the first string ends or
/// they differ, answering the difference. On every exit an updated thread (+0xc set) has bit
/// 1 of its word +0x70 cleared.
pub fn wcsicmp_l(e: &mut Engine, first: Ptr, second: Ptr, locale: u32) -> i32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE_CTOR, &args![update, locale]);
        let restore = |e: &mut Engine| {
            if e.mem.u8(update + 0xc) != 0 {
                let ptd = e.mem.u32(update + 8);
                let flags = e.mem.u32(ptd + 0x70);
                e.mem.set_u32(ptd + 0x70, flags & 0xffff_fffd);
            }
        };
        if first.addr() == 0 || second.addr() == 0 {
            invalid_argument(e);
            restore(e);
            return 0x7fff_ffff;
        }
        let info = e.mem.u32(update);
        let result = if e.mem.u32(info + 0x14) == 0 {
            compare_ascii(e, first.addr(), second.addr())
        } else {
            let (mut a, mut b) = (first.addr(), second.addr());
            loop {
                let c = u32::from(e.mem.u16(a));
                let x = e.call(TOWLOWER_L, &args![c, update]).u16();
                let c = u32::from(e.mem.u16(b));
                a = a.wrapping_add(2);
                let y = e.call(TOWLOWER_L, &args![c, update]).u16();
                b = b.wrapping_add(2);
                if x == 0 || x != y {
                    break i32::from(x) - i32::from(y);
                }
            }
        };
        restore(e);
        result
    })
}

// Translated from 00ecc071 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_wcsicmp` (Ghidra library match; cdecl): with a changed locale (`0126fdf4`) the answer
/// of `_wcsicmp_l` with no locale; otherwise a null string is invalid (`EINVAL`,
/// `_invalid_parameter`, `0x7fffffff`) and the strings are compared lowered by ASCII.
pub fn wcsicmp(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    if e.mem.u32(LOCALE_CHANGED) != 0 {
        return wcsicmp_l(e, first, second, 0);
    }
    if first.addr() == 0 || second.addr() == 0 {
        invalid_argument(e);
        return 0x7fff_ffff;
    }
    compare_ascii(e, first.addr(), second.addr())
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_bf60, wcsicmp_l(Ptr, Ptr, u32) -> i32),
        entry!(0x00ec_c071, wcsicmp(Ptr, Ptr) -> i32),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    fn wide(e: &mut Engine, s: &str) -> u32 {
        let units: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
        let p = e.mem.alloc(2 * units.len() as u32);
        for (i, u) in units.iter().enumerate() {
            e.mem.set_u16(p + 2 * i as u32, *u);
        }
        p
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(LOCALE_CHANGED, 4);
        let errno = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        e
    }

    #[test]
    fn ascii_comparison_and_null_strings() {
        let mut e = engine();
        let a = wide(&mut e, "Hello");
        let b = wide(&mut e, "hELLO");
        let c = wide(&mut e, "help");
        assert_eq!(e.call(0x00ec_c071, &args![a, b]).i32(), 0);
        assert_eq!(
            e.call(0x00ec_c071, &args![a, c]).i32(),
            i32::from(b'l') - i32::from(b'p')
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_c071, &args![a, 0u32]).i32(), 0x7fff_ffff);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(INVALID_PARAMETER, vec![0, 0, 0, 0, 0])));
    }

    #[test]
    fn locale_comparison_uses_towlower_and_restores_the_thread_flag() {
        let mut e = engine();
        let info = e.mem.alloc(0x20);
        let ptd = e.mem.alloc(0x80);
        e.mem.set_u32(ptd + 0x70, 0xff);
        e.register_double(LOCALE_UPDATE_CTOR, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 8, ptd);
            e.mem.set_u8(a[0] + 0xc, 1);
            Ret::default()
        });
        e.register(TOWLOWER_L, |_, a| (a[0] | 0x20).into_ret());
        e.mem.set_u32(LOCALE_CHANGED, 1);
        let a = wide(&mut e, "ABC");
        let b = wide(&mut e, "abd");
        // No LC_CTYPE handle: ASCII.
        assert_eq!(e.call(0x00ec_c071, &args![a, b]).i32(), -1);
        assert_eq!(e.mem.u32(ptd + 0x70), 0xfd);
        // A handle: through _towlower_l.
        e.mem.set_u32(info + 0x14, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_bf60, &args![a, b, 0u32]).i32(), -1);
        let lowered = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|c| c.0 == TOWLOWER_L)
            .count();
        assert_eq!(lowered, 6);
        e.mem.set_u32(ptd + 0x70, 0xff);
        assert_eq!(
            e.call(0x00ec_bf60, &args![0u32, b, 0u32]).i32(),
            0x7fff_ffff
        );
        assert_eq!(e.mem.u32(ptd + 0x70), 0xfd);
    }
}
