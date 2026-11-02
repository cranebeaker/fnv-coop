//! The player-folder guard: `NtCreateFile`, `NtOpenFile` and `NtSetInformationFile` in
//! ntdll are detoured, so every file open, creation, rename, hard link and delete of the
//! process (the game, its DLLs, the drivers' user-mode parts, this DLL) passes the check in
//! `nv_trace_core::guard` first. An open that could change something inside a protected folder
//! (or a rename or link whose target lies inside one) is refused with `STATUS_ACCESS_DENIED`
//! before it reaches the file system. Opens that could change something and do succeed are
//! checked again by the handle's final path (`GetFinalPathNameByHandleW`), which catches a
//! protected file reached through another name (a short name, a link); such a handle is
//! closed and the open refused too. Every refusal is written to the log ("guard: refused").
//!
//! Two answers keep the game behaving as in a normal run without touching anything:
//!
//! - an open-only or a delete of a file that doesn't exist is answered "not found" (what
//!   Windows answers), without opening anything ("guard: absent"; the game deletes
//!   `warnings.txt`, `programflow.txt` and `programmeroutput.txt` in its working folder at
//!   start-up);
//! - a new file (or folder) the game makes in the overlaid folder (its working folder, the
//!   game folder) is made in the run folder's overlay under the same relative name instead,
//!   and later opens of that name, reads included, go there ("guard: redirected").
//!
//! Changing an existing file of a protected folder is always refused.
//!
//! The guard is installed in `DllMain`, before the game runs an instruction. If it cannot be
//! installed the process is ended there (fail closed).

use crate::sys::{self, Handle, ObjectAttributes, STATUS_ACCESS_DENIED};
use nv_trace_core::guard::{self, Protected};
use nv_win as win;
use std::collections::HashSet;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering::*};
use std::sync::{Mutex, OnceLock};

static PROTECTED: OnceLock<Protected> = OnceLock::new();
static ORIG_CREATE: AtomicUsize = AtomicUsize::new(0);
static ORIG_OPEN: AtomicUsize = AtomicUsize::new(0);
static ORIG_SETINFO: AtomicUsize = AtomicUsize::new(0);
static DEPTH_TLS: AtomicU32 = AtomicU32::new(u32::MAX);
pub static REFUSED: AtomicU32 = AtomicU32::new(0);

type NtCreateFile = unsafe extern "system" fn(
    *mut Handle,
    u32,
    *const ObjectAttributes,
    *mut c_void,
    *mut c_void,
    u32,
    u32,
    u32,
    u32,
    *mut c_void,
    u32,
) -> i32;
type NtOpenFile = unsafe extern "system" fn(
    *mut Handle,
    u32,
    *const ObjectAttributes,
    *mut c_void,
    u32,
    u32,
) -> i32;
type NtSetInformationFile =
    unsafe extern "system" fn(Handle, *mut c_void, *mut c_void, u32, u32) -> i32;

/// Inside the guard on this thread already (its own lookups open files too): pass through.
struct Depth;

impl Depth {
    fn enter() -> Option<Depth> {
        let idx = DEPTH_TLS.load(Acquire);
        if idx == u32::MAX {
            return None;
        }
        // SAFETY: this thread's own slot.
        unsafe {
            if !win::TlsGetValue(idx).is_null() {
                return None;
            }
            win::TlsSetValue(idx, 1 as *mut c_void);
        }
        Some(Depth)
    }
}

impl Drop for Depth {
    fn drop(&mut self) {
        // SAFETY: this thread's own slot.
        unsafe { win::TlsSetValue(DEPTH_TLS.load(Acquire), std::ptr::null_mut()) };
    }
}

/// The path an `OBJECT_ATTRIBUTES` names, normalised; `None` when it can't be told (a root
/// handle whose path can't be read).
unsafe fn target(oa: *const ObjectAttributes) -> Option<String> {
    let oa = oa.as_ref()?;
    let name = sys::unicode(oa.object_name)?;
    if oa.root_directory.is_null() {
        Some(guard::normalize(&name))
    } else {
        let root = sys::final_path(oa.root_directory)?;
        Some(guard::join(&root, &name))
    }
}

