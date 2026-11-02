//! `NiSystem/nipath_xenon.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiStrnicmp` (`00aa1370`, `cdecl`: two texts and a count).
const NI_STRNICMP: u32 = 0x00aa_1370;
/// CRT `strchr` (`00ec7690`, `cdecl`).
const STRCHR: u32 = 0x00ec_7690;
/// CRT `toupper` (`00eca7f4`, `cdecl`).
const TOUPPER: u32 = 0x00ec_a7f4;
/// CRT `_getcwd` (`00eccbdc`, `cdecl`: buffer, size).
const GETCWD: u32 = 0x00ec_cbdc;
/// `"\\\\"` (two backslashes) in `.rdata` (`010a2564`).
const DOUBLE_BACKSLASH_TEXT: u32 = 0x010a_2564;
/// `"\\"` (one backslash) in `.rdata` (`01013444`).
const BACKSLASH_TEXT: u32 = 0x0101_3444;

/// The low byte of `toupper` of the path's first character, as the code
/// keeps it in `AL` (the character is sign-extended first, `MOVSX`).
fn upper_first(e: &mut Engine, path: u32) -> u8 {
    let c = e.mem.u8(path) as i8 as i32;
    e.call(TOUPPER, &args![c]).u8()
}

// Translated from 00aa2c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPath::StripAbsoluteBase` (Xbox PDB), `cdecl`, one word: a path starting
/// with two backslashes (`NiStrnicmp` over 2) answers the character after the
/// first backslash found from its third character on (`strchr(path + 2, '\\')
/// + 1`); one starting with a backslash answers `path + 1`; one starting with
/// a drive letter (`toupper` of the first character in `A`..`Z` and a `:`
/// second) answers `path + 2`, or `path + 3` when a backslash follows the
/// colon; any other path answers itself.
pub fn ni_path_strip_absolute_base(e: &mut Engine, path: Ptr) -> Ptr {
    let path = path.addr();
    let unc = e
        .call(NI_STRNICMP, &args![path, DOUBLE_BACKSLASH_TEXT, 2u32])
        .u32();
    if unc == 0 {
        let found = e.call(STRCHR, &args![path.wrapping_add(2), 0x5cu32]).u32();
        return Ptr::new(found.wrapping_add(1));
    }
    let rooted = e
        .call(NI_STRNICMP, &args![path, BACKSLASH_TEXT, 1u32])
        .u32();
    if rooted == 0 {
        return Ptr::new(path.wrapping_add(1));
    }
    let upper = upper_first(e, path);
    if e.mem.u8(path.wrapping_add(1)) != b':' {
        return Ptr::new(path);
    }
    if upper.wrapping_sub(0x41) > 0x19 {
        return Ptr::new(path);
    }
    let after = path.wrapping_add(2);
    if e.mem.u8(after) != b'\\' {
        return Ptr::new(after);
    }
    Ptr::new(after.wrapping_add(1))
}

