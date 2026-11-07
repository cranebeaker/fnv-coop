//! The classes of the culling family, once each, at their PC offsets. Class and field names marked
//! (Xbox PDB) are the prototype's; the PC build drops the Xbox's vector registers from the culling
//! process and packs the transforms (no padding to 16 bytes), so the offsets are the PC code's, read
//! from the functions that use them. A class read only as far as the culling code reads it says so;
//! its size is then where the last field read ends.
//!
//! Floats are `f32` fields: their bits load and store unchanged ([`engine_core::Mem`] moves them as
//! words), so a signalling NaN stays one until an x87 load in the code reads it ([`super::fld`]).

use crate::image;
use engine_core::ptr::{Field, Inline, Ptr};

image! {
    /// `NiPoint3` (Xbox PDB).
    pub struct NiPoint3 / NiPoint3Image: 0xc {
        0x0 x: f32,
        0x4 y: f32,
        0x8 z: f32,
    }

    /// `NiBound` (Xbox PDB): a sphere.
    pub struct NiBound / NiBoundImage: 0x10 {
        /// `m_kCenter` (Xbox PDB).
        0x0 m_kCenter: Inline<NiPoint3> = NiPoint3Image,
        /// `m_fRadius` (Xbox PDB).
        0xc m_fRadius: f32,
    }

    /// `NiPlane` (Xbox PDB): the points `p` with `normal . p = constant`.
    pub struct NiPlane / NiPlaneImage: 0x10 {
        /// `m_kNormal` (Xbox PDB).
        0x0 m_kNormal: Inline<NiPoint3> = NiPoint3Image,
        /// `m_fConstant` (Xbox PDB).
        0xc m_fConstant: f32,
    }

    /// `NiMatrix3` (Xbox PDB): rows of three, row-major.
    pub struct NiMatrix3 / NiMatrix3Image: 0x24 {
        /// `m_pEntry` (Xbox PDB).
        0x0 m_pEntry: [f32; 9],
    }

    /// `NiTransform` (Xbox PDB), 0x34 bytes on PC (0x40 on the Xbox).
    pub struct NiTransform / NiTransformImage: 0x34 {
        /// `m_Rotate` (Xbox PDB).
        0x00 m_Rotate: Inline<NiMatrix3> = NiMatrix3Image,
        /// `m_Translate` (Xbox PDB).
        0x24 m_Translate: Inline<NiPoint3> = NiPoint3Image,
        /// `m_fScale` (Xbox PDB).
        0x30 m_fScale: f32,
    }

    /// `NiAVObject` (Xbox PDB), 0x9c bytes on PC: what every scene object has.
    pub struct NiAVObject / NiAVObjectImage: 0x9c {
        0x00 vtable: u32,
        /// `NiRefObject::m_uiRefCount` (Xbox PDB).
        0x04 m_uiRefCount: u32,
        /// `NiObjectNET::m_kName` (Xbox PDB): the name's string, or null.
        0x08 m_kName: u32,
        0x0c m_spControllers: u32,
        0x10 unknown_10: [u32; 2],
        /// `m_pkParent` (Xbox PDB).
        0x18 m_pkParent: Ptr<NiNode>,
        /// `m_spCollisionObject` (Xbox PDB).
        0x1c m_spCollisionObject: u32,
        /// `m_kWorldBound` (Xbox PDB): a pointer to the world bound, or null.
        0x20 m_kWorldBound: Ptr<NiBound>,
        /// `m_kPropertyList` (Xbox PDB).
        0x24 m_kPropertyList: [u32; 3],
        /// `m_uFlags` (Xbox PDB): see the `FLAG_*` constants of the family.
        0x30 m_uFlags: u32,
        /// `m_kLocal` (Xbox PDB).
        0x34 m_kLocal: Inline<NiTransform> = NiTransformImage,
        /// `m_kWorld` (Xbox PDB).
        0x68 m_kWorld: Inline<NiTransform> = NiTransformImage,
    }

    /// `NiNode` (Xbox PDB), 0xac bytes on PC: an object with children.
    pub struct NiNode / NiNodeImage: 0xac {
        0x00 base: Inline<NiAVObject> = NiAVObjectImage,
        /// `m_kChildren` (Xbox PDB, an `NiTObjectArray`): its vtable.
        0x9c children_vtable: u32,
        /// `m_kChildren.m_pBase` (Xbox PDB): the child pointers, some null.
        0xa0 m_pBase: Ptr<Ptr<NiAVObject>>,
        /// `m_kChildren.m_usMaxSize` (Xbox PDB).
        0xa4 m_usMaxSize: u16,
        /// `m_kChildren.m_usSize` (Xbox PDB): the slots in use, nulls included.
        0xa6 m_usSize: u16,
        /// `m_kChildren.m_usESize` (Xbox PDB).
        0xa8 m_usESize: u16,
        /// `m_kChildren.m_usGrowBy` (Xbox PDB).
        0xaa m_usGrowBy: u16,
    }

    /// `NiGeometry` (Xbox PDB), as far as the culling code reads it.
    pub struct NiGeometry / NiGeometryImage: 0xc0 {
        0x00 base: Inline<NiAVObject> = NiAVObjectImage,
        /// `m_PropertyState` (Xbox PDB).
        0x9c m_PropertyState: [u32; 7],
        /// `m_spModelData` (Xbox PDB).
        0xb8 m_spModelData: Ptr<NiGeometryData>,
        /// `m_spSkinInstance` (Xbox PDB).
        0xbc m_spSkinInstance: u32,
    }

    /// `NiGeometryData` (Xbox PDB), as far as the culling code reads it.
    pub struct NiGeometryData / NiGeometryDataImage: 0x20 {
        0x00 head: [u32; 4],
        /// `m_kBound` (Xbox PDB): the model-space bound.
        0x10 m_kBound: Inline<NiBound> = NiBoundImage,
    }

    /// `NiFrustum` (Xbox PDB).
    pub struct NiFrustum / NiFrustumImage: 0x1c {
        0x00 m_fLeft: f32,
        0x04 m_fRight: f32,
        0x08 m_fTop: f32,
        0x0c m_fBottom: f32,
        0x10 m_fNear: f32,
        0x14 m_fFar: f32,
        /// `m_bOrtho` (Xbox PDB).
        0x18 m_bOrtho: u8,
        0x19 padding_19: [u8; 3],
    }

    /// `NiFrustumPlanes` (Xbox PDB).
    pub struct NiFrustumPlanes / NiFrustumPlanesImage: 0x64 {
        /// `m_akCullingPlanes` (Xbox PDB): near, far, left, right, top, bottom.
        0x00 m_akCullingPlanes: [NiPlaneImage; 6],
        /// `m_uiActivePlanes` (Xbox PDB): bit `i` for plane `i`.
        0x60 m_uiActivePlanes: u32,
    }

    /// `NiCullingProcess` (Xbox PDB), 0x90 bytes on PC (0x120 on the Xbox, with its vector registers).
    pub struct NiCullingProcess / NiCullingProcessImage: 0x90 {
        0x00 vtable: u32,
        /// `m_bUseVirtualAppend` (Xbox PDB).
        0x04 m_bUseVirtualAppend: [u8; 4],
        /// `m_pkVisibleSet` (Xbox PDB).
        0x08 m_pkVisibleSet: u32,
        /// `m_pkCamera` (Xbox PDB).
        0x0c m_pkCamera: u32,
        /// `m_kFrustum` (Xbox PDB).
        0x10 m_kFrustum: Inline<NiFrustum> = NiFrustumImage,
        /// `m_kPlanes` (Xbox PDB).
        0x2c m_kPlanes: Inline<NiFrustumPlanes> = NiFrustumPlanesImage,
    }

    /// `BSCullingProcess` (Xbox PDB), 0xc8 bytes on PC.
    pub struct BSCullingProcess / BSCullingProcessImage: 0xc8 {
        0x00 base: Inline<NiCullingProcess> = NiCullingProcessImage,
        /// `kCullMode` (Xbox PDB): see the `MODE_*` constants of the family.
        0x90 kCullMode: u32,
        /// `eTypeStackA` (Xbox PDB).
        0x94 eTypeStackA: [u32; 10],
        /// `iCTStackIndex` (Xbox PDB).
        0xbc iCTStackIndex: u32,
        /// `pCompoundFrustum` (Xbox PDB).
        0xc0 pCompoundFrustum: Ptr<BSCompoundFrustum>,
        /// `spAccumulator` (Xbox PDB).
        0xc4 spAccumulator: u32,
    }

    /// `BSMultiBoundNode` (Xbox PDB), 0xb4 bytes on PC: a node whose children share one bound shape.
    pub struct BSMultiBoundNode / BSMultiBoundNodeImage: 0xb4 {
        0x00 base: Inline<NiNode> = NiNodeImage,
        /// `spMultiBound` (Xbox PDB).
        0xac spMultiBound: Ptr<BSMultiBound>,
        /// `CullingMode` (Xbox PDB): the mode its children are culled in.
        0xb0 CullingMode: u32,
    }

    /// `BSMultiBound` (Xbox PDB).
    pub struct BSMultiBound / BSMultiBoundImage: 0x10 {
        0x00 vtable: u32,
        0x04 m_uiRefCount: u32,
        /// `uiBoundFrameCount` (Xbox PDB).
        0x08 uiBoundFrameCount: u32,
        /// `spShape` (Xbox PDB).
        0x0c spShape: Ptr<BSMultiBoundShape>,
    }

    /// `BSMultiBoundShape` (Xbox PDB).
    pub struct BSMultiBoundShape / BSMultiBoundShapeImage: 0xc {
        0x00 vtable: u32,
        0x04 m_uiRefCount: u32,
        /// `kCullResult` (Xbox PDB): see the `RESULT_*` constants.
        0x08 kCullResult: u32,
    }

    /// `BSMultiBoundAABB` (Xbox PDB): an axis-aligned box.
    pub struct BSMultiBoundAABB / BSMultiBoundAABBImage: 0x24 {
        0x00 vtable: u32,
        0x04 m_uiRefCount: u32,
        /// `kCullResult` (Xbox PDB).
        0x08 kCullResult: u32,
        /// `Center` (Xbox PDB).
        0x0c Center: Inline<NiPoint3> = NiPoint3Image,
        /// `HalfExtents` (Xbox PDB).
        0x18 HalfExtents: Inline<NiPoint3> = NiPoint3Image,
    }

    /// `BSCompoundFrustum` (Xbox PDB): the view frustum, and portal and occluder plane sets joined
    /// by a little program of operations.
    pub struct BSCompoundFrustum / BSCompoundFrustumImage: 0xa4 {
        /// `bPrethreaded` (Xbox PDB).
        0x00 bPrethreaded: u8,
        0x01 padding_01: [u8; 3],
        /// `kPlanes` (Xbox PDB, a `BSSimpleArray`): its vtable.
        0x04 kPlanes_vtable: u32,
        /// `kPlanes.pBuffer` (Xbox PDB): the plane sets.
        0x08 kPlanes_pBuffer: Ptr<NiFrustumPlanes>,
        /// `kPlanes.iSize` (Xbox PDB).
        0x0c kPlanes_iSize: u32,
        /// `kPlanes.iReservedSize` (Xbox PDB): what the active-plane state is sized and bounded by.
        0x10 kPlanes_iReservedSize: u32,
        /// `kFunctionOperators` (Xbox PDB, a `BSSimpleArray`): its vtable.
        0x14 kFunctionOperators_vtable: u32,
        /// `kFunctionOperators.pBuffer` (Xbox PDB): the operations.
        0x18 kFunctionOperators_pBuffer: Ptr<FunctionOp>,
        0x1c kFunctionOperators_iSize: u32,
        0x20 kFunctionOperators_iReservedSize: u32,
        /// `iFreePlane` (Xbox PDB): the last plane set in use.
        0x24 iFreePlane: i32,
        /// `iFreeOp` (Xbox PDB): the operations in use; none means it culls nothing.
        0x28 iFreeOp: i32,
        /// `iFirstOp` (Xbox PDB).
        0x2c iFirstOp: i32,
        /// `kViewFrustum` (Xbox PDB).
        0x30 kViewFrustum: Inline<NiFrustumPlanes> = NiFrustumPlanesImage,
        /// `kViewPosition` (Xbox PDB).
        0x94 kViewPosition: Inline<NiPoint3> = NiPoint3Image,
        /// `bSkipViewFrustum` (Xbox PDB).
        0xa0 bSkipViewFrustum: u8,
        0xa1 padding_a1: [u8; 3],
    }

    /// `BSCompoundFrustum::FunctionOp` (Xbox PDB): an operation, and where to go on a pass or a fail.
    pub struct FunctionOp / FunctionOpImage: 0xc {
        /// `op` (Xbox PDB): see the `OP_*` constants.
        0x0 op: u32,
        /// `jp` (Xbox PDB).
        0x4 jp: u32,
        /// `jf` (Xbox PDB).
        0x8 jf: u32,
    }
}

impl NiMatrix3 {
    /// Entry `(row, column)` of the matrix.
    #[inline(always)]
    pub const fn entry(row: u32, column: u32) -> Field<NiMatrix3, f32> {
        Field::new(4 * (3 * row + column))
    }
}
