//! Mines: placed (`PGRE` references) or laid by a thrower (weapon
//! animation types 11 and 12), as the game runs them (read from
//! FalloutNV.exe 1.4.0.525 with Ghidra; class and function names from the
//! Xbox 360 prototype's symbols, marked (Xbox PDB); evidence and gaps in
//! `docs/EXPLOSIVES.md`).
//!
//! - A mine is a grenade projectile whose record has the "alt. trigger"
//!   flag and a proximity (`GrenadeProjectile::IsMine`, `009bdf80`;
//!   [`crate::explosions::ProjectileRecord::is_mine`]). Its fuse isn't set
//!   when it's made (`009bda10`): it waits, every frame
//!   (`Projectile::CheckExplosion`, `009c3190`), for someone to come within
//!   its proximity (`Projectile::CheckExplosionProximity`, `009c39e0`:
//!   [`check_proximity`]); that sets the fuse ([`fuse_for`]) and the fuse
//!   then counts down as a grenade's, blinking faster as it runs out
//!   ([`blink_interval`]).
//! - Who sets it off (`Projectile::GetMineReactsToTarget`, `009c3930`:
//!   [`reacts_to`]): the living, not its owner's faction, not its layer's
//!   friends or allies; the player only as their perks' "Calculate Mine
//!   Explode Chance" (entry point 4: Light Step's 0) lets them, once
//!   ([`Mine::spares_player`]).
//! - E on an armed mine disarms it (`BGSProjectile::Activate` (Xbox PDB);
//!   `Projectile::TurnOff`, `009c43e0`: [`disarm`]); E on a disarmed one
//!   takes it: its projectile's default weapon source (the frag mine's
//!   `WeapMineFrag`). See [`activation`].
//! - `fMineAgeMax` (exe 0: never) replaces `fGrenadeAgeMax` for mines
//!   (`009b41b0`).

use std::collections::BTreeSet;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::PLAYER_REF;
use crate::explosions::{proj_flags, ProjectileRecord};
use crate::scripting::{base_of, game_setting, Facts, GameState};

/// The mine settings (exe default, then what `FalloutNV.esm` sets).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MineSettings {
    /// `fMinesDelayMin` (exe 0.1, data 0.2): the shortest fuse.
    pub delay_min: f32,
    /// `fMineExteriorRadiusMult` (exe 1, data 1.6): outdoors the proximity
    /// is this much larger.
    pub exterior_radius_mult: f32,
    /// `iProjectileMineShooterCanTrigger` (exe 0, data 1): whether the one
    /// who laid it sets it off.
    pub shooter_can_trigger: bool,
    /// `fMinesBlinkMax` (2), `fMinesBlinkFast` (0.1), `fMinesBlinkSlow`
    /// (0.25): the blinking of a running fuse.
    pub blink_max: f32,
    pub blink_fast: f32,
    pub blink_slow: f32,
    /// `iMineDisarmExperience` (exe 0, data 5).
    pub disarm_experience: f32,
}

impl MineSettings {
    pub fn read(order: &LoadOrder) -> MineSettings {
        let s = |n: &str, d: f32| game_setting(order, n).unwrap_or(d);
        MineSettings {
            delay_min: s("fMinesDelayMin", 0.1),
            exterior_radius_mult: s("fMineExteriorRadiusMult", 1.0),
            shooter_can_trigger: s("iProjectileMineShooterCanTrigger", 0.0) > 0.0,
            blink_max: s("fMinesBlinkMax", 2.0),
            blink_fast: s("fMinesBlinkFast", 0.1),
            blink_slow: s("fMinesBlinkSlow", 0.25),
            disarm_experience: s("iMineDisarmExperience", 0.0),
        }
    }
}

