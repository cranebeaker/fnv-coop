//! The radio (`FalloutRadio` (Xbox PDB), FalloutNV.exe 1.4.0.525; the
//! research notes `radio-spec.md`): stations are placed talking activators
//! whose base has the radio flag (form flag 0x20000, `004fedc0`) and carry
//! radio data (`XRDO`: radius, range type, static %, position reference).
//!
//! * Every `iRadioUpdateInterval` ms outdoors, once on entering an interior
//!   (`00832ad0`), the stations in range are found (`004ff1a0`); an
//!   enabled one in range for the first time is discovered (sound
//!   `UIRadioSignalFound`, "%s signal found.", not on the first pass after
//!   a new game or a load: `00833d00`), and each station's signal strength
//!   is worked out (clear below `c = 1 − static%/100` of the radius, then
//!   straight down to nothing at the edge, truncated).
//! * The Pip-Boy's DATA › Radio lists the discovered stations that are
//!   enabled and meant for the Pip-Boy (`0079bea0`): in range first, then
//!   by name (`strcmp`), out of range at alpha 128. Clicking an in-range
//!   row turns the radio off, and on and tuned to it unless it was the one
//!   playing (`00796fd0` case 0x19).
//! * Turning the radio on clears both music decks and holds the music
//!   manager (`008324e0`); tuning makes the station's state
//!   (`00832cb0`: its start somewhere in the last 30 s, so a new
//!   programme starts at once), and each station plays conversations made
//!   from `RadioHello` (`0061b440` with `0061a7d0`): the first line that can
//!   be said, or one of a run of random ones, then a random untried link
//!   (`TCLT`) that gives a line, up to 100 lines; run-immediately lines'
//!   result scripts run as the conversation is made, the others' first
//!   script as the line starts (`00834260`).
//! * A line is a song (its response's sound form, `data\sound\` + the
//!   sound's file, on a music deck as type 7 in step with the line's start)
//!   or the DJ's voice file; the next line starts 50 ms after one ends; a
//!   line with neither is skipped. A weak signal adds the static loop at
//!   `((100 − strength)/100)^0.75`; out of range the radio goes off with
//!   `UIRadioSignalLost` / "%s signal lost." and comes back on by itself
//!   when the station does (`008331c0`, `00833d00`).
//!
//! * Script functions act on this same radio (`world::more_functions::
//!   radio`): `PipboyRadio` / `PipBoyRadioOff` switch and tune it as the
//!   Pip-Boy's list does (`008324e0`, `00832240(station, 1)`, which also
//!   asks for a range pass now), `StartRadioConversation` (`00835be0`)
//!   replaces a station's programme with one from the topic given, 50 ms
//!   on, and `SetNPCRadio` (`EnableNPCRadio` (Xbox PDB), `00835810`) adds a
//!   receiver (`FORadioReceiver` (Xbox PDB)) to a station: the station then
//!   runs although the Pip-Boy isn't tuned to it (`00834260` skips a
//!   station only when it's neither the Pip-Boy's nor has receivers), and
//!   each receiver starts the line playing when the player is within its
//!   hearing distance (a creature `fCreatureRadioMax` × 1.1, else the
//!   station's `SNAM` sound's, or `AMLRadio`'s, largest attenuation
//!   distance × 1.1, else 3000), in step with the line's start; a song as
//!   its `_mono` file. A receiver in another worldspace or interior than
//!   the player stops for good (`bShouldBePlaying` cleared).
//!
//! Labelled guesses: the path-finding range tests (range type 0 seen from
//! an interior, types 2 and 3 across doors, `006d4d20` / `006d4eb0`) aren't
//! traced: an interior player is out of range of a type 0 station in
//! another place, in range of a type 2 or 3 station only in its own cell;
//! a voice line ends when its file's length has passed (the game's sound
//! calls back at its end), and a receiver's sound with the station's line;
//! script functions take the audio clock of the last frame as now; the
//! receivers' loudness as the Pip-Boy's (the 3D sound's category, flags
//! 0x500102, isn't traced); vfunc +0x21c read as "is a creature".

use std::collections::HashMap;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};

use crate::dialogue::{self, Info, Speaker, PLAYER_REF};
use crate::scripting::GameState;

/// A talking activator's form flag: a radio station (`004fedc0`).
pub const RADIO_FLAG: u32 = 0x0002_0000;
/// Not on the Pip-Boy (`004fee20`): in-world speakers.
pub const NON_PIPBOY_FLAG: u32 = 0x1000_0000;
/// Broadcasting from the start (`ContinuousBroadcast`, vfunc +0xb4).
pub const BROADCAST_FLAG: u32 = 0x4000_0000;
/// The topic stations' programmes come from (`0061a2d0(7, 0)`).
pub const RADIO_HELLO: &str = "RadioHello";
/// The gap before a line follows the last (`00834260`: 0x32 ms).
pub const GAP_MS: u64 = 50;
/// A programme's most lines (`0061b440`: stops at 100).
pub const MOST_LINES: usize = 100;
/// `iRadioUpdateInterval` (exe 75; the INI says 250).
pub const UPDATE_INTERVAL_MS: u64 = 75;
/// `fRadioStaticAtOuterRadiusPct` (a fraction).
pub const STATIC_AT_OUTER: f32 = 0.1;
/// The sounds and the message icon.
pub const SIGNAL_FOUND: &str = "UIRadioSignalFound";
pub const SIGNAL_LOST: &str = "UIRadioSignalLost";
pub const STATIC_LOOP: &str = "UIRadioStaticLP";
pub const TOWER_ICON: &str = "Interface\\Icons\\Message Icons\\glow_message_radio_tower.dds";
/// The receivers' attenuation model (`pRadioAttenuationModel` (Xbox
/// PDB), `011dd440`, looked up by `00832ad0`).
pub const ATTENUATION_MODEL: &str = "AMLRadio";
/// `fCreatureRadioMax:Audio` (exe `01013970`).
pub const CREATURE_RADIO_MAX: f32 = 2000.0;
/// A receiver's hearing distance without a sound (`0104fca0`).
pub const DEFAULT_HEARING: f32 = 3000.0;

const XRDO: FourCC = FourCC::new(b"XRDO");
const SNAM: FourCC = FourCC::new(b"SNAM");
const CREA: FourCC = FourCC::new(b"CREA");
const NAME: FourCC = FourCC::new(b"NAME");
const TACT: FourCC = FourCC::new(b"TACT");
const REFR: FourCC = FourCC::new(b"REFR");
const VNAM: FourCC = FourCC::new(b"VNAM");

/// A reference's radio data (`XRDO`, extra data 0x68).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RadioData {
    pub radius: f32,
    /// 0 a radius, 1 everywhere, 2 the worldspace (and interiors linked to
    /// it), 3 linked interiors, 4 the current cell.
    pub range: u32,
    pub static_pct: f32,
    pub position: Option<FormId>,
}

