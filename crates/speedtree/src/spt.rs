//! What a `.spt` file holds, read token by token as the game's
//! SpeedTreeRT reads it (`CSpeedTreeRT::LoadTree`, `00b059e0`), with the
//! defaults its constructors give whatever a file leaves out.
//!
//! The file opens with token 1000 and the string `__IdvSpt_02_`, then
//! blocks until 1001 (`00b2b550`): 1002 the tree (`00b2bde0`), 1004 the
//! leaves (`00b3d1b0`), 1011 the leaves' wind (`00b408b0`); then
//! optionally 7000, the leaf levels of detail (`00b2cbc0`). After that,
//! until the file ends (`00b059e0`): 8000 lighting (`00b2a940`), 9000
//! levels of detail (`00b092a0`), 10000 billboards (`00b09960`), 11000
//! the seed level (`00b097e0`), 12000 collision objects (`00b0a490`),
//! 13000 fronds (`00b18340`), 15000 and 16000 more per-level branch
//! values (`00b2d090`, `00b2d2e0`), 16013, 16014, 18000 (`00b3e750`),
//! 19000 (`00b0d500`), 20000 billboard texture coordinates (`00b0d810`),
//! 21000, 21001, 22000. An unknown token ends the reading there.

use crate::reader::{Reader, Result, SptError};
use crate::spline::Spline;

/// The magic string after token 1000.
pub const MAGIC: &str = "__IdvSpt_02_";

/// One level of branches (`00b3f8e0`, a 0x74-byte object whose defaults
/// are set by `00b3f310`). Level 0 is the trunk; the last level holds the
/// leaves; levels from the frond level on are drawn as fronds.
#[derive(Debug, Clone, PartialEq)]
pub struct BranchLevel {
    /// 6008, +0x00 (6): points around the branch.
    pub cross_sections: i32,
    /// 6009, +0x04 (3): segments along the branch.
    pub segments: i32,
    /// 6010, +0x08 (0.3): where along the parent children start.
    pub first_branch: f32,
    /// 6011, +0x0c (1.0): where they end.
    pub last_branch: f32,
    /// 6012, +0x10 (0.3): children per tree size of the parent's length.
    pub frequency: f32,
    /// 6013, +0x14 (1.0): texture repeats around.
    pub u_tile: f32,
    /// 6014, +0x18 (1.0): texture repeats along.
    pub v_tile: f32,
    /// 6015, +0x1c (true): `u_tile` absolute (else × radius × π).
    pub u_tile_absolute: bool,
    /// 6016, +0x1d (false): `v_tile` absolute (else × length ÷ size).
    pub v_tile_absolute: bool,
    /// 15002, +0x1e (false): the texture turns with the branch's twist.
    pub twist_texture: bool,
    /// 15003, +0x20 (0): a texture offset alternating between children.
    pub u_offset: f32,
    /// 16002, +0x24 (1): segments spaced by `(i/n)^this`.
    pub segment_exponent: f32,
    /// 16003, +0x28 (0): flares.
    pub flares: i32,
    /// 16004–16012, +0x2c–+0x4c (1, 30, 10, 1, 0.5, 0.25, 0.3, 0.1, 1).
    pub flare_values: [f32; 9],
    /// 6000, +0x50: disturbance.
    pub disturbance: Spline,
    /// 6001, +0x54: gravity.
    pub gravity: Spline,
    /// 6002, +0x58: flexibility.
    pub flexibility: Spline,
    /// 6003, +0x5c: flexibility along the branch.
    pub flexibility_scale: Spline,
    /// 6004, +0x60: length (× tree size).
    pub length: Spline,
    /// 6005, +0x64: radius (× tree size).
    pub radius: Spline,
    /// 6006, +0x68: radius along the branch.
    pub radius_scale: Spline,
    /// 6007, +0x6c: start angle.
    pub start_angle: Spline,
    /// 6017, +0x70: gravity along the branch.
    pub gravity_scale: Spline,
}

impl Default for BranchLevel {
    fn default() -> Self {
        let empty = Spline::parse("");
        BranchLevel {
            cross_sections: 6,
            segments: 3,
            first_branch: 0.3,
            last_branch: 1.0,
            frequency: 0.3,
            u_tile: 1.0,
            v_tile: 1.0,
            u_tile_absolute: true,
            v_tile_absolute: false,
            twist_texture: false,
            u_offset: 0.0,
            segment_exponent: 1.0,
            flares: 0,
            flare_values: [1.0, 30.0, 10.0, 1.0, 0.5, 0.25, 0.3, 0.1, 1.0],
            disturbance: empty.clone(),
            gravity: empty.clone(),
            flexibility: empty.clone(),
            flexibility_scale: empty.clone(),
            length: empty.clone(),
            radius: empty.clone(),
            radius_scale: empty.clone(),
            start_angle: empty.clone(),
            gravity_scale: empty,
        }
    }
}

