//! The two ends of a voice chat as the game uses them, with no audio device
//! in sight (the `capture` feature adds cpal on top):
//!
//! * [`VoiceTx`]: microphone samples at any rate in, wire bytes out. Resamples
//!   to the codec rate, cuts codec frames, applies push-to-talk or voice
//!   activity, self-mute and a bitrate/packet counter.
//! * [`VoiceRx`]: wire bytes in per speaker, one stereo mix out, with
//!   per-player volume and mute, distance attenuation and pan from where the
//!   speakers stand ([`Positions`]), and a "who is speaking" list for the
//!   name tag.
//!
//! The wire bytes are [`crate::envelope::VoicePacket::encode`] output. The
//! game wraps them in its own message and fixes the speaker byte from the
//! transport's sender (the host never trusts the byte inside).

use std::collections::BTreeMap;

use crate::codec::{AdpcmCodec, Codec, CODEC_ADPCM};
use crate::envelope::VoicePacket;
use crate::mixer::{Mixer, Sender, Spatializer};
use crate::vad::{EnergyVad, Gate, Mode};

/// Sample rate of the built-in codec (the Opus codec uses 48 kHz).
pub const ADPCM_RATE: u32 = 16_000;
/// Samples per 20 ms frame of the built-in codec.
pub const ADPCM_FRAME: usize = 320;
/// Gain is full inside this distance (game units, about 70 to the metre).
pub const NEAR: f32 = 1_200.0;
/// Gain reaches the floor at this distance.
pub const FAR: f32 = 7_000.0;
/// Quietest a speaker in the same cell gets (voice carries across a level).
pub const FLOOR: f32 = 0.15;
/// Gain for a speaker in another cell (heard flat, like a radio).
pub const OTHER_CELL_GAIN: f32 = 0.5;
/// A speaker counts as speaking this long after their last frame (ms).
pub const SPEAKING_HOLD_MS: u64 = 300;
/// Speakers heard at once (the rest are dropped).
pub const MAX_SPEAKERS: usize = 8;

/// The codec to use: Opus when it is built in, else the built-in ADPCM.
pub fn default_codec() -> Box<dyn Codec> {
    #[cfg(feature = "opus")]
    {
        if let Ok(c) = crate::opus_codec::OpusCodec::new() {
            return Box::new(c);
        }
    }
    Box::new(AdpcmCodec::new(ADPCM_RATE, ADPCM_FRAME))
}

/// A decoder for a codec id (the receive side builds one per speaker).
pub fn codec_for(id: u8) -> Option<Box<dyn Codec>> {
    match id {
        CODEC_ADPCM => Some(Box::new(AdpcmCodec::new(ADPCM_RATE, ADPCM_FRAME))),
        #[cfg(feature = "opus")]
        crate::codec::CODEC_OPUS => crate::opus_codec::OpusCodec::new()
            .ok()
            .map(|c| Box::new(c) as Box<dyn Codec>),
        _ => None,
    }
}

/// How the microphone opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activation {
    /// Only while the key is held.
    PushToTalk,
    /// When the voice is loud enough.
    VoiceActivity,
}

/// Linear resampler for a continuous mono stream (keeps its phase between
/// calls). Good enough for speech at the rates games use.
#[derive(Clone, Debug)]
pub struct Resampler {
    from: u32,
    to: u32,
    pos: f64,
    last: f32,
}

impl Resampler {
    /// From `from` Hz to `to` Hz.
    pub fn new(from: u32, to: u32) -> Self {
        Self {
            from: from.max(1),
            to: to.max(1),
            pos: 0.0,
            last: 0.0,
        }
    }

    /// Resamples a chunk of the stream.
    pub fn process(&mut self, input: &[f32], out: &mut Vec<f32>) {
        if self.from == self.to {
            out.extend_from_slice(input);
            return;
        }
        let step = f64::from(self.from) / f64::from(self.to);
        // `pos` is the fractional input index of the next output sample,
        // relative to the start of this chunk; index -1 is the previous chunk's last sample.
        while self.pos < input.len() as f64 {
            let i = self.pos.floor();
            let frac = (self.pos - i) as f32;
            let a = if i < 0.0 {
                self.last
            } else {
                input[i as usize]
            };
            let b = if (i + 1.0) < input.len() as f64 {
                input[(i + 1.0) as usize]
            } else {
                *input.last().unwrap_or(&a)
            };
            out.push(a + (b - a) * frac);
            self.pos += step;
        }
        self.pos -= input.len() as f64;
        if let Some(l) = input.last() {
            self.last = *l;
        }
    }
}

