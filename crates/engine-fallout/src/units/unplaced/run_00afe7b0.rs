//! `(unplaced)/run_00afe7b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The vtable `00afe9f0` installs: its RTTI type descriptor reads
/// `.?AV?$BSSimpleArray@PBD$0EAA@@@` (`BSSimpleArray<char const *, 1024>`).
const SIMPLE_ARRAY_OF_STRINGS_VTABLE: u32 = 0x0104_72d0;
/// `006b3eb0` (thiscall, two words: size, reserved size): clears the array's
/// three words and allocates the larger of the two through the array's own
/// vtable +4.
const SIMPLE_ARRAY_INITIALISE: u32 = 0x006b_3eb0;
/// `0042fb60` (thiscall, three words: destination, source, count): moves
/// `count` words, forwards or backwards as the ranges need.
const MOVE_WORDS: u32 = 0x0042_fb60;
/// Import slot of KERNEL32 `CreateFileA` (stdcall, seven words).
const CREATE_FILE_A: u32 = 0x00fd_f088;
/// Import slot of KERNEL32 `GetLastError` (stdcall, no words).
const GET_LAST_ERROR: u32 = 0x00fd_f094;
/// `00afeef0` (thiscall on the same file object, four words: two distance
/// words, a move method, an out address): the seek of this unit, not
/// translated here.
const FILE_SEEK: u32 = 0x00af_eef0;

// Translated from 00afe9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<char const *, 1024>`
/// constructor with a size (the class from the RTTI of the vtable it
/// installs). Sets the vtable, then initialises the array with
/// `006b3eb0(size, size)`. Returns `this`.
pub fn fn_00afe9f0(e: &mut Engine, this: Ptr<BSSimpleArray>, size: u32) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_OF_STRINGS_VTABLE);
    e.call(SIMPLE_ARRAY_INITIALISE, &args![this, size, size]);
    this
}

// Translated from 00afea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes `count` 4-byte elements at `index`
/// from a `BSSimpleArray`. The elements after the gap are not shifted one by
/// one: the last `min(iSize - index - count, count)` elements (unsigned
/// arithmetic) are moved into the gap with `0042fb60`, then `iSize` is
/// lowered by `count`.
///
/// The exe also counts an index from 0 up to `count` with an empty body
/// (an element destructor that compiled to nothing); it has no effect.
pub fn fn_00afea20(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, count: u32) {
    let size = e.get(this, BSSimpleArray::iSize);
    let tail = size.wrapping_sub(index).wrapping_sub(count);
    let moved = if tail < count { tail } else { count };
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let source = buffer
        .wrapping_add(size.wrapping_mul(4))
        .wrapping_sub(moved.wrapping_mul(4));
    let destination = buffer.wrapping_add(index.wrapping_mul(4));
    e.call(MOVE_WORDS, &args![this, destination, source, moved]);
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_sub(count));
}

/// The file object `00afeac0` opens (class not named in the exe's RTTI or
/// the engine map): the access kind at +4, the creation kind at +8, a flag
/// byte at +0xc and the Windows handle at +0x10.
const FILE_ACCESS_KIND: u32 = 0x4;
const FILE_CREATION_KIND: u32 = 0x8;
const FILE_OVERLAPPED_FLAG: u32 = 0xc;
const FILE_HANDLE: u32 = 0x10;

