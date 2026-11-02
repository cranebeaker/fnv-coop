//! initgen: writes the static-initializer table of the engine crate
//! (`crates/engine/data/static_initializers.tsv`) from the disassembly of
//! FalloutNV.exe 1.4.0.525's `__cinit` entries.
//!
//! ```text
//! cargo run --release -- --exe <FalloutNV.exe> --cache <dir> \
//!     [--ghidra http://127.0.0.1:8090] [--out <tsv>] [--report <file>]
//! ```
//!
//! The entries are read from the executable's own tables (`.rdata`,
//! `_initterm_e` `010113c4..010113e8` and `_initterm` `00fdf4a4..010113c0`,
//! as `__cinit` `00eccd14` passes them). Each entry's disassembly comes from a
//! read-only Ghidra server (`disassemble_function`) and is cached in `--cache`
//! (private: disassembly is never committed). An entry becomes a row only when
//! its whole body matches one shape exactly, instruction by instruction;
//! anything else is refused with the reason and left for hand translation.
//! See README.md for the shapes and the row format.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

/// `_initterm_e` and `_initterm` ranges of `__cinit` (`00eccd40..00eccd6b`).
const C_TABLE: (u32, u32) = (0x0101_13c4, 0x0101_13e8);
const CPP_TABLE: (u32, u32) = (0x00fd_f4a4, 0x0101_13c0);
/// `_atexit` (LIBCMT).
const ATEXIT: u32 = 0x00ec_658f;
/// Registration callees whose answer is the incoming `ECX`.
const READS_ECX: [u32; 1] = [0x00a2_e180];
/// The math helper the constants call: `sqrt` of a float, as a float.
const SQRT_HELPER: u32 = 0x0040_19b0;

struct Args {
    exe: PathBuf,
    cache: PathBuf,
    ghidra: String,
    out: Option<PathBuf>,
    report: Option<PathBuf>,
}

fn args() -> Result<Args, String> {
    let mut a = std::env::args().skip(1);
    let (mut exe, mut cache, mut out, mut report) = (None, None, None, None);
    let mut ghidra = "http://127.0.0.1:8090".to_string();
    while let Some(k) = a.next() {
        let mut v = || a.next().ok_or(format!("{k} needs a value"));
        match k.as_str() {
            "--exe" => exe = Some(PathBuf::from(v()?)),
            "--cache" => cache = Some(PathBuf::from(v()?)),
            "--ghidra" => ghidra = v()?,
            "--out" => out = Some(PathBuf::from(v()?)),
            "--report" => report = Some(PathBuf::from(v()?)),
            _ => return Err(format!("unknown argument {k}")),
        }
    }
    Ok(Args {
        exe: exe.ok_or("--exe <FalloutNV.exe> is required")?,
        cache: cache.ok_or("--cache <dir> is required")?,
        ghidra,
        out,
        report,
    })
}

/// The word at virtual address `va` of a PE file.
fn pe_reader(file: &[u8]) -> Result<impl Fn(u32) -> Option<u32> + '_, String> {
    let rd = |o: usize| -> Option<u32> {
        file.get(o..o + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    let pe = rd(0x3c).ok_or("not a PE file")? as usize;
    let n = file
        .get(pe + 6..pe + 8)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .ok_or("truncated PE header")? as usize;
    let opt = file
        .get(pe + 20..pe + 22)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .ok_or("truncated PE header")? as usize;
    let base = rd(pe + 24 + 28).ok_or("truncated PE header")?;
    let mut sections = Vec::new();
    for i in 0..n {
        let s = pe + 24 + opt + 40 * i;
        let (va, size, ptr) = (
            rd(s + 12).ok_or("bad section")?,
            rd(s + 16).ok_or("bad section")?,
            rd(s + 20).ok_or("bad section")?,
        );
        sections.push((base + va, size, ptr));
    }
    Ok(move |va: u32| {
        sections.iter().find_map(|&(start, size, ptr)| {
            let off = va.checked_sub(start)?;
            (off + 4 <= size)
                .then(|| rd(ptr as usize + off as usize))
                .flatten()
        })
    })
}

/// The non-null entries of both tables, in `__cinit`'s order.
fn entries(exe: &Path) -> Result<Vec<u32>, String> {
    let file = std::fs::read(exe).map_err(|e| format!("{}: {e}", exe.display()))?;
    let word = pe_reader(&file)?;
    let mut out = Vec::new();
    for (start, end) in [C_TABLE, CPP_TABLE] {
        for slot in (start..end).step_by(4) {
            let f = word(slot).ok_or(format!("{slot:08x} is not in the file"))?;
            if f != 0 {
                out.push(f);
            }
        }
    }
    Ok(out)
}

/// The disassembly of the function at `addr` (cached).
fn disassembly(args: &Args, addr: u32) -> Result<String, String> {
    let path = args.cache.join(format!("{addr:08x}.txt"));
    if let Ok(text) = std::fs::read_to_string(&path) {
        if !text.is_empty() {
            return Ok(text);
        }
    }
    let host = args
        .ghidra
        .trim_start_matches("http://")
        .trim_end_matches('/');
    let mut stream = TcpStream::connect(host).map_err(|e| format!("{host}: {e}"))?;
    write!(
        stream,
        "GET /disassemble_function?address={addr:08x} HTTP/1.0\r\nHost: {host}\r\n\r\n"
    )
    .map_err(|e| e.to_string())?;
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|e| e.to_string())?;
    let body = response
        .split_once("\r\n\r\n")
        .map(|(_, b)| b.to_string())
        .unwrap_or_default();
    std::fs::create_dir_all(&args.cache).map_err(|e| e.to_string())?;
    std::fs::write(&path, &body).map_err(|e| e.to_string())?;
    Ok(body)
}

/// One instruction's text (`MNEMONIC operands`), trailing spaces removed.
fn instructions(text: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let Some((addr, ins)) = line.split_once(": ") else {
            return Err(format!("not a disassembly line: {line}"));
        };
        if addr.len() != 8 || !addr.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("not a disassembly line: {line}"));
        }
        out.push(ins.trim().to_string());
    }
    Ok(out)
}

/// The instructions of the function at `addr`: Ghidra's disassembly, or, for
/// an address Ghidra made no function of, the entry's own bytes decoded.
fn function_instructions(args: &Args, addr: u32) -> Result<Vec<String>, String> {
    let text = disassembly(args, addr)?;
    if text.starts_with('{') {
        return decode(addr, &raw_bytes(args, addr)?);
    }
    instructions(&text)
}

