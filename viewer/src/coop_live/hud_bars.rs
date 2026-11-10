//! Graphical health bars drawn with Bevy UI nodes: over the other players
//! (their name tags, `puppets::name_tags`) and over people and creatures that
//! were hurt or are under the crosshair (`npc_bars`).
//!
//! A bar is a root node (dark frame) with one fill child. Callers write the
//! fraction and the screen position into [`HealthBar`]; `render_bars` turns
//! that into node sizes and colour.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use cellview::space;
use esm::FormId;

use super::CoopPuppet;
use crate::ai::Walker;
use crate::crosshair::Crosshair;
use crate::dialogue::DialogueState;
use crate::GameFiles;

/// Bar width and height in pixels.
pub const BAR_W: f32 = 56.0;
pub const BAR_H: f32 = 7.0;
/// Seconds a bar stays over an NPC after it took damage.
pub const SHOW_SECS: f32 = 5.0;
/// Units above an NPC's feet where its bar floats (before scale).
const HEAD: f32 = 150.0;
/// Bars are only made for people nearer than this (game units).
const NPC_RANGE: f32 = 4000.0;

/// What a bar shows this frame.
#[derive(Component, Default)]
pub struct HealthBar {
    /// Health fraction, 0 to 1.
    pub frac: f32,
    /// Screen position of the bar's top left corner (`None`: hidden).
    pub at: Option<Vec2>,
}

/// The coloured part of a bar.
#[derive(Component)]
pub struct BarFill;

/// Health as a fraction of full, clamped to 0..=1 (full when `max` is 0).
pub fn bar_fraction(hp: f64, max: f64) -> f32 {
    if max <= 0.0 {
        return 1.0;
    }
    (hp / max).clamp(0.0, 1.0) as f32
}

/// Fill colour by health fraction: green, amber, red.
pub fn bar_color(frac: f32) -> Color {
    if frac < 0.25 {
        Color::srgb(0.85, 0.2, 0.15)
    } else if frac < 0.6 {
        Color::srgb(0.95, 0.75, 0.2)
    } else {
        Color::srgb(0.3, 0.8, 0.3)
    }
}

/// When damage to an actor first shows (or grows) the bar stays until the
/// returned time. `prev`: the damage seen last frame (`None`: first sight),
/// `now_damage`: the damage now, `until`: the current show-until time.
pub fn shown_until(prev: Option<f64>, now_damage: f64, now: f32, until: f32) -> f32 {
    match prev {
        Some(p) if now_damage > p + 0.01 => now + SHOW_SECS,
        _ => until,
    }
}

/// Spawns a hidden bar and returns its root.
pub fn spawn_bar(commands: &mut Commands) -> Entity {
    let fill = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(bar_color(1.0)),
            BarFill,
        ))
        .id();
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(BAR_W),
                height: Val::Px(BAR_H),
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            Visibility::Hidden,
            HealthBar::default(),
        ))
        .add_child(fill)
        .id()
}

/// Applies each bar's fraction and position to its nodes.
#[allow(clippy::type_complexity)]
pub fn render_bars(
    mut roots: Query<(&HealthBar, &mut Node, &mut Visibility, &Children), Without<BarFill>>,
    mut fills: Query<(&mut Node, &mut BackgroundColor), (With<BarFill>, Without<HealthBar>)>,
) {
    for (bar, mut node, mut vis, kids) in &mut roots {
        let Some(at) = bar.at else {
            if *vis != Visibility::Hidden {
                *vis = Visibility::Hidden;
            }
            continue;
        };
        node.left = Val::Px(at.x);
        node.top = Val::Px(at.y);
        *vis = Visibility::Inherited;
        for k in kids.iter() {
            if let Ok((mut fnode, mut bg)) = fills.get_mut(k) {
                fnode.width = Val::Percent(bar.frac * 100.0);
                bg.0 = bar_color(bar.frac);
            }
        }
    }
}

/// Per-actor memory for [`npc_bars`].
pub struct NpcSeen {
    damage: f64,
    until: f32,
    bar: Option<Entity>,
}

