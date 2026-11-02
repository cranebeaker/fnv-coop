//! A small worldspace with water, for the water tests: `TestLake` (0xE10)
//! whose default water type is `TestWater` (0xE00) at height 500, with
//! three squares:
//!
//! - 0,0 (`LakeShore`, 0xE20): flat ground at 900 under its own water at
//!   1000 of type `TestPondWater` (0xE01), and a placed pool (`TestPool`,
//!   0xE50; the reference 0xE21) at 2000, 2000, 1100, scale 0.5, turned a
//!   quarter clockwise;
//! - 1,0 (0xE30): water at the worldspace's default height (its `XCLW` is
//!   the largest float), below its ground at 1000;
//! - 2,0 (0xE40): no water (its `DATA` lacks the flag, though it has an
//!   `XCLW`);
//! - 3,0 ([`SEA`]): water at the distant-water height, 777.
//!
//! And an interior, `TestBath` (0xE60), flagged as having water (interiors
//! never get a cell's water) with the pool placed in it (0xE61). The
//! worldspace's distant water is `TestPondWater` at 777 (`NAM3`, `NAM4`),
//! and its distant-land chunk at 0,0 carries distant water
//! ([`lod_water_nif`]).

use crate::{
    dds, f32s, group, land, placed, quad, record, sized, sub, zstr, Geometry, NifBuilder, TempData,
};

pub const WATER: u32 = 0xE00;
pub const POND_WATER: u32 = 0xE01;
pub const WORLD: u32 = 0xE10;
pub const SHORE: u32 = 0xE20;
pub const POOL_REF: u32 = 0xE21;
pub const DRY_LAND: u32 = 0xE30;
pub const NO_WATER: u32 = 0xE40;
pub const POOL: u32 = 0xE50;
pub const BATH: u32 = 0xE60;
pub const BATH_POOL_REF: u32 = 0xE61;
/// Square 3,0: water at the worldspace's distant-water height ("sea
/// level"), above its ground.
pub const SEA: u32 = 0xE70;

/// The worldspace's default water height.
pub const DEFAULT_HEIGHT: f32 = 500.0;
/// Square 0,0's own water height, and its ground.
pub const SHORE_HEIGHT: f32 = 1000.0;
pub const SHORE_GROUND: f32 = 900.0;
/// The pool: where it's placed, its scale, its flags (refracts and depth,
/// no reflection, as Goodsprings' pools) and its model's half-size.
pub const POOL_AT: [f32; 3] = [2000.0, 2000.0, 1100.0];
pub const POOL_SCALE: f32 = 0.5;
pub const POOL_FLAGS: u32 = 0x1000_0E02;
pub const POOL_HALF_SIZE: f32 = 256.0;
/// `TestWater`'s colours (shallow, deep, reflection) and opacity.
pub const SHALLOW: [u8; 3] = [30, 90, 120];
pub const DEEP: [u8; 3] = [10, 30, 40];
pub const REFLECTION: [u8; 3] = [100, 110, 120];
pub const OPACITY: u8 = 60;
/// `TestPondWater`'s shallow colour.
pub const POND_SHALLOW: [u8; 3] = [200, 50, 50];
/// The noise textures: the worldspace's and the type's (as written).
pub const WORLD_NOISE: &str = "Data\\Textures\\Water\\WorldNoise.dds";
pub const TYPE_NOISE: &str = "Data\\Textures\\Water\\TestNoise.dds";

/// A water type's 196-byte `DNAM`: sun power 100, reflectivity 0.5,
/// fresnel 0.25, fog 10 to 500 (amount 0.8), the colours, noise strength
/// 10, winds toward 0°, 90°, 180° at 0.1, 0.2, 0.3, depth falloff 0 to
/// 0.05, noise tile 1000 units, distortion 400, reflection multiplier 20
/// (2 in the shader), amplitudes 0.3, 0.5, 0.2.
pub fn visual(shallow: [u8; 3]) -> Vec<u8> {
    let mut d = vec![0u8; 196];
    let mut put = |at: usize, v: f32| d[at..at + 4].copy_from_slice(&v.to_le_bytes());
    for (at, v) in [
        (16, 100.0),
        (20, 0.5),
        (24, 0.25),
        (32, 10.0),
        (36, 500.0),
        (96, 10.0),
        (100, 0.0),
        (104, 90.0),
        (108, 180.0),
        (112, 0.1),
        (116, 0.2),
        (120, 0.3),
        (124, 0.0),
        (128, 0.05),
        (132, 0.8),
        (136, 1000.0),
        (152, 400.0),
        (160, 20.0),
        (184, 0.3),
        (188, 0.5),
        (192, 0.2),
    ] {
        put(at, v);
    }
    d[40..43].copy_from_slice(&shallow);
    d[44..47].copy_from_slice(&DEEP);
    d[48..51].copy_from_slice(&REFLECTION);
    d
}

