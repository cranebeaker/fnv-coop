//! Runs every listed function on the real CPU (`nv-call`) and in the engine
//! with the same inputs and compares the results bit for bit.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use engine::{Engine, Ret};

use crate::list::Pure;
use crate::source::Ty;
use crate::vectors::{self, Case, Outcome};

pub struct Options {
    pub exe: PathBuf,
    pub nv_call: PathBuf,
    pub out_dir: PathBuf,
    pub per_function: usize,
    /// A vector nv-call has not finished after this long is abandoned
    /// (`ExeTimeout`): nv-call is stopped and restarted after it.
    pub exe_timeout: std::time::Duration,
    /// A vector that took the exe longer than this is not run on the engine
    /// side (`ExeSlow`): a correct translation may take far longer.
    pub slow: std::time::Duration,
    /// The engine side running one vector longer than this stops the whole
    /// check with an error naming the vector.
    pub rust_timeout: std::time::Duration,
    /// The x87 control word both sides run with (`--fpcw`): nv-call loads it
    /// before each call, and each engine gets it as its main thread's word
    /// (`crt::set_x87_control_word`), so the x87 helpers round to its
    /// precision. `None`: nv-call's default 0x027F and the engine's.
    pub fpcw: Option<u16>,
}

/// How one vector came out.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Match,
    /// Both sides failed the same way: the exe's access violation and the
    /// engine's unmapped read are at the same address (or both divided by
    /// zero). Consistent, but says nothing about the result.
    BothFailed,
    /// The engine read memory it never maps: outside the exe's data sections
    /// (an out-of-range table index from a random input, landing in `.text`,
    /// nv-call's buffer arena, its heap or a DLL). The exe read the same
    /// address, which nv-call's process happened to have mapped:
    /// inconclusive, not a translation difference.
    OutOfImageRead,
    Mismatch,
    /// Both results are NaN but the bits differ (sign or payload).
    NanBits,
    ExeFaultOnly,
    RustPanicOnly,
    /// Both failed, but differently (another address, or another kind of
    /// failure) where the engine had the memory to compare.
    FaultMismatch,
    /// The exe removed a different number of stack bytes than the listed
    /// convention says: the list, not the translation, is wrong.
    Convention,
    /// Same result, but a buffer a pointer argument points into differs after
    /// the call (one side stored where the other didn't).
    BufferMismatch,
    /// The exe ran past the time limit on this vector (a loop over a count
    /// taken from the inputs); not run on the engine side. Inconclusive.
    ExeTimeout,
    /// The exe took over the slow threshold; not run on the engine side.
    /// Inconclusive.
    ExeSlow,
    /// nv-call's process ended on this vector (an exception outside the
    /// harness's reach). Inconclusive.
    ExeCrash,
    /// The exe called a Windows import (nv-call runs none: its fault kind `import`), as the C
    /// runtime's error path does on some inputs (errno through `__getptd`, `GetLastError`).
    /// The engine provides those imports, so the two sides cannot be compared past that
    /// call. Inconclusive.
    ExeImport,
    /// The exe hit an access violation, ran past `--exe-timeout` or ended
    /// nv-call's process, in a function listed as reading only its buffers at
    /// known offsets, which by construction neither faults nor loops on: the
    /// data flow missed a pointer (one walked in a loop, say), so the list,
    /// not the translation, is wrong.
    DataFlowMiss,
    /// The exe returned a value, but the engine read memory it never maps,
    /// in a function whose exe reads no address computed from its inputs
    /// (`Pure::computed_reads` false): the translation read memory the exe
    /// didn't, such as a number used as a pointer (arguments in the wrong
    /// order, a field dereferenced that the exe only computes with).
    RustStrayRead,
}

use crate::vectors::ARENA;

/// How much of the arena the engine maps around the buffers (zero beyond
/// them, as nv-call's arena is).
const ARENA_MAP: u32 = 0x0010_0000;

/// Fails loudly when the engine heap (its top `top`) has grown into the arena.
fn heap_guard(top: u32, id: &str, when: &str) -> Result<(), String> {
    if top >= ARENA {
        Err(format!(
            "engine heap top {top:08x} reached the buffer arena at {ARENA:08x} ({when} vector {id}); \
             the pointer-argument check cannot run"
        ))
    } else {
        Ok(())
    }
}

impl Kind {
    pub fn is_failure(&self) -> bool {
        !matches!(
            self,
            Kind::Match
                | Kind::ExeImport
                | Kind::BothFailed
                | Kind::OutOfImageRead
                | Kind::ExeTimeout
                | Kind::ExeSlow
                | Kind::ExeCrash
        )
    }
}

/// What `Engine::with_exe` maps of FalloutNV.exe 1.4.0.525: `.rdata`, `.data`,
/// `.tls` and `CONST` (0x00fdf000 to the end of CONST). A panic on a read
/// inside this range is a real difference; outside it the engine has no
/// bytes to give, whatever the exe read there.
const IMAGE: std::ops::Range<u32> = 0x00fd_f000..0x0127_4000;

