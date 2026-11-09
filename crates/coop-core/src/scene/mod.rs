//! Scenes and world time.
//!
//! A *scene* is anything the host's story decides takes the players over for a
//! while: a locked camera sequence, a script's control lock, a Bink movie, the
//! ending slideshow, a forced dialogue camera, the opening. The host opens and
//! ends scenes; every peer (the host's own game too) mirrors them through a
//! [`SceneClient`] and plays or locks the same thing at the same scene time.
//!
//! * [`SceneHost`]: the authority. Barrier rule (everyone watches together, or
//!   non-participants roam and merge in later), per-player lock sets, skip
//!   rules (host skip or a unanimous vote), late joiner replay, disconnects,
//!   ordered ending slides with the group commitment, and the once-per-event
//!   time skips and the clock.
//! * [`SceneClient`]: what one machine sees and what it should do now.
//!
//! Pure state machines: messages in, addressed messages out, no clocks (the
//! caller passes `now_ms`), no randomness.

mod client;
mod host;

use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

pub use client::{Notice, SceneClient, SceneView};
pub use host::{SOut, SceneEvent, SceneHost, ScenePolicy};

/// Milliseconds between the barrier opening and the scene starting.
pub const GO_LEAD_MS: u32 = 300;
/// Longest the barrier waits for a participant to be ready.
pub const GATHER_TIMEOUT_MS: u64 = 8_000;
/// Scenes open at once; more are refused.
pub const MAX_SCENES: usize = 4;
/// Longest movie file name.
pub const MAX_NAME: usize = 200;
/// Longest slide list.
pub const MAX_SLIDES: usize = 256;
/// Clock broadcast period.
pub const CLOCK_EVERY_MS: u64 = 1_000;

/// A set of player slots (bit n is slot n; slots 0 to 7).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Mask(pub u8);

impl Mask {
    /// Nobody.
    pub const NONE: Mask = Mask(0);

    /// Just this player.
    pub fn of(p: PlayerId) -> Mask {
        if p.0 < 8 {
            Mask(1 << p.0)
        } else {
            Mask(0)
        }
    }

    /// Everyone in the list.
    pub fn from_players(ps: &[PlayerId]) -> Mask {
        ps.iter().fold(Mask::NONE, |m, p| m.with(*p))
    }

    /// True when the player is in the set.
    pub fn contains(self, p: PlayerId) -> bool {
        p.0 < 8 && self.0 & (1 << p.0) != 0
    }

    /// With the player added.
    pub fn with(self, p: PlayerId) -> Mask {
        Mask(self.0 | Mask::of(p).0)
    }

    /// With the player removed.
    pub fn without(self, p: PlayerId) -> Mask {
        Mask(self.0 & !Mask::of(p).0)
    }

    /// Number of players.
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// True when empty.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// True when every player of `self` is in `o`.
    pub fn is_subset_of(self, o: Mask) -> bool {
        self.0 & !o.0 == 0
    }

    /// The players, ascending.
    pub fn players(self) -> Vec<PlayerId> {
        (0..8u8)
            .filter(|s| self.0 & (1 << s) != 0)
            .map(PlayerId)
            .collect()
    }
}

/// The player controls a scene can take away; bit n is the n-th control of
/// `DisablePlayerControls` (movement, Pip-Boy and menus, fighting, view
/// switch, looking, rollover, sneaking).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Locks(pub u8);

impl Locks {
    /// Nothing locked.
    pub const NONE: Locks = Locks(0);
    /// Every control locked.
    pub const ALL: Locks = Locks(0x7F);
    /// Movement.
    pub const MOVEMENT: u8 = 1 << 0;
    /// Pip-Boy and the menus it opens.
    pub const MENUS: u8 = 1 << 1;
    /// Fighting.
    pub const FIGHTING: u8 = 1 << 2;
    /// View switch.
    pub const POV: u8 = 1 << 3;
    /// Looking.
    pub const LOOKING: u8 = 1 << 4;
    /// Rollover (activation prompts).
    pub const ROLLOVER: u8 = 1 << 5;
    /// Sneaking.
    pub const SNEAKING: u8 = 1 << 6;

    /// From the seven flags of a game state (`controls_off`).
    pub fn from_flags(f: &[bool; 7]) -> Locks {
        Locks(
            f.iter()
                .enumerate()
                .fold(0u8, |m, (i, on)| if *on { m | (1 << i) } else { m }),
        )
    }

