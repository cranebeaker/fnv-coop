//! The platform's process calls the C runtime's start-up and heap make (KERNEL32 imports):
//! `GetModuleFileNameA`, `GetCommandLineA`, `GetEnvironmentStringsW`/`FreeEnvironmentStringsW`,
//! `GetCurrentProcessId`, and `HeapFree`/`HeapReAlloc` on the CRT's heap.
//!
//! Measured on this machine (2026-10-27, `build43-scratch/measure/platform5`, a 32-bit program
//! calling the real functions; the paths and the environment it saw are this machine's and are
//! not recorded here, only how the calls behave):
//! - `GetModuleFileNameA(NULL, buffer, size)` for a path of `n` characters: with `size` above
//!   `n` it copies the path and its zero, answers `n` and sets the last error to 0; with `size`
//!   1..=`n` it copies the first `size - 1` characters and a zero, answers `size` and sets
//!   `ERROR_INSUFFICIENT_BUFFER` (122); with `size` 0 it writes nothing, answers 0 and sets 122.
//! - `GetCommandLineA` answers the same pointer on every call, and its text is
//!   `GetCommandLineW`'s (an ASCII command line, character for character).
//! - `GetEnvironmentStringsW` answers a new block on every call: the process's variables as
//!   `NAME=value` strings, each with its zero, in the order the process holds them (the order
//!   Rust's `std::env::vars_os` lists them; not sorted), and a final zero.
//!   `FreeEnvironmentStringsW` answers TRUE and leaves the last error as it was.
//! - `GetCurrentProcessId` answers the process's id (`std::process::id()`), the same on every
//!   call.
//! - `HeapFree(heap, 0, NULL)` answers TRUE; `HeapFree` of a block answers TRUE; neither changes
//!   the last error. `HeapReAlloc(heap, flags, block, size)` keeps the block's bytes up to the
//!   smaller size, with `HEAP_ZERO_MEMORY` (8) zeroes the bytes added by growing, makes
//!   `HeapSize` answer `size` (0 included) and leaves the last error as it was; whether the
//!   block moves is the heap's choice (here the platform allocator's).

use super::KernelObject;
use crate::{Engine, Ptr};
use std::collections::BTreeSet;

/// `ERROR_INSUFFICIENT_BUFFER`.
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
/// `HEAP_ZERO_MEMORY`.
pub const HEAP_ZERO_MEMORY: u32 = 8;

/// The process side's state (`Kernel::process`).
#[derive(Debug, Default)]
pub struct Process {
    /// Where `GetCommandLineA`'s text is (0: not made yet).
    command_line_a_at: u32,
    /// The environment blocks `GetEnvironmentStringsW` handed out and not freed.
    environment_blocks: BTreeSet<u32>,
    /// The filter `SetUnhandledExceptionFilter` set last (0: none).
    unhandled_exception_filter: u32,
}

/// The executable's full path as the game's ANSI string: the folder the process started in
/// (`Kernel::current_directory`, where Steam starts the game) and `FalloutNV.exe`, the same path
/// `GetCommandLineW` quotes.
pub fn executable_path(e: &Engine) -> Vec<u8> {
    let mut path = e.kernel.current_directory.clone();
    if !path.ends_with(b"\\") && !path.is_empty() {
        path.push(b'\\');
    }
    path.extend_from_slice(b"FalloutNV.exe");
    path
}

// Platform: replaces KERNEL32.DLL GetModuleFileNameA (import)
/// `GetModuleFileNameA(module, buffer, size)` for the executable (`module` NULL): its path
/// ([`executable_path`]) with the measured truncation (module comment). Another module is not
/// provided (stops the run).
pub fn get_module_file_name_a(e: &mut Engine, module: u32, buffer: Ptr, size: u32) -> u32 {
    assert!(
        module == 0,
        "GetModuleFileNameA({module:#x}): only the executable (NULL) is provided"
    );
    let path = executable_path(e);
    let n = path.len() as u32;
    if size == 0 {
        e.kernel.last_error = ERROR_INSUFFICIENT_BUFFER;
        return 0;
    }
    if size > n {
        e.mem.set_cstr(buffer.addr(), &path);
        e.kernel.last_error = 0;
        return n;
    }
    e.mem.set_cstr(buffer.addr(), &path[..(size - 1) as usize]);
    e.kernel.last_error = ERROR_INSUFFICIENT_BUFFER;
    size
}

