//! Pure rules engine: `decide(table, event, state) -> Decision`.
//!
//! No clocks, no randomness, no hash-order dependence. Everything time related arrives in
//! `State` as elapsed seconds, so the same inputs always give the same answer and every input
//! gets an answer (`Decision::Reject` is an answer).

use crate::load::PolicyTable;
use crate::model::*;

/// Player slot. Slot 0 is the host player.
pub type Slot = u8;

/// Up to eight slots as a bit set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct PlayerMask(pub u8);

impl PlayerMask {
    pub const NONE: PlayerMask = PlayerMask(0);
    pub fn of(slot: Slot) -> PlayerMask {
        if slot < 8 {
            PlayerMask(1 << slot)
        } else {
            PlayerMask(0)
        }
    }
    pub fn contains(self, slot: Slot) -> bool {
        slot < 8 && self.0 & (1 << slot) != 0
    }
    pub fn and(self, o: PlayerMask) -> PlayerMask {
        PlayerMask(self.0 & o.0)
    }
    pub fn minus(self, o: PlayerMask) -> PlayerMask {
        PlayerMask(self.0 & !o.0)
    }
    pub fn or(self, o: PlayerMask) -> PlayerMask {
        PlayerMask(self.0 | o.0)
    }
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub fn is_subset_of(self, o: PlayerMask) -> bool {
        self.0 & !o.0 == 0
    }
}

