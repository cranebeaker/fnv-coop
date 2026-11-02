//! `NiSystem/run_00aa2820` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa2820`: the jump target of `00aa29d0`.
const JUMP_TARGET_00AA29D0: u32 = 0x00aa_2820;

// Translated from 00aa29d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00aa2820`, passing on `this` and its two arguments; returns the target result. Return type (bool): in the target `00aa2820` every return sets AL (`XOR AL,AL` twice, `MOV AL,1` once); the class is unnamed.
pub fn fn_00aa29d0(e: &mut Engine, this: Ptr, first: Ptr, second: u32) -> bool {
    e.call(JUMP_TARGET_00AA29D0, &args![this, first, second])
        .bool()
}

/// CRT `_strcat_s` (`00ec6bd2`, `cdecl`: destination, its size, source).
const STRCAT_S: u32 = 0x00ec_6bd2;
/// `00a574e0` (`cdecl`, three words: destination, its size, text): called to
/// append the separator after a directory part.
const APPEND_SEPARATOR: u32 = 0x00a5_74e0;
/// `"\\"` in `.rdata` (`01013444`).
const BACKSLASH_TEXT: u32 = 0x0101_3444;
/// `"."` in `.rdata` (`01044c28`).
const DOT_TEXT: u32 = 0x0104_4c28;

/// The length of the NUL-terminated text at `a` (the exe's inline `strlen` loop).
fn text_length(e: &Engine, a: u32) -> u32 {
    e.mem.cstr(a).len() as u32
}

/// Whether `c` is one of the two path separators the code tests (`\` and `/`).
fn is_separator(c: u8) -> bool {
    c == b'\\' || c == b'/'
}

// Translated from 00aa2820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiFilename`, `RET 8`: the
/// output buffer and its size): joins the parts back into a path. A null
/// buffer answers false. The needed size is counted first: 1 (the NUL), 2 more
/// for a drive (`m_acDrive` +0x100), the directory (`m_acDir` +0) and the
/// subdirectory (`m_acSubDir` +0x303) each with one more for a separator when
/// they do not end in `\` or `/`, the name (`m_acFname` +0x203), and the
/// extension (`m_acExt` +0x103) with one more when it does not start with `.`;
/// above the size answers false without writing. Otherwise the buffer gets the
/// drive and `:`, then the directory and the subdirectory (each followed by
/// `00a574e0(buffer, size, "\\")` when the buffer does not then end in a
/// separator), the name, and `.` (unless the extension starts with one) and the
/// extension, all through `_strcat_s`; answers true.
pub fn fn_00aa2820(e: &mut Engine, this: Ptr, out: Ptr, size: u32) -> bool {
    let t = this.addr();
    let out = out.addr();
    if out == 0 {
        return false;
    }
    let dir = t;
    let drive = t + 0x100;
    let ext = t + 0x103;
    let fname = t + 0x203;
    let subdir = t + 0x303;
    let mut needed: u32 = if e.mem.u8(drive) != 0 { 3 } else { 1 };
    for part in [dir, subdir] {
        if e.mem.u8(part) != 0 {
            let length = text_length(e, part);
            needed = needed.wrapping_add(length);
            let last = e.mem.u8(part.wrapping_add(length).wrapping_sub(1));
            if !is_separator(last) {
                needed = needed.wrapping_add(1);
            }
        }
    }
    if e.mem.u8(fname) != 0 {
        needed = needed.wrapping_add(text_length(e, fname));
    }
    let first = e.mem.u8(ext);
    if first != 0 {
        if first != b'.' {
            needed = needed.wrapping_add(1);
        }
        needed = needed.wrapping_add(text_length(e, ext));
    }
    if needed > size {
        return false;
    }
    e.mem.set_u8(out, 0);
    let letter = e.mem.u8(drive);
    if letter != 0 {
        e.mem.set_u8(out, letter);
        e.mem.set_u8(out + 1, b':');
        e.mem.set_u8(out + 2, 0);
    }
    for part in [dir, subdir] {
        if e.mem.u8(part) != 0 {
            e.call(STRCAT_S, &args![out, size, part]);
            let length = text_length(e, out);
            let last = e.mem.u8(out.wrapping_add(length).wrapping_sub(1));
            if !is_separator(last) {
                e.call(APPEND_SEPARATOR, &args![out, size, BACKSLASH_TEXT]);
            }
        }
    }
    if e.mem.u8(fname) != 0 {
        e.call(STRCAT_S, &args![out, size, fname]);
    }
    let first = e.mem.u8(ext);
    if first != 0 {
        if first != b'.' {
            e.call(STRCAT_S, &args![out, size, DOT_TEXT]);
        }
        e.call(STRCAT_S, &args![out, size, ext]);
    }
    true
}

