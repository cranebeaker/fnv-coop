// New Vegas's water, ported from its water shaders in
// Data\Shaders\shaderpackage013.sdp: `WATER000.vso` (vertex) with
// `WATER000.pso` (outdoors, reflection + refraction + depth), `WATER001`
// (outdoors without reflection), `WATER008` (inside) and `WATER009` (inside
// without reflection); without depth `WATER004`, `005`, `012`, `013`; and
// distant water `WATER033`. Shader defs: REFLECTIONS, INTERIOR, DEPTH,
// LOD. Which one the game picks, and where every constant comes from, is in
// `world::water` and `nv-re\findings\water.md`; per pixel, in game units
// (P the surface, E the eye, V = E - P):
//
//   D      = depth map: the path under water and the vertical depth below
//            the surface, both / the water's fog far distance (0..1)
//   fall   = saturate((D.y - start) / (end - start))       depth falloff
//   fade   = saturate(2 - |V.xy| / 4096)                    ripples fade out
//   N      = normalize(fade fall n.x, fade fall n.y, fall n.z + 1)
//   P'     = P + N.xy * D.y fall lerp(4, distortion, saturate(|V.xy| / 5000))
//            (in the model's own axes)
//   refl   = reflection at P' on screen; refr = the scene copy at P'
//   refrU  = (refr - fogB FogColor) / (1.0001 - fogB)       the fog taken out
//   waterC = lerp(shallow, deep, Dy)  (x N.L with L = (s.x, 4 s.y, s.z)
//            outdoors)
//   murk   = fall fogAmount saturate((path - near) / (far - near))
//   body   = lerp(refrU, waterC, murk)
//   F      = (1 - V.N)^5;  fres = F + (1 - F) F0
//   ... then reflection or the reflection colour by Dx fres, the sun's two
//   highlights outdoors, and the scene's fog (see `fragment`).
//
// Everything is on stored (gamma-encoded) values, as the game's frame
// buffer holds them: the scene copy and the reflection come in as the
// other passes wrote them and are divided back to stored values; the
// result is written as stored values for the image space pass.
//
// Differences from the game: the ripples' normal map is worked out here
// per pixel (the game renders it into a 256 x 256 texture every frame: the
// three scrolled layers summed, then a Sobel filter; the same sums at the
// same steps); the depth map comes from the scene's own depth (full
// resolution, not 512 x 512), with the game's two values and 8-bit steps.

#import bevy_pbr::{
    mesh_functions,
    view_transformations::{position_world_to_clip, position_ndc_to_world, frag_coord_to_ndc},
}
#import bevy_pbr::mesh_view_bindings as view_bindings

struct WaterParams {
    shallow: vec4<f32>,
    deep: vec4<f32>,
    reflection: vec4<f32>,
    // FresnelRI: fresnel F0, the scene fog's power, shininess, reflection
    // multiplier.
    fresnel_ri: vec4<f32>,
    // VarAmounts: sun power, reflectivity, opacity, distortion.
    var_amounts: vec4<f32>,
    // FogParam: scene fog far, far - near, water fog far, far - near (game
    // units).
    fog_param: vec4<f32>,
    // FogColor: the scene fog's colour, and the water's fog amount.
    fog_color: vec4<f32>,
    // DepthFalloff: start, end.
    depth_falloff: vec4<f32>,
    // SunDir: toward the sun (game axes), its visibility.
    sun_dir: vec4<f32>,
    // SunColor.
    sun_color: vec4<f32>,
    // The ripples: normal strength, world units per noise texture, 1 to
    // follow the model's texture coordinates, unused.
    noise: vec4<f32>,
    // Each layer's scroll in noise texture lengths a second: (u1, v1, u2,
    // v2), (u3, v3, -, -).
    layer_motion_a: vec4<f32>,
    layer_motion_b: vec4<f32>,
    // Layer amplitudes and texture coordinate scales (1, 2, 3, -).
    amplitudes: vec4<f32>,
    uv_scales: vec4<f32>,
    // The model's own x and y axes in the world (x.x, x.y, y.x, y.y).
    axes: vec4<f32>,
    // x: the surface's height (game units); y: the brightness the lit
    // shaders scale by (`GameLighting.scale.x`).
    surface: vec4<f32>,
}

