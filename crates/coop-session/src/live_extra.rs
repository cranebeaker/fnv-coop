//! Voice, markers and environment glue of the live layer, kept out of
//! `live.rs` so that file only carries one-line hooks into it.
//!
//! * Voice: the viewer hands encoded voice frames in with
//!   [`crate::Intent::Voice`] and reads other players' frames from
//!   [`crate::LiveView::voice`]. The envelope bytes are opaque here.
//! * Markers and pings: [`crate::Intent::Social`] carries a
//!   `MarkerSet`/`MarkerClear`; other players' markers come back in
//!   [`crate::LiveView::markers`].
//! * Weather and time: the host publishes its sky and clock
//!   (`coop_core::env`), a guest applies them to its `GameState` through
//!   `world::coop::env`, smoothing `GameHour`.

use std::collections::BTreeSet;

use coop_core::env::{EnvMsg, HostEnv, SkyState, TimeState};
use coop_core::ids::PlayerId;
use coop_core::proto::Msg;
use coop_core::social::{SocialMsg, VoicePacket};
use coop_core::sync::GuestLive;
use world::coop::apply::Ctx;
use world::coop::env::{apply_sky, read_clock, read_sky, write_clock, SkyShot};
use world::scripting::GameState;

use crate::live::{Intent, LiveView};

/// One voice frame from another player, as received (in order, deduplicated).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoiceFrame {
    /// Who spoke (the transport's word, not the packet's).
    pub speaker: PlayerId,
    /// The envelope bytes (`coop_voice::envelope::VoicePacket::encode`).
    pub bytes: Vec<u8>,
}

/// Most voice frames handed to the viewer in one frame (older ones go).
pub const MAX_VOICE_PER_FRAME: usize = 64;

/// State the live layer keeps for these features.
#[derive(Default)]
pub(crate) struct Extra {
    env: HostEnv,
    seen: BTreeSet<PlayerId>,
    /// Sky messages applied (guest).
    pub skies_applied: u32,
    last_shot: Option<SkyShot>,
    /// Sky messages sent (host).
    pub env_sent: u32,
}

/// The wire message for a voice or social intent from `me`; `None` for the
/// other intents.
pub(crate) fn social_msg(intent: Intent, me: PlayerId) -> Option<Msg> {
    match intent {
        Intent::Voice(bytes) => {
            if bytes.len() < 6 {
                return None;
            }
            let seq = u16::from_le_bytes([bytes[4], bytes[5]]);
            Some(Msg::Social(SocialMsg::Voice(VoicePacket {
                speaker: me,
                seq,
                payload: bytes,
            })))
        }
        Intent::Social(m) => Some(Msg::Social(m)),
        _ => None,
    }
}

impl Extra {
    /// Host: the sky and clock to publish this frame, addressed to every guest.
    pub(crate) fn host_env(
        &mut self,
        cx: Ctx,
        gs: &GameState,
        members: &[PlayerId],
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let now_set: BTreeSet<PlayerId> =
            members.iter().copied().filter(|p| !p.is_host()).collect();
        if now_set.difference(&self.seen).next().is_some() {
            self.env.resend();
        }
        self.seen = now_set.clone();
        if now_set.is_empty() {
            return Vec::new();
        }
        let shot = read_sky(&gs.weather);
        let (days, hour, scale) = read_clock(cx.order, gs);
        let sky = SkyState {
            current: shot.current,
            previous: shot.previous,
            forced: shot.forced,
            fade_milli: shot.fade_milli,
            started_centi: shot.started_centi,
        };
        let mut out = Vec::new();
        for m in self
            .env
            .tick(now_ms, sky, TimeState::from_game(days, hour, scale))
        {
            self.env_sent += 1;
            for p in &now_set {
                out.push((*p, Msg::Env(m)));
            }
        }
        out
    }

    /// Guest: applies the host's sky and returns the smoothed absolute hours.
    pub(crate) fn guest_env(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut GuestLive,
        now_ms: u64,
    ) -> Option<f64> {
        let hours = live.env.clock.sample(now_ms);
        // The scene layer's clock follower (`world::coop::scene::follow_clock`) owns the guest's
        // clock once the host's clock has arrived: it keeps `GameDaysPassed` continuous, as the
        // game does. Writing the whole days here as well set the day back every frame.
        if let Some(h) = hours.filter(|_| !gs.coop.live.scene.follow) {
            write_clock(cx.order, gs, h);
        }
        if let Some(s) = live.env.take_sky() {
            let shot = SkyShot {
                current: s.current,
                previous: s.previous,
                forced: s.forced,
                fade_milli: s.fade_milli,
                started_centi: s.started_centi,
            };
            let hour =
                hours.map_or_else(|| read_clock(cx.order, gs).1, |h| h.rem_euclid(24.0) as f32);
            apply_sky(&mut gs.weather, &shot, hour);
            self.skies_applied += 1;
            self.last_shot = Some(shot);
        } else if let Some(shot) = self.last_shot {
            // the game's own weather step (a new climate, a script) may have dropped the host's
            // override; the host owns the sky, so put it back at once
            if gs.weather.forced != Some(esm::FormId(shot.current)) {
                let hour =
                    hours.map_or_else(|| read_clock(cx.order, gs).1, |h| h.rem_euclid(24.0) as f32);
                apply_sky(&mut gs.weather, &shot, hour);
            }
        }
        hours
    }
}

/// Moves the voice frames the guest layer accepted into the view.
pub(crate) fn drain_voice(view: &mut LiveView, live: &mut GuestLive) {
    for v in live.voice_in.drain(..) {
        view.voice.push(VoiceFrame {
            speaker: v.speaker,
            bytes: v.payload,
        });
    }
    let n = view.voice.len();
    if n > MAX_VOICE_PER_FRAME {
        view.voice.drain(..n - MAX_VOICE_PER_FRAME);
    }
}

/// Convenience for tests and the budget report: is this a time message.
pub fn is_time(m: &EnvMsg) -> bool {
    matches!(m, EnvMsg::Time(_))
}