/// Who caused the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// A script, timer or system running on the host.
    Host,
    /// An intent sent by a player (guests, or the host player acting as a player).
    Player(Slot),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardKind {
    Caps,
    Xp,
    Item,
    QuestItem,
    Perk(String),
    Karma,
    Reputation,
    /// Cost taken from the player who speaks (hand-ins, bribes).
    Payment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompanionOp {
    Recruit,
    Dismiss,
    Wait,
    Follow,
    Trade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneKind {
    Ending,
    TimeSkip,
    Cinematic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    SetStage {
        stage: u32,
    },
    Reward {
        reward_id: u32,
        what: RewardKind,
    },
    ShadowWrite {
        class: ShadowClass,
        slot: Slot,
        var: Option<String>,
    },
    Companion {
        op: CompanionOp,
        owner: Slot,
        critter: bool,
    },
    Scene {
        kind: SceneKind,
    },
    /// An irreversible fork put to the party.
    Choice {
        choice: u32,
    },
    /// A confirmation line ("I'm ready") that needs every player.
    ReadyRequest {
        info: u32,
    },
    PlayerDown {
        slot: Slot,
    },
    FastTravel,
    Save,
    /// A guest asks to run a script locally. `script` is a form id.
    GuestScript {
        script: FormId,
    },
    /// A player hit `victim`, given as aliases joined by `|` (editor id and 8-digit hex);
    /// `recent` earlier hits by that player fell inside the window.
    Hit {
        victim: String,
        recent: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub quest: FormId,
    pub origin: Origin,
    pub kind: EventKind,
}

/// Session facts the engine needs. The caller (host) keeps these current; the engine never mutates them.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    /// Players counted for gates. Late joiners are not in here until the scene ends.
    pub present: PlayerMask,
    /// Player who started the current dialogue or activation for this quest.
    pub initiator: Option<Slot>,
    /// Players currently inside the quest's tether zone.
    pub in_zone: PlayerMask,
    pub tether_elapsed_s: u32,
    pub tether_mode: TetherMode,
    pub barrier_passed: PlayerMask,
    pub barrier_elapsed_s: u32,
    pub ready_yes: PlayerMask,
    pub ready_no: PlayerMask,
    pub ready_elapsed_s: u32,
    pub votes_yes: PlayerMask,
    pub votes_no: PlayerMask,
    pub vote_elapsed_s: u32,
    /// Stages of this quest already committed (for exclusive stages).
    pub committed_stages: Vec<u32>,
    /// Quests already committed in an exclusive group.
    pub committed_quests: Vec<FormId>,
    /// (reward id, slot) pairs already granted.
    pub granted: Vec<(u32, Slot)>,
    /// Quests whose scene currently holds fast-travel and save locks.
    pub active_scenes: Vec<FormId>,
    /// (humanoid, critter) followers per slot.
    pub followers: [(u8, u8); 8],
    /// The once-per-event effect (time skip) already ran inside its window.
    pub event_done: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TetherMode {
    /// Tethers hold the stage and may warp stragglers.
    #[default]
    On,
    /// Tethers never hold anybody.
    Warn,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarrierKind {
    Tether,
    AllPlayersPass,
    ReadyCheck,
    Consent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanOut {
    /// Every recipient gets the full amount.
    Full,
    /// The amount is divided between recipients.
    Split,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reject {
    NotHost,
    NotInitiator,
    NotOwner,
    NotPresent,
    /// Another stage of the same quest was committed first.
    StageExcluded,
    /// A rival quest of an exclusive group was committed first.
    GroupCommitted,
    AlreadyGranted,
    AlreadyDone,
    FastTravelLocked,
    SaveSuppressed,
    FollowerCap,
    PartyCap,
    Declined,
    TimedOut,
    GuestScriptSuppressed,
    /// A deliberate hit on a protected actor after the allowed number.
    FriendlyFire,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Apply once on the host to the shared story state; normal delta replication follows.
    Apply,
    /// Deliver the effect to these players (rewards, quest items, payment, scenes).
    Replicate {
        recipients: PlayerMask,
        mode: FanOut,
    },
    /// Hold until the listed players have arrived, passed, or answered.
    DeferToBarrier {
        barrier: BarrierKind,
        waiting: PlayerMask,
        timeout_s: u32,
    },
    /// The tether wait ran out: warp the stragglers into the zone, then apply.
    GatherThenApply {
        stragglers: PlayerMask,
    },
    /// Write to this player's private copy only.
    PerPlayerShadow {
        slot: Slot,
        class: ShadowClass,
    },
    Reject {
        reason: Reject,
    },
}

fn reject(r: Reject) -> Decision {
    Decision::Reject { reason: r }
}

/// Convenience free function.
pub fn decide(table: &PolicyTable, event: &Event, state: &State) -> Decision {
    table.decide(event, state)
}

/// Two form references name the same form: `plugin:id` against 8-digit hex compare by local id.
pub fn same_form(a: &str, b: &str) -> bool {
    match (local_id(a), local_id(b)) {
        (Some(x), Some(y)) => x == y,
        _ => a.eq_ignore_ascii_case(b),
    }
}

fn local_id(s: &str) -> Option<u32> {
    let h = s.rsplit(':').next()?;
    u32::from_str_radix(h, 16).ok().map(|v| v & 0x00FF_FFFF)
}

impl PolicyTable {
    pub fn decide(&self, event: &Event, state: &State) -> Decision {
        match self.decide_inner(event, state) {
            // Nobody to deliver to is a refusal, not an empty success.
            Decision::Replicate { recipients, .. } if recipients.is_empty() => {
                reject(Reject::NotPresent)
            }
            d => d,
        }
    }

    fn decide_inner(&self, event: &Event, state: &State) -> Decision {
        let fallback;
        let row = match self.row(event.quest) {
            Some(r) => r,
            None => {
                fallback = QuestRow::fallback(event.quest);
                &fallback
            }
        };
        // A player who is not counted (late joiner, gone) cannot act.
        if let Origin::Player(s) = event.origin {
            if !state.present.contains(s) {
                return reject(Reject::NotPresent);
            }
        }
        match &event.kind {
            EventKind::SetStage { stage } => self.stage(row, event, state, *stage),
            EventKind::Reward { reward_id, what } => {
                self.reward(row, event, state, *reward_id, what)
            }
            EventKind::ShadowWrite { class, slot, var } => {
                self.shadow(row, event, state, *class, *slot, var.as_deref())
            }
            EventKind::Companion { op, owner, critter } => {
                self.companion(row, event, state, *op, *owner, *critter)
            }
            EventKind::Scene { kind } => self.scene(row, event, state, *kind),
            EventKind::Choice { .. } => self.choice(row, event, state),
            EventKind::ReadyRequest { .. } => self.ready(row, state),
            EventKind::PlayerDown { slot } => match &row.shadow {
                Some(sh) if sh.down.is_some() => Decision::PerPlayerShadow {
                    slot: *slot,
                    class: if sh.scope == "collar" {
                        ShadowClass::Collar
                    } else {
                        ShadowClass::QuestVar
                    },
                },
                _ => Decision::Apply,
            },
            EventKind::FastTravel => {
                if self.scene_flag(row, state, |r| r.fast_travel_lock) {
                    reject(Reject::FastTravelLocked)
                } else {
                    Decision::Apply
                }
            }
            EventKind::Save => {
                if self.scene_flag(row, state, |r| r.save_suppress) {
                    reject(Reject::SaveSuppressed)
                } else {
                    Decision::Apply
                }
            }
            EventKind::Hit { victim, recent } => self.friendly_fire(row, victim, *recent),
            EventKind::GuestScript { script } => match event.origin {
                Origin::Host => Decision::Apply,
                Origin::Player(_) => {
                    let ok = row
                        .guest_local
                        .iter()
                        .filter_map(|s| local_id(s))
                        .any(|l| l == script & 0x00FF_FFFF);
                    if ok {
                        Decision::Apply
                    } else {
                        reject(Reject::GuestScriptSuppressed)
                    }
                }
            },
        }
    }

    fn scene_flag(&self, row: &QuestRow, state: &State, f: impl Fn(&QuestRow) -> bool) -> bool {
        if f(row) && state.active_scenes.contains(&row.id) {
            return true;
        }
        state
            .active_scenes
            .iter()
            .any(|id| self.row(*id).map(&f).unwrap_or(false))
    }

    fn authorize(&self, row: &QuestRow, origin: Origin, state: &State) -> Option<Decision> {
        if row.gate_exempt {
            return None;
        }
        match (row.advance, origin) {
            (_, Origin::Host) => None,
            (Advance::Any, _) => None,
            (Advance::Host, Origin::Player(_)) => Some(reject(Reject::NotHost)),
            (Advance::Initiator, Origin::Player(s)) => match state.initiator {
                Some(i) if i != s => Some(reject(Reject::NotInitiator)),
                _ => None,
            },
        }
    }

    fn rivals_committed(&self, row: &QuestRow, state: &State) -> bool {
        self.group_rivals(row.id)
            .iter()
            .any(|r| state.committed_quests.contains(r))
    }

    fn stage(&self, row: &QuestRow, event: &Event, state: &State, stage: u32) -> Decision {
        if let Some(d) = self.authorize(row, event.origin, state) {
            return d;
        }
        if row.gate_exempt {
            return Decision::Apply;
        }
        if self.rivals_committed(row, state) {
            return reject(Reject::GroupCommitted);
        }
        if let Some(x) = &row.exclusive {
            if x.stages.contains(&stage)
                && state
                    .committed_stages
                    .iter()
                    .any(|c| *c != stage && x.stages.contains(c))
            {
                return reject(Reject::StageExcluded);
            }
        }
        // Per-player quest: the stage is that player's own.
        if row.scope == Scope::PerPlayer {
            let who = match (event.origin, state.initiator) {
                (Origin::Player(s), _) => Some(s),
                (Origin::Host, Some(i)) => Some(i),
                _ => None,
            };
            return match who {
                Some(slot) => Decision::PerPlayerShadow {
                    slot,
                    class: ShadowClass::QuestStage,
                },
                None => Decision::Replicate {
                    recipients: state.present,
                    mode: FanOut::Full,
                },
            };
        }
        if let Some(d) = self.tether_gate(row, state, Some(stage)) {
            return d;
        }
        if let Some(b) = &row.barrier {
            if b.stages.contains(&stage) {
                let missing = state.present.minus(state.barrier_passed);
                if !missing.is_empty() {
                    let t = self.policy().barrier_timeout_s;
                    if state.barrier_elapsed_s >= t {
                        return reject(Reject::TimedOut);
                    }
                    return Decision::DeferToBarrier {
                        barrier: BarrierKind::AllPlayersPass,
                        waiting: missing,
                        timeout_s: t,
                    };
                }
            }
        }
        Decision::Apply
    }

    fn tether_gate(&self, row: &QuestRow, state: &State, stage: Option<u32>) -> Option<Decision> {
        let t = row.tether.as_ref()?;
        if state.tether_mode != TetherMode::On {
            return None;
        }
        if let Some(s) = stage {
            if !t.stages.is_empty() && !t.stages.contains(&s) {
                return None;
            }
        }
        let missing = state.present.minus(state.in_zone);
        if missing.is_empty() {
            return None;
        }
        let limit = t.timeout_s.unwrap_or(self.policy().tether_timeout_s);
        if state.tether_elapsed_s >= limit && t.gather == Gather::AutoWarp {
            return Some(Decision::GatherThenApply {
                stragglers: missing,
            });
        }
        Some(Decision::DeferToBarrier {
            barrier: BarrierKind::Tether,
            waiting: missing,
            timeout_s: limit,
        })
    }

    fn reward(
        &self,
        row: &QuestRow,
        event: &Event,
        state: &State,
        reward_id: u32,
        what: &RewardKind,
    ) -> Decision {
        if matches!(event.origin, Origin::Player(_)) {
            return reject(Reject::NotHost);
        }
        let initiator_mask = state
            .initiator
            .map(PlayerMask::of)
            .unwrap_or(PlayerMask::NONE);
        let (mut recipients, mode) = match what {
            // Karma and reputation are one value for the party.
            RewardKind::Karma | RewardKind::Reputation => return Decision::Apply,
            RewardKind::Payment => (initiator_mask, FanOut::Full),
            RewardKind::QuestItem => match row.quest_items {
                ItemRoute::All => (state.present, FanOut::Full),
                ItemRoute::Initiator => (initiator_mask, FanOut::Full),
            },
            RewardKind::Perk(p) => {
                let perks = row.perks.as_ref();
                let hit = |l: &Vec<String>| {
                    l.iter()
                        .any(|x| p.split('|').any(|alias| same_form(x, alias)))
                };
                match perks {
                    // The recruiter is the player who ran the recruit conversation.
                    Some(pk) if hit(&pk.initiator) || hit(&pk.recruiter) => {
                        (initiator_mask, FanOut::Full)
                    }
                    _ => (state.present, FanOut::Full),
                }
            }
            RewardKind::Caps | RewardKind::Xp | RewardKind::Item => match row.reward {
                Reward::Everyone => (state.present, FanOut::Full),
                Reward::Initiator => (initiator_mask, FanOut::Full),
                Reward::Split => (state.present, FanOut::Split),
            },
        };
        if recipients.is_empty() {
            return reject(Reject::NotInitiator);
        }
        recipients = recipients.and(state.present);
        if recipients.is_empty() {
            return reject(Reject::NotPresent);
        }
        let mut fresh = PlayerMask::NONE;
        for s in 0..8u8 {
            if recipients.contains(s) && !state.granted.contains(&(reward_id, s)) {
                fresh = fresh.or(PlayerMask::of(s));
            }
        }
        if fresh.is_empty() {
            return reject(Reject::AlreadyGranted);
        }
        Decision::Replicate {
            recipients: fresh,
            mode,
        }
    }

    fn shadow(
        &self,
        row: &QuestRow,
        event: &Event,
        state: &State,
        class: ShadowClass,
        slot: Slot,
        var: Option<&str>,
    ) -> Decision {
        if let Origin::Player(s) = event.origin {
            if s != slot {
                return reject(Reject::NotOwner);
            }
        }
        if slot >= 8 {
            return reject(Reject::NotPresent);
        }
        if let (Some(sh), Some(v)) = (&row.shadow, var) {
            let named = |l: &Vec<String>| l.iter().any(|x| x == v);
            if named(&sh.vars) || named(&sh.objvars) {
                return Decision::PerPlayerShadow {
                    slot,
                    class: ShadowClass::QuestVar,
                };
            }
        }
        let _ = state;
        match self.residence(class) {
            Residence::PerPlayer => Decision::PerPlayerShadow { slot, class },
            Residence::Shared => Decision::Apply,
        }
    }

    fn companion(
        &self,
        row: &QuestRow,
        event: &Event,
        state: &State,
        op: CompanionOp,
        owner: Slot,
        critter: bool,
    ) -> Decision {
        if owner >= 8 {
            return reject(Reject::NotPresent);
        }
        let party_owned = matches!(&row.companion, Some(c) if c.owner == CompanionOwner::Party);
        if party_owned {
            return Decision::Apply;
        }
        if let Origin::Player(s) = event.origin {
            if s != owner {
                return reject(Reject::NotOwner);
            }
        }
        if op == CompanionOp::Recruit {
            let cap = self.policy().follower_cap;
            let (h, c) = state.followers[owner as usize];
            if critter && c >= cap.critter || !critter && h >= cap.humanoid {
                return reject(Reject::FollowerCap);
            }
            let total: u32 = (0..8u8)
                .filter(|s| state.present.contains(*s))
                .map(|s| {
                    state.followers[s as usize].0 as u32 + state.followers[s as usize].1 as u32
                })
                .sum();
            if total >= self.policy().party_cap as u32 {
                return reject(Reject::PartyCap);
            }
        }
        Decision::PerPlayerShadow {
            slot: owner,
            class: ShadowClass::Followers,
        }
    }

    fn scene(&self, row: &QuestRow, event: &Event, state: &State, kind: SceneKind) -> Decision {
        if matches!(event.origin, Origin::Player(_)) {
            return reject(Reject::NotHost);
        }
        if row.gate_exempt {
            return Decision::Apply;
        }
        if self.rivals_committed(row, state) {
            return reject(Reject::GroupCommitted);
        }
        if kind == SceneKind::TimeSkip && row.once.is_some() && state.event_done {
            return reject(Reject::AlreadyDone);
        }
        if let Some(d) = self.tether_gate(row, state, None) {
            return d;
        }
        if let Some(b) = &row.barrier {
            if b.before.is_some() || b.stages.is_empty() {
                let missing = state.present.minus(state.barrier_passed);
                if !missing.is_empty() {
                    let t = self.policy().barrier_timeout_s;
                    if state.barrier_elapsed_s >= t {
                        return reject(Reject::TimedOut);
                    }
                    return Decision::DeferToBarrier {
                        barrier: BarrierKind::AllPlayersPass,
                        waiting: missing,
                        timeout_s: t,
                    };
                }
            }
        }
        // Everyone is locked, warped and shown the scene together.
        Decision::Replicate {
            recipients: state.present,
            mode: FanOut::Full,
        }
    }

    fn choice(&self, row: &QuestRow, event: &Event, state: &State) -> Decision {
        let c = match &row.consent {
            Some(c) => c,
            None => {
                return self
                    .authorize(row, event.origin, state)
                    .unwrap_or(Decision::Apply)
            }
        };
        let present = state.present;
        let yes = state.votes_yes.and(present);
        let no = state.votes_no.and(present).minus(yes);
        match c.mode {
            ConsentMode::Host => {
                if event.origin == Origin::Host || matches!(event.origin, Origin::Player(0)) {
                    Decision::Apply
                } else {
                    reject(Reject::NotHost)
                }
            }
            ConsentMode::Initiator => match (event.origin, state.initiator) {
                (Origin::Host, _) => Decision::Apply,
                (Origin::Player(s), Some(i)) if s == i => Decision::Apply,
                _ => reject(Reject::NotInitiator),
            },
            ConsentMode::Unanimous => {
                if !no.is_empty() {
                    reject(Reject::Declined)
                } else if present.is_subset_of(yes) && !present.is_empty() {
                    Decision::Apply
                } else if state.vote_elapsed_s >= c.timeout_s {
                    reject(Reject::TimedOut)
                } else {
                    Decision::DeferToBarrier {
                        barrier: BarrierKind::Consent,
                        waiting: present.minus(yes),
                        timeout_s: c.timeout_s,
                    }
                }
            }
            ConsentMode::Vote => {
                let answered = yes.or(no);
                let all_in = present.is_subset_of(answered);
                if all_in || state.vote_elapsed_s >= c.timeout_s {
                    let (y, n) = (yes.count(), no.count());
                    if y > n || (y == n && y > 0 && yes.contains(0)) {
                        Decision::Apply
                    } else {
                        reject(Reject::Declined)
                    }
                } else {
                    Decision::DeferToBarrier {
                        barrier: BarrierKind::Consent,
                        waiting: present.minus(answered),
                        timeout_s: c.timeout_s,
                    }
                }
            }
        }
    }

    fn ready(&self, row: &QuestRow, state: &State) -> Decision {
        let r = match &row.ready {
            Some(r) => r,
            None => return Decision::Apply,
        };
        let present = state.present;
        if !state.ready_no.and(present).is_empty() {
            return reject(Reject::Declined);
        }
        let yes = state.ready_yes.and(present);
        if present.is_subset_of(yes) {
            return Decision::Apply;
        }
        if state.ready_elapsed_s >= r.timeout_s {
            return reject(Reject::TimedOut);
        }
        Decision::DeferToBarrier {
            barrier: BarrierKind::ReadyCheck,
            waiting: present.minus(yes),
            timeout_s: r.timeout_s,
        }
    }

    fn friendly_fire(&self, row: &QuestRow, victim: &str, recent: u32) -> Decision {
        match &row.friendly_fire {
            Some(ff)
                if ff
                    .actors
                    .iter()
                    .any(|a| victim.split('|').any(|al| same_form(a, al))) =>
            {
                if recent + 1 > ff.deliberate_hits {
                    reject(Reject::FriendlyFire)
                } else {
                    Decision::Apply
                }
            }
            _ => Decision::Apply,
        }
    }

    /// Quests whose friendly-fire guard protects `victim`.
    pub fn guards_for(&self, victim: &str) -> Vec<&QuestRow> {
        self.rows()
            .iter()
            .filter(|r| {
                r.friendly_fire.as_ref().is_some_and(|ff| {
                    ff.actors
                        .iter()
                        .any(|a| victim.split('|').any(|al| same_form(a, al)))
                })
            })
            .collect()
    }
}