/// A flat model carrying a `WaterShaderProperty` (as the game's
/// `Water_Placeable` models do), `half` units each way around its middle.
pub fn water_nif(half: f32) -> Vec<u8> {
    shapes_nif(&[(square(half), true, [0.0; 3])])
}

/// A flat square `half` units each way around the model's middle.
fn square(half: f32) -> Geometry {
    quad(
        [
            [-half, -half, 0.0],
            [half, -half, 0.0],
            [half, half, 0.0],
            [-half, half, 0.0],
        ],
        [0.0, 0.0, 1.0],
    )
}

/// A model of shapes under a root node: each its geometry, whether it
/// carries a `WaterShaderProperty` (else no property at all), and where
/// its own node moves it.
fn shapes_nif(shapes: &[(Geometry, bool, [f32; 3])]) -> Vec<u8> {
    let mut b = NifBuilder::default();
    let mut root = b.av("Root", &[]);
    root.extend((shapes.len() as u32).to_le_bytes());
    let mut blocks: Vec<(String, Vec<u8>)> = Vec::new();
    // Block numbers: the root, then per shape its node, its property (if
    // any) and its data.
    let mut next = 1i32;
    let mut children = Vec::new();
    for (g, water, at) in shapes {
        children.push(next);
        let props: Vec<i32> = if *water { vec![next + 1] } else { Vec::new() };
        let data_block = next + 1 + props.len() as i32;
        let mut shape = b.av("Water", &props);
        // The node's own translation (`av` writes none for non-root nodes).
        shape[16..28].copy_from_slice(&f32s(at));
        shape.extend(data_block.to_le_bytes());
        shape.extend((-1i32).to_le_bytes());
        shape.extend(0u32.to_le_bytes());
        shape.extend((-1i32).to_le_bytes());
        shape.push(0);
        blocks.push(("NiTriShape".into(), shape));
        if *water {
            // The shader's own fields aren't read; only its type matters.
            let mut shader = b.net("");
            shader.extend(1u16.to_le_bytes());
            for v in [17u32, 0, 0] {
                shader.extend(v.to_le_bytes());
            }
            shader.extend(1.0f32.to_le_bytes());
            shader.extend(sized(""));
            blocks.push(("WaterShaderProperty".into(), shader));
        }
        blocks.push(("NiTriShapeData".into(), shape_data(g)));
        next = data_block + 1;
    }
    for c in &children {
        root.extend(c.to_le_bytes());
    }
    root.extend(0u32.to_le_bytes());
    b.blocks = std::iter::once(("BSFadeNode".to_string(), root))
        .chain(blocks)
        .collect();
    b.build()
}

/// A shape's `NiTriShapeData`.
fn shape_data(g: &Geometry) -> Vec<u8> {
    let (positions, normals, uvs, triangles) = (&g.0, &g.1, &g.2, &g.3);
    let half = positions
        .iter()
        .flatten()
        .fold(1.0f32, |m, v| m.max(v.abs()));
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
    data.extend(f32s(&[0.0, 0.0, 0.0, half * 1.5]));
    data.push(0);
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
    data
}

/// Distant land for the lake world's chunk at 0,0, as the game ships it:
/// the land (here a shape with a property) and the distant water, a shape
/// with no property moved by its own node to the chunk's corner (here
/// 4096 east, 0 north), its vertices at the water's height. Its one strip
/// crosses from square 1,0 into 2,0.
pub fn lod_water_nif() -> Vec<u8> {
    let strip = quad(
        [
            [2048.0, 1000.0, LOD_WATER_HEIGHT],
            [6144.0, 1000.0, LOD_WATER_HEIGHT],
            [6144.0, 1200.0, LOD_WATER_HEIGHT],
            [2048.0, 1200.0, LOD_WATER_HEIGHT],
        ],
        [0.0, 0.0, 1.0],
    );
    shapes_nif(&[
        (square(100.0), true, [0.0; 3]),
        (strip, false, [4096.0, 0.0, 0.0]),
    ])
}

/// The distant water's height in the test chunk.
pub const LOD_WATER_HEIGHT: f32 = 777.0;

