//! The live ladder: players in the world, one logged rung at a time, so that a run on a PC names
//! the step that failed.
//!
//! 1. This game's player is read (space, place, 3D).
//! 2. Another player's state arrives.
//! 3. A guest's player, once it is in the world itself, is moved to the host's space by the
//!    game's own position request (the way a door moves the player), again each time the host
//!    is in another space.
//! 4. A remote player in a space present here gets an avatar (the adapter's `Body`).
//! 5. The avatar is driven every frame.
//!
//! Each rung writes `COOP-RUNG n entering ...` once, before its first call into the game (a
//! crash still names the rung), and `COOP-RUNG n PASS|FAIL ...` once. [`Cap`] stops the ladder
//! at a rung without a rebuild.

use std::collections::BTreeMap;

use coop_core::avatar::{quant_heading, quant_pos, AvatarState, Sample};
use coop_core::ids::{CellId, PlayerId};
use coop_engine::avatar::{Avatar, Body, Drive, Left, Look, Remote, Step};
use coop_engine::travel::{self, Local, Refusal};
use coop_engine::{addr, Program};

/// How far up the ladder a run goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cap {
    /// Rungs 1 and 2: states are read, sent and logged; the game is not touched.
    Log,
    /// Up to rung 3: a guest is moved to the host's space too.
    Warp,
    /// All five rungs.
    Avatar,
}

impl Cap {
    /// `log`, `warp` or `avatar`; anything else, or nothing, is the whole ladder.
    pub fn parse(v: &str) -> Cap {
        match v.trim() {
            "log" => Cap::Log,
            "warp" => Cap::Warp,
            _ => Cap::Avatar,
        }
    }
}

/// Frames that a move may take before the rung reports it failed.
pub const WARP_FRAMES: u32 = 600;
/// At least this long between two move requests of a guest.
pub const WARP_EVERY_MS: u64 = 5000;
/// How long a guest's player may be out of the world, with the host in it, before the log says
/// so.
const HINT_AFTER_MS: u64 = 15_000;
/// Frames that an avatar is driven before rung 5 reports.
pub const DRIVES_TO_PASS: u64 = 100;
/// A remote player faster than this (units a second) runs; slower than [`STAND_SPEED`] stands.
const RUN_SPEED: f32 = 250.0;
const STAND_SPEED: f32 = 10.0;

/// This game's player as the state sent to the others: the space as its cell.
pub fn state_of(l: &Local) -> AvatarState {
    AvatarState {
        cell: CellId(l.space),
        pos: l.place.map(quant_pos),
        heading: quant_heading(l.heading),
        ..AvatarState::default()
    }
}

/// The frame's command of an avatar for a remote player's sample: its place, its heading in
/// degrees, and the movement flags from its speed (the game's mover then picks the animation).
pub fn remote_of(s: &Sample) -> Remote {
    let speed = (s.vel[0] * s.vel[0] + s.vel[1] * s.vel[1]).sqrt();
    let flags = if speed < STAND_SPEED {
        0
    } else if speed >= RUN_SPEED {
        addr::move_flag::FORWARD | addr::move_flag::RUN
    } else {
        addr::move_flag::FORWARD | addr::move_flag::WALK
    };
    Remote {
        cell: s.cell.0,
        look: Look::default(),
        drive: Drive {
            place: s.pos,
            heading: s.heading.to_degrees(),
            flags,
            with_z: false,
        },
    }
}

#[derive(Debug, Default)]
struct Peer {
    body: Body,
    /// Placing failed: not tried again in this run.
    failed: bool,
    driven: u64,
    placed: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Warp {
    Idle,
    Waiting { space: u32, frames: u32 },
}

/// The ladder's state on one machine.
pub struct Ladder {
    guest: bool,
    cap: Cap,
    emit: fn(&str),
    /// Every line written, in order.
    pub lines: Vec<String>,
    started: bool,
    entered: [bool; 6],
    reported: [bool; 6],
    warp: Warp,
    last_warp_ms: Option<u64>,
    /// Move requests that the game took.
    pub warps: u32,
    waiting_since: Option<u64>,
    hinted: bool,
    peers: BTreeMap<PlayerId, Peer>,
}

fn print_now(line: &str) {
    use std::io::Write as _;
    println!("{line}");
    let _ = std::io::stdout().flush();
}

impl Ladder {
    /// A ladder for a guest or a host, up to `cap`. Lines are printed at once and kept in
    /// [`Ladder::lines`].
    pub fn new(guest: bool, cap: Cap) -> Ladder {
        Ladder {
            guest,
            cap,
            emit: print_now,
            lines: Vec::new(),
            started: false,
            entered: [false; 6],
            reported: [false; 6],
            warp: Warp::Idle,
            last_warp_ms: None,
            warps: 0,
            waiting_since: None,
            hinted: false,
            peers: BTreeMap::new(),
        }
    }

