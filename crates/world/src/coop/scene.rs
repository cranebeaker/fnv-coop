//! Scenes, movies, control locks, endings and world time on `GameState`.
//!
//! The host's story decides when a scene starts and ends. Three kinds of site
//! tell the bridge about it, all through hooks that cost one authority test in
//! single player:
//!
//! * a script's `PlayBink` ([`GameState::coop_scene_movie`]),
//! * a script's `DisablePlayerControls` / `EnablePlayerControls`
//!   ([`GameState::coop_controls_begin`], [`GameState::coop_controls_end`]),
//! * a stage whose quest the campaign policy marks `cinematic` or `ending`
//!   (`coop_gate_stage`).
//!
//! [`SceneBridge::host_frame`] turns the notes into [`SceneHost`] scenes, merges
//! roaming players in, repeats the clock and runs approved time skips once.
//! [`SceneBridge::local_frame`] (every machine, the host's too) turns the
//! [`SceneClient`]'s notices into world changes: control locks for this
//! player, the movie to play (with its scene time), the slide, the time skip,
//! and a guest's clock following the host's smoothly.
//!
//! Waiting and sleeping: the menu's *begin* no longer starts the hours
//! ([`GameState::coop_time_intercept`]); it becomes a party vote
//! ([`coop_core::rules::VoteKind::Sleep`] / `Wait`, everyone present must
//! agree). When the vote passes the host runs the hours once and announces the
//! resulting clock; no guest runs a wait of its own.

use coop_core::ids::PlayerId;
use coop_core::proto::Msg;
use coop_core::rules::VoteKind;
use coop_core::scene::{
    BarrierRule, CameraMode, ClockSnap, EndReason, Locks, Mask, Movie, Notice, Phase, SceneClient,
    SceneEvent, SceneKind, SceneMsg, SceneSpec, SceneView, SkipRule,
};
use coop_core::sync::HostLive;
use esm::{FormId, LoadOrder};

use super::apply::Ctx;
use super::policy_gate::PolicyGate;
use super::Authority;
use crate::scripting::GameState;

/// How far apart (game hours) the guest's clock may drift before it jumps.
pub const CLOCK_JUMP_HOURS: f64 = 0.5;
/// Seconds the guest takes to close a drift (the slew time constant).
pub const CLOCK_SLEW_S: f32 = 0.5;
/// Longest an ending lasts when the story gives no limit (ms).
pub const ENDING_LIMIT_MS: u32 = 600_000;
/// Longest a policy cinematic lasts when the row gives no limit (ms).
pub const CINEMATIC_LIMIT_MS: u32 = 120_000;
/// Most hours one approved skip runs.
const MAX_SKIP_HOURS: u32 = 48;
/// Slides an ending scene takes (the story gives no count when it opens).
pub const ENDING_SLIDES: u16 = coop_core::scene::MAX_SLIDES as u16;

/// A movie the viewer should play now.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartMovie {
    /// The scene it belongs to.
    pub scene: u32,
    /// The movie.
    pub movie: Movie,
    /// Scene time already played (a late joiner or a merge starts later in the file).
    pub offset_ms: u32,
}

/// A time skip to show on this machine (fade, message).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeShown {
    /// Hours.
    pub hours: u16,
    /// A sleep (fade to black) rather than a wait.
    pub sleeping: bool,
}

/// What the bridge counts, for the state dump.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SceneCounters {
    /// Scenes this machine is in or watching now.
    pub active: u32,
    /// Id of the main scene (0: none).
    pub id: u32,
    /// 0 none, 1 gathering, 2 playing.
    pub phase: u8,
    /// Kind of the main scene as a number (see [`kind_code`]).
    pub kind: u8,
    /// Locks in force for this player.
    pub locks: u8,
    /// Scenes opened here so far.
    pub opened: u32,
    /// Scenes closed here so far.
    pub closed: u32,
    /// Reason of the last close (see [`reason_code`]).
    pub last_reason: u8,
    /// Movies started here.
    pub movies: u32,
    /// Time skips applied here.
    pub time_skips: u32,
    /// Hours the skips covered.
    pub skip_hours: u32,
    /// Highest ending slide shown.
    pub slide: u16,
    /// Ending tags committed.
    pub committed: u32,
    /// Seconds of the scene clock at the last frame.
    pub elapsed_ms: u32,
}