// Translated from 00afeac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: opens `path` with `CreateFileA` and keeps the
/// handle at +0x10. The access kind (+4) 0, 1, 2 asks for `GENERIC_READ`,
/// read and write, `GENERIC_WRITE` (any other value for no access); the
/// creation kind (+8) 0 and 1 open an existing file (3), 2 creates always
/// (2), 3 truncates (5), 4 creates a new file (1), anything else passes 0.
/// The flags are `FILE_FLAG_RANDOM_ACCESS` (0x08000000), with
/// `FILE_FLAG_NO_BUFFERING | FILE_FLAG_OVERLAPPED` (0x60000000) added when
/// the byte at +0xc is set; the share mode is `FILE_SHARE_READ`.
///
/// When creation kind 1 fails, the open is retried with "create always".
/// A failed open answers `GetLastError` mapped to 0 (no error), 1
/// (`ERROR_FILE_NOT_FOUND`), 2 (`ERROR_PATH_NOT_FOUND`), 3
/// (`ERROR_WRITE_PROTECT`, 0x13) or 4 (anything else). A successful open
/// with creation kind 1 and the flag byte clear answers the seek
/// `00afeef0(0, 0, 2, &end)` (to the end of the file); otherwise 0.
pub fn fn_00afeac0(e: &mut Engine, this: Ptr, path: Ptr) -> u32 {
    let base = this.addr();
    let access = match e.mem.u32(base + FILE_ACCESS_KIND) {
        0 => 0x8000_0000u32,
        1 => 0xc000_0000,
        2 => 0x4000_0000,
        _ => 0,
    };
    let creation = match e.mem.u32(base + FILE_CREATION_KIND) {
        0 | 1 => 3u32,
        2 => 2,
        3 => 5,
        4 => 1,
        _ => 0,
    };
    let mut flags = 0x0800_0000u32;
    if e.mem.u8(base + FILE_OVERLAPPED_FLAG) != 0 {
        flags |= 0x6000_0000;
    }
    let share = 1u32;
    let handle = e
        .call(
            CREATE_FILE_A,
            &args![path, access, share, 0u32, creation, flags, 0u32],
        )
        .u32();
    e.mem.set_u32(base + FILE_HANDLE, handle);
    if e.mem.u32(base + FILE_HANDLE) == u32::MAX && e.mem.u32(base + FILE_CREATION_KIND) == 1 {
        let handle = e
            .call(
                CREATE_FILE_A,
                &args![path, access, share, 0u32, 2u32, flags, 0u32],
            )
            .u32();
        e.mem.set_u32(base + FILE_HANDLE, handle);
    }
    let mut result = 0u32;
    if e.mem.u32(base + FILE_HANDLE) == u32::MAX {
        // `switch (GetLastError())` through a byte map over 0..=0x13.
        result = match e.call(GET_LAST_ERROR, &args![]).u32() {
            0 => 0,
            2 => 1,
            3 => 2,
            0x13 => 3,
            _ => 4,
        };
    }
    if result == 0
        && e.mem.u32(base + FILE_CREATION_KIND) == 1
        && e.mem.u8(base + FILE_OVERLAPPED_FLAG) == 0
    {
        result = e.with_stack(8, |e, end| {
            e.call(FILE_SEEK, &args![this, 0u32, 0u32, 2u32, end]).u32()
        });
    }
    result
}

/// Import slot of KERNEL32 `SetFilePointerEx` (stdcall, five words: the handle, the
/// distance's low and high words, the new position's address and the move method).
const SET_FILE_POINTER_EX: u32 = 0x00fd_f140;

