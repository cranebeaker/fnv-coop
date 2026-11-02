//! families: look-alike families of FalloutNV.exe functions.
//!
//! `fetch` caches every function's disassembly from the read-only Ghidra
//! server in a private directory. `group` normalises each function's
//! instructions (see `normalise`), groups functions whose normalised
//! sequences are equal at two levels (strict and loose), joins the groups
//! with the engine map and the ledger, and writes `families.tsv` and
//! `summary.txt`. `show` prints one function's families and translated
//! siblings (and, with `--shape`, its normalised shape) to the terminal.
//!
//! Nothing read from the executable reaches the outputs: they hold
//! addresses, counts, Xbox PDB names from the engine map, statuses and Rust
//! paths from the ledger, and hashes of the normalised shapes.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

const USAGE: &str = "\
usage:
  families fetch --map <engine_map.tsv> --cache <dir> [--ghidra <url>] [--threads <n>]
  families group --map <engine_map.tsv> --ledger <ledger.tsv> --cache <dir>
                 --out <families.tsv> --summary <summary.txt> [--focus-unit <unit>]...
  families show  --map <engine_map.tsv> --ledger <ledger.tsv> --cache <dir> [--shape] <address>...";

#[derive(Default)]
struct Args {
    command: String,
    map: Option<PathBuf>,
    ledger: Option<PathBuf>,
    cache: Option<PathBuf>,
    out: Option<PathBuf>,
    summary: Option<PathBuf>,
    ghidra: String,
    threads: usize,
    focus_units: Vec<String>,
    shape: bool,
    addresses: Vec<u32>,
}

fn args() -> Result<Args, String> {
    let mut a = Args {
        ghidra: "http://127.0.0.1:8090".into(),
        threads: 6,
        ..Default::default()
    };
    let mut it = std::env::args().skip(1);
    a.command = it.next().ok_or("no command")?;
    while let Some(arg) = it.next() {
        let mut value = || it.next().ok_or(format!("{arg} needs a value"));
        match arg.as_str() {
            "--map" => a.map = Some(value()?.into()),
            "--ledger" => a.ledger = Some(value()?.into()),
            "--cache" => a.cache = Some(value()?.into()),
            "--out" => a.out = Some(value()?.into()),
            "--summary" => a.summary = Some(value()?.into()),
            "--ghidra" => a.ghidra = value()?,
            "--threads" => a.threads = value()?.parse().map_err(|e| format!("--threads: {e}"))?,
            "--focus-unit" => a.focus_units.push(value()?),
            "--shape" => a.shape = true,
            s if a.command == "show" => a.addresses.push(
                u32::from_str_radix(s.trim_start_matches("0x"), 16)
                    .map_err(|_| format!("not an address: {s}"))?,
            ),
            s => return Err(format!("unknown argument {s}")),
        }
    }
    Ok(a)
}

fn need<'a>(p: &'a Option<PathBuf>, flag: &str) -> Result<&'a Path, String> {
    p.as_deref().ok_or(format!("{flag} is required"))
}

/// One engine-map row.
struct Function {
    addr: u32,
    size: u32,
    name: String,
    unit: String,
    subsystem: String,
}

fn engine_map(path: &Path) -> Result<Vec<Function>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.starts_with("address\t") {
            continue;
        }
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 7 {
            return Err(format!("short engine-map row: {line}"));
        }
        out.push(Function {
            addr: u32::from_str_radix(c[0], 16).map_err(|_| format!("bad address: {line}"))?,
            size: c[1].parse().map_err(|_| format!("bad size: {line}"))?,
            name: c[2].to_string(),
            unit: c[5].to_string(),
            subsystem: c[6].to_string(),
        });
    }
    Ok(out)
}

/// One ledger row: status and the first Rust citation.
struct Status {
    status: String,
    rust: String,
}

fn ledger(path: &Path) -> Result<HashMap<u32, Status>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = HashMap::new();
    for line in text.lines().skip(1) {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 6 {
            continue;
        }
        let Ok(addr) = u32::from_str_radix(c[0], 16) else { continue };
        let rust = c.get(6).and_then(|r| r.split(',').next()).unwrap_or("").to_string();
        out.insert(addr, Status { status: c[5].to_string(), rust });
    }
    Ok(out)
}

// ---------------------------------------------------------------- fetch

