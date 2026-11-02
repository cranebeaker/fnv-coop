//! dshow-measure: what DirectShow's filter graph answers for the game's music, for the
//! platform's filter graph (`engine_core::units::platform::dshow`). 32-bit, as the game.
//! Reads the MP3 files given (the player's `Data\Music`, read only); writes nothing but stdout.
//! The graph's sound is muted (`put_Volume(-10000)`) before it runs.
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/dshow-measure/Cargo.toml -- <mp3 files..> > dshow.tsv`
//!
//! Lines: `file<TAB>path`; `render<TAB>HRESULT`; `filters<TAB>names, render order`;
//! `duration<TAB>hr<TAB>the double's bits`; with `--timing` also `run<TAB>samples` (milliseconds
//! since `Run`:`get_CurrentPosition` bits, at each change, for 1.5 s), `volume<TAB>value hr..`,
//! `seek<TAB>..` and `event<TAB>ms code p1 p2` for the events after a seek to 0.5 s before the end.
//!
//! `--states` (with one file; `--poll-state` also polls `GetState(0)` while waiting, `--only
//! <prefix>` runs only the scenarios whose name starts with it): the state
//! transitions the music thread (`00830b30`) can reach, each scenario of [`SCENARIOS`] on a
//! fresh graph of the file, muted before anything else. Lines: `scenario<TAB>name<TAB>steps`;
//! `call<TAB>name<TAB>ms<TAB>action<TAB>HRESULT<TAB>ms the call took<TAB>position after`
//! (milliseconds since the graph was ready; `GetState(0)` as `state hr value`);
//! `event<TAB>name<TAB>ms<TAB>code p1 p2<TAB>position` for each event as `GetEvent(.., 0)`
//! (polled without sleeping) first hands it out; `moved<TAB>name<TAB>ms<TAB>position` for the
//! first three position changes of each wait, `ended<TAB>name<TAB>ms<TAB>position` when it
//! becomes the duration, and `waited<TAB>name<TAB>ms<TAB>position` at the wait's end; with
//! `--poll-state`, `state<TAB>name<TAB>ms<TAB>hr value` at each change.
//!
//! `--rounds <letters>` (`--period <ms>`, default 100): the game's rounds as `--game`, one
//! letter a round: `P` `Pause`, `R` `Run`, `S` `Stop`, `E` `put_CurrentPosition(duration - 1)`;
//! the graph never ran before. The thread's wait ends early when the main loop sets its event
//! (`0082fb70`, every frame), so rounds can come much faster than 100 ms.
use std::ffi::c_void;
use std::io::Write;

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
    fn QueryPerformanceCounter(out: *mut i64) -> i32;
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
    fn Sleep(ms: u32);
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
type FnD = unsafe extern "system" fn(*mut Obj, f64) -> u32;

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

fn now_ms(freq: i64, origin: i64) -> f64 {
    let mut c = 0i64;
    unsafe { QueryPerformanceCounter(&mut c) };
    (c - origin) as f64 * 1000.0 / freq as f64
}

unsafe fn qi(o: *mut Obj, iid: &[u8; 16]) -> *mut Obj {
    let mut out: *mut Obj = std::ptr::null_mut();
    slot::<Fn2>(o, 0)(o, iid.as_ptr() as usize, &mut out as *mut _ as usize);
    out
}

unsafe fn filters(graph: *mut Obj) -> String {
    let mut en: *mut Obj = std::ptr::null_mut();
    slot::<Fn1>(graph, 0x14)(graph, &mut en as *mut _ as usize);
    let mut names = Vec::new();
    loop {
        let mut f: *mut Obj = std::ptr::null_mut();
        let mut got = 0u32;
        if slot::<Fn3>(en, 0xc)(
            en,
            1,
            &mut f as *mut _ as usize,
            &mut got as *mut _ as usize,
        ) != 0
            || got == 0
        {
            break;
        }
        let mut info = [0u16; 130];
        slot::<Fn1>(f, 0x30)(f, info.as_mut_ptr() as usize);
        let end = info[..128].iter().position(|&c| c == 0).unwrap_or(128);
        names.push(String::from_utf16_lossy(&info[..end]));
        let g = (info.as_ptr().add(128) as *const *mut Obj).read_unaligned();
        if !g.is_null() {
            slot::<Fn0>(g, 8)(g);
        }
        slot::<Fn0>(f, 8)(f);
    }
    slot::<Fn0>(en, 8)(en);
    names.join(", ")
}

