//! Every animation the other players play, on their avatars (docs/coop/SPECS.md S41), part of
//! [`crate::look::Look`].
//!
//! The owner sends what its player's body does:
//!
//! - in each state (20 a second): looking down the sights (`FLAG_SIGHTS`), knocked down
//!   (`FLAG_KNOCKED`), and the furniture it uses (`AvatarState::seat`); its turning on the spot
//!   is already in the movement flags or follows from its heading;
//! - as action events, in order, once each: each group its game starts in a section other than
//!   movement that the avatar would not start by itself (`ActionKind::Group`, a hit reaction, a
//!   stagger, a block, a death, a get-up, an equip of another kind), each idle (`Idle`, a
//!   gesture, a fidget, a furniture idle), each jump (`Jump`), and its walking and running
//!   speeds when they change (`Speed`).
//!
//! The other games play them on its avatar with the game's own calls ([`coop_engine::anim`]):
//! the group as `PlayGroup` plays one, the idle as `PlayIdle`, the jump as the mobile object's
//! own (its controller then plays the jump, the fall and the landing as it leaves the ground and
//! comes back with its player's place), the sights through `SetIronSights`, the speeds stored in
//! its process (so its legs move at its player's speed), knocked down and up again through its
//! fatigue (also while its player is downed or dead), sat on and got up from the furniture with
//! the player's own sit and get-up packages, and its heading's turns on the spot as the turn
//! flags of its forced movement.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::avatar::{
    ActionKind, AvatarState, Sample, FLAG_DEAD, FLAG_DOWNED, FLAG_KNOCKED, FLAG_SIGHTS,
};
use coop_core::ids::PlayerId;
use coop_engine::anim::{self, Body};
use coop_engine::{actors, addr, look, puppet};
use engine_core::Engine;

/// An avatar whose sights are not as asked is asked again this often (ms).
pub const SIGHTS_EVERY_MS: u64 = 1000;
/// A turn on the spot read from the heading is held this long after the heading last changed
/// (ms; co-op's own number, so a slow turn is one turn).
pub const TURN_HOLD_MS: u64 = 250;
/// The walking and running speeds are sent again when either changed by this share.
pub const SPEED_CHANGE: f32 = 0.02;

/// An `ActionKind::Group` event's word: the section (bits 16..24) and the group's type.
pub fn group_arg(section: u32, kind: u8) -> u32 {
    ((section & 0xff) << 16) | u32::from(kind)
}

/// An `ActionKind::Speed` event's word: the walking speed (bits 0..16) and the running speed, in
/// whole units a second.
pub fn speed_arg(walk: f32, run: f32) -> u32 {
    let q = |v: f32| v.round().clamp(0.0, f32::from(u16::MAX)) as u32;
    q(walk) | (q(run) << 16)
}

/// The walking and running speeds of an `ActionKind::Speed` word.
pub fn speeds_of(arg: u32) -> (f32, f32) {
    (f32::from(arg as u16), f32::from((arg >> 16) as u16))
}

#[derive(Debug, Default)]
struct Peer {
    pending: Vec<(ActionKind, u32)>,
    speeds: Option<(f32, f32)>,
}

#[derive(Debug, Default)]
struct Shown {
    heading: Option<f32>,
    turn: Option<(u16, u64)>,
    sights: Option<(bool, u64)>,
    down: Option<bool>,
    seat: Option<u32>,
    speeds: Option<(f32, f32)>,
}

/// Counters, for the status line and the tests.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Groups, idles and jumps played on avatars.
    pub groups: u64,
    /// See [`Counts::groups`].
    pub idles: u64,
    /// See [`Counts::groups`].
    pub jumps: u64,
    /// Sights set, speeds given, knock-downs and get-ups from them, furniture sat on and left,
    /// turns on the spot read from a heading.
    pub sights: u64,
    /// See [`Counts::sights`].
    pub speeds: u64,
    /// See [`Counts::sights`].
    pub knocks: u64,
    /// See [`Counts::sights`].
    pub sits: u64,
    /// See [`Counts::sights`].
    pub get_ups: u64,
    /// See [`Counts::sights`].
    pub turns: u64,
    /// This player's events sent.
    pub sent: u64,
}

/// This player's body for the others, and the others' on their avatars here.
#[derive(Debug, Default)]
pub struct Anim {
    last: Option<Body>,
    speed_sent: Option<(f32, f32)>,
    peers: BTreeMap<PlayerId, Peer>,
    shown: BTreeMap<u32, Shown>,
    /// See [`Counts`].
    pub counts: Counts,
}

