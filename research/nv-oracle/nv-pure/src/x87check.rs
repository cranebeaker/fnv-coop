//! `nv-pure x87`: the engine's x87 arithmetic helpers (`engine::x87`) and
//! helper-built models of a few pure x87 functions of the exe, against the real
//! CPU, under several x87 control words (precision and rounding control).
//!
//! The helpers are checked through small stubs, one x87 instruction each,
//! that nv-call runs from a snapshot region (our own code, not the game's):
//! `FLD` an operand from the stack, apply `FADD`/`FSUB`/`FMUL`/`FDIV` with a
//! `double` or `float` memory operand (or `FSQRT`, or `FMUL` by 1 for
//! `x87_round`), return ST0. The engine side calls the helper with the same
//! operands under the same control word; ST0 (rounded to `f64`, exact for every
//! result the model can hold) must have the helper's bits.
//!
//! The functions are exe functions whose x87 code is a few arithmetic
//! instructions on `float` arguments and `double` constants; their models do the
//! same instructions through the helpers, one call per instruction, as a
//! translation does on the main thread's path (docs/ENGINE_CRATE.md, "x87
//! precision"). They are checked at 0x027F and 0x007F (rounding to nearest: the
//! `float` stores of the model, `x87_store32`, round to nearest).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use engine::x87::{
    set_control_word, x87_add, x87_div, x87_load32, x87_load64 as fld64, x87_m32, x87_mul,
    x87_round, x87_sqrt, x87_store32, x87_sub, DEFAULT_CONTROL_WORD,
};
use engine::Engine;

use crate::list::Pure;
use crate::source::Ty;
use crate::vectors::{self, Outcome, Rng, Val};

/// Where the stubs go in nv-call's process: free address space, allocated
/// executable for the snapshot region.
const STUB_BASE: u32 = 0x2000_0000;
const STUB_SIZE: u32 = 16;

/// The control words of the helper vectors: 53-bit and 24-bit precision with
/// round to nearest (how the game's threads run), and 24-bit with the three
/// directed modes. 53-bit with a directed mode is not modelled.
const HELPER_WORDS: [u16; 5] = [0x027f, 0x007f, 0x047f, 0x087f, 0x0c7f];
/// The control words of the function models.
const FUNCTION_WORDS: [u16; 2] = [0x027f, 0x007f];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// How the instruction gets its operands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Form {
    /// `FLD qword [esp+4]; F<op> qword [esp+12]`: two doubles.
    F64M64,
    /// `FLD qword [esp+4]; F<op> dword [esp+12]`: a double and a float memory
    /// operand (`x87_m32`).
    F64M32,
    /// `FLD dword [esp+4]; F<op> dword [esp+8]`: a loaded float (`x87_load32`)
    /// and a float memory operand.
    F32M32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Binary(Form, Op),
    /// `FLD qword [esp+4]; FSQRT`.
    Sqrt,
    /// `FLD qword [esp+4]; FLD1; FMULP`: the value rounded to the precision
    /// (`x87_round`), for values that are not NaN.
    Round,
}

impl Case {
    fn all() -> Vec<Case> {
        let mut v = Vec::new();
        for form in [Form::F64M64, Form::F64M32, Form::F32M32] {
            for op in [Op::Add, Op::Sub, Op::Mul, Op::Div] {
                v.push(Case::Binary(form, op));
            }
        }
        v.push(Case::Sqrt);
        v.push(Case::Round);
        v
    }

    fn name(self) -> String {
        match self {
            Case::Binary(form, op) => format!("{op:?}-{form:?}").to_lowercase(),
            Case::Sqrt => "sqrt".into(),
            Case::Round => "round".into(),
        }
    }