/// Whether a changing open of `path` must be refused. A path that is not a drive or UNC
/// path after normalisation (an NT device path such as `\Device\HarddiskVolume3\...`) can't
/// be compared with the protected folders and is refused too.
fn refuse_path(path: Option<&str>) -> bool {
    let Some(p) = path else { return true };
    let comparable = p.as_bytes().get(1) == Some(&b':') || p.starts_with("\\\\");
    let device_file = p.starts_with("\\device\\harddisk") || p.starts_with("device\\harddisk");
    PROTECTED.get().is_some_and(|g| g.refuses(p)) || (device_file && !comparable)
}

fn refuse_final(h: Handle) -> bool {
    match sys::final_path(h) {
        Some(p) => PROTECTED
            .get()
            .is_some_and(|g| g.contains(&guard::normalize(&p))),
        None => false,
    }
}

/// The game folder's overlay: new files the game makes in its working folder (the game
/// folder, protected) are made under `dir` with the same relative name instead, and later
/// opens of those names are served from there. `made` holds the relative names (normalised)
/// created there so far.
struct Overlay {
    root: String,
    dir: String,
    made: Mutex<HashSet<String>>,
}

static OVERLAY: OnceLock<Overlay> = OnceLock::new();

/// Sets the overlay: `root` (the game folder) is redirected into `dir`.
pub fn set_overlay(root: &str, dir: &str) {
    let _ = OVERLAY.set(Overlay {
        root: guard::normalize(root),
        dir: dir.trim_end_matches('\\').to_string(),
        made: Mutex::new(HashSet::new()),
    });
}

/// The relative name of `path` under the overlaid folder.
fn overlay_rel(path: &str) -> Option<(&'static Overlay, String)> {
    let o = OVERLAY.get()?;
    let rest = path.strip_prefix(o.root.as_str())?.strip_prefix('\\')?;
    (!rest.is_empty()).then(|| (o, rest.to_string()))
}

fn overlay_made(o: &Overlay, rel: &str) -> bool {
    o.made.lock().map(|m| m.contains(rel)).unwrap_or(false)
}

/// Whether anything was made in the overlay yet (reads are only looked at after that).
fn overlay_in_use() -> bool {
    OVERLAY
        .get()
        .is_some_and(|o| o.made.lock().map(|m| !m.is_empty()).unwrap_or(false))
}

enum Decision {
    Pass,
    /// Answer this status without opening anything.
    Answer(i32),
    /// Open this Win32 path instead: (path, relative name, whether this makes the name).
    Redirect(String, String, bool),
    Refuse,
}

/// What to do with an open of `path`. `changes`: the open could change something;
/// `open_only`: it never creates (`NtOpenFile`, or `FILE_OPEN`).
fn decide(path: Option<&str>, changes: bool, open_only: bool) -> Decision {
    if !changes {
        // Reads: a name the game made in the overlay is read from there.
        if let Some((o, rel)) = path.and_then(overlay_rel) {
            if overlay_made(o, &rel) {
                return Decision::Redirect(format!("{}\\{rel}", o.dir), rel, false);
            }
        }
        return Decision::Pass;
    }
    if !refuse_path(path) {
        return Decision::Pass;
    }
    let Some(p) = path else {
        return Decision::Refuse;
    };
    if let Some((o, rel)) = overlay_rel(p) {
        if overlay_made(o, &rel) {
            return Decision::Redirect(format!("{}\\{rel}", o.dir), rel, false);
        }
        return match absent(p) {
            // An existing player file: never changed.
            None => Decision::Refuse,
            Some(status) if open_only => Decision::Answer(status),
            Some(_) => Decision::Redirect(format!("{}\\{rel}", o.dir), rel, true),
        };
    }
    match absent(p) {
        Some(status) if open_only => Decision::Answer(status),
        _ => Decision::Refuse,
    }
}

/// The buffers of a redirected `OBJECT_ATTRIBUTES`; they must live until the call is made.
struct Redirected {
    attrs: ObjectAttributes,
    _name: Box<sys::UnicodeString>,
    _wide: Vec<u16>,
}

/// An `OBJECT_ATTRIBUTES` naming the Win32 `path` with the flags of `oa`.
unsafe fn redirected(oa: *const ObjectAttributes, path: &str) -> Redirected {
    let wide: Vec<u16> = format!("\\??\\{path}").encode_utf16().collect();
    let name = Box::new(sys::UnicodeString {
        length: (wide.len() * 2) as u16,
        max_length: (wide.len() * 2) as u16,
        buffer: wide.as_ptr(),
    });
    let src = &*oa;
    let attrs = ObjectAttributes {
        length: src.length,
        root_directory: std::ptr::null_mut(),
        object_name: &*name,
        attributes: src.attributes,
        security_descriptor: src.security_descriptor,
        security_qos: src.security_qos,
    };
    Redirected {
        attrs,
        _name: name,
        _wide: wide,
    }
}

