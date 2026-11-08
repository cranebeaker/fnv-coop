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

/// The process side's state (`State::process`).
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
    if state!(e).process.command_line_a_at == 0 {
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
        state!(e).process.command_line_a_at = block;
    }
    state!(e).process.command_line_a_at
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
    state!(e).process.environment_blocks.insert(block);
    block
}

// Platform: replaces KERNEL32.DLL FreeEnvironmentStringsW (import)
/// `FreeEnvironmentStringsW(block)`: frees a block of [`get_environment_strings_w`] and answers
/// TRUE, the last error left as it was (measured). Any other address is not provided.
pub fn free_environment_strings_w(e: &mut Engine, block: u32) -> u32 {
    assert!(
        state!(e).process.environment_blocks.remove(&block),
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
    std::mem::replace(&mut state!(e).process.unhandled_exception_filter, filter)
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
        e.mem.forget_asked(block).is_some(),
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
    let Some(old) = e.mem.asked_size(block) else {
        panic!("HeapReAlloc of {block:08x}, which the CRT's heap did not hand out: not provided");
    };
    let new = e.mem.alloc(size);
    let kept = e.mem.bytes(block, old.min(size));
    e.mem.write(new, &kept);
    if flags & HEAP_ZERO_MEMORY != 0 && size > old {
        e.mem.write(new + old, &vec![0u8; (size - old) as usize]);
    }
    e.mem.forget_asked(block);
    e.mem.free(block);
    e.mem.set_asked(new, size);
    new
}

/// `MEMORYSTATUS(EX)`'s total virtual size for this process, a large-address-aware 32-bit one on
/// 64-bit Windows: the span of `SYSTEM_INFO`'s application addresses, 0x10000..=0xFFFEFFFF
/// ([`super::MAXIMUM_APPLICATION_ADDRESS`]). A vanilla preset, once there is one, would be a plain
/// 32-bit process here too: 0x7FFE0000, with `dwTotalPhys` and the other sizes clamped at 2 GB.
pub const TOTAL_VIRTUAL: u32 = super::MAXIMUM_APPLICATION_ADDRESS + 1 - 0x1_0000;

/// `ERROR_INVALID_PARAMETER`.
const ERROR_INVALID_PARAMETER: u32 = 87;
/// `MEMORYSTATUSEX`'s size, which its `dwLength` must hold.
const MEMORY_STATUS_EX_BYTES: u32 = 0x40;

/// What `GlobalMemoryStatus(Ex)` report of the machine rather than of the process.
#[derive(Clone, Copy, Default)]
struct MachineMemory {
    load: u32,
    total_phys: u64,
    avail_phys: u64,
    total_page_file: u64,
    avail_page_file: u64,
}

/// The real machine's memory, as `GetSystemInfo` reports its real processors.
fn machine_memory() -> Option<MachineMemory> {
    #[cfg(windows)]
    {
        use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        let mut status = MEMORYSTATUSEX {
            dwLength: MEMORY_STATUS_EX_BYTES,
            ..Default::default()
        };
        // SAFETY: `status` is a live `MEMORYSTATUSEX` with its size in `dwLength`.
        unsafe { GlobalMemoryStatusEx(&mut status) }.ok()?;
        Some(MachineMemory {
            load: status.dwMemoryLoad,
            total_phys: status.ullTotalPhys,
            avail_phys: status.ullAvailPhys,
            total_page_file: status.ullTotalPageFile,
            avail_page_file: status.ullAvailPageFile,
        })
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// `MEMORYSTATUS.dwAvailVirtual`: what the heap can still hand out, which is what a caller can
/// still get; the free addresses outside the heap serve only reservations at a chosen address.
fn avail_virtual(e: &Engine) -> u64 {
    e.mem.heap_available().min(u64::from(TOTAL_VIRTUAL))
}

fn write_memory_status(e: &mut Engine, p: u32, machine: MachineMemory) {
    // Documented: a size that does not fit in the 32-bit field reports -1.
    let word = |size: u64| size.min(u64::from(u32::MAX)) as u32;
    let avail = avail_virtual(e) as u32;
    e.mem.set_u32(p, 0x20);
    e.mem.set_u32(p + 0x04, machine.load);
    e.mem.set_u32(p + 0x08, word(machine.total_phys));
    e.mem.set_u32(p + 0x0c, word(machine.avail_phys));
    e.mem.set_u32(p + 0x10, word(machine.total_page_file));
    e.mem.set_u32(p + 0x14, word(machine.avail_page_file));
    e.mem.set_u32(p + 0x18, TOTAL_VIRTUAL);
    e.mem.set_u32(p + 0x1c, avail);
}

fn write_memory_status_ex(e: &mut Engine, p: u32, machine: MachineMemory) {
    let avail = avail_virtual(e);
    e.mem.set_u32(p + 0x04, machine.load);
    e.mem.set_u64(p + 0x08, machine.total_phys);
    e.mem.set_u64(p + 0x10, machine.avail_phys);
    e.mem.set_u64(p + 0x18, machine.total_page_file);
    e.mem.set_u64(p + 0x20, machine.avail_page_file);
    e.mem.set_u64(p + 0x28, u64::from(TOTAL_VIRTUAL));
    e.mem.set_u64(p + 0x30, avail);
    e.mem.set_u64(p + 0x38, 0);
}

// Platform: replaces KERNEL32.DLL GlobalMemoryStatus (import)
/// Fills the 0x20-byte `MEMORYSTATUS` for the 4 GB process: the machine's real load, physical
/// memory and page file, the virtual sizes of [`TOTAL_VIRTUAL`] and [`avail_virtual`]. The exe's
/// one caller reads `dwAvailPhys`: `00aa9879`, in the free-list report `00aa9760`, which nothing
/// calls. From Microsoft's documentation; not measured against a 32-bit program.
pub fn global_memory_status(e: &mut Engine, out: Ptr) {
    write_memory_status(e, out.addr(), machine_memory().unwrap_or_default());
}

// Platform: replaces KERNEL32.DLL GlobalMemoryStatusEx (import)
/// Fills the 0x40-byte `MEMORYSTATUSEX` as [`global_memory_status`] does, its sizes unclamped.
/// FALSE with `ERROR_INVALID_PARAMETER` when `dwLength` is not 0x40, or when the machine does not
/// answer. The exe's one caller reads `ullAvailPageFile`: `00aa3958`, in the memory manager's
/// constructor `00aa3880`, for which the host stands in (`STAND_INS`, `00fb66a0`).
pub fn global_memory_status_ex(e: &mut Engine, out: Ptr) -> u32 {
    if e.mem.u32(out.addr()) != MEMORY_STATUS_EX_BYTES {
        e.kernel.last_error = ERROR_INVALID_PARAMETER;
        return 0;
    }
    let Some(machine) = machine_memory() else {
        e.kernel.last_error = ERROR_INVALID_PARAMETER;
        return 0;
    };
    write_memory_status_ex(e, out.addr(), machine);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.kernel.current_directory = b"C:\\Games\\FNV".to_vec();
        e
    }

    /// The fields of both structures for a machine with more than the 32-bit fields hold, and
    /// the virtual sizes agreeing with `GetSystemInfo` and following the heap.
    #[test]
    fn memory_status_answers_describe_the_4_gb_process() {
        const AT: u32 = 0x1000;
        let mut e = engine();
        e.mem.map(AT, 0x1000);
        assert_eq!(TOTAL_VIRTUAL, 0xfffe_0000);
        let machine = MachineMemory {
            load: 40,
            total_phys: 32 << 30,
            avail_phys: 0xc000_0000,
            total_page_file: 40 << 30,
            avail_page_file: 20 << 30,
        };
        write_memory_status(&mut e, AT, machine);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(AT + i * 4)).collect();
        let room = e.mem.heap_available();
        // The two ranges of the heap, less what the engine has taken already.
        assert!(room > 0xc000_0000 && room <= 0xce00_0000, "{room:#x}");
        assert_eq!(
            words,
            [
                0x20,
                40,
                u32::MAX,
                0xc000_0000,
                u32::MAX,
                u32::MAX,
                0xfffe_0000,
                room as u32
            ]
        );
        write_memory_status_ex(&mut e, AT, machine);
        assert_eq!(e.mem.u32(AT + 0x04), 40);
        assert_eq!(e.mem.u64(AT + 0x08), 32 << 30);
        assert_eq!(e.mem.u64(AT + 0x10), 0xc000_0000);
        assert_eq!(e.mem.u64(AT + 0x18), 40 << 30);
        assert_eq!(e.mem.u64(AT + 0x20), 20 << 30);
        assert_eq!(e.mem.u64(AT + 0x28), 0xfffe_0000);
        assert_eq!(e.mem.u64(AT + 0x30), room);
        assert_eq!(e.mem.u64(AT + 0x38), 0);
        let block = e.mem.alloc(0x10_0000);
        assert_eq!(avail_virtual(&e), room - 0x10_0000);
        e.mem.free(block);
    }

    /// The import itself: `dwLength` is checked first, and the real machine answers.
    #[test]
    fn memory_status_ex_checks_its_length() {
        const AT: u32 = 0x1000;
        let mut e = engine();
        e.mem.map(AT, 0x1000);
        e.kernel.last_error = 0;
        e.mem.set_u32(AT, 0x20);
        e.mem.set_u64(AT + 0x28, 0x1234);
        assert_eq!(global_memory_status_ex(&mut e, Ptr::new(AT)), 0);
        assert_eq!(e.kernel.last_error, 87);
        assert_eq!(e.mem.u64(AT + 0x28), 0x1234, "nothing written");
        // The machine's sizes are only read on Windows so far.
        if cfg!(windows) {
            e.mem.set_u32(AT, 0x40);
            assert_eq!(global_memory_status_ex(&mut e, Ptr::new(AT)), 1);
            assert_eq!(e.mem.u32(AT), 0x40);
            assert!(e.mem.u64(AT + 0x08) > 0, "the machine's physical memory");
            assert_eq!(e.mem.u64(AT + 0x28), 0xfffe_0000);
            global_memory_status(&mut e, Ptr::new(AT));
            assert_eq!(e.mem.u32(AT), 0x20);
            assert_eq!(e.mem.u32(AT + 0x18), 0xfffe_0000);
        }
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
        let heap = engine_core::units::platform::test_support::heap_create(&mut e, 0, 0x1000, 0);
        let block = engine_core::units::crt::stdio::test_support::crt_alloc(&mut e, 0x20);
        let pattern: Vec<u8> = (1..=0x20).collect();
        e.mem.write(block, &pattern);
        e.kernel.last_error = 0x1234;
        let grown = heap_re_alloc(&mut e, heap, HEAP_ZERO_MEMORY, block, 0x40);
        assert_eq!(e.mem.bytes(grown, 0x20), pattern);
        assert!(e.mem.bytes(grown + 0x20, 0x20).iter().all(|&b| b == 0));
        assert_eq!(e.mem.asked_size(grown), Some(0x40));
        assert!(e.mem.asked_size(block).is_none() || grown == block);
        let shrunk = heap_re_alloc(&mut e, heap, 0, grown, 0);
        assert_eq!(e.mem.asked_size(shrunk), Some(0));
        assert_eq!(heap_free(&mut e, heap, 0, 0), 1);
        assert_eq!(heap_free(&mut e, heap, 0, shrunk), 1);
        assert!(e.mem.asked_size(shrunk).is_none());
        assert_eq!(e.kernel.last_error, 0x1234);
    }
}