@group(2) @binding(0) var<uniform> water: WaterParams;
@group(2) @binding(1) var noise_texture: texture_2d<f32>;
@group(2) @binding(2) var noise_sampler: sampler;
@group(2) @binding(3) var reflection_texture: texture_2d<f32>;
@group(2) @binding(4) var reflection_sampler: sampler;
@group(2) @binding(5) var depth_texture: texture_depth_multisampled_2d;

// Game units per meter (64 units to 0.9144 m).
const UNITS_PER_METER: f32 = 69.99125;

fn to_game(b: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(b.x, -b.z, b.y) * UNITS_PER_METER;
}

fn to_bevy(g: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(g.x, g.z, -g.y) / UNITS_PER_METER;
}

struct WaterVertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct WaterVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vertex(vertex: WaterVertex) -> WaterVertexOutput {
    var out: WaterVertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    out.uv = vertex.uv;
    return out;
}

// The ripples' height at a noise texture coordinate (`ISNOISESCROLLANDBLEND`):
// layer 1 from the noise's blue, 2 from its green, 3 from its red, each at
// its scale and scrolled place, around one half by its amplitude.
fn ripple_height(uv: vec2<f32>, o1: vec2<f32>, o2: vec2<f32>, o3: vec2<f32>) -> f32 {
    let s = water.uv_scales;
    let a = water.amplitudes;
    let l1 = textureSampleLevel(noise_texture, noise_sampler, uv * s.x + o1, 0.0).b;
    let l2 = textureSampleLevel(noise_texture, noise_sampler, uv * s.y + o2, 0.0).g;
    let l3 = textureSampleLevel(noise_texture, noise_sampler, uv * s.z + o3, 0.0).r;
    let sum = a.x * (2.0 * l1 - 1.0) + a.y * (2.0 * l2 - 1.0) + a.z * (2.0 * l3 - 1.0);
    return saturate(0.5 + 0.5 * sum);
}

// The ripples' normal (`ISNOISENORMALMAP`): a 3 x 3 Sobel filter of the
// height one 1/256 step around, `normalize(-k Gx, k Gy, 1)` with k the
// water's noise strength; the game stores it as n / 2 + 1/2 and the water
// shader reads it back as 2 stored - 1.
fn ripple_normal(uv: vec2<f32>) -> vec3<f32> {
    let t = view_bindings::globals.time;
    let o1 = fract(water.layer_motion_a.xy * t);
    let o2 = fract(water.layer_motion_a.zw * t);
    let o3 = fract(water.layer_motion_b.xy * t);
    let d = 1.0 / 256.0;
    let tl = ripple_height(uv + vec2<f32>(-d, -d), o1, o2, o3);
    let l = ripple_height(uv + vec2<f32>(-d, 0.0), o1, o2, o3);
    let bl = ripple_height(uv + vec2<f32>(-d, d), o1, o2, o3);
    let tc = ripple_height(uv + vec2<f32>(0.0, -d), o1, o2, o3);
    let bc = ripple_height(uv + vec2<f32>(0.0, d), o1, o2, o3);
    let tr = ripple_height(uv + vec2<f32>(d, -d), o1, o2, o3);
    let r = ripple_height(uv + vec2<f32>(d, 0.0), o1, o2, o3);
    let br = ripple_height(uv + vec2<f32>(d, d), o1, o2, o3);
    let gx = (tr + 2.0 * r + br) - (tl + 2.0 * l + bl);
    let gy = (bl + 2.0 * bc + br) - (tl + 2.0 * tc + tr);
    let k = water.noise.x;
    return normalize(vec3<f32>(-k * gx, k * gy, 1.0));
}

// The depth map at this pixel (the game's depth pass, `004ec800`, with the
// `WATER_DEPTH` shaders): what's drawn under the water here, as (the part
// of the eye's ray under the water, the depth below the surface), each /
// the water's fog far distance, kept between 0 and 1 and in 8-bit steps
// (an A8R8G8B8 target). Only what's below the surface + 3 units
// (`fRefractionWaterPlaneBias`) is drawn into it; where nothing is, the
// game fills it first with a value not traced (1 taken here, a guess).
fn depth_map(frag: vec4<f32>, eye: vec3<f32>) -> vec2<f32> {
    let z = textureLoad(depth_texture, vec2<i32>(frag.xy), 0);
    if (z <= 0.0) {
        return vec2<f32>(1.0);
    }
    let ndc = vec3<f32>(frag_coord_to_ndc(frag).xy, z);
    let b = to_game(position_ndc_to_world(ndc));
    let h = water.surface.x;
    if (b.z > h + 3.0) {
        return vec2<f32>(1.0);
    }
    let far = max(water.fog_param.z, 1e-3);
    let path = (h - b.z) / max(eye.z - b.z, 1e-3) * length(b - eye) / far;
    let depth = abs(h - b.z) / far;
    return round(saturate(vec2<f32>(path, depth)) * 255.0) / 255.0;
}

