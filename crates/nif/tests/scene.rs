//! NIF tests against files assembled byte by byte.

use nif::{Error, Nif};

const ROT_Z90: [[f32; 3]; 3] = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
const IDENTITY: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

struct Xf {
    translation: [f32; 3],
    rotation: [[f32; 3]; 3],
    scale: f32,
}

const NO_XF: Xf = Xf {
    translation: [0.0; 3],
    rotation: IDENTITY,
    scale: 1.0,
};

struct Builder {
    version: u32,
    bs_version: u32,
    av_flags_u32: bool,
    material_crc: bool,
    strings: Vec<String>,
    blocks: Vec<Option<(String, Vec<u8>)>>,
    roots: Vec<i32>,
}

fn f32s(out: &mut Vec<u8>, values: &[f32]) {
    for v in values {
        out.extend(v.to_le_bytes());
    }
}

fn sized(s: &str) -> Vec<u8> {
    let mut v = (s.len() as u32).to_le_bytes().to_vec();
    v.extend(s.as_bytes());
    v
}

impl Builder {
    fn new() -> Self {
        Builder {
            version: 0x1402_0007,
            bs_version: 34,
            av_flags_u32: true,
            material_crc: false,
            strings: Vec::new(),
            blocks: Vec::new(),
            roots: vec![0],
        }
    }

    fn reserve(&mut self, n: usize) {
        self.blocks.resize(n, None);
    }

    fn set(&mut self, index: usize, type_name: &str, bytes: Vec<u8>) {
        if self.blocks.len() <= index {
            self.blocks.resize(index + 1, None);
        }
        self.blocks[index] = Some((type_name.to_string(), bytes));
    }

    fn string(&mut self, s: &str) -> i32 {
        if s.is_empty() {
            return -1;
        }
        match self.strings.iter().position(|x| x == s) {
            Some(i) => i as i32,
            None => {
                self.strings.push(s.to_string());
                (self.strings.len() - 1) as i32
            }
        }
    }

    fn object_net(&mut self, name: &str) -> Vec<u8> {
        let mut v = self.string(name).to_le_bytes().to_vec();
        v.extend(0u32.to_le_bytes()); // no extra data
        v.extend((-1i32).to_le_bytes()); // no controller
        v
    }

    fn av(&mut self, name: &str, flags: u32, xf: &Xf, props: &[i32]) -> Vec<u8> {
        let mut v = self.object_net(name);
        if self.av_flags_u32 {
            v.extend(flags.to_le_bytes());
        } else {
            v.extend((flags as u16).to_le_bytes());
        }
        f32s(&mut v, &xf.translation);
        for row in &xf.rotation {
            f32s(&mut v, row);
        }
        f32s(&mut v, &[xf.scale]);
        v.extend((props.len() as u32).to_le_bytes());
        for p in props {
            v.extend(p.to_le_bytes());
        }
        v.extend((-1i32).to_le_bytes()); // collision
        v
    }

    fn node(
        &mut self,
        name: &str,
        flags: u32,
        xf: &Xf,
        props: &[i32],
        children: &[i32],
    ) -> Vec<u8> {
        let mut v = self.av(name, flags, xf, props);
        v.extend((children.len() as u32).to_le_bytes());
        for c in children {
            v.extend(c.to_le_bytes());
        }
        v.extend(0u32.to_le_bytes()); // effects
        v
    }

    fn shape(&mut self, name: &str, flags: u32, xf: &Xf, props: &[i32], data: i32) -> Vec<u8> {
        let mut v = self.av(name, flags, xf, props);
        v.extend(data.to_le_bytes());
        v.extend((-1i32).to_le_bytes()); // skin
        v.extend(0u32.to_le_bytes()); // material count
        v.extend((-1i32).to_le_bytes()); // active material
        v.push(0); // material needs update
        v
    }

    #[allow(clippy::too_many_arguments)]
    fn geometry_data(
        &self,
        positions: &[[f32; 3]],
        normals: &[[f32; 3]],
        tangents: bool,
        colors: &[[f32; 4]],
        uvs: &[[f32; 2]],
    ) -> Vec<u8> {
        let mut v = 0i32.to_le_bytes().to_vec(); // group id
        v.extend((positions.len() as u16).to_le_bytes());
        v.extend([0, 0]); // keep, compress flags
        v.push(1);
        for p in positions {
            f32s(&mut v, p);
        }
        let flags: u16 = u16::from(!uvs.is_empty()) | if tangents { 0x1000 } else { 0 };
        v.extend(flags.to_le_bytes());
        if self.material_crc {
            v.extend(0xDEAD_BEEFu32.to_le_bytes());
        }
        v.push(u8::from(!normals.is_empty()));
        for n in normals {
            f32s(&mut v, n);
        }
        if tangents {
            // First array, then second: along x, then along y.
            for _ in positions {
                f32s(&mut v, &[1.0, 0.0, 0.0]);
            }
            for _ in positions {
                f32s(&mut v, &[0.0, 1.0, 0.0]);
            }
        }
        f32s(&mut v, &[0.0, 0.0, 0.0, 1.5]); // bounding sphere
        v.push(u8::from(!colors.is_empty()));
        for c in colors {
            f32s(&mut v, c);
        }
        for uv in uvs {
            f32s(&mut v, uv);
        }
        v.extend(0u16.to_le_bytes()); // consistency
        v.extend((-1i32).to_le_bytes()); // additional data
        v
    }

    fn shape_data(
        &self,
        positions: &[[f32; 3]],
        normals: &[[f32; 3]],
        uvs: &[[f32; 2]],
        triangles: &[[u16; 3]],
    ) -> Vec<u8> {
        let colors = vec![[1.0, 0.5, 0.25, 1.0]; positions.len()];
        let mut v = self.geometry_data(positions, normals, true, &colors, uvs);
        v.extend((triangles.len() as u16).to_le_bytes());
        v.extend(((triangles.len() * 3) as u32).to_le_bytes());
        v.push(1);
        for t in triangles {
            for i in t {
                v.extend(i.to_le_bytes());
            }
        }
        // One match group of two vertices.
        v.extend(1u16.to_le_bytes());
        v.extend(2u16.to_le_bytes());
        v.extend([0, 0, 1, 0]);
        v
    }

    fn strips_data(&self, positions: &[[f32; 3]], strips: &[&[u16]]) -> Vec<u8> {
        let mut v = self.geometry_data(positions, &[], false, &[], &[]);
        v.extend(0u16.to_le_bytes()); // triangle count (unused)
        v.extend((strips.len() as u16).to_le_bytes());
        for s in strips {
            v.extend((s.len() as u16).to_le_bytes());
        }
        v.push(1);
        for s in strips {
            for p in *s {
                v.extend(p.to_le_bytes());
            }
        }
        v
    }

    fn shader_common(&mut self) -> Vec<u8> {
        let mut v = self.object_net("");
        v.extend(1u16.to_le_bytes()); // shade flags
        v.extend(1u32.to_le_bytes()); // shader type
        v.extend(0x8000_0000u32.to_le_bytes()); // flags
        v.extend(1u32.to_le_bytes()); // flags 2
        f32s(&mut v, &[1.0]); // env map scale
        v.extend(3u32.to_le_bytes()); // clamp mode
        v
    }

    fn lit_shader(&mut self, texture_set: i32) -> Vec<u8> {
        let mut v = self.shader_common();
        v.extend(texture_set.to_le_bytes());
        f32s(&mut v, &[0.0]); // refraction strength
        v.extend(0i32.to_le_bytes()); // refraction period
        f32s(&mut v, &[4.0, 1.0]); // parallax
        v
    }

    fn unlit_shader(&mut self, file: &str) -> Vec<u8> {
        let mut v = self.shader_common();
        v.extend(sized(file));
        f32s(&mut v, &[0.9, 0.2, 0.8, 0.1]); // falloff
        v
    }

