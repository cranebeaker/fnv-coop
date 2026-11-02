// New Vegas's lighting, in place of Bevy's physically based lighting, for
// every lit surface. Ported from the game's own lit-surface pixel shaders
// in Data\Shaders\shaderpackage013.sdp (the package RendererInfo.txt names
// on this PC); `SLS2029` lights with the directional light and five point
// lights in one pass, `SLS2004` / `SLS2012` add the glow map:
//
//   light = AmbientColor
//         + EmittanceColor * GlowMap
//         + DirectionalColor * saturate(N.L)
//         + sum of PointColor * saturate(N.L) * (1 - saturate(d^2 / r^2))
//   color = max(light, 0) * texture * vertex color
//   color = lerp(color, FogColor, fog amount)
//   color += specular (a separate additive pass in the game)
//   color += reflection (another additive pass; see `reflection`)
//
// N is the normal map's normal where there is one.
//
// all on the colors as stored (the game never converts to linear light).
// Textures are sampled as stored, as the game's samplers do (filtering
// averages stored values); vertex colors arrive decoded and are encoded
// back. This shader does the game's arithmetic and writes stored values:
// the frame buffer holds what the game's does, blending happens on those
// values as in the game, and the image space pass decodes the finished
// picture for the screen. With the camera's exposure cancelled and no tone
// mapping, the screen then shows the game's own numbers.
//
// Per vertex, as the game does: its vertex shaders work out each point
// light's direction (normalized), the specular half vector, fog and the
// fade by viewing angle at each vertex, and the GPU blends them across the
// triangle. Every vertex here carries its triangle's three corners
// (`GAME_CORNERS`), so the fragment shader works those out at the corners
// and blends them with the same weights. (The game turns each corner's
// light direction into that corner's normal-map frame before blending;
// here they're blended in world space and turned with the pixel's frame,
// which is the same wherever a triangle's corners share a frame: walls,
// floors and ceilings.) Point lights fade with the exact distance, as in
// the game.
//
// Specular is the light's color alone (the recorded specular passes get
// the plain light colors; the material's specular color isn't used) with
// the material's glossiness as the power. Guess: every light (the
// directional one too) gets a highlight.
//
// People's skin and hair are lit as the game's own skin passes and hair
// shader do (both compiled by the game at run time; read from the actors
// recording): see `preview::cell::Shading` and `actor` below.

#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::main_pass_post_lighting_processing,
    mesh_functions,
    skinning,
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_view_bindings as view_bindings
#import bevy_pbr::pbr_types
#import bevy_pbr::mesh_bindings::mesh
#ifdef BINDLESS
#import bevy_render::bindless::{bindless_samplers_filtering, bindless_textures_2d, bindless_textures_cube}
#endif

// Bevy's mesh vertex input, plus the triangle's corners.
struct GameVertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS_A
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(4) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(5) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(6) joint_indices: vec4<u32>,
    @location(7) joint_weights: vec4<f32>,
#endif
#ifdef GAME_CORNERS
    @location(8) corner_a: vec3<f32>,
    @location(9) corner_b: vec3<f32>,
    @location(10) corner_c: vec3<f32>,
#endif
}

// Bevy's vertex output, plus the triangle's corners in world space (the
// same at all three vertices, so they arrive unchanged).
struct GameVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_UVS_A
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_UVS_B
    @location(3) uv_b: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(4) world_tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(5) color: vec4<f32>,
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    @location(6) @interpolate(flat) instance_index: u32,
#endif
#ifdef VISIBILITY_RANGE_DITHER
    @location(7) @interpolate(flat) visibility_range_dither: i32,
#endif
#ifdef GAME_CORNERS
    @location(8) corner_a: vec3<f32>,
    @location(9) corner_b: vec3<f32>,
    @location(10) corner_c: vec3<f32>,
#endif
}

