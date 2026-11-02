//! The game's music files (`Data\Music`, MP3) for the music manager
//! (`world::music`) and for whoever plays its decks: how long a track is,
//! what a folder holds, and the samples, decoded with the project's own
//! decoder (`mp3`) on a thread of their own a little ahead of playback, so
//! a long track never holds up a frame.
//!
//! A deck starts a track where the manager says (a new layer starts at
//! the old one's position): the decoder runs through the track up to that
//! point and adds the time that took, so the sound lines up with the
//! deck's clock.

use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, Receiver, TryRecvError};
use std::sync::Arc;
use std::time::Instant;

use assets::{Assets, IniSettings};
use world::music::{Deck, MusicFiles, Volumes};

/// The music's volumes from the game's INI settings (`[Audio]`
/// `fDefaultMasterVolume`, `fDefaultMusicVolume`, `fDefaultRadioVolume`,
/// `fMainMenuMusicVolume`; where no file sets one, this install's
/// `Fallout_default.ini` values: 1, 0.6, 0.5, 0.6).
pub fn volumes(settings: &IniSettings) -> Volumes {
    let d = Volumes::default();
    let get = |key: &str, default: f32| settings.float("Audio", key).unwrap_or(default);
    Volumes {
        master: get("fDefaultMasterVolume", d.master),
        music: get("fDefaultMusicVolume", d.music),
        radio: get("fDefaultRadioVolume", d.radio),
        menu: get("fMainMenuMusicVolume", d.menu),
    }
}

/// What a track is: how long it plays and its samples' format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackInfo {
    pub length_ms: u32,
    pub rate: u32,
    pub channels: u16,
}

/// What a track is, read from its bytes: its length is its samples per
/// channel (the encoder's delay and padding cut when its LAME tag gives
/// them, as the decoder plays it) over its rate. `None` when it isn't an
/// MP3 the decoder reads.
pub fn track_info(bytes: &[u8]) -> Option<TrackInfo> {
    let decoder = mp3::Decoder::new(bytes).ok()?;
    let rate = decoder.sample_rate().max(1);
    let samples = match decoder.info().gapless_samples {
        Some(n) => n,
        None => {
            let scan = mp3::scan(bytes).ok()?;
            (scan.frames * decoder.info().samples_per_frame) as u64
        }
    };
    Some(TrackInfo {
        length_ms: (samples as f64 * 1000.0 / f64::from(rate)).round() as u32,
        rate,
        channels: decoder.channels(),
    })
}

/// How long a track plays, in ms ([`track_info`]).
pub fn track_length_ms(bytes: &[u8]) -> Option<u32> {
    track_info(bytes).map(|t| t.length_ms)
}

/// The music files the game can see, each one's [`TrackInfo`] remembered
/// once read.
#[derive(Debug, Clone, Default)]
pub struct MusicLibrary {
    tracks: HashMap<String, Option<TrackInfo>>,
}

impl MusicLibrary {
    /// The files, through the game's file list.
    pub fn files<'a>(&'a mut self, assets: &'a Assets) -> LibraryFiles<'a> {
        LibraryFiles {
            assets,
            library: self,
        }
    }

    /// A track's information (reading the file the first time).
    pub fn track(&mut self, assets: &Assets, path: &str) -> Option<TrackInfo> {
        *self
            .tracks
            .entry(path.to_ascii_lowercase())
            .or_insert_with(|| {
                let bytes = assets.read(path).ok()??;
                track_info(&bytes)
            })
    }
}

/// [`MusicFiles`] over the game's files.
pub struct LibraryFiles<'a> {
    assets: &'a Assets,
    library: &'a mut MusicLibrary,
}

