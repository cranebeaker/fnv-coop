//! Water, drawn with the game's water shaders (`water.wgsl`; the records
//! and rules are `world::water`, the surfaces `cellview::water`).
//!
//! The game's water pixel shader mixes four inputs besides its own values;
//! here each is made the closest way Bevy allows:
//!
//! - **Refraction**: the game copies the frame just before drawing water.
//!   The water's material reads Bevy's own copy of the frame taken before
//!   its transmissive pass (`view_transmission_texture`), so water is drawn
//!   in that pass (opaque and alpha-tested things are in the copy; blended
//!   ones and the sky, drawn later here, are not).
//! - **Depth**: the game draws what's under the water into a 512 × 512
//!   map of path length and depth. Here the scene's depth after the opaque
//!   pass is copied (a render-graph step between the opaque and
//!   transmissive passes) and the shader works out the same two values from
//!   it per pixel.
//! - **Reflection**: the game renders the scene mirrored in the water's
//!   plane into a 1024 × 1024 target (`iWaterReflectWidth`/`Height`, 4×
//!   multisampled), then blurs it (`bUseWaterReflectionBlur`, radius
//!   `iWaterBlurAmount` + 1 = 5; `water_blur.wgsl`). Here a second camera
//!   does: placed at the mirror image of the eye with a mirrored transform,
//!   the water plane as its near plane (an oblique projection: nothing
//!   under the water shows), its picture flipped left to right so
//!   triangles keep their winding (the shader samples it flipped back).
//!   What it draws follows the game: outdoor water not at the "sea level"
//!   reflects only the sky (`004eaf80`), "sea level" water and water inside
//!   the whole scene (`004eaa00`, `004e9d40`). One plane is mirrored per
//!   frame: the nearest reflecting water's (the game does up to two
//!   outdoors and four inside, one per height, and distant water samples
//!   the sea level's). Not done: its silhouette and low-detail modes (off
//!   in this install), the player's own body in it.
//! - **Ripples**: worked out in the shader (see `water.wgsl`).
//!
//! Distant water (`DistantWater`): the water shapes of the distant-land
//! chunks around the player (`cellview::water::lod_water`), drawn with
//! `WATER033` and blended, hidden over loaded squares.
//!
//! The sun (outdoors) follows the clock: its direction is the sun disc's
//! (`world::weather::sun_at`; the game normalizes the sun node's position,
//! read as the disc's: inferred), its colour the weather's Sunlight colour
//! of the hour (not the sunlight dimmer), its visibility the disc's × 100,
//! at most 1 (`004e1bc0`; that the value is the disc's fade is inferred).
//!
//! Not done: underwater (the game's `WATER016`, drawn when the eye is below
//! the surface; the surface isn't drawn from below here), water without
//! refraction (`WATER002`, `003`, `006`, `007` and their inside versions:
//! no water in New Vegas lacks it; drawn with refraction), the lit-water
//! pass for point lights (`WATER036`), wading ripples (`WATER017`–`032`).

// The shader-layout derive generates checking functions the compiler
// reports as unused.
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use bevy::asset::{load_internal_asset, weak_handle, RenderAssetUsages};
use bevy::core_pipeline::core_3d::graph::{Core3d, Node3d};
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::ecs::query::QueryItem;
use bevy::ecs::system::SystemParam;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Vec3A;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::camera::{CameraProjection, Exposure, RenderTarget, SubCameraView};
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::mesh::{Indices, MeshVertexBufferLayoutRef, PrimitiveTopology};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{
    NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
};
use bevy::render::render_resource::binding_types::{sampler, texture_2d};
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::texture::GpuImage;
use bevy::render::view::{RenderLayers, ViewDepthTexture, ViewTarget};
use bevy::render::RenderApp;
use bevy::transform::TransformSystem;
use cellview::{space, ViewerScene, WaterData};
use world::water::placed_flags;

use crate::grade::ImageSpaceGrade;

const SHADER: Handle<Shader> = weak_handle!("2f6b8d14-9c3e-4a75-b1e0-7d4a5c9e3f62");
const BLUR_SHADER: Handle<Shader> = weak_handle!("8a3c5e71-4d2b-4f96-a0c8-1e7b9d6f2a43");

/// The render layer water is drawn on (the main camera sees it; the
/// reflection camera doesn't).
pub const WATER_LAYER: usize = 22;
/// The render layer the sky is also on, for sky-only reflections.
pub const SKY_LAYER: usize = 21;

/// How strongly the reflection's near plane leans (see
/// [`mirror_projection`]): small enough that nothing on screen above the
/// water is cut off at the far end.
const OBLIQUE_SCALE: f32 = 0.5;

/// The water shader's constants, as `water.wgsl` reads them.
#[derive(Clone, Copy, Debug, PartialEq, ShaderType)]
pub struct WaterParams {
    pub shallow: Vec4,
    pub deep: Vec4,
    pub reflection: Vec4,
    pub fresnel_ri: Vec4,
    pub var_amounts: Vec4,
    pub fog_param: Vec4,
    pub fog_color: Vec4,
    pub depth_falloff: Vec4,
    pub sun_dir: Vec4,
    pub sun_color: Vec4,
    pub noise: Vec4,
    pub layer_motion_a: Vec4,
    pub layer_motion_b: Vec4,
    pub amplitudes: Vec4,
    pub uv_scales: Vec4,
    pub axes: Vec4,
    pub surface: Vec4,
}

