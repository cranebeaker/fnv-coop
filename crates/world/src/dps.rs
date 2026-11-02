//! The damage-per-second figure and what it chooses: the game's
//! `CombatFormulas::CalcWeaponDamagePerSecond` (Xbox PDB, `00645380`) with
//! every argument its callers pass, the rate of fire it reads off the
//! player's animations (`CombatFormulas::GetWeaponShotsPerSecond`,
//! `00645dc0`), the combat rating built on it
//! (`CombatFormulas::CalcCombatWeaponDPS`, `00646060`), and the weapon an
//! actor picks to hold (`InventoryChanges::GetBestWeapon`, `004c7400`).
//! Read from `FalloutNV.exe` 1.4.0.525 and the Xbox 360 prototype's
//! decompile (`x360_weapons.c`); `docs/COMPANIONS.md` has the rules.
//!
//! The figure itself (`00645380`), as every caller asks it (its five
//! callers pass 0 for the arguments it has beyond these):
//!
//! - `who`'s weapon (none: their fists) at a condition 0..1, 0 when it's
//!   broken;
//! - damage = `DATA` damage × `fDamageWeaponMult` (+ a fitted damage mod),
//!   × 1.3 with a split beam fitted, through the ammunition's damage
//!   effects, + the projectiles × the projectile's explosion damage;
//!   grenades and mines their explosion × `fDamageWeaponMult` instead;
//! - × (`fDamageSkillBase` + `fDamageSkillMult` × skill ÷ 100), then the
//!   holder's perks' "Calculate Weapon Damage" (entry point 0) when asked
//!   (the item cards ask, the combat rating doesn't);
//! - the clip (with a clip mod) over the time it takes to fire, the jams
//!   and the reload, the semi-automatic delay and automatic bursts for
//!   others than the player, the critical share; × the condition's
//!   multiplier (`npc_combat::dps_with` has the arithmetic);
//! - the shots a second: the weapon's `DNAM` attack shots a second, or,
//!   when an inventory entry is passed (the cards and `GetBestWeapon` pass
//!   one), [`shots_per_second`] — which reads **the player's** third-person
//!   attack animation for semi-automatic weapons, whoever holds the gun.
//! - the ammunition: the one passed (the card passes what the player's gun
//!   is loaded with), else the weapon's own first.

use esm::{FormId, FourCC, LoadOrder};

use crate::animation::{self, groups, pick::Library};
use crate::combat::{self, Weapon};
use crate::combat_ai::{CombatStyle, Setting};
use crate::dialogue::PLAYER_REF;
use crate::npc_combat::{self, DpsInputs, DpsWeapon};
use crate::perks::{self, Tab};
use crate::scripting::{Facts, GameState};
use crate::weapon_mods;

/// What a damage-per-second figure is asked for (`00645380`'s arguments).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DpsCall<'a> {
    pub who: FormId,
    /// `None`: fists.
    pub weapon: Option<&'a Weapon>,
    /// 0..1.
    pub condition: f32,
    /// The fourth argument: the holder's perks (entry point 0) on the
    /// damage.
    pub perks: bool,
    /// The fifth: an inventory entry — the holder's fitted mods count, and
    /// the shots a second come from [`shots_per_second`].
    pub entry: bool,
    /// The last: the ammunition (`None`, or the weapon itself: the
    /// weapon's own first, `00525980(0)`).
    pub ammo: Option<FormId>,
}

