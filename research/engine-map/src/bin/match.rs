//! PC-to-Xbox function matching for the engine map (policy: ADR-0002).
//!
//! ```text
//! match <pc-dir> <xb-dir> <out-dir>
//! ```
//!
//! `<pc-dir>` is the output of `research/ghidra/NvEngineMap.java`
//! (`functions.tsv`, `vtables.tsv`), `<xb-dir>` the output of the `xbox`
//! binary. Writes to `<out-dir>`:
//!
//! - `matches.tsv`: `pc, xbox, tier, name`, one row per matched PC function.
//! - `names.csv`: the same names for `NvImportNameMap.java`
//!   (`address,name,source,pin`, sanitized and de-duplicated).
//! - `report.txt`: counts per tier, and the hold-out check of the call-graph
//!   tier.
//!
//! Tiers, strongest first:
//!
//! - `vt`: the PC RTTI type name equals the Xbox vftable's class (compared
//!   as mangled text, so no demangler decides anything), both tables have the
//!   same number of slots, and slot k of one is paired with slot k of the
//!   other. Primary tables are paired with primary tables; a class's
//!   secondary tables are paired in address order when both sides have the
//!   same number of them. A pair is kept only if the PC function has exactly
//!   one Xbox candidate over all tables and that Xbox function exactly one PC
//!   candidate, which drops stubs that identical-code folding shares between
//!   methods.
//! - `cg`: call-graph alignment. For every matched pair, the PC call list and
//!   the Xbox call list (in call-site order) are aligned on calls already
//!   matched (longest common subsequence). Between two such anchors (or an
//!   anchor and the end of the list), a gap of the same length on both sides
//!   pairs its calls position by position. A candidate is accepted if it is
//!   the only candidate for both functions and either came from a gap of
//!   length one or was proposed by two different matched callers. Repeats to
//!   a fixed point.
//!
//! The hold-out check seeds the call-graph tier with half of the `vt` pairs
//! (chosen by address parity of a hash) and counts how many of the other half
//! it rediscovers, and how many of those it gets wrong.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;

#[path = "../names.rs"]
mod names;
use names::{ppc_helper, sanitize};

type R<T> = Result<T, Box<dyn std::error::Error>>;

struct PcFn {
    calls: Vec<u32>,
    strings: Vec<String>,
}

struct XbFn {
    name: String,
    calls: Vec<u32>,
    strings: Vec<String>,
}

fn hex(s: &str) -> u32 {
    u32::from_str_radix(s, 16).unwrap_or_else(|_| panic!("bad hex {s:?}"))
}

fn list(s: &str) -> Vec<u32> {
    if s.is_empty() {
        Vec::new()
    } else {
        s.split(',').map(hex).collect()
    }
}

fn rows(path: &Path) -> R<Vec<Vec<String>>> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(text
        .lines()
        .skip(1)
        .map(|l| l.split('\t').map(str::to_string).collect())
        .collect())
}

/// Pairs accepted so far, both directions.
#[derive(Default, Clone)]
struct Pairs {
    pc: HashMap<u32, u32>,
    xb: HashMap<u32, u32>,
}

impl Pairs {
    fn add(&mut self, p: u32, x: u32) {
        self.pc.insert(p, x);
        self.xb.insert(x, p);
    }
}

