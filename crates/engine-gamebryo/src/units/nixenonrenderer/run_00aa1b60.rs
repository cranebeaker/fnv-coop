//! `NiXenonRenderer/run_00aa1b60` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function pointer `011ac224` and its default, `00aa1b60`.
const HANDLER: u32 = 0x011a_c224;
const DEFAULT_HANDLER: u32 = 0x00aa_1b60;

// Translated from 00aa1b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aa1b60`): installs `handler` in the function pointer
/// `011ac224` (cdecl, one word); a null handler installs the default `00aa1b60` (which the
/// exe stores first either way).
pub fn fn_00aa1b90(e: &mut Engine, handler: u32) {
    e.mem.set_u32(HANDLER, DEFAULT_HANDLER);
    if handler != 0 {
        e.mem.set_u32(HANDLER, handler);
    }
}

/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, the size), and
/// the constructor `00aa14d0` (`thiscall`, three words) run on the 0x30-byte block.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const CONSTRUCT_00AA14D0: u32 = 0x00aa_14d0;

// Translated from 00aa1b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words): the default of the function pointer
/// `011ac224`. Allocates 0x30 bytes from the memory manager and returns the object
/// `00aa14d0` constructs there from the three words (its result), or 0 when the allocation
/// fails.
pub fn fn_00aa1b60(e: &mut Engine, a1: u32, a2: u32, a3: u32) -> u32 {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x30u32])
        .u32();
    if block == 0 {
        return 0;
    }
    e.call(CONSTRUCT_00AA14D0, &args![block, a1, a2, a3]).u32()
}

/// The two path function pointers `fn_00aa1bb0` calls (`cdecl`, the path; a byte answer):
/// `011ac230` (its default `00aa1670` answers whether the path is a directory, through
/// `GetFileAttributesA`) and `011ac22c` (its default `00aa1650` makes the directory, through
/// `CreateDirectoryA`).
const IS_DIRECTORY_POINTER: u32 = 0x011a_c230;
const MAKE_DIRECTORY_POINTER: u32 = 0x011a_c22c;
/// `_strcpy_s` (`00ec65a6`, `cdecl`: destination, size, source) and `NiPath::Standardize`
/// (`00aa2a00`, `cdecl`, the path).
const STRCPY_S: u32 = 0x00ec_65a6;
const NI_PATH_STANDARDIZE: u32 = 0x00aa_2a00;
/// The local path buffer's size.
const PATH_SIZE: u32 = 0x104;

fn path_call(e: &mut Engine, pointer: u32, path: u32) -> bool {
    let function = e.mem.u32(pointer);
    e.call(function, &args![path]).u8() != 0
}

// Translated from 00aa1bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the path; called by the shader save and load code):
/// makes sure every directory of `path` exists. `true` at once when the whole path answers
/// the check through `011ac230`; `false` for a path longer than 0x104 characters. Otherwise
/// the path is copied into a 0x104-byte local (`_strcpy_s`) and standardized
/// (`NiPath::Standardize`); after a drive (`x:`) and any leading separators, at each `/` or
/// `\` up to the buffer's end (the scan runs past the copied text, over whatever the buffer
/// holds) the prefix before it must pass the check or be made through `011ac22c`, else the
/// result is `false`. Finally the whole buffer must pass the check or be made.
// Translation incomplete: 00aa1bb0 - the /GS stack cookie check (00ec408c) is not modelled.
pub fn fn_00aa1bb0(e: &mut Engine, path: u32) -> bool {
    if path_call(e, IS_DIRECTORY_POINTER, path) {
        return true;
    }
    let mut end = path;
    while e.mem.u8(end) != 0 {
        end = end.wrapping_add(1);
    }
    if end.wrapping_sub(path) > PATH_SIZE {
        return false;
    }
    e.with_stack(PATH_SIZE + 4, |e, buffer| {
        let buffer = buffer.addr();
        e.call(STRCPY_S, &args![buffer, PATH_SIZE, path]);
        e.call(NI_PATH_STANDARDIZE, &args![buffer]);
        let mut index = if e.mem.u8(buffer + 1) == b':' { 2 } else { 0 };
        loop {
            let c = e.mem.u8(buffer + index);
            if c != b'\\' && c != b'/' {
                break;
            }
            index += 1;
            if index >= PATH_SIZE {
                break;
            }
        }
        while index < PATH_SIZE {
            let c = e.mem.u8(buffer + index);
            if c == b'/' || c == b'\\' {
                e.mem.set_u8(buffer + index, 0);
                if !path_call(e, IS_DIRECTORY_POINTER, buffer)
                    && !path_call(e, MAKE_DIRECTORY_POINTER, buffer)
                {
                    return false;
                }
                e.mem.set_u8(buffer + index, c);
            }
            index += 1;
        }
        path_call(e, IS_DIRECTORY_POINTER, buffer) || path_call(e, MAKE_DIRECTORY_POINTER, buffer)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_1b90, fn_00aa1b90(u32)),
        entry!(0x00aa_1b60, fn_00aa1b60(u32, u32, u32) -> u32),
        entry!(0x00aa_1bb0, fn_00aa1bb0(u32) -> bool),
    ]
}

