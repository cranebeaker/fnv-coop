//! Conformance rows of a remote player's avatar (SPEC-06 slices 1 and 2): another machine's
//! player as an actor of this game.
//!
//! The rows need a backend that runs the game's own actors frame by frame: the engine
//! backend ([`crate::engine_peer`]). On another backend each row is PENDING. Each row has a
//! mutation twin (`mutate`): the row breaks its own scenario, and then it must fail.
//!
//! The test harness is the network in these rows: it gives the avatar each frame's command.
//! The live path between two games is a later slice.
//!
//! Facts that the rows use (measured on the game program, 2026-11-10, in Doc Mitchell's
//! house): a floor at Z 7361 from X 2090 to 2300 at Y 1611.5. A walk command of 1.4 units a
//! frame with the flags "forward, walk" plays the forward group, and the avatar stays within
//! 3.4 units of its command.

use crate::backend::{bad, need, AvatarState, BackendPeer, CoopBackend, GuestRules, Test, Verdict};
use crate::data::Data;

const SLOT: u8 = 1;
/// The floor of the walk.
const Y: f32 = 1611.5;
const Z: f32 = 7361.0;
const START_X: f32 = 2300.0;
/// One frame's step of the commanded path.
const STEP: f32 = 1.4;
/// The movement flags "forward" and "walk".
const WALK: u32 = 0x101;
/// The forward movement group.
const FORWARD: u32 = 3;
/// The life state of a restrained actor.
const RESTRAINED: u32 = 5;
/// A race of the base game that is not the player base's own (measured: the base's own is
/// 00000019).
const RACE: u32 = 0x0000_424a;

/// A backend with no engine avatar gives PENDING, another fault gives a failure.
fn made(r: Result<u32, String>) -> Result<u32, Verdict> {
    need(r)
}

fn state(p: &dyn BackendPeer, when: &str) -> Result<AvatarState, Verdict> {
    p.avatar_state(SLOT)
        .ok_or_else(|| bad(format!("{when}: the game has no avatar")))
}

fn frames(p: &mut dyn BackendPeer, n: u32) -> Result<(), Verdict> {
    if p.frames(n) {
        Ok(())
    } else {
        Err(bad("the game ran no frame"))
    }
}

/// The avatar follows a commanded walk of 300 frames (150 one way, 150 back): it stays
/// within 5 units of each command, the game plays the forward group, and the hold stays.
/// Twin: with no movement flags the body slides: the forward group does not play.
fn avatar_follows_walk(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut guest = need(b.guest(d, GuestRules::Policy))?;
    made(guest.avatar_make(SLOT, false, 0, [START_X, Y, Z], 270.0))?;
    frames(&mut *guest, 10)?;
    let flags = if mutate { 0 } else { WALK };
    let mut x = START_X;
    let mut worst = 0.0f32;
    let mut walking = 0;
    for frame in 0..300 {
        let back = frame >= 150;
        x += if back { STEP } else { -STEP };
        let heading = if back { 90.0 } else { 270.0 };
        need(guest.avatar_drive(SLOT, [x, Y, Z], heading, flags))?;
        frames(&mut *guest, 1)?;
        let seen = state(&*guest, "in the walk")?;
        worst = worst.max((seen.place[0] - x).abs());
        if (seen.place[1] - Y).abs() > 5.0 {
            return Err(bad(format!(
                "the avatar left its line: y {}",
                seen.place[1]
            )));
        }
        if seen.life != RESTRAINED {
            return Err(bad(format!(
                "the avatar lost its hold: life state {}",
                seen.life
            )));
        }
        if seen.group == FORWARD {
            walking += 1;
        }
    }
    if worst > 5.0 {
        return Err(bad(format!(
            "the avatar was {worst} units from its command"
        )));
    }
    // The first frames start the group.
    if walking < 290 {
        return Err(bad(format!(
            "the avatar slides: the forward group played in {walking} of 300 frames"
        )));
    }
    Ok(())
}

