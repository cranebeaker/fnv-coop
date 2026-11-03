//! Campaign policy enforcement inside the world.
//!
//! [`PolicyGate`] turns `GameState` facts plus session presence into
//! `coop_policy::State`, asks `PolicyTable::decide`, and carries the answer
//! out: Apply, Replicate (reward routing into player shadows, once only),
//! DeferToBarrier (stage held until every present player passed),
//! GatherThenApply, PerPlayerShadow, Reject.
//!
//! The gate lives in `GameState::coop.gate` (`None` by default: no
//! enforcement). World sites call the small `GameState::coop_*` helpers,
//! guarded by `authority != Single`.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use coop_core::ids::PlayerId;
use coop_policy::{
    CompanionOp, Decision, Event, EventKind, FanOut, Origin, PlayerMask, PolicyTable, Reject,
    RewardKind, Slot, State, TetherMode,
};
use esm::{FormId, LoadOrder};

use super::shadow::ShadowPlan;
use crate::dialogue::PLAYER_REF;
use crate::scripting::GameState;

/// Ledger slot used for rewards applied once to shared state.
pub const SHARED_SLOT: Slot = 0xFF;

/// A stable id for a reward, to dedupe by.
pub fn reward_id(quest: FormId, stage: u16, index: u16) -> u32 {
    quest.0.wrapping_mul(0x9E37_79B1) ^ (u32::from(stage) << 16 | u32::from(index))
}

/// What a quest reward gives.
#[derive(Clone, Debug, PartialEq)]
pub enum Grant {
    /// Bottle caps.
    Caps(i32),
    /// Experience points.
    Xp(f64),
    /// Items.
    Item { item: FormId, count: i32 },
    /// Quest items.
    QuestItem { item: FormId, count: i32 },
    /// A perk (or its next rank).
    Perk(FormId),
    /// Karma (shared).
    Karma(i32),
    /// A cost taken from the speaker.
    Payment { item: FormId, count: i32 },
}

impl Grant {
    fn kind(&self, order: &LoadOrder) -> RewardKind {
        match self {
            Grant::Caps(_) => RewardKind::Caps,
            Grant::Xp(_) => RewardKind::Xp,
            Grant::Item { .. } => RewardKind::Item,
            Grant::QuestItem { .. } => RewardKind::QuestItem,
            // rows name perks by editor id or `plugin:id`; send both aliases
            Grant::Perk(p) => {
                let edid = order
                    .get(*p)
                    .and_then(|r| r.editor_id().ok().flatten())
                    .unwrap_or_default();
                RewardKind::Perk(format!("{edid}|{:08X}", p.0))
            }
            Grant::Karma(_) => RewardKind::Karma,
            Grant::Payment { .. } => RewardKind::Payment,
        }
    }

    /// The grant with its amount scaled to `share` out of `total`; the
    /// first `remainder` recipients get one extra.
    fn portion(&self, n: i64, index: i64) -> Grant {
        let part = |total: i64| total / n + i64::from(index < total % n);
        match self {
            Grant::Caps(c) => Grant::Caps(part(i64::from(*c)) as i32),
            Grant::Xp(x) => Grant::Xp(part(x.floor() as i64) as f64),
            Grant::Item { item, count } => Grant::Item {
                item: *item,
                count: part(i64::from(*count)) as i32,
            },
            other => other.clone(),
        }
    }
}

/// Per-quest session facts (barrier progress, votes, tether).
#[derive(Clone, Debug, Default)]
pub struct QuestSession {
    /// Player who started the current dialogue or activation.
    pub initiator: Option<Slot>,
    /// Players inside the tether zone.
    pub in_zone: PlayerMask,
    /// Seconds the tether has been holding.
    pub tether_s: f32,
    /// Players that passed the barrier.
    pub passed: PlayerMask,
    /// Seconds the barrier has been waiting.
    pub barrier_s: f32,
    /// Ready check answers.
    pub ready_yes: PlayerMask,
    /// Ready check refusals.
    pub ready_no: PlayerMask,
    /// Seconds since the ready check began.
    pub ready_s: f32,
    /// Votes.
    pub votes_yes: PlayerMask,
    /// Votes against.
    pub votes_no: PlayerMask,
    /// Seconds since the vote began.
    pub vote_s: f32,
}

