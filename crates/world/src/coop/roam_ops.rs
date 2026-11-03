//! Script commands on references in cells the host has not loaded.
//!
//! Quest and story scripts run only on the host, which loads only the scene
//! around its own player. Most commands act on state the `GameState` keeps
//! by FormID (enabled or disabled, position, inventory, actor values,
//! ownership, locks, quest stages), and that state is global: it applies to
//! the reference whenever any peer loads its cell, with no queue. A few
//! commands need a live entity (an animation, an AI package, combat, an
//! activation that opens something). For those the host queues the command
//! with the reference's cell when the cell is not loaded here, and replays
//! it, in order, when the cell is loaded by the host or forwarded to the
//! guest that loaded it.
//!
//! [`classify`] is the one table of which commands need a loaded entity.
//! Unknown commands are state commands (they run immediately, as today).

use coop_core::ids::CellId;
use coop_core::roam::{DArg, DeferredOp};
use esm::{FormId, LoadOrder};

use super::facts::player_of;
use super::Authority;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{GameState, Runner, Value};

/// How a script command treats an unloaded reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpClass {
    /// Works on `GameState` keyed by FormID: runs at once, applies when the
    /// cell loads.
    State,
    /// Needs a loaded entity: waits for the cell.
    NeedsLoaded,
}

/// Commands that act on a live entity (3D, animation, AI, physics).
const NEEDS_LOADED: &[&str] = &[
    "Activate",
    "PlayGroup",
    "PlayIdle",
    "PlayIdleEx",
    "StopCombat",
    "StopCombatAlarm",
    "StartCombat",
    "StartConversation",
    "SayTo",
    "Say",
    "Look",
    "StopLook",
    "EvaluatePackage",
    "SetRestrained",
    "SetAlert",
    "SetUnconscious",
    "PushActorAway",
    "KnockDown",
    "ForceFlee",
    "ForceTakeCover",
    "AddScriptPackage",
    "RemoveScriptPackage",
    "DropMe",
    "Dispel",
    "CastSpell",
    "PlaceAtMe",
    "CreateFullActorCopy",
    "PlaySound",
    "PlaySound3D",
    "StopSound",
    "EquipItem",
    "UnequipItem",
    "ModifyFaceGen",
    "PlayMagicEffectVisuals",
    "PlayMagicShaderVisuals",
    "SetCombatStyle",
    "RewardKarma",
];

/// Which class a function name belongs to.
pub fn classify(func: &str) -> OpClass {
    if NEEDS_LOADED.iter().any(|n| n.eq_ignore_ascii_case(func)) {
        OpClass::NeedsLoaded
    } else {
        OpClass::State
    }
}

fn to_darg(v: &Value) -> DArg {
    match v {
        Value::Form(f) => DArg::Form(f.0),
        Value::Number(n) => DArg::num(*n),
        Value::Text(t) => DArg::Text(t.clone()),
    }
}

fn from_darg(a: &DArg) -> Value {
    match a {
        DArg::Form(f) => Value::Form(FormId(*f)),
        DArg::Num(_) => Value::Number(a.as_num().unwrap_or(0.0)),
        DArg::Text(t) => Value::Text(t.clone()),
    }
}

/// The cell a reference stands in now.
fn cell_of(gs: &GameState, order: &LoadOrder, r: FormId) -> Option<FormId> {
    gs.place(order, r).map(|p| p.1)
}

/// The hook in the script `Host::call`: returns `Some(value)` when the
/// command was queued for later, `None` to run it now. Runs on the host
/// only, and only once the session has reported the host's loaded cells.
pub fn defer(
    gs: &mut GameState,
    order: &LoadOrder,
    func: &str,
    target: Option<FormId>,
    args: &[Value],
) -> Option<f64> {
    if gs.coop.authority != Authority::Host {
        return None;
    }
    let loaded = gs.coop.live.host_loaded.as_ref()?;
    let t = target?;
    if t == PLAYER_REF || player_of(t).is_some() || classify(func) != OpClass::NeedsLoaded {
        return None;
    }
    let cell = cell_of(gs, order, t)?;
    if loaded.contains(&cell.0) {
        return None;
    }
    gs.coop.live.deferred_out.push((
        t,
        cell,
        func.to_string(),
        args.iter().map(to_darg).collect(),
    ));
    Some(0.0)
}

/// Replays a queued operation on the host (its cell is loaded now).
pub fn replay(r: &mut Runner<'_>, op: &DeferredOp) -> Option<f64> {
    let args: Vec<Value> = op.args.iter().map(from_darg).collect();
    r.apply_call(&op.func, Some(FormId(op.target.0)), &args)
}

/// Registers every exterior cell of a worldspace in the shared cell table,
/// so loaded sets, interest and the tether zone span grid neighbours. The
/// viewer calls this once per worldspace it loads (host side, and guests
/// that want puppet culling to use the same table).
pub fn register_world_grid(
    graph: &mut coop_core::roam::CellGraph,
    grid: &crate::exterior::WorldGrid,
) {
    for ((x, y), cell) in &grid.cells {
        graph.register_exterior(CellId(cell.0), grid.world.form_id.0, *x, *y);
    }
}

/// Registers one door pair or interior link.
pub fn register_link(graph: &mut coop_core::roam::CellGraph, a: FormId, b: FormId) {
    graph.link(CellId(a.0), CellId(b.0));
}

/// The cell id of an operation, as the scene model keys it.
pub fn cell_id(c: FormId) -> CellId {
    CellId(c.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification_is_case_insensitive_and_defaults_to_state() {
        assert_eq!(classify("playgroup"), OpClass::NeedsLoaded);
        assert_eq!(classify("StartCombat"), OpClass::NeedsLoaded);
        for state in [
            "Enable",
            "Disable",
            "MoveTo",
            "SetPos",
            "AddItem",
            "RemoveItem",
            "SetStage",
            "SetAV",
            "ModAV",
            "Kill",
            "Lock",
            "Unlock",
            "SetOwnership",
            "SomethingNew",
        ] {
            assert_eq!(classify(state), OpClass::State, "{state}");
        }
    }

    #[test]
    fn arguments_survive_the_queue() {
        let v = [
            Value::Form(FormId(7)),
            Value::Number(1.5),
            Value::Text("x".into()),
        ];
        let back: Vec<Value> = v.iter().map(|a| from_darg(&to_darg(a))).collect();
        assert_eq!(back, v);
    }
}
