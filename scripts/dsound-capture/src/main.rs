//! dsound-capture: what DirectSound 8 answers on this machine to the calls FalloutNV.exe 1.4.0.525
//! makes, as the platform's sound table (`engine_core::units::platform::dsound`), written to
//! stdout. 32-bit, as the game. The table describes the player's own sound device: it stays on
//! the player's machine (git-ignored), never in the repository. Windows only:
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/dsound-capture/Cargo.toml > %LOCALAPPDATA%\nv-rs\dsound.tsv`
//! (or any file `NV_DSOUND_TABLE` names).
//!
//! What the game calls (see `crates/engine-core/src/units/platform/dsound.rs`):
//! - `BSWin32Audio::Init` `00aea230`: `DirectSoundCreate8(NULL)`, `SetCooperativeLevel(window,
//!   DSSCL_PRIORITY)`, `GetCaps`, the primary buffer (`DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D |
//!   DSBCAPS_CTRLVOLUME`), its `IDirectSound3DListener`, `SetPosition` and `SetDopplerFactor`.
//! - Sounds (`00aeb270`/`00aec530`, `00af0630`, `00af1430`): secondary buffers of the file's PCM
//!   format with `DSBCAPS_CTRLFREQUENCY | DSBCAPS_CTRLVOLUME | DSBCAPS_CTRLFX` (the last left out
//!   under 151 ms) and either `DSBCAPS_CTRLPAN` or `DSBCAPS_CTRL3D | DSBCAPS_GETCURRENTPOSITION2 |
//!   DSBCAPS_MUTE3DATMAXDISTANCE`, `DS3DALG_HRTF_LIGHT` or none; `IDirectSoundBuffer8` and
//!   `IDirectSound3DBuffer`; `Lock`/`Unlock`; `SetFX` with the distortion or the I3DL2 reverb
//!   and `GetObjectInPath`; `Play`, `Stop`, the cursor, volume, pan and frequency.
//!
//! Lines (tab separated; words and results in hex):
//! - `DsCreate`, `DsCooperativeLevel`, `DsCaps` (the 0x60-byte `DSCAPS`), `DsPrimary`,
//!   `DsListener` (the `QueryInterface` result, then `GetAllParameters`' 0x40 bytes):
//!   `name<TAB>words<TAB>result<TAB>bytes`.
//! - `DsBuffer<TAB>flags alg tag channels rate bits bytes<TAB>result<TAB>DSBCAPS bytes<TAB>format
//!   bytes<TAB>frequency status play write<TAB>QI IDirectSoundBuffer8 result, QI
//!   IDirectSound3DBuffer result`.
//! - `DsFx<TAB>effect flags format bytes<TAB>SetFX result<TAB>its result code<TAB>
//!   GetObjectInPath result<TAB>GetAllParameters bytes`.
//! - `DsSet<TAB>method value<TAB>result<TAB>value read back`.
//! - `DsCursor<TAB>rate channels bits bytes looping frequency<TAB>samples` where the samples are
//!   `microseconds:play:write` at each change, from `Play`.
//! - `DsEnd`.
//!
//! With `replays`, `setpos`, `endphase`, `freqphase` or `endstop` as the argument, only those
//! cases (`replays`: `Play` again after the end, `Stop`, `SetCurrentPosition`; `setpos`:
//! `SetCurrentPosition` while a buffer plays, after its end and when stopped; `endphase`: `Play`
//! again after ends and stops at period counts of every remainder; `freqphase`: the steps around
//! `SetFrequency`; `endstop`: a buffer stopped at its end, and a looping one turned not looping;
//! `DsReplay`, `DsSetPos`, `DsEndPhase`, `DsFreqPhase` and `DsEndStop` lines, see the
//! functions).
use std::ffi::c_void;
use std::io::Write;

#[link(name = "dsound")]
extern "system" {
    fn DirectSoundCreate8(device: *const u8, out: *mut *mut Obj, outer: *mut c_void) -> u32;
}
#[link(name = "kernel32")]
extern "system" {
    fn QueryPerformanceCounter(out: *mut i64) -> i32;
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
    fn Sleep(ms: u32);
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

/// A COM object: a pointer to its vtable.
#[repr(C)]
struct Obj {
    vtable: *const usize,
}

/// The function in slot `slot` (byte offset / 4) of `o`'s vtable, as `F`.
unsafe fn slot<F: Copy>(o: *mut Obj, offset: usize) -> F {
    let f = *(*o).vtable.add(offset / 4);
    std::mem::transmute_copy(&f)
}

type Fn0 = unsafe extern "system" fn(*mut Obj) -> u32;
type Fn1 = unsafe extern "system" fn(*mut Obj, usize) -> u32;
type Fn2 = unsafe extern "system" fn(*mut Obj, usize, usize) -> u32;
type Fn3 = unsafe extern "system" fn(*mut Obj, usize, usize, usize) -> u32;
type Fn4 = unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize) -> u32;
type Fn7 =
    unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize, usize, usize, usize) -> u32;
type FnF1 = unsafe extern "system" fn(*mut Obj, f32, u32) -> u32;
type FnF3 = unsafe extern "system" fn(*mut Obj, f32, f32, f32, u32) -> u32;

const IID_BUFFER8: [u8; 16] = [
    0x49, 0xa4, 0x25, 0x68, 0x24, 0x75, 0x82, 0x4d, 0x92, 0x0f, 0x50, 0xe3, 0x6a, 0xb3, 0xab, 0x1e,
];
const IID_3DBUFFER: [u8; 16] = [
    0x86, 0xfa, 0x9a, 0x27, 0x81, 0x49, 0xce, 0x11, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60,
];
const IID_LISTENER: [u8; 16] = [
    0x84, 0xfa, 0x9a, 0x27, 0x81, 0x49, 0xce, 0x11, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60,
];
/// The GUIDs FalloutNV.exe passes (read from its `.rdata`: `01100ff4`, `01100fc4`, `01101004`,
/// `01100fd4`; the HRTF light algorithm from `00aeb270`'s stores).
const GUID_DISTORTION: [u8; 16] = [
    0x90, 0x4c, 0x11, 0xef, 0x1d, 0xcd, 0x4e, 0x48, 0x96, 0xe5, 0x09, 0xcf, 0xaf, 0x91, 0x2a, 0x21,
];
const IID_DISTORTION8: [u8; 16] = [
    0x26, 0x43, 0xcf, 0x8e, 0x5f, 0x45, 0x8b, 0x4d, 0xbd, 0xa9, 0x8d, 0x5d, 0x3e, 0x9e, 0x3e, 0x0b,
];
const GUID_I3DL2: [u8; 16] = [
    0x71, 0x5e, 0x98, 0xef, 0xc7, 0xd5, 0xd4, 0x42, 0xba, 0x4d, 0x2d, 0x07, 0x3e, 0x2e, 0x96, 0xf4,
];
const IID_I3DL2_8: [u8; 16] = [
    0x6a, 0x6a, 0x16, 0x4b, 0x66, 0x0d, 0xf3, 0x43, 0x80, 0xe3, 0xee, 0x62, 0x80, 0xde, 0xe1, 0xa4,
];
const ALG_HRTF_LIGHT: [u32; 4] = [0xc241_3342, 0x11d2_1c1b, 0xc000_f594, 0xca8a_c24f];

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

