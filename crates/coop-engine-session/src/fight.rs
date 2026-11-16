//! Shared fights (docs/coop/SPECS.md S15; docs/coop/PLAN.md M6): the host's game is the
//! authority for actors that are not players.
//!
//! **State.** About 20 times a second the host reads the actors its game runs around its player
//! (`coop_engine::fight::near`) and tracks each placed one in coop-core's actor ledger
//! (`ActorHost`), pinned to the host as its owner, so the ledger's own rules announce it to the
//! guests in that space and relay its states (place, heading, health, life, combat flag, combat
//! target). A guest holds every such actor that is in its own space the way an avatar is held:
//! AI off, moved to the host's place, health set to the host's, killed through the game when the
//! host says it died. An actor the host no longer runs is given back to the guest's game.
//!
//! **Damage.** A guest's weapon hit on a held actor never touches the guest's copy: the hit wrap
//! keeps it, and it goes to the host as coop-core's `ActorMsg::Hit`. The ledger judges it (one
//! per id, a rate limit, the same space, not dead, an essential actor never below 1), and what
//! it accepts the host's game applies through its own damage applier, credited to the guest's
//! avatar; a kill the ledger decides is the game's `Actor::Kill`. The result comes back in the
//! actor's next state, and a death also through the story channel. A hit of the host's actors on
//! a guest's avatar is kept from the avatar and becomes coop-core's `PlayerHurt` through the
//! host's vitals; the guest's game applies it to its own player.
//!
//! **Threat.** Twice a second coop-core's `ThreatDirector` picks, for each of the host's actors
//! that is fighting a player, the player it attends to; when that is another player than the
//! game's own choice the host's game is asked to start that combat (the body of `StartCombat`),
//! against the guest's avatar or the host's player. An actor fighting someone who is not a
//! player is left alone. Avatars stop being ghosts so that they can be fought (on a guest too: its
//! game runs the actors near its player after a hand-off).
//!
//! **Downed and revive.** In a session a player does not die at once: the game's kill of its
//! own player is held, the player is left with 1 health and no controls, and the machine tells
//! the host (`RulesMsg::ReportDown`). coop-core's `Reviver` runs the rest on the host: another
//! player standing next to the downed one (this machine asks for them, there is no key to hold
//! yet) revives them after its hold time at its share of health; nobody coming in its window is
//! a respawn (up again at full health where they lie); everyone down is a wipe, and then every
//! game lets its player die as the game does.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;

use crate::track::{Place, Track};
use coop_core::actors::{ActorMsg, ActorState};
use coop_core::avatar::{
    dequant_heading, dequant_pos, quant_heading, quant_pos, AvatarState, FLAG_DEAD as AVATAR_DEAD,
    FLAG_DOWNED as AVATAR_DOWNED, FLAG_OPENING as AVATAR_OPENING,
};
use coop_core::combat::{CombatMsg, DamageKind, ThreatInput, LIMB_COUNT};
use coop_core::ids::{CellId, FormId, PlayerId, Seq};
use coop_core::interest::Audience;
use coop_core::proto::Msg;
use coop_core::rules::revive::CLIENT_RANGE;
use coop_core::rules::{Notice, RulesMsg};
use coop_core::sync::{GuestLive, GuestPhase, GuestSync, HostSync};
use coop_engine::addr::PLAYER_ALL_CONTROLS;
use coop_engine::fight::{self, Role, Snap};
use coop_engine::puppet::{self, Aim, Attack};
use coop_engine::{actors, gate, harm, live};

use crate::{smooth, vats};
use engine_core::Engine;

/// Following (see [`Fight::follow`]): how far from the host's place an actor may be, in the
/// game's units (about three metres), and for how long, before it is put there.
pub const FOLLOW_DISTANCE: f32 = 200.0;

/// How far a copy a guest's game takes over may be from where the host's game had it before it
/// is put there (units): a copy that stayed behind while nobody was near, not one that walked a
/// few steps of its own.
pub const TAKEOVER_DRIFT: f32 = 1000.0;
/// See [`FOLLOW_DISTANCE`].
pub const FOLLOW_MS: u64 = 1500;
/// How long a copy in a conversation in this game is left to it (not led, put back or asked to
/// take the host's package): enough for a scene's lines (PC run 30: Joe Cobb and Trudy), not a
/// conversation that never moves on (the harness: a scene replayed in the guest's game stood
/// still with nobody speaking), after which it is followed again.
pub const TALK_GRACE_MS: u64 = 30_000;

/// A copy in another space than the host's actor is brought through at most this often.
pub const BRING_EVERY_MS: u64 = 3000;

/// A followed actor that does not run the host's package is told to pick again this often at
/// most, and this many times for one package.
pub const PACKAGE_ASK_MS: u64 = 3000;
/// See [`PACKAGE_ASK_MS`].
pub const PACKAGE_ASKS_AT_MOST: u32 = 5;
/// A copy is brought from another space this many times at most.
pub const BRING_AT_MOST: u32 = 3;

/// How much of the fights a run shares (`NV_COOP_FIGHT`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cap {
    /// The host's actors are shown on the guests; every game still applies its own hits.
    State,
    /// Also: hits go through the host.
    Damage,
    /// Also: the host's actors can turn on a guest.
    Threat,
    /// Also: a player who would die is down and can be revived.
    Revive,
}

impl Cap {
    /// `state`, `damage` or `threat`; anything else, or nothing, is everything this build shares.
    pub fn parse(v: &str) -> Cap {
        match v.trim() {
            "state" => Cap::State,
            "damage" => Cap::Damage,
            "threat" => Cap::Threat,
            _ => Cap::Revive,
        }
    }
}

/// `ActorState::flags`: the actor is in combat.
pub const FLAG_COMBAT: u16 = 1;
/// `ActorState::flags`: the actor is dying or dead.
pub const FLAG_DEAD: u16 = 2;
/// `ActorState::target` for a target that is a player: this plus the player's slot (the id
/// coop-core's vitals use for a player as an attacker).
pub const PLAYER_TARGET_BASE: u32 = 0xfe00_0000;
/// How near a guest's avatar an actor the host's game has loaded must be to be tracked (and
/// so be run by the nearest player's game).
pub const HANDOFF_RADIUS: f32 = 4096.0;
/// Host: at least this long between two changes of who an actor that greets players is run by.
pub const GREET_HOLD_MS: u64 = 5000;

/// How long an actor must have been in the host's close-actors list without a break before it
/// counts as run in full by the host's game (ms). The game's list leaves an actor out and takes
/// it back from one moment to the next near its edge.
pub const CLOSE_SETTLE_MS: u64 = 5_000;
/// How far a copy whose package is aimed at the player may be from the host's place.
pub const AIMED_DISTANCE: f32 = 800.0;
/// The host reads and publishes its actors this often.
pub const PUBLISH_EVERY_MS: u64 = 50;
/// The host's threat director looks at the fights this often.
pub const THREAT_EVERY_MS: u64 = 500;

/// A helper whose request to revive got no result asks again after this long (longer than the
/// reviver's hold time).
pub const REVIVE_ASK_EVERY_MS: u64 = 4000;
/// After a wipe the player's death is held again this much later (the game was loaded again).
pub const SPARE_AGAIN_MS: u64 = 10_000;
// vats
/// Health lost outside `Actor::HitMe` (a fall, a trap, an effect) is taken from the game and
/// sent on this often, as whole points (docs/coop/SPECS.md S22).
pub const HARM_EVERY_MS: u64 = 100;

/// The player `ActorState::target` names, if it names one.
pub fn target_player(target: u32) -> Option<PlayerId> {
    (target & 0xff00_0000 == PLAYER_TARGET_BASE).then_some(PlayerId(target as u8))
}

/// A copy whose combat group does not know its owner's target yet is asked to start a combat
/// with it at most this often (ms).
pub const AIM_START_EVERY_MS: u64 = 1_000;
/// A copy that fights here while its owner's actor fights nobody stops after this long (ms): the
/// owner's game may take a moment to take up a fight a player began here.
pub const CALM_AFTER_MS: u64 = 2_000;
/// A copy that went for this game's player asks its owner's game to take the fight up at most
/// this often (ms).
pub const ENGAGE_EVERY_MS: u64 = 2_000;

/// The copies in this game of actors another game runs, shown doing what the owner's actor does
/// (docs/coop/DECISIONS.md D6, stage B): fighting its target, attacking when it attacks.
#[derive(Debug, Default)]
pub struct Puppeting {
    /// The owner's attack count last seen for each copy.
    seen: BTreeMap<u32, u8>,
    /// When each copy was last asked to start a combat with its owner's target.
    started: BTreeMap<u32, u64>,
    /// Since when each copy has fought here while its owner's actor fought nobody.
    calm_since: BTreeMap<u32, u64>,
    /// The copies named to the wraps this frame, each with its owner's target here.
    copies: BTreeMap<u32, u32>,
    /// Copies turned to their owner's target.
    pub turns: u32,
    /// Combats started so that a copy's group knows its owner's target.
    pub starts: u32,
    /// Combats of copies ended because their owner's actor fought nobody.
    pub stops: u32,
    /// The owner's attacks replayed on a copy.
    pub replays: u32,
    /// Replays the copy's game did not queue (its weapon, ammunition or animation).
    pub replays_refused: u32,
    /// Copies that went for this game's player while their owner's actor fought nobody, this
    /// frame: the owner's game is asked to take that fight up.
    engage: Vec<u32>,
    /// When each copy last asked so.
    asked: BTreeMap<u32, u64>,
    /// Such asks sent.
    pub asks: u32,
    /// Fights this game's actors took up for another game's copy.
    pub taken_up: u32,
}

impl Puppeting {
    /// One phrase for the log.
    pub fn status(&self) -> String {
        format!(
            "copies {}: turned to their owner's target {}, combats started {}, stopped {}, attacks replayed {} (not queued {}), fights asked of the owner {}, taken up for a copy {}",
            self.copies.len(),
            self.turns,
            self.starts,
            self.stops,
            self.replays,
            self.replays_refused,
            self.asks,
            self.taken_up
        )
    }
}

/// After the owner's actor has stood this long (ms), its copy is left to its own AI (its package
/// is the owner's): one that only paused is kept on the owner's path, standing.
pub const SETTLE_MS: i64 = 1_500;
/// How fast a copy left too far from where the owner's actor stands walks back there (units a
/// second; about a walk).
pub const GLIDE_SPEED: f32 = 150.0;
/// A copy this far from where it is to be is put there at once (units).
pub const SNAP_DIST: f32 = 1024.0;
/// A copy further than this from its owner's path (units) catches up rather than being put on
/// it ...
pub const CATCH_DIST: f32 = 64.0;
/// ... this fast (units a second; about twice a run).
pub const CATCH_SPEED: f32 = 900.0;
/// The movement flags a copy walking back is given: forward, walking.
const WALK_FORWARD: u16 = 0x101;

/// How a copy is followed ([`Fight::follow_copy`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Follow {
    /// Its owner's actor fights: on its path every frame.
    Fight,
    /// On its path while the owner's actor moves or has just stopped, its own AI's after.
    Path,
    /// Only walked back when further than [`AIMED_DISTANCE`]: a copy whose package is aimed at
    /// the player, which may walk to this game's player while the owner's walks to its own.
    WalkBack,
}

/// Where the copies of actors another game runs are shown (docs/coop/DECISIONS.md D6, stage B):
/// along the owner's path a little behind its clock ([`crate::track`]), with the owner's gait.
#[derive(Debug, Default)]
pub struct Following {
    /// Each copy's owner and its states.
    tracks: BTreeMap<u32, (Option<PlayerId>, Track)>,
    /// The copies whose movement flags this game forces, and the flags.
    forced: BTreeMap<u32, u16>,
    /// The copies forced this frame.
    kept: BTreeSet<u32>,
    /// The copies whose place was asked this frame.
    seen: BTreeSet<u32>,
    /// Copies walking back to where their owner's actor stands.
    gliding: BTreeSet<u32>,
    /// The frame before this one (ms).
    last_ms: Option<u64>,
    /// This frame's length (ms).
    dt_ms: u64,
    /// Frames a copy was put on its owner's path.
    pub leads: u64,
    /// Copies put where they are to be at once, being too far.
    pub snaps: u32,
    /// Copies in a cell this game had let go, brought to their place near this player.
    pub brought: u32,
}

