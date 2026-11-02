//! The nv-trace call record (a diagnostic, not exe behaviour): with `NV_TRACE_OUT=FILE` and
//! `NV_TRACE_LIST=LIST`, every call through [`crate::Engine::call`] to an address in LIST is
//! written to FILE as it is entered (its argument words) and as it returns (`EAX`, `EDX`, `ST0`),
//! with the game thread it ran on. The format is the one `research/nv-trace` records the real
//! FalloutNV.exe in, so `nv-trace diff` can compare the two runs call by call.
//!
//! The file: the 8 bytes `NVTRACE1`, a `u32` length and that many bytes of header text
//! (`key=value` lines; `fn=` lines repeat the list), then blocks of `u32` thread id, `u32`
//! length and that many bytes of records, all little-endian. Records:
//!
//! - enter: `u8 1`, `u8` word count `n`, `u16 0`, `u32` address, `u32` sequence number, then
//!   the `n` argument words as [`crate::Engine::call`] got them (`this` first for `thiscall`);
//! - leave: `u8 2`, `u8` flags (bit 0: `ST0` is valid), `u16 0`, `u32` address, `u32` the
//!   sequence number of its enter record, `u32 EAX`, `u32 EDX`, `u64` the bits of `ST0` as a
//!   double.
//! - thread: `u8 3`, `u8 0`, `u16 0`, `u32` the thread's start address (0 for the main
//!   thread), `u32` its start parameter; written once, before a thread's first other record,
//!   so the diff can pair the port's threads with the real game's by what they run.
//! - stack: `u8 4`, `u8` count `n`, `u16 0`, `u32` the sequence number of its enter record, then
//!   `n` times (`u8` word index, `u8` depth, `u16 0`, `i32` offset): the words that point into
//!   a live `Engine::with_stack` block (the exe's stack frames), the block's depth (0: the
//!   innermost) and the offset into it (the exe's offset from the stack pointer after the
//!   prologue, for frames mirrored with the disassembly's offsets).
//! - dump: `u8 5`, `u8` count `n`, `u16 0`, `u32` the sequence number of its enter record, then `n`
//!   times (`u32` address, `u16` length, `u16` 1 when read, the bytes padded to 4): the list's
//!   dumps at entry ([`DumpRule`]); a hash (`hash=`, `shader=`) is written as its 8 bytes and
//!   the 4-byte length hashed.
//! - return dump: `u8 7`, the same layout: the list's `rdump=`/`rhash=`, taken when the call
//!   returns from the words it was entered with.
//!
//! LIST has one function per line, its address (hex) first; the rest of the line (convention,
//! word types, return kind, name) is for `nv-trace` and copied into the header. `#` starts a
//! comment. Nothing is read or allocated per call while the variables are unset: the engine
//! holds no [`CallTrace`] and [`crate::Engine::call`] checks one `Option`.
//!
//! Calls one translation makes to another directly (inside one unit file) don't pass through
//! [`crate::Engine::call`] and are not recorded.

use std::collections::HashSet;
use std::io::Write;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// The 8 bytes a trace file starts with.
pub const MAGIC: &[u8; 8] = b"NVTRACE1";
/// Record kinds.
pub const ENTER: u8 = 1;
pub const LEAVE: u8 = 2;
pub const THREAD: u8 = 3;
pub const STACK: u8 = 4;
pub const DUMP: u8 = 5;
pub const RDUMP: u8 = 7;

/// Where the records go: one file for the whole process, shared by every engine.
pub struct Sink {
    file: std::io::BufWriter<std::fs::File>,
    /// Flush after every record (the default: a killed run keeps everything written).
    flush_each: bool,
    /// Threads whose thread record was written.
    seen: HashSet<u32>,
}

impl Sink {
    /// Creates `path` and writes the header with `header` as its text.
    pub fn create(path: &std::path::Path, header: &str, flush_each: bool) -> std::io::Result<Self> {
        let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
        file.write_all(MAGIC)?;
        file.write_all(&(header.len() as u32).to_le_bytes())?;
        file.write_all(header.as_bytes())?;
        file.flush()?;
        Ok(Sink {
            file,
            flush_each,
            seen: HashSet::new(),
        })
    }

    fn block(&mut self, thread: (u32, u32, u32), record: &[u8]) {
        let (tid, start, parameter) = thread;
        if self.seen.insert(tid) {
            let mut t = vec![THREAD, 0, 0, 0];
            t.extend_from_slice(&start.to_le_bytes());
            t.extend_from_slice(&parameter.to_le_bytes());
            let _ = self.file.write_all(&tid.to_le_bytes());
            let _ = self.file.write_all(&(t.len() as u32).to_le_bytes());
            let _ = self.file.write_all(&t);
        }
        let _ = self.file.write_all(&tid.to_le_bytes());
        let _ = self.file.write_all(&(record.len() as u32).to_le_bytes());
        let _ = self.file.write_all(record);
        if self.flush_each {
            let _ = self.file.flush();
        }
    }

