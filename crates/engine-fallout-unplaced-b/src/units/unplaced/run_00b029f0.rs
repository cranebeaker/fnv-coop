//! `(unplaced)/run_00b029f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SpeedTree RT structures (the engine map puts the run in the SpeedTree
//! library's range; its functions have no names). Their identities come
//! from layouts, not names: `00b02ef0` constructs the 0xa0-byte object
//! that `BSTreeModel::InitFromBase` allocates, whose fields match the Xbox
//! PDB's `CSpeedTreeRT` (0xa0 bytes); `00b029f0`/`00b02ac0` build and clear
//! a 0x12c-byte structure laid out as `CSpeedTreeRT::SGeometry` (two
//! `SIndexed` of 0x38 bytes, two `SLeaf` of 0x3c, three `SBillboard` of
//! 0x14, with the alpha-test floats between them); `00b02e20`/`00b02e80`
//! do the same for the 0x1c-byte `CSpeedTreeRT::STextures`. Which of these
//! functions are the constructors or destructors in the source is not
//! confirmed, so they are `fn_<address>`.
//!
//! The code is unoptimized (every field written in source order). The
//! functions with an exception frame are partial: the frame and its unwind
//! states (and in `00b02ef0` the two catch handlers) are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `CSpeedTreeRT::SGeometry::SIndexed` (Xbox PDB), 0x38 bytes.
    pub struct SpeedTreeIndexed: 0x38 {
        /// `m_nDiscreteLodLevel` (Xbox PDB).
        0x00 m_nDiscreteLodLevel: i32,
        /// `m_usNumStrips` (Xbox PDB).
        0x04 m_usNumStrips: u16,
        /// `m_pStripLengths` (Xbox PDB).
        0x08 m_pStripLengths: u32,
        /// `m_pStrips` (Xbox PDB).
        0x0C m_pStrips: u32,
        /// `m_usVertexCount` (Xbox PDB).
        0x10 m_usVertexCount: u16,
        /// `m_pColors` (Xbox PDB).
        0x14 m_pColors: u32,
        /// `m_pNormals` (Xbox PDB).
        0x18 m_pNormals: u32,
        /// `m_pBinormals` (Xbox PDB).
        0x1C m_pBinormals: u32,
        /// `m_pTangents` (Xbox PDB).
        0x20 m_pTangents: u32,
        /// `m_pCoords` (Xbox PDB).
        0x24 m_pCoords: u32,
        /// `m_pTexCoords0` (Xbox PDB).
        0x28 m_pTexCoords0: u32,
        /// `m_pTexCoords1` (Xbox PDB).
        0x2C m_pTexCoords1: u32,
        /// `m_pWindWeights` (Xbox PDB).
        0x30 m_pWindWeights: u32,
        /// `m_pWindMatrixIndices` (Xbox PDB).
        0x34 m_pWindMatrixIndices: u32,
    }

    /// `CSpeedTreeRT::SGeometry::SLeaf` (Xbox PDB), 0x3c bytes.
    pub struct SpeedTreeLeaf: 0x3c {
        /// `m_bIsActive` (Xbox PDB).
        0x00 m_bIsActive: u8,
        /// `m_fAlphaTestValue` (Xbox PDB).
        0x04 m_fAlphaTestValue: f32,
        /// `m_nDiscreteLodLevel` (Xbox PDB).
        0x08 m_nDiscreteLodLevel: i32,
        /// `m_usLeafCount` (Xbox PDB).
        0x0C m_usLeafCount: u16,
        /// `m_pLeafMapIndices` (Xbox PDB).
        0x10 m_pLeafMapIndices: u32,
        /// `m_pLeafClusterIndices` (Xbox PDB).
        0x14 m_pLeafClusterIndices: u32,
        /// `m_pCenterCoords` (Xbox PDB).
        0x18 m_pCenterCoords: u32,
        /// `m_pLeafMapTexCoords` (Xbox PDB).
        0x1C m_pLeafMapTexCoords: u32,
        /// `m_pLeafMapCoords` (Xbox PDB).
        0x20 m_pLeafMapCoords: u32,
        /// `m_pColors` (Xbox PDB).
        0x24 m_pColors: u32,
        /// `m_pNormals` (Xbox PDB).
        0x28 m_pNormals: u32,
        /// `m_pBinormals` (Xbox PDB).
        0x2C m_pBinormals: u32,
        /// `m_pTangents` (Xbox PDB).
        0x30 m_pTangents: u32,
        /// `m_pWindWeights` (Xbox PDB).
        0x34 m_pWindWeights: u32,
        /// `m_pWindMatrixIndices` (Xbox PDB).
        0x38 m_pWindMatrixIndices: u32,
    }

    /// `CSpeedTreeRT::SGeometry::SBillboard` (Xbox PDB), 0x14 bytes.
    pub struct SpeedTreeBillboard: 0x14 {
        /// `m_bIsActive` (Xbox PDB).
        0x00 m_bIsActive: u8,
        /// `m_pTexCoords` (Xbox PDB).
        0x04 m_pTexCoords: u32,
        /// `m_pCoords` (Xbox PDB).
        0x08 m_pCoords: u32,
        /// `m_fAlphaTestValue` (Xbox PDB).
        0x0C m_fAlphaTestValue: f32,
        /// `m_usWhich` (Xbox PDB).
        0x10 m_usWhich: u16,
    }

    /// `CSpeedTreeRT::SGeometry` (Xbox PDB), 0x12c bytes; the members are
    /// at the offsets of [`GEOMETRY_PARTS`].
    pub struct SpeedTreeGeometry: 0x12c {
        /// `m_fBranchAlphaTestValue` (Xbox PDB).
        0x38 m_fBranchAlphaTestValue: f32,
        /// `m_fFrondAlphaTestValue` (Xbox PDB).
        0x74 m_fFrondAlphaTestValue: f32,
    }

    /// `CSpeedTreeRT::STextures` (Xbox PDB), 0x1c bytes.
    pub struct SpeedTreeTextures: 0x1c {
        /// `m_pBranchTextureFilename` (Xbox PDB).
        0x00 m_pBranchTextureFilename: u32,
        /// `m_uiLeafTextureCount` (Xbox PDB).
        0x04 m_uiLeafTextureCount: u32,
        /// `m_pLeafTextureFilenames` (Xbox PDB).
        0x08 m_pLeafTextureFilenames: u32,
        /// `m_uiFrondTextureCount` (Xbox PDB).
        0x0C m_uiFrondTextureCount: u32,
        /// `m_pFrondTextureFilenames` (Xbox PDB).
        0x10 m_pFrondTextureFilenames: u32,
        /// `m_pCompositeFilename` (Xbox PDB).
        0x14 m_pCompositeFilename: u32,
        /// `m_pSelfShadowFilename` (Xbox PDB).
        0x18 m_pSelfShadowFilename: u32,
    }

    /// `CSpeedTreeRT` (Xbox PDB), 0xa0 bytes.
    pub struct SpeedTree: 0xa0 {
        /// `m_pEngine` (Xbox PDB).
        0x00 m_pEngine: u32,
        /// `m_pBranchGeometry` (Xbox PDB).
        0x04 m_pBranchGeometry: u32,
        /// `m_pLeafGeometry` (Xbox PDB).
        0x08 m_pLeafGeometry: u32,
        /// `m_pLightingEngine` (Xbox PDB).
        0x0C m_pLightingEngine: u32,
        /// `m_pWindEngine` (Xbox PDB).
        0x10 m_pWindEngine: u32,
        /// `m_pSimpleBillboard` (Xbox PDB).
        0x14 m_pSimpleBillboard: u32,
        /// `m_eLeafLodMethod` (Xbox PDB).
        0x18 m_eLeafLodMethod: u32,
        /// `m_fLeafLodTransitionRadius` (Xbox PDB).
        0x1C m_fLeafLodTransitionRadius: f32,
        /// `m_fLeafLodCurveExponent` (Xbox PDB).
        0x20 m_fLeafLodCurveExponent: f32,
        /// `m_fLeafSizeIncreaseFactor` (Xbox PDB).
        0x24 m_fLeafSizeIncreaseFactor: f32,
        /// `m_fLeafTransitionFactor` (Xbox PDB).
        0x28 m_fLeafTransitionFactor: f32,
        /// `m_pLeafLodSizeFactors` (Xbox PDB).
        0x2C m_pLeafLodSizeFactors: u32,
        /// `m_pInstanceRefCount` (Xbox PDB).
        0x30 m_pInstanceRefCount: u32,
        /// `m_pInstanceData` (Xbox PDB).
        0x34 m_pInstanceData: u32,
        /// `m_pInstanceList` (Xbox PDB).
        0x38 m_pInstanceList: u32,
        /// `m_nFrondLevel` (Xbox PDB).
        0x3C m_nFrondLevel: i32,
        /// `m_pTreeSizes` (Xbox PDB).
        0x40 m_pTreeSizes: u32,
        /// `m_ucTargetAlphaValue` (Xbox PDB).
        0x44 m_ucTargetAlphaValue: u8,
        /// `m_bTreeComputed` (Xbox PDB).
        0x45 m_bTreeComputed: u8,
        /// `m_nBranchWindLevel` (Xbox PDB).
        0x48 m_nBranchWindLevel: i32,
        /// `m_pEmbeddedTexCoords` (Xbox PDB).
        0x4C m_pEmbeddedTexCoords: u32,
        /// `m_pProjectedShadow` (Xbox PDB).
        0x50 m_pProjectedShadow: u32,
        /// `m_usNum360Billboards` (Xbox PDB).
        0x54 m_usNum360Billboards: u16,
        /// `m_pCollisionObjects` (Xbox PDB).
        0x58 m_pCollisionObjects: u32,
        /// `m_pFrondEngine` (Xbox PDB).
        0x5C m_pFrondEngine: u32,
        /// `m_pFrondGeometry` (Xbox PDB).
        0x60 m_pFrondGeometry: u32,
        /// `m_usNumFrondLodLevels` (Xbox PDB).
        0x64 m_usNumFrondLodLevels: u16,
        /// `m_pUserData` (Xbox PDB).
        0x68 m_pUserData: u32,
        /// `m_b360Billboard` (Xbox PDB).
        0x6C m_b360Billboard: u8,
        /// `m_bHorizontalBillboard` (Xbox PDB).
        0x6D m_bHorizontalBillboard: u8,
    }
}

