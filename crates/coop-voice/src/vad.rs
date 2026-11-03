//! Energy VAD and push-to-talk gate.

/// RMS of a frame, 0.0..=1.0 (full scale).
pub fn rms(frame: &[i16]) -> f32 {
    if frame.is_empty() {
        return 0.0;
    }
    let sum: f64 = frame.iter().map(|s| (*s as f64 / 32768.0).powi(2)).sum();
    (sum / frame.len() as f64).sqrt() as f32
}

/// Simple energy detector with a hangover so word endings are not clipped.
pub struct EnergyVad {
    pub threshold: f32,
    pub hangover_frames: u32,
    left: u32,
}

impl EnergyVad {
    pub fn new(threshold: f32, hangover_frames: u32) -> Self {
        Self {
            threshold,
            hangover_frames,
            left: 0,
        }
    }

    pub fn process(&mut self, frame: &[i16]) -> bool {
        if rms(frame) >= self.threshold {
            self.left = self.hangover_frames;
            true
        } else if self.left > 0 {
            self.left -= 1;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    PushToTalk,
    VoiceActivity,
    AlwaysOpen,
    Muted,
}

/// Decides whether a captured frame is transmitted.
pub struct Gate {
    pub mode: Mode,
    ptt_down: bool,
    vad: EnergyVad,
}

impl Gate {
    pub fn new(mode: Mode, vad: EnergyVad) -> Self {
        Self {
            mode,
            ptt_down: false,
            vad,
        }
    }

    pub fn set_ptt(&mut self, down: bool) {
        self.ptt_down = down;
    }

    pub fn ptt_down(&self) -> bool {
        self.ptt_down
    }

    /// Always feeds the VAD so its hangover stays consistent across modes.
    pub fn open(&mut self, frame: &[i16]) -> bool {
        let voiced = self.vad.process(frame);
        match self.mode {
            Mode::PushToTalk => self.ptt_down,
            Mode::VoiceActivity => voiced,
            Mode::AlwaysOpen => true,
            Mode::Muted => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vad_hangover() {
        let mut v = EnergyVad::new(0.05, 2);
        let loud = vec![10000i16; 160];
        let quiet = vec![0i16; 160];
        assert!(!v.process(&quiet));
        assert!(v.process(&loud));
        assert!(v.process(&quiet));
        assert!(v.process(&quiet));
        assert!(!v.process(&quiet));
    }

    #[test]
    fn ptt_and_modes() {
        let mut g = Gate::new(Mode::PushToTalk, EnergyVad::new(0.05, 0));
        let quiet = vec![0i16; 160];
        assert!(!g.open(&quiet));
        g.set_ptt(true);
        assert!(g.open(&quiet));
        g.mode = Mode::Muted;
        assert!(!g.open(&quiet));
        g.mode = Mode::VoiceActivity;
        assert!(!g.open(&quiet));
        assert!(g.open(&vec![9000i16; 160]));
    }
}
