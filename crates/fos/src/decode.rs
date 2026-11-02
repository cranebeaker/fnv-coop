//! Decoders for the parts of a save researched so far (docs/FOS_SAVES.md).
//! Each reads one change form's or global data entry's pipe buffer and
//! fails unless it ends exactly where the data does.

use crate::{save_type as t, ChangeForm, Error, GlobalData, Pipe, RefId, Result};

mod actor;
mod extra;
mod package;

pub(crate) use actor::{actor_form_captured, reference_captured};

pub use actor::{
    actor_form, modifiers, projectile, reference, Actor, ActorFields, ActorForm, Mobile, Player,
    Process, Reference, ReferenceData, ACTOR_EXTRA, REFR_EXTRA,
};
pub use extra::{
    active_effects, extra_list, inventory, kind as extra_kind, saved_under, Context, Extra,
    ItemChange, Lock, Teleport,
};

/// `QuestFlag` (Xbox PDB): the quest's run-time flags (`CHANGE_QUEST_FLAGS`).
pub mod quest_flag {
    pub const ENABLED: u8 = 0x01;
    pub const COMPLETED: u8 = 0x02;
    pub const ALLOW_REPEATS: u8 = 0x04;
    pub const ALLOW_REPEAT_STAGES: u8 = 0x08;
    pub const STARTS_ENABLED: u8 = 0x10;
    pub const DISPLAYED_IN_HUD: u8 = 0x20;
    pub const FAILED: u8 = 0x40;
}

/// A quest's change form (`TESQuest::SaveGame` (Xbox PDB), `0060e810`;
/// read back by `0060eaf0`).
#[derive(Debug, Clone, PartialEq)]
pub struct Quest {
    pub form_flags: Option<u32>,
    /// [`quest_flag`]s.
    pub flags: Option<u8>,
    pub script_delay: Option<f32>,
    pub stages: Option<Vec<Stage>>,
    pub script: Option<ScriptLocals>,
    pub objectives: Option<Vec<Objective>>,
}

impl Quest {
    /// The stage the game makes current on loading: the highest one done
    /// (`0060d670`).
    pub fn current_stage(&self) -> Option<u8> {
        self.stages
            .as_ref()?
            .iter()
            .filter(|s| s.done)
            .map(|s| s.index)
            .max()
    }
}

/// `TESQuestStage` (Xbox PDB): `QUEST_STAGE_DATA` (index, done) and its
/// items.
#[derive(Debug, Clone, PartialEq)]
pub struct Stage {
    pub index: u8,
    pub done: bool,
    pub items: Vec<StageItem>,
}

/// `TESQuestStageItem` (Xbox PDB): its index and the date its log entry
/// was written, if it was.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StageItem {
    pub index: u8,
    pub log_date: Option<Date>,
}

/// `Date` (Xbox PDB): `sDate` (day of the year) and `sYear`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub day: u16,
    pub year: u16,
}

/// `BGSQuestObjective` (Xbox PDB): index and `QUEST_OBJECTIVE_STATE`
/// (0 dormant, 1 displayed, 2 completed, 3 completed and displayed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Objective {
    pub index: u32,
    pub state: u32,
}

impl Objective {
    pub fn completed(&self) -> bool {
        self.state & 2 != 0
    }

    pub fn displayed(&self) -> bool {
        self.state & 1 != 0
    }
}

/// `ScriptLocals` (Xbox PDB) as saved (`005a9db0`; read by `005a9f20`).
#[derive(Debug, Clone, PartialEq)]
pub struct ScriptLocals {
    pub variables: Vec<Variable>,
    /// The 8 bytes behind `m_pScriptEffectData`, when there are any.
    pub effect_data: Option<[u8; 8]>,
    /// The flag 0x1000 byte; the loader reads it only for versions over
    /// 0x14.
    pub flag: Option<u8>,
}

/// A script variable: its index in the script and its value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Variable {
    pub id: u32,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Number(f64),
    /// A reference variable (bit 0x80000000 on the id).
    Ref(RefId),
}

/// Reads values of these sizes, each followed by `|`, and drops them.
fn skip(p: &mut Pipe<'_>, sizes: &[usize]) -> Result<()> {
    for &n in sizes {
        p.bytes(n)?;
    }
    Ok(())
}

fn form_flags(p: &mut Pipe<'_>, flags: u32) -> Result<Option<u32>> {
    if flags & 1 != 0 {
        Ok(Some(p.u32()?))
    } else {
        Ok(None)
    }
}

