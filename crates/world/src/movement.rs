//! How people turn and walk, and when they stop, read from `FalloutNV.exe`
//! with Ghidra (`%USERPROFILE%\nv-re\findings\ai_rules.md`; the addresses
//! are given with each rule). The viewer measures and moves; the rules are
//! here.
//!
//! - Turning ([`Turn`], [`walking_turn`]): one model, in code, at a fixed
//!   rate; animations only decorate it. People turn
//!   `fCharacterDefaultTurningSpeed` (90) degrees a second, creatures their
//!   record's `TNAM` (0: `fCreatureDefaultTurningSpeed`, 45); in place × the
//!   INI's `[Pathfinding]` scale (1.5, in combat 2.5; creatures 1.25), while
//!   walking up to 3 × the rate, slower when nearly lined up, with the
//!   forward speed cut on sharp turns ([`walking_turn`]).
//! - Paths ([`advance_progress`], [`arrived`]): the point steered at moves
//!   on in tenths of a segment; a walk ends within the request's radius in
//!   2D, its height within 180.
//! - Reaching someone ([`within_distance`], [`location_radius`],
//!   [`target_reach`]): the game's distance rules for travel, dialogue and
//!   activation.
//! - Packages looked at again ([`PackageClock`]): every 20 s and whenever
//!   the game hour changes.
//! - Avoiding each other ([`Avoidance`]).
//! - People out of sight ([`ProcessLevel`], [`walk_polyline`]).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_f32;
use crate::scripting::{base_of, game_setting};

/// One degree in radians, as the game writes it (`0.017453292`): turns
/// smaller than this aren't started (`009e7610`) and count as done
/// (`009e7d70`).
pub const ONE_DEGREE: f32 = 0.017_453_292;

/// A path request's height tolerance for arriving (`006e2420` sets 180.0
/// at request +0x70; `009e8000` compares the goal's height gap with it).
pub const HEIGHT_TOLERANCE: f32 = 180.0;

/// A path request's actor radius (`006e2420`: 35.0 at +0x68); the path
/// follower moves its steering point on within half of it (`009e27e0`
/// copies it to handler +0x18 and half to +0x14).
pub const REQUEST_RADIUS: f32 = 35.0;

/// How far the steering point moves on per step, in path progress
/// (`009e3d50`: point index plus a fraction, +0.1 at a time).
pub const PATH_STEP: f32 = 0.1;

/// The radius `IsWithinDistance` adds for someone without a character
/// controller, and the least it adds for anyone (`008b2f90`).
pub const LEAST_ADDED_RADIUS: f32 = 32.0;

/// How long packages last before they're looked at again, seconds
/// (`008da670` resets process +0x2b4 to 20.0).
pub const PACKAGE_RECHECK_SECONDS: f32 = 20.0;

/// The settings turning, walking and reaching read: game settings (`GMST`
/// where the data sets them, else the exe's defaults) and the INI's
/// `[Pathfinding]` floats (exe defaults; this install sets none).
#[derive(Debug, Clone, PartialEq)]
pub struct MoveSettings {
    /// `fCharacterDefaultTurningSpeed` (90 degrees a second, `008d3290`).
    pub character_turn: f32,
    /// `fCreatureDefaultTurningSpeed` (45, `008d47f0`, when `TNAM` is 0).
    pub creature_turn: f32,
    /// `[Pathfinding] fAITurnSpeedScale` 1.5, `fAICombatTurnSpeedScale`
    /// 2.5, `fCreatureTurnSpeedScale` 1.25, `fCreatureCombatTurnSpeedScale`
    /// 1.25 (`009e7d70`, `009e16e0`).
    pub ai_turn_scale: f32,
    pub ai_combat_turn_scale: f32,
    pub creature_turn_scale: f32,
    pub creature_combat_turn_scale: f32,
    /// `fActorTurnAnimMinTime` (0.3 s): a turn in place lasts at least this
    /// (`009e7610`).
    pub turn_anim_min_time: f32,
    /// `iActorTurnDegree` (100) and `iActorKeepTurnDegree` (10), degrees.
    pub turn_degree: f32,
    pub keep_turn_degree: f32,
    /// `iAIDistanceRadiusMinLocation` (100): the radius when nothing else
    /// gives one (`00678670`, `006787e0`).
    pub min_location_radius: f32,
    /// `fMoveBaseSpeed` (85 in the exe; the data's value wins) and
    /// `fMoveRunMult` (4): walking and running speed for people moved out
    /// of sight (`00647d10`, `00647f00`).
    pub base_speed: f32,
    pub run_mult: f32,
    /// `fMoveOneCrippledLegSpeedMult` 0.85, `fMoveTwoCrippledLegsSpeedMult`
    /// 0.75 (`00647d10`).
    pub one_leg: f32,
    pub two_legs: f32,
    pub avoidance: AvoidanceSettings,
}

impl MoveSettings {
    /// Reads the game settings, and the INI's through `ini(section, key)`.
    pub fn read(order: &LoadOrder, ini: &dyn Fn(&str, &str) -> Option<f32>) -> MoveSettings {
        let g = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
        let p = |key: &str, default: f32| ini("Pathfinding", key).unwrap_or(default);
        MoveSettings {
            character_turn: g("fCharacterDefaultTurningSpeed", 90.0),
            creature_turn: g("fCreatureDefaultTurningSpeed", 45.0),
            ai_turn_scale: p("fAITurnSpeedScale", 1.5),
            ai_combat_turn_scale: p("fAICombatTurnSpeedScale", 2.5),
            creature_turn_scale: p("fCreatureTurnSpeedScale", 1.25),
            creature_combat_turn_scale: p("fCreatureCombatTurnSpeedScale", 1.25),
            turn_anim_min_time: g("fActorTurnAnimMinTime", 0.3),
            turn_degree: g("iActorTurnDegree", 100.0),
            keep_turn_degree: g("iActorKeepTurnDegree", 10.0),
            min_location_radius: g("iAIDistanceRadiusMinLocation", 100.0),
            base_speed: g("fMoveBaseSpeed", 85.0),
            run_mult: g("fMoveRunMult", 4.0),
            one_leg: g("fMoveOneCrippledLegSpeedMult", 0.85),
            two_legs: g("fMoveTwoCrippledLegsSpeedMult", 0.75),
            avoidance: AvoidanceSettings::read(ini),
        }
    }

    /// The exe's defaults, for tests and when nothing is loaded.
    pub fn defaults() -> MoveSettings {
        MoveSettings {
            character_turn: 90.0,
            creature_turn: 45.0,
            ai_turn_scale: 1.5,
            ai_combat_turn_scale: 2.5,
            creature_turn_scale: 1.25,
            creature_combat_turn_scale: 1.25,
            turn_anim_min_time: 0.3,
            turn_degree: 100.0,
            keep_turn_degree: 10.0,
            min_location_radius: 100.0,
            base_speed: 85.0,
            run_mult: 4.0,
            one_leg: 0.85,
            two_legs: 0.75,
            avoidance: AvoidanceSettings::read(&|_, _| None),
        }
    }

