//! Per-player shadow state.
//!
//! The player is not an object in `GameState`: it is the constant
//! [`PLAYER_REF`] and a handful of rows keyed by it (`items`, `actor_values`,
//! `perks`, `skill_points`...). To run a script or a dialogue result on behalf
//! of player N as if that player were `PLAYER_REF`, the host swaps player N's
//! rows into those slots, runs, and swaps the previous owner back.
//!
//! * [`PlayerShadow`]: the rows that belong to one player.
//! * [`ShadowPlan`]: which categories are per player (built from the
//!   `shadow_classes` of the policy table, or [`ShadowPlan::all`]).
//! * [`ShadowBank`]: the parked shadows of every player who is not the live
//!   owner of the `PLAYER_REF` rows. Lives in [`super::CoopState`].
//! * [`ShadowGuard`]: panic-safe swap (`GameState::as_player`).
//!
//! Shared stats are never swapped: karma, reputation, faction rank, quest
//! state (except quests the policy marks per player), globals, topics.
//!
//! Replication: with shadows enabled the shared export leaves the swapped
//! rows out and each player's shadow travels as a blob on a synthetic form
//! ([`shadow_form`]), chunked into `Field::Custom` pieces under tags
//! `0x0100..0x0F00`. Every peer exports every slot (the live owner is read
//! from the live rows, the others from the bank), so hashes converge and a
//! guest receives its own inventory and skills authoritatively from the host.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};

use coop_core::schema::{Field, Key};
use coop_policy::{PolicyTable, Residence, Scope, ShadowClass};
use esm::FormId;

use super::export::{cf, Fields};
use super::Domain;
use crate::dialogue::PLAYER_REF;
use crate::scripting::GameState;

/// Synthetic forms `SHADOW_FORM_BASE + slot` carry a player's shadow.
pub const SHADOW_FORM_BASE: u32 = 0xFC00_0000;
/// First custom tag used by shadow blob chunks.
pub const TAG_BASE: u16 = 0x0100;
/// One past the last tag.
pub const TAG_END: u16 = 0x0F00;
/// Bytes per chunk (well under `MAX_CUSTOM_BYTES`).
pub const CHUNK: usize = 480;
/// Wire format version, first byte of every blob.
const VERSION: u8 = 1;

/// The synthetic form that carries `slot`'s shadow.
pub fn shadow_form(slot: u8) -> FormId {
    FormId(SHADOW_FORM_BASE + u32::from(slot))
}

/// The slot a synthetic shadow form belongs to.
pub fn slot_of_shadow_form(f: FormId) -> Option<u8> {
    let d = f.0.wrapping_sub(SHADOW_FORM_BASE);
    (d < 8).then_some(d as u8)
}

/// Which categories of state are per player.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShadowPlan {
    /// SPECIAL, skills, skill points, tag skills.
    pub skills: bool,
    /// Experience points and level.
    pub level_xp: bool,
    /// Perks and ranks.
    pub perks: bool,
    /// Teammates (followers).
    pub followers: bool,
    /// Casino data.
    pub casino: bool,
    /// Quests each player runs separately (stage, done stages, flags,
    /// objectives).
    pub quests: BTreeSet<FormId>,
    /// Quest script variables (lower case names) that are per player.
    pub quest_vars: BTreeMap<FormId, Vec<String>>,
}

impl ShadowPlan {
    /// Every category per player, no per-player quests.
    pub fn all() -> ShadowPlan {
        ShadowPlan {
            skills: true,
            level_xp: true,
            perks: true,
            followers: true,
            casino: true,
            quests: BTreeSet::new(),
            quest_vars: BTreeMap::new(),
        }
    }

    /// Builds the plan from the policy table: residence of each shadow class
    /// and the per-player quest rows. Level and experience follow the
    /// `skills` class (a character's progression is one unit).
    pub fn from_policy(t: &PolicyTable) -> ShadowPlan {
        let pp = |c: ShadowClass| t.residence(c) == Residence::PerPlayer;
        let mut plan = ShadowPlan {
            skills: pp(ShadowClass::Skills),
            level_xp: pp(ShadowClass::Skills),
            perks: pp(ShadowClass::Perks),
            followers: pp(ShadowClass::Followers),
            casino: pp(ShadowClass::Casino),
            quests: BTreeSet::new(),
            quest_vars: BTreeMap::new(),
        };
        for row in t.rows() {
            let id = FormId(row.id);
            if row.scope == Scope::PerPlayer {
                plan.quests.insert(id);
            }
            if let Some(sh) = &row.shadow {
                if !sh.vars.is_empty() && pp(ShadowClass::QuestVar) {
                    let mut v: Vec<String> =
                        sh.vars.iter().map(|s| s.to_ascii_lowercase()).collect();
                    v.sort();
                    v.dedup();
                    plan.quest_vars.insert(id, v);
                }
            }
        }
        plan
    }