    /// The stub's machine code (cdecl, result in ST0).
    fn code(self) -> Vec<u8> {
        let modrm = |op: Op| match op {
            Op::Add => 0x44,
            Op::Mul => 0x4c,
            Op::Sub => 0x64,
            Op::Div => 0x74,
        };
        match self {
            Case::Binary(Form::F64M64, op) => {
                vec![0xdd, 0x44, 0x24, 0x04, 0xdc, modrm(op), 0x24, 0x0c, 0xc3]
            }
            Case::Binary(Form::F64M32, op) => {
                vec![0xdd, 0x44, 0x24, 0x04, 0xd8, modrm(op), 0x24, 0x0c, 0xc3]
            }
            Case::Binary(Form::F32M32, op) => {
                vec![0xd9, 0x44, 0x24, 0x04, 0xd8, modrm(op), 0x24, 0x08, 0xc3]
            }
            Case::Sqrt => vec![0xdd, 0x44, 0x24, 0x04, 0xd9, 0xfa, 0xc3],
            Case::Round => vec![0xdd, 0x44, 0x24, 0x04, 0xd9, 0xe8, 0xde, 0xc9, 0xc3],
        }
    }

    /// The argument types (raw bits in `Val`).
    fn params(self) -> Vec<Ty> {
        match self {
            Case::Binary(Form::F64M64, _) => vec![Ty::F64, Ty::F64],
            Case::Binary(Form::F64M32, _) => vec![Ty::F64, Ty::F32],
            Case::Binary(Form::F32M32, _) => vec![Ty::F32, Ty::F32],
            Case::Sqrt | Case::Round => vec![Ty::F64],
        }
    }

    /// The engine's result under the control word loaded now.
    fn helper(self, args: &[Val]) -> f64 {
        let f64_of = |v: Val| f64::from_bits(v.bits);
        match self {
            Case::Binary(form, op) => {
                let (a, b) = match form {
                    Form::F64M64 => (fld64(args[0].bits), f64_of(args[1])),
                    Form::F64M32 => (fld64(args[0].bits), x87_m32(args[1].bits as u32)),
                    Form::F32M32 => (
                        x87_load32(args[0].bits as u32),
                        x87_m32(args[1].bits as u32),
                    ),
                };
                match op {
                    Op::Add => x87_add(a, b),
                    Op::Sub => x87_sub(a, b),
                    Op::Mul => x87_mul(a, b),
                    Op::Div => x87_div(a, b),
                }
            }
            Case::Sqrt => x87_sqrt(fld64(args[0].bits)),
            Case::Round => x87_round(fld64(args[0].bits)),
        }
    }
}

/// Doubles every case meets: zeros, small integers, values a float cannot
/// hold, 24-bit rounding ties, values beyond the float range, float denormals
/// (as doubles), infinities and NaNs.
pub const F64_EDGES: &[u64] = &[
    0x0000_0000_0000_0000, // 0
    0x8000_0000_0000_0000, // -0
    0x3ff0_0000_0000_0000, // 1
    0xbff0_0000_0000_0000, // -1
    0x3ff8_0000_0000_0000, // 1.5
    0x4008_0000_0000_0000, // 3
    0x3fd5_5555_5555_5555, // 1/3 (double)
    0x3e70_0000_0000_0000, // 2^-24
    0x3ff0_0000_2000_0000, // 1 + 2^-23 (a float)
    0x3ff0_0000_1000_0000, // 1 + 2^-24 (a tie at 24 bits)
    0x3ff0_0000_3000_0000, // 1 + 3 * 2^-24 (a tie, rounds up to even)
    0x3ff0_0000_1000_0001, // 1 + 2^-24 + 2^-52 (just above the tie)
    0x3ff0_0000_0fff_ffff, // just below the tie
    0x3e70_0000_0000_0040, // 2^-24 + 2^-70: 1 + this is a tie in f64, not exactly
    0xbe70_0000_0000_0040, // -(2^-24 + 2^-70)
    0x3fef_ffff_f000_0000, // 1 - 2^-25 (a tie below 1)
    0x4197_d784_0000_0000, // 1e8
    0x4415_af1d_78b5_8c40, // 1e20
    0x4629_9a6c_5c8a_5e89, // 1e30-ish (not a float)
    0x47ef_ffff_e000_0000, // FLT_MAX
    0x47f0_0000_0000_0000, // 2^128, beyond float range
    0x4c16_9a3d_be2a_8d31, // ~1e58, beyond float range
    0x3810_0000_0000_0000, // 2^-126, smallest normal float
    0x36a0_0000_0000_0000, // 2^-149, smallest float denormal
    0x37d0_0000_0000_0001, // 2^-130 + a bit (a float denormal's range)
    0x3380_0000_0000_0000, // 2^-199, below the float range
    0x7e37_e43c_8800_759c, // 1e300
    0x7fef_ffff_ffff_ffff, // DBL_MAX
    0x0010_0000_0000_0000, // smallest normal double
    0x7ff0_0000_0000_0000, // +inf
    0xfff0_0000_0000_0000, // -inf
    0x7ff8_0000_0000_0000, // qNaN
    0xfff8_0000_0000_0000, // -qNaN (the indefinite)
    0x7ff0_0000_0000_0001, // sNaN
    0x7ffc_0000_1234_5678, // qNaN with payload
];