    fn build(&self) -> Vec<u8> {
        let blocks: Vec<&(String, Vec<u8>)> = self
            .blocks
            .iter()
            .map(|b| b.as_ref().expect("every reserved block filled"))
            .collect();
        let mut types: Vec<&str> = Vec::new();
        for (t, _) in &blocks {
            if !types.contains(&t.as_str()) {
                types.push(t);
            }
        }
        let v = self.version;
        let mut out = format!(
            "Gamebryo File Format, Version {}.{}.{}.{}\n",
            v >> 24,
            (v >> 16) & 0xFF,
            (v >> 8) & 0xFF,
            v & 0xFF
        )
        .into_bytes();
        out.extend(self.version.to_le_bytes());
        out.push(1);
        out.extend(11u32.to_le_bytes());
        out.extend((blocks.len() as u32).to_le_bytes());
        out.extend(self.bs_version.to_le_bytes());
        for s in ["tester", "", ""] {
            out.push((s.len() + 1) as u8);
            out.extend(s.as_bytes());
            out.push(0);
        }
        out.extend((types.len() as u16).to_le_bytes());
        for t in &types {
            out.extend(sized(t));
        }
        for (t, _) in &blocks {
            let i = types.iter().position(|x| x == t).unwrap() as u16;
            out.extend(i.to_le_bytes());
        }
        for (_, bytes) in &blocks {
            out.extend((bytes.len() as u32).to_le_bytes());
        }
        out.extend((self.strings.len() as u32).to_le_bytes());
        out.extend((self.strings.iter().map(String::len).max().unwrap_or(0) as u32).to_le_bytes());
        for s in &self.strings {
            out.extend(sized(s));
        }
        out.extend(0u32.to_le_bytes()); // groups
        for (_, bytes) in &blocks {
            out.extend(bytes);
        }
        out.extend((self.roots.len() as u32).to_le_bytes());
        for r in &self.roots {
            out.extend(r.to_le_bytes());
        }
        out
    }
}

const SQUARE: [[f32; 3]; 4] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [-1.0, 0.0, 0.0],
    [0.0, -1.0, 0.0],
];

/// Scene used by most tests. Block indexes are fixed so blocks can refer
/// to each other before they are written.
fn sample(mut b: Builder) -> Vec<u8> {
    b.reserve(21);
    let root_xf = Xf {
        translation: [10.0, 0.0, 0.0],
        rotation: ROT_Z90,
        scale: 2.0,
    };
    let root = b.node(
        "Scene Root",
        0x0008_000E,
        &root_xf,
        &[7],
        &[1, 2, 3, 4, 5, 6, 20, 99],
    );
    b.set(0, "BSFadeNode", root);
    let box_xf = Xf {
        translation: [0.0, 0.0, 1.0],
        ..NO_XF
    };
    let shape = b.shape("Box", 0, &box_xf, &[9, 10], 11);
    b.set(1, "NiTriShape", shape);
    let shape = b.shape("Strip", 0, &NO_XF, &[12, 13], 14);
    b.set(2, "NiTriStrips", shape);
    let shape = b.shape("Hidden", 1, &NO_XF, &[], 11);
    b.set(3, "NiTriShape", shape);
    let node = b.node("EditorMarker", 0, &NO_XF, &[], &[15]);
    b.set(4, "NiNode", node);
    let node = b.node("Collision", 0, &NO_XF, &[], &[16]);
    b.set(5, "RootCollisionNode", node);
    let mut switch = b.node("Switch", 0, &NO_XF, &[], &[17, 18]);
    switch.extend(0u16.to_le_bytes());
    switch.extend(1u32.to_le_bytes()); // second child active
    b.set(6, "NiSwitchNode", switch);

    let mut alpha = b.object_net("");
    alpha.extend(0x0201u16.to_le_bytes());
    alpha.push(128);
    b.set(7, "NiAlphaProperty", alpha);
    b.set(8, "bhkCollisionObject", vec![1, 2, 3, 4, 5, 6]);
    let lit = b.lit_shader(19);
    b.set(9, "BSShaderPPLightingProperty", lit);
    let mut material = b.object_net("");
    f32s(
        &mut material,
        &[1.0, 1.0, 1.0, 0.1, 0.2, 0.3, 10.0, 0.75, 1.0],
    );
    b.set(10, "NiMaterialProperty", material);
    let normals = [[0.0, 0.0, 1.0]; 4];
    let uvs = [[1.0, 0.25], [0.5, 0.0], [0.0, 0.5], [0.5, 1.0]];
    let data = b.shape_data(&SQUARE, &normals, &uvs, &[[0, 1, 2], [0, 2, 3], [0, 1, 9]]);
    b.set(11, "NiTriShapeData", data);
    let unlit = b.unlit_shader("textures\\fx\\glow.dds");
    b.set(12, "BSShaderNoLightingProperty", unlit);
    let mut stencil = b.object_net("");
    stencil.extend((3u16 << 10).to_le_bytes());
    stencil.extend(0u32.to_le_bytes());
    stencil.extend(0xFFFF_FFFFu32.to_le_bytes());
    b.set(13, "NiStencilProperty", stencil);
    let five = [
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 2.0, 0.0],
    ];
    let strips = b.strips_data(&five, &[&[0, 1, 2, 3, 4], &[4, 4, 0, 1]]);
    b.set(14, "NiTriStripsData", strips);
    for (i, name) in [
        (15, "UnderMarker"),
        (16, "CollisionMesh"),
        (17, "SwitchOff"),
        (18, "SwitchOn"),
    ] {
        let shape = b.shape(name, 0, &NO_XF, &[], 11);
        b.set(i, "NiTriShape", shape);
    }
    let mut set = 6i32.to_le_bytes().to_vec();
    for t in [
        "textures\\test\\box.dds",
        "textures\\test\\box_n.dds",
        "",
        "",
        "",
        "",
    ] {
        set.extend(sized(t));
    }
    b.set(19, "BSShaderTextureSet", set);
    b.set(20, "NiPointLight", vec![0; 10]);
    b.build()
}

/// Game units per Havok unit.
const H: f32 = nif::collision::HAVOK_SCALE;

fn close(a: [f32; 3], b: [f32; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-4)
}

/// A `bhkRigidBody(T)` as the game's files lay it out (236 bytes): shape,
/// layer, then the translation at 52, rotation quaternion at 68 and
/// motion system at 212.
fn rigid_body(shape: i32, layer: u8, translation: [f32; 3], quat: [f32; 4], motion: u8) -> Vec<u8> {
    let mut v = vec![0u8; 236];
    v[0..4].copy_from_slice(&shape.to_le_bytes());
    v[4] = layer;
    for (k, x) in translation.iter().enumerate() {
        v[52 + 4 * k..56 + 4 * k].copy_from_slice(&x.to_le_bytes());
    }
    for (k, x) in quat.iter().enumerate() {
        v[68 + 4 * k..72 + 4 * k].copy_from_slice(&x.to_le_bytes());
    }
    v[212] = motion;
    v
}

/// A ragdoll body as the game's skeletons lay it out: shape, layer, the
/// frame at 52/68, inertia rows at 116, centre at 164, mass and the rest at
/// 180, then the constraints.
fn ragdoll_body(shape: i32, translation: [f32; 3], constraints: &[i32]) -> Vec<u8> {
    let mut v = rigid_body(shape, 8, translation, [0.0, 0.0, 0.0, 1.0], 6);
    v.truncate(228);
    // Flags 0x40 and part number 2 (the body) after the layer.
    v[5] = 0x40 | 2;
    let mut put = |at: usize, x: f32| v[at..at + 4].copy_from_slice(&x.to_le_bytes());
    put(116, 1.0);
    put(136, 2.0);
    put(156, 3.0);
    put(164, 0.5);
    for (k, x) in [8.0, 0.1, 0.05, 10.0, 0.8, 100.0, 30.0]
        .into_iter()
        .enumerate()
    {
        put(180 + 4 * k, x);
    }
    v.extend((constraints.len() as u32).to_le_bytes());
    for c in constraints {
        v.extend(c.to_le_bytes());
    }
    v.extend(0u32.to_le_bytes()); // body flags
    v
}

