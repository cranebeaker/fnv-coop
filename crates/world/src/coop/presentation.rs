//! Presentation of host-run scripts (G39): fades and the other image space
//! modifiers, sounds and music.
//!
//! The host's scripts run only on the host, and its game shows their
//! presentation there. The script functions call
//! [`GameState::coop_present`] (one authority test in single player). The
//! host's live frame sends each cue to every other player
//! ([`host_frame`], `SceneMsg::Present`). Every guest applies it in its
//! scene frame ([`apply`]): a modifier goes on or off in `modifiers`, and
//! the event the viewer shows (`Event::ImageSpace`, `Sound`, `Music`) is
//! queued as the host's script queued it there. A modifier that stays on is
//! replayed to a late joiner.

use coop_core::ids::PlayerId;
use coop_core::proto::Msg;
use coop_core::scene::Cue;
use coop_core::sync::HostLive;
use esm::FormId;

use super::Authority;
use crate::scripting::{Event, GameState};

/// Most cues the host takes from its scripts in one frame (a script that
/// plays a sound every frame cannot flood the link).
pub const MAX_CUES_PER_FRAME: usize = 16;

/// Per-machine presentation facts (never replicated, never saved).
#[derive(Clone, Debug, Default)]
pub struct PresentFacts {
    /// Cues the host's scripts made this frame.
    pub notes: Vec<Cue>,
    /// Cues the host sent.
    pub sent: u32,
    /// Cues the host dropped (more than [`MAX_CUES_PER_FRAME`] in a frame).
    pub dropped: u32,
    /// Cues applied here (a guest).
    pub applied: u32,
    /// Image space cues applied here.
    pub imods: u32,
    /// Sounds applied here.
    pub sounds: u32,
    /// Music tracks applied here.
    pub music: u32,
}

impl GameState {
    /// Presentation hook: a host script applied or removed an image space
    /// modifier, or played a sound or a music track. The host sends it to
    /// every other player.
    pub fn coop_present(&mut self, cue: Cue) {
        if self.coop.authority != Authority::Host {
            return;
        }
        let p = &mut self.coop.live.scene.present;
        if p.notes.len() < MAX_CUES_PER_FRAME {
            p.notes.push(cue);
        } else {
            p.dropped += 1;
        }
    }
}

/// Host, every frame: the cues of the frame go to every other player.
pub fn host_frame(gs: &mut GameState, live: &mut HostLive) -> Vec<(PlayerId, Msg)> {
    let mut outs = Vec::new();
    for cue in std::mem::take(&mut gs.coop.live.scene.present.notes) {
        gs.coop.live.scene.present.sent += 1;
        let o = live.scene.present(cue);
        outs.extend(live.route_scene(o));
    }
    outs
}

/// A guest: shows one cue of the host's as the host's script showed it.
pub fn apply(gs: &mut GameState, cue: Cue) {
    let p = &mut gs.coop.live.scene.present;
    p.applied += 1;
    match cue {
        Cue::ImageSpace { form, on } => {
            p.imods += 1;
            let m = FormId(form);
            gs.modifiers.retain(|x| *x != m);
            if on {
                gs.modifiers.push(m);
            }
            gs.events.push(Event::ImageSpace(m, on));
        }
        Cue::Sound { form } => {
            p.sounds += 1;
            gs.events.push(Event::Sound(FormId(form)));
        }
        Cue::Music { form } => {
            p.music += 1;
            gs.events.push(Event::Music(FormId(form)));
        }
    }
}

/// The dump's `scene.present` object.
pub fn json(gs: &GameState) -> String {
    let p = &gs.coop.live.scene.present;
    format!(
        "{{\"sent\":{},\"dropped\":{},\"applied\":{},\"imods\":{},\"sounds\":{},\"music\":{},\"on\":{}}}",
        p.sent,
        p.dropped,
        p.applied,
        p.imods,
        p.sounds,
        p.music,
        gs.modifiers.len()
    )
}
