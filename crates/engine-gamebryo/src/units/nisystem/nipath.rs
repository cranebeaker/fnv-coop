//! `NiSystem/nipath.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const SLASH: u8 = b'/';
const BACKSLASH: u8 = b'\\';

// Translated from 00aa2a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPath::Standardize` (engine map): rewrites the C string at `path` in place
/// (`cdecl`, one stack word). Every `/` becomes a backslash, and a backslash
/// that follows a backslash is dropped, except that the first two characters
/// are never dropped (so a leading double backslash of a network path stays).
/// Returns true when it changed a character or dropped one (`AL`, set by a `/`
/// or a dropped backslash), false for a null pointer or an unchanged path.
pub fn ni_path_standardize(e: &mut Engine, path: Ptr) -> bool {
    if path.is_null() {
        return false;
    }
    let text = e.mem.cstr(path.addr());
    let length = text.len() as u32;
    let mut changed = false;
    let mut out = 0u32;
    let mut after_backslash = false;
    let mut index = 0u32;
    while index < length {
        let mut c = text[index as usize];
        if c == SLASH {
            changed = true;
            c = BACKSLASH;
        }
        let is_backslash = c == BACKSLASH;
        if is_backslash && after_backslash && index > 1 {
            changed = true;
        } else {
            e.mem.set_u8(path.addr().wrapping_add(out), c);
            after_backslash = is_backslash;
            out += 1;
        }
        index += 1;
    }
    e.mem.set_u8(path.addr().wrapping_add(out), 0);
    changed
}

/// `strstr` (`00ec7750`), `strrchr` (`00ec6e30`), `_strcpy_s` (`00ec65a6`) and
/// `_strcat_s` (`00ec6bd2`), cdecl.
const STRSTR: u32 = 0x00ec_7750;
const STRRCHR: u32 = 0x00ec_6e30;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCAT_S: u32 = 0x00ec_6bd2;
/// `NiPath::IsRelative` (`00aa2cc0`) and `NiPath::StripAbsoluteBase` (`00aa2c50`), both in
/// `NiSystem/nipath_xenon.cpp` (engine map; cdecl, one word).
const IS_RELATIVE: u32 = 0x00aa_2cc0;
const STRIP_ABSOLUTE_BASE: u32 = 0x00aa_2c50;
/// `"\"` (`01013444`).
const BACKSLASH_TEXT: u32 = 0x0101_3444;

/// The length of the C string at `text`, read byte by byte as the inlined loops do.
fn inline_length(e: &Engine, text: u32) -> u32 {
    let mut at = text;
    while e.mem.u8(at) != 0 {
        at = at.wrapping_add(1);
    }
    at.wrapping_sub(text)
}

// Translated from 00aa2a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPath::RemoveSlashDotSlash` (engine map; cdecl, one word): while `path` holds `\.\`
/// (`strstr` with the pattern in a stack word), the text from that place is replaced by
/// the text two characters on (`strcpy_s`, its size the length from that place).
pub fn ni_path_remove_slash_dot_slash(e: &mut Engine, path: Ptr) {
    e.with_stack(4, |e, pattern| {
        e.mem.set_u32(pattern.addr(), 0x005c_2e5c);
        let mut found = e.call(STRSTR, &args![path, pattern]).u32();
        while found != 0 {
            let length = inline_length(e, found);
            e.call(STRCPY_S, &args![found, length, found.wrapping_add(2)]);
            found = e.call(STRSTR, &args![path, pattern]).u32();
        }
    });
}

