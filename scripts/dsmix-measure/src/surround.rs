//! `DSMIX_SURROUND`: what DirectSound hands the Windows audio engine for each kind of buffer the
//! game makes, on whatever channel layout the default device gives this process, taken in this
//! process (no loopback): `IAudioClient` (`Initialize`, `IsFormatSupported`, `GetMixFormat`,
//! `GetService`, and `IAudioClient3::InitializeSharedAudioStream` when it shares the vtable) and
//! `IAudioRenderClient` (`GetBuffer`, `ReleaseBuffer`) are wrapped before `DirectSoundCreate8`,
//! so the format DirectSound asks for and every packet it releases are kept.
//!
//! The device's layout depends on the process: this machine gives an 8-channel mix format
//! (mask 0x63F) to a program named `FalloutNV.exe` and 2 channels to any other name, so the same
//! program is run under both names (a copy in its own scratch folder).
//!
//! Writes into `<out-dir>`:
//! - `device.tsv`: the default render endpoint (id, name, physical speakers, the engine's device
//!   format, the mix format for the console and multimedia roles), every wrapped `IAudioClient`
//!   call DirectSound made with its format and answer, `GetCaps` (the 0x60-byte `DSCAPS`),
//!   `GetSpeakerConfig`, the primary buffer's caps and format, the listener's defaults.
//! - `cases.tsv`: one line per case, its settings, the calls' answers, the buffer's
//!   `CreateSoundBuffer` answer and caps, the frames captured and their channels.
//! - `cases.txt`: the same cases for `DSMIX_REPORT`, tab separated: name, kind, source channels,
//!   rate, bits, frames, seed, device channels.
//! - `<case>.f32`: DirectSound's packets as released (interleaved, the device's channels).
//! - `<case>.times`: each packet (microseconds from `Play`, frames) and for a moving source each
//!   `SetPosition` (`pos` microseconds, step, x y z).
//!
//! `DSMIX_REPORT=1 dsmix-measure <dir>` then reads them back (see [`report`]).
//!
//! DirectSound mixes two channels on an endpoint of more until the first 3D buffer is made, then
//! the endpoint's layout for good, closing its stream and opening another: every render client's
//! packets are kept (each packet's frame size in `<case>.times`). With `DSMIX_LOCKFIRST=1` a 3D
//! buffer is made and released before the cases, so every case is on the endpoint's layout.
use super::{guid, qi, qpc, signal_seeded, slot, Fn0, Fn1, Fn2, Fn3, Fn4, Fn7, FnF1, FnF3, Obj};
use std::ffi::c_void;
use std::io::Write;

#[link(name = "kernel32")]
extern "system" {
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
    fn Sleep(ms: u32);
}
#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(reserved: *mut c_void, flags: u32) -> u32;
    fn CoCreateInstance(clsid: *const [u8; 16], outer: *mut c_void, context: u32, iid: *const [u8; 16], out: *mut *mut Obj) -> u32;
}
#[link(name = "dsound")]
extern "system" {
    fn DirectSoundCreate8(device: *const u8, out: *mut *mut Obj, outer: *mut c_void) -> u32;
}
#[link(name = "user32")]
extern "system" {
    fn RegisterClassA(class: *const WndClassA) -> u16;
    #[allow(clippy::too_many_arguments)]
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

unsafe extern "system" fn wndproc(w: *mut c_void, m: u32, wp: usize, lp: isize) -> isize {
    DefWindowProcA(w, m, wp, lp)
}

/// A window of the program's own, for `SetCooperativeLevel` (the game passes its main window).
unsafe fn window() -> *mut c_void {
    let name = b"dsmix-surround\0";
    let class = WndClassA {
        style: 0,
        procedure: wndproc,
        class_extra: 0,
        window_extra: 0,
        instance: std::ptr::null_mut(),
        icon: std::ptr::null_mut(),
        cursor: std::ptr::null_mut(),
        background: std::ptr::null_mut(),
        menu_name: std::ptr::null(),
        class_name: name.as_ptr(),
    };
    RegisterClassA(&class);
    CreateWindowExA(0, name.as_ptr(), name.as_ptr(), 0, 0, 0, 64, 64, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut())
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// A `WAVEFORMATEX` (and what follows it, `cbSize` bytes) as bytes.
unsafe fn format_bytes(fmt: *const u8) -> Vec<u8> {
    if fmt.is_null() {
        return Vec::new();
    }
    let tag = u16::from_le_bytes([*fmt, *fmt.add(1)]);
    let extra = if tag == 1 { 0 } else { u16::from_le_bytes([*fmt.add(16), *fmt.add(17)]) as usize };
    std::slice::from_raw_parts(fmt, 18 + extra).to_vec()
}

/// The wrapped `IAudioClient` and `IAudioRenderClient` slots.
mod hooks {
    use super::super::{guid, qpc, slot, Fn2, Fn4, Obj};
    use super::format_bytes;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    #[link(name = "kernel32")]
    extern "system" {
        fn VirtualProtect(at: *mut u8, size: usize, protect: u32, old: *mut u32) -> i32;
    }

    /// Every wrapped `IAudioClient` call, as text, in order.
    pub static CALLS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    /// Audio client -> its frame size (from the format it was initialized with).
    static CLIENTS: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());
    /// Render client -> frame size.
    static RENDERS: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());
    /// The packets released by DirectSound's render clients (one at a time: the stream is
    /// closed and opened again when the device's layout is locked), their clocks, frames and
    /// frame sizes.
    static TAKEN: Mutex<Vec<u8>> = Mutex::new(Vec::new());
    static TIMES: Mutex<Vec<(i64, u32, usize)>> = Mutex::new(Vec::new());
    /// The render client that released last; a change is logged.
    static CURRENT: AtomicUsize = AtomicUsize::new(0);
    pub static OTHERS: AtomicUsize = AtomicUsize::new(0);
    /// The frame size used when a render client's format is not known.
    static FALLBACK: AtomicUsize = AtomicUsize::new(0);
    /// The data pointer of the last `GetBuffer`, per render client (one at a time).
    static LAST: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());
    static ORIG: Mutex<[usize; 8]> = Mutex::new([0; 8]);
    const INIT: usize = 0;
    const SUPPORTED: usize = 1;
    const MIX: usize = 2;
    const SERVICE: usize = 3;
    const SHARED: usize = 4;
    const GET: usize = 5;
    const RELEASE: usize = 6;

    fn orig(k: usize) -> usize {
        ORIG.lock().unwrap()[k]
    }
    fn log(s: String) {
        CALLS.lock().unwrap().push(s);
    }
    fn hexs(b: &[u8]) -> String {
        b.iter().map(|x| format!("{x:02x}")).collect()
    }
    fn frame_of(fmt: &[u8]) -> usize {
        if fmt.len() >= 14 {
            u16::from_le_bytes([fmt[12], fmt[13]]) as usize
        } else {
            0
        }
    }
    fn set(map: &Mutex<Vec<(usize, usize)>>, k: usize, v: usize) {
        let mut m = map.lock().unwrap();
        m.retain(|&(a, _)| a != k);
        m.push((k, v));
    }
    fn get(map: &Mutex<Vec<(usize, usize)>>, k: usize) -> Option<usize> {
        map.lock().unwrap().iter().find(|&&(a, _)| a == k).map(|&(_, v)| v)
    }

    type Init = unsafe extern "system" fn(*mut Obj, u32, u32, i64, i64, *const u8, *const u8) -> u32;
    type Supported = unsafe extern "system" fn(*mut Obj, u32, *const u8, *mut *mut u8) -> u32;
    type Mix = unsafe extern "system" fn(*mut Obj, *mut *mut u8) -> u32;
    type Service = unsafe extern "system" fn(*mut Obj, *const [u8; 16], *mut *mut Obj) -> u32;
    type Shared = unsafe extern "system" fn(*mut Obj, u32, u32, *const u8, *const u8) -> u32;
    type Get = unsafe extern "system" fn(*mut Obj, u32, *mut *mut u8) -> u32;
    type Release = unsafe extern "system" fn(*mut Obj, u32, u32) -> u32;

    unsafe extern "system" fn init(this: *mut Obj, share: u32, flags: u32, duration: i64, period: i64, fmt: *const u8, session: *const u8) -> u32 {
        let f: Init = std::mem::transmute(orig(INIT));
        let r = f(this, share, flags, duration, period, fmt, session);
        let b = format_bytes(fmt);
        set(&CLIENTS, this as usize, frame_of(&b));
        log(format!("Initialize\tclient {:x} share {share} flags {flags:x} duration {duration} period {period}\t{r:x}\t{}", this as usize, hexs(&b)));
        r
    }
    unsafe extern "system" fn supported(this: *mut Obj, share: u32, fmt: *const u8, closest: *mut *mut u8) -> u32 {
        let f: Supported = std::mem::transmute(orig(SUPPORTED));
        let r = f(this, share, fmt, closest);
        let c = if closest.is_null() || (*closest).is_null() { String::new() } else { hexs(&format_bytes(*closest)) };
        log(format!("IsFormatSupported\tclient {:x} share {share} {}\t{r:x}\t{c}", this as usize, hexs(&format_bytes(fmt))));
        r
    }
    unsafe extern "system" fn mix(this: *mut Obj, out: *mut *mut u8) -> u32 {
        let f: Mix = std::mem::transmute(orig(MIX));
        let r = f(this, out);
        let b = if r == 0 && !out.is_null() { hexs(&format_bytes(*out)) } else { String::new() };
        log(format!("GetMixFormat\tclient {:x}\t{r:x}\t{b}", this as usize));
        r
    }
    unsafe extern "system" fn service(this: *mut Obj, iid: *const [u8; 16], out: *mut *mut Obj) -> u32 {
        let f: Service = std::mem::transmute(orig(SERVICE));
        let r = f(this, iid, out);
        let render = guid("f294acfc-3146-4483-a7bf-addca7c260e2");
        let is_render = !iid.is_null() && *iid == render;
        if r == 0 && is_render && !out.is_null() {
            let frame = get(&CLIENTS, this as usize).unwrap_or(0);
            set(&RENDERS, *out as usize, frame);
        }
        log(format!(
            "GetService\tclient {:x} {}\t{r:x}\t{:x}",
            this as usize,
            if iid.is_null() { String::new() } else { hexs(&*iid) },
            if out.is_null() { 0 } else { *out as usize }
        ));
        r
    }
    unsafe extern "system" fn shared(this: *mut Obj, flags: u32, period: u32, fmt: *const u8, session: *const u8) -> u32 {
        let f: Shared = std::mem::transmute(orig(SHARED));
        let r = f(this, flags, period, fmt, session);
        let b = format_bytes(fmt);
        set(&CLIENTS, this as usize, frame_of(&b));
        log(format!("InitializeSharedAudioStream\tclient {:x} flags {flags:x} period {period}\t{r:x}\t{}", this as usize, hexs(&b)));
        r
    }
    unsafe extern "system" fn get_buffer(this: *mut Obj, frames: u32, data: *mut *mut u8) -> u32 {
        let f: Get = std::mem::transmute(orig(GET));
        let r = f(this, frames, data);
        if r == 0 && !data.is_null() {
            set(&LAST, this as usize, *data as usize);
        }
        r
    }
    unsafe extern "system" fn release(this: *mut Obj, frames: u32, flags: u32) -> u32 {
        if let Some(at) = get(&LAST, this as usize) {
            let frame = get(&RENDERS, this as usize).filter(|&f| f != 0).unwrap_or_else(|| FALLBACK.load(Ordering::SeqCst));
            let before = CURRENT.swap(this as usize, Ordering::SeqCst);
            if before != this as usize {
                OTHERS.fetch_add(1, Ordering::SeqCst);
                log(format!("render client {:x} releases its first packet\tframe bytes {frame}\tqpc {}", this as usize, qpc()));
            }
            TIMES.lock().unwrap().push((qpc(), frames, frame));
            let n = frames as usize * frame;
            let mut t = TAKEN.lock().unwrap();
            if flags & 2 != 0 {
                t.extend(std::iter::repeat(0u8).take(n));
            } else {
                t.extend_from_slice(std::slice::from_raw_parts(at as *const u8, n));
            }
        }
        let f: Release = std::mem::transmute(orig(RELEASE));
        f(this, frames, flags)
    }

    unsafe fn patch(vt: *mut usize, index: usize, k: usize, to: usize) {
        let mut old = 0u32;
        assert!(VirtualProtect(vt.add(index) as *mut u8, 4, 0x04, &mut old) != 0, "VirtualProtect");
        ORIG.lock().unwrap()[k] = *vt.add(index);
        *vt.add(index) = to;
    }

    /// Wraps the slots, through an audio client and a render client of `dev` initialized with
    /// the mix format `fmt` (whose frame size is the fallback). Answers what it found out.
    pub unsafe fn install(dev: *mut Obj, fmt: *const u8) -> String {
        FALLBACK.store(frame_of(&format_bytes(fmt)), Ordering::SeqCst);
        let mut client: *mut Obj = std::ptr::null_mut();
        let iid_client = guid("1cb9ad4c-dbfa-4c32-b178-c2f568a703b2");
        let r = slot::<Fn4>(dev, 0xc)(dev, iid_client.as_ptr() as usize, 0x17, 0, &mut client as *mut _ as usize);
        assert_eq!(r, 0, "Activate (render)");
        let r = slot::<super::super::FnInit>(client, 0xc)(client, 0, 0, 10_000_000, 0, fmt, std::ptr::null());
        assert_eq!(r, 0, "IAudioClient::Initialize render");
        let mut render: *mut Obj = std::ptr::null_mut();
        let iid = guid("f294acfc-3146-4483-a7bf-addca7c260e2");
        let r = slot::<Fn2>(client, 0x38)(client, iid.as_ptr() as usize, &mut render as *mut _ as usize);
        assert_eq!(r, 0, "IAudioRenderClient");
        // IAudioClient3 on the same object (then its vtable holds slot 20 too).
        let mut c3: *mut Obj = std::ptr::null_mut();
        let iid3 = guid("7ed4ee07-8e67-4cd4-8c1a-2b7a5987ad42");
        let q3 = slot::<Fn2>(client, 0)(client, iid3.as_ptr() as usize, &mut c3 as *mut _ as usize);
        let same3 = q3 == 0 && c3 == client;
        let vt = (*client).vtable as *mut usize;
        patch(vt, 3, INIT, init as *const () as usize);
        patch(vt, 7, SUPPORTED, supported as *const () as usize);
        patch(vt, 8, MIX, mix as *const () as usize);
        patch(vt, 14, SERVICE, service as *const () as usize);
        if same3 {
            patch(vt, 20, SHARED, shared as *const () as usize);
        }
        let rv = (*render).vtable as *mut usize;
        patch(rv, 3, GET, get_buffer as *const () as usize);
        patch(rv, 4, RELEASE, release as *const () as usize);
        format!(
            "hooks\tclient vtable {:x} render vtable {:x} IAudioClient3 {q3:x} {}\tfallback frame {}",
            vt as usize,
            rv as usize,
            if same3 { "same object (InitializeSharedAudioStream wrapped)" } else { "apart (not wrapped)" },
            FALLBACK.load(Ordering::SeqCst)
        )
    }
    /// What the kept render client released since the last call.
    pub fn take(out: &mut Vec<u8>) {
        out.append(&mut TAKEN.lock().unwrap());
    }
    pub fn take_times(out: &mut Vec<(i64, u32, usize)>) {
        out.append(&mut TIMES.lock().unwrap());
    }
    /// The frame size of the packets of the render client that released last.
    pub fn frame() -> usize {
        let current = CURRENT.load(Ordering::SeqCst);
        get(&RENDERS, current).filter(|&f| f != 0).unwrap_or_else(|| FALLBACK.load(Ordering::SeqCst))
    }
}

