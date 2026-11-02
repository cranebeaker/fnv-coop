//! Decoders for the block types needed to draw a mesh. Layouts follow the
//! community NIF format description for version 20.2.0.7 with Bethesda
//! version 34 (Fallout 3 / New Vegas).

use crate::error::Result;
use crate::math::{Transform, Vec3};
use crate::reader::Reader;

/// Layout details that vary between files of the same version. They are
/// detected per file by checking which variant consumes a block exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Layout {
    /// Scene object flags are 32-bit (Bethesda version > 26) or 16-bit.
    pub av_flags_u32: bool,
    /// Geometry data carries a 4-byte material CRC after its flags.
    pub material_crc: bool,
}

pub(crate) struct Ctx<'a> {
    pub strings: &'a [String],
    pub bs_version: u32,
    pub layout: Layout,
}

impl Ctx<'_> {
    fn string(&self, r: &mut Reader, what: &str) -> Result<String> {
        let index = r.i32(what)?;
        Ok(usize::try_from(index)
            .ok()
            .and_then(|i| self.strings.get(i))
            .cloned()
            .unwrap_or_default())
    }
}

/// Which decoder a block type name maps to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Node,
    Geometry,
    TriShapeData,
    TriStripsData,
    Shader(ShaderLayout),
    TextureSet,
    Texturing,
    SourceTexture,
    Material,
    Alpha,
    Stencil,
    ZBuffer,
    Other,
}

/// Where a shader property keeps its texture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShaderLayout {
    /// A reference to a texture set, after the lighting fields.
    TextureSet,
    /// A texture path after the lighting fields (texture clamp mode).
    FileAfterClamp,
    /// A texture path right after the base shader fields.
    FileAfterScale,
}

pub(crate) fn kind_of(type_name: &str) -> Kind {
    match type_name {
        "NiNode" | "BSFadeNode" | "BSMultiBoundNode" | "BSOrderedNode" | "BSValueNode"
        | "BSBlastNode" | "BSDamageStage" | "BSDebrisNode" | "NiBillboardNode" | "NiSwitchNode"
        | "NiLODNode" | "NiBone" | "BSTreeNode" | "RootCollisionNode" => Kind::Node,
        "NiTriShape" | "NiTriStrips" | "BSSegmentedTriShape" | "BSLODTriShape" => Kind::Geometry,
        "NiTriShapeData" => Kind::TriShapeData,
        "NiTriStripsData" => Kind::TriStripsData,
        "BSShaderPPLightingProperty" | "Lighting30ShaderProperty" => {
            Kind::Shader(ShaderLayout::TextureSet)
        }
        "BSShaderNoLightingProperty" | "SkyShaderProperty" | "TileShaderProperty" => {
            Kind::Shader(ShaderLayout::FileAfterClamp)
        }
        "TallGrassShaderProperty" => Kind::Shader(ShaderLayout::FileAfterScale),
        "BSShaderTextureSet" => Kind::TextureSet,
        "NiTexturingProperty" => Kind::Texturing,
        "NiSourceTexture" => Kind::SourceTexture,
        "NiMaterialProperty" => Kind::Material,
        "NiAlphaProperty" => Kind::Alpha,
        "NiStencilProperty" => Kind::Stencil,
        "NiZBufferProperty" => Kind::ZBuffer,
        _ => Kind::Other,
    }
}

/// Fields shared by every named object.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNet {
    pub name: String,
    pub extra_data: Vec<i32>,
    pub controller: i32,
}

/// Fields shared by every object placed in the scene.
#[derive(Debug, Clone, PartialEq)]
pub struct AvObject {
    pub net: ObjectNet,
    pub flags: u32,
    pub transform: Transform,
    pub properties: Vec<i32>,
    pub collision: i32,
}