/// Counters of the send side.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TxStats {
    /// Voiced frames sent.
    pub frames: u64,
    /// Packets built (frames and end markers).
    pub packets: u64,
    /// Bytes built.
    pub bytes: u64,
}

/// Microphone to wire bytes.
pub struct VoiceTx {
    sender: Sender,
    resample: Resampler,
    rate: u32,
    frame: usize,
    pending: Vec<f32>,
    ready: Vec<i16>,
    muted: bool,
    /// Counters.
    pub stats: TxStats,
    /// True while the gate was open for the last frame.
    pub talking: bool,
}

impl VoiceTx {
    /// A sender for `speaker` (the byte inside the packet) fed at `mic_rate` Hz.
    pub fn new(speaker: u8, mic_rate: u32, activation: Activation, codec: Box<dyn Codec>) -> Self {
        let rate = codec.sample_rate();
        let frame = codec.frame_samples();
        let mode = match activation {
            Activation::PushToTalk => Mode::PushToTalk,
            Activation::VoiceActivity => Mode::VoiceActivity,
        };
        let gate = Gate::new(mode, EnergyVad::new(0.02, 8));
        Self {
            sender: Sender::new(speaker, codec, gate),
            resample: Resampler::new(mic_rate, rate),
            rate,
            frame,
            pending: Vec::new(),
            ready: Vec::new(),
            muted: false,
            stats: TxStats::default(),
            talking: false,
        }
    }

    /// Built-in codec at the default settings.
    pub fn with_default_codec(speaker: u8, mic_rate: u32, activation: Activation) -> Self {
        Self::new(speaker, mic_rate, activation, default_codec())
    }

    /// Codec sample rate.
    pub fn codec_rate(&self) -> u32 {
        self.rate
    }

    /// Push-to-talk key state.
    pub fn set_ptt(&mut self, down: bool) {
        self.sender.gate.set_ptt(down);
    }

    /// Changes how the microphone opens.
    pub fn set_activation(&mut self, a: Activation) {
        self.sender.gate.mode = match a {
            Activation::PushToTalk => Mode::PushToTalk,
            Activation::VoiceActivity => Mode::VoiceActivity,
        };
    }

    /// Self-mute: nothing is sent while on.
    pub fn set_muted(&mut self, m: bool) {
        self.muted = m;
    }

    /// True when self-muted.
    pub fn muted(&self) -> bool {
        self.muted
    }

    /// Feeds microphone samples (mono f32 at the rate given to `new`).
    /// Returns the wire packets to send now.
    pub fn push_mic(&mut self, mic: &[f32]) -> Vec<Vec<u8>> {
        let mut r = Vec::new();
        self.resample.process(mic, &mut r);
        self.pending.extend(r);
        let mut out = Vec::new();
        while self.pending.len() >= self.frame {
            self.ready.clear();
            self.ready.extend(
                self.pending
                    .drain(..self.frame)
                    .map(|s| (s.clamp(-1.0, 1.0) * 32767.0) as i16),
            );
            if self.muted {
                // keep the gate's end-of-talk edge: a muted open mic ends the utterance
                self.sender.gate.set_ptt(false);
                let quiet = vec![0i16; self.frame];
                if let Some(p) = self.sender.process(&quiet) {
                    out.push(self.finish(p));
                }
                self.talking = false;
                continue;
            }
            match self.sender.process(&self.ready) {
                Some(p) => {
                    self.talking = !p.is_end_of_talk();
                    out.push(self.finish(p));
                }
                None => self.talking = false,
            }
        }
        out
    }

    fn finish(&mut self, p: VoicePacket) -> Vec<u8> {
        let b = p.encode();
        if !p.is_end_of_talk() {
            self.stats.frames += 1;
        }
        self.stats.packets += 1;
        self.stats.bytes += b.len() as u64;
        b
    }
}

