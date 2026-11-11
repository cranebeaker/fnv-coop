//! Held actors move between two states instead of jumping (docs/coop/SPECS.md S21).
//!
//! An actor's owner sends its place about 20 times a second, and only when it changed. Set
//! straight into the game that is a jump per state. This module gives each held actor the
//! buffer coop-core uses for the other players' avatars (`avatar::InterpBuffer`): the newest
//! known state is fed to it at the avatars' own rate, and the place the actor is put at is the
//! buffer's, a tenth of a second behind the newest state and blended between the two states
//! around that moment. A move to another space or a long jump is taken at once, as the buffer
//! does for an avatar. Health, life and the combat flags are not smoothed.

use std::collections::BTreeMap;

use coop_core::actors::ActorState;
use coop_core::avatar::{
    dequant_heading, dequant_pos, AvatarState, InterpBuffer, SEND_INTERVAL_MS,
};
use coop_core::ids::Seq;

#[derive(Debug, Default)]
struct Track {
    buffer: InterpBuffer,
    /// The number of the last sample fed (the time in send intervals).
    fed: u32,
}

/// The buffers of the actors a machine holds.
#[derive(Debug, Default)]
pub struct Smooth {
    tracks: BTreeMap<u32, Track>,
}

impl Smooth {
    /// Where to put actor `id`, whose newest known state is `state`, at `now_ms`: its position
    /// and heading (radians).
    pub fn place(&mut self, id: u32, state: &ActorState, now_ms: u64) -> ([f32; 3], f32) {
        let t = self.tracks.entry(id).or_default();
        // One sample per send interval, numbered by the time, so the buffer sees the steady
        // stream it was written for whether the state changed or not.
        let n = ((now_ms / SEND_INTERVAL_MS) as u32).max(1);
        if n > t.fed {
            t.fed = n;
            t.buffer.push(
                now_ms,
                Seq(n),
                AvatarState {
                    cell: state.cell,
                    pos: state.pos,
                    heading: state.heading,
                    ..AvatarState::default()
                },
            );
        }
        match t.buffer.sample(now_ms) {
            Some(s) => (s.pos, s.heading),
            None => (state.pos.map(dequant_pos), dequant_heading(state.heading)),
        }
    }

    /// Forgets every actor not in `keep` (it is not held any more).
    pub fn keep(&mut self, keep: impl Fn(u32) -> bool) {
        self.tracks.retain(|id, _| keep(*id));
    }

    /// How many actors have a buffer.
    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    /// Whether no actor has a buffer.
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use coop_core::avatar::quant_pos;
    use coop_core::ids::CellId;

    fn at(cell: u32, x: f32) -> ActorState {
        ActorState {
            cell: CellId(cell),
            pos: [quant_pos(x), 0, 0],
            ..ActorState::default()
        }
    }

    #[test]
    fn an_actor_goes_from_one_state_to_the_next_in_steps() {
        let mut s = Smooth::default();
        // It stood at 0 for a second; then the owner says 100.
        for t in (0..1000).step_by(16) {
            assert_eq!(s.place(7, &at(1, 0.0), t).0[0], 0.0);
        }
        let mut seen = Vec::new();
        for t in (1000..1300).step_by(16) {
            seen.push(s.place(7, &at(1, 100.0), t).0[0]);
        }
        // Not there at once, never backwards, there in the end.
        assert!(seen[0] < 50.0, "{seen:?}");
        assert!(seen.windows(2).all(|w| w[0] <= w[1]), "{seen:?}");
        assert!(seen.iter().any(|x| *x > 0.0 && *x < 100.0), "{seen:?}");
        assert_eq!(*seen.last().unwrap(), 100.0);
    }

    #[test]
    fn another_space_or_a_long_jump_is_taken_at_once() {
        let mut s = Smooth::default();
        for t in (0..500).step_by(50) {
            s.place(7, &at(1, 0.0), t);
        }
        assert_eq!(s.place(7, &at(2, 9000.0), 500).0[0], 9000.0);
        assert_eq!(s.place(7, &at(2, 20000.0), 550).0[0], 20000.0);
        // A forgotten actor starts anew.
        s.keep(|_| false);
        assert!(s.is_empty());
        assert_eq!(s.place(7, &at(1, 5.0), 600).0[0], 5.0);
        assert_eq!(s.len(), 1);
    }
}
