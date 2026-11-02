//! `Havok SDK/run_00da25a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Windows imports, by import slot.
const GET_FILE_ATTRIBUTES: u32 = 0x00fd_f160;
const FIND_FIRST_FILE: u32 = 0x00fd_f070;
const FIND_NEXT_FILE: u32 = 0x00fd_f068;
const FIND_CLOSE: u32 = 0x00fd_f064;
const LOAD_LIBRARY: u32 = 0x00fd_f0b0;
const GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
const FREE_LIBRARY: u32 = 0x00fd_f0a8;
const OUTPUT_DEBUG_STRING: u32 = 0x00fd_f16c;
const GET_CURRENT_PROCESS: u32 = 0x00fd_f110;
/// The thread's memory router (`00c85750`); its `+0xc` is the temporary allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkStringBuf` functions: the empty constructor (`00cbbd80`, thiscall), the constructor from a
/// C string (`00d61a50`, thiscall `(text)`), the replace-character function (`00d61840`, thiscall
/// `(from, to, flag)`), the ends-with test (`00d61700`, thiscall `(text)`) and the append
/// (`00d61df0`, thiscall `(text)`).
const STRING_BUFFER_NEW: u32 = 0x00cb_bd80;
const STRING_BUFFER_FROM_TEXT: u32 = 0x00d6_1a50;
const STRING_BUFFER_REPLACE: u32 = 0x00d6_1840;
const STRING_BUFFER_ENDS_WITH: u32 = 0x00d6_1700;
const STRING_BUFFER_APPEND: u32 = 0x00d6_1df0;
/// `hkString::strCmp` (`00cb0fa0`, cdecl) and the string copy `00cb1160` (cdecl `(text,
/// allocator)`, answers the copy).
const STRING_COMPARE: u32 = 0x00cb_0fa0;
const STRING_COPY: u32 = 0x00cb_1160;
/// The grow-by-one `00c90940` (cdecl `(allocator, array, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// The function pointer cell at `011b3dc0` that turns a path into the platform's (cdecl `(path,
/// string buffer)`).
const NATIVE_PATH_POINTER: u32 = 0x011b_3dc0;
/// The text constants `"/"`, `"*"`, `"."` and `".."`.
const TEXT_SLASH: u32 = 0x0106_3cbc;
const TEXT_STAR: u32 = 0x0101_ffbc;
const TEXT_DOT: u32 = 0x0104_4c28;
const TEXT_DOT_DOT: u32 = 0x010a_2560;
/// The `FILE_ATTRIBUTE_DIRECTORY` bit.
const DIRECTORY_ATTRIBUTE: u32 = 0x10;
/// The `FILETIME` of the Unix epoch (1970-01-01) in 100 ns ticks since 1601.
const UNIX_EPOCH_FILETIME: u64 = 0x019d_b1de_d53e_8000;

/// Offsets in the exe's 0x274-byte frame of `00da25a0`: the two `hkStringBuf`s (0x8c bytes each)
/// and the `WIN32_FIND_DATAA` (0x140 bytes). Its `cFileName` is at `+0x2c`.
const FRAME_NATIVE_PATH: u32 = 0x1c;
const FRAME_PATH: u32 = 0xa8;
const FRAME_FIND_DATA: u32 = 0x134;

/// Releases the storage of an `hkStringBuf`-style array `buffer` (data, size, capacity word): its
/// size is set to 0 and an owned block (the capacity word's top bit clear) is freed through the
/// temporary allocator's slot `+0x10` `(data, bytes)`.
fn release_string_buffer(e: &mut Engine, buffer: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let capacity = e.mem.u32(buffer + 8);
    e.mem.set_u32(buffer + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(buffer);
        e.vcall(allocator, 0x10, &args![data, capacity & 0x3fff_ffff]);
    }
}

