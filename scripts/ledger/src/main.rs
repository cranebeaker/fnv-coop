//! Function ledger (docs/ENGINE_PORT_PLAN.md, Phase 0).
//!
//! ```text
//! cargo run --release --manifest-path scripts/ledger/Cargo.toml [-- --check] [--tsv <file>]
//! ```
//!
//! Reads the engine map (`research/engine-map/engine_map.tsv`, one row per
//! function of FalloutNV.exe 1.4.0.525) and scans the Rust sources in
//! `crates/` and `viewer/` for executable addresses. Each function gets one
//! status:
//!
//! - `translated`: an address in it is cited on a translation marker line in
//!   an engine crate (`crates/engine`, `crates/engine-*`; ADR-0003:
//!   `Translated from <addr>` or `(decompiled, FalloutNV.exe ...)`; when the
//!   marker line itself holds no address, the line above counts). A marker
//!   elsewhere (the older viewer crates `crates/world`, `crates/physics`, ...)
//!   does not count: the engine still has to translate that function, so it
//!   is `traced`. This is the rule of the task queue ([`Func::in_engine`]).
//! - `static init (table)`: a static initializer, or an `atexit` destructor,
//!   run from a generated table (`crates/engine-core/data/static_initializers.tsv`,
//!   `crates/engine-core/data/atexit_destructors.tsv`; research/initgen): engine
//!   coverage, but not a hand translation.
//! - `traced`: an address in it is cited anywhere else in Rust, including a
//!   translation marker outside the engine crates (the viewer describes it,
//!   the engine still needs it).
//! - `platform` / `library`: the platform layer stands in for it (operating
//!   system glue, the Direct3D 9 / D3DX libraries, Xbox system services), or
//!   it is compiler or runtime-library code (`base_status`). The game's
//!   renderer is game code (ADR-0007).
//! - `open`: none of the above.
//!
//! Functions Ghidra never created join the map from
//! `research/engine-map/ghidra_gaps.tsv` (`add_gaps`). Functions the map
//! placed in a subsystem but in no unit get one from
//! `research/engine-map/unit_overrides.tsv` (an RTTI vtable slot of a class
//! whose unit is known) or else a run unit (`run_units`).
//!
//! An address counts for the function whose entry is the nearest at or
//! below it, if the address lies within that function's size. Addresses
//! outside every function (data, or code outside Ghidra's functions) are
//! counted separately.
//!
//! Writes `docs/LEDGER.md`. `--check` writes nothing and fails if the file
//! is out of date. `--tsv <file>` also writes the per-function ledger
//! (address, size, name, subsystem, unit, status, Rust locations).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

mod units;

/// `.text` of FalloutNV.exe 1.4.0.525 (image base 0x400000).
const TEXT: (u32, u32) = (0x0040_1000, 0x00fd_e600);

/// Default status of a subsystem when no Rust cites the function.
/// `library`: compiler-generated code and the C/C++ runtime and zlib.
/// `platform`: the platform layer stands in for it (docs/ENGINE_PORT_PLAN.md,
/// "Which parts get translated"): the Direct3D 9 / D3DX and other Xbox and
/// DirectX libraries, system, file, thread and device layers, Xbox system
/// services, movie playback and the debug transport, except the units in
/// [`LOGIC_UNITS`]. The game's renderer (`NiXenonRenderer`, the Xbox name of
/// the PC `NiDX9Renderer`, and `BSShader`) is game code: it is translated and
/// its Direct3D 9 calls go to our own Direct3D 9 layer (ADR-0007), so only the
/// functions in [`PLATFORM_FUNCS`] stay `platform` there.
fn base_status(subsystem: &str) -> &'static str {
    match subsystem {
        "compiler: static init/exit" | "LIBCMT" | "libcpmt" | "OLDNAMES" | "zLib" => "library",
        "d3d9"
        | "d3dx9"
        | "xgraphics"
        | "XAPILIB"
        | "xaudio2"
        | "x3Daudio"
        | "xonline"
        | "xnet"
        | "xmcore"
        | "XBOXKRNL"
        | "xbdm"
        | "xmp"
        | "binkxenon"
        | "BSSystem"
        | "BSSystemUtilities"
        | "BSDevices"
        | "BSMovie"
        | "RemoteLog"
        | "BSDiag"
        | "fallout shared/steam" => "platform",
        _ => "open",
    }
}

/// Functions in game-code subsystems that stay `platform` although their unit
/// is game code: `00ee6e2e` is a 6-byte `JMP` through the D3DX9_38.DLL import
/// `D3DXGetVertexShaderProfile` (an import thunk; the map's call-graph name
/// for it, in the renderer, is wrong). Our Direct3D 9 layer supplies D3DX.
const PLATFORM_FUNCS: &[u32] = &[0x00ee_6e2e];

/// Functions in platform subsystems that are game code, so they are `open`
/// although their subsystem or unit stays `platform` (read in the exe,
/// 2026-10-17): archive and file helpers without OS calls in `BSSystem`
/// (archive name hashing, buffered reads, `ArchiveManager::TrimFileName`,
/// `BSFile` buffers), the `NiTMap<const char*, char*>` template code in the
/// movie player object (including the two slots the map named into
/// `BSDiag`, placed through `unit_overrides.tsv`), and a 7-byte
/// return-the-argument stub in Havok vtables that the map named into
/// `RemoteLog`; and the tasklet manager (2026-10-20: `BSTaskletManager::QInstance`,
/// `CreateTaskGroup`, the manager's constructor `00b01d50` and base `00b02720`, its worker and
/// starter threads and their loop, list and hook helpers), whose threads the thread host runs.
/// `Controls::Poll` (`00a23010`) and its empty end `00a232c0` in `BSDevices` read the devices
/// through the DirectInput layer each frame.
/// Only their OS calls go to the platform layer.
const GAME_FUNCS: &[u32] = &[
    0x00a2_3010,
    0x00a2_32c0,
    0x00af_cfe0,
    0x00af_d390,
    0x00af_d4e0,
    0x00af_d860,
    0x00af_d960,
    0x00af_dbc0,
    0x00af_de50,
    0x00af_f180,
    0x00af_fd80,
    0x00af_ff50,
    0x00b0_0880,
    0x00b0_08a0,
    0x00b0_08b0,
    0x00b0_08e0,
    0x00b0_08f0,
    0x00b0_0970,
    0x00b0_0a00,
    0x00b0_0a80,
    0x00b0_0c90,
    0x00b0_0cf0,
    0x00b0_0f20,
    0x00b0_1020,
    0x00b0_1040,
    0x00b0_1070,
    0x00b0_11b0,
    0x00b0_11e0,
    0x00b0_1220,
    0x00b0_12a0,
    0x00b0_12f0,
    0x00b0_14b0,
    0x00b0_1800,
    0x00b0_1bc0,
    0x00b0_1be0,
    0x00b0_1c00,
    0x00b0_1c20,
    0x00b0_1d50,
    0x00b0_2130,
    0x00b0_23b0,
    0x00b0_23f0,
    0x00b0_2420,
    0x00b0_2460,
    0x00b0_2670,
    0x00b0_2720,
    0x00b0_29a0,
    0x00dc_1660,
    0x00ec_1a40,
    0x00ec_1a60,
    0x00ec_1a90,
    0x00ec_1c30,
    0x00ec_1cc0,
    0x00ec_1d50,
    0x00ec_1dc0,
    0x00ec_1df0,
    0x00ec_1e40,
    0x00ec_1ee0,
    0x00ec_2ff0,
];

