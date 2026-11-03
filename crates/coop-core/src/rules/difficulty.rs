//! Co-op difficulty scaling. The model and numbers come from the old mod; the
//! server computes the values once, so no machine ever writes a derived
//! value back into game data.

use serde::{Deserialize, Serialize};

use crate::ids::FormId;

/// Time a party-size change takes to blend in.
pub const RAMP_MS: u64 = 10_000;

/// Campaign difficulty property, chosen once by the host.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    /// Single-player numbers regardless of party size.
    Normal,
    /// Scaled for the party.
    CoopBalanced,
}

/// The scaling factors in force.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scaling {
    /// Party focus multiplier, 1 + 0.7 (N-1).
    pub focus: f32,
    /// Enemy toughness: player damage is divided by this, 1 + 0.5 (N-1).
    pub toughness: f32,
    /// Extra enemies per enemy, 0.2 (N-1).
    pub extras: f32,
    /// Most extras alive at once, 2 (N-1).
    pub max_extras_alive: u32,
    /// Damage taken by players, +10 percent per extra player.
    pub damage_to_players: f32,
    /// Kill XP and body supplies: 1 / (1 + extras).
    pub reward: f32,
}

impl Scaling {
    /// No scaling.
    pub const NEUTRAL: Scaling = Scaling {
        focus: 1.0,
        toughness: 1.0,
        extras: 0.0,
        max_extras_alive: 0,
        damage_to_players: 1.0,
        reward: 1.0,
    };

    /// Scaling for `players` in `mode`.
    pub fn for_party(players: u32, mode: Mode) -> Scaling {
        if mode == Mode::Normal || players <= 1 {
            return Self::NEUTRAL;
        }
        let extra = (players - 1) as f32;
        let extras = 0.2 * extra;
        Scaling {
            focus: 1.0 + 0.7 * extra,
            toughness: 1.0 + 0.5 * extra,
            extras,
            max_extras_alive: 2 * (players - 1),
            damage_to_players: 1.0 + 0.1 * extra,
            reward: 1.0 / (1.0 + extras),
        }
    }

    /// Player damage after enemy toughness.
    pub fn damage_dealt(&self, base: f32) -> f32 {
        base / self.toughness
    }

    /// Experience for a kill.
    pub fn xp(&self, base: u32) -> u32 {
        (base as f32 * self.reward).round() as u32
    }

    /// Whether this enemy brings an extra (deterministic per net id, so the
    /// host decides once and every machine could recompute it).
    pub fn spawns_extra(&self, enemy: FormId) -> bool {
        unit_hash(enemy, 0x5EED) < self.extras.min(1.0)
    }

    /// Whether a loot entry of container `id` survives thinning. Stacks are
    /// thinned by probability `reward`; the roll is a hash of the net id so it
    /// is stable and runs once.
    pub fn keeps_loot(&self, id: FormId) -> bool {
        unit_hash(id, 0x100D) < self.reward
    }
}

