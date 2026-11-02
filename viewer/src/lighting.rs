//! New Vegas's lighting for lit surfaces, in place of Bevy's physically
//! based lighting: an extension of the standard material whose fragment
//! shader (`game_lit.wgsl`) adds up the cell's lights the way the game
//! does. The standard material still does everything else: textures,
//! vertex colors, transparency, culling. Follows Bevy 0.16's
//! `extended_material` example.

// The shader-layout derive generates checking functions the compiler
// reports as unused.
#![allow(dead_code)]

use bevy::asset::{load_internal_asset, weak_handle};
use bevy::pbr::{
    ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
};
use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef};
use bevy::render::render_resource::{
    AsBindGroup, CompareFunction, RenderPipelineDescriptor, ShaderRef, ShaderType,
    SpecializedMeshPipelineError, VertexFormat,
};
use bevy::render::render_resource::{
    AsBindGroupError, BindGroupLayout, BindGroupLayoutEntry, UnpreparedBindGroup,
};
use bevy::render::renderer::RenderDevice;

const SHADER: Handle<Shader> = weak_handle!("3b8e2f41-7c95-4d1a-b06e-9f4a2c7d5e18");

/// The three corners of the triangle a vertex belongs to, in the mesh's
/// space. The game works several things out at each vertex and blends
/// them across the triangle (each light's direction, the specular half
/// vector, fog, the fade by viewing angle); with the corners, the shader
/// works them out at the corners and blends them the same way. Meshes
/// carrying them are drawn unindexed, each triangle with its own three
/// vertices.
pub const ATTRIBUTE_CORNER_A: MeshVertexAttribute =
    MeshVertexAttribute::new("GameCornerA", 3_814_202_901, VertexFormat::Float32x3);
pub const ATTRIBUTE_CORNER_B: MeshVertexAttribute =
    MeshVertexAttribute::new("GameCornerB", 3_814_202_902, VertexFormat::Float32x3);
pub const ATTRIBUTE_CORNER_C: MeshVertexAttribute =
    MeshVertexAttribute::new("GameCornerC", 3_814_202_903, VertexFormat::Float32x3);

/// The most point lights a surface adds up (the shader's array size).
pub const MAX_LIGHTS: usize = 64;

/// A lit surface's material.
pub type GameLitMaterial = ExtendedMaterial<StandardMaterial, LitExtension>;

/// A point light, as the shader reads it.
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType, Reflect)]
pub struct GameLight {
    /// Position and radius, in meters in Bevy's space.
    pub position_radius: Vec4,
    /// Color times its brightness multiplier.
    pub color: Vec4,
}

/// Everything the shader adds up, in the game's gamma-encoded units.
#[derive(Clone, Copy, Debug, PartialEq, ShaderType, Reflect)]
pub struct GameLighting {
    pub ambient: Vec4,
    pub directional_color: Vec4,
    /// Toward the light.
    pub directional_direction: Vec4,
    /// The surface's own glow; `w` is 1 when the glow map masks it.
    pub emissive: Vec4,
    /// `x`: the luminance shown at full brightness at the starting
    /// exposure; `y`: how many of `lights` are used; `z`: 1 when the
    /// ambient, the sun and the fog are the hour's light outdoors, shared
    /// by every surface (`crate::shared_light`), not these fields.
    pub scale: Vec4,
    /// The fog's color as stored (0..1), and its power in `w`.
    pub fog_color: Vec4,
    /// Fog near and far distances and the game's near clip plane
    /// (`cellview::GAME_NEAR_CLIP`), in meters; `w` is 1 when there's fog.
    pub fog_range: Vec4,
    /// The specular pass: in `w`, the material's glossiness (the power).
    /// `xyz` holds the material's specular color, which the game doesn't
    /// use (the shader ignores it).
    pub specular: Vec4,
    /// `x`: 1 with a normal map; `y`: 1 when the surface gets specular;
    /// `z`: 1 for a no-lighting surface, whose color is `emissive.rgb`;
    /// `w`: for a no-lighting surface, how it fogs (0 toward the fog color,
    /// 1 to black for added effects, 2 to white for multiplied ones); for
    /// a lit one, 1 when it's alpha-blended.
    pub surface: Vec4,
    /// Fade by viewing angle (no-lighting surfaces): start and stop angle
    /// cosines, start and stop opacity.
    pub falloff: Vec4,
    /// The reflection: `x` its strength (0: none); `y` 1 with a mask
    /// texture; `z` 1 for window reflections; `w` the material's opacity.
    pub environment: Vec4,
    /// How the surface is drawn: `x` how much nearer a decal is drawn, in
    /// Bevy's depth (added to depth; 0 for everything else; see
    /// [`decal_depth_offset`]); `y` the alpha test's comparison (0 none, 1
    /// never, 2 less, 3 equal, 4 less or equal, 5 greater, 6 not equal, 7
    /// greater or equal, 8 always), `z` its threshold (0..1).
    pub draw: Vec4,
    pub lights: [GameLight; MAX_LIGHTS],
    /// People's surfaces (`preview::cell::Shading`): `x` 0 plain, 1 skin,
    /// 2 hair; `y` what skin's directional light is multiplied by (the
    /// image space's `world::Hdr::skin_directional`). After the lights, so
    /// the terrain shader's copy of the layout stays a prefix of it.
    pub actor: Vec4,
    /// Hair: the hair shader's `HairTint` (the NPC's hair colour, 0..1).
    pub hair_tint: Vec4,
}

