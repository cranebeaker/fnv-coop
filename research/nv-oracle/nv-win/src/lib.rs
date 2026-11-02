//! Hand-written declarations for the few Windows API functions the nv-oracle
//! tools call. Only 32-bit Windows is supported; elsewhere this crate is
//! empty so the rest of the workspace can still be checked on a Linux host.
//!
//! Everything links against `kernel32`, which every Windows program already
//! loads, so no import libraries beyond the toolchain's own are needed.

#![cfg(windows)]
#![allow(non_snake_case, clippy::upper_case_acronyms)]

use std::ffi::c_void;
use std::mem::size_of;

pub type Handle = *mut c_void;
pub type Bool = i32;

pub const INVALID_HANDLE_VALUE: Handle = -1isize as Handle;

pub const MEM_COMMIT: u32 = 0x1000;
pub const MEM_RESERVE: u32 = 0x2000;
pub const MEM_RELEASE: u32 = 0x8000;
pub const MEM_FREE: u32 = 0x10000;

pub const PAGE_NOACCESS: u32 = 0x01;
pub const PAGE_READONLY: u32 = 0x02;
pub const PAGE_READWRITE: u32 = 0x04;
pub const PAGE_WRITECOPY: u32 = 0x08;
pub const PAGE_EXECUTE: u32 = 0x10;
pub const PAGE_EXECUTE_READ: u32 = 0x20;
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;
pub const PAGE_GUARD: u32 = 0x100;

pub const DLL_PROCESS_DETACH: u32 = 0;
pub const DLL_PROCESS_ATTACH: u32 = 1;
pub const DLL_THREAD_ATTACH: u32 = 2;
pub const DLL_THREAD_DETACH: u32 = 3;

pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 0x1;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 0x4;

pub const CREATE_SUSPENDED: u32 = 0x4;
pub const INFINITE: u32 = 0xFFFF_FFFF;
pub const WAIT_OBJECT_0: u32 = 0;
pub const TH32CS_SNAPTHREAD: u32 = 0x4;
pub const THREAD_SUSPEND_RESUME: u32 = 0x2;
pub const THREAD_GET_CONTEXT: u32 = 0x8;
pub const THREAD_SET_CONTEXT: u32 = 0x10;
pub const CONTEXT_CONTROL: u32 = 0x0001_0001;
pub const CONTEXT_INTEGER: u32 = 0x0001_0002;
pub const CONTEXT_FULL_X86: u32 = 0x0001_0007;
pub const PROCESS_ALL_ACCESS: u32 = 0x001F_FFFF;
pub const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x1000;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x200;

#[repr(C)]
pub struct MemoryBasicInformation {
    pub base_address: *mut c_void,
    pub allocation_base: *mut c_void,
    pub allocation_protect: u32,
    pub region_size: usize,
    pub state: u32,
    pub protect: u32,
    pub kind: u32,
}

