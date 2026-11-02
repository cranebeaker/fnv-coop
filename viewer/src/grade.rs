//! The game's last steps on the finished picture: bloom, the HDR
//! brightness limit, and the cell's image space (saturation, a tint,
//! brightness and contrast), then the conversion to linear light for the
//! screen. Ported from the game's own passes, read from a recording of a
//! frame (apitrace) and the shaders in its package:
//!
//! 1. `ISHDRDOWN4`: the picture shrunk 4× in each direction, four times
//!    (1920×1080 → 480×270 → 120×67 → 30×16 → 7×4), each pixel the average
//!    of four bilinear samples one source texel out diagonally.
//! 2. `ISHDRDS4ADAPT`: the average brightness, a 1×1 value: the four corner
//!    pixels of the 7×4 picture averaged (the game samples outside the
//!    texture with clamping, so it reads the corners), eased toward from
//!    the last frame's average by `1 − speed^seconds` (eye adaptation: see
//!    [`adapt_eyes`]), its length clamped to [0.01, limit].
//! 3. `ISBPBLUR13`: at 480×270, a vertical 13-tap blur of
//!    `max(color − bright clamp, 0) × bright scale`; alpha holds the
//!    average's red + green + blue.
//! 4. `ISBLUR13`: the same blur horizontally, alpha carried along.
//! 5. `ISHDRBLENDINSHADERCIN`: `scene × L / max(a, L) + bloom × 0.5 /
//!    max(a, L)` (a: the bloom's alpha, L: the limit), then the cinematic
//!    step. The game then draws its HUD over the result (`hud`); its
//!    picture is laid over here, still on stored values.
//!
//! The blur weights are a Gaussian with σ = radius / 2 over taps −radius
//! … radius (radius 6 and 7 give exactly the recorded weights; the game has
//! blur shaders up to 15 taps, so larger radii are drawn as 7). All of it runs
//! on stored (gamma-encoded) values, as the game's frame buffer holds them.
//! The render-graph plumbing follows Bevy 0.16's `custom_post_processing`
//! example.

// The shader-layout derive generates checking functions the compiler
// reports as unused.
#![allow(dead_code)]

use bevy::asset::{load_internal_asset, weak_handle};
use bevy::core_pipeline::core_3d::graph::{Core3d, Node3d};
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::extract_component::{
    ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
    UniformComponentPlugin,
};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{
    NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
};
use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::sync_world::MainEntity;
use bevy::render::texture::{CachedTexture, FallbackImageZero, GpuImage, TextureCache};
use bevy::render::view::ViewTarget;
use bevy::render::{Render, RenderApp, RenderSet};
use std::collections::HashMap;

const SHADER: Handle<Shader> = weak_handle!("6d1f3a52-8c47-4e0b-9a61-2f5c7e9b1d34");

/// What a camera's final passes use, as the shader reads it. Every camera
/// needs one: the last pass also turns the picture's stored values into
/// linear light ([`ImageSpaceGrade::NEUTRAL`] when the cell has no image
/// space); cameras without one skip the passes.
#[derive(Component, Clone, Copy, Debug, PartialEq, ExtractComponent, ShaderType)]
pub struct ImageSpaceGrade {
    /// Tint color, and how much of it.
    pub tint: Vec4,
    /// Saturation, contrast, the brightness contrast spreads around, and
    /// brightness.
    pub cinematic: Vec4,
    /// Bloom: bright clamp (threshold), bright scale, blur radius in
    /// texels, and the clamp on the average brightness's length.
    pub bloom: Vec4,
    /// `x`: the final pass's brightness limit (`HDRParam.x`); `y`: 1 when
    /// the bloom and limit apply at all; `z`: the eye adaptation speed (the
    /// average pass's `HDRParam.z`); `w`: the frame's seconds as the
    /// average pass takes them (`TimingData.z`; below 0: take this frame's
    /// average as it is). See [`adapt_eyes`].
    pub hdr: Vec4,
    /// The colour the picture fades to, and how far (`Fade`, the final
    /// pass's last step: `lerp(c, Fade.rgb, Fade.w)`); set by image space
    /// modifiers.
    pub fade: Vec4,
    /// The image space modifiers' blur and the menus' still background
    /// (`world::menu_background`): `x` the blur pass's radius in texels (0:
    /// none), `y` how far its weights are toward that radius's row from the
    /// one below; `z` 1 while the background is held; `w` which capture it
    /// is (a new number captures again).
    pub background: Vec4,
}

