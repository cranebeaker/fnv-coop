//! `d3d9-capture fullscreen`: how the retail Direct3D 9 runtime answers a full-screen device as
//! the game asks for it (#73). `NiDX9Renderer::Initialize` (`00e72e60`, its present parameters
//! from `00e6bf50`) asks with the player's `[Display]` size (`iSize W`, `iSize H`); the format
//! and the other parameters below are the ones it asks with.
//! The window is the game's (`WS_VISIBLE` only, class style 3, made in `0086a850`).
//!
//! This mode changes the display mode while the device lives: run it only when the display may
//! go full-screen. Each line: `fullscreen`, the case, then the HRESULT (hex) or the values read
//! back. Usage: `d3d9-capture fullscreen [WIDTH HEIGHT [X Y]]`: the player's `[Display]` size
//! (default 1280 720), and where the window starts (default 0 0).
use std::ffi::c_void;
use std::io::Write;

type P = *mut c_void;

#[link(name = "user32")]
extern "system" {
    fn GetWindowRect(window: P, rect: *mut [i32; 4]) -> i32;
    fn GetClientRect(window: P, rect: *mut [i32; 4]) -> i32;
    fn SetForegroundWindow(window: P) -> i32;
    fn GetForegroundWindow() -> P;
    fn PeekMessageA(msg: *mut [usize; 8], window: P, min: u32, max: u32, remove: u32) -> i32;
    fn TranslateMessage(msg: *const [usize; 8]) -> i32;
    fn DispatchMessageA(msg: *const [usize; 8]) -> isize;
}

/// `IDirect3D9` slots.
const D3D_ENUM_ADAPTER_MODES: usize = 7;
const D3D_GET_ADAPTER_MODE_COUNT: usize = 6;
const D3D_GET_ADAPTER_DISPLAY_MODE: usize = 8;
const D3D_CREATE_DEVICE: usize = 16;
/// `IDirect3DDevice9` slots.
const RELEASE: usize = 2;
const TEST_COOPERATIVE_LEVEL: usize = 3;
const GET_DISPLAY_MODE: usize = 8;
const GET_SWAP_CHAIN: usize = 14;
const RESET: usize = 16;
const PRESENT: usize = 17;
const GET_BACK_BUFFER: usize = 18;
const GET_DEPTH_STENCIL_SURFACE: usize = 40;
const BEGIN_SCENE: usize = 41;
const END_SCENE: usize = 42;
const CLEAR: usize = 43;
const GET_VIEWPORT: usize = 48;
/// `IDirect3DSwapChain9::GetPresentParameters`, `IDirect3DSurface9::GetDesc`.
const SWAP_CHAIN_PRESENT_PARAMETERS: usize = 9;
const SURFACE_GET_DESC: usize = 12;

const WS_VISIBLE: u32 = 0x1000_0000;

unsafe fn call(object: P, index: usize, args: &[usize]) -> u32 {
    let f = *(*(object as *const *const usize)).add(index);
    match args.len() {
        0 => std::mem::transmute::<usize, unsafe extern "system" fn(P) -> u32>(f)(object),
        1 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize) -> u32>(f)(object, args[0]),
        2 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize) -> u32>(f)(object, args[0], args[1]),
        3 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2]),
        4 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3]),
        5 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4]),
        6 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4], args[5]),
        n => panic!("{n} arguments"),
    }
}

/// `D3DPRESENT_PARAMETERS` as 14 words (the window word is pointer-sized here).
#[repr(C)]
#[derive(Clone, Copy)]
struct Present {
    width: u32,
    height: u32,
    format: u32,
    count: u32,
    multisample: u32,
    quality: u32,
    swap_effect: u32,
    window: P,
    windowed: i32,
    auto_depth: i32,
    depth_format: u32,
    flags: u32,
    refresh: u32,
    interval: u32,
}

impl Present {
    fn words(&self) -> String {
        format!(
            "{:x} {:x} {:x} {:x} {:x} {:x} {:x} window:{} {:x} {:x} {:x} {:x} {:x} {:x}",
            self.width,
            self.height,
            self.format,
            self.count,
            self.multisample,
            self.quality,
            self.swap_effect,
            if self.window.is_null() { "null" } else { "set" },
            self.windowed,
            self.auto_depth,
            self.depth_format,
            self.flags,
            self.refresh,
            self.interval
        )
    }
}