#[test]
fn reads_a_segmented_shapes_segments() {
    let mut b = Builder::new();
    b.reserve(3);
    let root = b.node("Scene Root", 0, &NO_XF, &[], &[1]);
    b.set(0, "BSFadeNode", root);
    // A shape with three triangles, the first in one segment, the other
    // two in the next.
    let mut shape = b.shape("Block", 0, &NO_XF, &[], 2);
    shape.extend(2u32.to_le_bytes());
    for (flags, index, count) in [(0u8, 0u32, 1u32), (1, 3, 2)] {
        shape.push(flags);
        shape.extend(index.to_le_bytes());
        shape.extend(count.to_le_bytes());
    }
    b.set(1, "BSSegmentedTriShape", shape);
    let five = [
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 2.0, 0.0],
    ];
    let data = b.shape_data(
        &five,
        &[[0.0, 0.0, 1.0]; 5],
        &[[0.0, 0.0]; 5],
        &[[0, 1, 2], [1, 3, 2], [2, 3, 4]],
    );
    b.set(2, "NiTriShapeData", data);
    let nif = Nif::parse(b.build()).unwrap();
    let mesh = &nif.scene().unwrap().meshes[0];
    assert_eq!(mesh.triangles.len(), 3);
    let segments = nif.segments(mesh.block);
    assert_eq!(
        segments,
        [
            nif::Segment {
                flags: 0,
                first_triangle: 0,
                triangles: 1
            },
            nif::Segment {
                flags: 1,
                first_triangle: 1,
                triangles: 2
            }
        ]
    );
    // Other shapes have none.
    assert!(nif.segments(0).is_empty());
}

#[test]
fn reads_a_skeletons_bound() {
    let mut b = Builder::new();
    b.reserve(2);
    // The top node with one extra data block: the BSBound.
    let mut root = b.string("Bip01").to_le_bytes().to_vec();
    root.extend(1u32.to_le_bytes());
    root.extend(1i32.to_le_bytes());
    root.extend((-1i32).to_le_bytes());
    root.extend(0u32.to_le_bytes());
    f32s(&mut root, &[0.0; 3]);
    for row in &NO_XF.rotation {
        f32s(&mut root, row);
    }
    f32s(&mut root, &[1.0]);
    root.extend(0u32.to_le_bytes()); // properties
    root.extend((-1i32).to_le_bytes()); // collision
    root.extend(0u32.to_le_bytes()); // children
    root.extend(0u32.to_le_bytes()); // effects
    b.set(0, "NiNode", root);
    let mut bound = b.string("BBX").to_le_bytes().to_vec();
    f32s(&mut bound, &[0.0, 0.0, 50.0, 30.0, 70.0, 50.0]);
    b.set(1, "BSBound", bound);
    let nif = Nif::parse(b.build()).unwrap();
    assert_eq!(
        nif.bound(),
        Some(nif::Bound {
            center: [0.0, 0.0, 50.0],
            half_extents: [30.0, 70.0, 50.0]
        })
    );
    // A model without one.
    assert_eq!(Nif::parse(sample(Builder::new())).unwrap().bound(), None);
}

#[test]
fn reads_a_skeletons_ragdoll() {
    use nif::JointLimit;
    let mut b = Builder::new();
    b.reserve(11);
    let root = b.node("Scene Root", 0, &NO_XF, &[], &[1]);
    b.set(0, "BSFadeNode", root);
    // The pelvis 70 up, the spine 10 above it; each with a body.
    let up = |z: f32| Xf {
        translation: [0.0, 0.0, z],
        ..NO_XF
    };
    let mut pelvis = b.node("Bip01 Pelvis", 0, &up(70.0), &[], &[2]);
    pelvis[72..76].copy_from_slice(&3i32.to_le_bytes());
    b.set(1, "NiNode", pelvis);
    let mut spine = b.node("Bip01 Spine", 0, &up(10.0), &[], &[]);
    spine[72..76].copy_from_slice(&6i32.to_le_bytes());
    b.set(2, "NiNode", spine);
    let blend = |target: i32, body: i32| {
        let mut v = target.to_le_bytes().to_vec();
        v.extend(1u16.to_le_bytes());
        v.extend(body.to_le_bytes());
        f32s(&mut v, &[1.0, 1.0]);
        v
    };
    b.set(3, "bhkBlendCollisionObject", blend(1, 4));
    b.set(4, "bhkRigidBody", ragdoll_body(5, [0.0, 0.0, 10.0], &[]));
    let mut capsule = 0u32.to_le_bytes().to_vec();
    f32s(&mut capsule, &[1.0, 0.0, 0.0]);
    f32s(&mut capsule, &[0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0]);
    b.set(5, "bhkCapsuleShape", capsule);
    b.set(6, "bhkBlendCollisionObject", blend(2, 7));
    b.set(
        7,
        "bhkRigidBody",
        ragdoll_body(8, [0.0, 0.0, 80.0 / H], &[9, 10]),
    );
    let mut sphere = 0u32.to_le_bytes().to_vec();
    f32s(&mut sphere, &[0.5]);
    b.set(8, "bhkSphereShape", sphere);
    // A ragdoll joint, spine on pelvis, at the spine's origin.
    let common = |v: &mut Vec<u8>| {
        v.extend(2u32.to_le_bytes());
        v.extend(7i32.to_le_bytes());
        v.extend(4i32.to_le_bytes());
        v.extend(1u32.to_le_bytes());
    };
    let mut ragdoll = Vec::new();
    common(&mut ragdoll);
    for v4 in [
        [0.0, 0.0, 1.0, 0.0], // twist A
        [1.0, 0.0, 0.0, 0.0], // plane A
        [0.0, 1.0, 0.0, 0.0], // motor A
        [0.0, 0.0, 0.0, 0.0], // pivot A
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 10.0 / H, 0.0], // pivot B
    ] {
        f32s(&mut ragdoll, &v4);
    }
    f32s(&mut ragdoll, &[0.3, -0.1, 0.2, -0.4, 0.5, 100.0]);
    ragdoll.push(0);
    assert_eq!(ragdoll.len(), 169);
    b.set(9, "bhkRagdollConstraint", ragdoll);
    // And a hinge wrapped in a malleable constraint (type 2).
    let mut hinge = Vec::new();
    common(&mut hinge);
    hinge.extend(2u32.to_le_bytes());
    common(&mut hinge);
    for v4 in [
        [0.0, 1.0, 0.0, 0.0], // axle A
        [1.0, 0.0, 0.0, 0.0], // perpendicular A
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 0.0], // pivot A
        [0.0, 1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 10.0 / H, 0.0], // pivot B
    ] {
        f32s(&mut hinge, &v4);
    }
    f32s(&mut hinge, &[-0.5, 1.5, 100.0]);
    hinge.push(0);
    f32s(&mut hinge, &[0.9]);
    assert_eq!(hinge.len(), 181);
    b.set(10, "bhkMalleableConstraint", hinge);

    let nif = Nif::parse(b.build()).unwrap();
    let ragdoll = nif.ragdoll().unwrap().expect("a ragdoll");
    assert_eq!(ragdoll.bodies.len(), 2);
    let pelvis = &ragdoll.bodies[0];
    assert_eq!(
        (pelvis.bone_name.as_str(), pelvis.bone),
        ("Bip01 Pelvis", 1)
    );
    // Havok units × 6.9991257 (the game's scale), inertia × its square;
    // the part number from the filter's second byte.
    assert!(close(pelvis.frame.translation, [0.0, 0.0, 10.0 * H]));
    assert!(close(pelvis.center, [0.5 * H, 0.0, 0.0]));
    assert!(close(pelvis.inertia, [H * H, 2.0 * H * H, 3.0 * H * H]));
    assert_eq!(pelvis.part, 2);
    assert_eq!(
        (
            pelvis.mass,
            pelvis.friction,
            pelvis.linear_damping,
            pelvis.layer
        ),
        (8.0, 10.0, 0.1, 8)
    );
    assert_eq!(
        (pelvis.max_linear_speed, pelvis.max_angular_speed),
        (100.0 * H, 30.0)
    );
    let (a, c, r) = pelvis.capsule.unwrap();
    assert!(close(a, [0.0; 3]) && close(c, [H, 0.0, 0.0]) && r == H);
    let spine = &ragdoll.bodies[1];
    assert_eq!(spine.bone_name, "Bip01 Spine");
    assert!(close(spine.frame.translation, [0.0, 0.0, 80.0]));
    assert_eq!(spine.capsule.map(|c| c.2), Some(0.5 * H));

    assert_eq!(ragdoll.joints.len(), 2);
    let joint = &ragdoll.joints[0];
    assert_eq!(joint.bodies, [1, 0]);
    assert!(close(joint.pivots[0], [0.0; 3]) && close(joint.pivots[1], [0.0, 0.0, 10.0]));
    let JointLimit::Ragdoll {
        twist,
        plane,
        cone,
        plane_range,
        twist_range,
    } = joint.limit
    else {
        panic!("{:?}", joint.limit)
    };
    assert!(close(twist[0], [0.0, 0.0, 1.0]) && close(plane[1], [1.0, 0.0, 0.0]));
    assert_eq!(
        (cone, plane_range, twist_range),
        (0.3, (-0.1, 0.2), (-0.4, 0.5))
    );
    let JointLimit::Hinge {
        axle,
        perpendicular,
        range,
    } = ragdoll.joints[1].limit
    else {
        panic!("{:?}", ragdoll.joints[1].limit)
    };
    assert!(close(axle[1], [0.0, 1.0, 0.0]) && close(perpendicular[0], [1.0, 0.0, 0.0]));
    assert_eq!(range, (-0.5, 1.5));
    assert!(close(ragdoll.joints[1].pivots[1], [0.0, 0.0, 10.0]));
}