/// A number for a scene kind (dump and tests).
pub fn kind_code(k: SceneKind) -> u8 {
    match k {
        SceneKind::Control => 1,
        SceneKind::Cinematic => 2,
        SceneKind::Movie => 3,
        SceneKind::Ending => 4,
        SceneKind::Dialogue => 5,
        SceneKind::Opening => 6,
    }
}

/// A number for an end reason.
pub fn reason_code(r: EndReason) -> u8 {
    match r {
        EndReason::Finished => 1,
        EndReason::Skipped => 2,
        EndReason::TimedOut => 3,
        EndReason::Aborted => 4,
        EndReason::Empty => 5,
    }
}

/// A person a host script made talk (`StartConversation`, `SayTo`, `Say`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Talk {
    /// Who talks.
    pub speaker: FormId,
    /// Who they talk to (0: no one in particular).
    pub to: FormId,
    /// The topic (`None`: their greeting).
    pub topic: Option<FormId>,
    /// `StartConversation` (a menu) rather than one line.
    pub conversation: bool,
}

/// What forced talk and the pick checks did on this machine (state dump).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TalkCounters {
    /// Talk notes the host took from its scripts.
    pub notes: u32,
    /// Dialogue scenes the host opened for a script's conversation.
    pub scenes: u32,
    /// Lines the host said to everyone without a menu (`SayTo`, `Say`).
    pub lines: u32,
    /// Talk notes the host could not share (busy, no line to say).
    pub dropped: u32,
    /// Picks the host refused (the line's conditions fail for the picker).
    pub refused: u32,
    /// Picks the host accepted after the check.
    pub checked: u32,
    /// Picks of this machine's player the host refused.
    pub refusals: u32,
    /// Frames this machine ran while it watched another player's session.
    pub watched_frames: u32,
    /// Milliseconds of those frames.
    pub watched_ms: u32,
    /// Milliseconds of those frames in which the world clock moved on here.
    pub watched_clock_ms: u32,
    /// Milliseconds of all frames in which the world clock moved on here.
    pub clock_ms: u32,
    /// How far the world clock here fell behind real time since the first
    /// frame, in real milliseconds (the time it stood still).
    pub clock_lag_ms: u32,
}

/// Per-machine scene facts on `GameState` (never replicated, never saved).
#[derive(Clone, Debug, Default)]
pub struct SceneFacts {
    /// People host scripts made talk: the host shares them.
    pub talk_notes: Vec<Talk>,
    /// Forced talk and pick check counters.
    pub talk: TalkCounters,
    /// Host: its own conversation has had no pick for longer than the
    /// bound, so the world's time runs on while its menu is up.
    pub conversation_runs_clock: bool,
    /// Fades, sounds and music of host scripts (see [`super::presentation`]).
    pub present: super::presentation::PresentFacts,
    /// Movies scripts started (quest, movie): the host turns them into scenes.
    pub movie_notes: Vec<(Option<FormId>, Movie)>,
    /// Stages the policy marked as host scenes: (quest, kind).
    pub stage_notes: Vec<(FormId, SceneKind)>,
    /// What the scripts last set for the player's controls (the truth for
    /// `GetPlayerControlsDisabled` while `controls_off` shows what this
    /// player is under).
    pub script_controls: Option<[bool; 7]>,
    /// A script changed the controls (the quest that ran it).
    pub controls_dirty: Option<Option<FormId>>,
    /// Waits and sleeps the local player asked for: (sleeping, hours).
    pub time_asks: Vec<(bool, u32)>,
    /// Waits and sleeps the party approved, for the host to run once.
    pub time_approved: Vec<(bool, u32)>,
    /// The host is running an approved skip: `sleep::begin` may start.
    pub time_exec: bool,
    /// Slides the host's slideshow showed.
    pub slide_notes: Vec<u16>,
    /// The host's slideshow ended.
    pub ending_done: bool,
    /// Abort every scene (a save loaded, the story reset).
    pub abort: bool,
    /// The movie the viewer should play now.
    pub movie_start: Option<StartMovie>,
    /// The viewer stops the movie now (the scene ended or was skipped).
    pub movie_stop: bool,
    /// The viewer reports a movie on screen.
    pub movie_busy: bool,
    /// Skip votes the local player cast (yes or no).
    pub skip_asks: Vec<bool>,
    /// The main scene as this machine shows it.
    pub view: Option<SceneView>,
    /// The ending slide to show (the viewer takes it).
    pub slide: Option<u16>,
    /// A time skip to show (the viewer takes it).
    pub time_shown: Option<TimeShown>,
    /// The guest follows the host's clock (no clock global is written by deltas).
    pub follow: bool,
    /// The clock values the host replicated, while the guest follows.
    pub replicated: std::collections::BTreeMap<FormId, f32>,
    /// Counters.
    pub counters: SceneCounters,
}

