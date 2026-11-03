//! Fighting: weapons as their records describe them, the damage a hit does
//! by the game's settings, health, and dying.
//!
//! Weapons (`WEAP`, checked on the 9mm pistol): `DATA` value i32, health
//! i32, weight f32, damage i16, clip size u8 (100, 150, 1.5, 16, 13);
//! `NAM0` the ammunition; `DNAM` (204 bytes): animation type u32 at 0 (0
//! hand to hand, 1 one-handed melee, 2 two-handed melee, 3 one-handed
//! pistol …), ammo use u8 at 14, min spread f32 at 16, spread at 20,
//! projectile at 36, projectile count u8 at 42, min and max range at 44 and
//! 48, attack shots per second at 88 (3.125), reload time at 92 (1.67 s),
//! the skill (actor value i32) at 104 (41, Guns); `CRDT` critical damage
//! u16, critical chance multiplier f32.
//!
//! A hit, read from the game's code (`%USERPROFILE%\nv-re\findings\
//! hits.md`): the weapon's damage ([`weapon_damage`]: skill, arms,
//! condition curve, melee/unarmed bonus, scale), a critical adding the
//! weapon's critical damage ([`critical`]), the armour ([`through_armour`]:
//! resistance, then threshold, at least 20%, then the ammunition's damage
//! effects), the body part it lands on (`world::body_parts`: limb damage,
//! the part's multiplier — headshots ×2 for shots), then a sneak attack's
//! ×2 / ×5 ([`sneak_multiplier`]) when a part was found. Weapons are in
//! full condition unless scripts change it (no wear yet). Blocking adds
//! to the threshold ([`hit_through_armour`], `world::melee`); power
//! attacks multiply the weapon's damage ([`weapon_damage`]). Not here yet:
//! difficulty (Normal, ×1).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::dialogue::PLAYER_REF;
use crate::perks::{self, Tab};
use crate::scripting::{base_of, game_setting, Event, Facts, GameState};

const WEAP: FourCC = FourCC::new(b"WEAP");
const CREA: FourCC = FourCC::new(b"CREA");
const DNAM: FourCC = FourCC::new(b"DNAM");
const NAM0: FourCC = FourCC::new(b"NAM0");
const CRDT: FourCC = FourCC::new(b"CRDT");
const SNAM: FourCC = FourCC::new(b"SNAM");

/// Actor value numbers used here.
pub mod av {
    pub const STRENGTH: u16 = 5;
    pub const ENDURANCE: u16 = 7;
    pub const HEALTH: u16 = 16;
    pub const MELEE_WEAPONS: u16 = 38;
    pub const UNARMED: u16 = 45;
    pub const DAMAGE_THRESHOLD: u16 = 76;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Weapon {
    pub form_id: FormId,
    pub name: String,
    pub damage: f32,
    pub clip: u32,
    pub health: i32,
    /// `DNAM` animation type: 0 hand to hand, 1–2 melee, 3+ guns and the
    /// rest.
    pub animation: u32,
    /// The ammunition it takes: `NAM0` names a form list (`FLST`, its
    /// `LNAM`s: `AmmoList556mm` holds five kinds of 5.56mm rounds) or one
    /// `AMMO`.
    pub ammo: Vec<FormId>,
    pub ammo_use: u8,
    pub min_spread: f32,
    pub spread: f32,
    pub projectile: Option<FormId>,
    pub projectiles: u8,
    pub min_range: f32,
    pub max_range: f32,
    pub shots_per_second: f32,
    pub reload_time: f32,
    pub skill: u16,
    pub crit_damage: f32,
    pub crit_mult: f32,
    /// The attack sound (first `SNAM`).
    pub sound: Option<FormId>,
    /// `DNAM` u8 at 41 and 15: which attack and reload animation it plays
    /// (see `world::actor::first_person_attack` and `…_reload`).
    pub attack_animation: u8,
    pub reload_animation: u8,
    /// `DNAM` f32 at 180 and 196: how hard a killing hit throws the body
    /// (Havok units a second ÷ 2.5) and, for guns, the distance past which
    /// it throws only a tenth as hard (game units).
    pub kill_impulse: f32,
    pub impulse_distance: f32,
    /// `DNAM` f32 at 8: melee reach (× 128 units: the machete's 0.5 = 64).
    pub reach: f32,
    /// `DNAM` f32 at 116: how much of a hit's limb damage it does (the
    /// game's weapon `+0x168`, read by `009b6620`).
    pub limb_damage_mult: f32,
    /// `DNAM` u8 at 12 and u32 at 56: the two flag sets (0x02 in the first
    /// automatic; 0x20 in the second "range fixed", 0x200 short burst).
    pub flags1: u8,
    pub flags2: u32,
    /// `DNAM` f32 at 64: the fire rate, shots a second for automatic
    /// weapons (the 9mm submachine gun's 11).
    pub fire_rate: f32,
    /// `DNAM` f32 at 60: the animation attack multiplier (the machete's
    /// 1.3).
    pub attack_mult: f32,
    /// `DNAM` f32 at 100: the aim arc, degrees (0 on the vanilla guns).
    pub aim_arc: f32,
    /// `DNAM` f32 at 128 and 132: the semi-automatic fire delay, seconds
    /// (0 and 0.3 on the 9mm pistol).
    pub semi_auto_delay: (f32, f32),
    /// `DNAM` f32 at 4: the weapon's speed, the rate its attack
    /// animations start from (1 on the vanilla weapons; the game's weapon
    /// `+0xf8`, read by `004e4620`).
    pub speed: f32,
    /// What its shots' cone is multiplied by: 1, or a fitted split beam
    /// mod's second value (`00523150`; `world::weapon_mods::modded`).
    pub cone_mult: f32,
    /// `CRDT` u32 at 12: the critical effect, a spell cast on whoever a
    /// critical hit strikes (the weapon's `+0x1cc`, `0051f4d0`): the laser
    /// weapons' `LaserDisintegrationFXSpell`, the plasma ones'
    /// `PlasmaEffect` (see `docs/ENERGY_WEAPONS.md`).
    pub crit_effect: Option<FormId>,
    /// `CRDT` u8 at 8, flag 0x01 "on death" (the weapon's `+0x1c8`,
    /// `009b73b0`): the effect only when the critical hit kills.
    pub crit_on_death: bool,
    /// `DNAM` i32 at 120: the actor value that resists its damage (the
    /// weapon's `+0x16c`, `009b6600`; -1 none): 60 Energy Resistance on
    /// the lasers and plasma weapons, 61 EMP on the pulse gun, 52 Fire on
    /// the flamers.
    pub resist: Option<u16>,
}

impl Weapon {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Weapon> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == WEAP)?;
        let record = rr.record_shared().ok()?;
        let data = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 15)?;
        let d = &data.data;
        let dnam = record.get(DNAM).map(|s| s.data.as_slice()).unwrap_or(&[]);
        let f = |at: usize| (dnam.len() >= at + 4).then(|| le_f32(dnam, at));
        let u = |at: usize| (dnam.len() >= at + 4).then(|| le_u32(dnam, at));
        let form = |raw: u32| Some(rr.plugin.to_global(FormId(raw))).filter(|f| f.0 != 0);
        let crdt = record.get(CRDT).map(|s| s.data.as_slice()).unwrap_or(&[]);
        Some(Weapon {
            form_id: id,
            name: record.full_name().unwrap_or_default(),
            damage: f32::from(i16::from_le_bytes([d[12], d[13]])),
            clip: u32::from(d[14]),
            health: le_u32(d, 4) as i32,
            animation: u(0).unwrap_or(0),
            ammo: record
                .get(NAM0)
                .filter(|s| s.data.len() >= 4)
                .and_then(|s| form(le_u32(&s.data, 0)))
                .map(|a| ammo_kinds(order, a))
                .unwrap_or_default(),
            ammo_use: dnam.get(14).copied().unwrap_or(1),
            min_spread: f(16).unwrap_or(0.0),
            spread: f(20).unwrap_or(0.0),
            projectile: u(36).and_then(form),
            projectiles: dnam.get(42).copied().unwrap_or(1).max(1),
            min_range: f(44).unwrap_or(0.0),
            max_range: f(48).unwrap_or(0.0),
            shots_per_second: f(88).unwrap_or(1.0),
            reload_time: f(92).unwrap_or(1.0),
            skill: u(104).map_or(41, |v| v as u16),
            crit_damage: if crdt.len() >= 2 {
                f32::from(u16::from_le_bytes([crdt[0], crdt[1]]))
            } else {
                0.0
            },
            crit_mult: if crdt.len() >= 8 {
                le_f32(crdt, 4)
            } else {
                1.0
            },
            sound: record
                .get(SNAM)
                .filter(|s| s.data.len() >= 4)
                .and_then(|s| form(le_u32(&s.data, 0))),
            attack_animation: dnam.get(41).copied().unwrap_or(255),
            reload_animation: dnam.get(15).copied().unwrap_or(0),
            kill_impulse: f(180).unwrap_or(0.0),
            impulse_distance: f(196).unwrap_or(0.0),
            reach: f(8).unwrap_or(0.0),
            limb_damage_mult: f(116).unwrap_or(1.0),
            flags1: dnam.get(12).copied().unwrap_or(0),
            flags2: u(56).unwrap_or(0),
            fire_rate: f(64).unwrap_or(0.0),
            attack_mult: f(60).unwrap_or(1.0),
            aim_arc: f(100).unwrap_or(0.0),
            semi_auto_delay: (f(128).unwrap_or(0.0), f(132).unwrap_or(0.0)),
            speed: f(4).unwrap_or(1.0),
            cone_mult: 1.0,
            crit_effect: (crdt.len() >= 16).then(|| le_u32(crdt, 12)).and_then(form),
            crit_on_death: crdt.get(8).is_some_and(|f| f & 0x01 != 0),
            resist: u(120)
                .map(|v| v as i32)
                .filter(|&v| v >= 0)
                .map(|v| v as u16),
        })
    }

    /// Whether it fires in bursts (`009d0a30`): automatic (`DNAM` flag
    /// 0x02 in the first set) or "short burst" (0x200 in the second).
    pub fn is_automatic(&self) -> bool {
        self.flags1 & 0x02 != 0 || self.flags2 & 0x200 != 0
    }

    /// How long one attack lasts, for the combat AI's pacing (inferred,
    /// `findings\combat_ai.md` §5): a gun's 1 ÷ its attack shots a second
    /// (`DNAM` 88, the editor's figure for its attack); a melee weapon's
    /// attack animation (`animation`, seconds) sped up by its attack
    /// multiplier (`DNAM` 60), as the field's name says.
    pub fn attack_seconds(&self, animation: f32) -> f32 {
        if self.is_melee() {
            animation
                / if self.attack_mult > 0.0 {
                    self.attack_mult
                } else {
                    1.0
                }
        } else {
            self.shot_interval()
        }
    }

    /// Melee weapons and fists (animation types 0–2).
    pub fn is_melee(&self) -> bool {
        self.animation <= 2
    }

    /// Held in both hands (`00646cb0`): animation types 2 (two-handed
    /// melee), 5, 6, 8 and 9 (rifles, automatics, handles, launchers); the
    /// code leaves out 7, energy rifles.
    pub fn two_handed(&self) -> bool {
        matches!(self.animation, 2 | 5 | 6 | 8 | 9)
    }

    /// The ammunition a holder would load: the kind the Ammo Swap control
    /// loaded (`GameState::ammo_loaded`, the process's current ammo,
    /// `00525980`) while it's in the list and carried; else the first kind
    /// it takes that they carry, else the first kind (which kind the game
    /// picks when several are carried and none was chosen isn't traced).
    pub fn ammo_in_use(
        &self,
        order: &LoadOrder,
        state: &GameState,
        holder: FormId,
    ) -> Option<FormId> {
        if let Some(&chosen) = state.ammo_loaded.get(&holder) {
            if self.ammo.contains(&chosen) && state.item_count(order, holder, chosen) > 0 {
                return Some(chosen);
            }
        }
        self.ammo
            .iter()
            .copied()
            .find(|&a| state.item_count(order, holder, a) > 0)
            .or_else(|| self.ammo.first().copied())
    }

    /// Seconds between shots (or swings).
    pub fn shot_interval(&self) -> f32 {
        1.0 / self.shots_per_second.max(0.1)
    }

    /// How far a melee attack reaches between the bodies' edges
    /// (`CombatUtilities::GetWeaponReach`, Xbox PDB, `009a69c0`): a melee
    /// weapon's (animation types 0–2, hand-to-hand ones too: the brass
    /// knuckles' 1 gives 128) reach × `fCombatDistance` (128), a gun 128,
    /// no weapon a person's `fHandReachMult` (0.5) × 128 = 64 (× the
    /// attacker's scale, left to the caller).
    // Translated from 009a69c0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn melee_reach(weapon: Option<&Weapon>) -> f32 {
        match weapon {
            Some(w) if w.is_melee() => w.reach * 128.0,
            Some(_) => 128.0,
            None => 64.0,
        }
    }

    /// A shot's projectiles and the cone they fly in (radians), as the
    /// game fires them (`00523150`): the weapon's projectile count (the
    /// ammunition's `DAT2` count × ammo use when it gives one), each
    /// carrying the weapon's damage ÷ the count; the cone the weapon's min
    /// spread (degrees) after the ammunition's spread effects — the
    /// spread × wobble term is always 0 in the game, and the player's sway
    /// doesn't move unscoped shots; all of it × [`Weapon::cone_mult`].
    pub fn shot(&self, order: &LoadOrder, ammo: Option<FormId>) -> (u32, f32) {
        let from_ammo = ammo
            .and_then(|a| order.get(a))
            .and_then(|r| r.record().ok())
            .and_then(|r| r.get(FourCC::new(b"DAT2")).map(|s| s.data.clone()))
            .filter(|d| d.len() >= 4)
            .map_or(0, |d| le_u32(&d, 0));
        let count = if from_ammo > 0 {
            from_ammo * u32::from(self.ammo_use.max(1))
        } else {
            u32::from(self.projectiles.max(1))
        };
        let effects = ammo.map(|a| ammo_effects(order, a)).unwrap_or_default();
        let cone = with_ammo(&effects, 3, self.min_spread)
            .max(0.0)
            .to_radians()
            * self.cone_mult;
        (count, cone)
    }

    /// How far its shots go: its projectile's range (`PROJ` `DATA` f32 at
    /// 12: the 9mm bullet's 10000).
    pub fn range(&self, order: &LoadOrder) -> Option<f32> {
        let d = order
            .get(self.projectile?)?
            .record()
            .ok()?
            .get(esm::sig::DATA)
            .filter(|s| s.data.len() >= 16)?
            .data
            .clone();
        Some(le_f32(&d, 12)).filter(|r| *r > 0.0)
    }
}

