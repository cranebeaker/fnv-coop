//! The game's music (`world::music`): each frame the music manager runs
//! on the game's state (where the player is, the hour and climate, who's
//! aware of the player, whether battle music is wanted, `PlayMusic`), and
//! its two decks play through Bevy's audio: each track decoded on a thread
//! of its own ahead of playback (`cellview::music::TrackStream`), started
//! where its deck is (a new layer where the old one was), at the deck's
//! volume (master × music volume × the track's decibels, faded as the
//! deck fades). The sets' intro, outro and incidental sounds play once at
//! the music's volume.
//!
//! Guesses here: those sounds' loudness (master × music volume; the
//! sound records' own attenuation isn't applied), the audio clock (the
//! viewer's real time), and a held deck (one frame after a battle intro)
//! playing on rather than pausing.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy::audio::{AddAudioSource, AudioPlayer, Decodable, PlaybackSettings, Source};
use bevy::prelude::*;
use cellview::music::{MusicLibrary, Next, TrackStream};
use esm::FormId;
use world::dialogue::PLAYER_REF;
use world::music::{
    aware_actors, player_strength, CombatMusic, CombatMusicSettings, MusicDirector, MusicEvent,
    MusicInputs, PlayerPlace,
};
use world::sound::Sound;
use world::weather::{Climate, DEFAULT_CLIMATE};

use crate::dialogue::{DialogueState, Talkers};
use crate::GameFiles;

/// Registers the music.
pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Music>()
            .init_resource::<MusicRequests>()
            .add_audio_source::<MusicTrack>()
            // The game's music manager runs in its main loop, which a
            // movie holds until it ends (`movie`).
            .add_systems(
                Update,
                // Post-swap audio, after the radio (0086f640).
                play_music.run_if(not(crate::movie::playing)).in_set(
                    crate::frame_order::InterfaceSet::at(
                        world::frame::interface::AUDIO_UPDATE,
                        0x0086_f65f,
                    ),
                ),
            );
    }
}

/// Music types scripts asked for (`PlayMusic`), oldest first.
#[derive(Resource, Default)]
pub struct MusicRequests(pub Vec<FormId>);

/// What one deck plays through Bevy.
struct Playing {
    generation: u64,
    seeks: u32,
    entity: Entity,
    gain: Arc<AtomicU32>,
}

/// The music manager and the sounds playing its decks.
#[derive(Resource, Default)]
pub struct Music {
    director: Option<MusicDirector>,
    library: MusicLibrary,
    combat: CombatMusic,
    combat_settings: Option<CombatMusicSettings>,
    climate: Option<(FormId, Option<Climate>)>,
    decks: [Option<Playing>; 2],
}

/// Changes the decks' volumes (the start menu's Audio page,
/// `StartMenu::SetMasterVol` … (Xbox PDB)); before the manager has started,
/// it starts from the INI's and then this applies next time.
pub fn set_volumes(music: &mut Music, change: impl FnOnce(&mut world::music::Volumes)) {
    if let Some(d) = music.director.as_mut() {
        change(&mut d.decks.volumes);
    }
}

/// The music's clock, ms (the audio manager's: from 1 s, never 0), which
/// the radio's times use too.
pub fn audio_clock(real: &Time<Real>) -> u64 {
    real.elapsed().as_millis() as u64 + 1000
}

/// The radio holds the music manager (`008325a0`) or lets it go.
pub fn radio_hold(music: &mut Music, on: bool) {
    if let Some(d) = music.director.as_mut() {
        d.suspended = on;
    }
}

/// Both decks cleared at once (`008304a0`).
pub fn radio_clear(music: &mut Music) {
    if let Some(d) = music.director.as_mut() {
        d.clear_decks();
    }
}

/// A song's length (relative to `Data`), as the decks read it.
pub fn song_ms(music: &mut Music, game: &cellview::Game, path: &str) -> Option<u32> {
    use world::music::MusicFiles;
    music.library.files(&game.assets).duration_ms(path)
}

/// The radio's song onto a deck as type 7, in step with `sync`.
pub fn radio_song(music: &mut Music, game: &cellview::Game, path: &str, sync: u64, now: u64) {
    let Music {
        director, library, ..
    } = music;
    if let Some(d) = director.as_mut() {
        let mut files = library.files(&game.assets);
        d.radio_song(path, sync, now, &mut files);
    }
}

/// Where a track's samples come from.
enum Feed {
    /// An MP3, decoded on its own thread.
    Stream(TrackStream),
    /// A sound already decoded (a WAV).
    Clip(Vec<f32>),
}

