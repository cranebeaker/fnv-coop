//! Combat depth in the world: remote players as targets and victims.
//!
//! * [`puppet_hurt`]: the `combat::hurt` hook for a remote player's puppet.
//!   An NPC's blow becomes a request to the host (it applies the party
//!   scaling, the down rules and the friendly-fire rule, and tells the
//!   victim's machine); a player's blow on a player goes through the
//!   friendly-fire rule.
//! * [`CombatFacts`]: per-machine facts (queued notes, the remote players'
//!   stealth, counters). Lives in `LiveFacts::combat`.
//! * [`CombatBridge`]: the per-frame driver the live adapters call. It keeps
//!   puppets at their real places, carries notes to the host, applies the
//!   host's verdicts to the local player (health, down, radiation, crippled
//!   limbs), splits enemy attention between players, attributes crime to
//!   the acting player, hears remote shots, and plans encounter extras.
//! * [`bot_line`]: the `coop ...` combat verbs shared by the viewer's test
//!   bot and the headless simulator.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::actors::ActorMsg;
use coop_core::authority::AuthorityTable;
use coop_core::avatar::{dequant_heading, dequant_pos, FLAG_DEAD, FLAG_DOWNED, FLAG_SNEAK};
use coop_core::combat::{
    BlastEvent, CombatClient, CombatMsg, DamageKind, EffectKind, EffectNote, ExtraPlan, FireEvent,
    HurtNote, Threat, ThreatDirector, ThreatInput, VatsResult, VatsShot,
};
use coop_core::ids::{CellId, FormId as CForm, PlayerId};
use coop_core::proto::Msg;
use coop_core::rules::{ReviveEvent, RulesEvent};
use coop_core::sync::{GuestLive, HostLive};
use esm::{FormId, FourCC, LoadOrder};

use super::apply::Ctx;
use super::facts::{player_of, puppet_of, SpawnNote};
use crate::combat::Weapon;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{game_setting, Facts, GameState};

/// Outgoing messages.
type Outs = Vec<(PlayerId, Msg)>;

/// Body-part condition values for limbs head, torso, left arm, right arm,
/// left leg, right leg.
pub const LIMB_AV: [u16; 6] = [25, 26, 27, 28, 29, 30];
/// The radiation counter value.
const RADS_AV: u16 = 54;
/// Sneak skill and action points values.
const SNEAK_AV: u16 = 42;
const AP_AV: u16 = 12;
/// How often the slow facts (shadows, stealth numbers) are re-read.
const SLOW_MS: u64 = 2_000;
/// How often enemies re-decide whom they attend to.
const RETARGET_MS: u64 = 300;
/// A sneak skill assumed for a player nobody has told us about.
const DEFAULT_SNEAK: u8 = 25;

fn cf(f: FormId) -> CForm {
    CForm(f.0)
}

fn wf(f: CForm) -> FormId {
    FormId(f.0)
}

/// What the AI needs to know about one remote player.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stealth {
    /// Sneaking now.
    pub sneaking: bool,
    /// Moving.
    pub moving: bool,
    /// Running.
    pub running: bool,
    /// Downed.
    pub downed: bool,
    /// Dead.
    pub dead: bool,
    /// Sneak skill.
    pub sneak_skill: f32,
    /// Level.
    pub level: u16,
}

impl Default for Stealth {
    fn default() -> Self {
        Stealth {
            sneaking: false,
            moving: false,
            running: false,
            downed: false,
            dead: false,
            sneak_skill: f32::from(DEFAULT_SNEAK),
            level: 1,
        }
    }
}

/// Where a puppet stands, as `GameState::place` reports.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PuppetPlace {
    /// Interior cell or worldspace.
    pub space: FormId,
    /// Cell.
    pub cell: FormId,
    /// Feet.
    pub pos: [f32; 3],
    /// Heading, radians.
    pub heading: f32,
}

/// An NPC blow on a player, waiting to go to the host.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NpcNote {
    /// The NPC.
    pub attacker: FormId,
    /// The player.
    pub victim: PlayerId,
    /// Points.
    pub amount: f64,
    /// Kind.
    pub kind: DamageKind,
    /// Limb.
    pub limb: u8,
}

/// A player's blow on another player.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PvpNote {
    /// The victim.
    pub victim: PlayerId,
    /// Points.
    pub amount: f64,
    /// Kind.
    pub kind: DamageKind,
    /// Limb.
    pub limb: u8,
}

/// One VATS shot the local player queued.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VatsAsk {
    /// Target actor.
    pub target: FormId,
    /// Limb.
    pub limb: u8,
    /// AP cost.
    pub cost: u16,
    /// Hit chance, per mille.
    pub chance: u16,
    /// Damage.
    pub damage: u16,
}

/// Counters for dumps and tests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CombatStats {
    /// NPC blows sent to the host (or resolved there).
    pub npc_hits_sent: u32,
    /// Player on player blows sent.
    pub pvp_sent: u32,
    /// Players the host saw another player put down.
    pub pvp_downs: u32,
    /// Hurts applied to the local player.
    pub hurts_applied: u32,
    /// Points of those.
    pub hurt_points: f64,
    /// Times the local player went down from a hurt.
    pub downs: u32,
    /// Effects applied to the local player.
    pub effects_applied: u32,
    /// Shots sent.
    pub fires_sent: u32,
    /// Other players' shots heard and shown.
    pub fires_seen: u32,
    /// Blasts sent.
    pub blasts_sent: u32,
    /// Other players' blasts shown.
    pub blasts_seen: u32,
    /// Area hits this machine dealt.
    pub blast_hits: u32,
    /// VATS queues sent.
    pub vats_sent: u32,
    /// VATS shots that came back as hits.
    pub vats_hits: u32,
    /// Times an enemy changed whom it attends to.
    pub retargets: u32,
    /// Crimes attributed to a remote player.
    pub crimes: u32,
    /// Extras created.
    pub extras_made: u32,
    /// Extras cleared.
    pub extras_cleared: u32,
    /// Frames the world clock was asked to slow for VATS while shared.
    pub vats_clock_kept: u32,
    /// Detection values the `probe` verb measured, oldest first (tests).
    pub probes: Vec<i32>,
    /// What the `probeparty` verb measured, oldest first (tests): the
    /// party's value (the best-seen player) and each player's, by slot.
    pub party_probes: Vec<(i32, BTreeMap<u8, i32>)>,
}

impl CombatStats {
    /// The dump object (`combat` in the state dump).
    pub fn json(&self) -> String {
        let base = format!(
            "{{\"npc_hits_sent\":{},\"pvp_sent\":{},\"hurts_applied\":{},\"hurt_points\":{:.1},\"downs\":{},\"effects_applied\":{},\"fires_sent\":{},\"fires_seen\":{},\"blasts_sent\":{},\"blasts_seen\":{},\"blast_hits\":{},\"vats_sent\":{},\"vats_hits\":{},\"retargets\":{},\"crimes\":{},\"extras_made\":{},\"extras_cleared\":{},\"vats_clock_kept\":{},\"probe_first\":{},\"probe_last\":{},\"probe_drop\":{}}}",
            self.npc_hits_sent,
            self.pvp_sent,
            self.hurts_applied,
            self.hurt_points,
            self.downs,
            self.effects_applied,
            self.fires_sent,
            self.fires_seen,
            self.blasts_sent,
            self.blasts_seen,
            self.blast_hits,
            self.vats_sent,
            self.vats_hits,
            self.retargets,
            self.crimes,
            self.extras_made,
            self.extras_cleared,
            self.vats_clock_kept,
            self.probes.first().copied().unwrap_or(0),
            self.probes.last().copied().unwrap_or(0),
            self.probes.first().copied().unwrap_or(0) - self.probes.last().copied().unwrap_or(0)
        );
        let party = self.party_probe_json();
        format!("{},{party}}}", base.strip_suffix('}').unwrap_or(&base))
    }

