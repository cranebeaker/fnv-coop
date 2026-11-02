//! Builders for game data used by the tests: NIF models, DDS textures,
//! plugins, and a small test room assembled into a temporary Data folder.
//! Nothing from the game is needed.

#![allow(clippy::missing_panics_doc)]

use std::fs;
use std::path::{Path, PathBuf};

pub mod ai;
pub mod companion_gear;
pub mod crafting;
pub mod dialogue;
pub mod energy;
pub mod factions;
pub mod fatigue;
pub mod fighting;
pub mod functions;
pub mod impacts;
pub mod launchers;
pub mod living;
pub mod lod;
pub mod long_paths;
pub mod melee;
pub mod mods;
pub mod more;
pub mod music;
pub mod packages;
pub mod particles;
pub mod quest_targets;
pub mod ref_scripts;
pub mod revise;
pub mod trees;
pub mod vats;
pub mod water;

#[derive(Default)]
pub struct NifBuilder {
    strings: Vec<String>,
    blocks: Vec<(String, Vec<u8>)>,
    /// Give the root node a quarter turn anticlockwise, which the game
    /// ignores for placed models.
    root_turned: bool,
    /// Move the root node (models that aren't placed, like distant land,
    /// are put in the world this way).
    root_moved: [f32; 3],
}

