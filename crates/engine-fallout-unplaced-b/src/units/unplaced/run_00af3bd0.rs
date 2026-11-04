//! `(unplaced)/run_00af3bd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A file-name wildcard matcher the engine map does not name: `?`, `*`,
//! `[...]` sets (with `^` negation, `a-z` ranges and `\` escapes) and `\`
//! escapes, optionally case-insensitive, on names first reduced to the
//! path parts the option word at `011f804c` selects:
//!
//! - bit 0: compare bytes as they are (otherwise through `tolower`);
//! - bit 1: keep the drive and directory, bit 2: keep the extension (with
//!   neither, the name is copied whole);
//! - bit 3: skip leading spaces.

#[allow(unused_imports)]
use crate::prelude::*;

/// The matcher's option word.
const OPTIONS: u32 = 0x011f_804c;
/// `00eca7f4`: `tolower` (cdecl).
const TO_LOWER: u32 = 0x00ec_a7f4;
/// `00ec65a6`: `strcpy_s(dst, size, src)` (cdecl).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `00ec7f1e`: `_splitpath_s` (cdecl, nine words).
const SPLITPATH_S: u32 = 0x00ec_7f1e;
/// `00ec825b`: `_makepath_s` (cdecl, six words).
const MAKEPATH_S: u32 = 0x00ec_825b;
/// The size the names are reduced into (`MAX_PATH`).
const NAME_SIZE: u32 = 0x104;
/// The two name buffers of [`fn_00af3bd0`] (0x108 bytes each on its stack).
const NAME_BUFFER: u32 = 0x108;
/// `_splitpath_s` component buffers.
const DRIVE_SIZE: u32 = 3;
const PART_SIZE: u32 = 0x100;

fn options(e: &Engine) -> u32 {
    e.global(OPTIONS)
}

/// The byte as compared: itself with option bit 0, else `tolower` of it
/// (sign-extended), low byte.
fn fold(e: &mut Engine, c: u8) -> u8 {
    if options(e) & 1 != 0 {
        c
    } else {
        e.call(TO_LOWER, &args![c as i8 as i32]).u8()
    }
}

// Translated from 00af3bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `text` matches `pattern` (1) or not (0): both are first reduced
/// by [`fn_00af4070`] into stack buffers, then matched by [`fn_00af3c40`],
/// which must return exactly 1.
// Translation incomplete: 00af3bd0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00af3bd0(e: &mut Engine, text: Ptr, pattern: Ptr) -> u32 {
    e.with_stack(NAME_BUFFER, |e, text_name| {
        fn_00af4070(e, text_name, text);
        e.with_stack(NAME_BUFFER, |e, pattern_name| {
            fn_00af4070(e, pattern_name, pattern);
            (fn_00af3c40(e, text_name, pattern_name) == 1) as u32
        })
    })
}

// Translated from 00af3c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Matches `text` against `pattern` recursively: 1 for a match, 0 for
/// none, 2 when a `*` ran out of text. `?` tests the text pointer (not the
/// byte) before advancing; `*` at the pattern's end matches everything;
/// in a set, a byte is compared zero-extended with the text's byte
/// sign-extended (as the exe does), and a range's upper bound is compared
/// as signed bytes.
pub fn fn_00af3c40(e: &mut Engine, text: Ptr, pattern: Ptr) -> i32 {
    let mut s = text.addr();
    let mut p = pattern.addr();
    let mut c = e.mem.u8(p);
    p = p.wrapping_add(1);
    if c == 0 {
        return (e.mem.u8(s) == 0) as i32;
    }
    match c {
        b'?' => {
            if s == 0 {
                0
            } else {
                fn_00af3c40(e, Ptr::new(s.wrapping_add(1)), Ptr::new(p))
            }
        }
        b'*' => {
            if e.mem.u8(p) == 0 {
                return 1;
            }
            while e.mem.u8(s) != 0 {
                let r = fn_00af3c40(e, Ptr::new(s), Ptr::new(p));
                if r != 0 {
                    return r;
                }
                s = s.wrapping_add(1);
            }
            2
        }
        b'[' => match_set(e, s, p),
        _ => {
            if c == b'\\' {
                c = e.mem.u8(p);
                p = p.wrapping_add(1);
                if c == 0 {
                    return 0;
                }
            }
            let wanted = fold(e, c);
            let got = e.mem.u8(s);
            let got = fold(e, got);
            if wanted == got {
                fn_00af3c40(e, Ptr::new(s.wrapping_add(1)), Ptr::new(p))
            } else {
                0
            }
        }
    }
}

