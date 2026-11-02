//! Opt-in: the executable's static initializers as `host::Host::start` runs
//! them (`engine::static_init`), checked against the executable.
//!
//! ```text
//! NV_EXE="C:\...\Fallout New Vegas\FalloutNV.exe" cargo test -p host --release \
//!     --test static_initializers -- --ignored --nocapture
//! ```

use engine::static_init::{self, Arg, Row, Source};
use std::path::PathBuf;

fn host() -> host::Host {
    let exe = PathBuf::from(std::env::var("NV_EXE").expect("NV_EXE: the installed FalloutNV.exe"));
    host::Host::start(&exe.parent().unwrap().join("Data")).unwrap_or_else(|e| panic!("{e}"))
}

/// The `Setting` (Xbox PDB) constructors, the vtable each leaves and its
/// collection's instance: game settings (`00404a70`), INI settings
/// (`0044f560`) and the third collection (`0045d180`).
const SETTINGS: [(u32, u32, u32); 7] = [
    (0x0040_e0b0, 0x0101_2114, 0x011c_8048),
    (0x0040_c150, 0x0101_2114, 0x011c_8048),
    (0x0044_f440, 0x0101_7720, 0x011f_96a0),
    (0x0045_cf30, 0x0101_7720, 0x011f_96a0),
    (0x0045_cfe0, 0x0101_7720, 0x011f_96a0),
    (0x004a_4a40, 0x0101_84fc, 0x011f_35a4),
    (0x0045_d060, 0x0101_84fc, 0x011f_35a4),
];

/// How many entries the run covered, and where it stopped.
#[test]
#[ignore = "needs the player's FalloutNV.exe (set NV_EXE)"]
fn the_run_and_what_it_skipped() {
    let host = host();
    let r = &host.static_init;
    let c = static_init::entries(&host.engine, static_init::C_TABLE).len();
    let cpp = static_init::entries(&host.engine, static_init::CPP_TABLE).len();
    println!(
        "entries: {c} C + {cpp} C++; run from the table: {}, by address: {}, \
         skipped: {} without a row or translation, {} with an untranslated callee, \
         {} stopped part-way",
        r.rows,
        r.translated,
        r.open.len(),
        r.callee_open.len(),
        r.stopped.len()
    );
    let mut callees = std::collections::BTreeMap::<u32, usize>::new();
    for (_, c) in &r.callee_open {
        *callees.entry(*c).or_default() += 1;
    }
    let mut by_count: Vec<_> = callees.into_iter().collect();
    by_count.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    println!(
        "untranslated callees (entries): {:08x?}",
        &by_count[..by_count.len().min(30)]
    );
    let mut stops = std::collections::BTreeMap::<String, usize>::new();
    for (_, s) in &r.stopped {
        *stops.entry(s.clone()).or_default() += 1;
    }
    let mut stops: Vec<_> = stops.into_iter().collect();
    stops.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    for (s, n) in stops.iter().take(20) {
        println!("  stopped {n}: {s}");
    }
    assert!(r.c_failed.is_none(), "{:x?}", r.c_failed);
    assert!(r.rows > 10_000, "{} rows run", r.rows);
}

/// Three game settings as their initializers leave them: the `Setting` at
/// its global (value +4, name +8), with the name and default each
/// initializer passes (read from Ghidra: `00f62250`, `00f691f0`,
/// `00f691c0`, `FLD float ptr [default]`).
#[test]
#[ignore = "needs the player's FalloutNV.exe (set NV_EXE)"]
fn sample_settings_hold_the_executables_defaults() {
    let host = host();
    for (initializer, setting, name, default) in [
        (
            0x00f6_2250u32,
            0x011c_e7c0u32,
            0x0105_4438u32,
            0x0101_712cu32,
        ), // fGunShellLifetime
        (0x00f6_91f0, 0x011d_0ad8, 0x0105_7dd4, 0x0102_bc30), // fPlayerMaxResistance
        (0x00f6_91c0, 0x011d_1020, 0x0105_7dc4, 0x0102_49d8), // fMaxArmorRating
    ] {
        let text = String::from_utf8_lossy(&host.engine.mem.cstr(name)).into_owned();
        assert_eq!(
            host.engine.mem.u32(setting + 8),
            name,
            "{text} ({initializer:08x}): name"
        );
        assert_eq!(
            host.engine.mem.u32(setting + 4),
            host.engine.mem.u32(default),
            "{text} ({initializer:08x}): default"
        );
        assert_eq!(host.engine.mem.u32(setting), 0x0101_2114, "{text}: vtable");
    }
}