#[test]
fn reads_collision_shapes_in_game_units() {
    use nif::CollisionShape;
    let mut b = Builder::new();
    b.reserve(9);
    // Root node (moved, which a placed model ignores) whose collision
    // object names a rigid body with its own offset and a 90° turn.
    let root_xf = Xf {
        translation: [100.0, 0.0, 0.0],
        rotation: IDENTITY,
        scale: 1.0,
    };
    let mut root = b.node("Scene Root", 0x0008_000E, &root_xf, &[], &[5]);
    // The node's collision link is the av block's last field (no
    // properties here): bytes 72..76.
    root[72..76].copy_from_slice(&1i32.to_le_bytes());
    b.set(0, "BSFadeNode", root);
    let mut object = 0i32.to_le_bytes().to_vec();
    object.extend(1u16.to_le_bytes());
    object.extend(2i32.to_le_bytes());
    b.set(1, "bhkCollisionObject", object);
    let s = std::f32::consts::FRAC_1_SQRT_2;
    let mut body = rigid_body(3, 4, [1.0, 0.0, 0.0], [0.0, 0.0, s, s], 1);
    // Its physics values: inertia rows (diagonal 1, 2, 3), the centre of
    // mass 1 Havok unit up, mass 2, damping, friction, restitution, most
    // speeds, penetration depth, then deactivator, solver deactivation
    // and quality after the motion system.
    for (at, x) in [
        (116, 1.0f32),
        (136, 2.0),
        (156, 3.0),
        (172, 1.0),
        (180, 2.0),
        (184, 0.1),
        (188, 0.05),
        (192, 0.5),
        (196, 0.4),
        (200, 100.0),
        (204, 31.57),
        (208, 0.15),
    ] {
        body[at..at + 4].copy_from_slice(&x.to_le_bytes());
    }
    body[213..216].copy_from_slice(&[2, 2, 3]);
    b.set(2, "bhkRigidBodyT", body);
    // A box 2 x 1 x 0.5 Havok units across (half sizes 1, 0.5, 0.25), of
    // Havok material 9 (wood).
    let mut boxed = vec![0u8; 16];
    boxed[0..4].copy_from_slice(&9u32.to_le_bytes());
    f32s(&mut boxed, &[1.0, 0.5, 0.25, 0.0]);
    b.set(3, "bhkBoxShape", boxed);
    // A second collision object on a child node: a packed triangle mesh
    // in two runs, a static floor and a non-collidable piece.
    let mut child = b.node("Floor", 0, &NO_XF, &[], &[]);
    child[72..76].copy_from_slice(&6i32.to_le_bytes());
    b.set(5, "NiNode", child);
    let mut object = 5i32.to_le_bytes().to_vec();
    object.extend(1u16.to_le_bytes());
    object.extend(7i32.to_le_bytes());
    b.set(6, "bhkCollisionObject", object);
    b.set(
        7,
        "bhkRigidBody",
        rigid_body(4, 1, [0.0; 3], [0.0, 0.0, 0.0, 1.0], 7),
    );
    let mut packed = vec![0u8; 16];
    f32s(&mut packed, &[1.0, 1.0, 1.0, 0.0]);
    packed.extend([0u8; 20]);
    packed.extend(8i32.to_le_bytes());
    b.set(4, "bhkPackedNiTriStripsShape", packed);
    let mut data = 2u32.to_le_bytes().to_vec();
    for t in [[0u16, 1, 2, 0], [3, 4, 5, 0]] {
        for x in t {
            data.extend(x.to_le_bytes());
        }
    }
    data.extend(6u32.to_le_bytes());
    data.push(0); // not compressed
    f32s(
        &mut data,
        &[
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, // floor
            0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
        ],
    );
    data.extend(2u16.to_le_bytes());
    // The runs' Havok materials: dirt (2), hollow metal (16).
    for (layer, count, material) in [(1u8, 3u32, 2u32), (15, 3, 16)] {
        data.extend([layer, 0, 0, 0]);
        data.extend(count.to_le_bytes());
        data.extend(material.to_le_bytes());
    }
    b.set(8, "hkPackedNiTriStripsData", data);

    let nif = Nif::parse(b.build()).unwrap();
    let collision = nif.placed_collision().unwrap();
    assert!(collision.unhandled.is_empty(), "{:?}", collision.unhandled);
    assert_eq!(collision.parts.len(), 3);

    // The box: Havok units times 7, turned 90° and moved 1 Havok unit
    // along x by its body; the top node's transform left out.
    let boxed = &collision.parts[0];
    assert_eq!((boxed.layer, boxed.dynamic), (4, true));
    // The body's values: in game units, its centre placed with the body
    // (1 Havok unit along x, then up), its tensor turned 90° about z (x
    // and y swap) and scaled by 7².
    let body = &boxed.body;
    assert_eq!((body.block, body.mass, body.motion), (2, 2.0, 1));
    assert_eq!(
        (body.deactivator, body.solver_deactivation, body.quality),
        (2, 2, 3)
    );
    assert!(close(body.center, [H, 0.0, H]), "{:?}", body.center);
    let h2 = H * H;
    assert!(
        (body.inertia[0][0] - 2.0 * h2).abs() < 1e-3,
        "{:?}",
        body.inertia
    );
    assert!((body.inertia[1][1] - 1.0 * h2).abs() < 1e-3);
    assert!((body.inertia[2][2] - 3.0 * h2).abs() < 1e-3);
    assert!(body.inertia[0][1].abs() < 1e-3);
    assert_eq!(
        (
            body.linear_damping,
            body.angular_damping,
            body.friction,
            body.restitution
        ),
        (0.1, 0.05, 0.5, 0.4)
    );
    assert!((body.max_linear_speed - 100.0 * H).abs() < 1e-2);
    assert!((body.max_angular_speed - 31.57).abs() < 1e-5);
    assert!((body.penetration_depth - 0.15 * H).abs() < 1e-5);
    // Parts of the floor's body name its own block.
    assert_eq!(collision.parts[1].body.block, 7);
    // Each part keeps its shape's (or sub-part's) Havok material.
    assert_eq!(boxed.material, 9);
    assert_eq!(
        (collision.parts[1].material, collision.parts[2].material),
        (2, 16)
    );
    let CollisionShape::Convex { vertices, planes } = &boxed.shape else {
        panic!("{:?}", boxed.shape)
    };
    let lo = (0..3).map(|k| vertices.iter().map(|v| v[k]).fold(f32::INFINITY, f32::min));
    let hi = (0..3).map(|k| {
        vertices
            .iter()
            .map(|v| v[k])
            .fold(f32::NEG_INFINITY, f32::max)
    });
    let lo: Vec<f32> = lo.collect();
    let hi: Vec<f32> = hi.collect();
    // x: 1 ± 0.5 Havok units (the half height 0.5 turned onto x); y: ±1.
    assert!(
        close([lo[0], lo[1], lo[2]], [0.5 * H, -H, -0.25 * H]),
        "{lo:?}"
    );
    assert!(
        close([hi[0], hi[1], hi[2]], [1.5 * H, H, 0.25 * H]),
        "{hi:?}"
    );
    // Every corner lies on or inside every face.
    for v in vertices {
        for p in planes {
            assert!(p[0] * v[0] + p[1] * v[1] + p[2] * v[2] + p[3] <= 1e-4);
        }
    }

    // The triangle mesh, split by run: the floor on the static layer, the
    // other piece on the non-collidable one; fixed bodies don't move.
    let floor = &collision.parts[1];
    assert_eq!((floor.layer, floor.dynamic), (1, false));
    let CollisionShape::Triangles {
        vertices,
        triangles,
    } = &floor.shape
    else {
        panic!("{:?}", floor.shape)
    };
    assert_eq!(triangles, &[[0, 1, 2]]);
    assert!(close(vertices[1], [H, 0.0, 0.0]));
    assert_eq!(collision.parts[2].layer, 15);
    assert!(!nif::collision::layers::blocks_walking(15));
    assert!(nif::collision::layers::blocks_walking(1));
}

