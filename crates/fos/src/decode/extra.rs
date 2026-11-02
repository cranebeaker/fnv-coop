//! A reference's extra data as the game saves it
//! (`ExtraDataList::SaveGame` (Xbox PDB), `00426a30`), and inventories
//! (`InventoryChanges::SaveGame` (Xbox PDB), `004d4090`).
//!
//! Which extra data a reference writes depends on its change flags: the
//! table at `01183d30` gives each extra type the change flags it is saved
//! under (see [`saved_under`]). The list is a vsval count, then per entry
//! the extra type (`u8|`) and its data; the reader dispatches on the type.

use super::package::{actor_package_data, package_body};
use super::{counted, script_locals, skip, ScriptLocals};
use crate::inventory_source::{InventoryCapture, ItemSpans, StackSpans};
use crate::{Error, Pipe, RefId, Result};
use std::ops::Range;

/// `EXTRA_DATA_TYPE` (Xbox PDB) names of the types a save can hold.
pub mod kind {
    pub const SCRIPT: u8 = 0x0D;
    pub const WORN: u8 = 0x16;
    pub const PACKAGE_START_LOCATION: u8 = 0x18;
    pub const PACKAGE: u8 = 0x19;
    pub const TRESPASS_PACKAGE: u8 = 0x1A;
    pub const RUN_ONCE_PACKAGES: u8 = 0x1B;
    pub const REFERENCE_POINTER: u8 = 0x1C;
    pub const FOLLOWER: u8 = 0x1D;
    pub const LEVCREA_MOD: u8 = 0x1E;
    pub const GHOST: u8 = 0x1F;
    pub const OWNERSHIP: u8 = 0x21;
    pub const GLOBAL: u8 = 0x22;
    pub const RANK: u8 = 0x23;
    pub const COUNT: u8 = 0x24;
    pub const HEALTH: u8 = 0x25;
    pub const USES: u8 = 0x26;
    pub const TIME_LEFT: u8 = 0x27;
    pub const CHARGE: u8 = 0x28;
    pub const LIGHT: u8 = 0x29;
    pub const LOCK: u8 = 0x2A;
    pub const TELEPORT: u8 = 0x2B;
    pub const MAP_MARKER: u8 = 0x2C;
    pub const LEVELED_CREATURE: u8 = 0x2E;
    pub const LEVELED_ITEM: u8 = 0x2F;
    pub const SCALE: u8 = 0x30;
    pub const MAGIC_CASTER: u8 = 0x32;
    pub const MAGIC_TARGET: u8 = 0x33;
    pub const PLAYER_CRIME_LIST: u8 = 0x35;
    pub const ITEM_DROPPER: u8 = 0x39;
    pub const MERCHANT_CONTAINER: u8 = 0x3C;
    pub const CANNOT_WEAR: u8 = 0x3E;
    pub const POISON: u8 = 0x3F;
    pub const FRIEND_HITS: u8 = 0x45;
    pub const HEAD_TRACK_TARGET: u8 = 0x46;
    pub const STARTING_WORLD_OR_CELL: u8 = 0x49;
    pub const HOT_KEY: u8 = 0x4A;
    pub const INFO_GENERAL_TOPIC: u8 = 0x4D;
    pub const NO_RUMORS: u8 = 0x4E;
    pub const TERMINAL_STATE: u8 = 0x50;
    pub const ACTIVATE_REF_CHILDREN: u8 = 0x54;
    pub const TALKING_ACTOR: u8 = 0x55;
    pub const OBJECT_HEALTH: u8 = 0x56;
    pub const MODEL_SWAP: u8 = 0x5B;
    pub const RADIUS: u8 = 0x5C;
    pub const RADIATION: u8 = 0x5D;
    pub const FACTION_CHANGES: u8 = 0x5E;
    pub const DISMEMBERED_LIMBS: u8 = 0x5F;
    pub const ACTOR_CAUSE: u8 = 0x60;
    pub const OPEN_CLOSE_ACTIVATE_REF: u8 = 0x6C;
    pub const AMMO: u8 = 0x6E;
    pub const PACKAGE_DATA: u8 = 0x70;
    pub const SAY_ONCE_A_DAY_TOPIC_INFO: u8 = 0x73;
    pub const ENCOUNTER_ZONE: u8 = 0x74;
    pub const SAY_TO_TOPIC_INFO: u8 = 0x75;
    pub const GUARDED_REF_DATA: u8 = 0x7C;
    pub const ASH_PILE_REF: u8 = 0x89;
    pub const FOLLOWER_SWIM_BREADCRUMBS: u8 = 0x8B;
    pub const WEAPON_MOD_SLOTS: u8 = 0x8D;
    pub const SECURITRON_FACE: u8 = 0x8F;
    pub const AUDIO_MARKER: u8 = 0x90;
    pub const AUDIO_BUOY_MARKER: u8 = 0x91;
    pub const SPECIAL_RENDER_FLAGS: u8 = 0x92;
}

