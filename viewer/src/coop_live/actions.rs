//! Combat actions in co-op: the local player's swings, shots, reloads,
//! draws, blocks, hits taken and death become events for the other players
//! ([`emit`]); the events that arrive are played on the puppet with the
//! animation groups the game data gives the weapon ([`play`]).

use bevy::prelude::*;
use coop_session::ActionKind;
use esm::FormId;
use world::animation::group;
use world::dialogue::PLAYER_REF;

use crate::actors::ActorRig;
use crate::ai::Walker;
use crate::combat::PlayerAttack;
use crate::dialogue::DialogueState;
use crate::GameFiles;

/// What the last frame looked like, to find what changed.
#[derive(Default)]
pub struct Seen {
    fired: Option<f32>,
    reload: Option<f32>,
    readied: Option<(f32, bool)>,
    blocking: bool,
    aiming: bool,
    health: Option<f64>,
    dead: bool,
}

/// Queues an event for each combat change of the local player; the live
/// layer sends them (`gs.coop.live.local_actions`).
pub fn emit(
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    attack: Res<PlayerAttack>,
    mut seen: Local<Seen>,
) {
    let order = &game.0.order;
    let gs = &mut state.0;
    let weapon = world::combat::weapon_equipped(order, gs, PLAYER_REF);
    let mut out: Vec<(ActionKind, u32)> = Vec::new();
    if attack.fired_at != seen.fired {
        if attack.fired_at.is_some() {
            let kind = if weapon.as_ref().is_none_or(|w| w.is_melee()) {
                ActionKind::Swing
            } else {
                ActionKind::Shoot
            };
            out.push((kind, u32::from(attack.attack_group)));
        }
        seen.fired = attack.fired_at;
    }
    if attack.reload_started != seen.reload {
        if let (Some(_), Some(w)) = (attack.reload_started, weapon.as_ref()) {
            out.push((
                ActionKind::Reload,
                u32::from(group::RELOAD_A + w.reload_animation.min(22)),
            ));
        }
        seen.reload = attack.reload_started;
    }
    if attack.readied_at != seen.readied {
        if let Some((_, drawn)) = attack.readied_at {
            let held = weapon.as_ref().map_or(0, |w| w.form_id.0);
            out.push(if drawn {
                (ActionKind::Draw, held)
            } else {
                (ActionKind::Holster, 0)
            });
        }
        seen.readied = attack.readied_at;
    }
    if attack.blocking && !seen.blocking {
        out.push((ActionKind::Block, 0));
    }
    if attack.iron_sights && !seen.aiming {
        out.push((ActionKind::Aim, 0));
    }
    seen.blocking = attack.blocking;
    seen.aiming = attack.iron_sights;
    let health = world::combat::health(order, gs, PLAYER_REF);
    if let (Some(before), Some(now)) = (seen.health, health) {
        if now < before - 0.5 {
            out.push((ActionKind::Hurt, (before - now).ceil() as u32));
        }
    }
    seen.health = health;
    let dead = gs.dead.contains(&PLAYER_REF);
    if dead && !seen.dead {
        out.push((ActionKind::Death, 0));
    }
    seen.dead = dead;
    gs.coop.live.local_actions.extend(out);
}

/// Plays one event of a remote player on its puppet.
pub fn play(
    rig: &mut ActorRig,
    walker: &Walker,
    gs: &mut world::scripting::GameState,
    puppet: FormId,
    now: f32,
    kind: ActionKind,
    arg: u32,
) {
    let attack_group = |g: u32| match g {
        26..=0xa8 => g as u8,
        _ => group::ATTACK_RIGHT,
    };
    match kind {
        ActionKind::Swing | ActionKind::Shoot => {
            // The picker drops an attack asked for while its weapon is not
            // drawn. A swinger with bare fists demonstrably has them up, but
            // the fists have no equip to play, so the picker may never have
            // learned it: raise them now, or the swing is lost and the
            // puppet stands with its arms down.
            if fists_only(rig) {
                rig.want_drawn = true;
                rig.picker.drawn = true;
            }
            rig.attack_group = attack_group(arg);
            rig.attack_at = Some(now);
        }
        ActionKind::Reload => rig.reload_at = Some((now, arg.min(0xff) as u8)),
        ActionKind::Draw => rig.want_drawn = true,
        ActionKind::Holster => rig.want_drawn = false,
        // the hit reaction the game's idle tree answers a hit with (`sitting::idle_requests`)
        ActionKind::Hurt | ActionKind::Knockdown => {
            gs.hits_taken
                .push((puppet, i32::from(world::body_parts::part::TORSO), false))
        }
        ActionKind::Death => {
            if rig.ragdoll.is_none() {
                rig.go_limp(now, walker.placement(), None, None);
            }
        }
        // no third-person animation of their own to start here
        ActionKind::Aim | ActionKind::Block => {}
    }
}

/// The puppet holds nothing (bare fists), as far as its dressing has said.
pub fn fists_only(rig: &ActorRig) -> bool {
    matches!(rig.held_weapon, Some(None))
}