/// A stage waiting on a barrier or a gather.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingStage {
    /// Quest.
    pub quest: FormId,
    /// Stage.
    pub stage: u16,
    /// Player that caused it (`None`: host script).
    pub origin: Origin,
    /// Initiator when it was held.
    pub initiator: Option<Slot>,
}

/// A stage that must be run as one player.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Redirect {
    /// The owning player.
    pub slot: Slot,
    /// Quest.
    pub quest: FormId,
    /// Stage.
    pub stage: u16,
}

/// A held stage that can go now.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Released {
    /// The stage.
    pub stage: PendingStage,
    /// The decision that released it.
    pub decision: Decision,
}

/// Counters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GateStats {
    /// Applied once on shared state.
    pub applied: u32,
    /// Replicated to players.
    pub replicated: u32,
    /// Held.
    pub deferred: u32,
    /// Redirected to a player's shadow.
    pub per_player: u32,
    /// Refused, by reason.
    pub rejected: BTreeMap<String, u32>,
}

/// Outcome of routing one reward.
#[derive(Clone, Debug, PartialEq)]
pub struct RewardOutcome {
    /// What the policy said.
    pub decision: Decision,
    /// What was applied to which player (`SHARED_SLOT` for shared state).
    pub applied: Vec<(Slot, Grant)>,
}

/// The enforcement point.
#[derive(Clone, Debug)]
pub struct PolicyGate {
    table: Arc<PolicyTable>,
    /// Players counted for gates (the host is always in).
    pub present: PlayerMask,
    members: PlayerMask,
    held: PlayerMask,
    /// Load-order quest id to the id the table uses (DLC rows keep the
    /// plugin's own index byte).
    remap: BTreeMap<u32, u32>,
    unmap: BTreeMap<u32, u32>,
    /// Tether behaviour.
    pub tether_mode: TetherMode,
    /// Who causes the events now (host scripts by default).
    pub origin: Origin,
    initiator: Option<Slot>,
    ledger: BTreeSet<(u32, Slot)>,
    sessions: BTreeMap<FormId, QuestSession>,
    scenes: BTreeSet<FormId>,
    /// Followers that count as critters.
    pub critters: BTreeSet<FormId>,
    pending: Vec<PendingStage>,
    redirects: Vec<Redirect>,
    gathers: Vec<(FormId, PlayerMask)>,
    /// Counters.
    pub stats: GateStats,
}

impl PolicyGate {
    /// A gate over `table` with only the host present.
    pub fn new(table: Arc<PolicyTable>) -> PolicyGate {
        PolicyGate {
            table,
            present: PlayerMask::of(0),
            members: PlayerMask::of(0),
            held: PlayerMask::NONE,
            remap: BTreeMap::new(),
            unmap: BTreeMap::new(),
            tether_mode: TetherMode::On,
            origin: Origin::Host,
            initiator: None,
            ledger: BTreeSet::new(),
            sessions: BTreeMap::new(),
            scenes: BTreeSet::new(),
            critters: BTreeSet::new(),
            pending: Vec::new(),
            redirects: Vec::new(),
            gathers: Vec::new(),
            stats: GateStats::default(),
        }
    }

    /// The policy table.
    pub fn table(&self) -> &PolicyTable {
        &self.table
    }

    /// Installs the load-order to table id map (see `configure_campaign`).
    pub fn set_remap(&mut self, pairs: impl IntoIterator<Item = (u32, u32)>) {
        self.remap.clear();
        self.unmap.clear();
        for (load, policy) in pairs {
            self.remap.insert(load, policy);
            self.unmap.insert(policy, load);
        }
    }

