//! Real script variable names on guests.
//!
//! Variables replicate by (owner, index). A guest that applies one without
//! game data in hand has to invent a name (`coopvarN`). [`heal`] replaces
//! such locals with the owner's real script variables, keeping the values by
//! index, as soon as game data is available (every `apply_changes` that has
//! `Opts.ctx`, and `GuestBridge` frames).

use script::interp::Locals;

use crate::coop::apply::Ctx;
use crate::scripting::GameState;

/// True when the locals carry invented names.
pub fn is_unnamed(l: &Locals) -> bool {
    l.iter().any(|(n, _, _)| n.starts_with("coopvar"))
}

/// Renames invented variables. Returns how many owners were fixed.
pub fn heal(cx: Ctx, gs: &mut GameState) -> usize {
    let owners: Vec<_> = gs
        .variables
        .iter()
        .filter(|(_, l)| is_unnamed(l))
        .map(|(o, _)| *o)
        .collect();
    let mut fixed = 0;
    for o in owners {
        let Some(base) = crate::scripting::script_of(cx.order, o)
            .and_then(|s| cx.scripts.script(cx.order, s))
            .map(|s| Locals::new(&s))
        else {
            continue;
        };
        let old: Vec<f64> = gs
            .variables
            .get(&o)
            .map(|l| l.iter().map(|(_, _, v)| v).collect())
            .unwrap_or_default();
        let names: Vec<String> = base.iter().map(|(n, _, _)| n.to_string()).collect();
        let mut fresh = base;
        for (i, n) in names.iter().enumerate() {
            if let Some(v) = old.get(i) {
                fresh.set(n, *v);
            }
        }
        gs.variables.insert(o, fresh);
        fixed += 1;
    }
    gs.coop.campaign.unnamed_vars = gs.variables.values().any(is_unnamed);
    fixed
}