/// A track for Bevy's audio: its samples and the loudness to play them at
/// (set each frame from its deck).
#[derive(Asset, TypePath, Clone)]
pub struct MusicTrack {
    feed: Arc<Mutex<Option<Feed>>>,
    gain: Arc<AtomicU32>,
    rate: u32,
    channels: u16,
}

impl MusicTrack {
    fn new(feed: Feed, rate: u32, channels: u16, gain: Arc<AtomicU32>) -> MusicTrack {
        MusicTrack {
            feed: Arc::new(Mutex::new(Some(feed))),
            gain,
            rate: rate.max(1),
            channels: channels.max(1),
        }
    }
}

/// Plays a [`MusicTrack`]: its samples × the gain, which moves to a new
/// value over about 20 ms (a deck's volume changes once a frame). While
/// the decoding thread hasn't caught up, silence.
pub struct TrackDecoder {
    feed: Option<Feed>,
    chunk: Vec<f32>,
    at: usize,
    rate: u32,
    channels: u16,
    gain: Arc<AtomicU32>,
    applied: f32,
    ended: bool,
}

impl TrackDecoder {
    /// The next samples: a decoded chunk, 10 ms of silence while waiting
    /// for one, a clip's samples (once); nothing at the end.
    fn refill(&mut self) {
        self.at = 0;
        self.chunk = match &mut self.feed {
            Some(Feed::Stream(s)) => match s.poll() {
                Next::Chunk(c) => c.samples,
                Next::Waiting => {
                    vec![0.0; (self.rate as usize / 100).max(1) * usize::from(self.channels)]
                }
                Next::Ended => Vec::new(),
            },
            Some(Feed::Clip(samples)) => std::mem::take(samples),
            None => Vec::new(),
        };
        if self.chunk.is_empty() {
            self.ended = true;
        }
    }
}

impl Iterator for TrackDecoder {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.ended {
            return None;
        }
        let s = self.chunk[self.at];
        self.at += 1;
        let target = f32::from_bits(self.gain.load(Ordering::Relaxed));
        let step = 50.0 / (self.rate as f32 * f32::from(self.channels));
        self.applied += (target - self.applied).clamp(-step, step);
        if self.at >= self.chunk.len() {
            self.refill();
        }
        Some(s * self.applied)
    }
}

impl Source for TrackDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        self.channels
    }
    fn sample_rate(&self) -> u32 {
        self.rate
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Decodable for MusicTrack {
    type DecoderItem = f32;
    type Decoder = TrackDecoder;
    fn decoder(&self) -> TrackDecoder {
        let feed = self.feed.lock().ok().and_then(|mut f| f.take());
        let mut d = TrackDecoder {
            feed,
            chunk: Vec::new(),
            at: 0,
            rate: self.rate,
            channels: self.channels,
            applied: f32::from_bits(self.gain.load(Ordering::Relaxed)),
            gain: self.gain.clone(),
            ended: false,
        };
        d.refill();
        d
    }
}

/// Plays a sound record once at a loudness (the sets' intros, outros and
/// phrases: OGG files, or WAV).
fn play_sound(
    commands: &mut Commands,
    game: &cellview::Game,
    tracks: &mut Assets<MusicTrack>,
    id: FormId,
    pick: u64,
    loudness: f32,
) {
    let order = &game.order;
    let name = order
        .get(id)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.editor_id())
        .unwrap_or_else(|| id.to_string());
    let Some((path, bytes)) = Sound::load(order, id).and_then(|s| game.sound_file(&s, pick)) else {
        println!("Music: the sound {name} wasn't found.");
        return;
    };
    println!("Music: the sound {name} ({path}).");
    match crate::sounds::read_sound(&path, &bytes) {
        Ok(pcm) => {
            let samples = pcm
                .samples
                .iter()
                .map(|&s| f32::from(s) / 32768.0)
                .collect();
            let gain = Arc::new(AtomicU32::new(loudness.to_bits()));
            let handle = tracks.add(MusicTrack::new(
                Feed::Clip(samples),
                pcm.rate,
                pcm.channels,
                gain,
            ));
            commands.spawn((AudioPlayer(handle), PlaybackSettings::DESPAWN));
        }
        Err(e) => println!("Music: couldn't play {path}: {e}"),
    }
}