/// The `[` case of [`fn_00af3c40`]; `p` is just past the `[`.
fn match_set(e: &mut Engine, s: u32, mut p: u32) -> i32 {
    if e.mem.u8(s) == 0 {
        return 0;
    }
    let negate = e.mem.u8(p) == b'^';
    if negate {
        p = p.wrapping_add(1);
    }
    // Find the closing `]`, skipping escaped bytes.
    let mut end = p;
    let mut escaped = false;
    loop {
        let ch = e.mem.u8(end);
        if ch == 0 {
            break;
        }
        if escaped {
            escaped = false;
        } else if ch == b'\\' {
            escaped = true;
        } else if ch == b']' {
            break;
        }
        end = end.wrapping_add(1);
    }
    if e.mem.u8(end) != b']' {
        return 0;
    }
    let after = end.wrapping_add(1);
    let mut low: u8 = 0;
    let mut escaped = e.mem.u8(p) == b'-';
    while p < end {
        let ch = e.mem.u8(p);
        if !escaped && ch == b'\\' {
            escaped = true;
        } else if !escaped && ch == b'-' {
            low = e.mem.u8(p.wrapping_sub(1));
        } else {
            let got = e.mem.u8(s);
            let got = fold(e, got);
            if e.mem.u8(p.wrapping_add(1)) != b'-' {
                if low == 0 {
                    low = e.mem.u8(p);
                }
                while (low as i8) <= (e.mem.u8(p) as i8) {
                    let candidate = fold(e, low);
                    if i32::from(candidate) == i32::from(got as i8) {
                        return if negate {
                            0
                        } else {
                            fn_00af3c40(e, Ptr::new(s.wrapping_add(1)), Ptr::new(after))
                        };
                    }
                    low = low.wrapping_add(1);
                }
            }
            escaped = false;
            low = 0;
        }
        p = p.wrapping_add(1);
    }
    if negate {
        fn_00af3c40(e, Ptr::new(s.wrapping_add(1)), Ptr::new(after))
    } else {
        0
    }
}

// Translated from 00af3fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `text` contains an unescaped wildcard byte (`?`, `[`, `]`, `*`
/// or `^`); a `\` followed by any byte skips that byte.
pub fn fn_00af3fd0(e: &mut Engine, text: Ptr) -> bool {
    let mut s = text.addr();
    loop {
        let c = e.mem.u8(s);
        if c == 0 {
            return false;
        }
        if c == b'\\' && e.mem.u8(s.wrapping_add(1)) != 0 {
            s = s.wrapping_add(1);
        } else if matches!(c, b'?' | b'[' | b']' | b'*' | b'^') {
            return true;
        }
        s = s.wrapping_add(1);
    }
}

