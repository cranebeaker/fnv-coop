//! Impacts on screen: what `hiteffects` asks for when a shot or blow
//! strikes, by the rules read from the game's code.
//!
//! - **Effect models** (`006890b0`, `00c4b8a0`, `cellview::impacts`): the
//!   impact's model at the point, its Z axis along the impact's orientation
//!   (`world::impacts::effect_axis`) turned by U(0, 1) radians, for its
//!   animation's length (else what it was given: the impact's duration on
//!   the world, a second for blood), its own controllers played from when
//!   it appears and its billboards facing the camera.
//! - **World decals** (`world::decals`): the impact's decal clipped onto the
//!   struck objects' `NiTriStrips` pieces (and the land's), drawn with the
//!   texture set's diffuse and normal map over the surface (pulled toward
//!   the camera as the game's decals are), lit like it, faded as the game
//!   fades them and kept to the game's limits.
//!
//! Not done: decals on people (skinned decals, `004a2070`) and on the
//! attacker's weapon (`0088fb00`), the decal's parallax (`TX04`; the
//! viewer has no parallax), effect models' particle systems (left out,
//! said in the log), and blood sprays following the struck bone.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::view::NoFrustumCulling;
use cellview::impacts::{effect_piece_at, effect_placement, EffectModel};
use cellview::space;
use esm::FormId;
use world::decals::{self, DecalBox, DecalMesh, DecalRolls};
use world::impacts::{Impact, Material, TextureSet};

use crate::lighting::GameLitMaterial;
use crate::scripts::PlacedRef;
use crate::walk::game_point;
use crate::{FlyCamera, GameFiles, Spawner};

/// A piece the game puts world decals on: drawn from an `NiTriStrips`
/// whose shader isn't itself a decal (`004a1a70`, `004a2020(0x1a)`).
#[derive(Component)]
pub struct DecalReceiver;

/// A quarter of the land. The game finds the land's geometry as an
/// `NiTriStrips` (`009c20e0`: `004b59f0`, then `00c85ef0` with the type at
/// `011f4a20`) and gives it decals without the placed objects' check.
#[derive(Component)]
pub struct LandReceiver;

/// An effect model to put at a hit.
#[derive(Debug, Clone)]
pub struct EffectRequest {
    pub model: String,
    pub point: [f32; 3],
    /// Its Z axis (`world::impacts::effect_axis`, or against the blood's
    /// spray).
    pub axis: [f32; 3],
    /// U(0, 1), radians about its Z axis.
    pub roll: f32,
    /// How long without an animation (`world::impacts::effect_lifetime`).
    pub given: f32,
}

/// A decal to put where a hit struck.
#[derive(Debug, Clone)]
pub struct DecalRequest {
    /// The impact whose decal data (`DODT`), texture set and angle
    /// threshold it is.
    pub impact: FormId,
    pub point: [f32; 3],
    pub normal: [f32; 3],
    pub rolls: DecalRolls,
    /// The box's depth when not the decal data's (wall spatter: 48).
    pub depth: Option<f32>,
    /// What takes it: placed references, and the land.
    pub targets: DecalTargets,
}

/// The objects a decal goes on.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecalTargets {
    pub references: Vec<u32>,
    pub land: bool,
}

/// This frame's effects and decals (`hiteffects` fills it).
#[derive(Resource, Default)]
pub struct ImpactRequests {
    pub effects: Vec<EffectRequest>,
    pub decals: Vec<DecalRequest>,
}

/// One effect on screen.
struct LiveEffect {
    model: Arc<EffectModel>,
    /// Entity, its piece in the model, its draw, the point its mesh is
    /// built around.
    pieces: Vec<(Entity, usize, usize, Vec3)>,
    placement: nif::Transform,
    age: f32,
    lifetime: f32,
}

/// One decal (one piece's) on screen.
struct LiveDecal {
    entity: Entity,
    age: f32,
}

/// The effects and decals on screen, and the models read.
#[derive(Resource, Default)]
pub struct ImpactFx {
    models: HashMap<String, Option<Arc<EffectModel>>>,
    effects: Vec<LiveEffect>,
    decals: VecDeque<LiveDecal>,
    /// An effect model is on screen with some of its life left (for
    /// `NV_SHOT_ON_IMPACT`).
    pub shown: bool,
}

pub struct ImpactFxPlugin;