// Platform: replaces KERNEL32.DLL GetCommandLineA (import)
/// `GetCommandLineA()`: the text `GetCommandLineW` answers (`super::window::get_command_line_w`)
/// as ANSI, made once; the same pointer on every call (measured). A character outside ASCII is
/// not provided (the code page's table is not captured; stops the run).
pub fn get_command_line_a(e: &mut Engine) -> u32 {
    if e.kernel.process.command_line_a_at == 0 {
        let mut at = super::window::get_command_line_w(e);
        let mut text = Vec::new();
        loop {
            let unit = e.mem.u16(at);
            if unit == 0 {
                break;
            }
            assert!(
                unit < 0x80,
                "GetCommandLineA: the non-ASCII character {unit:#x} is not provided"
            );
            text.push(unit as u8);
            at += 2;
        }
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, &text);
        e.kernel.process.command_line_a_at = block;
    }
    e.kernel.process.command_line_a_at
}

/// The process's environment as `GetEnvironmentStringsW` lays it out (module comment): the
/// UTF-16 units of each `NAME=value` and its zero, then the final zero.
pub fn environment_units() -> Vec<u16> {
    let mut units = Vec::new();
    for (name, value) in std::env::vars_os() {
        let entry = format!("{}={}", name.to_string_lossy(), value.to_string_lossy());
        units.extend(entry.encode_utf16());
        units.push(0);
    }
    units.push(0);
    units
}

// Platform: replaces KERNEL32.DLL GetEnvironmentStringsW (import)
/// `GetEnvironmentStringsW()`: a new block (measured) holding the process's environment
/// ([`environment_units`]): the variables this process runs with, as Windows gives a program
/// its parent's. An empty environment is not provided (stops the run).
pub fn get_environment_strings_w(e: &mut Engine) -> u32 {
    let units = environment_units();
    assert!(
        units.len() > 1,
        "GetEnvironmentStringsW: an empty environment is not provided"
    );
    let block = e.mem.alloc(2 * units.len() as u32);
    for (i, unit) in units.iter().enumerate() {
        e.mem.set_u16(block + 2 * i as u32, *unit);
    }
    e.kernel.process.environment_blocks.insert(block);
    block
}

// Platform: replaces KERNEL32.DLL FreeEnvironmentStringsW (import)
/// `FreeEnvironmentStringsW(block)`: frees a block of [`get_environment_strings_w`] and answers
/// TRUE, the last error left as it was (measured). Any other address is not provided.
pub fn free_environment_strings_w(e: &mut Engine, block: u32) -> u32 {
    assert!(
        e.kernel.process.environment_blocks.remove(&block),
        "FreeEnvironmentStringsW({block:#x}): not a block GetEnvironmentStringsW answered (not provided)"
    );
    e.mem.free(block);
    1
}

// Platform: replaces KERNEL32.DLL SetUnhandledExceptionFilter (import)
/// `SetUnhandledExceptionFilter(filter)`: keeps `filter` (null included) and answers the one
/// set before, null for the first call of the process; the last error is left as it was.
/// Measured on this machine (2026-10-27, `build43-scratch/measure/suef-nocrt` and
/// `platform5`): in a 32-bit process no C runtime started, the first call answered null and the
/// second the first's filter; in one whose runtime start-up had set its own filter, the first
/// call answered that one; each later call answered the one before. The filter is only kept:
/// nothing here raises an unhandled exception to call it.
pub fn set_unhandled_exception_filter(e: &mut Engine, filter: u32) -> u32 {
    std::mem::replace(&mut e.kernel.process.unhandled_exception_filter, filter)
}