/// A 3D setting: position, velocity, minimum and maximum distance; the listener's rolloff and
/// Doppler factors.
#[derive(Clone, Copy, Debug)]
struct ThreeD {
    pos: [f32; 3],
    vel: [f32; 3],
    min: f32,
    max: f32,
    rolloff: f32,
    doppler: f32,
}

/// A source moving round the listener: radius, height, start angle and degrees per 10 ms step
/// (azimuth from +z towards +x), set with `DS3D_DEFERRED` and committed as the game does.
#[derive(Clone, Copy, Debug)]
struct Moving {
    radius: f32,
    height: f32,
    start: f32,
    step: f32,
}

#[derive(Clone, Debug)]
struct Case {
    name: String,
    channels: u16,
    rate: u32,
    bits: u16,
    flags: u32,
    /// `DS3DALG_HRTF_LIGHT` (the game asks for it when the caps report hardware buffers).
    hrtf: bool,
    volume: i32,
    pan: i32,
    frequency: u32,
    three_d: Option<ThreeD>,
    distortion: Option<[f32; 5]>,
    /// The listener's front and top (else +z and +y).
    orientation: Option<([f32; 3], [f32; 3])>,
    /// A frequency set while it plays: after this many 10 ms steps, this frequency.
    later: Option<(u32, u32)>,
    moving: Option<Moving>,
    frames: usize,
    steps: u32,
    looping: bool,
    seed: u32,
    /// A second buffer played with this one (a 2D mono buffer of seed 7 at pan 0, started
    /// first): the sum's order on this layout.
    with_2d: bool,
    /// Two more 2D mono 48 kHz buffers (seeds 7 and 9, pans -2000 and 3000), made after this
    /// one (the third made while two exist) and started before it.
    three: bool,
    /// A volume or pan set while it plays: after this many 10 ms steps, this value (the ramp).
    later_volume: Option<(u32, i32)>,
    later_pan: Option<(u32, i32)>,
    /// After this many steps a 3D mono buffer (seed 11, 3 units to the right) is made and
    /// started while this one plays: on a device not locked yet, the moment of the lock.
    lock_at: Option<u32>,
}

/// The game's flag sets (`00aeb270`): 2D `0x2e0` (`CTRLFREQUENCY | CTRLPAN | CTRLVOLUME |
/// CTRLFX`), 3D `0x302b0` (`CTRL3D | CTRLFREQUENCY | CTRLVOLUME | CTRLFX | GETCURRENTPOSITION2 |
/// MUTE3DATMAXDISTANCE`), `CTRLFX` left out under 151 ms; `DSBCAPS_GLOBALFOCUS` (0x8000) added
/// because the measuring window never has the focus.
const FLAGS_2D: u32 = 0x2e0 | 0x8000;
const FLAGS_2D_NOFX: u32 = 0xe0 | 0x8000;
const FLAGS_3D: u32 = 0x302b0 | 0x8000;
const FLAGS_3D_NOFX: u32 = 0x300b0 | 0x8000;
const ALG_HRTF_LIGHT: [u32; 4] = [0xc241_3342, 0x11d2_1c1b, 0xc000_f594, 0xca8a_c24f];
const GAME_FX: [f32; 5] = [-18.2, 0.0, 5000.0, 1135.0, 5000.0];