unsafe fn bytes_of<T>(t: &T) -> Vec<u8> {
    std::slice::from_raw_parts(t as *const T as *const u8, std::mem::size_of::<T>()).to_vec()
}

unsafe extern "system" fn wndproc(w: *mut c_void, m: u32, wp: usize, lp: isize) -> isize {
    DefWindowProcA(w, m, wp, lp)
}

unsafe fn window() -> *mut c_void {
    let name = b"dsound-capture\0";
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
    CreateWindowExA(
        0,
        name.as_ptr(),
        name.as_ptr(),
        0,
        0,
        0,
        64,
        64,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    )
}

#[repr(C)]
struct WaveFormat {
    tag: u16,
    channels: u16,
    rate: u32,
    avg: u32,
    align: u16,
    bits: u16,
    extra: u16,
}

#[repr(C)]
struct BufferDesc {
    size: u32,
    flags: u32,
    bytes: u32,
    reserved: u32,
    format: *const WaveFormat,
    alg: [u32; 4],
}

fn format(channels: u16, rate: u32, bits: u16) -> WaveFormat {
    let align = channels * bits / 8;
    WaveFormat {
        tag: 1,
        channels,
        rate,
        avg: rate * align as u32,
        align,
        bits,
        extra: 0,
    }
}

/// A secondary buffer as the game asks for it; the result and the buffer (null on failure).
unsafe fn create(
    ds: *mut Obj,
    flags: u32,
    alg: bool,
    f: &WaveFormat,
    bytes: u32,
) -> (u32, *mut Obj) {
    let desc = BufferDesc {
        size: 0x24,
        flags,
        bytes,
        reserved: 0,
        format: f,
        alg: if alg { ALG_HRTF_LIGHT } else { [0; 4] },
    };
    let mut b: *mut Obj = std::ptr::null_mut();
    let hr = slot::<Fn3>(ds, 0xc)(ds, &desc as *const _ as usize, &mut b as *mut _ as usize, 0);
    (
        hr,
        if hr as i32 >= 0 {
            b
        } else {
            std::ptr::null_mut()
        },
    )
}

unsafe fn release(o: *mut Obj) {
    if !o.is_null() {
        slot::<Fn0>(o, 8)(o);
    }
}

unsafe fn qi(o: *mut Obj, iid: &[u8; 16]) -> (u32, *mut Obj) {
    let mut out: *mut Obj = std::ptr::null_mut();
    let hr = slot::<Fn2>(o, 0)(o, iid.as_ptr() as usize, &mut out as *mut _ as usize);
    (hr, out)
}

fn now_us(freq: i64, origin: i64) -> i64 {
    let mut c = 0i64;
    unsafe { QueryPerformanceCounter(&mut c) };
    (c - origin) * 1_000_000 / freq
}

/// The flag sets the game uses: 2D (`CTRLPAN`) and 3D, each with and without `CTRLFX`.
const FLAG_SETS: [u32; 4] = [0x2e0, 0xe0, 0x302b0, 0x300b0];
const RATES: [u32; 7] = [8000, 11025, 16000, 22050, 32000, 44100, 48000];

unsafe fn buffers(o: &mut impl Write, ds: *mut Obj) {
    for flags in FLAG_SETS {
        let algs: &[bool] = if flags & 0x10 != 0 {
            &[false, true]
        } else {
            &[false]
        };
        for &alg in algs {
            for channels in [1u16, 2] {
                for rate in RATES {
                    for bits in [8u16, 16] {
                        let f = format(channels, rate, bits);
                        let a = f.align as u32;
                        let ms = |m: u32| (f.avg * m / 1000) / a * a;
                        let mut sizes = vec![0, 1, a, 2 * a, 3, 4, 100 * a, ms(1000), ms(250)];
                        // Around 150 ms, block by block.
                        let c = ms(150);
                        for d in 0..6u32 {
                            sizes.push(c + d * a);
                            sizes.push(c.saturating_sub((d + 1) * a));
                        }
                        sizes.sort_unstable();
                        sizes.dedup();
                        for bytes in sizes {
                            buffer(o, ds, flags, alg, &f, bytes);
                        }
                    }
                }
            }
        }
    }
}

unsafe fn buffer(
    o: &mut impl Write,
    ds: *mut Obj,
    flags: u32,
    alg: bool,
    f: &WaveFormat,
    bytes: u32,
) {
    let (hr, b) = create(ds, flags, alg, f, bytes);
    let words = format!(
        "{flags:x} {} {:x} {:x} {:x} {:x} {bytes:x}",
        u32::from(alg),
        f.tag,
        f.channels,
        f.rate,
        f.bits
    );
    if b.is_null() {
        writeln!(o, "DsBuffer\t{words}\t{hr:x}\t\t\t\t").unwrap();
        return;
    }
    let mut caps = [0u32; 5];
    caps[0] = 0x14;
    slot::<Fn1>(b, 0xc)(b, caps.as_mut_ptr() as usize);
    let mut fmt = [0u8; 0x12];
    let mut written = 0u32;
    slot::<Fn3>(b, 0x14)(
        b,
        fmt.as_mut_ptr() as usize,
        0x12,
        &mut written as *mut _ as usize,
    );
    let mut freq = 0u32;
    slot::<Fn1>(b, 0x20)(b, &mut freq as *mut _ as usize);
    let mut status = 0u32;
    slot::<Fn1>(b, 0x24)(b, &mut status as *mut _ as usize);
    let (mut play, mut write) = (0u32, 0u32);
    slot::<Fn2>(b, 0x10)(
        b,
        &mut play as *mut _ as usize,
        &mut write as *mut _ as usize,
    );
    let (q8, b8) = qi(b, &IID_BUFFER8);
    let (q3, b3) = qi(b, &IID_3DBUFFER);
    let caps_hex = hex(&bytes_of(&caps));
    writeln!(
        o,
        "DsBuffer\t{words}\t{hr:x}\t{caps_hex}\t{} {written:x}\t{freq:x} {status:x} {play:x} {write:x}\t{q8:x} {q3:x}",
        hex(&fmt)
    )
    .unwrap();
    release(b3);
    release(b8);
    release(b);
}

/// `DSEFFECTDESC` as the game fills it.
#[repr(C)]
struct EffectDesc {
    size: u32,
    flags: u32,
    class: [u8; 16],
    reserved1: u32,
    reserved2: u32,
}