    /// The scale on turning in place (`009e7d70`): people 1.5 (2.5 in
    /// combat, actor +0x104), creatures 1.25 either way.
    pub fn in_place_scale(&self, creature: bool, in_combat: bool) -> f32 {
        match (creature, in_combat) {
            (false, false) => self.ai_turn_scale,
            (false, true) => self.ai_combat_turn_scale,
            (true, false) => self.creature_turn_scale,
            (true, true) => self.creature_combat_turn_scale,
        }
    }

    /// Radians a second someone turns in place: their turning speed (degrees
    /// a second, [`turning_speed`]) × the scale.
    pub fn in_place_rate(&self, speed: f32, creature: bool, in_combat: bool) -> f32 {
        speed * ONE_DEGREE * self.in_place_scale(creature, in_combat)
    }
}

const TNAM: FourCC = FourCC::new(b"TNAM");

/// Someone's turning speed, degrees a second (actor vfunc +0x354): people
/// `fCharacterDefaultTurningSpeed` (`008d3290`, 90); creatures their `CREA`
/// record's `TNAM` float (base +0x134, `00821640`; the record writer
/// `005f7c20` names it), 0 → `fCreatureDefaultTurningSpeed` (`008d47f0`,
/// 45). The tutorial gecko `VCG02CrGecko` has 200.
pub fn turning_speed(order: &LoadOrder, actor: FormId, settings: &MoveSettings) -> f32 {
    let Some(base) = base_of(order, actor).and_then(|b| order.get(b)) else {
        return settings.character_turn;
    };
    if base.entry.header.kind.as_bytes() != b"CREA" {
        return settings.character_turn;
    }
    let own = base
        .record()
        .ok()
        .and_then(|r| {
            r.get(TNAM)
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_f32(&s.data, 0))
        })
        .unwrap_or(0.0);
    if own == 0.0 {
        settings.creature_turn
    } else {
        own
    }
}

/// An angle wrapped into −π..π.
pub fn wrap_pi(a: f32) -> f32 {
    use std::f32::consts::{PI, TAU};
    let mut a = a.rem_euclid(TAU);
    if a > PI {
        a -= TAU;
    }
    a
}

/// Which way a turn goes, for the animation (movement flags 0x10 left,
/// 0x20 right).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnSide {
    Left,
    Right,
}

/// A turn in place toward a heading (`009dce80` → `009e0930` →
/// `009e7610` to start, `009e7d70` each frame).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Turn {
    /// The heading wanted, 0..2π.
    pub target: f32,
    /// Turning clockwise (the heading growing).
    pub right: bool,
    /// The turn state (mover +0xdc): set when a turn starts, cleared once
    /// the heading is reached and the least time has gone by.
    pub active: bool,
    /// What's left of `fActorTurnAnimMinTime` (mover +0x94).
    pub hold: f32,
}

impl Turn {
    /// Starts a turn to `target` (`009e7610`): the target wrapped to 0..2π;
    /// nothing when it's at most one degree away; the way round (the
    /// shorter, by the sign of the difference) fixed now; the turn held for
    /// at least `fActorTurnAnimMinTime`. Whether it started.
    pub fn request(&mut self, heading: f32, target: f32, settings: &MoveSettings) -> bool {
        let target = target.rem_euclid(std::f32::consts::TAU);
        let difference = wrap_pi(target - heading);
        if difference.abs() <= ONE_DEGREE {
            return false;
        }
        self.right = difference > 0.0;
        self.active = true;
        self.target = target;
        self.hold = settings.turn_anim_min_time;
        true
    }

    /// One frame of the turn (`009e7d70`): within a degree the heading is
    /// set to the target; else it moves by `rate × dt` the fixed way round,
    /// or, when that would pass the target, is set to it. The turn ends once
    /// the heading is there and its least time is over. The side turned
    /// this frame (none once there), for the turn animation.
    pub fn update(&mut self, heading: &mut f32, dt: f32, rate: f32) -> Option<TurnSide> {
        if !self.active {
            return None;
        }
        let remaining = wrap_pi(self.target - *heading).abs();
        let mut side = None;
        let step = rate * dt;
        if remaining <= ONE_DEGREE || step > remaining {
            *heading = self.target;
            if self.hold <= 0.0 {
                self.active = false;
            }
        } else if self.right {
            *heading += step;
            side = Some(TurnSide::Right);
        } else {
            *heading -= step;
            side = Some(TurnSide::Left);
        }
        self.hold -= dt;
        side
    }

    /// Whether the body should turn to face a point it looks at
    /// (`008a3100`): more than `iActorTurnDegree` × 0.8 (80°) off, or, while
    /// already turning, `iActorKeepTurnDegree` × 0.8 (8°).
    pub fn should_face(&self, heading: f32, toward: f32, settings: &MoveSettings) -> bool {
        let threshold = if self.active {
            settings.keep_turn_degree
        } else {
            settings.turn_degree
        } * 0.8
            * ONE_DEGREE;
        wrap_pi(toward - heading).abs() > threshold
    }
}

/// The heading toward a point, radians clockwise from north.
pub fn heading_to(from: [f32; 3], to: [f32; 3]) -> f32 {
    (to[0] - from[0]).atan2(to[1] - from[1])
}

/// How a walker turned this frame, and what's left of its forward speed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WalkTurn {
    pub heading: f32,
    /// The frame's forward motion is multiplied by this (`009e4800`'s
    /// second output; 1 when it doesn't change it).
    pub forward: f32,
}

/// The walking turn's slowdown near alignment (`009e4800`): for someone
/// on a path whose turning speed is above 45, the angle between their
/// facing and the look-ahead point ÷ 90°, kept within 0.1..1; else 1.
pub fn walking_turn_factor(speed: f32, on_path: bool, angle_to_look_ahead: f32) -> f32 {
    if on_path && speed > 45.0 {
        (wrap_pi(angle_to_look_ahead).abs() / std::f32::consts::FRAC_PI_2).clamp(0.1, 1.0)
    } else {
        1.0
    }
}

/// The look-ahead point for [`walking_turn_factor`] (`009e4800`): the
/// steering point, pushed on along the path by (move speed − distance to
/// it) when nearer than a second of travel.
pub fn look_ahead_point(
    path: &[[f32; 3]],
    progress: f32,
    position: [f32; 3],
    speed: f32,
) -> [f32; 3] {
    let steer = path_point(path, progress);
    let d = flat_distance(position, steer);
    if d < speed {
        along_path(path, progress, speed - d)
    } else {
        steer
    }
}