/// Which of the game's water shaders a surface uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct WaterKey {
    /// `REFLECTIONS` (`WATER000`, `008`) or not (`001`, `009`).
    pub reflections: bool,
    /// `INTERIORWATER` (`008`–`015`): no sun.
    pub interior: bool,
    /// `DEPTH`.
    pub depth: bool,
    /// `LOD` (`WATER033`): distant water, blended.
    pub lod: bool,
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(WaterKey)]
pub struct WaterMaterial {
    #[uniform(0)]
    pub params: WaterParams,
    /// The ripples' noise, sampled as stored, repeating.
    #[texture(1)]
    #[sampler(2)]
    pub noise: Option<Handle<Image>>,
    /// The reflection camera's picture.
    #[texture(3)]
    #[sampler(4)]
    pub reflection: Option<Handle<Image>>,
    /// The scene's depth after its opaque pass.
    #[texture(5, sample_type = "depth", multisampled = true)]
    pub depth: Option<Handle<Image>>,
    pub key: WaterKey,
}

impl From<&WaterMaterial> for WaterKey {
    fn from(m: &WaterMaterial) -> Self {
        m.key
    }
}

impl Material for WaterMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    /// Drawn after the opaque pass, with its copy of the frame: the
    /// refraction.
    fn reads_view_transmission_texture(&self) -> bool {
        true
    }

    /// Distant water is blended by its opacity (`WATER033`'s alpha; the
    /// game's water passes blend source alpha over the rest, `00b83dd0`);
    /// the rest writes alpha 1.
    fn alpha_mode(&self) -> AlphaMode {
        if self.key.lod {
            AlphaMode::Blend
        } else {
            AlphaMode::Opaque
        }
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ])?];
        let k = key.bind_group_data;
        let mut defs = Vec::new();
        if k.reflections {
            defs.push("REFLECTIONS".into());
        }
        if k.interior {
            defs.push("INTERIOR".into());
        }
        if k.depth {
            defs.push("DEPTH".into());
        }
        if k.lod {
            defs.push("LOD".into());
        }
        descriptor.vertex.shader_defs.extend(defs.iter().cloned());
        if let Some(fragment) = descriptor.fragment.as_mut() {
            fragment.shader_defs.extend(defs);
        }
        Ok(())
    }
}

/// A water surface on screen.
#[derive(Component, Clone)]
pub struct WaterSurface {
    /// Game units.
    pub height: f32,
    /// The plane its reflection is mirrored in: its own height, except
    /// distant water's, mirrored in the worldspace's distant-water height
    /// (`NAM4`: the plane of the distant water's group, `004e4c80`).
    pub mirror: f32,
    /// Its extent seen from above (game x, y).
    pub lo: Vec2,
    pub hi: Vec2,
    pub reflections: bool,
    pub reflects_scene: bool,
    pub interior: bool,
    /// Distant water (mirrored only when there's no nearer water).
    pub lod: bool,
    pub material: Handle<WaterMaterial>,
}

/// The pictures water reads besides its textures.
#[derive(Resource)]
pub struct WaterTargets {
    /// The scene's depth, copied after the opaque pass: its size and
    /// multisampling follow the main camera's.
    pub depth: Handle<Image>,
    pub depth_size: (UVec2, u32),
    /// The reflection camera's picture.
    pub reflection: Handle<Image>,
}

/// On the main camera: where its depth is copied to for the water.
#[derive(Component, Clone, ExtractComponent)]
pub struct WaterDepthTarget(pub Handle<Image>);

/// The camera drawing the reflection.
#[derive(Component)]
pub struct ReflectionCamera;

/// What spawning water needs besides the commands and meshes.
#[derive(SystemParam)]
pub struct WaterSpawn<'w> {
    materials: ResMut<'w, Assets<WaterMaterial>>,
    targets: Option<Res<'w, WaterTargets>>,
}

/// The reflection camera's projection (see [`mirror_projection`]); it
/// keeps the main camera's shape whatever its picture's size.
#[derive(Debug, Clone)]
pub struct MirrorProjection {
    pub clip_from_view: Mat4,
    pub far: f32,
    pub base: PerspectiveProjection,
}

impl CameraProjection for MirrorProjection {
    fn get_clip_from_view(&self) -> Mat4 {
        self.clip_from_view
    }

    fn get_clip_from_view_for_sub(&self, _sub_view: &SubCameraView) -> Mat4 {
        self.clip_from_view
    }

    fn update(&mut self, _width: f32, _height: f32) {}

    fn far(&self) -> f32 {
        self.far
    }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        self.base.get_frustum_corners(z_near, z_far)
    }
}

/// The reflection camera's projection: the main camera's (Bevy's infinite
/// reverse-Z perspective), with its near plane replaced by `plane` (in the
/// reflection camera's view space, `plane · v ≥ 0` kept: the water, so
/// nothing under it shows; the "oblique near plane" construction) and left
/// and right swapped (the camera's transform is a mirror image; swapping
/// keeps triangles' winding, and the shader samples the picture flipped
/// back).
pub fn mirror_projection(base: Mat4, plane: Vec4) -> Mat4 {
    let row = |r: usize| {
        Vec4::new(
            base.x_axis[r],
            base.y_axis[r],
            base.z_axis[r],
            base.w_axis[r],
        )
    };
    let len = plane.truncate().length().max(1e-6);
    let plane = plane / len;
    // Depth = (w − a · plane·v) / w: 1 on the water, less above it.
    let depth = row(3) - OBLIQUE_SCALE * plane;
    Mat4::from_cols(-row(0), row(1), depth, row(3)).transpose()
}