    /// The same ladder writing nowhere but [`Ladder::lines`] (tests).
    pub fn quiet(mut self) -> Ladder {
        self.emit = |_| {};
        self
    }

    fn say(&mut self, line: String) {
        (self.emit)(&line);
        self.lines.push(line);
    }

    /// Writes rung `n`'s entering line, once.
    fn entering(&mut self, n: usize, what: String) {
        if !self.reported[n] && !self.entered[n] {
            self.entered[n] = true;
            self.say(format!("COOP-RUNG {n} entering: {what}"));
        }
    }

    /// Writes rung `n`'s result, once.
    fn result(&mut self, n: usize, pass: bool, what: String) {
        if !self.reported[n] {
            self.reported[n] = true;
            let verdict = if pass { "PASS" } else { "FAIL" };
            self.say(format!("COOP-RUNG {n} {verdict} {what}"));
        }
    }

    /// True when rung `n` reported.
    pub fn reported(&self, n: usize) -> bool {
        self.reported.get(n).copied().unwrap_or(false)
    }

    /// One frame, between two of the game's frames. `peers`: the other players' interpolated
    /// states (those in the world). Returns this player's place, for sending.
    pub fn frame(
        &mut self,
        p: &mut dyn Program,
        now_ms: u64,
        peers: &[(PlayerId, Sample)],
    ) -> Option<Local> {
        if !self.started {
            self.started = true;
            self.say("COOP-RUNG 1 entering: reading this game's player".into());
        }
        let mut me = travel::local(p);
        if let Some(l) = me {
            self.result(
                1,
                true,
                format!(
                    "me in space {:08x} at ({:.0}, {:.0}, {:.0}), 3D {}",
                    l.space,
                    l.place[0],
                    l.place[1],
                    l.place[2],
                    if l.loaded { "yes" } else { "no" }
                ),
            );
        }
        if let Some((who, s)) = peers.first() {
            self.result(
                2,
                true,
                format!(
                    "player {} in space {:08x} at ({:.0}, {:.0}, {:.0})",
                    who.0, s.cell.0, s.pos[0], s.pos[1], s.pos[2]
                ),
            );
        }
        if self.guest && self.cap >= Cap::Warp {
            if let Some((_, host)) = peers.iter().find(|(who, _)| who.is_host()) {
                let host = *host;
                self.warp_step(p, now_ms, &host, me);
                me = travel::local(p);
            }
        }
        // No avatar is placed or driven while this player's own move is under way.
        if self.cap >= Cap::Avatar && self.warp == Warp::Idle {
            self.avatars(p, peers);
        }
        me
    }

    fn warp_step(&mut self, p: &mut dyn Program, now_ms: u64, host: &Sample, me: Option<Local>) {
        let host_space = host.cell.0;
        // Only a player that is in the world itself is moved: at the main menu, or before a new
        // game put the player somewhere, the game is not in a state to move it.
        let Some(me) = me.filter(|l| l.loaded) else {
            let since = *self.waiting_since.get_or_insert(now_ms);
            if !self.hinted && now_ms.saturating_sub(since) >= HINT_AFTER_MS {
                self.hinted = true;
                self.say(
                    "co-op live: the host is in the world, but this game's player is not yet (at the menu: press New Game)"
                        .into(),
                );
            }
            return;
        };
        self.waiting_since = None;
        match self.warp {
            Warp::Waiting { space, frames } => {
                if me.space == space {
                    self.warp = Warp::Idle;
                    self.result(3, true, format!("moved to the host in {space:08x}"));
                } else if frames >= WARP_FRAMES {
                    self.warp = Warp::Idle;
                    self.result(
                        3,
                        false,
                        format!(
                            "still in {:08x} {WARP_FRAMES} frames after asking to move to {space:08x}",
                            me.space
                        ),
                    );
                } else {
                    self.warp = Warp::Waiting {
                        space,
                        frames: frames + 1,
                    };
                }
            }
            Warp::Idle if me.space == host_space => {
                self.result(
                    3,
                    true,
                    format!("already with the host in {host_space:08x}"),
                );
            }
            Warp::Idle => {
                if self
                    .last_warp_ms
                    .is_some_and(|t| now_ms.saturating_sub(t) < WARP_EVERY_MS)
                {
                    return;
                }
                self.last_warp_ms = Some(now_ms);
                self.entering(
                    3,
                    format!("asking the game to move this player to the host in {host_space:08x}"),
                );
                match travel::request_move(p, host_space, host.pos, host.heading) {
                    Ok(()) => {
                        self.warps += 1;
                        self.warp = Warp::Waiting {
                            space: host_space,
                            frames: 0,
                        };
                    }
                    // Not a failure of the rung: the game says not now, and is asked later.
                    Err(Refusal::Busy) => {}
                    Err(why) => self.result(3, false, why.to_string()),
                }
            }
        }
    }

