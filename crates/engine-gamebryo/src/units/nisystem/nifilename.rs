//! `NiSystem/nifilename.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// CRT `strncpy_s` (`00ec8d5f`, `cdecl`: destination, size, source, count).
const STRNCPY_S: u32 = 0x00ec_8d5f;
/// CRT `_ismbblead` (`00ed5c8d`, `cdecl`, the character as a signed byte widened).
const ISMBBLEAD: u32 = 0x00ed_5c8d;
/// CRT `_errno` (`00ec85e3`): the address of the thread's `errno`.
const ERRNO: u32 = 0x00ec_85e3;
/// CRT `_invalid_parameter` (`00ec7c30`, `cdecl`, five words).
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
/// `EINVAL` and `ERANGE`.
const EINVAL: i32 = 0x16;
const ERANGE: i32 = 0x22;

/// One of the four output buffers: its address and size.
#[derive(Clone, Copy)]
struct Part {
    buffer: u32,
    size: u32,
}

impl Part {
    /// The pair check at the top: a null buffer needs size 0, a buffer a size above 0.
    fn valid(self) -> bool {
        (self.buffer == 0) == (self.size == 0)
    }

    /// `strncpy_s(buffer, size, source, count)`; the result is not used.
    fn copy(self, e: &mut Engine, source: u32, count: u32) {
        e.call(STRNCPY_S, &args![self.buffer, self.size, source, count]);
    }

    /// The cleanup's `*buffer = 0` when the buffer is there and its size above 0.
    fn clear_if_sized(self, e: &mut Engine) {
        if self.buffer != 0 && self.size != 0 {
            e.mem.set_u8(self.buffer, 0);
        }
    }
}

// Translated from 00ec7f1e (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiFilename::Splitpath` in the engine map; the body is the CRT's `_splitpath_s`
/// (`cdecl`, nine words: path, drive, drive size, directory, directory size, file name, file
/// name size, extension, extension size). Answers 0, `EINVAL` (0x16) or `ERANGE` (0x22).
///
/// A null path, or a buffer whose size disagrees with it (null with a size, or a buffer of
/// size 0), is invalid. A drive is `path[0..2]` when the character after the first (or the
/// first, for a one-character path) is `':'`; a drive buffer smaller than 3 is a range error.
/// The scan steps over the second byte of each double-byte lead character (`_ismbblead`,
/// which can step over the terminator as the exe does) and remembers the character after the
/// last `/` or `\` and the last `.`. The directory runs up to that character, the file name
/// up to the last dot (when the dot is in the name part) or the end, the extension from the
/// dot. A part that does not fit its buffer (length at least the size) is a range error.
/// On any error every sized buffer gets an empty string and `errno` is set; an invalid
/// argument also calls `_invalid_parameter` (`00ec7c30`) with five zeros.
#[allow(clippy::too_many_arguments)] // the CRT's nine stack words
pub fn fn_00ec7f1e(
    e: &mut Engine,
    path: u32,
    drive: u32,
    drive_size: u32,
    dir: u32,
    dir_size: u32,
    fname: u32,
    fname_size: u32,
    ext: u32,
    ext_size: u32,
) -> i32 {
    let drive = Part {
        buffer: drive,
        size: drive_size,
    };
    let dir = Part {
        buffer: dir,
        size: dir_size,
    };
    let fname = Part {
        buffer: fname,
        size: fname_size,
    };
    let ext = Part {
        buffer: ext,
        size: ext_size,
    };
    let invalid = path == 0 || !drive.valid() || !dir.valid() || !fname.valid() || !ext.valid();
    // `[EBP + 8]`: the path, advanced past the drive and the directory as the body goes.
    let mut current = path;
    if !invalid && split(e, &mut current, drive, dir, fname, ext) {
        return 0;
    }
    drive.clear_if_sized(e);
    dir.clear_if_sized(e);
    fname.clear_if_sized(e);
    ext.clear_if_sized(e);
    let errno = e.call(ERRNO, &args![]).u32();
    if current == 0 || invalid {
        e.mem.set_i32(errno, EINVAL);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        EINVAL
    } else {
        e.mem.set_i32(errno, ERANGE);
        ERANGE
    }
}

