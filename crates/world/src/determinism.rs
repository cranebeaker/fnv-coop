//! Determinism support for host/guest state comparison.
//!
//! The simulation state ([`GameState`]) must evolve identically on any
//! machine that is given the same seed and the same inputs. This module
//! holds the pieces for that: a settable [`SimSeed`], small seeded random
//! streams ([`SimRng`]) for code that wants its own stream instead of the
//! shared game dice, and [`state_hash`], a stable 64-bit digest of the
//! shared state. See `crates/world/DETERMINISM.md` for the audit and the
//! rules for contributors.

use crate::save;
use crate::scripting::GameState;

/// The seed of a run's dice. [`SimSeed::DEFAULT`] is what a new
/// single-player game has always used, so default behaviour is unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimSeed(pub u64);

impl SimSeed {
    /// The dice a new game starts with.
    pub const DEFAULT: SimSeed = SimSeed(0x9E37_79B9_7F4A_7C15);

    /// The shared dice state for this seed (xorshift must never hold 0).
    pub fn dice(self) -> u64 {
        self.0.max(1)
    }

    /// An independent stream for `label` (an actor's form ID, a system's
    /// constant): the same seed and label always give the same numbers,
    /// and different labels give unrelated ones. Drawing from it does not
    /// touch the shared dice.
    pub fn stream(self, label: u64) -> SimRng {
        SimRng::new(self.0, label)
    }
}

impl Default for SimSeed {
    fn default() -> Self {
        SimSeed::DEFAULT
    }
}

/// splitmix64 finaliser, used to spread seeds and labels.
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A small seeded xorshift64 stream: the same generator as
/// [`GameState::roll`], for code that needs numbers of its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimRng(u64);

impl SimRng {
    pub fn new(seed: u64, label: u64) -> SimRng {
        SimRng(mix(seed ^ mix(label)).max(1))
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// A number in `0..n` (0 when `n` is 0).
    pub fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            self.next_u64() % n
        }
    }

    /// A number in `0..1`.
    pub fn unit(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }
}

impl GameState {
    /// Restarts the game's dice from `seed`. Everything that rolls
    /// (`GameState::roll`, leveled lists, hit chances, scripts'
    /// `GetRandomPercent`) then follows the seed.
    pub fn set_sim_seed(&mut self, seed: SimSeed) {
        self.dice = seed.dice();
    }

    /// An independent stream derived from the dice as they stand now,
    /// without advancing them.
    pub fn sim_stream(&self, label: u64) -> SimRng {
        SimRng::new(self.dice, label)
    }
}