// Bevy's mesh vertex shader (`bevy_pbr::render::mesh`), less morph
// targets, which these meshes don't use, plus the corners. People and
// creatures are skinned (`SKINNED`): their bones move them.
@vertex
fn vertex(vertex: GameVertex) -> GameVertexOutput {
    set_game_slot(vertex.instance_index);
    var out: GameVertexOutput;
#ifdef SKINNED
    let world_from_local = skinning::skin_model(vertex.joint_indices, vertex.joint_weights, vertex.instance_index);
#else
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
#endif
#ifdef VERTEX_NORMALS
#ifdef SKINNED
    out.world_normal = skinning::skin_normals(world_from_local, vertex.normal);
#else
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
#endif
#endif
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    // Decals are pulled nearer by a constant in depth (0 for the rest), as
    // the game's depth bias does; depth is z / w, so add w times it.
    out.position.z += out.position.w * game_draw().x;
#ifdef VERTEX_UVS_A
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(world_from_local, vertex.tangent, vertex.instance_index);
#endif
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    out.instance_index = vertex.instance_index;
#endif
#ifdef VISIBILITY_RANGE_DITHER
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex.instance_index, world_from_local[3]);
#endif
#ifdef GAME_CORNERS
    out.corner_a = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.corner_a, 1.0)).xyz;
    out.corner_b = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.corner_b, 1.0)).xyz;
    out.corner_c = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.corner_c, 1.0)).xyz;
#endif
    return out;
}

// What Bevy's material functions take.
fn bevy_input(g: GameVertexOutput) -> VertexOutput {
    var in: VertexOutput;
    in.position = g.position;
    in.world_position = g.world_position;
    in.world_normal = g.world_normal;
#ifdef VERTEX_UVS_A
    in.uv = g.uv;
#endif
#ifdef VERTEX_UVS_B
    in.uv_b = g.uv_b;
#endif
#ifdef VERTEX_TANGENTS
    in.world_tangent = g.world_tangent;
#endif
#ifdef VERTEX_COLORS
    in.color = g.color;
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    in.instance_index = g.instance_index;
#endif
#ifdef VISIBILITY_RANGE_DITHER
    in.visibility_range_dither = g.visibility_range_dither;
#endif
    return in;
}

// The triangle's corners and this pixel's weights for each: what the GPU
// blends per-vertex values with. Without corners, the pixel itself.
struct Corners {
    a: vec3<f32>,
    b: vec3<f32>,
    c: vec3<f32>,
    weights: vec3<f32>,
}

fn corners_of(g: GameVertexOutput) -> Corners {
    let p = g.world_position.xyz;
    var out: Corners;
    out.a = p;
    out.b = p;
    out.c = p;
    out.weights = vec3<f32>(1.0, 0.0, 0.0);
#ifdef GAME_CORNERS
    let e0 = g.corner_b - g.corner_a;
    let e1 = g.corner_c - g.corner_a;
    let e2 = p - g.corner_a;
    let d00 = dot(e0, e0);
    let d01 = dot(e0, e1);
    let d11 = dot(e1, e1);
    let denom = d00 * d11 - d01 * d01;
    // Degenerate (zero-area) triangles keep the pixel's own values.
    if (abs(denom) > 1e-12 * max(d00 * d11, 1e-30)) {
        let d20 = dot(e2, e0);
        let d21 = dot(e2, e1);
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        out.a = g.corner_a;
        out.b = g.corner_b;
        out.c = g.corner_c;
        out.weights = vec3<f32>(1.0 - v - w, v, w);
    }
#endif
    return out;
}

// Normalized direction toward `toward`, worked out at each corner and
// blended (the game's per-vertex light direction).
fn blended_toward(k: Corners, toward: vec3<f32>) -> vec3<f32> {
    let d = k.weights.x * normalize(toward - k.a)
        + k.weights.y * normalize(toward - k.b)
        + k.weights.z * normalize(toward - k.c);
    return d * inverseSqrt(max(dot(d, d), 1e-12));
}

// The specular half vector between a light (a position, or with
// `is_direction` a direction) and the eye, worked out at each corner and
// blended (`SLS2048.vso` and kin).
fn blended_half(k: Corners, light: vec3<f32>, is_direction: bool, eye: vec3<f32>) -> vec3<f32> {
    let h = k.weights.x * half_at(k.a, light, is_direction, eye)
        + k.weights.y * half_at(k.b, light, is_direction, eye)
        + k.weights.z * half_at(k.c, light, is_direction, eye);
    return h * inverseSqrt(max(dot(h, h), 1e-12));
}