/// The game's rounds (`00830b30`): every 100 ms `get_CurrentPosition`, `get_Duration`,
/// `put_Volume(-10000)`, `Run`, then `GetEvent(.., 0)` until it fails, each event freed; on
/// events 1..3 (a looping file) `Stop`, `put_CurrentPosition(0.0)`, `Run`. Round 5 pauses,
/// round 8 runs again, round 10 seeks to 1 s before the end. 40 rounds.
unsafe fn game_rounds(
    o: &mut impl Write,
    freq: i64,
    control: *mut Obj,
    position: *mut Obj,
    audio: *mut Obj,
    event: *mut Obj,
    d: f64,
) {
    let mut origin = 0i64;
    QueryPerformanceCounter(&mut origin);
    for round in 0..40 {
        Sleep(100);
        let t = now_ms(freq, origin);
        let (mut p, mut dur) = (0f64, 0f64);
        let rp = slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
        let rd = slot::<Fn1>(position, 0x1c)(position, &mut dur as *mut _ as usize);
        let rv = slot::<Fn1>(audio, 0x1c)(audio, (-10000i32) as usize);
        let (action, rc) = match round {
            5..=7 => ("pause", slot::<Fn0>(control, 0x20)(control)),
            25..=27 => ("stop", slot::<Fn0>(control, 0x24)(control)),
            10 => {
                let r = slot::<FnD>(position, 0x20)(position, d - 1.0);
                ("seek", r | slot::<Fn0>(control, 0x1c)(control) << 16)
            }
            _ => ("run", slot::<Fn0>(control, 0x1c)(control)),
        };
        let mut events = Vec::new();
        loop {
            let (mut code, mut p1, mut p2) = (0i32, 0usize, 0usize);
            let r = slot::<Fn4>(event, 0x20)(
                event,
                &mut code as *mut _ as usize,
                &mut p1 as *mut _ as usize,
                &mut p2 as *mut _ as usize,
                0,
            );
            if r as i32 != 0 {
                if events.is_empty() {
                    events.push(format!("none {r:x}"));
                }
                break;
            }
            slot::<Fn3>(event, 0x30)(event, code as usize, p1, p2);
            events.push(format!("{code:x}:{p1:x}:{p2:x}"));
            if (1..=3).contains(&code) {
                let s = slot::<Fn0>(control, 0x24)(control);
                let z = slot::<FnD>(position, 0x20)(position, 0.0);
                let r = slot::<Fn0>(control, 0x1c)(control);
                events.push(format!("restart {s:x} {z:x} {r:x}"));
            }
        }
        writeln!(
            o,
            "round\t{round}\t{t:.1}\t{rp:x} {p}\t{rd:x} {dur}\t{rv:x}\t{action} {rc:x}\t{}",
            events.join(" ")
        )
        .unwrap();
    }
}