impl Following {
    /// One phrase for the log.
    pub fn status(&self) -> String {
        format!(
            "copies shown along their owner's path {} frames, put at once {}, brought into this player's cells {}",
            self.leads, self.snaps, self.brought
        )
    }
}

/// The form id in this game of an actor state's target (`ActorState::target`): this game's
/// player, another player's avatar, or the actor it names; 0 for none or a player without an
/// avatar here.
pub fn local_target(target: u32, me: PlayerId, avatars: &[(PlayerId, u32)]) -> u32 {
    match target_player(target) {
        Some(p) if p == me => actors::PLAYER_REF_ID,
        Some(p) => avatars.iter().find(|(q, _)| *q == p).map_or(0, |a| a.1),
        None => target,
    }
}

#[derive(Debug, Default)]
struct Tracked {
    seq: u32,
    last: Option<ActorState>,
    /// The actor as the game had it at the last read.
    snap: Option<Snap>,
}

/// The fights' state on one machine.
pub struct Fight {
    guest: bool,
    cap: Cap,
    emit: fn(&str),
    /// Every line written, in order.
    pub lines: Vec<String>,
    started: bool,
    /// The avatars the hit wrap knows.
    avatars: BTreeSet<u32>,
    /// Host: the actors in the ledger.
    tracked: BTreeMap<u32, Tracked>,
    published_ms: Option<u64>,
    threat_ms: Option<u64>,
    /// Guest: the actors held this frame.
    held: BTreeSet<u32>,
    /// Guest: a quest of this player's own and one of its stages; until that stage is done in
    /// this game no actor is held. A player plays the opening in its own game, with its own Doc
    /// Mitchell acting (PC run 8, 2026-11-10: held at the host's place with his AI off, he never
    /// started it, and the guest lay looking at the ceiling).
    pub hold_after: Option<(u32, u8)>,
    /// Guest: whether the last frame held nothing for that reason.
    waiting: bool,
    /// Guest: first holds written to the log so far.
    first_holds: u32,
    /// Guest: follow instead of hold. The host's actors keep their own AI in this game (so they
    /// walk, sit and talk animated, as PC run 10 showed a held actor does not) and one that has
    /// been further than [`FOLLOW_DISTANCE`] from the host's place for [`FOLLOW_MS`] is put
    /// there and picks its package again.
    pub follow: bool,
    /// Guest: since when each actor has been that far off.
    far_since: BTreeMap<u32, u64>,
    /// Since when each actor has been in a conversation in this game (see [`TALK_GRACE_MS`]).
    talk_since: BTreeMap<u32, u64>,
    /// How many times a copy was sent walking back to where its owner's actor is.
    pub nudges: u32,
    /// Guest: frames a followed actor was left alone because it was in a conversation here.
    pub talks_left: u64,
    /// Host: the package each tracked actor was last seen running.
    packages: BTreeMap<u32, u32>,
    /// Host: actors it ran near its player and no longer does, whose package it still says.
    watched: BTreeSet<u32>,
    /// Guest: when each actor was last told to evaluate because it did not run the host's
    /// package, and how many times that was done.
    package_asked: BTreeMap<u32, u64>,
    /// See [`Fight::package_asked`].
    pub package_asks: u32,
    /// Guest: for each actor, the package it was last asked to take and how many times.
    package_tries: BTreeMap<u32, (u32, u32)>,
    /// Guest: how many times each actor was brought from another space.
    brought_times: BTreeMap<u32, u32>,
    /// The parts of following, each with its own switch so that a fault can be traced to one of
    /// them (docs/coop/DECISIONS.md D5, condition 6): keeping a copy where its owner's actor
    /// stands (D7; off, it is its own AI's and walks back when it strays), bringing one from
    /// another space, giving the host's packages.
    pub lead_on: bool,
    /// See [`Fight::lead_on`].
    pub bring_on: bool,
    /// See [`Fight::lead_on`].
    pub packages_on: bool,
    /// Guest: when each actor was last brought from another space, and how many times one was.
    brought: BTreeMap<u32, u64>,
    /// See [`Fight::brought`].
    pub brings: u32,
    /// Guest: actors taken over from the host's game whose copy here was in another space and
    /// was brought out to where the host's game had them.
    pub takeover_brings: u32,
    /// Guest: actors not held because this game's copy is in another space than the host's.
    elsewhere: BTreeSet<u32>,
    /// Guest: actors whose host's copy has left this player's space while this game's has not.
    left_behind: BTreeSet<u32>,
    /// Hand-off: the nearest player's game runs an actor (`NV_COOP_HANDOFF=off`: the host's
    /// runs every one).
    pub handoff: bool,
    /// Host: the actors a guest's game runs now.
    theirs: BTreeSet<u32>,
    /// Host: frames a copy of an actor a guest runs was put at that guest's place.
    pub guest_run_moves: u64,
    /// Host: this player's hits on actors a guest runs, put into the ledger.
    pub hosts_hits_forwarded: u32,
    /// Host: hits on a guest's avatar by the copy of an actor a guest runs, left to the guests'
    /// games.
    pub theirs_hits_left: u32,
    /// Host: the id of the last such hit.
    next_hit: u32,
    /// Guest: states sent of the actors this game runs.
    pub owned_published: u64,
    /// Guest: damage the ledger said was done to an actor this game runs, taken here.
    pub owned_damage_taken: u32,
    /// Guest: when this game's actors' states were last sent.
    owned_published_ms: Option<u64>,
    /// Guest: the last state sent of each actor this game runs.
    owned_last: BTreeMap<u32, ActorState>,
    /// Guest: the actors this game ran at the last frame.
    owned_before: BTreeSet<u32>,
    /// Host: the player each actor last spoke with (a conversation's first line), whose game runs
    /// it while that player is near it.
    engaged: BTreeMap<u32, PlayerId>,
    /// Host: when each actor was last given to the player it came to greet.
    greeted_at: BTreeMap<u32, u64>,
    /// Host: actors given to the player they came to greet.
    pub greets_given: u32,
    /// Host: when the game's close-actors list last left each tracked actor out.
    left_out_at: BTreeMap<u32, u64>,
    /// Guest: frames an actor fighting on the host was kept at the host's place.
    pub fight_leads: u64,
    /// Host: how often the players of a space were told an actor had left it.
    pub left_told: u64,
    /// Host: states published.
    pub published: u64,
    /// Host: guests' hits its game applied.
    pub hits_applied: u32,
    /// Host: kills the ledger decided, enacted in its game.
    pub kills: u32,
    /// Host: hits of its game on a guest's avatar, sent to that guest as damage.
    pub hurts_sent: u32,
    /// Host: combats its game was asked to start because the threat director chose a player.
    pub retargets: u32,
    /// Guest: holds of an actor (one per actor and frame).
    pub moves: u64,
    /// Guest: times the game had switched a held actor's AI back on.
    pub woke: u64,
    /// Guest: actors given back to this game.
    pub released: u32,
    /// Guest: deaths the host announced, enacted here.
    pub deaths: u32,
    /// Guest: this player's hits sent to the host.
    pub hits_sent: u32,
    /// Guest: damage from the host applied to this game's player.
    pub hurts_taken: u32,
    /// This game's player is down.
    down: bool,
    /// The player's disabled-controls byte from before it went down.
    saved_controls: u8,
    /// The highest health this game's player had in the session (its full health, for a
    /// revive's share).
    max_health: f32,
    /// The downed player this one is reviving, and when it last asked.
    helping: Option<(PlayerId, u64)>,
    wiped_ms: Option<u64>,
    /// Times this game's player went down.
    pub downs: u32,
    /// Times this game's player got up again (revived, or respawned).
    pub ups: u32,
    /// Times everyone was down and this game's player died as the game does.
    pub wipes: u32,
    /// The copies of actors another game runs.
    pub puppets: Puppeting,
    /// Where those copies are shown.
    pub following: Following,
    // vats
    /// V.A.T.S. with several players (docs/coop/SPECS.md S21).
    pub vats: vats::Share,
    /// Losses of health outside `Actor::HitMe` decided as hits are (`NV_COOP_HARM=on`); off,
    /// every such loss is the game's own, as before.
    pub harm_on: bool,
    harm_ms: Option<u64>,
    /// Losses of health outside `Actor::HitMe` sent on: a guest's to the host, the host's to
    /// the guest whose avatar lost it.
    pub harms_sent: u32,
    /// Guest: held actors move between two states instead of jumping (`NV_COOP_SMOOTH=on`).
    pub smooth_on: bool,
    smooth: smooth::Smooth,
}

fn print_now(line: &str) {
    println!("{line}");
    let _ = std::io::stdout().flush();
}

fn health16(h: f32) -> u16 {
    h.ceil().clamp(0.0, f32::from(u16::MAX)) as u16
}

/// A hit's health damage as the points the wire carries; 0 for nothing.
fn damage16(amount: f32) -> u16 {
    if amount > 0.0 {
        amount.round().clamp(1.0, 5000.0) as u16
    } else {
        0
    }
}

fn dist(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
}

impl Fight {
    /// The fights of a guest or a host, up to `cap`. Lines are printed at once and kept in
    /// [`Fight::lines`].
    pub fn new(guest: bool, cap: Cap) -> Fight {
        Fight {
            guest,
            cap,
            emit: print_now,
            lines: Vec::new(),
            started: false,
            avatars: BTreeSet::new(),
            tracked: BTreeMap::new(),
            published_ms: None,
            threat_ms: None,
            held: BTreeSet::new(),
            hold_after: None,
            waiting: false,
            first_holds: 0,
            follow: false,
            far_since: BTreeMap::new(),
            talk_since: BTreeMap::new(),
            nudges: 0,
            talks_left: 0,
            packages: BTreeMap::new(),
            watched: BTreeSet::new(),
            package_asked: BTreeMap::new(),
            package_asks: 0,
            package_tries: BTreeMap::new(),
            brought_times: BTreeMap::new(),
            lead_on: true,
            bring_on: true,
            packages_on: true,
            brought: BTreeMap::new(),
            brings: 0,
            takeover_brings: 0,
            elsewhere: BTreeSet::new(),
            left_behind: BTreeSet::new(),
            handoff: true,
            theirs: BTreeSet::new(),
            guest_run_moves: 0,
            hosts_hits_forwarded: 0,
            theirs_hits_left: 0,
            next_hit: 0,
            owned_published: 0,
            owned_damage_taken: 0,
            owned_published_ms: None,
            owned_last: BTreeMap::new(),
            owned_before: BTreeSet::new(),
            left_out_at: BTreeMap::new(),
            engaged: BTreeMap::new(),
            greeted_at: BTreeMap::new(),
            greets_given: 0,
            fight_leads: 0,
            left_told: 0,
            published: 0,
            hits_applied: 0,
            kills: 0,
            hurts_sent: 0,
            retargets: 0,
            moves: 0,
            woke: 0,
            released: 0,
            deaths: 0,
            hits_sent: 0,
            hurts_taken: 0,
            down: false,
            saved_controls: 0,
            max_health: 0.0,
            helping: None,
            wiped_ms: None,
            downs: 0,
            ups: 0,
            wipes: 0,
            puppets: Puppeting::default(),
            following: Following::default(),
            // vats
            vats: vats::Share::new(vats::Mode::Off),
            harm_on: false,
            harm_ms: None,
            harms_sent: 0,
            smooth_on: false,
            smooth: smooth::Smooth::default(),
        }
    }

    /// The same writing nowhere but [`Fight::lines`] (tests).
    pub fn quiet(mut self) -> Fight {
        self.emit = |_| {};
        self
    }

    fn say(&mut self, line: String) {
        (self.emit)(&line);
        self.lines.push(line);
    }

    /// The cap this run was given.
    pub fn cap(&self) -> Cap {
        self.cap
    }