fn half_at(p: vec3<f32>, light: vec3<f32>, is_direction: bool, eye: vec3<f32>) -> vec3<f32> {
    var to_light = light;
    if (!is_direction) {
        to_light = normalize(light - p);
    }
    return normalize(to_light + normalize(eye - p));
}

struct GameLight {
    // Position and radius, in meters in Bevy's space.
    position_radius: vec4<f32>,
    // Color times its brightness multiplier.
    color: vec4<f32>,
}

struct GameLighting {
    ambient: vec4<f32>,
    directional_color: vec4<f32>,
    // Toward the light.
    directional_direction: vec4<f32>,
    // The surface's own glow; w is 1 when the glow map masks it.
    emissive: vec4<f32>,
    // x: the luminance shown at full brightness at the starting exposure,
    // which cancels the camera's exposure there; y: how many lights; z: 1
    // when the ambient, sun and fog are the hour's shared light outdoors.
    scale: vec4<f32>,
    // The fog's color as stored, and its power in w.
    fog_color: vec4<f32>,
    // Fog near and far in meters; w is 1 when there's fog.
    fog_range: vec4<f32>,
    // The material's specular color, and its glossiness in w.
    specular: vec4<f32>,
    // x: 1 with a normal map; y: 1 when the surface gets specular; z: 1
    // for a no-lighting surface (its color in emissive.rgb); w: for a
    // no-lighting surface, how it fogs (0 toward the fog color, 1 to black,
    // 2 to white); for a lit one, 1 when it's alpha-blended.
    surface: vec4<f32>,
    // Fade by viewing angle: start and stop cosines, start and stop opacity.
    falloff: vec4<f32>,
    // The reflection: strength (0: none), 1 with a mask texture, 1 for
    // window reflections, the material's opacity.
    environment: vec4<f32>,
    // x: how much nearer a decal is drawn, added to Bevy's depth (the
    // game's D3DRS_DEPTHBIAS, `lighting::decal_depth_offset`); y: the alpha
    // test's comparison (0 none, 1 never, 2 less, 3 equal, 4 less or equal,
    // 5 greater, 6 not equal, 7 greater or equal, 8 always); z: its
    // threshold.
    draw: vec4<f32>,
    lights: array<GameLight, 64>,
    // People's surfaces: x 0 plain, 1 skin, 2 hair; y the factor skin's
    // directional light is multiplied by (the image space's float 2).
    actor: vec4<f32>,
    // Hair: the NPC's hair colour (the hair shader's HairTint).
    hair_tint: vec4<f32>,
}

// The hour's light outdoors, shared by every surface (`shared_light.rs`).
struct SharedLight {
    ambient: vec4<f32>,
    directional_color: vec4<f32>,
    directional_direction: vec4<f32>,
    fog_color: vec4<f32>,
    fog_range: vec4<f32>,
}

// The material's resources (`lighting::GameLit`). Bindless (`BINDLESS`):
// one array of every material's lighting, the textures in Bevy's bindless
// arrays, each material's entries found through its slot in the index
// table (the mesh's `material_and_lightmap_bind_group_slot`); otherwise
// each bound on its own.
struct GameLitIndices {
    lighting: u32,
    glow: u32,
    glow_sampler: u32,
    normal: u32,
    normal_sampler: u32,
    environment: u32,
    environment_sampler: u32,
    environment_mask: u32,
    environment_mask_sampler: u32,
    shared_light: u32,
}

#ifdef BINDLESS
@group(2) @binding(100) var<storage> game_lit_indices: array<GameLitIndices>;
@group(2) @binding(101) var<storage> game_lit: array<GameLighting>;
#else
@group(2) @binding(50) var<uniform> game: GameLighting;
@group(2) @binding(51) var glow_texture: texture_2d<f32>;
@group(2) @binding(52) var glow_sampler: sampler;
@group(2) @binding(53) var normal_texture: texture_2d<f32>;
@group(2) @binding(54) var normal_sampler: sampler;
@group(2) @binding(55) var environment_texture: texture_cube<f32>;
@group(2) @binding(56) var environment_sampler: sampler;
@group(2) @binding(57) var environment_mask: texture_2d<f32>;
@group(2) @binding(58) var environment_mask_sampler: sampler;
@group(2) @binding(59) var shared_light_texture: texture_2d<f32>;
#endif