/// A hex number `0x...` (or a negative `-0x...`) as a 32-bit word.
fn number(s: &str) -> Option<u32> {
    if let Some(h) = s.strip_prefix("-0x") {
        return u32::from_str_radix(h, 16).ok().map(|v| v.wrapping_neg());
    }
    u32::from_str_radix(s.strip_prefix("0x")?, 16).ok()
}

/// `[0x...]` as an address.
fn memory(s: &str) -> Option<u32> {
    number(s.strip_prefix('[')?.strip_suffix(']')?)
}

/// `<size> ptr [0x...]`: the size word and the address.
fn sized_memory(s: &str) -> Option<(&str, u32)> {
    let (size, rest) = s.split_once(" ptr ")?;
    Some((size, memory(rest)?))
}

/// The body between `PUSH EBP; MOV EBP,ESP` and `POP EBP; RET`, or, for a
/// function without a frame (the ones Ghidra made no function of, decoded
/// from their bytes), everything before its `RET`.
fn body(ins: &[String]) -> Option<&[String]> {
    let n = ins.len();
    if n >= 4
        && ins[0] == "PUSH EBP"
        && ins[1] == "MOV EBP,ESP"
        && ins[n - 2] == "POP EBP"
        && ins[n - 1] == "RET"
    {
        return Some(&ins[2..n - 2]);
    }
    (n >= 1 && ins[n - 1] == "RET" && ins[0] != "PUSH EBP").then(|| &ins[..n - 1])
}

/// A `cdecl` caller's clean-up of one pushed word: `ADD ESP,0x4`, or
/// `POP ECX` (the word lands in `ECX`, which no shape reads).
fn pops_one(s: &str) -> bool {
    s == "ADD ESP,0x4" || s == "POP ECX"
}

/// `CALL f; MOV [g],EAX`: a registration call whose answer is kept.
fn call_store(b: &[String]) -> Option<String> {
    let [call, store] = b else { return None };
    let f = number(call.strip_prefix("CALL ")?)?;
    // `00a2e180` returns `ECX` (left by the previous initializer: `_initterm`
    // doesn't set it), so its answer isn't a value of its own.
    if READS_ECX.contains(&f) {
        return None;
    }
    let g = memory(store.strip_prefix("MOV ")?.strip_suffix(",EAX")?)?;
    Some(format!("reg\t{f:08x} {g:08x}"))
}

/// An x87 load that feeds a pushed argument: `FLD float/double ptr [a]`,
/// `FLD1` or `FLDZ`, as `s.<a>`, `d.<a>`, `1` or `0`.
fn x87_source(s: &str) -> Option<String> {
    match s {
        "FLD1" => Some("1".into()),
        "FLDZ" => Some("0".into()),
        _ => {
            let (size, a) = sized_memory(s.strip_prefix("FLD ")?)?;
            Some(format!("{}.{a:08x}", x87_size(size).ok()?))
        }
    }
}

/// A constructor run on a global, its destructor perhaps registered:
/// pushed arguments (`PUSH imm` a word; `PUSH ECX; <x87 load>; FSTP float
/// ptr [ESP]` a float; `SUB ESP,0x8; <x87 load>; FSTP double ptr [ESP]` a
/// double), `MOV ECX,g; CALL ctor`, then nothing or `PUSH dtor;
/// CALL _atexit; ADD ESP,0x4` (or `POP ECX`). The row lists the arguments
/// in the constructor's order (the last pushed first).
fn object(b: &[String]) -> Option<String> {
    let mut pushed = Vec::new();
    let mut i = 0;
    while !b.get(i)?.starts_with("MOV ECX,") {
        let s = b[i].as_str();
        if s == "PUSH ECX" || s == "SUB ESP,0x8" {
            let float = s == "PUSH ECX";
            let source = x87_source(b.get(i + 1)?)?;
            let store = if float {
                "FSTP float ptr [ESP]"
            } else {
                "FSTP double ptr [ESP]"
            };
            if b.get(i + 2)? != store {
                return None;
            }
            pushed.push(format!("{}:{source}", if float { 'f' } else { 'd' }));
            i += 3;
        } else {
            pushed.push(format!("w:{:08x}", number(s.strip_prefix("PUSH ")?)?));
            i += 1;
        }
    }
    let this = number(b[i].strip_prefix("MOV ECX,")?)?;
    let ctor = number(b.get(i + 1)?.strip_prefix("CALL ")?)?;
    if READS_ECX.contains(&ctor) || ctor == ATEXIT {
        return None;
    }
    let dtor = match &b[i + 2..] {
        [] => "-".to_string(),
        [push, atexit, pop] => {
            let d = number(push.strip_prefix("PUSH ")?)?;
            if number(atexit.strip_prefix("CALL ")?)? != ATEXIT || !pops_one(pop) {
                return None;
            }
            format!("{d:08x}")
        }
        _ => return None,
    };
    pushed.reverse();
    let mut row = format!("object\t{ctor:08x} {this:08x} {dtor}");
    for a in pushed {
        row.push(' ');
        row.push_str(&a);
    }
    Some(row)
}