/// Health bars over people and creatures that took damage in the last few
/// seconds (from anyone: the replicated damage is in the game state on host
/// and guest alike) or that the crosshair is on.
#[allow(clippy::too_many_arguments)]
pub fn npc_bars(
    mut commands: Commands,
    time: Res<Time>,
    state: Res<DialogueState>,
    game: Res<GameFiles>,
    crosshair: Res<Crosshair>,
    cameras: Query<(&Camera, &GlobalTransform), With<crate::FlyCamera>>,
    walkers: Query<(&Walker, Option<&CoopPuppet>)>,
    mut bars: Query<&mut HealthBar>,
    mut seen: Local<HashMap<FormId, NpcSeen>>,
) {
    let Ok((camera, camera_at)) = cameras.single() else {
        return;
    };
    let gs = &state.0;
    let now = time.elapsed_secs();
    let me = gs.player_position.unwrap_or([0.0; 3]);
    let target = crosshair.target();
    let mut alive: HashSet<FormId> = HashSet::new();
    for (w, mark) in &walkers {
        // a remote player's body has its own tag
        if mark.is_some_and(|m| m.0.is_some()) {
            continue;
        }
        let r = w.reference;
        alive.insert(r);
        let dmg = gs.damage.get(&r).copied().unwrap_or(0.0);
        let entry = seen.entry(r).or_insert(NpcSeen {
            damage: dmg,
            until: 0.0,
            bar: None,
        });
        entry.until = shown_until(Some(entry.damage), dmg, now, entry.until);
        entry.damage = dmg;
        let wanted = (now < entry.until || target == Some(r))
            && !gs.dead.contains(&r)
            && (w.position[0] - me[0]).hypot(w.position[1] - me[1]) <= NPC_RANGE;
        let mut place = None;
        if wanted {
            let order = &game.0.order;
            if let (Some(max), Some(hp)) = (
                world::combat::max_health(order, gs, r),
                world::combat::health(order, gs, r),
            ) {
                let head = Vec3::from(space::point([
                    w.position[0],
                    w.position[1],
                    w.position[2] + HEAD * w.scale.max(0.1),
                ]));
                if let Ok(at) = camera.world_to_viewport(camera_at, head) {
                    place = Some((
                        Vec2::new(at.x - BAR_W / 2.0, at.y - BAR_H),
                        bar_fraction(hp, max),
                    ));
                }
            }
        }
        match (place, entry.bar) {
            (Some((at, frac)), None) => {
                let e = spawn_bar(&mut commands);
                commands.entity(e).insert(HealthBar { frac, at: Some(at) });
                entry.bar = Some(e);
            }
            (Some((at, frac)), Some(e)) => {
                if let Ok(mut b) = bars.get_mut(e) {
                    b.frac = frac;
                    b.at = Some(at);
                }
            }
            (None, Some(e)) => {
                if let Ok(mut b) = bars.get_mut(e) {
                    b.at = None;
                }
            }
            (None, None) => {}
        }
    }
    seen.retain(|r, s| {
        let keep = alive.contains(r);
        if !keep {
            if let Some(e) = s.bar {
                commands.entity(e).despawn();
            }
        }
        keep
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fraction_is_clamped() {
        assert_eq!(bar_fraction(50.0, 100.0), 0.5);
        assert_eq!(bar_fraction(-5.0, 100.0), 0.0);
        assert_eq!(bar_fraction(150.0, 100.0), 1.0);
        assert_eq!(bar_fraction(10.0, 0.0), 1.0);
    }

    #[test]
    fn colour_follows_health() {
        assert_eq!(bar_color(1.0), bar_color(0.6));
        assert_ne!(bar_color(0.5), bar_color(0.9));
        assert_ne!(bar_color(0.1), bar_color(0.5));
    }

    #[test]
    fn damage_shows_the_bar_for_a_while() {
        // first sight never shows
        assert_eq!(shown_until(None, 30.0, 10.0, 0.0), 0.0);
        // no new damage keeps the old time
        assert_eq!(shown_until(Some(30.0), 30.0, 12.0, 15.0), 15.0);
        // new damage restarts it
        assert_eq!(shown_until(Some(30.0), 40.0, 12.0, 0.0), 12.0 + SHOW_SECS);
        // healing does not
        assert_eq!(shown_until(Some(30.0), 20.0, 12.0, 0.0), 0.0);
    }
}
