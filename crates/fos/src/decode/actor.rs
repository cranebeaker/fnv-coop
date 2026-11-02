//! References' own data (`TESObjectREFR::SaveGame` (Xbox PDB),
//! `00562230`), mobile objects (`MobileObject::SaveGame`, `00932880`),
//! actors (`Actor::SaveGame`, `008aaf40`; `Character::SaveGame`,
//! `008d33a0`), their AI processes and movers, the player
//! (`PlayerCharacter::SaveGame`, `009590f0`) and projectiles
//! (`Projectile::SaveGame`, `009c4ff0`). Names are the Xbox prototype's
//! (Xbox PDB); PC member offsets are the Xbox ones less 0x10 in these
//! classes.

use super::extra::{
    extra_list, inventory, inventory_with_capture, item_change, Context, Extra, ItemChange,
};
use super::package::{
    actor_package, detailed_path_handler, effects, pathing_location, pathing_request,
    pathing_solution,
};
use super::{counted, form_flags, reference_start, skip, InitialData};
use crate::inventory_source::InventoryCapture;
use crate::{save_type as t, ChangeForm, Error, Pipe, RefId, Result};

/// `REFR` change flags that bring the extra data list
/// (`00562230`; actors use [`ACTOR_EXTRA`]).
pub const REFR_EXTRA: u32 = 0xA402_1C40;
/// The same for actors (vtable +0x100, `IsActor`, holds).
pub const ACTOR_EXTRA: u32 = 0xA406_1840;

/// The part every reference writes (`00562230`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ReferenceData<'a> {
    pub form_flags: Option<u32>,
    /// `CHANGE_REFR_SCALE`.
    pub scale: Option<f32>,
    pub extra: Option<Vec<Extra>>,
    /// `CHANGE_REFR_INVENTORY` or `CHANGE_REFR_LEVELED_INVENTORY`.
    pub inventory: Option<Vec<ItemChange>>,
    /// `CHANGE_REFR_ANIMATION` (not actors): the animation's own data
    /// (`00563650`), not interpreted.
    pub animation: Option<&'a [u8]>,
}

fn reference_data<'a>(
    p: &mut Pipe<'a>,
    flags: u32,
    actor: bool,
    cx: Context,
    capture: Option<&mut InventoryCapture>,
) -> Result<ReferenceData<'a>> {
    let form_flags = form_flags(p, flags)?;
    let scale = if flags & 0x10 != 0 {
        Some(p.f32()?)
    } else {
        None
    };
    let mask = if actor { ACTOR_EXTRA } else { REFR_EXTRA };
    let extra = if flags & mask != 0 {
        Some(extra_list(p, cx)?)
    } else {
        None
    };
    let inventory = if flags & 0x0800_0020 != 0 {
        Some(match capture {
            Some(capture) => inventory_with_capture(p, cx, capture)?,
            None => inventory(p, cx)?,
        })
    } else {
        None
    };
    let animation = if flags & 0x1000_0000 != 0 && !actor {
        let n = p.vsval()? as usize;
        Some(p.raw(n)?)
    } else {
        None
    };
    Ok(ReferenceData {
        form_flags,
        scale,
        extra,
        inventory,
        animation,
    })
}

/// A `REFR` change form.
#[derive(Debug, Clone, PartialEq)]
pub struct Reference<'a> {
    pub initial: InitialData,
    pub havok: Option<&'a [u8]>,
    pub data: ReferenceData<'a>,
}

pub fn reference<'a>(cf: &ChangeForm<'a>) -> Result<Reference<'a>> {
    reference_impl(cf, None)
}

pub(crate) fn reference_captured<'a>(
    cf: &ChangeForm<'a>,
    capture: &mut InventoryCapture,
) -> Result<Reference<'a>> {
    reference_impl(cf, Some(capture))
}

fn reference_impl<'a>(
    cf: &ChangeForm<'a>,
    capture: Option<&mut InventoryCapture>,
) -> Result<Reference<'a>> {
    if cf.save_type != t::REFR {
        return Err(Error::new(cf.offset, "not a REFR change form"));
    }
    let start = reference_start(cf)?;
    let mut p = start.rest;
    let cx = Context {
        version: cf.version,
        npc: true,
    };
    let data = reference_data(&mut p, cf.flags, false, cx, capture)?;
    p.finish("reference")?;
    Ok(Reference {
        initial: start.initial,
        havok: start.havok,
        data,
    })
}

