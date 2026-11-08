//! The classes of the shader family, once each, at their PC offsets. Names marked (Xbox PDB) are the
//! prototype's, as the engine crate's layouts already use them; the others say what the code does with
//! the field. A class read only as far as these functions read it says so; its size is then where the
//! last field read ends. Arrays are read element by element ([`super::elem`]).

use crate::image;
use engine_core::ptr::{Inline, Ptr};

image! {
    /// `BSShaderProperty::RenderPass` (the engine map's name), as far as read: the geometry, the pass
    /// type, the byte the accumulator registers it by and the byte the batch renderer groups it by.
    pub struct RenderPass / RenderPassImage: 0x8 {
        0x0 pGeometry: Ptr<NiGeometry>,
        /// The pass type (`usPassEnum`, the engine crate's name).
        0x4 usPassEnum: u16,
        /// Which group the accumulator's register function files the pass under (1 to 14).
        0x6 ucRegisterKind: u8,
        /// The batch renderer keeps passes with this byte set ahead of the others and renders a run of
        /// passes with the same byte at once.
        0x7 ucSortFlag: u8,
    }

    /// A node of the batch renderer's pass lists: next, previous, the pass.
    pub struct PassNode / PassNodeImage: 0xc {
        0x0 pNext: u32,
        0x4 pPrev: u32,
        0x8 pItem: u32,
    }

    /// A pass list (0x14 bytes): the nodes in use, the nodes kept for reuse, the count.
    pub struct PassList / PassListImage: 0x14 {
        0x00 unknown_00: u32,
        0x04 pHead: u32,
        0x08 pTail: u32,
        /// The chain of nodes kept for reuse (returned to the global pool by `00b63e90`).
        0x0c pSpare: u32,
        0x10 uiCount: u32,
    }

    /// One pass type's group in a batch renderer: five pass lists and the total.
    pub struct PassGroup / PassGroupImage: 0x68 {
        0x00 kLists: [u32; 25],
        0x64 uiCount: u32,
    }

    /// The holder `00b9abc0` adds a pass to: a flag (bit 0: keep passes in its own list), the count
    /// added, the size its batch renderer is made with, the batch renderer made on first use, the list.
    pub struct PassHolder / PassHolderImage: 0x20 {
        0x00 ucFlags: u8,
        0x01 padding_01: u8,
        0x02 usCount: u16,
        0x04 uiRendererSize: u32,
        0x08 pRenderer: Ptr<BSBatchRenderer>,
        0x0c kList: Inline<PassList> = PassListImage,
    }

    /// An entry of the active pass type list: the type, the next entry.
    pub struct ActivePass / ActivePassImage: 0x8 {
        0x0 usItem: u16,
        0x2 padding_02: [u8; 2],
        0x4 pNext: u32,
    }

    /// `BSBatchRenderer` (Xbox PDB), 0x98 bytes.
    pub struct BSBatchRenderer / BSBatchRendererImage: 0x98 {
        0x00 vtable: u32,
        0x04 unknown_04: u32,
        /// `pRenderPassA` (Xbox PDB): the pass groups, 0x68 bytes each.
        0x08 pRenderPassA: u32,
        /// The map from pass type to group index (an inlined `NiTMap`).
        0x0c RenderPassMap_vtable: u32,
        0x10 RenderPassMap_m_uiHashSize: u32,
        0x14 RenderPassMap_m_ppkHashTable: u32,
        0x18 RenderPassMap_m_uiCount: u32,
        0x1c uiRenderPassCount: u32,
        /// The lowest and highest pass type `00b99900` takes from the active list.
        0x20 uiFirstPass: u32,
        0x24 uiLastPass: u32,
        0x28 pInstanceRenderer: u32,
        0x2c kActivePassIndexList: Inline<ActivePass> = ActivePassImage,
        0x34 iGroupingAlphas: i32,
        0x38 ucGroupingActive: u8,
        0x39 padding_39: [u8; 3],
        /// The pass holders by register group (`00b9b3b0`).
        0x3c pGroups: [u32; 13],
        /// The alpha holders by flag, plain (`00b9b3d0`) and while grouping alphas.
        0x70 pAlphaGroups: [u32; 8],
        0x90 pGroupedAlphas: [u32; 2],
    }

    /// `BSShaderAccumulator` (Xbox PDB), as far as read.
    pub struct BSShaderAccumulator / BSShaderAccumulatorImage: 0x1ac {
        0x000 vtable: u32,
        0x004 unknown_004: u32,
        /// `m_pkCamera` (Gamebryo's `NiAccumulator`): nothing is drawn without one.
        0x008 m_pkCamera: u32,
        /// The geometry list `00b64f90` empties when there is no list at +0x18: head, tail, count.
        0x00c kGeometry_head: u32,
        0x010 kGeometry_tail: u32,
        0x014 kGeometry_count: u32,
        0x018 pGeometryList: u32,
        /// The sorted geometry: its count, its items, the one being drawn.
        0x01c uiSortedCount: u32,
        0x020 unknown_020: u32,
        0x024 ppSorted: u32,
        0x028 unknown_028: u32,
        0x02c uiSortedCurrent: u32,
        0x030 unknown_030: [u8; 10],
        0x03a bDoDepthPasses: u8,
        0x03b unknown_03b: [u8; 0x49],
        0x084 bAccumulate: u8,
        0x085 b1stPerson: u8,
        0x086 unknown_086: [u8; 0xde],
        /// The register function answers at once for a shader of type 0xd while this is set.
        0x164 ucSkipType13: u8,
        0x165 unknown_165: u8,
        /// With this set, a pass whose bound radius is below the `int` at +0x168 is flagged.
        0x166 ucRadiusTest: u8,
        0x167 unknown_167: u8,
        0x168 iRadiusLimit: i32,
        0x16c bRenderDecals: u8,
        0x16d unknown_16d: [u8; 7],
        /// `pBatchRenderer` by thread, `uiBatchRendererCount` of them.
        0x174 pBatchRenderer: [u32; 6],
        0x18c uiBatchRendererCount: u32,
        0x190 unknown_190: [u32; 3],
        0x19c eRenderMode: u32,
        0x1a0 unknown_1a0: u32,
        /// The pass `00b64f90` draws each sorted geometry through.
        0x1a4 kSortedPass: Inline<RenderPass> = RenderPassImage,
    }

    /// `NiGeometry` (Xbox PDB), as far as read, with Bethesda's shader at +0xc0.
    pub struct NiGeometry / NiGeometryImage: 0xc4 {
        0x00 vtable: u32,
        0x04 unknown_04: [u32; 7],
        0x20 m_kWorldBound: u32,
        0x24 unknown_24: [u32; 17],
        /// `m_kWorld` (Xbox PDB): the world transform, 0x34 bytes.
        0x68 m_kWorld: [u32; 13],
        /// `m_PropertyState` (Xbox PDB): its first word is the alpha property.
        0x9c m_PropertyState: [u32; 3],
        0xa8 pShaderProperty: Ptr<BSShaderProperty>,
        /// A property whose flags (+0x18) say how the geometry pairs its passes.
        0xac pShadeProperty: Ptr<NiProperty>,
        0xb0 unknown_b0: [u32; 2],
        0xb8 m_spModelData: Ptr<NiGeometryData>,
        0xbc m_spSkinInstance: Ptr<NiSkinInstance>,
        0xc0 m_spShader: u32,
    }

    /// A property's flags word, as far as read (`NiAlphaProperty` and the like).
    pub struct NiProperty / NiPropertyImage: 0x1c {
        0x00 vtable: u32,
        0x04 unknown_04: [u32; 5],
        0x18 m_usFlags: u16,
        0x1a unknown_1a: [u8; 2],
    }

    /// `BSShaderProperty` (Xbox PDB), as far as read.
    pub struct BSShaderProperty / BSShaderPropertyImage: 0x40 {
        0x00 vtable: u32,
        0x04 unknown_04: [u32; 6],
        /// -1 marks a property `00be1ff0` treats as none.
        0x1c unknown_1c: u32,
        0x20 ulFlags: u32,
        0x24 ulFlags2: u32,
        0x28 fAlpha: f32,
        0x2c fFadeAlpha: f32,
        0x30 unknown_30: [u32; 2],
        0x38 iLastRenderPassState: u32,
        0x3c pRenderPassList: u32,
    }

    /// `NiGeometryData` (Xbox PDB), as far as read.
    pub struct NiGeometryData / NiGeometryDataImage: 0x38 {
        0x00 vtable: u32,
        0x04 unknown_04: [u32; 2],
        0x0c unknown_0c: u16,
        /// The low 12 bits are cleared after a skinned geometry is drawn.
        0x0e m_usDirtyFlags: u16,
        0x10 unknown_10: [u32; 9],
        0x34 m_pkBuffData: u32,
    }

    /// `NiSkinInstance` (Xbox PDB), as far as read, with `BSDismemberSkinInstance`'s partition flags.
    pub struct NiSkinInstance / NiSkinInstanceImage: 0x40 {
        0x00 vtable: u32,
        0x04 unknown_04: [u32; 2],
        0x0c m_spSkinPartition: Ptr<NiSkinPartition>,
        0x10 unknown_10: [u32; 10],
        /// Four bytes a partition: byte 0 drawn, byte 1 starts a run.
        0x38 pPartitionFlags: u32,
        0x3c bIsRenderable: u8,
        0x3d padding_3d: [u8; 3],
    }

    /// `NiSkinPartition` (Xbox PDB).
    pub struct NiSkinPartition / NiSkinPartitionImage: 0x10 {
        0x0 vtable: u32,
        0x4 m_uiRefCount: u32,
        0x8 m_uiPartitions: u32,
        0xc m_pkPartitions: u32,
    }

    /// `NiSkinPartition::Partition` (Xbox PDB), 0x2c bytes, as far as read.
    pub struct Partition / PartitionImage: 0x2c {
        0x00 unknown_00: [u32; 10],
        0x28 m_pkBuffData: u32,
    }

    /// `NiDX9Renderer` (Xbox PDB name of the PC class), as far as read.
    pub struct NiDX9Renderer / NiDX9RendererImage: 0xa40 {
        0x000 vtable: u32,
        0x004 unknown_004: [u32; 2],
        /// The property state and effects of the geometry being drawn.
        0x00c m_pkCurrProp: u32,
        0x010 m_pkCurrEffects: u32,
        0x014 unknown_014: [u32; 157],
        0x288 m_pkD3DDevice9: u32,
        0x28c unknown_28c: [u32; 234],
        /// Two directions the model transform turns into model space (+0x64c, +0x658).
        0x634 kDirectionA: [f32; 3],
        0x640 kDirectionB: [f32; 3],
        0x64c kModelDirectionA: [f32; 3],
        0x658 kModelDirectionB: [f32; 3],
        0x664 unknown_664: [u32; 146],
        0x8ac pGeometryGroupManager: u32,
        0x8b0 pGeometryGroup: u32,
        0x8b4 unknown_8b4: u32,
        0x8b8 m_pkRenderState: Ptr<NiDX9RenderState>,
        0x8bc unknown_8bc: [u32; 33],
        /// The world matrix (`00b71a40`'s form of the transform).
        0x940 m_kWorld: [f32; 16],
        0x980 unknown_980: [u32; 32],
        0xa00 m_kView: [f32; 16],
    }

    /// `NiDX9RenderState` (Gamebryo's name for the renderer's state cache), as far as read: what was
    /// last given to the device, so a repeat isn't sent again.
    pub struct NiDX9RenderState / NiDX9RenderStateImage: 0x1114 {
        0x0000 vtable: u32,
        0x0004 unknown_0004: [u32; 29],
        /// `00e88730`: two floats and their difference.
        0x0078 fRangeFirst: f32,
        0x007c fRangeSecond: f32,
        0x0080 fRangeDifference: f32,
        0x0084 unknown_0084: [u32; 20],
        /// Cull modes, two a kind (`00e879b0` picks by +0xf4).
        0x00d4 auiCullModes: [u32; 8],
        0x00f4 uiCullSide: u32,
        0x00f8 unknown_00f8: [u32; 10],
        /// Render states: the value at 8 bytes a state, and whether each is locked.
        0x0120 auiRenderStates: [u32; 0x200],
        0x0920 abLocked: [u8; 0x100],
        /// Texture stage states, 8 a stage, 8 bytes each.
        0x0a20 auiStageStates: [u32; 0x100],
        /// Sampler states, 5 a stage, 8 bytes each.
        0x0e20 auiSamplerStates: [u32; 0xa0],
        0x10a0 apTextures: [u32; 16],
        0x10e0 pVertexShader: u32,
        0x10e4 pSavedVertexShader: u32,
        0x10e8 pPixelShader: u32,
        0x10ec pSavedPixelShader: u32,
        0x10f0 unknown_10f0: u32,
        0x10f4 ucForceAlphaTestA: u8,
        0x10f5 ucForceAlphaTestB: u8,
        0x10f6 padding_10f6: [u8; 2],
        0x10f8 m_pkD3DDevice9: u32,
        0x10fc unknown_10fc: u32,
        0x1100 ucDeclarationSet: u8,
        0x1101 padding_1101: [u8; 3],
        0x1104 unknown_1104: [u32; 2],
        0x110c pDeclaration: u32,
        0x1110 pSavedDeclaration: u32,
    }

    /// The per-stage state cache a texture stage keeps at +0x10 (unnamed): two banks of eight values
    /// with set bytes, and three of five with set bytes and counts.
    pub struct StageStateCache / StageStateCacheImage: 0xb8 {
        0x00 unknown_00: [u32; 3],
        0x0c auiValuesA: [u32; 8],
        0x2c abSetA: [u8; 8],
        0x34 unknown_34: [u32; 2],
        0x3c auiValuesB: [u32; 8],
        0x5c abSetB: [u8; 8],
        0x64 uiCountC: u32,
        0x68 unknown_68: u32,
        0x6c auiValuesC: [u32; 5],
        0x80 abSetC: [u8; 5],
        0x85 padding_85: [u8; 3],
        0x88 uiCountD: u32,
        0x8c unknown_8c: u32,
        0x90 auiValuesD: [u32; 5],
        0xa4 abSetD: [u8; 5],
        0xa9 padding_a9: [u8; 3],
        0xac uiCountE: u32,
        0xb0 abSetE: [u8; 5],
        0xb5 padding_b5: [u8; 3],
    }

    /// `NiD3DTextureStage` (Xbox PDB name), as far as read.
    pub struct NiD3DTextureStage / NiD3DTextureStageImage: 0x14 {
        0x00 m_eFilterMode: u32,
        0x04 m_uiStage: u32,
        0x08 m_pkTexture: u32,
        0x0c unknown_0c: u32,
        0x10 pStateCache: Ptr<StageStateCache>,
    }

    /// `NiTArray` (Xbox PDB) of `NiPointer`s.
    pub struct NiTArray / NiTArrayImage: 0x10 {
        0x0 vtable: u32,
        0x4 m_pBase: u32,
        0x8 m_usMaxSize: u16,
        0xa m_usSize: u16,
        0xc m_usESize: u16,
        0xe m_usGrowBy: u16,
    }

    /// An entry of a shader constant map, as far as read: its count and whether it is enabled.
    pub struct ConstantEntry / ConstantEntryImage: 0xc {
        0x0 vtable: u32,
        0x4 m_uiRefCount: u32,
        0x8 ucEnabled: u8,
        0x9 padding_09: [u8; 3],
    }

    /// `NiD3DShaderConstantMap` (Xbox PDB), as far as read: its two entry arrays.
    pub struct NiD3DShaderConstantMap / NiD3DShaderConstantMapImage: 0x2c {
        0x00 unknown_00: [u32; 3],
        0x0c m_aspEntries: Inline<NiTArray> = NiTArrayImage,
        0x1c m_aspPlatformEntries: Inline<NiTArray> = NiTArrayImage,
    }

    /// `NiGeometryBufferData` (Xbox PDB), as far as read: its stream count and streams.
    pub struct NiGeometryBufferData / NiGeometryBufferDataImage: 0x2c {
        0x00 unknown_00: [u32; 8],
        0x20 m_uiStreamCount: u32,
        0x24 unknown_24: u32,
        0x28 m_ppkVBInfo: u32,
    }

    /// A light as `00c03010` reads it: its colour (inline, or at +0x34) and its fade.
    pub struct ShaderLight / ShaderLightImage: 0x44 {
        0x00 unknown_00: [u32; 10],
        0x28 kColor: [f32; 3],
        0x34 pColor: u32,
        0x38 unknown_38: [u32; 2],
        0x40 fFade: f32,
    }

    /// `ShadowSceneNode` (Xbox PDB), as far as read: its light list.
    pub struct ShadowSceneNode / ShadowSceneNodeImage: 0xc0 {
        0x00 unknown_00: [u32; 45],
        0xb4 kLights_head: u32,
        0xb8 kLights_tail: u32,
        0xbc kLights_count: u32,
    }
}
