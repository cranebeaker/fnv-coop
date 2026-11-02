//! dsmix-measure: what DirectSound 8 sends to the sound device for a known buffer and known
//! settings, captured with WASAPI loopback on the default output device, for the platform's
//! mixer (`engine_core::units::platform::mixer`). 32-bit, as the game. The test sounds are
//! made up (noise from a fixed generator, quiet: about -30 dBFS) and short; they are heard on
//! the speakers while it runs. Writes `<out-dir>/<case>.f32` (the device's samples as captured,
//! interleaved) and `<out-dir>/cases.tsv` (the case, its settings, the device format, frames
//! captured). Nothing else is written.
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/dsmix-measure/Cargo.toml -- <out-dir> [endpoint-id description]`
//!
//! With an endpoint (the GUID part of its id) and the DirectSound description of the same
//! device, it plays on and records that device instead of the default one: a device without
//! audio enhancements is needed for bit-exact captures (the default one here smeared the
//! signal). Modes, by environment variable: none (the base cases), `DSMIX_SWEEP` (volumes, pans,
//! 3D angles and distances), `DSMIX_DENSE` (every 7th volume), `DSMIX_3D` (a 3D position grid),
//! `DSMIX_MULTI` (several voices at once: the order of the sum), `DSMIX_EXACT` (the exact 3D stage
//! and distortion: turned listeners, the game's distances, the effect's formats and edges),
//! `DSMIX_SRC` (the sample-rate converter: see `src_cases`), `DSMIX_SETPOS` with `DSMIX_HOOK`
//! (`SetCurrentPosition` while a buffer plays: see `setpos`; `DSMIX_SETPOS_ORDER`: the sum order
//! after it, see `setpos_order`).
use std::ffi::c_void;
use std::io::Write;

#[link(name = "dsound")]
extern "system" {
    fn DirectSoundCreate8(device: *const u8, out: *mut *mut Obj, outer: *mut c_void) -> u32;
}
#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(reserved: *mut c_void, flags: u32) -> u32;
    fn CoCreateInstance(clsid: *const [u8; 16], outer: *mut c_void, context: u32, iid: *const [u8; 16], out: *mut *mut Obj) -> u32;
}
#[link(name = "kernel32")]
extern "system" {
    fn Sleep(ms: u32);
    fn QueryPerformanceCounter(out: *mut i64) -> i32;
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
}

fn qpc() -> i64 {
    let mut c = 0i64;
    unsafe { QueryPerformanceCounter(&mut c) };
    c
}
#[link(name = "user32")]
extern "system" {
    fn GetDesktopWindow() -> *mut c_void;
}

#[repr(C)]
struct Obj {
    vtable: *const usize,
}

unsafe fn slot<F: Copy>(o: *mut Obj, offset: usize) -> F {
    let f = *(*o).vtable.add(offset / 4);
    std::mem::transmute_copy(&f)
}

type Fn0 = unsafe extern "system" fn(*mut Obj) -> u32;
type Fn1 = unsafe extern "system" fn(*mut Obj, usize) -> u32;
type Fn2 = unsafe extern "system" fn(*mut Obj, usize, usize) -> u32;
type Fn3 = unsafe extern "system" fn(*mut Obj, usize, usize, usize) -> u32;
type Fn4 = unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize) -> u32;
type Fn5 = unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize, usize) -> u32;
type Fn7 = unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize, usize, usize, usize) -> u32;
type FnInit = unsafe extern "system" fn(*mut Obj, u32, u32, i64, i64, *const u8, *const c_void) -> u32;
type FnF1 = unsafe extern "system" fn(*mut Obj, f32, u32) -> u32;
type FnF3 = unsafe extern "system" fn(*mut Obj, f32, f32, f32, u32) -> u32;

fn guid(s: &str) -> [u8; 16] {
    let h: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    let b = |i: usize| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap();
    [b(3), b(2), b(1), b(0), b(5), b(4), b(7), b(6), b(8), b(9), b(10), b(11), b(12), b(13), b(14), b(15)]
}

unsafe fn qi(o: *mut Obj, iid: &[u8; 16]) -> *mut Obj {
    let mut out: *mut Obj = std::ptr::null_mut();
    slot::<Fn2>(o, 0)(o, iid.as_ptr() as usize, &mut out as *mut _ as usize);
    out
}

/// The test signal: noise from a fixed 32-bit generator, `frames` frames of `channels`
/// channels, `bits` bits, about -30 dBFS (16-bit: +-1000; 8-bit: 128 +- 4).
pub fn signal(frames: usize, channels: usize, bits: u16) -> Vec<u8> {
    signal_seeded(frames, channels, bits, 0x1234_5678)
}

/// [`signal`] from another seed (the voices played together in `DSMIX_MULTI`).
pub fn signal_seeded(frames: usize, channels: usize, bits: u16, seed: u32) -> Vec<u8> {
    let mut x = seed;
    let mut out = Vec::new();
    for _ in 0..frames * channels {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let v = (x % 2001) as i32 - 1000;
        if bits == 16 {
            out.extend_from_slice(&(v as i16).to_le_bytes());
        } else {
            out.push((128 + v / 250) as u8);
        }
    }
    out
}

struct Case {
    name: &'static str,
    channels: u16,
    rate: u32,
    bits: u16,
    flags: u32,
    volume: i32,
    pan: i32,
    frequency: u32,
    /// 3D: position, velocity, min distance, max distance; listener rolloff, Doppler.
    three_d: Option<([f32; 3], [f32; 3], f32, f32, f32, f32)>,
    distortion: Option<[f32; 5]>,
    /// 3D: the listener's front and top (else +z and +y).
    orientation: Option<([f32; 3], [f32; 3])>,
    /// 3D: the position set with `DS3D_DEFERRED` and never committed.
    uncommitted: bool,
    /// Frames in the buffer (else half a second).
    frames: Option<usize>,
    /// A frequency set while it plays: after this many 10 ms steps, this frequency.
    later: Option<(u32, u32)>,
    /// A frequency set on another buffer of the same kind (made, never played) while this one
    /// plays: after this many 10 ms steps, this frequency.
    other: Option<(u32, u32)>,
}

fn cases() -> Vec<Case> {
    let base = Case {
        name: "",
        channels: 1,
        rate: 48000,
        bits: 16,
        flags: 0xe0 | 0x8000,
        volume: 0,
        pan: 0,
        frequency: 0,
        three_d: None,
        distortion: None,
        orientation: None,
        uncommitted: false,
        frames: None,
        later: None,
        other: None,
    };
    let mut v = Vec::new();
    let mut add = |name: &'static str, f: &dyn Fn(&mut Case)| {
        let mut c = Case { name, ..base };
        f(&mut c);
        v.push(c);
    };
    add("mono48", &|_| {});
    add("stereo48", &|c| c.channels = 2);
    add("mono48_8bit", &|c| c.bits = 8);
    for (n, vol) in [("vol_m100", -100), ("vol_m600", -600), ("vol_m2000", -2000), ("vol_m6000", -6000)] {
        add(n, &move |c| c.volume = vol);
    }
    for (n, pan) in [("pan_m10000", -10000), ("pan_m5000", -5000), ("pan_m1000", -1000), ("pan_p1000", 1000), ("pan_p5000", 5000), ("pan_p10000", 10000)] {
        add(n, &move |c| c.pan = pan);
    }
    add("stereo48_pan_p1000", &|c| {
        c.channels = 2;
        c.pan = 1000
    });
    for (n, rate) in [("mono22050", 22050), ("mono44100", 44100), ("mono11025", 11025), ("mono32000", 32000)] {
        add(n, &move |c| c.rate = rate);
    }
    for (n, f) in [("freq48_to_24000", 24000), ("freq48_to_47000", 47000), ("freq48_to_60000", 60000)] {
        add(n, &move |c| c.frequency = f);
    }
    for (n, pos, vel) in [
        ("3d_front_1", [0.0, 0.0, 1.0], [0.0; 3]),
        ("3d_right_1", [1.0, 0.0, 0.0], [0.0; 3]),
        ("3d_left_1", [-1.0, 0.0, 0.0], [0.0; 3]),
        ("3d_behind_1", [0.0, 0.0, -1.0], [0.0; 3]),
        ("3d_front_10", [0.0, 0.0, 10.0], [0.0; 3]),
        ("3d_right_3", [3.0, 0.0, 0.0], [0.0; 3]),
        ("3d_front_right_2", [1.4142135, 0.0, 1.4142135], [0.0; 3]),
        ("3d_above_1", [0.0, 1.0, 0.0], [0.0; 3]),
        ("3d_doppler", [0.0, 0.0, 5.0], [0.0, 0.0, -50.0]),
    ] {
        add(n, &move |c| {
            c.flags = 0x300b0 | 0x8000;
            c.three_d = Some((pos, vel, 1.0, 1.0e9, 1.0, 1.0));
        });
    }
    add("3d_front_10_min5", &|c| {
        c.flags = 0x300b0 | 0x8000;
        c.three_d = Some(([0.0, 0.0, 10.0], [0.0; 3], 5.0, 1.0e9, 1.0, 1.0));
    });
    add("3d_front_10_rolloff2", &|c| {
        c.flags = 0x300b0 | 0x8000;
        c.three_d = Some(([0.0, 0.0, 10.0], [0.0; 3], 1.0, 1.0e9, 2.0, 1.0));
    });
    add("3d_front_10_max4", &|c| {
        c.flags = 0x300b0 | 0x8000;
        c.three_d = Some(([0.0, 0.0, 10.0], [0.0; 3], 1.0, 4.0, 1.0, 1.0));
    });
    for (n, gain) in [("distortion_m18", -18.0f32), ("distortion_m40", -40.0), ("distortion_0", 0.0)] {
        add(n, &move |c| {
            c.flags = 0x2e0 | 0x8000;
            c.distortion = Some([gain, 0.0, 5000.0, 1135.0, 5000.0]);
        });
    }
    add("distortion_m18_edge50", &|c| {
        c.flags = 0x2e0 | 0x8000;
        c.distortion = Some([-18.0, 50.0, 2400.0, 2400.0, 8000.0]);
    });
    if std::env::var_os("DSMIX_SRC").is_some() {
        src_cases(&mut add);
    }
    if std::env::var_os("DSMIX_SWEEP").is_some() {
        let leak = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };
        for vol in [-1, -5, -9, -10, -11, -49, -50, -51, -99, -101, -150, -199, -200, -555, -1000, -1234, -3000, -5000, -8000, -9000, -9999, -10000] {
            add(leak(format!("sweep_vol_{vol}")), &move |c| c.volume = vol);
        }
        for pan in [-1, -9, -10, -11, -50, -99, -100, -101, -250, -2000, -3000, -4000, -4500, -4900, -5001, -6000, -9999] {
            add(leak(format!("sweep_pan_{pan}")), &move |c| c.pan = pan);
        }
        add("sweep_vol_m300_pan_m700", &|c| {
            c.volume = -300;
            c.pan = -700
        });
        for deg in (0..=180).step_by(15) {
            let a = (deg as f32).to_radians();
            add(leak(format!("sweep_az_{deg}")), &move |c| {
                c.flags = 0x300b0 | 0x8000;
                c.three_d = Some(([a.sin(), 0.0, a.cos()], [0.0; 3], 1.0, 1.0e9, 1.0, 1.0));
            });
        }
        for (d, min, roll) in [(0.5f32, 1.0f32, 1.0f32), (1.5, 1.0, 1.0), (2.0, 1.0, 1.0), (3.0, 1.0, 1.0), (7.0, 1.0, 1.0), (100.0, 1.0, 1.0), (2.0, 1.0, 0.5), (2.0, 1.0, 0.0), (20.0, 3.0, 1.0), (4.0, 1.0, 1.5)] {
            add(leak(format!("sweep_dist_{d}_{min}_{roll}")), &move |c| {
                c.flags = 0x300b0 | 0x8000;
                c.three_d = Some(([0.0, 0.0, d], [0.0; 3], min, 1.0e9, roll, 1.0));
            });
        }
        add("sweep_az_30_vol_m500", &|c| {
            c.flags = 0x300b0 | 0x8000;
            c.volume = -500;
            c.three_d = Some(([0.5, 0.0, 0.8660254], [0.0; 3], 1.0, 1.0e9, 1.0, 1.0));
        });
    }
    if std::env::var_os("DSMIX_3D").is_some() {
        let leak = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };
        for xi in -8..=8 {
            for zi in -4..=4 {
                for yi in [0, 2] {
                    let (x, y, z) = (xi as f32 * 0.5, yi as f32 * 0.5, zi as f32);
                    if x == 0.0 && y == 0.0 && z == 0.0 {
                        continue;
                    }
                    add(leak(format!("sweep_dense_3d_{x}_{y}_{z}")), &move |c| {
                        c.flags = 0x300b0 | 0x8000;
                        c.three_d = Some(([x, y, z], [0.0; 3], 1.0, 1.0e9, 1.0, 1.0));
                    });
                }
            }
        }
    }
    if std::env::var_os("DSMIX_DENSE").is_some() {
        let leak = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };
        for vol in (1..=3000).step_by(7).map(|x: i32| -x) {
            add(leak(format!("sweep_dense_vol_{vol}")), &move |c| c.volume = vol);
        }
    }
    if std::env::var_os("DSMIX_EXACT").is_some() {
        exact_cases(&mut add);
    }
    if std::env::var_os("DSMIX_SRC").is_some() {
        v.retain(|c| c.name.starts_with("src_"));
    }
    if std::env::var_os("DSMIX_SWEEP").is_some() {
        v.retain(|c| c.name.starts_with("sweep_"));
    }
    if std::env::var_os("DSMIX_EXACT").is_some() {
        v.retain(|c| c.name.starts_with("x_"));
    }
    v
}

/// `DSMIX_EXACT`: the cases for the exact 3D stage and distortion (`platform::ds3d`,
/// `platform::dsfx`): turned listeners, the game's distances (both 1e9), height, the near zone,
/// an uncommitted deferred position; the distortion with the game's parameters on mono and
/// stereo, 16 and 8 bits, every edge band, and buffers at 11025 and 32000 Hz played at
/// 48000 Hz (the effect runs at the format's rate, so its limits at a third of the rate show
/// without resampling).
fn exact_cases(add: &mut dyn FnMut(&'static str, &dyn Fn(&mut Case))) {
    let leak = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };
    let three = 0x300b0 | 0x8000;
    // Turned listeners (front, top), a source at a few places.
    let turns: [(&str, [f32; 3], [f32; 3]); 7] = [
        ("yaw30", [0.5, 0.0, 0.8660254], [0.0, 1.0, 0.0]),
        ("yaw100", [0.98480775, 0.0, -0.17364818], [0.0, 1.0, 0.0]),
        ("yaw33_7", [0.55484474, 0.0, 0.8319391], [0.0, 1.0, 0.0]),
        ("yawm71_2", [-0.9466330, 0.0, 0.32227355], [0.0, 1.0, 0.0]),
        ("pitch20", [0.0, 0.34202015, 0.9396926], [0.0, 0.9396926, -0.34202015]),
        ("roll25", [0.0, 0.0, 1.0], [-0.42261827, 0.90630779, 0.0]),
        ("mixed", [0.3, 0.2, 0.9], [0.1, 1.0, -0.2]),
    ];
    for (tn, front, top) in turns {
        for (pn, pos) in [
            ("far", [3.0f32, 0.0, 4.0]),
            ("left", [-4.0, 1.0, 0.5]),
            ("near", [0.7, 0.3, -1.1]),
        ] {
            add(leak(format!("x_turn_{tn}_{pn}")), &move |c| {
                c.flags = three;
                c.three_d = Some((pos, [0.0; 3], 1.0, 1.0e9, 1.0, 1.0));
                c.orientation = Some((front, top));
            });
        }
    }
    // The game's distances, a source near and far, with height.
    for (n, pos) in [
        ("g_ahead", [0.0f32, 0.0, 3.0]),
        ("g_near", [0.6, 0.0, 0.8]),
        ("g_close", [1.2, 0.0, -0.4]),
        ("g_above", [0.0, 5.0, 0.0]),
        ("g_far", [-30.0, 2.0, 40.0]),
        ("g_box", [2.4, 0.0, 2.4]),
        ("g_edge", [2.49, 0.0, 0.1]),
    ] {
        add(leak(format!("x_3d_{n}")), &move |c| {
            c.flags = three;
            c.three_d = Some((pos, [0.0; 3], 1.0e9, 1.0e9, 1.0, 1.0));
        });
    }
    add("x_3d_uncommitted", &|c| {
        c.flags = three;
        c.three_d = Some(([-3.0, 0.0, 1.0], [0.0; 3], 1.0, 1.0e9, 1.0, 1.0));
        c.uncommitted = true;
    });
    add("x_3d_vol_m1234_min3", &|c| {
        c.flags = three;
        c.volume = -1234;
        c.three_d = Some(([1.0, 0.0, 6.0], [0.0; 3], 3.0, 1.0e9, 1.0, 1.0));
    });
    // The distortion.
    let game = [-18.2f32, 0.0, 5000.0, 1135.0, 5000.0];
    for (n, channels, bits) in [("mono16", 1u16, 16u16), ("stereo16", 2, 16), ("mono8", 1, 8), ("stereo8", 2, 8)] {
        add(leak(format!("x_fx_game_{n}")), &move |c| {
            c.flags = 0x2e0 | 0x8000;
            c.channels = channels;
            c.bits = bits;
            c.distortion = Some(game);
        });
    }
    for k in 0..30 {
        // An edge in each of the 30 bands (shifts 2 to 31).
        let edge = ((k as f32 + 0.5) / 29.0 * 100.0).min(100.0);
        add(leak(format!("x_fx_edge_{k}")), &move |c| {
            c.flags = 0x2e0 | 0x8000;
            c.distortion = Some([-30.0, edge, 3000.0, 2000.0, 7000.0]);
        });
    }
    for rate in [11025u32, 32000] {
        add(leak(format!("x_fx_rate{rate}")), &move |c| {
            c.flags = 0x2e0 | 0x8000;
            c.rate = rate;
            c.frequency = 48000;
            c.distortion = Some(game);
        });
    }
}

/// The default output device's loopback capture client: (audio client, capture client, the
/// mix format's bytes).
unsafe fn loopback(endpoint: Option<&str>) -> (*mut Obj, *mut Obj, Vec<u8>) {
    let mut en: *mut Obj = std::ptr::null_mut();
    let r = CoCreateInstance(
        &guid("bcde0395-e52f-467c-8e3d-c4579291692e"),
        std::ptr::null_mut(),
        0x17,
        &guid("a95664d2-9614-4f35-a746-de8db63617e6"),
        &mut en,
    );
    assert_eq!(r, 0, "MMDeviceEnumerator");
    let mut dev: *mut Obj = std::ptr::null_mut();
    match endpoint {
        Some(id) => {
            let wide: Vec<u16> = format!("{{0.0.0.00000000}}.{{{id}}}").encode_utf16().chain([0]).collect();
            let r = slot::<Fn2>(en, 0x14)(en, wide.as_ptr() as usize, &mut dev as *mut _ as usize);
            assert_eq!(r, 0, "no endpoint {id}");
        }
        None => {
            slot::<Fn3>(en, 0x10)(en, 0, 0, &mut dev as *mut _ as usize);
        }
    }
    let mut client: *mut Obj = std::ptr::null_mut();
    let iid_client = guid("1cb9ad4c-dbfa-4c32-b178-c2f568a703b2");
    let r = slot::<Fn4>(dev, 0xc)(dev, iid_client.as_ptr() as usize, 0x17, 0, &mut client as *mut _ as usize);
    assert_eq!(r, 0, "IMMDevice::Activate");
    let mut fmt: *const u8 = std::ptr::null();
    slot::<Fn1>(client, 0x20)(client, &mut fmt as *mut _ as usize);
    let size = 18 + u16::from_le_bytes([*fmt.add(16), *fmt.add(17)]) as usize;
    let fmt_bytes = std::slice::from_raw_parts(fmt, size).to_vec();
    let r = slot::<FnInit>(client, 0xc)(client, 0, 0x2_0000, 20_000_000, 0, fmt, std::ptr::null());
    assert_eq!(r, 0, "IAudioClient::Initialize loopback");
    let mut cap: *mut Obj = std::ptr::null_mut();
    let iid_cap = guid("c8adbd64-e71e-48a0-a4de-185c395cd317");
    slot::<Fn2>(client, 0x38)(client, iid_cap.as_ptr() as usize, &mut cap as *mut _ as usize);
    if std::env::var_os("DSMIX_HOOK").is_some() {
        let frame_bytes = u16::from_le_bytes([fmt_bytes[12], fmt_bytes[13]]) as usize;
        hook::install(dev, fmt, frame_bytes);
    }
    (client, cap, fmt_bytes)
}

/// Drains the capture client into `out` (bytes); silent packets add zeros.
unsafe fn drain(cap: *mut Obj, frame_bytes: usize, out: &mut Vec<u8>) {
    if hook::ON.load(std::sync::atomic::Ordering::SeqCst) {
        let mut discard = Vec::new();
        hook::ON.store(false, std::sync::atomic::Ordering::SeqCst);
        drain(cap, frame_bytes, &mut discard);
        hook::ON.store(true, std::sync::atomic::Ordering::SeqCst);
        hook::take(out);
        return;
    }
    loop {
        let mut n = 0u32;
        slot::<Fn1>(cap, 0x14)(cap, &mut n as *mut _ as usize);
        if n == 0 {
            return;
        }
        let (mut data, mut frames, mut flags) = (std::ptr::null_mut::<u8>(), 0u32, 0u32);
        slot::<Fn5>(cap, 0xc)(cap, &mut data as *mut _ as usize, &mut frames as *mut _ as usize, &mut flags as *mut _ as usize, 0, 0);
        let len = frames as usize * frame_bytes;
        if flags & 2 != 0 {
            out.extend(std::iter::repeat(0u8).take(len));
        } else {
            out.extend_from_slice(std::slice::from_raw_parts(data, len));
        }
        slot::<Fn1>(cap, 0x10)(cap, frames as usize);
    }
}

fn main() {
    let dir = std::path::PathBuf::from(std::env::args().nth(1).expect("the output folder"));
    std::fs::create_dir_all(&dir).unwrap();
    let mut tsv = std::fs::File::create(dir.join("cases.tsv")).unwrap();
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), 0);
        let endpoint = std::env::args().nth(2);
        let (client, cap, fmt) = loopback(endpoint.as_deref());
        let channels = u16::from_le_bytes([fmt[2], fmt[3]]) as usize;
        let frame_bytes = u16::from_le_bytes([fmt[12], fmt[13]]) as usize;
        writeln!(tsv, "format\t{}", fmt.iter().map(|b| format!("{b:02x}")).collect::<String>()).unwrap();
        let mut ds: *mut Obj = std::ptr::null_mut();
        let device = match &endpoint {
            Some(id) => Some(direct_sound_guid(id)),
            None => None,
        };
        let r = DirectSoundCreate8(device.as_ref().map_or(std::ptr::null(), |g| g.as_ptr()), &mut ds, std::ptr::null_mut());
        writeln!(tsv, "device	{endpoint:?}	{device:x?}	{r:x}").unwrap();
        slot::<Fn2>(ds, 0x18)(ds, GetDesktopWindow() as usize, 2);
        // The primary buffer and its listener, as the game makes them.
        let desc: [u32; 9] = [0x24, 0x91, 0, 0, 0, 0, 0, 0, 0];
        let mut primary: *mut Obj = std::ptr::null_mut();
        slot::<Fn3>(ds, 0xc)(ds, desc.as_ptr() as usize, &mut primary as *mut _ as usize, 0);
        let listener = qi(primary, &guid("279afa84-4981-11ce-a521-0020af0be560"));
        slot::<Fn0>(client, 0x28)(client);
        let mut warm = Vec::new();
        Sleep(300);
        drain(cap, frame_bytes, &mut warm);
        if std::env::var_os("DSMIX_MULTI").is_some() {
            multi(ds, cap, frame_bytes, &dir, &mut tsv);
            slot::<Fn0>(client, 0x2c)(client);
            return;
        }
        if std::env::var_os("DSMIX_SETPOS_ORDER").is_some() {
            setpos_order(ds, cap, frame_bytes, &dir, &mut tsv);
            slot::<Fn0>(client, 0x2c)(client);
            return;
        }
        if std::env::var_os("DSMIX_SETPOS").is_some() {
            setpos(ds, cap, frame_bytes, &dir, &mut tsv);
            slot::<Fn0>(client, 0x2c)(client);
            return;
        }
        for c in cases() {
            let dense = c.name.starts_with("sweep_dense");
            let frames = c.frames.unwrap_or(if dense { 2400 } else { (c.rate / 2) as usize }); // half a second
            let align = c.channels * c.bits / 8;
            let wf: [u8; 18] = {
                let mut b = [0u8; 18];
                b[0..2].copy_from_slice(&1u16.to_le_bytes());
                b[2..4].copy_from_slice(&c.channels.to_le_bytes());
                b[4..8].copy_from_slice(&c.rate.to_le_bytes());
                b[8..12].copy_from_slice(&(c.rate * u32::from(align)).to_le_bytes());
                b[12..14].copy_from_slice(&align.to_le_bytes());
                b[14..16].copy_from_slice(&c.bits.to_le_bytes());
                b
            };
            let bytes = frames as u32 * u32::from(align);
            let d: [u32; 9] = [0x24, c.flags, bytes, 0, wf.as_ptr() as u32, 0, 0, 0, 0];
            let mut b: *mut Obj = std::ptr::null_mut();
            let r = slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b as *mut _ as usize, 0);
            if b.is_null() {
                writeln!(tsv, "{}\tcreate {r:x}", c.name).unwrap();
                continue;
            }
            let mut b2: *mut Obj = std::ptr::null_mut();
            if c.other.is_some() {
                slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b2 as *mut _ as usize, 0);
            }
            let data = signal(frames, c.channels as usize, c.bits);
            let (mut p, mut n) = (std::ptr::null_mut::<u8>(), 0u32);
            slot::<Fn7>(b, 0x2c)(b, 0, bytes as usize, &mut p as *mut _ as usize, &mut n as *mut _ as usize, 0, 0, 0);
            std::ptr::copy_nonoverlapping(data.as_ptr(), p, n as usize);
            slot::<Fn4>(b, 0x4c)(b, p as usize, n as usize, 0, 0);
            let mut results = Vec::new();
            results.push(slot::<Fn1>(b, 0x3c)(b, c.volume as usize));
            if c.flags & 0x40 != 0 {
                results.push(slot::<Fn1>(b, 0x40)(b, c.pan as usize));
            }
            if c.frequency != 0 {
                results.push(slot::<Fn1>(b, 0x44)(b, c.frequency as usize));
            }
            let mut three_d: *mut Obj = std::ptr::null_mut();
            if let Some((pos, vel, min, max, rolloff, doppler)) = c.three_d {
                three_d = qi(b, &guid("279afa86-4981-11ce-a521-0020af0be560"));
                let (front, top) = c.orientation.unwrap_or(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]));
                let w = |v: f32| v.to_bits() as usize;
                results.push(slot::<Fn7>(listener, 0x34)(listener, w(front[0]), w(front[1]), w(front[2]), w(top[0]), w(top[1]), w(top[2]), 0));
                results.push(slot::<FnF3>(three_d, 0x4c)(three_d, pos[0], pos[1], pos[2], u32::from(c.uncommitted)));
                results.push(slot::<FnF3>(three_d, 0x50)(three_d, vel[0], vel[1], vel[2], 0));
                results.push(slot::<FnF1>(three_d, 0x44)(three_d, min, 0));
                results.push(slot::<FnF1>(three_d, 0x40)(three_d, max, 0));
                results.push(slot::<FnF1>(listener, 0x3c)(listener, rolloff, 0));
                results.push(slot::<FnF1>(listener, 0x30)(listener, doppler, 0));
            }
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
            // Commit any deferred 3D settings (not for an uncommitted case), then capture: what is
            // there, then the play.
            if !c.uncommitted {
                slot::<Fn0>(listener, 0x44)(listener);
            }
            let mut before = Vec::new();
            drain(cap, frame_bytes, &mut before);
            let mut out = Vec::new();
            slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
            for step in 0..if dense { 30 } else { 90 } {
                if let Some((at, f)) = c.later {
                    if step == at {
                        results.push(slot::<Fn1>(b, 0x44)(b, f as usize));
                    }
                }
                if let Some((at, f)) = c.other {
                    if step == at && !b2.is_null() {
                        results.push(slot::<Fn1>(b2, 0x44)(b2, f as usize));
                    }
                }
                Sleep(10);
                drain(cap, frame_bytes, &mut out);
            }
            slot::<Fn0>(b, 0x48)(b);
            std::fs::write(dir.join(format!("{}.f32", c.name)), &out).unwrap();
            writeln!(
                tsv,
                "{}\t{} {} {} {:x} vol {} pan {} freq {} 3d {:?} fx {:?} orientation {:?} uncommitted {} length {:?} later {:?}\tresults {:x?}\tframes {} of {channels} channels",
                c.name,
                c.channels,
                c.rate,
                c.bits,
                c.flags,
                c.volume,
                c.pan,
                c.frequency,
                c.three_d,
                c.distortion,
                c.orientation,
                c.uncommitted,
                c.frames,
                c.later,
                results,
                out.len() / frame_bytes
            )
            .unwrap();
            if !b2.is_null() {
                slot::<Fn0>(b2, 8)(b2);
            }
            for o in [fx, three_d, b] {
                if !o.is_null() {
                    slot::<Fn0>(o, 8)(o);
                }
            }
            Sleep(100);
        }
        slot::<Fn0>(client, 0x2c)(client);
    }
}

/// `DSMIX_SETPOS` (with `DSMIX_HOOK`): `SetCurrentPosition` while a buffer plays, as the game
/// calls it (`00aed080`: a byte, then `Play(DSBPLAY_LOOPING)`). Each case: a one-second buffer of
/// the test signal, `Play` (looping or not), the mix drained every 10 ms; at about 100 ms the
/// cursors, the call, the cursors again, optionally `Play(0, 0, flags)`; then about 400 ms more.
/// Writes `<case>.f32` (DirectSound's mix as it handed it to Windows), `<case>.times` (each packet
/// released: microseconds from `Play` and frames) and a line in cases.tsv: the case, its
/// settings, the call's clock (us from `Play`, before and after), its answer, the cursors
/// before and after, and the frames captured.
unsafe fn setpos(
    ds: *mut Obj,
    cap: *mut Obj,
    frame_bytes: usize,
    dir: &std::path::Path,
    tsv: &mut std::fs::File,
) {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    let us = |c: i64, origin: i64| (c - origin) * 1_000_000 / freq;
    // (name, channels, rate, bits, looping, frame, extra byte, Play flags after the call)
    type SetposCase = (&'static str, u16, u32, u16, bool, u32, u32, Option<u32>);
    let cases: [SetposCase; 14] = [
        ("s48fx_loop_half", 1, 48000, 16, true, 24000, 0, None),
        (
            "s22fx_loop_half_play",
            1,
            22050,
            16,
            true,
            11025,
            0,
            Some(1),
        ),
        ("s48_loop_half", 1, 48000, 16, true, 24000, 0, None),
        ("s48_loop_half_play", 1, 48000, 16, true, 24000, 0, Some(1)),
        ("s48_loop_odd_play", 1, 48000, 16, true, 24000, 1, Some(1)),
        ("s48_loop_near_end", 1, 48000, 16, true, 47999, 0, None),
        ("s48_once_late", 1, 48000, 16, false, 36000, 0, None),
        ("s48_once_back", 1, 48000, 16, false, 0, 0, None),
        ("s22_loop_half", 1, 22050, 16, true, 11025, 0, None),
        ("s22_loop_half_play", 1, 22050, 16, true, 11025, 0, Some(1)),
        ("s22_once_late", 1, 22050, 16, false, 16537, 0, None),
        ("s22_loop_near_end", 1, 22050, 16, true, 22049, 0, None),
        ("s44s_loop_half", 2, 44100, 16, true, 22050, 0, None),
        ("s44s_loop_half_play", 2, 44100, 16, true, 22050, 0, Some(1)),
    ];
    let read = |b: *mut Obj| {
        let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
        slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
        slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
        format!("{p:x}:{w:x}:{s:x}")
    };
    for (name, channels, rate, bits, looping, frame, extra, then) in cases {
        let frames = rate as usize;
        let align = channels * bits / 8;
        let mut wf = [0u8; 18];
        wf[0..2].copy_from_slice(&1u16.to_le_bytes());
        wf[2..4].copy_from_slice(&channels.to_le_bytes());
        wf[4..8].copy_from_slice(&rate.to_le_bytes());
        wf[8..12].copy_from_slice(&(rate * u32::from(align)).to_le_bytes());
        wf[12..14].copy_from_slice(&align.to_le_bytes());
        wf[14..16].copy_from_slice(&bits.to_le_bytes());
        let bytes = frames as u32 * u32::from(align);
        // The game's 2D flags with `DSBCAPS_GLOBALFOCUS`: the capture's window never has the
        // focus, and without it DirectSound mutes the buffer.
        let flags = 0x82e0u32;
        let d: [u32; 9] = [0x24, flags, bytes, 0, wf.as_ptr() as u32, 0, 0, 0, 0];
        let mut b: *mut Obj = std::ptr::null_mut();
        let r = slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b as *mut _ as usize, 0);
        if b.is_null() {
            writeln!(tsv, "{name}\tcreate {r:x}").unwrap();
            continue;
        }
        let data = signal(frames, channels as usize, bits);
        let (mut p, mut n) = (std::ptr::null_mut::<u8>(), 0u32);
        slot::<Fn7>(b, 0x2c)(
            b,
            0,
            bytes as usize,
            &mut p as *mut _ as usize,
            &mut n as *mut _ as usize,
            0,
            0,
            0,
        );
        std::ptr::copy_nonoverlapping(data.as_ptr(), p, n as usize);
        slot::<Fn4>(b, 0x4c)(b, p as usize, n as usize, 0, 0);
        // The distortion as the game sets it (its parameters as in `exact_cases`).
        let mut fx_results = Vec::new();
        if name.contains("fx") {
            let b8 = qi(b, &guid("6825a449-7524-4d82-920f-50e36ab3ab1e"));
            let mut ed = [0u32; 8];
            ed[0] = 0x20;
            let g = guid("ef114c90-cd1d-484e-96e5-09cfaf912a21");
            std::ptr::copy_nonoverlapping(g.as_ptr(), ed.as_mut_ptr().add(2) as *mut u8, 16);
            let mut code = 0u32;
            fx_results.push(slot::<Fn3>(b8, 0x54)(
                b8,
                1,
                ed.as_ptr() as usize,
                &mut code as *mut _ as usize,
            ));
            let iid = guid("8ecf4326-455f-4d8b-bda9-8d5d3e9e3e0b");
            let mut fx: *mut Obj = std::ptr::null_mut();
            fx_results.push(slot::<Fn4>(b8, 0x5c)(
                b8,
                g.as_ptr() as usize,
                0,
                iid.as_ptr() as usize,
                &mut fx as *mut _ as usize,
            ));
            if !fx.is_null() {
                let game = [-18.2f32, 0.0, 5000.0, 1135.0, 5000.0];
                fx_results.push(slot::<Fn1>(fx, 0xc)(fx, game.as_ptr() as usize));
                slot::<Fn0>(fx, 8)(fx);
            }
            slot::<Fn0>(b8, 8)(b8);
        }
        let mut discard = Vec::new();
        drain(cap, frame_bytes, &mut discard);
        let mut discard_times = Vec::new();
        hook::take_times(&mut discard_times);
        let at = frame * u32::from(align) + extra;
        let mut out = Vec::new();
        let origin = qpc();
        let play = slot::<Fn3>(b, 0x30)(b, 0, 0, looping as usize);
        let mut call = String::new();
        let mut done = false;
        loop {
            Sleep(10);
            drain(cap, frame_bytes, &mut out);
            let t = us(qpc(), origin);
            if !done && t >= 95_000 {
                done = true;
                let before = read(b);
                let t0 = us(qpc(), origin);
                let r = slot::<Fn1>(b, 0x34)(b, at as usize);
                let t1 = us(qpc(), origin);
                let after = read(b);
                let replay = match then {
                    Some(f) => format!(
                        "{:x} {}",
                        slot::<Fn3>(b, 0x30)(b, 0, 0, f as usize),
                        read(b)
                    ),
                    None => "- -".to_string(),
                };
                call = format!("call {t0} {t1} at {at:x} answer {r:x} before {before} after {after} play {replay}");
            }
            if t >= 500_000 {
                break;
            }
        }
        slot::<Fn0>(b, 0x48)(b);
        drain(cap, frame_bytes, &mut out);
        let mut times = Vec::new();
        hook::take_times(&mut times);
        let times: Vec<String> = times
            .iter()
            .map(|&(c, f)| format!("{} {f}", us(c, origin)))
            .collect();
        std::fs::write(dir.join(format!("{name}.f32")), &out).unwrap();
        std::fs::write(dir.join(format!("{name}.times")), times.join("\n")).unwrap();
        writeln!(
            tsv,
            "{name}\t{channels} {rate} {bits} {flags:x} looping {looping} frame {frame} extra {extra} then {then:?} fx {fx_results:x?}\tplay {play:x} {call}\tframes {} of 2 channels",
            out.len() / frame_bytes
        )
        .unwrap();
        slot::<Fn0>(b, 8)(b);
        Sleep(100);
    }
}

/// `DSMIX_SETPOS_ORDER` (with `DSMIX_HOOK`): where a buffer moved by `SetCurrentPosition` while it
/// plays is added in the sum. Three looping one-second 48 kHz mono 16-bit buffers of the test
/// signal (seeds 1, 2, 3) at their own volume and pan, played one right after the other (A, B,
/// C); at about 100 ms one of them is moved to its frame 24000; 400 ms more. Writes
/// `<case>.f32` and a line in cases.tsv: the case, the voices' volumes and pans, which one moved,
/// the call's answer and clock.
unsafe fn setpos_order(
    ds: *mut Obj,
    cap: *mut Obj,
    frame_bytes: usize,
    dir: &std::path::Path,
    tsv: &mut std::fs::File,
) {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    let us = |c: i64, origin: i64| (c - origin) * 1_000_000 / freq;
    let voices: [(i32, i32); 3] = [(-100, 0), (-600, -1000), (-2000, 3000)];
    for (name, moved) in [
        ("order_move_b", 1usize),
        ("order_move_a", 0),
        ("order_move_c", 2),
        ("order_move_b2", 1),
    ] {
        let frames = 48000usize;
        let mut wf = [0u8; 18];
        wf[0..2].copy_from_slice(&1u16.to_le_bytes());
        wf[2..4].copy_from_slice(&1u16.to_le_bytes());
        wf[4..8].copy_from_slice(&48000u32.to_le_bytes());
        wf[8..12].copy_from_slice(&96000u32.to_le_bytes());
        wf[12..14].copy_from_slice(&2u16.to_le_bytes());
        wf[14..16].copy_from_slice(&16u16.to_le_bytes());
        let bytes = frames as u32 * 2;
        let d: [u32; 9] = [0x24, 0x82e0, bytes, 0, wf.as_ptr() as u32, 0, 0, 0, 0];
        let mut bs = Vec::new();
        for (k, &(volume, pan)) in voices.iter().enumerate() {
            let mut b: *mut Obj = std::ptr::null_mut();
            slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b as *mut _ as usize, 0);
            let data = signal_seeded(frames, 1, 16, k as u32 + 1);
            let (mut p, mut n) = (std::ptr::null_mut::<u8>(), 0u32);
            slot::<Fn7>(b, 0x2c)(
                b,
                0,
                bytes as usize,
                &mut p as *mut _ as usize,
                &mut n as *mut _ as usize,
                0,
                0,
                0,
            );
            std::ptr::copy_nonoverlapping(data.as_ptr(), p, n as usize);
            slot::<Fn4>(b, 0x4c)(b, p as usize, n as usize, 0, 0);
            slot::<Fn1>(b, 0x3c)(b, volume as usize);
            slot::<Fn1>(b, 0x40)(b, pan as usize);
            bs.push(b);
        }
        let mut discard = Vec::new();
        drain(cap, frame_bytes, &mut discard);
        let mut out = Vec::new();
        let origin = qpc();
        for &b in &bs {
            slot::<Fn3>(b, 0x30)(b, 0, 0, 1);
        }
        let started = us(qpc(), origin);
        let mut call = String::new();
        loop {
            Sleep(10);
            drain(cap, frame_bytes, &mut out);
            let t = us(qpc(), origin);
            if call.is_empty() && t >= 95_000 {
                let b = bs[moved];
                let t0 = us(qpc(), origin);
                let r = slot::<Fn1>(b, 0x34)(b, 2 * 24000);
                call = format!("call {t0} answer {r:x}");
            }
            if t >= 500_000 {
                break;
            }
        }
        for &b in &bs {
            slot::<Fn0>(b, 0x48)(b);
        }
        drain(cap, frame_bytes, &mut out);
        std::fs::write(dir.join(format!("{name}.f32")), &out).unwrap();
        writeln!(tsv, "{name}	voices {voices:?} moved {moved}	played by {started} {call}	frames {} of 2 channels", out.len() / frame_bytes).unwrap();
        for b in bs {
            slot::<Fn0>(b, 8)(b);
        }
        Sleep(100);
    }
}

/// `DSMIX_MULTI`: several mono 48 kHz 16-bit voices (seeds 1, 2, ...) with their own volume and
/// pan, played one right after the other, captured together: how the voices are added.
unsafe fn multi(ds: *mut Obj, cap: *mut Obj, frame_bytes: usize, dir: &std::path::Path, tsv: &mut std::fs::File) {
    let groups: [(&str, &[(i32, i32)]); 4] = [
        ("multi2", &[(0, 0), (-600, 0)]),
        ("multi3", &[(-100, 0), (-600, -1000), (-2000, 3000)]),
        ("multi4", &[(-50, 0), (-350, 0), (-1234, 0), (-777, 0)]),
        ("multi3_loud", &[(0, 0), (-1, 0), (-2, 0)]),
    ];
    for (name, voices) in groups {
        let frames = 24000usize;
        let wf: [u8; 18] = [1, 0, 1, 0, 0x80, 0xbb, 0, 0, 0, 0x77, 1, 0, 2, 0, 16, 0, 0, 0];
        let bytes = frames as u32 * 2;
        let mut made = Vec::new();
        for (i, &(volume, pan)) in voices.iter().enumerate() {
            let d: [u32; 9] = [0x24, 0xe0 | 0x8000, bytes, 0, wf.as_ptr() as u32, 0, 0, 0, 0];
            let mut b: *mut Obj = std::ptr::null_mut();
            slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b as *mut _ as usize, 0);
            assert!(!b.is_null(), "{name}: CreateSoundBuffer");
            let mut data = signal_seeded(frames, 1, 16, i as u32 + 1);
            if name.ends_with("_loud") {
                // Near full scale, so that the sum's rounding shows.
                for s in data.chunks_mut(2) {
                    let v = i32::from(i16::from_le_bytes([s[0], s[1]])) * 30;
                    s.copy_from_slice(&(v as i16).to_le_bytes());
                }
            }
            let (mut p, mut n) = (std::ptr::null_mut::<u8>(), 0u32);
            slot::<Fn7>(b, 0x2c)(b, 0, bytes as usize, &mut p as *mut _ as usize, &mut n as *mut _ as usize, 0, 0, 0);
            std::ptr::copy_nonoverlapping(data.as_ptr(), p, n as usize);
            slot::<Fn4>(b, 0x4c)(b, p as usize, n as usize, 0, 0);
            slot::<Fn1>(b, 0x3c)(b, volume as usize);
            slot::<Fn1>(b, 0x40)(b, pan as usize);
            made.push(b);
        }
        let mut before = Vec::new();
        drain(cap, frame_bytes, &mut before);
        let mut out = Vec::new();
        for &b in &made {
            slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
        }
        for _ in 0..70 {
            Sleep(10);
            drain(cap, frame_bytes, &mut out);
        }
        for &b in &made {
            slot::<Fn0>(b, 0x48)(b);
            slot::<Fn0>(b, 8)(b);
        }
        std::fs::write(dir.join(format!("{name}.f32")), &out).unwrap();
        writeln!(tsv, "{name}\tvoices {voices:?}\tframes {}", out.len() / frame_bytes).unwrap();
        Sleep(100);
    }
}

type EnumCallback = unsafe extern "system" fn(*const [u8; 16], *const u8, *const u8, *mut c_void) -> i32;
#[link(name = "dsound")]
extern "system" {
    fn DirectSoundEnumerateA(callback: EnumCallback, context: *mut c_void) -> u32;
}

/// The DirectSound device whose description contains the text in `context`.
unsafe extern "system" fn find_device(g: *const [u8; 16], description: *const u8, _module: *const u8, context: *mut c_void) -> i32 {
    let found = &mut *(context as *mut (String, Option<[u8; 16]>));
    let text = std::ffi::CStr::from_ptr(description as *const i8).to_string_lossy();
    if !g.is_null() && text.contains(found.0.as_str()) {
        found.1 = Some(*g);
        return 0;
    }
    1
}

/// The DirectSound device GUID for the description given as the program's third argument.
fn direct_sound_guid(_id: &str) -> [u8; 16] {
    let text = std::env::args().nth(3).expect("the device's description (third argument)");
    let mut found = (text, None);
    unsafe { DirectSoundEnumerateA(find_device, &mut found as *mut _ as *mut c_void) };
    found.1.unwrap_or_else(|| panic!("no DirectSound device {}", found.0))
}

/// `DSMIX_SRC`: the sample-rate converter (`mixer::Resampler`): where a voice that is not at the
/// device's rate ends, volume and pan with it, stereo, 8-bit, and `SetFrequency` before and
/// while it plays (rates whose ratio needs the converter's table and the ones that do not).
fn src_cases(add: &mut dyn FnMut(&'static str, &dyn Fn(&mut Case))) {
    // The device check: at the device's rate, every sample as written (the exact path).
    add("src_check_48000", &|_| {});
    // Lengths around where the converter has read the last frame (44100 on 48000: 32 frames ahead).
    for (n, f) in [("src_end_22059", 22059usize), ("src_end_22081", 22081), ("src_end_22082", 22082), ("src_end_22100", 22100), ("src_end_22083", 22083)] {
        add(n, &move |c| {
            c.rate = 44100;
            c.frames = Some(f)
        });
    }
    add("src_vol_44100", &|c| {
        c.rate = 44100;
        c.volume = -600
    });
    add("src_pan_32000", &|c| {
        c.rate = 32000;
        c.volume = -1234;
        c.pan = -3000
    });
    add("src_stereo_44100", &|c| {
        c.rate = 44100;
        c.channels = 2
    });
    add("src_8bit_22050", &|c| {
        c.rate = 22050;
        c.bits = 8
    });
    // Changes of 0.2% or less keep the device on its first converter.
    for (n, rate, f) in [("src_small_44100_44150", 44100, 44150), ("src_small_22050_22071", 22050, 22071)] {
        add(n, &move |c| {
            c.rate = rate;
            c.frequency = f
        });
    }
    add("src_small_later_44100_44130", &|c| {
        c.rate = 44100;
        c.later = Some((15, 44130))
    });
    add("src_small_later_48000_48050", &|c| c.later = Some((15, 48050)));
    add("src_small_later_22071_22090", &|c| {
        c.rate = 22071;
        c.later = Some((15, 22090))
    });
    // Another buffer's large change turns on the second converter while this one plays.
    add("src_swap_44100", &|c| {
        c.rate = 44100;
        c.other = Some((15, 30000))
    });
    for (n, rate, f) in [("src_freq_22050_23456", 22050, 23456), ("src_freq_44100_44101", 44100, 44101), ("src_freq_32000_36000", 32000, 36000), ("src_freq_22050_19000", 22050, 19000)] {
        add(n, &move |c| {
            c.rate = rate;
            c.frequency = f
        });
    }
    // The frequency changes above made DirectSound turn on its other converter for the device
    // (docs/SOUND.md): the same kinds of buffers again.
    add("src_apl_48000", &|_| {});
    add("src_apl_44100", &|c| c.rate = 44100);
    add("src_apl_stereo_44100", &|c| {
        c.rate = 44100;
        c.channels = 2
    });
    add("src_apl_pan_32000", &|c| {
        c.rate = 32000;
        c.volume = -1234;
        c.pan = -3000
    });
    add("src_apl_end_22059", &|c| {
        c.rate = 44100;
        c.frames = Some(22059)
    });
    add("src_apl_end_22052", &|c| {
        c.rate = 44100;
        c.frames = Some(22052)
    });
    add("src_apl_8bit_22050", &|c| {
        c.rate = 22050;
        c.bits = 8
    });
    for (n, rate, at, f) in [("src_later_48000_44100", 48000, 15, 44100), ("src_later_22050_30000", 22050, 15, 30000), ("src_later_44100_23456", 44100, 15, 23456)] {
        add(n, &move |c| {
            c.rate = rate;
            c.later = Some((at, f))
        });
    }
}

/// `DSMIX_HOOK`: instead of the device's loopback, what DirectSound itself hands the Windows
/// audio engine, taken in this process: `IAudioRenderClient::GetBuffer` and `ReleaseBuffer` are
/// wrapped (the class's vtable is shared by every render client of the process) and each
/// released packet is kept. This is DirectSound's mix before any processing of the device's, so
/// any device serves.
mod hook {
    use super::{guid, slot, Fn2, Fn4, FnInit, Obj};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;
    pub static ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    static TAKEN: Mutex<Vec<u8>> = Mutex::new(Vec::new());
    /// Each released packet: the clock (`QueryPerformanceCounter`) and its frames.
    static TIMES: Mutex<Vec<(i64, u32)>> = Mutex::new(Vec::new());
    static LAST: AtomicUsize = AtomicUsize::new(0);
    static FRAME: AtomicUsize = AtomicUsize::new(8);
    static GET: AtomicUsize = AtomicUsize::new(0);
    static RELEASE: AtomicUsize = AtomicUsize::new(0);
    type Get = unsafe extern "system" fn(*mut Obj, u32, *mut *mut u8) -> u32;
    type Release = unsafe extern "system" fn(*mut Obj, u32, u32) -> u32;
    #[link(name = "kernel32")]
    extern "system" {
        fn VirtualProtect(at: *mut u8, size: usize, protect: u32, old: *mut u32) -> i32;
    }
    unsafe extern "system" fn get(this: *mut Obj, frames: u32, data: *mut *mut u8) -> u32 {
        let f: Get = std::mem::transmute(GET.load(Ordering::SeqCst));
        let r = f(this, frames, data);
        if r == 0 && !data.is_null() {
            LAST.store(*data as usize, Ordering::SeqCst);
        }
        r
    }
    unsafe extern "system" fn release(this: *mut Obj, frames: u32, flags: u32) -> u32 {
        let at = LAST.load(Ordering::SeqCst);
        if at != 0 {
            TIMES.lock().unwrap().push((super::qpc(), frames));
            let n = frames as usize * FRAME.load(Ordering::SeqCst);
            let mut t = TAKEN.lock().unwrap();
            if flags & 2 != 0 {
                t.extend(std::iter::repeat(0u8).take(n));
            } else {
                t.extend_from_slice(std::slice::from_raw_parts(at as *const u8, n));
            }
        }
        let f: Release = std::mem::transmute(RELEASE.load(Ordering::SeqCst));
        f(this, frames, flags)
    }
    /// Wraps the render client's slots, using a client of `dev` with the mix format `fmt`.
    pub unsafe fn install(dev: *mut Obj, fmt: *const u8, frame_bytes: usize) {
        FRAME.store(frame_bytes, Ordering::SeqCst);
        let mut client: *mut Obj = std::ptr::null_mut();
        let iid_client = guid("1cb9ad4c-dbfa-4c32-b178-c2f568a703b2");
        let r = slot::<Fn4>(dev, 0xc)(dev, iid_client.as_ptr() as usize, 0x17, 0, &mut client as *mut _ as usize);
        assert_eq!(r, 0, "Activate (render)");
        let r = slot::<FnInit>(client, 0xc)(client, 0, 0, 10_000_000, 0, fmt, std::ptr::null());
        assert_eq!(r, 0, "IAudioClient::Initialize render");
        let mut render: *mut Obj = std::ptr::null_mut();
        let iid = guid("f294acfc-3146-4483-a7bf-addca7c260e2");
        let r = slot::<Fn2>(client, 0x38)(client, iid.as_ptr() as usize, &mut render as *mut _ as usize);
        assert_eq!(r, 0, "IAudioRenderClient");
        let vt = (*render).vtable as *mut usize;
        let mut old = 0u32;
        assert!(VirtualProtect(vt.add(3) as *mut u8, 8, 0x04, &mut old) != 0);
        GET.store(*vt.add(3), Ordering::SeqCst);
        RELEASE.store(*vt.add(4), Ordering::SeqCst);
        *vt.add(3) = get as *const () as usize;
        *vt.add(4) = release as *const () as usize;
        ON.store(true, Ordering::SeqCst);
    }
    /// What DirectSound released since the last call.
    pub fn take(out: &mut Vec<u8>) {
        out.append(&mut TAKEN.lock().unwrap());
    }
    /// The packets' clocks and frames since the last call.
    pub fn take_times(out: &mut Vec<(i64, u32)>) {
        out.append(&mut TIMES.lock().unwrap());
    }
}
