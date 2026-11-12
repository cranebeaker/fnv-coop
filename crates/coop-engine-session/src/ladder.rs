//! The live ladder (docs/coop/DECISIONS.md D3): players in the world, one logged rung at a time, so
//! a run on a PC names the step that failed.
//!
//! 1. This game's player is read (cell, position, 3D).
//! 2. Another player's state arrives.
//! 3. A guest's player, once it is in the world itself, is moved to the host (a position request,
//!    the way a door moves the player), again when the host is in another space.
//! 4. A remote player in this player's cell gets an avatar (`PlaceAtMe`, AI off).
//! 5. The avatar is moved every frame; its AI is kept off.
//!
//! Each rung writes `COOP-RUNG n entering ...` before its first call into the game, at once (a
//! crash still names the rung), and `COOP-RUNG n PASS|FAIL ...` once. [`Cap`] stops the ladder at a
//! rung without a rebuild.

use std::collections::BTreeMap;
use std::io::Write as _;

use coop_core::avatar::{quant_heading, quant_pos, AvatarState, Sample};
use coop_core::ids::{CellId, PlayerId};
use coop_engine::live::{self, Avatar, Local};
use engine_core::Engine;

/// How far up the ladder a run goes (`NV_COOP_LIVE`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cap {
    /// Rungs 1 and 2: states are read, sent and logged; the game is not touched.
    Log,
    /// Up to rung 3: a guest is also moved to the host's cell.
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

/// The base form an avatar is placed from unless told otherwise: the player's own NPC.
pub const DEFAULT_AVATAR_BASE: u32 = 7;

/// Frames a warp may take before it is reported as failed.
const WARP_FRAMES: u32 = 600;
/// At least this long between two warps of a guest.
const WARP_EVERY_MS: u64 = 5000;
/// How long a guest's player may be out of the world, with the host in it, before the log says so.
const HINT_AFTER_MS: u64 = 15_000;
/// Moves of an avatar before rung 5 reports.
const MOVES_TO_PASS: u64 = 100;

/// This game's player as the state sent to the others.
pub fn state_of(l: &Local) -> AvatarState {
    AvatarState {
        cell: CellId(l.cell),
        pos: l.pos.map(quant_pos),
        heading: quant_heading(l.heading),
        ..AvatarState::default()
    }
}