    pub fn flush(&mut self) {
        let _ = self.file.flush();
    }
}

/// An engine's trace: the addresses it records and the shared sink.
pub struct CallTrace {
    addrs: HashSet<u32>,
    sink: Arc<Mutex<Sink>>,
    /// Per address from the list: its tokens ([`ListEntry`]).
    rules: std::collections::HashMap<u32, ListEntry>,
    /// The list's per-frame function (convention `frame`): `frames=` counts its calls.
    frame_fn: Option<u32>,
    /// Calls entered and calls recorded, by address.
    counts: std::cell::RefCell<std::collections::HashMap<u32, (u64, u64)>>,
    /// The frame each `frames=` function's recording opened at.
    opened: std::cell::RefCell<std::collections::HashMap<u32, u64>>,
}

/// One list line as the engine uses it: the address and its tokens (the grammar is
/// research/nv-trace's `dump.rs` and `list.rs`): `max=N` (record at most N calls),
/// `after=ADDR:N` (only once the N-th call of ADDR, a listed function, was entered),
/// `frames=S..E` (only during frames S to E-1, counted from the first call let through;
/// `frames=N` is `0..N`), `when=EXPR==N` (only the calls where the value is N; `!=` too), and
/// the memory taken with each recorded call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListEntry {
    pub addr: u32,
    pub max: Option<u64>,
    pub after: Option<(u32, u64)>,
    /// `dump=`, `rdump=`, `hash=`, `rhash=`, `shader=` ([`DumpRule`]).
    pub dumps: Vec<DumpRule>,
    pub frames: Option<(u64, u64)>,
    /// `when=EXPR==N` / `when=EXPR!=N`: (expression, N, equal).
    pub when: Vec<(DumpExpr, u32, bool)>,
    /// The line's convention is `frame` (the per-frame function).
    pub frame: bool,
}

/// A dump address: a call word, an absolute address or the word read at an inner address, plus
/// an offset (`wN+OFF`, `this` = `w0`, `0x011f9444`, `*(EXPR)+OFF`). The same syntax as
/// research/nv-trace's `dump.rs`, kept here because the engine crates take no research crates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DumpExpr {
    Word(usize, i64),
    Abs(u32, i64),
    Deref(Box<DumpExpr>, i64),
}

/// A dump's length: fixed, or a word of the call times a factor (`w3*16`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DumpLen {
    Fixed(u32),
    Word(usize, u32),
}

/// What a dump keeps: the bytes, their FNV-1a hash (`hash=`), or the hash and the bytes of the
/// Direct3D 9 shader bytecode at the address up to its end token (`shader=`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DumpKind {
    Bytes,
    Hash,
    Shader,
}

/// One dump token of a list line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DumpRule {
    pub expr: DumpExpr,
    pub len: DumpLen,
    pub kind: DumpKind,
    /// Taken when the call returns (`rdump=`, `rhash=`), from the words it was entered with.
    pub at_return: bool,
}

/// The most bytes a dump keeps and a hash covers.
pub const MAX_DUMP: u32 = 0xfff0;
pub const MAX_HASH: u32 = 0x400_0000;

fn dump_number(s: &str) -> Option<i64> {
    match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        Some(h) => i64::from_str_radix(h, 16).ok(),
        None => s.parse().ok(),
    }
}

fn dump_offsets(s: &str) -> Option<i64> {
    let mut total = 0i64;
    let mut rest = s;
    while !rest.is_empty() {
        let sign = match rest.as_bytes()[0] {
            b'+' => 1,
            b'-' => -1,
            _ => return None,
        };
        rest = &rest[1..];
        let end = rest.find(['+', '-']).unwrap_or(rest.len());
        total += sign * dump_number(&rest[..end])?;
        rest = &rest[end..];
    }
    Some(total)
}

impl DumpExpr {
    pub fn parse(s: &str) -> Option<DumpExpr> {
        if let Some(inner) = s.strip_prefix("*(") {
            let mut depth = 1;
            let close = inner.char_indices().find_map(|(i, c)| {
                match c {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => {}
                }
                (depth == 0).then_some(i)
            })?;
            let e = DumpExpr::parse(&inner[..close])?;
            return Some(DumpExpr::Deref(
                Box::new(e),
                dump_offsets(&inner[close + 1..])?,
            ));
        }
        let end = s.find(['+', '-']).unwrap_or(s.len());
        let off = dump_offsets(&s[end..])?;
        match &s[..end] {
            "this" => Some(DumpExpr::Word(0, off)),
            b => match b.strip_prefix('w') {
                Some(n) => Some(DumpExpr::Word(n.parse().ok()?, off)),
                None => Some(DumpExpr::Abs(u32::try_from(dump_number(b)?).ok()?, off)),
            },
        }
    }