// Translated from 00afeef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the seek of the file object (thiscall, four words: the
/// distance's low and high words, the move method and the address of an 8-byte position;
/// `RET 0x10`). Answers 4 while the flag byte (+0xc) is set. Otherwise moves the handle
/// (+0x10) with `SetFilePointerEx` (the new position on its stack, cleared first); on
/// success copies the new position to `position` and answers 0; on failure answers
/// `GetLastError` mapped as [`fn_00afeac0`] does: 0 to 0, 2 to 1, 3 to 2, 0x13 to 3 and
/// anything else to 4 (the exe's byte map at `00afefe8`, as `match` arms).
pub fn fn_00afeef0(
    e: &mut Engine,
    this: Ptr,
    distance_low: u32,
    distance_high: u32,
    method: u32,
    position: Ptr,
) -> u32 {
    let base = this.addr();
    e.with_stack(8, |e, moved| {
        if e.mem.u8(base + FILE_OVERLAPPED_FLAG) != 0 {
            return 4;
        }
        let handle = e.mem.u32(base + FILE_HANDLE);
        let done = e
            .call(
                SET_FILE_POINTER_EX,
                &args![handle, distance_low, distance_high, moved, method],
            )
            .u32();
        if done == 0 {
            return match e.call(GET_LAST_ERROR, &args![]).u32() {
                0 => 0,
                2 => 1,
                3 => 2,
                0x13 => 3,
                _ => 4,
            };
        }
        let low = e.mem.u32(moved.addr());
        e.mem.set_u32(position.addr(), low);
        let high = e.mem.u32(moved.addr() + 4);
        e.mem.set_u32(position.addr() + 4, high);
        0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_eef0, fn_00afeef0(Ptr, u32, u32, u32, Ptr) -> u32),
        entry!(
            0x00af_e9f0,
            fn_00afe9f0(Ptr<BSSimpleArray>, u32) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00af_ea20, fn_00afea20(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x00af_eac0, fn_00afeac0(Ptr, Ptr) -> u32),
        entry!(0x00af_e7b0, fn_00afe7b0(u32, u32) -> u32),
        entry!(0x00af_e7d0, fn_00afe7d0(u32, u32, u32) -> u32),
        entry!(0x00af_e830, fn_00afe830(u32, u32, u32)),
        entry!(0x00af_ecd0, fn_00afecd0(Ptr, u32, u32, u32, Ptr) -> u32),
        entry!(0x00af_ede0, fn_00afede0(Ptr, u32, u32, u32, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
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
    fn constructor_sets_the_vtable_and_initialises_with_the_size_twice() {
        let mut e = Engine::new();
        e.register(SIMPLE_ARRAY_INITIALISE, |_, _| Ret::default());
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        let result = e.call(0x00af_e9f0, &args![array, 7u32]).u32();
        assert_eq!(result, array.addr());
        assert_eq!(e.mem.u32(array.addr()), SIMPLE_ARRAY_OF_STRINGS_VTABLE);
        assert_eq!(
            calls_to(&e, SIMPLE_ARRAY_INITIALISE),
            vec![vec![array.addr(), 7, 7]]
        );
    }

    fn array(e: &mut Engine, size: u32) -> (Ptr<BSSimpleArray>, u32) {
        e.register(MOVE_WORDS, |_, _| Ret::default());
        let array: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(0x40);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, size);
        (array, buffer)
    }

    #[test]
    fn remove_moves_the_last_elements_into_the_gap() {
        let mut e = Engine::new();
        let (array, buffer) = array(&mut e, 10);
        e.call_log = Some(vec![]);
        // Index 2, count 3: the tail after the gap is 5 long, so 3 move.
        e.call(0x00af_ea20, &args![array, 2u32, 3u32]);
        assert_eq!(
            calls_to(&e, MOVE_WORDS),
            vec![vec![array.addr(), buffer + 8, buffer + 28, 3]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 7);
    }

    #[test]
    fn remove_near_the_end_moves_only_the_short_tail() {
        let mut e = Engine::new();
        let (array, buffer) = array(&mut e, 6);
        e.call_log = Some(vec![]);
        // Index 1, count 4: one element follows the gap.
        e.call(0x00af_ea20, &args![array, 1u32, 4u32]);
        assert_eq!(
            calls_to(&e, MOVE_WORDS),
            vec![vec![array.addr(), buffer + 4, buffer + 20, 1]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
    }

    /// A file object with the given kinds and flag byte; `CreateFileA`
    /// answers the handles in turn, `GetLastError` answers `error` and the
    /// seek 0x55.
    fn open(access: u32, creation: u32, flag: u8, handles: &[u32], error: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object + FILE_ACCESS_KIND, access);
        e.mem.set_u32(object + FILE_CREATION_KIND, creation);
        e.mem.set_u8(object + FILE_OVERLAPPED_FLAG, flag);
        let queue = std::rc::Rc::new(std::cell::RefCell::new(handles.to_vec()));
        e.register_double(CREATE_FILE_A, move |_, _| {
            queue.borrow_mut().remove(0).into_ret()
        });
        e.register_double(GET_LAST_ERROR, move |_, _| error.into_ret());
        e.register(FILE_SEEK, |_, _| 0x55u32.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x00af_eac0, &args![object, 0x1234u32]).u32();
        assert_eq!(e.mem.u32(object + FILE_HANDLE), *handles.last().unwrap());
        (e, result)
    }

    #[test]
    fn open_for_reading_passes_the_access_creation_and_flags() {
        let (e, result) = open(0, 0, 0, &[0x40], 0);
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&e, CREATE_FILE_A),
            vec![vec![0x1234, 0x8000_0000, 1, 0, 3, 0x0800_0000, 0]]
        );
        assert!(calls_to(&e, FILE_SEEK).is_empty());
    }

    #[test]
    fn the_kinds_map_to_access_and_creation_values() {
        for (access, creation, want_access, want_creation) in [
            (1, 2, 0xc000_0000u32, 2u32),
            (2, 3, 0x4000_0000, 5),
            (3, 4, 0, 1),
            (0, 7, 0x8000_0000, 0),
        ] {
            let (e, _) = open(access, creation, 1, &[0x40], 0);
            assert_eq!(
                calls_to(&e, CREATE_FILE_A),
                vec![vec![
                    0x1234,
                    want_access,
                    1,
                    0,
                    want_creation,
                    0x6800_0000,
                    0
                ]]
            );
        }
    }

    #[test]
    fn creation_kind_one_retries_and_then_seeks_to_the_end() {
        let (e, result) = open(1, 1, 0, &[u32::MAX, 0x40], 0);
        assert_eq!(result, 0x55);
        let opens = calls_to(&e, CREATE_FILE_A);
        assert_eq!(opens.len(), 2);
        assert_eq!(opens[1], vec![0x1234, 0xc000_0000, 1, 0, 2, 0x0800_0000, 0]);
        let seeks = calls_to(&e, FILE_SEEK);
        assert_eq!(seeks.len(), 1);
        assert_eq!(&seeks[0][1..4], &[0, 0, 2]);
    }

    #[test]
    fn creation_kind_one_with_the_flag_does_not_seek() {
        let (e, result) = open(1, 1, 1, &[0x40], 0);
        assert_eq!(result, 0);
        assert!(calls_to(&e, FILE_SEEK).is_empty());
    }

    #[test]
    fn a_failed_open_maps_the_last_error() {
        for (error, want) in [(0u32, 0u32), (2, 1), (3, 2), (0x13, 3), (5, 4), (0x14, 4)] {
            let (_, result) = open(0, 0, 0, &[u32::MAX], error);
            assert_eq!(result, want, "error {error:#x}");
        }
        // A failure with no error code still counts as success for the seek.
        let (e, result) = open(0, 1, 0, &[u32::MAX, u32::MAX], 0);
        assert_eq!(result, 0x55);
        assert_eq!(calls_to(&e, FILE_SEEK).len(), 1);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn seek_copies_the_new_position() {
        let mut e = Engine::new();
        e.register(SET_FILE_POINTER_EX, |e, a| {
            // The new position is the distance itself.
            e.mem.set_u32(a[3], a[1]);
            e.mem.set_u32(a[3] + 4, a[2]);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + FILE_HANDLE, 0x44);
        let position = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00af_eef0, &args![this, 0x10u32, 0x2u32, 1u32, position])
            .u32();
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(position), 0x10);
        assert_eq!(e.mem.u32(position + 4), 2);
        let log = e.call_log.take().unwrap();
        let call = log.iter().find(|(a, _)| *a == SET_FILE_POINTER_EX).unwrap();
        assert_eq!(
            [call.1[0], call.1[1], call.1[2], call.1[4]],
            [0x44, 0x10, 2, 1]
        );
    }

    #[test]
    fn seek_maps_the_error_and_refuses_with_the_flag() {
        let mut e = Engine::new();
        e.register(SET_FILE_POINTER_EX, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x14);
        let position = e.mem.alloc(8);
        for (error, want) in [
            (0u32, 0u32),
            (1, 4),
            (2, 1),
            (3, 2),
            (5, 4),
            (0x13, 3),
            (0x14, 4),
        ] {
            e.register_double(GET_LAST_ERROR, move |_, _| error.into_ret());
            let result = e
                .call(0x00af_eef0, &args![this, 0u32, 0u32, 0u32, position])
                .u32();
            assert_eq!(result, want, "error {error:#x}");
        }
        assert_eq!(e.mem.u32(position), 0);
        e.mem.set_u8(this + FILE_OVERLAPPED_FLAG, 1);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00af_eef0, &args![this, 0u32, 0u32, 0u32, position])
            .u32();
        assert_eq!(result, 4);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

/// `memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// Finds the archive holding a file (`00af6160`, cdecl: the name, the
/// flags; placed in `BSSystem/archive.cpp` by the engine map).
const FIND_ARCHIVE: u32 = 0x00af_6160;
/// `00b01b60` (cdecl: a name, the 8-byte result; placed in `BSSystem` by the
/// engine map).
const FILE_INFO: u32 = 0x00b0_1b60;
/// `00ec7f1e` (cdecl, nine words, `_splitpath_s`'s shape: the path, then
/// four buffer and size pairs; the engine map names it
/// `NiFilename::Splitpath`).
const SPLIT_PATH: u32 = 0x00ec_7f1e;
/// `strlen`, `strncmp`, `strcat_s`, `strcpy_s` (CRT, cdecl).
const STRLEN: u32 = 0x00ec_6130;
const STRNCMP: u32 = 0x00ec_8a19;
const STRCAT_S: u32 = 0x00ec_6bd2;
const STRCPY_S: u32 = 0x00ec_65a6;
/// `"..\\"`, `"\\"` and the empty string the code falls back to.
const PARENT_PREFIX: u32 = 0x010a_46c4;
const BACKSLASH: u32 = 0x010a_46c8;
const EMPTY_010A46BE: u32 = 0x010a_46be;
/// The size of the path buffers.
const PATH_SIZE: u32 = 0x104;
/// Import slots of `ReadFile` and `WriteFile` (stdcall, five words: the
/// handle, the buffer, the size, the count's address and the overlapped
/// pointer).
const READ_FILE: u32 = 0x00fd_f080;
const WRITE_FILE: u32 = 0x00fd_f0dc;

// Translated from 00afe7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00afe7d0`] with the flags 0xFFFF (cdecl; no caller or vtable in the
/// exe), whose answer it passes on.
pub fn fn_00afe7b0(e: &mut Engine, name: u32, out: u32) -> u32 {
    fn_00afe7d0(e, name, out, 0xffff)
}

// Translated from 00afe7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks a file up (cdecl; called by `CompileVertexShaderHLSL`,
/// `CompilePixelShaderHLSL` and [`fn_00afe7b0`]): clears the 8 bytes at
/// `out`, finds the archive holding `name` (`00af6160` with `flags`); when
/// there is one, its vtable slot +0x18 answers the name to use instead;
/// then `00b01b60(name, out)`, whose answer it passes on.
pub fn fn_00afe7d0(e: &mut Engine, name: u32, out: u32, flags: u32) -> u32 {
    e.call(MEMSET, &[out, 0, 8]);
    let archive = e.call(FIND_ARCHIVE, &[name, flags]).u32();
    let mut name = name;
    if archive != 0 {
        name = e.vcall(archive, 0x18, &[]).u32();
    }
    e.call(FILE_INFO, &[name, out]).u32()
}

// Translated from 00afe830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves a path relative to the directory of `path` into `out` (cdecl,
/// a `PATH_SIZE`-byte buffer; called by `00be1eed`'s code): takes the
/// directory part (`00ec7f1e`) without a trailing backslash; each leading
/// `"..\\"` of `relative` cuts the directory at its last backslash past the
/// first character (at its start when there is none) and drops the prefix;
/// with at least one, `out` is the directory, `"\\"` and the rest of
/// `relative` (`strcat_s`), otherwise a copy of `relative` (`strcpy_s`).
/// The trailing-backslash test of an empty directory reads the byte before
/// the buffer, the top byte of the exe's local that holds its address; the
/// frame here keeps that local just before the buffer too.
// Translation incomplete: 00afe830 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00afe830(e: &mut Engine, path: u32, relative: u32, out: u32) {
    e.mem.set_u8(out, 0);
    e.with_stack(4 + PATH_SIZE, |e, frame| {
        let holder = frame.addr();
        let directory = holder + 4;
        e.call(SPLIT_PATH, &[path, 0, 0, directory, PATH_SIZE, 0, 0, 0, 0]);
        let mut text = directory;
        e.mem.set_u32(holder, text);
        let length = e.call(STRLEN, &[text]).u32();
        let last = text.wrapping_add(length).wrapping_sub(1);
        if e.mem.u8(last) == b'\\' {
            e.mem.set_u8(last, 0);
        }
        let mut relative = relative;
        let mut climbed = false;
        while e.call(STRNCMP, &[relative, PARENT_PREFIX, 3]).u32() == 0 {
            climbed = true;
            let mut cut = text;
            let length = e.call(STRLEN, &[text]).u32();
            let mut index = length.wrapping_sub(1) as i32;
            while index > 0 {
                if e.mem.u8(text.wrapping_add(index as u32)) == b'\\' {
                    cut = text.wrapping_add(index as u32);
                    break;
                }
                index -= 1;
            }
            if cut != 0 {
                e.mem.set_u8(cut, 0);
                relative = relative.wrapping_add(3);
            } else {
                text = EMPTY_010A46BE;
                e.mem.set_u32(holder, text);
            }
        }
        if climbed {
            e.call(STRCAT_S, &[out, PATH_SIZE, text]);
            e.call(STRCAT_S, &[out, PATH_SIZE, BACKSLASH]);
            e.call(STRCAT_S, &[out, PATH_SIZE, relative]);
        } else {
            e.call(STRCPY_S, &[out, PATH_SIZE, relative]);
        }
    });
}

/// A failed transfer's `GetLastError` mapped as the file object's other
/// methods map it (the exe's byte map over 0..=0x13, as `match` arms): 0 to
/// 0, 2 to 1, 3 to 2, 0x13 to 3, anything else to 4.
fn transfer_error(e: &mut Engine) -> u32 {
    match e.call(GET_LAST_ERROR, &[]).u32() {
        0 => 0,
        2 => 1,
        3 => 2,
        0x13 => 3,
        _ => 4,
    }
}

/// `ReadFile` or `WriteFile` on the file object's handle: 4 while its flag
/// byte (+0xc) is set; on success the count (zero-extended to 8 bytes) at
/// `count_out` and 0; on failure [`transfer_error`].
fn transfer(e: &mut Engine, api: u32, this: Ptr, buffer: u32, size: u32, count_out: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u8(base + FILE_OVERLAPPED_FLAG) != 0 {
        return 4;
    }
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 0);
        let handle = e.mem.u32(base + FILE_HANDLE);
        let done = e.call(api, &[handle, buffer, size, count.addr(), 0]).u32();
        if done == 0 {
            return transfer_error(e);
        }
        let moved = e.mem.u32(count.addr());
        e.mem.set_u32(count_out.addr(), moved);
        e.mem.set_u32(count_out.addr() + 4, 0);
        0
    })
}

