//! The local map's pictures (`TESObjectCELL::TakeLocalMapPicture` (Xbox
//! PDB), `0054e830`; `world::local_map` has the rules): one tile seen from
//! straight above, orthographic, 4352 units square into 128 × 128 pixels,
//! cleared to black, the meshes the caller gives (the game: those whose
//! property has the runtime flag "Show in Local Map" and a bound of 50 or
//! more, `00b64440`), without lights. The colour is the local-map pass's (`SLS2084.vso` with
//! `SLS2087.pso`, shader package 13): the surface's normal in view space,
//! `n × 0.5 + 0.5` (the vertex shader packs it, the pixel shader unpacks,
//! normalizes and packs it again), alpha 1.
//!
//! Here the packed normal is worked out per vertex and drawn as the vertex
//! colour [guess: the game normalizes per pixel; the difference is only
//! inside a triangle], and faces turned away from the camera are left out
//! (the renderer's default; ceilings face down) [guess: the pass's cull
//! mode isn't traced].

use crate::raster::{Camera, Material, MeshInput, Renderer, Vec3};
use world::local_map::{TilePicture, PICTURE_HALF, PICTURE_PIXELS};

/// One mesh for the pictures, in the map's frame (world units, z up).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapMesh {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub triangles: Vec<[u16; 3]>,
}

/// A tile's picture as RGBA bytes, `PICTURE_PIXELS` square, north up: the
/// camera at the tile's picture middle, `top` + the lift high (`0054ee80`:
/// the land's top + 20000 outdoors; `0054f500`: the bound's top + 40000
/// indoors).
pub fn picture(meshes: &[MapMesh], tile: &TilePicture, top: f32, lift: f32) -> Vec<u8> {
    let camera = Camera::top_down(tile.centre, top + lift, PICTURE_HALF);
    let n = PICTURE_PIXELS;
    let mut r = Renderer::new(n, n, [0.0; 3], camera);
    for m in meshes {
        // In view space x east, y north, z down (into the picture): an
        // upward face is (0, 0, −1).
        let colors: Vec<[f32; 4]> = m
            .normals
            .iter()
            .map(|nn| {
                let v = normalize([nn[0], nn[1], -nn[2]]);
                [v[0] * 0.5 + 0.5, v[1] * 0.5 + 0.5, v[2] * 0.5 + 0.5, 1.0]
            })
            .collect();
        let input = MeshInput {
            positions: &m.positions,
            normals: &m.normals,
            uvs: &[],
            colors: &colors,
            triangles: &m.triangles,
        };
        r.draw(
            &input,
            Material {
                lit: false,
                ..Material::default()
            },
        );
    }
    r.finish();
    r.to_rgba8(1).2
}

fn normalize(v: Vec3) -> Vec3 {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l > 1e-12 {
        [v[0] / l, v[1] / l, v[2] / l]
    } else {
        [0.0, 0.0, 0.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A floor seen from above is (0.5, 0.5, 0) (its normal up, away from
    /// the view), a wall facing north (0.5, 1, 0.5); outside them, black.
    #[test]
    fn a_picture_is_the_view_space_normal() {
        let tile = world::local_map::tile_picture([2048.0, 2048.0, 0.0], false, 1, 0);
        let floor = MapMesh {
            positions: vec![[0.0, 0.0, 0.0], [2000.0, 0.0, 0.0], [0.0, 2000.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            triangles: vec![[0, 1, 2]],
        };
        let px = picture(&[floor], &tile, 0.0, 20_000.0);
        assert_eq!(px.len(), 128 * 128 * 4);
        // Near the south-west corner (bottom left of the picture).
        let at = |x: usize, y: usize| &px[(y * 128 + x) * 4..(y * 128 + x) * 4 + 4];
        assert_eq!(at(5, 120), &[128, 128, 0, 255]);
        // The north-east corner is empty.
        assert_eq!(at(120, 5), &[0, 0, 0, 255]);
    }
}
