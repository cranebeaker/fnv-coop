//! Which translations the oracle check can run: translated, complete, pure in
//! the exe (transitively), with a signature whose arguments `nv-call` can
//! pass and whose calling convention agrees with the exe's own stack use.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::purity::{self, Facts};
use crate::source::{Source, Ty};

/// One checkable function.
#[derive(Debug, Clone)]
pub struct Pure {
    pub addr: u32,
    pub cc: &'static str,
    pub params: Vec<Ty>,
    pub ret: Ty,
    pub unit: String,
    /// Every function it reaches (all must be translated for the Rust side).
    pub callees: Vec<u32>,
    /// Constants of the function and its callees (input tokens, see
    /// `purity::Facts::consts`), used as boundary inputs.
    pub consts: Vec<String>,
    /// One buffer per pointer parameter (the fields read and, in the store
    /// tier, stored through it).
    pub bufs: Vec<Buf>,
    /// The exe may read memory at an address computed from the inputs (a
    /// table index): the function or a callee has LOADs no input explains,
    /// or no data flow. Only then can an engine read of memory it never maps
    /// be the exe's own read (inconclusive); otherwise it is a read the exe
    /// didn't make (`RustStrayRead`).
    pub computed_reads: bool,
}

/// The buffer a pointer parameter points into.
#[derive(Debug, Clone, PartialEq)]
pub struct Buf {
    /// Index of the parameter in the signature.
    pub param: usize,
    /// Buffer size in bytes (16-aligned, with room past the last field).
    pub size: u32,
    /// The pointer passed is the buffer's address plus `bias` (room for
    /// negative offsets, `CONTAINING_RECORD`-style).
    pub bias: u32,
    /// (offset from the pointer, size) of every field the exe reads or stores.
    pub fields: Vec<(i64, u32)>,
}

impl Buf {
    /// From the loads and stores the exe makes through the pointer.
    pub fn from_reads(param: usize, reads: &[(i64, u32)]) -> Buf {
        let mut fields = reads.to_vec();
        fields.sort_unstable();
        fields.dedup();
        let lowest = fields.iter().map(|f| f.0).min().unwrap_or(0).min(0);
        let end = fields
            .iter()
            .map(|f| f.0 + i64::from(f.1))
            .max()
            .unwrap_or(0)
            .max(16);
        let bias = ((-lowest + 15) / 16 * 16) as u32;
        let size = ((i64::from(bias) + end + 15) / 16 * 16 + 16) as u32;
        Buf {
            param,
            size,
            bias,
            fields,
        }
    }
}

/// The exe input (NvPurity names: `ecx`, `edx`, `s<n>`) that carries each
/// parameter under `cc`.
pub fn inputs_for(cc: &str, params: &[Ty]) -> Vec<String> {
    let mut regs: Vec<&str> = match cc {
        "thiscall" => vec!["ecx"],
        "fastcall" => vec!["ecx", "edx"],
        _ => vec![],
    };
    regs.reverse();
    let mut stack = 0usize;
    let mut out = Vec::new();
    for (i, t) in params.iter().enumerate() {
        let in_reg = match cc {
            "thiscall" => i == 0,
            "fastcall" => t.words() == 1 && *t != Ty::F32 && !regs.is_empty(),
            _ => false,
        };
        if in_reg {
            if let Some(r) = regs.pop() {
                out.push(r.to_string());
                continue;
            }
        }
        out.push(format!("s{stack}"));
        stack += 4 * t.words();
    }
    out
}