// The scene's fog at a distance: (1 - saturate((far - d) / (far - near)))
// ^ power, as the lit shaders fog.
fn scene_fog(d: f32) -> f32 {
    let span = max(water.fog_param.y, 1e-3);
    return pow(1.0 - saturate((water.fog_param.x - d) / span), water.fresnel_ri.y);
}

// A point's place on screen (0..1, v down), with the main camera.
fn screen_of(p: vec3<f32>) -> vec2<f32> {
    let clip = view_bindings::view.clip_from_world * vec4<f32>(to_bevy(p), 1.0);
    return vec2<f32>(0.5 + 0.5 * clip.x / clip.w, 0.5 - 0.5 * clip.y / clip.w);
}

@fragment
fn fragment(in: WaterVertexOutput) -> @location(0) vec4<f32> {
#ifdef LOD
    return distant_water(in);
#else
    return near_water(in);
#endif
}

// Distant water (`WATER033`): flat, without ripples, refraction or depth;
// its colour and opacity by flat distance (the water type's opacity up to
// 4096 units, opaque from 8192), the reflection at the point itself, the
// sun's highlight along the reflected view (the fixed one is a constant
// here: the shader's 2.4e-9), the scene's fog. Blended by its opacity.
fn distant_water(in: WaterVertexOutput) -> vec4<f32> {
    let brightness = water.surface.y * view_bindings::view.exposure;
    let p = to_game(in.world_position.xyz);
    let eye = to_game(view_bindings::view.world_position);
    let v = eye - p;
    let dxy = length(v.xy);
    let dist = length(v);
    let to_eye = v / max(dist, 1e-6);
    let ss = screen_of(p);
    let refl = textureSampleLevel(reflection_texture, reflection_sampler, vec2<f32>(1.0 - ss.x, ss.y), 0.0).rgb / brightness;
    let reflected = mix(water.reflection.rgb, refl, water.var_amounts.y) * water.fresnel_ri.w;
    let fade = saturate(2.0 - dxy / 4096.0);
    let opacity = water.var_amounts.z;
    let a = saturate((1.0 - fade) * (1.0 - opacity) + opacity);
    let water_color = mix(water.shallow.rgb, water.deep.rgb, a);
    let f = pow(1.0 - saturate(to_eye.z), 5.0);
    let fresnel = f + (1.0 - f) * water.fresnel_ri.x;
    let reflected_view = vec3<f32>(-to_eye.x, -to_eye.y, to_eye.z);
    let spec = pow(saturate(dot(reflected_view, water.sun_dir.xyz)), water.var_amounts.x) + 2.4064943e-9;
    let color = water_color + fresnel * (reflected - water_color)
        + spec * water.sun_color.rgb * water.sun_dir.w;
    let shaded = mix(color, water.fog_color.rgb, scene_fog(dist));
    return vec4<f32>(shaded * brightness, a);
}