/// Deterministic value in [0, 1) from a form id and a salt.
fn unit_hash(f: FormId, salt: u32) -> f32 {
    let mut x = f.0 ^ salt.wrapping_mul(0x9E37_79B1);
    x ^= x >> 16;
    x = x.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 13;
    x = x.wrapping_mul(0xC2B2_AE35);
    x ^= x >> 16;
    (x >> 8) as f32 / (1u32 << 24) as f32
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Blends scaling when the party size changes (smoothstep over [`RAMP_MS`]).
#[derive(Clone, Copy, Debug)]
pub struct Ramp {
    mode: Mode,
    players: u32,
    from: Scaling,
    to: Scaling,
    start_ms: u64,
}

impl Ramp {
    /// A ramp already settled at `players`.
    pub fn new(mode: Mode, players: u32) -> Self {
        let s = Scaling::for_party(players, mode);
        Ramp {
            mode,
            players,
            from: s,
            to: s,
            start_ms: 0,
        }
    }

    /// Party size now in force (target).
    pub fn players(&self) -> u32 {
        self.players
    }

    /// The party size changed; blends from whatever is in force right now.
    pub fn set_players(&mut self, players: u32, now_ms: u64) {
        if players == self.players {
            return;
        }
        self.from = self.current(now_ms);
        self.to = Scaling::for_party(players, self.mode);
        self.players = players;
        self.start_ms = now_ms;
    }

    /// Scaling at `now_ms`.
    pub fn current(&self, now_ms: u64) -> Scaling {
        let t = (now_ms.saturating_sub(self.start_ms) as f32 / RAMP_MS as f32).clamp(0.0, 1.0);
        let s = t * t * (3.0 - 2.0 * t);
        let (a, b) = (&self.from, &self.to);
        Scaling {
            focus: lerp(a.focus, b.focus, s),
            toughness: lerp(a.toughness, b.toughness, s),
            extras: lerp(a.extras, b.extras, s),
            max_extras_alive: if t <= 0.0 {
                a.max_extras_alive
            } else if t >= 1.0 {
                b.max_extras_alive
            } else {
                a.max_extras_alive.max(b.max_extras_alive)
            },
            damage_to_players: lerp(a.damage_to_players, b.damage_to_players, s),
            reward: lerp(a.reward, b.reward, s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn digest_numbers() {
        let s = Scaling::for_party(3, Mode::CoopBalanced);
        assert!(close(s.focus, 2.4));
        assert!(close(s.toughness, 2.0));
        assert!(close(s.extras, 0.4));
        assert_eq!(s.max_extras_alive, 4);
        assert!(close(s.reward, 1.0 / 1.4));
        assert!(close(s.damage_to_players, 1.2));
        let s4 = Scaling::for_party(4, Mode::CoopBalanced);
        assert!(close(s4.focus, 3.1) && close(s4.toughness, 2.5) && close(s4.extras, 0.6));
    }

    #[test]
    fn solo_and_normal_mode_are_neutral() {
        assert_eq!(Scaling::for_party(1, Mode::CoopBalanced), Scaling::NEUTRAL);
        assert_eq!(Scaling::for_party(4, Mode::Normal), Scaling::NEUTRAL);
        assert_eq!(Scaling::for_party(0, Mode::CoopBalanced), Scaling::NEUTRAL);
    }

    #[test]
    fn damage_and_xp_follow_the_factors() {
        let s = Scaling::for_party(2, Mode::CoopBalanced);
        assert!(close(s.damage_dealt(30.0), 20.0));
        assert_eq!(s.xp(120), 100);
    }

    #[test]
    fn ramp_blends_over_ten_seconds_and_hits_the_end_points() {
        let mut r = Ramp::new(Mode::CoopBalanced, 1);
        r.set_players(3, 5000);
        assert_eq!(r.current(5000), Scaling::for_party(1, Mode::CoopBalanced));
        let mid = r.current(5000 + RAMP_MS / 2);
        assert!(close(mid.focus, (1.0 + 2.4) / 2.0));
        assert_eq!(
            r.current(5000 + RAMP_MS),
            Scaling::for_party(3, Mode::CoopBalanced)
        );
        assert_eq!(
            r.current(1_000_000),
            Scaling::for_party(3, Mode::CoopBalanced)
        );
    }

    #[test]
    fn retarget_mid_ramp_starts_from_the_current_value() {
        let mut r = Ramp::new(Mode::CoopBalanced, 1);
        r.set_players(4, 0);
        let at = r.current(RAMP_MS / 2);
        r.set_players(2, RAMP_MS / 2);
        assert!(close(r.current(RAMP_MS / 2).focus, at.focus), "no jump");
        assert_eq!(r.players(), 2);
    }

    proptest! {
        #[test]
        fn ramp_is_monotonic_between_end_points(a in 1u32..5, b in 1u32..5, t1 in 0u64..12_000, t2 in 0u64..12_000) {
            let mut r = Ramp::new(Mode::CoopBalanced, a);
            r.set_players(b, 0);
            let (lo, hi) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
            let (x, y) = (r.current(lo).focus, r.current(hi).focus);
            if b >= a { prop_assert!(y + 1e-4 >= x) } else { prop_assert!(y <= x + 1e-4) }
        }

        #[test]
        fn loot_thinning_is_deterministic_and_proportional(n in 2u32..5) {
            let s = Scaling::for_party(n, Mode::CoopBalanced);
            let kept = (0..4000u32).filter(|i| s.keeps_loot(FormId(0x0100_0000 + i))).count() as f32 / 4000.0;
            prop_assert!((kept - s.reward).abs() < 0.05, "kept {kept} want {}", s.reward);
            prop_assert_eq!(s.keeps_loot(FormId(77)), s.keeps_loot(FormId(77)));
        }
    }

    #[test]
    fn extras_roll_matches_rate() {
        let s = Scaling::for_party(4, Mode::CoopBalanced);
        let n = (0..5000u32)
            .filter(|i| s.spawns_extra(FormId(0x0200_0000 + i)))
            .count() as f32
            / 5000.0;
        assert!((n - s.extras).abs() < 0.04, "{n}");
    }
}
