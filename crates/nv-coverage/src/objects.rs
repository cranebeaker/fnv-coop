//! Object script walk: runs each distinct script attached to a placed
//! reference once, with every event block it has, against the stub world,
//! and records panics, stack overflows and functions the host does not carry out.
//!
//! A stack overflow cannot be caught in-process, so the walk runs in a worker
//! process (`nv-coverage object-worker`) that prints one line per script; when
//! the worker dies, the parent notes the script it was on as an overflow and
//! starts a new worker after it.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::io::Write;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::process::{Command, Stdio};

use esm::{FourCC, LoadOrder};
use world::scripting::{script_of, GameState, Runner, ScriptCache};

#[derive(Debug, Default)]
pub struct ObjectReport {
    pub references_with_script: usize,
    pub distinct_scripts: usize,
    pub ran_clean: usize,
    pub panicked: usize,
    pub panics: Vec<String>,
    /// Functions the host does not carry out (not in the handled lists) → scripts that hit them.
    pub unhandled: BTreeMap<String, usize>,
    /// Handled functions that could not answer in the stub world → scripts.
    pub stub_limited: BTreeMap<String, usize>,
    pub scripts_with_unhandled: usize,
    /// Scripts that killed the worker (stack overflow or abort): (editor id, reference).
    pub overflows: Vec<(String, String)>,
}

fn names(map: impl Iterator<Item = String>) -> String {
    map.collect::<Vec<_>>().join(",")
}

/// The worker: prints one `B` line before each script and one `E` or `P` line after it.
pub fn worker(order: &LoadOrder, handled: &HashSet<String>, from: usize, out: &mut impl Write) {
    let cache = ScriptCache::default();
    let mut state = GameState::new(order);
    let mut seen = BTreeSet::new();
    let mut refs = 0usize;
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for kind in [b"REFR", b"ACHR", b"ACRE"] {
        for rr in order.records_of_type(FourCC::new(kind)) {
            let Some(script_id) = script_of(order, rr.form_id) else {
                continue;
            };
            refs += 1;
            if !seen.insert(script_id) {
                continue;
            }
            let ordinal = seen.len();
            if ordinal <= from {
                continue;
            }
            let Some(parsed) = cache.script(order, script_id) else {
                continue;
            };
            let editor = order
                .get(script_id)
                .and_then(|r| r.editor_id().ok().flatten())
                .unwrap_or_default();
            let events: Vec<(&str, Option<esm::FormId>)> = parsed
                .blocks
                .iter()
                .map(|b| (b.kind.as_str(), Some(world::dialogue::PLAYER_REF)))
                .collect();
            // Debugging aid: `NV_OBJ_ONLY=<editor id>` runs only that script.
            if let Ok(only) = std::env::var("NV_OBJ_ONLY") {
                if !only.eq_ignore_ascii_case(&editor) {
                    continue;
                }
            }
            let _ = writeln!(out, "B\t{ordinal}\t{editor}\t{}", rr.form_id);
            let _ = out.flush();
            let before = state.unhandled.clone();
            let ran = catch_unwind(AssertUnwindSafe(|| {
                Runner::new(order, &cache, &mut state)
                    .run_reference_script(rr.form_id, &events, None);
            }));
            match ran {
                Err(p) => {
                    let msg = p
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                        .unwrap_or_default();
                    let _ = writeln!(out, "P\t{}", msg.replace(['\n', '\t'], " "));
                }
                Ok(()) => {
                    let (mut missing, mut stub) = (Vec::new(), Vec::new());
                    for (k, n) in &state.unhandled {
                        if before.get(k).copied().unwrap_or(0) < *n {
                            if handled.contains(&k.to_ascii_lowercase()) {
                                stub.push(k.to_string());
                            } else {
                                missing.push(k.to_string());
                            }
                        }
                    }
                    let _ = writeln!(
                        out,
                        "E\t{}\t{}",
                        names(missing.into_iter()),
                        names(stub.into_iter())
                    );
                }
            }
            let _ = out.flush();
        }
    }
    std::panic::set_hook(hook);
    let _ = writeln!(out, "T\t{refs}\t{}", seen.len());
}

/// The parent: runs workers until one finishes, folding their lines into a report.
pub fn walk(exe: &Path, dir: &Path) -> ObjectReport {
    let mut rep = ObjectReport::default();
    let mut from = 0usize;
    let mut attempts = 0;
    // A script the worker died in once; dying in it again makes it a crash (a first death can be a
    // machine running out of memory).
    let mut suspect: Option<usize> = None;
    let mut startup_failures = 0;
    loop {
        attempts += 1;
        let out = Command::new(exe)
            .arg("object-worker")
            .arg("--data")
            .arg(dir)
            .arg("--from")
            .arg(from.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output();
        let Ok(out) = out else { break };
        let text = String::from_utf8_lossy(&out.stdout).into_owned();
        let mut open: Option<(usize, String, String)> = None;
        let mut finished = false;
        for line in text.lines() {
            let f: Vec<&str> = line.split('\t').collect();
            match f.as_slice() {
                ["B", n, editor, r] => {
                    open = Some((n.parse().unwrap_or(0), editor.to_string(), r.to_string()))
                }
                ["E", missing, stub] => {
                    if let Some((n, _, _)) = open.take() {
                        from = from.max(n);
                    }
                    let split = |s: &str| {
                        s.split(',')
                            .filter(|x| !x.is_empty())
                            .map(str::to_string)
                            .collect::<Vec<_>>()
                    };
                    let (m, s) = (split(missing), split(stub));
                    for x in &m {
                        *rep.unhandled.entry(x.clone()).or_default() += 1;
                    }
                    for x in &s {
                        *rep.stub_limited.entry(x.clone()).or_default() += 1;
                    }
                    if m.is_empty() {
                        rep.ran_clean += 1;
                    } else {
                        rep.scripts_with_unhandled += 1;
                    }
                }
                ["P", msg] => {
                    if let Some((n, e, r)) = open.take() {
                        from = from.max(n);
                        rep.panicked += 1;
                        if rep.panics.len() < 10 {
                            rep.panics.push(format!("{e} on {r}: {msg}"));
                        }
                    }
                }
                ["T", refs, distinct] => {
                    rep.references_with_script = refs.parse().unwrap_or(0);
                    rep.distinct_scripts = distinct.parse().unwrap_or(0);
                    finished = true;
                }
                _ => {}
            }
        }
        if finished || attempts > 200 {
            break;
        }
        match open {
            // The worker died inside this script.
            Some((n, e, r)) => {
                if suspect == Some(n) {
                    rep.overflows.push((e, r));
                    from = from.max(n);
                    suspect = None;
                } else {
                    suspect = Some(n);
                }
            }
            None => {
                startup_failures += 1;
                if startup_failures > 3 {
                    break;
                }
            }
        }
    }
    rep
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_join() {
        assert_eq!(names(["a".to_string(), "b".to_string()].into_iter()), "a,b");
    }
}
