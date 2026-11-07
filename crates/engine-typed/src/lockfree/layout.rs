//! The classes of the lock-free map family, once each, at their PC offsets. Class and field names
//! marked (Xbox PDB) are the prototype's; the others say what the PC code does with them.
//!
//! One body serves every key and value type of a map in the exe (identical code folding and
//! the template's own sharing), so keys and values are plain words here: a key is whatever the map's
//! virtual functions take, a value whatever its holder assignment takes.

use crate::image;
use engine_core::ptr::{Inline, Ptr, Scalar};
use engine_core::Mem;

/// A pointer word whose bit 0 marks the node it sits in as deleted (`MarkedPointer`, Xbox PDB): a
/// bucket head, or a node's link to the next node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Marked(pub u32);

impl Marked {
    /// The node it points at, the mark cleared.
    #[inline(always)]
    pub fn node(self) -> Ptr<HashListNode> {
        Ptr::new(self.0 & !1)
    }

    /// Whether the mark is set.
    #[inline(always)]
    pub fn is_marked(self) -> bool {
        self.0 & 1 != 0
    }

    /// The word, as the game compares it.
    #[inline(always)]
    pub fn word(self) -> u32 {
        self.0
    }
}

impl Scalar for Marked {
    const SIZE: u32 = 4;
    #[inline(always)]
    fn load(m: &Mem, a: u32) -> Self {
        Marked(m.u32(a))
    }
    #[inline(always)]
    fn store(self, m: &Mem, a: u32) {
        m.set_u32(a, self.0)
    }
}