/// The kinds of ammunition a weapon's `NAM0` stands for: a form list's
/// entries, or the one.
fn ammo_kinds(order: &LoadOrder, id: FormId) -> Vec<FormId> {
    let Some(rr) = order.get(id) else {
        return vec![id];
    };
    if rr.entry.header.kind.as_bytes() != b"FLST" {
        return vec![id];
    }
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .get_all(FourCC::new(b"LNAM"))
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .collect()
}

/// What a hit on a person or creature did (`world::scripting::Runner::
/// hit_at`).
#[derive(Debug, Clone, PartialEq)]
pub struct Hit {
    /// The health it took.
    pub dealt: f32,
    /// The body part it landed on (`world::body_parts::part`), if any.
    pub part: Option<u8>,
    /// Whether it was critical.
    pub critical: bool,
    /// The multiplier it took last (0: no part found, nothing multiplied).
    pub multiplier: f32,
    /// What it did to the part.
    pub hurt: Option<crate::body_parts::PartHurt>,
    /// The attacker's "Knockdown Chance" perk knocked them down
    /// (`world::melee::knocks_down`).
    pub knocked_down: bool,
    /// The fatigue damage it did (`world::fatigue`): bare fists' half of
    /// their damage, a bean bag's; 0 when the target's fatigue was already
    /// at `fMinimumFatigue` or below.
    pub fatigue: f32,
}

/// The damage one of the attacker's hits with a weapon (or fists, `None`)
/// does before the target's armour (`00644ce0`, read from the game's
/// code; `findings\hits.md` §3): ((base × S × power) + A) × C × the
/// attacker's scale, where base is the weapon's damage (fists 1); S =
/// `fDamageSkillBase` 0.5 + `fDamageSkillMult` 0.5 × skill / 100 × R for
/// guns and melee weapons, R alone for hand-to-hand weapons and fists, R
/// (melee and unarmed only) = 0.2 + 0.8 × the arms' share intact; A = the
/// Melee Damage actor value (17) for melee weapons, Unarmed Damage
/// (`fAVDUnarmedDamageBase` 0.5 + `…Mult` 0.05 × Unarmed) for
/// hand-to-hand and fists; C = 1 above 75% condition, else 1 − 0.67 ×
/// (0.75 − condition) (the `fDamage…WeapCond…` settings aren't used for
/// this). `power` is `fDamagePowerAttackBonus` (2) for a power attack
/// with a weapon (hand-to-hand ones too); fists' damage is worked out
/// with 1 there (`00646310`) and a power attack's bonus comes after the
/// armour instead ([`fists_power_bonus`]). A player's teammate's damage
/// is also × their Nerve ([`crate::companions::nerve`]).
pub fn weapon_damage(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    weapon: Option<&Weapon>,
    power: bool,
) -> f32 {
    let condition = weapon.map_or(1.0, |w| weapon_condition(state, attacker, w.form_id));
    weapon_damage_at(order, state, attacker, weapon, power, condition)
}

/// [`weapon_damage`] with the weapon at a condition (0..1) other than its
/// own (the repair menus' "after" figures, `006450f0` with a condition).
pub fn weapon_damage_at(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    weapon: Option<&Weapon>,
    power: bool,
    condition: f32,
) -> f32 {
    let setting = |n: &str, default: f32| game_setting(order, n).unwrap_or(default);
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let av = |a: u16| facts.current_actor_value(attacker, a).unwrap_or(0.0) as f32;
    let animation = weapon.map_or(0, |w| w.animation);
    let melee = weapon.map_or(true, |w| w.is_melee());
    // Arms intact: the average of the two for most melee kinds, the right
    // alone for the rest; guns don't care.
    let arms = if melee {
        let (left, right) = (av(27) > 0.0, av(28) > 0.0);
        let share = if matches!(animation, 0 | 2 | 5..=9) {
            (f32::from(u8::from(left)) + f32::from(u8::from(right))) / 2.0
        } else {
            f32::from(u8::from(right))
        };
        setting("fDamageArmConditionBase", 0.2) + setting("fDamageArmConditionMult", 0.8) * share
    } else {
        1.0
    };
    let skill_factor = match weapon {
        Some(w) if w.animation != 0 => {
            setting("fDamageSkillBase", 0.5)
                + setting("fDamageSkillMult", 0.5) * av(w.skill) / 100.0 * arms
        }
        _ => arms,
    };
    let unarmed = || {
        setting("fAVDUnarmedDamageBase", 0.5)
            + setting("fAVDUnarmedDamageMult", 0.05) * av(av::UNARMED)
    };
    let added = match weapon {
        None => unarmed(),
        Some(w) if w.animation == 0 => unarmed(),
        Some(w) if matches!(w.animation, 1 | 2 | 13) => av(17).max(0.0),
        Some(_) => 0.0,
    } + weapon
        .and_then(|w| {
            // A damage mod's value (`004bd8d0` in `00644ce0`).
            crate::weapon_mods::bonus(
                order,
                crate::weapon_mods::flags(state, attacker, w.form_id),
                w.form_id,
                crate::weapon_mods::effect::DAMAGE,
            )
        })
        .unwrap_or(0.0);
    let power = if power && weapon.is_some() {
        setting("fDamagePowerAttackBonus", 2.0)
    } else {
        1.0
    };
    let base = weapon.map_or(1.0, |w| w.damage) * setting("fDamageWeaponMult", 1.0);
    let condition = if weapon.is_none() || condition > 0.75 {
        1.0
    } else {
        1.0 - 0.67 * (0.75 - condition)
    };
    let scale = state.scales.get(&attacker).copied().unwrap_or(1.0);
    // A teammate's Nerve (`00644ce0`'s last factor).
    let nerve = crate::companions::nerve(order, state, attacker);
    (base * skill_factor * power + added) * condition * scale * nerve
}