/// The address of an engine "read of unmapped memory at <hex>" panic.
fn unmapped_read(msg: &str) -> Option<u32> {
    let rest = msg.split("unmapped memory at ").nth(1)?;
    let hex: String = rest.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    u32::from_str_radix(&hex, 16).ok()
}

pub struct Report {
    pub functions: usize,
    pub skipped: Vec<(u32, String)>,
    pub counts: BTreeMap<Kind, usize>,
    /// (address, unit, vector id, inputs, exe, rust, kind) for every failure.
    pub failures: Vec<(u32, String, String, String, String, String, Kind)>,
    pub failing_functions: BTreeMap<u32, usize>,
    /// Functions whose exe results (values, not faults) take two or more
    /// distinct values over the inputs: the matches there are not vacuous.
    pub varying: usize,
    /// Functions whose exe results are all the same value (or that only
    /// faulted): a match there only shows the constant is right.
    pub constant: Vec<u32>,
    /// f32 results where the exe's ST0 holds more precision than a float
    /// (ST0 rounded to double differs from ST0 rounded to float): the `f32`
    /// signature drops bits a caller using ST0 directly would see.
    /// Informational; per function, the number of vectors.
    pub extended: BTreeMap<u32, usize>,
    /// (address, vector id, engine panic) of every `OutOfImageRead`: listed
    /// in coverage.tsv, since they say nothing about the result.
    pub out_of_image: Vec<(u32, String, String)>,
}

fn expected_pop(p: &Pure) -> u32 {
    let words: usize = p.params.iter().map(|t| t.words()).sum();
    let stack = match p.cc {
        "thiscall" => words.saturating_sub(1),
        "fastcall" => words.saturating_sub(2),
        _ => words,
    };
    match p.cc {
        "cdecl" => 0,
        _ => 4 * stack as u32,
    }
}

fn is_nan32(b: u32) -> bool {
    f32::from_bits(b).is_nan()
}

fn is_nan64(b: u64) -> bool {
    f64::from_bits(b).is_nan()
}

/// (exe text, rust text, kind)
fn compare(
    ret: Ty,
    exe: &Outcome,
    rust: &Result<Ret, String>,
    want_pop: u32,
) -> (String, String, Kind) {
    let rust_text = match rust {
        Ok(r) => format!("eax=0x{:08x} edx=0x{:08x} st0={:?}", r.eax, r.edx, r.st0),
        Err(m) => format!("panic: {m}"),
    };
    let (eax, edx, f32b, f64b, popped) = match exe {
        Outcome::Fault {
            text,
            name,
            read_at,
        } => {
            // The exe needed Windows here: nothing past that call is comparable.
            if text.contains("\"kind\":\"import\"") {
                return (format!("fault {text}"), rust_text, Kind::ExeImport);
            }
            let k = match rust {
                Ok(_) => Kind::ExeFaultOnly,
                Err(m) => match (read_at, unmapped_read(m)) {
                    (Some(a), Some(b)) if *a == b => Kind::BothFailed,
                    // The engine stopped at memory it never maps; the exe read
                    // that address (mapped in nv-call's process) and failed later.
                    (_, Some(b)) if !IMAGE.contains(&b) => Kind::OutOfImageRead,
                    // Rust's own panics ("attempt to divide by zero", "... remainder with a
                    // divisor of zero") and translations that model the fault explicitly,
                    // each in its own words ("integer divide error (#DE) in ...", "integer
                    // divide fault at 0x00e2f8c9", "division by zero in the game"): any
                    // panic about a division, when the exe raised #DE.
                    _ if name == "integer_divide_by_zero"
                        && ["divide", "division", "divisor"]
                            .iter()
                            .any(|w| m.to_ascii_lowercase().contains(w)) =>
                    {
                        Kind::BothFailed
                    }
                    _ => Kind::FaultMismatch,
                },
            };
            return (format!("fault {text}"), rust_text, k);
        }
        Outcome::Value {
            eax,
            edx,
            st0_f32,
            st0_f64,
            popped,
        } => (*eax, *edx, *st0_f32, *st0_f64, *popped),
    };
    let r = match rust {
        Ok(r) => *r,
        Err(m) => {
            let k = match unmapped_read(m) {
                Some(a) if !IMAGE.contains(&a) => Kind::OutOfImageRead,
                _ => Kind::RustPanicOnly,
            };
            return (format!("eax=0x{eax:08x}"), rust_text, k);
        }
    };
    if popped != want_pop {
        return (
            format!("popped {popped} bytes, list expects {want_pop}"),
            rust_text,
            Kind::Convention,
        );
    }
    let (exe_text, kind) = match ret {
        Ty::Bool => {
            let (a, b) = (eax as u8 != 0, r.eax as u8 != 0);
            (
                format!("al={}", eax as u8),
                if a == b { Kind::Match } else { Kind::Mismatch },
            )
        }
        Ty::U8 | Ty::I8 => (
            format!("al=0x{:02x}", eax as u8),
            if eax as u8 == r.eax as u8 {
                Kind::Match
            } else {
                Kind::Mismatch
            },
        ),
        Ty::U16 | Ty::I16 => (
            format!("ax=0x{:04x}", eax as u16),
            if eax as u16 == r.eax as u16 {
                Kind::Match
            } else {
                Kind::Mismatch
            },
        ),
        Ty::U64 => (
            format!("edx:eax=0x{edx:08x}{eax:08x}"),
            if eax == r.eax && edx == r.edx {
                Kind::Match
            } else {
                Kind::Mismatch
            },
        ),
        Ty::F32 => match f32b {
            None => ("ST0 empty".into(), Kind::Mismatch),
            Some(b) => {
                let rb = (r.st0 as f32).to_bits();
                let k = if b == rb {
                    Kind::Match
                } else if is_nan32(b) && is_nan32(rb) {
                    Kind::NanBits
                } else {
                    Kind::Mismatch
                };
                (
                    format!("st0={:?}(0x{b:08x}) rust_f32=0x{rb:08x}", f32::from_bits(b)),
                    k,
                )
            }
        },
        Ty::F64 => match f64b {
            None => ("ST0 empty".into(), Kind::Mismatch),
            Some(b) => {
                let rb = r.st0.to_bits();
                let k = if b == rb {
                    Kind::Match
                } else if is_nan64(b) && is_nan64(rb) {
                    Kind::NanBits
                } else {
                    Kind::Mismatch
                };
                (format!("st0={:?}(0x{b:016x})", f64::from_bits(b)), k)
            }
        },
        _ => (
            format!("eax=0x{eax:08x}"),
            if eax == r.eax {
                Kind::Match
            } else {
                Kind::Mismatch
            },
        ),
    };
    (exe_text, rust_text, kind)
}