    /// The first frame: the hit wrap learns what this machine is.
    fn start(&mut self, e: &mut Engine) {
        if self.started {
            return;
        }
        self.started = true;
        if self.cap < Cap::Damage {
            return;
        }
        if self.cap >= Cap::Revive {
            fight::set_spare(e, true);
        }
        // vats: losses outside hits only when asked for.
        harm::set_on(e, self.harm_on);
        if fight::hits_wrapped(e) {
            fight::set_role(e, if self.guest { Role::Guest } else { Role::Host });
        } else {
            self.say(
                "co-op fights: this game has no Actor::HitMe to wrap; hits are not shared".into(),
            );
        }
    }

    /// Tells the hit wrap which references are players' avatars, when that changed. Where actors
    /// may fight every player, a new avatar stops being a ghost: on the host, and on a guest too,
    /// whose game runs the actors near its player after a hand-off (harness, 2026-11-14: the
    /// geckos a guest's game ran never went for the host's avatar there, a ghost, while their
    /// copies in the host's game went for the host and were stopped over and over).
    fn know_avatars(&mut self, e: &mut Engine, avatars: &[(PlayerId, u32)]) {
        let ids: BTreeSet<u32> = avatars.iter().map(|a| a.1).collect();
        if ids == self.avatars {
            return;
        }
        if self.cap >= Cap::Threat {
            for id in ids.difference(&self.avatars) {
                fight::set_ghost(e, *id, false);
            }
        }
        fight::set_avatars(e, ids.iter().copied());
        self.avatars = ids;
    }

    /// An actor of the host's game as the state the ledger carries. A target that is a player
    /// (the host's own, or a guest's avatar) is named by the player.
    fn state_of(
        s: &Snap,
        avatars: &[(PlayerId, u32)],
        me: PlayerId,
        attack: Option<Attack>,
    ) -> ActorState {
        let target = if s.target == actors::PLAYER_REF_ID {
            PLAYER_TARGET_BASE + u32::from(me.0)
        } else if let Some((p, _)) = avatars
            .iter()
            .find(|(_, a)| *a == s.target && s.target != 0)
        {
            PLAYER_TARGET_BASE + u32::from(p.0)
        } else {
            s.target
        };
        ActorState {
            cell: CellId(s.space),
            pos: s.pos.map(quant_pos),
            heading: quant_heading(s.heading),
            anim: s.moves,
            health: health16(s.health),
            target,
            flags: if s.in_combat { FLAG_COMBAT } else { 0 } | if s.dead() { FLAG_DEAD } else { 0 },
            attack_seq: attack.map_or(0, |a| a.seq),
            attack_group: attack.map_or(0, |a| a.group),
            // (stamped when sent)
            ms: 0,
        }
    }

    /// One frame of copy `id` of an actor another game runs, whose state there is `s`: named to
    /// the wraps with its owner's target, turned to that target (or started on a combat with it
    /// when its group does not know it yet), stopped when it fights while its owner's actor has
    /// not for [`CALM_AFTER_MS`], and given each new attack of the owner's. The callers name no
    /// copy in another space than the owner's actor, nor one left to a conversation (D6,
    /// condition 3): those are their own AI's.
    fn puppet(
        &mut self,
        e: &mut Engine,
        id: u32,
        s: &ActorState,
        me: PlayerId,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) {
        // (the callers name only a living copy in the owner's actor's space)
        let Some(here) = fight::snap_of(e, id) else {
            return;
        };
        // (a guest publishes no state of a dead actor, and a guest's copy of a dead one is killed
        // before it is named)
        let fights = s.flags & FLAG_COMBAT != 0;
        let lock = if fights {
            local_target(s.target, me, avatars)
        } else {
            0
        };
        let p = &mut self.puppets;
        p.copies.insert(id, lock);
        // The wraps know its owner's target before anything below asks the game for a change.
        puppet::set_copy(e, id, lock);
        if lock != 0 {
            p.calm_since.remove(&id);
            match puppet::aim(e, id, lock) {
                Aim::Turned => p.turns += 1,
                Aim::NotFighting | Aim::Unknown => {
                    let due = p
                        .started
                        .get(&id)
                        .map_or(true, |t| now_ms.saturating_sub(*t) >= AIM_START_EVERY_MS);
                    if due && fight::start_combat(e, id, lock) {
                        p.started.insert(id, now_ms);
                        p.starts += 1;
                    }
                }
                Aim::Already | Aim::Missing => {}
            }
        } else if here.in_combat && !fights {
            // It went for this game's own player: each game's actors notice only that game's
            // player (the others are avatars), so the owner's game is asked to have its actor
            // fight this player (harness, 2026-11-14: the geckos a guest's game ran stood while
            // their copies in the host's game went for the host). Unless it does, the copy stops.
            let due = p
                .asked
                .get(&id)
                .map_or(true, |t| now_ms.saturating_sub(*t) >= ENGAGE_EVERY_MS);
            if here.target == actors::PLAYER_REF_ID && due && self.cap >= Cap::Threat {
                p.asked.insert(id, now_ms);
                p.engage.push(id);
            }
            let since = *p.calm_since.entry(id).or_insert(now_ms);
            if now_ms.saturating_sub(since) >= CALM_AFTER_MS && puppet::stop(e, id) {
                p.calm_since.remove(&id);
                p.stops += 1;
            }
        } else {
            p.calm_since.remove(&id);
        }
        // A new attack of the owner's: replayed here. The first state seen of a copy only sets
        // the count (its last attack is over).
        let before = p.seen.insert(id, s.attack_seq);
        if before.is_some_and(|b| b != s.attack_seq) {
            if puppet::replay(e, id, s.attack_group) {
                p.replays += 1;
            } else {
                p.replays_refused += 1;
            }
        }
    }

    /// Starts a frame of the copies: none named yet.
    fn puppets_begin(&mut self) {
        self.puppets.copies.clear();
    }

    /// Takes the latest state `s` of actor `id`, which `owner`'s game runs, into its track; called
    /// each frame for every actor another game runs, so that a state is timed when it arrives
    /// (one first taken long after it came would be timed wrong: a copy left to a conversation
    /// was led on as if its actor had only just stopped).
    fn feed(&mut self, id: u32, owner: Option<PlayerId>, s: &ActorState, now_ms: u64) {
        self.following.seen.insert(id);
        let (o, track) = self.following.tracks.entry(id).or_default();
        if *o != owner {
            // Another game's clock.
            *o = owner;
            track.clear();
        }
        track.push(now_ms, s);
    }

    /// Where copy `id` is to be shown now; `None` before its first state.
    fn place(&mut self, id: u32, now_ms: u64) -> Option<Place> {
        self.following.tracks.get_mut(&id)?.1.at(now_ms)
    }

    /// Has copy `id` walk or run as `moves` say (0: stand), through the game's forced movement,
    /// asked of the game only when it changes.
    fn gait(&mut self, e: &mut Engine, id: u32, moves: u16) {
        let f = &mut self.following;
        f.kept.insert(id);
        if f.forced.get(&id) != Some(&moves) && puppet::force_moves(e, id, moves) {
            f.forced.insert(id, moves);
        }
    }

    /// Puts copy `id` on its owner's path: at `place`, with the owner's gait. One further than
    /// [`CATCH_DIST`] from it catches up at [`CATCH_SPEED`]; one further than [`SNAP_DIST`] is
    /// put there at once (and counted).
    fn go_along(&mut self, e: &mut Engine, id: u32, place: &Place) -> bool {
        let Some(here) = fight::place_of(e, id) else {
            return false;
        };
        let off = dist(here, place.pos);
        // Off the path (it was its own AI's a moment ago): it catches up quickly, never jumps,
        // unless it is so far that only a jump makes sense.
        let to = if off > SNAP_DIST {
            self.following.snaps += 1;
            if self.following.snaps <= 60 {
                self.say(format!(
                    "co-op fights: actor {id:08x} was {off:.0} units from its owner's path: put on it at ({:.0}, {:.0}, {:.0})",
                    place.pos[0], place.pos[1], place.pos[2]
                ));
            }
            place.pos
        } else if off > CATCH_DIST {
            let step = CATCH_SPEED * self.following.dt_ms as f32 / 1000.0;
            let f = (step / off).min(1.0);
            [0, 1, 2].map(|i| here[i] + (place.pos[i] - here[i]) * f)
        } else {
            place.pos
        };
        self.following.gliding.remove(&id);
        if fight::lead(e, id, to, place.heading).is_none() {
            return false;
        }
        self.gait(e, id, place.moves);
        self.following.leads += 1;
        true
    }

    /// One frame of a copy walking back to `place`, where its owner's actor stands: a step of
    /// [`GLIDE_SPEED`] towards it, facing the way, walking, until it is there (the caller puts
    /// one further than [`SNAP_DIST`] there at once). Whether it is still on its way.
    fn glide(&mut self, e: &mut Engine, id: u32, place: &Place) -> bool {
        let Some(here) = fight::place_of(e, id) else {
            self.following.gliding.remove(&id);
            return false;
        };
        let off = dist(here, place.pos);
        if off < 1.0 {
            self.following.gliding.remove(&id);
            return false;
        }
        let step = GLIDE_SPEED * self.following.dt_ms as f32 / 1000.0;
        let f = (step / off).min(1.0);
        let to = [0, 1, 2].map(|i| here[i] + (place.pos[i] - here[i]) * f);
        // Heading 0 faces +Y, turning clockwise.
        let heading = (place.pos[0] - here[0]).atan2(place.pos[1] - here[1]);
        fight::lead(e, id, to, heading);
        self.gait(e, id, WALK_FORWARD);
        true
    }

    /// One frame of copy `id`, to be shown at `place`, as `how` says. While its owner's actor
    /// fights, moves or crouches, the copy goes along the owner's path with its gait; and,
    /// with [`Fight::lead_on`], it stands where that actor stands once it has stopped
    /// (docs/coop/DECISIONS.md D7: PC run 31, Sunny stood by her campfire in the host's game and
    /// further off in the guest's, where her copy, left to its own AI, had picked its own spot).
    /// Without it (and for a copy whose package is aimed at the player) the copy is its own AI's
    /// once that actor has stood [`SETTLE_MS`] (its package is the owner's, so it sits and idles
    /// as its game has it), and one further than its room from that actor for [`FOLLOW_MS`]
    /// walks back to it. Whether it was put on the owner's path this frame.
    fn follow_copy(
        &mut self,
        e: &mut Engine,
        id: u32,
        place: &Place,
        how: Follow,
        now_ms: u64,
    ) -> bool {
        // A copy in a cell this game let go is neither drawn nor moved by `SetPos` (PC run 31: the
        // host's Sunny, left at the wells as the host stood at the saloon, was "put on her path"
        // every frame and never moved). It is brought to its place when that is within this
        // game's loaded cells, and left alone otherwise.
        if !live::is_loaded(e, id) {
            if live::bring_into_view(e, id, place.pos) {
                self.following.brought += 1;
            }
            return false;
        }
        // (between two states, and past a last state that still moves, `still_ms` is 0)
        // A crouched one is kept crouched where it is (PC run 31: Sunny crouched at the well in
        // the guest's game stood up in the host's once she had stood still a moment).
        let (keep, room) = match how {
            Follow::Fight => (true, FOLLOW_DISTANCE),
            Follow::Path => (
                self.lead_on || place.still_ms < SETTLE_MS || place.sneaking(),
                FOLLOW_DISTANCE,
            ),
            Follow::WalkBack => (false, AIMED_DISTANCE),
        };
        if keep {
            self.far_since.remove(&id);
            return self.go_along(e, id, place);
        }
        let Some(here) = fight::place_of(e, id) else {
            return false;
        };
        let off = dist(here, place.pos);
        if off > SNAP_DIST {
            self.far_since.remove(&id);
            return self.go_along(e, id, place);
        }
        if self.following.gliding.contains(&id) {
            if self.glide(e, id, place) {
                return false;
            }
            self.far_since.remove(&id);
        }
        if off <= room {
            self.far_since.remove(&id);
            return false;
        }
        let since = *self.far_since.entry(id).or_insert(now_ms);
        if now_ms.saturating_sub(since) < FOLLOW_MS {
            return false;
        }
        self.far_since.remove(&id);
        self.following.gliding.insert(id);
        self.nudges += 1;
        if self.nudges <= 60 {
            self.say(format!(
                "co-op fights: actor {id:08x} was {off:.0} units from where its owner's game has it: walks back"
            ));
        }
        self.glide(e, id, place);
        false
    }

