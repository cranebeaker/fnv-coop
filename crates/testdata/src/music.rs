//! The music tests' data: a hall with two audio markers, the controllers,
//! sets and music types they play, a region with incidental and battle
//! music for when no controller plays, and MP3 files of silence.
//!
//! - `TestMusicHall` ([`HALL`], an interior): its acoustic space
//!   `TestSpace` ([`SPACE`]) names the region `TestAudioRegion`
//!   ([`REGION`]: incidental set [`INCIDENTAL_SET`], battle sets
//!   [`BATTLE_SET`] and [`BATTLE_SET_2`]);
//! - the marker [`MARKER`] at 0, 0 (radius 2000) plays `TestController`
//!   ([`CONTROLLER`]: neutral list the location set [`LOCATION_SET`];
//!   enemy list the dungeon set [`DUNGEON_SET`]; battle list both battle
//!   sets; its faction `TestTownFaction` has no relation to the player);
//! - the marker [`FAR_MARKER`] at 6500, 0 (no `XRDS`: 5000, so it
//!   overlaps the first from 1500 to 2000 east) plays
//!   `TestOddController` ([`ODD_CONTROLLER`]: location list the location
//!   set, enemy list the dungeon set; its faction `TestOddFaction`'s
//!   relation to the player has **modifier 1** and reaction 3, friend).
//!
//! The location set: day layers `loc\test\day_1low.mp3` (−4 dB, 100 %),
//! `day_2mid` (−5, 60 %), `day_3high` (−6, 20 %); night layers
//! `night_1low` … (−3 dB each, same boundaries); least time on a layer
//! 6 s, cross-fade 9 s. So by day within 894 units of the marker
//! `day_3high`, within 1549 `day_2mid`, within 2000 `day_1low`.

use crate::{group, placed, record, stat, sub, zstr, TempData};

pub const AUDIO_MARKER: u32 = 0x23;
pub const PLAYER_FACTION: u32 = 0x1B2A4;
pub const TOWN_FACTION: u32 = 0xA01;
pub const ODD_FACTION: u32 = 0xA02;
pub const STINGER: u32 = 0xA10;
pub const NO_MUSIC: u32 = 0xA11;
pub const FOLDER_MUSIC: u32 = 0xA12;
pub const LOCATION_SET: u32 = 0xA20;
pub const DUNGEON_SET: u32 = 0xA21;
pub const BATTLE_SET: u32 = 0xA22;
pub const INCIDENTAL_SET: u32 = 0xA23;
pub const BATTLE_SET_2: u32 = 0xA24;
pub const CONTROLLER: u32 = 0xA30;
pub const ODD_CONTROLLER: u32 = 0xA31;
pub const INTRO: u32 = 0xA40;
pub const OUTRO: u32 = 0xA41;
pub const DAY_PHRASE: u32 = 0xA42;
pub const NIGHT_PHRASE: u32 = 0xA43;
pub const REGION: u32 = 0xA50;
pub const SPACE: u32 = 0xA51;
pub const HALL: u32 = 0xB00;
pub const MARKER: u32 = 0xB01;
pub const FAR_MARKER: u32 = 0xB02;
pub const FAR_MARKER_AT: [f32; 3] = [6500.0, 0.0, 0.0];

/// The location set's files, day layers 1–3 then night.
pub const LOCATION_FILES: [&str; 6] = [
    "loc\\test\\day_1low.mp3",
    "loc\\test\\day_2mid.mp3",
    "loc\\test\\day_3high.mp3",
    "loc\\test\\night_1low.mp3",
    "loc\\test\\night_2mid.mp3",
    "loc\\test\\night_3high.mp3",
];
/// The dungeon set's battle, explore and suspense tracks.
pub const DUNGEON_FILES: [&str; 3] = [
    "dngn\\test\\battle.mp3",
    "dngn\\test\\explore.mp3",
    "dngn\\test\\suspense.mp3",
];
pub const BATTLE_FILE: &str = "bttl\\test\\loop.mp3";
pub const BATTLE_FILE_2: &str = "bttl\\test\\loop2.mp3";
pub const STINGER_FILE: &str = "scr\\test_stinger.mp3";

