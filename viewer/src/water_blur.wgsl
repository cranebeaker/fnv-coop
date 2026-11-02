// The blur the game gives the water's reflection (`bUseWaterReflectionBlur`):
// its Gaussian blur of radius `iWaterBlurAmount` + 1 texels (5 here),
// horizontally then vertically. The weights are taken as the bloom's
// (`grade.wgsl`): σ = radius / 2 over the taps −radius … radius,
// normalized; the game builds both with the same code (`00ba4270`;
// inferred).

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var source: texture_2d<f32>;
@group(0) @binding(1) var linear_sampler: sampler;

const RADIUS: f32 = f32(#{BLUR_RADIUS});

fn weight(k: f32) -> f32 {
    let sigma = max(RADIUS * 0.5, 1e-3);
    var total = 0.0;
    for (var i = -RADIUS; i <= RADIUS; i += 1.0) {
        total += exp(-(i * i) / (2.0 * sigma * sigma));
    }
    return exp(-(k * k) / (2.0 * sigma * sigma)) / total;
}

fn blurred(uv: vec2<f32>, step: vec2<f32>) -> vec4<f32> {
    var sum = vec4<f32>(0.0);
    for (var k = -RADIUS; k <= RADIUS; k += 1.0) {
        sum += weight(k) * textureSample(source, linear_sampler, uv + step * k);
    }
    return sum;
}

@fragment
fn horizontal(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let d = 1.0 / vec2<f32>(textureDimensions(source));
    return blurred(in.uv, vec2<f32>(d.x, 0.0));
}

@fragment
fn vertical(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let d = 1.0 / vec2<f32>(textureDimensions(source));
    return blurred(in.uv, vec2<f32>(0.0, d.y));
}