/// Offsets in [`SpeedTreeGeometry`] (Xbox PDB): `m_sBranches`, `m_sFronds`
/// (`SIndexed`), `m_sLeaves0`, `m_sLeaves1` (`SLeaf`), `m_sBillboard0`,
/// `m_sBillboard1`, `m_sHorizontalBillboard` (`SBillboard`).
const GEOMETRY_PARTS: [u32; 7] = [0x00, 0x3c, 0x78, 0xb4, 0xf0, 0x104, 0x118];

/// `CSpeedTreeRT::m_afHorizontalCoords` (Xbox PDB): 12 floats at +0x70.
const HORIZONTAL_COORDS: u32 = 0x70;
const HORIZONTAL_COORD_COUNT: u32 = 12;

/// `-1.0f` (`01012054`), the alpha-test default.
const MINUS_ONE: u32 = 0x0101_2054;
/// Float constants the tree constructor loads (`0.07`, `0.7`, `0.5` in the
/// exe: `0104eab4`, `01018238`, `01016248`).
const LEAF_LOD_TRANSITION_RADIUS: u32 = 0x0104_eab4;
const LEAF_LOD_CURVE_EXPONENT: u32 = 0x0101_8238;
const LEAF_SIZE_INCREASE_FACTOR: u32 = 0x0101_6248;

/// `MemoryManager` singleton and its `Allocate` / `Deallocate` (Xbox PDB).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

