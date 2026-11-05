//! Per-player avatar stream: quantized state sent unreliably at a fixed rate,
//! a reliable parts channel for rarely-changing data, the host hub that
//! validates and relays, and (in [`interp`]) the receiver-side buffer.

pub mod interp;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ids::{CellId, FormId, PlayerId, Seq};

pub use interp::{InterpBuffer, Sample};

/// Milliseconds between avatar state sends (20 Hz).
pub const SEND_INTERVAL_MS: u64 = 50;

/// The host's bound on a guest's movement (#74). A sample may move as far as
/// the guest's run speed ([`AvatarHub::set_speed`]) × this covers in the time
/// since the last sample, from a credit that keeps at most
/// [`MOVE_CREDIT_MS`] of time (samples that arrive together after a stall).
pub const MOVE_SLACK: f32 = 1.5;
/// Units the credit holds on top of the speed (steps, slopes, pushes).
pub const MOVE_GRACE: f32 = 128.0;
/// The most time a guest's movement credit keeps.
pub const MOVE_CREDIT_MS: u64 = 2_000;
/// Samples within which a player may jump after a warp the host sent (the
/// load before the first sample from the new place).
pub const WARP_SAMPLES: u32 = 1_200;
/// Samples after a warp's jump that may still jump (the party ring's spread).
pub const WARP_AFTER: u32 = 40;
/// How long a guest's samples may move too far before the host sends the
/// guest back to its last accepted place. The wait lets a harness warp
/// (`crate::roam::RoamMsg::TestWarp`) that arrives after its first samples
/// still open the window.
pub const CORRECT_AFTER_MS: u64 = 1_000;
/// Position fixed point: 16 steps per world unit.
pub const POS_Q: f32 = 16.0;
/// Velocity fixed point: 4 steps per unit per second.
pub const VEL_Q: f32 = 4.0;
/// Most equipment entries in one parts message.
pub const MAX_EQUIP: usize = 32;
/// Largest appearance blob.
pub const MAX_APPEARANCE: usize = 512;

/// Avatar flag: downed, waiting for a revive.
pub const FLAG_DOWNED: u16 = 1;
/// Avatar flag: sneaking.
pub const FLAG_SNEAK: u16 = 2;
/// Avatar flag: weapon drawn.
pub const FLAG_DRAWN: u16 = 4;
/// Avatar flag: dead.
pub const FLAG_DEAD: u16 = 8;

/// Quantizes a world coordinate.
pub fn quant_pos(v: f32) -> i32 {
    (v * POS_Q).round() as i32
}

/// Inverse of [`quant_pos`].
pub fn dequant_pos(q: i32) -> f32 {
    q as f32 / POS_Q
}

/// Quantizes a heading in radians to a full-circle u16.
pub fn quant_heading(rad: f32) -> u16 {
    let t = rad.rem_euclid(std::f32::consts::TAU) / std::f32::consts::TAU;
    ((t * 65536.0) as u32 & 0xFFFF) as u16
}

/// Inverse of [`quant_heading`] (0..TAU).
pub fn dequant_heading(q: u16) -> f32 {
    q as f32 / 65536.0 * std::f32::consts::TAU
}

/// Quantizes a velocity component (units per second), saturating.
pub fn quant_vel(v: f32) -> i16 {
    (v * VEL_Q).round().clamp(i16::MIN as f32, i16::MAX as f32) as i16
}

/// One avatar sample: everything that changes every few frames.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvatarState {
    /// Cell the avatar stands in.
    pub cell: CellId,
    /// Position, [`POS_Q`] fixed point.
    pub pos: [i32; 3],
    /// Heading, full circle in 65536 steps.
    pub heading: u16,
    /// Velocity, [`VEL_Q`] fixed point.
    pub vel: [i16; 3],
    /// Current animation state id.
    pub anim: u16,
    /// Wielded weapon form (0 = none).
    pub weapon: u32,
    /// Health points (saturating).
    pub health: u16,
    /// `FLAG_*` bits.
    pub flags: u16,
    /// The furniture reference the avatar is using (sitting down, seated or
    /// getting up; 0 = none), so the other machines pose it in the seat and
    /// mark the furniture's marker as taken.
    pub seat: u32,
}