    /// True when the actor value row of the player belongs to a player.
    pub fn av_per_player(&self, id: u16) -> bool {
        if id == crate::reputation::KARMA {
            return false;
        }
        if id == crate::experience::XP {
            return self.level_xp;
        }
        if (5..=11).contains(&id) || (32..=45).contains(&id) {
            return self.skills;
        }
        true
    }
}

/// State of one player that is not replicated (host only, kept in memory).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LocalExtras {
    /// Caravan cards and record.
    pub caravan: crate::caravan::Collection,
    /// Casino data.
    pub casinos: Vec<crate::casino::CasinoData>,
    /// Effects working on the player, oldest first.
    pub effects: Vec<crate::magic::ActiveEffect>,
}

/// Every row that belongs to one player.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerShadow {
    /// Actor values (never karma).
    pub actor_values: BTreeMap<u16, f64>,
    /// Actor value damage (health, rads...).
    pub value_damage: BTreeMap<u16, f64>,
    /// Health lost.
    pub damage: Option<f64>,
    /// Dead.
    pub dead: bool,
    /// Items held (item to count).
    pub items: BTreeMap<FormId, i32>,
    /// The inventory was copied from the record (`stocked`).
    pub stocked: bool,
    /// Equipped items.
    pub equipped: Option<Vec<FormId>>,
    /// Item condition (f32 bits).
    pub weapon_health: BTreeMap<FormId, u32>,
    /// Fitted weapon mods.
    pub weapon_mods: BTreeMap<FormId, u8>,
    /// Weapons dropped.
    pub dropped: BTreeSet<FormId>,
    /// Equip-locked items.
    pub equip_locked: BTreeSet<FormId>,
    /// Chosen ammunition.
    pub ammo_loaded: Option<FormId>,
    /// Hot keys.
    pub hotkeys: [Option<FormId>; 8],
    /// Pip-Boy statistics.
    pub misc_stats: BTreeMap<u8, u32>,
    /// Skill points put into skills.
    pub skill_points: BTreeMap<u16, u32>,
    /// Tag skills.
    pub tag_skills: BTreeSet<u16>,
    /// Tag slots.
    pub tag_slots: BTreeMap<u8, u16>,
    /// Level.
    pub level: u16,
    /// A level-up is waiting.
    pub level_up_pending: bool,
    /// Perks.
    pub perks: BTreeSet<FormId>,
    /// Perk ranks past the first.
    pub perk_ranks: BTreeMap<FormId, u8>,
    /// Teammates.
    pub teammates: BTreeSet<FormId>,
    /// Where the player is.
    pub cell: Option<FormId>,
    /// Worldspace when outdoors.
    pub world: Option<FormId>,
    /// Feet position (f32 bits).
    pub position: Option<[u32; 3]>,
    /// Heading (f32 bits).
    pub heading: u32,
    /// Name.
    pub name: Option<String>,
    /// Sex.
    pub female: Option<bool>,
    /// Per-player quests: current stage.
    pub quest_stage: BTreeMap<FormId, u16>,
    /// Per-player quests: done stages.
    pub stages_done: BTreeSet<(FormId, u16)>,
    /// Per-player quests: flag bits (1 running, 2 completed, 4 failed, 8 announced).
    pub quest_flags: BTreeMap<FormId, u8>,
    /// Per-player quests: objectives (2 means hidden-completed).
    pub objectives: BTreeMap<(FormId, i32), u8>,
    /// Per-player quest variables.
    pub quest_vars: BTreeMap<(FormId, String), u64>,
    /// Not replicated.
    pub local: LocalExtras,
}

impl Default for PlayerShadow {
    fn default() -> Self {
        PlayerShadow::fresh()
    }
}

impl PlayerShadow {
    /// A player who has just joined: level 1, the record's starting kit.
    pub fn fresh() -> PlayerShadow {
        PlayerShadow {
            actor_values: BTreeMap::new(),
            value_damage: BTreeMap::new(),
            damage: None,
            dead: false,
            items: BTreeMap::new(),
            stocked: false,
            equipped: None,
            weapon_health: BTreeMap::new(),
            weapon_mods: BTreeMap::new(),
            dropped: BTreeSet::new(),
            equip_locked: BTreeSet::new(),
            ammo_loaded: None,
            hotkeys: [None; 8],
            misc_stats: BTreeMap::new(),
            skill_points: BTreeMap::new(),
            tag_skills: BTreeSet::new(),
            tag_slots: BTreeMap::new(),
            level: 1,
            level_up_pending: false,
            perks: BTreeSet::new(),
            perk_ranks: BTreeMap::new(),
            teammates: BTreeSet::new(),
            cell: None,
            world: None,
            position: None,
            heading: 0,
            name: None,
            female: None,
            quest_stage: BTreeMap::new(),
            stages_done: BTreeSet::new(),
            quest_flags: BTreeMap::new(),
            objectives: BTreeMap::new(),
            quest_vars: BTreeMap::new(),
            local: LocalExtras::default(),
        }
    }