/// The member constructors `00b02ef0` runs on the blocks it allocates
/// (`this` in ECX), named here by the `CSpeedTreeRT` field (Xbox PDB layout)
/// the result is stored in; only `CFrondEngine::CFrondEngine` is named by
/// the engine map, the other classes are not confirmed.
const WIND_ENGINE_CONSTRUCTOR: u32 = 0x00b3_73c0;
const INDEXED_GEOMETRY_CONSTRUCTOR: u32 = 0x00b2_e810;
const TREE_ENGINE_CONSTRUCTOR: u32 = 0x00b2_ac80;
const LIGHTING_ENGINE_CONSTRUCTOR: u32 = 0x00b2_9fd0;
const LEAF_GEOMETRY_CONSTRUCTOR: u32 = 0x00b2_7660;
const SIMPLE_BILLBOARD_CONSTRUCTOR: u32 = 0x00b2_7060;
const INSTANCE_LIST_CONSTRUCTOR: u32 = 0x00b0_dce0;
const FROND_ENGINE_CONSTRUCTOR: u32 = 0x00b1_7fd0;

/// The global list the new tree is added to (`00b0ddc0`, `this` `011f8bdc`,
/// argument: the address of a word holding the tree), its critical section
/// (`011f8bc4`), the count incremented with it (`011f8b7c`) and the word
/// that receives the tree's `m_pLightingEngine` (`011f8bf8`).
const TREE_LIST: u32 = 0x011f_8bdc;
const TREE_LIST_ADD: u32 = 0x00b0_ddc0;
const TREE_LIST_SECTION: u32 = 0x011f_8bc4;
const TREE_COUNT: u32 = 0x011f_8b7c;
const LAST_LIGHTING_ENGINE: u32 = 0x011f_8bf8;

/// Import slots (KERNEL32).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

/// The float constant at `address`, as `FLD`/`FSTP` copy it.
fn constant(e: &Engine, address: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(address)))
}

// Translated from 00b029f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b029f0 - the exception frame (FS:[0] handler 00f239b8, unwind states 0 to 5 for the members built so far) is not translated.
/// Builds a `CSpeedTreeRT::SGeometry` (`this` in ECX, returns `this`):
/// the two `SIndexed` (`00b02b70`), each followed by its alpha-test value
/// set to -1.0 (`01012054`), the two `SLeaf` (`00b02c90`) and the three
/// `SBillboard` (`00b02dc0`).
pub fn fn_00b029f0(e: &mut Engine, this: Ptr<SpeedTreeGeometry>) -> Ptr<SpeedTreeGeometry> {
    let t = this.addr();
    fn_00b02b70(e, Ptr::new(t + GEOMETRY_PARTS[0]));
    let value = constant(e, MINUS_ONE);
    e.set(this, SpeedTreeGeometry::m_fBranchAlphaTestValue, value);
    fn_00b02b70(e, Ptr::new(t + GEOMETRY_PARTS[1]));
    let value = constant(e, MINUS_ONE);
    e.set(this, SpeedTreeGeometry::m_fFrondAlphaTestValue, value);
    fn_00b02c90(e, Ptr::new(t + GEOMETRY_PARTS[2]));
    fn_00b02c90(e, Ptr::new(t + GEOMETRY_PARTS[3]));
    fn_00b02dc0(e, Ptr::new(t + GEOMETRY_PARTS[4]));
    fn_00b02dc0(e, Ptr::new(t + GEOMETRY_PARTS[5]));
    fn_00b02dc0(e, Ptr::new(t + GEOMETRY_PARTS[6]));
    this
}

// Translated from 00b02ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b02ac0 - the exception frame (FS:[0] handler 00f239b8, unwind states 5 to 0) is not translated.
/// Clears a `CSpeedTreeRT::SGeometry` (`this` in ECX) in reverse member
/// order: the three `SBillboard` (`00b02e00`), the two `SLeaf`
/// (`00b02d40`), the two `SIndexed` (`00b02c10`).
pub fn fn_00b02ac0(e: &mut Engine, this: Ptr<SpeedTreeGeometry>) {
    let t = this.addr();
    fn_00b02e00(e, Ptr::new(t + GEOMETRY_PARTS[6]));
    fn_00b02e00(e, Ptr::new(t + GEOMETRY_PARTS[5]));
    fn_00b02e00(e, Ptr::new(t + GEOMETRY_PARTS[4]));
    fn_00b02d40(e, Ptr::new(t + GEOMETRY_PARTS[3]));
    fn_00b02d40(e, Ptr::new(t + GEOMETRY_PARTS[2]));
    fn_00b02c10(e, Ptr::new(t + GEOMETRY_PARTS[1]));
    fn_00b02c10(e, Ptr::new(t + GEOMETRY_PARTS[0]));
}

// Translated from 00b02b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds an `SIndexed` (`this` in ECX, returns `this`):
/// `m_nDiscreteLodLevel` -1, every other field 0.
pub fn fn_00b02b70(e: &mut Engine, this: Ptr<SpeedTreeIndexed>) -> Ptr<SpeedTreeIndexed> {
    e.set(this, SpeedTreeIndexed::m_nDiscreteLodLevel, -1);
    e.set(this, SpeedTreeIndexed::m_usNumStrips, 0);
    e.set(this, SpeedTreeIndexed::m_pStripLengths, 0);
    e.set(this, SpeedTreeIndexed::m_pStrips, 0);
    e.set(this, SpeedTreeIndexed::m_usVertexCount, 0);
    e.set(this, SpeedTreeIndexed::m_pColors, 0);
    e.set(this, SpeedTreeIndexed::m_pNormals, 0);
    e.set(this, SpeedTreeIndexed::m_pBinormals, 0);
    e.set(this, SpeedTreeIndexed::m_pTangents, 0);
    e.set(this, SpeedTreeIndexed::m_pCoords, 0);
    e.set(this, SpeedTreeIndexed::m_pTexCoords0, 0);
    e.set(this, SpeedTreeIndexed::m_pTexCoords1, 0);
    e.set(this, SpeedTreeIndexed::m_pWindWeights, 0);
    e.set(this, SpeedTreeIndexed::m_pWindMatrixIndices, 0);
    this
}