/// The scenarios of `--states`: a name and its steps, run in order on a fresh graph. Steps: `R`
/// `Run`, `P` `Pause`, `S` `Stop`, `X` `GetState(0)`, `k<s>` `put_CurrentPosition(s)`, `e<s>`
/// `put_CurrentPosition(duration - s)`, `w<ms>` wait, polling the events.
const SCENARIOS: &[(&str, &str)] = &[
    // A graph that never ran, paused: the music thread's first round with flags & 4 (0083193b).
    ("pause-new", "X P X w1000 X P X w300 R X w1000 S X w300"),
    // The next round runs (100 ms later), or at once.
    ("pause-new-run100", "P w100 R X w1000"),
    ("pause-new-run0", "P R X w1000"),
    // Paused again 10 ms later.
    ("pause-new-pause10", "P w10 P w500"),
    // Stopped after the pause (flags & 2 next round), then run.
    ("pause-new-stop", "P w100 S X w300 R X w500"),
    // Stop on a graph that never ran (flags & 2 in the first round), then run.
    ("stop-new", "S X w300 R X w500"),
    // The synch seek (008318b2) before the first pause, and while paused.
    ("seek-new-pause", "k10 X w100 P X w500 R X w500"),
    ("pause-new-seek", "P w300 k5 X w300 R X w500"),
    // Paused from stopped after a run (EC_CLOCK_CHANGED again?).
    ("run-stop-pause", "R w500 S w200 P X w500 R X w500"),
    // Running paused, stopped, seeked, paused, run.
    (
        "run-pause-stop-pause",
        "R w300 P w200 S w200 k1 P X w300 R w300",
    ),
    // At the end: paused there, run.
    ("end-pause", "e0 X P X w500 R X w500"),
    // Paused from stopped 0.3 s before the end, then run to the end (EC_COMPLETE).
    ("near-end-pause-run", "e0.3 P w300 R w1000"),
    ("pause-new-seek-near-end", "P w300 e0.3 R w1000"),
    // The loop restart while paused: Stop, put_CurrentPosition(0), Pause.
    ("paused-stop-seek0-pause", "R w300 P w100 S k0 P X w300"),
    // The next round can come at once (the main loop sets the thread's event every frame,
    // 0082fb70): calls at once after a pause or run from stopped.
    ("pause-new-pause0", "P P X w300 R X w300"),
    ("pause-new-stop0", "P S X w300 R X w300"),
    ("pause-new-seek0", "P k5 X w300 R X w300"),
    ("pause-new-run2", "P w2 R X w300"),
    ("pause-new-run5", "P w5 R X w300"),
    ("run-new-pause0", "R P X w300 R X w300"),
    ("run-new-stop0", "R S X w300 R X w300"),
    ("run-new-run0", "R R X w300"),
    ("run-new-seek0", "R k5 X w300"),
    // The same after a run (no EC_CLOCK_CHANGED).
    ("stopped-pause-stop0", "R w300 S w100 P S X w300"),
    ("stopped-run-pause0", "R w300 S w100 R P X w300"),
    // Calls 2 or 5 ms after a pause from stopped (before its EC_PAUSED).
    ("pause-new-stop2", "P w2 S X w300"),
    ("pause-new-pause2", "P w2 P X w300"),
    ("pause-new-seek2", "P w2 k5 X w300"),
    ("pause-new-run0-stop5", "P R w5 S X w300"),
    ("pause-new-run0-pause5", "P R w5 P X w300"),
    // Calls 5 ms after a run from stopped (before its EC_PAUSED).
    ("run-new-run5", "R w5 R X w300"),
    ("run-new-pause5", "R w5 P X w300 R w300"),
    ("run-new-stop5", "R w5 S X w300"),
    // How long after the position reaches the end EC_COMPLETE comes.
    ("near-end-run", "e0.3 R w1000"),
    ("near-end-run2", "R w300 e0.3 w1000"),
    // When a run or pause from stopped settles: a Run that long after it.
    ("settle-run-0.5", "R w0.5 R w300"),
    ("settle-run-1", "R w1 R w300"),
    ("settle-run-1.5", "R w1.5 R w300"),
    ("settle-run-2", "R w2 R w300"),
    ("settle-run-3", "R w3 R w300"),
    ("settle-pause-0.5", "P w0.5 R w300"),
    ("settle-pause-1", "P w1 R w300"),
    ("settle-pause-1.5", "P w1.5 R w300"),
];

struct Graph {
    gb: *mut Obj,
    control: *mut Obj,
    position: *mut Obj,
    audio: *mut Obj,
    event: *mut Obj,
    d: f64,
    freq: i64,
    origin: i64,
}

impl Graph {
    unsafe fn open(path: &str, freq: i64) -> Option<Graph> {
        let mut gb: *mut Obj = std::ptr::null_mut();
        let hr = CoCreateInstance(
            &guid("e436ebb3-524f-11ce-9f53-0020af0ba770"),
            std::ptr::null_mut(),
            1,
            &guid("56a868a9-0ad4-11ce-b03a-0020af0ba770"),
            &mut gb,
        );
        if hr != 0 {
            return None;
        }
        let wide: Vec<u16> = path.encode_utf16().chain([0]).collect();
        if slot::<Fn2>(gb, 0x34)(gb, wide.as_ptr() as usize, 0) != 0 {
            slot::<Fn0>(gb, 8)(gb);
            return None;
        }
        let control = qi(gb, &guid("56a868b1-0ad4-11ce-b03a-0020af0ba770"));
        let position = qi(gb, &guid("56a868b2-0ad4-11ce-b03a-0020af0ba770"));
        let audio = qi(gb, &guid("56a868b3-0ad4-11ce-b03a-0020af0ba770"));
        let event = qi(gb, &guid("56a868b6-0ad4-11ce-b03a-0020af0ba770"));
        if [control, position, audio, event]
            .iter()
            .any(|x| x.is_null())
        {
            return None;
        }
        // Muted before anything plays.
        slot::<Fn1>(audio, 0x1c)(audio, (-10000i32) as usize);
        let mut d = 0f64;
        slot::<Fn1>(position, 0x1c)(position, &mut d as *mut _ as usize);
        let mut origin = 0i64;
        QueryPerformanceCounter(&mut origin);
        Some(Graph {
            gb,
            control,
            position,
            audio,
            event,
            d,
            freq,
            origin,
        })
    }
    fn t(&self) -> f64 {
        now_ms(self.freq, self.origin)
    }
    unsafe fn pos(&self) -> f64 {
        let mut p = 0f64;
        slot::<Fn1>(self.position, 0x24)(self.position, &mut p as *mut _ as usize);
        p
    }
    /// `GetState(0, &s)`: the answer and the state (0 stopped, 1 paused, 2 running).
    unsafe fn state(&self) -> (u32, i32) {
        let mut s = -1i32;
        let r = slot::<Fn2>(self.control, 0x28)(self.control, 0, &mut s as *mut _ as usize);
        (r, s)
    }
    /// Every event `GetEvent(.., 0)` hands out now, logged and freed.
    unsafe fn drain(&self, o: &mut impl Write, name: &str) {
        loop {
            let (mut code, mut p1, mut p2) = (0i32, 0usize, 0usize);
            let r = slot::<Fn4>(self.event, 0x20)(
                self.event,
                &mut code as *mut _ as usize,
                &mut p1 as *mut _ as usize,
                &mut p2 as *mut _ as usize,
                0,
            );
            if r != 0 {
                break;
            }
            let t = self.t();
            let p = self.pos();
            writeln!(o, "event\t{name}\t{t:.3}\t{code:x} {p1:x} {p2:x}\t{p}").unwrap();
            slot::<Fn3>(self.event, 0x30)(self.event, code as usize, p1, p2);
        }
    }
    unsafe fn close(self) {
        for x in [self.event, self.audio, self.position, self.control, self.gb] {
            slot::<Fn0>(x, 8)(x);
        }
    }
}