/// Straight-line constants: x87 loads, arithmetic and stores (the stack
/// starts and ends empty), the sqrt helper
/// (`PUSH ECX; FLD float ptr [a]; FSTP float ptr [ESP]; CALL 004019b0;
/// ADD ESP,0x4`, its answer pushed), and plain moves into globals.
fn constants(b: &[String]) -> Result<String, String> {
    let mut ops = Vec::new();
    let mut depth = 0i32;
    // Whether `EAX` holds a call's answer (`CALL f` with no arguments).
    let mut eax = false;
    let mut i = 0;
    while i < b.len() {
        let s = b[i].as_str();
        // The sqrt helper.
        if s == "PUSH ECX" {
            let w = b
                .get(i..i + 5)
                .ok_or("PUSH ECX outside the helper pattern")?;
            let a = w[1]
                .strip_prefix("FLD ")
                .and_then(sized_memory)
                .filter(|(size, _)| *size == "float")
                .map(|(_, a)| a)
                .ok_or("PUSH ECX outside the helper pattern")?;
            if w[2] != "FSTP float ptr [ESP]"
                || w[3].strip_prefix("CALL ").and_then(number) != Some(SQRT_HELPER)
                || !pops_one(&w[4])
            {
                return Err(format!("PUSH ECX outside the helper pattern at {s}"));
            }
            ops.push(format!("sqrt {a:08x}"));
            depth += 1;
            i += 5;
            continue;
        }
        let (mn, operand) = s.split_once(' ').unwrap_or((s, ""));
        let op = match (mn, operand) {
            ("FLD1", "") => {
                depth += 1;
                "fld1".to_string()
            }
            ("FLDZ", "") => {
                depth += 1;
                "fldz".to_string()
            }
            ("FCHS", "") if depth >= 1 => "fchs".to_string(),
            ("CALL", f) if depth == 0 => {
                let f = number(f).ok_or(format!("CALL {f}"))?;
                if f == ATEXIT || f == SQRT_HELPER || READS_ECX.contains(&f) {
                    return Err(format!("CALL {f:08x} with arguments"));
                }
                eax = true;
                format!("call {f:08x}")
            }
            ("FLD", m) => {
                let (size, a) = sized_memory(m).ok_or(format!("FLD {m}"))?;
                depth += 1;
                format!("fld.{} {a:08x}", x87_size(size)?)
            }
            ("FMUL" | "FDIV" | "FDIVR" | "FADD" | "FSUB" | "FSUBR", m) if depth >= 1 => {
                let (size, a) = sized_memory(m).ok_or(format!("{mn} {m}"))?;
                format!("{}.{} {a:08x}", mn.to_lowercase(), x87_size(size)?)
            }
            ("FMULP" | "FDIVP" | "FDIVRP" | "FADDP" | "FSUBP" | "FSUBRP", "") if depth >= 2 => {
                depth -= 1;
                mn.to_lowercase()
            }
            ("FSTP", m) if depth >= 1 => {
                let (size, a) = sized_memory(m).ok_or(format!("FSTP {m}"))?;
                depth -= 1;
                format!("fstp.{} {a:08x}", x87_size(size)?)
            }
            ("MOV", m) => {
                let (dst, src) = m.split_once(',').ok_or(format!("MOV {m}"))?;
                if src == "EAX" && eax && memory(dst).is_some() {
                    format!("st.eax {:08x}", memory(dst).unwrap_or(0))
                } else if let (Some((size, g)), Some(v)) = (sized_memory(dst), number(src)) {
                    format!("st.{} {g:08x} {v:08x}", size_tag(size)?)
                } else if let Some(reg) = ["EAX", "ECX", "EDX"].iter().find(|r| dst == **r) {
                    // MOV reg,[a] followed by MOV [g],reg.
                    let a = memory(src)
                        .or_else(|| {
                            sized_memory(src)
                                .filter(|(s, _)| *s == "dword")
                                .map(|(_, a)| a)
                        })
                        .ok_or(format!("MOV {m}"))?;
                    let next = b.get(i + 1).ok_or("load without a store")?;
                    let store = next.strip_prefix("MOV ").ok_or("load without a store")?;
                    let (gd, gs) = store.split_once(',').ok_or("load without a store")?;
                    if gs != *reg {
                        return Err(format!("load into {reg} not stored next"));
                    }
                    let g = memory(gd)
                        .or_else(|| {
                            sized_memory(gd)
                                .filter(|(s, _)| *s == "dword")
                                .map(|(_, g)| g)
                        })
                        .ok_or("load without a store")?;
                    i += 1;
                    if *reg == "EAX" {
                        eax = false;
                    }
                    format!("cp.d {g:08x} {a:08x}")
                } else {
                    return Err(format!("MOV {m}"));
                }
            }
            _ => return Err(format!("instruction {s} (x87 depth {depth})")),
        };
        ops.push(op);
        i += 1;
    }
    if depth != 0 {
        return Err(format!("x87 stack left at depth {depth}"));
    }
    if ops.is_empty() {
        return Err("empty body".into());
    }
    Ok(format!("const\t{}", ops.join(";")))
}

/// The operand size of an x87 memory operand: a float or a double only.
fn x87_size(size: &str) -> Result<&'static str, String> {
    match size {
        "float" => Ok("s"),
        "double" => Ok("d"),
        _ => Err(format!("x87 operand size {size}")),
    }
}

/// The size of a move's destination: a dword or a byte only.
fn size_tag(size: &str) -> Result<&'static str, String> {
    match size {
        "dword" => Ok("w"),
        "byte" => Ok("b"),
        _ => Err(format!("operand size {size}")),
    }
}

/// The row of one entry, or why it has none.
fn row(ins: &[String]) -> Result<String, String> {
    let b = body(ins).ok_or("not a PUSH EBP/MOV EBP,ESP ... POP EBP/RET body")?;
    if let Some(r) = call_store(b) {
        return Ok(r);
    }
    if let Some(r) = object(b) {
        return Ok(r);
    }
    constants(b)
}

fn main() {
    if let Some(list) = std::env::args().skip_while(|a| a != "--rust").nth(1) {
        rust_main(&list);
        return;
    }
    if std::env::args().any(|a| a == "--atexit") {
        atexit_main();
        return;
    }
    let args = match args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("initgen: {e}");
            std::process::exit(2);
        }
    };
    let list = match entries(&args.exe) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("initgen: {e}");
            std::process::exit(1);
        }
    };
    let mut table = String::new();
    let _ = writeln!(
        table,
        "# Generated by research/initgen from FalloutNV.exe 1.4.0.525; do not edit.\n\
         # One row per __cinit entry that matches a shape: entry, shape, parameters\n\
         # (see research/initgen/README.md). Entries without a row are hand\n\
         # translations or open."
    );
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut refused = String::new();
    for (n, &addr) in list.iter().enumerate() {
        let result = function_instructions(&args, addr).and_then(|ins| row(&ins));
        match result {
            Ok(r) => {
                let shape = r.split('\t').next().unwrap_or("");
                *counts
                    .entry(match shape {
                        "reg" => "reg",
                        "object" => "object",
                        _ if r.contains("sqrt ") => "const (sqrt helper)",
                        _ => "const",
                    })
                    .or_default() += 1;
                let _ = writeln!(table, "{addr:08x}\t{r}");
            }
            Err(why) => {
                *counts.entry("refused").or_default() += 1;
                let _ = writeln!(refused, "{addr:08x}\t{why}");
            }
        }
        if (n + 1) % 1000 == 0 {
            eprintln!("initgen: {} of {}", n + 1, list.len());
        }
    }
    eprintln!("initgen: {} entries", list.len());
    for (k, v) in &counts {
        eprintln!("  {k}: {v}");
    }
    match &args.out {
        Some(p) => std::fs::write(p, &table).unwrap_or_else(|e| panic!("{}: {e}", p.display())),
        None => print!("{table}"),
    }
    if let Some(p) = &args.report {
        std::fs::write(p, &refused).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
    }
}

// ---- Entries Ghidra made no function of ----------------------------------------

