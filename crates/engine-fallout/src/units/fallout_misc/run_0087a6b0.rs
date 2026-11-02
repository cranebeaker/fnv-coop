//! `fallout/misc/run_0087a6b0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Small methods of `TaskQueueInterface` (the object `004537b0` returns, the
//! global at `011df1a8`; its layout is in `taskqueueinterface.rs`) and the
//! loop that drains a `BSPackedTaskQueue`. The Xbox PDB names no method of
//! these, so they keep `fn_<address>` names and are described by what they
//! do.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_misc::taskqueueinterface::TaskQueueInterface;

/// `0086c600` (the map's `BSPackedTaskQueue::ThreadBeginInput`) on a queue.
const QUEUE_BEGIN_INPUT: u32 = 0x0086_c600;
/// `0086cdd0` (unnamed in the map): the call `0086c600` pairs with, on a
/// queue.
const QUEUE_END_INPUT: u32 = 0x0086_cdd0;
/// `0040fc90`: the current thread id.
const CURRENT_THREAD_ID: u32 = 0x0040_fc90;
/// `00543c30`: the byte at `+0x24` of its `this` (here a queue's
/// `bProcessing`).
const QUEUE_IS_PROCESSING: u32 = 0x0054_3c30;
/// `00559450`: the first word of its `this` (here the semaphore's count).
const SEMAPHORE_COUNT: u32 = 0x0055_9450;
/// `0069b080`: whether the word at `+0xc` of its `this` is zero.
const WORD_AT_0XC_IS_ZERO: u32 = 0x0069_b080;
/// The vtable slot of `BSTMessageQueue::TryPop` (`+0x10`): pops one task
/// into the buffer passed on the stack, returns whether there was one.
const TRY_POP_SLOT: u32 = 0x10;
/// Size of the task record `BSPackedTask` the loop pops (the local buffer).
const PACKED_TASK_SIZE: u32 = 0x20;

layout! {
    /// `BSPackedTaskQueue` (Xbox PDB, 0x28 bytes); the PC code reads the
    /// fields below at the same offsets.
    pub struct BSPackedTaskQueue: 0x28 {
        /// `Queue.pHead` (Xbox PDB `BSTCommonScrapHeapMessageQueue::pHead`).
        0x0C pHead: Ptr,
        /// `pUnpackFunc` (Xbox PDB): called with each popped task.
        0x20 pUnpackFunc: u32,
        /// `bProcessing` (Xbox PDB).
        0x24 bProcessing: bool,
    }
}

/// Address of the `InputSources` semaphore (`+0x14`) of a queue.
fn input_sources(queue: Ptr<BSPackedTaskQueue>) -> u32 {
    queue.addr().wrapping_add(0x14)
}

// Translated from 0087a6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0086c600` (the map's `BSPackedTaskQueue::ThreadBeginInput`) on the
/// queue in `pQueue` (`+0`).
pub fn fn_0087a6b0(e: &mut Engine, this: Ptr<TaskQueueInterface>) {
    let queue = e.get(this, TaskQueueInterface::pQueue);
    e.call(QUEUE_BEGIN_INPUT, &args![queue]);
}

// Translated from 0087a6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0086cdd0` (the call `0086c600` pairs with) on the queue in
/// `pQueue` (`+0`).
pub fn fn_0087a6d0(e: &mut Engine, this: Ptr<TaskQueueInterface>) {
    let queue = e.get(this, TaskQueueInterface::pQueue);
    e.call(QUEUE_END_INPUT, &args![queue]);
}

// Translated from 0087a6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0086c600` (the map's `BSPackedTaskQueue::ThreadBeginInput`) on the
/// queue in `pSecondaryQueue` (`+4`).
pub fn fn_0087a6f0(e: &mut Engine, this: Ptr<TaskQueueInterface>) {
    let queue = e.get(this, TaskQueueInterface::pSecondaryQueue);
    e.call(QUEUE_BEGIN_INPUT, &args![queue]);
}

