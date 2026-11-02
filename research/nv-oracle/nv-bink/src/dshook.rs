//! `--hook-dsound`: records what the library does with DirectSound when it plays a movie's sound
//! (`BinkOpenDirectSound`): every call it makes on the device and its buffers, with the time,
//! the thread, the arguments and the answer, to `dsound.tsv` in the output folder.
//!
//! `DirectSoundCreate` (which the library finds with `GetProcAddress`) is patched in this
//! process to jump to [`create`], which makes the object with `DirectSoundCreate8` and
//! replaces the slots it records in the device's and each buffer's vtable (made writable with
//! `VirtualProtect`). The originals are called through; nothing else changes. Only this
//! scratch process is affected.

use std::ffi::c_void;
use std::io::Write;
use std::sync::Mutex;

type Obj = *mut *mut usize;

static LOG: Mutex<Option<std::fs::File>> = Mutex::new(None);
/// The original functions, by (vtable, slot offset): the primary and secondary buffers have
/// their own vtables.
static ORIG: Mutex<Vec<(usize, usize, usize)>> = Mutex::new(Vec::new());
static mut FREQ: i64 = 0;
static mut ORIGIN: i64 = 0;

#[link(name = "kernel32")]
extern "system" {
    fn QueryPerformanceCounter(out: *mut i64) -> i32;
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
    fn GetCurrentThreadId() -> u32;
    fn LoadLibraryA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
}

fn now_us() -> i64 {
    let mut c = 0i64;
    unsafe {
        QueryPerformanceCounter(&mut c);
        (c - ORIGIN) * 1_000_000 / FREQ
    }
}

pub fn log(text: String) {
    let t = now_us();
    let tid = unsafe { GetCurrentThreadId() };
    if let Some(f) = LOG.lock().unwrap().as_mut() {
        let _ = writeln!(f, "{t}\t{tid}\t{text}");
    }
}

unsafe fn orig_of(this: Obj, offset: usize) -> usize {
    let vt = *this as usize;
    ORIG.lock()
        .unwrap()
        .iter()
        .find(|(v, o, _)| *v == vt && *o == offset)
        .map(|e| e.2)
        .expect("an original slot")
}

unsafe fn patch(obj: Obj, offset: usize, f: usize) {
    let vt = *obj;
    let at = vt.add(offset / 4);
    let mut o = ORIG.lock().unwrap();
    if o.iter().any(|(v, of, _)| *v == vt as usize && *of == offset) {
        return;
    }
    o.push((vt as usize, offset, *at));
    let mut old = 0u32;
    VirtualProtect(at as *mut c_void, 4, 0x40, &mut old);
    *at = f;
    VirtualProtect(at as *mut c_void, 4, old, &mut old);
}

/// Patches `DirectSoundCreate` in this process (before the library looks it up).
pub unsafe fn install(out: &std::path::Path) {
    QueryPerformanceFrequency(std::ptr::addr_of_mut!(FREQ));
    QueryPerformanceCounter(std::ptr::addr_of_mut!(ORIGIN));
    *LOG.lock().unwrap() = Some(std::fs::File::create(out.join("dsound.tsv")).unwrap());
    log("# time_us\tthread\tcall".into());
    let dll = LoadLibraryA(b"dsound.dll\0".as_ptr());
    let target = GetProcAddress(dll, b"DirectSoundCreate\0".as_ptr()) as *mut u8;
    let mut old = 0u32;
    VirtualProtect(target as *mut c_void, 5, 0x40, &mut old);
    *target = 0xe9;
    let rel = (create as *const () as usize as isize - (target as isize + 5)) as i32;
    std::ptr::copy_nonoverlapping(rel.to_le_bytes().as_ptr(), target.add(1), 4);
    VirtualProtect(target as *mut c_void, 5, old, &mut old);
}

type Create8 = unsafe extern "system" fn(*const c_void, *mut Obj, *mut c_void) -> u32;

unsafe extern "system" fn create(guid: *const c_void, out: *mut Obj, outer: *mut c_void) -> u32 {
    let dll = LoadLibraryA(b"dsound.dll\0".as_ptr());
    let f: Create8 = std::mem::transmute(GetProcAddress(dll, b"DirectSoundCreate8\0".as_ptr()));
    let r = f(guid, out, outer);
    log(format!("DirectSoundCreate(guid {guid:?}) = {r:#x}"));
    if r == 0 {
        let d = *out;
        patch(d, 0xc, create_buffer as *const () as usize);
        patch(d, 0x18, cooperative as *const () as usize);
        patch(d, 0x8, d_release as *const () as usize);
        patch(d, 0x10, d_get_caps as *const () as usize);
        patch(d, 0x0, d_qi as *const () as usize);
    }
    r
}