    /// The table id of a quest form of the running load order.
    pub fn pid(&self, quest: FormId) -> u32 {
        self.remap.get(&quest.0).copied().unwrap_or(quest.0)
    }

    /// The quest form of the running load order for a table id.
    pub fn lid(&self, policy: u32) -> FormId {
        FormId(self.unmap.get(&policy).copied().unwrap_or(policy))
    }

    /// The row of a quest form of the running load order.
    pub fn row(&self, quest: FormId) -> Option<&coop_policy::QuestRow> {
        self.table.row(self.pid(quest))
    }

    fn mapped(&self, ev: &Event) -> Event {
        let mut e = ev.clone();
        e.quest = self.pid(FormId(ev.quest));
        e
    }

    /// The shared table.
    pub fn table_arc(&self) -> Arc<PolicyTable> {
        self.table.clone()
    }

    /// Claims a reward id for a slot; false when it was granted before.
    pub fn claim(&mut self, id: u32, slot: Slot) -> bool {
        self.ledger.insert((id, slot))
    }

    /// The initiator of the current run (see [`Self::set_initiator`]).
    pub fn initiator(&self) -> Option<Slot> {
        self.initiator
    }

    /// Players with an open interaction on `quest` (set by `begin_interaction`).
    pub fn session(&self, quest: FormId) -> Option<&QuestSession> {
        self.sessions.get(&quest)
    }

    /// Mutable session facts (producers).
    pub fn session_mut(&mut self, quest: FormId) -> &mut QuestSession {
        self.sessions.entry(quest).or_default()
    }

    /// Quests that have session facts.
    pub fn session_quests(&self) -> Vec<FormId> {
        self.sessions.keys().copied().collect()
    }

    /// Sets presence from the session's players (the host is always there).
    pub fn set_present(&mut self, players: impl IntoIterator<Item = PlayerId>) {
        let mut m = PlayerMask::of(0);
        for p in players {
            m = m.or(PlayerMask::of(p.0));
        }
        self.members = m;
        // late joiners wait out a running scene (policy.late_join)
        self.held = self.held.and(m).minus(PlayerMask::of(0));
        self.present = m.minus(self.held);
    }

    /// Everyone connected, late joiners included.
    pub fn members(&self) -> PlayerMask {
        self.members
    }

    /// Players held out of `present` until the scene ends.
    pub fn held(&self) -> PlayerMask {
        self.held
    }

    /// Holds a joiner out of the gates (the host is never held).
    pub fn hold_joiner(&mut self, slot: Slot) {
        if slot != 0 {
            self.held = self.held.or(PlayerMask::of(slot));
            self.present = self.present.minus(self.held);
        }
    }

    /// Lets every held joiner in; returns them.
    pub fn release_joiners(&mut self) -> PlayerMask {
        let h = self.held;
        self.held = PlayerMask::NONE;
        self.present = self.members;
        h
    }

    /// Scenes holding fast travel and saves now.
    pub fn scenes_active(&self) -> bool {
        !self.scenes.is_empty()
    }

    /// Sets the global initiator (used by `as_player`); returns the old one.
    pub fn set_initiator(&mut self, i: Option<Slot>) -> Option<Slot> {
        std::mem::replace(&mut self.initiator, i)
    }

    /// A player starts a dialogue or activation for `quest`.
    pub fn begin_interaction(&mut self, quest: FormId, slot: Slot) {
        self.sessions.entry(quest).or_default().initiator = Some(slot);
    }

    /// A player passed the barrier of `quest`.
    pub fn barrier_pass(&mut self, quest: FormId, slot: Slot) {
        let s = self.sessions.entry(quest).or_default();
        s.passed = s.passed.or(PlayerMask::of(slot));
    }