/// The lake world (see the module notes).
pub fn lake(tag: &str) -> TempData {
    let data = TempData::new(tag);
    data.write("meshes/water/pool.nif", &water_nif(POOL_HALF_SIZE));
    data.write(
        "meshes/landscape/lod/testlake/testlake.level4.x0.y0.nif",
        &lod_water_nif(),
    );
    data.write("textures/water/worldnoise.dds", &dds([128, 128, 255]));
    data.write("textures/water/testnoise.dds", &dds([128, 128, 128]));
    data.write("textures/test/ground.dds", &dds([120, 100, 80]));

    let water_type = |id: u32, name: &str, noise: Option<&str>, shallow: [u8; 3]| {
        let mut d = sub(b"EDID", &zstr(name));
        if let Some(n) = noise {
            d.extend(sub(b"NNAM", &zstr(n)));
        }
        d.extend(sub(b"ANAM", &[OPACITY]));
        d.extend(sub(b"FNAM", &[0]));
        d.extend(sub(b"DATA", &[0, 0]));
        d.extend(sub(b"DNAM", &visual(shallow)));
        record(b"WATR", id, &d)
    };
    let mut waters = water_type(WATER, "TestWater", Some(TYPE_NOISE), SHALLOW);
    waters.extend(water_type(POND_WATER, "TestPondWater", None, POND_SHALLOW));

    let mut pool = sub(b"EDID", &zstr("TestPool"));
    pool.extend(sub(b"MODL", &zstr("Water\\Pool.nif")));
    let mut dnam = POOL_FLAGS.to_le_bytes().to_vec();
    dnam.extend(POND_WATER.to_le_bytes());
    pool.extend(sub(b"DNAM", &dnam));
    let pools = record(b"PWAT", POOL, &pool);
    let pool_ref = |id: u32, at: [f32; 3], turn: f32| {
        placed(
            id,
            POOL,
            at,
            [0.0, 0.0, turn],
            &sub(b"XSCL", &POOL_SCALE.to_le_bytes()),
        )
    };

    let cell = |name: Option<&str>, square: (i32, i32), flags: u8, xclw: Option<f32>| {
        let mut d = name.map(|e| sub(b"EDID", &zstr(e))).unwrap_or_default();
        d.extend(sub(b"DATA", &[flags]));
        let mut xclc = square.0.to_le_bytes().to_vec();
        xclc.extend(square.1.to_le_bytes());
        xclc.extend([0u8; 4]);
        d.extend(sub(b"XCLC", &xclc));
        if let Some(h) = xclw {
            d.extend(sub(b"XCLW", &h.to_le_bytes()));
        }
        d
    };
    let children = |cell: u32, contents: &[u8]| {
        group(
            cell.to_le_bytes(),
            6,
            &group(cell.to_le_bytes(), 9, contents),
        )
    };
    let flat = |id: u32, height: f32| {
        land(
            id,
            height / 8.0,
            &[0i8; 33 * 33],
            &vec![[255, 255, 255]; 33 * 33],
            &[],
        )
    };

    let mut shore = cell(Some("LakeShore"), (0, 0), 0x02, Some(SHORE_HEIGHT));
    shore.extend(sub(b"XCWT", &POND_WATER.to_le_bytes()));
    let mut squares = record(b"CELL", SHORE, &shore);
    let mut shore_children = flat(0xE22, SHORE_GROUND);
    shore_children.extend(pool_ref(POOL_REF, POOL_AT, 90.0));
    squares.extend(children(SHORE, &shore_children));
    squares.extend(record(
        b"CELL",
        DRY_LAND,
        &cell(None, (1, 0), 0x02, Some(f32::MAX)),
    ));
    squares.extend(children(DRY_LAND, &flat(0xE31, 1000.0)));
    squares.extend(record(
        b"CELL",
        NO_WATER,
        &cell(None, (2, 0), 0x00, Some(2000.0)),
    ));
    squares.extend(children(NO_WATER, &flat(0xE41, 0.0)));
    squares.extend(record(
        b"CELL",
        SEA,
        &cell(None, (3, 0), 0x02, Some(LOD_WATER_HEIGHT)),
    ));
    squares.extend(children(SEA, &flat(0xE71, LOD_WATER_HEIGHT - 100.0)));
    let world_children = group([0; 4], 4, &group([0; 4], 5, &squares));

    let mut world = sub(b"EDID", &zstr("TestLake"));
    world.extend(sub(b"FULL", &zstr("Test Lake")));
    world.extend(sub(b"NAM2", &WATER.to_le_bytes()));
    world.extend(sub(b"NAM3", &POND_WATER.to_le_bytes()));
    world.extend(sub(b"NAM4", &LOD_WATER_HEIGHT.to_le_bytes()));
    world.extend(sub(b"DNAM", &f32s(&[-2500.0, DEFAULT_HEIGHT])));
    world.extend(sub(b"XNAM", &zstr(WORLD_NOISE)));
    world.extend(sub(b"DATA", &[0]));
    let mut worlds = record(b"WRLD", WORLD, &world);
    worlds.extend(group(WORLD.to_le_bytes(), 1, &world_children));

    let mut bath = sub(b"EDID", &zstr("TestBath"));
    bath.extend(sub(b"DATA", &[0x03]));
    bath.extend(sub(b"XCLW", &50f32.to_le_bytes()));
    let mut interiors = record(b"CELL", BATH, &bath);
    interiors.extend(group(
        BATH.to_le_bytes(),
        6,
        &group(
            BATH.to_le_bytes(),
            9,
            &pool_ref(BATH_POOL_REF, [0.0, 0.0, 10.0], 0.0),
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
    plugin.extend(group(*b"WATR", 0, &waters));
    plugin.extend(group(*b"PWAT", 0, &pools));
    plugin.extend(interiors);
    plugin.extend(group(*b"WRLD", 0, &worlds));
    data.write("FalloutNV.esm", &plugin);
    data
}
