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
}

impl AvatarMsg {
    /// True for the unreliable stream message.
    pub fn is_stream(&self) -> bool {
        matches!(self, AvatarMsg::State { .. })
    }

    /// Structural bounds.
    pub fn is_sane(&self) -> bool {
        match self {
            AvatarMsg::State { .. } => true,
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
}

impl AvatarSender {
    /// A sender for `player`.
    pub fn new(player: PlayerId) -> Self {
        AvatarSender { player, seq: Seq(0), next_at: 0, parts_rev: 0 }
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
        Some(AvatarMsg::State { player: self.player, seq: self.seq, state: *state })
    }

    /// Publishes new parts under the next revision.
    pub fn set_parts(&mut self, equipment: Vec<(u8, FormId)>, appearance: Vec<u8>) -> AvatarMsg {
        self.parts_rev += 1;
        AvatarMsg::Parts { player: self.player, parts: AvatarParts { rev: self.parts_rev, equipment, appearance } }
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

/// Host-side table of every avatar's newest state and parts.
#[derive(Clone, Debug, Default)]
pub struct AvatarHub {
    states: BTreeMap<PlayerId, (Seq, AvatarState)>,
    parts: BTreeMap<PlayerId, AvatarParts>,
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

    /// Validates a message from `from`; returns the relay to perform.
    pub fn on_msg(&mut self, from: PlayerId, msg: AvatarMsg) -> Option<Relay> {
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
                self.states.insert(from, (*seq, *state));
                Some(Relay { cell: Some(state.cell), msg })
            }
            AvatarMsg::Parts { player, parts } => {
                if *player != from {
                    self.forged += 1;
                    return None;
                }
                if self.parts.get(&from).map_or(false, |p| parts.rev <= p.rev) {
                    self.stale += 1;
                    return None;
                }
                self.parts.insert(from, parts.clone());
                Some(Relay { cell: None, msg })
            }
        }
    }

    /// Newest state of a player.
    pub fn state(&self, p: PlayerId) -> Option<&AvatarState> {
        self.states.get(&p).map(|(_, s)| s)
    }

    /// Parts messages for everyone known, for a peer that just joined.
    pub fn parts_snapshot(&self) -> Vec<AvatarMsg> {
        self.parts.iter().map(|(p, parts)| AvatarMsg::Parts { player: *p, parts: parts.clone() }).collect()
    }

    /// Forgets a player.
    pub fn remove(&mut self, p: PlayerId) {
        self.states.remove(&p);
        self.parts.remove(&p);
    }

    /// Distance between two avatars; `None` when either is unknown or they
    /// stand in different cells.
    pub fn distance(&self, a: PlayerId, b: PlayerId) -> Option<f32> {
        let (sa, sb) = (self.state(a)?, self.state(b)?);
        (sa.cell == sb.cell).then(|| {
            let d: f32 = (0..3).map(|i| dequant_pos(sa.pos[i] - sb.pos[i]).powi(2)).sum();
            d.sqrt()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn st(cell: u32, x: f32) -> AvatarState {
        AvatarState { cell: CellId(cell), pos: [quant_pos(x), 0, 0], ..Default::default() }
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
        assert!(seqs.windows(2).all(|w| w[1].1 == w[0].1.next() && w[1].0 - w[0].0 == 50));
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
        let m = |p: u8, q: u32, c: u32| AvatarMsg::State { player: PlayerId(p), seq: Seq(q), state: st(c, 1.0) };
        assert!(h.on_msg(PlayerId(1), m(2, 1, 5)).is_none());
        assert_eq!(h.forged, 1);
        let r = h.on_msg(PlayerId(1), m(1, 3, 5)).unwrap();
        assert_eq!(r.cell, Some(CellId(5)));
        assert!(h.on_msg(PlayerId(1), m(1, 3, 5)).is_none());
        assert!(h.on_msg(PlayerId(1), m(1, 2, 5)).is_none());
        assert_eq!(h.stale, 2);
        assert!(h.on_msg(PlayerId(1), m(1, 4, 6)).is_some());
    }

    #[test]
    fn hub_accepts_restarted_sender() {
        let mut h = AvatarHub::new();
        let m = |q: u32| AvatarMsg::State { player: PlayerId(1), seq: Seq(q), state: st(1, 0.0) };
        assert!(h.on_msg(PlayerId(1), m(50_000)).is_some());
        assert!(h.on_msg(PlayerId(1), m(1)).is_some());
        assert!(h.on_msg(PlayerId(1), m(2)).is_some());
    }

    #[test]
    fn parts_revisions_only_move_forward() {
        let mut s = AvatarSender::new(PlayerId(1));
        let mut h = AvatarHub::new();
        let a = s.set_parts(vec![(0, FormId(5))], vec![1, 2]);
        let b = s.set_parts(vec![(0, FormId(6))], vec![1, 2]);
        assert!(h.on_msg(PlayerId(1), b.clone()).is_some());
        assert!(h.on_msg(PlayerId(1), a).is_none(), "older revision after newer");
        assert!(h.on_msg(PlayerId(1), b).is_none(), "duplicate");
        assert_eq!(h.parts_snapshot().len(), 1);
    }

    #[test]
    fn distance_requires_same_cell() {
        let mut h = AvatarHub::new();
        for (p, c, x) in [(1u8, 1u32, 0.0f32), (2, 1, 30.0), (3, 2, 30.0)] {
            h.on_msg(PlayerId(p), AvatarMsg::State { player: PlayerId(p), seq: Seq(1), state: st(c, x) });
        }
        assert!((h.distance(PlayerId(1), PlayerId(2)).unwrap() - 30.0).abs() < 0.1);
        assert_eq!(h.distance(PlayerId(1), PlayerId(3)), None);
        assert_eq!(h.distance(PlayerId(1), PlayerId(9)), None);
    }

    #[test]
    fn sanity_bounds() {
        let big = AvatarMsg::Parts {
            player: PlayerId(1),
            parts: AvatarParts { rev: 1, equipment: vec![], appearance: vec![0; MAX_APPEARANCE + 1] },
        };
        assert!(!big.is_sane());
    }
}
