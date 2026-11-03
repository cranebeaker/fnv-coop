//! The Pip-Boy's world map: map markers, finding them and travelling to
//! them.
//!
//! A map marker is a placed reference (persistent, in its worldspace's
//! persistent cell) with `XMRK`, then `FNAM` flags (0x01 shown on the map
//! from the start, 0x02 can be travelled to), `FULL` its name, `TNAM` its
//! kind (u8), `XRDS` how close the player must come to find it (Jimmy's
//! Well 500, Crimson Caravan Company 3,500) and `XLKR` where travelling
//! there puts the player (else the marker itself). Finding one is read
//! from the game's code ([`discover`]). Fast travel puts the player at the
//! arrival point at once (the game also passes time on the way; not here).

use std::collections::HashSet;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::scripting::GameState;

const XMRK: FourCC = FourCC::new(b"XMRK");

/// One map marker.
#[derive(Debug, Clone, PartialEq)]
pub struct MapMarker {
    pub reference: FormId,
    pub name: String,
    pub position: [f32; 3],
    /// How close the player must come to find it.
    pub radius: f32,
    pub flags: u8,
    pub kind: u8,
    /// Where travelling there puts the player, when not the marker.
    pub arrival: Option<FormId>,
}

/// `FNAM` flag: on the map from the start.
pub const VISIBLE: u8 = 0x01;