    /// The seven flags.
    pub fn flags(self) -> [bool; 7] {
        let mut f = [false; 7];
        for (i, x) in f.iter_mut().enumerate() {
            *x = self.0 & (1 << i) != 0;
        }
        f
    }

    /// Both sets together.
    pub fn union(self, o: Locks) -> Locks {
        Locks((self.0 | o.0) & 0x7F)
    }

    /// True when nothing is locked.
    pub fn is_none(self) -> bool {
        self.0 & 0x7F == 0
    }

    /// True when the control bit is locked.
    pub fn has(self, bit: u8) -> bool {
        self.0 & bit != 0
    }
}

/// What kind of takeover this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SceneKind {
    /// A script's control lock with no camera of its own.
    Control,
    /// A scripted camera sequence.
    Cinematic,
    /// A Bink movie a script started.
    Movie,
    /// The ending slideshow.
    Ending,
    /// A forced dialogue camera.
    Dialogue,
    /// The opening sequence.
    Opening,
}

/// Who owns the camera during a scene.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CameraMode {
    /// Everyone keeps their own camera.
    Free,
    /// The scene's scripted camera.
    Scripted,
    /// The dialogue camera.
    Dialogue,
    /// Full-screen movie.
    Movie,
}

/// How the party enters a scene.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarrierRule {
    /// Everyone watches together: the scene starts when every participant is
    /// ready (or the gather times out).
    Together,
    /// The scene starts at once for its participants; everyone else keeps
    /// playing and merges in when [`SceneHost::merge`] adds them.
    Roam,
}

/// Who may end a scene early.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkipRule {
    /// Nobody.
    Never,
    /// The host.
    HostOnly,
    /// Every participant must vote yes.
    Unanimous,
}

/// A movie a script started (`PlayBink`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Movie {
    /// File name as the script gave it.
    pub file: String,
    /// The player may interrupt it.
    pub interruptable: bool,
    /// Game audio is muted while it plays.
    pub mute_audio: bool,
    /// Music is paused while it plays.
    pub pause_music: bool,
    /// Letterboxed fit.
    pub letterbox: bool,
}

/// What the host's story decides about a scene.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneSpec {
    /// Kind.
    pub kind: SceneKind,
    /// Game-defined tag (quest or script form id).
    pub tag: u32,
    /// Who is in it (empty: every present player).
    pub participants: Mask,
    /// What the participants cannot do.
    pub locks: Locks,
    /// Camera owner.
    pub camera: CameraMode,
    /// The movie, for `Movie` scenes.
    pub movie: Option<Movie>,
    /// How the party enters.
    pub barrier: BarrierRule,
    /// Who may skip.
    pub skip: SkipRule,
    /// The scene ends by itself after this long (0: never).
    pub timeout_ms: u32,
    /// Slides of an ending (count; 0 for the others).
    pub slides: u16,
    /// Cell the scene happens in (0: anywhere); roaming players who enter it
    /// merge in.
    pub cell: u32,
}

impl SceneSpec {
    /// A spec with everything off.
    pub fn new(kind: SceneKind, tag: u32) -> SceneSpec {
        SceneSpec {
            kind,
            tag,
            participants: Mask::NONE,
            locks: Locks::NONE,
            camera: CameraMode::Free,
            movie: None,
            barrier: BarrierRule::Together,
            skip: SkipRule::HostOnly,
            timeout_ms: 0,
            slides: 0,
            cell: 0,
        }
    }

    fn is_sane(&self) -> bool {
        self.movie
            .as_ref()
            .map_or(true, |m| m.file.len() <= MAX_NAME)
            && usize::from(self.slides) <= MAX_SLIDES
    }
}

/// Where a scene stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    /// Waiting for the participants (barrier).
    Gather,
    /// Running.
    Playing,
}

/// A scene as every peer knows it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneInfo {
    /// Scene id.
    pub id: u32,
    /// What the host decided.
    pub spec: SceneSpec,
    /// Players in it now.
    pub participants: Mask,
    /// Late joiners held (all controls locked) until it ends.
    pub waiting: Mask,
    /// Locks in force (the spec's, changed by `Lock`).
    pub locks: Locks,
    /// Phase.
    pub phase: Phase,
    /// Scene time already played, for a replay to a late joiner.
    pub elapsed_ms: u32,
    /// Ending slide shown now.
    pub slide: u16,
}

