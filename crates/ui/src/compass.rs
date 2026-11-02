//! The compass strip (`00779070`) for menus other than the HUD that carry
//! one: V.A.T.S.'s hit points bracket builds the same compass window from
//! `HUDTemplates.xml` and the game updates it with the same function
//! (`007ec810` calls `00779070` with the V.A.T.S. menu's tiles). The rules
//! are the HUD's (`crate::hud`, where they're described); this takes the
//! tiles as arguments.

use crate::hud::{bearing, CompassActor, CompassMarker};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// A compass window's pieces (the setup's `+0x78`…`+0x84` in V.A.T.S.,
/// `+0x34`…`+0x3c` and the window in the HUD).
#[derive(Debug, Clone, Default)]
pub struct CompassTiles {
    pub window: TileId,
    pub markers: Vec<TileId>,
    pub quests: Vec<TileId>,
    pub player: Option<TileId>,
    pub npcs: Vec<TileId>,
    /// The `_Heading` and `_Distance` user traits.
    pub heading_trait: i32,
    pub distance_trait: i32,
}

/// What the compass shows.
#[derive(Debug, Clone, Default)]
pub struct CompassInput {
    /// Degrees clockwise from north.
    pub heading: f32,
    pub position: [f32; 3],
    pub interior: bool,
    pub markers: Vec<CompassMarker>,
    pub actors: Vec<CompassActor>,
    /// The active quest's targets ([`CompassQuest`]).
    pub quests: Vec<CompassQuest>,
    /// Alpha for full strength (`fHudOpacity` × 255).
    pub opacity: f32,
}

fn distance_sq(a: [f32; 3], b: [f32; 3]) -> f32 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}

/// A quest target the compass points at: the place of the reference it
/// follows (`world::quest_targets`: the first door on the way, else the
/// target).
#[derive(Debug, Clone, PartialEq)]
pub struct CompassQuest {
    pub position: [f32; 3],
}

/// The clock the quest icons blink by (`00778c20`): the game's tick count
/// in milliseconds (`011f6394`+0x14) and the frame's seconds (+0x0c).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlinkClock {
    pub now_ms: f64,
    pub seconds: f32,
}

/// The quest icons' blink settings (`00778c20`), the exe's defaults
/// (`FalloutNV.esm` has no `GMST` for them). Distances are compared
/// squared (`0077f9c0` squares each setting; the distance passed is the
/// squared one).
pub mod blink {
    pub const MAX_DIST: f32 = 2000.0;
    pub const MIN_DIST: f32 = 256.0;
    pub const THRESHOLD_DIST: f32 = 500.0;
    pub const SLOWEST_BLINK_TIME: f32 = 750.0;
    pub const FASTEST_BLINK_TIME: f32 = 1500.0;
    pub const THRESHOLD_BLINK_TIME: f32 = 1000.0;
    pub const LONGEST_PAUSE: f32 = 1500.0;
    pub const SHORTEST_PAUSE: f32 = 50.0;
    pub const THRESHOLD_PAUSE: f32 = 600.0;
}

/// `004b3ab0`: `(b - a) × ((x - from) / (to - from)) + a`.
fn lerp(a: f32, b: f32, from: f32, to: f32, x: f32) -> f32 {
    (b - a) * ((x - from) / (to - from)) + a
}