/// A fists' power attack's bonus (`009b5170`): with no weapon, a person's
/// (not a creature's) power attack doesn't change the damage
/// (`00646310` asks `00644ce0` with 1) but sets the hit's bonus
/// multiplier (`fBonusMult`, `+0x5c`, `009b7840`: the larger of it and
/// what's there) to `fDamagePowerAttackBonus` (exe 3, data 2); the body
/// part's multiplier may raise it (`009b6620`), and it multiplies the
/// damage after the armour (`009b73d0`). 0: none.
// Translated from 009b5170 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn fists_power_bonus(order: &LoadOrder, power: bool) -> f32 {
    if power {
        game_setting(order, "fDamagePowerAttackBonus").unwrap_or(3.0)
    } else {
        0.0
    }
}

/// [`weapon_damage`] for a weapon, without a power attack.
pub fn hit_damage(order: &LoadOrder, state: &GameState, attacker: FormId, weapon: &Weapon) -> f32 {
    weapon_damage(order, state, attacker, Some(weapon), false)
}

/// Whether a hit is critical (`009b7060`): chance % = Crit Chance
/// (`fAVDCritLuckBase` 0 + `fAVDCritLuckMult` 1 × Luck) × the weapon's
/// critical multiplier (`CRDT`; fists 1), then the attacker's perks'
/// "Calculate My Critical Hit Chance" (entry point 1: Ninja × 1.15 with
/// melee and unarmed weapons, Laser Commander + 10) and the target's
/// "Modify Enemy Critical Hit Chance" (36: Dream Crusher × 0.5), both
/// asked about the weapon (the fists when none), plus
/// `fVATSCriticalChanceBonus` (5) points for the player's queued V.A.T.S.
/// hits (`world::vats::critical_bonus`), rolled per mille; a sneak attack
/// (the player sneaking, unseen by the target: its detection of them
/// below 1) is certain (× `fCombatSneakAttackBonusMult` 100). `roll` is a
/// number 0..1000.
pub fn critical(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    weapon: Option<&Weapon>,
    target: FormId,
    sneak_attack: bool,
    roll: u64,
) -> bool {
    let setting = |n: &str, default: f32| game_setting(order, n).unwrap_or(default);
    let luck = Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(attacker, 11)
    .unwrap_or(0.0) as f32;
    let mut chance = setting("fAVDCritLuckBase", 0.0) + setting("fAVDCritLuckMult", 1.0) * luck;
    chance /= critical_divisor(weapon);
    if let Some(m) = weapon.map(|w| w.crit_mult).filter(|m| *m >= 0.0) {
        chance *= m;
    }
    let held = perks::weapon_tab(weapon.map(|w| w.form_id));
    chance = perks::apply_for(
        order,
        state,
        attacker,
        perks::entry::CALCULATE_MY_CRITICAL_HIT_CHANCE,
        chance,
        &[held, Tab::Target(target)],
    );
    chance = perks::apply_for(
        order,
        state,
        target,
        perks::entry::MODIFY_ENEMY_CRITICAL_HIT_CHANCE,
        chance,
        &[held, Tab::Target(attacker)],
    );
    chance += crate::vats::critical_bonus(order, state, attacker);
    if sneak_attack {
        chance *= setting("fCombatSneakAttackBonusMult", 100.0);
    }
    (roll % 1000) < (chance * 10.0) as u64
}

/// What a hit's critical chance is divided by (`00646d80`, the chance
/// `009b7060` starts from): an automatic weapon's (`DNAM` flags 0x02,
/// `00524b40`) fire rate (`DNAM` f32 at 64, `00821640`), 1 when that's 0
/// and for anything else: the Gatling laser's 30 shots a second, the
/// laser RCW's 9, share what one shot of a semi-automatic would have.
// Translated from 00646d80 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn critical_divisor(weapon: Option<&Weapon>) -> f32 {
    match weapon {
        Some(w) if w.flags1 & 0x02 != 0 && w.fire_rate != 0.0 => w.fire_rate,
        _ => 1.0,
    }
}

/// `BannedEffectsOnSitters` (`FLST` 001768D7 in `FalloutNV.esm`, looked
/// up by its ID by `0089a760`): the goo and the two disintegrations.
pub const BANNED_EFFECTS_ON_SITTERS: FormId = FormId(0x0017_68D7);

/// A critical hit's effect after its damage (`0089a760`, at
/// `0089b295`–`0089b43a`): the weapon's critical effect (`CRDT`, carried
/// by the hit from `009b7060`; fists have none) is cast on the target,
/// by the target's own caster (`+0x88`, `00815b00`, `00824110`), when
/// the target was alive before the hit, and
///
/// - not when the effect is "on death" and the target is essential
///   (`0087f3d0`) or the hit didn't kill (`DamageHealthAndFatigue`,
///   vtable `+0x338`, returns `IsDead` after the damage);
/// - not when the target is in a sit or sleep state (`GetSitSleepState`,
///   vtable `+0x214`, not 0) and `BannedEffectsOnSitters` holds it;
/// - not with `bDisableAllGore` (INI, `011df7f8`, default 0; not read
///   here).
///
/// The laser and plasma weapons' effects are script effects whose magic
/// effects aren't dispelled by death (`MGEF` flag 0x10000000,
/// `world::magic::tick`): they disintegrate or goo the corpse
/// (`SetCriticalStage`, `AttachAshPile`). What was cast, if anything.
// Translated from 0089a760 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn critical_effect(
    order: &LoadOrder,
    state: &mut GameState,
    target: FormId,
    weapon: Option<&Weapon>,
    was_alive: bool,
    dead_now: bool,
) -> Option<FormId> {
    let w = weapon?;
    let effect = w.crit_effect?;
    if !was_alive {
        return None;
    }
    let essential = crate::more_functions::is_essential(order, state, target);
    if (essential || !dead_now) && w.crit_on_death {
        return None;
    }
    if sit_sleep_state(state, target) != 0
        && crate::script_functions::form_list(order, state, BANNED_EFFECTS_ON_SITTERS)
            .contains(&effect)
    {
        return None;
    }
    crate::magic::add_spell(order, state, target, effect, target, false);
    Some(effect)
}

/// Someone's sit or sleep state as the game keeps it (`GetSitSleepState`,
/// the value `GetSitting` and `GetSleeping` map, `world::furniture`): 0
/// when neither; someone placed in furniture without a state yet counts
/// as seated (4).
pub fn sit_sleep_state(state: &GameState, who: FormId) -> u8 {
    match state.sitters.get(&who) {
        Some(sitter) => sitter.state.number(),
        None if state.furniture.contains_key(&who) => 4,
        None => 0,
    }
}

/// What a critical hit adds to its damage (`009b7060`): the weapon's
/// critical damage (`CRDT`; fists: the hit × the setting
/// `fCombatUnarmedCritDamageMult` 1) after the attacker's perks' "Calculate
/// My Critical Hit Damage" (entry point 2: Better Criticals × 1.5, Hunter ×
/// 1.75 against animals, asked about the weapon and the target); nothing
/// when that's 0 or less.
pub fn critical_damage(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    weapon: Option<&Weapon>,
    target: FormId,
    hit: f32,
) -> f32 {
    let base = match weapon {
        Some(w) => w.crit_damage,
        None => hit * game_setting(order, "fCombatUnarmedCritDamageMult").unwrap_or(1.0),
    };
    let extra = perks::apply_for(
        order,
        state,
        attacker,
        perks::entry::CALCULATE_MY_CRITICAL_HIT_DAMAGE,
        base,
        &[
            perks::weapon_tab(weapon.map(|w| w.form_id)),
            Tab::Target(target),
        ],
    );
    extra.max(0.0)
}

/// An ammunition's effects (`RCIL` → `AMEF` `DATA`: kind u32 — 0 damage,
/// 1 damage resistance, 2 damage threshold, 3 spread, 4 weapon wear, 5
/// fatigue — operation u32 — 0 add, 1 multiply, 2 subtract — and the
/// value f32). The 5.56mm hollow point: damage × 1.75, the target's DT ×
/// 3.
pub fn ammo_effects(order: &LoadOrder, ammo: FormId) -> Vec<(u32, u32, f32)> {
    let Some(rr) = order.get(ammo) else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .get_all(FourCC::new(b"RCIL"))
        .filter(|s| s.data.len() >= 4)
        .filter_map(|s| {
            let effect = rr.plugin.to_global(FormId(le_u32(&s.data, 0)));
            let d = order
                .get(effect)
                .filter(|r| r.entry.header.kind.as_bytes() == b"AMEF")?
                .record()
                .ok()?
                .get(esm::sig::DATA)
                .filter(|s| s.data.len() >= 12)?
                .data
                .clone();
            Some((le_u32(&d, 0), le_u32(&d, 4), le_f32(&d, 8)))
        })
        .collect()
}

/// A value after the ammunition's effects of one kind, in their order.
pub fn with_ammo(effects: &[(u32, u32, f32)], kind: u32, value: f32) -> f32 {
    effects
        .iter()
        .filter(|e| e.0 == kind)
        .fold(value, |v, &(_, op, x)| match op {
            0 => v + x,
            1 => v * x,
            2 => v - x,
            _ => v,
        })
}

/// A hit's damage through the target's armour (`009b5a30`): at least
/// `fMinDamMultiplier` (0.2) of it stays; damage resistance takes its
/// share first (Damage Resistance, actor value 18, plus worn armour's,
/// after the ammunition's resistance effects, out of 100, at most
/// `fMaxArmorRating` 85%), then the damage threshold is taken off (its
/// actor value and worn armour's, after the ammunition's threshold
/// effects, then the perks': see [`hit_through_armour`]), then the
/// ammunition's damage effects (hollow points × 1.75: after the
/// threshold, as the game does it). With no attacker: nobody's perks. A
/// player's teammate's resistance and threshold are × their Nerve
/// ([`crate::companions::nerve`]).
pub fn through_armour(
    order: &LoadOrder,
    state: &GameState,
    damage: f32,
    target: FormId,
    ammo: Option<FormId>,
) -> f32 {
    hit_through_armour(order, state, damage, None, target, ammo)
}

/// [`through_armour`] for `attacker`'s hit with a weapon (`None`: the
/// fists), with both sides' perks as `009b5a30` applies them: the
/// threshold, after the ammunition's threshold effects, less the
/// attacker's "Modify Damage Threshold (attacker)" (entry point 58:
/// Piercing Strike 15 with melee and unarmed weapons, Shotgun Surgeon 10
/// with shotguns; asked about the weapon and the target) plus the target's
/// "Modify Damage Threshold (defender)" (56: Toughness + 3, Stonewall + 5
/// against melee; asked about the attacker and the attacker's weapon),
/// never below 0; then, after the ammunition's damage effects, the
/// attacker's "Calculate Weapon Damage" (0: Cowboy × 1.25 with its
/// weapons, Lady Killer × 1.1 against women, Bloody Mess × 1.05) on what's
/// left, before the 20% floor.
pub fn hit_through_armour(
    order: &LoadOrder,
    state: &GameState,
    damage: f32,
    attacker: Option<(FormId, Option<FormId>)>,
    target: FormId,
    ammo: Option<FormId>,
) -> f32 {
    armour_hit(order, state, damage, attacker, target, ammo).damage
}