/// One leaf texture (1007..1008, 0x54 bytes, defaults `00b3d720`).
#[derive(Debug, Clone, PartialEq)]
pub struct LeafTexture {
    /// 4000, +0x00 (false): blossom leaves (placed by the blossom rule).
    pub blossom: bool,
    /// 4001, +0x04 (0.8, 0.8, 0.8): the leaves' colour.
    pub color: [f32; 3],
    /// 4002, +0x10 (0.2): ± that much added to the colour.
    pub color_variance: f32,
    /// 4003, +0x14: the file name (the path's last part).
    pub file: String,
    /// 4004, +0x30 (0.5, 1, 0): the card's pivot as a share of its size.
    pub origin: [f32; 3],
    /// 4005, +0x3c (0.12, 0.12, 0): the card's size as a share of the
    /// tree size.
    pub size: [f32; 3],
    /// 4006, +0x48 (10, 10, 0): the card's size; replaced by
    /// `size × tree size` when the tree is grown with a size.
    pub absolute_size: [f32; 3],
}

impl Default for LeafTexture {
    fn default() -> Self {
        LeafTexture {
            blossom: false,
            color: [0.8, 0.8, 0.8],
            color_variance: 0.2,
            file: String::new(),
            origin: [0.5, 1.0, 0.0],
            size: [0.12, 0.12, 0.0],
            absolute_size: [10.0, 10.0, 0.0],
        }
    }
}

/// The leaves' settings (1004, engine +0x94, defaults `00b3cd60`).
#[derive(Debug, Clone, PartialEq)]
pub struct LeafInfo {
    /// 3009, +0x00 (true): leaves darker deeper inside the tree.
    pub dimming: bool,
    /// 3010, +0x04 (1.0): how much (the game's TREE record replaces it:
    /// `CNAM` leaf dimming).
    pub dimming_amount: f32,
    /// +0x08: branch dimming (set only by the game, `CNAM`).
    pub branch_dimming: f32,
    /// 3008, +0x0c (2): leaf collision: 1 against every leaf grown so far,
    /// 2 against this tree's.
    pub collision: i32,
    /// 1009: the leaf textures.
    pub textures: Vec<LeafTexture>,
    /// 3007, +0x28 (0.5): collision box half size per card size.
    pub collision_scale: f32,
    /// 3000, +0x2c (0.75): blossom: from this far along.
    pub blossom_distance: f32,
    /// 3002, +0x30 (0.8): blossom: kept when a random 0..1 is at least this.
    pub blossom_level: f32,
    /// 3001, +0x34 (1): ancestors up for the leaf's normal (and blossoms).
    pub normal_depth: i32,
    /// +0x38, +0x3c: the leaves' least and most bud angle (set only by
    /// the game, `CNAM`).
    pub bud_angles: [f32; 2],
    /// +0x40 (3): rocking groups (the game sets 1 or 2).
    pub rocking_groups: i32,
    /// +0x44 (4): leaf levels of detail.
    pub lods: i32,
}

impl Default for LeafInfo {
    fn default() -> Self {
        LeafInfo {
            dimming: true,
            dimming_amount: 1.0,
            branch_dimming: 0.0,
            collision: 2,
            textures: Vec::new(),
            collision_scale: 0.5,
            blossom_distance: 0.75,
            blossom_level: 0.8,
            normal_depth: 1,
            bud_angles: [0.0, 0.0],
            rocking_groups: 3,
            lods: 4,
        }
    }
}

/// The leaves' rocking and rustling (1011, engine +0x10c, defaults
/// `00b40850`).
#[derive(Debug, Clone, PartialEq)]
pub struct LeafWind {
    /// 5004, +0x00 (0.5, 0.5, 0).
    pub angles: [f32; 3],
    /// 5002, +0x0c.
    pub speeds: [f32; 3],
    /// 5005, +0x18.
    pub value: f32,
}

impl Default for LeafWind {
    fn default() -> Self {
        LeafWind {
            angles: [0.5, 0.5, 0.0],
            speeds: [0.0; 3],
            value: 0.0,
        }
    }
}