// Platform: replaces KERNEL32.DLL GetCurrentProcessId (import)
/// `GetCurrentProcessId()`: the id of the process the game runs in (this one), the same on
/// every call (measured).
pub fn get_current_process_id(_e: &mut Engine) -> u32 {
    std::process::id()
}

/// Whether `heap` is a heap of `HeapCreate` (`super::heap_create`).
fn is_heap(e: &Engine, heap: u32) -> bool {
    matches!(e.kernel.object(heap), Some(KernelObject::Heap))
}

// Platform: replaces KERNEL32.DLL HeapFree (import)
/// `HeapFree(heap, flags, block)` on the CRT's heap (`_crtheap`, from `HeapCreate`): a null
/// block answers TRUE; a block the heap handed out (a platform block, as `HeapAlloc`,
/// `HeapReAlloc` and the platform `malloc` make them) is freed and answers TRUE; the last error
/// is left as it was (measured). Another heap, flags other than 0 or another address are not
/// provided (stop the run).
pub fn heap_free(e: &mut Engine, heap: u32, flags: u32, block: u32) -> u32 {
    assert!(
        is_heap(e, heap) && flags == 0,
        "HeapFree(heap {heap:#x}, flags {flags:#x}) is not provided"
    );
    if block == 0 {
        return 1;
    }
    assert!(
        e.kernel.crt_block_sizes.remove(&block).is_some(),
        "HeapFree of {block:08x}, which the CRT's heap did not hand out: not provided"
    );
    e.mem.free(block);
    1
}