impl SceneFacts {
    /// `PlayBink` hook with the script's own `Video` (the caller tested the
    /// authority; a field of the state, so it can run while events are held).
    pub fn note_video(&mut self, quest: Option<FormId>, v: &crate::scripting::Video) {
        self.movie_notes.push((
            quest,
            Movie {
                file: v.file.clone(),
                interruptable: v.interruptable,
                mute_audio: v.mute_audio,
                pause_music: v.pause_music,
                letterbox: v.letterbox,
            },
        ));
    }
}

impl GameState {
    /// `PlayBink` hook: a script started a movie. In a co-op session the
    /// scene layer plays it on every machine together; the viewer must not
    /// queue it itself ([`GameState::coop_scene_owns_movies`]).
    pub fn coop_scene_movie(&mut self, quest: Option<FormId>, movie: Movie) {
        if self.coop.authority == Authority::Single {
            return;
        }
        self.coop.live.scene.movie_notes.push((quest, movie));
    }

    /// True when the scene layer plays script movies (every co-op machine).
    pub fn coop_scene_owns_movies(&self) -> bool {
        self.coop.authority != Authority::Single
    }

    /// Before a script changes the controls: the truth the scripts see is
    /// put back in `controls_off` (a player outside the scene may show
    /// none).
    pub fn coop_controls_begin(&mut self) {
        if self.coop.authority == Authority::Single {
            return;
        }
        if let Some(sc) = self.coop.live.scene.script_controls {
            self.controls_off = sc;
        }
    }

    /// After a script changed the controls.
    pub fn coop_controls_end(&mut self, quest: Option<FormId>) {
        if self.coop.authority == Authority::Single {
            return;
        }
        self.coop.live.scene.script_controls = Some(self.controls_off);
        self.coop.live.scene.controls_dirty = Some(quest);
    }

    /// `sleep::begin` hook: true when the hours must not start here (the
    /// request became a party vote).
    pub fn coop_time_intercept(&mut self, hours: u32, sleeping: bool) -> bool {
        if self.coop.authority == Authority::Single || self.coop.live.scene.time_exec {
            return false;
        }
        if hours > 0 {
            self.coop.live.scene.time_asks.push((sleeping, hours));
        }
        true
    }

    /// Set stage hook: records the stage when the policy marks it a host
    /// scene.
    pub(crate) fn coop_scene_stage(&mut self, quest: FormId) {
        if self.coop.authority != Authority::Host {
            return;
        }
        let kind = self
            .coop
            .gate
            .as_ref()
            .and_then(|g| g.table().row(quest.0))
            .and_then(|r| r.host_decision);
        let kind = match kind {
            Some(coop_policy::HostDecision::Cinematic) => SceneKind::Cinematic,
            Some(coop_policy::HostDecision::Ending) => SceneKind::Ending,
            _ => return,
        };
        let notes = &mut self.coop.live.scene.stage_notes;
        if !notes.contains(&(quest, kind)) {
            notes.push((quest, kind));
        }
    }

    /// Clock global hook (apply): true when a guest follows the host's clock
    /// itself. The replicated value is kept aside (the shared hash reads it,
    /// so the guest agrees with the host) and not written over the guest's
    /// smooth clock.
    pub(crate) fn coop_clock_applied(&mut self, form: FormId, v: f32) -> bool {
        if self.coop.authority == Authority::Guest
            && self.coop.live.scene.follow
            && self.coop.clock.contains(&form)
        {
            self.coop.live.scene.replicated.insert(form, v);
            return true;
        }
        false
    }

    /// Export hook: the value of a clock global the host replicated, when the
    /// local one is a smooth follower's.
    pub(crate) fn coop_clock_exported(&self, form: FormId, _local: f32) -> Option<f32> {
        self.coop.live.scene.replicated.get(&form).copied()
    }

    /// `StartConversation`, `SayTo` and `Say` hook: a script made a person
    /// talk. On the host the live layer shares it: a conversation with the
    /// player opens a dialogue scene for everyone, and a line is said to
    /// everyone (see [`super::dialogue::DialogueBridge::forced_frame`]).
    pub fn coop_talk(
        &mut self,
        speaker: FormId,
        to: FormId,
        topic: Option<FormId>,
        conversation: bool,
    ) {
        if self.coop.authority != Authority::Host
            || speaker == crate::dialogue::PLAYER_REF
            || super::facts::player_of(speaker).is_some()
        {
            return;
        }
        self.coop.live.scene.talk_notes.push(Talk {
            speaker,
            to,
            topic,
            conversation,
        });
    }

