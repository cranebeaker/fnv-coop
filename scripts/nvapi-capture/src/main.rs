//! nvapi-capture: what the player's `nvapi.dll` answers to the calls FalloutNV.exe makes on its
//! renderer start-up, as lines of the platform's machine table (`d3d9_adapter.tsv`, see
//! `scripts/d3d9-capture`), written to stdout. 32-bit, as the game (a 64-bit process cannot load
//! the 32-bit `nvapi.dll`).
//!
//! The game (`009fea2a`, its statically linked NvAPI start-up) loads `nvapi.dll`, finds
//! `nvapi_QueryInterface`, asks it for the function of id `0150e828` and calls it, then asks it
//! for every id of its (cell, id) table at `011a6228` (read here from the player's
//! FalloutNV.exe, read only). `Renderer::Init`'s multi-GPU probe (004da670) then calls, after the
//! Direct3D 9 device exists: `f951a4d1(0, struct 0x8c bytes, version 0x1008c)`, `9abdd40d(i,
//! &handle)` for i = 0.. while it answers 0 (at most 128), `48b3ea59(handles[64], &count)`,
//! `e5ac921f(handles[64], &count)` and `4b708b54(the IDirect3D9 object, struct 0x1c bytes,
//! version 0x1001c)`. This tool makes the same calls in the same order (an `IDirect3D9` and a
//! windowed HAL device first, as the game has them then) and writes each one's status and the
//! bytes it wrote. The answers describe the player's hardware and driver: the table stays on
//! the player's machine (git-ignored), never in the repository. Windows only:
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/nvapi-capture/Cargo.toml -- "<game folder>\FalloutNV.exe" >> %LOCALAPPDATA%\nv-rs\d3d9_adapter.tsv`
//!
//! Lines: `NvApiLoad<TAB><TAB>found (1/0)<TAB>GetLastError when not found`,
//! `NvApiProc<TAB><TAB>found (1/0)`, `NvApiQuery<TAB>id<TAB>found (1/0)`,
//! `NvApiCall<TAB>id words..<TAB>status<TAB>out bytes after a call with them zeroed<TAB>the same
//! after a call with them set to ff<TAB>that call's status` (bytes the call wrote read the same in
//! both; see [`twice`]).
//! A line `NvApiEnd` closes the capture.
use std::ffi::c_void;
use std::io::Write;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn GetLastError() -> u32;
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
}
#[link(name = "d3d9")]
extern "system" {
    fn Direct3DCreate9(sdk: u32) -> *mut *const usize;
}
#[link(name = "user32")]
extern "system" {
    fn RegisterClassA(class: *const WndClassA) -> u16;
    fn CreateWindowExA(
        ex_style: u32,
        class: *const u8,
        title: *const u8,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: *mut c_void,
        menu: *mut c_void,
        instance: *mut c_void,
        param: *mut c_void,
    ) -> *mut c_void;
    fn DestroyWindow(window: *mut c_void) -> i32;
    fn DefWindowProcA(window: *mut c_void, message: u32, w: usize, l: isize) -> isize;
}

#[repr(C)]
struct WndClassA {
    style: u32,
    procedure: unsafe extern "system" fn(*mut c_void, u32, usize, isize) -> isize,
    class_extra: i32,
    window_extra: i32,
    instance: *mut c_void,
    icon: *mut c_void,
    cursor: *mut c_void,
    background: *mut c_void,
    menu_name: *const u8,
    class_name: *const u8,
}

type QueryInterface = unsafe extern "C" fn(u32) -> *mut c_void;
type Call0 = unsafe extern "C" fn() -> i32;
type CallWordPtr = unsafe extern "C" fn(u32, *mut u8) -> i32;
type CallPtrPtr = unsafe extern "C" fn(*mut u8, *mut u8) -> i32;

/// The (cell, id) table's address in FalloutNV.exe 1.4.0.525 and the first id `009fea2a` asks.
const TABLE: u32 = 0x011a_6228;
const INITIALIZE: u32 = 0x0150_e828;
const DRIVER_VERSION: u32 = 0xf951_a4d1;
const DISPLAY_HANDLE: u32 = 0x9abd_d40d;
const LOGICAL_GPUS: u32 = 0x48b3_ea59;
const PHYSICAL_GPUS: u32 = 0xe5ac_921f;
const SLI_STATE: u32 = 0x4b70_8b54;

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