/// The mirror image in the horizontal plane at Bevy height `h` (meters).
pub fn mirror_in(h: f32) -> Mat4 {
    Mat4::from_cols(
        Vec4::X,
        Vec4::NEG_Y,
        Vec4::Z,
        Vec4::new(0.0, 2.0 * h, 0.0, 1.0),
    )
}

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER, "water.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, BLUR_SHADER, "water_blur.wgsl", Shader::from_wgsl);
        app.add_plugins((
            MaterialPlugin::<WaterMaterial> {
                prepass_enabled: false,
                shadows_enabled: false,
                ..default()
            },
            ExtractComponentPlugin::<WaterDepthTarget>::default(),
            ExtractComponentPlugin::<ReflectionBlur>::default(),
        ))
        .init_resource::<DistantWater>()
        .add_systems(Startup, setup_water)
        .add_systems(
            Update,
            (
                prepare_main_camera,
                tag_sky,
                stream_distant_water,
                light_water,
            )
                .chain()
                // Not in the frame's stages: the GPU's water around the camera
                // (`crate::frame_order::ViewerSet::AfterFrame`).
                .in_set(crate::frame_order::ViewerSet::AfterFrame),
        )
        .add_systems(
            PostUpdate,
            place_reflection_camera.before(TransformSystem::TransformPropagate),
        );
        let blur = app
            .world()
            .get_resource::<crate::GameFiles>()
            .and_then(|g| g.0.water_settings().blur);
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        if let Some(radius) = blur {
            render_app.insert_resource(BlurRadius(radius));
        }
        render_app
            .add_render_graph_node::<ViewNodeRunner<DepthCopyNode>>(Core3d, WaterDepthLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::MainOpaquePass,
                    WaterDepthLabel,
                    Node3d::MainTransmissivePass,
                ),
            )
            .add_render_graph_node::<ViewNodeRunner<BlurNode>>(Core3d, WaterBlurLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    WaterBlurLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        if render_app.world().contains_resource::<BlurRadius>() {
            render_app.init_resource::<BlurPipeline>();
        }
    }
}

/// The scene's depth copied for the water, after the opaque pass and
/// before the transmissive one (where water is drawn).
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct WaterDepthLabel;

#[derive(Default)]
struct DepthCopyNode;

impl ViewNode for DepthCopyNode {
    type ViewQuery = (&'static ViewDepthTexture, &'static WaterDepthTarget);

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (depth, target): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let images = world.resource::<RenderAssets<GpuImage>>();
        let Some(image) = images.get(&target.0) else {
            return Ok(());
        };
        let (from, to) = (&depth.texture, &image.texture);
        if from.size() != to.size()
            || from.sample_count() != to.sample_count()
            || from.format() != to.format()
        {
            return Ok(());
        }
        render_context.command_encoder().copy_texture_to_texture(
            from.as_image_copy(),
            to.as_image_copy(),
            from.size(),
        );
        Ok(())
    }
}

/// On the reflection camera: its picture is blurred (see `water_blur.wgsl`).
#[derive(Component, Clone, Copy, ExtractComponent)]
pub struct ReflectionBlur;

/// The reflection's blur, after the reflection camera's main pass.
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct WaterBlurLabel;

#[derive(Default)]
struct BlurNode;