/// Appends an entry (name copy, time, kind) to the array of 12-byte entries `entries`
/// (data, size, capacity word, allocator at `+0xc`), growing it first when it is full.
fn add_directory_entry(e: &mut Engine, entries: u32, name: u32, time: u32, kind: u32) {
    let size = e.mem.u32(entries + 4);
    if size == e.mem.u32(entries + 8) & 0x3fff_ffff {
        let allocator = e.mem.u32(entries + 0xc);
        e.call(ARRAY_GROW, &args![allocator, entries, 0xcu32]);
    }
    let size = e.mem.u32(entries + 4);
    let data = e.mem.u32(entries);
    let slot = data.wrapping_add(size.wrapping_mul(12));
    e.mem.set_u32(entries + 4, size.wrapping_add(1));
    let allocator = e.mem.u32(entries + 0xc);
    let copy = e.call(STRING_COPY, &args![name, allocator]).u32();
    e.mem.set_u32(slot + 8, kind);
    e.mem.set_u32(slot + 4, time);
    e.mem.set_u32(slot, copy);
}

// Translated from 00da25a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lists a directory: cdecl `(path, entries)`, answers 0 when the directory was read and 1 when
/// `path` is not a directory. The path goes through a string buffer: when it is longer than the
/// terminator, its `\` become `/` and a `/` is appended unless it ends with one, then the
/// function pointer at `011b3dc0` turns it into the platform's form in a second buffer. That
/// one must name a directory (`GetFileAttributesA`; any answer but `-1` with the directory bit
/// counts), gets a `*` appended unless it is empty, and `FindFirstFileA` / `FindNextFileA`
/// walk it, skipping `.` and `..`. Every other item adds one 12-byte entry to the array
/// `entries` (data, size, capacity word, allocator at `+0xc`): the name copied with `00cb1160`,
/// and for a directory 0 at `+4` and 2 at `+8`, for a file the last write time in seconds since
/// 1970 (the low word of the signed 64-bit division by 10,000,000 that the exe does with
/// `__alldiv`) and 1. The find handle is closed and both buffers are released. The exe's name
/// for it is unknown.
pub fn fn_00da25a0(e: &mut Engine, path: u32, entries: u32) -> u32 {
    e.with_stack(0x274, |e, frame| {
        let base = frame.addr();
        let native = base + FRAME_NATIVE_PATH;
        let source = base + FRAME_PATH;
        let find = base + FRAME_FIND_DATA;
        e.call(STRING_BUFFER_NEW, &args![native]);
        e.call(STRING_BUFFER_FROM_TEXT, &args![source, path]);
        if (e.mem.u32(source + 4).wrapping_sub(1) as i32) > 0 {
            e.call(
                STRING_BUFFER_REPLACE,
                &args![source, 0x5cu32, 0x2fu32, 1u32],
            );
            if e.call(STRING_BUFFER_ENDS_WITH, &args![source, TEXT_SLASH])
                .u32()
                == 0
            {
                e.call(STRING_BUFFER_APPEND, &args![source, TEXT_SLASH]);
            }
            let text = e.mem.u32(source);
            let convert = e.mem.u32(NATIVE_PATH_POINTER);
            e.call(convert, &args![text, native]);
        }
        release_string_buffer(e, source);
        let text = e.mem.u32(native);
        let attributes = e.call(GET_FILE_ATTRIBUTES, &args![text]).u32();
        if attributes == 0xffff_ffff || (attributes >> 4) & 1 == 0 {
            release_string_buffer(e, native);
            return 1;
        }
        if e.mem.u32(native + 4).wrapping_sub(1) != 0 {
            e.call(STRING_BUFFER_APPEND, &args![native, TEXT_STAR]);
        }
        let text = e.mem.u32(native);
        let handle = e.call(FIND_FIRST_FILE, &args![text, find]).u32();
        if handle != 0xffff_ffff {
            loop {
                let name = find + 0x2c;
                let is_dot = e.call(STRING_COMPARE, &args![name, TEXT_DOT]).u32() == 0;
                if !is_dot && e.call(STRING_COMPARE, &args![name, TEXT_DOT_DOT]).u32() != 0 {
                    if e.mem.u32(find) & DIRECTORY_ATTRIBUTE == 0 {
                        let low = e.mem.u32(find + 0x14);
                        let high = e.mem.u32(find + 0x18);
                        let ticks = ((u64::from(high) << 32) | u64::from(low))
                            .wrapping_sub(UNIX_EPOCH_FILETIME)
                            as i64;
                        let seconds = (ticks / 10_000_000) as u32;
                        add_directory_entry(e, entries, name, seconds, 1);
                    } else {
                        add_directory_entry(e, entries, name, 0, 2);
                    }
                }
                if e.call(FIND_NEXT_FILE, &args![handle, find]).u32() == 0 {
                    break;
                }
            }
        }
        e.call(FIND_CLOSE, &args![handle]);
        release_string_buffer(e, native);
        0
    })
}