/// The ids of the exe's (cell, id) table, up to the zero id, read from the file's sections.
fn table_ids(exe: &[u8]) -> Vec<u32> {
    let pe = u32_at(exe, 0x3c) as usize;
    assert_eq!(&exe[pe..pe + 4], b"PE\0\0", "not a PE file");
    let sections = u16::from_le_bytes([exe[pe + 6], exe[pe + 7]]) as usize;
    let optional = u16::from_le_bytes([exe[pe + 20], exe[pe + 21]]) as usize;
    let image_base = u32_at(exe, pe + 24 + 28);
    let first = pe + 24 + optional;
    let file_offset = |va: u32| -> usize {
        let rva = va - image_base;
        for s in 0..sections {
            let h = first + 40 * s;
            let (address, raw_size, raw) = (u32_at(exe, h + 12), u32_at(exe, h + 16), u32_at(exe, h + 20));
            if rva >= address && rva < address + raw_size {
                return (raw + rva - address) as usize;
            }
        }
        panic!("{va:#x} is not in the file's sections");
    };
    let mut ids = Vec::new();
    let mut entry = TABLE;
    loop {
        let id = u32_at(exe, file_offset(entry + 4));
        if id == 0 {
            return ids;
        }
        ids.push(id);
        entry += 8;
    }
}

/// An `IDirect3D9` and a windowed HAL device on a window of its own, as the game has them when
/// its multi-GPU probe runs. Returns the `IDirect3D9`.
unsafe fn direct3d(o: &mut impl Write) -> *mut c_void {
    let instance = GetModuleHandleA(std::ptr::null());
    let name = b"nv-rs nvapi-capture\0";
    let class = WndClassA {
        style: 3,
        procedure: DefWindowProcA,
        class_extra: 0,
        window_extra: 0,
        instance,
        icon: std::ptr::null_mut(),
        cursor: std::ptr::null_mut(),
        background: std::ptr::null_mut(),
        menu_name: std::ptr::null(),
        class_name: name.as_ptr(),
    };
    RegisterClassA(&class);
    let window = CreateWindowExA(0, name.as_ptr(), name.as_ptr(), 0x1000_0000, 0, 0, 320, 240,
        std::ptr::null_mut(), std::ptr::null_mut(), instance, std::ptr::null_mut());
    let d3d = Direct3DCreate9(32);
    assert!(!d3d.is_null());
    // D3DPRESENT_PARAMETERS: window-sized, D3DFMT_UNKNOWN, D3DSWAPEFFECT_DISCARD, windowed.
    let mut present = [0u32; 14];
    present[6] = 1;
    present[7] = window as u32;
    present[8] = 1;
    type CreateDevice =
        unsafe extern "system" fn(*mut c_void, u32, u32, *mut c_void, u32, *mut u32, *mut *mut c_void) -> u32;
    let create: CreateDevice = std::mem::transmute(*(*d3d).add(16));
    let mut device: *mut c_void = std::ptr::null_mut();
    // D3DCREATE_HARDWARE_VERTEXPROCESSING.
    let hr = create(d3d.cast(), 0, 1, window, 0x40, present.as_mut_ptr(), &mut device);
    writeln!(o, "NvApiDevice\t\t{hr:x}").unwrap();
    let _ = DestroyWindow;
    d3d.cast()
}

