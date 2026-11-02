//! The executable's static initializers, run as `__cinit` (`00eccd14`) runs
//! them before `WinMain`: `_initterm_e` over the C initializers
//! (`010113c4..010113e8`; a non-zero answer stops it) and `_initterm` over
//! the C++ dynamic initializers (`00fdf4a4..010113c0`), each a pointer in
//! `.rdata`, null ones skipped, in table order.
//!
//! Most entries are tiny and regular. Each one that matches a shape exactly
//! is a row of the generated table `data/static_initializers.tsv`
//! (research/initgen, from the entry's code, every instruction checked;
//! anything else is refused there), and runs through the one translation of
//! its shape below:
//!
//! - `reg f g`: `CALL f; MOV [g],EAX` ([`run_reg`]).
//! - `object ctor g dtor args..`: a constructor run on the global `g` with
//!   pushed arguments (the `Setting`s (Xbox PDB) with their names and
//!   defaults among them), then its destructor given to `_atexit` (none:
//!   `-`) ([`run_object`]).
//! - `const ops`: straight-line constants: x87 loads, arithmetic and stores,
//!   the float square-root helper `004019b0`, calls without arguments whose
//!   answer is stored, plain moves ([`run_const`]).
//!
//! Every other entry runs by its address when it is translated (by hand);
//! the rest are skipped and counted, so the run never stops in an open
//! function. A row whose callee (a constructor, the helper) isn't translated
//! is skipped the same way.

use crate::Engine;
use std::collections::HashMap;

/// The generated table (research/initgen).
const TABLE: &str = include_str!("../data/static_initializers.tsv");

/// The `_initterm_e` and `_initterm` ranges `__cinit` passes.
pub const C_TABLE: (u32, u32) = (0x0101_13c4, 0x0101_13e8);
pub const CPP_TABLE: (u32, u32) = (0x00fd_f4a4, 0x0101_13c0);
/// `_atexit` (LIBCMT).
const ATEXIT: u32 = 0x00ec_658f;
/// The float square-root helper (`004019b0` → `004019d0` → `sqrt`
/// `00ec6054`).
const SQRT_HELPER: u32 = 0x0040_19b0;
const SQRT: u32 = 0x00ec_6054;

/// An x87 load that feeds a pushed argument or a constant: `FLD float ptr
/// [a]`, `FLD double ptr [a]`, `FLD1`, `FLDZ`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Source {
    F32(u32),
    F64(u32),
    One,
    Zero,
}

/// A pushed argument of an `object` row: a word (`PUSH imm`), a float
/// (`PUSH ECX; <load>; FSTP float ptr [ESP]`) or a double (`SUB ESP,8;
/// <load>; FSTP double ptr [ESP]`, two words).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arg {
    Word(u32),
    Float(Source),
    Double(Source),
}

