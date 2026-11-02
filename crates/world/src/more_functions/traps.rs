//! Traps: `SetVATSTarget` (with [`vats_targetable`], the test V.A.T.S.
//! makes of an object) and `FireWeapon` (with [`shot_from`], where the shot
//! leaves and which way it goes). Notes: `docs/DEAD_MONEY.md` "Traps".

use std::collections::BTreeSet;

use esm::{FormId, LoadOrder};
use nif::math::{Transform, Vec3};

use super::destruction::Destructible;
use super::{kind_of, placed, Shown};
use crate::scripting::{Event, GameState, Runner, Value};
use crate::RotationConvention;

/// The functions here, by the game's own names.
pub const FUNCTIONS: &[&str] = &["SetVATSTarget", "FireWeapon"];

/// References whose V.A.T.S. targeting is the opposite of their base's
/// (the reference's flag 0x04000000, set by `004846e0`), saved.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VatsOverrides(pub BTreeSet<FormId>);

/// The node a weapon's shot leaves a placed object from, when its model
/// has one (`00525700`; a weapon's own node name, when it has one, isn't
/// read here): first this name, then [`PROJECTILE_NODE_ALT`].
pub const PROJECTILE_NODE: &str = "ProjectileNode";
pub const PROJECTILE_NODE_ALT: &str = "##ProjectileNode";

/// Carries out one of [`FUNCTIONS`]; `on` is the reference the script runs
/// on.
pub(super) fn carry_out(
    runner: &mut Runner,
    name: &str,
    on: Option<FormId>,
    args: &[Value],
) -> Option<f64> {
    let order = runner.order;
    let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
    match name {
        // `005daae0`: on a destructible reference, targeting wanted (the
        // number not 0) the same as its base's flag clears the override,
        // else sets it. Always succeeds.
        "SetVATSTarget" => {
            let r = on?;
            if destructible(order, runner.state, r).is_some() {
                let wanted = arg(0).number() as i32 != 0;
                if base_targetable(order, runner.state, r) == wanted {
                    runner.state.more.vats_overrides.0.remove(&r);
                } else {
                    runner.state.more.vats_overrides.0.insert(r);
                }
            }
            Some(1.0)
        }
        // `005da570`: the reference fires the weapon (`00523150`). Not a
        // weapon: the game only reports it.
        "FireWeapon" => {
            let from = on?;
            let weapon = arg(0).form();
            if !kind_of(order, weapon).is_some_and(|k| k.as_bytes() == b"WEAP") {
                println!(
                    "SCRIPTS: FireWeapon in script on '{from}' called with non-weapon parameter."
                );
                return Some(1.0);
            }
            runner
                .state
                .events
                .push(Event::More(Shown::WeaponFired { from, weapon }));
            Some(1.0)
        }
        _ => None,
    }
}

/// The destruction data of a reference's base. The game asks the
/// reference's flag 0x01000000 (`00452370`), which its destruction code
/// (`00477d10`) also asks; here a reference whose base has destruction
/// data counts as having it.
fn destructible(order: &LoadOrder, state: &GameState, r: FormId) -> Option<Destructible> {
    let base = placed::base_now(order, state, r)?;
    Destructible::load(order, base)
}

/// The base's own "V.A.T.S. targetable" flag (`00576100`: `DEST` flags
/// bit 0x01).
fn base_targetable(order: &LoadOrder, state: &GameState, r: FormId) -> bool {
    destructible(order, state, r).is_some_and(|d| d.flags & 0x01 != 0)
}

/// Whether V.A.T.S. may target an object (`00576070`, asked by its target
/// gathering `007f52c0`): a destructible reference whose base's flag says
/// so, turned the other way by `SetVATSTarget`'s override.
pub fn vats_targetable(order: &LoadOrder, state: &GameState, r: FormId) -> bool {
    if destructible(order, state, r).is_none() {
        return false;
    }
    base_targetable(order, state, r) != state.more.vats_overrides.0.contains(&r)
}

/// Where a shot from a placed object leaves and which way it flies (game
/// space, a unit vector), as the game's weapon fire (`00523150`) works it
/// out for something that isn't an actor: from the object's position along
/// its own angles, or, when its model has a projectile node (`node`, in the
/// model's space), from that node along the node's facing. Facing is the
/// model's +Y.
pub fn shot_from(
    position: Vec3,
    angles: Vec3,
    scale: f32,
    node: Option<Transform>,
) -> (Vec3, Vec3) {
    let placed = RotationConvention::DEFAULT.transform(position, angles, scale);
    let world = match node {
        Some(n) => placed.then_child(&n),
        None => placed,
    };
    let r = world.rotation;
    let forward = [r[0][1], r[1][1], r[2][1]];
    let length = (forward[0] * forward[0] + forward[1] * forward[1] + forward[2] * forward[2])
        .sqrt()
        .max(1e-6);
    (
        world.translation,
        [
            forward[0] / length,
            forward[1] / length,
            forward[2] / length,
        ],
    )
}

/// Saved lines.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    for r in &state.more.vats_overrides.0 {
        line(format!("vatsoverride {:08X}", r.0));
    }
}

/// A saved line back.
pub(crate) fn load_line(state: &mut GameState, parts: &[&str]) -> Option<Result<(), String>> {
    if *parts.first()? != "vatsoverride" {
        return None;
    }
    Some(
        match parts.get(1).and_then(|s| u32::from_str_radix(s, 16).ok()) {
            Some(r) => {
                state.more.vats_overrides.0.insert(FormId(r));
                Ok(())
            }
            None => Err(format!("can't read '{}'", parts.join(" "))),
        },
    )
}