impl ViewNode for BlurNode {
    type ViewQuery = (&'static ViewTarget, &'static ReflectionBlur);

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let Some(blur) = world.get_resource::<BlurPipeline>() else {
            return Ok(());
        };
        let cache = world.resource::<PipelineCache>();
        let (Some(horizontal), Some(vertical)) = (
            cache.get_render_pipeline(blur.horizontal),
            cache.get_render_pipeline(blur.vertical),
        ) else {
            return Ok(());
        };
        for pipeline in [horizontal, vertical] {
            let post = view_target.post_process_write();
            let bind_group = render_context.render_device().create_bind_group(
                "water_blur_bind_group",
                &blur.layout,
                &BindGroupEntries::sequential((post.source, &blur.sampler)),
            );
            let mut pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some("water_blur_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: post.destination,
                    resolve_target: None,
                    ops: Operations::default(),
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_render_pipeline(pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        Ok(())
    }
}

/// The blur's two passes, for the radius the settings give.
#[derive(Resource)]
struct BlurPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    horizontal: CachedRenderPipelineId,
    vertical: CachedRenderPipelineId,
}

/// The reflection blur's radius (texels), from the settings.
#[derive(Resource, Clone, Copy)]
struct BlurRadius(u32);

impl FromWorld for BlurPipeline {
    fn from_world(world: &mut World) -> Self {
        let radius = world.get_resource::<BlurRadius>().map_or(1, |r| r.0);
        let device = world.resource::<RenderDevice>();
        let layout = device.create_bind_group_layout(
            "water_blur_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );
        // Bilinear and clamped.
        let sampler = device.create_sampler(&SamplerDescriptor {
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..default()
        });
        let mut queue = |entry: &'static str| {
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some(format!("water_blur_{entry}").into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader: BLUR_SHADER,
                        shader_defs: vec![ShaderDefVal::UInt("BLUR_RADIUS".into(), radius)],
                        entry_point: entry.into(),
                        targets: vec![Some(ColorTargetState {
                            format: ViewTarget::TEXTURE_FORMAT_HDR,
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                    zero_initialize_workgroup_memory: false,
                })
        };
        let horizontal = queue("horizontal");
        let vertical = queue("vertical");
        Self {
            layout,
            sampler,
            horizontal,
            vertical,
        }
    }
}

/// A depth picture the size of the main camera's.
fn depth_image(size: UVec2, samples: u32) -> Image {
    Image {
        data: None,
        texture_descriptor: TextureDescriptor {
            label: Some("water_depth"),
            size: Extent3d {
                width: size.x.max(1),
                height: size.y.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: samples,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            // (Multisampled textures must be render attachments too.)
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::Default,
        texture_view_descriptor: None,
        asset_usage: RenderAssetUsages::RENDER_WORLD,
    }
}

/// The reflection's target and camera, and a first depth target.
fn setup_water(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    game: Res<crate::GameFiles>,
) {
    let settings = game.0.water_settings();
    let (w, h) = settings.reflection_size;
    let reflection = images.add(Image {
        data: None,
        texture_descriptor: TextureDescriptor {
            label: Some("water_reflection"),
            size: Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST,
            view_formats: &[],
        },
        // Clamped and bilinear, as the game samples it (`00be0cf0`).
        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::ClampToEdge,
            address_mode_v: ImageAddressMode::ClampToEdge,
            mag_filter: ImageFilterMode::Linear,
            min_filter: ImageFilterMode::Linear,
            ..default()
        }),
        texture_view_descriptor: None,
        // Kept in the main world too: a camera's target size is read there.
        asset_usage: RenderAssetUsages::default(),
    });
    let samples = 4;
    let depth = images.add(depth_image(UVec2::ONE, samples));
    commands.insert_resource(WaterTargets {
        depth,
        depth_size: (UVec2::ONE, samples),
        reflection: reflection.clone(),
    });
    let msaa = match settings.multisamples {
        1 => Msaa::Off,
        2 => Msaa::Sample2,
        8 => Msaa::Sample8,
        _ => Msaa::Sample4,
    };
    let mut camera = commands.spawn((
        Camera3d {
            screen_space_specular_transmission_steps: 0,
            ..default()
        },
        Camera {
            target: RenderTarget::from(reflection),
            // Before the main camera.
            order: -1,
            hdr: true,
            ..default()
        },
        Projection::custom(MirrorProjection {
            clip_from_view: Mat4::IDENTITY,
            far: 5000.0,
            base: PerspectiveProjection::default(),
        }),
        // Stored values, as the scene's: no tone mapping or dithering.
        Tonemapping::None,
        // No light clusters: nothing here is lit by Bevy's lights.
        bevy::pbr::ClusterConfig::None,
        DebandDither::Disabled,
        msaa,
        Exposure::default(),
        Transform::default(),
        RenderLayers::none(),
        ReflectionCamera,
    ));
    if settings.blur.is_some() {
        camera.insert(ReflectionBlur);
    }
}

/// The main camera draws the water's layer and keeps its depth copyable;
/// the depth target follows its size.
#[allow(clippy::type_complexity)]
fn prepare_main_camera(
    mut commands: Commands,
    mut cameras: Query<
        (
            Entity,
            &Camera,
            &mut Camera3d,
            Option<&RenderLayers>,
            &Msaa,
            Option<&WaterDepthTarget>,
        ),
        (
            With<ImageSpaceGrade>,
            Without<ReflectionCamera>,
            Without<crate::viewmodel::FirstPersonCamera>,
        ),
    >,
    targets: Option<ResMut<WaterTargets>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
) {
    let Some(mut targets) = targets else {
        return;
    };
    for (entity, camera, mut camera_3d, layers, msaa, target) in &mut cameras {
        if layers.is_none() {
            commands
                .entity(entity)
                .insert(RenderLayers::from_layers(&[0, WATER_LAYER]));
        }
        let usages = TextureUsages::from_bits_truncate(camera_3d.depth_texture_usages.0);
        if !usages.contains(TextureUsages::COPY_SRC) {
            camera_3d.depth_texture_usages = (usages | TextureUsages::COPY_SRC).into();
        }
        let Some(size) = camera.physical_target_size() else {
            continue;
        };
        let wanted = (size, msaa.samples().max(1));
        if targets.depth_size != wanted {
            if wanted.1 == 1 {
                println!("  water: the depth needs multisampling; water reads none");
            }
            let handle = images.add(depth_image(size, wanted.1));
            targets.depth = handle.clone();
            targets.depth_size = wanted;
            for (_, m) in materials.iter_mut() {
                m.depth = Some(handle.clone());
            }
            commands.entity(entity).insert(WaterDepthTarget(handle));
        } else if target.is_none() {
            commands
                .entity(entity)
                .insert(WaterDepthTarget(targets.depth.clone()));
        }
    }
}

/// The sky is on the sky layer too, for sky-only reflections.
fn tag_sky(
    mut commands: Commands,
    skies: Query<Entity, (With<crate::SkyEntity>, Without<RenderLayers>)>,
) {
    for entity in &skies {
        commands
            .entity(entity)
            .insert(RenderLayers::from_layers(&[0, SKY_LAYER]));
    }
}

/// The water constants for a surface, before the hour's sun and fog
/// outdoors (`00b864d0`; see `water.wgsl`). `None` without the type's
/// visual data.
pub fn water_params(
    data: &WaterData,
    fog: Option<&cellview::Fog>,
    brightness: f32,
) -> Option<WaterParams> {
    let mut params = type_params(&data.water_type, data.flags, fog, brightness)?;
    params.axes = Vec4::new(
        data.axes[0][0],
        data.axes[0][1],
        data.axes[1][0],
        data.axes[1][1],
    );
    params.surface.x = data.height;
    Some(params)
}

/// The constants a water type and its flags give (`water_params` without
/// a surface's own place and axes).
fn type_params(
    water_type: &world::water::WaterType,
    flags: u32,
    fog: Option<&cellview::Fog>,
    brightness: f32,
) -> Option<WaterParams> {
    let v = water_type.visual?;
    let rgb = |c: [u8; 4]| {
        Vec4::new(
            f32::from(c[0]) / 255.0,
            f32::from(c[1]) / 255.0,
            f32::from(c[2]) / 255.0,
            1.0,
        )
    };
    let motion = |i: usize| {
        let (sin, cos) = v.wind_directions[i].to_radians().sin_cos();
        Vec2::new(sin, cos) * v.wind_speeds[i]
    };
    let (m1, m2, m3) = (motion(0), motion(1), motion(2));
    // "No underwater fog" placed water gets no fog amount above water
    // either (`004e3590`).
    let fog_amount = if flags & placed_flags::NO_UNDERWATER_FOG != 0 {
        0.0
    } else {
        v.fog_amount
    };
    let mut params = WaterParams {
        shallow: rgb(v.shallow),
        deep: rgb(v.deep),
        reflection: rgb(v.reflection),
        fresnel_ri: Vec4::new(v.fresnel, 1.0, v.shininess, v.reflection_multiplier()),
        var_amounts: Vec4::new(
            v.sun_power,
            v.reflectivity,
            f32::from(water_type.opacity) / 100.0,
            v.distortion,
        ),
        fog_param: Vec4::new(1.0e9, 1.0, v.fog_far, v.fog_far - v.fog_near),
        fog_color: Vec4::new(0.0, 0.0, 0.0, fog_amount),
        depth_falloff: Vec4::new(v.falloff_constant()[0], v.falloff_constant()[1], 0.0, 0.0),
        sun_dir: Vec4::new(0.0, 0.0, 1.0, 0.0),
        sun_color: Vec4::ZERO,
        noise: Vec4::new(
            v.noise_scale,
            v.noise_tile_size.max(1e-3),
            if flags & placed_flags::OBJECT_UVS != 0 {
                1.0
            } else {
                0.0
            },
            0.0,
        ),
        layer_motion_a: Vec4::new(m1.x, m1.y, m2.x, m2.y),
        layer_motion_b: Vec4::new(m3.x, m3.y, 0.0, 0.0),
        amplitudes: Vec4::new(v.amplitudes[0], v.amplitudes[1], v.amplitudes[2], 0.0),
        uv_scales: Vec4::new(
            v.noise_uv_scale(0),
            v.noise_uv_scale(1),
            v.noise_uv_scale(2),
            0.0,
        ),
        axes: Vec4::new(1.0, 0.0, 0.0, 1.0),
        surface: Vec4::new(0.0, brightness, 0.0, 0.0),
    };
    if let Some(f) = fog {
        set_fog(&mut params, f);
    }
    Some(params)
}

/// The scene's fog in the water constants: `FogParam.xy` = (far, far −
/// near), `FresnelRI.y` = its power, `FogColor.rgb` its colour.
fn set_fog(params: &mut WaterParams, fog: &cellview::Fog) {
    params.fog_param.x = fog.far;
    params.fog_param.y = (fog.far - fog.near).max(1e-3);
    params.fresnel_ri.y = fog.power;
    params.fog_color = Vec4::new(fog.color[0], fog.color[1], fog.color[2], params.fog_color.w);
}

/// A surface as a Bevy mesh (Bevy's space), drawn with no transform: its
/// positions (game units, world space), texture coordinates (zero when
/// none) and triangles.
fn water_mesh(positions: &[[f32; 3]], uvs: &[[f32; 2]], indices: &[u16]) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    let count = positions.len();
    let uvs: Vec<[f32; 2]> = (0..count)
        .map(|i| uvs.get(i).copied().unwrap_or([0.0, 0.0]))
        .collect();
    let positions: Vec<[f32; 3]> = positions.iter().map(|&p| space::point(p)).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; count]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U16(indices.to_vec()));
    mesh
}

