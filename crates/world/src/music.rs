//! Music, as `FalloutNV.exe` plays it: read from its code (see
//! `%USERPROFILE%\nv-re\findings\music.md`; the functions named below are
//! the game's).
//!
//! New Vegas's music doesn't come from a cell's music type: `XCMO` is read
//! into a field nothing reads. It comes from:
//!
//! - **audio markers**: placed `AudioMarker` statics (base `00000023`,
//!   `MMRK`) that name a media location controller (`ALOC`, `CNAM`) and
//!   have a radius (`XRDS`, 5000 when 0). The controller picks a media set
//!   (`MSET`) from one of its lists, and the set plays MP3s from
//!   `Data\Music`: a location set up to three layers by how near the
//!   marker the player stands, a dungeon set its explore, suspense and
//!   battle tracks, a battle set its loop;
//! - a region's sound data (`REGN` data type 7: `RDSI` an incidental set,
//!   `RDSB` battle sets) when no controller plays;
//! - scripts: `PlayMusic` (`MUSC`), which holds the rest off until its
//!   track ends.
//!
//! [`MusicDirector`] runs the game's music manager (`0082fb70`) over its
//! two decks ([`Decks`], the game's "FalloutAudioMedia"); a player (the
//! viewer) follows the decks: which file, from where, how loud.
//! [`CombatMusic`] is the combat manager's "combat music wanted" answer.

mod combat;
mod decks;
mod director;

pub use combat::{player_strength, CombatGroup, CombatMusic, CombatMusicSettings};
pub use decks::{flags, gain, kind, Deck, Decks, Requested, Volumes};
pub use director::{
    aware_actors, AwareActor, MusicDirector, MusicEvent, MusicFiles, MusicInputs, PlayerPlace,
};

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};

const MUSC: FourCC = FourCC::new(b"MUSC");
const MSET: FourCC = FourCC::new(b"MSET");
const ALOC: FourCC = FourCC::new(b"ALOC");
const REGN: FourCC = FourCC::new(b"REGN");
const ASPC: FourCC = FourCC::new(b"ASPC");
const FACT: FourCC = FourCC::new(b"FACT");
const MMRK: FourCC = FourCC::new(b"MMRK");
const XRDS: FourCC = FourCC::new(b"XRDS");
const XCAS: FourCC = FourCC::new(b"XCAS");
const XCLR: FourCC = FourCC::new(b"XCLR");
const RDAT: FourCC = FourCC::new(b"RDAT");
const XNAM: FourCC = FourCC::new(b"XNAM");

/// The audio marker static every audio marker is placed from (a fixed form
/// in every game; the code keeps it in `011ca228`).
pub const AUDIO_MARKER: FormId = FormId(0x23);

/// An audio marker's radius when its `XRDS` is 0 or missing (`00568cb0`,
/// the float at `01030020`).
pub const DEFAULT_MARKER_RADIUS: f32 = 5000.0;

/// Music files and folders are named relative to `Data\Music`; the game
/// prepends this (`"Data\\Music\\"`). Paths here are relative to `Data`,
/// lower case, as `assets` names files.
pub const MUSIC_FOLDER: &str = "music\\";

fn music_path(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| format!("{MUSIC_FOLDER}{}", value.to_ascii_lowercase()))
}

fn form(rr: &esm::RecordRef<'_>, data: &[u8]) -> Option<FormId> {
    (data.len() >= 4)
        .then(|| rr.plugin.to_global(FormId(le_u32(data, 0))))
        .filter(|f| f.0 != 0)
}

fn float(data: &[u8]) -> Option<f32> {
    (data.len() >= 4).then(|| le_f32(data, 0))
}

