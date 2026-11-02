// New Vegas's tree leaves, ported from its shaders in
// Data\Shaders\shaderpackage013.sdp: `STLEAF001.vso` (vertex; the recorded
// Goodsprings frame used exactly this code) and `STLEAF2001TMS.pso` (pixel,
// with transparency multisampling).
//
// Every leaf is four vertices at the leaf's position. Each vertex's
// BLENDINDICES b holds: b.x the wind weight, b.y the wind matrix × 4, b.z the
// LeafBase entry (corner + 4 × card) plus the leaf's brightness as a
// fraction, b.w the level's card scale. With R, U the camera's right and up
// in the world's axes (BillboardRight, BillboardUp):
//
//   rustle = sin(π(b.z/48 + RustleParams.y)) × RustleParams.z × .x
//   rock   = sin(π(b.z/48 + RockParams.y))   × RockParams.z   × .x
//   c      = b.w × LeafBase[floor(b.z)]            (0, y, z)
//   y', z' = c turned by `rock` about x
//   off    = y' R + z' U, its x and y turned by `rustle` about z
//   p      = position + off; p = lerp(p, WindMatrices[b.y] p, b.x)
//   light  = saturate(normalize(normalize(c) × LeafLighting.y + normal) ·
//            LightVector) × DiffColor × SunDimmer.x + AmbientColor
//   oT1    = (frac(b.z) × light.rgb, light.a)
//   colour = lerp(texture × oT1, FogColor, fog); alpha = 2 × texture alpha
//
// The game's angles go through the shader compiler's range reduction (the
// `frc` steps), kept as they are. The cards are offset in the tree's own
// space by the world's camera axes, as the game does (no tree in the
// game's files is turned). Alpha to coverage: on this PC's NVIDIA card the
// game's `ATOC` with the alpha test on turns the test into coverage (the
// alpha reference, which the levels' cross-fade sets, then has no effect).

#import bevy_pbr::{
    forward_io::FragmentOutput,
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_view_bindings as view_bindings

struct LeafVertex {
    @builtin(instance_index) instance_index: u32,
    // The leaf's position and normal in the tree's space (game units).
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(8) blend: vec4<f32>,
}

struct LeafOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) light: vec4<f32>,
    @location(2) fog: f32,
}

struct Leaves {
    // AmbientColor.
    ambient: vec4<f32>,
    // DiffColor: the sun's light with the sunlight dimmer.
    diffuse: vec4<f32>,
    // Toward the sun, the world's (game) axes.
    sun_direction: vec4<f32>,
    // x: SunDimmer.x (the image space's tree dimmer); y: the luminance
    // shown at full brightness (see `game_lit.wgsl`).
    dimmer: vec4<f32>,
    // BillboardRight, BillboardUp: the camera's axes in the game's.
    billboard_right: vec4<f32>,
    billboard_up: vec4<f32>,
    // RockParams, RustleParams, LeafLighting.
    rock: vec4<f32>,
    rustle: vec4<f32>,
    lighting: vec4<f32>,
    // The fog's colour as stored (power in w); near and far in meters, the
    // near clip in z, 1 in w when there's fog.
    fog_color: vec4<f32>,
    fog_range: vec4<f32>,
    // WindMatrices: four matrices, rows.
    wind: array<vec4<f32>, 16>,
    // LeafBase.
    leaf_base: array<vec4<f32>, 48>,
}

@group(2) @binding(100) var<uniform> leaves: Leaves;
@group(2) @binding(101) var leaf_texture: texture_2d<f32>;
@group(2) @binding(102) var leaf_sampler: sampler;

// The shader compiler's range reduction before `sincos`.
fn reduced(x: f32, k: f32) -> f32 {
    return fract(x * k + 0.5) * 6.28318548 - 3.14159274;
}

// A Bevy world position's length after the game's projection, in meters:
// x and y scaled as the projection does, the depth from the near plane (as
// `grass.wgsl`).
fn projected_length(world: vec3<f32>) -> f32 {
    let v = view_bindings::view.view_from_world * vec4<f32>(world, 1.0);
    let projection = view_bindings::view.clip_from_view;
    let depth = -v.z;
    let p = vec3<f32>(v.x * projection[0][0], v.y * projection[1][1], depth - leaves.fog_range.z);
    return length(p);
}