fn counted<'a, T>(
    p: &mut Pipe<'a>,
    mut item: impl FnMut(&mut Pipe<'a>) -> Result<T>,
) -> Result<Vec<T>> {
    let n = p.vsval()?;
    // Every item takes at least one byte and its '|'.
    if n as usize > p.remaining() / 2 {
        return Err(Error::new(p.position(), format!("{n} items don't fit")));
    }
    (0..n).map(|_| item(p)).collect()
}

fn pipe<'a>(cf: &ChangeForm<'a>) -> Pipe<'a> {
    Pipe::new(cf.data, cf.data_offset)
}

fn expect_type(cf: &ChangeForm<'_>, save_types: &[u8]) -> Result<()> {
    if save_types.contains(&cf.save_type) {
        Ok(())
    } else {
        Err(Error::new(
            cf.offset,
            format!("change form of save type {}", cf.save_type),
        ))
    }
}

/// Script locals (`005a9db0`), version as the change form's.
pub fn script_locals(p: &mut Pipe<'_>, version: u8) -> Result<ScriptLocals> {
    let variables = counted(p, |p| {
        let id = p.u32()?;
        let value = if id & 0x8000_0000 != 0 {
            Value::Ref(p.ref_id()?)
        } else {
            Value::Number(p.f64()?)
        };
        Ok(Variable {
            id: id & 0x7FFF_FFFF,
            value,
        })
    })?;
    let effect_data = if p.u8()? != 0 {
        let mut a = [0; 8];
        a.copy_from_slice(p.bytes(8)?);
        Some(a)
    } else {
        None
    };
    let flag = if version > 0x14 { Some(p.u8()?) } else { None };
    Ok(ScriptLocals {
        variables,
        effect_data,
        flag,
    })
}

/// A `QUST` change form.
pub fn quest(cf: &ChangeForm<'_>) -> Result<Quest> {
    expect_type(cf, &[t::QUST])?;
    let mut p = pipe(cf);
    let f = cf.flags;
    let form_flags = form_flags(&mut p, f)?;
    let flags = if f & 0x2 != 0 { Some(p.u8()?) } else { None };
    let script_delay = if f & 0x4 != 0 { Some(p.f32()?) } else { None };
    let stages = if f & 0x8000_0000 != 0 {
        Some(counted(&mut p, |p| {
            let index = p.u8()?;
            let done = p.u8()? != 0;
            let items = counted(p, |p| {
                let index = p.u8()?;
                let log_date = if p.u8()? != 0 {
                    let b = p.bytes(4)?;
                    Some(Date {
                        day: u16::from_le_bytes([b[0], b[1]]),
                        year: u16::from_le_bytes([b[2], b[3]]),
                    })
                } else {
                    None
                };
                Ok(StageItem { index, log_date })
            })?;
            Ok(Stage { index, done, items })
        })?)
    } else {
        None
    };
    let script = if f & 0x4000_0000 != 0 {
        Some(script_locals(&mut p, cf.version)?)
    } else {
        None
    };
    let objectives = if f & 0x2000_0000 != 0 {
        Some(counted(&mut p, |p| {
            Ok(Objective {
                index: p.u32()?,
                state: p.u32()?,
            })
        })?)
    } else {
        None
    };
    p.finish("quest")?;
    Ok(Quest {
        form_flags,
        flags,
        script_delay,
        stages,
        script,
        objectives,
    })
}

/// Global data 3, `GLOBAL_DATA_GLOBALS` (`0084cc90`): each `GLOB` and its
/// value. Constant globals aren't saved.
pub fn globals(g: &GlobalData<'_>) -> Result<Vec<(RefId, f32)>> {
    let mut p = Pipe::new(g.data, g.offset);
    let out = counted(&mut p, |p| Ok((p.ref_id()?, p.f32()?)))?;
    p.finish("globals")?;
    Ok(out)
}

/// Global data 0, `GLOBAL_DATA_MISC_STATS` (`004d5fa0`): the misc
/// statistics in the order of the game's name table (`01189280`).
pub fn misc_stats(g: &GlobalData<'_>) -> Result<Vec<u32>> {
    let mut p = Pipe::new(g.data, g.offset);
    let n = p.u32()?;
    if n as usize > p.remaining() / 5 {
        return Err(Error::new(g.offset, format!("{n} misc stats don't fit")));
    }
    let out = (0..n).map(|_| p.u32()).collect::<Result<Vec<_>>>()?;
    p.finish("misc stats")?;
    Ok(out)
}

