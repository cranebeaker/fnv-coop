//! Movies a script plays (`PlayBink`), shown as the game shows them.
//!
//! In the game the script command does not return until the movie is over:
//! its handler (`005d15d0`) calls the movie player, whose loop (`00ec0e00`,
//! `00ec1060`) owns the screen and the input until the last frame or until
//! the movie is interrupted. Here the game's clock stops instead
//! (`Time<Virtual>` paused) and input is withheld from everything else.
//!
//! What the game does each frame, from its code (1.4.0.525):
//!
//! - Decodes the next frame when Bink says it is due (`BinkWait`), copying
//!   it with `BinkCopyToBufferRect` (32-bit surface) into 256x256 X8R8G8B8
//!   tiles (`00ebf8e0`; an edge tile is a power of two big, its spare rows
//!   and columns black). Here `bink::Decoder::to_bgrx` makes the same bytes.
//! - Clears the screen to opaque black (`Clear` through the device's vtable
//!   at +0xac) and draws each tile as a quad with pre-transformed vertices
//!   (`00ec0280`, FVF `XYZRHW | TEX1`, texture coordinates 0 to 1), linear
//!   filtering and clamped addressing (`00ec0460`), no blending and no
//!   depth (`00ebff30`).
//! - Places the movie with the scale and offsets from `00ec2aa0`,
//!   `00ec2b20` and `00ec2bb0` (see [`placement`]).
//! - Stops on control 5 or 28 when the movie is interruptible (`00867440`).
//!
//! Direct3D 9 samples a pixel at its integer coordinates and newer APIs at
//! its centre, so each tile is drawn half a pixel right of and below where
//! the game's vertices put it, to sample the texture at the same places.
//! Filtering here works on linear values (sRGB textures) where the game
//! filtered the stored bytes; texel centres come out exactly the same.
//!
//! Movie sound plays at full loudness: the game never calls
//! `BinkSetVolume`, so Bink's default applies.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Instant;

use bevy::asset::RenderAssetUsages;
use bevy::audio::{AudioPlayer, AudioSink, AudioSinkPlayback, PlaybackSettings};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::image::ImageSampler;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::view::RenderLayers;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use world::scripting::Video;

use crate::sounds::PcmSound;

/// The layer only the movie's camera draws.
const MOVIE_LAYER: usize = 27;

/// The game's tile size (`00ec0cd0` passes 0x100 to `00ebf8e0`).
const TILE: u32 = 256;

/// Movies asked for and the one playing.
#[derive(Resource)]
pub struct Movies {
    /// The Data folder; movies are loose files in its `Video` folder.
    folder: PathBuf,
    /// Whether to play them (`--movies`, `--no-movies`).
    play: bool,
    pub queue: VecDeque<Video>,
    playing: Option<Playing>,
}

impl Movies {
    pub fn new(folder: PathBuf, play: bool) -> Self {
        Movies {
            folder,
            play,
            queue: VecDeque::new(),
            playing: None,
        }
    }
}

struct Tile {
    image: Handle<Image>,
    /// Source rectangle in the frame: x, y, width, height (the copy may run
    /// past the frame's edge, which stays black).
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

struct Playing {
    video: Video,
    bytes: Vec<u8>,
    /// Each frame's video data as a range of `bytes`.
    frames: Vec<std::ops::Range<usize>>,
    fps: f64,
    decoder: bink::Decoder,
    /// Frames decoded so far.
    decoded: usize,
    started: Instant,
    camera: Entity,
    sprites: Vec<Entity>,
    tiles: Vec<Tile>,
    sound: Option<Entity>,
    /// Whether the game's clock was already stopped (left so afterwards).
    clock_was_paused: bool,
    /// Sounds this movie paused, to resume after it.
    paused: Vec<Entity>,
    frame: Vec<u8>,
}

/// Where the game puts a movie of `w x h` on a screen of `sw x sh`: the
/// scale and the top-left corner, in screen pixels.
///
/// Letterboxed (`CalculateScale` 00ec2aa0 and the offsets 00ec2b20, 00ec2bb0
/// with the flag set): the width fills the screen and the picture is
/// centred vertically. Otherwise the height fills it and the picture is
/// centred horizontally. The screen offsets the game adds (its vtable
/// +0x34, +0x38) are 0 for a full-screen device here.
pub fn placement(w: u32, h: u32, sw: f32, sh: f32, letterbox: bool) -> (f32, f32, f32) {
    if letterbox {
        let scale = sw / w as f32;
        (scale, 0.0, (sh - h as f32 * scale) / 2.0)
    } else {
        let scale = sh / h as f32;
        (scale, (sw - w as f32 * scale) / 2.0, 0.0)
    }
}

/// Smallest power of two (at least 4) that holds `n`, as `00ebfc80` rounds
/// an edge tile.
fn edge_tile(n: u32) -> u32 {
    match n {
        0..=2 => n,
        _ => n.next_power_of_two().clamp(4, 1024),
    }
}

/// The tiles `00ebf8e0` makes for a frame of `w x h`: full 256x256 tiles,
/// then a narrower column and a shorter row rounded up to a power of two.
fn tile_layout(w: u32, h: u32) -> Vec<(u32, u32, u32, u32)> {
    let (cols, rows) = (w / TILE, h / TILE);
    let (rw, rh) = (edge_tile(w % TILE), edge_tile(h % TILE));
    let mut out = Vec::new();
    let row = |y: u32, th: u32, out: &mut Vec<_>| {
        for c in 0..cols {
            out.push((c * TILE, y, TILE, th));
        }
        if rw != 0 {
            out.push((cols * TILE, y, rw, th));
        }
    };
    for r in 0..rows {
        row(r * TILE, TILE, &mut out);
    }
    if rh != 0 {
        row(rows * TILE, rh, &mut out);
    }
    out
}

fn tile_image(w: u32, h: u32) -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::linear();
    image
}