/// A leaf the file lists for one level of detail (7004..7005, `00b405c0`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StoredLeaf {
    /// 7015.
    pub position: [f32; 3],
    /// 7007.
    pub rock_group: u8,
    /// 7006 (then 32 bits skipped).
    pub color: u32,
    /// 7008.
    pub dimming: u8,
    /// 7010.
    pub normal: [f32; 3],
    /// 7011.
    pub texture: u8,
    /// 7012 ÷ 255.
    pub wind_weight: f32,
    /// 7013.
    pub wind_group: i32,
}

/// Fronds (13000, `00b18340`, frond engine defaults `00b17fd0`).
#[derive(Debug, Clone, PartialEq)]
pub struct FrondInfo {
    /// 13002, +0x48 (1): branch levels from this one on are fronds.
    pub level: i32,
    /// 13003, +0x38 (1).
    pub kind: i32,
    /// 13004, +0x3c (2).
    pub blades: i32,
    /// 13005, +0x40: the frond's profile.
    pub profile: Spline,
    /// 13006, +0x44 (4).
    pub profile_segments: i32,
    /// 13007, +0x4c (false): fronds drawn.
    pub enabled: bool,
    /// 13008: textures, each with four values (14003–14006: 0.5, 1, 0, 0).
    pub textures: Vec<FrondTexture>,
    /// 13009, +0x68 (4).
    pub lods: i32,
    /// 13010–13013, +0x6c–+0x78 (1, 0, 0, 0.05).
    pub values: [f32; 4],
    /// 14007, +0x7c (2).
    pub value_7c: i32,
    /// 14008, +0x80 (1).
    pub value_80: i32,
}

impl Default for FrondInfo {
    fn default() -> Self {
        FrondInfo {
            level: 1,
            kind: 1,
            blades: 2,
            profile: Spline::parse(
                "BezierSpline 0.0 1.0 0.0 { 3 0 0.00138887 0.337009 0.941501 0.132767 0.493215 0.998903 1 0.00102074 0.23702 1 -6.24607e-008 0.307222 -0.951638 0.126974 }",
            ),
            profile_segments: 4,
            enabled: false,
            textures: Vec::new(),
            lods: 4,
            values: [1.0, 0.0, 0.0, 0.05],
            value_7c: 2,
            value_80: 1,
        }
    }
}

/// One frond texture (14000..14001, 0x2c bytes).
#[derive(Debug, Clone, PartialEq)]
pub struct FrondTexture {
    /// 14002, the path's last part.
    pub file: String,
    /// 14003–14006 (0.5, 1, 0, 0).
    pub values: [f32; 4],
}

/// Levels of detail (9000, `00b092a0` and `00b2ceb0`).
#[derive(Debug, Clone, PartialEq)]
pub struct LodInfo {
    /// 9002, CSpeedTreeRT +0x18 (1).
    pub mode: i32,
    /// 9003, +0x1c (0.07).
    pub value_1c: f32,
    /// 9004, +0x20 (0.7).
    pub value_20: f32,
    /// 9009, +0x24 (0.5).
    pub value_24: f32,
    /// 9007, engine +0x78 (6): branch levels of detail.
    pub branch_lods: i32,
    /// 9008, engine +0xf4 (0.5): the smallest level's share of branches.
    pub branch_min: f32,
    /// 9012, engine +0xf8 (1.0): the largest level's share.
    pub branch_max: f32,
    /// 9010, engine +0xfc (0.3, 0 means 0.1).
    pub value_fc: f32,
    /// 9013, engine +0x100 (0): randomness in which branches go.
    pub branch_random: f32,
    /// 9014, engine +0x104 (0.05).
    pub branch_keep: f32,
    /// 9011, engine +0xd8: leaf levels of detail.
    pub leaf_lods: Option<i32>,
}

impl Default for LodInfo {
    fn default() -> Self {
        LodInfo {
            mode: 1,
            value_1c: 0.07,
            value_20: 0.7,
            value_24: 0.5,
            branch_lods: 6,
            branch_min: 0.5,
            branch_max: 1.0,
            value_fc: 0.3,
            branch_random: 0.0,
            branch_keep: 0.05,
            leaf_lods: None,
        }
    }
}