// The material being drawn's slot (set first thing in each entry point).
var<private> game_slot: u32;

fn set_game_slot(instance_index: u32) {
    game_slot = mesh[instance_index].material_and_lightmap_bind_group_slot & 0xffffu;
}

#ifdef BINDLESS
fn game_index() -> u32 {
    return game_lit_indices[game_slot].lighting;
}
fn game_ambient() -> vec4<f32> { return game_lit[game_index()].ambient; }
fn game_directional_color() -> vec4<f32> { return game_lit[game_index()].directional_color; }
fn game_directional_direction() -> vec4<f32> { return game_lit[game_index()].directional_direction; }
fn game_emissive() -> vec4<f32> { return game_lit[game_index()].emissive; }
fn game_scale() -> vec4<f32> { return game_lit[game_index()].scale; }
fn game_fog_color() -> vec4<f32> { return game_lit[game_index()].fog_color; }
fn game_fog_range() -> vec4<f32> { return game_lit[game_index()].fog_range; }
fn game_specular() -> vec4<f32> { return game_lit[game_index()].specular; }
fn game_surface() -> vec4<f32> { return game_lit[game_index()].surface; }
fn game_falloff() -> vec4<f32> { return game_lit[game_index()].falloff; }
fn game_environment() -> vec4<f32> { return game_lit[game_index()].environment; }
fn game_draw() -> vec4<f32> { return game_lit[game_index()].draw; }
fn game_light(i: u32) -> GameLight { return game_lit[game_index()].lights[i]; }
fn game_actor() -> vec4<f32> { return game_lit[game_index()].actor; }
fn game_hair_tint() -> vec4<f32> { return game_lit[game_index()].hair_tint; }
fn sample_glow(uv: vec2<f32>) -> vec4<f32> {
    let k = game_lit_indices[game_slot];
    return textureSample(bindless_textures_2d[k.glow], bindless_samplers_filtering[k.glow_sampler], uv);
}
fn sample_normal(uv: vec2<f32>) -> vec4<f32> {
    let k = game_lit_indices[game_slot];
    return textureSample(bindless_textures_2d[k.normal], bindless_samplers_filtering[k.normal_sampler], uv);
}
fn sample_environment(along: vec3<f32>) -> vec4<f32> {
    let k = game_lit_indices[game_slot];
    return textureSample(bindless_textures_cube[k.environment], bindless_samplers_filtering[k.environment_sampler], along);
}
fn sample_environment_mask(uv: vec2<f32>) -> vec4<f32> {
    let k = game_lit_indices[game_slot];
    return textureSample(bindless_textures_2d[k.environment_mask], bindless_samplers_filtering[k.environment_mask_sampler], uv);
}
fn shared_light_vector(i: i32) -> vec4<f32> {
    return textureLoad(bindless_textures_2d[game_lit_indices[game_slot].shared_light], vec2<i32>(i, 0), 0);
}
#else
fn game_ambient() -> vec4<f32> { return game.ambient; }
fn game_directional_color() -> vec4<f32> { return game.directional_color; }
fn game_directional_direction() -> vec4<f32> { return game.directional_direction; }
fn game_emissive() -> vec4<f32> { return game.emissive; }
fn game_scale() -> vec4<f32> { return game.scale; }
fn game_fog_color() -> vec4<f32> { return game.fog_color; }
fn game_fog_range() -> vec4<f32> { return game.fog_range; }
fn game_specular() -> vec4<f32> { return game.specular; }
fn game_surface() -> vec4<f32> { return game.surface; }
fn game_falloff() -> vec4<f32> { return game.falloff; }
fn game_environment() -> vec4<f32> { return game.environment; }
fn game_draw() -> vec4<f32> { return game.draw; }
fn game_light(i: u32) -> GameLight { return game.lights[i]; }
fn game_actor() -> vec4<f32> { return game.actor; }
fn game_hair_tint() -> vec4<f32> { return game.hair_tint; }
fn sample_glow(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(glow_texture, glow_sampler, uv);
}
fn sample_normal(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(normal_texture, normal_sampler, uv);
}
fn sample_environment(along: vec3<f32>) -> vec4<f32> {
    return textureSample(environment_texture, environment_sampler, along);
}
fn sample_environment_mask(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(environment_mask, environment_mask_sampler, uv);
}
fn shared_light_vector(i: i32) -> vec4<f32> {
    return textureLoad(shared_light_texture, vec2<i32>(i, 0), 0);
}
#endif