fn game3d(pos: [f32; 3]) -> ThreeD {
    ThreeD { pos, vel: [0.0; 3], min: 1.0e9, max: 1.0e9, rolloff: 1.0, doppler: 1.0 }
}
fn near3d(pos: [f32; 3], min: f32) -> ThreeD {
    ThreeD { pos, vel: [0.0; 3], min, max: 1.0e9, rolloff: 1.0, doppler: 1.0 }
}
fn at(az: f32, r: f32, y: f32) -> [f32; 3] {
    let a = az.to_radians();
    [r * a.sin(), y, r * a.cos()]
}

fn cases() -> Vec<Case> {
    let base = Case {
        name: String::new(),
        channels: 1,
        rate: 48000,
        bits: 16,
        flags: FLAGS_2D,
        hrtf: false,
        volume: 0,
        pan: 0,
        frequency: 0,
        three_d: None,
        distortion: None,
        orientation: None,
        later: None,
        moving: None,
        frames: 12000,
        steps: 45,
        looping: false,
        seed: 0x1234_5678,
        with_2d: false,
        three: false,
        later_volume: None,
        later_pan: None,
        lock_at: None,
    };
    let mut v: Vec<Case> = Vec::new();
    let mut add = |name: String, f: &dyn Fn(&mut Case)| {
        let mut c = Case { name, ..base.clone() };
        f(&mut c);
        if c.frames == 12000 {
            c.frames = (c.rate / 4) as usize;
        }
        v.push(c);
    };
    // 2D.
    add("d2_mono48".into(), &|_| {});
    add("d2_mono48_nofx".into(), &|c| c.flags = FLAGS_2D_NOFX);
    add("d2_stereo48".into(), &|c| c.channels = 2);
    add("d2_mono48_8bit".into(), &|c| c.bits = 8);
    add("d2_stereo48_8bit".into(), &|c| {
        c.channels = 2;
        c.bits = 8
    });
    for vol in [-1, -100, -600, -2000, -6000] {
        add(format!("d2_vol_{vol}"), &move |c| c.volume = vol);
    }
    for pan in [-10000, -5000, -2000, -1000, -100, 100, 1000, 2000, 5000, 10000] {
        add(format!("d2_pan_{pan}"), &move |c| c.pan = pan);
    }
    add("d2_vol_m300_pan_m700".into(), &|c| {
        c.volume = -300;
        c.pan = -700
    });
    for pan in [-3000, 1000, 10000] {
        add(format!("d2_stereo48_pan_{pan}"), &move |c| {
            c.channels = 2;
            c.pan = pan
        });
    }
    add("d2_stereo48_vol_m600".into(), &|c| {
        c.channels = 2;
        c.volume = -600
    });
    for rate in [11025u32, 22050, 32000, 44100] {
        add(format!("d2_mono{rate}"), &move |c| c.rate = rate);
    }
    add("d2_stereo44100".into(), &|c| {
        c.channels = 2;
        c.rate = 44100
    });
    add("d2_mono44100_pan_m3000".into(), &|c| {
        c.rate = 44100;
        c.pan = -3000
    });
    add("d2_fx_game_mono48".into(), &|c| c.distortion = Some(GAME_FX));
    add("d2_fx_game_stereo48".into(), &|c| {
        c.channels = 2;
        c.distortion = Some(GAME_FX)
    });
    add("d2_fx_game_mono48_pan_p3000".into(), &|c| {
        c.pan = 3000;
        c.distortion = Some(GAME_FX)
    });
    // A volume and a pan changed while playing (DirectSound's 4 ms ramp to the new gains),
    // mono and stereo, at 48 kHz (one second, looping).
    for (n, ch, vol, pan) in [
        ("r2_mono_vol_later_m600", 1u16, Some(-600), None),
        ("r2_mono_pan_later_m3000", 1, None, Some(-3000)),
        ("r2_stereo_vol_later_m600", 2, Some(-600), None),
        ("r2_stereo_pan_later_p2000", 2, None, Some(2000)),
    ] {
        add(n.into(), &move |c| {
            c.channels = ch;
            c.frames = 48000;
            c.looping = true;
            c.later_volume = vol.map(|v| (12, v));
            c.later_pan = pan.map(|p| (12, p));
        });
    }
    // A 3D buffer made while a 2D one plays: on a device whose layout is not locked yet (the
    // first 3D buffer of the run), the lock.
    add("k_lock_2d_then_3d".into(), &|c| {
        c.frames = 48000;
        c.looping = true;
        c.lock_at = Some(12);
    });
    // 3D, the game's distances (minimum and maximum 1e9, rolloff 1): azimuth every 5 degrees at
    // 3 units (outside the near zone) and every 45 at 1 unit (inside it).
    for deg in (0..360).step_by(5) {
        add(format!("g3_az{deg}_r3"), &move |c| {
            c.flags = FLAGS_3D;
            c.three_d = Some(game3d(at(deg as f32, 3.0, 0.0)));
        });
    }
    for deg in (0..360).step_by(45) {
        add(format!("g3_az{deg}_r1"), &move |c| {
            c.flags = FLAGS_3D;
            c.three_d = Some(game3d(at(deg as f32, 1.0, 0.0)));
        });
    }
    for (n, pos) in [
        ("above", [0.0f32, 3.0, 0.0]),
        ("below", [0.0, -3.0, 0.0]),
        ("above_front", [0.0, 2.0, 2.0]),
        ("above_left", [-2.0, 2.0, 0.0]),
        ("below_back", [0.0, -2.0, -2.0]),
        ("below_right_back", [1.5, -2.0, -1.5]),
        ("at_listener", [0.0, 0.0, 0.0]),
        ("near_front", [0.0, 0.0, 0.5]),
        ("near_right", [0.5, 0.0, 0.0]),
        ("near_back_left", [-0.3, 0.0, -0.4]),
        ("box_edge", [2.49, 0.0, 0.1]),
        ("far", [-30.0, 2.0, 40.0]),
        ("far_back", [100.0, 0.0, -300.0]),
    ] {
        add(format!("g3_{n}"), &move |c| {
            c.flags = FLAGS_3D;
            c.three_d = Some(game3d(pos));
        });
    }
    // 3D with the HRTF algorithm (asked by the game when the caps report hardware buffers).
    for deg in [0u32, 30, 60, 90, 135, 180, 225, 270, 315] {
        add(format!("h3_az{deg}_r3"), &move |c| {
            c.flags = FLAGS_3D;
            c.hrtf = true;
            c.three_d = Some(game3d(at(deg as f32, 3.0, 0.0)));
        });
    }
    add("h3_above".into(), &|c| {
        c.flags = FLAGS_3D;
        c.hrtf = true;
        c.three_d = Some(game3d([0.0, 3.0, 0.0]));
    });
    add("h3_az45_r1".into(), &|c| {
        c.flags = FLAGS_3D;
        c.hrtf = true;
        c.three_d = Some(game3d(at(45.0, 1.0, 0.0)));
    });
    // 3D without CTRLFX, with the distortion, with a volume, at other rates and 8 bits.
    add("g3_nofx_az60_r3".into(), &|c| {
        c.flags = FLAGS_3D_NOFX;
        c.three_d = Some(game3d(at(60.0, 3.0, 0.0)));
    });
    add("g3_fx_game_az100_r3".into(), &|c| {
        c.flags = FLAGS_3D;
        c.distortion = Some(GAME_FX);
        c.three_d = Some(game3d(at(100.0, 3.0, 0.0)));
    });
    add("g3_vol_m600_az200_r3".into(), &|c| {
        c.flags = FLAGS_3D;
        c.volume = -600;
        c.three_d = Some(game3d(at(200.0, 3.0, 0.0)));
    });
    for rate in [22050u32, 32000, 44100] {
        add(format!("g3_rate{rate}_az60_r3"), &move |c| {
            c.flags = FLAGS_3D;
            c.rate = rate;
            c.three_d = Some(game3d(at(60.0, 3.0, 0.0)));
        });
    }
    add("g3_8bit_az250_r3".into(), &|c| {
        c.flags = FLAGS_3D;
        c.bits = 8;
        c.three_d = Some(game3d(at(250.0, 3.0, 0.0)));
    });
    // Turned listeners (front, top), the source 3 units ahead in the world.
    for (n, front, top) in [
        ("yaw30", [0.5f32, 0.0, 0.8660254], [0.0f32, 1.0, 0.0]),
        ("yaw100", [0.98480775, 0.0, -0.17364818], [0.0, 1.0, 0.0]),
        ("yaw33_7", [0.55484474, 0.0, 0.8319391], [0.0, 1.0, 0.0]),
        ("yawm135", [-0.70710678, 0.0, -0.70710678], [0.0, 1.0, 0.0]),
        ("pitch20", [0.0, 0.34202015, 0.9396926], [0.0, 0.9396926, -0.34202015]),
        ("roll25", [0.0, 0.0, 1.0], [-0.42261827, 0.90630779, 0.0]),
        ("mixed", [0.3, 0.2, 0.9], [0.1, 1.0, -0.2]),
    ] {
        add(format!("t3_{n}_ahead3"), &move |c| {
            c.flags = FLAGS_3D;
            c.orientation = Some((front, top));
            c.three_d = Some(game3d([0.0, 0.0, 3.0]));
        });
    }
    add("t3_pitch20_above3".into(), &|c| {
        c.flags = FLAGS_3D;
        c.orientation = Some(([0.0, 0.34202015, 0.9396926], [0.0, 0.9396926, -0.34202015]));
        c.three_d = Some(game3d([0.0, 3.0, 0.0]));
    });
    // 3D with a minimum distance of 1 (distance attenuation), inside the minimum, rolloff 2, a
    // maximum of 4 (`MUTE3DATMAXDISTANCE` is set).
    for (n, pos, min) in [
        ("front_0_5", [0.0f32, 0.0, 0.5], 1.0f32),
        ("front_1", [0.0, 0.0, 1.0], 1.0),
        ("front_2", [0.0, 0.0, 2.0], 1.0),
        ("front_10", [0.0, 0.0, 10.0], 1.0),
        ("front_100", [0.0, 0.0, 100.0], 1.0),
        ("right_10", [10.0, 0.0, 0.0], 1.0),
        ("back_left_7", [-4.9497476, 0.0, -4.9497476], 1.0),
        ("inside_min5", [0.0, 0.0, 2.0], 5.0),
        ("inside_min5_left", [-2.0, 0.0, 0.0], 5.0),
    ] {
        add(format!("n3_{n}"), &move |c| {
            c.flags = FLAGS_3D;
            c.three_d = Some(near3d(pos, min));
        });
    }
    add("n3_front_10_rolloff2".into(), &|c| {
        c.flags = FLAGS_3D;
        c.three_d = Some(ThreeD { rolloff: 2.0, ..near3d([0.0, 0.0, 10.0], 1.0) });
    });
    add("n3_front_10_max4".into(), &|c| {
        c.flags = FLAGS_3D;
        c.three_d = Some(ThreeD { max: 4.0, ..near3d([0.0, 0.0, 10.0], 1.0) });
    });
    // A source moving round the listener (the position set every 10 ms, deferred and
    // committed): 4 degrees a step at 3 units, 6 a step at 1 unit, 1 a step across the front.
    for (n, m) in [
        ("m3_round_r3", Moving { radius: 3.0, height: 0.0, start: 0.0, step: 4.0 }),
        ("m3_round_r1", Moving { radius: 1.0, height: 0.0, start: 0.0, step: 6.0 }),
        ("m3_slow_front_r3", Moving { radius: 3.0, height: 0.0, start: -45.0, step: 1.0 }),
    ] {
        add(n.into(), &move |c| {
            c.flags = FLAGS_3D;
            c.frames = 48000;
            c.steps = 100;
            c.looping = true;
            c.three_d = Some(game3d(at(m.start, m.radius, m.height)));
            c.moving = Some(m);
        });
    }
    // Two voices: a 2D one started first, then this one.
    add("w2_2d_and_3d_az90".into(), &|c| {
        c.flags = FLAGS_3D;
        c.with_2d = true;
        c.three_d = Some(game3d(at(90.0, 3.0, 0.0)));
    });
    add("w2_2d_and_2d_pan_m2000".into(), &|c| {
        c.pan = -2000;
        c.with_2d = true;
    });
    // Frequency: changes of 1/500 or less (the first converter stays), set before and while
    // playing.
    add("f_small_44100_44150".into(), &|c| {
        c.rate = 44100;
        c.frequency = 44150
    });
    add("f_small_later_48000_48050".into(), &|c| {
        c.later = Some((12, 48050));
    });
    add("f3_small_later_48000_48050_az60".into(), &|c| {
        c.flags = FLAGS_3D;
        c.later = Some((12, 48050));
        c.three_d = Some(game3d(at(60.0, 3.0, 0.0)));
    });
    // Three 2D buffers at 48 kHz, the third made while two exist (the DLL's buffer count rule
    // for the second converter), then a 3D one at 48 kHz after them.
    add("w3_three_2d".into(), &|c| {
        c.pan = 1000;
        c.three = true;
    });
    add("w3_after_g3_az60_r3".into(), &|c| {
        c.flags = FLAGS_3D;
        c.three_d = Some(game3d(at(60.0, 3.0, 0.0)));
    });
    add("w3_after_mono48".into(), &|_| {});
    // The rest turn on DirectSound's second converter for the device, for good: last.
    add("f_big_48000_to_24000".into(), &|c| c.frequency = 24000);
    add("f_big_later_22050_30000".into(), &|c| {
        c.rate = 22050;
        c.later = Some((12, 30000));
    });
    add("f3_doppler_az30".into(), &|c| {
        c.flags = FLAGS_3D;
        c.three_d = Some(ThreeD { vel: [0.0, 0.0, -50.0], ..game3d(at(30.0, 5.0, 0.0)) });
    });
    add("a_mono48".into(), &|_| {});
    add("a_stereo44100".into(), &|c| {
        c.channels = 2;
        c.rate = 44100
    });
    add("a_g3_az60_r3".into(), &|c| {
        c.flags = FLAGS_3D;
        c.three_d = Some(game3d(at(60.0, 3.0, 0.0)));
    });
    add("a_g3_rate22050_az200_r3".into(), &|c| {
        c.flags = FLAGS_3D;
        c.rate = 22050;
        c.three_d = Some(game3d(at(200.0, 3.0, 0.0)));
    });
    if let Ok(only) = std::env::var("DSMIX_ONLY") {
        v.retain(|c| c.name.starts_with(only.as_str()));
    }
    v
}

