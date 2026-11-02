//! What people do: AI packages (`PACK`) and finding their way on the
//! navmesh (`NAVM`).
//!
//! A person's base lists packages (`PKID`) in order; the one they follow is
//! the first whose conditions pass and whose schedule covers the time (a
//! script can put one first, `AddScriptPackage`). A package (`PACK`):
//! `PKDT` (flags u32, type u8, unused u8, behaviour flags u16, type flags
//! u16, 2 unused bytes), `PLDT` where (kind i32, form u32, radius i32),
//! `PSDT` when (month i8, day of week i8, date u8, hour i8, duration i32;
//! -1 / 0 for "any"), `PTDT` the target, `CTDA` conditions asked about the
//! person. Package types (New Vegas): 0 find, 1 follow, 2 escort, 3 eat,
//! 4 sleep, 5 wander, 6 travel, 7 accompany, 8 use item at, 9 ambush, 10
//! flee, 12 sandbox, 13 patrol, 14 guard, 15 dialogue, 16 use weapon
//! (`VCG01DocMitchellTravelToPlayerAtTester` is a 6, with
//! "`GetStage VCG01 >= 55`").
//!
//! A navmesh: `NVVX` vertices (3 floats), `NVTR` triangles of 16 bytes
//! (three vertex numbers, the triangle across each edge, -1 for none,
//! flags, cover flags). The triangle across edge `i` shares the edge from
//! vertex `i` to vertex `i + 1` (checked on Doc Mitchell's house: its
//! triangle 0 lists triangle 1 across its third edge, and they share those
//! two vertices). A path is first tried as a straight line over the
//! navmesh (`bUseStraightLineCheckFirst`, `006cc5e0` → `006cd1f0`): when
//! the line stays on it (with room to either side above the land), that's
//! the path. Otherwise the game's navmesh search ([`navsearch`]) finds the
//! triangles and its path smoother (`PathSmootherPOVSearch`, `smoother`)
//! the points, keeping the walker's radius off the corners.

use std::collections::HashMap;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::dialogue::{read_condition, Condition};
use crate::movement::Spot;
use crate::scripting::{Facts, GameState};

pub mod actions;
pub mod data;
pub mod doors;
pub mod flee;
pub mod guard;
pub mod navinfo;
pub mod navsearch;
pub mod offmesh;
pub mod procedures;
mod smoother;
pub mod talk;

const PACK: FourCC = FourCC::new(b"PACK");
const PKDT: FourCC = FourCC::new(b"PKDT");
const PLDT: FourCC = FourCC::new(b"PLDT");
const PSDT: FourCC = FourCC::new(b"PSDT");
const PTDT: FourCC = FourCC::new(b"PTDT");
const PKDD: FourCC = FourCC::new(b"PKDD");
const CTDA: FourCC = FourCC::new(b"CTDA");
const PKID: FourCC = FourCC::new(b"PKID");
const NAVM: FourCC = FourCC::new(b"NAVM");
const NVVX: FourCC = FourCC::new(b"NVVX");
const NVTR: FourCC = FourCC::new(b"NVTR");
const NVEX: FourCC = FourCC::new(b"NVEX");
const NVDP: FourCC = FourCC::new(b"NVDP");
const XLKR: FourCC = FourCC::new(b"XLKR");

/// Package types (`PKDT` byte 4; the procedure each runs:
/// [`procedures::of_kind`]).
pub mod kinds {
    pub const FIND: u8 = 0;
    pub const FOLLOW: u8 = 1;
    pub const ESCORT: u8 = 2;
    pub const EAT: u8 = 3;
    pub const SLEEP: u8 = 4;
    pub const WANDER: u8 = 5;
    pub const TRAVEL: u8 = 6;
    pub const ACCOMPANY: u8 = 7;
    pub const USE_ITEM_AT: u8 = 8;
    pub const AMBUSH: u8 = 9;
    pub const FLEE: u8 = 10;
    pub const SANDBOX: u8 = 12;
    pub const PATROL: u8 = 13;
    pub const GUARD: u8 = 14;
    pub const DIALOGUE: u8 = 15;
    pub const USE_WEAPON: u8 = 16;
    /// The packages the game makes itself, by the exe's type-name table
    /// (`0119bcb0`, one name per type): 21 "Alarm" (tested by `008a61b0`).
    pub const ALARM: u8 = 21;
}

/// A `PLDT`/`PLD2` as the loader keeps it (`0067f060`: kind i32, form u32,
/// radius i32 on disk): "in a cell" (1) has no radius, "near the current
/// location" (2) and "near the editor location" (3) no form.
// Translated from 0067f060 (decompiled, FalloutNV.exe 1.4.0.525).
pub(crate) fn read_location(data: &[u8], global: impl Fn(FormId) -> FormId) -> Location {
    let kind = le_u32(data, 0) as i32;
    let raw = le_u32(data, 4);
    let form = match kind {
        // A reference, a cell, an object.
        0 | 1 | 4 => global(FormId(raw)),
        2 | 3 => FormId(0),
        _ => FormId(raw),
    };
    let radius = if kind == 1 { 0 } else { le_u32(data, 8) as i32 };
    Location { kind, form, radius }
}

/// A reference's linked reference (`XLKR`; New Vegas's is a keyword then
/// the reference, or just the reference).
pub fn linked_ref(order: &LoadOrder, reference: FormId) -> Option<FormId> {
    let rr = order.get(reference)?;
    let record = rr.record().ok()?;
    let s = record.get(XLKR).filter(|s| s.data.len() >= 4)?;
    let at = if s.data.len() >= 8 { 4 } else { 0 };
    Some(rr.plugin.to_global(FormId(le_u32(&s.data, at)))).filter(|f| f.0 != 0)
}

/// A package location's radius as the procedures read it (`00676280`):
/// its own (0 for "in a cell"), except that a reference to an activator
/// with radius 0 gives round(half its bounds' diagonal).
pub fn location_radius_of(order: &LoadOrder, loc: &Location) -> u32 {
    let own = loc.radius.max(0) as u32;
    if own == 0 && loc.kind == 0 {
        let activator = crate::scripting::base_of(order, loc.form)
            .and_then(|b| order.get(b))
            .is_some_and(|rr| rr.entry.header.kind.as_bytes() == b"ACTI");
        if activator {
            return half_bounds_diagonal(order, loc.form).round().max(0.0) as u32;
        }
    }
    own
}

/// Where a package takes place (`PLDT`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    /// 0 near a reference, 1 in a cell, 2 near the current location, 3 near
    /// the editor location, 4 an object, 5 an object type, 6 near the
    /// linked reference, 7 at the package's location.
    pub kind: i32,
    pub form: FormId,
    pub radius: i32,
}

/// When a package applies (`PSDT`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Schedule {
    /// 0..11, -1 any.
    pub month: i8,
    /// 0 Sunday … 6 Saturday, 7 weekdays, 8 weekends, 9 Monday Wednesday
    /// Friday, 10 Tuesday Thursday; -1 any.
    pub day_of_week: i8,
    /// Day of the month, 0 any.
    pub date: u8,
    /// Starting hour, -1 any.
    pub hour: i8,
    /// Hours.
    pub duration: i32,
}

impl Schedule {
    /// Whether it covers a moment: month (0..11), day of the week (0
    /// Sunday), date and hour (fractional).
    pub fn covers(&self, month: i32, weekday: i32, date: i32, hour: f32) -> bool {
        if self.month >= 0 && i32::from(self.month) != month {
            return false;
        }
        let day_ok = match self.day_of_week {
            d if d < 0 => true,
            d @ 0..=6 => i32::from(d) == weekday,
            7 => (1..=5).contains(&weekday),
            8 => weekday == 0 || weekday == 6,
            9 => [1, 3, 5].contains(&weekday),
            10 => [2, 4].contains(&weekday),
            _ => true,
        };
        if !day_ok || (self.date > 0 && i32::from(self.date) != date) {
            return false;
        }
        if self.hour < 0 || self.duration <= 0 {
            return true;
        }
        let start = f32::from(self.hour);
        let end = start + self.duration as f32;
        // Past midnight it wraps.
        (hour >= start && hour < end) || (end > 24.0 && hour < end - 24.0)
    }
}

/// An AI package.
#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    pub kind: u8,
    pub flags: u32,
    pub location: Option<Location>,
    pub schedule: Schedule,
    pub conditions: Vec<Condition>,
    /// Who it's aimed at (`PTDT`: kind i32, form, a count or distance):
    /// for dialogue packages, who to talk to and how close they must come
    /// (Sunny Smiles' greeting: the player, 256).
    pub target: Option<(i32, FormId, i32)>,
    /// A dialogue package's topic (`PKDD`, after the field of view);
    /// `None` for a greeting.
    pub topic: Option<FormId>,
    /// The package's begin, end and change actions. These are separate
    /// from its ordinary idle list; the opening uses them for the player's
    /// wakeup, situp and standup animations.
    pub actions: actions::PackageActions,
    /// Its type's own data (`PKW3`, `PKPT`, `PKE2`, `PKFD`, `PLD2`).
    pub data: data::TypeData,
}

impl Package {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Package> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == PACK)?;
        let record = rr.record_shared().ok()?;
        let pkdt = record.get(PKDT).filter(|s| s.data.len() >= 5)?;
        let location = record
            .get(PLDT)
            .filter(|s| s.data.len() >= 12)
            .map(|s| read_location(&s.data, |id| rr.plugin.to_global(id)));
        let schedule = record
            .get(PSDT)
            .filter(|s| s.data.len() >= 8)
            .map(|s| Schedule {
                month: s.data[0] as i8,
                day_of_week: s.data[1] as i8,
                date: s.data[2],
                hour: s.data[3] as i8,
                duration: le_u32(&s.data, 4) as i32,
            })
            .unwrap_or(Schedule {
                month: -1,
                day_of_week: -1,
                date: 0,
                hour: -1,
                duration: 0,
            });
        let target = record.get(PTDT).filter(|s| s.data.len() >= 12).map(|s| {
            let kind = le_u32(&s.data, 0) as i32;
            let raw = FormId(le_u32(&s.data, 4));
            // Kind 0 names a reference.
            let form = if kind == 0 {
                rr.plugin.to_global(raw)
            } else {
                raw
            };
            (kind, form, le_u32(&s.data, 8) as i32)
        });
        let topic = record
            .get(PKDD)
            .filter(|s| s.data.len() >= 8)
            .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 4))))
            .filter(|t| t.0 != 0);
        Some(Package {
            form_id: id,
            editor_id: record.editor_id(),
            kind: pkdt.data[4],
            flags: le_u32(&pkdt.data, 0),
            location,
            schedule,
            conditions: record
                .get_all(CTDA)
                .filter_map(|s| read_condition(&rr, &s.data))
                .collect(),
            target,
            topic,
            actions: actions::PackageActions::read(&record, |id| rr.plugin.to_global(id)),
            data: data::read(&record, |id| rr.plugin.to_global(id)),
        })
    }

    /// The procedures its type runs, when known ([`procedures::of_kind`]).
    pub fn procedures(&self) -> Option<&'static [u8]> {
        procedures::of_kind(self.kind)
    }
}

const PLD2: FourCC = FourCC::new(b"PLD2");

/// A dialogue package's own data (`PKDD`, 24 bytes; laid out as the
/// package-data save and load `0067b630`/`0067b7d0` and the getters
/// `00672710`–`00672850` read it).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogueData {
    /// f32 at 0: only the dialogue camera's zoom reads it (× FOV ÷ 100,
    /// `00761a20`, `00953060`): not a field of view the speaker needs.
    pub fov: f32,
    /// Form at 4: the topic (`None`: their greeting).
    pub topic: Option<FormId>,
    /// Byte 8 flag 0x01: no head tracking (`00672800`).
    pub no_head_tracking: bool,
    /// Byte 9 flag 0x01: the target's movement isn't taken over
    /// (`006727b0`).
    pub dont_control_target: bool,
    /// Byte 16 non-zero: "Say To" (`00672710`): a line said, no dialogue.
    pub say_to: bool,
}