/// Lighting (8000, `00b2a940`, defaults `00b29fd0`): branch, leaf and
/// frond lighting methods and materials.
#[derive(Debug, Clone, PartialEq)]
pub struct Lighting {
    /// 8002, 8004, 8008 (+0x00, +0x38, +0x78): the methods.
    pub methods: [i32; 3],
    /// 8003, 8005, 8009: thirteen values each (materials).
    pub materials: [[f32; 13]; 3],
    /// 8006, +0x70 (0.5).
    pub value_70: f32,
    /// 8007, +0x74 (0).
    pub value_74: i32,
}

impl Default for Lighting {
    fn default() -> Self {
        let a = [
            0.8, 0.8, 0.8, 0.2, 0.2, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let b = [
            1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        Lighting {
            methods: [0, 0, 0],
            materials: [a, b, b],
            value_70: 0.5,
            value_74: 0,
        }
    }
}

/// A collision object (12000).
#[derive(Debug, Clone, PartialEq)]
pub struct CollisionObject {
    /// 12002 sphere (0), 12003 capsule (1), 12004 box (2).
    pub kind: i32,
    pub position: [f32; 3],
    pub dimensions: [f32; 3],
}

/// The whole file.
#[derive(Debug, Clone, PartialEq)]
pub struct SptFile {
    /// 2000, engine +0x24: the branch texture (the path's last part).
    pub branch_texture: String,
    /// 2001, engine +0x40 (1100): far level-of-detail distance.
    pub lod_far: f32,
    /// 2003, engine +0x44 (100): near distance.
    pub lod_near: f32,
    /// 2005, engine +0x48: the seed (0: random, 1: the default).
    pub seed: Option<i32>,
    /// 2006, engine +0x4c: the tree's size.
    pub size: f32,
    /// 2007, engine +0x50: ± that much.
    pub size_variance: f32,
    /// 1014: the branch levels.
    pub levels: Vec<BranchLevel>,
    /// 1004: the leaves.
    pub leaves: LeafInfo,
    /// 1011: the leaves' rocking.
    pub leaf_wind: LeafWind,
    /// 7000: leaves stored per level of detail (none in the game's files).
    pub stored_leaves: Option<Vec<Vec<StoredLeaf>>>,
    /// 8000.
    pub lighting: Lighting,
    /// 9000.
    pub lod: LodInfo,
    /// 10000: billboards (raw values).
    pub billboards: Option<Billboards>,
    /// 11002, engine +0x108 (1): the branch level whose branches each get
    /// the next seed.
    pub seed_level: i32,
    /// 12000.
    pub collision: Vec<CollisionObject>,
    /// 13000.
    pub fronds: FrondInfo,
    /// 16013, engine +0x54: the C library's `srand` value.
    pub srand: i32,
    /// 16014, CSpeedTreeRT +0x28.
    pub value_16014: Option<f32>,
    /// 18000: three vectors and a texture.
    pub block_18000: Option<Block18000>,
    /// 19002.
    pub texture_19002: Option<String>,
    /// 21000, 21001: the leaves' wind +0x3c, +0x40.
    pub wind_3c: Option<f32>,
    pub wind_40: Option<f32>,
    /// 22000 (`011f8c04`): every level's branches carry their own wind
    /// weight.
    pub wind_per_level: bool,
    /// Tokens after the end that weren't read (the game stops there).
    pub trailing: Option<i32>,
}

/// The 18000 block: three vectors and a texture.
pub type Block18000 = ([f32; 3], [f32; 3], [f32; 3], String);

/// Billboards (10000, `00b09960`) and their texture coordinates (20000).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Billboards {
    /// 10002: eight values per entry.
    pub a: Vec<[f32; 8]>,
    /// 10003.
    pub b: Vec<[f32; 8]>,
    /// 10004.
    pub c: Vec<[f32; 8]>,
    /// 10005 / 20002: a texture.
    pub texture: String,
    /// 10006 / 20003, 10007 / 20004.
    pub flags: [bool; 2],
    /// 20005: eight values.
    pub coords: Option<[f32; 8]>,
}

/// The path's last part (`00b09f90`).
fn file_name(path: &str) -> String {
    match path.rfind(['/', '\\']) {
        Some(i) => path[i + 1..].to_string(),
        None => path.to_string(),
    }
}

impl SptFile {
    /// Reads a file's bytes as `00b059e0` does.
    pub fn parse(bytes: &[u8]) -> Result<SptFile> {
        let mut r = Reader::new(bytes);
        let mut spt = SptFile {
            branch_texture: String::new(),
            lod_far: 1100.0,
            lod_near: 100.0,
            seed: None,
            size: 0.0,
            size_variance: 0.0,
            levels: Vec::new(),
            leaves: LeafInfo::default(),
            leaf_wind: LeafWind::default(),
            stored_leaves: None,
            lighting: Lighting::default(),
            lod: LodInfo::default(),
            billboards: None,
            seed_level: 1,
            collision: Vec::new(),
            fronds: FrondInfo::default(),
            srand: 0,
            value_16014: None,
            block_18000: None,
            texture_19002: None,
            wind_3c: None,
            wind_40: None,
            wind_per_level: false,
            trailing: None,
        };
        r.expect(1000, "missing begin_file token")?;
        if r.string()? != MAGIC {
            return r.error("not a valid SpeedTree SPT file");
        }
        loop {
            match r.int()? {
                1001 => break,
                1002 => spt.read_tree(&mut r)?,
                1004 => spt.read_leaves(&mut r)?,
                1011 => spt.read_leaf_wind(&mut r)?,
                t => return r.error(format!("malformed SpeedTree SPT file (token {t})")),
            }
        }
        if r.peek() == Some(7000) {
            r.int()?;
            spt.read_stored_leaves(&mut r)?;
        }
        while !r.at_end() {
            let t = r.int()?;
            match t {
                8000 => spt.read_lighting(&mut r)?,
                9000 => spt.read_lod(&mut r)?,
                10000 => spt.read_billboards(&mut r)?,
                11000 => spt.read_seed_level(&mut r)?,
                12000 => spt.read_collision(&mut r)?,
                13000 => spt.read_fronds(&mut r)?,
                15000 => spt.read_texture_controls(&mut r)?,
                16000 => spt.read_flares(&mut r)?,
                16013 => spt.srand = r.int()?,
                16014 => spt.value_16014 = Some(r.float()?),
                18000 => spt.read_18000(&mut r)?,
                19000 => spt.read_19000(&mut r)?,
                20000 => spt.read_billboard_coords(&mut r)?,
                21000 => spt.wind_3c = Some(r.float()?),
                21001 => spt.wind_40 = Some(r.float()?),
                22000 => spt.wind_per_level = r.flag()?,
                other => {
                    spt.trailing = Some(other);
                    break;
                }
            }
        }
        Ok(spt)
    }

