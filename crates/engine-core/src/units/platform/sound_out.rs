//! The sound the platform's mixer makes, played on the player's default output device through
//! WASAPI in shared mode, on a host thread of its own (the game never sees that thread). The
//! device's own mix format is used (32-bit float, as the measured devices have it); another
//! format is not provided. Windows only; elsewhere [`start`] answers `None` and nothing is
//! heard.
//!
//! The thread asks the [`Source`] for as many frames as the device can take each time the device
//! signals it (event driven, the device's period), so what plays is at most one device buffer
//! behind what the mixer was asked for.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// What the output thread plays: frames at the device's rate and channel count.
pub trait Source: Send {
    /// Fills `out` (interleaved, `channels` per frame) with the next frames.
    fn render(&mut self, rate: u32, channels: u16, out: &mut [f32]);
}

/// A running output: the thread and its stop flag.
pub struct Output {
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
    /// The device's rate and channels.
    pub rate: u32,
    pub channels: u16,
}

impl Drop for Output {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

/// The default output device's mix rate (its shared-mode format, the rate DirectSound mixes at),
/// asked once on a thread of its own; `None` when there is no device (or not on Windows).
pub fn device_rate() -> Option<u32> {
    #[cfg(windows)]
    {
        static RATE: std::sync::OnceLock<Option<u32>> = std::sync::OnceLock::new();
        *RATE.get_or_init(|| {
            std::thread::spawn(|| unsafe { wasapi::mix_rate() })
                .join()
                .ok()
                .flatten()
        })
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// Starts playing `source` on the default output device; `None` when there is no device (or
/// not on Windows).
pub fn start(source: Arc<Mutex<dyn Source>>) -> Option<Output> {
    #[cfg(windows)]
    {
        wasapi::start(source)
    }
    #[cfg(not(windows))]
    {
        let _ = source;
        None
    }
}

#[cfg(windows)]
mod wasapi {
    use super::*;
    use std::ffi::c_void;

    #[link(name = "ole32")]
    extern "system" {
        fn CoInitializeEx(reserved: *mut c_void, flags: u32) -> u32;
        fn CoCreateInstance(
            clsid: *const [u8; 16],
            outer: *mut c_void,
            context: u32,
            iid: *const [u8; 16],
            out: *mut *mut Obj,
        ) -> u32;
        fn CoTaskMemFree(p: *mut c_void);
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn CreateEventW(
            attributes: *mut c_void,
            manual: i32,
            initial: i32,
            name: *const u16,
        ) -> *mut c_void;
        fn WaitForSingleObject(handle: *mut c_void, ms: u32) -> u32;
        fn CloseHandle(handle: *mut c_void) -> i32;
        fn GetCurrentThread() -> *mut c_void;
        fn SetThreadPriority(thread: *mut c_void, priority: i32) -> i32;
    }
    #[link(name = "avrt")]
    extern "system" {
        fn AvSetMmThreadCharacteristicsW(task: *const u16, index: *mut u32) -> *mut c_void;
        fn AvSetMmThreadPriority(task: *mut c_void, priority: i32) -> i32;
        fn AvRevertMmThreadCharacteristics(task: *mut c_void) -> i32;
    }

    /// Runs the calling thread as the player's 32-bit dsound.dll runs its mixing thread (read
    /// from the DLL: `1005cbd8`..`1005cbff`, `1005ceb3`..`1005cee0`): time-critical priority (15),
    /// registered with the multimedia class scheduler as an "Audio" task at high priority
    /// (`AVRT_PRIORITY_HIGH`), so busy processors do not hold up the device's refills. Answers
    /// the task to give back when the thread ends (null when it was refused; nothing changes
    /// then).
    unsafe fn audio_priority() -> *mut c_void {
        SetThreadPriority(GetCurrentThread(), 15);
        let task: Vec<u16> = "Audio\0".encode_utf16().collect();
        let mut index = 0u32;
        let handle = AvSetMmThreadCharacteristicsW(task.as_ptr(), &mut index);
        if handle.is_null() || handle as isize == -1 {
            return std::ptr::null_mut();
        }
        AvSetMmThreadPriority(handle, 1);
        handle
    }

    #[repr(C)]
    struct Obj {
        vtable: *const usize,
    }

    unsafe fn slot<F: Copy>(o: *mut Obj, offset: usize) -> F {
        let f = *(*o).vtable.add(offset / std::mem::size_of::<usize>());
        std::mem::transmute_copy(&f)
    }

    type Fn0 = unsafe extern "system" fn(*mut Obj) -> u32;
    type Fn1 = unsafe extern "system" fn(*mut Obj, usize) -> u32;
    type Fn2 = unsafe extern "system" fn(*mut Obj, usize, usize) -> u32;
    type Fn3 = unsafe extern "system" fn(*mut Obj, usize, usize, usize) -> u32;
    type Fn4 = unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize) -> u32;
    type FnInit =
        unsafe extern "system" fn(*mut Obj, u32, u32, i64, i64, *const u8, *const c_void) -> u32;

    fn guid(s: &str) -> [u8; 16] {
        let h: Vec<u8> = s
            .bytes()
            .filter(u8::is_ascii_hexdigit)
            .map(|c| (c as char).to_digit(16).unwrap_or(0) as u8)
            .collect();
        let b = |i: usize| h[2 * i] << 4 | h[2 * i + 1];
        [
            b(3),
            b(2),
            b(1),
            b(0),
            b(5),
            b(4),
            b(7),
            b(6),
            b(8),
            b(9),
            b(10),
            b(11),
            b(12),
            b(13),
            b(14),
            b(15),
        ]
    }

    /// The default device's mix rate (`GetMixFormat`), on a thread with COM of its own.
    pub unsafe fn mix_rate() -> Option<u32> {
        CoInitializeEx(std::ptr::null_mut(), 0);
        let mut en: *mut Obj = std::ptr::null_mut();
        let r = CoCreateInstance(
            &guid("bcde0395-e52f-467c-8e3d-c4579291692e"),
            std::ptr::null_mut(),
            0x17,
            &guid("a95664d2-9614-4f35-a746-de8db63617e6"),
            &mut en,
        );
        if r != 0 {
            return None;
        }
        let w = std::mem::size_of::<usize>();
        let mut dev: *mut Obj = std::ptr::null_mut();
        let r = slot::<Fn3>(en, 4 * w)(en, 0, 0, &mut dev as *mut _ as usize);
        slot::<Fn0>(en, 2 * w)(en);
        if r != 0 {
            return None;
        }
        let mut client: *mut Obj = std::ptr::null_mut();
        let iid = guid("1cb9ad4c-dbfa-4c32-b178-c2f568a703b2");
        let r = slot::<Fn4>(dev, 3 * w)(
            dev,
            iid.as_ptr() as usize,
            0x17,
            0,
            &mut client as *mut _ as usize,
        );
        slot::<Fn0>(dev, 2 * w)(dev);
        if r != 0 {
            return None;
        }
        let mut fmt: *mut u8 = std::ptr::null_mut();
        let r = slot::<Fn1>(client, 8 * w)(client, &mut fmt as *mut _ as usize);
        let rate = (r == 0 && !fmt.is_null())
            .then(|| u32::from_le_bytes([*fmt.add(4), *fmt.add(5), *fmt.add(6), *fmt.add(7)]));
        if !fmt.is_null() {
            CoTaskMemFree(fmt as *mut c_void);
        }
        slot::<Fn0>(client, 2 * w)(client);
        rate
    }

    /// The device's client, render client, event, buffer size, rate and channels.
    struct Device {
        client: *mut Obj,
        render: *mut Obj,
        event: *mut c_void,
        frames: u32,
        rate: u32,
        channels: u16,
    }

    unsafe fn open() -> Option<Device> {
        CoInitializeEx(std::ptr::null_mut(), 0);
        let mut en: *mut Obj = std::ptr::null_mut();
        let r = CoCreateInstance(
            &guid("bcde0395-e52f-467c-8e3d-c4579291692e"),
            std::ptr::null_mut(),
            0x17,
            &guid("a95664d2-9614-4f35-a746-de8db63617e6"),
            &mut en,
        );
        if r != 0 {
            return None;
        }
        let mut dev: *mut Obj = std::ptr::null_mut();
        let r = slot::<Fn3>(en, 0x10 * std::mem::size_of::<usize>() / 4)(
            en,
            0,
            0,
            &mut dev as *mut _ as usize,
        );
        slot::<Fn0>(en, 2 * std::mem::size_of::<usize>())(en);
        if r != 0 {
            return None;
        }
        let w = std::mem::size_of::<usize>();
        let mut client: *mut Obj = std::ptr::null_mut();
        let iid = guid("1cb9ad4c-dbfa-4c32-b178-c2f568a703b2");
        let r = slot::<Fn4>(dev, 3 * w)(
            dev,
            iid.as_ptr() as usize,
            0x17,
            0,
            &mut client as *mut _ as usize,
        );
        slot::<Fn0>(dev, 2 * w)(dev);
        if r != 0 {
            return None;
        }
        let mut fmt: *mut u8 = std::ptr::null_mut();
        slot::<Fn1>(client, 8 * w)(client, &mut fmt as *mut _ as usize);
        let tag = u16::from_le_bytes([*fmt, *fmt.add(1)]);
        let channels = u16::from_le_bytes([*fmt.add(2), *fmt.add(3)]);
        let rate = u32::from_le_bytes([*fmt.add(4), *fmt.add(5), *fmt.add(6), *fmt.add(7)]);
        let bits = u16::from_le_bytes([*fmt.add(14), *fmt.add(15)]);
        // IEEE float, directly (3) or through WAVE_FORMAT_EXTENSIBLE (0xfffe) with the float
        // subformat {00000003-0000-0010-8000-00aa00389b71}.
        let float = tag == 3 || (tag == 0xfffe && *fmt.add(24) == 3);
        if !float || bits != 32 {
            CoTaskMemFree(fmt as *mut c_void);
            panic!("the sound device mixes {bits}-bit format {tag:#x}: not provided (32-bit float only)");
        }
        // Shared, event driven, the engine's default period.
        let r = slot::<FnInit>(client, 3 * w)(client, 0, 0x4_0000, 0, 0, fmt, std::ptr::null());
        CoTaskMemFree(fmt as *mut c_void);
        if r != 0 {
            return None;
        }
        let event = CreateEventW(std::ptr::null_mut(), 0, 0, std::ptr::null());
        slot::<Fn1>(client, 13 * w)(client, event as usize);
        let mut frames = 0u32;
        slot::<Fn1>(client, 4 * w)(client, &mut frames as *mut _ as usize);
        let mut render: *mut Obj = std::ptr::null_mut();
        let iid = guid("f294acfc-3146-4483-a7bf-addca7c260e2");
        slot::<Fn2>(client, 14 * w)(
            client,
            iid.as_ptr() as usize,
            &mut render as *mut _ as usize,
        );
        if render.is_null() {
            return None;
        }
        Some(Device {
            client,
            render,
            event,
            frames,
            rate,
            channels,
        })
    }

    pub fn start(source: Arc<Mutex<dyn Source>>) -> Option<Output> {
        let stop = Arc::new(AtomicBool::new(false));
        let (tx, rx) = std::sync::mpsc::channel();
        let flag = stop.clone();
        let thread = std::thread::Builder::new()
            .name("nv sound out".into())
            .spawn(move || unsafe {
                let task = audio_priority();
                match open() {
                    Some(d) => {
                        let _ = tx.send(Some((d.rate, d.channels)));
                        run(&d, &source, &flag);
                    }
                    None => {
                        let _ = tx.send(None);
                    }
                }
                // As dsound.dll does before its thread ends (`AvRevertMmThreadCharacteristics`).
                if !task.is_null() {
                    AvRevertMmThreadCharacteristics(task);
                }
            })
            .ok()?;
        let (rate, channels) = rx.recv().ok().flatten()?;
        Some(Output {
            stop,
            thread: Some(thread),
            rate,
            channels,
        })
    }

    unsafe fn run(d: &Device, source: &Arc<Mutex<dyn Source>>, stop: &AtomicBool) {
        let w = std::mem::size_of::<usize>();
        let mut scratch = vec![0f32; d.frames as usize * d.channels as usize];
        slot::<Fn0>(d.client, 10 * w)(d.client);
        while !stop.load(Ordering::Relaxed) {
            WaitForSingleObject(d.event, 200);
            let mut padding = 0u32;
            slot::<Fn1>(d.client, 6 * w)(d.client, &mut padding as *mut _ as usize);
            let free = d.frames.saturating_sub(padding) as usize;
            if free == 0 {
                continue;
            }
            let mut data: *mut f32 = std::ptr::null_mut();
            if slot::<Fn2>(d.render, 3 * w)(d.render, free, &mut data as *mut _ as usize) != 0 {
                continue;
            }
            let n = free * d.channels as usize;
            let out = &mut scratch[..n];
            out.fill(0.0);
            if let Ok(mut s) = source.lock() {
                s.render(d.rate, d.channels, out);
            }
            std::ptr::copy_nonoverlapping(out.as_ptr(), data, n);
            slot::<Fn2>(d.render, 4 * w)(d.render, free, 0);
        }
        slot::<Fn0>(d.client, 11 * w)(d.client);
        slot::<Fn0>(d.render, 2 * w)(d.render);
        slot::<Fn0>(d.client, 2 * w)(d.client);
        CloseHandle(d.event);
    }
}