/// Global data 8, `GLOBAL_DATA_WEATHER` (`Sky::SaveGame` (Xbox PDB),
/// `0063eb70`): the sky's weathers and timing. Member names are the Xbox
/// prototype's `Sky`.
#[derive(Debug, Clone, PartialEq)]
pub struct Sky {
    /// `pCurrentWeather`, `pLastWeather`, `pDefaultWeather`,
    /// `pOverrideWeather`.
    pub current: RefId,
    pub last: RefId,
    pub default: RefId,
    pub override_weather: RefId,
    /// `fCurrentGameHour`, `fLastWeatherUpdate`, `fCurrentWeatherPct`.
    pub hour: f32,
    pub last_update: f32,
    pub weather_pct: f32,
    /// `uiFlags`.
    pub flags: u32,
    /// `fAccelBeginPct`.
    pub accel_begin_pct: f32,
    /// `WaterFogColor`, `fFogHeight`, `fFogPower`.
    pub water_fog_color: [f32; 3],
    pub fog_height: f32,
    pub fog_power: f32,
    /// `eMode`.
    pub mode: u32,
}

pub fn sky(g: &GlobalData<'_>) -> Result<Sky> {
    let mut p = Pipe::new(g.data, g.offset);
    let out = Sky {
        current: p.ref_id()?,
        last: p.ref_id()?,
        default: p.ref_id()?,
        override_weather: p.ref_id()?,
        hour: p.f32()?,
        last_update: p.f32()?,
        weather_pct: p.f32()?,
        flags: p.u32()?,
        accel_begin_pct: p.f32()?,
        water_fog_color: [p.f32()?, p.f32()?, p.f32()?],
        fog_height: p.f32()?,
        fog_power: p.f32()?,
        mode: p.u32()?,
    };
    p.finish("weather")?;
    Ok(out)
}

/// Global data 10, `GLOBAL_DATA_RADIO` (`FalloutRadio::SaveGame` (Xbox
/// PDB), `008366e0`).
#[derive(Debug, Clone, PartialEq)]
pub struct Radio {
    /// `011dd428`, not interpreted.
    pub value: u32,
    /// The Pip-Boy radio is on (`011dd434`).
    pub on: bool,
    /// A string (`011dd448`).
    pub text: String,
    /// The stations' saved states (`011dd58c`, `00835f10`): the station
    /// and four bytes.
    pub stations: Vec<(RefId, [u8; 4])>,
    /// The stations whose lines are under way (`011dd554`, `00836600`).
    pub playing: Vec<RefId>,
    /// The station tuned to (`011dd42c`) and one that went out of range
    /// (`011dd430`).
    pub active: RefId,
    pub lost: RefId,
    /// The discovered stations (`011dd59c`), in order found.
    pub discovered: Vec<RefId>,
}

pub fn radio(g: &GlobalData<'_>) -> Result<Radio> {
    let mut p = Pipe::new(g.data, g.offset);
    let value = p.u32()?;
    let on = p.u8()? != 0;
    let text = p.wstr()?;
    let stations = counted(&mut p, |p| {
        Ok((p.ref_id()?, [p.u8()?, p.u8()?, p.u8()?, p.u8()?]))
    })?;
    let playing = counted(&mut p, |p| {
        let station = p.ref_id()?;
        // `00836110`: time into the line, two values, two bytes, a value,
        // the lines left, and the dialogue under way if any.
        skip(p, &[4, 4, 1, 1, 4])?;
        counted(p, |p| p.ref_id())?;
        if p.u8()? != 0 {
            counted(p, package::dialogue)?;
            p.u16()?;
        }
        Ok(station)
    })?;
    let active = p.ref_id()?;
    let lost = p.ref_id()?;
    let discovered = counted(&mut p, |p| p.ref_id())?;
    p.finish("radio")?;
    Ok(Radio {
        value,
        on,
        text,
        stations,
        playing,
        active,
        lost,
        discovered,
    })
}

/// Global data 1, `GLOBAL_DATA_LOCATION` (`SaveLocationData` (Xbox PDB),
/// `0084c490`).
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    /// `TESDataHandler::iNextID` (Xbox PDB): the next created form id.
    pub next_created_id: u32,
    /// `TES::pWorldSpace` (Xbox PDB).
    pub worldspace: RefId,
    /// `TES::iCurrentGridX`, `iCurrentGridY` (Xbox PDB).
    pub grid: [i32; 2],
    /// The player's worldspace, or parent cell when inside.
    pub player_space: RefId,
    pub player_position: [f32; 3],
    /// `LoadingMenu::SaveGame` (Xbox PDB) `0078d6b0`: a refID, three
    /// 4-byte values and a byte, not interpreted.
    pub loading_menu: (RefId, [u32; 3], u8),
}