fn fnv(hash: &mut u64, bytes: &[u8]) {
    for &b in bytes {
        *hash ^= u64::from(b);
        *hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
}

/// A stable digest of the shared simulation state. It covers the canonical
/// save text (every field there is written in sorted order, the dice
/// included) plus the transient fields the save leaves out and that
/// change what happens next: the player's cell, queued events, noise and
/// who has hit whom. Two states with equal hashes are the same state for
/// co-op purposes; equal runs must give equal hashes on every machine.
pub fn state_hash(state: &GameState) -> u64 {
    let mut hash = 0xCBF2_9CE4_8422_2325u64;
    fnv(&mut hash, save::save(state, None).as_bytes());
    fnv(
        &mut hash,
        format!("cell:{:?}\n", state.player_cell).as_bytes(),
    );
    for e in &state.events {
        fnv(&mut hash, format!("event:{e:?}\n").as_bytes());
    }
    let mut noise: Vec<_> = state.noise.iter().collect();
    noise.sort_by_key(|(k, _)| **k);
    for (who, n) in noise {
        fnv(&mut hash, format!("noise:{who:?}:{n:?}\n").as_bytes());
    }
    let mut hits: Vec<_> = state.hit_targets.iter().collect();
    hits.sort_by_key(|(k, _)| **k);
    for (who, list) in hits {
        fnv(&mut hash, format!("hit:{who:?}:{list:?}\n").as_bytes());
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scripting::Event;
    use esm::{FormId, LoadOrder};

    /// An empty game: only the exe's default settings.
    fn empty_order(tag: &str) -> (testdata::TempData, LoadOrder) {
        use testdata::{record, sub};
        let data = testdata::TempData::empty(tag);
        let mut hedr = 1.34f32.to_le_bytes().to_vec();
        hedr.extend([0; 8]);
        data.write("FalloutNV.esm", &record(b"TES4", 0, &sub(b"HEDR", &hedr)));
        let order =
            LoadOrder::from_data_dir(data.path(), &esm::ActivePlugins::OfficialOnly).unwrap();
        (data, order)
    }

    /// A short headless scenario: many actors knocked down at once (the
    /// map the get-up order used to depend on), rolls from the shared dice
    /// between ticks, a roll-driven item grab and some noise, then time
    /// passing in frame-sized steps. Returns the hash after every tick.
    fn scenario(order: &LoadOrder, seed: SimSeed, steps: &[f32]) -> Vec<u64> {
        let mut state = GameState::default();
        state.set_sim_seed(seed);
        for i in 0..48u32 {
            // Several share a timer, so they all get up on the same tick.
            state
                .more
                .down
                .insert(FormId(0xA000 + i), 1.0 + (i % 4) as f32);
            state.noise.insert(
                FormId(0xB000 + i),
                crate::noise::Noise {
                    value: i as i32,
                    timer: 2.0,
                },
            );
        }
        let mut hashes = vec![state_hash(&state)];
        for &dt in steps {
            let r = state.roll();
            *state
                .items
                .entry((FormId(0x14), FormId(0xC000 + (r % 7) as u32)))
                .or_insert(0) += 1;
            crate::combat::advance_down(order, &mut state, dt);
            crate::noise::update(&mut state, dt);
            hashes.push(state_hash(&state));
        }
        hashes
    }

    const STEPS: [f32; 6] = [0.5, 0.5, 1.0, 1.0, 2.0, 1.0];

    #[test]
    fn same_seed_and_inputs_give_the_same_hashes() {
        let (_data, order) = empty_order("determinism-same");
        let a = scenario(&order, SimSeed(1234), &STEPS);
        for _ in 0..5 {
            // Each run builds fresh hash maps with their own random
            // iteration order; the hashes must not care.
            assert_eq!(a, scenario(&order, SimSeed(1234), &STEPS));
        }
        // The scenario really did things: events were queued and the
        // hash moved on every tick.
        let mut state = GameState::default();
        state.more.down.insert(FormId(0xA000), 0.5);
        crate::combat::advance_down(&order, &mut state, 1.0);
        assert!(state
            .events
            .iter()
            .any(|e| matches!(e, Event::GotUp { .. })));
        assert!(a.windows(2).all(|w| w[0] != w[1]));
    }

    #[test]
    fn get_up_events_come_in_a_fixed_order() {
        let (_data, order) = empty_order("determinism-order");
        let mut state = GameState::default();
        for i in 0..64u32 {
            state.more.down.insert(FormId(0xA000 + i), 0.5);
        }
        crate::combat::advance_down(&order, &mut state, 1.0);
        let got_up: Vec<FormId> = state
            .events
            .iter()
            .filter_map(|e| match e {
                Event::GotUp { who } => Some(*who),
                _ => None,
            })
            .collect();
        assert_eq!(got_up.len(), 64);
        assert!(got_up.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn a_different_seed_changes_the_hash() {
        let (_data, order) = empty_order("determinism-seed");
        let a = scenario(&order, SimSeed(1234), &STEPS);
        let b = scenario(&order, SimSeed(1235), &STEPS);
        assert_ne!(a[0], b[0]);
        assert_ne!(a.last(), b.last());
    }

    #[test]
    fn a_different_input_changes_the_hash() {
        let (_data, order) = empty_order("determinism-input");
        let a = scenario(&order, SimSeed(1234), &STEPS);
        let mut other = STEPS;
        other[2] = 0.25;
        let b = scenario(&order, SimSeed(1234), &other);
        assert_ne!(a.last(), b.last());
    }

    #[test]
    fn the_default_seed_is_the_new_game_dice() {
        let mut state = GameState::default();
        state.set_sim_seed(SimSeed::DEFAULT);
        assert_eq!(state.dice, 0x9E37_79B9_7F4A_7C15);
        state.set_sim_seed(SimSeed(0));
        assert_eq!(state.dice, 1);
    }

    #[test]
    fn streams_repeat_and_differ_by_label_without_touching_the_dice() {
        let mut state = GameState::default();
        state.set_sim_seed(SimSeed(99));
        let before = state.dice;
        let mut a = state.sim_stream(7);
        let mut b = state.sim_stream(7);
        let mut c = state.sim_stream(8);
        assert_eq!(state.dice, before);
        let xs: Vec<u64> = (0..8).map(|_| a.next_u64()).collect();
        assert_eq!(xs, (0..8).map(|_| b.next_u64()).collect::<Vec<_>>());
        assert_ne!(xs, (0..8).map(|_| c.next_u64()).collect::<Vec<_>>());
        assert!(a.unit() < 1.0 && a.below(10) < 10);
    }
}