/// Import slot of `MessageBoxA` (`stdcall`: owner, text, caption, type).
const API_MESSAGE_BOX: u32 = 0x00fd_f2f4;

// Translated from 00aa29e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the text and the caption): shows a
/// message box with no owner and type 0x40000 (`MB_TOPMOST`), then answers 0.
pub fn fn_00aa29e0(e: &mut Engine, text: Ptr, caption: Ptr) -> u32 {
    e.call(API_MESSAGE_BOX, &args![0u32, text, caption, 0x40000u32]);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_29e0, fn_00aa29e0(Ptr, Ptr) -> u32),
        entry!(0x00aa_29d0, fn_00aa29d0(Ptr, Ptr, u32) -> bool),
        entry!(0x00aa_2820, fn_00aa2820(Ptr, Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00aa29d0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00aa_29d0;
        const TARGET: u32 = 0x00aa_2820;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]).u32(),
            1
        );
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(
            e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]).u32(),
            0
        );
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    /// A `_strcat_s` / `00a574e0` double that appends the source text.
    fn append(e: &mut Engine, a: &[u32]) -> Ret {
        let mut text = e.mem.cstr(a[0]);
        text.extend(e.mem.cstr(a[2]));
        e.mem.set_cstr(a[0], &text);
        Ret::default()
    }

    fn filename(
        e: &mut Engine,
        drive: &[u8],
        dir: &[u8],
        sub: &[u8],
        name: &[u8],
        ext: &[u8],
    ) -> u32 {
        let this = e.mem.alloc(0x404);
        e.mem.set_cstr(this + 0x100, drive);
        e.mem.set_cstr(this, dir);
        e.mem.set_cstr(this + 0x303, sub);
        e.mem.set_cstr(this + 0x203, name);
        e.mem.set_cstr(this + 0x103, ext);
        this
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(BACKSLASH_TEXT, 4);
        e.mem.set_cstr(BACKSLASH_TEXT, b"\\");
        e.map(DOT_TEXT, 4);
        e.mem.set_cstr(DOT_TEXT, b".");
        e.register(STRCAT_S, append);
        e.register(APPEND_SEPARATOR, append);
        e
    }

    #[test]
    fn fn_00aa2820_joins_the_parts() {
        let mut e = engine();
        let this = filename(&mut e, b"c", b"\\data", b"sub/", b"file", b"nif");
        let out = e.mem.alloc(0x40);
        // 3 + "\data" 5 + 1 + "sub/" 4 + "file" 4 + 1 + "nif" 3 = 21.
        assert!(e.call(0x00aa_2820, &args![this, out, 21u32]).bool());
        assert_eq!(e.mem.cstr(out), b"c:\\data\\sub/file.nif".to_vec());
        let out = e.mem.alloc(0x40);
        e.mem.set_u8(out, b'z');
        assert!(!e.call(0x00aa_2820, &args![this, out, 20u32]).bool());
        assert_eq!(e.mem.u8(out), b'z');
    }

    #[test]
    fn fn_00aa2820_keeps_a_dotted_extension_and_refuses_a_null_buffer() {
        let mut e = engine();
        let this = filename(&mut e, b"", b"", b"", b"a", b".b");
        let out = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00aa_2820, &args![this, out, 4u32]).bool());
        assert_eq!(e.mem.cstr(out), b"a.b".to_vec());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.1.get(2) != Some(&DOT_TEXT)));
        assert!(!e.call(0x00aa_2820, &args![this, 0u32, 4u32]).bool());
    }
}

#[cfg(test)]
mod tests_n084 {
    use super::*;

    #[test]
    fn fn_00aa29e0_shows_a_topmost_box_and_answers_0() {
        let mut e = Engine::new();
        e.register(API_MESSAGE_BOX, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_29e0, &args![0x9000u32, 0x9100u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let boxes: Vec<_> = log.iter().filter(|c| c.0 == API_MESSAGE_BOX).collect();
        assert_eq!(
            boxes,
            vec![&(API_MESSAGE_BOX, vec![0, 0x9000, 0x9100, 0x40000])]
        );
    }
}