    fn avatars(&mut self, p: &mut dyn Program, peers: &[(PlayerId, Sample)]) {
        // A player who left, or who is not in the world: its avatar goes.
        let gone: Vec<PlayerId> = self
            .peers
            .keys()
            .copied()
            .filter(|who| !peers.iter().any(|(q, _)| q == who))
            .collect();
        for who in gone {
            if let Some(mut peer) = self.peers.remove(&who) {
                if peer.body.frame(p, None) == Step::Removed {
                    self.say(format!("co-op live: the avatar of player {} left", who.0));
                }
            }
        }
        for (who, s) in peers {
            let remote = remote_of(s);
            let peer = self.peers.entry(*who).or_default();
            if peer.failed {
                continue;
            }
            if peer.body.avatar.is_none() && !self.entered[4] {
                self.entering(4, format!("placing an avatar for player {}", who.0));
            }
            let peer = self.peers.entry(*who).or_default();
            let step = peer.body.frame(p, Some(&remote));
            match step {
                Step::Made(id) => {
                    peer.placed += 1;
                    let (placed, space) = (peer.placed, peer.body.space);
                    if placed > 1 {
                        self.say(format!(
                            "co-op live: player {} has avatar {id:08x} in {space:08x} (placed {placed} times)",
                            who.0
                        ));
                    }
                    self.result(4, true, format!("player {} has avatar {id:08x}", who.0));
                }
                Step::Driven => {
                    peer.driven += 1;
                    if peer.driven == 1 {
                        let id = peer.body.avatar.map_or(0, |a| a.id);
                        self.entering(5, format!("driving avatar {id:08x}"));
                    }
                    let peer = &self.peers[who];
                    if peer.driven == DRIVES_TO_PASS {
                        let id = peer.body.avatar.map_or(0, |a| a.id);
                        self.result(
                            5,
                            true,
                            format!("avatar {id:08x} driven {DRIVES_TO_PASS} times"),
                        );
                    }
                }
                Step::Removed => {
                    self.say(format!(
                        "co-op live: the avatar of player {} left (its space is not here)",
                        who.0
                    ));
                }
                Step::Absent => {}
                Step::Lost(why) => {
                    if peer.body.base.is_none() || peer.placed == 0 {
                        peer.failed = true;
                        self.result(4, false, format!("{why:?}"));
                    } else {
                        self.say(format!(
                            "co-op live: the avatar of player {} is gone ({why:?}); placing it again",
                            who.0
                        ));
                    }
                }
            }
        }
    }

    /// Removes every avatar (the session ends).
    pub fn close(&mut self, p: &mut dyn Program) {
        for (_, mut peer) in std::mem::take(&mut self.peers) {
            peer.body.frame(p, None);
        }
    }

    /// The avatars in this game now, each with its player's slot: for the avatars' damage
    /// and save rules (the glue's avatar guard).
    pub fn bodies(&self) -> Vec<(Avatar, u8)> {
        self.peers
            .iter()
            .filter_map(|(who, peer)| peer.body.avatar.map(|a| (a, who.0)))
            .collect()
    }

    /// The avatar of player `slot` left the game for a save or a load. After a save it is
    /// placed again; after a load its base copy is gone too, and both are made again.
    pub fn avatar_left(&mut self, slot: u8, why: Left) {
        if let Some(peer) = self.peers.get_mut(&PlayerId(slot)) {
            peer.body.avatar = None;
            if why == Left::Load {
                peer.body.base = None;
            }
        }
    }

    /// The avatar ids in this game now, each with its player.
    pub fn avatars_now(&self) -> Vec<(PlayerId, u32)> {
        self.peers
            .iter()
            .filter_map(|(who, peer)| peer.body.avatar.map(|a| (*who, a.id)))
            .collect()
    }