// The ambient, the sun and the fog: the surface's own, or the hour's
// shared light outdoors.
fn outdoors() -> bool {
    return game_scale().z > 0.5;
}
fn ambient_light() -> vec4<f32> {
    if (outdoors()) {
        return shared_light_vector(0);
    }
    return game_ambient();
}
fn directional_color() -> vec4<f32> {
    if (outdoors()) {
        return shared_light_vector(1);
    }
    return game_directional_color();
}
fn directional_direction() -> vec4<f32> {
    if (outdoors()) {
        return shared_light_vector(2);
    }
    return game_directional_direction();
}
fn fog_color() -> vec4<f32> {
    if (outdoors()) {
        return shared_light_vector(3);
    }
    return game_fog_color();
}
fn fog_range() -> vec4<f32> {
    if (outdoors()) {
        return shared_light_vector(4);
    }
    return game_fog_range();
}

// The sRGB curve both ways, exactly as the GPU applies it to sRGB textures
// and to the screen, so encoding undoes its decoding.
fn srgb_encode(linear: vec3<f32>) -> vec3<f32> {
    let c = max(linear, vec3<f32>(0.0));
    return select(1.055 * pow(c, vec3<f32>(1.0 / 2.4)) - 0.055, c * 12.92, c <= vec3<f32>(0.0031308));
}

// The specular highlight for one light, before its color (`SLS2047`,
// `SLS2053`): Blinn-Phong with the material's glossiness as the power,
// masked by the normal map's alpha, and faded out where the surface turns
// from the light: below N.L = 0.2 it's also times saturate(N.L + 0.5).
// `h` is the half vector the game blends from the vertices; the light's
// direction for the fade is exact (the specular vertex shaders pass it
// unnormalized, and the blend of that is exact).
fn highlight(n: vec3<f32>, h: vec3<f32>, to_light: vec3<f32>, mask: f32) -> f32 {
    var s = pow(max(saturate(dot(n, h)), 1e-6), max(game_specular().w, 1e-3)) * mask;
    let n_dot_l = dot(n, to_light);
    if (n_dot_l <= 0.2) {
        s *= saturate(n_dot_l + 0.5);
    }
    return s;
}

