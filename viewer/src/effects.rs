//! Image space modifiers scripts applied (`ApplyImageSpaceModifier`,
//! `world::modifier`) played on the camera's final pass: each from when it
//! was applied, until it's removed or (animatable ones) has played out.
//! What the passes take is drawn (bloom, brightness limit, cinematic
//! values, tint, fade colour, and a blur of 1 to 7: `world::menu_background`);
//! a blur under 1, double vision, radial blur and depth of field aren't.
//! The menus' background modifier (`menu_background`) plays here too, at
//! strength 1 while the background is held.

use std::collections::HashMap;

use bevy::prelude::*;
use esm::FormId;
use world::modifier::Modifier;

use crate::dialogue::DialogueState;
use crate::grade::ImageSpaceGrade;
use crate::{GameFiles, Grading};

/// When each playing modifier was applied (seconds), and the records read.
#[derive(Resource, Default)]
pub struct Effects {
    started: HashMap<FormId, f32>,
    records: HashMap<FormId, Option<Modifier>>,
    /// Modifiers hits apply (`hiteffects`: `GetHit` on the player), each
    /// an instance of its own: the modifier, when it started (seconds) and
    /// its strength (`world::impacts::with_strength`).
    pub instances: Vec<(FormId, f32, f32)>,
    /// The menus' background while it's held: the modifier it was captured
    /// with and which capture it is (`menu_background`).
    pub background: Option<(Option<FormId>, u32)>,
}

#[allow(clippy::too_many_arguments)]
pub fn play_effects(
    time: Res<Time>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    grading: Res<Grading>,
    mut effects: ResMut<Effects>,
    exterior: Option<Res<crate::exterior::Exterior>>,
    mut weathers: ResMut<crate::weather::Weathers>,
    mut cameras: Query<&mut ImageSpaceGrade>,
) {
    let order = &game.0.order;
    let now = time.elapsed_secs();
    let applied = state.0.modifiers.clone();
    let Effects {
        started,
        records,
        instances,
        background,
    } = &mut *effects;
    started.retain(|id, _| applied.contains(id));
    let mut values = Vec::new();
    // Outdoors, the weather's modifiers for the hour (sunrise, day, sunset,
    // night: `world::weather::Weather::modifier_at`), the two weathers'
    // blended while one fades into the other. (Recorded at Goodsprings:
    // `NVWastelandIS` arrived in the final pass as brightness × 1.3 and its
    // tint, in the sky as "LUM ramp no tex" × 0.8 and on lit surfaces as
    // the sunlight dimmer × 1.1.)
    if let Some(e) = exterior.as_ref() {
        let hour = state
            .0
            .global(order, "GameHour")
            .unwrap_or(world::weather::DEFAULT_HOUR);
        if let Some(w) = weathers.mix(order, &state.0, e.weather) {
            let clock = world::weather::SkyClock::new(
                e.grid.climate.as_ref(),
                world::weather::SkySettings::load(order),
            );
            values.extend(w.modifier_at(order, &clock, hour));
        }
    }
    let mut ended = Vec::new();
    for id in applied {
        let since = *started.entry(id).or_insert(now);
        let Some(m) = records
            .entry(id)
            .or_insert_with(|| Modifier::load(order, id))
        else {
            continue;
        };
        let age = now - since;
        if m.finished(age) {
            ended.push(id);
        } else {
            values.push(m.at(age));
        }
    }
    if !ended.is_empty() {
        state.0.modifiers.retain(|m| !ended.contains(m));
    }
    // Hits' own instances, at their strengths, until played out.
    instances.retain(|&(id, since, strength)| {
        let Some(m) = records
            .entry(id)
            .or_insert_with(|| Modifier::load(order, id))
        else {
            return false;
        };
        let age = now - since;
        if m.finished(age) {
            return false;
        }
        values.push(world::impacts::with_strength(&m.at(age), strength));
        true
    });
    // The menus' background modifier: not animatable, so its first keys,
    // at strength 1 (`00871dc0` → `005299a0(imad, 1.0, 0)`).
    if let Some((Some(id), _)) = *background {
        if let Some(m) = records
            .entry(id)
            .or_insert_with(|| Modifier::load(order, id))
        {
            values.push(m.at(0.0));
        }
    }
    let base = if grading.on {
        grading.grade
    } else {
        grading.grade.without_cinematic()
    };
    let mut grade = base.with_modifiers(&values);
    if let Some((_, generation)) = *background {
        grade.background.z = 1.0;
        grade.background.w = generation as f32;
    }
    for mut current in &mut cameras {
        if *current != grade {
            *current = grade;
        }
    }
}