/// The decal pull ([`GameLighting::draw`]`.x`) in Bevy's depth for a
/// camera whose near plane is `near_meters`. The game adds
/// `preview::cell::decal_depth_bias` to its own depth, `z = A − B / d` with
/// `B` its projection's near × far / (far − near) (`depth_scale`, game
/// units), which draws a decal at distance d as if at d' with
/// `1 / d' = 1 / d − bias / B`. Bevy's depth (reverse, infinite far plane)
/// is `near / d`, so the same pull is the constant `near × (−bias) / B`
/// (near in game units) added to it.
pub fn decal_depth_offset(near_meters: f32, depth_scale: f32) -> f32 {
    let near_units = near_meters / cellview::space::METERS_PER_UNIT;
    near_units * -cellview::decal_depth_bias() / depth_scale
}

/// The alpha test's comparison as the shader takes it
/// ([`GameLighting::draw`]`.y`).
pub fn alpha_test_code(test: Option<(cellview::AlphaTest, f32)>) -> (f32, f32) {
    use cellview::AlphaTest::*;
    match test {
        None => (0.0, 0.0),
        Some((func, threshold)) => (
            match func {
                Never => 1.0,
                Less => 2.0,
                Equal => 3.0,
                LessEqual => 4.0,
                Greater => 5.0,
                NotEqual => 6.0,
                GreaterEqual => 7.0,
                Always => 8.0,
            },
            threshold,
        ),
    }
}

/// What picks a surface's pipeline besides the standard material's own
/// settings: for a model's piece (`game`), the game's depth test and
/// write per mesh (blended surfaces that write depth, like the ceiling
/// fan, hide what's drawn after them) and whether it's a decal (pulled
/// nearer in the shader rather than by Bevy's depth bias, which then only
/// orders it). Other surfaces (`game` false: the sky) keep Bevy's choices.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct DrawKey {
    pub game: bool,
    pub depth_test: bool,
    pub depth_write: bool,
    pub decal: bool,
}

impl From<&GameLit> for DrawKey {
    fn from(material: &GameLit) -> Self {
        material.key
    }
}

impl GameLighting {
    /// `actor.x` for a surface's shading.
    pub fn shading_code(shading: preview::cell::Shading) -> f32 {
        match shading {
            preview::cell::Shading::Plain => 0.0,
            preview::cell::Shading::Skin => 1.0,
            preview::cell::Shading::Hair => 2.0,
        }
    }
}

