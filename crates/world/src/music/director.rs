//! The music manager (`0082fb70`, run once a frame from the main loop),
//! the controllers it updates (`00595600`) and the sets they play
//! (`00597990`: location `00597a60`, dungeon `00598060`, battle
//! `00598550`, incidental `00598890`), over the two [`Decks`].
//!
//! Each frame, in order:
//!
//! 1. (The Pip-Boy radio being on stops everything: no radio here.)
//! 2. While `PlayMusic`'s track (or menu music) plays, nothing else runs.
//! 3. A region battle set already playing goes on until the fight ends.
//! 4. The audio marker holding the player (else the nearest, in 2D,
//!    chosen again about once a second) has its controller updated; if it
//!    plays, that's all.
//! 5. Otherwise the place's acoustic space's region (or an exterior
//!    square's region with incidental music) plays a battle set from its
//!    `RDSB`s in a fight, else its incidental set's phrases when the last
//!    deck is empty; the location and dungeon decks fade out over 1 s.
//!
//! What's guessed here (everything else is read from the code): which
//! actors count as aware of the player (the game's detection isn't run:
//! anyone fighting the player, and anyone alive within the sneak
//! distance), the acoustic space (the cell's own `XCAS`; placed acoustic
//! spaces aren't read), the random numbers (the game's Mersenne twister
//! is shared with everything else, so its picks can't be matched: the
//! rules for them are the game's), and a folder's file order (Windows'
//! listing, taken as sorted by name).

use std::collections::HashMap;
use std::sync::Arc;

use esm::{FormId, LoadOrder};

use super::decks::{kind, Decks, Requested, Volumes};
use super::{
    acoustic_region, acoustic_space, audio_markers_in_cell, audio_markers_in_world,
    exterior_sound_region, faction_modifier, AudioMarker, LocationController, MediaSet, MusicType,
    RegionSound, SetKind, LIST_NAMES,
};
use crate::weather::Climate;

/// Where music files are and how long they play.
pub trait MusicFiles {
    /// A music file's length in ms (relative to `Data`), `None` when it
    /// can't be opened (the game then refuses the request: "File '%s' does
    /// not exist!").
    fn duration_ms(&mut self, path: &str) -> Option<u32>;
    /// The `.mp3` files in a folder (relative to `Data`, ending in `\`), as
    /// paths relative to `Data`, in the order Windows lists them.
    fn folder(&mut self, folder: &str) -> Vec<String>;
}

/// Where the player is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerPlace {
    /// The interior, or the exterior square.
    pub cell: Option<FormId>,
    /// The worldspace, outdoors.
    pub world: Option<FormId>,
    /// The player's feet.
    pub position: [f32; 3],
}

/// Someone aware of the player (the game's list at player+0xD48: those who
/// detect the player, or lost them but still search), and whether they
/// fight the player.
#[derive(Debug, Clone, PartialEq)]
pub struct AwareActor {
    pub actor: FormId,
    pub hostile: bool,
    /// Their factions now.
    pub factions: Vec<FormId>,
}

/// What the manager is given each frame.
#[derive(Debug, Clone)]
pub struct MusicInputs<'a> {
    /// The audio clock, ms.
    pub now_ms: u64,
    /// `GameHour`.
    pub hour: f32,
    /// The sky's climate (indoors the last one, `DefaultClimate` when
    /// there was none), for day and night.
    pub climate: Option<&'a Climate>,
    pub player: Option<PlayerPlace>,
    /// Combat music wanted ([`super::CombatMusic`]).
    pub combat: bool,
    pub aware: &'a [AwareActor],
    /// The player is dead.
    pub dead: bool,
}

/// Something for the player to hear or be told.
#[derive(Debug, Clone, PartialEq)]
pub enum MusicEvent {
    /// A sound record to play at the music's volume (sets' intro and
    /// outro sounds and incidental phrases are played with the music
    /// category's flags, 0x901; and `MUSDeath` when the player dies).
    Sound(FormId),
    /// What the music did, in words.
    Note(String),
}

/// A set's state, kept on the `MSET` form itself in the game (so shared by
/// every controller using it): +0x2c, +0x30…+0x34.
#[derive(Debug, Clone, Default, PartialEq)]
struct SetState {
    /// When it last requested a track (+0x2c; not cleared by a reset).
    requested: Option<u64>,
    /// A battle is on: the intro played (+0x30).
    battle: bool,
    /// The combat flag last seen (+0x31).
    combat_seen: bool,
    /// The suspense flag last seen (+0x32).
    suspense: bool,
    /// The day flag last seen (+0x33).
    day: bool,
    /// The layer last played, or for a dungeon whether the player was
    /// inside (+0x34).
    layer: u8,
}

/// A controller's state (on the `ALOC` form in the game).
#[derive(Debug, Clone)]
struct ControllerState {
    record: Arc<LocationController>,
    /// The lists in their current order (picking moves the pick last).
    lists: [Vec<FormId>; 6],
    /// When it may start a set again (+0x24).
    next: u64,
    playing: bool,
    /// Someone aware of the player is hostile (+0x35).
    hostile_aware: bool,
    day: bool,
    /// Distance² over radius², 1000 outside (+0x3c).
    ratio: f32,
    hostiles: u32,
    members: u32,
    /// The faction state: 0 neutral, 1 enemy, 2 ally, 3 friend (+0x48).
    state: i32,
    set: Option<FormId>,
}

