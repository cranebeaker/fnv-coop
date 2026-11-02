//! A stand-in program for testing the recorder without the game (original code written for
//! the test). Linked at 0x00400000 without relocation, so its functions have fixed
//! addresses.
//!
//! - `nv-trace-fixture list`: prints a function list for its three test functions.
//! - `nv-trace-fixture` (run by `nv-trace-launch --exe`): calls the functions, then checks
//!   the guard from inside the process: writes into the working directory (the launcher's
//!   `--game`, a protected folder) must fail, a write to the folder `SHGetFolderPathA` gives
//!   must work and land in the run folder. It prints what happened to `fixture.txt` in that
//!   folder, and ends with `ExitProcess`.

#![cfg_attr(not(all(windows, target_arch = "x86")), allow(dead_code))]

#[cfg(not(all(windows, target_arch = "x86")))]
fn main() {}

#[cfg(all(windows, target_arch = "x86"))]
std::arch::global_asm!(
    ".text",
    ".globl _fx_add",
    "_fx_add:",
    "    push ebp",
    "    mov ebp, esp",
    "    mov eax, [ebp + 8]",
    "    add eax, [ebp + 12]",
    "    pop ebp",
    "    ret",
    ".globl _fx_this",
    "_fx_this:",
    "    push ebp",
    "    mov ebp, esp",
    "    mov eax, [ecx]",
    "    add eax, [ebp + 8]",
    "    pop ebp",
    "    ret 4",
    ".globl _fx_twice",
    "_fx_twice:",
    "    push ebp",
    "    mov ebp, esp",
    "    fld dword ptr [ebp + 8]",
    "    fadd dword ptr [ebp + 8]",
    "    pop ebp",
    "    ret",
);

#[cfg(all(windows, target_arch = "x86"))]
extern "C" {
    fn fx_add(a: u32, b: u32) -> u32;
    fn fx_twice(x: f32) -> f32;
}

#[cfg(all(windows, target_arch = "x86"))]
extern "thiscall" {
    fn fx_this(this: *const u32, a: u32) -> u32;
}

#[cfg(all(windows, target_arch = "x86"))]
#[link(name = "shell32")]
extern "system" {
    fn SHGetFolderPathA(hwnd: usize, csidl: i32, token: usize, flags: u32, path: *mut u8) -> i32;
}

#[cfg(all(windows, target_arch = "x86"))]
#[link(name = "kernel32")]
extern "system" {
    fn ExitProcess(code: u32) -> !;
}

#[cfg(all(windows, target_arch = "x86"))]
#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
}

#[cfg(all(windows, target_arch = "x86"))]
#[link(name = "user32")]
extern "system" {
    fn CreateWindowExA(
        ex_style: u32,
        class: *const u8,
        title: *const u8,
        style: u32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        parent: *mut c_void,
        menu: *mut c_void,
        instance: *mut c_void,
        param: *mut c_void,
    ) -> *mut c_void;
    fn DestroyWindow(window: *mut c_void) -> i32;
}

#[cfg(all(windows, target_arch = "x86"))]
use std::ffi::c_void;