/// An MP3 of `frames` silent frames: MPEG-1 Layer III, 44.1 kHz, 128
/// kbit/s, stereo, 1152 samples a frame, every frame's side information
/// zero (no samples coded, so silence).
pub fn silent_mp3(frames: usize) -> Vec<u8> {
    // 144 × 128000 / 44100 = 417 bytes (no padding).
    let mut frame = vec![0u8; 417];
    frame[..4].copy_from_slice(&[0xFF, 0xFB, 0x90, 0x00]);
    frame.repeat(frames)
}

fn edid(name: &str) -> Vec<u8> {
    sub(b"EDID", &zstr(name))
}

fn form(kind: &[u8; 4], id: u32) -> Vec<u8> {
    sub(kind, &id.to_le_bytes())
}

fn float(kind: &[u8; 4], v: f32) -> Vec<u8> {
    sub(kind, &v.to_le_bytes())
}

fn faction(id: u32, name: &str, toward_player: Option<(i32, u32)>) -> Vec<u8> {
    let mut d = edid(name);
    if let Some((modifier, reaction)) = toward_player {
        let mut x = PLAYER_FACTION.to_le_bytes().to_vec();
        x.extend(modifier.to_le_bytes());
        x.extend(reaction.to_le_bytes());
        d.extend(sub(b"XNAM", &x));
    }
    record(b"FACT", id, &d)
}

fn music_type(id: u32, name: &str, file: Option<&str>, decibels: f32) -> Vec<u8> {
    let mut d = edid(name);
    if let Some(f) = file {
        d.extend(sub(b"FNAM", &zstr(f)));
    }
    d.extend(float(b"ANAM", decibels));
    record(b"MUSC", id, &d)
}

/// A media set: its kind, up to six tracks (file, dB, boundary), enabled
/// layers, `DNAM` `ENAM` `FNAM` `GNAM`, and its two sounds.
fn media_set(
    id: u32,
    name: &str,
    kind: u32,
    tracks: &[(&str, f32, f32)],
    enabled: u8,
    times: [f32; 4],
    sounds: [Option<u32>; 2],
) -> Vec<u8> {
    let mut d = edid(name);
    d.extend(sub(b"NAM1", &kind.to_le_bytes()));
    let paths = [b"NAM2", b"NAM3", b"NAM4", b"NAM5", b"NAM6", b"NAM7"];
    let decibels = [b"NAM8", b"NAM9", b"NAM0", b"ANAM", b"BNAM", b"CNAM"];
    let bounds = [b"JNAM", b"KNAM", b"LNAM", b"MNAM", b"NNAM", b"ONAM"];
    for (i, (file, _, _)) in tracks.iter().enumerate() {
        d.extend(sub(paths[i], &zstr(file)));
    }
    for i in 0..6 {
        let (_, db, bound) = tracks.get(i).copied().unwrap_or(("", -6.0, 100.0));
        d.extend(float(decibels[i], db));
        d.extend(float(bounds[i], bound));
    }
    d.extend(sub(b"PNAM", &[enabled]));
    for (kind, v) in [b"DNAM", b"ENAM", b"FNAM", b"GNAM"].iter().zip(times) {
        d.extend(float(kind, v));
    }
    for (kind, s) in [b"HNAM", b"INAM"].iter().zip(sounds) {
        if let Some(s) = s {
            d.extend(form(kind, s));
        }
    }
    d.extend(sub(b"DATA", &[]));
    record(b"MSET", id, &d)
}