    /// `probe_party_first`, `_last`, `_drop` and `probe_slots.<slot>.{first,
    /// last,drop}` from the `probeparty` samples (zero without any).
    fn party_probe_json(&self) -> String {
        let first = self.party_probes.first();
        let last = self.party_probes.last();
        let pf = first.map_or(0, |p| p.0);
        let pl = last.map_or(0, |p| p.0);
        let mut slots: Vec<String> = Vec::new();
        if let (Some(f), Some(l)) = (first, last) {
            for (slot, a) in &f.1 {
                let b = l.1.get(slot).copied().unwrap_or(*a);
                let key = match slot {
                    0 => "host".to_string(),
                    1 => "guest".to_string(),
                    n => format!("guest{n}"),
                };
                slots.push(format!(
                    "\"{key}\":{{\"first\":{a},\"last\":{b},\"drop\":{}}}",
                    a - b
                ));
            }
        }
        format!(
            "\"probe_party_first\":{pf},\"probe_party_last\":{pl},\"probe_party_drop\":{},\"probe_slots\":{{{}}}",
            pf - pl,
            slots.join(",")
        )
    }
}

/// Per-machine combat facts (never replicated, never saved).
#[derive(Clone, Debug, Default)]
pub struct CombatFacts {
    /// NPC blows on remote players this frame.
    pub npc_hits: Vec<NpcNote>,
    /// Blows on other players this frame.
    pub pvp_hits: Vec<PvpNote>,
    /// Kind and limb the next blow on a puppet has (the caller sets it,
    /// default bullet on the torso).
    pub next_kind: Option<(DamageKind, u8)>,
    /// Shots the local player fired (the viewer pushes them).
    pub fires: Vec<FireEvent>,
    /// The newest shot, waiting for its impact (flushed into `fires`).
    pub last_shot: Option<FireEvent>,
    /// The bot's sneak override (the viewer's frame writes the avatar flags
    /// from the player, so the bot's request is kept here).
    pub bot_sneak: Option<bool>,
    /// Things that went off because of the local player.
    pub blasts: Vec<BlastEvent>,
    /// VATS shots queued by the local player (one queue per frame).
    pub vats: Vec<VatsAsk>,
    /// VATS entered (true) or left (false).
    pub vats_marks: Vec<bool>,
    /// Local player is in VATS now.
    pub vats_active: bool,
    /// What came of the last VATS queues, for the VATS presentation.
    pub vats_results: Vec<(u32, Vec<VatsResult>, u16)>,
    /// Other players' shots to show (impact decals, flashes).
    pub shown_fires: Vec<FireEvent>,
    /// Other players' explosions to show.
    pub shown_blasts: Vec<BlastEvent>,
    /// Remote players' stealth state, by player.
    pub stealth: BTreeMap<PlayerId, Stealth>,
    /// Where the puppets stand.
    pub puppets: BTreeMap<FormId, PuppetPlace>,
    /// The local player's lasting ailments (poison, bleeding, knocked down
    /// until second).
    pub poison: u16,
    /// Bleeding limbs.
    pub bleeding: BTreeSet<u8>,
    /// Knocked down until this session time (ms).
    pub knocked_until_ms: u64,
    /// Live extras, by id.
    pub extras: BTreeSet<FormId>,
    /// Counters.
    pub stats: CombatStats,
    /// Debug: the threat director's first choice for each enemy and each
    /// switch, with its inputs (newest last, at most [`THREAT_LOG_MAX`]).
    pub threat_log: Vec<String>,
}

/// Lines kept in [`CombatFacts::threat_log`].
pub const THREAT_LOG_MAX: usize = 16;

impl CombatFacts {
    /// The kind and limb for the next blow on a puppet.
    pub fn set_next(&mut self, kind: DamageKind, limb: u8) {
        self.next_kind = Some((kind, limb));
    }
}

/// The `combat::hurt` hook for a remote player's puppet. Returns false (a
/// remote player's health is theirs; the host judges the blow).
pub(crate) fn puppet_hurt(
    state: &mut GameState,
    victim: PlayerId,
    amount: f64,
    by: FormId,
) -> bool {
    if state.coop.live.replaying || amount <= 0.0 {
        return false;
    }
    let f = &mut state.coop.live.combat;
    let (kind, limb) = f.next_kind.take().unwrap_or((DamageKind::Bullet, 1));
    if by == PLAYER_REF || player_of(by).is_some() {
        f.pvp_hits.push(PvpNote {
            victim,
            amount,
            kind,
            limb,
        });
    } else {
        f.npc_hits.push(NpcNote {
            attacker: by,
            victim,
            amount,
            kind,
            limb,
        });
    }
    false
}

/// A body part of the game's body part data as one of the six limbs of the
/// wire (head 0, torso 1, left arm 2, right arm 3, left leg 4, right leg 5,
/// the weapon in hand 6; 7: none).
pub fn limb_of_part(part: Option<u8>) -> u8 {
    match part {
        Some(1 | 2) => 0,
        Some(0) => 1,
        Some(3 | 4) => 2,
        Some(5 | 6) => 3,
        Some(7..=9) => 4,
        Some(10..=12) => 5,
        Some(14) => 6,
        _ => 7,
    }
}

/// The local player fired `weapon` (a no-op in single player). Its impact
/// follows through [`note_impact`]; whatever is not completed is sent as a
/// muzzle-only shot at the end of the frame.
pub fn note_shot(state: &mut GameState, weapon: FormId) {
    if state.coop.authority == super::Authority::Single {
        return;
    }
    let cell = state.player_cell.map_or(0, |c| c.0);
    let at = state.player_position.unwrap_or([0.0; 3]);
    let f = &mut state.coop.live.combat;
    if let Some(prev) = f.last_shot.take() {
        f.fires.push(prev);
    }
    f.last_shot = Some(FireEvent {
        shooter: PlayerId(0),
        id: 0,
        weapon: cf(weapon),
        cell: CellId(cell),
        origin: [quant(at[0]), quant(at[1]), quant(at[2])],
        hit: [quant(at[0]), quant(at[1]), quant(at[2])],
        target: CForm(0),
        limb: 7,
        surface: 0,
    });
}

/// Where the newest shot landed, on whom and on which body part.
pub fn note_impact(
    state: &mut GameState,
    point: [f32; 3],
    target: Option<FormId>,
    part: Option<u8>,
) {
    if let Some(e) = state.coop.live.combat.last_shot.as_mut() {
        e.hit = [quant(point[0]), quant(point[1]), quant(point[2])];
        e.target = cf(target.unwrap_or(FormId(0)));
        e.limb = limb_of_part(part);
        e.surface = u8::from(target.is_none());
    }
}

/// The space (interior cell or worldspace) a remote player in `cell` is in,
/// as the local machine files it: the local worldspace for the local cell and
/// for any exterior cell of that worldspace.
fn remote_space(order: Option<&LoadOrder>, gs: &GameState, cell: FormId) -> FormId {
    let Some(world) = gs.player_world else {
        return cell;
    };
    if gs.player_cell == Some(cell) || cell == world {
        return world;
    }
    let same = order.is_some_and(|o| o.get(cell).is_some_and(|rr| o.world_of(&rr) == Some(world)));
    if same {
        world
    } else {
        cell
    }
}

/// Where a puppet stands (`GameState::place` asks for synthetic forms).
pub(crate) fn puppet_place(
    state: &GameState,
    r: FormId,
) -> Option<(FormId, FormId, [f32; 3], f32)> {
    let p = state.coop.live.combat.puppets.get(&r)?;
    Some((p.space, p.cell, p.pos, p.heading))
}

