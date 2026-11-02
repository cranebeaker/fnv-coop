//! Layouts used by every subsystem: Gamebryo's and Bethesda's containers,
//! strings and reference-counted bases (Xbox PDB, sizes and offsets checked
//! against PC code). Lead-owned: translators use these and ask for
//! additions in their report instead of declaring their own copies.
//!
//! Templates are declared once; the element type is the translator's to
//! know (a `NiTArray<Actor *>`'s `m_pBase` holds `Actor` pointers).

use crate::layout;
use crate::ptr::Inline;

layout! {
    /// `NiRefObject` (Xbox PDB): vtable at +0, reference count at +4.
    pub struct NiRefObject: 0x08 {
        /// `m_uiRefCount` (Xbox PDB).
        0x04 m_uiRefCount: u32,
    }

    /// `NiTArray<T, Interface>` (Xbox PDB), 0x10 bytes for every `T`:
    /// vtable, element pointer, then four 16-bit counts.
    pub struct NiTArray: 0x10 {
        /// `m_pBase` (Xbox PDB): the elements, 4 bytes each for pointer `T`.
        0x04 m_pBase: u32,
        /// `m_usMaxSize` (Xbox PDB): capacity.
        0x08 m_usMaxSize: u16,
        /// `m_usSize` (Xbox PDB): one past the highest used index.
        0x0A m_usSize: u16,
        /// `m_usESize` (Xbox PDB): number of non-null elements.
        0x0C m_usESize: u16,
        /// `m_usGrowBy` (Xbox PDB).
        0x0E m_usGrowBy: u16,
    }

    /// `BSSimpleArray<T, 1024>` (Xbox PDB), 0x10 bytes for every `T`.
    pub struct BSSimpleArray: 0x10 {
        /// `pBuffer` (Xbox PDB).
        0x04 pBuffer: u32,
        /// `iSize` (Xbox PDB).
        0x08 iSize: u32,
        /// `iReservedSize` (Xbox PDB).
        0x0C iReservedSize: u32,
    }

    /// `BSSimpleList<T>` (Xbox PDB): one node, item then next.
    pub struct BSSimpleList: 0x08 {
        /// `m_item` (Xbox PDB).
        0x00 m_item: u32,
        /// `m_pkNext` (Xbox PDB).
        0x04 m_pkNext: u32,
    }

    /// `BSStringT<char>` (Xbox PDB), the game's `BSString`.
    pub struct BSStringT: 0x08 {
        /// `pString` (Xbox PDB).
        0x00 pString: u32,
        /// `sLen` (Xbox PDB).
        0x04 sLen: u16,
        /// `sMaxLen` (Xbox PDB).
        0x06 sMaxLen: u16,
    }

    /// `NiFixedString` (Xbox PDB): a handle to a pooled string.
    pub struct NiFixedString: 0x04 {
        /// `m_kHandle` (Xbox PDB).
        0x00 m_kHandle: u32,
    }
}