impl Plugin for ImpactFxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImpactRequests>()
            .init_resource::<ImpactFx>()
            .add_systems(
                Update,
                (start_impacts, age_impacts)
                    .chain()
                    // Kept: the hits' effects start the impacts. Placed for
                    // order only with them, after the actors' movement pass
                    // (`crate::frame_order::AiSet`), outside its gate (their
                    // exact place isn't traced).
                    .after(crate::hiteffects::play_hits)
                    .in_set(crate::frame_order::FrameSet::Stage(
                        world::frame::Stage::AiStart,
                    ))
                    .before(crate::frame_order::AiSet::next(
                        crate::frame_order::ACTORS_MOVEMENT,
                    )),
            )
            .add_systems(
                PostUpdate,
                place_effects.after(bevy::transform::TransformSystem::TransformPropagate),
            );
    }
}

/// The decal settings from the INI (`fDecalLifetime`, `uMaxDecals`,
/// `iMaxDecalsPerFrame`), else the exe's defaults.
struct DecalSettings {
    lifetime: f32,
    max: usize,
    per_frame: usize,
}

impl DecalSettings {
    fn of(game: &cellview::Game) -> DecalSettings {
        let s = &game.settings;
        DecalSettings {
            lifetime: s
                .float("Display", "fDecalLifetime")
                .unwrap_or(decals::LIFETIME),
            max: s
                .float("Decals", "uMaxDecals")
                .map_or(decals::MAX_DECALS, |v| v.max(0.0) as usize),
            per_frame: s
                .float("Display", "iMaxDecalsPerFrame")
                .map_or(decals::MAX_PER_FRAME, |v| v.max(0.0) as usize),
        }
    }
}

/// The impact material a collision triangle counts as: its Havok
/// material's, the land's dirt without one (`00457880`).
pub fn triangle_material(collider: &physics::Collider, t: u32) -> Material {
    collider
        .material(t)
        .map_or(Material::Dirt, |h| Material::from_havok(h & 0x1f))
}

/// Whether a placed reference's base takes decals (`004a1060`: not
/// placeable water, people, creatures or projectiles).
pub fn takes_decals(order: &esm::LoadOrder, reference: u32) -> bool {
    let base = world::scripting::base_of(order, FormId(reference));
    !base.and_then(|b| order.get(b)).is_some_and(|r| {
        [b"PWAT", b"NPC_", b"CREA", b"PROJ"]
            .iter()
            .any(|k| r.entry.header.kind == esm::FourCC::new(k))
    })
}

/// What a struck triangle is for decals: its placed reference, or the land
/// (reference 0 without a material: the land's collision).
pub fn struck_target(
    order: &esm::LoadOrder,
    collider: &physics::Collider,
    t: u32,
) -> Option<DecalTargets> {
    let reference = collider.reference(t);
    if reference == 0 {
        return collider.material(t).is_none().then(|| DecalTargets {
            references: Vec::new(),
            land: true,
        });
    }
    takes_decals(order, reference).then(|| DecalTargets {
        references: vec![reference],
        land: false,
    })
}

/// The objects within `reach` of `point` whose collision there is of
/// `material` (`009c20e0`: the shapes the `DecalCaster` query finds whose
/// struck sub-shape has the impact's material; only what lies in the
/// decal's box can take any of it, so the box is what's looked through).
pub fn decal_targets(
    order: &esm::LoadOrder,
    collider: &physics::Collider,
    point: [f32; 3],
    reach: f32,
    material: Material,
) -> DecalTargets {
    let lo = point.map(|c| c - reach);
    let hi = point.map(|c| c + reach);
    let mut out = DecalTargets::default();
    for t in collider.near(lo, hi) {
        let tri = collider.triangle(t);
        let outside =
            (0..3).any(|k| tri.iter().all(|p| p[k] < lo[k]) || tri.iter().all(|p| p[k] > hi[k]));
        if outside || triangle_material(collider, t) != material {
            continue;
        }
        let Some(found) = struck_target(order, collider, t) else {
            continue;
        };
        out.land |= found.land;
        for r in found.references {
            if !out.references.contains(&r) {
                out.references.push(r);
            }
        }
    }
    out
}