/// Longest common subsequence of matched calls: (i, j) index pairs.
fn anchors(a: &[u32], b: &[u32], pairs: &Pairs) -> Vec<(usize, usize)> {
    let (n, m) = (a.len(), b.len());
    if n == 0 || m == 0 || n * m > 250_000 {
        return Vec::new();
    }
    let eq = |i: usize, j: usize| pairs.pc.get(&a[i]) == Some(&b[j]);
    let mut dp = vec![0u16; (n + 1) * (m + 1)];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[i * (m + 1) + j] = if eq(i, j) {
                dp[(i + 1) * (m + 1) + j + 1] + 1
            } else {
                dp[(i + 1) * (m + 1) + j].max(dp[i * (m + 1) + j + 1])
            };
        }
    }
    let (mut i, mut j, mut out) = (0, 0, Vec::new());
    while i < n && j < m {
        if eq(i, j) {
            out.push((i, j));
            i += 1;
            j += 1;
        } else if dp[(i + 1) * (m + 1) + j] >= dp[i * (m + 1) + j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}

/// Call-graph propagation from `seed`; returns the pairs it added.
fn propagate(seed: &Pairs, pc: &BTreeMap<u32, PcFn>, xb: &BTreeMap<u32, XbFn>) -> Vec<(u32, u32)> {
    let mut pairs = seed.clone();
    let mut added = Vec::new();
    loop {
        // candidate pc -> xb -> (parents, strong)
        let mut cand: HashMap<u32, HashMap<u32, (HashSet<u32>, bool)>> = HashMap::new();
        let mut back: HashMap<u32, HashSet<u32>> = HashMap::new();
        for (&p, &x) in &pairs.pc {
            let (Some(pf), Some(xf)) = (pc.get(&p), xb.get(&x)) else {
                continue;
            };
            let a = &pf.calls;
            let b = &xf.calls;
            let mut segs = Vec::new();
            let (mut pi, mut pj) = (0usize, 0usize);
            for (i, j) in anchors(a, b, &pairs) {
                segs.push((pi, i, pj, j));
                pi = i + 1;
                pj = j + 1;
            }
            segs.push((pi, a.len(), pj, b.len()));
            for (i0, i1, j0, j1) in segs {
                let len = i1 - i0;
                if len == 0 || len != j1 - j0 {
                    continue;
                }
                for k in 0..len {
                    let (cp, cx) = (a[i0 + k], b[j0 + k]);
                    if pairs.pc.contains_key(&cp) || pairs.xb.contains_key(&cx) {
                        continue;
                    }
                    let e = cand.entry(cp).or_default().entry(cx).or_default();
                    e.0.insert(p);
                    e.1 |= len == 1;
                    back.entry(cx).or_default().insert(cp);
                }
            }
        }
        let mut round = Vec::new();
        for (&cp, xs) in &cand {
            if xs.len() != 1 {
                continue;
            }
            let (&cx, (parents, strong)) = xs.iter().next().unwrap();
            if back[&cx].len() != 1 {
                continue;
            }
            if *strong || parents.len() >= 2 {
                round.push((cp, cx));
            }
        }
        if round.is_empty() {
            round = signature_pairs(&pairs, pc, xb);
        }
        if round.is_empty() {
            break;
        }
        round.sort();
        for &(p, x) in &round {
            pairs.add(p, x);
        }
        added.extend(round);
    }
    added
}

fn main() -> R<()> {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage: match <pc-dir> <xb-dir> <out-dir>".into());
    }
    let (pcd, xbd, out) = (Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]));
    fs::create_dir_all(out)?;

    let mut pc: BTreeMap<u32, PcFn> = BTreeMap::new();
    for r in rows(&pcd.join("functions.tsv"))? {
        pc.insert(
            hex(&r[0]),
            PcFn {
                calls: list(&r[6]),
                strings: split_strings(&r[8]),
            },
        );
    }
    let mut xb: BTreeMap<u32, XbFn> = BTreeMap::new();
    for r in rows(&xbd.join("xb_funcs.tsv"))? {
        xb.insert(
            hex(&r[0]),
            XbFn {
                name: r[4].clone(),
                calls: list(r.get(6).map(String::as_str).unwrap_or("")),
                strings: split_strings(r.get(7).map(String::as_str).unwrap_or("")),
            },
        );
    }
    let helpers: HashSet<u32> = xb
        .iter()
        .filter(|(_, f)| ppc_helper(&f.name))
        .map(|(&a, _)| a)
        .collect();
    for f in xb.values_mut() {
        f.calls.retain(|c| !helpers.contains(c));
    }

    // vt tier.
    let mut pc_vt: BTreeMap<String, Vec<(u32, Vec<u32>)>> = BTreeMap::new();
    for r in rows(&pcd.join("vtables.tsv"))? {
        let key = r[2].get(4..).unwrap_or("").to_string();
        let off: u32 = r[3].parse()?;
        pc_vt.entry(key).or_default().push((off, list(&r[4])));
    }
    type XbTables = (Option<Vec<u32>>, Vec<(u32, Vec<u32>)>);
    let mut xb_vt: BTreeMap<String, XbTables> = BTreeMap::new();
    for r in rows(&xbd.join("xb_vtables.tsv"))? {
        let m = &r[1];
        let Some(body) = m.strip_prefix("??_7") else {
            continue;
        };
        let Some(k) = body.find("@@6B") else { continue };
        let key = body[..k + 2].to_string();
        let slots = list(&r[3]);
        let e = xb_vt.entry(key).or_default();
        if &body[k + 2..] == "6B@" {
            e.0 = Some(slots);
        } else {
            e.1.push((hex(&r[0]), slots));
        }
    }
    let mut cand_pc: HashMap<u32, BTreeSet<u32>> = HashMap::new();
    let mut cand_xb: HashMap<u32, BTreeSet<u32>> = HashMap::new();
    let (mut tables_paired, mut tables_len_differ, mut classes_both) = (0, 0, 0);
    for (key, tabs) in &mut pc_vt {
        let Some((xprim, xsec)) = xb_vt.get(key) else {
            continue;
        };
        classes_both += 1;
        tabs.sort();
        let mut pairs: Vec<(&Vec<u32>, &Vec<u32>)> = Vec::new();
        let prim: Vec<_> = tabs.iter().filter(|t| t.0 == 0).collect();
        let sec: Vec<_> = tabs.iter().filter(|t| t.0 != 0).collect();
        if let (Some(xp), [p]) = (xprim, prim.as_slice()) {
            pairs.push((&p.1, xp));
        }
        let mut xsec = xsec.clone();
        xsec.sort();
        if sec.len() == xsec.len() {
            for (p, x) in sec.iter().zip(xsec.iter()) {
                pairs.push((&p.1, &x.1));
            }
        }
        for (ps, xs) in pairs {
            if ps.len() != xs.len() {
                tables_len_differ += 1;
                continue;
            }
            tables_paired += 1;
            for (&p, &x) in ps.iter().zip(xs.iter()) {
                if !pc.contains_key(&p) {
                    continue;
                }
                cand_pc.entry(p).or_default().insert(x);
                cand_xb.entry(x).or_default().insert(p);
            }
        }
    }
    let mut vt = Pairs::default();
    for (&p, xs) in &cand_pc {
        if xs.len() == 1 {
            let x = *xs.iter().next().unwrap();
            if cand_xb[&x].len() == 1 {
                vt.add(p, x);
            }
        }
    }
    let vt_dropped = cand_pc.len() - vt.pc.len();

    // str tier, checked against vt where both name a function.
    let st = string_pairs(&pc, &xb);
    let (mut st_agree, mut st_conflict) = (0, 0);
    for (p, x) in &st.pc {
        let vp = vt.pc.get(p);
        let vx = vt.xb.get(x);
        if vp == Some(x) {
            st_agree += 1;
        } else if vp.is_some() || vx.is_some() {
            st_conflict += 1;
        }
    }
    let mut base = vt.clone();
    for (&p, &x) in &st.pc {
        if !base.pc.contains_key(&p) && !base.xb.contains_key(&x) {
            base.add(p, x);
        }
    }

    // Hold-out check of the cg tier.
    let half = |p: u32| (p.wrapping_mul(2_654_435_761) >> 16) & 1 == 0;
    let mut seed = Pairs::default();
    for (&p, &x) in &vt.pc {
        if half(p) {
            seed.add(p, x);
        }
    }
    let held = propagate(&seed, &pc, &xb);
    let (mut ho_total, mut ho_right, mut ho_wrong, mut ho_new) = (0, 0, 0, 0);
    for (p, x) in &held {
        match vt.pc.get(p) {
            Some(v) if v == x => {
                ho_total += 1;
                ho_right += 1
            }
            Some(_) => {
                ho_total += 1;
                ho_wrong += 1
            }
            None => ho_new += 1,
        }
    }
    let ho_wrong_xb = held
        .iter()
        .filter(|(p, x)| !vt.pc.contains_key(p) && vt.xb.contains_key(x))
        .count();
    let ho_hidden = vt.pc.len() - seed.pc.len();

    // Full run.
    let cg = propagate(&base, &pc, &xb);
    let mut all: BTreeMap<u32, (u32, &str)> = BTreeMap::new();
    for (&p, &x) in &vt.pc {
        all.insert(p, (x, "vt"));
    }
    for (&p, &x) in &base.pc {
        all.entry(p).or_insert((x, "str"));
    }
    for &(p, x) in &cg {
        all.insert(p, (x, "cg"));
    }

    // Names: sanitized, repeated names get _ovN in address order.
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut f = fs::File::create(out.join("matches.tsv"))?;
    let mut csv = fs::File::create(out.join("names.csv"))?;
    writeln!(f, "pc\txbox\ttier\tname")?;
    writeln!(csv, "address,name,source,pin")?;
    let mut per_tier: BTreeMap<&str, usize> = BTreeMap::new();
    for (&p, &(x, tier)) in &all {
        let mut n = sanitize(&xb[&x].name);
        let k = seen.entry(n.clone()).or_insert(0);
        *k += 1;
        if *k > 1 {
            n = format!("{n}_ov{k}");
        }
        *per_tier.entry(tier).or_default() += 1;
        writeln!(f, "{p:08x}\t{x:08x}\t{tier}\t{n}")?;
        writeln!(csv, "{p:08x},{n},xbox_pdb.{tier},xbox-memdebug-036BA387")?;
    }

    let report = format!(
        "pc functions {}\nxbox functions {}\nppc helpers dropped from call lists {}\n\
         classes with vtables on both sides {classes_both}\ntables paired {tables_paired}\n\
         tables with different slot counts {tables_len_differ}\n\
         vt candidates dropped as not one-to-one {vt_dropped}\n\
         str pairs {}, of which also vt and agreeing {st_agree}, conflicting with vt {st_conflict}
\n         matched per tier {per_tier:?}\nmatched total {}\n\n\
         hold-out: seeded with {} of {} vt pairs; cg rediscovered {ho_total} of the {ho_hidden} hidden \
         ({ho_right} right, {ho_wrong} wrong); {ho_new} pairs outside vt; \
         {ho_wrong_xb} of those used an Xbox function that vt gives to another PC function\n",
        pc.len(),
        xb.len(),
        helpers.len(),
        st.pc.len(),
        all.len(),
        seed.pc.len(),
        vt.pc.len(),
    );
    fs::write(out.join("report.txt"), &report)?;
    print!("{report}");
    Ok(())
}