impl RadioData {
    pub fn parse(data: &[u8]) -> Option<RadioData> {
        (data.len() >= 16).then(|| RadioData {
            radius: le_f32(data, 0),
            range: le_u32(data, 4),
            static_pct: le_f32(data, 8),
            position: Some(FormId(le_u32(data, 12))).filter(|f| f.0 != 0),
        })
    }
}

/// A placed station.
#[derive(Debug, Clone, PartialEq)]
pub struct StationRef {
    pub reference: FormId,
    pub base: FormId,
    pub name: String,
    pub data: RadioData,
    pub world: Option<FormId>,
    pub cell: Option<FormId>,
    pub position: [f32; 3],
    /// The base's form flags.
    pub base_flags: u32,
    /// The base's voice type (`VNAM`), whose files the DJ's lines are.
    pub voice: Option<FormId>,
}

impl StationRef {
    pub fn pipboy(&self) -> bool {
        self.base_flags & NON_PIPBOY_FLAG == 0
    }
}

/// Every placed station (`AddRadioStation`, `004ff150`, as each reference
/// is set up): references of talking activators with the radio flag.
pub fn station_refs(order: &LoadOrder) -> Vec<StationRef> {
    let mut bases: HashMap<FormId, (u32, String, Option<FormId>)> = HashMap::new();
    for rr in order.records_of_type(TACT) {
        let flags = rr.entry.header.flags;
        if flags & RADIO_FLAG == 0 {
            continue;
        }
        let Ok(record) = rr.record() else {
            continue;
        };
        let voice = record
            .get(VNAM)
            .filter(|s| s.data.len() >= 4)
            .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
            .filter(|f| f.0 != 0);
        bases.insert(
            rr.form_id,
            (flags, record.full_name().unwrap_or_default(), voice),
        );
    }
    let mut out = Vec::new();
    if bases.is_empty() {
        return out;
    }
    for rr in order.records_of_type(REFR) {
        // Every reference in the game is looked at: only those with radio
        // data are read whole.
        if !matches!(rr.subrecord(XRDO), Ok(Some(_))) {
            continue;
        }
        let Ok(record) = rr.record() else {
            continue;
        };
        let Some(xrdo) = record.get(XRDO).and_then(|s| RadioData::parse(&s.data)) else {
            continue;
        };
        let Some(base) = record
            .get(NAME)
            .filter(|s| s.data.len() >= 4)
            .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        else {
            continue;
        };
        let Some((flags, name, voice)) = bases.get(&base) else {
            continue;
        };
        let mut data = xrdo;
        data.position = data.position.map(|p| rr.plugin.to_global(p));
        let position = record
            .get(esm::sig::DATA)
            .filter(|s| s.data.len() >= 12)
            .map_or([0.0; 3], |s| {
                [le_f32(&s.data, 0), le_f32(&s.data, 4), le_f32(&s.data, 8)]
            });
        out.push(StationRef {
            reference: rr.form_id,
            base,
            name: name.clone(),
            data,
            world: order.world_of(&rr),
            cell: order.cell_of(&rr),
            position,
            base_flags: *flags,
            voice: *voice,
        });
    }
    out
}

/// Where the player is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Place {
    pub cell: Option<FormId>,
    pub world: Option<FormId>,
    pub position: [f32; 3],
}

/// The stations in range and their distances (`004ff1a0`): disabled
/// stations count only while the player is in an interior.
// Translated from 004ff1a0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn in_range(
    stations: &[StationRef],
    place: &Place,
    disabled: &dyn Fn(FormId) -> bool,
    position_of: &dyn Fn(FormId) -> Option<(Option<FormId>, [f32; 3])>,
) -> Vec<(FormId, f32)> {
    let mut out = Vec::new();
    if place.cell.is_none() {
        return out;
    }
    for s in stations {
        if disabled(s.reference) && place.world.is_some() {
            continue;
        }
        let mut dist = f32::MAX;
        let hit = match s.data.range {
            0 => {
                let (world, pos) = match s.data.position {
                    Some(p) => position_of(p).unwrap_or((s.world, s.position)),
                    None => (s.world, s.position),
                };
                match (world, place.world) {
                    (Some(w), Some(pw)) if w == pw => {
                        let d = [
                            place.position[0] - pos[0],
                            place.position[1] - pos[1],
                            place.position[2] - pos[2],
                        ];
                        dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                        dist < s.data.radius
                    }
                    // An interior player: the game measures the path through
                    // doors (`006d4eb0`, not traced): out of range here.
                    _ => false,
                }
            }
            1 => true,
            2 => match place.world {
                Some(pw) => s.world == Some(pw),
                // Through doors (`006d4d20`, not traced): its own cell only.
                None => s.cell == place.cell,
            },
            3 => place.world.is_none() && s.cell == place.cell,
            4 => place.world.is_none() && s.cell == place.cell,
            _ => false,
        };
        if hit {
            out.push((s.reference, dist));
        }
    }
    out
}

/// The signal strength 0..100 at a distance (`00833d00`): clear up to
/// `c` of the radius (`c = 1 − static%/100`, or `1 −
/// fRadioStaticAtOuterRadiusPct` when the station gives none), then the
/// static rises in a straight line, truncated.
// Translated from 00833d00 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn strength(data: &RadioData, dist: f32) -> u8 {
    let c = if data.static_pct != 0.0 {
        1.0 - data.static_pct / 100.0
    } else {
        1.0 - STATIC_AT_OUTER
    };
    if data.range == 0 && data.radius > 0.0 && dist > 0.0 && dist < f32::MAX {
        let ratio = dist / data.radius;
        if ratio > c {
            let stat = ((ratio - c) / (1.0 - c) * 100.0) as i32;
            return (100 - stat.clamp(0, 100)) as u8;
        }
    }
    100
}

/// The static loop's volume at a strength (`008331c0`: `01016264` 0.75).
pub fn static_volume(strength: u8) -> f32 {
    ((100.0 - f32::from(strength)) / 100.0).powf(0.75)
}

/// One line of a programme: the line and whether it's a song.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub info: Info,
}

impl Item {
    /// The first response's sound form (`TRDT` bytes 16..20): a song.
    pub fn sound(&self) -> Option<FormId> {
        self.info.responses.first().and_then(|r| r.sound)
    }
}

/// A station's state (`FORadioStation` (Xbox PDB), `00832cb0`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Station {
    pub reference: FormId,
    /// The programme and the line playing.
    pub items: Vec<Item>,
    pub current: Option<usize>,
    /// When the line started (audio clock ms) and how long it lasts (< 1
    /// unknown or over).
    pub start: u64,
    pub duration: i64,
    pub power: u8,
    pub target: u8,
    pub lost: bool,
    pub running: bool,
    /// Its receivers (`StationUsers` (Xbox PDB), `+0x1c`).
    pub users: Vec<Receiver>,
    /// The topic a script started (`StartRadioConversation`; `None` the
    /// default) while the programme made from it plays.
    pub started: Option<Option<FormId>>,
    /// That conversation, loaded from a save, starts at the next frame.
    pub pending_start: bool,
}