    /// 1002 (`00b2bde0`), until 1003.
    fn read_tree(&mut self, r: &mut Reader) -> Result<()> {
        loop {
            match r.int()? {
                1003 => return Ok(()),
                2000 => self.branch_texture = file_name(&r.string()?),
                1014 => self.read_levels(r)?,
                2001 => self.lod_far = r.float()?,
                2002 => {
                    r.flag()?;
                }
                2003 => self.lod_near = r.float()?,
                2004 => {
                    r.int()?;
                }
                2005 => {
                    // `00b2c0c0`: 0 means a random seed, 1 leaves it.
                    let seed = r.int()?;
                    if seed != 1 {
                        self.seed = Some(seed);
                    }
                }
                2006 => self.size = r.float()?,
                2007 => self.size_variance = r.float()?,
                t => return r.error(format!("malformed general tree information (token {t})")),
            }
        }
    }

    /// 1014 (`00b2c790`): a count, the levels, 1015.
    fn read_levels(&mut self, r: &mut Reader) -> Result<()> {
        let count = r.int()?;
        self.levels.clear();
        for _ in 0..count.max(0) {
            self.levels.push(read_level(r)?);
        }
        r.expect(1015, "malformed branch data")
    }

    /// 1004 (`00b3d1b0`), until 1005.
    fn read_leaves(&mut self, r: &mut Reader) -> Result<()> {
        let leaf = &mut self.leaves;
        loop {
            match r.int()? {
                1005 => return Ok(()),
                3000 => leaf.blossom_distance = r.float()?,
                3001 => leaf.normal_depth = r.int()?,
                3002 => leaf.blossom_level = r.float()?,
                3003 | 3006 => {
                    r.flag()?;
                }
                3004 | 3005 => {
                    r.float()?;
                }
                3007 => leaf.collision_scale = r.float()?,
                3008 => leaf.collision = r.int()?,
                3009 => leaf.dimming = r.flag()?,
                3010 => leaf.dimming_amount = r.float()?,
                1009 => {
                    // A token (1006) the game reads past, then the count.
                    r.int()?;
                    let count = r.int()?;
                    leaf.textures.clear();
                    for _ in 0..count.max(0) {
                        let mut tex = LeafTexture::default();
                        // 1007 and the first field's token.
                        r.int()?;
                        loop {
                            match r.int()? {
                                1008 => break,
                                4000 => tex.blossom = r.flag()?,
                                4001 => tex.color = r.vec3()?,
                                4002 => tex.color_variance = r.float()?,
                                4003 => tex.file = file_name(&r.string()?),
                                4004 => tex.origin = r.vec3()?,
                                4005 => tex.size = r.vec3()?,
                                4006 => tex.absolute_size = r.vec3()?,
                                4007 => {
                                    r.float()?;
                                }
                                t => {
                                    return r.error(format!(
                                        "malformed single leaf information (token {t})"
                                    ))
                                }
                            }
                        }
                        leaf.textures.push(tex);
                    }
                    // The list's end (1010).
                    r.int()?;
                }
                t => return r.error(format!("malformed general leaf information (token {t})")),
            }
        }
    }

