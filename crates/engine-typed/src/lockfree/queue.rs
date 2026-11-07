//! The lock-free queue (`LockFreeQueue<T>`, Xbox PDB), the maps' sibling: as far as this family
//! reaches it.

use super::layout::{LockFreeQueue, NodeSlot};
use super::{OPERATOR_NEW, SCOPE_BEGIN, SCOPE_CONTEXT, SCOPE_END, SCOPE_MODE};
use engine_core::ptr::Ptr;
use engine_core::{args, Engine};

/// The name of the thread-safe structures' header, for the scope, and the line.
const HEADER_FILE: u32 = 0x0101_71a0;
const ALLOCATE_LINE: u32 = 0x95;
/// The queue interface's size and constructor (`thiscall(interface, queue, first slot, second
/// slot)`).
const QUEUE_INTERFACE_SIZE: u32 = 0x14;
const QUEUE_INTERFACE_CONSTRUCT: u32 = 0x0044_cd00;
/// Referenced-node slots a thread.
const SLOTS_PER_THREAD: u32 = 2;

// Typed from 0044b660
/// `LockFreeQueue<T>::AllocateInterface(index)` (Xbox PDB; the queues' virtual slot +4), in a
/// memory-context scope: a queue interface (`operator new(0x14)`, `0044cd00`) on this queue and the
/// two referenced-node slots of thread `index`; null when the allocation failed.
pub fn allocate_interface(e: &mut Engine, this: Ptr<LockFreeQueue>, index: u32) -> u32 {
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_BEGIN,
            &args![scope, SCOPE_CONTEXT, SCOPE_MODE, HEADER_FILE, ALLOCATE_LINE],
        );
        let block = e.call(OPERATOR_NEW, &args![QUEUE_INTERFACE_SIZE]).u32();
        let made = if block == 0 {
            0
        } else {
            let first = index.wrapping_mul(SLOTS_PER_THREAD);
            let slots = e.get(this, LockFreeQueue::pReferencedNodes);
            let second = slots.index(first, 4).byte_add(4).cast::<NodeSlot>();
            let slots = e.get(this, LockFreeQueue::pReferencedNodes);
            let first = slots.index(first, 4);
            e.call(
                QUEUE_INTERFACE_CONSTRUCT,
                &args![block, this, first, second],
            )
            .u32()
        };
        e.call(SCOPE_END, &args![scope]);
        made
    })
}