impl AvObject {
    /// The "hidden" (app-culled) flag.
    pub fn is_hidden(&self) -> bool {
        self.flags & 1 != 0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub av: AvObject,
    pub children: Vec<i32>,
    pub effects: Vec<i32>,
    /// For switch and LOD nodes: the one child that is shown.
    pub active_child: Option<usize>,
    /// For `NiBillboardNode`: how it turns toward the camera (its
    /// billboard mode: 0 always face the camera, 1 rotate about its up
    /// axis, 2 rigid face the camera, 3 always face the centre, 4 rigid
    /// face the centre, 5 rotate about up, Bethesda's variant).
    pub billboard: Option<u16>,
}

/// A shape: a scene object that draws a piece of geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    pub av: AvObject,
    pub data: i32,
    pub skin: i32,
}

/// Vertex and triangle data; strips are converted to triangles.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GeometryData {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    /// The first texture coordinate set.
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    pub triangles: Vec<[u16; 3]>,
    pub center: Vec3,
    pub radius: f32,
    pub has_tangents: bool,
    /// Per-vertex tangent space for normal maps, stored after the normals
    /// when the geometry flags have bit 0x1000: the first array, then the
    /// second, in the file's order (`nvinspect nif` reports which one runs
    /// along the texture's U direction).
    pub tangents: Vec<Vec3>,
    pub bitangents: Vec<Vec3>,
}

/// A Bethesda shader property: `BSShaderPPLightingProperty` (lit, with a
/// texture set) or one of the shaders that name a single texture
/// (no-lighting, sky, tile and tall grass).
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderProperty {
    pub net: ObjectNet,
    /// The block type, e.g. `BSShaderPPLightingProperty`.
    pub type_name: String,
    /// Uses a texture set (lit shaders) rather than a single file.
    pub lit: bool,
    pub shade_flags: u16,
    pub shader_type: u32,
    pub shader_flags: u32,
    pub shader_flags2: u32,
    pub env_map_scale: f32,
    pub texture_clamp: u32,
    /// Lit shaders: reference to a [`TextureSet`].
    pub texture_set: i32,
    /// Other shaders: the single texture path.
    pub file_name: Option<String>,
    /// The bytes left after the fields read here matched what this shader
    /// type should have. False points at a misread layout.
    pub layout_ok: bool,
    /// No-lighting shaders (glows, light beams, haze): how opacity changes
    /// with the viewing angle.
    pub falloff: Option<Falloff>,
}

/// Opacity by viewing angle, for effect surfaces. The angles are stored as
/// cosines of the angle between the view and the surface normal: at
/// `start_angle` the opacity is `start_opacity`, at `stop_angle` it's
/// `stop_opacity`, blending in between.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Falloff {
    pub start_angle: f32,
    pub stop_angle: f32,
    pub start_opacity: f32,
    pub stop_opacity: f32,
}

/// The older material style (`NiTexturingProperty`): texture slots pointing
/// at [`SourceTexture`] blocks. Only the base (diffuse) slot is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TexturingProperty {
    pub flags: u16,
    /// Reference to the base texture's `NiSourceTexture`, or -1.
    pub base_texture: i32,
}

/// A texture file used by a [`TexturingProperty`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceTexture {
    /// Stored in a separate file (as opposed to embedded pixel data).
    pub external: bool,
    pub file_name: String,
}