impl MusicFiles for LibraryFiles<'_> {
    fn duration_ms(&mut self, path: &str) -> Option<u32> {
        self.library.track(self.assets, path).map(|t| t.length_ms)
    }

    /// The game lists the folder with Windows (`005917b0`: every file
    /// whose extension is `.mp3`); Windows gives NTFS folders in name
    /// order, taken here as sorted without case (a guess at the exact
    /// collation).
    fn folder(&mut self, folder: &str) -> Vec<String> {
        let folder = format!("{}\\", folder.trim_end_matches('\\').to_ascii_lowercase());
        let mut files: Vec<String> = self
            .assets
            .paths()
            .filter(|p| {
                p.starts_with(&folder) && !p[folder.len()..].contains('\\') && p.ends_with(".mp3")
            })
            .map(str::to_string)
            .collect();
        files.sort();
        files
    }
}

/// Samples decoded from a track, interleaved.
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub rate: u32,
    pub channels: u16,
    pub samples: Vec<f32>,
}

/// Sample frames (per channel) a chunk holds: one MP3 frame's worth.
const CHUNK_FRAMES: usize = 1152;
/// Chunks decoded ahead of playback (about 0.6 s at 48 kHz).
const AHEAD: usize = 24;

/// A track read in order from a position, starting over at its end when
/// it loops (without a gap: the decoder trims the encoder's padding).
pub struct TrackReader {
    decoder: mp3::Decoder<Arc<[u8]>>,
    looping: bool,
    /// Sample frames given since the start of the track.
    at: u64,
    ended: bool,
}

impl TrackReader {
    pub fn new(bytes: Arc<[u8]>, looping: bool) -> Option<TrackReader> {
        Some(TrackReader {
            decoder: mp3::Decoder::new(bytes).ok()?,
            looping,
            at: 0,
            ended: false,
        })
    }

    pub fn rate(&self) -> u32 {
        self.decoder.sample_rate()
    }

    pub fn channels(&self) -> u16 {
        self.decoder.channels()
    }

    /// Where it is in the track, ms.
    pub fn position_ms(&self) -> f64 {
        self.at as f64 * 1000.0 / f64::from(self.rate().max(1))
    }

    pub fn ended(&self) -> bool {
        self.ended
    }

    /// Up to `frames` sample frames, interleaved; fewer (or none) only at
    /// the end of a track that doesn't loop.
    pub fn read(&mut self, frames: usize) -> Vec<f32> {
        let channels = usize::from(self.channels().max(1));
        let mut out = vec![0.0f32; frames * channels];
        let mut filled = 0;
        while filled < out.len() && !self.ended {
            let n = self.decoder.read_f32(&mut out[filled..]);
            filled += n;
            self.at += (n / channels) as u64;
            if filled < out.len() {
                if self.looping && self.at > 0 {
                    self.decoder.rewind();
                    self.at = 0;
                } else {
                    self.ended = true;
                }
            }
        }
        out.truncate(filled);
        out
    }

    /// Runs forward to `ms` into the track (decoding and dropping; past
    /// the end of a looping track it goes round again). False if the
    /// track ended first.
    pub fn skip_to(&mut self, ms: f64) -> bool {
        let rate = f64::from(self.rate().max(1));
        let mut target = (ms.max(0.0) * rate / 1000.0).round() as u64;
        if target < self.at {
            self.decoder.rewind();
            self.at = 0;
        }
        while self.at < target {
            let want = (target - self.at).min(16 * CHUNK_FRAMES as u64) as usize;
            let before = self.at;
            let got = self.read(want).len() / usize::from(self.channels().max(1));
            if self.ended {
                return false;
            }
            if self.at < before {
                // Went round: what's left of the target from the start.
                let length = before + got as u64 - self.at;
                target = target.saturating_sub(length);
            }
            if got == 0 {
                return false;
            }
        }
        true
    }
}

/// A track decoded on a thread of its own, ahead of playback. Dropping it
/// ends the thread.
pub struct TrackStream {
    receiver: Receiver<Chunk>,
}

/// What a stream has now.
#[derive(Debug, Clone, PartialEq)]
pub enum Next {
    Chunk(Chunk),
    /// Nothing decoded yet (the thread is still reading or catching up).
    Waiting,
    /// The track ended (or couldn't be read).
    Ended,
}