/// A collision triangle's normal, facing back along a ray going `dir`.
pub fn facing_normal(tri: [[f32; 3]; 3], dir: [f32; 3]) -> Option<[f32; 3]> {
    let [a, b, c] = tri;
    let e1 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let e2 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let n = [
        e1[1] * e2[2] - e1[2] * e2[1],
        e1[2] * e2[0] - e1[0] * e2[2],
        e1[0] * e2[1] - e1[1] * e2[0],
    ];
    let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if l <= 1e-9 {
        return None;
    }
    let n = n.map(|v| v / l);
    let along = n[0] * dir[0] + n[1] * dir[1] + n[2] * dir[2];
    Some(if along > 0.0 { n.map(|v| -v) } else { n })
}

type Receivers<'w, 's> = Query<
    'w,
    's,
    (
        &'static Mesh3d,
        &'static GlobalTransform,
        Option<&'static PlacedRef>,
        Has<LandReceiver>,
    ),
    Or<(With<DecalReceiver>, With<LandReceiver>)>,
>;

/// Puts this frame's effects and decals on screen.
pub fn start_impacts(
    game: Res<GameFiles>,
    mut requests: ResMut<ImpactRequests>,
    mut fx: ResMut<ImpactFx>,
    mut spawner: Spawner,
    receivers: Receivers,
) {
    if requests.effects.is_empty() && requests.decals.is_empty() {
        return;
    }
    let game = game.0.clone();
    let effects = std::mem::take(&mut requests.effects);
    let decal_requests = std::mem::take(&mut requests.decals);
    let Some(lighting) = spawner.place_lighting.get() else {
        return;
    };
    for r in effects {
        let key = r.model.to_ascii_lowercase();
        let Some(model) = fx
            .models
            .entry(key)
            .or_insert_with(|| game.effect_model(&r.model).map(Arc::new))
            .clone()
        else {
            continue;
        };
        let lifetime = world::impacts::effect_lifetime(model.animation, r.given);
        let rotation = world::impacts::effect_rotation(r.axis, r.roll);
        let placement = effect_placement(r.point, rotation);
        let entities = spawner.spawn_with(&model.scene, Some(lighting));
        let mut spawned = entities.into_iter();
        let mut pieces = Vec::new();
        for (k, (i, draw)) in model
            .scene
            .draws
            .iter()
            .enumerate()
            .filter(|(_, d)| d.actor.is_none())
            .enumerate()
        {
            let Some(entity) = spawned.next() else {
                break;
            };
            let center =
                Vec3::from(crate::sort_center(&model.scene.meshes[draw.mesh]).unwrap_or([0.0; 3]));
            pieces.push((entity, k, i, center));
            spawner
                .commands
                .entity(entity)
                .remove::<(crate::Moving, crate::Facing)>()
                .insert((NoFrustumCulling, Visibility::Hidden));
        }
        // Water or particles in the model aren't placed with it.
        for rest in spawned {
            if let Ok(mut e) = spawner.commands.get_entity(rest) {
                e.despawn();
            }
        }
        println!(
            "  impact effect {} at {:.0},{:.0},{:.0} for {lifetime:.2} s{}",
            r.model,
            r.point[0],
            r.point[1],
            r.point[2],
            if model.particles {
                " (its particle systems left out)"
            } else {
                ""
            }
        );
        fx.effects.push(LiveEffect {
            model,
            pieces,
            placement,
            age: 0.0,
            lifetime,
        });
    }
    if decal_requests.is_empty() {
        return;
    }
    let settings = DecalSettings::of(&game);
    let mut made = 0;
    for r in decal_requests {
        let order = &game.order;
        let Some(impact) = Impact::load(order, r.impact) else {
            continue;
        };
        let (Some(decal), Some(set)) = (
            impact.decal,
            impact.texture_set.and_then(|t| TextureSet::load(order, t)),
        ) else {
            continue;
        };
        let depth = r.depth.unwrap_or(decal.depth);
        let size = decal.size(r.rolls.size, false);
        let b = DecalBox::sized(
            r.point,
            r.normal,
            &decal,
            impact.angle_threshold,
            (size, depth),
            r.rolls,
        );
        let reach = (size * size * 0.5 + depth * depth).sqrt();
        if decal_texture(&game, &set.diffuse, false).is_none() {
            continue;
        }
        let mut pieces = 0;
        for (mesh3d, global, placed, land) in &receivers {
            if made >= settings.per_frame {
                break;
            }
            let wanted = if land {
                r.targets.land
            } else {
                placed.is_some_and(|p| r.targets.references.contains(&p.0))
            };
            if !wanted {
                continue;
            }
            let Some(mesh) = spawner.meshes.get(&mesh3d.0) else {
                continue;
            };
            let Some(geometry) = clip_piece(mesh, global, &b, reach) else {
                continue;
            };
            let scene = decal_scene(&game, &b, geometry, &decal, &set);
            for e in spawner.spawn_with(&scene, Some(lighting)) {
                fx.decals.push_back(LiveDecal {
                    entity: e,
                    age: 0.0,
                });
            }
            made += 1;
            pieces += 1;
        }
        if pieces > 0 {
            println!(
                "  decal {} ({}) {size:.0} wide on {pieces} piece(s) at {:.0},{:.0},{:.0}",
                set.diffuse.as_deref().unwrap_or(""),
                impact.editor_id.as_deref().unwrap_or(""),
                r.point[0],
                r.point[1],
                r.point[2]
            );
        }
        // The oldest go past the limit (`0068be90`).
        while fx.decals.len() > settings.max {
            if let Some(old) = fx.decals.pop_front() {
                if let Ok(mut e) = spawner.commands.get_entity(old.entity) {
                    e.despawn();
                }
            }
        }
    }
}