fn split_strings(s: &str) -> Vec<String> {
    if s.is_empty() {
        Vec::new()
    } else {
        s.split('|').map(str::to_string).collect()
    }
}

/// `str` tier: a string referenced by exactly one PC function and exactly
/// one Xbox function pairs the two. Pairs must be one-to-one over all such
/// strings. Strings shorter than 8 characters are ignored (too likely to
/// recur by chance, e.g. "%s%s").
fn string_pairs(pc: &BTreeMap<u32, PcFn>, xb: &BTreeMap<u32, XbFn>) -> Pairs {
    fn owners<'a>(
        it: impl Iterator<Item = (u32, &'a Vec<String>)>,
    ) -> HashMap<&'a str, BTreeSet<u32>> {
        let mut m: HashMap<&str, BTreeSet<u32>> = HashMap::new();
        for (a, ss) in it {
            for s in ss {
                if s.len() >= 8 {
                    m.entry(s.as_str()).or_default().insert(a);
                }
            }
        }
        m
    }
    let po = owners(pc.iter().map(|(&a, f)| (a, &f.strings)));
    let xo = owners(xb.iter().map(|(&a, f)| (a, &f.strings)));
    let mut cp: HashMap<u32, BTreeSet<u32>> = HashMap::new();
    let mut cx: HashMap<u32, BTreeSet<u32>> = HashMap::new();
    for (s, ps) in &po {
        let Some(xs) = xo.get(s) else { continue };
        if ps.len() == 1 && xs.len() == 1 {
            let (p, x) = (*ps.iter().next().unwrap(), *xs.iter().next().unwrap());
            cp.entry(p).or_default().insert(x);
            cx.entry(x).or_default().insert(p);
        }
    }
    let mut out = Pairs::default();
    for (&p, xs) in &cp {
        if xs.len() == 1 {
            let x = *xs.iter().next().unwrap();
            if cx[&x].len() == 1 {
                out.add(p, x);
            }
        }
    }
    out
}