/// The body after the argument checks (`00ec7f71` on): true when it is done, false when it
/// goes to the cleanup (`00ec8095`) with a range error.
fn split(e: &mut Engine, path: &mut u32, drive: Part, dir: Part, fname: Part, ext: Part) -> bool {
    // At most one step: the second character, or the terminator of an empty or
    // one-character path.
    let mut cursor = *path;
    let mut steps = 1u32;
    while e.mem.u8(cursor) != 0 {
        steps -= 1;
        cursor = cursor.wrapping_add(1);
        if steps == 0 {
            break;
        }
    }
    if e.mem.u8(cursor) == b':' {
        if drive.buffer != 0 {
            if drive.size < 3 {
                return false;
            }
            drive.copy(e, *path, 2);
        }
        *path = cursor.wrapping_add(1);
    } else if drive.buffer != 0 {
        e.mem.set_u8(drive.buffer, 0);
    }

    let mut cursor = *path;
    let mut after_slash = 0u32;
    let mut dot = 0u32;
    if e.mem.u8(cursor) != 0 {
        loop {
            let character = e.mem.u8(cursor) as i8 as i32;
            if e.call(ISMBBLEAD, &args![character]).u32() != 0 {
                cursor = cursor.wrapping_add(1);
            } else {
                match e.mem.u8(cursor) {
                    b'/' | b'\\' => after_slash = cursor.wrapping_add(1),
                    b'.' => dot = cursor,
                    _ => {}
                }
            }
            cursor = cursor.wrapping_add(1);
            if e.mem.u8(cursor) == 0 {
                break;
            }
        }
    }
    if after_slash != 0 {
        if dir.buffer != 0 {
            let length = after_slash.wrapping_sub(*path);
            if dir.size <= length {
                return false;
            }
            dir.copy(e, *path, length);
        }
        *path = after_slash;
    } else if dir.buffer != 0 {
        e.mem.set_u8(dir.buffer, 0);
    }

    if dot == 0 || dot < *path {
        if fname.buffer != 0 {
            let length = cursor.wrapping_sub(*path);
            if fname.size <= length {
                return false;
            }
            fname.copy(e, *path, length);
        }
        if ext.buffer != 0 {
            e.mem.set_u8(ext.buffer, 0);
        }
        return true;
    }
    if fname.buffer != 0 {
        let length = dot.wrapping_sub(*path);
        if fname.size <= length {
            return false;
        }
        fname.copy(e, *path, length);
    }
    if ext.buffer == 0 {
        return true;
    }
    let length = cursor.wrapping_sub(dot);
    if ext.size <= length {
        return false;
    }
    ext.copy(e, dot, length);
    true
}