/// A hit through armour: the damage left, and what it wears off the
/// target's armour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArmourHit {
    pub damage: f32,
    /// The hit's `fArmorDamage` (Xbox PDB `HitData`, `009b5a30`):
    /// `fDamageToArmorPercentage` (0.5) × the damage × the resistance's
    /// share, plus `fDamageToArmorPercentage` × the threshold taken off (at
    /// most what's left above the 20% floor); none for a hit doing fatigue
    /// damage (none here). See [`wear_armour`].
    pub armour_damage: f32,
}

/// [`hit_through_armour`] with what the hit wears off the armour.
pub fn armour_hit(
    order: &LoadOrder,
    state: &GameState,
    damage: f32,
    attacker: Option<(FormId, Option<FormId>)>,
    target: FormId,
    ammo: Option<FormId>,
) -> ArmourHit {
    let setting = |n: &str, default: f32| game_setting(order, n).unwrap_or(default);
    let effects = ammo.map(|a| ammo_effects(order, a)).unwrap_or_default();
    let least = damage * setting("fMinDamMultiplier", 0.2);
    // A teammate's Nerve raises both, before the ammunition's effects
    // (`009b5a30`).
    let nerve = crate::companions::nerve(order, state, target);
    let resist = with_ammo(
        &effects,
        1,
        damage_resistance(order, state, target).min(100.0) * nerve,
    );
    let resist = (resist / 100.0)
        .min(setting("fMaxArmorRating", 85.0) / 100.0)
        .max(0.0);
    let mut threshold = with_ammo(
        &effects,
        2,
        worn_damage_threshold(order, state, target).max(0.0) * nerve,
    );
    if let Some((who, weapon)) = attacker {
        // A block adds the blocker's skill to the threshold, before the
        // perks (`world::melee::block_bonus`).
        if let Some(block) = crate::melee::block_bonus(order, state, who, weapon, target) {
            threshold += block;
        }
        let held = perks::weapon_tab(weapon);
        threshold -= perks::apply_for(
            order,
            state,
            who,
            perks::entry::MODIFY_DAMAGE_THRESHOLD_ATTACKER,
            0.0,
            &[held, Tab::Target(target)],
        );
        threshold += perks::apply_for(
            order,
            state,
            target,
            perks::entry::MODIFY_DAMAGE_THRESHOLD_DEFENDER,
            0.0,
            &[Tab::Target(who), held],
        );
    }
    let wear = setting("fDamageToArmorPercentage", 0.5);
    let mut armour_damage = wear * damage * resist;
    let after = damage * (1.0 - resist) - threshold.max(0.0);
    let mut after = with_ammo(&effects, 0, after);
    if let Some((who, weapon)) = attacker {
        after = perks::apply_for(
            order,
            state,
            who,
            perks::entry::CALCULATE_WEAPON_DAMAGE,
            after,
            &[perks::weapon_tab(weapon), Tab::Target(target)],
        );
    }
    armour_damage += wear * threshold.max(0.0).min(after - least);
    let weapon = attacker
        .and_then(|(_, w)| w)
        .and_then(|w| Weapon::load(order, w));
    ArmourHit {
        damage: after.max(least) * resisted_share(order, state, weapon.as_ref(), target),
        armour_damage,
    }
}

/// What's left of a hit after the target resists the weapon's kind of
/// damage, the last step of the armour (`009b5a30`, after the 20% floor,
/// so it can go below it): with a resist type (`DNAM` i32 at 120, the
/// weapon's `+0x16c`, `009b6600`; -1 none), r = min(the target's actor
/// value, 100) ÷ 100, and when 0 < r ≤ 1 the hit × (1 − r). Lasers and
/// plasma weapons are resisted by Energy Resistance (actor value 60), the
/// pulse gun by EMP Resistance (61), flamers by Fire Resistance (52).
// Translated from 009b5a30 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn resisted_share(
    order: &LoadOrder,
    state: &GameState,
    weapon: Option<&Weapon>,
    target: FormId,
) -> f32 {
    let Some(kind) = weapon.and_then(|w| w.resist) else {
        return 1.0;
    };
    let value = Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(target, kind)
    .unwrap_or(0.0) as f32;
    let r = value.min(100.0) / 100.0;
    if 0.0 < r && r <= 1.0 {
        1.0 - r
    } else {
        1.0
    }
}

/// A piece of armour's damage threshold or resistance at a condition
/// 0..1 (`004be0b0`, `004bdf90`): the record's figure truncated, × 1 above
/// half condition and 0.5 + the condition at or below (`00646360`,
/// `00646d40`), rounded up (`00476b20`).
pub fn armour_figure_at(value: f32, condition: f32) -> f32 {
    let factor = if condition <= 0.5 {
        1.0 - (0.5 - condition)
    } else {
        1.0
    };
    (value.trunc() * factor).ceil()
}

/// What someone's worn armour adds to a figure (`008d2110` the damage
/// threshold, `008d22b0` the resistance): each piece worn once, its
/// figure at its condition ([`armour_figure_at`]); `pick` reads the figure
/// from the record's `DNAM`.
fn worn_armour_sum(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    pick: &dyn Fn(&[u8]) -> Option<f32>,
) -> f32 {
    let mut seen: Vec<FormId> = Vec::new();
    let mut sum = 0.0;
    for &item in state.equipped.get(&who).into_iter().flatten() {
        if seen.contains(&item) {
            continue;
        }
        let Some(rr) = order
            .get(item)
            .filter(|r| r.entry.header.kind.as_bytes() == b"ARMO")
        else {
            continue;
        };
        let Some(v) = rr
            .record()
            .ok()
            .and_then(|r| r.get(DNAM).and_then(|d| pick(&d.data)))
        else {
            continue;
        };
        seen.push(item);
        sum += armour_figure_at(v, weapon_condition(state, who, item));
    }
    sum
}

/// A worn armour's damage resistance (`004bdf90`): its `DNAM` u16 at 0 (read unsigned, `MOVZX`)
/// (the form's `+0x178`) ÷ 100 (`004be080`), cut to a whole number (the
/// FPU's truncating mode, control word | 0xc00): Vault 11's jumpsuit
/// stores 100 for DR 1. At its condition: [`armour_figure_at`].
// Translated from 004bdf90 and 004be080 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn armour_resistance(stored: u16) -> f32 {
    (f32::from(stored) / 100.0).trunc()
}

/// Someone's damage resistance: their actor value (18) plus worn armour's
/// (`ARMO` `DNAM` u16 at 0 in hundredths, [`armour_resistance`]; each
/// piece at its condition, the sum at most `fMaxArmorRating`: `008d22b0`).
pub fn damage_resistance(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    // In hundredths (`004be080`); [`armour_figure_at`] truncates it as
    // `004bdf90` does.
    let mut worn = worn_armour_sum(order, state, who, &|d| {
        (d.len() >= 2).then(|| f32::from(u16::from_le_bytes([d[0], d[1]])) / 100.0)
    });
    let most = game_setting(order, "fMaxArmorRating").unwrap_or(90.0);
    if most > 0.0 {
        worn = worn.min(most);
    }
    worn + Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, 18)
    .unwrap_or(0.0) as f32
}

/// A sneak attack's multiplier (`009b73d0`): × `fCombatDamageBonus
/// SneakingMult` (2) for guns, × `fCombatDamageBonusMeleeSneakingMult` (5)
/// for melee and unarmed.
pub fn sneak_multiplier(order: &LoadOrder, melee: bool) -> f32 {
    if melee {
        game_setting(order, "fCombatDamageBonusMeleeSneakingMult").unwrap_or(5.0)
    } else {
        game_setting(order, "fCombatDamageBonusSneakingMult").unwrap_or(2.0)
    }
}

/// An item's condition, 0 to 1 (weapons and armour): full unless scripts
/// changed it (`SetWeaponHealthPerc`, `AddItemHealthPercent`), it was worn
/// down ([`damage_item`]) or repaired (`world::repair`).
pub fn weapon_condition(state: &GameState, holder: FormId, weapon: FormId) -> f32 {
    state
        .weapon_health
        .get(&(holder, weapon))
        .copied()
        .unwrap_or(1.0)
}

/// A weapon of `holder`'s loses `points` of health ([`damage_item`]: the
/// perks' "Modify Item Damage", entry point 68: Built to Destroy × 1.15,
/// Regular Maintenance × 0.5).
pub fn damage_weapon(
    order: &LoadOrder,
    state: &mut GameState,
    holder: FormId,
    weapon: &Weapon,
    points: f32,
) {
    damage_item(order, state, holder, weapon.form_id, points);
}

/// An item of `holder`'s (a weapon or armour) loses `points` of health
/// (`00891360`): nothing for none; the holder's perks' "Modify Item
/// Damage" (entry point 68) first; then its health less the points, nothing
/// left below 1; the condition kept as a share of its full health
/// (`world::repair::max_health`). The player is told when one drops from
/// 25% or more to below it: `sWeaponLowCond` / `sArmorLowCond`, with
/// `WPNBreak`. A weapon left with nothing breaks ([`break_weapon`]).
pub fn damage_item(
    order: &LoadOrder,
    state: &mut GameState,
    holder: FormId,
    item: FormId,
    points: f32,
) {
    let full = crate::repair::max_health(order, item) as f32;
    if points <= 0.0 || full <= 0.0 {
        return;
    }
    let points = perks::apply_for(
        order,
        state,
        holder,
        perks::entry::MODIFY_ITEM_DAMAGE,
        points,
        &[],
    );
    let before = weapon_condition(state, holder, item);
    let mut health = before * full - points;
    if health < 1.0 {
        health = 0.0;
    }
    let after = health / full;
    state.weapon_health.insert((holder, item), after);
    if holder == PLAYER_REF && (before * 100.0).min(100.0) >= 25.0 && after * 100.0 < 25.0 {
        let kind = order.get(item).map(|r| r.entry.header.kind);
        let (setting, default) = if kind == Some(WEAP) {
            (
                "sWeaponLowCond",
                "Your weapon condition is dangerously low.",
            )
        } else {
            ("sArmorLowCond", "Your armor condition is dangerously low.")
        };
        let text = crate::scripting::game_setting_text(order, setting)
            .unwrap_or_else(|| default.to_string());
        // Type 2: the sad Vault Boy.
        state.events.push(Event::Message {
            title: None,
            text,
            buttons: Vec::new(),
            icon: crate::message_icon::for_setting(setting).map(str::to_string),
        });
        if let Some(sound) = order.form_by_editor_id("WPNBreak") {
            state.events.push(Event::Sound(sound));
        }
    }
    if health <= 0.0 && order.get(item).is_some_and(|r| r.entry.header.kind == WEAP) {
        break_weapon(order, state, holder, item);
    }
}