#[link(name = "kernel32")]
extern "system" {
    fn SwitchToThread() -> i32;
}

/// `--states`: every scenario of [`SCENARIOS`] (whose name starts with `only`) on a fresh graph
/// of `path`.
unsafe fn states(o: &mut impl Write, freq: i64, path: &str, poll_state: bool, only: &str) {
    for &(name, steps) in SCENARIOS.iter().filter(|s| s.0.starts_with(only)) {
        writeln!(o, "scenario\t{name}\t{steps}").unwrap();
        let Some(g) = Graph::open(path, freq) else {
            writeln!(o, "open\t{name}\tfailed").unwrap();
            continue;
        };
        for step in steps.split(' ') {
            let (what, arg) = step.split_at(1);
            if what == "w" {
                let ms: f64 = arg.parse().unwrap();
                let end = g.t() + ms;
                let (mut last, mut moved) = (g.pos().to_bits(), 0);
                let mut last_state = if poll_state { g.state() } else { (0, -1) };
                while g.t() < end {
                    g.drain(o, name);
                    let p = g.pos();
                    if p.to_bits() != last {
                        last = p.to_bits();
                        if p == g.d {
                            writeln!(o, "ended\t{name}\t{:.3}\t{p}", g.t()).unwrap();
                        } else if moved < 3 {
                            moved += 1;
                            writeln!(o, "moved\t{name}\t{:.3}\t{p}", g.t()).unwrap();
                        }
                    }
                    if poll_state {
                        let s = g.state();
                        if s != last_state {
                            last_state = s;
                            writeln!(o, "state\t{name}\t{:.3}\t{:x} {}", g.t(), s.0, s.1).unwrap();
                        }
                    }
                    SwitchToThread();
                }
                writeln!(o, "waited\t{name}\t{:.3}\t{}", g.t(), g.pos()).unwrap();
                continue;
            }
            let t0 = g.t();
            let r = match what {
                "R" => slot::<Fn0>(g.control, 0x1c)(g.control),
                "P" => slot::<Fn0>(g.control, 0x20)(g.control),
                "S" => slot::<Fn0>(g.control, 0x24)(g.control),
                "X" => {
                    let (r, s) = g.state();
                    let t1 = g.t();
                    writeln!(
                        o,
                        "call\t{name}\t{t0:.3}\tstate\t{r:x} {s}\t{:.3}\t{}",
                        t1 - t0,
                        g.pos()
                    )
                    .unwrap();
                    continue;
                }
                "k" => {
                    let s: f64 = arg.parse().unwrap();
                    slot::<FnD>(g.position, 0x20)(g.position, s)
                }
                "e" => {
                    let s: f64 = arg.parse().unwrap();
                    slot::<FnD>(g.position, 0x20)(g.position, g.d - s)
                }
                _ => unreachable!("a step of SCENARIOS"),
            };
            let t1 = g.t();
            writeln!(
                o,
                "call\t{name}\t{t0:.3}\t{step}\t{r:x}\t{:.3}\t{}",
                t1 - t0,
                g.pos()
            )
            .unwrap();
            g.drain(o, name);
        }
        g.close();
    }
}