    /// The value from the call's words; `None` when a word is missing or a pointer can't be
    /// read.
    pub fn eval(&self, words: &[u32], mem: &crate::Mem) -> Option<u32> {
        match self {
            DumpExpr::Word(i, off) => Some(words.get(*i)?.wrapping_add(*off as u32)),
            DumpExpr::Abs(a, off) => Some(a.wrapping_add(*off as u32)),
            DumpExpr::Deref(inner, off) => {
                let at = inner.eval(words, mem)?;
                Some(read_word(mem, at)?.wrapping_add(*off as u32))
            }
        }
    }
}

fn read_word(mem: &crate::Mem, at: u32) -> Option<u32> {
    let mut b = [0u8; 4];
    mem.try_read(at, &mut b).ok()?;
    Some(u32::from_le_bytes(b))
}

impl DumpLen {
    fn parse(s: &str, max: u32) -> Option<DumpLen> {
        if let Some(w) = s.strip_prefix('w') {
            let (n, k) = w.split_once('*').unwrap_or((w, "1"));
            let k = u32::try_from(dump_number(k)?).ok().filter(|&k| k > 0)?;
            return Some(DumpLen::Word(n.parse().ok()?, k));
        }
        let l = u32::try_from(dump_number(s)?)
            .ok()
            .filter(|&l| l > 0 && l <= max)?;
        Some(DumpLen::Fixed(l))
    }

    fn eval(self, words: &[u32], max: u32) -> Option<u32> {
        let l = match self {
            DumpLen::Fixed(l) => l,
            DumpLen::Word(i, k) => words.get(i)?.checked_mul(k)?,
        };
        (l > 0 && l <= max).then_some(l)
    }
}

impl DumpRule {
    /// One token (`dump=`, `rdump=`, `hash=`, `rhash=`, `shader=`); `None` for other tokens
    /// and malformed ones.
    pub fn parse(token: &str) -> Option<DumpRule> {
        let (key, v) = token.split_once('=')?;
        let (kind, at_return) = match key {
            "dump" => (DumpKind::Bytes, false),
            "rdump" => (DumpKind::Bytes, true),
            "hash" => (DumpKind::Hash, false),
            "rhash" => (DumpKind::Hash, true),
            "shader" => (DumpKind::Shader, false),
            _ => return None,
        };
        let (expr, len) = if kind == DumpKind::Shader {
            (DumpExpr::parse(v)?, DumpLen::Fixed(4))
        } else {
            let (e, len) = v.rsplit_once(':')?;
            let max = if kind == DumpKind::Hash {
                MAX_HASH
            } else {
                MAX_DUMP
            };
            (DumpExpr::parse(e)?, DumpLen::parse(len, max)?)
        };
        Some(DumpRule {
            expr,
            len,
            kind,
            at_return,
        })
    }

    /// The bytes this dump records for a call: the memory, or for a hash the 8-byte FNV-1a hash
    /// and the 4-byte length hashed, for a shader followed by the bytecode when it fits in a dump;
    /// `None` when unreadable.
    fn take(&self, at: u32, words: &[u32], mem: &crate::Mem) -> Option<Vec<u8>> {
        let read = |len: u32| {
            let mut bytes = vec![0u8; len as usize];
            mem.try_read(at, &mut bytes).ok().map(|_| bytes)
        };
        match self.kind {
            DumpKind::Bytes => read(self.len.eval(words, MAX_DUMP)?),
            DumpKind::Hash => read(self.len.eval(words, MAX_HASH)?).map(|b| hash_payload(&b)),
            DumpKind::Shader => read(shader_len(at, mem)?).map(|b| shader_payload(&b)),
        }
    }
}

/// 64-bit FNV-1a.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

fn hash_payload(bytes: &[u8]) -> Vec<u8> {
    let mut out = fnv1a(bytes).to_le_bytes().to_vec();
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out
}

/// A shader's dump: the hash payload, then the bytecode when the whole fits in a dump.
fn shader_payload(bytes: &[u8]) -> Vec<u8> {
    let mut out = hash_payload(bytes);
    if out.len() + bytes.len() <= MAX_DUMP as usize {
        out.extend_from_slice(bytes);
    }
    out
}

/// The length of the Direct3D 9 shader bytecode at `at` (from its version token to its end
/// token 0x0000FFFF included, comments skipped by their length); `None` when unreadable or
/// longer than 256 KiB.
fn shader_len(at: u32, mem: &crate::Mem) -> Option<u32> {
    let mut off = 4u32;
    while off < 0x4_0000 {
        let token = read_word(mem, at.wrapping_add(off))?;
        if token == 0x0000_ffff {
            return Some(off + 4);
        }
        if token & 0xffff == 0xfffe {
            off += 4 * ((token >> 16) & 0x7fff);
        }
        off += 4;
    }
    None
}

/// The sequence numbers, shared by every engine of the process (one numbering per file).
static SEQ: AtomicU32 = AtomicU32::new(0);