/// A remote player's stealth state, for detection (`None`: not a puppet).
pub(crate) fn stealth_of(state: &GameState, r: FormId) -> Option<Stealth> {
    let p = player_of(r)?;
    state
        .coop
        .live
        .combat
        .stealth
        .get(&p)
        .copied()
        .or(Some(Stealth::default()))
}

/// The remote players as things the AI can see: (puppet form, feet, moving,
/// running). Downed and dead players are not targets.
pub fn remote_seen(state: &GameState) -> Vec<(FormId, [f32; 3], bool, bool)> {
    let f = &state.coop.live.combat;
    f.puppets
        .iter()
        .filter_map(|(form, place)| {
            let p = player_of(*form)?;
            let s = f.stealth.get(&p).copied().unwrap_or_default();
            (!s.downed && !s.dead).then_some((*form, place.pos, s.moving, s.running))
        })
        .collect()
}

/// The remote players as blast and mine candidates: (puppet form, feet).
pub fn remote_candidates(state: &GameState) -> Vec<(FormId, [f32; 3])> {
    state
        .coop
        .live
        .combat
        .puppets
        .iter()
        .filter(|(f, _)| {
            player_of(**f).is_some_and(|p| {
                !state
                    .coop
                    .live
                    .combat
                    .stealth
                    .get(&p)
                    .is_some_and(|s| s.dead)
            })
        })
        .map(|(f, p)| (*f, p.pos))
        .collect()
}

// -- party detection -----------------------------------------------------

/// How well `npc` detects each player of the party now: the local player
/// (`PLAYER_REF`) and every remote player's puppet, each with their own
/// sneak, motion, skill and level (`Facts::detection`). Downed and dead
/// players and players outside the NPC's place are left out.
pub fn party_values(order: &LoadOrder, gs: &GameState, npc: FormId) -> Vec<(FormId, i32)> {
    let facts = Facts {
        order,
        state: gs,
        speaker: None,
    };
    let mut out = Vec::new();
    let local_out = gs.coop.live.downed || gs.dead.contains(&PLAYER_REF);
    if !local_out {
        if let Some(v) = facts.detection(npc, PLAYER_REF) {
            out.push((PLAYER_REF, v));
        }
    }
    for (form, _, _, _) in remote_seen(gs) {
        if let Some(v) = facts.detection(npc, form) {
            out.push((form, v));
        }
    }
    out
}

/// The party rule for stealth: an NPC detects the party as well as it
/// detects the player it sees best (the highest detection value). One
/// player in plain sight gives the party away, however well the others
/// sneak. `None` when no player shares the NPC's place.
pub fn party_detection(order: &LoadOrder, gs: &GameState, npc: FormId) -> Option<(FormId, i32)> {
    party_values(order, gs, npc)
        .into_iter()
        .fold(None, |best, (f, v)| match best {
            Some((_, b)) if b >= v => best,
            _ => Some((f, v)),
        })
}

/// Whom each enemy in combat attends to, by player slot: its combat
/// target when that is a player (`PLAYER_REF` is `me`, a puppet its
/// player). Dead enemies and other targets are left out.
pub fn enemy_targets(gs: &GameState, me: PlayerId) -> Vec<(FormId, PlayerId)> {
    gs.combat
        .iter()
        .filter(|(enemy, _)| !gs.dead.contains(*enemy) && player_of(**enemy).is_none())
        .filter_map(|(enemy, t)| {
            let p = if *t == PLAYER_REF { me } else { player_of(*t)? };
            Some((*enemy, p))
        })
        .collect()
}

/// Whether the shared simulation may be slowed or paused for VATS or a menu
/// on this machine. In a co-op session never: the presentation slows, the
/// world does not.
pub fn world_clock_may_change(state: &GameState) -> bool {
    state.coop.authority == super::Authority::Single
}

/// Something exploded because of the local player (or a mine): the other
/// machines are shown it. Damage was already dealt by the caller's own area
/// pass. A no-op in single player.
pub fn note_blast(
    state: &mut GameState,
    explosion: FormId,
    at: [f32; 3],
    damage: f32,
    radius: f32,
    mine: bool,
) {
    if state.coop.authority == super::Authority::Single {
        return;
    }
    let cell = state.player_cell.map_or(0, |c| c.0);
    state.coop.live.combat.blasts.push(BlastEvent {
        source: PlayerId(0),
        id: 0,
        explosion: cf(explosion),
        origin: CForm(0),
        cell: CellId(cell),
        pos: [quant(at[0]), quant(at[1]), quant(at[2])],
        damage: damage.clamp(0.0, 5000.0) as u16,
        radius: radius.clamp(0.0, 4096.0) as u16,
        kind: if mine {
            DamageKind::Mine
        } else {
            DamageKind::Explosion
        },
    });
}

/// Records that a VATS frame asked for a world slowdown and was refused.
pub fn note_vats_clock_kept(state: &mut GameState) {
    state.coop.live.combat.stats.vats_clock_kept += 1;
}

/// A hurt reaches the local player (health, down).
pub fn apply_hurt(order: &LoadOrder, gs: &mut GameState, h: &HurtNote) -> bool {
    if gs.dead.contains(&PLAYER_REF) || gs.coop.live.downed {
        return false;
    }
    let left = crate::combat::health(order, gs, PLAYER_REF).unwrap_or(f64::MAX);
    let mut amount = f64::from(h.amount);
    if amount >= left {
        amount = (left - 1.0).max(0.0);
        gs.coop.live.down_pending = true;
        gs.coop.live.combat.stats.downs += 1;
    }
    if amount > 0.0 {
        crate::combat::hurt_core(order, gs, PLAYER_REF, amount, wf(h.attacker));
    }
    let st = &mut gs.coop.live.combat.stats;
    st.hurts_applied += 1;
    st.hurt_points += amount;
    true
}

/// An effect reaches the local player.
pub fn apply_effect(order: &LoadOrder, gs: &mut GameState, e: &EffectNote, now_ms: u64) {
    match e.effect {
        EffectKind::Radiation => {
            crate::magic::change(
                order,
                gs,
                PLAYER_REF,
                RADS_AV,
                -f64::from(e.amount),
                FormId(0),
            );
        }
        EffectKind::Cripple => {
            if let Some(av) = LIMB_AV.get(usize::from(e.limb)) {
                crate::magic::change(order, gs, PLAYER_REF, *av, -100.0, FormId(0));
            }
        }
        EffectKind::Poison => gs.coop.live.combat.poison = if e.ms == 0 { 0 } else { e.amount },
        EffectKind::Bleed => {
            gs.coop.live.combat.bleeding.insert(e.limb);
        }
        EffectKind::Knockdown => gs.coop.live.combat.knocked_until_ms = now_ms + u64::from(e.ms),
        EffectKind::Cure => {
            let f = &mut gs.coop.live.combat;
            f.poison = 0;
            f.bleeding.clear();
            for av in LIMB_AV {
                if gs
                    .value_damage
                    .get(&(PLAYER_REF, av))
                    .copied()
                    .unwrap_or(0.0)
                    > 0.0
                {
                    crate::magic::change(order, gs, PLAYER_REF, av, 100.0, FormId(0));
                }
            }
        }
    }
    gs.coop.live.combat.stats.effects_applied += 1;
}