/// What a set's update sees of its controller.
#[derive(Debug, Clone, Copy)]
struct ControllerView {
    ratio: f32,
    day: bool,
    hostile_aware: bool,
    track_end: u32,
}

/// A music type linked: its tracks (a folder's, in reverse listing order)
/// and the last one picked (+0x30).
#[derive(Debug, Clone)]
struct LinkedMusic {
    record: MusicType,
    tracks: Vec<String>,
    last: usize,
}

/// The music manager with its decks: see the module notes.
#[derive(Debug, Clone)]
pub struct MusicDirector {
    pub decks: Decks,
    sets: HashMap<FormId, Option<Arc<MediaSet>>>,
    set_states: HashMap<FormId, SetState>,
    controllers: HashMap<FormId, Option<ControllerState>>,
    music_types: HashMap<FormId, Option<LinkedMusic>>,
    /// Each region's battle sets in their current order (picks move to the
    /// front).
    region_battles: HashMap<FormId, Vec<FormId>>,
    /// The controller updated last (`011dd374`).
    current: Option<FormId>,
    /// A region battle set playing (`011dd378`).
    region_battle: Option<FormId>,
    /// The least time on a layer or track runs out (`011dd37c`).
    gate: u64,
    /// Battle recovery, and the next incidental phrase (`011ca9fc`).
    timer: u64,
    place: Option<(Option<FormId>, Option<FormId>)>,
    markers: Vec<AudioMarker>,
    world_markers: HashMap<FormId, Vec<AudioMarker>>,
    /// The chosen marker (player+0x7DC), dropped about once a second.
    chosen: Option<usize>,
    cleared_at: u64,
    noted_marker: Option<Option<FormId>>,
    /// The square's region sound data (`011dd380`).
    sound_region: Option<FormId>,
    /// The acoustic space (`011dcfb4`).
    acoustic: Option<FormId>,
    was_dead: bool,
    rng: u64,
    events: Vec<MusicEvent>,
    /// The Pip-Boy radio is on (`011dd313`, set by `008325a0`): the
    /// manager's frame (`0082fb70`) returns at once.
    pub suspended: bool,
}

/// A float time in ms as the game rounds one (`fistp`, to nearest).
fn ms_round(v: f64) -> u64 {
    v.round().max(0.0) as u64
}

/// As the game truncates one (`_ftol`).
fn ms_trunc(v: f64) -> u32 {
    v.max(0.0) as u32
}

fn deck_name(i: usize) -> char {
    if i == 0 {
        'A'
    } else {
        'B'
    }
}

impl MusicDirector {
    /// A new game's music: nothing playing. `seed` starts the dice.
    pub fn new(volumes: Volumes, seed: u64) -> MusicDirector {
        MusicDirector {
            decks: Decks::new(volumes),
            sets: HashMap::new(),
            set_states: HashMap::new(),
            controllers: HashMap::new(),
            music_types: HashMap::new(),
            region_battles: HashMap::new(),
            current: None,
            region_battle: None,
            gate: 0,
            timer: 0,
            place: None,
            markers: Vec::new(),
            world_markers: HashMap::new(),
            chosen: None,
            cleared_at: 0,
            noted_marker: None,
            sound_region: None,
            acoustic: None,
            was_dead: false,
            rng: seed.max(1),
            events: Vec::new(),
            suspended: false,
        }
    }

    /// The radio's song (`008331c0` → `008300c0(7, data\sound\…, 500,
    /// no loop, forced, 0 dB, the item's start)`): onto a deck as type 7,
    /// in step with `sync`.
    pub fn radio_song(&mut self, path: &str, sync: u64, now: u64, files: &mut dyn MusicFiles) {
        self.request(
            kind::RADIO,
            Some(path),
            500,
            false,
            true,
            0.0,
            sync,
            now,
            files,
        );
    }

    /// Both decks cleared at once (`008304a0`: the radio turned on or off,
    /// an item's end).
    pub fn clear_decks(&mut self) {
        self.decks
            .request(kind::CLEAR, None, None, 0, false, true, 0.0, 0, 0);
        self.current = None;
    }

    /// What happened since last asked.
    pub fn take_events(&mut self) -> Vec<MusicEvent> {
        std::mem::take(&mut self.events)
    }

    fn note(&mut self, text: String) {
        self.events.push(MusicEvent::Note(text));
    }

