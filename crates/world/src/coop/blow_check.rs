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
//! - a V.A.T.S. attack (the blow's attack kind) is paid from the action
//!   points of the host's copy of the guest: the host's price of that kind
//!   ([`crate::vats::prices`]), shared by a shot's pellets. A queued shot
//!   was paid when the host resolved its queue. Without the points, the
//!   blow counts as a plain attack.
//!
//! A part the host does not grant is dropped from the blow, not the blow.
//! A wrong weapon or follower refuses the blow.
//!
//! An explosion's blow ([`BlastBlow`]) is counted as the guest's explosion
//! hit is (`Runner::explosion_hit`), from the host's own data:
//! - the weapon must be in the striker's hand or carried in the host's
//!   copy, or the copy of that guest held it in hand earlier in the session
//!   (a grenade thrown or a mine set leaves the hand before it goes off);
//! - the explosion must be one that weapon makes
//!   ([`crate::explosions::weapon_explosions`]);
//! - its centre must be within [`MAX_BLAST_REACH`] of the guest's puppet;
//! - the damage is the explosion's base damage for the host's copy of the
//!   striker ([`crate::explosions::base_damage`]), shared by the falloff at
//!   the victim's place in the host's world, then through the armour
//!   ([`crate::explosions::hit_damage`]). A victim outside the radius there
//!   takes nothing;
//! - the explosion's object effect (an incendiary's fire, a pulse's EMP) is
//!   cast on the victim in the host's world, by the guest's puppet or its
//!   follower (as [`crate::explosions::cast_enchantment`]). The host's
//!   world deals that effect's damage. The ledger credits the guest with
//!   the damage that the caster deals (an effect at once,
//!   `LiveHostAdapter`). The guest does not send its own effect damage.

use std::collections::BTreeSet;

use coop_core::actors::{BlastBlow, Blow};
use coop_core::combat::MAX_BLAST_REACH;
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
    /// The explosion is not one that the weapon makes.
    NotItsExplosion,
    /// The explosion's centre is too far from the guest.
    OutOfReach,
    /// The victim is outside the explosion's radius in the host's world.
    OutsideRadius,
}

/// A blow as the host counts it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Counted {
    /// The health damage (after the party scaling, as a guest's hit has it).
    pub damage: f64,
    /// The [`Blow`] flags the host did not grant.
    pub dropped: u8,
    /// The host did not grant the V.A.T.S. attack (no points in its copy).
    pub vats_dropped: bool,
    /// The action points the host's copy of the guest paid for the blow.
    pub ap: f32,
    /// The explosion's object effect (`ENCH`) that the host's world casts
    /// on the victim (#74).
    pub effect: Option<FormId>,
}

/// How the host pays for a V.A.T.S. blow.
#[derive(Clone, Copy)]
pub enum Pay<'a> {
    /// From the copy's action points, at the host's prices.
    Points(&'a crate::vats::Settings),
    /// Paid already: a queued shot (`VatsHost::resolve`).
    Paid,
}

