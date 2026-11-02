//! The game's sounds (`world::sound`): doors opening, sounds scripts play
//! (`PlaySound`), and the loop the place plays (its acoustic space's, by
//! the hour). WAV files are decoded here (`cellview::sound`), OGG files by
//! Bevy. Sounds aren't placed in the world yet (no distance falloff or
//! direction), and music (MP3 files) isn't played.

use std::sync::Arc;
use std::time::Duration;

use bevy::audio::{AudioPlayer, Decodable, PlaybackSettings, Source};
use bevy::prelude::*;
use esm::FormId;
use world::sound::{ambient_loop, Sound};

use crate::dialogue::DialogueState;
use crate::GameFiles;

/// Decoded samples (from a WAV file).
#[derive(Asset, TypePath, Clone)]
pub struct PcmSound {
    channels: u16,
    rate: u32,
    samples: Arc<[i16]>,
}

impl PcmSound {
    /// Interleaved 16-bit samples at this rate.
    pub fn new(channels: u16, rate: u32, samples: Vec<i16>) -> Self {
        PcmSound {
            channels: channels.max(1),
            rate: rate.max(1),
            samples: samples.into(),
        }
    }
}

pub struct PcmDecoder {
    sound: PcmSound,
    at: usize,
}

impl Iterator for PcmDecoder {
    type Item = i16;
    fn next(&mut self) -> Option<i16> {
        let s = self.sound.samples.get(self.at).copied();
        self.at += 1;
        s
    }
}

impl Source for PcmDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        self.sound.channels
    }
    fn sample_rate(&self) -> u32 {
        self.sound.rate
    }
    fn total_duration(&self) -> Option<Duration> {
        let frames = self.sound.samples.len() as f64 / f64::from(self.sound.channels.max(1));
        Some(Duration::from_secs_f64(
            frames / f64::from(self.sound.rate.max(1)),
        ))
    }
}

impl Decodable for PcmSound {
    type DecoderItem = i16;
    type Decoder = PcmDecoder;
    fn decoder(&self) -> PcmDecoder {
        PcmDecoder {
            sound: self.clone(),
            at: 0,
        }
    }
}

/// A decoded WAV file as a sound to play.
pub(crate) fn pcm_sound(pcm: cellview::sound::Pcm) -> PcmSound {
    PcmSound {
        channels: pcm.channels,
        rate: pcm.rate,
        samples: Arc::from(pcm.samples.into_boxed_slice()),
    }
}

/// Sounds to play once (sound records), queued by doors and scripts.
#[derive(Resource, Default)]
pub struct SoundRequests(pub Vec<FormId>);

/// The place's loop playing now: for which (cell, sound), and its entity.
#[derive(Resource, Default)]
pub struct Ambient {
    key: Option<(FormId, FormId)>,
    entity: Option<Entity>,
}

/// Starts a sound record's file playing; the entity, or `None` when it
/// can't be found or read.
pub(crate) fn play(
    commands: &mut Commands,
    game: &cellview::Game,
    wavs: &mut Assets<PcmSound>,
    sound: &Sound,
    pick: u64,
    looping: bool,
) -> Option<Entity> {
    let settings = if looping {
        PlaybackSettings::LOOP
    } else {
        PlaybackSettings::DESPAWN
    };
    play_with(commands, game, wavs, sound, pick, settings)
}

/// [`play`] with these playback settings (a volume of its own).
pub(crate) fn play_with(
    commands: &mut Commands,
    game: &cellview::Game,
    wavs: &mut Assets<PcmSound>,
    sound: &Sound,
    pick: u64,
    settings: PlaybackSettings,
) -> Option<Entity> {
    let handle = wavs.add(decoded(game, sound, pick)?);
    Some(commands.spawn((AudioPlayer(handle), settings)).id())
}

/// The samples of the file a sound record plays ([`cellview::Game::
/// sound_path`]), read and decoded once and kept: doing it again for every
/// gunshot or footstep cost milliseconds each time. Up to
/// [`DECODED_SAMPLES`] samples are kept in all, the least recently played
/// file going first.
fn decoded(game: &cellview::Game, sound: &Sound, pick: u64) -> Option<PcmSound> {
    static CACHE: std::sync::OnceLock<std::sync::Mutex<DecodedSounds>> = std::sync::OnceLock::new();
    let cache = CACHE.get_or_init(Default::default);
    let path = game.sound_path(sound, pick)?;
    if let Some(sound) = cache.lock().ok().and_then(|mut c| c.get(&path)) {
        return Some(sound);
    }
    let bytes = game.assets.read(&path).ok()??;
    let pcm = read_sound(&path, &bytes)
        .map_err(|e| println!("  couldn't play {path}: {e}"))
        .ok()?;
    let sound = pcm_sound(pcm);
    if let Ok(mut c) = cache.lock() {
        c.insert(path, sound.clone());
    }
    Some(sound)
}

/// Samples kept by [`decoded`] (64 MB of 16-bit samples).
const DECODED_SAMPLES: usize = 32 << 20;

/// Decoded sound files by path, each with when it was last played.
#[derive(Default)]
struct DecodedSounds {
    files: std::collections::HashMap<String, (PcmSound, u64)>,
    samples: usize,
    clock: u64,
}

impl DecodedSounds {
    fn get(&mut self, path: &str) -> Option<PcmSound> {
        self.clock += 1;
        let clock = self.clock;
        self.files.get_mut(path).map(|(sound, used)| {
            *used = clock;
            sound.clone()
        })
    }