pub fn location(g: &GlobalData<'_>) -> Result<Location> {
    let mut p = Pipe::new(g.data, g.offset);
    let next_created_id = p.u32()?;
    let worldspace = p.ref_id()?;
    let grid = [p.i32()?, p.i32()?];
    let player_space = p.ref_id()?;
    let b = p.bytes(12)?;
    let f = |i: usize| f32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
    let player_position = [f(0), f(4), f(8)];
    let loading_menu = (p.ref_id()?, [p.u32()?, p.u32()?, p.u32()?], p.u8()?);
    p.finish("location")?;
    Ok(Location {
        next_created_id,
        worldspace,
        grid,
        player_space,
        player_position,
        loading_menu,
    })
}

/// The data written before a reference's or cell's own data
/// (`BGSSaveLoadInitialData` (Xbox PDB), `0084eb80`; kind from `0084e730`).
#[derive(Debug, Clone, PartialEq)]
pub enum InitialData {
    None,
    /// An exterior cell: worldspace (index into the worldspace id array),
    /// grid square and detach time. `short` tells the 10-byte form.
    ExteriorCell {
        worldspace: u16,
        x: i16,
        y: i16,
        detach_time: u32,
        short: bool,
    },
    InteriorCell {
        detach_time: u32,
    },
    /// A moved reference: its cell (or worldspace outside), position and
    /// rotation (radians).
    Location {
        space: RefId,
        position: [f32; 3],
        rotation: [f32; 3],
    },
    /// A reference created in game, with its flags and base object.
    Created {
        space: RefId,
        position: [f32; 3],
        rotation: [f32; 3],
        flags: u8,
        base: RefId,
    },
    /// A reference moved to another cell, with where the editor placed it.
    Moved {
        space: RefId,
        position: [f32; 3],
        rotation: [f32; 3],
        editor_cell: RefId,
        editor_grid: [i16; 2],
    },
}

impl InitialData {
    /// Where a reference stands, if this says.
    pub fn place(&self) -> Option<(RefId, [f32; 3], [f32; 3])> {
        match *self {
            InitialData::Location {
                space,
                position,
                rotation,
            }
            | InitialData::Created {
                space,
                position,
                rotation,
                ..
            }
            | InitialData::Moved {
                space,
                position,
                rotation,
                ..
            } => Some((space, position, rotation)),
            _ => None,
        }
    }
}

fn le_ref(b: &[u8]) -> RefId {
    RefId(u32::from(b[0]) << 16 | u32::from(b[1]) << 8 | u32::from(b[2]))
}

fn le_f32s(b: &[u8]) -> [f32; 3] {
    let f = |i: usize| f32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
    [f(0), f(4), f(8)]
}

fn i16_at(b: &[u8], i: usize) -> i16 {
    i16::from_le_bytes([b[i], b[i + 1]])
}

fn u32_at(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}

/// Whether a save type is a reference (`0084e730` with `00564900`).
pub fn is_reference(save_type: u8) -> bool {
    save_type <= t::PFLA
}

/// Reads the initial data at the start of a change form.
pub fn initial_data(cf: &ChangeForm<'_>, p: &mut Pipe<'_>) -> Result<InitialData> {
    let f = cf.flags;
    if is_reference(cf.save_type) {
        let size = if cf.ref_id.is_created() {
            31
        } else if f & 0x8 != 0 {
            34
        } else if f & 0x6 != 0 {
            27
        } else {
            return Ok(InitialData::None);
        };
        let b = p.bytes(size)?;
        let (space, position, rotation) = (le_ref(b), le_f32s(&b[3..]), le_f32s(&b[15..]));
        return Ok(match size {
            31 => InitialData::Created {
                space,
                position,
                rotation,
                flags: b[27],
                base: le_ref(&b[28..]),
            },
            34 => InitialData::Moved {
                space,
                position,
                rotation,
                editor_cell: le_ref(&b[27..]),
                editor_grid: [i16_at(b, 30), i16_at(b, 32)],
            },
            _ => InitialData::Location {
                space,
                position,
                rotation,
            },
        });
    }
    if cf.save_type == t::CELL && f & 0x4000_0000 != 0 {
        if f & 0x3000_0000 != 0 {
            let short = f & 0x2000_0000 == 0;
            let b = p.bytes(if short { 10 } else { 8 })?;
            let worldspace = u16::from_le_bytes([b[0], b[1]]);
            return Ok(if short {
                InitialData::ExteriorCell {
                    worldspace,
                    x: i16_at(b, 2),
                    y: i16_at(b, 4),
                    detach_time: u32_at(b, 6),
                    short,
                }
            } else {
                InitialData::ExteriorCell {
                    worldspace,
                    x: i16::from(b[2] as i8),
                    y: i16::from(b[3] as i8),
                    detach_time: u32_at(b, 4),
                    short,
                }
            });
        }
        let b = p.bytes(4)?;
        return Ok(InitialData::InteriorCell {
            detach_time: u32_at(b, 0),
        });
    }
    Ok(InitialData::None)
}

