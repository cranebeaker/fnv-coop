//! Glows that follow the time of day. Meshes marked for external emittance
//! whose color comes from a region's weather (`cellview::EmittanceLink`:
//! a region the reference's Emittance names, or with none the player's
//! weather region) take the color the game works out every frame
//! (`world::weather::EmittanceNow`), here once the clock has moved a game
//! minute or the weather state changed.
//!
//! Starting indoors with no weather state (no save, no outdoors yet), the
//! viewer acts as if the player had just come in through one of the
//! place's load doors from outside: the climate and weather region are
//! those of the square the door leads to, and the regions' weathers are
//! rolled once, as the game's sky does on its first outdoor frame. That is
//! this viewer's starting assumption, not the game's (a new game, or a save,
//! brings its own state).

use std::collections::BTreeMap;

use bevy::prelude::*;
use esm::FormId;
use world::weather::{EmittanceNow, SkyMode, WeatherState};

use crate::dialogue::DialogueState;
use crate::exterior::Exterior;
use crate::lighting::GameLitMaterial;
use crate::GameFiles;

/// A piece whose glow follows a region's weather.
#[derive(Component)]
pub struct Glow(pub cellview::EmittanceLink);

/// `--weather-region`: the player's weather region to start with.
#[derive(Resource, Default)]
pub struct StartRegion(pub Option<String>);

/// What the glows were last worked out for.
#[derive(Resource, Default)]
pub struct Glows {
    applied: Option<Applied>,
    /// The interior the starting assumption was made for.
    arrived: Option<u32>,
}

#[derive(PartialEq)]
struct Applied {
    hour: f32,
    climate: Option<FormId>,
    region: Option<FormId>,
    weathers: BTreeMap<FormId, FormId>,
}

/// One game minute, in hours.
const MINUTE: f32 = 1.0 / 60.0;

/// Every frame: once the clock has moved a game minute or the weather state
/// changed, every glow's color; otherwise the new ones'.
pub fn follow_emittance(
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    mut glows: ResMut<Glows>,
    mut materials: ResMut<Assets<GameLitMaterial>>,
    all: Query<(&MeshMaterial3d<GameLitMaterial>, &Glow)>,
    new: Query<(&MeshMaterial3d<GameLitMaterial>, &Glow), Added<Glow>>,
) {
    let order = &game.0.order;
    let s = &state.0;
    let hour = s
        .global(order, "GameHour")
        .unwrap_or(world::weather::DEFAULT_HOUR);
    let w = &s.weather;
    let now = Applied {
        hour,
        climate: w.climate,
        region: w.region,
        weathers: w.region_weathers.clone(),
    };
    let due = glows.applied.as_ref().is_none_or(|a| {
        (a.hour - hour).abs() >= MINUTE
            || a.climate != now.climate
            || a.region != now.region
            || a.weathers != now.weathers
    });
    if !due && new.is_empty() {
        return;
    }
    let chosen: Vec<(AssetId<GameLitMaterial>, cellview::EmittanceLink)> = if due {
        all.iter().map(|(m, g)| (m.0.id(), g.0)).collect()
    } else {
        new.iter().map(|(m, g)| (m.0.id(), g.0)).collect()
    };
    let regions: Vec<FormId> = chosen.iter().filter_map(|(_, l)| l.region).collect();
    let colors = EmittanceNow::new(order, w, hour, regions);
    for (id, link) in chosen {
        if let Some(m) = materials.get_mut(id) {
            let [r, g, b] = link.glow(&colors);
            let e = &mut m.extension.lighting.emissive;
            (e.x, e.y, e.z) = (r, g, b);
        }
    }
    if due {
        glows.applied = Some(now);
    }
}

/// Indoors, once per place while the sky has no climate yet, the starting
/// assumption above (the first load door leading outdoors); then, once,
/// `--weather-region` in place of the region that gave.
pub fn arrive_indoors(
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    mut glows: ResMut<Glows>,
    mut start: ResMut<StartRegion>,
    exterior: Option<Res<Exterior>>,
    doors: Option<Res<crate::walk::Doors>>,
    here: Option<Res<crate::scripts::Here>>,
) {
    let order = &game.0.order;
    let cell = here.and_then(|h| h.0);
    if exterior.is_none() && state.0.weather.climate.is_none() {
        if let (Some(doors), Some(cell)) = (doors.as_deref(), cell) {
            if glows.arrived != Some(cell) {
                glows.arrived = Some(cell);
                come_in(order, &mut state.0, doors);
            }
        }
    }
    let ready = exterior.is_some() || (cell.is_some() && glows.arrived == cell);
    if !ready {
        return;
    }
    if let Some(name) = start.0.take() {
        let id = FormId::parse_hex(&name)
            .filter(|id| order.get(*id).is_some())
            .or_else(|| order.form_by_editor_id(&name));
        match id {
            Some(region) => {
                state.0.weather.region = Some(region);
                println!("Weather region: {name}.");
            }
            None => println!("--weather-region: no region '{name}'."),
        }
    }
}

/// The weather state as if the player had just come in through the first
/// of `doors` leading outdoors.
fn come_in(
    order: &esm::LoadOrder,
    s: &mut world::scripting::GameState,
    doors: &crate::walk::Doors,
) {
    let Some((door, world)) = doors.0.iter().find_map(|d| d.world.map(|w| (d, FormId(w)))) else {
        return;
    };
    let Ok(grid) = world::WorldGrid::load(order, world) else {
        return;
    };
    let Some(square) = grid.cell_at(world::square_of(door.arrive)) else {
        return;
    };
    let hour = s
        .global(order, "GameHour")
        .unwrap_or(world::weather::DEFAULT_HOUR);
    let weather: &mut WeatherState = &mut s.weather;
    weather.enter_cell(order, square, Some(world), door.arrive);
    let globals = s.globals.clone();
    let mut dice = s.dice;
    s.weather.update(
        order,
        SkyMode::Exterior,
        hour,
        |g| globals.get(&g).copied().unwrap_or(0.0),
        || crate::weather::xorshift(&mut dice),
    );
    s.dice = dice;
    let name = |id: Option<FormId>| {
        id.and_then(|f| order.get(f))
            .and_then(|r| r.record().ok())
            .and_then(|r| r.editor_id())
            .unwrap_or_else(|| "none".into())
    };
    println!(
        "Weather (as if come in from {}): climate {}, weather region {}.",
        door.cell_label,
        name(s.weather.climate),
        name(s.weather.region)
    );
}