// Translated from 00b02c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears an `SIndexed`'s pointers (`this` in ECX): the per-vertex arrays
/// (+0x14 to +0x34), then `m_pStripLengths` and `m_pStrips`. The counts
/// and the level are kept.
pub fn fn_00b02c10(e: &mut Engine, this: Ptr<SpeedTreeIndexed>) {
    e.set(this, SpeedTreeIndexed::m_pColors, 0);
    e.set(this, SpeedTreeIndexed::m_pNormals, 0);
    e.set(this, SpeedTreeIndexed::m_pBinormals, 0);
    e.set(this, SpeedTreeIndexed::m_pTangents, 0);
    e.set(this, SpeedTreeIndexed::m_pCoords, 0);
    e.set(this, SpeedTreeIndexed::m_pTexCoords0, 0);
    e.set(this, SpeedTreeIndexed::m_pTexCoords1, 0);
    e.set(this, SpeedTreeIndexed::m_pWindWeights, 0);
    e.set(this, SpeedTreeIndexed::m_pWindMatrixIndices, 0);
    e.set(this, SpeedTreeIndexed::m_pStripLengths, 0);
    e.set(this, SpeedTreeIndexed::m_pStrips, 0);
}

// Translated from 00b02c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds an `SLeaf` (`this` in ECX, returns `this`): inactive,
/// `m_fAlphaTestValue` -1.0 (`01012054`), `m_nDiscreteLodLevel` -1, every
/// other field 0.
pub fn fn_00b02c90(e: &mut Engine, this: Ptr<SpeedTreeLeaf>) -> Ptr<SpeedTreeLeaf> {
    e.set(this, SpeedTreeLeaf::m_bIsActive, 0);
    let value = constant(e, MINUS_ONE);
    e.set(this, SpeedTreeLeaf::m_fAlphaTestValue, value);
    e.set(this, SpeedTreeLeaf::m_nDiscreteLodLevel, -1);
    e.set(this, SpeedTreeLeaf::m_usLeafCount, 0);
    e.set(this, SpeedTreeLeaf::m_pLeafMapIndices, 0);
    e.set(this, SpeedTreeLeaf::m_pLeafClusterIndices, 0);
    e.set(this, SpeedTreeLeaf::m_pCenterCoords, 0);
    e.set(this, SpeedTreeLeaf::m_pLeafMapTexCoords, 0);
    e.set(this, SpeedTreeLeaf::m_pLeafMapCoords, 0);
    e.set(this, SpeedTreeLeaf::m_pColors, 0);
    e.set(this, SpeedTreeLeaf::m_pNormals, 0);
    e.set(this, SpeedTreeLeaf::m_pBinormals, 0);
    e.set(this, SpeedTreeLeaf::m_pTangents, 0);
    e.set(this, SpeedTreeLeaf::m_pWindWeights, 0);
    e.set(this, SpeedTreeLeaf::m_pWindMatrixIndices, 0);
    this
}

// Translated from 00b02d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears an `SLeaf`'s pointers (`this` in ECX), in the exe's order: +0x10,
/// +0x14, +0x18, +0x24 to +0x30, +0x1c, +0x20, +0x34, +0x38. The flag,
/// alpha-test value, level and count are kept.
pub fn fn_00b02d40(e: &mut Engine, this: Ptr<SpeedTreeLeaf>) {
    e.set(this, SpeedTreeLeaf::m_pLeafMapIndices, 0);
    e.set(this, SpeedTreeLeaf::m_pLeafClusterIndices, 0);
    e.set(this, SpeedTreeLeaf::m_pCenterCoords, 0);
    e.set(this, SpeedTreeLeaf::m_pColors, 0);
    e.set(this, SpeedTreeLeaf::m_pNormals, 0);
    e.set(this, SpeedTreeLeaf::m_pBinormals, 0);
    e.set(this, SpeedTreeLeaf::m_pTangents, 0);
    e.set(this, SpeedTreeLeaf::m_pLeafMapTexCoords, 0);
    e.set(this, SpeedTreeLeaf::m_pLeafMapCoords, 0);
    e.set(this, SpeedTreeLeaf::m_pWindWeights, 0);
    e.set(this, SpeedTreeLeaf::m_pWindMatrixIndices, 0);
}

// Translated from 00b02dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds an `SBillboard` (`this` in ECX, returns `this`): inactive, null
/// coordinates, `m_fAlphaTestValue` -1.0 (`01012054`), `m_usWhich` 0.
pub fn fn_00b02dc0(e: &mut Engine, this: Ptr<SpeedTreeBillboard>) -> Ptr<SpeedTreeBillboard> {
    e.set(this, SpeedTreeBillboard::m_bIsActive, 0);
    e.set(this, SpeedTreeBillboard::m_pTexCoords, 0);
    e.set(this, SpeedTreeBillboard::m_pCoords, 0);
    let value = constant(e, MINUS_ONE);
    e.set(this, SpeedTreeBillboard::m_fAlphaTestValue, value);
    e.set(this, SpeedTreeBillboard::m_usWhich, 0);
    this
}

// Translated from 00b02e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears an `SBillboard`'s two pointers (`this` in ECX).
pub fn fn_00b02e00(e: &mut Engine, this: Ptr<SpeedTreeBillboard>) {
    e.set(this, SpeedTreeBillboard::m_pTexCoords, 0);
    e.set(this, SpeedTreeBillboard::m_pCoords, 0);
}

// Translated from 00b02e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds an `STextures` (`this` in ECX, returns `this`): every field 0.
pub fn fn_00b02e20(e: &mut Engine, this: Ptr<SpeedTreeTextures>) -> Ptr<SpeedTreeTextures> {
    e.set(this, SpeedTreeTextures::m_pBranchTextureFilename, 0);
    e.set(this, SpeedTreeTextures::m_uiLeafTextureCount, 0);
    e.set(this, SpeedTreeTextures::m_pLeafTextureFilenames, 0);
    e.set(this, SpeedTreeTextures::m_uiFrondTextureCount, 0);
    e.set(this, SpeedTreeTextures::m_pFrondTextureFilenames, 0);
    e.set(this, SpeedTreeTextures::m_pCompositeFilename, 0);
    e.set(this, SpeedTreeTextures::m_pSelfShadowFilename, 0);
    this
}

