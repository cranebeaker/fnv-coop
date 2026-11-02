//! The controls' buffer (`world::frame::input`, docs/FRAME_SKELETON.md
//! "PR 8 result"): the game polls the controls once a frame, at
//! `Main::OnIdle_PollControls` (`0086f390`, step 33; `Controls::Poll`,
//! `00a23010`), after the player's update (step 14) and before the
//! interface's idle. So the player reads the poll the previous frame made, as
//! the interface left it, and the interface reads this frame's.
//!
//! Here Bevy's input resources (the keys, the mouse buttons, the mouse's
//! motion and wheel) are that buffer. Bevy reads the devices in `PreUpdate`
//! (with the viewer's own device-level systems: the test keys, a movie
//! withholding the input); [`hold_last_poll`], first in `Update`, keeps that
//! read aside and puts back what the buffer held when the last frame ended;
//! [`poll_controls`], at `Controls::Poll`'s sub-step of step 33, puts this
//! frame's read in; [`keep_buffer`], in `Last`, keeps what the buffer holds
//! at the end of the frame (with what the menus took out of it) for the next
//! frame's player. A system reads the buffer as its place in the frame
//! finds it: before step 33 the last poll, after it this one.
//!
//! [`PlayerInput`] is the buffer as the player's step read it this frame,
//! for the player's controls the viewer handles later in the frame for
//! order (doors, grabbing, the quick save and load keys, waiting).
//!
//! What isn't the buffer: the frame's Tab and Alt test reads the keyboard
//! itself (`GetAsyncKeyState`, [`ControlsBuffer::device_keys`]); the menus'
//! typed characters are Bevy's keyboard events, read where the menus run
//! (the exe's interface drains the keyboard's event queue in its idle,
//! `00a23820`). The pad isn't read (the viewer has none). Head and hand
//! poses (docs/VR.md) are views, not controls, and are not in the buffer.

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;

/// One state of the buffer: what a poll read.
#[derive(Debug, Clone, Default)]
pub struct Polled {
    pub keys: ButtonInput<KeyCode>,
    pub buttons: ButtonInput<MouseButton>,
    pub motion: AccumulatedMouseMotion,
    pub scroll: AccumulatedMouseScroll,
}

/// The input resources the buffer lives in (absent without Bevy's input
/// plugin, as in the frame's tests).
type Live<'w> = (
    Option<ResMut<'w, ButtonInput<KeyCode>>>,
    Option<ResMut<'w, ButtonInput<MouseButton>>>,
    Option<ResMut<'w, AccumulatedMouseMotion>>,
    Option<ResMut<'w, AccumulatedMouseScroll>>,
);

fn read(live: &Live) -> Polled {
    Polled {
        keys: live.0.as_deref().cloned().unwrap_or_default(),
        buttons: live.1.as_deref().cloned().unwrap_or_default(),
        motion: live.2.as_deref().copied().unwrap_or_default(),
        scroll: live.3.as_deref().copied().unwrap_or_default(),
    }
}

fn write(live: &mut Live, p: &Polled) {
    if let Some(k) = live.0.as_deref_mut() {
        k.clone_from(&p.keys);
    }
    if let Some(b) = live.1.as_deref_mut() {
        b.clone_from(&p.buttons);
    }
    if let Some(m) = live.2.as_deref_mut() {
        *m = p.motion;
    }
    if let Some(s) = live.3.as_deref_mut() {
        *s = p.scroll;
    }
}

/// The buffer between frames and this frame's device read until the poll
/// takes it.
#[derive(Resource, Debug, Default)]
pub struct ControlsBuffer {
    /// What the buffer held when the last frame ended. Empty before the
    /// first poll (the controls' object starts zeroed).
    held: Polled,
    /// This frame's device read (Bevy's `PreUpdate`), until step 33.
    device: Option<Polled>,
}

impl ControlsBuffer {
    /// The keyboard as the devices give it this frame (`GetAsyncKeyState`:
    /// the frame's Tab and Alt test, `0086e682`, `0086e69a`).
    pub fn device_keys(&self) -> Option<&ButtonInput<KeyCode>> {
        self.device.as_ref().map(|d| &d.keys)
    }
}

/// The buffer as the player's step read it this frame (copied at the poll,
/// before this frame's read goes in).
#[derive(Resource, Debug, Clone, Default)]
pub struct PlayerInput {
    pub keys: ButtonInput<KeyCode>,
    pub buttons: ButtonInput<MouseButton>,
}

/// First in `Update`: keeps this frame's device read for the poll and puts
/// back the buffer as the last frame left it.
pub fn hold_last_poll(mut buffer: ResMut<ControlsBuffer>, mut live: Live) {
    let device = read(&live);
    write(&mut live, &buffer.held);
    buffer.device = Some(device);
}

/// `Controls::Poll` (`00a23010`, called at `0086f39e` in step 33): this
/// frame's device read goes into the buffer. What the player's step read
/// is kept in [`PlayerInput`].
pub fn poll_controls(
    mut buffer: ResMut<ControlsBuffer>,
    mut player: ResMut<PlayerInput>,
    mut live: Live,
) {
    let read_by_player = read(&live);
    player.keys = read_by_player.keys;
    player.buttons = read_by_player.buttons;
    if let Some(device) = buffer.device.take() {
        write(&mut live, &device);
    }
}