/// The start of a reference's change form: initial data and, with
/// `CHANGE_REFR_HAVOK_MOVE`, the Havok block (`00562de0`), then the rest.
#[derive(Debug, Clone)]
pub struct ReferenceStart<'a> {
    pub initial: InitialData,
    pub havok: Option<&'a [u8]>,
    pub rest: Pipe<'a>,
}

pub fn reference_start<'a>(cf: &ChangeForm<'a>) -> Result<ReferenceStart<'a>> {
    if !is_reference(cf.save_type) {
        return Err(Error::new(cf.offset, "not a reference"));
    }
    let mut p = pipe(cf);
    let initial = initial_data(cf, &mut p)?;
    let havok = if cf.flags & 0x4 != 0 {
        let n = p.vsval()? as usize;
        if n > p.remaining() {
            return Err(Error::new(p.position(), "Havok block runs past the data"));
        }
        // The block is the Havok writer's own pipe values; take it whole.
        let start = p.position();
        let all = cf.data;
        let block = &all[start..start + n];
        p = Pipe::new(&all[start + n..], cf.data_offset + start + n);
        Some(block)
    } else {
        None
    };
    Ok(ReferenceStart {
        initial,
        havok,
        rest: p,
    })
}

/// How far a change form decodes.
#[derive(Debug, Clone, PartialEq)]
pub enum Coverage {
    /// Decoded to exactly its length.
    Exact,
    /// Not decoded: the reason.
    Skipped(&'static str),
    /// Decoding failed or didn't end with the data.
    Failed(Error),
}

/// A cell's local map fog (`CELL_SEENDATA`): 256 seen points (32 bytes,
/// `SeenData` (Xbox PDB) `0087a0c0`) for an exterior cell, or per interior
/// section its `cSectionX`, `cSectionY` and points (`IntSeenData`,
/// `0087a4e0`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Seen {
    Exterior([u8; 32]),
    Interior(Vec<(i8, i8, [u8; 32])>),
}

/// A `CELL` change form (`TESObjectCELL::SaveGame` (Xbox PDB),
/// `00555630`).
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub initial: InitialData,
    pub form_flags: Option<u32>,
    /// `CELL_FLAGS`: the cell's flags & 0x60 with its second flags.
    pub flags: Option<u8>,
    pub seen: Option<Seen>,
    /// `CELL_FULLNAME`.
    pub name: Option<String>,
    /// `CELL_OWNERSHIP`.
    pub owner: Option<RefId>,
}

fn bits32(b: &[u8]) -> [u8; 32] {
    let mut a = [0; 32];
    a.copy_from_slice(b);
    a
}

/// Reads a `CELL` change form; the seen data's layout depends on whether
/// the cell is inside (`IntSeenData`) or out (`SeenData`): pass it when
/// known, else both are tried and exactly one must fit.
pub fn cell_data(cf: &ChangeForm<'_>, interior: Option<bool>) -> Result<Cell> {
    expect_type(cf, &[t::CELL])?;
    let attempt = |interior: bool| -> Result<Cell> {
        let mut p = pipe(cf);
        let initial = initial_data(cf, &mut p)?;
        let f = cf.flags;
        let form_flags = form_flags(&mut p, f)?;
        let flags = if f & 0x2 != 0 { Some(p.u8()?) } else { None };
        let seen = if f & 0x8000_0000 != 0 {
            Some(if interior {
                Seen::Interior(counted(&mut p, |p| {
                    let x = p.u8()? as i8;
                    let y = p.u8()? as i8;
                    Ok((x, y, bits32(p.bytes(32)?)))
                })?)
            } else {
                Seen::Exterior(bits32(p.bytes(32)?))
            })
        } else {
            None
        };
        let name = if f & 0x4 != 0 { Some(p.wstr()?) } else { None };
        let owner = if f & 0x8 != 0 {
            Some(p.ref_id()?)
        } else {
            None
        };
        p.finish("cell")?;
        Ok(Cell {
            initial,
            form_flags,
            flags,
            seen,
            name,
            owner,
        })
    };
    match interior {
        Some(i) => attempt(i),
        None if cf.flags & 0x8000_0000 == 0 => attempt(false),
        None => match (attempt(false), attempt(true)) {
            (Ok(a), Err(_)) | (Err(_), Ok(a)) => Ok(a),
            (Err(e), Err(_)) => Err(e),
            (Ok(_), Ok(_)) => Err(Error::new(
                cf.offset,
                "cell seen data fits both the interior and exterior layouts",
            )),
        },
    }
}