fn controller(id: u32, name: &str, flags: u32, lists: &[(&[u8; 4], u32)], faction: u32) -> Vec<u8> {
    let mut d = edid(name);
    d.extend(sub(b"NAM1", &flags.to_le_bytes()));
    d.extend(float(b"NAM2", 0.0));
    d.extend(float(b"NAM3", 0.0));
    d.extend(float(b"NAM4", 0.0));
    d.extend(sub(b"NAM5", &64u32.to_le_bytes()));
    d.extend(sub(b"NAM6", &255u32.to_le_bytes()));
    for (kind, set) in lists {
        d.extend(form(kind, *set));
    }
    d.extend(form(b"RNAM", faction));
    d.extend(sub(b"FNAM", &0u32.to_le_bytes()));
    record(b"ALOC", id, &d)
}

fn sound(id: u32, name: &str, file: &str) -> Vec<u8> {
    let mut d = edid(name);
    d.extend(sub(b"FNAM", &zstr(file)));
    record(b"SOUN", id, &d)
}

fn audio_marker(id: u32, at: [f32; 3], controller: u32, radius: Option<f32>) -> Vec<u8> {
    let mut extra = sub(b"MMRK", &[]);
    extra.extend(sub(b"FULL", &[0]));
    extra.extend(form(b"CNAM", controller));
    extra.extend(sub(b"BNAM", &1u32.to_le_bytes()));
    extra.extend(float(b"MNAM", 0.33));
    extra.extend(float(b"NNAM", 0.67));
    if let Some(r) = radius {
        extra.extend(float(b"XRDS", r));
    }
    placed(id, AUDIO_MARKER, at, [0.0; 3], &extra)
}

/// Frames in each of the test's MP3s: [`TRACK_MS`] long.
pub const TRACK_FRAMES: usize = 200;
/// 200 × 1152 samples at 44.1 kHz.
pub const TRACK_MS: u32 = 5224;