impl<'a> DpsCall<'a> {
    /// As the combat controller asks (`00646060` from `009993c0`): no
    /// perks, no entry, the weapon's own ammunition.
    pub fn controller(who: FormId, weapon: Option<&'a Weapon>, condition: f32) -> DpsCall<'a> {
        DpsCall {
            who,
            weapon,
            condition,
            perks: false,
            entry: false,
            ammo: None,
        }
    }
}

/// A weapon's shots a second as the damage figure reads them with an
/// inventory entry (`CombatFormulas::GetWeaponShotsPerSecond` (Xbox PDB),
/// asked without the reload): for a semi-automatic weapon, when the player
/// has animations (`PlayerCharacter::GetAnimation(0)`, the third-person
/// ones), 1 ÷ the time of the `a:` key ([`animation::action_times`],
/// action 3) of the attack group the weapon plays (`DNAM` byte 41, 0xff
/// meaning `AttackRight`), looked up for the weapon's animation kind as any
/// group is (`00495740`; 0 when the lookup lands on another group);
/// otherwise (automatic, or no animations) its fire rate (`DNAM` 64) × the
/// attack multiplier (`DNAM` 60, + a fitted fire-rate mod). Either × the
/// weapon's speed (`DNAM` 4) × the attack multiplier again. `w` is the
/// weapon as its mods have it ([`weapon_mods::modded`]).
///
/// Translated from 00645dc0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn shots_per_second(w: &Weapon, anims: Option<&mut dyn Library>) -> f32 {
    let automatic = w.flags1 & 0x02 != 0;
    let rate = match anims {
        Some(lib) if !automatic => {
            let group = if w.attack_animation == 0xff {
                animation::group::ATTACK_RIGHT
            } else {
                w.attack_animation
            };
            let id = groups::id(0, groups::weapon_kind(w.animation), group, false);
            let found = lib.set().lookup(id);
            if groups::group_of(found) == group {
                lib.sequence(found).map_or(0.0, |seq| {
                    1.0 / animation::action_times(group, &seq.text_keys)[3]
                })
            } else {
                0.0
            }
        }
        _ => w.fire_rate * w.attack_mult,
    };
    w.speed * rate * w.attack_mult
}

/// The damage a second `call` asks for (`00645380`; see the module notes),
/// with the player's third-person animations when there are any (the
/// shots a second with an entry, [`shots_per_second`]).
///
/// Translated from 00645380 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn weapon_dps(
    order: &LoadOrder,
    state: &GameState,
    call: &DpsCall,
    anims: Option<&mut dyn Library>,
    s: Setting,
) -> f32 {
    let who = call.who;
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let av = |a: u16| facts.current_actor_value(who, a).unwrap_or(0.0) as f32;
    let set = |a: u16| state.actor_values.get(&(who, a)).map(|v| *v as f32);
    let crit_chance =
        set(14).unwrap_or_else(|| s("fAVDCritLuckBase", 0.0) + s("fAVDCritLuckMult", 1.0) * av(11));
    let unarmed_skill = av(combat::av::UNARMED);
    let unarmed_damage = set(56).unwrap_or_else(|| {
        s("fAVDUnarmedDamageBase", 0.5) + s("fAVDUnarmedDamageMult", 0.05) * unarmed_skill
    });
    let creature = combat::is_creature(order, who);
    let weapon = call.weapon.map(|w| {
        let flags = if call.entry {
            weapon_mods::flags(state, who, w.form_id)
        } else {
            0
        };
        let has = |e: i32| weapon_mods::has_effect(order, flags, w.form_id, e);
        // Clip, projectiles and attack multiplier as the mods change them
        // (`004fe160`, `00525b20`, `00646020`).
        let modded = if flags != 0 {
            weapon_mods::modded(order, state, who, w.clone())
        } else {
            w.clone()
        };
        let explosion = npc_combat::explosion_of(order, w);
        let weapon_mult = s("fDamageWeaponMult", 1.0);
        let mut damage = w.damage * weapon_mult
            + weapon_mods::bonus(order, flags, w.form_id, weapon_mods::effect::DAMAGE)
                .unwrap_or(0.0);
        if has(weapon_mods::effect::SPLIT_BEAM) {
            damage *= 1.3;
        }
        let ammo = call
            .ammo
            .filter(|&a| a != w.form_id)
            .or_else(|| w.ammo.first().copied());
        if let Some(a) = ammo {
            damage = npc_combat::ammo_damage(order, a, damage);
        }
        if let Some(e) = &explosion {
            // Each projectile's explosion (`00525b20`: the weapon's count,
            // a split beam's more, or the ammunition's).
            damage += modded.shot(order, ammo).0 as f32 * e.damage;
        }
        let rate = if call.entry {
            shots_per_second(&modded, anims)
        } else {
            w.shots_per_second
        };
        DpsWeapon {
            animation: w.animation,
            damage,
            explosive_damage: explosion.as_ref().map(|e| e.damage * weapon_mult),
            skill: av(w.skill),
            clip: modded.clip,
            shots_per_second: rate,
            reload_time: w.reload_time,
            jam_time: npc_combat::dnam_f32(order, w.form_id, 96).unwrap_or(0.0),
            flags1: w.flags1,
            flags2: w.flags2,
            semi_auto_delay: w.semi_auto_delay,
            crit_chance: crit_chance * w.crit_mult,
            crit_damage: w.crit_damage,
        }
    });
    let perk = |d: f32| {
        if call.perks {
            // `005e58f0(0, holder, weapon, holder, &damage)`.
            perks::apply_for(
                order,
                state,
                who,
                perks::entry::CALCULATE_WEAPON_DAMAGE,
                d,
                &[
                    perks::weapon_tab(call.weapon.map(|w| w.form_id)),
                    Tab::Target(who),
                ],
            )
        } else {
            d
        }
    };
    let d = npc_combat::dps_with(
        &DpsInputs {
            weapon,
            condition: call.condition,
            player: who == PLAYER_REF,
            creature_damage: if creature {
                combat::creature_damage(order, who)
            } else {
                None
            },
            unarmed_damage,
            unarmed_skill,
        },
        &perk,
        s,
    );
    if d.is_finite() {
        d
    } else {
        0.0
    }
}