/// A dialogue package's data ([`DialogueData`]).
pub fn dialogue_data(order: &LoadOrder, package: FormId) -> Option<DialogueData> {
    let rr = order.get(package).filter(|r| r.entry.header.kind == PACK)?;
    let record = rr.record().ok()?;
    let s = record.get(PKDD).filter(|s| s.data.len() >= 8)?;
    let byte = |i: usize| s.data.get(i).copied().unwrap_or(0);
    Some(DialogueData {
        fov: le_f32(&s.data, 0),
        topic: Some(rr.plugin.to_global(FormId(le_u32(&s.data, 4)))).filter(|t| t.0 != 0),
        no_head_tracking: byte(8) & 0x01 != 0,
        dont_control_target: byte(9) & 0x01 != 0,
        say_to: byte(16) != 0,
    })
}

/// A package's second location (`PLD2`, laid out as `PLDT`; `00672dd0`):
/// for dialogue packages, where the target must be before the talk starts.
pub fn second_location(order: &LoadOrder, package: FormId) -> Option<Location> {
    let rr = order.get(package).filter(|r| r.entry.header.kind == PACK)?;
    let record = rr.record().ok()?;
    let s = record.get(PLD2).filter(|s| s.data.len() >= 12)?;
    Some(read_location(&s.data, |id| rr.plugin.to_global(id)))
}

/// What a reference is, for its radius ([`crate::movement::Spot`]):
/// furniture, an `XMarker`/`XMarkerHeading`, someone (asleep when their
/// sit state is 9: settled in a bed), or anything else; with half its
/// bounds' diagonal (`OBND` × its scale; `00571600`, `0050ebf0`).
pub fn spot_of(order: &LoadOrder, state: &GameState, reference: FormId) -> Spot {
    let base = crate::scripting::base_of(order, reference);
    if base == Some(crate::movement::X_MARKER) || base == Some(crate::movement::X_MARKER_HEADING) {
        return Spot::Marker;
    }
    if GameState::is_furniture(order, reference) {
        return Spot::Furniture;
    }
    let half_diagonal = half_bounds_diagonal(order, reference);
    let person = order
        .get(reference)
        .is_some_and(|rr| matches!(rr.entry.header.kind.as_bytes(), b"ACHR" | b"ACRE"))
        || reference == crate::dialogue::PLAYER_REF;
    if person {
        let asleep = state
            .sitters
            .get(&reference)
            .is_some_and(|s| s.question().1 == 3);
        Spot::Person {
            asleep,
            half_diagonal,
        }
    } else {
        Spot::Object { half_diagonal }
    }
}

const OBND: FourCC = FourCC::new(b"OBND");
const XSCL: FourCC = FourCC::new(b"XSCL");

/// Half the diagonal of a reference's bounds: its base's `OBND` box ×
/// its scale (`XSCL`).
pub fn half_bounds_diagonal(order: &LoadOrder, reference: FormId) -> f32 {
    let scale = order
        .get(reference)
        .and_then(|rr| rr.record().ok())
        .and_then(|r| {
            r.get(XSCL)
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_f32(&s.data, 0))
        })
        .unwrap_or(1.0);
    crate::scripting::base_of(order, reference)
        .and_then(|b| order.get(b))
        .and_then(|rr| rr.record().ok())
        .and_then(|r| {
            r.get(OBND).filter(|s| s.data.len() >= 12).map(|s| {
                let v =
                    |i: usize| f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]));
                let d = [v(3) - v(0), v(4) - v(1), v(5) - v(2)];
                0.5 * (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() * scale
            })
        })
        .unwrap_or(0.0)
}

/// The radius an actor's path requests carry (`006e29f0`: 0.6 × the actor's
/// width, `008be280` → `00885140`: its bounds' x extent × its scale,
/// `GetScale` `00567400`). The bounds are the base's `OBND` here, × the
/// reference's `XSCL`; the game asks the actor's bound vfuncs
/// (`00933630`/`00933700`), which take the process's own bound first when
/// it has one (+0x5f0, not traced). No bounds: the request's default, 35.
pub fn request_radius(order: &LoadOrder, reference: FormId) -> f32 {
    let scale = order
        .get(reference)
        .and_then(|rr| rr.record().ok())
        .and_then(|r| {
            r.get(XSCL)
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_f32(&s.data, 0))
        })
        .unwrap_or(1.0);
    crate::scripting::base_of(order, reference)
        .and_then(|b| order.get(b))
        .and_then(|rr| rr.record().ok())
        .and_then(|r| {
            r.get(OBND).filter(|s| s.data.len() >= 12).map(|s| {
                let v =
                    |i: usize| f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]));
                (v(3) - v(0)) * scale * 0.6
            })
        })
        .filter(|r| *r > 0.0)
        .unwrap_or(crate::movement::REQUEST_RADIUS)
}

/// Whether someone is at a dialogue package's second location (`PLD2`;
/// the package's vfunc +0x140, `00676390`): for a trigger (an activator
/// with a primitive, `0067f220` → `0062ddb0`) inside its volume; else
/// within its radius (`PLD2`'s own, else by what the reference is,
/// [`crate::movement::location_radius`]) of it, measured as
/// `IsWithinDistance` (adding their radius except for markers and
/// furniture). Not in the same place: no.
pub fn at_second_location(
    order: &LoadOrder,
    state: &GameState,
    location: &Location,
    who: FormId,
    min_radius: f32,
) -> bool {
    let Some((here, _, at, _)) = state.place(order, who) else {
        return false;
    };
    let reference = match location.kind {
        0 => location.form,
        _ => return false,
    };
    let Some((there, _, spot_at, _)) = state.place(order, reference) else {
        return false;
    };
    if here != there {
        return false;
    }
    if let Some(p) = crate::placement_of(order, reference).filter(|p| p.primitive.is_some()) {
        if p.base_type.as_bytes() == b"ACTI" {
            let prim = p.primitive.expect("checked");
            let s = if p.scale > 0.0 { p.scale } else { 1.0 };
            let m = crate::RotationConvention::DEFAULT.matrix(p.rotation);
            // The body against the volume: three points of it, 10, 64 and
            // 110 above the feet, as the viewer's triggers test (the game
            // tests the actor's collision shape, `0062df20`).
            return [10.0, 64.0, 110.0].iter().any(|up| {
                let d = [
                    at[0] - spot_at[0],
                    at[1] - spot_at[1],
                    at[2] + up - spot_at[2],
                ];
                let local = [0, 1, 2].map(|i| m[0][i] * d[0] + m[1][i] * d[1] + m[2][i] * d[2]);
                if prim.shape == 2 {
                    local.iter().map(|v| v * v).sum::<f32>() <= (prim.half[0] * s).powi(2)
                } else {
                    (0..3).all(|i| local[i].abs() <= prim.half[i] * s)
                }
            });
        }
    }
    let spot = spot_of(order, state, reference);
    let radius = crate::movement::location_radius(location.radius, spot, min_radius);
    let add = !matches!(spot, Spot::Marker | Spot::Furniture);
    crate::movement::within_distance(
        at,
        128.0,
        Some(crate::combat_ai::PERSON_RADIUS),
        spot_at,
        radius,
        add,
    )
}

/// What a dialogue package (type 15) has its person do now: its list is
/// TRAVEL → DIALOGUE_ACTIVATE → WAIT → DIALOGUE (`011a3ff0` list 10).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogueStep {
    /// To the package's place first (TRAVEL, `008e5e90`).
    Travel,
    /// The target isn't at the second location (`PLD2`) yet.
    Wait,
    /// Walk up to the target until within `reach` (a path with that
    /// radius, `008b36f0`; for "Say To" measured in 3D, else as
    /// `IsWithinDistance` adding the walker's radius).
    Approach { reach: f32 },
    /// Say the topic (else `HELLO`) as a line, no dialogue menu (the GREET
    /// procedure).
    Say,
    /// Start the conversation: the dialogue menu for the player (the
    /// type-0x1c package activates the player, `008e9640`), else a
    /// conversation between the two.
    Talk,
}

/// The step of a dialogue package for `who` now (`008e8600`, with the
/// type-0x1c package it makes, `008b19c0`, and the activate procedure
/// `008e9640`): first the travel to its place (`PLDT`; skipped when there's
/// none, or it's the person themselves without "near the editor location").
/// Then, for "Say To" with the player as target: within the target distance
/// (`PTDT`, ≤ 0 → 120, 3D) the line, else walk up. Otherwise, with the
/// player as target and a second location (`PLD2`): wait until the player
/// is at it ([`at_second_location`]); then talk at once if within reach
/// (the package vfunc +0x144 `00676e40`), else walk up and talk there.
/// Without one: walk up until within reach — the target distance, else
/// `iAIDistanceRadiusMinLocation` (100), plus the walker's radius (at least
/// 32), in 2D at body height ([`crate::movement::within_distance`]) — and
/// talk. (`at_place`: whether the travel is done.)
pub fn dialogue_step(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    package: &Package,
    at_place: bool,
    my_radius: f32,
) -> Option<DialogueStep> {
    let (kind, target, distance) = package.target?;
    if kind != 0 || target.0 == 0 {
        return None;
    }
    let travel = package
        .location
        .is_some_and(|l| !(l.kind == 2 || (l.kind == 0 && l.form == who)));
    if travel && !at_place {
        return Some(DialogueStep::Travel);
    }
    let data = dialogue_data(order, package.form_id);
    let (here, _, me, _) = state.place(order, who)?;
    let (there, _, at, _) = state.place(order, target)?;
    if here != there {
        return Some(DialogueStep::Wait);
    }
    let player = target == crate::dialogue::PLAYER_REF;
    if player && data.is_some_and(|d| d.say_to) {
        let reach = crate::movement::say_to_reach(distance);
        let d = (0..3).map(|i| (at[i] - me[i]).powi(2)).sum::<f32>().sqrt();
        return Some(if d > reach {
            DialogueStep::Approach { reach }
        } else {
            DialogueStep::Say
        });
    }
    let min = min_location_radius(order);
    let reach = crate::movement::target_reach(distance, spot_of(order, state, target), min);
    let within = crate::movement::within_distance(me, 128.0, Some(my_radius), at, reach, true);
    if player {
        if let Some(l2) = second_location(order, package.form_id) {
            if !at_second_location(order, state, &l2, target, min) {
                return Some(DialogueStep::Wait);
            }
        }
    }
    Some(if within {
        DialogueStep::Talk
    } else {
        DialogueStep::Approach { reach }
    })
}

/// The ring a wander spot is chosen in (`008ed420`): from 32 to 0.75 × the
/// package's radius around the centre (`0040ebd0(32.0, r × 0.75)`). How the
/// game's navmesh search picks within it isn't traced.
pub fn wander_ring(radius: f32) -> (f32, f32) {
    (32.0, 0.75 * radius)
}

/// Below this radius a wander package's people just stand (`008ed420`:
/// 60). Only wander packages (the package's list type 1): the procedure
/// run for a sandbox (or a guard) package skips this test, and the
/// [`WANDER_LEASH`] one.
pub const LEAST_WANDER_RADIUS: f32 = 60.0;

/// How far beyond its radius a wander package's person may stray from the
/// middle before going back to it (`008ed420`: radius + 250, then the
/// procedure goes back to the travel). Not for sandboxes (their own rule is
/// radius + 150, [`crate::sandbox::Sandbox::strayed`]).
pub const WANDER_LEASH: f32 = 250.0;

/// What the wander procedure (`008ed420`) has someone with a wander package
/// (type 5) do, once the travel to the package's place is over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WanderStep {
    /// Back to the place (the travel again): farther than the radius +
    /// [`WANDER_LEASH`] from the middle, or (with a radius under
    /// [`LEAST_WANDER_RADIUS`]) not at the place.
    Back,
    /// Stand where they are, facing an `XMarkerHeading`'s heading if the
    /// place is one ([`arrival_heading`]): a radius under
    /// [`LEAST_WANDER_RADIUS`].
    Stand,
    /// Wander: a spot in [`wander_ring`] around the middle now and then
    /// ([`crate::sandbox::WanderTimer`]).
    Wander,
}