/// A modifier list (`ModifierList::SaveGame` (Xbox PDB), `00937b50`):
/// actor value and amount.
pub fn modifiers(p: &mut Pipe<'_>) -> Result<Vec<(u8, f32)>> {
    counted(p, |p| Ok((p.u8()?, p.f32()?)))
}

/// What the processes keep that a game continues from.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Process {
    /// `PROCESS_LEVEL`: 0 high, 1 middle high, 2 middle low, 3 low.
    pub level: u8,
    /// The package running (`ActorPackage`).
    pub package: Option<RefId>,
    /// `CHANGE_ACTOR_DAMAGE_MODIFIERS` (low process, `00910450`).
    pub damage_modifiers: Option<Vec<(u8, f32)>>,
    /// `CHANGE_ACTOR_TEMP_MODIFIERS` (middle low process, `0092ea30`).
    pub temp_modifiers: Option<Vec<(u8, f32)>>,
}

/// `BaseProcess::SaveGame` (`008d0f30`) and `LowProcess::SaveGame`
/// (`00910450`).
fn low_process(p: &mut Pipe<'_>, flags: u32, out: &mut Process, version: u8) -> Result<()> {
    skip(p, &[4, 4, 4])?;
    out.package = actor_package(p, version)?;
    skip(p, &[1, 4])?;
    p.ref_id()?;
    skip(p, &[4, 4, 4, 1, 2, 4, 4])?;
    for _ in 0..5 {
        p.ref_id()?;
    }
    counted(p, |p| p.ref_id().map(drop))?;
    if flags & 0x20_0000 != 0 {
        out.damage_modifiers = Some(modifiers(p)?);
    }
    Ok(())
}

/// `MiddleLowProcess::SaveGame` (`0092ea30`).
fn middle_low_process(p: &mut Pipe<'_>, flags: u32, out: &mut Process, v: u8) -> Result<()> {
    low_process(p, flags, out, v)?;
    p.u32()?;
    if flags & 0x10_0000 != 0 {
        out.temp_modifiers = Some(modifiers(p)?);
    }
    Ok(())
}

/// `MiddleHighProcess::SaveGame` (`00926a20`).
fn middle_high_process(p: &mut Pipe<'_>, flags: u32, out: &mut Process, v: u8) -> Result<()> {
    middle_low_process(p, flags, out, v)?;
    skip(
        p,
        &[
            1, 1, 1, 4, 4, 4, 1, 12, 4, 1, 1, 1, 2, 12, 1, 1, 1, 1, 4, 1, 4, 4, 1, 1, 1, 2, 4, 1,
            4, 4, 1, 1, 4, 4, 4, 4, 1,
        ],
    )?;
    for _ in 0..4 {
        p.ref_id()?;
    }
    p.u32()?;
    counted(p, |p| p.ref_id().map(drop))?;
    actor_package(p, v)?;
    // The animation (`0049ab40`) as a sized block, when the buffer's
    // owner says so (vtable +0x8) and `CHANGE_REFR_ANIMATION` is set.
    if flags & 0x1000_0000 != 0 {
        let n = p.vsval()? as usize;
        p.raw(n)?;
    }
    effects(p)?;
    for _ in 0..3 {
        p.ref_id()?;
    }
    counted(p, |p| {
        p.ref_id()?;
        skip(p, &[4, 4, 1, 1, 1, 1, 1, 1])
    })?;
    Ok(())
}

/// A queued item (`008d7070`).
fn queued_item(p: &mut Pipe<'_>) -> Result<()> {
    p.ref_id()?;
    skip(p, &[1, 4, 12, 4, 1, 1, 1, 4, 1])
}

