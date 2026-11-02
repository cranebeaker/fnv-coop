//! `BSSystem/run_00afd230` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A reference holder's constructor and the file-name hash: `00afd4e0` hashes a lower-cased
//! name and extension into an 8-byte value (bytes 0 to 3 and a word at +4), and the two
//! callers split a path first (`NiFilename::Splitpath`, `00ec7f1e`) and hash the directory and
//! the file name.

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiFilename::Splitpath` (`00ec7f1e`, the engine map's name; cdecl `_splitpath_s` order:
/// path, drive, size, directory, size, name, size, extension, size).
const SPLITPATH: u32 = 0x00ec_7f1e;
/// `_strcpy_s` (`00ec65a6`, cdecl: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `_strcat_s` (`00ec6bd2`, cdecl: destination, size, source).
const STRCAT_S: u32 = 0x00ec_6bd2;
/// `_strlen` (`00ec6130`, cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// `tolower` (`00ec67aa`, cdecl: the character as an `int`).
const TOLOWER: u32 = 0x00ec_67aa;
/// Six 10-byte extension entries (`""`, `".nif"`, `".kf"`, `".dds"`, `".wav"`, `".adp"`);
/// the hash compares the first word of each with the first word of the extension.
const EXTENSION_TABLE: u32 = 0x011a_cfbc;

fn lower(e: &mut Engine, byte: u8) -> u32 {
    e.call(TOLOWER, &args![u32::from(byte)]).u32()
}

/// `h * 0x1003f + c` as the code computes it (`(h << 6) + c + (h << 16) - h`), wrapping.
fn fold(hash: u32, character: u32) -> u32 {
    (hash << 6)
        .wrapping_add(character)
        .wrapping_add(hash << 16)
        .wrapping_sub(hash)
}

/// The hash's middle part: from `count`, walks `p` and `q = p + skip` together while `*q` is
/// not 0, folding `tolower(*p)`; then the count goes to byte 2, the hash to +4, and the
/// lower-cased `p[0]` and `p[1]` to bytes 1 and 0.
fn hash_middle(e: &mut Engine, out: u32, start: u32, skip: u32, count: u32, mut hash: u32) {
    let mut p = start;
    let mut q = start + skip;
    let mut count = count;
    while e.mem.u8(q) != 0 {
        let c = e.mem.u8(p);
        let character = lower(e, c);
        hash = fold(hash, character);
        count = count.wrapping_add(1);
        p += 1;
        q += 1;
    }
    e.mem.set_u8(out + 2, count as u8);
    e.mem.set_u32(out + 4, hash);
    let c = e.mem.u8(p);
    let first = lower(e, c) as u8;
    e.mem.set_u8(out + 1, first);
    let c = e.mem.u8(p + 1);
    let second = lower(e, c) as u8;
    e.mem.set_u8(out, second);
}

// Translated from 00afd230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a holder of one reference (the start-up's array constructor runs it over
/// two arrays at `011f8194` and `011f81b4`): stores null, then, if the stored word is not 0
/// (the test follows the store, so this never runs; translated anyway), increments the count
/// at +0x15c of the held object (`InterlockedIncrement`, the address computed as +0x158 + 4).
pub fn fn_00afd230(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
    let held = e.mem.u32(this.addr());
    if held != 0 {
        let count = held.wrapping_add(0x158).wrapping_add(4);
        e.call(INTERLOCKED_INCREMENT, &args![count]);
    }
}

// Translated from 00afd4e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afd4e0 - the /GS stack-cookie check (00ec408c) is not translated
/// Hashes the name at `name` and the extension at `extension` (may be null) into the 8 bytes
/// at `out`: byte 3 is `tolower(name[0])`, the word at +4 starts at 0. An empty name clears
/// bytes 0 to 2 and ends (the extension is not read). Otherwise, with `p = name + 1`:
/// - when `p[0]` to `p[3]` are all non-zero the compiler's folded test (`XOR EAX,EAX; JZ`)
///   still skips the first block, which never runs (translated anyway: count from 5, `q =
///   p + 4`, byte 3 = `tolower(p[2])`);
/// - when `p[0]` and `p[1]` are non-zero: count from 3, `q = p + 2`, folding `tolower(*p)`
///   into the hash (`h * 0x1003f + c`) while `*q` is not 0; byte 2 = the count, +4 = the
///   hash, byte 1 = `tolower(p[0])`, byte 0 = `tolower(p[1])` at the end of the walk;
/// - when only `p[0]` is non-zero: byte 0 = `tolower(p[0])`, byte 2 = 2; when it is 0: byte 2
///   = 1 and byte 0 = byte 3; byte 1 = 0 in both.
///
/// Then, for a non-null extension, its lower-cased copy goes into a 16-byte stack buffer
/// (with its terminator), its fold is added to +4, and if the buffer's first word equals the
/// first word of entry `i` of the six-entry table at `011acfbc`, bit 7 of byte 3, byte 0 and
/// byte 1 gets bit 2, 1 and 0 of `i` added (the first match only).
pub fn fn_00afd4e0(e: &mut Engine, name: Ptr, extension: Ptr, out: Ptr) {
    let out = out.addr();
    let c = e.mem.u8(name.addr());
    let first = lower(e, c) as u8;
    e.mem.set_u8(out + 3, first);
    e.mem.set_u32(out + 4, 0);
    let hash = 0u32;
    if e.mem.u8(out + 3) == 0 {
        e.mem.set_u8(out + 2, 0);
        e.mem.set_u8(out + 1, 0);
        e.mem.set_u8(out, 0);
        return;
    }
    let p = name.addr() + 1;
    // `XOR EAX,EAX; JZ`: the last condition of the first test is a folded false.
    let folded = std::hint::black_box(false);
    if e.mem.u8(p) != 0
        && e.mem.u8(p + 1) != 0
        && e.mem.u8(p + 2) != 0
        && e.mem.u8(p + 3) != 0
        && folded
    {
        let c = e.mem.u8(p + 2);
        let third = lower(e, c) as u8;
        e.mem.set_u8(out + 3, third);
        hash_middle(e, out, p, 4, 5, hash);
    } else if e.mem.u8(p) != 0 && e.mem.u8(p + 1) != 0 {
        hash_middle(e, out, p, 2, 3, hash);
    } else {
        if e.mem.u8(p) != 0 {
            let c = e.mem.u8(p);
            let lowered = lower(e, c) as u8;
            e.mem.set_u8(out, lowered);
            e.mem.set_u8(out + 2, 2);
        } else {
            e.mem.set_u8(out + 2, 1);
            let byte = e.mem.u8(out + 3);
            e.mem.set_u8(out, byte);
        }
        e.mem.set_u8(out + 1, 0);
    }
    if extension.addr() == 0 {
        return;
    }
    // The 16-byte buffer of the 0x40-byte frame at EBP-0x34.
    e.with_stack(0x40, |e, frame| {
        let buffer = frame.addr() + 0x0c;
        let mut write = buffer;
        let mut p = extension.addr();
        let mut hash = 0u32;
        while e.mem.u8(p) != 0 {
            let c = e.mem.u8(p);
            let character = lower(e, c);
            e.mem.set_u8(write, character as u8);
            write += 1;
            hash = fold(hash, character);
            p += 1;
        }
        e.mem.set_u8(write, 0);
        let total = e.mem.u32(out + 4).wrapping_add(hash);
        e.mem.set_u32(out + 4, total);
        let first_word = e.mem.u32(buffer);
        for index in 0..6u32 {
            if e.mem.u32(EXTENSION_TABLE + index * 10) == first_word {
                let byte = e.mem.u8(out + 3);
                e.mem
                    .set_u8(out + 3, byte.wrapping_add(((index << 5) & 0x80) as u8));
                let byte = e.mem.u8(out);
                e.mem
                    .set_u8(out, byte.wrapping_add(((index << 6) & 0x80) as u8));
                let byte = e.mem.u8(out + 1);
                e.mem
                    .set_u8(out + 1, byte.wrapping_add(((index << 7) & 0x80) as u8));
                break;
            }
        }
    });
}