/// Why a scene ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndReason {
    /// The story ended it.
    Finished,
    /// The party skipped it.
    Skipped,
    /// It timed out.
    TimedOut,
    /// The host's game aborted it (or the host left).
    Aborted,
    /// Nobody was left in it.
    Empty,
}

/// The clock the host owns, on the replicated grid.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClockSnap {
    /// `GameHour` bits.
    pub hour: u32,
    /// `GameDaysPassed` bits.
    pub days_passed: u32,
    /// `GameYear`.
    pub year: u16,
    /// `GameMonth` (from 0).
    pub month: u8,
    /// `GameDay`.
    pub day: u8,
    /// `TimeScale` bits.
    pub scale: u32,
    /// Weather the story forced (form id, 0: none).
    pub weather: u32,
}

impl ClockSnap {
    /// `GameHour`.
    pub fn hour_f(&self) -> f32 {
        f32::from_bits(self.hour)
    }
    /// `GameDaysPassed`.
    pub fn days_f(&self) -> f32 {
        f32::from_bits(self.days_passed)
    }
    /// `TimeScale`.
    pub fn scale_f(&self) -> f32 {
        f32::from_bits(self.scale)
    }
}

/// Most image space modifiers the host keeps on for late joiners.
pub const MAX_IMODS: usize = 32;

/// What a host script shows or plays, for every other player to see and
/// hear (G39): the host's game showed it already.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cue {
    /// An image space modifier (a fade, a flash, a blur) on or off.
    ImageSpace {
        /// The modifier (`IMAD` form id).
        form: u32,
        /// On (applied) or off (removed).
        on: bool,
    },
    /// A sound (`SOUN` form id).
    Sound {
        /// The sound.
        form: u32,
    },
    /// A music track (`MUSC` form id).
    Music {
        /// The track.
        form: u32,
    },
}

/// Scene and time messages (all reliable and ordered).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneMsg {
    /// Host: a scene opened.
    Open {
        /// The scene.
        scene: SceneInfo,
    },
    /// Host: the barrier released; the scene starts this long from now.
    Go {
        /// Scene id.
        id: u32,
        /// Milliseconds from the receipt.
        start_in_ms: u32,
    },
    /// Host: a player merged into a running scene.
    Merged {
        /// Scene id.
        id: u32,
        /// Who.
        player: PlayerId,
    },
    /// Host: the locks of a scene changed.
    Lock {
        /// Scene id.
        id: u32,
        /// New lock set.
        locks: Locks,
    },
    /// Host: ending slide to show (ordered; a lower index is stale).
    Slide {
        /// Scene id.
        id: u32,
        /// Slide index.
        index: u16,
    },
    /// Host: the scene is over.
    Close {
        /// Scene id.
        id: u32,
        /// Why.
        reason: EndReason,
    },
    /// Host to a joiner: the scenes in progress and the endings committed.
    Sync {
        /// Scenes with their elapsed time.
        scenes: Vec<SceneInfo>,
        /// Ending tags the party committed to.
        committed: Vec<u32>,
    },
    /// Host: a time skip happened (once per event).
    TimeSkip {
        /// Event id; a peer applies each id once.
        event: u32,
        /// Hours that passed.
        hours: u16,
        /// A sleep (fade to black) rather than a wait.
        sleeping: bool,
        /// The clock after the skip.
        clock: ClockSnap,
    },
    /// Host: the clock.
    Clock {
        /// The clock now.
        clock: ClockSnap,
    },
    /// Player: ready for the barrier.
    Ready {
        /// Scene id.
        id: u32,
    },
    /// Player: finished playing the scene here.
    Done {
        /// Scene id.
        id: u32,
    },
    /// Player: wants to skip (or not).
    SkipVote {
        /// Scene id.
        id: u32,
        /// Yes or no.
        yes: bool,
    },
    /// Player: wants to merge into a running scene (arrived in its cell).
    Join {
        /// Scene id.
        id: u32,
    },
    /// Host: a host script's presentation. A peer shows each event once;
    /// event 0 replays a modifier that stays on (a late joiner, a rejoin).
    Present {
        /// Event id (0: a replay of a standing modifier).
        event: u32,
        /// What to show or play.
        cue: Cue,
    },
}