/// The host's count of the `blows` of player `from` on `victim`, one result
/// for each blow. `held`: the weapons that the host's copy of `from` held in
/// hand earlier in the session.
pub fn count_blows(
    order: &LoadOrder,
    gs: &mut GameState,
    from: PlayerId,
    victim: FormId,
    blows: &[Blow],
    held: &BTreeSet<FormId>,
    pay: Pay,
) -> Vec<Result<Counted, Refusal>> {
    let puppet = puppet_of(from);
    let places = Places {
        victim: gs.place(order, victim).map(|p| p.2),
        guest: gs.place(order, puppet).map(|p| p.2),
    };
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
                match b.blast {
                    Some(blast) => {
                        let c = count_blast(order, g, b, blast, victim, places, held)?;
                        // the explosion's object effect on the victim
                        // (`009b00a0`), cast by the striker as the host's
                        // world knows it: the guest's puppet, else its
                        // follower. The host's world deals its damage.
                        if let Some(effect) = c.effect {
                            let caster = b.by.map_or(puppet, |f| FormId(f.0));
                            crate::magic::apply(order, g, victim, effect, caster, false);
                        }
                        Ok(c)
                    }
                    None => count_one(order, g, b, victim, sneaking, unseen, pay),
                }
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
    pay: Pay,
) -> Result<Counted, Refusal> {
    let attacker = b.by.map_or(PLAYER_REF, |f| FormId(f.0));
    let weapon = combat::weapon_in_hand(order, g, attacker);
    if weapon.as_ref().map(|w| CForm(w.form_id.0)) != b.weapon {
        return Err(Refusal::NotInHand);
    }
    let melee = weapon.as_ref().map_or(true, |w| w.is_melee());
    // A V.A.T.S. attack's price, before the pellets share the damage.
    let prices = match pay {
        Pay::Points(s) => Some(crate::vats::prices(order, g, s)),
        Pay::Paid => None,
    };
    // A shot's pellets share its damage (`viewer::combat`, `world::combat::Weapon::shot`).
    let mut pellets = 1u32;
    let weapon = weapon.map(|mut w| {
        if !melee {
            let ammo = w.ammo_in_use(order, g, attacker);
            let (count, _) = w.shot(order, ammo);
            pellets = count.max(1);
            w.damage /= pellets as f32;
        }
        w
    });
    // A V.A.T.S. attack pays its price from the copy's action points, shared
    // by the shot's pellets (`vats::spend`, as the guest's own attack pays).
    let mut kind = b.vats.filter(|_| attacker == PLAYER_REF);
    let mut ap = 0.0;
    if let (Some(k), Some(prices)) = (kind, prices) {
        let price = prices.get(&k).map(|c| c / pellets as f32);
        match price {
            Some(c) if crate::vats::action_points(order, g) >= c => {
                crate::vats::spend(g, c);
                ap = c;
            }
            _ => kind = None,
        }
    }
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
    if let Some(kind) = kind {
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
        vats_dropped: b.vats.is_some() && kind.is_none(),
        ap,
        effect: None,
    })
}

/// Where the victim and the guest's puppet are in the host's world.
#[derive(Clone, Copy)]
struct Places {
    victim: Option<[f32; 3]>,
    guest: Option<[f32; 3]>,
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    let d = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

/// One explosion's blow, with the guest's rows in `PLAYER_REF` (`with_player`).
fn count_blast(
    order: &LoadOrder,
    g: &mut GameState,
    b: &Blow,
    blast: BlastBlow,
    victim: FormId,
    places: Places,
    held: &BTreeSet<FormId>,
) -> Result<Counted, Refusal> {
    let attacker = b.by.map_or(PLAYER_REF, |f| FormId(f.0));
    let id = FormId(b.weapon.ok_or(Refusal::NotInHand)?.0);
    let in_hand = combat::weapon_in_hand(order, g, attacker).is_some_and(|w| w.form_id == id);
    let carried = g.item_count(order, attacker, id) > 0;
    let had = attacker == PLAYER_REF && held.contains(&id);
    if !(in_hand || carried || had) {
        return Err(Refusal::NotInHand);
    }
    let weapon = combat::Weapon::load(order, id).ok_or(Refusal::NotInHand)?;
    let explosion = FormId(blast.explosion.0);
    if !crate::explosions::weapon_explosions(order, &weapon).contains(&explosion) {
        return Err(Refusal::NotItsExplosion);
    }
    let e = crate::explosions::ExplosionRecord::load(order, explosion)
        .ok_or(Refusal::NotItsExplosion)?;
    let at = blast.at.map(coop_core::avatar::dequant_pos);
    let (Some(victim_at), Some(guest_at)) = (places.victim, places.guest) else {
        return Err(Refusal::OutOfReach);
    };
    if distance(at, guest_at) > MAX_BLAST_REACH {
        return Err(Refusal::OutOfReach);
    }
    let radius = crate::explosions::blast_radius(order, g, Some(attacker), Some(id), &e);
    let share = crate::explosions::falloff(radius, distance(at, victim_at));
    if share <= 0.0 {
        return Err(Refusal::OutsideRadius);
    }
    let base = crate::explosions::base_damage(order, g, Some(attacker), Some(&weapon), &e);
    // The best roll (0) still misses: no critical chance at all.
    let critical = b.has(Blow::CRITICAL)
        && combat::critical(order, g, attacker, Some(&weapon), victim, false, 0);
    let dealt = crate::explosions::hit_damage(
        order,
        g,
        base * share,
        Some(attacker),
        Some(&weapon),
        victim,
        critical,
    );
    let granted = if critical { Blow::CRITICAL } else { 0 };
    Ok(Counted {
        damage: super::facts::scaled_damage(g, victim, attacker, f64::from(dealt)),
        dropped: b.flags & !granted,
        vats_dropped: false,
        ap: 0.0,
        effect: e.enchantment,
    })
}
