//! The combat AI, read from the game's code (`%USERPROFILE%\nv-re\
//! findings\combat_ai.md`, decompiled in `%USERPROFILE%\nv-re\decomp\
//! combat`): who starts a fight on noticing whom, combat styles (`CSTY`),
//! how gunmen keep their distance and when they shoot, how melee fighters
//! close in and choose between attacking, blocking and waiting, and when a
//! lost target is searched for and given up.
//!
//! Everything here is a plain rule over numbers the caller supplies
//! (distances, angles, random numbers, times), so the viewer only measures
//! and moves. Settings come through a [`Setting`] lookup: the load order's
//! value, else the exe's default given at each call.
//!
//! - Noticing ([`detection_interval`], [`starts_combat`]): every actor
//!   works out its detection value (`world::detection`) for the player and
//!   the other actors every `fDetectionTimerSetting` (0.3 s) in combat, and
//!   out of combat 0.3 s plus a random stagger of at most 5 s (`008e40d0`).
//!   When the value first rises above `fSneakNoticedMin` (−20: noticed, not
//!   necessarily seen) it starts a fight if its aggression says so for its
//!   reaction (`world::factions::attacks_on_sight`; `008f5480`, `008ff350`).
//!   Allies join by their Assistance ([`assists_against`]); the
//!   unaggressive run from those who'd attack them when much weaker
//!   ([`flees_on_sight`]).
//! - Ranged ([`ranged_band`], [`Engage`], [`RangedAttack`]): between the
//!   weapon's min range and its max range (× the style's multipliers);
//!   out of that band for 2 s they move, inside it every 2–5 s they strafe
//!   (75%); semi-automatic shots once the attack has ended and a random
//!   delay (the weapon's semi-auto delay × the style's) has passed, with the
//!   target inside the aim arc; automatic weapons fire 1 s bursts with 1 s
//!   pauses.
//! - Melee ([`melee_reach`], [`melee_approach`], [`melee_scores`],
//!   [`melee_choice`]): run until the gap between the bodies is within
//!   reach + 64, then fast-walk into reach; after each attack roll attack,
//!   block or hold by the style's chances.
//! - Losing the target ([`TargetMemory`]): unseen for 15 s they search; unseen
//!   for 30 s and never seen, or 60 s and more than 4096 units away, they give
//!   up and go back to their packages.

use std::collections::HashMap;
use std::sync::Mutex;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::combat::Weapon;
use crate::dialogue::PLAYER_REF;
use crate::factions::{self, Reaction};
use crate::scripting::{base_of, game_setting, Facts, GameState};

const CSTD: FourCC = FourCC::new(b"CSTD");
const CSAD: FourCC = FourCC::new(b"CSAD");
const CSSD: FourCC = FourCC::new(b"CSSD");
const ZNAM: FourCC = FourCC::new(b"ZNAM");
const AIDT: FourCC = FourCC::new(b"AIDT");

/// The world's gravity, game units a second² (98.1 Havok units ×
/// 6.9991; `findings\physics.md`).
pub const WORLD_GRAVITY: f32 = 686.61;

/// The combat style actors without one of their own use: the engine's own
/// form `0000003D` (`DefaultCombatstyle` in `FalloutNV.esm`). That it's the
/// fallback is inferred from its being a built-in form (IDs below 0x800).
pub const DEFAULT_STYLE: FormId = FormId(0x3D);

/// People's collision radius, game units: the character controller's
/// (23 + 17.5) ÷ 2, one size for everyone (`00c72410`; `findings\
/// physics.md`).
pub const PERSON_RADIUS: f32 = 20.25;

/// A game setting by name, with the exe's default for when the load order
/// doesn't set it.
pub type Setting<'a> = &'a dyn Fn(&str, f32) -> f32;

/// Game settings looked up once and kept (a detection value alone asks
/// for about twenty).
#[derive(Default)]
pub struct SettingCache(Mutex<HashMap<String, Option<f32>>>);

impl SettingCache {
    /// The setting's value in `order`, else `default`.
    pub fn get(&self, order: &LoadOrder, name: &str, default: f32) -> f32 {
        let mut map = self.0.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = map.get(name) {
            return v.unwrap_or(default);
        }
        let v = game_setting(order, name);
        map.insert(name.to_string(), v);
        v.unwrap_or(default)
    }
}

// ---------------------------------------------------------------------------
// Combat styles
// ---------------------------------------------------------------------------

/// The indices of the style's advanced values (`CSAD`, 21 floats) that the
/// rules here use.
pub mod advanced {
    pub const BLOCK_SKILL_MULT: usize = 10;
    pub const BLOCK_SKILL_BASE: usize = 11;
    pub const BLOCK_UNDER_ATTACK: usize = 12;
    pub const BLOCK_NOT_UNDER_ATTACK: usize = 13;
    pub const ATTACK_SKILL_MULT: usize = 14;
    pub const ATTACK_SKILL_BASE: usize = 15;
    pub const ATTACK_UNDER_ATTACK: usize = 16;
    pub const ATTACK_NOT_UNDER_ATTACK: usize = 17;
    pub const ATTACK_DURING_BLOCK: usize = 18;
    pub const POWER_ATTACK_FATIGUE_BASE: usize = 19;
    pub const POWER_ATTACK_FATIGUE_MULT: usize = 20;
}

/// A combat style (`CSTY`), as the game loads it (`00505180`): `CSTD` (92
/// bytes), `CSAD` (84 bytes, 21 floats) and `CSSD` (64 bytes) copied byte
/// for byte; a `CSSD` shorter than 64 keeps the rest; records older than
/// form version 12 get both ranged range multipliers set to 1. Which bytes
/// mean what is read from the getters (`findings\combat_ai.md` §3);
/// `CSTD` chances are signed bytes as the code reads them.
#[derive(Debug, Clone, PartialEq)]
pub struct CombatStyle {
    pub form_id: Option<FormId>,
    /// `CSTD` u8 at 0 and 1: dodge chance, and left/right of those.
    pub dodge_chance: u8,
    pub dodge_left_right_chance: u8,
    /// `CSTD` u8 at 36 and 37: block and attack chance (`00644640`,
    /// `00798420`).
    pub block_chance: u8,
    pub attack_chance: u8,
    /// `CSTD` f32 at 40, 44, 48: added to the attack score against a
    /// recoiling or staggered target, an unconscious one, and unarmed.
    pub recoil_attack_bonus: f32,
    pub unconscious_attack_bonus: f32,
    pub hand_to_hand_attack_bonus: f32,
    /// `CSTD` u8 at 52, f32 at 56 and 60: the power attack chance and its
    /// bonuses against a recoiling and an unconscious target.
    pub power_attack_chance: u8,
    pub recoil_power_attack_bonus: f32,
    pub unconscious_power_attack_bonus: f32,
    /// `CSTD` 5 × u8 at 64: power attack directions (normal, forward, back,
    /// left, right).
    pub power_attack_weights: [u8; 5],
    /// `CSTD` f32 at 72 and 76: how long a melee hold lasts, seconds.
    pub hold_timer: (f32, f32),
    /// `CSTD` u16 at 80: 0x04 flee by personal survival (`009928c0`).
    pub flags: u16,
    /// `CSAD`'s 21 values (see [`advanced`]).
    pub advanced: [f32; 21],
    /// `CSSD` f32 at 0 and 4: the cover search radius and take-cover
    /// chance; 8/12, 16/20, 24/28 the wait, wait-to-fire and fire timers
    /// (min, max) of fighting from cover.
    pub cover_search_radius: f32,
    pub take_cover_chance: f32,
    pub wait_timer: (f32, f32),
    pub wait_to_fire_timer: (f32, f32),
    pub fire_timer: (f32, f32),
    /// `CSSD` f32 at 32 and 44: the ranged range multipliers on the
    /// weapon's min and max range (`009a9350`, `00508050`).
    pub range_mult_min: f32,
    pub range_mult_max: f32,
    /// `CSSD` u32 at 40: weapon restrictions (0 none, 1 melee only, 2
    /// ranged only; from the data).
    pub weapon_restrictions: u32,
    /// `CSSD` f32 at 48: the most a target may be off to the side, degrees
    /// across (0: no limit; `00639aa0`).
    pub max_targeting_fov: f32,
    /// `CSSD` f32 at 52: the combat radius.
    pub combat_radius: f32,
    /// `CSSD` f32 at 56 and 60: multipliers on the weapon's semi-automatic
    /// fire delay (`009d1b30`, `009d1b50`).
    pub semi_auto_delay_mult: (f32, f32),
}

