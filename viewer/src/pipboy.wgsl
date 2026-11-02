// The Pip-Boy screen's effect (`pipboy.rs`), the game's ISIFSCANBLEND.pso
// from shader package 13, on stored values:
//
//   y = t.y + Params.y, less Params.z (1) past it   (the picture rolled)
//   glow = mean of MaskColors at the 3 x 3 taps (x, y) +- Offsets
//   c = glow x Params.x x Tint + Colors(x, y)
//   c.rgb += Distort(0.5, t.y x DistortParams.x - DistortParams.y + 1)
//            x DistortParams.z x Tint x 0.35       (when DistortParams.x > 0)
//   c *= Scanlines(t x Params.w)                    (when Params.w > 0)
//
// Both MaskColors and Colors are the menus' picture here. The result is
// what the game's 8-bit render target would hold (clamped to 0..1), handed
// on decoded to linear light, since the arm's material encodes what it
// samples back to stored values (`game_lit.wgsl`).

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Screen {
    // Params: blur intensity, scroll, 1, scanline frequency.
    params: vec4<f32>,
    // DistortParams: vertical scale (0: no band), progress, horizontal
    // scale.
    distort: vec4<f32>,
    // Tint: the Pip-Boy colour.
    tint: vec4<f32>,
    // Offsets: the blur radius in texture coordinates.
    offsets: vec4<f32>,
}

@group(2) @binding(0) var<uniform> u: Screen;
@group(2) @binding(1) var picture: texture_2d<f32>;
@group(2) @binding(2) var picture_sampler: sampler;
@group(2) @binding(3) var scanlines: texture_2d<f32>;
@group(2) @binding(4) var scanlines_sampler: sampler;
@group(2) @binding(5) var band: texture_2d<f32>;
@group(2) @binding(6) var band_sampler: sampler;

fn decode(c: f32) -> f32 {
    if c <= 0.04045 {
        return c / 12.92;
    }
    return pow((c + 0.055) / 1.055, 2.4);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let t = mesh.uv;
    var y = t.y + u.params.y;
    if u.params.z - y < 0.0 {
        y = y - u.params.z;
    }
    let p = vec2<f32>(t.x, y);
    let o = u.offsets.xy;
    var glow = vec4<f32>(0.0);
    for (var i = -1; i <= 1; i = i + 1) {
        for (var j = -1; j <= 1; j = j + 1) {
            let at = p + vec2<f32>(f32(i) * o.x, f32(j) * o.y);
            glow = glow + textureSampleLevel(picture, picture_sampler, at, 0.0) * (1.0 / 9.0);
        }
    }
    let base = textureSampleLevel(picture, picture_sampler, p, 0.0);
    var c = glow * u.params.x * u.tint + base;
    let bright = textureSampleLevel(
        band,
        band_sampler,
        vec2<f32>(0.5, t.y * u.distort.x - u.distort.y + 1.0),
        0.0,
    );
    if u.distort.x > 0.0 {
        c = vec4<f32>(c.rgb + bright.rgb * u.distort.z * u.tint.rgb * 0.35, c.a);
    }
    let lines = textureSampleLevel(scanlines, scanlines_sampler, t * u.params.w, 0.0);
    if u.params.w > 0.0 {
        c = c * lines;
    }
    let s = clamp(c, vec4<f32>(0.0), vec4<f32>(1.0));
    return vec4<f32>(decode(s.r), decode(s.g), decode(s.b), s.a);
}