/// Applies a decision around the original call `call(oa)`.
unsafe fn apply(
    what: &str,
    oa: *const ObjectAttributes,
    path: Option<&str>,
    decision: Decision,
    call: impl FnOnce(*const ObjectAttributes) -> i32,
) -> i32 {
    match decision {
        Decision::Pass => call(oa),
        Decision::Answer(status) => {
            crate::log(&format!(
                "guard: absent {what} {} (answered {status:#x}, as Windows would)",
                path.unwrap_or("")
            ));
            status
        }
        Decision::Refuse => {
            crate::log(&format!(
                "guard: refused {what} {}",
                path.unwrap_or("(path unknown)")
            ));
            REFUSED.fetch_add(1, Relaxed);
            STATUS_ACCESS_DENIED
        }
        Decision::Redirect(to, rel, make) => {
            if make {
                if let Some(parent) = std::path::Path::new(&to).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
            }
            let r = redirected(oa, &to);
            let status = call(&r.attrs);
            if make && status >= 0 {
                if let Some(o) = OVERLAY.get() {
                    if let Ok(mut m) = o.made.lock() {
                        m.insert(rel);
                    }
                }
            }
            crate::log(&format!(
                "guard: redirected {what} {} -> {to} (status {status:#x})",
                path.unwrap_or("")
            ));
            status
        }
    }
}

/// The final-path check of a changing open that was let through: a protected file reached
/// under another name is closed again and refused.
unsafe fn check_final(what: &str, oa: *const ObjectAttributes, handle: *mut Handle) -> bool {
    if handle.is_null() || !refuse_final(*handle) {
        return false;
    }
    sys::nt_close(*handle);
    *handle = std::ptr::null_mut();
    crate::log(&format!(
        "guard: refused {what} after open (final path protected) {}",
        target(oa).as_deref().unwrap_or("?")
    ));
    REFUSED.fetch_add(1, Relaxed);
    true
}

unsafe extern "system" fn nt_create_file(
    handle: *mut Handle,
    access: u32,
    oa: *const ObjectAttributes,
    iosb: *mut c_void,
    size: *mut c_void,
    attrs: u32,
    share: u32,
    disposition: u32,
    options: u32,
    ea: *mut c_void,
    ea_len: u32,
) -> i32 {
    let orig: NtCreateFile = std::mem::transmute(ORIG_CREATE.load(Acquire));
    let call = |oa| {
        orig(
            handle,
            access,
            oa,
            iosb,
            size,
            attrs,
            share,
            disposition,
            options,
            ea,
            ea_len,
        )
    };
    let Some(_depth) = Depth::enter() else {
        return call(oa);
    };
    let changes = guard::writes(access, Some(disposition), options);
    if !changes && !overlay_in_use() {
        return call(oa);
    }
    let path = target(oa);
    let decision = decide(path.as_deref(), changes, disposition == guard::FILE_OPEN);
    let passed = matches!(decision, Decision::Pass);
    let what =
        format!("NtCreateFile access {access:#x} disposition {disposition} options {options:#x}");
    let status = apply(&what, oa, path.as_deref(), decision, call);
    if passed && changes && status >= 0 && check_final(&what, oa, handle) {
        return STATUS_ACCESS_DENIED;
    }
    status
}

unsafe extern "system" fn nt_open_file(
    handle: *mut Handle,
    access: u32,
    oa: *const ObjectAttributes,
    iosb: *mut c_void,
    share: u32,
    options: u32,
) -> i32 {
    let orig: NtOpenFile = std::mem::transmute(ORIG_OPEN.load(Acquire));
    let call = |oa| orig(handle, access, oa, iosb, share, options);
    let Some(_depth) = Depth::enter() else {
        return call(oa);
    };
    let changes = guard::writes(access, None, options);
    if !changes && !overlay_in_use() {
        return call(oa);
    }
    let path = target(oa);
    let decision = decide(path.as_deref(), changes, true);
    let passed = matches!(decision, Decision::Pass);
    let what = format!("NtOpenFile access {access:#x} options {options:#x}");
    let status = apply(&what, oa, path.as_deref(), decision, call);
    if passed && changes && status >= 0 && check_final(&what, oa, handle) {
        return STATUS_ACCESS_DENIED;
    }
    status
}

