//! The player's camera: first and third person (`world::player_camera`,
//! the game's own rules), driven by the view key (F, the game's default
//! for Toggle POV: tap it to switch, hold it and move the mouse to look
//! around the player), the mouse wheel (zoom; out of first person, in to
//! it), vanity mode, the temporary views (furniture: `sitting`; the
//! Pip-Boy) and talking (first person).
//!
//! The camera entity's transform stays the player's eye for everything
//! that runs during the frame (aiming, activation, the first-person view:
//! the aim is not the camera, `docs/VR.md`); in third person
//! [`place_view`] moves it behind the player after the frame's systems
//! have run (before transforms propagate), and [`restore_eye`] puts the
//! eye back first thing next frame. Billboards and the sky still follow
//! the eye (they're placed during the frame); not the original's.
//!
//! Not modelled here: the body fading when the camera is inside it, the
//! HUD's mode while the view key is held, iron sights (first person), the
//! game's first-person camera node (the eye is the viewer's, at
//! `cellview::EYE_HEIGHT`).

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use cellview::{space, EYE_HEIGHT};
use world::dialogue::PLAYER_REF;
use world::player_camera::{CameraSettings, Frame, IniCamera, PlayerCamera, View, WHEEL_NOTCH};

use crate::walk::{game_point, Player};
use crate::FlyCamera;

/// The player's camera and what it needs between systems.
#[derive(Resource)]
pub struct PlayerView {
    pub camera: PlayerCamera,
    pub settings: CameraSettings,
    /// The frame the camera last saw (the furniture's temporary view asks
    /// it between updates).
    pub frame: Frame,
    /// The mouse turns the camera around the player this frame (the view
    /// key held), not the player.
    pub mouse_taken: bool,
    /// The eye moved away from for the third-person view, put back next
    /// frame.
    eye: Option<Transform>,
    /// The field of view this module last gave the camera (degrees, the
    /// game's 4:3 width), so others' changes are left alone.
    fov: Option<f32>,
    /// `DisablePlayerControls`' POV flag as last seen.
    pov_was_off: bool,
    /// The next placement snaps (after arriving somewhere).
    snap: bool,
    /// The temporary third person the player's death began.
    death_view: bool,
}

/// The INI's camera settings (`[General]`, `[HAVOK]`), with the exe's
/// defaults.
fn ini_camera(ini: &assets::IniSettings) -> IniCamera {
    let d = IniCamera::default();
    IniCamera {
        snap_dist: ini
            .float("General", "fZoom3rdPersonSnapDist")
            .unwrap_or(d.snap_dist),
        caster_size: ini
            .float("HAVOK", "fCameraCasterSize")
            .unwrap_or(d.caster_size),
        caster_player_size: ini
            .float("HAVOK", "fCameraCasterPlayerSize")
            .unwrap_or(d.caster_player_size),
        disable_auto_vanity: ini
            .get("General", "bDisableAutoVanityMode")
            .and_then(|v| v.trim().parse::<i32>().ok())
            .map_or(d.disable_auto_vanity, |v| v != 0),
    }
}

impl PlayerView {
    pub fn new(order: &esm::LoadOrder, ini: &assets::IniSettings) -> PlayerView {
        let settings = CameraSettings::read(order, ini_camera(ini));
        PlayerView {
            camera: PlayerCamera::new(&settings),
            settings,
            frame: Frame {
                feet: [0.0; 3],
                heading: 0.0,
                pitch: 0.0,
                eye_height: EYE_HEIGHT,
                scale: 1.0,
                eye: [0.0, 0.0, EYE_HEIGHT],
                dt: 0.0,
                dead: false,
            },
            mouse_taken: false,
            eye: None,
            fov: None,
            pov_was_off: false,
            snap: true,
            death_view: false,
        }
    }

    /// Begins the temporary third person furniture gives
    /// (`ForceTemp3rdPerson(1)`, from `TESFurniture::Activate`).
    pub fn force_temp_third(&mut self) -> bool {
        let frame = self.frame;
        self.camera.force_temp_third(true, &self.settings, &frame)
    }

    /// The temporary third person's end (`UpdateTemp3rdPerson`): true when
    /// first person came back.
    pub fn update_temp_third(&mut self, action_playing: bool) -> bool {
        let frame = self.frame;
        self.camera
            .update_temp_third(action_playing, false, &self.settings, &frame)
    }