/// A `bhkCollisionObject` on node `target` naming body `body`.
fn collision_object(target: i32, body: i32) -> Vec<u8> {
    let mut v = target.to_le_bytes().to_vec();
    v.extend(1u16.to_le_bytes());
    v.extend(body.to_le_bytes());
    v
}

/// A `bhkPackedNiTriStripsShape` (56 bytes): radius at 8, scale at 16,
/// the data at 52.
fn packed_shape(radius: f32, scale: f32, data: i32) -> Vec<u8> {
    let mut v = vec![0u8; 8];
    f32s(&mut v, &[radius, 0.0, scale, scale, scale, 0.0, radius]);
    f32s(&mut v, &[scale, scale, scale, 0.0]);
    v.extend(data.to_le_bytes());
    assert_eq!(v.len(), 56);
    v
}

/// `hkPackedNiTriStripsData` with one triangle and one sub-part.
fn packed_triangle(corners: [[f32; 3]; 3], layer: u8) -> Vec<u8> {
    let mut v = 1u32.to_le_bytes().to_vec();
    for x in [0u16, 1, 2, 0] {
        v.extend(x.to_le_bytes());
    }
    v.extend(3u32.to_le_bytes());
    v.push(0);
    for c in corners {
        f32s(&mut v, &c);
    }
    v.extend(1u16.to_le_bytes());
    v.extend([layer, 0, 0, 0]);
    v.extend(3u32.to_le_bytes());
    v.extend(0u32.to_le_bytes());
    v
}

#[test]
fn collision_keeps_its_shapes_scale_not_its_nodes() {
    // A static collection's piece placed at half size: its node is scaled
    // for drawing and its packed triangles carry the same scale (as the
    // game's `meshes\scol` files do). The body takes the node's place and
    // turn but not its scale, so the triangles end up half size, not a
    // quarter.
    let mut b = Builder::new();
    b.reserve(6);
    let root = b.node("Scene Root", 0, &NO_XF, &[], &[1]);
    b.set(0, "BSFadeNode", root);
    let piece_xf = Xf {
        translation: [10.0, 0.0, 0.0],
        rotation: ROT_Z90,
        scale: 0.5,
    };
    let mut piece = b.node("Piece", 0, &piece_xf, &[], &[]);
    piece[72..76].copy_from_slice(&2i32.to_le_bytes());
    b.set(1, "NiNode", piece);
    b.set(2, "bhkCollisionObject", collision_object(1, 3));
    b.set(
        3,
        "bhkRigidBody",
        rigid_body(4, 1, [0.0; 3], [0.0, 0.0, 0.0, 1.0], 7),
    );
    b.set(4, "bhkPackedNiTriStripsShape", packed_shape(0.1, 0.5, 5));
    b.set(
        5,
        "hkPackedNiTriStripsData",
        packed_triangle([[0.0; 3], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]], 1),
    );
    let nif = Nif::parse(b.build()).unwrap();
    let collision = nif.placed_collision().unwrap();
    assert_eq!(collision.parts.len(), 1);
    let part = &collision.parts[0];
    let nif::CollisionShape::Triangles { vertices, .. } = &part.shape else {
        panic!("{:?}", part.shape)
    };
    // (2, 0, 0) Havok units × 0.5 × 7, turned 90° onto y, at the node.
    assert!(close(vertices[1], [10.0, H, 0.0]), "{:?}", vertices[1]);
    // The node it hangs on, and the shell around it (0.1 Havok units).
    assert_eq!(part.node, 1);
    assert!((part.shell - 0.1 * H).abs() < 1e-5, "{}", part.shell);
    assert!(!part.dynamic && !part.keyframed);
}

#[test]
fn keyframed_bodies_are_marked_and_no_collision_bodies_left_out() {
    // A door: its frame (fixed) on the top node, its leaf (keyframed, the
    // door's animation swings it) on a child; and a body flagged "no
    // collision" (0x40 in the filter's second byte), which collides with
    // nothing in the game.
    let mut b = Builder::new();
    b.reserve(10);
    let root = b.node("Door", 0, &NO_XF, &[], &[4, 7]);
    let mut root = root;
    root[72..76].copy_from_slice(&1i32.to_le_bytes());
    b.set(0, "BSFadeNode", root);
    b.set(1, "bhkCollisionObject", collision_object(0, 2));
    b.set(
        2,
        "bhkRigidBody",
        rigid_body(3, 1, [0.0; 3], [0.0, 0.0, 0.0, 1.0], 7),
    );
    let mut frame = 0u32.to_le_bytes().to_vec();
    f32s(&mut frame, &[0.1, 0.0, 0.0]);
    f32s(&mut frame, &[1.0, 1.0, 1.0, 0.0]);
    b.set(3, "bhkBoxShape", frame);
    let mut leaf = b.node("Leaf", 0, &NO_XF, &[], &[]);
    leaf[72..76].copy_from_slice(&5i32.to_le_bytes());
    b.set(4, "NiNode", leaf);
    b.set(5, "bhkCollisionObject", collision_object(4, 6));
    b.set(
        6,
        "bhkRigidBody",
        rigid_body(3, 2, [0.0; 3], [0.0, 0.0, 0.0, 1.0], 6),
    );
    let mut ghost = b.node("Ghost", 0, &NO_XF, &[], &[]);
    ghost[72..76].copy_from_slice(&8i32.to_le_bytes());
    b.set(7, "NiNode", ghost);
    b.set(8, "bhkCollisionObject", collision_object(7, 9));
    let mut body = rigid_body(3, 1, [0.0; 3], [0.0, 0.0, 0.0, 1.0], 7);
    body[5] = 0x40;
    b.set(9, "bhkRigidBody", body);
    let nif = Nif::parse(b.build()).unwrap();
    let collision = nif.placed_collision().unwrap();
    assert_eq!(collision.parts.len(), 2, "{:?}", collision.parts);
    assert_eq!(collision.no_collision, 1);
    let frame = &collision.parts[0];
    assert_eq!((frame.layer, frame.keyframed, frame.node), (1, false, 0));
    // A box's shell is its radius (0.1 Havok units).
    assert!((frame.shell - 0.1 * H).abs() < 1e-5);
    let leaf = &collision.parts[1];
    assert_eq!(
        (leaf.layer, leaf.keyframed, leaf.dynamic, leaf.node),
        (2, true, false, 4)
    );
}

#[test]
fn reads_strips_collision_shapes() {
    // `bhkNiTriStripsShape` (as in `traps\terminaldesktrap01.nif`): two
    // strips data blocks, each with its own filter; the second's layer
    // (non-collidable) comes through as its own.
    let mut b = Builder::new();
    b.reserve(6);
    let mut root = b.node("Trap", 0, &NO_XF, &[], &[]);
    root[72..76].copy_from_slice(&1i32.to_le_bytes());
    b.set(0, "BSFadeNode", root);
    b.set(1, "bhkCollisionObject", collision_object(0, 2));
    b.set(
        2,
        "bhkRigidBody",
        rigid_body(3, 1, [0.0; 3], [0.0, 0.0, 0.0, 1.0], 7),
    );
    let mut strips = 0u32.to_le_bytes().to_vec();
    f32s(&mut strips, &[0.1]);
    strips.extend([0u8; 20]);
    strips.extend(1u32.to_le_bytes());
    f32s(&mut strips, &[1.0, 1.0, 1.0, 0.0]);
    strips.extend(2u32.to_le_bytes());
    strips.extend(4i32.to_le_bytes());
    strips.extend(5i32.to_le_bytes());
    strips.extend(2u32.to_le_bytes());
    strips.extend([1u8, 0, 0, 0]);
    strips.extend([15u8, 0, 0, 0]);
    assert_eq!(strips.len(), 72);
    b.set(3, "bhkNiTriStripsShape", strips);
    let square = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
    ];
    let data = b.strips_data(&square, &[&[0, 1, 2, 3]]);
    b.set(4, "NiTriStripsData", data);
    let data = b.strips_data(&square, &[&[0, 1, 2]]);
    b.set(5, "NiTriStripsData", data);
    let nif = Nif::parse(b.build()).unwrap();
    let collision = nif.placed_collision().unwrap();
    assert!(collision.unhandled.is_empty(), "{:?}", collision.unhandled);
    assert_eq!(collision.parts.len(), 2);
    let nif::CollisionShape::Triangles {
        vertices,
        triangles,
    } = &collision.parts[0].shape
    else {
        panic!()
    };
    assert_eq!(triangles.len(), 2);
    assert!(close(vertices[3], [H, H, 0.0]));
    assert_eq!(collision.parts[0].layer, 1);
    assert!((collision.parts[0].shell - 0.1 * H).abs() < 1e-5);
    assert_eq!(collision.parts[1].layer, 15);
}