/// `Engine::with_exe` without rereading the file: the checked data sections
/// and imports parsed once, mapped into a new engine per vector (the same
/// steps as `engine::exe::map_data_sections` and `Engine::with_exe`), with
/// `--fpcw` set as each new engine's main-thread x87 control word.
struct FreshEngine {
    sections: Vec<(u32, u32, Vec<u8>)>,
    imports: Vec<engine::exe::Import>,
    fpcw: Option<u16>,
}

impl FreshEngine {
    fn load(exe: &Path, fpcw: Option<u16>) -> Result<Self, String> {
        let file = fs::read(exe).map_err(|e| format!("{}: {e}", exe.display()))?;
        let secs = engine::exe::checked_sections(exe, &file)?;
        let mut sections = Vec::new();
        for (name, va, vsize, _) in engine::exe::SECTIONS {
            let s = secs
                .iter()
                .find(|s| s.name == name)
                .ok_or_else(|| format!("no {name} section"))?;
            let n = s.raw.len().min(vsize as usize);
            sections.push((va, vsize.max(s.raw.len() as u32), s.raw[..n].to_vec()));
        }
        Ok(FreshEngine {
            sections,
            imports: engine::exe::imports(&file)?,
            fpcw,
        })
    }

    fn make(&self) -> Engine {
        let mut e = Engine::new();
        for (va, len, bytes) in &self.sections {
            e.mem.map(*va, *len);
            e.mem.write(*va, bytes);
        }
        e.register_imports(&self.imports);
        if let Some(word) = self.fpcw {
            engine::units::crt::set_x87_control_word(&mut e, word);
        }
        e
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn panic_text(p: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else {
        "panic".into()
    }
}

pub fn run(list: &[Pure], o: &Options) -> Result<Report, String> {
    if !o.nv_call.is_file() {
        return Err(format!(
            "{} not found; build it with `cargo build --release --target i686-pc-windows-msvc` in research/nv-oracle",
            o.nv_call.display()
        ));
    }
    fs::create_dir_all(&o.out_dir).map_err(|e| format!("{}: {e}", o.out_dir.display()))?;

    // Which functions the engine can run: itself and every callee translated.
    let probe = Engine::with_exe(&o.exe)?;
    let mut skipped = Vec::new();
    let mut runnable = Vec::new();
    for p in list {
        if !probe.is_translated(p.addr) {
            skipped.push((p.addr, "not registered in the engine".to_string()));
        } else if let Some(c) = p.callees.iter().find(|c| !probe.is_translated(**c)) {
            skipped.push((p.addr, format!("callee {c:08x} is not translated")));
        } else {
            runnable.push(p);
        }
    }
    drop(probe);

    // Every vector, as (id, line), in order.
    let mut items: Vec<(String, String)> = Vec::new();
    let mut cases: Vec<(&Pure, Vec<Case>)> = Vec::new();
    for p in &runnable {
        let cs: Vec<Case> = if p.bufs.is_empty() {
            vectors::inputs(&p.params, o.per_function, p.addr as u64, &p.consts)
                .into_iter()
                .map(|args| Case {
                    args,
                    bufs: Vec::new(),
                })
                .collect()
        } else {
            vectors::pointer_cases(&p.params, &p.bufs, o.per_function, p.addr as u64, &p.consts)
        };
        for (i, c) in cs.iter().enumerate() {
            let id = format!("{:08x}_{i}", p.addr);
            let line = if p.bufs.is_empty() {
                vectors::vector_line(&id, p.addr, p.cc, &c.args, p.ret)
            } else {
                vectors::vector_line_buffers(&id, p.addr, p.cc, c, &p.bufs, p.ret)
            };
            items.push((id, line));
        }
        cases.push((p, cs));
    }
    // The exe side: nv-call, streamed and restarted after a vector that runs
    // too long or ends its process.
    let buffers = cases.iter().any(|(p, _)| !p.bufs.is_empty());
    let exe = run_nv_call(o, &items, buffers)?;
    let res_path = o.out_dir.join("results.jsonl");
    fs::write(&res_path, exe.lines.join("\n"))
        .map_err(|e| format!("{}: {e}", res_path.display()))?;
    let exe_results: BTreeMap<String, Outcome> = exe
        .lines
        .iter()
        .filter_map(|l| vectors::parse_result(l))
        .collect();
    let exe_buffers: BTreeMap<String, vectors::Buffers> = exe
        .lines
        .iter()
        .filter_map(|l| vectors::parse_buffers(l))
        .collect();

    // The engine side on a worker thread, under a watchdog: a vector the exe
    // finished in time but the engine doesn't finish at all stops the check.
    let template = FreshEngine::load(&o.exe, o.fpcw)?;
    let progress = AtomicU64::new(0);
    let current = Mutex::new(String::new());
    let quiet = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let joined = std::thread::scope(|scope| {
        // The engine's large translations (block state machines with big
        // frames) need more than a spawned thread's default 2 MB stack.
        let worker = std::thread::Builder::new()
            .stack_size(256 << 20)
            .spawn_scoped(scope, || {
                engine_side(
                    &cases,
                    &template,
                    &exe,
                    &exe_results,
                    &exe_buffers,
                    &progress,
                    &current,
                )
            })
            .expect("spawn the engine-side worker");
        let mut seen = 0;
        let mut since = Instant::now();
        while !worker.is_finished() {
            std::thread::sleep(Duration::from_millis(200));
            let now = progress.load(Ordering::Relaxed);
            if now != seen {
                seen = now;
                since = Instant::now();
            } else if since.elapsed() > o.rust_timeout {
                let what = current.lock().map(|c| c.clone()).unwrap_or_default();
                eprintln!(
                    "nv-pure: the engine side ran over {}s on {what}, a vector the exe finished \
                     within {}ms: a translation that loops where the exe returns (or a very slow \
                     one). Stopping.",
                    o.rust_timeout.as_secs(),
                    o.slow.as_millis()
                );
                std::process::exit(3);
            }
        }
        worker.join()
    });
    panic::set_hook(quiet);
    let mut report = match joined {
        Ok(r) => r?,
        Err(_) => return Err("the engine worker thread panicked".into()),
    };
    report.skipped = skipped;

    let mut tsv = String::from("address\tunit\tvector\tinputs\texe\trust\tkind\n");
    for (a, u, id, ins, ex, ru, k) in &report.failures {
        let _ = writeln!(tsv, "{a:08x}\t{u}\t{id}\t{ins}\t{ex}\t{ru}\t{k:?}");
    }
    let rep = o.out_dir.join("failures.tsv");
    fs::write(&rep, tsv).map_err(|e| format!("{}: {e}", rep.display()))?;

    // Coverage notes: which matches only show a constant, which f32 results
    // were wider than a float in ST0, and which vectors the exe didn't finish.
    let mut notes = String::from("address\tunit\tnote\n");
    let unit = |a: u32| {
        cases
            .iter()
            .find(|(p, _)| p.addr == a)
            .map(|(p, _)| p.unit.clone())
            .unwrap_or_default()
    };
    for a in &report.constant {
        let _ = writeln!(
            notes,
            "{a:08x}\t{}\tconstant or fault-only exe results",
            unit(*a)
        );
    }
    for (a, id, rust) in &report.out_of_image {
        let _ = writeln!(
            notes,
            "{a:08x}\t{}\tout-of-image read (inconclusive): vector {id}, engine {rust}",
            unit(*a)
        );
    }
    for (a, n) in &report.extended {
        let _ = writeln!(
            notes,
            "{a:08x}\t{}\tf32 result wider than a float in {n} vector(s)",
            unit(*a)
        );
    }
    for (what, ids) in [
        ("exe timeout", &exe.timed_out),
        ("exe slow", &exe.slow),
        ("exe crash", &exe.crashed),
    ] {
        for id in ids {
            let a = u32::from_str_radix(id.split('_').next().unwrap_or(""), 16).unwrap_or(0);
            let _ = writeln!(notes, "{a:08x}\t{}\t{what}: vector {id}", unit(a));
        }
    }
    let path = o.out_dir.join("coverage.tsv");
    fs::write(&path, notes).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(report)
}

/// What nv-call produced: its result lines, and the vectors it did not finish.
pub struct ExeRun {
    pub lines: Vec<String>,
    pub timed_out: BTreeSet<String>,
    pub slow: BTreeSet<String>,
    pub crashed: BTreeSet<String>,
}

/// Fails unless a header line reports nv-call's arena at [`ARENA`].
fn check_arena(header: &str) -> Result<(), String> {
    let base = crate::json::Value::parse(header).and_then(|h| {
        h.get("arena_base")
            .and_then(|v| v.as_str())
            .map(str::to_string)
    });
    let at = base
        .as_deref()
        .and_then(|b| u32::from_str_radix(b.trim_start_matches("0x"), 16).ok());
    if at == Some(ARENA) {
        Ok(())
    } else {
        Err(format!(
            "nv-call placed its buffer arena at {base:?}, not {ARENA:#010x}; the engine maps the \
             buffers at nv-call's addresses, so the pointer-argument check cannot run"
        ))
    }
}

/// Runs the vectors through nv-call, reading its output as it comes. A vector
/// with no result line within `exe_timeout` is abandoned: nv-call is stopped and
/// started again on the vectors after it. The same when nv-call's process ends
/// on a vector. A vector that took longer than `slow` is noted.
fn run_nv_call(o: &Options, items: &[(String, String)], buffers: bool) -> Result<ExeRun, String> {
    use std::io::{BufRead, BufReader};
    use std::process::Stdio;
    use std::sync::mpsc::{self, RecvTimeoutError};

    let mut run = ExeRun {
        lines: Vec::new(),
        timed_out: BTreeSet::new(),
        slow: BTreeSet::new(),
        crashed: BTreeSet::new(),
    };
    let all: String = items.iter().map(|(_, l)| format!("{l}\n")).collect();
    let vec_path = o.out_dir.join("vectors.jsonl");
    fs::write(&vec_path, all).map_err(|e| format!("{}: {e}", vec_path.display()))?;
    let part = o.out_dir.join("vectors.part.jsonl");
    let mut start = 0;
    while start < items.len() {
        let text: String = items[start..]
            .iter()
            .map(|(_, l)| format!("{l}\n"))
            .collect();
        fs::write(&part, text).map_err(|e| format!("{}: {e}", part.display()))?;
        let mut command = Command::new(&o.nv_call);
        command.arg(&o.exe).arg(&part);
        if let Some(word) = o.fpcw {
            command.arg("--fpcw").arg(format!("0x{word:04x}"));
        }
        let mut child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("running {}: {e}", o.nv_call.display()))?;
        let out = child.stdout.take().ok_or("nv-call has no stdout")?;
        let (tx, rx) = mpsc::channel::<String>();
        std::thread::spawn(move || {
            for l in BufReader::new(out).lines().map_while(Result::ok) {
                if tx.send(l).is_err() {
                    break;
                }
            }
        });
        let mut next = start;
        let mut header = false;
        let mut last = Instant::now();
        loop {
            match rx.recv_timeout(o.exe_timeout) {
                Ok(line) => {
                    if line.contains("\"type\":\"header\"") {
                        header = true;
                        if buffers {
                            if let Err(e) = check_arena(&line) {
                                let _ = child.kill();
                                let _ = child.wait();
                                return Err(e);
                            }
                        }
                        last = Instant::now();
                        continue;
                    }
                    let result = line.contains("\"type\":\"result\"");
                    if !(result || line.contains("\"type\":\"error\"")) {
                        continue;
                    }
                    if next < items.len() && last.elapsed() > o.slow {
                        run.slow.insert(items[next].0.clone());
                    }
                    last = Instant::now();
                    if result {
                        run.lines.push(line);
                    }
                    next += 1;
                }
                Err(RecvTimeoutError::Timeout) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    if next < items.len() {
                        run.timed_out.insert(items[next].0.clone());
                    }
                    start = next + 1;
                    break;
                }
                Err(RecvTimeoutError::Disconnected) => {
                    let status = child.wait().map_err(|e| format!("nv-call: {e}"))?;
                    if !header {
                        return Err(format!(
                            "nv-call stopped before its header (exit {:?}): setup failed",
                            status.code()
                        ));
                    }
                    if next < items.len() {
                        if matches!(status.code(), Some(0) | Some(2)) {
                            return Err(format!(
                                "nv-call exited ({:?}) with {} vector(s) unanswered",
                                status.code(),
                                items.len() - next
                            ));
                        }
                        run.crashed.insert(items[next].0.clone());
                        start = next + 1;
                    } else {
                        start = items.len();
                    }
                    break;
                }
            }
        }
    }
    let _ = fs::remove_file(&part);
    Ok(run)
}