/// The change flags an extra type is saved under (the table at
/// `01183d30`, one `u32` per type); 0 for types never saved. A type with
/// 0x40000000 in its mask is saved for created references only.
pub fn saved_under(kind: u8) -> u32 {
    use kind::*;
    match kind {
        SCRIPT => 0x8000_0400,
        WORN
        | REFERENCE_POINTER
        | COUNT
        | HEALTH
        | USES
        | TIME_LEFT
        | CHARGE
        | LIGHT
        | LEVELED_ITEM
        | SCALE
        | CANNOT_WEAR
        | POISON
        | STARTING_WORLD_OR_CELL
        | HOT_KEY
        | WEAPON_MOD_SLOTS => 0x400,
        PACKAGE_START_LOCATION | FOLLOWER | AMMO => 0x800,
        OWNERSHIP | GLOBAL | RANK => 0x440,
        LOCK | MERCHANT_CONTAINER => 0x1000,
        TELEPORT | DISMEMBERED_LIMBS => 0x2_0000,
        LEVELED_CREATURE => 0x4_0000,
        ACTIVATE_REF_CHILDREN => 0x400_0000,
        ENCOUNTER_ZONE => 0x2000_0000,
        LEVCREA_MOD | RADIUS | RADIATION => 0x4000_0000,
        PACKAGE
        | TRESPASS_PACKAGE
        | RUN_ONCE_PACKAGES
        | GHOST
        | MAP_MARKER
        | MAGIC_CASTER
        | MAGIC_TARGET
        | PLAYER_CRIME_LIST
        | ITEM_DROPPER
        | FRIEND_HITS
        | HEAD_TRACK_TARGET
        | INFO_GENERAL_TOPIC
        | NO_RUMORS
        | TERMINAL_STATE
        | TALKING_ACTOR
        | OBJECT_HEALTH
        | MODEL_SWAP
        | FACTION_CHANGES
        | ACTOR_CAUSE
        | OPEN_CLOSE_ACTIVATE_REF
        | PACKAGE_DATA
        | SAY_ONCE_A_DAY_TOPIC_INFO
        | SAY_TO_TOPIC_INFO
        | GUARDED_REF_DATA
        | ASH_PILE_REF
        | FOLLOWER_SWIM_BREADCRUMBS
        | SECURITRON_FACE
        | SPECIAL_RENDER_FLAGS => 0x8000_0000,
        _ => 0,
    }
}

/// `REFR_LOCK` (Xbox PDB) as `ExtraLock` saves it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lock {
    /// `cBaseLevel`: 0 to 100, 255 needs a key.
    pub level: u8,
    /// `cFlags`: bit 0x1 locked.
    pub flags: u8,
    pub key: RefId,
    pub tries: u32,
    pub times_unlocked: u32,
}

/// `DoorTeleportData` (Xbox PDB): where a load door leads (`0043aa40`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Teleport {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub flags: u8,
    pub linked_door: RefId,
}

/// One extra data entry. Types whose data matters for continuing a game
/// keep it; the rest are read past and keep only their type.
#[derive(Debug, Clone, PartialEq)]
pub enum Extra {
    Script {
        script: RefId,
        locals: ScriptLocals,
    },
    Worn,
    Ghost,
    CannotWear,
    Ownership(RefId),
    Global(RefId),
    Rank(i32),
    Count(i16),
    Health(f32),
    Uses(u8),
    TimeLeft(f32),
    Charge(f32),
    Lock(Lock),
    Teleport(Teleport),
    /// `MapMarkerData::cFlags` (Xbox PDB): 0x1 visible, 0x2 can travel.
    MapMarker(u8),
    /// `ExtraLeveledCreature` (Xbox PDB): `pOriginalBase` (the leveled
    /// list), `pTemplate` (what it picked), and the change flags of the
    /// base the reference was given (its data is read past).
    LeveledCreature {
        original_base: RefId,
        template: RefId,
        flags: u32,
    },
    Scale(f32),
    MerchantContainer(RefId),
    Poison(RefId),
    HotKey(u8),
    /// `bFlags` and the lock level of a terminal.
    TerminalState {
        flags: u8,
        level: u8,
    },
    ObjectHealth(f32),
    Radiation(f32),
    /// Factions and ranks (`ExtraFactionChanges`).
    FactionChanges(Vec<(RefId, i8)>),
    Ammo {
        ammo: RefId,
        count: i32,
    },
    EncounterZone(RefId),
    WeaponModSlots(u8),
    /// Any other type, read past.
    Other(u8),
}