/// A weapon with no health left breaks (`00891360` at 0): the player is
/// told (`sWeaponBreak`, `WPNBreak`) and it's taken off; someone else drops
/// it (the actor's slot 0x3cc), unless it can't be dropped (`DNAM` flags
/// 0x08), is embedded (0x20) or is a quest item, which they only put
/// away. (Broken things can't be put back on: `GameState::equip`.)
fn break_weapon(order: &LoadOrder, state: &mut GameState, holder: FormId, item: FormId) {
    if holder == PLAYER_REF {
        let text = crate::scripting::game_setting_text(order, "sWeaponBreak")
            .unwrap_or_else(|| "Your weapon has broken.".to_string());
        state.events.push(Event::Message {
            title: None,
            text,
            buttons: Vec::new(),
            icon: Some(crate::message_icon::SAD.to_string()),
        });
        if let Some(sound) = order.form_by_editor_id("WPNBreak") {
            state.events.push(Event::Sound(sound));
        }
        state.unequip_item(order, holder, item);
        return;
    }
    let flags = Weapon::load(order, item).map_or(0, |w| w.flags1);
    let keep =
        flags & (0x08 | 0x20) != 0 || crate::script_functions::is_quest_item(order, state, item);
    state.unequip_item(order, holder, item);
    if !keep {
        state.dropped.insert((holder, item));
    }
}

/// What a hit on the player wears off their armour (`0089a760`, with the
/// hit's `fArmorDamage`, Xbox PDB `HitData`): when above 0, a hit on the
/// head (part 1 or 2) wears what's worn on the head (biped slot 0), else
/// the hair slot's (1); any other hit what's worn on the upper body (2)
/// (`0089d8b0`); through [`damage_item`]. Only the player's armour wears
/// (the actor's slot 0x360).
pub fn wear_armour(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    part: Option<u8>,
    points: f32,
) {
    if who != PLAYER_REF || points <= 0.0 {
        return;
    }
    let head = matches!(part, Some(1) | Some(2));
    let slots: &[u32] = if head { &[0x1, 0x2] } else { &[0x4] };
    let worn: Vec<FormId> = state.equipped.get(&who).cloned().unwrap_or_default();
    let biped = |item: FormId| {
        order
            .get(item)
            .filter(|r| r.entry.header.kind.as_bytes() == b"ARMO")
            .and_then(|r| r.record().ok())
            .and_then(|r| {
                r.get(FourCC::new(b"BMDT"))
                    .filter(|s| s.data.len() >= 4)
                    .map(|s| le_u32(&s.data, 0))
            })
    };
    for &slot in slots {
        if let Some(&item) = worn
            .iter()
            .find(|&&i| biped(i).is_some_and(|b| b & slot != 0))
        {
            damage_item(order, state, who, item, points);
            return;
        }
    }
}

/// What an attack wears off the weapon used (`00893a40` as an attack
/// starts, `00646260`): `fDamageToWeaponValue` points of health (1 in
/// the data), after the ammunition's weapon-wear effects (kind 4) — then
/// through [`damage_weapon`] with the holder's perks.
pub fn attack_wear(order: &LoadOrder, ammo: Option<FormId>) -> f32 {
    let base = game_setting(order, "fDamageToWeaponValue").unwrap_or(1.0);
    let effects = ammo.map(|a| ammo_effects(order, a)).unwrap_or_default();
    with_ammo(&effects, 4, base)
}

/// How fast someone moves, beyond the base speed and crippled legs
/// (`00885bf0`): the player's perks' "Modify Run Speed" (entry point 42:
/// Travel Light × 1.1 in light armour) on the whole movement speed —
/// walking, running and sneaking alike, as the code applies it.
pub fn movement_speed_mult(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    perks::apply_for(order, state, who, perks::entry::MODIFY_RUN_SPEED, 1.0, &[])
}

/// The rate an attack animation plays at (`00893a40` as an attack starts,
/// `00895110`): the weapon's speed (`DNAM` f32 at 4, 1 on the vanilla
/// weapons) through the attacker's perks' "Modify Attack Speed" (entry
/// point 43: Fast Shot × 1.2 with guns, Slayer × 1.3 with melee and
/// unarmed; asked about the weapon, the fists when none) × its animation
/// attack multiplier (`DNAM` 60; the machete's 1.3). A weapon mod raising
/// the rate of fire would add to the multiplier (mods aren't here).
pub fn attack_rate(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    weapon: Option<&Weapon>,
) -> f32 {
    let speed = weapon.map_or(1.0, |w| w.speed);
    let speed = perks::apply_for(
        order,
        state,
        who,
        perks::entry::MODIFY_ATTACK_SPEED,
        speed,
        &[perks::weapon_tab(weapon.map(|w| w.form_id))],
    );
    speed * weapon.map_or(1.0, |w| w.attack_mult)
}

/// The rate reload animations play at (`008c17c0`): 1 + (Agility −
/// `fAgilityReloadBase` 5) × `fAgilityReloadModifier` 0.1 (the exe's
/// defaults: Agility 5 gives 1, 10 gives 1.5), through the perks' "Reload
/// Speed" (entry point 37: Rapid Reload × 1.25; asked about the weapon
/// held). The reload takes the weapon's reload time ÷ this.
pub fn reload_rate(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    weapon: Option<&Weapon>,
) -> f32 {
    animation_rate(order, state, who, weapon, perks::entry::RELOAD_SPEED)
}

/// The rate equip animations play at (`008c1940`): as [`reload_rate`],
/// with the perks' "Equip Speed" (entry point 38: Quick Draw × 1.5).
/// Nothing plays equip animations here yet.
pub fn equip_rate(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    weapon: Option<&Weapon>,
) -> f32 {
    animation_rate(order, state, who, weapon, perks::entry::EQUIP_SPEED)
}

fn animation_rate(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    weapon: Option<&Weapon>,
    entry: u8,
) -> f32 {
    let setting = |n: &str, default: f32| game_setting(order, n).unwrap_or(default);
    let agility = Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, 10)
    .unwrap_or(0.0) as f32;
    let rate = 1.0
        + (agility - setting("fAgilityReloadBase", 5.0)) * setting("fAgilityReloadModifier", 0.1);
    perks::apply_for(
        order,
        state,
        who,
        entry,
        rate,
        &[perks::weapon_tab(weapon.map(|w| w.form_id))],
    )
}

/// The ammunition item a shot gives back (`00523150`, the player only): an
/// ammunition's `DAT2` names the case or cell it leaves (a form at 12) and
/// the percentage of shots that leave one (f32 at 16: the 9mm round's
/// "Case, 9mm" 25%), through the perks' "Modify Chance for Ammo Item"
/// (entry point 57: Hand Loader × 2 with guns, Vigilant Recycler × 2 with
/// energy weapons; asked about the weapon); the item comes when a roll of
/// 0..100 is at most the chance. `roll` is the shooter's roll.
pub fn ammo_item_recovered(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    weapon: &Weapon,
    ammo: FormId,
    roll: u64,
) -> Option<FormId> {
    if who != PLAYER_REF {
        return None;
    }
    let rr = order.get(ammo)?;
    let d = rr
        .record()
        .ok()?
        .get(FourCC::new(b"DAT2"))
        .filter(|s| s.data.len() >= 20)?
        .data
        .clone();
    let item = rr.plugin.to_global(FormId(le_u32(&d, 12)));
    if item.0 == 0 {
        return None;
    }
    let chance = perks::apply_for(
        order,
        state,
        who,
        perks::entry::MODIFY_CHANCE_FOR_AMMO_ITEM,
        le_f32(&d, 16),
        &[Tab::Weapon(weapon.form_id)],
    );
    ((roll % 100) as f32 <= chance).then_some(item)
}

/// What gets through a damage threshold: the rest, but at least
/// `fMinDamMultiplier` (0.2) of the hit.
pub fn after_threshold(order: &LoadOrder, damage: f32, threshold: f32) -> f32 {
    let least = game_setting(order, "fMinDamMultiplier").unwrap_or(0.2);
    (damage - threshold).max(damage * least)
}

/// Whether someone is a creature (`CREA`) rather than a person.
pub fn is_creature(order: &LoadOrder, who: FormId) -> bool {
    base_of(order, who)
        .and_then(|b| order.get(b))
        .is_some_and(|r| r.entry.header.kind == CREA)
}

/// Someone's full health (`00643670`). The player: the record's base
/// health + `fAVDHealthEnduranceMult` (20) × (Endurance +
/// `fAVDHealthEnduranceOffset`) + `fAVDHealthLevelMult` (5) × (level − 1):
/// 200 at the start with Endurance 5. Other people: the base +
/// `fAVDNPCHealthEnduranceMult` (5) × (Endurance +
/// `fAVDNPCHealthEnduranceOffset`), no level term. Endurance is the
/// permanent value (implants count, chems don't). Creatures: their
/// record's health. People whose records are "auto-calculated" take
/// another formula in the game (`00603fc0`, not traced).
pub fn max_health(order: &LoadOrder, state: &GameState, who: FormId) -> Option<f64> {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    // Effects that raise it (Buffout's +60 while it lasts).
    let base_value = facts.base_actor_value(who, av::HEALTH)?
        + crate::magic::modifier(state, who, av::HEALTH, false);
    if is_creature(order, who) {
        return Some(base_value);
    }
    let setting = |n: &str| game_setting(order, n).map_or(0.0, f64::from);
    let endurance = facts
        .permanent_actor_value(who, av::ENDURANCE)
        .unwrap_or(0.0);
    if who == PLAYER_REF {
        let level = f64::from(state.player_level.max(1));
        return Some(
            base_value
                + setting("fAVDHealthEnduranceMult")
                    * (endurance + setting("fAVDHealthEnduranceOffset"))
                + setting("fAVDHealthLevelMult") * (level - 1.0),
        );
    }
    Some(
        base_value
            + setting("fAVDNPCHealthEnduranceMult")
                * (endurance + setting("fAVDNPCHealthEnduranceOffset")),
    )
}