/// Someone playing a station through their own speaker
/// (`FalloutRadio::FORadioReceiver` (Xbox PDB), made by `00835810`).
#[derive(Debug, Clone, PartialEq)]
pub struct Receiver {
    pub reference: FormId,
    /// `bShouldBePlaying` (`+0x1c`).
    pub playing: bool,
    /// The line it started last: (item, the line's start).
    pub line: Option<(usize, u64)>,
    /// Its static loop is on.
    pub static_on: bool,
}

/// What the radio asks the game to do.
#[derive(Debug, Clone, PartialEq)]
pub enum RadioEvent {
    /// A sound by editor ID (`UIRadioSignalFound`, `UIRadioSignalLost`).
    Sound(&'static str),
    /// A HUD message with the radio tower icon, 2 s.
    Message(String),
    /// Both music decks cleared (`008304a0`).
    ClearDecks,
    /// The music manager held or let go (`008325a0`).
    HoldMusic(bool),
    /// A song onto a deck as type 7, in step with `sync`.
    Song {
        path: String,
        sync: u64,
    },
    /// The DJ's voice file at a volume (`fDefaultRadioVolume` × strength).
    Voice {
        path: String,
        volume: f32,
    },
    StopVoice,
    /// The static loop at a volume, or off.
    Static(Option<f32>),
    /// A result script run on the station.
    Script {
        source: String,
        speaker: FormId,
    },
    /// The tuned station changed or the radio went off (for the list).
    Changed,
    /// A receiver starts the station's line `offset` ms in: a song (its
    /// mono `.ogg` file, [`mono_song`]) or the DJ's voice file.
    Receiver {
        reference: FormId,
        path: String,
        song: bool,
        volume: f32,
        offset: u64,
    },
    /// A receiver's sound stopped (`00ad88f0` on its handle).
    ReceiverStop(FormId),
    /// A receiver's static loop at a volume, or off.
    ReceiverStatic {
        reference: FormId,
        volume: Option<f32>,
    },
}

/// The game's side the radio needs: files and lengths.
pub trait RadioFiles {
    /// Whether a voice file exists, and its length in ms.
    fn voice_ms(&mut self, path: &str) -> Option<u32>;
    /// A song's length in ms (`data\sound\…`).
    fn song_ms(&mut self, path: &str) -> Option<u32>;
}

/// The radio (`011dd42c` … `011dd59c`).
#[derive(Debug, Clone, Default)]
pub struct Radio {
    /// The placed stations, read once.
    pub stations_placed: Option<Vec<StationRef>>,
    pub stations: Vec<Station>,
    /// The discovered stations (`011dd59c`), in order found.
    pub discovered: Vec<FormId>,
    /// The Pip-Boy radio is on (`011dd434`) and tuned to (`011dd42c`).
    pub on: bool,
    pub active: Option<FormId>,
    /// A tuned station that went out of range (`011dd430`).
    pub lost_station: Option<FormId>,
    /// The first range pass since a new game or a load is silent
    /// (`011a179c`).
    pub not_first_pass: bool,
    last_find: u64,
    /// The interior the one pass was made in (`011dd435`).
    interior_pass: Option<FormId>,
    /// The last in-range list, for the Pip-Boy.
    pub in_range: Vec<FormId>,
    last_enable_check: u64,
    enabled_states: Vec<bool>,
    voice_playing: bool,
    static_on: bool,
    rng: u64,
    /// The audio clock at the last frame (ms): now, for the script
    /// functions, which run between frames.
    pub clock: u64,
    /// What script functions asked for since the last frame.
    pub pending: Vec<RadioEvent>,
    /// A range pass is due at the next frame (`ForceRadioStationUpdate`,
    /// `00832ad0(1)`; a script's tune, `00833d00(1)`).
    pub force_update: bool,
    /// `fCreatureRadioMax` from the INI ([`CREATURE_RADIO_MAX`] when
    /// `None`).
    pub creature_radio_max: Option<f32>,
}

impl Radio {
    fn placed<'a>(&'a mut self, order: &LoadOrder) -> &'a [StationRef] {
        self.stations_placed
            .get_or_insert_with(|| station_refs(order))
            .as_slice()
    }

    fn rand(&mut self) -> u64 {
        // xorshift: the picks are uniform, as `rand() % n` is.
        let mut x = self.rng.max(1);
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng = x;
        x
    }

    pub fn seed(&mut self, seed: u64) {
        self.rng = seed | 1;
    }

    /// A station's reference record.
    pub fn station_ref(&self, reference: FormId) -> Option<&StationRef> {
        self.stations_placed
            .as_ref()?
            .iter()
            .find(|s| s.reference == reference)
    }

    /// DATA › Radio's rows (`0079bea0`): (reference, name, in range,
    /// tuned), in range first then by name.
    // Translated from 0079bea0 / 0079be30 / 0079bd70 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn rows(&self, disabled: &dyn Fn(FormId) -> bool) -> Vec<(FormId, String, bool, bool)> {
        let mut rows: Vec<(FormId, String, bool, bool)> = self
            .discovered
            .iter()
            .filter_map(|&r| {
                let s = self.station_ref(r)?;
                if disabled(r) || !s.pipboy() {
                    return None;
                }
                Some((
                    r,
                    s.name.clone(),
                    self.in_range.contains(&r),
                    self.on && self.active == Some(r),
                ))
            })
            .collect();
        rows.sort_by(|a, b| {
            b.2.cmp(&a.2)
                .then_with(|| a.1.as_bytes().cmp(b.1.as_bytes()))
        });
        rows
    }

    /// `008324e0`: the Pip-Boy radio on (decks cleared, the music manager
    /// held) or off (the decks cleared if the radio had them, the music
    /// manager let go, voice and static stopped, no station tuned).
    // Translated from 008324e0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn enable(&mut self, on: bool, events: &mut Vec<RadioEvent>) {
        if !on {
            if self.on {
                events.push(RadioEvent::ClearDecks);
                events.push(RadioEvent::HoldMusic(false));
            }
            events.push(RadioEvent::StopVoice);
            events.push(RadioEvent::Static(None));
            self.voice_playing = false;
            self.static_on = false;
            self.active = None;
        } else {
            events.push(RadioEvent::ClearDecks);
            events.push(RadioEvent::HoldMusic(true));
        }
        self.on = on;
        events.push(RadioEvent::Changed);
    }

    /// `00832240`: tunes the Pip-Boy to a station (made the first time,
    /// `00832cb0`: its start up to 30 s ago, so a programme starts next
    /// frame).
    // Translated from 00832240 / 00832cb0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn tune(&mut self, reference: FormId, now: u64, events: &mut Vec<RadioEvent>) {
        if !self.on {
            return;
        }
        self.station_index(reference, now);
        self.active = Some(reference);
        self.lost_station = None;
        events.push(RadioEvent::Changed);
    }

    /// A station's state, made the first time (`00832cb0`: its start
    /// somewhere in the last 30 s, `00944460(0, 30000)`).
    // Translated from 00832cb0 (decompiled, FalloutNV.exe 1.4.0.525)
    fn station_index(&mut self, reference: FormId, now: u64) -> usize {
        if let Some(i) = self.stations.iter().position(|s| s.reference == reference) {
            return i;
        }
        let back = self.rand() % 30_000;
        self.stations.push(Station {
            reference,
            start: now.saturating_sub(back),
            duration: 1,
            power: 100,
            target: 100,
            ..Station::default()
        });
        self.stations.len() - 1
    }

    /// A saved station state back: a fresh programme at the first frame.
    pub fn restore_station(&mut self, reference: FormId) -> usize {
        self.station_index(reference, 0)
    }

    /// `PipboyRadio on` / `off`, `PipBoyRadioOff` (`008324e0`).
    pub fn script_enable(&mut self, on: bool) {
        let mut events = std::mem::take(&mut self.pending);
        self.enable(on, &mut events);
        self.pending = events;
    }

    /// `PipboyRadio tune` with a station (`00832240(station, 1)`): the lost
    /// station forgotten and a range pass now (`00833d00(1)`, here at the
    /// next frame), then tuned as a click tunes.
    // Translated from 00832240 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn script_tune(&mut self, reference: FormId) {
        if !self.on {
            return;
        }
        self.lost_station = None;
        self.force_update = true;
        let mut events = std::mem::take(&mut self.pending);
        self.tune(reference, self.clock, &mut events);
        self.pending = events;
    }

    /// The stations in range of the player now, first found first
    /// (`004ff1a0` with the player): what `PipboyRadio tune` without a
    /// station tunes to.
    pub fn first_in_range(&mut self, order: &LoadOrder, state: &GameState) -> Option<FormId> {
        let placed = self.placed(order).to_vec();
        let disabled = |r: FormId| disabled(order, state, r);
        in_range(&placed, &place_of(state), &disabled, &|r| {
            position_of(order, r)
        })
        .first()
        .map(|&(r, _)| r)
    }

    /// `StartRadioConversation` (`00835be0`) on a station: what it played
    /// stopped (the Pip-Boy's voice and the radio's decks when it's the
    /// tuned one, its receivers' sounds), and a programme from `topic`
    /// (`None`: the default, `0061a2d0(7, 0)`, `RadioHello`) made now,
    /// starting in 50 ms; no line: over at once.
    // Translated from 00835be0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn start_conversation(
        &mut self,
        order: &LoadOrder,
        scripts: &crate::scripting::ScriptCache,
        state: &mut GameState,
        station: FormId,
        topic: Option<FormId>,
    ) {
        self.placed(order);
        let now = self.clock;
        let i = self.station_index(station, now);
        let mut events = std::mem::take(&mut self.pending);
        if self.on && self.active == Some(station) {
            self.stop_output(&mut events);
        }
        for u in &mut self.stations[i].users {
            u.line = None;
            events.push(RadioEvent::ReceiverStop(u.reference));
        }
        let items = topic
            .or_else(|| order.form_by_editor_id(RADIO_HELLO))
            .map(|t| self.programme(order, scripts, state, station, t))
            .unwrap_or_default();
        let st = &mut self.stations[i];
        st.started = Some(topic);
        st.items = items;
        st.start = now + GAP_MS;
        if st.items.is_empty() {
            st.current = None;
            st.duration = 1;
        } else {
            st.current = Some(0);
            st.duration = 0;
            let info = &st.items[0].info;
            if info.flags & dialogue::RUN_IMMEDIATELY == 0 {
                if let Some(s) = info.begin_script.clone() {
                    events.push(RadioEvent::Script {
                        source: s,
                        speaker: station,
                    });
                }
            }
        }
        self.pending = events;
    }

    /// `SetNPCRadio 1` (`EnableNPCRadio` (Xbox PDB), `00835810`): the
    /// station with that base already playing (`GetActiveStation`,
    /// `00832830`), else the placed one of that base in range of `who`
    /// (`InitStation`, `00832cb0` → `008356e0`), gets `who` as a receiver,
    /// playing. False when there's no such station.
    // Translated from 00835810 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn enable_npc_radio(
        &mut self,
        order: &LoadOrder,
        state: &GameState,
        who: FormId,
        base: FormId,
    ) -> bool {
        let placed = self.placed(order).to_vec();
        let base_of = |r: FormId| placed.iter().find(|s| s.reference == r).map(|s| s.base);
        let i = match self
            .stations
            .iter()
            .position(|s| base_of(s.reference) == Some(base))
        {
            Some(i) => i,
            None => {
                let Some((space, cell, position, _)) = state.place(order, who) else {
                    return false;
                };
                let around = Place {
                    cell: Some(cell),
                    world: (space != cell).then_some(space),
                    position,
                };
                let disabled = |r: FormId| disabled(order, state, r);
                let found = in_range(&placed, &around, &disabled, &|r| position_of(order, r))
                    .into_iter()
                    .find(|&(r, _)| base_of(r) == Some(base));
                let Some((reference, _)) = found else {
                    return false;
                };
                self.station_index(reference, self.clock)
            }
        };
        self.add_receiver(i, who);
        true
    }

    /// `who` as station `i`'s receiver, playing (one receiver a person
    /// here: the game lists the same receiver again on the new station).
    fn add_receiver(&mut self, i: usize, who: FormId) {
        for s in &mut self.stations {
            s.users.retain(|u| u.reference != who);
        }
        self.stations[i].users.push(Receiver {
            reference: who,
            playing: true,
            line: None,
            static_on: false,
        });
    }

    /// A saved receiver back.
    pub fn restore_receiver(&mut self, who: FormId, station: FormId) {
        let i = self.restore_station(station);
        self.add_receiver(i, who);
    }

    /// `SetNPCRadio 0` (`DisableNPCRadio` (Xbox PDB), `00835980`): `who`'s
    /// receiver silenced and let go.
    // Translated from 00835980 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn disable_npc_radio(&mut self, who: FormId) {
        let mut found = false;
        for s in &mut self.stations {
            let before = s.users.len();
            s.users.retain(|u| u.reference != who);
            found |= s.users.len() != before;
        }
        if found {
            self.pending.push(RadioEvent::ReceiverStop(who));
            self.pending.push(RadioEvent::ReceiverStatic {
                reference: who,
                volume: None,
            });
        }
    }

    /// The people playing a station, by person (for saves and the
    /// viewer).
    pub fn receivers(&self) -> impl Iterator<Item = (FormId, FormId)> + '_ {
        self.stations
            .iter()
            .flat_map(|s| s.users.iter().map(move |u| (u.reference, s.reference)))
    }

    /// A click on a DATA › Radio row (`00796fd0` case 0x19): only in-range
    /// rows; the radio off if on; on and tuned unless the row was the one
    /// playing.
    // Translated from 00796fd0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn click_row(&mut self, reference: FormId, now: u64) -> Vec<RadioEvent> {
        let mut events = Vec::new();
        if !self.in_range.contains(&reference) {
            return events;
        }
        let was_tuned = self.on && self.active == Some(reference);
        if self.on {
            self.enable(false, &mut events);
        }
        if !was_tuned {
            self.enable(true, &mut events);
            self.tune(reference, now, &mut events);
        }
        events
    }

    /// One frame (`00832ad0`): the range pass when it's due, then every
    /// station, the tuned one to the Pip-Boy.
    // Translated from 00832ad0 (decompiled, FalloutNV.exe 1.4.0.525)
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        order: &LoadOrder,
        scripts: &crate::scripting::ScriptCache,
        state: &mut GameState,
        place: &Place,
        now: u64,
        interval: u64,
        radio_volume: f32,
        files: &mut dyn RadioFiles,
    ) -> Vec<RadioEvent> {
        self.clock = now;
        let mut events = std::mem::take(&mut self.pending);
        if self.last_find == 0 {
            self.last_find = now;
        }
        let mut find = false;
        if interval < now.saturating_sub(self.last_find) {
            find = true;
            self.last_find = now;
        }
        if place.world.is_none() {
            find = self.interior_pass != place.cell;
            self.interior_pass = place.cell;
        } else {
            self.interior_pass = None;
        }
        // `00832ad0(1)`: asked for by a script.
        if std::mem::take(&mut self.force_update) {
            find = true;
        }
        // Conversations a script started, from a save.
        for i in 0..self.stations.len() {
            if std::mem::take(&mut self.stations[i].pending_start) {
                let (r, topic) = (self.stations[i].reference, self.stations[i].started);
                self.start_conversation(order, scripts, state, r, topic.flatten());
            }
        }
        events.append(&mut self.pending);
        // A station enabled or disabled since (`Enable` resets the interior
        // pass, `005743f0`): looked at with each interval.
        if find || interval < now.saturating_sub(self.last_enable_check) {
            self.last_enable_check = now;
            let placed = self.placed(order).to_vec();
            let states: Vec<bool> = placed
                .iter()
                .map(|s| disabled(order, state, s.reference))
                .collect();
            if states != self.enabled_states {
                if !self.enabled_states.is_empty() {
                    find = true;
                }
                self.enabled_states = states;
            }
        }
        self.update_stations(order, state, place, find, now, &mut events);
        for i in 0..self.stations.len() {
            self.update_station(
                order,
                scripts,
                state,
                i,
                now,
                radio_volume,
                files,
                &mut events,
            );
        }
        events
    }

    /// `00833d00`: discovery and the stations' strength.
    // Translated from 00833d00 (decompiled, FalloutNV.exe 1.4.0.525)
    fn update_stations(
        &mut self,
        order: &LoadOrder,
        state: &GameState,
        place: &Place,
        find: bool,
        now: u64,
        events: &mut Vec<RadioEvent>,
    ) {
        let disabled = |r: FormId| !crate::placement::enabled_now(order, r, &state.disabled);
        let position_of = |r: FormId| position_of(order, r);
        let mut found: Vec<(FormId, f32)> = Vec::new();
        if find {
            let placed = self.placed(order).to_vec();
            found = in_range(&placed, place, &disabled, &position_of);
            for &(r, _) in &found {
                if !disabled(r) && !self.discovered.contains(&r) {
                    if self.not_first_pass {
                        let name = self
                            .station_ref(r)
                            .map(|s| s.name.clone())
                            .unwrap_or_default();
                        events.push(RadioEvent::Sound(SIGNAL_FOUND));
                        events.push(RadioEvent::Message(format!("{name} signal found.")));
                    }
                    self.discovered.push(r);
                    events.push(RadioEvent::Changed);
                }
            }
            self.not_first_pass = true;
            self.in_range = found.iter().map(|(r, _)| *r).collect();
        }
        for i in 0..self.stations.len() {
            let r = self.stations[i].reference;
            let mut back = false;
            if find {
                match found.iter().find(|(f, _)| *f == r) {
                    Some(&(_, d)) => {
                        let data = self.station_ref(r).map(|s| s.data).unwrap_or_default();
                        let st = &mut self.stations[i];
                        st.lost = false;
                        st.target = strength(&data, d);
                        back = true;
                    }
                    None => {
                        let st = &mut self.stations[i];
                        st.lost = true;
                        st.target = 0;
                    }
                }
            }
            if back && self.lost_station == Some(r) {
                let name = self
                    .station_ref(r)
                    .map(|s| s.name.clone())
                    .unwrap_or_default();
                events.push(RadioEvent::Sound(SIGNAL_FOUND));
                events.push(RadioEvent::Message(format!("{name} signal found.")));
                self.enable(true, events);
                self.tune(r, now, events);
                self.lost_station = None;
            }
        }
    }

    /// `00834260` for the Pip-Boy's station: the line over → the next
    /// (its first script run) 50 ms on, or a new programme; then
    /// `008331c0`.
    // Translated from 00834260 (decompiled, FalloutNV.exe 1.4.0.525)
    #[allow(clippy::too_many_arguments)]
    fn update_station(
        &mut self,
        order: &LoadOrder,
        scripts: &crate::scripting::ScriptCache,
        state: &mut GameState,
        i: usize,
        now: u64,
        radio_volume: f32,
        files: &mut dyn RadioFiles,
        events: &mut Vec<RadioEvent>,
    ) {
        let r = self.stations[i].reference;
        let active = self.on && self.active == Some(r);
        // Neither the Pip-Boy's nor played by anyone: left alone.
        if !active && self.stations[i].users.is_empty() {
            return;
        }
        if !crate::placement::enabled_now(order, r, &state.disabled) {
            for u in &mut self.stations[i].users {
                u.line = None;
                events.push(RadioEvent::ReceiverStop(u.reference));
            }
            if active {
                self.enable(false, events);
            }
            return;
        }
        let st = &mut self.stations[i];
        if now + 100 < st.start {
            st.start = now;
        }
        let over = st.duration > 0 && (st.start as i64 + st.duration) < now as i64;
        if over {
            let next = st.current.map_or(0, |c| c + 1);
            let st = &mut self.stations[i];
            if st.current.is_some() && next < st.items.len() {
                st.current = Some(next);
                st.start = now + GAP_MS;
                st.duration = 0;
                let info = st.items[next].info.clone();
                if info.flags & dialogue::RUN_IMMEDIATELY == 0 {
                    if let Some(s) = info.begin_script.clone() {
                        events.push(RadioEvent::Script {
                            source: s,
                            speaker: r,
                        });
                    }
                }
            } else {
                st.items.clear();
                st.current = None;
                st.start = 0;
                st.duration = 0;
                st.started = None;
            }
            for u in &mut self.stations[i].users {
                if u.line.take().is_some() {
                    events.push(RadioEvent::ReceiverStop(u.reference));
                }
            }
            if active {
                self.stop_output(events);
            }
            if self.stations[i].current.is_none() {
                let broadcasting = self.broadcasting(order, state, r);
                self.stations[i].running = broadcasting;
                let topic = order.form_by_editor_id(RADIO_HELLO);
                if let Some(topic) = topic.filter(|_| broadcasting) {
                    let items = self.programme(order, scripts, state, r, topic);
                    let st = &mut self.stations[i];
                    st.items = items;
                    if !st.items.is_empty() {
                        st.current = Some(0);
                        st.start = now + GAP_MS;
                        st.duration = 0;
                        let info = st.items[0].info.clone();
                        if info.flags & dialogue::RUN_IMMEDIATELY == 0 {
                            if let Some(s) = info.begin_script {
                                events.push(RadioEvent::Script {
                                    source: s,
                                    speaker: r,
                                });
                            }
                        }
                    }
                }
            }
        }
        let st = &mut self.stations[i];
        st.power = st.target;
        if active {
            self.pipboy_update(order, i, now, radio_volume, files, events);
        }
        self.receivers_update(order, state, i, now, radio_volume, files, events);
    }

    /// `00834260`'s receivers: each playing one in the player's worldspace
    /// (outdoors) or interior, its static bed by the station's strength,
    /// and the line started when it hasn't been and the player is within
    /// hearing (`00834c3d` … `008352c4`).
    // Translated from 00834260 (disassembly, FalloutNV.exe 1.4.0.525)
    #[allow(clippy::too_many_arguments)]
    fn receivers_update(
        &mut self,
        order: &LoadOrder,
        state: &GameState,
        i: usize,
        now: u64,
        radio_volume: f32,
        files: &mut dyn RadioFiles,
        events: &mut Vec<RadioEvent>,
    ) {
        if self.stations[i].users.is_empty() {
            return;
        }
        let r = self.stations[i].reference;
        let power = self.stations[i].power;
        let player_space = state.player_world.or(state.player_cell);
        let player_at = state.player_position.unwrap_or([0.0; 3]);
        let line = self.line_file(order, i);
        let hearing_sound = self.hearing_sound(order, r);
        let creature_max = self.creature_radio_max.unwrap_or(CREATURE_RADIO_MAX);
        for k in 0..self.stations[i].users.len() {
            let u = self.stations[i].users[k].clone();
            if !u.playing {
                continue;
            }
            let place = state.place(order, u.reference);
            let elsewhere = place.map_or(true, |(space, ..)| Some(space) != player_space);
            if elsewhere {
                let u = &mut self.stations[i].users[k];
                u.playing = false;
                u.line = None;
                events.push(RadioEvent::ReceiverStop(u.reference));
                if std::mem::take(&mut u.static_on) {
                    events.push(RadioEvent::ReceiverStatic {
                        reference: u.reference,
                        volume: None,
                    });
                }
                continue;
            }
            let at = place.map_or([0.0; 3], |p| p.2);
            if power < 100 {
                self.stations[i].users[k].static_on = true;
                events.push(RadioEvent::ReceiverStatic {
                    reference: u.reference,
                    volume: Some(static_volume(power)),
                });
            } else if std::mem::take(&mut self.stations[i].users[k].static_on) {
                events.push(RadioEvent::ReceiverStatic {
                    reference: u.reference,
                    volume: None,
                });
            }
            let st = &self.stations[i];
            let Some(current) = st.current else {
                continue;
            };
            if now < st.start || u.line == Some((current, st.start)) {
                continue;
            }
            let Some((path, song)) = line.clone() else {
                continue;
            };
            let creature = crate::more_functions::placed::base_now(order, state, u.reference)
                .and_then(|b| order.get(b))
                .is_some_and(|b| b.entry.header.kind == CREA);
            let hearing = if !creature {
                hearing_sound.map_or(DEFAULT_HEARING, |far| far * 1.1)
            } else {
                creature_max * 1.1
            };
            let d = [
                player_at[0] - at[0],
                player_at[1] - at[1],
                player_at[2] - at[2],
            ];
            if (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() > hearing {
                continue;
            }
            let start = st.start;
            if st.duration == 0 {
                let ms = if song {
                    files.song_ms(&path)
                } else {
                    files.voice_ms(&path)
                };
                self.stations[i].duration = ms.map_or(1, i64::from).max(1);
            }
            events.push(RadioEvent::Receiver {
                reference: u.reference,
                path: if song { mono_song(&path) } else { path },
                song,
                volume: radio_volume * f32::from(power) / 100.0,
                offset: now - start,
            });
            self.stations[i].users[k].line = Some((current, start));
        }
    }

    /// The line playing's file: a song (`sound\…`, relative to `Data`) or
    /// the DJ's voice file, and whether it's a song.
    fn line_file(&self, order: &LoadOrder, i: usize) -> Option<(String, bool)> {
        let st = &self.stations[i];
        let item = st.items.get(st.current?)?;
        if let Some(path) = item.sound().and_then(|snd| sound_file(order, snd)) {
            return Some((format!("sound\\{}", path.to_ascii_lowercase()), true));
        }
        let v = self.station_ref(st.reference)?.voice?;
        let resp = item.info.responses.first()?;
        Some((dialogue::voice_path(order, &item.info, resp, v)?, false))
    }

    /// The largest attenuation distance of the sound a station's receivers
    /// are heard by: its base's own (`SNAM`, `TESObjectACTI` +0x7c), else
    /// [`ATTENUATION_MODEL`] (`00553b90`: the byte × 100, here
    /// `Sound::max_distance`).
    fn hearing_sound(&self, order: &LoadOrder, r: FormId) -> Option<f32> {
        let own = self.station_ref(r).and_then(|s| {
            let rr = order.get(s.base)?;
            let record = rr.record().ok()?;
            let id = record.get(SNAM).filter(|d| d.data.len() >= 4)?;
            Some(rr.plugin.to_global(FormId(le_u32(&id.data, 0))))
        });
        own.or_else(|| order.form_by_editor_id(ATTENUATION_MODEL))
            .and_then(|id| crate::sound::Sound::load(order, id))
            .map(|s| s.max_distance)
    }

    /// Whether a station broadcasts: `SetBroadcastState`'s word for its
    /// base, else the base's flag 0x40000000.
    fn broadcasting(&self, order: &LoadOrder, state: &GameState, r: FormId) -> bool {
        let Some(s) = self.station_ref(r) else {
            return false;
        };
        match state.more.broadcasting.get(&s.base) {
            Some(&b) => b,
            None => order
                .get(s.base)
                .is_some_and(|b| b.entry.header.flags & BROADCAST_FLAG != 0),
        }
    }

    fn stop_output(&mut self, events: &mut Vec<RadioEvent>) {
        if self.voice_playing {
            events.push(RadioEvent::StopVoice);
            self.voice_playing = false;
        }
        events.push(RadioEvent::ClearDecks);
        events.push(RadioEvent::HoldMusic(true));
    }

    /// `0061b440`: a programme from a topic (`RadioHello`, or the one a
    /// script started): the line picked (`0061a7d0`), run-immediately
    /// lines' scripts run, then a random untried link that gives a line,
    /// until none does or there are 100.
    // Translated from 0061b440 (decompiled, FalloutNV.exe 1.4.0.525)
    fn programme(
        &mut self,
        order: &LoadOrder,
        scripts: &crate::scripting::ScriptCache,
        state: &mut GameState,
        r: FormId,
        topic: FormId,
    ) -> Vec<Item> {
        let Some(s) = self.station_ref(r).cloned() else {
            return Vec::new();
        };
        let speaker = Speaker {
            reference: r,
            base: s.base,
            name: Some(s.name.clone()),
            voice: s.voice,
            race: None,
            female: false,
            factions: Vec::new(),
        };
        let mut items: Vec<Item> = Vec::new();
        let mut current = Some(topic);
        while let Some(t) = current.take() {
            if items.len() >= MOST_LINES {
                break;
            }
            let roll = self.rand();
            let Some(info) = pick(order, state, t, &speaker, roll) else {
                break;
            };
            // Run-immediately lines' two scripts run as the programme is
            // made (`0061f170(0)`, `(1)`), so the next picks see them.
            if info.flags & dialogue::RUN_IMMEDIATELY != 0 {
                for source in [&info.begin_script, &info.end_script].into_iter().flatten() {
                    crate::scripting::Runner::new(order, scripts, state).run_source(
                        source,
                        Some(r),
                        Some(r),
                    );
                }
            }
            let links = info.choices.clone();
            items.push(Item { info });
            let mut untried: Vec<FormId> = links;
            while !untried.is_empty() {
                let k = (self.rand() % untried.len() as u64) as usize;
                let next = untried.swap_remove(k);
                if pick(order, state, next, &speaker, 0).is_some() {
                    current = Some(next);
                    break;
                }
            }
        }
        items
    }

    /// `008331c0`: the Pip-Boy's output: out of range → off; the static
    /// bed; the line playing started (a song on the decks, else the DJ's
    /// voice file; neither: skipped).
    // Translated from 008331c0 (disassembly, FalloutNV.exe 1.4.0.525)
    fn pipboy_update(
        &mut self,
        order: &LoadOrder,
        i: usize,
        now: u64,
        radio_volume: f32,
        files: &mut dyn RadioFiles,
        events: &mut Vec<RadioEvent>,
    ) {
        let r = self.stations[i].reference;
        if self.stations[i].lost {
            let name = self
                .station_ref(r)
                .map(|s| s.name.clone())
                .unwrap_or_default();
            events.push(RadioEvent::Sound(SIGNAL_LOST));
            events.push(RadioEvent::Message(format!("{name} signal lost.")));
            self.lost_station = Some(r);
            self.enable(false, events);
            return;
        }
        let power = self.stations[i].power;
        if power < 100 {
            self.static_on = true;
            events.push(RadioEvent::Static(Some(static_volume(power))));
        } else if self.static_on {
            self.static_on = false;
            events.push(RadioEvent::Static(None));
        }
        let st = &self.stations[i];
        // Already playing its line (the deck or the voice), or waiting for
        // its start.
        if st.duration != 0 || st.current.is_none() || now < st.start {
            return;
        }
        let Some(item) = st.items.get(st.current.unwrap_or(0)).cloned() else {
            return;
        };
        let start = st.start;
        let voice = self.station_ref(r).and_then(|s| s.voice).and_then(|v| {
            let resp = item.info.responses.first()?;
            dialogue::voice_path(order, &item.info, resp, v)
        });
        let song = item.sound().and_then(|snd| sound_file(order, snd));
        let mut duration: i64 = 1;
        if let Some(path) = song {
            let path = format!("data\\sound\\{}", path.to_ascii_lowercase());
            let deck_path = path.trim_start_matches("data\\").to_string();
            if let Some(ms) = files.song_ms(&deck_path) {
                events.push(RadioEvent::Song {
                    path: deck_path,
                    sync: start,
                });
                duration = i64::from(ms);
            }
        } else if let Some(path) = voice {
            if let Some(ms) = files.voice_ms(&path) {
                events.push(RadioEvent::Voice {
                    path,
                    volume: radio_volume * f32::from(power) / 100.0,
                });
                self.voice_playing = true;
                duration = i64::from(ms);
            }
        }
        self.stations[i].duration = duration.max(1);
    }
}

