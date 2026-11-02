// New Vegas's tree branches: the vertex shader `STB2004.vso` from
// Data\Shaders\shaderpackage013.sdp (the branch shader's first pass:
// ambient + the sun + texture, with fog) and the lit pixel shader of that
// pass (`SLS2001.pso`, the one its outputs fit).
//
// Each vertex's BLENDINDICES b: b.x the wind weight, b.y the wind matrix ×
// 4, b.z the branch's brightness (its vertex colour).
//
//   p      = lerp(position, WindMatrices[b.y] position, b.x)
//   L      = normalize(T·L, B·L, N·L)   (the light in the vertex's frame)
//   n      = normalize(2 × NormalMap − 1)
//   colour = max(Ambient + Sun × saturate(n · L), 0) × BaseMap × b.z
//   colour = lerp(colour, FogColor, fog)
//
// No branch was drawn in the Goodsprings recording (the shrubs in view were
// past their branches' distance), so which of the branch shader's passes
// the game draws (point lights, highlights) isn't confirmed: only this one
// is drawn.

#import bevy_pbr::{
    forward_io::FragmentOutput,
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_view_bindings as view_bindings

struct BranchVertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(8) blend: vec4<f32>,
    // TANGENT (around the branch) and BINORMAL.
    @location(9) tangent: vec3<f32>,
    @location(10) binormal: vec3<f32>,
}

struct BranchOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) light: vec3<f32>,
    @location(2) brightness: f32,
    @location(3) fog: f32,
}

struct Branches {
    ambient: vec4<f32>,
    // PSLightColor: the sun's light with the sunlight dimmer.
    sun_color: vec4<f32>,
    // Toward the sun, the world's (game) axes.
    sun_direction: vec4<f32>,
    // x: the luminance shown at full brightness.
    flags: vec4<f32>,
    fog_color: vec4<f32>,
    fog_range: vec4<f32>,
    wind: array<vec4<f32>, 16>,
}

@group(2) @binding(100) var<uniform> branches: Branches;
@group(2) @binding(101) var base_map: texture_2d<f32>;
@group(2) @binding(102) var base_sampler: sampler;
@group(2) @binding(103) var normal_map: texture_2d<f32>;
@group(2) @binding(104) var normal_sampler: sampler;

fn projected_length(world: vec3<f32>) -> f32 {
    let v = view_bindings::view.view_from_world * vec4<f32>(world, 1.0);
    let projection = view_bindings::view.clip_from_view;
    let depth = -v.z;
    let p = vec3<f32>(v.x * projection[0][0], v.y * projection[1][1], depth - branches.fog_range.z);
    return length(p);
}

@vertex
fn vertex(v: BranchVertex) -> BranchOutput {
    var out: BranchOutput;
    let b = v.blend;
    let i = i32(round(b.y - fract(b.y)));
    let p0 = vec4<f32>(v.position, 1.0);
    let swayed = vec4<f32>(
        dot(branches.wind[i], p0),
        dot(branches.wind[i + 1], p0),
        dot(branches.wind[i + 2], p0),
        dot(branches.wind[i + 3], p0),
    );
    let p = (swayed - p0) * b.x + p0;

    let world_from_local = mesh_functions::get_world_from_local(v.instance_index);
    let world = (world_from_local * vec4<f32>(p.xyz, 1.0)).xyz;
    out.position = position_world_to_clip(world);

    // `LightData`: the sun's direction in the tree's space.
    let s = branches.sun_direction.xyz;
    let m = mat3x3<f32>(world_from_local[0].xyz, world_from_local[1].xyz, world_from_local[2].xyz);
    let l = normalize(transpose(m) * vec3<f32>(s.x, s.z, -s.y));
    out.light = normalize(vec3<f32>(dot(v.tangent, l), dot(v.binormal, l), dot(v.normal, l)));
    out.brightness = b.z;

    var fog = 0.0;
    if (branches.fog_range.w > 0.5) {
        // Meters, as the fog's range.
        let d = projected_length(world);
        let span = max(branches.fog_range.y - branches.fog_range.x, 1e-4);
        fog = pow(saturate((d - branches.fog_range.x) / span), branches.fog_color.w);
    }
    out.fog = fog;
    out.uv = v.uv;
    return out;
}

@fragment
fn fragment(in: BranchOutput) -> FragmentOutput {
    let base = textureSample(base_map, base_sampler, in.uv);
    let stored = textureSample(normal_map, normal_sampler, in.uv);
    let n = normalize((stored.xyz - 0.5) * 2.0);
    let ndl = saturate(dot(n, in.light));
    let light = max(branches.ambient.rgb + branches.sun_color.rgb * ndl, vec3<f32>(0.0));
    let lit = light * base.rgb * in.brightness;
    let shaded = mix(lit, branches.fog_color.rgb, in.fog);
    var out: FragmentOutput;
    out.color = vec4<f32>(
        shaded * branches.flags.x * view_bindings::view.exposure,
        // Alpha is texture alpha × AmbientColor.w (1); drawn opaque.
        base.a,
    );
    return out;
}
