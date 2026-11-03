//! Microphone capture and speaker playback behind the `capture` feature
//! (cpal). Not compiled by default or in tests: the pure pipeline lives in
//! [`crate::pipeline`] and is what the tests cover. This file only moves
//! samples between the sound card and two queues.
//!
//! Capture: the device callback pushes mono f32 at the device rate into
//! [`Capture::queue`]; the game drains it each frame with [`Capture::take`]
//! and feeds [`crate::pipeline::VoiceTx::push_mic`] (which resamples).
//!
//! Playback: the game mixes stereo frames at the codec rate with
//! [`crate::pipeline::VoiceRx::mix`] and hands them to [`Playback::push`],
//! which resamples to the device rate. The output callback plays silence
//! when the queue runs dry and the queue never holds more than
//! [`MAX_QUEUED_MS`] of audio.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::pipeline::Resampler;

/// Most audio kept waiting in a queue (ms); older samples are dropped.
pub const MAX_QUEUED_MS: u32 = 250;

/// Shared queue of samples.
pub type SampleQueue = Arc<Mutex<VecDeque<f32>>>;

fn lock(q: &SampleQueue) -> std::sync::MutexGuard<'_, VecDeque<f32>> {
    q.lock().unwrap_or_else(|e| e.into_inner())
}

/// The microphone.
pub struct Capture {
    _stream: cpal::Stream,
    /// Mono samples at [`Capture::device_rate`].
    pub queue: SampleQueue,
    /// The device's sample rate.
    pub device_rate: u32,
}

impl Capture {
    /// Takes everything captured since the last call.
    pub fn take(&self, out: &mut Vec<f32>) {
        out.extend(lock(&self.queue).drain(..));
    }
}

fn push_mono<T: Copy>(
    q: &SampleQueue,
    data: &[T],
    channels: usize,
    rate: u32,
    to_f32: impl Fn(T) -> f32,
) {
    let mut g = lock(q);
    for fr in data.chunks(channels.max(1)) {
        g.push_back(fr.iter().map(|s| to_f32(*s)).sum::<f32>() / fr.len() as f32);
    }
    let cap = (rate as usize * MAX_QUEUED_MS as usize) / 1000;
    while g.len() > cap {
        g.pop_front();
    }
}

/// Opens the default input device.
pub fn start() -> Result<Capture, String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let host = cpal::default_host();
    let dev = host.default_input_device().ok_or("no input device")?;
    let cfg = dev.default_input_config().map_err(|e| e.to_string())?;
    let channels = cfg.channels() as usize;
    let device_rate: u32 = cfg.sample_rate().0;
    let format = cfg.sample_format();
    let queue: SampleQueue = Arc::new(Mutex::new(VecDeque::new()));
    let q = queue.clone();
    let err = |e: cpal::StreamError| eprintln!("co-op voice capture error: {e}");
    let conf: cpal::StreamConfig = cfg.into();
    let stream = match format {
        cpal::SampleFormat::F32 => dev.build_input_stream(
            &conf,
            move |d: &[f32], _| push_mono(&q, d, channels, device_rate, |s| s),
            err,
            None,
        ),
        cpal::SampleFormat::I16 => dev.build_input_stream(
            &conf,
            move |d: &[i16], _| push_mono(&q, d, channels, device_rate, |s| f32::from(s) / 32768.0),
            err,
            None,
        ),
        cpal::SampleFormat::U16 => dev.build_input_stream(
            &conf,
            move |d: &[u16], _| {
                push_mono(&q, d, channels, device_rate, |s| {
                    (f32::from(s) - 32768.0) / 32768.0
                })
            },
            err,
            None,
        ),
        other => return Err(format!("unsupported microphone sample format {other:?}")),
    }
    .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(Capture {
        _stream: stream,
        queue,
        device_rate,
    })
}