impl Anim {
    /// Puts this player's sights, knock-down and furniture into its state.
    pub fn stamp(&self, e: &mut Engine, state: &mut AvatarState) {
        let Some(b) = anim::player_body(e) else {
            return;
        };
        if b.sights {
            state.flags |= FLAG_SIGHTS;
        }
        if b.knock != 0 {
            state.flags |= FLAG_KNOCKED;
        }
        if b.sit != 0 && actors::shareable(b.furniture) {
            state.seat = b.furniture;
        }
    }

    /// The events this player's body gives since the last frame (the first frame only reads it,
    /// and sends the speeds).
    pub fn own(&mut self, e: &mut Engine) -> Vec<(ActionKind, u32)> {
        let mut out = Vec::new();
        let Some(b) = anim::player_body(e) else {
            return out;
        };
        let reloading = look::pose(e).is_some_and(|p| p.reloading);
        if let Some(last) = self.last {
            for (s, (&now, &was)) in b.groups.iter().zip(last.groups.iter()).enumerate() {
                let s = s as u32;
                if now != was && !reloading && anim::replayable(e, s, now) {
                    out.push((ActionKind::Group, group_arg(s, anim::group_type(now))));
                }
            }
            if b.idle != last.idle && b.idle != 0 && actors::shareable(b.idle) {
                out.push((ActionKind::Idle, b.idle));
            }
            if b.controller == addr::CONTROLLER_JUMPING && last.controller != b.controller {
                out.push((ActionKind::Jump, 0));
            }
        }
        let changed = |a: f32, b: f32| (a - b).abs() > SPEED_CHANGE * a.abs().max(1.0);
        let speeds_moved = match self.speed_sent {
            None => true,
            Some((w, r)) => changed(w, b.walk) || changed(r, b.run),
        };
        if speeds_moved && b.walk > 0.0 && b.run > 0.0 {
            self.speed_sent = Some((b.walk, b.run));
            out.push((ActionKind::Speed, speed_arg(b.walk, b.run)));
        }
        self.last = Some(b);
        self.counts.sent += out.len() as u64;
        out
    }

    /// Drops the events not shown by the last frame (they are cues of that moment).
    pub fn clear_pending(&mut self) {
        for peer in self.peers.values_mut() {
            peer.pending.clear();
        }
    }

    /// Takes one of player `p`'s action events; false when it is not one of this module's.
    pub fn take(&mut self, p: PlayerId, kind: ActionKind, arg: u32) -> bool {
        let peer = self.peers.entry(p).or_default();
        match kind {
            ActionKind::Group | ActionKind::Idle | ActionKind::Jump => {
                peer.pending.push((kind, arg))
            }
            ActionKind::Speed => peer.speeds = Some(speeds_of(arg)),
            _ => return false,
        }
        true
    }

    /// The movement flags to force on avatar `avatar` shown at `s`: its player's, with a turn on
    /// the spot when its heading turns while it stands; `None` while it uses furniture (its
    /// package moves it).
    pub fn moves(&mut self, avatar: u32, s: &Sample, now_ms: u64) -> Option<u16> {
        let shown = self.shown.entry(avatar).or_default();
        let before = shown.heading.replace(s.heading);
        if s.seat != 0 {
            shown.turn = None;
            return None;
        }
        let standing =
            s.anim & (addr::MOVE_DIRECTIONS | addr::MOVE_TURN_LEFT | addr::MOVE_TURN_RIGHT) == 0;
        let mut moves = s.anim;
        if let Some(before) = before {
            let turned = anim::turn(s.anim, before, s.heading);
            if turned != s.anim {
                let bits = turned & !s.anim;
                if shown.turn.map(|t| t.0) != Some(bits) {
                    self.counts.turns += 1;
                }
                shown.turn = Some((bits, now_ms));
            }
        }
        match shown.turn {
            Some((bits, at)) if standing && now_ms.saturating_sub(at) < TURN_HOLD_MS => {
                moves |= bits
            }
            Some(_) if standing => shown.turn = None,
            _ => {}
        }
        Some(moves)
    }