    /// Whether copy `id` may be brought through a door now: not within [`BRING_EVERY_MS`] of the
    /// last time, nor more than [`BRING_AT_MOST`] times (one that leaves again each time is left
    /// to its game). Counts it when it may.
    fn bring_due(&mut self, id: u32, now_ms: u64) -> bool {
        let due = self
            .brought
            .get(&id)
            .map_or(true, |t| now_ms.saturating_sub(*t) >= BRING_EVERY_MS);
        let times = self.brought_times.get(&id).copied().unwrap_or(0);
        if !self.bring_on || !due || times >= BRING_AT_MOST {
            return false;
        }
        self.brought.insert(id, now_ms);
        self.brought_times.insert(id, times + 1);
        if times + 1 == BRING_AT_MOST {
            self.say(format!(
                "co-op fights: actor {id:08x} has been brought {BRING_AT_MOST} times and leaves again each time: left to this game from now"
            ));
        }
        true
    }

    /// Starts a frame of the copies' places.
    fn following_begin(&mut self, now_ms: u64) {
        let f = &mut self.following;
        f.kept.clear();
        f.seen.clear();
        f.dt_ms = f.last_ms.map_or(0, |t| now_ms.saturating_sub(t).min(250));
        f.last_ms = Some(now_ms);
    }

    /// Ends a frame of the copies' places: a copy not forced this frame (left to its own AI, no
    /// longer a copy, or gone) has its own movement back.
    fn following_end(&mut self, e: &mut Engine) {
        let f = &mut self.following;
        let free: Vec<u32> = f
            .forced
            .keys()
            .filter(|id| !f.kept.contains(id))
            .copied()
            .collect();
        for id in free {
            puppet::free_moves(e, id);
            self.following.forced.remove(&id);
        }
        let f = &mut self.following;
        let kept = &f.kept;
        f.gliding.retain(|id| kept.contains(id));
        let seen = &f.seen;
        f.tracks.retain(|id, _| seen.contains(id));
    }

    /// Ends a frame of the copies: the wraps are told which they are (one no longer a copy is its
    /// game's own again), and what is kept of the others is let go.
    fn puppets_end(&mut self, e: &mut Engine) {
        let p = &mut self.puppets;
        puppet::set_copies(e, p.copies.iter().map(|(a, b)| (*a, *b)));
        let copies = &p.copies;
        p.seen.retain(|id, _| copies.contains_key(id));
        p.started.retain(|id, _| copies.contains_key(id));
        p.calm_since.retain(|id, _| copies.contains_key(id));
        p.asked.retain(|id, _| copies.contains_key(id));
    }

    /// Has this game's actor `actor` fight `player` for another game's copy that went for that
    /// player, unless it fights already (`player` is named in this game: its own player or an
    /// avatar).
    fn take_up(&mut self, e: &mut Engine, actor: u32, target: u32) {
        // (a player without an avatar here is 0: no form, no fight)
        let idle = fight::snap_of(e, actor).is_some_and(|s| !s.in_combat && !s.dead());
        if idle && fight::start_combat(e, actor, target) {
            self.puppets.taken_up += 1;
        }
    }