/// `HighProcess::SaveGame` (`008fc4f0`).
fn high_process(p: &mut Pipe<'_>, flags: u32, out: &mut Process, v: u8) -> Result<()> {
    middle_high_process(p, flags, out, v)?;
    skip(
        p,
        &[
            1, 1, 1, 1, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 1, 12, 4, 4, 4, 4, 4, 4, 1, 4,
            1, 4, 1, 4, 4, 1, 4, 4, 4, 1, 1, 4, 4, 1, 1, 4, 4, 1, 4, 4, 4, 4, 4, 1, 1, 4, 1, 4, 1,
            1, 4, 1, 1, 1, 1,
        ],
    )?;
    for _ in 0..7 {
        p.ref_id()?;
    }
    for _ in 0..6 {
        p.ref_id()?;
        p.u8()?;
    }
    for _ in 0..3 {
        counted(p, |p| p.ref_id().map(drop))?;
    }
    if p.u8()? != 0 {
        super::package::dialogue(p)?;
    }
    counted(p, |p| {
        // A path avoid node (`006dc890`) and its actors (`008d73b0`).
        let kind = p.u8()?;
        skip(p, &[4, 4, 12])?;
        if kind == 1 {
            p.bytes(12)?;
        }
        skip(p, &[4, 4])?;
        p.ref_id()?;
        p.ref_id().map(drop)
    })?;
    counted(p, queued_item)?;
    counted(p, queued_item)?;
    if p.u8()? != 0 {
        // `008d7270`.
        skip(p, &[4, 12, 4, 4])?;
        p.ref_id()?;
    }
    // `00928880` as a sized block.
    let n = p.vsval()? as usize;
    p.raw(n)?;
    Ok(())
}

fn process(p: &mut Pipe<'_>, level: u8, flags: u32, version: u8) -> Result<Process> {
    let mut out = Process {
        level,
        ..Process::default()
    };
    match level {
        0 => high_process(p, flags, &mut out, version)?,
        1 => middle_high_process(p, flags, &mut out, version)?,
        2 => middle_low_process(p, flags, &mut out, version)?,
        3 => low_process(p, flags, &mut out, version)?,
        _ => return Err(Error::new(p.position(), format!("process level {level}"))),
    }
    Ok(out)
}

/// `MobileObject::SaveGame` (`00932880`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mobile<'a> {
    pub data: ReferenceData<'a>,
    /// None when the object has no process (level 0xFF).
    pub process: Option<Process>,
}

fn mobile<'a>(
    p: &mut Pipe<'a>,
    flags: u32,
    actor: bool,
    cx: Context,
    capture: Option<&mut InventoryCapture>,
) -> Result<Mobile<'a>> {
    let level = p.u8()?;
    let data = reference_data(p, flags, actor, cx, capture)?;
    skip(p, &[1, 1, 1, 1, 1, 1, 1, 1, 4, 4, 1, 1])?;
    p.ref_id()?;
    p.ref_id()?;
    let process = if level == 0xFF {
        None
    } else {
        Some(process(p, level, flags, cx.version)?)
    };
    Ok(Mobile { data, process })
}

/// `ActorMover::SaveGame` (`009df100`; `PlayerMover`, `009ea5a0`, adds
/// its own fields).
fn mover(p: &mut Pipe<'_>, player: bool) -> Result<()> {
    skip(
        p,
        &[2, 2, 1, 4, 1, 4, 1, 1, 12, 12, 4, 1, 1, 1, 1, 1, 4, 4, 4],
    )?;
    pathing_location(p)?;
    p.ref_id()?;
    let parts = p.u8()?;
    if parts & 1 != 0 {
        let kind = p.u8()?;
        pathing_request(p, kind)?;
    }
    if parts & 2 != 0 {
        pathing_solution(p)?;
    }
    if parts & 8 != 0 {
        detailed_path_handler(p)?;
    } else if parts & 4 != 0 {
        // VirtualActorPathHandler (`009eb390`).
        skip(p, &[4, 4, 4])?;
    }
    if player {
        skip(p, &[12, 4, 4, 4])?;
    }
    Ok(())
}

/// Some of the fixed fields `Actor::SaveGame` (`008aaf40`) writes, by
/// their Xbox names (PC offset + 0x10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ActorFields {
    /// `bProcessMe` (+0xbc): the AI runs (`SetActorsAI` clears it).
    pub process_me: bool,
    /// `bForceSneak` (+0x125).
    pub force_sneak: bool,
    /// `bDeadFlag` (+0x118).
    pub dead_flag: bool,
    /// `eCriticalStage` (+0x10c).
    pub critical_stage: u32,
    /// `bPlayerTeammate` (+0x18d).
    pub teammate: bool,
    /// `bIgnoreCrime` (+0x144).
    pub ignore_crime: bool,
    /// `iMinorCrimes`, `iMajorCrimes` (+0x13c, +0x140).
    pub minor_crimes: u32,
    pub major_crimes: u32,
}

/// The fixed run `Actor::SaveGame` writes after the mobile object, by
/// size; [`ActorFields`] picks from it by position.
const ACTOR_FIXED: [usize; 31] = [
    4, 1, 1, 1, 1, 4, 1, 4, 1, 1, 1, 1, 1, 1, 4, 4, 4, 1, 1, 1, 4, 4, 1, 1, 4, 1, 4, 1, 4, 4, 4,
];

