//! The sound the platform's mixer makes, played on the player's default output device through
//! WASAPI in shared mode, on a host thread of its own per stream (the game never sees those
//! threads). The endpoint's mix format gives the rate and the speaker layout ([`Layout`]: its
//! channels and speaker mask; on this machine 8 channels, mask 0x63F, for a program named
//! `FalloutNV.exe` and 2 channels, mask 0x3, for any other name, measured). A stream is opened
//! either in that format as it is, or in another layout at the same rate, 32-bit float, the way
//! DirectSound opens its two-channel stream on a larger endpoint: `IsFormatSupported` first, then
//! `Initialize` with that format (measured as `FalloutNV.exe`, 2026-11-03: `S_OK`). A mix format
//! that is not 32-bit float, or a layout DirectSound does not place 3D buffers on
//! (`super::speakers::Layout::provided`), is not provided: [`start`] then refuses, named, and
//! nothing is heard. Windows only; elsewhere [`start`] answers that there is no device.
//!
//! The thread asks the [`Source`] for as many frames as the device can take each time the device
//! signals it (event driven, the device's period), so what plays is at most one device buffer
//! behind what the mixer was asked for.

pub use super::speakers::Layout;
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
    /// The stream's rate, and its channels and speakers.
    pub rate: u32,
    pub layout: Layout,
}

/// Why [`start`] opened no stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// No default output device (or not on Windows).
    NoDevice,
    /// The endpoint's mix format is not provided (named).
    Format(String),
    /// The endpoint does not take the layout asked for (`IsFormatSupported` other than `S_OK`).
    Layout,
}

#[cfg(test)]
impl Output {
    /// A stream that plays nothing (no thread), as an endpoint opened it: the tests'.
    pub fn idle(rate: u32, layout: Layout) -> Output {
        Output {
            stop: Arc::new(AtomicBool::new(false)),
            thread: None,
            rate,
            layout,
        }
    }
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
    device_mix().map(|(rate, _)| rate)
}