fn get(host: &str, addr: u32) -> Result<String, String> {
    let mut stream = TcpStream::connect(host).map_err(|e| format!("{host}: {e}"))?;
    write!(
        stream,
        "GET /disassemble_function?address=0x{addr:08x} HTTP/1.0\r\nHost: {host}\r\n\r\n"
    )
    .map_err(|e| e.to_string())?;
    let mut response = String::new();
    stream.read_to_string(&mut response).map_err(|e| e.to_string())?;
    let (head, body) = response.split_once("\r\n\r\n").ok_or("no HTTP body")?;
    if !head.starts_with("HTTP/1.1 200") && !head.starts_with("HTTP/1.0 200") {
        return Err(head.lines().next().unwrap_or("").to_string());
    }
    Ok(body.to_string())
}

fn fetch(a: &Args) -> Result<(), String> {
    let functions = engine_map(need(&a.map, "--map")?)?;
    let cache = need(&a.cache, "--cache")?.to_path_buf();
    std::fs::create_dir_all(&cache).map_err(|e| e.to_string())?;
    let host = a.ghidra.trim_start_matches("http://").trim_end_matches('/').to_string();
    let todo: Vec<u32> = functions
        .iter()
        .map(|f| f.addr)
        .filter(|addr| !cache.join(format!("{addr:08x}.txt")).exists())
        .collect();
    eprintln!("families: {} functions, {} to fetch", functions.len(), todo.len());
    let next = AtomicUsize::new(0);
    let failed = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..a.threads.max(1) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(&addr) = todo.get(i) else { break };
                if i % 2000 == 0 {
                    eprintln!("families: {i}/{}", todo.len());
                }
                let mut result = Err(String::new());
                for _ in 0..3 {
                    result = get(&host, addr);
                    if result.is_ok() {
                        break;
                    }
                }
                match result {
                    Ok(body) if body.lines().next().is_some_and(|l| l.len() > 10 && l.as_bytes()[8] == b':') => {
                        let path = cache.join(format!("{addr:08x}.txt"));
                        if let Err(e) = std::fs::write(&path, body) {
                            failed.lock().unwrap().push(format!("{addr:08x}: {e}"));
                        }
                    }
                    Ok(body) => failed
                        .lock()
                        .unwrap()
                        .push(format!("{addr:08x}: {}", body.lines().next().unwrap_or("empty"))),
                    Err(e) => failed.lock().unwrap().push(format!("{addr:08x}: {e}")),
                }
            });
        }
    });
    let failed = failed.into_inner().unwrap();
    for f in failed.iter().take(20) {
        eprintln!("families: not fetched {f}");
    }
    eprintln!("families: {} not fetched", failed.len());
    Ok(())
}

// ------------------------------------------------------------ normalise

/// The two shape levels.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Level {
    /// Registers, stack and struct offsets, scales and small immediates
    /// kept; addresses, branch-out and call targets and large constants
    /// masked; branches inside the function become instruction indices.
    Strict,
    /// Strict, with every immediate, displacement and scale masked too.
    Loose,
}

const REGISTERS: &[&str] = &[
    "EAX", "EBX", "ECX", "EDX", "ESI", "EDI", "EBP", "ESP", "AX", "BX", "CX", "DX", "SI", "DI",
    "BP", "SP", "AL", "AH", "BL", "BH", "CL", "CH", "DL", "DH", "CS", "DS", "ES", "FS", "GS", "SS",
    "ST0", "ST1", "ST2", "ST3", "ST4", "ST5", "ST6", "ST7", "MM0", "MM1", "MM2", "MM3", "MM4",
    "MM5", "MM6", "MM7", "XMM0", "XMM1", "XMM2", "XMM3", "XMM4", "XMM5", "XMM6", "XMM7",
];

/// The executable's image range: a value in it is taken for an address.
fn is_address(v: u32) -> bool {
    (0x0040_0000..0x0130_0000).contains(&v)
}

/// A number as Ghidra prints it (`0x1f`, `-0x8`, `12`).
fn number(s: &str) -> Option<u32> {
    let (neg, s) = match s.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, s),
    };
    let v = match s.strip_prefix("0x") {
        Some(h) => u32::from_str_radix(h, 16).ok()?,
        None => s.parse().ok()?,
    };
    Some(if neg { v.wrapping_neg() } else { v })
}