// Platform: replaces KERNEL32.DLL HeapReAlloc (import)
/// `HeapReAlloc(heap, flags, block, size)` on the CRT's heap with flags 0 or `HEAP_ZERO_MEMORY`:
/// a block of `size` bytes holding the old block's bytes up to the smaller size (the bytes
/// added by growing zero with `HEAP_ZERO_MEMORY`; the platform's blocks start zeroed in any
/// case), whose `HeapSize` is `size`; the old block is freed; the last error is left as it was
/// (measured). Another heap, other flags, a block the heap did not hand out or a null block are
/// not provided (stop the run).
pub fn heap_re_alloc(e: &mut Engine, heap: u32, flags: u32, block: u32, size: u32) -> u32 {
    assert!(
        is_heap(e, heap) && flags & !HEAP_ZERO_MEMORY == 0,
        "HeapReAlloc(heap {heap:#x}, flags {flags:#x}) is not provided"
    );
    let Some(old) = e.kernel.crt_block_sizes.get(&block).copied() else {
        panic!("HeapReAlloc of {block:08x}, which the CRT's heap did not hand out: not provided");
    };
    let new = e.mem.alloc(size);
    let kept = e.mem.bytes(block, old.min(size));
    e.mem.write(new, &kept);
    if flags & HEAP_ZERO_MEMORY != 0 && size > old {
        e.mem.write(new + old, &vec![0u8; (size - old) as usize]);
    }
    e.kernel.crt_block_sizes.remove(&block);
    e.mem.free(block);
    e.kernel.crt_block_sizes.insert(new, size);
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.kernel.current_directory = b"C:\\Games\\FNV".to_vec();
        e
    }

    #[test]
    fn module_file_name_truncates_as_measured() {
        let mut e = engine();
        let path = b"C:\\Games\\FNV\\FalloutNV.exe";
        let n = path.len() as u32;
        let buffer = e.mem.alloc(0x40);
        let fill = |e: &mut Engine| e.mem.write(buffer, &[0xaa; 0x40]);
        fill(&mut e);
        e.kernel.last_error = 0x1234;
        assert_eq!(
            get_module_file_name_a(&mut e, 0, Ptr::new(buffer), 0x104),
            n
        );
        assert_eq!(e.mem.cstr(buffer), path.to_vec());
        assert_eq!(e.kernel.last_error, 0);
        // Exactly the length: one character short and a zero, ERROR_INSUFFICIENT_BUFFER.
        fill(&mut e);
        assert_eq!(get_module_file_name_a(&mut e, 0, Ptr::new(buffer), n), n);
        assert_eq!(e.mem.cstr(buffer), path[..path.len() - 1].to_vec());
        assert_eq!(e.mem.u8(buffer + n), 0xaa);
        assert_eq!(e.kernel.last_error, 122);
        fill(&mut e);
        assert_eq!(get_module_file_name_a(&mut e, 0, Ptr::new(buffer), 1), 1);
        assert_eq!(e.mem.bytes(buffer, 2), vec![0, 0xaa]);
        // Size 0 writes nothing.
        fill(&mut e);
        e.kernel.last_error = 0;
        assert_eq!(get_module_file_name_a(&mut e, 0, Ptr::new(buffer), 0), 0);
        assert_eq!(e.mem.u8(buffer), 0xaa);
        assert_eq!(e.kernel.last_error, 122);
    }

    #[test]
    fn the_ansi_command_line_is_the_wide_one_made_once() {
        let mut e = engine();
        let a = get_command_line_a(&mut e);
        assert_eq!(e.mem.cstr(a), b"\"C:\\Games\\FNV\\FalloutNV.exe\"".to_vec());
        assert_eq!(get_command_line_a(&mut e), a);
    }

    #[test]
    fn environment_blocks_are_new_each_time_and_freed() {
        let mut e = engine();
        let a = get_environment_strings_w(&mut e);
        let b = get_environment_strings_w(&mut e);
        assert_ne!(a, b);
        let units = environment_units();
        for (i, unit) in units.iter().enumerate() {
            assert_eq!(e.mem.u16(a + 2 * i as u32), *unit);
        }
        assert_eq!(units[units.len() - 1], 0);
        assert_eq!(units[units.len() - 2], 0);
        e.kernel.last_error = 0x1234;
        assert_eq!(free_environment_strings_w(&mut e, a), 1);
        assert_eq!(free_environment_strings_w(&mut e, b), 1);
        assert_eq!(e.kernel.last_error, 0x1234);
    }

    #[test]
    fn the_unhandled_exception_filter_answers_the_one_before() {
        let mut e = engine();
        e.kernel.last_error = 0x1234;
        assert_eq!(set_unhandled_exception_filter(&mut e, 0x00ed_d4a5), 0);
        assert_eq!(
            set_unhandled_exception_filter(&mut e, 0x0040_1000),
            0x00ed_d4a5
        );
        assert_eq!(set_unhandled_exception_filter(&mut e, 0), 0x0040_1000);
        assert_eq!(set_unhandled_exception_filter(&mut e, 0), 0);
        assert_eq!(e.kernel.last_error, 0x1234);
    }

    #[test]
    fn the_process_id_is_this_process() {
        let mut e = engine();
        assert_eq!(get_current_process_id(&mut e), std::process::id());
    }

    #[test]
    fn heap_blocks_grow_with_zeros_and_free() {
        let mut e = engine();
        let heap = super::super::heap_create(&mut e, 0, 0x1000, 0);
        let block = crate::units::crt::crt_alloc(&mut e, 0x20);
        let pattern: Vec<u8> = (1..=0x20).collect();
        e.mem.write(block, &pattern);
        e.kernel.last_error = 0x1234;
        let grown = heap_re_alloc(&mut e, heap, HEAP_ZERO_MEMORY, block, 0x40);
        assert_eq!(e.mem.bytes(grown, 0x20), pattern);
        assert!(e.mem.bytes(grown + 0x20, 0x20).iter().all(|&b| b == 0));
        assert_eq!(e.kernel.crt_block_sizes.get(&grown), Some(&0x40));
        assert!(!e.kernel.crt_block_sizes.contains_key(&block) || grown == block);
        let shrunk = heap_re_alloc(&mut e, heap, 0, grown, 0);
        assert_eq!(e.kernel.crt_block_sizes.get(&shrunk), Some(&0));
        assert_eq!(heap_free(&mut e, heap, 0, 0), 1);
        assert_eq!(heap_free(&mut e, heap, 0, shrunk), 1);
        assert!(!e.kernel.crt_block_sizes.contains_key(&shrunk));
        assert_eq!(e.kernel.last_error, 0x1234);
    }
}
