//! `music <PLACE> ...`: what music the game plays somewhere (see
//! `world::music`): the audio markers there, the controller and set that
//! play, the region music, and the music manager run for a while with what
//! it starts, optionally mixed down to a .wav file.

use std::io::Write;
use std::path::{Path, PathBuf};

use assets::Assets;
use cellview::music::{Mixdown, MusicLibrary};
use esm::{FormId, LoadOrder};
use world::music::{
    acoustic_region, acoustic_space, audio_markers_in_cell, audio_markers_in_world,
    exterior_sound_region, faction_modifier, LocationController, MediaSet, MusicDirector,
    MusicEvent, MusicInputs, PlayerPlace, RegionSound, SetKind, LIST_NAMES,
};
use world::weather::{Climate, DEFAULT_CLIMATE};
use world::{find_worldspace, square_of, WorldGrid};

use crate::records::describe_id;
use crate::render_cmd::world_error;
use crate::CliError;

pub const USAGE: &str =
    "music <PLACE> [X Y] [at X Y] [hour H] [seconds S] [combat S [E]] [play MUSIC] [wav OUT]";

/// What the command was asked.
struct Request {
    place: String,
    square: Option<(i32, i32)>,
    at: Option<[f32; 2]>,
    hour: Option<f32>,
    seconds: f32,
    /// A fight from, and until.
    combat: Option<(f32, f32)>,
    play: Option<String>,
    wav: Option<PathBuf>,
}

fn parse(args: &[String]) -> Result<Request, CliError> {
    let usage = |what: &str| CliError::Usage(format!("music: {what}; usage: {USAGE}"));
    let number = |s: Option<&String>, what: &str| -> Result<f32, CliError> {
        s.and_then(|v| v.parse::<f32>().ok())
            .filter(|v| v.is_finite())
            .ok_or_else(|| usage(&format!("{what} needs a number")))
    };
    let mut r = Request {
        place: args.first().cloned().ok_or_else(|| usage("give a place"))?,
        square: None,
        at: None,
        hour: None,
        seconds: 15.0,
        combat: None,
        play: None,
        wav: None,
    };
    let mut i = 1;
    while i < args.len() {
        match args[i].to_ascii_lowercase().as_str() {
            "at" => {
                r.at = Some([
                    number(args.get(i + 1), "at")?,
                    number(args.get(i + 2), "at")?,
                ]);
                i += 3;
            }
            "hour" => {
                r.hour = Some(number(args.get(i + 1), "hour")?);
                i += 2;
            }
            "seconds" => {
                r.seconds = number(args.get(i + 1), "seconds")?.clamp(0.1, 3600.0);
                i += 2;
            }
            "combat" => {
                let from = number(args.get(i + 1), "combat")?;
                match args.get(i + 2).and_then(|s| s.parse::<f32>().ok()) {
                    Some(to) => {
                        r.combat = Some((from, to));
                        i += 3;
                    }
                    None => {
                        r.combat = Some((from, f32::MAX));
                        i += 2;
                    }
                }
            }
            "play" => {
                r.play = Some(
                    args.get(i + 1)
                        .cloned()
                        .ok_or_else(|| usage("play needs a music type"))?,
                );
                i += 2;
            }
            "wav" => {
                r.wav = Some(PathBuf::from(
                    args.get(i + 1).ok_or_else(|| usage("wav needs a file"))?,
                ));
                i += 2;
            }
            _ => match (
                args[i].parse::<i32>(),
                args.get(i + 1).map(|s| s.parse::<i32>()),
            ) {
                (Ok(x), Some(Ok(y))) if r.square.is_none() => {
                    r.square = Some((x, y));
                    i += 2;
                }
                _ => return Err(usage(&format!("didn't understand '{}'", args[i]))),
            },
        }
    }
    Ok(r)
}

fn name_of(order: &LoadOrder, id: FormId) -> String {
    order
        .get(id)
        .and_then(|r| r.editor_id().ok().flatten())
        .unwrap_or_else(|| id.to_string())
}