/// `--rounds`: the game's rounds (`00830b30`) on a graph that never ran, one letter a round.
unsafe fn scripted_rounds(o: &mut impl Write, freq: i64, path: &str, letters: &str, period: u32) {
    let Some(g) = Graph::open(path, freq) else {
        writeln!(o, "open\tfailed").unwrap();
        return;
    };
    writeln!(o, "rounds\t{letters}\tduration {}\tperiod {period} ms", g.d).unwrap();
    for (round, letter) in letters.chars().enumerate() {
        Sleep(period);
        let t = g.t();
        let p = g.pos();
        let mut dur = 0f64;
        slot::<Fn1>(g.position, 0x1c)(g.position, &mut dur as *mut _ as usize);
        slot::<Fn1>(g.audio, 0x1c)(g.audio, (-10000i32) as usize);
        let rc = match letter {
            'P' => slot::<Fn0>(g.control, 0x20)(g.control),
            'R' => slot::<Fn0>(g.control, 0x1c)(g.control),
            'S' => slot::<Fn0>(g.control, 0x24)(g.control),
            'E' => slot::<FnD>(g.position, 0x20)(g.position, g.d - 1.0),
            _ => unreachable!("a round letter"),
        };
        let mut events = Vec::new();
        loop {
            let (mut code, mut p1, mut p2) = (0i32, 0usize, 0usize);
            let r = slot::<Fn4>(g.event, 0x20)(
                g.event,
                &mut code as *mut _ as usize,
                &mut p1 as *mut _ as usize,
                &mut p2 as *mut _ as usize,
                0,
            );
            if r != 0 {
                break;
            }
            slot::<Fn3>(g.event, 0x30)(g.event, code as usize, p1, p2);
            events.push(format!("{code:x}:{p1:x}:{p2:x}"));
            if (1..=3).contains(&code) {
                let s = slot::<Fn0>(g.control, 0x24)(g.control);
                let z = slot::<FnD>(g.position, 0x20)(g.position, 0.0);
                let r = slot::<Fn0>(g.control, 0x1c)(g.control);
                events.push(format!("restart {s:x} {z:x} {r:x}"));
            }
        }
        let (sr, s) = g.state();
        writeln!(
            o,
            "round\t{round}\t{t:.1}\t{p}\t{letter} {rc:x}\tstate {sr:x} {s}\t{}",
            events.join(" ")
        )
        .unwrap();
    }
    g.close();
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let timing = args.iter().any(|a| a == "--timing");
    let game = args.iter().any(|a| a == "--game");
    let grab = args
        .iter()
        .position(|a| a == "--grab")
        .map(|i| args[i + 1].clone());
    let rounds = args
        .iter()
        .position(|a| a == "--rounds")
        .map(|i| args[i + 1].clone());
    if args.iter().any(|a| a == "--states") || rounds.is_some() {
        let poll_state = args.iter().any(|a| a == "--poll-state");
        let period_at = args.iter().position(|a| a == "--period");
        let period: u32 = period_at.map_or(100, |i| args[i + 1].parse().unwrap());
        let only_at = args.iter().position(|a| a == "--only");
        let only = only_at.map_or("", |i| args[i + 1].as_str());
        let path = args
            .iter()
            .enumerate()
            .find(|&(i, a)| {
                !a.starts_with("--")
                    && Some(a) != rounds.as_ref()
                    && period_at.map(|p| p + 1) != Some(i)
                    && only_at.map(|p| p + 1) != Some(i)
            })
            .map(|(_, a)| a)
            .expect("an MP3 file");
        let mut o = std::io::stdout().lock();
        unsafe {
            CoInitializeEx(std::ptr::null_mut(), 0);
            let mut freq = 0i64;
            QueryPerformanceFrequency(&mut freq);
            match &rounds {
                Some(letters) => scripted_rounds(&mut o, freq, path, letters, period),
                None => states(&mut o, freq, path, poll_state, only),
            }
        }
        return;
    }
    let mut o = std::io::stdout().lock();
    let clsid_graph = guid("e436ebb3-524f-11ce-9f53-0020af0ba770");
    let iid_builder = guid("56a868a9-0ad4-11ce-b03a-0020af0ba770");
    let iid_control = guid("56a868b1-0ad4-11ce-b03a-0020af0ba770");
    let iid_position = guid("56a868b2-0ad4-11ce-b03a-0020af0ba770");
    let iid_audio = guid("56a868b3-0ad4-11ce-b03a-0020af0ba770");
    let iid_event = guid("56a868b6-0ad4-11ce-b03a-0020af0ba770");
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), 0);
        name_probe(&mut o);
        let mut freq = 0i64;
        QueryPerformanceFrequency(&mut freq);
        let skip: Vec<&String> = grab.iter().collect();
        for path in args
            .iter()
            .filter(|a| !a.starts_with("--") && !skip.contains(a))
        {
            writeln!(o, "file\t{path}").unwrap();
            let mut gb: *mut Obj = std::ptr::null_mut();
            let hr = CoCreateInstance(&clsid_graph, std::ptr::null_mut(), 1, &iid_builder, &mut gb);
            if hr != 0 {
                writeln!(o, "create\t{hr:x}").unwrap();
                continue;
            }
            let wide: Vec<u16> = path.encode_utf16().chain([0]).collect();
            let hr = slot::<Fn2>(gb, 0x34)(gb, wide.as_ptr() as usize, 0);
            writeln!(o, "render\t{hr:x}").unwrap();
            writeln!(o, "filters\t{}", filters(gb)).unwrap();
            let control = qi(gb, &iid_control);
            let position = qi(gb, &iid_position);
            let audio = qi(gb, &iid_audio);
            let event = qi(gb, &iid_event);
            let mut d = 0f64;
            let hr = slot::<Fn1>(position, 0x1c)(position, &mut d as *mut _ as usize);
            writeln!(o, "duration\t{hr:x}\t{:016x}\t{d}", d.to_bits()).unwrap();
            if timing && !audio.is_null() {
                for v in [-10001i32, 1, 0, -10000] {
                    let r = slot::<Fn1>(audio, 0x1c)(audio, v as usize);
                    let mut back = 0i32;
                    slot::<Fn1>(audio, 0x20)(audio, &mut back as *mut _ as usize);
                    writeln!(o, "volume\t{v}\t{r:x}\t{back}").unwrap();
                }
                let mut origin = 0i64;
                QueryPerformanceCounter(&mut origin);
                let r = slot::<Fn0>(control, 0x1c)(control);
                let mut samples = Vec::new();
                let mut last = u64::MAX;
                loop {
                    let t = now_ms(freq, origin);
                    if t > 1500.0 {
                        break;
                    }
                    let mut p = 0f64;
                    slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                    if p.to_bits() != last {
                        samples.push(format!("{t:.3}:{p}"));
                        last = p.to_bits();
                    }
                }
                writeln!(o, "run\t{r:x}\t{}", samples.join(" ")).unwrap();
                // Pause, read, run again.
                let r = slot::<Fn0>(control, 0x20)(control);
                let mut p = 0f64;
                Sleep(200);
                slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                Sleep(200);
                let mut p2 = 0f64;
                slot::<Fn1>(position, 0x24)(position, &mut p2 as *mut _ as usize);
                writeln!(o, "pause\t{r:x}\t{p} {p2}").unwrap();
                let target = d - 0.5;
                let r = slot::<FnD>(position, 0x20)(position, target);
                let mut p = 0f64;
                slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                writeln!(o, "seek\t{target}\t{r:x}\t{p}").unwrap();
                QueryPerformanceCounter(&mut origin);
                let r = slot::<Fn0>(control, 0x1c)(control);
                writeln!(o, "rerun\t{r:x}").unwrap();
                loop {
                    let t = now_ms(freq, origin);
                    if t > 3000.0 {
                        break;
                    }
                    let (mut code, mut p1, mut p2) = (0i32, 0usize, 0usize);
                    let r = slot::<Fn4>(event, 0x20)(
                        event,
                        &mut code as *mut _ as usize,
                        &mut p1 as *mut _ as usize,
                        &mut p2 as *mut _ as usize,
                        10,
                    );
                    if r == 0 {
                        let mut p = 0f64;
                        slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                        writeln!(o, "event\t{t:.3}\t{code:x} {p1:x} {p2:x}\tposition {p}").unwrap();
                        slot::<Fn3>(event, 0x30)(event, code as usize, p1, p2);
                    }
                }
                let mut p = 0f64;
                slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                writeln!(o, "end\tposition {p}").unwrap();
                // A seek past the end and to a negative time.
                for t in [d + 1.0, -1.0] {
                    let r = slot::<FnD>(position, 0x20)(position, t);
                    let mut p = 0f64;
                    slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                    writeln!(o, "seek\t{t}\t{r:x}\t{p}").unwrap();
                }
                let r = slot::<Fn0>(control, 0x24)(control);
                writeln!(o, "stop\t{r:x}").unwrap();
            }
            if let Some(dir) = &grab {
                grab_pcm(&mut o, gb, dir, path);
            }
            if game && !audio.is_null() {
                game_rounds(&mut o, freq, control, position, audio, event, d);
            }
            for x in [event, audio, position, control, gb] {
                if !x.is_null() {
                    slot::<Fn0>(x, 8)(x);
                }
            }
        }
    }
}