@vertex
fn vertex(v: LeafVertex) -> LeafOutput {
    var out: LeafOutput;
    let b = v.blend;
    let r = leaves.billboard_right;
    let u = leaves.billboard_up;

    // Rustle: a turn about the view axis.
    let rustle_phase = sin(reduced(b.z * 0.020833334 + leaves.rustle.y, 0.49999958));
    let rustle = reduced(rustle_phase * leaves.rustle.z * leaves.rustle.x, 0.159154937);
    let rc = cos(rustle);
    let rs = sin(rustle);
    let row0 = vec3<f32>(rc, -rs, 0.0);
    let row1 = vec3<f32>(rs, rc, 0.0);

    // Rock: a turn about the card's own x.
    let rock_phase = sin(reduced(b.z * 0.020833334 + leaves.rock.y, 0.49999958));
    let rock = reduced(rock_phase * leaves.rock.z * leaves.rock.x, 0.159154937);
    let kc = cos(rock);
    let ks = sin(rock);

    let corner_index = i32(round(b.z - fract(b.z)));
    let matrix_index = i32(round(b.y - fract(b.y)));
    let c = b.w * leaves.leaf_base[corner_index];
    let y = dot(vec3<f32>(0.0, kc, -ks), c.xyz);
    let z = dot(vec3<f32>(0.0, ks, kc), c.xyz);
    let offset = vec4<f32>(
        y * dot(row0, r.xyz) + z * dot(row0, u.xyz),
        y * dot(row1, r.xyz) + z * dot(row1, u.xyz),
        y * r.z + z * u.z,
        y * r.w + z * u.w,
    );
    let p = offset + vec4<f32>(v.position, 1.0);
    let swayed = vec4<f32>(
        dot(leaves.wind[matrix_index], p),
        dot(leaves.wind[matrix_index + 1], p),
        dot(leaves.wind[matrix_index + 2], p),
        dot(leaves.wind[matrix_index + 3], p),
    );
    let local = mix(p, swayed, b.x);

    let world_from_local = mesh_functions::get_world_from_local(v.instance_index);
    let world = (world_from_local * vec4<f32>(local.xyz, 1.0)).xyz;
    out.position = position_world_to_clip(world);

    // The light in the tree's own space (`00bb1960`): the world's (game)
    // direction turned back by the tree's rotation.
    let s = leaves.sun_direction.xyz;
    let bevy_sun = vec3<f32>(s.x, s.z, -s.y);
    // The mesh's matrix takes the tree's space (game axes) to Bevy's world:
    // its transpose takes the world's direction back (up to the scale).
    let m = mat3x3<f32>(world_from_local[0].xyz, world_from_local[1].xyz, world_from_local[2].xyz);
    let light_dir = normalize(transpose(m) * bevy_sun);
    let n = normalize(normalize(c.xyz) * leaves.lighting.y + v.normal);
    let ndl = clamp(dot(n, light_dir), 0.0, 1.0);
    let light = ndl * leaves.diffuse * leaves.dimmer.x + leaves.ambient;
    out.light = vec4<f32>(fract(b.z) * light.rgb, light.a);

    var fog = 0.0;
    if (leaves.fog_range.w > 0.5) {
        // Meters, as the fog's range.
        let d = projected_length(world);
        let span = max(leaves.fog_range.y - leaves.fog_range.x, 1e-4);
        fog = pow(saturate((d - leaves.fog_range.x) / span), leaves.fog_color.w);
    }
    out.fog = fog;
    out.uv = v.uv;
    return out;
}

@fragment
fn fragment(in: LeafOutput) -> FragmentOutput {
    // The texture is sampled as stored, as the game samples it.
    let texel = textureSample(leaf_texture, leaf_sampler, in.uv);
    let lit = texel.rgb * in.light.rgb;
    let shaded = mix(lit, leaves.fog_color.rgb, in.fog);
    var out: FragmentOutput;
    out.color = vec4<f32>(
        shaded * leaves.dimmer.y * view_bindings::view.exposure,
        saturate(texel.a + texel.a),
    );
    return out;
}