fn main() {
    let exe_path = std::env::args().nth(1).expect("usage: nvapi-capture <FalloutNV.exe>");
    let exe = std::fs::read(&exe_path).expect("FalloutNV.exe readable");
    let ids = table_ids(&exe);
    let out = std::io::stdout();
    let mut o = out.lock();
    unsafe {
        let d3d = direct3d(&mut o);
        let module = LoadLibraryA(b"nvapi.dll\0".as_ptr());
        if module.is_null() {
            writeln!(o, "NvApiLoad\t\t0\t{:x}", GetLastError()).unwrap();
            writeln!(o, "NvApiEnd").unwrap();
            return;
        }
        writeln!(o, "NvApiLoad\t\t1\t0").unwrap();
        let query = GetProcAddress(module, b"nvapi_QueryInterface\0".as_ptr());
        writeln!(o, "NvApiProc\t\t{}", u32::from(!query.is_null())).unwrap();
        if query.is_null() {
            writeln!(o, "NvApiEnd").unwrap();
            return;
        }
        let query: QueryInterface = std::mem::transmute(query);
        let ask = |o: &mut std::io::StdoutLock, id: u32| -> *mut c_void {
            let f = query(id);
            writeln!(o, "NvApiQuery\t{id:x}\t{}", u32::from(!f.is_null())).unwrap();
            f
        };
        let init = ask(&mut o, INITIALIZE);
        if init.is_null() {
            writeln!(o, "NvApiEnd").unwrap();
            return;
        }
        let init: Call0 = std::mem::transmute(init);
        let status = init();
        writeln!(o, "NvApiCall\t{INITIALIZE:x}\t{:x}\t", status as u32).unwrap();
        if status < 0 {
            writeln!(o, "NvApiEnd").unwrap();
            return;
        }
        let mut functions = std::collections::HashMap::new();
        for &id in &ids {
            let f = ask(&mut o, id);
            if !f.is_null() {
                functions.insert(id, f);
            }
        }
        // The probe's calls, in its order; a missing function is the game's stub (-3), which
        // the platform answers itself, so it is skipped here.
        if let Some(&f) = functions.get(&DRIVER_VERSION) {
            let f: CallWordPtr = std::mem::transmute(f);
            twice(&mut o, &format!("{DRIVER_VERSION:x} 0 1008c"), 0x8c, Some(0x1008c), |b| f(0, b));
        }
        if let Some(&f) = functions.get(&DISPLAY_HANDLE) {
            let f: CallWordPtr = std::mem::transmute(f);
            for index in 0..0x80u32 {
                let status =
                    twice(&mut o, &format!("{DISPLAY_HANDLE:x} {index:x}"), 4, None, |b| f(index, b));
                if status != 0 {
                    break;
                }
            }
        }
        for id in [LOGICAL_GPUS, PHYSICAL_GPUS] {
            if let Some(&f) = functions.get(&id) {
                let f: CallPtrPtr = std::mem::transmute(f);
                // The handle array (64 words), then the count.
                twice(&mut o, &format!("{id:x}"), 0x104, None, |b| f(b, b.add(0x100)));
            }
        }
        if let Some(&f) = functions.get(&SLI_STATE) {
            type CallObjectPtr = unsafe extern "C" fn(*mut c_void, *mut u8) -> i32;
            let f: CallObjectPtr = std::mem::transmute(f);
            twice(&mut o, &format!("{SLI_STATE:x} direct3d 1001c"), 0x1c, Some(0x1001c), |b| f(d3d, b));
        }
        writeln!(o, "NvApiEnd").unwrap();
    }
}

/// Makes one call twice, its out bytes filled with 00 and then with ff (the first word set to
/// `version` when given, as the game sets a structure's version), and writes the line: the
/// status of the zero-filled call (the game zeroes its structures), the bytes after each call
/// and the second call's status. A byte the call wrote reads the same after both; one it left
/// alone reads 00 and then ff. Returns the first status.
unsafe fn twice(
    o: &mut impl Write,
    words: &str,
    size: usize,
    version: Option<u32>,
    call: impl Fn(*mut u8) -> i32,
) -> i32 {
    let mut after = Vec::new();
    let mut statuses = Vec::new();
    for fill in [0u8, 0xff] {
        let mut buffer = vec![fill; size];
        if let Some(v) = version {
            buffer[..4].copy_from_slice(&v.to_le_bytes());
        }
        statuses.push(call(buffer.as_mut_ptr()));
        after.push(buffer);
    }
    writeln!(
        o,
        "NvApiCall\t{words}\t{:x}\t{}\t{}\t{:x}",
        statuses[0] as u32,
        hex(&after[0]),
        hex(&after[1]),
        statuses[1] as u32
    )
    .unwrap();
    statuses[0]
}