/// The measured volume law (`platform::mixer::gain`, docs/SOUND.md): millibels to the `f32`
/// multiplier DirectSound applies.
fn law(millibels: i32) -> f32 {
    if millibels >= 0 {
        return 1.0;
    }
    if millibels <= -10000 {
        return 0.0;
    }
    let tenth = ((millibels + 10) as f64 / 10.0).round() * 10.0;
    let d = tenth.min(-10.0);
    let k = (65536.0 * 10f64.powf(d / 2000.0)).floor() as f32;
    k * (1.0f32 / 65535.0)
}

/// What the stereo 3D stage gives a buffer before its pan (`platform::ds3d`, the listener at
/// the origin): the volume plus `trunc(602.059991328 * log2(f)) - 500` for the distance factor
/// `f`, -10000 muted beyond the maximum distance. For a 2D buffer, the volume.
fn base_mb(c: &Case) -> i32 {
    let Some(t) = c.three_d else {
        return c.volume;
    };
    let [x, y, z] = t.pos.map(f64::from);
    let d = ((y * y + x * x) + z * z).sqrt() as f32;
    let (d, beyond) = if d < t.max { (d, false) } else { (t.max, true) };
    let f = if d > t.min && t.rolloff > 0.0 { t.min / ((d - t.min) * t.rolloff + t.min) } else { 1.0 };
    let mb = if beyond && c.flags & 0x20000 != 0 || !(f64::from(f) > 0.0) { -10000 } else { (602.059_991_328f64 * f64::from(f).log2()) as i32 - 500 };
    (c.volume + mb).max(-10000)
}

/// The millibels `p` (highest first) for which `law(p) * law(base)` is `g` exactly, and those
/// for which `law(p)` alone is `g`.
fn decompose(g: f32, base: i32) -> String {
    let b = law(base);
    let pair = (-10000..=0).rev().find(|&p| law(p) * b == g);
    let single = (-10000..=0).rev().find(|&p| law(p) == g);
    format!(
        "mb[{}]{}",
        pair.map_or("?".to_string(), |p| format!("{p}+{base}")),
        single.map_or(String::new(), |s| format!(" alone {s}"))
    )
}

/// What a case is for [`report`]: `exact` (48 kHz, nothing moving or changing: per-channel
/// gains sample for sample), `moving`, or `shape` (resampled, the effect, a change while
/// playing, two voices: levels and which channels carry copies of which).
fn kind(c: &Case) -> &'static str {
    if c.moving.is_some() {
        "moving"
    } else if c.rate == 48000
        && c.frequency == 0
        && c.later.is_none()
        && c.later_volume.is_none()
        && c.later_pan.is_none()
        && c.lock_at.is_none()
        && c.distortion.is_none()
        && !c.with_2d
        && !c.three
        && !c.name.starts_with("a_")
        && !c.name.starts_with("w3_after")
        && !c.name.starts_with("f3_doppler")
    {
        "exact"
    } else {
        "shape"
    }
}

unsafe fn property(store: *mut Obj, fmtid: &str, pid: u32) -> String {
    let mut key = [0u8; 20];
    key[..16].copy_from_slice(&guid(fmtid));
    key[16..].copy_from_slice(&pid.to_le_bytes());
    let mut pv = [0u32; 4];
    let r = slot::<Fn2>(store, 0x14)(store, key.as_ptr() as usize, pv.as_mut_ptr() as usize);
    if r != 0 {
        return format!("error {r:x}");
    }
    let vt = pv[0] & 0xffff;
    match vt {
        0 => "empty".into(),
        19 => format!("ui4 {:x}", pv[2]),
        31 => {
            let mut p = pv[2] as *const u16;
            let mut s = Vec::new();
            while !p.is_null() && *p != 0 {
                s.push(*p);
                p = p.add(1);
            }
            format!("text {}", String::from_utf16_lossy(&s))
        }
        65 => format!("blob {}", hex(std::slice::from_raw_parts(pv[3] as *const u8, pv[2] as usize))),
        _ => format!("vt {vt} {:08x} {:08x}", pv[2], pv[3]),
    }
}

