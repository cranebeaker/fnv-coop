//! Using furniture (chairs, benches, beds, wall leans) as the game does it,
//! read from `FalloutNV.exe` with Ghidra (the function addresses are
//! given with each rule; "inferred" marks what was deduced rather than
//! seen in the code). The markers come from the furniture's model
//! (`nif::FurnitureMarker`), the animations from the idle tree
//! (`crate::idles`), their lengths and root movement from the caller
//! ([`Animations`]).
//!
//! Someone heads for a piece of furniture: the nearest free marker the
//! furniture's `MNAM` allows ([`nearest_free`]) is theirs from then on
//! (scripts' `IsCurrentFurnitureRef` is already true; `GetSitting` still
//! 0). They walk to it (arrival within `fAIFurnitureDestinationRadius`,
//! 5), turn to its heading, and within 40 units the sit procedure runs
//! ([`Sitter::update`]): the seated loop is loaded (`GetSitting` 1), the
//! entry animation picked (`GetSitting` 2), the actor put exactly on the
//! marker facing its heading, the entry played, and at its end the
//! heading jumps by the marker's `fFurnitureMarkerNNHeadingDelta`
//! (seated, `GetSitting` 3). Getting up (`Actor::StandUp`) turns back by
//! the delta, plays the exit (`GetSitting` 4) and adds half a turn at the
//! end. Beds go the same way through the sleep states (`GetSleeping`).

use std::f32::consts::{PI, TAU};

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;

const MNAM: FourCC = FourCC::new(b"MNAM");

/// `FURN` `MNAM`: bit `i` lets marker `i` of the model be used; bit 30
/// marks sitting furniture, bit 31 a bed (`00509450`, `005093f0`,
/// `00509420`). Doc Mitchell's chair `SubChairDirtyF` has 0x40000004:
/// only its third marker (number 14, the front) is used.
pub const SIT_FURNITURE: u32 = 0x4000_0000;
pub const BED: u32 = 0x8000_0000;

/// How near the marker someone must be for the sit procedure to start
/// (the double 40.0 at `01035810`, `00904f50`).
pub const SIT_REACH: f32 = 40.0;

/// How near the marker a walk to furniture ends:
/// `fAIFurnitureDestinationRadius` (5, the exe's default).
pub const DESTINATION_RADIUS: f32 = 5.0;

/// A piece of furniture's `MNAM` (0 without one).
pub fn marker_flags(order: &LoadOrder, furniture_base: FormId) -> u32 {
    order
        .get(furniture_base)
        .and_then(|rr| rr.record().ok())
        .and_then(|r| {
            r.get(MNAM)
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_u32(&s.data, 0))
        })
        .unwrap_or(0)
}

/// The sit / sleep state of someone's AI process (a byte at +0x13d), with
/// the game's own names (its string table at `0118c6a8`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SitState {
    #[default]
    Normal,
    LoadSitIdle,
    WantToSit,
    WaitingForSitAnim,
    Sitting,
    WantToStand,
    LoadingSleepIdle,
    WantToSleep,
    WaitingForSleepAnim,
    Sleeping,
    WantToWake,
}

impl SitState {
    /// The state's number (0–10).
    pub fn number(self) -> u8 {
        self as u8
    }

    pub fn name(self) -> &'static str {
        [
            "Normal",
            "Load sit idle",
            "Want to sit",
            "Waiting for sit anim",
            "Sitting",
            "Want to stand",
            "Loading sleep idle",
            "Want to sleep",
            "Waiting for sleep anim",
            "Sleeping",
            "Want to wake",
        ][self as usize]
    }

    /// What `GetSitting` gives (`0059dd90`): 1 loading the seated loop, 2
    /// sitting down (both "want" and "waiting"), 3 seated, 4 getting up,
    /// else 0 (sleep states too).
    pub fn get_sitting(self) -> u8 {
        match self as u8 {
            1 => 1,
            2 | 3 => 2,
            4 => 3,
            5 => 4,
            _ => 0,
        }
    }

    /// What `GetSleeping` gives (`0059dc90`): the same for the sleep
    /// states (6 → 1, 7 and 8 → 2, 9 → 3, 10 → 4).
    pub fn get_sleeping(self) -> u8 {
        match self as u8 {
            6 => 1,
            7 | 8 => 2,
            9 => 3,
            10 => 4,
            _ => 0,
        }
    }

    /// Settled in: seated or asleep.
    pub fn is_settled(self) -> bool {
        matches!(self, SitState::Sitting | SitState::Sleeping)
    }

    fn sleep(self) -> bool {
        self as u8 >= 6
    }
}

/// Bed markers (`005094f0`: numbers 1 to 9) go through the sleep states.
pub fn is_sleep_marker(number: u8) -> bool {
    (1..=9).contains(&number)
}

/// Markers that load a seated loop (`00509510`: 10 to 20, and 26).
pub fn is_sit_marker(number: u8) -> bool {
    (10..=20).contains(&number) || number == 26
}

/// Markers below 21 keep their user once the entry has played; from 21
/// on ("use and leave"), the entry plays and the user is let go
/// (`009213e0`; what those markers are used for in the game's data isn't
/// checked).
pub fn keeps_user(number: u8) -> bool {
    number < 21
}

/// A furniture marker where the furniture stands: the process's copy of
/// it (+0x148).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PlacedMarker {
    /// Its place in the model's list (the `MNAM` bit).
    pub index: u8,
    /// Its marker number (11 chair left … 14 front, 1/2 bed sides…).
    pub number: u8,
    pub position: [f32; 3],
    /// Radians clockwise from north.
    pub heading: f32,
}

/// A piece of furniture's markers where it stands (`005686b0`): each
/// offset turned clockwise by the furniture's heading and scaled, from
/// its position; each heading the furniture's plus the marker's, in
/// [0, 2π), kept in thousandths of a radian as the game stores it
/// (`00c54550`). Only the furniture's heading turns them (furniture
/// stands upright).
pub fn place_markers(
    markers: &[nif::FurnitureMarker],
    position: [f32; 3],
    heading: f32,
    scale: f32,
) -> Vec<PlacedMarker> {
    let (s, c) = heading.sin_cos();
    markers
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let [x, y, z] = m.offset.map(|v| v * scale);
            let h = (heading + m.heading).rem_euclid(TAU);
            PlacedMarker {
                index: i as u8,
                number: m.marker,
                position: [
                    position[0] + x * c + y * s,
                    position[1] - x * s + y * c,
                    position[2] + z,
                ],
                heading: ((h * 1000.0).round() / 1000.0).rem_euclid(TAU),
            }
        })
        .collect()
}