/// The damage a second the combat rating gives a weapon
/// (`CombatFormulas::CalcCombatWeaponDPS` (Xbox PDB)): 0 for one "not used
/// in normal combat" (flags2 0x40, `00646230`); [`weapon_dps`] without
/// perks ÷ 10 for grenades, 0 for mines; ÷ 10000 for a gun when `style`
/// allows melee only (weapon restrictions 1) or for melee and fists when
/// it allows ranged only (2). The companion wheel's Ranged/Melee switch
/// works through this: its lines set `FollowersCombatStyleRanged` /
/// `…Melee`.
///
/// Translated from 00646060 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn combat_weapon_dps(
    order: &LoadOrder,
    state: &GameState,
    call: &DpsCall,
    style: &CombatStyle,
    anims: Option<&mut dyn Library>,
    s: Setting,
) -> f32 {
    let weapon = call.weapon;
    if weapon.is_some_and(|w| w.flags2 & 0x40 != 0) {
        return 0.0;
    }
    let call = DpsCall {
        perks: false,
        ..*call
    };
    let mut d = weapon_dps(order, state, &call, anims, s);
    let k = weapon.map_or(npc_combat::kind::HAND_TO_HAND, |w| {
        npc_combat::combat_weapon_type(order, w, s)
    });
    if k == npc_combat::kind::GRENADE {
        d /= 10.0;
    } else if k == npc_combat::kind::MINE {
        d = 0.0;
    }
    npc_combat::restricted(d, weapon.map(npc_combat::is_gun), style.weapon_restrictions)
}

/// The ammunition `who` would fire `w` with
/// (`InventoryChanges::GetBestAmmoForWeapon` (Xbox PDB)): whether it takes
/// any, and the first kind it takes that they carry. (A list's kinds added
/// by script are looked at after the rest, `004c7300`; none are here.)
///
/// Translated from 004c7300 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn best_ammo(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    w: &Weapon,
) -> (bool, Option<FormId>) {
    let found = w
        .ammo
        .iter()
        .copied()
        .find(|&a| state.item_count(order, who, a) > 0);
    (!w.ammo.is_empty(), found)
}

/// Whether `who` may pick `w` to hold (`Actor::CanUseWeapon` (Xbox PDB),
/// asked as `GetBestWeapon` asks, playable or not): never a "player only"
/// weapon (flags2 0x1); a person any other; a creature only one in its
/// record's weapon list (`CREA` `LNAM`, a form list: ED-E's
/// `EmbeddedWeapons`).
///
/// Translated from 008bc9d0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn can_use_weapon(order: &LoadOrder, who: FormId, w: &Weapon) -> bool {
    if w.flags2 & 0x1 != 0 {
        return false;
    }
    if !combat::is_creature(order, who) {
        return true;
    }
    creature_weapon_list(order, who).contains(&w.form_id)
}

/// A creature's weapon list (`CREA` `LNAM`, the base's +0x158 set by
/// `005f8bd0`): the form list's entries.
pub fn creature_weapon_list(order: &LoadOrder, who: FormId) -> Vec<FormId> {
    let Some(base) = crate::scripting::base_of(order, who) else {
        return Vec::new();
    };
    let list = order
        .get(base)
        .and_then(|rr| {
            let record = rr.record().ok()?;
            let d = record.get(FourCC::new(b"LNAM"))?;
            (d.data.len() >= 4)
                .then(|| rr.plugin.to_global(FormId(crate::cell::le_u32(&d.data, 0))))
        })
        .filter(|f| f.0 != 0);
    list.map_or_else(Vec::new, |l| perks::form_list(order, l))
}

