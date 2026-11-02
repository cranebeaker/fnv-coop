//! AI state the game saves with actors: the running package
//! (`ActorPackage`, `008c65d0`), packages made in game (`TESPackage` and
//! its subclasses, the combat controller), their per-actor data, and the
//! pathing the movers keep. nv-rs re-evaluates AI on loading, so this is
//! read past, not kept; it is decoded so every byte is accounted for.

use super::extra::active_effects;
use super::{counted, skip};
use crate::{Error, Pipe, RefId, Result};

/// Reads a run of fixed-size values, each followed by `|`.
fn run(p: &mut Pipe<'_>, sizes: &[usize]) -> Result<()> {
    skip(p, sizes)
}

/// `009a5eb0`: a time stamp and a value (two 4-byte values).
fn timer(p: &mut Pipe<'_>) -> Result<()> {
    run(p, &[4, 4])
}

/// `PathingLocation::SaveGame` (Xbox PDB), `006def40`.
pub(crate) fn pathing_location(p: &mut Pipe<'_>) -> Result<()> {
    p.bytes(12)?;
    p.ref_id()?;
    p.ref_id()?;
    p.ref_id()?;
    run(p, &[4, 2, 1, 1])
}

/// `PathingCoverLocation::SaveGame` (Xbox PDB), `006e48a0`.
fn cover_location(p: &mut Pipe<'_>) -> Result<()> {
    pathing_location(p)?;
    run(p, &[12, 12, 12, 1, 1, 1, 1, 1, 1, 1])
}

/// `PathingAvoidNode::SaveGame` (Xbox PDB), `006dc890`.
fn avoid_node(p: &mut Pipe<'_>) -> Result<()> {
    let kind = p.u8()?;
    run(p, &[4, 4, 12])?;
    if kind == 1 {
        p.bytes(12)?;
    }
    Ok(())
}

/// A pathing request (`PathingRequest::SaveGame` (Xbox PDB), `006e3230`,
/// and its subclasses by `GetType`, vtable +0x10).
pub(crate) fn pathing_request(p: &mut Pipe<'_>, kind: u8) -> Result<()> {
    let at = p.position();
    pathing_location(p)?;
    pathing_location(p)?;
    run(
        p,
        &[
            4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 1, 1, 1, 1, 4, 1, 1, 12, 1, 1, 1, 4, 1, 1,
        ],
    )?;
    counted(p, avoid_node)?;
    match kind {
        // PathingRequest, PathingRequestCover.
        0 | 1 => {}
        // ClosePoint, OptimalLocation (`006e5f00`).
        2 | 7 => run(p, &[4, 4])?,
        // Flee (`006e54b0`).
        3 => run(p, &[4])?,
        // Hide (`006e5740`).
        4 => {
            pathing_location(p)?;
            run(p, &[4])?;
        }
        // LOS (`006e5b20`).
        5 => {
            counted(p, |p| p.bytes(12).map(drop))?;
            run(p, &[4, 12, 4])?;
        }
        // SafeStraightLine (`006e60e0`).
        6 => {
            run(p, &[4, 4])?;
            p.ref_id()?;
        }
        // CoveredMove (`006e4fd0`).
        8 => {
            run(p, &[4, 4, 4, 4, 4, 4, 12, 12, 1])?;
            counted(p, |p| p.u32().map(drop))?;
            counted(p, |p| p.bytes(12).map(drop))?;
        }
        _ => return Err(Error::new(at, format!("pathing request type {kind}"))),
    }
    Ok(())
}

/// `PathingSolution::SaveGame` (Xbox PDB), `006e8b10`.
pub(crate) fn pathing_solution(p: &mut Pipe<'_>) -> Result<()> {
    run(p, &[1, 4, 4])?;
    counted(p, |p| {
        p.u32()?;
        p.ref_id()?;
        pathing_location(p)
    })?;
    counted(p, |p| {
        run(p, &[4, 12])?;
        p.ref_id()?;
        pathing_location(p)
    })?;
    counted(p, avoid_node)?;
    counted(p, |p| p.ref_id().map(drop))?;
    Ok(())
}

/// `DetailedActorPathHandler::SaveGame` (Xbox PDB), `009e8f80`.
pub(crate) fn detailed_path_handler(p: &mut Pipe<'_>) -> Result<()> {
    run(p, &[12; 5])?;
    run(p, &[4; 26])?;
    run(p, &[1; 9])?;
    run(p, &[4, 12, 4])?;
    p.ref_id()?;
    counted(p, |p| p.ref_id().map(drop))?;
    Ok(())
}

