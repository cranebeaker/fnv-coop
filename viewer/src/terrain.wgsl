// New Vegas's terrain, ported from its landscape shaders in
// Data\Shaders\shaderpackage013.sdp: `SLS2092`-`SLS2147` (pixel; seven
// groups for one to seven textures) with `SLS2100.vso` (vertex), as the
// Goodsprings recording drew them (`SLS2124`, `SLS2132`, `SLS2140`,
// `SLS2144`):
//
//   m     = normalize(sum of weight_i * (2 * NormalMap_i - 1))
//   l     = (normalize(T) . L, normalize(B) . L, normalize(N) . L)
//           (the light turned into the vertex's frame, each axis
//           interpolated and normalized on its own)
//   light = AmbientColor + SunColor * saturate(m . l)
//           (+ point lights, in the variants with PSLightPosition)
//   color = light * (sum of weight_i * BaseMap_i) * vertex color
//   color = lerp(color, FogColor, fog)       (fog worked out per vertex)
//
// The weights come per vertex from the engine (see
// `world::land::blend_weights`; recorded: texcoord 1 = base and layers
// 1-3, texcoord 2 = layers 4-6), the frame is the game's
// (`world::land::TerrainMesh::tangents`), the vertex color is the
// terrain's `VCLR`, and everything is on stored (gamma-encoded) values:
// the textures are sampled as stored, so filtering averages stored
// values as the game's samplers do (the recording sets no sRGB sampler
// state), and the result is written as stored values for the image space
// pass to decode.
//
// Past the loaded area's inner part the game blends each quarter toward
// the distant land's texture in a second pass (see `lod_blend` below).

#import bevy_pbr::{
    forward_io::FragmentOutput,
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_view_bindings as view_bindings

struct TerrainVertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(4) tangent: vec4<f32>,
    @location(5) color: vec4<f32>,
    @location(8) weights_a: vec4<f32>,
    @location(9) weights_b: vec4<f32>,
    @location(10) binormal: vec3<f32>,
}

struct TerrainVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(4) world_tangent: vec4<f32>,
    @location(5) color: vec4<f32>,
    @location(8) weights_a: vec4<f32>,
    @location(9) weights_b: vec4<f32>,
    @location(10) world_binormal: vec3<f32>,
    // x: the fog amount; y: how much of the distant land's texture shows
    // (the second pass's alpha); zw: where in that texture.
    @location(11) fog_blend: vec4<f32>,
}

struct GameLight {
    position_radius: vec4<f32>,
    color: vec4<f32>,
}

// The same uniform as `game_lit.wgsl`'s; the terrain uses its ambient,
// sun, fog, scale and lights.
struct GameLighting {
    ambient: vec4<f32>,
    directional_color: vec4<f32>,
    directional_direction: vec4<f32>,
    emissive: vec4<f32>,
    scale: vec4<f32>,
    fog_color: vec4<f32>,
    fog_range: vec4<f32>,
    specular: vec4<f32>,
    surface: vec4<f32>,
    falloff: vec4<f32>,
    environment: vec4<f32>,
    draw: vec4<f32>,
    lights: array<GameLight, 64>,
}