/// The map markers among a cell's references (a worldspace's persistent
/// cell holds its markers), by name.
pub fn markers(order: &LoadOrder, cell: FormId) -> Vec<MapMarker> {
    let mut out = Vec::new();
    for rr in order.references_in_cell(cell) {
        let Ok(record) = rr.record() else { continue };
        if record.get(XMRK).is_none() {
            continue;
        }
        let mut flags = 0;
        let mut seen = false;
        for sub in &record.subrecords {
            if sub.kind == XMRK {
                seen = true;
            } else if seen && sub.kind.as_bytes() == b"FNAM" {
                flags = sub.data.first().copied().unwrap_or(0);
                break;
            }
        }
        let data = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 12);
        let Some(data) = data else { continue };
        let f = |kind: &[u8; 4]| {
            record
                .get(FourCC::new(kind))
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_f32(&s.data, 0))
        };
        let arrival = record
            .get(FourCC::new(b"XLKR"))
            .filter(|s| s.data.len() >= 4)
            .map(|s| {
                let at = if s.data.len() >= 8 { 4 } else { 0 };
                rr.plugin.to_global(FormId(le_u32(&s.data, at)))
            })
            .filter(|f| f.0 != 0);
        out.push(MapMarker {
            reference: rr.form_id,
            name: record
                .full_name()
                .filter(|n| !n.is_empty())
                .unwrap_or_else(|| rr.form_id.to_string()),
            position: [
                le_f32(&data.data, 0),
                le_f32(&data.data, 4),
                le_f32(&data.data, 8),
            ],
            radius: f(b"XRDS").unwrap_or(0.0),
            flags,
            kind: record
                .get(FourCC::new(b"TNAM"))
                .and_then(|s| s.data.first().copied())
                .unwrap_or(0),
            arrival,
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Markers the player now finds, standing at `feet` (`00779070`): those
/// not yet both shown and travellable whose radius the player is inside,
/// measured in 3D; the radius is the marker's `XRDS`, or
/// `iMapMarkerRevealDistance` (1000, the exe's default) when that's 0
/// (`00568cb0`). They're added to the found ones, each worth
/// `iXPRewardDiscoverMapMarker` experience. Nothing is found by the dead.
pub fn discover(
    order: &LoadOrder,
    state: &mut GameState,
    markers: &[MapMarker],
    feet: [f32; 3],
) -> Vec<MapMarker> {
    let mut found = Vec::new();
    if state.dead.contains(&crate::dialogue::PLAYER_REF) {
        return found;
    }
    let reveal =
        crate::scripting::game_setting(order, "iMapMarkerRevealDistance").unwrap_or(1000.0);
    for m in markers {
        // `00779070` awards discovery only when visibility or fast travel
        // changes. A marker already revealed and travelable by ShowMap has
        // neither change and gives no second message or reward.
        if shown(state, m) && can_travel(state, m) {
            continue;
        }
        let radius = if m.radius > 0.0 { m.radius } else { reveal };
        let d: f32 = (0..3)
            .map(|i| (m.position[i] - feet[i]).powi(2))
            .sum::<f32>()
            .sqrt();
        // Strictly inside.
        if d < radius {
            state.discovered.insert(m.reference);
            found.push(m.clone());
            crate::stats::bump(state, crate::stats::LOCATIONS_DISCOVERED, 1);
            // The HUD's quest text: `sDiscoveredText` over the place's
            // name (`0076b960`, `world::quest_text`).
            let text = crate::scripting::game_setting_text(order, "sDiscoveredText")
                .unwrap_or_else(|| "You have discovered".into());
            crate::quest_text::discovered(state, &text, &m.name);
            crate::experience::reward_setting(order, state, "iXPRewardDiscoverMapMarker");
        }
    }
    found
}

/// Whether the map shows a marker: found, on the map from the start, or
/// revealed by a script (`ShowMap`).
pub fn shown(state: &GameState, m: &MapMarker) -> bool {
    state.discovered.contains(&m.reference)
        || state.map_markers.contains(&m.reference)
        || m.flags & VISIBLE != 0
}

/// Whether the player can travel there: found (or revealed and able to be
/// travelled to by its flags).
pub fn can_travel(state: &GameState, m: &MapMarker) -> bool {
    state.discovered.contains(&m.reference)
        || m.flags & 0x03 == 0x03
        || state.map_marker_travel.contains(&m.reference)
        || (m.flags & 0x02 != 0 && state.map_markers.contains(&m.reference))
}

/// Why the player can't fast travel from where they are (`0093d660`), in
/// the game's words: an enemy near (`sNoFastTravelHostileActorsNear`), a
/// script's `EnableFastTravel 0` (`sNoFastTravelScriptBlock`), carrying
/// more than their carry weight (`sNoFastTravelOverencumbered`) unless
/// their perks' "Has Fast Travel Always" (entry point 51, Long Haul) give
/// 1 or more, or an interior without the cell's `DATA` flag 0x04 (most
/// can't be left by fast travel: Doc Mitchell's house, the saloon;
/// `sNoFastTravelCell`). Not checked here: alarms, health damage, being in
/// the air.
pub fn travel_refused(order: &LoadOrder, state: &GameState) -> Option<String> {
    travel_refusal(order, state).map(|(text, _)| text)
}

/// [`travel_refused`] with the refusal's picture: the sad Vault Boy,
/// the neutral one for carrying too much (`0093d660`'s types).
pub fn travel_refusal(order: &LoadOrder, state: &GameState) -> Option<(String, &'static str)> {
    let text = |name: &str, fallback: &str| {
        let text =
            crate::scripting::game_setting_text(order, name).unwrap_or_else(|| fallback.into());
        let icon = crate::message_icon::for_setting(name).unwrap_or(crate::message_icon::NEUTRAL);
        (text, icon)
    };
    if state.enemies_near(order) {
        return Some(text(
            "sNoFastTravelHostileActorsNear",
            "You cannot fast travel when enemies are nearby.",
        ));
    }
    if state.over_encumbered(order, crate::dialogue::PLAYER_REF)
        && crate::perks::apply(
            order,
            state,
            crate::perks::entry::HAS_FAST_TRAVEL_ALWAYS,
            0.0,
        ) < 1.0
    {
        return Some(text(
            "sNoFastTravelOverencumbered",
            "You cannot fast travel while overencumbered.",
        ));
    }
    if !state.set_by_scripts.fast_travel.enabled {
        return Some(text(
            "sNoFastTravelScriptBlock",
            "Fast travel is currently unavailable from this location.",
        ));
    }
    if state.player_world.is_none() {
        let allowed = state
            .player_cell
            .and_then(|c| order.get(c))
            .and_then(|rr| rr.record().ok())
            .and_then(|r| r.get(esm::sig::DATA).and_then(|s| s.data.first().copied()))
            .is_some_and(|flags| flags & 0x04 != 0);
        if !allowed {
            return Some(text(
                "sNoFastTravelCell",
                "You cannot fast travel from this location.",
            ));
        }
    }
    None
}

/// The game hours a trip takes (`0093cdf0`): the route's length over the
/// player's run speed (`fMoveBaseSpeed` 77 × `fMoveRunMult` 4 = 308 units
/// a second), as game time at `TimeScale`; a trip of an hour or more
/// counts whole hours only. The game measures the path its pathfinding
/// would take; here the straight line in the same worldspace (a
/// simplification), and nothing between places (the game uses 0 when it
/// finds no path).
pub fn travel_hours(order: &LoadOrder, state: &GameState, to: [f32; 3], to_space: FormId) -> f32 {
    let (Some(from), Some(space)) = (
        state.player_position,
        state.player_world.or(state.player_cell),
    ) else {
        return 0.0;
    };
    if space != to_space {
        return 0.0;
    }
    let length = (0..3)
        .map(|i| (to[i] - from[i]).powi(2))
        .sum::<f32>()
        .sqrt();
    let setting = |n: &str, d: f32| crate::scripting::game_setting(order, n).unwrap_or(d);
    let speed = setting("fMoveBaseSpeed", 77.0) * setting("fMoveRunMult", 4.0);
    let scale = state.global(order, "TimeScale").unwrap_or(30.0);
    let hours = length / speed.max(1.0) / 3600.0 * scale;
    if hours >= 1.0 {
        hours.floor()
    } else {
        hours
    }
}

/// Travels to a marker: unless refused ([`travel_refused`]), the time the
/// trip takes passes ([`travel_hours`]) and the player goes to its arrival
/// point (else the marker), as a script's `MoveTo` would take them (the
/// viewer loads the place). No random encounters (the game has none
/// either). Returns where they go.
pub fn travel(order: &LoadOrder, state: &mut GameState, m: &MapMarker) -> Result<FormId, String> {
    if let Some(why) = travel_refused(order, state) {
        return Err(why);
    }
    let to = m.arrival.unwrap_or(m.reference);
    let (space, cell, position, _) = state
        .place(order, to)
        .ok_or_else(|| format!("Can't find where {} is.", m.name))?;
    let hours = travel_hours(order, state, position, space);
    if hours > 0.0 {
        let scale = state.global(order, "TimeScale").unwrap_or(30.0).max(1e-3);
        state.advance_clock(order, hours * 3600.0 / scale);
    }
    state.player_world = (space != cell).then_some(space);
    state.player_cell = Some(cell);
    state.player_position = Some(position);
    crate::script_functions::player_moved(state);
    state.events.push(crate::scripting::Event::MoveTo {
        what: crate::dialogue::PLAYER_REF,
        to,
    });
    Ok(to)
}

/// The player's own map marker (`PlayerCharacter` `+0x6f4`, a marker
/// reference the game makes on first use, `00952e60`): the worldspace (or
/// interior cell) it's in and where.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CustomMarker {
    pub space: FormId,
    pub position: [f32; 3],
}

/// Sets (or moves) the player's marker (`00952e60`).
pub fn set_custom_marker(state: &mut GameState, space: FormId, position: [f32; 3]) {
    state.custom_marker = Some(CustomMarker { space, position });
}

/// Removes it (`00952f90`).
pub fn remove_custom_marker(state: &mut GameState) {
    state.custom_marker = None;
}

/// The world map shows its worldspace's picture with a border: a place's
/// share between the map's north-west and south-east corners (`MNAM`'s
/// cells) is × 0.796875 + 0.1015625 on the picture (`0079c380` on the
/// world map's tab: `01075010`, `01075008`), and back × 1.2549020 −
/// 0.12745098 (`0079c450`: `01075020`, `01075018`).
pub const MAP_BORDER_SCALE: f32 = 0.796875;
pub const MAP_BORDER_OFFSET: f32 = 0.1015625;
pub const MAP_UNBORDER_SCALE: f32 = 1.254_902;
pub const MAP_UNBORDER_OFFSET: f32 = 0.127_450_99;

/// A place's point on the world map's picture (0 to 1 across and down)
/// between the corners `nw` and `se`. Translated from 0079c380
/// (decompiled, FalloutNV.exe 1.4.0.525), `MapMenu::WorldToMapCoord`
/// (Xbox PDB), the world map's tab.
pub fn world_to_map(nw: [f32; 2], se: [f32; 2], p: [f32; 2]) -> [f32; 2] {
    let share = [
        (p[0] - nw[0]) / (se[0] - nw[0]),
        (p[1] - nw[1]) / (se[1] - nw[1]),
    ];
    share.map(|s| s * MAP_BORDER_SCALE + MAP_BORDER_OFFSET)
}

/// The place at a point on the world map's picture, height 0 (the game
/// then puts it on the ground, `00588e40`: not here). Translated from
/// 0079c450 (decompiled, FalloutNV.exe 1.4.0.525),
/// `MapMenu::MapToWorldCoord` (Xbox PDB), the world map's tab.
pub fn map_to_world(nw: [f32; 2], se: [f32; 2], at: [f32; 2]) -> [f32; 3] {
    let share = at.map(|a| a * MAP_UNBORDER_SCALE - MAP_UNBORDER_OFFSET);
    [
        (se[0] - nw[0]) * share[0] + nw[0],
        (se[1] - nw[1]) * share[1] + nw[1],
        0.0,
    ]
}

/// Every marker found so far (for saving).
pub fn found(state: &GameState) -> &HashSet<FormId> {
    &state.discovered
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `0079c380` / `0079c450`: the picture's border, and back again.
    #[test]
    fn places_and_points_on_the_world_maps_picture() {
        let (nw, se) = ([-4096.0, 4096.0], [4096.0, -4096.0]);
        // The middle stays the middle; the corners land on the border.
        assert_eq!(world_to_map(nw, se, [0.0, 0.0]), [0.5, 0.5]);
        assert_eq!(world_to_map(nw, se, nw), [0.1015625, 0.1015625]);
        let back = map_to_world(nw, se, world_to_map(nw, se, [1000.0, -2000.0]));
        assert!((back[0] - 1000.0).abs() < 0.5 && (back[1] + 2000.0).abs() < 0.5);
        assert_eq!(back[2], 0.0);
        let mut state = GameState::default();
        set_custom_marker(&mut state, FormId(0xDA726), [1.0, 2.0, 0.0]);
        assert_eq!(state.custom_marker.unwrap().space, FormId(0xDA726));
        remove_custom_marker(&mut state);
        assert_eq!(state.custom_marker, None);
    }
}