@fragment
fn fragment(
    game_in: GameVertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    set_game_slot(game_in.instance_index);
    let in = bevy_input(game_in);
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    // The alpha before Bevy's own handling (which makes opaque surfaces'
    // alpha 1): what the game's alpha test compares.
    let raw_alpha = pbr_input.material.base_color.a;
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out: FragmentOutput;
    let base = pbr_input.material.base_color;
    let textured = (pbr_input.material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_BASE_COLOR_TEXTURE_BIT) != 0u;
    let surface = stored_surface(in, base.rgb, textured);
    let p = pbr_input.world_position.xyz;
    let eye = view_bindings::view.world_position.xyz;
    let to_eye = normalize(eye - p);
    let k = corners_of(game_in);
    // Fog per vertex, blended.
    let fog = k.weights.x * fog_amount(k.a) + k.weights.y * fog_amount(k.b) + k.weights.z * fog_amount(k.c);
    // The vertex normal as it is: the game's lit shaders can't tell which
    // side of a two-sided surface is drawn, so they never turn it around
    // (Bevy's own normal here would be flipped on back faces).
    let vertex_normal = normalize(in.world_normal);
    var shaded: vec3<f32>;
    var alpha = base.a;
    // No-lighting surfaces fade by viewing angle (per vertex, blended).
    var fade = 1.0;
    if (game_surface().z > 0.5) {
        fade = k.weights.x * falloff_opacity(abs(dot(vertex_normal, normalize(eye - k.a))))
            + k.weights.y * falloff_opacity(abs(dot(vertex_normal, normalize(eye - k.b))))
            + k.weights.z * falloff_opacity(abs(dot(vertex_normal, normalize(eye - k.c))));
    }
    // The game's alpha test, on the alpha its pixel shader writes.
    if (!alpha_test_passes(raw_alpha * fade)) {
        discard;
    }

    if (game_surface().z > 0.5) {
        // A no-lighting surface (`NOLIGHTTEXVC` and its kin): texture ×
        // vertex color × MaterialColor, its opacity faded by viewing
        // angle, and fog by the kind of surface.
        shaded = surface * game_emissive().rgb;
        alpha *= fade;
        if (game_surface().w > 1.5) {
            shaded = mix(shaded, vec3<f32>(1.0), saturate(1.5 * fog));
        } else if (game_surface().w > 0.5) {
            shaded *= 1.0 - fog;
        } else {
            shaded = mix(shaded, fog_color().rgb, fog);
        }
    } else {
    var n = vertex_normal;
    // The normal reflections use: the reflection pass's vertex shader
    // scales the texture's U and V directions by 0.1, so the normal map
    // tilts reflections only a tenth as much.
    var reflect_normal = vertex_normal;
    // The specular mask: the normal map's alpha. Without a normal map it's
    // a guess (full strength).
    var specular_mask = 1.0;
#ifdef VERTEX_TANGENTS
#ifdef VERTEX_UVS_A
    // The normal map (`SLS2001` and the rest): its stored value times 2
    // minus 1, normalized, in the frame of the texture's U (red), V
    // (green) and the vertex normal (blue); U and V come from the mesh
    // (`cellview::MeshData::tangents`).
    if (game_surface().x > 0.5) {
        let t = normalize(in.world_tangent.xyz - vertex_normal * dot(in.world_tangent.xyz, vertex_normal));
        let b = in.world_tangent.w * cross(vertex_normal, t);
        let stored = sample_normal(in.uv);
        let m = normalize(stored.rgb * 2.0 - 1.0);
        n = normalize(m.x * t + m.y * b + m.z * vertex_normal);
        reflect_normal = normalize(0.1 * (m.x * t + m.y * b) + m.z * vertex_normal);
        specular_mask = stored.a;
    }
#endif
#endif
    let gets_specular = game_surface().y > 0.5;
    var specular = vec3<f32>(0.0);
    // People (`preview::cell::Shading`): skin's own passes and the hair
    // shader, as the game draws them (compiled by the game at run time,
    // read from the actors recording).
    let is_skin = game_actor().x > 0.5 && game_actor().x < 1.5;
    let is_hair = game_actor().x > 1.5;
    // The view direction per vertex (skin's rim) and per pixel (hair).
    let to_eye_blended = blended_toward(k, eye);
    // Hair: the bent normal its highlight uses, half the normal map's
    // normal plus the vertex normal.
    let bent = normalize(0.5 * n + vertex_normal);
    var hair_specular = vec3<f32>(0.0);

    var light = ambient_light().rgb;
    let sun = normalize(directional_direction().xyz);
    // Skin's directional pass (`SLS1002`) gets the light times the image
    // space's factor, its highlight too.
    var sun_color = directional_color().rgb;
    if (is_skin) {
        sun_color *= game_actor().y;
    }
    light += sun_color * saturate(dot(n, sun));
    if (gets_specular) {
        let h = blended_half(k, sun, true, eye);
        specular += saturate(highlight(n, h, sun, specular_mask) * sun_color);
    }
    let count = min(u32(game_scale().y), 64u);
    for (var i = 0u; i < count; i += 1u) {
        let l = game_light(i);
        let to_light = l.position_radius.xyz - p;
        let d2 = dot(to_light, to_light);
        let r2 = l.position_radius.w * l.position_radius.w;
        if (d2 < r2) {
            // The direction per vertex (`SLS2020.vso`), the fade by the
            // exact distance (the pixel shader's). The hair shader works
            // the direction out per pixel.
            let exact = to_light * inverseSqrt(max(d2, 1e-8));
            var dir = blended_toward(k, l.position_radius.xyz);
            if (is_hair) {
                dir = exact;
            }
            let fade = 1.0 - saturate(d2 / r2);
            let reach = l.color.rgb * fade;
            let n_dot_l = dot(n, dir);
            light += reach * saturate(n_dot_l);
            if (is_skin) {
                // A rim where the surface turns from the eye with the
                // light behind it, and a red glow where the light wraps
                // past the lit edge (smooth steps of N.L and of
                // (N.L + 0.3) / 1.3).
                let n_dot_v = saturate(dot(n, to_eye_blended));
                let rim = (1.0 - n_dot_v) * (1.0 - n_dot_v) * saturate(dot(to_eye_blended, -dir));
                let x = saturate(n_dot_l);
                let w = saturate((n_dot_l + 0.3) * 0.769230783);
                let wrap = saturate(w * w * (3.0 - 2.0 * w) - x * x * (3.0 - 2.0 * x));
                light += fade * (rim * (0.5 * l.color.rgb + vec3<f32>(0.15, 0.0, 0.0))
                    + vec3<f32>(0.3, 0.0, 0.0) * wrap);
            }
            if (is_hair) {
                let half_vector = normalize(to_eye + exact);
                let s = 1.0 - saturate(abs(dot(bent, exact) - dot(bent, half_vector)));
                hair_specular += 0.7 * pow(s, 30.0) * reach * max(dot(exact, vertex_normal), 0.0);
            }
            if (gets_specular) {
                let h = blended_half(k, l.position_radius.xyz, false, eye);
                specular += saturate(highlight(n, h, exact, specular_mask) * reach);
            }
        }
    }
    if (is_hair) {
        specular += hair_specular * hair_mask(in) * specular_mask;
    }

    // EmittanceColor times the glow map's stored color.
    var mask = vec3<f32>(1.0);
#ifdef VERTEX_UVS_A
    if (game_emissive().w > 0.5) {
        mask = sample_glow(in.uv).rgb;
    }
#endif
    light += game_emissive().rgb * mask;
    light = max(light, vec3<f32>(0.0));

    // Specular is added before fog, as the game draws it (in one pass, or
    // in passes added before its fog pass).
    shaded = mix(surface * light + specular, fog_color().rgb, fog);

    // The reflection, added on top of the finished surface (the game's
    // separate pass blends ONE + ONE). On an alpha-blended surface it's
    // divided by the alpha so that blending adds it whole, as the game's
    // pass does.
    if (game_environment().x > 0.0) {
        // The view direction per vertex, as the reflection pass's vertex
        // shader gives it.
        var reflected = reflection(in, reflect_normal, blended_toward(k, eye), specular_mask, fog);
        if (game_surface().w > 0.5) {
            reflected /= max(alpha, 1e-3);
        }
        shaded += reflected;
    }
    }

    // Written as stored (gamma-encoded) values, as the game's frame buffer
    // holds them, so blended surfaces (light beams, glass, decals) blend
    // the way the game blends them; the image space pass (`grade.wgsl`)
    // turns the finished picture into linear light for the screen.
    out.color = vec4<f32>(shaded * game_scale().x * view_bindings::view.exposure, alpha);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}

