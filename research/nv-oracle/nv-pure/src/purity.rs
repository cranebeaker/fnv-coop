//! The exe-side purity facts (`research/ghidra/NvPurity.java`) and the
//! transitive closure over the call graph.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct Facts {
    pub ret_pop: i64,
    pub calls: Vec<u32>,
    pub indirect: u32,
    pub imports: u32,
    pub w_global: u32,
    pub w_pointer: u32,
    pub w_implicit: u32,
    pub r_pointer: u32,
    pub fs: u32,
    /// `ECX`/`EDX` read before written in the entry block.
    pub reg_in: Vec<String>,
    /// Stack argument bytes the body reads (lower bound); -1 unknown.
    pub stack_args: i64,
    /// Constants the body compares and computes with, as input tokens:
    /// `i<decimal>`, `f<8 hex>` (float bits), `d<16 hex>` (double bits).
    /// Empty when the table predates the `consts_*` columns.
    pub consts: Vec<String>,
    /// What goes through each input used as a pointer (`arg_access`), by
    /// input name (`ecx`, `edx`, `s<n>`). `None` when the data flow was not
    /// run for the function (or the table predates the column).
    pub args: Option<BTreeMap<String, Access>>,
    /// LOADs / STOREs whose address no input explains; -1 unknown.
    pub untracked_loads: i64,
    pub untracked_stores: i64,
    /// Instructions whose result no input determines (RDTSC, CPUID, ...);
    /// 0 when the table has no such column.
    pub hidden_in: u32,
}

/// The accesses through one pointer input (NvPurity `arg_access`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Access {
    /// (byte offset from the pointer, size) of every load.
    pub reads: Vec<(i64, u32)>,
    /// (byte offset, size) of every store.
    pub writes: Vec<(i64, u32)>,
    /// A load or store at an offset the data flow can't pin down.
    pub unknown: bool,
    /// Offsets of pointer fields that are themselves dereferenced.
    pub chases: Vec<i64>,
    /// Callees the pointer (or one derived from it) is passed to; "?" for
    /// an indirect call.
    pub passed: Vec<String>,
    /// The pointer itself is stored somewhere.
    pub escapes: bool,
    /// The input's value is used at all (recorded for the registers no
    /// standard convention passes arguments in: eax, ebx, esi, edi, ebp).
    pub used: bool,
}

impl Access {
    /// Memory is read or written through the input (not only passed on or
    /// compared).
    pub fn dereferenced(&self) -> bool {
        self.unknown || !self.reads.is_empty() || !self.writes.is_empty() || !self.chases.is_empty()
    }
}

/// Parses one `arg_access` cell: `ecx=r4.4,w8.2|s0=r0.4,c12`.
pub fn parse_access(cell: &str) -> Option<BTreeMap<String, Access>> {
    if cell == "-" {
        return None;
    }
    let mut out = BTreeMap::new();
    for part in cell.split('|').filter(|p| !p.is_empty()) {
        let (name, items) = part.split_once('=')?;
        let mut a = Access::default();
        for it in items.split(',').filter(|i| !i.is_empty()) {
            let (kind, rest) = it.split_at(1);
            match kind {
                "r" | "w" if rest == "?" => a.unknown = true,
                "r" | "w" => {
                    let (off, size) = rest.split_once('.')?;
                    let v = (off.parse().ok()?, size.parse().ok()?);
                    if kind == "r" {
                        a.reads.push(v);
                    } else {
                        a.writes.push(v);
                    }
                }
                "c" => a.chases.push(rest.parse().ok()?),
                "p" => a.passed.push(rest.to_string()),
                "e" => a.escapes = true,
                "u" => a.used = true,
                _ => return None,
            }
        }
        out.insert(name.to_string(), a);
    }
    Some(out)
}

impl Facts {
    /// Nothing but its own frame: no stores outside it, no unknown or
    /// imported calls, no TLS/SEH.
    pub fn locally_pure(&self) -> bool {
        self.indirect == 0
            && self.imports == 0
            && self.w_global == 0
            && self.w_pointer == 0
            && self.w_implicit == 0
            && self.fs == 0
    }
}