/// A camera whose final passes are left to a later camera on the same
/// window: the main camera's, since the first-person view (`viewmodel`) is
/// drawn by its own camera after it, over a cleared depth buffer, and the
/// game's image space passes come after that pass. The marked camera keeps
/// its `ImageSpaceGrade` (everything that sets the grade sets it there);
/// the later camera's is copied from it each frame.
#[derive(Component, Clone, Copy, Default, ExtractComponent)]
pub struct GradeDeferred;

impl ImageSpaceGrade {
    /// Leaves the picture as it is: no cinematic change, no bloom.
    pub const NEUTRAL: Self = Self {
        tint: Vec4::new(1.0, 1.0, 1.0, 0.0),
        cinematic: Vec4::new(1.0, 1.0, 0.0, 1.0),
        bloom: Vec4::new(1.0, 0.0, 0.0, 1.0),
        hdr: Vec4::new(1.0, 0.0, 0.0, 0.0),
        fade: Vec4::ZERO,
        background: Vec4::ZERO,
    };

    /// The same with image space modifiers playing (`world::modifier`):
    /// each value the passes use becomes value × multiply + add, modifier
    /// after modifier; the tints are averaged weighted by their amounts
    /// (the game's code does this: `00b8d020`), the record's own tint among
    /// them, and the largest amount kept: confirmed by the Goodsprings
    /// recording, where `NVDefaultExterior`'s tint at 0.33 and
    /// `NVWastelandIS`'s at 0.392 arrived as their amount-weighted average
    /// at 0.392. Fades apply one after another.
    pub fn with_modifiers(self, values: &[world::modifier::ModifierValues]) -> Self {
        use world::modifier::track;
        if values.is_empty() {
            return self;
        }
        let apply = |base: f32, t: usize| {
            values
                .iter()
                .fold(base, |v, m| v * m.multiply[t] + m.add[t])
        };
        let mut out = self;
        out.bloom = Vec4::new(
            apply(self.bloom.x, track::BRIGHT_CLAMP),
            apply(self.bloom.y, track::BRIGHT_SCALE),
            apply(self.bloom.z, track::BLUR_RADIUS).clamp(0.0, 7.0),
            apply(self.bloom.w, track::UPPER_LUM_CLAMP),
        );
        out.hdr.x = apply(self.hdr.x, track::TARGET_LUM).max(1e-3);
        out.hdr.z = apply(self.hdr.z, track::EYE_ADAPT_SPEED);
        out.cinematic = Vec4::new(
            apply(self.cinematic.x, track::SATURATION),
            apply(self.cinematic.y, track::CONTRAST),
            apply(self.cinematic.z, track::CONTRAST_AVERAGE),
            apply(self.cinematic.w, track::BRIGHTNESS),
        );
        let tints: Vec<Vec4> = std::iter::once(self.tint)
            .chain(values.iter().map(|m| Vec4::from_array(m.tint)))
            .filter(|t| t.w > 0.0)
            .collect();
        let weight: f32 = tints.iter().map(|t| t.w).sum();
        if weight > 0.0 {
            let color = tints.iter().map(|t| t.truncate() * t.w).sum::<Vec3>() / weight;
            let amount = tints.iter().map(|t| t.w).fold(0.0, f32::max);
            out.tint = color.extend(amount);
        }
        // mix(mix(c, f1, a1), f2, a2) as one mix(c, F, A).
        let mut fade = Vec4::ZERO;
        for m in values {
            let f = Vec4::from_array(m.fade);
            let a = fade.w + f.w - fade.w * f.w;
            if a > 0.0 {
                let rgb = (fade.truncate() * fade.w * (1.0 - f.w) + f.truncate() * f.w) / a;
                fade = rgb.extend(a);
            }
        }
        out.fade = fade;
        // The blur: the largest of the modifiers' (`00b8ccb0`), drawn by
        // the blur effect's pass for it (`world::menu_background`).
        let blur = values.iter().map(|m| m.blur).fold(0.0, f32::max);
        let (radius, mix) = world::menu_background::blur_pass(blur).map_or((0.0, 0.0), |r| {
            let f = 1.0 - (r as f32 - blur);
            (r as f32, if f == 0.0 { 1.0 } else { f })
        });
        out.background.x = radius;
        out.background.y = mix;
        out
    }

