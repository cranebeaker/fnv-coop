//! `fallout shared/distant terrain system/bgsterrainnode.cpp` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BGSTerrainNode` is one square of the distant-terrain quad tree: it
//! covers `iLevel` cells from (`sBaseCellX`, `sBaseCellY`), owns up to four
//! children (the four quarters) and, depending on the distance to the
//! viewer, a terrain chunk, a distant object block and a tree block. The
//! chunk, block and tree-block functions belong to other units and are
//! called by address; so are the small field getters the code calls (the
//! linker shares one body between many classes, so their map names are
//! often unrelated).
//!
//! x87 note: the translations compute in `f64` and round to `f32` where the
//! code stores a `float` (docs/ENGINE_CRATE.md); floats the code copies with
//! `FLD`/`FSTP` go through [`x87_quiet`].

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiPoint3;

/// `00401000` (cdecl): allocates the given number of bytes.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (cdecl): frees the block (the deleting destructors' delete).
const DEALLOCATE: u32 = 0x0040_1030;
/// `00403d30` (cdecl `(dest, value, count)`): forwards to `memset`.
const MEMSET: u32 = 0x0040_3d30;
/// The scope guard the update functions and the constructor keep on their
/// stack (a 4-byte local): `(guard, level, 1, source file, line)`, then
/// its destructor.
const GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// `"D:\\_Fallout3\\Platforms\\Common\\Code\\Fallout Shared\\Distant
/// Terrain System\\BGSTerrainNode.cpp"`.
const SOURCE_FILE: u32 = 0x0106_e4e0;
/// `"MODELS: Morph distances are invalid.  Please ensure that
/// fMorphDistanceMult is less than fSplitDistanceMult under
/// [TerrainManager] in the INI (or delete both)"`.
const MORPH_DISTANCE_WARNING: u32 = 0x0106_e440;
/// `005b5e40` (cdecl): called with the warning text above.
const PRINT_WARNING: u32 = 0x005b_5e40;

/// `Setting` value getters: `this + 4`, or a zeroed scratch value when
/// `this` is null. `00403e20` is the float one, `0043d4d0` the unsigned
/// one, `00408d60` the byte one.
const SETTING_FLOAT: u32 = 0x0040_3e20;
const SETTING_UNSIGNED: u32 = 0x0043_d4d0;
const SETTING_BYTE: u32 = 0x0040_8d60;
/// The `Setting` objects the node reads (their names are the strings their
/// static initializers pass).
const SPLIT_DISTANCE_MULT: u32 = 0x011d_86e0; // fSplitDistanceMult:TerrainManager
const MORPH_START_DISTANCE_MULT: u32 = 0x011d_8754; // fMorphStartDistanceMult:TerrainManager
const MORPH_END_DISTANCE_MULT: u32 = 0x011d_86d4; // fMorphEndDistanceMult:TerrainManager
const BLOCK_LOAD_DISTANCE: u32 = 0x011d_877c; // fBlockLoadDistance:TerrainManager
const BLOCK_LOAD_DISTANCE_LOW: u32 = 0x011d_8724; // fBlockLoadDistanceLow:TerrainManager
const TREE_LOAD_DISTANCE: u32 = 0x011d_8788; // fTreeLoadDistance:TerrainManager
const DETAIL_TEXTURE_SCALE: u32 = 0x011d_86f8; // fDetailTextureScale:TerrainManager
const TEXTURE_FADE_TIME: u32 = 0x011d_8740; // uTerrainTextureFadeTime:TerrainManager
const KEEP_LOW_DETAIL_TERRAIN: u32 = 0x011d_8760; // bKeepLowDetailTerrain:TerrainManager
const GRIDS_TO_LOAD: u32 = 0x011c_63cc; // uGridsToLoad:General

/// `4096.0` (`double`): units per cell.
const UNITS_PER_CELL: u32 = 0x0101_7a10;
/// `2048.0` (`double`): half a cell.
const HALF_CELL: u32 = 0x0101_6968;
/// `0.5` (`double`).
const ONE_HALF: u32 = 0x0101_1588;
/// `0.5` (`float`): the parent-position offsets.
const ONE_HALF_FLOAT: u32 = 0x0101_6248;
/// `4.0` (`double`).
const FOUR: u32 = 0x0101_db80;
/// The morph distances of a node without a parent (`float`s, `00800000`
/// and `7f7fffff` in the exe).
const ROOT_MORPH_START: u32 = 0x0102_efc4;
const ROOT_MORPH_END: u32 = 0x0101_6970;

/// The global pointer the code passes as `this` of `TES::GetWorldSpace`
/// (`004fd3e0`, Xbox PDB map) and `TES::IsCellLoaded` (`004511e0`).
const TES_POINTER: u32 = 0x011d_ea10;
/// The global word [`fn_006fd8e0`] returns (compared with a node's
/// manager).
const ACTIVE_MANAGER: u32 = 0x011d_8670;
/// The byte [`fn_006fe540`] returns: written by
/// `BGSDistantTreeBlock::AllocateStaticData` (`006f77a0`) and `006f83f0`.
const TREE_STATIC_DATA_FLAG: u32 = 0x011d_85a0;
/// The object whose word at `+0x14` (read by `00825c00`) the texture morph
/// uses as the current time.
const TIMER_OBJECT: u32 = 0x011f_6394;

/// Field getters called by address (`this` only): the word at `+4`,
/// `+0x18`, `+0x1c`, `+0x24`, `+0x28`, `+0x14`, `+0x88` and `+0`.
const WORD_AT_4: u32 = 0x0072_6070;
const WORD_AT_18: u32 = 0x0096_11e0;
const WORD_AT_1C: u32 = 0x0044_1110;
const WORD_AT_24: u32 = 0x0059_bb30;
const WORD_AT_28: u32 = 0x0045_cd60;
const WORD_AT_14: u32 = 0x0082_5c00;
/// `TES::GetWorldSpace` (Xbox PDB map): the word at `+0x88`.
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `*this` (a smart pointer's object, or the first word).
const WORD_AT_0: u32 = 0x0055_9450;
/// The byte at `+0x28`, `+0x29`, `+0x5c`, `+0x23`, `+0x20`, `+0x24`.
const BYTE_AT_28: u32 = 0x0097_4d90;
const BYTE_AT_29: u32 = 0x0075_9d80;
const BYTE_AT_5C: u32 = 0x006f_b150;
const BYTE_AT_23: u32 = 0x006f_6070;
const BYTE_AT_20: u32 = 0x0052_9ea0;
const BYTE_AT_24: u32 = 0x0054_3c30;
/// The signed word at `+0x08` (`004a8ae0`, map name
/// `TESActorBaseData::GetFatigue`) and at `+0x0a` (`0047d3f0`).
const SHORT_AT_8: u32 = 0x004a_8ae0;
const SHORT_AT_A: u32 = 0x0047_d3f0;
/// The smart pointer at `+0x2c`, `+0x08`, `+0x0c` (dereferenced).
const POINTER_AT_2C: u32 = 0x0056_c7f0;
const POINTER_AT_8: u32 = 0x0055_85e0;
const POINTER_AT_C: u32 = 0x0043_b230;

/// `00452dc0(this, x, y)`: stores the two floats at `this`, returns `this`.
const POINT2_SET: u32 = 0x0045_2dc0;
/// `004e8880(this, out, other)`: `out = this - other` (two floats),
/// returns `out`.
const POINT2_SUBTRACT: u32 = 0x004e_8880;
/// `00416870(this, x, y, z)`: stores the three floats, returns `this`.
const POINT3_SET: u32 = 0x0041_6870;
/// `006815c0(this)`: returns `this` (the `NiPoint3` member's constructor).
const POINT3_CONSTRUCT: u32 = 0x0068_15c0;
/// `00439ef0(this, out, other)`: `out = this - other` (three floats),
/// returns `out`.
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
/// `00457990(this)`: the length of the point, in `ST0`.
const POINT3_LENGTH: u32 = 0x0045_7990;
/// `004019d0` (cdecl, a `float`): the square root, in `ST0`.
const SQUARE_ROOT: u32 = 0x0040_19d0;
/// `00408840` (cdecl, a `float`): forwards to `00408860` (the absolute
/// value), in `ST0`.
const ABSOLUTE: u32 = 0x0040_8840;

/// The terrain chunk (`bgsterrainchunk.cpp`): constructor `(chunk, node,
/// level, x, y)` and destructor.
const CHUNK_CONSTRUCT: u32 = 0x006f_a210;
const CHUNK_DESTRUCT: u32 = 0x006f_a420;
/// Chunk functions called with the chunk only.
const CHUNK_CALL_006FACB0: u32 = 0x006f_acb0;
const CHUNK_CALL_006FB6E0: u32 = 0x006f_b6e0;
const CHUNK_CALL_006FB760: u32 = 0x006f_b760;
/// `006faa90(chunk, float)`: called with the morph factor.
const CHUNK_SET_MORPH: u32 = 0x006f_aa90;
/// `BGSTerrainChunk::GetTexture` (Xbox PDB map): a chunk and an index.
const CHUNK_GET_TEXTURE: u32 = 0x006f_abc0;
/// `006fb940(geometry, x, y, factor)`: three floats.
const GEOMETRY_SET_MORPH: u32 = 0x006f_b940;
/// `006ff380(node)`: ends the texture morph (not in this unit's queue).
const END_TEXTURE_MORPH: u32 = 0x006f_f380;
/// `007fa950(chunk)`: a word the update keeps (non-zero: handed to
/// `00450f90(word, 1)`).
const CHUNK_PENDING_OBJECT: u32 = 0x007f_a950;
const PENDING_OBJECT_CALL: u32 = 0x0045_0f90;
/// `00507700(geometry, float)`: called with the detail texture scale.
const GEOMETRY_SET_SCALE: u32 = 0x0050_7700;
/// `005467c0(node, flag)`, called around the virtual at `+0xbc`.
const SCENE_NODE_FLAG: u32 = 0x0054_67c0;
/// `00586340(world, flag)`: a world space test.
const WORLD_TEST: u32 = 0x0058_6340;

/// The distant object block (`bgsdistantobjectblock.cpp`): constructor
/// `(block, node, level, x, y, low)` and destructor.
const BLOCK_CONSTRUCT: u32 = 0x006f_5160;
const BLOCK_DESTRUCT: u32 = 0x006f_5370;
const BLOCK_CALL_006F5CA0: u32 = 0x006f_5ca0;
const BLOCK_CALL_006F5AF0: u32 = 0x006f_5af0;
/// `(block, world, level, x, y)`: one for a block that has its low-detail
/// flag (`+0x23`) set and the node wants low detail, one for neither.
const BLOCK_CALL_006F6370: u32 = 0x006f_6370;
const BLOCK_CALL_006F6540: u32 = 0x006f_6540;
/// `006f5800(block, 1)`: an object of the block.
const BLOCK_OBJECTS: u32 = 0x006f_5800;
/// `BGSDistantObjectBlock::UpdateStingerVisibility` (Xbox PDB map):
/// `(block, objects, level, x, y)`.
const BLOCK_UPDATE_STINGER_VISIBILITY: u32 = 0x006f_5970;
const BLOCK_CALL_006F5A70: u32 = 0x006f_5a70;

/// The tree block (`bgsdistanttreeblock.cpp`): constructor `(trees, node,
/// level, x, y)` and destructor.
const TREES_CONSTRUCT: u32 = 0x006f_7540;
const TREES_DESTRUCT: u32 = 0x006f_76b0;
/// `BGSDistantTreeBlock::AllocateStaticData` (Xbox PDB map, cdecl `world`).
const TREES_ALLOCATE_STATIC_DATA: u32 = 0x006f_77a0;
const TREES_CALL_006F8C40: u32 = 0x006f_8c40;
/// `BGSDistantTreeBlock::UpdateShaderProperties` (Xbox PDB map).
const TREES_UPDATE_SHADER_PROPERTIES: u32 = 0x006f_91a0;
/// `006f83f0()`: no arguments.
const TREES_CALL_006F83F0: u32 = 0x006f_83f0;

/// The node's scalar deleting destructor (`006fca40(node, flags)`).
const NODE_DELETE: u32 = 0x006f_ca40;

/// `BSSegmentedTriShape` (Xbox PDB map names).
const SHAPE_GET_NUM_SEGMENTS: u32 = 0x006f_bfe0;
const SHAPE_ENABLE_SEGMENT: u32 = 0x00a8_6ff0;
const SHAPE_DISABLE_SEGMENT: u32 = 0x00a8_7020;
const SHAPE_MERGE_ENABLED_SEGMENTS: u32 = 0x00a8_7040;
const SHAPE_ENABLE_ALL_SEGMENTS: u32 = 0x00a8_72d0;
/// `TESWorldSpace::GetCellFromCellCoord` (Xbox PDB map): `(world, x, y)`.
const WORLD_GET_CELL: u32 = 0x0058_75a0;
/// `TES::IsCellLoaded` (Xbox PDB map): `(tes, cell, 1)`.
const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
/// `TESObjectCELL::All3DWithVisibleDistantFadingIn` (Xbox PDB map).
const CELL_ALL_3D_FADING_IN: u32 = 0x0055_7d30;

/// `NiPointer` constructor `(pointer, object)` and destructor.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;

/// The level at which nodes carry tree blocks ([`fn_006fe510`]).
const TREE_LEVEL: u32 = 8;

layout! {
    /// `BGSTerrainNode` (Xbox PDB), 0x60 bytes (the allocation size on PC;
    /// every offset below is the one the PC code uses).
    pub struct BGSTerrainNode: 0x60 {
        /// `pManager` (Xbox PDB): `BGSTerrainManager*`.
        0x00 pManager: Ptr,
        /// `iLevel` (Xbox PDB): the node's size in cells.
        0x04 iLevel: u32,
        /// `sBaseCellX` (Xbox PDB).
        0x08 sBaseCellX: i16,
        /// `sBaseCellY` (Xbox PDB).
        0x0A sBaseCellY: i16,
        /// `bHasGeometry` (Xbox PDB).
        0x0C bHasGeometry: bool,
        /// `bChildrenLoaded` (Xbox PDB).
        0x0D bChildrenLoaded: bool,
        /// `pChunk` (Xbox PDB): `BGSTerrainChunk*`.
        0x10 pChunk: Ptr,
        /// `pBlock` (Xbox PDB): `BGSDistantObjectBlock*`.
        0x14 pBlock: Ptr,
        /// `pTrees` (Xbox PDB): `BGSDistantTreeBlock*`.
        0x18 pTrees: Ptr,
        /// `pSceneNode` (Xbox PDB): `NiNode*`.
        0x1C pSceneNode: Ptr,
        /// `pChildren[4]` (Xbox PDB): the first child (see [`child`]).
        0x20 pChildren: Ptr<BGSTerrainNode>,
        /// `pParent` (Xbox PDB).
        0x30 pParent: Ptr<BGSTerrainNode>,
        /// `Center` (Xbox PDB): `NiPoint3`.
        0x34 Center: Inline<NiPoint3>,
        /// `fCornerRadius` (Xbox PDB).
        0x40 fCornerRadius: f32,
        /// `fSplitDistance` (Xbox PDB).
        0x44 fSplitDistance: f32,
        /// `fStartMorphDistance` (Xbox PDB).
        0x48 fStartMorphDistance: f32,
        /// `fEndMorphDistance` (Xbox PDB).
        0x4C fEndMorphDistance: f32,
        /// `uiFadeStartTime` (Xbox PDB).
        0x50 uiFadeStartTime: u32,
        /// `bTexFading` (Xbox PDB).
        0x54 bTexFading: bool,
        /// `ParentPosition` (Xbox PDB): which quarter of the parent (1 to 4,
        /// 0 for the root).
        0x58 ParentPosition: u32,
        /// `bPersistentNode` (Xbox PDB).
        0x5C bPersistentNode: bool,
        /// `bSplitNode` (Xbox PDB).
        0x5D bSplitNode: bool,
    }
}

/// `pChildren[index]`.
pub(crate) fn child(e: &Engine, this: Ptr<BGSTerrainNode>, index: u32) -> Ptr<BGSTerrainNode> {
    Ptr::new(e.mem.u32(this.addr() + 0x20 + 4 * index))
}

fn set_child(e: &mut Engine, this: Ptr<BGSTerrainNode>, index: u32, node: Ptr<BGSTerrainNode>) {
    e.mem.set_u32(this.addr() + 0x20 + 4 * index, node.addr());
}

/// Runs `body` inside the scope guard (`level`, line `line`).
fn with_scope_guard<R>(
    e: &mut Engine,
    level: u32,
    line: u32,
    body: impl FnOnce(&mut Engine) -> R,
) -> R {
    e.with_stack(4, |e, guard| {
        e.call(
            GUARD_CONSTRUCT,
            &args![guard, level, 1u32, SOURCE_FILE, line],
        );
        let result = body(e);
        e.call(GUARD_DESTRUCT, &args![guard]);
        result
    })
}

/// The float value of a `Setting`.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_FLOAT, &args![setting]).u32();
    e.mem.f32(value)
}