/// Runs the music manager for this frame and keeps the decks playing.
#[allow(clippy::too_many_arguments)]
pub fn play_music(
    mut commands: Commands,
    game: Res<GameFiles>,
    real: Res<Time<Real>>,
    time: Res<Time>,
    state: Res<DialogueState>,
    talkers: Res<Talkers>,
    mut music: ResMut<Music>,
    mut requests: ResMut<MusicRequests>,
    mut tracks: ResMut<Assets<MusicTrack>>,
) {
    let shared = Arc::clone(&game.0);
    let game: &cellview::Game = &shared;
    let order = &game.order;
    let state = &state.0;
    let music = &mut *music;
    let director = music.director.get_or_insert_with(|| {
        let volumes = cellview::music::volumes(&game.settings);
        println!(
            "Music: master volume {}, music volume {} (INI).",
            volumes.master, volumes.music
        );
        MusicDirector::new(volumes, state.dice)
    });
    let combat_settings = *music
        .combat_settings
        .get_or_insert_with(|| CombatMusicSettings::load(order));
    // The audio clock in ms (from 1 s: the game's is never 0).
    let now = real.elapsed().as_millis() as u64 + 1000;
    let hour = state
        .global(order, "GameHour")
        .unwrap_or(world::weather::DEFAULT_HOUR);
    let climate_id = state.weather.climate.unwrap_or(DEFAULT_CLIMATE);
    if music.climate.as_ref().map(|(id, _)| *id) != Some(climate_id) {
        music.climate = Some((climate_id, Climate::load(order, climate_id)));
    }
    let climate = music.climate.as_ref().and_then(|(_, c)| c.as_ref());
    let player = state.player_position.map(|position| PlayerPlace {
        cell: state.player_cell,
        world: state.player_world,
        position,
    });
    let people: Vec<(FormId, [f32; 3])> = talkers
        .0
        .iter()
        .map(|t| (t.reference, t.position))
        .collect();
    let outdoors = state.player_world.is_some();
    let (combat, aware) = match player {
        Some(p) => {
            let t = time.elapsed_secs();
            let groups = music
                .combat
                .groups(order, state, p.position, outdoors, &people, t);
            let on =
                music
                    .combat
                    .update(t, &combat_settings, player_strength(order, state), &groups);
            (
                on,
                aware_actors(order, state, p.position, outdoors, &people),
            )
        }
        None => (false, Vec::new()),
    };
    let inputs = MusicInputs {
        now_ms: now,
        hour,
        climate,
        player,
        combat,
        aware: &aware,
        dead: state.dead.contains(&PLAYER_REF),
    };
    let mut files = music.library.files(&game.assets);
    let played = std::mem::take(&mut requests.0);
    for m in &played {
        director.play_music(order, *m, &inputs, &mut files);
    }
    if played.is_empty() {
        director.update(order, &inputs, &mut files);
    }
    director.tick(now);

    let volumes = director.decks.volumes;
    let loudness = volumes.master * volumes.music;
    for (i, event) in director.take_events().into_iter().enumerate() {
        match event {
            MusicEvent::Note(n) => println!("{n}"),
            MusicEvent::Sound(s) => play_sound(
                &mut commands,
                game,
                &mut tracks,
                s,
                state.dice.wrapping_add(i as u64),
                loudness,
            ),
        }
    }

    // Each deck's track: started (again) when the deck has a new one or
    // jumped, stopped when the deck empties, at the deck's volume.
    let decks = director.decks.decks.clone();
    for (i, deck) in decks.iter().enumerate() {
        let want = deck.is_live().then_some((deck.generation, deck.seeks));
        let have = music.decks[i].as_ref().map(|p| (p.generation, p.seeks));
        if want != have {
            if let Some(p) = music.decks[i].take() {
                if let Ok(mut e) = commands.get_entity(p.entity) {
                    e.despawn();
                }
            }
            let Some(path) = deck.path.clone().filter(|_| deck.is_live()) else {
                continue;
            };
            let Some(info) = music.library.track(&game.assets, &path) else {
                continue;
            };
            let reader = Arc::clone(&shared);
            let file = path.clone();
            let stream = TrackStream::start(
                move || reader.assets.read(&file).ok().flatten(),
                deck.position_ms,
                deck.looping(),
            );
            let gain = Arc::new(AtomicU32::new(deck.volume.to_bits()));
            let handle = tracks.add(MusicTrack::new(
                Feed::Stream(stream),
                info.rate,
                info.channels,
                gain.clone(),
            ));
            let entity = commands
                .spawn((AudioPlayer(handle), PlaybackSettings::ONCE))
                .id();
            music.decks[i] = Some(Playing {
                generation: deck.generation,
                seeks: deck.seeks,
                entity,
                gain,
            });
        }
        if let Some(p) = &music.decks[i] {
            p.gain.store(deck.volume.to_bits(), Ordering::Relaxed);
        }
    }
}
