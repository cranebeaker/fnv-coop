//! `BSSystem/filefinder.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Archive lookup: `(path, extra)`, cdecl, returns whether the file is in an
/// archive (the engine map places it in `BSSystem/archive.cpp`).
const ARCHIVE_CONTAINS: u32 = 0x00af_6320;
/// Thiscall on the finder with `(flags, extra)`: whether the loose-file
/// search applies (`00b01bc0`, 26 bytes).
const LOOKS_IN_LOOSE_FILES: u32 = 0x00b0_1bc0;
/// Thiscall on the finder with `(flags, extra)`: whether the search path
/// list applies (`00b01be0`, 26 bytes).
const LOOKS_IN_SEARCH_PATHS: u32 = 0x00b0_1be0;
/// `_access(path, mode)` (cdecl), -1 on failure.
const ACCESS: u32 = 0x00ec_bf05;
/// `strlen` (cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// `strcpy_s(dest, size, source)` (cdecl).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `strcat_s(dest, size, source)` (cdecl).
const STRCAT_S: u32 = 0x00ec_6bd2;
/// `_strnicmp(a, b, count)` (cdecl, folded body `00ec7ec0`).
const STRNICMP: u32 = 0x00ec_7ec0;
/// A byte global; the search paths are only used when it is set and
/// [`SEARCH_PATHS_DISABLED`] is clear.
const SEARCH_PATHS_ENABLED: u32 = 0x011a_cdf8;
/// A byte global; when set the flags are cleared before the lookups.
const SEARCH_PATHS_DISABLED: u32 = 0x011a_cdf9;
/// Size of the path buffer the function builds a candidate in.
const PATH_BUFFER_SIZE: u32 = 0x104;

layout! {
    /// `FileFinder` (Xbox PDB), 0x10 bytes: the search-path array
    /// (`PathA`, a `BSSimpleArray<char const *,1024>`) at +0.
    pub struct FileFinder: 0x10 {
        /// `PathA.pBuffer` (PC +4): the array of path prefixes.
        0x04 pBuffer: u32,
        /// `PathA.iSize` (PC +8): number of entries.
        0x08 iSize: u32,
    }
}

// Translated from 00afe220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afe220 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// File lookup on a finder (the engine map gives no name): `path` is a
/// `char[0x104]` buffer, `flags` and `extra` are passed on to the lookup
/// helpers. Returns 2 when the archive lookup knows the path (skipped when
/// `flags` has bit 0), 1 when the path exists as given or, after a prefix
/// from the search-path array was prepended, as one of those (the path
/// buffer is then replaced by the prefixed path), 0 when nothing is found.
///
/// The decompiler drops the return values; they are read from the
/// disassembly.
pub fn fn_00afe220(
    e: &mut Engine,
    this: Ptr<FileFinder>,
    path: Ptr,
    flags: u32,
    extra: u32,
) -> u32 {
    if flags & 1 == 0 && e.call(ARCHIVE_CONTAINS, &args![path, extra]).bool() {
        return 2;
    }
    let mut flags = flags;
    if e.global::<u8>(SEARCH_PATHS_ENABLED) == 0 || e.global::<u8>(SEARCH_PATHS_DISABLED) != 0 {
        flags = 0;
    }
    if e.call(LOOKS_IN_LOOSE_FILES, &args![this, flags, extra])
        .bool()
        && e.call(ACCESS, &args![path, 0u32]).i32() != -1
    {
        return 1;
    }
    if !e
        .call(LOOKS_IN_SEARCH_PATHS, &args![this, flags, extra])
        .bool()
    {
        return 0;
    }
    let count = e.get(this, FileFinder::iSize);
    let mut index = 0u32;
    while index < count {
        let buffer = e.get(this, FileFinder::pBuffer);
        let prefix = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
        if prefix != 0 {
            let length = e.call(STRLEN, &args![prefix]).u32();
            if e.call(STRNICMP, &args![path, prefix, length]).i32() != 0 {
                let found = e.with_stack(PATH_BUFFER_SIZE, |e, candidate| {
                    e.call(STRCPY_S, &args![candidate, PATH_BUFFER_SIZE, prefix]);
                    e.call(STRCAT_S, &args![candidate, PATH_BUFFER_SIZE, path]);
                    if e.call(ACCESS, &args![candidate, 0u32]).i32() != -1 {
                        e.call(STRCPY_S, &args![path, PATH_BUFFER_SIZE, candidate]);
                        true
                    } else {
                        false
                    }
                });
                if found {
                    return 1;
                }
            }
        }
        index += 1;
    }
    0
}

