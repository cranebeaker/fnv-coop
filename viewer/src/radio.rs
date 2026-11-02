//! The radio (`world::radio`) in the viewer: every frame the world's radio
//! runs (`FalloutRadio::Update`, called from the main loop `0086f640`
//! after the audio manager and before the music manager), and what it asks
//! for is done here: the found / lost sounds and HUD messages, the DJ's
//! voice files, the songs on the music decks (type 7), the static loop,
//! the line's result scripts. DATA › Radio's rows click through
//! `pipboy` into [`click`]. The Pip-Boy being put away doesn't touch it.
//! Script functions (`PipboyRadio`, `StartRadioConversation`,
//! `SetNPCRadio` …) act on the same radio; a person playing a station
//! (a receiver) plays its lines here too, not placed in the world (as the
//! viewer's other voices: no distance falloff or direction).

use std::collections::HashMap;

use bevy::audio::{AudioPlayer, AudioSink, AudioSinkPlayback, PlaybackSettings, Volume};
use bevy::prelude::*;
use bevy::time::Real;
use esm::FormId;
use world::radio::{RadioEvent, RadioFiles};

use crate::dialogue::DialogueState;
use crate::sounds::PcmSound;
use crate::GameFiles;

pub struct RadioPlugin;

impl Plugin for RadioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RadioOut>()
            // Post-swap audio: after sound updates, before music (0086f640).
            .add_systems(
                Update,
                run_radio.in_set(crate::frame_order::InterfaceSet::at(
                    world::frame::interface::AUDIO_UPDATE,
                    0x0086_f657,
                )),
            );
    }
}

/// What the radio plays now: the DJ's voice and the static loop.
#[derive(Resource, Default)]
pub struct RadioOut {
    voice: Option<Entity>,
    static_loop: Option<(Entity, f32)>,
    /// Events from a click, done next frame.
    pending: Vec<RadioEvent>,
    /// What each receiver plays now, and its static loop.
    receivers: HashMap<FormId, Entity>,
    receiver_static: HashMap<FormId, (Entity, f32)>,
}

fn despawn(commands: &mut Commands, e: Option<Entity>) {
    if let Some(e) = e {
        if let Ok(mut c) = commands.get_entity(e) {
            c.despawn();
        }
    }
}

/// A DATA › Radio row clicked (`00796fd0` case 0x19).
pub fn click(
    state: &mut world::scripting::GameState,
    out: &mut RadioOut,
    reference: u32,
    now: u64,
) {
    let events = state.radio.click_row(esm::FormId(reference), now);
    out.pending.extend(events);
}

/// The files the radio asks about.
struct Files<'a> {
    game: &'a cellview::Game,
    music: &'a mut crate::music::Music,
}

impl RadioFiles for Files<'_> {
    fn voice_ms(&mut self, path: &str) -> Option<u32> {
        let bytes = self.game.assets.read(path).ok().flatten()?;
        crate::pipboy::audio_ms(path, &bytes).map(|ms| ms.round() as u32)
    }

    fn song_ms(&mut self, path: &str) -> Option<u32> {
        crate::music::song_ms(self.music, self.game, path)
    }
}

/// `iRadioUpdateInterval` (the INI's, else the exe's 75).
fn interval(game: &cellview::Game) -> u64 {
    game.settings
        .get("Audio", "iRadioUpdateInterval")
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(world::radio::UPDATE_INTERVAL_MS)
}

