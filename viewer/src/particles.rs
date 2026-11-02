//! Particle systems in placed models (`cellview::particles`), run as the
//! game runs them (`world::particles`) and drawn with its particle shaders
//! (`particles.wgsl`). Every frame each placed object's systems are moved on
//! to the seconds since it appeared, with the sequences it plays driving
//! controllers a controller manager owns; then each system's quads, facing
//! the camera, become its mesh, sorted back to front when blended. The fog
//! is the place's (`viewmodel::PlaceLighting`, which outdoors follows the
//! hour), the colour of systems flagged for external emittance follows a
//! region's weather as glows do (`emittance`).

// The shader-layout derive generates checking functions the compiler
// reports as unused.
#![allow(dead_code)]

use bevy::asset::{load_internal_asset, weak_handle, RenderAssetUsages};
use bevy::pbr::{
    ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, MeshVertexBufferLayoutRef, PrimitiveTopology};
use bevy::render::render_resource::{
    AsBindGroup, CompareFunction, RenderPipelineDescriptor, ShaderRef, ShaderType,
    SpecializedMeshPipelineError,
};
use bevy::render::view::NoFrustumCulling;
use cellview::particles::{corners_for_drawing, quad_indices, ParticleData, ParticleLook};
use cellview::{space, Blend, ViewerScene};
use world::particles::{CameraBasis, PlacedParticles};

use crate::dialogue::DialogueState;
use crate::FlyCamera;

const SHADER: Handle<Shader> = weak_handle!("2b8e5f17-4c3a-4d6e-9a1b-7f0c2d8e4a61");

pub type ParticleMaterial = ExtendedMaterial<StandardMaterial, ParticleShader>;

/// What the particle shader reads (`particles.wgsl`).
#[derive(Clone, Copy, Debug, PartialEq, ShaderType, Reflect)]
pub struct ParticleParams {
    /// `MaterialColor`.
    pub color: Vec4,
    pub fog_color: Vec4,
    pub fog_range: Vec4,
    /// Output mode, alpha test and reference, full-brightness scale.
    pub mode: Vec4,
    /// The entity's place, meters.
    pub origin: Vec4,
}

/// The depth test and write the game sets for the system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ParticleKey {
    pub depth_test: bool,
    pub depth_write: bool,
}

impl From<&ParticleShader> for ParticleKey {
    fn from(material: &ParticleShader) -> Self {
        material.key
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[bind_group_data(ParticleKey)]
pub struct ParticleShader {
    #[uniform(100)]
    pub params: ParticleParams,
    pub key: ParticleKey,
    #[texture(101)]
    #[sampler(102)]
    pub texture: Option<Handle<Image>>,
}

impl MaterialExtension for ParticleShader {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let draw = key.bind_group_data;
        if let Some(depth) = descriptor.depth_stencil.as_mut() {
            depth.depth_write_enabled = draw.depth_write;
            if !draw.depth_test {
                depth.depth_compare = CompareFunction::Always;
            }
        }
        if descriptor.vertex.shader != SHADER {
            return Ok(());
        }
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(5),
        ])?];
        Ok(())
    }
}

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER, "particles.wgsl", Shader::from_wgsl);
        app.add_plugins(MaterialPlugin::<ParticleMaterial>::default())
            .add_systems(
                PostUpdate,
                run_particles.before(bevy::transform::TransformSystem::TransformPropagate),
            );
    }
}

/// One system of a placed object, drawn.
struct Drawn {
    entity: Entity,
    mesh: Handle<Mesh>,
    material: Handle<ParticleMaterial>,
    look: ParticleLook,
    /// Drawn back to front (blended, not added).
    sorted: bool,
}

/// A placed object's particle systems running.
#[derive(Component)]
pub struct ParticleEmitter {
    run: PlacedParticles,
    placed: nif::Transform,
    /// When it appeared (seconds of the app's clock).
    appeared: Option<f32>,
    systems: Vec<Option<Drawn>>,
}

/// How the colour reaches the picture (`particles.wgsl`'s `mode.x`).
fn output_mode(look: &ParticleLook) -> f32 {
    match look.blend {
        Blend::Add if look.adds_alpha => 1.0,
        Blend::Add => 2.0,
        Blend::Opaque | Blend::Mask(_) => 3.0,
        _ => 0.0,
    }
}

fn alpha_mode(look: &ParticleLook) -> AlphaMode {
    match look.blend {
        Blend::Add => AlphaMode::Add,
        Blend::Opaque | Blend::Mask(_) => AlphaMode::Opaque,
        _ => AlphaMode::Blend,
    }
}

/// Puts a place's particle systems on screen; returns the entities.
pub(crate) fn spawn_particles(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ParticleMaterial>,
    scene: &ViewerScene,
    textures: &[Option<Handle<Image>>],
) -> Vec<Entity> {
    let mut out = Vec::new();
    for (k, data) in scene.particles.iter().enumerate() {
        out.push(spawn_one(
            commands, meshes, materials, data, textures, k as u32,
        ));
    }
    out
}

