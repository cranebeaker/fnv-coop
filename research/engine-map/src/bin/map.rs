//! Engine map: every PC function with its Xbox name (where matched), its
//! source unit and subsystem (policy: ADR-0002).
//!
//! ```text
//! map <pc-dir> <xb-dir> <match-dir> <out.tsv>
//! ```
//!
//! Reads `NvEngineMap.java`'s `functions.tsv`, the `xbox` binary's
//! `xb_funcs.tsv`/`xb_modules.tsv` and the `match` binary's `matches.tsv`.
//! Writes one row per PC function, sorted by address:
//!
//! `address, size, name, name_tier, xbox, unit, subsystem, placed`
//!
//! - `name`: the Xbox PDB name (`name_tier` `vt` or `cg`, see `match.rs`),
//!   or a Function ID name for an unnamed function placed in a C/C++ runtime
//!   library (`fid`), or empty. Function ID names elsewhere are not used:
//!   most of them are small game functions that look like library code.
//! - `unit`: the Xbox object file's main source file (lower case, relative
//!   to the code root; for libraries `<subsystem>/<file>`); `subsystem` its
//!   folder or library (see `unit_of`).
//! - `placed`: how the unit was found.
//!   - `name`: the matched Xbox function's module.
//!   - `range`: unnamed, and the nearest named functions before and after it
//!     (by address) come from one unit. MSVC lays out each object file's
//!     code contiguously, so a function between two functions of one object
//!     belongs to it.
//!   - `calls`: unnamed, every placed function that calls it is in one unit,
//!     and that unit is the unit of the nearest named function before or
//!     after it (a static helper at the edge of its object).
//!   - `range-sub`: the named neighbours are different units of one
//!     subsystem; only the subsystem is set.
//!   - `init`, `atexit`: the compiler-generated tail of `.text` (see below).
//!   - `none`: not placed.
//!
//!   `report.txt` holds hold-out checks for `range` and `calls`: hide every
//!   other named function, place it the same way, compare.
//!
//! The tail of `.text`: after the last function that has a caller or sits
//! in a vtable (`vt` tier), every
//! function is compiler-generated (some carry Xbox names such as
//! `` `dynamic initializer for ...` ``; they keep the name and the unit of
//! the source file whose globals they set up). Functions referenced
//! only from data (the C++ initializer table) or not at all are dynamic
//! initializers of globals (`init`); functions whose address is pushed by
//! code (handed to `atexit`) are the matching destructors (`atexit`). The
//! initializers hold the default values of many globals, game settings
//! included, so they are data sources even though they are not engine
//! logic.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Write;
use std::path::Path;

type R<T> = Result<T, Box<dyn std::error::Error>>;

fn rows(path: &Path) -> R<Vec<Vec<String>>> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(text
        .lines()
        .skip(1)
        .map(|l| l.split('\t').map(str::to_string).collect())
        .collect())
}

fn hex(s: &str) -> u32 {
    u32::from_str_radix(s, 16).unwrap_or_else(|_| panic!("bad hex {s:?}"))
}

/// Unit and subsystem of an Xbox module.
fn unit_of(lib: &str, source: &str, obj: &str) -> (String, String) {
    let src = source.to_ascii_lowercase().replace('\\', "/");
    let obj_file = obj
        .rsplit(['\\', '/'])
        .next()
        .unwrap_or(obj)
        .to_ascii_lowercase();
    if lib == "exe" {
        let rel = src.split("/code/").nth(1).unwrap_or(&src).to_string();
        let parts: Vec<&str> = rel.split('/').collect();
        let sub = match parts.as_slice() {
            [top, dir, _, ..] => format!("{top}/{dir}"),
            [top, _] => top.to_string(),
            _ => "exe (no source)".to_string(),
        };
        return (if rel.is_empty() { obj_file } else { rel }, sub);
    }
    let file = match src.rsplit('/').next() {
        Some(f) if !f.is_empty() => f.to_string(),
        _ => obj_file,
    };
    let lib_stem = lib.split('.').next().unwrap_or(lib);
    let sub = if lib_stem == "BSHavok"
        && (src.contains("/havok/source/") || obj.contains("xbox360_net"))
    {
        "Havok SDK".to_string()
    } else {
        lib_stem.to_string()
    };
    (format!("{sub}/{file}"), sub)
}

struct Row {
    addr: u32,
    size: String,
    fid: String,
    calls: Vec<u32>,
    callers: usize,
    code_refs: usize,
    name: String,
    tier: String,
    xbox: String,
    unit: String,
    sub: String,
    placed: &'static str,
}