fn hex(s: &str) -> Option<u32> {
    u32::from_str_radix(s.trim(), 16).ok()
}

/// Parses NvPurity's TSV (header row first).
pub fn parse(text: &str) -> Result<BTreeMap<u32, Facts>, String> {
    let mut lines = text.lines();
    let header: Vec<&str> = lines
        .next()
        .ok_or("empty purity table")?
        .split('\t')
        .collect();
    let col = |name: &str| {
        header
            .iter()
            .position(|h| *h == name)
            .ok_or_else(|| format!("purity table has no column {name} (rerun NvPurity.java)"))
    };
    let (c_entry, c_ret, c_calls, c_tails) = (
        col("entry")?,
        col("ret_pop")?,
        col("calls")?,
        col("tail_calls")?,
    );
    let (c_ind, c_imp, c_wg, c_wp, c_wi) = (
        col("indirect")?,
        col("imports")?,
        col("w_global")?,
        col("w_pointer")?,
        col("w_implicit")?,
    );
    let (c_rp, c_fs, c_reg, c_sa) = (
        col("r_pointer")?,
        col("fs")?,
        col("reg_in")?,
        col("stack_args")?,
    );
    let c_ci = header.iter().position(|h| *h == "consts_int");
    let c_cf = header.iter().position(|h| *h == "consts_float");
    let c_aa = header.iter().position(|h| *h == "arg_access");
    let c_ul = header.iter().position(|h| *h == "untracked_loads");
    let c_us = header.iter().position(|h| *h == "untracked_stores");
    let c_hi = header.iter().position(|h| *h == "hidden_in");
    let mut out = BTreeMap::new();
    for (n, l) in lines.enumerate() {
        let c: Vec<&str> = l.split('\t').collect();
        let bad = || format!("purity table line {}: malformed", n + 2);
        let num = |i: usize| {
            c.get(i)
                .and_then(|v| v.trim().parse::<i64>().ok())
                .ok_or_else(bad)
        };
        let list = |i: usize| -> Vec<u32> {
            c.get(i)
                .map(|v| {
                    v.split(',')
                        .filter(|s| !s.is_empty())
                        .filter_map(hex)
                        .collect()
                })
                .unwrap_or_default()
        };
        let entry = c.get(c_entry).and_then(|v| hex(v)).ok_or_else(bad)?;
        let mut calls = list(c_calls);
        calls.extend(list(c_tails));
        out.insert(
            entry,
            Facts {
                ret_pop: num(c_ret)?,
                calls,
                indirect: num(c_ind)? as u32,
                imports: num(c_imp)? as u32,
                w_global: num(c_wg)? as u32,
                w_pointer: num(c_wp)? as u32,
                w_implicit: num(c_wi)? as u32,
                r_pointer: num(c_rp)? as u32,
                fs: num(c_fs)? as u32,
                reg_in: c
                    .get(c_reg)
                    .map(|v| {
                        v.split(',')
                            .filter(|s| !s.is_empty())
                            .map(str::to_string)
                            .collect()
                    })
                    .unwrap_or_default(),
                stack_args: num(c_sa)?,
                consts: {
                    let field = |i: Option<usize>| -> Vec<&str> {
                        i.and_then(|i| c.get(i))
                            .map(|v| v.split(',').filter(|s| !s.is_empty()).collect())
                            .unwrap_or_default()
                    };
                    let mut k: Vec<String> = field(c_ci).iter().map(|v| format!("i{v}")).collect();
                    k.extend(field(c_cf).iter().map(|v| v.to_string()));
                    k
                },
                args: match c_aa.and_then(|i| c.get(i)) {
                    Some(cell) if *cell != "-" => Some(parse_access(cell).ok_or_else(bad)?),
                    _ => None,
                },
                untracked_loads: c_ul.map(num).transpose()?.unwrap_or(-1),
                untracked_stores: c_us.map(num).transpose()?.unwrap_or(-1),
                hidden_in: c_hi.map(num).transpose()?.unwrap_or(0) as u32,
            },
        );
    }
    Ok(out)
}