impl CallTrace {
    pub fn new(addrs: impl IntoIterator<Item = u32>, sink: Arc<Mutex<Sink>>) -> Self {
        CallTrace {
            addrs: addrs.into_iter().collect(),
            sink,
            rules: Default::default(),
            frame_fn: None,
            counts: Default::default(),
            opened: Default::default(),
        }
    }

    /// A trace of the list's entries with their tokens.
    pub fn with_rules(entries: &[ListEntry], sink: Arc<Mutex<Sink>>) -> Self {
        let mut t = CallTrace::new(entries.iter().map(|e| e.addr), sink);
        t.frame_fn = entries.iter().find(|e| e.frame).map(|e| e.addr);
        t.rules = entries
            .iter()
            .filter(|e| {
                e.max.is_some()
                    || e.after.is_some()
                    || e.frames.is_some()
                    || !e.when.is_empty()
                    || !e.dumps.is_empty()
            })
            .map(|e| (e.addr, e.clone()))
            .collect();
        t
    }

    /// For a listed `addr` being entered with `args`: counts the call and says whether it is
    /// recorded (its `after=` event has happened, it is inside its `frames=` window, it meets
    /// its `when=` conditions and its `max=` is not reached), in the real recorder's order.
    pub fn admit(&self, addr: u32, args: &[u32], mem: &crate::Mem) -> bool {
        let mut counts = self.counts.borrow_mut();
        counts.entry(addr).or_default().0 += 1;
        let Some(rule) = self.rules.get(&addr) else {
            return true;
        };
        if let Some((event, n)) = rule.after {
            if counts.get(&event).map_or(0, |c| c.0) < n {
                return false;
            }
        }
        if let Some((first, end)) = rule.frames {
            let now = self
                .frame_fn
                .and_then(|a| counts.get(&a))
                .map_or(0, |c| c.0);
            let open = *self.opened.borrow_mut().entry(addr).or_insert(now);
            let rel = now.saturating_sub(open);
            if rel < first || rel >= end {
                return false;
            }
        }
        let meets = rule.when.iter().all(|(expr, value, equal)| {
            expr.eval(args, mem)
                .is_some_and(|v| (v == *value) == *equal)
        });
        if !meets {
            return false;
        }
        let recorded = &mut counts.entry(addr).or_default().1;
        if rule.max.is_some_and(|m| *recorded >= m) {
            return false;
        }
        *recorded += 1;
        true
    }

    /// Whether calls to `addr` are recorded.
    #[inline]
    pub fn wants(&self, addr: u32) -> bool {
        self.addrs.contains(&addr)
    }

    /// Writes the enter record; returns its sequence number.
    /// `thread` is (id, start address, start parameter), see [`thread_of`].
    pub fn enter(&self, thread: (u32, u32, u32), addr: u32, args: &[u32]) -> u32 {
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let n = args.len().min(255);
        let mut r = Vec::with_capacity(12 + 4 * n);
        r.extend_from_slice(&[ENTER, n as u8, 0, 0]);
        r.extend_from_slice(&addr.to_le_bytes());
        r.extend_from_slice(&seq.to_le_bytes());
        for w in &args[..n] {
            r.extend_from_slice(&w.to_le_bytes());
        }
        self.write(thread, &r);
        seq
    }

    /// Writes the stack record of the call `seq`: (word index, depth, offset) for each word that
    /// points into a live stack block.
    pub fn stack(&self, thread: (u32, u32, u32), seq: u32, words: &[(u8, u8, i32)]) {
        let n = words.len().min(255);
        let mut r = Vec::with_capacity(8 + 8 * n);
        r.extend_from_slice(&[STACK, n as u8, 0, 0]);
        r.extend_from_slice(&seq.to_le_bytes());
        for &(index, depth, offset) in &words[..n] {
            r.extend_from_slice(&[index, depth, 0, 0]);
            r.extend_from_slice(&offset.to_le_bytes());
        }
        self.write(thread, &r);
    }

    /// Writes the dump record of the call `seq` to `addr` (the list's entry dumps for it),
    /// reading `mem`: per dump the address (0 when it can't be computed), the length, whether
    /// it was read, and the bytes padded to 4.
    pub fn dump(
        &self,
        thread: (u32, u32, u32),
        seq: u32,
        addr: u32,
        args: &[u32],
        mem: &crate::Mem,
    ) {
        self.dump_side(thread, seq, addr, args, mem, false);
    }

    /// The same for the list's return dumps (`rdump=`, `rhash=`), from the words the call was
    /// entered with: a record of kind [`RDUMP`].
    pub fn rdump(
        &self,
        thread: (u32, u32, u32),
        seq: u32,
        addr: u32,
        args: &[u32],
        mem: &crate::Mem,
    ) {
        self.dump_side(thread, seq, addr, args, mem, true);
    }