/// The unsigned value of a `Setting`.
fn setting_unsigned(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_UNSIGNED, &args![setting]).u32();
    e.mem.u32(value)
}

/// The world space of the node's manager (`*this` is the manager, whose
/// first word is the world).
fn manager_world(e: &mut Engine, this: Ptr<BGSTerrainNode>) -> u32 {
    let manager = e.call(WORD_AT_0, &args![this]).u32();
    e.call(WORD_AT_0, &args![manager]).u32()
}

/// The node's square in units: `iLevel * 4096` as a `float`.
fn node_size(e: &Engine, this: Ptr<BGSTerrainNode>) -> f32 {
    let level = e.get(this, BGSTerrainNode::iLevel) as u64 as f64;
    (level * e.global::<f64>(UNITS_PER_CELL)) as f32
}

/// The distance from `position` to the node's square, as the update and
/// test functions compute it inline: both points are built on the stack
/// with `00452dc0` and handed to [`fn_006fe830`]; the result is stored as a
/// `float`.
fn distance_to_square(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) -> f32 {
    e.with_stack(0x10, |e, locals| {
        let point = locals.addr();
        let center = point + 8;
        let position_x = e.mem.u32(position.addr());
        let position_y = e.mem.u32(position.addr() + 4);
        e.call(
            POINT2_SET,
            &args![
                point,
                x87_quiet_bits(position_x),
                x87_quiet_bits(position_y)
            ],
        );
        let center_point = this.at(BGSTerrainNode::Center);
        let center_x = e.get(center_point, NiPoint3::x).to_bits();
        let center_y = e.get(center_point, NiPoint3::y).to_bits();
        e.call(
            POINT2_SET,
            &args![center, x87_quiet_bits(center_x), x87_quiet_bits(center_y)],
        );
        let size = node_size(e, this);
        let (center_x, center_y) = (e.mem.f32(center), e.mem.f32(center + 4));
        let (point_x, point_y) = (e.mem.f32(point), e.mem.f32(point + 4));
        fn_006fe830(e, this, center_x, center_y, size, point_x, point_y) as f32
    })
}

/// A new terrain chunk for the node (`0x30` bytes, null when the
/// allocation fails).
fn new_chunk(e: &mut Engine, this: Ptr<BGSTerrainNode>) -> Ptr {
    let storage = e.call(ALLOCATE, &args![0x30u32]).u32();
    if storage == 0 {
        return Ptr::NULL;
    }
    let level = e.get(this, BGSTerrainNode::iLevel);
    let x = e.get(this, BGSTerrainNode::sBaseCellX) as u16 as u32;
    let y = e.get(this, BGSTerrainNode::sBaseCellY) as u16 as u32;
    e.call(CHUNK_CONSTRUCT, &args![storage, this, level, x, y])
        .ptr()
}

// Translated from 006fd210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainNode::BGSTerrainNode` (Xbox PDB): sets up a node of `level`
/// cells at (`x`, `y`) under `manager` and `parent`: its center and corner
/// radius, whether it has geometry (`level` up to the manager's `+0x18`),
/// and, above the manager's `+0x1c` level, its four children (each half the
/// size; the child flag is set when `bKeepLowDetailTerrain` is on and the
/// half level equals the manager's `+0x18`), the split distance and the
/// morph distances from the parent's corner radius (warning when the start
/// is below the end). Returns `this`.
// Translation incomplete: 006fd210 - SEH exception unwinding (FS:[0] frame) is unsupported.
#[allow(clippy::too_many_arguments)]
pub fn bgs_terrain_node_constructor(
    e: &mut Engine,
    this: Ptr<BGSTerrainNode>,
    manager: Ptr,
    parent: Ptr<BGSTerrainNode>,
    x: i16,
    y: i16,
    level: u32,
    persistent: bool,
) -> Ptr<BGSTerrainNode> {
    let center = this.at(BGSTerrainNode::Center);
    e.call(POINT3_CONSTRUCT, &args![center]);
    with_scope_guard(e, 0x22, 0x4c, |e| {
        e.set(this, BGSTerrainNode::bSplitNode, true);
        e.set(this, BGSTerrainNode::pChunk, Ptr::NULL);
        e.set(this, BGSTerrainNode::pBlock, Ptr::NULL);
        e.set(this, BGSTerrainNode::pTrees, Ptr::NULL);
        e.set(this, BGSTerrainNode::pManager, manager);
        e.set(this, BGSTerrainNode::pParent, parent);
        e.set(this, BGSTerrainNode::iLevel, level);
        e.set(this, BGSTerrainNode::sBaseCellX, x);
        e.set(this, BGSTerrainNode::sBaseCellY, y);
        for index in 0..4 {
            set_child(e, this, index, Ptr::NULL);
        }
        let max_level = e.call(WORD_AT_18, &args![manager]).u32();
        e.set(this, BGSTerrainNode::bHasGeometry, level <= max_level);

        let units_per_cell: f64 = e.global(UNITS_PER_CELL);
        let one_half: f64 = e.global(ONE_HALF);
        let size = ((level << 12) as u64 as f64) as f32;
        let cell_x = e.get(this, BGSTerrainNode::sBaseCellX) as f64;
        let cell_y = e.get(this, BGSTerrainNode::sBaseCellY) as f64;
        let center_x = (cell_x * units_per_cell + size as f64 * one_half) as f32;
        let center_y = (cell_y * units_per_cell + size as f64 * one_half) as f32;
        let base_x = (cell_x * units_per_cell) as f32;
        let base_y = (cell_y * units_per_cell) as f32;
        let radius = e.with_stack(0x24, |e, locals| {
            let base = locals.addr();
            let built = base + 0x0c;
            let difference = base + 0x18;
            e.call(POINT3_SET, &args![base, base_x, base_y, 0.0f32]);
            let point = e
                .call(POINT3_SET, &args![built, center_x, center_y, 0.0f32])
                .u32();
            for offset in [0, 4, 8] {
                let word = e.mem.u32(point + offset);
                e.mem.set_u32(center.addr() + offset, word);
            }
            let point = e
                .call(POINT3_SUBTRACT, &args![center, difference, base])
                .u32();
            e.call(POINT3_LENGTH, &args![point]).f32()
        });
        e.set(this, BGSTerrainNode::fCornerRadius, radius);
        e.set(this, BGSTerrainNode::bPersistentNode, persistent);

        let min_level = e.call(WORD_AT_1C, &args![manager]).u32();
        if level > min_level {
            let half_level = level >> 1;
            let mid_x = (x as i32).wrapping_add(half_level as i32) as u16;
            let mid_y = (y as i32).wrapping_add(half_level as i32) as u16;
            let mut child_flag = false;
            let keep = e.call(SETTING_BYTE, &args![KEEP_LOW_DETAIL_TERRAIN]).u32();
            if e.mem.u8(keep) != 0 {
                let max_level = e.call(WORD_AT_18, &args![manager]).u32();
                if half_level == max_level {
                    child_flag = true;
                }
            }
            let quarters = [
                (x as u16, y as u16),
                (x as u16, mid_y),
                (mid_x, y as u16),
                (mid_x, mid_y),
            ];
            for (index, (child_x, child_y)) in quarters.into_iter().enumerate() {
                let storage = e.call(ALLOCATE, &args![0x60u32]).u32();
                let node = if storage == 0 {
                    Ptr::NULL
                } else {
                    bgs_terrain_node_constructor(
                        e,
                        Ptr::new(storage),
                        manager,
                        this,
                        child_x as i16,
                        child_y as i16,
                        half_level,
                        child_flag,
                    )
                };
                set_child(e, this, index as u32, node);
            }
            for (index, position) in [(0, 1u32), (1, 3), (2, 2), (3, 4)] {
                let node = child(e, this, index);
                e.set(node, BGSTerrainNode::ParentPosition, position);
            }
            let multiplier = setting_float(e, SPLIT_DISTANCE_MULT);
            let radius = e.get(this, BGSTerrainNode::fCornerRadius);
            let split = (radius as f64 * multiplier as f64) as f32;
            e.set(this, BGSTerrainNode::fSplitDistance, split);
        } else {
            e.call(MEMSET, &args![this.addr() + 0x20, 0u32, 0x10u32]);
            e.set(this, BGSTerrainNode::fSplitDistance, 0.0);
        }

        if parent.is_null() {
            let start = x87_quiet(e.global::<f32>(ROOT_MORPH_START));
            let end = x87_quiet(e.global::<f32>(ROOT_MORPH_END));
            e.set(this, BGSTerrainNode::fStartMorphDistance, start);
            e.set(this, BGSTerrainNode::fEndMorphDistance, end);
            e.set(this, BGSTerrainNode::ParentPosition, 0);
        } else {
            let multiplier = setting_float(e, MORPH_START_DISTANCE_MULT);
            let parent_radius = e.get(parent, BGSTerrainNode::fCornerRadius);
            let start = (parent_radius as f64 * multiplier as f64) as f32;
            e.set(this, BGSTerrainNode::fStartMorphDistance, start);
            let multiplier = setting_float(e, MORPH_END_DISTANCE_MULT);
            let parent_radius = e.get(parent, BGSTerrainNode::fCornerRadius);
            let end = (parent_radius as f64 * multiplier as f64) as f32;
            e.set(this, BGSTerrainNode::fEndMorphDistance, end);
            // `FCOMPP` of start against end, `TEST AH,5` / `JP`: the warning
            // only when start < end (not when unordered).
            let start = e.get(this, BGSTerrainNode::fStartMorphDistance);
            let end = e.get(this, BGSTerrainNode::fEndMorphDistance);
            if (start as f64) < end as f64 {
                e.call(PRINT_WARNING, &args![MORPH_DISTANCE_WARNING]);
            }
        }
        e.set(this, BGSTerrainNode::bChildrenLoaded, false);
    });
    this
}

// Translated from 006fd750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node's destructor body (what the scalar deleting destructor
/// `006fca40` calls): unloads everything ([`fn_006fd7b0`] with 7) and
/// deletes each child through `006fca40(child, 1)`.
pub fn fn_006fd750(e: &mut Engine, this: Ptr<BGSTerrainNode>) {
    fn_006fd7b0(e, this, 7);
    for index in 0..4 {
        let node = child(e, this, index);
        if !node.is_null() {
            e.call(NODE_DELETE, &args![node, 1u32]);
        }
    }
}

// Translated from 006fd7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unloads what `flags` selects: bit 0 the chunk (deleted, unless the node
/// is persistent, its manager is the [`fn_006fd8e0`] one and `flags` is not
/// 7: then only `006fb6e0` runs on it; only for a node with geometry), bit
/// 1 the distant object block, bit 2 the tree block. Clears the parent's
/// `bChildrenLoaded`.
pub fn fn_006fd7b0(e: &mut Engine, this: Ptr<BGSTerrainNode>, flags: u32) {
    if e.get(this, BGSTerrainNode::bHasGeometry)
        && !e.get(this, BGSTerrainNode::pChunk).is_null()
        && flags & 1 != 0
    {
        let keep = e.get(this, BGSTerrainNode::bPersistentNode)
            && e.get(this, BGSTerrainNode::pManager).addr() == fn_006fd8e0(e)
            && flags != 7;
        if keep {
            let chunk = e.get(this, BGSTerrainNode::pChunk);
            e.call(CHUNK_CALL_006FB6E0, &args![chunk]);
        } else {
            let chunk = e.get(this, BGSTerrainNode::pChunk);
            if !chunk.is_null() {
                fn_006fd8f0(e, chunk, 1);
            }
            e.set(this, BGSTerrainNode::pChunk, Ptr::NULL);
        }
    }
    let block = e.get(this, BGSTerrainNode::pBlock);
    if !block.is_null() && flags & 2 != 0 {
        fn_006fd920(e, block, 1);
        e.set(this, BGSTerrainNode::pBlock, Ptr::NULL);
    }
    let trees = e.get(this, BGSTerrainNode::pTrees);
    if !trees.is_null() && flags & 4 != 0 {
        fn_006fd950(e, trees, 1);
        e.set(this, BGSTerrainNode::pTrees, Ptr::NULL);
    }
    let parent = e.get(this, BGSTerrainNode::pParent);
    if !parent.is_null() {
        e.set(parent, BGSTerrainNode::bChildrenLoaded, false);
    }
}

// Translated from 006fd8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global word at `011d8670` (the manager [`fn_006fd7b0`] compares a
/// node's manager with).
pub fn fn_006fd8e0(e: &mut Engine) -> u32 {
    e.global(ACTIVE_MANAGER)
}