    /// A player entered (or left) the tether zone.
    pub fn set_in_zone(&mut self, quest: FormId, slot: Slot, inside: bool) {
        let s = self.sessions.entry(quest).or_default();
        s.in_zone = if inside {
            s.in_zone.or(PlayerMask::of(slot))
        } else {
            s.in_zone.minus(PlayerMask::of(slot))
        };
    }

    /// A ready check answer.
    pub fn ready_answer(&mut self, quest: FormId, slot: Slot, yes: bool) {
        let s = self.sessions.entry(quest).or_default();
        if yes {
            s.ready_yes = s.ready_yes.or(PlayerMask::of(slot));
        } else {
            s.ready_no = s.ready_no.or(PlayerMask::of(slot));
        }
    }

    /// A scene holds fast travel and saves while it plays.
    pub fn scene(&mut self, quest: FormId, on: bool) {
        if on {
            self.scenes.insert(quest);
        } else {
            self.scenes.remove(&quest);
        }
    }

    /// Advances barrier, tether, ready and vote clocks.
    pub fn tick(&mut self, dt: f32) {
        for s in self.sessions.values_mut() {
            s.tether_s += dt;
            s.barrier_s += dt;
            s.ready_s += dt;
            s.vote_s += dt;
        }
    }

    /// Forgets a quest's barrier, vote and tether progress.
    pub fn reset_session(&mut self, quest: FormId) {
        self.sessions.remove(&quest);
    }

    /// Facts the engine needs, from the game and the session.
    pub fn state_for(&self, gs: &GameState, quest: FormId) -> State {
        let s = self.sessions.get(&quest).cloned().unwrap_or_default();
        let row = self.row(quest);
        let committed_stages = match row.and_then(|r| r.exclusive.as_ref()) {
            Some(x) => x
                .stages
                .iter()
                .copied()
                .filter(|st| gs.stages_done.contains(&(quest, *st as u16)))
                .collect(),
            None => Vec::new(),
        };
        let committed_quests = self
            .table
            .group_rivals(self.pid(quest))
            .into_iter()
            .filter(|r| {
                let f = self.lid(*r);
                gs.stages.contains_key(&f) || gs.completed.contains(&f) || gs.failed.contains(&f)
            })
            .collect();
        let mut followers = [(0u8, 0u8); 8];
        for (slot, f) in followers.iter_mut().enumerate() {
            if let Some(sh) = gs.shadow_of(slot as u8) {
                let crit = sh
                    .teammates
                    .iter()
                    .filter(|t| self.critters.contains(t))
                    .count() as u8;
                *f = (sh.teammates.len() as u8 - crit, crit);
            } else if slot as u8 == gs.active_slot() {
                let crit = gs
                    .teammates
                    .iter()
                    .filter(|t| self.critters.contains(t))
                    .count() as u8;
                *f = (gs.teammates.len() as u8 - crit, crit);
            }
        }
        State {
            present: self.present,
            initiator: s.initiator.or(self.initiator),
            in_zone: s.in_zone,
            tether_elapsed_s: s.tether_s as u32,
            tether_mode: self.tether_mode,
            barrier_passed: s.passed,
            barrier_elapsed_s: s.barrier_s as u32,
            ready_yes: s.ready_yes,
            ready_no: s.ready_no,
            ready_elapsed_s: s.ready_s as u32,
            votes_yes: s.votes_yes,
            votes_no: s.votes_no,
            vote_elapsed_s: s.vote_s as u32,
            committed_stages,
            committed_quests,
            granted: self.ledger.iter().copied().collect(),
            active_scenes: self.scenes.iter().map(|f| f.0).collect(),
            followers,
            event_done: false,
        }
    }

    /// Decides an event against the current game.
    pub fn decide(&self, gs: &GameState, ev: &Event) -> Decision {
        self.table
            .decide(&self.mapped(ev), &self.state_for(gs, FormId(ev.quest)))
    }

