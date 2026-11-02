//! Import-table redirections in the game exe (only the exe's own imports: the system's and
//! the drivers' DLLs keep theirs):
//!
//! - `SHELL32!SHGetFolderPathA`: the Documents, local and roaming application data folders
//!   answer the scratch copies, so the game reads its INI files and `plugins.txt` from there
//!   and writes its INI changes, `RendererInfo.txt` and saves there. Other folders pass through.
//! - `steam_api.dll`: every import answers as when no Steam client runs (what the port's
//!   platform layer answers: `SteamAPI_Init` false, the accessors null), so the run does not
//!   involve the Steam client at all.
//! - `KERNEL32!ExitProcess`: drains the trace before the process ends.
//! - `DINPUT8!DirectInput8Create`: the scripted keys go into the game's keyboard
//!   ([`crate::dinput`]).

use crate::sys;
use nv_win as win;
use std::ffi::c_void;
use std::sync::OnceLock;

pub struct Folders {
    pub personal: String,
    pub local_appdata: String,
    pub appdata: String,
}

static FOLDERS: OnceLock<Folders> = OnceLock::new();
static ORIG_SHGFP: OnceLock<usize> = OnceLock::new();
/// Every import slot of the exe (the hooks can patch a listed one, `crate::hooks`).
pub static SLOTS: OnceLock<Vec<u32>> = OnceLock::new();

type ShGetFolderPathA =
    unsafe extern "system" fn(*mut c_void, i32, *mut c_void, u32, *mut u8) -> i32;

/// CSIDL values (the low byte; the high bits are flags such as `CSIDL_FLAG_CREATE`).
const CSIDL_APPDATA: i32 = 0x1a;
const CSIDL_LOCAL_APPDATA: i32 = 0x1c;
const CSIDL_PERSONAL: i32 = 0x05;

unsafe extern "system" fn sh_get_folder_path_a(
    hwnd: *mut c_void,
    csidl: i32,
    token: *mut c_void,
    flags: u32,
    path: *mut u8,
) -> i32 {
    if let Some(f) = FOLDERS.get() {
        let ours = match csidl & 0xff {
            CSIDL_PERSONAL => Some(&f.personal),
            CSIDL_LOCAL_APPDATA => Some(&f.local_appdata),
            CSIDL_APPDATA => Some(&f.appdata),
            _ => None,
        };
        if let Some(p) = ours {
            if !path.is_null() && p.len() < 260 {
                std::ptr::copy_nonoverlapping(p.as_ptr(), path, p.len());
                *path.add(p.len()) = 0;
                crate::log(&format!("folder: SHGetFolderPathA({csidl:#x}) -> {p}"));
                return 0;
            }
        }
    }
    match ORIG_SHGFP.get() {
        Some(&o) => {
            std::mem::transmute::<usize, ShGetFolderPathA>(o)(hwnd, csidl, token, flags, path)
        }
        None => 0x8000_4005u32 as i32,
    }
}

/// Every steam_api.dll import: cdecl, nothing the game reads but `EAX` (false or null).
extern "C" fn steam_none() -> u32 {
    0
}

extern "system" fn exit_process(code: u32) -> ! {
    crate::finish_run(code);
    // SAFETY: the real ExitProcess.
    unsafe { sys::ExitProcess(code) }
}

static ORIG_GPA: OnceLock<usize> = OnceLock::new();

type GetProcAddress = unsafe extern "system" fn(*mut c_void, *const u8) -> *mut c_void;

/// The exe's `GetProcAddress`: the real answer, except that `Direct3DCreate9` answers
/// [`crate::d3d::direct3d_create9`] (which calls the real one) while the list names device
/// methods still to hook. Names by ordinal and every other name pass through.
unsafe extern "system" fn get_proc_address(module: *mut c_void, name: *const u8) -> *mut c_void {
    let Some(&orig) = ORIG_GPA.get() else {
        return std::ptr::null_mut();
    };
    let f = std::mem::transmute::<usize, GetProcAddress>(orig)(module, name);
    if !f.is_null()
        && name as usize > 0xffff
        && crate::hooks::device_hooks_pending()
        && std::ffi::CStr::from_ptr(name as *const std::ffi::c_char).to_bytes()
            == b"Direct3DCreate9"
    {
        let last_error = win::GetLastError();
        crate::d3d::ORIG_CREATE9.store(f as usize, std::sync::atomic::Ordering::Release);
        crate::log("d3d: GetProcAddress(Direct3DCreate9) answers the recorder's wrapper");
        win::SetLastError(last_error);
        return crate::d3d::direct3d_create9 as *const () as *mut c_void;
    }
    f
}