/// The wander procedure's choice for a wander package (`008ed420`, list
/// type 1): `radius` the package's (`PLDT`), `own_place` when the middle is
/// the person's own position ("in a cell" without a reference: then the
/// radius test is skipped), `at_place` whether they're at the package's
/// place (the package's own test, vfunc +0x13c), `to_middle` their distance
/// from the middle (3D, `00457910`).
pub fn wander_step(radius: f32, own_place: bool, at_place: bool, to_middle: f32) -> WanderStep {
    if radius < LEAST_WANDER_RADIUS && !own_place {
        return if at_place {
            WanderStep::Stand
        } else {
            WanderStep::Back
        };
    }
    if to_middle > radius + WANDER_LEASH {
        WanderStep::Back
    } else {
        WanderStep::Wander
    }
}

/// The radius a wander package's person wanders in, and whether its middle
/// is their own position (`008ed420`): the package's `PLDT` radius
/// (`00676280`: for an activator with radius 0, half its bounds' diagonal,
/// rounded); "in a cell" (kind 1) puts the middle on the person, and indoors
/// the spots then come from a radius of 800 (outdoors it has none: the
/// loader drops that kind's radius, [`read_location`]).
pub fn wander_radius(order: &LoadOrder, package: &Package, interior: bool) -> (f32, bool) {
    let Some(loc) = package.location else {
        return (0.0, false);
    };
    if loc.kind == 1 {
        let r = if interior { 800.0 } else { loc.radius as f32 };
        return (r, true);
    }
    let mut r = loc.radius.max(0) as f32;
    if r == 0.0 && loc.kind == 0 {
        let activator = crate::scripting::base_of(order, loc.form)
            .and_then(|b| order.get(b))
            .is_some_and(|rr| rr.entry.header.kind.as_bytes() == b"ACTI");
        if activator {
            r = half_bounds_diagonal(order, loc.form).round();
        }
    }
    (r, false)
}

/// The heading of a package's place when it's an `XMarkerHeading` (form
/// 0x34): the marker's own.
pub fn marker_heading(order: &LoadOrder, state: &GameState, package: &Package) -> Option<f32> {
    let loc = package.location.filter(|l| l.kind == 0)?;
    (crate::scripting::base_of(order, loc.form) == Some(crate::movement::X_MARKER_HEADING))
        .then(|| state.place(order, loc.form).map(|p| p.3))
        .flatten()
}

/// The heading someone turns to at the end of the travel procedure to
/// their package's place (`008e5e90` in sight: a turn in place, `008bb5c0`;
/// `0090ad40` out of sight: set at once): the place's own when it's an
/// `XMarkerHeading` ([`marker_heading`]), else for "near the editor
/// location" (kind 3) where they were placed facing (the actor's start
/// rotation, +0x16c; that the location gives no reference then is
/// inferred). Only for the packages known to start with that procedure:
/// travel (list 0) and dialogue (list 10); the other lists' steps aren't
/// traced. Not while using furniture (the caller's).
pub fn arrival_heading(
    order: &LoadOrder,
    state: &GameState,
    actor: FormId,
    package: &Package,
) -> Option<f32> {
    if package.kind != kinds::TRAVEL && package.kind != kinds::DIALOGUE {
        return None;
    }
    let loc = package.location?;
    match loc.kind {
        0 => marker_heading(order, state, package),
        3 => crate::scripting::whereabouts(order, actor).map(|w| w.heading),
        _ => None,
    }
}

/// A person's packages in order (`PKID` on their base).
pub fn packages_of(order: &LoadOrder, base: FormId) -> Vec<FormId> {
    let Some(rr) = order.get(base) else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .get_all(PKID)
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .collect()
}

/// The package a person follows now: one a script gave them, else the
/// first of theirs whose conditions pass (asked about them, with the
/// player as the target) and whose schedule covers the game's clock.
pub fn current_package(order: &LoadOrder, state: &GameState, actor: FormId) -> Option<Package> {
    let chosen = chosen_package(order, state, actor);
    // [G] The player's teammates follow them when they have nothing to do
    // (the game's follower behaviour isn't traced: this is a package that
    // keeps them [`TEAMMATE_DISTANCE`] from the player). Off unless
    // [`crate::guesses`] is on: the base game's companions follow by their
    // own packages.
    if !crate::guesses::enabled() {
        return chosen;
    }
    if teammate_follows(chosen.as_ref())
        && state.teammates.contains(&actor)
        && !state.dead.contains(&actor)
    {
        return Some(Package {
            form_id: FormId(0),
            editor_id: Some("TeammateFollowsPlayer".into()),
            kind: kinds::FOLLOW,
            flags: 0,
            location: None,
            schedule: Schedule::default(),
            conditions: Vec::new(),
            target: Some((0, crate::dialogue::PLAYER_REF, TEAMMATE_DISTANCE)),
            topic: None,
            actions: Default::default(),
            data: Default::default(),
        });
    }
    chosen
}

/// Whether a teammate with this package (if any) follows the player: with
/// none, or one that only fills time (sandbox, wander, patrol, find). A
/// guard package is the "wait here" order: the companions' own wait packages
/// (Boone's `FollowersBooneFollowPlayerWAIT`, Dog's in Dead Money) are
/// guard packages whose conditions hold while the player has told them to
/// wait, so the teammate stays where they are. [G] that the rest of the
/// game's follower behaviour follows from the data like this.
pub fn teammate_follows(chosen: Option<&Package>) -> bool {
    !chosen.is_some_and(|p| {
        !matches!(
            p.kind,
            kinds::SANDBOX | kinds::WANDER | kinds::PATROL | kinds::FIND
        )
    })
}

/// How near a teammate keeps to the player ([G] a guess, with
/// [`crate::guesses`] on).
pub const TEAMMATE_DISTANCE: i32 = 200;

fn chosen_package(order: &LoadOrder, state: &GameState, actor: FormId) -> Option<Package> {
    // Coming to warn the trespassing player (`world::living::trespass`).
    if let Some(p) = crate::living::trespass::package(state, actor) {
        return Some(p);
    }
    if let Some(&p) = state.script_packages.get(&actor) {
        return Package::load(order, p);
    }
    let base = crate::scripting::base_of(order, actor)?;
    let g = |name: &str| state.global(order, name).unwrap_or(0.0);
    let (year, month, date, hour) = (
        g("GameYear") as i32,
        g("GameMonth") as i32,
        g("GameDay") as i32,
        g("GameHour"),
    );
    let weekday = crate::scripting::day_of_week(year, month, date);
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    packages_of(order, base)
        .into_iter()
        .filter_map(|p| Package::load(order, p))
        .find(|p| {
            p.schedule.covers(month, weekday, date, hour)
                && facts.conditions_pass(&p.conditions, actor, crate::dialogue::PLAYER_REF)
        })
}

/// Whom a follow or accompany package keeps near, and how near: its
/// target (`PTDT`, a specific reference) and the target's value, which
/// for these is the distance (`CheyenneAccompany`: Sunny Smiles, 128;
/// `VFactionSquadPackageNCRFollowLeader`: 420). A guess at the field's
/// meaning, from those values; at least 64.
pub fn followed(package: &Package) -> Option<(FormId, f32)> {
    if package.kind != kinds::FOLLOW && package.kind != kinds::ACCOMPANY {
        return None;
    }
    let (kind, who, distance) = package.target?;
    (kind == 0 && who.0 != 0).then_some((who, (distance as f32).max(64.0)))
}

/// `iAIDistanceRadiusMinLocation` (100): the radius when nothing else
/// gives one.
pub fn min_location_radius(order: &LoadOrder) -> f32 {
    crate::scripting::game_setting(order, "iAIDistanceRadiusMinLocation").unwrap_or(100.0)
}

/// Where a package sends a person, as a point and how close counts as
/// there: near a reference (where it stands), near the person's editor
/// location (where they're placed), near their linked reference, near
/// whom they follow. The radius is the travel's (`00678670`,
/// [`crate::movement::location_radius`]): the package's own, else by what's
/// there; "near the editor location" measures by the person themselves
/// (`0067f2a0` gives the actor as the reference). `None` for packages that
/// don't lead anywhere this can work out.
pub fn destination(
    order: &LoadOrder,
    state: &GameState,
    actor: FormId,
    package: &Package,
) -> Option<([f32; 3], f32)> {
    if let Some((who, distance)) = followed(package) {
        let (here, ..) = state.place(order, actor)?;
        let (there, _, position, _) = state.place(order, who)?;
        return (here == there).then_some((position, distance));
    }
    let loc = package.location?;
    let min = min_location_radius(order);
    let radius_of = |reference: FormId| {
        crate::movement::location_radius(loc.radius, spot_of(order, state, reference), min)
    };
    let radius = if matches!(loc.kind, 0 | 3 | 6) {
        0.0
    } else {
        loc.radius.max(0) as f32
    };
    let target = match loc.kind {
        0 => loc.form,
        3 => actor,
        6 => linked_ref(order, actor)?,
        _ => return None,
    };
    if loc.kind == 3 {
        // Their editor location: where they were placed (a `MoveTo`
        // doesn't change it, `world::ai::guard`); only in the place they
        // are now (else the way there is a door, [`target_place`]).
        let placed = crate::scripting::whereabouts(order, actor)?;
        let (here, ..) = state.place(order, actor)?;
        if placed.world.unwrap_or(placed.cell) != here {
            return None;
        }
        return Some((placed.position, radius_of(actor).max(radius)));
    }
    // Only somewhere in the same interior or worldspace as where they are
    // now: the way to another place is through a door (`door_toward`).
    let (here, ..) = state.place(order, actor)?;
    let (there, _, position, _) = state.place(order, target)?;
    if here != there {
        return None;
    }
    Some((position, radius_of(target).max(radius)))
}

/// Where a package's target is: its interior cell or worldspace, and
/// position (where scripts have moved it, else as placed); for "near the
/// editor location", where `actor` was placed in the editor.
pub fn target_place(
    order: &LoadOrder,
    state: &GameState,
    actor: FormId,
    package: &Package,
) -> Option<(FormId, [f32; 3])> {
    let target = match (followed(package), package.location) {
        (Some((who, _)), _) => who,
        (None, Some(loc)) if loc.kind == 0 => loc.form,
        (None, Some(loc)) if loc.kind == 3 => {
            let placed = crate::scripting::whereabouts(order, actor)?;
            return Some((placed.world.unwrap_or(placed.cell), placed.position));
        }
        _ => return None,
    };
    let (space, _, position, _) = state.place(order, target)?;
    Some((space, position))
}

/// A load door someone can take toward another place: the door to walk
/// to, and where it puts them (the place, the cell, position and heading,
/// radians clockwise from north).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DoorWay {
    pub door: FormId,
    pub at: [f32; 3],
    pub to_space: FormId,
    pub to_cell: FormId,
    pub to: [f32; 3],
    pub heading: f32,
}