/// The radio's frame.
#[allow(clippy::too_many_arguments)]
pub fn run_radio(
    mut commands: Commands,
    game: Res<GameFiles>,
    scripts: Res<crate::scripts::Scripts>,
    mut state: ResMut<DialogueState>,
    real: Res<Time<Real>>,
    mut music: ResMut<crate::music::Music>,
    mut out: ResMut<RadioOut>,
    mut sounds: ResMut<crate::sounds::SoundRequests>,
    mut messages: ResMut<crate::hud::HudMessages>,
    mut wavs: ResMut<Assets<PcmSound>>,
    mut sinks: Query<&mut AudioSink>,
    settings: Option<Res<crate::game_menus::start::GameSettings>>,
) {
    let game = &game.0;
    let order = &game.order;
    let now = crate::music::audio_clock(&real);
    if state.0.player_cell.is_none() {
        return;
    }
    let radio_volume = settings
        .as_ref()
        .and_then(|s| {
            s.values
                .get(&ui::menus::start::Setting::RadioVolume)
                .copied()
        })
        .unwrap_or_else(|| cellview::music::volumes(&game.settings).radio);
    let master = cellview::music::volumes(&game.settings).master;
    let place = world::radio::place_of(&state.0);
    let mut events = std::mem::take(&mut out.pending);
    {
        let state = &mut state.0;
        let mut radio = std::mem::take(&mut state.radio);
        // `fCreatureRadioMax:Audio` (the INI's, else the exe's).
        radio.creature_radio_max.get_or_insert_with(|| {
            game.settings
                .get("Audio", "fCreatureRadioMax")
                .and_then(|v| v.trim().parse::<f32>().ok())
                .unwrap_or(world::radio::CREATURE_RADIO_MAX)
        });
        let mut files = Files {
            game,
            music: &mut music,
        };
        events.extend(radio.update(
            order,
            &scripts.0,
            state,
            &place,
            now,
            interval(game),
            radio_volume,
            &mut files,
        ));
        state.radio = radio;
    }
    for e in events {
        match e {
            RadioEvent::Sound(name) => {
                if let Some(id) = order.form_by_editor_id(name) {
                    sounds.0.push(id);
                }
            }
            RadioEvent::Message(text) => {
                println!("Radio: {text}");
                messages.queue.push(crate::hud::HudMessage::with_icon(
                    text,
                    Some(world::radio::TOWER_ICON),
                ));
            }
            RadioEvent::ClearDecks => crate::music::radio_clear(&mut music),
            RadioEvent::HoldMusic(on) => crate::music::radio_hold(&mut music, on),
            RadioEvent::Song { path, sync } => {
                println!("Radio: song {path}.");
                crate::music::radio_song(&mut music, game, &path, sync, now);
            }
            RadioEvent::Voice { path, volume } => {
                if let Some(e) = out.voice.take() {
                    if let Ok(mut c) = commands.get_entity(e) {
                        c.despawn();
                    }
                }
                let bytes = game.assets.read(&path).ok().flatten();
                if let Some(handle) =
                    bytes.and_then(|b| crate::sounds::voice_handle(&path, &b, &mut wavs))
                {
                    println!("Radio: {path}.");
                    let settings =
                        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume * master));
                    out.voice = Some(commands.spawn((AudioPlayer(handle), settings)).id());
                }
            }
            RadioEvent::StopVoice => {
                if let Some(e) = out.voice.take() {
                    if let Ok(mut c) = commands.get_entity(e) {
                        c.despawn();
                    }
                }
            }
            RadioEvent::Static(Some(v)) => match out.static_loop {
                Some((e, _)) => out.static_loop = Some((e, v)),
                None => {
                    let sound = order
                        .form_by_editor_id(world::radio::STATIC_LOOP)
                        .and_then(|id| world::sound::Sound::load(order, id));
                    if let Some(sound) = sound {
                        if let Some(e) =
                            crate::sounds::play(&mut commands, game, &mut wavs, &sound, 0, true)
                        {
                            commands.entity(e).insert(
                                PlaybackSettings::LOOP.with_volume(Volume::Linear(v * master)),
                            );
                            out.static_loop = Some((e, v));
                        }
                    }
                }
            },
            RadioEvent::Static(None) => {
                if let Some((e, _)) = out.static_loop.take() {
                    if let Ok(mut c) = commands.get_entity(e) {
                        c.despawn();
                    }
                }
            }
            RadioEvent::Script { source, speaker } => {
                let flow = world::scripting::Runner::new(order, &scripts.0, &mut state.0)
                    .run_source(&source, Some(speaker), Some(speaker));
                if flow == world::scripting::Flow::Stopped {
                    println!("  (a radio line's result script stopped: it needs a function not carried out yet)");
                }
            }
            RadioEvent::Changed => {}
            RadioEvent::Receiver {
                reference,
                path,
                song: _,
                volume,
                offset,
            } => {
                despawn(&mut commands, out.receivers.remove(&reference));
                // The file (a song's mono OGG or a voice file) decoded and
                // started `offset` ms in, in step with the station.
                let playing = game
                    .assets
                    .read(&path)
                    .ok()
                    .flatten()
                    .and_then(|b| crate::sounds::read_sound(&path, &b).ok())
                    .map(|mut pcm| {
                        let skip = (offset as f64 / 1000.0
                            * f64::from(pcm.rate)
                            * f64::from(pcm.channels)) as usize;
                        let skip = skip - skip % usize::from(pcm.channels.max(1));
                        pcm.samples.drain(..skip.min(pcm.samples.len()));
                        let handle = wavs.add(crate::sounds::pcm_sound(pcm));
                        let settings =
                            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume * master));
                        commands.spawn((AudioPlayer(handle), settings)).id()
                    });
                match playing {
                    Some(e) => {
                        println!("Radio: {reference} plays {path}.");
                        out.receivers.insert(reference, e);
                    }
                    None => println!("Radio: {reference} can't play {path} (not found)."),
                }
            }
            RadioEvent::ReceiverStop(reference) => {
                despawn(&mut commands, out.receivers.remove(&reference));
            }
            RadioEvent::ReceiverStatic {
                reference,
                volume: Some(v),
            } => match out.receiver_static.get_mut(&reference) {
                Some((_, have)) => *have = v,
                None => {
                    let sound = order
                        .form_by_editor_id(world::radio::STATIC_LOOP)
                        .and_then(|id| world::sound::Sound::load(order, id));
                    if let Some(e) = sound.and_then(|s| {
                        crate::sounds::play(&mut commands, game, &mut wavs, &s, 0, true)
                    }) {
                        commands
                            .entity(e)
                            .insert(PlaybackSettings::LOOP.with_volume(Volume::Linear(v * master)));
                        out.receiver_static.insert(reference, (e, v));
                    }
                }
            },
            RadioEvent::ReceiverStatic {
                reference,
                volume: None,
            } => {
                despawn(
                    &mut commands,
                    out.receiver_static.remove(&reference).map(|(e, _)| e),
                );
            }
        }
    }
    // The static loops' volumes follow the strength.
    let loops = out.static_loop.iter().chain(out.receiver_static.values());
    for &(e, v) in loops {
        if let Ok(mut sink) = sinks.get_mut(e) {
            sink.set_volume(Volume::Linear(v * master));
        }
    }
}