/// The default render endpoint's description; answers the console device and its mix format.
unsafe fn device(o: &mut impl Write) -> (*mut Obj, Vec<u8>) {
    let mut en: *mut Obj = std::ptr::null_mut();
    let r = CoCreateInstance(&guid("bcde0395-e52f-467c-8e3d-c4579291692e"), std::ptr::null_mut(), 0x17, &guid("a95664d2-9614-4f35-a746-de8db63617e6"), &mut en);
    assert_eq!(r, 0, "MMDeviceEnumerator");
    let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
    writeln!(o, "process\t{exe}\t{} bit", std::mem::size_of::<usize>() * 8).unwrap();
    let mut console = (std::ptr::null_mut(), Vec::new());
    for (role, name) in [(0usize, "console"), (1, "multimedia"), (2, "communications")] {
        let mut dev: *mut Obj = std::ptr::null_mut();
        let r = slot::<Fn3>(en, 0x10)(en, 0, role, &mut dev as *mut _ as usize);
        if r != 0 {
            writeln!(o, "endpoint\t{name}\terror {r:x}").unwrap();
            continue;
        }
        let mut id: *const u16 = std::ptr::null();
        slot::<Fn1>(dev, 0x14)(dev, &mut id as *mut _ as usize);
        let mut s = Vec::new();
        let mut p = id;
        while !p.is_null() && *p != 0 {
            s.push(*p);
            p = p.add(1);
        }
        let mut store: *mut Obj = std::ptr::null_mut();
        slot::<Fn2>(dev, 0x10)(dev, 0, &mut store as *mut _ as usize);
        let (friendly, speakers, engine, form) = if store.is_null() {
            (String::new(), String::new(), String::new(), String::new())
        } else {
            (
                property(store, "a45c254e-df1c-4efd-8020-67d146a850e0", 14),
                property(store, "1da5d803-d492-4edd-8c23-e0c0ffee7f0e", 3),
                property(store, "f19f064d-082c-4e27-bc73-6882a1bb8e4c", 0),
                property(store, "1da5d803-d492-4edd-8c23-e0c0ffee7f0e", 0),
            )
        };
        let mut client: *mut Obj = std::ptr::null_mut();
        let iid_client = guid("1cb9ad4c-dbfa-4c32-b178-c2f568a703b2");
        let r = slot::<Fn4>(dev, 0xc)(dev, iid_client.as_ptr() as usize, 0x17, 0, &mut client as *mut _ as usize);
        let mut fmt: *const u8 = std::ptr::null();
        let m = if r == 0 { slot::<Fn1>(client, 0x20)(client, &mut fmt as *mut _ as usize) } else { r };
        let mix = format_bytes(fmt);
        let (mut period, mut minimum) = (0i64, 0i64);
        if r == 0 {
            slot::<Fn2>(client, 0x24)(client, &mut period as *mut _ as usize, &mut minimum as *mut _ as usize);
        }
        writeln!(
            o,
            "endpoint\t{name}\t{}\tname {friendly}\tphysical speakers {speakers}\tform factor {form}\tengine format {engine}\tmix format {m:x} {}\tperiod {period} minimum {minimum}",
            String::from_utf16_lossy(&s),
            hex(&mix)
        )
        .unwrap();
        if role == 0 {
            console = (dev, mix);
        }
    }
    console
}

/// `DSMIX_SURROUND`: see the module.
pub fn run(dir: &std::path::Path) {
    std::fs::create_dir_all(dir).unwrap();
    let mut dev_o = std::fs::File::create(dir.join("device.tsv")).unwrap();
    let mut tsv = std::fs::File::create(dir.join("cases.tsv")).unwrap();
    let mut txt = std::fs::File::create(dir.join("cases.txt")).unwrap();
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), 0);
        let (dev, mix) = device(&mut dev_o);
        assert!(!dev.is_null(), "no default render device");
        let installed = hooks::install(dev, mix.as_ptr());
        writeln!(dev_o, "{installed}").unwrap();
        let window = window();
        let mut ds: *mut Obj = std::ptr::null_mut();
        let r = DirectSoundCreate8(std::ptr::null(), &mut ds, std::ptr::null_mut());
        writeln!(dev_o, "DirectSoundCreate8\t{r:x}").unwrap();
        assert!(!ds.is_null(), "DirectSoundCreate8");
        let r = slot::<Fn2>(ds, 0x18)(ds, window as usize, 2);
        writeln!(dev_o, "SetCooperativeLevel\tDSSCL_PRIORITY\t{r:x}").unwrap();
        let mut caps = [0u32; 0x18];
        caps[0] = 0x60;
        let r = slot::<Fn1>(ds, 0x10)(ds, caps.as_mut_ptr() as usize);
        let caps_bytes: Vec<u8> = caps.iter().flat_map(|w| w.to_le_bytes()).collect();
        writeln!(dev_o, "GetCaps\t{r:x}\t{}\tflags {:x} free hw mixing all {} max hw 3d all {}", hex(&caps_bytes), caps[1], caps[8], caps[11]).unwrap();
        let mut speakers = 0u32;
        let r = slot::<Fn1>(ds, 0x20)(ds, &mut speakers as *mut _ as usize);
        writeln!(dev_o, "GetSpeakerConfig\t{r:x}\t{speakers:x}\tconfig {:x} geometry {:x}", speakers & 0xff, speakers >> 16).unwrap();
        // The primary buffer and its listener, as the game makes them (`00aea230`).
        let desc: [u32; 9] = [0x24, 0x91, 0, 0, 0, 0, 0, 0, 0];
        let mut primary: *mut Obj = std::ptr::null_mut();
        let r = slot::<Fn3>(ds, 0xc)(ds, desc.as_ptr() as usize, &mut primary as *mut _ as usize, 0);
        let mut pcaps = [0u32; 5];
        pcaps[0] = 0x14;
        let mut pfmt = [0u8; 0x40];
        let mut written = 0u32;
        let (rc, rf) = if primary.is_null() {
            (0, 0)
        } else {
            (
                slot::<Fn1>(primary, 0xc)(primary, pcaps.as_mut_ptr() as usize),
                slot::<Fn3>(primary, 0x14)(primary, pfmt.as_mut_ptr() as usize, 0x40, &mut written as *mut _ as usize),
            )
        };
        writeln!(dev_o, "primary\t{r:x}\tcaps {rc:x} {:x?}\tformat {rf:x} {}", pcaps, hex(&pfmt[..written as usize])).unwrap();
        let listener = qi(primary, &guid("279afa84-4981-11ce-a521-0020af0be560"));
        let mut all = [0u8; 0x40];
        all[0] = 0x40;
        let r = slot::<Fn1>(listener, 0xc)(listener, all.as_mut_ptr() as usize);
        writeln!(dev_o, "listener\t{r:x}\t{}", hex(&all)).unwrap();
        // As `00aea230`: the Doppler factor 1.
        slot::<FnF1>(listener, 0x30)(listener, 1.0, 0);
        // `DSMIX_LOCKFIRST`: a 3D buffer made and released at once, so the device's layout is
        // locked to the mix format before every case (DirectSound locks it at the first 3D
        // buffer and keeps it).
        if std::env::var_os("DSMIX_LOCKFIRST").is_some() {
            let mut wf = [0u8; 18];
            wf[0..2].copy_from_slice(&1u16.to_le_bytes());
            wf[2..4].copy_from_slice(&1u16.to_le_bytes());
            wf[4..8].copy_from_slice(&48000u32.to_le_bytes());
            wf[8..12].copy_from_slice(&96000u32.to_le_bytes());
            wf[12..14].copy_from_slice(&2u16.to_le_bytes());
            wf[14..16].copy_from_slice(&16u16.to_le_bytes());
            let d: [u32; 9] = [0x24, FLAGS_3D, 96000, 0, wf.as_ptr() as u32, 0, 0, 0, 0];
            let mut b: *mut Obj = std::ptr::null_mut();
            let r = slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b as *mut _ as usize, 0);
            writeln!(dev_o, "lock first\tCreateSoundBuffer 3D {r:x}").unwrap();
            if !b.is_null() {
                slot::<Fn0>(b, 8)(b);
            }
        }
        let mut freq = 0i64;
        QueryPerformanceFrequency(&mut freq);
        let us = |c: i64, origin: i64| (c - origin) * 1_000_000 / freq;
        Sleep(300);
        let mut warm = Vec::new();
        hooks::take(&mut warm);
        let mut warm_t = Vec::new();
        hooks::take_times(&mut warm_t);
        let frame = hooks::frame();
        let channels = if frame == 0 { 0 } else { frame / 4 };
        writeln!(dev_o, "packets\tframe bytes {frame} channels {channels}\twarm-up {} bytes {} packets", warm.len(), warm_t.len()).unwrap();
        for c in cases() {
            let (results, created, bcaps, out, times, moves) = play(ds, listener, &c, &us);
            let frame = hooks::frame();
            std::fs::write(dir.join(format!("{}.f32", c.name)), &out).unwrap();
            let sizes: std::collections::BTreeSet<usize> = times.iter().map(|t| t.2).collect();
            if sizes.len() > 1 {
                writeln!(dev_o, "layout changed in case {}\tframe bytes {sizes:?}", c.name).unwrap();
            }
            let mut t: Vec<String> = times.iter().map(|&(a, f, b)| format!("{a} {f} {b}")).collect();
            t.extend(moves);
            std::fs::write(dir.join(format!("{}.times", c.name)), t.join("\n")).unwrap();
            writeln!(
                tsv,
                "{}\t{} {} {} flags {:x} hrtf {} vol {} pan {} freq {} 3d {:?} fx {:?} orientation {:?} later {:?} moving {:?} frames {} looping {} with_2d {} three {} later_volume {:?} later_pan {:?} lock_at {:?}\tcreate {created:x} caps {bcaps:x?}\tresults {results:x?}\tframes {} of {} channels",
                c.name,
                c.channels,
                c.rate,
                c.bits,
                c.flags,
                c.hrtf,
                c.volume,
                c.pan,
                c.frequency,
                c.three_d,
                c.distortion,
                c.orientation,
                c.later,
                c.moving,
                c.frames,
                c.looping,
                c.with_2d,
                c.three,
                c.later_volume,
                c.later_pan,
                c.lock_at,
                out.len() / frame.max(1),
                frame / 4
            )
            .unwrap();
            writeln!(txt, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", c.name, kind(&c), c.channels, c.rate, c.bits, c.frames, c.seed, frame / 4, base_mb(&c)).unwrap();
            Sleep(100);
        }
        let mut calls = hooks::CALLS.lock().unwrap().clone();
        calls.push(format!("other render clients' packets\t{}", hooks::OTHERS.load(std::sync::atomic::Ordering::SeqCst)));
        for l in calls {
            writeln!(dev_o, "{l}").unwrap();
        }
        // The speaker configuration again, after the device ran.
        let mut speakers = 0u32;
        let r = slot::<Fn1>(ds, 0x20)(ds, &mut speakers as *mut _ as usize);
        writeln!(dev_o, "GetSpeakerConfig (end)\t{r:x}\t{speakers:x}").unwrap();
        slot::<Fn0>(listener, 8)(listener);
        slot::<Fn0>(primary, 8)(primary);
        slot::<Fn0>(ds, 8)(ds);
    }
}

