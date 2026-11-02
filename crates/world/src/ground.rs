//! What keeps a walking actor on the ground besides its character
//! controller: the two rules of `MobileObject::Move` (named by its own
//! message, "AI: MobileObject::Move called on '%s' (%08X) with invalid
//! angle."; `0092f260`, FalloutNV.exe 1.4.0.525), which every actor's move
//! goes through (the player's too) before and after the controller.
//!
//! - **Far from the camera** (`0092f849`…`0092fc6e`): a mobile actor other
//!   than the player, further than `fCharControllerWarpDistSqr` (squared)
//!   from the world camera, isn't moved by its controller at all. The move
//!   is added to its position and its feet are put on the navmesh under the
//!   new spot (the triangle the location resolves to, `006d97b0` →
//!   `006d9830`: `ResolveToClosestNavmeshAndTriangle` (Xbox PDB) `006dd6f0`,
//!   then the triangle's height there, `00697980`, within 180 above or
//!   below). With no triangle there the move goes on to the controller as
//!   usual.
//! - **Under the land** (`0093012a`…`00930186`): after the controller has
//!   moved someone outdoors, if the land under them is more than 30 units
//!   above their feet they are put on the land.

use esm::{FormId, LoadOrder};

use crate::ai::NavMesh;

/// The creature `ACBS` flag "immobile" (`005f0c80`: a `CREA` base, form
/// type 0x2b, with flag 0x800000).
pub const IMMOBILE: u32 = 0x0080_0000;

/// Whether an actor's base is an immobile creature (actor vfunc +0x4b4,
/// `0087d750` → `005f0c80`); people never are.
pub fn immobile(order: &LoadOrder, base: FormId) -> bool {
    let creature = order
        .get(base)
        .is_some_and(|r| r.entry.header.kind.as_bytes() == b"CREA");
    creature && crate::activation::actor_flags(order, base) & IMMOBILE != 0
}

/// `fCharControllerWarpDistSqr:HAVOK`: the exe's default, 6,000,000 (the
/// float at `010c4b18`, stored by the setting's initialiser at `00fbcd10`
/// into the setting at `01267bd4`); not set by any of the game's INI
/// files. About 2449.5 units.
pub const CHAR_CONTROLLER_WARP_DIST_SQR: f32 = 6_000_000.0;

/// How far the land may stand above someone's feet before they are put on
/// it (the double 30.0 at `0101db88`, `0093015e`).
pub const UNDER_LAND: f32 = 30.0;

/// How far above or below the feet the navmesh triangle's height may be for
/// a far actor to be put on it (`006d9830` passes 180, 180 to `00697980`).
pub const NAVMESH_WINDOW: f32 = 180.0;

/// What an actor is, for [`moves_without_controller`].
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Mover {
    /// The player (`0092f8a4`: never).
    pub player: bool,
    /// A creature whose `ACBS` flags have "immobile" (0x800000; actor
    /// vfunc +0x4b4, `0087d750` → `005f0c80`).
    pub immobile: bool,
    /// Knocked down (knock state 3, `00437bd0`), dead (+0x22c), dying
    /// (+0x2e8) or unconscious (+0x230) (`0092f7b3`…`0092f81b`).
    pub down: bool,
}

/// Whether `MobileObject::Move` moves this actor without its controller:
/// further than the warp distance from the world camera (`0092f849`: the
/// squared distance from the world scene graph's camera, `011deb7c` +0xac,
/// its world translation +0x8c, greater than the setting), and none of the
/// exceptions of [`Mover`] (`0092f868`…`0092f8aa`).
pub fn moves_without_controller(mover: Mover, camera_distance_sq: f32, warp_dist_sq: f32) -> bool {
    warp_dist_sq < camera_distance_sq && !mover.player && !mover.immobile && !mover.down
}

