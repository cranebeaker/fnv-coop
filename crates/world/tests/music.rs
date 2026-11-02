//! The music manager on the test hall (`testdata::music::hall`): audio
//! markers, controllers, the four kinds of set, region music, `PlayMusic`,
//! and the decks they play on.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::music::*;
use world::music::{
    audio_markers_in_cell, faction_modifier, kind, AudioMarker, AwareActor, LocationController,
    MediaSet, MusicDirector, MusicEvent, MusicFiles, MusicInputs, MusicType, PlayerPlace,
    RegionSound, SetKind, Volumes,
};
use world::weather::Climate;

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = hall(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn path(file: &str) -> String {
    format!("music\\{file}")
}

/// Every test track is a minute long; the folder holds two.
struct Files;

impl MusicFiles for Files {
    fn duration_ms(&mut self, p: &str) -> Option<u32> {
        let mut known: Vec<String> = LOCATION_FILES.iter().map(|f| path(f)).collect();
        known.extend(DUNGEON_FILES.iter().map(|f| path(f)));
        known.extend([BATTLE_FILE, BATTLE_FILE_2, STINGER_FILE].map(path));
        known.extend([
            "music\\explore\\a.mp3".into(),
            "music\\explore\\b.mp3".into(),
        ]);
        known.iter().any(|k| k == p).then_some(60_000)
    }

    fn folder(&mut self, f: &str) -> Vec<String> {
        if f == "music\\explore\\" {
            vec![
                "music\\explore\\a.mp3".into(),
                "music\\explore\\b.mp3".into(),
            ]
        } else {
            Vec::new()
        }
    }
}

/// New Vegas's climate times: sunrise 6–8, sunset 18–20.
fn climate() -> Climate {
    Climate {
        form_id: FormId(0x15F),
        editor_id: None,
        weathers: Vec::new(),
        sunrise: (6.0, 8.0),
        sunset: (18.0, 20.0),
        sun: None,
        sun_glare: None,
        stars: None,
    }
}

/// The manager run frame by frame (16 ms), with the player somewhere in
/// the hall.
struct Run {
    order: LoadOrder,
    music: MusicDirector,
    now: u64,
    hour: f32,
    climate: Climate,
    at: [f32; 3],
    combat: bool,
    aware: Vec<AwareActor>,
    events: Vec<MusicEvent>,
}

impl Run {
    fn new(order: LoadOrder, at: [f32; 3]) -> Run {
        Run {
            order,
            music: MusicDirector::new(Volumes::default(), 7),
            now: 10_000,
            hour: 10.0,
            climate: climate(),
            at,
            combat: false,
            aware: Vec::new(),
            events: Vec::new(),
        }
    }

    /// One frame; with `play`, a script's `PlayMusic` first.
    fn step(&mut self, play: Option<u32>) {
        self.now += 16;
        let inputs = MusicInputs {
            now_ms: self.now,
            hour: self.hour,
            climate: Some(&self.climate),
            player: Some(PlayerPlace {
                cell: Some(FormId(HALL)),
                world: None,
                position: self.at,
            }),
            combat: self.combat,
            aware: &self.aware,
            dead: false,
        };
        match play {
            Some(m) => self
                .music
                .play_music(&self.order, FormId(m), &inputs, &mut Files),
            None => self.music.update(&self.order, &inputs, &mut Files),
        }
        self.music.tick(self.now);
        self.events.extend(self.music.take_events());
    }

    fn frame(&mut self) {
        self.step(None);
    }

    fn run(&mut self, ms: u64) {
        for _ in 0..ms.div_ceil(16) {
            self.frame();
        }
    }

    fn play(&mut self, music: u32) {
        self.step(Some(music));
    }

    /// The deck holding a file, if one does (and isn't fading out).
    fn deck(&self, file: &str) -> Option<&world::music::Deck> {
        let p = path(file);
        self.music
            .decks
            .decks
            .iter()
            .find(|d| d.is_live() && !d.stopping() && d.path.as_deref() == Some(p.as_str()))
    }

    fn sounds(&self) -> Vec<FormId> {
        self.events
            .iter()
            .filter_map(|e| match e {
                MusicEvent::Sound(s) => Some(*s),
                MusicEvent::Note(_) => None,
            })
            .collect()
    }
}

#[test]
fn records_read_as_the_game_reads_them() {
    let (_data, order) = order("records");
    let stinger = MusicType::load(&order, FormId(STINGER)).unwrap();
    assert_eq!(
        stinger.file.as_deref(),
        Some("music\\scr\\test_stinger.mp3")
    );
    assert!(!stinger.loops());
    assert_eq!(stinger.volume_db(), -7.74);
    // `1NoMusic` has no file; a folder's sign says it loops.
    assert_eq!(
        MusicType::load(&order, FormId(NO_MUSIC)).unwrap().file,
        None
    );
    let folder = MusicType::load(&order, FormId(FOLDER_MUSIC)).unwrap();
    assert!(folder.is_folder() && folder.loops());
    assert_eq!(folder.volume_db(), -3.0);

    let set = MediaSet::load(&order, FormId(LOCATION_SET)).unwrap();
    assert_eq!(set.kind, Some(SetKind::Location));
    assert_eq!(
        set.layers[2].file.as_deref(),
        Some("music\\loc\\test\\day_3high.mp3")
    );
    assert_eq!(
        (set.layers[2].decibels, set.layers[2].boundary),
        (-6.0, 20.0)
    );
    assert_eq!((set.enabled, set.dnam, set.fnam), (0x3F, 6.0, 9.0));
    let battle = MediaSet::load(&order, FormId(BATTLE_SET)).unwrap();
    assert_eq!(battle.kind, Some(SetKind::Battle));
    assert_eq!(
        (battle.hnam, battle.inam),
        (Some(FormId(INTRO)), Some(FormId(OUTRO)))
    );

    let c = LocationController::load(&order, FormId(CONTROLLER)).unwrap();
    // Lists are kept in reverse file order.
    assert_eq!(c.lists[5], vec![FormId(BATTLE_SET_2), FormId(BATTLE_SET)]);
    assert_eq!(c.lists[0], vec![FormId(LOCATION_SET)]);
    assert_eq!((c.default_list(), c.track_end()), (0, 0));
    assert!(c.climate_days());
    assert_eq!(c.faction, Some(FormId(TOWN_FACTION)));
    // Day for a climate controller: from the middle of sunrise to the
    // middle of sunset.
    let cl = climate();
    assert!(!c.is_day(Some(&cl), 6.9));
    assert!(c.is_day(Some(&cl), 7.0));
    assert!(c.is_day(Some(&cl), 19.0));
    assert!(!c.is_day(Some(&cl), 19.1));

    // The cell's markers, last placed first; no `XRDS` is 5000.
    let markers = audio_markers_in_cell(&order, FormId(HALL));
    let ids: Vec<u32> = markers.iter().map(|m| m.reference.0).collect();
    assert_eq!(ids, vec![FAR_MARKER, MARKER]);
    assert_eq!(markers[0].radius, 5000.0);
    assert_eq!(markers[1].radius, 2000.0);
    assert_eq!(markers[1].controller, Some(FormId(CONTROLLER)));
    assert!(AudioMarker::load(&order, FormId(0xB03)).is_none());

    let region = RegionSound::load(&order, FormId(REGION)).unwrap();
    assert_eq!(region.incidental, Some(FormId(INCIDENTAL_SET)));
    assert_eq!(
        region.battle,
        vec![FormId(BATTLE_SET), FormId(BATTLE_SET_2)]
    );
    assert_eq!(
        world::music::acoustic_region(&order, FormId(SPACE)),
        Some(FormId(REGION))
    );

    // The controllers read a faction's relation's *modifier*.
    let player = FormId(PLAYER_FACTION);
    assert_eq!(faction_modifier(&order, FormId(ODD_FACTION), player), 1);
    assert_eq!(faction_modifier(&order, FormId(TOWN_FACTION), player), 0);
}

#[test]
fn the_marker_holding_the_player_wins_else_the_nearest() {
    let (_data, order) = order("markers");
    let mut run = Run::new(order, [500.0, 0.0, 0.0]);
    run.frame();
    let chosen = |run: &Run| {
        let (markers, i) = run.music.markers();
        i.map(|i| markers[i].reference.0)
    };
    assert_eq!(chosen(&run), Some(MARKER));
    // Both hold the player at 1800 east: the first listed wins, though
    // the other is nearer. The choice waits a second.
    run.at = [1800.0, 0.0, 0.0];
    run.frame();
    assert_eq!(chosen(&run), Some(MARKER));
    run.run(1100);
    assert_eq!(chosen(&run), Some(FAR_MARKER));
    // Neither: the nearest.
    run.at = [-4000.0, 3000.0, 0.0];
    run.run(1100);
    assert_eq!(chosen(&run), Some(MARKER));
}

#[test]
fn a_location_set_plays_the_layer_for_the_distance_in_step() {
    let (_data, order) = order("layers");
    let mut run = Run::new(order, [500.0, 0.0, 0.0]);
    run.frame();
    // 500 units from the marker, 6 % of its radius²: the innermost layer,
    // fading in over 9 s at −6 dB.
    let d = run.deck(LOCATION_FILES[2]).expect("day_3high plays");
    assert_eq!(d.kind, kind::LOCATION);
    assert_eq!(d.decibels, -6.0);
    assert!(d.looping());
    run.run(9500);
    let full = world::music::gain(kind::LOCATION, -6.0, &Volumes::default());
    assert!((run.deck(LOCATION_FILES[2]).unwrap().volume - full).abs() < 1e-4);
    let position = run.deck(LOCATION_FILES[2]).unwrap().position_ms;
    // 1200 units out (36 %): the middle layer, starting where the first
    // one is.
    run.at = [1200.0, 0.0, 0.0];
    run.frame();
    let mid = run.deck(LOCATION_FILES[1]).expect("day_2mid plays");
    assert!(
        (mid.position_ms - (position + 16.0)).abs() < 40.0,
        "{}",
        mid.position_ms
    );
    assert_eq!(mid.decibels, -5.0);
    // Back in at once: the least time on a layer (6 s) holds it.
    run.at = [500.0, 0.0, 0.0];
    run.run(5000);
    assert!(run.deck(LOCATION_FILES[1]).is_some());
    assert!(run.deck(LOCATION_FILES[2]).is_none());
    run.run(1500);
    assert!(run.deck(LOCATION_FILES[2]).is_some());
    // By night, the night layers.
    run.hour = 22.0;
    run.run(7000);
    assert!(run.deck(LOCATION_FILES[5]).is_some());
}

#[test]
fn a_track_starts_again_its_cross_fade_before_its_end() {
    let (_data, order) = order("restart");
    let mut run = Run::new(order, [500.0, 0.0, 0.0]);
    run.frame();
    let first = run.deck(LOCATION_FILES[2]).unwrap().generation;
    // A minute long, cross-fade 9 s: asked for again 51 s in, from the
    // start, while the old one fades out.
    run.run(50_900);
    assert_eq!(run.deck(LOCATION_FILES[2]).unwrap().generation, first);
    run.run(200);
    let again = run.deck(LOCATION_FILES[2]).unwrap();
    assert_ne!(again.generation, first);
    assert!(again.position_ms < 250.0, "{}", again.position_ms);
    assert!(run
        .music
        .decks
        .decks
        .iter()
        .any(|d| d.generation == first && d.stopping()));
}

#[test]
fn a_fight_brings_battle_music_and_its_end_the_location_back() {
    let (_data, order) = order("battle");
    let mut run = Run::new(order, [500.0, 0.0, 0.0]);
    run.run(10_000);
    run.combat = true;
    run.frame();
    // A battle set from the controller's battle list, its intro, the loop
    // fading in over 3 s; the location fades out over the same.
    assert_eq!(run.sounds(), vec![FormId(INTRO)]);
    let battle = run
        .music
        .decks
        .decks
        .iter()
        .find(|d| d.kind == kind::BATTLE)
        .expect("a battle deck")
        .clone();
    assert!(battle.path == Some(path(BATTLE_FILE)) || battle.path == Some(path(BATTLE_FILE_2)));
    assert_eq!((battle.decibels, battle.fade_ms), (-10.0, 3000));
    // Held for the battle set's 1 ms, then playing.
    assert!(!battle.running());
    run.frame();
    let running = run
        .music
        .decks
        .decks
        .iter()
        .find(|d| d.kind == kind::BATTLE)
        .unwrap()
        .running();
    assert!(running);
    assert!(run.deck(LOCATION_FILES[2]).is_none());
    run.run(20_000);
    // The fight over: the outro, the loop fading, the location from the
    // start (fading in over 9 s; the loop's fade-out stretched to match).
    run.combat = false;
    run.events.clear();
    run.frame();
    assert_eq!(run.sounds(), vec![FormId(OUTRO)]);
    let location = run.deck(LOCATION_FILES[2]).expect("the location again");
    assert!(location.position_ms < 50.0);
    let fading = run
        .music
        .decks
        .decks
        .iter()
        .find(|d| d.kind == kind::BATTLE)
        .unwrap();
    assert!(fading.stopping() && fading.fade_ms == 9000);
}

#[test]
fn play_music_holds_everything_else_until_it_ends() {
    let (_data, order) = order("stinger");
    let mut run = Run::new(order, [500.0, 0.0, 0.0]);
    run.run(10_000);
    run.play(STINGER);
    let s = run.deck(STINGER_FILE).expect("the stinger");
    assert_eq!((s.kind, s.decibels, s.fade_ms), (kind::SCRIPT, -7.74, 1000));
    assert!(!s.looping());
    // The location fades out over the stinger's second, and nothing
    // starts while it plays.
    run.run(30_000);
    assert!(run.deck(LOCATION_FILES[2]).is_none());
    assert!(run.deck(STINGER_FILE).is_some());
    // It ends at a minute; the location set starts again from its start.
    run.run(31_000);
    assert!(run.deck(STINGER_FILE).is_none());
    let back = run.deck(LOCATION_FILES[2]).expect("the location again");
    assert!(back.position_ms < 1000.0, "{}", back.position_ms);
    // `1NoMusic` asks for nothing.
    let before = run.music.decks.clone();
    run.play(NO_MUSIC);
    assert_eq!(
        run.music.decks.decks[0].generation,
        before.decks[0].generation
    );
    assert_eq!(
        run.music.decks.decks[1].generation,
        before.decks[1].generation
    );
    // A folder's tracks: picked from its files.
    run.play(FOLDER_MUSIC);
    let picked = run
        .music
        .decks
        .decks
        .iter()
        .find(|d| d.kind == kind::SCRIPT)
        .and_then(|d| d.path.clone())
        .unwrap();
    assert!(picked.starts_with("music\\explore\\"), "{picked}");
}

#[test]
fn outside_every_radius_the_region_plays_its_phrases() {
    let (_data, order) = order("region");
    let mut run = Run::new(order, [-4000.0, 3000.0, 0.0]);
    run.run(40_000);
    // Nothing on the decks; the region's day phrase every 4–16 s.
    assert!(run.music.decks.decks.iter().all(|d| !d.is_live()));
    let sounds = run.sounds();
    assert!((3..=11).contains(&sounds.len()), "{sounds:?}");
    assert!(sounds.iter().all(|&s| s == FormId(DAY_PHRASE)));
    // By night, the night phrase.
    run.hour = 22.0;
    run.events.clear();
    run.run(20_000);
    assert!(run.sounds().contains(&FormId(NIGHT_PHRASE)));
    // A fight out here: a region battle set.
    run.combat = true;
    run.frame();
    assert!(run
        .music
        .decks
        .decks
        .iter()
        .any(|d| d.kind == kind::BATTLE && !d.stopping()));
    assert!(run.music.region_battle().is_some());
}

#[test]
fn a_faction_member_aware_of_the_player_turns_to_the_list_its_modifier_names() {
    let (_data, order) = order("faction");
    // By the far marker, whose controller plays its location list.
    let mut run = Run::new(order, [6500.0, 500.0, 0.0]);
    run.run(2000);
    assert!(run.deck(LOCATION_FILES[2]).is_some());
    // Someone of its faction comes to know of the player: the faction's
    // relation to the player says "friend" but its modifier is 1, which
    // the game reads as enemy: the enemy list's dungeon set, exploring,
    // once the least time on the location's layer (6 s) is up.
    run.aware = vec![AwareActor {
        actor: FormId(0xC00),
        hostile: false,
        factions: vec![FormId(ODD_FACTION)],
    }];
    run.run(3000);
    assert!(run.deck(DUNGEON_FILES[1]).is_none());
    run.run(1500);
    let explore = run
        .deck(DUNGEON_FILES[1])
        .expect("the dungeon's explore track");
    assert_eq!(explore.kind, kind::DUNGEON);
    // They turn hostile: suspense once the least time on a track (6 s)
    // is up.
    run.aware[0].hostile = true;
    run.run(7000);
    assert!(run.deck(DUNGEON_FILES[2]).is_some());
    // A fight: the dungeon's own battle track and intro, at once.
    run.events.clear();
    run.combat = true;
    run.frame();
    assert!(run.deck(DUNGEON_FILES[0]).is_some());
    assert_eq!(run.sounds(), vec![FormId(INTRO)]);
}