type Played = (Vec<u32>, u32, [u32; 5], Vec<u8>, Vec<(i64, u32, usize)>, Vec<String>);

/// Makes, fills, sets and plays one case; answers the calls' results, `CreateSoundBuffer`'s
/// answer, the buffer's caps, the packets, their times (from `Play`) and the moves.
unsafe fn play(ds: *mut Obj, listener: *mut Obj, c: &Case, us: &dyn Fn(i64, i64) -> i64) -> Played {
    let mut results = Vec::new();
    let make = |channels: u16, rate: u32, bits: u16, flags: u32, hrtf: bool, frames: usize, seed: u32| -> (u32, *mut Obj) {
        let align = channels * bits / 8;
        let mut wf = [0u8; 18];
        wf[0..2].copy_from_slice(&1u16.to_le_bytes());
        wf[2..4].copy_from_slice(&channels.to_le_bytes());
        wf[4..8].copy_from_slice(&rate.to_le_bytes());
        wf[8..12].copy_from_slice(&(rate * u32::from(align)).to_le_bytes());
        wf[12..14].copy_from_slice(&align.to_le_bytes());
        wf[14..16].copy_from_slice(&bits.to_le_bytes());
        let bytes = frames as u32 * u32::from(align);
        let alg = if hrtf { ALG_HRTF_LIGHT } else { [0; 4] };
        let d: [u32; 9] = [0x24, flags, bytes, 0, wf.as_ptr() as u32, alg[0], alg[1], alg[2], alg[3]];
        let mut b: *mut Obj = std::ptr::null_mut();
        let r = slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b as *mut _ as usize, 0);
        if b.is_null() {
            return (r, b);
        }
        let data = signal_seeded(frames, channels as usize, bits, seed);
        let (mut p, mut n) = (std::ptr::null_mut::<u8>(), 0u32);
        slot::<Fn7>(b, 0x2c)(b, 0, bytes as usize, &mut p as *mut _ as usize, &mut n as *mut _ as usize, 0, 0, 0);
        std::ptr::copy_nonoverlapping(data.as_ptr(), p, n as usize);
        slot::<Fn4>(b, 0x4c)(b, p as usize, n as usize, 0, 0);
        (r, b)
    };
    let (created, b) = make(c.channels, c.rate, c.bits, c.flags, c.hrtf, c.frames, c.seed);
    if b.is_null() {
        return (results, created, [0; 5], Vec::new(), Vec::new(), Vec::new());
    }
    let mut bcaps = [0u32; 5];
    bcaps[0] = 0x14;
    slot::<Fn1>(b, 0xc)(b, bcaps.as_mut_ptr() as usize);
    results.push(slot::<Fn1>(b, 0x3c)(b, c.volume as usize));
    if c.flags & 0x40 != 0 {
        results.push(slot::<Fn1>(b, 0x40)(b, c.pan as usize));
    }
    if c.frequency != 0 {
        results.push(slot::<Fn1>(b, 0x44)(b, c.frequency as usize));
    }
    // The listener as the game sets it (`00aea8b0`): at the origin, its orientation, deferred;
    // the rolloff factor (deferred at start-up); then the buffer's settings, its position
    // deferred (`00aef090`), and one commit.
    let w = |v: f32| v.to_bits() as usize;
    let (front, top) = c.orientation.unwrap_or(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]));
    results.push(slot::<FnF3>(listener, 0x38)(listener, 0.0, 0.0, 0.0, 1));
    results.push(slot::<Fn7>(listener, 0x34)(listener, w(front[0]), w(front[1]), w(front[2]), w(top[0]), w(top[1]), w(top[2]), 1));
    let mut three_d: *mut Obj = std::ptr::null_mut();
    if let Some(t) = c.three_d {
        three_d = qi(b, &guid("279afa86-4981-11ce-a521-0020af0be560"));
        results.push(slot::<FnF1>(listener, 0x3c)(listener, t.rolloff, 1));
        results.push(slot::<FnF1>(listener, 0x30)(listener, t.doppler, 1));
        results.push(slot::<FnF1>(three_d, 0x44)(three_d, t.min, 1));
        results.push(slot::<FnF1>(three_d, 0x40)(three_d, t.max, 1));
        results.push(slot::<FnF3>(three_d, 0x50)(three_d, t.vel[0], t.vel[1], t.vel[2], 1));
        results.push(slot::<FnF3>(three_d, 0x4c)(three_d, t.pos[0], t.pos[1], t.pos[2], 1));
    }
    results.push(slot::<Fn0>(listener, 0x44)(listener));
    let mut fx: *mut Obj = std::ptr::null_mut();
    if let Some(params) = c.distortion {
        let b8 = qi(b, &guid("6825a449-7524-4d82-920f-50e36ab3ab1e"));
        let mut ed = [0u32; 8];
        ed[0] = 0x20;
        let g = guid("ef114c90-cd1d-484e-96e5-09cfaf912a21");
        std::ptr::copy_nonoverlapping(g.as_ptr(), ed.as_mut_ptr().add(2) as *mut u8, 16);
        let mut code = 0u32;
        results.push(slot::<Fn3>(b8, 0x54)(b8, 1, ed.as_ptr() as usize, &mut code as *mut _ as usize));
        let iid = guid("8ecf4326-455f-4d8b-bda9-8d5d3e9e3e0b");
        results.push(slot::<Fn4>(b8, 0x5c)(b8, g.as_ptr() as usize, 0, iid.as_ptr() as usize, &mut fx as *mut _ as usize));
        if !fx.is_null() {
            results.push(slot::<Fn1>(fx, 0xc)(fx, params.as_ptr() as usize));
        }
        slot::<Fn0>(b8, 8)(b8);
    }
    let other = if c.with_2d { make(1, 48000, 16, FLAGS_2D, false, 12000, 7).1 } else { std::ptr::null_mut() };
    // Two more 2D buffers made after this one (the third made while two exist).
    let mut more: Vec<*mut Obj> = Vec::new();
    if c.three {
        for (seed, pan) in [(7u32, -2000i32), (9, 3000)] {
            let (r, o) = make(1, 48000, 16, FLAGS_2D, false, 12000, seed);
            results.push(r);
            if !o.is_null() {
                results.push(slot::<Fn1>(o, 0x40)(o, pan as usize));
                more.push(o);
            }
        }
    }
    let mut discard = Vec::new();
    hooks::take(&mut discard);
    let mut discard_t = Vec::new();
    hooks::take_times(&mut discard_t);
    let origin = qpc();
    if !other.is_null() {
        results.push(slot::<Fn3>(other, 0x30)(other, 0, 0, 0));
    }
    for &o in &more {
        results.push(slot::<Fn3>(o, 0x30)(o, 0, 0, 0));
    }
    results.push(slot::<Fn3>(b, 0x30)(b, 0, 0, c.looping as usize));
    let mut moves = Vec::new();
    let (mut lock_b, mut lock_3d) = (std::ptr::null_mut::<Obj>(), std::ptr::null_mut::<Obj>());
    for step in 0..c.steps {
        if let Some((at_step, f)) = c.later {
            if step == at_step {
                let t = us(qpc(), origin);
                results.push(slot::<Fn1>(b, 0x44)(b, f as usize));
                moves.push(format!("freq {t} {step} {f}"));
            }
        }
        if let Some((at_step, v)) = c.later_volume {
            if step == at_step {
                let t = us(qpc(), origin);
                results.push(slot::<Fn1>(b, 0x3c)(b, v as usize));
                moves.push(format!("vol {t} {} {step} {v}", us(qpc(), origin)));
            }
        }
        if let Some((at_step, p)) = c.later_pan {
            if step == at_step {
                let t = us(qpc(), origin);
                results.push(slot::<Fn1>(b, 0x40)(b, p as usize));
                moves.push(format!("pan {t} {} {step} {p}", us(qpc(), origin)));
            }
        }
        if c.lock_at == Some(step) {
            let t = us(qpc(), origin);
            let (r, o) = make(1, 48000, 16, FLAGS_3D, false, 48000, 11);
            results.push(r);
            let t1 = us(qpc(), origin);
            if !o.is_null() {
                lock_b = o;
                lock_3d = qi(o, &guid("279afa86-4981-11ce-a521-0020af0be560"));
                results.push(slot::<FnF1>(lock_3d, 0x44)(lock_3d, 1.0e9, 0));
                results.push(slot::<FnF1>(lock_3d, 0x40)(lock_3d, 1.0e9, 0));
                results.push(slot::<FnF3>(lock_3d, 0x4c)(lock_3d, 3.0, 0.0, 0.0, 0));
                results.push(slot::<Fn3>(o, 0x30)(o, 0, 0, 1));
            }
            moves.push(format!("lock {t} {t1} {step} {} {r:x}", us(qpc(), origin)));
        }
        if let Some(m) = c.moving {
            if step > 0 {
                let p = at(m.start + m.step * step as f32, m.radius, m.height);
                let t = us(qpc(), origin);
                slot::<FnF3>(three_d, 0x4c)(three_d, p[0], p[1], p[2], 1);
                slot::<Fn0>(listener, 0x44)(listener);
                let t1 = us(qpc(), origin);
                moves.push(format!("pos {t} {t1} {step} {} {} {}", p[0], p[1], p[2]));
            }
        }
        Sleep(10);
    }
    slot::<Fn0>(b, 0x48)(b);
    if !other.is_null() {
        slot::<Fn0>(other, 0x48)(other);
    }
    for &o in more.iter().chain([lock_b].iter()) {
        if !o.is_null() {
            slot::<Fn0>(o, 0x48)(o);
        }
    }
    let mut out = Vec::new();
    hooks::take(&mut out);
    let mut times = Vec::new();
    hooks::take_times(&mut times);
    let times = times.iter().map(|&(t, f, size)| (us(t, origin), f, size)).collect();
    for o in [fx, three_d, other, lock_3d, lock_b, b].into_iter().chain(more) {
        if !o.is_null() {
            slot::<Fn0>(o, 8)(o);
        }
    }
    (results, created, bcaps, out, times, moves)
}