/// Writes the music test data (see the module notes): the plugin and a
/// silent MP3 for each track ([`TRACK_MS`] long), and two in the folder
/// `music\explore\` that `TestFolderMusic` names.
pub fn hall(tag: &str) -> TempData {
    let data = TempData::new(tag);
    let track = silent_mp3(TRACK_FRAMES);
    let mut files: Vec<&str> = LOCATION_FILES.to_vec();
    files.extend(DUNGEON_FILES);
    files.extend([BATTLE_FILE, BATTLE_FILE_2, STINGER_FILE]);
    for f in files {
        data.write(&format!("music/{}", f.replace('\\', "/")), &track);
    }
    data.write("music/explore/b_second.mp3", &track);
    data.write("music/explore/a_first.mp3", &track);

    let mut statics = stat(AUDIO_MARKER, "AudioMarker", "Marker_Audio.nif");
    statics.extend(stat(0xA70, "Wall", "Test\\Wall.nif"));
    let mut factions = faction(TOWN_FACTION, "TestTownFaction", None);
    factions.extend(faction(ODD_FACTION, "TestOddFaction", Some((1, 3))));
    let mut music = music_type(STINGER, "TestStinger", Some(STINGER_FILE), -7.74);
    music.extend(music_type(NO_MUSIC, "1NoMusic", None, -6.0));
    music.extend(music_type(
        FOLDER_MUSIC,
        "TestFolderMusic",
        Some("explore\\"),
        3.0,
    ));
    let mut sets = media_set(
        LOCATION_SET,
        "TestLocationSet",
        1,
        &[
            (LOCATION_FILES[0], -4.0, 100.0),
            (LOCATION_FILES[1], -5.0, 60.0),
            (LOCATION_FILES[2], -6.0, 20.0),
            (LOCATION_FILES[3], -3.0, 100.0),
            (LOCATION_FILES[4], -3.0, 60.0),
            (LOCATION_FILES[5], -3.0, 20.0),
        ],
        0x3F,
        [6.0, 15.0, 9.0, 0.0],
        [None, None],
    );
    sets.extend(media_set(
        DUNGEON_SET,
        "TestDungeonSet",
        2,
        &[
            (DUNGEON_FILES[0], -2.5, 100.0),
            (DUNGEON_FILES[1], -4.0, 100.0),
            (DUNGEON_FILES[2], -4.5, 100.0),
        ],
        0,
        [6.0, 10.0, 6.0, 0.0],
        [Some(INTRO), Some(OUTRO)],
    ));
    sets.extend(media_set(
        BATTLE_SET,
        "TestBattleSet",
        0,
        &[(BATTLE_FILE, -10.0, 100.0)],
        0,
        [1.0, 3.0, 10.0, 0.0],
        [Some(INTRO), Some(OUTRO)],
    ));
    sets.extend(media_set(
        INCIDENTAL_SET,
        "TestIncidentalSet",
        3,
        &[],
        0,
        [4.0, 6.0, 16.0, 12.0],
        [Some(DAY_PHRASE), Some(NIGHT_PHRASE)],
    ));
    sets.extend(media_set(
        BATTLE_SET_2,
        "TestBattleSet2",
        0,
        &[(BATTLE_FILE_2, -10.0, 100.0)],
        0,
        [1.0, 3.0, 10.0, 0.0],
        [Some(INTRO), Some(OUTRO)],
    ));
    let mut controllers = controller(
        CONTROLLER,
        "TestController",
        0x40,
        &[
            (b"HNAM", LOCATION_SET),
            (b"YNAM", DUNGEON_SET),
            (b"GNAM", BATTLE_SET),
            (b"GNAM", BATTLE_SET_2),
        ],
        TOWN_FACTION,
    );
    controllers.extend(controller(
        ODD_CONTROLLER,
        "TestOddController",
        0x44,
        &[(b"LNAM", LOCATION_SET), (b"YNAM", DUNGEON_SET)],
        ODD_FACTION,
    ));
    let mut sounds = sound(INTRO, "TestIntro", "fx\\mus\\test\\intro.wav");
    sounds.extend(sound(OUTRO, "TestOutro", "fx\\mus\\test\\outro.wav"));
    sounds.extend(sound(DAY_PHRASE, "TestDayPhrase", "fx\\mus\\inc\\day\\"));
    sounds.extend(sound(
        NIGHT_PHRASE,
        "TestNightPhrase",
        "fx\\mus\\inc\\night\\",
    ));
    let mut region = edid("TestAudioRegion");
    let mut rdat = 7u32.to_le_bytes().to_vec();
    rdat.extend([0, 50, 0, 0]);
    region.extend(sub(b"RDAT", &rdat));
    region.extend(form(b"RDSI", INCIDENTAL_SET));
    region.extend(form(b"RDSB", BATTLE_SET));
    region.extend(form(b"RDSB", BATTLE_SET_2));
    let regions = record(b"REGN", REGION, &region);
    let mut space = edid("TestSpace");
    space.extend(form(b"RDAT", REGION));
    let spaces = record(b"ASPC", SPACE, &space);

    let mut hall = edid("TestMusicHall");
    hall.extend(sub(b"DATA", &[1]));
    hall.extend(form(b"XCAS", SPACE));
    let mut cells = record(b"CELL", HALL, &hall);
    let mut refs = audio_marker(MARKER, [0.0, 0.0, 0.0], CONTROLLER, Some(2000.0));
    refs.extend(audio_marker(
        FAR_MARKER,
        FAR_MARKER_AT,
        ODD_CONTROLLER,
        None,
    ));
    refs.extend(placed(0xB03, 0xA70, [100.0, 0.0, 0.0], [0.0; 3], &[]));
    cells.extend(group(
        HALL.to_le_bytes(),
        6,
        &group(HALL.to_le_bytes(), 9, &refs),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &cells)));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"STAT", 0, &statics));
    plugin.extend(group(*b"FACT", 0, &factions));
    plugin.extend(group(*b"SOUN", 0, &sounds));
    plugin.extend(group(*b"MUSC", 0, &music));
    plugin.extend(group(*b"MSET", 0, &sets));
    plugin.extend(group(*b"ALOC", 0, &controllers));
    plugin.extend(group(*b"REGN", 0, &regions));
    plugin.extend(group(*b"ASPC", 0, &spaces));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}