/// A sound form's file (`SOUN` `FNAM`).
pub fn sound_file(order: &LoadOrder, sound: FormId) -> Option<String> {
    let rr = order.get(sound)?;
    let record = rr.record().ok()?;
    record
        .get(FourCC::new(b"FNAM"))
        .map(|s| s.zstring())
        .filter(|s| !s.is_empty())
}

/// `0061a7d0`: the line a station says from a topic (the station both
/// speaker and listener): lines of running quests whose conditions pass,
/// the first that isn't random, else one of the run of random ones
/// (`dialogue::choose`).
// Translated from 0061a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn pick(
    order: &LoadOrder,
    state: &GameState,
    topic: FormId,
    speaker: &Speaker,
    roll: u64,
) -> Option<Info> {
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: Some(speaker),
    };
    let mut quest_ok: HashMap<FormId, bool> = HashMap::new();
    let available = dialogue::topic_lines(order, topic)
        .into_iter()
        .filter(|info| {
            let Some(q) = info.quest else {
                return false;
            };
            let ok = *quest_ok.entry(q).or_insert_with(|| {
                state.running.contains(&q)
                    && facts.conditions_pass(
                        &crate::quest::quest_conditions(order, q),
                        speaker.reference,
                        speaker.reference,
                    )
            });
            ok && dialogue::line_available(order, info, speaker, speaker.reference, state)
        });
    dialogue::choose(available, roll)
}

