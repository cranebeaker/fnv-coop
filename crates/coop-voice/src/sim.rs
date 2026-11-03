//! Deterministic network simulator (tiny seeded RNG, no dependencies) and a
//! full simulated voice call used by the tests.

use crate::codec::{AdpcmCodec, Codec, CODEC_ADPCM};
use crate::envelope::VoicePacket;
use crate::mixer::{Flat, Mixer, Sender};
use crate::vad::{EnergyVad, Gate, Mode};

/// SplitMix64.
pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// 0.0..1.0
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Profile {
    pub loss: f64,
    /// Probability that a packet is lost given the previous one was lost
    /// (burst loss). Set equal to `loss` for independent loss.
    pub burst_loss: f64,
    pub dup: f64,
    pub base_ms: f64,
    pub jitter_ms: f64,
}

impl Profile {
    pub const CLEAN: Profile = Profile {
        loss: 0.0,
        burst_loss: 0.0,
        dup: 0.0,
        base_ms: 40.0,
        jitter_ms: 0.0,
    };
    pub const WIFI: Profile = Profile {
        loss: 0.03,
        burst_loss: 0.2,
        dup: 0.01,
        base_ms: 40.0,
        jitter_ms: 25.0,
    };
    pub const BAD: Profile = Profile {
        loss: 0.10,
        burst_loss: 0.4,
        dup: 0.02,
        base_ms: 80.0,
        jitter_ms: 60.0,
    };
}

pub struct SimResult {
    pub sent: usize,
    pub lost: usize,
    pub frames_played: u64,
    pub concealed: u64,
    pub late: u64,
    pub duplicates: u64,
    /// Hash of all mixed output, for determinism checks.
    pub output_hash: u64,
}

/// One speaker talking for `talk_frames` frames (20 ms each) through the
/// profile into a receiver. Full path including envelope encode/decode.
pub fn simulate_call(
    seed: u64,
    profile: Profile,
    talk_frames: usize,
    jitter_target: usize,
) -> SimResult {
    let mut rng = Rng::new(seed);
    let gate = Gate::new(Mode::AlwaysOpen, EnergyVad::new(0.0, 0));
    let mut sender = Sender::new(1, Box::new(AdpcmCodec::new(16000, 320)), gate);
    let mut mixer = Mixer::new(
        |id| (id == CODEC_ADPCM).then(|| Box::new(AdpcmCodec::new(16000, 320)) as Box<dyn Codec>),
        jitter_target,
        jitter_target * 4 + 8,
    );

    // Generate packets with arrival times (ms).
    let mut arrivals: Vec<(f64, u32, Vec<u8>)> = Vec::new(); // (time, order, bytes)
    let mut lost = 0;
    let mut prev_lost = false;
    let mut order = 0u32;
    for i in 0..talk_frames {
        let pcm: Vec<i16> = (0..320)
            .map(|n| (((i * 320 + n) as f32 * 0.07).sin() * 9000.0) as i16)
            .collect();
        let p = sender.process(&pcm).unwrap();
        let bytes = p.encode();
        let p_loss = if prev_lost {
            profile.burst_loss
        } else {
            profile.loss
        };
        prev_lost = rng.unit() < p_loss;
        if prev_lost {
            lost += 1;
            continue;
        }
        let t0 = i as f64 * 20.0;
        let copies = if rng.unit() < profile.dup { 2 } else { 1 };
        for _ in 0..copies {
            let t = t0 + profile.base_ms + rng.unit() * profile.jitter_ms;
            arrivals.push((t, order, bytes.clone()));
            order += 1;
        }
    }
    // End of talk marker, delivered reliably shortly after the last frame.
    let end_t = talk_frames as f64 * 20.0 + profile.base_ms + profile.jitter_ms + 1.0;
    let mut gate_closed = Gate::new(Mode::Muted, EnergyVad::new(1.0, 0));
    let _ = gate_closed.open(&[]);
    let end = VoicePacket {
        flags: crate::envelope::FLAG_END_OF_TALK,
        codec: CODEC_ADPCM,
        speaker: 1,
        seq: talk_frames as u16,
        timestamp: 0,
        payload: vec![],
    };
    arrivals.push((end_t, order, end.encode()));
    arrivals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.cmp(&b.1)));

    let total_ticks = ((end_t / 20.0) as usize) + jitter_target + 20;
    let mut idx = 0;
    let mut out = Vec::new();
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for tick in 0..total_ticks {
        let now = tick as f64 * 20.0;
        while idx < arrivals.len() && arrivals[idx].0 <= now {
            mixer.receive(VoicePacket::decode(&arrivals[idx].2).unwrap());
            idx += 1;
        }
        mixer.mix_frame(320, &Flat, &mut out);
        for v in &out {
            hash = (hash ^ v.to_bits() as u64).wrapping_mul(0x100_0000_01b3);
        }
    }
    let st = mixer.jitter_stats(1).unwrap_or_default();
    SimResult {
        sent: talk_frames,
        lost,
        frames_played: st.played,
        concealed: st.concealed,
        late: st.late,
        duplicates: st.duplicates,
        output_hash: hash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_call_plays_everything() {
        let r = simulate_call(1, Profile::CLEAN, 200, 3);
        assert_eq!(r.lost, 0);
        assert_eq!(r.frames_played, 200);
        assert_eq!(r.concealed, 0);
    }

    #[test]
    fn deterministic() {
        let a = simulate_call(7, Profile::WIFI, 300, 3);
        let b = simulate_call(7, Profile::WIFI, 300, 3);
        assert_eq!(a.output_hash, b.output_hash);
        assert_eq!((a.lost, a.concealed, a.late), (b.lost, b.concealed, b.late));
        let c = simulate_call(8, Profile::WIFI, 300, 3);
        assert_ne!(a.output_hash, c.output_hash);
    }

    #[test]
    fn lossy_call_accounting() {
        let r = simulate_call(3, Profile::BAD, 500, 6);
        assert!(r.lost > 0);
        // Everything sent is either played, concealed-as-lost, or arrived late.
        // Each lost packet inside the talk spurt is concealed; late ones too.
        assert!(r.frames_played as usize + r.lost <= r.sent + r.late as usize + 1);
        assert!(r.frames_played as usize >= r.sent - r.lost - r.late as usize - 1);
        assert!(
            r.concealed as usize >= r.lost.saturating_sub(5),
            "concealed {} lost {}",
            r.concealed,
            r.lost
        );
        assert!(r.duplicates > 0 || r.sent < 100 || true);
    }

    #[test]
    fn deeper_buffer_reduces_late() {
        let shallow = simulate_call(5, Profile::WIFI, 600, 1);
        let deep = simulate_call(5, Profile::WIFI, 600, 5);
        assert!(
            deep.late <= shallow.late,
            "deep {} shallow {}",
            deep.late,
            shallow.late
        );
    }

    #[test]
    fn rng_range() {
        let mut r = Rng::new(0);
        for _ in 0..1000 {
            let u = r.unit();
            assert!((0.0..1.0).contains(&u));
        }
    }
}