    /// 1011 (`00b408b0`), until 1012.
    fn read_leaf_wind(&mut self, r: &mut Reader) -> Result<()> {
        loop {
            match r.int()? {
                1012 => return Ok(()),
                5000 | 5001 | 5003 => {
                    r.vec3()?;
                }
                5002 => self.leaf_wind.speeds = r.vec3()?,
                5004 => self.leaf_wind.angles = r.vec3()?,
                5005 => self.leaf_wind.value = r.float()?,
                5006 => {
                    r.flag()?;
                }
                t => return r.error(format!("malformed general wind information (token {t})")),
            }
        }
    }

    /// 7000 (`00b2cbc0`).
    fn read_stored_leaves(&mut self, r: &mut Reader) -> Result<()> {
        let count = r.int()?;
        self.lod.leaf_lods = Some(count);
        let mut lods = Vec::new();
        loop {
            let t = r.int()?;
            if t == 7001 {
                break;
            }
            if lods.len() as i32 >= count || t != 7002 {
                return r.error("malformed leaf lod data");
            }
            let mut leaves = Vec::new();
            loop {
                match r.int()? {
                    7003 => break,
                    7004 => leaves.push(read_stored_leaf(r)?),
                    _ => return r.error("malformed leaf lod data"),
                }
            }
            lods.push(leaves);
        }
        self.stored_leaves = Some(lods);
        Ok(())
    }

    /// 8000 (`00b2a940`), until 8001.
    fn read_lighting(&mut self, r: &mut Reader) -> Result<()> {
        let l = &mut self.lighting;
        loop {
            match r.int()? {
                8001 => return Ok(()),
                8002 => l.methods[0] = r.int()?,
                8003 => read13(r, &mut l.materials[0])?,
                8004 => l.methods[1] = r.int()?,
                8005 => read13(r, &mut l.materials[1])?,
                8006 => l.value_70 = r.float()?,
                8007 => l.value_74 = r.int()?,
                8008 => l.methods[2] = r.int()?,
                8009 => read13(r, &mut l.materials[2])?,
                t => return r.error(format!("malformed lighting information (token {t})")),
            }
        }
    }

    /// 9000 (`00b092a0`), until 9001.
    fn read_lod(&mut self, r: &mut Reader) -> Result<()> {
        loop {
            match r.int()? {
                9001 => return Ok(()),
                9002 => self.lod.mode = r.int()?,
                9003 => self.lod.value_1c = r.float()?,
                9004 => self.lod.value_20 = r.float()?,
                9009 => self.lod.value_24 = r.float()?,
                9005 => loop {
                    // `00b2ceb0`, until 9006.
                    match r.int()? {
                        9006 => break,
                        9007 => self.lod.branch_lods = r.int()?,
                        9008 => self.lod.branch_min = r.float()?,
                        9010 => {
                            let v = r.float()?;
                            self.lod.value_fc = if v == 0.0 { 0.1 } else { v };
                        }
                        9011 => self.lod.leaf_lods = Some(r.int()?),
                        9012 => self.lod.branch_max = r.float()?,
                        9013 => self.lod.branch_random = r.float()?,
                        9014 => self.lod.branch_keep = r.float()?,
                        t => return r.error(format!("malformed engine lod data (token {t})")),
                    }
                },
                t => return r.error(format!("malformed lod info (token {t})")),
            }
        }
    }