/// Why a translated function is not on the list (counted in the summary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Skip {
    Incomplete,
    NoSignature,
    NotInExeTable,
    Impure,
    VoidResult,
    PointerArgument,
    ReadsThroughPointer,
    /// The exe's stack cleanup or register use does not fit the Rust
    /// signature under any convention (a candidate translation issue, listed).
    ConventionMismatch,
    /// An argument in a register other than ECX/EDX (EAX, EBX, ESI, EDI,
    /// EBP), which nv-call cannot pass.
    RegisterInput,
    /// It or a callee executes an instruction whose result no input
    /// determines (RDTSC, CPUID, ...): the exe and the engine can't agree.
    HiddenInput,
    /// It reaches itself and reads through a pointer: a recursive call may
    /// dereference a pointer loaded from the buffers.
    RecursivePointer,
    /// No RET in the body (a thunk that tail-jumps): the stack cleanup, and
    /// so the convention, can't be checked.
    NoRet,
    /// A leading `f64` that the exe takes in x87 ST0 (`nv-call` cannot
    /// preset ST0).
    St0Input,
    /// Pointer tier: the data flow was not run or gave no result.
    NoDataFlow,
    /// Pointer tier: loads or stores through pointers no argument explains
    /// (a pointer read from memory, a callee's result).
    UntrackedPointer,
    /// Pointer tier: an access through an argument at an unknown offset.
    PointerOffsetUnknown,
    /// Pointer tier: a pointer field of the argument is dereferenced (needs
    /// nested buffers).
    PointerChase,
    /// Pointer tier: the pointer is handed to a callee.
    PointerPassed,
    /// Pointer tier: the pointer itself is stored somewhere.
    PointerEscapes,
    /// A parameter the Rust signature types as a scalar is used as a pointer
    /// by the exe.
    ScalarUsedAsPointer,
    /// Pointer tier: the exe dereferences an input (register or stack slot)
    /// that matches no parameter.
    UnmatchedInput,
}

pub struct Listing {
    pub pure: Vec<Pure>,
    pub skipped: BTreeMap<Skip, usize>,
    pub convention_mismatches: Vec<(u32, String)>,
}

/// The convention `nv-call` should use, or why none fits.
fn convention(params: &[Ty], f: &Facts) -> Result<&'static str, String> {
    let words: usize = params.iter().map(|t| t.words()).sum();
    let reg: Vec<&str> = f.reg_in.iter().map(String::as_str).collect();
    let (cc, stack_words) = match reg.as_slice() {
        [] => {
            let int_word = |t: Option<&Ty>| t.is_some_and(|t| t.words() == 1 && *t != Ty::F32);
            if f.ret_pop == 0 {
                ("cdecl", words)
            } else if f.ret_pop == 4 * words as i64 {
                ("stdcall", words)
            // No register read, but the exe pops one (two) words less than the
            // signature has: a method that ignores `this` (and EDX). Calling it as
            // thiscall (fastcall) is right either way: the exe sees the same stack
            // words, and a `this` it doesn't read changes nothing (00a38de0, a
            // shared virtual stub that answers 0).
            } else if f.ret_pop == 4 * (words as i64 - 1) && int_word(params.first()) {
                ("thiscall", words - 1)
            } else if f.ret_pop == 4 * (words as i64 - 2)
                && int_word(params.first())
                && int_word(params.get(1))
            {
                ("fastcall", words - 2)
            } else {
                return Err(format!(
                    "pops {} bytes, Rust has {} argument words",
                    f.ret_pop, words
                ));
            }
        }
        ["ECX"] => {
            if params.first().map(|t| t.words()) != Some(1) {
                return Err("reads ECX on entry but the first parameter is not one word".into());
            }
            ("thiscall", words - 1)
        }
        ["ECX", "EDX"] => {
            let in_regs = params
                .iter()
                .take(2)
                .filter(|t| t.words() == 1 && **t != Ty::F32)
                .count();
            if in_regs < 2 {
                return Err(
                    "reads ECX and EDX on entry but the first two parameters are not integer words"
                        .into(),
                );
            }
            ("fastcall", words - 2)
        }
        other => return Err(format!("reads {} on entry", other.join(","))),
    };
    if cc != "cdecl" && f.ret_pop != 4 * stack_words as i64 {
        return Err(format!(
            "{cc}: pops {} bytes, Rust leaves {} stack words",
            f.ret_pop, stack_words
        ));
    }
    if f.stack_args > 4 * stack_words as i64 {
        return Err(format!(
            "reads {} stack argument bytes, Rust passes {}",
            f.stack_args,
            4 * stack_words
        ));
    }
    Ok(cc)
}

