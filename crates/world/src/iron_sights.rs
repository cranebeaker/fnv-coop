//! Looking down the sights, as `FalloutNV.exe` 1.4.0.525 does it for the
//! player:
//!
//! - the Aim control (6, the right mouse button) held or just pressed
//!   (`0093e860`, at `00941f4f`) puts a drawn gun's sights up
//!   (`008bb650(1, 0, 0)`, `SetIronSights` on the process, vfunc +0x400);
//!   letting go takes them down. Not while the view is switching (want
//!   ≠ shown third person), in V.A.T.S., with the weapon away, or in anim
//!   action 7. A drawn melee weapon or fists block instead (`00894cc0(1)`;
//!   [`aim_control`] says which). Sneaking toggled takes them down.
//! - the field of view (`0095de30`, every frame): the world's and the
//!   first-person pass's eased toward the weapon's sight field of view
//!   (`WEAP` `DNAM` f32 at 28, the weapon's +0x110, as an absolute value)
//!   at 30 × the frame's seconds ÷ `fIronSightsFOVTimeChange` degrees a
//!   frame, the first-person one scaled by the default first-person over
//!   world ratio ([`step_fov`]).
//! - the animations: the weapon's groups switch to their `…IS` variant
//!   (group + 3; `008bb650` via `00897910`): the first-person files
//!   `<kind>aimis.kf`, `<kind>attack…is.kf` ([`first_person_is`]).
//! - running is off while aiming (`world::locomotion::may_run`).
//! - with `bTrueIronSights` (on by default) the weapon's `##SightingNode`
//!   is kept (`008bbbf0`, player +0xe34 `m_pWeaponSightingNode` (Xbox
//!   PDB)) and the HUD hides the crosshair while it's set in first person
//!   (`00771700`: mask bit 0x40 cleared).
//!
//! Scopes ([`scoped`], [`scope_model`]): the viewer's `scope` draws the
//! overlay and sways the view; the gun's sway is `world::gun_wobble`;
//! blocking is `world::melee`. Not here: weapon mods' zoom (mod effect
//! 0xe, taken off the sight field of view in `0095de30`: this viewer keeps
//! no weapon mods), the iron-sights depth of field (`009650a0`).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_f32;

/// Below this the sight field of view isn't used (the double 5.0 at
/// `01020998`).
pub const MIN_SIGHT_FOV: f32 = 5.0;

/// What the field-of-view easing reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FovSettings {
    /// `fDefaultWorldFOV:Display` (exe 75) and `fDefault1stPersonFOV:Display`
    /// (exe 75; INI files often set 55).
    pub world: f32,
    pub first_person: f32,
    /// `fIronSightsZoomDefault:Combat` (50): with no weapon out.
    pub zoom_default: f32,
    /// `fIronSightsFOVTimeChange` (0.25).
    pub time_change: f32,
    /// `bIronSightsZoomEnable:Combat` (1).
    pub enabled: bool,
}

impl Default for FovSettings {
    fn default() -> Self {
        FovSettings {
            world: 75.0,
            first_person: 75.0,
            zoom_default: 50.0,
            time_change: 0.25,
            enabled: true,
        }
    }
}

/// The player's two fields of view (`PlayerCharacter` +0x670 `fWorldFOV`
/// (Xbox PDB) and +0x674, the first-person pass's).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fov {
    pub world: f32,
    pub first_person: f32,
}

/// A weapon's sight field of view (`WEAP` `DNAM` f32 at 28, the weapon's
/// +0x110 `00508070`, made positive `00408860`).
pub fn sight_fov(order: &LoadOrder, weapon: FormId) -> Option<f32> {
    let record = order.get(weapon)?.record().ok()?;
    let dnam = record
        .get(FourCC::new(b"DNAM"))
        .filter(|s| s.data.len() >= 32)?;
    Some(le_f32(&dnam.data, 28).abs())
}

/// A weapon's scope model (`MOD3`, the weapon's `+0x1e8` model;
/// `interface\HUD\scope01.nif` on the hunting rifle).
pub fn scope_model(order: &LoadOrder, weapon: FormId) -> Option<String> {
    let record = order.get(weapon)?.record().ok()?;
    let model = record.get(FourCC::new(b"MOD3"))?.zstring();
    (!model.trim().is_empty()).then_some(model)
}

/// `DNAM` flags2 (u32 at 56) 0x2000, "scope from mod": the scope is only
/// there with a mod giving the zoom effect (0xe) installed.
pub const SCOPE_FROM_MOD: u32 = 0x2000;

/// Whether looking down the sights of `weapon` is looking through its
/// scope (`008bb650`, `00962de0`): it has a scope model and, when the
/// scope comes from a mod, a mod with the zoom effect (0xe) is installed
/// (`004bd8d0(0xe)`). `zoom_mod`: the installed mods' zoom effect value,
/// `None` without one (this viewer keeps no weapon mods yet, so always
/// `None`).
pub fn scoped(order: &LoadOrder, weapon: FormId, flags2: u32, zoom_mod: Option<f32>) -> bool {
    scope_model(order, weapon).is_some() && (flags2 & SCOPE_FROM_MOD == 0 || zoom_mod.is_some())
}