/// Everything the host told the local player and the other players' shots.
fn apply_incoming(order: &LoadOrder, gs: &mut GameState, client: &mut CombatClient, now_ms: u64) {
    for h in std::mem::take(&mut client.hurts) {
        apply_hurt(order, gs, &h);
    }
    for e in std::mem::take(&mut client.effects) {
        apply_effect(order, gs, &e, now_ms);
    }
    for f in std::mem::take(&mut client.fires) {
        hear_shot(order, gs, &f);
        gs.coop.live.combat.stats.fires_seen += 1;
        gs.coop.live.combat.shown_fires.push(f);
    }
    for b in std::mem::take(&mut client.blasts) {
        gs.coop.live.combat.stats.blasts_seen += 1;
        gs.coop.live.combat.shown_blasts.push(b);
    }
    for (id, (results, spent)) in std::mem::take(&mut client.vats_done) {
        let hits = results.iter().filter(|r| r.hit).count() as u32;
        gs.coop.live.combat.stats.vats_hits += hits;
        crate::vats::spend(gs, f32::from(spent));
        gs.coop.live.combat.vats_results.push((id, results, spent));
    }
    for (p, (sneak, level)) in client.stealth.clone() {
        let s = gs.coop.live.combat.stealth.entry(p).or_default();
        s.sneak_skill = f32::from(sneak);
        s.level = level.max(1);
    }
}

/// A remote player's shot is heard by this machine's AI (that player's own
/// noise, so the NPCs react to the one who fired).
fn hear_shot(order: &LoadOrder, gs: &mut GameState, f: &FireEvent) {
    if let Some(w) = Weapon::load(order, wf(f.weapon)) {
        crate::noise::attacked(order, gs, puppet_of(f.shooter), &w);
    }
}

/// The local resolution of a blast on this machine's world: every
/// candidate within the radius takes the falloff share. People are hurt
/// through `combat::hurt` (a guest's NPC hits become requests, a puppet's
/// hurt becomes a player blow for the host to judge). Returns the hits.
#[allow(clippy::too_many_arguments)]
pub fn resolve_blast(
    order: &LoadOrder,
    gs: &mut GameState,
    center: [f32; 3],
    radius: f32,
    damage: f32,
    by: FormId,
    kind: DamageKind,
    candidates: &[(FormId, [f32; 3])],
) -> u32 {
    let mut hits = 0;
    for t in crate::explosions::blast_targets(center, radius, candidates) {
        let amount = f64::from(damage * t.share);
        if amount <= 0.0 {
            continue;
        }
        if player_of(t.reference).is_some() {
            gs.coop.live.combat.set_next(kind, 1);
        }
        crate::combat::hurt(order, gs, t.reference, amount, by);
        hits += 1;
    }
    gs.coop.live.combat.stats.blast_hits += hits;
    hits
}

/// Per-machine driver, held by the live adapters.
#[derive(Default)]
pub struct CombatBridge {
    threat: ThreatDirector,
    next_retarget_ms: u64,
    next_slow_ms: u64,
    events_cursor: usize,
    /// Spawn requests for extras in flight, by request id.
    pending_extras: BTreeMap<u32, (ExtraPlan, FormId, CellId)>,
    seen_spawns: BTreeSet<FormId>,
    announced: BTreeMap<PlayerId, (u8, u16)>,
    skills: BTreeMap<PlayerId, (u8, u16)>,
}

/// A remote player as a machine sees them.
#[derive(Clone, Copy, Debug)]
pub struct Remote {
    /// Slot.
    pub player: PlayerId,
    /// Cell.
    pub cell: u32,
    /// Feet.
    pub pos: [f32; 3],
    /// Heading.
    pub heading: f32,
    /// Velocity.
    pub vel: [f32; 3],
    /// Avatar flags.
    pub flags: u16,
}

impl CombatBridge {
    /// A fresh bridge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Puts the puppets where the remote players are and refreshes their
    /// stealth state.
    pub fn sync_remotes(&mut self, gs: &mut GameState, me: PlayerId, remotes: &[Remote]) {
        self.sync_remotes_in(None, gs, me, remotes);
    }

    /// [`Self::sync_remotes`] that also knows the load order, so a player in
    /// another exterior cell of the worldspace the local player is in is
    /// filed under that worldspace (not under their own grid cell, which
    /// would make the AI treat them as being somewhere else).
    pub fn sync_remotes_in(
        &mut self,
        order: Option<&LoadOrder>,
        gs: &mut GameState,
        me: PlayerId,
        remotes: &[Remote],
    ) {
        let mut seen = BTreeSet::new();
        for r in remotes {
            if r.player == me {
                continue;
            }
            let form = puppet_of(r.player);
            seen.insert(form);
            let cell = FormId(r.cell);
            let space = remote_space(order, gs, cell);
            gs.coop.live.combat.puppets.insert(
                form,
                PuppetPlace {
                    space,
                    cell,
                    pos: r.pos,
                    heading: r.heading,
                },
            );
            gs.positions.insert(form, (r.pos, r.heading));
            let speed = (r.vel[0] * r.vel[0] + r.vel[1] * r.vel[1]).sqrt();
            let skill = self.skills.get(&r.player).copied();
            let s = gs.coop.live.combat.stealth.entry(r.player).or_default();
            s.sneaking = r.flags & FLAG_SNEAK != 0;
            s.moving = speed > 10.0;
            s.running = speed > 200.0 && !s.sneaking;
            s.downed = r.flags & FLAG_DOWNED != 0;
            s.dead = r.flags & FLAG_DEAD != 0;
            if let Some((sneak, level)) = skill {
                s.sneak_skill = f32::from(sneak);
                s.level = level.max(1);
            }
        }
        gs.coop.live.combat.puppets.retain(|f, _| seen.contains(f));
        gs.coop
            .live
            .combat
            .stealth
            .retain(|p, _| seen.contains(&puppet_of(*p)));
    }

    /// Turns the local queues into messages for the host. `send` takes one.
    fn local_notes(
        &mut self,
        gs: &mut GameState,
        client: &mut CombatClient,
        ap: u16,
        mut send: impl FnMut(CombatMsg),
    ) {
        let f = &mut gs.coop.live.combat;
        if let Some(e) = f.last_shot.take() {
            f.fires.push(e);
        }
        let mut by: BTreeMap<(FormId, PlayerId, u8, u8), f64> = BTreeMap::new();
        let mut kinds: BTreeMap<(FormId, PlayerId, u8, u8), DamageKind> = BTreeMap::new();
        for n in std::mem::take(&mut f.npc_hits) {
            let k = (n.attacker, n.victim, n.limb, n.kind as u8);
            *by.entry(k).or_insert(0.0) += n.amount;
            kinds.insert(k, n.kind);
        }
        for (k, amount) in by {
            let pts = amount.ceil().clamp(1.0, 5000.0) as u16;
            f.stats.npc_hits_sent += 1;
            send(client.npc_hit(cf(k.0), k.1, pts, kinds[&k], k.2));
        }
        let mut pv: BTreeMap<(PlayerId, u8, u8), (f64, DamageKind)> = BTreeMap::new();
        for n in std::mem::take(&mut f.pvp_hits) {
            let e = pv
                .entry((n.victim, n.limb, n.kind as u8))
                .or_insert((0.0, n.kind));
            e.0 += n.amount;
        }
        for ((victim, limb, _), (amount, kind)) in pv {
            f.stats.pvp_sent += 1;
            send(client.player_hit(victim, amount.ceil().clamp(1.0, 5000.0) as u16, kind, limb));
        }
        for e in std::mem::take(&mut f.fires) {
            f.stats.fires_sent += 1;
            send(client.fire(e));
        }
        for e in std::mem::take(&mut f.blasts) {
            f.stats.blasts_sent += 1;
            send(client.blast(e));
        }
        let asks = std::mem::take(&mut f.vats);
        if !asks.is_empty() {
            let shots: Vec<VatsShot> = asks
                .iter()
                .take(coop_core::combat::MAX_VATS_SHOTS)
                .map(|a| VatsShot {
                    target: cf(a.target),
                    limb: a.limb,
                    cost: a.cost,
                    chance: a.chance,
                    damage: a.damage,
                })
                .collect();
            f.stats.vats_sent += 1;
            let (_, m) = client.vats(ap, shots);
            send(m);
        }
        for on in std::mem::take(&mut f.vats_marks) {
            send(client.vats_mark(on));
        }
    }