unsafe fn effects(o: &mut impl Write, ds: *mut Obj) {
    for (name, class, iid, params) in [
        ("distortion", GUID_DISTORTION, IID_DISTORTION8, 0x14usize),
        ("i3dl2", GUID_I3DL2, IID_I3DL2_8, 0x30usize),
    ] {
        for flags in [0x2e0u32, 0x302b0, 0xe0] {
            for (channels, rate) in [(1u16, 22050u32), (2, 44100), (1, 44100), (1, 48000)] {
                let f = format(channels, rate, 16);
                let bytes = f.avg; // one second
                let (hr, b) = create(ds, flags, false, &f, bytes);
                if b.is_null() {
                    writeln!(o, "DsFx\t{name} {flags:x} {channels:x} {rate:x} {bytes:x}\tcreate {hr:x}\t\t\t").unwrap();
                    continue;
                }
                let (_, b8) = qi(b, &IID_BUFFER8);
                let desc = EffectDesc {
                    size: 0x20,
                    flags: 0,
                    class,
                    reserved1: 0,
                    reserved2: 0,
                };
                let mut code = 0xffff_ffffu32;
                let r = slot::<Fn3>(b8, 0x54)(
                    b8,
                    1,
                    &desc as *const _ as usize,
                    &mut code as *mut _ as usize,
                );
                let mut fx: *mut Obj = std::ptr::null_mut();
                let g = slot::<Fn4>(b8, 0x5c)(
                    b8,
                    class.as_ptr() as usize,
                    0,
                    iid.as_ptr() as usize,
                    &mut fx as *mut _ as usize,
                );
                let mut all = vec![0u8; params];
                if !fx.is_null() {
                    slot::<Fn1>(fx, 0x10)(fx, all.as_mut_ptr() as usize);
                }
                writeln!(
                    o,
                    "DsFx\t{name} {flags:x} {channels:x} {rate:x} {bytes:x}\t{r:x}\t{code:x}\t{g:x}\t{}",
                    hex(&all)
                )
                .unwrap();
                if !fx.is_null() {
                    fx_settings(o, name, fx);
                    release(fx);
                }
                release(b8);
                release(b);
            }
        }
    }
}

/// The effect settings the game makes: the distortion's `SetAllParameters` (gain from the INI,
/// edge 0, post-EQ 5000 Hz / 1135 Hz, pre-lowpass 5000 Hz) over a range of gains, and the
/// reverb's `SetPreset` over every preset number and a few past them.
unsafe fn fx_settings(o: &mut impl Write, name: &str, fx: *mut Obj) {
    if name == "distortion" {
        for gain in [-61.0f32, -60.0, -40.0, -18.0, -10.0, 0.0, 0.5] {
            let p: [f32; 5] = [gain, 0.0, 5000.0, 1135.0, 5000.0];
            let r = slot::<Fn1>(fx, 0xc)(fx, p.as_ptr() as usize);
            let mut back = [0u8; 0x14];
            slot::<Fn1>(fx, 0x10)(fx, back.as_mut_ptr() as usize);
            writeln!(
                o,
                "DsSet\tdistortion {:08x}\t{r:x}\t{}",
                gain.to_bits(),
                hex(&back)
            )
            .unwrap();
        }
    } else {
        for preset in 0..32u32 {
            let r = slot::<Fn1>(fx, 0x14)(fx, preset as usize);
            let mut back = [0u8; 0x30];
            slot::<Fn1>(fx, 0x10)(fx, back.as_mut_ptr() as usize);
            writeln!(o, "DsSet\ti3dl2preset {preset:x}\t{r:x}\t{}", hex(&back)).unwrap();
        }
    }
}

/// Volume, pan and frequency at the edges of their ranges, on a 2D and a 3D buffer.
unsafe fn settings(o: &mut impl Write, ds: *mut Obj) {
    for flags in [0x2e0u32, 0x302b0] {
        let f = format(1, 22050, 16);
        let (_, b) = create(ds, flags, false, &f, f.avg);
        for v in [-10001i32, -10000, -2500, -1, 0, 1] {
            let r = slot::<Fn1>(b, 0x3c)(b, v as usize);
            let mut back = 0i32;
            slot::<Fn1>(b, 0x18)(b, &mut back as *mut _ as usize);
            writeln!(o, "DsSet\t{flags:x} volume {v:x}\t{r:x}\t{back:x}").unwrap();
        }
        for v in [-10001i32, -10000, 0, 10000, 10001] {
            let r = slot::<Fn1>(b, 0x40)(b, v as usize);
            let mut back = 0i32;
            slot::<Fn1>(b, 0x1c)(b, &mut back as *mut _ as usize);
            writeln!(o, "DsSet\t{flags:x} pan {v:x}\t{r:x}\t{back:x}").unwrap();
        }
        for v in [0u32, 1, 99, 100, 11025, 22050, 100000, 200000, 200001] {
            let r = slot::<Fn1>(b, 0x44)(b, v as usize);
            let mut back = 0u32;
            slot::<Fn1>(b, 0x20)(b, &mut back as *mut _ as usize);
            writeln!(o, "DsSet\t{flags:x} frequency {v:x}\t{r:x}\t{back:x}").unwrap();
        }
        // Lock as the game does: (0, size, &p, &n, NULL, NULL, 0) and the whole buffer.
        let (mut p1, mut n1, mut p2, mut n2) = (0usize, 0u32, 0usize, 0u32);
        let r = slot::<Fn7>(b, 0x2c)(
            b,
            0,
            f.avg as usize,
            &mut p1 as *mut _ as usize,
            &mut n1 as *mut _ as usize,
            0,
            0,
            0,
        );
        writeln!(o, "DsSet\t{flags:x} lock 0 {:x} 0\t{r:x}\t{n1:x}", f.avg).unwrap();
        let u = slot::<Fn4>(b, 0x4c)(b, p1, n1 as usize, 0, 0);
        writeln!(o, "DsSet\t{flags:x} unlock\t{u:x}\t").unwrap();
        let r = slot::<Fn7>(b, 0x2c)(
            b,
            0,
            0,
            &mut p1 as *mut _ as usize,
            &mut n1 as *mut _ as usize,
            &mut p2 as *mut _ as usize,
            &mut n2 as *mut _ as usize,
            2,
        );
        writeln!(
            o,
            "DsSet\t{flags:x} lock 0 0 2\t{r:x}\t{n1:x} {} {n2:x}",
            u32::from(p2 != 0)
        )
        .unwrap();
        slot::<Fn4>(b, 0x4c)(b, p1, n1 as usize, p2, n2 as usize);
        if flags & 0x10 != 0 {
            let (q, b3) = qi(b, &IID_3DBUFFER);
            let mut all = [0u8; 0x40];
            all[0] = 0x40;
            let g = slot::<Fn1>(b3, 0xc)(b3, all.as_mut_ptr() as usize);
            writeln!(o, "DsSet\t3dbuffer defaults\t{q:x} {g:x}\t{}", hex(&all)).unwrap();
            for (name, offset, v) in [
                ("mindistance", 0x44usize, 1.0e9f32),
                ("maxdistance", 0x40, 1.0e9),
                ("mindistance", 0x44, 0.0),
                ("maxdistance", 0x40, -1.0),
            ] {
                let r = slot::<FnF1>(b3, offset)(b3, v, 1);
                writeln!(o, "DsSet\t3dbuffer {name} {:08x}\t{r:x}\t", v.to_bits()).unwrap();
            }
            let r = slot::<FnF3>(b3, 0x4c)(b3, -3.4e37, -3.4e37, -3.4e37, 1);
            writeln!(o, "DsSet\t3dbuffer position fdcccccc\t{r:x}\t").unwrap();
            release(b3);
        }
        release(b);
    }
}