image! {
    /// `LockFreeMap<K, V>` (Xbox PDB), 0x40 bytes on PC (`operator new(0x40)` before its constructor).
    /// The Xbox class ends at 0x1c; the PC build adds the lock at 0x20 that the iteration takes.
    pub struct LockFreeMap / LockFreeMapImage: 0x40 {
        0x00 vtable: u32,
        /// `pReferencedNodes` (Xbox PDB): three node slots a thread, the nodes each thread is reading.
        0x04 pReferencedNodes: Ptr<NodeSlot>,
        /// `iNumBuckets` (Xbox PDB).
        0x08 iNumBuckets: u32,
        /// `pBuckets` (Xbox PDB): `iNumBuckets` bucket heads.
        0x0c pBuckets: Ptr<BucketHead>,
        /// `iDeleteBatchSize` (Xbox PDB).
        0x10 iDeleteBatchSize: u32,
        /// `pInterfaceManager` (Xbox PDB).
        0x14 pInterfaceManager: Ptr<InterfaceManager>,
        /// `iCount` (Xbox PDB).
        0x18 iCount: u32,
        0x1c unknown_1c: u32,
        /// The lock the iteration takes (`0040fbf0` .. `0040fba0`).
        0x20 lock: Inline<MapLock> = [u32; 2],
        0x28 unknown_28: [u32; 6],
    }

    /// The map's lock: the owning thread and how many times it holds it.
    pub struct MapLock / MapLockImage: 8 {
        0x0 owner: u32,
        0x4 count: u32,
    }

    /// One bucket's head: a marked pointer to its first node.
    pub struct BucketHead / BucketHeadImage: 4 {
        0x0 head: Marked,
    }

    /// One of the map's referenced-node slots.
    pub struct NodeSlot / NodeSlotImage: 4 {
        0x0 node: Ptr<HashListNode>,
    }

    /// `HashListNode` (Xbox PDB, a class of each map), 0xc bytes (`operator new(0xc)`).
    pub struct HashListNode / HashListNodeImage: 0xc {
        0x0 key: u32,
        /// The value's holder while the node is in the map; once deleted, the next node waiting
        /// to be freed.
        0x4 value: u32,
        /// The link to the next node; its mark says this node is deleted.
        0x8 next: Marked,
    }

    /// `LockFreeMapIterator` (Xbox PDB), 0x10 bytes.
    pub struct LockFreeMapIterator / LockFreeMapIteratorImage: 0x10 {
        0x0 vtable: u32,
        /// `iBucket` (Xbox PDB).
        0x4 iBucket: u32,
        /// `LastKey` (Xbox PDB).
        0x8 LastKey: u32,
        /// `cFlags` (Xbox PDB): bit 0 the iteration has a last key, bit 1 it is at its end.
        0xc cFlags: u8,
        0xd padding_d: [u8; 3],
    }

    /// `ThreadSpecificInterfaceManager<..>` (Xbox PDB), 0x10 bytes (`operator new(0x10)`): one
    /// interface a thread, found through a TLS slot.
    pub struct InterfaceManager / InterfaceManagerImage: 0x10 {
        /// How many threads may have an interface.
        0x0 max_threads: u32,
        /// The TLS slot holding the current thread's interface.
        0x4 tls_index: u32,
        /// `max_threads` entries.
        0x8 entries: Ptr<InterfaceEntry>,
        /// How many interfaces were handed out (counted up before the limit is checked).
        0xc handed_out: u32,
    }

    /// One thread's entry in an [`InterfaceManager`].
    pub struct InterfaceEntry / InterfaceEntryImage: 8 {
        0x0 thread_id: u32,
        0x4 interface: Ptr<MapInterface>,
    }

    /// `LockFreeMapInterface` (Xbox PDB), 0x24 bytes (`operator new(0x24)`): a thread's view of a
    /// map, where it keeps its place in a bucket and the nodes it is reading.
    pub struct MapInterface / MapInterfaceImage: 0x24 {
        0x00 map: Ptr<LockFreeMap>,
        /// This thread's three referenced-node slots in the map's `pReferencedNodes`.
        0x04 next_slot: Ptr<NodeSlot>,
        0x08 current_slot: Ptr<NodeSlot>,
        0x0c previous_slot: Ptr<NodeSlot>,
        /// The link that points at the current node (a bucket head, or the previous node's link).
        0x10 previous: Ptr<BucketHead>,
        0x14 current: Marked,
        0x18 next: Marked,
        /// The nodes this thread unlinked and hasn't freed yet, through their `value` words.
        0x1c deleted: Ptr<HashListNode>,
        0x20 deleted_count: u32,
    }

    /// `LockFreeQueue<T>` (Xbox PDB), as far as this family reads it; its PC size isn't confirmed here.
    pub struct LockFreeQueue / LockFreeQueueImage: 0x1c {
        0x00 vtable: u32,
        /// `pHead`, `pTail` (Xbox PDB).
        0x04 pHead: u32,
        0x08 pTail: u32,
        /// `iDeleteBatchSize` (Xbox PDB).
        0x0c iDeleteBatchSize: u32,
        /// `pReferencedNodes` (Xbox PDB): two node slots a thread.
        0x10 pReferencedNodes: Ptr<NodeSlot>,
        /// `pInterfaceManager` (Xbox PDB).
        0x14 pInterfaceManager: u32,
        /// `iCount` (Xbox PDB).
        0x18 iCount: u32,
    }

    /// A counted pointer's slot (`NiPointer<T>`, Xbox PDB): the object, whose count is at +8.
    pub struct Holder / HolderImage: 4 {
        0x0 object: Ptr<RefObject>,
    }

    /// The counted object a [`Holder`] holds (`NiRefObject`, Xbox PDB), as far as the holder reads it.
    pub struct RefObject / RefObjectImage: 0xc {
        0x0 vtable: u32,
        0x4 unread_4: u32,
        /// `m_uiRefCount` (Xbox PDB).
        0x8 m_uiRefCount: u32,
    }
}

impl LockFreeMap {
    /// The bucket head of bucket `i` (what the folded `0044ee20` answers).
    #[inline(always)]
    pub fn bucket(buckets: Ptr<BucketHead>, i: u32) -> Ptr<BucketHead> {
        buckets.index(i, BucketHead::SIZE_WORDS * 4)
    }
}

impl BucketHead {
    const SIZE_WORDS: u32 = 1;
}

impl HashListNode {
    /// The node's `next` link seen as a bucket head: both are a marked pointer, and the search
    /// keeps either as the link to the current node.
    #[inline(always)]
    pub fn next_link(node: Ptr<HashListNode>) -> Ptr<BucketHead> {
        node.byte_add(HashListNode::next.off).cast()
    }

    /// The node's `value` holder.
    #[inline(always)]
    pub fn value_holder(node: Ptr<HashListNode>) -> Ptr<Holder> {
        node.byte_add(HashListNode::value.off).cast()
    }
}

impl InterfaceManager {
    /// Entry `i`.
    #[inline(always)]
    pub fn entry(entries: Ptr<InterfaceEntry>, i: u32) -> Ptr<InterfaceEntry> {
        entries.index(i, 8)
    }
}