/// Up to 256 bytes of the program at `addr` (Ghidra `read_memory`, the
/// unpacked image; cached as hex in `<cache>/raw`).
fn raw_bytes(args: &Args, addr: u32) -> Result<Vec<u8>, String> {
    let dir = args.cache.join("raw");
    let path = dir.join(format!("{addr:08x}.hex"));
    let hex = match std::fs::read_to_string(&path) {
        Ok(h) if !h.is_empty() => h,
        _ => {
            let host = args
                .ghidra
                .trim_start_matches("http://")
                .trim_end_matches('/');
            let mut stream = TcpStream::connect(host).map_err(|e| format!("{host}: {e}"))?;
            write!(
                stream,
                "GET /read_memory?address={addr:08x}&length=256 HTTP/1.0\r\nHost: {host}\r\n\r\n"
            )
            .map_err(|e| e.to_string())?;
            let mut response = String::new();
            stream
                .read_to_string(&mut response)
                .map_err(|e| e.to_string())?;
            let hex = response
                .split("\"hex\":\"")
                .nth(1)
                .and_then(|r| r.split('"').next())
                .ok_or(format!("{addr:08x}: no bytes from read_memory"))?
                .to_string();
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            std::fs::write(&path, &hex).map_err(|e| e.to_string())?;
            hex
        }
    };
    (0..hex.len() / 2)
        .map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

/// Decodes straight-line code at `addr` up to its first `RET` into the
/// instruction text Ghidra prints, for the few encodings the shapes use.
/// Any other byte stops it with an error (the entry is refused).
fn decode(addr: u32, bytes: &[u8]) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut i = 0usize;
    let at = |i: usize| addr.wrapping_add(i as u32);
    let d32 = |i: usize| -> Result<u32, String> {
        bytes
            .get(i..i + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or_else(|| format!("{:08x}: truncated", at(i)))
    };
    let byte = |i: usize| -> Result<u8, String> {
        bytes
            .get(i)
            .copied()
            .ok_or_else(|| format!("{:08x}: truncated", at(i)))
    };
    loop {
        let b0 = byte(i)?;
        let (text, len) = match b0 {
            0xc3 => {
                out.push("RET".to_string());
                return Ok(out);
            }
            0x55 => ("PUSH EBP".to_string(), 1),
            0x5d => ("POP EBP".to_string(), 1),
            0x51 => ("PUSH ECX".to_string(), 1),
            0x59 => ("POP ECX".to_string(), 1),
            0x8b if byte(i + 1)? == 0xec => ("MOV EBP,ESP".to_string(), 2),
            0x68 => (format!("PUSH 0x{:x}", d32(i + 1)?), 5),
            0x6a => {
                let v = byte(i + 1)? as i8;
                if v < 0 {
                    (format!("PUSH -0x{:x}", -(v as i32)), 2)
                } else {
                    (format!("PUSH 0x{v:x}"), 2)
                }
            }
            0xe8 => {
                let target = at(i + 5).wrapping_add(d32(i + 1)?);
                (format!("CALL 0x{target:08x}"), 5)
            }
            // A tail jump ends the function as a `RET` does.
            0xe9 => {
                let target = at(i + 5).wrapping_add(d32(i + 1)?);
                out.push(format!("JMP 0x{target:08x}"));
                return Ok(out);
            }
            0xa3 => (format!("MOV [0x{:08x}],EAX", d32(i + 1)?), 5),
            0xa1 => (format!("MOV EAX,[0x{:08x}]", d32(i + 1)?), 5),
            0xb9 => (format!("MOV ECX,0x{:x}", d32(i + 1)?), 5),
            0xc7 if byte(i + 1)? == 0x05 => (
                format!("MOV dword ptr [0x{:08x}],0x{:x}", d32(i + 2)?, d32(i + 6)?),
                10,
            ),
            0xc6 if byte(i + 1)? == 0x05 => (
                format!("MOV byte ptr [0x{:08x}],0x{:x}", d32(i + 2)?, byte(i + 6)?),
                7,
            ),
            0x8b | 0x89 => {
                let reg = match byte(i + 1)? {
                    0x0d => "ECX",
                    0x15 => "EDX",
                    m => return Err(format!("{:08x}: MOV modrm {m:02x}", at(i))),
                };
                let m = d32(i + 2)?;
                if b0 == 0x8b {
                    (format!("MOV {reg},dword ptr [0x{m:08x}]"), 6)
                } else {
                    (format!("MOV dword ptr [0x{m:08x}],{reg}"), 6)
                }
            }
            0x83 if byte(i + 1)? == 0xc4 => (format!("ADD ESP,0x{:x}", byte(i + 2)?), 3),
            0x83 if byte(i + 1)? == 0xec => (format!("SUB ESP,0x{:x}", byte(i + 2)?), 3),
            0xd9 | 0xdd | 0xd8 | 0xdc | 0xde => {
                let m = byte(i + 1)?;
                let size = if b0 == 0xdd || b0 == 0xdc {
                    "double"
                } else {
                    "float"
                };
                match (b0, m) {
                    (0xd9, 0xe8) => ("FLD1".to_string(), 2),
                    (0xd9, 0xee) => ("FLDZ".to_string(), 2),
                    (0xd9, 0xe0) => ("FCHS".to_string(), 2),
                    (0xd9, 0x1c) if byte(i + 2)? == 0x24 => ("FSTP float ptr [ESP]".to_string(), 3),
                    (0xdd, 0x1c) if byte(i + 2)? == 0x24 => {
                        ("FSTP double ptr [ESP]".to_string(), 3)
                    }
                    (0xd9 | 0xdd, 0x05) => (format!("FLD {size} ptr [0x{:08x}]", d32(i + 2)?), 6),
                    (0xd9 | 0xdd, 0x1d) => (format!("FSTP {size} ptr [0x{:08x}]", d32(i + 2)?), 6),
                    (0xd8 | 0xdc, m) if m & 0xc7 == 0x05 => {
                        let op = ["FADD", "FMUL", "", "", "FSUB", "FSUBR", "FDIV", "FDIVR"]
                            [(m >> 3) as usize & 7];
                        if op.is_empty() {
                            return Err(format!("{:08x}: x87 compare", at(i)));
                        }
                        (format!("{op} {size} ptr [0x{:08x}]", d32(i + 2)?), 6)
                    }
                    (0xde, 0xc1) => ("FADDP".to_string(), 2),
                    (0xde, 0xc9) => ("FMULP".to_string(), 2),
                    (0xde, 0xe1) => ("FSUBRP".to_string(), 2),
                    (0xde, 0xe9) => ("FSUBP".to_string(), 2),
                    (0xde, 0xf1) => ("FDIVRP".to_string(), 2),
                    (0xde, 0xf9) => ("FDIVP".to_string(), 2),
                    _ => return Err(format!("{:08x}: x87 {b0:02x} {m:02x}", at(i))),
                }
            }
            _ => return Err(format!("{:08x}: byte {b0:02x}", at(i))),
        };
        out.push(text);
        i += len;
        if i >= bytes.len() {
            return Err(format!("{addr:08x}: no RET in {} bytes", bytes.len()));
        }
    }
}

// ---- The atexit destructors ----------------------------------------------------

/// The engine map's unit of the compiler-generated `atexit` destructors:
/// the functions the static initializers hand to `_atexit` (`00ec658f`).
const ATEXIT_UNIT: &str = "compiler: atexit destructors";
/// Size in bytes of each shape's code (the engine map's size is checked
/// against it, so no byte outside the matched instructions exists).
const CALL_SIZE: u32 = 15;
const TAIL_SIZE: u32 = 10;
/// `.text` of FalloutNV.exe 1.4.0.525; a destructor lies inside it.
const TEXT: std::ops::Range<u32> = 0x0040_1000..0x00fd_e600;

/// `(address, size)` of every function of the engine map's atexit unit.
fn atexit_list(map: &str) -> Result<Vec<(u32, u32)>, String> {
    let mut cols: Vec<&str> = Vec::new();
    let mut out = Vec::new();
    for line in map.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split('\t').collect();
        if cols.is_empty() {
            cols = f;
            continue;
        }
        let col = |k: &str| {
            cols.iter()
                .position(|c| *c == k)
                .and_then(|i| f.get(i))
                .copied()
                .ok_or(format!("engine map: no column {k}"))
        };
        if col("unit")? != ATEXIT_UNIT {
            continue;
        }
        let addr = u32::from_str_radix(col("address")?, 16).map_err(|e| format!("{line}: {e}"))?;
        let size = col("size")?.parse().map_err(|e| format!("{line}: {e}"))?;
        out.push((addr, size));
    }
    if out.is_empty() {
        return Err(format!("engine map: no function of unit {ATEXIT_UNIT}"));
    }
    Ok(out)
}