/// Units inside platform subsystems that are game logic rather than OS calls,
/// so they are translated like game code: archive lookup and the file layer
/// above the OS reads (which file wins), and D3DX's maths. SpeedTree is not a
/// platform subsystem at all: its tree building, splines, random numbers and
/// wind are maths that decide every tree's shape. Only the last step of each
/// (handing data to the OS) is a platform stand-in.
const LOGIC_UNITS: &[&str] = &[
    "BSSystem/archive.cpp",
    "BSSystem/bsfile.cpp",
    "BSSystem/bsfilecache.cpp",
    "BSSystem/bssearchpath.cpp",
    "BSSystem/filefinder.cpp",
    "d3dx9/d3dxmath.cpp",
];

/// [`base_status`] for one function: a [`LOGIC_UNITS`] unit is `open`.
fn unit_status(subsystem: &str, unit: &str) -> &'static str {
    if LOGIC_UNITS.contains(&unit) {
        "open"
    } else {
        base_status(subsystem)
    }
}

/// [`unit_status`] for one function, with the [`PLATFORM_FUNCS`] and
/// [`GAME_FUNCS`] exceptions.
fn func_status(addr: u32, subsystem: &str, unit: &str) -> &'static str {
    if PLATFORM_FUNCS.contains(&addr) {
        "platform"
    } else if GAME_FUNCS.contains(&addr) {
        "open"
    } else {
        unit_status(subsystem, unit)
    }
}

/// The status of a function run from a generated table ([`TABLES`]: static
/// initializers, `atexit` destructors): covered by the engine, but not a hand
/// translation.
const TABLE: &str = "static init (table)";

const STATUSES: [&str; 6] = ["translated", TABLE, "traced", "platform", "library", "open"];

struct Func {
    addr: u32,
    size: u32,
    name: String,
    tier: String,
    unit: String,
    subsystem: String,
    placed: String,
    translated: BTreeSet<String>,
    /// Rows of the static-initializer table that run this function.
    table: BTreeSet<String>,
    replaced: BTreeSet<String>,
    traced: BTreeSet<String>,
    incomplete: BTreeSet<String>,
}

/// Whether a Rust location (`crates/...:line`) is in one of the engine crates:
/// `crates/engine` (the facade) or `crates/engine-*` (the core and the parts).
fn in_engine_crate(loc: &str) -> bool {
    loc.starts_with("crates/engine/") || loc.starts_with("crates/engine-")
}

impl Func {
    /// The status when no Rust cites the function ([`func_status`]).
    fn base(&self) -> &'static str {
        func_status(self.addr, &self.subsystem, &self.unit)
    }

    /// Translated, run from the static-initializer table or replaced inside
    /// the engine crates ([`in_engine_crate`]): the engine has it. A translation elsewhere (an older
    /// marker in `crates/world`) still needs its engine version.
    fn in_engine(&self) -> bool {
        !self.partial_in_engine()
            && self
                .translated
                .iter()
                .chain(&self.table)
                .chain(&self.replaced)
                .any(|l| in_engine_crate(l))
    }

    fn partial_in_engine(&self) -> bool {
        self.incomplete.iter().any(|l| in_engine_crate(l))
    }

    fn queue_status(&self) -> &'static str {
        if self.partial_in_engine() {
            "partial"
        } else if self.in_engine() {
            "done"
        } else if self.base() != "open" {
            self.base()
        } else {
            self.status()
        }
    }

    /// Whether a translation marker sits in an engine crate. A marker only in
    /// another crate (the older viewer crates) does not count as a translation.
    fn translated_in_engine(&self) -> bool {
        self.translated.iter().any(|l| in_engine_crate(l))
    }

    fn status(&self) -> &'static str {
        if self.translated_in_engine() {
            "translated"
        } else if !self.table.is_empty() {
            TABLE
        } else if !self.replaced.is_empty() {
            "platform"
        } else if !self.traced.is_empty() || !self.translated.is_empty() {
            // A translation marker outside the engine crates only describes
            // the function: `traced`, the engine still needs it.
            "traced"
        } else {
            self.base()
        }
    }
}

/// Executable addresses on a line: 8 hex digits starting `00` (optionally
/// `0x`-prefixed), not part of a longer word, inside `.text`.
fn addresses(line: &str) -> Vec<u32> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 8 <= b.len() {
        let prev_ok = i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
        let next_ok = i + 8 == b.len() || !(b[i + 8].is_ascii_alphanumeric() || b[i + 8] == b'_');
        let hex = b[i..i + 8].iter().all(|c| c.is_ascii_hexdigit());
        let mut is_0x = false;
        if i >= 2 && (&b[i - 2..i] == b"0x" || &b[i - 2..i] == b"0X") {
            is_0x = i == 2 || !(b[i - 3].is_ascii_alphanumeric() || b[i - 3] == b'_');
        }
        if hex && (prev_ok || is_0x) && next_ok && &b[i..i + 2] == b"00" {
            if let Ok(v) = u32::from_str_radix(&line[i..i + 8], 16) {
                if (TEXT.0..TEXT.1).contains(&v) {
                    out.push(v);
                    i += 8;
                    continue;
                }
            }
        }
        i += 1;
    }
    out
}

/// How a citation counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cite {
    Traced,
    /// An explicit gap, kept in the engine task queue even with a marker.
    Incomplete,
    /// On a translation marker (ADR-0003).
    Translated,
    /// On a `Platform: replaces <addr>` marker: Rust platform code stands in
    /// for the function (the allocator, file and thread layers).
    Replaced,
}

fn marker(line: &str) -> Option<Cite> {
    if line.contains("Translation incomplete:") {
        Some(Cite::Incomplete)
    } else if line.contains("Translated from") || line.contains("decompiled, FalloutNV") {
        Some(Cite::Translated)
    } else if line.contains("Platform: replaces") {
        Some(Cite::Replaced)
    } else {
        None
    }
}

/// (address, kind, line) for every citation in one file.
fn scan(text: &str) -> Vec<(u32, Cite, usize)> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let here = addresses(l);
        let m = marker(l);
        if let (Some(k), true, true) = (m, here.is_empty(), i > 0) {
            for a in addresses(lines[i - 1]) {
                out.push((a, k, i));
            }
        }
        for a in here {
            out.push((a, m.unwrap_or(Cite::Traced), i + 1));
        }
    }
    out
}

/// The generated table of the static initializers
/// (`crates/engine-core/data/static_initializers.tsv`, research/initgen): each
/// row runs the `__cinit` entry in its first column through the shape
/// executors of `crates/engine-core/src/static_init.rs`. It counts toward
/// coverage under its own status ([`TABLE`]), not as a hand translation.
const INITIALIZER_TABLE: &str = "crates/engine-core/data/static_initializers.tsv";

/// The generated table of the `atexit` destructors
/// (`crates/engine-core/data/atexit_destructors.tsv`, research/initgen `--atexit`):
/// each row is the translation of the function in its first column
/// (`crates/engine-core/src/static_exit.rs`), counted exactly as the rows of
/// [`INITIALIZER_TABLE`].
const EXIT_TABLE: &str = "crates/engine-core/data/atexit_destructors.tsv";