/// The default output device's mix rate and layout (its shared-mode format), asked once on a
/// thread of its own; `None` when there is no device (or not on Windows).
pub fn device_mix() -> Option<(u32, Layout)> {
    #[cfg(windows)]
    {
        static MIX: std::sync::OnceLock<Option<(u32, Layout)>> = std::sync::OnceLock::new();
        *MIX.get_or_init(|| {
            std::thread::spawn(|| unsafe { wasapi::mix_format() })
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

/// The layout of a mix format (`WAVEFORMATEX` bytes, and what follows them): a
/// `WAVEFORMATEXTENSIBLE`'s (tag 0xFFFE, at least 22 more bytes) channel mask, else DirectSound's
/// rule (`super::speakers::Layout::of_mix`).
pub fn layout_of(format: &[u8]) -> Option<Layout> {
    let w16 = |o: usize| {
        format
            .get(o..o + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
    };
    let (tag, channels) = (w16(0)?, w16(2)?);
    let mask = if tag == 0xfffe && w16(16)? >= 22 {
        let b = format.get(20..24)?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    } else {
        None
    };
    Some(Layout::of_mix(channels, mask))
}

/// Why a mix layout is not provided, for stderr.
pub fn not_provided(l: Layout) -> String {
    if l.mask == 0 {
        // `10056ab3`: on a device with mask 0 a buffer must have the device's channels and may
        // not have `DSBCAPS_CTRL3D` or `DSBCAPS_CTRLPAN`; every buffer the game makes has one or
        // the other (`00aeb270` in the exe: 0x40 or 0x30010, each with 0x2a0).
        return format!(
            "the sound device mixes {} channels with speaker mask 0 (direct out): DirectSound \
             refuses every buffer the game makes there (each is 3D or panned), so the original \
             is silent on it too",
            l.channels
        );
    }
    format!(
        "the sound device mixes {} channels, speaker mask {:#x}: not provided (DirectSound places \
         3D sound only on mono, stereo, quad, surround, 5.1 and 7.1 layouts)",
        l.channels, l.mask
    )
}

/// Starts playing `source` on the default output device, in its mix format as it is (`want`
/// `None`, or the mix format's own layout) or in the layout `want` at the mix rate, 32-bit float
/// (when the endpoint takes it).
pub fn start(source: Arc<Mutex<dyn Source>>, want: Option<Layout>) -> Result<Output, Refused> {
    #[cfg(windows)]
    {
        wasapi::start(source, want)
    }
    #[cfg(not(windows))]
    {
        let _ = (source, want);
        Err(Refused::NoDevice)
    }
}

/// The `WAVEFORMATEXTENSIBLE` of 32-bit float at `rate` in `layout`, as DirectSound gives
/// `IsFormatSupported` and `Initialize` (measured: `feff0200 80bb0000 00dc0500 0800 2000 1600
/// 2000 03000000` and the float subformat for two channels at 48000 Hz).
pub fn float_format(rate: u32, layout: Layout) -> [u8; 40] {
    let block = 4 * layout.channels;
    let mut f = [0u8; 40];
    f[0..2].copy_from_slice(&0xfffeu16.to_le_bytes());
    f[2..4].copy_from_slice(&layout.channels.to_le_bytes());
    f[4..8].copy_from_slice(&rate.to_le_bytes());
    f[8..12].copy_from_slice(&(rate * u32::from(block)).to_le_bytes());
    f[12..14].copy_from_slice(&block.to_le_bytes());
    f[14..16].copy_from_slice(&32u16.to_le_bytes());
    f[16..18].copy_from_slice(&22u16.to_le_bytes());
    f[18..20].copy_from_slice(&32u16.to_le_bytes());
    f[20..24].copy_from_slice(&layout.mask.to_le_bytes());
    // KSDATAFORMAT_SUBTYPE_IEEE_FLOAT {00000003-0000-0010-8000-00aa00389b71}.
    f[24..40].copy_from_slice(&[
        3, 0, 0, 0, 0, 0, 0x10, 0, 0x80, 0, 0, 0xaa, 0, 0x38, 0x9b, 0x71,
    ]);
    f
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

    /// The default device's mix rate and layout (`GetMixFormat`), on a thread with COM of its
    /// own.
    pub unsafe fn mix_format() -> Option<(u32, Layout)> {
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
        let mix = if r == 0 && !fmt.is_null() {
            let rate = u32::from_le_bytes([*fmt.add(4), *fmt.add(5), *fmt.add(6), *fmt.add(7)]);
            layout_of(&format_bytes(fmt)).map(|l| (rate, l))
        } else {
            None
        };
        if !fmt.is_null() {
            CoTaskMemFree(fmt as *mut c_void);
        }
        slot::<Fn0>(client, 2 * w)(client);
        mix
    }

    /// A `WAVEFORMATEX` and the `cbSize` bytes after it (a PCM one has none).
    unsafe fn format_bytes(fmt: *const u8) -> Vec<u8> {
        let tag = u16::from_le_bytes([*fmt, *fmt.add(1)]);
        let extra = if tag == 1 {
            0
        } else {
            usize::from(u16::from_le_bytes([*fmt.add(16), *fmt.add(17)]))
        };
        std::slice::from_raw_parts(fmt, 18 + extra).to_vec()
    }

    /// The device's client, render client, event, buffer size, rate and layout.
    struct Device {
        client: *mut Obj,
        render: *mut Obj,
        event: *mut c_void,
        frames: u32,
        rate: u32,
        layout: Layout,
    }

    unsafe fn open(want: Option<Layout>) -> Result<Device, Refused> {
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
            return Err(Refused::NoDevice);
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
            return Err(Refused::NoDevice);
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
            return Err(Refused::NoDevice);
        }
        let mut fmt: *mut u8 = std::ptr::null_mut();
        if slot::<Fn1>(client, 8 * w)(client, &mut fmt as *mut _ as usize) != 0 || fmt.is_null() {
            slot::<Fn0>(client, 2 * w)(client);
            return Err(Refused::NoDevice);
        }
        let tag = u16::from_le_bytes([*fmt, *fmt.add(1)]);
        let rate = u32::from_le_bytes([*fmt.add(4), *fmt.add(5), *fmt.add(6), *fmt.add(7)]);
        let bits = u16::from_le_bytes([*fmt.add(14), *fmt.add(15)]);
        // IEEE float, directly (3) or through WAVE_FORMAT_EXTENSIBLE (0xfffe) with the float
        // subformat {00000003-0000-0010-8000-00aa00389b71}.
        let float = tag == 3 || (tag == 0xfffe && *fmt.add(24) == 3);
        let mix = match layout_of(&format_bytes(fmt)) {
            _ if !float || bits != 32 => Err(format!(
                "the sound device mixes {bits}-bit format {tag:#x}: not provided (32-bit float only)"
            )),
            Some(l) if l.provided() => Ok(l),
            Some(l) => Err(not_provided(l)),
            None => Err("the sound device's mix format cannot be read".to_string()),
        };
        let mix = match mix {
            Ok(l) => l,
            Err(why) => {
                CoTaskMemFree(fmt as *mut c_void);
                slot::<Fn0>(client, 2 * w)(client);
                return Err(Refused::Format(why));
            }
        };
        // Another layout than the mix format's (DirectSound's two channels on a larger
        // endpoint): asked with `IsFormatSupported` first, as DirectSound does; only `S_OK`
        // opens it.
        let (layout, other) = match want {
            Some(l) if l != mix => {
                let f = float_format(rate, l);
                let mut closest: *mut u8 = std::ptr::null_mut();
                let r = slot::<Fn3>(client, 7 * w)(
                    client,
                    0,
                    f.as_ptr() as usize,
                    &mut closest as *mut _ as usize,
                );
                if !closest.is_null() {
                    CoTaskMemFree(closest as *mut c_void);
                }
                if r != 0 {
                    CoTaskMemFree(fmt as *mut c_void);
                    slot::<Fn0>(client, 2 * w)(client);
                    return Err(Refused::Layout);
                }
                (l, Some(f))
            }
            _ => (mix, None),
        };
        // Shared, event driven, the engine's default period.
        let format = other.as_ref().map_or(fmt as *const u8, |f| f.as_ptr());
        let r = slot::<FnInit>(client, 3 * w)(client, 0, 0x4_0000, 0, 0, format, std::ptr::null());
        CoTaskMemFree(fmt as *mut c_void);
        if r != 0 {
            slot::<Fn0>(client, 2 * w)(client);
            return Err(Refused::NoDevice);
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
            CloseHandle(event);
            slot::<Fn0>(client, 2 * w)(client);
            return Err(Refused::NoDevice);
        }
        Ok(Device {
            client,
            render,
            event,
            frames,
            rate,
            layout,
        })
    }

    pub fn start(source: Arc<Mutex<dyn Source>>, want: Option<Layout>) -> Result<Output, Refused> {
        let stop = Arc::new(AtomicBool::new(false));
        let (tx, rx) = std::sync::mpsc::channel();
        let flag = stop.clone();
        let thread = std::thread::Builder::new()
            .name("nv sound out".into())
            .spawn(move || unsafe {
                let task = audio_priority();
                match open(want) {
                    Ok(d) => {
                        let _ = tx.send(Ok((d.rate, d.layout)));
                        run(&d, &source, &flag);
                    }
                    Err(why) => {
                        let _ = tx.send(Err(why));
                    }
                }
                // As dsound.dll does before its thread ends (`AvRevertMmThreadCharacteristics`).
                if !task.is_null() {
                    AvRevertMmThreadCharacteristics(task);
                }
            })
            .map_err(|_| Refused::NoDevice)?;
        let (rate, layout) = rx.recv().map_err(|_| Refused::NoDevice)??;
        Ok(Output {
            stop,
            thread: Some(thread),
            rate,
            layout,
        })
    }

    unsafe fn run(d: &Device, source: &Arc<Mutex<dyn Source>>, stop: &AtomicBool) {
        let w = std::mem::size_of::<usize>();
        let channels = d.layout.channels;
        let mut scratch = vec![0f32; d.frames as usize * usize::from(channels)];
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
            let n = free * usize::from(channels);
            let out = &mut scratch[..n];
            out.fill(0.0);
            if let Ok(mut s) = source.lock() {
                s.render(d.rate, channels, out);
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