/// Puts a place's water on screen; returns the entities.
pub(crate) fn spawn_water(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    water: &mut WaterSpawn,
    scene: &ViewerScene,
    textures: &[Option<Handle<Image>>],
    brightness: f32,
) -> Vec<Entity> {
    let mut out = Vec::new();
    let Some(targets) = water.targets.as_ref() else {
        return out;
    };
    for data in &scene.water {
        let Some(params) = water_params(data, scene.fog.as_ref(), brightness) else {
            println!("  water: {} has no visual data; not drawn", data.name);
            continue;
        };
        let material = water.materials.add(WaterMaterial {
            params,
            noise: data.noise.and_then(|i| textures.get(i).cloned().flatten()),
            reflection: Some(targets.reflection.clone()),
            depth: Some(targets.depth.clone()),
            key: WaterKey {
                reflections: data.reflections,
                interior: data.interior,
                depth: data.depth,
                lod: false,
            },
        });
        let (mut lo, mut hi) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
        for p in &data.positions {
            lo = lo.min(Vec2::new(p[0], p[1]));
            hi = hi.max(Vec2::new(p[0], p[1]));
        }
        out.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(water_mesh(&data.positions, &data.uvs, &data.indices))),
                    MeshMaterial3d(material.clone()),
                    Transform::IDENTITY,
                    RenderLayers::layer(WATER_LAYER),
                    WaterSurface {
                        height: data.height,
                        mirror: data.height,
                        lo,
                        hi,
                        reflections: data.reflections,
                        reflects_scene: data.reflects_scene,
                        interior: data.interior,
                        lod: false,
                        material,
                    },
                    crate::SceneEntity,
                ))
                .id(),
        );
    }
    out
}