    /// Whether the first-person view is the one wanted (no temporary third
    /// person, first person asked for).
    pub fn first_person_wanted(&self) -> bool {
        !self.camera.temp_third.active && !self.camera.want_third
    }
}

/// The frame's facts for the camera: the player's feet, heading and pitch
/// (game conventions: clockwise from north, positive looking down), the eye.
fn frame(player: &Player, fly: &FlyCamera, eye: &Transform, dt: f32, dead: bool) -> Frame {
    Frame {
        feet: player.character.feet,
        heading: (-fly.yaw).rem_euclid(std::f32::consts::TAU),
        pitch: -fly.pitch,
        // `fEyeHeight`: the viewer's eye height (measured, not read from
        // the game's `CalculateCachedEyeLevel`); the player's scale as 1.
        eye_height: EYE_HEIGHT,
        scale: 1.0,
        eye: game_point(eye.translation),
        dt,
        dead,
    }
}

/// Puts the eye back where the frame's systems expect the camera.
pub fn restore_eye(
    mut view: ResMut<PlayerView>,
    mut cameras: Query<&mut Transform, With<FlyCamera>>,
) {
    let Some(eye) = view.eye.take() else {
        return;
    };
    if let Ok(mut transform) = cameras.single_mut() {
        *transform = eye;
    }
}

/// The wheel this frame in the game's units (120 a notch).
fn wheel_delta(scroll: &AccumulatedMouseScroll) -> i32 {
    let notches = match scroll.unit {
        MouseScrollUnit::Line => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / 40.0,
    };
    (notches * WHEEL_NOTCH as f32).round() as i32
}

/// The camera's input each frame, before the mouse turns the player
/// (`look_around`): talking, the Pip-Boy, the POV control being turned off,
/// the view key, the wheel, the mouse while the view key is held, and
/// vanity mode.
#[allow(clippy::too_many_arguments)]
pub fn view_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    (motion, scroll): (Res<AccumulatedMouseMotion>, Res<AccumulatedMouseScroll>),
    state: Res<crate::dialogue::DialogueState>,
    player: Res<Player>,
    (menus, conversation, vats): (
        Res<crate::menus::Menus>,
        Res<crate::dialogue::Conversation>,
        Res<crate::vats::Vats>,
    ),
    mut view: ResMut<PlayerView>,
    cameras: Query<(&Transform, &FlyCamera)>,
) {
    view.mouse_taken = false;
    if !player.walking || !player.ready {
        view.snap = true;
        return;
    }
    let Ok((eye, fly)) = cameras.single() else {
        return;
    };
    let state = &state.0;
    let dead = state.dead.contains(&PLAYER_REF);
    let f = frame(&player, fly, eye, time.delta_secs(), dead);
    view.frame = f;
    let PlayerView {
        camera,
        settings,
        death_view,
        ..
    } = &mut *view;
    let s = &*settings;
    // The Pip-Boy coming up takes first person for as long as it's up.
    if menus.pipboy && !camera.temp_first {
        camera.force_temp_first(s, &f);
    }
    let menu_mode = menus.is_open() || vats.is_on();
    camera.update_temp_first(menu_mode, s, &f);
    // Knocked down or paralysed, the player's control handler forces a
    // temporary third person (`0093e860` at `0093fb6d`…`0093fbd3`:
    // `GetKnockState` or `IsParalyzed`, then `ForceTemp3rdPerson(1)` unless
    // already in third person): the death camera. A dead player's body is
    // a ragdoll; its knock state then is taken as set (not traced).
    if dead && !camera.third_person && !camera.temp_third.active {
        camera.force_temp_third(true, s, &f);
        *death_view = true;
    } else if !dead && *death_view {
        // Alive again (a save loaded): the temporary view ends
        // (`UpdateTemp3rdPerson` with nothing playing).
        if camera.update_temp_third(false, false, s, &f) || !camera.temp_third.active {
            *death_view = false;
        }
    }
    // Talking: first person.
    if conversation.0.as_ref().is_some_and(|t| !t.is_line_only()) {
        camera.focus_on_actor(s, &f);
        return;
    }
    // `DisablePlayerControls` with the POV flag: first person wanted
    // (`0095f590`).
    let pov_off = state.controls_off[world::scripting::controls::POV];
    if pov_off && !view.pov_was_off {
        view.camera.want_third = false;
    }
    view.pov_was_off = pov_off;
    let PlayerView {
        camera, settings, ..
    } = &mut *view;
    let s = &*settings;
    let ai_controlled = state.script_packages.contains_key(&PLAYER_REF);
    camera.view_key_input(
        keys.pressed(KeyCode::KeyF),
        keys.just_released(KeyCode::KeyF),
        menu_mode,
        pov_off,
        false,
        s,
        &f,
    );
    let wheel = wheel_delta(&scroll);
    camera.wheel(wheel, menu_mode, pov_off, ai_controlled, s, &f);
    let (dx, dy) = (motion.delta.x.round() as i32, motion.delta.y.round() as i32);
    let taken = !menu_mode && camera.look(dx, dy, false, s, f.dt);
    // Any input this frame stops vanity mode or restarts its wait.
    let input = keys.get_pressed().next().is_some()
        || buttons.get_pressed().next().is_some()
        || dx != 0
        || dy != 0
        || wheel != 0;
    if !menu_mode {
        camera.vanity_update(input, pov_off, s, &f);
    }
    view.mouse_taken = taken;
}

