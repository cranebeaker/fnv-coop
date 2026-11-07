//! The Windows calls the launcher makes, by hand as the other tools here do (no dependencies).
//! Reading only, apart from the launcher's own windows: nothing here writes the registry or a
//! system setting.
use std::ffi::{c_void, OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

#[link(name = "kernel32")]
extern "system" {
    fn GetACP() -> u32;
    fn GetShortPathNameW(long: *const u16, short: *mut u16, size: u32) -> u32;
    fn GetLocalTime(time: *mut [u16; 8]);
    fn CreateMutexW(security: *const c_void, owner: i32, name: *const u16) -> *mut c_void;
    fn GetLastError() -> u32;
}
#[link(name = "advapi32")]
extern "system" {
    fn RegOpenKeyExW(
        key: isize,
        sub: *const u16,
        options: u32,
        access: u32,
        out: *mut isize,
    ) -> i32;
    fn RegQueryValueExW(
        key: isize,
        name: *const u16,
        reserved: *mut u32,
        kind: *mut u32,
        data: *mut u8,
        size: *mut u32,
    ) -> i32;
    fn RegEnumKeyExW(
        key: isize,
        index: u32,
        name: *mut u16,
        size: *mut u32,
        reserved: *mut u32,
        class: *mut u16,
        class_size: *mut u32,
        time: *mut u64,
    ) -> i32;
    fn RegCloseKey(key: isize) -> i32;
}
#[link(name = "shell32")]
extern "system" {
    fn SHGetFolderPathW(
        owner: *mut c_void,
        folder: i32,
        token: *mut c_void,
        flags: u32,
        path: *mut u16,
    ) -> i32;
}
#[link(name = "version")]
extern "system" {
    fn GetFileVersionInfoSizeW(name: *const u16, handle: *mut u32) -> u32;
    fn GetFileVersionInfoW(name: *const u16, handle: u32, size: u32, data: *mut u8) -> i32;
    fn VerQueryValueW(
        block: *const u8,
        sub: *const u16,
        out: *mut *const u8,
        size: *mut u32,
    ) -> i32;
}
#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(reserved: *mut c_void, flags: u32) -> i32;
    fn CoCreateInstance(
        class: *const [u8; 16],
        outer: *mut c_void,
        context: u32,
        iid: *const [u8; 16],
        out: *mut *mut Obj,
    ) -> i32;
    fn CoTaskMemFree(memory: *mut c_void);
}

/// A zero-terminated wide string.
pub fn wide(text: &OsStr) -> Vec<u16> {
    text.encode_wide().chain(std::iter::once(0)).collect()
}

pub fn wide_str(text: &str) -> Vec<u16> {
    wide(OsStr::new(text))
}

/// The wide string up to its zero.
pub fn from_wide(text: &[u16]) -> OsString {
    let end = text.iter().position(|&c| c == 0).unwrap_or(text.len());
    OsString::from_wide(&text[..end])
}

/// The system's ANSI code page.
pub fn ansi_code_page() -> u32 {
    // SAFETY: no arguments.
    unsafe { GetACP() }
}

/// The 8.3 form of an existing path, when Windows has one.
pub fn short_path(path: &Path) -> Option<PathBuf> {
    let long = wide(path.as_os_str());
    let mut buffer = vec![0u16; 1024];
    // SAFETY: both strings live through the call; the size is the buffer's.
    let n = unsafe { GetShortPathNameW(long.as_ptr(), buffer.as_mut_ptr(), buffer.len() as u32) };
    (n > 0 && (n as usize) < buffer.len()).then(|| PathBuf::from(from_wide(&buffer)))
}

/// `yyyy-mm-dd hh:mm:ss` and `yyyymmdd-hhmmss`, local time.
pub fn now() -> (String, String) {
    let mut t = [0u16; 8];
    // SAFETY: SYSTEMTIME is eight 16-bit words.
    unsafe { GetLocalTime(&mut t) };
    (
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            t[0], t[1], t[3], t[4], t[5], t[6]
        ),
        format!(
            "{:04}{:02}{:02}-{:02}{:02}{:02}",
            t[0], t[1], t[3], t[4], t[5], t[6]
        ),
    )
}

/// True when another copy of the launcher already runs. The mutex lives as long as this process.
pub fn another_is_running() -> bool {
    let name = wide_str("Local\\nv-rs-preview-launcher");
    // SAFETY: the name lives through the call; the handle is kept open on purpose.
    unsafe {
        let handle = CreateMutexW(std::ptr::null(), 0, name.as_ptr());
        !handle.is_null() && GetLastError() == 183
    }
}

