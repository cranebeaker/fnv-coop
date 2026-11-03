//! Voice chat in the viewer, behind the cargo feature `voice` (default off).
//!
//! With the feature: the microphone (cpal) feeds `coop_voice::pipeline::VoiceTx`, whose packets leave as
//! `Intent::Voice` on the unreliable channel; the voice frames other players
//! sent (`LiveView::voice`) feed `VoiceRx`, which positions each speaker from their
//! puppet, applies the per-player volume and mute, and mixes to the speakers
//! (cpal). Push-to-talk is `--coop-ptt-key` (default Backslash), F8 toggles
//! self-mute, F7 deafens. Without the feature this file is a stub: the chat
//! commands answer that voice is not built, and nothing opens a sound device.
//!
//! Chat commands (typed with Y): `/mute NAME`, `/unmute NAME`, `/vol NAME 0-200`,
//! `/voice` (status).

use std::sync::Mutex;

/// Per-player settings shared between the chat commands and the audio system,
/// by player name (lower case).
#[derive(Default)]
pub struct Controls {
    muted: Vec<String>,
    volume: Vec<(String, f32)>,
    /// Slots heard in the last 300 ms (written by the audio system).
    pub speaking: Vec<u8>,
    /// Self-mute (F8).
    pub self_muted: bool,
    /// Deafened (F7).
    pub deafened: bool,
}

impl Controls {
    /// True when `name` is muted.
    pub fn is_muted(&self, name: &str) -> bool {
        self.muted.iter().any(|m| m == &name.to_ascii_lowercase())
    }

    /// Volume of `name` (1 = normal).
    pub fn volume_of(&self, name: &str) -> f32 {
        let n = name.to_ascii_lowercase();
        self.volume
            .iter()
            .find(|(k, _)| *k == n)
            .map_or(1.0, |(_, v)| *v)
    }

    /// Mutes or unmutes by name.
    pub fn mute(&mut self, name: &str, on: bool) {
        let n = name.to_ascii_lowercase();
        self.muted.retain(|m| *m != n);
        if on {
            self.muted.push(n);
        }
    }

    /// Sets a volume by name (0 to 2).
    pub fn set_volume(&mut self, name: &str, v: f32) {
        let n = name.to_ascii_lowercase();
        self.volume.retain(|(k, _)| *k != n);
        self.volume.push((n, v.clamp(0.0, 2.0)));
    }
}

static CONTROLS: Mutex<Option<Controls>> = Mutex::new(None);

/// Runs `f` on the shared controls.
pub fn with_controls<R>(f: impl FnOnce(&mut Controls) -> R) -> R {
    let mut g = CONTROLS.lock().unwrap_or_else(|e| e.into_inner());
    f(g.get_or_insert_with(Controls::default))
}

/// Whether a player (slot) is speaking right now (for the name tag).
pub fn is_speaking(slot: u8) -> bool {
    with_controls(|c| c.speaking.contains(&slot))
}

/// Handles a chat line that starts with `/`; the reply is shown to the player.
pub fn command(line: &str) -> Option<String> {
    let w: Vec<&str> = line.split_whitespace().collect();
    match (w.first().copied(), w.get(1).copied()) {
        (Some("/mute"), Some(n)) => {
            with_controls(|c| c.mute(n, true));
            Some(format!("{n} is muted"))
        }
        (Some("/unmute"), Some(n)) => {
            with_controls(|c| c.mute(n, false));
            Some(format!("{n} is not muted"))
        }
        (Some("/vol"), Some(n)) => {
            let v: f32 = w.get(2).and_then(|x| x.parse().ok())?;
            with_controls(|c| c.set_volume(n, v / 100.0));
            Some(format!("{n} volume {:.0}%", v.clamp(0.0, 200.0)))
        }
        (Some("/voice"), _) => Some(status()),
        _ => None,
    }
}

#[cfg(not(feature = "voice"))]
fn status() -> String {
    "Voice chat is not built into this viewer (cargo feature voice)".into()
}

#[cfg(not(feature = "voice"))]
pub fn register(_app: &mut bevy::prelude::App) {}

#[cfg(feature = "voice")]
fn status() -> String {
    with_controls(|c| {
        format!(
            "Voice: mic {}, speakers {}",
            if c.self_muted { "muted" } else { "on" },
            if c.deafened { "off" } else { "on" }
        )
    })
}

#[cfg(feature = "voice")]
pub use real::register;

#[cfg(feature = "voice")]
mod real {
    use bevy::prelude::*;
    use coop_session::Intent;
    use coop_voice::capture::{self, Capture, Playback};
    use coop_voice::pipeline::{default_codec, Activation, VoiceRx, VoiceTx};

    use super::with_controls;
    use crate::coop::CoopLink;
    use crate::coop_live::hud_social::settings;
    use crate::coop_live::LiveNow;
    use crate::dialogue::DialogueState;
    use crate::frame_order::ViewerSet;
    use crate::hud::{HudMessage, HudMessages};