/// A non-negative `0x...` address operand.
fn address_operand(s: &str) -> Option<u32> {
    u32::from_str_radix(s.strip_prefix("0x")?, 16).ok()
}

/// The instructions with every number replaced by `#`: the shape of a body
/// the matchers refuse, to count the refused ones by shape.
fn signature(ins: &[String]) -> String {
    let mut out = String::new();
    for (n, s) in ins.iter().enumerate() {
        if n > 0 {
            out.push_str("; ");
        }
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '0' && chars.peek() == Some(&'x') {
                chars.next();
                while chars.peek().is_some_and(|d| d.is_ascii_hexdigit()) {
                    chars.next();
                }
                out.push('#');
            } else {
                out.push(c);
            }
        }
    }
    out
}

/// What a destructor function does, when it is one of the exact shapes:
/// `(shape, global, destructor)`.
///
/// - `call`: `PUSH EBP; MOV EBP,ESP; MOV ECX,g; CALL d; POP EBP; RET`
///   (15 bytes).
/// - `tail`: `MOV ECX,g; JMP d` (10 bytes).
fn atexit_shape(ins: &[String], size: u32) -> Result<(&'static str, u32, u32), String> {
    let text: Vec<&str> = ins.iter().map(String::as_str).collect();
    let (shape, mov, target, expect) = match text.as_slice() {
        ["PUSH EBP", "MOV EBP,ESP", mov, call, "POP EBP", "RET"] => {
            ("call", mov, call.strip_prefix("CALL "), CALL_SIZE)
        }
        [mov, jmp] => ("tail", mov, jmp.strip_prefix("JMP "), TAIL_SIZE),
        _ => return Err(format!("shape {}", signature(ins))),
    };
    let global = mov.strip_prefix("MOV ECX,").and_then(address_operand);
    let dtor = target.and_then(address_operand);
    let (Some(global), Some(dtor)) = (global, dtor) else {
        return Err(format!("shape {}", signature(ins)));
    };
    if size != expect {
        return Err(format!("{shape} shape of {size} bytes, expected {expect}"));
    }
    if !TEXT.contains(&dtor) {
        return Err(format!("{shape}: destructor {dtor:#010x} is outside .text"));
    }
    Ok((shape, global, dtor))
}

/// Whether the destructor takes no stack arguments: every `RET` of its code
/// is a plain `RET` (a `__thiscall` callee with `RET n` would read words the
/// caller didn't push). A body with no `RET` but a direct tail `JMP` is
/// followed (at most four jumps).
fn takes_no_stack_arguments(args: &Args, dtor: u32) -> Result<(), String> {
    let mut at = dtor;
    for _ in 0..4 {
        let ins = function_instructions(args, at).map_err(|e| format!("callee {at:#010x}: {e}"))?;
        if let Some(r) = ins.iter().find(|s| s.starts_with("RET ")) {
            return Err(format!("callee {at:#010x} returns with {r}"));
        }
        if ins.iter().any(|s| s == "RET") {
            return Ok(());
        }
        match ins.last().and_then(|s| s.strip_prefix("JMP ")) {
            Some(t) => {
                at = address_operand(t).ok_or(format!("callee {at:#010x} ends in JMP {t}"))?;
            }
            None => return Err(format!("callee {at:#010x} has no RET")),
        }
    }
    Err(format!("callee {dtor:#010x}: more than four tail jumps"))
}