pub fn build(src: &Source, facts: &BTreeMap<u32, Facts>) -> Listing {
    let pure_set = purity::transitively_pure(facts);
    let mut out = Listing {
        pure: Vec::new(),
        skipped: BTreeMap::new(),
        convention_mismatches: Vec::new(),
    };
    let mut skipped: BTreeMap<Skip, usize> = BTreeMap::new();
    let mut skip = |s: Skip| *skipped.entry(s).or_insert(0) += 1;
    for (&addr, unit) in &src.translated {
        if src.incomplete.contains(&addr) {
            skip(Skip::Incomplete);
            continue;
        }
        let Some(Some(sig)) = src.signatures.get(&addr) else {
            skip(Skip::NoSignature);
            continue;
        };
        let Some(f) = facts.get(&addr) else {
            skip(Skip::NotInExeTable);
            continue;
        };
        // Phase 2 of #193: the function itself may store through its pointer
        // arguments (at known offsets, which the data flow below decides) when
        // that is its only impurity and everything it calls is pure.
        let stores_through_args = !pure_set.contains(&addr)
            && f.w_pointer > 0
            && f.indirect == 0
            && f.imports == 0
            && f.w_global == 0
            && f.w_implicit == 0
            && f.fs == 0
            && f.calls.iter().all(|c| pure_set.contains(c))
            && sig.params.contains(&Ty::Ptr);
        if !pure_set.contains(&addr) && !stores_through_args {
            skip(Skip::Impure);
            continue;
        }
        // The clock and the like (hkSystemClock 00dd8e00 reads RDTSC): the two
        // sides can never agree.
        if std::iter::once(&addr)
            .chain(purity::callees(facts, addr).iter())
            .any(|c| facts.get(c).is_some_and(|g| g.hidden_in > 0))
        {
            skip(Skip::HiddenInput);
            continue;
        }
        // A function that stores through its arguments has its buffers as a
        // result; one that doesn't has nothing to compare.
        if sig.ret == Ty::Void && !stores_through_args {
            skip(Skip::VoidResult);
            continue;
        }
        let callees: BTreeSet<u32> = purity::callees(facts, addr);
        let pointer_tier = sig.params.contains(&Ty::Ptr);
        if !pointer_tier && sig.ret == Ty::Ptr {
            skip(Skip::PointerArgument);
            continue;
        }
        let callees_read_pointer = callees
            .iter()
            .any(|c| facts.get(c).is_none_or(|g| g.r_pointer > 0));
        if callees_read_pointer || (!pointer_tier && f.r_pointer > 0) {
            skip(Skip::ReadsThroughPointer);
            continue;
        }
        // A function that reaches itself is its own callee, which the callee
        // check above leaves out: a recursive call may walk a pointer loaded from
        // the buffers (00b15110 recurses on `m_pParent`, 00e3ec00 likewise).
        let recursive = std::iter::once(&addr)
            .chain(callees.iter())
            .any(|c| facts.get(c).is_some_and(|g| g.calls.contains(&addr)));
        if recursive && f.r_pointer > 0 {
            skip(Skip::RecursivePointer);
            continue;
        }
        if sig.params.first() == Some(&Ty::F64)
            && f.reg_in.is_empty()
            && f.ret_pop == 0
            && f.stack_args < 8
        {
            skip(Skip::St0Input);
            continue;
        }
        // ECX/EDX on entry: the entry-block scan, plus the data flow's inputs when
        // they are dereferenced or used as a number (it sees registers used after
        // a branch too, which the scan stops at). A register the data flow only
        // sees flowing into a call is not evidence: the decompiler lets ECX reach
        // any callee whose convention it doesn't know (_ftol2 and the like).
        let mut with_regs = f.clone();
        if let Some(args) = &f.args {
            for (name, reg) in [("ecx", "ECX"), ("edx", "EDX")] {
                let used = args.get(name).is_some_and(|a| a.dereferenced() || a.used);
                if used && !with_regs.reg_in.iter().any(|r| r == reg) {
                    with_regs.reg_in.push(reg.to_string());
                }
            }
            with_regs.reg_in.sort();
        }
        // An argument in EAX, EBX, ESI, EDI or EBP (whole-program optimized
        // code, #202), from the data flow: nv-call passes only ECX and EDX.
        if f.args.as_ref().is_some_and(|args| {
            args.keys()
                .any(|k| matches!(k.as_str(), "eax" | "ebx" | "esi" | "edi" | "ebp"))
        }) {
            skip(Skip::RegisterInput);
            continue;
        }
        // A body without a RET (a thunk that tail-jumps): its stack cleanup,
        // and so its convention, is unknown. Not a translation issue.
        if with_regs.ret_pop < 0 {
            skip(Skip::NoRet);
            continue;
        }
        let cc = match convention(&sig.params, &with_regs) {
            Ok(cc) => cc,
            Err(why) => {
                skip(Skip::ConventionMismatch);
                out.convention_mismatches.push((addr, why));
                continue;
            }
        };
        let mut bufs = Vec::new();
        let mut as_pointer: Vec<usize> = Vec::new();
        if pointer_tier {
            let Some(args) = &f.args else {
                skip(Skip::NoDataFlow);
                continue;
            };
            if f.untracked_loads != 0 || f.untracked_stores != 0 {
                skip(Skip::UntrackedPointer);
                continue;
            }
            let names = inputs_for(cc, &sig.params);
            let mut why = None;
            for (k, (t, name)) in sig.params.iter().zip(&names).enumerate() {
                let Some(a) = args.get(name) else {
                    if *t == Ty::Ptr {
                        bufs.push(Buf::from_reads(k, &[]));
                    }
                    continue;
                };
                // A number the exe computes with, passes on or stores: no buffer.
                if *t != Ty::Ptr && !a.dereferenced() {
                    continue;
                }
                // A word the Rust types as `u32`/`i32` but the exe dereferences is
                // a pointer (out-parameters are often typed so): it gets a buffer
                // and is listed as `ptr`, since the call passes a word either way.
                // A narrower scalar can't hold an address.
                if !matches!(t, Ty::Ptr | Ty::U32 | Ty::I32) {
                    why = Some(Skip::ScalarUsedAsPointer);
                    break;
                }
                if *t != Ty::Ptr {
                    as_pointer.push(k);
                }
                why = if a.unknown {
                    Some(Skip::PointerOffsetUnknown)
                } else if !a.chases.is_empty() {
                    Some(Skip::PointerChase)
                } else if !a.passed.is_empty() {
                    Some(Skip::PointerPassed)
                } else if a.escapes {
                    Some(Skip::PointerEscapes)
                } else {
                    None
                };
                if why.is_some() {
                    break;
                }
                // Fields read and fields written: fills put known bytes under
                // every store, so a missing, partial or misplaced one shows.
                bufs.push(Buf::from_reads(
                    k,
                    &[a.reads.as_slice(), &a.writes].concat(),
                ));
            }
            // An input the exe dereferences that is none of the parameters
            // (the convention check normally rules these out first).
            if why.is_none()
                && args
                    .iter()
                    .any(|(name, a)| !names.contains(name) && a.dereferenced())
            {
                why = Some(Skip::UnmatchedInput);
            }
            // A function in the store tier whose stores the data flow does not
            // put on an argument (all must be: `untracked_stores` is 0 above).
            if why.is_none() && stores_through_args && !args.values().any(|a| !a.writes.is_empty())
            {
                why = Some(Skip::UntrackedPointer);
            }
            if let Some(w) = why {
                skip(w);
                continue;
            }
        }
        out.pure.push(Pure {
            addr,
            cc,
            params: sig
                .params
                .iter()
                .enumerate()
                .map(|(k, t)| if as_pointer.contains(&k) { Ty::Ptr } else { *t })
                .collect(),
            ret: sig.ret,
            unit: unit.to_string_lossy().replace('\\', "/"),
            consts: {
                let mut k: Vec<String> = Vec::new();
                for a in std::iter::once(&addr).chain(callees.iter()) {
                    for t in facts.get(a).map(|g| g.consts.as_slice()).unwrap_or(&[]) {
                        if !k.contains(t) && k.len() < 48 {
                            k.push(t.clone());
                        }
                    }
                }
                k
            },
            computed_reads: std::iter::once(&addr).chain(callees.iter()).any(|a| {
                facts
                    .get(a)
                    .is_none_or(|g| g.args.is_none() || g.untracked_loads != 0)
            }),
            callees: callees.into_iter().collect(),
            bufs,
        });
    }
    out.skipped = skipped;
    out
}