/// Someone's base health (the actor value's base, `008803a0`: the record's
/// health plus what Endurance and level add, without effects), as limb
/// damage measures parts against: [`max_health`] less what effects add.
pub fn base_health(order: &LoadOrder, state: &GameState, who: FormId) -> Option<f64> {
    Some(max_health(order, state, who)? - crate::magic::modifier(state, who, av::HEALTH, false))
}

/// Someone's health now: full health less the damage taken.
pub fn health(order: &LoadOrder, state: &GameState, who: FormId) -> Option<f64> {
    let full = max_health(order, state, who)?;
    Some(full - state.damage.get(&who).copied().unwrap_or(0.0))
}

/// Someone takes damage: their health goes down, and at 0 they die (their
/// script's `OnDeath` runs where the viewer runs scripts). Whether they
/// died now.
pub fn hurt(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    amount: f64,
    by: FormId,
) -> bool {
    if state.coop.authority != crate::coop::Authority::Single {
        return crate::coop::facts::hurt_hooked(order, state, who, amount, by);
    }
    hurt_core(order, state, who, amount, by)
}

/// [`hurt`] without the co-op hook (what the hook calls once it has decided).
pub(crate) fn hurt_core(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    amount: f64,
    by: FormId,
) -> bool {
    if state.dead.contains(&who) || amount <= 0.0 {
        return false;
    }
    // Someone essential who is down takes no more harm (until a script
    // takes the flag off them: then they can be killed on the ground).
    // [G] Not traced: whether `Actor::Kill` (`0089d900`) is entered again
    // for life state 6 (its first test, vtable +0x22c, isn't read). Either
    // way they get up with full health (`008a0960`).
    if state.more.down.contains_key(&who) && crate::more_functions::is_essential(order, state, who)
    {
        return false;
    }
    *state.damage.entry(who).or_insert(0.0) += amount;
    state.last_blow.insert(who, (by, amount));
    crate::experience::count_damage(state, who, by, amount);
    if health(order, state, who).is_some_and(|h| h <= 0.0) {
        // Translated from 0089d900 (decompiled, FalloutNV.exe 1.4.0.525;
        // `Actor::Kill`): with `bEssentialTakeNoDamage` (INI, default 1,
        // `00fa8770`) an essential actor (`0087f3d0`) doesn't die. One
        // already unconscious (life state 3) or restrained (5) is set back
        // to `fEssentialHealthPercentReGain` of its base health; any other
        // goes down (life state 6, `ACTOR_LIFE_STATE_ESSENTIAL_DOWN` (Xbox
        // PDB), `008a1800(6)`) for `fEssentialDeathTime` seconds
        // (`SetEssentialDownTimer`, process +0xe8) with its health and
        // conditions restored (`008a0960`).
        if crate::more_functions::is_essential(order, state, who) {
            if state.unconscious.contains(&who) || state.set_by_scripts.restrained.contains(&who) {
                let regain = f64::from(
                    crate::scripting::game_setting(order, "fEssentialHealthPercentReGain")
                        .unwrap_or(0.3),
                );
                if let Some(base) = base_health(order, state, who) {
                    let full = max_health(order, state, who).unwrap_or(base);
                    state.damage.insert(who, (full - regain * base).max(0.0));
                }
                return false;
            }
            state.more.down.insert(who, essential_down_time(order));
            restore_after_essential_down(state, who);
            state.combat.remove(&who);
            state.combat.retain(|_, target| *target != who);
            state.events.push(Event::KnockedOut { who });
            return false;
        }
        state.more.down.remove(&who);
        state.dead.insert(who);
        state.deaths.insert(
            who,
            DeathStart {
                killer: Some(by),
                hit: true,
            },
        );
        // The dead stop fighting, and nobody fights them any more.
        state.combat.remove(&who);
        state.combat.retain(|_, target| *target != who);
        state.hit_targets.remove(&who);
        for list in state.hit_targets.values_mut() {
            list.retain(|t| *t != who);
        }
        state.events.push(Event::Died { who, by });
        if by == PLAYER_REF {
            let kind = if is_creature(order, who) {
                crate::stats::CREATURES_KILLED
            } else {
                crate::stats::PEOPLE_KILLED
            };
            crate::stats::bump(state, kind, 1);
            crate::stats::bump(state, crate::stats::TOTAL_THINGS_KILLED, 1);
            // Karma by the victim's own (`world::reputation::kill_karma`).
            let karma = crate::reputation::kill_karma(order, state, who);
            if karma != 0 {
                crate::reputation::reward_karma(order, state, karma);
            }
        }
        crate::experience::on_kill(order, state, who, by);
        return true;
    }
    false
}

/// How long an essential actor stays down: `fEssentialDeathTime` (the
/// exe's default 10 s, `00f69130`; no plugin sets it).
pub fn essential_down_time(order: &LoadOrder) -> f32 {
    crate::scripting::game_setting(order, "fEssentialDeathTime").unwrap_or(10.0)
}

/// `008a0960` (`Actor::RestoreFullHealthAndConditions`, Xbox PDB, by
/// size): health's damage restored (to above 0, so all of it), the same
/// for actor value 0x16, and the limb conditions (0x19-0x1f) by 1000.
// Translated from 008a0960 (decompiled, FalloutNV.exe 1.4.0.525). Only
// health is kept as damage here; the other values aren't damaged by
// this project's hits.
fn restore_after_essential_down(state: &mut GameState, who: FormId) {
    state.damage.remove(&who);
}

/// Those who are down count their seconds; at 0 they get up
/// (`Event::GotUp`). The player's teammate's seconds stand still while
/// the player fights.
// Translated from 00888b50 (decompiled, FalloutNV.exe 1.4.0.525): in life
// state 6 the process's essential-down timer counts down
// (`ModEssentialDownTimer`, +0xe0) while the actor lies knocked down; at 0
// or below the life state goes back to alive (`008a1800(0)`) and health
// and conditions are restored again (`008a0960`).
pub fn advance_down(_order: &LoadOrder, state: &mut GameState, seconds: f32) {
    if state.more.down.is_empty() {
        return;
    }
    // A teammate's time runs only while the player isn't fighting
    // (`crate::companions::down_time_runs`, the same function's test).
    let held: Vec<FormId> = state
        .more
        .down
        .keys()
        .filter(|w| !crate::companions::down_time_runs(state, **w))
        .copied()
        .collect();
    let mut up = Vec::new();
    for (who, left) in state.more.down.iter_mut() {
        if held.contains(who) {
            continue;
        }
        *left -= seconds;
        if *left <= 0.0 {
            up.push(*who);
        }
    }
    // The map's order is random per process; the events must not be.
    up.sort();
    for who in up {
        state.more.down.remove(&who);
        if state.dead.contains(&who) {
            continue;
        }
        restore_after_essential_down(state, who);
        state.events.push(Event::GotUp { who });
    }
}
/// How hard a killing hit throws the body, as a change of speed every body
/// of its ragdoll takes (game units a second; read from the game's code,
/// `008ae000`). With a weapon: its kill impulse × 2.5 Havok units a second
/// (9mm pistol 4 → 70 units a second; the varmint rifle's 0 throws
/// nothing), and for guns a tenth of that past the weapon's impulse
/// distance (`horizontal` is the shooter's distance across the ground).
/// Without one (a creature's bite, a punch): `fDeathForceForceMin` …
/// `Max` by where the damage falls between `fDeathForceDamageMin` and
/// `Max` (melee 0.1–40 → 20–60), or the `fDeathForceRanged…` ones
/// (0.1–10 → 20–40, the exe's defaults) for shots (`006466e0`).
pub fn death_push(
    order: &LoadOrder,
    weapon: Option<&Weapon>,
    damage: f64,
    ranged: bool,
    horizontal: f32,
) -> f32 {
    if let Some(w) = weapon.filter(|w| w.form_id.0 != 0) {
        let mut speed = w.kill_impulse * 2.5;
        let d = w.impulse_distance;
        if ranged && d > 0.0 {
            let (d2, far2) = (horizontal * horizontal, d * d);
            if d2 > far2 {
                // The game compares with 2 × D, not 2 × D², so past the
                // distance it's simply a tenth for any distance above 2.
                speed *= if d2 <= 2.0 * d {
                    (0.9 * (2.0 * far2 - d2) / far2 + 0.1).max(0.1)
                } else {
                    0.1
                };
            }
        }
        return speed * nif::collision::HAVOK_SCALE;
    }
    let setting =
        |name: &str, default: f32| crate::scripting::game_setting(order, name).unwrap_or(default);
    let kind = if ranged { "Ranged" } else { "" };
    let (d0, d1) = (
        setting(&format!("fDeathForce{kind}DamageMin"), 0.1),
        setting(
            &format!("fDeathForce{kind}DamageMax"),
            if ranged { 10.0 } else { 40.0 },
        ),
    );
    let (f0, f1) = (
        setting(&format!("fDeathForce{kind}ForceMin"), 20.0),
        setting(
            &format!("fDeathForce{kind}ForceMax"),
            if ranged { 40.0 } else { 60.0 },
        ),
    );
    let t = if d1 > d0 {
        ((damage.abs() as f32 - d0) / (d1 - d0)).clamp(0.0, 1.0)
    } else {
        1.0
    };
    (f0 + (f1 - f0) * t) * nif::collision::HAVOK_SCALE
}

/// The damage a fall does (`008a62b0`): nothing up to
/// `fJumpFallHeightMin` (600) units, then `fJumpFallHeightMult` (0.025, the
/// exe's default) × (fall − 600) ^ `fJumpFallHeightExponent` (1.65): 700
/// → 49.9, 1000 → 491. Measured from where the feet left the ground.
pub fn fall_damage(order: &LoadOrder, fall: f32) -> f64 {
    let setting = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
    let least = setting("fJumpFallHeightMin", 600.0);
    if fall <= least {
        return 0.0;
    }
    let mult = setting("fJumpFallHeightMult", 0.025);
    let exponent = setting("fJumpFallHeightExponent", 1.65);
    f64::from(mult * (fall - least).powf(exponent))
}