impl Default for CombatStyle {
    /// The values `FalloutNV.esm` gives `DefaultCombatstyle`, for load
    /// orders that don't have it (test plugins).
    fn default() -> Self {
        CombatStyle {
            form_id: None,
            dodge_chance: 75,
            dodge_left_right_chance: 50,
            block_chance: 30,
            attack_chance: 40,
            recoil_attack_bonus: 5.0,
            unconscious_attack_bonus: 5.0,
            hand_to_hand_attack_bonus: 5.0,
            power_attack_chance: 25,
            recoil_power_attack_bonus: 5.0,
            unconscious_power_attack_bonus: 5.0,
            power_attack_weights: [20; 5],
            hold_timer: (0.5, 1.5),
            flags: 0x1C1,
            advanced: [
                -20.0, 0.0, -110.0, 1.0, 1.0, 0.75, 1.0, 0.7, 1.0, 0.5, 20.0, 0.0, 2.0, 1.0, 20.0,
                0.0, 0.75, 1.0, 0.5, 5.0, -10.0,
            ],
            cover_search_radius: 2048.0,
            take_cover_chance: 100.0,
            wait_timer: (2.0, 2.0),
            wait_to_fire_timer: (10.0, 10.0),
            fire_timer: (2.0, 2.0),
            range_mult_min: 1.0,
            range_mult_max: 1.0,
            weapon_restrictions: 0,
            max_targeting_fov: 0.0,
            combat_radius: 10000.0,
            semi_auto_delay_mult: (1.0, 1.0),
        }
    }
}

impl CombatStyle {
    /// A style from its three subrecords (any may be short or missing: what
    /// they don't give keeps [`CombatStyle::default`]'s value) and the
    /// record's form version.
    pub fn parse(cstd: &[u8], csad: &[u8], cssd: &[u8], version: u16) -> CombatStyle {
        let mut s = CombatStyle::default();
        let byte = |d: &[u8], at: usize, old: u8| d.get(at).copied().unwrap_or(old);
        let float = |d: &[u8], at: usize, old: f32| {
            if d.len() >= at + 4 {
                le_f32(d, at)
            } else {
                old
            }
        };
        s.dodge_chance = byte(cstd, 0, s.dodge_chance);
        s.dodge_left_right_chance = byte(cstd, 1, s.dodge_left_right_chance);
        s.block_chance = byte(cstd, 36, s.block_chance);
        s.attack_chance = byte(cstd, 37, s.attack_chance);
        s.recoil_attack_bonus = float(cstd, 40, s.recoil_attack_bonus);
        s.unconscious_attack_bonus = float(cstd, 44, s.unconscious_attack_bonus);
        s.hand_to_hand_attack_bonus = float(cstd, 48, s.hand_to_hand_attack_bonus);
        s.power_attack_chance = byte(cstd, 52, s.power_attack_chance);
        s.recoil_power_attack_bonus = float(cstd, 56, s.recoil_power_attack_bonus);
        s.unconscious_power_attack_bonus = float(cstd, 60, s.unconscious_power_attack_bonus);
        for (i, w) in s.power_attack_weights.iter_mut().enumerate() {
            *w = byte(cstd, 64 + i, *w);
        }
        s.hold_timer = (
            float(cstd, 72, s.hold_timer.0),
            float(cstd, 76, s.hold_timer.1),
        );
        if cstd.len() >= 82 {
            s.flags = u16::from_le_bytes([cstd[80], cstd[81]]);
        }
        for (i, v) in s.advanced.iter_mut().enumerate() {
            *v = float(csad, i * 4, *v);
        }
        s.cover_search_radius = float(cssd, 0, s.cover_search_radius);
        s.take_cover_chance = float(cssd, 4, s.take_cover_chance);
        s.wait_timer = (
            float(cssd, 8, s.wait_timer.0),
            float(cssd, 12, s.wait_timer.1),
        );
        s.wait_to_fire_timer = (
            float(cssd, 16, s.wait_to_fire_timer.0),
            float(cssd, 20, s.wait_to_fire_timer.1),
        );
        s.fire_timer = (
            float(cssd, 24, s.fire_timer.0),
            float(cssd, 28, s.fire_timer.1),
        );
        s.range_mult_min = float(cssd, 32, s.range_mult_min);
        if cssd.len() >= 44 {
            s.weapon_restrictions = le_u32(cssd, 40);
        }
        s.range_mult_max = float(cssd, 44, s.range_mult_max);
        s.max_targeting_fov = float(cssd, 48, s.max_targeting_fov);
        s.combat_radius = float(cssd, 52, s.combat_radius);
        s.semi_auto_delay_mult = (
            float(cssd, 56, s.semi_auto_delay_mult.0),
            float(cssd, 60, s.semi_auto_delay_mult.1),
        );
        if version < 12 {
            s.range_mult_min = 1.0;
            s.range_mult_max = 1.0;
        }
        s
    }

    /// A `CSTY` record.
    pub fn load(order: &LoadOrder, id: FormId) -> Option<CombatStyle> {
        let rr = order
            .get(id)
            .filter(|r| r.entry.header.kind.as_bytes() == b"CSTY")?;
        let record = rr.record().ok()?;
        let data = |kind: FourCC| record.get(kind).map_or(&[][..], |s| s.data.as_slice());
        let mut style =
            CombatStyle::parse(data(CSTD), data(CSAD), data(CSSD), rr.entry.header.version);
        style.form_id = Some(id);
        Some(style)
    }

    /// Someone's combat style: their base record's `ZNAM` (their
    /// template's when it gives the AI data: which template flag covers the
    /// style is a guess), else [`DEFAULT_STYLE`], else the default values.
    pub fn of(order: &LoadOrder, who: FormId) -> CombatStyle {
        person(order, who)
            .and_then(|b| crate::actor::data_record(order, b, crate::actor::USE_AI_DATA))
            .and_then(|(rr, r)| {
                let s = r.get(ZNAM).filter(|s| s.data.len() >= 4)?;
                Some(rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
            })
            .and_then(|id| CombatStyle::load(order, id))
            .or_else(|| CombatStyle::load(order, DEFAULT_STYLE))
            .unwrap_or_default()
    }
}

/// A placed person's base record, or the record itself when it's a person
/// or creature.
fn person(order: &LoadOrder, who: FormId) -> Option<FormId> {
    let kind = order.get(who)?.entry.header.kind;
    if kind.as_bytes() == b"NPC_" || kind.as_bytes() == b"CREA" {
        return Some(who);
    }
    base_of(order, who)
}

/// One byte of someone's `AIDT` (their template's when it gives the AI
/// data), or an actor value scripts set instead.
fn ai_byte(order: &LoadOrder, state: &GameState, who: FormId, index: usize, av: u16) -> u8 {
    if let Some(&v) = state.actor_values.get(&(who, av)) {
        return v.clamp(0.0, 255.0) as u8;
    }
    person(order, who)
        .and_then(|b| crate::actor::data_record(order, b, crate::actor::USE_AI_DATA))
        .and_then(|(_, r)| r.get(AIDT).and_then(|s| s.data.get(index).copied()))
        .unwrap_or(0)
}

/// Aggression now (actor value 0, `AIDT` byte 0): 0 unaggressive, 1
/// aggressive, 2 very aggressive, 3 frenzied.
pub fn aggression(order: &LoadOrder, state: &GameState, who: FormId) -> u8 {
    ai_byte(order, state, who, 0, 0).min(3)
}

/// Confidence now (actor value 1, `AIDT` byte 1): 0 cowardly … 4
/// foolhardy.
pub fn confidence(order: &LoadOrder, state: &GameState, who: FormId) -> u8 {
    ai_byte(order, state, who, 1, 1).min(4)
}

/// Assistance now (actor value 57 via `008beae0`, `AIDT` byte 14): 0
/// helps nobody, 1 allies, 2 friends and allies.
pub fn assistance(order: &LoadOrder, state: &GameState, who: FormId) -> u8 {
    ai_byte(order, state, who, 14, 57)
}

// ---------------------------------------------------------------------------
// Noticing and starting fights
// ---------------------------------------------------------------------------

/// Seconds until an actor's next detection run (`008e40d0`, timer at
/// process+0x2f8): `fDetectionTimerSetting` (0.3) in combat; otherwise that
/// plus `unit` (0..1) × min(frame time × the number of actors running
/// detection, 5 s), which spreads the work over frames.
pub fn detection_interval(
    in_combat: bool,
    frame_time: f32,
    actors: usize,
    unit: f32,
    s: Setting,
) -> f32 {
    let base = s("fDetectionTimerSetting", 0.3);
    if in_combat {
        base
    } else {
        base + unit * (frame_time * actors as f32).min(5.0)
    }
}

/// Detection only runs for actors nearer the player than this (the double
/// 8192.0 at `01084d28`).
pub const DETECTION_RANGE: f32 = 8192.0;

/// Whether `who`, having just noticed `target` (its detection value rising
/// above `fSneakNoticedMin` −20; considered once per rise, `008f5480` for
/// the player, `008ff350` for others), starts a fight with it: its
/// aggression for its reaction (`world::factions::attacks_on_sight`,
/// `008b06d0`), and with `iCombatTargetPlayerSoftCap` (15) others already
/// fighting the player, only a value above 0 (seen) joins in. (The aggro
/// radius's temporary guard package, `AIDT` bytes 15–19, isn't done: its
/// behaviour isn't traced.)
pub fn starts_combat(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    target: FormId,
    value: i32,
    s: Setting,
) -> bool {
    if value as f32 <= s("fSneakNoticedMin", -20.0) || who == target {
        return false;
    }
    if state.dead.contains(&who) || state.dead.contains(&target) {
        return false;
    }
    // No fight with or by a ghost (`SetGhost`; `008ff350` asks both).
    if crate::more_functions::is_ghost(state, who) || crate::more_functions::is_ghost(state, target)
    {
        return false;
    }
    if !factions::attacks_on_sight(order, state, who, target) {
        return false;
    }
    if target == PLAYER_REF {
        let cap = s("iCombatTargetPlayerSoftCap", 15.0).max(0.0) as usize;
        let already = state
            .combat
            .iter()
            .filter(|(w, t)| **t == PLAYER_REF && **w != who && !state.dead.contains(w))
            .count();
        if already >= cap && value <= 0 {
            return false;
        }
    }
    true
}

/// Whether `helper` takes `friend`'s side (`00992530` → `008b0970`): not
/// when frenzied; with Assistance 1 for an ally, 2 for a friend or ally,
/// by `helper`'s reaction to `friend`.
pub fn helps(order: &LoadOrder, state: &GameState, helper: FormId, friend: FormId) -> bool {
    if aggression(order, state, helper) == 3 {
        return false;
    }
    let reaction = factions::reaction(order, state, helper, friend);
    match assistance(order, state, helper) {
        1 => reaction == Reaction::Ally,
        2 => matches!(reaction, Reaction::Ally | Reaction::Friend),
        _ => false,
    }
}

/// Whom `helper` joins a fight against (`008ff350`): `friend`, which it
/// sees (`friend_value` above 0), is fighting someone; `helper` would help
/// it ([`helps`]); that someone isn't already `helper`'s target and
/// `helper` detects it at 1 or more (`enemy_value`).
pub fn assists_against(
    order: &LoadOrder,
    state: &GameState,
    helper: FormId,
    friend: FormId,
    friend_value: i32,
    enemy_value: impl FnOnce(FormId) -> Option<i32>,
) -> Option<FormId> {
    if friend_value <= 0 || state.dead.contains(&friend) {
        return None;
    }
    let enemy = *state.combat.get(&friend)?;
    if enemy == helper || state.combat.get(&helper) == Some(&enemy) || state.dead.contains(&enemy) {
        return None;
    }
    if !helps(order, state, helper, friend) {
        return None;
    }
    (enemy_value(enemy)? >= 1).then_some(enemy)
}

/// What someone hurt by another person or creature does about it
/// ([`attacked_by`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Attacked {
    /// They start fighting the attacker (`BaseProcess::EnterCombat`, Xbox
    /// PDB, process vtable +0x33c).
    pub enters_combat: bool,
    /// Already fighting (or starting to), they take the attacker on as one
    /// more target (`CombatController::AddTarget`, Xbox PDB, `0097f930`).
    pub new_target: bool,
}