#[derive(Debug, Default)]
struct Peer {
    avatar: Option<Avatar>,
    /// The space (interior cell or worldspace) the avatar was placed in. Moving an avatar changes
    /// its place, not its cell: one placed elsewhere stays there, unseen (PC run, 2026-11-10: a
    /// guest moved outdoors to the host kept the avatar it had placed indoors).
    space: u32,
    /// Avatars placed for this player so far.
    placed: u32,
    /// Placing failed: not tried again in this run.
    failed: bool,
    moved: u64,
    woke: u64,
    off_by: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Warp {
    Idle,
    Waiting { cell: u32, frames: u32 },
}

/// How long a player that has walked out of its opening's room is left before it follows the
/// host (the trigger at the door runs on the next frames).
pub const OUTSIDE_FIRST_MS: u64 = 2000;

/// The ladder's state on one machine.
pub struct Ladder {
    guest: bool,
    cap: Cap,
    base: u32,
    emit: fn(&str),
    /// Every line written, in order.
    pub lines: Vec<String>,
    started: bool,
    entered: [bool; 6],
    reported: [bool; 6],
    /// Guest: a quest of this player's own and one of its stages; until that stage is done in
    /// this game the player is not moved to the host. A player plays the opening in its own game,
    /// and a host that leaves the house first would otherwise pull the guest out of it.
    pub follow_after: Option<(u32, u8)>,
    /// The room this player's opening was played in, while it has not walked out of it.
    opening_room: Option<u32>,
    /// When this player was first seen out of that room.
    outside_since: Option<u64>,
    stay_said: bool,
    warp: Warp,
    last_warp_ms: Option<u64>,
    warps: u32,
    /// When the host was first known while this guest's player was not in the world.
    waiting_since: Option<u64>,
    hinted: bool,
    peers: BTreeMap<PlayerId, Peer>,
}

fn print_now(line: &str) {
    println!("{line}");
    let _ = std::io::stdout().flush();
}

fn dist(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
}

impl Ladder {
    /// A ladder for a guest or a host, up to `cap`, with avatars of base form `base`. Lines are
    /// printed at once and kept in [`Ladder::lines`].
    pub fn new(guest: bool, cap: Cap, base: u32) -> Ladder {
        Ladder {
            guest,
            cap,
            base,
            emit: print_now,
            lines: Vec::new(),
            started: false,
            entered: [false; 6],
            reported: [false; 6],
            follow_after: None,
            opening_room: None,
            outside_since: None,
            stay_said: false,
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

    /// One frame, between two of the game's frames. `peers` are the other players' interpolated
    /// states (those in the world). Returns this player's place, for sending.
    pub fn frame(
        &mut self,
        e: &mut Engine,
        now_ms: u64,
        peers: &[(PlayerId, Sample)],
    ) -> Option<Local> {
        if !self.started {
            self.started = true;
            self.say("COOP-RUNG 1 entering: reading this game's player".into());
        }
        let mut me = live::local(e);
        if let Some(l) = me {
            self.result(
                1,
                true,
                format!(
                    "me in cell {:08x} at ({:.0}, {:.0}, {:.0}), 3D {}",
                    l.cell,
                    l.pos[0],
                    l.pos[1],
                    l.pos[2],
                    if l.loaded { "yes" } else { "no" }
                ),
            );
        }
        if let Some((p, s)) = peers.first() {
            self.result(
                2,
                true,
                format!(
                    "player {} in cell {:08x} at ({:.0}, {:.0}, {:.0})",
                    p.0, s.cell.0, s.pos[0], s.pos[1], s.pos[2]
                ),
            );
        }
        if self.guest && self.cap >= Cap::Warp {
            if let Some((_, host)) = peers.iter().find(|(p, _)| p.is_host()) {
                let host = *host;
                self.warp_step(e, now_ms, &host, me);
                me = live::local(e);
            }
        }
        // No avatar is placed or moved while this player's own move to another cell is under way.
        if self.cap >= Cap::Avatar && self.warp == Warp::Idle {
            self.avatars(e, me, peers);
        }
        me
    }

    fn warp_step(&mut self, e: &mut Engine, now_ms: u64, host: &Sample, me: Option<Local>) {
        let host_cell = host.cell.0;
        // Only a player that is in the world itself is moved: at the main menu, or before a new
        // game has put the player somewhere, the game is not in a state to move it (PC run,
        // 2026-11-10: a move from the menu never finished).
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
        // The opening is this player's own until it has walked out of the room it was played in:
        // the stage that ends it is set indoors, and the trigger outside the door gives the player
        // their weapons back (PC run 19: a guest moved to the host as soon as the stage was done
        // never stepped on it, and could draw no weapon). A player that joins with its opening
        // long done is not held.
        let opening_running = self.follow_after.is_some_and(|(quest, stage)| {
            coop_engine::quest::record(e, quest).is_some_and(|r| !r.done.contains(&stage))
        });
        if opening_running {
            self.opening_room.get_or_insert(me.cell);
            self.outside_since = None;
        }
        let in_opening_room = self.opening_room == Some(me.cell);
        if !opening_running && !in_opening_room {
            if let Some(room) = self.opening_room {
                let since = *self.outside_since.get_or_insert(now_ms);
                if now_ms.saturating_sub(since) < OUTSIDE_FIRST_MS {
                    return;
                }
                let _ = room;
                self.opening_room = None;
            }
        }
        if matches!(self.warp, Warp::Idle)
            && me.cell != host_cell
            && (opening_running || in_opening_room)
        {
            if !self.stay_said {
                self.stay_said = true;
                self.say(
                    "co-op live: the host is elsewhere; this player follows once its own opening is done and it has walked out"
                        .into(),
                );
            }
            return;
        }
        match self.warp {
            Warp::Waiting { cell, frames } => {
                if me.cell == cell {
                    self.warp = Warp::Idle;
                    self.result(3, true, format!("moved to the host in {cell:08x}"));
                } else if frames >= WARP_FRAMES {
                    self.warp = Warp::Idle;
                    self.result(
                        3,
                        false,
                        format!(
                            "still in {:08x} {WARP_FRAMES} frames after asking to move to {cell:08x}",
                            me.cell
                        ),
                    );
                } else {
                    self.warp = Warp::Waiting {
                        cell,
                        frames: frames + 1,
                    };
                }
            }
            Warp::Idle if me.cell == host_cell => {
                self.result(3, true, format!("already with the host in {host_cell:08x}"));
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
                    format!("asking the game to move this player to the host in {host_cell:08x}"),
                );
                match live::warp(e, host_cell, host.pos, host.heading) {
                    Ok(()) => {
                        self.warps += 1;
                        self.warp = Warp::Waiting {
                            cell: host_cell,
                            frames: 0,
                        };
                    }
                    // Not a failure of the rung: the game says not now; asked again later.
                    Err(live::Refusal::Busy) => {}
                    Err(why) => self.result(3, false, why.to_string()),
                }
            }
        }
    }

    fn avatars(&mut self, e: &mut Engine, me: Option<Local>, peers: &[(PlayerId, Sample)]) {
        // A player who left, or is not in the world: their avatar goes.
        let gone: Vec<PlayerId> = self
            .peers
            .keys()
            .copied()
            .filter(|p| !peers.iter().any(|(q, _)| q == p))
            .collect();
        for p in gone {
            self.remove(e, p, "left");
            self.peers.remove(&p);
        }
        for (p, s) in peers {
            let here = me.is_some_and(|l| l.loaded && l.cell == s.cell.0);
            if !here {
                self.remove(e, *p, "is in another cell");
                continue;
            }
            let space = s.cell.0;
            if self
                .peers
                .get(p)
                .is_some_and(|peer| peer.avatar.is_some() && peer.space != space)
            {
                self.remove(e, *p, "was placed in another cell");
            }
            let peer = self.peers.entry(*p).or_default();
            if peer.avatar.is_none() && !peer.failed {
                let base = self.base;
                self.entering(
                    4,
                    format!("PlaceAtMe of base form {base:08x} for player {}", p.0),
                );
                match live::avatar_create(e, base) {
                    Ok(a) => {
                        let peer = self.peers.entry(*p).or_default();
                        peer.avatar = Some(a);
                        peer.space = space;
                        peer.placed += 1;
                        if peer.placed > 1 {
                            let n = peer.placed;
                            self.say(format!(
                                "co-op live: player {} has avatar {:08x} in {space:08x} (placed {n} times)",
                                p.0, a.id
                            ));
                        }
                        self.result(
                            4,
                            true,
                            format!("player {} has avatar {:08x} (base {base:08x})", p.0, a.id),
                        );
                    }
                    Err(why) => {
                        self.peers.entry(*p).or_default().failed = true;
                        self.result(4, false, why.to_string());
                    }
                }
            }
            let Some(a) = self.peers.entry(*p).or_default().avatar else {
                continue;
            };
            self.entering(5, format!("moving avatar {:08x}", a.id));
            match live::avatar_hold(e, a, s.pos, s.heading) {
                Some(h) => {
                    let peer = self.peers.entry(*p).or_default();
                    peer.moved += 1;
                    peer.woke += u64::from(h.woke);
                    peer.off_by = dist(h.pos, s.pos);
                    if peer.moved == MOVES_TO_PASS {
                        let (woke, off) = (peer.woke, peer.off_by);
                        self.result(
                            5,
                            true,
                            format!(
                                "avatar {:08x} moved {MOVES_TO_PASS} times, AI woke {woke} times, off by {off:.1} units",
                                a.id
                            ),
                        );
                    }
                }
                None => {
                    self.peers.entry(*p).or_default().avatar = None;
                    self.say(format!(
                        "co-op live: avatar {:08x} of player {} is gone from the game; placing it again",
                        a.id, p.0
                    ));
                }
            }
        }
    }

    fn remove(&mut self, e: &mut Engine, p: PlayerId, why: &str) {
        let Some(a) = self.peers.get_mut(&p).and_then(|peer| peer.avatar.take()) else {
            return;
        };
        live::avatar_remove(e, a);
        self.say(format!(
            "co-op live: avatar {:08x} removed (player {} {why})",
            a.id, p.0
        ));
    }

    /// fights: the other players' avatars in this game now, as (player, the avatar's form id).
    pub fn avatar_ids(&self) -> Vec<(PlayerId, u32)> {
        self.peers
            .iter()
            .filter_map(|(p, peer)| Some((*p, peer.avatar?.id)))
            .collect()
    }

    /// Removes every avatar (the session ends).
    pub fn close(&mut self, e: &mut Engine) {
        for p in self.peers.keys().copied().collect::<Vec<_>>() {
            self.remove(e, p, "session ended");
        }
    }

    /// The highest rung that passed, and one status phrase for the log.
    pub fn status(&self) -> String {
        let top = (1..=5).rev().find(|n| self.reported[*n]).unwrap_or(0);
        let avatars: Vec<String> = self
            .peers
            .iter()
            .filter_map(|(p, peer)| {
                peer.avatar.map(|a| {
                    format!(
                        "player {} avatar {:08x} moved {} woke {} off {:.1}",
                        p.0, a.id, peer.moved, peer.woke, peer.off_by
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