/// An immediate or displacement value.
fn value(v: u32, level: Level) -> String {
    if is_address(v) {
        "A".into()
    } else if (0x0130_0000..0xffff_0000).contains(&v) {
        // Float bit patterns, hashes, masks, other globals' addresses.
        "K".into()
    } else if level == Level::Loose {
        "I".into()
    } else if v >= 0xffff_0000 {
        format!("-{:x}", v.wrapping_neg())
    } else {
        format!("{v:x}")
    }
}

/// A memory operand `size ptr SEG:[terms]`.
fn memory(op: &str, level: Level) -> String {
    let (prefix, rest) = op.split_once('[').unwrap_or(("", op));
    let inner = rest.trim_end_matches(']');
    let terms: Vec<String> = inner
        .split(" + ")
        .map(|t| {
            let t = t.trim();
            if let Some((r, scale)) = t.split_once('*') {
                if level == Level::Loose {
                    format!("{r}*S")
                } else {
                    format!("{r}*{}", number(scale).map_or(scale.to_string(), |v| v.to_string()))
                }
            } else if let Some(v) = number(t) {
                value(v, level)
            } else {
                t.to_string()
            }
        })
        .collect();
    format!("{}[{}]", prefix.trim(), terms.join("+"))
}

/// One instruction (`MNEMONIC op,op`) at `level`. `index` maps addresses of
/// the function's own instructions to their position.
fn instruction(ins: &str, level: Level, index: &HashMap<u32, usize>) -> String {
    let (mnemonic, ops) = ins.split_once(' ').unwrap_or((ins, ""));
    let ops = ops.trim();
    let branch = mnemonic == "CALL" || mnemonic.starts_with('J') || mnemonic.starts_with("LOOP");
    let mut out = mnemonic.to_string();
    if ops.is_empty() {
        return out;
    }
    let mut parts = Vec::new();
    for op in ops.split(',') {
        let op = op.trim();
        let n = if op.contains('[') {
            memory(op, level)
        } else if REGISTERS.contains(&op) || op.starts_with("ST(") {
            op.to_string()
        } else if let Some(v) = number(op) {
            if branch {
                match index.get(&v) {
                    Some(i) => format!("L{i}"),
                    None if mnemonic == "CALL" => "F".into(),
                    None => "X".into(),
                }
            } else {
                value(v, level)
            }
        } else {
            op.to_string()
        };
        parts.push(n);
    }
    out.push(' ');
    out.push_str(&parts.join(","));
    out
}

/// The function's instructions as `(address, text)`.
fn listing(text: &str) -> Vec<(u32, String)> {
    text.lines()
        .filter_map(|line| {
            let (addr, ins) = line.trim_end().split_once(": ")?;
            Some((u32::from_str_radix(addr, 16).ok()?, ins.trim().to_string()))
        })
        .collect()
}

/// The normalised shape of a listing, one instruction per line.
fn normalise(list: &[(u32, String)], level: Level) -> Vec<String> {
    let index: HashMap<u32, usize> = list.iter().enumerate().map(|(i, (a, _))| (*a, i)).collect();
    list.iter().map(|(_, ins)| instruction(ins, level, &index)).collect()
}