// Translated from 00aa2cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPath::IsRelative` (Xbox PDB), `cdecl`, one word: a path shorter than
/// two characters is relative; one starting with `\` or `/` is not, nor one
/// starting with a drive letter (`toupper` of the first character in
/// `A`..`Z` followed by `:`); any other path is relative.
pub fn ni_path_is_relative(e: &mut Engine, path: Ptr) -> bool {
    let path = path.addr();
    let mut length = 0u32;
    while e.mem.u8(path.wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    if length < 2 {
        return true;
    }
    let first = e.mem.u8(path);
    if first == b'\\' || first == b'/' {
        return false;
    }
    let upper = upper_first(e, path);
    if e.mem.u8(path.wrapping_add(1)) != b':' {
        return true;
    }
    upper.wrapping_sub(0x41) > 0x19
}

// Translated from 00aa2d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPath::GetCurrentWorkingDirectory` (Xbox PDB), `cdecl`, two words (the
/// buffer and its size): `_getcwd(buffer, size)`, answering 1 when it gives a
/// non-null result and 0 otherwise (the whole of `EAX`).
pub fn ni_path_get_current_working_directory(e: &mut Engine, buffer: Ptr, size: u32) -> u32 {
    let result = e.call(GETCWD, &args![buffer, size]).u32();
    u32::from(result != 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_2c50, ni_path_strip_absolute_base(Ptr) -> Ptr),
        entry!(0x00aa_2cc0, ni_path_is_relative(Ptr) -> bool),
        entry!(
            0x00aa_2d10,
            ni_path_get_current_working_directory(Ptr, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine whose `NiStrnicmp` compares case-insensitively over the
    /// count, `strchr` searches and `toupper` upper-cases, as the CRT does.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_STRNICMP, |e, a| {
            for i in 0..a[2] {
                let x = e.mem.u8(a[0] + i).to_ascii_lowercase();
                let y = e.mem.u8(a[1] + i).to_ascii_lowercase();
                if x != y {
                    return 1u32.into_ret();
                }
                if x == 0 {
                    break;
                }
            }
            0u32.into_ret()
        });
        e.register(STRCHR, |e, a| {
            let mut p = a[0];
            loop {
                let c = e.mem.u8(p);
                if u32::from(c) == a[1] {
                    return p.into_ret();
                }
                if c == 0 {
                    return 0u32.into_ret();
                }
                p += 1;
            }
        });
        e.register(TOUPPER, |_, a| {
            u32::from((a[0] as u8).to_ascii_uppercase()).into_ret()
        });
        e.map(DOUBLE_BACKSLASH_TEXT, 4);
        e.mem.write(DOUBLE_BACKSLASH_TEXT, b"\\\\\0");
        e.map(BACKSLASH_TEXT, 4);
        e.mem.write(BACKSLASH_TEXT, b"\\\0");
        e
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.write(p, s);
        e.mem.set_u8(p + s.len() as u32, 0);
        p
    }

    #[test]
    fn ni_path_strip_absolute_base_handles_each_form() {
        let mut e = engine();
        let unc = text(&mut e, b"\\\\server\\share\\x");
        assert_eq!(e.call(0x00aa_2c50, &args![unc]).u32(), unc + 9);
        let rooted = text(&mut e, b"\\dir\\x");
        assert_eq!(e.call(0x00aa_2c50, &args![rooted]).u32(), rooted + 1);
        let drive = text(&mut e, b"c:\\dir");
        assert_eq!(e.call(0x00aa_2c50, &args![drive]).u32(), drive + 3);
        let bare_drive = text(&mut e, b"d:dir");
        assert_eq!(
            e.call(0x00aa_2c50, &args![bare_drive]).u32(),
            bare_drive + 2
        );
        let digit = text(&mut e, b"1:dir");
        assert_eq!(e.call(0x00aa_2c50, &args![digit]).u32(), digit);
        let relative = text(&mut e, b"dir\\x");
        assert_eq!(e.call(0x00aa_2c50, &args![relative]).u32(), relative);
    }

    #[test]
    fn ni_path_is_relative_handles_each_form() {
        let mut e = engine();
        for (path, relative) in [
            (&b"a"[..], true),
            (b"\\dir", false),
            (b"/dir", false),
            (b"c:dir", false),
            (b"1:dir", true),
            (b"dir\\x", true),
        ] {
            let p = text(&mut e, path);
            assert_eq!(e.call(0x00aa_2cc0, &args![p]).bool(), relative);
        }
    }

    #[test]
    fn ni_path_get_current_working_directory_answers_one_for_a_result() {
        let mut e = Engine::new();
        e.register(GETCWD, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x00aa_2d10, &args![0x9000u32, 0x104u32]).u32(), 1);
        assert_eq!(e.call(0x00aa_2d10, &args![0u32, 0x104u32]).u32(), 0);
    }
}