    fn count(&mut self, d: &Decision) {
        match d {
            Decision::Apply => self.stats.applied += 1,
            Decision::Replicate { .. } => self.stats.replicated += 1,
            Decision::DeferToBarrier { .. } | Decision::GatherThenApply { .. } => {
                self.stats.deferred += 1
            }
            Decision::PerPlayerShadow { .. } => self.stats.per_player += 1,
            Decision::Reject { reason } => {
                *self
                    .stats
                    .rejected
                    .entry(format!("{reason:?}"))
                    .or_default() += 1
            }
        }
    }

    /// Consulted at `set_stage`: true when the stage may run now.
    pub fn stage_check(&mut self, gs: &GameState, quest: FormId, stage: u16) -> bool {
        let ev = Event {
            quest: quest.0,
            origin: self.origin,
            kind: EventKind::SetStage {
                stage: u32::from(stage),
            },
        };
        let d = self.decide(gs, &ev);
        if std::env::var_os("COOP_GATE_DEBUG").is_some() {
            eprintln!(
                "gate stage_check {:08X} stage {} -> {:?}",
                quest.0, stage, d
            );
        }
        self.count(&d);
        match d {
            Decision::Apply | Decision::Replicate { .. } => true,
            Decision::PerPlayerShadow { slot, .. } => {
                if gs.active_slot() == slot || !gs.coop.bank.enabled() {
                    true
                } else {
                    let r = Redirect { slot, quest, stage };
                    if !self.redirects.contains(&r) {
                        self.redirects.push(r);
                    }
                    false
                }
            }
            Decision::DeferToBarrier { .. } => {
                self.hold(quest, stage);
                false
            }
            Decision::GatherThenApply { stragglers } => {
                self.hold(quest, stage);
                if !self.gathers.iter().any(|(q, _)| *q == quest) {
                    self.gathers.push((quest, stragglers));
                }
                false
            }
            Decision::Reject { .. } => false,
        }
    }

    fn hold(&mut self, quest: FormId, stage: u16) {
        let p = PendingStage {
            quest,
            stage,
            origin: self.origin,
            initiator: self.initiator,
        };
        if !self.pending.contains(&p) {
            self.pending.push(p);
        }
    }

    /// Re-decides every held stage: those that may go are returned (the
    /// caller runs `set_stage` for them again), refused ones are dropped.
    pub fn poll(&mut self, gs: &GameState) -> Vec<Released> {
        let held = std::mem::take(&mut self.pending);
        let mut out = Vec::new();
        for p in held {
            let ev = Event {
                quest: p.quest.0,
                origin: p.origin,
                kind: EventKind::SetStage {
                    stage: u32::from(p.stage),
                },
            };
            let d = self
                .table
                .decide(&self.mapped(&ev), &self.state_for(gs, p.quest));
            match d {
                Decision::Apply | Decision::Replicate { .. } | Decision::PerPlayerShadow { .. } => {
                    self.gathers.retain(|(q, _)| *q != p.quest);
                    out.push(Released {
                        stage: p,
                        decision: d,
                    });
                }
                Decision::Reject { .. } => {
                    self.count(&d);
                    self.gathers.retain(|(q, _)| *q != p.quest);
                }
                _ => self.pending.push(p),
            }
        }
        out
    }

    /// Stages being held.
    pub fn pending(&self) -> &[PendingStage] {
        &self.pending
    }

    /// Stages that belong to one player's shadow and must be run via
    /// `GameState::as_player(slot)`.
    pub fn take_redirects(&mut self) -> Vec<Redirect> {
        std::mem::take(&mut self.redirects)
    }

    /// Warps the viewer should perform: (quest, stragglers).
    pub fn take_gathers(&mut self) -> Vec<(FormId, PlayerMask)> {
        std::mem::take(&mut self.gathers)
    }