fn spawn_one(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ParticleMaterial>,
    data: &ParticleData,
    textures: &[Option<Handle<Image>>],
    seed: u32,
) -> Entity {
    let mut run = PlacedParticles::new(
        &data.model.systems,
        data.model.sequences.clone(),
        data.playing.clone(),
        data.reference ^ seed.wrapping_mul(0x9E37_79B9),
    );
    run.set_motion(
        data.motion
            .iter()
            .filter_map(|&(i, runs)| {
                let s = data.model.all_sequences.get(i)?;
                Some((std::sync::Arc::new(s.clone()), runs))
            })
            .collect(),
    );
    let mut systems = Vec::new();
    let emitter = commands
        .spawn((
            Transform::IDENTITY,
            Visibility::default(),
            crate::SceneEntity,
            crate::scripts::PlacedRef(data.reference),
        ))
        .id();
    for (system, look) in data.model.systems.iter().zip(&data.looks) {
        // Left out: the place's notes say why (`cellview::particles`).
        if !look.drawn {
            systems.push(None);
            continue;
        }
        let origin = Vec3::from(space::point(
            data.placed.then_child(&system.transform()).translation,
        ));
        let (test, threshold) = crate::lighting::alpha_test_code(look.alpha_test);
        let material = materials.add(ParticleMaterial {
            base: StandardMaterial {
                alpha_mode: alpha_mode(look),
                cull_mode: None,
                double_sided: true,
                unlit: true,
                ..default()
            },
            extension: ParticleShader {
                params: ParticleParams {
                    color: Vec4::from(look.color),
                    fog_color: Vec4::ZERO,
                    fog_range: Vec4::ZERO,
                    mode: Vec4::new(
                        output_mode(look),
                        test,
                        threshold,
                        crate::full_brightness_nits(),
                    ),
                    origin: origin.extend(0.0),
                },
                key: ParticleKey {
                    depth_test: look.depth_test,
                    depth_write: look.depth_write,
                },
                texture: look
                    .texture
                    .and_then(|i| textures.get(i).cloned().flatten()),
            },
        });
        let mesh = meshes.add(empty_mesh());
        let entity = commands
            .spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(origin),
                Visibility::Hidden,
                NoFrustumCulling,
                ChildOf(emitter),
            ))
            .id();
        let sorted = matches!(look.blend, Blend::Blend | Blend::MaskedBlend(_));
        systems.push(Some(Drawn {
            entity,
            mesh,
            material,
            look: look.clone(),
            sorted,
        }));
    }
    commands.entity(emitter).insert(ParticleEmitter {
        run,
        placed: data.placed,
        appeared: None,
        systems,
    });
    emitter
}

/// A mesh with no quads yet.
fn empty_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<[f32; 2]>::new());
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, Vec::<[f32; 4]>::new());
    mesh.insert_indices(Indices::U32(Vec::new()));
    mesh
}

/// A direction from Bevy's space to the game's.
fn game_direction(d: Vec3) -> [f32; 3] {
    [d.x, -d.z, d.y]
}

/// Every frame: each placed object's systems moved on, their quads built
/// facing the camera, the fog and emittance colour of the moment.
#[allow(clippy::too_many_arguments)]
fn run_particles(
    time: Res<Time>,
    game: Res<crate::GameFiles>,
    state: Res<DialogueState>,
    place: Res<crate::viewmodel::PlaceLighting>,
    cameras: Query<&Transform, With<FlyCamera>>,
    mut emitters: Query<&mut ParticleEmitter>,
    mut drawn: Query<&mut Visibility>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    let basis = CameraBasis {
        right: game_direction(camera.right().into()),
        up: game_direction(camera.up().into()),
        direction: game_direction(camera.forward().into()),
    };
    let now = time.elapsed_secs();
    let lighting = place.get();
    // Colours that follow a region's weather (`emittance`), for the
    // regions named, once a frame.
    let order = &game.0.order;
    let links: Vec<cellview::EmittanceLink> = emitters
        .iter()
        .flat_map(|e| e.systems.iter().flatten().filter_map(|d| d.look.emittance))
        .collect();
    let emittance = (!links.is_empty()).then(|| {
        let hour = state
            .0
            .global(order, "GameHour")
            .unwrap_or(world::weather::DEFAULT_HOUR);
        let regions: Vec<esm::FormId> = links.iter().filter_map(|l| l.region).collect();
        world::weather::EmittanceNow::new(order, &state.0.weather, hour, regions)
    });
    for mut emitter in &mut emitters {
        let emitter = &mut *emitter;
        let appeared = *emitter.appeared.get_or_insert(now);
        let seconds = now - appeared;
        emitter.run.update(seconds, &emitter.placed, [0.0; 3]);
        for (system, drawn_system) in emitter.run.systems.iter().zip(&emitter.systems) {
            let Some(d) = drawn_system else {
                continue;
            };
            let corners = system.quads(&basis, d.sorted);
            if let Ok(mut visibility) = drawn.get_mut(d.entity) {
                *visibility = if corners.is_empty() {
                    Visibility::Hidden
                } else {
                    Visibility::Inherited
                };
            }
            // The fog and colour of the moment, written only when they
            // change (a written material is prepared again).
            let Some(current) = materials.get(&d.material).map(|m| m.extension.params) else {
                continue;
            };
            let mut wanted = current;
            if let Some(l) = &lighting {
                wanted.fog_color = l.fog_color;
                wanted.fog_range = l.fog_range;
            }
            if let (Some(link), Some(colors)) = (d.look.emittance, emittance.as_ref()) {
                let [r, g, b] = link.glow(colors);
                (wanted.color.x, wanted.color.y, wanted.color.z) = (r, g, b);
            }
            if wanted != current {
                if let Some(m) = materials.get_mut(&d.material) {
                    m.extension.params = wanted;
                }
            }
            if corners.is_empty() {
                continue;
            }
            let origin = current.origin.truncate();
            let (positions, colors, uvs) = corners_for_drawing(&corners, &d.look.atlas);
            let positions: Vec<[f32; 3]> = positions
                .iter()
                .map(|&p| (Vec3::from(space::point(p)) - origin).into())
                .collect();
            if let Some(mesh) = meshes.get_mut(&d.mesh) {
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
                mesh.insert_indices(Indices::U32(quad_indices(corners.len() / 4)));
            }
        }
    }
}