/// A texture set's texture (`TX00` diffuse, `TX01` normal map: data).
fn decal_texture(
    game: &cellview::Game,
    path: &Option<String>,
    linear: bool,
) -> Option<cellview::TextureData> {
    let path = assets::texture_path(path.as_deref()?);
    let bytes = game.assets.read(&path).ok()??;
    let mut t = cellview::TextureData::from_dds(path, bytes).ok()?;
    t.linear = linear;
    Some(t)
}

/// A piece's triangles in the game's space clipped to a decal's box, if
/// any of it falls in.
fn clip_piece(
    mesh: &Mesh,
    global: &GlobalTransform,
    b: &DecalBox,
    reach: f32,
) -> Option<DecalMesh> {
    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        return None;
    };
    let Some(VertexAttributeValues::Float32x3(normals)) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
    else {
        return None;
    };
    let m = global.compute_matrix();
    let turn = Mat3::from_mat4(m);
    let to_game = |v: Vec3| [v.x, -v.z, v.y];
    let corners: Vec<usize> = match mesh.indices() {
        Some(indices) => indices.iter().collect(),
        None => (0..positions.len()).collect(),
    };
    // The piece's vertices by what they are (the viewer's meshes repeat
    // them per triangle), for sharing as the game shares its own.
    let mut ids: HashMap<[u32; 6], u32> = HashMap::new();
    let mut out = DecalMesh::default();
    let lo = b.origin.map(|c| c - reach);
    let hi = b.origin.map(|c| c + reach);
    for &tri in corners.as_chunks::<3>().0 {
        if tri
            .iter()
            .any(|&i| i >= positions.len() || i >= normals.len())
        {
            continue;
        }
        let p = tri.map(|i| game_point(m.transform_point3(Vec3::from(positions[i]))));
        if (0..3).any(|k| p.iter().all(|v| v[k] < lo[k]) || p.iter().all(|v| v[k] > hi[k])) {
            continue;
        }
        let mut corner = |j: usize| {
            let n = to_game((turn * Vec3::from(normals[tri[j]])).normalize_or_zero());
            let key = [p[j][0], p[j][1], p[j][2], n[0], n[1], n[2]].map(f32::to_bits);
            let next = ids.len() as u32;
            let id = *ids.entry(key).or_insert(next);
            (p[j], n, id)
        };
        let c = [corner(0), corner(1), corner(2)];
        out.add_triangle(b, c);
        if out.full {
            break;
        }
    }
    (!out.is_empty()).then_some(out)
}