/// A folder Windows knows: `CSIDL_PERSONAL` 5 (the real Documents, wherever OneDrive moved it),
/// `CSIDL_APPDATA` 0x1a, `CSIDL_LOCAL_APPDATA` 0x1c.
pub fn known_folder(csidl: i32) -> Option<PathBuf> {
    let mut buffer = vec![0u16; 1024];
    // SAFETY: the buffer is longer than MAX_PATH, as the call asks.
    let hr = unsafe {
        SHGetFolderPathW(
            std::ptr::null_mut(),
            csidl,
            std::ptr::null_mut(),
            0,
            buffer.as_mut_ptr(),
        )
    };
    (hr == 0).then(|| PathBuf::from(from_wide(&buffer)))
}

pub const HKLM: isize = 0x8000_0002u32 as i32 as isize;
pub const HKCU: isize = 0x8000_0001u32 as i32 as isize;
const KEY_READ: u32 = 0x2_0019;
const KEY_WOW64_32KEY: u32 = 0x200;

/// A registry key open for reading.
struct Key(isize);

impl Key {
    fn open(root: isize, path: &str) -> Option<Key> {
        let sub = wide_str(path);
        let mut key = 0isize;
        // SAFETY: the name lives through the call; `key` receives the handle.
        let r =
            unsafe { RegOpenKeyExW(root, sub.as_ptr(), 0, KEY_READ | KEY_WOW64_32KEY, &mut key) };
        (r == 0).then_some(Key(key))
    }

    fn string(&self, name: &str) -> Option<OsString> {
        let name = wide_str(name);
        let mut size = 0u32;
        let mut kind = 0u32;
        // SAFETY: a size query, then a read into a buffer of that size.
        unsafe {
            if RegQueryValueExW(
                self.0,
                name.as_ptr(),
                std::ptr::null_mut(),
                &mut kind,
                std::ptr::null_mut(),
                &mut size,
            ) != 0
            {
                return None;
            }
            // REG_SZ, REG_EXPAND_SZ.
            if kind != 1 && kind != 2 {
                return None;
            }
            let mut data = vec![0u16; size as usize / 2 + 1];
            if RegQueryValueExW(
                self.0,
                name.as_ptr(),
                std::ptr::null_mut(),
                &mut kind,
                data.as_mut_ptr().cast(),
                &mut size,
            ) != 0
            {
                return None;
            }
            Some(from_wide(&data))
        }
    }

    fn word(&self, name: &str) -> Option<u32> {
        let name = wide_str(name);
        let mut data = 0u32;
        let mut size = 4u32;
        let mut kind = 0u32;
        // SAFETY: a 4-byte buffer and its size.
        let r = unsafe {
            RegQueryValueExW(
                self.0,
                name.as_ptr(),
                std::ptr::null_mut(),
                &mut kind,
                (&mut data as *mut u32).cast(),
                &mut size,
            )
        };
        (r == 0 && kind == 4).then_some(data)
    }

