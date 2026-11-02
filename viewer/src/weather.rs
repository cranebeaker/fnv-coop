//! The weather outdoors, as the game picks and changes it
//! (`world::weather::WeatherState`): on entering a square the climate and
//! the player's weather region are looked at again, and every frame the
//! weather step rolls when due and fades between weathers. What's drawn
//! (`daylight`, `effects`, `grass`) reads the mix of the two weathers.

use std::collections::HashMap;

use bevy::prelude::*;
use esm::FormId;
use world::weather::{SkyMode, Weather, WeatherMix};

use crate::dialogue::DialogueState;
use crate::exterior::Exterior;
use crate::walk::game_point;
use crate::{FlyCamera, GameFiles};

/// Weathers read so far, and the cell the weather last saw the player
/// enter.
#[derive(Resource, Default)]
pub struct Weathers {
    loaded: HashMap<FormId, Option<Weather>>,
    entered: Option<FormId>,
}

impl Weathers {
    fn load(&mut self, order: &esm::LoadOrder, id: FormId) {
        self.loaded
            .entry(id)
            .or_insert_with(|| Weather::load(order, id));
    }

    /// The weather to show: the game state's current and fading-out
    /// weathers, else `fallback` (the first square's) on its own.
    pub fn mix(
        &mut self,
        order: &esm::LoadOrder,
        state: &world::scripting::GameState,
        fallback: Option<FormId>,
    ) -> Option<WeatherMix<'_>> {
        let w = &state.weather;
        let current = w.current.or(fallback)?;
        self.load(order, current);
        if let Some(p) = w.previous {
            self.load(order, p);
        }
        let current = self.loaded.get(&current)?.as_ref()?;
        let previous = w
            .previous
            .and_then(|p| self.loaded.get(&p))
            .and_then(Option::as_ref);
        Some(WeatherMix {
            current,
            previous,
            fade: if previous.is_some() { w.fade } else { 1.0 },
        })
    }
}

/// The game's dice (`GameState::roll`'s xorshift), on a copy of the state's
/// so the weather step can borrow the rest.
pub(crate) fn xorshift(x: &mut u64) -> u64 {
    let mut v = (*x).max(1);
    v ^= v << 13;
    v ^= v >> 7;
    v ^= v << 17;
    *x = v;
    v
}

/// Outdoors, every frame: a new square → `enter_cell`; then the weather
/// step at the game's hour.
pub fn run_weather(
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    exterior: Option<Res<Exterior>>,
    cameras: Query<&Transform, With<FlyCamera>>,
    mut weathers: ResMut<Weathers>,
) {
    let Some(exterior) = exterior else {
        weathers.entered = None;
        return;
    };
    let Ok(camera) = cameras.single() else {
        return;
    };
    let order = &game.0.order;
    let s = &mut state.0;
    let cell = exterior.grid.cell_at(exterior.here(camera));
    if cell != weathers.entered {
        if let Some(c) = cell {
            let feet = s
                .player_position
                .unwrap_or_else(|| game_point(camera.translation));
            s.weather
                .enter_cell(order, c, Some(exterior.grid.world.form_id), feet);
        }
        weathers.entered = cell;
    }
    let hour = s
        .global(order, "GameHour")
        .unwrap_or(world::weather::DEFAULT_HOUR);
    let globals = &s.globals;
    let mut dice = s.dice;
    let before = (s.weather.current, s.weather.previous);
    s.weather.update(
        order,
        SkyMode::Exterior,
        hour,
        |g| globals.get(&g).copied().unwrap_or(0.0),
        || xorshift(&mut dice),
    );
    s.dice = dice;
    if (s.weather.current, s.weather.previous) != before {
        let name = |id: Option<FormId>| {
            id.and_then(|w| order.get(w))
                .and_then(|r| r.record().ok())
                .and_then(|r| r.editor_id())
                .unwrap_or_else(|| "none".into())
        };
        match s.weather.previous {
            Some(_) => println!(
                "Weather: {} fading into {}.",
                name(s.weather.previous),
                name(s.weather.current)
            ),
            None => println!("Weather: {}.", name(s.weather.current)),
        }
    }
}