/// In `Last`: the buffer as the frame leaves it is what the next frame's
/// player reads. When the frame didn't poll (Tab and Alt held: it returns
/// after step 1) the buffer keeps the last poll, and Bevy's input goes on
/// from this frame's device read (a key's going down or up while unpolled
/// is seen once the frame polls again, but not as a press or release).
pub fn keep_buffer(mut buffer: ResMut<ControlsBuffer>, mut live: Live) {
    match buffer.device.take() {
        Some(device) => write(&mut live, &device),
        None => buffer.held = read(&live),
    }
}

/// Adds the buffer's resources and its systems outside the frame's sets
/// (`poll_controls` is placed by `frame_order`).
pub fn add(app: &mut App) {
    app.init_resource::<ControlsBuffer>()
        .init_resource::<PlayerInput>()
        .add_systems(Last, keep_buffer);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An app with the three systems in order and the input resources, the
    /// device read done by hand as Bevy's `PreUpdate` would.
    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .init_resource::<Seen>();
        add(&mut app);
        app.add_systems(
            Update,
            (
                hold_last_poll,
                see("player"),
                poll_controls,
                see("interface"),
                take_keys,
            )
                .chain(),
        );
        app
    }

    #[derive(Resource, Default)]
    struct Seen(Vec<(&'static str, bool, f32)>);

    fn see(
        who: &'static str,
    ) -> impl FnMut(Res<ButtonInput<KeyCode>>, Res<AccumulatedMouseMotion>, ResMut<Seen>) {
        move |keys, motion, mut seen| {
            seen.0
                .push((who, keys.just_pressed(KeyCode::KeyE), motion.delta.x));
        }
    }

    /// A menu that takes the keys while `Take` is set (the interface's
    /// clears, which the next frame's player sees).
    #[derive(Resource, Default)]
    struct Take(bool);

    fn take_keys(take: Option<Res<Take>>, mut keys: ResMut<ButtonInput<KeyCode>>) {
        if take.is_some_and(|t| t.0) {
            keys.reset_all();
        }
    }

    /// Bevy's `PreUpdate` for one frame: the last frame's presses end, the
    /// new ones go in.
    fn device(app: &mut App, press: Option<KeyCode>, dx: f32) {
        let w = app.world_mut();
        let mut keys = w.resource_mut::<ButtonInput<KeyCode>>();
        keys.clear();
        if let Some(k) = press {
            keys.press(k);
        }
        w.resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(dx, 0.0);
    }

    fn frame(app: &mut App, press: Option<KeyCode>, dx: f32) -> Vec<(&'static str, bool, f32)> {
        app.world_mut().resource_mut::<Seen>().0.clear();
        device(app, press, dx);
        app.update();
        app.world().resource::<Seen>().0.clone()
    }

    /// A press is seen by the interface in its frame (after the poll) and
    /// by the player in the next (before it), once each; so is the mouse's
    /// motion; the player's copy is what it read.
    #[test]
    fn the_interface_first_then_the_player() {
        let mut app = app();
        assert_eq!(
            frame(&mut app, Some(KeyCode::KeyE), 5.0),
            [("player", false, 0.0), ("interface", true, 5.0)]
        );
        assert!(!app
            .world()
            .resource::<PlayerInput>()
            .keys
            .pressed(KeyCode::KeyE));
        assert_eq!(
            frame(&mut app, None, 0.0),
            [("player", true, 5.0), ("interface", false, 0.0)]
        );
        assert!(app
            .world()
            .resource::<PlayerInput>()
            .keys
            .just_pressed(KeyCode::KeyE));
        assert_eq!(
            frame(&mut app, None, 0.0),
            [("player", false, 0.0), ("interface", false, 0.0)]
        );
        // Still held on the device throughout.
        assert!(app
            .world()
            .resource::<ButtonInput<KeyCode>>()
            .pressed(KeyCode::KeyE));
    }

    /// What the interface takes out of the buffer the player doesn't see
    /// (`Controls::ClearKeystrokes` in the idle, read next frame).
    #[test]
    fn the_player_sees_what_the_interface_left() {
        let mut app = app();
        app.insert_resource(Take(true));
        assert_eq!(
            frame(&mut app, Some(KeyCode::KeyE), 0.0),
            [("player", false, 0.0), ("interface", true, 0.0)]
        );
        app.insert_resource(Take(false));
        assert_eq!(
            frame(&mut app, None, 0.0),
            [("player", false, 0.0), ("interface", false, 0.0)]
        );
    }

    /// Without a poll (Tab and Alt held) the buffer keeps the last poll and
    /// the device's state goes on.
    #[test]
    fn a_frame_without_a_poll_keeps_the_buffer() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>();
        add(&mut app);
        app.add_systems(Update, hold_last_poll);
        device(&mut app, Some(KeyCode::KeyW), 0.0);
        app.update();
        assert!(app
            .world()
            .resource::<ButtonInput<KeyCode>>()
            .pressed(KeyCode::KeyW));
        assert!(app
            .world()
            .resource::<ControlsBuffer>()
            .held
            .keys
            .get_pressed()
            .next()
            .is_none());
    }
}
