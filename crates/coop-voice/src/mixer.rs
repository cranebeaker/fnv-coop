//! Per-speaker receive side: jitter buffer + decoder per speaker, mixed to
//! interleaved stereo f32 with distance/pan hooks, plus the send side.

use crate::codec::Codec;
use crate::envelope::{VoicePacket, FLAG_END_OF_TALK, FLAG_PTT_ACTIVE};
use crate::jitter::{JitterBuffer, Pop};
use crate::vad::Gate;
use std::collections::BTreeMap;

/// Game side hook: where is this speaker relative to the listener.
pub trait Spatializer {
    /// Linear gain 0.0..=1.0 (distance attenuation, mutes, ducking).
    fn gain(&self, speaker: u8) -> f32;
    /// Pan -1.0 (left) .. 1.0 (right).
    fn pan(&self, speaker: u8) -> f32;
}

/// Everyone at full volume, centered.
pub struct Flat;
impl Spatializer for Flat {
    fn gain(&self, _: u8) -> f32 {
        1.0
    }
    fn pan(&self, _: u8) -> f32 {
        0.0
    }
}

struct Speaker {
    jitter: JitterBuffer,
    codec: Box<dyn Codec>,
    scratch: Vec<i16>,
}

pub struct Mixer {
    make_codec: Box<dyn Fn(u8) -> Option<Box<dyn Codec>> + Send>,
    jitter_target: usize,
    jitter_max: usize,
    speakers: BTreeMap<u8, Speaker>,
    pub master_gain: f32,
}

impl Mixer {
    /// `make_codec(codec_id)` builds a decoder for a speaker's codec id.
    pub fn new(
        make_codec: impl Fn(u8) -> Option<Box<dyn Codec>> + Send + 'static,
        jitter_target: usize,
        jitter_max: usize,
    ) -> Self {
        Self {
            make_codec: Box::new(make_codec),
            jitter_target,
            jitter_max,
            speakers: BTreeMap::new(),
            master_gain: 1.0,
        }
    }

    pub fn speaker_count(&self) -> usize {
        self.speakers.len()
    }

    pub fn jitter_stats(&self, speaker: u8) -> Option<crate::jitter::Stats> {
        self.speakers.get(&speaker).map(|s| s.jitter.stats)
    }

    pub fn remove_speaker(&mut self, speaker: u8) {
        self.speakers.remove(&speaker);
    }

    pub fn receive(&mut self, p: VoicePacket) {
        if !self.speakers.contains_key(&p.speaker) {
            let Some(codec) = (self.make_codec)(p.codec) else {
                return;
            };
            self.speakers.insert(
                p.speaker,
                Speaker {
                    jitter: JitterBuffer::new(self.jitter_target, self.jitter_max),
                    codec,
                    scratch: Vec::new(),
                },
            );
        }
        let sp = self.speakers.get_mut(&p.speaker).unwrap();
        if p.codec != sp.codec.id() {
            match (self.make_codec)(p.codec) {
                Some(c) => sp.codec = c,
                None => return,
            }
        }
        if p.is_end_of_talk() {
            sp.jitter.finish();
        } else {
            sp.jitter.push(p.seq, p.payload);
        }
    }

    /// Produce one frame of interleaved stereo into `out` (cleared first).
    /// Returns the number of speakers that contributed.
    pub fn mix_frame(
        &mut self,
        frame_samples: usize,
        spatial: &dyn Spatializer,
        out: &mut Vec<f32>,
    ) -> usize {
        out.clear();
        out.resize(frame_samples * 2, 0.0);
        let mut active = 0;
        for (&id, sp) in self.speakers.iter_mut() {
            let popped = sp.jitter.pop();
            sp.scratch.clear();
            let ok = match &popped {
                Pop::Idle => continue,
                Pop::Frame(d) => sp.codec.decode(Some(d), &mut sp.scratch).is_ok(),
                Pop::Conceal => sp.codec.decode(None, &mut sp.scratch).is_ok(),
            };
            if !ok || sp.scratch.len() != frame_samples {
                continue;
            }
            active += 1;
            let g = spatial.gain(id).clamp(0.0, 1.0) * self.master_gain;
            let pan = spatial.pan(id).clamp(-1.0, 1.0);
            // constant-power pan
            let a = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
            let (gl, gr) = (a.cos() * g, a.sin() * g);
            for (i, s) in sp.scratch.iter().enumerate() {
                let v = *s as f32 / 32768.0;
                out[i * 2] += v * gl;
                out[i * 2 + 1] += v * gr;
            }
        }
        for s in out.iter_mut() {
            *s = s.clamp(-1.0, 1.0);
        }
        active
    }
}