#[test]
fn reads_the_header_and_block_table() {
    let nif = Nif::parse(sample(Builder::new())).unwrap();
    let h = nif.header();
    assert_eq!(h.version, 0x1402_0007);
    assert_eq!((h.user_version, h.bs_version), (11, 34));
    assert_eq!(h.author, "tester");
    assert_eq!(nif.blocks().len(), 21);
    assert_eq!(nif.block_type(0), "BSFadeNode");
    assert_eq!(nif.block_type(8), "bhkCollisionObject");
    assert_eq!(nif.roots(), &[0]);
    assert_eq!(nif.block_name(1).as_deref(), Some("Box"));
    assert_eq!(nif.type_counts()[0], ("NiTriShape".to_string(), 6));
}

#[test]
fn collects_visible_meshes_only() {
    let scene = Nif::parse(sample(Builder::new())).unwrap().scene().unwrap();
    let names: Vec<&str> = scene.meshes.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(names, ["Box", "Strip", "SwitchOn"]);
    assert_eq!(scene.unhandled.get("NiPointLight"), Some(&1));
    assert_eq!(scene.invalid_references, 1);
    // One out-of-range triangle in the shared data, drawn by two shapes.
    assert_eq!(scene.dropped_triangles, 2);
}

#[test]
fn composes_node_transforms() {
    let scene = Nif::parse(sample(Builder::new())).unwrap().scene().unwrap();
    let boxed = &scene.meshes[0];
    // (1,0,0) + box offset (0,0,1) -> scaled by 2 -> rotated 90° about Z -> moved +10 X.
    let first = boxed.model_positions().next().unwrap();
    assert!(close(first, [10.0, 2.0, 2.0]), "{first:?}");
}

#[test]
fn placed_models_leave_out_the_top_node_transform() {
    let nif = Nif::parse(sample(Builder::new())).unwrap();
    let viewer = nif.scene().unwrap();
    let placed = nif.placed_scene().unwrap();
    // Both report the root's transform...
    for scene in [&viewer, &placed] {
        let root = scene.root_transform.unwrap();
        assert_eq!(root.translation, [10.0, 0.0, 0.0]);
        assert_eq!(root.scale, 2.0);
    }
    // ...but only the viewer's scene applies it; child transforms stay.
    let first = placed.meshes[0].model_positions().next().unwrap();
    assert!(close(first, [1.0, 0.0, 1.0]), "{first:?}");
    assert_eq!(placed.meshes.len(), viewer.meshes.len());

    // A root without a transform reports none.
    let mut b = Builder::new();
    b.reserve(3);
    let root = b.node("Root", 0, &NO_XF, &[], &[1]);
    b.set(0, "NiNode", root);
    let shape = b.shape("Plain", 0, &NO_XF, &[], 2);
    b.set(1, "NiTriShape", shape);
    let data = b.shape_data(&SQUARE, &[], &[], &[[0, 1, 2]]);
    b.set(2, "NiTriShapeData", data);
    let plain = Nif::parse(b.build()).unwrap().placed_scene().unwrap();
    assert!(plain.root_transform.is_none());
}

#[test]
fn reads_geometry_and_properties() {
    let scene = Nif::parse(sample(Builder::new())).unwrap().scene().unwrap();
    let boxed = &scene.meshes[0];
    assert_eq!(boxed.positions.len(), 4);
    assert_eq!(boxed.normals.len(), 4);
    assert_eq!(boxed.uvs[0], [1.0, 0.25]);
    assert_eq!(boxed.colors[0], [1.0, 0.5, 0.25, 1.0]);
    assert_eq!(boxed.triangles, vec![[0, 1, 2], [0, 2, 3]]);
    assert_eq!(boxed.diffuse_texture(), Some("textures\\test\\box.dds"));
    assert_eq!(boxed.textures[1], "textures\\test\\box_n.dds");
    assert_eq!(boxed.normal_texture(), Some("textures\\test\\box_n.dds"));
    // The tangent space stored after the normals, both arrays in order.
    assert_eq!(boxed.tangents, vec![[1.0, 0.0, 0.0]; 4]);
    assert_eq!(boxed.bitangents, vec![[0.0, 1.0, 0.0]; 4]);
    let material = boxed.material.as_ref().unwrap();
    assert_eq!(material.emissive, [0.1, 0.2, 0.3]);
    assert_eq!(material.alpha, 0.75);
    // The alpha property sits on the root node and is inherited.
    let alpha = boxed.alpha.unwrap();
    assert!(alpha.blending() && alpha.testing());
    assert_eq!(alpha.threshold, 128);
    assert!(!boxed.double_sided);
    assert!(!boxed.skinned);
    assert!(boxed.shader.as_ref().unwrap().layout_ok);
}

#[test]
fn reads_the_depth_property_and_the_stored_bound() {
    // A shape under a node carrying an `NiZBufferProperty` (test on, write
    // off, "less or equal"), scaled by 2 and moved up 1.
    let mut b = Builder::new();
    b.reserve(4);
    let root = b.node("Root", 0, &NO_XF, &[1], &[2]);
    b.set(0, "NiNode", root);
    let mut zbuffer = b.object_net("");
    zbuffer.extend((0x0001u16 | (3 << 2)).to_le_bytes());
    b.set(1, "NiZBufferProperty", zbuffer);
    let xf = Xf {
        translation: [0.0, 0.0, 1.0],
        rotation: IDENTITY,
        scale: 2.0,
    };
    let shape = b.shape("Glass", 0, &xf, &[], 3);
    b.set(2, "NiTriShape", shape);
    let data = b.shape_data(&SQUARE, &[[0.0, 0.0, 1.0]; 4], &[], &[[0, 1, 2]]);
    b.set(3, "NiTriShapeData", data);
    let scene = Nif::parse(b.build()).unwrap().placed_scene().unwrap();
    let mesh = &scene.meshes[0];
    let z = mesh.zbuffer.expect("the node's depth property applies");
    assert!(z.test() && !z.write());
    assert!(scene.unreadable_properties.is_empty());
    // The data's bounding sphere (centre 0,0,0, radius 1.5), as stored and
    // in the model's space.
    assert_eq!(mesh.bound, ([0.0; 3], 1.5));
    assert_eq!(mesh.model_bound(), ([0.0, 0.0, 1.0], 3.0));
    // The nodes down to it, as an animation would move them; composed
    // without one they give its transform.
    let names: Vec<&str> = mesh.nodes.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["Root", "Glass"]);
    assert_eq!(mesh.posed_transform(&[]), mesh.transform);
    // Meshes without one have none.
    let plain = Nif::parse(sample(Builder::new())).unwrap().scene().unwrap();
    assert!(plain.meshes.iter().all(|m| m.zbuffer.is_none()));
}