/// Per-actor package data (`ActorPackageData` subclasses, type from
/// vtable +0x8, saved by vtable +0xC); the type is the package type.
pub(crate) fn actor_package_data(p: &mut Pipe<'_>, kind: u8) -> Result<()> {
    let at = p.position();
    match kind {
        // EscortActorPackageData (`009f0bb0`).
        2 => {
            p.ref_id()?;
            counted(p, |p| p.ref_id().map(drop))?;
        }
        // SandBoxActorPackageData (`009f5a60`).
        12 => {
            run(
                p,
                &[4, 4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 1, 4, 1, 4, 4, 4, 4, 4, 6],
            )?;
            p.ref_id()?;
            p.ref_id()?;
            p.ref_id()?;
            counted(p, |p| {
                run(p, &[4, 4, 4])?;
                p.ref_id().map(drop)
            })?;
        }
        // PatrolActorPackageData (`009f33d0`).
        13 => {
            run(p, &[4, 4, 4, 1, 1, 4])?;
            p.ref_id()?;
            counted(p, |p| p.ref_id().map(drop))?;
        }
        // GuardActorPackageData (`009f2850`).
        14 => {
            run(p, &[4, 4, 4])?;
            p.ref_id()?;
        }
        // UseWeaponActorPackageData: saves nothing (`004534f0`).
        16 => {}
        _ => return Err(Error::new(at, format!("actor package data type {kind}"))),
    }
    Ok(())
}

/// `PackageLocation::SaveGame` (Xbox PDB), `0067fb60`.
fn package_location(p: &mut Pipe<'_>) -> Result<()> {
    let kind = p.u8()? as i8;
    p.u32()?;
    if (0..5).contains(&kind) {
        p.ref_id()?;
    } else if kind == 5 {
        p.u32()?;
    }
    Ok(())
}

/// `PackageTarget::SaveGame` (Xbox PDB), `00680720`.
fn package_target(p: &mut Pipe<'_>) -> Result<()> {
    let kind = p.u8()?;
    run(p, &[4, 4])?;
    if kind < 2 {
        p.ref_id()?;
    } else if kind == 2 {
        p.u32()?;
    }
    Ok(())
}

/// A package's own data (`TESPackageData` subclasses, vtable +0x14),
/// by package type (the subclass names match the types).
fn package_data(p: &mut Pipe<'_>, ptype: u8) -> Result<()> {
    let optional_location = |p: &mut Pipe<'_>| -> Result<()> {
        if p.u8()? != 0 {
            package_location(p)?;
        }
        Ok(())
    };
    match ptype {
        // Eat, use item at, ambush (`0067b000`).
        3 | 8 | 9 => optional_location(p),
        // Follow, escort (`0067bf40`).
        1 | 2 => {
            p.u32()?;
            optional_location(p)
        }
        // Patrol (`0067c5e0`).
        13 => run(p, &[1, 1]),
        // Dialogue (`0067b6f0`).
        15 | 28 => {
            run(p, &[4, 1, 1, 1, 4, 1, 4])?;
            p.ref_id()?;
            optional_location(p)
        }
        // Use weapon (`0067d550`).
        16 => {
            run(p, &[1, 1, 1, 1, 1, 1, 2, 2, 2, 4, 4])?;
            p.ref_id()?;
            if p.u8()? != 0 {
                package_target(p)?;
            }
            optional_location(p)
        }
        // TESPackageData itself saves nothing (`004534f0`).
        _ => Ok(()),
    }
}

/// `TESPackage::SaveGame` (Xbox PDB), `006797c0`, for a package made in
/// game (it writes nothing for one from a plugin).
pub(crate) fn tes_package(p: &mut Pipe<'_>, ptype: u8) -> Result<()> {
    p.bytes(12)?;
    let parts = p.u8()?;
    if parts & 1 != 0 {
        package_location(p)?;
    }
    if parts & 2 != 0 {
        package_target(p)?;
    }
    if parts & 4 != 0 {
        package_data(p, ptype)?;
    }
    p.u32()?;
    Ok(())
}

/// A dialogue item (`0083d690`).
fn dialogue_item(p: &mut Pipe<'_>) -> Result<()> {
    p.wstr()?;
    p.wstr()?;
    run(p, &[4, 4, 1])?;
    for _ in 0..3 {
        p.ref_id()?;
    }
    Ok(())
}

/// A topic's dialogue (`0083ce40`).
pub(crate) fn dialogue(p: &mut Pipe<'_>) -> Result<()> {
    counted(p, dialogue_item)?;
    p.u16()?;
    for _ in 0..4 {
        p.ref_id()?;
    }
    Ok(())
}