/// `--atexit --map <engine_map.tsv> --cache <dir> [--ghidra <url>]
/// [--out <tsv>] [--report <file>]`: the shut-down table.
fn atexit_main() {
    let mut a = std::env::args().skip(1);
    let (mut map, mut cache, mut out, mut report) = (None, None, None, None);
    let mut ghidra = "http://127.0.0.1:8090".to_string();
    let mut by_hand: Vec<u32> = Vec::new();
    while let Some(k) = a.next() {
        let mut v = || {
            a.next()
                .unwrap_or_else(|| fail(&format!("{k} needs a value")))
        };
        match k.as_str() {
            "--atexit" => {}
            "--map" => map = Some(PathBuf::from(v())),
            "--cache" => cache = Some(PathBuf::from(v())),
            "--ghidra" => ghidra = v(),
            "--out" => out = Some(PathBuf::from(v())),
            "--report" => report = Some(PathBuf::from(v())),
            "--by-hand" => {
                by_hand = v()
                    .split(',')
                    .map(|h| {
                        u32::from_str_radix(h.trim(), 16)
                            .unwrap_or_else(|_| fail("--by-hand takes hex addresses"))
                    })
                    .collect();
            }
            _ => fail(&format!("unknown argument {k}")),
        }
    }
    let map = map.unwrap_or_else(|| fail("--map <engine_map.tsv> is required"));
    let args = Args {
        exe: PathBuf::new(),
        cache: cache.unwrap_or_else(|| fail("--cache <dir> is required")),
        ghidra,
        out,
        report,
    };
    let text =
        std::fs::read_to_string(&map).unwrap_or_else(|e| fail(&format!("{}: {e}", map.display())));
    let list = atexit_list(&text).unwrap_or_else(|e| fail(&e));
    let mut table = String::from(
        "# Generated by research/initgen --atexit from FalloutNV.exe 1.4.0.525; do not edit.\n\
         # One row per atexit destructor function that matches a shape: entry, shape,\n\
         # parameters `global destructor` (see research/initgen/README.md). Functions\n\
         # without a row are hand translations or open.\n",
    );
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut shapes: BTreeMap<String, usize> = BTreeMap::new();
    let mut refused = String::new();
    for (n, &(addr, size)) in list.iter().enumerate() {
        // A function translated by hand is the translation; no row duplicates it.
        if by_hand.contains(&addr) {
            *counts.entry("by hand, no row".into()).or_default() += 1;
            let _ = writeln!(
                refused,
                "{addr:08x}\t{size}\ttranslated by hand (--by-hand)"
            );
            continue;
        }
        let result = function_instructions(&args, addr).and_then(|ins| {
            let (shape, global, dtor) = atexit_shape(&ins, size)?;
            takes_no_stack_arguments(&args, dtor)?;
            Ok(format!("{shape}\t{global:08x} {dtor:08x}"))
        });
        match result {
            Ok(r) => {
                let shape = r.split('\t').next().unwrap_or("").to_string();
                *counts.entry(shape).or_default() += 1;
                let _ = writeln!(table, "{addr:08x}\t{r}");
            }
            Err(why) => {
                *shapes
                    .entry(signature(std::slice::from_ref(&why)))
                    .or_default() += 1;
                *counts.entry("refused".into()).or_default() += 1;
                let _ = writeln!(refused, "{addr:08x}\t{size}\t{why}");
            }
        }
        if (n + 1) % 1000 == 0 {
            eprintln!("initgen: {} of {}", n + 1, list.len());
        }
    }
    eprintln!("initgen: {} atexit destructors", list.len());
    for (k, v) in &counts {
        eprintln!("  {k}: {v}");
    }
    let mut top: Vec<_> = shapes.iter().collect();
    top.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (k, v) in top.iter().take(15) {
        eprintln!("  refused {v:5}  {k}");
    }
    match &args.out {
        Some(p) => std::fs::write(p, &table).unwrap_or_else(|e| panic!("{}: {e}", p.display())),
        None => print!("{table}"),
    }
    if let Some(p) = &args.report {
        std::fs::write(p, &refused).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
    }
}

fn fail(why: &str) -> ! {
    eprintln!("initgen: {why}");
    std::process::exit(2);
}

// ---- Rust for the registration callees -----------------------------------------

/// One store of a registration function.
enum Store {
    Word(u32, u32),
    Byte(u32, u8),
}

/// Rust for a one-shot registration function (the callees of the `reg`
/// rows): `[XOR EAX,EAX; CMP byte ptr [g],AL | CMP byte ptr [g],0x0]; JZ;
/// [XOR EAX,EAX;] RET;` then stores (`MOV [a],EAX` with `EAX` 0,
/// `MOV dword/byte ptr [a],imm`), at most one `PUSH b; PUSH a; CALL h;
/// ADD ESP,0x8`, and `MOV EAX,0x1; RET`. `None` for anything else.
pub fn registration_rust(addr: u32, ins: &[String]) -> Option<String> {
    let (guard, rest) = match ins {
        [x, cmp, jz, ret, rest @ ..] if x == "XOR EAX,EAX" && ret == "RET" => {
            let g = memory(cmp.strip_prefix("CMP byte ptr ")?.strip_suffix(",AL")?)?;
            jz.strip_prefix("JZ ")?;
            (g, rest)
        }
        [cmp, jz, x, ret, rest @ ..] if x == "XOR EAX,EAX" && ret == "RET" => {
            let g = memory(cmp.strip_prefix("CMP byte ptr ")?.strip_suffix(",0x0")?)?;
            jz.strip_prefix("JZ ")?;
            (g, rest)
        }
        _ => return None,
    };
    let [body @ .., one, ret] = rest else {
        return None;
    };
    if one != "MOV EAX,0x1" || ret != "RET" {
        return None;
    }
    let mut lines = Vec::new();
    let mut pushes: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < body.len() {
        let s = body[i].as_str();
        if let Some(v) = s.strip_prefix("PUSH ").and_then(number) {
            pushes.push(v);
        } else if let Some(h) = s.strip_prefix("CALL ").and_then(number) {
            if pushes.len() != 2 || body.get(i + 1).map(String::as_str) != Some("ADD ESP,0x8") {
                return None;
            }
            // cdecl: the last push is the first argument.
            lines.push(format!(
                "    e.call(0x{:04x}_{:04x}, &args![0x{:x}u32, 0x{:x}u32]);",
                h >> 16,
                h & 0xffff,
                pushes[1],
                pushes[0]
            ));
            pushes.clear();
            i += 1;
        } else {
            let m = s.strip_prefix("MOV ")?;
            let (dst, src) = m.split_once(',')?;
            let store = if src == "EAX" {
                // Only before EAX is set again: EAX is still 0 here.
                Store::Word(memory(dst)?, 0)
            } else {
                let (size, a) = sized_memory(dst)?;
                let v = number(src)?;
                match size {
                    "dword" => Store::Word(a, v),
                    "byte" => Store::Byte(a, v as u8),
                    _ => return None,
                }
            };
            lines.push(match store {
                Store::Word(a, v) => format!(
                    "    e.mem.set_u32(0x{:04x}_{:04x}, 0x{v:x});",
                    a >> 16,
                    a & 0xffff
                ),
                Store::Byte(a, v) => format!(
                    "    e.mem.set_u8(0x{:04x}_{:04x}, 0x{v:x});",
                    a >> 16,
                    a & 0xffff
                ),
            });
        }
        i += 1;
    }
    if !pushes.is_empty() {
        return None;
    }
    Some(format!(
        "// Translated from {addr:08x} (PC disassembly, FalloutNV.exe 1.4.0.525)\n\
         /// A one-shot registration of animation function tables (no Xbox PDB\n\
         /// name; called by the static initializers of every unit that includes\n\
         /// its header): the first call (the guard byte `{guard:08x}` clear)\n\
         /// stores the table entries and answers 1; later calls answer 0.\n\
         pub fn fn_{addr:08x}(e: &mut Engine) -> u32 {{\n    \
             if e.mem.u8(0x{:04x}_{:04x}) != 0 {{\n        return 0;\n    }}\n{}\n    1\n}}\n",
        guard >> 16,
        guard & 0xffff,
        lines.join("\n")
    ))
}