#[test]
fn reads_sequences_changing_a_materials_opacity_and_glow() {
    // A window's "Right": its glow card's alpha from 1 to 0 over 0.25 s,
    // and its glow colour held; the controllers named as the game's files
    // name them.
    let mut b = Builder::new();
    b.reserve(6);
    let root = b.node("Window", 0, &NO_XF, &[], &[]);
    b.set(0, "BSFadeNode", root);
    let mut seq = b.string("Right").to_le_bytes().to_vec();
    seq.extend(2u32.to_le_bytes());
    seq.extend(1u32.to_le_bytes());
    for (interp, controller, id) in [
        (2i32, "NiAlphaController", ""),
        (4, "NiMaterialColorController", "SELF_ILLUM"),
    ] {
        seq.extend(interp.to_le_bytes());
        seq.extend((-1i32).to_le_bytes());
        seq.push(0);
        for s in ["Object01:0", "NiMaterialProperty", controller, id, ""] {
            seq.extend(b.string(s).to_le_bytes());
        }
    }
    f32s(&mut seq, &[1.0]); // weight
    seq.extend(5i32.to_le_bytes()); // text keys
    seq.extend(1u32.to_le_bytes()); // clamp
    f32s(&mut seq, &[1.0, 0.0, 0.333]); // frequency, start, stop
    seq.extend((-1i32).to_le_bytes()); // manager
    seq.extend(b.string("Window").to_le_bytes());
    b.set(1, "NiControllerSequence", seq);
    let mut alpha = Vec::new();
    f32s(&mut alpha, &[f32::MIN]);
    alpha.extend(3i32.to_le_bytes());
    b.set(2, "NiFloatInterpolator", alpha);
    let mut keys = 2u32.to_le_bytes().to_vec();
    keys.extend(1u32.to_le_bytes()); // linear
    f32s(&mut keys, &[0.0, 1.0, 0.25, 0.0]);
    b.set(3, "NiFloatData", keys);
    let mut glow = Vec::new();
    f32s(&mut glow, &[0.27, 0.21, 0.09]);
    glow.extend((-1i32).to_le_bytes()); // no data: the value alone
    b.set(4, "NiPoint3Interpolator", glow);
    // Its text keys, as a door's sequence carries them: `start`, the
    // sound to play, `end`.
    let mut keys = b.string("").to_le_bytes().to_vec();
    keys.extend(3u32.to_le_bytes());
    for (time, text) in [(0.0, "start"), (0.0, "sound: DRSWoodOpen"), (0.333, "end")] {
        f32s(&mut keys, &[time]);
        keys.extend(b.string(text).to_le_bytes());
    }
    b.set(5, "NiTextKeyExtraData", keys);
    let nif = Nif::parse(b.build()).unwrap();
    let right = &nif.sequences().unwrap()[0];
    assert_eq!(right.name, "Right");
    assert!(right.tracks.is_empty());
    assert_eq!(
        right.text_keys,
        vec![
            (0.0, "start".to_string()),
            (0.0, "sound: DRSWoodOpen".to_string()),
            (0.333, "end".to_string())
        ]
    );
    let [alpha, glow] = &right.materials[..] else {
        panic!("{:?}", right.materials);
    };
    assert_eq!(alpha.node, "Object01:0");
    assert_eq!(alpha.target, nif::MaterialTarget::Alpha);
    assert_eq!(alpha.float_at(0.125), Some(0.5));
    assert_eq!(alpha.float_at(0.333), Some(0.0));
    assert_eq!(glow.target, nif::MaterialTarget::Emissive);
    assert_eq!(glow.color_at(0.2), Some([0.27, 0.21, 0.09]));
}

#[test]
fn converts_strips_to_triangles() {
    let scene = Nif::parse(sample(Builder::new())).unwrap().scene().unwrap();
    let strip = &scene.meshes[1];
    // Alternate triangles are flipped; the degenerate joiner is dropped.
    assert_eq!(
        strip.triangles,
        vec![[0, 1, 2], [1, 3, 2], [2, 3, 4], [4, 1, 0]]
    );
    assert_eq!(strip.diffuse_texture(), Some("textures\\fx\\glow.dds"));
    let falloff = strip.shader.as_ref().unwrap().falloff.unwrap();
    assert_eq!(
        [
            falloff.start_angle,
            falloff.stop_angle,
            falloff.start_opacity,
            falloff.stop_opacity
        ],
        [0.9, 0.2, 0.8, 0.1]
    );
    assert!(!strip.shader.as_ref().unwrap().lit);
    assert!(strip.shader.as_ref().unwrap().layout_ok);
    assert!(strip.double_sided);
    assert!(strip.normals.is_empty() && strip.uvs.is_empty());
}

#[test]
fn detects_layout_variants_from_block_sizes() {
    let reference = Nif::parse(sample(Builder::new())).unwrap().scene().unwrap();
    let mut short_flags = Builder::new();
    short_flags.av_flags_u32 = false;
    let mut with_crc = Builder::new();
    with_crc.material_crc = true;
    for builder in [short_flags, with_crc] {
        let scene = Nif::parse(sample(builder)).unwrap().scene().unwrap();
        assert_eq!(scene.meshes, reference.meshes);
    }
}

#[test]
fn exports_obj_with_separate_index_bases() {
    let nif = Nif::parse(sample(Builder::new())).unwrap();
    let scene = nif.scene().unwrap();
    let mut out = Vec::new();
    nif::obj::write_obj(&mut out, &scene.meshes, "test.nif").unwrap();
    let text = String::from_utf8(out).unwrap();
    let count = |prefix: &str| text.lines().filter(|l| l.starts_with(prefix)).count();
    assert_eq!(count("v "), 13);
    assert_eq!(count("vt "), 8);
    assert_eq!(count("vn "), 8);
    assert_eq!(count("o "), 3);
    // Z-up (10, 2, 2) becomes Y-up (10, 2, -2).
    assert!(text.contains("\nv 10 2 -2\n"), "{text}");
    assert!(text.contains("\nf 1/1/1 2/2/2 3/3/3\n"));
    assert!(text.contains("\nf 5 6 7\n"));
    // SwitchOn: vertices continue at 10, UVs and normals at 5 (the strip had none).
    assert!(text.contains("\nf 10/5/5 11/6/6 12/7/7\n"), "{text}");
    assert!(text.contains("# diffuse texture: textures\\test\\box.dds"));
}

#[test]
fn survives_reference_cycles() {
    let mut b = Builder::new();
    b.reserve(2);
    let a = b.node("A", 0, &NO_XF, &[], &[1]);
    b.set(0, "NiNode", a);
    let back = b.node("B", 0, &NO_XF, &[], &[0, 1]);
    b.set(1, "NiNode", back);
    let scene = Nif::parse(b.build()).unwrap().scene().unwrap();
    assert!(scene.meshes.is_empty());
}

#[test]
fn rejects_other_files_and_versions() {
    assert!(matches!(
        Nif::parse(b"TES4 not a mesh".to_vec()),
        Err(Error::NotANif { .. })
    ));
    assert!(matches!(Nif::parse(Vec::new()), Err(Error::NotANif { .. })));

    let mut oblivion = b"Gamebryo File Format, Version 20.0.0.5\n".to_vec();
    oblivion.extend(0x1400_0005u32.to_le_bytes());
    let err = Nif::parse(oblivion).err().unwrap();
    assert!(err.to_string().contains("20.0.0.5"), "{err}");

    let mut skyrim = Builder::new();
    skyrim.bs_version = 83;
    let nif = Nif::parse(sample(skyrim)).unwrap(); // the header still reads
    assert!(matches!(nif.scene(), Err(Error::Unsupported(_))));
}

#[test]
fn reports_truncated_files() {
    let full = sample(Builder::new());
    for cut in [20, 45, 120, full.len() / 2, full.len() - 20] {
        let result = Nif::parse(full[..cut].to_vec()).and_then(|n| n.scene());
        assert!(result.is_err(), "truncating to {cut} bytes should fail");
    }
}