/// A package made in game, by its type (`PTYPE` (Xbox PDB)); the class
/// is the one `TESPackage::CreatePackage` (Xbox PDB), `00670b90`, makes
/// for the type.
pub(crate) fn package_body(p: &mut Pipe<'_>, ptype: u8, version: u8) -> Result<()> {
    match ptype {
        // AlarmPackage (`009ecec0`, which doesn't save the base).
        21 => {
            counted(p, |p| run(p, &[1, 2]))?;
        }
        // CombatController (`009819f0`).
        18 => combat_controller(p, version)?,
        // DialoguePackage (`009f01d0`).
        15 | 28 => {
            tes_package(p, ptype)?;
            for _ in 0..6 {
                p.ref_id()?;
            }
            run(p, &[4, 1, 1, 1, 1, 1, 1, 1, 4, 4, 1, 1, 1])?;
            if p.u8()? != 0 {
                counted(p, dialogue)?;
                p.u16()?;
                p.u16()?;
            }
        }
        // FleePackage (`009f2380`).
        22 => {
            tes_package(p, ptype)?;
            run(p, &[1, 1, 12, 4, 1, 1, 1])?;
            p.ref_id()?;
            p.ref_id()?;
            counted(p, |p| p.ref_id().map(drop))?;
        }
        // TrespassPackage (`009f9790`).
        23 => {
            tes_package(p, ptype)?;
            run(p, &[4, 4, 4, 4, 4])?;
            p.ref_id()?;
            p.ref_id()?;
        }
        // SpectatorPackage (`009f87d0`).
        24 => {
            tes_package(p, ptype)?;
            run(p, &[4, 4, 4, 4, 1, 12])?;
            counted(p, |p| {
                run(p, &[4, 4, 4, 12, 12, 1, 1])?;
                p.ref_id()?;
                p.ref_id().map(drop)
            })?;
        }
        // BackUpPackage (`009edb60`).
        39 => {
            tes_package(p, ptype)?;
            p.bytes(12)?;
        }
        // TESPackage and SearchPackage.
        _ => tes_package(p, ptype)?,
    }
    Ok(())
}

/// The package an actor is running (`ActorPackage`, saved by `008c65d0`):
/// the package, for one made in game its type and data, the per-actor
/// package data, and three values and a target.
pub(crate) fn actor_package(p: &mut Pipe<'_>, version: u8) -> Result<Option<RefId>> {
    let package = p.ref_id()?;
    if package == RefId(0) {
        return Ok(None);
    }
    if package.is_created() {
        let ptype = p.u8()?;
        if ptype != 0xFF {
            package_body(p, ptype, version)?;
        }
    }
    let data = p.u8()?;
    if data != 0xFF {
        actor_package_data(p, data)?;
    }
    run(p, &[4, 4, 4])?;
    p.ref_id()?;
    Ok(Some(package))
}

/// A combat target's cover data (`009a2de0`).
fn combat_target_cover(p: &mut Pipe<'_>) -> Result<()> {
    run(p, &[1, 1, 1, 4, 12])?;
    cover_location(p)?;
    run(p, &[4, 4])?;
    run(p, &[1; 6])
}

/// `00527b70`: a position and a cell or worldspace.
fn location_ref(p: &mut Pipe<'_>) -> Result<()> {
    p.bytes(12)?;
    p.ref_id().map(drop)
}

/// A search area (`009a2f50`).
fn combat_search(p: &mut Pipe<'_>) -> Result<()> {
    run(p, &[12, 12, 4, 1, 4, 4, 4])?;
    counted(p, cover_location)?;
    Ok(())
}

/// `CombatState` (`009a17a0`).
fn combat_state(p: &mut Pipe<'_>) -> Result<()> {
    run(p, &[1, 4])?;
    for _ in 0..6 {
        p.ref_id()?;
    }
    counted(p, |p| p.ref_id().map(drop))?;
    p.ref_id()?;
    run(p, &[4; 13])?;
    run(p, &[1; 8])?;
    p.ref_id()?;
    run(p, &[4, 4, 12, 4, 1, 1, 4, 4, 4, 4, 1])?;
    p.ref_id()?;
    run(p, &[4, 4, 4, 4, 4])?;
    timer(p)?;
    timer(p)?;
    run(p, &[4])?;
    timer(p)?;
    run(p, &[4])?;
    let parts = p.u8()?;
    if parts & 0x01 != 0 {
        combat_target_cover(p)?;
    }
    if parts & 0x04 != 0 {
        combat_target_cover(p)?;
    }
    if parts & 0x10 != 0 {
        location_ref(p)?;
        run(p, &[4, 4, 4])?;
        p.ref_id()?;
    }
    if parts & 0x20 != 0 {
        run(p, &[1, 1, 4, 4])?;
        p.ref_id()?;
        location_ref(p)?;
    }
    if parts & 0x02 != 0 {
        combat_search(p)?;
    }
    if parts & 0x08 != 0 {
        combat_search(p)?;
    }
    counted(p, |p| run(p, &[4, 4]))?;
    counted(p, |p| {
        location_ref(p)?;
        run(p, &[4])
    })?;
    run(p, &[4, 1])?;
    let actors = p.u8()?;
    for i in 0..5 {
        if actors & (1 << i) != 0 {
            p.ref_id()?;
            p.ref_id()?;
        }
    }
    for _ in 0..2 {
        p.ref_id()?;
        timer(p)?;
    }
    p.u32()?;
    p.ref_id()?;
    run(p, &[1, 4])?;
    for _ in 0..11 {
        timer(p)?;
    }
    Ok(())
}