    /// A guest asks to run `script` locally.
    pub fn guest_script_allowed(
        &self,
        gs: &GameState,
        quest: FormId,
        script: FormId,
        slot: Slot,
    ) -> bool {
        let ev = Event {
            quest: quest.0,
            origin: Origin::Player(slot),
            kind: EventKind::GuestScript { script: script.0 },
        };
        matches!(self.decide(gs, &ev), Decision::Apply)
    }

    /// Decides an intent a guest sent to set a stage (rejected for
    /// host-only quests).
    pub fn intent_set_stage(
        &self,
        gs: &GameState,
        slot: Slot,
        quest: FormId,
        stage: u16,
    ) -> Decision {
        let ev = Event {
            quest: quest.0,
            origin: Origin::Player(slot),
            kind: EventKind::SetStage {
                stage: u32::from(stage),
            },
        };
        self.decide(gs, &ev)
    }

    /// Fast travel allowed?
    pub fn fast_travel_ok(&self, gs: &GameState) -> bool {
        let ev = Event {
            quest: 0,
            origin: self.origin,
            kind: EventKind::FastTravel,
        };
        matches!(self.decide(gs, &ev), Decision::Apply)
    }

    /// Saving allowed?
    pub fn save_ok(&self, gs: &GameState) -> bool {
        let ev = Event {
            quest: 0,
            origin: self.origin,
            kind: EventKind::Save,
        };
        matches!(self.decide(gs, &ev), Decision::Apply)
    }

    /// A companion event for the live owner of the player rows.
    pub fn companion(
        &mut self,
        gs: &GameState,
        op: CompanionOp,
        quest: FormId,
        critter: bool,
    ) -> Decision {
        let owner = self.initiator.unwrap_or(gs.active_slot());
        // a host script run on behalf of a player acts as that player
        let ev = Event {
            quest: quest.0,
            origin: self.acting_origin(),
            kind: EventKind::Companion { op, owner, critter },
        };
        let d = self.decide(gs, &ev);
        self.count(&d);
        d
    }

    /// The origin of player-attributable events: the player a host script
    /// runs on behalf of (`GameState::as_player`), else the plain origin.
    pub fn acting_origin(&self) -> Origin {
        self.initiator.map(Origin::Player).unwrap_or(self.origin)
    }

    /// A write to a per-player shadow class, checked against the player the
    /// script runs for (`NotOwner` when it targets another slot).
    pub fn shadow_write(
        &mut self,
        gs: &GameState,
        quest: FormId,
        class: coop_policy::ShadowClass,
        slot: Slot,
        var: Option<&str>,
    ) -> Decision {
        let ev = Event {
            quest: quest.0,
            origin: self.acting_origin(),
            kind: EventKind::ShadowWrite {
                class,
                slot,
                var: var.map(str::to_string),
            },
        };
        let d = self.decide(gs, &ev);
        self.count(&d);
        d
    }

    /// A companion command from a player (not a script): owner only, except
    /// plot companions of a party-owned row.
    pub fn companion_command(
        &mut self,
        gs: &GameState,
        quest: FormId,
        slot: Slot,
        owner: Slot,
        op: CompanionOp,
    ) -> Decision {
        let ev = Event {
            quest: quest.0,
            origin: Origin::Player(slot),
            kind: EventKind::Companion {
                op,
                owner,
                critter: false,
            },
        };
        let d = self.decide(gs, &ev);
        self.count(&d);
        d
    }

    /// A hit by `slot` on `victim`: the friendly fire guard.
    pub fn hit_check(
        &mut self,
        gs: &GameState,
        quest: FormId,
        slot: Slot,
        victim: &str,
        recent: u32,
    ) -> Decision {
        let ev = Event {
            quest: quest.0,
            origin: Origin::Player(slot),
            kind: EventKind::Hit {
                victim: victim.to_string(),
                recent,
            },
        };
        let d = self.decide(gs, &ev);
        self.count(&d);
        d
    }