// Translated from 00afe0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afe0d0 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// Unnamed in the engine map (thiscall on the finder, four words): finds
/// `path` with [`fn_00afe220`] and answers its result (0 not found, 1 a
/// file, 2 in an archive). An `extra` of -1 becomes 0xffff. A path with a
/// drive (`?:`) adds 5 to the flags; one starting `.\` adds 4 and loses
/// those two characters. A leading `\` is dropped when the path is copied
/// into a `char[0x104]` local (`strcpy_s`). When `out` is not null it gets
/// the found path (the local, which the lookup may have prefixed), or an
/// empty string when nothing was found.
pub fn fn_00afe0d0(
    e: &mut Engine,
    this: Ptr<FileFinder>,
    path: Ptr,
    out: Ptr,
    flags: u32,
    extra: u32,
) -> u32 {
    let extra = if extra == u32::MAX { 0xffff } else { extra };
    let mut flags = flags;
    let mut path = path.addr();
    if e.mem.u8(path.wrapping_add(1)) == b':' {
        flags |= 5;
    } else if e.mem.u8(path) == b'.' && e.mem.u8(path.wrapping_add(1)) == b'\\' {
        flags |= 4;
        path = path.wrapping_add(2);
    }
    e.with_stack(PATH_BUFFER_SIZE, |e, local| {
        let source = if e.mem.u8(path) == b'\\' {
            path.wrapping_add(1)
        } else {
            path
        };
        e.call(STRCPY_S, &args![local, PATH_BUFFER_SIZE, source]);
        let found = fn_00afe220(e, this, local, flags, extra);
        if !out.is_null() {
            if found == 0 {
                e.mem.set_u8(out.addr(), 0);
            } else {
                e.call(STRCPY_S, &args![out, PATH_BUFFER_SIZE, local]);
            }
        }
        found
    })
}

/// The `FileFinder` singleton pointer (`011f81dc`).
const FILE_FINDER: u32 = 0x011f_81dc;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, thiscall:
/// size) and deallocate (`00aa4060`, thiscall: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `BSFile::BSFile` (Xbox PDB, `00b00260`; thiscall, four words: path,
/// open mode, a word passed through, 0 here) and the `BSFile` size.
const BSFILE_CONSTRUCTOR: u32 = 0x00b0_0260;
const BSFILE_SIZE: u32 = 0x158;
/// The `BSFile` open, vtable +0x20 (`BSFile::Open`, two words).
const OPEN_SLOT: u32 = 0x20;
/// `ArchiveManager::GetFile` (Xbox PDB, `00af5fa0`; cdecl: path, the word
/// passed through, extra).
const ARCHIVE_GET_FILE: u32 = 0x00af_5fa0;
/// The local `00afdf20` gives the lookup for the found path.
const FOUND_PATH_SIZE: u32 = 0x310;

// Translated from 00afdf20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afdf20 - the FS:[0] exception frame (unwind around the BSFile constructors) and the /GS stack cookie check (00ec408c) are not translated
/// `FileFinder::GetFile_ov2` (Xbox PDB; cdecl, four words): answers a file
/// object for `path`, or null; null too while there is no `FileFinder`
/// (`011f81dc`). Open mode 1 constructs a `BSFile` for `path` directly
/// (0x158 bytes from the memory manager; null when that fails) without
/// opening it. Any other mode looks the path up ([`fn_00afe0d0`] with
/// flags 0): in an archive (2) the answer is
/// `ArchiveManager::GetFile(found, passed, extra)`; a loose file (1) gets a
/// `BSFile` for the found path, opened through vtable +0x20 with `(0, 0)`;
/// not found answers null.
///
/// The exe's archive branch also tests the (always null) result first and
/// would only set an unused flag otherwise; that test is kept.
pub fn file_finder_get_file_ov2(
    e: &mut Engine,
    path: Ptr,
    mode: u32,
    passed: u32,
    extra: u32,
) -> u32 {
    let mut result = 0u32;
    let finder = e.global::<u32>(FILE_FINDER);
    if finder == 0 {
        return result;
    }
    if mode == 1 {
        let block = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, BSFILE_SIZE])
            .u32();
        result = if block == 0 {
            0
        } else {
            e.call(BSFILE_CONSTRUCTOR, &args![block, path, 1u32, passed, 0u32])
                .u32()
        };
        return result;
    }
    e.with_stack(FOUND_PATH_SIZE, |e, found_path| {
        let found = fn_00afe0d0(e, Ptr::new(finder), path, found_path, 0, extra);
        if found == 0 {
            return;
        }
        if found == 2 {
            if result == 0 {
                result = e
                    .call(ARCHIVE_GET_FILE, &args![found_path, passed, extra])
                    .u32();
            }
            return;
        }
        let block = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, BSFILE_SIZE])
            .u32();
        result = if block == 0 {
            0
        } else {
            e.call(
                BSFILE_CONSTRUCTOR,
                &args![block, found_path, mode, passed, 0u32],
            )
            .u32()
        };
        e.vcall(result, OPEN_SLOT, &args![0u32, 0u32]);
    });
    result
}