/// The radio's lines in a save: `radio <on> <tuned>`, `radiofound
/// <ref>` per discovered station.
pub fn save_lines(radio: &Radio, id: &dyn Fn(FormId) -> String) -> Vec<String> {
    let mut out = vec![format!(
        "radio {} {}",
        u8::from(radio.on),
        radio.active.map_or("-".to_string(), id)
    )];
    for &r in &radio.discovered {
        out.push(format!("radiofound {}", id(r)));
    }
    out
}

/// Whether a reference is disabled now (its record's flag 0x800, or a
/// script's `Enable` / `Disable`).
pub fn disabled(order: &LoadOrder, state: &GameState, reference: FormId) -> bool {
    !crate::placement::enabled_now(order, reference, &state.disabled)
}

/// The player's place as the radio wants it.
pub fn place_of(state: &GameState) -> Place {
    Place {
        cell: state.player_cell,
        world: state.player_world,
        position: state.player_position.unwrap_or([0.0; 3]),
    }
}

/// The file a receiver plays a song from: `_mono` put before the
/// extension (`00834260`, unless the name has `_mono.` already), and, as a
/// streamed sound, the extension made `.ogg` (`00af1e00` turns `.mp3` and
/// `.wav` into `.ogg`; the games' mono songs are `.ogg` files).
// Translated from 00834260 / 00af1e00 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn mono_song(path: &str) -> String {
    let mut p = path.to_string();
    if !p.contains("_mono.") {
        if let Some(dot) = p.rfind('.') {
            p.insert_str(dot, "_mono");
        }
    }
    for ext in [".mp3", ".wav"] {
        if let Some(at) = p.find(ext) {
            p.replace_range(at..at + ext.len(), ".ogg");
            break;
        }
    }
    p
}