/// The marker someone heading for the furniture takes (`005686b0`): of
/// those whose bit is set in its `MNAM` and that nobody has taken
/// (`taken`, by marker index), the nearest to them (3D distance).
pub fn nearest_free(
    markers: &[PlacedMarker],
    flags: u32,
    taken: impl Fn(u8) -> bool,
    from: [f32; 3],
) -> Option<PlacedMarker> {
    let d = |p: [f32; 3]| (0..3).map(|k| (p[k] - from[k]).powi(2)).sum::<f32>();
    markers
        .iter()
        .filter(|m| m.index < 30 && flags & (1 << m.index) != 0 && !taken(m.index))
        .min_by(|a, b| d(a.position).total_cmp(&d(b.position)))
        .copied()
}

/// The first usable marker nobody has taken (`005682c0`: the sandbox's
/// reservation when it picks a piece of furniture).
pub fn first_free(markers: &[PlacedMarker], flags: u32, taken: impl Fn(u8) -> bool) -> Option<u8> {
    markers
        .iter()
        .map(|m| m.index)
        .find(|&i| i < 30 && flags & (1 << i) != 0 && !taken(i))
}

/// The game's settings for a marker number: `fFurnitureMarkerNNDeltaX`,
/// `DeltaY`, `DeltaZ` and `HeadingDelta` (`005099c0` and the table of
/// setting pointers at `01189fa0`). Missing ones are 0, as the exe's
/// defaults are.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MarkerSettings {
    pub delta: [f32; 3],
    pub heading_delta: f32,
}

impl MarkerSettings {
    pub fn read(order: &LoadOrder, number: u8) -> MarkerSettings {
        let get = |what: &str| {
            crate::scripting::game_setting(order, &format!("fFurnitureMarker{number:02}{what}"))
                .unwrap_or(0.0)
        };
        MarkerSettings {
            delta: [get("DeltaX"), get("DeltaY"), get("DeltaZ")],
            heading_delta: get("HeadingDelta"),
        }
    }
}

/// Where the user of a marker is once settled, and their heading, as the
/// game puts someone already seated (a reload while seated `00927d70`,
/// seated when the 3D loads `00925700`, an instant sit `0088d2f0`): the
/// deltas turned clockwise by the marker's heading from the marker (+y
/// its facing, +x its right), the height delta only × (scale − 1)
/// (`00509920`: so at scale 1 the feet stay at the marker's height), and
/// the heading plus the heading delta. `scale` is the reference's scale ×
/// the person's height.
pub fn seat(marker: &PlacedMarker, settings: &MarkerSettings, scale: f32) -> ([f32; 3], f32) {
    let [dx, dy, dz] = settings.delta;
    let (s, c) = marker.heading.sin_cos();
    let p = marker.position;
    (
        [
            p[0] + dx * c + dy * s,
            p[1] - dx * s + dy * c,
            p[2] + (scale - 1.0) * dz,
        ],
        (marker.heading + settings.heading_delta).rem_euclid(TAU),
    )
}

/// Animation lengths and root movement, from the files (`nif::Sequence`).
pub trait Animations {
    /// How long an animation (a `.kf` under `meshes\`) plays, in seconds.
    fn length(&mut self, model: &str) -> Option<f32>;
    /// How far its accumulation root has moved `time` seconds in, in the
    /// skeleton's axes (+y forward): see `nif::Sequence::root_offset`.
    fn root_offset(&mut self, model: &str, time: f32) -> [f32; 3];
}

/// An entry or exit animation playing.
#[derive(Debug, Clone, PartialEq)]
pub struct Playing {
    pub idle: FormId,
    /// The `.kf`, under `meshes\`.
    pub model: String,
    pub length: f32,
    pub elapsed: f32,
    /// Where the actor stood and faced when it began: its root's movement
    /// is added to that, turned by the heading.
    pub from: [f32; 3],
    pub heading: f32,
}

/// What a step of the sit procedure came to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Still on the way in or out.
    Busy,
    /// Seated (or asleep) now.
    Settled,
    /// Let go of the furniture: up and away, or a use-and-leave marker
    /// used.
    Released,
    /// The idle tree has no entry animation for the marker: the game
    /// gives up ("AI: Missing furniture entry animation…", state back to
    /// normal).
    Failed,
}

/// Someone using a piece of furniture: the sit procedure's state for them.
#[derive(Debug, Clone, PartialEq)]
pub struct Sitter {
    pub state: SitState,
    pub furniture: FormId,
    pub marker: PlacedMarker,
    pub settings: MarkerSettings,
    /// The seated loop ("dynamic idle", the idle tree at `GetSitting` 1)
    /// loaded on the way in: (idle, `.kf`).
    pub dynamic_idle: Option<(FormId, String)>,
    /// The entry or exit animation playing.
    pub playing: Option<Playing>,
    /// Asked to get up (`Actor::StandUp`, which keeps asking until done)
    /// while not yet settled, or while a seated idle is starting.
    pub stand_requested: bool,
    /// Where the actor stands and faces (radians clockwise from north).
    pub position: [f32; 3],
    pub heading: f32,
    /// Set by the step that turned the actor by the marker's heading
    /// delta or half a turn: the procedure sets the animation's
    /// `cSkipNextBlend` there (`004974a0`; `world::animation::Player::
    /// skip_next_blend`), so the entry, exit or seated loop is swapped in
    /// the frame the heading changes, not blended across it. The caller
    /// takes it (and clears it) after each [`Sitter::update`].
    pub skip_next_blend: bool,
    /// The furniture's model has collision (`004b66d0` counts its 3D's
    /// collision objects, asked by the process's `00920fd0`): the caller
    /// sets it, from the model. See [`Sitter::others_pass_through`].
    pub furniture_collides: bool,
}