/// Whether the game would have an inventory entry for `item` of `who`'s
/// (an `ExtraContainerChanges` entry; `GetBestWeapon` passes it to the
/// figure, which then counts mods and reads the rate off the animations):
/// something changed it — worn, worn down, modded, or a count other than
/// the record gives (given, taken, picked from a leveled list).
pub fn has_entry(order: &LoadOrder, state: &GameState, who: FormId, item: FormId) -> bool {
    has_extra(state, who, item)
        || state.item_count(order, who, item)
            != crate::scripting::base_contents(order, who)
                .iter()
                .filter(|(i, _)| *i == item)
                .map(|(_, n)| n)
                .sum::<i32>()
}

/// Whether the entry has an extra data list (worn, its health or mods
/// recorded): `GetBestWeapon` reads the condition off it.
fn has_extra(state: &GameState, who: FormId, item: FormId) -> bool {
    state.is_equipped(who, item)
        || state.weapon_health.contains_key(&(who, item))
        || state.weapon_mods.contains_key(&(who, item))
}

/// The weapon `who` would pick to hold (`InventoryChanges::GetBestWeapon`
/// (Xbox PDB), asked for any kind (6) or one combat kind): with
/// `keep_locked`, a weapon in hand that a script locked there
/// (`EquipItem`'s no-unequip flag) is kept. Otherwise every weapon they
/// carry — the record's own first, in its order, then the rest — that is
/// of the kind, that they may use ([`can_use_weapon`]) and that has
/// ammunition with it or needs none ([`best_ammo`]), is rated by
/// [`combat_weapon_dps`] (their combat style; with the ammunition found,
/// and an inventory entry when it has one, [`has_entry`]), at its
/// condition — one with no health left is passed over; one with no extra
/// data is rated at the condition of the last one that had it (1 at
/// first), as the game's loop leaves it — and the highest rating above 0
/// wins, the first on a tie. (The game also returns nothing while the
/// actor is planting an explosive, process +0x444; not modelled.)
///
/// Translated from 004c7400 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn best_weapon(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    kind: Option<usize>,
    keep_locked: bool,
    mut anims: Option<&mut dyn Library>,
    s: Setting,
) -> Option<FormId> {
    let held = equipped_weapon(order, state, who);
    if keep_locked {
        if let Some(w) = held.filter(|&w| state.equip_locked.contains(&(who, w))) {
            return Some(w);
        }
    }
    let inventory = state.inventory(order, who);
    let carried = |item: FormId| {
        inventory
            .iter()
            .find(|(i, _)| *i == item)
            .is_some_and(|(_, n)| *n > 0)
    };
    // The base container's entries first, then the changes' (`004c7400`'s
    // two loops).
    let mut items: Vec<FormId> = Vec::new();
    for (item, _) in crate::scripting::base_contents(order, who) {
        if carried(item) && !items.contains(&item) {
            items.push(item);
        }
    }
    for (item, n) in &inventory {
        if *n > 0 && !items.contains(item) {
            items.push(*item);
        }
    }
    let style = crate::more_functions::combat_style(order, state, who);
    let mut best: Option<(f32, FormId)> = None;
    let mut condition = 1.0f32;
    for item in items {
        let Some(w) = Weapon::load(order, item) else {
            continue;
        };
        if kind.is_some_and(|k| npc_combat::combat_weapon_type(order, &w, s) != k) {
            continue;
        }
        let (needs, ammo) = best_ammo(order, state, who, &w);
        if !can_use_weapon(order, who, &w) || (needs && ammo.is_none()) {
            continue;
        }
        if has_extra(state, who, item) {
            condition = combat::weapon_condition(state, who, item);
            if condition <= 0.0 {
                continue;
            }
        }
        let call = DpsCall {
            who,
            weapon: Some(&w),
            condition,
            perks: false,
            entry: has_entry(order, state, who, item),
            ammo,
        };
        let lib: Option<&mut dyn Library> = match anims {
            Some(ref mut a) => Some(&mut **a),
            None => None,
        };
        let d = combat_weapon_dps(order, state, &call, &style, lib, s);
        if best.map_or(0.0, |(b, _)| b) < d {
            best = Some((d, item));
        }
    }
    best.map(|(_, w)| w)
}

/// The weapon someone has equipped.
pub fn equipped_weapon(order: &LoadOrder, state: &GameState, who: FormId) -> Option<FormId> {
    state.equipped.get(&who).and_then(|worn| {
        worn.iter().copied().find(|&i| {
            order
                .get(i)
                .is_some_and(|r| r.entry.header.kind.as_bytes() == b"WEAP")
        })
    })
}