/// A reference's worldspace and position as placed (a station's position
/// reference, `XRDO`).
fn position_of(order: &LoadOrder, r: FormId) -> Option<(Option<FormId>, [f32; 3])> {
    let rr = order.get(r)?;
    let record = rr.record().ok()?;
    let d = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 12)?;
    Some((
        order.world_of(&rr),
        [le_f32(&d.data, 0), le_f32(&d.data, 4), le_f32(&d.data, 8)],
    ))
}

/// Whether the player is the listener a condition asks about (unused by
/// the radio: both ends are the station).
pub const LISTENER: FormId = PLAYER_REF;

#[cfg(test)]
mod tests {
    use super::*;

    fn data(radius: f32, range: u32, static_pct: f32) -> RadioData {
        RadioData {
            radius,
            range,
            static_pct,
            position: None,
        }
    }

    /// A receiver's song: the `_mono` file, streamed as `.ogg`.
    #[test]
    fn a_receivers_song_is_its_mono_ogg() {
        assert_eq!(
            mono_song("sound\\songs\\radionv\\mus_lazy_day_blues.mp3"),
            "sound\\songs\\radionv\\mus_lazy_day_blues_mono.ogg"
        );
        assert_eq!(mono_song("sound\\a_mono.ogg"), "sound\\a_mono.ogg");
        assert_eq!(mono_song("sound\\b.wav"), "sound\\b_mono.ogg");
    }