/// FNV-1a, 64 bits: a stable family id across runs and machines.
fn fnv(lines: &[String]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for l in lines {
        for b in l.bytes().chain(std::iter::once(b'\n')) {
            h ^= b as u64;
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    }
    h
}

// ---------------------------------------------------------------- group

struct Shaped {
    f: Function,
    insns: usize,
    strict: u64,
    loose: u64,
}

fn shapes(a: &Args) -> Result<Vec<Shaped>, String> {
    let functions = engine_map(need(&a.map, "--map")?)?;
    let cache = need(&a.cache, "--cache")?;
    let mut out = Vec::new();
    let mut missing = 0;
    for f in functions {
        let Ok(text) = std::fs::read_to_string(cache.join(format!("{:08x}.txt", f.addr))) else {
            missing += 1;
            continue;
        };
        let list = listing(&text);
        if list.is_empty() {
            missing += 1;
            continue;
        }
        out.push(Shaped {
            insns: list.len(),
            strict: fnv(&normalise(&list, Level::Strict)),
            loose: fnv(&normalise(&list, Level::Loose)),
            f,
        });
    }
    if missing > 0 {
        eprintln!("families: {missing} functions have no cached disassembly");
    }
    Ok(out)
}

fn is_translated(s: Option<&Status>) -> bool {
    s.is_some_and(|s| s.status == "translated")
}
fn is_open(s: Option<&Status>) -> bool {
    s.is_some_and(|s| s.status == "open")
}

/// Per-scope counts of open functions and those with a translated sibling.
#[derive(Default, Clone, Copy)]
struct Payoff {
    functions: usize,
    open: usize,
    strict: usize,
    loose: usize,
    strict_big: usize,
    loose_big: usize,
    /// Open functions in a strict / loose family of two or more.
    strict_fam: usize,
    loose_fam: usize,
    /// Functions (any status) in a strict / loose family of two or more.
    in_strict: usize,
    in_loose: usize,
}

fn group(a: &Args) -> Result<(), String> {
    let shaped = shapes(a)?;
    let status = ledger(need(&a.ledger, "--ledger")?)?;
    let st = |addr: u32| status.get(&addr);

    let mut strict: HashMap<u64, Vec<usize>> = HashMap::new();
    let mut loose: HashMap<u64, Vec<usize>> = HashMap::new();
    for (i, s) in shaped.iter().enumerate() {
        strict.entry(s.strict).or_default().push(i);
        loose.entry(s.loose).or_default().push(i);
    }
    let has_translated =
        |members: &[usize]| members.iter().any(|&i| is_translated(st(shaped[i].f.addr)));
    let strict_t: HashMap<u64, bool> = strict.iter().map(|(k, v)| (*k, has_translated(v))).collect();
    let loose_t: HashMap<u64, bool> = loose.iter().map(|(k, v)| (*k, has_translated(v))).collect();

    // Rows: every strict family of two or more, and every loose family that
    // joins more than one strict family.
    struct Row<'a> {
        id: String,
        level: &'static str,
        members: &'a [usize],
        open: usize,
    }
    let mut rows = Vec::new();
    for (k, m) in &strict {
        if m.len() >= 2 {
            rows.push(Row { id: format!("s{k:016x}"), level: "strict", members: m, open: 0 });
        }
    }
    for (k, m) in &loose {
        let parts: BTreeSet<u64> = m.iter().map(|&i| shaped[i].strict).collect();
        if m.len() >= 2 && parts.len() > 1 {
            rows.push(Row { id: format!("l{k:016x}"), level: "loose", members: m, open: 0 });
        }
    }
    for r in &mut rows {
        r.open = r.members.iter().filter(|&&i| is_open(st(shaped[i].f.addr))).count();
    }
    // A family with nothing left to translate is no batch.
    rows.retain(|r| r.open > 0);
    rows.sort_by(|x, y| {
        y.open
            .cmp(&x.open)
            .then(y.members.len().cmp(&x.members.len()))
            .then(x.id.cmp(&y.id))
    });

    let mut tsv = String::new();
    tsv.push_str(
        "# Look-alike families of FalloutNV.exe 1.4.0.525 functions, by normalised instruction shape.\n\
         # Generated by research/families (group); see its README for the masking rules.\n\
         # Holds addresses, counts, engine-map names and units, ledger statuses and Rust paths only.\n\
         family\tlevel\tinsns\tmembers\topen\ttranslated\tunits\tnames\ttranslated_members\topen_members\tother_members\n",
    );
    for r in &rows {
        let mut members: Vec<usize> = r.members.to_vec();
        members.sort_by_key(|&i| shaped[i].f.addr);
        let mut units = BTreeSet::new();
        let mut names = BTreeSet::new();
        let (mut tr, mut op, mut other) = (Vec::new(), Vec::new(), Vec::new());
        for &i in &members {
            let f = &shaped[i].f;
            if !f.unit.is_empty() {
                units.insert(f.unit.as_str());
            }
            if !f.name.is_empty() {
                names.insert(f.name.as_str());
            }
            match st(f.addr) {
                Some(s) if s.status == "translated" => tr.push(format!("{:08x} {}", f.addr, s.rust)),
                Some(s) if s.status == "open" => op.push(format!("{:08x}", f.addr)),
                Some(s) => other.push(format!("{:08x}:{}", f.addr, s.status)),
                None => other.push(format!("{:08x}:unlisted", f.addr)),
            }
        }
        let names: Vec<&str> = names.into_iter().take(3).collect();
        let n_units = units.len();
        let mut units: Vec<&str> = units.into_iter().take(12).collect();
        let more = format!("+{} more", n_units.saturating_sub(12));
        if n_units > 12 {
            units.push(&more);
        }
        tsv.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            r.id,
            r.level,
            shaped[members[0]].insns,
            members.len(),
            r.open,
            tr.len(),
            units.join(", "),
            names.join(" | "),
            tr.join("; "),
            op.join(" "),
            other.join(" "),
        ));
    }
    std::fs::write(need(&a.out, "--out")?, tsv).map_err(|e| e.to_string())?;

    // Summary.
    const BIG: usize = 10;
    let mut total = Payoff::default();
    let mut by_subsystem: BTreeMap<String, Payoff> = BTreeMap::new();
    let mut by_unit: BTreeMap<String, Payoff> = BTreeMap::new();
    let mut not_init = Payoff::default();
    for s in &shaped {
        let sf = strict[&s.strict].len();
        let lf = loose[&s.loose].len();
        let open = is_open(st(s.f.addr));
        let sub = if s.f.subsystem.is_empty() { "(none)" } else { s.f.subsystem.as_str() };
        let mut scopes = vec![&mut total, by_subsystem.entry(sub.to_string()).or_default()];
        if sub != "compiler: static init/exit" {
            scopes.push(&mut not_init);
        }
        let unit_entry;
        if a.focus_units.contains(&s.f.unit) {
            unit_entry = by_unit.entry(s.f.unit.clone()).or_default();
            scopes.push(unit_entry);
        }
        for p in scopes {
            p.functions += 1;
            p.in_strict += (sf >= 2) as usize;
            p.in_loose += (lf >= 2) as usize;
            if open {
                p.open += 1;
                p.strict_fam += (sf >= 2) as usize;
                p.loose_fam += (lf >= 2) as usize;
                if strict_t[&s.strict] {
                    p.strict += 1;
                    p.strict_big += (s.insns >= BIG) as usize;
                }
                if loose_t[&s.loose] {
                    p.loose += 1;
                    p.loose_big += (s.insns >= BIG) as usize;
                }
            }
        }
    }
    let strict_n = strict.values().filter(|m| m.len() >= 2).count();
    let loose_n = loose.values().filter(|m| m.len() >= 2).count();
    let mut sum = String::new();
    sum.push_str("# Look-alike families: summary (research/families group)\n\n");
    sum.push_str(&format!(
        "functions with disassembly          {} ({} outside `compiler: static init/exit`)\n\
         strict families (>= 2 members)      {strict_n}, holding {} functions ({} outside static init)\n\
         loose families (>= 2 members)       {loose_n}, holding {} functions ({} outside static init)\n\
         open functions                      {}\n\
         open in a family of >= 2            strict {}, loose {}\n\
         open with a translated sibling      strict {} (of them >= {BIG} insns: {}), loose {} (>= {BIG} insns: {})\n\n",
        shaped.len(),
        not_init.functions,
        total.in_strict,
        not_init.in_strict,
        total.in_loose,
        not_init.in_loose,
        total.open,
        total.strict_fam,
        total.loose_fam,
        total.strict,
        total.strict_big,
        total.loose,
        total.loose_big,
    ));
    let line = |name: &str, p: &Payoff| {
        format!(
            "{:<38}{:>6}{:>6}{:>7}{:>7}{:>7}{:>7}{:>7}{:>7}\n",
            name,
            p.functions,
            p.open,
            p.strict_fam,
            p.strict,
            p.strict_big,
            p.loose_fam,
            p.loose,
            p.loose_big
        )
    };
    let header = format!(
        "{:<38}{:>6}{:>6}{:>7}{:>7}{:>7}{:>7}{:>7}{:>7}\n",
        "scope", "funcs", "open", "s-fam", "s-tr", "s-tr10", "l-fam", "l-tr", "l-tr10"
    );
    sum.push_str("Open functions by subsystem\n");
    sum.push_str("(s-fam/l-fam: open functions in a strict/loose family of two or more;\n");
    sum.push_str(" s-tr/l-tr: open functions whose strict/loose family has a translated member;\n");
    sum.push_str(" s-tr10/l-tr10: of those, functions of 10 or more instructions)\n\n");
    sum.push_str(&header);
    let mut subs: Vec<_> = by_subsystem.iter().collect();
    subs.sort_by(|x, y| y.1.open.cmp(&x.1.open).then(x.0.cmp(y.0)));
    for (name, p) in subs {
        sum.push_str(&line(name, p));
    }
    if !by_unit.is_empty() {
        sum.push_str("\nFocus units\n\n");
        sum.push_str(&header);
        for (name, p) in &by_unit {
            sum.push_str(&line(name, p));
        }
    }
    std::fs::write(need(&a.summary, "--summary")?, &sum).map_err(|e| e.to_string())?;
    print!("{sum}");
    Ok(())
}