    fn dump_side(
        &self,
        thread: (u32, u32, u32),
        seq: u32,
        addr: u32,
        args: &[u32],
        mem: &crate::Mem,
        at_return: bool,
    ) {
        let Some(rule) = self.rules.get(&addr) else {
            return;
        };
        let rules: Vec<&DumpRule> = rule
            .dumps
            .iter()
            .filter(|d| d.at_return == at_return)
            .take(255)
            .collect();
        if rules.is_empty() {
            return;
        }
        let kind = if at_return { RDUMP } else { DUMP };
        let mut r = vec![kind, rules.len() as u8, 0, 0];
        r.extend_from_slice(&seq.to_le_bytes());
        for d in rules {
            let at = d.expr.eval(args, mem);
            let bytes = at.and_then(|a| d.take(a, args, mem));
            r.extend_from_slice(&at.unwrap_or(0).to_le_bytes());
            match bytes {
                Some(bytes) => {
                    r.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                    r.extend_from_slice(&1u16.to_le_bytes());
                    r.extend_from_slice(&bytes);
                    r.resize(r.len() + (4 - bytes.len() % 4) % 4, 0);
                }
                None => r.extend_from_slice(&[0, 0, 0, 0]),
            }
        }
        self.write(thread, &r);
    }

    /// Writes the leave record of the call `seq`.
    pub fn leave(&self, thread: (u32, u32, u32), addr: u32, seq: u32, ret: &crate::Ret) {
        let mut r = Vec::with_capacity(28);
        r.extend_from_slice(&[LEAVE, 1, 0, 0]);
        r.extend_from_slice(&addr.to_le_bytes());
        r.extend_from_slice(&seq.to_le_bytes());
        r.extend_from_slice(&ret.eax.to_le_bytes());
        r.extend_from_slice(&ret.edx.to_le_bytes());
        r.extend_from_slice(&ret.st0.to_bits().to_le_bytes());
        self.write(thread, &r);
    }

    fn write(&self, thread: (u32, u32, u32), record: &[u8]) {
        if let Ok(mut sink) = self.sink.lock() {
            sink.block(thread, record);
        }
    }
}

/// The running game thread as the trace names it: (id, start address, start parameter); the
/// main thread has start 0.
pub fn thread_of(e: &crate::Engine) -> (u32, u32, u32) {
    let t = &e.kernel.threads;
    match t.list.get(t.current) {
        Some(g) => (g.id, g.start, g.parameter),
        None => (crate::thread_host::current_id(e), 0, 0),
    }
}

/// The words of `args` that point into a live stack block of `frames` (innermost last): (index,
/// depth from the innermost, offset into the block).
pub fn stack_words(frames: &[(u32, u32)], args: &[u32]) -> Vec<(u8, u8, i32)> {
    let mut out = Vec::new();
    if frames.is_empty() {
        return out;
    }
    for (i, &w) in args.iter().enumerate().take(255) {
        if let Some((depth, &(start, _))) = frames
            .iter()
            .rev()
            .enumerate()
            .find(|(_, &(s, len))| w >= s && w - s < len)
        {
            out.push((i as u8, depth.min(254) as u8, w.wrapping_sub(start) as i32));
        }
    }
    out
}

/// The entries of a function list: each line's address, whether its convention is `frame`, and
/// its tokens ([`ListEntry`]; `after=ADDR` is `ADDR:1`). Malformed tokens are skipped (the
/// research tool's parser names them).
pub fn parse_entries(text: &str) -> Vec<ListEntry> {
    text.lines()
        .filter_map(|line| {
            let line = line.split('#').next()?.trim();
            let mut parts = line.split_whitespace();
            let first = parts
                .next()?
                .trim_start_matches("0x")
                .trim_start_matches("0X");
            let addr = u32::from_str_radix(first, 16).ok()?;
            let mut entry = ListEntry {
                addr,
                max: None,
                after: None,
                dumps: Vec::new(),
                frames: None,
                when: Vec::new(),
                frame: false,
            };
            for (i, p) in parts.enumerate() {
                if i == 0 {
                    entry.frame = p == "frame";
                }
                if let Some(n) = p.strip_prefix("max=") {
                    entry.max = n.parse().ok();
                } else if let Some(n) = p.strip_prefix("frames=") {
                    entry.frames = parse_frames(n);
                } else if let Some(v) = p.strip_prefix("after=") {
                    let (a, n) = v.split_once(':').unwrap_or((v, "1"));
                    let a = u32::from_str_radix(a.trim_start_matches("0x"), 16).ok();
                    entry.after = a.zip(n.parse().ok());
                } else if let Some(v) = p.strip_prefix("when=") {
                    let (e, n, equal) = match v.split_once("!=") {
                        Some((e, n)) => (e, n, false),
                        None => match v.split_once("==") {
                            Some((e, n)) => (e, n, true),
                            None => continue,
                        },
                    };
                    let n = dump_number(n).and_then(|n| u32::try_from(n).ok());
                    if let (Some(e), Some(n)) = (DumpExpr::parse(e), n) {
                        entry.when.push((e, n, equal));
                    }
                } else if let Some(d) = DumpRule::parse(p) {
                    entry.dumps.push(d);
                }
            }
            Some(entry)
        })
        .collect()
}