/// A mine as the proximity test sees it: its record, who laid it (an
/// actor), the faction owning it (a placed mine's `XOWN`, when a faction),
/// where it lies, and whether that's outdoors.
#[derive(Debug, Clone, PartialEq)]
pub struct Mine {
    pub projectile: ProjectileRecord,
    pub shooter: Option<FormId>,
    pub owner: Option<FormId>,
    pub position: [f32; 3],
    pub exterior: bool,
    /// The player didn't set it off once and never will (run-time flag
    /// 0x4000, set by `009c39e0` when the perk roll fails).
    pub spares_player: bool,
    /// Disarmed (run-time flag 0x200, `009c43e0`).
    pub disarmed: bool,
}

/// The faction relation codes `008b87a0` returns that let a mine react:
/// neutral (0) and enemy (1).
fn reacts_by_relation(
    order: &LoadOrder,
    state: &GameState,
    shooter: FormId,
    target: FormId,
) -> bool {
    matches!(
        crate::factions::reaction(order, state, shooter, target),
        crate::factions::Reaction::Neutral | crate::factions::Reaction::Enemy
    )
}

/// Whether a mine reacts to `target` (`009c3930`): not its layer unless
/// `iProjectileMineShooterCanTrigger`; not the dead; not a member of the
/// faction owning it (`008b8290`); not its layer's friends or allies
/// (`008b87a0` must say neutral or enemy).
// Translated from 009c3930 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn reacts_to(
    order: &LoadOrder,
    state: &GameState,
    s: &MineSettings,
    mine: &Mine,
    target: FormId,
) -> bool {
    if !s.shooter_can_trigger && Some(target) == mine.shooter {
        return false;
    }
    if state.dead.contains(&target) {
        return false;
    }
    if let Some(f) = mine.owner {
        if crate::factions::factions_of(order, state, target).contains(&f) {
            return false;
        }
    }
    match mine.shooter {
        Some(shooter) => reacts_by_relation(order, state, shooter, target),
        None => true,
    }
}

/// The fuse a mine gets when `by` sets it off (`009c39e0`): for people
/// (the player too: the actor's vtable `+0x218`, true for characters,
/// false for creatures) their Explosives skill (actor value 35) × the
/// record's timer ÷ 100 + `fMinesDelayMin`; for creatures
/// `fMinesDelayMin`.
// Translated from 009c39e0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn fuse_for(
    order: &LoadOrder,
    state: &GameState,
    s: &MineSettings,
    projectile: &ProjectileRecord,
    by: FormId,
) -> f32 {
    let character = by == PLAYER_REF
        || base_of(order, by)
            .and_then(|b| order.get(b))
            .is_some_and(|r| r.entry.header.kind == FourCC::new(b"NPC_"));
    if !character {
        return s.delay_min;
    }
    let explosives = Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(by, EXPLOSIVES)
    .unwrap_or(0.0) as f32;
    explosives * projectile.timer / 100.0 + s.delay_min
}

/// The Explosives skill's actor value.
const EXPLOSIVES: u16 = 35;

/// The proximity a mine reacts within (`009c39e0`): the record's (`PROJ`
/// `DATA` f32 at 28), × `fMineExteriorRadiusMult` outdoors.
pub fn proximity_radius(s: &MineSettings, mine: &Mine) -> f32 {
    let r = mine.projectile.proximity;
    if mine.exterior {
        r * s.exterior_radius_mult
    } else {
        r
    }
}