/// The idle tree's answer for someone using furniture, asked with
/// `GetSitting`, `GetSleeping` and `GetFurnitureMarkerID` set: (idle,
/// `.kf`).
pub type PickIdle<'a> = dyn FnMut(u8, u8, u8) -> Option<(FormId, String)> + 'a;

impl Sitter {
    /// Someone heading for `marker` of `furniture`, from where they are.
    pub fn new(
        furniture: FormId,
        marker: PlacedMarker,
        settings: MarkerSettings,
        position: [f32; 3],
        heading: f32,
    ) -> Sitter {
        Sitter {
            state: SitState::Normal,
            furniture,
            marker,
            settings,
            dynamic_idle: None,
            playing: None,
            stand_requested: false,
            position,
            heading,
            skip_next_blend: false,
            furniture_collides: false,
        }
    }

    /// Someone already seated (or asleep): at the seat ([`seat`]),
    /// settled, with the seated loop `pick` gives.
    pub fn seated(
        furniture: FormId,
        marker: PlacedMarker,
        settings: MarkerSettings,
        scale: f32,
        pick: &mut PickIdle,
    ) -> Sitter {
        let (position, heading) = seat(&marker, &settings, scale);
        let sleep = is_sleep_marker(marker.number);
        let loading = if sleep {
            SitState::LoadingSleepIdle
        } else {
            SitState::LoadSitIdle
        };
        Sitter {
            state: if sleep {
                SitState::Sleeping
            } else {
                SitState::Sitting
            },
            furniture,
            marker,
            settings,
            dynamic_idle: pick(loading.get_sitting(), loading.get_sleeping(), marker.number),
            playing: None,
            stand_requested: false,
            position,
            heading,
            skip_next_blend: false,
            furniture_collides: false,
        }
    }

    /// Whether other people's character controllers pass through this
    /// one's (translated from `00920d00`, the process's sit-state setter,
    /// and `00c711d0`, the controller's contact callback; FalloutNV.exe
    /// 1.4.0.525). Entering "want to sit" or "want to sleep" (or seated
    /// straight from "load sit idle", 1 → 4, and 6 → 9), when
    /// `00920fd0` holds — a sit marker (`00509510`: 10–20, 26) or a bed
    /// marker (`005094f0`: under 10) on furniture whose model has
    /// collision — the setter sets flag 0x08000000 on the actor's
    /// controller (+0x410 → +0x414, through `00629670`); "want to stand",
    /// "want to wake" or back to normal clears it. A controller meeting a
    /// character (layer 30) whose controller has that flag drops the
    /// contact (its plane's normal and velocity zeroed). So from sitting
    /// down until getting up begins, a sitter doesn't block anyone; while
    /// getting up they do again.
    pub fn others_pass_through(&self) -> bool {
        use SitState::*;
        matches!(
            self.state,
            WantToSit | WaitingForSitAnim | Sitting | WantToSleep | WaitingForSleepAnim | Sleeping
        ) && self.furniture_collides
            && (is_sit_marker(self.marker.number) || self.marker.number < 10)
    }

    /// Whether they've reached the marker: within [`SIT_REACH`].
    pub fn in_reach(&self, at: [f32; 3]) -> bool {
        let d: f32 = (0..3)
            .map(|k| (self.marker.position[k] - at[k]).powi(2))
            .sum();
        d.sqrt() < SIT_REACH
    }

    /// Asks them to get up (`Actor::StandUp`, `008a75a0`, then the stand
    /// update `00921e80` each frame): from seated, the exit plays; while
    /// still sitting down, once seated; a use-and-leave marker's entry is
    /// left to finish.
    pub fn stand_up(&mut self) {
        self.stand_requested = true;
    }