/// The engine side for every vector (on the watchdog's worker thread).
fn engine_side(
    cases: &[(&Pure, Vec<Case>)],
    template: &FreshEngine,
    exe: &ExeRun,
    exe_results: &BTreeMap<String, Outcome>,
    exe_buffers: &BTreeMap<String, vectors::Buffers>,
    progress: &AtomicU64,
    current: &Mutex<String>,
) -> Result<Report, String> {
    let mut report = Report {
        functions: cases.len(),
        skipped: Vec::new(),
        counts: BTreeMap::new(),
        failures: Vec::new(),
        failing_functions: BTreeMap::new(),
        varying: 0,
        constant: Vec::new(),
        extended: BTreeMap::new(),
        out_of_image: Vec::new(),
    };
    for (p, cs) in cases {
        let want_pop = expected_pop(p);
        let mut distinct = std::collections::BTreeSet::new();
        for (i, case) in cs.iter().enumerate() {
            let id = format!("{:08x}_{i}", p.addr);
            progress.fetch_add(1, Ordering::Relaxed);
            let shown = || {
                let mut shown: Vec<String> = case
                    .args
                    .iter()
                    .enumerate()
                    .map(|(k, v)| match p.bufs.iter().position(|b| b.param == k) {
                        Some(j) if v.ty == Ty::Ptr => format!("&b{j}"),
                        _ => v.show(),
                    })
                    .collect();
                for (j, init) in case.bufs.iter().enumerate() {
                    shown.push(format!("b{j}={}", hex(init)));
                }
                shown.join(", ")
            };
            let unfinished = if exe.timed_out.contains(&id) {
                Some(Kind::ExeTimeout)
            } else if exe.crashed.contains(&id) {
                Some(Kind::ExeCrash)
            } else if exe.slow.contains(&id) {
                Some(Kind::ExeSlow)
            } else {
                None
            };
            // A read-only pointer function loops only over counts from its
            // fields, with no varying reads: an exe that runs past the limit
            // walked a pointer the data flow missed (005f65d0, 00a089c0). An
            // exe process that ends on the vector is read the same way (it
            // can neither fault nor loop on its buffers).
            // A stack overflow in the exe is the same miss (a pointer cycle the
            // data flow did not see, as in 00b15110 with a branch that is its
            // own parent), and the engine side cannot run it: a Rust stack
            // overflow ends the whole check.
            let overflowed = matches!(
                exe_results.get(&id),
                Some(Outcome::Fault { name, .. }) if name == "stack_overflow"
            );
            let stuck = overflowed || matches!(unfinished, Some(Kind::ExeTimeout | Kind::ExeCrash));
            if stuck && !p.bufs.is_empty() {
                *report.counts.entry(Kind::DataFlowMiss).or_insert(0) += 1;
                *report.failing_functions.entry(p.addr).or_insert(0) += 1;
                report.failures.push((
                    p.addr,
                    p.unit.clone(),
                    id,
                    shown(),
                    if overflowed {
                        "stack overflow in the exe (a pointer cycle)".into()
                    } else if unfinished == Some(Kind::ExeTimeout) {
                        "no result within --exe-timeout (a loop)".into()
                    } else {
                        "nv-call's process ended on this vector".into()
                    },
                    String::new(),
                    Kind::DataFlowMiss,
                ));
                continue;
            }
            if let Some(k) = unfinished {
                *report.counts.entry(k).or_insert(0) += 1;
                continue;
            }
            if overflowed {
                // A scalar function that recurses without end on its inputs:
                // nothing the engine side can be run against.
                *report.counts.entry(Kind::ExeCrash).or_insert(0) += 1;
                continue;
            }
            if let Ok(mut c) = current.lock() {
                *c = format!("{id} ({}) with [{}]", p.unit, shown());
            }
            let mut e = template.make();
            heap_guard(e.mem.heap_top(), &id, "before")?;
            // The buffers where nv-call puts them: the arena base, 16-aligned, in order.
            let addrs = vectors::buffer_addrs(&p.bufs);
            let used = addrs
                .last()
                .zip(p.bufs.last())
                .map_or(0, |(a, b)| a - ARENA + b.size);
            if !p.bufs.is_empty() {
                e.mem.map(ARENA, ARENA_MAP.max((used + 0xffff) & !0xffff));
                for (a, init) in addrs.iter().zip(&case.bufs) {
                    e.mem.write(*a, init);
                }
            }
            let mut words = Vec::new();
            for (k, v) in case.args.iter().enumerate() {
                match p.bufs.iter().position(|b| b.param == k) {
                    Some(j) if v.ty == Ty::Ptr => words.push(addrs[j] + p.bufs[j].bias),
                    _ => v.put(&mut words),
                }
            }
            let rust = panic::catch_unwind(AssertUnwindSafe(|| e.call(p.addr, &words)))
                .map_err(panic_text);
            heap_guard(e.mem.heap_top(), &id, "after")?;
            let rust_bufs: Vec<Vec<u8>> = addrs
                .iter()
                .zip(&p.bufs)
                .map(|(a, b)| e.mem.bytes(*a, b.size))
                .collect();
            if let Some(Outcome::Value {
                eax,
                edx,
                st0_f32,
                st0_f64,
                ..
            }) = exe_results.get(&id)
            {
                let key = match p.ret {
                    Ty::F32 => st0_f32.map(u64::from).unwrap_or(u64::MAX),
                    Ty::F64 => st0_f64.unwrap_or(u64::MAX),
                    Ty::Bool => u64::from(*eax as u8 != 0),
                    Ty::U8 | Ty::I8 => u64::from(*eax as u8),
                    Ty::U16 | Ty::I16 => u64::from(*eax as u16),
                    Ty::U64 => u64::from(*eax) | u64::from(*edx) << 32,
                    _ => u64::from(*eax),
                };
                distinct.insert(key);
                if let (Ty::F32, Some(f), Some(d)) = (p.ret, st0_f32, st0_f64) {
                    let (f, d) = (f32::from_bits(*f), f64::from_bits(*d));
                    if !d.is_nan() && f as f64 != d {
                        *report.extended.entry(p.addr).or_insert(0) += 1;
                    }
                }
            }
            let Some(exe_out) = exe_results.get(&id) else {
                *report.counts.entry(Kind::Mismatch).or_insert(0) += 1;
                *report.failing_functions.entry(p.addr).or_insert(0) += 1;
                report.failures.push((
                    p.addr,
                    p.unit.clone(),
                    id,
                    shown(),
                    "no result line (nv-call rejected the vector)".into(),
                    String::new(),
                    Kind::Mismatch,
                ));
                continue;
            };
            let (mut exe_text, mut rust_text, mut kind) = compare(p.ret, exe_out, &rust, want_pop);
            if let (false, Outcome::Fault { name, .. }) = (p.bufs.is_empty(), exe_out) {
                if name == "access_violation" {
                    kind = Kind::DataFlowMiss;
                }
            }
            // The exe returned normally, reading only its buffers, image
            // constants and the stack: an engine read anywhere else is the
            // translation's own (00448a60 with its arguments swapped).
            if kind == Kind::OutOfImageRead
                && matches!(exe_out, Outcome::Value { .. })
                && !p.computed_reads
            {
                kind = Kind::RustStrayRead;
            }
            if kind == Kind::OutOfImageRead {
                report
                    .out_of_image
                    .push((p.addr, id.clone(), rust_text.clone()));
            }
            if kind == Kind::Match && !p.bufs.is_empty() {
                let exe_b = exe_buffers.get(&id);
                for (j, rb) in rust_bufs.iter().enumerate() {
                    let name = format!("b{j}");
                    let eb = exe_b
                        .and_then(|v| v.iter().find(|(n, _)| *n == name))
                        .map(|(_, b)| b);
                    if eb != Some(rb) {
                        kind = Kind::BufferMismatch;
                        exe_text =
                            format!("{name} after = {}", eb.map(|b| hex(b)).unwrap_or_default());
                        rust_text = format!("{name} after = {}", hex(rb));
                        break;
                    }
                }
            }
            *report.counts.entry(kind.clone()).or_insert(0) += 1;
            if kind.is_failure() {
                *report.failing_functions.entry(p.addr).or_insert(0) += 1;
                report.failures.push((
                    p.addr,
                    p.unit.clone(),
                    id,
                    shown(),
                    exe_text,
                    rust_text,
                    kind,
                ));
            }
        }
        if distinct.len() >= 2 {
            report.varying += 1;
        } else {
            report.constant.push(p.addr);
        }
    }
    Ok(report)
}