/// The name the graph gives a filter added with a NULL name: the File Source (Async) filter
/// added with `AddFilter(f, NULL)`, then `QueryFilterInfo`.
unsafe fn name_probe(o: &mut impl Write) {
    let clsid_graph = guid("e436ebb3-524f-11ce-9f53-0020af0ba770");
    let iid_graph = guid("56a8689f-0ad4-11ce-b03a-0020af0ba770");
    let clsid_async = guid("e436ebb5-524f-11ce-9f53-0020af0ba770");
    let iid_filter = guid("56a86895-0ad4-11ce-b03a-0020af0ba770");
    let mut fg: *mut Obj = std::ptr::null_mut();
    let mut f: *mut Obj = std::ptr::null_mut();
    let a = CoCreateInstance(&clsid_graph, std::ptr::null_mut(), 1, &iid_graph, &mut fg);
    let b = CoCreateInstance(&clsid_async, std::ptr::null_mut(), 1, &iid_filter, &mut f);
    for _ in 0..2 {
        let r = slot::<Fn2>(fg, 0xc)(fg, f as usize, 0);
        let mut info = [0u16; 130];
        slot::<Fn1>(f, 0x30)(f, info.as_mut_ptr() as usize);
        let end = info[..128].iter().position(|&c| c == 0).unwrap_or(128);
        writeln!(
            o,
            "addfilter\t{a:x} {b:x}\t{r:x}\t{}",
            String::from_utf16_lossy(&info[..end])
        )
        .unwrap();
        let g = (info.as_ptr().add(128) as *const *mut Obj).read_unaligned();
        if !g.is_null() {
            slot::<Fn0>(g, 8)(g);
        }
    }
    slot::<Fn0>(f, 8)(f);
    slot::<Fn0>(fg, 8)(fg);
}