/// `GetProcAddress` of `name` in the library whose handle is the word at `state` (read again for
/// every symbol, as the exe does).
fn load_symbol(e: &mut Engine, state: u32, name: u32) -> u32 {
    let module = e.mem.u32(state);
    e.call(GET_PROC_ADDRESS, &args![module, name]).u32()
}

// Translated from 00da2840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the debug help library for the stack tracer: thiscall on the library state (12 words:
/// module at `+0`, a loaded flag at `+4`, the entry points from `+8`). The library named by the
/// pointer at `011b76d0` is loaded; when that fails a message goes to the debugger and, if the
/// module is still 0, nothing else happens. Otherwise the flag becomes 1 and the entry points
/// are looked up in the order `SymInitialize` (`+8`), `SymRefreshModuleList` (`+0xc`; a missing
/// one only draws a message: old library), `SymGetOptions` (`+0x10`), `SymSetOptions` (`+0x14`),
/// `SymGetSymFromAddr` (`+0x18`), `StackWalk64` (`+0x1c`), `SymFunctionTableAccess64` (`+0x20`),
/// `SymGetModuleBase64` (`+0x24`), `SymGetLineFromAddr64` (`+0x28`) and `SymCleanup` (`+0x2c`).
/// Then the options are read, set to `options | 0x80000010` and `SymInitialize(process, 0, 1)`
/// is called. The exe's name for it is unknown (the state of `hkStackTracer`).
pub fn fn_00da2840(e: &mut Engine, state: u32) {
    let library_name = e.mem.u32(0x011b_76d0);
    let module = e.call(LOAD_LIBRARY, &args![library_name]).u32();
    e.mem.set_u32(state, module);
    if module == 0 {
        e.call(OUTPUT_DEBUG_STRING, &args![0x010d_4da0u32]);
        if e.mem.u32(state) == 0 {
            return;
        }
    }
    e.mem.set_u32(state + 4, 1);
    let symbol = load_symbol(e, state, 0x010d_4d90);
    e.mem.set_u32(state + 8, symbol);
    let symbol = load_symbol(e, state, 0x010d_4d78);
    e.mem.set_u32(state + 0xc, symbol);
    if symbol == 0 {
        e.call(OUTPUT_DEBUG_STRING, &args![0x010d_4cf8u32]);
    }
    let names = [
        (0x10u32, 0x010d_4ce4u32),
        (0x14, 0x010d_4cd4),
        (0x18, 0x010d_4cc0),
        (0x1c, 0x010d_4cb4),
        (0x20, 0x010d_4c98),
        (0x24, 0x010d_4c84),
        (0x28, 0x010d_4c6c),
        (0x2c, 0x010d_4c60),
    ];
    for (slot, name) in names {
        let symbol = load_symbol(e, state, name);
        e.mem.set_u32(state + slot, symbol);
    }
    let get_options = e.mem.u32(state + 0x10);
    let options = e.call(get_options, &args![]).u32();
    let set_options = e.mem.u32(state + 0x14);
    e.call(set_options, &args![options | 0x8000_0010]);
    let process = e.call(GET_CURRENT_PROCESS, &args![]).u32();
    let initialize = e.mem.u32(state + 8);
    e.call(initialize, &args![process, 0u32, 1u32]);
}