// ----------------------------------------------------------------- show

fn show(a: &Args) -> Result<(), String> {
    let shaped = shapes(a)?;
    let status = ledger(need(&a.ledger, "--ledger")?)?;
    let cache = need(&a.cache, "--cache")?;
    for &addr in &a.addresses {
        let Some(me) = shaped.iter().find(|s| s.f.addr == addr) else {
            println!("{addr:08x}: not in the engine map or not cached");
            continue;
        };
        println!("{addr:08x} {} ({}, {} insns)", me.f.name, me.f.unit, me.insns);
        for (label, key, pick) in [
            ("strict", me.strict, (|s: &Shaped| s.strict) as fn(&Shaped) -> u64),
            ("loose", me.loose, |s: &Shaped| s.loose),
        ] {
            let members: Vec<&Shaped> = shaped.iter().filter(|s| pick(s) == key).collect();
            println!("  {label} {}{key:016x}: {} members", &label[..1], members.len());
            for m in members.iter().take(40) {
                let s = status.get(&m.f.addr);
                println!(
                    "    {:08x} {:<10} {} {}",
                    m.f.addr,
                    s.map_or("?", |s| s.status.as_str()),
                    m.f.name,
                    s.map_or("", |s| s.rust.as_str())
                );
            }
        }
        if a.shape {
            let text = std::fs::read_to_string(cache.join(format!("{addr:08x}.txt")))
                .map_err(|e| e.to_string())?;
            let list = listing(&text);
            for (s, l) in normalise(&list, Level::Strict).iter().zip(normalise(&list, Level::Loose)) {
                println!("    {s:<50} {l}");
            }
        }
        let _ = me.f.size;
    }
    Ok(())
}