/// Where the player is asked to be.
fn place(order: &LoadOrder, r: &Request) -> Result<PlayerPlace, CliError> {
    if let Some(world) = find_worldspace(order, &r.place).map_err(world_error)? {
        let grid = WorldGrid::load(order, world).map_err(world_error)?;
        let size = world::land::CELL_SIZE;
        let feet = match (r.at, r.square) {
            (Some([x, y]), _) => [x, y, 0.0],
            (None, Some((x, y))) => [(x as f32 + 0.5) * size, (y as f32 + 0.5) * size, 0.0],
            (None, None) => [0.5 * size, 0.5 * size, 0.0],
        };
        return Ok(PlayerPlace {
            cell: grid.cell_at(square_of(feet)),
            world: Some(world),
            position: feet,
        });
    }
    let cells = world::find_cells(order, &r.place).map_err(world_error)?;
    let Some(&cell) = cells.first() else {
        return Err(CliError::NotFound(format!(
            "no cell or worldspace matches '{}'",
            r.place
        )));
    };
    let info = world::cell_info(order, cell).map_err(world_error)?;
    if let (false, Some(world)) = (info.interior, info.world) {
        let size = world::land::CELL_SIZE;
        let (x, y) = info.grid.unwrap_or((0, 0));
        let feet = match r.at {
            Some([px, py]) => [px, py, 0.0],
            None => [(x as f32 + 0.5) * size, (y as f32 + 0.5) * size, 0.0],
        };
        return Ok(PlayerPlace {
            cell: Some(cell),
            world: Some(world),
            position: feet,
        });
    }
    let feet = match r.at {
        Some([x, y]) => [x, y, 0.0],
        None => world::load_cell(order, cell)
            .ok()
            .and_then(|c| c.arrivals.first().map(|a| a.position))
            .unwrap_or([0.0; 3]),
    };
    Ok(PlayerPlace {
        cell: Some(cell),
        world: None,
        position: feet,
    })
}

fn describe_set(out: &mut impl Write, set: &MediaSet, radius: Option<f32>) -> Result<(), CliError> {
    let kind = set.kind.map_or("unknown kind", SetKind::name);
    writeln!(out, "    {} ({kind} set):", set.label())?;
    match set.kind {
        Some(SetKind::Location) => {
            for (i, l) in set.layers.iter().enumerate() {
                let day = if i < 3 { "day" } else { "night" };
                let on = set.enabled & (1 << i) != 0;
                let reach = radius.map_or(String::new(), |r| {
                    format!(
                        " (within {:.0} units)",
                        r * (l.boundary / 100.0).max(0.0).sqrt()
                    )
                });
                writeln!(
                    out,
                    "      {day} layer {}: {} at {} dB, boundary {} %{reach}{}",
                    i % 3 + 1,
                    l.file.as_deref().unwrap_or("(none)"),
                    l.decibels,
                    l.boundary,
                    if on { "" } else { ", off" }
                )?;
            }
            writeln!(
                out,
                "      least time on a layer {} s, cross-fade {} s",
                set.dnam, set.fnam
            )?;
        }
        Some(SetKind::Dungeon) => {
            for (i, what) in ["battle", "explore", "suspense"].iter().enumerate() {
                let l = &set.layers[i];
                writeln!(
                    out,
                    "      {what}: {} at {} dB",
                    l.file.as_deref().unwrap_or("(none)"),
                    l.decibels
                )?;
            }
            writeln!(
                out,
                "      least time on a track {} s, cross-fade {} s",
                set.dnam, set.fnam
            )?;
        }
        Some(SetKind::Battle) => {
            let l = &set.layers[0];
            writeln!(
                out,
                "      loop: {} at {} dB, fades {} s, recovery {} s, held {} ms after the intro",
                l.file.as_deref().unwrap_or("(none)"),
                l.decibels,
                set.enam,
                set.fnam,
                set.dnam
            )?;
        }
        Some(SetKind::Incidental) => {
            writeln!(
                out,
                "      a phrase every {}–{} s by day, {}–{} s by night",
                set.dnam, set.fnam, set.enam, set.gnam
            )?;
        }
        None => {}
    }
    Ok(())
}

fn sound_names(order: &LoadOrder, set: &MediaSet) -> String {
    let n = |s: Option<FormId>| s.map_or("none".into(), |s| name_of(order, s));
    match set.kind {
        Some(SetKind::Incidental) => format!("day {}, night {}", n(set.hnam), n(set.inam)),
        _ => format!("intro {}, outro {}", n(set.hnam), n(set.inam)),
    }
}

