//! The crosshair's pick: which reference the player looks at, as
//! FalloutNV.exe 1.4.0.525's view caster (`ViewCaster.cpp`, the source file
//! its asserts name) finds it each frame for the HUD's Info panel and the
//! Activate control.
//!
//! `0070bc20` (the HUD's crosshair update) makes a sphere phantom of
//! `fActivatePickSphereRadius` (`00631ab0`: a sphere shape, filter word
//! layer 40 `CUSTOMPICK2` with the player's system group) and asks
//! `00631d60` for what lies along the camera's view (its forward column,
//! `00439f50(1)`) within `iActivatePickLength`. `00631d60`:
//!
//! 1. Moves the phantom to the eye and casts it linearly to the end
//!    (`setPositionAndLinearCast`, `00c9eba0`) with an all-hits collector,
//!    sorted (`00cabad0`): every shape the sphere touches whose layer
//!    layer 40 touches (`physics::layers`; layer 40 also ignores the
//!    "no collision" flag, `00c84740`).
//! 2. For each hit in order (not the player, not a reference without one):
//!    an exact pick along the view's line, either Havok's (a ray against
//!    that one body's shape, `00632c30`, at most
//!    `iMaxViewCasterPicksHavok` bodies) for rigid bodies on the clutter,
//!    weapon and projectile layers (4–6), or a static-layer body whose shape
//!    is of type 10, or of one particular base form (`011ca240`); else
//!    Gamebryo's (`NiPick` of the reference's 3D, triangles, both faces,
//!    all results sorted, `004b2780`, at most `iMaxViewCasterPicksGamebryo`
//!    references). Then, the "fuzzy" candidate (`bUseFuzzyPicking`): hits
//!    on layers other than static (1), terrain (13), ground (17) and
//!    portal (18) (projectile (6) only for a body; an actor's controller
//!    (30) not while it sits or sleeps, see below) whose touching point is
//!    nearer the view's line than the best so far, at most
//!    `iMaxViewCasterPicksFuzzy` of them checked by a ray from the eye to
//!    that point (layer 40's filter) meeting nothing or that reference
//!    first.
//! 3. The nearer of the exact picks (Gamebryo's first result, Havok's only
//!    when strictly nearer). If there is none, or it's a static, static
//!    collection, tree or a light that can't be carried (base form types
//!    0x20, 0x21, 0x25, 0x1e; the caller's [`Scene::fuzzy_replaces`]), the
//!    fuzzy candidate takes its place, at the distance from the eye to its
//!    touching point.
//!
//! Translated from 00631d60 (decompiled, FalloutNV.exe 1.4.0.525). The
//! settings' values are the exe's (`00f56a90`, `00f56ac0`, `00f56af0`,
//! `00f56b20`; nothing in `Fallout_default.ini` changes them).
//!
//! What the caller's [`Scene`] stands for (this viewer has no Havok world
//! or scene graph): the shapes are the collider's triangles (each placed
//! reference's collision) and capsules for actors; both exact picks are a
//! ray against the reference's own shapes (Gamebryo's would be against its
//! drawn triangles). The static-layer "shape type 10" and form `011ca240`
//! cases of the Havok branch aren't told apart here (both branches cast the
//! same ray). The actor exclusion (`00632d00` on the controller == 1 and
//! the process's state, vtable +0x4bc, 4 or 9: the values `GetSitting` and
//! `GetSleeping` count as sitting and sleeping, `0059dd90`, `0059dc90`)
//! needs the controller's +0x600, which isn't named: [`Scene::excluded`]
//! is asked and the viewer answers no.

use crate::layers::layer;
use crate::vec::*;
use crate::Vec3;

/// The phantom's layer (`00631ab0`: `004a39f0(0x28)`).
pub const PICK_LAYER: u8 = layer::CUSTOMPICK2;

/// The view caster's settings (`[Interface]`), with the exe's values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// `bUseFuzzyPicking` (1).
    pub fuzzy: bool,
    /// `iMaxViewCasterPicksFuzzy` (5).
    pub max_fuzzy: u32,
    /// `iMaxViewCasterPicksHavok` (10).
    pub max_havok: u32,
    /// `iMaxViewCasterPicksGamebryo` (10).
    pub max_gamebryo: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            fuzzy: true,
            max_fuzzy: 5,
            max_havok: 10,
            max_gamebryo: 10,
        }
    }
}