/// [`cell_data`]'s initial data.
pub fn cell(cf: &ChangeForm<'_>, interior: Option<bool>) -> Result<InitialData> {
    cell_data(cf, interior).map(|c| c.initial)
}

/// The face's FaceGen coefficients as `NPC_FACE` saves them (`00608f00`):
/// geometry symmetric and asymmetric, texture symmetric and asymmetric.
/// The game takes the counts from the NPC's own arrays at run time; these
/// are the sizes of the record's `FGGS`, `FGGA` and `FGTS` (no save seen
/// so far has `NPC_FACE`, so they are not checked against one).
pub const FACE_COEFFICIENTS: [usize; 4] = [50, 30, 50, 0];

/// `NPC_FACE`: a byte (whether the face is the NPC's own), the
/// coefficients, two refIDs (hair, eyes), the hair length and colour, and
/// the head parts (`00608f00`).
fn npc_face(p: &mut Pipe<'_>) -> Result<()> {
    p.u8()?;
    for &n in &FACE_COEFFICIENTS {
        for _ in 0..n {
            p.u32()?;
        }
    }
    p.ref_id()?;
    p.ref_id()?;
    skip(p, &[4, 4])?;
    counted(p, |p| p.ref_id())?;
    Ok(())
}

/// What an actor base's change data holds (`TESActorBase::SaveGame` (Xbox
/// PDB), `005f1f30`, then `TESNPC` `00608f00` or `TESCreature`
/// `005fb330`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ActorBase {
    pub form_flags: Option<u32>,
    /// `ACTOR_BASE_DATA`: the 24 bytes of `ACBS` (level at 8, karma at 16).
    pub data: Option<[u8; 24]>,
    /// `ACTOR_BASE_ATTRIBUTES`: S.P.E.C.I.A.L.
    pub attributes: Option<[u8; 7]>,
    /// `ACTOR_BASE_FULLNAME`.
    pub name: Option<String>,
    /// `NPC_SKILLS`: 14 skill values then 14 offsets.
    pub skills: Option<[u8; 28]>,
    /// `NPC_CLASS`.
    pub class: Option<RefId>,
    /// `NPC_GENDER`.
    pub gender: Option<u8>,
}

impl ActorBase {
    /// The level in `ACBS`.
    pub fn level(&self) -> Option<i16> {
        self.data.map(|d| i16::from_le_bytes([d[8], d[9]]))
    }

    /// The karma in `ACBS`.
    pub fn karma(&self) -> Option<f32> {
        self.data
            .map(|d| f32::from_le_bytes([d[16], d[17], d[18], d[19]]))
    }
}

/// An actor base's change data, which a leveled actor's extra data also
/// carries.
pub(crate) fn actor_base_data(p: &mut Pipe<'_>, f: u32, npc: bool) -> Result<ActorBase> {
    let mut out = ActorBase {
        form_flags: form_flags(p, f)?,
        ..ActorBase::default()
    };
    if f & 0x2 != 0 {
        let mut a = [0; 24];
        a.copy_from_slice(p.bytes(24)?);
        out.data = Some(a);
    }
    if f & 0x10 != 0 {
        counted(p, |p| p.ref_id())?;
        counted(p, |p| p.ref_id())?;
    }
    if f & 0x4 != 0 {
        let mut a = [0; 7];
        a.copy_from_slice(p.bytes(7)?);
        out.attributes = Some(a);
    }
    if f & 0x8 != 0 {
        p.bytes(20)?;
    }
    if f & 0x20 != 0 {
        out.name = Some(p.wstr()?);
    }
    if npc {
        if f & 0x200 != 0 {
            let mut a = [0; 28];
            a.copy_from_slice(p.bytes(28)?);
            out.skills = Some(a);
        }
        if f & 0x400 != 0 {
            out.class = Some(p.ref_id()?);
        }
        if f & 0x200_0000 != 0 {
            p.ref_id()?;
            p.ref_id()?;
        }
        if f & 0x800 != 0 {
            npc_face(p)?;
        }
        if f & 0x100_0000 != 0 {
            out.gender = Some(p.u8()?);
        }
    } else if f & 0x200 != 0 {
        p.u8()?;
        p.u8()?;
        p.u8()?;
    }
    Ok(out)
}