/// Someone lands after a fall: the damage (the player's legs, each with
/// `iFallLegDamageChance` (50) percent, also lose half of it:
/// `fFallLegDamageMult`; the exe's defaults), and for the player a hard or
/// light landing sound (`FSTLandHardHeavy` above
/// `fHardLandingDamageThreshold` 500 from the INI, else
/// `FSTLandHardLight`). Creatures that fly (`ACBS` flag 0x20) take none,
/// nor does the player's teammate, nor a person other than the player in
/// a worldspace flagged "no NPC fall damage" (`WRLD` `DATA` 0x40, the
/// worldspace's +0x4c, `00586320`). The damage taken.
///
/// Translated from 008a62b0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn land(order: &LoadOrder, state: &mut GameState, who: FormId, fall: f32) -> f64 {
    let damage = fall_damage(order, fall);
    if damage <= 0.0 || flies(order, who) {
        return 0.0;
    }
    if who != PLAYER_REF {
        let person = base_of(order, who)
            .and_then(|b| order.get(b))
            .is_some_and(|r| r.entry.header.kind.as_bytes() == b"NPC_");
        let world = state.place(order, who).map(|(space, ..)| space);
        let spared_here = world
            .and_then(|w| order.get(w))
            .filter(|r| r.entry.header.kind.as_bytes() == b"WRLD")
            .and_then(|r| r.record().ok())
            .and_then(|r| r.get(esm::sig::DATA).and_then(|d| d.data.first().copied()))
            .is_some_and(|f| f & 0x40 != 0);
        if (person && spared_here) || state.teammates.contains(&who) {
            return 0.0;
        }
    }
    hurt(order, state, who, damage, who);
    // Fall damage goes to the damage virtual (`0089d6f0`) directly, not
    // through the hit handler: no killing hit's push.
    not_a_hit(state, who);
    if who == PLAYER_REF {
        let chance = game_setting(order, "iFallLegDamageChance").unwrap_or(50.0);
        let share = f64::from(game_setting(order, "fFallLegDamageMult").unwrap_or(0.5));
        // Left and right mobility conditions.
        for leg in [29u16, 30] {
            if (state.roll() % 100) as f32 >= chance {
                continue;
            }
            *state.value_damage.entry((who, leg)).or_insert(0.0) += damage * share;
        }
        let sound = if damage > 500.0 {
            "FSTLandHardHeavy"
        } else {
            "FSTLandHardLight"
        };
        if let Some(s) = order.form_by_editor_id(sound) {
            state.events.push(Event::Sound(s));
        }
    }
    damage
}

/// Whether a creature flies (`ACBS` flag 0x20).
fn flies(order: &LoadOrder, who: FormId) -> bool {
    let Some(rr) = base_of(order, who)
        .and_then(|b| order.get(b))
        .filter(|r| r.entry.header.kind == CREA)
    else {
        return false;
    };
    rr.record()
        .ok()
        .and_then(|r| r.get(FourCC::new(b"ACBS")).map(|s| s.data.clone()))
        .is_some_and(|d| d.len() >= 4 && le_u32(&d, 0) & 0x20 != 0)
}

/// How much of the death push a ragdoll body takes, by its body part
/// number (`nif::RagdollBody::part`; the game's table `0119acc8`): head,
/// body, spine and upper arms all of it, forearms and thighs 0.75, hands
/// and calves 0.5, feet 0.25, the rest 0.75. (The body part hit takes 1.5
/// in the game; which part a hit lands on isn't tracked here.)
pub fn death_push_share(part: u8) -> f32 {
    match part {
        0..=5 | 11 => 1.0,
        6 | 8 | 12 | 14 => 0.75,
        7 | 9 | 13 | 15 => 0.5,
        10 | 16 => 0.25,
        _ => 0.75,
    }
}

/// Where the death push comes from: 2048 units back along the attack's
/// direction from where it struck (`008ae000`: behind the impact point
/// for shots; for melee 1024 × (back from the weapon − the hit direction)
/// from the attacker, which comes to about the same line).
pub fn death_push_origin(struck: [f32; 3], direction: [f32; 3]) -> [f32; 3] {
    let len = direction
        .iter()
        .map(|c| c * c)
        .sum::<f32>()
        .sqrt()
        .max(1e-6);
    [0, 1, 2].map(|i| struck[i] - 2048.0 * direction[i] / len)
}

/// A death just begun wasn't dealt by a hit (fall damage, effects,
/// `KillActor`): no killing hit's push follows.
pub fn not_a_hit(state: &mut GameState, who: FormId) {
    if let Some(d) = state.deaths.get_mut(&who) {
        d.hit = false;
    }
}

/// How a death began, for handing the body to its ragdoll
/// ([`GameState::deaths`]): who killed (none when `KillActor` names nobody:
/// `005be2a0` passes no attacker to the death routine `0089d900`), and
/// whether a killing hit's push follows (the hit handler `0089a760` calls
/// `008ae000` after a hit that killed; `KillActor` deals no hit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeathStart {
    pub killer: Option<FormId>,
    pub hit: bool,
}

/// The velocity every ragdoll body gains as the body goes limp, before any
/// killing hit's push (game units a second). Translated from `0089d900`
/// (`Actor::Kill`, Xbox PDB; FalloutNV.exe 1.4.0.525, `0089ee31`…
/// `0089f05c`), for an actor in the high process with its 3D, knock state 0
/// or 6, and a ragdoll (`Actor` +0x1b0 clear, set by `CreateRagdollInstance`
/// `0087e130` when there's none): after the character controller is taken
/// away (`RemoveCharController`, vtable +0x2a0) every body is made dynamic
/// (`00c6a350(root, 1, …)`) and gets, added to its own (`0062b8d0` →
/// `0062b930`, mass-independent, `004a3e90` turning game units into
/// Havok's), a direction × `fDeathForceForceMin` (`011cf724`; 20 in
/// FalloutNV.esm, the exe's default 35): the controller's last velocity
/// made a unit vector (`004a0c10`: nothing under 1e-6), or, standing
/// still, the way the actor faces (`GetHeading`, vtable +0x2bc, turning
/// (0, 1, 0): (sin h, cos h, 0)); killed by the player (and not the player
/// itself), from the player's controller to the victim's instead.
/// `player_line` is (victim, player) positions in that case.
// Translated from 0089d900 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn death_nudge(
    order: &LoadOrder,
    moving: [f32; 3],
    heading: f32,
    player_line: Option<([f32; 3], [f32; 3])>,
) -> [f32; 3] {
    let unit = |v: [f32; 3]| {
        let len = v.iter().map(|c| c * c).sum::<f32>().sqrt();
        if len <= 1e-6 {
            [0.0; 3]
        } else {
            v.map(|c| c / len)
        }
    };
    let moving_len = moving.iter().map(|c| c * c).sum::<f32>().sqrt();
    let direction = if let Some((victim, player)) = player_line {
        unit([0, 1, 2].map(|i| victim[i] - player[i]))
    } else if moving_len != 0.0 {
        unit(moving)
    } else {
        let (s, c) = heading.sin_cos();
        [s, c, 0.0]
    };
    let force = game_setting(order, "fDeathForceForceMin").unwrap_or(35.0);
    direction.map(|c| c * force)
}

/// Someone's damage threshold as the Pip-Boy shows it (`00782a90`): their
/// actor value (creatures' and people's base is 0) plus what their worn
/// armour gives (`ARMO` `DNAM`: DR, DT f32 at 4), plus the player's perks'
/// "Modify Damage Threshold (defender)" (entry point 56: Toughness adds 3,
/// then 6) asked about themselves and the weapon they hold. A hit asks for
/// it with the attacker instead ([`hit_through_armour`]).
pub fn damage_threshold(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    let threshold = worn_damage_threshold(order, state, who);
    let held = weapon_in_hand(order, state, who).map(|w| w.form_id);
    perks::apply_for(
        order,
        state,
        who,
        perks::entry::MODIFY_DAMAGE_THRESHOLD_DEFENDER,
        threshold,
        &[Tab::Target(who), perks::weapon_tab(held)],
    )
}

/// Someone's damage threshold before perks: their actor value plus worn
/// armour's (`ARMO` `DNAM` f32 at 4, each piece at its condition:
/// `008d2110`).
pub fn worn_damage_threshold(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    let worn = worn_armour_sum(order, state, who, &|d| (d.len() >= 8).then(|| le_f32(d, 4)));
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    worn + facts
        .current_actor_value(who, av::DAMAGE_THRESHOLD)
        .unwrap_or(0.0) as f32
}

/// The weapon someone has in hand: the one equipped, else (for people and
/// creatures with none equipped) the first weapon they carry. Not one
/// they've dropped (`world::body_parts::hurt_part`).
pub fn weapon_in_hand(order: &LoadOrder, state: &GameState, who: FormId) -> Option<Weapon> {
    let dropped = |i: FormId| state.dropped.contains(&(who, i));
    let equipped = state
        .equipped
        .get(&who)
        .into_iter()
        .flatten()
        .copied()
        .find(|&i| order.get(i).is_some_and(|r| r.entry.header.kind == WEAP) && !dropped(i));
    let disarmed = state.dropped.iter().any(|(w, _)| *w == who);
    // Someone whose fight chose their fists (`world::npc_combat`) put
    // their weapon away.
    if equipped.is_none() && state.npc_combat.unarmed.contains(&who) {
        return None;
    }
    // Else the player's first; someone else's best (`world::actor::
    // best_weapon`), from what they carry now or, untouched, what their
    // record gives them (as they're drawn).
    let id = equipped.or_else(|| {
        if who == PLAYER_REF && state.coop.authority != crate::coop::Authority::Single {
            // A co-op player holds what they equipped, never "the first
            // weapon carried" (that put the binoculars in every hand).
            None
        } else if crate::coop::facts::player_of(who).is_some() {
            None // a remote player's puppet: what the avatar says it holds
        } else if who == PLAYER_REF {
            state
                .inventory(order, who)
                .into_iter()
                .map(|(i, _)| i)
                .find(|&i| order.get(i).is_some_and(|r| r.entry.header.kind == WEAP))
        } else if state.stocked.contains(&who) || disarmed {
            let items: Vec<(FormId, i32)> = state
                .inventory(order, who)
                .into_iter()
                .filter(|(i, _)| !dropped(*i))
                .collect();
            crate::actor::best_weapon(order, &items)
        } else {
            crate::actor::carried_weapon(order, base_of(order, who)?)
        }
    })?;
    // As its mods change it (`world::weapon_mods`).
    Some(crate::weapon_mods::modded(
        order,
        state,
        who,
        Weapon::load(order, id)?,
    ))
}