/// Splits `path` into the four buffers of `frame` (drive, directory, name, extension, each
/// first cleared to an empty string in the exe's order: drive, directory, name, extension).
fn split(e: &mut Engine, path: u32, drive: u32, directory: u32, name: u32, extension: u32) {
    e.mem.set_u8(drive, 0);
    e.mem.set_u8(directory, 0);
    e.mem.set_u8(name, 0);
    e.mem.set_u8(extension, 0);
    e.call(
        SPLITPATH,
        &args![path, drive, 0x10u32, directory, 0x104u32, name, 0x104u32, extension, 0x10u32],
    );
}

/// Rebuilds the directory (drive then directory) into `buffer`, `"."` when it is empty, and
/// drops one trailing backslash; `length` is the frame's word that keeps `strlen`'s result.
fn directory_of(e: &mut Engine, buffer: u32, drive: u32, directory: u32, length: u32) {
    e.call(STRCPY_S, &args![buffer, 0x104u32, drive]);
    e.call(STRCAT_S, &args![buffer, 0x104u32, directory]);
    if e.mem.u8(buffer) == 0 {
        e.mem.set_u8(buffer, b'.');
        e.mem.set_u8(buffer + 1, 0);
    }
    let len = e.call(STRLEN, &args![buffer]).u32();
    e.mem.set_u32(length, len);
    let last = buffer.wrapping_add(len).wrapping_sub(1);
    if e.mem.u8(last) == b'\\' {
        e.mem.set_u8(last, 0);
    }
}