pub const HEADER: &str = "\
# Pure translations for the automatic oracle check (research/nv-oracle/nv-pure, #182).
# Generated by `nv-pure list`: translated and complete in the engine crates, pure in
# FalloutNV.exe 1.4.0.525 (NvPurity.java, transitively), scalar arguments and result,
# or pointer arguments read (phase 1, #193) and stored through (phase 2: the
# function's only stores; the result may be `void`, its buffers are then its
# result) at known offsets. `u32`/`i32` parameters the exe dereferences are listed
# as `ptr`. Addresses, signatures, the constants each function compares with (used
# as boundary inputs) and, per pointer parameter, `param:size:bias:off.size;...`
# (the buffer and the fields the exe reads or stores), and `computed_reads` (1 when
# the exe may read an address computed from the inputs, such as a table index).
# Regenerate after translations land.
address\tcc\tparams\tret\tunit\tcallees\tconsts\tbuffers\tcomputed_reads
";

pub fn to_tsv(list: &[Pure]) -> String {
    let mut s = String::from(HEADER);
    for p in list {
        let params: Vec<&str> = p.params.iter().map(|t| t.name()).collect();
        let callees: Vec<String> = p.callees.iter().map(|c| format!("{c:08x}")).collect();
        let _ = writeln!(
            s,
            "{:08x}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            p.addr,
            p.cc,
            params.join(","),
            p.ret.name(),
            p.unit,
            callees.join(","),
            p.consts.join(","),
            p.bufs
                .iter()
                .map(|b| {
                    let f: Vec<String> = b.fields.iter().map(|(o, z)| format!("{o}.{z}")).collect();
                    format!("{}:{}:{}:{}", b.param, b.size, b.bias, f.join(";"))
                })
                .collect::<Vec<_>>()
                .join("|"),
            u8::from(p.computed_reads)
        );
    }
    s
}