    /// The local player's action points for the VATS claim.
    fn claim_ap(order: &LoadOrder, gs: &GameState) -> u16 {
        // no action points known for this player: claim the cap and let the
        // host hold them to what its shadow says
        if crate::vats::max_action_points(order, gs) <= 0.0 {
            return 255;
        }
        crate::vats::action_points(order, gs).clamp(0.0, 255.0) as u16
    }

    // -- enemy attention -------------------------------------------------

    /// Re-decides whom each enemy this machine owns attends to.
    pub fn retarget(
        &mut self,
        order: &LoadOrder,
        gs: &mut GameState,
        owned: &[FormId],
        me: PlayerId,
        now_ms: u64,
    ) {
        if now_ms < self.next_retarget_ms {
            return;
        }
        self.next_retarget_ms = now_ms + RETARGET_MS;
        let focus = gs.coop.live.scaling.map_or(1.0, |s| s.focus);
        let remotes: Vec<(PlayerId, FormId)> = gs
            .coop
            .live
            .combat
            .puppets
            .keys()
            .filter_map(|f| player_of(*f).map(|p| (p, *f)))
            .collect();
        if remotes.is_empty() {
            return;
        }
        let local_down = gs.coop.live.downed;
        let local_dead = gs.dead.contains(&PLAYER_REF);
        for &enemy in owned {
            if gs.dead.contains(&enemy) {
                self.threat.forget(cf(enemy));
                continue;
            }
            let Some(&cur) = gs.combat.get(&enemy) else {
                continue;
            };
            let player_side = cur == PLAYER_REF || player_of(cur).is_some();
            if !player_side {
                continue;
            }
            let Some((space, cell, at, _)) = gs.place(order, enemy) else {
                continue;
            };
            let mut inputs = Vec::new();
            let mut forms: BTreeMap<PlayerId, FormId> = BTreeMap::new();
            let mut consider =
                |p: PlayerId, form: FormId, downed: bool, dead: bool, gs: &GameState| {
                    let Some((sp, ce, pos, _)) = gs.place(order, form) else {
                        return;
                    };
                    let d = ((pos[0] - at[0]).powi(2)
                        + (pos[1] - at[1]).powi(2)
                        + (pos[2] - at[2]).powi(2))
                    .sqrt();
                    let seen = Facts {
                        order,
                        state: gs,
                        speaker: None,
                    }
                    .detection(enemy, form)
                    .is_some_and(|v| v > crate::detection::SEEN);
                    inputs.push(ThreatInput {
                        player: p,
                        dist: d,
                        seen,
                        downed,
                        dead,
                        same_cell: sp == space || ce == cell,
                    });
                    forms.insert(p, form);
                };
            consider(me, PLAYER_REF, local_down, local_dead, gs);
            for (p, form) in &remotes {
                let s = gs
                    .coop
                    .live
                    .combat
                    .stealth
                    .get(p)
                    .copied()
                    .unwrap_or_default();
                consider(*p, *form, s.downed, s.dead, gs);
            }
            // the game may have turned the enemy since the last choice (a script's
            // StartCombat): the director starts from what the enemy really does
            let had = self.threat.current(cf(enemy));
            let actual = forms.iter().find(|(_, f)| **f == cur).map(|(p, _)| *p);
            if let (Some(a), Some(h)) = (actual, had) {
                if self.threat.adopt(cf(enemy), a, now_ms) {
                    let log = &mut gs.coop.live.combat.threat_log;
                    log.push(format!(
                        "{now_ms} {:08X} game set {:08X} (director had {})",
                        enemy.0, cur.0, h.0
                    ));
                    if log.len() > THREAT_LOG_MAX {
                        log.remove(0);
                    }
                }
            }
            let first = had.is_none();
            let Some(Threat { target, switched }) =
                self.threat.choose(cf(enemy), now_ms, focus, &inputs)
            else {
                continue;
            };
            if first || switched {
                let seen: Vec<String> = inputs
                    .iter()
                    .map(|i| {
                        format!(
                            "{}:{:.0}:{}{}{}",
                            i.player.0,
                            i.dist,
                            if i.seen { "seen" } else { "unseen" },
                            if i.same_cell { "" } else { ":away" },
                            if i.downed || i.dead { ":down" } else { "" }
                        )
                    })
                    .collect();
                let log = &mut gs.coop.live.combat.threat_log;
                log.push(format!(
                    "{now_ms} {:08X} was {:08X} pick {} switched {switched} [{}]",
                    enemy.0,
                    cur.0,
                    target.0,
                    seen.join(" ")
                ));
                if log.len() > THREAT_LOG_MAX {
                    log.remove(0);
                }
            }
            if switched {
                if let Some(form) = forms.get(&target) {
                    if *form != cur {
                        gs.combat.insert(enemy, *form);
                        gs.coop.live.combat.stats.retargets += 1;
                    }
                }
            }
        }
    }

    /// The damage that the threat director counts for `p` on `enemy`.
    pub fn threat_damage(&self, enemy: FormId, p: PlayerId) -> f32 {
        self.threat.damage_by(cf(enemy), p)
    }

    /// Feeds the director the damage the host announced (`Damaged`).
    fn feed_threat(&mut self, client: &mut coop_core::actors::ActorClient) {
        for (actor, by, amount) in std::mem::take(&mut client.damage_log) {
            self.threat.damaged(actor, by, f32::from(amount));
        }
    }

    // -- host --------------------------------------------------------------