/// One shape the phantom's cast touched.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CastHit {
    /// The placed reference it belongs to (0: none).
    pub reference: u32,
    /// Its body (`004b59f0`: the collidable's entity), `None` for a phantom
    /// (an actor's character controller).
    pub body: Option<u64>,
    /// Its Havok layer.
    pub layer: u8,
    /// Where the sphere touches it.
    pub point: Vec3,
}

/// What the pick asks of the world it looks into.
pub trait Scene {
    /// How far along the view's line the reference's own shapes are met
    /// (passing everything else), if they are.
    fn exact(&self, reference: u32) -> Option<f32>;
    /// The reference a ray from the eye to `to` meets first (0 for a shape
    /// with no reference), `None` when it meets nothing.
    fn first_on_line(&self, to: Vec3) -> Option<u32>;
    /// Whether the fuzzy candidate may take the place of this exact pick
    /// (its base is a static, static collection, tree, a light that can't
    /// be carried, or it has none).
    fn fuzzy_replaces(&self, reference: u32) -> bool;
    /// Whether an actor's controller hit can't be the fuzzy candidate
    /// (sitting or sleeping, see the module notes).
    fn excluded(&self, reference: u32) -> bool;
}

/// What the crosshair is on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pick {
    pub reference: u32,
    /// From the eye: along the view for an exact pick; to the touching point
    /// for a fuzzy one.
    pub distance: f32,
    /// The fuzzy candidate took the place of the exact pick (`00631d60`'s
    /// fifth argument).
    pub fuzzy: bool,
    /// Where: on the view's line, or the touching point.
    pub point: Vec3,
}

/// The pick (`00631d60`): `hits` in the order the cast found them (nearest
/// first), `skip` the player.
// Translated from 00631d60 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn pick(
    origin: Vec3,
    direction: Vec3,
    hits: &[CastHit],
    skip: u32,
    scene: &impl Scene,
    s: &Settings,
) -> Option<Pick> {
    let dir = normalize(direction);
    // Havok's exact picks (by body) and Gamebryo's (by reference).
    let mut havok: Option<(u32, f32)> = None;
    let mut gamebryo: Option<(u32, f32)> = None;
    let (mut havok_count, mut gamebryo_count, mut fuzzy_count) = (0u32, 0u32, 0u32);
    let mut last_body: Option<u64> = None;
    let mut last_reference: Option<u32> = None;
    // The fuzzy candidate: reference, distance, its point's distance from
    // the view's line, the point.
    let mut fuzzy: Option<(u32, f32, Vec3)> = None;
    let mut fuzzy_off_line = f32::MAX;
    for h in hits {
        let r = h.reference;
        if r == 0 || r == skip {
            continue;
        }
        let havok_branch = h.body.is_some() && (4..=6).contains(&h.layer);
        if havok_branch && h.body != last_body {
            last_body = h.body;
            if havok_count < s.max_havok {
                havok_count += 1;
                if let Some(d) = scene.exact(r) {
                    if havok.map_or(true, |(_, bd)| d < bd) {
                        havok = Some((r, d));
                    }
                }
            }
        } else if last_reference != Some(r) && gamebryo_count < s.max_gamebryo {
            // (A body Havok's pick already had goes to Gamebryo's too, by
            // its reference.)
            gamebryo_count += 1;
            if let Some(d) = scene.exact(r) {
                if gamebryo.map_or(true, |(_, bd)| d < bd) {
                    gamebryo = Some((r, d));
                }
            }
        }
        let may_be_fuzzy = match h.layer {
            1 | 13 | 17 | 18 => false,
            6 => h.body.is_some(),
            30 => !scene.excluded(r),
            _ => true,
        };
        if may_be_fuzzy && s.fuzzy {
            let v = sub(h.point, origin);
            let along = scale(dir, dot(v, dir));
            let off_line = length(sub(v, along));
            if off_line < fuzzy_off_line && fuzzy_count < s.max_fuzzy {
                if fuzzy.is_some_and(|(fr, _, _)| fr == r) {
                    fuzzy = Some((r, length(v), h.point));
                    fuzzy_off_line = off_line;
                } else {
                    fuzzy_count += 1;
                    let seen = scene.first_on_line(h.point);
                    if seen.map_or(true, |first| first == r) {
                        fuzzy = Some((r, length(v), h.point));
                        fuzzy_off_line = off_line;
                    }
                }
            }
        }
        last_reference = Some(r);
    }
    let mut out = match (gamebryo, havok) {
        (Some((_, gd)), Some((hr, hd))) if hd < gd => Some((hr, hd)),
        (Some(g), _) => Some(g),
        (None, h) => h,
    }
    .map(|(reference, distance)| Pick {
        reference,
        distance,
        fuzzy: false,
        point: add(origin, scale(dir, distance)),
    });
    if s.fuzzy {
        let replace = out.map_or(true, |p| scene.fuzzy_replaces(p.reference));
        if let Some((r, d, point)) = fuzzy {
            if replace && out.map_or(true, |p| p.reference != r) {
                out = Some(Pick {
                    reference: r,
                    distance: d,
                    fuzzy: true,
                    point,
                });
            }
        }
    }
    out
}