/// Outdoors, the sun and the fog of the hour, as `daylight` gives the rest
/// of the place (a game minute at a time, and for new water at once).
#[allow(clippy::too_many_arguments)]
fn light_water(
    game: Res<crate::GameFiles>,
    state: Res<crate::dialogue::DialogueState>,
    exterior: Option<Res<crate::exterior::Exterior>>,
    surfaces: Query<&WaterSurface>,
    new: Query<(), Added<WaterSurface>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut applied: Local<Option<f32>>,
    mut weather: Local<Option<(esm::FormId, world::weather::Weather)>>,
) {
    let Some(exterior) = exterior else {
        *applied = None;
        return;
    };
    let order = &game.0.order;
    let Some(id) = exterior.weather else {
        return;
    };
    if weather.as_ref().map(|w| w.0) != Some(id) {
        *weather = world::weather::Weather::load(order, id).map(|w| (id, w));
        *applied = None;
    }
    let Some((_, weather)) = weather.as_ref() else {
        return;
    };
    let hour = state
        .0
        .global(order, "GameHour")
        .unwrap_or(world::weather::DEFAULT_HOUR);
    let moved = applied.is_none_or(|h| (h - hour).abs() >= 1.0 / 60.0);
    if !moved && new.is_empty() {
        return;
    }
    *applied = Some(hour);
    let clock = world::weather::SkyClock::new(
        exterior.grid.climate.as_ref(),
        world::weather::SkySettings::load(order),
    );
    let light = world::weather::WeatherMix::single(weather).light_at(&clock, hour);
    let (_, _, fog) = cellview::exterior_light(&light);
    let sun = world::weather::sun_at(&clock, hour);
    let toward = Vec3::from(sun.disc).normalize_or(Vec3::Z);
    let color = weather
        .color_at(world::weather::SkyColor::Sunlight, &clock, hour)
        .map(|c| c / 255.0);
    for surface in &surfaces {
        if surface.interior {
            continue;
        }
        let Some(m) = materials.get_mut(&surface.material) else {
            continue;
        };
        let p = &mut m.params;
        if let Some(f) = &fog {
            set_fog(p, f);
        }
        p.sun_dir = toward.extend((100.0 * sun.visibility).clamp(0.0, 1.0));
        p.sun_color = Vec4::new(color[0], color[1], color[2], 1.0);
    }
}

/// Each frame: the reflection camera mirrors the main camera in the
/// nearest reflecting water below the eye, drawing the sky alone or the
/// whole scene as that water asks; with none, it rests.
#[allow(clippy::type_complexity)]
fn place_reflection_camera(
    main: Query<
        (&Transform, &Projection, &Exposure),
        (
            With<ImageSpaceGrade>,
            Without<ReflectionCamera>,
            Without<crate::viewmodel::FirstPersonCamera>,
        ),
    >,
    surfaces: Query<(&WaterSurface, &Visibility)>,
    mut reflection: Query<
        (
            &mut Transform,
            &mut Projection,
            &mut RenderLayers,
            &mut Exposure,
        ),
        With<ReflectionCamera>,
    >,
) {
    let Ok((mut transform, mut projection, mut layers, mut exposure)) = reflection.single_mut()
    else {
        return;
    };
    let Ok((eye, main_projection, main_exposure)) = main.single() else {
        return;
    };
    let eye_game = Vec3::from(game_point(eye.translation));
    // The nearest reflecting water below the eye (flat distance to its
    // extent), the higher one when level; distant water only when there's
    // no other, and only where it's shown.
    let mut best: Option<(bool, f32, f32, &WaterSurface)> = None;
    for (s, visibility) in &surfaces {
        let above = eye_game.z - s.mirror;
        if !s.reflections || above <= 1.0 || *visibility == Visibility::Hidden {
            continue;
        }
        let gap = (s.lo - eye_game.truncate())
            .max(eye_game.truncate() - s.hi)
            .max(Vec2::ZERO)
            .length();
        let better = match best {
            None => true,
            Some((lod, g, a, _)) => {
                (lod && !s.lod) || (lod == s.lod && (gap < g || (gap == g && above < a)))
            }
        };
        if better {
            best = Some((s.lod, gap, above, s));
        }
    }
    // With no water to mirror it draws nothing (kept running: a camera
    // switched on mid-run misses its first frame's view data).
    let Some((_, _, _, surface)) = best else {
        if *layers != RenderLayers::none() {
            *layers = RenderLayers::none();
        }
        return;
    };
    // Only water on screen shows the reflection: with none of it in the
    // view this frame (as the view is now, after this frame's turn), the
    // reflection isn't drawn. Looking down, the mirror camera sits below
    // the ground looking up through everything above the water's plane;
    // drawing that for no water was most of the frame.
    if !water_in_view(eye, main_projection, &surfaces) {
        if *layers != RenderLayers::none() {
            *layers = RenderLayers::none();
        }
        return;
    }
    let Projection::Perspective(main_perspective) = main_projection else {
        return;
    };
    let h = surface.mirror * space::METERS_PER_UNIT;
    let view = eye.compute_matrix();
    *transform = Transform::from_matrix(mirror_in(h) * view);
    let plane = view.transpose() * Vec4::new(0.0, -1.0, 0.0, h);
    let base = main_perspective.get_clip_from_view();
    *projection = Projection::custom(MirrorProjection {
        clip_from_view: mirror_projection(base, plane),
        far: main_perspective.far,
        base: main_perspective.clone(),
    });
    let wanted = if surface.reflects_scene {
        RenderLayers::from_layers(&[0, SKY_LAYER])
    } else {
        RenderLayers::layer(SKY_LAYER)
    };
    if *layers != wanted {
        *layers = wanted;
    }
    if exposure.ev100 != main_exposure.ev100 {
        exposure.ev100 = main_exposure.ev100;
    }
}