/// The play and write cursors from `Play` on, at each change, for about 600 ms.
unsafe fn cursors(o: &mut impl Write, ds: *mut Obj) {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    for (channels, rate, bits, ms, looping, frequency) in [
        (1u16, 22050u32, 16u16, 1000u32, 1u32, 0u32),
        (2, 44100, 16, 1000, 1, 0),
        (1, 44100, 16, 1000, 1, 0),
        (2, 48000, 16, 1000, 1, 0),
        (1, 11025, 8, 1000, 1, 0),
        (1, 22050, 16, 300, 0, 0),
        (1, 22050, 16, 1000, 1, 11025),
        (1, 22050, 16, 1000, 1, 44100),
    ] {
        let f = format(channels, rate, bits);
        let bytes = f.avg * ms / 1000 / f.align as u32 * f.align as u32;
        let (_, b) = create(ds, 0x100e0 | 0x200, false, &f, bytes);
        if frequency != 0 {
            slot::<Fn1>(b, 0x44)(b, frequency as usize);
        }
        let mut origin = 0i64;
        let mut samples = Vec::new();
        let (mut last_p, mut last_w, mut last_s) = (u32::MAX, u32::MAX, u32::MAX);
        QueryPerformanceCounter(&mut origin);
        slot::<Fn3>(b, 0x30)(b, 0, 0, looping as usize);
        loop {
            let t = now_us(freq, origin);
            if t > 600_000 {
                break;
            }
            let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
            slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
            slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
            if (p, w, s) != (last_p, last_w, last_s) {
                samples.push(format!("{t}:{p:x}:{w:x}:{s:x}"));
                (last_p, last_w, last_s) = (p, w, s);
            }
        }
        slot::<Fn0>(b, 0x48)(b);
        let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
        slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
        slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
        writeln!(
            o,
            "DsCursor\t{rate:x} {channels:x} {bits:x} {bytes:x} {looping:x} {frequency:x}\t{}\tstopped {p:x}:{w:x}:{s:x}",
            samples.join(" ")
        )
        .unwrap();
        release(b);
        Sleep(50);
    }
}

/// `Play` again on a non-looping buffer, after:
/// - `ended`: it played to its end (both cursors read 0, status 0);
/// - `stopped`: it played to its end, then `Stop`;
/// - `setpos`: it played to its end, then `SetCurrentPosition(0)`;
/// - `again`: it played to its end, was played again to its end, then once more;
/// - `midstop`: it played for 50 ms, then `Stop` and `SetCurrentPosition(0)`;
/// - `fresh`: never played.
///
/// Before the last `Play` the cursors and status are written (`before`), then every change for
/// 500 ms after it.
unsafe fn replays(o: &mut impl Write, ds: *mut Obj) {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    let read = |b: *mut Obj| {
        let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
        slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
        slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
        (p, w, s)
    };
    let play_to_end = |b: *mut Obj| {
        slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
        let mut origin = 0i64;
        QueryPerformanceCounter(&mut origin);
        while read(b).2 & 1 != 0 && now_us(freq, origin) < 2_000_000 {}
    };
    for (channels, rate, bits) in [(1u16, 22050u32, 16u16), (2, 44100, 16)] {
        for variant in ["ended", "stopped", "setpos", "again", "midstop", "fresh"] {
            let f = format(channels, rate, bits);
            let bytes = f.avg * 200 / 1000 / f.align as u32 * f.align as u32;
            let (_, b) = create(ds, 0x100e0 | 0x200, false, &f, bytes);
            let mut answers = Vec::new();
            match variant {
                "ended" => play_to_end(b),
                "stopped" => {
                    play_to_end(b);
                    answers.push(slot::<Fn0>(b, 0x48)(b));
                }
                "setpos" => {
                    play_to_end(b);
                    answers.push(slot::<Fn1>(b, 0x34)(b, 0));
                }
                "again" => {
                    play_to_end(b);
                    play_to_end(b);
                }
                "midstop" => {
                    slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
                    Sleep(50);
                    answers.push(slot::<Fn0>(b, 0x48)(b));
                    answers.push(slot::<Fn1>(b, 0x34)(b, 0));
                }
                _ => {}
            }
            let before = read(b);
            let mut origin = 0i64;
            QueryPerformanceCounter(&mut origin);
            let hr = slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
            let mut samples = Vec::new();
            let mut last = (u32::MAX, u32::MAX, u32::MAX);
            loop {
                let t = now_us(freq, origin);
                if t > 500_000 {
                    break;
                }
                let now = read(b);
                if now != last {
                    samples.push(format!("{t}:{:x}:{:x}:{:x}", now.0, now.1, now.2));
                    last = now;
                }
            }
            let answers: Vec<String> = answers.iter().map(|a| format!("{a:x}")).collect();
            writeln!(
                o,
                "DsReplay\t{rate:x} {channels:x} {bits:x} {bytes:x} {variant}\tanswers {} before {:x}:{:x}:{:x} play {hr:x}\t{}",
                answers.join(","),
                before.0,
                before.1,
                before.2,
                samples.join(" ")
            )
            .unwrap();
            slot::<Fn0>(b, 0x48)(b);
            release(b);
            Sleep(50);
        }
    }
}