/// One operation of a `const` row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Load(Source),
    /// `FCHS`.
    Negate,
    /// `F<op> float/double ptr [a]` on `ST0` (`true`: a double).
    Arith(Arith, u32, bool),
    /// `F<op>P` (no operands: `ST1 = ST1 <op> ST0`, pop; the reversed forms
    /// swap the operands).
    ArithPop(Arith),
    /// `FSTP float/double ptr [g]`.
    StoreF32(u32),
    StoreF64(u32),
    /// `PUSH ECX; FLD float ptr [a]; FSTP float ptr [ESP]; CALL 004019b0;
    /// ADD ESP,4`: the helper's answer pushed.
    Sqrt(u32),
    /// `CALL f` without arguments: its answer in `EAX`.
    Call(u32),
    /// `MOV [g],EAX` after a call.
    StoreEax(u32),
    /// `MOV dword/byte ptr [g],imm`.
    StoreWord(u32, u32),
    StoreByte(u32, u8),
    /// `MOV reg,[a]; MOV [g],reg`.
    Copy(u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arith {
    Mul,
    Div,
    DivR,
    Add,
    Sub,
    SubR,
}

/// One row of the table.
#[derive(Debug, Clone, PartialEq)]
pub enum Row {
    Reg {
        function: u32,
        global: u32,
    },
    Object {
        constructor: u32,
        this: u32,
        destructor: Option<u32>,
        args: Vec<Arg>,
    },
    Const(Vec<Op>),
}

fn hex(s: &str) -> Result<u32, String> {
    u32::from_str_radix(s, 16).map_err(|_| format!("not a hex word: {s}"))
}

fn source(s: &str) -> Result<Source, String> {
    match s {
        "1" => Ok(Source::One),
        "0" => Ok(Source::Zero),
        _ => match s.split_once('.') {
            Some(("s", a)) => Ok(Source::F32(hex(a)?)),
            Some(("d", a)) => Ok(Source::F64(hex(a)?)),
            _ => Err(format!("x87 source {s}")),
        },
    }
}

fn parse_arg(s: &str) -> Result<Arg, String> {
    match s.split_once(':') {
        Some(("w", v)) => Ok(Arg::Word(hex(v)?)),
        Some(("f", v)) => Ok(Arg::Float(source(v)?)),
        Some(("d", v)) => Ok(Arg::Double(source(v)?)),
        _ => Err(format!("argument {s}")),
    }
}

fn parse_op(s: &str) -> Result<Op, String> {
    let mut w = s.split(' ');
    let name = w.next().unwrap_or("");
    let mut arg = || {
        w.next()
            .ok_or(format!("{s}: missing operand"))
            .and_then(hex)
    };
    let arith = |m: &str| match m {
        "fmul" => Some(Arith::Mul),
        "fdiv" => Some(Arith::Div),
        "fdivr" => Some(Arith::DivR),
        "fadd" => Some(Arith::Add),
        "fsub" => Some(Arith::Sub),
        "fsubr" => Some(Arith::SubR),
        _ => None,
    };
    Ok(match name {
        "fld.s" => Op::Load(Source::F32(arg()?)),
        "fld.d" => Op::Load(Source::F64(arg()?)),
        "fld1" => Op::Load(Source::One),
        "fldz" => Op::Load(Source::Zero),
        "fchs" => Op::Negate,
        "fstp.s" => Op::StoreF32(arg()?),
        "fstp.d" => Op::StoreF64(arg()?),
        "sqrt" => Op::Sqrt(arg()?),
        "call" => Op::Call(arg()?),
        "st.eax" => Op::StoreEax(arg()?),
        "st.w" => {
            let g = arg()?;
            Op::StoreWord(g, arg()?)
        }
        "st.b" => {
            let g = arg()?;
            Op::StoreByte(g, arg()? as u8)
        }
        "cp.d" => {
            let g = arg()?;
            Op::Copy(g, arg()?)
        }
        "fmulp" => Op::ArithPop(Arith::Mul),
        "fdivp" => Op::ArithPop(Arith::Div),
        "fdivrp" => Op::ArithPop(Arith::DivR),
        "faddp" => Op::ArithPop(Arith::Add),
        "fsubp" => Op::ArithPop(Arith::Sub),
        "fsubrp" => Op::ArithPop(Arith::SubR),
        m => {
            let (base, size) = m.split_once('.').ok_or(format!("unknown op {s}"))?;
            let a = arith(base).ok_or(format!("unknown op {s}"))?;
            let double = match size {
                "s" => false,
                "d" => true,
                _ => return Err(format!("unknown op {s}")),
            };
            Op::Arith(a, arg()?, double)
        }
    })
}

/// The table's rows by entry address.
pub fn parse(text: &str) -> Result<HashMap<u32, Row>, String> {
    let mut rows = HashMap::new();
    for (n, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let at = |e: String| format!("static_initializers.tsv:{}: {e}", n + 1);
        let mut cols = line.split('\t');
        let (Some(entry), Some(shape), Some(params)) = (cols.next(), cols.next(), cols.next())
        else {
            return Err(at("three columns expected".into()));
        };
        let entry = hex(entry).map_err(at)?;
        let p: Vec<&str> = params.split(' ').collect();
        let row = match shape {
            "reg" if p.len() == 2 => Row::Reg {
                function: hex(p[0]).map_err(at)?,
                global: hex(p[1]).map_err(at)?,
            },
            "object" if p.len() >= 3 => Row::Object {
                constructor: hex(p[0]).map_err(at)?,
                this: hex(p[1]).map_err(at)?,
                destructor: match p[2] {
                    "-" => None,
                    d => Some(hex(d).map_err(at)?),
                },
                args: p[3..]
                    .iter()
                    .map(|a| parse_arg(a))
                    .collect::<Result<_, _>>()
                    .map_err(at)?,
            },
            "const" => Row::Const(
                params
                    .split(';')
                    .map(parse_op)
                    .collect::<Result<_, _>>()
                    .map_err(at)?,
            ),
            _ => return Err(at(format!("shape {shape} with {} parameters", p.len()))),
        };
        if rows.insert(entry, row).is_some() {
            return Err(at(format!("{entry:08x} twice")));
        }
    }
    Ok(rows)
}

/// The rows of the committed table.
pub fn table() -> HashMap<u32, Row> {
    parse(TABLE).expect("data/static_initializers.tsv")
}

// ---- x87 ---------------------------------------------------------------------

// The x87 registers hold extended values; with the precision control Windows
// gives a thread (`0x27f`: 53-bit mantissa, round to nearest; the CRT's
// `sqrt` checks for it at `00ec6068`) every result is rounded to a double's
// mantissa, which `f64` arithmetic reproduces. The extended exponent range is
// not modelled (no initializer's constant comes near a double's overflow or
// denormal range). NaNs follow the x87's rules below.

fn quiet(v: f64) -> f64 {
    if v.is_nan() {
        f64::from_bits(v.to_bits() | 0x0008_0000_0000_0000)
    } else {
        v
    }
}

/// `FLD float`: exact; a signaling NaN comes out quiet, payload kept.
fn load_f32(bits: u32) -> f64 {
    let nan = bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0;
    if nan {
        let sign = u64::from(bits >> 31) << 63;
        let payload = u64::from(bits & 0x007f_ffff) << 29;
        f64::from_bits(sign | 0x7ff8_0000_0000_0000 | payload)
    } else {
        f64::from(f32::from_bits(bits))
    }
}

/// `FSTP float`: rounded to nearest; a NaN keeps its sign and the top of
/// its payload, quiet.
fn store_f32(v: f64) -> u32 {
    if v.is_nan() {
        let b = v.to_bits();
        let sign = ((b >> 63) as u32) << 31;
        sign | 0x7fc0_0000 | ((b >> 29) as u32 & 0x003f_ffff)
    } else {
        (v as f32).to_bits()
    }
}

/// The x87 default NaN ("real indefinite").
const INDEFINITE: u64 = 0xfff8_0000_0000_0000;

/// `a <op> b` as the x87 computes it: a NaN operand gives the NaN with the
/// larger significand (quieted), an invalid operation the indefinite NaN.
fn arith(op: Arith, a: f64, b: f64) -> f64 {
    let (x, y) = match op {
        Arith::DivR | Arith::SubR => (b, a),
        _ => (a, b),
    };
    if x.is_nan() || y.is_nan() {
        let pick = match (x.is_nan(), y.is_nan()) {
            (true, true) => {
                let m = |v: f64| v.to_bits() & 0x000f_ffff_ffff_ffff;
                if m(x) >= m(y) {
                    x
                } else {
                    y
                }
            }
            (true, false) => x,
            _ => y,
        };
        return quiet(pick);
    }
    let r = match op {
        Arith::Mul => x * y,
        Arith::Div | Arith::DivR => x / y,
        Arith::Add => x + y,
        Arith::Sub | Arith::SubR => x - y,
    };
    if r.is_nan() {
        f64::from_bits(INDEFINITE)
    } else {
        r
    }
}

fn load(e: &Engine, s: Source) -> f64 {
    match s {
        Source::F32(a) => load_f32(e.mem.u32(a)),
        Source::F64(a) => quiet(e.mem.f64(a)),
        Source::One => 1.0,
        Source::Zero => 0.0,
    }
}

// ---- Shapes ------------------------------------------------------------------

/// Shape `reg`: `CALL f; MOV [g],EAX` (the one-shot NiAnimation
/// registrations: their answer kept).
pub fn run_reg(e: &mut Engine, function: u32, global: u32) {
    let v = e.call(function, &[]).u32();
    e.mem.set_u32(global, v);
}

/// Shape `object`: `<pushes>; MOV ECX,g; CALL ctor; [PUSH dtor; CALL
/// _atexit; ADD ESP,4 | POP ECX]`: the constructor on the global with its
/// arguments (a float or double pushed through the x87 stack), then the
/// destructor registered.
pub fn run_object(
    e: &mut Engine,
    constructor: u32,
    this: u32,
    destructor: Option<u32>,
    args: &[Arg],
) {
    let mut words = vec![this];
    for a in args {
        match *a {
            Arg::Word(v) => words.push(v),
            Arg::Float(s) => words.push(store_f32(load(e, s))),
            Arg::Double(s) => {
                let bits = load(e, s).to_bits();
                words.push(bits as u32);
                words.push((bits >> 32) as u32);
            }
        }
    }
    e.call(constructor, &words);
    if let Some(d) = destructor {
        e.call(ATEXIT, &[d]);
    }
}

/// Shape `const`: straight-line constants, the x87 stack empty before and
/// after.
pub fn run_const(e: &mut Engine, ops: &[Op]) {
    let mut st: Vec<f64> = Vec::new();
    let mut eax = 0u32;
    let top = |st: &mut Vec<f64>| st.pop().expect("x87 stack (checked by initgen)");
    for op in ops {
        match *op {
            Op::Load(s) => st.push(load(e, s)),
            Op::Negate => {
                let v = top(&mut st);
                st.push(f64::from_bits(v.to_bits() ^ (1 << 63)));
            }
            Op::Arith(kind, a, double) => {
                let m = load(
                    e,
                    if double {
                        Source::F64(a)
                    } else {
                        Source::F32(a)
                    },
                );
                let v = top(&mut st);
                st.push(arith(kind, v, m));
            }
            Op::ArithPop(kind) => {
                let st0 = top(&mut st);
                let st1 = top(&mut st);
                st.push(arith(kind, st1, st0));
            }
            Op::StoreF32(g) => {
                let v = top(&mut st);
                e.mem.set_u32(g, store_f32(v));
            }
            Op::StoreF64(g) => {
                let v = top(&mut st);
                e.mem.set_f64(g, v);
            }
            Op::Sqrt(a) => {
                let passed = store_f32(load_f32(e.mem.u32(a)));
                st.push(e.call(SQRT_HELPER, &[passed]).f64());
            }
            Op::Call(f) => eax = e.call(f, &[]).u32(),
            Op::StoreEax(g) => e.mem.set_u32(g, eax),
            Op::StoreWord(g, v) => e.mem.set_u32(g, v),
            Op::StoreByte(g, v) => e.mem.set_u8(g, v),
            Op::Copy(g, a) => {
                let v = e.mem.u32(a);
                e.mem.set_u32(g, v);
            }
        }
    }
}

/// The functions a row calls, which must be translated for it to run.
fn callees(row: &Row) -> Vec<u32> {
    match row {
        Row::Reg { function, .. } => vec![*function],
        Row::Object {
            constructor,
            destructor,
            ..
        } => {
            let mut c = vec![*constructor];
            if destructor.is_some() {
                c.push(ATEXIT);
            }
            c
        }
        Row::Const(ops) => {
            let mut c = Vec::new();
            for op in ops {
                match op {
                    Op::Sqrt(_) => c.extend([SQRT_HELPER, SQRT]),
                    Op::Call(f) => c.push(*f),
                    _ => {}
                }
            }
            c
        }
    }
}

fn run_row(e: &mut Engine, row: &Row) {
    match row {
        Row::Reg { function, global } => run_reg(e, *function, *global),
        Row::Object {
            constructor,
            this,
            destructor,
            args,
        } => run_object(e, *constructor, *this, *destructor, args),
        Row::Const(ops) => run_const(e, ops),
    }
}

/// What a run did.
#[derive(Debug, Default, Clone)]
pub struct Report {
    /// Entries run from a table row.
    pub rows: usize,
    /// Entries run by address (hand translations).
    pub translated: usize,
    /// Entries the caller's platform stand-ins ran.
    pub platform: usize,
    /// Entries skipped: no row and not translated.
    pub open: Vec<u32>,
    /// Rows skipped because a callee isn't translated: (entry, callee).
    pub callee_open: Vec<(u32, u32)>,
    /// Entries that stopped part-way in a function nobody has translated
    /// yet (a callee of a callee): (entry, the stop). What they wrote before
    /// the stop stays.
    pub stopped: Vec<(u32, String)>,
    /// A C initializer that answered non-zero, which stops `__cinit`.
    pub c_failed: Option<(u32, u32)>,
}

impl Report {
    pub fn skipped(&self) -> usize {
        self.open.len() + self.callee_open.len() + self.stopped.len()
    }

    /// One line per skipped entry, in the report's order: the entry, why
    /// (`open`: neither a row nor a translation; `callee X`: its row's callee
    /// isn't translated; `stopped: ...`: stopped part-way) and, when its row
    /// tells, the global it builds ([`row_global`]). Entries without a row
    /// build what only their code says (an engine-map name helps:
    /// `crates/host/tests/static_initializers.rs`).
    pub fn skip_lines(&self, rows: &HashMap<u32, Row>) -> Vec<String> {
        let global = |entry: u32| match rows.get(&entry).and_then(row_global) {
            Some(g) => format!(" (builds {g:08x})"),
            None => String::new(),
        };
        let mut out = Vec::with_capacity(self.skipped());
        for &entry in &self.open {
            out.push(format!("{entry:08x} open{}", global(entry)));
        }
        for &(entry, callee) in &self.callee_open {
            out.push(format!("{entry:08x} callee {callee:08x}{}", global(entry)));
        }
        for (entry, stop) in &self.stopped {
            out.push(format!("{entry:08x} stopped: {stop}{}", global(*entry)));
        }
        out
    }
}

/// The global a row builds: an object row's `this`, a `reg` row's global, a
/// `const` row's first store.
pub fn row_global(row: &Row) -> Option<u32> {
    match row {
        Row::Reg { global, .. } => Some(*global),
        Row::Object { this, .. } => Some(*this),
        Row::Const(ops) => ops.iter().find_map(|op| match *op {
            Op::StoreF32(g)
            | Op::StoreF64(g)
            | Op::StoreEax(g)
            | Op::StoreWord(g, _)
            | Op::StoreByte(g, _)
            | Op::Copy(_, g) => Some(g),
            _ => None,
        }),
    }
}

/// An entry a run would skip, found without running anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skip {
    /// Neither a row nor a translation.
    Open(u32),
    /// A row whose callee (the second word) isn't translated.
    Callee(u32, u32),
}