/// The proximity test (`009c39e0`) for a mine whose fuse isn't running:
/// the first of `people` (the living others near, with their positions)
/// it reacts to within its proximity ([`reacts_to`]) sets it off; else
/// the player (`player`: their position) when it reacts to them and, unless
/// it already spares them, their perks' "Calculate Mine Explode Chance"
/// (entry point 4, from 100) beats `roll` % 100 — a failed roll spares
/// them from then on. Returns the fuse it gets ([`fuse_for`]).
// Translated from 009c39e0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn check_proximity(
    order: &LoadOrder,
    state: &GameState,
    s: &MineSettings,
    mine: &mut Mine,
    people: &[(FormId, [f32; 3])],
    player: Option<[f32; 3]>,
    roll: u64,
) -> Option<f32> {
    if mine.disarmed {
        return None;
    }
    let r = proximity_radius(s, mine);
    let r2 = r * r;
    let near = |p: [f32; 3]| {
        let d = [0, 1, 2].map(|k| p[k] - mine.position[k]);
        d[0] * d[0] + d[1] * d[1] + d[2] * d[2] < r2
    };
    for &(who, p) in people {
        if who == PLAYER_REF || crate::more_functions::is_ghost(state, who) {
            continue;
        }
        if near(p) && reacts_to(order, state, s, mine, who) {
            return Some(fuse_for(order, state, s, &mine.projectile, who));
        }
    }
    let p = player?;
    if mine.spares_player || !near(p) || !reacts_to(order, state, s, mine, PLAYER_REF) {
        return None;
    }
    let chance = crate::perks::apply_for(
        order,
        state,
        PLAYER_REF,
        crate::perks::entry::CALCULATE_MINE_EXPLODE_CHANCE,
        100.0,
        &[crate::perks::Tab::Weapon(mine.projectile.form_id)],
    );
    if ((roll % 100) as i64) < chance.round() as i64 {
        Some(fuse_for(order, state, s, &mine.projectile, PLAYER_REF))
    } else {
        mine.spares_player = true;
        None
    }
}

/// How long until a running fuse's next blink (`009c3190`): with
/// `fMinesBlinkMax` or less to go, from `fMinesBlinkFast` at 0 up to
/// `fMinesBlinkSlow` at the max ((slow − fast) ÷ max × fuse + fast); more
/// to go, `fMinesBlinkSlow`. Each blink also restarts the countdown sound.
// Translated from 009c3190 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn blink_interval(s: &MineSettings, fuse: f32) -> f32 {
    if fuse <= s.blink_max {
        (s.blink_slow - s.blink_fast) / s.blink_max * fuse + s.blink_fast
    } else {
        s.blink_slow
    }
}

/// What E does to a mine (`BGSProjectile::Activate` (Xbox PDB)).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MineUse {
    /// Armed, and its record "can be disabled" (flag 0x20): disarm it.
    Disarm,
    /// Disarmed, or never armed, and "can be picked up" (0x40): take it
    /// (as its default weapon source).
    PickUp(Option<FormId>),
    Nothing,
}

pub fn activation(mine: &Mine) -> MineUse {
    let p = &mine.projectile;
    if p.flags & proj_flags::CAN_BE_DISABLED != 0 && !mine.disarmed {
        MineUse::Disarm
    } else if p.flags & proj_flags::CAN_BE_PICKED_UP != 0 {
        MineUse::PickUp(p.weapon_source)
    } else {
        MineUse::Nothing
    }
}

/// Disarms a mine (`Projectile::TurnOff`, `009c43e0`): it's turned off
/// (and plays its disable sound unless `silent`); when it wasn't silent and
/// either its fuse was running (`set_off`) or it would react to
/// `activator`, the player is credited: the "Mines Disarmed" statistic
/// (11, `004d5c60(0xb)`) and `iMineDisarmExperience` (5). Returns whether
/// it was credited.
// Translated from 009c43e0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn disarm(
    order: &LoadOrder,
    state: &mut GameState,
    s: &MineSettings,
    mine: &mut Mine,
    activator: FormId,
    set_off: bool,
    silent: bool,
) -> bool {
    if mine.disarmed || mine.projectile.flags & proj_flags::CAN_BE_DISABLED == 0 {
        return false;
    }
    mine.disarmed = true;
    if silent || !(set_off || reacts_to(order, state, s, mine, activator)) {
        return false;
    }
    crate::stats::bump(state, crate::stats::MINES_DISARMED, 1);
    if s.disarm_experience > 0.0 {
        crate::experience::reward(order, state, f64::from(s.disarm_experience));
    }
    true
}