    /// 10000 (`00b09960`), until 10001.
    fn read_billboards(&mut self, r: &mut Reader) -> Result<()> {
        let b = self.billboards.get_or_insert_with(Billboards::default);
        loop {
            match r.int()? {
                10001 => return Ok(()),
                10002 => b.a = read_eights(r)?,
                10003 => b.b = read_eights(r)?,
                10004 => b.c = read_eights(r)?,
                10005 => b.texture = file_name(&r.string()?),
                10006 => b.flags[1] = r.flag()?,
                10007 => b.flags[0] = r.flag()?,
                t => return r.error(format!("malformed billboard info (token {t})")),
            }
        }
    }

    /// 11000 (`00b097e0`), until 11001.
    fn read_seed_level(&mut self, r: &mut Reader) -> Result<()> {
        loop {
            match r.int()? {
                11001 => return Ok(()),
                11002 => self.seed_level = r.int()?,
                t => return r.error(format!("malformed new wind info (token {t})")),
            }
        }
    }

    /// 12000 (`00b0a490`), until 12001.
    fn read_collision(&mut self, r: &mut Reader) -> Result<()> {
        loop {
            let kind = match r.int()? {
                12001 => return Ok(()),
                12002 => 0,
                12003 => 1,
                12004 => 2,
                t => return r.error(format!("unknown collision object type (token {t})")),
            };
            let position = r.vec3()?;
            let mut dimensions = [0.0; 3];
            for d in dimensions.iter_mut().take(kind as usize + 1) {
                *d = r.float()?;
            }
            self.collision.push(CollisionObject {
                kind,
                position,
                dimensions,
            });
        }
    }

    /// 13000 (`00b18340`), until 13001.
    fn read_fronds(&mut self, r: &mut Reader) -> Result<()> {
        let f = &mut self.fronds;
        loop {
            match r.int()? {
                13001 => return Ok(()),
                13002 => f.level = r.int()?,
                13003 => f.kind = r.int()?,
                13004 => f.blades = r.int()?,
                13005 => f.profile = Spline::parse(&r.string()?),
                13006 => f.profile_segments = r.int()?,
                13007 => f.enabled = r.flag()?,
                13008 => {
                    let count = r.int()?;
                    f.textures.clear();
                    for _ in 0..count.max(0) {
                        let mut tex = FrondTexture {
                            file: String::new(),
                            values: [0.5, 1.0, 0.0, 0.0],
                        };
                        // 14000 and the first field's token.
                        r.int()?;
                        loop {
                            match r.int()? {
                                14001 => break,
                                14002 => tex.file = file_name(&r.string()?),
                                14003 => tex.values[0] = r.float()?,
                                14004 => tex.values[1] = r.float()?,
                                14005 => tex.values[2] = r.float()?,
                                14006 => tex.values[3] = r.float()?,
                                t => {
                                    return r.error(format!(
                                        "malformed frond texture information (token {t})"
                                    ))
                                }
                            }
                        }
                        f.textures.push(tex);
                    }
                }
                13009 => f.lods = r.int()?,
                13010 => f.values[0] = r.float()?,
                13011 => f.values[1] = r.float()?,
                13012 => f.values[2] = r.float()?,
                13013 => f.values[3] = r.float()?,
                14007 => f.value_7c = r.int()?,
                14008 => f.value_80 = r.int()?,
                t => return r.error(format!("malformed frond info (token {t})")),
            }
        }
    }

    /// 15000 (`00b2d090`): per level 15002 flag, 15003 value; then 15001.
    fn read_texture_controls(&mut self, r: &mut Reader) -> Result<()> {
        for i in 0..self.levels.len() {
            r.expect(15002, "malformed texture controls")?;
            self.levels[i].twist_texture = r.flag()?;
            r.expect(15003, "malformed texture controls")?;
            self.levels[i].u_offset = r.float()?;
        }
        r.expect(15001, "malformed texture controls")
    }

    /// 16000 (`00b2d2e0`): per level 16002–16012; then 16001.
    fn read_flares(&mut self, r: &mut Reader) -> Result<()> {
        for i in 0..self.levels.len() {
            r.expect(16002, "malformed flare info")?;
            self.levels[i].segment_exponent = r.float()?;
            r.expect(16003, "malformed flare info")?;
            self.levels[i].flares = r.int()?;
            for (k, token) in (16004..=16012).enumerate() {
                r.expect(token, "malformed flare info")?;
                self.levels[i].flare_values[k] = r.float()?;
            }
        }
        r.expect(16001, "malformed flare info")
    }

