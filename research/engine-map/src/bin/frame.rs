//! Frame map: the call tree of the game's per-frame function (policy: ADR-0002).
//!
//! ```text
//! frame <pc-dir> <xb-dir> <engine_map.tsv> <frame.tsv> [<report.tsv>]
//! ```
//!
//! The frame is `Main::OnIdle` (Xbox PDB), PC `0086e650`, Xbox `8269f3c8`
//! (docs/FRAME_SKELETON.md). Reads `NvEngineMap.java`'s `functions.tsv`
//! (ordered direct calls, strings), the `xbox` binary's `xb_funcs.tsv` and
//! the committed engine map. Writes one row per call site of the tree, depth
//! first, in call order; a function called twice gets two rows (and two
//! subtrees). Depth 0 is `Main::OnIdle` itself; the tree stops at depth 3.
//!
//! `seq, depth, call, address, name, name_tier, xbox, unit, subsystem, note`
//!
//! - `call`: the call's position (1-based) in its caller's PC call list.
//! - `name`, `name_tier`, `xbox`: from the engine map; where the map has no
//!   Xbox name, from the hand-confirmed pairs below (`frame`) or from
//!   aligning the caller's PC and Xbox call lists (`align-sim`,
//!   `align-gap`). Names are sanitized as in `match.rs`.
//! - `unit`, `subsystem`: from the engine map.
//! - `note`: `recursive` when the function is already on the path above it
//!   (not expanded again); empty otherwise.
//!
//! Only direct calls are visible: calls through a vtable or a function
//! pointer (`bhkWorld::Update`, the AI task threads) do not appear.
//!
//! **Alignment.** For a caller whose Xbox counterpart is known, the two call
//! lists (PowerPC save/restore helpers left out, as in `match.rs`) are
//! aligned in three passes:
//!
//! 1. anchors: calls already paired (longest common subsequence);
//! 2. `align-sim`: inside each gap between anchors, calls whose own callees
//!    (PC side mapped to Xbox) and referenced strings share at least
//!    `MIN_SIM` members, aligned in order to the largest total;
//! 3. `align-gap`: a remaining gap of exactly one call on both sides pairs
//!    the two (the same rule as `match.rs`'s strong gap).
//!
//! A proposal is accepted only if every call site of the PC function in the
//! tree proposes the same Xbox function, no other PC function is proposed
//! for it, and neither side is already paired; accepted pairs then serve as
//! anchors and as callers' Xbox counterparts, to a fixed point. These pairs
//! are local to the frame (not checked over the whole exe like
//! `match.rs`), so they are leads, kept apart by their tier.
//!
//! The optional report lists the root's alignment (PC call, Xbox call, both
//! names, the kind) for review; it holds Xbox names of calls the PC lacks,
//! so it stays private. Ledger statuses are not written here: the ledger
//! computes them from the Rust sources (scripts/ledger).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

#[path = "../names.rs"]
mod names;
use names::{ppc_helper, sanitize};

type R<T> = Result<T, Box<dyn std::error::Error>>;

/// `Main::OnIdle` on both sides.
const ROOT_PC: u32 = 0x0086_e650;
const ROOT_XB: u32 = 0x8269_f3c8;
const MAX_DEPTH: usize = 3;
/// Shared callees plus shared strings for an `align-sim` pair.
const MIN_SIM: usize = 2;