/// Unit (or subsystem only) from the named neighbours in `idx`.
fn by_range(idx: &[usize], i: usize, out: &[Row]) -> Option<(String, String, bool)> {
    let k = idx.partition_point(|&j| j < i);
    let (p, n) = (*idx.get(k.wrapping_sub(1))?, *idx.get(k)?);
    if out[p].unit == out[n].unit {
        Some((out[p].unit.clone(), out[p].sub.clone(), true))
    } else if out[p].sub == out[n].sub {
        Some((String::new(), out[p].sub.clone(), false))
    } else {
        None
    }
}

/// Unit from callers that agree with a named neighbour in `idx`.
fn by_calls(
    idx: &[usize],
    i: usize,
    out: &[Row],
    callers: &HashMap<u32, Vec<usize>>,
    known: &dyn Fn(usize) -> Option<String>,
) -> Option<String> {
    let mut unit: Option<String> = None;
    for &c in callers.get(&out[i].addr)? {
        let u = known(c)?;
        match &unit {
            Some(x) if *x != u => return None,
            _ => unit = Some(u),
        }
    }
    let u = unit?;
    let k = idx.partition_point(|&j| j < i);
    let near = [k.wrapping_sub(1), k]
        .iter()
        .filter_map(|&j| idx.get(j))
        .any(|&j| out[j].unit == u);
    near.then_some(u)
}