    /// Copies the player's rows out of `gs` (nothing is removed).
    pub fn read(gs: &GameState, plan: &ShadowPlan) -> PlayerShadow {
        let me = |h: &FormId| *h == PLAYER_REF;
        let mut s = PlayerShadow::fresh();
        s.actor_values = gs
            .actor_values
            .iter()
            .filter(|((w, id), _)| me(w) && plan.av_per_player(*id))
            .map(|((_, id), v)| (*id, *v))
            .collect();
        s.value_damage = gs
            .value_damage
            .iter()
            .filter(|((w, _), _)| me(w))
            .map(|((_, id), v)| (*id, *v))
            .collect();
        s.damage = gs.damage.get(&PLAYER_REF).copied();
        s.dead = gs.dead.contains(&PLAYER_REF);
        s.items = gs
            .items
            .iter()
            .filter(|((h, _), _)| me(h))
            .map(|((_, i), n)| (*i, *n))
            .collect();
        s.stocked = gs.stocked.contains(&PLAYER_REF);
        s.equipped = gs.equipped.get(&PLAYER_REF).cloned();
        s.weapon_health = gs
            .weapon_health
            .iter()
            .filter(|((h, _), _)| me(h))
            .map(|((_, i), v)| (*i, v.to_bits()))
            .collect();
        s.weapon_mods = gs
            .weapon_mods
            .iter()
            .filter(|((h, _), _)| me(h))
            .map(|((_, i), v)| (*i, *v))
            .collect();
        s.dropped = gs
            .dropped
            .iter()
            .filter(|(h, _)| me(h))
            .map(|(_, i)| *i)
            .collect();
        s.equip_locked = gs
            .equip_locked
            .iter()
            .filter(|(h, _)| me(h))
            .map(|(_, i)| *i)
            .collect();
        s.ammo_loaded = gs.ammo_loaded.get(&PLAYER_REF).copied();
        s.hotkeys = gs.hotkeys;
        s.misc_stats = gs.misc_stats.clone();
        if plan.skills {
            s.skill_points = gs.skill_points.clone();
            s.tag_skills = gs.tag_skills.clone();
            s.tag_slots = gs.tag_slots.clone();
        }
        if plan.level_xp {
            s.level = gs.player_level;
            s.level_up_pending = gs.level_up_pending;
        }
        if plan.perks {
            s.perks = gs.perks.iter().copied().collect();
            s.perk_ranks = gs.perk_ranks.iter().map(|(k, v)| (*k, *v)).collect();
        }
        if plan.followers {
            s.teammates = gs.teammates.iter().copied().collect();
        }
        s.cell = gs.player_cell;
        s.world = gs.player_world;
        s.position = gs.player_position.map(|p| p.map(f32::to_bits));
        s.heading = gs.player_heading.to_bits();
        s.name = gs.player_name.clone();
        s.female = gs.player_female;
        for q in &plan.quests {
            if let Some(v) = gs.stages.get(q) {
                s.quest_stage.insert(*q, *v);
            }
            let bits = [
                (gs.running.contains(q), 1u8),
                (gs.completed.contains(q), 2),
                (gs.failed.contains(q), 4),
                (gs.quests_announced.contains(q), 8),
            ]
            .iter()
            .filter(|(on, _)| *on)
            .fold(0, |a, (_, b)| a | b);
            if bits != 0 {
                s.quest_flags.insert(*q, bits);
            }
        }
        s.stages_done = gs
            .stages_done
            .iter()
            .filter(|(q, _)| plan.quests.contains(q))
            .copied()
            .collect();
        for ((q, i), done) in &gs.objectives {
            if plan.quests.contains(q) {
                s.objectives.insert((*q, *i), if *done { 3 } else { 1 });
            }
        }
        for (q, i) in &gs.set_by_scripts.hidden_completed {
            if plan.quests.contains(q) {
                s.objectives.insert((*q, *i), 2);
            }
        }
        for (q, names) in &plan.quest_vars {
            if let Some(locals) = gs.variables.get(q) {
                for n in names {
                    // zero is the default: a machine that has not laid out the quest's
                    // variables yet must read the same shadow as one that has
                    if let Some(v) = locals.get(n).filter(|v| v.to_bits() != 0) {
                        s.quest_vars.insert((*q, n.clone()), v.to_bits());
                    }
                }
            }
        }
        if plan.casino {
            s.local.casinos = gs.casinos.clone();
        }
        s.local.caravan = gs.caravan.clone();
        s.local.effects = gs
            .active_effects
            .iter()
            .filter(|e| e.target == PLAYER_REF)
            .cloned()
            .collect();
        s
    }