/// `FILE_RENAME_INFORMATION` / `FILE_LINK_INFORMATION` (x86): flags or `ReplaceIfExists` at
/// +0, root directory at +4, name length (bytes) at +8, the name from +12.
unsafe fn rename_target(info: *const u8, len: u32) -> Option<String> {
    if info.is_null() || len < 12 {
        return None;
    }
    let root = *(info.add(4) as *const Handle);
    let name_len = *(info.add(8) as *const u32) as usize;
    if 12 + name_len > len as usize {
        return None;
    }
    let name = String::from_utf16_lossy(std::slice::from_raw_parts(
        info.add(12) as *const u16,
        name_len / 2,
    ));
    if root.is_null() {
        Some(guard::normalize(&name))
    } else {
        Some(guard::join(&sys::final_path(root)?, &name))
    }
}

unsafe extern "system" fn nt_set_information_file(
    handle: Handle,
    iosb: *mut c_void,
    info: *mut c_void,
    len: u32,
    class: u32,
) -> i32 {
    let orig: NtSetInformationFile = std::mem::transmute(ORIG_SETINFO.load(Acquire));
    let depth = Depth::enter();
    if depth.is_some() && guard::names_target(class) {
        let to = rename_target(info as *const u8, len);
        // The source too: a handle opened before the guard, or one the final-path check let
        // through, must not move a protected file out either.
        let from_protected = refuse_final(handle);
        if refuse_path(to.as_deref()) || from_protected {
            crate::log(&format!(
                "guard: refused NtSetInformationFile class {class} to {} (source protected: {from_protected})",
                to.as_deref().unwrap_or("(path unknown)")
            ));
            REFUSED.fetch_add(1, Relaxed);
            return STATUS_ACCESS_DENIED;
        }
    }
    orig(handle, iosb, info, len, class)
}

/// Installs the three detours. Any failure is an error (the caller ends the process).
pub fn install(protected: Protected, region: &mut crate::stubs::Region) -> Result<(), String> {
    if protected.roots().is_empty() {
        return Err("no protected folders configured".into());
    }
    let _ = PROTECTED.set(protected);
    // SAFETY: plain TLS allocation.
    let idx = unsafe { win::TlsAlloc() };
    if idx == u32::MAX {
        return Err("cannot allocate the guard's TLS slot".into());
    }
    DEPTH_TLS.store(idx, Release);
    for (name, ours, orig) in [
        (
            "NtCreateFile",
            nt_create_file as *const () as usize,
            &ORIG_CREATE,
        ),
        ("NtOpenFile", nt_open_file as *const () as usize, &ORIG_OPEN),
        (
            "NtSetInformationFile",
            nt_set_information_file as *const () as usize,
            &ORIG_SETINFO,
        ),
    ] {
        let at = sys::proc_address("ntdll.dll", name);
        if at == 0 {
            return Err(format!("ntdll!{name} not found"));
        }
        crate::hooks::detour(at as u32, ours as u32, region, |tramp| {
            orig.store(tramp as usize, Release)
        })
        .map_err(|e| format!("ntdll!{name}: {e}"))?;
    }
    Ok(())
}

/// For a refused open of an existing-file-only kind: when the file isn't there, the answer
/// Windows itself gives (`STATUS_OBJECT_NAME_NOT_FOUND`, or `STATUS_OBJECT_PATH_NOT_FOUND`
/// for a missing folder), so the game sees what it would see with the folder untouched (the
/// game deletes `warnings.txt`, `programflow.txt` and `programmeroutput.txt` in its working
/// folder at start-up; none exists). Nothing is opened either way.
fn absent(path: &str) -> Option<i32> {
    let wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
    // SAFETY: a NUL-terminated path; the last error is put back.
    unsafe {
        let saved = win::GetLastError();
        let attrs = sys::GetFileAttributesW(wide.as_ptr());
        let err = win::GetLastError();
        win::SetLastError(saved);
        if attrs != u32::MAX {
            return None;
        }
        match err {
            2 => Some(0xC000_0034u32 as i32),
            3 => Some(0xC000_003Au32 as i32),
            _ => None,
        }
    }
}