fn pump() {
    let mut msg = [0usize; 8];
    unsafe {
        while PeekMessageA(&mut msg, std::ptr::null_mut(), 0, 0, 1) != 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

unsafe fn line(o: &mut impl Write, case: &str, value: String) {
    writeln!(o, "fullscreen\t{case}\t{value}").unwrap();
    o.flush().unwrap();
}

unsafe fn mode(d3d: P) -> [u32; 4] {
    let mut m = [0u32; 4];
    call(d3d, D3D_GET_ADAPTER_DISPLAY_MODE, &[0, m.as_mut_ptr() as usize]);
    m
}

unsafe fn window_state(window: P) -> String {
    let (mut r, mut c) = ([0i32; 4], [0i32; 4]);
    GetWindowRect(window, &mut r);
    GetClientRect(window, &mut c);
    let style = crate::GetWindowLongA(window, -16) as u32;
    let ex = crate::GetWindowLongA(window, -20) as u32;
    format!(
        "rect {} {} {} {} client {} {} style {style:x} exstyle {ex:x} foreground {} screen {}x{}",
        r[0],
        r[1],
        r[2],
        r[3],
        c[2],
        c[3],
        GetForegroundWindow() == window,
        crate::GetSystemMetrics(0),
        crate::GetSystemMetrics(1)
    )
}

pub unsafe fn measure(o: &mut impl Write) {
    let arg = |i: usize, d: u32| std::env::args().nth(i).and_then(|v| v.parse().ok()).unwrap_or(d);
    let (width, height) = (arg(2, 1280), arg(3, 720));
    // Where the window starts, so that a run can tell whether the runtime moves it.
    let (left, top) = (arg(4, 0) as i32, arg(5, 0) as i32);
    let instance = crate::GetModuleHandleA(std::ptr::null());
    let name = b"nv-rs d3d9-capture fullscreen\0";
    let class = crate::WndClassA {
        style: 3,
        procedure: crate::DefWindowProcA,
        class_extra: 0,
        window_extra: 0,
        instance,
        icon: std::ptr::null_mut(),
        cursor: std::ptr::null_mut(),
        background: std::ptr::null_mut(),
        menu_name: std::ptr::null(),
        class_name: name.as_ptr(),
    };
    crate::RegisterClassA(&class);
    // As WinMain: WS_VISIBLE only, the client size of the device, at 0,0 unless given.
    let mut rect = [0, 0, width as i32, height as i32];
    crate::AdjustWindowRectEx(&mut rect, WS_VISIBLE, 0, 0);
    let window = crate::CreateWindowExA(0, name.as_ptr(), name.as_ptr(), WS_VISIBLE, left, top,
        rect[2] - rect[0], rect[3] - rect[1], std::ptr::null_mut(), std::ptr::null_mut(), instance,
        std::ptr::null_mut());
    assert!(!window.is_null());
    SetForegroundWindow(window);
    pump();
    let d3d = crate::Direct3DCreate9(32) as P;
    assert!(!d3d.is_null());

    let before = mode(d3d);
    line(o, "adapter mode before", format!("{} {} {} {:x}", before[0], before[1], before[2], before[3]));
    // Whether the size is one of the adapter's X8R8G8B8 modes, and at which rates.
    let count = call(d3d, D3D_GET_ADAPTER_MODE_COUNT, &[0, 22]);
    let mut rates = Vec::new();
    for i in 0..count as usize {
        let mut m = [0u32; 4];
        call(d3d, D3D_ENUM_ADAPTER_MODES, &[0, 22, i, m.as_mut_ptr() as usize]);
        if m[0] == width && m[1] == height {
            rates.push(m[2]);
        }
    }
    line(o, "listed X8R8G8B8 rates of the size", format!("{rates:?} of {count} modes"));
    line(o, "window before", window_state(window));

    // The game's parameters (the stop line): X8R8G8B8, one back buffer, discard, full-screen,
    // no automatic depth (D24S8 named), a lockable back buffer, the default rate, interval one.
    let game = Present {
        width,
        height,
        format: 22,
        count: 1,
        multisample: 0,
        quality: 0,
        swap_effect: 1,
        window,
        windowed: 0,
        auto_depth: 0,
        depth_format: 0x4b,
        flags: 1,
        refresh: 0,
        interval: 1,
    };
    // Behaviour 0x54: hardware vertex processing, a pure device, multithreaded.
    let create = |pp: &mut Present, focus: P| -> (u32, P) {
        let mut device: P = std::ptr::null_mut();
        let hr = call(d3d, D3D_CREATE_DEVICE, &[0, 1, focus as usize, 0x54, pp as *mut Present as usize, &mut device as *mut P as usize]);
        pump();
        (hr, device)
    };

    let mut pp = game;
    let (hr, device) = create(&mut pp, window);
    line(o, "CreateDevice game", format!("{hr:x}"));
    line(o, "parameters after", pp.words());
    if !device.is_null() {
        let after = mode(d3d);
        line(o, "adapter mode with the device", format!("{} {} {} {:x}", after[0], after[1], after[2], after[3]));
        let mut dm = [0u32; 4];
        let hr = call(device, GET_DISPLAY_MODE, &[0, dm.as_mut_ptr() as usize]);
        line(o, "GetDisplayMode", format!("{hr:x} {} {} {} {:x}", dm[0], dm[1], dm[2], dm[3]));
        line(o, "window with the device", window_state(window));
        line(o, "TestCooperativeLevel", format!("{:x}", call(device, TEST_COOPERATIVE_LEVEL, &[])));
        let mut chain: P = std::ptr::null_mut();
        let hr = call(device, GET_SWAP_CHAIN, &[0, &mut chain as *mut P as usize]);
        let mut cp = game;
        if !chain.is_null() {
            call(chain, SWAP_CHAIN_PRESENT_PARAMETERS, &[&mut cp as *mut Present as usize]);
            call(chain, RELEASE, &[]);
        }
        line(o, "swap chain parameters", format!("{hr:x} {}", cp.words()));
        let mut back: P = std::ptr::null_mut();
        let hr = call(device, GET_BACK_BUFFER, &[0, 0, 0, &mut back as *mut P as usize]);
        let mut desc = [0u32; 8];
        if !back.is_null() {
            call(back, SURFACE_GET_DESC, &[desc.as_mut_ptr() as usize]);
            call(back, RELEASE, &[]);
        }
        line(o, "back buffer desc", format!("{hr:x} {desc:x?}"));
        let mut depth: P = std::ptr::null_mut();
        let hr = call(device, GET_DEPTH_STENCIL_SURFACE, &[&mut depth as *mut P as usize]);
        if !depth.is_null() {
            call(depth, RELEASE, &[]);
        }
        line(o, "GetDepthStencilSurface", format!("{hr:x} {}", !depth.is_null()));
        let mut vp = [0u32; 6];
        call(device, GET_VIEWPORT, &[vp.as_mut_ptr() as usize]);
        line(o, "viewport", format!("{vp:x?}"));
        for i in 0..3 {
            let b = call(device, BEGIN_SCENE, &[]);
            let c = call(device, CLEAR, &[0, 0, 1, 0xff00_0000, 0, 0]);
            let e = call(device, END_SCENE, &[]);
            let p = call(device, PRESENT, &[0, 0, 0, 0]);
            pump();
            line(o, &format!("frame {i}"), format!("begin {b:x} clear {c:x} end {e:x} present {p:x}"));
        }
        let mut rp = game;
        let hr = call(device, RESET, &[&mut rp as *mut Present as usize]);
        pump();
        line(o, "Reset to the same parameters", format!("{hr:x} {}", rp.words()));
        line(o, "TestCooperativeLevel after Reset", format!("{:x}", call(device, TEST_COOPERATIVE_LEVEL, &[])));
        let mut chain: P = std::ptr::null_mut();
        let hr = call(device, GET_SWAP_CHAIN, &[0, &mut chain as *mut P as usize]);
        let mut cp = game;
        if !chain.is_null() {
            call(chain, SWAP_CHAIN_PRESENT_PARAMETERS, &[&mut cp as *mut Present as usize]);
            call(chain, RELEASE, &[]);
        }
        line(o, "swap chain parameters after Reset", format!("{hr:x} {}", cp.words()));
        line(o, "window after Reset", window_state(window));
        let refs = call(device, RELEASE, &[]);
        pump();
        line(o, "Release", format!("{refs}"));
        let restored = mode(d3d);
        line(o, "adapter mode after", format!("{} {} {} {:x}", restored[0], restored[1], restored[2], restored[3]));
        line(o, "window after", window_state(window));
    }

    // Refusals: each changes one parameter of the game's (no display change when refused).
    let cases: [(&str, fn(&mut Present)); 5] = [
        ("refresh 1", |p| p.refresh = 1),
        ("width one less", |p| p.width -= 1),
        ("format unknown", |p| p.format = 0),
        ("4 back buffers", |p| p.count = 4),
        ("no device window", |p| p.window = std::ptr::null_mut()),
    ];
    for (case, change) in cases {
        let mut pp = game;
        change(&mut pp);
        let (hr, device) = create(&mut pp, window);
        line(o, &format!("CreateDevice {case}"), format!("{hr:x} after: {}", pp.words()));
        if !device.is_null() {
            call(device, RELEASE, &[]);
            pump();
        }
    }
    call(d3d, RELEASE, &[]);
    crate::DestroyWindow(window);
    pump();
}