    /// Removes the player's rows from `gs` (the inverse of [`Self::install`]).
    pub fn clear(gs: &mut GameState, plan: &ShadowPlan) {
        gs.actor_values
            .retain(|(w, id), _| !(*w == PLAYER_REF && plan.av_per_player(*id)));
        gs.value_damage.retain(|(w, _), _| *w != PLAYER_REF);
        gs.damage.remove(&PLAYER_REF);
        gs.dead.remove(&PLAYER_REF);
        gs.items.retain(|(h, _), _| *h != PLAYER_REF);
        gs.stocked.remove(&PLAYER_REF);
        gs.equipped.remove(&PLAYER_REF);
        gs.weapon_health.retain(|(h, _), _| *h != PLAYER_REF);
        gs.weapon_mods.retain(|(h, _), _| *h != PLAYER_REF);
        gs.dropped.retain(|(h, _)| *h != PLAYER_REF);
        gs.equip_locked.retain(|(h, _)| *h != PLAYER_REF);
        gs.ammo_loaded.remove(&PLAYER_REF);
        gs.misc_stats.clear();
        if plan.skills {
            gs.skill_points.clear();
            gs.tag_skills.clear();
            gs.tag_slots.clear();
        }
        if plan.perks {
            gs.perks.clear();
            gs.perk_ranks.clear();
        }
        if plan.followers {
            gs.teammates.clear();
        }
        for q in &plan.quests {
            gs.stages.remove(q);
            gs.running.remove(q);
            gs.completed.remove(q);
            gs.failed.remove(q);
            gs.quests_announced.remove(q);
        }
        if !plan.quests.is_empty() {
            gs.stages_done.retain(|(q, _)| !plan.quests.contains(q));
            gs.objectives.retain(|(q, _), _| !plan.quests.contains(q));
            gs.set_by_scripts
                .hidden_completed
                .retain(|(q, _)| !plan.quests.contains(q));
        }
        gs.active_effects.retain(|e| e.target != PLAYER_REF);
    }

    /// Writes the shadow into the (cleared) player rows of `gs`.
    pub fn install(&self, gs: &mut GameState, plan: &ShadowPlan) {
        for (id, v) in &self.actor_values {
            gs.actor_values.insert((PLAYER_REF, *id), *v);
        }
        for (id, v) in &self.value_damage {
            gs.value_damage.insert((PLAYER_REF, *id), *v);
        }
        if let Some(d) = self.damage {
            gs.damage.insert(PLAYER_REF, d);
        }
        if self.dead {
            gs.dead.insert(PLAYER_REF);
        }
        for (i, n) in &self.items {
            gs.items.insert((PLAYER_REF, *i), *n);
        }
        if self.stocked {
            gs.stocked.insert(PLAYER_REF);
        }
        if let Some(e) = &self.equipped {
            gs.equipped.insert(PLAYER_REF, e.clone());
        }
        for (i, v) in &self.weapon_health {
            gs.weapon_health
                .insert((PLAYER_REF, *i), f32::from_bits(*v));
        }
        for (i, v) in &self.weapon_mods {
            gs.weapon_mods.insert((PLAYER_REF, *i), *v);
        }
        for i in &self.dropped {
            gs.dropped.insert((PLAYER_REF, *i));
        }
        for i in &self.equip_locked {
            gs.equip_locked.insert((PLAYER_REF, *i));
        }
        if let Some(a) = self.ammo_loaded {
            gs.ammo_loaded.insert(PLAYER_REF, a);
        }
        gs.hotkeys = self.hotkeys;
        gs.misc_stats = self.misc_stats.clone();
        if plan.skills {
            gs.skill_points = self.skill_points.clone();
            gs.tag_skills = self.tag_skills.clone();
            gs.tag_slots = self.tag_slots.clone();
        }
        if plan.level_xp {
            gs.player_level = self.level;
            gs.level_up_pending = self.level_up_pending;
        }
        if plan.perks {
            gs.perks = self.perks.iter().copied().collect();
            gs.perk_ranks = self.perk_ranks.iter().map(|(k, v)| (*k, *v)).collect();
        }
        if plan.followers {
            gs.teammates = self.teammates.iter().copied().collect();
        }
        gs.player_cell = self.cell;
        gs.player_world = self.world;
        gs.player_position = self.position.map(|p| p.map(f32::from_bits));
        gs.player_heading = f32::from_bits(self.heading);
        gs.player_name = self.name.clone();
        gs.player_female = self.female;
        for (q, v) in &self.quest_stage {
            gs.stages.insert(*q, *v);
        }
        for (q, bits) in &self.quest_flags {
            for (b, set) in [
                (1u8, &mut gs.running),
                (2, &mut gs.completed),
                (4, &mut gs.failed),
                (8, &mut gs.quests_announced),
            ] {
                if bits & b != 0 {
                    set.insert(*q);
                }
            }
        }
        gs.stages_done.extend(self.stages_done.iter().copied());
        for ((q, i), st) in &self.objectives {
            match st {
                1 => {
                    gs.objectives.insert((*q, *i), false);
                }
                2 => {
                    gs.set_by_scripts.hidden_completed.insert((*q, *i));
                }
                3 => {
                    gs.objectives.insert((*q, *i), true);
                }
                _ => {}
            }
        }
        for (q, names) in &plan.quest_vars {
            if let Some(locals) = gs.variables.get_mut(q) {
                for n in names {
                    let v = self
                        .quest_vars
                        .get(&(*q, n.clone()))
                        .map_or(0.0, |b| f64::from_bits(*b));
                    locals.set(n, v);
                }
            }
        }
        if plan.casino {
            gs.casinos = self.local.casinos.clone();
        }
        gs.caravan = self.local.caravan.clone();
        gs.active_effects.extend(self.local.effects.iter().cloned());
    }
}

