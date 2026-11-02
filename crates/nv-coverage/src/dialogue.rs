//! Dialogue coverage: every INFO's conditions and result scripts.

use std::collections::{BTreeMap, HashSet};
use std::panic::{catch_unwind, AssertUnwindSafe};

use esm::{FourCC, LoadOrder};
use world::scripting::{GameState, Runner, ScriptCache};

use crate::scan::{calls_of, Kind, Source};

#[derive(Debug, Default)]
pub struct DialogueReport {
    pub infos: usize,
    pub infos_with_conditions: usize,
    pub conditions: usize,
    pub infos_unsupported_condition: usize,
    /// Condition function → INFOs that use it (unsupported only).
    pub unsupported_conditions: BTreeMap<String, usize>,
    pub results: usize,
    pub results_parse_failed: usize,
    pub results_missing_function: usize,
    /// Result function → result scripts that use it (unsupported only).
    pub unsupported_results: BTreeMap<String, usize>,
    pub results_panicked: usize,
    pub results_ran_clean: usize,
    pub panics: Vec<String>,
}

impl DialogueReport {
    pub fn condition_percent(&self) -> f64 {
        if self.infos_with_conditions == 0 {
            return 100.0;
        }
        100.0 * (1.0 - self.infos_unsupported_condition as f64 / self.infos_with_conditions as f64)
    }
    pub fn result_percent(&self) -> f64 {
        if self.results == 0 {
            return 100.0;
        }
        100.0 * self.results_ran_clean as f64 / self.results as f64
    }
}

pub fn analyse(order: &LoadOrder, sources: &[Source], handled: &HashSet<String>) -> DialogueReport {
    let mut rep = DialogueReport::default();
    let ctda = FourCC::new(b"CTDA");
    for rr in order.records_of_type(FourCC::new(b"INFO")) {
        let Ok(r) = rr.record() else { continue };
        rep.infos += 1;
        let mut names = std::collections::BTreeSet::new();
        let mut any = false;
        for c in r.get_all(ctda) {
            any = true;
            rep.conditions += 1;
            if let Some(f) = c.data.get(8..10) {
                let idx = u16::from_le_bytes([f[0], f[1]]);
                let name = script::function_name(idx);
                if !handled.contains(&name.to_ascii_lowercase()) {
                    names.insert(name);
                }
            }
        }
        if any {
            rep.infos_with_conditions += 1;
        }
        if !names.is_empty() {
            rep.infos_unsupported_condition += 1;
            for n in names {
                *rep.unsupported_conditions.entry(n).or_default() += 1;
            }
        }
    }
    let scripts = ScriptCache::default();
    let mut state = GameState::new(order);
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for s in sources.iter().filter(|s| s.kind == Kind::DialogueResult) {
        rep.results += 1;
        let Ok(parsed) = &s.parsed else {
            rep.results_parse_failed += 1;
            continue;
        };
        let mut missing = std::collections::BTreeSet::new();
        for f in calls_of(parsed) {
            let name = script::function_name(f);
            if !handled.contains(&name.to_ascii_lowercase()) {
                missing.insert(name);
            }
        }
        if !missing.is_empty() {
            rep.results_missing_function += 1;
            for m in missing {
                *rep.unsupported_results.entry(m).or_default() += 1;
            }
            continue;
        }
        // Statically complete: run it against the stub world for panics.
        let text = s.text.clone();
        let ran = catch_unwind(AssertUnwindSafe(|| {
            let mut runner = Runner::new(order, &scripts, &mut state);
            runner.run_source(&text, None, s.quest);
        }));
        match ran {
            Ok(()) => rep.results_ran_clean += 1,
            Err(_) => {
                rep.results_panicked += 1;
                if rep.panics.len() < 10 {
                    rep.panics.push(format!("{} {}", s.editor, s.form));
                }
            }
        }
    }
    std::panic::set_hook(hook);
    rep
}