pub fn music(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    data_dir: &Path,
    args: &[String],
) -> Result<(), CliError> {
    let r = parse(args)?;
    let player = place(order, &r)?;
    let (cell, world, feet) = (player.cell, player.world, player.position);
    let state = world::scripting::GameState::new(order);
    let hour = r
        .hour
        .or_else(|| state.global(order, "GameHour"))
        .unwrap_or(world::weather::DEFAULT_HOUR);
    // The sky's climate: the worldspace's outdoors; indoors in a new game
    // there was none before, so `DefaultClimate`.
    let climate_id = world
        .and_then(|w| world::Worldspace::load(order, w).ok())
        .and_then(|w| w.climate)
        .unwrap_or(DEFAULT_CLIMATE);
    let climate = Climate::load(order, climate_id);
    let settings = assets::IniSettings::load(&assets::default_settings_files(data_dir));
    let volumes = cellview::music::volumes(&settings);

    writeln!(
        out,
        "Place: {}{}, the player at ({:.1}, {:.1}); {:02}:{:02}",
        cell.map_or("no cell".into(), |c| describe_id(order, c)),
        world.map_or(String::new(), |w| format!(" in {}", name_of(order, w))),
        feet[0],
        feet[1],
        hour.floor() as u32,
        ((hour - hour.floor()) * 60.0).round() as u32
    )?;
    if let Some(c) = &climate {
        writeln!(
            out,
            "Climate {}: controllers' day {:.2}–{:.2} h (the middles of sunrise and sunset), incidental day {:.2}–{:.2} h",
            name_of(order, c.form_id),
            (c.sunrise.0 + c.sunrise.1) / 2.0,
            (c.sunset.0 + c.sunset.1) / 2.0,
            c.sunrise.0,
            c.sunset.1
        )?;
    }
    writeln!(
        out,
        "Volumes: master {}, music {} (INI)",
        volumes.master, volumes.music
    )?;

    // The audio markers, in the game's order.
    let markers = match (world, cell) {
        (Some(w), _) => audio_markers_in_world(order, w),
        (None, Some(c)) => audio_markers_in_cell(order, c),
        _ => Vec::new(),
    };
    writeln!(out)?;
    writeln!(
        out,
        "Audio markers ({}), in the order the game walks them:",
        markers.len()
    )?;
    let mut by_distance: Vec<usize> = (0..markers.len()).collect();
    by_distance.sort_by(|&a, &b| {
        markers[a]
            .distance2(feet)
            .total_cmp(&markers[b].distance2(feet))
    });
    for (shown, &i) in by_distance.iter().enumerate() {
        let m = &markers[i];
        if shown >= 8 {
            writeln!(out, "  ... ({} more, farther)", markers.len() - shown)?;
            break;
        }
        let d = m.distance2(feet).sqrt();
        writeln!(
            out,
            "  #{i} {} at ({:.0}, {:.0}): {:.0} units away, radius {:.0}{}; controller {}",
            m.reference,
            m.position[0],
            m.position[1],
            d,
            m.radius,
            if m.distance2(feet) < m.radius * m.radius {
                " (holds the player)"
            } else {
                ""
            },
            m.controller.map_or("none".into(), |c| name_of(order, c))
        )?;
    }

    // The music manager, run.
    let mut library = MusicLibrary::default();
    let mut director = MusicDirector::new(volumes, 0x9E37_79B9_7F4A_7C15);
    let mut mix = r
        .wav
        .as_ref()
        .map(|_| (Mixdown::new(48_000), Vec::<f32>::new()));
    let step = 16u64;
    let mut now = 1000u64;
    let steps = (f64::from(r.seconds) * 1000.0 / step as f64).ceil() as u64;
    let play = match &r.play {
        Some(m) => Some(crate::records::find_record(order, m).map(|rr| rr.form_id)?),
        None => None,
    };
    let mut log = Vec::new();
    for n in 0..steps {
        now += step;
        let t = (n * step) as f32 / 1000.0;
        let inputs = MusicInputs {
            now_ms: now,
            hour,
            climate: climate.as_ref(),
            player: Some(player),
            combat: r.combat.is_some_and(|(from, to)| t >= from && t < to),
            aware: &[],
            dead: false,
        };
        let mut files = library.files(assets);
        match (n, play) {
            (0, Some(m)) => director.play_music(order, m, &inputs, &mut files),
            _ => director.update(order, &inputs, &mut files),
        }
        director.tick(now);
        for e in director.take_events() {
            log.push((t, e));
        }
        if let Some((mixdown, samples)) = &mut mix {
            let frames = (48_000 * step / 1000) as usize;
            mixdown.mix(
                &director.decks.decks,
                frames,
                &mut |p| assets.read(p).ok().flatten(),
                samples,
            );
        }
    }

    // What was chosen.
    let (listed, chosen) = director.markers();
    writeln!(out)?;
    match chosen.map(|i| listed[i].clone()) {
        Some(m) => {
            let d = m.distance2(feet);
            let pct = if d <= m.radius * m.radius {
                100.0 * d / (m.radius * m.radius)
            } else {
                100_000.0
            };
            writeln!(
                out,
                "Chosen marker: {} ({:.0} units away: {:.1} % of its radius², {})",
                m.reference,
                d.sqrt(),
                pct,
                if pct < 100.0 { "inside" } else { "outside" }
            )?;
            if let Some(c) = m
                .controller
                .and_then(|c| LocationController::load(order, c))
            {
                writeln!(
                    out,
                    "Controller {}: NAM1 0x{:02X} (plays its {} list; {}; day and night {}), delay {} s",
                    c.label(),
                    c.flags,
                    LIST_NAMES.get(c.default_list() as usize).copied().unwrap_or("(no)"),
                    if c.track_end() == 0 { "tracks start again at their end" } else { "a set ends with its track" },
                    if c.climate_days() { "by the climate" } else { "by NAM5/NAM6" },
                    c.delay
                )?;
                if let Some(f) = c.faction {
                    writeln!(
                        out,
                        "  faction {}: its relation's modifier toward the player is {} (the list it plays while a member is aware of the player: {})",
                        name_of(order, f),
                        faction_modifier(order, f, world::factions::PLAYER_FACTION),
                        match faction_modifier(order, f, world::factions::PLAYER_FACTION) {
                            m @ 0..=3 => LIST_NAMES[m as usize],
                            _ => "none",
                        }
                    )?;
                }
                for (i, list) in c.lists.iter().enumerate() {
                    if list.is_empty() {
                        continue;
                    }
                    writeln!(
                        out,
                        "  {} list (as kept, last in the file first): {}",
                        LIST_NAMES[i],
                        list.iter()
                            .map(|s| name_of(order, *s))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )?;
                }
                let mut seen = Vec::new();
                for &s in c.lists.iter().flatten() {
                    if seen.contains(&s) {
                        continue;
                    }
                    seen.push(s);
                    if let Some(set) = MediaSet::load(order, s) {
                        describe_set(out, &set, Some(m.radius))?;
                        if matches!(set.kind, Some(SetKind::Battle | SetKind::Dungeon)) {
                            writeln!(out, "      sounds: {}", sound_names(order, &set))?;
                        }
                    }
                }
            }
        }
        None => writeln!(out, "No audio marker here.")?,
    }

    // Region music.
    let space = cell.and_then(|c| acoustic_space(order, c));
    let square_region = match (world, cell) {
        (Some(w), Some(c)) => exterior_sound_region(order, c, w, feet),
        _ => None,
    };
    let space_region = space.and_then(|s| acoustic_region(order, s));
    writeln!(out)?;
    writeln!(
        out,
        "Acoustic space: {}{}",
        space.map_or("none".into(), |s| name_of(order, s)),
        space_region.map_or(String::new(), |r| format!(", region {}", name_of(order, r)))
    )?;
    if let Some(r) = square_region {
        writeln!(
            out,
            "The square's region with incidental music: {}",
            name_of(order, r)
        )?;
    }
    if let Some(data) = square_region
        .or(space_region)
        .and_then(|r| RegionSound::load(order, r))
    {
        writeln!(
            out,
            "Region music (when no controller plays) from {}: incidental {}; battle sets {}",
            name_of(order, data.region),
            data.incidental.map_or("none".into(), |s| name_of(order, s)),
            if data.battle.is_empty() {
                "none".into()
            } else {
                data.battle
                    .iter()
                    .map(|s| name_of(order, *s))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        )?;
        if let Some(set) = data.incidental.and_then(|s| MediaSet::load(order, s)) {
            describe_set(out, &set, None)?;
            writeln!(out, "      sounds: {}", sound_names(order, &set))?;
        }
    }

    writeln!(out)?;
    writeln!(
        out,
        "The music manager over {} s{}:",
        r.seconds,
        r.combat
            .map_or(String::new(), |(from, to)| if to == f32::MAX {
                format!(", a fight from {from} s")
            } else {
                format!(", a fight from {from} s to {to} s")
            })
    )?;
    for (t, e) in &log {
        match e {
            MusicEvent::Note(n) => writeln!(out, "  {t:6.2} s  {n}")?,
            MusicEvent::Sound(s) => writeln!(out, "  {t:6.2} s  Sound: {}", name_of(order, *s))?,
        }
    }
    writeln!(out, "At the end:")?;
    for line in director.describe_decks() {
        writeln!(out, "  {line}")?;
    }

    if let (Some(path), Some((_, samples))) = (&r.wav, mix) {
        let pcm: Vec<i16> = samples.iter().map(|&s| mp3::to_i16(s)).collect();
        let file = std::fs::File::create(path).map_err(|e| CliError::Open {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;
        let mut w = std::io::BufWriter::new(file);
        mp3::wav::write(&mut w, 48_000, 2, &pcm)?;
        w.flush()?;
        writeln!(
            out,
            "Wrote {:.1} s of the decks mixed (48 kHz stereo, at their volumes) to {}",
            pcm.len() as f64 / 96_000.0,
            path.display()
        )?;
    }
    Ok(())
}