/// Rarely-changing avatar data, versioned by `rev`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvatarParts {
    /// Increases with every change; receivers keep the highest.
    pub rev: u32,
    /// Equipped forms by slot.
    pub equipment: Vec<(u8, FormId)>,
    /// Opaque appearance blob (race, face, ...).
    pub appearance: Vec<u8>,
}

/// Most undelivered action events a receiver keeps per player.
pub const MAX_PENDING_ACTIONS: usize = 32;

/// What a combat action event says happened.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionKind {
    /// The weapon (or fists) came out; `arg` is the weapon form (0 fists).
    Draw,
    /// The weapon was put away.
    Holster,
    /// A melee swing began; `arg` is the animation group.
    Swing,
    /// A shot was fired; `arg` is the animation group.
    Shoot,
    /// Looking down the sights began.
    Aim,
    /// A reload began; `arg` is the animation group.
    Reload,
    /// Blocking began.
    Block,
    /// Took a hit; `arg` is the damage in whole points.
    Hurt,
    /// Knocked down.
    Knockdown,
    /// Died.
    Death,
}

impl ActionKind {
    /// The kind a bot word names (`swing`, `shoot`, `draw`, ...), case
    /// blind; `attack` is a swing.
    pub fn parse(word: &str) -> Option<ActionKind> {
        Some(match word.to_ascii_lowercase().as_str() {
            "draw" => ActionKind::Draw,
            "holster" => ActionKind::Holster,
            "swing" | "attack" => ActionKind::Swing,
            "shoot" | "fire" => ActionKind::Shoot,
            "aim" => ActionKind::Aim,
            "reload" => ActionKind::Reload,
            "block" => ActionKind::Block,
            "hurt" => ActionKind::Hurt,
            "knockdown" => ActionKind::Knockdown,
            "death" | "die" => ActionKind::Death,
            _ => return None,
        })
    }
}

/// One combat action event with its per-player sequence number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvatarAction {
    /// Per-player event number, rising by one per event from 1.
    pub n: Seq,
    /// What happened.
    pub kind: ActionKind,
    /// Kind-specific argument.
    pub arg: u32,
}

/// Receiver side of the action channel: events in order, once each, with
/// the count of ones that never came (a reliable channel loses none; a
/// lossy one drops some, and old or repeated ones are ignored).
#[derive(Clone, Debug, Default)]
pub struct ActionRx {
    last: Option<Seq>,
    pending: std::collections::VecDeque<AvatarAction>,
    /// Events accepted.
    pub accepted: u32,
    /// Numbers skipped over (events that never arrived).
    pub missed: u32,
    /// Repeats and stale ones dropped.
    pub dropped: u32,
}

impl ActionRx {
    /// Takes one event; true when it was new.
    pub fn push(&mut self, a: AvatarAction) -> bool {
        if let Some(last) = self.last {
            if !a.n.is_newer_than(last) {
                self.dropped += 1;
                return false;
            }
            self.missed += a.n.0.wrapping_sub(last.0).saturating_sub(1).min(1000);
        }
        self.last = Some(a.n);
        self.accepted += 1;
        if self.pending.len() >= MAX_PENDING_ACTIONS {
            self.pending.pop_front();
        }
        self.pending.push_back(a);
        true
    }

    /// The events not yet taken, oldest first.
    pub fn take(&mut self) -> Vec<AvatarAction> {
        self.pending.drain(..).collect()
    }
}

/// Avatar wire messages.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvatarMsg {
    /// Unreliable state sample.
    State {
        /// Whose avatar.
        player: PlayerId,
        /// Per-sender sample number.
        seq: Seq,
        /// The sample.
        state: AvatarState,
    },
    /// Reliable parts update.
    Parts {
        /// Whose avatar.
        player: PlayerId,
        /// The parts.
        parts: AvatarParts,
    },
    /// Reliable combat action event (added after protocol 2's first
    /// messages; an old peer cannot decode it, hence the version bump).
    Action {
        /// Whose avatar.
        player: PlayerId,
        /// The event.
        action: AvatarAction,
    },
}