/// The bytes the decoder delivered, gathered by the sample grabber's callback.
static mut GRABBED: Vec<u8> = Vec::new();

unsafe extern "system" fn cb_query(this: *mut Obj, _iid: *const u8, out: *mut *mut Obj) -> u32 {
    *out = this;
    0
}
unsafe extern "system" fn cb_ref(_this: *mut Obj) -> u32 {
    1
}
unsafe extern "system" fn cb_sample(_this: *mut Obj, _time: f64, _sample: *mut Obj) -> u32 {
    0
}
#[allow(static_mut_refs)]
unsafe extern "system" fn cb_buffer(
    _this: *mut Obj,
    _time: f64,
    buffer: *const u8,
    len: i32,
) -> u32 {
    GRABBED.extend_from_slice(std::slice::from_raw_parts(buffer, len as usize));
    0
}

#[repr(C)]
struct CallbackVtable {
    query: unsafe extern "system" fn(*mut Obj, *const u8, *mut *mut Obj) -> u32,
    add_ref: unsafe extern "system" fn(*mut Obj) -> u32,
    release: unsafe extern "system" fn(*mut Obj) -> u32,
    sample: unsafe extern "system" fn(*mut Obj, f64, *mut Obj) -> u32,
    buffer: unsafe extern "system" fn(*mut Obj, f64, *const u8, i32) -> u32,
}
static CALLBACK: CallbackVtable = CallbackVtable {
    query: cb_query,
    add_ref: cb_ref,
    release: cb_ref,
    sample: cb_sample,
    buffer: cb_buffer,
};

/// The first pin of `f` in direction `dir` (0 input, 1 output).
unsafe fn pin_of(f: *mut Obj, dir: u32) -> *mut Obj {
    let mut en: *mut Obj = std::ptr::null_mut();
    slot::<Fn1>(f, 0x28)(f, &mut en as *mut _ as usize);
    loop {
        let mut p: *mut Obj = std::ptr::null_mut();
        let mut got = 0u32;
        if slot::<Fn3>(en, 0xc)(
            en,
            1,
            &mut p as *mut _ as usize,
            &mut got as *mut _ as usize,
        ) != 0
            || got == 0
        {
            slot::<Fn0>(en, 8)(en);
            return std::ptr::null_mut();
        }
        let mut d = 9u32;
        slot::<Fn1>(p, 0x24)(p, &mut d as *mut _ as usize);
        if d == dir {
            slot::<Fn0>(en, 8)(en);
            return p;
        }
        slot::<Fn0>(p, 8)(p);
    }
}

/// The filter of the graph whose name starts with `name`.
unsafe fn find_filter(graph: *mut Obj, name: &str) -> *mut Obj {
    let wide: Vec<u16> = name.encode_utf16().chain([0]).collect();
    let mut f: *mut Obj = std::ptr::null_mut();
    slot::<Fn2>(graph, 0x18)(graph, wide.as_ptr() as usize, &mut f as *mut _ as usize);
    f
}