/// An actor's shape for the pick: a living one's character controller (a
/// phantom on layer 30, `body` `None`), or a dead one's ragdoll bones
/// (bodies on their own layer).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capsule {
    pub reference: u32,
    pub body: Option<u64>,
    pub layer: u8,
    pub a: Vec3,
    pub b: Vec3,
    pub radius: f32,
}

/// The world the pick looks into: the collider's triangles (by the placed
/// reference each comes from) and the actors' capsules, seen from `origin`
/// along `direction`, and the caller's answers about references.
pub struct World<'a, R: Fn(u32) -> bool, E: Fn(u32) -> bool> {
    pub collider: &'a crate::Collider,
    pub capsules: &'a [Capsule],
    pub origin: Vec3,
    pub direction: Vec3,
    /// [`Scene::fuzzy_replaces`].
    pub fuzzy_replaces: R,
    /// [`Scene::excluded`].
    pub excluded: E,
}

impl<R: Fn(u32) -> bool, E: Fn(u32) -> bool> World<'_, R, E> {
    /// The phantom's cast (step 1 of the module notes): a sphere of
    /// `radius` from the eye for `length`, on [`PICK_LAYER`], every shape
    /// it touches nearest first.
    pub fn cast(&self, length: f32, radius: f32) -> Vec<CastHit> {
        let filter = crate::layers::Filter::shared();
        let dir = normalize(self.direction);
        let c = self.collider;
        let mut hits: Vec<(f32, CastHit)> = c
            .spherecast_all(self.origin, dir, length, radius, PICK_LAYER)
            .into_iter()
            .map(|h| {
                let reference = c.reference(h.triangle);
                let layer = c.layer(h.triangle);
                (
                    h.distance,
                    CastHit {
                        reference,
                        // One body per reference and layer here.
                        body: Some(u64::from(reference) << 8 | u64::from(layer)),
                        layer,
                        point: h.point,
                    },
                )
            })
            .collect();
        for k in self.capsules {
            if !filter.layers_touch(PICK_LAYER, k.layer) {
                continue;
            }
            if let Some((d, point)) =
                crate::sphere_sweep_capsule(self.origin, dir, length, radius, (k.a, k.b, k.radius))
            {
                hits.push((
                    d,
                    CastHit {
                        reference: k.reference,
                        body: k.body,
                        layer: k.layer,
                        point,
                    },
                ));
            }
        }
        hits.sort_by(|a, b| a.0.total_cmp(&b.0));
        hits.into_iter().map(|(_, h)| h).collect()
    }

    /// The whole pick: [`Self::cast`] then [`pick`].
    pub fn pick(&self, length: f32, radius: f32, skip: u32, s: &Settings) -> Option<Pick> {
        let hits = self.cast(length, radius);
        pick(self.origin, self.direction, &hits, skip, self, s)
    }
}