    /// `random[lo, hi)` (`00944460`): lo + (a random number mod (hi − lo)).
    fn random(&mut self, lo: u64, hi: u64) -> u64 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng = x;
        if hi <= lo {
            return lo;
        }
        lo + (x >> 32) % (hi - lo)
    }

    fn media_set(&mut self, order: &LoadOrder, id: FormId) -> Option<Arc<MediaSet>> {
        self.sets
            .entry(id)
            .or_insert_with(|| MediaSet::load(order, id).map(Arc::new))
            .clone()
    }

    fn controller_loaded(&mut self, order: &LoadOrder, id: FormId) -> bool {
        self.controllers
            .entry(id)
            .or_insert_with(|| {
                LocationController::load(order, id).map(|r| ControllerState {
                    lists: r.lists.clone(),
                    record: Arc::new(r),
                    next: 0,
                    playing: false,
                    hostile_aware: false,
                    day: false,
                    ratio: 0.0,
                    hostiles: 0,
                    members: 0,
                    state: 0,
                    set: None,
                })
            })
            .is_some()
    }

    fn ctrl(&mut self, id: FormId) -> &mut ControllerState {
        self.controllers
            .get_mut(&id)
            .and_then(Option::as_mut)
            .expect("a loaded controller")
    }

    /// A set's reset (`00595cb0`): clears +0x28 and +0x30…+0x34, not the
    /// time it last asked for a track.
    fn reset_set(&mut self, id: FormId) {
        let st = self.set_states.entry(id).or_default();
        *st = SetState {
            requested: st.requested,
            ..SetState::default()
        };
    }

    /// A request to the decks, with the file's length and a note.
    #[allow(clippy::too_many_arguments)]
    fn request(
        &mut self,
        kind: u8,
        path: Option<&str>,
        fade_ms: u32,
        looping: bool,
        force: bool,
        decibels: f32,
        sync: u64,
        now: u64,
        files: &mut dyn MusicFiles,
    ) {
        let duration = path.and_then(|p| files.duration_ms(p));
        let r = self.decks.request(
            kind, path, duration, fade_ms, looping, force, decibels, sync, now,
        );
        let what = path.unwrap_or("nothing");
        match r {
            Requested::Opened(deck) => {
                let g = super::gain(kind, decibels, &self.decks.volumes);
                let from = if sync != 0 {
                    ", in step with the track before"
                } else {
                    ""
                };
                self.note(format!(
                    "Music ({} deck {}): {what} at {decibels} dB (×{g:.3}), fading in over {:.1} s{}{from}.",
                    kind::name(kind),
                    deck_name(deck),
                    f64::from(fade_ms) / 1000.0,
                    if looping { ", looping" } else { "" },
                ));
            }
            Requested::OutOfPriority => self.note(format!(
                "Music: {what} refused (\"Trying to open media out of priority\")."
            )),
            Requested::Missing => self.note(format!("Music: {what} can't be found.")),
            Requested::Nothing => {}
        }
    }

    /// The player moved into a cell (`0094cae0`, `00947d80`): the audio
    /// markers are listed again, the region sound data (`011dd380`) chosen
    /// again (outdoors), and the acoustic space taken from the cell.
    fn enter(&mut self, order: &LoadOrder, player: &PlayerPlace) {
        let key = (player.cell, player.world);
        if self.place == Some(key) {
            return;
        }
        self.place = Some(key);
        self.markers = match (player.world, player.cell) {
            (Some(w), _) => self
                .world_markers
                .entry(w)
                .or_insert_with(|| audio_markers_in_world(order, w))
                .clone(),
            (None, Some(c)) => audio_markers_in_cell(order, c),
            _ => Vec::new(),
        };
        self.chosen = None;
        self.sound_region = match (player.world, player.cell) {
            (Some(w), Some(c)) => exterior_sound_region(order, c, w, player.position),
            _ => None,
        };
        self.acoustic = player.cell.and_then(|c| acoustic_space(order, c));
    }

    /// The audio marker (`00969930`): walking the list, a nearer marker
    /// replaces the best so far while none holding the player has been
    /// found, and one holding the player replaces it the first time; so
    /// the first marker (in list order) holding the player, else the
    /// nearest. In 2D; "holding" is distance² < radius². Kept until the
    /// acoustic-space update drops it, after more than 1000 ms.
    fn choose_marker(&mut self, position: [f32; 3], now: u64) -> Option<usize> {
        if now.saturating_sub(self.cleared_at) >= 1001 {
            self.chosen = None;
            self.cleared_at = now;
        }
        if self.chosen.is_none() {
            let mut best = None;
            let mut best_d = f32::MAX;
            let mut found = false;
            for (i, m) in self.markers.iter().enumerate() {
                let d = m.distance2(position);
                let inside = d < m.radius * m.radius;
                if best_d <= d {
                    if inside && !found {
                        best = Some(i);
                        best_d = d;
                        found = inside;
                    }
                } else if !found {
                    best = Some(i);
                    best_d = d;
                    found = inside;
                }
            }
            self.chosen = best;
        }
        self.chosen
    }

    /// `PlayMusic` (`005c24b0` → `00830010`): the music type's track as
    /// kind 6, fading in over 1 s, looping if `ANAM` > 0, at −|`ANAM`| dB,
    /// not forced; then the decks' stops are cleared and the manager runs
    /// once. A music type with no file (`1NoMusic`) asks for nothing.
    pub fn play_music(
        &mut self,
        order: &LoadOrder,
        music: FormId,
        inputs: &MusicInputs,
        files: &mut dyn MusicFiles,
    ) {
        if let Some((record, track)) = self.pick_track(order, music, files) {
            self.request(
                kind::SCRIPT,
                track.as_deref(),
                1000,
                record.loops(),
                false,
                record.volume_db(),
                0,
                inputs.now_ms,
                files,
            );
        }
        self.decks.clear_stops();
        self.update(order, inputs, files);
    }

    /// A music type's track (`00591a40`): no list, its file; one, that;
    /// otherwise at random, never the last one again (r = random[0, n); if
    /// r is the last pick and that isn't 0, 0; if both are 0, 1).
    fn pick_track(
        &mut self,
        order: &LoadOrder,
        music: FormId,
        files: &mut dyn MusicFiles,
    ) -> Option<(MusicType, Option<String>)> {
        if !self.music_types.contains_key(&music) {
            let linked = MusicType::load(order, music).map(|record| {
                // The link (`00591410`): a folder's MP3s, each put at the
                // front; the last pick starts at random[0, n − 1).
                let mut tracks = match &record.file {
                    Some(f) if record.is_folder() => files.folder(f),
                    _ => Vec::new(),
                };
                tracks.reverse();
                LinkedMusic {
                    record,
                    tracks,
                    last: 0,
                }
            });
            let n = linked.as_ref().map_or(0, |l| l.tracks.len() as u64);
            let last = self.random(0, n.saturating_sub(1)) as usize;
            let linked = linked.map(|l| LinkedMusic { last, ..l });
            self.music_types.insert(music, linked);
        }
        let n = self
            .music_types
            .get(&music)?
            .as_ref()
            .map_or(0, |l| l.tracks.len());
        let drawn = if n > 1 {
            self.random(0, n as u64) as usize
        } else {
            0
        };
        let linked = self.music_types.get_mut(&music)?.as_mut()?;
        let track = match n {
            0 => linked.record.file.clone(),
            1 => Some(linked.tracks[0].clone()),
            _ => {
                let mut r = drawn;
                if linked.last != 0 && linked.last == r {
                    r = 0;
                    linked.last = 0;
                } else if linked.last == 0 && r == 0 {
                    r = 1;
                    linked.last = 1;
                } else {
                    linked.last = r;
                }
                Some(linked.tracks[r].clone())
            }
        };
        Some((linked.record.clone(), track))
    }

    /// Picks from a controller list (`00595560`): the first entry, then
    /// each later one replaces it one time in four (random[0, 4) = 0); the
    /// pick moves to the end of the list.
    fn pick_from_list(&mut self, ctrl: FormId, list: usize) -> Option<FormId> {
        let entries = self.ctrl(ctrl).lists[list].clone();
        let mut pick = None;
        for &f in &entries {
            let r = self.random(0, 4);
            if r == 0 || pick.is_none() {
                pick = Some(f);
            }
        }
        let f = pick?;
        let l = &mut self.ctrl(ctrl).lists[list];
        if let Some(i) = l.iter().position(|&x| x == f) {
            l.remove(i);
        }
        l.push(f);
        Some(f)
    }

    /// Picks a region battle set (`004f4f90`): as a controller's list but
    /// one time in three, and the pick moves to the front.
    fn pick_region_battle(&mut self, data: &RegionSound) -> Option<FormId> {
        let entries = self
            .region_battles
            .entry(data.region)
            .or_insert_with(|| data.battle.clone())
            .clone();
        let mut pick = None;
        for &f in &entries {
            let r = self.random(0, 3);
            if r == 0 || pick.is_none() {
                pick = Some(f);
            }
        }
        let f = pick?;
        let l = self.region_battles.get_mut(&data.region)?;
        if let Some(i) = l.iter().position(|&x| x == f) {
            l.remove(i);
        }
        l.insert(0, f);
        Some(f)
    }

    /// One frame of the music manager (`0082fb70`); see the module notes.
    /// The decks then play on with [`Self::tick`].
    pub fn update(&mut self, order: &LoadOrder, inputs: &MusicInputs, files: &mut dyn MusicFiles) {
        let now = inputs.now_ms;
        if self.suspended {
            return;
        }
        // The player's death (`0089d900`): every deck fades out over 1 s,
        // and `MUSDeath` plays.
        if inputs.dead && !self.was_dead {
            self.decks.fade_out(1000, kind::EMPTY, now);
            if let Some(s) = order.form_by_editor_id("MUSDeath") {
                self.events.push(MusicEvent::Sound(s));
            }
        }
        self.was_dead = inputs.dead;
        if self.decks.is_playing(kind::SCRIPT, true) || self.decks.is_playing(kind::MENU, true) {
            self.current = None;
            return;
        }
        let recent_kind = self.decks.most_recent().kind;
        let combat = inputs.combat;
        if let Some(p) = &inputs.player {
            self.enter(order, p);
        }
        if let Some(set) = self.region_battle {
            if self.update_set(order, set, None, now, combat, inputs, files) {
                return;
            }
        }
        self.region_battle = None;
        let Some(player) = inputs.player else {
            return;
        };
        let marker = self
            .choose_marker(player.position, now)
            .map(|i| self.markers[i].clone());
        let noted = marker.as_ref().map(|m| m.reference);
        if self.noted_marker != Some(noted) {
            self.noted_marker = Some(noted);
            if let Some(m) = &marker {
                let d = m.distance2(player.position).sqrt();
                self.note(format!(
                    "Music: audio marker {} ({:.0} units away, radius {:.0}), controller {}.",
                    m.reference,
                    d,
                    m.radius,
                    m.controller.map_or("none".into(), |c| {
                        order
                            .get(c)
                            .and_then(|r| r.editor_id().ok().flatten())
                            .unwrap_or_else(|| c.to_string())
                    })
                ));
            }
        }
        if let Some(m) = marker {
            if let Some(ctrl) = m.controller.filter(|&c| self.controller_loaded(order, c)) {
                if self.update_controller(order, ctrl, &m, inputs, files) {
                    if inputs.dead {
                        self.decks.fade_out(1000, kind::EMPTY, now);
                    } else if self.decks.is_playing(kind::EMPTY, true) && self.current.is_none() {
                        // (Can't happen: a controller is current here.)
                        for k in [kind::BATTLE, kind::UNUSED, kind::LOCATION, kind::DUNGEON] {
                            self.decks.fade_out(5000, k, now);
                        }
                    }
                    return;
                }
            }
        }
        self.current = None;
        let Some(region) = self.acoustic.and_then(|s| acoustic_region(order, s)) else {
            return;
        };
        if self.decks.is_playing(kind::LOCATION, true) {
            self.decks.fade_out(1000, kind::LOCATION, now);
        }
        if self.decks.is_playing(kind::DUNGEON, true) {
            self.decks.fade_out(1000, kind::DUNGEON, now);
        }
        let data = match self.sound_region {
            Some(r) => RegionSound::load(order, r),
            None => RegionSound::load(order, region),
        };
        let Some(data) = data else { return };
        if combat && self.region_battle.is_none() {
            self.region_battle = self.pick_region_battle(&data);
            if let Some(s) = self.region_battle {
                self.reset_set(s);
            }
        }
        let battling = match self.region_battle {
            Some(s) => self.update_set(order, s, None, now, combat, inputs, files),
            None => false,
        };
        if !battling {
            self.region_battle = None;
            if let Some(incidental) = data.incidental {
                if recent_kind == kind::EMPTY {
                    self.update_set(order, incidental, None, now, false, inputs, files);
                }
            }
        }
    }

    /// The decks play on to `now` (their threads' ticks).
    pub fn tick(&mut self, now: u64) {
        self.decks.tick(now);
    }

    /// A controller's update (`00595600`): see the findings, §2.4.
    fn update_controller(
        &mut self,
        order: &LoadOrder,
        id: FormId,
        marker: &AudioMarker,
        inputs: &MusicInputs,
        files: &mut dyn MusicFiles,
    ) -> bool {
        let now = inputs.now_ms;
        let combat = inputs.combat;
        let Some(player) = inputs.player else {
            return false;
        };
        if self.current != Some(id) {
            if let Some(s) = self.ctrl(id).set.take() {
                self.reset_set(s);
            }
        }
        self.current = Some(id);
        if !self.ctrl(id).playing {
            if let Some(s) = self.ctrl(id).set.take() {
                self.reset_set(s);
            }
        }
        let record = self.ctrl(id).record.clone();
        let mut state = record.faction.map_or(0, |f| {
            faction_modifier(order, f, crate::factions::PLAYER_FACTION)
        });
        let d2 = marker.distance2(player.position);
        let r2 = marker.radius * marker.radius;
        let ratio = if d2 <= r2 { d2 / r2 } else { 1000.0 };
        let mut members = 0;
        let mut hostiles = 0;
        let mut faction_seen = false;
        for a in inputs.aware {
            if let Some(f) = record.faction {
                if a.factions.contains(&f) {
                    members += 1;
                    faction_seen = true;
                    if a.hostile {
                        state = 1;
                    }
                }
            }
            if a.hostile {
                hostiles += 1;
            }
        }
        let day = record.is_day(inputs.climate, inputs.hour);
        let default_list = record.default_list() as i32;
        let c = self.ctrl(id);
        let (old_members, old_day, old_state) = (c.members, c.day, c.state);
        c.members = members;
        c.hostiles = hostiles;
        c.hostile_aware = hostiles != 0;
        c.day = day;
        c.state = state;
        c.ratio = ratio;
        if !(c.ratio >= 100.0 || c.next <= now || c.playing) {
            return false;
        }
        c.playing = false;
        if let Some(s) = c.set {
            let playing = self.update_set(order, s, Some(id), now, combat, inputs, files);
            self.ctrl(id).playing = playing;
        }
        if old_day != day
            || old_state != state
            || (state != default_list && (old_members != 0) != (members != 0))
        {
            if let Some(s) = self.ctrl(id).set.take() {
                self.reset_set(s);
            }
        }
        if !self.ctrl(id).playing && ratio < 100.0 {
            if let Some(s) = self.ctrl(id).set.take() {
                self.reset_set(s);
            }
            let list = if faction_seen {
                match state {
                    0..=3 => Some(state as usize),
                    _ => None,
                }
            } else {
                record.list_for(default_list)
            };
            if let Some(list) = list {
                let picked = self.pick_from_list(id, list);
                self.ctrl(id).set = picked;
                if let Some(p) = picked {
                    let name = self
                        .media_set(order, p)
                        .map_or(p.to_string(), |s| s.label());
                    self.note(format!(
                        "Music: {} picks {name} from its {} list.",
                        record.label(),
                        LIST_NAMES[list]
                    ));
                }
            }
        }
        let set_code = self
            .ctrl(id)
            .set
            .and_then(|s| self.media_set(order, s))
            .map(|s| s.code);
        let ratio_now = self.ctrl(id).ratio;
        if !matches!(set_code, Some(0) | Some(2)) && ratio_now < 100.0 && combat {
            if let Some(s) = self.ctrl(id).set.take() {
                self.reset_set(s);
            }
            if self.region_battle.is_none() {
                let picked = self.pick_from_list(id, 5);
                self.ctrl(id).set = picked;
                if let Some(p) = picked {
                    let name = self
                        .media_set(order, p)
                        .map_or(p.to_string(), |s| s.label());
                    self.note(format!(
                        "Music: {} picks {name} from its battle list.",
                        record.label()
                    ));
                }
            }
        }
        if !self.ctrl(id).playing {
            if let Some(s) = self.ctrl(id).set {
                let playing = self.update_set(order, s, Some(id), now, combat, inputs, files);
                self.ctrl(id).playing = playing;
            }
        }
        let playing = self.ctrl(id).playing;
        if playing {
            self.ctrl(id).next = ms_round(f64::from(record.delay) * 1000.0 + now as f64);
        } else if self.ctrl(id).ratio > 1.0 {
            self.current = None;
        }
        playing
    }

    /// A set's update (`00597990`): with a controller, battle, location or
    /// dungeon; without (region music), battle, or incidental once its
    /// time comes (which answers "not playing").
    #[allow(clippy::too_many_arguments)]
    fn update_set(
        &mut self,
        order: &LoadOrder,
        id: FormId,
        ctrl: Option<FormId>,
        now: u64,
        combat: bool,
        inputs: &MusicInputs,
        files: &mut dyn MusicFiles,
    ) -> bool {
        let Some(set) = self.media_set(order, id) else {
            return false;
        };
        let view = ctrl.map(|c| {
            let c = self.ctrl(c);
            ControllerView {
                ratio: c.ratio,
                day: c.day,
                hostile_aware: c.hostile_aware,
                track_end: c.record.track_end(),
            }
        });
        match (view, set.kind) {
            (None, Some(SetKind::Incidental)) => {
                if now < self.timer {
                    return true;
                }
                self.incidental(&set, now, inputs);
                false
            }
            (_, Some(SetKind::Battle)) => self.battle(&set, now, combat, files),
            (Some(v), Some(SetKind::Location)) => self.location(&set, v, now, combat, files),
            (Some(v), Some(SetKind::Dungeon)) => self.dungeon(&set, v, now, combat, files),
            _ => false,
        }
    }

    /// A location set (`00597a60`, deck kind 2): in a fight, "not playing"
    /// at once (the controller turns to a battle set). Its track restarts
    /// `FNAM` s before its end (when the controller's `NAM1` bits 4–5 say
    /// so; else the set is over). The layer by distance; a new layer (or
    /// day flag, or nothing playing) starts once the least time on a layer
    /// is up, in step with the track before, cross-fading over `FNAM` s.
    fn location(
        &mut self,
        set: &MediaSet,
        ctrl: ControllerView,
        now: u64,
        combat: bool,
        files: &mut dyn MusicFiles,
    ) -> bool {
        let playing = self.decks.is_playing(kind::LOCATION, false);
        if combat {
            return false;
        }
        let fade = f64::from(set.fnam) * 1000.0;
        let duration = self.decks.duration(kind::LOCATION);
        let st = self.set_states.entry(set.form_id).or_default().clone();
        let mut restart = false;
        if let Some(requested) = st.requested {
            if duration != 0 && (requested + u64::from(duration)) as f64 - fade < now as f64 {
                self.decks.fade_out(ms_trunc(fade), kind::LOCATION, now);
                self.set_states.entry(set.form_id).or_default().requested = Some(now);
                if ctrl.track_end != 0 {
                    return false;
                }
                restart = true;
            }
        }
        let layer = set.layer_at(ctrl.ratio * 100.0, ctrl.day);
        let changed = !playing || st.combat_seen || st.day != ctrl.day || st.layer != layer;
        let mut result = true;
        if (changed && self.gate < now) || restart {
            match set.layer_track(layer, ctrl.day) {
                None => {
                    if playing {
                        self.decks.fade_out(ms_trunc(fade), kind::LOCATION, now);
                    }
                    result = false;
                }
                Some(track) => {
                    self.gate = ms_round(f64::from(set.dnam) * 1000.0 + now as f64);
                    let position = self.decks.position(kind::LOCATION);
                    let sync = if position != 0 && !restart {
                        now.saturating_sub(position)
                    } else {
                        0
                    };
                    let file = track.file.clone();
                    self.request(
                        kind::LOCATION,
                        file.as_deref(),
                        ms_round(fade) as u32,
                        ctrl.track_end == 0,
                        true,
                        track.decibels,
                        sync,
                        now,
                        files,
                    );
                    self.decks.clear_stops();
                    self.set_states.entry(set.form_id).or_default().requested = Some(now);
                }
            }
            let st = self.set_states.entry(set.form_id).or_default();
            st.combat_seen = false;
            st.day = ctrl.day;
            st.layer = layer;
        }
        result
    }

    /// A dungeon set (`00598060`, deck kind 3): its battle track in a
    /// fight (the intro sound first, a 1 s fade), else suspense while
    /// someone aware of the player is hostile, else explore (the outro
    /// after a fight; `FNAM` fades; the least time `DNAM`); it starts again
    /// when the fight, suspense or being inside changes. Out of the radius
    /// and past the recovery time, it fades out. Its track-end check asks
    /// the length of a **location** deck (as the game's code does), so it
    /// never fires: dungeon tracks loop.
    fn dungeon(
        &mut self,
        set: &MediaSet,
        ctrl: ControllerView,
        now: u64,
        combat: bool,
        files: &mut dyn MusicFiles,
    ) -> bool {
        let playing = self.decks.is_playing(kind::DUNGEON, true);
        let inside = ctrl.ratio < 100.0;
        let st = self.set_states.entry(set.form_id).or_default().clone();
        if !st.combat_seen && combat {
            self.gate = 0;
        }
        let fade = f64::from(set.fnam) * 1000.0;
        let duration = self.decks.duration(kind::LOCATION);
        let mut restart = false;
        if let Some(requested) = st.requested {
            if duration != 0 && (requested + u64::from(duration)) as f64 - fade < now as f64 {
                self.decks.fade_out(ms_trunc(fade), kind::LOCATION, now);
                self.set_states.entry(set.form_id).or_default().requested = Some(now);
                if ctrl.track_end != 0 {
                    return false;
                }
                restart = true;
            }
        }
        let suspense = ctrl.hostile_aware;
        let changed = !playing
            || st.combat_seen != combat
            || st.suspense != suspense
            || (st.layer != u8::from(inside) && st.layer == 0);
        if (changed && self.gate < now) || restart {
            let track = if !combat {
                if st.battle {
                    if let Some(outro) = set.inam {
                        self.events.push(MusicEvent::Sound(outro));
                    }
                }
                if suspense {
                    2
                } else {
                    1
                }
            } else {
                if !st.battle {
                    if let Some(intro) = set.hnam {
                        self.events.push(MusicEvent::Sound(intro));
                    }
                }
                0
            };
            self.set_states.entry(set.form_id).or_default().battle = combat;
            self.gate = if combat {
                now
            } else {
                ms_round(now as f64 + f64::from(set.dnam) * 1000.0)
            };
            let position = self.decks.position(kind::DUNGEON);
            let sync = if position != 0 && !restart {
                now.saturating_sub(position)
            } else {
                0
            };
            let fade_ms = if track == 0 {
                1000
            } else {
                ms_round(fade) as u32
            };
            let layer = set.layers[track].clone();
            self.request(
                kind::DUNGEON,
                layer.file.as_deref(),
                fade_ms,
                true,
                true,
                layer.decibels,
                sync,
                now,
                files,
            );
            self.decks.clear_stops();
            let st = self.set_states.entry(set.form_id).or_default();
            st.combat_seen = combat;
            st.suspense = suspense;
            st.layer = u8::from(inside);
            true
        } else if !inside && self.timer < now && !st.battle {
            if playing {
                self.decks.fade_out(ms_trunc(fade), kind::DUNGEON, now);
            }
            false
        } else {
            true
        }
    }

    /// A battle set (`00598550`, deck kind 4). A fight starting: the intro
    /// sound, the loop (track 0 at `NAM8` dB) fading in over `ENAM` s, and
    /// both decks held until `DNAM` **ms** have passed (1 in the data: the
    /// next frame); the loop is asked for again 1 s before its end,
    /// cross-fading into itself. The fight over: the loop fades out over
    /// `ENAM` s, the outro plays, and `FNAM` s of recovery begin (the
    /// current controller waits as long). Answers whether there's a fight.
    fn battle(
        &mut self,
        set: &MediaSet,
        now: u64,
        combat: bool,
        files: &mut dyn MusicFiles,
    ) -> bool {
        let st = self.set_states.entry(set.form_id).or_default().clone();
        let enam = f64::from(set.enam) * 1000.0;
        let track = set.layers[0].clone();
        if !combat {
            if self.decks.is_playing(kind::BATTLE, false) && st.battle {
                self.decks.fade_out(ms_trunc(enam), kind::BATTLE, now);
                if let Some(outro) = set.inam {
                    self.events.push(MusicEvent::Sound(outro));
                }
                self.timer = ms_round(f64::from(set.fnam) * 1000.0 + now as f64);
                if let Some(c) = self.current {
                    let timer = self.timer;
                    if let Some(Some(c)) = self.controllers.get_mut(&c) {
                        c.next = timer;
                    }
                }
                self.set_states.entry(set.form_id).or_default().battle = false;
            }
            return false;
        }
        if !self.decks.is_playing(kind::BATTLE, false) && !st.battle {
            if let Some(intro) = set.hnam {
                self.events.push(MusicEvent::Sound(intro));
            }
            self.timer = now.saturating_sub(1);
            self.set_states.entry(set.form_id).or_default().battle = true;
            self.request(
                kind::BATTLE,
                track.file.as_deref(),
                ms_round(enam) as u32,
                true,
                true,
                track.decibels,
                0,
                now,
                files,
            );
            self.decks.clear_stops();
            self.decks.hold(true);
        }
        if (self.timer as f64 + f64::from(set.dnam)) < now as f64 {
            self.decks.hold(false);
            self.set_states.entry(set.form_id).or_default().battle = true;
        }
        let duration = u64::from(self.decks.duration(kind::BATTLE));
        if duration != 0 && (self.timer + duration).saturating_sub(1000) < now {
            self.request(
                kind::BATTLE,
                track.file.as_deref(),
                ms_round(enam) as u32,
                true,
                true,
                track.decibels,
                0,
                now,
                files,
            );
            self.decks.clear_stops();
            self.timer = now;
        }
        true
    }

    /// An incidental set (`00598890`, region music only): once its time
    /// comes, by day (from the climate's sunrise begin to its sunset end,
    /// raw times) the day sound (`HNAM`) and the next in random[`DNAM`,
    /// `FNAM`) s, by night the night sound (`INAM`) and random[`ENAM`,
    /// `GNAM`) s.
    fn incidental(&mut self, set: &MediaSet, now: u64, inputs: &MusicInputs) {
        if now <= self.timer {
            return;
        }
        let hour = inputs.hour;
        let day = inputs
            .climate
            .map_or(true, |c| c.sunrise.0 <= hour && hour <= c.sunset.1);
        let (sound, lo, hi) = if day {
            (set.hnam, set.dnam, set.fnam)
        } else {
            (set.inam, set.enam, set.gnam)
        };
        if let Some(s) = sound {
            self.events.push(MusicEvent::Sound(s));
        }
        let lo = u64::from(ms_trunc(f64::from(lo) * 1000.0));
        let hi = u64::from(ms_trunc(f64::from(hi) * 1000.0));
        self.timer = self.random(lo, hi) + now;
    }

    /// The controller playing now, if any.
    pub fn current_controller(&self) -> Option<FormId> {
        self.current
    }

    /// The set a controller plays, if any.
    pub fn controller_set(&self, controller: FormId) -> Option<FormId> {
        self.controllers
            .get(&controller)
            .and_then(Option::as_ref)
            .and_then(|c| c.set)
    }

    /// The region battle set playing, if any.
    pub fn region_battle(&self) -> Option<FormId> {
        self.region_battle
    }

    /// The audio markers listed for the player's place, in the game's
    /// order, and which one is chosen.
    pub fn markers(&self) -> (&[AudioMarker], Option<usize>) {
        (&self.markers, self.chosen)
    }

    /// The region sound data an exterior square gives (`011dd380`), and the
    /// acoustic space.
    pub fn regions(&self) -> (Option<FormId>, Option<FormId>) {
        (self.sound_region, self.acoustic)
    }

    /// The decks in words.
    pub fn describe_decks(&self) -> Vec<String> {
        self.decks
            .decks
            .iter()
            .enumerate()
            .map(|(i, d)| {
                if !d.is_live() {
                    return format!("deck {}: empty", deck_name(i));
                }
                let mut state = Vec::new();
                if d.stopping() {
                    state.push("fading out");
                }
                if d.looping() {
                    state.push("looping");
                }
                if !d.running() {
                    state.push("held");
                }
                format!(
                    "deck {}: {} {} at {:.1} s of {:.1} s, {} dB, volume {:.3}{}",
                    deck_name(i),
                    kind::name(d.kind),
                    d.path.as_deref().unwrap_or("?"),
                    d.position_ms / 1000.0,
                    f64::from(d.duration_ms) / 1000.0,
                    d.decibels,
                    d.volume,
                    if state.is_empty() {
                        String::new()
                    } else {
                        format!(" ({})", state.join(", "))
                    }
                )
            })
            .collect()
    }
}