/// Parked shadows and the plan.
#[derive(Clone, Debug, Default)]
pub struct ShadowBank {
    /// Which categories are per player. `None`: shadows are off.
    pub plan: Option<ShadowPlan>,
    /// The slot whose rows are in the live `PLAYER_REF` slots (0 on a host).
    pub active: u8,
    /// Shadows of every other slot.
    pub parked: BTreeMap<u8, PlayerShadow>,
    /// Slots changed since the last [`GameState::take_shadow_dirty`].
    pub dirty: BTreeSet<u8>,
}

impl ShadowBank {
    /// True when shadows are on.
    pub fn enabled(&self) -> bool {
        self.plan.is_some()
    }
}

impl GameState {
    /// Turns per-player shadows on. `me` is the slot whose rows are live now
    /// (0 for the host, the guest's own slot on a guest). Both peers of a
    /// session must use the same plan.
    pub fn enable_shadows(&mut self, plan: ShadowPlan, me: u8) {
        self.coop.bank = ShadowBank {
            plan: Some(plan),
            active: me,
            ..ShadowBank::default()
        };
    }

    /// The slot that owns the live `PLAYER_REF` rows.
    pub fn active_slot(&self) -> u8 {
        self.coop.bank.active
    }

    /// Makes sure `slot` has a shadow (a fresh one if it joined just now).
    /// Returns false when shadows are off.
    pub fn register_player(&mut self, slot: u8) -> bool {
        if !self.coop.bank.enabled() {
            return false;
        }
        if slot != self.coop.bank.active && !self.coop.bank.parked.contains_key(&slot) {
            self.coop.bank.parked.insert(slot, PlayerShadow::fresh());
            self.coop.bank.dirty.insert(slot);
            self.emit(Domain::Shadow, shadow_form(slot));
        }
        true
    }

    /// Drops a departed player's parked shadow.
    pub fn forget_player(&mut self, slot: u8) {
        if slot != self.coop.bank.active && self.coop.bank.parked.remove(&slot).is_some() {
            self.coop.bank.dirty.insert(slot);
            self.emit(Domain::Shadow, shadow_form(slot));
        }
    }

    /// Slots changed since the last call.
    pub fn take_shadow_dirty(&mut self) -> BTreeSet<u8> {
        std::mem::take(&mut self.coop.bank.dirty)
    }

    /// A copy of `slot`'s shadow wherever it lives.
    pub fn shadow_of(&self, slot: u8) -> Option<PlayerShadow> {
        let plan = self.coop.bank.plan.as_ref()?;
        if slot == self.coop.bank.active {
            Some(PlayerShadow::read(self, plan))
        } else {
            self.coop.bank.parked.get(&slot).cloned()
        }
    }

    /// Makes `slot` the owner of the live `PLAYER_REF` rows.
    fn swap_to(&mut self, slot: u8) {
        let Some(plan) = self.coop.bank.plan.clone() else {
            return;
        };
        let prev = self.coop.bank.active;
        if prev == slot {
            return;
        }
        let parked = PlayerShadow::read(self, &plan);
        PlayerShadow::clear(self, &plan);
        let next = self
            .coop
            .bank
            .parked
            .remove(&slot)
            .unwrap_or_else(PlayerShadow::fresh);
        next.install(self, &plan);
        self.coop.bank.parked.insert(prev, parked);
        self.coop.bank.active = slot;
        crate::coop::campaign::extra::swap(self, prev, slot);
    }

    /// Runs as player `slot`: until the guard drops (also while unwinding),
    /// `PLAYER_REF` rows are that player's. With shadows off the guard is a
    /// plain pass-through.
    pub fn as_player(&mut self, slot: u8) -> ShadowGuard<'_> {
        let prev = self.coop.bank.active;
        let on = self.coop.bank.enabled();
        let prev_initiator = self.coop.gate.as_mut().map(|g| g.set_initiator(Some(slot)));
        if on {
            self.swap_to(slot);
        }
        ShadowGuard {
            gs: self,
            prev,
            slot,
            on,
            prev_initiator,
        }
    }

    /// Closure form of [`Self::as_player`].
    pub fn with_player<R>(&mut self, slot: u8, f: impl FnOnce(&mut GameState) -> R) -> R {
        let mut g = self.as_player(slot);
        f(&mut g)
    }
}