// Translated from 00afdf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FileFinder::GetFile` (Xbox PDB; cdecl, three words):
/// [`file_finder_get_file_ov2`] with `extra` 0xffff.
pub fn file_finder_get_file(e: &mut Engine, path: Ptr, mode: u32, passed: u32) -> u32 {
    file_finder_get_file_ov2(e, path, mode, passed, 0xffff)
}

/// `strstr` (`00ec7750`, cdecl) and the one-character patterns `"*"` and
/// `"?"` in `.rdata`.
const STRSTR: u32 = 0x00ec_7750;
const PATTERN_STAR: u32 = 0x010a_46bc;
const PATTERN_QUESTION: u32 = 0x010a_46c0;
/// `00afe5b0` (cdecl, four words, the same as `BuildFileList`'s): the list
/// build for a name without wildcards (this unit, not translated here).
const BUILD_PLAIN_FILE_LIST: u32 = 0x00af_e5b0;
/// `00b01930` (cdecl: pattern, second word, list): adds the loose files.
const ADD_LOOSE_FILES: u32 = 0x00b0_1930;
/// `ArchiveManager::AddToFileList` (Xbox PDB, `00af6ba0`; cdecl: list,
/// pattern, second word, third word).
const ARCHIVE_ADD_TO_FILE_LIST: u32 = 0x00af_6ba0;
/// `00470470` (thiscall, no stack words): empties the 8-byte list before
/// it is freed.
const LIST_DESTRUCTOR: u32 = 0x0047_0470;
/// The size of the list object (two words, both cleared when made here).
const LIST_SIZE: u32 = 8;

// Translated from 00afe420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FileFinder::BuildFileList` (Xbox PDB; cdecl, four words): null for a
/// null `pattern` or a zero `second`. A pattern without `*` or `?` is
/// handed to `00afe5b0` with the same four words. Otherwise the files go
/// into `list` (an 8-byte list, made with both words cleared when `list` is
/// null; a failed allocation leaves it null): loose files through
/// `00b01930` unless the search paths are on (`011acdf8` set and `011acdf9`
/// clear), then the archives' through `ArchiveManager::AddToFileList`.
/// A list whose two words are both still 0 is destroyed (`00470470`, then
/// freed) and null is answered; otherwise the list.
pub fn file_finder_build_file_list(
    e: &mut Engine,
    pattern: Ptr,
    second: u32,
    third: u32,
    list: u32,
) -> u32 {
    if pattern.is_null() || second == 0 {
        return 0;
    }
    if e.call(STRSTR, &args![pattern, PATTERN_STAR]).u32() == 0
        && e.call(STRSTR, &args![pattern, PATTERN_QUESTION]).u32() == 0
    {
        return e
            .call(BUILD_PLAIN_FILE_LIST, &args![pattern, second, third, list])
            .u32();
    }
    let mut list = list;
    if list == 0 {
        let block = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, LIST_SIZE])
            .u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
            e.mem.set_u32(block + 4, 0);
        }
        list = block;
    }
    if e.global::<u8>(SEARCH_PATHS_ENABLED) == 0 || e.global::<u8>(SEARCH_PATHS_DISABLED) != 0 {
        e.call(ADD_LOOSE_FILES, &args![pattern, second, list]);
    }
    e.call(
        ARCHIVE_ADD_TO_FILE_LIST,
        &args![list, pattern, second, third],
    );
    if e.mem.u32(list.wrapping_add(4)) == 0 && e.mem.u32(list) == 0 {
        if list != 0 {
            e.call(LIST_DESTRUCTOR, &args![list]);
            e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, list]);
        }
        list = 0;
    }
    list
}

/// `00b01af0` (cdecl, one word: the path): whether the loose file exists.
const LOOSE_FILE_EXISTS: u32 = 0x00b0_1af0;
/// `strrchr` (`00ec6e30`) and `memcpy` (`00ec44d0`), cdecl.
const STRRCHR: u32 = 0x00ec_6e30;
const MEMCPY: u32 = 0x00ec_44d0;
/// `005ae3d0` (thiscall on the list, one word: the address of the name): adds a name to
/// the list.
const LIST_ADD_NAME: u32 = 0x005a_e3d0;

// Translated from 00afe1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on the finder, two words): [`fn_00afe220`] with
/// `path`, `flags` and 0xFFFF, whose answer it passes on.
pub fn fn_00afe1f0(e: &mut Engine, this: Ptr<FileFinder>, path: Ptr, flags: u32) -> u32 {
    fn_00afe220(e, this, path, flags, 0xffff)
}

// Translated from 00afe400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): `FileFinder::BuildFileList` with
/// `pattern`, `second`, 0xFFFF and no list, whose answer it passes on.
pub fn fn_00afe400(e: &mut Engine, pattern: Ptr, second: u32) -> u32 {
    file_finder_build_file_list(e, pattern, second, 0xffff, 0)
}

