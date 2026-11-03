//! Drivers the viewer calls once per frame.
//!
//! Host: [`HostBridge::collect`] after the frame's scripts and rules ran
//! (end of `ViewerSet::AfterFrame`). Guest: [`GuestBridge::sync`] at the
//! start of the frame (`ViewerSet::Input`), after the network pump.

use std::collections::BTreeMap;

use coop_core::delta::StateStore;
use coop_core::ids::Rev;
use coop_core::sync::HostSync;

use super::apply::{ApplyReport, Opts};
use super::backend::WorldBackend;
use super::export::{domain_of, export, is_tombstone, tombstone_of, wf};
use super::{Domain, Effect};
use crate::scripting::GameState;

/// What one collect did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CollectReport {
    /// A full keyed diff ran (otherwise the call was skipped as idle).
    pub ran: bool,
    /// Store writes made.
    pub written: u32,
    /// Slots that went back to default (tombstones written).
    pub cleared: u32,
    /// Writes whose (domain, subject) had no matching effect.
    pub uncovered: BTreeMap<Domain, u32>,
}

/// Host side: turns `GameState` changes into store writes.
pub struct HostBridge {
    /// Run the full keyed diff at least every this many calls even with no
    /// effects (the backstop for writers that emit nothing).
    pub backstop_every: u32,
    calls: u32,
    /// Total uncovered writes by domain since creation (a CI signal).
    pub uncovered_total: BTreeMap<Domain, u32>,
}

impl Default for HostBridge {
    fn default() -> Self {
        HostBridge {
            backstop_every: 20,
            calls: 0,
            uncovered_total: BTreeMap::new(),
        }
    }
}

impl HostBridge {
    /// Drains effects and mirrors the shared domains into `host.store`.
    pub fn collect(&mut self, gs: &mut GameState, host: &mut HostSync) -> CollectReport {
        let effects = std::mem::take(&mut gs.coop.out);
        let overflow = std::mem::take(&mut gs.coop.overflow);
        self.calls += 1;
        let due = self.backstop_every != 0 && self.calls % self.backstop_every == 0;
        let mut rep = CollectReport::default();
        if effects.is_empty() && !overflow && !due {
            return rep;
        }
        rep.ran = true;
        let want = export(gs);
        let mut touched = Vec::new();
        for (k, f) in &want {
            if host.write(k.form, f.clone()).is_some() {
                touched.push(*k);
                rep.written += 1;
            }
        }
        let stale: Vec<_> = host
            .store
            .iter()
            .filter(|(k, e)| !want.contains_key(k) && !is_tombstone(&e.field))
            .map(|(k, e)| (*k, tombstone_of(&e.field)))
            .collect();
        for (k, t) in stale {
            if host.write(k.form, t).is_some() {
                touched.push(k);
                rep.cleared += 1;
            }
        }
        if !overflow {
            let seen: std::collections::HashSet<Effect> = effects.into_iter().collect();
            for k in touched {
                let d = domain_of(&k);
                if !seen.contains(&Effect {
                    domain: d,
                    form: wf(k.form),
                }) {
                    *rep.uncovered.entry(d).or_default() += 1;
                    *self.uncovered_total.entry(d).or_default() += 1;
                }
            }
        }
        rep
    }
}

/// Guest side: makes `GameState` follow the replicated store.
#[derive(Default)]
pub struct GuestBridge {
    last: Option<(Rev, Rev)>,
    /// Applies performed.
    pub applies: u32,
    /// Shared slots that differed from the store when no store change was
    /// pending (local drift, repaired).
    pub drift_repairs: u32,
}

impl GuestBridge {
    /// Applies the store to `gs` when it changed since the last call. Pass
    /// `recheck` to also verify an unchanged store against the state (drift
    /// detection; repairs by re-applying).
    pub fn sync(
        &mut self,
        gs: &mut GameState,
        store: &StateStore,
        recheck: bool,
        opts: Opts,
    ) -> Option<ApplyReport> {
        let stamp = (store.rev(), store.base());
        if self.last == Some(stamp) && !recheck {
            return None;
        }
        // The first non-empty store is the join snapshot: no announcements.
        let first = self.last.map_or(true, |(r, _)| r == Rev::ZERO);
        let rep = gs.apply_store(
            store,
            if first {
                Opts {
                    present: false,
                    ..opts
                }
            } else {
                opts
            },
        );
        if self.last == Some(stamp) && rep.set + rep.cleared > 0 {
            self.drift_repairs += 1;
        }
        self.last = Some(stamp);
        self.applies += 1;
        Some(rep)
    }

    /// The store revision last applied to the game state, when the store
    /// then held every revision up to it (no gaps).
    pub fn applied_rev(&self) -> Option<Rev> {
        self.last.filter(|(r, b)| r == b).map(|(r, _)| r)
    }

    /// Forget what was applied (after a reload of the guest state).
    pub fn reset(&mut self) {
        self.last = None;
    }
}