/// One frame of turning while walking (`009e4800`): toward `desired`, by at
/// most turning speed × dt × 3 × `factor` ([`walking_turn_factor`]). Within
/// a degree the heading is taken as wanted. While the turn left is more
/// than this frame's, forward motion is cut: past 2.7 radians × 0.05, past
/// 2.0 × 0.1, past 1.6 × 0.2. (The code also halves it near the path's
/// end, within a radius of the actor that isn't traced: left out.)
pub fn walking_turn(heading: f32, desired: f32, speed: f32, dt: f32, factor: f32) -> WalkTurn {
    let difference = wrap_pi(desired - heading);
    if difference.abs() < ONE_DEGREE {
        return WalkTurn {
            heading: desired,
            forward: 1.0,
        };
    }
    let step = speed * ONE_DEGREE * dt * 3.0 * factor;
    let a = difference.abs();
    let forward = if step < a {
        if a > 2.7 {
            0.05
        } else if a > 2.0 {
            0.1
        } else if a > 1.6 {
            0.2
        } else {
            1.0
        }
    } else {
        1.0
    };
    WalkTurn {
        heading: heading + difference.clamp(-step, step),
        forward,
    }
}

/// A point along a path by progress (point index + fraction).
pub fn path_point(path: &[[f32; 3]], progress: f32) -> [f32; 3] {
    if path.is_empty() {
        return [0.0; 3];
    }
    let last = path.len() - 1;
    let p = progress.clamp(0.0, last as f32);
    let i = (p.floor() as usize).min(last);
    if i >= last {
        return path[last];
    }
    let f = p - i as f32;
    let (a, b) = (path[i], path[i + 1]);
    [0, 1, 2].map(|k| a[k] + (b[k] - a[k]) * f)
}

/// The point `distance` further along a path from a progress.
fn along_path(path: &[[f32; 3]], progress: f32, distance: f32) -> [f32; 3] {
    if path.len() < 2 {
        return path_point(path, progress);
    }
    let mut at = path_point(path, progress);
    let mut left = distance;
    let mut i = (progress.max(0.0).floor() as usize).min(path.len() - 1);
    while i + 1 < path.len() {
        let next = path[i + 1];
        let d = distance3(at, next);
        if d >= left {
            let f = if d > 0.0 { left / d } else { 0.0 };
            return [0, 1, 2].map(|k| at[k] + (next[k] - at[k]) * f);
        }
        left -= d;
        at = next;
        i += 1;
    }
    at
}

/// Moves the steering point on (`009e3d50`): while the walker has passed
/// it along its segment, or is within max(4 × (this frame's move)²,
/// ([`REQUEST_RADIUS`] ÷ 2)²) of it, the progress grows by
/// [`PATH_STEP`], up to the last point. The new progress.
pub fn advance_progress(
    path: &[[f32; 3]],
    progress: f32,
    position: [f32; 3],
    frame_move: f32,
) -> f32 {
    if path.len() < 2 {
        return progress;
    }
    let last = (path.len() - 1) as f32;
    let near = (4.0 * frame_move * frame_move).max((REQUEST_RADIUS * 0.5).powi(2));
    let mut p = progress.min(last);
    while p < last {
        let s = path_point(path, p);
        let i = (p.floor() as usize).min(path.len() - 2);
        let segment = [0, 1].map(|k| path[i + 1][k] - path[i][k]);
        let to = [s[0] - position[0], s[1] - position[1]];
        let passed = to[0] * segment[0] + to[1] * segment[1] < 0.0;
        let d2 = to[0] * to[0] + to[1] * to[1];
        if !passed && d2 >= near {
            break;
        }
        p = (p + PATH_STEP).min(last);
    }
    p
}

/// Whether a walk is over (`009e8000`): within the request's radius of the
/// goal in 2D (distance² at most max(r², 0.1), `009e0470`), and the height
/// gap within the request's tolerance ([`HEIGHT_TOLERANCE`]).
pub fn arrived(position: [f32; 3], goal: [f32; 3], radius: f32) -> bool {
    let d2 = (goal[0] - position[0]).powi(2) + (goal[1] - position[1]).powi(2);
    d2 <= (radius * radius).max(0.1) && (goal[2] - position[2]).abs() <= HEIGHT_TOLERANCE
}

/// `IsWithinDistance` (`008b2f90`): whether `target` is within `distance`
/// of someone standing at `me`, `height` tall (128 when not known), the
/// distance grown by their radius (`radius`: their character controller's,
/// at least 32; 32 without one) when `add_radius`. In 2D when the target's
/// height is within the person's height ± 30, else 3D. (With a character
/// controller in some states the game always measures in 2D; which states
/// isn't traced.)
pub fn within_distance(
    me: [f32; 3],
    height: f32,
    radius: Option<f32>,
    target: [f32; 3],
    distance: f32,
    add_radius: bool,
) -> bool {
    let height = if height == 0.0 { 128.0 } else { height };
    let mut reach = distance;
    if add_radius {
        reach += radius.map_or(LEAST_ADDED_RADIUS, |r| r.max(LEAST_ADDED_RADIUS));
    }
    let flat = me[2] - 30.0 < target[2] && target[2] < me[2] + height + 30.0;
    let d = if flat {
        flat_distance(me, target)
    } else {
        distance3(me, target)
    };
    d <= reach
}

/// What a package location or target is, for its radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spot {
    /// A piece of furniture.
    Furniture,
    /// An `XMarker` or `XMarkerHeading` (forms 0x3B, 0x34).
    Marker,
    /// Someone; asleep (sit state 9) or not.
    Person { asleep: bool, half_diagonal: f32 },
    /// Anything else placed: its bounds' half diagonal.
    Object { half_diagonal: f32 },
    /// No reference: "near the editor location" (kind 3) or none.
    NoReference { editor_location: bool },
}

/// The game's `XMarkerHeading` and `XMarker` (`0046a370` makes them: forms
/// 0x34 and 0x3B; the AI reads them through `[011ca248]`, `[011ca244]`).
pub const X_MARKER_HEADING: FormId = FormId(0x34);
pub const X_MARKER: FormId = FormId(0x3B);

/// A travel's radius (`00678670`): the package's own (`PLDT`) when set;
/// else by what's there: furniture 10, a marker 20, someone asleep 90,
/// anything else round(half its bounds' diagonal) + 20; "near the editor
/// location" without a reference 10; still nothing → `min` (100).
pub fn location_radius(own: i32, spot: Spot, min: f32) -> f32 {
    if own != 0 {
        return own as f32;
    }
    let r = match spot {
        Spot::Furniture => 10.0,
        Spot::Marker => 20.0,
        Spot::Person { asleep: true, .. } => 90.0,
        Spot::Person { half_diagonal, .. } | Spot::Object { half_diagonal } => {
            (half_diagonal.round() + 20.0).max(0.0)
        }
        Spot::NoReference {
            editor_location: true,
        } => 10.0,
        Spot::NoReference { .. } => 0.0,
    };
    if r == 0.0 {
        min
    } else {
        r
    }
}