impl<R: Fn(u32) -> bool, E: Fn(u32) -> bool> Scene for World<'_, R, E> {
    fn exact(&self, reference: u32) -> Option<f32> {
        let dir = normalize(self.direction);
        let shapes = self.collider.raycast_reference(self.origin, dir, reference);
        let capsules = self
            .capsules
            .iter()
            .filter(|k| k.reference == reference)
            .filter_map(|k| crate::ray_capsule(self.origin, dir, k.a, k.b, k.radius))
            .min_by(f32::total_cmp);
        match (shapes, capsules) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }

    fn first_on_line(&self, to: Vec3) -> Option<u32> {
        let v = sub(to, self.origin);
        let d = length(v);
        if d < 1e-6 {
            return None;
        }
        let n = scale(v, 1.0 / d);
        let filter = crate::layers::Filter::shared();
        let mut best = self
            .collider
            .raycast_layer(self.origin, n, d, PICK_LAYER)
            .map(|(t, tri)| (t, self.collider.reference(tri)));
        for k in self.capsules {
            if !filter.layers_touch(PICK_LAYER, k.layer) {
                continue;
            }
            if let Some(t) = crate::ray_capsule(self.origin, n, k.a, k.b, k.radius) {
                if t <= d && best.map_or(true, |(bt, _)| t < bt) {
                    best = Some((t, k.reference));
                }
            }
        }
        best.map(|(_, r)| r)
    }

    fn fuzzy_replaces(&self, reference: u32) -> bool {
        (self.fuzzy_replaces)(reference)
    }

    fn excluded(&self, reference: u32) -> bool {
        (self.excluded)(reference)
    }
}