/// Where the speakers are, as the game last saw them.
#[derive(Clone, Debug, Default)]
pub struct Positions {
    /// Listener position, heading (radians, 0 faces +Y, clockwise) and cell.
    pub listener: ([f32; 3], f32, u32),
    /// Speaker slot to (position, cell).
    pub speakers: BTreeMap<u8, ([f32; 3], u32)>,
}

impl Positions {
    /// Distance gain: full inside [`NEAR`], falling to [`FLOOR`] at [`FAR`].
    pub fn distance_gain(d: f32) -> f32 {
        if d <= NEAR {
            1.0
        } else if d >= FAR {
            FLOOR
        } else {
            1.0 - (1.0 - FLOOR) * (d - NEAR) / (FAR - NEAR)
        }
    }

    fn geometry(&self, s: u8) -> Option<(f32, f32)> {
        let (pos, cell) = self.speakers.get(&s)?;
        let (lp, h, lc) = self.listener;
        if *cell != lc {
            return None;
        }
        let dx = pos[0] - lp[0];
        let dy = pos[1] - lp[1];
        let d = (dx * dx + dy * dy).sqrt();
        // right-hand component of the direction, from the heading
        let (rx, ry) = (h.cos(), -h.sin());
        let pan = if d > 1.0 {
            ((dx * rx + dy * ry) / d).clamp(-1.0, 1.0)
        } else {
            0.0
        };
        // sources very close pan less
        let pan = pan * (d / NEAR).min(1.0);
        Some((d, pan))
    }
}

struct Spatial<'a> {
    pos: &'a Positions,
    volume: &'a BTreeMap<u8, f32>,
    muted: &'a BTreeMap<u8, bool>,
    master: f32,
}

impl Spatializer for Spatial<'_> {
    fn gain(&self, s: u8) -> f32 {
        if self.muted.get(&s).copied().unwrap_or(false) {
            return 0.0;
        }
        let v = self.volume.get(&s).copied().unwrap_or(1.0);
        let g = self
            .pos
            .geometry(s)
            .map_or(OTHER_CELL_GAIN, |(d, _)| Positions::distance_gain(d));
        (g * v * self.master).clamp(0.0, 1.0)
    }

    fn pan(&self, s: u8) -> f32 {
        self.pos.geometry(s).map_or(0.0, |(_, p)| p)
    }
}

/// Counters of the receive side.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RxStats {
    /// Packets accepted.
    pub packets: u64,
    /// Packets that did not parse.
    pub bad: u64,
    /// Packets refused because the speaker is muted.
    pub muted: u64,
    /// Mixed frames with at least one speaker.
    pub mixed: u64,
}

/// Wire bytes of several speakers to one stereo stream.
pub struct VoiceRx {
    mixer: Mixer,
    volume: BTreeMap<u8, f32>,
    muted: BTreeMap<u8, bool>,
    last_heard: BTreeMap<u8, u64>,
    /// Where people stand (set each frame by the game).
    pub positions: Positions,
    master: f32,
    deafened: bool,
    /// Counters.
    pub stats: RxStats,
}

impl Default for VoiceRx {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceRx {
    /// Receiver for the built-in codecs.
    pub fn new() -> Self {
        Self {
            mixer: Mixer::new(codec_for, 3, 12),
            volume: BTreeMap::new(),
            muted: BTreeMap::new(),
            last_heard: BTreeMap::new(),
            positions: Positions::default(),
            master: 1.0,
            deafened: false,
            stats: RxStats::default(),
        }
    }

    /// Per-player volume (0..=2; 1 is normal).
    pub fn set_volume(&mut self, speaker: u8, v: f32) {
        self.volume.insert(speaker, v.clamp(0.0, 2.0));
    }

    /// Per-player volume.
    pub fn volume(&self, speaker: u8) -> f32 {
        self.volume.get(&speaker).copied().unwrap_or(1.0)
    }

    /// Mutes or unmutes one player for this listener.
    pub fn set_muted(&mut self, speaker: u8, m: bool) {
        self.muted.insert(speaker, m);
        if m {
            self.mixer.remove_speaker(speaker);
        }
    }