// Translated from 00afe5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words): the file list of a pattern without
/// wildcards. `list` (made with both words cleared when null; a failed allocation leaves it
/// null) gets, when the file exists (as a loose file, `00b01af0`, unless the search paths
/// are on, `011acdf8` set and `011acdf9` clear; else in an archive, `00af6320` with
/// `third`), the name made of `second` up to its last `\` followed by what comes after the
/// pattern's last `\` (a block from the memory manager, `005ae3d0`); nothing when either
/// has no `\`. A list whose two words are both still 0 is destroyed (`00470470`, then
/// freed) and null is answered; otherwise the list.
pub fn fn_00afe5b0(e: &mut Engine, pattern: u32, second: u32, third: u32, list: u32) -> u32 {
    let mut list = list;
    if list == 0 {
        let block = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, LIST_SIZE])
            .u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
            e.mem.set_u32(block + 4, 0);
        }
        list = block;
    }
    let mut found = false;
    if e.global::<u8>(SEARCH_PATHS_ENABLED) == 0 || e.global::<u8>(SEARCH_PATHS_DISABLED) != 0 {
        found = e.call(LOOSE_FILE_EXISTS, &args![pattern]).bool();
    }
    if !found {
        found = e.call(ARCHIVE_CONTAINS, &args![pattern, third]).bool();
    }
    if found {
        let mut length = e.call(STRLEN, &args![second]).u32();
        let second_slash = e.call(STRRCHR, &args![second, 0x5cu32]).u32();
        let pattern_slash = e.call(STRRCHR, &args![pattern, 0x5cu32]).u32();
        if second_slash != 0 && pattern_slash != 0 {
            let tail = e.call(STRLEN, &args![second_slash.wrapping_add(1)]).u32();
            length = length.wrapping_sub(tail);
            let name = e.call(STRLEN, &args![pattern_slash.wrapping_add(1)]).u32();
            let size = length.wrapping_add(name).wrapping_add(1);
            let block = e
                .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
                .u32();
            e.call(MEMCPY, &args![block, second, length]);
            e.mem.set_u8(block.wrapping_add(length), 0);
            e.call(STRCAT_S, &args![block, size, pattern_slash.wrapping_add(1)]);
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), block);
                e.call(LIST_ADD_NAME, &args![list, local]);
            });
        }
    }
    if e.mem.u32(list.wrapping_add(4)) == 0 && e.mem.u32(list) == 0 {
        if list != 0 {
            e.call(LIST_DESTRUCTOR, &args![list]);
            // `MOV EDX,1; AND EDX,1; JZ`: the free always runs.
            e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, list]);
        }
        list = 0;
    }
    list
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_e1f0, fn_00afe1f0(Ptr<FileFinder>, Ptr, u32) -> u32),
        entry!(0x00af_e400, fn_00afe400(Ptr, u32) -> u32),
        entry!(0x00af_e5b0, fn_00afe5b0(u32, u32, u32, u32) -> u32),
        entry!(
            0x00afe220,
            fn_00afe220(Ptr<FileFinder>, Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00af_e0d0,
            fn_00afe0d0(Ptr<FileFinder>, Ptr, Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00af_df20,
            file_finder_get_file_ov2(Ptr, u32, u32, u32) -> u32
        ),
        entry!(0x00af_df00, file_finder_get_file(Ptr, u32, u32) -> u32),
        entry!(
            0x00af_e420,
            file_finder_build_file_list(Ptr, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A finder with the given prefixes, a path buffer holding `path`, and
    /// doubles for every callee: the archive knows `archive_hit`, the loose
    /// files `loose`, the search paths `searched`, and `_access` succeeds for
    /// the strings in `existing`. The log records `(address, first word)`.
    struct Scene {
        e: Engine,
        finder: Ptr<FileFinder>,
        path: Ptr,
        log: Rc<RefCell<Vec<u32>>>,
    }

    fn scene(
        prefixes: &[&[u8]],
        path: &[u8],
        archive_hit: bool,
        loose: bool,
        searched: bool,
        existing: &'static [&'static [u8]],
    ) -> Scene {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.set_global(SEARCH_PATHS_ENABLED, 1u8);
        let finder: Ptr<FileFinder> = e.new_object();
        let table = e.mem.alloc(4 * prefixes.len() as u32 + 4);
        for (i, prefix) in prefixes.iter().enumerate() {
            let text = e.mem.alloc(0x40);
            e.mem.set_cstr(text, prefix);
            e.mem.set_u32(table + 4 * i as u32, text);
        }
        e.set(finder, FileFinder::pBuffer, table);
        e.set(finder, FileFinder::iSize, prefixes.len() as u32);
        let path_buffer = e.mem.alloc(PATH_BUFFER_SIZE);
        e.mem.set_cstr(path_buffer, path);
        let log = Rc::new(RefCell::new(Vec::new()));
        let seen = log.clone();
        e.register_double(ARCHIVE_CONTAINS, move |_, _| {
            seen.borrow_mut().push(ARCHIVE_CONTAINS);
            Ret {
                eax: archive_hit as u32,
                ..Ret::default()
            }
        });
        let seen = log.clone();
        e.register_double(LOOKS_IN_LOOSE_FILES, move |_, a| {
            seen.borrow_mut().push(LOOKS_IN_LOOSE_FILES + a[1]);
            Ret {
                eax: loose as u32,
                ..Ret::default()
            }
        });
        let seen = log.clone();
        e.register_double(LOOKS_IN_SEARCH_PATHS, move |_, a| {
            seen.borrow_mut().push(LOOKS_IN_SEARCH_PATHS + a[1]);
            Ret {
                eax: searched as u32,
                ..Ret::default()
            }
        });
        e.register_double(ACCESS, move |e, a| {
            let text = e.mem.cstr(a[0]);
            let found = existing.iter().any(|name| name[..] == text[..]);
            Ret {
                eax: if found { 0 } else { u32::MAX },
                ..Ret::default()
            }
        });
        e.register(STRLEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
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
        e.register(STRNICMP, |e, a| {
            let first = e.mem.cstr(a[0]);
            let second = e.mem.cstr(a[1]);
            let n = a[2] as usize;
            let lower = |text: &[u8]| {
                text.iter()
                    .take(n)
                    .map(u8::to_ascii_lowercase)
                    .collect::<Vec<_>>()
            };
            Ret {
                eax: (lower(&first) != lower(&second)) as u32,
                ..Ret::default()
            }
        });
        Scene {
            e,
            finder,
            path: Ptr::new(path_buffer),
            log,
        }
    }

    fn find(s: &mut Scene, flags: u32) -> u32 {
        let (finder, path) = (s.finder, s.path);
        s.e.call(0x00af_e220, &args![finder, path, flags, 7u32])
            .u32()
    }

    #[test]
    fn an_archive_hit_returns_two_without_touching_the_disk() {
        let mut s = scene(&[], b"meshes\\a.nif", true, true, true, &[b"meshes\\a.nif"]);
        assert_eq!(find(&mut s, 0), 2);
        assert_eq!(*s.log.borrow(), vec![ARCHIVE_CONTAINS]);
    }

    #[test]
    fn flag_one_skips_the_archive_lookup() {
        let mut s = scene(&[], b"a.nif", true, true, false, &[b"a.nif"]);
        // Bit 0 of the flags skips the archive; the search paths are enabled,
        // so the flags are handed on unchanged.
        assert_eq!(find(&mut s, 1), 1);
        assert_eq!(*s.log.borrow(), vec![LOOKS_IN_LOOSE_FILES + 1]);
    }

    #[test]
    fn the_flags_survive_when_the_search_paths_are_enabled() {
        let mut s = scene(&[], b"a.nif", false, true, false, &[b"a.nif"]);
        s.e.set_global(SEARCH_PATHS_ENABLED, 1u8);
        assert_eq!(find(&mut s, 6), 1);
        assert_eq!(
            *s.log.borrow(),
            vec![ARCHIVE_CONTAINS, LOOKS_IN_LOOSE_FILES + 6]
        );
        s.e.set_global(SEARCH_PATHS_DISABLED, 1u8);
        s.log.borrow_mut().clear();
        assert_eq!(find(&mut s, 6), 1);
        assert_eq!(
            *s.log.borrow(),
            vec![ARCHIVE_CONTAINS, LOOKS_IN_LOOSE_FILES]
        );
    }

    #[test]
    fn a_prefix_is_prepended_and_the_path_replaced() {
        let mut s = scene(
            &[b"", b"data\\"],
            b"a.nif",
            false,
            false,
            true,
            &[b"data\\a.nif"],
        );
        assert_eq!(find(&mut s, 0), 1);
        assert_eq!(s.e.mem.cstr(s.path.addr()), b"data\\a.nif");
    }

    #[test]
    fn a_path_already_starting_with_the_prefix_is_not_prefixed_again() {
        let mut s = scene(
            &[b"data\\"],
            b"data\\a.nif",
            false,
            false,
            true,
            &[b"data\\data\\a.nif"],
        );
        assert_eq!(find(&mut s, 0), 0);
        assert_eq!(s.e.mem.cstr(s.path.addr()), b"data\\a.nif");
    }

    #[test]
    fn nothing_found_returns_zero() {
        let mut s = scene(&[b"x\\"], b"a.nif", false, false, true, &[]);
        assert_eq!(find(&mut s, 0), 0);
        let mut s = scene(&[b"x\\"], b"a.nif", false, false, false, &[]);
        assert_eq!(find(&mut s, 0), 0);
    }
}

#[cfg(test)]
mod b006_tests {
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

    /// Doubles for the string helpers and the lookup's callees: the archive
    /// never knows the path, the loose files are searched and `_access`
    /// succeeds for `existing`.
    fn engine(existing: &'static [&'static [u8]]) -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.map(0x011f_6000, 0x3000);
        e.set_global(SEARCH_PATHS_ENABLED, 1u8);
        e.register(ARCHIVE_CONTAINS, |_, _| 0u32.into_ret());
        e.register(LOOKS_IN_LOOSE_FILES, |_, _| 1u32.into_ret());
        e.register(LOOKS_IN_SEARCH_PATHS, |_, _| 0u32.into_ret());
        e.register_double(ACCESS, move |e, a| {
            let text = e.mem.cstr(a[0]);
            let found = existing.iter().any(|name| name[..] == text[..]);
            (if found { 0 } else { u32::MAX }).into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn finder(e: &mut Engine) -> Ptr<FileFinder> {
        e.new_object()
    }

    fn lookup(e: &mut Engine, path: &[u8], flags: u32, extra: u32) -> (u32, Vec<u8>) {
        let finder = finder(e);
        let text = e.mem.alloc(0x40);
        e.mem.set_cstr(text, path);
        let out = e.mem.alloc(PATH_BUFFER_SIZE);
        e.mem.set_cstr(out, b"junk");
        let found = e
            .call(0x00af_e0d0, &args![finder, text, out, flags, extra])
            .u32();
        (found, e.mem.cstr(out))
    }

    #[test]
    fn lookup_copies_the_found_path_without_a_leading_backslash() {
        let mut e = engine(&[b"meshes\\a.nif"]);
        let (found, out) = lookup(&mut e, b"\\meshes\\a.nif", 0, 3);
        assert_eq!(found, 1);
        assert_eq!(out, b"meshes\\a.nif");
        // The lookup got flags 0 and the extra word unchanged.
        assert_eq!(calls_to(&e, ARCHIVE_CONTAINS)[0][1], 3);
        assert_eq!(&calls_to(&e, LOOKS_IN_LOOSE_FILES)[0][1..], &[0, 3]);
    }

    #[test]
    fn lookup_flags_for_drives_and_dot_backslash() {
        let mut e = engine(&[b"c:\\x", b"y"]);
        let (found, out) = lookup(&mut e, b"c:\\x", 2, u32::MAX);
        assert_eq!((found, out.as_slice()), (1, &b"c:\\x"[..]));
        // A drive path sets bits 0 and 2 (bit 0 skips the archive), and -1
        // becomes 0xffff.
        assert!(calls_to(&e, ARCHIVE_CONTAINS).is_empty());
        assert_eq!(&calls_to(&e, LOOKS_IN_LOOSE_FILES)[0][1..], &[7, 0xffff]);
        let mut e = engine(&[b"y"]);
        let (found, out) = lookup(&mut e, b".\\y", 0, 0);
        assert_eq!((found, out.as_slice()), (1, &b"y"[..]));
        assert_eq!(calls_to(&e, LOOKS_IN_LOOSE_FILES)[0][1], 4);
    }

    #[test]
    fn lookup_empties_the_output_when_nothing_is_found() {
        let mut e = engine(&[]);
        let (found, out) = lookup(&mut e, b"none", 0, 0);
        assert_eq!(found, 0);
        assert_eq!(out, b"");
        // A null output is left alone.
        let finder = finder(&mut e);
        let text = e.mem.alloc(0x10);
        e.mem.set_cstr(text, b"none");
        let found = e
            .call(0x00af_e0d0, &args![finder, text, 0u32, 0u32, 0u32])
            .u32();
        assert_eq!(found, 0);
    }

    const VTABLE: u32 = 0x0300_0000;
    const OPEN: u32 = 0x0070_0000;

    /// `engine` plus a finder in `011f81dc` and doubles for the file
    /// makers: the allocation answers `block` and the constructor its
    /// `this` with the test vtable.
    fn get_file_engine(existing: &'static [&'static [u8]], block: u32) -> Engine {
        let mut e = engine(existing);
        let finder = finder(&mut e);
        e.set_global(FILE_FINDER, finder.addr());
        let mut slots = vec![0u32; OPEN_SLOT as usize / 4 + 1];
        slots[OPEN_SLOT as usize / 4] = OPEN;
        e.put_vtable(VTABLE, &slots);
        e.register(OPEN, |_, _| 1u32.into_ret());
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| block.into_ret());
        e.register(BSFILE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            a[0].into_ret()
        });
        e.register(ARCHIVE_GET_FILE, |_, _| 0x8800u32.into_ret());
        e
    }

    fn path(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(0x40);
        e.mem.set_cstr(at, text);
        at
    }

    #[test]
    fn get_file_without_a_finder_answers_null() {
        let mut e = engine(&[]);
        let name = path(&mut e, b"a");
        assert_eq!(e.call(0x00af_df20, &args![name, 0u32, 0u32, 0u32]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn get_file_mode_one_constructs_without_opening() {
        let block = 0x0090_0000;
        let mut e = get_file_engine(&[], block);
        e.map(block, 0x158);
        let name = path(&mut e, b"save.fos");
        assert_eq!(
            e.call(0x00af_df20, &args![name, 1u32, 5u32, 0u32]).u32(),
            block
        );
        assert_eq!(
            calls_to(&e, BSFILE_CONSTRUCTOR),
            vec![vec![block, name, 1, 5, 0]]
        );
        assert!(calls_to(&e, OPEN).is_empty());
        let mut e = get_file_engine(&[], 0);
        let name = path(&mut e, b"save.fos");
        assert_eq!(e.call(0x00af_df20, &args![name, 1u32, 5u32, 0u32]).u32(), 0);
        assert!(calls_to(&e, BSFILE_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn get_file_opens_a_loose_file() {
        let block = 0x0090_0000;
        let mut e = get_file_engine(&[b"a.txt"], block);
        e.map(block, 0x158);
        let name = path(&mut e, b"a.txt");
        assert_eq!(e.call(0x00af_df00, &args![name, 0u32, 5u32]).u32(), block);
        let made = calls_to(&e, BSFILE_CONSTRUCTOR);
        assert_eq!(made.len(), 1);
        assert_eq!(e.mem.cstr(made[0][1]), b"a.txt");
        assert_eq!(&made[0][2..], &[0, 5, 0]);
        assert_eq!(calls_to(&e, OPEN), vec![vec![block, 0, 0]]);
        // `GetFile` passes 0xffff as the extra word.
        assert_eq!(calls_to(&e, LOOKS_IN_LOOSE_FILES)[0][2], 0xffff);
    }

    #[test]
    fn get_file_takes_an_archived_file_from_the_archive_manager() {
        let mut e = get_file_engine(&[], 0);
        e.register(ARCHIVE_CONTAINS, |_, _| 1u32.into_ret());
        let name = path(&mut e, b"a.nif");
        assert_eq!(
            e.call(0x00af_df20, &args![name, 0u32, 5u32, 9u32]).u32(),
            0x8800
        );
        let asked = calls_to(&e, ARCHIVE_GET_FILE);
        assert_eq!(asked.len(), 1);
        assert_eq!(e.mem.cstr(asked[0][0]), b"a.nif");
        assert_eq!(&asked[0][1..], &[5, 9]);
        assert!(calls_to(&e, BSFILE_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn get_file_not_found_answers_null() {
        let mut e = get_file_engine(&[], 0);
        let name = path(&mut e, b"none");
        assert_eq!(e.call(0x00af_df20, &args![name, 0u32, 5u32, 9u32]).u32(), 0);
        assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
    }

    /// Doubles for `BuildFileList`'s callees; `strstr` finds the pattern's
    /// characters, the list makers record their calls, and
    /// `AddToFileList` sets the list's first word to `found`.
    fn list_engine(found: u32, block: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.register(STRSTR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let wanted = e.mem.u8(a[1]);
            match text.iter().position(|&c| c == wanted) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(BUILD_PLAIN_FILE_LIST, |_, _| 0x7100u32.into_ret());
        e.register(ADD_LOOSE_FILES, |_, _| Ret::default());
        e.register_double(ARCHIVE_ADD_TO_FILE_LIST, move |e, a| {
            e.mem.set_u32(a[0], found);
            Ret::default()
        });
        e.register(LIST_DESTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn pattern(e: &mut Engine, text: &[u8]) -> u32 {
        // `strstr` patterns live in `.rdata`; the test writes them.
        e.map(0x010a_4000, 0x1000);
        e.mem.set_cstr(PATTERN_STAR, b"*");
        e.mem.set_cstr(PATTERN_QUESTION, b"?");
        let at = e.mem.alloc(0x40);
        e.mem.set_cstr(at, text);
        at
    }

    #[test]
    fn build_file_list_rejects_null_and_hands_plain_names_on() {
        let mut e = list_engine(0, 0);
        assert_eq!(e.call(0x00af_e420, &args![0u32, 1u32, 2u32, 0u32]).u32(), 0);
        let name = pattern(&mut e, b"a.txt");
        assert_eq!(e.call(0x00af_e420, &args![name, 0u32, 2u32, 0u32]).u32(), 0);
        assert_eq!(
            e.call(0x00af_e420, &args![name, 1u32, 2u32, 3u32]).u32(),
            0x7100
        );
        assert_eq!(
            calls_to(&e, BUILD_PLAIN_FILE_LIST),
            vec![vec![name, 1, 2, 3]]
        );
    }

    #[test]
    fn build_file_list_fills_a_new_list() {
        let list = 0x0090_0000;
        let mut e = list_engine(0x55, list);
        e.map(list, 8);
        e.mem.set_u32(list, 0xdead);
        let name = pattern(&mut e, b"*.txt");
        assert_eq!(
            e.call(0x00af_e420, &args![name, 1u32, 2u32, 0u32]).u32(),
            list
        );
        // The search paths are off, so the loose files come first.
        assert_eq!(calls_to(&e, ADD_LOOSE_FILES), vec![vec![name, 1, list]]);
        assert_eq!(
            calls_to(&e, ARCHIVE_ADD_TO_FILE_LIST),
            vec![vec![list, name, 1, 2]]
        );
        assert_eq!(e.mem.u32(list + 4), 0);
        assert!(calls_to(&e, LIST_DESTRUCTOR).is_empty());
    }

    #[test]
    fn build_file_list_frees_an_empty_list_and_skips_loose_files_with_search_paths() {
        let mut e = list_engine(0, 0);
        e.set_global(SEARCH_PATHS_ENABLED, 1u8);
        let list = e.mem.alloc(8);
        let name = pattern(&mut e, b"a?.txt");
        assert_eq!(e.call(0x00af_e420, &args![name, 1u32, 2u32, list]).u32(), 0);
        assert!(calls_to(&e, ADD_LOOSE_FILES).is_empty());
        assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert_eq!(calls_to(&e, LIST_DESTRUCTOR), vec![vec![list]]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, list]]
        );
    }
}

#[cfg(test)]
mod n083_tests {
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
    fn lookup_wrapper_passes_0xffff_and_the_answer() {
        let mut e = Engine::new();
        e.register(ARCHIVE_CONTAINS, |_, _| 1u32.into_ret());
        let finder: Ptr<FileFinder> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_e1f0, &args![finder, 0x5000u32, 0u32]).u32(),
            2
        );
        assert_eq!(calls_to(&e, ARCHIVE_CONTAINS), vec![vec![0x5000, 0xffff]]);
    }

    #[test]
    fn file_list_wrapper_passes_0xffff_and_no_list() {
        let mut e = Engine::new();
        e.register(STRSTR, |_, _| Ret::default());
        e.register(BUILD_PLAIN_FILE_LIST, |_, _| 0x77u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_e400, &args![0x5000u32, 0x6000u32]).u32(),
            0x77
        );
        assert_eq!(
            calls_to(&e, BUILD_PLAIN_FILE_LIST),
            vec![vec![0x5000, 0x6000, 0xffff, 0]]
        );
        assert_eq!(e.call(0x00af_e400, &args![0u32, 0x6000u32]).u32(), 0);
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let at = text.iter().rposition(|&c| u32::from(c) == a[1]);
            at.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            e.mem.alloc(a[1].max(1)).into_ret()
        });
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(LIST_DESTRUCTOR, |_, _| Ret::default());
        e.register(LIST_ADD_NAME, |e, a| {
            let name = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], name);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn text(e: &mut Engine, value: &[u8]) -> u32 {
        let at = e.mem.alloc(0x40);
        e.mem.set_cstr(at, value);
        at
    }

    #[test]
    fn plain_file_list_names_the_found_file() {
        let mut e = engine();
        e.register(LOOSE_FILE_EXISTS, |_, _| 1u32.into_ret());
        e.register(ARCHIVE_CONTAINS, |_, _| Ret::default());
        let pattern = text(&mut e, b"meshes\\a\\b.nif");
        let second = text(&mut e, b"data\\meshes\\x.nif");
        let list = e
            .call(0x00af_e5b0, &args![pattern, second, 3u32, 0u32])
            .u32();
        assert_ne!(list, 0);
        assert_eq!(e.mem.cstr(e.mem.u32(list)), b"data\\meshes\\b.nif");
        assert!(calls_to(&e, ARCHIVE_CONTAINS).is_empty());
        // With the search paths on, only the archives are asked; given a list.
        e.mem.set_u8(SEARCH_PATHS_ENABLED, 1);
        e.register(ARCHIVE_CONTAINS, |_, _| 1u32.into_ret());
        let mine = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_e5b0, &args![pattern, second, 3u32, mine])
                .u32(),
            mine
        );
        assert!(calls_to(&e, LOOSE_FILE_EXISTS).is_empty());
        assert_eq!(calls_to(&e, ARCHIVE_CONTAINS), vec![vec![pattern, 3]]);
    }

    #[test]
    fn plain_file_list_without_a_find_is_destroyed() {
        let mut e = engine();
        e.register(LOOSE_FILE_EXISTS, |_, _| Ret::default());
        e.register(ARCHIVE_CONTAINS, |_, _| Ret::default());
        let pattern = text(&mut e, b"meshes\\b.nif");
        let second = text(&mut e, b"x.nif");
        assert_eq!(
            e.call(0x00af_e5b0, &args![pattern, second, 3u32, 0u32])
                .u32(),
            0
        );
        let made = calls_to(&e, MEMORY_MANAGER_ALLOCATE);
        assert_eq!(made, vec![vec![MEMORY_MANAGER, 8]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCTOR).len(), 1);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
        // Found, but `second` has no `\`: nothing added, destroyed too.
        e.register(ARCHIVE_CONTAINS, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_e5b0, &args![pattern, second, 3u32, 0u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, LIST_ADD_NAME).is_empty());
        assert_eq!(calls_to(&e, LIST_DESTRUCTOR).len(), 1);
    }
}