// Translated from 0087a710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0086cdd0` (the call `0086c600` pairs with) on the queue in
/// `pSecondaryQueue` (`+4`).
pub fn fn_0087a710(e: &mut Engine, this: Ptr<TaskQueueInterface>) {
    let queue = e.get(this, TaskQueueInterface::pSecondaryQueue);
    e.call(QUEUE_END_INPUT, &args![queue]);
}

// Translated from 0087a770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the queue holds no element: `0069b080` on it (its word at `+0xc`,
/// `pHead`, is zero). The map names `0069b080` `IsEnd`; its body is the zero
/// test.
pub fn fn_0087a770(e: &mut Engine, queue: Ptr<BSPackedTaskQueue>) -> bool {
    e.call(WORD_AT_0XC_IS_ZERO, &args![queue]).bool()
}

// Translated from 0087a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the interface's first queue (`pQueue`) has pending elements and
/// is not being drained: `true` only when `0087a770` says the queue is not
/// empty and `00543c30` (its `bProcessing` byte) is zero.
pub fn fn_0087a730(e: &mut Engine, this: Ptr<TaskQueueInterface>) -> bool {
    let queue = e
        .get(this, TaskQueueInterface::pQueue)
        .cast::<BSPackedTaskQueue>();
    if fn_0087a770(e, queue) {
        return false;
    }
    !e.call(QUEUE_IS_PROCESSING, &args![queue]).bool()
}

// Translated from 0087a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drains a `BSPackedTaskQueue`: sets `bProcessing`, then pops tasks (the
/// virtual `TryPop`, slot `+0x10`, into a 0x20-byte buffer) and passes each
/// to the `pUnpackFunc` callback (cdecl, the buffer as its one argument)
/// until a pop finds nothing and the semaphore count (`00559450` on
/// `InputSources`, `+0x14`) is not above zero; clears `bProcessing`.
/// Returns the last pop's result (`false`).
// Translation incomplete: 0087a7d0 - the /GS stack security cookie check (00ec408c) is not modeled.
pub fn fn_0087a7d0(e: &mut Engine, this: Ptr<BSPackedTaskQueue>) -> bool {
    e.set(this, BSPackedTaskQueue::bProcessing, true);
    let mut popped = true;
    e.with_stack(PACKED_TASK_SIZE, |e, task| {
        while popped || e.call(SEMAPHORE_COUNT, &args![input_sources(this)]).i32() > 0 {
            popped = e.vcall(this.addr(), TRY_POP_SLOT, &args![task]).bool();
            if popped {
                let unpack = e.get(this, BSPackedTaskQueue::pUnpackFunc);
                e.call(unpack, &args![task]);
            }
        }
    });
    e.set(this, BSPackedTaskQueue::bProcessing, false);
    popped
}

// Translated from 0087a790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the secondary queue the active one (`pActiveQueue`), records the
/// current thread as the processing thread, drains the first queue
/// (`pQueue`) and clears the processing thread.
pub fn fn_0087a790(e: &mut Engine, this: Ptr<TaskQueueInterface>) {
    let secondary = e.get(this, TaskQueueInterface::pSecondaryQueue);
    e.set(this, TaskQueueInterface::pActiveQueue, secondary);
    let thread = e.call(CURRENT_THREAD_ID, &[]).u32();
    e.set(this, TaskQueueInterface::iProcessingThread, thread);
    let queue = e
        .get(this, TaskQueueInterface::pQueue)
        .cast::<BSPackedTaskQueue>();
    fn_0087a7d0(e, queue);
    e.set(this, TaskQueueInterface::iProcessingThread, 0);
}

