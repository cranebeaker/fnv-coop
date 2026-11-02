// New Vegas's distant land, ported from its shaders: `SLS2002.vso` (in
// Data\Shaders\shaderpackage013.sdp) and the pixel shader the Goodsprings
// recording used (not in package 13; `SLS2003.pso` without the parent):
//
//   vertex: z     = lerp(texcoord1, z, GeomorphParams.x)   (geomorphing)
//           inside the area of loaded cells (HighDetailRange), the land is
//           lowered (GeomorphParams.y) so the real terrain covers it
//   base  = BaseMap at uv × 0.9921875 + 0.00390625 (a texel in)
//   n     = 2 × NormalMap(uv) − 1      (world-space normals, not normalized)
//   while a new quarter fades from its parent (LODTexParams.w < 1):
//           base = lerp(LODParentTex(inset uv × 0.5 + 0.9921875 ×
//           LODTexParams.xy + 0.001953125), base, w), and the normal map
//           likewise from LODParentNormals(uv × 0.5 + LODTexParams.xy)
//   light = max(AmbientColor + SunColor × saturate(n · L), 0)
//   color = base × (0.55 + 0.8 × noise at 1.75 × uv) × light, fogged
//
// On stored values, as `game_lit.wgsl`. Past the game's far clip plane
// nothing is drawn (`lod::GAME_FAR_CLIP`).

#import bevy_pbr::{
    forward_io::FragmentOutput,
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_view_bindings as view_bindings

struct LodVertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
    // The height the vertex geomorphs toward (texcoord1), meters up.
    @location(3) morph: f32,
}

struct LodVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

struct LodLand {
    ambient: vec4<f32>,
    sun_color: vec4<f32>,
    // Toward the sun, in Bevy's axes.
    sun_direction: vec4<f32>,
    // The fog's color as stored, and its power in w.
    fog_color: vec4<f32>,
    // Fog near and far, the game's near clip plane (meters); w is 1 when
    // there's fog.
    fog_range: vec4<f32>,
    // x: the luminance shown at full brightness (see `game_lit.wgsl`).
    scale: vec4<f32>,
    // The loaded cells' area: center x and z, half width (meters), and how
    // far the land inside it is lowered (meters).
    high_detail: vec4<f32>,
}

// The hour's light outdoors, shared by every surface (`shared_light.rs`):
// the land is outdoors only and always takes it.
struct SharedLight {
    ambient: vec4<f32>,
    directional_color: vec4<f32>,
    directional_direction: vec4<f32>,
    fog_color: vec4<f32>,
    fog_range: vec4<f32>,
}

@group(2) @binding(100) var<uniform> lod: LodLand;
@group(2) @binding(101) var base_texture: texture_2d<f32>;
@group(2) @binding(102) var base_sampler: sampler;
@group(2) @binding(103) var normal_texture: texture_2d<f32>;
@group(2) @binding(104) var normal_sampler: sampler;
@group(2) @binding(105) var noise_texture: texture_2d<f32>;
@group(2) @binding(106) var noise_sampler: sampler;
// x: geomorph factor; y, z: the chunk's corner in its parent's texture; w:
// how far it has faded from the parent's texture (1: its own only).
@group(2) @binding(107) var<uniform> chunk: vec4<f32>;
@group(2) @binding(108) var parent_texture: texture_2d<f32>;
@group(2) @binding(109) var parent_sampler: sampler;
@group(2) @binding(110) var parent_normal_texture: texture_2d<f32>;
@group(2) @binding(111) var parent_normal_sampler: sampler;
// x: the game's far clip plane (meters).
@group(2) @binding(112) var<uniform> clip: vec4<f32>;
@group(2) @binding(113) var<storage, read> shared_light: SharedLight;

@vertex
fn vertex(vertex: LodVertex) -> LodVertexOutput {
    var out: LodVertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    var local = vertex.position;
    local.y = mix(vertex.morph, local.y, chunk.x);
    var p = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(local, 1.0));
    let h = lod.high_detail;
    if (abs(p.x - h.x) < h.z && abs(p.z - h.y) < h.z) {
        p.y -= h.w;
    }
    out.world_position = p;
    out.position = position_world_to_clip(p.xyz);
    out.uv = vertex.uv;
    return out;
}

fn fog_amount(p: vec3<f32>) -> f32 {
    if (shared_light.fog_range.w < 0.5) {
        return 0.0;
    }
    let v = view_bindings::view.view_from_world * vec4<f32>(p, 1.0);
    let projection = view_bindings::view.clip_from_view;
    let d = length(vec3<f32>(v.x * projection[0][0], v.y * projection[1][1], -v.z - shared_light.fog_range.z));
    let span = max(shared_light.fog_range.y - shared_light.fog_range.x, 1e-4);
    return pow(saturate((d - shared_light.fog_range.x) / span), shared_light.fog_color.w);
}

@fragment
fn fragment(in: LodVertexOutput) -> FragmentOutput {
    // Sampled as stored, as the game samples every texture.
    let inset = in.uv * 0.9921875 + 0.00390625;
    var base = textureSample(base_texture, base_sampler, inset).rgb;
    var stored = textureSample(normal_texture, normal_sampler, in.uv).rgb;
    // Fading from the parent's texture (`cmp` on w − 1: at 1, its own only).
    let parent_base = textureSample(parent_texture, parent_sampler, inset * 0.5 + (0.9921875 * chunk.yz + 0.001953125)).rgb;
    let parent_normal = textureSample(parent_normal_texture, parent_normal_sampler, in.uv * 0.5 + chunk.yz).rgb;
    if (chunk.w < 1.0) {
        base = mix(parent_base, base, chunk.w);
        stored = mix(parent_normal, stored, chunk.w);
    }
    let noise = textureSample(noise_texture, noise_sampler, in.uv * 1.75).r;
    // The game's axes (x east, y north, z up) to Bevy's.
    let g = stored * 2.0 - 1.0;
    let n = vec3<f32>(g.x, g.z, -g.y);
    let light = max(shared_light.ambient.rgb + shared_light.directional_color.rgb * saturate(dot(n, shared_light.directional_direction.xyz)), vec3<f32>(0.0));
    let color = base * (0.55 + 0.8 * noise) * light;
    let shaded = mix(color, shared_light.fog_color.rgb, fog_amount(in.world_position.xyz));
    // The game's far clip plane: depth along the view.
    let view_depth = -(view_bindings::view.view_from_world * vec4<f32>(in.world_position.xyz, 1.0)).z;
    if (view_depth > clip.x) {
        discard;
    }
    var out: FragmentOutput;
    out.color = vec4<f32>(shaded * lod.scale.x * view_bindings::view.exposure, 1.0);
    return out;
}