layout! {
    /// `NiTPointerList<T>` (Xbox PDB), 0x0C bytes: head and tail of a
    /// doubly linked list of [`NiTListItem`]s, and the count kept by its
    /// allocator.
    pub struct NiTPointerList: 0x0C {
        /// `m_pkHead` (Xbox PDB).
        0x00 m_pkHead: u32,
        /// `m_pkTail` (Xbox PDB).
        0x04 m_pkTail: u32,
        /// `m_kAllocator.m_uiCount` (Xbox PDB): number of items.
        0x08 m_uiCount: u32,
    }

    /// `NiTListItem<T>`: one node of an [`NiTPointerList`]; next, previous,
    /// element (the order PC code walks them in).
    pub struct NiTListItem: 0x0C {
        /// `m_pkNext`.
        0x00 m_pkNext: u32,
        /// `m_pkPrev`.
        0x04 m_pkPrev: u32,
        /// `m_element`.
        0x08 m_element: u32,
    }

    /// `NiTPointerMap<K, V>` / `NiTMapBase` (Xbox PDB), 0x10 bytes: vtable,
    /// bucket count, bucket array, and the item count kept by its allocator.
    pub struct NiTPointerMap: 0x10 {
        /// `m_uiHashSize` (Xbox PDB): number of buckets.
        0x04 m_uiHashSize: u32,
        /// `m_ppkHashTable` (Xbox PDB).
        0x08 m_ppkHashTable: u32,
        /// `m_kAllocator.m_uiCount` (Xbox PDB): number of items.
        0x0C m_uiCount: u32,
    }

    /// `NiPlane` (Xbox PDB): normal and constant.
    pub struct NiPlane: 0x10 {
        0x00 m_kNormal_x: f32,
        0x04 m_kNormal_y: f32,
        0x08 m_kNormal_z: f32,
        /// `m_fConstant` (Xbox PDB).
        0x0C m_fConstant: f32,
    }

    /// `NiColorA` (Xbox PDB).
    pub struct NiColorA: 0x10 {
        0x00 r: f32,
        0x04 g: f32,
        0x08 b: f32,
        0x0C a: f32,
    }

    /// `TESForm` on PC, 0x18 bytes. The Xbox PDB's `TESForm` is 0x28: it
    /// also has the editor ID string and version-control fields (+0x10 to
    /// +0x1C), which the PC build does not have, so every field after
    /// `iFormID` sits 0x10 lower on PC than in the PDB, in every class
    /// derived from `TESForm`.
    pub struct TESForm: 0x18 {
        /// `cFormType` (Xbox PDB).
        0x04 cFormType: u8,
        /// `iFormFlags` (Xbox PDB).
        0x08 iFormFlags: u32,
        /// `iFormID` (Xbox PDB).
        0x0C iFormID: u32,
        /// `pSourceFiles` (Xbox PDB): a [`BSSimpleList`] of `TESFile *`
        /// (PDB +0x20).
        0x10 pSourceFiles: Inline<BSSimpleList>,
    }

    /// `BaseExtraList` / `ExtraDataList` (Xbox PDB), 0x20 bytes on both
    /// builds: vtable, the first `BSExtraData`, and 21 bytes of presence
    /// bits (one per extra-data type).
    pub struct BaseExtraList: 0x20 {
        /// `pHead` (Xbox PDB).
        0x04 pHead: u32,
        /// `iFlags[0]` (Xbox PDB): first byte of the 21-byte presence bit
        /// array; byte `t / 8`, bit `t % 8` is set when type `t` is present.
        0x08 iFlags: u8,
    }

    /// `NiAVObject` (Xbox PDB), the fields whose offsets are the same on PC
    /// (before the transforms, which the Xbox build aligns to 16 bytes). The
    /// PC size 0x9C is not yet checked against an allocation in the exe; do
    /// not allocate an `NiAVObject` by this size until it is.
    pub struct NiAVObject: 0x9C {
        /// `m_kName` (Xbox PDB): an [`NiFixedString`].
        0x08 m_kName: u32,
        /// `m_spControllers` (Xbox PDB).
        0x0C m_spControllers: u32,
        /// `m_pkParent` (Xbox PDB).
        0x18 m_pkParent: u32,
        /// `m_spCollisionObject` (Xbox PDB).
        0x1C m_spCollisionObject: u32,
        /// `m_kWorldBound` (Xbox PDB).
        0x20 m_kWorldBound: u32,
        /// `m_kPropertyList` (Xbox PDB): an [`NiTPointerList`].
        0x24 m_kPropertyList: Inline<NiTPointerList>,
        /// `m_uFlags` (Xbox PDB).
        0x30 m_uFlags: u32,
    }
}

/// `ExtraDataList` has the layout of [`BaseExtraList`].
pub type ExtraDataList = BaseExtraList;

/// `NiTMap<K, V>` has the `NiTMapBase` layout of [`NiTPointerMap`]
/// (vtable, bucket count, bucket array, item count).
pub type NiTMap = NiTPointerMap;

layout! {
    /// `NiPoint3` (Xbox PDB).
    pub struct NiPoint3: 0x0C {
        0x00 x: f32,
        0x04 y: f32,
        0x08 z: f32,
    }
}
