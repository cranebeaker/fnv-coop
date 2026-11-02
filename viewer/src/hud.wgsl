// The game's tile shaders, for the HUD (see `hud.rs`), on stored values:
//
//   TILE1000.pso: Src0(uv) × TintColor
//   TILE1001.vso/.pso: Src0(uv × TexScroll.zw + TexScroll.xy) × TintColor,
//                      alpha × AlphaMap(uv).r (the map at the unscrolled
//                      coordinates)
//
// `mode.x` is 1 for TILE1001 (the compass), 0 for TILE1000 (the scroll is
// then (0, 0, 1, 1)).

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct TileParams {
    tint: vec4<f32>,
    scroll: vec4<f32>,
    mode: vec4<f32>,
}

@group(2) @binding(0) var<uniform> params: TileParams;
@group(2) @binding(1) var src0: texture_2d<f32>;
@group(2) @binding(2) var src0_sampler: sampler;
@group(2) @binding(3) var alpha_map: texture_2d<f32>;
@group(2) @binding(4) var alpha_map_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var c = textureSample(src0, src0_sampler, mesh.uv * params.scroll.zw + params.scroll.xy) * params.tint;
    let mask = textureSample(alpha_map, alpha_map_sampler, mesh.uv).r;
    if params.mode.x > 0.5 {
        c.a = c.a * mask;
    }
#ifdef VERTEX_COLORS
    // A model piece's corners' alpha (the local map's fog of war).
    c = c * mesh.color;
#endif
    return c;
}