/// `NV_FIXTURE_D3D=1`: a Direct3D 9 device found the way the game finds it (`LoadLibraryA`,
/// `GetProcAddress("Direct3DCreate9")`), a few state calls, a shader, one draw and a Present,
/// for testing the recorder's device hooks. Returns a line for the report.
#[cfg(all(windows, target_arch = "x86"))]
fn d3d_test() -> String {
    type Create9 = unsafe extern "system" fn(u32) -> *mut c_void;
    // SAFETY: Direct3D 9 COM calls through the documented vtable slots (d3d9.h order), with
    // their documented arguments; every pointer passed lives through the call.
    unsafe {
        let slot = |obj: *mut c_void, n: usize| *(*(obj as *const *const usize)).add(n);
        let module = LoadLibraryA(c"d3d9.dll".as_ptr().cast());
        if module.is_null() {
            return "d3d: no d3d9.dll".into();
        }
        let create = GetProcAddress(module, c"Direct3DCreate9".as_ptr().cast());
        if create.is_null() {
            return "d3d: no Direct3DCreate9".into();
        }
        let d3d = std::mem::transmute::<*mut c_void, Create9>(create)(32);
        if d3d.is_null() {
            return "d3d: Direct3DCreate9 answered null".into();
        }
        let window = CreateWindowExA(
            0,
            c"STATIC".as_ptr().cast(),
            c"nv-trace-fixture".as_ptr().cast(),
            0,
            0,
            0,
            64,
            64,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        // D3DPRESENT_PARAMETERS: 64x64, format unknown, one buffer, discard, windowed.
        let mut params = [0u32; 14];
        params[0] = 64;
        params[1] = 64;
        params[3] = 1;
        params[6] = 1;
        params[7] = window as u32;
        params[8] = 1;
        let mut device: *mut c_void = std::ptr::null_mut();
        type CreateDevice = unsafe extern "system" fn(
            *mut c_void,
            u32,
            u32,
            *mut c_void,
            u32,
            *mut u32,
            *mut *mut c_void,
        ) -> i32;
        let hr = std::mem::transmute::<usize, CreateDevice>(slot(d3d, 16))(
            d3d,
            0,
            1,
            window,
            0x20,
            params.as_mut_ptr(),
            &mut device,
        );
        if hr < 0 || device.is_null() {
            DestroyWindow(window);
            return format!("d3d: CreateDevice failed {hr:#x}");
        }
        let dev = device;
        type W2 = unsafe extern "system" fn(*mut c_void, u32) -> i32;
        type W3 = unsafe extern "system" fn(*mut c_void, u32, u32) -> i32;
        type P2 = unsafe extern "system" fn(*mut c_void, *const c_void) -> i32;
        type CreateVs = unsafe extern "system" fn(*mut c_void, *const u32, *mut *mut c_void) -> i32;
        type Clear =
            unsafe extern "system" fn(*mut c_void, u32, *const c_void, u32, u32, f32, u32) -> i32;
        type DrawUp = unsafe extern "system" fn(*mut c_void, u32, u32, *const c_void, u32) -> i32;
        type Present = unsafe extern "system" fn(
            *mut c_void,
            *const c_void,
            *const c_void,
            *mut c_void,
            *const c_void,
        ) -> i32;
        type Unary = unsafe extern "system" fn(*mut c_void) -> i32;
        // vs_2_0: dcl_position v0; mov oPos, v0 (TEST DATA from the documented token layout).
        let vs_code = [
            0xfffe_0200u32,
            0x0200_001f,
            0x8000_0000,
            0x900f_0000,
            0x0200_0001,
            0xc00f_0000,
            0x90e4_0000,
            0x0000_ffff,
        ];
        let mut vs: *mut c_void = std::ptr::null_mut();
        let vs_hr =
            std::mem::transmute::<usize, CreateVs>(slot(dev, 91))(dev, vs_code.as_ptr(), &mut vs);
        let rs = std::mem::transmute::<usize, W3>(slot(dev, 57));
        rs(dev, 27, 1);
        rs(dev, 22, 1);
        std::mem::transmute::<usize, P2>(slot(dev, 92))(dev, vs as *const c_void);
        std::mem::transmute::<usize, P2>(slot(dev, 92))(dev, std::ptr::null());
        std::mem::transmute::<usize, W2>(slot(dev, 89))(dev, 0x44);
        std::mem::transmute::<usize, Unary>(slot(dev, 41))(dev);
        std::mem::transmute::<usize, Clear>(slot(dev, 43))(
            dev,
            0,
            std::ptr::null(),
            1,
            0xff20_4060,
            1.0,
            0,
        );
        // One triangle: x, y, z, rhw, colour.
        let tri: [u32; 15] = [
            0,
            0,
            0,
            0x3f80_0000,
            0xffff_ffff,
            0x4200_0000,
            0,
            0,
            0x3f80_0000,
            0xffff_ffff,
            0,
            0x4200_0000,
            0,
            0x3f80_0000,
            0xffff_ffff,
        ];
        let draw_hr = std::mem::transmute::<usize, DrawUp>(slot(dev, 83))(
            dev,
            4,
            1,
            tri.as_ptr() as *const c_void,
            20,
        );
        std::mem::transmute::<usize, Unary>(slot(dev, 42))(dev);
        let present_hr = std::mem::transmute::<usize, Present>(slot(dev, 17))(
            dev,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null_mut(),
            std::ptr::null(),
        );
        if !vs.is_null() {
            std::mem::transmute::<usize, Unary>(slot(vs, 2))(vs);
        }
        std::mem::transmute::<usize, Unary>(slot(dev, 2))(dev);
        std::mem::transmute::<usize, Unary>(slot(d3d, 2))(d3d);
        DestroyWindow(window);
        format!("d3d: vs {vs_hr:#x} draw {draw_hr:#x} present {present_hr:#x}")
    }
}

#[cfg(all(windows, target_arch = "x86"))]
fn main() {
    if std::env::var_os("NV_FIXTURE_D3D").is_some() {
        let line = d3d_test();
        println!("{line}");
        // SAFETY: ends the process through the recorder's ExitProcess redirection.
        unsafe { ExitProcess(0) }
    }
    if std::env::args().nth(1).as_deref() == Some("list") {
        println!("{:08x} cdecl ww u32 fx_add", fx_add as *const () as usize);
        println!(
            "{:08x} thiscall pw u32 fx_this",
            fx_this as *const () as usize
        );
        println!(
            "{:08x} cdecl f f32 fx_twice",
            fx_twice as *const () as usize
        );
        return;
    }
    let mut report = String::new();
    let cell = 40u32;
    let mut sum = 0u32;
    for i in 0..5u32 {
        // SAFETY: the test functions above, called with their conventions.
        unsafe {
            sum = sum.wrapping_add(fx_add(i, 100));
            sum = sum.wrapping_add(fx_this(&cell, i));
            sum = sum.wrapping_add(fx_twice(i as f32 + 0.25) as u32);
        }
    }
    report.push_str(&format!("sum={sum}\n"));
    // The folder the recorder gives for CSIDL_PERSONAL.
    let mut buf = [0u8; 260];
    // SAFETY: a MAX_PATH buffer.
    let hr = unsafe { SHGetFolderPathA(0, 5, 0, 0, buf.as_mut_ptr()) };
    let n = buf.iter().position(|&c| c == 0).unwrap_or(0);
    let personal = String::from_utf8_lossy(&buf[..n]).into_owned();
    report.push_str(&format!("personal hr={hr} {personal}\n"));
    // In the protected working directory: new files go to the overlay (and read back from
    // it), existing files can't be changed, an absent file's delete is "not found".
    let cwd = std::env::current_dir().unwrap_or_default();
    let tries: Vec<(&str, std::io::Result<()>)> = vec![
        (
            "new file (overlay)",
            std::fs::write("guard-test-new.txt", b"new"),
        ),
        (
            "new file in subfolder (overlay)",
            std::fs::write("Data/guard-test.txt", b"x"),
        ),
        (
            "new folder (overlay)",
            std::fs::create_dir("guard-test-dir"),
        ),
        (
            "new file in new folder (overlay)",
            std::fs::write("guard-test-dir/inner.txt", b"y"),
        ),
        (
            "append existing (refused)",
            std::fs::OpenOptions::new()
                .append(true)
                .open("existing.txt")
                .map(|_| ()),
        ),
        (
            "truncate existing (refused)",
            std::fs::write("existing.txt", b""),
        ),
        (
            "delete existing (refused)",
            std::fs::remove_file("existing.txt"),
        ),
        (
            "delete absent (not found)",
            std::fs::remove_file("absent.txt"),
        ),
        (
            "rename existing out (refused)",
            std::fs::rename("existing.txt", format!("{personal}/moved.txt")),
        ),
        (
            "existing by absolute path (refused)",
            std::fs::write(
                cwd.join("..")
                    .join(cwd.file_name().unwrap_or_default())
                    .join("existing.txt"),
                b"x",
            ),
        ),
    ];
    for (what, r) in &tries {
        report.push_str(&format!(
            "{what}: {}\n",
            match r {
                Ok(()) => "ok".to_string(),
                Err(e) => format!("error {:?}", e.kind()),
            }
        ));
    }
    let back = std::fs::read_to_string("guard-test-new.txt");
    report.push_str(&format!("read back new file: {back:?}\n"));
    let read = std::fs::read_to_string("existing.txt");
    report.push_str(&format!("read existing: {read:?}\n"));
    let ok = std::fs::write(format!("{personal}/fixture.txt"), report.as_bytes()).is_ok();
    println!("{report}wrote report: {ok}");
    // SAFETY: ends the process through the recorder's ExitProcess redirection.
    unsafe { ExitProcess(0) }
}