// Hair: how much of the hair colour a vertex takes, the hair shader's mask
// (the vertex colour's green). The tint `1 + mask × (2 × HairTint − 1)` is
// already in the vertex colour (`preview::actor`), so the mask is read back
// from it; a grey tint (no change) counts as a full mask.
fn hair_mask(in: VertexOutput) -> f32 {
    var mask = 1.0;
#ifdef VERTEX_COLORS
    let factor = srgb_encode(in.color.rgb) - vec3<f32>(1.0);
    let tint = 2.0 * game_hair_tint().rgb - vec3<f32>(1.0);
    let t2 = dot(tint, tint);
    if (t2 > 1e-4) {
        mask = saturate(dot(factor, tint) / t2);
    }
#endif
    return mask;
}

// The texture arrives as stored (it's sampled without sRGB decoding, so
// filtering averages stored values, as the game's samplers do); Bevy has
// multiplied it by the vertex color, which it holds decoded. The game
// multiplies their stored values, so take the vertex color back out and
// encode it on its own. Without a texture the base is the material's
// (linear) color, encoded.
fn stored_surface(in: VertexOutput, base: vec3<f32>, textured: bool) -> vec3<f32> {
    var unshaded = base;
#ifdef VERTEX_COLORS
    let vertex = in.color.rgb;
    unshaded = select(base / max(vertex, vec3<f32>(1e-6)), vec3<f32>(0.0), vertex <= vec3<f32>(0.0));
#endif
    var surface = unshaded;
    if (!textured) {
        surface = srgb_encode(unshaded);
    }
#ifdef VERTEX_COLORS
    surface *= srgb_encode(vertex);
#endif
    return surface;
}