#[cfg(test)]
mod n010_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00aa1b60_constructs_in_a_new_block() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0x0e10_0000u32.into_ret());
        e.register(CONSTRUCT_00AA14D0, |_, a| (a[0] + 4).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_1b60, &args![1u32, 2u32, 3u32]).u32(),
            0x0e10_0004
        );
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMORY_MANAGER_ALLOCATE, vec![MEMORY_MANAGER, 0x30])));
        assert!(log.contains(&(CONSTRUCT_00AA14D0, vec![0x0e10_0000, 1, 2, 3])));
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_1b60, &args![1u32, 2u32, 3u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == CONSTRUCT_00AA14D0));
    }

    const CHECK: u32 = 0x0e50_0000;
    const MAKE: u32 = 0x0e60_0000;

    type Seen = Rc<RefCell<Vec<(u32, String)>>>;

    /// An engine whose check answers `true` for the paths in `existing` and whose make
    /// answers `make_works`; every call's path is recorded.
    fn path_engine(existing: &'static [&'static str], make_works: bool) -> (Engine, Seen) {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.mem.set_u32(IS_DIRECTORY_POINTER, CHECK);
        e.mem.set_u32(MAKE_DIRECTORY_POINTER, MAKE);
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        fn text(e: &Engine, mut at: u32) -> String {
            let mut s = String::new();
            while e.mem.u8(at) != 0 {
                s.push(e.mem.u8(at) as char);
                at += 1;
            }
            s
        }
        let record = seen.clone();
        e.register_double(CHECK, move |e, a| {
            let path = text(e, a[0]);
            let answer = existing.contains(&path.as_str());
            record.borrow_mut().push((CHECK, path));
            u32::from(answer).into_ret()
        });
        let record = seen.clone();
        e.register_double(MAKE, move |e, a| {
            record.borrow_mut().push((MAKE, text(e, a[0])));
            u32::from(make_works).into_ret()
        });
        // `_strcpy_s` copies with the terminator and fills the rest with 'x' (so the scan
        // past the text sees no separator); the standardize double leaves the path.
        e.register(STRCPY_S, |e, a| {
            let mut i = 0;
            loop {
                let c = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, c);
                i += 1;
                if c == 0 {
                    break;
                }
            }
            while i < a[1] {
                e.mem.set_u8(a[0] + i, b'x');
                i += 1;
            }
            Ret::default()
        });
        e.register(NI_PATH_STANDARDIZE, |_, _| Ret::default());
        (e, seen)
    }

    fn put(e: &mut Engine, text: &str) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(at, text.as_bytes());
        e.mem.set_u8(at + text.len() as u32, 0);
        at
    }

    #[test]
    fn fn_00aa1bb0_answers_true_for_an_existing_directory() {
        let (mut e, seen) = path_engine(&["c:\\a\\b"], false);
        let path = put(&mut e, "c:\\a\\b");
        assert!(e.call(0x00aa_1bb0, &args![path]).bool());
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn fn_00aa1bb0_makes_each_directory_of_the_path() {
        let (mut e, seen) = path_engine(&["c:"], true);
        let path = put(&mut e, "c:\\a\\b");
        assert!(e.call(0x00aa_1bb0, &args![path]).bool());
        let seen = seen.borrow();
        // The whole path first, then the prefix at the separator after "a" (the one after
        // the drive is skipped), then the whole buffer.
        assert_eq!(seen[0], (CHECK, "c:\\a\\b".to_string()));
        assert_eq!(seen[1], (CHECK, "c:\\a".to_string()));
        assert_eq!(seen[2], (MAKE, "c:\\a".to_string()));
        assert_eq!(seen[3].0, CHECK);
        assert!(seen[3].1.starts_with("c:\\a\\b"));
        assert_eq!(seen[4].0, MAKE);
        assert_eq!(seen.len(), 5);
    }

    #[test]
    fn fn_00aa1bb0_fails_when_a_directory_cannot_be_made() {
        let (mut e, seen) = path_engine(&[], false);
        let path = put(&mut e, "/a/b");
        assert!(!e.call(0x00aa_1bb0, &args![path]).bool());
        let seen = seen.borrow();
        // The leading separator is skipped: the first prefix is "/a".
        assert_eq!(seen[1], (CHECK, "/a".to_string()));
        assert_eq!(seen[2], (MAKE, "/a".to_string()));
        assert_eq!(seen.len(), 3);
    }

    #[test]
    fn fn_00aa1bb0_refuses_a_path_longer_than_the_buffer() {
        let (mut e, seen) = path_engine(&[], true);
        let long = "a".repeat(0x105);
        let path = put(&mut e, &long);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00aa_1bb0, &args![path]).bool());
        assert_eq!(seen.borrow().len(), 1);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == STRCPY_S));
    }
}

#[cfg(test)]
mod b049_tests {
    use super::*;

    #[test]
    fn fn_00aa1b90_installs_the_handler_or_the_default() {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.call(0x00aa_1b90, &args![0x1234u32]);
        assert_eq!(e.mem.u32(HANDLER), 0x1234);
        e.call(0x00aa_1b90, &args![0u32]);
        assert_eq!(e.mem.u32(HANDLER), DEFAULT_HANDLER);
    }
}