    /// The host's per-frame combat work. Call after the rules and the actor
    /// pump of the frame.
    #[allow(clippy::too_many_arguments)]
    pub fn host_frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut HostLive,
        own: &mut GuestLive,
        auth: &mut AuthorityTable,
        rules_events: &[RulesEvent],
        now_ms: u64,
    ) -> Outs {
        let order = cx.order;
        let mut outs: Outs = Vec::new();
        let members = live.members().to_vec();
        // remote players at their real places
        let remotes: Vec<Remote> = members
            .iter()
            .filter(|p| !p.is_host())
            .filter_map(|p| {
                let s = live.avatars.state(*p)?;
                Some(Remote {
                    player: *p,
                    cell: s.cell.0,
                    pos: [
                        dequant_pos(s.pos[0]),
                        dequant_pos(s.pos[1]),
                        dequant_pos(s.pos[2]),
                    ],
                    heading: dequant_heading(s.heading),
                    vel: [0.0; 3],
                    flags: s.flags,
                })
            })
            .collect();
        let prev: BTreeMap<PlayerId, [f32; 3]> = gs
            .coop
            .live
            .combat
            .puppets
            .iter()
            .filter_map(|(f, p)| Some((player_of(*f)?, p.pos)))
            .collect();
        let dt = 0.05_f32;
        let remotes: Vec<Remote> = remotes
            .into_iter()
            .map(|mut r| {
                if let Some(o) = prev.get(&r.player) {
                    r.vel = [(r.pos[0] - o[0]) / dt, (r.pos[1] - o[1]) / dt, 0.0];
                }
                r
            })
            .collect();
        // slow facts: skills from the shadows, AP the host believes in
        if now_ms >= self.next_slow_ms {
            self.next_slow_ms = now_ms + SLOW_MS;
            for p in &members {
                if let Some(sh) = gs.shadow_of(p.0) {
                    let sneak = sh
                        .actor_values
                        .get(&SNEAK_AV)
                        .copied()
                        .unwrap_or(f64::from(DEFAULT_SNEAK))
                        .clamp(0.0, 100.0) as u8;
                    self.skills.insert(*p, (sneak, sh.level));
                    let max = sh.actor_values.get(&AP_AV).copied().unwrap_or(0.0);
                    let used = sh.value_damage.get(&AP_AV).copied().unwrap_or(0.0);
                    if !p.is_host() && max > 0.0 {
                        live.combat
                            .vats
                            .known_ap
                            .insert(*p, (max - used).clamp(0.0, 255.0) as u16);
                    }
                }
            }
            for (p, v) in self.skills.clone() {
                if self.announced.get(&p) != Some(&v) {
                    self.announced.insert(p, v);
                    outs.extend(live.route_combat_all(CombatMsg::Stealth {
                        player: p,
                        sneak: v.0,
                        level: v.1,
                    }));
                }
            }
        }
        self.sync_remotes_in(Some(order), gs, PlayerId::HOST, &remotes);
        // vitals mirror
        let scaling = live.rules.scaling(now_ms);
        let max_hp = |order: &LoadOrder, gs: &GameState| {
            crate::combat::max_health(order, gs, PLAYER_REF)
                .unwrap_or(100.0)
                .clamp(1.0, 65535.0) as u16
        };
        let host_hp = crate::combat::health(order, gs, PLAYER_REF)
            .unwrap_or(0.0)
            .clamp(0.0, 65535.0) as u16;
        live.combat
            .vitals
            .track(PlayerId::HOST, host_hp, max_hp(order, gs));
        // the host player's blows on other players since the last frame go
        // first: the host was up when it dealt them (a blast that puts its
        // thrower down still hurts the players beside the thrower)
        let mut msgs: Vec<CombatMsg> = Vec::new();
        {
            let f = &mut gs.coop.live.combat;
            for n in std::mem::take(&mut f.pvp_hits) {
                f.stats.pvp_sent += 1;
                let pts = n.amount.ceil().clamp(1.0, 5000.0) as u16;
                msgs.extend(live.combat.vitals.player_vs_player(
                    now_ms,
                    PlayerId::HOST,
                    n.victim,
                    pts,
                    n.kind,
                    n.limb,
                ));
            }
        }
        live.combat.vitals.report(
            PlayerId::HOST,
            now_ms,
            host_hp,
            gs.coop.live.downed,
            gs.dead.contains(&PLAYER_REF),
        );
        for p in &members {
            if p.is_host() {
                continue;
            }
            if let Some(s) = live.avatars.state(*p) {
                live.combat.vitals.track(*p, s.health, s.health.max(100));
                let down = s.flags & FLAG_DOWNED != 0 || live.rules.revive.is_downed(*p);
                live.combat
                    .vitals
                    .report(*p, now_ms, s.health, down, s.flags & FLAG_DEAD != 0);
            }
        }
        // the host player's own notes
        {
            let f = &mut gs.coop.live.combat;
            let mut by: BTreeMap<(FormId, PlayerId, u8, u8), (f64, DamageKind)> = BTreeMap::new();
            for n in std::mem::take(&mut f.npc_hits) {
                let e = by
                    .entry((n.attacker, n.victim, n.limb, n.kind as u8))
                    .or_insert((0.0, n.kind));
                e.0 += n.amount;
            }
            for ((attacker, victim, limb, _), (amount, kind)) in by {
                f.stats.npc_hits_sent += 1;
                let pts = amount.ceil().clamp(1.0, 5000.0) as u16;
                msgs.extend(live.combat.vitals.hurt(
                    now_ms,
                    victim,
                    cf(attacker),
                    pts,
                    kind,
                    limb,
                    &scaling,
                ));
            }
        }
        outs.extend(live.route_combat(msgs));
        // a player put down by another player's hit (credit only: no
        // experience, karma or crime is paid for it)
        for d in live.combat.vitals.take_downs() {
            gs.coop.live.combat.stats.pvp_downs += 1;
            println!(
                "Co-op: player {} put player {} down.",
                d.attacker.0, d.victim.0
            );
        }
        let mut local: Vec<CombatMsg> = Vec::new();
        let ap = Self::claim_ap(order, gs);
        self.local_notes(gs, &mut own.combat, ap, |m| local.push(m));
        for m in local {
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Combat(m), now_ms, auth));
        }
        // revives: back up, cured
        let mut cursor = self.events_cursor.min(rules_events.len());
        if rules_events.len() < self.events_cursor {
            cursor = 0;
        }
        for e in &rules_events[cursor..] {
            if let RulesEvent::Revive(ReviveEvent::Revived { player, .. }) = e {
                let m = live.combat.vitals.revived(now_ms, *player, 30);
                outs.extend(live.route_combat(m));
            }
        }
        self.events_cursor = rules_events.len();
        // poison, bleeding
        let m = live.combat.vitals.tick(now_ms, &scaling);
        outs.extend(live.route_combat(m));
        // what the other players did: shots are heard, kills get attributed
        for f in std::mem::take(&mut live.combat.fires) {
            if !f.shooter.is_host() {
                hear_shot(order, gs, &f);
            }
        }
        live.combat.blasts.clear();
        for a in std::mem::take(&mut live.combat.attributions) {
            self.attribute(order, gs, a.actor, a.by, a.amount, a.killed);
        }
        // the host's own hurts to enemies feed the director too
        for h in &gs.coop.live.hurts {
            if h.by == PLAYER_REF {
                self.threat
                    .damaged(cf(h.who), PlayerId::HOST, h.amount as f32);
            }
        }
        self.feed_threat(&mut own.actors);
        // the inbox of this frame
        for m in live.take_local() {
            own.on_msg(m, now_ms);
        }
        apply_incoming(order, gs, &mut own.combat, now_ms);
        let owned: Vec<FormId> = own.actors.owned().into_iter().map(wf).collect();
        self.retarget(order, gs, &owned, PlayerId::HOST, now_ms);
        outs.extend(self.extras(order, gs, live, own, auth, now_ms));
        outs
    }

    /// Crime and reputation for the player who hit or killed someone: run
    /// as that player, so the witnesses are the ones who can see them.
    fn attribute(
        &mut self,
        order: &LoadOrder,
        gs: &mut GameState,
        actor: CForm,
        by: PlayerId,
        amount: u16,
        killed: bool,
    ) {
        self.threat.damaged(actor, by, f32::from(amount));
        if by.is_host() || gs.coop.authority != super::Authority::Host {
            return; // the host's own hits run the game's normal paths
        }
        let victim = wf(actor);
        if crate::combat::is_creature(order, victim) {
            return;
        }
        let slot = by.0;
        gs.with_player(slot, |g| {
            if amount > 0 {
                crate::crime::assault(order, g, victim);
            }
            if killed {
                crate::crime::murder(order, g, victim, PLAYER_REF);
            }
        });
        gs.coop.live.combat.stats.crimes += 1;
    }

    // -- extras ------------------------------------------------------------

    fn extras(
        &mut self,
        order: &LoadOrder,
        gs: &mut GameState,
        live: &mut HostLive,
        own: &mut GuestLive,
        auth: &mut AuthorityTable,
        now_ms: u64,
    ) -> Outs {
        let mut outs = Vec::new();
        let scaling = live.rules.scaling(now_ms);
        // answers to earlier requests
        let answered: Vec<u32> = self
            .pending_extras
            .keys()
            .copied()
            .filter(|r| own.actors.spawn_results.contains_key(r))
            .collect();
        for req in answered {
            let (plan, base, cell) = self.pending_extras.remove(&req).expect("listed");
            match own.actors.spawn_results.remove(&req) {
                Some(coop_core::actors::SpawnOutcome::Granted { id }) => {
                    live.combat.extras.created_as(&plan, id, cell);
                    gs.coop.live.combat.extras.insert(wf(id));
                    gs.coop.live.combat.stats.extras_made += 1;
                    gs.coop.live.spawns.push(SpawnNote {
                        id: wf(id),
                        base: Some(base),
                        pos: plan.pos,
                    });
                }
                _ => live.combat.extras.refused(&plan),
            }
        }
        // new hostile dynamic spawns bring extras
        let spawns: Vec<(CForm, coop_core::actors::SpawnRec)> =
            live.actors.spawn.live().map(|(i, r)| (*i, *r)).collect();
        for (id, rec) in spawns {
            if !self.seen_spawns.insert(wf(id)) {
                continue;
            }
            let is_extra = gs.coop.live.combat.extras.contains(&wf(id))
                || rec.site.0 & 0xFF00_0000 == 0xFD00_0000;
            let hostile = hostile_base(order, wf(rec.base))
                || crate::factions::reaction(order, gs, wf(id), PLAYER_REF)
                    == crate::factions::Reaction::Enemy;
            if !hostile {
                continue;
            }
            let pos = [
                dequant_pos(rec.pos[0]),
                dequant_pos(rec.pos[1]),
                dequant_pos(rec.pos[2]),
            ];
            if let Some(plan) = live.combat.extras.consider(&scaling, id, pos, is_extra) {
                let health = crate::combat::max_health(order, gs, wf(rec.base))
                    .unwrap_or(100.0)
                    .clamp(1.0, 65535.0) as u16;
                let fixed = [quant(plan.pos[0]), quant(plan.pos[1]), quant(plan.pos[2])];
                let (req, m) = own
                    .actors
                    .request_spawn(plan.site, rec.base, rec.cell, fixed, health);
                self.pending_extras
                    .insert(req, (plan, wf(rec.base), rec.cell));
                outs.extend(live.on_msg(PlayerId::HOST, Msg::Actor(m), now_ms, auth));
            }
        }
        // dead extras and extras left behind
        for id in gs.coop.live.combat.extras.clone() {
            if gs.dead.contains(&id) {
                live.combat.extras.gone(cf(id));
                gs.coop.live.combat.extras.remove(&id);
            }
        }
        let players: Vec<(PlayerId, [f32; 3])> = live
            .members()
            .iter()
            .filter_map(|p| {
                let s = live.avatars.state(*p)?;
                Some((
                    *p,
                    [
                        dequant_pos(s.pos[0]),
                        dequant_pos(s.pos[1]),
                        dequant_pos(s.pos[2]),
                    ],
                ))
            })
            .collect();
        if !players.is_empty() {
            let far = live
                .combat
                .extras
                .to_clear(&players, |id| gs.place(order, wf(id)).map(|p| p.2));
            for id in far {
                gs.coop.live.combat.extras.remove(&wf(id));
                gs.coop.live.combat.stats.extras_cleared += 1;
                gs.coop.live.spawns.push(SpawnNote {
                    id: wf(id),
                    base: None,
                    pos: [0.0; 3],
                });
                outs.extend(live.on_msg(
                    PlayerId::HOST,
                    Msg::Actor(ActorMsg::Despawn { id }),
                    now_ms,
                    auth,
                ));
            }
        }
        outs
    }

    // -- guest -------------------------------------------------------------

    /// A guest's per-frame combat work; returns messages for the host.
    pub fn guest_frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut GuestLive,
        me: PlayerId,
        now_ms: u64,
    ) -> Vec<Msg> {
        let order = cx.order;
        let ids: Vec<PlayerId> = live.remote.keys().copied().collect();
        let mut remotes = Vec::new();
        for p in ids {
            if let Some(s) = live.sample(p, now_ms) {
                remotes.push(Remote {
                    player: p,
                    cell: s.cell.0,
                    pos: s.pos,
                    heading: s.heading,
                    vel: s.vel,
                    flags: s.flags,
                });
            }
        }
        self.skills = live.combat.stealth.clone();
        self.sync_remotes_in(Some(order), gs, me, &remotes);
        let mut out = Vec::new();
        let ap = Self::claim_ap(order, gs);
        self.local_notes(gs, &mut live.combat, ap, |m| out.push(Msg::Combat(m)));
        self.feed_threat(&mut live.actors);
        for h in &gs.coop.live.pending_hits {
            if h.by == PLAYER_REF {
                self.threat.damaged(cf(h.who), me, h.amount as f32);
            }
        }
        apply_incoming(order, gs, &mut live.combat, now_ms);
        let owned: Vec<FormId> = live.actors.owned().into_iter().map(wf).collect();
        self.retarget(order, gs, &owned, me, now_ms);
        out
    }
}