impl SceneMsg {
    /// Structural validation of a decoded message.
    pub fn is_sane(&self) -> bool {
        match self {
            SceneMsg::Open { scene } => scene.spec.is_sane(),
            SceneMsg::Sync { scenes, committed } => {
                scenes.len() <= MAX_SCENES
                    && committed.len() <= 256
                    && scenes.iter().all(|s| s.spec.is_sane())
            }
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::{decode, encode, Channel, Msg};

    fn info() -> SceneInfo {
        let mut spec = SceneSpec::new(SceneKind::Movie, 7);
        spec.movie = Some(Movie {
            file: "a.bik".into(),
            interruptable: true,
            mute_audio: true,
            pause_music: false,
            letterbox: true,
        });
        SceneInfo {
            id: 3,
            participants: Mask(5),
            waiting: Mask(2),
            locks: Locks::ALL,
            phase: Phase::Playing,
            elapsed_ms: 1234,
            slide: 2,
            spec,
        }
    }

    #[test]
    fn every_message_survives_the_wire() {
        let clock = ClockSnap {
            hour: 9.5f32.to_bits(),
            days_passed: 3.25f32.to_bits(),
            year: 2281,
            month: 9,
            day: 19,
            scale: 30.0f32.to_bits(),
            weather: 0x1234,
        };
        let msgs = vec![
            SceneMsg::Open { scene: info() },
            SceneMsg::Go {
                id: 3,
                start_in_ms: 300,
            },
            SceneMsg::Merged {
                id: 3,
                player: PlayerId(2),
            },
            SceneMsg::Lock {
                id: 3,
                locks: Locks(5),
            },
            SceneMsg::Slide { id: 3, index: 4 },
            SceneMsg::Close {
                id: 3,
                reason: EndReason::Skipped,
            },
            SceneMsg::Sync {
                scenes: vec![info()],
                committed: vec![1, 2],
            },
            SceneMsg::TimeSkip {
                event: 9,
                hours: 8,
                sleeping: true,
                clock,
            },
            SceneMsg::Clock { clock },
            SceneMsg::Ready { id: 3 },
            SceneMsg::Done { id: 3 },
            SceneMsg::SkipVote { id: 3, yes: true },
            SceneMsg::Join { id: 3 },
            SceneMsg::Present {
                event: 4,
                cue: Cue::ImageSpace {
                    form: 0x174411,
                    on: true,
                },
            },
            SceneMsg::Present {
                event: 5,
                cue: Cue::Sound { form: 0x177F9E },
            },
            SceneMsg::Present {
                event: 6,
                cue: Cue::Music { form: 0x171B4A },
            },
        ];
        for m in msgs {
            let msg = Msg::Scene(m);
            assert_eq!(msg.channel(), Channel::ReliableOrdered);
            let bytes = encode(&msg).unwrap();
            assert_eq!(decode(Channel::ReliableOrdered, &bytes).unwrap(), msg);
        }
    }

    #[test]
    fn an_oversize_movie_name_or_scene_list_is_refused() {
        let mut i = info();
        i.spec.movie.as_mut().unwrap().file = "x".repeat(MAX_NAME + 1);
        let bytes = postcard::to_allocvec(&Msg::Scene(SceneMsg::Open { scene: i })).unwrap();
        assert!(decode(Channel::ReliableOrdered, &bytes).is_err());
        let many = SceneMsg::Sync {
            scenes: vec![info(); MAX_SCENES + 1],
            committed: vec![],
        };
        let bytes = postcard::to_allocvec(&Msg::Scene(many)).unwrap();
        assert!(decode(Channel::ReliableOrdered, &bytes).is_err());
    }

    #[test]
    fn locks_and_masks_round_trip_their_flags() {
        let f = [true, false, true, false, true, false, true];
        assert_eq!(Locks::from_flags(&f).flags(), f);
        assert_eq!(
            Mask::from_players(&[PlayerId(0), PlayerId(3)]).players(),
            vec![PlayerId(0), PlayerId(3)]
        );
        assert!(
            !Mask::of(PlayerId(9)).contains(PlayerId(9)),
            "slots past 7 do not exist"
        );
    }
}