/// Reads an `NPC_` or `CREA` change form.
pub fn actor_base_form(cf: &ChangeForm<'_>) -> Result<ActorBase> {
    expect_type(cf, &[t::NPC_, t::CREA])?;
    let mut p = pipe(cf);
    let out = actor_base_data(&mut p, cf.flags, cf.save_type == t::NPC_)?;
    p.finish("actor base")?;
    Ok(out)
}

/// An `NPC_` or `CREA` change form's full name, when saved.
pub fn actor_base(cf: &ChangeForm<'_>) -> Result<Option<String>> {
    actor_base_form(cf).map(|b| b.name)
}

/// A `FACT` change form (`005fd690`).
#[derive(Debug, Clone, PartialEq)]
pub struct Faction {
    /// Reactions: faction, modifier, group reaction (`0048c9d0`).
    pub reactions: Option<Vec<(RefId, i32, i32)>>,
    pub flags: Option<u32>,
    /// `iMajorCrime`, `iMinorCrime` (Xbox PDB).
    pub crimes: Option<(i32, i32)>,
}

pub fn faction(cf: &ChangeForm<'_>) -> Result<Faction> {
    expect_type(cf, &[t::FACT])?;
    let f = cf.flags;
    let mut p = pipe(cf);
    form_flags(&mut p, f)?;
    let reactions = if f & 0x4 != 0 {
        Some(counted(&mut p, |p| Ok((p.ref_id()?, p.i32()?, p.i32()?)))?)
    } else {
        None
    };
    let flags = if f & 0x2 != 0 { Some(p.u32()?) } else { None };
    let crimes = if f & 0x8000_0000 != 0 {
        Some((p.i32()?, p.i32()?))
    } else {
        None
    };
    p.finish("faction")?;
    Ok(Faction {
        reactions,
        flags,
        crimes,
    })
}

/// A `CLAS` change form (`005f7150`): the four tag skills (actor values,
/// -1 unused).
pub fn class(cf: &ChangeForm<'_>) -> Result<Option<[i32; 4]>> {
    expect_type(cf, &[t::CLAS])?;
    let mut p = pipe(cf);
    form_flags(&mut p, cf.flags)?;
    let tags = if cf.flags & 0x2 != 0 {
        Some([p.i32()?, p.i32()?, p.i32()?, p.i32()?])
    } else {
        None
    };
    p.finish("class")?;
    Ok(tags)
}

/// A `CHAL` (`005f5780`, `CHALLENGE_PROGRESS` (Xbox PDB): `iProgress`,
/// `nProgressFlags`) or `REPU` (`00616660`: fame, infamy as raw bits)
/// change form: two 4-byte values, always written.
pub fn pair(cf: &ChangeForm<'_>) -> Result<(u32, u32)> {
    expect_type(cf, &[t::CHAL, t::REPU])?;
    let mut p = pipe(cf);
    form_flags(&mut p, cf.flags)?;
    let pair = (p.u32()?, p.u32()?);
    p.finish("challenge or reputation")?;
    Ok(pair)
}

/// Change forms that carry only their flags (`TESForm::SaveGame`,
/// `00484d60`): `INFO` (said once) and `NOTE` (read) among them.
pub fn flags_only(cf: &ChangeForm<'_>) -> Result<Option<u32>> {
    let mut p = pipe(cf);
    let flags = form_flags(&mut p, cf.flags)?;
    p.finish("form flags")?;
    Ok(flags)
}

/// What a base form's change form holds (`TESForm::SaveGame` `00484d60`
/// and the small `SaveGame`s of the base form classes).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BaseForm {
    pub form_flags: Option<u32>,
    /// `CHANGE_BASE_OBJECT_VALUE` (`0048ea40`).
    pub value: Option<u32>,
    /// `CHANGE_BASE_OBJECT_FULLNAME` (`ACTI`, `TACT`, `TERM`, `FURN`:
    /// `00511860`).
    pub name: Option<String>,
    /// `CHANGE_BOOK_TEACHES_SKILL` (`00515340`).
    pub teaches: Option<u8>,
    /// Forms a list gained in game (`FLST` `00590080`, raw `u32` count)
    /// or a leveled list's added entries (`00488c50`: form, level, count,
    /// health or -1).
    pub added: Option<Vec<RefId>>,
    /// `TACT` speaker (`004ff800`, `CHANGE_TALKING_ACTIVATOR_SPEAKER`) or
    /// `WATR` `WATER_REMAPPED` (`005806e0`).
    pub other: Option<RefId>,
}

