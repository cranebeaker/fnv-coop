// New Vegas's particles, ported from its shaders in
// Data\Shaders\shaderpackage013.sdp: `NOLIGHT017.vso` (with an atlas;
// `NOLIGHT016.vso` without, the same but for the texture coordinates) and
// `NOLIGHTTEXVC.pso` / `NOLIGHTTEXVCPMA.pso` (pixel). The quads come built
// on the CPU (`world::particles::System::quads`), in world space, the
// texture coordinates already moved into the atlas piece:
//
//   fog   = (1 - saturate((far - |clip.xyz|) / (far - near))) ^ power
//           (|clip.xyz| the projected position's length, its depth from the
//           near plane)
//   c     = texture * vertex colour * MaterialColor
//   NOLIGHTTEXVC:     rgb = lerp(c.rgb, FogColor, fog)    (Toggles 0, 0)
//                         = c.rgb * (1 - fog)            (added: Toggles.x)
//                     alpha = c.a
//   NOLIGHTTEXVCPMA:  rgb = c.rgb * (1 - fog) * c.a, alpha 1, added One/One
//
// On stored (gamma-encoded) values, as `game_lit.wgsl`: the picture holds
// stored values and the image space pass decodes it.

#import bevy_pbr::{
    forward_io::FragmentOutput,
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_view_bindings as view_bindings

struct ParticleVertex {
    // World position, Bevy's meters, relative to the entity's place.
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(5) color: vec4<f32>,
}

struct ParticleVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) fog: f32,
}

struct Particles {
    // MaterialColor: RGB and the material's alpha.
    color: vec4<f32>,
    // The fog's colour as stored, its power in w.
    fog_color: vec4<f32>,
    // Fog near and far in meters, the near clip plane in meters; w is 1
    // when there's fog.
    fog_range: vec4<f32>,
    // x: how the colour reaches the picture (0 blended, 1 added × alpha
    // (PMA), 2 added, 3 opaque); y: the alpha test (as `game_lit.wgsl`), z
    // its reference; w: the luminance shown at full brightness.
    mode: vec4<f32>,
    // The entity's place (meters): vertices are relative to it.
    origin: vec4<f32>,
}

@group(2) @binding(100) var<uniform> particles: Particles;
@group(2) @binding(101) var particle_texture: texture_2d<f32>;
@group(2) @binding(102) var particle_sampler: sampler;

const METERS_PER_UNIT: f32 = 0.0142875;

@vertex
fn vertex(v: ParticleVertex) -> ParticleVertexOutput {
    var out: ParticleVertexOutput;
    let world = v.position + particles.origin.xyz;
    out.position = position_world_to_clip(world);
    var fog = 0.0;
    if (particles.fog_range.w > 0.5) {
        // The projected position's length: x and y as the projection
        // scales them, the depth from the near plane.
        let view = view_bindings::view.view_from_world * vec4<f32>(world, 1.0);
        let projection = view_bindings::view.clip_from_view;
        let depth = -view.z;
        let p = vec3<f32>(view.x * projection[0][0], view.y * projection[1][1], depth - particles.fog_range.z);
        let d = length(p);
        let span = max(particles.fog_range.y - particles.fog_range.x, 1e-4);
        fog = pow(1.0 - saturate((particles.fog_range.y - d) / span), particles.fog_color.w);
    }
    out.fog = fog;
    out.uv = v.uv;
    out.color = v.color;
    return out;
}

fn alpha_test_passes(a: f32) -> bool {
    let func = u32(particles.mode.y + 0.5);
    let r = particles.mode.z;
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

@fragment
fn fragment(in: ParticleVertexOutput) -> FragmentOutput {
    // Sampled as stored, as the game samples it.
    let texel = textureSample(particle_texture, particle_sampler, in.uv);
    let c = texel * in.color * particles.color;
    let mode = u32(particles.mode.x + 0.5);
    let scale = particles.mode.w * view_bindings::view.exposure;
    var out: FragmentOutput;
    switch mode {
        case 1u: {
            // NOLIGHTTEXVCPMA: the output alpha is 1, which the test sees.
            if (!alpha_test_passes(1.0)) {
                discard;
            }
            out.color = vec4<f32>(c.rgb * (1.0 - in.fog) * c.a * scale, 0.0);
        }
        case 2u: {
            if (!alpha_test_passes(c.a)) {
                discard;
            }
            out.color = vec4<f32>(c.rgb * (1.0 - in.fog) * scale, 0.0);
        }
        case 3u: {
            if (!alpha_test_passes(c.a)) {
                discard;
            }
            out.color = vec4<f32>(mix(c.rgb, particles.fog_color.rgb, in.fog) * scale, 1.0);
        }
        default: {
            if (!alpha_test_passes(c.a)) {
                discard;
            }
            // Blended source-alpha / inverse source-alpha, through Bevy's
            // premultiplied blend state.
            let rgb = mix(c.rgb, particles.fog_color.rgb, in.fog) * scale;
            out.color = vec4<f32>(rgb * c.a, c.a);
        }
    }
    return out;
}