fn read_f32(path: &std::path::Path) -> Vec<f32> {
    let b = std::fs::read(path).unwrap_or_default();
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

/// The source as `f32` the way DirectSound reads it (x / 32768; 8-bit (x - 128) / 128), one
/// `Vec` per source channel.
fn source(channels: usize, bits: u16, frames: usize, seed: u32) -> Vec<Vec<f32>> {
    let raw = signal_seeded(frames, channels, bits, seed);
    let mut out = vec![Vec::with_capacity(frames); channels];
    for i in 0..frames * channels {
        let v = if bits == 16 { f32::from(i16::from_le_bytes([raw[2 * i], raw[2 * i + 1]])) / 32768.0 } else { (f32::from(raw[i]) - 128.0) / 128.0 };
        out[i % channels].push(v);
    }
    out
}

/// Least squares of `y` on the columns `x` (one or two): the coefficients and the residual's
/// RMS over `y`'s RMS.
fn fit(y: &[f32], x: &[&[f32]]) -> (Vec<f64>, f64) {
    let n = y.len();
    let k = x.len();
    let mut a = vec![[0f64; 2]; 2];
    let mut bv = [0f64; 2];
    for i in 0..n {
        for p in 0..k {
            bv[p] += f64::from(x[p][i]) * f64::from(y[i]);
            for q in 0..k {
                a[p][q] += f64::from(x[p][i]) * f64::from(x[q][i]);
            }
        }
    }
    let coef = if k == 1 {
        vec![if a[0][0] == 0.0 { 0.0 } else { bv[0] / a[0][0] }]
    } else {
        let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
        if det == 0.0 {
            vec![0.0, 0.0]
        } else {
            vec![(bv[0] * a[1][1] - bv[1] * a[0][1]) / det, (a[0][0] * bv[1] - a[1][0] * bv[0]) / det]
        }
    };
    let (mut e, mut s) = (0f64, 0f64);
    for i in 0..n {
        let mut m = 0.0;
        for p in 0..k {
            m += coef[p] * f64::from(x[p][i]);
        }
        e += (f64::from(y[i]) - m).powi(2);
        s += f64::from(y[i]).powi(2);
    }
    (coef, if s == 0.0 { 0.0 } else { (e / s).sqrt() })
}

fn db(g: f64) -> String {
    if g == 0.0 {
        "-inf".into()
    } else {
        format!("{:.3}", 20.0 * g.abs().log10())
    }
}

/// For a mono source: whether `y` is exactly `f32(x * g)` for one `f32` `g` (the gain read at
/// the largest sample, then each neighbour of it in `f32` steps tried): the gain and the
/// samples that differ.
fn exact_gain(y: &[f32], x: &[f32]) -> (f32, usize) {
    let (mut best, mut at) = (0f32, 0usize);
    for i in 0..x.len() {
        if x[i].abs() > best {
            best = x[i].abs();
            at = i;
        }
    }
    if best == 0.0 {
        return (0.0, y.iter().filter(|v| **v != 0.0).count());
    }
    let g0 = y[at] / x[at];
    let mut result = (g0, usize::MAX);
    let mut g = g0;
    for _ in 0..8 {
        g = f32::from_bits(g.to_bits().wrapping_sub(1));
    }
    for _ in 0..17 {
        let bad = (0..x.len()).filter(|&i| x[i] * g != y[i]).count();
        if bad < result.1 {
            result = (g, bad);
        }
        g = f32::from_bits(g.to_bits().wrapping_add(1));
    }
    result
}

/// A case's packets split where the frame size changes (DirectSound closes its stream and opens
/// another when the device's layout is locked): each run's channels, its first packet's time
/// (microseconds from `Play`) and its samples. `<case>.times` lines of three numbers are the
/// packets (time, frames, frame bytes); the other lines are the moves.
fn segments(dir: &std::path::Path, name: &str, all: &[f32], dch: usize) -> Vec<(usize, i64, Vec<f32>)> {
    let times = std::fs::read_to_string(dir.join(format!("{name}.times"))).unwrap_or_default();
    let mut segs: Vec<(usize, i64, Vec<f32>)> = Vec::new();
    let mut at = 0usize;
    for l in times.lines() {
        let f: Vec<&str> = l.split(' ').collect();
        if f.len() != 3 {
            continue;
        }
        let (Ok(t), Ok(frames), Ok(size)) = (f[0].parse::<i64>(), f[1].parse::<usize>(), f[2].parse::<usize>()) else {
            continue;
        };
        let ch = size / 4;
        let n = frames * ch;
        if ch == 0 || at + n > all.len() {
            break;
        }
        match segs.last_mut() {
            Some(s) if s.0 == ch => s.2.extend_from_slice(&all[at..at + n]),
            _ => segs.push((ch, t, all[at..at + n].to_vec())),
        }
        at += n;
    }
    if segs.is_empty() {
        segs.push((dch, 0, all.to_vec()));
    }
    segs
}

/// Where in the mono source `x` (`len` frames) the samples `y` from `w0` come from: the offset
/// with the best normalised correlation over 256 frames.
fn locate(y: &[f32], w0: usize, x: &[f32]) -> (usize, f64) {
    let win = 256.min(y.len() - w0);
    let ny: f64 = y[w0..w0 + win].iter().map(|v| f64::from(*v).powi(2)).sum::<f64>().sqrt();
    let mut best = (0usize, 0f64);
    for off in 0..x.len() {
        let (mut c, mut nx) = (0f64, 0f64);
        for i in 0..win {
            let xv = f64::from(x[(off + i) % x.len()]);
            c += f64::from(y[w0 + i]) * xv;
            nx += xv * xv;
        }
        let r = if nx == 0.0 || ny == 0.0 { 0.0 } else { c / (nx.sqrt() * ny) };
        if r.abs() > best.1.abs() {
            best = (off, r);
        }
    }
    best
}

/// One run of packets of a case whose layout changed: per channel, its level, and for a mono
/// source which of the case's voices it carries (the source of the case's seed, or seed 11, 7
/// or 9: the lock's 3D buffer and the extra 2D buffers), where in it, and the exact gain.
fn segment_report(o: &mut impl Write, name: &str, i: usize, seg: &(usize, i64, Vec<f32>), sch: usize, bits: u16, frames: usize, seed: u32, src_rms: f64) {
    let (dch, t, all) = seg;
    let n = all.len() / dch;
    let chan: Vec<Vec<f32>> = (0..*dch).map(|k| (0..n).map(|f| all[f * dch + k]).collect()).collect();
    let mut parts = Vec::new();
    for (k, y) in chan.iter().enumerate() {
        let Some(w0) = y.iter().position(|v| *v != 0.0) else {
            let neg = y.iter().filter(|v| v.is_sign_negative()).count();
            parts.push(format!("ch{k} 0 (-0 x{neg})"));
            continue;
        };
        let rms = (y.iter().map(|v| f64::from(*v).powi(2)).sum::<f64>() / n as f64).sqrt();
        let mut s = format!("ch{k} {}dB from {w0}", db(rms / src_rms));
        if sch == 1 && n - w0 >= 64 {
            let mut best: Option<(u32, usize, f64, Vec<f32>)> = None;
            for (sd, len) in [(seed, frames), (11, 48000), (7, 12000), (9, 12000)] {
                let x = source(1, if sd == seed { bits } else { 16 }, len, sd).remove(0);
                let (off, r) = locate(y, w0, &x);
                if best.as_ref().map_or(true, |b| r.abs() > b.2.abs()) {
                    best = Some((sd, off, r, x));
                }
            }
            let (sd, off, r, x) = best.unwrap();
            // While the source has samples left (no wrap), and the channel goes on.
            let take = (x.len() - off).min(n - w0);
            let ys = &y[w0..w0 + take];
            let xs = &x[off..off + take];
            let (coef, res) = fit(ys, &[xs]);
            let (g, bad) = exact_gain(ys, xs);
            s += &format!(" seed {sd:x} at {off} corr {r:.4} gain {:.7}({}dB) res {res:.2e} over {take} f32 {:08x} misses {bad}", coef[0], db(coef[0]), g.to_bits());
        }
        parts.push(s);
    }
    writeln!(o, "{name}#{i}\tsegment\t{dch} channels from {t} us, {n} frames\t{}", parts.join("\t")).unwrap();
}

/// `DSMIX_REPORT=1 dsmix-measure <dir>`: per case, per device channel, what DirectSound sent.
/// `exact` cases: aligned on the source (the first frame any channel is not 0, then the best
/// offset within 64 frames), each channel's least-squares gain on the source channels (dB),
/// the residual (0 for a plain gain), and for a mono source the exact `f32` gain and the
/// samples it misses. `shape` cases: each channel's level (dB against the source's) and which
/// channel it is an exact multiple of. `moving` cases: the gains every 10 ms period. A case
/// whose packets change layout is reported per run of packets ([`segment_report`]). Channels
/// that carry nothing are `0`, with the count of negative zeros. Written to `<dir>/report.txt`.
pub fn report(dir: &std::path::Path) {
    let list = std::fs::read_to_string(dir.join("cases.txt")).expect("cases.txt");
    let mut o = std::fs::File::create(dir.join("report.txt")).unwrap();
    for line in list.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 8 {
            continue;
        }
        let (name, kind) = (f[0], f[1]);
        let sch: usize = f[2].parse().unwrap();
        let bits: u16 = f[4].parse().unwrap();
        let frames: usize = f[5].parse().unwrap();
        let seed: u32 = f[6].parse().unwrap();
        let dch: usize = f[7].parse().unwrap();
        let base: i32 = f.get(8).and_then(|v| v.parse().ok()).unwrap_or(0);
        if dch == 0 {
            continue;
        }
        let raw = read_f32(&dir.join(format!("{name}.f32")));
        let segs = segments(dir, name, &raw, dch);
        if segs.len() > 1 {
            let src = source(sch, bits, frames, seed);
            let src_rms = (src[0].iter().map(|v| f64::from(*v).powi(2)).sum::<f64>() / frames as f64).sqrt();
            for (i, seg) in segs.iter().enumerate() {
                segment_report(&mut o, name, i, seg, sch, bits, frames, seed, src_rms);
            }
            continue;
        }
        let (dch, _, all) = &segs[0];
        let dch = *dch;
        let n = all.len() / dch;
        let chan: Vec<Vec<f32>> = (0..dch).map(|k| (0..n).map(|i| all[i * dch + k]).collect()).collect();
        let first = (0..n).find(|&i| (0..dch).any(|k| chan[k][i] != 0.0));
        let Some(first) = first else {
            writeln!(o, "{name}\t{kind}\tsilent ({n} frames)").unwrap();
            continue;
        };
        let src = source(sch, bits, frames, seed);
        let src_rms = (src[0].iter().map(|v| f64::from(*v).powi(2)).sum::<f64>() / frames as f64).sqrt();
        match kind {
            "exact" => {
                // The offset that fits best (smallest residual summed over the channels, on the
                // first 2000 frames).
                let win = frames.min(2000);
                let mut best = (f64::MAX, first);
                for off in first.saturating_sub(64)..first + 64 {
                    if off + win > n {
                        break;
                    }
                    let xs: Vec<&[f32]> = src.iter().map(|v| &v[..win]).collect();
                    let mut total = 0.0;
                    for k in 0..dch {
                        let y = &chan[k][off..off + win];
                        if y.iter().all(|v| *v == 0.0) {
                            continue;
                        }
                        total += fit(y, &xs).1;
                    }
                    if total < best.0 {
                        best = (total, off);
                    }
                }
                let off = best.1;
                let take = frames.min(n.saturating_sub(off));
                let xs: Vec<&[f32]> = src.iter().map(|v| &v[..take]).collect();
                let mut parts = Vec::new();
                for k in 0..dch {
                    let y = &chan[k][off..off + take];
                    if y.iter().all(|v| *v == 0.0) {
                        parts.push(format!("ch{k} 0 (-0 x{})", chan[k].iter().filter(|v| v.is_sign_negative()).count()));
                        continue;
                    }
                    let (coef, res) = fit(y, &xs);
                    let mut s = format!("ch{k} {} res {res:.2e}", coef.iter().map(|g| format!("{g:.7}({}dB)", db(*g))).collect::<Vec<_>>().join(" "));
                    // One source channel reaches it: its exact `f32` gain and the millibels.
                    let only = if sch == 1 || coef[1].abs() < 1e-9 {
                        Some(0)
                    } else if coef[0].abs() < 1e-9 {
                        Some(1)
                    } else {
                        None
                    };
                    if let Some(c) = only {
                        let (g, bad) = exact_gain(y, xs[c]);
                        s += &format!(" src{c} f32 {:08x} {g:e} misses {bad} {}", g.to_bits(), decompose(g, base));
                    }
                    parts.push(s);
                }
                // Outside the voice: anything not zero before it or after it.
                let stray = (0..n).filter(|&i| i < off || i >= off + take).filter(|&i| (0..dch).any(|k| chan[k][i] != 0.0)).count();
                writeln!(o, "{name}\texact\toffset {off} (first {first}) frames {take} of {n} stray {stray}\t{}", parts.join("\t")).unwrap();
            }
            "moving" => {
                // Per period of 480 frames from the first: each channel's gain on the looping
                // source (mono), aligned at `first`.
                let x = &src[0];
                let mut rows = Vec::new();
                let mut start = first;
                let mut p = 0;
                while start + 480 <= n {
                    let xs: Vec<f32> = (0..480).map(|i| x[(start - first + i) % frames]).collect();
                    let mut row = format!("p{p} @{start}");
                    for k in 0..dch {
                        let y = &chan[k][start..start + 480];
                        let (coef, res) = fit(y, &[xs.as_slice()]);
                        row += &format!(" {:.4}/{res:.0e}", coef[0]);
                    }
                    rows.push(row);
                    start += 480;
                    p += 1;
                }
                writeln!(o, "{name}\tmoving\tfirst {first}\t{}", rows.join("\t")).unwrap();
            }
            _ => {
                let mut parts = Vec::new();
                for k in 0..dch {
                    let y = &chan[k];
                    let rms = (y.iter().map(|v| f64::from(*v).powi(2)).sum::<f64>() / frames as f64).sqrt();
                    if rms == 0.0 {
                        parts.push(format!("ch{k} 0 (-0 x{})", y.iter().filter(|v| v.is_sign_negative()).count()));
                        continue;
                    }
                    // An exact multiple of an earlier channel?
                    let mut copy = String::new();
                    for j in 0..k {
                        let z = &chan[j];
                        let m = (0..n).max_by(|&a, &b| z[a].abs().total_cmp(&z[b].abs())).unwrap();
                        if z[m] == 0.0 {
                            continue;
                        }
                        let r = y[m] / z[m];
                        let off = (0..n).filter(|&i| (z[i] * r - y[i]).abs() > 1e-7 * y[i].abs().max(1e-30)).count();
                        if off == 0 {
                            copy = format!(" = ch{j} x {r:e}");
                            break;
                        }
                    }
                    parts.push(format!("ch{k} {}dB{copy}", db(rms / src_rms)));
                }
                writeln!(o, "{name}\t{kind}\tfirst {first} frames {n}\t{}", parts.join("\t")).unwrap();
            }
        }
    }
}