    /// One frame of the sit or stand procedure, `dt` seconds:
    /// `009213e0` on the way in, `00921e80` on the way out. `seated_idle`
    /// says a seated idle (relaxing, talking) is still starting: not yet
    /// loaded or still blending in (`00498f80`,
    /// `world::animation::Player::idle_starting`). Getting up waits only
    /// for that; an idle already playing at full weight doesn't hold it, as
    /// the exit replaces it (`00497ca0` stops it with its blend-out).
    pub fn update(
        &mut self,
        dt: f32,
        seated_idle: bool,
        pick: &mut PickIdle,
        anims: &mut dyn Animations,
    ) -> Step {
        let number = self.marker.number;
        let sleep = is_sleep_marker(number);
        match self.state {
            SitState::Normal => {
                // Load the seated loop (skipped for markers that have none).
                let loading = if sleep {
                    SitState::LoadingSleepIdle
                } else {
                    SitState::LoadSitIdle
                };
                self.state = loading;
                if sleep || is_sit_marker(number) {
                    self.dynamic_idle = pick(loading.get_sitting(), loading.get_sleeping(), number);
                }
                // Want to sit: the entry animation.
                let want = if sleep {
                    SitState::WantToSleep
                } else {
                    SitState::WantToSit
                };
                self.state = want;
                let Some((idle, model)) = pick(want.get_sitting(), want.get_sleeping(), number)
                else {
                    self.state = SitState::Normal;
                    return Step::Failed;
                };
                // Exactly on the marker, facing its heading, then waiting
                // for the entry to play.
                self.position = self.marker.position;
                self.heading = self.marker.heading;
                self.playing = Some(Playing {
                    length: anims.length(&model).unwrap_or(0.0),
                    idle,
                    model,
                    elapsed: 0.0,
                    from: self.position,
                    heading: self.heading,
                });
                self.state = if sleep {
                    SitState::WaitingForSleepAnim
                } else {
                    SitState::WaitingForSitAnim
                };
                Step::Busy
            }
            SitState::WaitingForSitAnim | SitState::WaitingForSleepAnim => {
                if !self.play(dt, anims) {
                    // A use-and-leave marker asked to get up goes on to
                    // "want to stand" with its entry still playing.
                    if self.stand_requested && !keeps_user(number) {
                        self.state = SitState::WantToStand;
                    }
                    return Step::Busy;
                }
                self.playing = None;
                if !keeps_user(number) {
                    return self.release();
                }
                // Translated from 009213e0 (decompiled, FalloutNV.exe
                // 1.4.0.525), case 3/8 with the entry done and a marker
                // below 21: heading += the marker's delta (`00931d30`),
                // `cSkipNextBlend` (`004974a0`), then the entry is freed
                // (`00498910(1,0)` → `004994f0`, blend 0 with the flag).
                self.heading = (self.heading + self.settings.heading_delta).rem_euclid(TAU);
                self.skip_next_blend = true;
                self.state = if self.state.sleep() {
                    SitState::Sleeping
                } else {
                    SitState::Sitting
                };
                Step::Settled
            }
            SitState::Sitting | SitState::Sleeping => {
                if !self.stand_requested || seated_idle {
                    return Step::Busy;
                }
                let want = if self.state.sleep() {
                    SitState::WantToWake
                } else {
                    SitState::WantToStand
                };
                self.state = want;
                let Some((idle, model)) = pick(want.get_sitting(), want.get_sleeping(), number)
                else {
                    // No exit: up where they are, turned back by the delta
                    // and half a turn; `cSkipNextBlend` and the base loop
                    // cleared at once (`00921e80` case 4/9, `004974a0`,
                    // `00496080(0,0)`).
                    self.heading =
                        (self.heading - self.settings.heading_delta + PI).rem_euclid(TAU);
                    self.skip_next_blend = true;
                    return self.release();
                };
                // Back to the marker's heading for the exit, which starts
                // without a blend: `00921e80` case 5/10 once the queued exit
                // has loaded (heading −= delta, `004974a0`, `00498230`).
                self.heading = (self.heading - self.settings.heading_delta).rem_euclid(TAU);
                self.skip_next_blend = true;
                self.playing = Some(Playing {
                    length: anims.length(&model).unwrap_or(0.0),
                    idle,
                    model,
                    elapsed: 0.0,
                    from: self.position,
                    heading: self.heading,
                });
                Step::Busy
            }
            SitState::WantToStand | SitState::WantToWake => {
                if self.playing.is_some() && !self.play(dt, anims) {
                    return Step::Busy;
                }
                self.playing = None;
                // `00921e80` case 5/10 with the exit done: below marker 21,
                // `cSkipNextBlend` and heading += π, then the animations are
                // picked again (`00895110`), so the standing idle replaces
                // the exit's end pose in the same frame.
                if keeps_user(number) {
                    self.heading = (self.heading + PI).rem_euclid(TAU);
                    self.skip_next_blend = true;
                }
                self.release()
            }
            // Not yet seated: the stand update makes them seated first
            // (`00921e80`'s default case), the sit update carries on.
            SitState::LoadSitIdle
            | SitState::WantToSit
            | SitState::LoadingSleepIdle
            | SitState::WantToSleep => {
                self.state = if sleep {
                    SitState::Sleeping
                } else {
                    SitState::Sitting
                };
                Step::Settled
            }
        }
    }

    /// Plays the entry or exit on: the actor moves with its root (the
    /// accumulated root movement carries them from the marker to the seat
    /// and back: inferred, since nothing else in the procedure moves them
    /// there). Whether it has ended.
    fn play(&mut self, dt: f32, anims: &mut dyn Animations) -> bool {
        let Some(p) = self.playing.as_mut() else {
            return true;
        };
        p.elapsed = (p.elapsed + dt).min(p.length.max(0.0));
        let [x, y, z] = anims.root_offset(&p.model, p.elapsed);
        let (s, c) = p.heading.sin_cos();
        self.position = [
            p.from[0] + x * c + y * s,
            p.from[1] - x * s + y * c,
            p.from[2] + z,
        ];
        p.elapsed >= p.length
    }

    fn release(&mut self) -> Step {
        self.state = SitState::Normal;
        self.playing = None;
        Step::Released
    }

    /// The state values the idle tree's conditions ask
    /// (`GetSitting`, `GetSleeping`, `GetFurnitureMarkerID`).
    pub fn question(&self) -> (u8, u8, u8) {
        (
            self.state.get_sitting(),
            self.state.get_sleeping(),
            self.marker.number,
        )
    }
}

/// The markers of a piece of furniture others are using or heading for
/// (by marker index): taken until they let go (the game's occupied and
/// reserved bits, extra data 0x12 and 0x82 on the furniture).
pub fn taken_markers(
    state: &crate::scripting::GameState,
    furniture: FormId,
    except: FormId,
) -> Vec<u8> {
    state
        .sitters
        .iter()
        .filter(|(who, s)| **who != except && s.furniture == furniture)
        .map(|(_, s)| s.marker.index)
        .collect()
}

/// Whether two references are in the same cell now (`00575ca0`, the
/// parent cell): interiors by their cell; outdoors by the worldspace and
/// the grid square (4096 units) each stands in, since the game keeps an
/// exterior reference in the cell under it (persistent ones included).
pub fn same_cell(
    order: &LoadOrder,
    state: &crate::scripting::GameState,
    a: FormId,
    b: FormId,
) -> bool {
    let (Some((space_a, cell_a, at_a, _)), Some((space_b, cell_b, at_b, _))) =
        (state.place(order, a), state.place(order, b))
    else {
        return false;
    };
    if space_a != cell_a || space_b != cell_b {
        let square = |p: [f32; 3]| ((p[0] / 4096.0).floor(), (p[1] / 4096.0).floor());
        return space_a == space_b && square(at_a) == square(at_b);
    }
    cell_a == cell_b
}