// Fog, from the game's vertex shaders (`SLS2001.vso`, `NOLIGHT006.vso`):
// the distance is the length of the projected position (x and y scaled by
// the projection, plus the projection's depth: the view depth less the
// game's near plane, `fog_range.z`), and the amount is
// saturate((d - near) / (far - near)) ^ power. The game works it out per
// vertex; the fragment shader calls this at the triangle's corners.
fn fog_amount(p: vec3<f32>) -> f32 {
    let range = fog_range();
    if (range.w < 0.5) {
        return 0.0;
    }
    let v = view_bindings::view.view_from_world * vec4<f32>(p, 1.0);
    let projection = view_bindings::view.clip_from_view;
    let d = length(vec3<f32>(v.x * projection[0][0], v.y * projection[1][1], -v.z - range.z));
    let span = max(range.y - range.x, 1e-4);
    return pow(saturate((d - range.x) / span), fog_color().w);
}

// The reflection pass (`SLS2057`; window reflections `SLS2058`), with its
// vertex shader as recorded from the game: the view direction (toward the
// eye; turned around for windows) reflected about the normal, looked up in
// the cube map along the game's world axes, times the mask (the mask
// texture's red, else the normal map's alpha), the strength, the material's
// opacity and the vertex color, all as stored values, then times (1 - fog).
// The game normalizes the view direction per vertex; this is per pixel.
fn reflection(in: VertexOutput, n: vec3<f32>, to_eye: vec3<f32>, normal_alpha: f32, fog: f32) -> vec3<f32> {
    var v = to_eye;
    if (game_environment().z > 0.5) {
        v = -v;
    }
    let r = 2.0 * dot(n, v) * n - v;
    // Bevy's axes (y up, -z north) back to the game's (x east, y north,
    // z up), which the cube maps are laid out in.
    let along = vec3<f32>(r.x, -r.z, r.y);
    var c = sample_environment(along).rgb;
    var mask = normal_alpha;
#ifdef VERTEX_UVS_A
    if (game_environment().y > 0.5) {
        mask = sample_environment_mask(in.uv).r;
    }
#endif
    c *= mask * game_environment().x * game_environment().w;
#ifdef VERTEX_COLORS
    c *= srgb_encode(in.color.rgb);
#endif
    return c * (1.0 - fog);
}

// The fixed-function alpha test the game turns on per mesh
// (`D3DRS_ALPHAFUNC` against `D3DRS_ALPHAREF` / 255): whether a pixel with
// this alpha is kept. No test keeps everything.
fn alpha_test_passes(a: f32) -> bool {
    let func = u32(game_draw().y + 0.5);
    let r = game_draw().z;
    switch func {
        case 1u: { return false; }
        case 2u: { return a < r; }
        case 3u: { return a == r; }
        case 4u: { return a <= r; }
        case 5u: { return a > r; }
        case 6u: { return a != r; }
        case 7u: { return a >= r; }
        default: { return true; }
    }
}

// Opacity by viewing angle (`NOLIGHT006.vso`): a smooth S-curve from the
// start opacity at the start angle to the stop opacity at the stop angle
// (`falloff` holds the two cosines, then the two opacities).
fn falloff_opacity(cos_angle: f32) -> f32 {
    let f = game_falloff();
    if (abs(f.y - f.x) < 1e-6) {
        return select(f.w, f.z, cos_angle >= f.x);
    }
    let t = saturate((cos_angle - f.x) / (f.y - f.x));
    return f.z + t * t * (3.0 - 2.0 * t) * (f.w - f.z);
}