    /// The highest rung that reported, and one status phrase for the log.
    pub fn status(&self) -> String {
        let top = (1..=5).rev().find(|n| self.reported[*n]).unwrap_or(0);
        let avatars: Vec<String> = self
            .peers
            .iter()
            .filter_map(|(who, peer)| {
                peer.body.avatar.map(|a| {
                    format!(
                        "player {} avatar {:08x} driven {} placed {} times",
                        who.0, a.id, peer.driven, peer.placed
                    )
                })
            })
            .collect();
        format!(
            "live: last rung reported {top}, warps {}, {}",
            self.warps,
            if avatars.is_empty() {
                "no avatars".into()
            } else {
                avatars.join("; ")
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use coop_engine::fake::FakeProgram;

    const HOUSE: u32 = 0x0010_3df9;
    const WORLD: u32 = 0x000d_a726;
    const OUTSIDE: u32 = 0x000d_e0f1;

    fn sample(space: u32, x: f32) -> Sample {
        Sample {
            cell: CellId(space),
            pos: [x, 10.0, 7000.0],
            heading: 1.0,
            vel: [0.0; 3],
            anim: 0,
            weapon: 0,
            health: 0,
            flags: 0,
            seat: 0,
            extrapolated: false,
            teleported: false,
        }
    }

    /// One machine: a player in the house (with its 3D), the house attached, and the
    /// worldspace outside.
    fn machine() -> FakeProgram {
        let mut p = FakeProgram::new();
        let player = p.add_player([1.0, 2.0, 3.0]);
        let house = p.add_cell(HOUSE, addr::cell_state::ATTACHED);
        let world = p.add_world(WORLD);
        p.add_exterior(OUTSIDE, world, addr::cell_state::ATTACHED);
        p.set_u32(player + addr::REF_CELL, house);
        p.set_u32(player + addr::REF_NODE, 0x3000_0000);
        p
    }

    /// A guest whose host is in another space asks the game once to move there, waits for
    /// the game's frame, and passes rung 3 when it arrives. Mutation twin: a host's ladder
    /// moves nobody.
    #[test]
    fn a_guest_goes_to_the_host_in_another_space() {
        let mut p = machine();
        let mut ladder = Ladder::new(true, Cap::Avatar).quiet();
        let host = [(PlayerId::HOST, sample(WORLD, -80.0))];
        let me = ladder.frame(&mut p, 0, &host).unwrap();
        assert_eq!(me.space, HOUSE);
        assert_eq!(ladder.warps, 1);
        assert_eq!(p.moves.len(), 1);
        ladder.frame(&mut p, 16, &host);
        assert_eq!(p.moves.len(), 1, "one request while it waits");
        assert!(p.carry_out_move());
        let me = ladder.frame(&mut p, 32, &host).unwrap();
        assert_eq!(me.space, WORLD);
        assert!(ladder.reported(3));
        assert!(
            ladder
                .lines
                .iter()
                .any(|l| l.starts_with("COOP-RUNG 3 PASS moved")),
            "{:?}",
            ladder.lines
        );
        // With the host now, the guest sees the host's avatar.
        assert!(ladder.reported(4), "{:?}", ladder.lines);

        let mut twin = machine();
        let mut host_ladder = Ladder::new(false, Cap::Avatar).quiet();
        host_ladder.frame(&mut twin, 0, &[(PlayerId(1), sample(WORLD, 0.0))]);
        assert!(twin.moves.is_empty(), "the twin: a host is not moved");
    }

    /// A player not in the world yet (at the menu) is not moved, and the log says so once.
    /// A busy player is asked again later, not failed.
    #[test]
    fn a_guest_at_the_menu_or_busy_is_not_moved() {
        let mut p = machine();
        let player = p.u32(addr::PLAYER);
        p.set_u32(player + addr::REF_NODE, 0);
        let mut ladder = Ladder::new(true, Cap::Warp).quiet();
        let host = [(PlayerId::HOST, sample(WORLD, 0.0))];
        for t in 0..20 {
            ladder.frame(&mut p, t * 1000, &host);
        }
        assert!(p.moves.is_empty());
        assert_eq!(
            ladder
                .lines
                .iter()
                .filter(|l| l.contains("press New Game"))
                .count(),
            1
        );
        p.set_u32(player + addr::REF_NODE, 0x3000_0000);
        p.player_busy = true;
        ladder.frame(&mut p, 30_000, &host);
        assert!(!ladder.reported(3), "busy is not a failure");
        p.player_busy = false;
        ladder.frame(&mut p, 31_000, &host);
        assert!(p.moves.is_empty(), "asked again only after a while");
        ladder.frame(&mut p, 36_000, &host);
        assert_eq!(p.moves.len(), 1);
    }

    /// Rungs 4 and 5: a remote player in this space gets an avatar, driven each frame; rung 5
    /// passes after its count. In another space the avatar is placed anew. A player who leaves
    /// takes its avatar. Mutation twin: a ladder capped at the move places no avatar.
    #[test]
    fn a_remote_player_is_seen_and_placed_anew_in_another_space() {
        let mut p = machine();
        let mut ladder = Ladder::new(false, Cap::Avatar).quiet();
        let guest = PlayerId(1);
        for t in 0..=DRIVES_TO_PASS {
            ladder.frame(&mut p, t * 16, &[(guest, sample(HOUSE, t as f32))]);
        }
        assert!(
            ladder.reported(4) && ladder.reported(5),
            "{:?}",
            ladder.lines
        );
        let first = ladder.avatars_now()[0].1;
        // The host goes outdoors, and so does the guest.
        let player = p.u32(addr::PLAYER);
        let outside = p.call(addr::FORM_BY_ID, &[OUTSIDE]).eax;
        p.set_u32(player + addr::REF_CELL, outside);
        ladder.frame(&mut p, 5000, &[(guest, sample(WORLD, 0.0))]);
        let second = ladder.avatars_now()[0].1;
        assert_ne!(first, second, "placed anew");
        assert!(ladder.lines.iter().any(|l| l.contains("placed 2 times")));
        ladder.frame(&mut p, 5016, &[]);
        assert!(ladder.avatars_now().is_empty(), "the player left");
        assert_eq!(p.call(addr::FORM_BY_ID, &[second]).eax, 0);

        let mut twin = machine();
        let mut capped = Ladder::new(false, Cap::Warp).quiet();
        capped.frame(&mut twin, 0, &[(guest, sample(HOUSE, 0.0))]);
        assert!(capped.avatars_now().is_empty());
        assert!(!capped.reported(4));
    }

    /// An avatar that left for a save is placed again from the same base copy; after a load
    /// the base copy is made again too. Mutation twin: with nothing left, the avatar is driven.
    #[test]
    fn an_avatar_that_left_for_a_save_or_a_load_comes_back() {
        let mut p = machine();
        let mut ladder = Ladder::new(false, Cap::Avatar).quiet();
        let guest = [(PlayerId(1), sample(HOUSE, 0.0))];
        ladder.frame(&mut p, 0, &guest);
        let (first, slot) = ladder.bodies()[0];
        assert_eq!(slot, 1);
        ladder.frame(&mut p, 16, &guest);
        assert_eq!(ladder.bodies()[0].0, first, "the twin: driven, not placed");
        ladder.avatar_left(1, Left::Save);
        first.remove(&mut p);
        ladder.frame(&mut p, 32, &guest);
        let second = ladder.bodies()[0].0;
        assert_ne!(first, second);
        let copies = |p: &FakeProgram| p.calls.iter().filter(|(a, _)| *a == addr::NPC_COPY).count();
        assert_eq!(copies(&p), 1, "the same base copy");
        ladder.avatar_left(1, Left::Load);
        second.remove(&mut p);
        ladder.frame(&mut p, 48, &guest);
        assert_eq!(copies(&p), 2, "a new base copy after a load");
        assert_eq!(ladder.bodies().len(), 1);
    }

    #[test]
    fn the_cap_parses_and_the_speed_picks_the_gait() {
        assert_eq!(Cap::parse("log"), Cap::Log);
        assert_eq!(Cap::parse(" warp "), Cap::Warp);
        assert_eq!(Cap::parse(""), Cap::Avatar);
        let mut s = sample(HOUSE, 0.0);
        assert_eq!(remote_of(&s).drive.flags, 0);
        s.vel = [100.0, 0.0, 0.0];
        assert_eq!(
            remote_of(&s).drive.flags,
            addr::move_flag::FORWARD | addr::move_flag::WALK
        );
        s.vel = [0.0, 300.0, 0.0];
        assert_eq!(
            remote_of(&s).drive.flags,
            addr::move_flag::FORWARD | addr::move_flag::RUN
        );
        assert!((remote_of(&s).drive.heading - 1.0f32.to_degrees()).abs() < 1e-3);
    }
}