/// The weapon someone has equipped, and nothing else (no "first carried"
/// guess): what a co-op avatar shows and publishes.
pub fn weapon_equipped(order: &LoadOrder, state: &GameState, who: FormId) -> Option<Weapon> {
    let id = state
        .equipped
        .get(&who)
        .into_iter()
        .flatten()
        .copied()
        .find(|&i| {
            order.get(i).is_some_and(|r| r.entry.header.kind == WEAP)
                && !state.dropped.contains(&(who, i))
        })?;
    Some(crate::weapon_mods::modded(
        order,
        state,
        who,
        Weapon::load(order, id)?,
    ))
}

/// How far a projectile carries, as the combat AI measures it
/// (`009a7f00`): `PROJ` `DATA` (flags u16 at 0, type u16 at 2, gravity f32
/// at 4, speed f32 at 8, range f32 at 12); a hitscan one (flag 0x01, every
/// bullet) or one that doesn't fall reaches its range; one that falls
/// (gravity above 0, forced to 1 for lobbers, type 2) speed² ÷ (gravity ×
/// the world's 686.61 units a second²), the farthest a throw at 45° goes
/// (the constant at `011f1c00` is taken to be the world's gravity: inferred
/// from the formula).
pub fn projectile_reach(order: &LoadOrder, projectile: FormId) -> Option<f32> {
    let d = order
        .get(projectile)?
        .record()
        .ok()?
        .get(esm::sig::DATA)
        .filter(|s| s.data.len() >= 16)?
        .data
        .clone();
    let flags = u16::from_le_bytes([d[0], d[1]]);
    let kind = u16::from_le_bytes([d[2], d[3]]);
    let gravity = if kind == 2 { 1.0 } else { le_f32(&d, 4) };
    let (speed, range) = (le_f32(&d, 8), le_f32(&d, 12));
    if flags & 0x01 != 0 || gravity <= 0.0 {
        return Some(range);
    }
    Some(speed * speed / (gravity * crate::combat_ai::WORLD_GRAVITY))
}

/// The projectile a weapon's shot is: the ammunition in use's (`AMMO`
/// `DAT2` form at 4) when it names one, else the weapon's own (`DNAM` 36),
/// as V.A.T.S. reads it (`world::vats`). The rockets' high-explosive and
/// high-velocity loads (`MissileProjectileHE`, `…HV`), the 25 mm HE and
/// 40 mm incendiary grenades fly their own projectiles this way.
pub fn fired_projectile(
    order: &LoadOrder,
    state: &GameState,
    shooter: FormId,
    weapon: &Weapon,
) -> Option<FormId> {
    let from_ammo = weapon
        .ammo_in_use(order, state, shooter)
        .and_then(|a| order.get(a))
        .and_then(|rr| {
            let d = rr.record().ok()?.get(FourCC::new(b"DAT2"))?.data.clone();
            (d.len() >= 8)
                .then(|| rr.plugin.to_global(FormId(le_u32(&d, 4))))
                .filter(|f| f.0 != 0)
        });
    from_ammo.or(weapon.projectile)
}

/// A creature's attack reach and type (`CREA` `RNAM` u8, `DATA` byte 0;
/// the tutorial gecko 25, type 1): `None` for anyone else. From its
/// template when it takes its base data (reach; which template flag covers
/// `RNAM` is a guess) or stats (type) from one.
pub fn creature_reach(order: &LoadOrder, who: FormId) -> Option<(f32, u8)> {
    let base = base_of(order, who)?;
    if order.get(base)?.entry.header.kind != CREA {
        return None;
    }
    let (_, record) = crate::actor::data_record(order, base, crate::actor::USE_BASE_DATA)?;
    let reach = record
        .get(FourCC::new(b"RNAM"))
        .and_then(|s| s.data.first().copied())
        .unwrap_or(0);
    let (_, stats) = crate::actor::data_record(order, base, crate::actor::USE_STATS)?;
    let kind = stats
        .get(esm::sig::DATA)
        .and_then(|s| s.data.first().copied())
        .unwrap_or(0);
    Some((f32::from(reach), kind))
}

/// A creature's own attack: its record's damage (`CREA` `DATA` i16 at 8,
/// its template's when it takes its stats from one; its reach is
/// [`creature_reach`]).
pub fn creature_damage(order: &LoadOrder, who: FormId) -> Option<f32> {
    let base = base_of(order, who)?;
    if order.get(base)?.entry.header.kind != CREA {
        return None;
    }
    let (_, record) = crate::actor::data_record(order, base, crate::actor::USE_STATS)?;
    let d = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 10)?;
    Some(f32::from(i16::from_le_bytes([d.data[8], d.data[9]])))
}

/// The player's Ready Item key (R: the game's default keyboard binding,
/// `00a24b70`), as the player's code reads it each frame (`009466d0`, the
/// reload in `00948310`): while the key is down a timer runs (`011e07e0`);
/// nothing happens while the weapon is still being drawn or put away (the
/// game waits for its queued weapon action, `008a7570() == -1`); with
/// the weapon holstered the first frame draws it; with it out, a weapon
/// that can't reload (nothing loaded: fists, melee weapons) is put away at
/// once, a gun once the key has been held longer than
/// `fPlayerWeaponReloadTimer` ([`RELOAD_TIMER`]); releasing the key with a
/// gun out before that reloads (as long as it was held at all,
/// `00948310`). Letting go resets the timer and the press. Each
/// press draws or puts away once (`011e0bec`). Attacking with the weapon
/// holstered draws it instead of attacking (`00948310`); changing weapons
/// puts it away (`0088db20`, `0088d7d0`), and a new game starts with it
/// away (`process+0x135` starts 0).
/// `fPlayerWeaponReloadTimer` (the setting at `011cdfcc`, made by the
/// static initializer `00f5bd60` with 0.5): how long the Ready Item key is
/// held before a gun is put away instead of reloaded.
pub const RELOAD_TIMER: f32 = 0.5;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ReadyKey {
    /// How long the key has been held this press, seconds.
    pub held: f32,
    /// Whether this press has drawn or put the weapon away already.
    handled: bool,
}

/// The key this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    /// Down (pressed this frame or still held).
    Held,
    /// Let go this frame.
    Released,
    Up,
}

/// What the key asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadyAction {
    Nothing,
    Draw,
    PutAway,
    Reload,
}

impl ReadyKey {
    /// One frame of `seconds` with the key in `key`'s state. `out`: the
    /// weapon is out; `reloadable`: it has ammunition loaded (a gun with
    /// its ammunition equipped, `00525980`); `readying`: a drawing or
    /// putting away still plays.
    // Translated from 009466d0 and 00948310 (decompiled, FalloutNV.exe
    // 1.4.0.525)
    pub fn update(
        &mut self,
        key: KeyState,
        seconds: f32,
        out: bool,
        reloadable: bool,
        readying: bool,
    ) -> ReadyAction {
        match key {
            KeyState::Held => {
                self.held += seconds;
                if readying || self.handled {
                    ReadyAction::Nothing
                } else if !out {
                    self.handled = true;
                    ReadyAction::Draw
                } else if !reloadable || self.held > RELOAD_TIMER {
                    self.handled = true;
                    ReadyAction::PutAway
                } else {
                    ReadyAction::Nothing
                }
            }
            KeyState::Released => {
                let reload =
                    out && !readying && reloadable && self.held > 0.0 && self.held < RELOAD_TIMER;
                *self = ReadyKey::default();
                if reload {
                    ReadyAction::Reload
                } else {
                    ReadyAction::Nothing
                }
            }
            KeyState::Up => {
                *self = ReadyKey::default();
                ReadyAction::Nothing
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn worn_armour_resistance_is_stored_in_hundredths() {
        // `VaultSuit11` stores 100, `OutfitTrenchcoat` 500,
        // `DamWarTrooper` 2000, `ArmorTeslaPowerFX` 4000.
        assert_eq!(super::armour_resistance(100), 1.0);
        assert_eq!(super::armour_resistance(500), 5.0);
        assert_eq!(super::armour_resistance(2000), 20.0);
        assert_eq!(super::armour_resistance(4000), 40.0);
        assert_eq!(super::armour_resistance(250), 2.0);
        assert_eq!(super::armour_resistance(0), 0.0);
        // Unsigned (`MOVZX`): 40000 hundredths is 400, not negative.
        assert_eq!(super::armour_resistance(40000), 400.0);
    }

    use super::*;
    use KeyState::*;
    use ReadyAction::*;

    #[test]
    fn the_ready_key_draws_puts_away_and_reloads_as_the_game_does() {
        let mut key = ReadyKey::default();
        // Holstered: the press draws at once, and holding on does nothing
        // more; letting go while the drawing plays doesn't reload.
        assert_eq!(key.update(Held, 0.02, false, true, false), Draw);
        assert_eq!(key.update(Held, 0.02, true, true, true), Nothing);
        assert_eq!(key.update(Released, 0.02, true, true, true), Nothing);
        assert_eq!(key.update(Up, 0.02, true, true, false), Nothing);
        // A gun out: a tap reloads on release; while it's held nothing
        // happens until the hold time (0.5 s) is passed: then it's put
        // away, and letting go doesn't reload.
        assert_eq!(key.update(Held, 0.02, true, true, false), Nothing);
        assert_eq!(key.update(Held, 0.02, true, true, false), Nothing);
        assert!((key.held - 0.04).abs() < 1e-6);
        assert_eq!(key.update(Released, 0.02, true, true, false), Reload);
        assert_eq!(key.held, 0.0);
        assert_eq!(key.update(Held, 0.3, true, true, false), Nothing);
        assert_eq!(key.update(Held, 0.3, true, true, false), PutAway);
        assert_eq!(key.update(Held, 0.3, false, true, true), Nothing);
        assert_eq!(key.update(Released, 0.02, false, true, false), Nothing);
        // Fists or a melee weapon out: the press puts them away at once.
        assert_eq!(key.update(Held, 0.02, true, false, false), PutAway);
        assert_eq!(key.update(Held, 0.02, false, false, true), Nothing);
        assert_eq!(key.update(Released, 0.02, false, false, true), Nothing);
        // Nothing while a drawing or putting away still plays.
        assert_eq!(key.update(Held, 0.02, false, true, true), Nothing);
        assert_eq!(key.update(Up, 0.02, false, true, true), Nothing);
        // A release that was never held doesn't reload.
        assert_eq!(key.update(Released, 0.02, true, true, false), Nothing);
    }
}