    /// True when this player is muted.
    pub fn is_muted(&self, speaker: u8) -> bool {
        self.muted.get(&speaker).copied().unwrap_or(false)
    }

    /// Master volume (0..=1).
    pub fn set_master(&mut self, v: f32) {
        self.master = v.clamp(0.0, 1.0);
    }

    /// Hear nobody (and drop what arrives).
    pub fn set_deafened(&mut self, d: bool) {
        self.deafened = d;
    }

    /// A player left: forget them.
    pub fn remove(&mut self, speaker: u8) {
        self.mixer.remove_speaker(speaker);
        self.last_heard.remove(&speaker);
        self.positions.speakers.remove(&speaker);
    }

    /// A packet arrived from `speaker` (the slot the transport says sent it).
    pub fn on_bytes(&mut self, speaker: u8, bytes: &[u8], now_ms: u64) -> bool {
        let Ok(mut p) = VoicePacket::decode(bytes) else {
            self.stats.bad += 1;
            return false;
        };
        p.speaker = speaker;
        if self.deafened {
            return false;
        }
        if self.is_muted(speaker) {
            self.stats.muted += 1;
            return false;
        }
        if !self.mixer_has(speaker) && self.mixer.speaker_count() >= MAX_SPEAKERS {
            return false;
        }
        if !p.is_end_of_talk() {
            self.last_heard.insert(speaker, now_ms);
        }
        self.stats.packets += 1;
        self.mixer.receive(p);
        true
    }

    fn mixer_has(&self, s: u8) -> bool {
        self.mixer.jitter_stats(s).is_some()
    }

    /// One 20 ms stereo frame of `frame_samples` per channel (interleaved).
    pub fn mix(&mut self, frame_samples: usize, out: &mut Vec<f32>) -> usize {
        let sp = Spatial {
            pos: &self.positions,
            volume: &self.volume,
            muted: &self.muted,
            master: self.master,
        };
        let n = self.mixer.mix_frame(frame_samples, &sp, out);
        if n > 0 {
            self.stats.mixed += 1;
        }
        n
    }

    /// Slots heard within [`SPEAKING_HOLD_MS`] of `now_ms`.
    pub fn speaking(&self, now_ms: u64) -> Vec<u8> {
        self.last_heard
            .iter()
            .filter(|(_, t)| now_ms.saturating_sub(**t) <= SPEAKING_HOLD_MS)
            .map(|(s, _)| *s)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(n: usize, rate: u32, amp: f32) -> Vec<f32> {
        (0..n)
            .map(|i| (i as f32 / rate as f32 * 440.0 * std::f32::consts::TAU).sin() * amp)
            .collect()
    }

    fn talk(tx: &mut VoiceTx, ms: u32, mic_rate: u32) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        for _ in 0..(ms / 10) {
            out.extend(tx.push_mic(&tone((mic_rate / 100) as usize, mic_rate, 0.4)));
        }
        out
    }

    #[test]
    fn resampler_keeps_length_and_pitch_across_chunks() {
        let mut r = Resampler::new(48_000, 16_000);
        let mut out = Vec::new();
        let sig = tone(4800, 48_000, 1.0);
        for c in sig.chunks(480) {
            r.process(c, &mut out);
        }
        assert!((out.len() as i32 - 1600).abs() <= 2, "{}", out.len());
        // zero crossings of a 440 Hz tone over 0.1 s: about 88
        let z = out.windows(2).filter(|w| w[0] <= 0.0 && w[1] > 0.0).count();
        assert!((43..=45).contains(&z), "{z}");
        let mut same = Resampler::new(16_000, 16_000);
        let mut o = Vec::new();
        same.process(&[0.5, 0.25], &mut o);
        assert_eq!(o, vec![0.5, 0.25]);
    }

