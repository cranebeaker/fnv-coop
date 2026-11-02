//! Distant land beyond the outdoor test world's level-4 chunk: a
//! `.dlodsettings` quadtree file and chunks that carry geomorph heights (an
//! `NiAdditionalGeometryData` block), as the game's distant-land models do.

use super::{f32s, sized, Geometry, NifBuilder};

/// A worldspace's `.dlodsettings`: finest level, coarsest level with
/// models, root size, root cell, last cell, objects level.
pub fn lod_settings_file(
    min: u32,
    max: u32,
    root_level: u32,
    root: (i16, i16),
    last: (i16, i16),
    objects: u32,
) -> Vec<u8> {
    let mut v = Vec::new();
    for x in [min, max, root_level] {
        v.extend(x.to_le_bytes());
    }
    for x in [root.0, root.1, last.0, last.1] {
        v.extend(x.to_le_bytes());
    }
    v.extend(objects.to_le_bytes());
    v
}

/// A distant-land chunk: one lit shape under a root node moved by `by`,
/// whose vertices geomorph toward `morph` (one height per vertex, in the
/// shape's own space) through an `NiAdditionalGeometryData` block.
pub fn morphing_chunk_nif(g: &Geometry, texture: &str, by: [f32; 3], morph: &[f32]) -> Vec<u8> {
    let (positions, normals, uvs, triangles) = (&g.0, &g.1, &g.2, &g.3);
    let mut b = NifBuilder {
        root_moved: by,
        ..NifBuilder::default()
    };
    let mut root = b.av("Root", &[]);
    root.extend(1u32.to_le_bytes());
    root.extend(1i32.to_le_bytes());
    root.extend(0u32.to_le_bytes());
    let mut shape = b.av("Mesh", &[2]);
    shape.extend(4i32.to_le_bytes());
    shape.extend((-1i32).to_le_bytes());
    shape.extend(0u32.to_le_bytes());
    shape.extend((-1i32).to_le_bytes());
    shape.push(0);

    let mut data = 0i32.to_le_bytes().to_vec();
    data.extend((positions.len() as u16).to_le_bytes());
    data.extend([0, 0, 1]);
    for p in positions {
        data.extend(f32s(p));
    }
    data.extend(1u16.to_le_bytes());
    data.push(1);
    for n in normals {
        data.extend(f32s(n));
    }
    data.extend(f32s(&[0.0, 0.0, 0.0, 50_000.0]));
    data.push(0);
    for uv in uvs {
        data.extend(f32s(uv));
    }
    data.extend(0u16.to_le_bytes());
    // The additional data: block 5.
    data.extend(5i32.to_le_bytes());
    data.extend((triangles.len() as u16).to_le_bytes());
    data.extend((triangles.len() as u32 * 3).to_le_bytes());
    data.push(1);
    for t in triangles {
        for i in t {
            data.extend(i.to_le_bytes());
        }
    }
    data.extend(0u16.to_le_bytes());

    let mut shader = b.net("");
    shader.extend(1u16.to_le_bytes());
    for v in [1u32, 0, 0] {
        shader.extend(v.to_le_bytes());
    }
    shader.extend(1.0f32.to_le_bytes());
    shader.extend(3u32.to_le_bytes());
    shader.extend(3i32.to_le_bytes());
    shader.extend(f32s(&[0.0]));
    shader.extend(0i32.to_le_bytes());
    shader.extend(f32s(&[4.0, 1.0]));
    let mut set = 6i32.to_le_bytes().to_vec();
    for path in [texture, "", "", "", "", ""] {
        set.extend(sized(path));
    }

    // One channel of one float per vertex, in one data block.
    let n = morph.len() as u32;
    let mut extra = (n as u16).to_le_bytes().to_vec();
    extra.extend(1u32.to_le_bytes());
    for v in [1u32, 4, 4 * n, 4, 0, 0] {
        extra.extend(v.to_le_bytes());
    }
    extra.push(2);
    extra.extend(1u32.to_le_bytes());
    extra.push(1);
    for v in [4 * n, 1, 0, 1, 4] {
        extra.extend(v.to_le_bytes());
    }
    extra.extend(f32s(morph));

    b.blocks = vec![
        ("BSFadeNode".into(), root),
        ("NiTriShape".into(), shape),
        ("BSShaderPPLightingProperty".into(), shader),
        ("BSShaderTextureSet".into(), set),
        ("NiTriShapeData".into(), data),
        ("NiAdditionalGeometryData".into(), extra),
    ];
    b.build()
}
