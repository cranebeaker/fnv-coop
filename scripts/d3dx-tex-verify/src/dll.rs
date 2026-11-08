//! `--time-dll`: the player's 64-bit `d3dx9_38.dll` (System32) on the same textures, the way the
//! port loads them today (`engine_platform::d3dxreal::texture_levels`): the create call on a
//! real HAL device, every level read back with `LockRect(D3DLOCK_READONLY)`, the texture
//! released. Raw COM through the vtables; Windows only.
#![allow(clippy::upper_case_acronyms)]
use std::ffi::c_void;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
}
#[link(name = "user32")]
extern "system" {
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
}

/// `D3DPRESENT_PARAMETERS` (64-bit layout).
#[repr(C)]
#[derive(Default)]
struct Present {
    width: u32,
    height: u32,
    format: u32,
    count: u32,
    multisample: u32,
    quality: u32,
    swap: u32,
    window: usize,
    windowed: u32,
    auto_depth: u32,
    depth_format: u32,
    flags: u32,
    refresh: u32,
    interval: u32,
}

unsafe fn vt(object: *mut c_void, slot: usize) -> usize {
    *(*(object as *mut *const usize)).add(slot)
}

type CreateFromFile =
    unsafe extern "system" fn(*mut c_void, *const u8, u32, *mut *mut c_void) -> i32;

pub struct Dll {
    device: *mut c_void,
    texture: CreateFromFile,
    cube: CreateFromFile,
}

impl Dll {
    /// The DLL from the system folder and a HAL device made as the game makes it (behaviour
    /// 0x54: hardware vertex processing, pure, multithreaded), on a window that is never
    /// shown (no WS_VISIBLE: a windowed device needs a window, not a visible one).
    pub unsafe fn open() -> Dll {
        let d3d9 = LoadLibraryA(b"d3d9.dll\0".as_ptr());
        assert!(!d3d9.is_null());
        type Create9 = unsafe extern "system" fn(u32) -> *mut c_void;
        let create9: Create9 =
            std::mem::transmute(GetProcAddress(d3d9, b"Direct3DCreate9\0".as_ptr()));
        let d3d = create9(32);
        let class = b"STATIC\0";
        let window = CreateWindowExA(
            0,
            class.as_ptr(),
            class.as_ptr(),
            0,
            0,
            0,
            640,
            480,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        let mut present = Present {
            width: 640,
            height: 480,
            count: 1,
            swap: 1,
            window: window as usize,
            windowed: 1,
            ..Default::default()
        };
        type CreateDevice = unsafe extern "system" fn(
            *mut c_void,
            u32,
            u32,
            *mut c_void,
            u32,
            *mut Present,
            *mut *mut c_void,
        ) -> i32;
        let cd: CreateDevice = std::mem::transmute(vt(d3d, 16));
        let mut device = std::ptr::null_mut();
        let hr = cd(d3d, 0, 1, window, 0x54, &mut present, &mut device);
        assert_eq!(hr, 0, "CreateDevice");
        let dll = LoadLibraryA(b"d3dx9_38.dll\0".as_ptr());
        assert!(!dll.is_null(), "d3dx9_38.dll");
        Dll {
            device,
            texture: std::mem::transmute(GetProcAddress(
                dll,
                b"D3DXCreateTextureFromFileInMemory\0".as_ptr(),
            )),
            cube: std::mem::transmute(GetProcAddress(
                dll,
                b"D3DXCreateCubeTextureFromFileInMemory\0".as_ptr(),
            )),
        }
    }

    /// The create call and the read back of every level, as the port does it today; the bytes
    /// read (so nothing is optimized away).
    pub unsafe fn load(&self, data: &[u8], cube: bool) -> usize {
        let mut t = std::ptr::null_mut();
        let hr = (if cube { self.cube } else { self.texture })(
            self.device,
            data.as_ptr(),
            data.len() as u32,
            &mut t,
        );
        if hr != 0 || t.is_null() {
            return 0;
        }
        type Count = unsafe extern "system" fn(*mut c_void) -> u32;
        type Desc = unsafe extern "system" fn(*mut c_void, u32, *mut [u32; 8]) -> i32;
        type Lock =
            unsafe extern "system" fn(*mut c_void, u32, *mut [usize; 2], *const c_void, u32) -> i32;
        type Unlock = unsafe extern "system" fn(*mut c_void, u32) -> i32;
        type CubeLock = unsafe extern "system" fn(
            *mut c_void,
            u32,
            u32,
            *mut [usize; 2],
            *const c_void,
            u32,
        ) -> i32;
        type CubeUnlock = unsafe extern "system" fn(*mut c_void, u32, u32) -> i32;
        type Release = unsafe extern "system" fn(*mut c_void) -> u32;
        let levels = std::mem::transmute::<usize, Count>(vt(t, 13))(t);
        let mut sum = 0usize;
        for face in 0..if cube { 6 } else { 1 } {
            for level in 0..levels {
                let mut desc = [0u32; 8];
                std::mem::transmute::<usize, Desc>(vt(t, 17))(t, level, &mut desc);
                let mut locked = [0usize; 2];
                let lr = if cube {
                    std::mem::transmute::<usize, CubeLock>(vt(t, 19))(
                        t,
                        face,
                        level,
                        &mut locked,
                        std::ptr::null(),
                        0x10,
                    )
                } else {
                    std::mem::transmute::<usize, Lock>(vt(t, 19))(
                        t,
                        level,
                        &mut locked,
                        std::ptr::null(),
                        0x10,
                    )
                };
                if lr != 0 {
                    continue;
                }
                let (fmt, w, h) = (desc[0], desc[6] as usize, desc[7] as usize);
                let dxt = (0x3154_5844..=0x3554_5844).contains(&fmt);
                let (rows, row) = if dxt {
                    (
                        h.div_ceil(4),
                        w.div_ceil(4) * if fmt == 0x3154_5844 { 8 } else { 16 },
                    )
                } else {
                    (h, w * if fmt == 26 { 2 } else { 4 })
                };
                let pitch = locked[0] as u32 as i32 as isize;
                let bits = locked[1] as *const u8;
                let mut packed = Vec::with_capacity(rows * row);
                for r in 0..rows {
                    packed.extend_from_slice(std::slice::from_raw_parts(
                        bits.offset(r as isize * pitch),
                        row,
                    ));
                }
                sum += packed.len();
                if cube {
                    std::mem::transmute::<usize, CubeUnlock>(vt(t, 20))(t, face, level);
                } else {
                    std::mem::transmute::<usize, Unlock>(vt(t, 20))(t, level);
                }
            }
        }
        std::mem::transmute::<usize, Release>(vt(t, 2))(t);
        sum
    }
}