impl AvatarMsg {
    /// True for the unreliable stream message.
    pub fn is_stream(&self) -> bool {
        matches!(self, AvatarMsg::State { .. })
    }

    /// Structural bounds.
    pub fn is_sane(&self) -> bool {
        match self {
            AvatarMsg::State { .. } | AvatarMsg::Action { .. } => true,
            AvatarMsg::Parts { parts, .. } => {
                parts.equipment.len() <= MAX_EQUIP && parts.appearance.len() <= MAX_APPEARANCE
            }
        }
    }
}

/// Sends this player's own avatar at a fixed rate.
#[derive(Clone, Debug)]
pub struct AvatarSender {
    player: PlayerId,
    seq: Seq,
    next_at: u64,
    parts_rev: u32,
    action_n: u32,
}

impl AvatarSender {
    /// A sender for `player`.
    pub fn new(player: PlayerId) -> Self {
        AvatarSender {
            player,
            seq: Seq(0),
            next_at: 0,
            parts_rev: 0,
            action_n: 0,
        }
    }

    /// Emits a state message when the interval has elapsed.
    pub fn tick(&mut self, now_ms: u64, state: &AvatarState) -> Option<AvatarMsg> {
        if now_ms < self.next_at {
            return None;
        }
        // keep the cadence after a late call, but never burst to catch up
        self.next_at = if now_ms >= self.next_at + SEND_INTERVAL_MS {
            now_ms + SEND_INTERVAL_MS
        } else {
            self.next_at + SEND_INTERVAL_MS
        };
        self.seq = self.seq.next();
        Some(AvatarMsg::State {
            player: self.player,
            seq: self.seq,
            state: *state,
        })
    }

    /// A combat action event under the next event number.
    pub fn action(&mut self, kind: ActionKind, arg: u32) -> AvatarMsg {
        self.action_n = self.action_n.wrapping_add(1);
        AvatarMsg::Action {
            player: self.player,
            action: AvatarAction {
                n: Seq(self.action_n),
                kind,
                arg,
            },
        }
    }

    /// Publishes new parts under the next revision.
    pub fn set_parts(&mut self, equipment: Vec<(u8, FormId)>, appearance: Vec<u8>) -> AvatarMsg {
        self.parts_rev += 1;
        AvatarMsg::Parts {
            player: self.player,
            parts: AvatarParts {
                rev: self.parts_rev,
                equipment,
                appearance,
            },
        }
    }
}

/// What the hub accepted and where it should go.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relay {
    /// Message to forward.
    pub msg: AvatarMsg,
    /// Cell whose interested peers get it (`None` = everyone).
    pub cell: Option<CellId>,
}

/// A guest's run speed and movement credit, both in game units.
#[derive(Clone, Copy, Debug)]
struct Pace {
    speed: f32,
    credit: f32,
    at_ms: u64,
}

impl Pace {
    fn cap(speed: f32) -> f32 {
        speed * MOVE_SLACK * MOVE_CREDIT_MS as f32 / 1000.0 + MOVE_GRACE
    }
}

/// Host-side table of every avatar's newest state and parts.
#[derive(Clone, Debug, Default)]
pub struct AvatarHub {
    states: BTreeMap<PlayerId, (Seq, AvatarState)>,
    parts: BTreeMap<PlayerId, AvatarParts>,
    actions: BTreeMap<PlayerId, Seq>,
    paces: BTreeMap<PlayerId, Pace>,
    warps: BTreeMap<PlayerId, u32>,
    /// Per player: when its samples started to move too far (none accepted since).
    refused_since: BTreeMap<PlayerId, u64>,
    /// Per player: its last accepted sample with a place. The movement check
    /// starts there, and a correction sends the player back there.
    anchors: BTreeMap<PlayerId, AvatarState>,
    /// Samples dropped because they moved farther than the sender's speed
    /// allows, with no warp from the host (#74).
    pub too_far: u32,
    /// Messages dropped because the claimed player was not the sender.
    pub forged: u32,
    /// Stale or duplicate messages dropped.
    pub stale: u32,
}