/// `victim` was hurt by `attacker`, someone other than the player:
/// `Actor::AttackedBy` (Xbox PDB). Nothing for the dead, ghosts or a hit
/// on oneself. Unless the victim is fighting already with the attacker as
/// its target (`CombatController::IsActoraCombatTarget`, `0097fa10`), it
/// starts fighting them only when it can (`CanAttackActor` `008b0670`: a
/// teammate always, else [`factions::fights_when_hit`]: not a friend or
/// ally, unless someone is frenzied) **and** the attacker was fighting the
/// victim (the attacker's combat target, vtable +0x42c), or fighting the
/// player when the victim follows the player (`008b0ba0`, the process's
/// commanding actor; taken as the player's teammates). A stray shot from
/// someone fighting somebody else doesn't start a fight. Then, when the
/// victim is fighting (`0097f580`, likely `CombatController::
/// DamagedByAttacker`, Xbox PDB, by its arguments): an attacker that isn't
/// its target yet and that it would fight when hit ([`factions::
/// fights_when_hit`]) becomes another target, whoever they aimed at.
///
/// Not done (labelled): the detection of the attacker set to 3 when the hit
/// is noticed (process +0x504/+0xf0), an NPC's hit on a guard out of combat
/// raising an assault alarm (`008c0460` with the attacker, vtables +0x304
/// `IsActoraGuard`, +0x218 `IsNpc`, +0x37c `GetRace`, Xbox +4), the life
/// state 4 / commanding-actor early return, `SetBeenAttacked` (+0x474).
/// The player's hits go through `world::crime::assault` instead, and the
/// player as victim isn't decided here.
// Translated from 008987f0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn attacked_by(
    order: &LoadOrder,
    state: &GameState,
    victim: FormId,
    attacker: FormId,
) -> Attacked {
    let none = Attacked::default();
    if victim == attacker || attacker == PLAYER_REF || victim == PLAYER_REF {
        return none;
    }
    if state.dead.contains(&victim) || crate::more_functions::is_ghost(state, victim) {
        return none;
    }
    let fighting = state.combat.contains_key(&victim);
    let their_target = |t: FormId| {
        state.combat.get(&victim) == Some(&t)
            || state
                .hit_targets
                .get(&victim)
                .is_some_and(|l| l.contains(&t))
    };
    let already = fighting && their_target(attacker);
    let follows_player = state.teammates.contains(&victim);
    let aimed = state.combat.get(&attacker) == Some(&victim)
        || (follows_player && state.combat.get(&attacker) == Some(&PLAYER_REF));
    // `CanAttackActor` (`008b0670`), the victim not the player here.
    let can = follows_player || factions::fights_when_hit(order, state, victim, attacker);
    let enters_combat = !already && can && aimed;
    // `0097f580`: run when the victim has a combat controller (now).
    let new_target = (fighting || enters_combat)
        && !already
        && factions::fights_when_hit(order, state, victim, attacker);
    Attacked {
        enters_combat,
        new_target,
    }
}

/// How sure of itself someone must be to stand (`011c52a8`, filled by
/// `0047ed10`), by Confidence: `fConfidenceCowardly` (1000),
/// `…Cautious` (data 0.375), `…Average` (0.1875), `…Brave` (0.0375),
/// `…Foolhardy` (0).
pub fn confidence_threshold(confidence: u8, s: Setting) -> f32 {
    match confidence {
        0 => s("fConfidenceCowardly", 1000.0),
        1 => s("fConfidenceCautious", 2.0),
        2 => s("fConfidenceAverage", 1.0),
        3 => s("fConfidenceBrave", 0.5),
        _ => s("fConfidenceFoolhardy", 0.0),
    }
}

/// A fighter's strength (`008acbe0`): attack power × health ÷ (1 −
/// min(armour ÷ 30, 0.99)).
pub fn strength(attack_power: f32, health: f32, armour: f32) -> f32 {
    attack_power * health / (1.0 - (armour / 30.0).min(0.99))
}

/// Whether an unaggressive actor runs from someone who would attack it
/// (`008ff350`): its confidence threshold above its strength ÷ theirs.
pub fn flees(threshold: f32, mine: f32, theirs: f32) -> bool {
    theirs > 0.0 && threshold > mine / theirs
}

/// Someone's attack power for [`strength`] (`008be600`): taken as the
/// damage per second of what they fight with (inferred, from `00645380`):
/// a weapon's damage × projectiles ÷ its attack time (1 ÷ attack shots a
/// second; automatic weapons × 2 for the pauses between bursts), unarmed ×
/// `fUnarmedNPCDPSMult` (1.57); a creature's bite once a second (a guess).
pub fn attack_power(order: &LoadOrder, state: &GameState, who: FormId, s: Setting) -> f32 {
    if let Some(d) = crate::combat::creature_damage(order, who) {
        if crate::combat::weapon_in_hand(order, state, who).is_none() {
            return d.max(0.0);
        }
    }
    let weapon = crate::combat::weapon_in_hand(order, state, who);
    let damage = crate::combat::weapon_damage(order, state, who, weapon.as_ref(), false);
    match &weapon {
        Some(w) => {
            let count = f32::from(w.projectiles.max(1));
            let mut time = w.shot_interval();
            if w.is_automatic() {
                time *= 2.0;
            }
            damage * count / time
        }
        None => damage * s("fUnarmedNPCDPSMult", 1.57),
    }
}

/// Whether `who` (unaggressive, aggression 0) runs from `threat`, which
/// would attack it (`008ff350`): [`flees`] with its confidence threshold
/// and both [`strength`]s (health now, worn armour's resistance).
pub fn flees_on_sight(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    threat: FormId,
    s: Setting,
) -> bool {
    if aggression(order, state, who) != 0
        || state.dead.contains(&who)
        || !factions::attacks_on_sight(order, state, threat, who)
    {
        return false;
    }
    let power = |r: FormId| {
        let health = crate::combat::health(order, state, r)
            .unwrap_or(0.0)
            .max(0.0) as f32;
        let armour = crate::combat::damage_resistance(order, state, r);
        strength(attack_power(order, state, r, s), health, armour)
    };
    flees(
        confidence_threshold(confidence(order, state, who), s),
        power(who),
        power(threat),
    )
}