    // vats
    /// What the wraps kept from the game for the session: the weapon hits, and every
    /// [`HARM_EVERY_MS`] the health lost outside `Actor::HitMe`. The second kind is carried as
    /// damage over time: what made it is not read from the game, and coop-core neither scales
    /// that kind for the party nor ties it to a limb.
    fn blows(&mut self, e: &mut Engine, now_ms: u64) -> Vec<(fight::Hit, DamageKind)> {
        let mut out: Vec<(fight::Hit, DamageKind)> = fight::take_hits(e)
            .into_iter()
            .map(|h| (h, DamageKind::Bullet))
            .collect();
        if self
            .harm_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < HARM_EVERY_MS)
        {
            return out;
        }
        self.harm_ms = Some(now_ms);
        for h in harm::take(e) {
            self.harms_sent += 1;
            out.push((
                fight::Hit {
                    target: h.target,
                    aggressor: h.source,
                    amount: h.amount,
                    limb: -1,
                },
                DamageKind::Poison,
            ));
        }
        out
    }

    /// The host's frame, between two of the game's frames and before coop-core's tick: guests'
    /// hits the ledger accepted are applied, hits on avatars become the guests' damage, the
    /// actors around the host's player go into the ledger and their states out, and the threat
    /// director has its say. `avatars` are the other players' avatars in this game (never
    /// tracked: they are players). Returns the sends.
    pub fn host_step(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mut out = Vec::new();
        if self.guest {
            return out;
        }
        self.start(e);
        self.know_avatars(e, avatars);
        // vats: this game's V.A.T.S. does not stop or slow a world other players are in.
        if let Some(active) = self.vats.step(e, !sync.players().is_empty()) {
            out.extend(sync.on_local(Msg::Combat(CombatMsg::VatsMark {
                player: PlayerId::HOST,
                active,
            })));
        }
        if self.cap >= Cap::Damage {
            self.host_damage(e, sync, avatars, now_ms, &mut out);
        }
        self.host_follow(e, sync, avatars, now_ms);
        // A copy here of an actor a guest runs went for the host: that guest's game is asked to
        // have its actor fight the host. A guest's copy of one of the host's went for that
        // guest: the host's game has it fight that guest's avatar.
        for id in std::mem::take(&mut self.puppets.engage) {
            let owner = sync.live.actors.own.get(FormId(id)).and_then(|r| r.owner);
            if let Some(owner) = owner.filter(|o| *o != PlayerId::HOST) {
                out.push((
                    owner,
                    Msg::Actor(ActorMsg::Engage {
                        actor: FormId(id),
                        player: PlayerId::HOST,
                    }),
                ));
                self.puppets.asks += 1;
            }
        }
        for (actor, player) in std::mem::take(&mut sync.live.actors.engages) {
            // (the host passes one for an actor a guest runs to that guest)
            if self.cap >= Cap::Threat {
                let target = avatars
                    .iter()
                    .find(|(p, _)| *p == player)
                    .map_or(0, |a| a.1);
                self.take_up(e, actor.0, target);
            }
        }
        if self
            .published_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < PUBLISH_EVERY_MS)
        {
            return out;
        }
        self.published_ms = Some(now_ms);
        let mut seen = BTreeSet::new();
        // The actors near the host's player, and with hand-off those near a guest's avatar too:
        // the host's game may run those at a low level or not at all, and the guest's game, near
        // them, runs them (PC run 21: with the host's player at the saloon, Sunny and the geckos
        // outside stood still on the host and jumped about on the guest).
        // Where the guests are, as they say (their avatars in this game may not be loaded).
        let places: Vec<(PlayerId, u32, [f32; 3])> = if self.handoff {
            sync.players()
                .into_iter()
                .filter_map(|p| Some((p, sync.live.avatars.state(p).copied()?)))
                // A guest whose own opening runs is not in the shared world: its game runs its
                // own Doc Mitchell (PC run 26: the host's Doc Mitchell was handed to the guest,
                // in the same room of its own game, and stood still with his AI off; the host's
                // player never got up, and the guest's opening skipped lines).
                .filter(|(_, st)| st.flags & AVATAR_OPENING == 0)
                .map(|(p, st)| (p, st.cell.0, st.pos.map(dequant_pos)))
                .collect()
        } else {
            Vec::new()
        };
        // An actor in a part of an open world this game has not loaded has no space here (its
        // cell is let go; the game runs it at the lowest level): one standing within reach of a
        // guest out in a world space is in that guest's world space (harness, 2026-11-13: Sunny
        // at the first well, far from the host's player, had none, was never handed to the
        // guest near her, and the host's game ran her sneak and her talk at once, setting
        // stages 35, 40 and 45 together, as in PC run 25).
        let worlds: Vec<(u32, [f32; 3])> = places
            .iter()
            .filter(|(_, sp, _)| {
                coop_engine::forms::form(e, *sp).is_some_and(|f| {
                    e.mem
                        .u8(f.wrapping_add(coop_engine::addr::FORM_TYPE_OFFSET))
                        != coop_engine::addr::FORM_TYPE_CELL
                })
            })
            .map(|(_, sp, p)| (*sp, *p))
            .collect();
        let space_here = |e: &mut Engine, a: u32| -> u32 {
            match fight::space_of(e, a) {
                0 => {
                    let pos = fight::place_of_ptr(e, a);
                    worlds
                        .iter()
                        .find(|(_, p)| {
                            (0..3).map(|i| (p[i] - pos[i]).powi(2)).sum::<f32>().sqrt()
                                <= HANDOFF_RADIUS
                        })
                        .map_or(0, |(sp, _)| *sp)
                }
                sp => sp,
            }
        };
        let mut candidates = fight::near(e);
        // In the list now, and, for one a guest's game runs that the list left out at some time,
        // without a break for CLOSE_SETTLE_MS since (PC run 25: Sunny, at the edge of the host's
        // list between the wells, went from one game to the other every second or two, and
        // stood ~20 s).
        let id_of =
            |e: &mut Engine, a: u32| e.mem.u32(a.wrapping_add(coop_engine::addr::FORM_ID_OFFSET));
        let near_ids: BTreeSet<u32> = candidates.iter().map(|a| id_of(e, *a)).collect();
        for id in &self.theirs {
            if !near_ids.contains(id) {
                self.left_out_at.insert(*id, now_ms);
            }
        }
        self.left_out_at
            .retain(|a, t| self.theirs.contains(a) && now_ms.saturating_sub(*t) < CLOSE_SETTLE_MS);
        let close_to_host: BTreeSet<u32> = candidates
            .iter()
            .copied()
            .filter(|a| {
                self.left_out_at
                    .get(&id_of(e, *a))
                    .map_or(true, |t| now_ms.saturating_sub(*t) >= CLOSE_SETTLE_MS)
            })
            .collect();
        if !places.is_empty() {
            for a in fight::loaded(e) {
                if candidates.contains(&a) {
                    continue;
                }
                let (space, pos) = (space_here(e, a), fight::place_of_ptr(e, a));
                let close = places.iter().any(|(_, sp, p)| {
                    *sp == space
                        && (0..3).map(|i| (p[i] - pos[i]).powi(2)).sum::<f32>().sqrt()
                            <= HANDOFF_RADIUS
                });
                if close {
                    candidates.push(a);
                }
            }
        }
        for a in candidates {
            let mut s = fight::snap(e, a);
            if s.space == 0 {
                s.space = space_here(e, a);
            }
            // An actor a guest's game runs is where that game has it: this game's copy may be
            // elsewhere for a while (another space, a cell let go), and the nearest player is
            // the one near the guest's actor (PC run 31: Victor's copy here was elsewhere, and
            // the host's game took him back from the guest he was walking past).
            if self.theirs.contains(&s.id) {
                if let Some(r) = sync.live.actors.own.get(FormId(s.id)) {
                    s.space = r.state.cell.0;
                    s.pos = r.state.pos.map(dequant_pos);
                }
            }
            if !actors::shareable(s.id)
                || s.space == 0
                || avatars.iter().any(|(_, av)| *av == s.id)
                || actors::actor(e, s.id) != Some(a)
            {
                continue;
            }
            let id = FormId(s.id);
            let state = Self::state_of(&s, avatars, PlayerId::HOST, puppet::attack_of(e, s.id));
            if let std::collections::btree_map::Entry::Vacant(slot) = self.tracked.entry(s.id) {
                sync.live.actors.register(id, state);
                slot.insert(Tracked::default());
            }
            // Without hand-off every actor is the host's game's to run. With it, the nearest
            // player's game runs it, except an actor keeping company with the host's player (a
            // companion, someone leading it), whose package is about that player.
            let keeps_company = coop_engine::commands::current_package(e, s.id)
                .is_some_and(|p| p != 0 && coop_engine::commands::keeps_company(e, p));
            // The host's game runs at full detail only the actors close to its player: one it
            // does not, near a guest, is that guest's however near the host's player may be
            // (PC run 24: Sunny went back to the host's game whenever its player was nearer by
            // the numbers, walked where she runs and stood still at the sneak).
            let nearest_guest = || {
                places
                    .iter()
                    .filter(|(_, sp, _)| *sp == s.space)
                    .map(|(p, _, pos)| {
                        let d = (0..3)
                            .map(|i| (pos[i] - s.pos[i]).powi(2))
                            .sum::<f32>()
                            .sqrt();
                        (*p, d)
                    })
                    .filter(|(_, d)| *d <= HANDOFF_RADIUS)
                    .min_by(|a, b| a.1.total_cmp(&b.1))
                    .map(|(p, _)| p)
            };
            let was_engaged = self.engaged.get(&s.id).copied();
            let engaged = was_engaged.filter(|p| {
                if *p == PlayerId::HOST {
                    close_to_host.contains(&a)
                } else {
                    places.iter().any(|(q, sp, pos)| {
                        q == p
                            && *sp == s.space
                            && (0..3)
                                .map(|i| (pos[i] - s.pos[i]).powi(2))
                                .sum::<f32>()
                                .sqrt()
                                <= HANDOFF_RADIUS
                    })
                }
            });
            // It ends when that player is no longer near it, or it is dead.
            if was_engaged.is_some() && (engaged.is_none() || s.dead()) {
                self.engaged.remove(&s.id);
            }
            let engaged = engaged.filter(|_| !s.dead());
            // companions: a companion a guest hired is that guest's game's to run (its follow
            // package there is about that guest's own player).
            let kept_by = coop_engine::companion::kept_by(e, s.id)
                .filter(|p| *p != 0)
                .map(PlayerId);
            let pin = if let (true, Some(g)) = (self.handoff, kept_by) {
                Some(g)
            } else if !self.handoff || keeps_company {
                Some(PlayerId::HOST)
            } else if engaged.is_some() {
                engaged
            } else if close_to_host.contains(&a) {
                // The nearest player's game, as coop-core's ownership has it, when a guest in
                // the shared world is near it; the host's otherwise (a guest in its own opening
                // is not in the shared world, whatever its place says).
                match nearest_guest() {
                    Some(_) => None,
                    None => Some(PlayerId::HOST),
                }
            } else {
                nearest_guest()
            };
            sync.live.actors.own.set_follower(id, pin);
            seen.insert(s.id);
            // The ledger's health is the game's, and the game says who cannot die.
            sync.live.actors.set_protected(id, s.essential);
            if let Some(rec) = sync.live.actors.own.get_mut(id) {
                rec.state.health = state.health;
            }
            if s.dead() && !sync.live.actors.is_dead(id) {
                self.say(format!(
                    "co-op fights: actor {:08x} is dead in the host's game (life state {}, health {:.0}) at ({:.0}, {:.0}, {:.0}) in {:08x}",
                    s.id, s.life, s.health, s.pos[0], s.pos[1], s.pos[2], s.space
                ));
                sync.live.combat.threat.forget(id);
                for o in sync.live.actors.kill_story(id, None) {
                    out.extend(sync.live.route_extern(o.aud, Msg::Actor(o.msg)));
                }
            }
            // The package it runs, sent when it changes: a guest's copy is given the same. A
            // dead one runs none of the host's.
            // Nor one a guest's game runs: its copy there picks its own (this game's copy is kept
            // still and its package here says nothing).
            let package = if s.dead() || self.theirs.contains(&s.id) {
                Some(0)
            } else {
                coop_engine::commands::current_package(e, s.id)
            };
            if let Some(package) = package {
                if self.packages.insert(s.id, package) != Some(package) {
                    sync.write(FormId(s.id), crate::package_field(package));
                }
            }
            let t = self.tracked.entry(s.id).or_default();
            t.snap = Some(s);
            let Some(rec) = sync.live.actors.own.get(id).copied() else {
                continue;
            };
            if rec.owner != Some(PlayerId::HOST) {
                // A guest's game runs it: the host's copy is put where that game has it (each
                // frame, in `host_follow`), and its state is not the host's to send.
                t.last = None;
                continue;
            }
            if t.last != Some(state) {
                // States go to the players in the actor's space. One that has just left a space
                // (through a door) is told once more to the players in the space it left: theirs
                // would hold its last place there for ever, and their copies were kept at the
                // door (PC run 13: Chet fled outdoors on the host and stood at the door on the
                // guest; Joe Cobb likewise).
                let left = t.last.map(|l| l.cell).filter(|c| *c != state.cell);
                t.seq += 1;
                t.last = Some(state);
                self.published += 1;
                // (sent with the time it was taken: copies play the states back by it)
                let msg = ActorMsg::State {
                    actor: id,
                    epoch: rec.epoch,
                    seq: Seq(t.seq),
                    state: ActorState {
                        ms: now_ms as u32,
                        ..state
                    },
                };
                out.extend(sync.live.on_msg(
                    PlayerId::HOST,
                    Msg::Actor(msg.clone()),
                    now_ms,
                    &mut sync.auth,
                ));
                if let Some(old) = left {
                    self.left_told += 1;
                    for (to, m) in sync.live.route_extern(Audience::Cell(old), Msg::Actor(msg)) {
                        // A player who sees both spaces has it already.
                        if !out.iter().any(|(p, o)| *p == to && *o == m) {
                            out.push((to, m));
                        }
                    }
                }
            }
        }
        // An actor the host's game no longer runs near its player: out of the ledger, and the
        // guests told that nobody runs it (their own games take it back).
        let gone: Vec<u32> = self
            .tracked
            .keys()
            .copied()
            .filter(|id| !seen.contains(id))
            .collect();
        for id in gone {
            // One a guest's game runs stays tracked while this game still has it: the host's
            // own lists may leave it out from one moment to the next (PC run 22: Sunny was handed
            // over and taken back every second).
            if self.theirs.contains(&id) && actors::dead(e, id) == Some(false) {
                // Out of the host's lists altogether: the host's game is not running it, so it
                // stays with the guest's that is.
                let owner = sync.live.actors.own.get(FormId(id)).and_then(|r| r.owner);
                sync.live.actors.own.set_follower(FormId(id), owner);
                continue;
            }
            self.tracked.remove(&id);
            // Its package is still the host's to say: the host's game runs the actor on, out of
            // its player's sight, and a guest's copy that picked its own would part from it (PC
            // run 14: Sunny went outdoors on the host while its player stayed in; the guest's
            // copy walked out after her, was given back to its own game, and came back in).
            if self.packages.contains_key(&id) {
                self.watched.insert(id);
            }
            sync.live.combat.threat.forget(FormId(id));
            let rec = sync.live.actors.own.get(FormId(id)).copied();
            sync.live.actors.own.unregister(FormId(id), &mut sync.auth);
            if let Some(rec) = rec {
                out.extend(sync.live.route_extern(
                    Audience::Cell(rec.state.cell),
                    Msg::Actor(ActorMsg::Assign {
                        actor: FormId(id),
                        owner: None,
                        epoch: rec.epoch,
                        seed: Some(rec.state),
                    }),
                ));
            }
        }
        // The actors the host ran near its player and still runs out of sight: the package of
        // each is read on, and said when it changes. One that is gone, dead or switched off is
        // this game's no longer to say.
        self.watched.retain(|id| !seen.contains(id));
        let watched: Vec<u32> = self.watched.iter().copied().collect();
        for id in watched {
            let runs = actors::actor(e, id)
                .filter(|a| e.mem.u32(a + coop_engine::addr::ACTOR_PROCESS_OFFSET) != 0)
                .filter(|_| actors::dead(e, id) == Some(false))
                .filter(|_| actors::disabled(e, id) == Some(false))
                .and_then(|_| coop_engine::commands::current_package(e, id));
            match runs {
                Some(package) => {
                    if self.packages.insert(id, package) != Some(package) {
                        sync.write(FormId(id), crate::package_field(package));
                    }
                }
                None => {
                    self.watched.remove(&id);
                    if self.packages.remove(&id).is_some_and(|p| p != 0) {
                        sync.write(FormId(id), crate::package_field(0));
                    }
                }
            }
        }
        if self.cap >= Cap::Threat {
            self.host_threat(e, sync, avatars, now_ms);
        }
        out
    }

    /// Host: actor `npc` spoke with player `p` (a conversation began). While `p` is near it, `p`'s
    /// game runs it: its packages aimed at "the player" are about that player (PC run 27: the
    /// guest started Sunny's tutorial, the host's player stood nearer at the saloon, the host's
    /// game took her back, and she walked to the host's player and waited for it to talk).
    pub fn engage(&mut self, npc: u32, p: PlayerId) {
        if self.engaged.insert(npc, p) != Some(p) {
            self.say(format!(
                "co-op fights: actor {npc:08x} spoke with player {}: that player's game runs it while near",
                p.0
            ));
        }
    }

    /// Host: a copy of actor `npc` came to greet player `p` in that player's game (a force greet
    /// the copy's game refused, because another game runs the actor). `p`'s game runs it from
    /// now, as after a conversation ([`Self::engage`]), so that the actor greets whichever
    /// player comes to it; not while it is `talking` with someone, and not within
    /// [`GREET_HOLD_MS`] of the last such change (two players both in front of it).
    pub fn greet_asked(&mut self, npc: u32, p: PlayerId, talking: bool, now_ms: u64) {
        if talking || self.engaged.get(&npc) == Some(&p) {
            return;
        }
        if self
            .greeted_at
            .get(&npc)
            .is_some_and(|t| now_ms.saturating_sub(*t) < GREET_HOLD_MS)
        {
            return;
        }
        self.greeted_at.insert(npc, now_ms);
        self.engaged.insert(npc, p);
        self.greets_given += 1;
        self.say(format!(
            "co-op fights: actor {npc:08x} came to greet player {}: that player's game runs it while near",
            p.0
        ));
    }

    /// The host's part of the damage: what the ledger accepted of the guests' hits goes into
    /// the game, and the game's hits on avatars go to their players through the vitals.
    /// Host: the actors a guest's game runs are put, here, where that game has them, every frame,
    /// with the ledger's health; the hit wrap routes this player's hits on them through the
    /// ledger, and keeps theirs on this player (the guest's game says those).
    fn host_follow(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) {
        self.puppets_begin();
        self.following_begin(now_ms);
        let fed: Vec<(u32, Option<PlayerId>, ActorState)> = sync
            .live
            .actors
            .own
            .iter()
            .filter(|(_, r)| r.owner.is_some() && r.owner != Some(PlayerId::HOST))
            .map(|(f, r)| (f.0, r.owner, r.state))
            .collect();
        let guests_run: Vec<u32> = fed.iter().map(|f| f.0).collect();
        for (id, owner, s) in fed {
            self.feed(id, owner, &s, now_ms);
        }
        // Where this game's player is: a copy of an actor a guest runs in another space here is
        // brought to it when the guest's actor is in this player's space.
        let player = e.mem.u32(coop_engine::addr::PLAYER_SINGLETON);
        let mine =
            (player != 0).then(|| (fight::space_of(e, player), fight::place_of_ptr(e, player)));
        let mut theirs = BTreeSet::new();
        // Every actor a guest runs, near this player or not (PC run 31: Victor walked past the
        // guest; the host's copy was elsewhere, never seen here, and never brought).
        for id in guests_run {
            let Some(rec) = sync.live.actors.own.get(FormId(id)).copied() else {
                continue;
            };
            if rec.owner.is_none() || rec.owner == Some(PlayerId::HOST) {
                continue;
            }
            theirs.insert(id);
            let s = rec.state;
            // Led here to where the guest's game has it, its AI on, so that it walks animated
            // and keeps its feet on the ground (PC run 29: kept still with its AI off, Sunny's
            // copy sank into the floor and Joe Cobb's slid). Its own doing here is the gate's to
            // skip (`gate::set_puppets`): the guest's game does it and proposes what it changes.
            // In another space than the guest's (it went through a door there) it is not led:
            // its own AI here takes it through the door after it.
            // In a conversation here (a scene this game plays, PC run 30: Joe Cobb), it is not led:
            // the scene plays out where this game has it.
            let here = actors::actor(e, id).map(|a| fight::space_of(e, a));
            let elsewhere = here.is_some() && here != Some(s.cell.0);
            if let Some((_, at)) = mine.filter(|(space, _)| elsewhere && *space == s.cell.0) {
                let to = s.pos.map(dequant_pos);
                if dist(at, to) <= live::LOADED_REACH
                    && s.flags & FLAG_DEAD == 0
                    && actors::dead(e, id) == Some(false)
                    && self.bring_due(id, now_ms)
                    && fight::bring(e, id, to)
                {
                    self.brings += 1;
                    if self.brings <= 60 {
                        self.say(format!(
                            "co-op fights: actor {id:08x} was in {:08x} here and is in {:08x} in its guest's game: brought to ({:.0}, {:.0}, {:.0})",
                            here.unwrap_or(0), s.cell.0, to[0], to[1], to[2]
                        ));
                    }
                }
                continue;
            }
            if fight::place_of(e, id).is_some()
                && here == Some(s.cell.0)
                && !self.left_to_talk(e, id, now_ms)
            {
                // Along the guest's actor's path, a little behind, with its gait (D6).
                let place = self.place(id, now_ms);
                let how = if s.flags & FLAG_COMBAT != 0 {
                    Follow::Fight
                } else {
                    Follow::Path
                };
                if let Some(place) = place {
                    if self.follow_copy(e, id, &place, how, now_ms) {
                        self.guest_run_moves += 1;
                    }
                }
                // It fights whom the guest's actor fights, and attacks when that one does.
                if self.cap >= Cap::Damage {
                    self.puppet(e, id, &s, PlayerId::HOST, avatars, now_ms);
                }
            }
        }
        self.puppets_end(e);
        self.following_end(e);
        if theirs != self.theirs {
            let gained: Vec<u32> = theirs.difference(&self.theirs).copied().collect();
            let lost: Vec<u32> = self.theirs.difference(&theirs).copied().collect();
            for id in gained {
                // An earlier build kept such a copy still with its AI off.
                fight::release(e, id);
                self.say(format!(
                    "co-op fights: actor {id:08x} is nearer a guest: that guest's game runs it now"
                ));
            }
            for id in lost {
                // This game runs it again, from where the guest's game left it (its AI was on
                // throughout).
                self.say(format!(
                    "co-op fights: actor {id:08x} is the host's game's to run again"
                ));
            }
            self.theirs = theirs;
        }
        coop_engine::gate::set_puppets(e, self.theirs.iter().copied());
        if self.cap >= Cap::Damage {
            fight::set_ledgered(e, self.theirs.iter().copied());
        }
    }

    fn host_damage(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
        out: &mut Vec<(PlayerId, Msg)>,
    ) {
        let avatar_of = |p: PlayerId| avatars.iter().find(|(q, _)| *q == p).map_or(0, |a| a.1);
        for at in std::mem::take(&mut sync.live.combat.attributions) {
            let by = avatar_of(at.by);
            if at.amount > 0 && fight::hurt_actor(e, at.actor.0, f32::from(at.amount), by).is_some()
            {
                self.hits_applied += 1;
                sync.live
                    .combat
                    .threat
                    .damaged(at.actor, at.by, f32::from(at.amount));
            }
            if at.killed && fight::kill_by(e, at.actor.0, by) {
                self.kills += 1;
            }
        }
        // The host knows each guest's health from the guest's own avatar stream.
        for p in sync.players() {
            if let Some(s) = sync.live.avatars.state(p).copied() {
                let vitals = &mut sync.live.combat.vitals;
                vitals.track(p, s.health.max(1), s.health.max(1));
                vitals.report(
                    p,
                    now_ms,
                    s.health,
                    s.flags & AVATAR_DOWNED != 0,
                    s.flags & AVATAR_DEAD != 0,
                );
            }
        }
        // vats: also the health lost outside `Actor::HitMe`.
        for (h, kind) in self.blows(e, now_ms) {
            // This player's hit on an actor a guest's game runs: into the ledger, which tells
            // that game.
            if self.theirs.contains(&h.target) {
                let amount = damage16(h.amount);
                if amount > 0 {
                    self.next_hit += 1;
                    let msg = ActorMsg::Hit {
                        actor: FormId(h.target),
                        hit: self.next_hit,
                        amount,
                    };
                    out.extend(sync.live.on_msg(
                        PlayerId::HOST,
                        Msg::Actor(msg),
                        now_ms,
                        &mut sync.auth,
                    ));
                    self.hosts_hits_forwarded += 1;
                }
                continue;
            }
            let Some((victim, _)) = avatars.iter().find(|(_, a)| *a == h.target) else {
                continue;
            };
            let amount = damage16(h.amount);
            if amount == 0 {
                continue;
            }
            // An actor a guest's game runs: what it does to a guest's player is done in that
            // player's own game (its hit there is that game's), never sent from here as well. Its
            // copy here shows the guest's actor's attacks (D6), whose blows on an avatar would
            // otherwise land twice.
            if self.theirs.contains(&h.aggressor) {
                self.theirs_hits_left += 1;
                continue;
            }
            // The kind and the limb are not read from the game yet: a direct hit, no limb.
            let msgs = if h.aggressor == actors::PLAYER_REF_ID {
                sync.live.combat.vitals.player_vs_player(
                    now_ms,
                    PlayerId::HOST,
                    *victim,
                    amount,
                    kind,
                    LIMB_COUNT,
                )
            } else {
                let scaling = sync.live.rules.scaling(now_ms);
                sync.live.combat.vitals.hurt(
                    now_ms,
                    *victim,
                    FormId(h.aggressor),
                    amount,
                    kind,
                    LIMB_COUNT,
                    &scaling,
                )
            };
            self.hurts_sent += msgs.len() as u32;
            out.extend(sync.live.route_combat(msgs));
        }
    }

    /// The threat director's turn: for each actor of the host's game that is fighting a player,
    /// which player it attends to; the game is asked to fight that one when it is another.
    fn host_threat(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) {
        if self
            .threat_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < THREAT_EVERY_MS)
        {
            return;
        }
        self.threat_ms = Some(now_ms);
        let Some(me) = live::local(e) else {
            return;
        };
        let focus = sync.live.rules.scaling(now_ms).focus;
        let fighting: Vec<Snap> = self
            .tracked
            .values()
            .filter_map(|t| t.snap)
            .filter(|s| s.in_combat && !s.dead())
            // (a guest's game chooses for the actors it runs: their copies here show it)
            .filter(|s| !self.theirs.contains(&s.id))
            .collect();
        for s in fighting {
            // Only an actor whose own choice is a player is redirected.
            let at_player = s.target == actors::PLAYER_REF_ID
                || avatars.iter().any(|(_, a)| *a == s.target && s.target != 0);
            if !at_player {
                continue;
            }
            let mut inputs = vec![ThreatInput {
                player: PlayerId::HOST,
                dist: dist(me.pos, s.pos),
                seen: true,
                downed: self.down,
                dead: false,
                same_cell: me.cell == s.space,
            }];
            for (p, _) in avatars {
                let Some(a) = sync.live.avatars.state(*p) else {
                    continue;
                };
                inputs.push(ThreatInput {
                    player: *p,
                    dist: dist(a.pos.map(dequant_pos), s.pos),
                    seen: true,
                    downed: a.flags & AVATAR_DOWNED != 0,
                    dead: a.flags & AVATAR_DEAD != 0,
                    same_cell: a.cell.0 == s.space,
                });
            }
            let Some(choice) = sync
                .live
                .combat
                .threat
                .choose(FormId(s.id), now_ms, focus, &inputs)
            else {
                continue;
            };
            let want = if choice.target.is_host() {
                actors::PLAYER_REF_ID
            } else {
                match avatars.iter().find(|(p, _)| *p == choice.target) {
                    Some((_, a)) => *a,
                    None => continue,
                }
            };
            if want != s.target && fight::start_combat(e, s.id, want) {
                self.retargets += 1;
            }
        }
    }

    /// The guest's frame, between two of the game's frames: every actor the host runs that is in
    /// this player's space is held at the host's state, this player's hits go to the host, and
    /// the damage the host decided is taken. `avatars` are the other players' avatars in this
    /// game. Returns what to send to the host.
    pub fn guest_step(
        &mut self,
        e: &mut Engine,
        sync: &mut GuestSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
    ) -> Vec<Msg> {
        let mut out = Vec::new();
        if !self.guest {
            return out;
        }
        self.start(e);
        self.know_avatars(e, avatars);
        // vats: this game's V.A.T.S. does not stop or slow a world other players are in.
        if let Some(active) = self.vats.step(e, sync.phase() == GuestPhase::Active) {
            out.push(Msg::Combat(sync.live.combat.vats_mark(active)));
        }
        let waiting = self.hold_after.is_some_and(|(quest, stage)| {
            coop_engine::quest::record(e, quest).is_some_and(|r| !r.done.contains(&stage))
        });
        if waiting != self.waiting {
            self.waiting = waiting;
            self.say(if waiting {
                "co-op fights: no actor is held while this player's own opening runs".into()
            } else {
                "co-op fights: this player's opening is done; the host's actors are held from now"
                    .into()
            });
        }
        // The host's packages are this game's copies' too, once this player's opening is done.
        coop_engine::commands::set_packages_on(e, self.follow && self.packages_on && !waiting);
        let me = live::local(e).filter(|l| l.loaded && !waiting);
        let mut holding = BTreeSet::new();
        let mut near_me = BTreeSet::new();
        self.puppets_begin();
        self.following_begin(now_ms);
        let fed: Vec<(u32, Option<PlayerId>, ActorState)> = sync
            .live
            .actors
            .iter()
            .filter_map(|(f, a)| Some((f.0, a.owner, a.state?)))
            .collect();
        for (id, owner, s) in fed {
            self.feed(id, owner, &s, now_ms);
        }
        if let Some(me) = me {
            let known: Vec<(u32, coop_core::actors::RemoteActor)> =
                sync.live.actors.iter().map(|(f, a)| (f.0, *a)).collect();
            for (id, a) in known {
                let Some(s) = a.state else {
                    continue;
                };
                if a.owner != Some(PlayerId::HOST) || !actors::shareable(id) {
                    continue;
                }
                if s.cell.0 != me.cell {
                    // The host's has left this player's space and this copy has not yet: it is
                    // asked to take the host's package, which walks it out as it did the host's
                    // (PC run 13: Joe Cobb's copy stood still when the host's left the saloon).
                    let here = actors::actor(e, id).map(|copy| fight::space_of(e, copy));
                    if self.follow && here == Some(me.cell) {
                        self.ask_package(e, id, now_ms);
                        if self.left_behind.insert(id) {
                            self.say(format!(
                                "co-op fights: actor {id:08x} is in {:08x} on the host and still in {:08x} here: asked to follow it out",
                                s.cell.0, me.cell
                            ));
                        }
                    }
                    continue;
                }
                self.left_behind.remove(&id);
                near_me.insert(id);
                // This game's copy is in another space than the host's actor (the host's walked
                // out of a door, this one has not): moving it would change its place and not its
                // cell, and it would stand at the other space's coordinates in this one (PC run
                // 10, 2026-11-11: Sunny, outdoors on the host, was moved to outdoor coordinates
                // inside the saloon). It is left to this game until it is in that space too.
                let here = actors::actor(e, id).map(|copy| fight::space_of(e, copy));
                if here.is_some_and(|space| space != s.cell.0) {
                    // Followed, it is brought through: the game's own `MoveTo`, to this player's
                    // space at the host's place (PC run 12: Sunny's copy stood at the door the
                    // host's had gone through). Not more often than every few seconds.
                    let due = self
                        .brought
                        .get(&id)
                        .map_or(true, |t| now_ms.saturating_sub(*t) >= BRING_EVERY_MS);
                    let times = self.brought_times.get(&id).copied().unwrap_or(0);
                    if self.follow
                        && self.bring_on
                        && due
                        && times < BRING_AT_MOST
                        && a.alive
                        && s.flags & FLAG_DEAD == 0
                        && actors::dead(e, id) == Some(false)
                    {
                        self.brought.insert(id, now_ms);
                        self.brought_times.insert(id, times + 1);
                        if times + 1 == BRING_AT_MOST {
                            self.say(format!(
                                "co-op fights: actor {id:08x} has been brought {BRING_AT_MOST} times and leaves again each time: left to this game from now"
                            ));
                        }
                        let to = s.pos.map(dequant_pos);
                        if fight::bring(e, id, to) {
                            self.brings += 1;
                            if self.brings <= 60 {
                                self.say(format!(
                                    "co-op fights: actor {id:08x} was in {:08x} here and is in {:08x} on the host: brought to ({:.0}, {:.0}, {:.0})",
                                    here.unwrap_or(0), s.cell.0, to[0], to[1], to[2]
                                ));
                            }
                            continue;
                        }
                    }
                    if self.elsewhere.insert(id) {
                        self.say(format!(
                            "co-op fights: actor {id:08x} is in {:08x} on the host and in {:08x} here: not held",
                            s.cell.0,
                            here.unwrap_or(0)
                        ));
                    }
                    continue;
                }
                self.elsewhere.remove(&id);
                if !a.alive || s.flags & FLAG_DEAD != 0 {
                    // The death's scripts ran on the host.
                    if actors::dead(e, id) == Some(false) {
                        gate::applying(e, |e| actors::kill(e, id));
                        self.deaths += 1;
                        self.say(format!(
                            "co-op fights: actor {id:08x} is dead on the host (alive in its ledger: {}, flags {:04x}): killed here",
                            a.alive, s.flags
                        ));
                    }
                    continue;
                }
                if self.follow {
                    // With the damage shared, a followed actor's hits go through the host as a
                    // held one's do (it is in the set the hit wrap reads), and its health is the
                    // host's. It keeps its own AI: it fights here as this game has it, and what
                    // its blows do is the host's to say.
                    if self.cap >= Cap::Damage && fight::mend(e, id, f32::from(s.health)) {
                        holding.insert(id);
                    }
                    // In a conversation here (a scene this game plays as the host's does, or one
                    // with this game's player), it is left to it: neither put at the host's place
                    // nor asked to take the host's package (PC run 30: Joe Cobb's copy cut his line
                    // and ran off when the host's, done with the scene first, took his leaving
                    // package). Once it is over it picks a package again, the host's.
                    if s.flags & FLAG_COMBAT == 0 && self.left_to_talk(e, id, now_ms) {
                        self.far_since.remove(&id);
                        self.talks_left += 1;
                        continue;
                    }
                    self.ask_package(e, id, now_ms);
                    // Where the host's actor is to be shown: along its path, a little behind,
                    // with its gait (D6).
                    let place = self.place(id, now_ms);
                    // In a shared fight the place matters more than anything: an actor fighting
                    // on the host is kept on the host's path every frame (PC run 17: the guest's
                    // copies fought their own fight, and were put back in jumps and out of sight).
                    if self.cap >= Cap::Damage && s.flags & FLAG_COMBAT != 0 {
                        if let Some(place) = place {
                            self.follow_copy(e, id, &place, Follow::Fight, now_ms);
                        }
                        self.fight_leads += 1;
                        // It fights whom the host's actor fights, and attacks when that one does.
                        self.puppet(e, id, &s, sync.player, avatars, now_ms);
                        continue;
                    }
                    // Not fighting on the host: its own attacks are refused here too, and it stops
                    // a fight of its own that the host's actor has not taken up.
                    if self.cap >= Cap::Damage {
                        self.puppet(e, id, &s, sync.player, avatars, now_ms);
                    }
                    // A copy whose package is aimed at the player may turn to this game's
                    // player while the host's turns to its own: it is given room, and only
                    // walked back when it is further than that.
                    let aimed = coop_engine::commands::wanted_package(e, id)
                        .is_some_and(|p| coop_engine::commands::aimed_at_player(e, p));
                    let how = if aimed {
                        Follow::WalkBack
                    } else {
                        Follow::Path
                    };
                    if let Some(place) = place {
                        self.follow_copy(e, id, &place, how, now_ms);
                    }
                    continue;
                }
                let first = !self.held.contains(&id) && self.first_holds < 60;
                let before = if first {
                    fight::snap_of(e, id).map(|b| b.pos)
                } else {
                    None
                };
                // Held (not followed): between two states the actor is moved in steps
                // (smooth.rs).
                let (pos, heading) = if self.smooth_on {
                    self.smooth.place(id, &s, now_ms)
                } else {
                    (s.pos.map(dequant_pos), dequant_heading(s.heading))
                };
                if let Some(h) = fight::hold(e, id, pos, heading, f32::from(s.health)) {
                    if let Some(b) = before {
                        self.first_holds += 1;
                        let to = s.pos.map(dequant_pos);
                        self.say(format!(
                            "co-op fights: holding actor {id:08x}: here it was at ({:.0}, {:.0}, {:.0}), the host has it at ({:.0}, {:.0}, {:.0}), after the move it is at ({:.0}, {:.0}, {:.0})",
                            b[0], b[1], b[2], to[0], to[1], to[2], h.pos[0], h.pos[1], h.pos[2]
                        ));
                    }
                    self.moves += 1;
                    self.woke += u64::from(h.woke && self.held.contains(&id));
                    holding.insert(id);
                }
            }
        }
        self.puppets_end(e);
        self.following_end(e);
        // Only the actors the host runs near this player keep a "far since" time: one that is
        // seen again later starts afresh.
        self.far_since.retain(|id, _| near_me.contains(id));
        // Every actor the host says a package of keeps it here, wherever it and this player are
        // (the host takes its word back when it no longer runs the actor): one that walked out
        // after the host's must not turn round because this game would have it elsewhere (PC
        // run 13: Sunny's copy went out and came back in).
        // The actors this game runs (the host handed them over: this player is the nearest).
        let owned: Vec<u32> = sync.live.actors.owned().iter().map(|f| f.0).collect();
        // One of them picks its own package here, as this game has the story: the host's copy is
        // kept still while this game runs it, and its package there says nothing.
        let followed: BTreeSet<u32> = match me {
            Some(_) => coop_engine::commands::wanted_ids(e)
                .into_iter()
                .filter(|id| !owned.contains(id))
                .collect(),
            None => BTreeSet::new(),
        };
        // (Not told to pick again when it becomes this game's: the harness found that a pick
        // asked of the game at that moment left Sunny standing, never greeting this player; the
        // game's own evaluation picks the same package from the shared story.)
        for id in &owned {
            if !self.owned_before.contains(id) {
                self.say(format!(
                    "co-op fights: actor {id:08x} is this game's to run: it picks its own package"
                ));
                // It goes on from where the host's game had it: this game's copy, run by nobody
                // near while no player was near it, may have stayed behind (harness, 2026-11-13:
                // the guest's Sunny stood at the first well on a stale package while the host's
                // had walked on to the third; the guest's came back and took her over there).
                let host_state = sync
                    .live
                    .actors
                    .iter()
                    .find(|(f, _)| f.0 == *id)
                    .and_then(|(_, a)| a.state);
                let here_space = actors::actor(e, *id).map(|a| fight::space_of(e, a));
                if let Some(s) = host_state.filter(|s| {
                    here_space != Some(s.cell.0) && me.is_some_and(|l| l.cell == s.cell.0)
                }) {
                    // Its copy here is in another space than the host's actor, which is in this
                    // player's (harness, 2026-11-13: Sunny walked from the saloon to her campfire
                    // in the host's game and was handed to the guest there; the guest's copy, never
                    // near anyone, was still in the saloon, where this game cannot run her near its
                    // player, and both copies stood). It is brought out to the host's place, as a
                    // followed copy is through a door.
                    let to = s.pos.map(dequant_pos);
                    if fight::bring(e, *id, to) {
                        self.takeover_brings += 1;
                        self.say(format!(
                            "co-op fights: actor {id:08x} was in {:08x} here and is in {:08x} on the host: brought to ({:.0}, {:.0}, {:.0}) to be run here",
                            here_space.unwrap_or(0), s.cell.0, to[0], to[1], to[2]
                        ));
                    }
                } else if let (Some(s), Some(here)) = (host_state, fight::place_of(e, *id)) {
                    let to = s.pos.map(dequant_pos);
                    let off = (0..3)
                        .map(|i| (here[i] - to[i]).powi(2))
                        .sum::<f32>()
                        .sqrt();
                    let same_space =
                        actors::actor(e, *id).is_some_and(|a| fight::space_of(e, a) == s.cell.0);
                    if same_space && off > TAKEOVER_DRIFT {
                        fight::lead(e, *id, to, dequant_heading(s.heading));
                        self.say(format!(
                            "co-op fights: actor {id:08x} was {off:.0} units from where the host's game had it: put there"
                        ));
                    }
                }
            }
        }
        self.owned_before = owned.iter().copied().collect();
        // A copy near this player of an actor the host runs out of its own player's sight (no
        // place is told of it then, only its package): asked to take that package too.
        if let Some(me) = me.filter(|_| self.follow) {
            for id in &followed {
                if near_me.contains(id) || self.left_behind.contains(id) {
                    continue;
                }
                let here = actors::actor(e, *id).map(|copy| fight::space_of(e, copy));
                if here == Some(me.cell) {
                    self.ask_package(e, *id, now_ms);
                }
            }
        }
        coop_engine::commands::set_followed(e, followed);
        if self.cap >= Cap::Damage {
            // Every player's hit on them goes through the host's ledger, this one's too, and
            // the damage the ledger records is done here, where they are run.
            fight::set_ledgered(e, owned.iter().copied());
            for (actor, by, amount) in std::mem::take(&mut sync.live.actors.damage_log) {
                if !owned.contains(&actor.0) || amount == 0 {
                    continue;
                }
                let attacker = if by == sync.player {
                    actors::PLAYER_REF_ID
                } else {
                    avatars.iter().find(|(p, _)| *p == by).map_or(0, |a| a.1)
                };
                if fight::hurt_actor(e, actor.0, f32::from(amount), attacker).is_some() {
                    self.owned_damage_taken += 1;
                }
            }
        }
        if !owned.is_empty()
            && self
                .owned_published_ms
                .map_or(true, |t| now_ms.saturating_sub(t) >= PUBLISH_EVERY_MS)
        {
            self.owned_published_ms = Some(now_ms);
            for id in &owned {
                // Only one this game has loaded, alive (PC run 25: after a load, actors this
                // game no longer had were sent as they were not).
                if fight::place_of(e, *id).is_none() || actors::dead(e, *id) != Some(false) {
                    continue;
                }
                let Some(a) = actors::actor(e, *id) else {
                    continue;
                };
                let s = fight::snap(e, a);
                if s.space == 0 {
                    continue;
                }
                let state = Self::state_of(&s, avatars, sync.player, puppet::attack_of(e, *id));
                if self.owned_last.get(id) == Some(&state) {
                    continue;
                }
                let stamped = ActorState {
                    ms: now_ms as u32,
                    ..state
                };
                if let Some(msg) = sync.live.actors.publish(FormId(*id), stamped) {
                    self.owned_last.insert(*id, state);
                    self.owned_published += 1;
                    out.push(Msg::Actor(msg));
                }
            }
        }
        self.owned_last.retain(|id, _| owned.contains(id));
        for id in self.held.difference(&holding) {
            // (a followed actor's AI was never off)
            if !self.follow {
                fight::release(e, *id);
            }
            self.released += 1;
        }
        if holding != self.held {
            fight::set_held(e, holding.iter().copied());
            self.held = holding;
        }
        self.smooth.keep(|id| self.held.contains(&id));
        // A copy here went for this player: the game that runs the actor is asked to take the
        // fight up; another game's copy of an actor this game runs went for its player: this
        // game's actor fights that player's avatar.
        for id in std::mem::take(&mut self.puppets.engage) {
            out.push(Msg::Actor(ActorMsg::Engage {
                actor: FormId(id),
                player: sync.player,
            }));
            self.puppets.asks += 1;
        }
        for (actor, player) in std::mem::take(&mut sync.live.actors.engages) {
            if self.cap >= Cap::Threat {
                let target = local_target(
                    PLAYER_TARGET_BASE + u32::from(player.0),
                    sync.player,
                    avatars,
                );
                self.take_up(e, actor.0, target);
            }
        }
        if self.cap >= Cap::Damage {
            self.guest_damage(e, sync, avatars, now_ms, &mut out);
        }
        if self.cap >= Cap::Revive {
            let me = sync.player;
            for m in self.body(e, &mut sync.live, me, now_ms) {
                out.push(Msg::Rules(m));
            }
        }
        self.stamp(e, &mut sync.live.local);
        out
    }

    /// Whether this game's copy of actor `id` is left to a conversation it is in here: for
    /// [`TALK_GRACE_MS`] from when it was first seen in it.
    fn left_to_talk(&mut self, e: &mut Engine, id: u32, now_ms: u64) -> bool {
        if !fight::in_dialogue(e, id) {
            self.talk_since.remove(&id);
            return false;
        }
        let since = *self.talk_since.entry(id).or_insert(now_ms);
        now_ms.saturating_sub(since) < TALK_GRACE_MS
    }

    /// Asks this game's copy of actor `id` to pick its package again when it does not run the
    /// one the host's runs.
    fn ask_package(&mut self, e: &mut Engine, id: u32, now_ms: u64) {
        // Not running the host's package (an interrupt ended, a cell loaded): told
        // to pick again, without a reset of its AI, at most every few seconds and a
        // few times for one package (it may be one this copy cannot run).
        if let Some(wanted) = coop_engine::commands::wanted_package(e, id) {
            let due = self
                .package_asked
                .get(&id)
                .map_or(true, |t| now_ms.saturating_sub(*t) >= PACKAGE_ASK_MS);
            // A package the host has just changed to is asked for at once.
            let tries = self.package_tries.get(&id).copied();
            let changed = tries.map(|(p, _)| p) != Some(wanted);
            if (due || changed) && self.packages_on {
                self.package_asked.insert(id, now_ms);
                let n = match tries {
                    Some((p, n)) if p == wanted => n,
                    _ => 0,
                };
                let runs = coop_engine::commands::current_package(e, id);
                if runs == Some(wanted) {
                    self.package_tries.insert(id, (wanted, 0));
                } else if n < PACKAGE_ASKS_AT_MOST {
                    self.package_tries.insert(id, (wanted, n + 1));
                    self.package_asks += 1;
                    gate::applying(e, |e| coop_engine::commands::repick_package(e, id));
                    if n + 1 == PACKAGE_ASKS_AT_MOST {
                        self.say(format!(
                            "co-op packages: actor {id:08x} does not take the host's package {wanted:08x} (it runs {:08x}): no longer asked",
                            runs.unwrap_or(0)
                        ));
                    }
                }
            }
        }
    }

    /// The host's own player: what the other players' hits and the reviver decided for it.
    /// `view` is the host player's own view of the session (what the host sends to itself).
    /// Returns the sends.
    pub fn host_body(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        view: &mut GuestLive,
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mut out = Vec::new();
        if self.guest || self.cap < Cap::Damage {
            return out;
        }
        self.take_hurts(e, view);
        if self.cap >= Cap::Revive {
            for m in self.body(e, view, PlayerId::HOST, now_ms) {
                out.extend(sync.on_local(Msg::Rules(m)));
            }
        }
        out
    }

    /// Writes this game's player's health and fall into the state the other players get.
    pub fn stamp(&self, e: &mut Engine, state: &mut AvatarState) {
        // The host hands this player no actor while its own opening runs.
        if self.waiting {
            state.flags |= AVATAR_OPENING;
        }
        if self.cap < Cap::Damage {
            return;
        }
        if let Some(h) = fight::player_health(e) {
            state.health = health16(h);
        }
        if self.down {
            state.flags |= AVATAR_DOWNED;
        }
    }

    /// The damage the host decided for this game's player is taken by it.
    fn take_hurts(&mut self, e: &mut Engine, glive: &mut GuestLive) {
        for n in std::mem::take(&mut glive.combat.hurts) {
            if let Some(left) = fight::hurt_player(e, f32::from(n.amount), n.attacker.0) {
                self.hurts_taken += 1;
                // Without revives a player at no health dies as the game has it.
                if left <= 0.0 && self.cap < Cap::Revive {
                    fight::kill_player(e);
                }
            }
        }
    }

    /// Whether player `p` stands next to this game's player.
    fn next_to(e: &mut Engine, glive: &mut GuestLive, p: PlayerId, now_ms: u64) -> bool {
        let (Some(me), Some(s)) = (live::local(e).filter(|l| l.loaded), glive.sample(p, now_ms))
        else {
            return false;
        };
        me.cell == s.cell.0 && dist(me.pos, s.pos) <= CLIENT_RANGE
    }

    /// This game's player's fall and rise, and its help for another player who is down.
    /// `glive` is this player's view of the session. Returns what to tell the host's rules.
    fn body(
        &mut self,
        e: &mut Engine,
        glive: &mut GuestLive,
        me: PlayerId,
        now_ms: u64,
    ) -> Vec<RulesMsg> {
        let mut out = Vec::new();
        let Some(health) = fight::player_health(e) else {
            return out;
        };
        self.max_health = self.max_health.max(health);
        if self
            .wiped_ms
            .is_some_and(|t| now_ms.saturating_sub(t) >= SPARE_AGAIN_MS)
        {
            self.wiped_ms = None;
            fight::set_spare(e, true);
        }
        let fell = fight::take_falls(e) > 0 || health <= 0.0;
        if fell && !self.down && self.wiped_ms.is_none() {
            self.down = true;
            self.downs += 1;
            self.saved_controls = fight::controls(e).unwrap_or(0);
            self.say(
                "co-op fights: this player is down; another player standing next to them revives them"
                    .into(),
            );
            out.push(RulesMsg::ReportDown { revivable: true });
        }
        // The reviver's word for this player; other notices are not the fights' to take.
        let (mut up, mut wipe) = (None, false);
        glive.rules.notices.retain(|n| match n {
            Notice::Revived(pct) => {
                up = Some(f32::from(*pct));
                false
            }
            Notice::Respawn => {
                up = Some(100.0);
                false
            }
            Notice::Wipe => {
                wipe = true;
                false
            }
            _ => true,
        });
        if self.down {
            self.helping = None;
            if wipe {
                self.down = false;
                self.wipes += 1;
                self.wiped_ms = Some(now_ms);
                fight::set_controls(e, self.saved_controls);
                fight::set_spare(e, false);
                self.say("co-op fights: everyone is down; this player dies".into());
                fight::kill_player(e);
            } else if let Some(pct) = up {
                self.down = false;
                self.ups += 1;
                fight::set_controls(e, self.saved_controls);
                fight::set_player_health(e, (self.max_health * pct / 100.0).max(1.0));
                self.say(format!(
                    "co-op fights: this player is up again at {pct:.0} percent health"
                ));
            } else {
                if health < 1.0 {
                    fight::set_player_health(e, 1.0);
                }
                fight::set_controls(e, self.saved_controls | PLAYER_ALL_CONTROLS);
            }
            return out;
        }
        // Up: stand next to a downed player and the host is asked to let this one revive them.
        let downed: Vec<PlayerId> = glive
            .rules
            .downed
            .iter()
            .copied()
            .filter(|p| *p != me)
            .collect();
        match self.helping {
            Some((t, asked)) => {
                if !downed.contains(&t) {
                    self.helping = None;
                } else if !Self::next_to(e, glive, t, now_ms) {
                    out.push(RulesMsg::ReviveCancel);
                    self.helping = None;
                } else if now_ms.saturating_sub(asked) >= REVIVE_ASK_EVERY_MS {
                    out.push(RulesMsg::ReviveBegin { target: t });
                    self.helping = Some((t, now_ms));
                }
            }
            None => {
                for t in downed {
                    if Self::next_to(e, glive, t, now_ms) {
                        out.push(RulesMsg::ReviveBegin { target: t });
                        self.helping = Some((t, now_ms));
                        break;
                    }
                }
            }
        }
        out
    }

    /// Whether this game's player is down.
    pub fn down(&self) -> bool {
        self.down
    }

    /// The guest's part of the damage: this player's kept hits go to the host, and what the host
    /// decided for this player is taken by this game's player.
    fn guest_damage(
        &mut self,
        e: &mut Engine,
        sync: &mut GuestSync,
        avatars: &[(PlayerId, u32)],
        now_ms: u64,
        out: &mut Vec<Msg>,
    ) {
        // vats: also the health a held actor or an avatar lost outside `Actor::HitMe`.
        for (h, kind) in self.blows(e, now_ms) {
            let amount = damage16(h.amount);
            if amount == 0 {
                continue;
            }
            if let Some((victim, _)) = avatars.iter().find(|(_, a)| *a == h.target) {
                // Another player: the host's friendly-fire rule decides.
                if h.aggressor == actors::PLAYER_REF_ID {
                    out.push(Msg::Combat(
                        sync.live
                            .combat
                            .player_hit(*victim, amount, kind, LIMB_COUNT),
                    ));
                }
            } else {
                out.push(Msg::Actor(sync.live.actors.hit(FormId(h.target), amount)));
                self.hits_sent += 1;
            }
        }
        self.take_hurts(e, &mut sync.live);
    }

    /// Host: the actors a guest's game runs now (hand-off).
    pub fn theirs(&self) -> &BTreeSet<u32> {
        &self.theirs
    }

    /// The actors this guest holds now.
    pub fn held(&self) -> &BTreeSet<u32> {
        &self.held
    }

    /// Gives every held actor back to this game and takes this machine out of the hit wrap (the
    /// session ends).
    pub fn close(&mut self, e: &mut Engine) {
        for id in std::mem::take(&mut self.held) {
            fight::release(e, id);
            self.released += 1;
        }
        fight::set_held(e, []);
        fight::set_avatars(e, []);
        fight::set_role(e, Role::Off);
        harm::set_on(e, false);
        // Copies are their games' own again: their own attacks, targets and movement.
        self.puppets.copies.clear();
        puppet::set_copies(e, []);
        for id in std::mem::take(&mut self.following.forced).into_keys() {
            puppet::free_moves(e, id);
        }
        fight::set_spare(e, false);
        // vats
        self.vats.close(e);
        if self.down {
            self.down = false;
            fight::set_controls(e, self.saved_controls);
        }
        self.avatars.clear();
        self.started = false;
    }

    /// One status phrase for the log.
    pub fn status(&self) -> String {
        // vats
        format!(
            "{}; V.A.T.S. {:?}, entered {}; losses outside hits sent {}",
            self.fights_status(),
            self.vats.mode(),
            self.vats.entered,
            self.harms_sent
        )
    }

    fn fights_status(&self) -> String {
        if self.guest {
            format!(
                "fights: walked back to the host's place {}, brought through a door {}, asked to take the host's package {}; holding {} actors, moves {}, AI woke {}, released {}, deaths {}, hits sent {}, hurts taken {}, down {} up {}; {}; {}",
                self.nudges,
                self.brings,
                self.package_asks,
                self.held.len(),
                self.moves,
                self.woke,
                self.released,
                self.deaths,
                self.hits_sent,
                self.hurts_taken,
                self.downs,
                self.ups,
                self.puppets.status(),
                self.following.status()
            )
        } else {
            format!(
                "fights: tracking {} actors, states sent {}, guests' hits applied {}, kills {}, hurts sent {}, retargets {}, down {} up {}; {}; {}",
                self.tracked.len(),
                self.published,
                self.hits_applied,
                self.kills,
                self.hurts_sent,
                self.retargets,
                self.downs,
                self.ups,
                self.puppets.status(),
                self.following.status()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use coop_core::avatar::quant_pos;

    fn state(ms: u32, x: f32) -> ActorState {
        ActorState {
            pos: [x, 0.0, 0.0].map(quant_pos),
            anim: 1,
            ms,
            ..ActorState::default()
        }
    }

    /// An actor that changes owner is played back on the new owner's clock: the old owner's
    /// states (another clock, perhaps far ahead) do not hold the new ones back.
    #[test]
    fn a_new_owners_states_start_a_new_track() {
        let mut f = Fight::new(true, Cap::Revive).quiet();
        let id = 0x0010_4f25;
        f.feed(id, Some(PlayerId(1)), &state(1_000_000, 0.0), 10);
        f.feed(id, Some(PlayerId(1)), &state(1_000_050, 10.0), 60);
        f.feed(id, Some(PlayerId::HOST), &state(100, 500.0), 110);
        f.feed(id, Some(PlayerId::HOST), &state(150, 510.0), 160);
        let x = f.place(id, 400).unwrap().pos[0];
        assert!((x - 510.0).abs() < 0.01, "{x}");
    }
}