/// The speakers.
pub struct Playback {
    _stream: cpal::Stream,
    queue: SampleQueue,
    /// The device's sample rate.
    pub device_rate: u32,
    channels: usize,
    left: Resampler,
    right: Resampler,
    from: u32,
}

impl Playback {
    /// Queues one interleaved stereo block at `from` Hz.
    pub fn push(&mut self, stereo: &[f32], from: u32) {
        if from != self.from {
            self.from = from;
            self.left = Resampler::new(from, self.device_rate);
            self.right = Resampler::new(from, self.device_rate);
        }
        let l: Vec<f32> = stereo.iter().step_by(2).copied().collect();
        let r: Vec<f32> = stereo.iter().skip(1).step_by(2).copied().collect();
        let (mut lo, mut ro) = (Vec::new(), Vec::new());
        self.left.process(&l, &mut lo);
        self.right.process(&r, &mut ro);
        let mut g = lock(&self.queue);
        for (a, b) in lo.iter().zip(&ro) {
            g.push_back(*a);
            g.push_back(*b);
        }
        let cap = (self.device_rate as usize * MAX_QUEUED_MS as usize / 1000) * 2;
        while g.len() > cap {
            g.pop_front();
        }
        let _ = self.channels;
    }

    /// Milliseconds of audio waiting.
    pub fn queued_ms(&self) -> u32 {
        (lock(&self.queue).len() as u32 / 2) * 1000 / self.device_rate.max(1)
    }
}

fn fill<T: Copy>(
    q: &SampleQueue,
    out: &mut [T],
    channels: usize,
    from_f32: impl Fn(f32) -> T,
    silence: T,
) {
    let mut g = lock(q);
    for fr in out.chunks_mut(channels.max(1)) {
        let (l, r) = match (g.pop_front(), g.pop_front()) {
            (Some(l), Some(r)) => (l, r),
            _ => (0.0, 0.0),
        };
        for (i, s) in fr.iter_mut().enumerate() {
            *s = match i {
                0 => from_f32(l),
                1 => from_f32(r),
                _ => silence,
            };
        }
        if fr.len() == 1 {
            fr[0] = from_f32((l + r) * 0.5);
        }
    }
}

/// Opens the default output device.
pub fn start_playback() -> Result<Playback, String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let host = cpal::default_host();
    let dev = host.default_output_device().ok_or("no output device")?;
    let cfg = dev.default_output_config().map_err(|e| e.to_string())?;
    let channels = cfg.channels() as usize;
    let device_rate: u32 = cfg.sample_rate().0;
    let format = cfg.sample_format();
    let queue: SampleQueue = Arc::new(Mutex::new(VecDeque::new()));
    let q = queue.clone();
    let err = |e: cpal::StreamError| eprintln!("co-op voice playback error: {e}");
    let conf: cpal::StreamConfig = cfg.into();
    let stream = match format {
        cpal::SampleFormat::F32 => dev.build_output_stream(
            &conf,
            move |d: &mut [f32], _| fill(&q, d, channels, |s| s, 0.0),
            err,
            None,
        ),
        cpal::SampleFormat::I16 => dev.build_output_stream(
            &conf,
            move |d: &mut [i16], _| {
                fill(
                    &q,
                    d,
                    channels,
                    |s| (s.clamp(-1.0, 1.0) * 32767.0) as i16,
                    0,
                )
            },
            err,
            None,
        ),
        cpal::SampleFormat::U16 => dev.build_output_stream(
            &conf,
            move |d: &mut [u16], _| {
                fill(
                    &q,
                    d,
                    channels,
                    |s| ((s.clamp(-1.0, 1.0) * 32767.0) + 32768.0) as u16,
                    32768,
                )
            },
            err,
            None,
        ),
        other => return Err(format!("unsupported speaker sample format {other:?}")),
    }
    .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(Playback {
        _stream: stream,
        queue,
        device_rate,
        channels,
        left: Resampler::new(16_000, device_rate),
        right: Resampler::new(16_000, device_rate),
        from: 16_000,
    })
}