impl Extra {
    pub fn kind(&self) -> u8 {
        use kind::*;
        match self {
            Extra::Script { .. } => SCRIPT,
            Extra::Worn => WORN,
            Extra::Ghost => GHOST,
            Extra::CannotWear => CANNOT_WEAR,
            Extra::Ownership(_) => OWNERSHIP,
            Extra::Global(_) => GLOBAL,
            Extra::Rank(_) => RANK,
            Extra::Count(_) => COUNT,
            Extra::Health(_) => HEALTH,
            Extra::Uses(_) => USES,
            Extra::TimeLeft(_) => TIME_LEFT,
            Extra::Charge(_) => CHARGE,
            Extra::Lock(_) => LOCK,
            Extra::Teleport(_) => TELEPORT,
            Extra::MapMarker(_) => MAP_MARKER,
            Extra::LeveledCreature { .. } => LEVELED_CREATURE,
            Extra::Scale(_) => SCALE,
            Extra::MerchantContainer(_) => MERCHANT_CONTAINER,
            Extra::Poison(_) => POISON,
            Extra::HotKey(_) => HOT_KEY,
            Extra::TerminalState { .. } => TERMINAL_STATE,
            Extra::ObjectHealth(_) => OBJECT_HEALTH,
            Extra::Radiation(_) => RADIATION,
            Extra::FactionChanges(_) => FACTION_CHANGES,
            Extra::Ammo { .. } => AMMO,
            Extra::EncounterZone(_) => ENCOUNTER_ZONE,
            Extra::WeaponModSlots(_) => WEAPON_MOD_SLOTS,
            Extra::Other(k) => *k,
        }
    }
}

/// What the extra data reader needs to know about the reference.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// The change form's version (script locals depend on it).
    pub version: u8,
    /// Whether a leveled actor's base is an `NPC_` (else a `CREA`): its
    /// data is that base type's change data (`0x2E`).
    pub npc: bool,
}

