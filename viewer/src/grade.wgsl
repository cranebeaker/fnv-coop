// The game's last passes over the finished picture (see `grade.rs`): the
// bloom chain, the HDR brightness limit and the cell's image space, all on
// stored (gamma-encoded) values as the game's frame buffer holds them, and
// finally the conversion to linear light for the screen.
//
// The cinematic step, from `ISHDRBLENDINSHADERCIN` (values as the game
// sends them, recorded with apitrace):
//
//   lum = dot(c, (0.299, 0.587, 0.114))
//   c   = lerp(lum, c, saturation)
//   c   = lerp(c, lum * tint.rgb, tint amount)
//   c   = contrast * (brightness * c - contrast average) + contrast average
//   c   = lerp(c, fade.rgb, fade.a)

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var source: texture_2d<f32>;
@group(0) @binding(1) var linear_sampler: sampler;

struct ImageSpaceGrade {
    // Tint color, and how much of it.
    tint: vec4<f32>,
    // Saturation, contrast, the brightness contrast spreads around, and
    // brightness.
    cinematic: vec4<f32>,
    // Bright clamp (threshold), bright scale, blur radius in texels, and the
    // clamp on the average brightness's length.
    bloom: vec4<f32>,
    // x: the final pass's brightness limit; y: 1 when bloom and the limit
    // apply.
    hdr: vec4<f32>,
    // The colour the picture fades to, and how far.
    fade: vec4<f32>,
    // x: the modifiers' blur pass radius (0: none); y: how far its weights
    // are toward that row from the one below; z: the menus' background held;
    // w: which capture.
    background: vec4<f32>,
}
@group(0) @binding(2) var<uniform> grade: ImageSpaceGrade;
// The bloom (final pass) or the average brightness (bright pass).
@group(0) @binding(3) var extra: texture_2d<f32>;
// The HUD's picture (`hud.rs`): its pieces blended onto transparent black,
// so colour already times alpha (final pass only).
@group(0) @binding(4) var hud: texture_2d<f32>;

// Stored (sRGB-encoded) values to linear light, exactly as the GPU decodes.
fn srgb_decode(encoded: vec3<f32>) -> vec3<f32> {
    let c = max(encoded, vec3<f32>(0.0));
    return select(pow((c + 0.055) / 1.055, vec3<f32>(2.4)), c / 12.92, c <= vec3<f32>(0.04045));
}

fn texel(t: texture_2d<f32>) -> vec2<f32> {
    return 1.0 / vec2<f32>(textureDimensions(t));
}

// `ISHDRDOWN4`: four bilinear samples one source texel out diagonally,
// averaged. Alpha 0, as the game writes it.
@fragment
fn downsample(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let d = texel(source);
    var sum = vec3<f32>(0.0);
    sum += textureSample(source, linear_sampler, in.uv + vec2<f32>(-1.0, -1.0) * d).rgb;
    sum += textureSample(source, linear_sampler, in.uv + vec2<f32>(1.0, -1.0) * d).rgb;
    sum += textureSample(source, linear_sampler, in.uv + vec2<f32>(1.0, 1.0) * d).rgb;
    sum += textureSample(source, linear_sampler, in.uv + vec2<f32>(-1.0, 1.0) * d).rgb;
    return vec4<f32>(sum * 0.25, 0.0);
}

// `ISHDRDS4ADAPT`: the four corner pixels of the smallest picture averaged
// (the game samples one whole texture away from the middle, clamped), eased
// toward from the last frame's average (`extra` here; the game's `AvgLum`)
// by `1 − speed^seconds` (`HDRParam.z`, `TimingData.z`: `grade.hdr.zw`;
// seconds below 0 start over), its length clamped to [0.01, limit].
@fragment
fn average(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let last = vec2<i32>(textureDimensions(source)) - vec2<i32>(1);
    let sum = textureLoad(source, vec2<i32>(0, 0), 0).rgb
        + textureLoad(source, vec2<i32>(last.x, 0), 0).rgb
        + textureLoad(source, vec2<i32>(0, last.y), 0).rgb
        + textureLoad(source, last, 0).rgb;
    var a = sum * 0.25;
    let seconds = grade.hdr.w;
    if seconds >= 0.0 {
        var k = 0.0;
        if seconds > 0.0 {
            k = 1.0 - pow(max(grade.hdr.z, 0.0), seconds);
        }
        let previous = textureLoad(extra, vec2<i32>(0, 0), 0).rgb;
        a = k * a + (1.0 - k) * previous;
    }
    let len = max(length(a), 0.01);
    return vec4<f32>(a * (min(len, grade.bloom.w) / len), 0.0);
}