/// Reads the change form of a base object, list or other small form; an
/// error for types this crate doesn't read.
pub fn base_form(cf: &ChangeForm<'_>) -> Result<BaseForm> {
    let f = cf.flags;
    let mut p = pipe(cf);
    let mut out = BaseForm {
        form_flags: form_flags(&mut p, f)?,
        ..BaseForm::default()
    };
    let value = |p: &mut Pipe<'_>, out: &mut BaseForm| -> Result<()> {
        if f & 0x2 != 0 {
            out.value = Some(p.u32()?);
        }
        Ok(())
    };
    match cf.save_type {
        // ARMO, CLOT, LIGH, MISC, WEAP, AMMO, KEYM, IMOD, CHIP, CMNY.
        15 | 17 | 21 | 22 | 26 | 27 | 28 | t::IMOD | t::CHIP | t::CMNY => value(&mut p, &mut out)?,
        t::BOOK => {
            value(&mut p, &mut out)?;
            if f & 0x20 != 0 {
                out.teaches = Some(p.u8()?);
            }
        }
        // ACTI, TACT, TERM, FURN.
        12 | t::TACT | 14 | 25 => {
            if f & 0x4 != 0 {
                out.name = Some(p.wstr()?);
            }
            if cf.save_type == t::TACT && f & 0x80_0000 != 0 {
                out.other = Some(p.ref_id()?);
            }
        }
        // CONT, DOOR, INGR, STAT, ALCH, IDLM, NOTE, NAVM, RCPE, RCCT, CSNO,
        // LSCT, AMEF, CCRD, CDCK: form flags only.
        18 | 19 | 20 | 23 | 29 | 30 | t::NOTE | 36 | 45 | 46 | 48 | 49 | 51 | 52 | 54 => {}
        t::ECZN => {
            if f & 0x2 != 0 {
                p.u8()?;
            }
            if f & 0x8000_0000 != 0 {
                p.bytes(16)?;
            }
        }
        t::FLST => {
            if f & 0x8000_0000 != 0 {
                let n = p.u32()?;
                if n as usize > p.remaining() / 4 {
                    return Err(Error::new(p.position(), format!("{n} forms don't fit")));
                }
                out.added = Some((0..n).map(|_| p.ref_id()).collect::<Result<_>>()?);
            }
        }
        t::LVLC..=t::LVLI => {
            if f & 0x8000_0000 != 0 {
                out.added = Some(counted(&mut p, |p| {
                    let form = p.ref_id()?;
                    skip(p, &[2, 2, 4])?;
                    Ok(form)
                })?);
            }
        }
        t::WATR => {
            if f & 0x8000_0000 != 0 {
                out.other = Some(p.ref_id()?);
            }
        }
        _ => return Err(Error::new(cf.offset, "type not decoded")),
    }
    p.finish("base form")?;
    Ok(out)
}

/// A `PACK` change form (`TESPackage::SaveGame`, `006797c0`): nothing
/// for a package from a plugin; a package made in game isn't saved as
/// its own change form in any file seen, and is refused here.
pub fn package_form(cf: &ChangeForm<'_>) -> Result<()> {
    expect_type(cf, &[t::PACK])?;
    if cf.ref_id.is_created() {
        return Err(Error::new(cf.offset, "a package made in game"));
    }
    pipe(cf).finish("package")
}

/// How far this crate decodes a change form, checking the decodable ones
/// against their length.
pub fn coverage(cf: &ChangeForm<'_>) -> Coverage {
    coverage_of(cf, None)
}

/// [`coverage`], told the form id (the player's reference, 0x14, saves
/// more than other actors).
pub fn coverage_of(cf: &ChangeForm<'_>, form_id: Option<u32>) -> Coverage {
    let result = match cf.save_type {
        t::QUST => quest(cf).map(|_| ()),
        t::CELL => cell(cf, None).map(|_| ()),
        t::INFO => flags_only(cf).map(|_| ()),
        t::NPC_ | t::CREA => actor_base(cf).map(|_| ()),
        t::FACT => faction(cf).map(|_| ()),
        t::CLAS => class(cf).map(|_| ()),
        t::CHAL | t::REPU => pair(cf).map(|_| ()),
        t::REFR => reference(cf).map(|_| ()),
        t::ACHR | t::ACRE => actor_form(cf, form_id == Some(0x14)).map(|_| ()),
        3..=6 => projectile(cf).map(|_| ()),
        t::PACK => package_form(cf),
        // MSTT and PCBE: their classes' `SaveGame` wasn't found.
        24 | 44 => return Coverage::Skipped("type not decoded"),
        _ => base_form(cf).map(|_| ()),
    };
    match result {
        Ok(()) => Coverage::Exact,
        Err(e) => Coverage::Failed(e),
    }
}