/// Floats for the `float` operands.
pub const F32_EDGES: &[u32] = &[
    0x0000_0000, // 0
    0x8000_0000, // -0
    0x3f80_0000, // 1
    0xbf80_0000, // -1
    0x3fc0_0000, // 1.5
    0x4040_0000, // 3
    0x3eaa_aaab, // 1/3
    0x3380_0000, // 2^-24
    0x3f80_0001, // 1 + 2^-23
    0x7149_f2ca, // 1e30
    0x7f7f_ffff, // FLT_MAX
    0x0080_0000, // smallest normal
    0x0000_0001, // smallest denormal
    0x0000_0003, // 3 * 2^-149
    0x007f_ffff, // largest denormal
    0x7f80_0000, // +inf
    0xff80_0000, // -inf
    0x7fc0_0000, // qNaN
    0xffc0_0000, // -qNaN
    0x7fc1_2345, // qNaN with payload
    0x7f80_0001, // sNaN
    0xffbf_ffff, // -sNaN, large payload
];

fn random_float(r: &mut Rng) -> u32 {
    match r.next() % 8 {
        // any bit pattern (denormals, NaNs and infinities included)
        0 => r.next() as u32,
        // a float denormal
        1 => (r.next() as u32 & 0x807f_ffff) | 1,
        // moderate magnitudes
        _ => {
            let exponent = 100 + (r.next() % 55) as u32;
            (r.next() as u32 & 0x807f_ffff) | exponent << 23
        }
    }
}

/// A 24-bit value (a float), as a double.
fn float_valued(r: &mut Rng) -> u64 {
    let f = f32::from_bits(random_float(r));
    f64::from(f).to_bits()
}

fn random_double(r: &mut Rng) -> u64 {
    let sign = r.next() & 1 << 63;
    let fraction = r.next() & 0x000f_ffff_ffff_ffff;
    let exponent = match r.next() % 8 {
        // beyond the float range on either side
        0 => 1023 + 128 + r.next() % 300,
        1 => 1023 - 127 - r.next() % 300,
        // the float denormal range
        2 => 1023 - 127 - r.next() % 24,
        _ => 1023 - 40 + r.next() % 80,
    };
    sign | exponent << 52 | fraction
}

/// `a` (a float) and a second operand that puts `a + b` near a 24-bit
/// rounding tie: half a unit of `a`'s 24th bit, nudged by 2^-k of it (both
/// ways, or not at all). With k past 29 the double sum lands exactly on the tie
/// and only the sticky bits decide.
fn tie_pair(r: &mut Rng) -> (u64, u64) {
    let a = loop {
        let a = f64::from(f32::from_bits(random_float(r)));
        if a.is_finite() && a != 0.0 && a.abs() > 1e-30 {
            break a;
        }
    };
    let exponent = ((a.to_bits() >> 52) & 0x7ff) as i32 - 1023;
    let half_unit = 2f64.powi(exponent - 24);
    let k = 20 + (r.next() % 33) as i32;
    let nudge = match r.next() % 3 {
        0 => 0.0,
        1 => half_unit * 2f64.powi(-k),
        _ => -half_unit * 2f64.powi(-k),
    };
    let sign = if r.next() & 1 == 0 { 1.0 } else { -1.0 };
    (a.to_bits(), (sign * (half_unit + nudge)).to_bits())
}