/// `frames=N` (frames 0 to N-1) or `frames=S..E` (S to E-1): (S, E), E above S.
pub fn parse_frames(v: &str) -> Option<(u64, u64)> {
    let (s, e) = match v.split_once("..") {
        Some((s, e)) => (s.parse().ok()?, e.parse().ok()?),
        None => (0, v.parse().ok()?),
    };
    (e > s).then_some((s, e))
}

/// The addresses of a function list (the first word of each line, hex, `0x` optional).
pub fn parse_list(text: &str) -> Vec<u32> {
    text.lines()
        .filter_map(|line| {
            let line = line.split('#').next()?.trim();
            let first = line.split_whitespace().next()?;
            let first = first.trim_start_matches("0x").trim_start_matches("0X");
            u32::from_str_radix(first, 16).ok()
        })
        .collect()
}

/// The process's traced addresses and its sink.
type EnvTrace = (Vec<ListEntry>, Arc<Mutex<Sink>>);

/// The process's trace from `NV_TRACE_OUT` and `NV_TRACE_LIST`, read once: `None` unless both
/// are set and the list could be read and the file created (a failure is printed once).
fn from_env() -> Option<&'static EnvTrace> {
    static TRACE: OnceLock<Option<EnvTrace>> = OnceLock::new();
    TRACE
        .get_or_init(|| {
            let out = std::env::var_os("NV_TRACE_OUT")?;
            let list_path = std::env::var_os("NV_TRACE_LIST")?;
            let text = match std::fs::read_to_string(&list_path) {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("NV_TRACE_LIST {}: {err}", list_path.to_string_lossy());
                    return None;
                }
            };
            let addrs = parse_entries(&text);
            let mut header = String::from("source=port\nformat=1\n");
            header.push_str(&format!("list={}\n", list_path.to_string_lossy()));
            for line in text.lines() {
                let line = line.split('#').next().unwrap_or("").trim();
                if !line.is_empty() {
                    header.push_str(&format!("fn={line}\n"));
                }
            }
            let flush_each = std::env::var("NV_TRACE_FLUSH").map_or(true, |v| v != "buffered");
            match Sink::create(std::path::Path::new(&out), &header, flush_each) {
                Ok(sink) => Some((addrs, Arc::new(Mutex::new(sink)))),
                Err(err) => {
                    eprintln!("NV_TRACE_OUT {}: {err}", out.to_string_lossy());
                    None
                }
            }
        })
        .as_ref()
}

/// The trace a new engine starts with: `None` unless the environment asks for one.
pub fn for_new_engine() -> Option<Box<CallTrace>> {
    let (entries, sink) = from_env()?;
    Some(Box::new(CallTrace::with_rules(entries, sink.clone())))
}