    /// A fork put to the party (`Choice`), decided from the votes collected.
    pub fn choice(&mut self, gs: &GameState, quest: FormId, choice: u32) -> Decision {
        let ev = Event {
            quest: quest.0,
            origin: self.origin,
            kind: EventKind::Choice { choice },
        };
        let d = self.decide(gs, &ev);
        self.count(&d);
        d
    }

    /// A ready request raised by an INFO end script.
    pub fn ready_request(&mut self, gs: &GameState, quest: FormId, info: u32) -> Decision {
        let ev = Event {
            quest: quest.0,
            origin: self.origin,
            kind: EventKind::ReadyRequest { info },
        };
        let d = self.decide(gs, &ev);
        self.count(&d);
        d
    }

    /// Records a vote on a fork.
    pub fn vote(&mut self, quest: FormId, slot: Slot, yes: bool) {
        let s = self.sessions.entry(quest).or_default();
        if yes {
            s.votes_yes = s.votes_yes.or(PlayerMask::of(slot));
            s.votes_no = s.votes_no.minus(PlayerMask::of(slot));
        } else {
            s.votes_no = s.votes_no.or(PlayerMask::of(slot));
            s.votes_yes = s.votes_yes.minus(PlayerMask::of(slot));
        }
    }

    /// Starts a fresh vote clock for `quest`.
    pub fn begin_vote(&mut self, quest: FormId) {
        let s = self.sessions.entry(quest).or_default();
        s.votes_yes = PlayerMask::NONE;
        s.votes_no = PlayerMask::NONE;
        s.vote_s = 0.0;
    }

    /// Starts a fresh ready clock for `quest`.
    pub fn begin_ready(&mut self, quest: FormId) {
        let s = self.sessions.entry(quest).or_default();
        s.ready_yes = PlayerMask::NONE;
        s.ready_no = PlayerMask::NONE;
        s.ready_s = 0.0;
    }

    /// Routes one reward: decides, then applies it to each recipient's
    /// shadow exactly once (the ledger remembers `(reward_id, slot)`).
    pub fn route_reward(
        &mut self,
        gs: &mut GameState,
        order: &LoadOrder,
        quest: FormId,
        id: u32,
        grant: Grant,
    ) -> RewardOutcome {
        let ev = Event {
            quest: quest.0,
            origin: self.origin,
            kind: EventKind::Reward {
                reward_id: id,
                what: grant.kind(order),
            },
        };
        let d = self.decide(gs, &ev);
        let mut applied = Vec::new();
        match &d {
            Decision::Replicate { recipients, mode } => {
                let slots: Vec<Slot> = (0..8u8).filter(|s| recipients.contains(*s)).collect();
                for (i, slot) in slots.iter().enumerate() {
                    let part = match mode {
                        FanOut::Full => grant.clone(),
                        FanOut::Split => grant.portion(slots.len() as i64, i as i64),
                    };
                    if !self.ledger.insert((id, *slot)) {
                        continue;
                    }
                    gs.with_player(*slot, |g| give(g, order, &part));
                    applied.push((*slot, part));
                }
            }
            Decision::Apply => {
                if self.ledger.insert((id, SHARED_SLOT)) {
                    give(gs, order, &grant);
                    applied.push((SHARED_SLOT, grant.clone()));
                } else {
                    let r = Decision::Reject {
                        reason: Reject::AlreadyGranted,
                    };
                    self.count(&r);
                    return RewardOutcome {
                        decision: r,
                        applied,
                    };
                }
            }
            _ => {}
        }
        self.count(&d);
        RewardOutcome {
            decision: d,
            applied,
        }
    }

    /// Rewards already granted.
    pub fn granted(&self) -> impl Iterator<Item = &(u32, Slot)> {
        self.ledger.iter()
    }
}