/// PC-to-Xbox pairs on the frame confirmed by hand where the matcher has
/// none (tier `frame`). The evidence for each (shared callees, strings,
/// units, call order, all from the exe) is in docs/FRAME_SKELETON.md, "Evidence
/// for the pairs"; names are the Xbox PDB's.
const CONFIRMED: &[(u32, u32)] = &[
    (0x0086_e650, 0x8269_f3c8), // Main::OnIdle
    (0x0070_1450, 0x8253_bbf8), // FaderManager::IsFaderVisible
    (0x0087_82b0, 0x826a_0cf0), // MemoryLevelManager::RunNonDestructiveFree
    (0x0086_f190, 0x8269_5e38), // Main::OnIdle_FixActorBones
    (0x0086_f260, 0x8269_3660), // Main::OnIdle_UpdateTimer
    (0x00c3_dbf0, 0x829c_1c90), // IOManager::UpdateQueue
    (0x0045_7d70, 0x822a_5e50), // TES::ShowLoadingMenu
    (0x0086_efe0, 0x8269_33e8), // Main::OnIdle_ScaleLOD
    (0x0070_11d0, 0x8253_beb8), // FaderManager::UpdateFaders
    (0x004e_0110, 0x8232_3de0), // ScreenSplatter::Update
    (0x004d_e600, 0x8232_2430), // ScreenCustomSplatter::Update
    (0x0086_7a40, 0x8269_0510), // Calendar::Update
    (0x0045_5640, 0x822a_2b58), // TES::RunAnimations
    (0x0055_3820, 0x8238_66a8), // TESObjectCELL::RunAnimations
    (0x004b_aba0, 0x8230_93b0), // GridCellArray::RunAnimations
    (0x0097_8550, 0x8277_f8e0), // ProcessLists::RunActorScripts
    (0x0097_77a0, 0x8277_f498), // ProcessLists::UpdateRadiationList
    (0x0096_e9b0, 0x8277_d180), // ProcessLists::UpdateFollowerTempList
    (0x0066_52e0, 0x8249_cf10), // BSTreeManager::Update
    (0x0086_fbe0, 0x8269_3ec0), // Main::OnIdle_UpdateCurrentGridCell
    (0x0086_fd70, 0x8269_3fb8), // Main::OnIdle_DoInterfaceIdle
    (0x0070_27e0, 0x8253_d648), // Interface::PreIdleStuff
    (0x0070_2810, 0x8253_d670), // Interface::Idle
    (0x0070_2840, 0x8253_d698), // Interface::PostIdleStuff
    (0x0049_fef0, 0x822f_1ea8), // BGSDecalManager::GetInstance
    (0x0049_fff0, 0x822f_2928), // BGSDecalManager::UpdateDecals
    (0x00b5_4000, 0x82a3_e730), // BSShaderManager::SetFOV
    (0x0045_bc80, 0x822a_7190), // TES::ResetAllMultiBoundNodes
    (0x0045_b070, 0x822a_9cc0), // TES::UpdateMultiBoundVisibility
    (0x008c_a070, 0x826e_f978), // AITaskManager::StartTasksDuringRendering
    (0x008c_80e0, 0x826e_c9d0), // AILinearTaskThreadManager::SetMainRendering
    (0x008c_78c0, 0x826e_c6f0), // AILinearTaskThreadManager::StartThreads
    (0x008c_7990, 0x826e_c788), // AILinearTaskThreadManager::WaitForThreads
    (0x0086_fc60, 0x8269_5f10), // Main::OnIdle_UpdateAnimationsAndEffects
    (0x0097_4420, 0x8278_8160), // ProcessLists::UpdateTempEffects
    (0x0097_46c0, 0x8278_8470), // ProcessLists::UpdateTempEffectsParallel
    (0x0045_3550, 0x822a_8ce8), // TES::UpdateCellAnimations
    (0x0045_3860, 0x822a_43a8), // TES::LockHavokUpdateMT
    (0x0055_1890, 0x8239_2700), // TESObjectCELL::UpdateManagedNodes
    (0x004b_a9a0, 0x8230_9150), // GridCellArray::UpdateManagedNodes
    (0x00c5_0610, 0x829d_2fd8), // BSParticleSystemManager::UpdateParallel
    (0x006e_bc50, 0x8252_7680), // PathManager::Update
    (0x006a_61b0, 0x824d_3738), // NavMeshRender::Update
    (0x006c_0720, 0x824f_8568), // NavMeshObstacleManager::GetInstance
    (0x006c_3640, 0x824f_7a88), // NavMeshObstacleManager::Update
    (0x0099_1500, 0x827a_32c0), // CombatManager::Update
    (0x0086_ff70, 0x8269_ed48), // Main::Swap
    (0x0087_05d0, 0x8269_3fe0), // Main::PostSwapProcess
    (0x0087_0610, 0x8269_4090), // Main::OnIdle_PostThreadsProcess
    (0x005a_e270, 0x823e_6c68), // Script::ClearOptimizations
    (0x005a_9d60, 0x823e_6918), // ScriptLocals::ClearOptimizations
];

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