/// The height a far actor's feet are put at, standing at `p` after the
/// move: the navmesh triangle the location resolves to
/// ([`NavMesh::find_triangle`], `006dd6f0`), its height under the point,
/// when that height is within [`NAVMESH_WINDOW`] of the feet (`00697980`).
/// `None`: no triangle there, and the controller moves them after all
/// (`0092f9c4`).
// Translated from 006d9830 and 00697980 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn navmesh_height(mesh: &NavMesh, p: [f32; 3]) -> Option<f32> {
    let (_, above) = mesh.find_triangle(p)?;
    let height = p[2] + above;
    let below_by = p[2] - height;
    (-NAVMESH_WINDOW..=NAVMESH_WINDOW)
        .contains(&below_by)
        .then_some(height)
}

/// Where someone's feet go after their controller moved them, given the
/// land's height under them outdoors (`None` indoors or with no land
/// loaded there: `004572e0` finds none and leaves −2048, never above): put
/// on the land when it stands more than [`UNDER_LAND`] above them, else
/// left where the controller put them.
// Translated from 0092f260 at 0093012a..00930186 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn kept_above_land(feet_z: f32, land: Option<f32>) -> f32 {
    match land {
        Some(h) if h - feet_z > UNDER_LAND => h,
        _ => feet_z,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn far_movers_skip_the_controller_unless_player_immobile_or_down() {
        let w = CHAR_CONTROLLER_WARP_DIST_SQR;
        let npc = Mover::default();
        // 2449.5 units is the edge: just inside, the controller moves them.
        assert!(!moves_without_controller(npc, 2449.0f32.powi(2), w));
        assert!(moves_without_controller(npc, 2450.0f32.powi(2), w));
        // Exactly at the setting: still the controller (`threshold < d²`).
        assert!(!moves_without_controller(npc, w, w));
        let far = 5000.0f32.powi(2);
        for m in [
            Mover {
                player: true,
                ..npc
            },
            Mover {
                immobile: true,
                ..npc
            },
            Mover { down: true, ..npc },
        ] {
            assert!(!moves_without_controller(m, far, w), "{m:?}");
        }
    }

    #[test]
    fn feet_more_than_thirty_under_the_land_are_put_on_it() {
        assert_eq!(kept_above_land(100.0, Some(129.0)), 100.0);
        assert_eq!(kept_above_land(100.0, Some(130.0)), 100.0);
        assert_eq!(kept_above_land(100.0, Some(130.5)), 130.5);
        // Far under (fallen through): straight back up.
        assert_eq!(kept_above_land(-900.0, Some(8400.0)), 8400.0);
        // Above the land (on a roof, a bridge): left alone.
        assert_eq!(kept_above_land(500.0, Some(100.0)), 500.0);
        // Indoors, or no land: left alone.
        assert_eq!(kept_above_land(-5000.0, None), -5000.0);
    }

    #[test]
    fn far_movers_stand_on_the_navmesh_within_its_window() {
        use crate::ai::NavTriangle;
        // A slope: z rises 0.5 a unit eastward.
        let mesh = NavMesh {
            vertices: vec![
                [0.0, 0.0, 0.0],
                [400.0, 0.0, 200.0],
                [400.0, 400.0, 200.0],
                [0.0, 400.0, 0.0],
            ],
            triangles: vec![
                NavTriangle {
                    vertices: [0, 1, 2],
                    ..Default::default()
                },
                NavTriangle {
                    vertices: [0, 2, 3],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        // Walked uphill off the slope's surface: back onto it.
        let h = navmesh_height(&mesh, [200.0, 100.0, 60.0]).unwrap();
        assert!((h - 100.0).abs() < 1e-3, "{h}");
        let h = navmesh_height(&mesh, [200.0, 100.0, 140.0]).unwrap();
        assert!((h - 100.0).abs() < 1e-3, "{h}");
        // Off the navmesh: none (the controller moves them).
        assert_eq!(navmesh_height(&mesh, [500.0, 100.0, 100.0]), None);
        // Far over it (more than the resolve window): none.
        assert_eq!(navmesh_height(&mesh, [200.0, 100.0, 400.0]), None);
    }
}