/// A detection value between two actors as the viewer measures it: the
/// line of sight from the caller, the target's movement (`moving`,
/// `running`) for people other than the player (the player's from the
/// state); with the quick answers (`008e40d0`, `008a0d10`): nearer than 2
/// units 100 (others than the player).
pub fn detection_value(
    facts: &Facts,
    who: FormId,
    other: FormId,
    line_of_sight: bool,
    motion: Option<(bool, bool)>,
    s: Setting,
) -> Option<i32> {
    if crate::companions::hidden_with_player(facts.state, other) {
        return Some(crate::companions::HIDDEN_WITH_PLAYER);
    }
    let inputs = facts.detection_inputs(who, other, line_of_sight, motion)?;
    if other != PLAYER_REF && inputs.distance < 2.0 {
        return Some(100);
    }
    Some(crate::detection::value_with(&inputs, s))
}

// ---------------------------------------------------------------------------
// Moving
// ---------------------------------------------------------------------------

/// How fast a fighter moves (`00981570`, table `011a4884`): walking,
/// "fast walk" (walking × 1.5: strafes, dodges, the last of a melee
/// approach), running (approaches and long moves; their run speed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gait {
    Walk,
    FastWalk,
    Run,
}

impl Gait {
    /// The speed for walking and running speeds.
    pub fn speed(self, walk: f32, run: f32) -> f32 {
        match self {
            Gait::Walk => walk,
            Gait::FastWalk => walk * 1.5,
            Gait::Run => run,
        }
    }
}

// ---------------------------------------------------------------------------
// Ranged fighting
// ---------------------------------------------------------------------------

/// Where a gunman wants to be (`009a9180`): no nearer than `min`, no
/// farther than `optimal`, and never beyond `absolute_max`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Band {
    pub min: f32,
    pub optimal: f32,
    pub absolute_max: f32,
}

/// The band for a weapon (and how far its projectile carries,
/// `world::combat::projectile_reach`) or none (`009a9180`): the weapon's
/// `DNAM` min and max range × the style's range multipliers (unless the
/// weapon's "range fixed" flag, 0x20 in the second set); the absolute
/// maximum the optimal × `fCombatAbsoluteMaxRangeMult` (data 4); with a
/// projectile the optimal at most its reach × `fCombatProjectileMaxRange
/// OptimalMult` (0.85), the absolute maximum at most its reach, and the
/// minimum at most the optimal − 256 (not below 0). No weapon: 512 and 1024
/// × the multipliers. For a projectile that explodes see
/// [`ranged_band_blast`].
pub fn ranged_band(
    weapon: Option<(&Weapon, Option<f32>)>,
    style: &CombatStyle,
    s: Setting,
) -> Band {
    ranged_band_blast(weapon, None, style, s)
}

/// [`ranged_band`] for a weapon whose projectile explodes with `blast`
/// units of radius (`world::explosions::blast_radius`): the minimum is at
/// least the radius before it's held under the optimal − 256 (`009a9180`).
pub fn ranged_band_blast(
    weapon: Option<(&Weapon, Option<f32>)>,
    blast: Option<f32>,
    style: &CombatStyle,
    s: Setting,
) -> Band {
    let abs_mult = s("fCombatAbsoluteMaxRangeMult", 1.5);
    let Some((w, reach)) = weapon else {
        let optimal = 1024.0 * style.range_mult_max;
        return Band {
            min: 512.0 * style.range_mult_min,
            optimal,
            absolute_max: optimal * abs_mult,
        };
    };
    let (mut min, mut optimal) = (w.min_range, w.max_range);
    if w.flags2 & 0x20 == 0 {
        min *= style.range_mult_min;
        optimal *= style.range_mult_max;
    }
    let mut absolute_max = optimal * abs_mult;
    if let Some(r) = reach {
        optimal = optimal.min(r * s("fCombatProjectileMaxRangeOptimalMult", 0.85));
        absolute_max = absolute_max.min(r);
        if let Some(b) = blast {
            min = min.max(b);
        }
        min = min.min(optimal - 256.0).max(0.0);
    }
    Band {
        min,
        optimal,
        absolute_max,
    }
}

/// Whether a target this far off to the side and up or down (radians) is
/// within a weapon's aim arc (`009d1de0` → `009a6d70`): the arc is the
/// weapon's `DNAM` aim arc but at least 15°, across, so ±7.5° on the
/// vanilla guns, both ways.
pub fn within_aim_arc(aim_arc_degrees: f32, yaw_off: f32, pitch_off: f32) -> bool {
    let half = aim_arc_degrees.max(15.0).to_radians() * 0.5;
    yaw_off.abs() <= half && pitch_off.abs() <= half
}

/// Whether a target this far off to the side (radians) is within the
/// style's max targeting field of view (`CSSD` 48, degrees across; 0 no
/// limit). Outside it the ranged attack gives up ("Target outside
/// targeting FOV", `009d0a30`).
pub fn within_targeting_fov(style: &CombatStyle, yaw_off: f32) -> bool {
    style.max_targeting_fov <= 0.0
        || yaw_off.abs()
            <= style
                .max_targeting_fov
                .to_radians()
                .min(std::f32::consts::TAU)
                * 0.5
}

/// The pause a semi-automatic shot starts before the next (`009d0a30`
/// after starting an attack): between the weapon's semi-auto delay min ×
/// the style's min multiplier and its max × the style's max, `unit` (0..1)
/// of the way (`00476b70`). 0 to 0.3 s for the vanilla guns.
pub fn semi_auto_delay(weapon: &Weapon, style: &CombatStyle, unit: f32) -> f32 {
    let lo = weapon.semi_auto_delay.0 * style.semi_auto_delay_mult.0;
    let hi = weapon.semi_auto_delay.1 * style.semi_auto_delay_mult.1;
    lo + (hi - lo) * unit
}

/// What a ranged attack is doing (`CombatProcedureAttackRanged`): firing,
/// or (automatic weapons) pausing between bursts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FireState {
    Idle,
    Firing,
    Cooldown,
}

/// A ranged attack's timers (`009d0a30`): the delay between shots
/// (+0x38), the burst (+0x28) and the cooldown (+0x30).
#[derive(Debug, Clone, PartialEq)]
pub struct RangedAttack {
    pub state: FireState,
    /// When the delay after the last shot runs out.
    pub delay_until: f32,
    /// When the attack under way ends (its animation).
    pub attack_until: f32,
    /// When the burst, and the pause after it, end.
    pub burst_until: f32,
    pub cooldown_until: f32,
}

impl Default for RangedAttack {
    fn default() -> Self {
        RangedAttack {
            state: FireState::Idle,
            delay_until: f32::NEG_INFINITY,
            attack_until: f32::NEG_INFINITY,
            burst_until: f32::NEG_INFINITY,
            cooldown_until: f32::NEG_INFINITY,
        }
    }
}

impl RangedAttack {
    /// Whether to shoot at `now`. `aimed`: the target is within the aim arc
    /// (or nearer than the band's minimum) with a clear line;
    /// `attack_seconds` how long the attack lasts; `unit` a random 0..1 for
    /// the delay.
    ///
    /// Semi-automatic weapons shoot once the attack under way has ended and
    /// the delay it started ([`semi_auto_delay`], counted from the shot) has
    /// run out. Automatic ones (`Weapon::is_automatic`) hold the trigger
    /// for `fAutomaticWeaponBurstFireTime` (1 s), then pause
    /// `fAutomaticWeaponBurstCooldownTime` (1 s), and so on; within a burst
    /// the weapon's fire rate (`DNAM` 64) spaces the shots (inferred).
    /// (Some attack animations lengthen the burst by the time between their
    /// first two keys: not done.)
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        now: f32,
        weapon: &Weapon,
        style: &CombatStyle,
        aimed: bool,
        attack_seconds: f32,
        unit: f32,
        s: Setting,
    ) -> bool {
        if weapon.is_automatic() {
            let burst = s("fAutomaticWeaponBurstFireTime", 1.0);
            match self.state {
                FireState::Idle => {
                    self.state = FireState::Firing;
                    self.burst_until = now + burst;
                }
                FireState::Firing if now >= self.burst_until => {
                    self.state = FireState::Cooldown;
                    self.cooldown_until = now + s("fAutomaticWeaponBurstCooldownTime", 1.0);
                }
                FireState::Cooldown if now >= self.cooldown_until => {
                    self.state = FireState::Firing;
                    self.burst_until = now + burst;
                }
                _ => {}
            }
            if self.state != FireState::Firing || !aimed || now < self.delay_until {
                return false;
            }
            let rate = if weapon.fire_rate > 0.0 {
                weapon.fire_rate
            } else {
                weapon.shots_per_second
            };
            self.delay_until = now + 1.0 / rate.max(0.1);
            self.attack_until = now + attack_seconds;
            return true;
        }
        self.state = FireState::Firing;
        if !aimed || now < self.delay_until || now < self.attack_until {
            return false;
        }
        self.attack_until = now + attack_seconds;
        self.delay_until = now + semi_auto_delay(weapon, style, unit);
        true
    }
}