/// The entries of both tables (C, then C++, in order) that [`run`] would
/// skip, read without running any: those [`run`] counts in
/// [`Report::open`] and [`Report::callee_open`]. A stand-in's entry is
/// never one. (Entries that stop part-way, and the C table's end at a
/// non-zero answer, need a run.)
pub fn skips(e: &Engine, rows: &HashMap<u32, Row>, stand_ins: &[StandIn]) -> Vec<Skip> {
    let mut out = Vec::new();
    for entry in entries(e, C_TABLE).into_iter().chain(entries(e, CPP_TABLE)) {
        if stand_ins.iter().any(|(a, _)| *a == entry) {
            continue;
        }
        if let Some(row) = rows.get(&entry) {
            if let Some(c) = callees(row).into_iter().find(|&c| !e.is_translated(c)) {
                out.push(Skip::Callee(entry, c));
            }
        } else if !e.is_translated(entry) {
            out.push(Skip::Open(entry));
        }
    }
    out
}

/// The non-null entries of one table range, in order.
pub fn entries(e: &Engine, (start, end): (u32, u32)) -> Vec<u32> {
    (start..end)
        .step_by(4)
        .map(|slot| e.mem.u32(slot))
        .filter(|&f| f != 0)
        .collect()
}

/// Runs `f`; a stop in an untranslated function (the engine panics with
/// its address) comes back as its message instead.
pub(crate) fn guarded<R>(e: &mut Engine, f: impl FnOnce(&mut Engine) -> R) -> Result<R, String> {
    let mark = e.guest_stack_mark();
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(e))).map_err(|payload| {
        // The next initializer runs on the same stack: the stop's locals go back.
        e.guest_stack_restore(mark);
        let text = payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        text.lines().next().unwrap_or("").to_string()
    })
}