impl TrackStream {
    /// Starts a track `start_ms` in, reading its bytes with `load` on the
    /// new thread. The time spent reading and running up to that point is
    /// added, so the first samples are where the deck is by then.
    pub fn start(
        load: impl FnOnce() -> Option<Vec<u8>> + Send + 'static,
        start_ms: f64,
        looping: bool,
    ) -> TrackStream {
        let (sender, receiver) = sync_channel(AHEAD);
        let begun = Instant::now();
        let spawned = std::thread::Builder::new()
            .name("music track".into())
            .spawn(move || {
                let Some(bytes) = load() else { return };
                let Some(mut reader) = TrackReader::new(Arc::from(bytes), looping) else {
                    return;
                };
                // Catch up: each pass runs to where the deck is now.
                let mut target = start_ms;
                for _ in 0..4 {
                    if !reader.skip_to(target) {
                        return;
                    }
                    let now = start_ms + begun.elapsed().as_secs_f64() * 1000.0;
                    if now - target < 5.0 {
                        break;
                    }
                    target = now;
                }
                loop {
                    let samples = reader.read(CHUNK_FRAMES);
                    if samples.is_empty() {
                        return;
                    }
                    let chunk = Chunk {
                        rate: reader.rate(),
                        channels: reader.channels(),
                        samples,
                    };
                    if sender.send(chunk).is_err() {
                        return;
                    }
                }
            });
        if spawned.is_err() {
            let (_, receiver) = sync_channel(1);
            return TrackStream { receiver };
        }
        TrackStream { receiver }
    }

    /// The next chunk, without waiting.
    pub fn poll(&mut self) -> Next {
        match self.receiver.try_recv() {
            Ok(c) => Next::Chunk(c),
            Err(TryRecvError::Empty) => Next::Waiting,
            Err(TryRecvError::Disconnected) => Next::Ended,
        }
    }

    /// The next chunk, waiting for it (for writing a mix to a file).
    pub fn next_blocking(&mut self) -> Option<Chunk> {
        self.receiver.recv().ok()
    }
}

/// One deck's track in a [`Mixdown`].
struct Voice {
    deck: usize,
    generation: u64,
    seeks: u32,
    reader: TrackReader,
    /// Input frames: the one before the read point, and after.
    a: [f32; 2],
    b: [f32; 2],
    /// Where between them, 0..1.
    phase: f64,
    volume: f32,
}

impl Voice {
    fn frame(&mut self) -> Option<[f32; 2]> {
        let s = self.reader.read(1);
        match s.as_slice() {
            [l, r] => Some([*l, *r]),
            [m] => Some([*m, *m]),
            _ => None,
        }
    }
}

/// Mixes the decks' tracks into stereo at one rate, a step at a time,
/// without threads: for writing what the music sounds like to a file. Each
/// deck plays at its volume (ramped across a step from the last), its
/// track started where the deck is, again whenever the deck jumps.
pub struct Mixdown {
    pub rate: u32,
    voices: Vec<Voice>,
}

impl Mixdown {
    pub fn new(rate: u32) -> Mixdown {
        Mixdown {
            rate,
            voices: Vec::new(),
        }
    }