/// Bindless where the device allows (one bind group for many materials, so
/// their draws go together): the lighting in one array (binding 101), the
/// textures in Bevy's bindless arrays, found through the index table
/// (binding 100, entries 50 to 59); otherwise each its own binding 50 to
/// 59. (The standard material's own bindless entries are 0 to 30.)
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[bind_group_data(DrawKey)]
#[data(50, GameLighting, binding_array(101))]
#[bindless(limit(32), index_table(range(50..60), binding(100)))]
pub struct GameLit {
    pub lighting: GameLighting,
    /// Depth test and write, decal (see [`DrawKey`]).
    pub key: DrawKey,
    #[texture(51)]
    #[sampler(52)]
    pub glow: Option<Handle<Image>>,
    /// Sampled as stored (a linear texture); alpha is the specular mask.
    #[texture(53)]
    #[sampler(54)]
    pub normal_map: Option<Handle<Image>>,
    /// The reflection's cube map, sampled as stored.
    #[texture(55, dimension = "cube")]
    #[sampler(56)]
    pub environment: Option<Handle<Image>>,
    /// Where the reflection shows (red channel), sampled as stored.
    #[texture(57)]
    #[sampler(58)]
    pub environment_mask: Option<Handle<Image>>,
    /// The hour's light outdoors (`crate::shared_light::TEXTURE`, read
    /// texel by texel), read when `lighting.scale.z` is 1.
    #[texture(59, sample_type = "float", filterable = false)]
    pub shared: Handle<Image>,
}

impl From<&GameLit> for GameLighting {
    fn from(m: &GameLit) -> Self {
        m.lighting
    }
}

/// Whether the lit surfaces' materials are bindless: the device takes
/// bindless materials (the standard material's and [`GameLit`]'s own
/// checks) and filters 32-bit float textures (the shared light's texture,
/// in Bevy's bindless texture array). Set once the device is known
/// ([`GameLightingPlugin::finish`]), before the material's pipeline is made.
static BINDLESS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn bindless() -> bool {
    BINDLESS.load(std::sync::atomic::Ordering::Relaxed)
}

/// [`GameLit`] as the lit material's extension, bindless only where
/// [`BINDLESS`] says. Bevy 0.16's `ExtendedMaterial` asks the device
/// whether bindless works when it lays out its parts' bind groups but not
/// when it makes its pipeline, so a device without it (DX12 here) would
/// get bindless pipelines over plain bind groups; answering "no slots"
/// there keeps both plain.
#[derive(Asset, Reflect, Debug, Clone)]
pub struct LitExtension(pub GameLit);

impl From<GameLit> for LitExtension {
    fn from(lit: GameLit) -> Self {
        Self(lit)
    }
}

impl std::ops::Deref for LitExtension {
    type Target = GameLit;
    fn deref(&self) -> &GameLit {
        &self.0
    }
}

impl std::ops::DerefMut for LitExtension {
    fn deref_mut(&mut self) -> &mut GameLit {
        &mut self.0
    }
}

impl AsBindGroup for LitExtension {
    type Data = <GameLit as AsBindGroup>::Data;
    type Param = <GameLit as AsBindGroup>::Param;

    fn bindless_slot_count() -> Option<bevy::render::render_resource::BindlessSlabResourceLimit> {
        GameLit::bindless_slot_count().filter(|_| bindless())
    }

    fn bindless_supported(render_device: &RenderDevice) -> bool {
        bindless() && GameLit::bindless_supported(render_device)
    }

    fn label() -> Option<&'static str> {
        GameLit::label()
    }

    fn unprepared_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        param: &mut bevy::ecs::system::SystemParamItem<'_, '_, Self::Param>,
        force_no_bindless: bool,
    ) -> Result<UnpreparedBindGroup<Self::Data>, AsBindGroupError> {
        self.0.unprepared_bind_group(
            layout,
            render_device,
            param,
            force_no_bindless || !bindless(),
        )
    }

    fn bind_group_layout_entries(
        render_device: &RenderDevice,
        force_no_bindless: bool,
    ) -> Vec<BindGroupLayoutEntry> {
        GameLit::bind_group_layout_entries(render_device, force_no_bindless || !bindless())
    }

    fn bindless_descriptor() -> Option<bevy::render::render_resource::BindlessDescriptor> {
        GameLit::bindless_descriptor().filter(|_| bindless())
    }
}

