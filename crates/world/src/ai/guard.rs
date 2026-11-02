//! Guard packages (`PKDT` type 14): the `GUARD` procedure (`00902290`,
//! high-process vfunc +0x7f4), the only procedure of their list (41,
//! `011a3f7c`), repeated for as long as the package runs (list 0x29 steps
//! back at `DONE`, `008eeec0`).
//!
//! Each frame: the guarded reference becomes the process target; a seated
//! or sleeping guard stands up first; the guard goes to its place
//! (vfunc +0x7f8 `00912db0`) when not there and its mover is idle, at a
//! walk or a run by the remaining path (`008daa20`); then its state runs
//! (`00902640` go, `009026c0` watch, `00902c80` warn/attack). Watching
//! looks for intruders within the target's value (`PTDT` i32 at 8) of the
//! guarded reference; with nothing found a guard at its place wanders when
//! it has a radius, else turns to an `XMarkerHeading` location's heading.
//!
//! "Near the editor location" (kind 3) sends the guard to where it was
//! placed in the editor (actor vfunc +0x29c, `GetEditorLocationCoord`
//! (Xbox PDB), actor +0x160): a script's `MoveTo` doesn't change that
//! (`SetStartingPosition` (Xbox PDB), actor vfunc +0x46c, is only called
//! when the actor has no editor location, `004698a0`, `0055d760`). So
//! Goodsprings' `GSSettlerCFGunfightPackage` and `GSSettlerAMGunfightPackage`
//! take their settlers back from the `GSSettler0xPositionMarker`s that
//! `VMS16` stage 70 moves them to.
//!
//! Not done here: the intruder scan, warnings and attacks (`009026c0`'s
//! loop, `00902c80`; the Goodsprings guard packages' target value is 0,
//! for which the scan, comparing squared distances against 0, finds no
//! one), the guarded reference's extra data 0x1c override (`0041c8d0`),
//! and the object searches of location kinds 4 and 5.

use esm::{FormId, LoadOrder};

use super::{linked_ref, location_radius_of, Location, Package};
use crate::scripting::GameState;

/// Where a guard goes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Post {
    /// Where it was placed in the editor.
    Editor { position: [f32; 3] },
    /// A reference's position.
    Reference(FormId),
}

/// A guard's place and radii.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plan {
    /// The guarded reference (the process target): the package target's
    /// reference or linked reference, else the guard.
    pub guarded: FormId,
    pub post: Post,
    /// The radius `r` of the at-place test without a location and of the
    /// walk/run choice: the location's radius (`00676280`), else the
    /// target's value, else 30.
    pub radius: f32,
    /// The path's radius: max(r × 0.5, 15) (`00912db0`).
    pub path_radius: f32,
    /// Whether it has a location (`PLDT`): then "at the place" is the
    /// package's own test (vfunc +0x13c), else within `radius` of the
    /// guarded reference.
    pub has_location: bool,
}

/// The guarded reference (`006759e0`): the target's reference (kind 0),
/// the linked reference (kind 3), else the guard itself.
pub fn guarded(order: &LoadOrder, actor: FormId, package: &Package) -> FormId {
    match package.target {
        Some((0, form, _)) if form.0 != 0 => form,
        Some((3, _, _)) => linked_ref(order, actor).unwrap_or(actor),
        _ => actor,
    }
}

/// The guard's place and radii, or `None` when there's nowhere to go
/// (the location's reference missing).
// Translated from 00902290 and 00912db0 (decompiled, FalloutNV.exe
// 1.4.0.525).
pub fn plan(order: &LoadOrder, actor: FormId, package: &Package) -> Option<Plan> {
    let guarded = guarded(order, actor, package);
    let target_value = package.target.map(|t| t.2);
    let (post, radius) = match package.location {
        Some(loc) => (
            post_of(order, actor, guarded, &loc)?,
            location_radius_of(order, &loc) as f32,
        ),
        None => (
            Post::Reference(guarded),
            target_value.map_or(30.0, |v| v as f32),
        ),
    };
    Some(Plan {
        guarded,
        post,
        radius,
        path_radius: (radius * 0.5).max(15.0),
        has_location: package.location.is_some(),
    })
}

fn post_of(order: &LoadOrder, actor: FormId, guarded: FormId, loc: &Location) -> Option<Post> {
    Some(match loc.kind {
        3 => Post::Editor {
            position: crate::scripting::whereabouts(order, actor)?.position,
        },
        2 => Post::Reference(actor),
        6 => Post::Reference(linked_ref(order, actor)?),
        0 if loc.form.0 != 0 => Post::Reference(loc.form),
        // No reference (kinds 1, 4, 5 here): the process target.
        _ => Post::Reference(guarded),
    })
}

/// Whether the guard runs to its place (`008daa20`, called with the path's
/// remaining length, `r` and `2r`): always with the package's 0x2000 flag
/// ("always run", inferred name) or in combat; else, walking, it runs when
/// more than `2r` remains; running, it keeps running while at least `r`
/// remains.
// Translated from 008daa20 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn runs(remaining: f32, radius: f32, running: bool, flags: u32, in_combat: bool) -> bool {
    if flags & 0x2000 != 0 || in_combat {
        return true;
    }
    if running {
        radius <= remaining
    } else {
        remaining > 2.0 * radius
    }
}

/// The heading a guard at its place turns to when it doesn't wander
/// (`009026c0`): an `XMarkerHeading` location reference's (kind 0). "Near
/// the editor location" gives no reference there, so no turn.
pub fn facing(order: &LoadOrder, state: &GameState, package: &Package) -> Option<f32> {
    super::marker_heading(order, state, package)
}

/// Whether a guard at its place wanders (`009026c0`, vfunc +0x84c): with a
/// location, when its radius (`00676280`) isn't 0; without, when the
/// target's value is above 0 (around the guarded reference, that radius).
pub fn wanders(order: &LoadOrder, package: &Package) -> Option<f32> {
    match package.location {
        Some(loc) => {
            let r = location_radius_of(order, &loc);
            (r != 0).then_some(r as f32)
        }
        None => package
            .target
            .map(|t| t.2)
            .filter(|v| *v > 0)
            .map(|v| v as f32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guards_run_far_from_their_post_and_walk_near_it() {
        // Radius 0 (Goodsprings' guard packages): any distance left runs.
        assert!(runs(1.0, 0.0, false, 0, false));
        assert!(!runs(0.0, 0.0, false, 0, false));
        // Radius 100: walking, run beyond 200; running, keep it to 100.
        assert!(!runs(200.0, 100.0, false, 0, false));
        assert!(runs(201.0, 100.0, false, 0, false));
        assert!(runs(100.0, 100.0, true, 0, false));
        assert!(!runs(99.0, 100.0, true, 0, false));
        // Always run, and in combat.
        assert!(runs(0.0, 100.0, false, 0x2000, false));
        assert!(runs(0.0, 100.0, false, 0, true));
    }
}