/// Reads values of these sizes, each followed by `|`.
fn values<'a>(p: &mut Pipe<'a>, sizes: &[usize]) -> Result<Vec<&'a [u8]>> {
    sizes.iter().map(|&n| p.bytes(n)).collect()
}

fn word(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

/// What `Actor::SaveGame` (`008aaf40`) keeps that a game continues from.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Actor<'a> {
    pub mobile: Mobile<'a>,
    pub fields: ActorFields,
    /// `CHANGE_ACTOR_LIFESTATE`: `ACTOR_LIFE_STATE` (0 alive, 1 dying,
    /// 2 dead, 3 unconscious, ...).
    pub life_state: Option<u8>,
    /// `CHANGE_ACTOR_DISPOSITION_MODIFIERS`: toward whom, and the 4 bytes
    /// written with it.
    pub dispositions: Option<Vec<(RefId, u32)>>,
    /// `CHANGE_ACTOR_PERMANENT_MODIFIERS`.
    pub permanent_modifiers: Option<Vec<(u8, f32)>>,
    /// `CHANGE_ACTOR_OVERRIDE_MODIFIERS` (base value overrides).
    pub override_modifiers: Option<Vec<(u8, f32)>>,
}

fn actor<'a>(
    p: &mut Pipe<'a>,
    cf: &ChangeForm<'_>,
    player: bool,
    capture: Option<&mut InventoryCapture>,
) -> Result<Actor<'a>> {
    let flags = cf.flags;
    let cx = Context {
        version: cf.version,
        npc: cf.save_type == t::ACHR,
    };
    let mobile = mobile(p, flags, true, cx, capture)?;
    // In written order: the time since the last update (+0x114), +0x124
    // `bForceRun`, +0x125, +0xbc, +0xc4, +0xc8, +0x7d, +0x110, +0x118,
    // +0x126, +0x145, +0x146, +0x14c, +0x14d, +0x150, +0x154, +0x158,
    // +0x174, +0x175, +0x18d, +0x1a4, +0x1a8, +0xf0, +0xf1, +0x10c,
    // +0x134, +0x138, +0x144, +0x13c, +0x140, +0x120.
    let v = values(p, &ACTOR_FIXED)?;
    let fields = ActorFields {
        process_me: v[3][0] != 0,
        force_sneak: v[2][0] != 0,
        dead_flag: v[8][0] != 0,
        critical_stage: word(v[24]),
        teammate: v[19][0] != 0,
        ignore_crime: v[27][0] != 0,
        minor_crimes: word(v[28]),
        major_crimes: word(v[29]),
    };
    for _ in 0..3 {
        p.ref_id()?;
    }
    let life_state = if flags & 0x400 != 0 {
        Some(p.u8()?)
    } else {
        None
    };
    let dispositions = if flags & 0x8_0000 != 0 {
        Some(counted(p, |p| Ok((p.ref_id()?, p.u32()?)))?)
    } else {
        None
    };
    let permanent_modifiers = if flags & 0x80_0000 != 0 {
        Some(modifiers(p)?)
    } else {
        None
    };
    let override_modifiers = if flags & 0x40_0000 != 0 {
        Some(modifiers(p)?)
    } else {
        None
    };
    mover(p, player)?;
    if cf.save_type == t::ACHR {
        // `Character::SaveGame` (`008d33a0`).
        skip(p, &[1, 1])?;
    }
    Ok(Actor {
        mobile,
        fields,
        life_state,
        dispositions,
        permanent_modifiers,
        override_modifiers,
    })
}