// The blend toward the distant land (`LandBlendParams` and the blend
// pass's constants, recorded): the player's cell's middle (Bevy x and z,
// meters), the distance where the blend is full and the width it ramps
// over (meters); this quarter's corner in the distant-land texture (u, v,
// from its chunk's south-west), and 1 when there's a texture to blend to.
struct LandBlend {
    centre_full_width: vec4<f32>,
    offset: vec4<f32>,
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

@group(2) @binding(100) var<uniform> game: GameLighting;
@group(2) @binding(101) var base_0: texture_2d<f32>;
@group(2) @binding(102) var base_1: texture_2d<f32>;
@group(2) @binding(103) var base_2: texture_2d<f32>;
@group(2) @binding(104) var base_3: texture_2d<f32>;
@group(2) @binding(105) var base_4: texture_2d<f32>;
@group(2) @binding(106) var base_5: texture_2d<f32>;
@group(2) @binding(107) var base_6: texture_2d<f32>;
@group(2) @binding(108) var normal_0: texture_2d<f32>;
@group(2) @binding(109) var normal_1: texture_2d<f32>;
@group(2) @binding(110) var normal_2: texture_2d<f32>;
@group(2) @binding(111) var normal_3: texture_2d<f32>;
@group(2) @binding(112) var normal_4: texture_2d<f32>;
@group(2) @binding(113) var normal_5: texture_2d<f32>;
@group(2) @binding(114) var normal_6: texture_2d<f32>;
@group(2) @binding(115) var terrain_sampler: sampler;
@group(2) @binding(116) var<uniform> land_blend: LandBlend;
@group(2) @binding(117) var lod_base: texture_2d<f32>;
@group(2) @binding(118) var lod_normal: texture_2d<f32>;
@group(2) @binding(119) var lod_noise: texture_2d<f32>;
@group(2) @binding(120) var lod_sampler: sampler;
@group(2) @binding(121) var<storage, read> shared_light: SharedLight;

// As `game_lit.wgsl`'s: the length of the position after the game's
// projection (x and y scaled by it, depth from its near plane), then
// saturate((d - near) / (far - near)) ^ power.
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

@vertex
fn vertex(vertex: TerrainVertex) -> TerrainVertexOutput {
    var out: TerrainVertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    out.uv = vertex.uv;
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(world_from_local, vertex.tangent, vertex.instance_index);
    out.world_binormal = mesh_functions::mesh_tangent_local_to_world(world_from_local, vec4<f32>(vertex.binormal, 1.0), vertex.instance_index).xyz;
    out.color = vertex.color;
    out.weights_a = vertex.weights_a;
    out.weights_b = vertex.weights_b;
    // Fog per vertex, as `SLS2100.vso` works it out.
    out.fog_blend.x = fog_amount(out.world_position.xyz);
    // The blend pass's vertex shader: by the flat distance from the
    // player's cell's middle, 0 inside, rising to 1 at the full distance.
    let b = land_blend.centre_full_width;
    let d = length(out.world_position.xz - b.xy);
    out.fog_blend.y = (1.0 - clamp((b.z - d) / max(b.w, 1e-4), 0.0, 1.0)) * land_blend.offset.w;
    // Its texture coordinates: the quarter's own / 64 (a chunk is eight
    // quarters across) from the quarter's corner, u counted from the
    // east, a texel in from the edges.
    let uv = vertex.uv * 0.015625 + land_blend.offset.xy;
    out.fog_blend.z = (1.0 - uv.x) * 0.9921875 + 0.00390625;
    out.fog_blend.w = uv.y * 0.9921875 + 0.00390625;
    return out;
}

// One texture's stored color and its normal map's direction (2 * stored -
// 1), both times its weight.
struct Layer {
    color: vec3<f32>,
    normal: vec3<f32>,
}

fn layer(base: texture_2d<f32>, normal: texture_2d<f32>, uv: vec2<f32>, w: f32) -> Layer {
    var out: Layer;
    // Sampled even at weight 0, so the derivatives stay valid everywhere.
    out.color = textureSample(base, terrain_sampler, uv).rgb * w;
    out.normal = (textureSample(normal, terrain_sampler, uv).rgb * 2.0 - 1.0) * w;
    return out;
}

@fragment
fn fragment(in: TerrainVertexOutput) -> FragmentOutput {
    let wa = in.weights_a;
    let wb = in.weights_b;
    var color = vec3<f32>(0.0);
    var bent = vec3<f32>(0.0);
    var l: Layer;
    l = layer(base_0, normal_0, in.uv, wa.x); color += l.color; bent += l.normal;
    l = layer(base_1, normal_1, in.uv, wa.y); color += l.color; bent += l.normal;
    l = layer(base_2, normal_2, in.uv, wa.z); color += l.color; bent += l.normal;
    l = layer(base_3, normal_3, in.uv, wa.w); color += l.color; bent += l.normal;
    l = layer(base_4, normal_4, in.uv, wb.x); color += l.color; bent += l.normal;
    l = layer(base_5, normal_5, in.uv, wb.y); color += l.color; bent += l.normal;
    l = layer(base_6, normal_6, in.uv, wb.z); color += l.color; bent += l.normal;

    let t = normalize(in.world_tangent.xyz);
    let b = normalize(in.world_binormal);
    let vertex_normal = normalize(in.world_normal);
    let m = bent * inverseSqrt(max(dot(bent, bent), 1e-12));

    let p = in.world_position.xyz;
    let sun = normalize(shared_light.directional_direction.xyz);
    var light = shared_light.ambient.rgb;
    light += shared_light.directional_color.rgb * saturate(dot(m, vec3<f32>(dot(t, sun), dot(b, sun), dot(vertex_normal, sun))));
    let count = min(u32(game.scale.y), 64u);
    for (var i = 0u; i < count; i += 1u) {
        let g = game.lights[i];
        let to_light = g.position_radius.xyz - p;
        let d2 = dot(to_light, to_light);
        let r2 = g.position_radius.w * g.position_radius.w;
        if (d2 < r2) {
            let dir = to_light * inverseSqrt(max(d2, 1e-8));
            let n = normalize(m.x * t + m.y * b + m.z * vertex_normal);
            light += g.color.rgb * (1.0 - saturate(d2 / r2)) * saturate(dot(n, dir));
        }
    }

    // The vertex color arrives as stored values (the mesh carries them
    // unconverted).
    let fog = in.fog_blend.x;
    var shaded = mix(light * color * in.color.rgb, shared_light.fog_color.rgb, fog);
    // The blend pass, over the finished quarter (alpha blending, depth
    // equal): the distant land's colour there, lit by the ambient and the
    // sun with its world-space normal map, fogged the same way.
    if (in.fog_blend.y > 0.0) {
        shaded = mix(shaded, lod_blend(in.fog_blend.zw, sun, fog), in.fog_blend.y);
    }
    var out: FragmentOutput;
    out.color = vec4<f32>(shaded * game.scale.x * view_bindings::view.exposure, 1.0);
    return out;
}

// The blend pass's pixel shader (recorded; not in the package): the
// distant-land texture × (0.55 + 0.8 × the noise at 1.75 × the
// coordinates) × (ambient + sun × saturate(n · L)), n = 2 × the distant
// land's normal map − 1 in the world's axes (not normalized), then fog.
fn lod_blend(uv: vec2<f32>, sun: vec3<f32>, fog: f32) -> vec3<f32> {
    let base = textureSample(lod_base, lod_sampler, uv).rgb;
    let noise = textureSample(lod_noise, lod_sampler, uv * 1.75).r;
    let g = textureSample(lod_normal, lod_sampler, uv).rgb * 2.0 - 1.0;
    // The game's axes (x east, y north, z up) to Bevy's.
    let n = vec3<f32>(g.x, g.z, -g.y);
    let light = shared_light.ambient.rgb + shared_light.directional_color.rgb * saturate(dot(n, sun));
    return mix(base * (0.55 + 0.8 * noise) * light, shared_light.fog_color.rgb, fog);
}