impl MaterialExtension for LitExtension {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    /// The game's depth test and write for models' pieces ([`DrawKey`]);
    /// meshes with triangle corners get them as extra vertex inputs (and
    /// the `GAME_CORNERS` shader def); the rest of the layout is the one
    /// Bevy's own mesh pipeline builds.
    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let draw = key.bind_group_data;
        if draw.game {
            if let Some(depth) = descriptor.depth_stencil.as_mut() {
                depth.depth_write_enabled = draw.depth_write;
                if !draw.depth_test {
                    depth.depth_compare = CompareFunction::Always;
                }
                // Decals are pulled nearer in the vertex shader, as the
                // game's depth bias does; Bevy's bias only orders them.
                if draw.decal {
                    depth.bias.constant = 0;
                    depth.bias.slope_scale = 0.0;
                }
            }
        }
        if descriptor.vertex.shader != SHADER || !layout.0.contains(ATTRIBUTE_CORNER_A) {
            return Ok(());
        }
        let mut attributes = vec![Mesh::ATTRIBUTE_POSITION.at_shader_location(0)];
        for (attribute, location) in [
            (Mesh::ATTRIBUTE_NORMAL, 1),
            (Mesh::ATTRIBUTE_UV_0, 2),
            (Mesh::ATTRIBUTE_TANGENT, 4),
            (Mesh::ATTRIBUTE_COLOR, 5),
        ] {
            if layout.0.contains(attribute.id) {
                attributes.push(attribute.at_shader_location(location));
            }
        }
        attributes.extend([
            ATTRIBUTE_CORNER_A.at_shader_location(8),
            ATTRIBUTE_CORNER_B.at_shader_location(9),
            ATTRIBUTE_CORNER_C.at_shader_location(10),
        ]);
        descriptor.vertex.buffers = vec![layout.0.get_layout(&attributes)?];
        descriptor.vertex.shader_defs.push("GAME_CORNERS".into());
        if let Some(fragment) = descriptor.fragment.as_mut() {
            fragment.shader_defs.push("GAME_CORNERS".into());
        }
        Ok(())
    }
}

pub struct GameLightingPlugin;

impl Plugin for GameLightingPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER, "game_lit.wgsl", Shader::from_wgsl);
        app.add_plugins(MaterialPlugin::<GameLitMaterial>::default());
    }

    /// Before the material plugin's own (it was added after this): whether
    /// its materials are bindless ([`BINDLESS`]).
    fn finish(&self, app: &mut App) {
        let Some(render) = app.get_sub_app(bevy::render::RenderApp) else {
            return;
        };
        let Some(device) = render.world().get_resource::<RenderDevice>() else {
            return;
        };
        // `NVRS_NO_BINDLESS`: the plain way regardless (to compare).
        let on = std::env::var_os("NVRS_NO_BINDLESS").is_none()
            && GameLit::bindless_supported(device)
            && StandardMaterial::bindless_supported(device)
            && device
                .features()
                .contains(bevy::render::settings::WgpuFeatures::FLOAT32_FILTERABLE);
        BINDLESS.store(on, std::sync::atomic::Ordering::Relaxed);
        if !on {
            println!("  lit surfaces: the device takes no bindless materials; one bind group each");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decals_land_where_the_games_depth_bias_puts_them() {
        let near = 0.05;
        let m = cellview::space::METERS_PER_UNIT;
        let bias = cellview::decal_depth_bias();
        for (scale, d) in [
            (cellview::INTERIOR_DEPTH_SCALE, 200.0f32),
            (cellview::INTERIOR_DEPTH_SCALE, 3000.0),
            (cellview::EXTERIOR_DEPTH_SCALE, 1000.0),
        ] {
            // The game: depth A − B / d plus the bias, so 1/d' = 1/d − bias/B.
            let game = 1.0 / (1.0 / d - bias / scale);
            // Here: Bevy's depth near / d (meters) plus the offset.
            let depth = near / (d * m) + decal_depth_offset(near, scale);
            let here = near / depth / m;
            assert!((here - game).abs() < 1e-3 * d, "{d}: {here} vs {game}");
            assert!(game < d);
        }
        // 3000 units away indoors a decal comes 35.5 units nearer.
        let d = 3000.0;
        let game = 1.0 / (1.0 / d - bias / cellview::INTERIOR_DEPTH_SCALE);
        assert!((d - game - 35.5).abs() < 0.1, "{}", d - game);
    }

    #[test]
    fn alpha_tests_keep_the_games_comparison() {
        use cellview::AlphaTest;
        assert_eq!(alpha_test_code(None), (0.0, 0.0));
        assert_eq!(alpha_test_code(Some((AlphaTest::Greater, 0.5))), (5.0, 0.5));
        assert_eq!(alpha_test_code(Some((AlphaTest::Never, 0.0))).0, 1.0);
        assert_eq!(alpha_test_code(Some((AlphaTest::Always, 0.0))).0, 8.0);
    }
}