unsafe extern "system" fn d_qi(this: Obj, iid: *const [u8; 16], out: *mut usize) -> u32 {
    let f: unsafe extern "system" fn(Obj, *const [u8; 16], *mut usize) -> u32 = std::mem::transmute(orig_of(this, 0x0));
    let r = f(this, iid, out);
    log(format!("IDirectSound::QueryInterface({:02x?}) = {r:#x}", *iid));
    r
}

unsafe extern "system" fn d_release(this: Obj) -> u32 {
    let f: unsafe extern "system" fn(Obj) -> u32 = std::mem::transmute(orig_of(this, 0x8));
    let r = f(this);
    log(format!("IDirectSound::Release = {r}"));
    r
}

unsafe extern "system" fn d_get_caps(this: Obj, caps: *mut u32) -> u32 {
    let f: unsafe extern "system" fn(Obj, *mut u32) -> u32 = std::mem::transmute(orig_of(this, 0x10));
    let r = f(this, caps);
    log(format!("IDirectSound::GetCaps = {r:#x} flags {:#x}", *caps.add(1)));
    r
}

unsafe extern "system" fn cooperative(this: Obj, window: usize, level: u32) -> u32 {
    let f: unsafe extern "system" fn(Obj, usize, u32) -> u32 = std::mem::transmute(orig_of(this, 0x18));
    let r = f(this, window, level);
    log(format!("SetCooperativeLevel({window:#x}, {level}) = {r:#x}"));
    r
}

unsafe extern "system" fn create_buffer(this: Obj, desc: *const u32, out: *mut Obj, outer: usize) -> u32 {
    let f: unsafe extern "system" fn(Obj, *const u32, *mut Obj, usize) -> u32 =
        std::mem::transmute(orig_of(this, 0xc));
    let r = f(this, desc, out, outer);
    let size = *desc;
    let flags = *desc.add(1);
    let bytes = *desc.add(2);
    let wf = *desc.add(4) as *const u8;
    let format = if wf.is_null() {
        "none".to_string()
    } else {
        let n = 18 + u16::from_le_bytes([*wf.add(16), *wf.add(17)]) as usize;
        std::slice::from_raw_parts(wf, n.min(64)).iter().map(|b| format!("{b:02x}")).collect()
    };
    log(format!(
        "CreateSoundBuffer(size {size:#x}, flags {flags:#x}, bytes {bytes}, format {format}) = {r:#x} -> {:?}",
        if r == 0 { *out } else { std::ptr::null_mut() }
    ));
    if r == 0 {
        let b = *out;
        patch(b, 0x0, b_qi as *const () as usize);
        patch(b, 0x8, b_release as *const () as usize);
        patch(b, 0xc, b_get_caps as *const () as usize);
        patch(b, 0x10, get_position as *const () as usize);
        patch(b, 0x14, get_format as *const () as usize);
        patch(b, 0x24, get_status as *const () as usize);
        patch(b, 0x2c, lock as *const () as usize);
        patch(b, 0x30, play as *const () as usize);
        patch(b, 0x34, set_position as *const () as usize);
        patch(b, 0x38, set_format as *const () as usize);
        patch(b, 0x3c, set_volume as *const () as usize);
        patch(b, 0x40, set_pan as *const () as usize);
        patch(b, 0x44, set_frequency as *const () as usize);
        patch(b, 0x48, stop as *const () as usize);
        patch(b, 0x4c, unlock as *const () as usize);
    }
    r
}

macro_rules! simple {
    ($name:ident, $slot:expr, $text:expr, $($a:ident: $t:ty),*) => {
        unsafe extern "system" fn $name(this: Obj $(, $a: $t)*) -> u32 {
            let f: unsafe extern "system" fn(Obj $(, $t)*) -> u32 = std::mem::transmute(orig_of(this, $slot));
            let r = f(this $(, $a)*);
            log(format!(concat!($text, "({:?}", $(", ", stringify!($a), " {:#x}",)* ") = {:#x}"), this $(, $a)*, r));
            r
        }
    };
}