pub fn from_tsv(text: &str) -> Result<Vec<Pure>, String> {
    let mut out = Vec::new();
    for (n, l) in text.lines().enumerate() {
        if l.starts_with('#') || l.starts_with("address\t") || l.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        let bad = || format!("pure list line {}: malformed", n + 1);
        if c.len() < 6 {
            return Err(bad());
        }
        let cc = match c[1] {
            "cdecl" => "cdecl",
            "stdcall" => "stdcall",
            "thiscall" => "thiscall",
            "fastcall" => "fastcall",
            _ => return Err(bad()),
        };
        let params = c[2]
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| Ty::from_name(s).ok_or_else(bad))
            .collect::<Result<Vec<_>, _>>()?;
        out.push(Pure {
            addr: u32::from_str_radix(c[0], 16).map_err(|_| bad())?,
            cc,
            params,
            ret: Ty::from_name(c[3]).ok_or_else(bad)?,
            unit: c[4].to_string(),
            callees: c[5]
                .split(',')
                .filter(|s| !s.is_empty())
                .filter_map(|s| u32::from_str_radix(s, 16).ok())
                .collect(),
            consts: c
                .get(6)
                .map(|v| {
                    v.split(',')
                        .filter(|s| !s.is_empty())
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default(),
            // Missing (a list from before the column): the exe may read computed
            // addresses, so a stray engine read stays inconclusive as it was.
            computed_reads: c.get(8).is_none_or(|v| *v != "0"),
            bufs: match c.get(7).filter(|v| !v.is_empty()) {
                None => Vec::new(),
                Some(v) => v
                    .split('|')
                    .map(|b| {
                        let mut it = b.splitn(4, ':');
                        let num = |x: Option<&str>| x.and_then(|x| x.parse::<u32>().ok());
                        let (param, size, bias) = (num(it.next()), num(it.next()), num(it.next()));
                        let fields = it
                            .next()
                            .unwrap_or("")
                            .split(';')
                            .filter(|f| !f.is_empty())
                            .map(|f| {
                                let (o, z) = f.split_once('.')?;
                                Some((o.parse().ok()?, z.parse().ok()?))
                            })
                            .collect::<Option<Vec<_>>>();
                        Some(Buf {
                            param: param? as usize,
                            size: size?,
                            bias: bias?,
                            fields: fields?,
                        })
                    })
                    .collect::<Option<Vec<_>>>()
                    .ok_or_else(bad)?,
            },
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts(ret_pop: i64, reg: &[&str], stack_args: i64) -> Facts {
        Facts {
            ret_pop,
            calls: vec![],
            indirect: 0,
            imports: 0,
            w_global: 0,
            w_pointer: 0,
            w_implicit: 0,
            r_pointer: 0,
            fs: 0,
            reg_in: reg.iter().map(|s| s.to_string()).collect(),
            stack_args,
            consts: vec![],
            args: None,
            untracked_loads: 0,
            untracked_stores: 0,
            hidden_in: 0,
        }
    }

    #[test]
    fn conventions() {
        assert_eq!(
            convention(&[Ty::F32, Ty::F32], &facts(0, &[], 8)),
            Ok("cdecl")
        );
        assert_eq!(
            convention(&[Ty::F32, Ty::F32], &facts(8, &[], 8)),
            Ok("stdcall")
        );
        assert_eq!(
            convention(&[Ty::U32, Ty::I32], &facts(4, &["ECX"], 4)),
            Ok("thiscall")
        );
        assert_eq!(
            convention(&[Ty::U32, Ty::U32, Ty::F64], &facts(8, &["ECX", "EDX"], 8)),
            Ok("fastcall")
        );
        assert!(
            convention(&[Ty::F32], &facts(8, &[], 8)).is_err(),
            "pops more than it takes"
        );
        assert!(
            convention(&[Ty::F32], &facts(0, &[], 12)).is_err(),
            "reads more than it is given"
        );
        assert!(
            convention(&[Ty::U32, Ty::U32], &facts(0, &["ECX"], 4)).is_err(),
            "thiscall leaves a stack word"
        );
        // No register read, one (two) words fewer popped: `this` (and EDX) ignored.
        assert_eq!(
            convention(&[Ty::Ptr, Ty::U32], &facts(4, &[], -1)),
            Ok("thiscall")
        );
        assert_eq!(
            convention(&[Ty::Ptr, Ty::U32, Ty::U32], &facts(4, &[], -1)),
            Ok("fastcall")
        );
        assert!(
            convention(&[Ty::F32, Ty::U32], &facts(4, &[], -1)).is_err(),
            "a float can't be `this`"
        );
    }

    #[test]
    fn inputs_and_buffers() {
        assert_eq!(
            inputs_for("thiscall", &[Ty::Ptr, Ty::U32, Ty::F64, Ty::Ptr]),
            vec!["ecx", "s0", "s4", "s12"]
        );
        assert_eq!(inputs_for("cdecl", &[Ty::Ptr, Ty::U32]), vec!["s0", "s4"]);
        assert_eq!(
            inputs_for("fastcall", &[Ty::F32, Ty::U32, Ty::Ptr, Ty::U32]),
            vec!["s0", "ecx", "edx", "s4"]
        );
        let b = Buf::from_reads(1, &[(8, 4), (-4, 4), (8, 4)]);
        assert_eq!(
            (b.bias, b.fields.len()),
            (16, 2),
            "negative offset gets room, duplicates merge"
        );
        assert!(b.size >= 16 + 12 && b.size.is_multiple_of(16));
        let e = Buf::from_reads(0, &[]);
        assert_eq!((e.bias, e.size), (0, 32));
    }

    #[test]
    fn tsv_round_trip() {
        let p = Pure {
            addr: 0x00a0_1a20,
            cc: "stdcall",
            params: vec![Ty::F32, Ty::U16],
            ret: Ty::Bool,
            unit: "crates/engine/src/units/x.rs".into(),
            callees: vec![0x0040_1000],
            consts: vec!["i26".into(), "d3ff0000000000000".into()],
            bufs: vec![Buf::from_reads(0, &[(-4, 4), (8, 2)])],
            computed_reads: false,
        };
        let back = from_tsv(&to_tsv(std::slice::from_ref(&p))).unwrap();
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].addr, p.addr);
        assert_eq!(back[0].params, p.params);
        assert_eq!(back[0].ret, p.ret);
        assert_eq!(back[0].callees, p.callees);
        assert_eq!(back[0].consts, p.consts);
        assert_eq!(back[0].bufs, p.bufs);
        assert!(!back[0].computed_reads);
        let reads = Pure {
            computed_reads: true,
            ..p
        };
        let text = to_tsv(&[reads]);
        assert!(from_tsv(&text).unwrap()[0].computed_reads);
        // A list from before the column: computed reads assumed (inconclusive, as before).
        let line = text.lines().last().unwrap();
        let old = &line[..line.rfind('\t').unwrap()];
        assert!(from_tsv(old).unwrap()[0].computed_reads);
    }
}
