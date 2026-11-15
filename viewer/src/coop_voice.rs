//! voice: the audio side of co-op voice chat (docs/coop/SPECS.md S36), behind the cargo feature
//! `voice` (on by default).
//!
//! The microphone (cpal) feeds `coop_voice::pipeline::VoiceTx` (Opus, push to talk or an open
//! microphone, self-mute); its frames go into the session's `VoiceLink`, which sends them. The
//! frames the other players sent come out of the `VoiceLink` into `VoiceRx`, which places each
//! speaker by their avatar (the live ladder's places), applies the volume and the mutes, and mixes
//! them to the speakers (cpal). The keys are read from the game's window
//! (`engine_platform::d3d9real::held_keys`, DirectInput codes): push to talk, self-mute and
//! deafen; a HUD message says when the microphone or the other voices go off or on.
//!
//! The devices open on the first frame of a session with voice on; a missing microphone or
//! speaker is said once and the rest goes on without it. Without the feature this is a stub that
//! says voice chat is not in this build.

use coop_engine_session::voice::Settings;

/// Voice chat's settings from the environment, and the lines that say so.
pub fn settings() -> (Settings, Vec<String>) {
    let (s, mut notes) = Settings::read(|name| std::env::var(name).ok());
    notes.insert(0, s.line());
    (s, notes)
}

#[cfg(not(feature = "voice"))]
pub use stub::Audio;

#[cfg(not(feature = "voice"))]
mod stub {
    use std::collections::BTreeMap;

    use coop_core::ids::PlayerId;
    use coop_engine_session::voice::{Mode, Settings, VoiceLink};
    use coop_engine_session::voicelist::Prefs; // voiceui
    use engine_core::Engine;

    pub struct Audio;

    impl Audio {
        /// Without the feature: no audio; says so when voice was asked for.
        pub fn open(settings: Settings) -> (Option<Audio>, Option<String>) {
            let line = (settings.mode != Mode::Off).then(|| {
                "co-op voice: not built into this program (cargo feature voice)".to_string()
            });
            (None, line)
        }

        pub fn frame(
            &mut self,
            _e: &mut Engine,
            _link: &mut VoiceLink,
            _me: PlayerId,
            _names: &BTreeMap<PlayerId, String>, // voiceui
            _prefs: &Prefs,                      // voiceui
            _lines: &mut Vec<String>,
        ) {
        }
    }
}

#[cfg(feature = "voice")]
pub use real::Audio;

#[cfg(feature = "voice")]
mod real {
    use std::collections::BTreeMap;
    use std::time::Instant;

    use coop_core::ids::PlayerId;
    use coop_engine_session::voice::{FrameClock, Mode, Press, Settings, VoiceLink};
    use coop_engine_session::voicelist::Prefs; // voiceui
    use coop_voice::capture::{self, Capture, Playback};
    use coop_voice::pipeline::{default_codec, Activation, VoiceRx, VoiceTx};
    use engine_core::Engine;

    /// Devices, codec state and the keys (cpal streams are not `Send`: this stays on the
    /// main thread with the session).
    pub struct Audio {
        settings: Settings,
        capture: Option<Capture>,
        playback: Option<Playback>,
        tx: Option<VoiceTx>,
        rx: VoiceRx,
        tried: bool,
        mic: Vec<f32>,
        out: Vec<f32>,
        rate: u32,
        frame: usize,
        clock: FrameClock,
        last: Option<Instant>,
        mute: Press,
        deafen: Press,
        self_muted: bool,
        deafened: bool,
        started: Instant,
    }

    impl Audio {
        /// The audio side for `settings`; `None` when voice is off.
        pub fn open(settings: Settings) -> (Option<Audio>, Option<String>) {
            if settings.mode == Mode::Off {
                return (None, None);
            }
            let c = default_codec();
            let mut rx = VoiceRx::new();
            rx.set_master(settings.master);
            (
                Some(Audio {
                    rate: c.sample_rate(),
                    frame: c.frame_samples(),
                    settings,
                    capture: None,
                    playback: None,
                    tx: None,
                    rx,
                    tried: false,
                    mic: Vec::new(),
                    out: Vec::new(),
                    clock: FrameClock::default(),
                    last: None,
                    mute: Press::default(),
                    deafen: Press::default(),
                    self_muted: false,
                    deafened: false,
                    started: Instant::now(),
                }),
                None,
            )
        }