/// Texture paths: diffuse, normal map, glow, height, environment, env mask.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextureSet {
    pub textures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialProperty {
    pub specular: Vec3,
    pub emissive: Vec3,
    pub glossiness: f32,
    pub alpha: f32,
    pub emissive_mult: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlphaProperty {
    pub flags: u16,
    pub threshold: u8,
}

impl AlphaProperty {
    pub fn blending(&self) -> bool {
        self.flags & 0x0001 != 0
    }

    /// Alpha testing (cut-out transparency, e.g. foliage and fences).
    pub fn testing(&self) -> bool {
        self.flags & 0x0200 != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StencilProperty {
    pub flags: u16,
}

impl StencilProperty {
    /// Draw mode "both": no back-face culling.
    pub fn double_sided(&self) -> bool {
        (self.flags >> 10) & 3 == 3
    }
}

/// `NiZBufferProperty`: whether a mesh is tested against and written into
/// the depth buffer. In files of this version its only field is the flags
/// word: bit 0 test, bit 1 write, bits 2–5 the comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZBufferProperty {
    pub flags: u16,
}

impl ZBufferProperty {
    /// Tested against the depth buffer.
    pub fn test(&self) -> bool {
        self.flags & 0x1 != 0
    }

    /// Writes its depth.
    pub fn write(&self) -> bool {
        self.flags & 0x2 != 0
    }
}

/// A decoded block.
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Node(Node),
    Geometry(Geometry),
    GeometryData(GeometryData),
    Shader(ShaderProperty),
    TextureSet(TextureSet),
    Texturing(TexturingProperty),
    SourceTexture(SourceTexture),
    Material(MaterialProperty),
    Alpha(AlphaProperty),
    Stencil(StencilProperty),
    ZBuffer(ZBufferProperty),
    /// A type this reader doesn't decode.
    Other,
}

pub(crate) fn read_object_net(r: &mut Reader, ctx: &Ctx) -> Result<ObjectNet> {
    Ok(ObjectNet {
        name: ctx.string(r, "a name")?,
        extra_data: r.ref_list("extra data")?,
        controller: r.i32("a controller reference")?,
    })
}

pub(crate) fn read_av_object(r: &mut Reader, ctx: &Ctx) -> Result<AvObject> {
    let net = read_object_net(r, ctx)?;
    let flags = if ctx.layout.av_flags_u32 {
        r.u32("object flags")?
    } else {
        u32::from(r.u16("object flags")?)
    };
    let translation = r.vec3("a translation")?;
    let rotation = r.mat3("a rotation")?;
    let scale = r.f32("a scale")?;
    let properties = if ctx.bs_version <= 34 {
        r.ref_list("properties")?
    } else {
        Vec::new()
    };
    let collision = r.i32("a collision reference")?;
    Ok(AvObject {
        net,
        flags,
        transform: Transform {
            rotation,
            translation,
            scale,
        },
        properties,
        collision,
    })
}

pub(crate) fn read_node(r: &mut Reader, ctx: &Ctx, type_name: &str) -> Result<Node> {
    let av = read_av_object(r, ctx)?;
    let children = r.ref_list("children")?;
    let effects = r.ref_list("effects")?;
    let active_child = match type_name {
        "NiSwitchNode" => {
            r.u16("switch flags")?;
            Some(r.u32("the active child index")? as usize)
        }
        // Child 0 is the most detailed level.
        "NiLODNode" => Some(0),
        _ => None,
    };
    let billboard = match type_name {
        "NiBillboardNode" => Some(r.u16("the billboard mode")?),
        _ => None,
    };
    Ok(Node {
        av,
        children,
        effects,
        active_child,
        billboard,
    })
}

pub(crate) fn read_geometry(r: &mut Reader, ctx: &Ctx) -> Result<Geometry> {
    let av = read_av_object(r, ctx)?;
    Ok(Geometry {
        av,
        data: r.i32("a data reference")?,
        skin: r.i32("a skin reference")?,
    })
}

pub(crate) fn read_geometry_data(r: &mut Reader, ctx: &Ctx, strips: bool) -> Result<GeometryData> {
    r.i32("a group ID")?;
    let n = usize::from(r.u16("the vertex count")?);
    r.u8("keep flags")?;
    r.u8("compress flags")?;
    let positions = if r.bool("the has-vertices flag")? {
        r.counted(n, 12, "vertices", |r| r.vec3("a vertex"))?
    } else {
        Vec::new()
    };
    let data_flags = r.u16("geometry flags")?;
    if ctx.layout.material_crc {
        r.u32("a material CRC")?;
    }
    let has_normals = r.bool("the has-normals flag")?;
    let normals = if has_normals {
        r.counted(n, 12, "normals", |r| r.vec3("a normal"))?
    } else {
        Vec::new()
    };
    let has_tangents = has_normals && data_flags & 0x1000 != 0;
    let (tangents, bitangents) = if has_tangents {
        (
            r.counted(n, 12, "tangents", |r| r.vec3("a tangent"))?,
            r.counted(n, 12, "bitangents", |r| r.vec3("a bitangent"))?,
        )
    } else {
        (Vec::new(), Vec::new())
    };
    let center = r.vec3("the bounding sphere")?;
    let radius = r.f32("the bounding sphere")?;
    let colors = if r.bool("the has-colors flag")? {
        r.counted(n, 16, "vertex colors", |r| {
            Ok([
                r.f32("a color")?,
                r.f32("a color")?,
                r.f32("a color")?,
                r.f32("a color")?,
            ])
        })?
    } else {
        Vec::new()
    };
    let uv_sets = if ctx.bs_version > 0 {
        usize::from(data_flags & 1)
    } else {
        usize::from(data_flags & 63)
    };
    let mut uvs = Vec::new();
    for set in 0..uv_sets {
        let coords = r.counted(n, 8, "texture coordinates", |r| {
            Ok([
                r.f32("a texture coordinate")?,
                r.f32("a texture coordinate")?,
            ])
        })?;
        if set == 0 {
            uvs = coords;
        }
    }
    r.u16("consistency flags")?;
    r.i32("an additional data reference")?;

    let num_triangles = usize::from(r.u16("the triangle count")?);
    let triangles = if strips {
        read_strips(r)?
    } else {
        let _num_points = r.u32("the triangle point count")?;
        let triangles = if r.bool("the has-triangles flag")? {
            r.counted(num_triangles, 6, "triangles", |r| {
                Ok([
                    r.u16("a triangle")?,
                    r.u16("a triangle")?,
                    r.u16("a triangle")?,
                ])
            })?
        } else {
            Vec::new()
        };
        let groups = usize::from(r.u16("the match group count")?);
        for _ in 0..groups {
            let count = usize::from(r.u16("a match group")?);
            r.take(count * 2, "a match group")?;
        }
        triangles
    };

    Ok(GeometryData {
        positions,
        normals,
        uvs,
        colors,
        triangles,
        center,
        radius,
        has_tangents,
        tangents,
        bitangents,
    })
}

/// Reads triangle strips and converts them to a triangle list, flipping
/// every other triangle to keep a consistent winding and dropping
/// degenerate triangles used to join strips.
fn read_strips(r: &mut Reader) -> Result<Vec<[u16; 3]>> {
    let num_strips = usize::from(r.u16("the strip count")?);
    let lengths = r.counted(num_strips, 2, "strip lengths", |r| r.u16("a strip length"))?;
    let mut triangles = Vec::new();
    if !r.bool("the has-points flag")? {
        return Ok(triangles);
    }
    for len in lengths {
        let len = usize::from(len);
        let points = r.counted(len, 2, "strip points", |r| r.u16("a strip point"))?;
        for i in 2..len {
            let (a, b, c) = (points[i - 2], points[i - 1], points[i]);
            if a == b || b == c || a == c {
                continue;
            }
            triangles.push(if i % 2 == 0 { [a, b, c] } else { [a, c, b] });
        }
    }
    Ok(triangles)
}

pub(crate) fn read_shader(
    r: &mut Reader,
    ctx: &Ctx,
    layout: ShaderLayout,
    type_name: &str,
) -> Result<ShaderProperty> {
    let net = read_object_net(r, ctx)?;
    let shade_flags = r.u16("shade flags")?;
    let shader_type = r.u32("the shader type")?;
    let shader_flags = r.u32("shader flags")?;
    let shader_flags2 = r.u32("shader flags")?;
    let env_map_scale = r.f32("the environment map scale")?;
    let texture_clamp = if layout == ShaderLayout::FileAfterScale {
        0
    } else {
        r.u32("the texture clamp mode")?
    };
    let lit = layout == ShaderLayout::TextureSet;
    let (texture_set, file_name) = if lit {
        (r.i32("a texture set reference")?, None)
    } else {
        (-1, Some(r.sized_string("a texture path")?))
    };
    // The remaining fields (refraction, parallax...) aren't needed yet, but
    // their expected size confirms everything before them was read at the
    // right offsets. The no-lighting falloff is read below.
    let v = ctx.bs_version;
    let expected_rest = match type_name {
        "BSShaderPPLightingProperty" | "Lighting30ShaderProperty" => {
            (if v > 14 { 8 } else { 0 })
                + (if v > 24 { 8 } else { 0 })
                + (if v > 34 { 16 } else { 0 })
        }
        // Falloff angles and opacities, from Bethesda version 27 on.
        "BSShaderNoLightingProperty" if v > 26 => 16,
        "SkyShaderProperty" => 4,
        _ => 0,
    };
    let layout_ok = r.remaining() == expected_rest;
    let falloff = if layout_ok && type_name == "BSShaderNoLightingProperty" && expected_rest == 16 {
        Some(Falloff {
            start_angle: r.f32("the falloff start angle")?,
            stop_angle: r.f32("the falloff stop angle")?,
            start_opacity: r.f32("the falloff start opacity")?,
            stop_opacity: r.f32("the falloff stop opacity")?,
        })
    } else {
        None
    };
    Ok(ShaderProperty {
        net,
        type_name: type_name.to_string(),
        lit,
        shade_flags,
        shader_type,
        shader_flags,
        shader_flags2,
        env_map_scale,
        texture_clamp,
        texture_set,
        file_name,
        layout_ok,
        falloff,
    })
}

/// Reads up to the base texture slot; the other slots aren't needed yet.
pub(crate) fn read_texturing(r: &mut Reader, ctx: &Ctx) -> Result<TexturingProperty> {
    read_object_net(r, ctx)?;
    let flags = r.u16("texturing flags")?;
    let _slot_count = r.u32("the texture slot count")?;
    let base_texture = if r.bool("the has-base-texture flag")? {
        r.i32("the base texture reference")?
    } else {
        -1
    };
    Ok(TexturingProperty {
        flags,
        base_texture,
    })
}

pub(crate) fn read_source_texture(r: &mut Reader, ctx: &Ctx) -> Result<SourceTexture> {
    read_object_net(r, ctx)?;
    let external = r.u8("the external-file flag")? == 1;
    // External textures name their file; embedded ones keep the original
    // file name, which is still the best hint for finding the texture.
    let file_name = ctx.string(r, "a texture file name")?;
    Ok(SourceTexture {
        external,
        file_name,
    })
}

pub(crate) fn read_texture_set(r: &mut Reader) -> Result<TextureSet> {
    let n = r.i32("the texture count")?.max(0) as usize;
    Ok(TextureSet {
        textures: r.counted(n, 4, "texture paths", |r| r.sized_string("a texture path"))?,
    })
}

/// The material's field list varies by Bethesda version; the block size
/// tells which variant this is.
pub(crate) fn read_material(r: &mut Reader, ctx: &Ctx) -> Result<MaterialProperty> {
    read_object_net(r, ctx)?;
    let (ambient_diffuse, emissive_mult) = match r.remaining() {
        36 => (false, true),
        32 => (false, false),
        60 => (true, true),
        56 => (true, false),
        _ => (ctx.bs_version <= 21, ctx.bs_version > 21),
    };
    if ambient_diffuse {
        r.take(24, "ambient and diffuse colors")?;
    }
    Ok(MaterialProperty {
        specular: r.vec3("the specular color")?,
        emissive: r.vec3("the emissive color")?,
        glossiness: r.f32("the glossiness")?,
        alpha: r.f32("the alpha")?,
        emissive_mult: if emissive_mult {
            r.f32("the emissive multiplier")?
        } else {
            1.0
        },
    })
}

pub(crate) fn read_alpha(r: &mut Reader, ctx: &Ctx) -> Result<AlphaProperty> {
    read_object_net(r, ctx)?;
    Ok(AlphaProperty {
        flags: r.u16("alpha flags")?,
        threshold: r.u8("the alpha threshold")?,
    })
}

pub(crate) fn read_stencil(r: &mut Reader, ctx: &Ctx) -> Result<StencilProperty> {
    read_object_net(r, ctx)?;
    Ok(StencilProperty {
        flags: r.u16("stencil flags")?,
    })
}

pub(crate) fn read_zbuffer(r: &mut Reader, ctx: &Ctx) -> Result<ZBufferProperty> {
    read_object_net(r, ctx)?;
    Ok(ZBufferProperty {
        flags: r.u16("depth buffer flags")?,
    })
}