/// Rust for an indexed table store (the helper some registrations call):
/// `MOV EAX,[ESP+4]; MOV ECX,[ESP+8]; LEA EAX,[EAX+EAX*2];
/// LEA EAX,[ECX+EAX*2]; ADD EAX,EAX; ADD EAX,EAX;` then
/// `MOV dword ptr [EAX + base],imm` stores and `RET`.
pub fn indexed_rust(addr: u32, ins: &[String]) -> Option<String> {
    let head = [
        "MOV EAX,dword ptr [ESP + 0x4]",
        "MOV ECX,dword ptr [ESP + 0x8]",
        "LEA EAX,[EAX + EAX*0x2]",
        "LEA EAX,[ECX + EAX*0x2]",
        "ADD EAX,EAX",
        "ADD EAX,EAX",
    ];
    if ins.len() < head.len() + 1 || ins[..head.len()].iter().zip(head).any(|(a, b)| a != b) {
        return None;
    }
    let [stores @ .., ret] = &ins[head.len()..] else {
        return None;
    };
    if ret != "RET" {
        return None;
    }
    let mut lines = Vec::new();
    for s in stores {
        let (dst, v) = s.strip_prefix("MOV dword ptr [EAX + ")?.split_once("],")?;
        let base = number(dst)?;
        let v = number(v)?;
        lines.push(format!(
            "    e.mem.set_u32(at.wrapping_add(0x{:04x}_{:04x}), 0x{v:x});",
            base >> 16,
            base & 0xffff
        ));
    }
    Some(format!(
        "// Translated from {addr:08x} (PC disassembly, FalloutNV.exe 1.4.0.525)\n\
         /// Stores animation function-table entries for one (row, column) pair\n\
         /// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`\n\
         /// into each table.\n\
         pub fn fn_{addr:08x}(e: &mut Engine, row: u32, column: u32) {{\n    \
             let at = row\n        .wrapping_mul(3)\n        .wrapping_mul(2)\n        .wrapping_add(column)\n        .wrapping_mul(4);\n{}\n}}\n",
        lines.join("\n")
    ))
}