simple!(b_release, 0x8, "Release", );
simple!(play, 0x30, "Play", reserved: u32, priority: u32, flags: u32);
simple!(set_position, 0x34, "SetCurrentPosition", at: u32);
simple!(set_volume, 0x3c, "SetVolume", v: u32);
simple!(set_pan, 0x40, "SetPan", v: u32);
simple!(set_frequency, 0x44, "SetFrequency", v: u32);
simple!(stop, 0x48, "Stop", );
simple!(set_format, 0x38, "SetFormat", format: u32);

unsafe extern "system" fn b_qi(this: Obj, iid: *const [u8; 16], out: *mut usize) -> u32 {
    let f: unsafe extern "system" fn(Obj, *const [u8; 16], *mut usize) -> u32 = std::mem::transmute(orig_of(this, 0x0));
    let r = f(this, iid, out);
    log(format!("Buffer::QueryInterface({this:?}, {:02x?}) = {r:#x}", *iid));
    r
}

unsafe extern "system" fn b_get_caps(this: Obj, caps: *mut u32) -> u32 {
    let f: unsafe extern "system" fn(Obj, *mut u32) -> u32 = std::mem::transmute(orig_of(this, 0xc));
    let r = f(this, caps);
    log(format!("Buffer::GetCaps({this:?}) = {r:#x} flags {:#x} bytes {}", *caps.add(1), *caps.add(2)));
    r
}

unsafe extern "system" fn get_format(this: Obj, wf: *mut u8, size: u32, written: *mut u32) -> u32 {
    let f: unsafe extern "system" fn(Obj, *mut u8, u32, *mut u32) -> u32 = std::mem::transmute(orig_of(this, 0x14));
    let r = f(this, wf, size, written);
    log(format!("GetFormat({this:?}, size {size}) = {r:#x}"));
    r
}

unsafe extern "system" fn get_status(this: Obj, out: *mut u32) -> u32 {
    let f: unsafe extern "system" fn(Obj, *mut u32) -> u32 = std::mem::transmute(orig_of(this, 0x24));
    let r = f(this, out);
    log(format!("GetStatus({this:?}) = {r:#x} status {:#x}", *out));
    r
}

unsafe extern "system" fn get_position(this: Obj, play: *mut u32, write: *mut u32) -> u32 {
    let f: unsafe extern "system" fn(Obj, *mut u32, *mut u32) -> u32 = std::mem::transmute(orig_of(this, 0x10));
    let r = f(this, play, write);
    let p = if play.is_null() { -1 } else { *play as i64 };
    let w = if write.is_null() { -1 } else { *write as i64 };
    log(format!("GetCurrentPosition({this:?}) = {r:#x} play {p} write {w}"));
    r
}

#[allow(clippy::too_many_arguments)]
unsafe extern "system" fn lock(
    this: Obj,
    offset: u32,
    bytes: u32,
    p1: *mut usize,
    n1: *mut u32,
    p2: *mut usize,
    n2: *mut u32,
    flags: u32,
) -> u32 {
    let f: unsafe extern "system" fn(Obj, u32, u32, *mut usize, *mut u32, *mut usize, *mut u32, u32) -> u32 =
        std::mem::transmute(orig_of(this, 0x2c));
    let r = f(this, offset, bytes, p1, n1, p2, n2, flags);
    let second = if n2.is_null() { -1 } else { *n2 as i64 };
    log(format!("Lock({this:?}, offset {offset}, bytes {bytes}, flags {flags:#x}) = {r:#x} n1 {} n2 {second}", *n1));
    r
}

unsafe extern "system" fn unlock(this: Obj, p1: usize, n1: u32, p2: usize, n2: u32) -> u32 {
    let f: unsafe extern "system" fn(Obj, usize, u32, usize, u32) -> u32 = std::mem::transmute(orig_of(this, 0x4c));
    // A checksum of what was written, to tell silence from data.
    let sum = |p: usize, n: u32| -> u64 {
        if p == 0 {
            return 0;
        }
        std::slice::from_raw_parts(p as *const u8, n as usize).iter().map(|&b| u64::from(b)).sum()
    };
    let s = (sum(p1, n1), sum(p2, n2));
    let r = f(this, p1, n1, p2, n2);
    log(format!("Unlock({this:?}, n1 {n1}, n2 {n2}) = {r:#x} sums {s:?}"));
    r
}