// Translated from 00afecd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The file object's read (thiscall, four words, the third never read;
/// called by `DoRead`): `ReadFile(handle, buffer, size, &count, NULL)`; 4
/// while the flag byte (+0xc) is set; on success the count as 8 bytes at
/// `count_out` and 0; on failure `GetLastError` mapped as
/// `00afeac0` does.
pub fn fn_00afecd0(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    _unused_3: u32,
    count_out: Ptr,
) -> u32 {
    transfer(e, READ_FILE, this, buffer, size, count_out)
}

// Translated from 00afede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The file object's write (thiscall, four words, the third never read;
/// called by `DoWrite`): `WriteFile(handle, buffer, size, &count, NULL)`,
/// answering as [`fn_00afecd0`] does.
pub fn fn_00afede0(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    _unused_3: u32,
    count_out: Ptr,
) -> u32 {
    transfer(e, WRITE_FILE, this, buffer, size, count_out)
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    #[test]
    fn lookups_use_the_archive_name() {
        for archive in [0u32, 1] {
            const NAME_SLOT: u32 = 0x00de_ad18;
            let mut e = Engine::new();
            e.register(MEMSET, |_, a| a[0].into_ret());
            e.register(NAME_SLOT, |_, _| 0x4444u32.into_ret());
            e.register(FILE_INFO, |_, a| (a[0] + 1).into_ret());
            e.put_vtable(0x0074_0000, &[0, 0, 0, 0, 0, 0, NAME_SLOT]);
            let found = if archive != 0 {
                let a = e.mem.alloc(4);
                e.mem.set_u32(a, 0x0074_0000);
                a
            } else {
                0
            };
            e.register_double(FIND_ARCHIVE, move |_, _| found.into_ret());
            e.call_log = Some(vec![]);
            let answer = e.call(0x00af_e7b0, &[0x3000, 0x5000]).u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (MEMSET, vec![0x5000, 0, 8]));
            assert_eq!(log[2], (FIND_ARCHIVE, vec![0x3000, 0xffff]));
            if archive != 0 {
                assert_eq!(answer, 0x4445);
            } else {
                assert_eq!(answer, 0x3001);
            }
        }
    }

    /// An engine with the string functions doubled on real memory, the
    /// directory part given by `directory`.
    fn path_engine(directory: &'static [u8]) -> Engine {
        let mut e = Engine::new();
        e.map(0x010a_4000, 0x1000);
        e.mem.write(PARENT_PREFIX, b"..\\\0");
        e.mem.write(BACKSLASH, b"\\\0");
        e.register_double(SPLIT_PATH, move |e, a| {
            e.mem.write(a[3], directory);
            0u32.into_ret()
        });
        e.register(STRLEN, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            n.into_ret()
        });
        e.register(STRNCMP, |e, a| {
            let same = (0..a[2]).all(|i| e.mem.u8(a[0] + i) == e.mem.u8(a[1] + i));
            u32::from(!same).into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let mut i = 0;
            loop {
                let b = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, b);
                if b == 0 {
                    break;
                }
                i += 1;
            }
            0u32.into_ret()
        });
        e.register(STRCAT_S, |e, a| {
            let mut end = a[0];
            while e.mem.u8(end) != 0 {
                end += 1;
            }
            let mut i = 0;
            loop {
                let b = e.mem.u8(a[2] + i);
                e.mem.set_u8(end + i, b);
                if b == 0 {
                    break;
                }
                i += 1;
            }
            0u32.into_ret()
        });
        e
    }

    fn resolve(directory: &'static [u8], relative: &[u8]) -> Vec<u8> {
        let mut e = path_engine(directory);
        let rel = e.mem.alloc(0x40);
        e.mem.write(rel, relative);
        let out = e.mem.alloc(PATH_SIZE);
        e.mem.set_u8(out, 0x55);
        e.call(0x00af_e830, &[0x2000, rel, out]);
        let mut text = vec![];
        let mut i = 0;
        while e.mem.u8(out + i) != 0 {
            text.push(e.mem.u8(out + i));
            i += 1;
        }
        text
    }

    #[test]
    fn relative_paths_climb_the_directory() {
        assert_eq!(resolve(b"\\a\\b\\\0", b"c.txt\0"), b"c.txt".to_vec());
        assert_eq!(
            resolve(b"\\a\\b\\\0", b"..\\c.txt\0"),
            b"\\a\\c.txt".to_vec()
        );
        assert_eq!(
            resolve(b"\\a\\b\\\0", b"..\\..\\c.txt\0"),
            b"\\c.txt".to_vec()
        );
        // Without a backslash past the first character the cut is at the
        // start.
        assert_eq!(resolve(b"ab\0", b"..\\c\0"), b"\\c".to_vec());
    }

    fn file(e: &mut Engine, flag: u8) -> Ptr {
        let this = Ptr::new(e.mem.alloc(0x14));
        e.mem.set_u8(this.addr() + FILE_OVERLAPPED_FLAG, flag);
        e.mem.set_u32(this.addr() + FILE_HANDLE, 0x44);
        this
    }

    #[test]
    fn reads_and_writes_report_the_count() {
        for (addr, api) in [(0x00af_ecd0u32, READ_FILE), (0x00af_ede0, WRITE_FILE)] {
            let mut e = Engine::new();
            e.register(api, |e, a| {
                e.mem.set_u32(a[3], a[2] - 1);
                1u32.into_ret()
            });
            let this = file(&mut e, 0);
            let out = e.mem.alloc(8);
            e.mem.set_u32(out + 4, 0x77);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &[this.addr(), 0x9000, 16, 5, out]).u32(), 0);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1].1[..3], [0x44, 0x9000, 16]);
            assert_eq!(log[1].1[4], 0);
            assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (15, 0));
            // The flag byte answers 4 without a call.
            let this = file(&mut e, 1);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &[this.addr(), 0x9000, 16, 5, out]).u32(), 4);
            assert_eq!(e.call_log.take().unwrap().len(), 1);
        }
    }

    #[test]
    fn failed_transfers_map_the_error() {
        for (error, answer) in [(0u32, 0u32), (2, 1), (3, 2), (0x13, 3), (5, 4), (0x14, 4)] {
            let mut e = Engine::new();
            e.register(READ_FILE, |_, _| 0u32.into_ret());
            e.register_double(GET_LAST_ERROR, move |_, _| error.into_ret());
            let this = file(&mut e, 0);
            let out = e.mem.alloc(8);
            assert_eq!(
                e.call(0x00af_ecd0, &[this.addr(), 0, 1, 0, out]).u32(),
                answer
            );
        }
    }
}
