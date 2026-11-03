//! Validation of shots and blasts a peer reports: deduplication, rate limit
//! and range. The host relays accepted ones to the other machines of the cell
//! (visual copies: impact decals, sounds, flashes); damage is never carried
//! by a relay.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::{BlastEvent, FireEvent, MAX_REQ_PER_SEC};
use crate::avatar::dequant_pos;
use crate::ids::PlayerId;

/// Farthest a shot may travel from the shooter's avatar to its impact.
pub const MAX_SHOT_RANGE: f32 = 40_000.0;
/// Farthest a blast centre may be from the player who set it off.
pub const MAX_BLAST_REACH: f32 = 20_000.0;
const MEMORY: usize = 512;

/// Per-peer shot and blast checks.
#[derive(Clone, Debug, Default)]
pub struct FireHub {
    seen_fire: BTreeSet<(PlayerId, u32)>,
    seen_blast: BTreeSet<(PlayerId, u32)>,
    order: VecDeque<(bool, PlayerId, u32)>,
    fire_times: BTreeMap<PlayerId, VecDeque<u64>>,
    blast_times: BTreeMap<PlayerId, VecDeque<u64>>,
    /// Shots relayed.
    pub fires_ok: u32,
    /// Blasts accepted.
    pub blasts_ok: u32,
    /// Refused (duplicate, rate, range).
    pub refused: u32,
}

fn dist(a: [i32; 3], b: [f32; 3]) -> f32 {
    let p = [dequant_pos(a[0]), dequant_pos(a[1]), dequant_pos(a[2])];
    ((p[0] - b[0]).powi(2) + (p[1] - b[1]).powi(2) + (p[2] - b[2]).powi(2)).sqrt()
}

fn dist_fixed(a: [i32; 3], b: [i32; 3]) -> f32 {
    dist(a, [dequant_pos(b[0]), dequant_pos(b[1]), dequant_pos(b[2])])
}

impl FireHub {
    /// Empty hub.
    pub fn new() -> Self {
        Self::default()
    }

    fn remember(&mut self, blast: bool, p: PlayerId, id: u32) -> bool {
        let fresh = if blast {
            self.seen_blast.insert((p, id))
        } else {
            self.seen_fire.insert((p, id))
        };
        if !fresh {
            return false;
        }
        self.order.push_back((blast, p, id));
        while self.order.len() > MEMORY {
            if let Some((b, p, i)) = self.order.pop_front() {
                if b {
                    self.seen_blast.remove(&(p, i));
                } else {
                    self.seen_fire.remove(&(p, i));
                }
            }
        }
        true
    }

    fn rate(times: &mut BTreeMap<PlayerId, VecDeque<u64>>, p: PlayerId, now_ms: u64) -> bool {
        let t = times.entry(p).or_default();
        while t.front().is_some_and(|x| now_ms.saturating_sub(*x) >= 1000) {
            t.pop_front();
        }
        if t.len() >= MAX_REQ_PER_SEC {
            return false;
        }
        t.push_back(now_ms);
        true
    }

    /// A shot from `from`. `at` is where the host believes `from` stands
    /// (their avatar). Returns the event to relay, with the shooter filled.
    pub fn on_fire(
        &mut self,
        from: PlayerId,
        now_ms: u64,
        at: Option<[f32; 3]>,
        mut e: FireEvent,
    ) -> Option<FireEvent> {
        let near = at.map_or(true, |p| {
            dist(e.origin, p) <= MAX_BLAST_REACH && dist_fixed(e.origin, e.hit) <= MAX_SHOT_RANGE
        });
        if e.id == 0
            || !near
            || !Self::rate(&mut self.fire_times, from, now_ms)
            || !self.remember(false, from, e.id)
        {
            self.refused += 1;
            return None;
        }
        e.shooter = from;
        self.fires_ok += 1;
        Some(e)
    }

    /// A blast from `from`.
    pub fn on_blast(
        &mut self,
        from: PlayerId,
        now_ms: u64,
        at: Option<[f32; 3]>,
        mut e: BlastEvent,
    ) -> Option<BlastEvent> {
        let near = at.map_or(true, |p| dist(e.pos, p) <= MAX_BLAST_REACH);
        if e.id == 0
            || !near
            || !Self::rate(&mut self.blast_times, from, now_ms)
            || !self.remember(true, from, e.id)
        {
            self.refused += 1;
            return None;
        }
        e.source = from;
        self.blasts_ok += 1;
        Some(e)
    }
}
