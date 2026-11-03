//! The co-op live layer in the viewer: thin glue between the headless
//! `coop_session` live layer (avatars, shared actors, loot, dialogue, rules)
//! and the Bevy world. See docs/COOP.md "Live layer in the viewer".
//!
//! Frame order (all in `Update`):
//!
//! * `ViewerSet::Input`: `coop::receive` (packets in; a guest applies the
//!   host's state);
//! * the frame's systems run; `puppets::drive` and `actors::apply_remote`
//!   run just before `actors::animate_actors` so the poses they set are the
//!   ones animated;
//! * `ViewerSet::AfterFrame`: `intents::*` queue what the player did,
//!   [`frame::run`] samples the local player, runs the session's live frame
//!   and keeps its [`coop_session::LiveView`] in [`LiveNow`], then
//!   `puppets::sync` and the name tags follow it, then `coop::send`.
//!
//! Menus and time: in a co-op session the local player's menus (Pip-Boy,
//! container, barter, message boxes) do not stop the simulation or the
//! animations ([`menus_freeze_time`]), because the host's time is everyone's
//! time and a remote player's menu is never mirrored here. The dialogue menu
//! still stops this machine's simulation while it is open (the speaker is
//! held for the conversation); on the host that stops the world for the
//! guests too until the line or the conversation ends, which the session
//! bounds with the dialogue heartbeat. V.A.T.S. slow motion stays local.

mod actions;
mod actors;
mod bot;
mod chat;
mod frame;
mod hud_social;
mod intents;
mod interact;
mod puppets;
mod roam;
pub mod voice;
mod vote;

use std::sync::atomic::{AtomicBool, Ordering};

use bevy::prelude::*;
use coop_session::{LiveCounters, LiveView, PlayerId};

pub use bot::{is_bot_line, Bot};
pub use hud_social::{compass_extra, configure, party_points, Book};

use crate::frame_order::ViewerSet;

static LIVE: AtomicBool = AtomicBool::new(false);

/// The game executable version this build supports; quoted in the join handshake.
pub const EXE_VERSION: &str = "1.4.0.525";

/// False in a co-op session: the local player's menus do not freeze time.
pub fn menus_freeze_time() -> bool {
    !LIVE.load(Ordering::Relaxed)
}

/// Marks an actor entity that the live layer drives, so the AI systems
/// leave it alone (`ai::move_actors` skips it): `Some(player)` for the puppet
/// of a remote player, `None` for the local copy of an actor another peer
/// owns.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoopPuppet(pub Option<PlayerId>);

/// What the last live frame produced.
#[derive(Resource, Default)]
pub struct LiveNow {
    /// The view of the last frame.
    pub view: Option<LiveView>,
    /// Counters for the state dump.
    pub counters: LiveCounters,
    /// Glitch detectors over the remote avatars this machine draws.
    pub glitch: coop_session::glitch::GlitchWatch,
}

/// Registers the live layer's systems (added by `coop::CoopPlugin` once a
/// session is running).
pub struct LivePlugin;

impl Plugin for LivePlugin {
    fn build(&self, app: &mut App) {
        LIVE.store(true, Ordering::Relaxed);
        voice::register(app);
        app.init_resource::<LiveNow>()
            .init_resource::<puppets::Puppets>()
            .init_resource::<intents::Tap>()
            .init_resource::<intents::Lines>()
            .init_resource::<bot::Bot>()
            .init_resource::<chat::ChatLog>()
            .init_resource::<chat::Typing>()
            .init_resource::<hud_social::Book>()
            .add_systems(Startup, (chat::setup, hud_social::setup))
            .add_systems(
                Update,
                (
                    (
                        bot::run,
                        intents::loot,
                        intents::tap_dialogue,
                        actions::emit,
                    )
                        .chain()
                        .in_set(ViewerSet::AfterFrame)
                        .before(frame::run),
                    frame::run
                        .in_set(ViewerSet::AfterFrame)
                        .before(crate::coop::send),
                    (
                        puppets::sync,
                        puppets::name_tags,
                        actors::track,
                        actors::publish,
                        actors::adopt_remote,
                        actors::count,
                        intents::play_lines,
                        intents::follow_place,
                        roam::register_grid,
                        roam::apply_place,
                        intents::travel,
                        interact::follow_door,
                        interact::results,
                        chat::show,
                        vote::answer,
                    )
                        .in_set(ViewerSet::AfterFrame)
                        .after(frame::run)
                        .before(crate::coop::send),
                    chat::typing.in_set(ViewerSet::Input),
                    hud_social::ping_input
                        .in_set(ViewerSet::AfterFrame)
                        .before(frame::run),
                    (hud_social::publish, hud_social::edge_hints)
                        .in_set(ViewerSet::AfterFrame)
                        .after(frame::run),
                    hud_social::place_window,
                    (puppets::drive, actors::apply_remote).before(crate::actors::animate_actors),
                ),
            );
    }
}