/// The furniture and marker the game puts someone straight into once a
/// cell has loaded (translated from `00972d30`, FalloutNV.exe 1.4.0.525).
/// That pass runs after an interior or exterior cell load (`00453dc0`,
/// `00454450`, `004512c0`) and after the player is moved (`0093cdf0`),
/// over every high-process actor (`00968670`) that isn't disabled or dead,
/// isn't in combat and isn't using furniture (sit state 0). Its current
/// package's target (`00881650`), else the package location's reference
/// (`0067f390`: "near a reference"), is taken when it is furniture (base
/// form type 0x27) in the actor's cell; then the first marker its `MNAM`
/// allows that nobody occupies (`005682c0` with 1: occupied bits only),
/// placed (`00568500`) — and the instant sit `0088d2f0` seats them there
/// ([`Sitter::seated`]). `candidates` are those two in that order;
/// `placed` gives a piece of furniture's markers where it stands. None
/// when the first candidate in the cell has no marker free (the game then
/// doesn't seat them; what it does instead, `006d6f80`, isn't traced).
///
/// Not carried out here: the patrol branch (a patrol package's current
/// point), the linked-reference location (`00569b80`), the process's
/// fallback (+0x514), and the eat and sleep packages' search of the cell
/// for a free chair or bed.
pub fn seat_on_load(
    order: &LoadOrder,
    state: &crate::scripting::GameState,
    who: FormId,
    candidates: &[FormId],
    placed: impl Fn(FormId) -> Option<Vec<PlacedMarker>>,
) -> Option<(FormId, PlacedMarker)> {
    let furniture = candidates.iter().copied().find(|&f| {
        crate::scripting::GameState::is_furniture(order, f) && same_cell(order, state, who, f)
    })?;
    let base = crate::scripting::base_of(order, furniture)?;
    let flags = marker_flags(order, base);
    let markers = placed(furniture)?;
    // Occupied: marked by the sit update or an instant sit (`00568020`),
    // so whoever has begun sitting there; someone still on the way only
    // reserves it, which this pass doesn't ask.
    let occupied: Vec<u8> = state
        .sitters
        .iter()
        .filter(|(o, s)| **o != who && s.furniture == furniture && s.state != SitState::Normal)
        .map(|(_, s)| s.marker.index)
        .collect();
    let index = first_free(&markers, flags, |i| occupied.contains(&i))?;
    let marker = markers.iter().find(|m| m.index == index).copied()?;
    Some((furniture, marker))
}

/// What activating a piece of furniture does (`TESFurniture::Activate`,
/// Xbox PDB name; PC `005095b0`, slot 73 of the `TESFurniture` vtable at
/// `01026d0c`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Activation {
    /// The activator is already in furniture (its sit/sleep state isn't
    /// 0): it gets up (`Actor::StandUp`, actor vfunc +0x418; for the player
    /// `PlayerCharacter::InitiateGetUpPackage`, `00953ee0`), after a
    /// temporary third-person view begins ([`TempThirdPerson::begin`]).
    StandUp,
    /// No usable marker free (`005686b0` failed): the activation fails.
    NoMarker,
    /// A bed marker (numbers 1–9): the bed's sleep path (ownership and the
    /// player's sleep checks, then the sleep menu).
    Sleep(PlacedMarker),
    /// Any other marker: the player gets a temporary third-person view
    /// (`00950340(1)`) and a package to use the furniture
    /// (`PlayerCharacter::InitiateSitSleepPackage`, Xbox PDB name; PC
    /// `00953d80`: a travel package, type 6, with the furniture as its
    /// target, which runs the sit procedure [`player_approach`] /
    /// [`Sitter::update`]).
    Sit(PlacedMarker),
}

/// What activating furniture does for an actor in sit state `state`,
/// `marker` being the nearest free usable marker ([`nearest_free`]).
// Translated from 005095b0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn activate(state: SitState, marker: Option<PlacedMarker>) -> Activation {
    if state != SitState::Normal {
        return Activation::StandUp;
    }
    let Some(marker) = marker else {
        return Activation::NoMarker;
    };
    // 005094f0: marker numbers 1-9 are beds.
    if is_sleep_marker(marker.number) {
        Activation::Sleep(marker)
    } else {
        Activation::Sit(marker)
    }
}

/// The player's approach to a marker (the sit procedure's player branch):
/// 40 units or more from the marker, the player is put on it at once
/// (`SetPos` to the process's marker copy, `SetAngleZ` to its heading)
/// rather than walking there as people do; within 40 the sit update
/// ([`Sitter::update`]) runs. True when they were put there.
// Translated from 00904f50 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn player_approach(sitter: &mut Sitter) -> bool {
    if sitter.state != SitState::Normal || sitter.in_reach(sitter.position) {
        return false;
    }
    sitter.position = sitter.marker.position;
    sitter.heading = sitter.marker.heading;
    true
}

/// Whether the player's activate key is refused for the sit state alone:
/// in states 1, 2, 3, 5, 6, 7, 8 and 10 (sitting down, getting up, the
/// sleep equivalents), not when up (0), seated (4) or asleep (9). The
/// player update tests `GetSitSleepState() - 1` against the byte table at
/// `00944228` (jump table `00944220`) and then refuses the key, as it also
/// does while an animation action plays (`008a7570() != -1`, `00940605`).
// Translated from 009405b6..009405f0 (disassembled, FalloutNV.exe 1.4.0.525)
pub fn player_activation_blocked(state: SitState) -> bool {
    const TABLE: [u8; 10] = [0, 0, 0, 1, 0, 0, 0, 0, 1, 0];
    match state.number() {
        0 => false,
        n => TABLE[usize::from(n - 1)] == 0,
    }
}

/// The player's temporary third-person view while using furniture
/// (`PlayerCharacter` +0x64d and +0x64e: `bTemp3rdPerson` and
/// `bTemp3rdPersonSwitchBack`, Xbox PDB names at +0x65d/+0x65e).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TempThirdPerson {
    pub active: bool,
    /// Begun from first person: first person comes back when it ends.
    pub switch_back: bool,
}

impl TempThirdPerson {
    /// Begins it (`00950340`, called with 1 by `TESFurniture::Activate`
    /// before the sit package or the stand-up). Nothing while a temporary
    /// view is already on (`005721e0`: +0x64f `bTemp1stPerson` or +0x64d).
    /// True when the view must switch to third person now (the player was
    /// in first person, `+0x64a == 0`).
    // Translated from 00950340 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn begin(&mut self, third_person: bool, temp_first_person: bool) -> bool {
        if temp_first_person || self.active {
            return false;
        }
        self.active = true;
        if third_person {
            return false;
        }
        self.switch_back = true;
        true
    }

    /// Each frame of the player update (`00941d83`): it ends once no
    /// animation action plays (`GetAnimAction() == -1`, process vfunc
    /// +0x3e4), the player isn't knocked (`GetKnockState() == 0`, +0x40c)
    /// and the view key isn't being held (`011e07b8`/`011e07b9`). True when
    /// the view must switch back to first person now.
    // Translated from 009503d0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn update(&mut self, action_playing: bool, knocked: bool, pov_key_held: bool) -> bool {
        if !self.active || action_playing || knocked || pov_key_held {
            return false;
        }
        self.active = false;
        std::mem::take(&mut self.switch_back)
    }
}