/// The load door nearest `actor` that leads straight into `space` (an
/// interior cell or a worldspace): one in the cell they're in whose far
/// side (`XTEL`) is there, else, when `space` is an interior, one of its
/// doors' far sides that stands where they are (the door outside a
/// building). One door only: places two doors away aren't reached. How the
/// game's people find their way between places isn't traced.
pub fn door_toward(
    order: &LoadOrder,
    state: &GameState,
    actor: FormId,
    space: FormId,
) -> Option<DoorWay> {
    let (here_space, here_cell, here, _) = state.place(order, actor)?;
    let space_of = |r: FormId| {
        let w = crate::scripting::whereabouts(order, r)?;
        Some((w.world.unwrap_or(w.cell), w.cell))
    };
    let mut ways: Vec<DoorWay> = Vec::new();
    // A door here whose far side is there.
    for rr in order.references_in_cell(here_cell) {
        let Some(t) = crate::placement::teleport_of(&rr) else {
            continue;
        };
        let Some((to_space, to_cell)) = space_of(t.door) else {
            continue;
        };
        if to_space != space {
            continue;
        }
        let Some(w) = crate::scripting::whereabouts(order, rr.form_id) else {
            continue;
        };
        ways.push(DoorWay {
            door: rr.form_id,
            at: w.position,
            to_space,
            to_cell,
            to: t.position,
            heading: t.rotation[2],
        });
    }
    // Into an interior: its doors' far sides that stand here.
    if ways.is_empty()
        && order
            .get(space)
            .is_some_and(|r| r.entry.header.kind == esm::sig::CELL)
    {
        for rr in order.references_in_cell(space) {
            let Some(t) = crate::placement::teleport_of(&rr) else {
                continue;
            };
            let outside = t.door;
            if space_of(outside).map(|s| s.0) != Some(here_space) {
                continue;
            }
            let Some(w) = crate::scripting::whereabouts(order, outside) else {
                continue;
            };
            let Some(back) = order
                .get(outside)
                .and_then(|o| crate::placement::teleport_of(&o))
            else {
                continue;
            };
            ways.push(DoorWay {
                door: outside,
                at: w.position,
                to_space: space,
                to_cell: space,
                to: back.position,
                heading: back.rotation[2],
            });
        }
    }
    let d = |p: [f32; 3]| (0..3).map(|i| (p[i] - here[i]).powi(2)).sum::<f32>();
    ways.into_iter().min_by(|a, b| d(a.at).total_cmp(&d(b.at)))
}

/// The navmeshes people out of sight walk on, loaded once each: an
/// interior's whole navmesh, outdoors the 3 × 3 squares around a square.
#[derive(Default)]
pub struct NavCache {
    meshes: HashMap<(FormId, Option<(i32, i32)>), NavMesh>,
    grids: HashMap<FormId, Option<crate::WorldGrid>>,
    /// The navmesh info map (`navinfo`).
    pub infos: navinfo::NavInfos,
    /// Each walker's planned path: the place, the goal, where they stood
    /// after the last update, and the nodes still ahead.
    plans: HashMap<FormId, Plan>,
}

struct Plan {
    space: FormId,
    to: [f32; 3],
    at: [f32; 3],
    ahead: Vec<[f32; 3]>,
}

impl NavCache {
    /// The points still ahead of `who` (at `here` in `space`) on the way
    /// to `to`: their planned path's nodes (`navinfo::NavInfos::
    /// virtual_path`), kept from update to update as the game keeps the
    /// path's solution and planned anew when the goal or place changes or
    /// something else moved them. Where the place has no navmesh infos
    /// (data without `NAVI`, such as generated test worlds) the navmesh
    /// path around `here` is taken as before (unresolved: the game would
    /// have no pathing location). `None`: no way there.
    fn ahead(
        &mut self,
        order: &LoadOrder,
        space: FormId,
        who: FormId,
        here: [f32; 3],
        to: [f32; 3],
    ) -> Option<&mut Vec<[f32; 3]>> {
        let fresh = !self.plans.get(&who).is_some_and(|p| {
            p.space == space && distance2(p.to, to) <= 1.0 && distance2(p.at, here) <= 1.0
        });
        if fresh {
            self.plans.remove(&who);
            let ahead = match self.infos.virtual_path(order, space, here, to) {
                Some(nodes) => nodes.iter().skip(1).map(|n| n.position).collect(),
                None if self.infos.info_at(order, space, here).is_none()
                    || self.infos.info_at(order, space, to).is_none() =>
                {
                    self.around(order, space, here).path(here, to)?[1..].to_vec()
                }
                None => return None,
            };
            self.plans.insert(
                who,
                Plan {
                    space,
                    to,
                    at: here,
                    ahead,
                },
            );
        }
        self.plans.get_mut(&who).map(|p| &mut p.ahead)
    }

    /// Where `who` got to after an update (their plan goes on from there).
    fn walked(&mut self, who: FormId, at: [f32; 3]) {
        if let Some(p) = self.plans.get_mut(&who) {
            p.at = at;
        }
    }

    /// Their plan is over (arrived, or through a door).
    fn forget(&mut self, who: FormId) {
        self.plans.remove(&who);
    }

    /// The navmesh around a point in a place (an interior cell or a
    /// worldspace).
    pub fn around(&mut self, order: &LoadOrder, space: FormId, at: [f32; 3]) -> &NavMesh {
        let interior = order
            .get(space)
            .is_some_and(|r| r.entry.header.kind == esm::sig::CELL);
        let key = if interior {
            (space, None)
        } else {
            (space, Some(crate::square_of(at)))
        };
        if !self.meshes.contains_key(&key) {
            let mesh = match key.1 {
                None => NavMesh::load(order, space),
                Some((x, y)) => {
                    let grid = self
                        .grids
                        .entry(space)
                        .or_insert_with(|| crate::WorldGrid::load(order, space).ok());
                    let cells: Vec<FormId> = grid
                        .as_ref()
                        .map(|g| {
                            (-1..=1)
                                .flat_map(|dx| (-1..=1).map(move |dy| (x + dx, y + dy)))
                                .filter_map(|s| g.cells.get(&s).copied())
                                .collect()
                        })
                        .unwrap_or_default();
                    NavMesh::load_cells(order, &cells)
                }
            };
            self.meshes.insert(key, mesh);
        }
        &self.meshes[&key]
    }
}

/// What an update out of sight did ([`move_offstage`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Offstage {
    /// Nowhere to go, or no way there.
    Stayed,
    /// Walked on (and through doors, if any).
    Moved,
    /// At their package's place.
    Arrived,
}

/// The most doors someone out of sight goes through in one update.
const MOST_DOORS: usize = 4;

/// One update of someone out of sight (`009ea8a0`, the virtual path
/// handler, with the low process's travel `0090ad40`): they walk `distance`
/// ([`crate::movement::offstage_speed`] × [`crate::movement::
/// offstage_seconds`]) along their planned path toward their package's
/// place ([`destination`]): the virtual nodes of the navmesh info route
/// ([`navinfo`]; the virtual handler walks only those, the detailed path
/// being dropped when an actor leaves the high process, `006d53b0` →
/// `006c93a0`), point to point, stopping partway; toward a
/// place elsewhere to the load door that leads there ([`door_toward`]) and
/// through it (put at its far side, as `009ead90` teleports them), going on
/// with what's left of the distance. No path: they stay. (The game's paths
/// run through doors themselves; here a door is the end of one path and the
/// start of the next.)
pub fn move_offstage(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    distance: f32,
    navs: &mut NavCache,
) -> Offstage {
    let Some(package) = current_package(order, state, who) else {
        return Offstage::Stayed;
    };
    let mut left = distance;
    let mut moved = false;
    let result = |moved: bool| {
        if moved {
            Offstage::Moved
        } else {
            Offstage::Stayed
        }
    };
    for _ in 0..MOST_DOORS {
        let Some((space, _, here, heading)) = state.place(order, who) else {
            break;
        };
        if let Some((to, radius)) = destination(order, state, who, &package) {
            // At the place: facing an `XMarkerHeading`'s heading (or their
            // editor heading), set at once (`0090ad40`).
            let facing = arrival_heading(order, state, who, &package);
            if crate::movement::arrived(here, to, radius) {
                navs.forget(who);
                if let Some(h) = facing.filter(|h| *h != heading) {
                    state.positions.insert(who, (here, h));
                }
                travel_done(state, who, &package);
                return if moved {
                    Offstage::Moved
                } else {
                    Offstage::Arrived
                };
            }
            let Some(ahead) = navs.ahead(order, space, who, here, to) else {
                return result(moved);
            };
            let w = navinfo::walk_nodes(ahead, here, heading, left, radius);
            if w.done {
                navs.forget(who);
                state
                    .positions
                    .insert(who, (w.at, facing.unwrap_or(w.heading)));
                travel_done(state, who, &package);
                return Offstage::Arrived;
            }
            navs.walked(who, w.at);
            state.positions.insert(who, (w.at, w.heading));
            return Offstage::Moved;
        }
        let Some((target_space, _)) = target_place(order, state, who, &package) else {
            break;
        };
        if target_space == space {
            break;
        }
        let Some(way) = door_toward(order, state, who, target_space) else {
            break;
        };
        let Some(ahead) = navs.ahead(order, space, who, here, way.at) else {
            break;
        };
        let w = navinfo::walk_nodes(ahead, here, heading, left, 0.0);
        if !w.done {
            navs.walked(who, w.at);
            state.positions.insert(who, (w.at, w.heading));
            return Offstage::Moved;
        }
        navs.forget(who);
        left = w.left;
        state.stand(who);
        state.spaces.insert(who, (way.to_space, way.to_cell));
        state.positions.insert(who, (way.to, way.heading));
        moved = true;
    }
    result(moved)
}

/// A travel package out of sight at its place: its procedures reach
/// `DONE`, its end action (`0090ad40` → process vfunc +0x5a0), once.
fn travel_done(state: &mut GameState, who: FormId, package: &Package) {
    if package.kind == kinds::TRAVEL {
        actions::end(state, who, package.form_id);
    }
}

/// People scripts or doors have taken into a place from elsewhere: those
/// the state has in `space` (an interior cell or a worldspace) whose
/// placement is in another interior or worldspace, so loading the place
/// doesn't already bring them.
pub fn moved_into(order: &LoadOrder, state: &GameState, space: FormId) -> Vec<FormId> {
    let mut out: Vec<FormId> = state
        .spaces
        .iter()
        .filter(|(_, (s, _))| *s == space)
        .map(|(r, _)| *r)
        .filter(|&r| {
            crate::scripting::whereabouts(order, r)
                .is_some_and(|w| w.world.unwrap_or(w.cell) != space)
        })
        .filter(|r| {
            order.get(*r).is_some_and(|rr| {
                rr.entry.header.kind == esm::sig::ACHR || rr.entry.header.kind == esm::sig::ACRE
            })
        })
        .collect();
    out.sort();
    out
}