/// Every generated table whose rows count as [`TABLE`].
const TABLES: [&str; 2] = [INITIALIZER_TABLE, EXIT_TABLE];

/// (entry address, line) of every row of the initializer table.
fn table_rows(text: &str) -> Vec<(u32, usize)> {
    text.lines()
        .enumerate()
        .filter(|(_, l)| !l.starts_with('#') && !l.trim().is_empty())
        .filter_map(|(i, l)| {
            let first = l.split('\t').next()?;
            (first.len() == 8)
                .then(|| u32::from_str_radix(first, 16).ok())
                .flatten()
                .map(|a| (a, i + 1))
        })
        .collect()
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if p.is_dir() {
            if name != "target" && !name.starts_with('.') {
                rust_files(&p, out);
            }
        } else if name.ends_with(".rs") {
            out.push(p);
        }
    }
}

fn load_map(path: &Path) -> Result<(Vec<Func>, Vec<String>), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut header = Vec::new();
    let mut funcs = Vec::new();
    let mut cols: Vec<String> = Vec::new();
    for line in text.lines() {
        if let Some(h) = line.strip_prefix('#') {
            header.push(h.trim().to_string());
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if cols.is_empty() {
            cols = f.iter().map(|s| s.to_string()).collect();
            continue;
        }
        let get = |k: &str| -> String {
            cols.iter()
                .position(|c| c == k)
                .and_then(|i| f.get(i))
                .map(|s| s.to_string())
                .unwrap_or_default()
        };
        let addr = u32::from_str_radix(&get("address"), 16).map_err(|e| format!("{line}: {e}"))?;
        let subsystem = match get("subsystem") {
            s if s.is_empty() => "(unplaced)".to_string(),
            s => s,
        };
        funcs.push(Func {
            addr,
            size: get("size").parse().unwrap_or(0),
            name: get("name"),
            tier: get("name_tier"),
            unit: get("unit"),
            subsystem,
            placed: get("placed"),
            translated: BTreeSet::new(),
            table: BTreeSet::new(),
            replaced: BTreeSet::new(),
            traced: BTreeSet::new(),
            incomplete: BTreeSet::new(),
        });
    }
    funcs.sort_by_key(|f| f.addr);
    match fs::read_to_string(path.with_file_name(GAPS)) {
        Ok(t) => {
            add_gaps(&mut funcs, &t)?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("{GAPS}: {e}")),
    }
    match fs::read_to_string(path.with_file_name(OVERRIDES)) {
        Ok(t) => {
            apply_overrides(&mut funcs, &t)?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("{OVERRIDES}: {e}")),
    }
    run_units(&mut funcs);
    Ok((funcs, header))
}

/// Functions the exe runs that Ghidra never created, so the map has no row
/// for them (`research/engine-map/ghidra_gaps.tsv`, next to `engine_map.tsv`):
/// code reached only from data (vtables, tables, `hkSingletonInitNode`
/// records, the `__cinit` table), from pointer immediates or from calls and
/// tail jumps in other such code. Columns: address, size, unit, subsystem,
/// evidence; the unit and subsystem come from the neighbouring functions of
/// the map (empty when they differ). Such a function joins the map with
/// `placed` = `gap`.
const GAPS: &str = "ghidra_gaps.tsv";

/// Adds every row of [`GAPS`] to `funcs` (kept sorted). A row at or inside a
/// function of the map, or repeated, is an error.
fn add_gaps(funcs: &mut Vec<Func>, text: &str) -> Result<usize, String> {
    let mut add = Vec::new();
    for line in text
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty() && !l.starts_with("address\t"))
    {
        let f: Vec<&str> = line.split('\t').collect();
        let (Some(a), Some(size), Some(unit), Some(sub)) =
            (f.first(), f.get(1), f.get(2), f.get(3))
        else {
            return Err(format!("{GAPS}: bad row {line:?}"));
        };
        let addr = u32::from_str_radix(a, 16).map_err(|e| format!("{GAPS}: {line:?}: {e}"))?;
        let size: u32 = size.parse().map_err(|e| format!("{GAPS}: {line:?}: {e}"))?;
        let k = funcs.partition_point(|f| f.addr <= addr);
        if let Some(f) = k.checked_sub(1).map(|k| &funcs[k]) {
            if addr < f.addr + f.size.max(1) {
                return Err(format!(
                    "{GAPS}: {a} lies in the map's function {:08x}",
                    f.addr
                ));
            }
        }
        add.push(Func {
            addr,
            size,
            name: String::new(),
            tier: String::new(),
            unit: unit.to_string(),
            subsystem: if sub.is_empty() {
                "(unplaced)".to_string()
            } else {
                sub.to_string()
            },
            placed: "gap".into(),
            translated: BTreeSet::new(),
            table: BTreeSet::new(),
            replaced: BTreeSet::new(),
            traced: BTreeSet::new(),
            incomplete: BTreeSet::new(),
        });
    }
    let n = add.len();
    funcs.append(&mut add);
    funcs.sort_by_key(|f| f.addr);
    if let Some(w) = funcs.windows(2).find(|w| w[0].addr == w[1].addr) {
        return Err(format!("{GAPS}: {:08x} is listed twice", w[0].addr));
    }
    Ok(n)
}

/// Unit placements applied on top of the map (`research/engine-map/
/// unit_overrides.tsv`, next to `engine_map.tsv`): functions the map placed
/// in a subsystem but in no unit (or in a unit by a wrong name match), which
/// the exe shows belong to an existing unit's class (an RTTI vtable slot of a
/// class whose other methods are in that unit). Columns: address, unit, subsystem, replaces, evidence.
/// `replaces` is empty for a function the map left without a unit; to
/// correct a unit the map gave by a wrong name match it must name that unit.
const OVERRIDES: &str = "unit_overrides.tsv";

/// Sets the unit and subsystem of every row of [`OVERRIDES`] (`placed`
/// becomes `vtable`). A row whose `replaces` does not equal the function's
/// unit in the map (empty for none), or for an address that is no function
/// of the map, is an error: the map's placements change only where a row
/// names them.
fn apply_overrides(funcs: &mut [Func], text: &str) -> Result<usize, String> {
    let mut n = 0;
    for line in text
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty() && !l.starts_with("address\t"))
    {
        let f: Vec<&str> = line.split('\t').collect();
        let (Some(a), Some(unit), Some(sub), Some(replaces)) =
            (f.first(), f.get(1), f.get(2), f.get(3))
        else {
            return Err(format!("{OVERRIDES}: bad row {line:?}"));
        };
        let addr = u32::from_str_radix(a, 16).map_err(|e| format!("{OVERRIDES}: {line:?}: {e}"))?;
        let i = funcs
            .binary_search_by_key(&addr, |f| f.addr)
            .map_err(|_| format!("{OVERRIDES}: {a} is no function of the map"))?;
        let func = &mut funcs[i];
        if func.unit != *replaces {
            return Err(format!(
                "{OVERRIDES}: {a} has unit {:?}, the row replaces {replaces:?}",
                func.unit
            ));
        }
        func.unit = unit.to_string();
        func.subsystem = sub.to_string();
        func.placed = "vtable".into();
        n += 1;
    }
    Ok(n)
}

