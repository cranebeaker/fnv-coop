//! The host's count of a guest's blows (#74). A guest names each blow of its
//! player or followers ([`Blow`]). The host computes the blow's damage with
//! the same function the guest's blow uses ([`crate::blow::blow_damage`]),
//! from its own copy of that guest ([`GameState::with_player`]) and its own
//! copy of the victim. The guest's number is not used.
//!
//! The host decides each part of the blow from what it knows:
//! - the weapon must be the one in the striker's hand in the host's copy;
//!   a gun's shot shares its damage among its pellets, as the guest's shot does;
//! - a follower must be one of that guest's followers;
//! - a sneak attack needs the guest sneaking (its avatar) and a victim that
//!   does not detect the guest's puppet in the host's world;
//! - a power attack needs a melee weapon or fists, and no sneaking;
//! - a critical hit needs a chance above 0 for this striker, weapon and
//!   victim (the roll itself is the guest's).
//!
//! A part the host does not grant is dropped from the blow, not the blow.
//! A wrong weapon or follower refuses the blow.

use coop_core::actors::Blow;
use coop_core::ids::{FormId as CForm, PlayerId};
use esm::{FormId, LoadOrder};

use super::facts::puppet_of;
use crate::blow::{blow_damage, BlowInputs};
use crate::combat;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{Facts, GameState};

/// Why the host refuses a blow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The striker is not one of that player's followers.
    NotAFollower,
    /// The weapon is not the one in the striker's hand in the host's copy.
    NotInHand,
}

/// A blow as the host counts it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Counted {
    /// The health damage (after the party scaling, as a guest's hit has it).
    pub damage: f64,
    /// The [`Blow`] flags the host did not grant.
    pub dropped: u8,
}

/// The host's count of the `blows` of player `from` on `victim`, one result
/// for each blow.
pub fn count_blows(
    order: &LoadOrder,
    gs: &mut GameState,
    from: PlayerId,
    victim: FormId,
    blows: &[Blow],
) -> Vec<Result<Counted, Refusal>> {
    let puppet = puppet_of(from);
    let sneaking = super::combat_live::stealth_of(gs, puppet).is_some_and(|s| s.sneaking);
    let unseen = Facts {
        order,
        state: gs,
        speaker: None,
    }
    .detection(victim, puppet)
    .map_or(true, |v| v < 1);
    let followers: Vec<bool> = blows
        .iter()
        .map(|b| {
            b.by.map_or(true, |f| {
                super::campaign::owners::owner_of(gs, FormId(f.0)) == Some(from.0)
            })
        })
        .collect();
    gs.with_player(from.0, |g| {
        blows
            .iter()
            .zip(followers)
            .map(|(b, follower)| {
                if !follower {
                    return Err(Refusal::NotAFollower);
                }
                count_one(order, g, b, victim, sneaking, unseen)
            })
            .collect()
    })
}

/// One blow, with the guest's rows in `PLAYER_REF` (`with_player`).
fn count_one(
    order: &LoadOrder,
    g: &mut GameState,
    b: &Blow,
    victim: FormId,
    sneaking: bool,
    unseen: bool,
) -> Result<Counted, Refusal> {
    let attacker = b.by.map_or(PLAYER_REF, |f| FormId(f.0));
    let weapon = combat::weapon_in_hand(order, g, attacker);
    if weapon.as_ref().map(|w| CForm(w.form_id.0)) != b.weapon {
        return Err(Refusal::NotInHand);
    }
    let melee = weapon.as_ref().map_or(true, |w| w.is_melee());
    // A shot's pellets share its damage (`viewer::combat`, `world::combat::Weapon::shot`).
    let weapon = weapon.map(|mut w| {
        if !melee {
            let ammo = w.ammo_in_use(order, g, attacker);
            let (count, _) = w.shot(order, ammo);
            w.damage /= count.max(1) as f32;
        }
        w
    });
    let mut flags = b.flags;
    if attacker != PLAYER_REF || !sneaking || !unseen {
        flags &= !Blow::SNEAK;
    }
    // A power attack is a melee or unarmed move, and not while sneaking
    // (`viewer::combat`: `attack.power && !player_sneaking`).
    if !melee || (attacker == PLAYER_REF && sneaking) {
        flags &= !Blow::POWER;
    }
    // The V.A.T.S. hit playing on the guest, for its damage and critical
    // multipliers (`vats::attack_damage_mult`, `vats::critical_bonus`).
    let saved = (g.vats.mode, g.vats.attack.take());
    if let Some(kind) = b.vats.filter(|_| attacker == PLAYER_REF) {
        g.vats.mode = crate::vats::mode::PLAYBACK;
        g.vats.attack = Some(crate::vats::AttackFacts {
            weapon: weapon.as_ref().map(|w| w.form_id),
            target: victim,
            part_av: -1,
            kind,
            hit: true,
            distance: 0.0,
            paralyzing_palm: false,
        });
    }
    let sneak = flags & Blow::SNEAK != 0;
    // The best roll (0) still misses: no critical chance at all.
    if flags & Blow::CRITICAL != 0
        && !combat::critical(order, g, attacker, weapon.as_ref(), victim, sneak, 0)
    {
        flags &= !Blow::CRITICAL;
    }
    let d = blow_damage(
        order,
        g,
        BlowInputs {
            attacker,
            target: victim,
            weapon: weapon.as_ref(),
            part: b.part,
            power: flags & Blow::POWER != 0,
            sneak,
            critical: flags & Blow::CRITICAL != 0,
        },
    );
    (g.vats.mode, g.vats.attack) = saved;
    Ok(Counted {
        damage: super::facts::scaled_damage(g, victim, attacker, f64::from(d.dealt)),
        dropped: b.flags & !flags,
    })
}