// Translated from 00afd270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afd270 - the /GS stack-cookie check (00ec408c) is not translated
/// Hashes a path in two parts: splits `path` (`00ec7f1e`), hashes the file name with its
/// extension into `name_hash` (`00afd4e0`), then rebuilds the directory (drive and directory
/// in the name buffer, `"."` when empty, one trailing backslash dropped) and hashes it
/// without an extension into `directory_hash`.
pub fn fn_00afd270(e: &mut Engine, path: Ptr, directory_hash: Ptr, name_hash: Ptr) {
    // The 0x240-byte frame: name at EBP-0x240, drive at EBP-0x134, length at EBP-0x124,
    // directory at EBP-0x120, extension at EBP-0x14.
    e.with_stack(0x240, |e, frame| {
        let base = frame.addr();
        let (name, drive, length, directory, extension) =
            (base, base + 0x10c, base + 0x11c, base + 0x120, base + 0x22c);
        split(e, path.addr(), drive, directory, name, extension);
        fn_00afd4e0(e, Ptr::new(name), Ptr::new(extension), name_hash);
        directory_of(e, name, drive, directory, length);
        fn_00afd4e0(e, Ptr::new(name), Ptr::new(0), directory_hash);
    });
}

// Translated from 00afd390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afd390 - the /GS stack-cookie check (00ec408c) is not translated
/// Hashes `path` into `this` (`00afd4e0`) by `mode`: 2 hashes the whole path without an
/// extension; 0 splits it (`00ec7f1e`) and hashes the file name with its extension; any other
/// mode splits it and hashes the rebuilt directory (drive and directory, `"."` when empty,
/// one trailing backslash dropped) without an extension. Returns `this`.
pub fn fn_00afd390(e: &mut Engine, this: Ptr, path: Ptr, mode: u32) -> Ptr {
    // The 0x240-byte frame: `this` at EBP-0x240, length at EBP-0x23c, name at EBP-0x238,
    // drive at EBP-0x130, directory at EBP-0x120, extension at EBP-0x18.
    e.with_stack(0x240, |e, frame| {
        let base = frame.addr();
        let (length, name, drive, directory, extension) =
            (base + 4, base + 8, base + 0x110, base + 0x120, base + 0x228);
        e.mem.set_u32(base, this.addr());
        let mut hashed = path.addr();
        let mut hashed_extension = 0u32;
        if mode != 2 {
            split(e, path.addr(), drive, directory, name, extension);
            if mode == 0 {
                hashed_extension = extension;
                hashed = name;
            } else {
                directory_of(e, name, drive, directory, length);
                hashed = name;
            }
        }
        fn_00afd4e0(e, Ptr::new(hashed), Ptr::new(hashed_extension), this);
    });
    this
}

/// The formatted print into a buffer (`00406d00`, cdecl: buffer, size, format, values).
const PRINT_TO_BUFFER: u32 = 0x0040_6d00;
/// `"%c%02X%c%c%08X"` (`010a46ac`).
const HASH_TEXT_FORMAT: u32 = 0x010a_46ac;