/// Functions the map placed in a subsystem but in no unit get a synthetic
/// unit per run of such functions with the same subsystem:
/// `<subsystem>/run_<first address>`, so they have a file and an owner
/// like the others. Library subsystems are left alone; a run in a platform
/// subsystem keeps that subsystem's status (except [`GAME_FUNCS`]).
///
/// [`GAPS`] functions (`placed` = `gap`) do not change the map's runs, so
/// existing unit files keep their names: one without a unit joins the run
/// of the map function before it, else the run of the map function after
/// it (same subsystem), else a run of its own with the gap functions next
/// to it.
fn run_units(funcs: &mut [Func]) {
    let mut current: Option<(String, String)> = None;
    for f in funcs.iter_mut() {
        if f.placed == "gap" {
            continue;
        }
        if !f.unit.is_empty() || base_status(&f.subsystem) == "library" {
            current = None;
            continue;
        }
        match &current {
            Some((sub, unit)) if *sub == f.subsystem => f.unit = unit.clone(),
            _ => {
                let unit = format!("{}/run_{:08x}", f.subsystem, f.addr);
                current = Some((f.subsystem.clone(), unit.clone()));
                f.unit = unit;
            }
        }
    }
    let run_of = |f: &Func, sub: &str| {
        (f.placed != "gap" && f.unit.starts_with(&format!("{sub}/run_"))).then(|| f.unit.clone())
    };
    let mut own: Option<(String, String)> = None;
    for i in 0..funcs.len() {
        let f = &funcs[i];
        if f.placed != "gap" {
            own = None;
            continue;
        }
        if !f.unit.is_empty() || base_status(&f.subsystem) == "library" {
            continue;
        }
        let sub = f.subsystem.clone();
        let before = funcs[..i].iter().rev().find(|g| g.placed != "gap");
        let after = funcs[i + 1..].iter().find(|g| g.placed != "gap");
        let unit = before
            .and_then(|g| run_of(g, &sub))
            .or_else(|| after.and_then(|g| run_of(g, &sub)))
            .unwrap_or_else(|| match &own {
                Some((s, u)) if *s == sub => u.clone(),
                _ => {
                    let u = format!("{sub}/run_{:08x}", funcs[i].addr);
                    own = Some((sub.clone(), u.clone()));
                    u
                }
            });
        funcs[i].unit = unit;
    }
}

#[derive(Default, Clone)]
struct Counts {
    n: usize,
    bytes: u64,
    named: usize,
    by: [usize; 6],
}

impl Counts {
    fn add(&mut self, f: &Func) {
        self.n += 1;
        self.bytes += f.size as u64;
        self.named += (!f.name.is_empty()) as usize;
        let i = STATUSES.iter().position(|s| *s == f.status()).unwrap();
        self.by[i] += 1;
    }
}

fn pct(a: usize, b: usize) -> String {
    if b == 0 {
        "-".into()
    } else {
        format!("{:.1}%", 100.0 * a as f64 / b as f64)
    }
}

fn render(funcs: &[Func], header: &[String], unmapped: usize, cites: usize) -> String {
    let mut total = Counts::default();
    let mut subs: BTreeMap<&str, Counts> = BTreeMap::new();
    let mut tiers: BTreeMap<&str, usize> = BTreeMap::new();
    let mut placed: BTreeMap<&str, usize> = BTreeMap::new();
    for f in funcs {
        total.add(f);
        subs.entry(&f.subsystem).or_default().add(f);
        *tiers
            .entry(if f.tier.is_empty() { "(none)" } else { &f.tier })
            .or_default() += 1;
        *placed.entry(&f.placed).or_default() += 1;
    }
    let game_fns: Vec<&Func> = funcs.iter().filter(|f| f.base() == "open").collect();
    let game = game_fns.len();
    let game_done = game_fns
        .iter()
        .filter(|f| matches!(f.status(), "translated" | TABLE | "traced"))
        .count();
    let mut s = String::new();
    let _ = writeln!(
        s,
        "# Function ledger\n\n\
         Generated by `scripts/ledger` (docs/ENGINE_PORT_PLAN.md, Phase 0); do not edit.\n\
         Regenerate with:\n\n\
         ```sh\ncargo run --release --manifest-path scripts/ledger/Cargo.toml\n```\n\n\
         Every function of FalloutNV.exe 1.4.0.525 from the engine map\n\
         (`research/engine-map/engine_map.tsv`; how it is made: `research/engine-map/README.md`)\n\
         gets one status. Names, units and subsystems come from the Xbox 360 prototype\n\
         (Xbox PDB, ADR-0002), matched to PC functions as described there.\n"
    );
    if !header.is_empty() {
        let _ = writeln!(s, "Map provenance:\n");
        for h in header {
            let _ = writeln!(s, "- {h}");
        }
        let _ = writeln!(s);
    }
    let _ = writeln!(
        s,
        "| Status | Meaning |\n| --- | --- |\n\
         | translated | cited on a translation marker (ADR-0003) in an engine crate \
         (`crates/engine*`) |\n\
         | static init (table) | a static initializer or an `atexit` destructor run \
         from a generated table (`crates/engine-core/data/static_initializers.tsv`, \
         `crates/engine-core/data/atexit_destructors.tsv`; research/initgen), not a hand \
         translation |\n\
         | traced | cited elsewhere in Rust, including a translation marker outside the \
         engine crates (the viewer describes it, the engine still needs it) |\n\
         | platform | the platform layer stands in for it: OS glue, the Direct3D 9 / D3DX \
         libraries, Xbox system services (the renderer is game code, ADR-0007) |\n\
         | library | compiler-generated code, C/C++ runtime, zlib |\n\
         | open | none of the above |\n"
    );
    let _ = writeln!(s, "## Totals\n");
    let _ = writeln!(s, "| | Functions | Share |\n| --- | ---: | ---: |");
    for (i, st) in STATUSES.iter().enumerate() {
        let _ = writeln!(
            s,
            "| {st} | {} | {} |",
            total.by[i],
            pct(total.by[i], total.n)
        );
    }
    let _ = writeln!(s, "| **all** | **{}** | |", total.n);
    let partials: Vec<_> = funcs.iter().filter(|f| f.partial_in_engine()).collect();
    let _ = writeln!(
        s,
        "\nTranslation totals count engine-crate markers, not complete behavior; a marker \
         outside the engine crates counts as `traced`. **{}** engine functions\n\
         have explicit incomplete markers and remain `partial` in the task queue.\n",
        partials.len()
    );
    if !partials.is_empty() {
        let _ = writeln!(
            s,
            "| Partial engine function | Gap location |\n| --- | --- |"
        );
        for f in partials {
            let locations = f.incomplete.iter().cloned().collect::<Vec<_>>().join(", ");
            let _ = writeln!(s, "| `{:08x}` | `{locations}` |", f.addr);
        }
    }
    let _ = writeln!(
        s,
        "\nGame code (functions whose subsystem or unit is neither platform nor library): \
         **{game}** functions, of which {game_done} ({}) are translated, run from the \
         static-initializer table, or traced.\n",
        pct(game_done, game)
    );
    let _ = writeln!(
        s,
        "Rust cites {cites} distinct `.text` addresses; {unmapped} of them lie outside every \
         function in the map (mid-instruction data, or code no function covers).\n"
    );
    let _ = writeln!(
        s,
        "Names by tier (see the engine-map README for each tier's evidence):\n"
    );
    let _ = writeln!(s, "| Tier | Functions |\n| --- | ---: |");
    for (k, v) in &tiers {
        let _ = writeln!(s, "| {k} | {v} |");
    }
    let _ = writeln!(s, "\nHow the subsystem was found:\n");
    let _ = writeln!(s, "| Placed by | Functions |\n| --- | ---: |");
    for (k, v) in &placed {
        let _ = writeln!(s, "| {k} | {v} |");
    }
    let _ = writeln!(
        s,
        "\n## By subsystem\n\nLargest first. `Named` counts functions with an Xbox PDB \
         (or, in the runtime libraries, Function ID) name. KB is the size of the PC code.\n"
    );
    let _ = writeln!(
        s,
        "| Subsystem | Functions | KB | Named | translated | static init (table) | traced | platform | library | open | done |\n\
         | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    let mut rows: Vec<_> = subs.iter().collect();
    rows.sort_by(|a, b| b.1.n.cmp(&a.1.n).then(a.0.cmp(b.0)));
    for (name, c) in rows {
        let done = c.by[0] + c.by[1] + c.by[2];
        let base = if base_status(name) == "open" { c.n } else { 0 };
        let _ = writeln!(
            s,
            "| {name} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            c.n,
            c.bytes / 1024,
            c.named,
            c.by[0],
            c.by[1],
            c.by[2],
            c.by[3],
            c.by[4],
            c.by[5],
            if base == 0 {
                "-".into()
            } else {
                pct(done, base)
            }
        );
    }
    let _ = writeln!(
        s,
        "\n`done` is translated plus traced as a share of the subsystem; it is left out \
         (`-`) for platform and library subsystems."
    );
    s
}

/// One call site of the frame tree (`research/engine-map/frame.tsv`).
struct FrameRow {
    depth: usize,
    addr: u32,
    name: String,
}

/// Reads the frame table; a missing file gives no rows.
fn load_frame(path: &Path) -> Result<Vec<FrameRow>, String> {
    let Ok(text) = fs::read_to_string(path) else {
        return Ok(Vec::new());
    };
    let mut cols: Vec<&str> = Vec::new();
    let mut out = Vec::new();
    for line in text.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split('\t').collect();
        if cols.is_empty() {
            cols = f;
            continue;
        }
        let get = |k: &str| {
            cols.iter()
                .position(|c| *c == k)
                .and_then(|i| f.get(i))
                .copied()
                .unwrap_or("")
        };
        out.push(FrameRow {
            depth: get("depth").parse().map_err(|e| format!("{line}: {e}"))?,
            addr: u32::from_str_radix(get("address"), 16).map_err(|e| format!("{line}: {e}"))?,
            name: get("name").to_string(),
        });
    }
    Ok(out)
}