fn strings(s: &str) -> BTreeSet<String> {
    if s.is_empty() {
        BTreeSet::new()
    } else {
        s.split('|').map(str::to_string).collect()
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

#[derive(Clone, Default)]
struct MapRow {
    name: String,
    tier: String,
    xbox: Option<u32>,
    unit: String,
    subsystem: String,
}

/// The engine map: rows by address, and its comment header.
fn load_map(path: &Path) -> R<(HashMap<u32, MapRow>, Vec<String>)> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut header = Vec::new();
    let mut cols: Vec<&str> = Vec::new();
    let mut out = HashMap::new();
    for line in text.lines() {
        if let Some(h) = line.strip_prefix('#') {
            header.push(h.trim().to_string());
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if cols.is_empty() {
            cols = f;
            continue;
        }
        let get = |k: &str| -> String {
            cols.iter()
                .position(|c| *c == k)
                .and_then(|i| f.get(i))
                .map(|s| s.to_string())
                .unwrap_or_default()
        };
        let xbox = get("xbox");
        out.insert(
            hex(&get("address")),
            MapRow {
                name: get("name"),
                tier: get("name_tier"),
                xbox: (!xbox.is_empty()).then(|| hex(&xbox)),
                unit: get("unit"),
                subsystem: get("subsystem"),
            },
        );
    }
    Ok((out, header))
}

/// Monotone pairing of two lists with the largest total score; `score`
/// returns 0 for pairs that may not be made. Returns (i, j) pairs.
fn best_pairs(n: usize, m: usize, score: impl Fn(usize, usize) -> usize) -> Vec<(usize, usize)> {
    let w = m + 1;
    let mut dp = vec![0usize; (n + 1) * w];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            let s = score(i, j);
            let take = if s > 0 {
                dp[(i + 1) * w + j + 1] + s
            } else {
                0
            };
            dp[i * w + j] = take.max(dp[(i + 1) * w + j]).max(dp[i * w + j + 1]);
        }
    }
    let (mut i, mut j, mut out) = (0, 0, Vec::new());
    while i < n && j < m {
        let s = score(i, j);
        if s > 0 && dp[i * w + j] == dp[(i + 1) * w + j + 1] + s {
            out.push((i, j));
            i += 1;
            j += 1;
        } else if dp[(i + 1) * w + j] >= dp[i * w + j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}

/// How a PC call position was paired with an Xbox call position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Anchor,
    Sim(usize),
    Gap,
}

/// Gaps between consecutive pairs (and the list ends): (i0, i1, j0, j1).
fn gaps(pairs: &[(usize, usize)], n: usize, m: usize) -> Vec<(usize, usize, usize, usize)> {
    let mut out = Vec::new();
    let (mut pi, mut pj) = (0, 0);
    for &(i, j) in pairs {
        out.push((pi, i, pj, j));
        pi = i + 1;
        pj = j + 1;
    }
    out.push((pi, n, pj, m));
    out
}

/// Aligns a PC call list (length `n`) with an Xbox call list (length `m`)
/// (module docs, "Alignment"). `anchor(i, j)`: the two calls are already
/// paired; `sim(i, j)`: their similarity. Returns, per PC position, the
/// Xbox position and how it was paired.
fn align(
    n: usize,
    m: usize,
    anchor: impl Fn(usize, usize) -> bool,
    sim: impl Fn(usize, usize) -> usize,
) -> Vec<Option<(usize, Kind)>> {
    let mut out = vec![None; n];
    let anchors = best_pairs(n, m, |i, j| anchor(i, j) as usize);
    let mut all = Vec::new();
    for (i0, i1, j0, j1) in gaps(&anchors, n, m) {
        let inner = best_pairs(i1 - i0, j1 - j0, |i, j| {
            let s = sim(i0 + i, j0 + j);
            if s >= MIN_SIM {
                s
            } else {
                0
            }
        });
        for (i, j) in inner {
            out[i0 + i] = Some((j0 + j, Kind::Sim(sim(i0 + i, j0 + j))));
            all.push((i0 + i, j0 + j));
        }
    }
    for &(i, j) in &anchors {
        out[i] = Some((j, Kind::Anchor));
        all.push((i, j));
    }
    all.sort();
    for (i0, i1, j0, j1) in gaps(&all, n, m) {
        if i1 - i0 == 1 && j1 - j0 == 1 {
            out[i0] = Some((j0, Kind::Gap));
        }
    }
    out
}

struct PcFn {
    calls: Vec<u32>,
    strings: BTreeSet<String>,
}

struct XbFn {
    name: String,
    calls: Vec<u32>,
    strings: BTreeSet<String>,
}

struct Ctx {
    pc: HashMap<u32, PcFn>,
    xb: HashMap<u32, XbFn>,
    map: HashMap<u32, MapRow>,
    /// Pairs found for the frame: hand-confirmed and aligned.
    extra: HashMap<u32, (u32, &'static str)>,
}

struct Node {
    depth: usize,
    call: usize,
    addr: u32,
    row: MapRow,
    note: &'static str,
}

const NO_CALLS: &[u32] = &[];

impl Ctx {
    fn pc_calls(&self, a: u32) -> &[u32] {
        self.pc
            .get(&a)
            .map(|f| f.calls.as_slice())
            .unwrap_or(NO_CALLS)
    }

    fn xb_calls(&self, x: Option<u32>) -> &[u32] {
        x.and_then(|x| self.xb.get(&x))
            .map(|f| f.calls.as_slice())
            .unwrap_or(NO_CALLS)
    }

    /// The Xbox counterpart of a PC function: the map's, else the frame's.
    fn xbox_of(&self, p: u32) -> Option<u32> {
        self.map
            .get(&p)
            .and_then(|r| r.xbox)
            .or_else(|| self.extra.get(&p).map(|e| e.0))
    }

    /// The map has a name for it (Xbox or Function ID), or the frame has.
    fn named(&self, p: u32) -> bool {
        self.map.get(&p).is_some_and(|r| !r.name.is_empty()) || self.extra.contains_key(&p)
    }

    /// Shared callees (PC mapped to Xbox) plus shared strings.
    fn sim(&self, p: u32, x: u32) -> usize {
        let (Some(pf), Some(xf)) = (self.pc.get(&p), self.xb.get(&x)) else {
            return 0;
        };
        let pcs: BTreeSet<u32> = pf.calls.iter().filter_map(|&c| self.xbox_of(c)).collect();
        let xcs: BTreeSet<u32> = xf.calls.iter().copied().collect();
        pcs.intersection(&xcs).count() + pf.strings.intersection(&xf.strings).count()
    }

    fn align_calls(&self, addr: u32) -> Vec<Option<(usize, Kind)>> {
        let (a, b) = (self.pc_calls(addr), self.xb_calls(self.xbox_of(addr)));
        align(
            a.len(),
            b.len(),
            |i, j| self.xbox_of(a[i]) == Some(b[j]),
            |i, j| self.sim(a[i], b[j]),
        )
    }

    /// Visits every call site of the tree below `addr`, depth first:
    /// `f(depth, position, callee, alignment proposal, recursive)`.
    fn walk(
        &self,
        addr: u32,
        depth: usize,
        path: &mut Vec<u32>,
        f: &mut impl FnMut(usize, usize, u32, Option<(u32, Kind)>, bool),
    ) {
        if depth >= MAX_DEPTH {
            return;
        }
        let calls = self.pc_calls(addr);
        let xcalls = self.xb_calls(self.xbox_of(addr));
        let aligned = self.align_calls(addr);
        path.push(addr);
        for (k, &c) in calls.iter().enumerate() {
            let prop = aligned[k].map(|(j, kind)| (xcalls[j], kind));
            let recursive = path.contains(&c);
            f(depth + 1, k + 1, c, prop, recursive);
            if !recursive {
                self.walk(c, depth + 1, path, f);
            }
        }
        path.pop();
    }

    /// Accepts alignment proposals (module docs) until none is new.
    fn resolve(&mut self) {
        loop {
            let taken: HashSet<u32> = self
                .map
                .values()
                .filter_map(|r| r.xbox)
                .chain(self.extra.values().map(|e| e.0))
                .collect();
            // pc -> proposals (None: a call site with no proposal).
            let mut props: BTreeMap<u32, BTreeSet<Option<(u32, Kind)>>> = BTreeMap::new();
            self.walk(ROOT_PC, 0, &mut Vec::new(), &mut |_, _, c, prop, _| {
                if !self.named(c) {
                    let p = prop.filter(|(x, k)| *k != Kind::Anchor && !taken.contains(x));
                    props.entry(c).or_default().insert(p);
                }
            });
            let mut by_xb: HashMap<u32, Vec<(u32, &'static str)>> = HashMap::new();
            for (c, ps) in &props {
                let xs: BTreeSet<u32> = ps.iter().flatten().map(|p| p.0).collect();
                if ps.contains(&None) || xs.len() != 1 {
                    continue;
                }
                let x = *xs.iter().next().unwrap();
                let tier = if ps.iter().flatten().any(|p| matches!(p.1, Kind::Sim(_))) {
                    "align-sim"
                } else {
                    "align-gap"
                };
                by_xb.entry(x).or_default().push((*c, tier));
            }
            let mut added = 0;
            for (x, cs) in by_xb {
                if let [(c, tier)] = cs.as_slice() {
                    self.extra.insert(*c, (x, tier));
                    added += 1;
                }
            }
            if added == 0 {
                break;
            }
        }
    }

    /// The table's row for a PC function.
    fn row(&self, p: u32) -> MapRow {
        let mut row = self.map.get(&p).cloned().unwrap_or_default();
        if row.name.is_empty() {
            if let Some(&(x, tier)) = self.extra.get(&p) {
                row.xbox = Some(x);
                row.name = self
                    .xb
                    .get(&x)
                    .map(|f| sanitize(&f.name))
                    .unwrap_or_default();
                row.tier = tier.into();
            }
        }
        row
    }
}

fn main() -> R<()> {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 5 && a.len() != 6 {
        return Err(
            "usage: frame <pc-dir> <xb-dir> <engine_map.tsv> <frame.tsv> [<report.tsv>]".into(),
        );
    }
    let (pcd, xbd) = (Path::new(&a[1]), Path::new(&a[2]));
    let (map, map_header) = load_map(Path::new(&a[3]))?;

    let mut pc = HashMap::new();
    for r in rows(&pcd.join("functions.tsv"))? {
        pc.insert(
            hex(&r[0]),
            PcFn {
                calls: list(&r[6]),
                strings: strings(&r[8]),
            },
        );
    }
    let mut xb: HashMap<u32, XbFn> = HashMap::new();
    for r in rows(&xbd.join("xb_funcs.tsv"))? {
        xb.insert(
            hex(&r[0]),
            XbFn {
                name: r[4].clone(),
                calls: list(r.get(6).map(String::as_str).unwrap_or("")),
                strings: strings(r.get(7).map(String::as_str).unwrap_or("")),
            },
        );
    }
    let helpers: HashSet<u32> = xb
        .iter()
        .filter(|(_, f)| ppc_helper(&f.name))
        .map(|(&x, _)| x)
        .collect();
    for f in xb.values_mut() {
        f.calls.retain(|x| !helpers.contains(x));
    }
    let mut extra = HashMap::new();
    let map_xb: HashSet<u32> = map.values().filter_map(|r| r.xbox).collect();
    for &(p, x) in CONFIRMED {
        if map_xb.contains(&x) {
            return Err(format!(
                "{x:08x}: confirmed pair, but the map gives it to another function"
            )
            .into());
        }
        if let Some(r) = map.get(&p).filter(|r| !r.name.is_empty()) {
            return Err(format!("{p:08x}: confirmed pair, but the map names it {}", r.name).into());
        }
        if !xb.contains_key(&x) {
            return Err(format!("{x:08x}: confirmed pair, not an Xbox function").into());
        }
        extra.insert(p, (x, "frame"));
    }
    let mut ctx = Ctx { pc, xb, map, extra };
    ctx.resolve();

    let mut nodes = vec![Node {
        depth: 0,
        call: 0,
        addr: ROOT_PC,
        row: ctx.row(ROOT_PC),
        note: "",
    }];
    ctx.walk(
        ROOT_PC,
        0,
        &mut Vec::new(),
        &mut |depth, call, c, _, recursive| {
            nodes.push(Node {
                depth,
                call,
                addr: c,
                row: ctx.row(c),
                note: if recursive { "recursive" } else { "" },
            });
        },
    );

    let mut s = String::new();
    for h in &map_header {
        let _ = writeln!(s, "# Map: {h}");
    }
    let _ = writeln!(
        s,
        "# Call tree of Main::OnIdle (Xbox PDB, PC 0086e650) to depth {MAX_DEPTH}, direct calls in call order; \
         generated by research/engine-map (frame), see its README"
    );
    let _ = writeln!(
        s,
        "seq\tdepth\tcall\taddress\tname\tname_tier\txbox\tunit\tsubsystem\tnote"
    );
    let mut per_depth: BTreeMap<usize, [usize; 4]> = BTreeMap::new();
    for (i, n) in nodes.iter().enumerate() {
        let e = per_depth.entry(n.depth).or_default();
        e[0] += 1;
        e[1] += (!n.row.name.is_empty()) as usize;
        e[2] += (n.row.tier == "frame") as usize;
        e[3] += n.row.tier.starts_with("align") as usize;
        let _ = writeln!(
            s,
            "{}\t{}\t{}\t{:08x}\t{}\t{}\t{}\t{}\t{}\t{}",
            i + 1,
            n.depth,
            n.call,
            n.addr,
            n.row.name,
            n.row.tier,
            n.row.xbox.map(|x| format!("{x:08x}")).unwrap_or_default(),
            n.row.unit,
            n.row.subsystem,
            n.note
        );
    }
    fs::write(&a[4], s)?;
    for (d, [n, named, conf, al]) in &per_depth {
        println!(
            "depth {d}: {n} rows, {named} named ({conf} confirmed for the frame, {al} aligned)"
        );
    }

    if let Some(rep) = a.get(5) {
        fs::write(rep, report(&ctx))?;
    }
    Ok(())
}

/// The root's alignment, both lists merged in order, for review.
fn report(ctx: &Ctx) -> String {
    let calls = ctx.pc_calls(ROOT_PC);
    let xcalls = ctx.xb_calls(Some(ROOT_XB));
    let aligned = ctx.align_calls(ROOT_PC);
    let xname = |x: u32| ctx.xb.get(&x).map(|f| f.name.as_str()).unwrap_or("");
    let mut s = String::from("pc_call\tpc\tpc_name\tkind\txb_call\txb\txb_name\n");
    let mut next_j = 0;
    for (k, &c) in calls.iter().enumerate() {
        let pn = ctx.row(c).name;
        match aligned[k] {
            Some((j, kind)) => {
                for (jj, &x) in xcalls.iter().enumerate().take(j).skip(next_j) {
                    let _ = writeln!(s, "\t\t\txb-only\t{}\t{x:08x}\t{}", jj + 1, xname(x));
                }
                next_j = j + 1;
                let _ = writeln!(
                    s,
                    "{}\t{c:08x}\t{pn}\t{kind:?}\t{}\t{:08x}\t{}",
                    k + 1,
                    j + 1,
                    xcalls[j],
                    xname(xcalls[j])
                );
            }
            None => {
                let _ = writeln!(s, "{}\t{c:08x}\t{pn}\tpc-only\t\t\t", k + 1);
            }
        }
    }
    for (jj, &x) in xcalls.iter().enumerate().skip(next_j) {
        let _ = writeln!(s, "\t\t\txb-only\t{}\t{x:08x}\t{}", jj + 1, xname(x));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligns_anchors_similar_calls_and_single_gaps() {
        // PC:   A x B y C z w D
        // Xbox: A t x' B y' C z' w' u D
        // A, B, C, D anchors; x ~ x' by similarity; y alone in its gap;
        // z, w stay open (gap lengths differ).
        let pc = ["A", "x", "B", "y", "C", "z", "w", "D"];
        let xb = ["A", "t", "x'", "B", "y'", "C", "z'", "w'", "u", "D"];
        let got = align(
            pc.len(),
            xb.len(),
            |i, j| pc[i] == xb[j],
            |i, j| if pc[i] == "x" && xb[j] == "x'" { 3 } else { 0 },
        );
        assert_eq!(got[0], Some((0, Kind::Anchor)));
        assert_eq!(got[1], Some((2, Kind::Sim(3))));
        assert_eq!(got[2], Some((3, Kind::Anchor)));
        assert_eq!(got[3], Some((4, Kind::Gap)));
        assert_eq!(got[4], Some((5, Kind::Anchor)));
        assert_eq!(got[5], None);
        assert_eq!(got[6], None);
        assert_eq!(got[7], Some((9, Kind::Anchor)));
    }

    #[test]
    fn similarity_below_threshold_is_not_a_pair() {
        let got = align(
            1,
            2,
            |_, _| false,
            |_, j| if j == 1 { MIN_SIM - 1 } else { 0 },
        );
        assert_eq!(got[0], None);
    }

    #[test]
    fn confirmed_pairs_are_unique() {
        let pcs: HashSet<u32> = CONFIRMED.iter().map(|p| p.0).collect();
        let xbs: HashSet<u32> = CONFIRMED.iter().map(|p| p.1).collect();
        assert_eq!(pcs.len(), CONFIRMED.len());
        assert_eq!(xbs.len(), CONFIRMED.len());
    }
}