// Translated from 00da2930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStackTracer::~hkStackTracer` (RTTI `hkStackTracer`, vtable `010d4de4`), thiscall: puts its
/// own vtable in, and when the library state `[this + 8]` has a module (see `00da2840`) calls
/// `SymCleanup(process)` (`+0x2c`), clears the entry points (`+8` to `+0x2c`), frees the library
/// and clears the module; then it restores the base class's vtable (`0102e388`).
pub fn fn_00da2930(e: &mut Engine, this: u32) {
    let state = e.mem.u32(this + 8);
    e.mem.set_u32(this, 0x010d_4de4);
    if e.mem.u32(state) != 0 {
        let process = e.call(GET_CURRENT_PROCESS, &args![]).u32();
        let cleanup = e.mem.u32(state + 0x2c);
        e.call(cleanup, &args![process]);
        let module = e.mem.u32(state);
        for slot in 2..=11u32 {
            e.mem.set_u32(state + 4 * slot, 0);
        }
        e.call(FREE_LIBRARY, &args![module]);
        e.mem.set_u32(state, 0);
    }
    e.mem.set_u32(this, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_25a0, fn_00da25a0(u32, u32) -> u32),
        entry!(0x00da_2840, fn_00da2840(u32)),
        entry!(0x00da_2930, fn_00da2930(u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATOR_FREE: u32 = 0x0900_0010;

    fn put_text(e: &mut Engine, address: u32, text: &str) {
        for (i, byte) in text.bytes().enumerate() {
            e.mem.set_u8(address + i as u32, byte);
        }
        e.mem.set_u8(address + text.len() as u32, 0);
    }

    fn read_text(e: &Engine, address: u32) -> String {
        let mut text = String::new();
        let mut at = address;
        while e.mem.u8(at) != 0 {
            text.push(e.mem.u8(at) as char);
            at += 1;
        }
        text
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        for (address, text) in [
            (TEXT_SLASH, "/"),
            (TEXT_STAR, "*"),
            (TEXT_DOT, "."),
            (TEXT_DOT_DOT, ".."),
        ] {
            put_text(&mut e, address, text);
        }
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// One item the fake directory walk delivers: name, attributes and write time.
    type Item = (&'static str, u32, u64);

    fn deliver(e: &mut Engine, find: u32, item: &Item) {
        e.mem.set_u32(find, item.1);
        e.mem.set_u32(find + 0x14, item.2 as u32);
        e.mem.set_u32(find + 0x18, (item.2 >> 32) as u32);
        put_text(e, find + 0x2c, item.0);
    }

    /// Installs the doubles of everything `00da25a0` calls. The path buffers keep their text in
    /// blocks (`0x40` bytes); the native path function copies the path and the append double
    /// adds to a buffer's text. `attributes` is what `GetFileAttributesA` answers and `items`
    /// what the directory walk delivers (the first one from `FindFirstFileA`).
    fn install(e: &mut Engine, attributes: u32, items: &[Item]) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, ALLOCATOR_FREE);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, allocator);
        answer(e, MEMORY_ROUTER, router);
        answer(e, ALLOCATOR_FREE, 0);
        e.register(STRING_BUFFER_NEW, |e, a| {
            let text = e.mem.alloc(0x40);
            e.mem.set_u32(a[0], text);
            e.mem.set_u32(a[0] + 4, 1);
            e.mem.set_u32(a[0] + 8, 0x40);
            Ret::default()
        });
        e.register(STRING_BUFFER_FROM_TEXT, |e, a| {
            let text = e.mem.alloc(0x40);
            let copy = read_text(e, a[1]);
            put_text(e, text, &copy);
            e.mem.set_u32(a[0], text);
            e.mem.set_u32(a[0] + 4, copy.len() as u32 + 1);
            e.mem.set_u32(a[0] + 8, 0x8000_0040);
            Ret::default()
        });
        e.register(STRING_BUFFER_REPLACE, |e, a| {
            let mut at = e.mem.u32(a[0]);
            while e.mem.u8(at) != 0 {
                if u32::from(e.mem.u8(at)) == a[1] {
                    e.mem.set_u8(at, a[2] as u8);
                }
                at += 1;
            }
            Ret::default()
        });
        e.register(STRING_BUFFER_ENDS_WITH, |e, a| {
            let text = read_text(e, e.mem.u32(a[0]));
            let tail = read_text(e, a[1]);
            u32::from(text.ends_with(&tail)).into_ret()
        });
        e.register(STRING_BUFFER_APPEND, |e, a| {
            let text = e.mem.u32(a[0]);
            let mut joined = read_text(e, text);
            joined.push_str(&read_text(e, a[1]));
            put_text(e, text, &joined);
            e.mem.set_u32(a[0] + 4, joined.len() as u32 + 1);
            Ret::default()
        });
        e.mem.set_u32(NATIVE_PATH_POINTER, 0x0900_0300);
        e.register(0x0900_0300, |e, a| {
            let text = e.mem.alloc(0x40);
            put_text(e, text, &format!("C:\\{}", read_text(e, a[0])));
            e.mem.set_u32(a[1], text);
            e.mem.set_u32(a[1] + 4, read_text(e, text).len() as u32 + 1);
            e.mem.set_u32(a[1] + 8, 0x40);
            Ret::default()
        });
        answer(e, GET_FILE_ATTRIBUTES, attributes);
        e.register(STRING_COMPARE, |e, a| {
            let left = read_text(e, a[0]);
            let right = read_text(e, a[1]);
            (left.cmp(&right) as i32 as u32).into_ret()
        });
        e.register(STRING_COPY, |e, a| {
            let copy = e.mem.alloc(0x40);
            let text = read_text(e, a[0]);
            put_text(e, copy, &text);
            copy.into_ret()
        });
        e.register(ARRAY_GROW, |e, a| {
            let array = a[1];
            let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) * 2 + 2;
            let block = e.mem.alloc(capacity * 12);
            for byte in 0..e.mem.u32(array + 4) * 12 {
                let value = e.mem.u8(e.mem.u32(array) + byte);
                e.mem.set_u8(block + byte, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        let pending = Rc::new(RefCell::new(items.to_vec()));
        let first = pending.clone();
        e.register_double(FIND_FIRST_FILE, move |e, a| {
            if first.borrow().is_empty() {
                return Ret {
                    eax: 0xffff_ffff,
                    ..Ret::default()
                };
            }
            let item = first.borrow_mut().remove(0);
            deliver(e, a[1], &item);
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        e.register_double(FIND_NEXT_FILE, move |e, a| {
            if pending.borrow().is_empty() {
                return Ret::default();
            }
            let item = pending.borrow_mut().remove(0);
            deliver(e, a[1], &item);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        answer(e, FIND_CLOSE, 1);
    }

    /// An empty entry array (no storage, capacity 0, allocator word `0xa110c`).
    fn entry_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 0xc, 0xa_110c);
        array
    }

    fn entry(e: &Engine, array: u32, index: u32) -> (String, u32, u32) {
        let slot = e.mem.u32(array) + 12 * index;
        (
            read_text(e, e.mem.u32(slot)),
            e.mem.u32(slot + 4),
            e.mem.u32(slot + 8),
        )
    }

    #[test]
    fn a_directory_is_listed_with_files_timed_in_seconds_and_dots_skipped() {
        let mut e = engine();
        let items: [Item; 5] = [
            (".", 0x10, 0),
            ("..", 0x10, 0),
            ("a.txt", 0x20, UNIX_EPOCH_FILETIME + 5 * 10_000_000 + 9),
            ("sub", 0x10, 0x1234),
            // Before 1970: the quotient is negative and only its low word is stored.
            ("old", 0x20, UNIX_EPOCH_FILETIME - 3 * 10_000_000),
        ];
        install(&mut e, 0x10, &items);
        let path = e.mem.alloc(0x40);
        put_text(&mut e, path, "data\\meshes");
        let entries = entry_array(&mut e);
        let result = e.call(0x00da_25a0, &args![path, entries]).u32();
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(entries + 4), 3);
        assert_eq!(entry(&e, entries, 0), ("a.txt".to_string(), 5, 1));
        assert_eq!(entry(&e, entries, 1), ("sub".to_string(), 0, 2));
        assert_eq!(entry(&e, entries, 2), ("old".to_string(), 0xffff_fffd, 1));
        // The array grew from nothing for the first entry and when it was full for the third.
        let grows = calls(&e, ARRAY_GROW);
        assert_eq!(
            grows,
            vec![vec![0xa_110c, entries, 12], vec![0xa_110c, entries, 12]]
        );
        // The path became `data/meshes/` in the native form; a slash and a star were appended.
        let native = calls(&e, 0x0900_0300);
        assert_eq!(read_text(&e, native[0][0]), "data/meshes/");
        let appends = calls(&e, STRING_BUFFER_APPEND);
        assert_eq!(appends.len(), 2);
        assert_eq!(appends[0][1], TEXT_SLASH);
        assert_eq!(appends[1][1], TEXT_STAR);
    }

    #[test]
    fn the_path_buffers_are_released_and_the_find_handle_closed() {
        let mut e = engine();
        let items: [Item; 1] = [("x", 0x20, UNIX_EPOCH_FILETIME)];
        install(&mut e, 0x10, &items);
        let path = e.mem.alloc(0x40);
        put_text(&mut e, path, "dir");
        let entries = entry_array(&mut e);
        e.call(0x00da_25a0, &args![path, entries]);
        assert_eq!(calls(&e, FIND_CLOSE), vec![vec![0x77]]);
        // The first buffer is not owned (no free); the native one has 0x40 bytes of storage.
        let frees = calls(&e, ALLOCATOR_FREE);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][2], 0x40);
    }

    #[test]
    fn a_path_that_is_not_a_directory_answers_one_and_frees_the_native_buffer() {
        for attributes in [0xffff_ffffu32, 0x20, 0] {
            let mut e = engine();
            install(&mut e, attributes, &[("x", 0x20, 0)]);
            let path = e.mem.alloc(0x40);
            put_text(&mut e, path, "somewhere");
            let entries = entry_array(&mut e);
            let result = e.call(0x00da_25a0, &args![path, entries]).u32();
            assert_eq!(result, 1);
            assert!(calls(&e, FIND_FIRST_FILE).is_empty());
            assert!(calls(&e, FIND_CLOSE).is_empty());
            assert_eq!(e.mem.u32(entries + 4), 0);
            let frees = calls(&e, ALLOCATOR_FREE);
            assert_eq!(frees.len(), 1);
        }
    }

    #[test]
    fn an_empty_path_skips_the_conversion_and_the_star() {
        let mut e = engine();
        install(&mut e, 0x10, &[]);
        let path = e.mem.alloc(0x40);
        put_text(&mut e, path, "");
        let entries = entry_array(&mut e);
        let result = e.call(0x00da_25a0, &args![path, entries]).u32();
        // Nothing found: `FindFirstFileA` fails, the handle `-1` is still closed.
        assert_eq!(result, 0);
        assert!(calls(&e, STRING_BUFFER_REPLACE).is_empty());
        assert!(calls(&e, 0x0900_0300).is_empty());
        assert!(calls(&e, STRING_BUFFER_APPEND).is_empty());
        assert_eq!(calls(&e, FIND_CLOSE), vec![vec![0xffff_ffff]]);
    }

    #[test]
    fn a_path_ending_in_a_slash_gets_no_second_one() {
        let mut e = engine();
        install(&mut e, 0x10, &[]);
        let path = e.mem.alloc(0x40);
        put_text(&mut e, path, "a\\b\\");
        let entries = entry_array(&mut e);
        e.call(0x00da_25a0, &args![path, entries]);
        let native = calls(&e, 0x0900_0300);
        assert_eq!(read_text(&e, native[0][0]), "a/b/");
        // Only the star was appended.
        let appends = calls(&e, STRING_BUFFER_APPEND);
        assert_eq!(appends.len(), 1);
        assert_eq!(appends[0][1], TEXT_STAR);
    }

    /// Fakes the debug help imports: the library loads as `module`, symbols in `missing` are not
    /// found, the others are found at `0x0a000000` plus the low half of the name address.
    fn debug_help(e: &mut Engine, module: u32, missing: &'static [u32]) {
        e.mem.set_u32(0x011b_76d0, 0x010d_4c54);
        answer(e, LOAD_LIBRARY, module);
        e.register_double(GET_PROC_ADDRESS, move |_, a| {
            if missing.contains(&a[1]) {
                0u32.into_ret()
            } else {
                (0x0a00_0000 + (a[1] & 0xffff)).into_ret()
            }
        });
        answer(e, OUTPUT_DEBUG_STRING, 0);
        answer(e, GET_CURRENT_PROCESS, 0x4242);
        answer(e, 0x0a00_0000 + (0x010d_4ce4 & 0xffff), 0x5);
        answer(e, 0x0a00_0000 + (0x010d_4cd4 & 0xffff), 0);
        answer(e, 0x0a00_0000 + (0x010d_4d90 & 0xffff), 1);
    }

    #[test]
    fn the_library_is_loaded_and_its_entry_points_stored_in_order() {
        let mut e = engine();
        debug_help(&mut e, 0x6000, &[]);
        let state = e.mem.alloc(0x30);
        e.call(0x00da_2840, &args![state]);
        assert_eq!(calls(&e, LOAD_LIBRARY), vec![vec![0x010d_4c54]]);
        assert_eq!(e.mem.u32(state), 0x6000);
        assert_eq!(e.mem.u32(state + 4), 1);
        let names = [
            0x010d_4d90u32,
            0x010d_4d78,
            0x010d_4ce4,
            0x010d_4cd4,
            0x010d_4cc0,
            0x010d_4cb4,
            0x010d_4c98,
            0x010d_4c84,
            0x010d_4c6c,
            0x010d_4c60,
        ];
        let asked: Vec<u32> = calls(&e, GET_PROC_ADDRESS).iter().map(|a| a[1]).collect();
        assert_eq!(asked, names);
        for (i, name) in names.iter().enumerate() {
            assert_eq!(
                e.mem.u32(state + 8 + 4 * i as u32),
                0x0a00_0000 + (name & 0xffff)
            );
        }
        assert!(calls(&e, OUTPUT_DEBUG_STRING).is_empty());
        // The options (5) are or-ed with 0x80000010, then the library is initialised for the
        // current process: `SymInitialize(process, 0, 1)`.
        let set_options = 0x0a00_0000 + (0x010d_4cd4 & 0xffff);
        assert_eq!(calls(&e, set_options), vec![vec![0x8000_0015]]);
        let initialize = 0x0a00_0000 + (0x010d_4d90 & 0xffff);
        assert_eq!(calls(&e, initialize), vec![vec![0x4242, 0, 1]]);
    }

    #[test]
    fn a_missing_refresh_function_only_draws_a_message() {
        let mut e = engine();
        debug_help(&mut e, 0x6000, &[0x010d_4d78]);
        let state = e.mem.alloc(0x30);
        e.call(0x00da_2840, &args![state]);
        assert_eq!(calls(&e, OUTPUT_DEBUG_STRING), vec![vec![0x010d_4cf8]]);
        assert_eq!(e.mem.u32(state + 0xc), 0);
        assert_ne!(e.mem.u32(state + 0x10), 0);
        assert_eq!(calls(&e, GET_PROC_ADDRESS).len(), 10);
    }

    #[test]
    fn without_the_library_only_a_message_is_drawn() {
        let mut e = engine();
        debug_help(&mut e, 0, &[]);
        let state = e.mem.alloc(0x30);
        e.mem.set_u32(state + 4, 0x77);
        e.call(0x00da_2840, &args![state]);
        assert_eq!(calls(&e, OUTPUT_DEBUG_STRING), vec![vec![0x010d_4da0]]);
        assert!(calls(&e, GET_PROC_ADDRESS).is_empty());
        assert_eq!(e.mem.u32(state), 0);
        assert_eq!(e.mem.u32(state + 4), 0x77);
    }

    #[test]
    fn the_tracer_cleans_up_the_library_and_restores_the_base_vtable() {
        let mut e = engine();
        answer(&mut e, GET_CURRENT_PROCESS, 0x4242);
        answer(&mut e, FREE_LIBRARY, 1);
        answer(&mut e, 0x0a00_0500, 1);
        let state = e.mem.alloc(0x30);
        e.mem.set_u32(state, 0x6000);
        e.mem.set_u32(state + 4, 1);
        for slot in 2..12u32 {
            e.mem.set_u32(state + 4 * slot, 0x0a00_0000 + 0x100 * slot);
        }
        e.mem.set_u32(state + 0x2c, 0x0a00_0500);
        let tracer = e.mem.alloc(0x10);
        e.mem.set_u32(tracer + 8, state);
        e.call(0x00da_2930, &args![tracer]);
        assert_eq!(calls(&e, 0x0a00_0500), vec![vec![0x4242]]);
        assert_eq!(calls(&e, FREE_LIBRARY), vec![vec![0x6000]]);
        assert_eq!(e.mem.u32(state), 0);
        assert_eq!(e.mem.u32(state + 4), 1);
        for slot in 2..12u32 {
            assert_eq!(e.mem.u32(state + 4 * slot), 0);
        }
        assert_eq!(e.mem.u32(tracer), 0x0102_e388);
    }

    #[test]
    fn a_tracer_without_a_library_only_swaps_the_vtable() {
        let mut e = engine();
        let state = e.mem.alloc(0x30);
        let tracer = e.mem.alloc(0x10);
        e.mem.set_u32(tracer + 8, state);
        e.call(0x00da_2930, &args![tracer]);
        assert_eq!(e.mem.u32(tracer), 0x0102_e388);
        assert!(calls(&e, GET_CURRENT_PROCESS).is_empty());
        assert!(calls(&e, FREE_LIBRARY).is_empty());
    }
}