/// Copies one tile's part of a frame (`BinkCopyToBufferRect`): what lies
/// outside the frame stays black. Alpha is made opaque; the game's texture
/// format has none and its blending is off.
fn fill_tile(image: &mut Image, frame: &[u8], fw: u32, fh: u32, t: &Tile) {
    let Some(data) = image.data.as_mut() else {
        return;
    };
    for row in 0..t.h {
        let sy = t.y + row;
        if sy >= fh {
            break;
        }
        let w = t.w.min(fw.saturating_sub(t.x)) as usize;
        let src = ((sy * fw + t.x) * 4) as usize;
        let dst = (row * t.w * 4) as usize;
        data[dst..dst + w * 4].copy_from_slice(&frame[src..src + w * 4]);
        for px in data[dst..dst + w * 4].as_chunks_mut::<4>().0 {
            px[3] = 255;
        }
    }
}

/// Whether a movie has the screen (a run condition for what the game's
/// main loop does, which the movie holds).
pub fn playing(movies: Res<Movies>) -> bool {
    // A movie a script just asked for counts too: the command does not
    // return until it is over, so nothing runs in between.
    movies.playing.is_some() || (movies.play && !movies.queue.is_empty())
}

/// Starts the next movie asked for, once nothing else is playing.
#[allow(clippy::too_many_arguments)]
pub fn start_movies(
    mut commands: Commands,
    mut movies: ResMut<Movies>,
    mut images: ResMut<Assets<Image>>,
    mut sounds: ResMut<Assets<PcmSound>>,
    mut clock: ResMut<Time<Virtual>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    sinks: Query<(Entity, &AudioSink)>,
) {
    if movies.playing.is_some() {
        return;
    }
    let Some(video) = movies.queue.pop_front() else {
        return;
    };
    if !movies.play {
        println!(
            "Movie {}: skipped (--no-movies, or a screenshot).",
            video.file
        );
        return;
    }
    let path = movies.folder.join("Video").join(&video.file);
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            // The game reports "Could not open %s for playback." and goes on.
            println!(
                "Movie {}: could not open {}: {e}",
                video.file,
                path.display()
            );
            return;
        }
    };
    let (frames, fps, width, height, revision, flags, track) = {
        let movie = match bink::Movie::parse(&bytes) {
            Ok(m) => m,
            Err(e) => {
                println!("Movie {}: {e}", video.file);
                return;
            }
        };
        let base = bytes.as_ptr() as usize;
        let mut frames = Vec::with_capacity(movie.frame_count() as usize);
        for i in 0..movie.frame_count() {
            match movie.packet(i) {
                Ok(p) => {
                    let start = p.video.as_ptr() as usize - base;
                    frames.push(start..start + p.video.len());
                }
                Err(e) => {
                    println!("Movie {}: {e}", video.file);
                    return;
                }
            }
        }
        // Sound: the first track, decoded whole (the intro's in under a
        // second).
        let track = movie.audio.first().and_then(|t| {
            let mut d = match bink::AudioDecoder::new(t.sample_rate as u32, t.channels(), t.flags) {
                Ok(d) => d,
                Err(e) => {
                    println!("Movie {}: no sound: {e}", video.file);
                    return None;
                }
            };
            let mut samples = Vec::new();
            for i in 0..movie.frame_count() {
                let p = movie.packet(i).ok()?;
                if let Err(e) = d.decode_packet(p.audio[0], &mut samples) {
                    println!("Movie {}: sound stops at frame {i}: {e}", video.file);
                    break;
                }
            }
            Some((t.sample_rate as u32, t.channels(), samples))
        });
        (
            frames,
            movie.fps(),
            movie.width,
            movie.height,
            movie.revision,
            movie.video_flags,
            track,
        )
    };
    if frames.is_empty() || fps <= 0.0 {
        println!("Movie {}: no frames.", video.file);
        return;
    }
    println!(
        "Movie {}: {}x{}, {} frames at {:.3} fps{}{}{}{}.",
        video.file,
        width,
        height,
        frames.len(),
        fps,
        if video.interruptable {
            ", can be skipped"
        } else {
            ""
        },
        if video.mute_audio {
            ", game sound off"
        } else {
            ""
        },
        if video.pause_music {
            ", music paused"
        } else {
            ""
        },
        if video.letterbox { ", letterboxed" } else { "" },
    );

    // The game's clock stops while the movie has the screen.
    let clock_was_paused = clock.is_paused();
    clock.pause();
    // Its sound system is muted (MuteAudio) or its music paused. Music
    // and sounds are both plain audio players here, so either pauses what
    // is playing now.
    let mut paused = Vec::new();
    if video.mute_audio || video.pause_music {
        for (e, sink) in &sinks {
            if !sink.is_paused() {
                sink.pause();
                paused.push(e);
            }
        }
    }

    let camera = commands
        .spawn((
            Camera2d,
            Camera {
                // Over everything else, as the game's own loop draws.
                order: 100,
                hdr: false,
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            Tonemapping::None,
            DebandDither::Disabled,
            Msaa::Off,
            RenderLayers::layer(MOVIE_LAYER),
        ))
        .id();
    let (sw, sh, sf) = windows.single().map_or((1920.0, 1080.0, 1.0), |w| {
        (
            w.physical_width() as f32,
            w.physical_height() as f32,
            w.scale_factor(),
        )
    });
    let (scale, x0, y0) = placement(width, height, sw, sh, video.letterbox);
    let mut tiles = Vec::new();
    let mut sprites = Vec::new();
    for (x, y, w, h) in tile_layout(width, height) {
        let image = images.add(tile_image(w, h));
        // Screen position of the tile's corner, half a pixel on (see the
        // module notes), in the 2D camera's units (logical pixels, origin at
        // the centre, y up).
        let px = x0 + x as f32 * scale + 0.5;
        let py = y0 + y as f32 * scale + 0.5;
        let sprite = commands
            .spawn((
                Sprite {
                    image: image.clone(),
                    custom_size: Some(Vec2::new(w as f32 * scale, h as f32 * scale) / sf),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                Transform::from_xyz((px - sw / 2.0) / sf, (sh / 2.0 - py) / sf, 0.0),
                RenderLayers::layer(MOVIE_LAYER),
            ))
            .id();
        sprites.push(sprite);
        tiles.push(Tile { image, x, y, w, h });
    }
    let sound = track.map(|(rate, channels, samples)| {
        let handle = sounds.add(PcmSound::new(channels, rate, samples));
        commands
            .spawn((AudioPlayer(handle), PlaybackSettings::DESPAWN))
            .id()
    });

    movies.playing = Some(Playing {
        video,
        bytes,
        frames,
        fps,
        decoder: bink::Decoder::new(width, height, revision, flags),
        decoded: 0,
        started: Instant::now(),
        camera,
        sprites,
        tiles,
        sound,
        clock_was_paused,
        paused,
        frame: Vec::new(),
    });
}

/// Keeps the movie's input away from everything else while it plays: the
/// game's movie loop takes the keys itself (`00867440`), and nothing else
/// runs. Runs right after Bevy reads the input.
pub fn withhold_input(
    mut movies: ResMut<Movies>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut buttons: ResMut<ButtonInput<MouseButton>>,
    mut motion: ResMut<AccumulatedMouseMotion>,
) {
    let Some(playing) = movies.playing.as_mut() else {
        return;
    };
    // Control 5 (Activate, E) and 28 (Escape) end an interruptible movie.
    let skip = keys.just_pressed(KeyCode::KeyE) || keys.just_pressed(KeyCode::Escape);
    if skip && playing.video.interruptable {
        // Marked as finished; `play_movies` tears it down.
        playing.decoded = usize::MAX;
    }
    keys.reset_all();
    buttons.reset_all();
    motion.delta = Vec2::ZERO;
}

/// Shows the frame that is due and ends the movie after its last one (or
/// when it was interrupted).
pub fn play_movies(
    mut commands: Commands,
    mut movies: ResMut<Movies>,
    mut images: ResMut<Assets<Image>>,
    mut clock: ResMut<Time<Virtual>>,
    sinks: Query<(Entity, &AudioSink)>,
) {
    let Some(playing) = movies.playing.as_mut() else {
        return;
    };
    // Sounds that start after the movie (or in its first frame, before
    // their sink existed) are held too: the game's sound system stays
    // muted, or its music paused, until the movie ends.
    if playing.video.mute_audio || playing.video.pause_music {
        for (e, sink) in &sinks {
            if Some(e) != playing.sound && !sink.is_paused() {
                sink.pause();
                playing.paused.push(e);
            }
        }
    }
    let count = playing.frames.len();
    let interrupted = playing.decoded == usize::MAX;
    if !interrupted {
        // Due: frame k at k / fps seconds after the start. Frames are
        // decoded in order (each builds on the last); a few at most per
        // update, as the game's loop does one per pass.
        let due = ((playing.started.elapsed().as_secs_f64() * playing.fps) as usize).min(count - 1);
        let mut shown = false;
        let mut budget = 4;
        while playing.decoded <= due && playing.decoded < count && budget > 0 {
            let range = playing.frames[playing.decoded].clone();
            if let Err(e) = playing.decoder.decode(&playing.bytes[range]) {
                println!(
                    "Movie {}: frame {}: {e}",
                    playing.video.file, playing.decoded
                );
            }
            playing.decoded += 1;
            budget -= 1;
            shown = true;
        }
        if shown {
            playing.decoder.to_bgrx(&mut playing.frame);
            let (fw, fh) = (playing.decoder.width(), playing.decoder.height());
            for t in &playing.tiles {
                if let Some(image) = images.get_mut(&t.image) {
                    fill_tile(image, &playing.frame, fw, fh, t);
                }
            }
        }
        // The game's loop ends once the frame counter reaches the frame
        // count, after showing the last frame.
        let last_shown_for = playing.started.elapsed().as_secs_f64() - (count as f64) / playing.fps;
        if playing.decoded < count || last_shown_for < 0.0 {
            return;
        }
    }
    let Some(done) = movies.playing.take() else {
        return;
    };
    println!(
        "Movie {}: {}.",
        done.video.file,
        if interrupted {
            "interrupted"
        } else {
            "finished"
        }
    );
    commands.entity(done.camera).despawn();
    for e in done.sprites {
        commands.entity(e).despawn();
    }
    for t in &done.tiles {
        images.remove(&t.image);
    }
    // BinkClose: the movie's sound stops with it.
    if let Some(e) = done.sound {
        if let Ok(mut c) = commands.get_entity(e) {
            c.despawn();
        }
    }
    for e in done.paused {
        if let Ok((_, sink)) = sinks.get(e) {
            sink.play();
        }
    }
    if !done.clock_was_paused {
        clock.unpause();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_intro_fills_a_wide_screen_and_is_letterboxed_on_a_tall_one() {
        // 1280x720 on 1920x1080: one and a half times, no bars.
        assert_eq!(placement(1280, 720, 1920.0, 1080.0, true), (1.5, 0.0, 0.0));
        // On 1920x1200 the width fills it; bars of 60 above and below.
        assert_eq!(placement(1280, 720, 1920.0, 1200.0, true), (1.5, 0.0, 60.0));
        // Not letterboxed: the height fills it, centred across.
        assert_eq!(
            placement(1280, 720, 1920.0, 1200.0, false),
            (
                1200.0 / 720.0,
                (1920.0 - 1280.0 * (1200.0 / 720.0)) / 2.0,
                0.0
            )
        );
    }

    #[test]
    fn tiles_are_256_with_edges_rounded_up_to_a_power_of_two() {
        let tiles = tile_layout(1280, 720);
        // Five columns, two full rows and a third of 208 rows in a tile of
        // 256.
        assert_eq!(tiles.len(), 15);
        assert_eq!(tiles[0], (0, 0, 256, 256));
        assert_eq!(tiles[14], (1024, 512, 256, 256));
        let odd = tile_layout(300, 100);
        assert_eq!(odd, vec![(0, 0, 256, 128), (256, 0, 64, 128)]);
        assert_eq!(edge_tile(3), 4);
        assert_eq!(edge_tile(17), 32);
    }
}