    /// One frame of player `p`'s avatar `avatar`, shown at `s`: its events, speeds, sights,
    /// knock-down and furniture.
    pub fn frame(&mut self, e: &mut Engine, p: PlayerId, avatar: u32, s: &Sample, now_ms: u64) {
        let peer = self.peers.entry(p).or_default();
        let pending = std::mem::take(&mut peer.pending);
        let speeds = peer.speeds;
        for (kind, arg) in pending {
            let (played, count) = match kind {
                ActionKind::Group => (
                    anim::play_group(e, avatar, arg as u8),
                    &mut self.counts.groups,
                ),
                ActionKind::Idle => (anim::play_idle(e, avatar, arg), &mut self.counts.idles),
                ActionKind::Jump => (anim::jump(e, avatar), &mut self.counts.jumps),
                _ => continue,
            };
            *count += u64::from(played);
        }
        let shown = self.shown.entry(avatar).or_default();
        if let Some((walk, run)) = speeds {
            // Every frame: the process keeps the speeds only until it works them out again.
            if anim::set_speeds(e, avatar, walk, run) && shown.speeds != Some((walk, run)) {
                shown.speeds = Some((walk, run));
                self.counts.speeds += 1;
            }
        }
        let want = s.flags & FLAG_SIGHTS != 0;
        let has = anim::body_of_id(e, avatar).is_some_and(|b| b.sights);
        if want != has {
            let ask = match shown.sights {
                Some((asked, at)) => asked != want || now_ms.saturating_sub(at) >= SIGHTS_EVERY_MS,
                None => true,
            };
            if ask && anim::set_sights(e, avatar, want) {
                shown.sights = Some((want, now_ms));
                self.counts.sights += 1;
            }
        } else {
            shown.sights = None;
        }
        let down = s.flags & (FLAG_KNOCKED | FLAG_DOWNED | FLAG_DEAD) != 0;
        match shown.down {
            Some(was) if was == down => {}
            // A first frame standing asks nothing.
            None if !down => shown.down = Some(false),
            _ => {
                if anim::knock(e, avatar, down) {
                    shown.down = Some(down);
                    self.counts.knocks += 1;
                }
            }
        }
        let seat = s.seat;
        let had = shown.seat;
        match had {
            Some(was) if was == seat => {}
            None if seat == 0 => shown.seat = Some(0),
            was => {
                if seat != 0 {
                    puppet::free_moves(e, avatar);
                    if anim::sit(e, avatar, seat) {
                        self.counts.sits += 1;
                    }
                } else if was.is_some_and(|w| w != 0) && anim::get_up(e, avatar) {
                    self.counts.get_ups += 1;
                }
                self.shown.entry(avatar).or_default().seat = Some(seat);
            }
        }
    }

    /// What was kept for an avatar not in `seen` is let go.
    pub fn end_frame(&mut self, seen: &BTreeSet<u32>) {
        self.shown.retain(|id, _| seen.contains(id));
    }

    /// A player who left: what was known of them is forgotten.
    pub fn forget(&mut self, p: PlayerId) {
        self.peers.remove(&p);
    }

    /// One status phrase for the log.
    pub fn status(&self) -> String {
        let c = &self.counts;
        format!(
            "anim: groups {}, idles {}, jumps {}, sights {}, speeds {}, knocks {}, sits {}, get-ups {}, turns {}, sent {}",
            c.groups, c.idles, c.jumps, c.sights, c.speeds, c.knocks, c.sits, c.get_ups, c.turns, c.sent
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_event_words_go_and_come_back() {
        assert_eq!(group_arg(2, 0x45), 0x0002_0045);
        assert_eq!(speeds_of(speed_arg(95.4, 410.6)), (95.0, 411.0));
        assert_eq!(speeds_of(speed_arg(-3.0, 1e9)), (0.0, f32::from(u16::MAX)));
    }

    fn sample(anim: u16, heading: f32, seat: u32) -> Sample {
        Sample {
            cell: coop_core::ids::CellId(1),
            pos: [0.0; 3],
            heading,
            vel: [0.0; 3],
            anim,
            weapon: 0,
            health: 0,
            flags: 0,
            seat,
            extrapolated: false,
            teleported: false,
        }
    }

    #[test]
    fn a_turning_heading_while_standing_is_a_turn_held_a_moment() {
        let mut a = Anim::default();
        assert_eq!(a.moves(9, &sample(0, 1.0, 0), 0), Some(0));
        assert_eq!(
            a.moves(9, &sample(0, 1.2, 0), 50),
            Some(addr::MOVE_TURN_LEFT)
        );
        // Still for a moment: still turning; then standing.
        assert_eq!(
            a.moves(9, &sample(0, 1.2, 0), 100),
            Some(addr::MOVE_TURN_LEFT)
        );
        assert_eq!(a.moves(9, &sample(0, 1.2, 0), 50 + TURN_HOLD_MS), Some(0));
        assert_eq!(
            a.moves(9, &sample(0, 1.0, 0), 400),
            Some(addr::MOVE_TURN_RIGHT)
        );
        assert_eq!(a.counts.turns, 2);
        // Walking turns are the walk's; furniture: nothing forced.
        assert_eq!(a.moves(9, &sample(0x201, 2.0, 0), 450), Some(0x201));
        assert_eq!(a.moves(9, &sample(0, 2.5, 0x0010_7200), 500), None);
    }
}