    /// 18000 (`00b3e750`), until 18001.
    fn read_18000(&mut self, r: &mut Reader) -> Result<()> {
        let mut block = ([0.0; 3], [0.0; 3], [0.0; 3], String::new());
        loop {
            match r.int()? {
                18001 => break,
                18002 => block.0 = r.vec3()?,
                18003 => block.1 = r.vec3()?,
                18004 => block.2 = r.vec3()?,
                18005 => block.3 = file_name(&r.string()?),
                t => return r.error(format!("malformed frond info (token {t})")),
            }
        }
        self.block_18000 = Some(block);
        Ok(())
    }

    /// 19000 (`00b0d500`), until 19001.
    fn read_19000(&mut self, r: &mut Reader) -> Result<()> {
        loop {
            match r.int()? {
                19001 => return Ok(()),
                19002 => self.texture_19002 = Some(r.string()?),
                _ => {}
            }
        }
    }

    /// 20000 (`00b0d810`), until 20001; only with billboards.
    fn read_billboard_coords(&mut self, r: &mut Reader) -> Result<()> {
        let Some(b) = self.billboards.as_mut() else {
            return Ok(());
        };
        loop {
            match r.int()? {
                20001 => return Ok(()),
                20002 => b.texture = file_name(&r.string()?),
                20003 => b.flags[1] = r.flag()?,
                20004 => b.flags[0] = r.flag()?,
                20005 => {
                    let mut c = [0.0; 8];
                    for v in c.iter_mut() {
                        *v = r.float()?;
                    }
                    b.coords = Some(c);
                }
                t => return r.error(format!("malformed texture coord info (token {t})")),
            }
        }
    }
}

/// 1016..1017 (`00b3f8e0`).
fn read_level(r: &mut Reader) -> Result<BranchLevel> {
    let mut level = BranchLevel::default();
    r.expect(1016, "malformed branch data")?;
    loop {
        match r.int()? {
            1017 => return Ok(level),
            6000 => level.disturbance = Spline::parse(&r.string()?),
            6001 => level.gravity = Spline::parse(&r.string()?),
            6002 => level.flexibility = Spline::parse(&r.string()?),
            6003 => level.flexibility_scale = Spline::parse(&r.string()?),
            6004 => level.length = Spline::parse(&r.string()?),
            6005 => level.radius = Spline::parse(&r.string()?),
            6006 => level.radius_scale = Spline::parse(&r.string()?),
            6007 => level.start_angle = Spline::parse(&r.string()?),
            6008 => level.cross_sections = r.int()?,
            6009 => level.segments = r.int()?,
            6010 => level.first_branch = r.float()?,
            6011 => level.last_branch = r.float()?,
            6012 => level.frequency = r.float()?,
            6013 => level.u_tile = r.float()?,
            6014 => level.v_tile = r.float()?,
            6015 => level.u_tile_absolute = r.flag()?,
            6016 => level.v_tile_absolute = r.flag()?,
            6017 => level.gravity_scale = Spline::parse(&r.string()?),
            t => return r.error(format!("malformed general branch information (token {t})")),
        }
    }
}

/// 7004..7005 (`00b405c0`).
fn read_stored_leaf(r: &mut Reader) -> Result<StoredLeaf> {
    let mut leaf = StoredLeaf::default();
    loop {
        match r.int()? {
            7005 => return Ok(leaf),
            7006 => leaf.color = r.int_skip()?,
            7007 => leaf.rock_group = r.byte()?,
            7008 => leaf.dimming = r.byte()?,
            7010 => leaf.normal = r.vec3()?,
            7011 => leaf.texture = r.byte()?,
            7012 => leaf.wind_weight = r.byte()? as f32 / 255.0,
            7013 => leaf.wind_group = r.int()?,
            7015 => leaf.position = r.vec3()?,
            7016 => {
                r.float()?;
            }
            _ => return r.error("malformed billboard leaf"),
        }
    }
}

fn read13(r: &mut Reader, out: &mut [f32; 13]) -> Result<()> {
    for v in out.iter_mut() {
        *v = r.float()?;
    }
    Ok(())
}

fn read_eights(r: &mut Reader) -> Result<Vec<[f32; 8]>> {
    let n = r.int()?;
    let mut out = Vec::new();
    for _ in 0..n.max(0) {
        let mut e = [0.0; 8];
        for v in e.iter_mut() {
            *v = r.float()?;
        }
        out.push(e);
    }
    Ok(out)
}

impl From<SptError> for String {
    fn from(e: SptError) -> String {
        e.to_string()
    }
}