/// A music type (`MUSC`, `BGSMusicType`; loader `00591550`, link
/// `00591410`): what `PlayMusic` plays.
#[derive(Debug, Clone, PartialEq)]
pub struct MusicType {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `FNAM`, relative to `Data` (`music\scr\mus_scr_goodspringsstinger.
    /// mp3`), or a folder ending in `\` whose MP3s are picked from. `None`
    /// when the record has none or it's 2 characters or shorter (the link
    /// skips those): `1NoMusic`, which then plays nothing.
    pub file: Option<String>,
    /// `ANAM` (default −6, `0103395c`): decibels, and its sign says whether
    /// the track loops (`PlayMusic`, `00830010`): above 0 loops; it plays at
    /// −|ANAM| dB.
    pub decibels: f32,
}

impl MusicType {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<MusicType> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == MUSC)?;
        let record = rr.record().ok()?;
        let name = record
            .get(FourCC::new(b"FNAM"))
            .map(|s| s.zstring())
            .unwrap_or_default();
        Some(MusicType {
            form_id: id,
            editor_id: record.editor_id(),
            file: (name.trim().len() > 2).then(|| music_path(&name)).flatten(),
            decibels: record
                .get(FourCC::new(b"ANAM"))
                .and_then(|s| float(&s.data))
                .unwrap_or(-6.0),
        })
    }

    /// Whether `file` names a folder to pick tracks from.
    pub fn is_folder(&self) -> bool {
        self.file.as_ref().is_some_and(|f| f.ends_with('\\'))
    }

    /// `PlayMusic` loops the track when `ANAM` is above 0 (`00830050`).
    pub fn loops(&self) -> bool {
        self.decibels > 0.0
    }

    /// The volume `PlayMusic` plays it at: −|`ANAM`| dB (`00830080`).
    pub fn volume_db(&self) -> f32 {
        if self.decibels <= 0.0 {
            self.decibels
        } else {
            -self.decibels
        }
    }
}

/// What a media set does (`MSET` `NAM1`, at +0x44).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetKind {
    /// 0: a looping battle track with an intro and an outro sound.
    Battle,
    /// 1: up to three layers by day and three by night, by distance.
    Location,
    /// 2: battle, explore and suspense tracks.
    Dungeon,
    /// 3: short phrases (sound records) every so often, by day and night.
    Incidental,
}

impl SetKind {
    pub fn from_code(code: i32) -> Option<SetKind> {
        match code {
            0 => Some(SetKind::Battle),
            1 => Some(SetKind::Location),
            2 => Some(SetKind::Dungeon),
            3 => Some(SetKind::Incidental),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            SetKind::Battle => "battle",
            SetKind::Location => "location",
            SetKind::Dungeon => "dungeon",
            SetKind::Incidental => "incidental",
        }
    }
}

/// One of a media set's six tracks (16 bytes at +0x48 + 16 × n): a file,
/// its loudness and, for a location set, its boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct MediaLayer {
    /// Relative to `Data`; `None` when the set has no such track.
    pub file: Option<String>,
    /// Decibels (`NAM8` `NAM9` `NAM0` `ANAM` `BNAM` `CNAM` in track order;
    /// default −12, `01034424`).
    pub decibels: f32,
    /// A location layer's boundary, a percentage of the marker's radius
    /// **squared** (`JNAM`…`ONAM`; default 100).
    pub boundary: f32,
}

/// A media set (`MSET`, `MediaSet`; loader `00598bc0`). What its fields
/// mean depends on its kind:
///
/// | | location | dungeon | battle | incidental |
/// |---|---|---|---|---|
/// | tracks 0–2 | day layers 1–3 | battle, explore, suspense | 0: the loop | — |
/// | tracks 3–5 | night layers 1–3 | — | — | — |
/// | `DNAM` | least time on a layer (s) | least time on a track | the pause after the intro (**ms**) | least gap by day (s) |
/// | `ENAM` | — | — | the loop's fades (s) | least gap by night |
/// | `FNAM` | cross-fade, and lead before a track's end (s) | cross-fade | recovery after a fight (s) | most gap by day |
/// | `GNAM` | — | — | — | most gap by night |
/// | `HNAM` | — | intro sound | intro sound | day sound |
/// | `INAM` | — | outro sound | outro sound | night sound |
#[derive(Debug, Clone, PartialEq)]
pub struct MediaSet {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `NAM1` (default −1: none of the kinds).
    pub code: i32,
    pub kind: Option<SetKind>,
    pub layers: [MediaLayer; 6],
    /// `PNAM`: which layers a location set uses (0x01–0x04 day 1–3,
    /// 0x08–0x20 night 1–3).
    pub enabled: u8,
    pub dnam: f32,
    pub enam: f32,
    pub fnam: f32,
    pub gnam: f32,
    pub hnam: Option<FormId>,
    pub inam: Option<FormId>,
}

impl MediaSet {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<MediaSet> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == MSET)?;
        let record = rr.record().ok()?;
        let get = |kind: &[u8; 4]| record.get(FourCC::new(kind));
        let number = |kind: &[u8; 4]| get(kind).and_then(|s| float(&s.data));
        let paths = [b"NAM2", b"NAM3", b"NAM4", b"NAM5", b"NAM6", b"NAM7"];
        let decibels = [b"NAM8", b"NAM9", b"NAM0", b"ANAM", b"BNAM", b"CNAM"];
        let boundaries = [b"JNAM", b"KNAM", b"LNAM", b"MNAM", b"NNAM", b"ONAM"];
        let layers = [0, 1, 2, 3, 4, 5].map(|i| MediaLayer {
            file: get(paths[i]).and_then(|s| music_path(&s.zstring())),
            decibels: number(decibels[i]).unwrap_or(-12.0),
            boundary: number(boundaries[i]).unwrap_or(100.0),
        });
        let code = get(b"NAM1")
            .filter(|s| s.data.len() >= 4)
            .map_or(-1, |s| le_u32(&s.data, 0) as i32);
        Some(MediaSet {
            form_id: id,
            editor_id: record.editor_id(),
            code,
            kind: SetKind::from_code(code),
            layers,
            enabled: get(b"PNAM")
                .and_then(|s| s.data.first().copied())
                .unwrap_or(0),
            dnam: number(b"DNAM").unwrap_or(0.0),
            enam: number(b"ENAM").unwrap_or(0.0),
            fnam: number(b"FNAM").unwrap_or(0.0),
            gnam: number(b"GNAM").unwrap_or(0.0),
            hnam: get(b"HNAM").and_then(|s| form(&rr, &s.data)),
            inam: get(b"INAM").and_then(|s| form(&rr, &s.data)),
        })
    }

    /// A location set's layer for a distance (`00597a60`): `percent` is
    /// the distance² as a percentage of the radius² (≥ 100 outside). The
    /// innermost enabled layer whose boundary lies above it: layer 1 if
    /// enabled and `percent < B1 < 1000`, then layer 2 if enabled and
    /// `percent < B2 <` the last boundary taken, then layer 3 likewise. 0:
    /// none (outside every layer). Layers 1–3 are tracks 0–2 by day, 3–5 by
    /// night.
    pub fn layer_at(&self, percent: f32, day: bool) -> u8 {
        let first = if day { 0 } else { 3 };
        let mut layer = 0;
        let mut limit = 1000.0f32;
        for n in 0..3usize {
            let i = first + n;
            let bound = self.layers[i].boundary;
            if self.enabled & (1 << i) != 0 && percent < bound && bound < limit {
                layer = n as u8 + 1;
                limit = bound;
            }
        }
        layer
    }

    /// The track for a location layer (1–3) by day or night.
    pub fn layer_track(&self, layer: u8, day: bool) -> Option<&MediaLayer> {
        let i = usize::from(layer).checked_sub(1)?;
        self.layers
            .get(i + if day { 0 } else { 3 })
            .filter(|_| i < 3)
    }

    pub fn label(&self) -> String {
        self.editor_id
            .clone()
            .unwrap_or_else(|| self.form_id.to_string())
    }
}

/// The lists of a controller, in the order [`LocationController::lists`]
/// gives them.
pub const LIST_NAMES: [&str; 6] = ["neutral", "enemy", "ally", "friend", "location", "battle"];

/// A media location controller (`ALOC`; loader `005963d0`, update
/// `00595600`).
#[derive(Debug, Clone, PartialEq)]
pub struct LocationController {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `NAM1` (default 4, the constructor's): bits 0–3 the list it plays
    /// from (0 neutral, 1 enemy, 2 ally, 3 friend, 4 location; others
    /// none), bits 4–5 what happens at a track's end (0 it starts again,
    /// else the set is over), bit 6 (0x40) day and night from the climate
    /// (else from `NAM5`/`NAM6`). Every controller in `FalloutNV.esm` has
    /// bit 6.
    pub flags: u32,
    /// `NAM4` (default 60): seconds before a new set may start once one
    /// stops (0 in every controller of `FalloutNV.esm`).
    pub delay: f32,
    /// `NAM5`, `NAM6` (default 0x40, 0x150): day and night begin, in
    /// 24/256-hour steps (64 = 6:00).
    pub day_start: u32,
    pub night_start: u32,
    /// The lists, each as the game keeps it in memory: in **reverse** file
    /// order (each entry is put at the front, `005ae3d0`). In
    /// [`LIST_NAMES`] order: neutral (`HNAM`), enemy (`YNAM`), ally
    /// (`ZNAM`), friend (`XNAM`), location (`LNAM`), battle (`GNAM`).
    pub lists: [Vec<FormId>; 6],
    /// `RNAM`: the faction whose members aware of the player change the
    /// list (the code's message calls it a reputation; the data names
    /// factions).
    pub faction: Option<FormId>,
}

impl LocationController {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<LocationController> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == ALOC)?;
        let record = rr.record().ok()?;
        let get = |kind: &[u8; 4]| record.get(FourCC::new(kind));
        let u32_of = |kind: &[u8; 4]| {
            get(kind)
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_u32(&s.data, 0))
        };
        let list = |kind: &[u8; 4]| {
            let mut v: Vec<FormId> = record
                .get_all(FourCC::new(kind))
                .filter_map(|s| form(&rr, &s.data))
                .collect();
            v.reverse();
            v
        };
        Some(LocationController {
            form_id: id,
            editor_id: record.editor_id(),
            flags: u32_of(b"NAM1").unwrap_or(4),
            delay: get(b"NAM4").and_then(|s| float(&s.data)).unwrap_or(60.0),
            day_start: u32_of(b"NAM5").unwrap_or(0x40),
            night_start: u32_of(b"NAM6").unwrap_or(0x150),
            lists: [
                list(b"HNAM"),
                list(b"YNAM"),
                list(b"ZNAM"),
                list(b"XNAM"),
                list(b"LNAM"),
                list(b"GNAM"),
            ],
            faction: get(b"RNAM").and_then(|s| form(&rr, &s.data)),
        })
    }

    /// The list it plays from by default (`NAM1` bits 0–3).
    pub fn default_list(&self) -> u32 {
        self.flags & 0xF
    }

    /// `NAM1` bits 4–5: 0 when a track starts again at its end.
    pub fn track_end(&self) -> u32 {
        (self.flags >> 4) & 3
    }

    pub fn climate_days(&self) -> bool {
        self.flags & 0x40 != 0
    }

    /// The list for a reaction state (0 neutral, 1 enemy, 2 ally, 3 friend,
    /// 4 location; others none), as the update's switch has it.
    pub fn list_for(&self, state: i32) -> Option<usize> {
        match state {
            0 => Some(0),
            1 => Some(1),
            2 => Some(2),
            3 => Some(3),
            4 => Some(4),
            _ => None,
        }
    }

    /// Whether it's day for this controller (`00595d30`). With bit 6, the
    /// climate's: from the middle of sunrise to the middle of sunset,
    /// inclusive (`NVDefaultClimate`: 7:00–19:00; `DefaultClimate`
    /// 7:00–18:00); with no climate (no sky), day. Without it, `NAM5` ×
    /// 0.09375 ≤ hour ≤ `NAM6` × 0.09375.
    pub fn is_day(&self, climate: Option<&crate::weather::Climate>, hour: f32) -> bool {
        let hour = f64::from(hour);
        if self.climate_days() {
            let Some(c) = climate else { return true };
            let rise = (f64::from(c.sunrise.0) + f64::from(c.sunrise.1)) / 2.0;
            let set = (f64::from(c.sunset.0) + f64::from(c.sunset.1)) / 2.0;
            return rise <= hour && hour <= set;
        }
        let step = 0.09375;
        f64::from(self.day_start) * step <= hour && hour <= f64::from(self.night_start) * step
    }

    pub fn label(&self) -> String {
        self.editor_id
            .clone()
            .unwrap_or_else(|| self.form_id.to_string())
    }
}

/// A placed audio marker (`REFR` of [`AUDIO_MARKER`] with `MMRK`; the
/// subrecords `00589600` reads, in order: `MMRK`, `FULL`, `CNAM` the
/// controller, `BNAM`, `MNAM`, `NNAM` (the last three unread by the music
/// code), then the reference's `XRDS`).
#[derive(Debug, Clone, PartialEq)]
pub struct AudioMarker {
    pub reference: FormId,
    pub position: [f32; 3],
    pub controller: Option<FormId>,
    /// `XRDS`; [`DEFAULT_MARKER_RADIUS`] when 0 or missing.
    pub radius: f32,
}

impl AudioMarker {
    pub fn load(order: &LoadOrder, reference: FormId) -> Option<AudioMarker> {
        let rr = order.get(reference)?;
        if rr.entry.header.kind.as_bytes() != b"REFR" {
            return None;
        }
        // Its base first (a world's every reference is looked at): only an
        // audio marker's record is read whole.
        let name = rr.subrecord(esm::sig::NAME).ok()??;
        if form(&rr, &name) != Some(AUDIO_MARKER) {
            return None;
        }
        let record = rr.record().ok()?;
        let base = record
            .get(esm::sig::NAME)
            .and_then(|s| form(&rr, &s.data))?;
        if base != AUDIO_MARKER {
            return None;
        }
        let mut seen = false;
        let mut controller = None;
        for sub in &record.subrecords {
            if sub.kind == MMRK {
                seen = true;
            } else if seen && sub.kind.as_bytes() == b"CNAM" {
                controller = form(&rr, &sub.data);
                break;
            }
        }
        if !seen {
            return None;
        }
        let data = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 12)?;
        let radius = record
            .get(XRDS)
            .and_then(|s| float(&s.data))
            .filter(|r| r.abs() > 1e-6)
            .unwrap_or(DEFAULT_MARKER_RADIUS);
        Some(AudioMarker {
            reference,
            position: [
                le_f32(&data.data, 0),
                le_f32(&data.data, 4),
                le_f32(&data.data, 8),
            ],
            controller,
            radius,
        })
    }

    /// The 2D distance² to a point, the measure every music rule uses.
    pub fn distance2(&self, point: [f32; 3]) -> f32 {
        let dx = point[0] - self.position[0];
        let dy = point[1] - self.position[1];
        dy * dy + dx * dx
    }
}

/// An interior's audio markers in the order the game walks them
/// (`00947d80`): its references whose base is the audio marker, each put
/// at the front of the list, so the reverse of the cell's order.
pub fn audio_markers_in_cell(order: &LoadOrder, cell: FormId) -> Vec<AudioMarker> {
    let mut out: Vec<AudioMarker> = order
        .references_in_cell(cell)
        .into_iter()
        .filter_map(|rr| AudioMarker::load(order, rr.form_id))
        .collect();
    out.reverse();
    out
}

/// A worldspace's audio markers (`005883c0` with its own list only): those
/// placed in the worldspace's cells, persistent ones first, each put at
/// the front as for interiors. **Which references the game's list holds,
/// and their order, isn't traced**; it only matters where markers
/// overlap (the first one holding the player wins).
pub fn audio_markers_in_world(order: &LoadOrder, world: FormId) -> Vec<AudioMarker> {
    let mut persistent = Vec::new();
    let mut temporary = Vec::new();
    for rr in order.records_of_type(esm::sig::CELL) {
        if order.world_of(&rr) != Some(world) || rr.entry.header.is_deleted() {
            continue;
        }
        let is_persistent = rr.entry.header.flags & esm::flags::PERSISTENT != 0;
        for r in order.references_in_cell(rr.form_id) {
            if r.entry.header.kind.as_bytes() != b"REFR" {
                continue;
            }
            if let Some(m) = AudioMarker::load(order, r.form_id) {
                if is_persistent {
                    persistent.push(m);
                } else {
                    temporary.push(m);
                }
            }
        }
    }
    persistent.extend(temporary);
    persistent.reverse();
    persistent
}

/// A region's sound data (`REGN` data type 7, `TESRegionDataSound`;
/// loader `004f4860`): `RDAT` (type 7, override flag, priority), then
/// `RDMO` a music type (unused by the music), `RDSI` the incidental set,
/// `RDSB` battle sets (in file order), `RDSD` ambient sounds.
#[derive(Debug, Clone, PartialEq)]
pub struct RegionSound {
    pub region: FormId,
    pub override_others: bool,
    pub priority: u8,
    pub incidental: Option<FormId>,
    pub battle: Vec<FormId>,
}

impl RegionSound {
    pub fn load(order: &LoadOrder, region: FormId) -> Option<RegionSound> {
        let rr = order.get(region).filter(|r| r.entry.header.kind == REGN)?;
        let record = rr.record().ok()?;
        let mut kind = 0;
        let mut out: Option<RegionSound> = None;
        for sub in &record.subrecords {
            if sub.kind == RDAT && sub.data.len() >= 6 {
                kind = le_u32(&sub.data, 0);
                if kind == 7 && out.is_none() {
                    out = Some(RegionSound {
                        region,
                        override_others: sub.data[4] != 0,
                        priority: sub.data[5],
                        incidental: None,
                        battle: Vec::new(),
                    });
                }
                continue;
            }
            if kind != 7 {
                continue;
            }
            let Some(r) = out.as_mut() else { continue };
            match sub.kind.as_bytes() {
                // Only a non-zero form replaces it.
                b"RDSI" => {
                    if let Some(f) = form(&rr, &sub.data) {
                        r.incidental = Some(f);
                    }
                }
                b"RDSB" => r.battle.extend(form(&rr, &sub.data)),
                _ => {}
            }
        }
        out
    }
}

/// A cell's acoustic space (`XCAS`).
pub fn acoustic_space(order: &LoadOrder, cell: FormId) -> Option<FormId> {
    let rr = order.get(cell)?;
    let record = rr.record().ok()?;
    record.get(XCAS).and_then(|s| form(&rr, &s.data))
}

/// An acoustic space's region (`ASPC` `RDAT`, kept at +0x48): its sound
/// data is the music's fallback.
pub fn acoustic_region(order: &LoadOrder, space: FormId) -> Option<FormId> {
    let rr = order.get(space).filter(|r| r.entry.header.kind == ASPC)?;
    let record = rr.record().ok()?;
    record.get(RDAT).and_then(|s| form(&rr, &s.data))
}

/// The region sound data an exterior square gives the music (`011dd380`,
/// chosen as the player enters a cell, `0094cae0`): among the square's
/// regions (`XCLR`, walked in reverse, as the weather's) of this
/// worldspace whose outlines hold the player, the sound data of each in
/// turn: one that overrides is kept over none kept, or over a kept one of
/// lower priority; while none is kept or this one is the kept one, it's
/// taken if it has an incidental set (the last such wins). The walk order
/// and the override rule are the weather's (read); that the walk is in
/// reverse is taken from the weather code, not traced here.
pub fn exterior_sound_region(
    order: &LoadOrder,
    cell: FormId,
    world: FormId,
    position: [f32; 3],
) -> Option<FormId> {
    let rr = order.get(cell)?;
    let record = rr.record().ok()?;
    let mut regions: Vec<FormId> = record
        .get(XCLR)
        .map(|s| {
            s.data
                .chunks_exact(4)
                .filter_map(|c| form(&rr, c))
                .collect()
        })
        .unwrap_or_default();
    regions.reverse();
    let mut kept: Option<RegionSound> = None;
    let mut chosen = None;
    for id in regions {
        let Some(region) = crate::region::Region::load(order, id) else {
            continue;
        };
        if region.world.is_some_and(|w| w != world) {
            continue;
        }
        if !region.contains(position[0], position[1]) {
            continue;
        }
        let Some(data) = RegionSound::load(order, id) else {
            continue;
        };
        if data.override_others {
            let take = match &kept {
                None => true,
                Some(k) => k.region != data.region && k.priority < data.priority,
            };
            if take {
                kept = Some(data.clone());
            }
        }
        if kept.as_ref().map_or(true, |k| k.region == data.region) && data.incidental.is_some() {
            chosen = Some(id);
        }
    }
    chosen
}

/// The second number of a faction's relation (`XNAM`: faction, modifier,
/// reaction) toward another, 0 without one (`0048bf50`). The music's
/// controllers read this, the **modifier**, and treat it as a reaction (0
/// neutral, 1 enemy, 2 ally, 3 friend), which looks like a slip in the
/// game but is what its code does. Scripts' `SetEnemy`/`SetAlly` aren't
/// applied here: whether they change this number isn't traced.
pub fn faction_modifier(order: &LoadOrder, faction: FormId, toward: FormId) -> i32 {
    let Some(rr) = order.get(faction).filter(|r| r.entry.header.kind == FACT) else {
        return 0;
    };
    let Ok(record) = rr.record() else { return 0 };
    let modifier = record
        .get_all(XNAM)
        .filter(|s| s.data.len() >= 8)
        .find(|s| form(&rr, &s.data) == Some(toward))
        .map_or(0, |s| le_u32(&s.data, 4) as i32);
    modifier
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(enabled: u8, bounds: [f32; 6]) -> MediaSet {
        MediaSet {
            form_id: FormId(1),
            editor_id: None,
            code: 1,
            kind: Some(SetKind::Location),
            layers: bounds.map(|b| MediaLayer {
                file: Some("music\\x.mp3".into()),
                decibels: -4.0,
                boundary: b,
            }),
            enabled,
            dnam: 6.0,
            enam: 15.0,
            fnam: 9.0,
            gnam: 0.0,
            hnam: None,
            inam: None,
        }
    }

    #[test]
    fn location_layers_go_by_distance_squared() {
        // Doc Mitchell's set: 100 / 66.6 / 33.3, all six enabled.
        let s = set(0x3F, [100.0, 66.6, 33.3, 100.0, 66.6, 33.3]);
        assert_eq!(s.layer_at(7.1, true), 3);
        assert_eq!(s.layer_at(33.3, true), 2);
        assert_eq!(s.layer_at(50.0, false), 2);
        assert_eq!(s.layer_at(99.0, true), 1);
        assert_eq!(s.layer_at(100.0, true), 0);
        assert_eq!(s.layer_at(100_000.0, true), 0);
        // Layer 3 switched off: the middle one is as deep as it goes.
        let s = set(0x03, [100.0, 66.6, 33.3, 100.0, 66.6, 33.3]);
        assert_eq!(s.layer_at(1.0, true), 2);
        // A layer whose boundary isn't inside the last one's is skipped.
        let s = set(0x07, [50.0, 80.0, 20.0, 0.0, 0.0, 0.0]);
        assert_eq!(s.layer_at(60.0, true), 2);
        assert_eq!(s.layer_at(10.0, true), 3);
        assert_eq!(s.layer_at(30.0, true), 1);
        assert_eq!(s.layer_track(3, false).map(|l| l.boundary), Some(0.0));
        assert!(s.layer_track(0, true).is_none());
        assert!(s.layer_track(4, true).is_none());
    }
}