/// People (`ACHR`/`ACRE`) of one exterior grid square — its cell's and the
/// worldspace's persistent ones standing in it — that loading the square
/// with these enable states leaves out as disabled
/// ([`crate::placement::enabled_now`], the cell load's "disabled when the
/// cell loads").
pub fn disabled_people_in_square(
    order: &LoadOrder,
    grid: &crate::WorldGrid,
    square: (i32, i32),
    disabled: &crate::placement::Disabled,
) -> Vec<FormId> {
    let Some(cell) = grid.cell_at(square) else {
        return Vec::new();
    };
    let persistent = grid
        .persistent_in(square)
        .iter()
        .filter_map(|&id| order.get(id));
    let mut out: Vec<FormId> = order
        .references_in_cell(cell)
        .into_iter()
        .chain(persistent)
        .filter(|rr| {
            !rr.entry.header.is_deleted()
                && (rr.entry.header.kind == esm::sig::ACHR
                    || rr.entry.header.kind == esm::sig::ACRE)
        })
        .map(|rr| rr.form_id)
        .filter(|&r| !crate::placement::enabled_now(order, r, disabled))
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Of the people a loaded place left out as disabled
/// ([`disabled_people_in_square`]), those a script has enabled since —
/// themselves, or through their enable parent, as `GoodspringsPowderGangMarker`
/// (00105D4C) brings in the Powder Gangers of Ghost Town Gunfight — that are
/// alive and still placed in `space`. The game loads an enabled reference's
/// 3D (`Enable`, 005c43d0, then `Load3D`; `docs/SCRIPTS_RUNTIME.md`), so
/// they come into the place now.
pub fn enabled_since_load(
    order: &LoadOrder,
    state: &GameState,
    space: FormId,
    left_out: &[FormId],
) -> Vec<FormId> {
    let mut out: Vec<FormId> = left_out
        .iter()
        .copied()
        .filter(|r| !state.dead.contains(r))
        .filter(|&r| crate::placement::enabled_now(order, r, &state.disabled))
        .filter(|&r| state.place(order, r).is_some_and(|p| p.0 == space))
        .collect();
    out.sort();
    out.dedup();
    out
}

/// How far from the navmesh a path may start or end (a person or marker
/// standing just off its edge).
pub const OFF_MESH: f32 = 128.0;

/// How many times a path request searches and smooths (request +0xa8, 3
/// from its constructor `006e2420`).
const PATH_TRIES: usize = 3;

/// A cell's navmesh: every `NAVM` in it, joined.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NavMesh {
    pub vertices: Vec<[f32; 3]>,
    pub triangles: Vec<NavTriangle>,
    /// Door portals (`NVDP`, 8 bytes each: the door reference, its
    /// triangle, 2 unused): the triangles a path crosses a door on, by
    /// triangle (indices into `triangles`). Someone walking onto one
    /// opens the door if it's shut (`world::doors`, `009e20c0`).
    pub door_portals: HashMap<usize, FormId>,
    /// Which `NAVM` each triangle came from: the navmesh's form ID and its
    /// first triangle, in order (the navmesh a point is on picks its
    /// navmesh info, [`navinfo`]).
    pub owners: Vec<(FormId, usize)>,
    /// Obstacles marked on triangles at run time (someone stuck there,
    /// [`navsearch::NavObstacle`]).
    pub obstacles: Vec<navsearch::NavObstacle>,
    /// The terrain's heights (33 × 33, `LAND`) of the exterior squares the
    /// navmesh is of, by square: the straight-line test asks how high the
    /// ends stand above the land ([`Self::land_height`]).
    pub land: HashMap<(i32, i32), Vec<f32>>,
    /// Triangles under a closed door, flagged [`navsearch::DOOR`] at run
    /// time (`006997e0`, [`doors`]), by triangle: the door.
    pub door_triangles: HashMap<usize, FormId>,
    /// The doors marked closed, including those over no triangle (a gate
    /// standing off the navmesh), so they aren't marked again each frame.
    pub doors_closed: std::collections::BTreeSet<FormId>,
    /// What walkers may do with each door now (`006a6fa0`'s door test):
    /// set by whoever knows the doors' locks; a door not listed is walked
    /// through.
    pub door_rules: HashMap<FormId, navsearch::DoorWay>,
    /// The world's collision for the ray-cast ways onto and off the
    /// navmesh ([`offmesh`]); none: no such way.
    pub pick: Picker,
    /// `fJumpFallHeightMin` (exe default 256): how far the ray-cast way may
    /// drop at a step ([`offmesh`]).
    pub fall_height: f32,
    /// Which triangles lie over each square of a grid, for finding the one
    /// a point stands on without looking at all of them; made the first
    /// time it's asked for.
    pub index: TriangleIndex,
}

/// What's worked out once from a mesh, the first time it's asked for (a
/// cache of the mesh: never compared): the triangles by the squares of a
/// grid ([`GRID_SQUARE`] units across) their outline overlaps, seen from
/// above (grown by [`GRID_MARGIN`], as [`height_in`] takes points a hair
/// outside a triangle), each square's in the mesh's order; each
/// triangle's island, the triangles joined to it through their neighbours
/// (no path leaves one); and the squares' extent.
#[derive(Debug, Clone, Default)]
pub struct TriangleIndex(
    std::sync::OnceLock<HashMap<(i32, i32), Vec<usize>>>,
    std::sync::OnceLock<Vec<usize>>,
    std::sync::OnceLock<((i32, i32), (i32, i32))>,
);

impl PartialEq for TriangleIndex {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

/// The index's squares, in game units.
const GRID_SQUARE: f32 = 256.0;
/// How far past a triangle's outline it's indexed.
const GRID_MARGIN: f32 = 1.0;

fn grid_square(x: f32, y: f32) -> (i32, i32) {
    (
        (x / GRID_SQUARE).floor() as i32,
        (y / GRID_SQUARE).floor() as i32,
    )
}

/// Casts a ray through the world's collision: how far along from `from`
/// toward `to` (0..1) it meets something, if it does. The game's path ray
/// casts (`006e6f90`) are `bhkPickData` picks on layer 38 (`PATHPICK`,
/// `006e6c00`).
pub trait PathPick: Send + Sync {
    fn pick(&self, from: [f32; 3], to: [f32; 3]) -> Option<f32>;
}

/// A navmesh's [`PathPick`], if it has one.
#[derive(Clone, Default)]
pub struct Picker(pub Option<std::sync::Arc<dyn PathPick>>);

impl std::fmt::Debug for Picker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0.is_some() {
            "Picker(Some)"
        } else {
            "Picker(None)"
        })
    }
}

impl PartialEq for Picker {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => std::sync::Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NavTriangle {
    pub vertices: [usize; 3],
    /// The triangle across each edge (edge `i` runs from vertex `i` to
    /// vertex `i + 1`).
    pub neighbors: [Option<usize>; 3],
    /// Its flags as the game holds them: `NVTR`'s flags u16 and cover flags
    /// u16 read as one u32 (`00692950` reads the 16 bytes straight into the
    /// triangle; the search tests the u32 at +0xc, `00691140`). Bits used:
    /// [`navsearch::NO_LARGE_CREATURES`], [`navsearch::PREFERRED`],
    /// [`navsearch::WATER`], [`navsearch::DOOR`], and
    /// [`navsearch::OBSTACLE`] set at run time.
    pub flags: u32,
    /// Edges whose link to another navmesh is of kind 1 (the `NVEX`
    /// entry's first u32): the search doesn't cross them (`006a6fa0` tests
    /// the link record `0068f230` returns).
    pub closed: u8,
    /// Edges the data links to something (another triangle of this
    /// navmesh, or of another one, loaded or not): an edge with neither a
    /// link nor a loaded neighbour is an open edge, the navmesh's border
    /// (`0068f2c0` is −1), which the ray-cast way onto the navmesh aims
    /// at ([`offmesh`]).
    pub linked: u8,
}

impl NavMesh {
    /// A cell's navmesh.
    pub fn load(order: &LoadOrder, cell: FormId) -> NavMesh {
        NavMesh::load_cells(order, &[cell])
    }

    /// The navmeshes of several cells (the outdoor squares around the
    /// player), joined where they connect. An edge whose bit is set in the
    /// triangle's flags (0x1 edge 0, 0x2 edge 1, 0x4 edge 2) leads to
    /// another navmesh: its link counts into the external connections
    /// (`NVEX`, 10 bytes each: 4 unknown, the navmesh's form ID, the
    /// triangle in it). Checked on Goodsprings' square: 21 + 24 + 8 flagged
    /// edges for its 53 external connections, and their links all below
    /// 53.
    pub fn load_cells(order: &LoadOrder, cells: &[FormId]) -> NavMesh {
        let records: Vec<esm::RecordRef> = cells
            .iter()
            .flat_map(|&cell| order.in_cell(cell))
            .filter(|rr| rr.entry.header.kind == NAVM)
            .collect();
        NavMesh::from_records(&records)
    }

    /// Particular navmeshes (`NAVM` form IDs), joined where they connect.
    pub fn load_navmeshes(order: &LoadOrder, navmeshes: &[FormId]) -> NavMesh {
        let records: Vec<esm::RecordRef> = navmeshes
            .iter()
            .filter_map(|&n| order.get(n))
            .filter(|rr| rr.entry.header.kind == NAVM)
            .collect();
        NavMesh::from_records(&records)
    }

    /// The navmesh a triangle belongs to.
    pub fn owner_of(&self, triangle: usize) -> Option<FormId> {
        self.owners
            .iter()
            .take_while(|(_, first)| *first <= triangle)
            .last()
            .map(|(form, _)| *form)
    }

    /// The navmesh a point stands on (as [`Self::triangle_at`] finds its
    /// triangle), if the point is within [`OFF_MESH`] of it.
    pub fn navmesh_at(&self, p: [f32; 3]) -> Option<FormId> {
        self.triangle_near(p).and_then(|t| self.owner_of(t))
    }

    /// The point of the navmesh nearest `p` (seen from above, at the
    /// triangle's height), on the triangle whose height there is nearest
    /// `p`'s among the nearest ones. The game resolves a path location to
    /// its closest navmesh triangle (`PathingLocation::
    /// ResolveToClosestNavmeshAndTriangle` (Xbox PDB)); how it weighs
    /// height against distance isn't traced.
    pub fn closest_point(&self, p: [f32; 3]) -> Option<[f32; 3]> {
        self.closest_point_on(p, None)
    }

    /// [`Self::closest_point`] among one navmesh's triangles (a path node
    /// for a navmesh info is resolved onto that navmesh), or all of them.
    pub fn closest_point_on(&self, p: [f32; 3], navmesh: Option<FormId>) -> Option<[f32; 3]> {
        let range = match navmesh {
            None => 0..self.triangles.len(),
            Some(n) => {
                let i = self.owners.iter().position(|(form, _)| *form == n)?;
                let first = self.owners[i].1;
                let end = self.owners.get(i + 1).map_or(self.triangles.len(), |o| o.1);
                first..end
            }
        };
        let mut best: Option<(f32, f32, [f32; 3])> = None;
        for t in range {
            let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
            let q = closest_in_triangle(a, b, c, p);
            let flat = (q[0] - p[0]).powi(2) + (q[1] - p[1]).powi(2);
            let dz = (q[2] - p[2]).abs();
            let better = match best {
                None => true,
                Some((f, z, _)) => flat < f - 1e-3 || ((flat - f).abs() <= 1e-3 && dz < z),
            };
            if better {
                best = Some((flat, dz, q));
            }
        }
        best.map(|(_, _, q)| q)
    }