/// Whether any water surface shown is in the view from `eye`: its extent at
/// its height, as a box, against the view's frustum.
fn water_in_view(
    eye: &Transform,
    projection: &Projection,
    surfaces: &Query<(&WaterSurface, &Visibility)>,
) -> bool {
    use bevy::render::camera::CameraProjection;
    use bevy::render::primitives::{Aabb, Frustum};
    let clip_from_world = projection.get_clip_from_view() * eye.compute_matrix().inverse();
    let frustum = Frustum::from_clip_from_world(&clip_from_world);
    surfaces.iter().any(|(s, visibility)| {
        if *visibility == Visibility::Hidden {
            return false;
        }
        let corners = [
            [s.lo.x, s.lo.y],
            [s.hi.x, s.lo.y],
            [s.lo.x, s.hi.y],
            [s.hi.x, s.hi.y],
        ]
        .map(|[x, y]| Vec3::from(space::point([x, y, s.height])));
        let lo = corners.iter().fold(Vec3::INFINITY, |a, c| a.min(*c)) - Vec3::splat(0.5);
        let hi = corners.iter().fold(Vec3::NEG_INFINITY, |a, c| a.max(*c)) + Vec3::splat(0.5);
        frustum.intersects_obb(
            &Aabb::from_min_max(lo, hi),
            &bevy::math::Affine3A::IDENTITY,
            true,
            true,
        )
    })
}

/// Distant water: the pieces of the distant-land chunks around the player
/// (as far as distant land is drawn), loaded in the background, those over
/// loaded squares hidden (their own water is drawn there).
#[derive(Resource, Default)]
pub struct DistantWater {
    /// The worldspace (editor ID) they're for.
    world: Option<String>,
    chunks: HashMap<(i32, i32), DistantChunk>,
    channel: Option<(Sender<FinishedWater>, Mutex<Receiver<FinishedWater>>)>,
    /// The material all pieces share, and the loaded squares last applied.
    material: Option<Handle<WaterMaterial>>,
    shown_for: Option<HashSet<(i32, i32)>>,
    /// The worldspace's distant-water height (`NAM4`).
    mirror: Option<f32>,
}

enum DistantChunk {
    Loading,
    Loaded(Vec<(Entity, (i32, i32))>),
}

type FinishedWater = ((i32, i32), Vec<cellview::water::LodWaterPiece>);

/// Distant-water chunks loading at once.
const WATER_CHUNKS_LOADING_AT_ONCE: usize = 4;