/// What happened to the placed mines (`PGRE` references), saved: disarmed
/// ones and those gone (exploded or taken).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MineStates {
    pub disarmed: BTreeSet<FormId>,
    pub gone: BTreeSet<FormId>,
    /// Those whose fuse is counting down (the projectile's `+0xe4` below
    /// `FLT_MAX`, set by `009c39e0`), read when one is disarmed
    /// ([`use_placed`]). Not saved here.
    pub fuse_running: BTreeSet<FormId>,
}

/// A placed mine (`PGRE`): its reference, record and where it lies.
#[derive(Debug, Clone, PartialEq)]
pub struct Placed {
    pub reference: FormId,
    pub mine: Mine,
}

/// The placed mines in a place (an interior cell or a worldspace) not
/// gone, as their records and the state left them; the faction owning
/// each (`XOWN`, when it names a faction) owns it.
pub fn placed_in(order: &LoadOrder, state: &GameState, space: FormId) -> Vec<Placed> {
    let exterior = order
        .get(space)
        .is_some_and(|r| r.entry.header.kind == FourCC::new(b"WRLD"));
    let mut out = Vec::new();
    for rr in order.records_of_type(esm::sig::PGRE) {
        if rr.entry.header.is_deleted() || state.more.mines.gone.contains(&rr.form_id) {
            continue;
        }
        let here = if exterior {
            order.world_of(&rr) == Some(space)
        } else {
            order.cell_of(&rr) == Some(space)
        };
        if !here || !crate::enabled_now(order, rr.form_id, &state.disabled) {
            continue;
        }
        let Some(base) = base_of(order, rr.form_id) else {
            continue;
        };
        let Some(projectile) = ProjectileRecord::load(order, base) else {
            continue;
        };
        if !projectile.is_mine() {
            continue;
        }
        let Some((_, _, position, _)) = state.place(order, rr.form_id) else {
            continue;
        };
        let owner = rr
            .record()
            .ok()
            .and_then(|r| r.get(FourCC::new(b"XOWN")).map(|s| s.data.clone()))
            .filter(|d| d.len() >= 4)
            .map(|d| rr.plugin.to_global(FormId(le_u32(&d, 0))))
            .filter(|f| {
                order
                    .get(*f)
                    .is_some_and(|r| r.entry.header.kind == FourCC::new(b"FACT"))
            });
        out.push(Placed {
            reference: rr.form_id,
            mine: Mine {
                projectile,
                shooter: None,
                owner,
                position,
                exterior,
                spares_player: false,
                disarmed: state.more.mines.disarmed.contains(&rr.form_id),
            },
        });
    }
    out
}

/// The player takes a disarmed placed mine: its weapon source joins their
/// inventory and the mine is gone.
pub fn take(order: &LoadOrder, state: &mut GameState, placed: &Placed) -> Option<FormId> {
    let MineUse::PickUp(Some(item)) = activation(&placed.mine) else {
        return None;
    };
    state.pick_up(order, placed.reference, item, 1);
    gone(state, placed.reference);
    Some(item)
}

/// A placed mine is gone (exploded or taken): kept out of the place (its
/// reference off, as a taken item's, so its model goes) and saved so.
pub fn gone(state: &mut GameState, reference: FormId) {
    state.more.mines.gone.insert(reference);
    state.more.mines.disarmed.remove(&reference);
    state.more.mines.fuse_running.remove(&reference);
    if state.disabled.insert(reference, true) != Some(true) {
        state
            .events
            .push(crate::scripting::Event::Enable(reference, false));
    }
}

/// Saved lines: `minedisarmed <ref>`, `minegone <ref>`.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    for r in &state.more.mines.disarmed {
        line(format!("minedisarmed {:08X}", r.0));
    }
    for r in &state.more.mines.gone {
        line(format!("minegone {:08X}", r.0));
    }
}

pub(crate) fn load_line(state: &mut GameState, parts: &[&str]) -> Option<Result<(), String>> {
    let set = match *parts.first()? {
        "minedisarmed" => &mut state.more.mines.disarmed,
        "minegone" => &mut state.more.mines.gone,
        _ => return None,
    };
    Some(
        parts
            .get(1)
            .and_then(|s| u32::from_str_radix(s, 16).ok())
            .map(|r| {
                set.insert(FormId(r));
            })
            .ok_or_else(|| format!("can't read '{}'", parts.join(" "))),
    )
}