    /// From the cell's image space. Without HDR values there's no bloom.
    pub fn from_cell(grade: Option<&cellview::Grade>, hdr: Option<&cellview::Hdr>) -> Self {
        let mut out = Self::NEUTRAL;
        if let Some(grade) = grade {
            out = out.with_cinematic(grade);
        }
        if let Some(h) = hdr {
            // Recorded: the bright pass gets (bright clamp, bright scale),
            // the blur reaches the blur radius, the average is clamped to
            // the upper luminance clamp and the final limit is the target
            // luminance (Doc Mitchell's house and the Mojave Outpost
            // barracks, whose values differ). The game's widest blur
            // shaders take 15 taps, so the radius stops at 7: the barracks'
            // 8 was drawn with 7.
            out.bloom = Vec4::new(
                h.bright_clamp,
                h.bright_scale,
                h.blur_radius.clamp(0.0, 7.0),
                h.upper_lum_clamp,
            );
            out.hdr = Vec4::new(h.target_lum.max(1e-3), 1.0, h.eye_adapt_speed, 0.0);
        }
        out
    }

    /// The same, with the cinematic values (saturation, tint, brightness,
    /// contrast) taken from `grade`.
    pub fn with_cinematic(self, grade: &cellview::Grade) -> Self {
        let [r, g, b] = grade.tint;
        Self {
            // As stored: the game's image space manager hands the record's
            // tint color and amount to the shader unchanged when no image
            // space modifier is active (read from FalloutNV.exe and the
            // recording, which sent (0.69, 0.561, 0.302, 0.5)).
            tint: Vec4::new(r, g, b, grade.tint_amount),
            cinematic: Vec4::new(
                grade.saturation,
                grade.contrast,
                grade.contrast_average,
                grade.brightness,
            ),
            ..self
        }
    }

    /// The same without the cinematic change (bloom and HDR kept), for
    /// comparing.
    pub fn without_cinematic(self) -> Self {
        Self {
            tint: Self::NEUTRAL.tint,
            cinematic: Self::NEUTRAL.cinematic,
            ..self
        }
    }
}

/// The frame's seconds for the eye adaptation, as the game's average pass
/// gets them (`TimingData.z`): the frame's game time, so 0 while a menu is
/// open. Recorded: 0.24 at Goodsprings (the recording ran at about four
/// frames a second), and 0 in every frame with the console open (both
/// indoor recordings, and a later Goodsprings frame with the console open
/// and the same `HDRParam.z` 0.9); the game's own code for it isn't traced.
/// `menu_open`: a menu or the dialogue menu is up (the viewer's menus stop
/// its clock the same way). `starting`: a place is loading or has just
/// been entered, so the average is taken as it is (negative): what the
/// game's average holds after a loading screen isn't known.
pub fn adaptation_seconds(delta: f32, menu_open: bool, starting: bool) -> f32 {
    if starting {
        -1.0
    } else if menu_open {
        0.0
    } else {
        delta
    }
}

/// How far the average brightness moves toward this frame's in one frame
/// (`ISHDRDS4ADAPT`: `lerp(previous, current, 1 − HDRParam.z ^
/// TimingData.z)`): `1 − speed^seconds`, 0 with no time passing, 1 when
/// starting over. With `NVDefaultExterior`'s speed 0.9, a tenth of the way
/// a second.
pub fn adaptation_step(speed: f32, seconds: f32) -> f32 {
    if seconds < 0.0 {
        1.0
    } else if seconds == 0.0 {
        0.0
    } else {
        1.0 - speed.max(0.0).powf(seconds)
    }
}

/// Gives every camera's final passes the frame's seconds for the eye
/// adaptation (after the image space and its modifiers are set for the
/// frame).
#[allow(clippy::too_many_arguments)]
pub fn adapt_eyes(
    time: Res<Time>,
    menus: Option<Res<crate::menus::Menus>>,
    conversation: Option<Res<crate::dialogue::Conversation>>,
    exterior: Option<Res<crate::exterior::Exterior>>,
    grading: Option<Res<crate::Grading>>,
    mut started: Local<bool>,
    mut cameras: Query<&mut ImageSpaceGrade>,
) {
    let menu_open = menus.is_some_and(|m| m.is_open())
        || conversation.is_some_and(|c| c.0.as_ref().is_some_and(|t| !t.is_line_only()));
    let starting =
        !*started || grading.is_some_and(|g| g.is_changed()) || exterior.is_some_and(|e| e.busy());
    *started = true;
    let seconds = adaptation_seconds(time.delta_secs(), menu_open, starting);
    for mut grade in &mut cameras {
        if grade.hdr.w != seconds {
            grade.hdr.w = seconds;
        }
    }
}