/// `SetCurrentPosition` while a buffer plays (and after it played to its end), as the game calls
/// it: `00aed080` a random byte then `Play(DSBPLAY_LOOPING)`, `00aee580` a byte while it plays.
/// Each case: a buffer of `ms` ms with `flags`, `Play` (looping or not), at `when` us after it the
/// cursors (`before`), `SetCurrentPosition(at)` (its answer; the clock before and after the call),
/// at once the cursors again (`after`), optionally `Play(0, 0, flags)` (its answer) and the cursors
/// (`replay`), then every change of the cursors and status from `Play` on for `when` + 450 ms.
/// Lines: `DsSetPos<TAB>rate channels bits bytes flags variant when<TAB>at answer play-answer
/// <TAB>before after replay<TAB>scp-t0 scp-t1<TAB>samples` (samples `t:play:write:status`).
unsafe fn setpos(o: &mut impl Write, ds: *mut Obj) {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    let read = |b: *mut Obj| {
        let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
        slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
        slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
        (p, w, s)
    };
    let show = |c: (u32, u32, u32)| format!("{:x}:{:x}:{:x}", c.0, c.1, c.2);
    // (variant, looping, at as a function of (bytes, align), Play after it (flags), when)
    type At = fn(u32, u32) -> u32;
    let half: At = |b, a| b / 2 / a * a;
    let half_odd: At = |b, a| b / 2 / a * a + 1;
    let quarter3: At = |b, a| b * 3 / 4 / a * a;
    let zero: At = |_, _| 0;
    let near_end: At = |b, a| b - a;
    let last_byte: At = |b, _| b - 1;
    let end: At = |b, _| b;
    let past: At = |b, a| b + a;
    let low: At = |_, a| 100 * a;
    let mut cases: Vec<(&str, bool, At, Option<u32>, u32)> = Vec::new();
    for when in [100_000u32, 103_300, 106_600] {
        cases.push(("loop_half", true, half, None, when));
        cases.push(("loop_half_play", true, half, Some(1), when));
    }
    for (name, looping, at, then) in [
        ("loop_odd", true, half_odd, None),
        ("loop_odd_play", true, half_odd, Some(1)),
        ("loop_zero", true, zero, None),
        ("loop_low", true, low, None),
        ("loop_near_end", true, near_end, None),
        ("loop_last_byte", true, last_byte, None),
        ("loop_end", true, end, None),
        ("loop_past", true, past, None),
        ("loop_half_play_once", true, half, Some(0)),
        ("once_late", false, quarter3, None),
        ("once_late_play", false, quarter3, Some(0)),
        ("once_late_play_loop", false, quarter3, Some(1)),
        ("once_back", false, zero, None),
        ("once_near_end", false, near_end, None),
        ("once_end", false, end, None),
        ("once_past", false, past, None),
    ] {
        cases.push((name, looping, at, then, 100_000));
    }
    for (channels, rate, bits, flags, all) in [
        (1u16, 22050u32, 16u16, 0x102e0u32, true),
        (2, 44100, 16, 0x102e0, true),
        (1, 11025, 8, 0x102e0, false),
        (1, 22050, 16, 0x2e0, false),
        (1, 22050, 16, 0x302b0, false),
    ] {
        let f = format(channels, rate, bits);
        let a = f.align as u32;
        let bytes = f.avg / a * a; // one second
        for &(variant, looping, at_of, then, when) in &cases {
            if !all
                && !matches!(
                    variant,
                    "loop_half" | "loop_half_play" | "once_late" | "loop_odd"
                )
            {
                continue;
            }
            let (_, b) = create(ds, flags, false, &f, bytes);
            if b.is_null() {
                writeln!(o, "DsSetPos\t{rate:x} {channels:x} {bits:x} {bytes:x} {flags:x} {variant} {when}\tcreate failed").unwrap();
                continue;
            }
            let at = at_of(bytes, a);
            let mut origin = 0i64;
            QueryPerformanceCounter(&mut origin);
            slot::<Fn3>(b, 0x30)(b, 0, 0, looping as usize);
            let mut samples = Vec::new();
            let mut last = (u32::MAX, u32::MAX, u32::MAX);
            let sample = |samples: &mut Vec<String>, last: &mut (u32, u32, u32), t: i64| {
                let now = read(b);
                if now != *last {
                    samples.push(format!("{t}:{}", show(now)));
                    *last = now;
                }
            };
            loop {
                let t = now_us(freq, origin);
                if t >= i64::from(when) {
                    break;
                }
                sample(&mut samples, &mut last, t);
            }
            let before = read(b);
            let t0 = now_us(freq, origin);
            let hr = slot::<Fn1>(b, 0x34)(b, at as usize);
            let t1 = now_us(freq, origin);
            let after = read(b);
            let (play_hr, replay) = match then {
                Some(pf) => {
                    let r = slot::<Fn3>(b, 0x30)(b, 0, 0, pf as usize);
                    (format!("{r:x}"), show(read(b)))
                }
                None => ("-".to_string(), "-".to_string()),
            };
            samples.push(format!("|scp{t0}"));
            last = (u32::MAX, u32::MAX, u32::MAX);
            loop {
                let t = now_us(freq, origin);
                if t > i64::from(when) + 450_000 {
                    break;
                }
                sample(&mut samples, &mut last, t);
            }
            writeln!(
                o,
                "DsSetPos\t{rate:x} {channels:x} {bits:x} {bytes:x} {flags:x} {variant} {when}\t{at:x} {hr:x} {play_hr}\t{} {} {replay}\t{t0} {t1}\t{}",
                show(before),
                show(after),
                samples.join(" ")
            )
            .unwrap();
            slot::<Fn0>(b, 0x48)(b);
            release(b);
            Sleep(50);
        }
        // After the buffer played to its end (a 200 ms buffer, not looping): SetCurrentPosition,
        // the cursors for 50 ms, then Play and every change for 400 ms. And a stopped buffer
        // that never played set to an odd byte, then played.
        for (variant, at_of) in [
            ("ended_half", half),
            ("ended_odd", half_odd),
            ("ended_zero", zero),
            ("ended_end", end),
            ("fresh_odd", half_odd),
            ("fresh_last_byte", last_byte),
            ("fresh_end", end),
            ("fresh_past", past),
            ("stopped_end", end),
            ("stopped_half", half),
        ] {
            let bytes = f.avg * 200 / 1000 / a * a;
            let (_, b) = create(ds, flags, false, &f, bytes);
            if variant.starts_with("stopped") {
                slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
                Sleep(50);
                slot::<Fn0>(b, 0x48)(b);
            }
            if variant.starts_with("ended") {
                slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
                let mut origin = 0i64;
                QueryPerformanceCounter(&mut origin);
                while read(b).2 & 1 != 0 && now_us(freq, origin) < 2_000_000 {}
                Sleep(20);
            }
            let at = at_of(bytes, a);
            let before = read(b);
            let hr = slot::<Fn1>(b, 0x34)(b, at as usize);
            let after = read(b);
            Sleep(50);
            let later = read(b);
            let mut origin = 0i64;
            QueryPerformanceCounter(&mut origin);
            let play_hr = slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
            let mut samples = Vec::new();
            let mut last = (u32::MAX, u32::MAX, u32::MAX);
            loop {
                let t = now_us(freq, origin);
                if t > 400_000 {
                    break;
                }
                let now = read(b);
                if now != last {
                    samples.push(format!("{t}:{}", show(now)));
                    last = now;
                }
            }
            writeln!(
                o,
                "DsSetPos\t{rate:x} {channels:x} {bits:x} {bytes:x} {flags:x} {variant} 0\t{at:x} {hr:x} {play_hr:x}\t{} {} {}\t-\t{}",
                show(before),
                show(after),
                show(later),
                samples.join(" ")
            )
            .unwrap();
            slot::<Fn0>(b, 0x48)(b);
            release(b);
            Sleep(50);
        }
    }
}