/// Places the camera for the frame once everything has used the eye: in
/// third person behind the player's shoulder, kept out of walls by the
/// cell's collision, looking where the player looks; the field of view the
/// game gives that view.
#[allow(clippy::too_many_arguments)]
pub fn place_view(
    state: Res<crate::dialogue::DialogueState>,
    player: Res<Player>,
    vats: Res<crate::vats::Vats>,
    collision: Res<crate::walk::CellCollision>,
    mut view: ResMut<PlayerView>,
    mut cameras: Query<(&mut Transform, &FlyCamera, &mut Projection)>,
    iron: Option<Res<crate::viewmodel::IronSightsFov>>,
) {
    let Ok((mut transform, fly, mut projection)) = cameras.single_mut() else {
        return;
    };
    let walking = player.walking && player.ready;
    let shot = vats.shot_view();
    let placed = if walking && !shot {
        let dead = state.0.dead.contains(&PLAYER_REF);
        let f = frame(&player, fly, &transform, view.frame.dt, dead);
        let snap = std::mem::take(&mut view.snap);
        let PlayerView {
            camera, settings, ..
        } = &mut *view;
        let mut cast = |from: [f32; 3], to: [f32; 3]| -> Option<f32> {
            let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
            let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            if len < 1e-3 {
                return None;
            }
            let dir = [d[0] / len, d[1] / len, d[2] / len];
            collision.0.raycast(from, dir, len).map(|(t, _)| t)
        };
        camera.update(settings, &f, snap, &mut cast)
    } else {
        View::FirstPerson
    };
    let world_fov = cellview::GAME_FOV_DEGREES;
    let wanted_fov = match placed {
        View::ThirdPerson {
            position,
            look_at,
            fov,
        } => {
            view.eye = Some(*transform);
            let at = Vec3::from(space::point(position));
            let target = Vec3::from(space::point(look_at));
            *transform = Transform::from_translation(at).looking_at(target, Vec3::Y);
            fov.unwrap_or(world_fov)
        }
        // First person: the world's field of view as the sights ease it
        // (`viewmodel::IronSightsFov`, `0095de30`).
        View::FirstPerson => iron.map_or(world_fov, |i| i.fov.world),
    };
    // V.A.T.S. has its own field of view while it's on.
    if vats.is_on() {
        return;
    }
    if view.fov != Some(wanted_fov) {
        if view.fov.is_some() || wanted_fov != world_fov {
            if let Projection::Perspective(p) = projection.as_mut() {
                p.fov = cellview::vertical_fov(wanted_fov);
            }
        }
        view.fov = Some(wanted_fov);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wheel_notches_count_as_the_games_wheel_units() {
        let line = AccumulatedMouseScroll {
            unit: MouseScrollUnit::Line,
            delta: Vec2::new(0.0, -2.0),
        };
        assert_eq!(wheel_delta(&line), -240);
        let pixel = AccumulatedMouseScroll {
            unit: MouseScrollUnit::Pixel,
            delta: Vec2::new(0.0, 40.0),
        };
        assert_eq!(wheel_delta(&pixel), 120);
    }

    #[test]
    fn the_ini_settings_come_from_their_sections() {
        let mut ini = assets::IniSettings::default();
        assert_eq!(ini_camera(&ini), IniCamera::default());
        ini.add("[General]\r\nfZoom3rdPersonSnapDist=70\r\nbDisableAutoVanityMode=1\r\n");
        let c = ini_camera(&ini);
        assert_eq!(c.snap_dist, 70.0);
        assert!(c.disable_auto_vanity);
    }
}