// Translated from 0087a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records the current thread as the processing thread, drains the
/// secondary queue (`pSecondaryQueue`), clears the processing thread and
/// makes the first queue the active one.
pub fn fn_0087a850(e: &mut Engine, this: Ptr<TaskQueueInterface>) {
    let thread = e.call(CURRENT_THREAD_ID, &[]).u32();
    e.set(this, TaskQueueInterface::iProcessingThread, thread);
    let queue = e
        .get(this, TaskQueueInterface::pSecondaryQueue)
        .cast::<BSPackedTaskQueue>();
    fn_0087a7d0(e, queue);
    e.set(this, TaskQueueInterface::iProcessingThread, 0);
    let first = e.get(this, TaskQueueInterface::pQueue);
    e.set(this, TaskQueueInterface::pActiveQueue, first);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0087a6b0, fn_0087a6b0(Ptr<TaskQueueInterface>)),
        entry!(0x0087a6d0, fn_0087a6d0(Ptr<TaskQueueInterface>)),
        entry!(0x0087a6f0, fn_0087a6f0(Ptr<TaskQueueInterface>)),
        entry!(0x0087a710, fn_0087a710(Ptr<TaskQueueInterface>)),
        entry!(0x0087a730, fn_0087a730(Ptr<TaskQueueInterface>) -> bool),
        entry!(0x0087a770, fn_0087a770(Ptr<BSPackedTaskQueue>) -> bool),
        entry!(0x0087a790, fn_0087a790(Ptr<TaskQueueInterface>)),
        entry!(0x0087a7d0, fn_0087a7d0(Ptr<BSPackedTaskQueue>) -> bool),
        entry!(0x0087a850, fn_0087a850(Ptr<TaskQueueInterface>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A `TaskQueueInterface` over two fake queue addresses.
    fn interface(e: &mut Engine, first: u32, second: u32) -> Ptr<TaskQueueInterface> {
        let this = e.new_object::<TaskQueueInterface>();
        e.set(this, TaskQueueInterface::pQueue, Ptr::new(first));
        e.set(this, TaskQueueInterface::pSecondaryQueue, Ptr::new(second));
        this
    }

    #[test]
    fn begin_and_end_input_forward_to_the_chosen_queue() {
        let mut e = Engine::new();
        let this = interface(&mut e, 0x1000, 0x2000);
        e.register(0x0086_c600, |_, _| Ret::default());
        e.register(0x0086_cdd0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0087_a6b0, &[this.addr()]);
        e.call(0x0087_a6d0, &[this.addr()]);
        e.call(0x0087_a6f0, &[this.addr()]);
        e.call(0x0087_a710, &[this.addr()]);
        let a = this.addr();
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0087_a6b0, vec![a]),
                (0x0086_c600, vec![0x1000]),
                (0x0087_a6d0, vec![a]),
                (0x0086_cdd0, vec![0x1000]),
                (0x0087_a6f0, vec![a]),
                (0x0086_c600, vec![0x2000]),
                (0x0087_a710, vec![a]),
                (0x0086_cdd0, vec![0x2000]),
            ]
        );
    }

    #[test]
    fn empty_test_is_the_zero_test_of_the_word_at_0xc() {
        let mut e = Engine::new();
        let queue = e.new_object::<BSPackedTaskQueue>();
        e.register(0x0069_b080, |e, a| (e.mem.u32(a[0] + 0xc) == 0).into_ret());
        assert_eq!(e.call(0x0087_a770, &[queue.addr()]).u32(), 1);
        e.set(queue, BSPackedTaskQueue::pHead, Ptr::new(0x1234));
        assert_eq!(e.call(0x0087_a770, &[queue.addr()]).u32(), 0);
    }

    #[test]
    fn pending_and_idle_queue_is_the_only_true_case() {
        let mut e = Engine::new();
        let queue = e.new_object::<BSPackedTaskQueue>();
        let this = interface(&mut e, queue.addr(), 0);
        e.register(0x0069_b080, |e, a| (e.mem.u32(a[0] + 0xc) == 0).into_ret());
        e.register(0x0054_3c30, |e, a| (e.mem.u8(a[0] + 0x24) != 0).into_ret());
        // Empty queue: false, and the processing byte is not even asked.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_a730, &[this.addr()]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        // Pending and idle: true.
        e.set(queue, BSPackedTaskQueue::pHead, Ptr::new(0x1234));
        assert_eq!(e.call(0x0087_a730, &[this.addr()]).u32(), 1);
        // Pending but being processed: false.
        e.set(queue, BSPackedTaskQueue::bProcessing, true);
        assert_eq!(e.call(0x0087_a730, &[this.addr()]).u32(), 0);
    }

    /// Makes `queue` pop `tasks` (as one word each, written to the buffer)
    /// and records what the unpack callback sees.
    fn drain_setup(
        e: &mut Engine,
        tasks: Vec<u32>,
        count: i32,
    ) -> (Ptr<BSPackedTaskQueue>, Rc<RefCell<Vec<u32>>>) {
        let queue = e.new_object::<BSPackedTaskQueue>();
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0x0300_0100]);
        e.mem.set_u32(queue.addr(), 0x0300_0000);
        e.set(queue, BSPackedTaskQueue::pUnpackFunc, 0x0300_0200);
        let pending = Rc::new(RefCell::new(tasks));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let source = pending.clone();
        e.register_double(0x0300_0100, move |e, a| {
            // The queue is being processed while the loop runs.
            assert_eq!(e.mem.u8(a[0] + 0x24), 1);
            let mut source = source.borrow_mut();
            if source.is_empty() {
                false.into_ret()
            } else {
                let task = source.remove(0);
                e.mem.set_u32(a[1], task);
                true.into_ret()
            }
        });
        let sink = seen.clone();
        e.register_double(0x0300_0200, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        let remaining = Rc::new(RefCell::new(count));
        e.register_double(0x0055_9450, move |_, _| {
            let mut count = remaining.borrow_mut();
            let current = *count;
            *count -= 1;
            (current as u32).into_ret()
        });
        (queue, seen)
    }

    #[test]
    fn drain_unpacks_every_popped_task_and_waits_while_the_semaphore_is_positive() {
        let mut e = Engine::new();
        let (queue, seen) = drain_setup(&mut e, vec![11, 22], 2);
        e.call_log = Some(vec![]);
        // Two tasks pop; then empty pops while the count is 2, then 1, then 0.
        assert_eq!(e.call(0x0087_a7d0, &[queue.addr()]).u32(), 0);
        assert_eq!(*seen.borrow(), vec![11, 22]);
        let counts = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x0055_9450)
            .count();
        assert_eq!(counts, 3);
        assert_eq!(e.mem.u8(queue.addr() + 0x24), 0);
    }

    #[test]
    fn drain_of_an_empty_queue_with_no_count_pops_once() {
        let mut e = Engine::new();
        let (queue, seen) = drain_setup(&mut e, vec![], 0);
        e.call_log = Some(vec![]);
        e.call(0x0087_a7d0, &[queue.addr()]);
        assert!(seen.borrow().is_empty());
        let log = e.call_log.take().unwrap();
        assert_eq!((log[1].0, log[1].1[0]), (0x0300_0100, queue.addr()));
        assert_eq!(log[2], (0x0055_9450, vec![queue.addr() + 0x14]));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn the_two_drains_set_the_thread_and_the_active_queue() {
        let mut e = Engine::new();
        let (first, seen) = drain_setup(&mut e, vec![5], 0);
        let second = e.new_object::<BSPackedTaskQueue>();
        e.mem.set_u32(second.addr(), 0x0300_0000);
        e.set(second, BSPackedTaskQueue::pUnpackFunc, 0x0300_0200);
        let this = interface(&mut e, first.addr(), second.addr());
        e.register(0x0040_fc90, |_, _| 0x77u32.into_ret());
        e.call(0x0087_a790, &[this.addr()]);
        assert_eq!(*seen.borrow(), vec![5]);
        assert_eq!(e.get(this, TaskQueueInterface::pActiveQueue), second.cast());
        assert_eq!(e.get(this, TaskQueueInterface::iProcessingThread), 0);

        // The other order: drains the secondary queue, ends on the first.
        seen.borrow_mut().clear();
        e.call(0x0087_a850, &[this.addr()]);
        assert!(seen.borrow().is_empty());
        assert_eq!(e.get(this, TaskQueueInterface::pActiveQueue), first.cast());
        assert_eq!(e.get(this, TaskQueueInterface::iProcessingThread), 0);
    }
}