    /// Adds `frames` stereo frames of the decks as they are to `out`;
    /// `open` reads a file.
    pub fn mix(
        &mut self,
        decks: &[Deck; 2],
        frames: usize,
        open: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
        out: &mut Vec<f32>,
    ) {
        // Voices whose deck moved on (a new track, a jump, emptied) go.
        self.voices.retain(|v| {
            let d = &decks[v.deck];
            d.generation == v.generation && d.seeks == v.seeks
        });
        for (i, d) in decks.iter().enumerate() {
            if !d.is_live() || self.voices.iter().any(|v| v.deck == i) {
                continue;
            }
            let Some(reader) = d
                .path
                .as_deref()
                .and_then(&mut *open)
                .and_then(|b| TrackReader::new(Arc::from(b), d.looping()))
            else {
                continue;
            };
            let mut v = Voice {
                deck: i,
                generation: d.generation,
                seeks: d.seeks,
                reader,
                a: [0.0; 2],
                b: [0.0; 2],
                phase: 0.0,
                volume: d.volume,
            };
            v.reader.skip_to(d.position_ms);
            v.a = v.frame().unwrap_or([0.0; 2]);
            v.b = v.frame().unwrap_or([0.0; 2]);
            self.voices.push(v);
        }
        let start = out.len();
        out.resize(start + frames * 2, 0.0);
        for v in &mut self.voices {
            let d = &decks[v.deck];
            let step = f64::from(v.reader.rate()) / f64::from(self.rate);
            let (from, to) = (v.volume, if d.running() { d.volume } else { 0.0 });
            for f in 0..frames {
                let g = from + (to - from) * (f as f32 + 1.0) / frames as f32;
                let t = v.phase as f32;
                for c in 0..2 {
                    out[start + 2 * f + c] += g * (v.a[c] + (v.b[c] - v.a[c]) * t);
                }
                if d.running() {
                    v.phase += step;
                    while v.phase >= 1.0 {
                        v.phase -= 1.0;
                        v.a = v.b;
                        v.b = v.frame().unwrap_or([0.0; 2]);
                    }
                }
            }
            v.volume = to;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_silent_track_has_its_frames_length_and_loops_on_reading() {
        // 100 silent frames at 44.1 kHz: 115,200 samples, 2612 ms.
        let bytes = testdata::music::silent_mp3(100);
        assert_eq!(track_length_ms(&bytes), Some(2612));
        let mut r = TrackReader::new(Arc::from(bytes.clone()), true).unwrap();
        assert!(r.skip_to(2000.0));
        assert!((r.position_ms() - 2000.0).abs() < 0.1);
        // Past the end it goes round.
        assert!(r.skip_to(3000.0));
        assert!((r.position_ms() - (3000.0 - 115_200.0 / 44.1)).abs() < 0.1);
        let mut once = TrackReader::new(Arc::from(bytes), false).unwrap();
        assert!(!once.skip_to(3000.0));
        assert!(once.ended());
    }

    #[test]
    fn the_library_lists_a_folder_by_name_and_reads_lengths() {
        let data = testdata::music::hall("library");
        let assets = Assets::open_with(data.path(), &["FalloutNV.esm".to_string()], &[]).unwrap();
        let mut library = MusicLibrary::default();
        let mut files = library.files(&assets);
        assert_eq!(
            files.folder("music\\explore\\"),
            vec![
                "music\\explore\\a_first.mp3".to_string(),
                "music\\explore\\b_second.mp3".to_string()
            ]
        );
        assert_eq!(
            files.duration_ms("music\\loc\\test\\day_3high.mp3"),
            Some(testdata::music::TRACK_MS)
        );
        assert_eq!(files.duration_ms("music\\loc\\test\\missing.mp3"), None);
    }

    #[test]
    fn a_stream_starts_where_it_is_told_plus_the_time_it_took() {
        let bytes = testdata::music::silent_mp3(40);
        let begun = Instant::now();
        let mut s = TrackStream::start(move || Some(bytes), 500.0, false);
        let mut frames = 0;
        let mut waited = None;
        while let Some(c) = s.next_blocking() {
            waited.get_or_insert_with(|| begun.elapsed().as_secs_f64());
            assert_eq!((c.rate, c.channels), (44_100, 2));
            frames += c.samples.len() / 2;
        }
        // 40 × 1152 frames in all; 500 ms (22,050 frames) skipped, and as
        // much again as starting took (at most until the first samples
        // came).
        let skipped = 40 * 1152 - frames;
        let most = 22_050.0 + waited.unwrap() * 44_100.0 + 1.0;
        assert!(skipped >= 22_050 && skipped as f64 <= most, "{skipped}");
        assert_eq!(s.poll(), Next::Ended);
    }
}
