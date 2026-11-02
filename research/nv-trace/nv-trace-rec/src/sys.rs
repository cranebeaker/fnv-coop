//! Windows declarations the recorder needs beyond `nv-win`.

#![allow(non_snake_case, clippy::upper_case_acronyms)]

use std::ffi::c_void;

pub type Handle = *mut c_void;

#[repr(C)]
pub struct UnicodeString {
    pub length: u16,
    pub max_length: u16,
    pub buffer: *const u16,
}

#[repr(C)]
pub struct ObjectAttributes {
    pub length: u32,
    pub root_directory: Handle,
    pub object_name: *const UnicodeString,
    pub attributes: u32,
    pub security_descriptor: *mut c_void,
    pub security_qos: *mut c_void,
}

pub const STATUS_ACCESS_DENIED: i32 = 0xC000_0022u32 as i32;

#[link(name = "kernel32")]
extern "system" {
    pub fn GetFinalPathNameByHandleW(file: Handle, buf: *mut u16, len: u32, flags: u32) -> u32;
    pub fn ExitProcess(code: u32) -> !;
    pub fn GetFileAttributesW(name: *const u16) -> u32;
    pub fn OpenEventW(access: u32, inherit: i32, name: *const u16) -> Handle;
}

#[link(name = "user32")]
extern "system" {
    pub fn GetForegroundWindow() -> Handle;
    pub fn GetWindowThreadProcessId(window: Handle, pid: *mut u32) -> u32;
    pub fn EnumWindows(each: unsafe extern "system" fn(Handle, isize) -> i32, param: isize) -> i32;
    pub fn IsWindowVisible(window: Handle) -> i32;
    pub fn ShowWindow(window: Handle, cmd: i32) -> i32;
    pub fn BringWindowToTop(window: Handle) -> i32;
    pub fn SetForegroundWindow(window: Handle) -> i32;
}

/// `NtClose`, looked up in ntdll (not linked: the import library isn't always present).
pub fn nt_close(h: Handle) {
    type NtClose = unsafe extern "system" fn(Handle) -> i32;
    static PTR: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    let f = *PTR.get_or_init(|| proc_address("ntdll.dll", "NtClose"));
    if f != 0 {
        // SAFETY: ntdll's NtClose, called with a handle this process owns.
        unsafe { std::mem::transmute::<usize, NtClose>(f)(h) };
    }
}

/// The address of an export of an already loaded module, or 0.
pub fn proc_address(module: &str, name: &str) -> usize {
    let wide: Vec<u16> = module.encode_utf16().chain(std::iter::once(0)).collect();
    let cname: Vec<u8> = name.bytes().chain(std::iter::once(0)).collect();
    // SAFETY: plain lookups with NUL-terminated names.
    unsafe {
        let m = nv_win::GetModuleHandleW(wide.as_ptr());
        if m.is_null() {
            return 0;
        }
        nv_win::GetProcAddress(m, cname.as_ptr()) as usize
    }
}

/// The path of an open handle (`\\?\C:\...`), or `None`.
pub fn final_path(h: Handle) -> Option<String> {
    let mut buf = vec![0u16; 1024];
    // SAFETY: the buffer and its length go together.
    let n = unsafe { GetFinalPathNameByHandleW(h, buf.as_mut_ptr(), buf.len() as u32, 0) };
    if n == 0 || n as usize >= buf.len() {
        return None;
    }
    Some(String::from_utf16_lossy(&buf[..n as usize]))
}

/// A `UNICODE_STRING`'s text.
///
/// # Safety
/// `s` is null or points at a valid `UNICODE_STRING`.
pub unsafe fn unicode(s: *const UnicodeString) -> Option<String> {
    let s = s.as_ref()?;
    if s.buffer.is_null() {
        return Some(String::new());
    }
    let len = s.length as usize / 2;
    Some(String::from_utf16_lossy(std::slice::from_raw_parts(
        s.buffer, len,
    )))
}