// Translated from 00aa2ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPath::RemoveDotDots` (engine map; cdecl, one word): standardizes the path
/// ([`ni_path_standardize`]), removes `\.\` ([`ni_path_remove_slash_dot_slash`]), then
/// starts after the leading `.` and `\` of a relative path (`NiPath::IsRelative`) or after
/// the absolute base (`NiPath::StripAbsoluteBase`). There, for each `\..` (`strstr`), the
/// part before it loses its last component: the text from the last `\` before it (`strrchr`;
/// from the start, past the `\..\`, when there is none) is replaced by what follows the
/// `\..`, byte by byte up to the end of the original text.
pub fn ni_path_remove_dot_dots(e: &mut Engine, path: Ptr) {
    ni_path_standardize(e, path);
    ni_path_remove_slash_dot_slash(e, path);
    let mut start = path.addr();
    if e.call(IS_RELATIVE, &args![path]).bool() {
        if start == 0 {
            return;
        }
        loop {
            let c = e.mem.u8(start);
            if c != b'.' && c != BACKSLASH {
                break;
            }
            start = start.wrapping_add(1);
            if start == 0 {
                return;
            }
        }
    } else {
        start = e.call(STRIP_ABSOLUTE_BASE, &args![path]).u32();
    }
    if start == 0 {
        return;
    }
    let end = inline_length(e, start).wrapping_add(start).wrapping_add(1);
    e.with_stack(4, |e, pattern| {
        e.mem.set_u32(pattern.addr(), 0x002e_2e5c);
        let mut found = e.call(STRSTR, &args![start, pattern]).u32();
        while found != 0 {
            e.mem.set_u8(found, 0);
            let slash = e.call(STRRCHR, &args![start, u32::from(BACKSLASH)]).u32();
            let (mut to, from) = if slash != 0 {
                (slash, found.wrapping_add(3))
            } else if found.wrapping_add(3) != 0 {
                (start, found.wrapping_add(4))
            } else {
                (start, 0)
            };
            let mut count = end.wrapping_sub(from) as i32;
            if count > 0 {
                let distance = from.wrapping_sub(to);
                loop {
                    let byte = e.mem.u8(to.wrapping_add(distance));
                    e.mem.set_u8(to, byte);
                    to = to.wrapping_add(1);
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                }
            }
            found = e.call(STRSTR, &args![start, pattern]).u32();
        }
    });
}