fn main() -> R<()> {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 5 {
        return Err("usage: map <pc-dir> <xb-dir> <match-dir> <out.tsv>".into());
    }
    let (pcd, xbd, md) = (Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]));

    let mut modules = Vec::new();
    for r in rows(&xbd.join("xb_modules.tsv"))? {
        modules.push(unit_of(
            &r[2],
            r.get(3).map(String::as_str).unwrap_or(""),
            &r[1],
        ));
    }
    let mut xb_mod: HashMap<u32, usize> = HashMap::new();
    for r in rows(&xbd.join("xb_funcs.tsv"))? {
        xb_mod.insert(hex(&r[0]), r[2].parse()?);
    }
    let mut matched: HashMap<u32, (u32, String, String)> = HashMap::new();
    for r in rows(&md.join("matches.tsv"))? {
        matched.insert(hex(&r[0]), (hex(&r[1]), r[2].clone(), r[3].clone()));
    }

    let mut out: Vec<Row> = Vec::new();
    for r in rows(&pcd.join("functions.tsv"))? {
        let addr = hex(&r[0]);
        let fid = if r[3] == "ANALYSIS" && !r[2].starts_with("FID_conflict") {
            r[2].clone()
        } else {
            String::new()
        };
        let calls = if r[6].is_empty() {
            Vec::new()
        } else {
            r[6].split(',').map(hex).collect()
        };
        let mut row = Row {
            addr,
            size: r[1].clone(),
            fid,
            calls,
            callers: r[9].parse()?,
            code_refs: r[10].parse()?,
            name: String::new(),
            tier: String::new(),
            xbox: String::new(),
            unit: String::new(),
            sub: String::new(),
            placed: "none",
        };
        if let Some((x, tier, name)) = matched.get(&addr) {
            let (u, s) = &modules[xb_mod[x]];
            row.name = name.clone();
            row.tier = tier.clone();
            row.xbox = format!("{x:08x}");
            row.unit = u.clone();
            row.sub = s.clone();
            row.placed = "name";
        }
        out.push(row);
    }

    // Compiler-generated tail.
    let tail = out
        .iter()
        .rposition(|r| r.callers > 0 || r.tier == "vt")
        .map(|i| i + 1)
        .unwrap_or(out.len());
    for r in &mut out[tail..] {
        let atexit = r.code_refs > 0;
        r.placed = if atexit { "atexit" } else { "init" };
        r.sub = "compiler: static init/exit".into();
        if r.unit.is_empty() {
            r.unit = if atexit {
                "compiler: atexit destructors"
            } else {
                "compiler: dynamic initializers"
            }
            .into();
        }
    }

    let index: HashMap<u32, usize> = out.iter().enumerate().map(|(i, r)| (r.addr, i)).collect();
    let mut callers: HashMap<u32, Vec<usize>> = HashMap::new();
    for (i, r) in out.iter().enumerate() {
        for c in &r.calls {
            if index.contains_key(c) {
                let v = callers.entry(*c).or_default();
                if v.last() != Some(&i) {
                    v.push(i);
                }
            }
        }
    }

    // Hold-out checks.
    let named: Vec<usize> = (0..tail).filter(|&i| out[i].placed == "name").collect();
    let even: Vec<usize> = named.iter().copied().filter(|i| i % 2 == 0).collect();
    let (mut r_unit, mut r_unit_ok, mut r_sub, mut r_sub_ok) = (0, 0, 0, 0);
    let (mut c_n, mut c_ok) = (0, 0);
    let even_known =
        |c: usize| (c.is_multiple_of(2) && out[c].placed == "name").then(|| out[c].unit.clone());
    for &i in named.iter().filter(|i| *i % 2 == 1) {
        match by_range(&even, i, &out) {
            Some((u, _, true)) => {
                r_unit += 1;
                r_unit_ok += (u == out[i].unit) as usize;
            }
            Some((_, s, false)) => {
                r_sub += 1;
                r_sub_ok += (s == out[i].sub) as usize;
                if let Some(u) = by_calls(&even, i, &out, &callers, &even_known) {
                    c_n += 1;
                    c_ok += (u == out[i].unit) as usize;
                }
            }
            None => {
                if let Some(u) = by_calls(&even, i, &out, &callers, &even_known) {
                    c_n += 1;
                    c_ok += (u == out[i].unit) as usize;
                }
            }
        }
    }

    // Placement: range first, then callers (which may use range placements).
    let mut range_units: Vec<Option<(String, String, bool)>> = vec![None; out.len()];
    for (i, slot) in range_units.iter_mut().enumerate().take(tail) {
        if out[i].placed == "none" {
            *slot = by_range(&named, i, &out);
        }
    }
    for (i, ru) in range_units.iter().enumerate() {
        if let Some((u, s, true)) = ru {
            out[i].unit = u.clone();
            out[i].sub = s.clone();
            out[i].placed = "range";
        }
    }
    let unit_known: Vec<Option<String>> = out
        .iter()
        .map(|r| matches!(r.placed, "name" | "range").then(|| r.unit.clone()))
        .collect();
    let known = |c: usize| unit_known[c].clone();
    for i in 0..tail {
        if out[i].placed != "none" {
            continue;
        }
        if let Some(u) = by_calls(&named, i, &out, &callers, &known) {
            let sub = out[named[named.partition_point(|&j| j < i).min(named.len() - 1)]]
                .sub
                .clone();
            let s = out
                .iter()
                .find(|r| r.placed == "name" && r.unit == u)
                .map(|r| r.sub.clone())
                .unwrap_or(sub);
            out[i].unit = u;
            out[i].sub = s;
            out[i].placed = "calls";
        } else if let Some((_, s, false)) = &range_units[i] {
            out[i].sub = s.clone();
            out[i].placed = "range-sub";
        }
    }
    // Function ID names only inside the C/C++ runtime libraries.
    for r in &mut out {
        if r.name.is_empty() && !r.fid.is_empty() && matches!(r.sub.as_str(), "LIBCMT" | "libcpmt")
        {
            r.name = r.fid.clone();
            r.tier = "fid".into();
        }
    }

    let mut f = fs::File::create(&a[4])?;
    let info = |path: &Path, key: &str| -> String {
        fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .find_map(|l| {
                l.strip_prefix(key)
                    .and_then(|v| v.strip_prefix('\t'))
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "?".into())
    };
    writeln!(
        f,
        "# FalloutNV.exe 1.4.0.525 (unpacked), SHA-256 {}",
        info(&pcd.join("manifest.txt"), "exe_sha256")
    )?;
    writeln!(
        f,
        "# Names and units: Xbox PDB {} (GUID {}, age {}), ADR-0002",
        info(&xbd.join("xb_info.txt"), "pdb"),
        info(&xbd.join("xb_info.txt"), "guid"),
        info(&xbd.join("xb_info.txt"), "age")
    )?;
    writeln!(f, "# Generated by research/engine-map (match, map) from NvEngineMap.java output; see its README")?;
    writeln!(
        f,
        "address\tsize\tname\tname_tier\txbox\tunit\tsubsystem\tplaced"
    )?;
    let mut placed: BTreeMap<&str, usize> = BTreeMap::new();
    for r in &out {
        *placed.entry(r.placed).or_default() += 1;
        writeln!(
            f,
            "{:08x}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            r.addr, r.size, r.name, r.tier, r.xbox, r.unit, r.sub, r.placed
        )?;
    }
    let report = format!(
        "functions {}\nplaced {placed:?}\ncompiler tail from {:08x} ({} functions)\n\
         hold-out (every other named function hidden): range unit {r_unit}, right {r_unit_ok}; \
         range subsystem only {r_sub}, right {r_sub_ok}; calls unit {c_n}, right {c_ok}\n",
        out.len(),
        out.get(tail).map(|r| r.addr).unwrap_or(0),
        out.len() - tail,
    );
    print!("{report}");
    fs::write(md.join("map-report.txt"), report)?;
    Ok(())
}