// Translated from 00b02e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases an `STextures` (`this` in ECX): the leaf and frond filename
/// arrays are freed (`MemoryManager::Deallocate`, null included) and
/// cleared, then the branch, composite and self-shadow filenames are
/// cleared (the strings themselves are not freed here).
pub fn fn_00b02e80(e: &mut Engine, this: Ptr<SpeedTreeTextures>) {
    let leaves = e.get(this, SpeedTreeTextures::m_pLeafTextureFilenames);
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, leaves]);
    e.set(this, SpeedTreeTextures::m_pLeafTextureFilenames, 0);
    let fronds = e.get(this, SpeedTreeTextures::m_pFrondTextureFilenames);
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, fronds]);
    e.set(this, SpeedTreeTextures::m_pFrondTextureFilenames, 0);
    e.set(this, SpeedTreeTextures::m_pBranchTextureFilename, 0);
    e.set(this, SpeedTreeTextures::m_pCompositeFilename, 0);
    e.set(this, SpeedTreeTextures::m_pSelfShadowFilename, 0);
}

/// `new T(args)` as `00b02ef0` writes it: `MemoryManager::Allocate(size)`,
/// then the constructor on a non-null block (its `EAX` is the result), or
/// null.
fn allocate_and_construct(e: &mut Engine, size: u32, constructor: u32, args: &[u32]) -> u32 {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    if block == 0 {
        return 0;
    }
    let mut words = vec![block];
    words.extend_from_slice(args);
    e.call(constructor, &words).u32()
}