/// A gunman's next move (`CombatProcedureEngageTarget`, `009d3b80`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EngageMove {
    /// Nothing new.
    Stay,
    /// Sidestep this far, fast-walking (`left` which way: a coin toss here;
    /// the game's choice isn't traced).
    Strafe { distance: f32, left: bool },
    /// To a spot inside the band (the game searches the navmesh for one,
    /// `009d5000`), running when beyond the absolute maximum, else
    /// fast-walking.
    ToBand { run: bool },
    /// A step this far toward the target (`closer`) or away, fast-walking.
    Step { distance: f32, closer: bool },
    /// Beyond the absolute maximum: run straight at the target until within
    /// 128 units.
    RunAt,
    /// The target out of sight for half a second within 512 units: go to
    /// it (fast-walking).
    Approach,
}

/// A gunman keeping its distance (`009d3b80`): after a first wait of
/// 0.1–0.5 s, out of the band for 2 s (0.1 s beyond the absolute maximum
/// when too far) they move — to a spot in the band, else a 128–256 unit
/// step (else, beyond the absolute maximum, straight at the target); inside
/// it, whenever a 2–5 s timer runs out, 75% of the time they strafe 64–192
/// units sideways (128–384 with the target in sight), otherwise they'd
/// crouch or stand (crouching isn't done here: they stay). The timer starts
/// again when a move ends ([`Engage::arrived`]).
#[derive(Debug, Clone, PartialEq)]
pub struct Engage {
    started: bool,
    /// When the procedure's timer runs out.
    timer_until: f32,
    /// Since when they've been outside the band.
    out_since: Option<f32>,
    /// When the last move to a spot in the band began (the game tries that
    /// search at most once in 5 s).
    searched_at: f32,
    /// A move under way: no new decisions until it ends.
    pub moving: bool,
}

impl Default for Engage {
    fn default() -> Self {
        Engage {
            started: false,
            timer_until: f32::NEG_INFINITY,
            out_since: None,
            searched_at: f32::NEG_INFINITY,
            moving: false,
        }
    }
}

impl Engage {
    /// The decision at `now`, `distance` from the target, with `unseen_for`
    /// seconds since it was last seen; `random` gives numbers 0..1.
    pub fn update(
        &mut self,
        now: f32,
        distance: f32,
        band: &Band,
        in_sight: bool,
        unseen_for: f32,
        random: &mut dyn FnMut() -> f32,
    ) -> EngageMove {
        let between = |a: f32, b: f32, u: f32| a + (b - a) * u;
        if !self.started {
            self.started = true;
            self.timer_until = now + between(0.1, 0.5, random());
            return EngageMove::Stay;
        }
        let far = distance > band.optimal;
        let close = distance < band.min;
        let beyond = distance > band.absolute_max;
        if far || close {
            self.out_since.get_or_insert(now);
        } else {
            self.out_since = None;
        }
        if self.moving {
            return EngageMove::Stay;
        }
        if unseen_for > 0.5 && distance < 512.0 {
            self.moving = true;
            return EngageMove::Approach;
        }
        let expired = now >= self.timer_until;
        if far || close {
            let limit = if far && beyond { 0.1 } else { 2.0 };
            let out_for = now - self.out_since.unwrap_or(now);
            if expired || out_for > limit {
                self.moving = true;
                if now - self.searched_at >= 5.0 {
                    self.searched_at = now;
                    return EngageMove::ToBand { run: beyond };
                }
                if !beyond {
                    return EngageMove::Step {
                        distance: between(128.0, 256.0, random()),
                        closer: far,
                    };
                }
                return EngageMove::RunAt;
            }
        }
        if !expired {
            return EngageMove::Stay;
        }
        // `004dff00(0.25)` failing: three times in four.
        if random() >= 0.25 {
            self.moving = true;
            let distance = if in_sight {
                between(128.0, 384.0, random())
            } else {
                between(64.0, 192.0, random())
            };
            return EngageMove::Strafe {
                distance,
                left: random() < 0.5,
            };
        }
        self.timer_until = now + between(2.0, 5.0, random());
        EngageMove::Stay
    }

    /// A move has ended (arrived, or couldn't go): the 2–5 s timer starts
    /// again.
    pub fn arrived(&mut self, now: f32, unit: f32) {
        self.moving = false;
        self.timer_until = now + 2.0 + 3.0 * unit;
    }
}

// ---------------------------------------------------------------------------
// Melee
// ---------------------------------------------------------------------------

/// How far a melee attack reaches between the bodies' edges (`009a69c0`):
/// a melee weapon (animation types 0–2) its `DNAM` reach ×
/// `fCombatDistance` (128; the machete's 0.5 gives 64), a gun swung 128;
/// without a weapon a creature its `RNAM` (giants, type 7, ×
/// `fCombatGiantCreatureReachMult` 2), a person `fHandReachMult` (0.5) ×
/// 128 = 64; all × the actor's scale.
pub fn melee_reach(
    weapon: Option<&Weapon>,
    creature: Option<(f32, u8)>,
    scale: f32,
    s: Setting,
) -> f32 {
    let distance = s("fCombatDistance", 128.0);
    let reach = match (weapon, creature) {
        (Some(w), _) if w.is_melee() => w.reach * distance,
        (Some(_), _) => distance,
        (None, Some((rnam, kind))) => {
            rnam * if kind == 7 {
                s("fCombatGiantCreatureReachMult", 2.0)
            } else {
                1.0
            }
        }
        (None, None) => s("fHandReachMult", 0.5) * distance,
    };
    reach * scale
}

/// The gap between two bodies (`009a64d0`): the distance between them
/// less both collision radii (people [`PERSON_RADIUS`]).
pub fn gap(distance: f32, radius: f32, other_radius: f32) -> f32 {
    distance - (radius + other_radius)
}

/// A creature's collision radius from its skeleton's bound (`BSBound`
/// half extents) and scale: taken as people's is, the mean of the two
/// horizontal half extents (a guess: the game builds the controller from
/// the bound, `00c55170`, but how it gets the radius isn't traced).
pub fn creature_radius(half_extents: [f32; 3], scale: f32) -> f32 {
    (half_extents[0] + half_extents[1]) * 0.5 * scale
}

/// How a melee fighter closes in (`009cd430` and kin).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approach {
    /// Run straight at the target.
    Run,
    /// Fast-walk to a point within reach.
    FastWalk,
    /// Within reach: fight.
    InReach,
}

/// The approach for a gap and reach: within reach when the gap is less
/// (`009cefd0`); running while the gap is more than reach +
/// `fCombatMinEngageDistance` (64), fast-walking the rest.
pub fn melee_approach(gap: f32, reach: f32, s: Setting) -> Approach {
    if gap < reach {
        Approach::InReach
    } else if gap - reach > s("fCombatMinEngageDistance", 64.0) {
        Approach::Run
    } else {
        Approach::FastWalk
    }
}

/// What a melee fighter's choice depends on (`009cc510`).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MeleeSituation {
    /// The fighter's skill: the weapon's skill (`DNAM` 104), Unarmed
    /// without one; a creature's combat skill ([`melee_skill`]).
    pub skill: f32,
    /// The target attacking now; the fighter blocking now.
    pub target_attacking: bool,
    pub blocking: bool,
    /// The target recoiling or staggered, or unconscious.
    pub target_recoiling: bool,
    pub target_unconscious: bool,
    /// The fighter has no weapon.
    pub unarmed: bool,
    /// The fighter has a block animation (without one the block score is 0).
    pub can_block: bool,
    /// The target is fighting, and is the player.
    pub target_in_combat: bool,
    pub target_is_player: bool,
    /// Already holding (no new hold).
    pub holding: bool,
}

/// The three scores a melee choice is rolled from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeleeScores {
    pub attack: f32,
    pub block: f32,
    pub hold: f32,
}