/// `%USERPROFILE%\nv-re\oracle\pure` (private: results hold the exe's outputs).
pub fn default_out_dir() -> PathBuf {
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .unwrap_or_default();
    Path::new(&home).join("nv-re").join("oracle").join("pure")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn val(eax: u32, f: Option<u32>) -> Outcome {
        Outcome::Value {
            eax,
            edx: 0,
            st0_f32: f,
            st0_f64: None,
            popped: 0,
        }
    }

    #[test]
    fn verdicts() {
        let ok = |eax: u32, st0: f64| Ok::<Ret, String>(Ret { eax, edx: 0, st0 });
        assert_eq!(
            compare(Ty::Bool, &val(0x1234_5601, None), &ok(1, 0.0), 0).2,
            Kind::Match
        );
        assert_eq!(
            compare(Ty::U32, &val(5, None), &ok(6, 0.0), 0).2,
            Kind::Mismatch
        );
        assert_eq!(
            compare(Ty::F32, &val(0, Some(0x3fc0_0000)), &ok(0, 1.5), 0).2,
            Kind::Match
        );
        assert_eq!(
            compare(Ty::F32, &val(0, Some(0x7fc0_0001)), &ok(0, f64::NAN), 0).2,
            Kind::NanBits
        );
        assert_eq!(
            compare(Ty::U32, &val(5, None), &ok(5, 0.0), 4).2,
            Kind::Convention
        );
        let fault = |at: u32| Outcome::Fault {
            text: "{}".into(),
            name: "access_violation".into(),
            read_at: Some(at),
        };
        let read = |at: &str| Err::<Ret, String>(format!("read of unmapped memory at {at}"));
        assert_eq!(
            compare(Ty::U32, &fault(0x021f_91c8), &read("021f91c8"), 0).2,
            Kind::BothFailed,
            "same address"
        );
        assert_eq!(
            compare(Ty::U32, &fault(0x0400_0000), &read("011f91c8"), 0).2,
            Kind::FaultMismatch,
            "engine stopped inside its mapped data, the exe elsewhere"
        );
        assert_eq!(
            compare(Ty::U32, &fault(0x0400_0000), &read("00b89904"), 0).2,
            Kind::OutOfImageRead,
            "engine stopped at .text, which it doesn't map"
        );
        assert_eq!(
            compare(
                Ty::U32,
                &fault(0x0400_0000),
                &Err("open function 00401000".into()),
                0
            )
            .2,
            Kind::FaultMismatch
        );
        assert_eq!(
            compare(Ty::U32, &fault(0x0400_0000), &ok(1, 0.0), 0).2,
            Kind::ExeFaultOnly
        );
        // The exe called an import (the CRT's errno path): inconclusive, whatever the engine did.
        let import = Outcome::Fault {
            text: r#"{"kind":"import","name":"KERNEL32.dll!GetLastError","index":26}"#.into(),
            name: "KERNEL32.dll!GetLastError".into(),
            read_at: None,
        };
        for rust in [Err("open function 00000000".into()), ok(1, 0.0)] {
            assert_eq!(compare(Ty::U32, &import, &rust, 0).2, Kind::ExeImport);
        }
        let div = Outcome::Fault {
            text: "{}".into(),
            name: "integer_divide_by_zero".into(),
            read_at: None,
        };
        for m in [
            "attempt to divide by zero",
            "attempt to calculate the remainder with a divisor of zero",
            "hash map without buckets (division by zero in the game)",
            "integer divide error (#DE) in 00e39e60: 3 / 0",
            "00e062a0: the stride at +0x2a is zero (divide error)",
            "integer divide fault at 0x00e2f8c9",
        ] {
            assert_eq!(
                compare(Ty::U32, &div, &Err(m.into()), 0).2,
                Kind::BothFailed,
                "{m}"
            );
        }
        let oob = Err::<Ret, String>("read of unmapped memory at 3054e150".into());
        assert_eq!(
            compare(Ty::U32, &val(0, None), &oob, 0).2,
            Kind::OutOfImageRead
        );
        let text = Err::<Ret, String>("read of unmapped memory at 00b89904".into());
        assert_eq!(
            compare(Ty::U32, &val(0, None), &text, 0).2,
            Kind::OutOfImageRead
        );
        let data = Err::<Ret, String>("read of unmapped memory at 011f91c8".into());
        assert_eq!(
            compare(Ty::U32, &val(0, None), &data, 0).2,
            Kind::RustPanicOnly
        );
    }

    #[test]
    fn guards() {
        assert!(heap_guard(ARENA - 8, "x_0", "before").is_ok());
        let e = heap_guard(ARENA, "0069ae60_6", "after").unwrap_err();
        assert!(e.contains("after vector 0069ae60_6"), "{e}");
        assert!(heap_guard(ARENA + 0x100, "x_0", "before").is_err());

        let header =
            |base: &str| format!(r#"{{"type":"header","tool":"nv-call","arena_base":{base}}}"#);
        assert!(check_arena(&header("\"0x30000000\"")).is_ok());
        for bad in ["\"0x31000000\"", "\"junk\"", "805306368", "null"] {
            assert!(check_arena(&header(bad)).is_err(), "{bad}");
        }
        assert!(check_arena(r#"{"type":"header","tool":"nv-call"}"#).is_err());
        assert!(check_arena("not json").is_err());
    }
}