fn inputs(case: Case, count: usize, seed: u64) -> Vec<Vec<Val>> {
    let mut r = Rng::new(seed);
    let mut out = Vec::new();
    let v = |ty: Ty, bits: u64| Val { ty, bits };
    let params = case.params();
    let edges = |ty: Ty| -> Vec<u64> {
        match ty {
            Ty::F32 => F32_EDGES.iter().map(|&b| u64::from(b)).collect(),
            _ => F64_EDGES.to_vec(),
        }
    };
    if params.len() == 2 {
        for a in edges(params[0]) {
            for b in edges(params[1]) {
                out.push(vec![v(params[0], a), v(params[1], b)]);
            }
        }
    } else {
        for a in edges(Ty::F64) {
            out.push(vec![v(Ty::F64, a)]);
        }
    }
    for i in 0..count {
        let one = |r: &mut Rng, ty: Ty| match ty {
            Ty::F32 => u64::from(random_float(r)),
            _ => {
                if r.next() & 1 == 0 {
                    float_valued(r)
                } else {
                    random_double(r)
                }
            }
        };
        let args = match case {
            Case::Binary(Form::F64M64, Op::Add | Op::Sub) if i % 3 == 0 => {
                let (a, b) = tie_pair(&mut r);
                vec![v(Ty::F64, a), v(Ty::F64, b)]
            }
            Case::Sqrt => {
                let a = one(&mut r, Ty::F64) & !(u64::from(i % 5 != 0) << 63);
                vec![v(Ty::F64, a)]
            }
            _ => params.iter().map(|&t| v(t, one(&mut r, t))).collect(),
        };
        out.push(args);
    }
    if case == Case::Round {
        out.retain(|a| !f64::from_bits(a[0].bits).is_nan());
    }
    out
}

/// One pure exe function and its model through the helpers.
struct Function {
    addr: u32,
    params: usize,
    /// `float` arguments (bits), the constants it reads; the `float` result
    /// (bits) under the control word loaded now.
    model: fn(&[u32], &Constants) -> u32,
}

/// The `double` constants the models read, from the exe's data sections.
struct Constants {
    three_quarters: f64,
    condition_scale: f64,
    half: f64,
    one: f64,
}