/// The melee scores (`009cc510`, `00644680`, `00644580`):
///
/// - attack = (attack chance + recoiling × `CSTD` 40 + unconscious × 44 +
///   unarmed × 48 + `CSAD` 14 × skill / 100 + `CSAD` 15) × (`CSAD` 16 if the
///   target is attacking, else 17) × (`CSAD` 18 while blocking), + 100 when
///   the target isn't fighting and isn't the player (a per-target timer past
///   5 s adds 100 too: which timer isn't identified, not done);
/// - block (with a block animation) = (block chance + `CSAD` 11 + `CSAD` 10
///   × 0.5) × (`CSAD` 12 if the target is attacking, else 13) ×
///   `fBlockScoreNoShieldMult` (0.5) × (1 if attacking, else 0.25);
/// - hold = max(100 − (attack + block), `fCombatMaxHoldScore` (30; the
///   name is the findings', the setting sits at `011cea38`)), 0 while
///   already holding.
pub fn melee_scores(style: &CombatStyle, m: &MeleeSituation, s: Setting) -> MeleeScores {
    use advanced::*;
    let a = &style.advanced;
    let on = |b: bool, v: f32| if b { v } else { 0.0 };
    let mut attack = (f32::from(style.attack_chance as i8)
        + on(m.target_recoiling, style.recoil_attack_bonus)
        + on(m.target_unconscious, style.unconscious_attack_bonus)
        + on(m.unarmed, style.hand_to_hand_attack_bonus)
        + a[ATTACK_SKILL_MULT] * (m.skill / 100.0)
        + a[ATTACK_SKILL_BASE])
        * if m.target_attacking {
            a[ATTACK_UNDER_ATTACK]
        } else {
            a[ATTACK_NOT_UNDER_ATTACK]
        }
        * if m.blocking {
            a[ATTACK_DURING_BLOCK]
        } else {
            1.0
        };
    if !m.target_in_combat && !m.target_is_player {
        attack += 100.0;
    }
    let block = if m.can_block {
        (f32::from(style.block_chance as i8) + a[BLOCK_SKILL_BASE] + a[BLOCK_SKILL_MULT] * 0.5)
            * if m.target_attacking {
                a[BLOCK_UNDER_ATTACK]
            } else {
                a[BLOCK_NOT_UNDER_ATTACK]
            }
            * s("fBlockScoreNoShieldMult", 0.5)
            * if m.target_attacking { 1.0 } else { 0.25 }
    } else {
        0.0
    };
    let hold = if m.holding {
        0.0
    } else {
        (100.0 - (attack + block)).max(s("fCombatMaxHoldScore", 30.0))
    };
    MeleeScores {
        attack,
        block,
        hold,
    }
}

/// What a melee fighter does next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeleeChoice {
    Attack,
    Block,
    Hold,
    /// Already holding: carry on.
    Nothing,
}

/// The roll (`009cc510`): `roll` % trunc(attack + block + hold) (100 when
/// that's below 1); below the attack score attack, then below attack +
/// block block, else hold unless already holding.
pub fn melee_choice(scores: &MeleeScores, roll: u32, holding: bool) -> MeleeChoice {
    let total = (scores.attack + scores.block + scores.hold) as i32;
    let n = if total < 1 { 100 } else { total as u32 };
    let r = (roll % n) as f32;
    if r < scores.attack {
        MeleeChoice::Attack
    } else if scores.block > 0.0 && r < scores.attack + scores.block {
        MeleeChoice::Block
    } else if !holding {
        MeleeChoice::Hold
    } else {
        MeleeChoice::Nothing
    }
}

/// How long a hold lasts (`006465f0`): between the style's hold timer min
/// and max (`CSTD` 72, 76; 0.5–1.5 s by default), `unit` (0..1) of the way.
pub fn hold_seconds(style: &CombatStyle, unit: f32) -> f32 {
    let (a, b) = style.hold_timer;
    let (lo, hi) = (a.min(b), a.max(b));
    lo + (hi - lo) * unit
}

/// Whether an attack is a power attack (`00644810`): the style's power
/// attack chance (`CSTD` 52; 0 never, 100 or more always) + recoiling ×
/// `CSTD` 56 + unconscious × 60 + `CSAD` 19 + `CSAD` 20 × (1 − fatigue ÷
/// most fatigue) at least `roll` % 100.
pub fn power_attack(
    style: &CombatStyle,
    target_recoiling: bool,
    target_unconscious: bool,
    fatigue_share: f32,
    roll: u32,
) -> bool {
    use advanced::*;
    let chance = f32::from(style.power_attack_chance as i8);
    if chance == 0.0 {
        return false;
    }
    if chance >= 100.0 {
        return true;
    }
    let a = &style.advanced;
    let sum = chance
        + if target_recoiling {
            style.recoil_power_attack_bonus
        } else {
            0.0
        }
        + if target_unconscious {
            style.unconscious_power_attack_bonus
        } else {
            0.0
        }
        + a[POWER_ATTACK_FATIGUE_BASE]
        + a[POWER_ATTACK_FATIGUE_MULT] * (1.0 - fatigue_share);
    sum >= (roll % 100) as f32
}

/// A fighter's skill for the attack score (`009cc510`): a person's
/// weapon skill (`DNAM` 104), or Unarmed (45) without a weapon; a
/// creature's combat skill (`CREA` `DATA` byte 1; actor value 45 for
/// creatures, `0097fc70`), unless a script set it.
pub fn melee_skill(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    weapon: Option<&Weapon>,
) -> f32 {
    let creature = person(order, who).filter(|&b| {
        order
            .get(b)
            .is_some_and(|r| r.entry.header.kind.as_bytes() == b"CREA")
    });
    if let Some(base) = creature {
        if let Some(&v) = state.actor_values.get(&(who, 45)) {
            return v as f32;
        }
        return crate::actor::data_record(order, base, crate::actor::USE_STATS)
            .and_then(|(_, r)| r.get(esm::sig::DATA).and_then(|s| s.data.get(1).copied()))
            .map_or(0.0, f32::from);
    }
    let av = weapon.map_or(crate::combat::av::UNARMED, |w| w.skill);
    Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, av)
    .unwrap_or(0.0) as f32
}

// ---------------------------------------------------------------------------
// Losing the target
// ---------------------------------------------------------------------------

/// What a fighter knows of its target (`00987220`, `0098a580`): when it
/// was last seen (detected above 0), how often, and where.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetMemory {
    pub target: FormId,
    pub last_seen: f32,
    pub times_seen: u32,
    pub last_known: [f32; 3],
}

impl TargetMemory {
    /// A fight starting at `now` with the target at `at`.
    pub fn new(target: FormId, now: f32, at: [f32; 3]) -> TargetMemory {
        TargetMemory {
            target,
            last_seen: now,
            times_seen: 0,
            last_known: at,
        }
    }

    /// The target seen (its detection value above 0) at `now`, at `at`.
    pub fn saw(&mut self, now: f32, at: [f32; 3]) {
        self.last_seen = now;
        self.times_seen += 1;
        self.last_known = at;
    }

    /// Seconds since it was last seen.
    pub fn unseen_for(&self, now: f32) -> f32 {
        (now - self.last_seen).max(0.0)
    }

    /// Searching: unseen for `fCombatDetectionLostTime` (15 s; `0098add0`).
    pub fn searching(&self, now: f32, s: Setting) -> bool {
        self.unseen_for(now) >= s("fCombatDetectionLostTime", 15.0)
    }

    /// Giving up (`00987220`): unseen past `fCombatTargetLostRemoveTime`
    /// (30 s) and never seen; or past `…RemoveDistanceTime` (60 s) with the
    /// fighter farther than `…RemoveDistance` (4096) from it; or the target
    /// dead. (The search-count and hidden-target rules aren't done.)
    pub fn gives_up(&self, now: f32, distance: f32, target_dead: bool, s: Setting) -> bool {
        if target_dead {
            return true;
        }
        let unseen = self.unseen_for(now);
        if unseen <= s("fCombatTargetLostRemoveTime", 30.0) {
            return false;
        }
        if self.times_seen == 0 {
            return true;
        }
        unseen > s("fCombatTargetLostRemoveDistanceTime", 60.0)
            && distance > s("fCombatTargetLostRemoveDistance", 4096.0)
    }
}