#[test]
fn finds_textures_in_other_material_styles() {
    let mut b = Builder::new();
    b.reserve(12);
    let root = b.node("Root", 0, &NO_XF, &[], &[1, 2, 3, 4]);
    b.set(0, "NiNode", root);
    for (i, name, props) in [
        (1, "Hair", vec![5, 6]),
        (2, "Sky", vec![8]),
        (3, "Grass", vec![9]),
        (4, "Plain", vec![]),
    ] {
        let shape = b.shape(name, 0, &NO_XF, &props, 10);
        b.set(i, "NiTriShape", shape);
    }
    // Hair: a shader with no texture of its own, plus NiTexturingProperty.
    let mut hair_shader = b.shader_common();
    hair_shader.truncate(hair_shader.len() - 4); // no clamp mode on this type
    b.set(5, "HairShaderProperty", hair_shader);
    let mut texturing = b.object_net("");
    texturing.extend(0u16.to_le_bytes()); // flags
    texturing.extend(7u32.to_le_bytes()); // slot count
    texturing.push(1); // has base texture
    texturing.extend(7i32.to_le_bytes()); // -> NiSourceTexture
    texturing.extend([0, 0, 0]); // base slot flags + no transform
    texturing.extend([0; 6]); // other slots absent
    b.set(6, "NiTexturingProperty", texturing);
    let mut source = b.object_net("");
    source.push(1); // external
    let file = b.string("textures\\characters\\hair\\hairmessy.dds");
    source.extend(file.to_le_bytes());
    source.extend((-1i32).to_le_bytes());
    source.extend([0; 15]); // format preferences and flags
    b.set(7, "NiSourceTexture", source);

    let mut sky = b.shader_common();
    sky.extend(sized("textures\\sky\\stars.dds"));
    sky.extend(3u32.to_le_bytes()); // sky object type
    b.set(8, "SkyShaderProperty", sky);
    let mut grass = b.shader_common();
    grass.truncate(grass.len() - 4); // tall grass has no clamp mode
    grass.extend(sized("textures\\landscape\\grass\\grass01.dds"));
    b.set(9, "TallGrassShaderProperty", grass);

    let data = b.shape_data(&SQUARE, &[], &[], &[[0, 1, 2]]);
    b.set(10, "NiTriShapeData", data);
    b.set(11, "NiPointLight", vec![0; 4]);

    let scene = Nif::parse(b.build()).unwrap().scene().unwrap();
    let texture = |name: &str| {
        let mesh = scene.meshes.iter().find(|m| m.name == name).unwrap();
        mesh.diffuse_texture().map(str::to_string)
    };
    assert_eq!(
        texture("Hair").as_deref(),
        Some("textures\\characters\\hair\\hairmessy.dds")
    );
    assert_eq!(texture("Sky").as_deref(), Some("textures\\sky\\stars.dds"));
    assert_eq!(
        texture("Grass").as_deref(),
        Some("textures\\landscape\\grass\\grass01.dds")
    );
    assert_eq!(texture("Plain"), None);

    let hair = &scene.meshes[0];
    assert_eq!(
        hair.property_types,
        ["HairShaderProperty", "NiTexturingProperty"]
    );
    assert!(scene.meshes[3].property_types.is_empty());
    let sky_shader = scene.meshes[1].shader.as_ref().unwrap();
    assert_eq!(sky_shader.type_name, "SkyShaderProperty");
    assert!(!sky_shader.lit);
    assert!(sky_shader.layout_ok);
    assert!(scene.meshes[2].shader.as_ref().unwrap().layout_ok);
}

#[test]
fn flags_shader_blocks_of_unexpected_size() {
    let mut b = Builder::new();
    b.reserve(4);
    let root = b.node("Root", 0, &NO_XF, &[], &[1]);
    b.set(0, "NiNode", root);
    let shape = b.shape("Odd", 0, &NO_XF, &[2], 3);
    b.set(1, "NiTriShape", shape);
    let mut unlit = b.unlit_shader("textures\\x.dds");
    unlit.extend([0; 4]); // four bytes more than this shader type has
    b.set(2, "BSShaderNoLightingProperty", unlit);
    let data = b.shape_data(&SQUARE, &[], &[], &[[0, 1, 2]]);
    b.set(3, "NiTriShapeData", data);
    let scene = Nif::parse(b.build()).unwrap().scene().unwrap();
    let shader = scene.meshes[0].shader.as_ref().unwrap();
    assert!(!shader.layout_ok);
    // Fields past a misread layout aren't trusted.
    assert!(shader.falloff.is_none());
}

/// A light block: the scene object, switched on, no affected nodes, the
/// dimmer and colours, then (point lights) the attenuation.
fn light_block(b: &mut Builder, name: &str, xf: &Xf, point: bool) -> Vec<u8> {
    let mut v = b.av(name, 0x0008_000E, xf, &[]);
    v.push(1); // switched on
    v.extend(0u32.to_le_bytes()); // affected nodes
    f32s(&mut v, &[1.4]); // dimmer
    f32s(&mut v, &[0.0, 0.03, 0.05]); // ambient
    f32s(&mut v, &[0.72, 0.91, 0.74]); // diffuse
    f32s(&mut v, &[0.72, 0.91, 0.74]); // specular
    if point {
        f32s(&mut v, &[1.0, 0.0, 0.0]);
    }
    v
}

#[test]
fn reads_a_models_lights() {
    // As `LockInterface01.NIF` holds them: a point light under a moved
    // node, and an ambient light under the top node.
    let mut b = Builder::new();
    b.reserve(4);
    let root_xf = Xf {
        translation: [0.0, 0.0, 10.0],
        ..NO_XF
    };
    let root = b.node("Root", 0x0008_000E, &root_xf, &[], &[1, 3]);
    b.set(0, "BSFadeNode", root);
    let omni_xf = Xf {
        translation: [-15.0, 18.0, 27.0],
        ..NO_XF
    };
    let omni = b.node("Omni01", 0x0008_000E, &omni_xf, &[], &[2]);
    b.set(1, "NiNode", omni);
    let point = light_block(&mut b, "Omni01Light", &NO_XF, true);
    b.set(2, "NiPointLight", point);
    let ambient = light_block(&mut b, "Ambient", &NO_XF, false);
    b.set(3, "NiAmbientLight", ambient);
    let nif = Nif::parse(b.build()).unwrap();
    let lights = nif.lights().unwrap();
    assert_eq!(lights.len(), 2);
    let p = &lights[0];
    assert_eq!(p.kind, nif::LightKind::Point);
    assert!(
        close(p.position(), [-15.0, 18.0, 37.0]),
        "{:?}",
        p.position()
    );
    assert!(p.on);
    assert_eq!(p.dimmer, 1.4);
    assert_eq!(p.diffuse, [0.72, 0.91, 0.74]);
    assert_eq!(p.attenuation, Some([1.0, 0.0, 0.0]));
    let a = &lights[1];
    assert_eq!(a.kind, nif::LightKind::Ambient);
    assert_eq!(a.ambient, [0.0, 0.03, 0.05]);
    assert_eq!(a.attenuation, None);
    // Lights aren't drawn meshes.
    assert!(nif.scene().unwrap().meshes.is_empty());
}

#[test]
fn finds_a_placed_node_by_name() {
    // Root (moved, left out when placed) → "Barrel" (up 2) → a hidden
    // "ProjectileNode" (forward 3).
    let mut b = Builder::new();
    b.reserve(3);
    let moved = Xf {
        translation: [5.0, 0.0, 0.0],
        ..NO_XF
    };
    let root = b.node("Root", 0, &moved, &[], &[1]);
    b.set(0, "NiNode", root);
    let up = Xf {
        translation: [0.0, 0.0, 2.0],
        ..NO_XF
    };
    let barrel = b.node("Barrel", 0, &up, &[], &[2]);
    b.set(1, "NiNode", barrel);
    let ahead = Xf {
        translation: [0.0, 3.0, 0.0],
        ..NO_XF
    };
    let muzzle = b.node("ProjectileNode", 1, &ahead, &[], &[]);
    b.set(2, "NiNode", muzzle);
    let nif = Nif::parse(b.build()).unwrap();
    let found = nif.placed_node("projectilenode").unwrap();
    assert!(
        close(found.translation, [0.0, 3.0, 2.0]),
        "{:?}",
        found.translation
    );
    assert!(nif.placed_node("##ProjectileNode").is_none());
}

#[test]
fn a_models_own_controllers_play_as_one_sequence() {
    // The impact effects' way: a transform controller on the root node and
    // an alpha controller on the shape's material, outside any sequence.
    let nif = Nif::parse(testdata::impacts::controlled_effect_nif()).unwrap();
    assert!(nif.sequences().unwrap().is_empty());
    let s = nif.own_controllers().unwrap().expect("its controllers");
    assert_eq!((s.start, s.stop), (0.0, 0.3));
    assert!(!s.looping, "cycle type 2 clamps");
    let [grow] = &s.tracks[..] else {
        panic!("{:?}", s.tracks);
    };
    assert_eq!(grow.node, "Root");
    let scale = grow.sample(0.15).scale.unwrap();
    assert!((scale - 2.0).abs() < 1e-5, "{scale}");
    let [fade] = &s.materials[..] else {
        panic!("{:?}", s.materials);
    };
    assert_eq!(
        (fade.node.as_str(), fade.target),
        ("Quad", nif::MaterialTarget::Alpha)
    );
    assert!((fade.float_at(0.15).unwrap() - 0.5).abs() < 1e-5);
    // A model without controllers has none.
    let still = Nif::parse(testdata::impacts::animated_effect_nif(&[])).unwrap();
    assert!(still.own_controllers().unwrap().is_none());
}