/// How many cells a chunk is from a square (0 when the square is in it).
fn chunk_gap(chunk: (i32, i32), here: (i32, i32)) -> i32 {
    let n = cellview::LOD_CHUNK_CELLS;
    let gap = |lo: i32, at: i32| (lo - at).max(at - (lo + n - 1)).max(0);
    gap(chunk.0, here.0).max(gap(chunk.1, here.1))
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn stream_distant_water(
    mut commands: Commands,
    game: Res<crate::GameFiles>,
    exterior: Option<Res<crate::exterior::Exterior>>,
    cameras: Query<
        &Transform,
        (
            With<ImageSpaceGrade>,
            Without<ReflectionCamera>,
            Without<crate::viewmodel::FirstPersonCamera>,
        ),
    >,
    mut state: ResMut<DistantWater>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut water: WaterSpawn,
    mut visibilities: Query<&mut Visibility>,
) {
    let state = &mut *state;
    let Some(exterior) = exterior else {
        // Gone with the place (its entities are the place's).
        if state.world.is_some() {
            *state = DistantWater::default();
        }
        return;
    };
    let Some(world) = exterior.grid.world.editor_id.clone() else {
        return;
    };
    if state.world.as_deref() != Some(world.as_str()) {
        for chunk in state.chunks.values() {
            if let DistantChunk::Loaded(pieces) = chunk {
                for (entity, _) in pieces {
                    commands.entity(*entity).try_despawn();
                }
            }
        }
        *state = DistantWater::default();
        state.world = Some(world.clone());
        let (sender, receiver) = channel();
        state.channel = Some((sender, Mutex::new(receiver)));
        let order = &game.0.order;
        let brightness = crate::full_brightness_nits();
        state.mirror = world::water::WorldWater::load(order, exterior.grid.world.form_id)
            .and_then(|w| w.lod_height);
        state.material = cellview::water::lod_water_type(order, exterior.grid.world.form_id)
            .and_then(|t| {
                let flags = placed_flags::REFLECTS;
                let params = type_params(&t, flags, None, brightness)?;
                let targets = water.targets.as_ref()?;
                Some(water.materials.add(WaterMaterial {
                    params,
                    noise: None,
                    reflection: Some(targets.reflection.clone()),
                    depth: Some(targets.depth.clone()),
                    key: WaterKey {
                        reflections: true,
                        lod: true,
                        ..default()
                    },
                }))
            });
    }
    let Some(material) = state.material.clone() else {
        return;
    };
    let Ok(camera) = cameras.single() else {
        return;
    };
    let here = exterior.here(camera);
    let loaded = exterior.loaded_squares();

    // Finished chunks onto the screen.
    let finished: Vec<FinishedWater> = state
        .channel
        .as_ref()
        .and_then(|(_, r)| r.lock().ok().map(|r| r.try_iter().collect()))
        .unwrap_or_default();
    let mut added = false;
    for (cell, pieces) in finished {
        if chunk_gap(cell, here) > crate::exterior::DISTANT_CELLS {
            state.chunks.remove(&cell);
            continue;
        }
        let mut spawned = Vec::new();
        for piece in pieces {
            let (mut lo, mut hi) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
            for p in &piece.positions {
                lo = lo.min(Vec2::new(p[0], p[1]));
                hi = hi.max(Vec2::new(p[0], p[1]));
            }
            let entity = commands
                .spawn((
                    Mesh3d(meshes.add(water_mesh(&piece.positions, &[], &piece.indices))),
                    MeshMaterial3d(material.clone()),
                    Transform::IDENTITY,
                    RenderLayers::layer(WATER_LAYER),
                    WaterSurface {
                        height: piece.height,
                        mirror: state.mirror.unwrap_or(piece.height),
                        lo,
                        hi,
                        reflections: true,
                        // Its group is at the "sea level" (`NAM4`): the
                        // whole scene, mirrored there.
                        reflects_scene: state.mirror.is_some(),
                        interior: false,
                        lod: true,
                        material: material.clone(),
                    },
                    if loaded.contains(&piece.cell) {
                        Visibility::Hidden
                    } else {
                        Visibility::Inherited
                    },
                    crate::SceneEntity,
                ))
                .id();
            spawned.push((entity, piece.cell));
        }
        state.chunks.insert(cell, DistantChunk::Loaded(spawned));
        added = true;
    }

    // Far chunks off the screen.
    let far: Vec<(i32, i32)> = state
        .chunks
        .iter()
        .filter(|(c, s)| {
            !matches!(s, DistantChunk::Loading)
                && chunk_gap(**c, here) > crate::exterior::DISTANT_CELLS
        })
        .map(|(c, _)| *c)
        .collect();
    for cell in far {
        if let Some(DistantChunk::Loaded(pieces)) = state.chunks.remove(&cell) {
            for (entity, _) in pieces {
                commands.entity(entity).try_despawn();
            }
        }
    }

    // Pieces over loaded squares hidden.
    if added || state.shown_for.as_ref() != Some(&loaded) {
        for chunk in state.chunks.values() {
            if let DistantChunk::Loaded(pieces) = chunk {
                for (entity, cell) in pieces {
                    if let Ok(mut v) = visibilities.get_mut(*entity) {
                        let wanted = if loaded.contains(cell) {
                            Visibility::Hidden
                        } else {
                            Visibility::Inherited
                        };
                        if *v != wanted {
                            *v = wanted;
                        }
                    }
                }
            }
        }
        state.shown_for = Some(loaded);
    }

    // Missing chunks, nearest first.
    let loading = state
        .chunks
        .values()
        .filter(|c| matches!(c, DistantChunk::Loading))
        .count();
    let step = cellview::LOD_CHUNK_CELLS;
    let reach = crate::exterior::DISTANT_CELLS;
    let lo = |c: i32| (c - reach).div_euclid(step) * step;
    let hi = |c: i32| (c + reach).div_euclid(step) * step;
    let mut wanted = Vec::new();
    let mut x = lo(here.0);
    while x <= hi(here.0) {
        let mut y = lo(here.1);
        while y <= hi(here.1) {
            if !state.chunks.contains_key(&(x, y)) && chunk_gap((x, y), here) <= reach {
                wanted.push((x, y));
            }
            y += step;
        }
        x += step;
    }
    wanted.sort_by_key(|c| chunk_gap(*c, here));
    let Some((sender, _)) = state.channel.as_ref() else {
        return;
    };
    let sender = sender.clone();
    for cell in wanted
        .into_iter()
        .take(WATER_CHUNKS_LOADING_AT_ONCE.saturating_sub(loading))
    {
        state.chunks.insert(cell, DistantChunk::Loading);
        let game = Arc::clone(&game.0);
        let sender = sender.clone();
        let world = world.clone();
        std::thread::spawn(move || {
            let _ = sender.send((cell, cellview::water::lod_water(&game.assets, &world, cell)));
        });
    }
}

/// A point in Bevy's space back in game units.
fn game_point(p: Vec3) -> [f32; 3] {
    let s = space::METERS_PER_UNIT;
    [p.x / s, -p.z / s, p.y / s]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project(m: Mat4, v: Vec3) -> Vec4 {
        m * v.extend(1.0)
    }

    #[test]
    fn the_reflection_keeps_what_is_above_the_water_and_mirrors_left_and_right() {
        let base = Mat4::perspective_infinite_reverse_rh(1.0, 16.0 / 9.0, 0.05);
        // The reflection camera is the eye's mirror image, under the water:
        // in its view space the water is the plane y = 2, kept above it.
        let plane = Vec4::new(0.0, 1.0, 0.0, -2.0);
        let m = mirror_projection(base, plane);
        let inside = |c: Vec4| c.z >= 0.0 && c.z <= c.w;
        // A point above the water ahead: kept, x mirrored.
        let above = Vec3::new(1.0, 3.0, -10.0);
        let c = project(m, above);
        let b = project(base, above);
        assert!(inside(c), "{c:?}");
        assert!((c.x + b.x).abs() < 1e-5 && (c.y - b.y).abs() < 1e-5);
        // Under the water: cut away.
        assert!(!inside(project(m, Vec3::new(0.0, 1.0, -10.0))));
        // On the water: right at the near plane.
        let on = project(m, Vec3::new(0.0, 2.0, -10.0));
        assert!((on.z / on.w - 1.0).abs() < 1e-5);
        // Along a ray from the eye, nearer points stay in front (reverse Z:
        // larger depth).
        let near = project(m, Vec3::new(0.0, 3.0, -5.0));
        let far = project(m, Vec3::new(0.0, 30.0, -50.0));
        assert!(inside(near) && inside(far));
        assert!(near.z / near.w > far.z / far.w);
    }

    #[test]
    fn the_mirror_turns_heights_about_the_water() {
        let m = mirror_in(3.0);
        assert_eq!(
            m.transform_point3(Vec3::new(1.0, 5.0, 2.0)),
            Vec3::new(1.0, 1.0, 2.0)
        );
        assert!(m.determinant() < 0.0);
        // A mirrored camera transform comes back the same from Bevy's
        // scale-rotation-translation form.
        let camera = Transform::from_xyz(1.0, 10.0, 4.0)
            .looking_to(Vec3::new(0.3, -0.5, -1.0), Vec3::Y)
            .compute_matrix();
        let mirrored = m * camera;
        let back = Transform::from_matrix(mirrored).compute_matrix();
        assert!(back.abs_diff_eq(mirrored, 1e-4), "{back:?} {mirrored:?}");
    }
}