    /// `00833d00`: clear to 0.9 of the radius (the default static), then
    /// truncated steps down to nothing at the edge; a station's own static
    /// percentage moves the start (Black Mountain's 5 %: from 0.95).
    #[test]
    fn signal_strength_by_distance() {
        let d = data(50_000.0, 0, 0.0);
        assert_eq!(strength(&d, 10_000.0), 100);
        assert_eq!(strength(&d, 45_000.0), 100);
        assert_eq!(strength(&d, 47_500.0), 50);
        assert_eq!(strength(&d, 47_499.0), 51);
        let bm = data(50_000.0, 0, 5.0);
        assert_eq!(strength(&bm, 47_000.0), 100);
        assert_eq!(strength(&bm, 48_750.0), 50);
        // Everywhere stations are always clear.
        assert_eq!(strength(&data(0.0, 1, 0.0), 1.0e9), 100);
        assert!((static_volume(100)).abs() < 1e-6);
        assert!((static_volume(0) - 1.0).abs() < 1e-6);
        assert!((static_volume(84) - 0.16f32.powf(0.75)).abs() < 1e-6);
    }

    fn station(r: u32, range: u32, world: Option<u32>, cell: u32, pos: [f32; 3]) -> StationRef {
        StationRef {
            reference: FormId(r),
            base: FormId(r + 1),
            name: format!("S{r}"),
            data: data(1000.0, range, 0.0),
            world: world.map(FormId),
            cell: Some(FormId(cell)),
            position: pos,
            base_flags: RADIO_FLAG,
            voice: None,
        }
    }