/// About fifty settings across the three collections and every
/// constructor: name, value and vtable as the initializer's arguments
/// (read from the executable through the table's operands) give them.
#[test]
#[ignore = "needs the player's FalloutNV.exe (set NV_EXE)"]
fn settings_across_the_collections_hold_their_initializers_values() {
    let host = host();
    let e = &host.engine;
    let rows = static_init::table();
    let mut by_ctor: Vec<Vec<(u32, u32, Vec<Arg>)>> = vec![Vec::new(); SETTINGS.len()];
    let mut entries: Vec<_> = rows.iter().collect();
    entries.sort_by_key(|(a, _)| **a);
    for (entry, row) in entries {
        if let Row::Object {
            constructor,
            this,
            args,
            ..
        } = row
        {
            if let Some(k) = SETTINGS.iter().position(|(c, _, _)| c == constructor) {
                by_ctor[k].push((*entry, *this, args.clone()));
            }
        }
    }
    let mut checked = 0;
    for (k, list) in by_ctor.iter().enumerate() {
        let (ctor, vtable, collection) = SETTINGS[k];
        assert!(!list.is_empty(), "no settings by {ctor:08x}");
        assert_ne!(e.mem.u32(collection), 0, "collection of {ctor:08x}");
        let step = (list.len() / 8).max(1);
        for (entry, this, args) in list.iter().step_by(step).take(8) {
            let [Arg::Word(name), value] = args[..] else {
                panic!("{entry:08x}: arguments {args:?}");
            };
            let text = String::from_utf8_lossy(&e.mem.cstr(name)).into_owned();
            assert_eq!(e.mem.u32(this + 8), name, "{text} ({entry:08x}): name");
            assert_eq!(e.mem.u32(*this), vtable, "{text} ({entry:08x}): vtable");
            match value {
                Arg::Word(v) if ctor == 0x0045_cf30 => {
                    assert_eq!(e.mem.u8(this + 4), v as u8, "{text} ({entry:08x})")
                }
                Arg::Word(v) => assert_eq!(e.mem.u32(this + 4), v, "{text} ({entry:08x})"),
                Arg::Float(Source::F32(a)) => {
                    let bits = e.mem.u32(a);
                    // FLD/FSTP float: the same bits (no setting default is a NaN).
                    assert!(!f32::from_bits(bits).is_nan());
                    assert_eq!(e.mem.u32(this + 4), bits, "{text} ({entry:08x})")
                }
                Arg::Float(Source::One) => {
                    assert_eq!(e.mem.f32(this + 4), 1.0, "{text} ({entry:08x})")
                }
                Arg::Float(Source::Zero) => {
                    assert_eq!(e.mem.u32(this + 4), 0, "{text} ({entry:08x})")
                }
                other => panic!("{entry:08x}: value {other:?}"),
            }
            checked += 1;
        }
    }
    println!("settings checked: {checked}");
    assert!(checked >= 45, "{checked}");
}

/// The table entries a start-up would skip, listed without running anything
/// (`engine::static_init::skips` on the mapped executable, the host's stand-ins
/// counted as run): one line each with the entry, why, the global its row
/// builds (when it has a row) and its engine-map name
/// (`research/engine-map/engine_map.tsv`), e.g. the `Setting` it would make.
/// A coverage check for initializer work: no world, no `main`.
///
/// ```text
/// NV_EXE="C:\...\Fallout New Vegas\FalloutNV.exe" cargo test -p host --release \
///     --test static_initializers -- --ignored --nocapture the_skipped_entries
/// ```
#[test]
#[ignore = "needs the player's FalloutNV.exe (set NV_EXE)"]
fn the_skipped_entries_without_a_run() {
    let exe = PathBuf::from(std::env::var("NV_EXE").expect("NV_EXE: the installed FalloutNV.exe"));
    let engine = engine::Engine::with_exe(&exe).unwrap_or_else(|e| panic!("{e}"));
    let rows = static_init::table();
    let map = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../research/engine-map/engine_map.tsv"),
    )
    .unwrap_or_default();
    let names: std::collections::HashMap<u32, &str> = map
        .lines()
        .filter_map(|l| {
            let mut f = l.split('\t');
            let addr = u32::from_str_radix(f.next()?, 16).ok()?;
            Some((addr, f.nth(1)?))
        })
        .collect();
    let skips = static_init::skips(&engine, &rows, &host::STAND_INS);
    for skip in &skips {
        let (entry, why) = match *skip {
            static_init::Skip::Open(entry) => (entry, "open".to_string()),
            static_init::Skip::Callee(entry, c) => (entry, format!("callee {c:08x}")),
        };
        let global = rows
            .get(&entry)
            .and_then(static_init::row_global)
            .map_or(String::new(), |g| format!("{g:08x}"));
        let name = names.get(&entry).copied().unwrap_or("");
        println!("{entry:08x}\t{why}\t{global}\t{name}");
    }
    println!("skipped without a run: {}", skips.len());
}