pub fn f32s(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

pub fn sized(s: &str) -> Vec<u8> {
    let mut v = (s.len() as u32).to_le_bytes().to_vec();
    v.extend(s.as_bytes());
    v
}

impl NifBuilder {
    fn string(&mut self, s: &str) -> i32 {
        if s.is_empty() {
            return -1;
        }
        match self.strings.iter().position(|x| x == s) {
            Some(i) => i as i32,
            None => {
                self.strings.push(s.to_string());
                self.strings.len() as i32 - 1
            }
        }
    }

    fn net(&mut self, name: &str) -> Vec<u8> {
        let mut v = self.string(name).to_le_bytes().to_vec();
        v.extend(0u32.to_le_bytes());
        v.extend((-1i32).to_le_bytes());
        v
    }

    fn av(&mut self, name: &str, props: &[i32]) -> Vec<u8> {
        let mut v = self.net(name);
        v.extend(0u32.to_le_bytes());
        let moved = if name == "Root" {
            self.root_moved
        } else {
            [0.0; 3]
        };
        v.extend(f32s(&moved));
        if name == "Root" && self.root_turned {
            v.extend(f32s(&[0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
        } else {
            v.extend(f32s(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]));
        }
        v.extend(f32s(&[1.0]));
        v.extend((props.len() as u32).to_le_bytes());
        for p in props {
            v.extend(p.to_le_bytes());
        }
        v.extend((-1i32).to_le_bytes());
        v
    }

    /// One shape under a root node, drawn as `style` says.
    pub fn mesh(g: &Geometry, texture: &str, style: Style) -> Vec<u8> {
        let (positions, normals, uvs, triangles) = (&g.0, &g.1, &g.2, &g.3);
        let mut b = NifBuilder {
            root_turned: style == Style::TurnedRoot,
            root_moved: match style {
                Style::MovedRoot(by) => by,
                _ => [0.0; 3],
            },
            ..NifBuilder::default()
        };
        let mut root = b.av("Root", &[]);
        root.extend(1u32.to_le_bytes());
        root.extend(1i32.to_le_bytes());
        root.extend(0u32.to_le_bytes());
        let unlit = matches!(style, Style::Effect(_) | Style::Shadow);
        let (glow, external) = match style {
            Style::Glowing { glow, external } => (Some(glow), external),
            _ => (None, false),
        };
        let (props, data_block): (&[i32], i32) = if unlit {
            (&[2, 3, 4], 5)
        } else if glow.is_some() {
            (&[2, 4], 5)
        } else {
            (&[2], 4)
        };
        let mut shape = b.av("Mesh", props);
        shape.extend(data_block.to_le_bytes());
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
        data.extend(f32s(&[0.0, 0.0, 0.0, 500.0]));
        if style == Style::Shadow {
            // Black, half transparent: the shadow is all in the vertices.
            data.push(1);
            for _ in positions {
                data.extend(f32s(&[0.0, 0.0, 0.0, 0.5]));
            }
        } else {
            data.push(0);
        }
        for uv in uvs {
            data.extend(f32s(uv));
        }
        data.extend(0u16.to_le_bytes());
        data.extend((-1i32).to_le_bytes());
        data.extend((triangles.len() as u16).to_le_bytes());
        data.extend((triangles.len() as u32 * 3).to_le_bytes());
        data.push(1);
        for t in triangles {
            for i in t {
                data.extend(i.to_le_bytes());
            }
        }
        data.extend(0u16.to_le_bytes());

        b.blocks = if unlit {
            // Effects: glow texture, additive. Shadows: no texture, vertex
            // alpha and decal flags, ordinary blending, and (as in the
            // game's picture frames) the vertex-colors flag left unset.
            // Effects are depth-tested but don't write depth, like the
            // game's light beams (0x80000000 / 0).
            let (flags, flags2, file, blend, opacity) = match style {
                Style::Effect(opacity) => (0x8000_0000u32, 0u32, texture, 0x000Du16, opacity),
                _ => (0x8E00_0008, 0x0000_0001, "", 0x10ED, 1.0),
            };
            let mut shader = b.net("");
            shader.extend(1u16.to_le_bytes());
            for v in [1u32, flags, flags2] {
                shader.extend(v.to_le_bytes());
            }
            shader.extend(1.0f32.to_le_bytes());
            shader.extend(3u32.to_le_bytes());
            shader.extend(sized(file));
            shader.extend(f32s(&[1.0, 0.0, 1.0, 0.0])); // falloff
            let mut alpha = b.net("");
            alpha.extend(blend.to_le_bytes());
            alpha.push(0);
            let mut material = b.net("");
            material.extend(f32s(&[1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 10.0, opacity, 1.0]));
            vec![
                ("BSFadeNode".into(), root),
                ("NiTriShape".into(), shape),
                ("BSShaderNoLightingProperty".into(), shader),
                ("NiAlphaProperty".into(), alpha),
                ("NiMaterialProperty".into(), material),
                ("NiTriShapeData".into(), data),
            ]
        } else {
            let mut shader = b.net("");
            shader.extend(1u16.to_le_bytes());
            // Depth test and write on, as on (nearly) every game mesh.
            let flags = if external { 0xA000_0000 } else { 0x8000_0000 };
            for v in [1u32, flags, 1] {
                shader.extend(v.to_le_bytes());
            }
            shader.extend(1.0f32.to_le_bytes());
            shader.extend(3u32.to_le_bytes());
            shader.extend(3i32.to_le_bytes());
            shader.extend(f32s(&[0.0]));
            shader.extend(0i32.to_le_bytes());
            shader.extend(f32s(&[4.0, 1.0]));
            let mut set = 6i32.to_le_bytes().to_vec();
            for path in [texture, "", glow.unwrap_or(""), "", "", ""] {
                set.extend(sized(path));
            }
            let mut blocks: Vec<(String, Vec<u8>)> = vec![
                ("BSFadeNode".into(), root),
                ("NiTriShape".into(), shape),
                ("BSShaderPPLightingProperty".into(), shader),
                ("BSShaderTextureSet".into(), set),
            ];
            if glow.is_some() {
                // Self-lit in white: specular, emissive, glossiness, alpha,
                // emissive multiplier.
                let mut material = b.net("");
                material.extend(f32s(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 10.0, 1.0, 1.0]));
                blocks.push(("NiMaterialProperty".into(), material));
            }
            blocks.push(("NiTriShapeData".into(), data));
            blocks
        };
        b.build()
    }

    pub fn build(&self) -> Vec<u8> {
        let mut types: Vec<&str> = Vec::new();
        for (t, _) in &self.blocks {
            if !types.contains(&t.as_str()) {
                types.push(t);
            }
        }
        let mut out = b"Gamebryo File Format, Version 20.2.0.7\n".to_vec();
        out.extend(0x1402_0007u32.to_le_bytes());
        out.push(1);
        out.extend(11u32.to_le_bytes());
        out.extend((self.blocks.len() as u32).to_le_bytes());
        out.extend(34u32.to_le_bytes());
        for s in ["test", "", ""] {
            out.push(s.len() as u8 + 1);
            out.extend(s.as_bytes());
            out.push(0);
        }
        out.extend((types.len() as u16).to_le_bytes());
        for t in &types {
            out.extend(sized(t));
        }
        for (t, _) in &self.blocks {
            out.extend((types.iter().position(|x| x == t).unwrap() as u16).to_le_bytes());
        }
        for (_, data) in &self.blocks {
            out.extend((data.len() as u32).to_le_bytes());
        }
        out.extend((self.strings.len() as u32).to_le_bytes());
        out.extend((self.strings.iter().map(String::len).max().unwrap_or(0) as u32).to_le_bytes());
        for s in &self.strings {
            out.extend(sized(s));
        }
        out.extend(0u32.to_le_bytes());
        for (_, data) in &self.blocks {
            out.extend(data);
        }
        out.extend(1u32.to_le_bytes());
        out.extend(0i32.to_le_bytes());
        out
    }
}

pub type Geometry = (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[u16; 3]>);

/// A quad with corners counter-clockwise seen from the front.
pub fn quad(corners: [[f32; 3]; 4], normal: [f32; 3]) -> Geometry {
    (
        corners.to_vec(),
        vec![normal; 4],
        vec![[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
        vec![[0, 1, 2], [0, 2, 3]],
    )
}

pub fn boxed(lo: [f32; 3], hi: [f32; 3]) -> Geometry {
    let [x0, y0, z0] = lo;
    let [x1, y1, z1] = hi;
    let faces = [
        (
            [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]],
            [0.0, 0.0, 1.0],
        ),
        (
            [[x0, y1, z0], [x1, y1, z0], [x1, y0, z0], [x0, y0, z0]],
            [0.0, 0.0, -1.0],
        ),
        (
            [[x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1]],
            [0.0, -1.0, 0.0],
        ),
        (
            [[x1, y1, z0], [x0, y1, z0], [x0, y1, z1], [x1, y1, z1]],
            [0.0, 1.0, 0.0],
        ),
        (
            [[x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1]],
            [1.0, 0.0, 0.0],
        ),
        (
            [[x0, y1, z0], [x0, y0, z0], [x0, y0, z1], [x0, y1, z1]],
            [-1.0, 0.0, 0.0],
        ),
    ];
    let mut g: Geometry = Default::default();
    for (corners, n) in faces {
        let base = g.0.len() as u16;
        let (p, nn, uv, t) = quad(corners, n);
        g.0.extend(p);
        g.1.extend(nn);
        g.2.extend(uv);
        g.3.extend(t.iter().map(|t| t.map(|i| i + base)));
    }
    g
}

/// How a test model's one shape is drawn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Plain,
    /// A light effect with this material opacity.
    Effect(f32),
    /// A plain shape under a root node turned a quarter anticlockwise.
    TurnedRoot,
    /// A plain shape under a root node moved by this much.
    MovedRoot([f32; 3]),
    /// An untextured shadow decal made of vertex colors.
    Shadow,
    /// A lit shape that lights itself in white through this glow map; with
    /// `external`, in the color of the placed object's Emittance setting
    /// instead.
    Glowing {
        glow: &'static str,
        external: bool,
    },
}

pub fn nif(g: &Geometry, texture: &str) -> Vec<u8> {
    NifBuilder::mesh(g, texture, Style::Plain)
}

pub fn effect_nif(g: &Geometry, texture: &str, opacity: f32) -> Vec<u8> {
    NifBuilder::mesh(g, texture, Style::Effect(opacity))
}

/// A model whose root node is turned a quarter anticlockwise.
pub fn turned_root_nif(g: &Geometry, texture: &str) -> Vec<u8> {
    NifBuilder::mesh(g, texture, Style::TurnedRoot)
}

/// A model whose root node is moved, as distant land chunks are.
pub fn moved_root_nif(g: &Geometry, texture: &str, by: [f32; 3]) -> Vec<u8> {
    NifBuilder::mesh(g, texture, Style::MovedRoot(by))
}

/// A soft shadow like the one behind the game's picture frames.
pub fn shadow_nif(g: &Geometry) -> Vec<u8> {
    NifBuilder::mesh(g, "", Style::Shadow)
}

/// A lit model that glows through a glow map, like a lamp that's on. With
/// `external`, the glow takes its color from the placed object instead.
pub fn glowing_nif(g: &Geometry, texture: &str, glow: &'static str, external: bool) -> Vec<u8> {
    NifBuilder::mesh(g, texture, Style::Glowing { glow, external })
}

/// An uncompressed 32-bit DDS of one color.
pub fn dds(rgb: [u8; 3]) -> Vec<u8> {
    let (w, h) = (8u32, 8u32);
    let mut v = b"DDS ".to_vec();
    for x in [124u32, 0x100F, h, w, w * 4, 0, 1] {
        v.extend(x.to_le_bytes());
    }
    v.extend([0u8; 44]);
    for x in [
        32u32,
        0x41,
        0,
        32,
        0x00FF_0000,
        0x0000_FF00,
        0x0000_00FF,
        0xFF00_0000,
    ] {
        v.extend(x.to_le_bytes());
    }
    for x in [0x1000u32, 0, 0, 0, 0] {
        v.extend(x.to_le_bytes());
    }
    for _ in 0..w * h {
        v.extend([rgb[2], rgb[1], rgb[0], 255]);
    }
    v
}

// ---------------------------------------------------------------------------
// The plugin
// ---------------------------------------------------------------------------

pub fn sub(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut v = kind.to_vec();
    v.extend((data.len() as u16).to_le_bytes());
    v.extend(data);
    v
}

pub fn zstr(s: &str) -> Vec<u8> {
    let mut v = s.as_bytes().to_vec();
    v.push(0);
    v
}

pub fn record(kind: &[u8; 4], id: u32, data: &[u8]) -> Vec<u8> {
    let mut v = kind.to_vec();
    v.extend((data.len() as u32).to_le_bytes());
    v.extend(if kind == b"TES4" { 1u32 } else { 0 }.to_le_bytes());
    v.extend(id.to_le_bytes());
    v.extend([0; 4]);
    v.extend(15u16.to_le_bytes());
    v.extend([0; 2]);
    v.extend(data);
    v
}

pub fn group(label: [u8; 4], kind: i32, contents: &[u8]) -> Vec<u8> {
    let mut v = b"GRUP".to_vec();
    v.extend((24 + contents.len() as u32).to_le_bytes());
    v.extend(label);
    v.extend(kind.to_le_bytes());
    v.extend([0; 8]);
    v.extend(contents);
    v
}

pub fn stat(id: u32, editor_id: &str, model: &str) -> Vec<u8> {
    let mut d = sub(b"EDID", &zstr(editor_id));
    d.extend(sub(b"MODL", &zstr(model)));
    record(b"STAT", id, &d)
}

pub fn placed(id: u32, base: u32, pos: [f32; 3], degrees: [f32; 3], extra: &[u8]) -> Vec<u8> {
    let mut d = sub(b"NAME", &base.to_le_bytes());
    d.extend(extra);
    let r = degrees.map(f32::to_radians);
    d.extend(sub(
        b"DATA",
        &f32s(&[pos[0], pos[1], pos[2], r[0], r[1], r[2]]),
    ));
    record(b"REFR", id, &d)
}

pub struct TempData(PathBuf);

#[cfg(test)]
mod temporary_tests {
    #[test]
    fn windows_game_paths_write_under_native_directories() {
        let data = super::TempData::empty("windows-game-paths");
        data.write(r"meshes\test\floor.nif", b"fixture");
        assert_eq!(
            std::fs::read(data.path().join("meshes/test/floor.nif")).unwrap(),
            b"fixture"
        );
    }

    #[test]
    fn same_tag_fixtures_do_not_replace_or_remove_each_other() {
        let first = super::functions::functions("shared-tag");
        first.write("owner.txt", b"first fixture");
        let second = super::functions::functions("shared-tag");
        second.write("owner.txt", b"second fixture");
        assert_ne!(first.path(), second.path());
        assert_eq!(
            std::fs::read(first.path().join("owner.txt")).unwrap(),
            b"first fixture"
        );
        let second_path = second.path().to_path_buf();
        drop(second);
        assert!(!second_path.exists());
        assert_eq!(
            std::fs::read(first.path().join("owner.txt")).unwrap(),
            b"first fixture"
        );
        assert!(std::fs::read(first.path().join("FalloutNV.esm"))
            .unwrap()
            .starts_with(b"TES4"));
    }
}

impl Drop for TempData {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

impl TempData {
    fn new(tag: &str) -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        loop {
            let dir = std::env::temp_dir().join(format!(
                "nv-rs-{tag}-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&dir) {
                Ok(()) => return Self(dir),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("cannot create test fixture {dir:?}: {e}"),
            }
        }
    }

    /// An empty fixture folder, for a test that writes its own plugin.
    pub fn empty(tag: &str) -> Self {
        Self::new(tag)
    }

    pub fn write(&self, relative: &str, bytes: &[u8]) {
        // Game records use Windows separators; make fixture paths portable.
        let path = self.0.join(relative.replace('\\', "/"));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

pub const FLOOR_RGB: [u8; 3] = [40, 200, 40];
pub const WALL_RGB: [u8; 3] = [40, 40, 200];
pub const PLANK_RGB: [u8; 3] = [220, 30, 30];
/// The test room's image space: saturation, contrast average, contrast,
/// brightness, tint (red, green, blue) and tint amount.
pub const IMAGE_SPACE_CINEMATIC: [f32; 8] = [0.8, 0.5, 1.2, 1.1, 1.0, 0.8, 0.5, 0.3];
/// The test room's image space HDR values: eye adaptation speed, blur
/// radius, passes, emissive multiplier (2: lit glows come out doubled),
/// target, upper clamp, bright scale, bright clamp.
pub const IMAGE_SPACE_HDR: [f32; 8] = [0.3, 6.0, 4.0, 2.0, 1.0, 1.0, 2.4, 0.9];
/// The lamp's diffuse texture, and its glow map.
pub const LAMP_RGB: [u8; 3] = [100, 100, 100];
pub const LAMP_GLOW_RGB: [u8; 3] = [0, 150, 0];

/// A 512-unit square room: green floor, one blue wall on the north side
/// facing south, a red plank turned to point east, a model that's missing,
/// a texture that's missing, and in the east three lamps that glow through a
/// glow map: one in its own white, and two whose glow can take its color
/// from the placed object (one set to the warm light, one set to nothing).
pub fn room(tag: &str) -> TempData {
    let data = TempData::new(tag);

    data.write(
        "meshes/test/floor.nif",
        &nif(
            &quad(
                [
                    [-256.0, -256.0, 0.0],
                    [256.0, -256.0, 0.0],
                    [256.0, 256.0, 0.0],
                    [-256.0, 256.0, 0.0],
                ],
                [0.0, 0.0, 1.0],
            ),
            "textures\\test\\floor.dds",
        ),
    );
    // Along X, facing +Y.
    data.write(
        "meshes/test/wall.nif",
        &nif(
            &quad(
                [
                    [256.0, 0.0, 0.0],
                    [-256.0, 0.0, 0.0],
                    [-256.0, 0.0, 240.0],
                    [256.0, 0.0, 240.0],
                ],
                [0.0, 1.0, 0.0],
            ),
            "textures\\test\\wall.dds",
        ),
    );
    // Lies along +Y from its origin.
    data.write(
        "meshes/test/plank.nif",
        &nif(
            &boxed([-8.0, 0.0, 0.0], [8.0, 150.0, 6.0]),
            "textures\\test\\plank.dds",
        ),
    );
    data.write(
        "meshes/test/crate.nif",
        &nif(
            &boxed([-16.0, -16.0, 0.0], [16.0, 16.0, 32.0]),
            "textures\\test\\gone.dds",
        ),
    );
    data.write("textures/test/floor.dds", &dds(FLOOR_RGB));
    data.write("textures/test/wall.dds", &dds(WALL_RGB));
    data.write("textures/test/plank.dds", &dds(PLANK_RGB));
    // A glow card lying flat 100 units up, like a pool of light from a lamp.
    data.write(
        "meshes/test/glow.nif",
        &effect_nif(
            &quad(
                [
                    [20.0, -120.0, 100.0],
                    [120.0, -120.0, 100.0],
                    [120.0, -20.0, 100.0],
                    [20.0, -20.0, 100.0],
                ],
                [0.0, 0.0, 1.0],
            ),
            "textures\\test\\glow.dds",
            0.5,
        ),
    );
    data.write("textures/test/glow.dds", &dds([255, 255, 255]));
    // A shadow decal lying on the floor.
    data.write(
        "meshes/test/shadow.nif",
        &shadow_nif(&quad(
            [
                [-120.0, 20.0, 1.0],
                [-20.0, 20.0, 1.0],
                [-20.0, 120.0, 1.0],
                [-120.0, 120.0, 1.0],
            ],
            [0.0, 0.0, 1.0],
        )),
    );
    // A beam lying along +X whose root node is turned to +Y. Viewers show it
    // along +Y; the game, placing it, ignores the root and keeps it on +X.
    data.write(
        "meshes/test/beam.nif",
        &turned_root_nif(
            &boxed([0.0, -8.0, 0.0], [150.0, 8.0, 6.0]),
            "textures\\test\\plank.dds",
        ),
    );

    // A grey box whose glow map lights it green.
    let lamp_box = boxed([-20.0, -20.0, 0.0], [20.0, 20.0, 40.0]);
    data.write(
        "meshes/test/lamp.nif",
        &glowing_nif(
            &lamp_box,
            "textures\\test\\lamp.dds",
            "textures\\test\\lamp_g.dds",
            false,
        ),
    );
    data.write(
        "meshes/test/extlamp.nif",
        &glowing_nif(
            &lamp_box,
            "textures\\test\\lamp.dds",
            "textures\\test\\lamp_g.dds",
            true,
        ),
    );
    data.write("textures/test/lamp.dds", &dds(LAMP_RGB));
    data.write("textures/test/lamp_g.dds", &dds(LAMP_GLOW_RGB));

    let mut bases = stat(0x800, "Floor", "Test\\Floor.nif");
    bases.extend(stat(0x801, "Wall", "Test\\Wall.nif"));
    bases.extend(stat(0x802, "Plank", "Test\\Plank.nif"));
    bases.extend(stat(0x803, "Crate", "Test\\Crate.nif"));
    bases.extend(stat(0x804, "Statue", "Test\\Statue.nif"));
    bases.extend(stat(0x805, "Glow", "Test\\Glow.nif"));
    bases.extend(stat(0x806, "Beam", "Test\\Beam.nif"));
    bases.extend(stat(0x807, "Shadow", "Test\\Shadow.nif"));
    bases.extend(stat(0x808, "GlowLamp", "Test\\Lamp.nif"));
    bases.extend(stat(0x809, "ExtLamp", "Test\\ExtLamp.nif"));
    let coc = stat(0x32, "COCMarkerHeading", "marker_coc.nif");
    // A warm light far above the room, so it doesn't change the renders.
    let mut light = vec![0u8; 32];
    light[4..8].copy_from_slice(&300u32.to_le_bytes());
    light[8..12].copy_from_slice(&[255, 200, 100, 0]);
    light[16..20].copy_from_slice(&1.0f32.to_le_bytes());
    light[20..24].copy_from_slice(&90.0f32.to_le_bytes());
    let mut lamp = sub(b"EDID", &zstr("Lamp"));
    lamp.extend(sub(b"DATA", &light));
    lamp.extend(sub(b"FNAM", &f32s(&[1.5])));
    let lights = record(b"LIGH", 0x810, &lamp);
    // A warm image space (cinematic values after 25 others), its HDR
    // values leading: eye adaptation, blur radius, passes, emissive
    // multiplier, target, upper clamp, bright scale, bright clamp.
    let mut dnam = vec![0.0f32; 38];
    dnam[0..8].copy_from_slice(&IMAGE_SPACE_HDR);
    dnam[25..33].copy_from_slice(&IMAGE_SPACE_CINEMATIC);
    let mut space = sub(b"EDID", &zstr("TestImageSpace"));
    space.extend(sub(b"DNAM", &f32s(&dnam)));
    let image_spaces = record(b"IMGS", 0x820, &space);

    let mut refs = placed(0x901, 0x800, [0.0; 3], [0.0; 3], &[]);
    refs.extend(placed(
        0x902,
        0x801,
        [0.0, 256.0, 0.0],
        [0.0, 0.0, 180.0],
        &[],
    ));
    refs.extend(placed(0x903, 0x802, [0.0, 0.0, 0.0], [0.0, 0.0, 90.0], &[]));
    refs.extend(placed(0x904, 0x803, [-150.0, -150.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(0x905, 0x804, [150.0, -150.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(0x907, 0x805, [0.0; 3], [0.0; 3], &[]));
    refs.extend(placed(0x908, 0x806, [0.0, -200.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(0x909, 0x807, [0.0; 3], [0.0; 3], &[]));
    refs.extend(placed(0x90C, 0x808, [200.0, 200.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(0x90D, 0x809, [200.0, 120.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(
        0x90E,
        0x809,
        [200.0, 40.0, 0.0],
        [0.0; 3],
        &sub(b"XEMI", &0x810u32.to_le_bytes()),
    ));
    refs.extend(placed(
        0x906,
        0x802,
        [-200.0, 100.0, 0.0],
        [20.0, 0.0, 45.0],
        &[],
    ));

    // The coc marker stands in the south of the room, facing east.
    refs.extend(placed(
        0x90A,
        0x32,
        [0.0, -150.0, 0.0],
        [0.0, 0.0, 90.0],
        &[],
    ));
    refs.extend(placed(0x90B, 0x810, [0.0, 0.0, 5000.0], [0.0; 3], &[]));

    let mut cell = sub(b"EDID", &zstr("TestRoom"));
    cell.extend(sub(b"DATA", &[1]));
    let mut xcll = vec![100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    xcll.extend(f32s(&[0.0, 1000.0]));
    xcll.extend([0u8; 8]);
    xcll.extend(f32s(&[1.0, 1000.0, 1.0]));
    cell.extend(sub(b"XCLL", &xcll));
    cell.extend(sub(b"XCIM", &0x820u32.to_le_bytes()));
    let mut contents = record(b"CELL", 0x900, &cell);
    contents.extend(group(
        0x900u32.to_le_bytes(),
        6,
        &group(0x900u32.to_le_bytes(), 9, &refs),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    bases.extend(coc);
    plugin.extend(group(*b"STAT", 0, &bases));
    plugin.extend(group(*b"LIGH", 0, &lights));
    plugin.extend(group(*b"IMGS", 0, &image_spaces));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}

/// The outdoor test world's weather by day: ambient, sunlight, fog, upper
/// sky, horizon and lower sky.
/// The outdoor world's second weather (`TestStorm`, rainy), the global
/// that lets the climate pick it (`TestStormy`, 0), and the weather region
/// over square 1,0 whose weather it is (`TestStormRegion`).
pub const OUTDOOR_STORM: u32 = 0xB01;
pub const OUTDOOR_STORMY: u32 = 0xB30;
pub const OUTDOOR_STORM_REGION: u32 = 0xB40;
pub const OUTDOOR_AMBIENT: [u8; 3] = [60, 70, 80];
pub const OUTDOOR_SUNLIGHT: [u8; 3] = [250, 220, 160];
pub const OUTDOOR_FOG: [u8; 3] = [150, 160, 170];
pub const OUTDOOR_SKY: [[u8; 3]; 3] = [[70, 110, 160], [180, 190, 200], [120, 130, 140]];
/// The outdoor test weather's cloud layer 3 colour by day.
pub const OUTDOOR_CLOUDS: [u8; 3] = [232, 235, 238];
/// The tints (colour and amount) of the outdoor test weather's day and
/// night image space modifiers.
pub const OUTDOOR_DAY_TINT: [f32; 4] = [1.0, 0.74, 0.05, 0.4];
pub const OUTDOOR_NIGHT_TINT: [f32; 4] = [0.2, 0.45, 0.9, 0.6];
/// The outdoor test world's land textures.
pub const DIRT_RGB: [u8; 3] = [150, 110, 70];
pub const ROAD_RGB: [u8; 3] = [60, 60, 60];
/// Terrain heights: square 0,0 rises 8 units per grid step eastward from
/// this; square 1,0 is flat at the height where that ramp ends.
pub const OUTDOOR_BASE_HEIGHT: f32 = 1000.0;
/// The outdoor test world's map marker (persistent, in square 0,0) and
/// where it stands; travelling there arrives at the persistent rock
/// (0xC02, at 5000, 2000, 1256).
pub const MAP_MARKER: u32 = 0xC04;
pub const MAP_MARKER_NAME: &str = "Test Well";
pub const MAP_MARKER_AT: [f32; 3] = [3000.0, 1000.0, 1064.0];
/// The outdoor test world's grass (`GRAS`), which the dirt texture lists:
/// `Test\Grass.nif` (a 32 × 64 upright quad), density 100%, slopes 0–90°,
/// water rule 0, position range 16, height range 0.2, colour range 0.5,
/// wave period 10, uniform scaling and fit to slope.
pub const OUTDOOR_GRASS: u32 = 0xA60;

fn record_flagged(kind: &[u8; 4], id: u32, flags: u32, data: &[u8]) -> Vec<u8> {
    let mut v = record(kind, id, data);
    v[8..12].copy_from_slice(&flags.to_le_bytes());
    v
}

/// A terrain record: heights from `offset` (in units of 8) with one step
/// per point, vertex colours, and texture subrecords.
fn land(id: u32, offset: f32, steps: &[i8], colors: &[[u8; 3]], textures: &[u8]) -> Vec<u8> {
    let mut d = sub(b"DATA", &0x1fu32.to_le_bytes());
    d.extend(sub(b"VNML", &[0u8, 0, 127].repeat(33 * 33)));
    let mut vhgt = offset.to_le_bytes().to_vec();
    vhgt.extend(steps.iter().map(|&s| s as u8));
    vhgt.extend([0u8; 3]);
    d.extend(sub(b"VHGT", &vhgt));
    d.extend(sub(b"VCLR", &colors.concat()));
    d.extend(textures);
    record(b"LAND", id, &d)
}

/// An outdoor worldspace, `TestWorld` (0xC00), with two squares: 0,0
/// (`TestField`, 0xC10: a ramp rising east, a rock, a persistent door to
/// the interior `TestShack` (0xD00), and its south-west quarter painted
/// with dirt under two layers; the dirt grows [`OUTDOOR_GRASS`]) and 1,0
/// (flat, holding a persistent rock).
/// Its climate's one weather lights it.
pub fn outdoors(tag: &str) -> TempData {
    let data = TempData::new(tag);
    data.write(
        "meshes/test/rock.nif",
        &nif(
            &boxed([-50.0, -50.0, 0.0], [50.0, 50.0, 60.0]),
            "textures\\test\\rock.dds",
        ),
    );
    data.write("textures/test/rock.dds", &dds([128, 128, 128]));
    data.write("textures/test/dirt.dds", &dds(DIRT_RGB));
    data.write("textures/test/dirt_n.dds", &dds([128, 128, 255]));
    data.write("textures/test/road.dds", &dds(ROAD_RGB));
    data.write(
        "meshes/test/grass.nif",
        &nif(
            &quad(
                [
                    [-16.0, 0.0, 0.0],
                    [16.0, 0.0, 0.0],
                    [16.0, 0.0, 64.0],
                    [-16.0, 0.0, 64.0],
                ],
                [0.0, -1.0, 0.0],
            ),
            "textures\\test\\grass.dds",
        ),
    );
    data.write("textures/test/grass.dds", &dds([90, 140, 60]));
    data.write(
        "textures/landscape/dirtwasteland01.dds",
        &dds([200, 200, 200]),
    );
    data.write(
        "textures/landscape/dirtwasteland01_n.dds",
        &dds([128, 128, 255]),
    );
    // Distant land for the four cells from 0,0: one flat quad at height
    // 900 in the chunk's own space, which its root node moves to the
    // world (here 100 east, 200 north, 50 up).
    let span = 4.0 * 4096.0;
    data.write(
        "meshes/landscape/lod/testworld/testworld.level4.x0.y0.nif",
        &moved_root_nif(
            &quad(
                [
                    [0.0, 0.0, 900.0],
                    [span, 0.0, 900.0],
                    [span, span, 900.0],
                    [0.0, span, 900.0],
                ],
                [0.0, 0.0, 1.0],
            ),
            "Data\\Textures\\Landscape\\LOD\\TestWorld\\Diffuse\\TestWorld.n.Level4.X0.Y0.dds",
            [100.0, 200.0, 50.0],
        ),
    );
    data.write(
        "textures/landscape/lod/testworld/diffuse/testworld.n.level4.x0.y0.dds",
        &dds(DIRT_RGB),
    );
    // Its quadtree: one level-8 root at 0,0 over level-4 chunks, and that
    // level-8 chunk, whose corners geomorph toward heights 100 lower.
    data.write(
        "lodsettings/testworld.dlodsettings",
        &lod::lod_settings_file(4, 8, 8, (0, 0), (7, 7), 4),
    );
    let span8 = 8.0 * 4096.0;
    data.write(
        "meshes/landscape/lod/testworld/testworld.level8.x0.y0.nif",
        &lod::morphing_chunk_nif(
            &quad(
                [
                    [0.0, 0.0, 900.0],
                    [span8, 0.0, 900.0],
                    [span8, span8, 900.0],
                    [0.0, span8, 900.0],
                ],
                [0.0, 0.0, 1.0],
            ),
            "Data\\Textures\\Landscape\\LOD\\TestWorld\\Diffuse\\TestWorld.n.Level8.X0.Y0.dds",
            [0.0, 0.0, 50.0],
            &[800.0; 4],
        ),
    );

    let mut statics = stat(0xA20, "Rock", "Test\\Rock.nif");
    // The engine's map marker object (FalloutNV.esm's 0x10).
    statics.extend(stat(0x10, "MapMarker", "Markers\\MapMarker.nif"));
    let txst = |id: u32, name: &str, diffuse: &str, normal: Option<&str>| {
        let mut d = sub(b"EDID", &zstr(name));
        d.extend(sub(b"TX00", &zstr(diffuse)));
        if let Some(n) = normal {
            d.extend(sub(b"TX01", &zstr(n)));
        }
        record(b"TXST", id, &d)
    };
    let mut sets = txst(0xA10, "DirtSet", "Test\\Dirt.dds", Some("Test\\Dirt_n.dds"));
    sets.extend(txst(0xA11, "RoadSet", "Test\\Road.dds", None));
    let ltex = |id: u32, name: &str, set: u32, grasses: &[u32]| {
        let mut d = sub(b"EDID", &zstr(name));
        d.extend(sub(b"TNAM", &set.to_le_bytes()));
        d.extend(sub(b"SNAM", &[30]));
        for g in grasses {
            d.extend(sub(b"GNAM", &g.to_le_bytes()));
        }
        record(b"LTEX", id, &d)
    };
    let mut land_textures = ltex(0xA00, "TestDirt", 0xA10, &[OUTDOOR_GRASS]);
    land_textures.extend(ltex(0xA01, "TestRoad", 0xA11, &[]));
    let mut grass = sub(b"EDID", &zstr("TestGrass"));
    grass.extend(sub(b"MODL", &zstr("Test\\Grass.nif")));
    let mut grass_data = vec![100u8, 0, 90, 0, 0, 0, 0, 0];
    grass_data.extend(0u32.to_le_bytes());
    grass_data.extend(f32s(&[16.0, 0.2, 0.5, 10.0]));
    grass_data.extend([0x06, 0, 0, 0]);
    grass.extend(sub(b"DATA", &grass_data));
    let grasses = record(b"GRAS", OUTDOOR_GRASS, &grass);

    // Ten colours for each of six times of day; set the day ones.
    let mut nam0 = vec![0u8; 240];
    // By day and at high noon (time 4) alike, as most of the game's
    // weathers have it.
    let mut set_day = |which: usize, rgb: [u8; 3]| {
        for time in [1, 4] {
            let at = (which * 6 + time) * 4;
            nam0[at..at + 3].copy_from_slice(&rgb);
        }
    };
    set_day(0, OUTDOOR_SKY[0]);
    set_day(1, OUTDOOR_FOG);
    set_day(3, OUTDOOR_AMBIENT);
    set_day(4, OUTDOOR_SUNLIGHT);
    set_day(7, OUTDOOR_SKY[2]);
    set_day(8, OUTDOOR_SKY[1]);
    let mut weather = sub(b"EDID", &zstr("TestWeather"));
    weather.extend(sub(b"NAM0", &nam0));
    weather.extend(sub(
        b"FNAM",
        &f32s(&[100.0, 50_000.0, 50.0, 20_000.0, 0.5, 0.7]),
    ));
    // Clouds as in `NVWastelandGS`: layers 0–2 empty, layer 3 a texture
    // (speeds 52, 0, 0, 65), coloured (232, 235, 238) by day.
    for (kind, texture) in [
        (b"DNAM", "sky\\alpha.dds"),
        (b"CNAM", "sky\\alpha.dds"),
        (b"ANAM", "sky\\alpha.dds"),
        (b"BNAM", "sky\\TestClouds.dds"),
    ] {
        weather.extend(sub(kind, &zstr(texture)));
    }
    weather.extend(sub(b"ONAM", &[52, 0, 0, 65]));
    let mut cloud_colors = vec![0u8; 96];
    cloud_colors[(3 * 6 + 1) * 4..(3 * 6 + 1) * 4 + 3].copy_from_slice(&OUTDOOR_CLOUDS);
    weather.extend(sub(b"PNAM", &cloud_colors));
    // Image space modifiers by time of day (`[n] "IAD"`): day and night,
    // each a tint (see `OUTDOOR_DAY_TINT`, `OUTDOOR_NIGHT_TINT`).
    weather.extend(sub(&[1, b'I', b'A', b'D'], &0xB21u32.to_le_bytes()));
    weather.extend(sub(&[3, b'I', b'A', b'D'], &0xB22u32.to_le_bytes()));
    let weathers = record(b"WTHR", 0xB00, &weather);
    let modifier = |id: u32, name: &str, tint: [f32; 4]| {
        let mut d = sub(b"EDID", &zstr(name));
        d.extend(sub(b"DNAM", &f32s(&[0.0, 1.0])));
        let mut keys = f32s(&[0.0]);
        keys.extend(f32s(&tint));
        d.extend(sub(b"TNAM", &keys));
        record(b"IMAD", id, &d)
    };
    let mut modifiers = modifier(0xB21, "TestDayIS", OUTDOOR_DAY_TINT);
    modifiers.extend(modifier(0xB22, "TestNightIS", OUTDOOR_NIGHT_TINT));
    // A second weather, `TestStorm` (0xB01): rainy (`DATA` byte 11 0x04),
    // fading in over 0.25 h (byte 3 255), no colours of its own.
    let mut storm = sub(b"EDID", &zstr("TestStorm"));
    let mut storm_data = [0u8; 15];
    storm_data[3] = 255;
    storm_data[11] = 0x04;
    storm.extend(sub(b"DATA", &storm_data));
    let mut weathers = weathers;
    weathers.extend(record(b"WTHR", OUTDOOR_STORM, &storm));
    // The climate: `TestWeather` 100, and the storm 50 but only as the
    // global `TestStormy` says (0: never).
    let mut climate = sub(b"EDID", &zstr("TestClimate"));
    let mut wlst = 0xB00u32.to_le_bytes().to_vec();
    wlst.extend(100u32.to_le_bytes());
    wlst.extend(0u32.to_le_bytes());
    wlst.extend(OUTDOOR_STORM.to_le_bytes());
    wlst.extend(50u32.to_le_bytes());
    wlst.extend(OUTDOOR_STORMY.to_le_bytes());
    climate.extend(sub(b"WLST", &wlst));
    climate.extend(sub(b"TNAM", &[36, 48, 108, 120, 0, 0]));
    let climates = record(b"CLMT", 0xB10, &climate);
    let mut stormy = sub(b"EDID", &zstr("TestStormy"));
    stormy.extend(sub(b"FNAM", b"f"));
    stormy.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));
    let globals = record(b"GLOB", OUTDOOR_STORMY, &stormy);
    // A weather region over square 1,0 (x 4096..8192, y 0..4096) whose
    // weather is the storm (`RDAT` type 3, override 0, priority 50).
    let mut region = sub(b"EDID", &zstr("TestStormRegion"));
    region.extend(sub(b"WNAM", &0xC00u32.to_le_bytes()));
    region.extend(sub(b"RPLI", &0u32.to_le_bytes()));
    region.extend(sub(
        b"RPLD",
        &f32s(&[4096.0, 0.0, 8192.0, 0.0, 8192.0, 4096.0, 4096.0, 4096.0]),
    ));
    let mut rdat = 3u32.to_le_bytes().to_vec();
    rdat.extend([0, 50, 0, 0]);
    region.extend(sub(b"RDAT", &rdat));
    let mut rdwt = OUTDOOR_STORM.to_le_bytes().to_vec();
    rdwt.extend(100u32.to_le_bytes());
    rdwt.extend(0u32.to_le_bytes());
    region.extend(sub(b"RDWT", &rdwt));
    let regions = record(b"REGN", OUTDOOR_STORM_REGION, &region);

    // Square 0,0: a ramp rising one step (8 units) per point eastward.
    let mut ramp = vec![0i8; 33 * 33];
    for y in 0..33 {
        for x in 1..33 {
            ramp[y * 33 + x] = 1;
        }
    }
    let mut colors = vec![[255u8, 255, 255]; 33 * 33];
    colors[0] = [128, 64, 32];
    let quarter_texture = |kind: &[u8; 4], texture: u32, quarter: u8, layer: u16| {
        let mut d = texture.to_le_bytes().to_vec();
        d.extend([quarter, 0]);
        d.extend(layer.to_le_bytes());
        sub(kind, &d)
    };
    let vtxt = |points: &[(u16, f32)]| {
        let mut d = Vec::new();
        for &(at, opacity) in points {
            d.extend(at.to_le_bytes());
            d.extend([0u8; 2]);
            d.extend(opacity.to_le_bytes());
        }
        sub(b"VTXT", &d)
    };
    let mut textures = quarter_texture(b"BTXT", 0xA00, 0, 0);
    // Layer 1 (dirt again) is stored before layer 0 (road): layers go by
    // number.
    textures.extend(quarter_texture(b"ATXT", 0xA00, 0, 1));
    textures.extend(vtxt(&[(0, 0.5)]));
    textures.extend(quarter_texture(b"ATXT", 0xA01, 0, 0));
    textures.extend(vtxt(&[(0, 1.0), (18, 0.5)]));
    let base = OUTDOOR_BASE_HEIGHT / 8.0;
    let land0 = land(0xC11, base, &ramp, &colors, &textures);
    let land1 = land(
        0xC21,
        base + 32.0,
        &[0i8; 33 * 33],
        &vec![[255, 255, 255]; 33 * 33],
        &[],
    );

    let rock = |id: u32, flags: u32, pos: [f32; 3]| {
        let mut r = placed(id, 0xA20, pos, [0.0; 3], &[]);
        r[8..12].copy_from_slice(&flags.to_le_bytes());
        r
    };
    let exterior_cell = |id: u32, editor_id: Option<&str>, x: i32, y: i32| {
        let mut d = editor_id
            .map(|e| sub(b"EDID", &zstr(e)))
            .unwrap_or_default();
        d.extend(sub(b"DATA", &[0]));
        let mut xclc = x.to_le_bytes().to_vec();
        xclc.extend(y.to_le_bytes());
        xclc.extend([0u8; 4]);
        d.extend(sub(b"XCLC", &xclc));
        // Square 1,0 lies in the storm region.
        if (x, y) == (1, 0) {
            d.extend(sub(b"XCLR", &OUTDOOR_STORM_REGION.to_le_bytes()));
        }
        record(b"CELL", id, &d)
    };
    let children = |cell: u32, kind: i32, contents: &[u8]| {
        group(
            cell.to_le_bytes(),
            6,
            &group(cell.to_le_bytes(), kind, contents),
        )
    };

    // A door between the field and a shack: each side's `XTEL` names the
    // other door and where the player arrives.
    let door = |id: u32, flags: u32, pos: [f32; 3], to: u32, arrive: [f32; 3]| {
        let mut xtel = to.to_le_bytes().to_vec();
        xtel.extend(f32s(&[arrive[0], arrive[1], arrive[2], 0.0, 0.0, 1.5]));
        xtel.extend(0u32.to_le_bytes());
        let mut r = placed(id, 0xA30, pos, [0.0; 3], &sub(b"XTEL", &xtel));
        r[8..12].copy_from_slice(&flags.to_le_bytes());
        r
    };
    let mut door_base = sub(b"EDID", &zstr("TestDoor"));
    door_base.extend(sub(b"MODL", &zstr("Test\\Rock.nif")));
    let doors = record(b"DOOR", 0xA30, &door_base);

    let mut world_children = {
        let mut d = sub(b"DATA", &[0x02]);
        d.extend(sub(b"XCLC", &[0u8; 12]));
        record_flagged(b"CELL", 0xC01, 0x400, &d)
    };
    let mut persistent = rock(0xC02, 0x400, [5000.0, 2000.0, 1256.0]);
    persistent.extend(door(
        0xC03,
        0x400,
        [2000.0, 2000.0, 1128.0],
        0xD01,
        [0.0, 100.0, 0.0],
    ));
    // A map marker, "Test Well": found within 500 units, travelled to at
    // the persistent rock (its `XLKR`).
    let mut well = sub(b"XMRK", &[]);
    well.extend(sub(b"FNAM", &[0x02]));
    well.extend(sub(b"FULL", &zstr(MAP_MARKER_NAME)));
    well.extend(sub(b"TNAM", &[1, 0]));
    well.extend(sub(b"XRDS", &500f32.to_le_bytes()));
    well.extend(sub(b"XLKR", &0xC02u32.to_le_bytes()));
    let mut marker = placed(MAP_MARKER, 0x10, MAP_MARKER_AT, [0.0; 3], &well);
    marker[8..12].copy_from_slice(&0x400u32.to_le_bytes());
    persistent.extend(marker);
    world_children.extend(children(0xC01, 8, &persistent));
    let mut squares = exterior_cell(0xC10, Some("TestField"), 0, 0);
    let mut temporary = land0;
    temporary.extend(rock(0xC12, 0, [1000.0, 1000.0, 1064.0]));
    squares.extend(children(0xC10, 9, &temporary));
    squares.extend(exterior_cell(0xC20, None, 1, 0));
    squares.extend(children(0xC20, 9, &land1));
    world_children.extend(group([0; 4], 4, &group([0; 4], 5, &squares)));

    let mut world = sub(b"EDID", &zstr("TestWorld"));
    world.extend(sub(b"FULL", &zstr("Test World")));
    world.extend(sub(b"CNAM", &0xB10u32.to_le_bytes()));
    world.extend(sub(b"DNAM", &f32s(&[-2500.0, -2300.0])));
    world.extend(sub(b"DATA", &[0]));
    let mut worlds = record(b"WRLD", 0xC00, &world);
    worlds.extend(group(0xC00u32.to_le_bytes(), 1, &world_children));

    // The shack: an interior with the door back out.
    let mut shack = sub(b"EDID", &zstr("TestShack"));
    shack.extend(sub(b"DATA", &[1]));
    let mut interiors = record(b"CELL", 0xD00, &shack);
    interiors.extend(children(
        0xD00,
        8,
        &door(
            0xD01,
            0x400,
            [0.0, 0.0, 0.0],
            0xC03,
            [2000.0, 1900.0, 1128.0],
        ),
    ));
    let interiors = group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &interiors)),
    );

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    // Finding a place is worth 10 experience, as in the game.
    let mut xp = sub(b"EDID", &zstr("iXPRewardDiscoverMapMarker"));
    xp.extend(sub(b"DATA", &10i32.to_le_bytes()));
    let mut settings = record(b"GMST", 0xA40, &xp);
    // The sun's path as `FalloutNV.esm` sets it.
    for (i, (name, value)) in [
        ("fSunXExtreme", 800.0f32),
        ("fSunYExtreme", -100.0),
        ("fSunZExtreme", -100.0),
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = sub(b"EDID", &zstr(name));
        d.extend(sub(b"DATA", &value.to_le_bytes()));
        settings.extend(record(b"GMST", 0xA41 + i as u32, &d));
    }
    plugin.extend(group(*b"GMST", 0, &settings));
    plugin.extend(group(*b"STAT", 0, &statics));
    plugin.extend(group(*b"DOOR", 0, &doors));
    plugin.extend(group(*b"TXST", 0, &sets));
    plugin.extend(group(*b"GRAS", 0, &grasses));
    plugin.extend(group(*b"LTEX", 0, &land_textures));
    plugin.extend(group(*b"IMAD", 0, &modifiers));
    plugin.extend(group(*b"GLOB", 0, &globals));
    plugin.extend(group(*b"WTHR", 0, &weathers));
    plugin.extend(group(*b"CLMT", 0, &climates));
    plugin.extend(group(*b"REGN", 0, &regions));
    plugin.extend(interiors);
    plugin.extend(group(*b"WRLD", 0, &worlds));
    data.write("FalloutNV.esm", &plugin);
    data
}

/// The script test world's forms.
pub mod quest_ids {
    pub const GLOBAL: u32 = 0xA00;
    pub const MESSAGE: u32 = 0xA01;
    pub const QUEST_SCRIPT: u32 = 0xA02;
    pub const QUEST: u32 = 0xA03;
    pub const CAPS: u32 = 0xA04;
    pub const DOC: u32 = 0xA05;
    pub const DOC_SCRIPT: u32 = 0xA06;
    pub const DOOR_REF: u32 = 0xA07;
    pub const DOC_REF: u32 = 0xA08;
    pub const CELL: u32 = 0xA09;
    pub const DOOR: u32 = 0xA0A;
    pub const VOICE: u32 = 0xA0B;
    pub const GREETING_LINE: u32 = 0xA0C;
    pub const OTHER_LINE: u32 = 0xA0D;
    /// Top-level topics the doctor answers: "Tell me about yourself."
    /// (priority 60, teaches the secret) and "What's this town?" (90, its
    /// line's prompt "Where am I?").
    pub const TOPIC_ABOUT: u32 = 0xA0E;
    pub const ABOUT_LINE: u32 = 0xA0F;
    pub const TOPIC_TOWN: u32 = 0xA15;
    pub const TOWN_LINE: u32 = 0xA16;
    /// A Speech 25 check (not top-level): its passing and failing lines.
    pub const TOPIC_CHECK: u32 = 0xA70;
    pub const CHECK_PASSED: u32 = 0xA71;
    pub const CHECK_FAILED: u32 = 0xA72;
    /// A topic with no lines of its own: its `INFC` list names the secret's
    /// line, which stands under another topic.
    pub const TOPIC_LINKED: u32 = 0xA1B;
    /// Experience settings (three forms from here): picking an easy lock
    /// 30, hacking an easy terminal 30, finding a place 10.
    pub const XP_SETTINGS: u32 = 0xA73;
    /// The clock's globals, `GameHour` (10) and `TimeScale` (30), from here.
    pub const GAME_HOUR: u32 = 0xA76;
    /// Not top-level: offered once learned.
    pub const TOPIC_SECRET: u32 = 0xA17;
    pub const SECRET_LINE: u32 = 0xA18;
    /// Top-level, priority 99, but nobody here has a line for it.
    pub const TOPIC_NOBODY: u32 = 0xA19;
    pub const NOBODY_LINE: u32 = 0xA1A;
    pub const CHEST: u32 = 0xA10;
    pub const CHEST_REF: u32 = 0xA11;
    pub const CAPS_REF: u32 = 0xA12;
    pub const CUP: u32 = 0xA13;
    /// A leveled list: 2 cups at level 1, 100 caps from level 5.
    pub const LEVELED: u32 = 0xA14;
    /// The doctor's package: travel to the door from stage 10.
    pub const TRAVEL: u32 = 0xA20;
    pub const NAVMESH: u32 = 0xA21;
    /// A second cell east of the first, whose navmesh joins the first's
    /// along x = 200.
    pub const CELL2: u32 = 0xA22;
    pub const NAVMESH2: u32 = 0xA23;
    /// A region `TestCell` lists (`XCLR`).
    pub const REGION: u32 = 0xA24;
    /// Map markers in `TestCell`: one shown from the start, one not.
    pub const MARKER: u32 = 0xA25;
    pub const HIDDEN_MARKER: u32 = 0xA26;
    /// A message box (`TestChoice`): buttons 0 "First", 1 "Hidden" (only
    /// for the doctor), 2 "Third"; its text has two `%` values.
    pub const CHOICE: u32 = 0xA27;
    /// Game settings: `fAVDSkillSmallGunsBase` 2, `fAVDTagSkillBonus` 15,
    /// `iTraitMenuMaxNumTraits` 2 (three forms from here).
    pub const SETTINGS: u32 = 0xA28;
    /// A trait and a perk that isn't one.
    pub const TRAIT: u32 = 0xA2B;
    pub const PERK: u32 = 0xA2C;
    /// Levelling settings (`iXPBumpBase` 150, `iXPDeathRewardHealthThreshold`
    /// 40, `iMaxCharacterLevel` 30, `fAVDHealthLevelMult` 5; four forms
    /// from here), and perks with
    /// entry points: `QuickStudy` (experience × 1.1, from level 2),
    /// `Schooled` (skill points + 2, from level 4), `LadiesOnly` (women
    /// only).
    pub const LEVEL_SETTINGS: u32 = 0xA60;
    /// The Speech skill's record (`AVSpeech`, "Speech"): what the check's
    /// lines name in `KNAM`.
    pub const SPEECH_SKILL: u32 = 0xA69;
    /// `fAVDHealRateEndurance9Bonus` 10.
    pub const HEAL_RATE_SETTING: u32 = 0xA6A;
    /// A Science skill book (`TestScienceBook`), `fBookPerkBonus` 3, and a
    /// perk adding 1 to book points (`Bookworm`, entry point 11).
    pub const SCIENCE_BOOK: u32 = 0xA6B;
    pub const BOOK_SETTING: u32 = 0xA6C;
    pub const BOOKWORM: u32 = 0xA6D;
    /// `ToughSkin`: the ability `TestAbility`, and + 3 damage threshold
    /// for women (an entry point with a holder condition).
    pub const TOUGH_SKIN: u32 = 0xA6E;
    /// A reputation, `RepTestville` ("Testville"), most 20.
    pub const REPUTATION: u32 = 0xA6F;
    /// A hollow-point round (`TestHollowPoint`) with two ammunition
    /// effects, as the game's 5.56mm one: damage × 1.75, the target's DT ×
    /// 3.
    pub const HOLLOW_POINT: u32 = 0xA78;
    pub const HP_DAMAGE: u32 = 0xA79;
    pub const HP_THRESHOLD: u32 = 0xA7A;
    /// Body part data: people's (`DefaultBodyPartData`, the game's fixed
    /// form: head ×2 at 20% of health, torso 60%, arms and legs 25%), the
    /// player's (`PlayerBodyPartData`: head ×1 at 75%, legs 150%), and the
    /// gecko's own (`TestGeckoParts`, its `PNAM`: head ×2 at 25%, torso
    /// from `Bip01 Spine1` at 75%). Nodes as the game's skeletons name
    /// them.
    pub const DEFAULT_BODY_PARTS: u32 = 0x1D;
    pub const PLAYER_BODY_PARTS: u32 = 0x1C;
    pub const GECKO_BODY_PARTS: u32 = 0xA7B;
    /// A two-handed rifle (`TestRifle`, animation type 5, damage 20).
    pub const RIFLE: u32 = 0xA7C;
    /// `TestStimpak`: 30 at once of a "value and parts" effect (archetype
    /// 34, health), as the game's Stimpak restores health and limbs.
    pub const STIMPAK: u32 = 0xA7D;
    pub const VAC_EFFECT: u32 = 0xA7E;
    pub const QUICK_STUDY: u32 = 0xA66;
    pub const SCHOOLED: u32 = 0xA67;
    pub const LADIES_ONLY: u32 = 0xA68;
    /// An image space modifier (`TestFlash`, 2 s): a white fade that
    /// clears by half way, bright scale × 2 → 1 and + 0 → 0.5.
    pub const FLASH: u32 = 0xA2D;
    /// The game's barter settings and `fAVDSkillBarterBase` (five forms
    /// from here). `DocRef`'s merchant container (`XMRC`) is `ChestRef`.
    pub const BARTER_SETTINGS: u32 = 0xA30;
    /// Damage settings (five forms from here).
    pub const COMBAT_SETTINGS: u32 = 0xA38;
    /// A pistol (damage 16, Guns), a gecko (health 30, bite 8; dying adds
    /// 100 to `TestGlobal`) placed as `GeckoRef`, and a bottle
    /// (`BottleRef`) whose `OnHitWith TestPistol` adds 1.
    pub const PISTOL: u32 = 0xA40;
    pub const GECKO: u32 = 0xA41;
    pub const GECKO_SCRIPT: u32 = 0xA42;
    pub const GECKO_REF: u32 = 0xA43;
    pub const BOTTLE: u32 = 0xA44;
    pub const BOTTLE_SCRIPT: u32 = 0xA45;
    pub const BOTTLE_REF: u32 = 0xA46;
    /// A healing item (`TestMedicine`: +30 health at once) and its effect.
    pub const MEDICINE: u32 = 0xA47;
    pub const HEAL_EFFECT: u32 = 0xA48;
    /// Clothes: a shirt (upper body), a coat (upper body and left hand), a
    /// hat (its own slot).
    pub const SHIRT: u32 = 0xA49;
    pub const COAT: u32 = 0xA4A;
    pub const HAT: u32 = 0xA4B;
    /// A faction the doctor (aggressive) is in, with no relation to the
    /// player.
    pub const RAIDERS: u32 = 0xA4C;
    /// `TestTonic`: +2 Strength for 10 s (a recovering effect), 3 health
    /// a second for 4 s (`HEAL_EFFECT`), 10 rads at once (detrimental,
    /// resisted by Rad Resistance) and a script effect that gives a cap.
    pub const TONIC: u32 = 0xA4D;
    pub const STRENGTH_EFFECT: u32 = 0xA4E;
    pub const RADS_EFFECT: u32 = 0xA4F;
    pub const CAP_EFFECT: u32 = 0xA50;
    pub const CAP_SCRIPT: u32 = 0xA51;
    /// Spells: `TestSpell` (+1 Strength for 100 s) and `TestAbility` (an
    /// ability: +3 Strength while held).
    pub const SPELL: u32 = 0xA52;
    pub const ABILITY: u32 = 0xA53;
    /// A chair (`FURN`, no model file) placed as `ChairRef`.
    pub const CHAIR: u32 = 0xA54;
    pub const CHAIR_REF: u32 = 0xA55;
    /// A load door pair between `TestCell` (at 150,150) and `TestCell2`
    /// (at 450,0), each arriving by the other; a marker in `TestCell2`
    /// (`FarMarkerRef`, 500,100) and a travel package to it.
    pub const LINK_DOOR: u32 = 0xA56;
    pub const LINK_DOOR2: u32 = 0xA57;
    pub const FAR_MARKER: u32 = 0xA58;
    pub const FAR_TRAVEL: u32 = 0xA59;
    /// A follow package: the player, within 200.
    pub const FOLLOW_PLAYER: u32 = 0xA5A;
    /// A strongbox (`StrongboxRef`, the chest's base) locked at level 50
    /// with a key (`TestKey`); a terminal (`TestTerminal`, locked, very
    /// easy) placed as `TerminalRef` and linked to it, whose items unlock
    /// its link and show a note (`TestNote`).
    pub const STRONGBOX_REF: u32 = 0xA5B;
    pub const KEY: u32 = 0xA5C;
    pub const TERMINAL: u32 = 0xA5D;
    pub const TERMINAL_REF: u32 = 0xA5E;
    pub const NOTE: u32 = 0xA5F;
    /// The player's base record, a fixed form in every game.
    pub const PLAYER: u32 = 0x7;
    /// The `GREETING` topic, a fixed form in every game.
    pub const GREETING: u32 = 0xC8;
    /// Perks laid out as the game's, one entry point each (see
    /// `world::perks`): `Cowhand` (entry 0 × 1.25 with the weapons in
    /// `CowhandList`: the pistol; as Cowboy), `BeastSlayer` (0 × 1.5
    /// against creatures, tab 2 `GetIsCreature`; as Entomologist's shape),
    /// `Piercer` (58 set 15 with melee and unarmed weapons; as Piercing
    /// Strike), `Bulwark` (56 + 5 against melee and unarmed attacks, tab 2
    /// the attacker's weapon; as Stonewall), `Marksman` (1 + 10 with the
    /// list's weapons; as Laser Commander), `BetterCrits` (2 × 1.5; as
    /// Better Criticals), `DreamCrusher` (36 × 0.5), `Chemist` (25 × 2),
    /// `LeadBelly` (44 × 0.5), `BetterHealing` (12 × 1.2),
    /// `SilentRunning` (31 set 1), `TravelLight` (42 × 1.1 unless the coat
    /// is worn, `GetEquipped TestCoat != 1`), `PackRat` (55 set 1),
    /// `HandLoader` (57 × 2 with Guns), `RapidReload` (37 × 1.25),
    /// `LongHaul` (51 set 1), `FastShot` (43 × 1.2 with Guns or Energy
    /// Weapons), `BuiltToDestroy` (68 × 1.15, a trait), `Adamantium` (6 ×
    /// 0.5 against gun attacks: tab 2 `IsWeaponSkillType` Guns).
    pub const COWHAND: u32 = 0xA80;
    pub const COWHAND_LIST: u32 = 0xA81;
    pub const BEAST_SLAYER: u32 = 0xA82;
    pub const PIERCER: u32 = 0xA83;
    pub const BULWARK: u32 = 0xA84;
    pub const MARKSMAN: u32 = 0xA85;
    pub const BETTER_CRITS: u32 = 0xA86;
    pub const DREAM_CRUSHER: u32 = 0xA87;
    pub const CHEMIST: u32 = 0xA88;
    pub const LEAD_BELLY: u32 = 0xA89;
    pub const BETTER_HEALING: u32 = 0xA8A;
    pub const SILENT_RUNNING: u32 = 0xA8B;
    pub const TRAVEL_LIGHT: u32 = 0xA8C;
    pub const PACK_RAT: u32 = 0xA8D;
    pub const HAND_LOADER: u32 = 0xA8E;
    pub const RAPID_RELOAD: u32 = 0xA8F;
    pub const LONG_HAUL: u32 = 0xA90;
    pub const FAST_SHOT: u32 = 0xA91;
    pub const BUILT_TO_DESTROY: u32 = 0xA92;
    pub const ADAMANTIUM: u32 = 0xA93;
    /// `TestFood` (`ENIT` flag 0x02 food: 3 health a second for 4 s, 10
    /// rads at once, −1 Strength for 10 s from a hostile effect
    /// `TestHostileEffect`) and `TestChem` (0x04 medicine: 30 health at
    /// once, +2 Strength for 10 s).
    pub const FOOD: u32 = 0xA95;
    pub const HOSTILE_EFFECT: u32 = 0xA96;
    pub const CHEM: u32 = 0xA94;
    /// `TestCasedRound`: an ammunition whose `DAT2` leaves `TestCase` 25%
    /// of the time (as the 9mm round's "Case, 9mm").
    pub const CASED_AMMO: u32 = 0xA97;
    pub const CASE: u32 = 0xA98;
    /// A bundle of cases whose script, on reaching the player, gives 25
    /// cases and removes itself (as the game's `Case10mmAddScript`).
    pub const CASE_BUNDLE: u32 = 0xAF0;
    pub const CASE_BUNDLE_SCRIPT: u32 = 0xAF1;
    /// A bundle whose `OnAdd` only marks it and whose `GameMode` gives 5
    /// cases and removes it (as the game's `PrimerShotshellAddScript`).
    pub const LATE_BUNDLE: u32 = 0xAF2;
    pub const LATE_BUNDLE_SCRIPT: u32 = 0xAF3;
    /// A hat whose script sets `TestGlobal` to 99 when the player puts it
    /// on and 7 when they take it off (as the faction outfits' warnings).
    pub const SCRIPTED_HAT: u32 = 0xAF4;
    pub const SCRIPTED_HAT_SCRIPT: u32 = 0xAF5;
    /// A hat whose script removes it as it's put on (`OnEquip Player`,
    /// `RemoveMe`): which of two is taken, the worn one or the other.
    pub const VANISHING_HAT: u32 = 0xB70;
    pub const VANISHING_HAT_SCRIPT: u32 = 0xB71;
    /// The pistol's repair list (`REPL`: `TestPistolRepairList`, the
    /// rifle), the game data's repair settings (`fRepairSkillMax` 10,
    /// `fItemRepairCostMult` 2; two forms from here), `JuryRigging` (entry
    /// point 48 "Has Jury Rigging" set 1, as the game's perk) and a
    /// revolver like the pistol (one-handed pistol, Guns; value 50, health
    /// 100) that isn't on its list.
    pub const REPAIR_LIST: u32 = 0xAF6;
    pub const REPAIR_SETTINGS: u32 = 0xAF7;
    pub const JURY_RIGGING: u32 = 0xAF9;
    pub const REVOLVER: u32 = 0xAFA;
    /// Armour as the game's (`DATA` value, health, weight; `DNAM` DR 0 and
    /// DT): `TestArmor` on the upper body (DT 10, health 100, value 100),
    /// `TestHelmet` on the head (DT 4, health 50).
    pub const ARMOR: u32 = 0xAFB;
    pub const HELMET: u32 = 0xAFC;
    /// Things for dropping: `TestKeepsake`, whose `OnDrop Player` sets
    /// `TestGlobal` to 42 (as the Platinum Chip's removes its note), and
    /// `TestRefusedGift`, whose `OnAdd DocRef` drops it (as the companions'
    /// faction outfits' `DropMe`).
    pub const KEEPSAKE: u32 = 0xAFD;
    pub const KEEPSAKE_SCRIPT: u32 = 0xAFE;
    pub const REFUSED_GIFT: u32 = 0xAFF;
    pub const REFUSED_GIFT_SCRIPT: u32 = 0xB20;
    /// The companion wheel's: `FollowersWait`, Doc's line for it ("I'll sit
    /// tight.", begin `set DocRef.Waiting to 1`, end `set TestGlobal to
    /// 7`, as Boone's), `Regenerating` and his line for it ("Much
    /// better."), and the default objects (`DOBJ`, the Stimpak first:
    /// `TestStimpak`). `TestDocScript` has the four variables the wheel
    /// reads.
    pub const FOLLOWERS_WAIT: u32 = 0xB50;
    pub const WAIT_LINE: u32 = 0xB51;
    pub const REGENERATING: u32 = 0xB52;
    pub const REGENERATING_LINE: u32 = 0xB53;
    pub const DEFAULT_OBJECTS: u32 = 0xB54;
    /// Weapon mods: `TestModGun` (damage 10, clip 8, weight 3, health 100,
    /// value 100) with three slots: `TestExtMag` (clip +7), `TestBarrel`
    /// (damage +5) and `TestLightFrame` (weight −1), each worth 20.
    pub const MOD_GUN: u32 = 0xB5A;
    pub const EXT_MAG: u32 = 0xB5B;
    pub const BARREL: u32 = 0xB5C;
    pub const LIGHT_FRAME: u32 = 0xB5D;
    /// Its models: `MODL` `Weapons\ModGun.NIF`, `MWD1`
    /// `Weapons\ModGunExt.NIF`, `MWD3` `Weapons\ModGunExtBarrel.NIF`;
    /// first person `WNAM` `1stPersonModGun` (`Weapons\1stModGun.NIF`),
    /// `WNM1` `1stPersonModGunExt` (`Weapons\1stModGunExt.NIF`).
    pub const FIRST_PERSON_GUN: u32 = 0xB5E;
    pub const FIRST_PERSON_GUN_EXT: u32 = 0xB5F;
    /// Noise: `TestLoudGun` (a pistol, `VNAM` 0 loud) whose first slot is
    /// `TestSilencer` (effect 11) and second `TestBeamSplitter` (effect 12,
    /// +2 projectiles, cone × 0.5; min spread 2°), and `TestKnife`
    /// (one-handed melee).
    /// The gecko's `NAM5` is 0 (loud).
    pub const LOUD_GUN: u32 = 0xB60;
    pub const SILENCER: u32 = 0xB61;
    pub const KNIFE: u32 = 0xB62;
    pub const BEAM_SPLITTER: u32 = 0xB63;
    /// Caravan: `TestCardAce` (hearts ace), `TestCardQueen` (spades queen),
    /// both with `TestCardScript` (the game's `CardAddToPlayerScript`:
    /// `OnAdd`, held by the player, `AddCardToPlayer` and `RemoveMe`), and
    /// `TestCaravanDeck` with both.
    pub const CARD_ACE: u32 = 0xB55;
    pub const CARD_QUEEN: u32 = 0xB56;
    pub const CARD_SCRIPT: u32 = 0xB57;
    pub const CARAVAN_DECK: u32 = 0xB58;
    /// A casino laid out as FalloutNV.esm's are (`CSNO`: eight `MODL`s,
    /// `MOD2` to `MOD4`, seven `ICON`s, four `ICO2`s, a 0x38-byte `DATA`):
    /// `TestCasino` "Gommorah" with Gomorrah's numbers (shuffle 0.2,
    /// payout 1.5, two stops each, three decks, limit 9000, `TestQuest` its
    /// comps quest), played with `TestChip` (`CHIP`, no `DATA`).
    pub const CASINO: u32 = 0xB64;
    pub const CHIP: u32 = 0xB65;
    /// Settings the perks' rules read (seven forms from here):
    /// `fPackRatThreshold` 2, `fPackRatModifier` 0.5, `fAgilityReloadBase`
    /// 5, `fAgilityReloadModifier` 0.1, `fDamageToWeaponValue` 0.2,
    /// `fMagicMedicineSkillMult` 2, `fMagicSurvivalSkillMult` 2.
    pub const PERK_SETTINGS: u32 = 0xA99;
}

/// A body part data record's data (`BPTD`): `EDID`, the skeleton (`MODL`),
/// then per part (name, node, part type, damage multiplier, health %,
/// actor value) its subrecords as the game's records lay them out (`BPTN`,
/// `BPNN`, `BPNT`, `BPNI`, `BPND` of 84 bytes, `NAM1`, `NAM4`, `NAM5`).
pub fn body_part_data(editor_id: &str, parts: &[(&str, &str, u8, f32, u8, i8)]) -> Vec<u8> {
    let mut d = sub(b"EDID", &zstr(editor_id));
    d.extend(sub(b"MODL", &zstr("Characters\\_Male\\skeleton.NIF")));
    for &(name, node, kind, mult, health, value) in parts {
        d.extend(sub(b"BPTN", &zstr(name)));
        d.extend(sub(b"BPNN", &zstr(node)));
        d.extend(sub(b"BPNT", &zstr(node)));
        d.extend(sub(b"BPNI", &zstr(node)));
        let mut bpnd = vec![0u8; 84];
        bpnd[0..4].copy_from_slice(&mult.to_le_bytes());
        bpnd[4] = 0x09; // severable, explodable
        bpnd[5] = kind;
        bpnd[6] = health;
        bpnd[7] = value as u8;
        d.extend(sub(b"BPND", &bpnd));
        d.extend(sub(b"NAM1", &[0]));
        d.extend(sub(b"NAM4", &zstr(node)));
        d.extend(sub(b"NAM5", &[]));
    }
    d
}

/// One condition (`CTDA`, 28 bytes): `function(param1, param2) == value`,
/// asked about the subject.
pub fn condition(function: u16, params: [u32; 2], value: f32) -> Vec<u8> {
    condition_with(comparison::EQUAL, false, function, params, value)
}

/// A condition's comparison, in `CTDA` byte 0's top three bits.
pub mod comparison {
    pub const EQUAL: u8 = 0x00;
    pub const NOT_EQUAL: u8 = 0x20;
    pub const GREATER: u8 = 0x40;
    pub const LESS: u8 = 0x80;
}

/// [`condition`] with its comparison and whether it's joined to the next
/// by OR (`CTDA` byte 0 bit 0x01).
pub fn condition_with(
    comparison: u8,
    or: bool,
    function: u16,
    params: [u32; 2],
    value: f32,
) -> Vec<u8> {
    let mut d = vec![comparison | u8::from(or), 0, 0, 0];
    d.extend(value.to_le_bytes());
    d.extend(function.to_le_bytes());
    d.extend([0; 2]);
    d.extend(params[0].to_le_bytes());
    d.extend(params[1].to_le_bytes());
    d.extend([0; 8]);
    sub(b"CTDA", &d)
}

/// A world for scripts: the quest `TestQuest` (running from the start,
/// its script run every second) with objective 10 and stages 10 (shows
/// the objective and `TestMessage`) and 20 (two log entries: one whose
/// condition fails, then one that completes the quest, completes the
/// objective, sets `TestGlobal` to 7, enables the door and gives the
/// player 25 caps). The quest script sets stage 20 three seconds after
/// stage 10. `TestDoc` (placed as `DocRef` in `TestCell`) has a script that
/// counts activations in `iTalked`, and a greeting (said once) whose
/// result scripts set stage 10 and add 1 to `TestGlobal`; `DoorRef` starts
/// disabled. The cell also has a chest (`ChestRef`, 10 caps and a leveled
/// list giving 2 cups at level 1) and 5 caps on the floor; the player
/// starts with 3.
pub fn quests(tag: &str) -> TempData {
    use quest_ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let script = |id: u32, name: &str, source: &str| {
        let mut d = edid(name);
        d.extend(sub(b"SCHR", &[0; 20]));
        d.extend(sub(b"SCTX", source.as_bytes()));
        record(b"SCPT", id, &d)
    };

    let mut glob = edid("TestGlobal");
    glob.extend(sub(b"FNAM", b"s"));
    glob.extend(sub(b"FLTV", &5.0f32.to_le_bytes()));
    let mut mesg = edid("TestMessage");
    mesg.extend(sub(b"DESC", &zstr("Hello there.")));
    mesg.extend(sub(b"FULL", &zstr("Note")));
    let mut caps = edid("Caps001");
    caps.extend(sub(b"FULL", &zstr("Bottle Cap")));

    let mut scripts = script(
        CASE_BUNDLE_SCRIPT,
        "TestCaseBundleScript",
        "scn TestCaseBundleScript\nBegin OnAdd Player\n\tPlayer.AddItem TestCase 25\n\tRemoveMe\nEnd\n",
    );
    scripts.extend(script(
        CARD_SCRIPT,
        "TestCardScript",
        "scn TestCardScript
begin OnAdd
	if GetContainer != player
		return
	else
		AddCardToPlayer
		RemoveMe
	endif
end
",
    ));
    scripts.extend(script(
        KEEPSAKE_SCRIPT,
        "TestKeepsakeScript",
        "scn TestKeepsakeScript
Begin OnDrop Player
	set TestGlobal to 42
End
",
    ));
    scripts.extend(script(
        REFUSED_GIFT_SCRIPT,
        "TestRefusedGiftScript",
        "scn TestRefusedGiftScript
Begin OnAdd DocRef
	DropMe
End
",
    ));
    scripts.extend(script(
        LATE_BUNDLE_SCRIPT,
        "TestLateBundleScript",
        "scn TestLateBundleScript\nint killme\nBegin OnAdd Player\n\tset killme to 1\nEnd\nBegin GameMode\n\tif killme == 1\n\t\tPlayer.AddItem TestCase 5\n\t\tRemoveMe\n\tendif\nEnd\n",
    ));
    scripts.extend(script(
        VANISHING_HAT_SCRIPT,
        "TestVanishingHatScript",
        "scn TestVanishingHatScript\nBegin OnEquip Player\n\tRemoveMe\nEnd\n",
    ));
    scripts.extend(script(
        SCRIPTED_HAT_SCRIPT,
        "TestScriptedHatScript",
        "scn TestScriptedHatScript\nBegin OnEquip Player\n\tset TestGlobal to 99\nEnd\nBegin OnUnequip Player\n\tset TestGlobal to 7\nEnd\n",
    ));
    scripts.extend(script(
        QUEST_SCRIPT,
        "TestQuestScript",
        "scn TestQuestScript\n\
         short bRunTimer\n\
         float fTimer\n\
         Begin GameMode\n\
         \tif GetStage TestQuest == 10 && bRunTimer == 0\n\
         \t\tset bRunTimer to 1\n\
         \t\tset fTimer to 3\n\
         \telseif bRunTimer == 1\n\
         \t\tset fTimer to fTimer - GetSecondsPassed\n\
         \t\tif fTimer <= 0\n\
         \t\t\tset bRunTimer to 2\n\
         \t\t\tSetStage TestQuest 20\n\
         \t\tendif\n\
         \tendif\n\
         End",
    ));
    scripts.extend(script(
        GECKO_SCRIPT,
        "TestGeckoScript",
        "scn TestGeckoScript\nBegin OnDeath\n\tset TestGlobal to TestGlobal + 100\nEnd",
    ));
    scripts.extend(script(
        BOTTLE_SCRIPT,
        "TestBottleScript",
        "scn TestBottleScript\nBegin OnHitWith TestPistol\n\tset TestGlobal to TestGlobal + 1\nEnd",
    ));
    scripts.extend(script(
        CAP_SCRIPT,
        "TestCapEffectScript",
        "scn TestCapEffectScript\nbegin ScriptEffectStart\n\tadditem Caps001 1\nend",
    ));
    scripts.extend(script(
        DOC_SCRIPT,
        "TestDocScript",
        "scn TestDocScript\nshort iTalked\nshort iByDoor\n\
         short Waiting\nshort FollowerSwitchAggressive\nshort IsFollowingLong\n\
         short CombatStyleRanged\n\
         Begin OnActivate Player\n\tset iTalked to iTalked + 1\nEnd\n\
         Begin OnActivate DoorRef\n\tset iByDoor to 1\nEnd",
    ));

    let mut quest = edid("TestQuest");
    quest.extend(sub(b"SCRI", &QUEST_SCRIPT.to_le_bytes()));
    quest.extend(sub(b"FULL", &zstr("Test Quest")));
    let mut qdata = vec![0x01, 50, 0, 0];
    qdata.extend(1.0f32.to_le_bytes());
    quest.extend(sub(b"DATA", &qdata));
    // The quest's own condition: its dialogue is the doctor's.
    quest.extend(condition(72, [DOC, 0], 1.0));
    quest.extend(sub(b"INDX", &10i16.to_le_bytes()));
    quest.extend(sub(b"QSDT", &[0]));
    quest.extend(sub(b"CNAM", &zstr("Talked to the doctor.")));
    quest.extend(sub(b"SCHR", &[0; 20]));
    quest.extend(sub(
        b"SCTX",
        b"SetObjectiveDisplayed TestQuest 10 1\nShowMessage TestMessage",
    ));
    quest.extend(sub(b"INDX", &20i16.to_le_bytes()));
    // Never used: its condition (stage 99 done) fails.
    quest.extend(sub(b"QSDT", &[0]));
    quest.extend(condition(59, [QUEST, 99], 1.0));
    quest.extend(sub(b"CNAM", &zstr("Never.")));
    quest.extend(sub(b"QSDT", &[0x01]));
    quest.extend(sub(b"CNAM", &zstr("Done.")));
    quest.extend(sub(b"SCHR", &[0; 20]));
    quest.extend(sub(
        b"SCTX",
        b"SetObjectiveCompleted TestQuest 10 1\nset TestGlobal to 7\n\
          DoorRef.Enable\nPlayer.AddItem Caps001 25",
    ));
    quest.extend(sub(b"QOBJ", &10i32.to_le_bytes()));
    quest.extend(sub(b"NNAM", &zstr("Talk to the doctor.")));

    let mut voice = edid("TestVoice");
    voice.extend(sub(b"DNAM", &[0]));
    let mut doc = edid("TestDoc");
    doc.extend(sub(b"FULL", &zstr("Doc")));
    doc.extend(sub(b"ACBS", &[0; 24]));
    doc.extend(sub(b"VTCK", &VOICE.to_le_bytes()));
    doc.extend(sub(b"SCRI", &DOC_SCRIPT.to_le_bytes()));
    doc.extend(sub(b"PKID", &TRAVEL.to_le_bytes()));
    // In the raiders' faction (rank 0), and aggressive (AIDT byte 0).
    let mut membership = RAIDERS.to_le_bytes().to_vec();
    membership.extend([0; 4]);
    doc.extend(sub(b"SNAM", &membership));
    let mut aidt = vec![0u8; 20];
    aidt[0] = 1;
    doc.extend(sub(b"AIDT", &aidt));
    let mut doc_data = 100i32.to_le_bytes().to_vec();
    doc_data.extend([5, 6, 7, 8, 9, 10, 3]);
    doc.extend(sub(b"DATA", &doc_data));
    let mut door = edid("TestDoor");
    door.extend(sub(b"MODL", &zstr("test\\door.nif")));
    let content = |item: u32, n: i32| {
        let mut d = item.to_le_bytes().to_vec();
        d.extend(n.to_le_bytes());
        sub(b"CNTO", &d)
    };
    let mut chest = edid("TestChest");
    chest.extend(sub(b"FULL", &zstr("Chest")));
    chest.extend(content(CAPS, 10));
    chest.extend(content(LEVELED, 1));
    let mut cup = edid("TestCup");
    cup.extend(sub(b"FULL", &zstr("Cup")));
    // Value 2, weight 0.5.
    let mut cup_data = 2i32.to_le_bytes().to_vec();
    cup_data.extend(0.5f32.to_le_bytes());
    cup.extend(sub(b"DATA", &cup_data));
    let entry = |level: u16, form: u32, count: u16| {
        let mut d = level.to_le_bytes().to_vec();
        d.extend([0; 2]);
        d.extend(form.to_le_bytes());
        d.extend(count.to_le_bytes());
        d.extend([0; 2]);
        sub(b"LVLO", &d)
    };
    // Travel (type 6) to the door, any time, once TestQuest's stage is at
    // least 10 (comparison 0x60, "greater or equal").
    let mut travel = edid("TestTravelPackage");
    travel.extend(sub(b"PKDT", &[0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0]));
    let mut pldt = 0u32.to_le_bytes().to_vec();
    pldt.extend(DOOR_REF.to_le_bytes());
    pldt.extend(0u32.to_le_bytes());
    travel.extend(sub(b"PLDT", &pldt));
    travel.extend(sub(b"PSDT", &[0xFF, 0xFF, 0, 0xFF, 0, 0, 0, 0]));
    let mut at_least = condition(58, [QUEST, 0], 10.0);
    at_least[6] = 0x60;
    travel.extend(at_least);
    // Travel to the far marker, in the second cell.
    let mut far_travel = edid("TestFarTravelPackage");
    far_travel.extend(sub(b"PKDT", &[0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0]));
    let mut far_pldt = 0u32.to_le_bytes().to_vec();
    far_pldt.extend(FAR_MARKER.to_le_bytes());
    far_pldt.extend(0u32.to_le_bytes());
    far_travel.extend(sub(b"PLDT", &far_pldt));
    far_travel.extend(sub(b"PSDT", &[0xFF, 0xFF, 0, 0xFF, 0, 0, 0, 0]));
    // Follow the player, staying within 200.
    let mut follow = edid("TestFollowPlayerPackage");
    follow.extend(sub(b"PKDT", &[0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]));
    follow.extend(sub(b"PSDT", &[0xFF, 0xFF, 0, 0xFF, 0, 0, 0, 0]));
    let mut ptdt = 0u32.to_le_bytes().to_vec();
    ptdt.extend(0x14u32.to_le_bytes());
    ptdt.extend(200u32.to_le_bytes());
    ptdt.extend([0; 4]);
    follow.extend(sub(b"PTDT", &ptdt));
    let mut leveled = edid("TestLeveledLoot");
    leveled.extend(sub(b"LVLD", &[0]));
    leveled.extend(sub(b"LVLF", &[0]));
    leveled.extend(entry(1, CUP, 2));
    leveled.extend(entry(5, CAPS, 100));
    let mut player = edid("Player");
    player.extend(sub(b"ACBS", &[0; 24]));
    // Health 100, SPECIAL 5 each.
    let mut player_data = 100i32.to_le_bytes().to_vec();
    player_data.extend([5; 7]);
    player.extend(sub(b"DATA", &player_data));
    player.extend(content(CAPS, 3));
    // A message box: three buttons, the second only for the doctor (so
    // never shown to the player).
    let mut choice = edid("TestChoice");
    choice.extend(sub(b"DESC", &zstr("You have %.0f caps (%g%%).")));
    choice.extend(sub(b"FULL", &zstr("Choose")));
    choice.extend(sub(b"DNAM", &1u32.to_le_bytes()));
    choice.extend(sub(b"ITXT", &zstr("First")));
    choice.extend(sub(b"ITXT", &zstr("Hidden")));
    choice.extend(condition(72, [DOC, 0], 1.0));
    choice.extend(sub(b"ITXT", &zstr("Third")));
    // Game settings the character's numbers come from.
    let setting = |id: u32, name: &str, value: &[u8]| {
        let mut d = edid(name);
        d.extend(sub(b"DATA", value));
        record(b"GMST", id, &d)
    };
    let mut settings = setting(SETTINGS, "fAVDSkillSmallGunsBase", &2.0f32.to_le_bytes());
    settings.extend(setting(
        SETTINGS + 1,
        "fAVDTagSkillBonus",
        &15.0f32.to_le_bytes(),
    ));
    settings.extend(setting(
        SETTINGS + 2,
        "iTraitMenuMaxNumTraits",
        &2i32.to_le_bytes(),
    ));
    // Barter: the game's prices and the skill's base.
    for (i, (name, value)) in [
        ("fBarterBuyBase", 1.55f32),
        ("fBarterBuyMult", -0.45),
        ("fBarterSellBase", 0.45),
        ("fBarterSellMult", 0.45),
        ("fAVDSkillBarterBase", 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        settings.extend(setting(
            BARTER_SETTINGS + i as u32,
            name,
            &value.to_le_bytes(),
        ));
    }
    // An image space modifier: 2 seconds, animatable; bright scale (track
    // 6) multiplied 2 → 1; a white fade clearing by half way.
    let mut flash = edid("TestFlash");
    let mut dnam = 1u32.to_le_bytes().to_vec();
    dnam.extend(2.0f32.to_le_bytes());
    flash.extend(sub(b"DNAM", &dnam));
    flash.extend(sub(
        b"NAM3",
        &f32s(&[0.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 0.0]),
    ));
    flash.extend(sub(&[6, b'I', b'A', b'D'], &f32s(&[0.0, 2.0, 1.0, 1.0])));
    flash.extend(sub(&[0x46, b'I', b'A', b'D'], &f32s(&[0.0, 0.0, 1.0, 0.5])));
    // Combat: the damage settings, a pistol (damage 16, Guns), a gecko
    // (health 30, bites for 8; its death counted in TestGlobal), a bottle
    // that counts the pistol's hits (also in TestGlobal).
    for (i, (name, value)) in [
        ("fDamageSkillBase", 0.5f32),
        ("fDamageSkillMult", 0.5),
        ("fDamageGunWeapCondBase", 0.66),
        ("fDamageGunWeapCondMult", 0.34),
        ("fMinDamMultiplier", 0.2),
    ]
    .into_iter()
    .enumerate()
    {
        settings.extend(setting(
            COMBAT_SETTINGS + i as u32,
            name,
            &value.to_le_bytes(),
        ));
    }
    for (i, (name, value)) in [
        ("iXPRewardPickLockEasy", 30i32),
        ("iXPRewardHackComputerEasy", 30),
        ("iXPRewardDiscoverMapMarker", 10),
    ]
    .into_iter()
    .enumerate()
    {
        settings.extend(setting(XP_SETTINGS + i as u32, name, &value.to_le_bytes()));
    }
    let mut pistol = edid("TestPistol");
    pistol.extend(sub(b"FULL", &zstr("Pistol")));
    let mut pistol_data = 100i32.to_le_bytes().to_vec();
    pistol_data.extend(150i32.to_le_bytes());
    pistol_data.extend(1.5f32.to_le_bytes());
    pistol_data.extend(16i16.to_le_bytes());
    pistol_data.push(13);
    pistol.extend(sub(b"DATA", &pistol_data));
    let mut weapon_dnam = vec![0u8; 204];
    // No resist type (-1 at 120), as the game's guns.
    weapon_dnam[120..124].copy_from_slice(&(-1i32).to_le_bytes());
    weapon_dnam[0] = 3; // a one-handed pistol
    weapon_dnam[4..8].copy_from_slice(&1.0f32.to_le_bytes()); // speed
    weapon_dnam[60..64].copy_from_slice(&1.0f32.to_le_bytes()); // attack multiplier
    weapon_dnam[88..92].copy_from_slice(&3.125f32.to_le_bytes());
    weapon_dnam[104..108].copy_from_slice(&41u32.to_le_bytes()); // Guns
    weapon_dnam[116..120].copy_from_slice(&1.0f32.to_le_bytes()); // limb damage
    pistol.extend(sub(b"DNAM", &weapon_dnam.clone()));
    pistol.extend(sub(b"REPL", &REPAIR_LIST.to_le_bytes()));
    let mut revolver = edid("TestRevolver");
    revolver.extend(sub(b"FULL", &zstr("Revolver")));
    let mut revolver_data = 50i32.to_le_bytes().to_vec();
    revolver_data.extend(100i32.to_le_bytes());
    revolver_data.extend(1.0f32.to_le_bytes());
    revolver_data.extend(12i16.to_le_bytes());
    revolver_data.push(6);
    revolver.extend(sub(b"DATA", &revolver_data));
    revolver.extend(sub(b"DNAM", &weapon_dnam.clone()));
    // A two-handed rifle: damage 20, limb damage × 1.
    let mut rifle = edid("TestRifle");
    rifle.extend(sub(b"FULL", &zstr("Rifle")));
    let mut rifle_data = 200i32.to_le_bytes().to_vec();
    rifle_data.extend(250i32.to_le_bytes());
    rifle_data.extend(6.0f32.to_le_bytes());
    rifle_data.extend(20i16.to_le_bytes());
    rifle_data.push(5);
    rifle.extend(sub(b"DATA", &rifle_data));
    weapon_dnam[0] = 5;
    rifle.extend(sub(b"DNAM", &weapon_dnam));
    let mut gecko = edid("TestGecko");
    gecko.extend(sub(b"FULL", &zstr("Gecko")));
    gecko.extend(sub(b"PNAM", &GECKO_BODY_PARTS.to_le_bytes()));
    gecko.extend(sub(b"SCRI", &GECKO_SCRIPT.to_le_bytes()));
    gecko.extend(sub(b"ACBS", &[0; 24]));
    // Type, three skills, health 30, 2 unused, damage 8, SPECIAL.
    let mut gecko_data = vec![0u8, 0, 0, 0];
    gecko_data.extend(30i16.to_le_bytes());
    gecko_data.extend([0, 0]);
    gecko_data.extend(8i16.to_le_bytes());
    gecko_data.extend([5; 7]);
    gecko.extend(sub(b"DATA", &gecko_data));
    gecko.extend(sub(b"NAM5", &0u32.to_le_bytes()));
    let mut bottle = edid("TestBottle");
    bottle.extend(sub(b"SCRI", &BOTTLE_SCRIPT.to_le_bytes()));
    // A healing item: its effect restores health (archetype 0, actor
    // value 16), 30 at once; and clothes on overlapping slots.
    let mut heal_effect = edid("TestRestoreHealth");
    heal_effect.extend(sub(b"FULL", &zstr("Restore Health")));
    let mut mgef = vec![0u8; 72];
    mgef[68..72].copy_from_slice(&16u32.to_le_bytes());
    heal_effect.extend(sub(b"DATA", &mgef));
    let mut medicine = edid("TestMedicine");
    medicine.extend(sub(b"FULL", &zstr("Medicine")));
    medicine.extend(sub(b"DATA", &0.0f32.to_le_bytes()));
    medicine.extend(sub(b"ENIT", &[0; 20]));
    medicine.extend(sub(b"EFID", &HEAL_EFFECT.to_le_bytes()));
    let mut efit = 30u32.to_le_bytes().to_vec();
    efit.extend([0; 12]);
    efit.extend(16u32.to_le_bytes());
    medicine.extend(sub(b"EFIT", &efit));
    // A tonic with four kinds of effect (see `TONIC`), and two spells.
    let magic_effect =
        |id: u32, name: &str, flags: u32, script: u32, resist: i32, archetype: u32, av: i32| {
            let mut d = edid(name);
            d.extend(sub(b"FULL", &zstr(name)));
            let mut data = vec![0u8; 72];
            data[0..4].copy_from_slice(&flags.to_le_bytes());
            data[8..12].copy_from_slice(&script.to_le_bytes());
            data[16..20].copy_from_slice(&resist.to_le_bytes());
            data[64..68].copy_from_slice(&archetype.to_le_bytes());
            data[68..72].copy_from_slice(&av.to_le_bytes());
            d.extend(sub(b"DATA", &data));
            record(b"MGEF", id, &d)
        };
    let mut effects = magic_effect(STRENGTH_EFFECT, "TestFortifyStrength", 0x02, 0, -1, 0, 5);
    effects.extend(magic_effect(
        RADS_EFFECT,
        "TestDamageRads",
        0x04,
        0,
        20,
        0,
        54,
    ));
    effects.extend(magic_effect(
        CAP_EFFECT,
        "TestCapEffect",
        0,
        CAP_SCRIPT,
        -1,
        1,
        -1,
    ));
    // Health and the body's parts (archetype 34), 30 at once.
    effects.extend(magic_effect(
        VAC_EFFECT,
        "TestRestoreHealthAndConditions",
        0,
        0,
        -1,
        34,
        16,
    ));
    let mut stimpak = edid("TestStimpak");
    stimpak.extend(sub(b"FULL", &zstr("Stimpak")));
    stimpak.extend(sub(b"DATA", &0.0f32.to_le_bytes()));
    stimpak.extend(sub(b"ENIT", &[0; 20]));
    stimpak.extend(sub(b"EFID", &VAC_EFFECT.to_le_bytes()));
    let mut stim_efit = 30u32.to_le_bytes().to_vec();
    stim_efit.extend([0; 12]);
    stim_efit.extend(16u32.to_le_bytes());
    stimpak.extend(sub(b"EFIT", &stim_efit));
    let effect = |id: u32, magnitude: u32, duration: u32| {
        let mut d = sub(b"EFID", &id.to_le_bytes());
        let mut efit = magnitude.to_le_bytes().to_vec();
        efit.extend([0; 4]);
        efit.extend(duration.to_le_bytes());
        efit.extend([0; 4]);
        efit.extend((-1i32).to_le_bytes());
        d.extend(sub(b"EFIT", &efit));
        d
    };
    let mut tonic = edid("TestTonic");
    tonic.extend(sub(b"FULL", &zstr("Tonic")));
    tonic.extend(sub(b"DATA", &0.0f32.to_le_bytes()));
    tonic.extend(sub(b"ENIT", &[0; 20]));
    tonic.extend(effect(STRENGTH_EFFECT, 2, 10));
    tonic.extend(effect(HEAL_EFFECT, 3, 4));
    tonic.extend(effect(RADS_EFFECT, 10, 0));
    tonic.extend(effect(CAP_EFFECT, 0, 0));
    // A food and a medicine (see `quest_ids::FOOD`): `ENIT` value, flags
    // (0x02 food, 0x04 medicine), then the hostile effect that lowers
    // Strength (`MGEF` flags 0x01 hostile, 0x04 detrimental).
    effects.extend(magic_effect(
        HOSTILE_EFFECT,
        "TestHostileEffect",
        0x05,
        0,
        -1,
        0,
        5,
    ));
    let ingestible = |name: &str, flags: u8| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        d.extend(sub(b"DATA", &0.5f32.to_le_bytes()));
        let mut enit = 10i32.to_le_bytes().to_vec();
        enit.extend([flags, 0, 0, 0]);
        enit.extend([0; 12]);
        d.extend(sub(b"ENIT", &enit));
        d
    };
    let mut food = ingestible("TestFood", 0x02);
    food.extend(effect(HEAL_EFFECT, 3, 4));
    food.extend(effect(RADS_EFFECT, 10, 0));
    food.extend(effect(HOSTILE_EFFECT, 1, 10));
    let mut chem = ingestible("TestChem", 0x04);
    chem.extend(effect(HEAL_EFFECT, 30, 0));
    chem.extend(effect(STRENGTH_EFFECT, 2, 10));
    let spell = |id: u32, name: &str, kind: u32, magnitude: u32, duration: u32| {
        let mut d = edid(name);
        let mut spit = kind.to_le_bytes().to_vec();
        spit.extend([0; 12]);
        d.extend(sub(b"SPIT", &spit));
        d.extend(effect(STRENGTH_EFFECT, magnitude, duration));
        record(b"SPEL", id, &d)
    };
    let mut spells = spell(SPELL, "TestSpell", 0, 1, 100);
    spells.extend(spell(ABILITY, "TestAbility", 4, 3, 0));
    let clothes = |id: u32, name: &str, slots: u32| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        let mut bmdt = slots.to_le_bytes().to_vec();
        bmdt.extend([0; 4]);
        d.extend(sub(b"BMDT", &bmdt));
        record(b"ARMO", id, &d)
    };
    let mut apparel = clothes(SHIRT, "TestShirt", 0x04);
    apparel.extend(clothes(COAT, "TestCoat", 0x04 | 0x08));
    apparel.extend(clothes(HAT, "TestHat", 0x400));
    let mut hat = edid("TestScriptedHat");
    hat.extend(sub(b"FULL", &zstr("Scripted Hat")));
    hat.extend(sub(b"SCRI", &SCRIPTED_HAT_SCRIPT.to_le_bytes()));
    hat.extend(sub(b"BMDT", &[0, 4, 0, 0, 0, 0, 0, 0]));
    apparel.extend(record(b"ARMO", SCRIPTED_HAT, &hat));
    let mut vanishing = edid("TestVanishingHat");
    vanishing.extend(sub(b"FULL", &zstr("Vanishing Hat")));
    vanishing.extend(sub(b"SCRI", &VANISHING_HAT_SCRIPT.to_le_bytes()));
    vanishing.extend(sub(b"BMDT", &[0, 4, 0, 0, 0, 0, 0, 0]));
    apparel.extend(record(b"ARMO", VANISHING_HAT, &vanishing));
    let armour = |id: u32, name: &str, slots: u32, value: i32, health: i32, dt: f32| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        let mut bmdt = slots.to_le_bytes().to_vec();
        bmdt.extend([0; 4]);
        d.extend(sub(b"BMDT", &bmdt));
        let mut data = value.to_le_bytes().to_vec();
        data.extend(health.to_le_bytes());
        data.extend(5.0f32.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        let mut dnam = 0i16.to_le_bytes().to_vec();
        dnam.extend([0; 2]);
        dnam.extend(dt.to_le_bytes());
        dnam.extend([0; 4]);
        d.extend(sub(b"DNAM", &dnam));
        record(b"ARMO", id, &d)
    };
    apparel.extend(armour(ARMOR, "TestArmor", 0x04, 100, 100, 10.0));
    apparel.extend(armour(HELMET, "TestHelmet", 0x01, 40, 50, 4.0));
    // A trait (DATA: trait, level, ranks, playable, hidden) and a perk.
    let mut perks = Vec::new();
    for (id, name, is_trait) in [(TRAIT, "Test Trait", 1u8), (PERK, "Test Perk", 0)] {
        let mut d = edid(&name.replace(' ', ""));
        d.extend(sub(b"FULL", &zstr(name)));
        d.extend(sub(b"DESC", &zstr("Does something.")));
        d.extend(sub(b"DATA", &[is_trait, 1, 1, 1, 0]));
        perks.extend(record(b"PERK", id, &d));
    }
    // Perks with entry points (`PRKE` kind 2, rank 0; `DATA` entry,
    // function, tabs; `EPFT` 1; `EPFD` the value), laid out as the game's
    // Swift Learner (entry 9 × 1.1, level 2) and Educated (entry 10 + 2,
    // level 4); and one for women only (`GetIsSex` 1 == 1).
    for (id, name, level, entry, function, value) in [
        (QUICK_STUDY, "Quick Study", 2u8, 9u8, 3u8, 1.1f32),
        (SCHOOLED, "Schooled", 4, 10, 2, 2.0),
        (LADIES_ONLY, "Ladies Only", 2, 9, 3, 1.0),
        // Not offered at level-ups (level 0), as Comprehension's entry.
        (BOOKWORM, "Bookworm", 0, 11, 2, 1.0),
    ] {
        // Quick Study has two ranks, each with its own entry (× 1.1, then
        // × 1.2), as Swift Learner has three.
        let ranks: &[(u8, f32)] = if id == QUICK_STUDY {
            &[(0, value), (1, 1.2)]
        } else {
            &[(0, value)]
        };
        let mut d = edid(&name.replace(' ', ""));
        d.extend(sub(b"FULL", &zstr(name)));
        d.extend(sub(b"DESC", &zstr("Does something.")));
        d.extend(sub(b"DATA", &[0, level, ranks.len() as u8, 1, 0]));
        if id == LADIES_ONLY {
            d.extend(condition(70, [1, 0], 1.0));
        }
        for &(rank, value) in ranks {
            d.extend(sub(b"PRKE", &[2, rank, 0]));
            d.extend(sub(b"DATA", &[entry, function, 1]));
            d.extend(sub(b"EPFT", &[1]));
            d.extend(sub(b"EPFD", &value.to_le_bytes()));
            d.extend(sub(b"PRKF", &[]));
        }
        perks.extend(record(b"PERK", id, &d));
    }
    // A perk with an ability (`PRKE` kind 1: `TestAbility`, +3 Strength)
    // and an entry point with a condition on its holder (tab 0: damage
    // threshold + 3 for women only; three tabs, as the game's Toughness:
    // the holder, the attacker, the attacker's weapon).
    let mut tough = edid("ToughSkin");
    tough.extend(sub(b"FULL", &zstr("Tough Skin")));
    tough.extend(sub(b"DATA", &[0, 0, 1, 1, 0]));
    tough.extend(sub(b"PRKE", &[1, 0, 0]));
    tough.extend(sub(b"DATA", &ABILITY.to_le_bytes()));
    tough.extend(sub(b"PRKF", &[]));
    tough.extend(sub(b"PRKE", &[2, 0, 0]));
    tough.extend(sub(b"DATA", &[56, 2, 3]));
    tough.extend(sub(b"PRKC", &[0]));
    tough.extend(condition(70, [1, 0], 1.0));
    tough.extend(sub(b"EPFT", &[1]));
    tough.extend(sub(b"EPFD", &3.0f32.to_le_bytes()));
    tough.extend(sub(b"PRKF", &[]));
    perks.extend(record(b"PERK", TOUGH_SKIN, &tough));
    // Perks shaped as the game's (see `quest_ids`): one entry point each,
    // DATA (entry, function, the entry point's tab count), conditions by
    // tab. Condition functions: IsWeaponSkillType 109, IsInList 372,
    // GetIsCreature 64, GetEquipped 182 (the function table's numbers).
    let entry_perk = |id: u32,
                      name: &str,
                      is_trait: u8,
                      entry: u8,
                      function: u8,
                      tabs: u8,
                      value: f32,
                      conditions: &[(u8, Vec<u8>)]| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        // Not playable: the level-up and trait menus leave them out.
        d.extend(sub(b"DATA", &[is_trait, 2, 1, 0, 0]));
        d.extend(sub(b"PRKE", &[2, 0, 0]));
        d.extend(sub(b"DATA", &[entry, function, tabs]));
        for (tab, c) in conditions {
            d.extend(sub(b"PRKC", &[*tab]));
            d.extend(c);
        }
        d.extend(sub(b"EPFT", &[1]));
        d.extend(sub(b"EPFD", &value.to_le_bytes()));
        d.extend(sub(b"PRKF", &[]));
        record(b"PERK", id, &d)
    };
    let in_list = |tab: u8| (tab, condition(372, [COWHAND_LIST, 0], 1.0));
    let melee_or_unarmed = |tab: u8| {
        let mut c = condition_with(comparison::EQUAL, true, 109, [38, 0], 1.0);
        c.extend(condition(109, [45, 0], 1.0));
        (tab, c)
    };
    let guns = |tab: u8| (tab, condition(109, [41, 0], 1.0));
    let guns_or_energy = |tab: u8| {
        let mut c = condition_with(comparison::EQUAL, true, 109, [41, 0], 1.0);
        c.extend(condition(109, [34, 0], 1.0));
        (tab, c)
    };
    for (id, name, is_trait, entry, function, tabs, value, conditions) in [
        (
            COWHAND,
            "Cowhand",
            0u8,
            0u8,
            3u8,
            3u8,
            1.25f32,
            vec![in_list(1)],
        ),
        (
            BEAST_SLAYER,
            "BeastSlayer",
            0,
            0,
            3,
            3,
            1.5,
            vec![(2, condition(64, [0, 0], 1.0))],
        ),
        (
            PIERCER,
            "Piercer",
            0,
            58,
            1,
            3,
            15.0,
            vec![melee_or_unarmed(1)],
        ),
        (
            BULWARK,
            "Bulwark",
            0,
            56,
            2,
            3,
            5.0,
            vec![melee_or_unarmed(2)],
        ),
        (MARKSMAN, "Marksman", 0, 1, 2, 3, 10.0, vec![in_list(1)]),
        (BETTER_CRITS, "BetterCrits", 0, 2, 3, 3, 1.5, vec![]),
        (DREAM_CRUSHER, "DreamCrusher", 0, 36, 3, 3, 0.5, vec![]),
        (CHEMIST, "Chemist", 0, 25, 3, 1, 2.0, vec![]),
        (LEAD_BELLY, "LeadBelly", 0, 44, 3, 1, 0.5, vec![]),
        (BETTER_HEALING, "BetterHealing", 0, 12, 3, 1, 1.2, vec![]),
        (SILENT_RUNNING, "SilentRunning", 0, 31, 1, 1, 1.0, vec![]),
        (
            TRAVEL_LIGHT,
            "TravelLight",
            0,
            42,
            3,
            1,
            1.1,
            vec![(
                0,
                condition_with(comparison::NOT_EQUAL, false, 182, [COAT, 0], 1.0),
            )],
        ),
        (PACK_RAT, "PackRat", 0, 55, 1, 1, 1.0, vec![]),
        (HAND_LOADER, "HandLoader", 0, 57, 3, 2, 2.0, vec![guns(1)]),
        (RAPID_RELOAD, "RapidReload", 0, 37, 3, 2, 1.25, vec![]),
        (LONG_HAUL, "LongHaul", 0, 51, 1, 1, 1.0, vec![]),
        (
            FAST_SHOT,
            "FastShot",
            0,
            43,
            3,
            2,
            1.2,
            vec![guns_or_energy(1)],
        ),
        (
            BUILT_TO_DESTROY,
            "BuiltToDestroy",
            1,
            68,
            3,
            1,
            1.15,
            vec![],
        ),
        (ADAMANTIUM, "Adamantium", 0, 6, 3, 3, 0.5, vec![guns(2)]),
        (JURY_RIGGING, "JuryRigging", 0, 48, 1, 1, 1.0, vec![]),
    ] {
        perks.extend(entry_perk(
            id,
            name,
            is_trait,
            entry,
            function,
            tabs,
            value,
            &conditions,
        ));
    }
    for (i, (name, value)) in [("fRepairSkillMax", 10.0f32), ("fItemRepairCostMult", 2.0)]
        .into_iter()
        .enumerate()
    {
        settings.extend(setting(
            REPAIR_SETTINGS + i as u32,
            name,
            &value.to_le_bytes(),
        ));
    }
    for (i, (name, value)) in [
        ("fPackRatThreshold", 2.0f32),
        ("fPackRatModifier", 0.5),
        ("fAgilityReloadBase", 5.0),
        ("fAgilityReloadModifier", 0.1),
        ("fDamageToWeaponValue", 0.2),
        ("fMagicMedicineSkillMult", 2.0),
        ("fMagicSurvivalSkillMult", 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        settings.extend(setting(
            PERK_SETTINGS + i as u32,
            name,
            &value.to_le_bytes(),
        ));
    }
    for (i, (name, value)) in [
        ("iXPBumpBase", 150i32),
        ("iXPDeathRewardHealthThreshold", 40),
        ("iMaxCharacterLevel", 30),
    ]
    .into_iter()
    .enumerate()
    {
        settings.extend(setting(
            LEVEL_SETTINGS + i as u32,
            name,
            &value.to_le_bytes(),
        ));
    }
    settings.extend(setting(
        LEVEL_SETTINGS + 3,
        "fAVDHealthLevelMult",
        &5.0f32.to_le_bytes(),
    ));
    settings.extend(setting(
        HEAL_RATE_SETTING,
        "fAVDHealRateEndurance9Bonus",
        &10.0f32.to_le_bytes(),
    ));
    settings.extend(setting(
        BOOK_SETTING,
        "fBookPerkBonus",
        &3.0f32.to_le_bytes(),
    ));
    // A skill book (`DATA`: flags, skill 8 = Science, value, weight).
    let mut science_book = edid("TestScienceBook");
    science_book.extend(sub(b"FULL", &zstr("Science Book")));
    let mut book_data = vec![0u8, 8];
    book_data.extend(50i32.to_le_bytes());
    book_data.extend(2.0f32.to_le_bytes());
    science_book.extend(sub(b"DATA", &book_data));

    // A greeting for the doctor that starts the quest, and one for nobody.
    let greeting = {
        let mut d = edid("GREETING");
        d.extend(sub(b"FULL", &zstr("Hello")));
        record(b"DIAL", GREETING, &d)
    };
    // Said once only (flag 0x04).
    let mut line = sub(b"DATA", &[0, 0, 0x04, 0]);
    line.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
    line.extend(sub(b"TRDT", &[0; 24]));
    line.extend(sub(b"NAM1", &zstr("Welcome back.")));
    line.extend(condition(72, [DOC, 0], 1.0));
    line.extend(sub(b"SCHR", &[0; 20]));
    line.extend(sub(b"SCTX", b"SetStage TestQuest 10"));
    line.extend(sub(b"NEXT", &[]));
    line.extend(sub(b"SCHR", &[0; 20]));
    line.extend(sub(b"SCTX", b"set TestGlobal to TestGlobal + 1"));
    let mut lines = record(b"INFO", GREETING_LINE, &line);
    let mut other = sub(b"DATA", &[0, 0, 0, 0]);
    other.extend(sub(b"TRDT", &[0; 24]));
    other.extend(sub(b"NAM1", &zstr("Who are you?")));
    other.extend(condition(72, [0xFFF, 0], 1.0));
    lines.extend(record(b"INFO", OTHER_LINE, &other));
    let mut dialogue = greeting;
    dialogue.extend(group(GREETING.to_le_bytes(), 7, &lines));
    // The companion wheel's topics (the game's `FollowersWait`,
    // `Regenerating`), with Doc's lines.
    for (topic, info, name, said, scripts) in [
        (
            FOLLOWERS_WAIT,
            WAIT_LINE,
            "FollowersWait",
            "I'll sit tight.",
            Some(("set DocRef.Waiting to 1", "set TestGlobal to 7")),
        ),
        (
            REGENERATING,
            REGENERATING_LINE,
            "Regenerating",
            "Much better.",
            None,
        ),
    ] {
        let mut d = edid(name);
        d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        d.extend(sub(b"DATA", &[0, 0]));
        dialogue.extend(record(b"DIAL", topic, &d));
        let mut line = sub(b"DATA", &[0, 0, 0, 0]);
        line.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        line.extend(sub(b"TRDT", &[0; 24]));
        line.extend(sub(b"NAM1", &zstr(said)));
        line.extend(condition(72, [DOC, 0], 1.0));
        if let Some((begin, end)) = scripts {
            line.extend(sub(b"SCHR", &[0; 20]));
            line.extend(sub(b"SCTX", begin.as_bytes()));
            line.extend(sub(b"NEXT", &[]));
            line.extend(sub(b"SCHR", &[0; 20]));
            line.extend(sub(b"SCTX", end.as_bytes()));
        }
        dialogue.extend(group(topic.to_le_bytes(), 7, &record(b"INFO", info, &line)));
    }
    // Topics to ask about: (topic, its line, name, top-level, priority,
    // who answers, the line's prompt, a topic the line teaches).
    let topics = [
        (
            TOPIC_ABOUT,
            ABOUT_LINE,
            "Tell me about yourself.",
            true,
            60.0,
            DOC,
            None,
            Some(TOPIC_SECRET),
        ),
        (
            TOPIC_TOWN,
            TOWN_LINE,
            "What's this town?",
            true,
            90.0,
            DOC,
            Some("Where am I?"),
            None,
        ),
        (
            TOPIC_SECRET,
            SECRET_LINE,
            "The secret.",
            false,
            50.0,
            DOC,
            None,
            None,
        ),
        (
            TOPIC_NOBODY,
            NOBODY_LINE,
            "Nobody answers this.",
            true,
            99.0,
            0xFFF,
            None,
            None,
        ),
    ];
    for (topic, info, name, top, priority, who, prompt, teaches) in topics {
        let mut d = edid(&format!("Topic{topic:X}"));
        d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        d.extend(sub(b"FULL", &zstr(name)));
        d.extend(sub(b"PNAM", &f32::to_le_bytes(priority)));
        d.extend(sub(b"DATA", &[0, if top { 0x02 } else { 0 }]));
        dialogue.extend(record(b"DIAL", topic, &d));
        let mut line = sub(b"DATA", &[0, 0, 0, 0]);
        line.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        line.extend(sub(b"TRDT", &[0; 24]));
        line.extend(sub(b"NAM1", &zstr(&format!("About {name}"))));
        line.extend(condition(72, [who, 0], 1.0));
        if let Some(t) = teaches {
            line.extend(sub(b"NAME", &t.to_le_bytes()));
        }
        // Asking about him offers two follow-ups, one nobody answers.
        if topic == TOPIC_ABOUT {
            line.extend(sub(b"TCLT", &TOPIC_NOBODY.to_le_bytes()));
            line.extend(sub(b"TCLT", &TOPIC_TOWN.to_le_bytes()));
        }
        if let Some(p) = prompt {
            line.extend(sub(b"RNAM", &zstr(p)));
        }
        dialogue.extend(group(topic.to_le_bytes(), 7, &record(b"INFO", info, &line)));
    }
    // A Speech check as New Vegas writes them (Trudy's "< Speech 25 >"):
    // not top-level, two lines with their own prompts, one passing on
    // `GetActorValue Speech >= 25` run on the player, one on `< 25`.
    let mut d = edid("TestSpeechCheck");
    d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
    d.extend(sub(b"FULL", &zstr("< Speech 25 >")));
    d.extend(sub(b"DATA", &[0, 0]));
    dialogue.extend(record(b"DIAL", TOPIC_CHECK, &d));
    let mut check_lines = Vec::new();
    for (id, comparison, prompt, said) in [
        (CHECK_PASSED, 0x60u8, "Trust me.", "[SUCCEEDED] I do."),
        (CHECK_FAILED, 0x80, "Please?", "[FAILED] No."),
    ] {
        let mut line = sub(b"DATA", &[0, 0, 0, 0]);
        line.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        line.extend(sub(b"TRDT", &[0; 24]));
        line.extend(sub(b"NAM1", &zstr(said)));
        let mut ctda = condition(14, [43, 0], 25.0);
        ctda[6] = comparison;
        ctda[6 + 20] = 1;
        line.extend(ctda);
        line.extend(condition(72, [DOC, 0], 1.0));
        line.extend(sub(b"RNAM", &zstr(prompt)));
        // What it's a check of, for the choice's tag.
        line.extend(sub(b"KNAM", &SPEECH_SKILL.to_le_bytes()));
        check_lines.extend(record(b"INFO", id, &line));
    }
    dialogue.extend(group(TOPIC_CHECK.to_le_bytes(), 7, &check_lines));
    // A topic with no lines of its own, connected (`INFC`, then `INFX`, its
    // place) to the secret's line.
    let mut d = edid("TestLinkedTopic");
    d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
    d.extend(sub(b"INFC", &SECRET_LINE.to_le_bytes()));
    d.extend(sub(b"INFX", &0u32.to_le_bytes()));
    d.extend(sub(b"FULL", &zstr("Linked.")));
    d.extend(sub(b"DATA", &[0, 0]));
    dialogue.extend(record(b"DIAL", TOPIC_LINKED, &d));

    // Editor IDs come first in a record.
    let named_with = |id: u32, base: u32, pos: [f32; 3], name: &str, extra: &[u8]| {
        let r = placed(id, base, pos, [0.0; 3], extra);
        let mut d = edid(name);
        d.extend(&r[24..]);
        record(b"REFR", id, &d)
    };
    let named =
        |id: u32, base: u32, pos: [f32; 3], name: &str| named_with(id, base, pos, name, &[]);
    // A load door (the test door's base) to `to`, arriving at `arrive`
    // facing east.
    let link_door = |id: u32, pos: [f32; 3], to: u32, arrive: [f32; 3]| {
        let mut xtel = to.to_le_bytes().to_vec();
        xtel.extend(f32s(&[arrive[0], arrive[1], arrive[2], 0.0, 0.0, 1.5]));
        xtel.extend(0u32.to_le_bytes());
        placed(id, DOOR, pos, [0.0; 3], &sub(b"XTEL", &xtel))
    };
    // The doctor sells what's in the chest (his merchant container).
    let mut refs = named_with(
        DOC_REF,
        DOC,
        [0.0, 0.0, 0.0],
        "DocRef",
        &sub(b"XMRC", &CHEST_REF.to_le_bytes()),
    );
    refs[..4].copy_from_slice(b"ACHR");
    let mut door_ref = named(DOOR_REF, DOOR, [100.0, 0.0, 0.0], "DoorRef");
    // Flags: initially disabled.
    door_ref[8..12].copy_from_slice(&0x800u32.to_le_bytes());
    refs.extend(door_ref);
    refs.extend(named(CHEST_REF, CHEST, [0.0, 100.0, 0.0], "ChestRef"));
    let mut gecko_ref = named(GECKO_REF, GECKO, [0.0, 300.0, 0.0], "GeckoRef");
    gecko_ref[..4].copy_from_slice(b"ACRE");
    refs.extend(gecko_ref);
    refs.extend(named(BOTTLE_REF, BOTTLE, [300.0, 300.0, 0.0], "BottleRef"));
    refs.extend(named(CHAIR_REF, CHAIR, [-100.0, 0.0, 0.0], "ChairRef"));
    refs.extend(link_door(
        LINK_DOOR,
        [150.0, 150.0, 0.0],
        LINK_DOOR2,
        [400.0, 0.0, 0.0],
    ));
    let mut xloc = vec![50u8, 0, 0, 0];
    xloc.extend(KEY.to_le_bytes());
    xloc.extend([0; 12]);
    refs.extend(named_with(
        STRONGBOX_REF,
        CHEST,
        [0.0, -100.0, 0.0],
        "StrongboxRef",
        &sub(b"XLOC", &xloc),
    ));
    refs.extend(named_with(
        TERMINAL_REF,
        TERMINAL,
        [0.0, -150.0, 0.0],
        "TerminalRef",
        &sub(b"XLKR", &STRONGBOX_REF.to_le_bytes()),
    ));
    // Five caps on the floor (XCNT).
    refs.extend(placed(
        CAPS_REF,
        CAPS,
        [50.0, 50.0, 0.0],
        [0.0; 3],
        &sub(b"XCNT", &5u32.to_le_bytes()),
    ));
    // A navmesh: the square from (-200,-200) to (200,200) as two
    // triangles, counterclockwise, sharing the diagonal.
    let mut navmesh = sub(b"NVER", &11u32.to_le_bytes());
    navmesh.extend(sub(
        b"NVVX",
        &f32s(&[
            -200.0, -200.0, 0.0, 200.0, -200.0, 0.0, 200.0, 200.0, 0.0, -200.0, 200.0, 0.0,
        ]),
    ));
    // Triangle flags 0x1 / 0x2 / 0x4: that edge leads to another navmesh,
    // its link counting into the external connections (NVEX).
    let tri = |v: [u16; 3], n: [u16; 3], flags: u16| {
        let mut d = Vec::new();
        for x in v.into_iter().chain(n) {
            d.extend(x.to_le_bytes());
        }
        d.extend(flags.to_le_bytes());
        d.extend([0; 2]);
        d
    };
    let external = |mesh: u32, triangle: u16| {
        let mut d = vec![0; 4];
        d.extend(mesh.to_le_bytes());
        d.extend(triangle.to_le_bytes());
        sub(b"NVEX", &d)
    };
    // Triangle 0's edge 1 (the east side, x = 200) leads to the second
    // navmesh's triangle 1.
    let mut triangles = tri([0, 1, 2], [0xFFFF, 0, 1], 0x2);
    triangles.extend(tri([0, 2, 3], [0, 0xFFFF, 0xFFFF], 0));
    navmesh.extend(sub(b"NVTR", &triangles));
    navmesh.extend(external(NAVMESH2, 1));
    refs.extend(record(b"NAVM", NAVMESH, &navmesh));
    // The second: (200,-200) to (600,200); its triangle 1's edge 2 (the
    // west side) leads back.
    let mut navmesh2 = sub(b"NVER", &11u32.to_le_bytes());
    navmesh2.extend(sub(
        b"NVVX",
        &f32s(&[
            200.0, -200.0, 0.0, 600.0, -200.0, 0.0, 600.0, 200.0, 0.0, 200.0, 200.0, 0.0,
        ]),
    ));
    let mut triangles2 = tri([0, 1, 2], [0xFFFF, 0xFFFF, 1], 0);
    triangles2.extend(tri([0, 2, 3], [0, 0xFFFF, 0], 0x4));
    navmesh2.extend(sub(b"NVTR", &triangles2));
    navmesh2.extend(external(NAVMESH, 0));
    // A map marker, shown from the start (FNAM 0x01), and one that isn't.
    for (id, name, y, flags) in [
        (MARKER, "MarkerRef", -100.0, 0x01),
        (HIDDEN_MARKER, "HiddenMarkerRef", -150.0, 0),
    ] {
        let mut extra = sub(b"XMRK", &[]);
        extra.extend(sub(b"FNAM", &[flags]));
        let r = placed(id, 0x10, [0.0, y, 0.0], [0.0; 3], &extra);
        let mut d = edid(name);
        d.extend(&r[24..]);
        refs.extend(record(b"REFR", id, &d));
    }
    let mut cell = edid("TestCell");
    cell.extend(sub(b"DATA", &[1]));
    // The cell is in the test region.
    cell.extend(sub(b"XCLR", &REGION.to_le_bytes()));
    let mut contents = record(b"CELL", CELL, &cell);
    contents.extend(group(
        CELL.to_le_bytes(),
        6,
        &group(CELL.to_le_bytes(), 9, &refs),
    ));
    let mut cell2 = edid("TestCell2");
    cell2.extend(sub(b"DATA", &[1]));
    contents.extend(record(b"CELL", CELL2, &cell2));
    // The second cell: its door back, and a marker.
    let mut refs2 = record(b"NAVM", NAVMESH2, &navmesh2);
    refs2.extend(link_door(
        LINK_DOOR2,
        [450.0, 0.0, 0.0],
        LINK_DOOR,
        [150.0, 120.0, 0.0],
    ));
    refs2.extend(named(
        FAR_MARKER,
        CHEST,
        [500.0, 100.0, 0.0],
        "FarMarkerRef",
    ));
    contents.extend(group(
        CELL2.to_le_bytes(),
        6,
        &group(CELL2.to_le_bytes(), 9, &refs2),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"GMST", 0, &settings));
    // The game's clock: 10:00, at 30 game seconds a second.
    let mut globals = record(b"GLOB", GLOBAL, &glob);
    for (id, name, value) in [
        (GAME_HOUR, "GameHour", 10.0f32),
        (GAME_HOUR + 1, "TimeScale", 30.0),
    ] {
        let mut d = edid(name);
        d.extend(sub(b"FNAM", b"f"));
        d.extend(sub(b"FLTV", &value.to_le_bytes()));
        globals.extend(record(b"GLOB", id, &d));
    }
    plugin.extend(group(*b"GLOB", 0, &globals));
    let mut messages = record(b"MESG", MESSAGE, &mesg);
    messages.extend(record(b"MESG", CHOICE, &choice));
    plugin.extend(group(*b"MESG", 0, &messages));
    plugin.extend(group(*b"PERK", 0, &perks));
    let mut speech = edid("AVSpeech");
    speech.extend(sub(b"FULL", &zstr("Speech")));
    plugin.extend(group(*b"AVIF", 0, &record(b"AVIF", SPEECH_SKILL, &speech)));
    let effect = |id: u32, name: &str, kind: u32, op: u32, value: f32| {
        let mut d = edid(name);
        let mut data = kind.to_le_bytes().to_vec();
        data.extend(op.to_le_bytes());
        data.extend(value.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        record(b"AMEF", id, &d)
    };
    let mut ammo_effects = effect(HP_DAMAGE, "TestHPDamage", 0, 1, 1.75);
    ammo_effects.extend(effect(HP_THRESHOLD, "TestHPThreshold", 2, 1, 3.0));
    plugin.extend(group(*b"AMEF", 0, &ammo_effects));
    let mut hollow = edid("TestHollowPoint");
    hollow.extend(sub(b"RCIL", &HP_DAMAGE.to_le_bytes()));
    hollow.extend(sub(b"RCIL", &HP_THRESHOLD.to_le_bytes()));
    let mut ammunition = record(b"AMMO", HOLLOW_POINT, &hollow);
    // A round that leaves a case (`DAT2`: projectiles, projectile, weight,
    // the item left, the percentage of shots leaving it).
    let mut cased = edid("TestCasedRound");
    cased.extend(sub(b"FULL", &zstr("Cased Round")));
    let mut dat2 = 1u32.to_le_bytes().to_vec();
    dat2.extend(0u32.to_le_bytes());
    dat2.extend(0.03f32.to_le_bytes());
    dat2.extend(CASE.to_le_bytes());
    dat2.extend(25.0f32.to_le_bytes());
    cased.extend(sub(b"DAT2", &dat2));
    ammunition.extend(record(b"AMMO", CASED_AMMO, &cased));
    plugin.extend(group(*b"AMMO", 0, &ammunition));
    let mut cowhand_list = edid("CowhandList");
    cowhand_list.extend(sub(b"LNAM", &PISTOL.to_le_bytes()));
    let mut lists = record(b"FLST", COWHAND_LIST, &cowhand_list);
    let mut repair_list = edid("TestPistolRepairList");
    repair_list.extend(sub(b"LNAM", &RIFLE.to_le_bytes()));
    lists.extend(record(b"FLST", REPAIR_LIST, &repair_list));
    plugin.extend(group(*b"FLST", 0, &lists));
    let mut testville = edid("RepTestville");
    testville.extend(sub(b"FULL", &zstr("Testville")));
    testville.extend(sub(b"DATA", &20.0f32.to_le_bytes()));
    plugin.extend(group(*b"REPU", 0, &record(b"REPU", REPUTATION, &testville)));
    plugin.extend(group(*b"IMAD", 0, &record(b"IMAD", FLASH, &flash)));
    let mut weapons = record(b"WEAP", PISTOL, &pistol);
    weapons.extend(record(b"WEAP", RIFLE, &rifle));
    weapons.extend(record(b"WEAP", REVOLVER, &revolver));
    // A gun with three mod slots (`WMI1`–`WMI3`; `DNAM` effects at 140,
    // values at 152).
    let mut mod_gun = edid("TestModGun");
    mod_gun.extend(sub(b"FULL", &zstr("Mod Gun")));
    mod_gun.extend(sub(b"MODL", &zstr("Weapons\\ModGun.NIF")));
    mod_gun.extend(sub(b"MWD1", &zstr("Weapons\\ModGunExt.NIF")));
    mod_gun.extend(sub(b"MWD3", &zstr("Weapons\\ModGunExtBarrel.NIF")));
    mod_gun.extend(sub(b"WNAM", &FIRST_PERSON_GUN.to_le_bytes()));
    mod_gun.extend(sub(b"WNM1", &FIRST_PERSON_GUN_EXT.to_le_bytes()));
    for (sig, item) in [
        (b"WMI1", EXT_MAG),
        (b"WMI2", BARREL),
        (b"WMI3", LIGHT_FRAME),
    ] {
        mod_gun.extend(sub(sig, &item.to_le_bytes()));
    }
    let mut mod_gun_data = 100i32.to_le_bytes().to_vec();
    mod_gun_data.extend(100i32.to_le_bytes());
    mod_gun_data.extend(3.0f32.to_le_bytes());
    mod_gun_data.extend(10i16.to_le_bytes());
    mod_gun_data.push(8);
    mod_gun.extend(sub(b"DATA", &mod_gun_data));
    let mut mod_dnam = weapon_dnam.clone();
    mod_dnam[0] = 3;
    for (i, (effect, value)) in [(2u32, 7.0f32), (1, 5.0), (4, 1.0)].into_iter().enumerate() {
        mod_dnam[140 + 4 * i..144 + 4 * i].copy_from_slice(&effect.to_le_bytes());
        mod_dnam[152 + 4 * i..156 + 4 * i].copy_from_slice(&value.to_le_bytes());
    }
    mod_gun.extend(sub(b"DNAM", &mod_dnam));
    weapons.extend(record(b"WEAP", MOD_GUN, &mod_gun));
    let mut loud_gun = edid("TestLoudGun");
    loud_gun.extend(sub(b"WMI1", &SILENCER.to_le_bytes()));
    loud_gun.extend(sub(b"WMI2", &BEAM_SPLITTER.to_le_bytes()));
    loud_gun.extend(sub(b"DATA", &mod_gun_data));
    let mut loud_dnam = weapon_dnam.clone();
    loud_dnam[0] = 3;
    loud_dnam[140..144].copy_from_slice(&11u32.to_le_bytes());
    loud_dnam[144..148].copy_from_slice(&12u32.to_le_bytes());
    loud_dnam[156..160].copy_from_slice(&2.0f32.to_le_bytes());
    loud_dnam[188..192].copy_from_slice(&0.5f32.to_le_bytes());
    loud_dnam[16..20].copy_from_slice(&2.0f32.to_le_bytes()); // min spread
    loud_gun.extend(sub(b"DNAM", &loud_dnam));
    loud_gun.extend(sub(b"VNAM", &0u32.to_le_bytes()));
    weapons.extend(record(b"WEAP", LOUD_GUN, &loud_gun));
    let mut knife = edid("TestKnife");
    knife.extend(sub(b"DATA", &mod_gun_data));
    let mut knife_dnam = weapon_dnam.clone();
    knife_dnam[0] = 1;
    knife.extend(sub(b"DNAM", &knife_dnam));
    weapons.extend(record(b"WEAP", KNIFE, &knife));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    let first_person = |id: u32, name: &str, model: &str| {
        let mut d = edid(name);
        d.extend(sub(b"MODL", &zstr(model)));
        record(b"STAT", id, &d)
    };
    let mut first_person_models = first_person(
        FIRST_PERSON_GUN,
        "1stPersonModGun",
        "Weapons\\1stModGun.NIF",
    );
    first_person_models.extend(first_person(
        FIRST_PERSON_GUN_EXT,
        "1stPersonModGunExt",
        "Weapons\\1stModGunExt.NIF",
    ));
    plugin.extend(group(*b"STAT", 0, &first_person_models));
    let weapon_mod = |id: u32, name: &str| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        let mut data = 20i32.to_le_bytes().to_vec();
        data.extend(0.5f32.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        record(b"IMOD", id, &d)
    };
    let mut mods = weapon_mod(EXT_MAG, "TestExtMag");
    mods.extend(weapon_mod(BARREL, "TestBarrel"));
    mods.extend(weapon_mod(LIGHT_FRAME, "TestLightFrame"));
    mods.extend(weapon_mod(SILENCER, "TestSilencer"));
    mods.extend(weapon_mod(BEAM_SPLITTER, "TestBeamSplitter"));
    plugin.extend(group(*b"IMOD", 0, &mods));
    plugin.extend(group(*b"CREA", 0, &record(b"CREA", GECKO, &gecko)));
    let mut bodies = record(
        b"BPTD",
        DEFAULT_BODY_PARTS,
        &body_part_data(
            "DefaultBodyPartData",
            &[
                ("Head", "Bip01 Neck1", 1, 2.0, 20, 25),
                ("Torso", "Bip01", 0, 1.0, 60, 26),
                ("Left Arm", "Bip01 L UpperArm", 3, 1.0, 25, 27),
                ("Left Leg", "Bip01 L Thigh", 7, 1.0, 25, 29),
                ("Right Leg", "Bip01 R Thigh", 10, 1.0, 25, 30),
                ("Right Arm", "Bip01 R UpperArm", 5, 1.0, 25, 28),
            ],
        ),
    );
    bodies.extend(record(
        b"BPTD",
        PLAYER_BODY_PARTS,
        &body_part_data(
            "PlayerBodyPartData",
            &[
                ("Left Leg", "Bip01 L Thigh", 7, 1.0, 150, 29),
                ("Right Leg", "Bip01 R Thigh", 10, 1.0, 150, 30),
                ("Torso", "Bip01", 0, 1.0, 255, 26),
                ("Head", "Bip01 Neck1", 1, 1.0, 75, 25),
            ],
        ),
    ));
    bodies.extend(record(
        b"BPTD",
        GECKO_BODY_PARTS,
        &body_part_data(
            "TestGeckoParts",
            &[
                ("Head", "Bip01 Neck1", 1, 2.0, 25, 25),
                ("Torso", "Bip01 Spine1", 0, 1.0, 75, 26),
            ],
        ),
    ));
    plugin.extend(group(*b"BPTD", 0, &bodies));
    plugin.extend(group(*b"ACTI", 0, &record(b"ACTI", BOTTLE, &bottle)));
    effects.extend(record(b"MGEF", HEAL_EFFECT, &heal_effect));
    plugin.extend(group(*b"MGEF", 0, &effects));
    let mut aid = record(b"ALCH", MEDICINE, &medicine);
    aid.extend(record(b"ALCH", STIMPAK, &stimpak));
    aid.extend(record(b"ALCH", TONIC, &tonic));
    aid.extend(record(b"ALCH", FOOD, &food));
    aid.extend(record(b"ALCH", CHEM, &chem));
    plugin.extend(group(*b"ALCH", 0, &aid));
    plugin.extend(group(*b"SPEL", 0, &spells));
    plugin.extend(group(
        *b"BOOK",
        0,
        &record(b"BOOK", SCIENCE_BOOK, &science_book),
    ));
    let mut chair = edid("TestChair");
    chair.extend(sub(b"MODL", &zstr("Furniture\\TestChair.nif")));
    plugin.extend(group(*b"FURN", 0, &record(b"FURN", CHAIR, &chair)));
    let mut key = edid("TestKey");
    key.extend(sub(b"FULL", &zstr("Strongbox Key")));
    plugin.extend(group(*b"KEYM", 0, &record(b"KEYM", KEY, &key)));
    let mut note = edid("TestNote");
    note.extend(sub(b"FULL", &zstr("Memo")));
    note.extend(sub(b"DATA", &[1]));
    note.extend(sub(b"TNAM", &zstr("The code is 1234.")));
    plugin.extend(group(*b"NOTE", 0, &record(b"NOTE", NOTE, &note)));
    let mut terminal = edid("TestTerminal");
    terminal.extend(sub(b"FULL", &zstr("Office Terminal")));
    terminal.extend(sub(b"DESC", &zstr("Welcome, USER")));
    terminal.extend(sub(b"DNAM", &[1, 0, 3, 0]));
    terminal.extend(sub(b"ITXT", &zstr("Disengage Lock")));
    terminal.extend(sub(b"RNAM", &zstr("Unlocking...")));
    terminal.extend(sub(b"ANAM", &[0]));
    terminal.extend(sub(
        b"SCTX",
        &zstr("ref myLink\nset myLink to GetLinkedRef\nmyLink.Unlock"),
    ));
    terminal.extend(sub(b"ITXT", &zstr("Read memo")));
    terminal.extend(sub(b"ANAM", &[1]));
    terminal.extend(sub(b"INAM", &NOTE.to_le_bytes()));
    plugin.extend(group(*b"TERM", 0, &record(b"TERM", TERMINAL, &terminal)));
    plugin.extend(group(*b"ARMO", 0, &apparel));
    let mut raiders = edid("TestRaiders");
    raiders.extend(sub(b"DATA", &[0; 4]));
    let mut factions = record(b"FACT", RAIDERS, &raiders);
    // The player's faction, as in the game (`0001B2A4`).
    let mut player_faction = edid("PlayerFaction");
    player_faction.extend(sub(b"DATA", &[0; 4]));
    factions.extend(record(b"FACT", 0x1B2A4, &player_faction));
    plugin.extend(group(*b"FACT", 0, &factions));
    let mut misc = record(b"MISC", CAPS, &caps);
    misc.extend(record(b"MISC", CUP, &cup));
    let mut case = edid("TestCase");
    case.extend(sub(b"FULL", &zstr("Case")));
    let mut case_data = 1i32.to_le_bytes().to_vec();
    case_data.extend(0.0f32.to_le_bytes());
    case.extend(sub(b"DATA", &case_data));
    misc.extend(record(b"MISC", CASE, &case));
    let mut bundle = edid("TestCaseBundle");
    bundle.extend(sub(b"FULL", &zstr("Case Bundle")));
    bundle.extend(sub(b"SCRI", &CASE_BUNDLE_SCRIPT.to_le_bytes()));
    bundle.extend(sub(b"DATA", &case_data));
    misc.extend(record(b"MISC", CASE_BUNDLE, &bundle));
    let mut late = edid("TestLateBundle");
    late.extend(sub(b"FULL", &zstr("Late Bundle")));
    late.extend(sub(b"SCRI", &LATE_BUNDLE_SCRIPT.to_le_bytes()));
    late.extend(sub(b"DATA", &case_data));
    misc.extend(record(b"MISC", LATE_BUNDLE, &late));
    for (id, name, full, s) in [
        (KEEPSAKE, "TestKeepsake", "Keepsake", KEEPSAKE_SCRIPT),
        (
            REFUSED_GIFT,
            "TestRefusedGift",
            "Refused Gift",
            REFUSED_GIFT_SCRIPT,
        ),
    ] {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(full)));
        d.extend(sub(b"SCRI", &s.to_le_bytes()));
        d.extend(sub(b"DATA", &case_data));
        misc.extend(record(b"MISC", id, &d));
    }
    plugin.extend(group(*b"MISC", 0, &misc));
    plugin.extend(group(*b"LVLI", 0, &record(b"LVLI", LEVELED, &leveled)));
    let mut packages = record(b"PACK", TRAVEL, &travel);
    packages.extend(record(b"PACK", FAR_TRAVEL, &far_travel));
    packages.extend(record(b"PACK", FOLLOW_PLAYER, &follow));
    plugin.extend(group(*b"PACK", 0, &packages));
    plugin.extend(group(
        *b"REGN",
        0,
        &record(b"REGN", REGION, &edid("TestRegion")),
    ));
    plugin.extend(group(*b"SCPT", 0, &scripts));
    plugin.extend(group(*b"QUST", 0, &record(b"QUST", QUEST, &quest)));
    plugin.extend(group(*b"VTYP", 0, &record(b"VTYP", VOICE, &voice)));
    let mut npcs = record(b"NPC_", PLAYER, &player);
    npcs.extend(record(b"NPC_", DOC, &doc));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(group(*b"DOOR", 0, &record(b"DOOR", DOOR, &door)));
    plugin.extend(group(*b"CONT", 0, &record(b"CONT", CHEST, &chest)));
    plugin.extend(group(*b"DIAL", 0, &dialogue));
    // Caravan cards (suit, then value) and a deck.
    let caravan_card = |id: u32, name: &str, suit: u32, value: u32| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        d.extend(sub(b"SCRI", &CARD_SCRIPT.to_le_bytes()));
        d.extend(sub(b"INTV", &suit.to_le_bytes()));
        d.extend(sub(b"INTV", &value.to_le_bytes()));
        d.extend(sub(b"DATA", &2u32.to_le_bytes()));
        record(b"CCRD", id, &d)
    };
    let mut cards = caravan_card(CARD_ACE, "TestCardAce", 1, 1);
    cards.extend(caravan_card(CARD_QUEEN, "TestCardQueen", 2, 13));
    plugin.extend(group(*b"CCRD", 0, &cards));
    let mut caravan_deck = edid("TestCaravanDeck");
    caravan_deck.extend(sub(b"FULL", &zstr("Test Deck")));
    caravan_deck.extend(sub(b"CARD", &CARD_ACE.to_le_bytes()));
    caravan_deck.extend(sub(b"CARD", &CARD_QUEEN.to_le_bytes()));
    plugin.extend(group(
        *b"CDCK",
        0,
        &record(b"CDCK", CARAVAN_DECK, &caravan_deck),
    ));
    // A casino and its chip.
    let mut chip = edid("TestChip");
    chip.extend(sub(b"FULL", &zstr("Test Chip")));
    chip.extend(sub(b"MODL", &zstr("clutter\\casino\\PokerChip01.nif")));
    plugin.extend(group(*b"CHIP", 0, &record(b"CHIP", CHIP, &chip)));
    let mut casino = edid("TestCasino");
    casino.extend(sub(b"FULL", &zstr("Gommorah")));
    for m in [
        "NV_Blackjack-Chip_001.NIF",
        "NV_Blackjack-Chip_005.NIF",
        "NV_Blackjack-Chip_010.NIF",
        "NV_Blackjack-Chip_025.NIF",
        "NV_Blackjack-Chip_100.NIF",
        "NV_Blackjack-Chip_S.NIF",
        "NV_Roulette-Chip.NIF",
        "NV_SlotMachine-Minigame-Gom.NIF",
    ] {
        casino.extend(sub(b"MODL", &zstr(m)));
    }
    casino.extend(sub(b"MOD2", &zstr("NV_SlotMachine-Minigame-Gom.NIF")));
    casino.extend(sub(b"MOD3", &zstr("NV_Blackjack-Table-Gom.NIF")));
    casino.extend(sub(b"MOD4", &zstr("NV_Roulette-Table-Gom.NIF")));
    for n in [1, 2, 3, 4, 6, 7, 5] {
        casino.extend(sub(
            b"ICON",
            &zstr(&format!("NV_SlotMachine-Symbol{n}.dds")),
        ));
    }
    for _ in 0..4 {
        casino.extend(sub(b"ICO2", &zstr("DeckG\\cardG_back.dds")));
    }
    let mut casino_data = Vec::new();
    casino_data.extend(0.2f32.to_le_bytes());
    casino_data.extend(1.5f32.to_le_bytes());
    for _ in 0..7 {
        casino_data.extend(2i32.to_le_bytes());
    }
    casino_data.extend(3i32.to_le_bytes());
    casino_data.extend(9000i32.to_le_bytes());
    casino_data.extend(CHIP.to_le_bytes());
    casino_data.extend(QUEST.to_le_bytes());
    casino_data.extend([0u8; 4]);
    casino.extend(sub(b"DATA", &casino_data));
    plugin.extend(group(*b"CSNO", 0, &record(b"CSNO", CASINO, &casino)));
    // The default objects: the Stimpak first.
    let mut objects = edid("DefaultObjectManager");
    let mut slots = STIMPAK.to_le_bytes().to_vec();
    slots.extend([0; 4 * 33]);
    objects.extend(sub(b"DATA", &slots));
    plugin.extend(group(
        *b"DOBJ",
        0,
        &record(b"DOBJ", DEFAULT_OBJECTS, &objects),
    ));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}

/// Whether an RGB(A) pixel is within `tolerance` of `rgb` on every channel.
pub fn close_to(pixel: &[u8], rgb: [u8; 3], tolerance: i32) -> bool {
    pixel
        .iter()
        .zip(rgb)
        .all(|(&a, b)| (i32::from(a) - i32::from(b)).abs() <= tolerance)
}

/// Form IDs in the [`sitting`] world.
pub mod sitting_ids {
    /// The idle tree: `FurnitureIdles` (blocking) → `Sitting` (blocking) →
    /// `ChairSitting` (marker 14) → `StandUp`, `SittingChairIdles`,
    /// `DefaultSitIdle`, `SitDown`; then the root `GeneralIdles`
    /// (blocking) with `Shrug`, and a loose `Wave`.
    pub const FURNITURE_IDLES: u32 = 0x900;
    pub const SITTING: u32 = 0x901;
    pub const CHAIR_SITTING: u32 = 0x902;
    pub const STAND_UP: u32 = 0x903;
    pub const CHAIR_FRONT_STAND: u32 = 0x904;
    pub const SITTING_CHAIR_IDLES: u32 = 0x905;
    pub const RELAX: u32 = 0x906;
    pub const DEFAULT_SIT_IDLE: u32 = 0x907;
    pub const CHAIR_DYNAMIC_IDLE: u32 = 0x908;
    pub const SIT_DOWN: u32 = 0x909;
    pub const CHAIR_FRONT_SIT: u32 = 0x90A;
    pub const GENERAL_IDLES: u32 = 0x90B;
    pub const SHRUG: u32 = 0x90C;
    pub const WAVE: u32 = 0x90D;
    /// A chair (sitting furniture, marker 0 usable) and a bed.
    pub const CHAIR: u32 = 0x910;
    pub const BED: u32 = 0x911;
    /// An idle marker playing `WAVE` (in order), timer 16.
    pub const IDLE_MARKER: u32 = 0x912;
    pub const FOOD: u32 = 0x913;
    /// Someone sandboxing (energy 50) and someone owning a chair.
    pub const SITTER: u32 = 0x914;
    pub const SANDBOX: u32 = 0x915;
    pub const OWNER: u32 = 0x916;
    pub const CELL: u32 = 0x920;
    pub const SITTER_REF: u32 = 0x921;
    pub const CHAIR_REF: u32 = 0x922;
    /// A chair the owner owns.
    pub const OWNED_CHAIR_REF: u32 = 0x923;
    pub const BED_REF: u32 = 0x924;
    pub const IDLE_MARKER_REF: u32 = 0x925;
    pub const FOOD_REF: u32 = 0x926;
    /// The owner, 2000 units away (out of the sandbox's 512).
    pub const OWNER_REF: u32 = 0x927;
    /// A chair marked "ignored by sandbox" (`XIBS`).
    pub const IGNORED_CHAIR_REF: u32 = 0x928;
    pub const SETTINGS: u32 = 0x930;
    pub const GLOBALS: u32 = 0x940;
}

/// A world for sitting, idles and sandboxing: an idle tree shaped like the
/// game's furniture branch (`sitting_ids`), the chair marker 14 settings
/// (`fFurnitureMarker14…` as the game has them) and
/// `fSandboxDurationMultFurniture` 3, a chair (`MNAM` 0x40000001), a bed
/// (0x80000001), an idle marker, food, and `TestSitter` whose sandbox
/// package (no eating, 512 around their editor location) is their only
/// one, placed in `TestSittingCell` at the origin with the chair 100 east,
/// another chair 100 west owned by `TestOwner`, the bed 100 north, the
/// marker 100 south, the food at (50, 50) and a chair 200 north that's
/// ignored by sandboxes; the clock at 15:00.
pub fn sitting(tag: &str) -> TempData {
    use sitting_ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    // A condition with its comparison byte (0x00 =, 0x40 >, 0x80 <) and
    // OR flag (0x01).
    let ctda = |function: u16, param: u32, comparison: u8, value: f32| {
        let mut c = condition(function, [param, 0], value);
        c[6] = comparison;
        c
    };
    const GET_SITTING: u16 = 159;
    const GET_SLEEPING: u16 = 49;
    const MARKER_ID: u16 = 160;
    let folder = "Characters\\_Male\\IdleAnims";
    let mut idles = Vec::new();
    let mut idle = |id: u32,
                    name: &str,
                    file: Option<&str>,
                    (parent, previous): (u32, u32),
                    data: [u8; 8],
                    conditions: Vec<Vec<u8>>| {
        let mut d = edid(name);
        let model = file.map_or(folder.to_string(), |f| format!("{folder}\\{f}"));
        d.extend(sub(b"MODL", &zstr(&model)));
        for c in conditions {
            d.extend(c);
        }
        let mut anam = parent.to_le_bytes().to_vec();
        anam.extend(previous.to_le_bytes());
        d.extend(sub(b"ANAM", &anam));
        d.extend(sub(b"DATA", &data));
        idles.extend(record(b"IDLE", id, &d));
    };
    let blocking = [0x84, 0, 0, 0, 0, 0, 0, 0];
    idle(
        FURNITURE_IDLES,
        "FurnitureIdles",
        None,
        (0, 0),
        blocking,
        vec![
            ctda(GET_SITTING, 0, 0x41, 0.0),
            ctda(GET_SLEEPING, 0, 0x40, 0.0),
        ],
    );
    idle(
        SITTING,
        "Sitting",
        None,
        (FURNITURE_IDLES, 0),
        blocking,
        vec![ctda(GET_SITTING, 0, 0x40, 0.0)],
    );
    idle(
        CHAIR_SITTING,
        "ChairSitting",
        None,
        (SITTING, 0),
        [7, 0, 0, 0, 0, 0, 0, 0],
        vec![ctda(MARKER_ID, 0, 0, 14.0)],
    );
    idle(
        STAND_UP,
        "StandUp",
        None,
        (CHAIR_SITTING, 0),
        [0x94, 0, 0, 0, 0, 0, 0, 0],
        vec![ctda(GET_SITTING, 0, 0, 4.0)],
    );
    idle(
        CHAIR_FRONT_STAND,
        "ChairFrontStand",
        Some("Chair_ForwardExit.kf"),
        (STAND_UP, 0),
        [0x14, 0, 0, 0, 0, 0, 0, 0],
        vec![ctda(MARKER_ID, 0, 0, 14.0)],
    );
    idle(
        SITTING_CHAIR_IDLES,
        "SittingChairIdles",
        None,
        (CHAIR_SITTING, STAND_UP),
        [7, 0, 0, 0, 0, 0, 0, 0],
        vec![ctda(GET_SITTING, 0, 0, 3.0)],
    );
    // Loops 1 to 3, a 40 s replay delay.
    idle(
        RELAX,
        "SitChairRelaxIdleA",
        Some("SitChairRelaxA.kf"),
        (SITTING_CHAIR_IDLES, 0),
        [7, 1, 3, 0, 40, 0, 0, 0],
        Vec::new(),
    );
    idle(
        DEFAULT_SIT_IDLE,
        "DefaultSitIdle",
        None,
        (CHAIR_SITTING, SITTING_CHAIR_IDLES),
        [4, 0, 0, 0, 0, 0, 0, 0],
        vec![ctda(GET_SITTING, 0, 0, 1.0)],
    );
    idle(
        CHAIR_DYNAMIC_IDLE,
        "ChairDynamicIdle",
        Some("DynamicIdle_ChairSit.kf"),
        (DEFAULT_SIT_IDLE, 0),
        [0; 8],
        Vec::new(),
    );
    idle(
        SIT_DOWN,
        "SitDown",
        None,
        (CHAIR_SITTING, DEFAULT_SIT_IDLE),
        blocking,
        vec![ctda(GET_SITTING, 0, 0, 2.0)],
    );
    idle(
        CHAIR_FRONT_SIT,
        "ChairFrontSit",
        Some("Chair_ForwardEnter.kf"),
        (SIT_DOWN, 0),
        [0x14, 0, 0, 0, 0, 0, 0, 0],
        vec![ctda(MARKER_ID, 0, 0, 14.0)],
    );
    idle(
        GENERAL_IDLES,
        "GeneralIdles",
        None,
        (0, FURNITURE_IDLES),
        [0x87, 0, 0, 0, 0, 0, 0, 0],
        Vec::new(),
    );
    idle(
        SHRUG,
        "Shrug",
        Some("Shrug.kf"),
        (GENERAL_IDLES, 0),
        [7, 0, 0, 0, 0, 0, 0, 0],
        Vec::new(),
    );
    idle(
        WAVE,
        "LooseWave",
        Some("Wave.kf"),
        (0, GENERAL_IDLES),
        [0x47, 0, 0, 0, 0, 0, 0, 0],
        Vec::new(),
    );

    let setting = |id: u32, name: &str, value: f32| {
        let mut d = edid(name);
        d.extend(sub(b"DATA", &value.to_le_bytes()));
        record(b"GMST", id, &d)
    };
    let mut settings = Vec::new();
    for (i, (name, value)) in [
        ("fFurnitureMarker14DeltaX", 2.4809f32),
        ("fFurnitureMarker14DeltaY", 57.3572),
        ("fFurnitureMarker14DeltaZ", -28.948),
        ("fFurnitureMarker14HeadingDelta", std::f32::consts::PI),
        ("fSandboxDurationMultFurniture", 3.0),
    ]
    .into_iter()
    .enumerate()
    {
        settings.extend(setting(SETTINGS + i as u32, name, value));
    }
    let mut globals = Vec::new();
    for (i, (name, value)) in [
        ("GameHour", 15.0f32),
        ("TimeScale", 30.0),
        ("GameDaysPassed", 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = edid(name);
        d.extend(sub(b"FNAM", b"f"));
        d.extend(sub(b"FLTV", &value.to_le_bytes()));
        globals.extend(record(b"GLOB", GLOBALS + i as u32, &d));
    }

    let furniture = |id: u32, name: &str, mnam: u32| {
        let mut d = edid(name);
        d.extend(sub(b"MODL", &zstr("Furniture\\TestChair.nif")));
        d.extend(sub(b"MNAM", &mnam.to_le_bytes()));
        record(b"FURN", id, &d)
    };
    let mut furn = furniture(CHAIR, "TestSitChair", 0x4000_0001);
    furn.extend(furniture(BED, "TestBed", 0x8000_0001));
    let mut marker = edid("TestIdleMarker");
    marker.extend(sub(b"IDLF", &[0x01]));
    marker.extend(sub(b"IDLC", &[1]));
    marker.extend(sub(b"IDLT", &16.0f32.to_le_bytes()));
    marker.extend(sub(b"IDLA", &WAVE.to_le_bytes()));
    let mut food = edid("TestFood");
    food.extend(sub(b"DATA", &0.5f32.to_le_bytes()));

    // The sandbox: type 12, no eating (type flags 0x01), 512 around the
    // editor location (PLDT kind 3), any time.
    let mut sandbox = edid("TestSandbox");
    sandbox.extend(sub(b"PKDT", &[0, 0, 0, 0, 12, 0, 0, 0, 0x01, 0, 0, 0]));
    let mut pldt = 3u32.to_le_bytes().to_vec();
    pldt.extend(0u32.to_le_bytes());
    pldt.extend(512u32.to_le_bytes());
    sandbox.extend(sub(b"PLDT", &pldt));
    sandbox.extend(sub(b"PSDT", &[0xFF, 0xFF, 0, 0xFF, 0, 0, 0, 0]));
    let npc = |id: u32, name: &str, package: Option<u32>| {
        let mut d = edid(name);
        d.extend(sub(b"ACBS", &[0; 24]));
        // Energy 50 (AIDT byte 2).
        let mut aidt = vec![0u8; 20];
        aidt[2] = 50;
        d.extend(sub(b"AIDT", &aidt));
        if let Some(p) = package {
            d.extend(sub(b"PKID", &p.to_le_bytes()));
        }
        record(b"NPC_", id, &d)
    };
    let mut npcs = npc(SITTER, "TestSitter", Some(SANDBOX));
    npcs.extend(npc(OWNER, "TestOwner", None));

    let actor = |id: u32, base: u32, pos: [f32; 3]| {
        let mut r = placed(id, base, pos, [0.0; 3], &[]);
        r[..4].copy_from_slice(b"ACHR");
        r
    };
    let mut refs = actor(SITTER_REF, SITTER, [0.0, 0.0, 0.0]);
    refs.extend(placed(CHAIR_REF, CHAIR, [100.0, 0.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(
        OWNED_CHAIR_REF,
        CHAIR,
        [-100.0, 0.0, 0.0],
        [0.0; 3],
        &sub(b"XOWN", &OWNER.to_le_bytes()),
    ));
    refs.extend(placed(BED_REF, BED, [0.0, 100.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(
        IDLE_MARKER_REF,
        IDLE_MARKER,
        [0.0, -100.0, 0.0],
        [0.0, 0.0, 90.0],
        &[],
    ));
    refs.extend(placed(FOOD_REF, FOOD, [50.0, 50.0, 0.0], [0.0; 3], &[]));
    refs.extend(actor(OWNER_REF, OWNER, [2000.0, 0.0, 0.0]));
    refs.extend(placed(
        IGNORED_CHAIR_REF,
        CHAIR,
        [0.0, 200.0, 0.0],
        [0.0; 3],
        &sub(b"XIBS", &[]),
    ));
    let mut cell = edid("TestSittingCell");
    cell.extend(sub(b"DATA", &[1]));
    let mut contents = record(b"CELL", CELL, &cell);
    contents.extend(group(
        CELL.to_le_bytes(),
        6,
        &group(CELL.to_le_bytes(), 9, &refs),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"GMST", 0, &settings));
    plugin.extend(group(*b"GLOB", 0, &globals));
    plugin.extend(group(*b"FURN", 0, &furn));
    plugin.extend(group(*b"IDLM", 0, &record(b"IDLM", IDLE_MARKER, &marker)));
    plugin.extend(group(*b"ALCH", 0, &record(b"ALCH", FOOD, &food)));
    plugin.extend(group(*b"PACK", 0, &record(b"PACK", SANDBOX, &sandbox)));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(group(*b"IDLE", 0, &idles));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}
