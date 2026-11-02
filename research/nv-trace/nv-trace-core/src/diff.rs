//! The diff: where does the port's run first differ from the real game's?
//!
//! Both traces are split into threads, and the threads of one paired with the threads of the
//! other ([`pair_threads`]). For each pair of threads and each function, the k-th call in one
//! is compared with the k-th call in the other: its words by the list's word types and its
//! answer by the return kind. Pointers can't be compared as they are (two allocators, two
//! stacks), so each pointer word is replaced by "the n-th distinct pointer this function was
//! given on this thread" ([`Normalizer`]); a null stays null and an address inside the exe
//! image (globals, vtables, code) is compared as it is, since the port keeps the exe's
//! addresses for those.
//!
//! The report names the first call (in the real game's order, `seq`) whose values differ,
//! with the calls before it on both threads, and separately the first place the call counts
//! part (one side made a call the other never made).

use crate::format::{calls, Call, Trace};
use crate::list::{FnSpec, RetKind, WordKind};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;

/// Default range of the exe image (FalloutNV.exe 1.4.0.525 maps 0x00400000..0x0127xxxx; the
/// end is rounded up). Values inside it are code and data addresses, the same in both runs.
pub const IMAGE: (u32, u32) = (0x0040_0000, 0x0130_0000);

/// A word after normalisation.
#[derive(Clone, Copy, Debug, Eq)]
pub enum Norm {
    /// Compared as it is.
    Raw(u32),
    /// The n-th distinct (heap) pointer (from 1).
    Nth(u32),
    /// A stack address: the n-th distinct one (numbered apart from heap pointers) and, when
    /// the recorder knew it, its offset into the caller's frame. Two stack words are equal
    /// when both offsets are known and equal, else when their numbers are.
    Stack(u32, Option<i32>),
}

impl PartialEq for Norm {
    fn eq(&self, other: &Norm) -> bool {
        match (self, other) {
            (Norm::Raw(a), Norm::Raw(b)) | (Norm::Nth(a), Norm::Nth(b)) => a == b,
            (Norm::Stack(_, Some(a)), Norm::Stack(_, Some(b))) => a == b,
            (Norm::Stack(a, _), Norm::Stack(b, _)) => a == b,
            _ => false,
        }
    }
}

impl std::fmt::Display for Norm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Norm::Raw(v) => write!(f, "{v:08x}"),
            Norm::Nth(n) => write!(f, "ptr#{n}"),
            Norm::Stack(n, Some(o)) if *o < 0 => {
                write!(f, "stk#{n}(frame-{:#x})", o.unsigned_abs())
            }
            Norm::Stack(n, Some(o)) => write!(f, "stk#{n}(frame+{o:#x})"),
            Norm::Stack(n, None) => write!(f, "stk#{n}"),
        }
    }
}

/// "The n-th distinct pointer seen", per key (here: thread and function).
#[derive(Default)]
pub struct Normalizer {
    seen: HashMap<(u32, u32), HashMap<u32, u32>>,
    stack: HashMap<(u32, u32), HashMap<u32, u32>>,
    pub image: (u32, u32),
}

impl Normalizer {
    pub fn new(image: (u32, u32)) -> Self {
        Normalizer {
            seen: HashMap::new(),
            stack: HashMap::new(),
            image,
        }
    }

    /// A stack address, numbered apart from heap pointers.
    pub fn stack(&mut self, key: (u32, u32), v: u32, offset: Option<i32>) -> Norm {
        let map = self.stack.entry(key).or_default();
        let next = map.len() as u32 + 1;
        Norm::Stack(*map.entry(v).or_insert(next), offset)
    }

    pub fn ptr(&mut self, key: (u32, u32), v: u32) -> Norm {
        if v == 0 || (v >= self.image.0 && v < self.image.1) {
            return Norm::Raw(v);
        }
        let map = self.seen.entry(key).or_default();
        let next = map.len() as u32 + 1;
        Norm::Nth(*map.entry(v).or_insert(next))
    }
}

fn word(norm: &mut Normalizer, key: (u32, u32), kind: WordKind, v: u32) -> Norm {
    match kind {
        WordKind::Ptr => norm.ptr(key, v),
        WordKind::Word | WordKind::Float => Norm::Raw(v),
        WordKind::Byte => Norm::Raw(v & 0xff),
        WordKind::Half => Norm::Raw(v & 0xffff),
    }
}

/// A call's values after normalisation: its words and its answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape {
    pub words: Vec<Norm>,
    /// `None`: the call never returned while recorded.
    pub ret: Option<Vec<Norm>>,
}

fn st0_f32(bits: Option<u64>) -> Norm {
    match bits {
        Some(b) => Norm::Raw((f64::from_bits(b) as f32).to_bits()),
        None => Norm::Raw(0xdead_57a0),
    }
}