/// Whether a placed reference is an armed mine (its base a projectile with
/// a proximity, not disarmed nor gone): what the crosshair offers to
/// disarm (`00775a00`).
pub fn is_armed_mine(order: &LoadOrder, state: &GameState, reference: FormId) -> bool {
    !state.more.mines.disarmed.contains(&reference) && is_mine_reference(order, state, reference)
}

/// Whether a placed reference is a mine still lying there: its base a
/// projectile with a proximity.
pub fn is_mine_reference(order: &LoadOrder, state: &GameState, reference: FormId) -> bool {
    !state.more.mines.gone.contains(&reference)
        && base_of(order, reference)
            .and_then(|b| ProjectileRecord::load(order, b))
            .is_some_and(|p| p.proximity != 0.0)
}

/// The placed mines in a place as things the crosshair can pick
/// ([`crate::scripting::Interactive`], kind `PROJ`, with the projectile's
/// bounds and name).
pub fn interactive_in(
    order: &LoadOrder,
    state: &GameState,
    space: FormId,
) -> Vec<crate::scripting::Interactive> {
    placed_in(order, state, space)
        .into_iter()
        .filter_map(|p| {
            let base = base_of(order, p.reference)?;
            let record = order.get(base)?.record().ok()?;
            let bounds = record
                .get(FourCC::new(b"OBND"))
                .filter(|s| s.data.len() >= 12)
                .map(|s| {
                    let v = |i: usize| {
                        f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]))
                    };
                    ([v(0), v(1), v(2)], [v(3), v(4), v(5)])
                });
            let rotation = order
                .get(p.reference)
                .and_then(|r| r.record().ok())
                .and_then(|r| r.get(esm::sig::DATA).map(|d| d.data.clone()))
                .filter(|d| d.len() >= 24)
                .map_or([0.0; 3], |d| {
                    [3, 4, 5].map(|i| crate::cell::le_f32(&d, i * 4))
                });
            Some(crate::scripting::Interactive {
                reference: p.reference,
                base,
                script: None,
                count: 1,
                position: p.mine.position,
                rotation,
                scale: 1.0,
                trigger: None,
                bounds,
                name: record.full_name(),
                kind: FourCC::new(b"PROJ"),
            })
        })
        .collect()
}

/// What E on a placed mine did.
#[derive(Debug, Clone, PartialEq)]
pub enum MineUsed {
    /// Disarmed: the record's disable sound plays (`009c43e0`).
    Disarmed(Option<FormId>),
    /// Taken: the item's "added" message, when the game shows one.
    Taken(FormId, Option<String>),
}

/// E on a placed mine (`BGSProjectile::Activate`): an armed one is
/// disarmed ([`disarm`], credited when its fuse was running,
/// [`MineStates::fuse_running`], or it reacts to the player), a disarmed
/// one taken ([`take`]).
pub fn use_placed(order: &LoadOrder, state: &mut GameState, reference: FormId) -> Option<MineUsed> {
    let space = state.place(order, reference)?.0;
    let mut placed = placed_in(order, state, space)
        .into_iter()
        .find(|p| p.reference == reference)?;
    let s = MineSettings::read(order);
    match activation(&placed.mine) {
        MineUse::Disarm => {
            let set_off = state.more.mines.fuse_running.remove(&reference);
            disarm(
                order,
                state,
                &s,
                &mut placed.mine,
                PLAYER_REF,
                set_off,
                false,
            );
            state.more.mines.disarmed.insert(reference);
            Some(MineUsed::Disarmed(placed.mine.projectile.disable_sound))
        }
        MineUse::PickUp(Some(_)) => {
            let item = take(order, state, &placed)?;
            Some(MineUsed::Taken(
                item,
                crate::activation::pickup_message(order, item, 1),
            ))
        }
        _ => None,
    }
}