    fn sub_keys(&self) -> Vec<String> {
        let mut names = Vec::new();
        for index in 0.. {
            let mut name = [0u16; 256];
            let mut size = 256u32;
            // SAFETY: the buffer and its size; the optional arguments are null.
            let r = unsafe {
                RegEnumKeyExW(
                    self.0,
                    index,
                    name.as_mut_ptr(),
                    &mut size,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            };
            if r != 0 {
                break;
            }
            names.push(from_wide(&name).to_string_lossy().into_owned());
        }
        names
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        // SAFETY: the handle is ours.
        unsafe { RegCloseKey(self.0) };
    }
}

/// A string value in the registry (32-bit view, where installers write).
pub fn registry_string(root: isize, path: &str, name: &str) -> Option<OsString> {
    Key::open(root, path)?.string(name)
}

/// The value `value` of every key under `path` whose value `title` contains `wanted`.
pub fn registry_scan(
    root: isize,
    path: &str,
    title: &str,
    wanted: &str,
    value: &str,
) -> Vec<OsString> {
    let Some(parent) = Key::open(root, path) else {
        return vec![];
    };
    parent
        .sub_keys()
        .iter()
        .filter_map(|sub| {
            let key = Key::open(root, &format!("{path}\\{sub}"))?;
            let name = key.string(title)?;
            name.to_string_lossy()
                .to_ascii_lowercase()
                .contains(wanted)
                .then(|| key.string(value))
                .flatten()
        })
        .collect()
}

/// Windows' build, with its update number: `26200.7171`.
pub fn windows_build() -> String {
    let Some(key) = Key::open(HKLM, "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") else {
        return "unknown".into();
    };
    let build = key
        .string("CurrentBuildNumber")
        .map_or_else(|| "unknown".into(), |b| b.to_string_lossy().into_owned());
    format!("{build}.{}", key.word("UBR").unwrap_or(0))
}

/// A file's version from its version resource: `1.4.0.525`.
pub fn file_version(path: &Path) -> Option<String> {
    let name = wide(path.as_os_str());
    let root = wide_str("\\");
    let mut handle = 0u32;
    // SAFETY: each call gets the name, and a buffer of the size the first call gave.
    unsafe {
        let size = GetFileVersionInfoSizeW(name.as_ptr(), &mut handle);
        if size == 0 {
            return None;
        }
        let mut data = vec![0u8; size as usize];
        if GetFileVersionInfoW(name.as_ptr(), 0, size, data.as_mut_ptr()) == 0 {
            return None;
        }
        let mut out = std::ptr::null();
        let mut len = 0u32;
        if VerQueryValueW(data.as_ptr(), root.as_ptr(), &mut out, &mut len) == 0 || len < 52 {
            return None;
        }
        let info = std::slice::from_raw_parts(out, 52);
        let word = |o: usize| u32::from_le_bytes([info[o], info[o + 1], info[o + 2], info[o + 3]]);
        let (ms, ls) = (word(8), word(12));
        Some(format!(
            "{}.{}.{}.{}",
            ms >> 16,
            ms & 0xffff,
            ls >> 16,
            ls & 0xffff
        ))
    }
}

/// A COM object: a pointer to its table of functions.
#[repr(C)]
struct Obj {
    vtable: *const usize,
}

/// The default sound output device's id, as Windows names it; `None` when there is no device.
/// Only read (`IMMDeviceEnumerator::GetDefaultAudioEndpoint`, `IMMDevice::GetId`).
pub fn default_sound_device() -> Option<String> {
    const CLSID_ENUMERATOR: [u8; 16] = [
        0x95, 0x03, 0xde, 0xbc, 0x2f, 0xe5, 0x7c, 0x46, 0x8e, 0x3d, 0xc4, 0x57, 0x92, 0x91, 0x69,
        0x2e,
    ];
    const IID_ENUMERATOR: [u8; 16] = [
        0xd2, 0x64, 0x56, 0xa9, 0x14, 0x96, 0x35, 0x4f, 0xa7, 0x46, 0xde, 0x8d, 0xb6, 0x36, 0x17,
        0xe6,
    ];
    type GetDefault = unsafe extern "system" fn(*mut Obj, u32, u32, *mut *mut Obj) -> i32;
    type GetId = unsafe extern "system" fn(*mut Obj, *mut *mut u16) -> i32;
    type Release = unsafe extern "system" fn(*mut Obj) -> u32;
    // SAFETY: COM calls by their documented slots (IUnknown 0..2; the enumerator's
    // GetDefaultAudioEndpoint is slot 4, the device's GetId slot 5); each object is released.
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), 0);
        let mut enumerator: *mut Obj = std::ptr::null_mut();
        let hr = CoCreateInstance(
            &CLSID_ENUMERATOR,
            std::ptr::null_mut(),
            1,
            &IID_ENUMERATOR,
            &mut enumerator,
        );
        if hr < 0 || enumerator.is_null() {
            return None;
        }
        let mut device: *mut Obj = std::ptr::null_mut();
        let get: GetDefault = std::mem::transmute(*(*enumerator).vtable.add(4));
        let hr = get(enumerator, 0, 0, &mut device);
        let release: Release = std::mem::transmute(*(*enumerator).vtable.add(2));
        release(enumerator);
        if hr < 0 || device.is_null() {
            return None;
        }
        let mut id: *mut u16 = std::ptr::null_mut();
        let get: GetId = std::mem::transmute(*(*device).vtable.add(5));
        let hr = get(device, &mut id);
        let mut text = None;
        if hr >= 0 && !id.is_null() {
            let mut n = 0;
            while *id.add(n) != 0 {
                n += 1;
            }
            text = Some(String::from_utf16_lossy(std::slice::from_raw_parts(id, n)));
            CoTaskMemFree(id.cast());
        }
        let release: Release = std::mem::transmute(*(*device).vtable.add(2));
        release(device);
        text
    }
}