/// A platform stand-in for an entry: what the engine runs in its place
/// because the entry belongs to a layer nv-rs replaces (the memory manager).
pub type StandIn = (u32, fn(&mut Engine));

fn run_entry(
    e: &mut Engine,
    rows: &HashMap<u32, Row>,
    stand_ins: &[StandIn],
    entry: u32,
    report: &mut Report,
) -> Option<u32> {
    if let Some((_, f)) = stand_ins.iter().find(|(a, _)| *a == entry) {
        f(e);
        report.platform += 1;
        return Some(0);
    }
    if let Some(row) = rows.get(&entry) {
        if let Some(c) = callees(row).into_iter().find(|&c| !e.is_translated(c)) {
            report.callee_open.push((entry, c));
            return None;
        }
        match guarded(e, |e| run_row(e, row)) {
            Ok(()) => {
                report.rows += 1;
                Some(0)
            }
            Err(stop) => {
                report.stopped.push((entry, stop));
                None
            }
        }
    } else if e.is_translated(entry) {
        match guarded(e, |e| e.call(entry, &[]).u32()) {
            Ok(answer) => {
                report.translated += 1;
                Some(answer)
            }
            Err(stop) => {
                report.stopped.push((entry, stop));
                None
            }
        }
    } else {
        report.open.push(entry);
        None
    }
}