    /// Whether this machine's open conversation stops its world's time (the
    /// viewer's `scripts` gate around `Runner::update`). In single player and
    /// on a guest it always does. The host's time is everyone's time: its
    /// conversation stops it for at most one dialogue heartbeat
    /// ([`super::dialogue::CONVERSATION_HOLD_MS`]) after its last pick.
    pub fn coop_conversation_stops_time(&self) -> bool {
        self.coop.authority != Authority::Host || !self.coop.live.scene.conversation_runs_clock
    }

    /// The host's slideshow shows slide `index` (everyone sees it in order).
    pub fn coop_scene_slide(&mut self, index: u16) {
        if self.coop.authority == Authority::Host {
            self.coop.live.scene.slide_notes.push(index);
        }
    }

    /// The host's slideshow is over.
    pub fn coop_scene_ending_done(&mut self) {
        if self.coop.authority == Authority::Host {
            self.coop.live.scene.ending_done = true;
        }
    }

    /// The local player wants to skip the scene (or takes it back).
    pub fn coop_scene_skip(&mut self, yes: bool) {
        if self.coop.authority != Authority::Single {
            self.coop.live.scene.skip_asks.push(yes);
        }
    }

    /// Ends every scene (a save loaded, the story restarted).
    pub fn coop_scene_abort(&mut self) {
        if self.coop.authority == Authority::Host {
            self.coop.live.scene.abort = true;
        }
    }
}

fn fnv(s: &str) -> u32 {
    let mut h: u32 = 0x811c_9dc5;
    for b in s.bytes() {
        h ^= u32::from(b);
        h = h.wrapping_mul(0x0100_0193);
    }
    h
}

/// The seven lock flags of a policy mask such as `"1 1 1 0 1 1 1"`.
pub fn parse_mask(s: &str) -> Locks {
    let bits: Vec<bool> = s.split_whitespace().map(|w| w != "0").collect();
    let mut f = [false; 7];
    for (i, b) in bits.iter().take(7).enumerate() {
        f[i] = *b;
    }
    Locks::from_flags(&f)
}

fn clock_of(order: &LoadOrder, gs: &GameState) -> ClockSnap {
    let g = |n: &str, d: f32| gs.global(order, n).unwrap_or(d);
    ClockSnap {
        hour: g("GameHour", 0.0).to_bits(),
        days_passed: g("GameDaysPassed", 1.0).to_bits(),
        year: g("GameYear", 2281.0) as u16,
        month: g("GameMonth", 0.0) as u8,
        day: g("GameDay", 1.0) as u8,
        scale: g("TimeScale", 30.0).to_bits(),
        weather: gs.weather.forced.or(gs.weather.picked).map_or(0, |f| f.0),
    }
}

/// Writes a clock into the globals (and the weather the host picked).
fn set_clock(order: &LoadOrder, gs: &mut GameState, c: &ClockSnap) {
    gs.set_global(order, "GameYear", f32::from(c.year));
    gs.set_global(order, "GameMonth", f32::from(c.month));
    gs.set_global(order, "GameDay", f32::from(c.day));
    gs.set_global(order, "GameDaysPassed", c.days_f());
    gs.set_global(order, "GameHour", c.hour_f());
    gs.set_global(order, "TimeScale", c.scale_f());
    if c.weather != 0 {
        gs.weather.picked = Some(FormId(c.weather));
    }
}