    /// Devices, codec state and what has been tried (a non-send resource: cpal streams are not `Send`).
    struct Audio {
        capture: Option<Capture>,
        playback: Option<Playback>,
        tx: Option<VoiceTx>,
        rx: VoiceRx,
        tried: bool,
        mic: Vec<f32>,
        frame_acc: f64,
        rate: u32,
        frame: usize,
        out: Vec<f32>,
    }

    pub fn register(app: &mut App) {
        let c = default_codec();
        app.insert_non_send_resource(Audio {
            capture: None,
            playback: None,
            tx: None,
            rx: VoiceRx::new(),
            tried: false,
            mic: Vec::new(),
            frame_acc: 0.0,
            rate: c.sample_rate(),
            frame: c.frame_samples(),
            out: Vec::new(),
        })
        .add_systems(
            Update,
            run.in_set(ViewerSet::AfterFrame)
                .after(crate::coop_live::frame::run)
                .before(crate::coop::send),
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn run(
        time: Res<Time>,
        keys: Res<ButtonInput<KeyCode>>,
        mouse: Res<ButtonInput<MouseButton>>,
        mut audio: NonSendMut<Audio>,
        mut now: ResMut<LiveNow>,
        state: Res<DialogueState>,
        mut link: NonSendMut<CoopLink>,
        mut messages: ResMut<HudMessages>,
        cameras: Query<&Transform, With<crate::FlyCamera>>,
    ) {
        let cfg = settings();
        if cfg.voice == "off" {
            return;
        }
        let Some(view) = now.view.clone() else { return };
        let a = &mut *audio;
        if !a.tried {
            a.tried = true;
            match capture::start() {
                Ok(c) => {
                    let act = if cfg.voice == "vad" {
                        Activation::VoiceActivity
                    } else {
                        Activation::PushToTalk
                    };
                    a.tx = Some(VoiceTx::with_default_codec(view.me.0, c.device_rate, act));
                    a.capture = Some(c);
                }
                Err(e) => messages
                    .queue
                    .push(HudMessage::from(format!("Voice: no microphone ({e})"))),
            }
            match capture::start_playback() {
                Ok(p) => a.playback = Some(p),
                Err(e) => messages
                    .queue
                    .push(HudMessage::from(format!("Voice: no speakers ({e})"))),
            }
        }
        // keys
        if keys.just_pressed(KeyCode::F8) {
            let on = with_controls(|c| {
                c.self_muted = !c.self_muted;
                c.self_muted
            });
            messages.queue.push(HudMessage::from(if on {
                "Microphone muted".to_string()
            } else {
                "Microphone on".to_string()
            }));
        }
        if keys.just_pressed(KeyCode::F7) {
            let on = with_controls(|c| {
                c.deafened = !c.deafened;
                c.deafened
            });
            messages.queue.push(HudMessage::from(if on {
                "Voice chat off".to_string()
            } else {
                "Voice chat on".to_string()
            }));
        }
        let (self_muted, deafened) = with_controls(|c| (c.self_muted, c.deafened));
        // microphone to the party
        let now_ms = view.now_ms;
        if let (Some(cap), Some(tx)) = (a.capture.as_ref(), a.tx.as_mut()) {
            tx.set_ptt(cfg.ptt.pressed(&keys, &mouse));
            tx.set_muted(self_muted);
            a.mic.clear();
            cap.take(&mut a.mic);
            for pk in tx.push_mic(&a.mic) {
                now.counters.voice_out += 1;
                link.0.live_intent(Intent::Voice(pk));
            }
        }
        // the party to the speakers
        a.rx.set_deafened(deafened);
        let gs = &state.0;
        let heading = cameras.single().map_or(0.0, |t| {
            let f = t.forward().as_vec3();
            f.x.atan2(-f.z)
        });
        a.rx.positions.listener = (
            gs.player_position.unwrap_or([0.0; 3]),
            heading,
            gs.player_cell.map_or(0, |c| c.0),
        );
        a.rx.positions.speakers.clear();
        for av in &view.avatars {
            a.rx.positions
                .speakers
                .insert(av.player.0, (av.pos, av.cell));
            let (muted, vol) = with_controls(|c| (c.is_muted(&av.name), c.volume_of(&av.name)));
            a.rx.set_muted(av.player.0, muted);
            a.rx.set_volume(av.player.0, vol);
        }
        for f in &view.voice {
            if a.rx.on_bytes(f.speaker.0, &f.bytes, now_ms) {
                now.counters.voice_in += 1;
            }
        }
        for gone in &view.left {
            a.rx.remove(gone.0);
        }
        a.frame_acc += f64::from(time.delta_secs()) * 1000.0;
        let ms = a.frame as f64 * 1000.0 / f64::from(a.rate);
        let mut out = std::mem::take(&mut a.out);
        while a.frame_acc >= ms {
            a.frame_acc -= ms;
            if a.rx.mix(a.frame, &mut out) > 0 {
                now.counters.voiced_frames += 1;
                if let Some(p) = a.playback.as_mut() {
                    p.push(&out, a.rate);
                }
            }
        }
        a.out = out;
        let speaking = a.rx.speaking(now_ms);
        if !speaking.is_empty() {
            now.counters.speaking_frames += 1;
        }
        with_controls(|c| c.speaking = speaking);
    }
}