/// `Play` again after a buffer played to its end or was stopped, after a number of periods that
/// is not a multiple of 4 (11025 Hz, 110.25 frames a period) or of 2 (22050 Hz): does the next
/// `Play`'s write cursor go on in the steps the buffer was taking, or start them over?
/// - `ended`, `ended_setpos`, `again`: buffers of several lengths played to their end (every
///   change written, `first`), `again` once more to its end (`second`); `ended_setpos` then
///   moved to its middle; then `Play` and every change for 300 ms (`replay`).
/// - `stopped`: a one-second buffer stopped after `k` periods (counted from its write cursor's
///   changes), then `Play` and every change for 300 ms.
///
/// Lines: `DsEndPhase<TAB>rate channels bits bytes variant k<TAB>first<TAB>second<TAB>before
/// <TAB>replay` (samples `t:play:write:status`, `-` for none).
unsafe fn endphase(o: &mut impl Write, ds: *mut Obj) {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    let read = |b: *mut Obj| {
        let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
        slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
        slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
        (p, w, s)
    };
    let show = |c: (u32, u32, u32)| format!("{:x}:{:x}:{:x}", c.0, c.1, c.2);
    // Every change from now until `until` says stop (or `limit` us), with times from `origin`.
    let watch = |b: *mut Obj, limit: i64, until: &dyn Fn((u32, u32, u32)) -> bool| {
        let mut origin = 0i64;
        QueryPerformanceCounter(&mut origin);
        let mut samples = Vec::new();
        let mut last = (u32::MAX, u32::MAX, u32::MAX);
        loop {
            let t = now_us(freq, origin);
            let now = read(b);
            if now != last {
                samples.push(format!("{t}:{}", show(now)));
                last = now;
            }
            if until(now) || t > limit {
                break;
            }
        }
        samples
    };
    let join = |s: &[String]| {
        if s.is_empty() {
            "-".to_string()
        } else {
            s.join(" ")
        }
    };
    for (channels, rate, bits, lengths) in [
        (
            1u16,
            11025u32,
            8u16,
            &[2205u32, 2260, 2300, 2350, 2400, 2450, 2500, 2550][..],
        ),
        (1, 22050, 16, &[4300, 4350, 4410, 4450, 4500, 4600][..]),
    ] {
        let f = format(channels, rate, bits);
        let a = f.align as u32;
        for &frames in lengths {
            for variant in ["ended", "ended_setpos", "again"] {
                let bytes = frames * a;
                let (_, b) = create(ds, 0x102e0, false, &f, bytes);
                slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
                let first = watch(b, 2_000_000, &|c| c.2 & 1 == 0);
                let mut second = Vec::new();
                if variant == "again" {
                    slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
                    second = watch(b, 2_000_000, &|c| c.2 & 1 == 0);
                }
                Sleep(20);
                if variant == "ended_setpos" {
                    slot::<Fn1>(b, 0x34)(b, (bytes / 2 / a * a) as usize);
                }
                let before = read(b);
                slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
                let replay = watch(b, 300_000, &|_| false);
                writeln!(
                    o,
                    "DsEndPhase\t{rate:x} {channels:x} {bits:x} {bytes:x} {variant} -\t{}\t{}\t{}\t{}",
                    join(&first),
                    join(&second),
                    show(before),
                    join(&replay)
                )
                .unwrap();
                slot::<Fn0>(b, 0x48)(b);
                release(b);
                Sleep(30);
            }
        }
        for k in [3usize, 5, 6, 7, 9, 10] {
            let bytes = f.avg / a * a;
            let (_, b) = create(ds, 0x102e0, false, &f, bytes);
            slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
            // Until the write cursor has changed k times.
            let changes = std::cell::Cell::new(0usize);
            let last = std::cell::Cell::new(0u32);
            let first = watch(b, 1_000_000, &|c| {
                if c.1 != last.get() {
                    last.set(c.1);
                    changes.set(changes.get() + 1);
                }
                changes.get() >= k
            });
            slot::<Fn0>(b, 0x48)(b);
            let before = read(b);
            slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
            let replay = watch(b, 300_000, &|_| false);
            writeln!(
                o,
                "DsEndPhase\t{rate:x} {channels:x} {bits:x} {bytes:x} stopped {k}\t{}\t-\t{}\t{}",
                join(&first),
                show(before),
                join(&replay)
            )
            .unwrap();
            slot::<Fn0>(b, 0x48)(b);
            release(b);
            Sleep(30);
        }
    }
}

/// The write cursor's steps around `SetFrequency` (and after an end then `Stop`), at 11025 Hz
/// 8-bit and 22050 Hz 16-bit and frequencies whose frames per period are not whole. Every case
/// records every change of the cursors and status from its first `Play`, with markers `|f`
/// (`SetFrequency`), `|stop`, `|end` (the status read 0) and `|play` (`Play` again), each with
/// its time:
/// - `steady`: the frequency set before the first `Play`;
/// - `retune_stop m1 m2`: `SetFrequency` after `m1` periods (write cursor changes), `Stop` after
///   `m2` more, `Play`;
/// - `retune_end m1`: a 300 ms buffer, `SetFrequency` after `m1` periods, played to its end,
///   `Play`;
/// - `stop_retune m1`: `Stop` after `m1` periods, `SetFrequency`, `Play`;
/// - `ended_stop`: no new frequency; a buffer of the given length played to its end, `Stop`,
///   `Play`;
/// - `stopped m1`: no new frequency; `Stop` after `m1` periods, `Play`.
///
/// The first frequency change of more than 1/500 of the rate turns on DirectSound's second
/// converter for the device (`dsound` `100581e0`); the cases run in three parts: before it
/// (`dmo`), the large changes (`big`), and after it (`apl`). Lines: `DsFreqPhase<TAB>rate
/// channels bits bytes variant frequency m1 m2 part<TAB>answers<TAB>samples`.
unsafe fn freqphase(o: &mut impl Write, ds: *mut Obj) {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    let read = |b: *mut Obj| {
        let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
        slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
        slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
        (p, w, s)
    };
    // Every change into `s` (us from `origin`) until `until(write changes, cursors)` or `limit`.
    let watch = |b: *mut Obj,
                 origin: i64,
                 s: &mut Vec<String>,
                 limit: i64,
                 until: &dyn Fn(usize, (u32, u32, u32)) -> bool| {
        let (mut last, mut changes) = ((u32::MAX, u32::MAX, u32::MAX), 0usize);
        loop {
            let t = now_us(freq, origin);
            let now = read(b);
            if now != last {
                if last.0 != u32::MAX && now.1 != last.1 {
                    changes += 1;
                }
                s.push(format!("{t}:{:x}:{:x}:{:x}", now.0, now.1, now.2));
                last = now;
            }
            if until(changes, now) || t > limit {
                break;
            }
        }
    };
    let mark = |s: &mut Vec<String>, what: &str, origin: i64| {
        s.push(format!("|{what}@{}", now_us(freq, origin)));
    };
    let fmt11 = (1u16, 11025u32, 8u16);
    let fmt22 = (1u16, 22050u32, 16u16);
    type Case = (
        (u16, u32, u16),
        &'static str,
        u32,
        usize,
        usize,
        u32,
        &'static str,
    );
    let mut cases: Vec<Case> = Vec::new();
    // (format, variant, frequency, m1, m2, length in ms (0: the frames given in m2), part)
    for frames in [2260usize, 2350, 2450, 2550] {
        cases.push((fmt11, "ended_stop", 0, 0, frames, 0, "dmo"));
    }
    for part in ["dmo", "big", "apl"] {
        let pairs: &[((u16, u32, u16), u32)] = match part {
            "dmo" => &[(fmt11, 11040), (fmt22, 22071)],
            "big" => &[(fmt11, 16537), (fmt22, 11025)],
            _ => &[(fmt11, 11040), (fmt22, 22071)],
        };
        if part == "apl" {
            for f in [fmt11, fmt22] {
                cases.push((f, "steady", 0, 0, 0, 1000, part));
                cases.push((f, "stopped", 0, 5, 0, 1000, part));
            }
            for frames in [2260usize, 2350] {
                cases.push((fmt11, "ended_stop", 0, 0, frames, 0, part));
            }
        }
        for &(f, to) in pairs {
            cases.push((f, "steady", to, 0, 0, 1000, part));
            cases.push((f, "retune_stop", to, 5, 3, 1000, part));
            cases.push((f, "retune_stop", to, 6, 1, 1000, part));
            cases.push((f, "retune_stop", to, 3, 4, 1000, part));
            cases.push((f, "retune_end", to, 5, 0, 300, part));
            cases.push((f, "stop_retune", to, 5, 0, 1000, part));
            cases.push((f, "stop_retune", to, 6, 0, 1000, part));
        }
    }
    for ((channels, rate, bits), variant, to, m1, m2, ms, part) in cases {
        let f = format(channels, rate, bits);
        let a = f.align as u32;
        let bytes = if ms == 0 {
            m2 as u32 * a
        } else {
            f.avg * ms / 1000 / a * a
        };
        let (_, b) = create(ds, 0x102e0, false, &f, bytes);
        let mut answers = Vec::new();
        let mut s = Vec::new();
        let setf = |answers: &mut Vec<String>| {
            answers.push(format!("{:x}", slot::<Fn1>(b, 0x44)(b, to as usize)));
        };
        if variant == "steady" && to != 0 {
            setf(&mut answers);
        }
        let mut origin = 0i64;
        QueryPerformanceCounter(&mut origin);
        slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
        let ended = |_: usize, c: (u32, u32, u32)| c.2 & 1 == 0;
        match variant {
            "steady" => watch(b, origin, &mut s, 300_000, &|_, _| false),
            "retune_stop" => {
                watch(b, origin, &mut s, 1_000_000, &|n, _| n >= m1);
                setf(&mut answers);
                mark(&mut s, "f", origin);
                watch(b, origin, &mut s, 1_000_000, &|n, _| n >= m2);
                slot::<Fn0>(b, 0x48)(b);
                mark(&mut s, "stop", origin);
            }
            "retune_end" => {
                watch(b, origin, &mut s, 1_000_000, &|n, _| n >= m1);
                setf(&mut answers);
                mark(&mut s, "f", origin);
                watch(b, origin, &mut s, 2_000_000, &ended);
                mark(&mut s, "end", origin);
                Sleep(20);
            }
            "stop_retune" | "stopped" => {
                watch(b, origin, &mut s, 1_000_000, &|n, _| n >= m1);
                slot::<Fn0>(b, 0x48)(b);
                mark(&mut s, "stop", origin);
                if to != 0 {
                    setf(&mut answers);
                    mark(&mut s, "f", origin);
                }
            }
            _ => {
                // ended_stop
                watch(b, origin, &mut s, 2_000_000, &ended);
                mark(&mut s, "end", origin);
                Sleep(20);
                slot::<Fn0>(b, 0x48)(b);
                mark(&mut s, "stop", origin);
            }
        }
        if variant != "steady" {
            let t = now_us(freq, origin);
            slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
            mark(&mut s, "play", origin);
            watch(b, origin, &mut s, t + 300_000, &|_, _| false);
        }
        writeln!(
            o,
            "DsFreqPhase\t{rate:x} {channels:x} {bits:x} {bytes:x} {variant} {to} {m1} {m2} {part}\t{}\t{}",
            if answers.is_empty() { "-".to_string() } else { answers.join(",") },
            s.join(" ")
        )
        .unwrap();
        slot::<Fn0>(b, 0x48)(b);
        release(b);
        Sleep(30);
    }
}