/// The player's own data (`PlayerCharacter::SaveGame`, `009590f0`), as
/// far as nv-rs uses it; the rest is read past.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Player {
    /// `TemporaryActorValueModifiers`, `ScriptActorValueModifiers`,
    /// `DamageActorValueModifiers`: 77 actor values each.
    pub temporary_values: Vec<f32>,
    pub script_values: Vec<f32>,
    pub damage_values: Vec<f32>,
    pub health_modifier: f32,
    /// `pActiveQuest`.
    pub active_quest: RefId,
    /// `listNotes`.
    pub notes: Vec<RefId>,
    /// `listTopics` (the player's known topics).
    pub topics: Vec<RefId>,
    /// `Perks`: perk and rank.
    pub perks: Vec<(RefId, u8)>,
    /// `CompanionPerks`.
    pub companion_perks: Vec<(RefId, u8)>,
    /// `pCrimeCounts`: five counts.
    pub crime_counts: [u32; 5],
    /// The hot keys' items as plain form ids (0 none).
    pub hotkeys: [u32; 8],
    /// `bChargen` (+0x75c).
    pub chargen: bool,
    /// `bCanFastTravel` (+0x66d: bit 0 allowed, bit 1 kept as it is when
    /// the player is moved) and `bCanWait` (+0x66e).
    pub fast_travel: u8,
    pub can_wait: bool,
    /// `iNumberofStealWarnings`, `iNumberofPickpocketWarnings` (+0x228,
    /// +0x230).
    pub steal_warnings: u32,
    pub pickpocket_warnings: u32,
    /// `iTotalPlayingTime` (+0x790).
    pub playing_time: u32,
}

/// The fixed run after `Character::SaveGame` in the player's data, by
/// size, in written order: +0x64a, +0x64d, +0x651, +0x652, +0x654, +0x660,
/// +0x664, +0x668, +0x66c, +0x6cc, +0x6d0, +0x6d4, +0x6d8, +0x6dc, +0x6e8,
/// +0x681, +0x7c5, +0x7c6, +0x6e4, the map marker's position (12 bytes),
/// +0x698, +0x67c, +0x738, +0x658, +0x65c, +0x674, +0x670, +0x75c,
/// +0x730, +0x790, +0x680, +0x7c4, +0x63c, +0x640, +0x644, +0x200,
/// +0x240, +0x64e, +0x66d, +0x794, `011e0b5c`, +0xd6c, +0xd70, +0x228,
/// +0x22c, +0x230, +0x234, +0x608, +0xdf2, +0x64f, +0x650, +0x7c7,
/// +0x5f8, +0x1fc, +0x684.
const PLAYER_FIXED: [usize; 55] = [
    1, 1, 1, 1, 4, 4, 4, 4, 1, 1, 4, 4, 1, 4, 1, 1, 1, 1, 4, 12, 4, 4, 4, 1, 4, 4, 4, 1, 4, 4, 1,
    1, 4, 4, 4, 4, 1, 1, 1, 4, 4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 1, 1, 1, 4, 4,
];

/// Reads the player: [`Player`] around the [`Actor`] data.
fn player<'a>(
    p: &mut Pipe<'a>,
    cf: &ChangeForm<'_>,
    capture: Option<&mut InventoryCapture>,
) -> Result<(Actor<'a>, Player)> {
    let mut out = Player::default();
    let av_list = |p: &mut Pipe<'_>| -> Result<Vec<f32>> { (0..77).map(|_| p.f32()).collect() };
    out.temporary_values = av_list(p)?;
    out.script_values = av_list(p)?;
    out.damage_values = av_list(p)?;
    out.health_modifier = p.f32()?;
    let actor = actor(p, cf, true, capture)?;
    let cx = Context {
        version: cf.version,
        npc: true,
    };
    if cf.flags & 0x1000_0000 != 0 {
        let n = p.vsval()? as usize;
        p.raw(n)?;
    }
    let v = values(p, &PLAYER_FIXED)?;
    out.chargen = v[27][0] != 0;
    out.playing_time = word(v[29]);
    out.fast_travel = v[38][0];
    out.steal_warnings = word(v[43]);
    out.pickpocket_warnings = word(v[45]);
    for c in &mut out.crime_counts {
        *c = p.u32()?;
    }
    // `008d56c0`.
    skip(p, &[1, 4])?;
    // pActiveQuest, pDefaultClass, the map marker's, pOccupiedRegion,
    // its own, pClosestConversation, pAIConversationRunning,
    // pGrabbedObject, pLastExtDoorActivated, pAutoAimActor,
    // pPlayersTargetActor.
    out.active_quest = p.ref_id()?;
    for _ in 0..10 {
        p.ref_id()?;
    }
    out.topics = counted(p, |p| p.ref_id())?;
    out.notes = counted(p, |p| p.ref_id())?;
    // RockItLauncherAmmoList.
    counted(p, |p| item_change(p, cx).map(drop))?;
    // pListofPercievedActors.
    counted(p, |p| {
        p.ref_id()?;
        skip(p, &[1, 1])
    })?;
    out.perks = counted(p, |p| Ok((p.ref_id()?, p.u8()?)))?;
    // pListofActions, pListofCasinoData, the two caravan card lists and
    // the caravan totals.
    counted(p, |p| {
        skip(p, &[4, 4])?;
        p.ref_id().map(drop)
    })?;
    counted(p, |p| skip(p, &[4, 4, 2]))?;
    counted(p, |p| p.ref_id().map(drop))?;
    counted(p, |p| p.ref_id().map(drop))?;
    skip(p, &[4, 4, 4, 4, 4])?;
    // listQuestLog, listObjectives.
    counted(p, |p| {
        p.ref_id()?;
        skip(p, &[1, 1])
    })?;
    counted(p, |p| {
        p.ref_id()?;
        p.u32().map(drop)
    })?;
    effects(p)?;
    skip(p, &[4, 4, 4])?;
    out.companion_perks = counted(p, |p| Ok((p.ref_id()?, p.u8()?)))?;
    // `004d1360`, `005a6050`, then `bCanWait`.
    skip(p, &[1, 1])?;
    out.can_wait = p.u8()? != 0;
    for h in &mut out.hotkeys {
        *h = p.u32()?;
    }
    // `005aa930`: four lists kept outside the player.
    for _ in 0..4 {
        counted(p, |p| p.ref_id().map(drop))?;
    }
    Ok((actor, out))
}

