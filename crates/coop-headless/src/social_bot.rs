//! The social test-bot verbs: pings, waypoints and voice, modelled with the
//! real pieces (`PingBook`, `coop-voice`'s `VoiceTx`/`VoiceRx`) minus the
//! sound card. The microphone is a 440 Hz tone; the speakers are counters.
//!
//! ```text
//! coop ping [DISTANCE]       ping DISTANCE units ahead (default 600)
//! coop waypoint X Y          set the waypoint
//! coop unwaypoint            clear it
//! coop voice SECONDS         hold push-to-talk for SECONDS of tone
//! coop mute / coop unmute    self-mute the microphone
//! coop deafen / coop undeafen  hear nobody (drops what arrives)
//! coop say TEXT              chat to everyone
//! ```

use coop_core::ids::PlayerId;
use coop_core::social::compass::PingBook;
use coop_session::{Intent, LiveView, Session};
use coop_voice::pipeline::{Activation, VoiceRx, VoiceTx};

/// Microphone rate the model uses (what most sound cards report).
const MIC_RATE: u32 = 48_000;
/// Codec frame in the receive mixer (the built-in codec's, in samples).
const FRAME_MS: f64 = 20.0;

/// One peer's social bot.
pub struct SocialBot {
    book: PingBook,
    tx: Option<VoiceTx>,
    rx: VoiceRx,
    talk_until: f64,
    talking: bool,
    mix_acc: f64,
    phase: f64,
    /// Voice packets sent (frames and end markers).
    pub voice_out: u64,
    /// Voice packets accepted from others.
    pub voice_in: u64,
    /// 20 ms frames with someone audible in the mix.
    pub voiced_frames: u64,
    /// Pings and waypoints placed.
    pub pings: u64,
    /// Frames someone was shown as speaking (name tag indicator).
    pub speaking_frames: u64,
    /// Chat lines received from other players.
    pub chat_in: u64,
}

impl Default for SocialBot {
    fn default() -> Self {
        Self::new()
    }
}

impl SocialBot {
    /// Fresh bot.
    pub fn new() -> Self {
        SocialBot {
            book: PingBook::new(),
            tx: None,
            rx: VoiceRx::new(),
            talk_until: 0.0,
            talking: false,
            mix_acc: 0.0,
            phase: 0.0,
            voice_out: 0,
            voice_in: 0,
            voiced_frames: 0,
            pings: 0,
            speaking_frames: 0,
            chat_in: 0,
        }
    }

    /// True for the verbs this bot owns.
    pub fn owns(line: &str) -> bool {
        let w: Vec<String> = line
            .split_whitespace()
            .take(2)
            .map(str::to_ascii_lowercase)
            .collect();
        w.first().map(String::as_str) == Some("coop")
            && matches!(
                w.get(1).map(String::as_str),
                Some(
                    "ping"
                        | "waypoint"
                        | "unwaypoint"
                        | "voice"
                        | "mute"
                        | "unmute"
                        | "deafen"
                        | "undeafen"
                        | "say"
                )
            )
    }