/// Swaps a player's rows in for a scope and the previous owner back on drop.
pub struct ShadowGuard<'a> {
    gs: &'a mut GameState,
    prev: u8,
    slot: u8,
    on: bool,
    prev_initiator: Option<Option<u8>>,
}

impl Deref for ShadowGuard<'_> {
    type Target = GameState;
    fn deref(&self) -> &GameState {
        self.gs
    }
}

impl DerefMut for ShadowGuard<'_> {
    fn deref_mut(&mut self) -> &mut GameState {
        self.gs
    }
}

impl Drop for ShadowGuard<'_> {
    fn drop(&mut self) {
        if self.on {
            self.gs.swap_to(self.prev);
            self.gs.coop.bank.dirty.insert(self.slot);
            let (a, b) = (self.slot, self.prev);
            self.gs.emit(Domain::Shadow, shadow_form(a));
            if a != b {
                self.gs.emit(Domain::Shadow, shadow_form(b));
                self.gs.coop.bank.dirty.insert(b);
            }
        }
        if let (Some(g), Some(p)) = (self.gs.coop.gate.as_mut(), self.prev_initiator) {
            g.set_initiator(p);
        }
    }
}

// ---------------------------------------------------------------- codec

struct W(Vec<u8>);

impl W {
    fn u8(&mut self, v: u8) {
        self.0.push(v);
    }
    fn u16(&mut self, v: u16) {
        self.0.extend(v.to_le_bytes());
    }
    fn u32(&mut self, v: u32) {
        self.0.extend(v.to_le_bytes());
    }
    fn i32(&mut self, v: i32) {
        self.0.extend(v.to_le_bytes());
    }
    fn u64(&mut self, v: u64) {
        self.0.extend(v.to_le_bytes());
    }
    fn f(&mut self, v: f64) {
        self.u64(v.to_bits());
    }
    fn form(&mut self, v: FormId) {
        self.u32(v.0);
    }
    fn opt_form(&mut self, v: Option<FormId>) {
        self.u8(v.is_some() as u8);
        self.u32(v.map_or(0, |f| f.0));
    }
    fn str(&mut self, s: &str) {
        self.u32(s.len() as u32);
        self.0.extend(s.as_bytes());
    }
    fn len(&mut self, n: usize) {
        self.u32(n as u32);
    }
}

struct R<'a> {
    b: &'a [u8],
    i: usize,
}

impl R<'_> {
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        let s = self.b.get(self.i..self.i.checked_add(n)?)?;
        self.i += n;
        Some(s)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn i32(&mut self) -> Option<i32> {
        Some(self.u32()? as i32)
    }
    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }
    fn f(&mut self) -> Option<f64> {
        Some(f64::from_bits(self.u64()?))
    }
    fn form(&mut self) -> Option<FormId> {
        Some(FormId(self.u32()?))
    }
    fn opt_form(&mut self) -> Option<Option<FormId>> {
        let on = self.u8()? != 0;
        let v = self.u32()?;
        Some(on.then_some(FormId(v)))
    }
    fn str(&mut self) -> Option<String> {
        let n = self.u32()? as usize;
        String::from_utf8(self.take(n)?.to_vec()).ok()
    }
    fn len(&mut self) -> Option<usize> {
        let n = self.u32()? as usize;
        // Every entry takes at least a byte: refuses absurd lengths early.
        (n <= self.b.len()).then_some(n)
    }
}