/// Signature rule: an unmatched function's signature is the sorted set of
/// matched functions it calls and the sorted set of matched functions that
/// call it, PC side translated to Xbox addresses. If exactly one unmatched
/// PC function and exactly one unmatched Xbox function have a signature,
/// and it has at least two members, they are paired.
fn signature_pairs(
    pairs: &Pairs,
    pc: &BTreeMap<u32, PcFn>,
    xb: &BTreeMap<u32, XbFn>,
) -> Vec<(u32, u32)> {
    type Sig = (Vec<u32>, Vec<u32>);
    fn sigs<F>(
        calls: &BTreeMap<u32, Vec<u32>>,
        open: impl Fn(u32) -> bool,
        map: F,
    ) -> HashMap<Sig, Vec<u32>>
    where
        F: Fn(u32) -> Option<u32>,
    {
        let mut callers: HashMap<u32, BTreeSet<u32>> = HashMap::new();
        for (&a, cs) in calls {
            if let Some(ma) = map(a) {
                for &c in cs {
                    callers.entry(c).or_default().insert(ma);
                }
            }
        }
        let mut out: HashMap<Sig, Vec<u32>> = HashMap::new();
        for (&a, cs) in calls {
            if !open(a) {
                continue;
            }
            let callees: BTreeSet<u32> = cs.iter().filter_map(|&c| map(c)).collect();
            let up = callers.remove(&a).unwrap_or_default();
            if callees.len() + up.len() < 2 {
                continue;
            }
            out.entry((callees.into_iter().collect(), up.into_iter().collect()))
                .or_default()
                .push(a);
        }
        out
    }
    let pc_calls: BTreeMap<u32, Vec<u32>> = pc.iter().map(|(&a, f)| (a, f.calls.clone())).collect();
    let xb_calls: BTreeMap<u32, Vec<u32>> = xb.iter().map(|(&a, f)| (a, f.calls.clone())).collect();
    let ps = sigs(
        &pc_calls,
        |a| !pairs.pc.contains_key(&a),
        |a| pairs.pc.get(&a).copied(),
    );
    let xs = sigs(
        &xb_calls,
        |a| !pairs.xb.contains_key(&a),
        |a| pairs.xb.contains_key(&a).then_some(a),
    );
    let mut out = Vec::new();
    for (sig, p) in &ps {
        if let (1, Some(x)) = (p.len(), xs.get(sig)) {
            if x.len() == 1 {
                out.push((p[0], x[0]));
            }
        }
    }
    out.sort();
    out
}