/// A guest's clock one frame closer to the host's: it runs on at the host's
/// timescale, closes a small drift over [`CLOCK_SLEW_S`], and jumps when far
/// off (a skip, a fast travel, a long stall).
pub fn follow_clock(
    order: &LoadOrder,
    gs: &mut GameState,
    clock: Option<(ClockSnap, u64)>,
    now_ms: u64,
    dt: f32,
) {
    let Some((snap, at)) = clock else { return };
    gs.coop.live.scene.follow = true;
    let scale = f64::from(snap.scale_f());
    if scale <= 0.0 {
        set_clock(order, gs, &snap);
        return;
    }
    let ahead_h = now_ms.saturating_sub(at) as f64 / 1000.0 * scale / 3600.0;
    let target = (f64::from(snap.days_f()) * 24.0) + ahead_h;
    let days_before = gs.global(order, "GameDaysPassed").unwrap_or(snap.days_f());
    if (target - f64::from(days_before) * 24.0).abs() > CLOCK_JUMP_HOURS {
        set_clock(order, gs, &snap);
        gs.advance_clock(order, (ahead_h * 3600.0 / scale) as f32);
        return;
    }
    // The hour sets the slew. GameDaysPassed is a float, and each frame's
    // step loses part of itself in the rounding (more at a high frame rate),
    // so the day count is no measure of the hour. A slew from it made the
    // guest's hour run minutes ahead of (or behind) the host's.
    let hour = f64::from(gs.global(order, "GameHour").unwrap_or(snap.hour_f()));
    let err = (f64::from(snap.hour_f()) + ahead_h - hour + 12.0).rem_euclid(24.0) - 12.0;
    let slew = err * f64::from((dt / CLOCK_SLEW_S).min(1.0));
    let hours = (f64::from(dt) * scale / 3600.0 + slew).max(0.0);
    if hours > 0.0 {
        gs.advance_clock(order, (hours * 3600.0 / scale) as f32);
    }
    // The day count is the host's, carried on from its last message, and
    // never goes back. (It is not added up frame by frame: that rounds.)
    let days = days_before.max((target / 24.0) as f32);
    gs.set_global(order, "GameDaysPassed", days);
    if gs.global(order, "TimeScale") != Some(snap.scale_f()) {
        gs.set_global(order, "TimeScale", snap.scale_f());
    }
}

/// Scene bookkeeping for one machine.
#[derive(Default)]
pub struct SceneBridge {
    /// The control-lock scene the host opened for a script's lock.
    ctl: Option<u32>,
    /// Scenes opened for a quest (policy cinematics and endings).
    quest_scenes: std::collections::BTreeMap<u32, FormId>,
    /// The scene whose movie the viewer plays.
    movie_scene: Option<u32>,
    movie_was_busy: bool,
    /// The host's own kept slide.
    last_slide: u16,
    last_ms: u64,
    policy_set: bool,
    /// Fallback: movies the scene layer could not take, played at once.
    pub fallback_movies: u32,
}

impl SceneBridge {
    /// A fresh bridge.
    pub fn new() -> Self {
        Self::default()
    }

    fn gate_scene(gs: &mut GameState, quest: FormId, on: bool) {
        if let Some(g) = gs.coop.gate.as_mut() {
            g.scene(quest, on);
        }
    }

    fn row_controls(gs: &GameState, quest: Option<FormId>) -> Option<coop_policy::Controls> {
        let q = quest?;
        gs.coop.gate.as_ref()?.table().row(q.0)?.controls.clone()
    }

    fn initiator_of(gs: &GameState, quest: FormId) -> Option<u8> {
        let g: &PolicyGate = gs.coop.gate.as_ref()?;
        g.state_for(gs, quest).initiator
    }