fn functions() -> Vec<Function> {
    vec![
        // 00c5d300 (bsmain): FLD b; FSUB a; FMUL t; FADD a; FSTP float; FLD.
        Function {
            addr: 0x00c5_d300,
            params: 3,
            model: |a, _| {
                let (t, from, to) = (a[0], a[1], a[2]);
                let d = x87_sub(x87_load32(to), x87_m32(from));
                let p = x87_mul(d, x87_m32(t));
                x87_store32(x87_add(p, x87_m32(from)))
            },
        },
        // 004b3ab0 (globalfunc): FLD t; FSUB t0; FLD t1; FSUB t0; FDIVP;
        // FLD b; FSUB a; FMULP; FADD a; FSTP float; FLD.
        Function {
            addr: 0x004b_3ab0,
            params: 5,
            model: |a, _| {
                let (lo, hi, t0, t1, t) = (a[0], a[1], a[2], a[3], a[4]);
                let num = x87_sub(x87_load32(t), x87_m32(t0));
                let den = x87_sub(x87_load32(t1), x87_m32(t0));
                let ratio = x87_div(num, den);
                let span = x87_sub(x87_load32(hi), x87_m32(lo));
                x87_store32(x87_add(x87_mul(ratio, span), x87_m32(lo)))
            },
        },
        // 00646d00 (combatformulas): FLD c; FCOMP [0101de30]; JNZ on C0|C3:
        // FLD c; FSUBR [0101de30]; FMUL [01051680]; FLD1; FSUBRP; FSTP float;
        // FLD. Otherwise FLD1.
        Function {
            addr: 0x0064_6d00,
            params: 1,
            model: |a, k| condition_curve(a[0], k.three_quarters, k.condition_scale),
        },
        // 00646d40: the same with [01011588] (0.5) and [01012070] (1.0).
        Function {
            addr: 0x0064_6d40,
            params: 1,
            model: |a, k| condition_curve(a[0], k.half, k.one),
        },
        // 00647920 (CombatFormulas::CalcExplosionPower): when second <= first
        // (FCOMPP, exactly one of C0/C3): FLD second; FDIV first; FSTP p;
        // FLD p; FMUL p; FSTP p; FLD p; FLD1; FSUBRP; FSTP p; FLD p. Else 0.
        Function {
            addr: 0x0064_7920,
            params: 2,
            model: |a, _| {
                let (first, second) = (a[0], a[1]);
                // Not (second <= first), NaN included.
                if f32::from_bits(second)
                    .partial_cmp(&f32::from_bits(first))
                    .is_none_or(std::cmp::Ordering::is_gt)
                {
                    return 0;
                }
                let p = x87_store32(x87_div(x87_load32(second), x87_m32(first)));
                let p = x87_store32(x87_mul(x87_load32(p), x87_m32(p)));
                x87_store32(x87_sub(1.0, x87_load32(p)))
            },
        },
    ]
}

fn condition_curve(condition: u32, threshold: f64, scale: f64) -> u32 {
    // `FCOMP`: C0 for below, C3 for equal, both (and C2) for unordered.
    let c = f64::from(f32::from_bits(condition));
    if c > threshold {
        return 0x3f80_0000;
    }
    let d = x87_sub(threshold, x87_load32(condition));
    let p = x87_mul(d, scale);
    x87_store32(x87_sub(1.0, p))
}

pub struct Options {
    pub exe: PathBuf,
    pub nv_call: PathBuf,
    pub out_dir: PathBuf,
    pub count: usize,
    pub list: Vec<Pure>,
}

/// Counts per group (`(control word, case or function)`).
#[derive(Default)]
pub struct Tally {
    pub vectors: usize,
    pub matched: usize,
    pub failed: usize,
    pub faults: usize,
    /// Different, but a result below the smallest normal double (2^-1022),
    /// where the model's f64 itself has fewer bits than the FPU's extended
    /// value: outside the model, not a failure.
    pub outside: usize,
}

pub struct Report {
    pub helpers: BTreeMap<(u16, String), Tally>,
    pub functions: BTreeMap<(u16, String), Tally>,
    /// (group, id, inputs, exe, engine)
    pub failures: Vec<(String, String, String, String, String)>,
}

fn with_word(line: String, word: u16) -> String {
    let mut l = line;
    l.pop();
    format!("{l},\"fpcw\":\"0x{word:04X}\"}}")
}

fn show_f64(bits: u64) -> String {
    format!("{:?}(0x{bits:016x})", f64::from_bits(bits))
}

