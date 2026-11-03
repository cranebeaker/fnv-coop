//! The health of the person under the crosshair, shown below it in a co-op
//! session. The game's own enemy health meter is not drawn by this viewer
//! (`ui::hud::NOT_FOLLOWED`), so a guest punching or shooting someone could
//! not see the host's ledger answer. It reads the same health the host's
//! ledger writes into every machine's world (`ActorPump::apply`).

use bevy::prelude::*;
use world::coop::facts::player_of;
use world::dialogue::PLAYER_REF;

use crate::crosshair::Crosshair;
use crate::dialogue::DialogueState;
use crate::GameFiles;

/// Farthest pick (game units) whose health is shown.
const SHOW_RANGE: f32 = 6000.0;
/// Length of the bar in cells.
const CELLS: usize = 12;

/// The text node of the meter.
#[derive(Component)]
pub struct EnemyHealth;

/// Creates the meter, hidden.
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.72, 0.62)),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(35.0),
            width: Val::Percent(30.0),
            top: Val::Percent(58.0),
            ..default()
        },
        Visibility::Hidden,
        EnemyHealth,
    ));
}

/// The meter's text for a person at `health` of `max` (None when dead).
pub fn meter_text(name: &str, health: f64, max: f64) -> String {
    let max = max.max(1.0);
    let frac = (health / max).clamp(0.0, 1.0);
    let filled = ((frac * CELLS as f64).ceil() as usize).min(CELLS);
    format!(
        "{name}\n{}{}  {}/{}",
        "#".repeat(filled),
        "-".repeat(CELLS - filled),
        health.ceil() as i64,
        max.round() as i64
    )
}

/// Shows whoever the crosshair is on: name and health bar.
pub fn show(
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    crosshair: Res<Crosshair>,
    mut meter: Query<(&mut Text, &mut Visibility), With<EnemyHealth>>,
) {
    let Ok((mut text, mut vis)) = meter.single_mut() else {
        return;
    };
    let order = &game.0.order;
    let gs = &state.0;
    let mut label = None;
    if let Some(p) = crosshair.0.filter(|p| p.distance <= SHOW_RANGE) {
        let r = esm::FormId(p.reference);
        if r != PLAYER_REF && player_of(r).is_none() && !gs.dead.contains(&r) {
            if let Some(h) = world::combat::health(order, gs, r) {
                let max = world::combat::max_health(order, gs, r).unwrap_or(h);
                let name = world::script_functions::full_name(order, gs, r).unwrap_or_default();
                label = Some(meter_text(&name, h.max(0.0), max));
            }
        }
    }
    match label {
        Some(l) => {
            if text.0 != l {
                text.0 = l;
            }
            *vis = Visibility::Inherited;
        }
        None => *vis = Visibility::Hidden,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_meter_shows_name_bar_and_points() {
        assert_eq!(
            meter_text("Gecko", 60.0, 120.0),
            "Gecko\n######------  60/120"
        );
        assert!(meter_text("Gecko", 0.4, 120.0).contains("#-----------"));
        assert!(meter_text("Gecko", 500.0, 120.0).starts_with("Gecko\n############"));
    }
}