    /// Host: turns the notes of the frame into scenes, merges roamers in,
    /// runs approved time skips once and repeats the clock. Send the result.
    pub fn host_frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut HostLive,
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mut outs = Vec::new();
        if !self.policy_set {
            self.policy_set = true;
            if let Some(g) = gs.coop.gate.as_ref() {
                let mut p = live.scene.policy();
                p.late_join_waits =
                    g.table().policy().late_join.during_scene == coop_policy::LateJoinDuring::Wait;
                live.scene.set_policy(p);
            }
        }
        let members = live.members().to_vec();
        let all = Mask::from_players(&members);
        if std::mem::take(&mut gs.coop.live.scene.abort) {
            let o = live.scene.abort_all(now_ms);
            outs.extend(live.route_scene(o));
            self.ctl = None;
        }
        // what host scripts showed and played, for everyone else
        outs.extend(super::presentation::host_frame(gs, live));
        // movies scripts started
        for (quest, movie) in std::mem::take(&mut gs.coop.live.scene.movie_notes) {
            let mut spec = SceneSpec::new(
                SceneKind::Movie,
                quest.map_or_else(|| fnv(&movie.file), |q| q.0),
            );
            spec.locks = Locks::ALL;
            spec.camera = CameraMode::Movie;
            spec.participants = all;
            spec.barrier = BarrierRule::Together;
            spec.skip = if movie.interruptable {
                SkipRule::Unanimous
            } else {
                SkipRule::Never
            };
            spec.movie = Some(movie.clone());
            match live.scene.open(spec, now_ms) {
                Some((id, o)) => {
                    outs.extend(live.route_scene(o));
                    self.movie_scene = Some(id);
                }
                None => {
                    // too many scenes: the host plays it alone rather than not at all
                    self.fallback_movies += 1;
                    gs.coop.live.scene.movie_start = Some(StartMovie {
                        scene: 0,
                        movie,
                        offset_ms: 0,
                    });
                }
            }
        }
        // stages the policy marked as host scenes
        for (quest, kind) in std::mem::take(&mut gs.coop.live.scene.stage_notes) {
            let row_controls = Self::row_controls(gs, Some(quest));
            let mut spec = SceneSpec::new(kind, quest.0);
            let limit = row_controls
                .as_ref()
                .map_or(0, |c| c.timeout_s.saturating_mul(1000));
            match kind {
                SceneKind::Ending => {
                    spec.locks = Locks::ALL;
                    spec.camera = CameraMode::Movie;
                    spec.participants = all;
                    spec.barrier = BarrierRule::Together;
                    spec.skip = SkipRule::Unanimous;
                    spec.timeout_ms = if limit > 0 {
                        limit.max(ENDING_LIMIT_MS)
                    } else {
                        ENDING_LIMIT_MS
                    };
                    // the story shows its slides one at a time and gives no
                    // count at the open: the scene takes the longest list
                    spec.slides = ENDING_SLIDES;
                }
                _ => {
                    spec.locks = row_controls
                        .as_ref()
                        .and_then(|c| c.masks.first())
                        .map_or(Locks::ALL, |m| parse_mask(m));
                    spec.camera = CameraMode::Scripted;
                    spec.barrier = BarrierRule::Roam;
                    spec.skip = SkipRule::HostOnly;
                    spec.timeout_ms = if limit > 0 { limit } else { CINEMATIC_LIMIT_MS };
                    // the players standing where the host stands; the rest roam and merge in
                    let cell = gs.player_cell.map_or(0, |c| c.0);
                    spec.cell = cell;
                    let here: Vec<PlayerId> = members
                        .iter()
                        .copied()
                        .filter(|p| {
                            p.is_host() || live.avatars.state(*p).is_some_and(|s| s.cell.0 == cell)
                        })
                        .collect();
                    spec.participants = Mask::from_players(&here);
                    if matches!(
                        row_controls.as_ref().map(|c| c.scope),
                        Some(coop_policy::ControlScope::Initiator)
                    ) {
                        if let Some(i) = Self::initiator_of(gs, quest) {
                            spec.participants = Mask::of(PlayerId(i));
                        }
                    }
                }
            }
            if let Some((id, o)) = live.scene.open(spec, now_ms) {
                outs.extend(live.route_scene(o));
                self.quest_scenes.insert(id, quest);
                Self::gate_scene(gs, quest, true);
            }
        }
        // the story's slideshow
        for idx in std::mem::take(&mut gs.coop.live.scene.slide_notes) {
            if let Some(id) = self.ending_id(live) {
                {
                    let o = live.scene.slide(id, idx);
                    outs.extend(live.route_scene(o));
                }
            }
        }
        if std::mem::take(&mut gs.coop.live.scene.ending_done) {
            if let Some(id) = self.ending_id(live) {
                {
                    let o = live.scene.end(id, EndReason::Finished, now_ms);
                    outs.extend(live.route_scene(o));
                }
            }
        }
        // a script's control lock
        if let Some(quest) = gs.coop.live.scene.controls_dirty.take() {
            outs.extend(self.control_scene(gs, live, quest, all, now_ms));
        }
        // roamers who arrive in a scene's cell merge in
        for id in live.scene.ids() {
            let Some(info) = live.scene.scene(id, now_ms) else {
                continue;
            };
            if info.spec.cell == 0
                || info.spec.barrier != BarrierRule::Roam
                || info.phase != Phase::Playing
            {
                continue;
            }
            for p in &members {
                let at = if p.is_host() {
                    gs.player_cell.map(|c| c.0)
                } else {
                    live.avatars.state(*p).map(|s| s.cell.0)
                };
                if at == Some(info.spec.cell) && !info.participants.contains(*p) {
                    let o = live.scene.merge(id, *p, now_ms);
                    outs.extend(live.route_scene(o));
                }
            }
        }
        // time the party approved runs once, here
        for (sleeping, hours) in std::mem::take(&mut gs.coop.live.scene.time_approved) {
            let ran = run_skip(cx.order, gs, hours, sleeping);
            if ran > 0 {
                let snap = clock_of(cx.order, gs);
                {
                    let o = live.scene.time_skip(ran as u16, sleeping, snap, now_ms);
                    outs.extend(live.route_scene(o));
                }
            }
        }
        let snap = clock_of(cx.order, gs);
        {
            let o = live.scene.clock(snap, now_ms);
            outs.extend(live.route_scene(o));
        }
        self.after_events(gs, live);
        outs
    }

    fn ending_id(&self, live: &HostLive) -> Option<u32> {
        live.scene.ids().into_iter().find(|id| {
            self.quest_scenes.contains_key(id)
                && live
                    .scene
                    .scene(*id, 0)
                    .is_some_and(|s| s.spec.kind == SceneKind::Ending)
        })
    }

    fn control_scene(
        &mut self,
        gs: &mut GameState,
        live: &mut HostLive,
        quest: Option<FormId>,
        all: Mask,
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mask = Locks::from_flags(&gs.coop.live.scene.script_controls.unwrap_or([false; 7]));
        let mut outs = Vec::new();
        if mask.is_none() {
            if let Some(id) = self.ctl.take() {
                {
                    let o = live.scene.end(id, EndReason::Finished, now_ms);
                    outs.extend(live.route_scene(o));
                }
            }
            return outs;
        }
        if let Some(id) = self.ctl {
            if live.scene.scene(id, now_ms).is_some() {
                {
                    let o = live.scene.set_locks(id, mask);
                    outs.extend(live.route_scene(o));
                }
                return outs;
            }
        }
        let rc = Self::row_controls(gs, quest);
        let mut spec = SceneSpec::new(SceneKind::Control, quest.map_or(0, |q| q.0));
        spec.locks = mask;
        spec.barrier = BarrierRule::Roam;
        spec.participants = all;
        spec.skip = SkipRule::Never;
        spec.timeout_ms = rc.as_ref().map_or(0, |c| c.timeout_s.saturating_mul(1000));
        if let (Some(c), Some(q)) = (&rc, quest) {
            if c.scope == coop_policy::ControlScope::Initiator {
                if let Some(i) = Self::initiator_of(gs, q) {
                    spec.participants = Mask::of(PlayerId(i));
                }
            }
        }
        if let Some((id, o)) = live.scene.open(spec, now_ms) {
            outs.extend(live.route_scene(o));
            self.ctl = Some(id);
        }
        outs
    }

    fn after_events(&mut self, gs: &mut GameState, live: &mut HostLive) {
        for e in live.scene.take_events() {
            if let SceneEvent::Closed(id, _) = e {
                if let Some(q) = self.quest_scenes.remove(&id) {
                    Self::gate_scene(gs, q, false);
                }
                if self.ctl == Some(id) {
                    self.ctl = None;
                }
            }
        }
    }

    /// Every machine, every frame, after the live messages were delivered:
    /// scene notices become world changes. Returns messages for the host.
    pub fn local_frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        client: &mut SceneClient,
        host: bool,
        now_ms: u64,
    ) -> Vec<Msg> {
        let dt = if self.last_ms == 0 {
            0.0
        } else {
            (now_ms.saturating_sub(self.last_ms)).min(250) as f32 / 1000.0
        };
        self.last_ms = now_ms.max(1);
        client.tick(now_ms);
        let mut out = Vec::new();
        for n in client.take_notices() {
            match n {
                Notice::Opened(_) => {}
                Notice::Started { id, offset_ms } => {
                    let movie = client
                        .views(now_ms)
                        .into_iter()
                        .find(|v| v.id == id)
                        .and_then(|v| v.movie);
                    if let Some(movie) = movie {
                        gs.coop.live.scene.counters.movies += 1;
                        self.movie_scene = Some(id);
                        self.movie_was_busy = false;
                        gs.coop.live.scene.movie_start = Some(StartMovie {
                            scene: id,
                            movie,
                            offset_ms,
                        });
                    }
                }
                Notice::Closed(id, reason) => {
                    let c = &mut gs.coop.live.scene.counters;
                    c.closed += 1;
                    c.last_reason = reason_code(reason);
                    if self.movie_scene == Some(id) {
                        self.movie_scene = None;
                        gs.coop.live.scene.movie_stop = true;
                        gs.coop.live.scene.movie_start = None;
                    }
                }
                Notice::Locked(_) => {}
                Notice::Present(cue) => {
                    if !host {
                        super::presentation::apply(gs, cue);
                    }
                }
                Notice::Slide { index, .. } => {
                    gs.coop.live.scene.slide = Some(index);
                    let c = &mut gs.coop.live.scene.counters;
                    c.slide = c.slide.max(index);
                    self.last_slide = self.last_slide.max(index);
                }
                Notice::TimeSkip {
                    hours,
                    sleeping,
                    clock,
                    ..
                } => {
                    if !host {
                        set_clock(cx.order, gs, &clock);
                        if sleeping && !gs.living.hardcore {
                            crate::living::sleep::rested(cx.order, gs);
                        }
                    }
                    gs.coop.live.scene.time_shown = Some(TimeShown { hours, sleeping });
                    gs.coop.live.scene.counters.skip_hours += u32::from(hours);
                }
            }
        }
        // movie done here (the viewer reports it leaving the screen)
        let busy = gs.coop.live.scene.movie_busy;
        if let Some(id) = self.movie_scene {
            if self.movie_was_busy && !busy {
                client.done(id);
                self.movie_scene = None;
            }
        }
        self.movie_was_busy = busy;
        // skip votes
        if !host {
            gs.coop.live.scene.time_approved.clear();
        }
        let asks = std::mem::take(&mut gs.coop.live.scene.skip_asks);
        if let Some(v) = client.primary(now_ms) {
            for yes in asks {
                client.skip(v.id, yes);
            }
        }
        // waits and sleeps the local player asked for become party votes
        for (sleeping, hours) in std::mem::take(&mut gs.coop.live.scene.time_asks) {
            let kind = if sleeping {
                VoteKind::Sleep
            } else {
                VoteKind::Wait
            };
            out.push(Msg::Rules(coop_core::rules::RulesMsg::VoteRequest {
                kind,
                param: hours,
            }));
        }
        // the view and the locks
        let view = client.primary(now_ms);
        let locks = client.locks();
        {
            let sc = &mut gs.coop.live.scene;
            sc.view = view.clone();
            sc.counters.active = client.count() as u32;
            sc.counters.id = view.as_ref().map_or(0, |v| v.id);
            sc.counters.phase =
                view.as_ref()
                    .map_or(0, |v| if v.phase == Phase::Playing { 2 } else { 1 });
            sc.counters.kind = view.as_ref().map_or(0, |v| kind_code(v.kind));
            sc.counters.locks = locks.0;
            sc.counters.opened = client.opened_total;
            sc.counters.time_skips = client.skips;
            sc.counters.committed = client.committed().len() as u32;
            sc.counters.elapsed_ms = view.as_ref().map_or(0, |v| v.elapsed_ms);
        }
        if host {
            // the host's scripts own its own controls; a script lock that
            // does not include the host (initiator scope) shows none here
            if let (Some(script), Some(ctl)) = (gs.coop.live.scene.script_controls, self.ctl) {
                let mine = client
                    .views(now_ms)
                    .iter()
                    .find(|v| v.id == ctl)
                    .map_or(true, |v| v.participant);
                gs.controls_off = if mine { script } else { [false; 7] };
            }
        } else {
            let want = locks.flags();
            if gs.controls_off != want {
                gs.controls_off = want;
            }
            follow_clock(cx.order, gs, client.clock, now_ms, dt);
        }
        out.extend(client.take_out().into_iter().map(Msg::Scene));
        out
    }
}

/// Runs the hours of an approved wait or sleep, once, on the host. Returns
/// how many passed.
fn run_skip(order: &LoadOrder, gs: &mut GameState, hours: u32, sleeping: bool) -> u32 {
    let hours = hours.clamp(1, MAX_SKIP_HOURS);
    gs.coop.live.scene.time_exec = true;
    crate::living::sleep::begin(gs, hours, sleeping);
    let mut n = 0;
    while gs.living.hours_left > 0 && n < MAX_SKIP_HOURS {
        crate::living::sleep::pass_hour(order, gs);
        // every other player's hardcore needs pass the same hour
        super::campaign::needs::hour(order, gs, sleeping);
        n += 1;
    }
    gs.coop.live.scene.time_exec = false;
    n
}

/// Messages the host's own client sent ([`SceneBridge::local_frame`] with
/// `host`) are fed to the host layer by the adapter, like rules.
pub fn is_scene(m: &Msg) -> bool {
    matches!(
        m,
        Msg::Scene(
            SceneMsg::Done { .. }
                | SceneMsg::SkipVote { .. }
                | SceneMsg::Ready { .. }
                | SceneMsg::Join { .. }
        )
    )
}