/// Serializes a shadow (without [`LocalExtras`]) deterministically.
pub fn encode(s: &PlayerShadow) -> Vec<u8> {
    let mut w = W(vec![VERSION]);
    w.len(s.actor_values.len());
    for (k, v) in &s.actor_values {
        w.u16(*k);
        w.f(*v);
    }
    w.len(s.value_damage.len());
    for (k, v) in &s.value_damage {
        w.u16(*k);
        w.f(*v);
    }
    w.u8(s.damage.is_some() as u8);
    w.f(s.damage.unwrap_or(0.0));
    w.u8(s.dead as u8);
    w.len(s.items.len());
    for (k, v) in &s.items {
        w.form(*k);
        w.i32(*v);
    }
    w.u8(s.stocked as u8);
    w.u8(s.equipped.is_some() as u8);
    let eq = s.equipped.as_deref().unwrap_or(&[]);
    w.len(eq.len());
    for f in eq {
        w.form(*f);
    }
    w.len(s.weapon_health.len());
    for (k, v) in &s.weapon_health {
        w.form(*k);
        w.u32(*v);
    }
    w.len(s.weapon_mods.len());
    for (k, v) in &s.weapon_mods {
        w.form(*k);
        w.u8(*v);
    }
    for set in [&s.dropped, &s.equip_locked, &s.perks, &s.teammates] {
        w.len(set.len());
        for f in set.iter() {
            w.form(*f);
        }
    }
    w.opt_form(s.ammo_loaded);
    for h in &s.hotkeys {
        w.opt_form(*h);
    }
    w.len(s.misc_stats.len());
    for (k, v) in &s.misc_stats {
        w.u8(*k);
        w.u32(*v);
    }
    w.len(s.skill_points.len());
    for (k, v) in &s.skill_points {
        w.u16(*k);
        w.u32(*v);
    }
    w.len(s.tag_skills.len());
    for k in &s.tag_skills {
        w.u16(*k);
    }
    w.len(s.tag_slots.len());
    for (k, v) in &s.tag_slots {
        w.u8(*k);
        w.u16(*v);
    }
    w.u16(s.level);
    w.u8(s.level_up_pending as u8);
    w.len(s.perk_ranks.len());
    for (k, v) in &s.perk_ranks {
        w.form(*k);
        w.u8(*v);
    }
    w.opt_form(s.cell);
    w.opt_form(s.world);
    w.u8(s.position.is_some() as u8);
    for c in s.position.unwrap_or([0; 3]) {
        w.u32(c);
    }
    w.u32(s.heading);
    w.u8(s.name.is_some() as u8);
    w.str(s.name.as_deref().unwrap_or(""));
    w.u8(match s.female {
        None => 0,
        Some(false) => 1,
        Some(true) => 2,
    });
    w.len(s.quest_stage.len());
    for (k, v) in &s.quest_stage {
        w.form(*k);
        w.u16(*v);
    }
    w.len(s.stages_done.len());
    for (q, st) in &s.stages_done {
        w.form(*q);
        w.u16(*st);
    }
    w.len(s.quest_flags.len());
    for (k, v) in &s.quest_flags {
        w.form(*k);
        w.u8(*v);
    }
    w.len(s.objectives.len());
    for ((q, i), v) in &s.objectives {
        w.form(*q);
        w.i32(*i);
        w.u8(*v);
    }
    w.len(s.quest_vars.len());
    for ((q, n), v) in &s.quest_vars {
        w.form(*q);
        w.str(n);
        w.u64(*v);
    }
    w.0
}

/// Parses a blob made by [`encode`]. `None` when malformed or newer.
pub fn decode(b: &[u8]) -> Option<PlayerShadow> {
    let mut r = R { b, i: 0 };
    if r.u8()? != VERSION {
        return None;
    }
    let mut s = PlayerShadow::fresh();
    for _ in 0..r.len()? {
        let k = r.u16()?;
        s.actor_values.insert(k, r.f()?);
    }
    for _ in 0..r.len()? {
        let k = r.u16()?;
        s.value_damage.insert(k, r.f()?);
    }
    let has = r.u8()? != 0;
    let d = r.f()?;
    s.damage = has.then_some(d);
    s.dead = r.u8()? != 0;
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.items.insert(k, r.i32()?);
    }
    s.stocked = r.u8()? != 0;
    let has = r.u8()? != 0;
    let mut eq = Vec::new();
    for _ in 0..r.len()? {
        eq.push(r.form()?);
    }
    s.equipped = has.then_some(eq);
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.weapon_health.insert(k, r.u32()?);
    }
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.weapon_mods.insert(k, r.u8()?);
    }
    for which in 0..4 {
        for _ in 0..r.len()? {
            let f = r.form()?;
            match which {
                0 => s.dropped.insert(f),
                1 => s.equip_locked.insert(f),
                2 => s.perks.insert(f),
                _ => s.teammates.insert(f),
            };
        }
    }
    s.ammo_loaded = r.opt_form()?;
    for h in s.hotkeys.iter_mut() {
        *h = r.opt_form()?;
    }
    for _ in 0..r.len()? {
        let k = r.u8()?;
        s.misc_stats.insert(k, r.u32()?);
    }
    for _ in 0..r.len()? {
        let k = r.u16()?;
        s.skill_points.insert(k, r.u32()?);
    }
    for _ in 0..r.len()? {
        s.tag_skills.insert(r.u16()?);
    }
    for _ in 0..r.len()? {
        let k = r.u8()?;
        s.tag_slots.insert(k, r.u16()?);
    }
    s.level = r.u16()?;
    s.level_up_pending = r.u8()? != 0;
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.perk_ranks.insert(k, r.u8()?);
    }
    s.cell = r.opt_form()?;
    s.world = r.opt_form()?;
    let has = r.u8()? != 0;
    let p = [r.u32()?, r.u32()?, r.u32()?];
    s.position = has.then_some(p);
    s.heading = r.u32()?;
    let has = r.u8()? != 0;
    let n = r.str()?;
    s.name = has.then_some(n);
    s.female = match r.u8()? {
        1 => Some(false),
        2 => Some(true),
        _ => None,
    };
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.quest_stage.insert(k, r.u16()?);
    }
    for _ in 0..r.len()? {
        let q = r.form()?;
        s.stages_done.insert((q, r.u16()?));
    }
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.quest_flags.insert(k, r.u8()?);
    }
    for _ in 0..r.len()? {
        let q = r.form()?;
        let i = r.i32()?;
        s.objectives.insert((q, i), r.u8()?);
    }
    for _ in 0..r.len()? {
        let q = r.form()?;
        let n = r.str()?;
        s.quest_vars.insert((q, n), r.u64()?);
    }
    (r.i == b.len()).then_some(s)
}