/// The normalised shape of `c` (with `spec` from the list, if the function is listed).
pub fn shape(norm: &mut Normalizer, spec: Option<&FnSpec>, c: &Call) -> Shape {
    let key = (c.tid, c.addr);
    let words = c
        .words
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            if let Some(sw) = c.stack.iter().find(|s| s.index as usize == i) {
                // An offset is comparable for the caller's own frame (depth 0) or when the
                // recorder can't tell the depth (the real game: offset from the caller's
                // stack pointer).
                let off = (sw.depth == 0 || sw.depth == crate::format::DEPTH_UNKNOWN)
                    .then_some(sw.offset);
                return norm.stack(key, v, off);
            }
            let kind = spec
                .and_then(|s| s.words.get(i).copied())
                .unwrap_or(WordKind::Word);
            word(norm, key, kind, v)
        })
        .collect();
    let kind = spec.map_or(RetKind::U32, |s| s.ret);
    let ret = c.ret.map(|r| match kind {
        RetKind::Void => vec![],
        RetKind::U8 => vec![Norm::Raw(r.eax & 0xff)],
        RetKind::U16 => vec![Norm::Raw(r.eax & 0xffff)],
        RetKind::U32 => vec![Norm::Raw(r.eax)],
        RetKind::Ptr => vec![norm.ptr(key, r.eax)],
        RetKind::I64 => vec![Norm::Raw(r.eax), Norm::Raw(r.edx)],
        RetKind::F32 => vec![st0_f32(r.st0)],
        RetKind::F64 => {
            let b = r.st0.unwrap_or(0);
            vec![Norm::Raw(b as u32), Norm::Raw((b >> 32) as u32)]
        }
    });
    Shape { words, ret }
}

/// Pairs the threads of `a` with the threads of `b`. `fixed` pairs come first; the rest are
/// matched in order of their first call: each thread of `a` takes the unpaired thread of `b`
/// whose first calls (by address) agree with its own for longest, at least one call.
pub fn pair_threads(
    a: &BTreeMap<u32, Vec<Call>>,
    b: &BTreeMap<u32, Vec<Call>>,
    fixed: &[(u32, u32)],
) -> Vec<(u32, u32)> {
    let mut pairs: Vec<(u32, u32)> = fixed
        .iter()
        .copied()
        .filter(|(x, y)| a.contains_key(x) && b.contains_key(y))
        .collect();
    let by_first = |m: &BTreeMap<u32, Vec<Call>>| {
        let mut v: Vec<(u32, u32)> = m
            .iter()
            .filter_map(|(&t, c)| c.first().map(|c| (c.seq, t)))
            .collect();
        v.sort();
        v.into_iter().map(|(_, t)| t).collect::<Vec<u32>>()
    };
    const PREFIX: usize = 64;
    for ta in by_first(a) {
        if pairs.iter().any(|p| p.0 == ta) {
            continue;
        }
        let ca = &a[&ta];
        let mut best: Option<(usize, u32)> = None;
        for tb in by_first(b) {
            if pairs.iter().any(|p| p.1 == tb) {
                continue;
            }
            let cb = &b[&tb];
            let agree = ca
                .iter()
                .zip(cb)
                .take(PREFIX)
                .take_while(|(x, y)| x.addr == y.addr)
                .count();
            if agree > 0 && best.is_none_or(|(n, _)| agree > n) {
                best = Some((agree, tb));
            }
        }
        if let Some((_, tb)) = best {
            pairs.push((ta, tb));
        }
    }
    pairs
}

/// One call that differs.
#[derive(Clone, Debug)]
pub struct Mismatch {
    pub addr: u32,
    /// The call's number among this function's calls on the thread (from 0).
    pub nth: usize,
    pub a_tid: u32,
    pub b_tid: u32,
    /// Index of the call in each thread's call list.
    pub a_index: usize,
    pub b_index: usize,
    pub a_seq: u32,
    pub what: Vec<String>,
}

/// Where one side made a call the other didn't.
#[derive(Clone, Debug)]
pub struct CountGap {
    pub addr: u32,
    pub a_tid: u32,
    pub b_tid: u32,
    pub a_count: usize,
    pub b_count: usize,
    /// The first unmatched call: its thread index on the side that has it.
    pub a_index: Option<usize>,
    pub b_index: Option<usize>,
    /// Its seq on the side that has it.
    pub seq: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub context: usize,
    pub pairs: Vec<(u32, u32)>,
    pub image: Option<(u32, u32)>,
}

pub struct Report {
    pub pairs: Vec<(u32, u32)>,
    pub unpaired_a: Vec<u32>,
    pub unpaired_b: Vec<u32>,
    /// Per function: calls in a, calls in b (paired threads only), calls compared and found
    /// equal.
    pub counts: BTreeMap<u32, (usize, usize, usize)>,
    pub first_value: Option<Mismatch>,
    pub value_mismatches: usize,
    pub first_gap: Option<CountGap>,
    pub text: String,
}

fn spec_of(list: &[FnSpec], addr: u32) -> Option<&FnSpec> {
    list.iter().find(|s| s.addr == addr)
}

fn name_of(list: &[FnSpec], addr: u32) -> String {
    match spec_of(list, addr) {
        Some(s) if !s.name.is_empty() => format!("{addr:08x} {}", s.name),
        _ => format!("{addr:08x}"),
    }
}