    #[test]
    fn ptt_gates_the_microphone_and_ends_the_utterance() {
        let mut tx = VoiceTx::with_default_codec(1, 48_000, Activation::PushToTalk);
        assert!(talk(&mut tx, 200, 48_000).is_empty(), "closed");
        tx.set_ptt(true);
        let open = talk(&mut tx, 200, 48_000);
        assert!(open.len() >= 9, "{}", open.len());
        assert!(
            open.iter().all(|b| b.len() <= 480 + 12 + 12),
            "fits one voice message"
        );
        tx.set_ptt(false);
        let tail = talk(&mut tx, 100, 48_000);
        assert!(VoicePacket::decode(&tail[0]).unwrap().is_end_of_talk());
        assert_eq!(tail.len(), 1, "one end marker");
        assert!(tx.stats.frames >= 9 && tx.stats.packets == tx.stats.frames + 1);
    }

    #[test]
    fn vad_opens_on_voice_and_not_on_silence() {
        let mut tx = VoiceTx::with_default_codec(1, 16_000, Activation::VoiceActivity);
        for _ in 0..20 {
            assert!(tx.push_mic(&vec![0.0; 160]).is_empty());
        }
        assert!(!talk(&mut tx, 100, 16_000).is_empty());
    }

    #[test]
    fn self_mute_stops_sending_and_ends_the_utterance() {
        let mut tx = VoiceTx::with_default_codec(1, 16_000, Activation::PushToTalk);
        tx.set_ptt(true);
        assert!(!talk(&mut tx, 100, 16_000).is_empty());
        tx.set_muted(true);
        let after = talk(&mut tx, 200, 16_000);
        assert_eq!(after.len(), 1);
        assert!(VoicePacket::decode(&after[0]).unwrap().is_end_of_talk());
        assert!(talk(&mut tx, 200, 16_000).is_empty());
    }

    fn deliver(rx: &mut VoiceRx, from: u8, pk: &[Vec<u8>], now: u64) {
        for b in pk {
            assert!(rx.on_bytes(from, b, now));
        }
    }

    fn energy(out: &[f32]) -> (f32, f32) {
        (
            out.iter().step_by(2).map(|x| x.abs()).sum(),
            out.iter().skip(1).step_by(2).map(|x| x.abs()).sum(),
        )
    }

    #[test]
    fn rx_pans_by_position_and_attenuates_by_distance() {
        let mut tx = VoiceTx::with_default_codec(2, 16_000, Activation::PushToTalk);
        tx.set_ptt(true);
        let pk = talk(&mut tx, 200, 16_000);
        let mut near = VoiceRx::new();
        near.positions.listener = ([0.0; 3], 0.0, 1);
        near.positions.speakers.insert(2, ([2000.0, 0.0, 0.0], 1));
        deliver(&mut near, 2, &pk, 0);
        let mut far = VoiceRx::new();
        far.positions.listener = ([0.0; 3], 0.0, 1);
        far.positions.speakers.insert(2, ([0.0, 6500.0, 0.0], 1));
        deliver(&mut far, 2, &pk, 0);
        let (mut a, mut b) = (Vec::new(), Vec::new());
        let (mut ea, mut eb) = ((0.0, 0.0), (0.0, 0.0));
        for _ in 0..4 {
            near.mix(320, &mut a);
            far.mix(320, &mut b);
            let (l, r) = energy(&a);
            ea = (ea.0 + l, ea.1 + r);
            let (l, r) = energy(&b);
            eb = (eb.0 + l, eb.1 + r);
        }
        assert!(
            ea.1 > ea.0 * 3.0,
            "speaker to the east is in the right ear: {ea:?}"
        );
        let centre = (eb.0 - eb.1).abs() / (eb.0 + eb.1);
        assert!(centre < 0.1, "straight ahead is centred: {eb:?}");
        assert!(
            eb.0 + eb.1 < (ea.0 + ea.1) * 0.7,
            "far is quieter: {ea:?} {eb:?}"
        );
    }

    #[test]
    fn facing_east_moves_an_eastern_speaker_to_the_centre() {
        let mut p = Positions::default();
        p.listener = ([0.0; 3], std::f32::consts::FRAC_PI_2, 1);
        p.speakers.insert(1, ([3000.0, 0.0, 0.0], 1));
        let (_, pan) = p.geometry(1).unwrap();
        assert!(pan.abs() < 1e-4, "{pan}");
    }