    fn insert(&mut self, path: String, sound: PcmSound) {
        self.clock += 1;
        self.samples += sound.samples.len();
        if let Some((old, _)) = self.files.insert(path, (sound, self.clock)) {
            self.samples -= old.samples.len();
        }
        while self.samples > DECODED_SAMPLES && self.files.len() > 1 {
            let Some(oldest) = self
                .files
                .iter()
                .min_by_key(|(_, (_, used))| *used)
                .map(|(p, _)| p.clone())
            else {
                break;
            };
            if let Some((gone, _)) = self.files.remove(&oldest) {
                self.samples -= gone.samples.len();
            }
        }
    }
}

/// A voice line's file ready to play: decoded here like every sound (see
/// [`read_sound`]), `None` when it can't be read.
pub fn voice_handle(
    path: &str,
    bytes: &[u8],
    wavs: &mut Assets<PcmSound>,
) -> Option<Handle<PcmSound>> {
    let pcm = read_sound(path, bytes)
        .map_err(|e| println!("  couldn't play {path}: {e}"))
        .ok()?;
    Some(wavs.add(PcmSound {
        channels: pcm.channels,
        rate: pcm.rate,
        samples: Arc::from(pcm.samples.into_boxed_slice()),
    }))
}

/// A sound file's samples: a WAV, or an Ogg Vorbis file (`.ogg`) decoded
/// here. Bevy's own `.ogg` playback crashed the viewer (a native fault
/// while playing Dead Money's Villa music; decoding the same files alone is
/// fine), so every sound plays through the viewer's own sample sources.
pub fn read_sound(path: &str, bytes: &[u8]) -> Result<cellview::sound::Pcm, String> {
    if path.to_ascii_lowercase().ends_with(".ogg") {
        decode_ogg(bytes)
    } else {
        cellview::sound::read_wav(bytes)
    }
}

/// An Ogg Vorbis file's samples, interleaved.
pub fn decode_ogg(bytes: &[u8]) -> Result<cellview::sound::Pcm, String> {
    let mut reader = lewton::inside_ogg::OggStreamReader::new(std::io::Cursor::new(bytes))
        .map_err(|e| format!("not an Ogg Vorbis file: {e}"))?;
    let channels = u16::from(reader.ident_hdr.audio_channels);
    let rate = reader.ident_hdr.audio_sample_rate;
    let mut samples = Vec::new();
    while let Some(packet) = reader
        .read_dec_packet_itl()
        .map_err(|e| format!("damaged Ogg Vorbis data: {e}"))?
    {
        samples.extend(packet);
    }
    Ok(cellview::sound::Pcm {
        channels,
        rate,
        samples,
    })
}

/// Plays queued sounds, and keeps the place's loop going.
#[allow(clippy::too_many_arguments)]
pub fn play_sounds(
    mut commands: Commands,
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    mut requests: ResMut<SoundRequests>,
    mut ambient: ResMut<Ambient>,
    mut wavs: ResMut<Assets<PcmSound>>,
) {
    let order = &game.0.order;
    let state = &state.0;
    for (i, id) in std::mem::take(&mut requests.0).into_iter().enumerate() {
        if let Some(sound) = Sound::load(order, id) {
            let pick = state.dice.wrapping_add(i as u64);
            play(&mut commands, &game.0, &mut wavs, &sound, pick, false);
        }
    }
    // The place's loop: an interior's acoustic space, by the hour.
    let hour = state.global(order, "GameHour").unwrap_or(10.0);
    let wanted = match (state.player_world, state.player_cell) {
        (None, Some(cell)) => ambient_loop(order, cell, hour).map(|s| (cell, s)),
        _ => None,
    };
    if wanted == ambient.key {
        return;
    }
    if let Some(e) = ambient.entity.take() {
        if let Ok(mut entity) = commands.get_entity(e) {
            entity.despawn();
        }
    }
    ambient.key = wanted;
    if let Some((_, id)) = wanted {
        ambient.entity = Sound::load(order, id).and_then(|sound| {
            let entity = play(&mut commands, &game.0, &mut wavs, &sound, state.dice, true);
            match entity {
                Some(_) => println!("The place's sound: {}", sound.file),
                None => println!("The place's sound ({}) wasn't found.", sound.file),
            }
            entity
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sound(n: usize) -> PcmSound {
        PcmSound::new(1, 22050, vec![0; n])
    }

    /// Kept files come back as they were decoded; over the limit the least
    /// recently played go first, the newest always stays.
    #[test]
    fn decoded_sounds_keep_the_recently_played() {
        let mut c = DecodedSounds::default();
        let third = DECODED_SAMPLES / 3 + 1;
        c.insert("a".into(), sound(third));
        c.insert("b".into(), sound(third));
        assert_eq!(c.get("a").map(|s| s.samples.len()), Some(third));
        // "b" is now the least recently played: the third file pushes it out.
        c.insert("c".into(), sound(third));
        assert!(c.get("b").is_none());
        assert!(c.get("a").is_some() && c.get("c").is_some());
        assert_eq!(c.samples, 2 * third);
        // One file bigger than the limit still stays (the newest).
        c.insert("big".into(), sound(DECODED_SAMPLES + 1));
        assert!(c.get("big").is_some());
        assert_eq!(c.files.len(), 1);
        assert_eq!(c.samples, DECODED_SAMPLES + 1);
    }
}