/// A decal's piece as a scene of its own: its geometry in the game's space,
/// the texture set's diffuse (and normal map), the decal data's blending
/// (`0068be90`: alpha blending, `DODT` flag 0x02, and alpha testing, 0x04,
/// against 128, `004393e0`, `0094db80`), lit, drawn over the surface.
fn decal_scene(
    game: &cellview::Game,
    b: &DecalBox,
    g: DecalMesh,
    decal: &world::impacts::Decal,
    set: &TextureSet,
) -> cellview::ViewerScene {
    let mut scene = game.made_scene(Vec::new());
    scene.textures = decal_texture(game, &set.diffuse, false)
        .into_iter()
        .collect();
    let normal_index = decal_texture(game, &set.normal, true).map(|n| {
        scene.textures.push(n);
        1
    });
    let blending = decal.flags & world::impacts::Decal::ALPHA_BLENDING != 0;
    let testing = decal.flags & world::impacts::Decal::ALPHA_TESTING != 0;
    let threshold = 128.0 / 255.0;
    let blend = match (blending, testing) {
        (true, true) => cellview::Blend::MaskedBlend(threshold),
        (true, false) => cellview::Blend::Blend,
        (false, true) => cellview::Blend::Mask(threshold),
        (false, false) => cellview::Blend::Opaque,
    };
    // The texture's U runs along the box's U, V along its V.
    let tangents = g
        .normals
        .iter()
        .map(|n| {
            let d = b.u[0] * n[0] + b.u[1] * n[1] + b.u[2] * n[2];
            let t = Vec3::from([b.u[0] - d * n[0], b.u[1] - d * n[1], b.u[2] - d * n[2]])
                .normalize_or_zero();
            let side = Vec3::from(*n).cross(t).dot(Vec3::from(b.v));
            [t.x, t.y, t.z, if side < 0.0 { -1.0 } else { 1.0 }]
        })
        .collect();
    let colors = g
        .colors
        .iter()
        .map(|&[r, gr, bl, a]| {
            [
                cellview::linear(r),
                cellview::linear(gr),
                cellview::linear(bl),
                a,
            ]
        })
        .collect();
    scene.meshes = vec![cellview::MeshData {
        name: "decal".into(),
        shape_name: "decal".into(),
        // Built around the decal's point, which the draw moves it to.
        positions: g
            .positions
            .iter()
            .map(|p| [0, 1, 2].map(|k| p[k] - b.origin[k]))
            .collect(),
        normals: g.normals,
        tangents: normal_index.map(|_| tangents),
        uvs: g.uvs,
        colors: Some(colors),
        indices: g.indices,
        material: cellview::MaterialData {
            texture: Some(0),
            color: [1.0; 4],
            blend,
            alpha_test: testing.then_some((cellview::AlphaTest::Greater, threshold)),
            depth_test: true,
            depth_write: false,
            sort_center: [0.0; 3],
            unlit: false,
            double_sided: false,
            emissive: [0.0; 3],
            glow: None,
            decal: true,
            normal_map: normal_index,
            specular: None,
            unlit_color: [1.0; 3],
            falloff: None,
            environment: None,
            emittance: None,
            shading: preview::cell::Shading::Plain,
            hair_tint: None,
        },
        effect: false,
        rig: None,
        motion: None,
        billboard: None,
        local_map: false,
        actor_part: None,
        strips: false,
    }];
    scene.draws = vec![cellview::Draw {
        mesh: 0,
        transform: cellview::column_major(&nif::Transform {
            translation: b.origin,
            ..nif::Transform::IDENTITY
        }),
        actor: None,
        reference: 0,
    }];
    scene
}

/// Runs the effects' and decals' clocks: effects go at the end of their
/// lifetime (`006828a0`), decals fade and go (`0068c8e0`).
pub fn age_impacts(
    time: Res<Time>,
    game: Res<GameFiles>,
    mut fx: ResMut<ImpactFx>,
    mut commands: Commands,
    pieces: Query<&MeshMaterial3d<GameLitMaterial>>,
    mut materials: ResMut<Assets<GameLitMaterial>>,
) {
    let dt = time.delta_secs();
    if fx.effects.is_empty() && fx.decals.is_empty() {
        return;
    }
    let lifetime = DecalSettings::of(&game.0).lifetime;
    let fx = &mut *fx;
    fx.effects.retain_mut(|e| {
        e.age += dt;
        // A place change took its pieces away.
        let here = e.pieces.iter().all(|p| pieces.get(p.0).is_ok());
        let alive = here && e.age <= e.lifetime;
        if !alive {
            for p in &e.pieces {
                if let Ok(mut c) = commands.get_entity(p.0) {
                    c.despawn();
                }
            }
        }
        alive
    });
    fx.decals.retain_mut(|d| {
        d.age += dt;
        let Ok(material) = pieces.get(d.entity) else {
            return false;
        };
        match decals::fade(d.age, lifetime) {
            Some(strength) => {
                if strength < 1.0 {
                    if let Some(m) = materials.get_mut(&material.0) {
                        m.base.base_color.set_alpha(strength);
                        m.extension.lighting.environment.w = strength;
                    }
                }
                true
            }
            None => {
                if let Ok(mut c) = commands.get_entity(d.entity) {
                    c.despawn();
                }
                false
            }
        }
    });
}