    fn from_records(records: &[esm::RecordRef]) -> NavMesh {
        struct Part {
            form: FormId,
            first_vertex: usize,
            first_triangle: usize,
            raw: Vec<([u16; 3], [u16; 3], u32)>,
            external: Vec<(u32, FormId, u16)>,
            doors: Vec<(FormId, u16)>,
        }
        let mut mesh = NavMesh::default();
        let mut parts = Vec::new();
        {
            for rr in records {
                if rr.entry.header.kind != NAVM || rr.entry.header.is_deleted() {
                    continue;
                }
                // Decoded once and kept (`record_shared`): outdoors the
                // squares' navmeshes are joined again whenever the player
                // crosses into another square.
                let Ok(record) = rr.record_shared() else {
                    continue;
                };
                let (Some(vx), Some(tr)) = (record.get(NVVX), record.get(NVTR)) else {
                    continue;
                };
                let first_vertex = mesh.vertices.len();
                mesh.vertices.extend(
                    vx.data
                        .chunks_exact(12)
                        .map(|c| [le_f32(c, 0), le_f32(c, 4), le_f32(c, 8)]),
                );
                let raw: Vec<([u16; 3], [u16; 3], u32)> = tr
                    .data
                    .chunks_exact(16)
                    .map(|c| {
                        let u = |i: usize| u16::from_le_bytes([c[i], c[i + 1]]);
                        ([u(0), u(2), u(4)], [u(6), u(8), u(10)], le_u32(c, 12))
                    })
                    .collect();
                let external = record
                    .get(NVEX)
                    .map(|s| {
                        s.data
                            .chunks_exact(10)
                            .map(|c| {
                                (
                                    le_u32(c, 0),
                                    rr.plugin.to_global(FormId(le_u32(c, 4))),
                                    u16::from_le_bytes([c[8], c[9]]),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let doors = record
                    .get(NVDP)
                    .map(|s| {
                        s.data
                            .chunks_exact(8)
                            .map(|c| {
                                (
                                    rr.plugin.to_global(FormId(le_u32(c, 0))),
                                    u16::from_le_bytes([c[4], c[5]]),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                parts.push(Part {
                    form: rr.form_id,
                    first_vertex,
                    first_triangle: 0,
                    raw,
                    external,
                    doors,
                });
            }
        }
        let mut next = 0;
        for p in &mut parts {
            p.first_triangle = next;
            next += p.raw.len();
        }
        let start_of: HashMap<FormId, (usize, usize)> = parts
            .iter()
            .map(|p| (p.form, (p.first_triangle, p.raw.len())))
            .collect();
        let vertex_count = mesh.vertices.len();
        for p in &parts {
            let triangles = p.raw.len();
            mesh.owners.push((p.form, p.first_triangle));
            for &(v, n, flags) in &p.raw {
                let vertices = v.map(|v| (p.first_vertex + usize::from(v)).min(vertex_count - 1));
                let mut neighbors = [None; 3];
                let mut closed = 0u8;
                let mut linked = 0u8;
                for e in 0..3 {
                    let link = n[e];
                    if link == 0xFFFF {
                        continue;
                    }
                    linked |= 1 << e;
                    neighbors[e] = if flags & (1 << e) != 0 {
                        // To another navmesh, if it's loaded.
                        p.external
                            .get(usize::from(link))
                            .and_then(|(kind, form, t)| {
                                if *kind == 1 {
                                    closed |= 1 << e;
                                }
                                let (first, count) = *start_of.get(form)?;
                                (usize::from(*t) < count).then(|| first + usize::from(*t))
                            })
                    } else {
                        (usize::from(link) < triangles)
                            .then(|| p.first_triangle + usize::from(link))
                    };
                }
                mesh.triangles.push(NavTriangle {
                    vertices,
                    neighbors,
                    flags,
                    closed,
                    linked,
                });
            }
            for &(door, t) in &p.doors {
                if usize::from(t) < triangles && door.0 != 0 {
                    mesh.door_portals
                        .insert(p.first_triangle + usize::from(t), door);
                }
            }
        }
        mesh
    }

    fn corner(&self, t: usize, i: usize) -> [f32; 3] {
        self.vertices[self.triangles[t].vertices[i % 3]]
    }

    /// The triangles over each grid square ([`TriangleIndex`]).
    fn index(&self) -> &HashMap<(i32, i32), Vec<usize>> {
        self.index.0.get_or_init(|| {
            let mut squares: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
            for t in 0..self.triangles.len() {
                let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
                let lo = grid_square(
                    a[0].min(b[0]).min(c[0]) - GRID_MARGIN,
                    a[1].min(b[1]).min(c[1]) - GRID_MARGIN,
                );
                let hi = grid_square(
                    a[0].max(b[0]).max(c[0]) + GRID_MARGIN,
                    a[1].max(b[1]).max(c[1]) + GRID_MARGIN,
                );
                for x in lo.0..=hi.0 {
                    for y in lo.1..=hi.1 {
                        squares.entry((x, y)).or_default().push(t);
                    }
                }
            }
            squares
        })
    }

    /// Each triangle's island ([`TriangleIndex`]): the lowest triangle
    /// joined to it.
    fn islands(&self) -> &[usize] {
        self.index.1.get_or_init(|| {
            let mut parent: Vec<usize> = (0..self.triangles.len()).collect();
            fn root(parent: &mut [usize], mut t: usize) -> usize {
                while parent[t] != t {
                    parent[t] = parent[parent[t]];
                    t = parent[t];
                }
                t
            }
            for t in 0..self.triangles.len() {
                for n in self.triangles[t].neighbors.into_iter().flatten() {
                    if n >= parent.len() {
                        continue;
                    }
                    let (a, b) = (root(&mut parent, t), root(&mut parent, n));
                    if a != b {
                        parent[a.max(b)] = a.min(b);
                    }
                }
            }
            (0..self.triangles.len())
                .map(|t| root(&mut parent, t))
                .collect()
        })
    }

    /// Whether a path could join two triangles (the same island).
    fn joined(&self, a: usize, b: usize) -> bool {
        let islands = self.islands();
        islands.get(a) == islands.get(b)
    }

    fn centroid(&self, t: usize) -> [f32; 3] {
        let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
        [0, 1, 2].map(|k| (a[k] + b[k] + c[k]) / 3.0)
    }

    /// The triangle under a point, if the point is on (or within
    /// [`OFF_MESH`] of) the navmesh.
    fn triangle_near(&self, p: [f32; 3]) -> Option<usize> {
        let t = self.triangle_at(p)?;
        let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
        if height_in(a, b, c, p).is_some_and(|z| (z - p[2]).abs() < 200.0) {
            return Some(t);
        }
        // Nearest corner within reach.
        let near = [a, b, c, self.centroid(t)]
            .iter()
            .map(|q| distance2(*q, p))
            .fold(f32::INFINITY, f32::min);
        (near <= OFF_MESH * OFF_MESH).then_some(t)
    }

    /// The triangles whose outline (seen from above, grown by the grid's
    /// margin) comes within `reach` of a point across, in the mesh's order:
    /// every triangle with a corner within `reach` of it, or holding it,
    /// is among them (others nearby may be too).
    pub fn triangles_near(&self, p: [f32; 3], reach: f32) -> Vec<usize> {
        let index = self.index();
        let lo = grid_square(p[0] - reach, p[1] - reach);
        let hi = grid_square(p[0] + reach, p[1] + reach);
        let mut found: Vec<usize> = (lo.0..=hi.0)
            .flat_map(|x| (lo.1..=hi.1).map(move |y| (x, y)))
            .filter_map(|square| index.get(&square))
            .flatten()
            .copied()
            .collect();
        found.sort_unstable();
        found.dedup();
        found
    }

    /// The triangle whose middle is nearest a point (the first of those as
    /// near), looking at the grid's squares ring by ring out from the
    /// point's until no square left can hold a nearer one: a triangle is
    /// in the square of its middle, and a square `r` rings out is at least
    /// `r - 1` squares away across.
    fn nearest_middle(&self, p: [f32; 3]) -> Option<usize> {
        let index = self.index();
        if index.is_empty() {
            return None;
        }
        let ((x0, y0), (x1, y1)) = *self.index.2.get_or_init(|| {
            index.keys().fold(
                ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)),
                |((ax, ay), (bx, by)), &(x, y)| ((ax.min(x), ay.min(y)), (bx.max(x), by.max(y))),
            )
        });
        let (cx, cy) = grid_square(p[0], p[1]);
        let rings = [x0 - cx, cx - x1, y0 - cy, cy - y1]
            .iter()
            .map(|d| d.unsigned_abs())
            .max()
            .unwrap_or(0) as i32;
        let mut best: Option<(f32, usize)> = None;
        let consider = |best: &mut Option<(f32, usize)>, t: usize| {
            let d = distance2(self.centroid(t), p);
            if best.map_or(true, |(bd, bt)| d.total_cmp(&bd).then(t.cmp(&bt)).is_lt()) {
                *best = Some((d, t));
            }
        };
        for r in 0..=rings {
            if let Some((d, _)) = best {
                let gap = (r - 1).max(0) as f32 * GRID_SQUARE;
                if gap * gap > d {
                    break;
                }
            }
            for x in cx - r..=cx + r {
                for y in cy - r..=cy + r {
                    if (x - cx).abs() != r && (y - cy).abs() != r {
                        continue;
                    }
                    for &t in index.get(&(x, y)).into_iter().flatten() {
                        consider(&mut best, t);
                    }
                }
            }
        }
        best.map(|(_, t)| t)
    }

    /// The triangle a point stands on: one whose outline (seen from above)
    /// holds it, nearest in height; else the one whose middle is nearest.
    pub fn triangle_at(&self, p: [f32; 3]) -> Option<usize> {
        let mut best: Option<(f32, usize)> = None;
        // Only those over the point's grid square can hold it.
        let under = self.index().get(&grid_square(p[0], p[1]));
        for &t in under.into_iter().flatten() {
            let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
            if let Some(z) = height_in(a, b, c, p) {
                let dz = (z - p[2]).abs();
                if best.map_or(true, |(d, _)| dz < d) {
                    best = Some((dz, t));
                }
            }
        }
        if let Some((dz, t)) = best {
            if dz < 200.0 {
                return Some(t);
            }
        }
        self.nearest_middle(p)
    }

    /// A path from one point to another over the navmesh: the points to
    /// walk through, ending at `to`. `None` when no path joins them, or
    /// either end is off the navmesh (more than [`OFF_MESH`] from it).
    pub fn path(&self, from: [f32; 3], to: [f32; 3]) -> Option<Vec<[f32; 3]>> {
        self.path_with_doors(from, to).map(|(points, _)| points)
    }

    /// [`Self::path`], and the doors the path goes through: for each door
    /// portal triangle crossed (`NVDP`, in order), the door and the middle
    /// of its triangle. Someone walking the path opens each closed door
    /// when they reach it (`009e20c0`).
    #[allow(clippy::type_complexity)]
    pub fn path_with_doors(
        &self,
        from: [f32; 3],
        to: [f32; 3],
    ) -> Option<(Vec<[f32; 3]>, Vec<(FormId, [f32; 3])>)> {
        self.plan(from, to, &navsearch::PathRequest::default())
    }

    /// A path for a request (`navsearch::PathRequest`: the walker's radius,
    /// avoid nodes, doors), and the doors it goes through. With no avoid
    /// nodes a straight line is tried first (`bUseStraightLineCheckFirst`
    /// 1, `006cc5e0` asks for it only when the request's avoid array is
    /// empty, → `006cd1f0`); then the navmesh search
    /// ([`navsearch`], `006cd670`) and the smoothed line through its
    /// triangles.
    #[allow(clippy::type_complexity)]
    pub fn plan(
        &self,
        from: [f32; 3],
        to: [f32; 3],
        request: &navsearch::PathRequest,
    ) -> Option<(Vec<[f32; 3]>, Vec<(FormId, [f32; 3])>)> {
        // Both ends resolved onto the navmesh (`006dd6f0`), an end off it
        // joined by its ray-cast way ([`offmesh`], `006caa40`/`006cac90`).
        let start_end = self.start_end(from, request.radius)?;
        let goal_end = self.goal_end(to, request.radius, request.target_radius)?;
        let (start, goal) = (start_end.triangle, goal_end.triangle);
        let (a, b) = (start_end.point, goal_end.point);
        // The path: the way onto the navmesh, the navmesh part from `a` to
        // `b`, the way off it.
        let whole = |middle: Vec<[f32; 3]>| {
            let mut out = Vec::with_capacity(middle.len() + 2);
            if !start_end.way.is_empty() {
                out.push(from);
            }
            out.extend(middle);
            if !goal_end.way.is_empty() {
                out.push(to);
            }
            out
        };
        let doors_on = |triangles: &[usize]| -> Vec<(FormId, [f32; 3])> {
            triangles
                .iter()
                .filter_map(|&t| {
                    self.door_portals
                        .get(&t)
                        .or_else(|| self.door_triangles.get(&t))
                        .map(|&d| (d, self.centroid(t)))
                })
                .collect()
        };
        // The straight line: the doors are those of the triangles it
        // crosses.
        if request.avoid.is_empty() {
            if let Some(crossed) = self.line_crossing(a, b, start, goal) {
                if self.wide_line_clear(a, b, request.radius) {
                    return Some((whole(vec![a, b]), doors_on(&crossed)));
                }
            }
        }
        // Different islands (no neighbours join them): the search can't
        // reach the goal, so it isn't run (it would look at every triangle
        // of the first before failing; doors and obstacles only take
        // links away).
        if !self.joined(start, goal) {
            return None;
        }
        // Search and smooth, up to the request's tries (+0xa8: 3 from
        // `006e2420`, kept within 1..10 by `006cc5e0`): a smoothing that
        // failed names edges to keep off, and the search goes again.
        let mut keep_off: Vec<(usize, usize)> = request.avoid_edges.to_vec();
        let mut last = None;
        for _ in 0..PATH_TRIES {
            let request = navsearch::PathRequest {
                avoid_edges: &keep_off,
                ..*request
            };
            let Some(corridor) = self.corridor(start, goal, &request) else {
                break;
            };
            let (found, points, more) = smoother::smooth(self, a, b, &corridor, &request);
            last = Some((found, points, corridor));
            if found {
                break;
            }
            let before = keep_off.len();
            for e in more {
                if !keep_off.contains(&e) {
                    keep_off.push(e);
                }
            }
            if keep_off.len() == before {
                break;
            }
        }
        let (found, points, corridor) = last?;
        // No smoothed way after the tries: the request fails (`006cc5e0`
        // returns 1 or 2; its acceptance of a last try ending near the goal,
        // and a partial path for requests that allow one, +0xa3, aren't
        // followed). A walk short of the place would end as if there.
        if !found {
            return None;
        }
        Some((whole(points), doors_on(&corridor)))
    }

    /// The rest of the straight-line test (`006cd1f0`, after the line
    /// itself): when both ends stand at least 25 above the land under them
    /// (`TES` land height, `0045cbc0`; indoors there is no land, so always),
    /// the lines the walker's radius to either side, end to end, must stay
    /// on the navmesh too (`006d7350` on each). Whether the offset is the
    /// radius along the unit perpendicular, as here, or scaled by the
    /// line's length (`0045bb20` on the cross product) isn't certain.
    // Translated from 006cd1f0 (decompiled, FalloutNV.exe 1.4.0.525).
    fn wide_line_clear(&self, from: [f32; 3], to: [f32; 3], radius: f32) -> bool {
        let above = |p: [f32; 3]| self.land_height(p).map_or(true, |h| p[2] >= h + 25.0);
        if !(above(from) && above(to)) {
            return true;
        }
        let (dx, dy) = (to[0] - from[0], to[1] - from[1]);
        let d = dx.hypot(dy);
        if d < 1e-4 {
            return true;
        }
        let (px, py) = (dy / d * radius, -dx / d * radius);
        [1.0f32, -1.0].iter().all(|s| {
            let a = [from[0] + s * px, from[1] + s * py, from[2]];
            let b = [to[0] + s * px, to[1] + s * py, to[2]];
            self.walk_line(a, b, radius).is_some()
        })
    }

    /// The land's height under a point, from the squares' `LAND` heights
    /// (33 × 33 points 128 apart from the square's south-west corner;
    /// within a square of four points on one of its two halves). `None`
    /// where no land was
    /// given (indoors: the game's land height then is its default, below
    /// everything).
    pub fn land_height(&self, p: [f32; 3]) -> Option<f32> {
        const SQUARE: f32 = 4096.0;
        const STEP: f32 = 128.0;
        let square = crate::square_of(p);
        let heights = self.land.get(&square)?;
        let x = (p[0] - square.0 as f32 * SQUARE) / STEP;
        let y = (p[1] - square.1 as f32 * SQUARE) / STEP;
        let (ix, iy) = ((x.floor() as usize).min(31), (y.floor() as usize).min(31));
        let (fx, fy) = (x - ix as f32, y - iy as f32);
        let h = |i: usize, j: usize| heights.get(j * 33 + i).copied();
        let (sw, se, nw, ne) = (
            h(ix, iy)?,
            h(ix + 1, iy)?,
            h(ix, iy + 1)?,
            h(ix + 1, iy + 1)?,
        );
        // The diagonal in a checkerboard, as `land::Land::quarter_mesh`
        // splits the squares (south-west to north-east where column + row
        // is even).
        Some(if (ix + iy) % 2 == 0 {
            if fx >= fy {
                sw + (se - sw) * fx + (ne - se) * fy
            } else {
                sw + (ne - nw) * fx + (nw - sw) * fy
            }
        } else if fx + fy <= 1.0 {
            sw + (se - sw) * fx + (nw - sw) * fy
        } else {
            ne + (nw - ne) * (1.0 - fx) + (se - ne) * (1.0 - fy)
        })
    }

    /// Gives the navmesh an exterior square's land heights (`LAND`, 33 × 33).
    pub fn set_land(&mut self, square: (i32, i32), heights: Vec<f32>) {
        if heights.len() >= 33 * 33 {
            self.land.insert(square, heights);
        }
    }

    /// A path keeping away from others in the way (`009e5ae0` makes a new
    /// path request with them as avoid nodes, [`crate::movement::
    /// AvoidNode`], weighed by `navsearch::avoid_cost`); no straight line is
    /// tried with avoid nodes.
    pub fn path_avoiding(
        &self,
        from: [f32; 3],
        to: [f32; 3],
        avoid: &[crate::movement::AvoidNode],
    ) -> Option<Vec<[f32; 3]>> {
        let request = navsearch::PathRequest {
            avoid,
            ..Default::default()
        };
        self.plan(from, to, &request).map(|(points, _)| points)
    }

    /// Whether the straight line from `from` (on triangle `start`) to `to`
    /// (on `goal`) stays on the navmesh, crossing from triangle to triangle
    /// through shared edges: the triangles it crosses, in order, when it
    /// does.
    fn line_crossing(
        &self,
        from: [f32; 3],
        to: [f32; 3],
        start: usize,
        goal: usize,
    ) -> Option<Vec<usize>> {
        let mut t = start;
        let mut entered = -1.0f32;
        let mut crossed = Vec::new();
        for _ in 0..=self.triangles.len() {
            crossed.push(t);
            if t == goal {
                return Some(crossed);
            }
            // The edge the line leaves through: the crossing farthest on.
            let mut exit: Option<(f32, Option<usize>)> = None;
            for i in 0..3 {
                let (a, b) = (self.corner(t, i), self.corner(t, i + 1));
                if let Some(s) = crossing(from, to, a, b) {
                    if s > entered + 1e-5 && exit.map_or(true, |(e, _)| s > e) {
                        exit = Some((s, self.triangles[t].neighbors[i]));
                    }
                }
            }
            match exit {
                Some((s, Some(n))) => {
                    entered = s;
                    t = n;
                }
                _ => return None,
            }
        }
        None
    }

    /// The triangles from `start` to `goal` by the game's navmesh search
    /// ([`navsearch`]); none when it doesn't reach the goal (the game then
    /// builds a path to the nearest node and retries, `006cc5e0`: not
    /// followed here).
    fn corridor(
        &self,
        start: usize,
        goal: usize,
        request: &navsearch::PathRequest,
    ) -> Option<Vec<usize>> {
        let (found, route) = self.search(start, goal, request);
        found.then_some(route)
    }

    /// The edge two neighbouring triangles share, as (left, right) seen
    /// going from `a` into `b` (from above, left is counterclockwise).
    /// Leaving a counterclockwise triangle across the edge from its corner
    /// `i` to `i + 1`, corner `i + 1` is on the left; a clockwise one the
    /// other way round.
    fn portal(&self, a: usize, b: usize) -> Option<([f32; 3], [f32; 3])> {
        let i = self.triangles[a]
            .neighbors
            .iter()
            .position(|&n| n == Some(b))?;
        let (p, q) = (self.corner(a, i), self.corner(a, i + 1));
        let counterclockwise =
            cross2(self.corner(a, 0), self.corner(a, 1), self.corner(a, 2)) > 0.0;
        Some(if counterclockwise { (q, p) } else { (p, q) })
    }
}

fn distance2(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum()
}

#[cfg(test)]
fn same(a: [f32; 3], b: [f32; 3]) -> bool {
    distance2(a, b) < 1e-6
}

/// Where the segment `p`–`q` crosses the segment `a`–`b`, seen from above:
/// the fraction along `p`–`q`, if they cross.
fn crossing(p: [f32; 3], q: [f32; 3], a: [f32; 3], b: [f32; 3]) -> Option<f32> {
    let r = [q[0] - p[0], q[1] - p[1]];
    let s = [b[0] - a[0], b[1] - a[1]];
    let denom = r[0] * s[1] - r[1] * s[0];
    if denom.abs() < 1e-9 {
        return None;
    }
    let ap = [a[0] - p[0], a[1] - p[1]];
    let t = (ap[0] * s[1] - ap[1] * s[0]) / denom;
    let u = (ap[0] * r[1] - ap[1] * r[0]) / denom;
    ((-1e-5..=1.0 + 1e-5).contains(&t) && (-1e-5..=1.0 + 1e-5).contains(&u)).then_some(t)
}

/// Seen from above: positive when `c` is to the left of the line from `a`
/// to `b` (counterclockwise).
fn cross2(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

/// The triangle's height under a point, if the point is inside it seen
/// from above.
fn height_in(a: [f32; 3], b: [f32; 3], c: [f32; 3], p: [f32; 3]) -> Option<f32> {
    let d = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);
    if d.abs() < 1e-9 {
        return None;
    }
    let w1 = ((b[1] - c[1]) * (p[0] - c[0]) + (c[0] - b[0]) * (p[1] - c[1])) / d;
    let w2 = ((c[1] - a[1]) * (p[0] - c[0]) + (a[0] - c[0]) * (p[1] - c[1])) / d;
    let w3 = 1.0 - w1 - w2;
    let eps = -1e-4;
    (w1 >= eps && w2 >= eps && w3 >= eps).then(|| w1 * a[2] + w2 * b[2] + w3 * c[2])
}

/// The point of triangle `a b c` nearest `p` seen from above, at the
/// triangle's height there.
fn closest_in_triangle(a: [f32; 3], b: [f32; 3], c: [f32; 3], p: [f32; 3]) -> [f32; 3] {
    if let Some(z) = height_in(a, b, c, p) {
        return [p[0], p[1], z];
    }
    // Nearest on the edges.
    let on_edge = |u: [f32; 3], v: [f32; 3]| {
        let d = [v[0] - u[0], v[1] - u[1]];
        let len2 = d[0] * d[0] + d[1] * d[1];
        let t = if len2 > 1e-9 {
            (((p[0] - u[0]) * d[0] + (p[1] - u[1]) * d[1]) / len2).clamp(0.0, 1.0)
        } else {
            0.0
        };
        [0, 1, 2].map(|k| u[k] + (v[k] - u[k]) * t)
    };
    [on_edge(a, b), on_edge(b, c), on_edge(c, a)]
        .into_iter()
        .min_by(|x, y| {
            let fx = (x[0] - p[0]).powi(2) + (x[1] - p[1]).powi(2);
            let fy = (y[0] - p[0]).powi(2) + (y[1] - p[1]).powi(2);
            fx.total_cmp(&fy)
        })
        .unwrap_or(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An L-shaped corridor: a 100-wide strip going east, then north.
    ///
    /// ```text
    ///   6---7
    ///   |   |
    ///   4---5
    ///   |   |
    ///   2---3 (corner)
    /// 0-1...
    /// ```
    fn corridor_mesh() -> NavMesh {
        // Squares of 100: (0,0)-(100,100), (100,0)-(200,100),
        // (100,100)-(200,200), (100,200)-(200,300).
        let v = vec![
            [0.0, 0.0, 0.0],
            [100.0, 0.0, 0.0],
            [200.0, 0.0, 0.0],
            [0.0, 100.0, 0.0],
            [100.0, 100.0, 0.0],
            [200.0, 100.0, 0.0],
            [100.0, 200.0, 0.0],
            [200.0, 200.0, 0.0],
            [100.0, 300.0, 0.0],
            [200.0, 300.0, 0.0],
        ];
        let t = |vertices: [usize; 3], neighbors: [Option<usize>; 3]| NavTriangle {
            vertices,
            neighbors,
            ..Default::default()
        };
        NavMesh {
            vertices: v,
            triangles: vec![
                // Square A (0,1,4,3): triangles 0 (0,1,4) and 1 (0,4,3).
                t([0, 1, 4], [None, Some(2), Some(1)]),
                t([0, 4, 3], [Some(0), None, None]),
                // Square B (1,2,5,4): 2 (1,5,4), 3 (1,2,5).
                t([1, 5, 4], [Some(3), Some(4), Some(0)]),
                t([1, 2, 5], [None, None, Some(2)]),
                // Square C (4,5,7,6): 4 (4,5,7), 5 (4,7,6).
                t([4, 5, 7], [Some(2), None, Some(5)]),
                t([4, 7, 6], [Some(4), Some(6), None]),
                // Square D (6,7,9,8): 6 (6,7,9)... shares 7-6 with 5.
                t([7, 6, 8], [Some(5), None, Some(7)]),
                t([7, 8, 9], [Some(6), None, None]),
            ],
            // A door across square C's first triangle.
            door_portals: HashMap::from([(4, FormId(0x904))]),
            owners: vec![(FormId(0x901), 0), (FormId(0x902), 4)],
            obstacles: Vec::new(),
            land: HashMap::new(),
            ..Default::default()
        }
    }

    #[test]
    fn a_path_turns_the_corner_round_the_inside_corner() {
        let mesh = corridor_mesh();
        assert_eq!(mesh.triangle_at([20.0, 50.0, 0.0]), Some(1));
        let path = mesh.path([20.0, 50.0, 0.0], [150.0, 280.0, 0.0]).unwrap();
        // From the start to the circle of 1.2 × the request's radius (35)
        // round the inside corner (100,100), round it (the arc longer than
        // 50, so both tangent points stay), then on to the goal
        // (`smoother`).
        assert_eq!(path.len(), 4, "{path:?}");
        for p in &path[1..3] {
            let d = (p[0] - 100.0).hypot(p[1] - 100.0);
            assert!((d - 42.0).abs() < 1e-2, "{path:?}");
        }
        assert!(path[1][1] < 100.0 && path[2][0] > 100.0, "{path:?}");
        assert!(same(path[3], [150.0, 280.0, 0.0]));
        // In a straight line, no corners.
        let straight = mesh.path([150.0, 20.0, 0.0], [150.0, 280.0, 0.0]).unwrap();
        assert_eq!(straight.len(), 2, "{straight:?}");
    }

    #[test]
    fn a_path_names_the_doors_it_goes_through() {
        let mesh = corridor_mesh();
        // Through square C: the door, at its triangle's middle.
        let (_, doors) = mesh
            .path_with_doors([20.0, 50.0, 0.0], [150.0, 280.0, 0.0])
            .unwrap();
        assert_eq!(doors.len(), 1);
        assert_eq!(doors[0].0, FormId(0x904));
        assert!(same(doors[0].1, mesh.centroid(4)), "{:?}", doors[0].1);
        // A straight line (no search, `bUseStraightLineCheckFirst`) through
        // square C names the door too.
        let (line, doors) = mesh
            .path_with_doors([150.0, 20.0, 0.0], [150.0, 280.0, 0.0])
            .unwrap();
        assert_eq!(line.len(), 2, "{line:?}");
        assert_eq!(
            doors.iter().map(|d| d.0).collect::<Vec<_>>(),
            [FormId(0x904)]
        );
        // Within square A: none.
        let (_, none) = mesh
            .path_with_doors([20.0, 50.0, 0.0], [80.0, 20.0, 0.0])
            .unwrap();
        assert!(none.is_empty());
    }

    /// A triangle on its own (an island) is out of reach; the corridor's
    /// own ends still join.
    #[test]
    fn islands_have_no_paths_between_them() {
        let mut mesh = corridor_mesh();
        let first = mesh.vertices.len();
        mesh.vertices
            .extend([[1000.0, 0.0, 0.0], [1100.0, 0.0, 0.0], [1000.0, 100.0, 0.0]]);
        mesh.triangles.push(NavTriangle {
            vertices: [first, first + 1, first + 2],
            neighbors: [None; 3],
            flags: 0,
            closed: 0,
            linked: 0,
        });
        assert!(mesh.path([20.0, 50.0, 0.0], [1030.0, 30.0, 0.0]).is_none());
        assert!(mesh.path([20.0, 50.0, 0.0], [150.0, 250.0, 0.0]).is_some());
        let islands = mesh.islands();
        assert_eq!(islands[0], islands[1]);
        assert_ne!(islands[0], islands[mesh.triangles.len() - 1]);
    }

    /// The smoother's triangle under a point, by the grid, is the one
    /// looking at every triangle found: the first of the nearest in height
    /// whose outline holds the point, within 200.
    #[test]
    fn the_triangle_under_a_point_by_the_grid_is_the_one_every_triangle_gives() {
        for mesh in [corridor_mesh(), four_squares()] {
            let every = |p: [f32; 3]| {
                let mut best: Option<(f32, usize, f32)> = None;
                for t in 0..mesh.triangles.len() {
                    let [a, b, c] = [0, 1, 2].map(|i| mesh.corner(t, i));
                    if let Some(z) = height_in(a, b, c, p) {
                        let dz = (z - p[2]).abs();
                        if best.map_or(true, |(d, ..)| dz < d) {
                            best = Some((dz, t, z));
                        }
                    }
                }
                best.filter(|(dz, ..)| *dz < 200.0).map(|(_, t, z)| (t, z))
            };
            for x in (-120..=520).step_by(7) {
                for y in (-120..=520).step_by(9) {
                    for z in [0.0, 150.0, 199.0, 201.0, 400.0] {
                        let p = [x as f32, y as f32, z];
                        assert_eq!(mesh.triangle_under(p), every(p), "{p:?}");
                    }
                }
            }
            // On the corners and edges themselves, and a hair off them.
            for v in &mesh.vertices {
                for d in [0.0, 0.001, -0.001, 0.5, -0.5] {
                    let p = [v[0] + d, v[1] - d, v[2]];
                    assert_eq!(mesh.triangle_under(p), every(p), "{p:?}");
                }
            }
            for t in 0..mesh.triangles.len() {
                for e in 0..3 {
                    let (a, b) = (mesh.corner(t, e), mesh.corner(t, e + 1));
                    for k in [0.25f32, 0.5, 0.75] {
                        let p = [a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k, a[2]];
                        assert_eq!(mesh.triangle_under(p), every(p), "{p:?}");
                    }
                }
            }
        }
    }

    /// The grid finds the triangle that looking at every one finds: on,
    /// between, above and off the mesh.
    #[test]
    fn the_grid_finds_what_looking_at_every_triangle_finds() {
        for mesh in [corridor_mesh(), four_squares()] {
            let every = |p: [f32; 3]| {
                let mut best: Option<(f32, usize)> = None;
                for t in 0..mesh.triangles.len() {
                    let [a, b, c] = [0, 1, 2].map(|i| mesh.corner(t, i));
                    if let Some(z) = height_in(a, b, c, p) {
                        let dz = (z - p[2]).abs();
                        if best.map_or(true, |(d, _)| dz < d) {
                            best = Some((dz, t));
                        }
                    }
                }
                if let Some((dz, t)) = best {
                    if dz < 200.0 {
                        return Some(t);
                    }
                }
                (0..mesh.triangles.len()).min_by(|&x, &y| {
                    distance2(mesh.centroid(x), p).total_cmp(&distance2(mesh.centroid(y), p))
                })
            };
            for x in (-120..=520).step_by(7) {
                for y in (-120..=520).step_by(9) {
                    for z in [0.0, 150.0, 400.0] {
                        let p = [x as f32, y as f32, z];
                        assert_eq!(mesh.triangle_at(p), every(p), "{p:?}");
                    }
                }
            }
            // Far from the mesh (the nearest middle by the grid's rings).
            for x in (-3000..=3500).step_by(97) {
                for y in (-3000..=3500).step_by(89) {
                    for z in [0.0, 150.0, 400.0, -2000.0] {
                        let p = [x as f32, y as f32, z];
                        assert_eq!(mesh.triangle_at(p), every(p), "{p:?}");
                    }
                }
            }
            // On the corners and edges themselves.
            for v in &mesh.vertices {
                assert_eq!(mesh.triangle_at(*v), every(*v));
            }
        }
    }

    /// Near a point, the grid gives every triangle with a corner within
    /// reach or holding the point, in the mesh's order.
    #[test]
    fn triangles_near_holds_every_one_within_reach() {
        for mesh in [corridor_mesh(), four_squares()] {
            for x in (-300..=700).step_by(37) {
                for y in (-300..=700).step_by(41) {
                    for reach in [0.0, 10.0, 90.0, 300.0, 1000.0] {
                        let p = [x as f32, y as f32, 0.0];
                        let near = mesh.triangles_near(p, reach);
                        assert!(near.windows(2).all(|w| w[0] < w[1]));
                        for t in 0..mesh.triangles.len() {
                            let c = [0, 1, 2].map(|i| mesh.corner(t, i));
                            let close = c.iter().any(|v| (v[0] - p[0]).hypot(v[1] - p[1]) <= reach);
                            if close || height_in(c[0], c[1], c[2], p).is_some() {
                                assert!(near.contains(&t), "{p:?} {reach} {t}");
                            }
                        }
                    }
                }
            }
        }
    }

    /// Four squares of 100, two by two, each as two triangles: from the
    /// bottom left to the top right by the bottom right (triangles 3, 6) or
    /// by the top left (1, 4).
    fn four_squares() -> NavMesh {
        let mut vertices = Vec::new();
        for y in 0..3 {
            for x in 0..3 {
                vertices.push([x as f32 * 100.0, y as f32 * 100.0, 0.0]);
            }
        }
        let t = |vertices: [usize; 3], neighbors: [Option<usize>; 3]| NavTriangle {
            vertices,
            neighbors,
            ..Default::default()
        };
        NavMesh {
            vertices,
            triangles: vec![
                t([0, 1, 4], [None, Some(3), Some(1)]),
                t([0, 4, 3], [Some(0), Some(4), None]),
                t([1, 2, 5], [None, None, Some(3)]),
                t([1, 5, 4], [Some(2), Some(6), Some(0)]),
                t([3, 4, 7], [Some(1), Some(7), Some(5)]),
                t([3, 7, 6], [Some(4), None, None]),
                t([4, 5, 8], [Some(3), None, Some(7)]),
                t([4, 8, 7], [Some(6), None, Some(4)]),
            ],
            door_portals: HashMap::new(),
            owners: Vec::new(),
            obstacles: Vec::new(),
            land: HashMap::new(),
            ..Default::default()
        }
    }

    #[test]
    fn a_straight_line_is_tried_first_and_avoid_nodes_cost_more() {
        let mesh = four_squares();
        // The straight line stays on the navmesh: no search.
        let p = mesh.path([20.0, 30.0, 0.0], [180.0, 175.0, 0.0]).unwrap();
        assert_eq!(p, vec![[20.0, 30.0, 0.0], [180.0, 175.0, 0.0]]);
        // Leaving the navmesh: no line.
        assert!(mesh
            .line_crossing([20.0, 30.0, 0.0], [20.0, 300.0, 0.0], 0, 5)
            .is_none());
        // A* takes one way round; a costly avoid node on its middle square
        // sends it the other way.
        let (start, goal) = (0, 7);
        let plain = mesh.corridor(start, goal, &Default::default()).unwrap();
        let (busy, other) = if plain.contains(&3) { (3, 4) } else { (4, 3) };
        let node = crate::movement::AvoidNode {
            position: mesh.centroid(busy),
            radius: 10.0,
            cost: 2.0,
        };
        let avoiding = navsearch::PathRequest {
            avoid: &[node],
            ..Default::default()
        };
        let round = mesh.corridor(start, goal, &avoiding).unwrap();
        assert!(
            round.contains(&other) && !round.contains(&busy),
            "{round:?}"
        );
        assert!(mesh
            .path_avoiding([66.0, 33.0, 0.0], [133.0, 166.0, 0.0], &[node])
            .is_some());
    }

    #[test]
    fn schedules_cover_their_hours_and_days() {
        let night = Schedule {
            month: -1,
            day_of_week: -1,
            date: 0,
            hour: 22,
            duration: 8,
        };
        assert!(night.covers(0, 3, 1, 23.5));
        assert!(night.covers(0, 3, 1, 5.0));
        assert!(!night.covers(0, 3, 1, 12.0));
        let weekends = Schedule {
            day_of_week: 8,
            ..night
        };
        assert!(weekends.covers(0, 6, 1, 23.0) && !weekends.covers(0, 2, 1, 23.0));
        let any = Schedule {
            month: -1,
            day_of_week: -1,
            date: 0,
            hour: -1,
            duration: 0,
        };
        assert!(any.covers(5, 4, 20, 13.0));
    }
}
