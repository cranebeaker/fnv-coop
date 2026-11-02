//! A small SpeedTree `.spt` file in the game's token format, and an
//! outdoor test world with a tree placed on its one square.

use crate::{dds, f32s, group, record, sub, zstr, TempData};

/// The seed the test tree's record lists.
pub const TREE_SEED: i32 = 4242;
/// The tree record's form ID, and the placed reference's.
pub const TREE_BASE: u32 = 0xE00;
pub const TREE_REF: u32 = 0xE10;
/// Where the tree stands.
pub const TREE_AT: [f32; 3] = [1000.0, 2000.0, 0.0];

/// Builds a file token by token.
#[derive(Default)]
pub struct Spt(Vec<u8>);

impl Spt {
    pub fn int(&mut self, v: i32) -> &mut Self {
        self.0.extend(v.to_le_bytes());
        self
    }
    pub fn float(&mut self, v: f32) -> &mut Self {
        self.0.extend(v.to_le_bytes());
        self
    }
    pub fn flag(&mut self, v: bool) -> &mut Self {
        self.0.push(v as u8);
        self
    }
    pub fn vec3(&mut self, v: [f32; 3]) -> &mut Self {
        self.float(v[0]).float(v[1]).float(v[2])
    }
    pub fn string(&mut self, s: &str) -> &mut Self {
        self.int(s.len() as i32);
        self.0.extend(s.as_bytes());
        self
    }
    pub fn bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

/// A curve from `y0` at 0 to `y1` at 1 (a straight Bézier), scaled to
/// `min..max`, with `variance`.
pub fn curve(min: f32, max: f32, variance: f32, y0: f32, y1: f32) -> String {
    format!(
        "BezierSpline {min} {max} {variance} {{ 2 0 {y0} 1 {d} 0.1 1 {y1} 1 {d} 0.1 }}",
        d = y1 - y0
    )
}

/// One branch level (1016..1017).
fn level(
    s: &mut Spt,
    length: (f32, f32),
    radius: f32,
    cross_sections: i32,
    segments: i32,
    frequency: f32,
) {
    s.int(1016);
    s.int(6000).string(&curve(0.0, 1.0, 0.0, 0.0, 0.0));
    s.int(6001).string(&curve(0.0, 1.0, 0.0, 0.1, 0.1));
    s.int(6002).string(&curve(0.0, 1.0, 0.0, 0.2, 0.2));
    s.int(6003).string(&curve(0.0, 1.0, 0.0, 1.0, 1.0));
    s.int(6004)
        .string(&curve(0.0, 1.0, 0.05, length.0, length.1));
    s.int(6005).string(&curve(0.0, 1.0, 0.0, radius, radius));
    s.int(6006).string(&curve(0.0, 1.0, 0.0, 1.0, 0.5));
    s.int(6007).string(&curve(0.0, 90.0, 5.0, 0.5, 0.5));
    s.int(6017).string(&curve(0.0, 1.0, 0.0, 0.5, 0.5));
    s.int(6008).int(cross_sections);
    s.int(6009).int(segments);
    s.int(6010).float(0.2);
    s.int(6011).float(1.0);
    s.int(6012).float(frequency);
    s.int(6013).float(2.0);
    s.int(6014).float(5.0);
    s.int(6015).flag(true);
    s.int(6016).flag(true);
    s.int(1017);
}

/// A shrub-like tree: a trunk, one level of branches, leaves on a third
/// level; two leaf textures (so two rocking groups); two branch and two
/// leaf levels of detail; seed [`TREE_SEED`], size 20 ± 2.
pub fn spt_file() -> Vec<u8> {
    let mut s = Spt::default();
    s.int(1000).string("__IdvSpt_02_");
    s.int(1002);
    s.int(2000).string("C:\\Art\\TestBark.tga");
    s.int(2001).float(1100.0);
    s.int(2003).float(100.0);
    s.int(2005).int(TREE_SEED);
    s.int(2006).float(20.0);
    s.int(2007).float(2.0);
    s.int(1014).int(3);
    level(&mut s, (0.6, 0.6), 0.05, 6, 4, 60.0);
    level(&mut s, (0.4, 0.3), 0.02, 4, 3, 40.0);
    level(&mut s, (0.1, 0.1), 0.01, 3, 1, 20.0);
    s.int(1015);
    s.int(1003);

    s.int(1004);
    s.int(3000).float(0.75);
    s.int(3001).int(1);
    s.int(3002).float(0.8);
    s.int(3007).float(0.3);
    s.int(3008).int(1);
    s.int(3009).flag(true);
    s.int(3010).float(0.1);
    s.int(1009).int(1006).int(2);
    for (file, origin) in [
        ("TestLeaf1.tga", [0.5, 0.25, 0.0]),
        ("TestLeaf2.tga", [0.45, 0.3, 0.0]),
    ] {
        s.int(1007);
        s.int(4000).flag(false);
        s.int(4001).vec3([1.0, 1.0, 1.0]);
        s.int(4002).float(0.1);
        s.int(4003).string(file);
        s.int(4004).vec3(origin);
        s.int(4005).vec3([0.1, 0.1, 0.0]);
        s.int(4006).vec3([5.0, 5.0, 0.0]);
        s.int(1008);
    }
    s.int(1010);
    s.int(1005);
    s.int(1011).int(5004).vec3([0.01, 1.0, 0.0]).int(1012);
    s.int(1001);

    s.int(9000);
    s.int(9002).int(1);
    s.int(9003).float(0.1);
    s.int(9004).float(1.5);
    s.int(9009).float(0.6);
    s.int(9005);
    s.int(9007).int(2);
    s.int(9008).float(0.0);
    s.int(9010).float(0.4);
    s.int(9011).int(2);
    s.int(9012).float(0.3);
    s.int(9013).float(0.0);
    s.int(9014).float(0.05);
    s.int(9006);
    s.int(9001);
    // The leaves' places in the leaf texture: the first map's left half,
    // the second's right half.
    s.int(10000).int(10002).int(2);
    for e in [
        [0.5, 1.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0],
        [1.0, 1.0, 0.5, 1.0, 0.5, 0.0, 1.0, 0.0],
    ] {
        for v in e {
            s.float(v);
        }
    }
    s.int(10001);
    s.int(11000).int(11002).int(0).int(11001);
    s.int(16013).int(444);
    s.int(16014).float(0.25);
    s.int(21000).float(1.0);
    s.int(21001).float(0.07);
    s.int(22000).flag(true);
    s.bytes()
}

/// An outdoor world (`TreeWorld`, square 0,0) with the test tree placed at
/// [`TREE_AT`] (`XSED` 0), its `.spt` under `trees\`, its bark, bark normal
/// map and leaf texture.
pub fn trees(tag: &str) -> TempData {
    let data = TempData::new(tag);
    data.write("trees/testtree.spt", &spt_file());
    data.write("textures/trees/branches/testbark.dds", &dds([120, 90, 60]));
    data.write(
        "textures/trees/branches/testbark_n.dds",
        &dds([128, 128, 255]),
    );
    data.write("textures/trees/leaves/testleaves.dds", &dds([60, 120, 40]));

    let mut tree = sub(b"EDID", &zstr("TestTree"));
    tree.extend(sub(
        b"OBND",
        &[0xF0, 0xFF, 0xF0, 0xFF, 0, 0, 16, 0, 16, 0, 64, 0],
    ));
    tree.extend(sub(b"MODL", &zstr("\\TestTree.spt")));
    tree.extend(sub(b"ICON", &zstr("TestLeaves.dds")));
    tree.extend(sub(b"SNAM", &(TREE_SEED as u32).to_le_bytes()));
    let mut cnam = f32s(&[2.0, 5.0, 85.0, 0.5, 0.05]);
    cnam.extend(64i32.to_le_bytes());
    cnam.extend(f32s(&[0.07, 0.35]));
    tree.extend(sub(b"CNAM", &cnam));
    tree.extend(sub(b"BNAM", &f32s(&[175.0, 175.0])));
    let trees = record(b"TREE", TREE_BASE, &tree);

    let mut reference = sub(b"NAME", &TREE_BASE.to_le_bytes());
    reference.extend(sub(b"XSED", &[0]));
    reference.extend(sub(b"XSCL", &1.5f32.to_le_bytes()));
    reference.extend(sub(
        b"DATA",
        &f32s(&[TREE_AT[0], TREE_AT[1], TREE_AT[2], 0.0, 0.0, 0.0]),
    ));
    let placed = record(b"REFR", TREE_REF, &reference);

    let mut cell = sub(b"DATA", &[0]);
    cell.extend(sub(b"XCLC", &[0u8; 12]));
    let mut squares = record(b"CELL", 0xE20, &cell);
    squares.extend(group(
        0xE20u32.to_le_bytes(),
        6,
        &group(0xE20u32.to_le_bytes(), 9, &placed),
    ));
    let mut world = sub(b"EDID", &zstr("TreeWorld"));
    world.extend(sub(b"DATA", &[0]));
    let mut worlds = record(b"WRLD", 0xE30, &world);
    worlds.extend(group(
        0xE30u32.to_le_bytes(),
        1,
        &group([0; 4], 4, &group([0; 4], 5, &squares)),
    ));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"TREE", 0, &trees));
    plugin.extend(group(*b"WRLD", 0, &worlds));
    data.write("FalloutNV.esm", &plugin);
    data
}