/// Status of a frame row: its function's ledger status (an address that is
/// no function's entry counts as `open`).
fn frame_status(funcs: &[Func], addr: u32) -> &'static str {
    match funcs.binary_search_by_key(&addr, |f| f.addr) {
        Ok(i) => funcs[i].status(),
        Err(_) => "open",
    }
}

/// The "Frame" section: coverage of the `Main::OnIdle` tree per depth, and
/// the depth-1 calls with their status.
fn render_frame(funcs: &[Func], rows: &[FrameRow]) -> String {
    let mut s = String::new();
    if rows.is_empty() {
        return s;
    }
    let _ = writeln!(
        s,
        "\n## Frame\n\n\
         The call tree of `Main::OnIdle` (Xbox PDB, PC `0086e650`), the game's per-frame \
         function, from `research/engine-map/frame.tsv` (docs/FRAME_SKELETON.md): direct \
         calls in call order to depth 3, one row per call site, so a function called twice \
         counts twice. Statuses as above.\n"
    );
    let _ = writeln!(
        s,
        "| Depth | Call sites | Functions | translated | static init (table) | traced | platform | library | open |\n\
         | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    let mut depths: BTreeMap<usize, ([usize; 6], BTreeSet<u32>)> = BTreeMap::new();
    for r in rows {
        let e = depths.entry(r.depth).or_default();
        let st = frame_status(funcs, r.addr);
        e.0[STATUSES.iter().position(|x| *x == st).unwrap()] += 1;
        e.1.insert(r.addr);
    }
    for (d, (by, fs)) in &depths {
        let _ = writeln!(
            s,
            "| {d} | {} | {} | {} | {} | {} | {} | {} | {} |",
            by.iter().sum::<usize>(),
            fs.len(),
            by[0],
            by[1],
            by[2],
            by[3],
            by[4],
            by[5]
        );
    }
    let _ = writeln!(
        s,
        "\nThe calls of `Main::OnIdle` itself (depth 1), in order:\n\n\
         | # | Address | Name (Xbox PDB) | Status |\n| ---: | --- | --- | --- |"
    );
    for (i, r) in rows.iter().filter(|r| r.depth == 1).enumerate() {
        let _ = writeln!(
            s,
            "| {} | `{:08x}` | {} | {} |",
            i + 1,
            r.addr,
            if r.name.is_empty() {
                "-".to_string()
            } else {
                format!("`{}`", r.name)
            },
            frame_status(funcs, r.addr)
        );
    }
    s
}

/// Live-system wiring is separate from executable translation coverage.
fn render_wiring(table: &str) -> String {
    let mut counts = [[0usize; 3]; 2];
    for line in table
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("seq\t"))
    {
        let fields: Vec<_> = line.split('\t').collect();
        let depth: usize = fields[1].parse().expect("wiring depth");
        let status = match fields[5] {
            "system" => 0,
            "partial" => 1,
            "open" => 2,
            other => panic!("unknown wiring status {other}"),
        };
        counts[depth - 1][status] += 1;
    }
    let mut out = String::from("\n### Frame system wiring\n\nEvery depth-1 and depth-2 call is listed in [frame_wiring.tsv](../research/engine-map/frame_wiring.tsv), generated and checked by `world::frame::coverage`. Partial means some behavior exists, with its placement or missing work stated in the row; it does not mean the function is fully executed in play. Queries and unsupported calls stay open. Translation counts above are separate.\n\n| Depth | system | partial | open |\n| ---: | ---: | ---: | ---: |\n");
    for (depth, c) in counts.iter().enumerate() {
        let _ = writeln!(out, "| {} | {} | {} | {} |", depth + 1, c[0], c[1], c[2]);
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let check = args.iter().any(|a| a == "--check");
    let tsv = args
        .iter()
        .position(|a| a == "--tsv")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let (mut funcs, header) = match load_map(&root.join("research/engine-map/engine_map.tsv")) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ledger: {e}");
            std::process::exit(2);
        }
    };
    let mut files = Vec::new();
    rust_files(&root.join("crates"), &mut files);
    rust_files(&root.join("viewer"), &mut files);
    let mut cited = BTreeSet::new();
    let mut unmapped = BTreeSet::new();
    for path in &files {
        let Ok(text) = fs::read_to_string(path) else {
            continue;
        };
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        for (a, kind, line) in scan(&text) {
            cited.insert(a);
            let k = funcs.partition_point(|f| f.addr <= a);
            let Some(f) = k.checked_sub(1).map(|k| &mut funcs[k]) else {
                unmapped.insert(a);
                continue;
            };
            if a >= f.addr + f.size.max(1) {
                unmapped.insert(a);
                continue;
            }
            let loc = format!("{rel}:{line}");
            match kind {
                Cite::Translated => f.translated.insert(loc),
                Cite::Replaced => f.replaced.insert(loc),
                Cite::Traced => f.traced.insert(loc),
                Cite::Incomplete => f.incomplete.insert(loc),
            };
        }
    }
    for table in TABLES {
        let Ok(text) = fs::read_to_string(root.join(table)) else {
            continue;
        };
        for (a, line) in table_rows(&text) {
            cited.insert(a);
            let k = funcs.partition_point(|f| f.addr <= a);
            match k.checked_sub(1).map(|k| &mut funcs[k]) {
                Some(f) if a < f.addr + f.size.max(1) => {
                    f.table.insert(format!("{table}:{line}"));
                }
                _ => {
                    unmapped.insert(a);
                }
            }
        }
    }
    if let Some(a) = args.first() {
        if !matches!(
            a.as_str(),
            "units" | "scaffold" | "queue" | "--check" | "--tsv"
        ) {
            eprintln!("ledger: unknown argument {a} (commands: units, scaffold, queue; options: --check, --tsv <file>)");
            std::process::exit(2);
        }
    }
    match args.first().map(String::as_str) {
        Some("units") => {
            match units::regenerate(&root) {
                Ok(n) => println!("ledger: {n} unit modules"),
                Err(e) => {
                    eprintln!("ledger: {e}");
                    std::process::exit(2);
                }
            }
            return;
        }
        Some("scaffold") => {
            // scaffold <unit or subsystem>...: create unit files for every
            // unit named, or every unit of a subsystem named.
            let want: BTreeSet<&str> = args[1..].iter().map(String::as_str).collect();
            let mut done = BTreeSet::new();
            for f in &funcs {
                if f.unit.is_empty()
                    || f.base() != "open"
                    || !(want.contains(f.unit.as_str()) || want.contains(f.subsystem.as_str()))
                {
                    continue;
                }
                if done.insert(f.unit.clone()) {
                    match units::scaffold(&root, &f.unit, &f.subsystem) {
                        Ok(rel) => println!("{}	{rel}", f.unit),
                        Err(e) => {
                            eprintln!("ledger: {e}");
                            std::process::exit(2);
                        }
                    }
                }
            }
            if let Err(e) = units::regenerate(&root) {
                eprintln!("ledger: {e}");
                std::process::exit(2);
            }
            return;
        }
        Some("queue") => {
            queue(&root, &funcs, &args[1..]);
            return;
        }
        _ => {}
    }
    let frame = match load_frame(&root.join("research/engine-map/frame.tsv")) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ledger: frame.tsv: {e}");
            std::process::exit(2);
        }
    };
    let md = render(&funcs, &header, unmapped.len(), cited.len())
        + &render_frame(&funcs, &frame)
        + &render_wiring(
            &fs::read_to_string(root.join("research/engine-map/frame_wiring.tsv"))
                .expect("frame wiring table"),
        );
    let out = root.join("docs/LEDGER.md");
    if check {
        let cur = fs::read_to_string(&out).unwrap_or_default();
        if cur.replace("\r\n", "\n") != md {
            eprintln!("ledger: docs/LEDGER.md is out of date; run the ledger");
            std::process::exit(1);
        }
        println!("ledger: docs/LEDGER.md is up to date");
        return;
    }
    fs::write(&out, &md).expect("write docs/LEDGER.md");
    if let Some(t) = tsv {
        let mut s = String::from("address\tsize\tname\tsubsystem\tunit\tstatus\trust\n");
        for f in &funcs {
            let locs: Vec<&String> = f
                .translated
                .iter()
                .chain(&f.table)
                .chain(&f.replaced)
                .chain(&f.traced)
                .chain(&f.incomplete)
                .collect();
            let _ = writeln!(
                s,
                "{:08x}\t{}\t{}\t{}\t{}\t{}\t{}",
                f.addr,
                f.size,
                f.name,
                f.subsystem,
                f.unit,
                f.status(),
                locs.iter()
                    .map(|l| l.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
        fs::write(&t, s).expect("write tsv");
    }
    println!(
        "ledger: wrote docs/LEDGER.md ({} functions, {} cited addresses)",
        funcs.len(),
        cited.len()
    );
}

/// `queue`: functions the engine crates do not have yet, per unit
/// (unit, subsystem, functions, bytes, file, crate), largest first. `queue <unit>`: that unit's functions (address, size, status,
/// name), the work list for one translator.
fn queue(root: &Path, funcs: &[Func], args: &[String]) {
    if let Some(unit) = args.first() {
        // Optional address range [lo, hi): one part of a unit split between
        // several translators.
        let hex = |i: usize, d: u32| {
            args.get(i)
                .and_then(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16).ok())
                .unwrap_or(d)
        };
        let (lo, hi) = (hex(1, 0), hex(2, u32::MAX));
        for f in funcs
            .iter()
            .filter(|f| &f.unit == unit && f.addr >= lo && f.addr < hi)
        {
            // Compiler and runtime code placed in a unit (static
            // initializers) is not translation work even when cited.
            let st = f.queue_status();
            println!("{:08x}\t{}\t{}\t{}", f.addr, f.size, st, f.name);
        }
        return;
    }
    let mut per: BTreeMap<(&str, &str), (usize, u64)> = BTreeMap::new();
    for f in funcs
        .iter()
        .filter(|f| !f.in_engine() && !f.unit.is_empty() && f.base() == "open")
    {
        let e = per.entry((&f.subsystem, &f.unit)).or_default();
        e.0 += 1;
        e.1 += f.size as u64;
    }
    let mut rows: Vec<_> = per.into_iter().collect();
    rows.sort_by(|a, b| b.1 .1.cmp(&a.1 .1).then(a.0.cmp(&b.0)));
    for ((sub, unit), (n, bytes)) in rows {
        let (file, krate) = units::unit_path(root, unit, sub);
        println!("{unit}\t{sub}\t{n}\t{bytes}\t{file}\t{krate}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_rows_have_their_own_status_and_still_count_as_engine_coverage() {
        let mut f = func(0x00f6_2250, "compiler: static init/exit");
        assert_eq!(f.status(), "library");
        f.table.insert(format!("{INITIALIZER_TABLE}:2"));
        assert_eq!(f.status(), TABLE);
        assert!(f.in_engine());
        assert_eq!(f.queue_status(), "done");
        // A hand translation of the same entry wins.
        f.translated.insert("crates/engine/src/x.rs:1".into());
        assert_eq!(f.status(), "translated");
    }

    #[test]
    fn initializer_table_rows_cite_their_entries() {
        let text = "# generated\n\
                    00f62250\tobject\t0040e0b0 011ce7c0 00fcee30 w:01054438 f:s.0101712c\n\
                    \n\
                    xyz\tconst\n\
                    00f352d0\treg\t00a26e20 011c32f8\n";
        assert_eq!(table_rows(text), [(0x00f6_2250, 2), (0x00f3_52d0, 5)]);
    }

    #[test]
    fn exit_table_rows_cite_their_functions_and_are_a_table_too() {
        assert_eq!(TABLES, [INITIALIZER_TABLE, EXIT_TABLE]);
        let text = "# generated\n\
                    00fc7a70\tcall\t011c3264 00404a00\n\
                    00fde581\ttail\t0126fc95 00ec3b35\n";
        assert_eq!(table_rows(text), [(0x00fc_7a70, 2), (0x00fd_e581, 3)]);
        let mut f = func(0x00fc_7a70, "compiler: static init/exit");
        f.table.insert(format!("{EXIT_TABLE}:2"));
        assert_eq!(f.status(), TABLE);
        assert!(f.in_engine());
        assert_eq!(f.queue_status(), "done");
    }

    #[test]
    fn finds_addresses() {
        assert_eq!(
            addresses("`bhkWorld::Update` (`00c6ae70`)"),
            vec![0x00c6ae70]
        );
        assert_eq!(addresses("FUN_00c6ae70 0x00c6ae70"), vec![0x00c6ae70]);
        assert_eq!(addresses("a00c6ae70 00c6ae701"), Vec::<u32>::new());
        // Data (.rdata) and values outside .text are not functions.
        assert_eq!(addresses("01017d00 00001234"), Vec::<u32>::new());
    }

    #[test]
    fn marker_lines_translate() {
        let text = "/// `x` (`00c90e60`) traced\n\
                    // Translated from 00c95a80 and 00c90e60 (decompiled, FalloutNV.exe 1.4.0.525)\n\
                    /// the finder (`00afe220`,\n\
                    /// decompiled, FalloutNV.exe 1.4.0.525): wraps\n";
        let got = scan(text);
        assert!(got.contains(&(0x00c90e60, Cite::Traced, 1)));
        assert!(got.contains(&(0x00c95a80, Cite::Translated, 2)));
        assert!(got.contains(&(0x00c90e60, Cite::Translated, 2)));
        assert!(got
            .iter()
            .any(|&(a, t, _)| a == 0x00afe220 && t == Cite::Translated));
    }

    #[test]
    fn every_engine_crate_counts_as_the_engine() {
        assert!(in_engine_crate("crates/engine/src/lib.rs:1"));
        assert!(in_engine_crate(
            "crates/engine-havok/src/units/havok_sdk/hkaabb.rs:7"
        ));
        assert!(in_engine_crate(
            "crates/engine-core/data/atexit_destructors.tsv:2"
        ));
        assert!(!in_engine_crate("crates/world/src/ai.rs:1"));
        assert!(!in_engine_crate("crates/enginex/src/ai.rs:1"));
    }

    #[test]
    fn explicit_engine_gaps_stay_in_the_queue_until_closed() {
        let mut f = func(0x008c94e0, "fallout/ai");
        f.translated.insert("crates/engine/src/ai.rs:1".into());
        assert_eq!(f.queue_status(), "done");
        let markers = scan("// Translation incomplete: 008c94e0 - actor scheduling\n");
        assert_eq!(markers, vec![(0x008c94e0, Cite::Incomplete, 1)]);
        // A gap in a different adapter does not reopen a complete engine body.
        f.incomplete.insert("crates/world/src/ai.rs:1".into());
        assert_eq!(f.queue_status(), "done");
        f.incomplete.insert("crates/engine/src/ai.rs:2".into());
        assert!(!f.in_engine());
        assert_eq!(f.queue_status(), "partial");
        // The engine marker still counts in the totals even with a gap in the
        // same crate: `partial` is a task-queue state, the ledger counts
        // engine markers, not complete behavior. (The marker above is in
        // `crates/engine`; one only in `crates/world` would be `traced`.)
        assert_eq!(
            f.status(),
            "translated",
            "totals still count engine markers"
        );
        let report = render(&[f], &[], 0, 1);
        assert!(report.contains("**1** engine functions"));
        assert!(report.contains("| `008c94e0` |"));

        let mut f = func(0x008c94e0, "fallout/ai");
        f.incomplete.insert("crates/engine/src/ai.rs:2".into());
        assert_eq!(f.queue_status(), "partial", "a gap needs no fake body");
        f.incomplete.clear();
        assert_eq!(f.queue_status(), "open");
    }

    #[test]
    fn only_engine_markers_count_as_translated() {
        // A marker only in an older viewer crate describes the function but
        // the engine does not have it: traced, and still not done in the queue.
        let mut f = func(0x008c_94e0, "fallout/ai");
        f.translated.insert("crates/world/src/ai.rs:1".into());
        assert_eq!(f.status(), "traced");
        assert!(!f.in_engine());
        assert_eq!(f.queue_status(), "traced");
        // An engine marker makes it translated, whatever else cites it.
        f.translated
            .insert("crates/engine-fallout/src/ai.rs:1".into());
        assert_eq!(f.status(), "translated");
        assert_eq!(f.queue_status(), "done");
        // Old-crate marker plus an old-crate trace: still traced.
        let mut g = func(0x008c_9500, "fallout/ai");
        g.traced.insert("viewer/src/x.rs:1".into());
        g.translated.insert("crates/physics/src/y.rs:1".into());
        assert_eq!(g.status(), "traced");
        // The generated table keeps its own status.
        g.table.insert(format!("{INITIALIZER_TABLE}:2"));
        assert_eq!(g.status(), TABLE);
    }

    fn func(addr: u32, subsystem: &str) -> Func {
        Func {
            addr,
            size: 16,
            name: String::new(),
            tier: String::new(),
            unit: String::new(),
            subsystem: subsystem.into(),
            placed: String::new(),
            translated: BTreeSet::new(),
            table: BTreeSet::new(),
            replaced: BTreeSet::new(),
            traced: BTreeSet::new(),
            incomplete: BTreeSet::new(),
        }
    }

    #[test]
    fn frame_coverage_per_depth() {
        let mut funcs = vec![
            func(0x0086e650, "fallout/misc"),
            func(0x00a00000, "fallout/ai"),
            func(0x00b00000, "LIBCMT"),
            func(0x00c00000, "BSSystem"),
        ];
        funcs[1]
            .translated
            .insert("crates/engine/src/x.rs:1".into());
        let dir = std::env::temp_dir().join(format!("ledger-frame-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("frame.tsv");
        fs::write(
            &path,
            "# provenance\n\
             seq\tdepth\tcall\taddress\tname\tname_tier\txbox\tunit\tsubsystem\tnote\n\
             1\t0\t0\t0086e650\tMain::OnIdle\tframe\t8269f3c8\t\t\t\n\
             2\t1\t1\t00a00000\tA::B\tcg\t82000000\t\t\t\n\
             3\t2\t1\t00b00000\t\t\t\t\t\t\n\
             4\t1\t2\t00a00000\tA::B\tcg\t82000000\t\t\t\n\
             5\t1\t3\t00c00000\t\t\t\t\t\t\n\
             6\t1\t4\t00d00000\t\t\t\t\t\t\n",
        )
        .unwrap();
        let rows = load_frame(&path).unwrap();
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(rows.len(), 6);
        let md = render_frame(&funcs, &rows);
        // Depth 1: four call sites, three functions; the repeat counts twice;
        // 00d00000 is no function's entry and counts as open.
        assert!(md.contains("| 0 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1 |"), "{md}");
        assert!(md.contains("| 1 | 4 | 3 | 2 | 0 | 0 | 1 | 0 | 1 |"), "{md}");
        assert!(md.contains("| 2 | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 0 |"), "{md}");
        assert!(
            md.contains("| 2 | `00a00000` | `A::B` | translated |"),
            "{md}"
        );
        assert!(md.contains("| 4 | `00d00000` | - | open |"), "{md}");
        assert!(render_frame(&funcs, &[]).is_empty());
        assert!(load_frame(&dir.join("missing.tsv")).unwrap().is_empty());
    }

    #[test]
    fn statuses() {
        assert_eq!(base_status("LIBCMT"), "library");
        assert_eq!(base_status("BSSystem"), "platform");
        assert_eq!(base_status("d3dx9"), "platform");
        assert_eq!(base_status("Havok SDK"), "open");
        assert_eq!(base_status("SpeedTree"), "open");
        // ADR-0007: the renderer is game code.
        assert_eq!(base_status("NiXenonRenderer"), "open");
        assert_eq!(
            unit_status("BSShader", "BSShader/lighting30shader.cpp"),
            "open"
        );
        assert_eq!(unit_status("BSSystem", "BSSystem/archive.cpp"), "open");
        assert_eq!(
            unit_status("BSSystem", "BSSystem/bsfile_xenon.cpp"),
            "platform"
        );
        // The D3DX import thunk inside a renderer unit stays platform.
        let mut f = func(0x00ee_6e2e, "NiXenonRenderer");
        f.unit = "NiXenonRenderer/nid3dshaderprogramcreatorhlsl.cpp".into();
        assert_eq!(f.status(), "platform");
        assert_eq!(f.queue_status(), "platform");
        f.addr = 0x00ee_6e30;
        assert_eq!(f.status(), "open");
    }

    #[test]
    fn overrides_place_unitless_functions_and_move_placed_ones_only_when_named() {
        let mut funcs = vec![
            func(0x00e7_5000, "NiXenonRenderer"),
            func(0x00e7_5010, "NiXenonRenderer"),
            func(0x00e7_5020, "NiXenonRenderer"),
        ];
        funcs[2].unit = "NiXenonRenderer/nid3dpass.cpp".into();
        let text = "# comment\naddress\tunit\tsubsystem\treplaces\tevidence\n\
                    00e75010\tNiSystem/nifile.cpp\tNiSystem\t\tslot 4 of NiFile\n";
        assert_eq!(apply_overrides(&mut funcs, text), Ok(1));
        assert_eq!(funcs[1].unit, "NiSystem/nifile.cpp");
        assert_eq!(funcs[1].subsystem, "NiSystem");
        assert_eq!(funcs[1].placed, "vtable");
        // A placed function moves only when the row names its unit.
        assert!(apply_overrides(&mut funcs, "00e75020\tX/y.cpp\tX\t\te\n").is_err());
        assert!(apply_overrides(&mut funcs, "00e75020\tX/y.cpp\tX\tX/z.cpp\te\n").is_err());
        assert!(apply_overrides(&mut funcs, "00e75004\tX/y.cpp\tX\t\te\n").is_err());
        let fix = "00e75020\tX/y.cpp\tX\tNiXenonRenderer/nid3dpass.cpp\te\n";
        assert_eq!(apply_overrides(&mut funcs, fix), Ok(1));
        assert_eq!(funcs[2].unit, "X/y.cpp");
        run_units(&mut funcs);
        assert_eq!(funcs[0].unit, "NiXenonRenderer/run_00e75000");
    }

    #[test]
    fn gaps_join_the_map_as_open_functions_of_their_units() {
        let mut funcs = vec![
            func(0x00a8_9c70, "NiMain"),
            func(0x00a8_a620, "NiMain"),
            func(0x00db_83c0, "Havok SDK"),
        ];
        funcs[2].unit = "Havok SDK/hkversionpatchmanager.cpp".into();
        let text = "# comment\naddress\tsize\tunit\tsubsystem\tevidence\n\
                    00a89cc0\t1343\t\tNiMain\tpush 00a8a609\n\
                    00db8540\t157\tHavok SDK/hkversionpatchmanager.cpp\tHavok SDK\tjmp 00dba1fc\n\
                    00fbd610\t23\t\t\trdata 01010648\n";
        assert_eq!(add_gaps(&mut funcs, text), Ok(3));
        let addrs: Vec<u32> = funcs.iter().map(|f| f.addr).collect();
        assert_eq!(
            addrs,
            [
                0x00a8_9c70,
                0x00a8_9cc0,
                0x00a8_a620,
                0x00db_83c0,
                0x00db_8540,
                0x00fb_d610
            ]
        );
        assert_eq!(funcs[1].size, 1343);
        assert_eq!(funcs[1].placed, "gap");
        assert_eq!(funcs[4].unit, "Havok SDK/hkversionpatchmanager.cpp");
        assert_eq!(funcs[4].status(), "open");
        assert_eq!(funcs[5].subsystem, "(unplaced)");
        run_units(&mut funcs);
        assert_eq!(funcs[1].unit, "NiMain/run_00a89c70");
        assert_eq!(funcs[2].unit, "NiMain/run_00a89c70");
        // A row inside a function of the map, or repeated, is an error.
        assert!(add_gaps(&mut funcs, "00db83c4\t4\t\tHavok SDK\te\n").is_err());
        assert!(add_gaps(&mut funcs, "00a89cc0\t4\t\tNiMain\te\n").is_err());
        assert!(add_gaps(&mut funcs, "00a89cc0\n").is_err());
    }

    #[test]
    fn gaps_leave_the_runs_of_the_map_unchanged() {
        let mut funcs = vec![
            func(0x00a2_bd00, "NiMain"),
            func(0x00a2_bdb0, "NiAnimation"),
            func(0x00a2_bde0, "NiAnimation"),
            func(0x00a2_be00, "NiAnimation"),
            func(0x00a2_be60, "NiAnimation"),
            func(0x00a2_bf00, "Havok SDK"),
            func(0x00a2_bf20, "Havok SDK"),
        ];
        funcs[0].unit = "NiMain/nimain.cpp".into();
        for i in [1, 3, 6] {
            funcs[i].placed = "gap".into();
        }
        funcs[5].unit = "Havok SDK/hkbase.cpp".into();
        run_units(&mut funcs);
        let units: Vec<&str> = funcs.iter().map(|f| f.unit.as_str()).collect();
        assert_eq!(
            units,
            [
                "NiMain/nimain.cpp",
                // No run before it: the run after it.
                "NiAnimation/run_00a2bde0",
                "NiAnimation/run_00a2bde0",
                "NiAnimation/run_00a2bde0",
                "NiAnimation/run_00a2bde0",
                "Havok SDK/hkbase.cpp",
                // No run on either side: its own.
                "Havok SDK/run_00a2bf20",
            ]
        );
    }

    #[test]
    fn runs_form_in_platform_subsystems_and_keep_their_status() {
        let mut funcs = vec![
            func(0x00b0_1080, "BSSystem"),
            func(0x00b0_10d0, "BSSystem"),
            func(0x00b0_1100, "LIBCMT"),
            func(0x00b0_1130, "BSSystem"),
        ];
        run_units(&mut funcs);
        assert_eq!(funcs[0].unit, "BSSystem/run_00b01080");
        assert_eq!(funcs[1].unit, "BSSystem/run_00b01080");
        assert_eq!(funcs[2].unit, "", "library code gets no run");
        assert_eq!(funcs[3].unit, "BSSystem/run_00b01130");
        assert_eq!(funcs[0].status(), "platform");
        // Game code in a platform run is open.
        funcs[1].addr = 0x00af_d4e0;
        assert_eq!(funcs[1].status(), "open");
    }
}