// The blur's weight for a tap `k` texels out: a Gaussian with σ = radius /
// 2, normalized over the taps −radius … radius (radius 6 gives the weights
// the game sends).
fn blur_weight(k: f32, radius: f32) -> f32 {
    let sigma = max(radius * 0.5, 1e-3);
    var total = 0.0;
    for (var i = -radius; i <= radius; i += 1.0) {
        total += exp(-(i * i) / (2.0 * sigma * sigma));
    }
    return exp(-(k * k) / (2.0 * sigma * sigma)) / total;
}

// `ISBPBLUR13`: each tap's `max(color − bright clamp, 0) × bright scale`,
// blurred vertically; alpha is the average brightness's r + g + b.
@fragment
fn bright_blur(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let d = texel(source);
    let radius = round(grade.bloom.z);
    var sum = vec3<f32>(0.0);
    for (var k = -radius; k <= radius; k += 1.0) {
        let c = textureSample(source, linear_sampler, in.uv + vec2<f32>(0.0, k) * d).rgb;
        sum += blur_weight(k, radius) * max(c - grade.bloom.x, vec3<f32>(0.0)) * grade.bloom.y;
    }
    let avg = textureLoad(extra, vec2<i32>(0, 0), 0).rgb;
    return vec4<f32>(sum, dot(avg, vec3<f32>(1.0)));
}

// `ISBLUR13`: the same blur horizontally; alpha is the last tap's.
@fragment
fn blur(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let d = texel(source);
    let radius = round(grade.bloom.z);
    var sum = vec3<f32>(0.0);
    for (var k = -radius; k <= radius; k += 1.0) {
        sum += blur_weight(k, radius) * textureSample(source, linear_sampler, in.uv + vec2<f32>(k, 0.0) * d).rgb;
    }
    let alpha = textureSample(source, linear_sampler, in.uv + vec2<f32>(radius, 0.0) * d).a;
    return vec4<f32>(sum, alpha);
}

// The image space blur effect's pass (`world::menu_background`, `00ba4270`):
// `ISBLUR(2r+1)` with the weights of the table's rows max(r − 1, 1) and r
// mixed, taps −r … r texels along `step`.
fn modifier_blur(uv: vec2<f32>, step: vec2<f32>) -> vec4<f32> {
    let radius = grade.background.x;
    let low = max(radius - 1.0, 1.0);
    var sum = vec3<f32>(0.0);
    for (var k = -radius; k <= radius; k += 1.0) {
        var w_low = 0.0;
        if abs(k) <= low {
            w_low = blur_weight(k, low);
        }
        let w = mix(w_low, blur_weight(k, radius), grade.background.y);
        sum += w * textureSample(source, linear_sampler, uv + k * step).rgb;
    }
    return vec4<f32>(sum, 1.0);
}

// Down first (`BlurScale` (0, 1/height)), then across ((1/width, 0)).
@fragment
fn modifier_blur_down(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    return modifier_blur(in.uv, vec2<f32>(0.0, texel(source).y));
}

@fragment
fn modifier_blur_across(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    return modifier_blur(in.uv, vec2<f32>(texel(source).x, 0.0));
}

// The picture as it is (the background captured without a blur).
@fragment
fn copy(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    return textureSample(source, linear_sampler, in.uv);
}

// `ISHDRBLENDINSHADERCIN`: the scene and the bloom combined under the
// brightness limit, then the cinematic step, then linear light.
@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(source, linear_sampler, in.uv);
    var c = color.rgb;
    if grade.hdr.y > 0.5 {
        let bloom = textureSample(extra, linear_sampler, in.uv);
        let limit = grade.hdr.x;
        let scale = 1.0 / max(bloom.a, limit);
        c = c * limit * scale + max(bloom.rgb * 0.5 * scale, vec3<f32>(0.0));
    }
    let lum = dot(c, vec3<f32>(0.299, 0.587, 0.114));
    c = mix(vec3<f32>(lum), c, grade.cinematic.x);
    c = mix(c, lum * grade.tint.rgb, grade.tint.a);
    c = mix(vec3<f32>(grade.cinematic.z), c * grade.cinematic.w, grade.cinematic.y);
    // Last, the fade (`lerp(c, Fade.rgb, Fade.w)`).
    c = mix(c, grade.fade.rgb, grade.fade.a);
    // The game writes this into its 8-bit frame buffer (clamped), then
    // draws the HUD over it, each piece source alpha over inverse source
    // alpha: the same as laying the HUD's picture over it once.
    c = clamp(c, vec3<f32>(0.0), vec3<f32>(1.0));
    let size = vec2<i32>(textureDimensions(hud));
    let h = textureLoad(hud, clamp(vec2<i32>(in.position.xy), vec2<i32>(0), size - vec2<i32>(1)), 0);
    c = h.rgb + c * (1.0 - h.a);
    return vec4<f32>(srgb_decode(c), color.a);
}
