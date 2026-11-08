//! The classes of the dispatch family that the culling family's layouts don't have, at their PC
//! offsets, as far as these functions read them. The transforms, matrices, points, objects and nodes
//! are [`crate::culling::layout`]'s.

use crate::image;

image! {
    /// The game's recursive spin lock (`0040fbf0` takes it, `0040fba0` gives it back): the owning
    /// thread's id and how many times it holds it.
    pub struct LightLock / LightLockImage: 0x8 {
        0x0 owner: u32,
        0x4 count: u32,
    }

    /// `NiRefObject` (Xbox PDB): the vtable and the reference count.
    pub struct RefObject / RefObjectImage: 0x8 {
        0x0 vtable: u32,
        0x4 m_uiRefCount: u32,
    }

    /// A node of the game's singly linked list (`BSSimpleList`): the item, then the next node.
    pub struct ListNode / ListNodeImage: 0x8 {
        0x0 item: u32,
        0x4 next: u32,
    }
}