/// One frame of a quest icon's blinking: the alpha rises, holds at full
/// until its pause has passed, falls, and on reaching nothing starts
/// rising again with a new pause; nearer targets blink faster with
/// shorter pauses. The state is kept on the icon: `_AlphaDown` (0 rising,
/// 1 falling, 3 holding) and the pause's end in `user10` (trait 0x100e).
/// `d2` is the squared distance; `full` the alpha at full strength (the
/// game passes its HUD alpha, `011d979c`, which it also divides by).
// Translated from 00778c20 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn blink_icon(
    ui: &mut Ui,
    icon: TileId,
    alpha_down_trait: i32,
    d2: f32,
    full: f32,
    clock: BlinkClock,
) {
    use blink::*;
    let max = MAX_DIST * MAX_DIST;
    let min = MIN_DIST * MIN_DIST;
    let threshold = THRESHOLD_DIST * THRESHOLD_DIST;
    let d = if d2 > max {
        max
    } else if d2 < min {
        min
    } else {
        d2
    };
    let near = d < threshold;
    let state = ui.number(icon, alpha_down_trait);
    let speed = if near {
        lerp(THRESHOLD_BLINK_TIME, FASTEST_BLINK_TIME, threshold, min, d)
    } else {
        lerp(SLOWEST_BLINK_TIME, THRESHOLD_BLINK_TIME, max, threshold, d)
    };
    let alpha = ui.number(icon, t::ALPHA);
    // speed × full / the HUD alpha × seconds; the two alphas are the same
    // value here (none when it's 0).
    let step = (full != 0.0).then_some(speed * clock.seconds);
    let new = if state == 0.0 {
        step.map_or(0.0, |s| alpha + s)
    } else if state == 1.0 {
        step.map_or(0.0, |s| alpha - s)
    } else if state == 3.0 {
        full
    } else {
        0.0
    };
    ui.set_number(icon, t::ALPHA, new);
    let alpha = ui.number(icon, t::ALPHA);
    let timer = t::USER0 + 10;
    if alpha > full {
        ui.set_number(icon, t::ALPHA, full);
        ui.set_number(icon, alpha_down_trait, 3.0);
    } else if alpha < 0.0 {
        let pause = if near {
            lerp(THRESHOLD_PAUSE, SHORTEST_PAUSE, threshold, min, d)
        } else {
            lerp(LONGEST_PAUSE, THRESHOLD_PAUSE, max, threshold, d)
        };
        ui.set_number(icon, t::ALPHA, 0.0);
        ui.set_number(icon, alpha_down_trait, 0.0);
        ui.set_number(
            icon,
            timer,
            (clock.now_ms.floor() + f64::from(pause)) as f32,
        );
    } else if clock.now_ms.floor() > f64::from(ui.number(icon, timer)) && state == 3.0 {
        ui.set_number(icon, alpha_down_trait, 1.0);
    }
}

/// The quest icons (`00779070`, the loop over the current target list):
/// one per target, in order, while icons last; each points at its bearing
/// clamped to ±`limit` (so a target behind sits at an end rather than
/// hiding) and placed no further right than 70 past where the other icons
/// start fading; `_Distance` gets the squared distance. With `clock`
/// (the HUD's own compass: the game checks its menu's state +0x24 is 1)
/// each blinks ([`blink_icon`]). The game's "load door: full alpha"
/// branch compares the reference's own form type with `DOOR`'s (0x1c),
/// which a reference never has, so every icon blinks. The rest are
/// hidden.
#[allow(clippy::too_many_arguments)]
pub fn place_quests(
    ui: &mut Ui,
    icons: &[TileId],
    traits: [i32; 3],
    quests: &[CompassQuest],
    heading: f32,
    position: [f32; 3],
    half: i32,
    full: f32,
    clock: Option<BlinkClock>,
) {
    let [heading_trait, distance_trait, alpha_down_trait] = traits;
    let limit = 70.0f32;
    let fade_from = half as f32 * 0.8 * 2.0;
    let mut quests = quests.iter();
    for &icon in icons {
        let Some(q) = quests.next() else {
            ui.set_number(icon, t::VISIBLE, 0.0);
            continue;
        };
        ui.set_string(
            icon,
            t::FILENAME,
            "Interface\\HUD\\glow_hud_compass_objective_marker.dds",
        );
        let d2 = distance_sq(q.position, position);
        let rel = bearing(q.position, position, heading).clamp(-limit, limit);
        let x = ((2 * half) as f32 * ((rel + limit) / (2.0 * limit))).min(fade_from + 70.0);
        ui.set_number(icon, heading_trait, x);
        ui.set_number(icon, distance_trait, d2);
        ui.set_number(icon, t::VISIBLE, 1.0);
        if let Some(clock) = clock {
            blink_icon(ui, icon, alpha_down_trait, d2, full, clock);
        }
    }
}

