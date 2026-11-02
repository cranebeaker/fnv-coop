//! Microphone capture behind the `capture` feature (cpal + rubato).
//! Not compiled by default or in tests; NOT yet built or verified against
//! cpal 0.18 / rubato 0.15. Adjust to the exact API on first real build.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Shared queue of mono f32 samples at the device rate.
pub type SampleQueue = Arc<Mutex<VecDeque<f32>>>;

pub struct Capture {
    _stream: cpal::Stream,
    pub queue: SampleQueue,
    pub device_rate: u32,
}

pub fn start() -> Result<Capture, String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let host = cpal::default_host();
    let dev = host.default_input_device().ok_or("no input device")?;
    let cfg = dev.default_input_config().map_err(|e| e.to_string())?;
    let channels = cfg.channels() as usize;
    let device_rate = cfg.sample_rate().0;
    let queue: SampleQueue = Arc::new(Mutex::new(VecDeque::new()));
    let q = queue.clone();
    let stream = dev
        .build_input_stream(
            &cfg.into(),
            move |data: &[f32], _| {
                let mut g = q.lock().unwrap();
                for fr in data.chunks(channels) {
                    g.push_back(fr.iter().sum::<f32>() / channels as f32);
                }
                // bound latency: keep at most 1 s
                while g.len() > device_rate as usize {
                    g.pop_front();
                }
            },
            |e| eprintln!("capture error: {e}"),
            None,
        )
        .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(Capture { _stream: stream, queue, device_rate })
}

/// Resample mono f32 from `from` Hz to `to` Hz in fixed chunks.
pub struct Resampler {
    inner: rubato::FastFixedIn<f32>,
    chunk: usize,
}

impl Resampler {
    pub fn new(from: u32, to: u32, chunk: usize) -> Result<Self, String> {
        let inner = rubato::FastFixedIn::<f32>::new(
            to as f64 / from as f64,
            1.0,
            rubato::PolynomialDegree::Cubic,
            chunk,
            1,
        )
        .map_err(|e| e.to_string())?;
        Ok(Self { inner, chunk })
    }

    pub fn chunk(&self) -> usize { self.chunk }

    /// `input` must hold exactly `chunk` samples.
    pub fn process(&mut self, input: &[f32]) -> Result<Vec<i16>, String> {
        use rubato::Resampler as _;
        let out = self.inner.process(&[input.to_vec()], None).map_err(|e| e.to_string())?;
        Ok(out[0].iter().map(|s| (s.clamp(-1.0, 1.0) * 32767.0) as i16).collect())
    }
}