/// A combat procedure (`CombatProcedure` subclasses: type vtable +0x34,
/// saved by vtable +0x38; `00982350` writes 0xFF for none).
fn combat_procedure(p: &mut Pipe<'_>) -> Result<()> {
    let at = p.position();
    let kind = p.u8()?;
    if kind == 0xFF {
        return Ok(());
    }
    // CombatProcedure itself (`00996ed0`).
    p.u32()?;
    match kind {
        // AttackRanged (`009d2310`).
        0 => {
            run(p, &[4, 12])?;
            for _ in 0..5 {
                timer(p)?;
            }
        }
        // AttackMelee (`009d0690`).
        1 => {
            run(p, &[4, 4, 4, 4])?;
            for _ in 0..6 {
                timer(p)?;
            }
            run(p, &[4, 1, 4])?;
        }
        // AttackGrenade (`009cb9c0`).
        2 => {
            run(p, &[4, 1, 12, 12, 1])?;
            timer(p)?;
        }
        // AttackLow (`009cbfd0`).
        3 => {
            run(p, &[4, 4, 4])?;
            p.ref_id()?;
        }
        // Evade (`009d5fb0`).
        4 => run(p, &[4, 4])?,
        // SwitchWeapon, UseCombatItem (`009da970`).
        5 | 11 => {
            p.ref_id()?;
            p.u8()?;
        }
        // Move (`009d8880`).
        6 => {
            p.u32()?;
            location_ref(p)?;
            location_ref(p)?;
            p.u32()?;
            p.ref_id()?;
            run(p, &[4, 4])?;
            for _ in 0..4 {
                timer(p)?;
            }
            run(p, &[12, 1, 1])?;
        }
        // BeInCover (`009d3800`).
        7 => {
            run(p, &[4, 1, 12, 12, 12])?;
            for _ in 0..6 {
                timer(p)?;
            }
        }
        // ActivateObject (`009cacb0`).
        8 => {
            p.ref_id()?;
            p.ref_id()?;
            p.u8()?;
        }
        // HideFromTarget (`009d6820`).
        9 => {
            run(p, &[4, 4])?;
            timer(p)?;
            run(p, &[12, 12, 1, 1, 1])?;
        }
        // Search (`009da5c0`).
        10 => {
            p.u32()?;
            location_ref(p)?;
            for _ in 0..3 {
                timer(p)?;
            }
            p.ref_id()?;
        }
        // EngageTarget (`009d5880`).
        12 => {
            run(p, &[4, 4, 4, 4])?;
            timer(p)?;
            timer(p)?;
            run(p, &[1, 1, 1, 12])?;
            location_ref(p)?;
            location_ref(p)?;
            run(p, &[1, 4])?;
        }
        _ => return Err(Error::new(at, format!("combat procedure type {kind}"))),
    }
    Ok(())
}

/// `CombatController::SaveGame` (Xbox PDB), `009819f0`.
fn combat_controller(p: &mut Pipe<'_>, _version: u8) -> Result<()> {
    tes_package(p, 18)?;
    p.ref_id()?;
    p.ref_id()?;
    p.u32()?;
    combat_state(p)?;
    combat_procedure(p)?;
    combat_procedure(p)?;
    counted(p, combat_procedure)?;
    // CombatPlanner (`009948f0`).
    if p.u8()? != 0 {
        let n = p.u32()?;
        if n as usize > p.remaining() / 7 {
            return Err(Error::new(p.position(), format!("{n} plans don't fit")));
        }
        for _ in 0..n {
            run(p, &[1, 4])?;
        }
        run(p, &[4, 1])?;
    }
    run(p, &[1, 4])?;
    timer(p)?;
    run(p, &[1, 4, 4, 4, 1, 1, 1, 4, 4, 12, 1, 12, 1, 1, 1, 1])
}

/// A magic target's effects list, re-exported for the processes.
pub(crate) fn effects(p: &mut Pipe<'_>) -> Result<()> {
    active_effects(p).map(drop)
}
