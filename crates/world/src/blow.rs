//! One blow's damage without its side effects: what
//! [`crate::scripting::Runner::blow_at`] deals, and what a co-op host counts
//! for a guest's blow from its own copy of that guest
//! (`crate::coop::blow_check`). The arithmetic and its order are `blow_at`'s
//! (`009b5a30` and the functions it calls, see each one).

use esm::{FormId, LoadOrder};

use crate::body_parts::PartHit;
use crate::combat::{self, Weapon};
use crate::scripting::GameState;

/// What decides one blow, besides the two actors' state.
#[derive(Clone, Copy, Debug)]
pub struct BlowInputs<'a> {
    pub attacker: FormId,
    pub target: FormId,
    /// The weapon as it struck (a shot's pellet carries its share of the
    /// damage). `None`: fists, or a creature's own attack.
    pub weapon: Option<&'a Weapon>,
    /// The body part type hit.
    pub part: Option<u8>,
    pub power: bool,
    /// A sneak attack (the target did not detect the attacker).
    pub sneak: bool,
    /// A critical hit (the attacker's roll, [`combat::critical`]).
    pub critical: bool,
}

/// One blow's numbers.
#[derive(Clone, Debug)]
pub struct BlowDamage {
    /// The health damage the target takes.
    pub dealt: f32,
    /// The part's multiplier, with the fists' power bonus and the sneak
    /// attack's multiplier (0: no part of the body found).
    pub multiplier: f32,
    /// The fatigue the target takes.
    pub fatigue: f32,
    /// The condition points the target's armour loses.
    pub armour_damage: f32,
    /// Where the blow lands.
    pub at: PartHit,
}

/// The damage of one blow: the weapon's (or the creature's, or the fists')
/// damage, the V.A.T.S. and Nerve multipliers, the critical damage, the
/// armour, the part hit and the sneak attack's multiplier. It reads the
/// state and changes nothing.
pub fn blow_damage(order: &LoadOrder, state: &GameState, b: BlowInputs<'_>) -> BlowDamage {
    let BlowInputs {
        attacker,
        target,
        weapon,
        part,
        power,
        sneak,
        critical,
    } = b;
    let weapon_id = weapon.map(|w| w.form_id);
    // Fists' power attack: a bonus multiplier after the armour
    // (`world::combat::fists_power_bonus`); a creature's own attack has none.
    let mut fists_bonus = 0.0;
    let mut fatigue = 0.0;
    let mut damage = match weapon {
        Some(w) => combat::weapon_damage(order, state, attacker, Some(w), power),
        // Bare fists' fatigue damage, half their damage (`00646310`,
        // `world::fatigue`).
        None => combat::creature_damage(order, attacker).unwrap_or_else(|| {
            fists_bonus = combat::fists_power_bonus(order, power);
            let fists = combat::weapon_damage(order, state, attacker, None, false);
            fatigue = crate::fatigue::fists_fatigue(order, state, attacker, target, fists);
            fists
        }),
    };
    // V.A.T.S.'s melee moves and automatic melee weapons (`world::vats`).
    damage *= crate::vats::attack_damage_mult(order, state, attacker, weapon);
    // A teammate's Nerve once more for a melee, unarmed or creature attack
    // (`009b5170`; on top of `00644ce0`'s for weapons and fists); a shot's
    // damage (`009b5650`) has it once.
    if weapon.map_or(true, |w| w.is_melee()) {
        damage *= crate::companions::nerve(order, state, attacker);
    }
    if critical {
        damage += combat::critical_damage(order, state, attacker, weapon, target, damage);
    }
    let ammo = weapon.and_then(|w| w.ammo_in_use(order, state, attacker));
    let armoured = combat::armour_hit(
        order,
        state,
        damage,
        Some((attacker, weapon_id)),
        target,
        ammo,
    );
    let vats_mult = crate::vats::player_damage_mult(order, state, target);
    let after_armour = armoured.damage * vats_mult;
    // The fatigue damage through the armour, the ammunition's added
    // (`009b5a30`, `world::fatigue::through_armour`).
    fatigue = crate::fatigue::through_armour(
        order,
        fatigue,
        ammo,
        weapon.is_some(),
        damage,
        armoured.damage,
    ) * vats_mult;
    // A hit doing fatigue damage wears no armour (`009b5a30`).
    let armour_damage = if fatigue > 0.0 {
        0.0
    } else {
        armoured.armour_damage
    } * vats_mult;
    // The part it landed on: limb damage from the damage after armour, and
    // the hit's multiplier (`009b6620`).
    let at = crate::body_parts::part_hit(order, state, target, weapon, part, after_armour);
    let mut dealt = at.health_damage;
    // The multiplier last (`009b73d0`): the part's (or a fists' power
    // attack's bonus, the larger: `009b7840`), a sneak attack's bonus when
    // it's at least 1, then the damage × it.
    let mut multiplier = at.multiplier.max(fists_bonus);
    if multiplier > 0.0 {
        if multiplier >= 1.0 && sneak {
            multiplier *= combat::sneak_multiplier(order, weapon.map_or(true, |w| w.is_melee()));
        }
        dealt *= multiplier;
        // An unarmed blow's fatigue takes it too (`009b73d0`: attacker not a
        // creature, the hit's skill Unarmed).
        if weapon.map_or(true, |w| w.skill == combat::av::UNARMED)
            && !combat::is_creature(order, attacker)
        {
            fatigue *= multiplier;
        }
    }
    BlowDamage {
        dealt,
        multiplier,
        fatigue,
        armour_damage,
        at,
    }
}