// Translated from 006fd8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The terrain chunk's scalar deleting destructor: runs the destructor
/// `006fa420` and frees the chunk when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_006fd8f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CHUNK_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006fd920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The distant object block's scalar deleting destructor (destructor
/// `006f5370`). Returns `this`.
pub fn fn_006fd920(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BLOCK_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006fd950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree block's scalar deleting destructor (destructor `006f76b0`).
/// Returns `this`.
pub fn fn_006fd950(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(TREES_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006fd980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unloads `flags` (as [`fn_006fd7b0`]) in this node, when it has a chunk
/// or a block, and in the whole subtree; clears `bChildrenLoaded`. With bit
/// 2, when this node is the manager's `+4` node (its root), also runs
/// `006f83f0`.
pub fn fn_006fd980(e: &mut Engine, this: Ptr<BGSTerrainNode>, flags: u32) {
    if !e.get(this, BGSTerrainNode::pChunk).is_null()
        || !e.get(this, BGSTerrainNode::pBlock).is_null()
    {
        fn_006fd7b0(e, this, flags);
    }
    for index in 0..4 {
        let node = child(e, this, index);
        if !node.is_null() {
            fn_006fd980(e, node, flags);
        }
    }
    e.set(this, BGSTerrainNode::bChildrenLoaded, false);
    if flags & 4 != 0 {
        let manager = e.get(this, BGSTerrainNode::pManager);
        let root = e.call(WORD_AT_4, &args![manager]).u32();
        if this.addr() == root {
            e.call(TREES_CALL_006F83F0, &args![]);
        }
    }
}

// Translated from 006fda10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the chunks of the persistent nodes of the subtree.
pub fn fn_006fda10(e: &mut Engine, this: Ptr<BGSTerrainNode>) {
    let chunk = e.get(this, BGSTerrainNode::pChunk);
    if !chunk.is_null() && e.get(this, BGSTerrainNode::bPersistentNode) {
        fn_006fd8f0(e, chunk, 1);
        e.set(this, BGSTerrainNode::pChunk, Ptr::NULL);
    }
    for index in 0..4 {
        let node = child(e, this, index);
        if !node.is_null() {
            fn_006fda10(e, node);
        }
    }
}

// Translated from 006fdaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainNode::UpdateTerrain` (Xbox PDB): updates the subtree's
/// terrain for the viewer at `position`.
///
/// A node above the manager's `+0x18` level, or in split range
/// ([`fn_006fe550`]), splits: it keeps or makes its chunk only when
/// persistent, updates its children, and once all children are ready
/// ([`fn_006fe920`]) starts their texture morphs and unloads its own chunk.
/// Otherwise a node with geometry makes or refreshes its chunk, unloads its
/// children's data when they were loaded, and, with `update_water`, updates
/// the water visibility of the chunk's two `+0x08`/`+0x0c` objects.
// Translation incomplete: 006fdaa0 - SEH exception unwinding (FS:[0] frame) is unsupported.
pub fn bgs_terrain_node_update_terrain(
    e: &mut Engine,
    this: Ptr<BGSTerrainNode>,
    position: Ptr<NiPoint3>,
    update_water: bool,
) {
    with_scope_guard(e, 0x23, 0x11c, |e| {
        let manager = e.get(this, BGSTerrainNode::pManager);
        let max_level = e.call(WORD_AT_18, &args![manager]).u32();
        let above_max = max_level < e.get(this, BGSTerrainNode::iLevel);
        let in_split_range = fn_006fe550(e, this, position);
        if in_split_range && e.call(BYTE_AT_5C, &args![this]).bool() {
            let chunk = e.get(this, BGSTerrainNode::pChunk);
            if !chunk.is_null() && !e.call(BYTE_AT_29, &args![chunk]).bool() {
                let chunk = e.get(this, BGSTerrainNode::pChunk);
                e.call(CHUNK_CALL_006FACB0, &args![chunk]);
            }
        }
        if above_max || in_split_range {
            e.set(this, BGSTerrainNode::bSplitNode, true);
            if e.call(BYTE_AT_5C, &args![this]).bool()
                && e.get(this, BGSTerrainNode::pChunk).is_null()
            {
                let chunk = new_chunk(e, this);
                e.set(this, BGSTerrainNode::pChunk, chunk);
            }
            for index in 0..4 {
                let node = child(e, this, index);
                if !node.is_null() {
                    bgs_terrain_node_update_terrain(e, node, position, update_water);
                }
            }
            if !e.get(this, BGSTerrainNode::pChunk).is_null() && fn_006fe920(e, this, true) {
                for index in 0..4 {
                    let node = child(e, this, index);
                    let chunk = e.get(node, BGSTerrainNode::pChunk);
                    if !chunk.is_null() && e.call(BYTE_AT_28, &args![chunk]).bool() {
                        let node = child(e, this, index);
                        bgs_terrain_node_start_texture_morph(e, node);
                    }
                }
                fn_006fd7b0(e, this, 1);
            }
            if fn_006fe920(e, this, true) {
                e.set(this, BGSTerrainNode::bChildrenLoaded, true);
            }
        } else if e.get(this, BGSTerrainNode::bHasGeometry) {
            let chunk = e.get(this, BGSTerrainNode::pChunk);
            let pending = if chunk.is_null() {
                0
            } else {
                e.call(CHUNK_PENDING_OBJECT, &args![chunk]).u32()
            };
            e.set(this, BGSTerrainNode::bSplitNode, false);
            if e.get(this, BGSTerrainNode::pChunk).is_null() {
                let chunk = new_chunk(e, this);
                e.set(this, BGSTerrainNode::pChunk, chunk);
            } else {
                refresh_chunk(e, this);
            }
            let chunk = e.get(this, BGSTerrainNode::pChunk);
            if e.call(BYTE_AT_29, &args![chunk]).bool()
                && e.get(this, BGSTerrainNode::bChildrenLoaded)
            {
                for index in 0..4 {
                    let node = child(e, this, index);
                    if !node.is_null() {
                        fn_006fd980(e, node, 1);
                    }
                }
                e.set(this, BGSTerrainNode::bChildrenLoaded, false);
            }
            if pending != 0 {
                let world = manager_world(e, this);
                let tes = e.global::<u32>(TES_POINTER);
                if world != e.call(TES_GET_WORLD_SPACE, &args![tes]).u32() {
                    let tes = e.global::<u32>(TES_POINTER);
                    let current = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
                    if e.call(WORLD_TEST, &args![current, 1u32]).bool()
                        && !e.call(WORLD_TEST, &args![current, 0u32]).bool()
                        && bgs_terrain_node_check_intersects_loaded_range(e, this)
                    {
                        e.call(PENDING_OBJECT_CALL, &args![pending, 1u32]);
                    }
                }
            }
            if update_water && !e.get(this, BGSTerrainNode::pChunk).is_null() {
                let chunk = e.get(this, BGSTerrainNode::pChunk);
                let first = e.call(POINTER_AT_8, &args![chunk]).u32();
                let chunk = e.get(this, BGSTerrainNode::pChunk);
                let second = e.call(POINTER_AT_C, &args![chunk]).u32();
                for object in [first, second] {
                    if object != 0 {
                        let shape = e.vcall(object, 0x28, &args![]).ptr();
                        bgs_terrain_node_update_water_visibility(e, this, shape, 0);
                    }
                }
            }
        }
    });
}

/// The chunk part of [`bgs_terrain_node_update_terrain`] for a node that
/// already has a chunk (inline code there).
fn refresh_chunk(e: &mut Engine, this: Ptr<BGSTerrainNode>) {
    let chunk = e.get(this, BGSTerrainNode::pChunk);
    if e.call(BYTE_AT_28, &args![chunk]).bool()
        && e.call(BYTE_AT_29, &args![chunk]).bool()
        && e.call(BYTE_AT_5C, &args![this]).bool()
    {
        e.call(CHUNK_CALL_006FB760, &args![chunk]);
    }
    if !e.call(BYTE_AT_28, &args![chunk]).bool() || e.call(BYTE_AT_29, &args![chunk]).bool() {
        return;
    }
    let parent = e.get(this, BGSTerrainNode::pParent);
    if !parent.is_null() && !fn_006fe920(e, parent, true) {
        return;
    }
    e.call(CHUNK_CALL_006FACB0, &args![chunk]);
    let level = e.get(this, BGSTerrainNode::iLevel) as u64 as f64;
    let level_quarter = level / e.global::<f64>(FOUR);
    let scale = setting_float(e, DETAIL_TEXTURE_SCALE);
    let value = (scale as f64 * level_quarter) as f32;
    let geometry = e.call(POINTER_AT_2C, &args![chunk]).u32();
    e.call(GEOMETRY_SET_SCALE, &args![geometry, x87_quiet(value)]);
    let manager = e.call(WORD_AT_0, &args![this]).u32();
    let root = e.call(POINTER_AT_8, &args![manager]).u32();
    e.call(SCENE_NODE_FLAG, &args![root, 0u32]);
    e.vcall(root, 0xbc, &args![]);
    e.call(SCENE_NODE_FLAG, &args![root, 1u32]);
}

// Translated from 006fdfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the distant object blocks of the subtree for the viewer at
/// `position`: at the manager's block level (`+0x24`) a node in block range
/// ([`fn_006fe620`]) makes its block (low detail per [`fn_006fe6d0`]) or
/// refreshes it (and, with `update_blocks`, its visibility), switches it
/// between the two detail calls (`006f6370` / `006f6540`) and runs
/// `006f5af0` when its `+0x24` byte is set; out of range it deletes it.
/// Above the block level the children are updated.
// Translation incomplete: 006fdfc0 - SEH exception unwinding (FS:[0] frame) is unsupported.
pub fn fn_006fdfc0(
    e: &mut Engine,
    this: Ptr<BGSTerrainNode>,
    position: Ptr<NiPoint3>,
    update_blocks: bool,
) {
    with_scope_guard(e, 0x24, 0x1ae, |e| {
        let level = e.call(WORD_AT_4, &args![this]).u32();
        let manager = e.get(this, BGSTerrainNode::pManager);
        let block_level = e.call(WORD_AT_24, &args![manager]).u32();
        if level == block_level {
            if fn_006fe620(e, this, position) {
                let low = fn_006fe6d0(e, this, position);
                let block = e.get(this, BGSTerrainNode::pBlock);
                if block.is_null() {
                    let storage = e.call(ALLOCATE, &args![0x2cu32]).u32();
                    let block = if storage == 0 {
                        Ptr::NULL
                    } else {
                        let level = e.get(this, BGSTerrainNode::iLevel);
                        let x = e.get(this, BGSTerrainNode::sBaseCellX) as u16 as u32;
                        let y = e.get(this, BGSTerrainNode::sBaseCellY) as u16 as u32;
                        e.call(
                            BLOCK_CONSTRUCT,
                            &args![storage, this, level, x, y, low as u32],
                        )
                        .ptr()
                    };
                    e.set(this, BGSTerrainNode::pBlock, block);
                } else if fn_006fe280(e, block, false) != 0 {
                    let block = e.get(this, BGSTerrainNode::pBlock);
                    if e.call(BYTE_AT_20, &args![block]).bool() {
                        e.call(BLOCK_CALL_006F5CA0, &args![block]);
                    }
                    if update_blocks {
                        bgs_terrain_node_update_block_visibility(e, this, 0);
                    }
                }
                let block = e.get(this, BGSTerrainNode::pBlock);
                if !block.is_null() {
                    let level = e.get(this, BGSTerrainNode::iLevel);
                    let x = e.get(this, BGSTerrainNode::sBaseCellX) as u16 as u32;
                    let y = e.get(this, BGSTerrainNode::sBaseCellY) as u16 as u32;
                    if e.call(BYTE_AT_23, &args![block]).bool() && low {
                        let world = manager_world(e, this);
                        e.call(BLOCK_CALL_006F6370, &args![block, world, level, x, y]);
                    } else if !e.call(BYTE_AT_23, &args![block]).bool() && !low {
                        let world = manager_world(e, this);
                        e.call(BLOCK_CALL_006F6540, &args![block, world, level, x, y]);
                    }
                }
                let block = e.get(this, BGSTerrainNode::pBlock);
                if e.call(BYTE_AT_24, &args![block]).bool() {
                    e.call(BLOCK_CALL_006F5AF0, &args![block]);
                }
            } else {
                let block = e.get(this, BGSTerrainNode::pBlock);
                if !block.is_null() {
                    fn_006fd920(e, block, 1);
                    e.set(this, BGSTerrainNode::pBlock, Ptr::NULL);
                }
            }
        } else {
            let level = e.call(WORD_AT_4, &args![this]).u32();
            let block_level = e.call(WORD_AT_24, &args![manager]).u32();
            if level > block_level {
                for index in 0..4 {
                    let node = child(e, this, index);
                    if !node.is_null() {
                        fn_006fdfc0(e, node, position, update_blocks);
                    }
                }
            }
        }
    });
}

// Translated from 006fe280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object of the smart pointer at `this + 4`; with `use_empty` set, of
/// a temporary smart pointer built empty on the stack instead (then
/// destroyed).
// Translation incomplete: 006fe280 - SEH exception unwinding (FS:[0] frame) is unsupported.
pub fn fn_006fe280(e: &mut Engine, this: Ptr, use_empty: bool) -> u32 {
    if use_empty {
        e.with_stack(4, |e, local| {
            let pointer = e.call(POINTER_CONSTRUCT, &args![local, 0u32]).u32();
            let object = e.call(WORD_AT_0, &args![pointer]).u32();
            e.call(POINTER_DESTRUCT, &args![local]);
            object
        })
    } else {
        e.call(WORD_AT_0, &args![this.addr() + 4]).u32()
    }
}

// Translated from 006fe330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainNode::UpdateTrees` (Xbox PDB): at the tree level
/// ([`fn_006fe510`]) allocates the tree blocks' static data when needed
/// ([`fn_006fe540`]), then makes the node's tree block when in tree range
/// ([`fn_006fe780`]), refreshes it when its `+0x48` byte is set, or deletes
/// it out of range. Above the tree level the children are updated.
// Translation incomplete: 006fe330 - SEH exception unwinding (FS:[0] frame) is unsupported.
pub fn bgs_terrain_node_update_trees(
    e: &mut Engine,
    this: Ptr<BGSTerrainNode>,
    position: Ptr<NiPoint3>,
) {
    with_scope_guard(e, 0x25, 0x1ed, |e| {
        let level = e.call(WORD_AT_4, &args![this]).u32();
        if level == fn_006fe510(e) {
            if !fn_006fe540(e) {
                let world = manager_world(e, this);
                e.call(TREES_ALLOCATE_STATIC_DATA, &args![world]);
            }
            if fn_006fe780(e, this, position) {
                let trees = e.get(this, BGSTerrainNode::pTrees);
                if trees.is_null() {
                    let storage = e.call(ALLOCATE, &args![0x4cu32]).u32();
                    let trees = if storage == 0 {
                        Ptr::NULL
                    } else {
                        let level = e.get(this, BGSTerrainNode::iLevel);
                        let x = e.get(this, BGSTerrainNode::sBaseCellX) as u16 as u32;
                        let y = e.get(this, BGSTerrainNode::sBaseCellY) as u16 as u32;
                        e.call(TREES_CONSTRUCT, &args![storage, this, level, x, y])
                            .ptr()
                    };
                    e.set(this, BGSTerrainNode::pTrees, trees);
                } else if fn_006fe520(e, trees) {
                    e.call(TREES_CALL_006F8C40, &args![trees]);
                    e.call(TREES_UPDATE_SHADER_PROPERTIES, &args![trees]);
                }
            } else {
                let trees = e.get(this, BGSTerrainNode::pTrees);
                if !trees.is_null() {
                    fn_006fd950(e, trees, 1);
                    e.set(this, BGSTerrainNode::pTrees, Ptr::NULL);
                }
            }
        } else {
            let level = e.call(WORD_AT_4, &args![this]).u32();
            if level > fn_006fe510(e) {
                for index in 0..4 {
                    let node = child(e, this, index);
                    if !node.is_null() {
                        bgs_terrain_node_update_trees(e, node, position);
                    }
                }
            }
        }
    });
}

// Translated from 006fe510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The level at which nodes carry tree blocks: 8.
pub fn fn_006fe510(_e: &mut Engine) -> u32 {
    TREE_LEVEL
}

// Translated from 006fe520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x48` (called on a tree block).
pub fn fn_006fe520(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x48) != 0
}

// Translated from 006fe540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `011d85a0` (set once the tree blocks' static data is
/// allocated).
pub fn fn_006fe540(e: &mut Engine) -> bool {
    e.global::<u8>(TREE_STATIC_DATA_FLAG) != 0
}

// Translated from 006fe550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the node should split for the viewer at `position`: never at the
/// manager's `+0x1c` level, always when it intersects the loaded range,
/// otherwise when its split distance is greater than the distance to its
/// square.
pub fn fn_006fe550(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) -> bool {
    let manager = e.get(this, BGSTerrainNode::pManager);
    let min_level = e.call(WORD_AT_1C, &args![manager]).u32();
    if e.get(this, BGSTerrainNode::iLevel) == min_level {
        return false;
    }
    if bgs_terrain_node_check_intersects_loaded_range(e, this) {
        return true;
    }
    let distance = distance_to_square(e, this, position);
    // `FCOMPP` of the split distance against the distance, `TEST AH,41h`:
    // true only when greater (not when unordered).
    let split = e.get(this, BGSTerrainNode::fSplitDistance);
    split as f64 > distance as f64
}

/// Whether the setting's float is greater than the distance from
/// `position` to the node's square (`TEST AH,41h`: false when unordered).
fn setting_above_distance(
    e: &mut Engine,
    this: Ptr<BGSTerrainNode>,
    position: Ptr<NiPoint3>,
    setting: u32,
) -> bool {
    let distance = distance_to_square(e, this, position);
    let limit = setting_float(e, setting);
    limit as f64 > distance as f64
}

// Translated from 006fe620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the node's square is within `fBlockLoadDistance` of `position`
/// (strictly).
pub fn fn_006fe620(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) -> bool {
    setting_above_distance(e, this, position, BLOCK_LOAD_DISTANCE)
}

// Translated from 006fe6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the node's square is within `fBlockLoadDistanceLow` of
/// `position` (strictly).
pub fn fn_006fe6d0(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) -> bool {
    setting_above_distance(e, this, position, BLOCK_LOAD_DISTANCE_LOW)
}

// Translated from 006fe780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the node's square is within `fTreeLoadDistance` of `position`
/// (strictly).
pub fn fn_006fe780(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) -> bool {
    setting_above_distance(e, this, position, TREE_LOAD_DISTANCE)
}

// Translated from 006fe830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The distance from the point (`point_x`, `point_y`) to the square of side
/// `size` centered on (`center_x`, `center_y`): the corner is the center
/// minus half the size (`004e8880`), each axis contributes how far the
/// point lies outside the square on it, and the result is the square root
/// (`004019d0`) of the sum of squares, in `ST0`. `this` (the node) is not
/// read.
pub fn fn_006fe830(
    e: &mut Engine,
    _node: Ptr<BGSTerrainNode>,
    center_x: f32,
    center_y: f32,
    size: f32,
    point_x: f32,
    point_y: f32,
) -> f64 {
    let one_half: f64 = e.global(ONE_HALF);
    let size = x87_quiet(size);
    let half = (size as f64 * one_half) as f32;
    let (corner_x, corner_y) = e.with_stack(0x18, |e, locals| {
        let center = locals.addr();
        let halves = center + 8;
        let corner = center + 0x10;
        e.mem.set_f32(center, center_x);
        e.mem.set_f32(center + 4, center_y);
        let other = e
            .call(POINT2_SET, &args![halves, x87_quiet(half), x87_quiet(half)])
            .u32();
        e.call(POINT2_SUBTRACT, &args![center, corner, other]);
        (e.mem.f32(corner), e.mem.f32(corner + 4))
    });
    let along = |corner: f32, point: f32| -> f32 {
        let (corner, point) = (x87_quiet(corner) as f64, x87_quiet(point) as f64);
        // `TEST AH,5` / `JP`: corner >= point, or unordered.
        if corner.partial_cmp(&point) != Some(std::cmp::Ordering::Less) {
            (corner - point) as f32
        } else {
            let far = corner + size as f64;
            // `TEST AH,41h`: only when the point is strictly beyond.
            if point > far {
                (point - far) as f32
            } else {
                0.0
            }
        }
    };
    let dx = along(corner_x, point_x);
    let dy = along(corner_y, point_y);
    let sum = (dx as f64 * dx as f64 + dy as f64 * dy as f64) as f32;
    e.call(SQUARE_ROOT, &args![x87_quiet(sum)]).f64()
}

// Translated from 006fe920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the node's children are ready ([`fn_006fea10`] on all four), or
/// the node lies outside cells -40..=40 in x or -44..=44 in y (its
/// `+0x08`/`+0x0a` words, through their getters). Never at the manager's
/// `+0x1c` level.
pub fn fn_006fe920(e: &mut Engine, this: Ptr<BGSTerrainNode>, flag: bool) -> bool {
    let manager = e.get(this, BGSTerrainNode::pManager);
    let min_level = e.call(WORD_AT_1C, &args![manager]).u32();
    if e.get(this, BGSTerrainNode::iLevel) == min_level {
        return false;
    }
    let mut ready = true;
    for index in 0..4 {
        let node = child(e, this, index);
        if !fn_006fea10(e, node, flag) {
            ready = false;
            break;
        }
    }
    let x = e.call(SHORT_AT_8, &args![this]).u16() as i16 as i32;
    if x > 0x28 {
        return true;
    }
    let x = e.call(SHORT_AT_8, &args![this]).u16() as i16 as i32;
    if x < -0x28 {
        return true;
    }
    let y = e.call(SHORT_AT_A, &args![this]).u16() as i16 as i32;
    if y > 0x2c {
        return true;
    }
    let y = e.call(SHORT_AT_A, &args![this]).u16() as i16 as i32;
    if y < -0x2c {
        return true;
    }
    ready
}

// Translated from 006fea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the node is ready: its chunk's `+0x29` or `+0x28` byte is set,
/// or else [`fn_006fe920`] holds for it.
pub fn fn_006fea10(e: &mut Engine, this: Ptr<BGSTerrainNode>, flag: bool) -> bool {
    let chunk = e.get(this, BGSTerrainNode::pChunk);
    if !chunk.is_null() && e.call(BYTE_AT_29, &args![chunk]).bool() {
        return true;
    }
    let chunk = e.get(this, BGSTerrainNode::pChunk);
    if !chunk.is_null() && e.call(BYTE_AT_28, &args![chunk]).bool() {
        return true;
    }
    fn_006fe920(e, this, flag)
}

// Translated from 006fea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the morph factor ([`fn_006feb20`]) of the subtree's ready chunks
/// for the viewer at `position` and advances their texture fades; does not
/// descend below a ready, split, non-persistent node.
pub fn fn_006fea70(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) {
    let chunk = e.get(this, BGSTerrainNode::pChunk);
    if !chunk.is_null() && e.call(BYTE_AT_28, &args![chunk]).bool() {
        let factor = fn_006feb20(e, this, position);
        let chunk = e.get(this, BGSTerrainNode::pChunk);
        e.call(CHUNK_SET_MORPH, &args![chunk, factor]);
        if e.get(this, BGSTerrainNode::bTexFading) {
            fn_006ff3f0(e, this);
        }
        if !e.call(BYTE_AT_5C, &args![this]).bool() && e.get(this, BGSTerrainNode::bSplitNode) {
            return;
        }
    }
    if !child(e, this, 0).is_null() {
        for index in 0..4 {
            let node = child(e, this, index);
            fn_006fea70(e, node, position);
        }
    }
}

// Translated from 006feb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The morph factor for the viewer at `position`: 1 closer than the end
/// morph distance, 0 farther than the start morph distance, and in between
/// `1 - (distance - end) / (start - end)` (stored as a `float`).
pub fn fn_006feb20(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) -> f32 {
    let distance = distance_to_square(e, this, position) as f64;
    let end = e.get(this, BGSTerrainNode::fEndMorphDistance) as f64;
    // `TEST AH,41h`: 1 only when end > distance.
    if end > distance {
        return 1.0;
    }
    let start = e.get(this, BGSTerrainNode::fStartMorphDistance) as f64;
    // `TEST AH,5` / `JP`: 0 only when start < distance.
    if start < distance {
        return 0.0;
    }
    (1.0 - (distance - end) / (start - end)) as f32
}

// Translated from 006fec00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `position` lies within the node's square: half the side
/// (`iLevel * 2048`) is not less than the absolute offset (`00408840`) from
/// the center on y and then on x (an unordered comparison counts as
/// within).
pub fn fn_006fec00(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) -> bool {
    let level = e.get(this, BGSTerrainNode::iLevel) as u64 as f64;
    let half = (level * e.global::<f64>(HALF_CELL)) as f32 as f64;
    let center = this.at(BGSTerrainNode::Center);
    for (point_offset, center_field) in [(4, NiPoint3::y), (0, NiPoint3::x)] {
        let point = e.mem.f32(position.addr() + point_offset) as f64;
        let offset = (point - e.get(center, center_field) as f64) as f32;
        let distance = e.call(ABSOLUTE, &args![x87_quiet(offset)]).f64();
        // `TEST AH,5` / `JP`: outside only when half < distance.
        if half < distance {
            return false;
        }
    }
    true
}

// Translated from 006fec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The chunk drawn at `position`: the first node of the subtree containing
/// it ([`fn_006fec00`]) whose chunk has its `+0x28` byte set and which is
/// not split; null when none.
pub fn fn_006fec90(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) -> Ptr {
    let mut found = Ptr::NULL;
    if fn_006fec00(e, this, position) {
        let chunk = e.get(this, BGSTerrainNode::pChunk);
        if !chunk.is_null()
            && e.call(BYTE_AT_28, &args![chunk]).bool()
            && !e.get(this, BGSTerrainNode::bSplitNode)
        {
            found = e.get(this, BGSTerrainNode::pChunk);
        } else if !child(e, this, 0).is_null() {
            for index in 0..4 {
                let node = child(e, this, index);
                found = fn_006fec90(e, node, position);
                if !found.is_null() {
                    break;
                }
            }
        }
    }
    found
}

// Translated from 006fed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree block at `position`: the tree block of the tree-level node of
/// the subtree containing it; null when none.
pub fn fn_006fed30(e: &mut Engine, this: Ptr<BGSTerrainNode>, position: Ptr<NiPoint3>) -> Ptr {
    let mut found = Ptr::NULL;
    if fn_006fec00(e, this, position) {
        let level = e.call(WORD_AT_4, &args![this]).u32();
        if level == fn_006fe510(e) && !e.get(this, BGSTerrainNode::pTrees).is_null() {
            found = e.get(this, BGSTerrainNode::pTrees);
        } else {
            let level = e.call(WORD_AT_4, &args![this]).u32();
            if level > fn_006fe510(e) && !child(e, this, 0).is_null() {
                for index in 0..4 {
                    let node = child(e, this, index);
                    found = fn_006fed30(e, node, position);
                    if !found.is_null() {
                        break;
                    }
                }
            }
        }
    }
    found
}

// Translated from 006fede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainNode::CheckIntersectsLoadedRange` (Xbox PDB): whether the
/// node's cells overlap the loaded grid: the grid starts `uGridsToLoad / 2`
/// before the cell at the `TES` object's `+0x24`/`+0x28` words and spans
/// `uGridsToLoad` cells (signed comparisons, inclusive).
pub fn bgs_terrain_node_check_intersects_loaded_range(
    e: &mut Engine,
    this: Ptr<BGSTerrainNode>,
) -> bool {
    let tes = e.global::<u32>(TES_POINTER);
    let cell_x = e.call(WORD_AT_24, &args![tes]).u32();
    let grids = setting_unsigned(e, GRIDS_TO_LOAD);
    let start_x = cell_x.wrapping_sub(grids >> 1) as i32;
    let tes = e.global::<u32>(TES_POINTER);
    let cell_y = e.call(WORD_AT_28, &args![tes]).u32();
    let grids = setting_unsigned(e, GRIDS_TO_LOAD);
    let start_y = cell_y.wrapping_sub(grids >> 1) as i32;
    let level = e.get(this, BGSTerrainNode::iLevel) as i32;
    let x = e.get(this, BGSTerrainNode::sBaseCellX) as i32;
    let y = e.get(this, BGSTerrainNode::sBaseCellY) as i32;
    let grids = setting_unsigned(e, GRIDS_TO_LOAD) as i32;
    if start_x.wrapping_add(grids) < x || start_x > x.wrapping_add(level) {
        return false;
    }
    let grids = setting_unsigned(e, GRIDS_TO_LOAD) as i32;
    !(start_y.wrapping_add(grids) < y || start_y > y.wrapping_add(level))
}

/// The segment loop of [`bgs_terrain_node_update_block_visibility`] and
/// [`bgs_terrain_node_update_water_visibility`]: one segment per cell of
/// the node (row-major, `iLevel` per row, only those below the shape's
/// segment count); a segment whose `+0xc4` entry is empty ([`fn_006ff0a0`])
/// over a loaded cell (and, with `fading_check`, a cell whose 3D is all
/// visible and fading in) is disabled, every other one enabled; then the
/// enabled segments are merged.
fn update_segments(
    e: &mut Engine,
    this: Ptr<BGSTerrainNode>,
    shape: Ptr,
    world: u32,
    fading_check: bool,
) {
    let mut row = 0u32;
    while row < e.get(this, BGSTerrainNode::iLevel) {
        let mut column = 0u32;
        while column < e.get(this, BGSTerrainNode::iLevel) {
            let level = e.get(this, BGSTerrainNode::iLevel);
            let index = level.wrapping_mul(row).wrapping_add(column);
            let segments = e.call(SHAPE_GET_NUM_SEGMENTS, &args![shape]).u32();
            if index < segments {
                let x = (e.get(this, BGSTerrainNode::sBaseCellX) as i32).wrapping_add(row as i32);
                let y =
                    (e.get(this, BGSTerrainNode::sBaseCellY) as i32).wrapping_add(column as i32);
                let cell = e.call(WORLD_GET_CELL, &args![world, x, y]).u32();
                let disable = if fn_006ff0a0(e, shape, index as i32) {
                    false
                } else {
                    let tes = e.global::<u32>(TES_POINTER);
                    e.call(TES_IS_CELL_LOADED, &args![tes, cell, 1u32]).bool()
                        && (!fading_check || e.call(CELL_ALL_3D_FADING_IN, &args![cell]).bool())
                };
                if disable {
                    e.call(SHAPE_DISABLE_SEGMENT, &args![shape, index]);
                } else {
                    e.call(SHAPE_ENABLE_SEGMENT, &args![shape, index]);
                }
            }
            column += 1;
        }
        row += 1;
    }
    e.call(SHAPE_MERGE_ENABLED_SEGMENTS, &args![shape]);
}

// Translated from 006fee90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainNode::UpdateBlockVisibility` (Xbox PDB): when the node's
/// block has an object (`fn_006fe280(block, false)`), hides the segments of
/// its shape (the object's virtual at `+0x28`) over loaded cells whose 3D
/// is fading in, inside the loaded range, or enables them all outside it;
/// then updates the block's stingers (`UpdateStingerVisibility` inside the
/// range, `006f5a70` outside). The stack word is not read.
pub fn bgs_terrain_node_update_block_visibility(
    e: &mut Engine,
    this: Ptr<BGSTerrainNode>,
    _unused_1: u32,
) {
    let tes = e.global::<u32>(TES_POINTER);
    let world = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
    let block = e.get(this, BGSTerrainNode::pBlock);
    if !block.is_null() && fn_006fe280(e, block, false) != 0 {
        let block = e.get(this, BGSTerrainNode::pBlock);
        let object = fn_006fe280(e, block, false);
        let shape: Ptr = e.vcall(object, 0x28, &args![]).ptr();
        if bgs_terrain_node_check_intersects_loaded_range(e, this) {
            if !shape.is_null() {
                update_segments(e, this, shape, world, true);
            }
        } else if !shape.is_null() {
            e.call(SHAPE_ENABLE_ALL_SEGMENTS, &args![shape]);
        }
    }
    let block = e.get(this, BGSTerrainNode::pBlock);
    if !block.is_null() && e.call(BLOCK_OBJECTS, &args![block, 1u32]).u32() != 0 {
        if bgs_terrain_node_check_intersects_loaded_range(e, this) {
            let level = e.get(this, BGSTerrainNode::iLevel);
            let x = e.get(this, BGSTerrainNode::sBaseCellX) as u16 as u32;
            let y = e.get(this, BGSTerrainNode::sBaseCellY) as u16 as u32;
            let block = e.get(this, BGSTerrainNode::pBlock);
            let objects = e.call(BLOCK_OBJECTS, &args![block, 1u32]).u32();
            e.call(
                BLOCK_UPDATE_STINGER_VISIBILITY,
                &args![block, objects, level, x, y],
            );
        } else {
            let block = e.get(this, BGSTerrainNode::pBlock);
            let objects = e.call(BLOCK_OBJECTS, &args![block, 1u32]).u32();
            e.call(BLOCK_CALL_006F5A70, &args![block, objects]);
        }
    }
}

// Translated from 006ff0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether entry `index` of the 0x14-byte array at `this + 0xc4` (a
/// segmented shape's segments) has a zero word at `+4`.
pub fn fn_006ff0a0(e: &mut Engine, this: Ptr, index: i32) -> bool {
    let entries = e.mem.u32(this.addr() + 0xc4);
    let entry = entries.wrapping_add((index as u32).wrapping_mul(0x14));
    e.mem.u32(entry.wrapping_add(4)) == 0
}

// Translated from 006ff0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainNode::UpdateWaterVisibility` (Xbox PDB): with a `shape`,
/// hides its segments over loaded cells inside the loaded range, or enables
/// them all outside it. The second stack word is not read.
pub fn bgs_terrain_node_update_water_visibility(
    e: &mut Engine,
    this: Ptr<BGSTerrainNode>,
    shape: Ptr,
    _unused_2: u32,
) {
    let tes = e.global::<u32>(TES_POINTER);
    let world = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
    if shape.is_null() {
        return;
    }
    if bgs_terrain_node_check_intersects_loaded_range(e, this) {
        update_segments(e, this, shape, world, false);
    } else {
        e.call(SHAPE_ENABLE_ALL_SEGMENTS, &args![shape]);
    }
}

// Translated from 006ff210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The texture offset of the node's quarter of its parent, written to
/// `out` (two floats through `00452dc0`): (0.5, 0) for position 1, (0, 0)
/// for 2, (0.5, 0.5) for 3, (0, 0.5) for 4, (0, 0) otherwise. Returns
/// `out`.
pub fn fn_006ff210(e: &mut Engine, this: Ptr<BGSTerrainNode>, out: Ptr) -> Ptr {
    let half = x87_quiet(e.global::<f32>(ONE_HALF_FLOAT));
    // The switch's jump table (`006ff2a8`): positions 1, 2, 3, 4.
    let (x, y) = match e.get(this, BGSTerrainNode::ParentPosition) {
        1 => (half, 0.0),
        2 => (0.0, 0.0),
        3 => (half, half),
        4 => (0.0, half),
        _ => (0.0, 0.0),
    };
    e.call(POINT2_SET, &args![out, x87_quiet(x), x87_quiet(y)]);
    out
}

// Translated from 006ff2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainNode::StartTextureMorph` (Xbox PDB): gives the chunk's
/// geometry (`+0x2c`) the parent chunk's textures 0 and 1 (its virtual at
/// `+0xf0`), starts the fade (the time at `011f6394 + 0x14`), sets the
/// geometry's morph to the quarter's offset ([`fn_006ff210`]) and 0, and
/// marks the node fading. Nothing happens without geometry.
pub fn bgs_terrain_node_start_texture_morph(e: &mut Engine, this: Ptr<BGSTerrainNode>) {
    let parent = e.get(this, BGSTerrainNode::pParent);
    let parent_chunk = e.get(parent, BGSTerrainNode::pChunk);
    let first = e.call(CHUNK_GET_TEXTURE, &args![parent_chunk, 0u32]).u32();
    let parent = e.get(this, BGSTerrainNode::pParent);
    let parent_chunk = e.get(parent, BGSTerrainNode::pChunk);
    let second = e.call(CHUNK_GET_TEXTURE, &args![parent_chunk, 1u32]).u32();
    let chunk = e.get(this, BGSTerrainNode::pChunk);
    let geometry = e.call(POINTER_AT_2C, &args![chunk]).u32();
    if geometry == 0 {
        return;
    }
    e.vcall(geometry, 0xf0, &args![0u32, 1u32, first]);
    e.vcall(geometry, 0xf0, &args![1u32, 1u32, second]);
    e.with_stack(8, |e, offset| {
        fn_006ff210(e, this, offset);
        let now = e.call(WORD_AT_14, &args![TIMER_OBJECT]).u32();
        e.set(this, BGSTerrainNode::uiFadeStartTime, now);
        let x = x87_quiet(e.mem.f32(offset.addr()));
        let y = x87_quiet(e.mem.f32(offset.addr() + 4));
        e.call(GEOMETRY_SET_MORPH, &args![geometry, x, y, 0.0f32]);
    });
    e.set(this, BGSTerrainNode::bTexFading, true);
}

// Translated from 006ff3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances the texture fade: once `uTerrainTextureFadeTime` has passed
/// since the start (unsigned), ends it (`006ff380`); before that sets the
/// geometry's morph to the quarter's offset and the elapsed fraction.
/// Nothing happens without a chunk or geometry.
pub fn fn_006ff3f0(e: &mut Engine, this: Ptr<BGSTerrainNode>) {
    let now = e.call(WORD_AT_14, &args![TIMER_OBJECT]).u32();
    let elapsed = now.wrapping_sub(e.get(this, BGSTerrainNode::uiFadeStartTime));
    let chunk = e.get(this, BGSTerrainNode::pChunk);
    if chunk.is_null() || e.call(POINTER_AT_2C, &args![chunk]).u32() == 0 {
        return;
    }
    if elapsed > setting_unsigned(e, TEXTURE_FADE_TIME) {
        e.call(END_TEXTURE_MORPH, &args![this]);
        return;
    }
    let fade_time = setting_unsigned(e, TEXTURE_FADE_TIME);
    let fraction = (elapsed as f64 / fade_time as f64) as f32;
    e.with_stack(8, |e, offset| {
        fn_006ff210(e, this, offset);
        let x = x87_quiet(e.mem.f32(offset.addr()));
        let y = x87_quiet(e.mem.f32(offset.addr() + 4));
        let chunk = e.get(this, BGSTerrainNode::pChunk);
        let geometry = e.call(POINTER_AT_2C, &args![chunk]).u32();
        e.call(
            GEOMETRY_SET_MORPH,
            &args![geometry, x, y, x87_quiet(fraction)],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006fd210,
            bgs_terrain_node_constructor(
                Ptr<BGSTerrainNode>,
                Ptr,
                Ptr<BGSTerrainNode>,
                i16,
                i16,
                u32,
                bool,
            ) -> Ptr<BGSTerrainNode>
        ),
        entry!(0x006fd750, fn_006fd750(Ptr<BGSTerrainNode>)),
        entry!(0x006fd7b0, fn_006fd7b0(Ptr<BGSTerrainNode>, u32)),
        entry!(0x006fd8e0, fn_006fd8e0() -> u32),
        entry!(0x006fd8f0, fn_006fd8f0(Ptr, u32) -> Ptr),
        entry!(0x006fd920, fn_006fd920(Ptr, u32) -> Ptr),
        entry!(0x006fd950, fn_006fd950(Ptr, u32) -> Ptr),
        entry!(0x006fd980, fn_006fd980(Ptr<BGSTerrainNode>, u32)),
        entry!(0x006fda10, fn_006fda10(Ptr<BGSTerrainNode>)),
        entry!(
            0x006fdaa0,
            bgs_terrain_node_update_terrain(Ptr<BGSTerrainNode>, Ptr<NiPoint3>, bool)
        ),
        entry!(
            0x006fdfc0,
            fn_006fdfc0(Ptr<BGSTerrainNode>, Ptr<NiPoint3>, bool)
        ),
        entry!(0x006fe280, fn_006fe280(Ptr, bool) -> u32),
        entry!(
            0x006fe330,
            bgs_terrain_node_update_trees(Ptr<BGSTerrainNode>, Ptr<NiPoint3>)
        ),
        entry!(0x006fe510, fn_006fe510() -> u32),
        entry!(0x006fe520, fn_006fe520(Ptr) -> bool),
        entry!(0x006fe540, fn_006fe540() -> bool),
        entry!(
            0x006fe550,
            fn_006fe550(Ptr<BGSTerrainNode>, Ptr<NiPoint3>) -> bool
        ),
        entry!(
            0x006fe620,
            fn_006fe620(Ptr<BGSTerrainNode>, Ptr<NiPoint3>) -> bool
        ),
        entry!(
            0x006fe6d0,
            fn_006fe6d0(Ptr<BGSTerrainNode>, Ptr<NiPoint3>) -> bool
        ),
        entry!(
            0x006fe780,
            fn_006fe780(Ptr<BGSTerrainNode>, Ptr<NiPoint3>) -> bool
        ),
        entry!(
            0x006fe830,
            fn_006fe830(Ptr<BGSTerrainNode>, f32, f32, f32, f32, f32) -> f64
        ),
        entry!(0x006fe920, fn_006fe920(Ptr<BGSTerrainNode>, bool) -> bool),
        entry!(0x006fea10, fn_006fea10(Ptr<BGSTerrainNode>, bool) -> bool),
        entry!(0x006fea70, fn_006fea70(Ptr<BGSTerrainNode>, Ptr<NiPoint3>)),
        entry!(
            0x006feb20,
            fn_006feb20(Ptr<BGSTerrainNode>, Ptr<NiPoint3>) -> f32
        ),
        entry!(
            0x006fec00,
            fn_006fec00(Ptr<BGSTerrainNode>, Ptr<NiPoint3>) -> bool
        ),
        entry!(
            0x006fec90,
            fn_006fec90(Ptr<BGSTerrainNode>, Ptr<NiPoint3>) -> Ptr
        ),
        entry!(
            0x006fed30,
            fn_006fed30(Ptr<BGSTerrainNode>, Ptr<NiPoint3>) -> Ptr
        ),
        entry!(
            0x006fede0,
            bgs_terrain_node_check_intersects_loaded_range(Ptr<BGSTerrainNode>) -> bool
        ),
        entry!(
            0x006fee90,
            bgs_terrain_node_update_block_visibility(Ptr<BGSTerrainNode>, u32)
        ),
        entry!(0x006ff0a0, fn_006ff0a0(Ptr, i32) -> bool),
        entry!(
            0x006ff0d0,
            bgs_terrain_node_update_water_visibility(Ptr<BGSTerrainNode>, Ptr, u32)
        ),
        entry!(0x006ff210, fn_006ff210(Ptr<BGSTerrainNode>, Ptr) -> Ptr),
        entry!(
            0x006ff2c0,
            bgs_terrain_node_start_texture_morph(Ptr<BGSTerrainNode>)
        ),
        entry!(0x006ff3f0, fn_006ff3f0(Ptr<BGSTerrainNode>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn word_at<const OFFSET: u32>(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0] + OFFSET))
    }

    fn byte_at<const OFFSET: u32>(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u8(a[0] + OFFSET) as u32)
    }

    fn short_at<const OFFSET: u32>(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u16(a[0] + OFFSET) as u32)
    }

    fn returns_this(_: &mut Engine, a: &[u32]) -> Ret {
        ret(a[0])
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The calls made to `addr`, with their argument words.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The test world: the engine, the manager and the `TES` object.
    struct World {
        e: Engine,
        manager: Ptr,
        tes: u32,
    }

    /// Doubles for every callee outside this file (getters read the fields
    /// they read in the exe; the rest do nothing and return 0 or `this`),
    /// the exe's constants and settings, a manager (world `0x7000`, max
    /// level 2, min level 1, block level 4) and a `TES` object whose
    /// current cell is far away.
    fn world() -> World {
        let mut e = Engine::new();
        e.register(WORD_AT_0, word_at::<0>);
        e.register(WORD_AT_4, word_at::<4>);
        e.register(WORD_AT_14, word_at::<0x14>);
        e.register(WORD_AT_18, word_at::<0x18>);
        e.register(WORD_AT_1C, word_at::<0x1c>);
        e.register(WORD_AT_24, word_at::<0x24>);
        e.register(WORD_AT_28, word_at::<0x28>);
        e.register(TES_GET_WORLD_SPACE, word_at::<0x88>);
        e.register(POINTER_AT_2C, word_at::<0x2c>);
        e.register(POINTER_AT_8, word_at::<8>);
        e.register(POINTER_AT_C, word_at::<0xc>);
        e.register(BYTE_AT_20, byte_at::<0x20>);
        e.register(BYTE_AT_23, byte_at::<0x23>);
        e.register(BYTE_AT_24, byte_at::<0x24>);
        e.register(BYTE_AT_28, byte_at::<0x28>);
        e.register(BYTE_AT_29, byte_at::<0x29>);
        e.register(BYTE_AT_5C, byte_at::<0x5c>);
        e.register(SHORT_AT_8, short_at::<8>);
        e.register(SHORT_AT_A, short_at::<0xa>);
        for getter in [SETTING_FLOAT, SETTING_UNSIGNED, SETTING_BYTE] {
            e.register(getter, |_, a| ret(a[0] + 4));
        }
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(POINT2_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        e.register(POINT2_SUBTRACT, |e, a| {
            for offset in [0, 4] {
                let value = e.mem.f32(a[0] + offset) - e.mem.f32(a[2] + offset);
                e.mem.set_f32(a[1] + offset, value);
            }
            ret(a[1])
        });
        e.register(POINT3_SET, |e, a| {
            for (i, word) in a[1..4].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, *word);
            }
            ret(a[0])
        });
        e.register(POINT3_SUBTRACT, |e, a| {
            for offset in [0, 4, 8] {
                let value = e.mem.f32(a[0] + offset) - e.mem.f32(a[2] + offset);
                e.mem.set_f32(a[1] + offset, value);
            }
            ret(a[1])
        });
        e.register(POINT3_LENGTH, |e, a| {
            let (x, y, z) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8));
            st0(((x * x + y * y + z * z) as f64).sqrt())
        });
        e.register(
            SQUARE_ROOT,
            |_, a| st0((f32::from_bits(a[0]) as f64).sqrt()),
        );
        e.register(ABSOLUTE, |_, a| st0(f32::from_bits(a[0]).abs() as f64));
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        for constructor in [
            POINT3_CONSTRUCT,
            CHUNK_CONSTRUCT,
            BLOCK_CONSTRUCT,
            TREES_CONSTRUCT,
        ] {
            e.register(constructor, returns_this);
        }
        for callee in [
            DEALLOCATE,
            MEMSET,
            GUARD_CONSTRUCT,
            GUARD_DESTRUCT,
            PRINT_WARNING,
            POINTER_DESTRUCT,
            CHUNK_DESTRUCT,
            CHUNK_CALL_006FACB0,
            CHUNK_CALL_006FB6E0,
            CHUNK_CALL_006FB760,
            CHUNK_SET_MORPH,
            CHUNK_GET_TEXTURE,
            GEOMETRY_SET_MORPH,
            END_TEXTURE_MORPH,
            CHUNK_PENDING_OBJECT,
            PENDING_OBJECT_CALL,
            GEOMETRY_SET_SCALE,
            SCENE_NODE_FLAG,
            WORLD_TEST,
            BLOCK_DESTRUCT,
            BLOCK_CALL_006F5CA0,
            BLOCK_CALL_006F5AF0,
            BLOCK_CALL_006F6370,
            BLOCK_CALL_006F6540,
            BLOCK_OBJECTS,
            BLOCK_UPDATE_STINGER_VISIBILITY,
            BLOCK_CALL_006F5A70,
            TREES_DESTRUCT,
            TREES_ALLOCATE_STATIC_DATA,
            TREES_CALL_006F8C40,
            TREES_UPDATE_SHADER_PROPERTIES,
            TREES_CALL_006F83F0,
            NODE_DELETE,
            SHAPE_GET_NUM_SEGMENTS,
            SHAPE_ENABLE_SEGMENT,
            SHAPE_DISABLE_SEGMENT,
            SHAPE_MERGE_ENABLED_SEGMENTS,
            SHAPE_ENABLE_ALL_SEGMENTS,
            WORLD_GET_CELL,
            TES_IS_CELL_LOADED,
            CELL_ALL_3D_FADING_IN,
        ] {
            e.register(callee, nothing);
        }
        for page in [
            0x0101_1000,
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0102_e000,
            0x011c_6000,
            0x011d_8000,
            0x011d_e000,
            0x011f_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(UNITS_PER_CELL, 4096.0f64);
        e.set_global(HALF_CELL, 2048.0f64);
        e.set_global(ONE_HALF, 0.5f64);
        e.set_global(ONE_HALF_FLOAT, 0.5f32);
        e.set_global(FOUR, 4.0f64);
        e.set_global(ROOT_MORPH_START, 0x0080_0000u32);
        e.set_global(ROOT_MORPH_END, 0x7f7f_ffffu32);
        e.set_global(GRIDS_TO_LOAD + 4, 5u32);
        e.set_global(TEXTURE_FADE_TIME + 4, 1000u32);
        e.set_global(SPLIT_DISTANCE_MULT + 4, 2.0f32);
        e.set_global(MORPH_START_DISTANCE_MULT + 4, 1.5f32);
        e.set_global(MORPH_END_DISTANCE_MULT + 4, 1.0f32);
        e.set_global(DETAIL_TEXTURE_SCALE + 4, 3.0f32);

        let manager = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(manager.addr(), 0x7000);
        e.mem.set_u32(manager.addr() + 0x18, 2);
        e.mem.set_u32(manager.addr() + 0x1c, 1);
        e.mem.set_u32(manager.addr() + 0x24, 4);
        let tes = e.mem.alloc(0x100);
        e.mem.set_u32(tes + 0x24, 1000);
        e.mem.set_u32(tes + 0x28, 1000);
        e.mem.set_u32(tes + 0x88, 0x7000);
        e.set_global(TES_POINTER, tes);
        e.call_log = Some(vec![]);
        World { e, manager, tes }
    }

    impl World {
        /// A node of `level` cells at (`x`, `y`), centered as the
        /// constructor centers it.
        fn node(&mut self, level: u32, x: i16, y: i16) -> Ptr<BGSTerrainNode> {
            let e = &mut self.e;
            let node: Ptr<BGSTerrainNode> = e.new_object();
            e.set(node, BGSTerrainNode::pManager, self.manager);
            e.set(node, BGSTerrainNode::iLevel, level);
            e.set(node, BGSTerrainNode::sBaseCellX, x);
            e.set(node, BGSTerrainNode::sBaseCellY, y);
            let center = node.at(BGSTerrainNode::Center);
            let half = level as f32 * 2048.0;
            e.set(center, NiPoint3::x, x as f32 * 4096.0 + half);
            e.set(center, NiPoint3::y, y as f32 * 4096.0 + half);
            node
        }

        /// A zeroed object of `size` bytes.
        fn object(&mut self, size: u32) -> Ptr {
            Ptr::new(self.e.mem.alloc(size))
        }

        /// A point at (`x`, `y`, 0).
        fn point(&mut self, x: f32, y: f32) -> Ptr<NiPoint3> {
            let point: Ptr<NiPoint3> = self.e.new_object();
            self.e.set(point, NiPoint3::x, x);
            self.e.set(point, NiPoint3::y, y);
            point
        }

        /// Gives `object` a vtable whose slot `slot` is `target`.
        fn vtable(&mut self, object: u32, slot: u32, target: u32) {
            let vtable = self.e.mem.alloc(0x100);
            self.e.mem.set_u32(vtable + slot, target);
            self.e.mem.set_u32(object, vtable);
        }

        fn calls(&self, addr: u32) -> Vec<Vec<u32>> {
            calls_to(&self.e, addr)
        }

        /// Puts the `TES` object's current cell at (`x`, `y`).
        fn current_cell(&mut self, x: i32, y: i32) {
            self.e.mem.set_i32(self.tes + 0x24, x);
            self.e.mem.set_i32(self.tes + 0x28, y);
        }

        /// Runs the constructor through its address and checks it returns
        /// `this`.
        fn construct(
            &mut self,
            this: Ptr<BGSTerrainNode>,
            parent: Ptr<BGSTerrainNode>,
            x: i16,
            y: i16,
            level: u32,
            persistent: bool,
        ) {
            let manager = self.manager;
            let back = self
                .e
                .call(
                    0x006f_d210,
                    &args![this, manager, parent, x, y, level, persistent],
                )
                .u32();
            assert_eq!(back, this.addr());
        }
    }

    #[test]
    fn constructor_builds_a_leaf_root() {
        let mut w = world();
        let this: Ptr<BGSTerrainNode> = w.e.new_object();
        w.construct(this, Ptr::NULL, 2, -1, 1, true);
        let e = &w.e;
        assert!(e.get(this, BGSTerrainNode::bSplitNode));
        assert!(e.get(this, BGSTerrainNode::bHasGeometry));
        assert!(e.get(this, BGSTerrainNode::bPersistentNode));
        assert!(!e.get(this, BGSTerrainNode::bChildrenLoaded));
        let center = this.at(BGSTerrainNode::Center);
        assert_eq!(e.get(center, NiPoint3::x), 2.0 * 4096.0 + 2048.0);
        assert_eq!(e.get(center, NiPoint3::y), -4096.0 + 2048.0);
        assert_eq!(
            e.get(this, BGSTerrainNode::fCornerRadius),
            (2.0f64 * 2048.0 * 2048.0).sqrt() as f32
        );
        // Level 1 is the manager's +0x1c level: no children.
        assert_eq!(w.calls(MEMSET), vec![vec![this.addr() + 0x20, 0, 0x10]]);
        assert_eq!(e.get(this, BGSTerrainNode::fSplitDistance), 0.0);
        assert_eq!(
            e.get(this, BGSTerrainNode::fStartMorphDistance).to_bits(),
            0x0080_0000
        );
        assert_eq!(
            e.get(this, BGSTerrainNode::fEndMorphDistance).to_bits(),
            0x7f7f_ffff
        );
        let guard = w.calls(GUARD_CONSTRUCT);
        assert_eq!(&guard[0][1..], &[0x22, 1, SOURCE_FILE, 0x4c]);
        assert_eq!(w.calls(GUARD_DESTRUCT), vec![vec![guard[0][0]]]);
        assert!(w.calls(PRINT_WARNING).is_empty());
    }

    #[test]
    fn constructor_splits_into_four_quarters() {
        let mut w = world();
        // bKeepLowDetailTerrain on: the children of a level-4 node have the
        // manager's +0x18 level (2), so they get the flag.
        w.e.set_global(KEEP_LOW_DETAIL_TERRAIN + 4, 1u8);
        let this: Ptr<BGSTerrainNode> = w.e.new_object();
        w.construct(this, Ptr::NULL, 2, -1, 4, false);
        let e = &w.e;
        assert!(!e.get(this, BGSTerrainNode::bHasGeometry));
        let radius = e.get(this, BGSTerrainNode::fCornerRadius);
        assert_eq!(e.get(this, BGSTerrainNode::fSplitDistance), radius * 2.0);
        let expected = [((2, -1), 1), ((2, 1), 3), ((4, -1), 2), ((4, 1), 4)];
        for (index, ((x, y), position)) in expected.into_iter().enumerate() {
            let node = child(e, this, index as u32);
            assert_eq!(e.get(node, BGSTerrainNode::sBaseCellX), x);
            assert_eq!(e.get(node, BGSTerrainNode::sBaseCellY), y);
            assert_eq!(e.get(node, BGSTerrainNode::iLevel), 2);
            assert_eq!(e.get(node, BGSTerrainNode::ParentPosition), position);
            assert_eq!(e.get(node, BGSTerrainNode::pParent), this);
            assert!(e.get(node, BGSTerrainNode::bPersistentNode));
            assert!(e.get(node, BGSTerrainNode::bHasGeometry));
            // Morph distances from the parent's corner radius (1.5 and 1.0):
            // start is not below end, so no warning.
            assert_eq!(
                e.get(node, BGSTerrainNode::fStartMorphDistance),
                radius * 1.5
            );
            assert_eq!(e.get(node, BGSTerrainNode::fEndMorphDistance), radius);
        }
        assert!(w.calls(PRINT_WARNING).is_empty());
        // Grandchildren (level 1) are not flagged: half level 1 is not 2.
        let grandchild = child(e, child(e, this, 0), 0);
        assert!(!e.get(grandchild, BGSTerrainNode::bPersistentNode));
    }

    #[test]
    fn constructor_warns_when_morph_start_is_below_end() {
        let mut w = world();
        w.e.set_global(MORPH_START_DISTANCE_MULT + 4, 0.5f32);
        let parent = w.node(2, 0, 0);
        w.e.set(parent, BGSTerrainNode::fCornerRadius, 100.0);
        let this: Ptr<BGSTerrainNode> = w.e.new_object();
        w.construct(this, parent, 0, 0, 1, false);
        assert_eq!(w.calls(PRINT_WARNING), vec![vec![MORPH_DISTANCE_WARNING]]);
        assert_eq!(w.e.get(this, BGSTerrainNode::fStartMorphDistance), 50.0);
        assert_eq!(w.e.get(this, BGSTerrainNode::fEndMorphDistance), 100.0);
    }

    #[test]
    fn destructor_body_unloads_and_deletes_the_children() {
        let mut w = world();
        let this = w.node(4, 0, 0);
        let trees = w.object(0x4c);
        w.e.set(this, BGSTerrainNode::pTrees, trees);
        let first = w.node(2, 0, 0);
        let third = w.node(2, 2, 0);
        set_child(&mut w.e, this, 0, first);
        set_child(&mut w.e, this, 2, third);
        w.e.call(0x006f_d750, &args![this]);
        assert!(w.e.get(this, BGSTerrainNode::pTrees).is_null());
        assert_eq!(w.calls(TREES_DESTRUCT), vec![vec![trees.addr()]]);
        assert_eq!(
            w.calls(NODE_DELETE),
            vec![vec![first.addr(), 1], vec![third.addr(), 1]]
        );
    }

    #[test]
    fn unload_keeps_the_chunk_of_a_persistent_node_of_the_active_manager() {
        let mut w = world();
        let parent = w.node(4, 0, 0);
        w.e.set(parent, BGSTerrainNode::bChildrenLoaded, true);
        let this = w.node(2, 0, 0);
        let chunk = w.object(0x30);
        let block = w.object(0x2c);
        w.e.set(this, BGSTerrainNode::pParent, parent);
        w.e.set(this, BGSTerrainNode::bHasGeometry, true);
        w.e.set(this, BGSTerrainNode::bPersistentNode, true);
        w.e.set(this, BGSTerrainNode::pChunk, chunk);
        w.e.set(this, BGSTerrainNode::pBlock, block);
        w.e.set_global(ACTIVE_MANAGER, w.manager.addr());
        w.e.call(0x006f_d7b0, &args![this, 3u32]);
        assert_eq!(w.calls(CHUNK_CALL_006FB6E0), vec![vec![chunk.addr()]]);
        assert_eq!(w.e.get(this, BGSTerrainNode::pChunk), chunk);
        assert_eq!(w.calls(BLOCK_DESTRUCT), vec![vec![block.addr()]]);
        assert!(w.e.get(this, BGSTerrainNode::pBlock).is_null());
        assert!(!w.e.get(parent, BGSTerrainNode::bChildrenLoaded));

        // With 7 the chunk goes even then.
        w.e.call(0x006f_d7b0, &args![this, 7u32]);
        assert_eq!(w.calls(CHUNK_DESTRUCT), vec![vec![chunk.addr()]]);
        assert_eq!(
            w.calls(DEALLOCATE),
            vec![vec![block.addr()], vec![chunk.addr()]]
        );
        assert!(w.e.get(this, BGSTerrainNode::pChunk).is_null());

        // Without geometry the chunk is left alone.
        let other = w.object(0x30);
        w.e.set(this, BGSTerrainNode::pChunk, other);
        w.e.set(this, BGSTerrainNode::bHasGeometry, false);
        w.e.call(0x006f_d7b0, &args![this, 7u32]);
        assert_eq!(w.e.get(this, BGSTerrainNode::pChunk), other);
    }

    #[test]
    fn active_manager_is_the_global_word() {
        let mut w = world();
        w.e.set_global(ACTIVE_MANAGER, 0x1234u32);
        assert_eq!(w.e.call(0x006f_d8e0, &args![]).u32(), 0x1234);
    }

    #[test]
    fn deleting_destructors_destroy_and_free_on_bit_0() {
        for (addr, destructor) in [
            (0x006f_d8f0, CHUNK_DESTRUCT),
            (0x006f_d920, BLOCK_DESTRUCT),
            (0x006f_d950, TREES_DESTRUCT),
        ] {
            let mut w = world();
            let object = w.object(0x10);
            assert_eq!(w.e.call(addr, &args![object, 0u32]).u32(), object.addr());
            assert!(w.calls(DEALLOCATE).is_empty());
            assert_eq!(w.e.call(addr, &args![object, 3u32]).u32(), object.addr());
            assert_eq!(w.calls(destructor).len(), 2);
            assert_eq!(w.calls(DEALLOCATE), vec![vec![object.addr()]]);
        }
    }

    #[test]
    fn unload_subtree_reaches_children_and_the_root_tree_call() {
        let mut w = world();
        let this = w.node(4, 0, 0);
        w.e.set(this, BGSTerrainNode::bChildrenLoaded, true);
        let first = w.node(2, 0, 0);
        let trees = w.object(0x4c);
        let block = w.object(0x2c);
        w.e.set(first, BGSTerrainNode::pTrees, trees);
        w.e.set(first, BGSTerrainNode::pBlock, block);
        set_child(&mut w.e, this, 1, first);
        w.e.mem.set_u32(w.manager.addr() + 4, this.addr());
        w.e.call(0x006f_d980, &args![this, 4u32]);
        assert_eq!(w.calls(TREES_DESTRUCT), vec![vec![trees.addr()]]);
        assert!(w.calls(BLOCK_DESTRUCT).is_empty());
        assert!(!w.e.get(this, BGSTerrainNode::bChildrenLoaded));
        // The root runs 006f83f0 once; the child is not the root.
        assert_eq!(w.calls(TREES_CALL_006F83F0), vec![Vec::<u32>::new()]);
    }

    #[test]
    fn persistent_chunks_are_deleted_through_the_subtree() {
        let mut w = world();
        let this = w.node(4, 0, 0);
        let kept = w.object(0x30);
        w.e.set(this, BGSTerrainNode::pChunk, kept);
        let persistent = w.node(2, 0, 0);
        let chunk = w.object(0x30);
        w.e.set(persistent, BGSTerrainNode::pChunk, chunk);
        w.e.set(persistent, BGSTerrainNode::bPersistentNode, true);
        set_child(&mut w.e, this, 3, persistent);
        w.e.call(0x006f_da10, &args![this]);
        assert_eq!(w.e.get(this, BGSTerrainNode::pChunk), kept);
        assert!(w.e.get(persistent, BGSTerrainNode::pChunk).is_null());
        assert_eq!(w.calls(CHUNK_DESTRUCT), vec![vec![chunk.addr()]]);
    }

    #[test]
    fn update_terrain_splits_above_the_max_level() {
        let mut w = world();
        // Level 4 is the manager's +0x1c level here (no split-range test)
        // and above its +0x18 level: the node splits.
        w.e.mem.set_u32(w.manager.addr() + 0x1c, 4);
        let this = w.node(4, 3, 5);
        w.e.set(this, BGSTerrainNode::bPersistentNode, true);
        let position = w.point(0.0, 0.0);
        w.e.call(0x006f_daa0, &args![this, position, false]);
        assert!(w.e.get(this, BGSTerrainNode::bSplitNode));
        // A persistent splitting node makes its chunk.
        let chunk = w.e.get(this, BGSTerrainNode::pChunk);
        assert_eq!(
            w.calls(CHUNK_CONSTRUCT),
            vec![vec![chunk.addr(), this.addr(), 4, 3, 5]]
        );
        assert!(!w.e.get(this, BGSTerrainNode::bChildrenLoaded));
        let guard = w.calls(GUARD_CONSTRUCT);
        assert_eq!(&guard[0][1..], &[0x23, 1, SOURCE_FILE, 0x11c]);
    }

    #[test]
    fn update_terrain_hands_over_to_ready_children() {
        let mut w = world();
        // Manager: max level 2, min level 1; the node (level 4) splits, its
        // children (level 1, without geometry) do nothing.
        let this = w.node(4, 0, 0);
        let chunk = w.object(0x30);
        w.e.set(this, BGSTerrainNode::pChunk, chunk);
        w.e.set(this, BGSTerrainNode::bHasGeometry, true);
        let mut children = vec![];
        for index in 0..4 {
            let node = w.node(1, index as i16, 0);
            let child_chunk = w.object(0x30);
            w.e.mem.set_u8(child_chunk.addr() + 0x28, 1);
            w.e.set(node, BGSTerrainNode::pChunk, child_chunk);
            w.e.set(node, BGSTerrainNode::pParent, this);
            set_child(&mut w.e, this, index, node);
            children.push(node);
        }
        let position = w.point(0.0, 0.0);
        w.e.call(0x006f_daa0, &args![this, position, false]);
        // Every child's texture morph starts (two textures of this chunk).
        assert_eq!(w.calls(CHUNK_GET_TEXTURE).len(), 8);
        assert_eq!(&w.calls(CHUNK_GET_TEXTURE)[0], &vec![chunk.addr(), 0]);
        // The node's own chunk is unloaded and the children count as
        // loaded.
        assert_eq!(w.calls(CHUNK_DESTRUCT), vec![vec![chunk.addr()]]);
        assert!(w.e.get(this, BGSTerrainNode::pChunk).is_null());
        assert!(w.e.get(this, BGSTerrainNode::bChildrenLoaded));
        assert!(w.e.get(this, BGSTerrainNode::bSplitNode));
        for node in children {
            assert!(!w.e.get(node, BGSTerrainNode::bSplitNode));
        }
    }

    #[test]
    fn update_terrain_refreshes_a_geometry_node() {
        let mut w = world();
        // Level 1 = min level: no split-range test; below max: geometry.
        let this = w.node(1, 0, 0);
        w.e.set(this, BGSTerrainNode::bHasGeometry, true);
        w.e.set(this, BGSTerrainNode::bChildrenLoaded, true);
        let chunk = w.object(0x30);
        w.e.mem.set_u8(chunk.addr() + 0x28, 1);
        let geometry = w.object(0x10);
        w.e.mem.set_u32(chunk.addr() + 0x2c, geometry.addr());
        let water = w.object(0x10);
        w.e.mem.set_u32(chunk.addr() + 8, water.addr());
        w.vtable(water.addr(), 0x28, 0x0400_0028);
        w.e.register(0x0400_0028, |_, _| ret(0));
        w.e.set(this, BGSTerrainNode::pChunk, chunk);
        let root = w.object(0x10);
        w.e.mem.set_u32(w.manager.addr() + 8, root.addr());
        w.vtable(root.addr(), 0xbc, 0x0400_00bc);
        w.e.register(0x0400_00bc, nothing);
        // A pending object, the world differs from the current one, the
        // world tests pass and the node is in the loaded range.
        w.e.register(CHUNK_PENDING_OBJECT, |_, _| ret(0x5555));
        w.e.mem.set_u32(w.tes + 0x88, 0x7100);
        w.e.register(WORLD_TEST, |_, a| ret((a[1] == 1) as u32));
        w.current_cell(0, 0);
        let position = w.point(0.0, 0.0);
        w.e.call(0x006f_daa0, &args![this, position, true]);
        assert!(!w.e.get(this, BGSTerrainNode::bSplitNode));
        assert_eq!(w.calls(CHUNK_CALL_006FACB0), vec![vec![chunk.addr()]]);
        // Detail texture scale 3 times level / 4.
        assert_eq!(
            w.calls(GEOMETRY_SET_SCALE),
            vec![vec![geometry.addr(), 0.75f32.to_bits()]]
        );
        assert_eq!(
            w.calls(SCENE_NODE_FLAG),
            vec![vec![root.addr(), 0], vec![root.addr(), 1]]
        );
        assert_eq!(w.calls(0x0400_00bc), vec![vec![root.addr()]]);
        assert_eq!(w.calls(PENDING_OBJECT_CALL), vec![vec![0x5555, 1]]);
        // The water object's shape is null: no segment work.
        assert_eq!(w.calls(0x0400_0028), vec![vec![water.addr()]]);
        assert!(w.calls(SHAPE_ENABLE_ALL_SEGMENTS).is_empty());
        // Chunk byte +0x29 is clear: the children stay loaded.
        assert!(w.e.get(this, BGSTerrainNode::bChildrenLoaded));
    }

    #[test]
    fn update_blocks_builds_switches_and_deletes_blocks() {
        let mut w = world();
        // Block level 4; block range 1e9, low range 0.
        w.e.set_global(BLOCK_LOAD_DISTANCE + 4, 1.0e9f32);
        let this = w.node(4, 1, 2);
        let position = w.point(0.0, 0.0);
        w.e.call(0x006f_dfc0, &args![this, position, true]);
        let block = w.e.get(this, BGSTerrainNode::pBlock);
        assert_eq!(
            w.calls(BLOCK_CONSTRUCT),
            vec![vec![block.addr(), this.addr(), 4, 1, 2, 0]]
        );
        // A block without its +0x23 byte for a node not wanting low
        // detail.
        assert_eq!(
            w.calls(BLOCK_CALL_006F6540),
            vec![vec![block.addr(), 0x7000, 4, 1, 2]]
        );

        // Out of range: the block is deleted.
        w.e.set_global(BLOCK_LOAD_DISTANCE + 4, 0.0f32);
        w.e.call(0x006f_dfc0, &args![this, position, true]);
        assert!(w.e.get(this, BGSTerrainNode::pBlock).is_null());
        assert_eq!(w.calls(BLOCK_DESTRUCT), vec![vec![block.addr()]]);

        // Above the block level the children are updated.
        let parent = w.node(8, 0, 0);
        set_child(&mut w.e, parent, 2, this);
        let other = w.object(0x2c);
        w.e.set(this, BGSTerrainNode::pBlock, other);
        w.e.call(0x006f_dfc0, &args![parent, position, true]);
        assert!(w.e.get(this, BGSTerrainNode::pBlock).is_null());
    }

    #[test]
    fn update_blocks_refreshes_a_loaded_block() {
        let mut w = world();
        w.e.set_global(BLOCK_LOAD_DISTANCE + 4, 1.0e9f32);
        w.e.set_global(BLOCK_LOAD_DISTANCE_LOW + 4, 1.0e9f32);
        let this = w.node(4, 1, 2);
        let block = w.object(0x2c);
        let object = w.object(0x10);
        w.e.mem.set_u32(block.addr() + 4, object.addr());
        w.e.mem.set_u8(block.addr() + 0x20, 1);
        w.e.mem.set_u8(block.addr() + 0x23, 1);
        w.e.mem.set_u8(block.addr() + 0x24, 1);
        w.vtable(object.addr(), 0x28, 0x0400_0028);
        w.e.register(0x0400_0028, |_, _| ret(0));
        w.e.set(this, BGSTerrainNode::pBlock, block);
        let position = w.point(0.0, 0.0);
        w.e.call(0x006f_dfc0, &args![this, position, true]);
        assert_eq!(w.calls(BLOCK_CALL_006F5CA0), vec![vec![block.addr()]]);
        // The visibility update ran (it asked for the shape).
        assert_eq!(w.calls(0x0400_0028), vec![vec![object.addr()]]);
        assert_eq!(
            w.calls(BLOCK_CALL_006F6370),
            vec![vec![block.addr(), 0x7000, 4, 1, 2]]
        );
        assert_eq!(w.calls(BLOCK_CALL_006F5AF0), vec![vec![block.addr()]]);
    }

    #[test]
    fn smart_pointer_object_or_empty_temporary() {
        let mut w = world();
        let holder = w.object(8);
        w.e.mem.set_u32(holder.addr() + 4, 0x4444);
        assert_eq!(w.e.call(0x006f_e280, &args![holder, false]).u32(), 0x4444);
        assert_eq!(w.e.call(0x006f_e280, &args![holder, true]).u32(), 0);
        let built = w.calls(POINTER_CONSTRUCT);
        assert_eq!(built[0][1], 0);
        assert_eq!(w.calls(POINTER_DESTRUCT), vec![vec![built[0][0]]]);
    }

    #[test]
    fn update_trees_builds_refreshes_and_deletes_tree_blocks() {
        let mut w = world();
        w.e.set_global(TREE_LOAD_DISTANCE + 4, 1.0e9f32);
        let this = w.node(8, 1, 2);
        let position = w.point(0.0, 0.0);
        w.e.call(0x006f_e330, &args![this, position]);
        assert_eq!(w.calls(TREES_ALLOCATE_STATIC_DATA), vec![vec![0x7000]]);
        let trees = w.e.get(this, BGSTerrainNode::pTrees);
        assert_eq!(
            w.calls(TREES_CONSTRUCT),
            vec![vec![trees.addr(), this.addr(), 8, 1, 2]]
        );
        // With the static data allocated and the block's +0x48 byte set.
        w.e.set_global(TREE_STATIC_DATA_FLAG, 1u8);
        w.e.mem.set_u8(trees.addr() + 0x48, 1);
        w.e.call(0x006f_e330, &args![this, position]);
        assert_eq!(w.calls(TREES_ALLOCATE_STATIC_DATA).len(), 1);
        assert_eq!(w.calls(TREES_CALL_006F8C40), vec![vec![trees.addr()]]);
        assert_eq!(
            w.calls(TREES_UPDATE_SHADER_PROPERTIES),
            vec![vec![trees.addr()]]
        );
        // From a level-16 parent, out of range: deleted.
        w.e.set_global(TREE_LOAD_DISTANCE + 4, 0.0f32);
        let parent = w.node(16, 0, 0);
        set_child(&mut w.e, parent, 0, this);
        w.e.call(0x006f_e330, &args![parent, position]);
        assert!(w.e.get(this, BGSTerrainNode::pTrees).is_null());
        assert_eq!(w.calls(TREES_DESTRUCT), vec![vec![trees.addr()]]);
    }

    #[test]
    fn tree_level_flag_and_static_data_flag() {
        let mut w = world();
        assert_eq!(w.e.call(0x006f_e510, &args![]).u32(), 8);
        let trees = w.object(0x4c);
        assert!(!w.e.call(0x006f_e520, &args![trees]).bool());
        w.e.mem.set_u8(trees.addr() + 0x48, 1);
        assert!(w.e.call(0x006f_e520, &args![trees]).bool());
        assert!(!w.e.call(0x006f_e540, &args![]).bool());
        w.e.set_global(TREE_STATIC_DATA_FLAG, 1u8);
        assert!(w.e.call(0x006f_e540, &args![]).bool());
    }

    #[test]
    fn split_test_uses_level_loaded_range_and_split_distance() {
        let mut w = world();
        // Node of 2 cells at (0, 0): square 0..8192; viewer 1000 units past
        // its +x side.
        let this = w.node(2, 0, 0);
        let position = w.point(9192.0, 4096.0);
        w.e.set(this, BGSTerrainNode::fSplitDistance, 1000.5);
        assert!(w.e.call(0x006f_e550, &args![this, position]).bool());
        w.e.set(this, BGSTerrainNode::fSplitDistance, 1000.0);
        assert!(!w.e.call(0x006f_e550, &args![this, position]).bool());
        // NaN split distance: unordered, false.
        w.e.set(this, BGSTerrainNode::fSplitDistance, f32::NAN);
        assert!(!w.e.call(0x006f_e550, &args![this, position]).bool());
        // In the loaded range: true.
        w.current_cell(0, 0);
        assert!(w.e.call(0x006f_e550, &args![this, position]).bool());
        // At the manager's +0x1c level: false.
        w.e.set(this, BGSTerrainNode::iLevel, 1);
        assert!(!w.e.call(0x006f_e550, &args![this, position]).bool());
    }

    #[test]
    fn load_range_tests_compare_their_settings_with_the_distance() {
        for (addr, setting) in [
            (0x006f_e620, BLOCK_LOAD_DISTANCE),
            (0x006f_e6d0, BLOCK_LOAD_DISTANCE_LOW),
            (0x006f_e780, TREE_LOAD_DISTANCE),
        ] {
            let mut w = world();
            let this = w.node(2, 0, 0);
            let position = w.point(4096.0, -1000.0);
            w.e.set_global(setting + 4, 1000.25f32);
            assert!(w.e.call(addr, &args![this, position]).bool());
            w.e.set_global(setting + 4, 1000.0f32);
            assert!(!w.e.call(addr, &args![this, position]).bool());
            assert_eq!(w.calls(SETTING_FLOAT), vec![vec![setting], vec![setting]]);
        }
    }

    #[test]
    fn square_distance_measures_outside_the_square_only() {
        let mut w = world();
        let node = w.node(2, 0, 0);
        let call = |e: &mut Engine, x: f32, y: f32| {
            e.call(0x006f_e830, &args![node, 100.0f32, 100.0f32, 50.0f32, x, y])
                .f64()
        };
        // Square 75..125 on both axes.
        assert_eq!(call(&mut w.e, 100.0, 120.0), 0.0);
        assert_eq!(call(&mut w.e, 72.0, 129.0), 5.0);
        assert_eq!(call(&mut w.e, 128.0, 71.0), 5.0);
        // The point on the far edge counts as inside.
        assert_eq!(call(&mut w.e, 125.0, 75.0), 0.0);
        // A NaN point: the near side test is unordered and its difference
        // is taken.
        assert!(call(&mut w.e, f32::NAN, 100.0).is_nan());
    }

    #[test]
    fn children_ready_test() {
        let mut w = world();
        let this = w.node(4, 0, 0);
        let mut chunks = vec![];
        for index in 0..4 {
            let node = w.node(2, 0, 0);
            let chunk = w.object(0x30);
            w.e.mem.set_u8(chunk.addr() + 0x29, 1);
            w.e.set(node, BGSTerrainNode::pChunk, chunk);
            set_child(&mut w.e, this, index, node);
            chunks.push(chunk);
        }
        assert!(w.e.call(0x006f_e920, &args![this, true]).bool());
        // One child not ready (level 1 = min level: its own test fails).
        w.e.mem.set_u8(chunks[2].addr() + 0x29, 0);
        let third = child(&w.e, this, 2);
        w.e.set(third, BGSTerrainNode::iLevel, 1);
        assert!(!w.e.call(0x006f_e920, &args![this, true]).bool());
        // Outside the cell window the node counts as ready.
        for (x, y, ready) in [
            (41, 0, true),
            (40, 0, false),
            (-41, 0, true),
            (0, 45, true),
            (0, -45, true),
            (0, -44, false),
        ] {
            w.e.set(this, BGSTerrainNode::sBaseCellX, x);
            w.e.set(this, BGSTerrainNode::sBaseCellY, y);
            assert_eq!(
                w.e.call(0x006f_e920, &args![this, true]).bool(),
                ready,
                "{x} {y}"
            );
        }
        // At the min level: false.
        w.e.set(this, BGSTerrainNode::iLevel, 1);
        assert!(!w.e.call(0x006f_e920, &args![this, true]).bool());
    }

    #[test]
    fn node_ready_test() {
        let mut w = world();
        let this = w.node(1, 0, 0);
        // No chunk, min level: not ready.
        assert!(!w.e.call(0x006f_ea10, &args![this, false]).bool());
        let chunk = w.object(0x30);
        w.e.set(this, BGSTerrainNode::pChunk, chunk);
        w.e.mem.set_u8(chunk.addr() + 0x28, 1);
        assert!(w.e.call(0x006f_ea10, &args![this, false]).bool());
        w.e.mem.set_u8(chunk.addr() + 0x28, 0);
        w.e.mem.set_u8(chunk.addr() + 0x29, 1);
        assert!(w.e.call(0x006f_ea10, &args![this, false]).bool());
    }

    #[test]
    fn morph_update_sets_factors_down_to_unsplit_nodes() {
        let mut w = world();
        let this = w.node(2, 0, 0);
        w.e.set(this, BGSTerrainNode::fEndMorphDistance, 2000.0);
        w.e.set(this, BGSTerrainNode::bSplitNode, true);
        let chunk = w.object(0x30);
        w.e.mem.set_u8(chunk.addr() + 0x28, 1);
        w.e.set(this, BGSTerrainNode::pChunk, chunk);
        let leaf = w.node(1, 0, 0);
        for index in 0..4 {
            set_child(&mut w.e, this, index, leaf);
        }
        let position = w.point(9192.0, 4096.0);
        // Split and not persistent: stops here.
        w.e.call(0x006f_ea70, &args![this, position]);
        assert_eq!(
            w.calls(CHUNK_SET_MORPH),
            vec![vec![chunk.addr(), 1.0f32.to_bits()]]
        );
        // Not split: the four children are visited (no chunks there, so
        // the only morph is this node's).
        w.e.set(this, BGSTerrainNode::bSplitNode, false);
        w.e.call(0x006f_ea70, &args![this, position]);
        assert_eq!(w.calls(CHUNK_SET_MORPH).len(), 2);
        assert_eq!(w.calls(BYTE_AT_28).len(), 2);
    }

    #[test]
    fn morph_factor_between_end_and_start() {
        let mut w = world();
        let this = w.node(2, 0, 0);
        w.e.set(this, BGSTerrainNode::fEndMorphDistance, 1000.0);
        w.e.set(this, BGSTerrainNode::fStartMorphDistance, 3000.0);
        for (distance, factor) in [
            (500.0f32, 1.0f32),
            (1000.0, 1.0),
            (1500.0, 0.75),
            (3000.0, 0.0),
            (3500.0, 0.0),
        ] {
            let position = w.point(8192.0 + distance, 4096.0);
            assert_eq!(
                w.e.call(0x006f_eb20, &args![this, position]).f32(),
                factor,
                "{distance}"
            );
        }
    }

    #[test]
    fn contains_test_checks_y_then_x() {
        let mut w = world();
        // Level 2 at (0, 0): center (4096, 4096), half side 4096.
        let this = w.node(2, 0, 0);
        for (x, y, inside) in [
            (0.0f32, 8192.0f32, true),
            (4096.0, 8192.5, false),
            (-0.5, 4096.0, false),
            (f32::NAN, 4096.0, true),
        ] {
            let position = w.point(x, y);
            assert_eq!(w.e.call(0x006f_ec00, &args![this, position]).bool(), inside);
        }
        // y is tested first: a y miss does not test x.
        w.e.call_log = Some(vec![]);
        let position = w.point(-10.0, -10.0);
        w.e.call(0x006f_ec00, &args![this, position]);
        assert_eq!(w.calls(ABSOLUTE).len(), 1);
    }

    #[test]
    fn chunk_at_position_finds_the_unsplit_drawn_node() {
        let mut w = world();
        let this = w.node(2, 0, 0);
        let mut chunks = vec![];
        for (index, (x, y)) in [(0, 0), (0, 1), (1, 0), (1, 1)].into_iter().enumerate() {
            let node = w.node(1, x, y);
            let chunk = w.object(0x30);
            w.e.mem.set_u8(chunk.addr() + 0x28, 1);
            w.e.set(node, BGSTerrainNode::pChunk, chunk);
            set_child(&mut w.e, this, index as u32, node);
            chunks.push(chunk);
        }
        // Split node: the child under (6000, 1000) is the third.
        w.e.set(this, BGSTerrainNode::bSplitNode, true);
        let position = w.point(6000.0, 1000.0);
        assert_eq!(
            w.e.call(0x006f_ec90, &args![this, position]).u32(),
            chunks[2].addr()
        );
        // Unsplit with a ready chunk: its own.
        let chunk = w.object(0x30);
        w.e.mem.set_u8(chunk.addr() + 0x28, 1);
        w.e.set(this, BGSTerrainNode::pChunk, chunk);
        w.e.set(this, BGSTerrainNode::bSplitNode, false);
        assert_eq!(
            w.e.call(0x006f_ec90, &args![this, position]).u32(),
            chunk.addr()
        );
        // Outside: null.
        let far = w.point(-5.0, 1000.0);
        assert_eq!(w.e.call(0x006f_ec90, &args![this, far]).u32(), 0);
    }

    #[test]
    fn tree_block_at_position_comes_from_the_tree_level() {
        let mut w = world();
        let this = w.node(16, 0, 0);
        let mut blocks = vec![];
        for (index, (x, y)) in [(0, 0), (0, 8), (8, 0), (8, 8)].into_iter().enumerate() {
            let node = w.node(8, x, y);
            let trees = w.object(0x4c);
            w.e.set(node, BGSTerrainNode::pTrees, trees);
            set_child(&mut w.e, this, index as u32, node);
            blocks.push(trees);
        }
        let position = w.point(1000.0, 40000.0);
        assert_eq!(
            w.e.call(0x006f_ed30, &args![this, position]).u32(),
            blocks[1].addr()
        );
        // A tree-level node containing the point without a tree block: null.
        let lone = w.node(8, 0, 8);
        assert_eq!(w.e.call(0x006f_ed30, &args![lone, position]).u32(), 0);
    }

    #[test]
    fn loaded_range_overlap() {
        let mut w = world();
        // 5 grids around cell (10, 10): the range starts at 8 and spans 5,
        // so a 2-cell node overlaps from 6 to 13.
        let this = w.node(2, 0, 0);
        w.current_cell(10, 10);
        for (x, y, overlaps) in [
            (6, 10, true),
            (5, 10, false),
            (13, 10, true),
            (14, 10, false),
            (10, 6, true),
            (10, 5, false),
            (10, 13, true),
            (10, 14, false),
        ] {
            w.e.set(this, BGSTerrainNode::sBaseCellX, x);
            w.e.set(this, BGSTerrainNode::sBaseCellY, y);
            assert_eq!(
                w.e.call(0x006f_ede0, &args![this]).bool(),
                overlaps,
                "{x} {y}"
            );
        }
    }

    /// A segmented shape whose segment count is `filled.len()` and whose
    /// entries at `+0xc4` (0x14 bytes each) have `filled[i]` at `+4`. Cells
    /// are their coordinates packed; cells in row 1 are loaded, and of
    /// those the ones in column 0 have their 3D fading in.
    fn shape(w: &mut World, filled: &[u32]) -> Ptr {
        let shape = w.object(0xd0);
        let entries = w.e.mem.alloc(0x14 * filled.len() as u32);
        for (i, word) in filled.iter().enumerate() {
            w.e.mem.set_u32(entries + 0x14 * i as u32 + 4, *word);
        }
        w.e.mem.set_u32(shape.addr() + 0xc4, entries);
        w.e.mem.set_u32(shape.addr() + 0xc8, filled.len() as u32);
        w.e.register(SHAPE_GET_NUM_SEGMENTS, |e, a| ret(e.mem.u32(a[0] + 0xc8)));
        w.e.register(WORLD_GET_CELL, |_, a| ret(0x0100_0000 | (a[1] << 8) | a[2]));
        w.e.register(TES_IS_CELL_LOADED, |_, a| {
            ret(((a[1] >> 8) & 0xff == 1) as u32)
        });
        w.e.register(CELL_ALL_3D_FADING_IN, |_, a| ret((a[0] & 0xff == 0) as u32));
        shape
    }

    type SegmentCalls = (Vec<u32>, Vec<u32>);

    /// The segments enabled and disabled on `shape`.
    fn segments(w: &World, shape: Ptr) -> SegmentCalls {
        let of_shape = |addr| -> Vec<u32> {
            w.calls(addr)
                .into_iter()
                .filter(|a| a[0] == shape.addr())
                .map(|a| a[1])
                .collect()
        };
        (
            of_shape(SHAPE_ENABLE_SEGMENT),
            of_shape(SHAPE_DISABLE_SEGMENT),
        )
    }

    #[test]
    fn block_visibility_hides_segments_over_loaded_fading_cells() {
        let mut w = world();
        let this = w.node(2, 0, 0);
        // Three segments (index 3 is past the count); segments 1 and 2 have
        // entries.
        let shape = shape(&mut w, &[0, 7, 7]);
        let block = w.object(0x2c);
        let object = w.object(0x10);
        w.e.mem.set_u32(block.addr() + 4, object.addr());
        w.vtable(object.addr(), 0x28, 0x0400_0028);
        w.e.register(0x0400_0028, |e, a| ret(e.mem.u32(a[0] + 4)));
        w.e.mem.set_u32(object.addr() + 4, shape.addr());
        w.e.set(this, BGSTerrainNode::pBlock, block);
        w.e.register(BLOCK_OBJECTS, |_, _| ret(0x6666));
        w.current_cell(0, 0);
        w.e.call(0x006f_ee90, &args![this, 0u32]);
        // Segment index = 2 * row + column for cell (row, column). Row 1 is
        // loaded and column 0 fading in: index 2 (with an entry) is hidden;
        // index 1 has an entry but its cell is not loaded.
        assert_eq!(segments(&w, shape), (vec![0, 1], vec![2]));
        assert_eq!(
            w.calls(SHAPE_MERGE_ENABLED_SEGMENTS),
            vec![vec![shape.addr()]]
        );
        assert_eq!(
            w.calls(BLOCK_UPDATE_STINGER_VISIBILITY),
            vec![vec![block.addr(), 0x6666, 2, 0, 0]]
        );

        // Outside the loaded range: everything enabled, stingers through
        // 006f5a70.
        w.current_cell(100, 100);
        w.e.call(0x006f_ee90, &args![this, 0u32]);
        assert_eq!(w.calls(SHAPE_ENABLE_ALL_SEGMENTS), vec![vec![shape.addr()]]);
        assert_eq!(
            w.calls(BLOCK_CALL_006F5A70),
            vec![vec![block.addr(), 0x6666]]
        );
    }

    #[test]
    fn segment_entry_test() {
        let mut w = world();
        let shape = shape(&mut w, &[0, 9]);
        assert!(w.e.call(0x006f_f0a0, &args![shape, 0i32]).bool());
        assert!(!w.e.call(0x006f_f0a0, &args![shape, 1i32]).bool());
    }

    #[test]
    fn water_visibility_hides_segments_over_loaded_cells() {
        let mut w = world();
        let this = w.node(2, 0, 0);
        let shape = shape(&mut w, &[0, 5, 5, 0]);
        w.current_cell(0, 0);
        w.e.call(0x006f_f0d0, &args![this, shape, 0u32]);
        // Row 1 is loaded (indices 2, 3); of those only index 2 has an
        // entry. No fading test here.
        assert_eq!(segments(&w, shape), (vec![0, 1, 3], vec![2]));
        assert!(w.calls(CELL_ALL_3D_FADING_IN).is_empty());
        w.current_cell(100, 100);
        w.e.call(0x006f_f0d0, &args![this, shape, 0u32]);
        assert_eq!(w.calls(SHAPE_ENABLE_ALL_SEGMENTS), vec![vec![shape.addr()]]);
        // Without a shape nothing happens.
        w.e.call(0x006f_f0d0, &args![this, 0u32, 0u32]);
        assert_eq!(w.calls(SHAPE_MERGE_ENABLED_SEGMENTS).len(), 1);
    }

    #[test]
    fn parent_position_offsets() {
        let mut w = world();
        let this = w.node(1, 0, 0);
        let out = w.object(8);
        for (position, x, y) in [
            (0u32, 0.0f32, 0.0f32),
            (1, 0.5, 0.0),
            (2, 0.0, 0.0),
            (3, 0.5, 0.5),
            (4, 0.0, 0.5),
            (5, 0.0, 0.0),
        ] {
            w.e.set(this, BGSTerrainNode::ParentPosition, position);
            assert_eq!(w.e.call(0x006f_f210, &args![this, out]).u32(), out.addr());
            assert_eq!(w.e.mem.f32(out.addr()), x);
            assert_eq!(w.e.mem.f32(out.addr() + 4), y);
        }
    }

    #[test]
    fn start_texture_morph_sets_textures_and_fade() {
        let mut w = world();
        let parent = w.node(2, 0, 0);
        let parent_chunk = w.object(0x30);
        w.e.set(parent, BGSTerrainNode::pChunk, parent_chunk);
        let this = w.node(1, 1, 0);
        w.e.set(this, BGSTerrainNode::pParent, parent);
        w.e.set(this, BGSTerrainNode::ParentPosition, 3);
        let chunk = w.object(0x30);
        w.e.set(this, BGSTerrainNode::pChunk, chunk);
        w.e.register(CHUNK_GET_TEXTURE, |_, a| ret(0x0900 + a[1]));
        w.e.set_global(TIMER_OBJECT + 0x14, 777u32);
        // Without geometry: textures fetched, nothing else.
        w.e.call(0x006f_f2c0, &args![this]);
        assert_eq!(
            w.calls(CHUNK_GET_TEXTURE),
            vec![vec![parent_chunk.addr(), 0], vec![parent_chunk.addr(), 1]]
        );
        assert!(!w.e.get(this, BGSTerrainNode::bTexFading));

        let geometry = w.object(0x10);
        w.e.mem.set_u32(chunk.addr() + 0x2c, geometry.addr());
        w.vtable(geometry.addr(), 0xf0, 0x0400_00f0);
        w.e.register(0x0400_00f0, nothing);
        w.e.call(0x006f_f2c0, &args![this]);
        assert_eq!(
            w.calls(0x0400_00f0),
            vec![
                vec![geometry.addr(), 0, 1, 0x0900],
                vec![geometry.addr(), 1, 1, 0x0901]
            ]
        );
        assert_eq!(
            w.calls(GEOMETRY_SET_MORPH),
            vec![vec![geometry.addr(), 0.5f32.to_bits(), 0.5f32.to_bits(), 0]]
        );
        assert_eq!(w.e.get(this, BGSTerrainNode::uiFadeStartTime), 777);
        assert!(w.e.get(this, BGSTerrainNode::bTexFading));
    }

    #[test]
    fn texture_fade_advances_then_ends() {
        let mut w = world();
        let this = w.node(1, 0, 0);
        w.e.set(this, BGSTerrainNode::ParentPosition, 1);
        w.e.set(this, BGSTerrainNode::uiFadeStartTime, 100);
        let chunk = w.object(0x30);
        let geometry = w.object(0x10);
        w.e.mem.set_u32(chunk.addr() + 0x2c, geometry.addr());
        w.e.set(this, BGSTerrainNode::pChunk, chunk);
        w.e.set_global(TIMER_OBJECT + 0x14, 350u32);
        w.e.call(0x006f_f3f0, &args![this]);
        assert_eq!(
            w.calls(GEOMETRY_SET_MORPH),
            vec![vec![
                geometry.addr(),
                0.5f32.to_bits(),
                0,
                0.25f32.to_bits()
            ]]
        );
        // Exactly the fade time is still a step; past it the morph ends.
        w.e.set_global(TIMER_OBJECT + 0x14, 1100u32);
        w.e.call(0x006f_f3f0, &args![this]);
        assert!(w.calls(END_TEXTURE_MORPH).is_empty());
        w.e.set_global(TIMER_OBJECT + 0x14, 1101u32);
        w.e.call(0x006f_f3f0, &args![this]);
        assert_eq!(w.calls(END_TEXTURE_MORPH), vec![vec![this.addr()]]);
        // Without geometry: nothing.
        w.e.mem.set_u32(chunk.addr() + 0x2c, 0);
        w.e.call(0x006f_f3f0, &args![this]);
        assert_eq!(w.calls(END_TEXTURE_MORPH).len(), 1);
    }
}