fn quant(v: f32) -> i32 {
    coop_core::avatar::quant_pos(v)
}

/// The `probeparty` verb: puts `npc` `dist` units south of the local
/// player, facing them, reads [`party_values`] and [`party_detection`],
/// puts the NPC back and keeps the sample in the combat counters.
fn probe_party(order: &LoadOrder, gs: &mut GameState, npc: FormId, dist: f32) {
    let Some((space, cell, at, _)) = gs.place(order, PLAYER_REF) else {
        return;
    };
    let keep = (
        gs.positions.get(&npc).copied(),
        gs.spaces.get(&npc).copied(),
    );
    gs.positions
        .insert(npc, ([at[0], at[1] - dist, at[2]], 0.0));
    gs.spaces.insert(npc, (space, cell));
    let values = party_values(order, gs, npc);
    let best = party_detection(order, gs, npc);
    match keep.0 {
        Some(k) => gs.positions.insert(npc, k),
        None => gs.positions.remove(&npc),
    };
    match keep.1 {
        Some(k) => gs.spaces.insert(npc, k),
        None => gs.spaces.remove(&npc),
    };
    // the local player's slot: the host is slot 0, a guest its live rows' owner
    let me = if gs.coop.authority == super::Authority::Guest {
        gs.active_slot()
    } else {
        0
    };
    let by_slot: BTreeMap<u8, i32> = values
        .iter()
        .filter_map(|(f, v)| {
            let slot = if *f == PLAYER_REF {
                me
            } else {
                player_of(*f)?.0
            };
            Some((slot, *v))
        })
        .collect();
    if let Some((_, v)) = best {
        gs.coop.live.combat.stats.party_probes.push((v, by_slot));
    }
}

/// A creature base is always a threat to the party.
fn hostile_base(order: &LoadOrder, base: FormId) -> bool {
    order
        .get(base)
        .is_some_and(|r| r.entry.header.kind == FourCC::new(b"CREA"))
}

// ---------------------------------------------------------------------------
// The shared bot verbs
// ---------------------------------------------------------------------------