// Translated from 00b02ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b02ef0 - the exception frame (FS:[0] handler 00f23a59, unwind states 0 to 9 that free a block whose constructor threw) and the two catch handlers (00b03582 and 00b0361a, which format the caught exception's message) are not translated.
/// Builds a `CSpeedTreeRT` (`this` in ECX, returns `this`). The fields
/// first: the six engine pointers 0, `m_eLeafLodMethod` 1, the three leaf
/// LOD floats from `0104eab4`, `01018238`, `01016248`,
/// `m_fLeafTransitionFactor` 0.0, `m_nFrondLevel` and `m_nBranchWindLevel`
/// -1, `m_ucTargetAlphaValue` 0x54, the rest 0 (`m_pInstanceList`, `+0x60`
/// and the coordinates are set later). Then the members are allocated
/// (`MemoryManager::Allocate`) and constructed, each null when its
/// allocation fails, stored in the fields named: `m_pWindEngine` (+0x10,
/// `00b373c0`, 0x44 bytes), `m_pBranchGeometry` (+0x04, `00b2e810`, 400
/// bytes, with the wind engine and 0), `m_pEngine` (+0x00, `00b2ac80`,
/// 0x128 bytes, with the branch geometry), `m_pLightingEngine` (+0x0c,
/// `00b29fd0`, 0xb0), `m_pLeafGeometry` (+0x08, `00b27660`, 0x30, with the
/// wind engine), `m_pSimpleBillboard` (+0x14, `00b27060`, 0x34). The tree sizes (+0x40, 0x1c bytes)
/// and the instance count (+0x30, 4 bytes, set to 1) are allocated without
/// a null check; the instance list (+0x38, `00b0dce0`, 0x18 bytes; its
/// result is not used, the block is stored), the frond engine (+0x5c,
/// `CFrondEngine::CFrondEngine` `00b17fd0`, 0x84) and the frond geometry
/// (+0x60, `00b2e810`, 400 bytes, with the wind engine and 1) follow. Inside
/// the critical section `011f8bc4` the tree is added to the list at
/// `011f8bdc` (`00b0ddc0`, with the address of a word holding the tree) and
/// the count `011f8b7c` incremented; `011f8bf8` takes the lighting engine.
/// Finally the tree sizes get 0, 0, 0, 1, 1, 1, 1 and the twelve
/// horizontal coordinates 0.0.
///
/// The member names follow the Xbox PDB layout; the constructors' classes
/// other than `CFrondEngine` are not confirmed.
pub fn fn_00b02ef0(e: &mut Engine, this: Ptr<SpeedTree>) -> Ptr<SpeedTree> {
    e.set(this, SpeedTree::m_pEngine, 0);
    e.set(this, SpeedTree::m_pBranchGeometry, 0);
    e.set(this, SpeedTree::m_pLeafGeometry, 0);
    e.set(this, SpeedTree::m_pLightingEngine, 0);
    e.set(this, SpeedTree::m_pWindEngine, 0);
    e.set(this, SpeedTree::m_pSimpleBillboard, 0);
    e.set(this, SpeedTree::m_eLeafLodMethod, 1);
    let value = constant(e, LEAF_LOD_TRANSITION_RADIUS);
    e.set(this, SpeedTree::m_fLeafLodTransitionRadius, value);
    let value = constant(e, LEAF_LOD_CURVE_EXPONENT);
    e.set(this, SpeedTree::m_fLeafLodCurveExponent, value);
    let value = constant(e, LEAF_SIZE_INCREASE_FACTOR);
    e.set(this, SpeedTree::m_fLeafSizeIncreaseFactor, value);
    e.set(this, SpeedTree::m_fLeafTransitionFactor, 0.0);
    e.set(this, SpeedTree::m_pLeafLodSizeFactors, 0);
    e.set(this, SpeedTree::m_pInstanceRefCount, 0);
    e.set(this, SpeedTree::m_pInstanceData, 0);
    e.set(this, SpeedTree::m_nFrondLevel, -1);
    e.set(this, SpeedTree::m_pTreeSizes, 0);
    e.set(this, SpeedTree::m_ucTargetAlphaValue, 0x54);
    e.set(this, SpeedTree::m_bTreeComputed, 0);
    e.set(this, SpeedTree::m_nBranchWindLevel, -1);
    e.set(this, SpeedTree::m_pEmbeddedTexCoords, 0);
    e.set(this, SpeedTree::m_pProjectedShadow, 0);
    e.set(this, SpeedTree::m_usNum360Billboards, 0);
    e.set(this, SpeedTree::m_pCollisionObjects, 0);
    e.set(this, SpeedTree::m_pFrondEngine, 0);
    e.set(this, SpeedTree::m_usNumFrondLodLevels, 0);
    e.set(this, SpeedTree::m_pUserData, 0);
    e.set(this, SpeedTree::m_b360Billboard, 0);
    e.set(this, SpeedTree::m_bHorizontalBillboard, 0);

    let wind = allocate_and_construct(e, 0x44, WIND_ENGINE_CONSTRUCTOR, &[]);
    e.set(this, SpeedTree::m_pWindEngine, wind);
    let wind = e.get(this, SpeedTree::m_pWindEngine);
    let branches = allocate_and_construct(e, 400, INDEXED_GEOMETRY_CONSTRUCTOR, &[wind, 0]);
    e.set(this, SpeedTree::m_pBranchGeometry, branches);
    let branches = e.get(this, SpeedTree::m_pBranchGeometry);
    let engine = allocate_and_construct(e, 0x128, TREE_ENGINE_CONSTRUCTOR, &[branches]);
    e.set(this, SpeedTree::m_pEngine, engine);
    let lighting = allocate_and_construct(e, 0xb0, LIGHTING_ENGINE_CONSTRUCTOR, &[]);
    e.set(this, SpeedTree::m_pLightingEngine, lighting);
    let wind = e.get(this, SpeedTree::m_pWindEngine);
    let leaf_geometry = allocate_and_construct(e, 0x30, LEAF_GEOMETRY_CONSTRUCTOR, &[wind]);
    e.set(this, SpeedTree::m_pLeafGeometry, leaf_geometry);
    let billboard = allocate_and_construct(e, 0x34, SIMPLE_BILLBOARD_CONSTRUCTOR, &[]);
    e.set(this, SpeedTree::m_pSimpleBillboard, billboard);

    let sizes = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x1cu32])
        .u32();
    e.set(this, SpeedTree::m_pTreeSizes, sizes);
    let count = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 4u32])
        .u32();
    e.set(this, SpeedTree::m_pInstanceRefCount, count);
    let count = e.get(this, SpeedTree::m_pInstanceRefCount);
    e.mem.set_u32(count, 1);

    let list = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x18u32])
        .u32();
    if list != 0 {
        e.call(INSTANCE_LIST_CONSTRUCTOR, &args![list]);
    }
    e.set(this, SpeedTree::m_pInstanceList, list);
    let fronds = allocate_and_construct(e, 0x84, FROND_ENGINE_CONSTRUCTOR, &[]);
    e.set(this, SpeedTree::m_pFrondEngine, fronds);
    let wind = e.get(this, SpeedTree::m_pWindEngine);
    let frond_geometry = allocate_and_construct(e, 400, INDEXED_GEOMETRY_CONSTRUCTOR, &[wind, 1]);
    e.set(this, SpeedTree::m_pFrondGeometry, frond_geometry);

    e.call(ENTER_CRITICAL_SECTION, &args![TREE_LIST_SECTION]);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), this.addr());
        e.call(TREE_LIST_ADD, &args![TREE_LIST, local]);
    });
    let trees = e.mem.u32(TREE_COUNT);
    e.mem.set_u32(TREE_COUNT, trees.wrapping_add(1));
    e.call(LEAVE_CRITICAL_SECTION, &args![TREE_LIST_SECTION]);
    let lighting = e.get(this, SpeedTree::m_pLightingEngine);
    e.mem.set_u32(LAST_LIGHTING_ENGINE, lighting);

    for (k, value) in [0.0f32, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]
        .into_iter()
        .enumerate()
    {
        let sizes = e.get(this, SpeedTree::m_pTreeSizes);
        e.mem.set_f32(sizes + 4 * k as u32, value);
    }
    for k in 0..HORIZONTAL_COORD_COUNT {
        e.mem.set_f32(this.addr() + HORIZONTAL_COORDS + 4 * k, 0.0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b029f0,
            fn_00b029f0(Ptr<SpeedTreeGeometry>) -> Ptr<SpeedTreeGeometry>
        ),
        entry!(0x00b02ac0, fn_00b02ac0(Ptr<SpeedTreeGeometry>)),
        entry!(
            0x00b02b70,
            fn_00b02b70(Ptr<SpeedTreeIndexed>) -> Ptr<SpeedTreeIndexed>
        ),
        entry!(0x00b02c10, fn_00b02c10(Ptr<SpeedTreeIndexed>)),
        entry!(
            0x00b02c90,
            fn_00b02c90(Ptr<SpeedTreeLeaf>) -> Ptr<SpeedTreeLeaf>
        ),
        entry!(0x00b02d40, fn_00b02d40(Ptr<SpeedTreeLeaf>)),
        entry!(
            0x00b02dc0,
            fn_00b02dc0(Ptr<SpeedTreeBillboard>) -> Ptr<SpeedTreeBillboard>
        ),
        entry!(0x00b02e00, fn_00b02e00(Ptr<SpeedTreeBillboard>)),
        entry!(
            0x00b02e20,
            fn_00b02e20(Ptr<SpeedTreeTextures>) -> Ptr<SpeedTreeTextures>
        ),
        entry!(0x00b02e80, fn_00b02e80(Ptr<SpeedTreeTextures>)),
        entry!(0x00b02ef0, fn_00b02ef0(Ptr<SpeedTree>) -> Ptr<SpeedTree>),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const MINUS_ONE_BITS: u32 = 0xbf80_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_u32(MINUS_ONE, MINUS_ONE_BITS);
        e
    }

    /// A block of `size` bytes filled with 0xaa.
    fn dirty(e: &mut Engine, size: u32) -> u32 {
        let p = e.mem.alloc(size);
        e.mem.write(p, &vec![0xaa; size as usize]);
        p
    }

    #[test]
    fn x87_copy_quiets_only_signalling_nans() {
        assert_eq!(x87_quiet_bits(0x7f80_0001), 0x7fc0_0001);
        assert_eq!(x87_quiet_bits(0xff80_0001), 0xffc0_0001);
        assert_eq!(x87_quiet_bits(0x7fc0_0000), 0x7fc0_0000);
        assert_eq!(x87_quiet_bits(0x7f80_0000), 0x7f80_0000);
        assert_eq!(x87_quiet_bits(MINUS_ONE_BITS), MINUS_ONE_BITS);
    }

    #[test]
    fn geometry_is_built_and_cleared_member_by_member() {
        let mut e = engine();
        let g = dirty(&mut e, 0x12c);
        let r = e.call(0x00b0_29f0, &args![g]).u32();
        assert_eq!(r, g);
        // SIndexed: level -1, rest 0; alpha tests -1.0.
        for base in [0x00, 0x3c] {
            assert_eq!(e.mem.i32(g + base), -1);
            assert_eq!(e.mem.u16(g + base + 4), 0);
            assert_eq!(e.mem.u32(g + base + 0x34), 0);
        }
        assert_eq!(e.mem.u32(g + 0x38), MINUS_ONE_BITS);
        assert_eq!(e.mem.u32(g + 0x74), MINUS_ONE_BITS);
        // SLeaf: inactive, alpha -1.0, level -1.
        for base in [0x78, 0xb4] {
            assert_eq!(e.mem.u8(g + base), 0);
            assert_eq!(e.mem.u32(g + base + 4), MINUS_ONE_BITS);
            assert_eq!(e.mem.i32(g + base + 8), -1);
            assert_eq!(e.mem.u32(g + base + 0x38), 0);
        }
        // SBillboard: alpha -1.0, which 0.
        for base in [0xf0, 0x104, 0x118] {
            assert_eq!(e.mem.u32(g + base + 0xc), MINUS_ONE_BITS);
            assert_eq!(e.mem.u16(g + base + 0x10), 0);
        }
        // Padding is untouched.
        assert_eq!(e.mem.u16(g + 6), 0xaaaa);

        // The clear keeps counts, flags and alpha values.
        let g = dirty(&mut e, 0x12c);
        e.call(0x00b0_2ac0, &args![g]);
        for base in [0x00, 0x3c] {
            assert_eq!(e.mem.u32(g + base), 0xaaaa_aaaa);
            assert_eq!(e.mem.u32(g + base + 8), 0);
            assert_eq!(e.mem.u32(g + base + 0xc), 0);
            assert_eq!(e.mem.u16(g + base + 0x10), 0xaaaa);
            for off in (0x14..=0x34).step_by(4) {
                assert_eq!(e.mem.u32(g + base + off), 0);
            }
        }
        for base in [0x78, 0xb4] {
            assert_eq!(e.mem.u32(g + base + 4), 0xaaaa_aaaa);
            assert_eq!(e.mem.u32(g + base + 0xc), 0xaaaa_aaaa);
            for off in (0x10..=0x38).step_by(4) {
                assert_eq!(e.mem.u32(g + base + off), 0);
            }
        }
        for base in [0xf0, 0x104, 0x118] {
            assert_eq!(e.mem.u32(g + base), 0xaaaa_aaaa);
            assert_eq!(e.mem.u32(g + base + 4), 0);
            assert_eq!(e.mem.u32(g + base + 8), 0);
            assert_eq!(e.mem.u32(g + base + 0xc), 0xaaaa_aaaa);
        }
    }

    #[test]
    fn member_constructors_return_this_and_quiet_the_constant() {
        let mut e = engine();
        e.mem.set_u32(MINUS_ONE, 0x7f80_0002);
        let leaf = dirty(&mut e, 0x3c);
        assert_eq!(e.call(0x00b0_2c90, &args![leaf]).u32(), leaf);
        assert_eq!(e.mem.u32(leaf + 4), 0x7fc0_0002);
        let billboard = dirty(&mut e, 0x14);
        assert_eq!(e.call(0x00b0_2dc0, &args![billboard]).u32(), billboard);
        assert_eq!(e.mem.u8(billboard), 0);
        assert_eq!(e.mem.u32(billboard + 0xc), 0x7fc0_0002);
        let indexed = dirty(&mut e, 0x38);
        assert_eq!(e.call(0x00b0_2b70, &args![indexed]).u32(), indexed);
        assert_eq!(e.mem.u32(indexed), 0xffff_ffff);
        // Single-member clears.
        let indexed = dirty(&mut e, 0x38);
        e.call(0x00b0_2c10, &args![indexed]);
        assert_eq!(e.mem.u32(indexed), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(indexed + 8), 0);
        let leaf = dirty(&mut e, 0x3c);
        e.call(0x00b0_2d40, &args![leaf]);
        assert_eq!(e.mem.u32(leaf + 8), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(leaf + 0x1c), 0);
        let billboard = dirty(&mut e, 0x14);
        e.call(0x00b0_2e00, &args![billboard]);
        assert_eq!(e.mem.u32(billboard + 8), 0);
        assert_eq!(e.mem.u16(billboard + 0x10), 0xaaaa);
    }

    #[test]
    fn textures_are_zeroed_and_released() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let t = dirty(&mut e, 0x1c);
        assert_eq!(e.call(0x00b0_2e20, &args![t]).u32(), t);
        for off in (0..0x1c).step_by(4) {
            assert_eq!(e.mem.u32(t + off), 0);
        }
        let t = dirty(&mut e, 0x1c);
        e.mem.set_u32(t + 8, 0x1111);
        e.mem.set_u32(t + 0x10, 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x00b0_2e80, &args![t]);
        let frees: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            frees,
            vec![vec![MEMORY_MANAGER, 0x1111], vec![MEMORY_MANAGER, 0x2222]]
        );
        for off in [0x0, 0x8, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(t + off), 0);
        }
        assert_eq!(e.mem.u32(t + 4), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(t + 0xc), 0xaaaa_aaaa);
    }

    /// Engine for the tree constructor: constants mapped, allocation by
    /// `Mem` except for the sizes in `failing`, every member constructor
    /// recording its words and returning `this`.
    fn tree_engine(failing: &'static [u32]) -> (Engine, CallLog) {
        let mut e = engine();
        e.map(0x0104_e000, 0x1000);
        e.map(0x0101_8000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x011f_8000, 0x1000);
        e.mem.set_f32(LEAF_LOD_TRANSITION_RADIUS, 0.07);
        e.mem.set_f32(LEAF_LOD_CURVE_EXPONENT, 0.7);
        e.mem.set_f32(LEAF_SIZE_INCREASE_FACTOR, 0.5);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |e, a| {
            let eax = if failing.contains(&a[1]) {
                0
            } else {
                e.mem.alloc(a[1])
            };
            Ret {
                eax,
                ..Ret::default()
            }
        });
        let log: CallLog = Default::default();
        for address in [
            WIND_ENGINE_CONSTRUCTOR,
            INDEXED_GEOMETRY_CONSTRUCTOR,
            TREE_ENGINE_CONSTRUCTOR,
            LIGHTING_ENGINE_CONSTRUCTOR,
            LEAF_GEOMETRY_CONSTRUCTOR,
            SIMPLE_BILLBOARD_CONSTRUCTOR,
            INSTANCE_LIST_CONSTRUCTOR,
            FROND_ENGINE_CONSTRUCTOR,
        ] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                // 00b0dce0's result is ignored: return something else.
                let eax = if address == INSTANCE_LIST_CONSTRUCTOR {
                    0x1234
                } else {
                    a[0]
                };
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }
        let list_log = log.clone();
        e.register_double(TREE_LIST_ADD, move |e, a| {
            list_log
                .borrow_mut()
                .push((TREE_LIST_ADD, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        (e, log)
    }

    #[test]
    fn tree_constructor_builds_and_registers_the_members() {
        let (mut e, log) = tree_engine(&[]);
        let t = dirty(&mut e, 0xa0);
        let r = e.call(0x00b0_2ef0, &args![t]).u32();
        assert_eq!(r, t);
        let tree = Ptr::<SpeedTree>::new(t);
        let wind = e.get(tree, SpeedTree::m_pWindEngine);
        let branches = e.get(tree, SpeedTree::m_pBranchGeometry);
        let frond_geometry = e.get(tree, SpeedTree::m_pFrondGeometry);
        let log = log.borrow();
        let construct = |address: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(a, _)| *a == address)
                .map(|(_, w)| w.clone())
                .collect()
        };
        assert_eq!(construct(WIND_ENGINE_CONSTRUCTOR), vec![vec![wind]]);
        assert_eq!(
            construct(INDEXED_GEOMETRY_CONSTRUCTOR),
            vec![vec![branches, wind, 0], vec![frond_geometry, wind, 1]]
        );
        let engine = e.get(tree, SpeedTree::m_pEngine);
        assert_eq!(
            construct(TREE_ENGINE_CONSTRUCTOR),
            vec![vec![engine, branches]]
        );
        let leaf_geometry = e.get(tree, SpeedTree::m_pLeafGeometry);
        assert_eq!(
            construct(LEAF_GEOMETRY_CONSTRUCTOR),
            vec![vec![leaf_geometry, wind]]
        );
        let list = e.get(tree, SpeedTree::m_pInstanceList);
        assert_ne!(list, 0x1234);
        assert_eq!(construct(INSTANCE_LIST_CONSTRUCTOR), vec![vec![list]]);
        assert_eq!(construct(TREE_LIST_ADD), vec![vec![TREE_LIST, t]]);
        assert_eq!(e.mem.u32(TREE_COUNT), 1);
        assert_eq!(
            e.mem.u32(LAST_LIGHTING_ENGINE),
            e.get(tree, SpeedTree::m_pLightingEngine)
        );
        // Fields.
        assert_eq!(e.get(tree, SpeedTree::m_eLeafLodMethod), 1);
        assert_eq!(e.get(tree, SpeedTree::m_fLeafLodTransitionRadius), 0.07);
        assert_eq!(e.get(tree, SpeedTree::m_fLeafLodCurveExponent), 0.7);
        assert_eq!(e.get(tree, SpeedTree::m_fLeafSizeIncreaseFactor), 0.5);
        assert_eq!(e.get(tree, SpeedTree::m_fLeafTransitionFactor), 0.0);
        assert_eq!(e.get(tree, SpeedTree::m_nFrondLevel), -1);
        assert_eq!(e.get(tree, SpeedTree::m_nBranchWindLevel), -1);
        assert_eq!(e.get(tree, SpeedTree::m_ucTargetAlphaValue), 0x54);
        assert_eq!(e.get(tree, SpeedTree::m_usNum360Billboards), 0);
        assert_eq!(e.get(tree, SpeedTree::m_usNumFrondLodLevels), 0);
        // +0x46 (padding) is not written.
        assert_eq!(e.mem.u16(t + 0x46), 0xaaaa);
        let count = e.get(tree, SpeedTree::m_pInstanceRefCount);
        assert_eq!(e.mem.u32(count), 1);
        let sizes = e.get(tree, SpeedTree::m_pTreeSizes);
        let values: Vec<f32> = (0..7).map(|k| e.mem.f32(sizes + 4 * k)).collect();
        assert_eq!(values, vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]);
        for k in 0..12 {
            assert_eq!(e.mem.u32(t + 0x70 + 4 * k), 0);
        }
    }

    #[test]
    fn tree_constructor_leaves_failed_members_null() {
        let (mut e, log) = tree_engine(&[0x44, 0x18, 0x84]);
        let t = dirty(&mut e, 0xa0);
        e.call(0x00b0_2ef0, &args![t]);
        let tree = Ptr::<SpeedTree>::new(t);
        assert_eq!(e.get(tree, SpeedTree::m_pWindEngine), 0);
        assert_eq!(e.get(tree, SpeedTree::m_pInstanceList), 0);
        assert_eq!(e.get(tree, SpeedTree::m_pFrondEngine), 0);
        let log = log.borrow();
        assert!(!log.iter().any(|(a, _)| *a == WIND_ENGINE_CONSTRUCTOR
            || *a == INSTANCE_LIST_CONSTRUCTOR
            || *a == FROND_ENGINE_CONSTRUCTOR));
        // The branch geometry gets the null wind engine.
        let branches = e.get(tree, SpeedTree::m_pBranchGeometry);
        assert!(log
            .iter()
            .any(|(a, w)| *a == INDEXED_GEOMETRY_CONSTRUCTOR && *w == vec![branches, 0, 0]));
    }
}