/// How near someone comes to whom they walk up to (`006787e0`): furniture
/// 10, a marker 20, else the package's target distance (`PTDT`) when set,
/// else for a reference that isn't someone round(half its bounds' diagonal)
/// (+ 20 more when … the code's height test, not followed), else `min`
/// (`iAIDistanceRadiusMinLocation`, 100): a person target with no distance
/// is reached at 100 (plus the walker's radius, [`within_distance`]).
pub fn target_reach(target_distance: i32, spot: Spot, min: f32) -> f32 {
    let r = match spot {
        Spot::Furniture => 10.0,
        Spot::Marker => 20.0,
        _ if target_distance > 0 => target_distance as f32,
        Spot::Object { half_diagonal } => half_diagonal.round(),
        _ => 0.0,
    };
    if r == 0.0 {
        min
    } else {
        r
    }
}

/// A "Say To" dialogue package's reach (`008e8600`): the target distance,
/// at most 0 → 120.
pub fn say_to_reach(target_distance: i32) -> f32 {
    if target_distance <= 0 {
        120.0
    } else {
        target_distance as f32
    }
}

/// How near the one who starts a conversation comes before it begins
/// (`008b2170`: the conversation package that `StartConversation` and
/// people's own conversations make): its target distance 90, or 200 when
/// they're seated (sit state 4), measured as `IsWithinDistance`
/// ([`within_distance`], adding their radius).
pub fn conversation_reach(seated: bool) -> f32 {
    if seated {
        200.0
    } else {
        90.0
    }
}

/// When someone's packages are looked at again (`008da670`, process
/// vfunc +0x24): when forced (a script's `EvaluatePackage`, the end of a
/// conversation, `00762160`), when they have none, when the timer (process
/// +0x2b4, counted down by the frame's seconds) has run out — it's reset
/// to 20 s — and whenever the game hour, rounded, differs from when they
/// were last looked at. Only in sit states 0, 4 and 9 (the caller's).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PackageClock {
    pub timer: f32,
    pub hour: Option<i32>,
}

impl PackageClock {
    /// Whether to look now, counting the frame's `dt`.
    pub fn due(&mut self, dt: f32, game_hour: f32, forced: bool, has_package: bool) -> bool {
        let hour = game_hour.round() as i32;
        let due = forced || !has_package || self.timer <= 0.0 || self.hour != Some(hour);
        if due {
            self.hour = Some(hour);
            self.timer = PACKAGE_RECHECK_SECONDS;
        }
        self.timer -= dt;
        due
    }
}

/// The settings avoiding others reads (the INI's `[Pathfinding]`, exe
/// defaults; `009e5690`, `009e5ae0`, `009e6900`).
#[derive(Debug, Clone, PartialEq)]
pub struct AvoidanceSettings {
    /// `fAvoidanceTimeCheck` 0.75 s of travel looked ahead.
    pub time_check: f32,
    /// `fAvoidanceConeAngle` 45° walking, `fRunningWiderConeAngle` 30°
    /// running: the half-angle of the cone ahead.
    pub cone: f32,
    pub running_cone: f32,
    /// `fAvoidanceAvoidAllRadius` 100: anyone this near counts.
    pub avoid_all: f32,
    /// `fAvoidanceDetectionTime` 0.5 s blocked before acting.
    pub detection_time: f32,
    /// `fAvoidanceDefaultWaitTime` 5 s, `fAvoidanceMinWaitTime` 0.25 s.
    pub wait: f32,
    pub min_wait: f32,
    /// `fAvoidanceIgnoreTime` 5 s, `fAvoidanceIgnoreMinTime` 2.5 s after a
    /// way round.
    pub ignore: f32,
    pub ignore_min: f32,
    /// `fAvoidanceAvoidNodeCost` 2.
    pub node_cost: f32,
}

impl AvoidanceSettings {
    pub fn read(ini: &dyn Fn(&str, &str) -> Option<f32>) -> AvoidanceSettings {
        let p = |key: &str, default: f32| ini("Pathfinding", key).unwrap_or(default);
        AvoidanceSettings {
            time_check: p("fAvoidanceTimeCheck", 0.75),
            cone: p("fAvoidanceConeAngle", 45.0),
            running_cone: p("fRunningWiderConeAngle", 30.0),
            avoid_all: p("fAvoidanceAvoidAllRadius", 100.0),
            detection_time: p("fAvoidanceDetectionTime", 0.5),
            wait: p("fAvoidanceDefaultWaitTime", 5.0),
            min_wait: p("fAvoidanceMinWaitTime", 0.25),
            ignore: p("fAvoidanceIgnoreTime", 5.0),
            ignore_min: p("fAvoidanceIgnoreMinTime", 2.5),
            node_cost: p("fAvoidanceAvoidNodeCost", 2.0),
        }
    }
}

/// Someone (or the player) near a walker, for [`Avoidance::update`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Obstacle {
    pub who: FormId,
    pub position: [f32; 3],
    /// Units a second.
    pub velocity: [f32; 3],
    pub radius: f32,
    pub is_player: bool,
    /// Seated or asleep (sit state 4 or 9).
    pub seated: bool,
}

/// A place to keep away from when finding a way round (`009e6870`): its
/// middle, radius and cost.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AvoidNode {
    pub position: [f32; 3],
    pub radius: f32,
    pub cost: f32,
}

/// What a walker does about others in the way.
#[derive(Debug, Clone, PartialEq)]
pub enum AvoidStep {
    /// Carry on.
    Clear,
    /// Stand and wait (`fAvoidanceDefaultWaitTime`).
    Wait,
    /// Find a new way round these.
    Repath(Vec<AvoidNode>),
}

/// A walker's avoiding state, kept for one path (each path gets a new
/// path handler: `009dbdc0`): +0x78 the time blocked, +0x98 the wait left,
/// +0x74 the wait's recheck and the time the people gone round are left
/// out, +0xd0 waiting (2) or gone round (4).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Avoidance {
    pub blocked: f32,
    pub waiting: f32,
    pub recheck: f32,
    pub ignore: f32,
    /// Those gone round: left out while `ignore` lasts (`009e6900` skips
    /// those on the avoid list in states 3 and 4).
    pub avoided: Vec<FormId>,
}