/// The pitch limits (`Actor::SetAngleX` path): below the double at
/// `0108a7f0` (-1.5533430576324463) the pitch is set to the float at
/// `0108a7ec` (-1.553343); above the float at `0108a7f8` (1.553343) it's
/// held there, except for an actor in sit states 1–5 (sitting down,
/// seated, getting up), whose limit is `fSittingMaxLookingDown` degrees
/// (× the double `01023128`, π/180). Pitch is the game's: positive looks
/// down.
// Translated from 00931d90 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn clamp_pitch(pitch: f32, state: SitState, sitting_max_looking_down_degrees: f32) -> f32 {
    const LOWEST: f64 = -1.553_343_057_632_446_3;
    const LOWEST_SET: f32 = -1.553_343;
    let highest = if (1..6).contains(&state.number()) {
        (f64::from(sitting_max_looking_down_degrees) * 0.017_453_292_384_743_7) as f32
    } else {
        1.553_343
    };
    if f64::from(pitch) < LOWEST {
        LOWEST_SET
    } else if pitch <= highest {
        pitch
    } else {
        highest
    }
}

/// A form list's entries (`FLST` `LNAM`s), or the form itself.
pub fn list_or_one(order: &LoadOrder, id: FormId) -> Vec<FormId> {
    let Some(rr) = order.get(id) else {
        return vec![id];
    };
    if rr.entry.header.kind.as_bytes() != b"FLST" {
        return vec![id];
    }
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .get_all(FourCC::new(b"LNAM"))
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doc Mitchell's chair's front marker (`SubChairDirty01.nif`: number
    /// 14 at (-2.0, 62.8), heading 3.141) and the chair settings.
    fn front() -> nif::FurnitureMarker {
        nif::FurnitureMarker {
            offset: [-2.0, 62.8, -37.2],
            heading: 3141.0 / 1000.0,
            marker: 14,
        }
    }

    fn chair14() -> MarkerSettings {
        MarkerSettings {
            delta: [2.4809, 57.3572, -28.948],
            heading_delta: PI,
        }
    }

    struct Anims;
    impl Animations for Anims {
        fn length(&mut self, model: &str) -> Option<f32> {
            Some(if model.contains("Enter") { 1.73 } else { 1.53 })
        }
        // The front entry walks 55.4 forward, the exit 55.4 back, evenly.
        fn root_offset(&mut self, model: &str, time: f32) -> [f32; 3] {
            if model.contains("Enter") {
                [0.0, 55.4 * (time / 1.73).min(1.0), 0.0]
            } else {
                [0.0, -55.4 * (time / 1.53).min(1.0), 0.0]
            }
        }
    }

    fn pick(sitting: u8, _sleeping: u8, marker: u8) -> Option<(FormId, String)> {
        assert_eq!(marker, 14);
        match sitting {
            1 => Some((FormId(1), "DynamicIdle_ChairSit.kf".into())),
            2 => Some((FormId(2), "Chair_ForwardEnter.kf".into())),
            4 => Some((FormId(3), "Chair_ForwardExit.kf".into())),
            _ => None,
        }
    }

    #[test]
    fn get_sitting_and_get_sleeping_map_the_states_as_the_game_does() {
        let all = [
            SitState::Normal,
            SitState::LoadSitIdle,
            SitState::WantToSit,
            SitState::WaitingForSitAnim,
            SitState::Sitting,
            SitState::WantToStand,
            SitState::LoadingSleepIdle,
            SitState::WantToSleep,
            SitState::WaitingForSleepAnim,
            SitState::Sleeping,
            SitState::WantToWake,
        ];
        let sitting: Vec<u8> = all.iter().map(|s| s.get_sitting()).collect();
        let sleeping: Vec<u8> = all.iter().map(|s| s.get_sleeping()).collect();
        assert_eq!(sitting, [0, 1, 2, 2, 3, 4, 0, 0, 0, 0, 0]);
        assert_eq!(sleeping, [0, 0, 0, 0, 0, 0, 1, 2, 2, 3, 4]);
        assert_eq!(SitState::WaitingForSleepAnim.number(), 8);
        assert_eq!(SitState::Sitting.name(), "Sitting");
    }

    #[test]
    fn only_the_markers_mnam_allows_are_used_nearest_first() {
        // A chair turned a quarter clockwise (east), with left (11), right
        // (12) and front (14) markers; Doc's MNAM allows only the third.
        let markers = [
            nif::FurnitureMarker {
                offset: [-53.7, 7.5, -35.6],
                heading: 1.570,
                marker: 11,
            },
            nif::FurnitureMarker {
                offset: [49.0, 7.6, -35.6],
                heading: 4.712,
                marker: 12,
            },
            front(),
        ];
        let placed = place_markers(&markers, [100.0, 0.0, 0.0], PI / 2.0, 1.0);
        // The front marker 62.8 along the chair's facing (east) from it.
        let f = placed[2];
        assert!((f.position[0] - 162.8).abs() < 0.01 && (f.position[1] - 2.0).abs() < 0.01);
        assert!((f.heading - (PI / 2.0 + 3141.0 / 1000.0).rem_euclid(TAU)).abs() < 1e-3);
        let doc = 0x4000_0004;
        let from_left = [0.0, 0.0, 0.0];
        assert_eq!(
            nearest_free(&placed, doc, |_| false, from_left)
                .unwrap()
                .index,
            2
        );
        // All three allowed: the nearest.
        let all = 0x4000_0007;
        let near_right = placed[1].position;
        assert_eq!(
            nearest_free(&placed, all, |_| false, near_right)
                .unwrap()
                .number,
            12
        );
        assert!(nearest_free(&placed, doc, |i| i == 2, from_left).is_none());
        assert_eq!(first_free(&placed, all, |i| i == 0), Some(1));
    }

    #[test]
    fn the_seat_follows_the_marker_and_scale_only_moves_it_down_past_1() {
        let placed = place_markers(&[front()], [0.0, 0.0, 0.0], 0.0, 1.0)[0];
        let (p, h) = seat(&placed, &chair14(), 1.0);
        // Doc's chair: (-4.5, 5.4) at the marker's height, facing out.
        assert!(
            (p[0] + 4.45).abs() < 0.05 && (p[1] - 5.44).abs() < 0.05,
            "{p:?}"
        );
        assert!((p[2] + 37.2).abs() < 1e-4);
        assert!(h.min(TAU - h) < 0.01, "{h}");
        let (tall, _) = seat(&placed, &chair14(), 1.1);
        assert!((tall[2] - (-37.2 - 2.8948)).abs() < 1e-3, "{tall:?}");
    }

    #[test]
    fn sitting_down_plays_the_entry_from_the_marker_then_turns() {
        let placed = place_markers(&[front()], [0.0, 0.0, 0.0], 0.0, 1.0)[0];
        let mut s = Sitter::new(FormId(9), placed, chair14(), [3.0, 80.0, -37.2], 0.0);
        assert!(s.in_reach([3.0, 80.0, -37.2]));
        assert!(!s.in_reach([3.0, 140.0, -37.2]));
        let mut anims = Anims;
        let step = s.update(0.0, false, &mut pick, &mut anims);
        assert_eq!(step, Step::Busy);
        assert_eq!(s.state, SitState::WaitingForSitAnim);
        assert_eq!(s.question(), (2, 0, 14));
        assert_eq!(
            s.dynamic_idle.as_ref().unwrap().1,
            "DynamicIdle_ChairSit.kf"
        );
        // Snapped onto the marker, facing it (into the chair).
        assert_eq!(s.position, placed.position);
        assert!((s.heading - placed.heading).abs() < 1e-6);
        // Halfway: halfway along its facing.
        s.update(1.73 / 2.0, false, &mut pick, &mut anims);
        assert!(
            (s.position[1] - (62.8 - 27.7)).abs() < 0.1,
            "{:?}",
            s.position
        );
        assert_eq!(s.state.get_sitting(), 2);
        // No heading change yet, no blend skipped.
        assert!(!s.skip_next_blend);
        let step = s.update(1.0, false, &mut pick, &mut anims);
        assert_eq!(step, Step::Settled);
        assert_eq!(s.state, SitState::Sitting);
        assert_eq!(s.question().0, 3);
        // The heading turned by the delta: the entry gives way to the
        // seated loop without a blend (the report's "jump" was the entry
        // easing out over its 0.5 s across the half turn).
        assert!(std::mem::take(&mut s.skip_next_blend));
        // Ends 55.4 in, a few units from the settings' seat (inferred
        // root movement), and turned half a turn: facing out.
        assert!((s.position[1] - 7.4).abs() < 0.1, "{:?}", s.position);
        assert!(s.heading.min(TAU - s.heading) < 0.01, "{}", s.heading);
        // Standing waits for a seated idle to end.
        s.stand_up();
        assert_eq!(s.update(0.1, true, &mut pick, &mut anims), Step::Busy);
        assert_eq!(s.state, SitState::Sitting);
        assert!(!s.skip_next_blend);
        s.update(0.0, false, &mut pick, &mut anims);
        assert_eq!(s.state, SitState::WantToStand);
        assert_eq!(s.question().0, 4);
        // Turned back to the marker's heading for the exit, which cuts in.
        assert!((s.heading - placed.heading).abs() < 1e-3);
        assert!(std::mem::take(&mut s.skip_next_blend));
        s.update(0.5, false, &mut pick, &mut anims);
        assert!(!s.skip_next_blend);
        let step = s.update(2.0, false, &mut pick, &mut anims);
        assert_eq!(step, Step::Released);
        assert_eq!(s.state, SitState::Normal);
        assert!(s.skip_next_blend);
        // Back at the marker, facing away from the chair.
        assert!((s.position[1] - 62.8).abs() < 0.1, "{:?}", s.position);
        assert!(s.heading.min(TAU - s.heading) < 0.01, "{}", s.heading);
    }

    #[test]
    fn no_entry_animation_gives_up_and_no_exit_pops_up_in_place() {
        let placed = place_markers(&[front()], [0.0, 0.0, 0.0], 0.0, 1.0)[0];
        let mut s = Sitter::new(FormId(9), placed, chair14(), placed.position, 0.0);
        let mut none = |_: u8, _: u8, _: u8| None;
        assert_eq!(s.update(0.0, false, &mut none, &mut Anims), Step::Failed);
        assert_eq!(s.state, SitState::Normal);
        let mut seated = Sitter::seated(FormId(9), placed, chair14(), 1.0, &mut pick);
        assert_eq!(seated.state, SitState::Sitting);
        assert_eq!(seated.dynamic_idle.as_ref().unwrap().0, FormId(1));
        let at = seated.position;
        seated.stand_up();
        assert_eq!(
            seated.update(0.0, false, &mut none, &mut Anims),
            Step::Released
        );
        assert_eq!(seated.position, at);
        // Heading: seated (2π) less the delta, plus half a turn.
        assert!(seated.heading.min(TAU - seated.heading) < 0.01);
    }

    #[test]
    fn a_stand_request_while_sitting_down_waits_until_seated() {
        let placed = place_markers(&[front()], [0.0, 0.0, 0.0], 0.0, 1.0)[0];
        let mut s = Sitter::new(FormId(9), placed, chair14(), placed.position, 0.0);
        s.update(0.0, false, &mut pick, &mut Anims);
        s.stand_up();
        assert_eq!(s.update(0.5, false, &mut pick, &mut Anims), Step::Busy);
        assert_eq!(s.state, SitState::WaitingForSitAnim);
        assert_eq!(s.update(2.0, false, &mut pick, &mut Anims), Step::Settled);
        s.update(0.0, false, &mut pick, &mut Anims);
        assert_eq!(s.state, SitState::WantToStand);
    }

    #[test]
    fn activating_furniture_follows_tesfurniture_activate() {
        let placed = place_markers(&[front()], [0.0, 0.0, 0.0], 0.0, 1.0)[0];
        let bed = PlacedMarker {
            number: 1,
            ..placed
        };
        assert_eq!(
            activate(SitState::Normal, Some(placed)),
            Activation::Sit(placed)
        );
        assert_eq!(
            activate(SitState::Normal, Some(bed)),
            Activation::Sleep(bed)
        );
        assert_eq!(activate(SitState::Normal, None), Activation::NoMarker);
        // Anyone in furniture, in any state, gets up, whatever is used.
        for s in [
            SitState::LoadSitIdle,
            SitState::WaitingForSitAnim,
            SitState::Sitting,
            SitState::Sleeping,
        ] {
            assert_eq!(activate(s, Some(placed)), Activation::StandUp);
            assert_eq!(activate(s, None), Activation::StandUp);
        }
    }

    #[test]
    fn the_player_is_put_on_a_marker_40_or_more_away() {
        let placed = place_markers(&[front()], [0.0, 0.0, 0.0], 0.0, 1.0)[0];
        let mut far = Sitter::new(FormId(9), placed, chair14(), [300.0, 0.0, 0.0], 1.0);
        assert!(player_approach(&mut far));
        assert_eq!(far.position, placed.position);
        assert_eq!(far.heading, placed.heading);
        // Exactly 40 away counts as far (`!(d < 40)`).
        let at_40 = [
            placed.position[0] + 40.0,
            placed.position[1],
            placed.position[2],
        ];
        let mut edge = Sitter::new(FormId(9), placed, chair14(), at_40, 1.0);
        assert!(player_approach(&mut edge));
        let near_spot = [
            placed.position[0] + 39.0,
            placed.position[1],
            placed.position[2],
        ];
        let mut near = Sitter::new(FormId(9), placed, chair14(), near_spot, 1.0);
        assert!(!player_approach(&mut near));
        assert_eq!(near.position, near_spot);
        // Then the sit update takes them from the marker as for anyone.
        far.update(0.0, false, &mut pick, &mut Anims);
        assert_eq!(far.state, SitState::WaitingForSitAnim);
        assert!(!player_approach(&mut far));
    }

    #[test]
    fn the_activate_key_is_refused_only_between_settled_states() {
        let refused: Vec<u8> = (0..=10u8)
            .filter(|&n| {
                let s = [
                    SitState::Normal,
                    SitState::LoadSitIdle,
                    SitState::WantToSit,
                    SitState::WaitingForSitAnim,
                    SitState::Sitting,
                    SitState::WantToStand,
                    SitState::LoadingSleepIdle,
                    SitState::WantToSleep,
                    SitState::WaitingForSleepAnim,
                    SitState::Sleeping,
                    SitState::WantToWake,
                ][usize::from(n)];
                player_activation_blocked(s)
            })
            .collect();
        assert_eq!(refused, [1, 2, 3, 5, 6, 7, 8, 10]);
    }

    #[test]
    fn the_temporary_third_person_view_lasts_while_an_action_plays() {
        let mut v = TempThirdPerson::default();
        // From first person: switch now, and back when it ends.
        assert!(v.begin(false, false));
        assert!(v.active && v.switch_back);
        // A second begin while on changes nothing.
        assert!(!v.begin(false, false));
        assert!(!v.update(true, false, false));
        assert!(!v.update(false, true, false));
        assert!(!v.update(false, false, true));
        assert!(v.active);
        assert!(v.update(false, false, false));
        assert_eq!(v, TempThirdPerson::default());
        // From third person: nothing to switch either way.
        assert!(!v.begin(true, false));
        assert!(v.active && !v.switch_back);
        assert!(!v.update(false, false, false));
        assert!(!v.active);
        // A temporary first-person view blocks it.
        assert!(!v.begin(false, true));
        assert!(!v.active);
    }

    #[test]
    fn seated_pitch_is_limited_by_fsittingmaxlookingdown() {
        let down = 1.2;
        assert_eq!(clamp_pitch(down, SitState::Normal, 40.0), down);
        let limit = clamp_pitch(down, SitState::Sitting, 40.0);
        assert!((limit - 40f32.to_radians()).abs() < 1e-5, "{limit}");
        for s in [SitState::LoadSitIdle, SitState::WantToStand] {
            assert_eq!(clamp_pitch(down, s, 40.0), limit);
        }
        // Asleep (9) isn't limited; looking up isn't either.
        assert_eq!(clamp_pitch(down, SitState::Sleeping, 40.0), down);
        assert_eq!(clamp_pitch(-1.0, SitState::Sitting, 40.0), -1.0);
        assert_eq!(clamp_pitch(-2.0, SitState::Sitting, 40.0), -1.553_343);
        assert_eq!(clamp_pitch(2.0, SitState::Normal, 40.0), 1.553_343);
    }

    #[test]
    fn beds_go_through_the_sleep_states() {
        let bed = nif::FurnitureMarker {
            offset: [0.0, 0.0, 0.0],
            heading: 0.0,
            marker: 1,
        };
        let placed = place_markers(&[bed], [0.0, 0.0, 0.0], 0.0, 1.0)[0];
        let mut asked = Vec::new();
        let mut pick = |sitting: u8, sleeping: u8, _: u8| {
            asked.push((sitting, sleeping));
            Some((FormId(5), "BedLeft_Enter.kf".to_string()))
        };
        let mut s = Sitter::new(FormId(9), placed, MarkerSettings::default(), [0.0; 3], 0.0);
        s.update(0.0, false, &mut pick, &mut Anims);
        assert_eq!(s.state, SitState::WaitingForSleepAnim);
        s.update(5.0, false, &mut pick, &mut Anims);
        assert_eq!(s.state, SitState::Sleeping);
        assert_eq!(s.question(), (0, 3, 1));
        assert_eq!(asked, [(0, 1), (0, 2)]);
    }
}