/// An `ACHR` or `ACRE` change form.
#[derive(Debug, Clone, PartialEq)]
pub struct ActorForm<'a> {
    pub initial: InitialData,
    pub havok: Option<&'a [u8]>,
    pub actor: Actor<'a>,
    /// For the player's reference (form id 0x14).
    pub player: Option<Player>,
}

/// Reads an `ACHR` or `ACRE`; `player` says whether it is the player's
/// reference (form id 0x14, whose class saves more).
pub fn actor_form<'a>(cf: &ChangeForm<'a>, player_ref: bool) -> Result<ActorForm<'a>> {
    actor_form_impl(cf, player_ref, None)
}

pub(crate) fn actor_form_captured<'a>(
    cf: &ChangeForm<'a>,
    player_ref: bool,
    capture: &mut InventoryCapture,
) -> Result<ActorForm<'a>> {
    actor_form_impl(cf, player_ref, Some(capture))
}

fn actor_form_impl<'a>(
    cf: &ChangeForm<'a>,
    player_ref: bool,
    capture: Option<&mut InventoryCapture>,
) -> Result<ActorForm<'a>> {
    if cf.save_type != t::ACHR && cf.save_type != t::ACRE {
        return Err(Error::new(cf.offset, "not an ACHR or ACRE change form"));
    }
    let start = reference_start(cf)?;
    let mut p = start.rest;
    let (actor, player) = if player_ref {
        let (a, pl) = player(&mut p, cf, capture)?;
        (a, Some(pl))
    } else {
        (actor(&mut p, cf, false, capture)?, None)
    };
    p.finish("actor")?;
    Ok(ActorForm {
        initial: start.initial,
        havok: start.havok,
        actor,
        player,
    })
}

/// A projectile in flight (`PMIS`, `PGRE`, `PBEA`, `PFLA`).
pub fn projectile<'a>(cf: &ChangeForm<'a>) -> Result<Mobile<'a>> {
    if !(3..=6).contains(&cf.save_type) {
        return Err(Error::new(cf.offset, "not a projectile change form"));
    }
    let start = reference_start(cf)?;
    let mut p = start.rest;
    let cx = Context {
        version: cf.version,
        npc: true,
    };
    let mobile = mobile(&mut p, cf.flags, false, cx, None)?;
    skip(&mut p, &[4, 4, 4, 4, 4, 4, 4, 4, 4])?;
    for _ in 0..3 {
        p.ref_id()?;
    }
    skip(&mut p, &[12, 4, 1, 4, 16, 12, 4, 4, 4, 4, 4])?;
    if p.u8()? != 0 {
        item_change(&mut p, cx)?;
    }
    counted(&mut p, |p| {
        skip(p, &[12, 12, 4, 4, 1, 2, 2])?;
        p.ref_id().map(drop)
    })?;
    p.u8()?;
    match cf.save_type {
        // MissileProjectile (`009baaa0`).
        3 => skip(&mut p, &[4])?,
        // FlameProjectile (`009b3eb0`).
        6 => skip(&mut p, &[4, 4])?,
        // GrenadeProjectile, BeamProjectile (`00979bb0`).
        _ => {}
    }
    p.finish("projectile")?;
    Ok(mobile)
}