// Translated from 00af4070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reduces the name `source` into `destination` (0x104 bytes): leading
/// spaces skipped with option bit 3; copied whole (`strcpy_s`) unless
/// option bit 1 or 2 is set, else split (`_splitpath_s`) and rebuilt
/// (`_makepath_s`) without the drive and directory (bit 1 clear) and
/// without the extension (bit 2 clear).
// Translation incomplete: 00af4070 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00af4070(e: &mut Engine, destination: Ptr, source: Ptr) {
    let mut src = source.addr();
    if options(e) & 8 != 0 {
        while e.mem.u8(src) != 0 && e.mem.u8(src) == b' ' {
            src = src.wrapping_add(1);
        }
    }
    if options(e) & 6 == 0 {
        e.call(STRCPY_S, &args![destination, NAME_SIZE, src]);
        return;
    }
    // drive (4 bytes), directory, file name, extension.
    e.with_stack(4 + 3 * PART_SIZE, |e, parts| {
        let drive = parts.addr();
        let directory = drive + 4;
        let name = directory + PART_SIZE;
        let extension = name + PART_SIZE;
        e.call(
            SPLITPATH_S,
            &args![
                src, drive, DRIVE_SIZE, directory, PART_SIZE, name, PART_SIZE, extension, PART_SIZE
            ],
        );
        if options(e) & 2 == 0 {
            e.mem.set_u8(drive, 0);
            e.mem.set_u8(directory, 0);
        }
        if options(e) & 4 == 0 {
            e.mem.set_u8(extension, 0);
        }
        e.call(
            MAKEPATH_S,
            &args![destination, NAME_SIZE, drive, directory, name, extension],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af3bd0, fn_00af3bd0(Ptr, Ptr) -> u32),
        entry!(0x00af3c40, fn_00af3c40(Ptr, Ptr) -> i32),
        entry!(0x00af3fd0, fn_00af3fd0(Ptr) -> bool),
        entry!(0x00af4070, fn_00af4070(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles for the CRT: ASCII `tolower`, `strcpy_s`, and split/make
    /// path that keep only what the test needs (the name before the last
    /// `.` and the extension from it).
    fn engine(flags: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        e.set_global(OPTIONS, flags);
        e.register(TO_LOWER, |_, a| {
            (a[0] as u8).to_ascii_lowercase().into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(SPLITPATH_S, |e, a| {
            let s = e.mem.cstr(a[0]);
            let dot = s.iter().rposition(|&c| c == b'.').unwrap_or(s.len());
            e.mem.set_cstr(a[1], b"");
            e.mem.set_cstr(a[3], b"");
            e.mem.set_cstr(a[5], &s[..dot]);
            e.mem.set_cstr(a[7], &s[dot..]);
            Ret::default()
        });
        e.register(MAKEPATH_S, |e, a| {
            let mut s = e.mem.cstr(a[4]);
            s.extend(e.mem.cstr(a[5]));
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e
    }

    fn text(e: &mut Engine, s: &[u8]) -> Ptr {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s);
        Ptr::new(p)
    }

    fn matches(e: &mut Engine, s: &[u8], pattern: &[u8]) -> i32 {
        let s = text(e, s);
        let pattern = text(e, pattern);
        e.call(0x00af_3c40, &args![s, pattern]).i32()
    }

    #[test]
    fn wildcard_matching() {
        let mut e = engine(0);
        assert_eq!(matches(&mut e, b"abc", b"abc"), 1);
        assert_eq!(matches(&mut e, b"ABC", b"abc"), 1);
        assert_eq!(matches(&mut e, b"abd", b"abc"), 0);
        assert_eq!(matches(&mut e, b"abc", b"a?c"), 1);
        assert_eq!(matches(&mut e, b"abc", b"a*"), 1);
        assert_eq!(matches(&mut e, b"abcde", b"a*e"), 1);
        assert_eq!(matches(&mut e, b"abcd", b"a*e"), 2);
        assert_eq!(matches(&mut e, b"a*", b"a\\*"), 1);
        assert_eq!(matches(&mut e, b"ab", b"a\\*"), 0);
        assert_eq!(matches(&mut e, b"", b""), 1);
        assert_eq!(matches(&mut e, b"a", b""), 0);
    }

    #[test]
    fn set_matching() {
        let mut e = engine(0);
        assert_eq!(matches(&mut e, b"b1", b"[abc]1"), 1);
        assert_eq!(matches(&mut e, b"d1", b"[abc]1"), 0);
        assert_eq!(matches(&mut e, b"m", b"[a-z]"), 1);
        assert_eq!(matches(&mut e, b"5", b"[a-z]"), 0);
        assert_eq!(matches(&mut e, b"5", b"[^a-z]"), 1);
        assert_eq!(matches(&mut e, b"m", b"[^a-z]"), 0);
        assert_eq!(matches(&mut e, b"-", b"[-x]"), 1);
        assert_eq!(matches(&mut e, b"]", b"[\\]]"), 1);
        assert_eq!(matches(&mut e, b"a", b"[abc"), 0);
        assert_eq!(matches(&mut e, b"", b"[abc]"), 0);
    }

    #[test]
    fn case_sensitive_option() {
        let mut e = engine(1);
        assert_eq!(matches(&mut e, b"ABC", b"abc"), 0);
        assert_eq!(matches(&mut e, b"abc", b"abc"), 1);
        let s = text(&mut e, b"Q");
        let p = text(&mut e, b"q");
        e.call_log = Some(vec![]);
        e.call(0x00af_3c40, &args![s, p]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == TO_LOWER));
    }

    #[test]
    fn wildcard_detection() {
        let mut e = engine(0);
        for (s, expected) in [
            (&b"plain.nif"[..], false),
            (b"a*b", true),
            (b"a?b", true),
            (b"x[y", true),
            (b"x]y", true),
            (b"^", true),
            (b"a\\*b", false),
            (b"trailing\\", false),
        ] {
            let p = text(&mut e, s);
            assert_eq!(e.call(0x00af_3fd0, &args![p]).bool(), expected, "{s:?}");
        }
    }

    #[test]
    fn reduce_copies_whole_names_and_skips_spaces() {
        let mut e = engine(8);
        let out: Ptr = Ptr::new(e.mem.alloc(NAME_SIZE));
        let src = text(&mut e, b"  meshes\\a.nif");
        e.call(0x00af_4070, &args![out, src]);
        assert_eq!(e.mem.cstr(out.addr()), b"meshes\\a.nif");
        e.set_global(OPTIONS, 0u32);
        e.call(0x00af_4070, &args![out, src]);
        assert_eq!(e.mem.cstr(out.addr()), b"  meshes\\a.nif");
    }

    #[test]
    fn reduce_drops_the_extension_without_bit_2() {
        let mut e = engine(2);
        let out: Ptr = Ptr::new(e.mem.alloc(NAME_SIZE));
        let src = text(&mut e, b"rock.nif");
        e.call_log = Some(vec![]);
        e.call(0x00af_4070, &args![out, src]);
        let log = e.call_log.take().unwrap();
        let split = log.iter().find(|(a, _)| *a == SPLITPATH_S).unwrap();
        assert_eq!(
            [split.1[0], split.1[2], split.1[4], split.1[6], split.1[8]],
            [src.addr(), 3, 0x100, 0x100, 0x100]
        );
        assert_eq!(e.mem.cstr(out.addr()), b"rock");
        e.set_global(OPTIONS, 4u32);
        e.call(0x00af_4070, &args![out, src]);
        assert_eq!(e.mem.cstr(out.addr()), b"rock.nif");
    }

    #[test]
    fn match_reduces_both_names_first() {
        let mut e = engine(4);
        let name = text(&mut e, b"Rock01.NIF");
        let pattern = text(&mut e, b"rock*.nif");
        assert_eq!(e.call(0x00af_3bd0, &args![name, pattern]).u32(), 1);
        let other = text(&mut e, b"tree.nif");
        assert_eq!(e.call(0x00af_3bd0, &args![other, pattern]).u32(), 0);
        // A `*` that runs out of text (result 2) is not a match.
        let short = text(&mut e, b"rock");
        let star = text(&mut e, b"rock*x");
        assert_eq!(e.call(0x00af_3bd0, &args![short, star]).u32(), 0);
    }
}
