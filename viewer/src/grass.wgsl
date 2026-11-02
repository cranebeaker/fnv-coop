// New Vegas's grass, ported from its shaders in
// Data\Shaders\shaderpackage013.sdp: `GRASS2002.vso` (vertex, the variant
// for grass turned to the slope; `GRASS23x002.vso` is the same in vs_3_0)
// and `GRASS2000TMS.pso` (pixel, with transparency multisampling).
//
// Every vertex carries its blade's InstanceData I (see
// `world::grass::GrassInstance`): the blade's position with the terrain's
// normal packed into the fractions, and in w a size step plus brightness.
//
//   N     = 2 * frac(I.xyz) - 1              (not renormalized)
//   s     = 1 + 0.01 * I.w * ScaleMask
//   T     = normalize(|N.x| <= |N.y| and |N.x| <= |N.z| ? (0, -N.z, N.y)
//                                                       : (-N.z, 0, N.x))
//   B     = cross(T, N)
//   p     = B * v.x * s.x + T * v.y * s.y + N * v.z * s.z
//           (without FIT_TO_SLOPE: p = v * s)
//   p    += sin((I.x + I.y) / 128 + WindData.w) * WindData.z
//           * vertex alpha^2 * (WindData.x, WindData.y, 0)
//   world = p + I.xyz
//   b     = 0.75 * frac(I.w) + 0.25
//   light = b * Ambient + b * vertex rgb * saturate(DiffuseDir . N)
//           * DiffuseColor * AddlParams.x
//           (VERTLIT variants light with the model's normal instead)
//   fade  = 1 - saturate((|MVP * I.xyz1| - AlphaParam.z) / AlphaParam.w)
//           (the 4D length of the blade's clip-space position)
//   fog   = saturate((|clip.xyz| - near) / (far - near)) ^ power
//   color = lerp(light * texture, FogColor, fog)
//   alpha = saturate(1.75 * texture alpha) * fade    (alpha to coverage)
//
// Everything is in game units and the game's axes until the position is
// handed to Bevy, and on stored (gamma-encoded) values, as `game_lit.wgsl`
// and `terrain.wgsl`: the result is written as stored values and the image
// space pass decodes the finished picture. The projected depth in the
// fade and fog lengths is the game's: the view depth less its near plane
// (5 units; the factor it's then multiplied by is within 0.002% of 1
// outdoors), the 4D length's w the view depth.

#import bevy_pbr::{
    forward_io::FragmentOutput,
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_view_bindings as view_bindings

struct GrassVertex {
    // The model's vertex as stored, game units.
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(5) color: vec4<f32>,
    @location(8) instance: vec4<f32>,
}

struct GrassVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    // Ambient plus sunlight (the game's oT4 + oT5.xyz).
    @location(1) light: vec3<f32>,
    // x: fog amount, y: fade.
    @location(2) fog_fade: vec2<f32>,
}

struct Grass {
    // DiffuseDir: toward the sun, game axes.
    sun_direction: vec4<f32>,
    // DiffuseColor: the sun's colour.
    sun_color: vec4<f32>,
    // AmbientColor.
    ambient: vec4<f32>,
    // ScaleMask.
    scale_mask: vec4<f32>,
    // WindData: direction (game x, y), size, phase.
    wind: vec4<f32>,
    // AlphaParam.z, .w (fade start and length, game units), AddlParams.x
    // (the grass dimmer), AlphaTestRef (unused with multisampling).
    fade: vec4<f32>,
    // x: 1 to fit the slope; y: 1 to light by the model's normals; z: the
    // luminance shown at full brightness (see `game_lit.wgsl`).
    flags: vec4<f32>,
    // The fog's colour as stored, its power in w.
    fog_color: vec4<f32>,
    // Fog near and far in meters; w is 1 when there's fog.
    fog_range: vec4<f32>,
}

@group(2) @binding(100) var<uniform> grass: Grass;
@group(2) @binding(101) var grass_texture: texture_2d<f32>;
@group(2) @binding(102) var grass_sampler: sampler;

const METERS_PER_UNIT: f32 = 0.0142875;

// Game units and axes (x east, y north, z up) to Bevy's meters.
fn to_bevy(p: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(p.x, p.z, -p.y) * METERS_PER_UNIT;
}

// A position after the game's projection, in game units: x and y scaled
// as the projection does, the depth from the near plane (`fog_range.z`),
// and the view depth as w.
fn projected(world: vec3<f32>) -> vec4<f32> {
    let v = view_bindings::view.view_from_world * vec4<f32>(to_bevy(world), 1.0);
    let projection = view_bindings::view.clip_from_view;
    let depth = -v.z;
    return vec4<f32>(v.x * projection[0][0], v.y * projection[1][1], depth - grass.fog_range.z, depth) / METERS_PER_UNIT;
}

@vertex
fn vertex(v: GrassVertex) -> GrassVertexOutput {
    var out: GrassVertexOutput;
    let i = v.instance;

    let length4 = length(projected(i.xyz));
    let fade = 1.0 - saturate((length4 - grass.fade.x) / grass.fade.y);

    let n = 2.0 * fract(i.xyz) - 1.0;
    let s = 1.0 + 0.01 * i.w * grass.scale_mask.xyz;
    let flat_x = abs(n.x) <= abs(n.y) && abs(n.x) <= abs(n.z);
    let t = normalize(select(vec3<f32>(-n.z, 0.0, n.x), vec3<f32>(0.0, -n.z, n.y), flat_x));
    let b = cross(t, n);
    var p: vec3<f32>;
    var lit_normal = n;
    if (grass.flags.x > 0.5) {
        p = b * (v.position.x * s.x) + t * (v.position.y * s.y) + n * (v.position.z * s.z);
        if (grass.flags.y > 0.5) {
            lit_normal = b * v.normal.x + t * v.normal.y + n * v.normal.z;
        }
    } else {
        p = v.position * s;
        if (grass.flags.y > 0.5) {
            lit_normal = v.normal;
        }
    }
    let phase = (i.x + i.y) * 0.0078125 + grass.wind.w;
    p += sin(phase) * grass.wind.z * (v.color.a * v.color.a) * vec3<f32>(grass.wind.x, grass.wind.y, 0.0);
    let world = p + i.xyz;
    out.position = position_world_to_clip(to_bevy(world));

    let brightness = 0.75 * fract(i.w) + 0.25;
    let sun = saturate(dot(grass.sun_direction.xyz, lit_normal));
    out.light = brightness * grass.ambient.rgb
        + brightness * v.color.rgb * sun * grass.sun_color.rgb * grass.fade.z;

    var fog = 0.0;
    if (grass.fog_range.w > 0.5) {
        let d = length(projected(world).xyz) * METERS_PER_UNIT;
        let span = max(grass.fog_range.y - grass.fog_range.x, 1e-4);
        fog = pow(saturate((d - grass.fog_range.x) / span), grass.fog_color.w);
    }
    out.fog_fade = vec2<f32>(fog, fade);
    out.uv = v.uv;
    return out;
}

@fragment
fn fragment(in: GrassVertexOutput) -> FragmentOutput {
    // The texture is sampled as stored, as the game samples it.
    let texel = textureSample(grass_texture, grass_sampler, in.uv);
    let lit = in.light * texel.rgb;
    let shaded = mix(lit, grass.fog_color.rgb, in.fog_fade.x);
    var out: FragmentOutput;
    out.color = vec4<f32>(
        shaded * grass.flags.z * view_bindings::view.exposure,
        saturate(1.75 * texel.a) * in.fog_fade.y,
    );
    return out;
}