        fn open_devices(&mut self, me: PlayerId, lines: &mut Vec<String>) {
            self.tried = true;
            match capture::start() {
                Ok(c) => {
                    let act = match self.settings.mode {
                        Mode::Open => Activation::VoiceActivity,
                        _ => Activation::PushToTalk,
                    };
                    self.tx = Some(VoiceTx::with_default_codec(me.0, c.device_rate, act));
                    lines.push(format!(
                        "co-op voice: microphone open at {} Hz, codec {} Hz",
                        c.device_rate, self.rate
                    ));
                    self.capture = Some(c);
                }
                Err(e) => lines.push(format!(
                    "co-op voice: no microphone ({e}); you hear the others but they do not hear you"
                )),
            }
            match capture::start_playback() {
                Ok(p) => {
                    lines.push(format!("co-op voice: speakers at {} Hz", p.device_rate));
                    self.playback = Some(p);
                }
                Err(e) => lines.push(format!(
                    "co-op voice: no speakers ({e}); the others' voices are not played"
                )),
            }
        }

        /// One step after the session's: the keys, this player's microphone into the link,
        /// the other players' frames out of it to the speakers.
        pub fn frame(
            &mut self,
            e: &mut Engine,
            link: &mut VoiceLink,
            me: PlayerId,
            names: &BTreeMap<PlayerId, String>, // voiceui
            prefs: &Prefs,                      // voiceui
            lines: &mut Vec<String>,
        ) {
            if !self.tried {
                self.open_devices(me, lines);
            }
            // The keys, from the game's window (only while it has the focus).
            #[cfg(windows)]
            let keys = engine_platform::d3d9real::held_keys();
            #[cfg(not(windows))]
            let keys = [false; 256];
            let held = |code: u8| keys[code as usize];
            if self.mute.pressed(held(self.settings.mute_key)) {
                self.self_muted = !self.self_muted;
                let text = if self.self_muted {
                    "Voice chat: your microphone is off"
                } else {
                    "Voice chat: your microphone is on"
                };
                coop_engine::talk::show(e, text);
                lines.push(format!("co-op voice: {text}"));
            }
            if self.deafen.pressed(held(self.settings.deafen_key)) {
                self.deafened = !self.deafened;
                let text = if self.deafened {
                    "Voice chat: the other players are silent"
                } else {
                    "Voice chat: you hear the other players"
                };
                coop_engine::talk::show(e, text);
                lines.push(format!("co-op voice: {text}"));
            }
            // This player's microphone to the party.
            if let (Some(cap), Some(tx)) = (self.capture.as_ref(), self.tx.as_mut()) {
                tx.set_ptt(held(self.settings.ptt_key));
                tx.set_muted(self.self_muted);
                self.mic.clear();
                cap.take(&mut self.mic);
                for frame in tx.push_mic(&self.mic) {
                    link.queue(frame);
                }
                // voiceui: this player's own talking and its level, for the players list.
                let now = link.now_ms;
                link.speaking.local(tx.talking, now);
                if tx.talking && !self.mic.is_empty() {
                    let sq: f32 = self.mic.iter().map(|s| s * s).sum();
                    let rms = (sq / self.mic.len() as f32).sqrt();
                    link.speaking.local_level = coop_engine_session::voicelist::display_level(rms);
                }
            }
            // The party to the speakers, each where their avatar stands.
            self.rx.set_deafened(self.deafened);
            if let Some(l) = link.listener {
                self.rx.positions.listener = (l.pos, l.heading, l.space);
            }
            self.rx.positions.speakers.clear();
            for (p, place) in &link.speakers {
                self.rx
                    .positions
                    .speakers
                    .insert(p.0, (place.pos, place.space));
            }
            let now = Instant::now();
            let now_ms = now.duration_since(self.started).as_millis() as u64;
            for (p, bytes) in link.take_heard() {
                // voiceui: by the name every game knows (the host's roster), from the prefs.
                if let Some(name) = names.get(&p) {
                    self.rx.set_muted(p.0, prefs.is_muted(name));
                    self.rx.set_volume(p.0, prefs.volume_of(name));
                }
                self.rx.on_bytes(p.0, &bytes, now_ms);
            }
            let elapsed = self
                .last
                .map_or(0.0, |t| now.duration_since(t).as_secs_f64() * 1000.0);
            self.last = Some(now);
            let frame_ms = self.frame as f64 * 1000.0 / f64::from(self.rate.max(1));
            let mut out = std::mem::take(&mut self.out);
            for _ in 0..self.clock.due(elapsed, frame_ms) {
                if self.rx.mix(self.frame, &mut out) > 0 {
                    if let Some(p) = self.playback.as_mut() {
                        p.push(&out, self.rate);
                    }
                }
            }
            self.out = out;
            // voiceui: each speaking player's level as heard here, for the lists.
            let speaking = link.speaking.now(link.now_ms);
            link.levels.retain(|p, _| speaking.contains(p));
            for p in speaking {
                let l = coop_engine_session::voicelist::display_level(self.rx.level(p.0));
                link.levels.insert(p, l);
            }
        }
    }
}