    /// `004ff1a0`: radius stations by 3D distance in their worldspace,
    /// everywhere stations always, current-cell ones in their cell;
    /// disabled stations only for an interior player.
    #[test]
    fn stations_in_range_by_kind() {
        let s = [
            station(1, 0, Some(10), 100, [0.0, 0.0, 0.0]),
            station(3, 1, None, 200, [0.0; 3]),
            station(5, 4, None, 300, [0.0; 3]),
        ];
        let outside = Place {
            cell: Some(FormId(100)),
            world: Some(FormId(10)),
            position: [600.0, 0.0, 600.0],
        };
        let none = |_: FormId| false;
        let pos = |_: FormId| None;
        let hits: Vec<u32> = in_range(&s, &outside, &none, &pos)
            .iter()
            .map(|(r, _)| r.0)
            .collect();
        assert_eq!(hits, [1, 3]);
        let far = Place {
            position: [800.0, 0.0, 800.0],
            ..outside
        };
        let hits: Vec<u32> = in_range(&s, &far, &none, &pos)
            .iter()
            .map(|(r, _)| r.0)
            .collect();
        assert_eq!(hits, [3]);
        let inside = Place {
            cell: Some(FormId(300)),
            world: None,
            position: [0.0; 3],
        };
        let all_disabled = |_: FormId| true;
        let hits: Vec<u32> = in_range(&s, &inside, &all_disabled, &pos)
            .iter()
            .map(|(r, _)| r.0)
            .collect();
        assert_eq!(hits, [3, 5]);
        assert!(in_range(&s, &outside, &all_disabled, &pos).is_empty());
    }

    /// `0079bea0`: discovered, enabled, Pip-Boy stations; in range first,
    /// then by name; the tuned one marked. A click on an in-range row turns
    /// it on and tunes it; again turns it off; out of range does nothing.
    #[test]
    fn the_radio_list_and_its_clicks() {
        let mut radio = Radio::default();
        let mut a = station(1, 1, None, 1, [0.0; 3]);
        a.name = "Radio New Vegas".into();
        let mut b = station(3, 1, None, 1, [0.0; 3]);
        b.name = "Mojave Music Radio".into();
        let mut c = station(5, 1, None, 1, [0.0; 3]);
        c.name = "Black Mountain Radio".into();
        let mut hidden = station(7, 1, None, 1, [0.0; 3]);
        hidden.base_flags |= NON_PIPBOY_FLAG;
        radio.stations_placed = Some(vec![a, b, c, hidden]);
        radio.discovered = vec![FormId(1), FormId(3), FormId(5), FormId(7)];
        radio.in_range = vec![FormId(1), FormId(3), FormId(7)];
        let rows = radio.rows(&|_| false);
        let names: Vec<&str> = rows.iter().map(|r| r.1.as_str()).collect();
        assert_eq!(
            names,
            [
                "Mojave Music Radio",
                "Radio New Vegas",
                "Black Mountain Radio"
            ]
        );
        assert!(!rows[2].2);
        let ev = radio.click_row(FormId(1), 10_000);
        assert!(radio.on && radio.active == Some(FormId(1)));
        assert!(ev.contains(&RadioEvent::HoldMusic(true)));
        assert!(ev.contains(&RadioEvent::ClearDecks));
        let st = radio
            .stations
            .iter()
            .find(|s| s.reference == FormId(1))
            .unwrap();
        assert!(st.start <= 10_000 && st.duration == 1);
        assert!(radio.rows(&|_| false)[1].3);
        radio.click_row(FormId(1), 11_000);
        assert!(!radio.on && radio.active.is_none());
        assert!(radio.click_row(FormId(5), 12_000).is_empty());
        assert!(!radio.on);
    }
}