    /// Runs a line. `me` is this peer's slot, `pos`/`heading` where it stands.
    #[allow(clippy::too_many_arguments)]
    pub fn handle(
        &mut self,
        line: &str,
        t: f64,
        me: PlayerId,
        cell: u32,
        pos: [f32; 3],
        heading: f32,
        s: &mut Session,
    ) {
        let w: Vec<&str> = line.split_whitespace().collect();
        let num = |i: usize| w.get(i).and_then(|x| x.parse::<f32>().ok());
        let now_ms = (t * 1000.0) as u64;
        match w.get(1).map(|x| x.to_ascii_lowercase()).as_deref() {
            Some("ping") => {
                let d = num(2).unwrap_or(600.0);
                let at = [
                    pos[0] + heading.sin() * d,
                    pos[1] + heading.cos() * d,
                    pos[2],
                ];
                if let Some(msgs) = self.book.ping(me, cell, at, "ping", now_ms) {
                    self.pings += 1;
                    for m in msgs {
                        s.live_intent(Intent::Social(m));
                    }
                }
            }
            Some("waypoint") => {
                let m = self.book.waypoint(
                    me,
                    cell,
                    [num(2).unwrap_or(0.0), num(3).unwrap_or(0.0), pos[2]],
                    "waypoint",
                );
                self.pings += 1;
                s.live_intent(Intent::Social(m));
            }
            Some("unwaypoint") => {
                if let Some(m) = self.book.clear_waypoint() {
                    s.live_intent(Intent::Social(m));
                }
            }
            Some("voice") => {
                let secs = f64::from(num(2).unwrap_or(1.0));
                self.talk_until = t + secs;
                let tx = self.tx.get_or_insert_with(|| {
                    VoiceTx::with_default_codec(me.0, MIC_RATE, Activation::PushToTalk)
                });
                tx.set_ptt(true);
                self.talking = true;
            }
            Some("say") => {
                let text = w.iter().skip(2).copied().collect::<Vec<_>>().join(" ");
                if !text.is_empty() {
                    s.live_intent(Intent::Say(text));
                }
            }
            Some("mute") => self
                .tx
                .get_or_insert_with(|| {
                    VoiceTx::with_default_codec(me.0, MIC_RATE, Activation::PushToTalk)
                })
                .set_muted(true),
            Some("unmute") => self
                .tx
                .get_or_insert_with(|| {
                    VoiceTx::with_default_codec(me.0, MIC_RATE, Activation::PushToTalk)
                })
                .set_muted(false),
            Some("deafen") => self.rx.set_deafened(true),
            Some("undeafen") => self.rx.set_deafened(false),
            _ => {}
        }
    }

    /// Per frame, after the live frame. `view` is that frame's view.
    pub fn frame(&mut self, t: f64, dt: f32, s: &mut Session, view: Option<&LiveView>) {
        self.book.tick((t * 1000.0) as u64);
        let now_ms = (t * 1000.0) as u64;
        if let Some(tx) = self.tx.as_mut() {
            if self.talking && t >= self.talk_until {
                tx.set_ptt(false);
                self.talking = false;
            }
            // one frame's worth of microphone: tone while talking, silence after (so the end marker goes out)
            let n = (f64::from(dt) * f64::from(MIC_RATE)) as usize;
            let mic: Vec<f32> = (0..n)
                .map(|_| {
                    self.phase += 440.0 / f64::from(MIC_RATE);
                    if self.talking {
                        ((self.phase * std::f64::consts::TAU).sin() * 0.4) as f32
                    } else {
                        0.0
                    }
                })
                .collect();
            for pk in tx.push_mic(&mic) {
                self.voice_out += 1;
                s.live_intent(Intent::Voice(pk));
            }
        }
        if let Some(v) = view {
            self.chat_in += v.chat.iter().filter(|c| c.from != Some(v.me)).count() as u64;
            if let Some(p) = v.avatars.first() {
                // the first remote avatar stands for everyone in the model
                self.rx.positions.listener = ([0.0; 3], 0.0, p.cell);
                self.rx
                    .positions
                    .speakers
                    .insert(p.player.0, (p.pos, p.cell));
            }
            for f in &v.voice {
                if self.rx.on_bytes(f.speaker.0, &f.bytes, now_ms) {
                    self.voice_in += 1;
                }
            }
        }
        self.mix_acc += f64::from(dt) * 1000.0;
        let mut out = Vec::new();
        while self.mix_acc >= FRAME_MS {
            self.mix_acc -= FRAME_MS;
            if self.rx.mix(coop_voice::pipeline::ADPCM_FRAME, &mut out) > 0 {
                self.voiced_frames += 1;
            }
        }
        if !self.rx.speaking(now_ms).is_empty() {
            self.speaking_frames += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owns_only_its_verbs() {
        assert!(SocialBot::owns("coop ping 300"));
        assert!(SocialBot::owns("COOP Voice 2"));
        assert!(!SocialBot::owns("coop hit X 1"));
        assert!(!SocialBot::owns("ping"));
    }
}
