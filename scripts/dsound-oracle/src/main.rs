//! dsound-oracle: compares the platform's DirectSound 3D stage and distortion
//! (`engine-core` `platform/ds3d.rs` and `platform/dsfx.rs`, compiled in here as they are) with
//! the player's own `dsound.dll` and `dsdmo.dll`, run in this 32-bit process like the game's.
//! Nothing is heard and no device or setting is changed: the 3D buffers play silence, and the
//! effect runs on our test signals in memory.
//!
//! - 3D: random listeners (position, orientation) and sources (position, minimum and maximum
//!   distance, rolloff, volume, velocity), set through `IDirectSound3DListener` and
//!   `IDirectSound3DBuffer`; the channel multipliers `dsound.dll` hands its mixer (the software
//!   3D object's sink, `10057be0`: an `f32` per channel) and its Doppler frequency are read from
//!   the DLL's objects in this process and compared bit for bit with `ds3d::hear`. The objects
//!   are reached through the offsets of the analysed DLL (`SysWOW64` 10.0.26200; checked against
//!   the DLL's own vtables before use: another build stops the run).
//! - Deferred settings: a position set with `DS3D_DEFERRED` changes nothing until
//!   `CommitDeferredSettings`.
//! - Distortion: the effect object (`CoCreateInstance` of `GUID_DSFX_STANDARD_DISTORTION`) runs
//!   `IMediaObjectInPlace::Process` on noise in PCM 8 and 16 bit, mono and stereo, at several
//!   rates and parameters; its output is compared sample for sample with `dsfx::Distortion`.
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/dsound-oracle/Cargo.toml [cases]`
//!
//! Prints the counts of exact cases and the first differences; exits 1 on any difference.
#![allow(dead_code)]

#[path = "../../../crates/engine-core/src/units/platform/ds3d.rs"]
mod ds3d;
#[path = "../../../crates/engine-core/src/units/platform/dsfx.rs"]
mod dsfx;
#[path = "../../../crates/engine-core/src/units/platform/mixer.rs"]
mod mixer;

use std::ffi::c_void;

#[link(name = "dsound")]
extern "system" {
    fn DirectSoundCreate8(device: *const u8, out: *mut *mut Obj, outer: *mut c_void) -> u32;
}
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
}
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleA(name: *const u8) -> usize;
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
type Fn7 =
    unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize, usize, usize, usize) -> u32;
type FnF1 = unsafe extern "system" fn(*mut Obj, f32, u32) -> u32;
type FnF3 = unsafe extern "system" fn(*mut Obj, f32, f32, f32, u32) -> u32;
type FnProcess = unsafe extern "system" fn(*mut Obj, u32, *mut u8, i64, u32) -> u32;
type FnSetType = unsafe extern "system" fn(*mut Obj, u32, *const MediaType, u32) -> u32;

fn guid(s: &str) -> [u8; 16] {
    let h: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    let b = |i: usize| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap();
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

unsafe fn qi(o: *mut Obj, iid: &[u8; 16]) -> *mut Obj {
    let mut out: *mut Obj = std::ptr::null_mut();
    slot::<Fn2>(o, 0)(o, iid.as_ptr() as usize, &mut out as *mut _ as usize);
    out
}

/// A fixed 32-bit generator.
struct Rng(u32);

impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
    fn unit(&mut self) -> f32 {
        (self.next() >> 8) as f32 / 16_777_216.0
    }
    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.unit()
    }
    fn pick<T: Copy>(&mut self, v: &[T]) -> T {
        v[self.next() as usize % v.len()]
    }
    fn vec(&mut self, r: f32) -> [f32; 3] {
        [self.range(-r, r), self.range(-r, r), self.range(-r, r)]
    }
}

fn bits(v: f32) -> u32 {
    v.to_bits()
}

/// The software 3D object behind an `IDirectSound3DBuffer` and what it hands the mixer.
struct Inside {
    object: usize,
}

impl Inside {
    /// Follows the analysed DLL's pointers: the interface's `CImpDirectSound3dBuffer`
    /// (signature `3DBF` at +0x18) -> +0x1c -> +0x18 the `CWrapper3dObject` (vtable RVA 0x10a4)
    /// -> +0xbc the `CMultiPan3dObject` (vtable RVA 0x12e0). `None` while the wrapper has no
    /// object yet.
    unsafe fn find(three_d: *mut Obj, base: usize) -> Result<Option<Inside>, String> {
        let p = three_d as usize;
        let read = |a: usize| *(a as *const usize);
        if *((p + 0x18) as *const u32) != 0x4642_4433 {
            return Err("the 3D interface has no 3DBF signature at +0x18".into());
        }
        let wrapper = read(read(p + 0x1c) + 0x18);
        if read(wrapper) != base + 0x10a4 {
            return Err(format!(
                "unexpected wrapper vtable {:#x}",
                read(wrapper) - base
            ));
        }
        let object = read(wrapper + 0xbc);
        if object == 0 {
            return Ok(None);
        }
        if read(object) != base + 0x12e0 {
            return Err(format!(
                "unexpected 3D object vtable {:#x}",
                read(object) - base
            ));
        }
        Ok(Some(Inside { object }))
    }

    unsafe fn u32_at(&self, o: usize) -> u32 {
        *((self.object + o) as *const u32)
    }

    unsafe fn f32_at(&self, o: usize) -> f32 {
        *((self.object + o) as *const f32)
    }

    /// The speakers the listener pans over (`+0xec` listener, `+0xac`).
    unsafe fn speakers(&self) -> u32 {
        *((self.u32_at(0xec) as usize + 0xac) as *const u32)
    }

    /// The channel multipliers in the sink (`+0xf0`, its `+0xb0`).
    unsafe fn gains(&self) -> (f32, f32) {
        let sink = self.u32_at(0xf0) as usize;
        let g = *((sink + 0xb0) as *const usize) as *const f32;
        (*g, *g.add(1))
    }

    unsafe fn local(&self) -> [f32; 3] {
        [self.f32_at(0xfc), self.f32_at(0x100), self.f32_at(0x104)]
    }

    unsafe fn pans(&self) -> [i32; 2] {
        [self.u32_at(0x108) as i32, self.u32_at(0x10c) as i32]
    }

    /// The frequency after Doppler (`+0xa4`).
    unsafe fn frequency(&self) -> u32 {
        self.u32_at(0xa4)
    }
}

struct Report {
    name: &'static str,
    exact: usize,
    total: usize,
    shown: usize,
}

impl Report {
    fn new(name: &'static str) -> Report {
        Report {
            name,
            exact: 0,
            total: 0,
            shown: 0,
        }
    }
    fn check(&mut self, same: bool, what: impl FnOnce() -> String) {
        self.total += 1;
        if same {
            self.exact += 1;
        } else if self.shown < 12 {
            self.shown += 1;
            println!("  {} differs: {}", self.name, what());
        }
    }
    fn print(&self) -> bool {
        println!("{}: {} of {} exact", self.name, self.exact, self.total);
        self.exact == self.total
    }
}

/// The 3D comparison: `cases` random settings.
unsafe fn three_d(cases: usize) -> Vec<Report> {
    let base = GetModuleHandleA(b"dsound.dll\0".as_ptr());
    let mut ds: *mut Obj = std::ptr::null_mut();
    assert_eq!(
        DirectSoundCreate8(std::ptr::null(), &mut ds, std::ptr::null_mut()),
        0,
        "DirectSoundCreate8"
    );
    slot::<Fn2>(ds, 0x18)(ds, GetDesktopWindow() as usize, 2);
    let desc: [u32; 9] = [0x24, 0x91, 0, 0, 0, 0, 0, 0, 0];
    let mut primary: *mut Obj = std::ptr::null_mut();
    slot::<Fn3>(ds, 0xc)(
        ds,
        desc.as_ptr() as usize,
        &mut primary as *mut _ as usize,
        0,
    );
    let listener = qi(primary, &guid("279afa84-4981-11ce-a521-0020af0be560"));
    assert!(!listener.is_null(), "no listener");
    // A 3D buffer as the game makes them: mono 16-bit 48000 Hz, CTRL3D | CTRLFREQUENCY |
    // CTRLVOLUME | GETCURRENTPOSITION2 | MUTE3DATMAXDISTANCE | GLOBALFOCUS, half a second of
    // silence, playing looped.
    let wf: [u8; 18] = [
        1, 0, 1, 0, 0x80, 0xbb, 0, 0, 0, 0x77, 1, 0, 2, 0, 16, 0, 0, 0,
    ];
    let d: [u32; 9] = [
        0x24,
        0x300b0 | 0x8000,
        48000,
        0,
        wf.as_ptr() as u32,
        0,
        0,
        0,
        0,
    ];
    let mut b: *mut Obj = std::ptr::null_mut();
    assert_eq!(
        slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b as *mut _ as usize, 0),
        0,
        "CreateSoundBuffer"
    );
    let b3 = qi(b, &guid("279afa86-4981-11ce-a521-0020af0be560"));
    let mut inside = Inside::find(b3, base).unwrap_or_else(|e| panic!("{e}"));
    if inside.is_none() {
        slot::<Fn3>(b, 0x30)(b, 0, 0, 1);
        inside = Inside::find(b3, base).unwrap_or_else(|e| panic!("{e}"));
    }
    let inside = inside.expect("the 3D object (after Play)");
    let speakers = inside.speakers();
    assert_eq!(
        speakers, 2,
        "the default device's listener pans over {speakers} speakers, not 2"
    );

    let mut gains = Report::new("3D channel multipliers");
    let mut locals = Report::new("3D listener-frame positions");
    let mut doppler = Report::new("3D Doppler frequency");
    let mut deferred = Report::new("3D deferred position");
    // How many cases exercise each part (so a pass is not an empty one).
    let (mut heard, mut split, mut near, mut shifted, mut turned) = (0, 0, 0, 0, 0);
    let mut rng = Rng(0x2468_ace1);
    for case in 0..cases {
        // Listener.
        let game = case % 3 == 0;
        let (front, top) = if case % 7 == 0 {
            ([0.0f32, 0.0, 1.0], [0.0f32, 1.0, 0.0])
        } else if game {
            // A camera's: yaw and pitch, top square to the front.
            let (yaw, pitch) = (rng.range(-3.2, 3.2), rng.range(-1.4, 1.4));
            let f = [
                yaw.sin() * pitch.cos(),
                pitch.sin(),
                yaw.cos() * pitch.cos(),
            ];
            let t = [
                -yaw.sin() * pitch.sin(),
                pitch.cos(),
                -yaw.cos() * pitch.sin(),
            ];
            (f, t)
        } else {
            (rng.vec(2.0), rng.vec(2.0))
        };
        let Some((front_set, top_set)) = ds3d::orientation(front, top) else {
            continue;
        };
        let lpos = rng.vec(if game { 300.0 } else { 20.0 });
        let lvel = if case % 5 == 1 {
            rng.vec(30.0)
        } else {
            [0.0; 3]
        };
        let rolloff = if game {
            1.0
        } else {
            rng.pick(&[0.0, 0.5, 1.0, 2.5])
        };
        let dop = if case % 5 == 1 {
            rng.pick(&[0.5, 1.0, 3.0])
        } else {
            1.0
        };
        let w = |v: f32| v.to_bits() as usize;
        let r = slot::<Fn7>(listener, 0x34)(
            listener,
            w(front[0]),
            w(front[1]),
            w(front[2]),
            w(top[0]),
            w(top[1]),
            w(top[2]),
            0,
        );
        assert_eq!(r, 0, "SetOrientation");
        slot::<FnF3>(listener, 0x38)(listener, lpos[0], lpos[1], lpos[2], 0);
        slot::<FnF3>(listener, 0x40)(listener, lvel[0], lvel[1], lvel[2], 0);
        slot::<FnF1>(listener, 0x3c)(listener, rolloff, 0);
        slot::<FnF1>(listener, 0x30)(listener, dop, 0);
        // Source.
        let scale = rng.pick(&[0.3f32, 1.0, 2.0, 3.0, 8.0, 60.0]);
        let offset = rng.vec(scale);
        let pos = [
            lpos[0] + offset[0],
            lpos[1] + offset[1],
            lpos[2] + offset[2],
        ];
        let (min, max) = if game {
            (1.0e9f32, 1.0e9f32)
        } else {
            (
                rng.pick(&[0.5f32, 1.0, 3.0, 1.0e9]),
                rng.pick(&[1.0e9f32, 20.0, 4.0]),
            )
        };
        let vel = if case % 5 == 1 {
            rng.vec(40.0)
        } else {
            [0.0; 3]
        };
        let volume = -((rng.next() % 3001) as i32);
        slot::<Fn1>(b, 0x3c)(b, volume as usize);
        slot::<FnF1>(b3, 0x44)(b3, min, 0);
        slot::<FnF1>(b3, 0x40)(b3, max, 0);
        slot::<FnF3>(b3, 0x50)(b3, vel[0], vel[1], vel[2], 0);
        slot::<FnF3>(b3, 0x4c)(b3, pos[0], pos[1], pos[2], 0);

        let l = ds3d::Listener {
            position: ds3d::clamp_length(lpos),
            velocity: lvel,
            front: front_set,
            top: top_set,
            distance_factor: 1.0,
            rolloff,
            doppler: dop,
        };
        let s = ds3d::Source {
            position: pos,
            velocity: vel,
            min,
            max,
            mute_at_max: true,
            volume,
            frequency: 48000,
        };
        let ours = ds3d::hear(&s, &l);
        let theirs = inside.gains();
        let our_local = ds3d::to_listener(pos, lpos, ds3d::Angles::of(front_set, top_set));
        let their_local = inside.local();
        locals.check(our_local.map(bits) == their_local.map(bits), || {
            format!("case {case}: ours {our_local:?} the DLL's {their_local:?}")
        });
        gains.check(
            (bits(ours.gains.0), bits(ours.gains.1)) == (bits(theirs.0), bits(theirs.1)),
            || {
                format!(
                    "case {case}: ours {:?} the DLL's {theirs:?}; pan ours {:?} the DLL's {:?}; local {their_local:?}",
                    ours.gains,
                    ds3d::pan_millibels(our_local),
                    inside.pans()
                )
            },
        );
        heard += usize::from(theirs.0 > 0.0 || theirs.1 > 0.0);
        split += usize::from(theirs.0 != theirs.1);
        near += usize::from(our_local[0].abs() < 2.5 && our_local[2].abs() < 2.5);
        turned += usize::from(
            our_local.map(bits) != [pos[0] - lpos[0], pos[1] - lpos[1], pos[2] - lpos[2]].map(bits),
        );
        let their_hz = inside.frequency().clamp(100, 200_000);
        shifted += usize::from(their_hz != 48000);
        doppler.check(ours.frequency == their_hz, || {
            format!("case {case}: ours {} the DLL's {their_hz}", ours.frequency)
        });

        // A deferred move waits for the commit.
        if case % 11 == 0 {
            let before = inside.gains();
            let moved = [pos[0] + 2.0, pos[1], pos[2] - 1.5];
            slot::<FnF3>(b3, 0x4c)(b3, moved[0], moved[1], moved[2], 1);
            let waiting = inside.gains();
            slot::<Fn0>(listener, 0x44)(listener);
            let after = inside.gains();
            let ours = ds3d::hear(
                &ds3d::Source {
                    position: moved,
                    ..s
                },
                &l,
            );
            deferred.check(
                (bits(before.0), bits(before.1)) == (bits(waiting.0), bits(waiting.1))
                    && (bits(after.0), bits(after.1)) == (bits(ours.gains.0), bits(ours.gains.1)),
                || format!("case {case}: before {before:?} waiting {waiting:?} after {after:?} ours {:?}", ours.gains),
            );
        }
    }
    println!(
        "3D cases: {heard} heard, {split} with the channels apart, {near} in the near zone, {turned} turned by the listener, {shifted} Doppler-shifted"
    );
    slot::<Fn0>(b, 0x48)(b);
    for o in [b3, b, listener, primary, ds] {
        slot::<Fn0>(o, 8)(o);
    }
    vec![locals, gains, doppler, deferred]
}

#[repr(C)]
struct MediaType {
    major: [u8; 16],
    sub: [u8; 16],
    fixed: i32,
    temporal: i32,
    sample_size: u32,
    format_type: [u8; 16],
    unk: *mut c_void,
    format_size: u32,
    format: *const u8,
}

/// The player's distortion effect on `data` (PCM, in place).
unsafe fn dmo_process(data: &mut [u8], channels: u16, bits: u16, rate: u32, p: [f32; 5]) -> bool {
    let mut dmo: *mut Obj = std::ptr::null_mut();
    let r = CoCreateInstance(
        &guid("ef114c90-cd1d-484e-96e5-09cfaf912a21"),
        std::ptr::null_mut(),
        1,
        &guid("d8ad0f58-5494-4102-97c5-ec798e59bcf4"),
        &mut dmo,
    );
    assert_eq!(r, 0, "CoCreateInstance(distortion)");
    let align = channels * bits / 8;
    let mut wf = [0u8; 18];
    wf[0..2].copy_from_slice(&1u16.to_le_bytes());
    wf[2..4].copy_from_slice(&channels.to_le_bytes());
    wf[4..8].copy_from_slice(&rate.to_le_bytes());
    wf[8..12].copy_from_slice(&(rate * u32::from(align)).to_le_bytes());
    wf[12..14].copy_from_slice(&align.to_le_bytes());
    wf[14..16].copy_from_slice(&bits.to_le_bytes());
    let mt = MediaType {
        major: guid("73647561-0000-0010-8000-00aa00389b71"),
        sub: guid("00000001-0000-0010-8000-00aa00389b71"),
        fixed: 1,
        temporal: 0,
        sample_size: u32::from(align),
        format_type: guid("05589f81-c356-11ce-bf01-00aa0055595a"),
        unk: std::ptr::null_mut(),
        format_size: 18,
        format: wf.as_ptr(),
    };
    let steps = [
        slot::<FnSetType>(dmo, 0x20)(dmo, 0, &mt, 0),
        slot::<FnSetType>(dmo, 0x24)(dmo, 0, &mt, 0),
        slot::<Fn0>(dmo, 0x48)(dmo),
    ];
    let ok = steps == [0; 3];
    if !ok {
        println!("  the effect's types: {steps:x?}");
    }
    let fx = qi(dmo, &guid("8ecf4326-455f-4d8b-bda9-8d5d3e9e3e0b"));
    let inplace = qi(dmo, &guid("651b9ad0-0fc7-4aa9-9538-d89931010741"));
    let set = slot::<Fn1>(fx, 0xc)(fx, p.as_ptr() as usize);
    let run = slot::<FnProcess>(inplace, 0xc)(inplace, data.len() as u32, data.as_mut_ptr(), 0, 0);
    // SetAllParameters answers S_FALSE (1) when it held a frequency to a third of the rate.
    if set > 1 || run != 0 {
        println!("  SetAllParameters {set:x}, Process {run:x}");
    }
    let ok = ok && set <= 1 && run == 0;
    for o in [fx, inplace, dmo] {
        slot::<Fn0>(o, 8)(o);
    }
    ok
}

/// The distortion comparison.
unsafe fn distortion() -> Vec<Report> {
    let table = dsfx::edge_table().expect("the edge table from the player's dsdmo.dll");
    println!("edge table (measured from the player's dsdmo.dll): {table:?}");
    let mut report = Report::new("distortion samples");
    let mut cases = Report::new("distortion cases");
    let mut rng = Rng(0x1357_9bdf);
    let game = [-18.2f32, 0.0, 5000.0, 1135.0, 5000.0];
    let mut params = vec![
        game,
        [0.0, 100.0, 100.0, 100.0, 100.0],
        [-60.0, 0.0, 8000.0, 8000.0, 8000.0],
    ];
    for k in 0..30 {
        params.push([
            -25.0,
            ((k as f32 + 0.5) / 29.0 * 100.0).min(100.0),
            3000.0,
            2000.0,
            7000.0,
        ]);
    }
    for _ in 0..20 {
        params.push([
            rng.range(-60.0, 0.0),
            rng.range(0.0, 100.0),
            rng.range(100.0, 8000.0),
            rng.range(100.0, 8000.0),
            rng.range(100.0, 8000.0),
        ]);
    }
    for (pi, p) in params.iter().enumerate() {
        for &(channels, bits) in &[(1u16, 16u16), (2, 16), (1, 8), (2, 8)] {
            for &rate in &[11025u32, 22050, 32000, 44100, 48000] {
                let frames = 6000usize;
                let loud = [300i32, 3000, 30000][pi % 3];
                let mut data = Vec::new();
                for _ in 0..frames * usize::from(channels) {
                    let v = (rng.next() % (2 * loud as u32 + 1)) as i32 - loud;
                    if bits == 16 {
                        data.extend_from_slice(&(v as i16).to_le_bytes());
                    } else {
                        data.push((128 + v / 256) as u8);
                    }
                }
                let mut theirs = data.clone();
                assert!(
                    dmo_process(&mut theirs, channels, bits, rate, *p),
                    "the effect refused {p:?}"
                );
                let mut ours = data.clone();
                let mut d = dsfx::Distortion::new(dsfx::Coefficients::new(rate, *p, table));
                let align = usize::from(channels * bits / 8);
                for f in ours.chunks_mut(align) {
                    d.frame(f, channels, bits);
                }
                let step = usize::from(bits / 8);
                let mut same = true;
                for (i, (a, b)) in ours.chunks(step).zip(theirs.chunks(step)).enumerate() {
                    report.check(a == b, || {
                        format!("{p:?} {channels} ch {bits} bit {rate} Hz, sample {i}: ours {a:?} the DLL's {b:?}")
                    });
                    same &= a == b;
                }
                cases.check(same, || format!("{p:?} {channels} ch {bits} bit {rate} Hz"));
            }
        }
    }
    vec![report, cases]
}

fn main() {
    let cases: usize = std::env::args()
        .nth(1)
        .map_or(20_000, |s| s.parse().expect("a case count"));
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), 0);
        let mut all = three_d(cases);
        all.extend(distortion());
        let mut ok = true;
        for r in &all {
            ok &= r.print();
        }
        std::process::exit(if ok { 0 } else { 1 });
    }
}