/// Gives `grant` to whoever owns the live `PLAYER_REF` rows.
fn give(gs: &mut GameState, order: &LoadOrder, grant: &Grant) {
    let add = |gs: &mut GameState, item: FormId, n: i32| {
        if n <= 0 {
            return;
        }
        gs.stock(order, PLAYER_REF);
        *gs.items.entry((PLAYER_REF, item)).or_insert(0) += n;
        gs.emit(super::Domain::Items, PLAYER_REF);
        gs.added(order, PLAYER_REF, item, n);
    };
    match grant {
        Grant::Caps(n) => add(gs, crate::items::CAPS, *n),
        Grant::Item { item, count } | Grant::QuestItem { item, count } => add(gs, *item, *count),
        Grant::Xp(x) => {
            crate::experience::reward(order, gs, *x);
        }
        Grant::Perk(p) => crate::perks::add(order, gs, *p),
        Grant::Karma(k) => crate::reputation::reward_karma(order, gs, *k),
        Grant::Payment { item, count } => {
            gs.stock(order, PLAYER_REF);
            let have = gs.items.get(&(PLAYER_REF, *item)).copied().unwrap_or(0);
            let left = (have - count).max(0);
            if left == 0 {
                gs.items.remove(&(PLAYER_REF, *item));
            } else {
                gs.items.insert((PLAYER_REF, *item), left);
            }
            gs.emit(super::Domain::Items, PLAYER_REF);
        }
    }
}

impl GameState {
    /// The runtime form id of the policy row named `edid` (a quest's editor id). The gate's
    /// rows are remapped to the load order, so this is right whatever index a plugin has.
    pub fn coop_policy_quest(&self, edid: &str) -> Option<esm::FormId> {
        let row = self.coop.gate.as_ref()?.table().row_by_edid(edid)?;
        Some(esm::FormId(row.id))
    }

    /// Turns the campaign policy on: installs the gate and the shadow plan
    /// derived from the table. `me` is the slot owning the live rows.
    pub fn enable_coop_policy(&mut self, table: Arc<PolicyTable>, me: u8) {
        self.enable_shadows(ShadowPlan::from_policy(&table), me);
        self.coop.gate = Some(Box::new(PolicyGate::new(table)));
    }

    pub(crate) fn with_gate<R>(
        &mut self,
        default: R,
        f: impl FnOnce(&mut PolicyGate, &GameState) -> R,
    ) -> R {
        let Some(mut gate) = self.coop.gate.take() else {
            return default;
        };
        let r = f(&mut gate, self);
        self.coop.gate = Some(gate);
        r
    }

    /// `set_stage` hook: false when the policy holds or refuses the stage.
    pub fn coop_gate_stage(&mut self, quest: FormId, stage: u16) -> bool {
        let ok = self.with_gate(true, |g, gs| g.stage_check(gs, quest, stage));
        if ok {
            self.coop_scene_stage(quest);
        }
        ok
    }

    /// Fast travel hook: true when blocked.
    pub fn coop_fast_travel_blocked(&self) -> bool {
        self.coop
            .gate
            .as_ref()
            .is_some_and(|g| !g.fast_travel_ok(self))
    }

    /// Save hook: true when blocked.
    pub fn coop_save_blocked(&self) -> bool {
        self.coop.gate.as_ref().is_some_and(|g| !g.save_ok(self))
    }

    /// Companion recruit hook: true when the recruit may happen.
    pub fn coop_recruit_ok(&mut self, who: FormId) -> bool {
        self.with_gate(true, |g, gs| {
            let critter = g.critters.contains(&who);
            !matches!(
                g.companion(gs, CompanionOp::Recruit, FormId(0), critter),
                Decision::Reject { .. }
            )
        })
    }

    /// Routes a reward through the gate (no gate: the live player gets it).
    pub fn coop_reward(
        &mut self,
        order: &LoadOrder,
        quest: FormId,
        id: u32,
        grant: Grant,
    ) -> Option<RewardOutcome> {
        let mut gate = self.coop.gate.take()?;
        let r = gate.route_reward(self, order, quest, id, grant);
        self.coop.gate = Some(gate);
        Some(r)
    }
}