/// The rendered graph's decoder output through a sample grabber into a null renderer, run
/// without a clock; the bytes written to `dir`/<file name>.pcm and their count reported.
#[allow(static_mut_refs)]
unsafe fn grab_pcm(o: &mut impl Write, gb: *mut Obj, dir: &str, path: &str) {
    GRABBED.clear();
    let renderer = find_filter(gb, "Default DirectSound Device");
    let decoder = find_filter(gb, "MP3 Decoder DMO");
    let r1 = slot::<Fn1>(gb, 0x10)(gb, renderer as usize);
    let out = pin_of(decoder, 1);
    let iid_filter = guid("56a86895-0ad4-11ce-b03a-0020af0ba770");
    let mut grabber: *mut Obj = std::ptr::null_mut();
    let mut null: *mut Obj = std::ptr::null_mut();
    let r2 = CoCreateInstance(
        &guid("c1f400a0-3f08-11d3-9f0b-006008039e37"),
        std::ptr::null_mut(),
        1,
        &iid_filter,
        &mut grabber,
    );
    let r3 = CoCreateInstance(
        &guid("c1f400a4-3f08-11d3-9f0b-006008039e37"),
        std::ptr::null_mut(),
        1,
        &iid_filter,
        &mut null,
    );
    let name: Vec<u16> = "g".encode_utf16().chain([0]).collect();
    slot::<Fn2>(gb, 0xc)(gb, grabber as usize, name.as_ptr() as usize);
    let name: Vec<u16> = "n".encode_utf16().chain([0]).collect();
    slot::<Fn2>(gb, 0xc)(gb, null as usize, name.as_ptr() as usize);
    let sg = qi(grabber, &guid("6b652fff-11fe-4fce-92ad-0266b5d7c78f"));
    slot::<Fn1>(sg, 0xc)(sg, 0);
    slot::<Fn1>(sg, 0x18)(sg, 0);
    let cb = &CALLBACK as *const CallbackVtable;
    let cb_obj = Box::into_raw(Box::new(Obj {
        vtable: cb as *const usize,
    }));
    let r4 = slot::<Fn2>(sg, 0x24)(sg, cb_obj as usize, 1);
    let g_in = pin_of(grabber, 0);
    let g_out = pin_of(grabber, 1);
    let n_in = pin_of(null, 0);
    let r5 = slot::<Fn2>(gb, 0x2c)(gb, out as usize, g_in as usize);
    let r6 = slot::<Fn2>(gb, 0x2c)(gb, g_out as usize, n_in as usize);
    let mf = qi(gb, &guid("56a86899-0ad4-11ce-b03a-0020af0ba770"));
    let r7 = slot::<Fn1>(mf, 0x20)(mf, 0);
    let control = qi(gb, &guid("56a868b1-0ad4-11ce-b03a-0020af0ba770"));
    let event = qi(gb, &guid("56a868b6-0ad4-11ce-b03a-0020af0ba770"));
    slot::<Fn0>(control, 0x1c)(control);
    loop {
        let (mut code, mut p1, mut p2) = (0i32, 0usize, 0usize);
        let r = slot::<Fn4>(event, 0x20)(
            event,
            &mut code as *mut _ as usize,
            &mut p1 as *mut _ as usize,
            &mut p2 as *mut _ as usize,
            30_000,
        );
        if r != 0 {
            break;
        }
        slot::<Fn3>(event, 0x30)(event, code as usize, p1, p2);
        if code == 1 {
            break;
        }
    }
    slot::<Fn0>(control, 0x24)(control);
    // The format the grabber was connected with.
    let mut mt = [0u8; 0x48];
    slot::<Fn1>(sg, 0x14)(sg, mt.as_mut_ptr() as usize);
    let fmt_ptr = u32::from_le_bytes([mt[0x44], mt[0x45], mt[0x46], mt[0x47]]) as *const u8;
    let fmt = if fmt_ptr.is_null() {
        Vec::new()
    } else {
        std::slice::from_raw_parts(fmt_ptr, 18).to_vec()
    };
    let file = std::path::Path::new(path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    std::fs::write(
        std::path::Path::new(dir).join(format!("{file}.pcm")),
        &GRABBED,
    )
    .unwrap();
    writeln!(
        o,
        "grab\t{r1:x} {r2:x} {r3:x} {r4:x} {r5:x} {r6:x} {r7:x}\t{}\tformat {}",
        GRABBED.len(),
        fmt.iter().map(|b| format!("{b:02x}")).collect::<String>()
    )
    .unwrap();
    for x in [
        event, control, mf, sg, g_in, g_out, n_in, out, null, grabber, decoder, renderer,
    ] {
        if !x.is_null() {
            slot::<Fn0>(x, 8)(x);
        }
    }
}