// Translated from 00afd860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an 8-byte hash, one word): with an `out` buffer,
/// prints the hash into it (0x20 bytes, `"%c%02X%c%c%08X"`: bytes 3, 2, 1 and 0, then the
/// word at +4).
pub fn fn_00afd860(e: &mut Engine, this: Ptr, out: u32) {
    if out == 0 {
        return;
    }
    let at = this.addr();
    let word = e.mem.u32(at.wrapping_add(4));
    let bytes = [3u32, 2, 1, 0].map(|i| u32::from(e.mem.u8(at.wrapping_add(i))));
    e.call(
        PRINT_TO_BUFFER,
        &args![
            out,
            0x20u32,
            HASH_TEXT_FORMAT,
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            word
        ],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_d860, fn_00afd860(Ptr, u32)),
        entry!(0x00af_d230, fn_00afd230(Ptr)),
        entry!(0x00af_d4e0, fn_00afd4e0(Ptr, Ptr, Ptr)),
        entry!(0x00af_d270, fn_00afd270(Ptr, Ptr, Ptr)),
        entry!(0x00af_d390, fn_00afd390(Ptr, Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn string(e: &mut Engine, text: &str) -> u32 {
        let p = e.mem.alloc(text.len() as u32 + 1);
        for (i, b) in text.bytes().enumerate() {
            e.mem.set_u8(p + i as u32, b);
        }
        p
    }

    fn read_string(e: &Engine, mut p: u32) -> String {
        let mut s = String::new();
        while e.mem.u8(p) != 0 {
            s.push(e.mem.u8(p) as char);
            p += 1;
        }
        s
    }

    fn write_string(e: &mut Engine, p: u32, text: &str) {
        for (i, b) in text.bytes().enumerate() {
            e.mem.set_u8(p + i as u32, b);
        }
        e.mem.set_u8(p + text.len() as u32, 0);
    }

    /// `tolower` as a double, and the extension table.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(TOLOWER, |_, a| {
            u32::from((a[0] as u8).to_ascii_lowercase()).into_ret()
        });
        e.map(EXTENSION_TABLE, 60);
        for (i, ext) in ["", ".nif", ".kf", ".dds", ".wav", ".adp"]
            .iter()
            .enumerate()
        {
            write_string(&mut e, EXTENSION_TABLE + 10 * i as u32, ext);
        }
        e
    }

    fn hash(e: &mut Engine, name: &str, extension: Option<&str>) -> (u32, u32) {
        let n = string(e, name);
        let x = extension.map_or(0, |x| string(e, x));
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0xffff_ffff);
        e.mem.set_u32(out + 4, 0xffff_ffff);
        e.call(0x00af_d4e0, &args![n, x, out]);
        (e.mem.u32(out), e.mem.u32(out + 4))
    }

    fn bytes(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
        u32::from_le_bytes([b0, b1, b2, b3])
    }

    #[test]
    fn holder_constructor_stores_null() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |_, _| {
            panic!("the stored word is null")
        });
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1234);
        e.call(0x00af_d230, &args![this]);
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn hash_of_short_names() {
        let mut e = engine();
        assert_eq!(hash(&mut e, "", Some(".nif")), (bytes(0, 0, 0, 0), 0));
        assert_eq!(hash(&mut e, "A", None), (bytes(b'a', 0, 1, b'a'), 0));
        assert_eq!(hash(&mut e, "aB", None), (bytes(b'b', 0, 2, b'a'), 0));
    }

    #[test]
    fn hash_of_a_long_name_with_an_extension() {
        let mut e = engine();
        // "aBcDe": first 'a'; walk from "BcDe" with q two ahead: folds 'b', 'c'; ends on "De".
        let middle = fold(fold(0, u32::from(b'b')), u32::from(b'c'));
        assert_eq!(
            hash(&mut e, "aBcDe", None),
            (bytes(b'e', b'd', 5, b'a'), middle)
        );
        let ext = ".NIF"
            .bytes()
            .fold(0, |h, c| fold(h, u32::from(c.to_ascii_lowercase())));
        // ".nif" is entry 1: bit 0 of the index goes to bit 7 of byte 1.
        assert_eq!(
            hash(&mut e, "aBcDe", Some(".NIF")),
            (bytes(b'e', b'd' + 0x80, 5, b'a'), middle.wrapping_add(ext))
        );
        // ".adp" is entry 5 (bits 2 and 0): bytes 3 and 1.
        let ext = ".adp".bytes().fold(0, |h, c| fold(h, u32::from(c)));
        assert_eq!(
            hash(&mut e, "aBcDe", Some(".adp")),
            (
                bytes(b'e', b'd' + 0x80, 5, b'a' + 0x80),
                middle.wrapping_add(ext)
            )
        );
        // ".dds" is entry 3 (bits 1 and 0): bytes 0 and 1. An unknown one adds nothing.
        let ext = ".dds".bytes().fold(0, |h, c| fold(h, u32::from(c)));
        assert_eq!(
            hash(&mut e, "abc", Some(".dds")),
            (bytes(b'c' + 0x80, b'b' + 0x80, 3, b'a'), ext)
        );
        let ext = ".txt".bytes().fold(0, |h, c| fold(h, u32::from(c)));
        assert_eq!(
            hash(&mut e, "abc", Some(".txt")),
            (bytes(b'c', b'b', 3, b'a'), ext)
        );
    }

    /// `_splitpath_s`, `_strcpy_s`, `_strcat_s` and `strlen` as doubles: the path is split at
    /// its last backslash and last dot, with no drive.
    fn path_engine() -> Engine {
        let mut e = engine();
        e.register(SPLITPATH, |e, a| {
            let path = read_string(e, a[0]);
            let (directory, file) = match path.rfind('\\') {
                Some(i) => (path[..=i].to_string(), path[i + 1..].to_string()),
                None => (String::new(), path.clone()),
            };
            let (name, ext) = match file.rfind('.') {
                Some(i) => (file[..i].to_string(), file[i..].to_string()),
                None => (file.clone(), String::new()),
            };
            write_string(e, a[1], "");
            write_string(e, a[3], &directory);
            write_string(e, a[5], &name);
            write_string(e, a[7], &ext);
            Ret::default()
        });
        e.register(STRCPY_S, |e, a| {
            let s = read_string(e, a[2]);
            write_string(e, a[0], &s);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let s = read_string(e, a[0]) + &read_string(e, a[2]);
            write_string(e, a[0], &s);
            Ret::default()
        });
        e.register(STRLEN, |e, a| {
            (read_string(e, a[0]).len() as u32).into_ret()
        });
        e
    }

    fn expected(e: &mut Engine, name: &str, ext: Option<&str>) -> (u32, u32) {
        hash(e, name, ext)
    }

    #[test]
    fn path_hash_hashes_the_name_and_the_directory() {
        let mut e = path_engine();
        let path = string(&mut e, "meshes\\clutter\\cup.nif");
        let directory = e.mem.alloc(8);
        let name = e.mem.alloc(8);
        e.call(0x00af_d270, &args![path, directory, name]);
        let want_name = expected(&mut e, "cup", Some(".nif"));
        let want_directory = expected(&mut e, "meshes\\clutter", None);
        assert_eq!((e.mem.u32(name), e.mem.u32(name + 4)), want_name);
        assert_eq!(
            (e.mem.u32(directory), e.mem.u32(directory + 4)),
            want_directory
        );
        // No directory: ".".
        let path = string(&mut e, "cup.nif");
        e.call(0x00af_d270, &args![path, directory, name]);
        let want_directory = expected(&mut e, ".", None);
        assert_eq!(
            (e.mem.u32(directory), e.mem.u32(directory + 4)),
            want_directory
        );
    }

    #[test]
    fn hash_by_mode() {
        let mut e = path_engine();
        let path = string(&mut e, "textures\\Sky.dds");
        let this = e.mem.alloc(8);
        let cases = [
            (0u32, expected(&mut e, "Sky", Some(".dds"))),
            (1, expected(&mut e, "textures", None)),
            (2, expected(&mut e, "textures\\Sky.dds", None)),
        ];
        for (mode, want) in cases {
            e.call_log = Some(vec![]);
            let r = e.call(0x00af_d390, &args![this, path, mode]);
            assert_eq!(r.u32(), this);
            assert_eq!((e.mem.u32(this), e.mem.u32(this + 4)), want, "mode {mode}");
            let splits = e
                .call_log
                .take()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == SPLITPATH)
                .count();
            assert_eq!(splits, usize::from(mode != 2));
        }
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    #[test]
    fn hash_is_printed_into_the_buffer() {
        let mut e = Engine::new();
        e.register(PRINT_TO_BUFFER, |_, _| Ret::default());
        let hash = e.mem.alloc(8);
        e.mem
            .write(hash, &[0x61, 0x02, 0x63, 0x64, 0x78, 0x56, 0x34, 0x12]);
        e.call_log = Some(vec![]);
        e.call(0x00af_d860, &args![hash, 0u32]);
        e.call(0x00af_d860, &args![hash, 0x5000u32]);
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == PRINT_TO_BUFFER)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0].1,
            [
                0x5000,
                0x20,
                HASH_TEXT_FORMAT,
                0x64,
                0x63,
                0x02,
                0x61,
                0x1234_5678
            ]
        );
    }
}