pub fn run(o: &Options) -> Result<Report, String> {
    if !o.nv_call.is_file() {
        return Err(format!(
            "{} not found; build it with `cargo build --release --target i686-pc-windows-msvc` in research/nv-oracle",
            o.nv_call.display()
        ));
    }
    let snapshot = o.out_dir.join("x87-stubs");
    fs::create_dir_all(&snapshot).map_err(|e| format!("{}: {e}", snapshot.display()))?;
    let cases = Case::all();
    let mut blob = vec![0xccu8; cases.len() * STUB_SIZE as usize];
    for (i, c) in cases.iter().enumerate() {
        let code = c.code();
        blob[i * STUB_SIZE as usize..i * STUB_SIZE as usize + code.len()].copy_from_slice(&code);
    }
    let blob_path = snapshot.join(format!("{STUB_BASE:08x}.bin"));
    fs::write(&blob_path, &blob).map_err(|e| format!("{}: {e}", blob_path.display()))?;

    // The vectors: helpers at every word, functions at theirs.
    let mut lines = String::new();
    let mut helper_runs: Vec<(u16, Case, String, Vec<Val>)> = Vec::new();
    for &word in &HELPER_WORDS {
        for (i, &case) in cases.iter().enumerate() {
            let addr = STUB_BASE + STUB_SIZE * i as u32;
            for (n, args) in inputs(case, o.count, 0x87_0000 + i as u64)
                .into_iter()
                .enumerate()
            {
                let id = format!("h{word:04x}_{}_{n}", case.name());
                let line = vectors::vector_line(&id, addr, "cdecl", &args, Ty::F64);
                let _ = writeln!(lines, "{}", with_word(line, word));
                helper_runs.push((word, case, id, args));
            }
        }
    }
    let mut function_runs: Vec<(u16, usize, String, Vec<Val>)> = Vec::new();
    let funcs = functions();
    for &word in &FUNCTION_WORDS {
        for (fi, f) in funcs.iter().enumerate() {
            let consts = o
                .list
                .iter()
                .find(|p| p.addr == f.addr)
                .map(|p| p.consts.clone())
                .unwrap_or_default();
            let params = vec![Ty::F32; f.params];
            for (n, args) in vectors::inputs(&params, o.count, u64::from(f.addr), &consts)
                .into_iter()
                .enumerate()
            {
                let id = format!("f{word:04x}_{:08x}_{n}", f.addr);
                let line = vectors::vector_line(&id, f.addr, "cdecl", &args, Ty::F32);
                let _ = writeln!(lines, "{}", with_word(line, word));
                function_runs.push((word, fi, id, args));
            }
        }
    }
    let vec_path = o.out_dir.join("x87-vectors.jsonl");
    let res_path = o.out_dir.join("x87-results.jsonl");
    fs::write(&vec_path, &lines).map_err(|e| format!("{}: {e}", vec_path.display()))?;
    let out = Command::new(&o.nv_call)
        .arg(&o.exe)
        .arg(&vec_path)
        .arg("--snapshot")
        .arg(&snapshot)
        .arg("--keep-state")
        .output()
        .map_err(|e| format!("running {}: {e}", o.nv_call.display()))?;
    fs::write(&res_path, &out.stdout).map_err(|e| format!("{}: {e}", res_path.display()))?;
    if !matches!(out.status.code(), Some(0) | Some(2)) {
        return Err(format!(
            "nv-call failed with {:?}: {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let exe: BTreeMap<String, Outcome> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(vectors::parse_result)
        .collect();

    let mut report = Report {
        helpers: BTreeMap::new(),
        functions: BTreeMap::new(),
        failures: Vec::new(),
    };
    let shown =
        |args: &[Val]| -> String { args.iter().map(|v| v.show()).collect::<Vec<_>>().join(", ") };
    for (word, case, id, args) in &helper_runs {
        set_control_word(*word);
        let ours = case.helper(args).to_bits();
        set_control_word(DEFAULT_CONTROL_WORD);
        let t = report.helpers.entry((*word, case.name())).or_default();
        t.vectors += 1;
        let theirs = match exe.get(id) {
            Some(Outcome::Value {
                st0_f64: Some(b), ..
            }) => *b,
            other => {
                t.faults += 1;
                report.failures.push((
                    format!("{word:04x} {}", case.name()),
                    id.clone(),
                    shown(args),
                    format!("{other:?}"),
                    show_f64(ours),
                ));
                continue;
            }
        };
        let subnormal = |bits: u64| bits & 0x7ff0_0000_0000_0000 == 0 && bits << 1 != 0;
        if theirs == ours {
            t.matched += 1;
        } else if subnormal(theirs) || subnormal(ours) {
            t.outside += 1;
            report.failures.push((
                format!("{word:04x} {} (outside: f64 subnormal)", case.name()),
                id.clone(),
                shown(args),
                show_f64(theirs),
                show_f64(ours),
            ));
        } else {
            t.failed += 1;
            report.failures.push((
                format!("{word:04x} {}", case.name()),
                id.clone(),
                shown(args),
                show_f64(theirs),
                show_f64(ours),
            ));
        }
    }

    let e = Engine::with_exe(&o.exe)?;
    let k = Constants {
        three_quarters: e.global::<f64>(0x0101_de30),
        condition_scale: e.global::<f64>(0x0105_1680),
        half: e.global::<f64>(0x0101_1588),
        one: e.global::<f64>(0x0101_2070),
    };
    drop(e);
    for (word, fi, id, args) in &function_runs {
        let f = &funcs[*fi];
        let words: Vec<u32> = args.iter().map(|v| v.bits as u32).collect();
        set_control_word(*word);
        let ours = (f.model)(&words, &k);
        set_control_word(DEFAULT_CONTROL_WORD);
        let t = report
            .functions
            .entry((*word, format!("{:08x}", f.addr)))
            .or_default();
        t.vectors += 1;
        let theirs = match exe.get(id) {
            Some(Outcome::Value {
                st0_f32: Some(b), ..
            }) => *b,
            other => {
                t.faults += 1;
                report.failures.push((
                    format!("{word:04x} {:08x}", f.addr),
                    id.clone(),
                    shown(args),
                    format!("{other:?}"),
                    format!("0x{ours:08x}"),
                ));
                continue;
            }
        };
        if theirs == ours {
            t.matched += 1;
        } else {
            t.failed += 1;
            report.failures.push((
                format!("{word:04x} {:08x}", f.addr),
                id.clone(),
                shown(args),
                format!("{:?}(0x{theirs:08x})", f32::from_bits(theirs)),
                format!("{:?}(0x{ours:08x})", f32::from_bits(ours)),
            ));
        }
    }

    let mut tsv = String::from("group\tvector\tinputs\texe\tengine\n");
    for (g, id, ins, ex, en) in &report.failures {
        let _ = writeln!(tsv, "{g}\t{id}\t{ins}\t{ex}\t{en}");
    }
    let path = o.out_dir.join("x87-failures.tsv");
    fs::write(&path, tsv).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(report)
}

/// The helper vectors' file names, for the report.
pub fn files(out_dir: &Path) -> String {
    format!(
        "{} (vectors {}, results {})",
        out_dir.join("x87-failures.tsv").display(),
        out_dir.join("x87-vectors.jsonl").display(),
        out_dir.join("x87-results.jsonl").display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stubs_fit_and_inputs_repeat() {
        for c in Case::all() {
            assert!(c.code().len() <= STUB_SIZE as usize, "{c:?}");
            assert_eq!(inputs(c, 50, 7), inputs(c, 50, 7));
        }
        assert!(inputs(Case::Round, 200, 1)
            .iter()
            .all(|a| !f64::from_bits(a[0].bits).is_nan()));
    }

    #[test]
    fn models_agree_with_plain_f64_at_53_bits() {
        let k = Constants {
            three_quarters: 0.75,
            condition_scale: f64::from(0.68f32),
            half: 0.5,
            one: 1.0,
        };
        let f = functions();
        let lerp = (f[0].model)(&[0.25f32.to_bits(), 1.0f32.to_bits(), 3.0f32.to_bits()], &k);
        assert_eq!(f32::from_bits(lerp), 1.5);
        let curve = (f[2].model)(&[0.5f32.to_bits()], &k);
        assert_eq!(
            f32::from_bits(curve),
            (1.0 - (0.75 - 0.5) * f64::from(0.68f32)) as f32
        );
        assert_eq!((f[2].model)(&[0.9f32.to_bits()], &k), 0x3f80_0000);
        assert_eq!((f[4].model)(&[1.0f32.to_bits(), 2.0f32.to_bits()], &k), 0);
    }
}