/// How far around the last known spot a search looks (the smallest search
/// area: `fCombatSearchExteriorMinRadius` data 512, `…InteriorMinRadius`
/// data 256; the search's own choice of areas and spots isn't done).
pub fn search_radius(outdoors: bool, s: Setting) -> f32 {
    if outdoors {
        s("fCombatSearchExteriorMinRadius", 1024.0)
    } else {
        s("fCombatSearchInteriorMinRadius", 512.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exe(_: &str, default: f32) -> f32 {
        default
    }

    /// `FalloutNV.esm`'s values where it sets them.
    fn data(name: &str, default: f32) -> f32 {
        match name {
            "fCombatAbsoluteMaxRangeMult" => 4.0,
            "fConfidenceCautious" => 0.375,
            "fConfidenceAverage" => 0.1875,
            "fConfidenceBrave" => 0.0375,
            "fCombatSearchExteriorMinRadius" => 512.0,
            "fCombatSearchInteriorMinRadius" => 256.0,
            _ => default,
        }
    }

    fn pistol() -> Weapon {
        Weapon {
            form_id: FormId(1),
            name: "9mm".into(),
            damage: 16.0,
            clip: 13,
            health: 150,
            animation: 3,
            ammo: Vec::new(),
            ammo_use: 1,
            min_spread: 0.7,
            spread: 0.0,
            projectile: None,
            projectiles: 1,
            min_range: 256.0,
            max_range: 768.0,
            shots_per_second: 3.125,
            reload_time: 1.67,
            skill: 41,
            crit_damage: 16.0,
            crit_mult: 1.0,
            sound: None,
            attack_animation: 32,
            reload_animation: 11,
            kill_impulse: 4.0,
            impulse_distance: 0.0,
            reach: 0.0,
            limb_damage_mult: 1.0,
            flags1: 0x04,
            flags2: 0x2008,
            fire_rate: 1.0,
            attack_mult: 1.25,
            aim_arc: 0.0,
            semi_auto_delay: (0.0, 0.3),
            speed: 1.0,
            cone_mult: 1.0,
            crit_effect: None,
            crit_on_death: false,
            resist: None,
        }
    }

    #[test]
    fn the_9mm_pistols_band_is_the_findings() {
        // 256 / 768 with a 10000-unit bullet: min 256, optimal 768, the
        // absolute maximum 768 × 4 = 3072.
        let style = CombatStyle::default();
        let b = ranged_band(Some((&pistol(), Some(10000.0))), &style, &data);
        assert_eq!(
            b,
            Band {
                min: 256.0,
                optimal: 768.0,
                absolute_max: 3072.0
            }
        );
        // The style's multipliers stretch it, unless the range is fixed.
        let wide = CombatStyle {
            range_mult_min: 0.5,
            range_mult_max: 2.0,
            ..CombatStyle::default()
        };
        let b = ranged_band(Some((&pistol(), Some(10000.0))), &wide, &data);
        assert_eq!((b.min, b.optimal, b.absolute_max), (128.0, 1536.0, 6144.0));
        let fixed = Weapon {
            flags2: 0x20,
            ..pistol()
        };
        let b = ranged_band(Some((&fixed, Some(10000.0))), &wide, &data);
        assert_eq!((b.min, b.optimal), (256.0, 768.0));
        // A short projectile caps both, and the minimum stays 256 under the
        // optimal.
        let b = ranged_band(Some((&pistol(), Some(600.0))), &style, &data);
        assert_eq!((b.min, b.optimal, b.absolute_max), (254.0, 510.0, 600.0));
        // No weapon: 512 / 1024.
        let b = ranged_band(None, &style, &exe);
        assert_eq!((b.min, b.optimal, b.absolute_max), (512.0, 1024.0, 1536.0));
    }

    #[test]
    fn dynamite_keeps_its_thrower_out_of_the_blast() {
        // `WeapNVDynamite`'s 500 / 1024, its projectile's reach 1200² ÷
        // 686.61 (2097), its blast 750: min 750, optimal 1024.
        let dynamite = Weapon {
            min_range: 500.0,
            max_range: 1024.0,
            animation: 10,
            ..pistol()
        };
        let style = CombatStyle::default();
        let reach = Some(1200.0f32 * 1200.0 / WORLD_GRAVITY);
        let b = ranged_band_blast(Some((&dynamite, reach)), Some(750.0), &style, &data);
        assert_eq!((b.min, b.optimal), (750.0, 1024.0));
        // Still under the optimal − 256.
        let b = ranged_band_blast(Some((&dynamite, reach)), Some(900.0), &style, &data);
        assert_eq!(b.min, 768.0);
        assert_eq!(
            ranged_band(Some((&dynamite, reach)), &style, &data).min,
            500.0
        );
    }

    #[test]
    fn the_aim_arc_is_at_least_fifteen_degrees_across() {
        let d = |x: f32| x.to_radians();
        assert!(within_aim_arc(0.0, d(7.4), d(-7.4)));
        assert!(!within_aim_arc(0.0, d(7.6), 0.0));
        assert!(!within_aim_arc(0.0, 0.0, d(-7.6)));
        assert!(within_aim_arc(40.0, d(19.0), 0.0));
        let style = CombatStyle {
            max_targeting_fov: 90.0,
            ..CombatStyle::default()
        };
        assert!(within_targeting_fov(&style, d(44.0)));
        assert!(!within_targeting_fov(&style, d(46.0)));
        assert!(within_targeting_fov(&CombatStyle::default(), d(170.0)));
    }

    #[test]
    fn semi_automatic_shots_wait_for_the_attack_and_the_delay() {
        let w = pistol();
        let style = CombatStyle {
            semi_auto_delay_mult: (1.0, 2.0),
            ..CombatStyle::default()
        };
        // Between 0 × 1 and 0.3 × 2.
        assert!((semi_auto_delay(&w, &style, 0.5) - 0.3).abs() < 1e-6);
        let mut a = RangedAttack::default();
        assert!(a.update(0.0, &w, &style, true, 0.32, 1.0, &exe));
        // The delay (0.6 s from the shot) outlasts the attack (0.32 s).
        assert!(!a.update(0.4, &w, &style, true, 0.32, 0.0, &exe));
        assert!(a.update(0.6, &w, &style, true, 0.32, 0.0, &exe));
        // No delay: the attack's end decides.
        assert!(!a.update(0.9, &w, &style, true, 0.32, 0.0, &exe));
        assert!(a.update(0.93, &w, &style, true, 0.32, 0.0, &exe));
        // Off target: nothing.
        assert!(!a.update(5.0, &w, &style, false, 0.32, 0.0, &exe));
    }

    #[test]
    fn automatic_weapons_fire_one_second_bursts_with_one_second_pauses() {
        // Eight shots a second, checked every 1/64 s (both exact in binary).
        let smg = Weapon {
            flags1: 0x02,
            fire_rate: 8.0,
            ..pistol()
        };
        let style = CombatStyle::default();
        let mut a = RangedAttack::default();
        let mut shots = Vec::new();
        for i in 0..=256 {
            let t = i as f32 / 64.0;
            if a.update(t, &smg, &style, true, 0.1, 0.5, &exe) {
                shots.push(t);
            }
        }
        let in_window = |a: f32, b: f32| shots.iter().filter(|t| **t >= a && **t < b).count();
        // Eight a second within bursts at 0–1 s and 2–3 s, none between.
        assert_eq!(in_window(0.0, 1.0), 8, "{shots:?}");
        assert_eq!(in_window(1.0, 2.0), 0, "{shots:?}");
        assert_eq!(in_window(2.0, 3.0), 8, "{shots:?}");
        // "Short burst" (0x200 in the second set) is automatic too.
        assert!(Weapon {
            flags2: 0x200,
            ..pistol()
        }
        .is_automatic());
        assert!(!pistol().is_automatic());
    }

    #[test]
    fn gunmen_strafe_in_the_band_and_move_when_out_of_it() {
        let band = Band {
            min: 256.0,
            optimal: 768.0,
            absolute_max: 3072.0,
        };
        let mut e = Engage::default();
        let mut next = [0.0f32, 0.9, 0.3, 0.7, 0.1, 0.0, 0.5].into_iter().cycle();
        let mut random = move || next.next().unwrap();
        // The first wait: 0.1 s with a 0.
        assert_eq!(
            e.update(0.0, 500.0, &band, true, 0.0, &mut random),
            EngageMove::Stay
        );
        // In the band once it runs out: 0.9 ≥ 0.25 strafes 128 + 256 × 0.3.
        match e.update(0.2, 500.0, &band, true, 0.0, &mut random) {
            EngageMove::Strafe { distance, .. } => assert!((distance - 204.8).abs() < 1e-3),
            m => panic!("{m:?}"),
        }
        assert_eq!(
            e.update(0.3, 500.0, &band, true, 0.0, &mut random),
            EngageMove::Stay
        );
        e.arrived(1.0, 0.0);
        // Out of the band (too far) for under 2 s: nothing yet.
        assert_eq!(
            e.update(1.5, 900.0, &band, true, 0.0, &mut random),
            EngageMove::Stay
        );
        assert_eq!(
            e.update(2.9, 900.0, &band, true, 0.0, &mut random),
            EngageMove::Stay
        );
        // Past 2 s: to a spot in the band, fast-walking.
        assert_eq!(
            e.update(3.6, 900.0, &band, true, 0.0, &mut random),
            EngageMove::ToBand { run: false }
        );
        e.arrived(4.0, 0.0);
        assert_eq!(
            e.update(4.05, 500.0, &band, true, 0.0, &mut random),
            EngageMove::Stay
        );
        // Too close again within 5 s of that search: a step back.
        assert_eq!(
            e.update(4.1, 100.0, &band, true, 0.0, &mut random),
            EngageMove::Stay
        );
        match e.update(6.2, 100.0, &band, true, 0.0, &mut random) {
            EngageMove::Step { closer, .. } => assert!(!closer),
            m => panic!("{m:?}"),
        }
        e.arrived(6.5, 0.0);
        // Beyond the absolute maximum: 0.1 s is enough, and they run.
        let mut far = Engage::default();
        far.update(0.0, 4000.0, &band, true, 0.0, &mut random);
        assert_eq!(
            far.update(0.05, 4000.0, &band, true, 0.0, &mut random),
            EngageMove::Stay
        );
        assert_eq!(
            far.update(0.2, 4000.0, &band, true, 0.0, &mut random),
            EngageMove::ToBand { run: true }
        );
        // Lost from sight nearby: they go to the target.
        let mut near = Engage::default();
        near.update(0.0, 300.0, &band, false, 0.0, &mut random);
        assert_eq!(
            near.update(1.0, 300.0, &band, false, 0.6, &mut random),
            EngageMove::Approach
        );
    }

    #[test]
    fn melee_reach_by_weapon_creature_and_hands() {
        let machete = Weapon {
            animation: 1,
            reach: 0.5,
            attack_mult: 1.3,
            ..pistol()
        };
        assert_eq!(melee_reach(Some(&machete), None, 1.0, &exe), 64.0);
        // Its swing: the 1.13 s animation at 1.3 ×; the pistol's attack 1
        // ÷ 3.125 a second.
        assert!((machete.attack_seconds(1.13) - 0.8692).abs() < 1e-3);
        assert!((pistol().attack_seconds(0.5) - 0.32).abs() < 1e-6);
        assert_eq!(melee_reach(Some(&pistol()), None, 1.0, &exe), 128.0);
        assert_eq!(melee_reach(None, None, 1.0, &exe), 64.0);
        // The gecko's RNAM 25; a giant (type 7) doubles; scale counts.
        assert_eq!(melee_reach(None, Some((25.0, 1)), 1.0, &exe), 25.0);
        assert_eq!(melee_reach(None, Some((25.0, 7)), 2.0, &exe), 100.0);
        // Running until within reach + 64, then a fast walk.
        assert_eq!(melee_approach(200.0, 25.0, &exe), Approach::Run);
        assert_eq!(melee_approach(89.0, 25.0, &exe), Approach::FastWalk);
        assert_eq!(melee_approach(24.0, 25.0, &exe), Approach::InReach);
        assert_eq!(gap(100.0, PERSON_RADIUS, PERSON_RADIUS), 59.5);
        assert_eq!(Gait::FastWalk.speed(85.0, 354.0), 127.5);
    }

    #[test]
    fn the_geckos_melee_scores_are_the_findings() {
        // `CSNVGecko`: attack 60, block 40, hold 0.1–0.35, the default
        // advanced values; combat skill 40.
        let gecko = CombatStyle {
            attack_chance: 60,
            block_chance: 40,
            hold_timer: (0.1, 0.35),
            ..CombatStyle::default()
        };
        let m = MeleeSituation {
            skill: 40.0,
            target_is_player: true,
            unarmed: false,
            ..MeleeSituation::default()
        };
        let s = melee_scores(&gecko, &m, &exe);
        assert!((s.attack - 68.0).abs() < 1e-4, "{s:?}");
        assert_eq!(s.block, 0.0);
        assert!((s.hold - 32.0).abs() < 1e-4);
        // The player attacking: × 0.75 = 51; hold 49.
        let attacked = MeleeSituation {
            target_attacking: true,
            ..m
        };
        let s2 = melee_scores(&gecko, &attacked, &exe);
        assert!((s2.attack - 51.0).abs() < 1e-4 && (s2.hold - 49.0).abs() < 1e-4);
        // Rolls: 0..67 attack, 68..99 hold.
        assert_eq!(melee_choice(&s, 67, false), MeleeChoice::Attack);
        assert_eq!(melee_choice(&s, 68, false), MeleeChoice::Hold);
        assert_eq!(melee_choice(&s, 167, false), MeleeChoice::Attack);
        assert_eq!(melee_choice(&s, 99, true), MeleeChoice::Nothing);
        assert!((hold_seconds(&gecko, 0.5) - 0.225).abs() < 1e-6);
        // With a block animation and the target attacking: (40 + 0 + 10) ×
        // 2 × 0.5 = 50.
        let blocking = MeleeSituation {
            can_block: true,
            ..attacked
        };
        let s3 = melee_scores(&gecko, &blocking, &exe);
        assert!((s3.block - 50.0).abs() < 1e-4, "{s3:?}");
        assert_eq!(melee_choice(&s3, 60, false), MeleeChoice::Block);
        // A target not fighting (not the player): +100.
        let calm = MeleeSituation {
            target_is_player: false,
            ..m
        };
        assert!((melee_scores(&gecko, &calm, &exe).attack - 168.0).abs() < 1e-4);
        // Power attacks: 25 + 5 − 10 × 0 at full fatigue = 30.
        assert!(power_attack(&gecko, false, false, 1.0, 29));
        assert!(power_attack(&gecko, false, false, 1.0, 30));
        assert!(!power_attack(&gecko, false, false, 1.0, 31));
    }

    #[test]
    fn styles_read_their_three_subrecords() {
        let mut cstd = vec![0u8; 92];
        cstd[0] = 5;
        cstd[36] = 40;
        cstd[37] = 60;
        cstd[52] = 25;
        cstd[72..76].copy_from_slice(&0.1f32.to_le_bytes());
        cstd[76..80].copy_from_slice(&0.35f32.to_le_bytes());
        cstd[80..82].copy_from_slice(&0x1E0u16.to_le_bytes());
        let mut csad = vec![0u8; 84];
        csad[14 * 4..15 * 4].copy_from_slice(&20.0f32.to_le_bytes());
        let mut cssd = vec![0u8; 64];
        cssd[32..36].copy_from_slice(&0.5f32.to_le_bytes());
        cssd[40..44].copy_from_slice(&2u32.to_le_bytes());
        cssd[44..48].copy_from_slice(&2.0f32.to_le_bytes());
        cssd[56..60].copy_from_slice(&1.5f32.to_le_bytes());
        cssd[60..64].copy_from_slice(&3.0f32.to_le_bytes());
        let s = CombatStyle::parse(&cstd, &csad, &cssd, 15);
        assert_eq!(
            (s.dodge_chance, s.block_chance, s.attack_chance),
            (5, 40, 60)
        );
        assert_eq!(s.power_attack_chance, 25);
        assert_eq!(s.hold_timer, (0.1, 0.35));
        assert_eq!(s.flags, 0x1E0);
        assert_eq!(s.advanced[advanced::ATTACK_SKILL_MULT], 20.0);
        assert_eq!((s.range_mult_min, s.range_mult_max), (0.5, 2.0));
        assert_eq!(s.weapon_restrictions, 2);
        assert_eq!(s.semi_auto_delay_mult, (1.5, 3.0));
        // An old record (form version 11) has both range multipliers 1.
        let old = CombatStyle::parse(&cstd, &csad, &cssd, 11);
        assert_eq!((old.range_mult_min, old.range_mult_max), (1.0, 1.0));
        // A short CSSD keeps the rest.
        let short = CombatStyle::parse(&cstd, &csad, &cssd[..40], 15);
        assert_eq!(short.range_mult_max, 1.0);
        assert_eq!(short.range_mult_min, 0.5);
    }

    #[test]
    fn detection_runs_more_often_in_combat() {
        assert!((detection_interval(true, 0.016, 50, 1.0, &exe) - 0.3).abs() < 1e-6);
        // 0.3 + 0.5 × min(0.016 × 50, 5) = 0.7.
        assert!((detection_interval(false, 0.016, 50, 0.5, &exe) - 0.7).abs() < 1e-6);
        // The stagger is at most 5 s.
        assert!((detection_interval(false, 1.0, 50, 1.0, &exe) - 5.3).abs() < 1e-6);
    }

    #[test]
    fn the_weak_and_unconfident_run() {
        assert_eq!(confidence_threshold(2, &data), 0.1875);
        assert_eq!(confidence_threshold(0, &data), 1000.0);
        // Half the armour cap: health and power over 0.5.
        assert!((strength(10.0, 100.0, 15.0) - 2000.0).abs() < 1e-3);
        assert!((strength(10.0, 100.0, 300.0) - 100_000.0).abs() < 1.0);
        // A tenth as strong: an average one stands (0.1 < 0.1875 runs).
        assert!(flees(0.1875, 10.0, 100.0));
        assert!(!flees(0.1875, 50.0, 100.0));
        assert!(!flees(0.0, 0.0, 100.0));
    }

    #[test]
    fn lost_targets_are_searched_for_then_given_up() {
        let mut m = TargetMemory::new(FormId(7), 0.0, [0.0; 3]);
        assert!(!m.searching(14.0, &exe));
        assert!(m.searching(15.0, &exe));
        // Never seen: given up after 30 s.
        assert!(!m.gives_up(30.0, 100.0, false, &exe));
        assert!(m.gives_up(30.5, 100.0, false, &exe));
        // Seen once: 60 s, and only from more than 4096 away.
        m.saw(1.0, [10.0, 0.0, 0.0]);
        assert!(!m.gives_up(50.0, 5000.0, false, &exe));
        assert!(!m.gives_up(62.0, 1000.0, false, &exe));
        assert!(m.gives_up(62.0, 5000.0, false, &exe));
        assert!(m.gives_up(2.0, 10.0, true, &exe));
        assert_eq!(search_radius(true, &data), 512.0);
        assert_eq!(search_radius(false, &data), 256.0);
    }
}