/// One blocker found ahead (`009e6900`): whether it's in the way, and how
/// much (the cosine of its angle off the way ahead; squared when it's in
/// the far half of the reach).
fn blocking(
    me: [f32; 3],
    my_radius: f32,
    ahead: [f32; 2],
    look_ahead: f32,
    cos_cone: f32,
    other: &Obstacle,
    settings: &AvoidanceSettings,
) -> Option<f32> {
    let rel = [
        other.position[0] - me[0],
        other.position[1] - me[1],
        other.position[2] - me[2],
    ];
    let reach = my_radius + look_ahead + other.radius;
    let d2 = rel[0] * rel[0] + rel[1] * rel[1] + rel[2] * rel[2];
    if d2 >= reach * reach || rel[2].abs() >= 180.0 {
        return None;
    }
    let flat = (rel[0] * rel[0] + rel[1] * rel[1]).sqrt();
    let close = flat < settings.avoid_all;
    let edge = (flat - other.radius).max(0.0);
    let cos = if flat > 0.0 {
        (rel[0] * ahead[0] + rel[1] * ahead[1]) / flat
    } else {
        1.0
    };
    if cos < cos_cone && !close {
        return None;
    }
    let share = (edge / reach).clamp(0.0, 1.0);
    Some(if share.round() >= 1.0 { cos * cos } else { cos })
}

/// Who blocks a walker now, with each one's share and whether it walks.
#[allow(clippy::too_many_arguments)]
fn blockers<'a>(
    me: [f32; 3],
    my_radius: f32,
    ahead: [f32; 2],
    velocity: [f32; 3],
    running: bool,
    others: &'a [Obstacle],
    skip: &[FormId],
    settings: &AvoidanceSettings,
) -> Vec<(&'a Obstacle, f32, bool)> {
    let cos_cone = (if running {
        settings.running_cone
    } else {
        settings.cone
    })
    .to_radians()
    .cos();
    let own_speed = (velocity[0].powi(2) + velocity[1].powi(2)).sqrt();
    let mut out = Vec::new();
    for o in others {
        if skip.contains(&o.who) {
            continue;
        }
        let rel = [o.position[0] - me[0], o.position[1] - me[1]];
        let d2 = rel[0] * rel[0] + rel[1] * rel[1] + (o.position[2] - me[2]).powi(2);
        if !o.is_player && (d2 > 1.0e6 || o.radius < my_radius * 0.33) {
            continue;
        }
        // Behind and not close: not in the way.
        if !o.is_player
            && d2 > settings.avoid_all * settings.avoid_all
            && rel[0] * velocity[0] + rel[1] * velocity[1] < 0.0
        {
            continue;
        }
        let look = if o.is_player {
            settings.time_check * own_speed
        } else {
            let closing = [
                velocity[0] - o.velocity[0],
                velocity[1] - o.velocity[1],
                velocity[2] - o.velocity[2],
            ];
            let closing = (closing[0].powi(2) + closing[1].powi(2) + closing[2].powi(2)).sqrt();
            (my_radius + settings.time_check * closing).max(settings.avoid_all)
        };
        if let Some(share) = blocking(me, my_radius, ahead, look, cos_cone, o, settings) {
            let moving = (o.velocity[0].powi(2) + o.velocity[1].powi(2)).sqrt() > 1.0;
            out.push((o, share, moving));
        }
    }
    out
}

impl Avoidance {
    /// One frame of keeping out of others' way (`009e5ae0`, with `009e5690`
    /// and `009e6900`), for a walker at `me` heading `ahead` (a unit 2D
    /// vector) at `velocity` units a second, its own radius `my_radius`
    /// (the path request's, [`REQUEST_RADIUS`]), `running`, toward `target`
    /// (whom it walks up to, if anyone). Others count when within 1000,
    /// their radius at least a third of its own, and ahead (or within 100);
    /// they block when inside the reach (both radii + 0.75 s of the
    /// closing speed, at least 100; the player: 0.75 s of its own speed)
    /// within the cone ahead (45°, running 30°) or within 100. Blocked time
    /// builds up by each blocker's share × the frame (× 2 for someone
    /// standing still); once past 0.5 s: waiting (5 s, over sooner once no
    /// one walking is in the way for 0.25 s) when the blocker is whom it
    /// walks up to or someone walking, else a way round them (each standing
    /// blocker and the player an avoid node of its radius × 1.5, running
    /// 1.75, × 1.5 more when within both radii, half for the seated; cost
    /// 2), after which those are left out for 5 s (2.5 s after the player,
    /// or running). The blocked time doesn't wear off between blockers (the
    /// code takes it off only under a test, `0076b610`, that isn't traced).
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        me: [f32; 3],
        my_radius: f32,
        ahead: [f32; 2],
        velocity: [f32; 3],
        running: bool,
        target: Option<FormId>,
        others: &[Obstacle],
        settings: &AvoidanceSettings,
        dt: f32,
    ) -> AvoidStep {
        if self.waiting > 0.0 {
            self.waiting -= dt;
            if self.waiting <= 0.0 {
                self.waiting = 0.0;
                return AvoidStep::Clear;
            }
            // While waiting only someone walking keeps it waiting.
            let found = blockers(
                me,
                my_radius,
                ahead,
                velocity,
                running,
                others,
                &[],
                settings,
            );
            if found.iter().any(|(o, _, moving)| *moving && !o.is_player) {
                self.recheck = settings.min_wait;
            } else {
                self.recheck -= dt;
                if self.recheck <= 0.0 {
                    self.waiting = 0.0;
                    return AvoidStep::Clear;
                }
            }
            return AvoidStep::Wait;
        }
        if self.ignore > 0.0 {
            self.ignore -= dt;
            if self.ignore <= 0.0 {
                self.avoided.clear();
            }
        }
        let found = blockers(
            me,
            my_radius,
            ahead,
            velocity,
            running,
            others,
            &self.avoided,
            settings,
        );
        let mut wait = false;
        let mut player = false;
        let mut round: Vec<&Obstacle> = Vec::new();
        for &(o, share, moving) in &found {
            if o.is_player {
                self.blocked += share * dt;
                player = true;
                round.push(o);
            } else if !moving {
                self.blocked += share * 2.0 * dt;
                if target == Some(o.who) {
                    wait = true;
                }
                round.push(o);
            } else {
                self.blocked += share * dt;
                if !running {
                    wait = true;
                }
            }
        }
        if found.is_empty() || self.blocked <= settings.detection_time {
            return AvoidStep::Clear;
        }
        self.blocked = 0.0;
        if wait {
            self.waiting = settings.wait;
            self.recheck = settings.min_wait;
            return AvoidStep::Wait;
        }
        self.ignore = if player || running {
            settings.ignore_min
        } else {
            settings.ignore
        };
        self.avoided.extend(round.iter().map(|o| o.who));
        let widen = if running { 1.75 } else { 1.5 };
        let nodes = round
            .into_iter()
            .map(|o| {
                let d = flat_distance(me, o.position);
                let near = if d < o.radius + my_radius { 1.5 } else { 1.0 };
                let radius = if o.seated {
                    o.radius * 0.5 * widen
                } else {
                    o.radius * widen * near
                };
                AvoidNode {
                    position: o.position,
                    radius,
                    cost: settings.node_cost,
                }
            })
            .collect();
        AvoidStep::Repath(nodes)
    }
}