/// An effect piece's drawing: where, whether shown, its material.
type EffectPiece = (
    &'static mut Transform,
    &'static mut GlobalTransform,
    &'static mut Visibility,
    &'static MeshMaterial3d<GameLitMaterial>,
    &'static InheritedVisibility,
);

/// Puts the effects' pieces where their animation has them, turned to the
/// camera where they're billboards, once everything has moved this frame,
/// and gives them their animated opacity.
pub fn place_effects(
    mut fx: ResMut<ImpactFx>,
    cameras: Query<&GlobalTransform, With<FlyCamera>>,
    mut placed: Query<EffectPiece, Without<FlyCamera>>,

    mut materials: ResMut<Assets<GameLitMaterial>>,
) {
    let Ok(camera) = cameras.single().map(|c| c.compute_transform()) else {
        return;
    };
    let game = |v: Vec3| [v.x, -v.z, v.y];
    let eye = game(camera.translation / space::METERS_PER_UNIT);
    let axes = [
        game(camera.rotation * Vec3::X),
        game(camera.rotation * Vec3::Y),
        game(camera.rotation * Vec3::Z),
    ];
    let mut shown = false;
    for e in &fx.effects {
        let place = Mat4::from_cols_array(&cellview::column_major(&e.placement));
        for &(entity, piece, draw, center) in &e.pieces {
            let Some(p) = e.model.pieces.get(piece) else {
                continue;
            };
            let (moved, opacity) = effect_piece_at(
                p,
                e.model.controllers.as_ref(),
                e.age,
                &e.placement,
                eye,
                axes,
            );
            let d = &e.model.scene.draws[draw];
            let draw_world = (place * Mat4::from_cols_array(&d.transform)).to_cols_array();
            let m = Mat4::from_cols_array(&space::matrix(&draw_world))
                * Mat4::from_cols_array(&moved)
                * Mat4::from_translation(center);
            if let Ok((mut t, mut g, mut v, material, inherited)) = placed.get_mut(entity) {
                *t = Transform::from_matrix(m);
                *g = GlobalTransform::from(m);
                if *v != Visibility::Visible {
                    *v = Visibility::Visible;
                }
                if let (Some(a), Some(mat)) = (opacity, materials.get_mut(&material.0)) {
                    mat.base.base_color.set_alpha(a);
                    mat.extension.lighting.environment.w = a;
                }
                // Drawn this frame (made visible in an earlier one), with
                // some of its life left.
                shown |= inherited.get() && e.age + 0.02 < e.lifetime;
            }
        }
    }
    fx.shown = shown;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_struck_triangles_normal_faces_the_shooter_and_targets_follow_material() {
        let tri = [[0.0, -50.0, -50.0], [0.0, 50.0, -50.0], [0.0, 0.0, 50.0]];
        let n = facing_normal(tri, [1.0, 0.0, 0.0]).unwrap();
        assert!((n[0] + 1.0).abs() < 1e-6, "{n:?}");
        let n = facing_normal(tri, [-1.0, 0.0, 0.0]).unwrap();
        assert!((n[0] - 1.0).abs() < 1e-6);
        // A stone wall (Havok 0) placed by reference 0x1234, and the land
        // (no material, no reference) beneath it.
        let mut c = physics::Collider::new();
        c.add_placed(
            &tri,
            &[[0, 1, 2]],
            (0.0, 0, 0),
            None,
            physics::ANY_LAYER,
            0x1234,
        );
        c.add(
            &[
                [-100.0, -100.0, -60.0],
                [100.0, -100.0, -60.0],
                [0.0, 100.0, -60.0],
            ],
            &[[0, 1, 2]],
        );
        let order = esm::LoadOrder::from_plugins(Vec::new()).unwrap();
        let stone = decal_targets(&order, &c, [0.0, 0.0, -45.0], 20.0, Material::Stone);
        assert_eq!(stone.references, vec![0x1234]);
        assert!(!stone.land);
        let dirt = decal_targets(&order, &c, [0.0, 0.0, -45.0], 20.0, Material::Dirt);
        assert!(dirt.references.is_empty());
        assert!(dirt.land);
        // Out of reach: nothing.
        let far = decal_targets(&order, &c, [0.0, 0.0, 500.0], 20.0, Material::Stone);
        assert_eq!(far, DecalTargets::default());
    }
}