fn near_water(in: WaterVertexOutput) -> vec4<f32> {
    // What the other passes wrote is stored values times this.
    let brightness = water.surface.y * view_bindings::view.exposure;
    let p = to_game(in.world_position.xyz);
    let eye = to_game(view_bindings::view.world_position);
    let v = eye - p;
    let dxy = length(v.xy);
    let dist = length(v);
    let to_eye = v / max(dist, 1e-6);

    // Without depth (`WATER004`–`007`, `012`–`015`) the shaders take no
    // depth map: the falloff is 1 and the bend isn't scaled by depth.
    var dm = vec2<f32>(1.0);
    var fall = 1.0;
#ifdef DEPTH
    dm = depth_map(in.position, eye);
    let falloff_span = water.depth_falloff.y - water.depth_falloff.x;
    fall = saturate((dm.y - water.depth_falloff.x) / falloff_span);
#endif
    let fade = saturate(2.0 - dxy / 4096.0);

    // The ripples, from the world position (or the model's texture
    // coordinates, `ObjectUV`) over the noise tile size (`TexScale`).
    var uv_noise = p.xy / water.noise.y;
    if (water.noise.z > 0.5) {
        uv_noise = in.uv * 1000.0 / water.noise.y;
    }
    let n = ripple_normal(uv_noise);
    let normal = normalize(vec3<f32>(fade * fall * n.x, fade * fall * n.y, fall * n.z + 1.0));
    let dx = saturate(mix(1.0, dm.x, fade));
    let dy = saturate(mix(1.0, dm.y, fade));

    // Reflection and refraction looked up where the ripples bend the
    // surface to: the offset in the model's own axes.
    var k = mix(4.0, water.var_amounts.w, saturate(dxy / 5000.0));
#ifdef DEPTH
    k *= dm.y * fall;
#endif
    let offset = (normal.x * k) * water.axes.xy + (normal.y * k) * water.axes.zw;
    let ss = screen_of(p + vec3<f32>(offset, 0.0));
    let refr = textureSampleLevel(
        view_bindings::view_transmission_texture,
        view_bindings::view_transmission_sampler,
        ss,
        0.0,
    ).rgb / brightness;

    let fog_surface = scene_fog(dist);
    let f = pow(1.0 - saturate(dot(to_eye, normal)), 5.0);
    let fresnel = f + (1.0 - f) * water.fresnel_ri.x;
    // Outdoors the water's colour is lit by the sun, its direction's
    // north-south part counted four times (all the sun variants do this).
    var lit = 1.0;
    let sun = water.sun_dir.xyz;
#ifndef INTERIOR
    lit = saturate(dot(normalize(vec3<f32>(sun.x, 4.0 * sun.y, sun.z)), normal));
#endif
#ifdef REFLECTIONS
    // The reflection camera's picture is mirrored left to right (see
    // `water.rs`).
    let refl = textureSampleLevel(reflection_texture, reflection_sampler, vec2<f32>(1.0 - ss.x, ss.y), 0.0).rgb / brightness;
    let reflected = mix(water.reflection.rgb, refl, water.var_amounts.y) * water.fresnel_ri.w;
#endif

#ifdef DEPTH
    // The scene's fog taken back out of the copy (at the surface's
    // distance + the depth value: the shader's).
    let fog_bottom = scene_fog(dist + dy);
    let refr_unfogged = (refr - fog_bottom * water.fog_color.rgb) / (1.0001 - fog_bottom);
    let water_color = mix(water.shallow.rgb, water.deep.rgb, dy) * lit;
    let murk = fall * water.fog_color.w
        * (1.0 - saturate(water.fog_param.z * (1.0 - dx) / max(water.fog_param.w, 1e-3)));
    let body = mix(refr_unfogged, water_color, murk);
#ifdef REFLECTIONS
#ifdef INTERIOR
    var color = mix(mix(body, water_color, dy), reflected, dx * fresnel);
#else
    var color = mix(body, reflected, dx * fresnel);
#endif
#else
    let top = mix(water_color, water.reflection.rgb, dx * fresnel);
    var color = mix(body, top, dy);
#endif
#else
    // Without depth: the copy as it is, toward the water's colour by the
    // fresnel term, and the water's colour by its opacity (which fades to
    // full between 4096 and 8192 units away).
    let body = mix(refr, mix(water.shallow.rgb, water.deep.rgb, fresnel) * lit, fresnel);
    let a = saturate((1.0 - fade) * (1.0 - water.var_amounts.z) + water.var_amounts.z);
    let opaque_color = mix(water.shallow.rgb, water.deep.rgb, a);
#ifdef REFLECTIONS
    var color = mix(mix(body, opaque_color, a), reflected, fresnel);
#else
    var color = mix(body, mix(opaque_color, water.reflection.rgb, fresnel), a);
#endif
#endif

#ifndef INTERIOR
    // The sun's highlights: one in a fixed direction and one along the
    // reflected view, × the sun's colour and visibility.
    let reflected_view = 2.0 * dot(to_eye, normal) * normal - to_eye;
    let fixed = pow(saturate(-0.57 * normal.x + 0.82 * normal.z), 100.0);
    let along = pow(saturate(dot(reflected_view, sun)), water.var_amounts.x);
    color += (fixed + along) * water.sun_color.rgb * water.sun_dir.w;
#ifndef REFLECTIONS
    color = saturate(color);
#endif
#endif

    let shaded = mix(color, water.fog_color.rgb, fog_surface);
    return vec4<f32>(shaded * brightness, 1.0);
}