    #[test]
    fn other_cells_are_heard_flat_and_unknown_speakers_too() {
        let mut p = Positions::default();
        p.listener = ([0.0; 3], 0.0, 1);
        p.speakers.insert(1, ([0.0; 3], 2));
        assert!(p.geometry(1).is_none());
        let sp = Spatial {
            pos: &p,
            volume: &BTreeMap::new(),
            muted: &BTreeMap::new(),
            master: 1.0,
        };
        assert_eq!(sp.gain(1), OTHER_CELL_GAIN);
        assert_eq!(sp.gain(9), OTHER_CELL_GAIN);
    }

    #[test]
    fn mute_and_volume_are_per_player() {
        let mut tx = VoiceTx::with_default_codec(1, 16_000, Activation::PushToTalk);
        tx.set_ptt(true);
        let pk = talk(&mut tx, 200, 16_000);
        let mut rx = VoiceRx::new();
        rx.set_muted(3, true);
        assert!(!rx.on_bytes(3, &pk[0], 0));
        assert_eq!(rx.stats.muted, 1);
        rx.set_volume(1, 0.0);
        deliver(&mut rx, 1, &pk, 0);
        let mut out = Vec::new();
        for _ in 0..4 {
            rx.mix(320, &mut out);
            assert!(out.iter().all(|s| *s == 0.0), "volume 0 is silent");
        }
        rx.set_volume(1, 1.0);
        assert_eq!(rx.volume(1), 1.0);
        rx.set_volume(1, 9.0);
        assert_eq!(rx.volume(1), 2.0);
    }

    #[test]
    fn speaking_list_expires_and_deafen_drops() {
        let mut tx = VoiceTx::with_default_codec(1, 16_000, Activation::PushToTalk);
        tx.set_ptt(true);
        let pk = talk(&mut tx, 100, 16_000);
        let mut rx = VoiceRx::new();
        rx.on_bytes(4, &pk[0], 1000);
        assert_eq!(rx.speaking(1100), vec![4]);
        assert!(rx.speaking(1000 + SPEAKING_HOLD_MS + 1).is_empty());
        rx.set_deafened(true);
        assert!(!rx.on_bytes(5, &pk[0], 2000));
        assert!(rx.speaking(2000).is_empty());
    }

    #[test]
    fn the_speaker_byte_comes_from_the_transport_not_the_packet() {
        let mut tx = VoiceTx::with_default_codec(7, 16_000, Activation::PushToTalk);
        tx.set_ptt(true);
        let pk = talk(&mut tx, 40, 16_000);
        let mut rx = VoiceRx::new();
        rx.on_bytes(2, &pk[0], 0);
        assert_eq!(rx.speaking(0), vec![2]);
    }

    #[test]
    fn garbage_is_counted_not_played() {
        let mut rx = VoiceRx::new();
        assert!(!rx.on_bytes(1, &[1, 2, 3], 0));
        assert_eq!(rx.stats.bad, 1);
    }

    #[test]
    fn at_most_max_speakers_are_mixed() {
        let mut tx = VoiceTx::with_default_codec(1, 16_000, Activation::PushToTalk);
        tx.set_ptt(true);
        let pk = talk(&mut tx, 40, 16_000);
        let mut rx = VoiceRx::new();
        let mut accepted = 0;
        for s in 0..(MAX_SPEAKERS as u8 + 4) {
            if rx.on_bytes(s, &pk[0], 0) {
                accepted += 1;
            }
        }
        assert_eq!(accepted, MAX_SPEAKERS);
    }

    #[test]
    fn a_whole_call_survives_loss_with_the_jitter_buffer() {
        let mut tx = VoiceTx::with_default_codec(1, 16_000, Activation::PushToTalk);
        tx.set_ptt(true);
        let pk = talk(&mut tx, 1000, 16_000);
        let mut rx = VoiceRx::new();
        let mut out = Vec::new();
        let mut voiced = 0;
        for (i, b) in pk.iter().enumerate() {
            if i % 7 != 3 {
                rx.on_bytes(1, b, i as u64 * 20);
            }
            if rx.mix(320, &mut out) > 0 {
                voiced += 1;
            }
        }
        assert!(voiced >= pk.len() * 8 / 10, "{voiced} of {}", pk.len());
    }
}