/// Flushes the process's trace file, if there is one (for a run that ends on purpose).
pub fn flush() {
    if let Some((_, sink)) = from_env() {
        if let Ok(mut s) = sink.lock() {
            s.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, Ret};

    /// `dump=` writes the memory at an address computed from the words, pointers followed.
    #[test]
    fn dumps_follow_the_words() {
        let entries = parse_entries(
            "00403000 thiscall pp void dump=this+4:4 dump=*(w1)+2:2 dump=*(w1+0x100):4
",
        );
        assert_eq!(entries[0].dumps.len(), 3);
        let dir = std::env::temp_dir().join(format!("nv-call-trace-dump-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("d.nvtrace");
        let sink = Arc::new(Mutex::new(Sink::create(&path, "", true).unwrap()));
        let mut e = Engine::new();
        e.register(0x0040_3000, |_, _| Ret::default());
        let obj = e.mem.alloc(16);
        e.mem.set_u32(obj + 4, 0x1122_3344);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, obj);
        e.call_trace = Some(Box::new(CallTrace::with_rules(&entries, sink)));
        e.call(0x0040_3000, &[obj, holder]);
        e.call_trace = None;
        let bytes = std::fs::read(&path).unwrap();
        let word = |i: usize| u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap());
        // Header (12), thread block (20), enter block (8 + 12 + 8), then the dump block.
        let at = 12 + 20 + 28;
        let r = &bytes[at + 8..at + 8 + word(at + 4) as usize];
        assert_eq!(&r[..4], &[DUMP, 3, 0, 0]);
        // this+4: 4 bytes read.
        assert_eq!(u32::from_le_bytes(r[8..12].try_into().unwrap()), obj + 4);
        assert_eq!(&r[12..16], &[4, 0, 1, 0]);
        assert_eq!(
            u32::from_le_bytes(r[16..20].try_into().unwrap()),
            0x1122_3344
        );
        // *(w1)+2: the object's address read through the holder, plus 2.
        assert_eq!(u32::from_le_bytes(r[20..24].try_into().unwrap()), obj + 2);
        assert_eq!(&r[24..28], &[2, 0, 1, 0]);
        // *(w1+0x100): unmapped, so no address.
        assert_eq!(&r[32..40], &[0, 0, 0, 0, 0, 0, 0, 0]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `after=` starts a function's recording at the N-th call of another listed one, and
    /// `max=` stops it after N recorded calls.
    #[test]
    fn after_and_max_gate_the_recording() {
        let list = "00403000 cdecl - void
00403010 cdecl w void max=2 after=00403000:2 X
";
        let entries = parse_entries(list);
        assert_eq!(
            entries[1],
            ListEntry {
                addr: 0x0040_3010,
                max: Some(2),
                after: Some((0x0040_3000, 2)),
                dumps: Vec::new(),
                frames: None,
                when: Vec::new(),
                frame: false,
            }
        );
        let dir = std::env::temp_dir().join(format!("nv-call-trace-gate-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let sink = Arc::new(Mutex::new(
            Sink::create(&dir.join("g.nvtrace"), "", true).unwrap(),
        ));
        let t = CallTrace::with_rules(&entries, sink);
        let mem = crate::Mem::new();
        // Before the second call of 403000: not recorded.
        assert!(t.admit(0x0040_3000, &[], &mem));
        assert!(!t.admit(0x0040_3010, &[], &mem));
        assert!(t.admit(0x0040_3000, &[], &mem));
        // Now open, for two calls.
        assert!(t.admit(0x0040_3010, &[], &mem));
        assert!(t.admit(0x0040_3010, &[], &mem));
        assert!(!t.admit(0x0040_3010, &[], &mem));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `frames=N` records during N calls of the `frame` function from the first call let
    /// through; `when=` lets only the calls whose value matches through, and those that fail
    /// are not counted by `max=`.
    #[test]
    fn frames_and_conditions_gate_the_recording() {
        let list = "00403000 frame p void OnIdle
00403010 thiscall p void frames=2 X
00403020 thiscall p void max=1 when=*(this)==0x1234 Y
00403030 thiscall p void frames=1..2 Z
";
        let entries = parse_entries(list);
        assert!(entries[0].frame && !entries[1].frame);
        assert_eq!(entries[1].frames, Some((0, 2)));
        assert_eq!(parse_frames("120..124"), Some((120, 124)));
        assert_eq!(parse_frames("5..5"), None);
        let dir = std::env::temp_dir().join(format!("nv-call-trace-when-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let sink = Arc::new(Mutex::new(
            Sink::create(&dir.join("w.nvtrace"), "", true).unwrap(),
        ));
        let t = CallTrace::with_rules(&entries, sink);
        let mut e = Engine::new();
        let mem = &mut e.mem;
        let (a, b) = (mem.alloc(4), mem.alloc(4));
        mem.set_u32(a, 0x1234);
        mem.set_u32(b, 0x5678);
        // Opened at frame 0, so frames 0 and 1 only; 403030 opens at 0 and records frame 1.
        assert!(t.admit(0x0040_3010, &[0], mem));
        assert!(!t.admit(0x0040_3030, &[0], mem));
        assert!(t.admit(0x0040_3000, &[0], mem));
        assert!(t.admit(0x0040_3030, &[0], mem));
        assert!(t.admit(0x0040_3010, &[0], mem));
        assert!(t.admit(0x0040_3000, &[0], mem));
        assert!(!t.admit(0x0040_3010, &[0], mem));
        assert!(!t.admit(0x0040_3030, &[0], mem));
        // The other vtable is skipped and not counted; then one call, then the cap.
        assert!(!t.admit(0x0040_3020, &[b], mem));
        assert!(t.admit(0x0040_3020, &[a], mem));
        assert!(!t.admit(0x0040_3020, &[a], mem));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The tokens parse as the research tool's: absolute bases, word lengths, hashes, return
    /// dumps; a shader's hash covers its bytecode to the end token.
    #[test]
    fn dump_tokens_parse_and_hash() {
        let e = &parse_entries(
            "00403000 stdcall pppp u32 dump=w2:w3*16 hash=w1:w2 rdump=w3:4 shader=w1 dump=0x011f9444:8 X",
        )[0];
        assert_eq!(e.dumps.len(), 5);
        assert_eq!(e.dumps[0].len, DumpLen::Word(3, 16));
        assert_eq!(e.dumps[1].kind, DumpKind::Hash);
        assert!(e.dumps[2].at_return);
        assert_eq!(e.dumps[3].kind, DumpKind::Shader);
        assert_eq!(e.dumps[4].expr, DumpExpr::Abs(0x011f_9444, 0));
        assert_eq!(fnv1a(b"a"), 0xaf63_dc4c_8601_ec8c);
        let mut e = Engine::new();
        let mem = &mut e.mem;
        let at = mem.alloc(20);
        for (i, w) in [0xffff_0300u32, 0x0001_fffe, 0x1234, 1, 0x0000_ffff]
            .iter()
            .enumerate()
        {
            mem.set_u32(at + 4 * i as u32, *w);
        }
        assert_eq!(shader_len(at, mem), Some(20));
    }

    #[test]
    fn the_list_gives_the_first_word_of_each_line() {
        let list =
            "# speech\n0064aa00 thiscall ppf void AddPhoneme\n\n0x0064ca60 cdecl fppb u8 # x\n";
        assert_eq!(parse_list(list), vec![0x0064_aa00, 0x0064_ca60]);
    }

    /// A word that points into a live `with_stack` block gets a stack record: its index, the
    /// block's depth (0: innermost) and the offset into the block.
    #[test]
    fn words_into_a_stack_block_get_a_stack_record() {
        let dir = std::env::temp_dir().join(format!("nv-call-trace-stack-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("s.nvtrace");
        let sink = Arc::new(Mutex::new(Sink::create(&path, "", true).unwrap()));
        let mut e = Engine::new();
        e.register(0x0040_3000, |_, _| Ret::default());
        e.call_trace = Some(Box::new(CallTrace::new([0x0040_3000], sink)));
        e.with_stack(0x40, |e, outer| {
            e.with_stack(0x20, |e, inner| {
                e.call(0x0040_3000, &[7, inner.addr() + 4, outer.addr() + 0x30]);
            });
        });
        e.call_trace = None;
        let bytes = std::fs::read(&path).unwrap();
        let word = |i: usize| u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap());
        // Header (12), thread block (8 + 12), enter block (8 + 12 + 12), then the stack block.
        let at = 12 + 20 + 32;
        assert_eq!(word(at + 4), 24);
        let r = &bytes[at + 8..at + 32];
        assert_eq!(&r[..4], &[STACK, 2, 0, 0]);
        assert_eq!(&r[8..12], &[1, 0, 0, 0]);
        assert_eq!(i32::from_le_bytes(r[12..16].try_into().unwrap()), 4);
        assert_eq!(&r[16..20], &[2, 1, 0, 0]);
        assert_eq!(i32::from_le_bytes(r[20..24].try_into().unwrap()), 0x30);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Only listed addresses are written, each as an enter and a leave record in its block.
    #[test]
    fn listed_calls_are_written_with_words_and_answers() {
        let dir = std::env::temp_dir().join(format!("nv-call-trace-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("t.nvtrace");
        let sink = Arc::new(Mutex::new(
            Sink::create(&path, "source=test\n", true).unwrap(),
        ));
        let mut e = Engine::new();
        e.register(0x0040_3000, |_, a| Ret {
            eax: a.iter().sum(),
            st0: 1.5,
            ..Ret::default()
        });
        e.register(0x0040_3010, |_, _| Ret::default());
        e.call_trace = Some(Box::new(CallTrace::new([0x0040_3000], sink)));
        e.call(0x0040_3000, &[1, 2]);
        e.call(0x0040_3010, &[7]);
        e.call_trace = None;
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[..8], MAGIC);
        let hlen = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
        assert_eq!(&bytes[12..12 + hlen], b"source=test\n");
        let mut at = 12 + hlen;
        let word = |b: &[u8], i: usize| u32::from_le_bytes(b[i..i + 4].try_into().unwrap());
        // The thread record first: the main thread, start 0.
        let tid = word(&bytes, at);
        assert_eq!(tid, crate::thread_host::current_id(&e));
        assert_eq!(word(&bytes, at + 4), 12);
        assert_eq!(&bytes[at + 8..at + 12], &[THREAD, 0, 0, 0]);
        assert_eq!((word(&bytes, at + 12), word(&bytes, at + 16)), (0, 0));
        at += 20;
        // Enter block: main thread, 12 + 8 bytes.
        assert_eq!(word(&bytes, at), tid);
        assert_eq!(word(&bytes, at + 4), 20);
        let r = &bytes[at + 8..at + 28];
        assert_eq!(&r[..4], &[ENTER, 2, 0, 0]);
        assert_eq!(word(r, 4), 0x0040_3000);
        let seq = word(r, 8);
        assert_eq!((word(r, 12), word(r, 16)), (1, 2));
        at += 28;
        assert_eq!(word(&bytes, at + 4), 28);
        let r = &bytes[at + 8..at + 36];
        assert_eq!(&r[..4], &[LEAVE, 1, 0, 0]);
        assert_eq!((word(r, 4), word(r, 8), word(r, 12)), (0x0040_3000, seq, 3));
        assert_eq!(
            u64::from_le_bytes(r[20..28].try_into().unwrap()),
            1.5f64.to_bits()
        );
        // Nothing for the unlisted address.
        assert_eq!(bytes.len(), at + 36);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