/// The share of the last frame's move someone must have made, squared, not
/// to count as blocked (`010924a8`, 0.125, in `009e4cf0`).
pub const STUCK_MOVE_SHARE: f32 = 0.125;
/// Seconds blocked before they're stuck (`01016ff0`, 1.5).
pub const STUCK_SECONDS: f32 = 1.5;
/// Blocked time runs at this rate while their controller is up against
/// someone (`01018a90`, 0.2).
pub const STUCK_AGAINST_SOMEONE: f32 = 0.2;

/// The detailed path handler's stuck test (`009e4cf0`, run each frame
/// before the move, `009e0a00`), for someone walking on the ground: they're
/// blocked when they moved less than √0.125 of the last frame's move (the
/// move vector's length, handler +0x9c; positions compared flat, +0x40);
/// blocked time (+0x88) adds up the frames' seconds (× 0.2 while their
/// controller touches an actor), and goes back to 0 on any frame they're
/// not blocked; past 1.5 s they're stuck (+0xde) for the rest of the path
/// (each path gets a new handler, `009dbdc0`). The game also requires the
/// movement animation to be playing (`00495be0` on actor vfunc +0x1e4,
/// read only in outline) and ends the walk as failed once stuck, while its
/// avoidance state (+0xd0) is 0, 1 or 5, after marking the triangle they
/// stand on (`00691510`: an obstacle record and triangle flag
/// 0x80000000). The flying and swimming branch (3D distances) isn't
/// followed here.
// Translated from 009e4cf0 (decompiled, FalloutNV.exe 1.4.0.525).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Stuck {
    /// Where they stood at the last test.
    pub last: Option<[f32; 3]>,
    /// Seconds blocked.
    pub time: f32,
    /// Stuck.
    pub stuck: bool,
}

impl Stuck {
    /// One frame's test: where they stand now, how long the last frame's
    /// move was, whether their controller touches someone; whether they're
    /// stuck now.
    pub fn update(
        &mut self,
        position: [f32; 3],
        last_move: f32,
        against_someone: bool,
        dt: f32,
    ) -> bool {
        let blocked = self.last.is_some_and(|last| {
            let moved2 = (position[0] - last[0]).powi(2) + (position[1] - last[1]).powi(2);
            moved2 < last_move * last_move * STUCK_MOVE_SHARE
        });
        self.last = Some(position);
        if blocked {
            self.time += if against_someone {
                dt * STUCK_AGAINST_SOMEONE
            } else {
                dt
            };
        } else if !self.stuck {
            self.time = 0.0;
            return false;
        }
        if self.time > STUCK_SECONDS {
            self.stuck = true;
        }
        self.stuck
    }
}

/// How often the game moves someone, by where they are (`009334b0`
/// decides; the lists `0096bcd0`, `0096b810`, `0096b470`, `0096b050` run
/// them): high (their cell attached: every frame), middle-high (their cell
/// loading or detaching), middle-low (their cell in memory, in the cell
/// buffers), low (anywhere else; only references always in memory, the
/// persistent ones, are there to be moved).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessLevel {
    High,
    MiddleHigh,
    MiddleLow,
    Low,
}

impl ProcessLevel {
    /// Game hours between updates (0.15, 0.3, 1.0; high every frame).
    pub fn interval_hours(self) -> f32 {
        match self {
            ProcessLevel::High => 0.0,
            ProcessLevel::MiddleHigh => 0.15,
            ProcessLevel::MiddleLow => 0.3,
            ProcessLevel::Low => 1.0,
        }
    }

    /// Whether someone last moved at `last` (game hours since the start of
    /// the game, `None` never) is due at `now`: the interval has passed, or
    /// the clock went back (`0096b470` and kin also run them when the hour
    /// wraps past midnight; counted here in hours since the start, so that
    /// is the clock going on).
    pub fn due(self, last: Option<f64>, now: f64) -> bool {
        match last {
            None => true,
            Some(t) => now - t >= f64::from(self.interval_hours()) || now < t,
        }
    }
}

/// The real seconds of travel an update out of sight covers (`009ea8a0`):
/// the game hours since the last one × 3600 ÷ `TimeScale`, or 900 ÷
/// `TimeScale` (15 game minutes) when there's none stored; in combat the
/// frame's own seconds.
pub fn offstage_seconds(
    hours_since: Option<f32>,
    time_scale: f32,
    in_combat: bool,
    frame: f32,
) -> f32 {
    if in_combat {
        return frame;
    }
    let scale = time_scale.max(1e-3);
    match hours_since {
        Some(h) => h * 3600.0 / scale,
        None => 900.0 / scale,
    }
}

/// Walking speed for someone moved out of sight (`00647d10` through
/// `008a0b10`): `fMoveBaseSpeed` × SpeedMult ÷ 100 × the crippled-leg
/// multiplier (`legs_crippled` 0, 1 or 2), × `fMoveRunMult` running (in
/// combat). (The armour and weapon penalties the formula can take aren't
/// applied: whether this call passes them isn't traced.)
pub fn offstage_speed(
    settings: &MoveSettings,
    speed_mult: f32,
    legs_crippled: u8,
    running: bool,
) -> f32 {
    let legs = match legs_crippled {
        0 => 1.0,
        1 => settings.one_leg,
        _ => settings.two_legs,
    };
    let walk = (speed_mult * 0.01 * settings.base_speed * legs).max(0.0);
    if running {
        walk * settings.run_mult
    } else {
        walk
    }
}

/// Where a walk out of sight got to ([`walk_polyline`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolyWalk {
    pub at: [f32; 3],
    pub heading: f32,
    /// Distance not used (the end was reached before it ran out).
    pub left: f32,
    pub done: bool,
}

/// Where someone moved out of sight ends up after `distance` along a path
/// (`path`: the points after where they stand, `from`; `009ea8a0`): point
/// to point, facing each segment, stopping partway along the last one; done
/// when the last point is reached, or within `radius` of it on the last
/// segment.
pub fn walk_polyline(
    path: &[[f32; 3]],
    from: [f32; 3],
    heading: f32,
    distance: f32,
    radius: f32,
) -> PolyWalk {
    let mut at = from;
    let mut facing = heading;
    let mut left = distance;
    for (i, &next) in path.iter().enumerate() {
        let d = distance3(at, next);
        if d > 1e-3 {
            facing = heading_to(at, next);
        }
        let last = i + 1 == path.len();
        if d <= left {
            left -= d;
            at = next;
            continue;
        }
        let f = if d > 0.0 { left / d } else { 0.0 };
        at = [0, 1, 2].map(|k| at[k] + (next[k] - at[k]) * f);
        return PolyWalk {
            at,
            heading: facing,
            left: 0.0,
            done: last && d - left < radius,
        };
    }
    PolyWalk {
        at,
        heading: facing,
        left,
        done: true,
    }
}