fn f32_of(b: &[u8]) -> f32 {
    f32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

fn i32_of(b: &[u8]) -> i32 {
    i32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

/// The active effects of a magic target (`00806a10`): per effect the
/// magic item, the effect's index in it, its type, and its own data as a
/// vsval-sized block (`00806840`). Returned as (item, index, type).
pub fn active_effects(p: &mut Pipe<'_>) -> Result<Vec<(RefId, u8, u32)>> {
    counted(p, |p| {
        let item = p.ref_id()?;
        let index = p.u8()?;
        let kind = p.vsval()?;
        let n = p.vsval()? as usize;
        p.raw(n)?;
        Ok((item, index, kind))
    })
}

/// One extra data list (`00426a30`).
pub fn extra_list(p: &mut Pipe<'_>, cx: Context) -> Result<Vec<Extra>> {
    extra_list_with_spans(p, cx, None)
}

fn extra_list_with_spans(
    p: &mut Pipe<'_>,
    cx: Context,
    mut spans: Option<&mut Vec<Range<usize>>>,
) -> Result<Vec<Extra>> {
    counted(p, |p| {
        let start = spans.as_ref().map(|_| p.source_position());
        let value = extra(p, cx)?;
        if let (Some(spans), Some(start)) = (spans.as_mut(), start) {
            spans.push(start..p.source_position());
        }
        Ok(value)
    })
}

fn extra(p: &mut Pipe<'_>, cx: Context) -> Result<Extra> {
    use kind::*;
    let at = p.position();
    let k = p.u8()?;
    Ok(match k {
        SCRIPT => {
            let script = p.ref_id()?;
            let locals = script_locals(p, cx.version)?;
            Extra::Script { script, locals }
        }
        WORN => Extra::Worn,
        GHOST => Extra::Ghost,
        CANNOT_WEAR => Extra::CannotWear,
        // `EXTRA_LIGHT` is saved under 0x400 but has no case: the game
        // logs it and writes nothing after the type.
        LIGHT | AUDIO_MARKER | AUDIO_BUOY_MARKER => Extra::Other(k),
        PACKAGE_START_LOCATION => {
            p.ref_id()?;
            skip(p, &[12, 4])?;
            Extra::Other(k)
        }
        PACKAGE => {
            p.ref_id()?;
            p.ref_id()?;
            skip(p, &[4, 1, 1, 1])?;
            Extra::Other(k)
        }
        TRESPASS_PACKAGE => {
            // `TrespassPackage` (`009f9790`) when there is one.
            if p.ref_id()? != RefId(0) {
                package_body(p, 23, cx.version)?;
            }
            Extra::Other(k)
        }
        RUN_ONCE_PACKAGES | SAY_ONCE_A_DAY_TOPIC_INFO => {
            counted(p, |p| {
                p.ref_id()?;
                if k == RUN_ONCE_PACKAGES {
                    p.u8().map(drop)
                } else {
                    skip(p, &[4, 4])
                }
            })?;
            Extra::Other(k)
        }
        FOLLOWER | GUARDED_REF_DATA => {
            counted(p, |p| p.ref_id())?;
            Extra::Other(k)
        }
        REFERENCE_POINTER
        | ITEM_DROPPER
        | HEAD_TRACK_TARGET
        | STARTING_WORLD_OR_CELL
        | TALKING_ACTOR
        | OPEN_CLOSE_ACTIVATE_REF
        | ASH_PILE_REF => {
            p.ref_id()?;
            Extra::Other(k)
        }
        OWNERSHIP => Extra::Ownership(p.ref_id()?),
        GLOBAL => Extra::Global(p.ref_id()?),
        MERCHANT_CONTAINER => Extra::MerchantContainer(p.ref_id()?),
        POISON => Extra::Poison(p.ref_id()?),
        ENCOUNTER_ZONE => Extra::EncounterZone(p.ref_id()?),
        RANK => Extra::Rank(p.i32()?),
        COUNT => Extra::Count(p.u16()? as i16),
        HEALTH => Extra::Health(p.f32()?),
        USES => Extra::Uses(p.u8()?),
        TIME_LEFT => Extra::TimeLeft(p.f32()?),
        CHARGE => Extra::Charge(p.f32()?),
        SCALE => Extra::Scale(p.f32()?),
        OBJECT_HEALTH => Extra::ObjectHealth(p.f32()?),
        RADIATION => Extra::Radiation(p.f32()?),
        LEVCREA_MOD | ACTIVATE_REF_CHILDREN | RADIUS | ACTOR_CAUSE => {
            p.u32()?;
            Extra::Other(k)
        }
        LOCK => Extra::Lock(Lock {
            level: p.u8()?,
            flags: p.u8()?,
            key: p.ref_id()?,
            tries: p.u32()?,
            times_unlocked: p.u32()?,
        }),
        TELEPORT => {
            let a = p.bytes(12)?;
            let b = p.bytes(12)?;
            let at = |s: &[u8]| [f32_of(s), f32_of(&s[4..]), f32_of(&s[8..])];
            Extra::Teleport(Teleport {
                position: at(a),
                rotation: at(b),
                flags: p.u8()?,
                linked_door: p.ref_id()?,
            })
        }
        MAP_MARKER => Extra::MapMarker(p.u8()?),
        HOT_KEY => Extra::HotKey(p.u8()?),
        WEAPON_MOD_SLOTS => Extra::WeaponModSlots(p.u8()?),
        NO_RUMORS => {
            p.u8()?;
            Extra::Other(k)
        }
        LEVELED_CREATURE => {
            let original_base = p.ref_id()?;
            let template = p.ref_id()?;
            let flags = p.u32()?;
            super::actor_base_data(p, flags, cx.npc)?;
            Extra::LeveledCreature {
                original_base,
                template,
                flags,
            }
        }
        LEVELED_ITEM => {
            skip(p, &[4, 1])?;
            Extra::Other(k)
        }
        MAGIC_CASTER => {
            p.ref_id()?;
            p.ref_id()?;
            p.ref_id()?;
            Extra::Other(k)
        }
        MAGIC_TARGET => {
            p.ref_id()?;
            active_effects(p)?;
            Extra::Other(k)
        }
        PLAYER_CRIME_LIST => {
            counted(p, |p| skip(p, &[4, 4]))?;
            Extra::Other(k)
        }
        FRIEND_HITS => {
            counted(p, |p| p.u32())?;
            Extra::Other(k)
        }
        INFO_GENERAL_TOPIC => {
            p.wstr()?;
            skip(p, &[1, 1, 1, 1, 1])?;
            for _ in 0..4 {
                p.ref_id()?;
            }
            Extra::Other(k)
        }
        TERMINAL_STATE => Extra::TerminalState {
            flags: p.u8()?,
            level: p.u8()?,
        },
        MODEL_SWAP => {
            p.ref_id()?;
            p.u32()?;
            Extra::Other(k)
        }
        FACTION_CHANGES => Extra::FactionChanges(counted(p, |p| Ok((p.ref_id()?, p.u8()? as i8)))?),
        DISMEMBERED_LIMBS => {
            skip(p, &[2, 4, 4, 1])?;
            p.ref_id()?;
            counted(p, |p| {
                skip(p, &[1, 1, 1, 1])?;
                counted(p, |p| p.ref_id()).map(drop)
            })?;
            Extra::Other(k)
        }
        AMMO => {
            let ammo = p.ref_id()?;
            let count = i32_of(p.bytes(4)?);
            Extra::Ammo { ammo, count }
        }
        PACKAGE_DATA => {
            let t = p.u8()?;
            if t != 0xFF {
                actor_package_data(p, t)?;
            }
            Extra::Other(k)
        }
        SAY_TO_TOPIC_INFO => {
            p.ref_id()?;
            p.ref_id()?;
            p.u8()?;
            Extra::Other(k)
        }
        FOLLOWER_SWIM_BREADCRUMBS => {
            p.bytes(12)?;
            p.ref_id()?;
            p.u32()?;
            counted(p, |p| {
                p.bytes(12)?;
                p.ref_id()?;
                p.bytes(12)?;
                p.ref_id()?;
                p.u8().map(drop)
            })?;
            Extra::Other(k)
        }
        SECURITRON_FACE => {
            p.wstr()?;
            p.wstr()?;
            Extra::Other(k)
        }
        SPECIAL_RENDER_FLAGS => {
            skip(p, &[4, 4])?;
            Extra::Other(k)
        }
        _ => {
            return Err(Error::new(
                at,
                format!("extra data type {k:#04x} isn't one the game saves"),
            ))
        }
    })
}

/// One inventory entry (`ItemChange::SaveGame` (Xbox PDB), `004bed60`):
/// the item, how many more or fewer than the base record holds, and the
/// extra data of each stack (saved with the change flags set to
/// `0x400`).
#[derive(Debug, Clone, PartialEq)]
pub struct ItemChange {
    pub item: RefId,
    pub count: i32,
    pub stacks: Vec<Vec<Extra>>,
}

pub fn item_change(p: &mut Pipe<'_>, cx: Context) -> Result<ItemChange> {
    item_change_with_capture(p, cx, None)
}

fn item_change_with_capture(
    p: &mut Pipe<'_>,
    cx: Context,
    capture: Option<&mut InventoryCapture>,
) -> Result<ItemChange> {
    let start = capture.as_ref().map(|_| p.source_position());
    let item = p.ref_id()?;
    let count = p.i32()?;
    let mut stack_spans = Vec::new();
    let stacks = counted(p, |p| {
        if capture.is_none() {
            return extra_list(p, cx);
        }
        let start = p.source_position();
        let mut extras = Vec::new();
        let values = extra_list_with_spans(p, cx, Some(&mut extras))?;
        stack_spans.push(StackSpans {
            range: start..p.source_position(),
            extras,
        });
        Ok(values)
    })?;
    if let (Some(capture), Some(start)) = (capture, start) {
        capture.items.push(ItemSpans {
            range: start..p.source_position(),
            stacks: stack_spans,
        });
    }
    Ok(ItemChange {
        item,
        count,
        stacks,
    })
}

/// An inventory (`004d4090`): its item changes.
pub fn inventory(p: &mut Pipe<'_>, cx: Context) -> Result<Vec<ItemChange>> {
    counted(p, |p| item_change(p, cx))
}

pub(crate) fn inventory_with_capture(
    p: &mut Pipe<'_>,
    cx: Context,
    capture: &mut InventoryCapture,
) -> Result<Vec<ItemChange>> {
    counted(p, |p| item_change_with_capture(p, cx, Some(&mut *capture)))
}