/// One frame of the compass (`00779070`); returns the strip's scroll
/// (`TexScroll`: x offset, y, scale x, y).
pub fn update(ui: &mut Ui, tiles: &CompassTiles, input: &CompassInput) -> [f32; 4] {
    let opacity = input.opacity;
    let heading = input.heading.rem_euclid(360.0);
    let mut offset = heading / 360.0 - 0.15;
    if offset < 0.0 {
        offset += 1.0;
    }
    let compass_w = ui.number(tiles.window, t::WIDTH);
    let half = (compass_w / 2.0) as i32 - 35;
    let limit = 70.0f32;
    let fade_from = half as f32 * 0.8 * 2.0;
    let fade_to = (half * 2) as f32;
    let place = |rel: f32| -> f32 { (2 * half) as f32 * ((rel + limit) / (2.0 * limit)) };
    let fade = |x: f32| -> f32 {
        if x <= fade_from {
            opacity
        } else {
            opacity * (1.0 - (x - fade_from) / (fade_to - fade_from))
        }
    };
    let reach = 20000.0f32;
    let mut markers = input
        .markers
        .iter()
        .filter(|m| distance_sq(m.position, input.position) <= reach * reach);
    for &icon in &tiles.markers {
        let Some(m) = markers.next() else {
            ui.set_number(icon, t::VISIBLE, 0.0);
            continue;
        };
        let file = if m.found {
            "Interface\\HUD\\glow_hud_compass_landmark_discovered.dds"
        } else {
            "Interface\\HUD\\glow_hud_compass_landmark.dds"
        };
        ui.set_string(icon, t::FILENAME, file);
        let rel = bearing(m.position, input.position, heading);
        if rel.abs() >= limit {
            ui.set_number(icon, t::VISIBLE, 0.0);
            continue;
        }
        let x = place(rel);
        ui.set_number(icon, tiles.heading_trait, x);
        // Squared, as the game passes it (`004a7290`, then `fabs`).
        ui.set_number(
            icon,
            tiles.distance_trait,
            distance_sq(m.position, input.position),
        );
        ui.set_number(icon, t::VISIBLE, 1.0);
        ui.set_number(icon, t::ALPHA, fade(x));
    }
    let reach = if input.interior { 1500.0 } else { 3000.0 };
    let mut actors = input
        .actors
        .iter()
        .filter(|a| distance_sq(a.position, input.position) <= reach * reach);
    for &icon in &tiles.npcs {
        let Some(a) = actors.next() else {
            ui.set_number(icon, t::VISIBLE, 0.0);
            continue;
        };
        let rel = bearing(a.position, input.position, heading);
        if rel.abs() >= limit {
            ui.set_number(icon, t::VISIBLE, 0.0);
            continue;
        }
        let x = place(rel);
        ui.set_number(icon, tiles.heading_trait, x + 2.0);
        ui.set_number(icon, t::VISIBLE, 1.0);
        ui.set_number(icon, t::SYSTEMCOLOR, if a.hostile { 2.0 } else { 1.0 });
        ui.set_number(icon, t::ALPHA, fade(x));
    }
    // V.A.T.S.'s compass: placed, not blinking (see [`place_quests`]).
    place_quests(
        ui,
        &tiles.quests,
        [tiles.heading_trait, tiles.distance_trait, 0],
        &input.quests,
        heading,
        input.position,
        half,
        opacity,
        None,
    );
    if let Some(p) = tiles.player {
        ui.set_number(p, t::VISIBLE, 0.0);
    }
    [offset, 0.0, 1.0, 1.0]
}