/// Redirections for the exe's import table: (DLL, function, our function).
fn wanted(dll: &str, name: &str) -> Option<usize> {
    let dll = dll.to_ascii_lowercase();
    match (dll.as_str(), name) {
        ("shell32.dll", "SHGetFolderPathA") => Some(sh_get_folder_path_a as *const () as usize),
        ("kernel32.dll", "ExitProcess") => Some(exit_process as *const () as usize),
        ("steam_api.dll", _) => Some(steam_none as *const () as usize),
        ("dinput8.dll", "DirectInput8Create") => {
            Some(crate::dinput::direct_input8_create as *const () as usize)
        }
        ("d3d9.dll", "Direct3DCreate9") => Some(crate::d3d::direct3d_create9 as *const () as usize),
        // NiDX9Renderer (00e69410) finds Direct3DCreate9 with LoadLibraryA and GetProcAddress.
        ("kernel32.dll", "GetProcAddress") => Some(get_proc_address as *const () as usize),
        _ => None,
    }
}

unsafe fn cstr(p: *const u8) -> String {
    let mut n = 0;
    while *p.add(n) != 0 && n < 512 {
        n += 1;
    }
    String::from_utf8_lossy(std::slice::from_raw_parts(p, n)).into_owned()
}

/// Patches the exe's import address table. Returns one line per redirected slot.
pub fn install(folders: Folders) -> Result<Vec<String>, String> {
    let _ = FOLDERS.set(folders);
    // SAFETY: walking the loaded exe's headers and import directory, which the loader has
    // resolved (the process was started suspended and the loader ran for this DLL).
    unsafe {
        let base = win::GetModuleHandleW(std::ptr::null()) as usize;
        let rd = |off: usize| std::ptr::read_unaligned((base + off) as *const u32) as usize;
        if std::ptr::read_unaligned(base as *const u16) != 0x5a4d {
            return Err("exe has no MZ header".into());
        }
        let pe = rd(0x3c);
        // Optional header at pe + 24; the import directory is data directory 1 (PE32: +0x68).
        let imports = rd(pe + 24 + 0x68);
        if imports == 0 {
            return Err("exe has no import directory".into());
        }
        let mut lines = Vec::new();
        let mut all_slots = Vec::new();
        let mut desc = base + imports;
        loop {
            let names = std::ptr::read_unaligned(desc as *const u32) as usize;
            let dll_name = std::ptr::read_unaligned((desc + 12) as *const u32) as usize;
            let slots = std::ptr::read_unaligned((desc + 16) as *const u32) as usize;
            if dll_name == 0 {
                break;
            }
            let dll = cstr((base + dll_name) as *const u8);
            let lookup = if names != 0 { names } else { slots };
            let mut i = 0;
            loop {
                let entry = rd(lookup + 4 * i);
                if entry == 0 {
                    break;
                }
                all_slots.push((base + slots + 4 * i) as u32);
                if entry & 0x8000_0000 == 0 {
                    let name = cstr((base + entry + 2) as *const u8);
                    if let Some(ours) = wanted(&dll, &name) {
                        let slot = (base + slots + 4 * i) as *mut usize;
                        if name == "SHGetFolderPathA" {
                            let _ = ORIG_SHGFP.set(*slot);
                        }
                        if name == "GetProcAddress" {
                            let _ = ORIG_GPA.set(*slot);
                        }
                        if name == "Direct3DCreate9" {
                            crate::d3d::ORIG_CREATE9
                                .store(*slot, std::sync::atomic::Ordering::Release);
                        }
                        if name == "DirectInput8Create" {
                            crate::dinput::ORIG_CREATE
                                .store(*slot, std::sync::atomic::Ordering::Release);
                        }
                        let mut old = 0u32;
                        if win::VirtualProtect(
                            slot as *mut c_void,
                            4,
                            win::PAGE_READWRITE,
                            &mut old,
                        ) == 0
                        {
                            return Err(format!("cannot unprotect the slot of {dll}!{name}"));
                        }
                        *slot = ours;
                        let mut ignored = 0;
                        win::VirtualProtect(slot as *mut c_void, 4, old, &mut ignored);
                        lines.push(format!("iat={dll}!{name}"));
                    }
                }
                i += 1;
            }
            desc += 20;
        }
        let _ = SLOTS.set(all_slots);
        if ORIG_SHGFP.get().is_none() {
            return Err(
                "the exe does not import SHGetFolderPathA: its folders can't be redirected".into(),
            );
        }
        Ok(lines)
    }
}