/// A standing avatar holds its place, its heading and its restraint for 300 frames with no
/// command. Twin: the game's own command takes the restraint away, and no command gives it
/// back.
fn avatar_holds_place(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut guest = need(b.guest(d, GuestRules::Policy))?;
    let id = made(guest.avatar_make(SLOT, false, 0, [START_X, Y, Z], 270.0))?;
    frames(&mut *guest, 10)?;
    let before = state(&*guest, "before the wait")?;
    if mutate && !guest.adapter_line(&format!("\"{id:08X}\".SetRestrained 0")) {
        return Err(bad("the game did not run the twin's line"));
    }
    frames(&mut *guest, 300)?;
    let after = state(&*guest, "after the wait")?;
    if after.life != RESTRAINED {
        return Err(bad(format!(
            "the avatar is not held: life state {}",
            after.life
        )));
    }
    let moved = (0..3)
        .map(|i| (after.place[i] - before.place[i]).abs())
        .fold(0.0f32, f32::max);
    if moved > 1.0 || (after.heading - before.heading).abs() > 1.0 {
        return Err(bad(format!(
            "the avatar moved {moved} units, heading {} to {}",
            before.heading, after.heading
        )));
    }
    // A command gives a lost hold back (the driver's rule): the row's own check of it.
    need(guest.avatar_drive(SLOT, [START_X, Y, Z], 270.0, 0))?;
    frames(&mut *guest, 2)?;
    if state(&*guest, "after a command")?.life != RESTRAINED {
        return Err(bad("a command did not give the hold back"));
    }
    Ok(())
}

/// A removed avatar is gone from the game (its form id answers nothing), and the same
/// remote player gets an avatar again. Twin: with no removal the old body stays.
fn avatar_removed_and_made_again(
    b: &dyn CoopBackend,
    d: &Data,
    mutate: bool,
) -> Result<(), Verdict> {
    let mut guest = need(b.guest(d, GuestRules::Policy))?;
    let id = made(guest.avatar_make(SLOT, false, 0, [START_X, Y, Z], 270.0))?;
    frames(&mut *guest, 10)?;
    if !guest.has_form(id) {
        return Err(bad("the game does not know the new avatar's id"));
    }
    if !mutate && !guest.avatar_remove(SLOT) {
        return Err(bad("the game did not take the removal"));
    }
    // The game destroys the object in its next frame.
    frames(&mut *guest, 5)?;
    if guest.has_form(id) {
        return Err(bad("the old body is still in the game"));
    }
    if guest.avatar_state(SLOT).is_some() {
        return Err(bad("the removed avatar still has a state"));
    }
    made(guest.avatar_make(SLOT, false, 0, [START_X - 50.0, Y, Z], 270.0))?;
    frames(&mut *guest, 30)?;
    let again = state(&*guest, "after the second make")?;
    if again.life != RESTRAINED || (again.place[0] - (START_X - 50.0)).abs() > 1.0 {
        return Err(bad(format!("the second avatar is wrong: {again:?}")));
    }
    Ok(())
}

/// The avatar has the remote player's sex and race, by the game's own tests. Twin: a base
/// that was not written has neither.
fn avatar_look(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut guest = need(b.guest(d, GuestRules::Policy))?;
    let (female, race) = if mutate { (false, 0) } else { (true, RACE) };
    made(guest.avatar_make(SLOT, female, race, [START_X, Y, Z], 270.0))?;
    frames(&mut *guest, 10)?;
    match guest.avatar_look(SLOT, RACE) {
        Some((true, true)) => Ok(()),
        other => Err(bad(format!(
            "the avatar's look is {other:?}, want a female body of the race {RACE:08X}"
        ))),
    }
}

/// Local code does not kill an avatar: after lethal damage it lives and keeps its hold, one
/// kill was stopped, and the heal gives its health back. Twin: with no damage rules the
/// avatar dies.
fn avatar_survives_local_damage(
    b: &dyn CoopBackend,
    d: &Data,
    mutate: bool,
) -> Result<(), Verdict> {
    let mut guest = need(b.guest(d, GuestRules::Policy))?;
    let id = made(guest.avatar_make(SLOT, false, 0, [START_X, Y, Z], 270.0))?;
    frames(&mut *guest, 10)?;
    if !mutate {
        need(guest.avatar_guard())?;
    }
    if !guest.adapter_line(&format!("\"{id:08X}\".DamageAV Health 100000")) {
        return Err(bad("the game did not run the damage line"));
    }
    frames(&mut *guest, 30)?;
    let after = state(&*guest, "after lethal damage")?;
    if after.life != RESTRAINED {
        return Err(bad(format!(
            "the avatar is dead or loose: life state {}",
            after.life
        )));
    }
    match guest.avatar_hits() {
        Some((1, hits)) if hits.is_empty() => {}
        other => return Err(bad(format!("want one stopped kill and no hit: {other:?}"))),
    }
    if !guest.avatar_heal(SLOT) {
        return Err(bad("the game did not heal the avatar"));
    }
    frames(&mut *guest, 2)?;
    match guest.avatar_health(SLOT) {
        Some(part) if (part - 1.0).abs() < 0.001 => Ok(()),
        other => Err(bad(format!("the avatar's health is {other:?}, want 1"))),
    }
}