/// One call on one line: `seq`, depth, function, raw words (normalised in brackets where
/// that changes them) and answer.
pub fn describe(list: &[FnSpec], c: &Call, s: &Shape) -> String {
    let spec = spec_of(list, c.addr);
    let mut words = Vec::new();
    for (i, (&raw, n)) in c.words.iter().zip(&s.words).enumerate() {
        let kind = spec
            .and_then(|s| s.words.get(i).copied())
            .unwrap_or(WordKind::Word);
        let text = match (kind, n) {
            (WordKind::Float, _) => format!("{}f[{raw:08x}]", f32::from_bits(raw)),
            (_, Norm::Nth(k)) => format!("{raw:08x}=ptr#{k}"),
            (_, n @ Norm::Stack(..)) => format!("{raw:08x}={n}"),
            (_, Norm::Raw(v)) if *v != raw => format!("{raw:08x}&{v:x}"),
            _ => format!("{raw:08x}"),
        };
        words.push(text);
    }
    let ret = match (&c.ret, spec.map(|s| s.ret)) {
        (None, _) => " (no return recorded)".to_string(),
        (Some(_), Some(RetKind::Void)) => String::new(),
        (Some(r), Some(k)) if k.x87() => match r.st0 {
            Some(b) => format!(" -> st0 {}", f64::from_bits(b)),
            None => " -> st0 empty".into(),
        },
        (Some(r), _) => {
            let n = s.ret.as_ref().and_then(|v| v.first().copied());
            match n {
                Some(Norm::Nth(k)) => format!(" -> {:08x}=ptr#{k}", r.eax),
                _ => format!(" -> {:08x}", r.eax),
            }
        }
    };
    format!(
        "seq {:>8} {}{}({}){}",
        c.seq,
        "  ".repeat(c.depth.min(8) as usize),
        name_of(list, c.addr),
        words.join(", "),
        ret
    )
}

/// What a trace recorded of one function: how many calls, from and to which frame (counted by
/// the list's `frame` function over all threads), whether the count reached its `max=` and its
/// `after=` event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub count: u64,
    pub first_frame: u64,
    pub last_frame: u64,
    pub capped: bool,
    pub after: Option<(u32, u64)>,
}

/// The recording window of every function with a `max=` or `after=` in the trace's header.
pub fn windows(t: &Trace) -> BTreeMap<u32, Window> {
    let list = crate::list::from_header(&t.header).unwrap_or_default();
    let frame_fn = list
        .iter()
        .find(|s| s.conv == crate::list::Conv::Frame)
        .map(|s| s.addr);
    let mut frames: Vec<u32> = Vec::new();
    let mut by_fn: BTreeMap<u32, (u64, u32, u32)> = BTreeMap::new();
    let wanted: std::collections::HashMap<u32, &FnSpec> = list
        .iter()
        .filter(|s| s.max.is_some() || s.after.is_some())
        .map(|s| (s.addr, s))
        .collect();
    for recs in t.threads.values() {
        for r in recs {
            if let crate::format::Record::Enter { addr, seq, .. } = r {
                if Some(*addr) == frame_fn {
                    frames.push(*seq);
                }
                if wanted.contains_key(addr) {
                    let e = by_fn.entry(*addr).or_insert((0, u32::MAX, 0));
                    e.0 += 1;
                    e.1 = e.1.min(*seq);
                    e.2 = e.2.max(*seq);
                }
            }
        }
    }
    frames.sort_unstable();
    let frame_at = |seq: u32| frames.partition_point(|&f| f < seq) as u64;
    by_fn
        .into_iter()
        .map(|(addr, (count, first, last))| {
            let spec = wanted[&addr];
            (
                addr,
                Window {
                    count,
                    first_frame: frame_at(first),
                    last_frame: frame_at(last),
                    // Exactly the cap: a recorder that ignores `max=` (an older port build) records
                    // more, and is not stopped by it.
                    capped: spec.max == Some(count),
                    after: spec.after,
                },
            )
        })
        .collect()
}