/// Runs the static initializers of the mapped executable as `__cinit` does
/// (the C initializers until one answers non-zero, then the C++ ones),
/// skipping what isn't translated yet; an entry with a platform stand-in
/// runs that instead. The rest of `__cinit` (the floating
/// point set-up `00eddcc0` and the callbacks after the tables) is not run.
pub fn run(e: &mut Engine, stand_ins: &[StandIn]) -> Report {
    let rows = table();
    let mut report = Report::default();
    // A stop in an untranslated callee is recorded, not printed: the panic
    // hook is silenced for the run and put back after.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    run_tables(e, &rows, stand_ins, &mut report);
    std::panic::set_hook(hook);
    report
}

fn run_tables(
    e: &mut Engine,
    rows: &HashMap<u32, Row>,
    stand_ins: &[StandIn],
    report: &mut Report,
) {
    for entry in entries(e, C_TABLE) {
        if let Some(answer) = run_entry(e, rows, stand_ins, entry, report) {
            if answer != 0 {
                report.c_failed = Some((entry, answer));
                return;
            }
        }
    }
    for entry in entries(e, CPP_TABLE) {
        run_entry(e, rows, stand_ins, entry, report);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::RetVal;

    #[test]
    fn the_committed_table_parses() {
        let rows = table();
        assert!(rows.len() > 10_000, "{} rows", rows.len());
    }

    #[test]
    fn rows_parse_by_shape() {
        let rows = parse(
            "# c\n\
             00f352d0\treg\t00a26e20 011c32f8\n\
             00f62250\tobject\t0040e0b0 011ce7c0 00fcee30 w:01054438 f:s.0101712c\n\
             00f00000\tobject\t00633000 011ce7c0 - d:1 w:ffffffff\n\
             00f38280\tconst\tfld.s 0101782c;fmul.d 01015a40;fstp.s 011c3d74;call 00e02490;st.eax 011c3d78\n",
        )
        .unwrap();
        assert_eq!(
            rows[&0x00f3_52d0],
            Row::Reg {
                function: 0x00a2_6e20,
                global: 0x011c_32f8
            }
        );
        assert_eq!(
            rows[&0x00f6_2250],
            Row::Object {
                constructor: 0x0040_e0b0,
                this: 0x011c_e7c0,
                destructor: Some(0x00fc_ee30),
                args: vec![Arg::Word(0x0105_4438), Arg::Float(Source::F32(0x0101_712c))],
            }
        );
        assert_eq!(
            rows[&0x00f0_0000],
            Row::Object {
                constructor: 0x0063_3000,
                this: 0x011c_e7c0,
                destructor: None,
                args: vec![Arg::Double(Source::One), Arg::Word(0xffff_ffff)],
            }
        );
        assert_eq!(
            rows[&0x00f3_8280],
            Row::Const(vec![
                Op::Load(Source::F32(0x0101_782c)),
                Op::Arith(Arith::Mul, 0x0101_5a40, true),
                Op::StoreF32(0x011c_3d74),
                Op::Call(0x00e0_2490),
                Op::StoreEax(0x011c_3d78),
            ])
        );
        assert!(parse("00f352d0\treg\t00a26e20\n").is_err());
        assert!(parse("00f352d0\tconst\tfoo 1\n").is_err());
        assert!(parse("00f352d0\tobject\t1 2 - q:1\n").is_err());
    }

    #[test]
    fn constants_compute_in_double_precision_and_round_on_store() {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x100);
        e.map(0x011c_0000, 0x100);
        e.mem.set_u32(0x0101_0000, 3.0f32.to_bits());
        e.mem.set_f64(0x0101_0010, 0.1);
        e.mem.set_u32(0x0101_0020, 7.0f32.to_bits());
        e.register(0x00e0_2490, |_, _| 0x1234u32.into_ret());
        run_const(
            &mut e,
            &[
                Op::Load(Source::F32(0x0101_0000)),
                Op::Arith(Arith::Mul, 0x0101_0010, true),
                Op::Arith(Arith::DivR, 0x0101_0020, false),
                Op::StoreF32(0x011c_0000),
                Op::Load(Source::F32(0x0101_0000)),
                Op::Load(Source::One),
                Op::ArithPop(Arith::DivR),
                Op::StoreF64(0x011c_0010),
                Op::StoreWord(0x011c_0020, 0xdead_beef),
                Op::StoreByte(0x011c_0024, 8),
                Op::Copy(0x011c_0028, 0x0101_0020),
                Op::Call(0x00e0_2490),
                Op::StoreEax(0x011c_002c),
            ],
        );
        assert_eq!(
            e.mem.u32(0x011c_0000),
            ((7.0 / (3.0 * 0.1)) as f32).to_bits()
        );
        // FDIVRP: ST1 = ST0 / ST1 with ST0 = 1, ST1 = 3.
        assert_eq!(e.mem.f64(0x011c_0010), 1.0 / 3.0);
        assert_eq!(e.mem.u32(0x011c_0020), 0xdead_beef);
        assert_eq!(e.mem.u8(0x011c_0024), 8);
        assert_eq!(e.mem.u32(0x011c_0028), 7.0f32.to_bits());
        assert_eq!(e.mem.u32(0x011c_002c), 0x1234);
    }

    #[test]
    fn nans_follow_the_x87() {
        assert_eq!(store_f32(load_f32(0x7f80_0001)), 0x7fc0_0001);
        assert_eq!(store_f32(load_f32(0xffc0_1234)), 0xffc0_1234);
        assert_eq!(arith(Arith::Mul, 0.0, f64::INFINITY).to_bits(), INDEFINITE);
        assert_eq!(
            arith(Arith::Sub, f64::INFINITY, f64::INFINITY).to_bits(),
            INDEFINITE
        );
        let small = f64::from_bits(0x7ff8_0000_0000_0001);
        let large = f64::from_bits(0xfff8_0000_0000_0100);
        assert_eq!(arith(Arith::Add, small, large).to_bits(), large.to_bits());
        assert_eq!(arith(Arith::Add, large, small).to_bits(), large.to_bits());
    }

    #[test]
    fn objects_get_their_arguments_in_order_and_register_the_destructor() {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x100);
        e.mem.set_u32(0x0101_0000, 0x7f80_0001);
        e.mem.set_f64(0x0101_0008, 2.5);
        e.register(0x0040_e0b0, |_, a| a[0].into_ret());
        e.register(ATEXIT, |_, _| crate::Ret::default());
        e.call_log = Some(vec![]);
        run_object(
            &mut e,
            0x0040_e0b0,
            0x011c_0000,
            Some(0x00fc_0000),
            &[
                Arg::Word(0x0105_0000),
                Arg::Float(Source::F32(0x0101_0000)),
                Arg::Double(Source::F64(0x0101_0008)),
                Arg::Float(Source::One),
            ],
        );
        let log = e.call_log.take().unwrap();
        let d = 2.5f64.to_bits();
        assert_eq!(
            log[0],
            (
                0x0040_e0b0,
                vec![
                    0x011c_0000,
                    0x0105_0000,
                    0x7fc0_0001,
                    d as u32,
                    (d >> 32) as u32,
                    1.0f32.to_bits()
                ]
            )
        );
        assert_eq!(log[1], (ATEXIT, vec![0x00fc_0000]));
    }

    #[test]
    fn an_object_runs_while_its_destructor_is_untranslated() {
        // The destructor is only registered with `_atexit` (by address, as
        // the exe does) and never runs during a session, so it doesn't hold
        // the constructor back (Build 43: the process lists, 00fa9890).
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x20);
        e.register(0x00f0_0f00, |e, a| {
            e.mem.set_u32(a[0], 7);
            a[0].into_ret()
        });
        e.register(ATEXIT, |_, _| crate::Ret::default());
        assert!(!e.is_translated(0x00f0_0fd0));
        let rows = parse("00f00010\tobject\t00f00f00 01010010 00f00fd0\n").unwrap();
        let mut report = Report::default();
        assert_eq!(
            run_entry(&mut e, &rows, &[], 0x00f0_0010, &mut report),
            Some(0)
        );
        assert_eq!(e.mem.u32(0x0101_0010), 7);
        assert!(report.callee_open.is_empty());
        assert_eq!(report.rows, 1);
    }

    #[test]
    fn untranslated_entries_and_callees_are_skipped_and_counted() {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x20);
        let rows = parse("00f00010\treg\t00f00f00 01010010\n").unwrap();
        let mut report = Report::default();
        assert_eq!(
            run_entry(&mut e, &rows, &[], 0x00f0_0010, &mut report),
            None
        );
        assert_eq!(report.callee_open, [(0x00f0_0010, 0x00f0_0f00)]);
        assert_eq!(
            run_entry(&mut e, &rows, &[], 0x00f0_0020, &mut report),
            None
        );
        assert_eq!(report.open, [0x00f0_0020]);
        e.register(0x00f0_0f00, |_, _| 5u32.into_ret());
        assert_eq!(
            run_entry(&mut e, &rows, &[], 0x00f0_0010, &mut report),
            Some(0)
        );
        assert_eq!(e.mem.u32(0x0101_0010), 5);
        assert_eq!(report.rows, 1);
    }

    #[test]
    fn skips_are_listed_without_a_run_and_named_in_lines() {
        let mut e = Engine::new();
        e.map(C_TABLE.0, C_TABLE.1 - C_TABLE.0);
        e.map(CPP_TABLE.0, CPP_TABLE.1 - CPP_TABLE.0);
        // C: one translated entry; C++: a row with an untranslated callee,
        // an open entry, a stand-in's entry, and a row whose callee is
        // translated.
        e.mem.set_u32(C_TABLE.0, 0x00f0_0100);
        e.mem.set_u32(CPP_TABLE.0, 0x00f0_0010);
        e.mem.set_u32(CPP_TABLE.0 + 8, 0x00f0_0020);
        e.mem.set_u32(CPP_TABLE.0 + 12, 0x00f0_0030);
        e.mem.set_u32(CPP_TABLE.0 + 16, 0x00f0_0040);
        e.register(0x00f0_0100, |_, _| 0u32.into_ret());
        e.register(0x00f0_0f10, |_, _| 0u32.into_ret());
        let rows =
            parse("00f00010\treg\t00f00f00 01010010\n00f00040\treg\t00f00f10 01010020\n").unwrap();
        fn stand_in(_: &mut Engine) {}
        let stand_ins: [StandIn; 1] = [(0x00f0_0030, stand_in)];
        assert_eq!(
            skips(&e, &rows, &stand_ins),
            [
                Skip::Callee(0x00f0_0010, 0x00f0_0f00),
                Skip::Open(0x00f0_0020)
            ]
        );
        let report = Report {
            open: vec![0x00f0_0020],
            callee_open: vec![(0x00f0_0010, 0x00f0_0f00)],
            stopped: vec![(0x00f0_0040, "read of 00000000".to_string())],
            ..Report::default()
        };
        assert_eq!(
            report.skip_lines(&rows),
            [
                "00f00020 open",
                "00f00010 callee 00f00f00 (builds 01010010)",
                "00f00040 stopped: read of 00000000 (builds 01010020)",
            ]
        );
    }
}