fn flat_distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}

fn distance3(a: [f32; 3], b: [f32; 3]) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    #[test]
    fn people_turn_in_place_at_135_degrees_a_second_and_hold_the_turn() {
        let s = MoveSettings::defaults();
        let rate = s.in_place_rate(90.0, false, false);
        assert!((rate - 135f32.to_radians()).abs() < 1e-4);
        assert!((s.in_place_rate(90.0, false, true) - 225f32.to_radians()).abs() < 1e-4);
        assert!((s.in_place_rate(200.0, true, false) - 250f32.to_radians()).abs() < 1e-3);
        let mut turn = Turn::default();
        let mut heading = 0.0;
        // Under a degree: nothing.
        assert!(!turn.request(heading, 0.01, &s));
        assert!(turn.request(heading, FRAC_PI_2, &s));
        assert!(turn.right);
        // A quarter turn at 135°/s: 2/3 s.
        let mut t = 0.0f32;
        while turn.active && t < 5.0 {
            turn.update(&mut heading, 0.01, rate);
            t += 0.01;
        }
        assert!((heading - FRAC_PI_2).abs() < 1e-5, "{heading}");
        assert!((t - 0.67).abs() < 0.02, "{t}");
        // A tiny turn still lasts the least time.
        assert!(turn.request(heading, FRAC_PI_2 + 0.05, &s));
        let mut t = 0.0f32;
        while turn.active {
            turn.update(&mut heading, 0.01, rate);
            t += 0.01;
        }
        assert!((t - 0.31).abs() < 0.015, "{t}");
        // Leftward the short way across north.
        let mut h = 0.2;
        assert!(turn.request(h, 2.0 * PI - 0.2, &s));
        assert!(!turn.right);
        assert_eq!(turn.update(&mut h, 0.01, rate), Some(TurnSide::Left));
    }

    #[test]
    fn walking_turns_are_quicker_far_off_and_cut_the_pace() {
        // 90°/s × 3 = 270°/s far off; within a degree it's taken as wanted.
        let w = walking_turn(0.0, PI * 0.95, 90.0, 0.1, 1.0);
        assert!((w.heading - 27f32.to_radians()).abs() < 1e-4);
        assert_eq!(w.forward, 0.05);
        let w = walking_turn(0.0, 2.2, 90.0, 0.1, 1.0);
        assert_eq!(w.forward, 0.1);
        let w = walking_turn(0.0, 1.8, 90.0, 0.1, 1.0);
        assert_eq!(w.forward, 0.2);
        let w = walking_turn(0.0, 1.0, 90.0, 0.1, 1.0);
        assert_eq!(w.forward, 1.0);
        let w = walking_turn(0.0, 0.01, 90.0, 0.1, 1.0);
        assert_eq!((w.heading, w.forward), (0.01, 1.0));
        // Nearly lined up: a tenth of the rate.
        assert_eq!(walking_turn_factor(90.0, true, 0.05), 0.1);
        assert_eq!(walking_turn_factor(90.0, true, PI), 1.0);
        assert_eq!(walking_turn_factor(45.0, true, 0.05), 1.0);
    }

    #[test]
    fn the_steering_point_moves_on_in_tenths() {
        let path = [[0.0, 0.0, 0.0], [100.0, 0.0, 0.0], [100.0, 100.0, 0.0]];
        // Far from the first point's tenths: it stays.
        let p = advance_progress(&path, 0.5, [0.0, 0.0, 0.0], 1.0);
        assert_eq!(p, 0.5);
        // Standing at 45: passed 0.1..0.4, within 17.5 of 0.5 and 0.6.
        let p = advance_progress(&path, 0.1, [45.0, 0.0, 0.0], 1.0);
        assert!((p - 0.7).abs() < 1e-4, "{p}");
        // Arriving: 2D, the height within 180.
        assert!(arrived([0.0, 0.0, 0.0], [10.0, 0.0, 170.0], 10.0));
        assert!(!arrived([0.0, 0.0, 0.0], [10.0, 0.0, 190.0], 10.0));
        assert!(!arrived([0.0, 0.0, 0.0], [10.5, 0.0, 0.0], 10.0));
    }

    #[test]
    fn within_distance_adds_the_radius_and_measures_flat_at_body_height() {
        // 100 + at least 32.
        assert!(within_distance(
            [0.0; 3],
            128.0,
            Some(20.25),
            [131.0, 0.0, 0.0],
            100.0,
            true
        ));
        assert!(!within_distance(
            [0.0; 3],
            128.0,
            Some(20.25),
            [133.0, 0.0, 0.0],
            100.0,
            true
        ));
        assert!(!within_distance(
            [0.0; 3],
            128.0,
            None,
            [101.0, 0.0, 0.0],
            100.0,
            false
        ));
        // At head height: flat; far above: 3D.
        assert!(within_distance(
            [0.0; 3],
            128.0,
            None,
            [90.0, 0.0, 150.0],
            100.0,
            false
        ));
        assert!(!within_distance(
            [0.0; 3],
            128.0,
            None,
            [90.0, 0.0, 170.0],
            100.0,
            false
        ));
    }

    #[test]
    fn location_radii_follow_what_is_there() {
        // A conversation's starter comes within 90 (seated: 200) plus their
        // radius; "Say To" within its distance, 0 → 120.
        assert_eq!(conversation_reach(false), 90.0);
        assert_eq!(conversation_reach(true), 200.0);
        assert_eq!(say_to_reach(0), 120.0);
        assert_eq!(say_to_reach(256), 256.0);
        assert!(within_distance(
            [0.0; 3],
            128.0,
            Some(20.25),
            [121.0, 0.0, 0.0],
            conversation_reach(false),
            true
        ));
        assert!(!within_distance(
            [0.0; 3],
            128.0,
            Some(20.25),
            [123.0, 0.0, 0.0],
            conversation_reach(false),
            true
        ));
        assert_eq!(location_radius(256, Spot::Marker, 100.0), 256.0);
        assert_eq!(location_radius(0, Spot::Furniture, 100.0), 10.0);
        assert_eq!(location_radius(0, Spot::Marker, 100.0), 20.0);
        assert_eq!(
            location_radius(
                0,
                Spot::Person {
                    asleep: true,
                    half_diagonal: 70.0
                },
                100.0
            ),
            90.0
        );
        assert_eq!(
            location_radius(
                0,
                Spot::Object {
                    half_diagonal: 30.4
                },
                100.0
            ),
            50.0
        );
        assert_eq!(
            location_radius(
                0,
                Spot::NoReference {
                    editor_location: false
                },
                100.0
            ),
            100.0
        );
        // Walking up to the player: the package's distance, else 100.
        let player = Spot::Person {
            asleep: false,
            half_diagonal: 70.0,
        };
        assert_eq!(target_reach(256, player, 100.0), 256.0);
        assert_eq!(target_reach(0, player, 100.0), 100.0);
        assert_eq!(say_to_reach(0), 120.0);
    }

    #[test]
    fn packages_are_looked_at_every_20_seconds_and_each_hour() {
        let mut c = PackageClock::default();
        assert!(c.due(0.1, 10.0, false, true));
        let mut t = 0.1;
        let mut again = 0;
        while t < 41.0 {
            if c.due(0.1, 10.0, false, true) {
                again += 1;
            }
            t += 0.1;
        }
        assert_eq!(again, 2);
        // The hour changes: at once.
        assert!(c.due(0.1, 11.0, false, true));
        assert!(!c.due(0.1, 11.0, false, true));
        assert!(c.due(0.1, 11.0, true, true));
        assert!(c.due(0.1, 11.0, false, false));
    }

    #[test]
    fn walkers_wait_for_people_walking_and_go_round_those_standing() {
        let s = AvoidanceSettings::read(&|_, _| None);
        let me = [0.0, 0.0, 0.0];
        let ahead = [0.0, 1.0];
        let v = [0.0, 85.0, 0.0];
        let standing = Obstacle {
            who: FormId(2),
            position: [0.0, 60.0, 0.0],
            velocity: [0.0; 3],
            radius: 20.25,
            is_player: false,
            seated: false,
        };
        let mut a = Avoidance::default();
        let mut step = AvoidStep::Clear;
        let mut t = 0.0f32;
        while step == AvoidStep::Clear && t < 2.0 {
            step = a.update(
                me,
                REQUEST_RADIUS,
                ahead,
                v,
                false,
                None,
                &[standing],
                &s,
                0.05,
            );
            t += 0.05;
        }
        // Twice the time for someone standing, × the share (cos 0 = 1):
        // past 0.5 s after 0.3 s.
        assert!((t - 0.3).abs() < 0.06, "{t}");
        match step {
            AvoidStep::Repath(nodes) => {
                assert_eq!(nodes.len(), 1);
                assert!((nodes[0].radius - 20.25 * 1.5).abs() < 1e-3);
                assert_eq!(nodes[0].cost, 2.0);
            }
            other => panic!("{other:?}"),
        }
        // Ignored for 5 s after.
        assert_eq!(
            a.update(
                me,
                REQUEST_RADIUS,
                ahead,
                v,
                false,
                None,
                &[standing],
                &s,
                0.05
            ),
            AvoidStep::Clear
        );
        // Someone walking toward them: wait.
        let walking = Obstacle {
            velocity: [0.0, -85.0, 0.0],
            ..standing
        };
        let mut a = Avoidance::default();
        let mut step = AvoidStep::Clear;
        for _ in 0..40 {
            step = a.update(
                me,
                REQUEST_RADIUS,
                ahead,
                v,
                false,
                None,
                &[walking],
                &s,
                0.05,
            );
            if step != AvoidStep::Clear {
                break;
            }
        }
        assert_eq!(step, AvoidStep::Wait);
        assert_eq!(a.waiting, 5.0);
        // Off to the side: no one in the way.
        let aside = Obstacle {
            position: [300.0, 0.0, 0.0],
            ..standing
        };
        let mut a = Avoidance::default();
        for _ in 0..40 {
            assert_eq!(
                a.update(
                    me,
                    REQUEST_RADIUS,
                    ahead,
                    v,
                    false,
                    None,
                    &[aside],
                    &s,
                    0.05
                ),
                AvoidStep::Clear
            );
        }
    }

    #[test]
    fn people_out_of_sight_walk_their_paths_by_the_clock() {
        assert_eq!(ProcessLevel::MiddleLow.interval_hours(), 0.3);
        assert!(ProcessLevel::Low.due(None, 5.0));
        assert!(!ProcessLevel::Low.due(Some(4.5), 5.0));
        assert!(ProcessLevel::Low.due(Some(4.0), 5.0));
        // An hour at TimeScale 30: 120 s; none stored: 30 s.
        assert_eq!(offstage_seconds(Some(1.0), 30.0, false, 0.016), 120.0);
        assert_eq!(offstage_seconds(None, 30.0, false, 0.016), 30.0);
        let s = MoveSettings::defaults();
        assert!((offstage_speed(&s, 100.0, 0, false) - 85.0).abs() < 1e-4);
        assert!((offstage_speed(&s, 100.0, 1, true) - 85.0 * 0.85 * 4.0).abs() < 1e-3);
        let path = [[0.0, 0.0, 0.0], [100.0, 0.0, 0.0], [100.0, 100.0, 0.0]];
        let w = walk_polyline(&path[1..], path[0], 0.0, 150.0, 0.0);
        assert!((w.at[0] - 100.0).abs() < 1e-4 && (w.at[1] - 50.0).abs() < 1e-4);
        assert!(w.heading.abs() < 1e-5 && !w.done);
        let w = walk_polyline(&path[1..], path[0], 0.0, 500.0, 0.0);
        assert!(w.done && (w.at[1] - 100.0).abs() < 1e-4);
        assert!((w.left - 300.0).abs() < 1e-3);
    }

    #[test]
    fn walkers_held_back_for_a_second_and_a_half_are_stuck() {
        let mut s = Stuck::default();
        let dt = 0.1;
        // Moving their whole move: never stuck.
        let mut p = [0.0f32, 0.0, 0.0];
        for _ in 0..30 {
            assert!(!s.update(p, 5.0, false, dt));
            p[0] += 5.0;
        }
        assert_eq!(s.time, 0.0);
        // Held to a third of it (under √0.125 ≈ 0.354): blocked; past 1.5 s
        // stuck, and stuck stays.
        let mut stuck_after = None;
        for i in 0..30 {
            p[0] += 5.0 / 3.0;
            if s.update(p, 5.0, false, dt) && stuck_after.is_none() {
                stuck_after = Some(i);
            }
        }
        assert_eq!(stuck_after, Some(15), "{s:?}");
        assert!(s.update(p, 0.0, false, dt));
        // Up against someone the time runs at a fifth: 75 frames (and the
        // first, with nothing to compare).
        let mut s = Stuck::default();
        let mut n = 0;
        while !s.update(p, 5.0, true, dt) {
            n += 1;
        }
        assert!((75..=77).contains(&n), "{n}");
        // A frame not blocked starts the count again.
        let mut s = Stuck::default();
        for _ in 0..10 {
            s.update(p, 5.0, false, dt);
        }
        p[0] += 5.0;
        s.update(p, 5.0, false, dt);
        assert_eq!(s.time, 0.0);
    }
}