/// A buffer that does not loop, at its end: can its write cursor read its length while it plays,
/// and what do `Stop` and `Play` do there? Also a looping buffer turned not looping after it
/// went round (`Play(0, 0, 0)` while it plays, with and without a `SetFrequency` to the same
/// frequency just before). 22050 Hz 16-bit and 11025 Hz 8-bit, lengths around 20 periods'
/// frames (plus the first `Play`'s lead), first with DirectSound's first converter (`dmo`),
/// then with the second turned on by a large change on another buffer (`apl`). Cases:
/// - `atend`: `Play`; if a read shows the status playing with the write cursor at the length,
///   `Stop` there, then `Play`;
/// - `last`: `Play`; `Stop` at the first read with the write cursor less than one period's frames
///   from the length, then `Play`;
/// - `loopoff`, `loopoff_f`: a looping buffer played 1.5 times round, then `Play(0, 0, 0)` (after
///   `SetFrequency` to its own rate for `loopoff_f`).
///
/// With `more` as a second argument: 44100 and 22050 Hz stereo 16-bit and 48000 Hz mono 16-bit
/// (the device's rate here) instead.
///
/// Every change of the cursors and status from the first `Play` is written, with markers `|stop`,
/// `|play`, `|loopoff`, `|f` and `|end` (the status read 0). Lines: `DsEndStop<TAB>rate
/// channels bits bytes variant part<TAB>samples`.
unsafe fn endstop(o: &mut impl Write, ds: *mut Obj) {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    let read = |b: *mut Obj| {
        let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
        slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
        slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
        (p, w, s)
    };
    let watch = |b: *mut Obj,
                 origin: i64,
                 s: &mut Vec<String>,
                 limit: i64,
                 until: &dyn Fn((u32, u32, u32)) -> bool| {
        let mut last = (u32::MAX, u32::MAX, u32::MAX);
        loop {
            let t = now_us(freq, origin);
            let now = read(b);
            if now != last {
                s.push(format!("{t}:{:x}:{:x}:{:x}", now.0, now.1, now.2));
                last = now;
            }
            if until(now) || t > limit {
                return now;
            }
        }
    };
    let mark = |s: &mut Vec<String>, what: &str, origin: i64| {
        s.push(format!("|{what}@{}", now_us(freq, origin)));
    };
    for part in ["dmo", "apl"] {
        if part == "apl" {
            // A large frequency change on another buffer turns the second converter on.
            let f = format(1, 22050, 16);
            let (_, x) = create(ds, 0x102e0, false, &f, f.avg);
            slot::<Fn1>(x, 0x44)(x, 30000);
            slot::<Fn3>(x, 0x30)(x, 0, 0, 0);
            Sleep(30);
            slot::<Fn0>(x, 0x48)(x);
            release(x);
        }
        // `endstop more`: stereo 16-bit and the device's own rate (48000 Hz here) instead.
        let more = std::env::args().nth(2).as_deref() == Some("more");
        let leads: &[i32] = match (part, more) {
            ("dmo", false) => &[30, 31, 32, 33, 34, 35, 36],
            (_, false) => &[-1, 0, 1, 2, 3],
            ("dmo", true) => &[0, 1, 33],
            _ => &[0, 1, 2],
        };
        let formats: &[(u16, u32, u16, i32)] = if more {
            &[
                (2, 44100, 16, 8820),
                (2, 22050, 16, 4410),
                (1, 48000, 16, 9600),
            ]
        } else {
            &[(1, 22050, 16, 4410), (1, 11025, 8, 2205)]
        };
        for &(channels, rate, bits, frames20) in formats {
            let f = format(channels, rate, bits);
            let a = f.align as u32;
            let step = rate / 100 + 1;
            let mut cases: Vec<(&str, u32)> = Vec::new();
            for &d in leads {
                cases.push(("atend", (frames20 + d) as u32));
                cases.push(("last", (frames20 + d) as u32));
            }
            cases.push(("loopoff", frames20 as u32));
            cases.push(("loopoff_f", frames20 as u32));
            for (variant, frames) in cases {
                let bytes = frames * a;
                let (_, b) = create(ds, 0x102e0, false, &f, bytes);
                let mut s = Vec::new();
                let mut origin = 0i64;
                QueryPerformanceCounter(&mut origin);
                let looping = variant.starts_with("loopoff") as usize;
                slot::<Fn3>(b, 0x30)(b, 0, 0, looping);
                match variant {
                    "atend" => {
                        let c = watch(b, origin, &mut s, 2_000_000, &|c| {
                            c.2 & 1 == 0 || c.1 == bytes
                        });
                        if c.2 & 1 != 0 {
                            slot::<Fn0>(b, 0x48)(b);
                            mark(&mut s, "stop", origin);
                        } else {
                            mark(&mut s, "end", origin);
                        }
                    }
                    "last" => {
                        let c = watch(b, origin, &mut s, 2_000_000, &|c| {
                            c.2 & 1 == 0 || c.1 + step * a >= bytes
                        });
                        if c.2 & 1 != 0 {
                            slot::<Fn0>(b, 0x48)(b);
                            mark(&mut s, "stop", origin);
                        } else {
                            mark(&mut s, "end", origin);
                        }
                    }
                    _ => {
                        watch(b, origin, &mut s, 300_000, &|_| false);
                        if variant == "loopoff_f" {
                            slot::<Fn1>(b, 0x44)(b, rate as usize);
                            mark(&mut s, "f", origin);
                        }
                        slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
                        mark(&mut s, "loopoff", origin);
                        let t = now_us(freq, origin);
                        watch(b, origin, &mut s, t + 400_000, &|_| false);
                    }
                }
                if !variant.starts_with("loopoff") {
                    let c = read(b);
                    s.push(format!(
                        "{}:{:x}:{:x}:{:x}",
                        now_us(freq, origin),
                        c.0,
                        c.1,
                        c.2
                    ));
                    let t = now_us(freq, origin);
                    slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
                    mark(&mut s, "play", origin);
                    watch(b, origin, &mut s, t + 150_000, &|_| false);
                }
                writeln!(
                    o,
                    "DsEndStop\t{rate:x} {channels:x} {bits:x} {bytes:x} {variant} {part}\t{}",
                    s.join(" ")
                )
                .unwrap();
                slot::<Fn0>(b, 0x48)(b);
                release(b);
                Sleep(30);
            }
        }
    }
}