impl MemoryBasicInformation {
    pub fn zeroed() -> Self {
        // SAFETY: plain integers and pointers; all-zero is a valid value.
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct FloatingSaveArea {
    pub control_word: u32,
    pub status_word: u32,
    pub tag_word: u32,
    pub error_offset: u32,
    pub error_selector: u32,
    pub data_offset: u32,
    pub data_selector: u32,
    pub register_area: [u8; 80],
    pub cr0_npx_state: u32,
}

/// The 32-bit x86 `CONTEXT` structure.
#[repr(C)]
pub struct Context {
    pub context_flags: u32,
    pub dr0: u32,
    pub dr1: u32,
    pub dr2: u32,
    pub dr3: u32,
    pub dr6: u32,
    pub dr7: u32,
    pub float_save: FloatingSaveArea,
    pub seg_gs: u32,
    pub seg_fs: u32,
    pub seg_es: u32,
    pub seg_ds: u32,
    pub edi: u32,
    pub esi: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
    pub ebp: u32,
    pub eip: u32,
    pub seg_cs: u32,
    pub eflags: u32,
    pub esp: u32,
    pub seg_ss: u32,
    pub extended_registers: [u8; 512],
}

impl Context {
    pub fn zeroed() -> Self {
        // SAFETY: plain integers; all-zero is a valid value.
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct ExceptionRecord {
    pub exception_code: u32,
    pub exception_flags: u32,
    pub exception_record: *mut ExceptionRecord,
    pub exception_address: *mut c_void,
    pub number_parameters: u32,
    pub exception_information: [usize; 15],
}

/// The argument of a top-level exception filter.
#[repr(C)]
pub struct ExceptionPointers {
    pub exception_record: *mut ExceptionRecord,
    pub context_record: *mut Context,
}

#[repr(C)]
pub struct StartupInfoW {
    pub cb: u32,
    pub reserved: *mut u16,
    pub desktop: *mut u16,
    pub title: *mut u16,
    pub x: u32,
    pub y: u32,
    pub x_size: u32,
    pub y_size: u32,
    pub x_count_chars: u32,
    pub y_count_chars: u32,
    pub fill_attribute: u32,
    pub flags: u32,
    pub show_window: u16,
    pub cb_reserved2: u16,
    pub reserved2: *mut u8,
    pub std_input: Handle,
    pub std_output: Handle,
    pub std_error: Handle,
}

impl StartupInfoW {
    pub fn new() -> Self {
        // SAFETY: plain integers and null pointers.
        let mut si: StartupInfoW = unsafe { std::mem::zeroed() };
        si.cb = size_of::<StartupInfoW>() as u32;
        si
    }
}

impl Default for StartupInfoW {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
pub struct ProcessInformation {
    pub process: Handle,
    pub thread: Handle,
    pub process_id: u32,
    pub thread_id: u32,
}

#[repr(C)]
pub struct ThreadEntry32 {
    pub size: u32,
    pub usage: u32,
    pub thread_id: u32,
    pub owner_process_id: u32,
    pub base_priority: i32,
    pub delta_priority: i32,
    pub flags: u32,
}

const _: () = assert!(size_of::<MemoryBasicInformation>() == 28);
const _: () = assert!(size_of::<Context>() == 716);
const _: () = assert!(size_of::<ExceptionRecord>() == 80);
const _: () = assert!(size_of::<StartupInfoW>() == 68);
const _: () = assert!(size_of::<ProcessInformation>() == 16);
const _: () = assert!(size_of::<ThreadEntry32>() == 28);

pub type ThreadStart = unsafe extern "system" fn(*mut c_void) -> u32;
pub type TopLevelFilter = unsafe extern "system" fn(*const ExceptionPointers) -> i32;

pub const STD_ERROR_HANDLE: u32 = -12i32 as u32;

#[link(name = "kernel32")]
extern "system" {
    pub fn VirtualAlloc(
        addr: *mut c_void,
        size: usize,
        alloc_type: u32,
        protect: u32,
    ) -> *mut c_void;
    pub fn VirtualFree(addr: *mut c_void, size: usize, free_type: u32) -> Bool;
    pub fn VirtualProtect(
        addr: *mut c_void,
        size: usize,
        new_protect: u32,
        old_protect: *mut u32,
    ) -> Bool;
    pub fn VirtualQuery(
        addr: *const c_void,
        info: *mut MemoryBasicInformation,
        len: usize,
    ) -> usize;
    pub fn VirtualAllocEx(
        process: Handle,
        addr: *mut c_void,
        size: usize,
        alloc_type: u32,
        protect: u32,
    ) -> *mut c_void;
    pub fn VirtualFreeEx(process: Handle, addr: *mut c_void, size: usize, free_type: u32) -> Bool;
    pub fn WriteProcessMemory(
        process: Handle,
        addr: *mut c_void,
        buf: *const c_void,
        size: usize,
        written: *mut usize,
    ) -> Bool;
    pub fn FlushInstructionCache(process: Handle, addr: *const c_void, size: usize) -> Bool;
    pub fn LoadLibraryA(name: *const u8) -> Handle;
    pub fn LoadLibraryW(name: *const u16) -> Handle;
    pub fn GetModuleHandleW(name: *const u16) -> Handle;
    pub fn GetModuleHandleExW(flags: u32, name: *const u16, module: *mut Handle) -> Bool;
    pub fn GetModuleFileNameW(module: Handle, buf: *mut u16, size: u32) -> u32;
    pub fn GetProcAddress(module: Handle, name: *const u8) -> *mut c_void;
    pub fn GetLastError() -> u32;
    pub fn SetLastError(code: u32);
    pub fn GetCurrentProcess() -> Handle;
    pub fn GetCurrentProcessId() -> u32;
    pub fn GetCurrentThreadId() -> u32;
    pub fn GetTickCount() -> u32;
    pub fn Sleep(ms: u32);
    pub fn CloseHandle(h: Handle) -> Bool;
    pub fn WaitForSingleObject(h: Handle, ms: u32) -> u32;
    pub fn CreateThread(
        attrs: *mut c_void,
        stack: usize,
        start: ThreadStart,
        param: *mut c_void,
        flags: u32,
        thread_id: *mut u32,
    ) -> Handle;
    pub fn CreateRemoteThread(
        process: Handle,
        attrs: *mut c_void,
        stack: usize,
        start: *mut c_void,
        param: *mut c_void,
        flags: u32,
        thread_id: *mut u32,
    ) -> Handle;
    pub fn GetExitCodeThread(thread: Handle, code: *mut u32) -> Bool;
    pub fn GetExitCodeProcess(process: Handle, code: *mut u32) -> Bool;
    pub fn OpenProcess(access: u32, inherit: Bool, pid: u32) -> Handle;
    pub fn OpenThread(access: u32, inherit: Bool, tid: u32) -> Handle;
    pub fn SuspendThread(thread: Handle) -> u32;
    pub fn ResumeThread(thread: Handle) -> u32;
    pub fn GetThreadContext(thread: Handle, ctx: *mut Context) -> Bool;
    pub fn SetThreadContext(thread: Handle, ctx: *const Context) -> Bool;
    pub fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> Handle;
    pub fn Thread32First(snapshot: Handle, entry: *mut ThreadEntry32) -> Bool;
    pub fn Thread32Next(snapshot: Handle, entry: *mut ThreadEntry32) -> Bool;
    pub fn TlsAlloc() -> u32;
    pub fn TlsGetValue(index: u32) -> *mut c_void;
    pub fn TlsSetValue(index: u32, value: *mut c_void) -> Bool;
    pub fn CreateProcessW(
        app: *const u16,
        cmd: *mut u16,
        process_attrs: *mut c_void,
        thread_attrs: *mut c_void,
        inherit: Bool,
        flags: u32,
        env: *mut c_void,
        cwd: *const u16,
        startup: *const StartupInfoW,
        info: *mut ProcessInformation,
    ) -> Bool;
    pub fn TerminateProcess(process: Handle, code: u32) -> Bool;
    pub fn SetUnhandledExceptionFilter(filter: Option<TopLevelFilter>) -> Option<TopLevelFilter>;
    pub fn GetStdHandle(which: u32) -> Handle;
    pub fn WriteFile(
        file: Handle,
        buf: *const c_void,
        len: u32,
        written: *mut u32,
        overlapped: *mut c_void,
    ) -> Bool;
    pub fn GetFullPathNameW(
        name: *const u16,
        len: u32,
        buf: *mut u16,
        file_part: *mut *mut u16,
    ) -> u32;
    pub fn FormatMessageW(
        flags: u32,
        source: *const c_void,
        message_id: u32,
        language_id: u32,
        buf: *mut u16,
        size: u32,
        args: *const c_void,
    ) -> u32;
}

/// A NUL-terminated UTF-16 copy of `s`.
pub fn wide(s: &std::ffi::OsStr) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    s.encode_wide().chain(std::iter::once(0)).collect()
}

/// The system's text for a Windows error code (without trailing newline).
pub fn error_text(code: u32) -> String {
    let mut buf = [0u16; 512];
    // SAFETY: the buffer and its length are passed together.
    let n = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            std::ptr::null(),
            code,
            0,
            buf.as_mut_ptr(),
            buf.len() as u32,
            std::ptr::null(),
        )
    };
    if n == 0 {
        return format!("Windows error {code}");
    }
    String::from_utf16_lossy(&buf[..n as usize])
        .trim_end()
        .to_string()
}

/// Whether every byte of `[addr, addr + len)` is committed and readable.
///
/// This is how the probe checks a pointer before dereferencing it, so a bad
/// pointer in the game becomes a logged error instead of a crash.
pub fn is_readable(addr: u32, len: usize) -> bool {
    const READABLE: u32 = PAGE_READONLY
        | PAGE_READWRITE
        | PAGE_WRITECOPY
        | PAGE_EXECUTE_READ
        | PAGE_EXECUTE_READWRITE
        | PAGE_EXECUTE_WRITECOPY;
    if len == 0 {
        return true;
    }
    let end = u64::from(addr) + len as u64;
    if end > 0x1_0000_0000 {
        return false;
    }
    let mut cur = u64::from(addr);
    while cur < end {
        let mut info = MemoryBasicInformation::zeroed();
        // SAFETY: `info` is a valid, correctly sized output buffer.
        let got = unsafe {
            VirtualQuery(
                cur as usize as *const c_void,
                &mut info,
                size_of::<MemoryBasicInformation>(),
            )
        };
        if got == 0
            || info.state != MEM_COMMIT
            || info.protect & PAGE_GUARD != 0
            || info.protect & READABLE == 0
        {
            return false;
        }
        let region_end = info.base_address as usize as u64 + info.region_size as u64;
        if region_end <= cur {
            return false;
        }
        cur = region_end;
    }
    true
}