pub struct GradePlugin;

impl Plugin for GradePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER, "grade.wgsl", Shader::from_wgsl);
        app.add_plugins((
            ExtractComponentPlugin::<ImageSpaceGrade>::default(),
            ExtractComponentPlugin::<GradeDeferred>::default(),
            UniformComponentPlugin::<ImageSpaceGrade>::default(),
        ))
        // After the image space and its modifiers are set (`Update`).
        .add_systems(PostUpdate, adapt_eyes);
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .init_resource::<EyeAverages>()
            .init_resource::<HeldBackgrounds>()
            .add_systems(
                Render,
                prepare_bloom_textures.in_set(RenderSet::PrepareResources),
            )
            .add_render_graph_node::<ViewNodeRunner<GradeNode>>(Core3d, GradeLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    GradeLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<GradePipeline>();
    }
}

/// The intermediate pictures of the bloom chain, per camera.
#[derive(Component)]
struct BloomTextures {
    /// The four 4× downsamples, largest first.
    levels: [CachedTexture; 4],
    /// After the bright pass and vertical blur, then the horizontal blur.
    vertical: CachedTexture,
    horizontal: CachedTexture,
    /// The modifiers' blur, down then across, at the picture's size.
    blurred: [CachedTexture; 2],
}

/// The menus' still background per camera (`world::menu_background`): the
/// picture captured once (blurred by the modifier it was captured with)
/// and shown, unchanged, until the menus let it go.
#[derive(Resource, Default)]
struct HeldBackgrounds(HashMap<MainEntity, Held>);

struct Held {
    _texture: Texture,
    view: TextureView,
    size: Extent3d,
    /// The capture it holds (`ImageSpaceGrade::background.w`).
    generation: f32,
    /// Captured this frame.
    capture: bool,
}

/// The average brightness each camera keeps from frame to frame (the
/// game's `AvgLum`, which its average pass reads and rewrites): two 1×1
/// pictures taking turns, by the camera's entity.
#[derive(Resource, Default)]
struct EyeAverages(HashMap<MainEntity, Averages>);

struct Averages {
    _textures: [Texture; 2],
    views: [TextureView; 2],
    /// The one written this frame (the other holds the last frame's).
    written: usize,
}