fn main() {
    let mut o = std::io::stdout().lock();
    if std::env::args().nth(1).as_deref() == Some("endstop") {
        unsafe {
            let window = window();
            let mut ds: *mut Obj = std::ptr::null_mut();
            let hr = DirectSoundCreate8(std::ptr::null(), &mut ds, std::ptr::null_mut());
            writeln!(o, "DsCreate\t\t{hr:x}\t").unwrap();
            if ds.is_null() {
                return;
            }
            let hr = slot::<Fn2>(ds, 0x18)(ds, window as usize, 2);
            writeln!(o, "DsCooperativeLevel\t2\t{hr:x}\t").unwrap();
            endstop(&mut o, ds);
            release(ds);
            writeln!(o, "DsEnd").unwrap();
        }
        return;
    }
    let only_replays = std::env::args().nth(1).as_deref() == Some("replays");
    let only_setpos = std::env::args().nth(1).as_deref() == Some("setpos");
    if std::env::args().nth(1).as_deref() == Some("freqphase") {
        unsafe {
            let window = window();
            let mut ds: *mut Obj = std::ptr::null_mut();
            let hr = DirectSoundCreate8(std::ptr::null(), &mut ds, std::ptr::null_mut());
            writeln!(o, "DsCreate\t\t{hr:x}\t").unwrap();
            if ds.is_null() {
                return;
            }
            let hr = slot::<Fn2>(ds, 0x18)(ds, window as usize, 2);
            writeln!(o, "DsCooperativeLevel\t2\t{hr:x}\t").unwrap();
            freqphase(&mut o, ds);
            release(ds);
            writeln!(o, "DsEnd").unwrap();
        }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("endphase") {
        unsafe {
            let window = window();
            let mut ds: *mut Obj = std::ptr::null_mut();
            let hr = DirectSoundCreate8(std::ptr::null(), &mut ds, std::ptr::null_mut());
            writeln!(o, "DsCreate\t\t{hr:x}\t").unwrap();
            if ds.is_null() {
                return;
            }
            let hr = slot::<Fn2>(ds, 0x18)(ds, window as usize, 2);
            writeln!(o, "DsCooperativeLevel\t2\t{hr:x}\t").unwrap();
            endphase(&mut o, ds);
            release(ds);
            writeln!(o, "DsEnd").unwrap();
        }
        return;
    }
    unsafe {
        let window = window();
        let mut ds: *mut Obj = std::ptr::null_mut();
        let hr = DirectSoundCreate8(std::ptr::null(), &mut ds, std::ptr::null_mut());
        writeln!(o, "DsCreate\t\t{hr:x}\t").unwrap();
        if ds.is_null() {
            writeln!(o, "DsEnd").unwrap();
            return;
        }
        let hr = slot::<Fn2>(ds, 0x18)(ds, window as usize, 2);
        writeln!(o, "DsCooperativeLevel\t2\t{hr:x}\t").unwrap();
        if only_setpos {
            setpos(&mut o, ds);
            release(ds);
            writeln!(o, "DsEnd").unwrap();
            return;
        }
        if only_replays {
            replays(&mut o, ds);
            release(ds);
            writeln!(o, "DsEnd").unwrap();
            return;
        }
        let mut caps = [0u32; 0x18];
        caps[0] = 0x60;
        let hr = slot::<Fn1>(ds, 0x10)(ds, caps.as_mut_ptr() as usize);
        writeln!(o, "DsCaps\t\t{hr:x}\t{}", hex(&bytes_of(&caps))).unwrap();
        let primary = BufferDesc {
            size: 0x24,
            flags: 0x91,
            bytes: 0,
            reserved: 0,
            format: std::ptr::null(),
            alg: [0; 4],
        };
        let mut pb: *mut Obj = std::ptr::null_mut();
        let hr = slot::<Fn3>(ds, 0xc)(
            ds,
            &primary as *const _ as usize,
            &mut pb as *mut _ as usize,
            0,
        );
        let mut pfmt = [0u8; 0x12];
        let mut written = 0u32;
        if !pb.is_null() {
            slot::<Fn3>(pb, 0x14)(
                pb,
                pfmt.as_mut_ptr() as usize,
                0x12,
                &mut written as *mut _ as usize,
            );
        }
        writeln!(o, "DsPrimary\t91\t{hr:x}\t{} {written:x}", hex(&pfmt)).unwrap();
        if !pb.is_null() {
            let (q, l) = qi(pb, &IID_LISTENER);
            let mut all = [0u8; 0x40];
            all[0] = 0x40;
            let g = if l.is_null() {
                0xffff_ffff
            } else {
                slot::<Fn1>(l, 0xc)(l, all.as_mut_ptr() as usize)
            };
            writeln!(o, "DsListener\t\t{q:x} {g:x}\t{}", hex(&all)).unwrap();
            if !l.is_null() {
                let r = slot::<FnF1>(l, 0x30)(l, 1.0, 0);
                writeln!(o, "DsSet\tlistener doppler 3f800000\t{r:x}\t").unwrap();
                release(l);
            }
        }
        buffers(&mut o, ds);
        effects(&mut o, ds);
        settings(&mut o, ds);
        cursors(&mut o, ds);
        release(pb);
        release(ds);
        writeln!(o, "DsEnd").unwrap();
    }
}