/// One frame of `0095de30` outside V.A.T.S., unscoped: the targets are the
/// defaults, or while aiming (and not staggered: anim actions 9 and 0x11)
/// with a sight field of view above [`MIN_SIGHT_FOV`] that field of view
/// and its share of the first-person default; each field of view moves
/// toward its target by 30 × `dt` ÷ `fIronSightsFOVTimeChange` degrees,
/// not past it. `sight`: the weapon's ([`sight_fov`]) when one is out,
/// else `None` (`fIronSightsZoomDefault`).
// Translated from 0095de30 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn step_fov(
    fov: &mut Fov,
    settings: &FovSettings,
    aiming: bool,
    sight: Option<f32>,
    staggered: bool,
    dt: f32,
) {
    if !settings.enabled {
        return;
    }
    let sight = sight.unwrap_or(settings.zoom_default);
    let rate = dt * 30.0 / settings.time_change;
    let (mut world, mut first) = (settings.world, settings.first_person);
    if aiming && sight > MIN_SIGHT_FOV && !staggered {
        world = sight;
        first = sight / settings.world * settings.first_person;
    }
    let toward = |current: f32, target: f32| {
        if current > target {
            (current - rate).max(target)
        } else if current < target {
            (current + rate).min(target)
        } else {
            current
        }
    };
    fov.world = toward(fov.world, world);
    fov.first_person = toward(fov.first_person, first);
}

/// What the Aim control does with what's in hand (`0093e860`): a drawn
/// gun aims (iron sights); a drawn melee weapon (animation types 0–2,
/// `006450c0`) or no weapon blocks; nothing with the weapon away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AimControl {
    IronSights,
    Block,
    Nothing,
}

/// [`AimControl`] for the weapon in hand (`None`: fists) and whether it's
/// out (process vfunc +0x454, `008a16d0`).
pub fn aim_control(weapon_animation: Option<u32>, out: bool) -> AimControl {
    if !out {
        return AimControl::Nothing;
    }
    match weapon_animation {
        Some(kind) if kind > 2 => AimControl::IronSights,
        _ => AimControl::Block,
    }
}

/// The first-person iron-sights variant of a first-person animation file:
/// `…aim.kf` → `…aimis.kf`, `…attack8.kf` → `…attack8is.kf` (the game's
/// groups + 3, `008bb650`; the files beside the skeleton).
pub fn first_person_is(path: &str) -> String {
    match path.strip_suffix(".kf") {
        Some(stem) => format!("{stem}is.kf"),
        None => format!("{path}is"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_view_eases_to_the_sights_and_back() {
        let s = FovSettings {
            first_person: 55.0,
            ..FovSettings::default()
        };
        let mut fov = Fov {
            world: 75.0,
            first_person: 55.0,
        };
        // 1/60 s frames: 30 / 60 / 0.25 = 2 degrees a frame.
        step_fov(&mut fov, &s, true, Some(60.0), false, 1.0 / 60.0);
        assert!((fov.world - 73.0).abs() < 1e-4, "{}", fov.world);
        // The first-person pass aims at 60 / 75 × 55 = 44.
        assert!((fov.first_person - 53.0).abs() < 1e-4);
        for _ in 0..20 {
            step_fov(&mut fov, &s, true, Some(60.0), false, 1.0 / 60.0);
        }
        assert_eq!(fov.world, 60.0);
        assert!((fov.first_person - 44.0).abs() < 1e-4);
        // Let go: back to the defaults.
        for _ in 0..20 {
            step_fov(&mut fov, &s, false, Some(60.0), false, 1.0 / 60.0);
        }
        assert_eq!((fov.world, fov.first_person), (75.0, 55.0));
        // No weapon out: the default zoom; a tiny sight FOV: none.
        step_fov(&mut fov, &s, true, None, false, 10.0);
        assert_eq!(fov.world, 50.0);
        step_fov(&mut fov, &s, true, Some(4.0), false, 10.0);
        assert_eq!(fov.world, 75.0);
        // Turned off in the INI: nothing moves.
        let off = FovSettings {
            enabled: false,
            ..s
        };
        step_fov(&mut fov, &off, true, Some(30.0), false, 10.0);
        assert_eq!(fov.world, 75.0);
    }

    #[test]
    fn guns_aim_melee_and_fists_block() {
        assert_eq!(aim_control(Some(5), true), AimControl::IronSights);
        assert_eq!(aim_control(Some(3), true), AimControl::IronSights);
        assert_eq!(aim_control(Some(1), true), AimControl::Block);
        assert_eq!(aim_control(None, true), AimControl::Block);
        assert_eq!(aim_control(Some(5), false), AimControl::Nothing);
        assert_eq!(
            first_person_is("Characters\\_1stPerson\\2hraim.kf"),
            "Characters\\_1stPerson\\2hraimis.kf"
        );
    }
}