// Translated from 00aa2ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPath::ConvertToAbsolute` (engine map; cdecl, four words): writes into `out` (of
/// `size` bytes) `base`, a `\` when `base` does not end in one, then `relative`
/// (`strcpy_s`, `strcat_s`), removes its `..` parts ([`ni_path_remove_dot_dots`]) and
/// answers its length. When it would not fit (`size` below the lengths plus the `\` and the
/// terminator) `out` is emptied (when `size` is not 0) and 0 is answered.
pub fn ni_path_convert_to_absolute(
    e: &mut Engine,
    out: Ptr,
    size: u32,
    relative: u32,
    base: u32,
) -> u32 {
    let base_length = inline_length(e, base);
    let relative_length = inline_length(e, relative);
    let needs_slash = e.mem.u8(base_length.wrapping_add(base).wrapping_sub(1)) != BACKSLASH;
    let needed = u32::from(needs_slash)
        .wrapping_add(relative_length)
        .wrapping_add(base_length)
        .wrapping_add(1);
    if size < needed {
        if size != 0 {
            e.mem.set_u8(out.addr(), 0);
        }
        return 0;
    }
    e.call(STRCPY_S, &args![out, size, base]);
    if needs_slash {
        e.call(STRCAT_S, &args![out, size, BACKSLASH_TEXT]);
    }
    e.call(STRCAT_S, &args![out, size, relative]);
    ni_path_remove_dot_dots(e, out);
    inline_length(e, out.addr())
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_2a00, ni_path_standardize(Ptr) -> bool),
        entry!(0x00aa_2a70, ni_path_remove_slash_dot_slash(Ptr)),
        entry!(0x00aa_2ac0, ni_path_remove_dot_dots(Ptr)),
        entry!(
            0x00aa_2ba0,
            ni_path_convert_to_absolute(Ptr, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn standardize(text: &[u8]) -> (bool, Vec<u8>) {
        let mut e = Engine::new();
        let buffer = e.mem.alloc(0x40);
        e.mem.set_cstr(buffer, text);
        let changed = e.call(0x00aa_2a00, &args![buffer]).bool();
        (changed, e.mem.cstr(buffer))
    }

    #[test]
    fn a_null_path_gives_false() {
        let mut e = Engine::new();
        assert!(!e.call(0x00aa_2a00, &args![0u32]).bool());
    }

    #[test]
    fn a_clean_path_is_unchanged() {
        assert_eq!(
            standardize(b"meshes\\a.nif"),
            (false, b"meshes\\a.nif".to_vec())
        );
        assert_eq!(standardize(b""), (false, Vec::new()));
    }

    #[test]
    fn slashes_become_backslashes() {
        assert_eq!(
            standardize(b"meshes/a/b.nif"),
            (true, b"meshes\\a\\b.nif".to_vec())
        );
    }

    #[test]
    fn repeated_backslashes_collapse_after_the_first_two_characters() {
        assert_eq!(standardize(b"a\\\\b\\\\\\c"), (true, b"a\\b\\c".to_vec()));
        assert_eq!(standardize(b"x//y"), (true, b"x\\y".to_vec()));
    }

    #[test]
    fn a_leading_double_backslash_is_kept() {
        assert_eq!(
            standardize(b"\\\\srv\\\\share"),
            (true, b"\\\\srv\\share".to_vec())
        );
        assert_eq!(standardize(b"//srv"), (true, b"\\\\srv".to_vec()));
        assert_eq!(standardize(b"\\\\srv"), (false, b"\\\\srv".to_vec()));
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    /// An engine with working doubles for the CRT string helpers.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(STRSTR, |e, a| {
            let hay = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            let at = hay
                .windows(needle.len().max(1))
                .position(|w| w == needle.as_slice());
            at.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let at = text.iter().rposition(|&c| u32::from(c) == a[1]);
            at.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(IS_RELATIVE, |e, a| (e.mem.u8(a[0] + 1) != b':').into_ret());
        e.register(STRIP_ABSOLUTE_BASE, |_, a| (a[0] + 3).into_ret());
        e
    }

    fn text(e: &mut Engine, value: &[u8]) -> u32 {
        let at = e.mem.alloc(0x40);
        e.mem.set_cstr(at, value);
        at
    }

    #[test]
    fn slash_dot_slash_is_removed() {
        let mut e = engine();
        let path = text(&mut e, b"a\\.\\b\\.\\c");
        e.call(0x00aa_2a70, &args![path]);
        assert_eq!(e.mem.cstr(path), b"a\\b\\c");
    }

    #[test]
    fn dot_dots_drop_the_component_before_them() {
        let mut e = engine();
        let path = text(&mut e, b"a/b/../c");
        e.call(0x00aa_2ac0, &args![path]);
        assert_eq!(e.mem.cstr(path), b"a\\c");
        // No `\` before: from the start, past the `\..\`.
        let path = text(&mut e, b"a\\..\\c");
        e.call(0x00aa_2ac0, &args![path]);
        assert_eq!(e.mem.cstr(path), b"c");
        // Leading dots and backslashes of a relative path are kept as they are.
        let path = text(&mut e, b"..\\x\\y\\..\\z");
        e.call(0x00aa_2ac0, &args![path]);
        assert_eq!(e.mem.cstr(path), b"..\\x\\z");
        // An absolute path starts after its base.
        let path = text(&mut e, b"c:\\d\\..\\e");
        e.call(0x00aa_2ac0, &args![path]);
        assert_eq!(e.mem.cstr(path), b"c:\\e");
    }

    #[test]
    fn convert_to_absolute_joins_and_cleans() {
        let mut e = engine();
        e.map(BACKSLASH_TEXT, 4);
        e.mem.set_cstr(BACKSLASH_TEXT, b"\\");
        let out = e.mem.alloc(0x40);
        let base = text(&mut e, b"c:\\data");
        let relative = text(&mut e, b"x\\..\\y.nif");
        let r = e
            .call(0x00aa_2ba0, &args![out, 0x40u32, relative, base])
            .u32();
        assert_eq!(e.mem.cstr(out), b"c:\\data\\y.nif");
        assert_eq!(r, 13);
        // A base ending in `\` gets none added.
        let base = text(&mut e, b"c:\\data\\");
        let r = e
            .call(0x00aa_2ba0, &args![out, 0x40u32, relative, base])
            .u32();
        assert_eq!((e.mem.cstr(out), r), (b"c:\\data\\y.nif".to_vec(), 13));
        // Too small: emptied, 0; a zero size touches nothing.
        e.mem.set_cstr(out, b"keep");
        assert_eq!(
            e.call(0x00aa_2ba0, &args![out, 18u32, relative, base])
                .u32(),
            0
        );
        assert_eq!(e.mem.cstr(out), b"");
        e.mem.set_cstr(out, b"keep");
        assert_eq!(
            e.call(0x00aa_2ba0, &args![out, 0u32, relative, base]).u32(),
            0
        );
        assert_eq!(e.mem.cstr(out), b"keep");
    }
}