/// The chunk fields of `slot`'s blob.
fn chunks_of(slot: u8, blob: &[u8]) -> Vec<(Key, Field)> {
    blob.chunks(CHUNK)
        .take(usize::from(TAG_END - TAG_BASE))
        .enumerate()
        .map(|(i, c)| {
            let f = Field::Custom {
                tag: TAG_BASE + i as u16,
                data: c.to_vec(),
            };
            (f.key(cf(shadow_form(slot))), f)
        })
        .collect()
}

/// The shadow as it is replicated and hashed: without the per-frame
/// transients (cell, worldspace, feet position, heading). Each peer owns its
/// own copy of those (the live layer's avatars carry them), so they must not
/// enter the shared hash or one moving avatar would desync every peer.
fn replicated(s: &PlayerShadow) -> PlayerShadow {
    let mut r = s.clone();
    r.cell = None;
    r.world = None;
    r.position = None;
    r.heading = 0;
    r
}

/// Keeps `from`'s transients in `to`.
fn keep_transients(to: &mut PlayerShadow, from: &PlayerShadow) {
    to.cell = from.cell;
    to.world = from.world;
    to.position = from.position;
    to.heading = from.heading;
}

/// Adds every slot's shadow to `m` (the host and guests produce the same
/// fields for the same logical state).
pub fn export_into(gs: &GameState, m: &mut Fields) {
    let Some(plan) = gs.coop.bank.plan.as_ref() else {
        return;
    };
    let mut put = |slot: u8, s: &PlayerShadow| {
        for (k, f) in chunks_of(slot, &encode(&replicated(s))) {
            m.insert(k, f);
        }
    };
    let live = PlayerShadow::read(gs, plan);
    put(gs.coop.bank.active, &live);
    for (slot, s) in &gs.coop.bank.parked {
        put(*slot, s);
    }
}

/// Applies replicated changes to shadow forms. Rows of the live owner replace
/// the live rows; other slots are parked. Returns the slots touched.
pub fn apply_changes(gs: &mut GameState, changes: &[(Key, Option<Field>)]) -> BTreeSet<u8> {
    let mut touched = BTreeSet::new();
    let Some(plan) = gs.coop.bank.plan.clone() else {
        return touched;
    };
    let mut by_slot: BTreeMap<u8, Vec<&(Key, Option<Field>)>> = BTreeMap::new();
    for c in changes {
        if let Some(slot) = slot_of_shadow_form(FormId(c.0.form.0)) {
            by_slot.entry(slot).or_default().push(c);
        }
    }
    for (slot, list) in by_slot {
        let current = match gs.shadow_of(slot) {
            Some(s) => chunks_of(slot, &encode(&replicated(&s))),
            None => Vec::new(),
        };
        let mut chunks: BTreeMap<u16, Vec<u8>> = current
            .into_iter()
            .filter_map(|(_, f)| match f {
                Field::Custom { tag, data } => Some((tag, data)),
                _ => None,
            })
            .collect();
        for (k, f) in list {
            let tag = k.sub as u16;
            match f {
                Some(Field::Custom { data, .. }) if !data.is_empty() => {
                    chunks.insert(tag, data.clone());
                }
                _ => {
                    chunks.remove(&tag);
                }
            }
        }
        touched.insert(slot);
        if chunks.is_empty() {
            if slot != gs.coop.bank.active {
                gs.coop.bank.parked.remove(&slot);
            }
            continue;
        }
        let mut blob = Vec::new();
        let mut t = TAG_BASE;
        while let Some(c) = chunks.get(&t) {
            blob.extend_from_slice(c);
            t += 1;
        }
        let Some(mut shadow) = decode(&blob) else {
            continue;
        };
        if slot == gs.coop.bank.active {
            let old = PlayerShadow::read(gs, &plan);
            PlayerShadow::clear(gs, &plan);
            shadow.local = old.local.clone();
            keep_transients(&mut shadow, &old);
            shadow.install(gs, &plan);
        } else {
            if let Some(old) = gs.coop.bank.parked.get(&slot) {
                shadow.local = old.local.clone();
                keep_transients(&mut shadow, old);
            }
            gs.coop.bank.parked.insert(slot, shadow);
        }
    }
    touched
}