/// `--rust <addr,addr,...> --cache <dir> [--ghidra <url>]`: prints the Rust
/// translations of registration functions and their indexed helpers.
fn rust_main(list: &str) {
    let mut a = std::env::args().skip(1);
    let (mut cache, mut ghidra) = (None, "http://127.0.0.1:8090".to_string());
    while let Some(k) = a.next() {
        match k.as_str() {
            "--cache" => cache = a.next().map(PathBuf::from),
            "--ghidra" => ghidra = a.next().unwrap_or(ghidra),
            _ => {}
        }
    }
    let args = Args {
        exe: PathBuf::new(),
        cache: cache.expect("--cache <dir>"),
        ghidra,
        out: None,
        report: None,
    };
    for addr in list
        .split(',')
        .map(|s| u32::from_str_radix(s.trim(), 16).expect("hex address"))
    {
        let text = disassembly(&args, addr).expect("disassembly");
        let ins = instructions(&text).expect("instructions");
        match registration_rust(addr, &ins).or_else(|| indexed_rust(addr, &ins)) {
            Some(code) => println!("{code}"),
            None => eprintln!("initgen: {addr:08x} matches neither form; translate it by hand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn a_registration_call() {
        let ins = lines(&[
            "PUSH EBP",
            "MOV EBP,ESP",
            "CALL 0x00a26e20",
            "MOV [0x011c32f8],EAX",
            "POP EBP",
            "RET",
        ]);
        assert_eq!(row(&ins).unwrap(), "reg\t00a26e20 011c32f8");
    }

    #[test]
    fn a_float_setting() {
        let ins = lines(&[
            "PUSH EBP",
            "MOV EBP,ESP",
            "PUSH ECX",
            "FLD float ptr [0x0101712c]",
            "FSTP float ptr [ESP]",
            "PUSH 0x1054438",
            "MOV ECX,0x11ce7c0",
            "CALL 0x0040e0b0",
            "PUSH 0xfcee30",
            "CALL 0x00ec658f",
            "ADD ESP,0x4",
            "POP EBP",
            "RET",
        ]);
        assert_eq!(
            row(&ins).unwrap(),
            "object\t0040e0b0 011ce7c0 00fcee30 w:01054438 f:s.0101712c"
        );
    }

    #[test]
    fn a_word_setting_and_other_objects() {
        let word = lines(&[
            "PUSH EBP",
            "MOV EBP,ESP",
            "PUSH -0x1",
            "PUSH 0x1054438",
            "MOV ECX,0x11ce7c0",
            "CALL 0x0040c150",
            "PUSH 0xfcee30",
            "CALL 0x00ec658f",
            "ADD ESP,0x4",
            "POP EBP",
            "RET",
        ]);
        assert_eq!(
            row(&word).unwrap(),
            "object\t0040c150 011ce7c0 00fcee30 w:01054438 w:ffffffff"
        );
        // Something after the destructor's registration is not this shape.
        let mut wrong = word.clone();
        wrong.insert(9, "NOP".into());
        assert!(object(body(&wrong).unwrap()).is_none());
        // A double argument and no destructor.
        let double = lines(&[
            "SUB ESP,0x8",
            "FLD1",
            "FSTP double ptr [ESP]",
            "MOV ECX,0x11ce7c0",
            "CALL 0x00633000",
            "RET",
        ]);
        assert_eq!(row(&double).unwrap(), "object\t00633000 011ce7c0 - d:1");
    }

    #[test]
    fn constants_with_the_sqrt_helper_and_moves() {
        let ins = lines(&[
            "PUSH EBP",
            "MOV EBP,ESP",
            "PUSH ECX",
            "FLD float ptr [0x01011440]",
            "FSTP float ptr [ESP]",
            "CALL 0x004019b0",
            "ADD ESP,0x4",
            "FDIVR float ptr [0x01011444]",
            "FSTP float ptr [0x011c3190]",
            "MOV EAX,[0x01011000]",
            "MOV [0x011c3194],EAX",
            "MOV byte ptr [0x011c3198],0x1",
            "POP EBP",
            "RET",
        ]);
        assert_eq!(
            row(&ins).unwrap(),
            "const\tsqrt 01011440;fdivr.s 01011444;fstp.s 011c3190;cp.d 011c3194 01011000;st.b 011c3198 00000001"
        );
    }

    #[test]
    fn bytes_decode_as_ghidra_prints_them() {
        // FLD float [0x011b15c8]; FLD1; FDIVRP; FSTP float [0x01268284]; RET.
        let bytes = [
            0xd9, 0x05, 0xc8, 0x15, 0x1b, 0x01, 0xd9, 0xe8, 0xde, 0xf1, 0xd9, 0x1d, 0x84, 0x82,
            0x26, 0x01, 0xc3,
        ];
        let ins = decode(0x00fb_dae0, &bytes).unwrap();
        assert_eq!(
            ins,
            lines(&[
                "FLD float ptr [0x011b15c8]",
                "FLD1",
                "FDIVRP",
                "FSTP float ptr [0x01268284]",
                "RET"
            ])
        );
        assert_eq!(
            row(&ins).unwrap(),
            "const\tfld.s 011b15c8;fld1;fdivrp;fstp.s 01268284"
        );
        // A call is relative to the next instruction; unknown bytes stop it.
        let call = decode(0x0040_0000, &[0xe8, 0x10, 0x00, 0x00, 0x00, 0xc3]).unwrap();
        assert_eq!(call[0], "CALL 0x00400015");
        assert!(decode(0x0040_0000, &[0x90, 0xc3]).is_err());
    }

    #[test]
    fn a_framed_destructor_call() {
        let ins = lines(&[
            "PUSH EBP",
            "MOV EBP,ESP",
            "MOV ECX,0x11c3264",
            "CALL 0x00404a00",
            "POP EBP",
            "RET",
        ]);
        assert_eq!(
            atexit_shape(&ins, 15),
            Ok(("call", 0x011c_3264, 0x0040_4a00))
        );
        // The size is the shape's own: a longer encoding is another body.
        assert!(atexit_shape(&ins, 16).is_err());
    }

    #[test]
    fn a_destructor_tail_jump() {
        let ins = lines(&["MOV ECX,0x126fc95", "JMP 0x00ec3b35"]);
        assert_eq!(
            atexit_shape(&ins, 10),
            Ok(("tail", 0x0126_fc95, 0x00ec_3b35))
        );
        assert!(atexit_shape(&ins, 15).is_err());
    }

    #[test]
    fn other_destructors_are_refused_with_their_shape() {
        let call = [
            "PUSH EBP",
            "MOV EBP,ESP",
            "MOV ECX,0x11c3264",
            "CALL 0x00404a00",
            "POP EBP",
            "RET",
        ];
        // One instruction different at a time.
        for (at, other) in [
            (0, "NOP"),
            (1, "MOV EBP,ECX"),
            (2, "MOV ECX,dword ptr [0x11c3264]"),
            (2, "MOV EDX,0x11c3264"),
            (3, "CALL EAX"),
            (3, "CALL dword ptr [0x00fdf0e4]"),
            (3, "JMP 0x00404a00"),
            (4, "LEAVE"),
            (5, "RET 0x4"),
        ] {
            let mut ins = lines(&call);
            ins[at] = other.to_string();
            assert!(atexit_shape(&ins, 15).is_err(), "{at}: {other}");
        }
        // A tail jump through memory, or a negative or non-address operand.
        for jmp in [
            "JMP dword ptr [0x00fdf0e4]",
            "JMP EAX",
            "JMP -0x10",
            "JMP 0x00402000 ",
        ] {
            let ins = lines(&["MOV ECX,0x11c3264", jmp]);
            assert!(atexit_shape(&ins, 10).is_err(), "{jmp}");
        }
        // A destructor outside .text, an extra or a missing instruction.
        let outside = lines(&["MOV ECX,0x11c3264", "JMP 0x01000000"]);
        assert!(atexit_shape(&outside, 10).is_err());
        let mut longer = lines(&call);
        longer.insert(5, "NOP".to_string());
        assert!(atexit_shape(&longer, 16).is_err());
        assert!(atexit_shape(&lines(&call[..5]), 14).is_err());
        let err = atexit_shape(
            &lines(&["PUSH 0x1", "CALL dword ptr [0xfdf0e4]", "RET"]),
            12,
        )
        .unwrap_err();
        assert_eq!(err, "shape PUSH #; CALL dword ptr [#]; RET");
    }

    #[test]
    fn a_tail_jump_ends_decoded_bytes() {
        // MOV ECX,0x126fc95; JMP rel32 (target 0x00ec3b35).
        let at = 0x00fd_e581u32;
        let rel = 0x00ec_3b35u32.wrapping_sub(at + 10);
        let mut bytes = vec![0xb9, 0x95, 0xfc, 0x26, 0x01, 0xe9];
        bytes.extend(rel.to_le_bytes());
        bytes.push(0xcc);
        let ins = decode(at, &bytes).unwrap();
        assert_eq!(ins, lines(&["MOV ECX,0x126fc95", "JMP 0x00ec3b35"]));
        assert_eq!(
            atexit_shape(&ins, 10),
            Ok(("tail", 0x0126_fc95, 0x00ec_3b35))
        );
    }

    #[test]
    fn the_atexit_list_is_the_engine_maps_unit() {
        let map = "# header\n\
                   address\tsize\tname\tname_tier\txbox\tunit\tsubsystem\tplaced\n\
                   00401000\t21\t\t\t\t(unplaced)/run\t(unplaced)\t\n\
                   00fc7a70\t15\t\t\t\tcompiler: atexit destructors\tcompiler: static init/exit\tatexit\n\
                   00fde581\t10\t\t\t\tcompiler: atexit destructors\tcompiler: static init/exit\tatexit\n";
        assert_eq!(
            atexit_list(map),
            Ok(vec![(0x00fc_7a70, 15), (0x00fd_e581, 10)])
        );
        assert!(atexit_list("address\tsize\tunit\n00401000\t1\tx\n").is_err());
        assert!(atexit_list("address\tsize\n").is_err());
    }

    #[test]
    fn anything_else_is_refused() {
        // An unbalanced stack, a branch, an unknown call.
        for body_lines in [
            vec!["FLD1"],
            vec!["JNZ 0x00f00000"],
            vec!["CALL 0x00a26e20", "MOV [0x011c32f8],ECX"],
        ] {
            let mut ins = lines(&["PUSH EBP", "MOV EBP,ESP"]);
            ins.extend(lines(&body_lines));
            ins.extend(lines(&["POP EBP", "RET"]));
            assert!(row(&ins).is_err(), "{body_lines:?}");
        }
    }
}