/// Who's aware of the player, for the controllers' faction rule and the
/// dungeon sets' suspense: anyone fighting the player (hostile), and anyone
/// else alive and enabled within `fSneakMaxDistance` (2500; ×
/// `fSneakExteriorDistanceMult`, 2, outdoors), in 3D. **A guess**: the game
/// lists those its detection code says detect the player (or search for
/// them), which also weighs sight, light, sound and sneaking
/// (`world::detection`).
pub fn aware_actors(
    order: &LoadOrder,
    state: &crate::scripting::GameState,
    player: [f32; 3],
    outdoors: bool,
    people: &[(FormId, [f32; 3])],
) -> Vec<AwareActor> {
    use crate::scripting::game_setting;
    let reach = game_setting(order, "fSneakMaxDistance").unwrap_or(1500.0)
        * if outdoors {
            game_setting(order, "fSneakExteriorDistanceMult").unwrap_or(2.0)
        } else {
            1.0
        };
    let player_ref = crate::dialogue::PLAYER_REF;
    people
        .iter()
        .filter(|(who, _)| *who != player_ref)
        .filter(|(who, _)| !state.dead.contains(who) && state.disabled.get(who) != Some(&true))
        .filter_map(|&(who, at)| {
            let hostile = state.combat.get(&who) == Some(&player_ref);
            let d = (0..3)
                .map(|i| (at[i] - player[i]).powi(2))
                .sum::<f32>()
                .sqrt();
            (hostile || d <= reach).then(|| AwareActor {
                actor: who,
                hostile,
                factions: crate::factions::factions_of(order, state, who),
            })
        })
        .collect()
}