/// Combat verbs of the `coop ...` test bot, shared by the viewer's bot and
/// the headless simulator. `form` turns a word into a form (editor id or
/// hex). Returns whether the verb was one of ours.
///
/// ```text
/// coop npchit NPC SLOT DAMAGE [limb]    an NPC's blow on player SLOT
/// coop pvp SLOT DAMAGE                  a blow on another player
/// coop fire WEAPON X Y Z                the local player fired at a point
/// coop explode EXPL RADIUS DAMAGE       a blast at the local player's feet
/// coop vats TARGET LIMB CHANCE DAMAGE   one VATS shot (cost 20 AP)
/// coop vatsmark 0|1                     entered or left VATS
/// coop sneak 0|1                        the local player sneaks
/// coop probeparty NPC DIST              NPC's detection of the party (tests)
/// coop cripple LIMB | rads N | cure     effects on the local player
/// ```
pub fn bot_line(
    order: &LoadOrder,
    gs: &mut GameState,
    words: &[&str],
    form: &dyn Fn(&str) -> Option<FormId>,
) -> bool {
    let arg = |i: usize| words.get(i).copied();
    let num = |i: usize| arg(i).and_then(|w| w.parse::<f64>().ok());
    let me_cell = gs.player_cell.map_or(0, |c| c.0);
    let pos = gs.player_position.unwrap_or([0.0; 3]);
    let fixed = |p: [f32; 3]| [quant(p[0]), quant(p[1]), quant(p[2])];
    match arg(1).map(str::to_ascii_lowercase).as_deref() {
        Some("npchit") => {
            if let (Some(npc), Some(slot), Some(dmg)) = (arg(2).and_then(form), num(3), num(4)) {
                let limb = num(5).map_or(1, |l| l as u8);
                gs.coop.live.combat.set_next(DamageKind::Bullet, limb);
                crate::combat::hurt(order, gs, puppet_of(PlayerId(slot as u8)), dmg, npc);
            }
        }
        Some("pvp") => {
            if let (Some(slot), Some(dmg)) = (num(2), num(3)) {
                gs.coop.live.combat.set_next(DamageKind::Bullet, 1);
                crate::combat::hurt(order, gs, puppet_of(PlayerId(slot as u8)), dmg, PLAYER_REF);
            }
        }
        Some("fire") => {
            if let (Some(weapon), Some(x), Some(y), Some(z)) =
                (arg(2).and_then(form), num(3), num(4), num(5))
            {
                gs.coop.live.combat.fires.push(FireEvent {
                    shooter: PlayerId(0),
                    id: 0,
                    weapon: cf(weapon),
                    cell: CellId(me_cell),
                    origin: fixed(pos),
                    hit: fixed([x as f32, y as f32, z as f32]),
                    target: CForm(0),
                    limb: 7,
                    surface: 1,
                });
            }
        }
        Some("explode") => {
            if let (Some(expl), Some(radius), Some(damage)) =
                (arg(2).and_then(form), num(3), num(4))
            {
                gs.coop.live.combat.blasts.push(BlastEvent {
                    source: PlayerId(0),
                    id: 0,
                    explosion: cf(expl),
                    origin: CForm(0),
                    cell: CellId(me_cell),
                    pos: fixed(pos),
                    damage: damage as u16,
                    radius: radius as u16,
                    kind: DamageKind::Explosion,
                });
                // and the local resolution against everyone this machine knows
                let mut candidates = remote_candidates(gs);
                candidates.extend(
                    gs.coop
                        .live
                        .combat
                        .extras
                        .iter()
                        .filter_map(|e| gs.place(order, *e).map(|p| (*e, p.2))),
                );
                let people: Vec<(FormId, [f32; 3])> = gs
                    .positions
                    .iter()
                    .filter(|(f, _)| player_of(**f).is_none() && !gs.dead.contains(*f))
                    .map(|(f, (p, _))| (*f, *p))
                    .collect();
                candidates.extend(people);
                if !gs.dead.contains(&PLAYER_REF) {
                    candidates.push((PLAYER_REF, pos));
                }
                resolve_blast(
                    order,
                    gs,
                    pos,
                    radius as f32,
                    damage as f32,
                    PLAYER_REF,
                    DamageKind::Explosion,
                    &candidates,
                );
            }
        }
        Some("vats") => {
            if let (Some(target), Some(limb), Some(chance), Some(damage)) =
                (arg(2).and_then(form), num(3), num(4), num(5))
            {
                gs.coop.live.combat.vats.push(VatsAsk {
                    target,
                    limb: limb as u8,
                    cost: 20,
                    chance: chance as u16,
                    damage: damage as u16,
                });
            }
        }
        Some("vatsmark") => {
            let on = num(2).is_some_and(|v| v != 0.0);
            gs.coop.live.combat.vats_active = on;
            gs.coop.live.combat.vats_marks.push(on);
            if on && !world_clock_may_change(gs) {
                note_vats_clock_kept(gs);
            }
        }
        Some("sneak") => {
            let on = num(2).is_some_and(|v| v != 0.0);
            gs.coop.live.combat.bot_sneak = Some(on);
            gs.player_sneaking = on;
            if on {
                gs.coop.live.local_flags |= FLAG_SNEAK;
            } else {
                gs.coop.live.local_flags &= !FLAG_SNEAK;
            }
        }
        Some("cripple") => {
            if let Some(l) = num(2) {
                apply_effect(
                    order,
                    gs,
                    &EffectNote {
                        effect: EffectKind::Cripple,
                        amount: 0,
                        limb: l as u8,
                        ms: 0,
                    },
                    0,
                );
            }
        }
        Some("rads") => {
            if let Some(n) = num(2) {
                apply_effect(
                    order,
                    gs,
                    &EffectNote {
                        effect: EffectKind::Radiation,
                        amount: n as u16,
                        limb: 0,
                        ms: 0,
                    },
                    0,
                );
            }
        }
        Some("health") => {
            // coop health SLOT N (on the host): test worlds without a player
            // record get a fixed health value in that player's own rows
            if let (Some(slot), Some(h)) = (num(2), num(3)) {
                gs.with_player(slot as u8, |g| {
                    g.actor_values.insert((PLAYER_REF, 16), h);
                });
            }
        }
        Some("probe") => {
            // coop probe NPC DIST: how well NPC notices the first remote player
            // when standing DIST units from them, looking at them (tests)
            if let (Some(npc), Some(dist)) = (arg(2).and_then(form), num(3)) {
                let target = gs
                    .coop
                    .live
                    .combat
                    .puppets
                    .iter()
                    .next()
                    .map(|(f, p)| (*f, *p));
                if let Some((puppet, place)) = target {
                    let keep = (
                        gs.positions.get(&npc).copied(),
                        gs.spaces.get(&npc).copied(),
                    );
                    gs.positions.insert(
                        npc,
                        (
                            [place.pos[0], place.pos[1] - dist as f32, place.pos[2]],
                            0.0,
                        ),
                    );
                    gs.spaces.insert(npc, (place.space, place.cell));
                    let v = Facts {
                        order,
                        state: gs,
                        speaker: None,
                    }
                    .detection(npc, puppet);
                    match keep.0 {
                        Some(k) => gs.positions.insert(npc, k),
                        None => gs.positions.remove(&npc),
                    };
                    match keep.1 {
                        Some(k) => gs.spaces.insert(npc, k),
                        None => gs.spaces.remove(&npc),
                    };
                    if let Some(v) = v {
                        gs.coop.live.combat.stats.probes.push(v);
                    }
                }
            }
        }
        Some("probeparty") => {
            // coop probeparty NPC DIST: how well NPC detects the party (the
            // best-seen player, `party_detection`) and each player, standing
            // DIST units south of the local player and looking at them (tests)
            if let (Some(npc), Some(dist)) = (arg(2).and_then(form), num(3)) {
                probe_party(order, gs, npc, dist as f32);
            }
        }
        Some("cure") => apply_effect(
            order,
            gs,
            &EffectNote {
                effect: EffectKind::Cure,
                amount: 0,
                limb: 0,
                ms: 0,
            },
            0,
        ),
        _ => return false,
    }
    true
}

/// Light check that a game setting exists (kept for the diagnostics).
pub fn setting(order: &LoadOrder, name: &str, default: f32) -> f32 {
    game_setting(order, name).unwrap_or(default)
}