fn main() {
    let result = args().and_then(|a| match a.command.as_str() {
        "fetch" => fetch(&a),
        "group" => group(&a),
        "show" => show(&a),
        c => Err(format!("unknown command {c}")),
    });
    if let Err(e) = result {
        eprintln!("families: {e}\n{USAGE}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape(text: &str, level: Level) -> Vec<String> {
        normalise(&listing(text), level)
    }

    #[test]
    fn addresses_calls_and_constants_are_masked() {
        let t = "00401000: PUSH 0xef8ae8 \n\
                 00401005: MOV EAX,[0x011c16bc] \n\
                 0040100a: MOV dword ptr [EAX],0x3f800000 \n\
                 00401010: CALL 0x00483370 \n\
                 00401015: JZ 0x00401000 \n\
                 00401017: JMP 0x00402000 \n\
                 0040101c: MOV ECX,dword ptr [EBP + -0x10] \n\
                 0040101f: PUSH -0x1 \n\
                 00401021: RET 0x4 \n";
        assert_eq!(
            shape(t, Level::Strict),
            [
                "PUSH A",
                "MOV EAX,[A]",
                "MOV dword ptr[EAX],K",
                "CALL F",
                "JZ L0",
                "JMP X",
                "MOV ECX,dword ptr[EBP+-10]",
                "PUSH -1",
                "RET 4"
            ]
        );
        assert_eq!(shape(t, Level::Loose)[6], "MOV ECX,dword ptr[EBP+I]");
        assert_eq!(shape(t, Level::Loose)[8], "RET I");
    }

    #[test]
    fn element_sizes_differ_strictly_but_not_loosely() {
        let a = "00401000: MOV EAX,dword ptr [ECX + EDX*0x4 + 0x8] \n00401004: PUSH 0xc \n";
        let b = "00402000: MOV EAX,dword ptr [ECX + EDX*0x8 + 0x10] \n00402004: PUSH 0x18 \n";
        assert_ne!(shape(a, Level::Strict), shape(b, Level::Strict));
        assert_eq!(shape(a, Level::Loose), shape(b, Level::Loose));
    }
}