// Translated from 00aa27d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFilename::NiFilename` (Xbox PDB), `thiscall`, one stack word (the path):
/// clears the first byte of `m_acSubDir` (+0x303) and splits the path with
/// `_splitpath_s` (`00ec7f1e`, this unit) into `m_acDrive` (+0x100, size 3),
/// `m_acDir` (+0, size 0x100), `m_acFname` (+0x203, size 0x100) and `m_acExt`
/// (+0x103, size 0x100); the split's result is not used. Answers `this`.
pub fn ni_filename_ni_filename(e: &mut Engine, this: Ptr, path: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u8(t + 0x303, 0);
    fn_00ec7f1e(
        e,
        path.addr(),
        t + 0x100,
        3,
        t,
        0x100,
        t + 0x203,
        0x100,
        t + 0x103,
        0x100,
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ec_7f1e,
            fn_00ec7f1e(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> i32
        ),
        entry!(0x00aa_27d0, ni_filename_ni_filename(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles: `strncpy_s` copies `count` bytes and a terminator, `_ismbblead` is true for
    /// 0x81 only, `errno` lives in a test block.
    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(STRNCPY_S, |e, a| {
            for i in 0..a[3] {
                let byte = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            e.mem.set_u8(a[0] + a[3], 0);
            0u32.into_ret()
        });
        e.register(ISMBBLEAD, |_, a| {
            u32::from(a[0] as i8 as u8 == 0x81).into_ret()
        });
        let errno = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        (e, errno)
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        for (i, b) in s.iter().enumerate() {
            e.mem.set_u8(p + i as u32, *b);
        }
        p
    }

    fn read(e: &Engine, p: u32) -> Vec<u8> {
        let mut out = vec![];
        let mut i = 0;
        while e.mem.u8(p + i) != 0 {
            out.push(e.mem.u8(p + i));
            i += 1;
        }
        out
    }

    /// Four 32-byte buffers filled with 'x', and the call with sizes `sizes`.
    fn split_with(e: &mut Engine, path: u32, sizes: [u32; 4]) -> (i32, [u32; 4]) {
        let mut buffers = [0u32; 4];
        for buffer in buffers.iter_mut() {
            *buffer = e.mem.alloc(32);
            for i in 0..31 {
                e.mem.set_u8(*buffer + i, b'x');
            }
        }
        let result = e
            .call(
                0x00ec_7f1e,
                &args![
                    path, buffers[0], sizes[0], buffers[1], sizes[1], buffers[2], sizes[2],
                    buffers[3], sizes[3]
                ],
            )
            .i32();
        (result, buffers)
    }

    #[test]
    fn splits_drive_directory_name_and_extension() {
        let (mut e, _) = rig();
        let path = text(&mut e, b"C:\\data\\meshes/a.b.nif");
        let (result, b) = split_with(&mut e, path, [32; 4]);
        assert_eq!(result, 0);
        assert_eq!(read(&e, b[0]), b"C:");
        assert_eq!(read(&e, b[1]), b"\\data\\meshes/");
        assert_eq!(read(&e, b[2]), b"a.b");
        assert_eq!(read(&e, b[3]), b".nif");
    }

    #[test]
    fn a_name_without_extension_or_directory() {
        let (mut e, _) = rig();
        // The only dot is in the directory: the name runs to the end, no extension.
        let path = text(&mut e, b"dir.d/name");
        let (result, b) = split_with(&mut e, path, [32; 4]);
        assert_eq!(result, 0);
        assert_eq!(read(&e, b[0]), b"");
        assert_eq!(read(&e, b[1]), b"dir.d/");
        assert_eq!(read(&e, b[2]), b"name");
        assert_eq!(read(&e, b[3]), b"");
        let path = text(&mut e, b"plain");
        let (_, b) = split_with(&mut e, path, [32; 4]);
        assert_eq!(read(&e, b[1]), b"");
        assert_eq!(read(&e, b[2]), b"plain");
        // An empty path: everything empty.
        let path = text(&mut e, b"");
        let (result, b) = split_with(&mut e, path, [32; 4]);
        assert_eq!(result, 0);
        for buffer in b {
            assert_eq!(read(&e, buffer), b"");
        }
    }

    #[test]
    fn a_lead_byte_hides_the_next_character() {
        let (mut e, _) = rig();
        // 0x81 '\\' is one double-byte character: no directory there.
        let path = text(&mut e, b"a\x81\\b.c");
        let (result, b) = split_with(&mut e, path, [32; 4]);
        assert_eq!(result, 0);
        assert_eq!(read(&e, b[1]), b"");
        assert_eq!(read(&e, b[2]), b"a\x81\\b");
        assert_eq!(read(&e, b[3]), b".c");
    }

    #[test]
    fn null_buffers_are_skipped() {
        let (mut e, _) = rig();
        let path = text(&mut e, b"C:d/n.e");
        let result = e
            .call(
                0x00ec_7f1e,
                &args![path, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .i32();
        assert_eq!(result, 0);
    }

    #[test]
    fn invalid_arguments_set_einval() {
        let (mut e, errno) = rig();
        let path = text(&mut e, b"a/b.c");
        // A buffer of size 0.
        e.call_log = Some(vec![]);
        let (result, b) = split_with(&mut e, path, [32, 0, 32, 32]);
        assert_eq!(result, EINVAL);
        assert_eq!(e.mem.i32(errno), EINVAL);
        assert_eq!(read(&e, b[0]), b"");
        // The size-0 buffer is left alone.
        assert_eq!(e.mem.u8(b[1]), b'x');
        let log = e.call_log.take().unwrap();
        let invalid: Vec<_> = log.iter().filter(|c| c.0 == INVALID_PARAMETER).collect();
        assert_eq!(invalid[0].1, vec![0; 5]);
        // A null path.
        let (result, _) = split_with(&mut e, 0, [32; 4]);
        assert_eq!(result, EINVAL);
        // A null buffer with a size.
        let result = e
            .call(
                0x00ec_7f1e,
                &args![path, 0u32, 3u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .i32();
        assert_eq!(result, EINVAL);
    }

    #[test]
    fn parts_that_do_not_fit_set_erange() {
        let (mut e, errno) = rig();
        let path = text(&mut e, b"C:dir/name.ext");
        for sizes in [
            [2, 32, 32, 32],
            [32, 4, 32, 32],
            [32, 32, 4, 32],
            [32, 32, 32, 4],
        ] {
            e.call_log = Some(vec![]);
            let (result, b) = split_with(&mut e, path, sizes);
            assert_eq!(result, ERANGE, "{sizes:?}");
            assert_eq!(e.mem.i32(errno), ERANGE);
            for buffer in b {
                assert_eq!(read(&e, buffer), b"");
            }
            let log = e.call_log.take().unwrap();
            assert!(log.iter().all(|c| c.0 != INVALID_PARAMETER));
        }
        // A name without extension that does not fit.
        let path = text(&mut e, b"name");
        let (result, _) = split_with(&mut e, path, [32, 32, 4, 32]);
        assert_eq!(result, ERANGE);
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn ni_filename_ni_filename_splits_into_the_four_parts() {
        let mut e = Engine::new();
        e.register(STRNCPY_S, |_, _| Ret::default());
        e.register(ISMBBLEAD, |_, _| Ret::default());
        let this = e.mem.alloc(0x404);
        e.mem.set_u8(this + 0x303, b'x');
        let path = e.mem.alloc(8);
        e.mem.set_cstr(path, b"ab");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_27d0, &args![this, path]).u32(), this);
        assert_eq!(e.mem.u8(this + 0x303), 0);
        let copies: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == STRNCPY_S)
            .map(|c| c.1)
            .collect();
        // "ab" has no drive, no directory and no extension: only the name is copied.
        assert_eq!(copies, vec![vec![this + 0x203, 0x100, path, 2]]);
    }
}