fn prepare_bloom_textures(
    mut commands: Commands,
    mut cache: ResMut<TextureCache>,
    mut averages: ResMut<EyeAverages>,
    mut held: ResMut<HeldBackgrounds>,
    device: Res<RenderDevice>,
    views: Query<(Entity, &MainEntity, &ViewTarget, &ImageSpaceGrade)>,
) {
    for (_, main, target, grade) in &views {
        if grade.background.z < 0.5 {
            held.0.remove(main);
            continue;
        }
        let size = target.main_texture().size();
        let stale = held.0.get(main).is_none_or(|h| h.size != size);
        if stale {
            let texture = device.create_texture(&TextureDescriptor {
                label: Some("menu_background"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: ViewTarget::TEXTURE_FORMAT_HDR,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            let view = texture.create_view(&TextureViewDescriptor::default());
            held.0.insert(
                *main,
                Held {
                    _texture: texture,
                    view,
                    size,
                    generation: f32::NAN,
                    capture: false,
                },
            );
        }
        if let Some(h) = held.0.get_mut(main) {
            // A new capture (or a new picture size) draws it again; else it
            // stays as it was.
            h.capture = h.generation != grade.background.w;
            h.generation = grade.background.w;
        }
    }
    held.0
        .retain(|main, _| views.iter().any(|(_, m, _, _)| m == main));
    let mut seen = Vec::new();
    for (_, main, _, _) in &views {
        seen.push(*main);
        let a = averages.0.entry(*main).or_insert_with(|| {
            let textures = ["image_space_average_a", "image_space_average_b"].map(|label| {
                device.create_texture(&TextureDescriptor {
                    label: Some(label),
                    size: Extent3d {
                        width: 1,
                        height: 1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: ViewTarget::TEXTURE_FORMAT_HDR,
                    usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
            });
            let views = [0, 1].map(|i| textures[i].create_view(&TextureViewDescriptor::default()));
            Averages {
                _textures: textures,
                views,
                written: 0,
            }
        });
        a.written = 1 - a.written;
    }
    averages.0.retain(|main, _| seen.contains(main));
    for (entity, _, target, _) in &views {
        let size = target.main_texture().size();
        let mut texture = |width: u32, height: u32, label: &'static str| {
            cache.get(
                &device,
                TextureDescriptor {
                    label: Some(label),
                    size: Extent3d {
                        width: width.max(1),
                        height: height.max(1),
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: ViewTarget::TEXTURE_FORMAT_HDR,
                    usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
            )
        };
        let (mut w, mut h) = (size.width, size.height);
        let mut level = |label| {
            w = (w / 4).max(1);
            h = (h / 4).max(1);
            (w, h, label)
        };
        let sizes = [
            level("image_space_down_1"),
            level("image_space_down_2"),
            level("image_space_down_3"),
            level("image_space_down_4"),
        ];
        let levels = sizes.map(|(w, h, label)| texture(w, h, label));
        let (bw, bh) = (sizes[0].0, sizes[0].1);
        commands.entity(entity).insert(BloomTextures {
            levels,
            vertical: texture(bw, bh, "image_space_bloom_v"),
            horizontal: texture(bw, bh, "image_space_bloom_h"),
            blurred: [
                texture(size.width, size.height, "image_space_blur_down"),
                texture(size.width, size.height, "image_space_blur_across"),
            ],
        });
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct GradeLabel;

#[derive(Default)]
struct GradeNode;

impl ViewNode for GradeNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static MainEntity,
        &'static DynamicUniformIndex<ImageSpaceGrade>,
        &'static BloomTextures,
        Option<&'static GradeDeferred>,
        &'static ImageSpaceGrade,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, main, grade_index, textures, deferred, values): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        if deferred.is_some() {
            return Ok(());
        }
        let Some(averages) = world.resource::<EyeAverages>().0.get(main) else {
            return Ok(());
        };
        let grade_pipeline = world.resource::<GradePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let ids = &grade_pipeline.pipelines;
        let Some(pipelines) = [
            ids.down,
            ids.average,
            ids.bright,
            ids.blur,
            ids.last,
            ids.blur_down,
            ids.blur_across,
            ids.copy,
        ]
        .map(|id| pipeline_cache.get_render_pipeline(id))
        .into_iter()
        .collect::<Option<Vec<_>>>() else {
            return Ok(());
        };
        let uniforms = world.resource::<ComponentUniforms<ImageSpaceGrade>>();
        let Some(binding) = uniforms.uniforms().binding() else {
            return Ok(());
        };
        // The HUD's picture (`hud`), or nothing.
        let overlay = world
            .get_resource::<crate::hud::HudLayer>()
            .and_then(|layer| world.resource::<RenderAssets<GpuImage>>().get(&layer.0))
            .map_or(
                &world.resource::<FallbackImageZero>().texture_view,
                |image| &image.texture_view,
            );
        let pass = Pass {
            layout: &grade_pipeline.layout,
            sampler: &grade_pipeline.sampler,
            uniform: binding,
            offset: grade_index.index(),
            overlay,
        };
        let [down, average, bright, blur, last, blur_down, blur_across, copy] = pipelines[..]
        else {
            return Ok(());
        };

        // Bloom and the average brightness, from the finished scene.
        let scene = view_target.main_texture_view();
        let avg = &averages.views[averages.written];
        let previous = &averages.views[1 - averages.written];
        let mut source = scene;
        for level in &textures.levels {
            pass.draw(render_context, down, source, avg, &level.default_view);
            source = &level.default_view;
        }
        let vertical = &textures.vertical.default_view;
        let horizontal = &textures.horizontal.default_view;
        // The average pass eases from the last frame's average.
        pass.draw(render_context, average, source, previous, avg);
        pass.draw(
            render_context,
            bright,
            &textures.levels[0].default_view,
            avg,
            vertical,
        );
        pass.draw(render_context, blur, vertical, avg, horizontal);

        // The final pass reads the current picture and writes the adjusted
        // one; the view target swaps to it afterwards.
        let post_process = view_target.post_process_write();
        // The modifiers' blur (`world::menu_background`), down then across;
        // with the menus' background held, the picture captured once, then
        // shown as it is.
        let blurring = values.background.x >= 1.0;
        let [blur_a, blur_b] = &textures.blurred;
        let (blur_a, blur_b) = (&blur_a.default_view, &blur_b.default_view);
        let mut picture = post_process.source;
        if let Some(h) = world.resource::<HeldBackgrounds>().0.get(main) {
            if h.capture {
                if blurring {
                    pass.draw(render_context, blur_down, picture, avg, blur_a);
                    pass.draw(render_context, blur_across, blur_a, avg, &h.view);
                } else {
                    pass.draw(render_context, copy, picture, avg, &h.view);
                }
            }
            picture = &h.view;
        } else if blurring {
            pass.draw(render_context, blur_down, picture, avg, blur_a);
            pass.draw(render_context, blur_across, blur_a, avg, blur_b);
            picture = blur_b;
        }
        pass.draw(
            render_context,
            last,
            picture,
            horizontal,
            post_process.destination,
        );
        Ok(())
    }
}

/// What every pass binds besides its textures.
struct Pass<'a> {
    layout: &'a BindGroupLayout,
    sampler: &'a Sampler,
    uniform: BindingResource<'a>,
    offset: u32,
    /// The HUD's picture (read by the last pass only).
    overlay: &'a TextureView,
}

impl Pass<'_> {
    /// One full-screen pass reading `source` (and `extra`) into `target`.
    fn draw(
        &self,
        render_context: &mut RenderContext,
        pipeline: &RenderPipeline,
        source: &TextureView,
        extra: &TextureView,
        target: &TextureView,
    ) {
        let bind_group = render_context.render_device().create_bind_group(
            "image_space_bind_group",
            self.layout,
            &BindGroupEntries::sequential((
                source,
                self.sampler,
                self.uniform.clone(),
                extra,
                self.overlay,
            )),
        );
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("image_space_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[self.offset]);
        render_pass.draw(0..3, 0..1);
    }
}

struct Pipelines {
    down: CachedRenderPipelineId,
    average: CachedRenderPipelineId,
    bright: CachedRenderPipelineId,
    blur: CachedRenderPipelineId,
    last: CachedRenderPipelineId,
    blur_down: CachedRenderPipelineId,
    blur_across: CachedRenderPipelineId,
    copy: CachedRenderPipelineId,
}

#[derive(Resource)]
struct GradePipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipelines: Pipelines,
}

impl FromWorld for GradePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            "image_space_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<ImageSpaceGrade>(true),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                ),
            ),
        );
        // Bilinear and clamped, as the game samples these pictures.
        let sampler = render_device.create_sampler(&SamplerDescriptor {
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..default()
        });
        let mut queue = |entry: &'static str| {
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some(format!("image_space_{entry}").into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader: SHADER,
                        shader_defs: vec![],
                        entry_point: entry.into(),
                        // Every picture here is in the camera's HDR format.
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
        let pipelines = Pipelines {
            down: queue("downsample"),
            average: queue("average"),
            bright: queue("bright_blur"),
            blur: queue("blur"),
            last: queue("fragment"),
            blur_down: queue("modifier_blur_down"),
            blur_across: queue("modifier_blur_across"),
            copy: queue("copy"),
        };
        Self {
            layout,
            sampler,
            pipelines,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use world::modifier::{track, ModifierValues, TRACKS};

    fn values(fade: [f32; 4]) -> ModifierValues {
        ModifierValues {
            multiply: [1.0; TRACKS],
            add: [0.0; TRACKS],
            tint: [1.0, 1.0, 1.0, 0.0],
            fade,
            blur: 0.0,
            double_vision: 0.0,
        }
    }

    #[test]
    fn modifiers_scale_values_and_fades_stack() {
        let base = ImageSpaceGrade {
            bloom: Vec4::new(0.9, 2.4, 6.0, 1.0),
            ..ImageSpaceGrade::NEUTRAL
        };
        assert_eq!(base.with_modifiers(&[]), base);
        let mut flash = values([1.0, 1.0, 1.0, 0.5]);
        flash.multiply[track::BRIGHT_SCALE] = 2.0;
        flash.add[track::BRIGHT_SCALE] = 0.2;
        let dark = values([0.0, 0.0, 0.0, 0.5]);
        let out = base.with_modifiers(&[flash, dark]);
        assert!((out.bloom.y - 5.0).abs() < 1e-5);
        assert_eq!(out.bloom.x, 0.9);
        // Half to white then half to black: 3/4 faded, a third white.
        assert!((out.fade.w - 0.75).abs() < 1e-6);
        assert!((out.fade.x - 1.0 / 3.0).abs() < 1e-6);
        // No tint anywhere: unchanged.
        assert_eq!(out.tint, base.tint);
    }

    #[test]
    fn the_eye_adapts_a_tenth_of_the_way_a_second_outdoors() {
        // Recorded at Goodsprings: speed 0.9 (`NVDefaultExterior`'s eye
        // adapt speed, `HDRParam.z`), a 0.24 s frame (`TimingData.z`).
        let hdr = cellview::Hdr {
            eye_adapt_speed: 0.9,
            blur_radius: 7.0,
            emissive_mult: 1.5,
            target_lum: 1.4,
            upper_lum_clamp: 1.0,
            bright_scale: 2.0,
            bright_clamp: 0.6,
            skin_directional: 2.0,
        };
        let grade = ImageSpaceGrade::from_cell(None, Some(&hdr));
        assert_eq!(grade.hdr.z, 0.9);
        let step = adaptation_step(grade.hdr.z, 0.24);
        assert!((step - (1.0 - 0.9f32.powf(0.24))).abs() < 1e-6);
        assert!((step - 0.025_0).abs() < 1e-3, "{step}");
        // Over a second of frames: a tenth of the way.
        let mut left = 1.0f32;
        for _ in 0..60 {
            left *= 1.0 - adaptation_step(0.9, 1.0 / 60.0);
        }
        assert!((left - 0.9).abs() < 1e-4, "{left}");
        // With the console (any menu) open the game's frame time is 0: the
        // average stays (both indoor recordings, and Goodsprings with the
        // console open).
        assert_eq!(adaptation_seconds(0.24, true, false), 0.0);
        assert_eq!(adaptation_step(0.9, 0.0), 0.0);
        // While a place loads the viewer takes the average as it is.
        assert!(adaptation_seconds(0.24, false, true) < 0.0);
        assert_eq!(adaptation_step(0.9, -1.0), 1.0);
        assert_eq!(adaptation_seconds(0.24, false, false), 0.24);
    }

    #[test]
    fn modifiers_change_the_eye_adaptation_speed() {
        let base = ImageSpaceGrade {
            hdr: Vec4::new(1.0, 1.0, 0.9, 0.0),
            ..ImageSpaceGrade::NEUTRAL
        };
        let mut slow = values([0.0; 4]);
        slow.multiply[track::EYE_ADAPT_SPEED] = 0.5;
        assert_eq!(base.with_modifiers(&[slow]).hdr.z, 0.45);
    }

    #[test]
    fn the_weathers_modifier_gives_goodsprings_its_recorded_grade() {
        // `NVDefaultExterior`: saturation 1.1, contrast 1.1 around 0.2,
        // brightness 1, tint (0.984, 0.569, 0) at 0.33; the day's weather
        // modifier `NVWastelandIS` at its first keys: brightness × 1.3, tint
        // (1, 0.737, 0.051) at 0.392.
        let record = cellview::Grade {
            saturation: 1.1,
            contrast_average: 0.2,
            contrast: 1.1,
            brightness: 1.0,
            tint: [251.0 / 255.0, 145.0 / 255.0, 0.0],
            tint_amount: 0.33,
        };
        let base = ImageSpaceGrade::NEUTRAL.with_cinematic(&record);
        let mut day = values([0.0; 4]);
        day.multiply[track::BRIGHTNESS] = 1.3;
        day.tint = [1.0, 188.0 / 255.0, 13.0 / 255.0, 100.0 / 255.0];
        let out = base.with_modifiers(&[day]);
        // Recorded: `Cinematic` (1.1, 0.2, 1.1, 1.3) and `Tint` (0.99283,
        // 0.6602, 0.02768, 0.39216): the two tints averaged by their
        // amounts, the larger amount kept.
        assert!((out.cinematic.w - 1.3).abs() < 1e-5);
        assert!((out.cinematic.y - 1.1).abs() < 1e-5);
        let want = Vec4::new(0.99283, 0.6602, 0.02768, 0.39216);
        assert!(
            (out.tint - want).abs().max_element() < 1e-4,
            "{:?}",
            out.tint
        );
    }
}