/// The local player shoots at an avatar three times. Each hit is recorded with the player as
/// its source and the weapon, and the local game applies none: the avatar's health stays.
/// Twin: with no damage rules the health goes down.
fn avatar_hit_recorded_not_applied(
    b: &dyn CoopBackend,
    d: &Data,
    mutate: bool,
) -> Result<(), Verdict> {
    /// The 9 mm pistol, and the player's reference.
    const PISTOL: u32 = 0x000e_3778;
    const PLAYER: u32 = 0x14;
    let mut guest = need(b.guest_that_draws(d))?;
    // 100 units in front of the player's start place, which looks down the X axis.
    made(guest.avatar_make(SLOT, false, 0, [2250.0, 1603.6, 7360.6], 90.0))?;
    frames(&mut *guest, 10)?;
    if !mutate {
        need(guest.avatar_guard())?;
    }
    for _ in 0..3 {
        if !guest.adapter_line(&format!("player.FireWeapon {PISTOL:08X}")) {
            return Err(bad("the game did not run the shot"));
        }
        frames(&mut *guest, 20)?;
    }
    match guest.avatar_health(SLOT) {
        Some(part) if (part - 1.0).abs() < 0.001 => {}
        other => {
            return Err(bad(format!(
                "the local game applied the hits: the avatar's health is {other:?}"
            )))
        }
    }
    let (_, hits) = guest
        .avatar_hits()
        .ok_or_else(|| bad("the game has no hit record"))?;
    let good = hits
        .iter()
        .filter(|h| h.slot == SLOT && h.source == PLAYER && h.weapon == PISTOL && h.damage > 0.0)
        .count();
    if hits.len() != 3 || good != 3 {
        return Err(bad(format!(
            "want three recorded pistol hits, got {hits:?}"
        )));
    }
    if !guest
        .avatar_hits()
        .is_some_and(|(_, again)| again.is_empty())
    {
        return Err(bad("a hit was given two times"));
    }
    Ok(())
}

/// No save holds an avatar, and no load meets one: the avatar leaves the game before the
/// game's save runs, the same remote player gets an avatar again at once, and after a load
/// the slot is empty and takes a new avatar with the right look. Twin: with no rules the
/// avatar is in the game when the save runs.
///
/// Measured on the game program: an avatar of a temporary base copy does not come back from
/// a save in each case, so the row checks the moment of the save.
fn avatar_not_in_save(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    const SAVE: &str = "cooprow1";
    // A save draws its picture: the game must draw.
    let mut guest = need(b.guest_that_draws(d))?;
    let id = made(guest.avatar_make(SLOT, true, 0, [START_X, Y, Z], 270.0))?;
    frames(&mut *guest, 10)?;
    if !mutate {
        need(guest.avatar_guard())?;
    }
    if !guest.adapter_line(&format!("save {SAVE}")) {
        return Err(bad("the game did not run the save line"));
    }
    frames(&mut *guest, 30)?;
    if guest.has_form(id) {
        return Err(bad("the avatar was in the game when the save ran"));
    }
    if guest.avatar_state(SLOT).is_some() {
        return Err(bad("the slot still has an avatar after the save"));
    }
    // The next gap makes it again.
    made(guest.avatar_make(SLOT, true, 0, [START_X, Y, Z], 270.0))?;
    frames(&mut *guest, 10)?;
    if state(&*guest, "after the save")?.life != RESTRAINED {
        return Err(bad("the avatar that was made again is not held"));
    }
    if !guest.adapter_line(&format!("load {SAVE}")) {
        return Err(bad("the game did not run the load line"));
    }
    frames(&mut *guest, 400)?;
    if guest.avatar_state(SLOT).is_some() {
        return Err(bad("an avatar is in the slot after the load"));
    }
    made(guest.avatar_make(SLOT, true, 0, [START_X, Y, Z], 270.0))?;
    frames(&mut *guest, 30)?;
    if state(&*guest, "after the load")?.life != RESTRAINED {
        return Err(bad("the avatar after the load is not held"));
    }
    match guest.avatar_look(SLOT, RACE) {
        Some((true, _)) => Ok(()),
        other => Err(bad(format!(
            "the avatar after the load has the look {other:?}, want a female body"
        ))),
    }
}

/// The avatar's rows: id and function.
pub const TESTS: &[(&str, Test)] = &[
    ("avatar_follows_walk", avatar_follows_walk),
    ("avatar_holds_place", avatar_holds_place),
    (
        "avatar_removed_and_made_again",
        avatar_removed_and_made_again,
    ),
    ("avatar_look", avatar_look),
    ("avatar_survives_local_damage", avatar_survives_local_damage),
    (
        "avatar_hit_recorded_not_applied",
        avatar_hit_recorded_not_applied,
    ),
    ("avatar_not_in_save", avatar_not_in_save),
];