/// Compares `b` against the reference `a` (the real game).
pub fn diff(a: &Trace, b: &Trace, list: &[FnSpec], opts: &Options) -> Report {
    let image = opts.image.unwrap_or(IMAGE);
    // Where each side's recording of a function stopped at its cap (`max=`) or started at its
    // event (`after=`): count comparisons past those frames mean nothing.
    let wa = windows(a);
    let wb = windows(b);
    let capped = |w: &BTreeMap<u32, Window>, addr: u32| w.get(&addr).is_some_and(|w| w.capped);
    let mut ca = calls(a);
    let mut cb = calls(b);
    // Only functions both runs recorded are compared (each header lists what its side
    // hooked); a function one side never hooks is not a difference.
    let listed = |t: &Trace| -> Option<std::collections::HashSet<u32>> {
        let l = crate::list::from_header(&t.header).ok()?;
        (!l.is_empty()).then(|| l.iter().map(|s| s.addr).collect())
    };
    if let (Some(la), Some(lb)) = (listed(a), listed(b)) {
        for m in [&mut ca, &mut cb] {
            for cs in m.values_mut() {
                cs.retain(|c| la.contains(&c.addr) && lb.contains(&c.addr));
            }
            m.retain(|_, cs| !cs.is_empty());
        }
    }
    // Threads that run the same start routine pair first (in order of their first calls),
    // when both traces name their threads' start addresses; then `pair_threads` matches the
    // rest by their first calls.
    let mut fixed = opts.pairs.clone();
    let order = |m: &BTreeMap<u32, Vec<Call>>| {
        let mut v: Vec<(u32, u32)> = m
            .iter()
            .filter_map(|(&t, c)| c.first().map(|c| (c.seq, t)))
            .collect();
        v.sort();
        v.into_iter().map(|(_, t)| t).collect::<Vec<u32>>()
    };
    let order_b = order(&cb);
    for ta in order(&ca) {
        let Some(start) = a.starts.get(&ta) else {
            continue;
        };
        if fixed.iter().any(|p| p.0 == ta) {
            continue;
        }
        if let Some(&tb) = order_b
            .iter()
            .find(|tb| b.starts.get(tb) == Some(start) && !fixed.iter().any(|p| p.1 == **tb))
        {
            fixed.push((ta, tb));
        }
    }
    let pairs = pair_threads(&ca, &cb, &fixed);
    let unpaired_a: Vec<u32> = ca
        .keys()
        .copied()
        .filter(|t| !pairs.iter().any(|p| p.0 == *t))
        .collect();
    let unpaired_b: Vec<u32> = cb
        .keys()
        .copied()
        .filter(|t| !pairs.iter().any(|p| p.1 == *t))
        .collect();

    let mut na = Normalizer::new(image);
    let mut nb = Normalizer::new(image);
    // Shapes of every call, by thread, in order (normalisation follows call order).
    let shapes = |n: &mut Normalizer, m: &BTreeMap<u32, Vec<Call>>| -> BTreeMap<u32, Vec<Shape>> {
        m.iter()
            .map(|(&t, cs)| {
                (
                    t,
                    cs.iter()
                        .map(|c| shape(n, spec_of(list, c.addr), c))
                        .collect(),
                )
            })
            .collect()
    };
    let sa = shapes(&mut na, &ca);
    let sb = shapes(&mut nb, &cb);

    let mut counts: BTreeMap<u32, (usize, usize, usize)> = BTreeMap::new();
    let mut first_value: Option<Mismatch> = None;
    let mut value_mismatches = 0;
    // Per function: (real seq, call number, threads, first difference) of its first differing call.
    let mut per_fn: BTreeMap<u32, (u32, usize, u32, u32, String)> = BTreeMap::new();
    let mut first_gap: Option<CountGap> = None;
    for &(ta, tb) in &pairs {
        let (xa, xb) = (&ca[&ta], &cb[&tb]);
        // Per function: the indices of its calls on each thread.
        let mut by_fn: BTreeMap<u32, (Vec<usize>, Vec<usize>)> = BTreeMap::new();
        for (i, c) in xa.iter().enumerate() {
            by_fn.entry(c.addr).or_default().0.push(i);
        }
        for (i, c) in xb.iter().enumerate() {
            by_fn.entry(c.addr).or_default().1.push(i);
        }
        for (addr, (ia, ib)) in by_fn {
            let entry = counts.entry(addr).or_default();
            entry.0 += ia.len();
            entry.1 += ib.len();
            for (nth, (&i, &j)) in ia.iter().zip(&ib).enumerate() {
                let (x, y) = (&sa[&ta][i], &sb[&tb][j]);
                let mut what = Vec::new();
                if x.words.len() != y.words.len() {
                    what.push(format!(
                        "word count: real {} port {}",
                        x.words.len(),
                        y.words.len()
                    ));
                }
                for (k, (u, v)) in x.words.iter().zip(&y.words).enumerate() {
                    if u != v {
                        what.push(format!("word {k}: real {u} port {v}"));
                    }
                }
                match (&x.ret, &y.ret) {
                    (Some(u), Some(v)) if u != v => {
                        let show = |r: &Vec<Norm>| {
                            r.iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join(":")
                        };
                        what.push(format!("return: real {} port {}", show(u), show(v)));
                    }
                    // A thread's last call that never returned is where its recording ended
                    // (the run was stopped), not a difference.
                    (Some(_), None) if j + 1 < xb.len() => {
                        what.push("return: the port's call never returned".into())
                    }
                    (None, Some(_)) if i + 1 < xa.len() => {
                        what.push("return: the real call never returned".into())
                    }
                    _ => {}
                }
                if what.is_empty() {
                    entry.2 += 1;
                    continue;
                }
                value_mismatches += 1;
                let seq = xa[i].seq;
                if per_fn.get(&addr).is_none_or(|p| seq < p.0) {
                    per_fn.insert(addr, (seq, nth, ta, tb, what[0].clone()));
                }
                if first_value.as_ref().is_none_or(|m| seq < m.a_seq) {
                    first_value = Some(Mismatch {
                        addr,
                        nth,
                        a_tid: ta,
                        b_tid: tb,
                        a_index: i,
                        b_index: j,
                        a_seq: seq,
                        what,
                    });
                }
            }
            // A side that stopped at its cap has fewer calls by design, not a gap.
            let shorter_capped = (ia.len() < ib.len() && capped(&wa, addr))
                || (ib.len() < ia.len() && capped(&wb, addr));
            if ia.len() != ib.len() && !shorter_capped {
                let n = ia.len().min(ib.len());
                let (a_index, b_index, seq) = if ia.len() > n {
                    (Some(ia[n]), None, xa[ia[n]].seq)
                } else {
                    (None, Some(ib[n]), xb[ib[n]].seq)
                };
                let gap = CountGap {
                    addr,
                    a_tid: ta,
                    b_tid: tb,
                    a_count: ia.len(),
                    b_count: ib.len(),
                    a_index,
                    b_index,
                    seq,
                };
                // The earliest by the real game's order where the real side has the call;
                // a gap where only the port has extra calls ranks after every real one.
                let rank = |g: &CountGap| (g.a_index.is_none(), g.seq);
                if first_gap.as_ref().is_none_or(|g| rank(&gap) < rank(g)) {
                    first_gap = Some(gap);
                }
            }
        }
    }

    let mut text = String::new();
    let _ = writeln!(text, "nv-trace diff: real (a) against port (b)");
    for (side, w) in [("real", &wa), ("port", &wb)] {
        for (addr, win) in w {
            if win.capped || win.after.is_some() {
                let _ = writeln!(
                    text,
                    "recording window: {side} {} recorded {} calls, frames {}..={}{}{}",
                    name_of(list, *addr),
                    win.count,
                    win.first_frame,
                    win.last_frame,
                    if win.capped {
                        ", STOPPED AT ITS CAP (compare only these frames)"
                    } else {
                        ""
                    },
                    win.after
                        .map(|(e, n)| format!(", started after call {n} of {e:08x}"))
                        .unwrap_or_default()
                );
            }
        }
    }
    let _ = writeln!(
        text,
        "threads paired (real tid -> port tid): {}",
        pairs
            .iter()
            .map(|(x, y)| {
                let s = |t: &Trace, id: &u32| {
                    t.starts
                        .get(id)
                        .map_or(String::new(), |s| format!(" (start {s:08x})"))
                };
                format!("{x}{}->{y}{}", s(a, x), s(b, y))
            })
            .collect::<Vec<_>>()
            .join(", ")
    );
    if !unpaired_a.is_empty() || !unpaired_b.is_empty() {
        let _ = writeln!(
            text,
            "unpaired threads: real {:?} ({} calls), port {:?} ({} calls)",
            unpaired_a,
            unpaired_a.iter().map(|t| ca[t].len()).sum::<usize>(),
            unpaired_b,
            unpaired_b.iter().map(|t| cb[t].len()).sum::<usize>()
        );
    }
    let _ = writeln!(
        text,
        "\nper function (paired threads): real calls, port calls, equal"
    );
    for (addr, (x, y, eq)) in &counts {
        let _ = writeln!(text, "  {:<60} {x:>8} {y:>8} {eq:>8}", name_of(list, *addr));
    }
    for spec in list {
        if !counts.contains_key(&spec.addr) {
            let _ = writeln!(
                text,
                "  {:<60} {:>8} {:>8} {:>8}",
                name_of(list, spec.addr),
                0,
                0,
                0
            );
        }
    }
    let context = |side: &str, cs: &[Call], ss: &[Shape], at: usize, out: &mut String| {
        let from = at.saturating_sub(opts.context);
        for k in from..=at.min(cs.len().saturating_sub(1)) {
            let mark = if k == at { ">>" } else { "  " };
            let _ = writeln!(out, "  {side} {mark} {}", describe(list, &cs[k], &ss[k]));
        }
    };
    match &first_value {
        None => {
            let _ = writeln!(
                text,
                "\nno call with different values ({} compared)",
                counts.values().map(|c| c.2).sum::<usize>()
            );
        }
        Some(m) => {
            let _ = writeln!(
                text,
                "\nFIRST DIFFERENT CALL ({} differ in all): {} call #{} on real thread {} / port thread {} (real seq {})",
                value_mismatches,
                name_of(list, m.addr),
                m.nth,
                m.a_tid,
                m.b_tid,
                m.a_seq
            );
            for w in &m.what {
                let _ = writeln!(text, "  {w}");
            }
            let _ = writeln!(text, " real, the calls before it on its thread:");
            context("real", &ca[&m.a_tid], &sa[&m.a_tid], m.a_index, &mut text);
            let _ = writeln!(text, " port:");
            context("port", &cb[&m.b_tid], &sb[&m.b_tid], m.b_index, &mut text);
        }
    }
    if !per_fn.is_empty() {
        let _ = writeln!(
            text,
            "
first differing call of each function:"
        );
        for (addr, (seq, nth, ta, tb, what)) in &per_fn {
            let _ = writeln!(
                text,
                "  {} call #{nth} (real seq {seq}, threads {ta}/{tb}): {what}",
                name_of(list, *addr)
            );
        }
    }
    if let Some(g) = &first_gap {
        let _ = writeln!(
            text,
            "\nFIRST COUNT GAP: {} made {} times in the real game, {} in the port (threads {} / {})",
            name_of(list, g.addr),
            g.a_count,
            g.b_count,
            g.a_tid,
            g.b_tid
        );
        if let Some(i) = g.a_index {
            let _ = writeln!(text, " real has call #{} the port never made:", g.b_count);
            context("real", &ca[&g.a_tid], &sa[&g.a_tid], i, &mut text);
        }
        if let Some(j) = g.b_index {
            let _ = writeln!(
                text,
                " port has call #{} the real game never made:",
                g.a_count
            );
            context("port", &cb[&g.b_tid], &sb[&g.b_tid], j, &mut text);
        }
    }
    Report {
        pairs,
        unpaired_a,
        unpaired_b,
        counts,
        first_value,
        value_mismatches,
        first_gap,
        text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::Record;
    use crate::list;

    /// One test call: thread, address, words, `EAX`, `ST0`.
    type TestCall<'a> = (u32, u32, &'a [u32], u32, Option<f64>);

    /// A trace from test calls, each entered and left in turn.
    fn trace(calls: &[TestCall]) -> Trace {
        let mut t = Trace::default();
        for (seq, &(tid, addr, words, eax, st0)) in calls.iter().enumerate() {
            let recs = t.threads.entry(tid).or_default();
            recs.push(Record::Enter {
                addr,
                seq: seq as u32,
                words: words.to_vec(),
            });
            recs.push(Record::Leave {
                addr,
                seq: seq as u32,
                eax,
                edx: 0,
                st0: st0.map(f64::to_bits),
            });
        }
        t
    }

    fn specs() -> Vec<FnSpec> {
        list::parse(
            "00000100 thiscall ppf void Add\n00000200 cdecl fppb u8 Update\n00000300 thiscall p f32 Len\n",
        )
        .unwrap()
    }

    fn opts() -> Options {
        Options {
            context: 3,
            ..Options::default()
        }
    }

    #[test]
    fn equal_runs_with_different_heaps_and_tids_match() {
        let a = trace(&[
            (10, 0x100, &[0x0a00_0000, 0x0b00_0000, 0x3f00_0000], 7, None),
            (10, 0x100, &[0x0a00_0000, 0x0b00_0010, 0x3f80_0000], 9, None),
            (
                10,
                0x200,
                &[0x3f00_0000, 0x0c00_0000, 0, 0xcccc_cc01],
                0x1234_5601,
                None,
            ),
        ]);
        // Other addresses, other thread id, garbage above a byte argument and in a void EAX.
        let b = trace(&[
            (1, 0x100, &[0x2000_0000, 0x2100_0000, 0x3f00_0000], 0, None),
            (1, 0x100, &[0x2000_0000, 0x2200_0000, 0x3f80_0000], 0, None),
            (1, 0x200, &[0x3f00_0000, 0x2300_0000, 0, 1], 1, None),
        ]);
        let r = diff(&a, &b, &specs(), &opts());
        assert_eq!(r.pairs, [(10, 1)]);
        assert!(r.first_value.is_none(), "{}", r.text);
        assert!(r.first_gap.is_none());
        assert_eq!(r.counts[&0x100], (2, 2, 2));
    }

    #[test]
    fn the_first_differing_call_is_found_with_context() {
        let a = trace(&[
            (10, 0x100, &[0x0a00_0000, 0x0b00_0000, 0x3f00_0000], 0, None),
            (10, 0x300, &[0x0a00_0000], 0, Some(2.0)),
            // Same pointers again (this is ptr#1, the argument ptr#2), but the port passes a new one.
            (10, 0x100, &[0x0a00_0000, 0x0b00_0000, 0x3f00_0000], 0, None),
            (10, 0x300, &[0x0a00_0000], 0, Some(3.0)),
        ]);
        let b = trace(&[
            (2, 0x100, &[0x2000_0000, 0x2100_0000, 0x3f00_0000], 0, None),
            (2, 0x300, &[0x2000_0000], 0, Some(2.0)),
            (2, 0x100, &[0x2000_0000, 0x2100_0040, 0x3f00_0000], 0, None),
            (2, 0x300, &[0x2000_0000], 0, Some(2.5)),
        ]);
        let r = diff(&a, &b, &specs(), &opts());
        let m = r.first_value.as_ref().expect("a mismatch");
        assert_eq!((m.addr, m.nth, m.a_seq), (0x100, 1, 2));
        assert_eq!(m.what, ["word 1: real ptr#2 port ptr#3"]);
        assert_eq!(r.value_mismatches, 2);
        assert!(r.text.contains("FIRST DIFFERENT CALL"));
        // The context shows the calls before it on both threads.
        assert!(r
            .text
            .contains("real    seq        1 00000300 Len(0a000000=ptr#1) -> st0 2"));
        assert!(r.text.contains("port >> seq        2 00000100 Add("));
    }

    #[test]
    fn a_float_return_differs_by_its_bits() {
        let a = trace(&[(1, 0x300, &[0x0a00_0000], 0, Some(0.1f32 as f64))]);
        let b = trace(&[(1, 0x300, &[0x0b00_0000], 0, Some(0.1f64))]);
        let r = diff(&a, &b, &specs(), &opts());
        // 0.1 as a double rounds to the same float: equal as an f32 return.
        assert!(r.first_value.is_none(), "{}", r.text);
        let b = trace(&[(1, 0x300, &[0x0b00_0000], 0, Some(0.1000001f64))]);
        let r = diff(&a, &b, &specs(), &opts());
        assert!(r.first_value.unwrap().what[0].starts_with("return:"));
    }

    #[test]
    fn missing_and_extra_calls_are_count_gaps() {
        let a = trace(&[
            (1, 0x100, &[0x0a00_0000, 0, 0], 0, None),
            (1, 0x200, &[0, 0, 0, 0], 0, None),
            (1, 0x200, &[0, 0, 0, 0], 0, None),
        ]);
        let b = trace(&[
            (5, 0x100, &[0x0b00_0000, 0, 0], 0, None),
            (5, 0x200, &[0, 0, 0, 0], 0, None),
        ]);
        let r = diff(&a, &b, &specs(), &opts());
        assert!(r.first_value.is_none());
        let g = r.first_gap.unwrap();
        assert_eq!((g.addr, g.a_count, g.b_count, g.seq), (0x200, 2, 1, 2));
        assert!(r.text.contains("real has call #1 the port never made"));
    }

    #[test]
    fn word_counts_and_unreturned_calls_differ() {
        let a = trace(&[(1, 0x300, &[0x0a00_0000], 0, Some(1.0))]);
        let mut b = trace(&[(1, 0x300, &[0x0a00_0000, 5], 0, Some(1.0))]);
        let r = diff(&a, &b, &specs(), &opts());
        assert_eq!(r.first_value.unwrap().what, ["word count: real 1 port 2"]);
        // The port's last call never returned: the recording ended there, no difference.
        b = trace(&[(1, 0x300, &[0x0a00_0000], 0, Some(1.0))]);
        b.threads.get_mut(&1).unwrap().pop();
        let r = diff(&a, &b, &specs(), &opts());
        assert!(r.first_value.is_none(), "{}", r.text);
        // A call before others that never returned is one.
        let two: &[TestCall] = &[
            (1, 0x300, &[0x0a00_0000], 0, Some(1.0)),
            (1, 0x300, &[0x0a00_0000], 0, Some(1.0)),
        ];
        let a = trace(two);
        let mut b = trace(two);
        b.threads.get_mut(&1).unwrap().remove(1);
        let r = diff(&a, &b, &specs(), &opts());
        assert_eq!(
            r.first_value.unwrap().what,
            ["return: the port's call never returned"]
        );
    }

    #[test]
    fn a_capped_side_is_reported_and_not_a_gap() {
        // Real: frames (0x400) and two calls of 0x100, its cap; port: four calls.
        let mut a = trace(&[
            (1, 0x400, &[0], 0, None),
            (1, 0x100, &[0x0a00_0000, 0, 0], 0, None),
            (1, 0x400, &[0], 0, None),
            (1, 0x100, &[0x0a00_0000, 0, 0], 0, None),
            (1, 0x400, &[0], 0, None),
        ]);
        let b = trace(&[
            (2, 0x400, &[0], 0, None),
            (2, 0x100, &[0x0b00_0000, 0, 0], 0, None),
            (2, 0x400, &[0], 0, None),
            (2, 0x100, &[0x0b00_0000, 0, 0], 0, None),
            (2, 0x400, &[0], 0, None),
            (2, 0x100, &[0x0b00_0000, 0, 0], 0, None),
            (2, 0x100, &[0x0b00_0000, 0, 0], 0, None),
        ]);
        a.header = "fn=00000400 frame p void F
fn=00000100 thiscall ppf void max=2 Add
"
        .into();
        let w = windows(&a);
        assert_eq!(
            w[&0x100],
            Window {
                count: 2,
                first_frame: 1,
                last_frame: 2,
                capped: true,
                after: None
            }
        );
        let r = diff(&a, &b, &specs(), &opts());
        assert!(r.first_gap.is_none(), "{}", r.text);
        assert!(
            r.text.contains("frames 1..=2, STOPPED AT ITS CAP"),
            "{}",
            r.text
        );
    }

    #[test]
    fn functions_only_one_side_hooked_are_left_out() {
        let mut a = trace(&[(1, 0x100, &[0x0a00_0000, 0, 0], 0, None)]);
        let mut b = trace(&[
            (2, 0x200, &[0, 0, 0, 0], 0, None),
            (2, 0x100, &[0x0b00_0000, 0, 0], 0, None),
        ]);
        a.header = "fn=00000100 thiscall ppf void Add
"
        .into();
        b.header = "fn=00000100 thiscall ppf void Add
fn=00000200 cdecl fppb u8 Update
"
        .into();
        let r = diff(&a, &b, &specs(), &opts());
        assert_eq!(r.pairs, [(1, 2)]);
        assert!(
            r.first_gap.is_none() && r.first_value.is_none(),
            "{}",
            r.text
        );
    }

    #[test]
    fn threads_with_the_same_start_routine_pair_first() {
        // By first calls 7 would pair with 2; their start routines say 7 runs what 3 runs.
        let mut a = trace(&[
            (7, 0x100, &[0, 0, 0], 0, None),
            (8, 0x200, &[0, 0, 0, 0], 0, None),
        ]);
        let mut b = trace(&[
            (2, 0x100, &[0, 0, 0], 0, None),
            (3, 0x100, &[0, 0, 0], 0, None),
        ]);
        a.starts.insert(7, 0x00aa_0000);
        b.starts.insert(2, 0);
        b.starts.insert(3, 0x00aa_0000);
        let r = diff(&a, &b, &specs(), &opts());
        assert_eq!(r.pairs[0], (7, 3));
        assert!(
            r.text.contains("7 (start 00aa0000)->3 (start 00aa0000)"),
            "{}",
            r.text
        );
    }

    #[test]
    fn threads_pair_by_their_first_calls() {
        // Real: thread 7 runs 0x200s first, thread 8 the 0x100s. Port: tids swapped in order.
        let a = trace(&[
            (7, 0x200, &[0, 0, 0, 0], 0, None),
            (8, 0x100, &[0, 0, 0], 0, None),
            (7, 0x200, &[0, 0, 0, 0], 0, None),
        ]);
        let b = trace(&[
            (2, 0x100, &[0, 0, 0], 0, None),
            (3, 0x200, &[0, 0, 0, 0], 0, None),
            (3, 0x200, &[0, 0, 0, 0], 0, None),
        ]);
        let r = diff(&a, &b, &specs(), &opts());
        assert_eq!(r.pairs, [(7, 3), (8, 2)]);
        assert!(r.first_value.is_none() && r.first_gap.is_none());
        // A fixed pair wins.
        let o = Options {
            pairs: vec![(7, 2)],
            ..opts()
        };
        let r = diff(&a, &b, &specs(), &o);
        assert_eq!(r.pairs[0], (7, 2));
        assert!(r.first_gap.is_some());
    }

    #[test]
    fn stack_words_compare_by_frame_offset_apart_from_heap_pointers() {
        use crate::format::{Record, StackWord, DEPTH_UNKNOWN};
        // Real: the record argument is in the caller's frame at +0x1c0 both times (same
        // address). Port: first at +0x1c0, then at +0x220 of a frame block: a layout
        // difference the offsets show although both are "the first stack address" by number.
        let mut a = trace(&[
            (1, 0x300, &[0x0019_ff00], 0, Some(1.0)),
            (1, 0x300, &[0x0019_ff00], 0, Some(1.0)),
        ]);
        let mut b = trace(&[
            (2, 0x300, &[0x3000_01c0], 0, Some(1.0)),
            (2, 0x300, &[0x3100_0220], 0, Some(1.0)),
        ]);
        let add = |t: &mut Trace, tid: u32, seq: u32, depth: u8, offset: i32| {
            let recs = t.threads.get_mut(&tid).unwrap();
            let at = recs
                .iter()
                .position(|r| matches!(r, Record::Enter { seq: s, .. } if *s == seq))
                .unwrap();
            recs.insert(
                at + 1,
                Record::Stack {
                    seq,
                    words: vec![StackWord {
                        index: 0,
                        depth,
                        offset,
                    }],
                },
            );
        };
        add(&mut a, 1, 0, DEPTH_UNKNOWN, 0x1c0);
        add(&mut a, 1, 1, DEPTH_UNKNOWN, 0x1c0);
        add(&mut b, 2, 0, 0, 0x1c0);
        add(&mut b, 2, 1, 0, 0x220);
        let r = diff(&a, &b, &specs(), &opts());
        let m = r.first_value.expect("the second call differs");
        assert_eq!((m.addr, m.nth), (0x300, 1));
        assert_eq!(
            m.what,
            ["word 0: real stk#1(frame+0x1c0) port stk#2(frame+0x220)"]
        );
        // A heap pointer never equals a stack address.
        assert_ne!(Norm::Nth(1), Norm::Stack(1, None));
    }

    #[test]
    fn image_addresses_are_compared_as_they_are() {
        let mut n = Normalizer::new(IMAGE);
        assert_eq!(n.ptr((1, 1), 0x0101_0000), Norm::Raw(0x0101_0000));
        assert_eq!(n.ptr((1, 1), 0), Norm::Raw(0));
        assert_eq!(n.ptr((1, 1), 0x5000_0000), Norm::Nth(1));
        assert_eq!(n.ptr((1, 1), 0x6000_0000), Norm::Nth(2));
        assert_eq!(n.ptr((1, 1), 0x5000_0000), Norm::Nth(1));
        // Another function's pointers are counted on their own.
        assert_eq!(n.ptr((1, 2), 0x6000_0000), Norm::Nth(1));
    }
}