/// Send side: gate + codec + packetizer for the local player.
pub struct Sender {
    pub gate: Gate,
    codec: Box<dyn Codec>,
    speaker: u8,
    seq: u16,
    timestamp: u32,
    talking: bool,
}

impl Sender {
    pub fn new(speaker: u8, codec: Box<dyn Codec>, gate: Gate) -> Self {
        Self {
            gate,
            codec,
            speaker,
            seq: 0,
            timestamp: 0,
            talking: false,
        }
    }

    /// Feed one capture frame. Returns a packet to send, if any. On the
    /// open-to-closed edge an empty END_OF_TALK packet is returned once.
    pub fn process(&mut self, pcm: &[i16]) -> Option<VoicePacket> {
        let open = self.gate.open(pcm);
        let ts = self.timestamp;
        self.timestamp = self
            .timestamp
            .wrapping_add(self.codec.frame_samples() as u32);
        if open {
            self.talking = true;
            let mut payload = Vec::new();
            self.codec.encode(pcm, &mut payload);
            let seq = self.seq;
            self.seq = self.seq.wrapping_add(1);
            let flags = if self.gate.ptt_down() {
                FLAG_PTT_ACTIVE
            } else {
                0
            };
            Some(VoicePacket {
                flags,
                codec: self.codec.id(),
                speaker: self.speaker,
                seq,
                timestamp: ts,
                payload,
            })
        } else if self.talking {
            self.talking = false;
            // the end marker has its own sequence number: the next utterance must not reuse it
            let seq = self.seq;
            self.seq = self.seq.wrapping_add(1);
            Some(VoicePacket {
                flags: FLAG_END_OF_TALK,
                codec: self.codec.id(),
                speaker: self.speaker,
                seq,
                timestamp: ts,
                payload: Vec::new(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{AdpcmCodec, CODEC_ADPCM};
    use crate::vad::{EnergyVad, Mode};

    pub fn factory(id: u8) -> Option<Box<dyn Codec>> {
        (id == CODEC_ADPCM).then(|| Box::new(AdpcmCodec::new(16000, 320)) as Box<dyn Codec>)
    }

    struct Hard;
    impl Spatializer for Hard {
        fn gain(&self, _: u8) -> f32 {
            1.0
        }
        fn pan(&self, s: u8) -> f32 {
            if s == 1 {
                -1.0
            } else {
                1.0
            }
        }
    }

    fn tone() -> Vec<i16> {
        (0..320)
            .map(|i| ((i as f32 * 0.2).sin() * 10000.0) as i16)
            .collect()
    }

    #[test]
    fn sender_edges_and_mix_pan() {
        let gate = Gate::new(Mode::PushToTalk, EnergyVad::new(0.01, 0));
        let mut s = Sender::new(1, Box::new(AdpcmCodec::new(16000, 320)), gate);
        assert!(s.process(&tone()).is_none());
        s.gate.set_ptt(true);
        let mut m = Mixer::new(factory, 1, 8);
        for _ in 0..3 {
            let p = s.process(&tone()).unwrap();
            m.receive(VoicePacket::decode(&p.encode()).unwrap());
        }
        s.gate.set_ptt(false);
        let end = s.process(&tone()).unwrap();
        assert!(end.is_end_of_talk());
        assert!(s.process(&tone()).is_none());

        let mut out = vec![];
        assert_eq!(m.mix_frame(320, &Hard, &mut out), 1);
        let l: f32 = out.iter().step_by(2).map(|x| x.abs()).sum();
        let r: f32 = out.iter().skip(1).step_by(2).map(|x| x.abs()).sum();
        assert!(l > 1.0 && r < 0.01, "l {l} r {r}");
    }
}