/// Functions that are locally pure and whose callees are all (transitively)
/// pure; a callee missing from the table counts as impure. Greatest fixed
/// point, so mutually recursive pure functions stay pure.
pub fn transitively_pure(facts: &BTreeMap<u32, Facts>) -> BTreeSet<u32> {
    let mut pure: BTreeSet<u32> = facts
        .iter()
        .filter(|(_, f)| f.locally_pure())
        .map(|(a, _)| *a)
        .collect();
    loop {
        let drop: Vec<u32> = pure
            .iter()
            .copied()
            .filter(|a| facts[a].calls.iter().any(|c| !pure.contains(c)))
            .collect();
        if drop.is_empty() {
            return pure;
        }
        for a in drop {
            pure.remove(&a);
        }
    }
}

/// Every function reachable from `a` through calls (not including `a`).
pub fn callees(facts: &BTreeMap<u32, Facts>, a: u32) -> BTreeSet<u32> {
    let mut seen = BTreeSet::new();
    let mut stack: Vec<u32> = facts.get(&a).map(|f| f.calls.clone()).unwrap_or_default();
    while let Some(c) = stack.pop() {
        if c != a && seen.insert(c) {
            if let Some(f) = facts.get(&c) {
                stack.extend(f.calls.iter().copied());
            }
        }
    }
    seen
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: &str = "entry\tsize\tret_pop\tebp_frame\tcalls\ttail_calls\tindirect\timports\tw_stack\tw_global\tw_pointer\tw_implicit\tr_pointer\tfs\treg_in\tstack_args\n\
00401000\t10\t0\t1\t00401020\t\t0\t0\t1\t0\t0\t0\t0\t0\t\t4\n\
00401020\t10\t4\t1\t\t\t0\t0\t1\t0\t0\t0\t0\t0\tECX\t4\n\
00401040\t10\t0\t1\t00401060\t\t0\t0\t1\t0\t0\t0\t0\t0\t\t0\n\
00401060\t10\t0\t1\t\t\t0\t0\t1\t0\t1\t0\t0\t0\t\t0\n";

    #[test]
    fn access_cells() {
        let a = parse_access("ecx=c12,e,p00401000,r4.4,r8.2,w-4.1|s0=r?").unwrap();
        let e = &a["ecx"];
        assert_eq!(e.reads, vec![(4, 4), (8, 2)]);
        assert_eq!(e.writes, vec![(-4, 1)]);
        assert_eq!(e.chases, vec![12]);
        assert_eq!(e.passed, vec!["00401000".to_string()]);
        assert!(e.escapes && !e.unknown);
        assert!(a["s0"].unknown);
        let r = parse_access("eax=u|ecx=r0.4").unwrap();
        assert!(r["eax"].used && !r["eax"].dereferenced() && !r["ecx"].used);
        assert!(parse_access("-").is_none());
        assert_eq!(parse_access("").unwrap().len(), 0);
    }

    #[test]
    fn hidden_inputs() {
        assert_eq!(parse(T).unwrap()[&0x0040_1000].hidden_in, 0, "no column: 0");
        let t: String = T
            .lines()
            .enumerate()
            .map(|(i, l)| match i {
                0 => format!("{l}\thidden_in\n"),
                1 => format!("{l}\t2\n"),
                _ => format!("{l}\t0\n"),
            })
            .collect();
        assert_eq!(parse(&t).unwrap()[&0x0040_1000].hidden_in, 2);
    }

    #[test]
    fn closure() {
        let f = parse(T).unwrap();
        assert_eq!(f[&0x0040_1020].reg_in, vec!["ECX".to_string()]);
        let p = transitively_pure(&f);
        assert!(p.contains(&0x0040_1000) && p.contains(&0x0040_1020));
        assert!(!p.contains(&0x0040_1060), "stores through a pointer");
        assert!(!p.contains(&0x0040_1040), "calls an impure function");
        assert_eq!(
            callees(&f, 0x0040_1000).into_iter().collect::<Vec<_>>(),
            vec![0x0040_1020]
        );
    }
}