/// Whether a pick at `distance` is within the activation reach
/// (`0070bc20`: the activation target is set only within
/// `iActivatePickLength`, ends included, when the HUD's +0x75e flag is
/// set; otherwise only the crosshair's).
pub fn within(distance: f32, length: f32) -> bool {
    distance <= length
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A scene of upright boxes' worth of answers: exact distances by
    /// reference, the references a line meets in order of distance, and
    /// which are statics.
    struct Fake {
        exact: Vec<(u32, f32)>,
        line: Vec<(u32, f32)>,
        statics: Vec<u32>,
    }

    impl Scene for Fake {
        fn exact(&self, r: u32) -> Option<f32> {
            self.exact.iter().find(|e| e.0 == r).map(|e| e.1)
        }
        fn first_on_line(&self, to: Vec3) -> Option<u32> {
            let d = length(to);
            self.line
                .iter()
                .filter(|l| l.1 <= d + 1e-3)
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map(|l| l.0)
        }
        fn fuzzy_replaces(&self, r: u32) -> bool {
            self.statics.contains(&r)
        }
        fn excluded(&self, _: u32) -> bool {
            false
        }
    }

    fn hit(reference: u32, layer: u8, point: Vec3) -> CastHit {
        CastHit {
            reference,
            body: Some(u64::from(reference)),
            layer,
            point,
        }
    }

    const X: Vec3 = [1.0, 0.0, 0.0];

    #[test]
    fn the_nearest_exact_pick_wins() {
        let scene = Fake {
            exact: vec![(2, 80.0), (3, 60.0)],
            line: vec![],
            statics: vec![],
        };
        let hits = [hit(2, 10, [70.0, 0.0, 5.0]), hit(3, 4, [55.0, 0.0, 8.0])];
        let p = pick([0.0; 3], X, &hits, 7, &scene, &Settings::default()).unwrap();
        assert_eq!((p.reference, p.distance, p.fuzzy), (3, 60.0, false));
    }

    #[test]
    fn a_static_under_the_line_gives_way_to_the_fuzzy_candidate() {
        // The view passes just over a bottle (4, clutter) and meets the
        // table it stands on (9, a static): the table is the exact pick,
        // the bottle the fuzzy one.
        let scene = Fake {
            exact: vec![(9, 90.0)],
            line: vec![(4, 70.0), (9, 90.0)],
            statics: vec![9],
        };
        let hits = [hit(9, 1, [60.0, 0.0, -14.0]), hit(4, 4, [70.0, 0.0, -3.0])];
        let p = pick([0.0; 3], X, &hits, 7, &scene, &Settings::default()).unwrap();
        assert_eq!(p.reference, 4);
        assert!(p.fuzzy);
        assert!((p.distance - length([70.0, 0.0, -3.0])).abs() < 1e-4);
        // Without fuzzy picking the table stays.
        let off = Settings {
            fuzzy: false,
            ..Settings::default()
        };
        assert_eq!(
            pick([0.0; 3], X, &hits, 7, &scene, &off).unwrap().reference,
            9
        );
    }

    #[test]
    fn the_fuzzy_candidate_must_be_seen_and_nearest_the_line() {
        // Two things beside the line: 5 nearer the line but behind a wall
        // (8, a static the line to it meets first), 6 farther off it.
        let scene = Fake {
            exact: vec![],
            line: vec![(8, 40.0), (6, 31.0)],
            statics: vec![],
        };
        let hits = [hit(6, 10, [30.0, 10.0, 0.0]), hit(5, 10, [100.0, 2.0, 0.0])];
        let p = pick([0.0; 3], X, &hits, 7, &scene, &Settings::default()).unwrap();
        assert_eq!(p.reference, 6);
        // Static, terrain and the player are never fuzzy candidates.
        let hits = [
            hit(8, 1, [40.0, 1.0, 0.0]),
            hit(11, 13, [45.0, 1.0, 0.0]),
            hit(7, 30, [1.0, 0.0, 0.0]),
        ];
        assert_eq!(
            pick([0.0; 3], X, &hits, 7, &scene, &Settings::default()),
            None
        );
    }

    #[test]
    fn a_furniture_exact_pick_isnt_replaced() {
        let scene = Fake {
            exact: vec![(12, 100.0)],
            line: vec![],
            statics: vec![],
        };
        let hits = [hit(13, 10, [50.0, 1.0, 0.0]), hit(12, 1, [95.0, 0.0, 0.0])];
        let p = pick([0.0; 3], X, &hits, 7, &scene, &Settings::default()).unwrap();
        assert_eq!((p.reference, p.fuzzy), (12, false));
    }

    /// A box's corners and its twelve triangles.
    fn cube(lo: Vec3, hi: Vec3) -> (Vec<Vec3>, Vec<[u32; 3]>) {
        let v = (0..8)
            .map(|i| {
                [
                    if i & 1 == 0 { lo[0] } else { hi[0] },
                    if i & 2 == 0 { lo[1] } else { hi[1] },
                    if i & 4 == 0 { lo[2] } else { hi[2] },
                ]
            })
            .collect();
        let t = vec![
            [0, 1, 3],
            [0, 3, 2],
            [4, 6, 7],
            [4, 7, 5],
            [0, 4, 5],
            [0, 5, 1],
            [2, 3, 7],
            [2, 7, 6],
            [0, 2, 6],
            [0, 6, 4],
            [1, 5, 7],
            [1, 7, 3],
        ];
        (v, t)
    }

    fn place(c: &mut crate::Collider, lo: Vec3, hi: Vec3, layer: u8, reference: u32) {
        let (v, t) = cube(lo, hi);
        c.add_placed(&v, &t, (0.7, 0, crate::NO_MATERIAL), None, layer, reference);
    }

    const TABLE: u32 = 0x10;
    const BOTTLE: u32 = 0x11;
    const CHAIR: u32 = 0x12;
    const WALL: u32 = 0x13;
    const PERSON: u32 = 0x14;
    const PLAYER: u32 = 0x7;

    /// Doc's kitchen in miniature: a table (a static) 80 high, a bottle on
    /// it (clutter, 6 × 6 × 24, its body's triangles owned as the
    /// simulation adds them), a chair beside the table, a wall behind.
    fn kitchen() -> crate::Collider {
        let mut c = crate::Collider::new();
        place(&mut c, [80.0, -40.0, 0.0], [160.0, 40.0, 80.0], 1, TABLE);
        let (v, t) = cube([97.0, -3.0, 80.0], [103.0, 3.0, 104.0]);
        c.add_layered(&v, &t, (0.7, BOTTLE, crate::NO_MATERIAL), None, 4);
        place(&mut c, [80.0, 60.0, 0.0], [120.0, 100.0, 90.0], 1, CHAIR);
        place(&mut c, [200.0, -300.0, 0.0], [210.0, 300.0, 300.0], 1, WALL);
        c
    }

    fn look<'a>(
        c: &'a crate::Collider,
        capsules: &'a [Capsule],
        origin: Vec3,
        at: Vec3,
    ) -> Option<Pick> {
        let w = World {
            collider: c,
            capsules,
            origin,
            direction: normalize(sub(at, origin)),
            // The table and the wall are statics; the chair is furniture.
            fuzzy_replaces: |r| r == TABLE || r == WALL,
            excluded: |_| false,
        };
        w.pick(150.0, 16.0, PLAYER, &Settings::default())
    }

    #[test]
    fn aiming_at_the_middle_of_a_bottle_on_a_table_picks_it() {
        // Before: the sphere's first touch (the table top, 8 units under
        // the line) took the pick, so only looking above the bottle worked.
        let c = kitchen();
        let eye = [0.0, 0.0, 92.0];
        let first = c.spherecast(eye, [1.0, 0.0, 0.0], 150.0, 16.0).unwrap();
        assert_eq!(c.reference(first.triangle), TABLE);
        let p = look(&c, &[], eye, [100.0, 0.0, 92.0]).unwrap();
        assert_eq!((p.reference, p.fuzzy), (BOTTLE, false));
        assert!((p.distance - 97.0).abs() < 0.01, "{}", p.distance);
        // From standing height, looking down at it.
        let p = look(&c, &[], [0.0, 0.0, 130.0], [100.0, 0.0, 90.0]).unwrap();
        assert_eq!(p.reference, BOTTLE);
        // Just beside it the line meets the table: the bottle, nearest the
        // line of what the sphere touched, is the fuzzy pick.
        let p = look(&c, &[], eye, [100.0, 9.0, 84.0]).unwrap();
        assert_eq!((p.reference, p.fuzzy), (BOTTLE, true));
    }

    #[test]
    fn a_chair_is_picked_by_its_shape_and_a_wall_hides_what_is_behind() {
        let c = kitchen();
        let p = look(&c, &[], [0.0, 80.0, 120.0], [100.0, 80.0, 60.0]).unwrap();
        assert_eq!((p.reference, p.fuzzy), (CHAIR, false));
        // Behind the wall, a bottle isn't seen (the wall is the exact pick,
        // a static with no fuzzy candidate in sight).
        let mut c = kitchen();
        let (v, t) = cube([215.0, -3.0, 80.0], [221.0, 3.0, 104.0]);
        c.add_layered(&v, &t, (0.7, 0x20, crate::NO_MATERIAL), None, 4);
        let p = look(&c, &[], [120.0, 0.0, 130.0], [218.0, 0.0, 92.0]);
        assert!(p.map_or(true, |p| p.reference != 0x20), "{p:?}");
    }

    #[test]
    fn people_are_picked_by_their_controller_even_in_front_of_a_table() {
        let c = kitchen();
        // Someone standing between the eye and the table.
        let k = [Capsule {
            reference: PERSON,
            body: None,
            layer: 30,
            a: [50.0, 0.0, 21.0],
            b: [50.0, 0.0, 108.0],
            radius: 20.25,
        }];
        let p = look(&c, &k, [0.0, 0.0, 110.0], [50.0, 0.0, 90.0]).unwrap();
        assert_eq!((p.reference, p.fuzzy), (PERSON, false));
        // Looking past their shoulder at the wall: the fuzzy pick.
        let p = look(&c, &k, [0.0, 0.0, 110.0], [100.0, 45.0, 140.0]).unwrap();
        assert_eq!(p.reference, PERSON);
        assert!(p.fuzzy);
        // The player's own controller is never picked.
        let me = [Capsule {
            reference: PLAYER,
            ..k[0]
        }];
        assert_eq!(look(&c, &me, [50.0, 0.0, 110.0], [50.0, 0.0, 0.0]), None);
    }

    #[test]
    fn at_most_five_fuzzy_checks() {
        // Each new candidate is nearer the line than the last, but all are
        // hidden: after five checks, a visible sixth isn't looked at.
        let scene = Fake {
            exact: vec![],
            line: vec![(99, 20.0)],
            statics: vec![],
        };
        let mut hits: Vec<CastHit> = (0..5)
            .map(|i| hit(20 + i, 10, [50.0, 10.0 - i as f32, 0.0]))
            .collect();
        hits.push(hit(30, 10, [5.0, 0.5, 0.0]));
        assert_eq!(
            pick([0.0; 3], X, &hits, 7, &scene, &Settings::default()),
            None
        );
    }
}