impl AvatarHub {
    /// Empty hub.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the run speed of player `p` in game units a second (the host's
    /// copy of that player). Without one, the player's movement is not bound.
    pub fn set_speed(&mut self, p: PlayerId, speed: f32, now_ms: u64) {
        let pace = self.paces.entry(p).or_insert(Pace {
            speed,
            credit: Pace::cap(speed),
            at_ms: now_ms,
        });
        pace.speed = speed;
    }

    /// The host sent `p` a warp (a door, the tether, a respawn, a placement,
    /// a fast travel): its next samples may jump.
    pub fn open_warp(&mut self, p: PlayerId) {
        self.warps.insert(p, WARP_SAMPLES);
    }

    /// Whether `state` moves within the pace of `from` since its last
    /// accepted sample (horizontal distance; a jump needs a warp).
    fn within_pace(&mut self, from: PlayerId, state: &AvatarState, now_ms: u64) -> bool {
        // no place (cell 0): the player loads. Not checked, and not the anchor.
        if state.cell == CellId(0) {
            return true;
        }
        let ok = self.paced(from, state, now_ms);
        if ok {
            self.anchors.insert(from, *state);
        }
        ok
    }

    /// [`Self::within_pace`] for a sample with a place.
    fn paced(&mut self, from: PlayerId, state: &AvatarState, now_ms: u64) -> bool {
        let warp = self.warps.get(&from).copied().unwrap_or(0);
        if warp > 1 {
            self.warps.insert(from, warp - 1);
        } else {
            self.warps.remove(&from);
        }
        let (Some(old), Some(pace)) = (self.anchors.get(&from), self.paces.get_mut(&from)) else {
            return true;
        };
        let dt = now_ms.saturating_sub(pace.at_ms).min(MOVE_CREDIT_MS) as f32 / 1000.0;
        pace.at_ms = now_ms;
        let cap = Pace::cap(pace.speed);
        pace.credit = (pace.credit + pace.speed * MOVE_SLACK * dt).min(cap);
        let d = (0..2)
            .map(|i| dequant_pos(state.pos[i]) - dequant_pos(old.pos[i]))
            .map(|v| v * v)
            .sum::<f32>()
            .sqrt();
        if d <= pace.credit {
            pace.credit -= d;
            return true;
        }
        if warp > 0 {
            pace.credit = cap;
            self.warps.insert(from, (warp - 1).min(WARP_AFTER));
            return true;
        }
        false
    }

    /// The place to send `p` back to (its last accepted sample) when its
    /// samples moved too far for [`CORRECT_AFTER_MS`] with no warp. After a
    /// correction, the next one comes no sooner than the same wait.
    pub fn take_correction(&mut self, p: PlayerId, now_ms: u64) -> Option<AvatarState> {
        let since = self.refused_since.get_mut(&p)?;
        if now_ms < *since + CORRECT_AFTER_MS {
            return None;
        }
        *since = now_ms;
        self.anchors.get(&p).copied()
    }

    /// Validates a message from `from` at host time `now_ms`; returns the
    /// relay to perform.
    pub fn on_msg(&mut self, now_ms: u64, from: PlayerId, msg: AvatarMsg) -> Option<Relay> {
        match &msg {
            AvatarMsg::State { player, seq, state } => {
                if *player != from {
                    self.forged += 1;
                    return None;
                }
                if let Some((old, _)) = self.states.get(&from) {
                    // a long backwards jump means the sender restarted
                    let restarted = old.0.wrapping_sub(seq.0) > 1000 && !seq.is_newer_than(*old);
                    if !seq.is_newer_than(*old) && !restarted {
                        self.stale += 1;
                        return None;
                    }
                }
                if !self.within_pace(from, state, now_ms) {
                    self.too_far += 1;
                    self.refused_since.entry(from).or_insert(now_ms);
                    return None;
                }
                self.refused_since.remove(&from);
                self.states.insert(from, (*seq, *state));
                Some(Relay {
                    cell: Some(state.cell),
                    msg,
                })
            }
            AvatarMsg::Parts { player, parts } => {
                if *player != from {
                    self.forged += 1;
                    return None;
                }
                if self.parts.get(&from).is_some_and(|p| parts.rev <= p.rev) {
                    self.stale += 1;
                    return None;
                }
                self.parts.insert(from, parts.clone());
                Some(Relay { cell: None, msg })
            }
            AvatarMsg::Action { player, action } => {
                if *player != from {
                    self.forged += 1;
                    return None;
                }
                if self
                    .actions
                    .get(&from)
                    .is_some_and(|old| !action.n.is_newer_than(*old))
                {
                    self.stale += 1;
                    return None;
                }
                self.actions.insert(from, action.n);
                // to the ones who can see them; everyone until a position is known
                Some(Relay {
                    cell: self.states.get(&from).map(|(_, s)| s.cell),
                    msg,
                })
            }
        }
    }

    /// Newest state of a player.
    pub fn state(&self, p: PlayerId) -> Option<&AvatarState> {
        self.states.get(&p).map(|(_, s)| s)
    }

    /// Parts messages for everyone known, for a peer that just joined.
    pub fn parts_snapshot(&self) -> Vec<AvatarMsg> {
        self.parts
            .iter()
            .map(|(p, parts)| AvatarMsg::Parts {
                player: *p,
                parts: parts.clone(),
            })
            .collect()
    }

    /// Forgets a player.
    pub fn remove(&mut self, p: PlayerId) {
        self.states.remove(&p);
        self.parts.remove(&p);
        self.actions.remove(&p);
    }

    /// Distance between two avatars; `None` when either is unknown or they
    /// stand in different cells.
    pub fn distance(&self, a: PlayerId, b: PlayerId) -> Option<f32> {
        let (sa, sb) = (self.state(a)?, self.state(b)?);
        (sa.cell == sb.cell).then(|| {
            let d: f32 = (0..3)
                .map(|i| dequant_pos(sa.pos[i] - sb.pos[i]).powi(2))
                .sum();
            d.sqrt()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn st(cell: u32, x: f32) -> AvatarState {
        AvatarState {
            cell: CellId(cell),
            pos: [quant_pos(x), 0, 0],
            ..Default::default()
        }
    }

    proptest! {
        #[test]
        fn pos_roundtrip_error_bounded(v in -100_000f32..100_000f32) {
            prop_assert!((dequant_pos(quant_pos(v)) - v).abs() <= 0.5 / POS_Q + 1e-3);
        }

        #[test]
        fn heading_roundtrip_error_bounded(r in -20f32..20f32) {
            let back = dequant_heading(quant_heading(r));
            let want = r.rem_euclid(std::f32::consts::TAU);
            let mut d = (back - want).abs();
            d = d.min(std::f32::consts::TAU - d);
            prop_assert!(d < 2e-4, "{r} {back}");
        }
    }

    #[test]
    fn vel_saturates() {
        assert_eq!(quant_vel(1e9), i16::MAX);
        assert_eq!(quant_vel(-1e9), i16::MIN);
        assert_eq!(quant_vel(10.0), 40);
    }

    #[test]
    fn sender_is_fixed_rate_with_rising_seq() {
        let mut s = AvatarSender::new(PlayerId(1));
        let mut seqs = vec![];
        for t in (0..1000).step_by(10) {
            if let Some(AvatarMsg::State { seq, .. }) = s.tick(t, &AvatarState::default()) {
                seqs.push((t, seq));
            }
        }
        assert_eq!(seqs.len(), 20);
        assert!(seqs
            .windows(2)
            .all(|w| w[1].1 == w[0].1.next() && w[1].0 - w[0].0 == 50));
    }

    #[test]
    fn sender_does_not_burst_after_stall() {
        let mut s = AvatarSender::new(PlayerId(1));
        assert!(s.tick(0, &AvatarState::default()).is_some());
        assert!(s.tick(5000, &AvatarState::default()).is_some());
        assert!(s.tick(5010, &AvatarState::default()).is_none());
    }

    #[test]
    fn hub_rejects_forged_stale_and_relays_by_cell() {
        let mut h = AvatarHub::new();
        let m = |p: u8, q: u32, c: u32| AvatarMsg::State {
            player: PlayerId(p),
            seq: Seq(q),
            state: st(c, 1.0),
        };
        assert!(h.on_msg(0, PlayerId(1), m(2, 1, 5)).is_none());
        assert_eq!(h.forged, 1);
        let r = h.on_msg(0, PlayerId(1), m(1, 3, 5)).unwrap();
        assert_eq!(r.cell, Some(CellId(5)));
        assert!(h.on_msg(0, PlayerId(1), m(1, 3, 5)).is_none());
        assert!(h.on_msg(0, PlayerId(1), m(1, 2, 5)).is_none());
        assert_eq!(h.stale, 2);
        assert!(h.on_msg(0, PlayerId(1), m(1, 4, 6)).is_some());
    }

    #[test]
    fn hub_accepts_restarted_sender() {
        let mut h = AvatarHub::new();
        let m = |q: u32| AvatarMsg::State {
            player: PlayerId(1),
            seq: Seq(q),
            state: st(1, 0.0),
        };
        assert!(h.on_msg(0, PlayerId(1), m(50_000)).is_some());
        assert!(h.on_msg(0, PlayerId(1), m(1)).is_some());
        assert!(h.on_msg(0, PlayerId(1), m(2)).is_some());
    }

    #[test]
    fn parts_revisions_only_move_forward() {
        let mut s = AvatarSender::new(PlayerId(1));
        let mut h = AvatarHub::new();
        let a = s.set_parts(vec![(0, FormId(5))], vec![1, 2]);
        let b = s.set_parts(vec![(0, FormId(6))], vec![1, 2]);
        assert!(h.on_msg(0, PlayerId(1), b.clone()).is_some());
        assert!(
            h.on_msg(0, PlayerId(1), a).is_none(),
            "older revision after newer"
        );
        assert!(h.on_msg(0, PlayerId(1), b).is_none(), "duplicate");
        assert_eq!(h.parts_snapshot().len(), 1);
    }

    fn act(n: u32) -> AvatarAction {
        AvatarAction {
            n: Seq(n),
            kind: ActionKind::Swing,
            arg: n,
        }
    }

    #[test]
    fn action_rx_orders_dedupes_and_counts_the_missing() {
        let mut rx = ActionRx::default();
        // arrival order scrambled, with repeats and one event (4) lost
        for n in [1, 2, 2, 3, 1, 6, 5, 7, 7] {
            rx.push(act(n));
        }
        let got: Vec<u32> = rx.take().iter().map(|a| a.n.0).collect();
        assert_eq!(
            got,
            vec![1, 2, 3, 6, 7],
            "in order, once each; the late 5 is stale after 6"
        );
        assert_eq!(rx.accepted, 5);
        assert_eq!(rx.missed, 2, "4 and 5 never counted in order");
        assert_eq!(rx.dropped, 4);
        assert!(rx.take().is_empty());
    }

    #[test]
    fn action_rx_survives_a_wrap_and_bounds_its_queue() {
        let mut rx = ActionRx::default();
        for n in [u32::MAX - 1, u32::MAX, 0, 1] {
            assert!(rx.push(act(n)));
        }
        assert_eq!(rx.take().len(), 4);
        for n in 2..2 + 3 * MAX_PENDING_ACTIONS as u32 {
            rx.push(act(n));
        }
        assert_eq!(rx.take().len(), MAX_PENDING_ACTIONS);
    }

    #[test]
    fn hub_relays_actions_once_by_cell_and_rejects_forgeries() {
        let mut h = AvatarHub::new();
        let mut s = AvatarSender::new(PlayerId(1));
        h.on_msg(
            0,
            PlayerId(1),
            AvatarMsg::State {
                player: PlayerId(1),
                seq: Seq(1),
                state: st(5, 0.0),
            },
        );
        let a = s.action(ActionKind::Shoot, 7);
        assert!(a.is_sane() && !a.is_stream());
        let r = h.on_msg(0, PlayerId(1), a.clone()).unwrap();
        assert_eq!(r.cell, Some(CellId(5)));
        assert!(h.on_msg(0, PlayerId(1), a.clone()).is_none(), "duplicate");
        assert!(h.on_msg(0, PlayerId(2), a).is_none(), "forged");
        assert_eq!(h.forged, 1);
        let b = s.action(ActionKind::Death, 0);
        assert!(matches!(&b, AvatarMsg::Action { action, .. } if action.n == Seq(2)));
        assert!(h.on_msg(0, PlayerId(1), b).is_some());
    }

    #[test]
    fn action_words_parse() {
        assert_eq!(ActionKind::parse("Attack"), Some(ActionKind::Swing));
        assert_eq!(ActionKind::parse("shoot"), Some(ActionKind::Shoot));
        assert_eq!(ActionKind::parse("nope"), None);
    }

    #[test]
    fn distance_requires_same_cell() {
        let mut h = AvatarHub::new();
        for (p, c, x) in [(1u8, 1u32, 0.0f32), (2, 1, 30.0), (3, 2, 30.0)] {
            h.on_msg(
                0,
                PlayerId(p),
                AvatarMsg::State {
                    player: PlayerId(p),
                    seq: Seq(1),
                    state: st(c, x),
                },
            );
        }
        assert!((h.distance(PlayerId(1), PlayerId(2)).unwrap() - 30.0).abs() < 0.1);
        assert_eq!(h.distance(PlayerId(1), PlayerId(3)), None);
        assert_eq!(h.distance(PlayerId(1), PlayerId(9)), None);
    }

    #[test]
    fn sanity_bounds() {
        let big = AvatarMsg::Parts {
            player: PlayerId(1),
            parts: AvatarParts {
                rev: 1,
                equipment: vec![],
                appearance: vec![0; MAX_APPEARANCE + 1],
            },
        };
        assert!(!big.is_sane());
    }

    fn at(q: u32, x: f32) -> AvatarMsg {
        AvatarMsg::State {
            player: PlayerId(1),
            seq: Seq(q),
            state: st(1, x),
        }
    }

    /// #74: a guest moves as far as its speed covers; a jump needs a warp
    /// that the host sent, and the warp's window closes after the jump.
    #[test]
    fn hub_bounds_movement_by_speed_and_lets_warps_jump() {
        let mut h = AvatarHub::new();
        h.set_speed(PlayerId(1), 400.0, 0);
        assert!(h.on_msg(0, PlayerId(1), at(1, 0.0)).is_some());
        // a second at 400 units a second
        for i in 1..=20u32 {
            let x = 20.0 * i as f32;
            assert!(h
                .on_msg(50 * u64::from(i), PlayerId(1), at(1 + i, x))
                .is_some());
        }
        // 5000 units in 50 ms: dropped, the old place stays
        assert!(h.on_msg(1050, PlayerId(1), at(30, 5400.0)).is_none());
        assert_eq!(h.too_far, 1);
        assert_eq!(
            h.state(PlayerId(1)).map(|s| dequant_pos(s.pos[0])),
            Some(400.0)
        );
        // the host sends a warp: the jump goes through
        h.open_warp(PlayerId(1));
        assert!(h.on_msg(1100, PlayerId(1), at(31, 5400.0)).is_some());
        // the ring's spread may still jump for a short time
        assert!(h.on_msg(1150, PlayerId(1), at(32, 5900.0)).is_some());
        for i in 0..WARP_AFTER {
            let _ = h.on_msg(1200 + 50 * u64::from(i), PlayerId(1), at(33 + i, 5900.0));
        }
        let q = 33 + WARP_AFTER;
        assert!(
            h.on_msg(5000, PlayerId(1), at(q, 9900.0)).is_none(),
            "the window closed"
        );
        assert_eq!(h.too_far, 2);
    }

    /// The credit keeps two seconds of movement: samples that arrive
    /// together after a stall go through, a longer run in one sample does not.
    #[test]
    fn hub_credit_covers_a_stall_and_no_more() {
        let mut h = AvatarHub::new();
        h.set_speed(PlayerId(1), 400.0, 0);
        assert!(h.on_msg(0, PlayerId(1), at(1, 0.0)).is_some());
        // 1.5 s of samples arrive at once
        for i in 1..=30u32 {
            assert!(h
                .on_msg(1500, PlayerId(1), at(1 + i, 20.0 * i as f32))
                .is_some());
        }
        // ten seconds later, one sample 8000 units on: more than two seconds' credit
        assert!(h.on_msg(11_500, PlayerId(1), at(40, 8600.0)).is_none());
        // mutation twin: no speed set, no bound
        let mut free = AvatarHub::new();
        assert!(free.on_msg(0, PlayerId(1), at(1, 0.0)).is_some());
        assert!(free.on_msg(50, PlayerId(1), at(2, 9000.0)).is_some());
    }

    /// A guest whose samples keep moving too far goes back to its last
    /// accepted place after the wait. A warp that arrives within the wait
    /// lets the jump through, and no correction follows.
    #[test]
    fn hub_sends_a_far_guest_back_after_the_wait() {
        let mut h = AvatarHub::new();
        h.set_speed(PlayerId(1), 400.0, 0);
        assert!(h.on_msg(0, PlayerId(1), at(1, 0.0)).is_some());
        assert!(h.on_msg(50, PlayerId(1), at(2, 8000.0)).is_none());
        assert_eq!(
            h.take_correction(PlayerId(1), 50),
            None,
            "a late warp may come"
        );
        assert!(h.on_msg(1050, PlayerId(1), at(3, 8000.0)).is_none());
        let back = h.take_correction(PlayerId(1), 1050);
        assert_eq!(back.map(|s| dequant_pos(s.pos[0])), Some(0.0));
        assert_eq!(h.take_correction(PlayerId(1), 1100), None, "one per wait");
        // the guest is back: its samples go through, and no correction waits
        assert!(h.on_msg(1150, PlayerId(1), at(4, 10.0)).is_some());
        assert_eq!(h.take_correction(PlayerId(1), 9000), None);
        // a warp within the wait: the jump goes through, no correction
        assert!(h.on_msg(1200, PlayerId(1), at(5, 9000.0)).is_none());
        h.open_warp(PlayerId(1));
        assert!(h.on_msg(1250, PlayerId(1), at(6, 9000.0)).is_some());
        assert_eq!(h.take_correction(PlayerId(1), 5000), None);
    }

    /// A sample with no place (cell 0, the guest loads) passes and is not
    /// the anchor: the first place after it is checked against the last
    /// place, and the load does not use up a warp's window.
    #[test]
    fn hub_skips_samples_with_no_place() {
        let mut h = AvatarHub::new();
        h.set_speed(PlayerId(1), 400.0, 0);
        let nowhere = |q: u32| AvatarMsg::State {
            player: PlayerId(1),
            seq: Seq(q),
            state: st(0, 0.0),
        };
        // the first samples while loading, then the first place: no anchor yet
        assert!(h.on_msg(0, PlayerId(1), nowhere(1)).is_some());
        assert!(h.on_msg(50, PlayerId(1), at(2, 7000.0)).is_some());
        // a load with no warp: the place after it is checked against 7000
        assert!(h.on_msg(100, PlayerId(1), nowhere(3)).is_some());
        assert!(h.on_msg(150, PlayerId(1), at(4, 20.0)).is_none());
        let back = h.take_correction(PlayerId(1), 5000);
        assert_eq!(back.map(|s| dequant_pos(s.pos[0])), Some(7000.0));
        // a door: the warp outlasts a long load
        h.open_warp(PlayerId(1));
        for q in 0..2 * WARP_SAMPLES {
            let _ = h.on_msg(5050 + 50 * u64::from(q), PlayerId(1), nowhere(5 + q));
        }
        let q = 5 + 2 * WARP_SAMPLES;
        assert!(h.on_msg(200_000, PlayerId(1), at(q, 20.0)).is_some());
    }
}
