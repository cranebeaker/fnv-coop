//! `fallout/misc/taskqueueinterface.cpp` (Xbox PDB source unit).
//! FalloutNV.exe 1.4.0.525 task-record producers. The 32-byte records have
//! a tag followed by argument slots at four-byte intervals. Byte arguments
//! write only the low byte of their slot; unused bytes are unspecified in
//! the exe (the engine's temporary-memory allocator starts them at zero).
//! Compiler security-cookie checks and C++ exception unwinding are omitted.
//! QueueIfActive preserves the active-queue and synchronous-dispatch paths.

use crate::prelude::*;
use crate::types::BSSimpleArray;

layout! {
    /// `TaskQueueInterface` (Xbox PDB), verified by 0087a670's PC stores.
    pub struct TaskQueueInterface: 0x10 {
        0x00 pQueue: Ptr,
        0x04 pSecondaryQueue: Ptr,
        0x08 pActiveQueue: Ptr,
        0x0C iProcessingThread: u32,
    }
}

/// Build the exe's local task record, retaining only the fields the caller
/// writes. `prepare` includes its calls before the record is dispatched.
fn with_task(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    tag: u32,
    prepare: impl FnOnce(&mut Engine, Ptr),
) {
    e.with_stack(0x20, |e, task| {
        e.mem.set_u32(task.addr(), tag);
        prepare(e, task);
        task_queue_interface_queue_if_active(e, this, task);
    });
}

fn with_task_if_target(e: &mut Engine, this: Ptr<TaskQueueInterface>, object: Ptr) {
    e.with_stack(0x20, |e, task| {
        e.mem.set_u32(task.addr(), 0x12);
        let target = e.call(0x0059_bb30, &args![object]).u32();
        let actor = e.call(0x0082_2b40, &args![target]).u32();
        if actor != 0 {
            let target = e.call(0x0059_bb30, &args![object]).u32();
            let actor = e.call(0x0082_2b40, &args![target]).u32();
            if e.vcall(actor, 0x1d0, &[]).u32() == 0 {
                return;
            }
        }
        e.mem.set_u32(task.addr() + 4, object.addr());
        task_queue_interface_queue_if_active(e, this, task);
    });
}

// Translated from 0087a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the two queues, makes the first active and clears the thread.
pub fn fn_0087a670(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    first: Ptr,
    second: Ptr,
) -> Ptr<TaskQueueInterface> {
    e.set(this, TaskQueueInterface::pQueue, first);
    e.set(this, TaskQueueInterface::pSecondaryQueue, second);
    e.set(this, TaskQueueInterface::iProcessingThread, 0);
    let first = e.get(this, TaskQueueInterface::pQueue);
    e.set(this, TaskQueueInterface::pActiveQueue, first);
    this
}

// Translated from 0087a890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the argument through the object's virtual slot +4.
pub fn fn_0087a890(e: &mut Engine, this: Ptr, argument: u32) -> u8 {
    e.vcall(this.addr(), 4, &args![argument]).u8()
}

// Translation incomplete: 0087aad0 - compiler exception unwinding for the temporary reference is not implemented.
// Translated from 0087aad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the global container is enabled, constructs a temporary reference,
/// inserts it in the global collection, and destroys it on the normal path.
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_0087aad0(e: &mut Engine, _unused_this: Ptr<TaskQueueInterface>, object: Ptr) {
    let enabled = e.call(0x0040_8d60, &args![0x011d_f1b8u32]).u32();
    if e.mem.u8(enabled) != 0 {
        e.with_stack(4, |e, reference| {
            e.call(0x0063_3c90, &args![reference, object]);
            e.call(0x006c_61b0, &args![0x011d_f1c4u32, reference]);
            e.call(0x0045_cec0, &args![reference]);
        });
    }
}

// Translated from 0087ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a two-word entry when the global container is enabled. Reloads
/// the indexed element before writing its second word, as the exe does.
pub fn fn_0087ab50(e: &mut Engine, _unused_this: Ptr<TaskQueueInterface>, first: u32, second: u32) {
    let enabled = e.call(0x0040_8d60, &args![0x011d_f214u32]).u32();
    if e.mem.u8(enabled) != 0 {
        let index = e.call(0x0084_5460, &args![0x011d_f244u32]).u32();
        let entry = e.call(0x0069_50d0, &args![0x011d_f244u32, index]).u32();
        e.mem.set_u32(entry, first);
        let entry = e.call(0x0069_50d0, &args![0x011d_f244u32, index]).u32();
        e.mem.set_u32(entry.wrapping_add(4), second);
    }
}

// Translated from 0087adf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes the embedded member at +0x60 to 0040b460 (cdecl).
pub fn fn_0087adf0(e: &mut Engine, object: Ptr) {
    e.call(0x0040_b460, &args![object.addr().wrapping_add(0x60)]);
}

// Translation incomplete: 0087ae60 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues tag 0x0C only for a nonnull object with virtual slot +0x220
/// returning a nonzero low byte. Upper return bytes do not affect the gate.
pub fn fn_0087ae60(e: &mut Engine, this: Ptr<TaskQueueInterface>, object: Ptr) {
    if !object.is_null() && e.vcall(object.addr(), 0x220, &[]).u8() != 0 {
        with_task(e, this, 0x0c, |e, task| {
            e.mem.set_u32(task.addr() + 4, object.addr());
        });
    }
}

// Translation incomplete: 0087aec0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues tag 0x12 for a nonnull target with no actor, or with an actor
/// whose virtual slot +0x1D0 returns nonzero. The second actor lookup is
/// deliberately repeated rather than cached.
pub fn fn_0087aec0(e: &mut Engine, this: Ptr<TaskQueueInterface>, object: Ptr) {
    if object.is_null() {
        return;
    }
    with_task_if_target(e, this, object);
}

// Translation incomplete: 0087aa40 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueWeaponFire` (Xbox PDB): packs tag 0x20 and dispatches the task.
pub fn task_queue_interface_queue_weapon_fire(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
) {
    with_task(e, this, 0x20, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087aa90 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087aa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x04 and dispatches the task.
pub fn fn_0087aa90(e: &mut Engine, this: Ptr<TaskQueueInterface>) {
    with_task(e, this, 0x4, |_, _| {});
}

// Translation incomplete: 0087abb0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087abb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x06 and dispatches the task.
pub fn fn_0087abb0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    with_task(e, this, 0x6, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087ac00 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueNodeAttach` (Xbox PDB): packs tag 0x07 and dispatches the task.
pub fn task_queue_interface_queue_node_attach(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: Ptr,
    argument_3: u8,
    argument_4: u8,
) {
    with_task(e, this, 0x7, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u32(task.addr() + 8, argument_2.addr());
        e.mem.set_u8(task.addr() + 12, argument_3);
        e.mem.set_u8(task.addr() + 16, argument_4);
        e.call(0x0040_f6e0, &args![argument_1]);
        e.call(0x0040_f6e0, &args![argument_2]);
    });
}

// Translation incomplete: 0087ac60 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ac60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x08 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087ac60(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x8, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087acb0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x2F and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087acb0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x2f, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087ad00 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x09 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087ad00(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: u32,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x9, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u32(task.addr() + 8, argument_2);
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087ad50 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ad50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x0A and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087ad50(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0xa, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087ada0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x0B and dispatches the task.
/// A null argument 2 suppresses all calls.
pub fn fn_0087ada0(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: Ptr,
) {
    if argument_2.is_null() {
        return;
    }
    with_task(e, this, 0xb, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2.addr());
        fn_0087adf0(e, argument_2);
    });
}

// Translation incomplete: 0087ae10 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087ae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x11 and dispatches the task.
/// A null argument 2 suppresses all calls.
pub fn fn_0087ae10(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: Ptr,
) {
    if argument_2.is_null() {
        return;
    }
    with_task(e, this, 0x11, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2.addr());
        fn_0087adf0(e, argument_2);
    });
}

// Translation incomplete: 0087af50 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087af50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x0D and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087af50(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: f32,
    argument_3: u8,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0xd, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_f32(task.addr() + 8, x87_quiet(argument_2));
        e.mem.set_u8(task.addr() + 12, argument_3);
    });
}

// Translation incomplete: 0087afa0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087afa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x30 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087afa0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr, argument_2: u8) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x30, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u8(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087aff0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087aff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueActorKill` (Xbox PDB): packs tag 0x0E and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn task_queue_interface_queue_actor_kill(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: Ptr,
    argument_3: u32,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0xe, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u32(task.addr() + 8, argument_2.addr());
        e.mem.set_u32(task.addr() + 12, argument_3);
    });
}

// Translation incomplete: 0087b040 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x0F and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087b040(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: u8,
    argument_3: u8,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0xf, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u8(task.addr() + 8, argument_2);
        e.mem.set_u8(task.addr() + 12, argument_3);
    });
}

// Translation incomplete: 0087b090 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueActorKnockExplosion` (Xbox PDB): packs tag 0x10 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn task_queue_interface_queue_actor_knock_explosion(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: f32,
    argument_3: f32,
    argument_4: f32,
    argument_5: f32,
) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x10, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_f32(task.addr() + 8, x87_quiet(argument_2));
        e.mem.set_f32(task.addr() + 12, x87_quiet(argument_3));
        e.mem.set_f32(task.addr() + 16, x87_quiet(argument_4));
        e.mem.set_f32(task.addr() + 20, x87_quiet(argument_5));
    });
}

// Translation incomplete: 0087b0f0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x13 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087b0f0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    e.call(0x0040_f6e0, &args![argument_1]);
    with_task(e, this, 0x13, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
    });
}

// Translation incomplete: 0087b140 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueHavokRemove` (Xbox PDB): packs tag 0x14 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn task_queue_interface_queue_havok_remove(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
) {
    if argument_1.is_null() {
        return;
    }
    e.call(0x0040_f6e0, &args![argument_1]);
    with_task(e, this, 0x14, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
    });
}

// Translation incomplete: 0087b190 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x2C and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn fn_0087b190(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    if argument_1.is_null() {
        return;
    }
    with_task(e, this, 0x2c, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
    });
}

// Translation incomplete: 0087b1e0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueHavokMotionChange` (Xbox PDB): packs tag 0x22 and dispatches the task.
/// A null argument 1 suppresses all calls.
pub fn task_queue_interface_queue_havok_motion_change(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: u32,
) {
    if argument_1.is_null() {
        return;
    }
    e.call(0x0040_f6e0, &args![argument_1]);
    with_task(e, this, 0x22, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u32(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087b230 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x2D and dispatches the task.
pub fn fn_0087b230(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: u32) {
    with_task(e, this, 0x2d, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b270 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x29 and dispatches the task.
pub fn fn_0087b270(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: u32) {
    with_task(e, this, 0x29, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b2b0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x2A and dispatches the task.
pub fn fn_0087b2b0(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: Ptr) {
    with_task(e, this, 0x2a, |e, task| {
        let value = e.call(0x0084_e3a0, &args![argument_1]).u32();
        e.mem.set_u32(task.addr() + 4, value);
    });
}

// Translation incomplete: 0087b300 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueHavokAdjustForceAddCount` (Xbox PDB): packs tag 0x31 and dispatches the task.
pub fn task_queue_interface_queue_havok_adjust_force_add_count(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: Ptr,
    argument_2: u8,
    argument_3: u8,
    argument_4: u8,
) {
    with_task(e, this, 0x31, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1.addr());
        e.mem.set_u8(task.addr() + 8, argument_2);
        e.mem.set_u8(task.addr() + 12, argument_3);
        e.mem.set_u8(task.addr() + 16, argument_4);
        e.call(0x0040_f6e0, &args![argument_1]);
    });
}

// Translation incomplete: 0087b360 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueAttachWeapon` (Xbox PDB): packs tag 0x15 and dispatches the task.
pub fn task_queue_interface_queue_attach_weapon(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
) {
    with_task(e, this, 0x15, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087b3b0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x16 and dispatches the task.
pub fn fn_0087b3b0(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
) {
    with_task(e, this, 0x16, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
    });
}

// Translation incomplete: 0087b400 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueSet3DNull` (Xbox PDB): packs tag 0x17 and dispatches the task.
pub fn task_queue_interface_queue_set_3d_null(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
) {
    with_task(e, this, 0x17, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b440 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueDismemberLimb` (Xbox PDB): packs tag 0x18 and dispatches the task.
pub fn task_queue_interface_queue_dismember_limb(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
    argument_3: u32,
    argument_4: u8,
    argument_5: u8,
) {
    with_task(e, this, 0x18, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
        e.mem.set_u32(task.addr() + 12, argument_3);
        e.mem.set_u8(task.addr() + 16, argument_4);
        e.mem.set_u8(task.addr() + 20, argument_5);
    });
}

// Translation incomplete: 0087b4a0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x19 and dispatches the task.
pub fn fn_0087b4a0(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
    argument_3: f32,
) {
    with_task(e, this, 0x19, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
        e.mem.set_f32(task.addr() + 12, x87_quiet(argument_3));
    });
}

// Translation incomplete: 0087b4f0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x1A and dispatches the task.
pub fn fn_0087b4f0(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u32,
    argument_3: f32,
) {
    with_task(e, this, 0x1a, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u32(task.addr() + 8, argument_2);
        e.mem.set_f32(task.addr() + 12, x87_quiet(argument_3));
    });
}

// Translation incomplete: 0087b540 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x1B and dispatches the task.
pub fn fn_0087b540(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: u32) {
    with_task(e, this, 0x1b, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b580 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs tag 0x1C and dispatches the task.
pub fn fn_0087b580(e: &mut Engine, this: Ptr<TaskQueueInterface>, argument_1: u32) {
    with_task(e, this, 0x1c, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b5c0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueCharacterReset3D` (Xbox PDB): packs tag 0x1D and dispatches the task.
pub fn task_queue_interface_queue_character_reset_3d(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
) {
    with_task(e, this, 0x1d, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
    });
}

// Translation incomplete: 0087b600 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TaskQueueInterface::QueueSetOpenState` (Xbox PDB): packs tag 0x1E and dispatches the task.
pub fn task_queue_interface_queue_set_open_state(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    argument_1: u32,
    argument_2: u8,
    argument_3: u8,
) {
    with_task(e, this, 0x1e, |e, task| {
        e.mem.set_u32(task.addr() + 4, argument_1);
        e.mem.set_u8(task.addr() + 8, argument_2);
        e.mem.set_u8(task.addr() + 12, argument_3);
    });
}

// Translation incomplete: 0087b650 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs a value and three x87-rounded vector components into the task record.
pub fn fn_0087b650(e: &mut Engine, this: Ptr<TaskQueueInterface>, value: u32, vector: Ptr) {
    with_task(e, this, 0x21, |e, task| {
        e.mem.set_u32(task.addr() + 4, value);
        for offset in [0, 4, 8] {
            let value = x87_quiet(e.mem.f32(vector.addr() + offset));
            e.mem.set_f32(task.addr() + 8 + offset, value);
        }
    });
}

// Translation incomplete: 0087b700 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs a value and three x87-rounded vector components into the task record.
pub fn fn_0087b700(e: &mut Engine, this: Ptr<TaskQueueInterface>, value: f32, vector: Ptr) {
    with_task(e, this, 0x24, |e, task| {
        e.mem.set_u32(task.addr() + 4, x87_quiet(value).to_bits());
        for offset in [0, 4, 8] {
            let value = x87_quiet(e.mem.f32(vector.addr() + offset));
            e.mem.set_f32(task.addr() + 8 + offset, value);
        }
    });
}

// Translation incomplete: 0087b760 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs a value and three x87-rounded vector components into the task record.
pub fn task_queue_interface_queue_add_ripple(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    value: f32,
    vector: Ptr,
) {
    with_task(e, this, 0x25, |e, task| {
        e.mem.set_u32(task.addr() + 4, x87_quiet(value).to_bits());
        for offset in [0, 4, 8] {
            let value = x87_quiet(e.mem.f32(vector.addr() + offset));
            e.mem.set_f32(task.addr() + 8 + offset, value);
        }
    });
}

// Translation incomplete: 0087b6b0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// TaskQueueInterface task producer (Xbox PDB); byte slots retain their low byte only.
pub fn task_queue_interface_queue_actor_set_havok_weapon(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    object: u32,
    flag_0: u8,
) {
    with_task(e, this, 0x23, |e, task| {
        e.mem.set_u32(task.addr() + 4, object);
        e.mem.set_u8(task.addr() + 8, flag_0);
    });
}

// Translation incomplete: 0087b7c0 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// TaskQueueInterface task producer (Xbox PDB); byte slots retain their low byte only.
pub fn task_queue_interface_queue_actor_trigger_pain(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    object: u32,
    flag_0: u8,
    flag_1: u8,
) {
    with_task(e, this, 0x26, |e, task| {
        e.mem.set_u32(task.addr() + 4, object);
        e.mem.set_u8(task.addr() + 8, flag_0);
        e.mem.set_u8(task.addr() + 12, flag_1);
    });
}

// Translation incomplete: 0087b810 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b810 (decompiled, FalloutNV.exe 1.4.0.525)
/// TaskQueueInterface task producer (Xbox PDB); byte slots retain their low byte only.
pub fn task_queue_interface_queue_actor_apply_critical_stage(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    object: u32,
) {
    with_task(e, this, 0x27, |e, task| {
        e.mem.set_u32(task.addr() + 4, object);
    });
}

// Translation incomplete: 0087b850 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// TaskQueueInterface task producer (Xbox PDB); byte slots retain their low byte only.
pub fn task_queue_interface_queue_actor_reset_loaded_anims(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    object: u32,
) {
    with_task(e, this, 0x28, |e, task| {
        e.mem.set_u32(task.addr() + 4, object);
    });
}

// Translation incomplete: 0087b890 - compiler security-cookie validation at 00ec408c is not implemented.
// Translated from 0087b890 (decompiled, FalloutNV.exe 1.4.0.525)
/// TaskQueueInterface task producer (Xbox PDB); byte slots retain their low byte only.
pub fn task_queue_interface_queue_delete_active_effect(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    object: u32,
) {
    with_task(e, this, 0x2e, |e, task| {
        e.mem.set_u32(task.addr() + 4, object);
    });
}

// Translation incomplete: 0087b8d0 - compiler exception unwinding is not implemented.
// Translated from 0087b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates and initializes the global queue interface; allocation failure stores null.
pub fn fn_0087b8d0(e: &mut Engine, first: Ptr, second: Ptr) {
    let storage = e
        .call(0x00401000, &args![0x10u32])
        .ptr::<TaskQueueInterface>();
    let queue = if storage.is_null() {
        storage
    } else {
        fn_0087a670(e, storage, first, second)
    };
    e.set_global(0x011df1a8, queue.addr());
}

// Translated from 0087b950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the global queue interface when present, then clears it.
pub fn fn_0087b950(e: &mut Engine) {
    let queue = e.global::<u32>(0x011df1a8);
    if queue != 0 {
        e.call(0x007b3fa0, &args![queue, 1u32]);
    }
    e.set_global(0x011df1a8, 0u32);
}

// Translated from 0087ce50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the world-bound value through the original ST0 result.
pub fn fn_0087ce50(e: &mut Engine, object: Ptr) -> f64 {
    let bound = e.call(0x0043d450, &args![object]).u32();
    e.call(0x0084d030, &args![bound]).f64()
}

// Translated from 0087ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global reference collection address.
pub fn fn_0087ce70(e: &mut Engine) -> Ptr {
    let _ = e;
    Ptr::new(0x011c4104)
}

// Translated from 0087ce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the PC member at +0x40.
pub fn fn_0087ce80(e: &mut Engine, object: Ptr, value: u32) {
    e.mem.set_u32(object.addr().wrapping_add(0x40), value);
}

// Translated from 0087cea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the object only when the helper on its +0x60 member returns zero.
pub fn fn_0087cea0(e: &mut Engine, object: Ptr) {
    if e.call(0x004019a0, &args![object.addr().wrapping_add(0x60)])
        .u32()
        == 0
    {
        e.call(0x00401030, &args![object]);
    }
}

// Translated from 0087cee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests the bit-mask helper on the PC member at +0x410.
pub fn fn_0087cee0(e: &mut Engine, object: Ptr) -> bool {
    e.call(
        0x00621270,
        &args![object.addr().wrapping_add(0x410), 0x80u32],
    )
    .u32()
        != 0
}

// Translated from 0087cf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a four-word parallel-array task descriptor.
pub fn fn_0087cf10(e: &mut Engine, this: Ptr, callback: u32, context: u32, count: u32) -> Ptr {
    e.mem.set_u32(this.addr(), callback);
    e.mem.set_u32(this.addr() + 4, context);
    e.mem.set_u32(this.addr() + 8, count);
    e.mem.set_u32(this.addr() + 12, 0);
    this
}

// Translated from 0087cf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates a nonnull indexed node with the zero-initialized update context.
pub fn fn_0087cf50(e: &mut Engine, array: Ptr, index: u32) {
    let entry = e.call(0x006a7ad0, &args![array, index]).u32();
    let node = e.call(0x00559450, &args![entry]).u32();
    if node != 0 {
        e.with_stack(12, |e, update| {
            e.call(0x0043d410, &args![update, 0.0f32, 0u32, 0u32]);
            e.vcall(node, 0xc0, &args![update]);
        });
    }
}

// Translated from 0087cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// ActorHavokSyncParallelArrayTaskFunc (Xbox PDB): preserves each low-byte gate and live entry reload.
pub fn actor_havok_sync_parallel_array_task_func(e: &mut Engine, array: Ptr, index: u32) {
    let entry = e.call(0x00844160, &args![array, index]).u32();
    let actor = e.mem.u32(entry);
    if actor == 0 || e.mem.u32(actor + 0xac) == 0 {
        return;
    }
    if e.call(0x00552490, &args![e.mem.u32(actor + 0xac)]).u8() == 0 {
        return;
    }
    if e.call(0x00552470, &args![e.mem.u32(entry + 4)]).u8() != 0 {
        return;
    }
    if e.call(0x00933840, &args![e.mem.u32(entry)]).u8() != 0 {
        return;
    }
    e.call(0x00c6c3d0, &args![e.mem.u32(entry + 4), 2u32]);
}

// Translation incomplete: 0087d030 - compiler exception unwinding is not implemented.
// Translated from 0087d030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Expands large node groups into retained nonnull children, then clears the source array.
pub fn fn_0087d030(e: &mut Engine, output: Ptr) {
    let count = e.call(0x0044ddc0, &args![0x011df1c4u32]).u32();
    e.with_stack(8, |e, temporaries| {
        for index in 0..count {
            let entry = e.call(0x006a7ad0, &args![0x011df1c4u32, index]).u32();
            let node = e.call(0x00559450, &args![entry]).u32();
            let children = e.call(0x0043b480, &args![node]).u32();
            if children <= 10 {
                let temporary = temporaries.addr();
                e.call(0x00633c90, &args![temporary, node]);
                e.call(0x006c61b0, &args![output, temporary]);
                e.call(0x0045cec0, &args![temporary]);
            } else {
                for child_index in 0..children {
                    let child = e.call(0x0043b4a0, &args![node, child_index]).u32();
                    if child != 0 {
                        let temporary = temporaries.addr() + 4;
                        e.call(0x00633c90, &args![temporary, child]);
                        e.call(0x006c61b0, &args![output, temporary]);
                        e.call(0x0045cec0, &args![temporary]);
                    }
                }
            }
        }
    });
    e.call(0x006c6200, &args![0x011df1c4u32, 0u32]);
}

// Translated from 0087d160 (decompiled, FalloutNV.exe 1.4.0.525)
/// TaskQueueInterface::QueueIfActive (Xbox PDB): enqueues when active, otherwise executes synchronously.
pub fn task_queue_interface_queue_if_active(
    e: &mut Engine,
    this: Ptr<TaskQueueInterface>,
    task: Ptr,
) {
    if e.global::<u8>(0x011a31f4) == 0 {
        task_queue_interface_task_unpack_func(e, task);
    } else {
        let queue = e.call(0x0044ddc0, &args![this]).ptr();
        if fn_0087a890(e, queue, task.addr()) == 0 {
            e.call(0x00fdf0c8, &[]);
        }
    }
}

// Translation incomplete: 0087d1b0 - compiler exception unwinding is not implemented.
// Translated from 0087d1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// BSScrapArray<NiPointer<NiAVObject>,1024> constructor (Xbox PDB).
pub fn scrap_array_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x00822580, &args![this]);
    e.mem.set_u32(this.addr(), 0x01084074);
    let manager = e.call(0x00401020, &[]).u32();
    let heap = e.call(0x00aa42e0, &args![manager]).u32();
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(0x00822860, &args![this, 0u32, 0u32]);
    this
}

// Translation incomplete: 0087d230 - compiler exception unwinding is not implemented.
// Translated from 0087d230 (decompiled, FalloutNV.exe 1.4.0.525)
/// BSScrapArray<NiPointer<NiAVObject>,1024> destructor (Xbox PDB).
pub fn scrap_array_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x01084074);
    e.call(0x006c6200, &args![this, 1u32]);
    e.call(0x008225b0, &args![this]);
}

// Translated from 0087d290 (decompiled, FalloutNV.exe 1.4.0.525)
/// BSScrapArray<NiPointer<NiAVObject>,1024> scalar deleting destructor (Xbox PDB).
pub fn scrap_array_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scrap_array_destruct(e, this);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

// Translated from 0087d2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the ActorHavokSyncData array vtable and empty storage (Xbox PDB).
pub fn fn_0087d2c0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), 0x010840d0);
    fn_0087d340(e, this, 0, 0);
    this
}

// Translated from 0087d2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// BSSimpleArray<ActorHavokSyncData,1024> scalar deleting destructor (Xbox PDB).
pub fn actor_sync_array_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0087d320(e, this);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

// Translated from 0087d320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructs the ActorHavokSyncData array storage (Xbox PDB).
pub fn fn_0087d320(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010840d0);
    e.call(0x008454f0, &args![this, 1u32]);
}

// Translated from 0087d340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes array storage with unsigned max(capacity,size), then constructs size entries.
pub fn fn_0087d340(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = capacity.max(size);
    if capacity != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_0087d3d0(e, this.cast(), Ptr::new(buffer), size);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 0087d3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes each eight-byte entry through the placement-allocation helper.
pub fn fn_0087d3d0(e: &mut Engine, _unused_this: Ptr, buffer: Ptr, count: u32) {
    for index in 0..count {
        e.call(
            0x006e6da0,
            &args![8u32, buffer.addr().wrapping_add(index.wrapping_mul(8))],
        );
    }
}

// Translated from 0087d420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the NiNode pointer array vtable and empty storage (Xbox PDB).
pub fn fn_0087d420(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010840e4);
    e.call(0x00822860, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0087d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// BSSimpleArray<NiPointer<NiNode>,1024> scalar deleting destructor (Xbox PDB).
pub fn node_array_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0087d480, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

/// The dispatcher calls the original vector constructor; do not substitute
/// stores because its return and memory effects belong to another unit.
fn unpack_vector(e: &mut Engine, task: Ptr, output: Ptr) {
    let x = x87_quiet(e.mem.f32(task.addr() + 8));
    let y = x87_quiet(e.mem.f32(task.addr() + 12));
    let z = x87_quiet(e.mem.f32(task.addr() + 16));
    e.call(0x00416870, &args![output, x, y, z]);
}

fn unpack_script_task(e: &mut Engine, task: Ptr) {
    let object = e.call(0x004839c0, &args![e.mem.u32(task.addr() + 8)]).u32();
    if object == 0 {
        return;
    }
    match e.mem.u32(task.addr() + 4) {
        0x1007 | 0x1009 => {
            let callee = if e.mem.u32(task.addr() + 4) == 0x1007 {
                0x005c0740
            } else {
                0x005c0b10
            };
            let axis = e.mem.u8(task.addr() + 12);
            let value = x87_quiet(e.mem.f32(task.addr() + 16));
            e.call(callee, &args![object, axis, value]);
        }
        0x113c | 0x1156 | 0x116c => {
            let callee = match e.mem.u32(task.addr() + 4) {
                0x113c => 0x005c1c10,
                0x1156 => 0x005d3c20,
                _ => 0x005d4c20,
            };
            let value = x87_quiet(e.mem.f32(task.addr() + 12));
            e.call(callee, &args![object, value]);
        }
        0x109e | 0x124f => {
            let target = e
                .call(0x004839c0, &args![e.mem.u32(task.addr() + 12)])
                .u32();
            let x = x87_quiet(e.mem.f32(task.addr() + 16));
            let y = x87_quiet(e.mem.f32(task.addr() + 20));
            let z = x87_quiet(e.mem.f32(task.addr() + 24));
            let fade = u32::from(e.mem.u32(task.addr() + 4) == 0x124f);
            let use_fade = e.vcall(object, 0x100, &[]).u8() != 0
                && object != e.global::<u32>(0x011dea3c)
                && e.call(0x008d8520, &args![object]).u32() != 0
                && e.call(0x00931850, &args![object]).u32() == 0
                && e.vcall(object, 0x420, &[]).f64() > e.global::<f64>(0x01012060)
                && (fade as f64) > e.global::<f64>(0x01012060);
            if use_fade {
                let process = e.call(0x008d8520, &args![object]).u32();
                e.call(0x008fe9e0, &args![process, object, target, x, y, z]);
            } else {
                e.call(0x005ccb20, &args![object, target, x, y, z]);
            }
        }
        0x11df => {
            let target = e
                .call(0x004839c0, &args![e.mem.u32(task.addr() + 12)])
                .u32();
            e.call(
                0x005d9bc0,
                &args![object, target, e.mem.u32(task.addr() + 16)],
            );
        }
        _ => {}
    }
}

// Translation incomplete: 0087b990 - compiler exception unwinding for the temporary scrap array is not implemented.
// Translated from 0087b990 (decompiled, FalloutNV.exe 1.4.0.525)
/// TaskQueueInterface::TaskUnpackFunc (Xbox PDB). Dispatches tags 0..0x31;
/// tag 0x2B and out-of-range tags do nothing. The PC jump table at 0087cd88
/// determines the cases, including tag 0x0C falling through to 0x0D.
#[allow(clippy::too_many_lines)]
pub fn task_queue_interface_task_unpack_func(e: &mut Engine, task: Ptr) {
    let tag = e.mem.u32(task.addr());
    match tag {
        0 => {
            let value = x87_quiet(e.mem.f32(task.addr() + 4)) as f64;
            e.call(0x005b5e40, &args![0x01084064u32, value]);
        }
        1 => {
            e.call(
                0x005b5e40,
                &args![0x0108405cu32, e.mem.u32(task.addr() + 4)],
            );
        }
        2 => {
            let object = e.mem.u32(task.addr() + 4);
            e.with_stack(12, |e, vector| {
                unpack_vector(e, task, vector);
                e.call(0x00b553d0, &args![object, vector]);
            });
        }
        3 => {
            e.call(0x00552390, &args![e.mem.u32(task.addr() + 4)]);
        }
        4 => {
            if e.call(0x0044ddc0, &args![0x011df244u32]).u32() != 0 {
                let count = e.call(0x0044ddc0, &args![0x011df244u32]).u32();
                e.with_stack(16, |e, descriptor| {
                    fn_0087cf10(e, descriptor, 0x0087cfb0, 0x011df244, count);
                    e.call(0x00c44c90, &args![e.global::<u32>(0x01202df4), descriptor]);
                });
                e.call(0x008454f0, &args![0x011df244u32, 0u32]);
            }
            if e.call(0x0044ddc0, &args![0x011df1c4u32]).u32() != 0 {
                e.with_stack(36, |e, array| {
                    scrap_array_construct(e, array);
                    fn_0087d030(e, array);
                    let count = e.call(0x0044ddc0, &args![array]).u32();
                    let descriptor = Ptr::new(array.addr() + 20);
                    fn_0087cf10(e, descriptor, 0x0087cf50, array.addr(), count);
                    e.call(0x00c44c90, &args![e.global::<u32>(0x01202df4), descriptor]);
                    scrap_array_destruct(e, array);
                });
            }
            let gate = e.call(0x00408d60, &args![0x011df1b8u32]).u32();
            if e.mem.u8(gate) == 0 {
                e.call(0x00453900, &args![e.global::<u32>(0x011dea10)]);
            }
        }
        5 => {
            let object = e.mem.u32(task.addr() + 4);
            let was_zero = fn_0087ce50(e, Ptr::new(object)) == e.global::<f64>(0x01012060);
            e.call(0x005526f0, &args![object]);
            if was_zero && fn_0087ce50(e, Ptr::new(object)) > e.global::<f64>(0x01012060) {
                let reference = e.call(0x0056f930, &args![object]).u32();
                let cell = e
                    .call(0x00457070, &args![e.global::<u32>(0x011dea10)])
                    .u32();
                e.call(0x0054a070, &args![cell, reference, 0u32, 0u32]);
            }
            e.call(0x00401970, &args![object]);
        }
        6 | 0x2f => {
            let object = e.mem.u32(task.addr() + 4);
            e.call(
                if tag == 6 { 0x00552790 } else { 0x004ab0c0 },
                &args![object],
            );
            e.call(0x00401970, &args![object]);
        }
        7 => {
            let child = e.mem.u32(task.addr() + 4);
            let parent = e.mem.u32(task.addr() + 8);
            let attach = e.mem.u8(task.addr() + 12);
            let properties = e.mem.u8(task.addr() + 16);
            e.vcall(parent, 0xdc, &args![child, attach]);
            e.vcall(child, 0xd0, &[]);
            if properties != 0 {
                e.call(0x00a5a040, &args![child]);
            }
            e.call(0x00401970, &args![child]);
            e.call(0x00401970, &args![parent]);
        }
        8 => {
            let object = e.mem.u32(task.addr() + 4);
            let parent = e.call(0x009611e0, &args![object]).u32();
            if parent != 0 {
                e.vcall(parent, 0xe8, &args![object]);
            }
            e.call(0x00401970, &args![object]);
        }
        9 => {
            let object = e.mem.u32(task.addr() + 4);
            let scene = e.call(0x00450b80, &args![e.mem.u32(task.addr() + 8)]).u32();
            if scene != 0 {
                e.call(0x00b5b1c0, &args![scene, object]);
            }
            e.call(0x00401970, &args![object]);
        }
        10 => {
            let object = e.mem.u32(task.addr() + 4);
            e.with_stack(12, |e, update| {
                e.call(0x0043d410, &args![update, 0.0f32, 0u32, 0u32]);
                e.call(0x00a59c60, &args![object, update]);
            });
            e.call(0x00401970, &args![object]);
        }
        11 => {
            let actor = e.mem.u32(task.addr() + 4);
            let data = e.mem.u32(task.addr() + 8);
            let value = x87_quiet(e.mem.f32(data + 0x14));
            e.call(0x0088e8d0, &args![actor, value, data]);
            if e.mem.u32(data) != 0 {
                let value = x87_quiet(e.mem.f32(data + 0x14));
                e.call(0x0088fb00, &args![e.mem.u32(data), value, data, actor]);
            }
            fn_0087cea0(e, Ptr::new(data));
        }
        12 | 13 => {
            if tag == 12 {
                let object = e.mem.u32(task.addr() + 4);
                if e.vcall(object, 0x220, &[]).u8() != 0 {
                    e.vcall(object, 0x304, &[]);
                }
            }
            let object = e.mem.u32(task.addr() + 4);
            let value = x87_quiet(e.mem.f32(task.addr() + 8));
            let flag = e.mem.u8(task.addr() + 12);
            let collection = fn_0087ce70(e);
            let iterator = e.call(0x004b9ba0, &args![collection]).u32();
            e.with_stack(12, |e, state| {
                e.mem.set_u32(state.addr(), iterator);
                e.mem.set_u32(state.addr() + 4, 0);
                let mut found = false;
                while e.mem.u32(state.addr()) != 0 && !found {
                    e.mem.set_u32(state.addr() + 8, 0);
                    e.call(
                        0x006b7f20,
                        &args![collection, state, state.addr() + 8, state.addr() + 4],
                    );
                    found = e.mem.u32(state.addr() + 8) == object;
                }
                if found {
                    if e.vcall(object, 0x100, &[]).u8() != 0 {
                        e.vcall(object, 0x338, &args![value, 0.0f32, 0u32]);
                    } else {
                        e.vcall(object, 0x144, &args![value, flag]);
                    }
                }
            });
        }
        14 => {
            let actor = e.mem.u32(task.addr() + 4);
            let value_ptr = e.mem.u32(task.addr() + 8);
            let value = x87_quiet(e.mem.f32(value_ptr));
            e.call(0x0089d900, &args![actor, e.mem.u32(actor + 0xc0), value]);
            let dismember = e.mem.u32(task.addr() + 12);
            if dismember != 0 {
                e.call(
                    0x008b51b0,
                    &args![
                        actor,
                        e.mem.u32(dismember + 4),
                        e.mem.u32(dismember),
                        e.mem.u32(dismember + 8)
                    ],
                );
                e.call(0x00401030, &args![dismember]);
            }
            e.call(0x00401030, &args![value_ptr]);
        }
        15 => {
            e.call(
                0x00899200,
                &args![
                    e.mem.u32(task.addr() + 4),
                    e.mem.u8(task.addr() + 8),
                    e.mem.u8(task.addr() + 12)
                ],
            );
        }
        16 => {
            let actor = e.mem.u32(task.addr() + 4);
            e.with_stack(12, |e, vector| {
                unpack_vector(e, task, vector);
                let force = x87_quiet(e.mem.f32(task.addr() + 20));
                if actor != 0 && e.call(0x008d8520, &args![actor]).u32() != 0 {
                    let process = e.call(0x008d8520, &args![actor]).u32();
                    e.vcall(
                        process,
                        0x418,
                        &args![
                            actor,
                            e.mem.u32(vector.addr()),
                            e.mem.u32(vector.addr() + 4),
                            e.mem.u32(vector.addr() + 8),
                            force
                        ],
                    );
                }
            });
        }
        17 => {
            let actor = e.mem.u32(task.addr() + 4);
            let data = e.mem.u32(task.addr() + 8);
            if actor != 0 {
                e.call(0x0089a760, &args![actor, data, 0u32]);
            }
            fn_0087cea0(e, Ptr::new(data));
        }
        18 => {
            let object = e.mem.u32(task.addr() + 4);
            if object != 0 {
                let effect = e.call(0x0081b8e0, &args![object]).u32();
                fn_0087ce80(e, Ptr::new(object), effect);
            }
        }
        19 => {
            let object = e.mem.u32(task.addr() + 4);
            if object != 0 {
                let reference = e.call(0x0056f930, &args![object]).u32();
                let biped = if reference != 0 {
                    e.call(0x008d6f30, &args![reference]).u32()
                } else {
                    0
                };
                let node = if biped != 0 {
                    e.call(0x004543c0, &args![biped]).u32()
                } else {
                    0
                };
                if node != 0 {
                    e.vcall(node, 0xd0, &args![object, 1u32, 1u32, 0u32, 0u32]);
                }
                e.call(0x00401970, &args![object]);
            }
        }
        20 => {
            let object = e.mem.u32(task.addr() + 4);
            if object != 0 {
                e.call(0x00c6a270, &args![object, 1u32, 1u32, 0u32]);
                e.call(0x00c69ee0, &args![object, 1u32, 0u32]);
                e.call(0x00401970, &args![object]);
            }
        }
        21 => {
            e.call(
                0x00571760,
                &args![e.mem.u32(task.addr() + 4), e.mem.u32(task.addr() + 8)],
            );
        }
        22 => {
            let actor = e.mem.u32(task.addr() + 4);
            let weapon = e.mem.u32(task.addr() + 8);
            if e.call(0x008d8520, &args![actor]).u32() != 0 {
                let process = e.call(0x008d8520, &args![actor]).u32();
                if e.vcall(process, 0x148, &[]).u32() != 0 {
                    let process = e.call(0x008d8520, &args![actor]).u32();
                    let entry = e.vcall(process, 0x148, &[]).u32();
                    if e.call(0x0044ddc0, &args![entry]).u32() == weapon {
                        e.call(0x00571b50, &args![actor]);
                        if e.call(0x008d8520, &args![actor]).u32() != 0 {
                            let process = e.call(0x008d8520, &args![actor]).u32();
                            e.vcall(process, 0x160, &args![0u32, 0u32, 0u32]);
                        }
                    }
                }
            }
        }
        23 => {
            e.vcall(e.mem.u32(task.addr() + 4), 0x1cc, &args![0u32, 0u32]);
        }
        24 => {
            e.call(
                0x008b52a0,
                &args![
                    e.mem.u32(task.addr() + 4),
                    e.mem.u32(task.addr() + 8),
                    e.mem.u32(task.addr() + 12),
                    e.mem.u8(task.addr() + 12),
                    e.mem.u8(task.addr() + 16),
                    1u32,
                    0u32
                ],
            );
        }
        25 | 26 => {
            let object = e.mem.u32(task.addr() + 4);
            let group = e.mem.u32(task.addr() + 8);
            let value = x87_quiet(e.mem.f32(task.addr() + 12));
            let water = e
                .call(0x0070ec90, &args![e.global::<u32>(0x011dea10)])
                .u32();
            e.call(
                if tag == 25 { 0x004e5c80 } else { 0x004e5df0 },
                &args![water, object, group, value],
            );
        }
        27 | 28 => {
            let object = e.mem.u32(task.addr() + 4);
            let manager = e.call(0x006c0720, &[]).u32();
            e.call(
                if tag == 27 { 0x006c0dc0 } else { 0x006c0f10 },
                &args![manager, object],
            );
        }
        29 | 39 | 40 => {
            let object = e.mem.u32(task.addr() + 4);
            if object != 0 {
                e.call(
                    match tag {
                        29 => 0x008d3fa0,
                        39 => 0x008a1a70,
                        _ => 0x00887d00,
                    },
                    &args![object],
                );
            }
        }
        30 | 38 => {
            let object = e.mem.u32(task.addr() + 4);
            let first = e.mem.u8(task.addr() + 8);
            let second = e.mem.u8(task.addr() + 12);
            if object != 0 {
                e.call(
                    if tag == 30 { 0x0047aec0 } else { 0x008a7d50 },
                    &args![object, first, second],
                );
            }
        }
        31 => unpack_script_task(e, task),
        32 => {
            e.call(
                0x00523150,
                &args![e.mem.u32(task.addr() + 4), e.mem.u32(task.addr() + 8)],
            );
        }
        33 => {
            let object = e.mem.u32(task.addr() + 4);
            e.with_stack(12, |e, vector| {
                unpack_vector(e, task, vector);
                if object != 0 {
                    e.vcall(object, 0x2a8, &args![vector]);
                }
            });
        }
        34 => {
            let object = e.mem.u32(task.addr() + 4);
            let motion = e.mem.u32(task.addr() + 8);
            if object != 0 {
                e.call(0x00c6a350, &args![object, motion, 1u32, 0u32, 1u32]);
                e.call(0x00401970, &args![object]);
            }
        }
        35 => {
            let object = e.mem.u32(task.addr() + 4);
            let flag = e.mem.u8(task.addr() + 8);
            if object != 0 {
                e.call(0x008a5f20, &args![object, flag]);
            }
        }
        36 | 37 => {
            let value = x87_quiet(e.mem.f32(task.addr() + 4));
            e.with_stack(12, |e, vector| {
                unpack_vector(e, task, vector);
                let x = e.mem.u32(vector.addr());
                let y = e.mem.u32(vector.addr() + 4);
                let z = e.mem.u32(vector.addr() + 8);
                if tag == 36 {
                    e.call(0x0062eb50, &args![value, x, y, z]);
                } else {
                    let water = e
                        .call(0x0070ec90, &args![e.global::<u32>(0x011dea10)])
                        .u32();
                    e.call(0x004e8510, &args![water, x, y, z, value]);
                }
            });
        }
        41 => {
            e.vcall(e.mem.u32(task.addr() + 4), 0x1fc, &args![0u32]);
        }
        42 => {
            let object = e.call(0x004839c0, &args![e.mem.u32(task.addr() + 4)]).u32();
            if object != 0 && e.vcall(object, 0xf0, &[]).u8() != 0 {
                e.vcall(object, 0x1c0, &[]);
            }
        }
        44 => {
            let actor = e.mem.u32(task.addr() + 4);
            let controller = e.call(0x009306d0, &args![actor]).u32();
            if controller != 0 {
                let position = e.vcall(actor, 0x1f4, &[]).u32();
                e.call(0x005620e0, &args![controller, position]);
                let delta = e.call(0x00458b20, &[]).u32();
                e.call(0x0080fb00, &args![controller, delta]);
                e.call(0x005d10f0, &args![controller]);
                fn_0087cee0(e, Ptr::new(controller));
                e.call(0x00c6e030, &args![controller]);
            }
        }
        45 => {
            e.vcall(e.mem.u32(task.addr() + 4), 0x1c4, &[]);
        }
        46 => {
            let object = e.mem.u32(task.addr() + 4);
            if object != 0 {
                e.vcall(object, 0, &args![1u32]);
            }
        }
        48 => {
            e.call(
                0x00892460,
                &args![e.mem.u32(task.addr() + 4), e.mem.u8(task.addr() + 8)],
            );
        }
        49 => {
            let object = e.mem.u32(task.addr() + 4);
            e.call(
                0x00c69fc0,
                &args![
                    object,
                    e.mem.u8(task.addr() + 8),
                    e.mem.u8(task.addr() + 12),
                    e.mem.u8(task.addr() + 16)
                ],
            );
            e.call(0x00401970, &args![object]);
        }
        _ => {}
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0087b990, task_queue_interface_task_unpack_func(Ptr)),
        entry!(0x0087b650, fn_0087b650(Ptr<TaskQueueInterface>, u32, Ptr)),
        entry!(0x0087b700, fn_0087b700(Ptr<TaskQueueInterface>, f32, Ptr)),
        entry!(
            0x0087b760,
            task_queue_interface_queue_add_ripple(Ptr<TaskQueueInterface>, f32, Ptr)
        ),
        entry!(
            0x0087b6b0,
            task_queue_interface_queue_actor_set_havok_weapon(Ptr<TaskQueueInterface>, u32, u8)
        ),
        entry!(
            0x0087b7c0,
            task_queue_interface_queue_actor_trigger_pain(Ptr<TaskQueueInterface>, u32, u8, u8)
        ),
        entry!(
            0x0087b810,
            task_queue_interface_queue_actor_apply_critical_stage(Ptr<TaskQueueInterface>, u32)
        ),
        entry!(
            0x0087b850,
            task_queue_interface_queue_actor_reset_loaded_anims(Ptr<TaskQueueInterface>, u32)
        ),
        entry!(
            0x0087b890,
            task_queue_interface_queue_delete_active_effect(Ptr<TaskQueueInterface>, u32)
        ),
        entry!(0x0087b8d0, fn_0087b8d0(Ptr, Ptr)),
        entry!(0x0087b950, fn_0087b950()),
        entry!(0x0087ce50, fn_0087ce50(Ptr) -> f64),
        entry!(0x0087ce70, fn_0087ce70() -> Ptr),
        entry!(0x0087ce80, fn_0087ce80(Ptr, u32)),
        entry!(0x0087cea0, fn_0087cea0(Ptr)),
        entry!(0x0087cee0, fn_0087cee0(Ptr) -> bool),
        entry!(0x0087cf10, fn_0087cf10(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0087cf50, fn_0087cf50(Ptr, u32)),
        entry!(
            0x0087cfb0,
            actor_havok_sync_parallel_array_task_func(Ptr, u32)
        ),
        entry!(0x0087d030, fn_0087d030(Ptr)),
        entry!(
            0x0087d160,
            task_queue_interface_queue_if_active(Ptr<TaskQueueInterface>, Ptr)
        ),
        entry!(0x0087d1b0, scrap_array_construct(Ptr) -> Ptr),
        entry!(0x0087d230, scrap_array_destruct(Ptr)),
        entry!(0x0087d290, scrap_array_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x0087d2c0,
            fn_0087d2c0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x0087d2f0, actor_sync_array_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0087d320, fn_0087d320(Ptr)),
        entry!(0x0087d340, fn_0087d340(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0087d3d0, fn_0087d3d0(Ptr, Ptr, u32)),
        entry!(0x0087d420, fn_0087d420(Ptr) -> Ptr),
        entry!(0x0087d450, node_array_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x0087a670,
            fn_0087a670(Ptr<TaskQueueInterface>, Ptr, Ptr) -> Ptr<TaskQueueInterface>
        ),
        entry!(0x0087a890, fn_0087a890(Ptr, u32) -> u8),
        entry!(
            0x0087aa40,
            task_queue_interface_queue_weapon_fire(Ptr<TaskQueueInterface>, u32, u32)
        ),
        entry!(0x0087aa90, fn_0087aa90(Ptr<TaskQueueInterface>)),
        entry!(0x0087aad0, fn_0087aad0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087ab50, fn_0087ab50(Ptr<TaskQueueInterface>, u32, u32)),
        entry!(0x0087abb0, fn_0087abb0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087ac00,
            task_queue_interface_queue_node_attach(Ptr<TaskQueueInterface>, Ptr, Ptr, u8, u8)
        ),
        entry!(0x0087ac60, fn_0087ac60(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087acb0, fn_0087acb0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087ad00, fn_0087ad00(Ptr<TaskQueueInterface>, Ptr, u32)),
        entry!(0x0087ad50, fn_0087ad50(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087ada0, fn_0087ada0(Ptr<TaskQueueInterface>, u32, Ptr)),
        entry!(0x0087adf0, fn_0087adf0(Ptr)),
        entry!(0x0087ae10, fn_0087ae10(Ptr<TaskQueueInterface>, u32, Ptr)),
        entry!(0x0087ae60, fn_0087ae60(Ptr<TaskQueueInterface>, Ptr)),
        entry!(0x0087aec0, fn_0087aec0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087af50,
            fn_0087af50(Ptr<TaskQueueInterface>, Ptr, f32, u8)
        ),
        entry!(0x0087afa0, fn_0087afa0(Ptr<TaskQueueInterface>, Ptr, u8)),
        entry!(
            0x0087aff0,
            task_queue_interface_queue_actor_kill(Ptr<TaskQueueInterface>, Ptr, Ptr, u32)
        ),
        entry!(
            0x0087b040,
            fn_0087b040(Ptr<TaskQueueInterface>, Ptr, u8, u8)
        ),
        entry!(
            0x0087b090,
            task_queue_interface_queue_actor_knock_explosion(
                Ptr<TaskQueueInterface>,
                Ptr,
                f32,
                f32,
                f32,
                f32,
            )
        ),
        entry!(0x0087b0f0, fn_0087b0f0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087b140,
            task_queue_interface_queue_havok_remove(Ptr<TaskQueueInterface>, Ptr)
        ),
        entry!(0x0087b190, fn_0087b190(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087b1e0,
            task_queue_interface_queue_havok_motion_change(Ptr<TaskQueueInterface>, Ptr, u32)
        ),
        entry!(0x0087b230, fn_0087b230(Ptr<TaskQueueInterface>, u32)),
        entry!(0x0087b270, fn_0087b270(Ptr<TaskQueueInterface>, u32)),
        entry!(0x0087b2b0, fn_0087b2b0(Ptr<TaskQueueInterface>, Ptr)),
        entry!(
            0x0087b300,
            task_queue_interface_queue_havok_adjust_force_add_count(
                Ptr<TaskQueueInterface>,
                Ptr,
                u8,
                u8,
                u8,
            )
        ),
        entry!(
            0x0087b360,
            task_queue_interface_queue_attach_weapon(Ptr<TaskQueueInterface>, u32, u32)
        ),
        entry!(0x0087b3b0, fn_0087b3b0(Ptr<TaskQueueInterface>, u32, u32)),
        entry!(
            0x0087b400,
            task_queue_interface_queue_set_3d_null(Ptr<TaskQueueInterface>, u32)
        ),
        entry!(
            0x0087b440,
            task_queue_interface_queue_dismember_limb(
                Ptr<TaskQueueInterface>,
                u32,
                u32,
                u32,
                u8,
                u8,
            )
        ),
        entry!(
            0x0087b4a0,
            fn_0087b4a0(Ptr<TaskQueueInterface>, u32, u32, f32)
        ),
        entry!(
            0x0087b4f0,
            fn_0087b4f0(Ptr<TaskQueueInterface>, u32, u32, f32)
        ),
        entry!(0x0087b540, fn_0087b540(Ptr<TaskQueueInterface>, u32)),
        entry!(0x0087b580, fn_0087b580(Ptr<TaskQueueInterface>, u32)),
        entry!(
            0x0087b5c0,
            task_queue_interface_queue_character_reset_3d(Ptr<TaskQueueInterface>, u32)
        ),
        entry!(
            0x0087b600,
            task_queue_interface_queue_set_open_state(Ptr<TaskQueueInterface>, u32, u8, u8)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAPTURE_TASK: u32 = 0x0f00_f100;
    fn capture_tasks(
        e: &mut Engine,
        this: Ptr<TaskQueueInterface>,
        mut capture: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        e.map(0x011a31f4, 8);
        e.set_global(0x011a31f4, 1u8);
        let queue = virtual_object(e, 4, CAPTURE_TASK);
        e.register_double(0x0044ddc0, move |_, a| {
            assert_eq!(a, &[this.addr()]);
            queue.into_ret()
        });
        e.register_double(CAPTURE_TASK, move |e, a| {
            assert_eq!(a[0], queue.addr());
            capture(e, &[this.addr(), a[1]]);
            1u32.into_ret()
        });
    }

    /// Checks the task's initialized slots, exact argument order, retention
    /// before dispatch, byte truncation, float bits, and null-call suppression.
    fn check_record(
        address: u32,
        arguments: &[u32],
        tag: u32,
        payload: &[u32],
        before: &[(u32, Vec<u32>)],
        null_gate: Option<usize>,
        byte_slots: &[usize],
    ) {
        let mut e = Engine::new();
        let this: Ptr<TaskQueueInterface> = e.new_object();
        for callee in [0x0040_f6e0, 0x0040_b460] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(0x0084_e3a0, |_, a| {
            assert_eq!(a, &[0x1234_5678]);
            0x8765_4321u32.into_ret()
        });
        let expected = payload.to_vec();
        let byte_slots = byte_slots.to_vec();
        capture_tasks(&mut e, this, move |e, a| {
            assert_eq!(a.len(), 2);
            assert_eq!(a[0], this.addr());
            assert_eq!(e.mem.u32(a[1]), tag);
            for (index, value) in expected.iter().enumerate() {
                let slot = a[1] + 4 * (index as u32 + 1);
                if byte_slots.contains(&index) {
                    assert_eq!(e.mem.u8(slot), *value as u8);
                } else {
                    assert_eq!(e.mem.u32(slot), *value);
                }
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![this.addr()];
        words.extend_from_slice(arguments);
        e.call(address, &words);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), before.len() + 3);
        assert_eq!(&log[1..log.len() - 2], before);
        assert_eq!(log[log.len() - 2].0, 0x0044ddc0);
        assert_eq!(log.last().unwrap().0, CAPTURE_TASK);
        if let Some(index) = null_gate {
            words[index + 1] = 0;
            e.call_log = Some(vec![]);
            e.call(address, &words);
            assert_eq!(e.call_log.take().unwrap(), vec![(address, words)]);
        }
    }

    #[test]
    fn record_0087aa40() {
        check_record(
            0x0087aa40,
            &[0x12345678, 0x90abcdef],
            0x20,
            &[0x12345678, 0x90abcdef],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087aa90() {
        check_record(0x0087aa90, &[], 0x4, &[], &[], None, &[]);
    }

    #[test]
    fn record_0087abb0() {
        check_record(
            0x0087abb0,
            &[0x12345678],
            0x6,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087ac00() {
        check_record(
            0x0087ac00,
            &[0x12345678, 0x90abcdef, 0xdeadbe83, 0xdeadbe84],
            0x7,
            &[0x12345678, 0x90abcdef, 0x00000083, 0x00000084],
            &[
                (0x0040f6e0, vec![0x12345678]),
                (0x0040f6e0, vec![0x90abcdef]),
            ],
            None,
            &[2, 3],
        );
    }

    #[test]
    fn record_0087ac60() {
        check_record(
            0x0087ac60,
            &[0x12345678],
            0x8,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087acb0() {
        check_record(
            0x0087acb0,
            &[0x12345678],
            0x2f,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087ad00() {
        check_record(
            0x0087ad00,
            &[0x12345678, 0x90abcdef],
            0x9,
            &[0x12345678, 0x90abcdef],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087ad50() {
        check_record(
            0x0087ad50,
            &[0x12345678],
            0xa,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087ada0() {
        check_record(
            0x0087ada0,
            &[0x12345678, 0x90abcdef],
            0xb,
            &[0x12345678, 0x90abcdef],
            &[(0x0040b460, vec![0x90abce4f])],
            Some(1),
            &[],
        );
    }

    #[test]
    fn record_0087ae10() {
        check_record(
            0x0087ae10,
            &[0x12345678, 0x90abcdef],
            0x11,
            &[0x12345678, 0x90abcdef],
            &[(0x0040b460, vec![0x90abce4f])],
            Some(1),
            &[],
        );
    }

    #[test]
    fn record_0087af50() {
        check_record(
            0x0087af50,
            &[0x12345678, 0xc0600000, 0xdeadbe83],
            0xd,
            &[0x12345678, 0xc0600000, 0x00000083],
            &[],
            Some(0),
            &[2],
        );
    }

    #[test]
    fn record_0087afa0() {
        check_record(
            0x0087afa0,
            &[0x12345678, 0xdeadbe82],
            0x30,
            &[0x12345678, 0x00000082],
            &[],
            Some(0),
            &[1],
        );
    }

    #[test]
    fn record_0087aff0() {
        check_record(
            0x0087aff0,
            &[0x12345678, 0x90abcdef, 0x10203042],
            0xe,
            &[0x12345678, 0x90abcdef, 0x10203042],
            &[],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b040() {
        check_record(
            0x0087b040,
            &[0x12345678, 0xdeadbe82, 0xdeadbe83],
            0xf,
            &[0x12345678, 0x00000082, 0x00000083],
            &[],
            Some(0),
            &[1, 2],
        );
    }

    #[test]
    fn record_0087b090() {
        check_record(
            0x0087b090,
            &[0x12345678, 0xc0600000, 0x80000000, 0x3fa00000, 0x40980000],
            0x10,
            &[0x12345678, 0xc0600000, 0x80000000, 0x3fa00000, 0x40980000],
            &[],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b0f0() {
        check_record(
            0x0087b0f0,
            &[0x12345678],
            0x13,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b140() {
        check_record(
            0x0087b140,
            &[0x12345678],
            0x14,
            &[0x12345678],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b190() {
        check_record(
            0x0087b190,
            &[0x12345678],
            0x2c,
            &[0x12345678],
            &[],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b1e0() {
        check_record(
            0x0087b1e0,
            &[0x12345678, 0x90abcdef],
            0x22,
            &[0x12345678, 0x90abcdef],
            &[(0x0040f6e0, vec![0x12345678])],
            Some(0),
            &[],
        );
    }

    #[test]
    fn record_0087b230() {
        check_record(
            0x0087b230,
            &[0x12345678],
            0x2d,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b270() {
        check_record(
            0x0087b270,
            &[0x12345678],
            0x29,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b2b0() {
        check_record(
            0x0087b2b0,
            &[0x12345678],
            0x2a,
            &[0x87654321],
            &[(0x0084e3a0, vec![0x12345678])],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b300() {
        check_record(
            0x0087b300,
            &[0x12345678, 0xdeadbe82, 0xdeadbe83, 0xdeadbe84],
            0x31,
            &[0x12345678, 0x00000082, 0x00000083, 0x00000084],
            &[(0x0040f6e0, vec![0x12345678])],
            None,
            &[1, 2, 3],
        );
    }

    #[test]
    fn record_0087b360() {
        check_record(
            0x0087b360,
            &[0x12345678, 0x90abcdef],
            0x15,
            &[0x12345678, 0x90abcdef],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b3b0() {
        check_record(
            0x0087b3b0,
            &[0x12345678, 0x90abcdef],
            0x16,
            &[0x12345678, 0x90abcdef],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b400() {
        check_record(
            0x0087b400,
            &[0x12345678],
            0x17,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b440() {
        check_record(
            0x0087b440,
            &[0x12345678, 0x90abcdef, 0x10203042, 0xdeadbe84, 0xdeadbe85],
            0x18,
            &[0x12345678, 0x90abcdef, 0x10203042, 0x00000084, 0x00000085],
            &[],
            None,
            &[3, 4],
        );
    }

    #[test]
    fn record_0087b4a0() {
        check_record(
            0x0087b4a0,
            &[0x12345678, 0x90abcdef, 0x80000000],
            0x19,
            &[0x12345678, 0x90abcdef, 0x80000000],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b4f0() {
        check_record(
            0x0087b4f0,
            &[0x12345678, 0x90abcdef, 0x80000000],
            0x1a,
            &[0x12345678, 0x90abcdef, 0x80000000],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b540() {
        check_record(
            0x0087b540,
            &[0x12345678],
            0x1b,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b580() {
        check_record(
            0x0087b580,
            &[0x12345678],
            0x1c,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b5c0() {
        check_record(
            0x0087b5c0,
            &[0x12345678],
            0x1d,
            &[0x12345678],
            &[],
            None,
            &[],
        );
    }

    #[test]
    fn record_0087b600() {
        check_record(
            0x0087b600,
            &[0x12345678, 0xdeadbe82, 0xdeadbe83],
            0x1e,
            &[0x12345678, 0x00000082, 0x00000083],
            &[],
            None,
            &[1, 2],
        );
    }

    #[test]
    fn constructor_sets_all_fields_returns_this_and_preserves_guards() {
        // Lead native 0087a670 vectors: generated guarded receivers, each
        // fill combined with both queue pairs. No executable data is stored.
        for fill in [0xa5u8, 0xcc, 0xff] {
            for (first, second) in [(0u32, u32::MAX), (0x1234_5678, 0x90ab_cdef)] {
                let mut e = Engine::new();
                let buffer = e.mem.alloc(0x18);
                e.mem.write(buffer, &[fill; 0x18]);
                let this: Ptr<TaskQueueInterface> = Ptr::new(buffer + 4);
                let result = e.call(0x0087_a670, &args![this, first, second]).u32();
                assert_eq!(result, this.addr());
                assert_eq!(e.get(this, TaskQueueInterface::pQueue).addr(), first);
                assert_eq!(
                    e.get(this, TaskQueueInterface::pSecondaryQueue).addr(),
                    second
                );
                assert_eq!(e.get(this, TaskQueueInterface::pActiveQueue).addr(), first);
                assert_eq!(e.get(this, TaskQueueInterface::iProcessingThread), 0);
                assert_eq!(e.mem.u32(buffer), u32::from_le_bytes([fill; 4]));
                assert_eq!(e.mem.u32(buffer + 0x14), u32::from_le_bytes([fill; 4]));
            }
        }
    }

    fn virtual_object(e: &mut Engine, slot: u32, callback: u32) -> Ptr {
        let object = e.mem.alloc(4);
        let table = e.mem.alloc(slot + 4);
        e.mem.set_u32(object, table);
        e.mem.set_u32(table + slot, callback);
        Ptr::new(object)
    }

    #[test]
    fn virtual_forwarder_reloads_current_vtable_and_keeps_argument() {
        let mut e = Engine::new();
        let object = virtual_object(&mut e, 4, 0x0f00_0010);
        let second_table = e.mem.alloc(8);
        e.mem.set_u32(second_table + 4, 0x0f00_0020);
        e.register_double(0x0f00_0010, move |e, a| {
            assert_eq!(a, &[object.addr(), 0xfedc_ba98]);
            e.mem.set_u32(object.addr(), second_table);
            Ret::default()
        });
        e.register_double(0x0f00_0020, move |_, a| {
            assert_eq!(a, &[object.addr(), 0x7654_3210]);
            0xffff_ff81u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_a890, &args![object, 0xfedc_ba98u32]).u8(), 0);
        assert_eq!(
            e.call(0x0087_a890, &args![object, 0x7654_3210u32]).u8(),
            0x81
        );
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|c| c.0)
            .collect();
        assert_eq!(calls, [0x0087_a890, 0x0f00_0010, 0x0087_a890, 0x0f00_0020]);
    }

    #[test]
    fn guarded_reference_insertion_preserves_normal_cleanup_and_disabled_path() {
        for enabled_byte in [0u8, 0x80] {
            let mut e = Engine::new();
            let enabled = e.mem.alloc(1);
            e.mem.set_u8(enabled, enabled_byte);
            e.register_double(0x0040_8d60, move |_, a| {
                assert_eq!(a, &[0x011d_f1b8]);
                enabled.into_ret()
            });
            e.register(0x0063_3c90, |e, a| {
                assert_eq!(a.len(), 2);
                assert_eq!(a[1], 0x1234_5678);
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
            e.register(0x006c_61b0, |e, a| {
                assert_eq!(a[0], 0x011d_f1c4);
                assert_eq!(e.mem.u32(a[1]), 0x1234_5678);
                e.mem.set_u32(a[1], 0x8765_4321);
                Ret::default()
            });
            e.register(0x0045_cec0, |e, a| {
                assert_eq!(a.len(), 1);
                assert_eq!(e.mem.u32(a[0]), 0x8765_4321);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087_aad0, &args![0u32, 0x1234_5678u32]);
            let log = e.call_log.take().unwrap();
            let calls: Vec<_> = log.iter().map(|c| c.0).collect();
            if enabled_byte == 0 {
                assert_eq!(calls, [0x0087_aad0, 0x0040_8d60]);
            } else {
                assert_eq!(
                    calls,
                    [
                        0x0087_aad0,
                        0x0040_8d60,
                        0x0063_3c90,
                        0x006c_61b0,
                        0x0045_cec0
                    ]
                );
                assert_eq!(log[2].1[0], log[3].1[1]);
                assert_eq!(log[2].1[0], log[4].1[0]);
            }
        }
    }

    #[test]
    fn pair_insertion_reloads_indexed_element_and_respects_disabled_container() {
        for enabled_byte in [0u8, 0xff] {
            let mut e = Engine::new();
            let enabled = e.mem.alloc(1);
            let first = e.mem.alloc(8);
            let second = e.mem.alloc(8);
            e.mem.set_u8(enabled, enabled_byte);
            e.mem.set_u32(first + 4, 0xaabb_ccdd);
            e.mem.set_u32(second, 0x1122_3344);
            e.register_double(0x0040_8d60, move |_, a| {
                assert_eq!(a, &[0x011d_f214]);
                enabled.into_ret()
            });
            e.register(0x0084_5460, |_, a| {
                assert_eq!(a, &[0x011d_f244]);
                0xffff_fffeu32.into_ret()
            });
            let mut lookups = 0;
            e.register_double(0x0069_50d0, move |e, a| {
                assert_eq!(a, &[0x011d_f244, 0xffff_fffe]);
                lookups += 1;
                if lookups == 1 {
                    first.into_ret()
                } else {
                    assert_eq!(lookups, 2);
                    assert_eq!(e.mem.u32(first), 0x5566_7788);
                    second.into_ret()
                }
            });
            e.call_log = Some(vec![]);
            e.call(0x0087_ab50, &args![0u32, 0x5566_7788u32, 0x99aa_bbccu32]);
            let calls: Vec<_> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
            if enabled_byte == 0 {
                assert_eq!(calls, [0x0087_ab50, 0x0040_8d60]);
                assert_eq!(e.mem.u32(first), 0);
                assert_eq!(e.mem.u32(second + 4), 0);
            } else {
                assert_eq!(
                    calls,
                    [
                        0x0087_ab50,
                        0x0040_8d60,
                        0x0084_5460,
                        0x0069_50d0,
                        0x0069_50d0
                    ]
                );
                assert_eq!(e.mem.u32(first), 0x5566_7788);
                assert_eq!(e.mem.u32(second + 4), 0x99aa_bbcc);
            }
            assert_eq!(e.mem.u32(first + 4), 0xaabb_ccdd);
            assert_eq!(e.mem.u32(second), 0x1122_3344);
        }
    }

    #[test]
    fn embedded_member_helper_wraps_32_bit_address() {
        for object in [0x1234_5678u32, 0xffff_ffc0] {
            let mut e = Engine::new();
            e.register_double(0x0040_b460, move |_, a| {
                assert_eq!(a, &[object.wrapping_add(0x60)]);
                Ret::default()
            });
            e.call(0x0087_adf0, &args![object]);
        }
    }

    #[test]
    fn object_gate_checks_null_and_only_low_return_byte() {
        for result in [0u32, 0x100, 0xffff_ff00, 0x101, 0x80] {
            let mut e = Engine::new();
            let this: Ptr<TaskQueueInterface> = e.new_object();
            let object = virtual_object(&mut e, 0x220, 0x0f00_0030);
            e.register_double(0x0f00_0030, move |_, a| {
                assert_eq!(a, &[object.addr()]);
                result.into_ret()
            });
            capture_tasks(&mut e, this, move |e, a| {
                assert_eq!(a[0], this.addr());
                assert_eq!(e.mem.u32(a[1]), 0x0c);
                assert_eq!(e.mem.u32(a[1] + 4), object.addr());
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087_ae60, &args![this, 0u32]);
            assert_eq!(e.call_log.take().unwrap().len(), 1);
            e.call_log = Some(vec![]);
            e.call(0x0087_ae60, &args![this, object]);
            let log = e.call_log.take().unwrap();
            assert_eq!(log.len(), if result as u8 == 0 { 2 } else { 4 });
            assert_eq!(log[1].0, 0x0f00_0030);
            if result as u8 != 0 {
                assert_eq!(log[2].0, 0x0044ddc0);
                assert_eq!(log[3].0, CAPTURE_TASK);
            }
        }
    }

    #[test]
    fn target_actor_gate_reloads_both_getters_and_uses_full_virtual_result() {
        // None, actor without process, and nonzero process (including a
        // zero low byte) exercise every branch at 0087aef1 and 0087af17.
        for process in [None, Some(0u32), Some(0x100), Some(u32::MAX)] {
            let mut e = Engine::new();
            let this: Ptr<TaskQueueInterface> = e.new_object();
            let object = e.mem.alloc(4);
            let actor_first = e.mem.alloc(4);
            let actor_second = virtual_object(&mut e, 0x1d0, 0x0f00_0040);
            let mut target_lookups = 0;
            e.register_double(0x0059_bb30, move |_, a| {
                assert_eq!(a, &[object]);
                target_lookups += 1;
                (0x1000u32 + target_lookups).into_ret()
            });
            let mut actor_lookups = 0;
            e.register_double(0x0082_2b40, move |_, a| {
                actor_lookups += 1;
                assert_eq!(a, &[0x1000u32 + actor_lookups]);
                if process.is_none() {
                    0u32.into_ret()
                } else if actor_lookups == 1 {
                    actor_first.into_ret()
                } else {
                    assert_eq!(actor_lookups, 2);
                    actor_second.into_ret()
                }
            });
            e.register_double(0x0f00_0040, move |_, a| {
                assert_eq!(a, &[actor_second.addr()]);
                process.unwrap().into_ret()
            });
            capture_tasks(&mut e, this, move |e, a| {
                assert_eq!(a[0], this.addr());
                assert_eq!(e.mem.u32(a[1]), 0x12);
                assert_eq!(e.mem.u32(a[1] + 4), object);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087_aec0, &args![this, 0u32]);
            assert_eq!(e.call_log.take().unwrap().len(), 1);
            e.call_log = Some(vec![]);
            e.call(0x0087_aec0, &args![this, object]);
            let calls: Vec<_> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
            let mut expected = vec![0x0087_aec0, 0x0059_bb30, 0x0082_2b40];
            if process.is_some() {
                expected.extend([0x0059_bb30, 0x0082_2b40, 0x0f00_0040]);
            }
            if process != Some(0) {
                expected.extend([0x0044ddc0, CAPTURE_TASK]);
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn float_records_quiet_signaling_nan_but_word_fields_preserve_payloads() {
        check_record(
            0x0087_af50,
            &[0x1234_5678, 0x7f80_0001, 0x81],
            13,
            &[0x1234_5678, 0x7fc0_0001, 0x81],
            &[],
            Some(0),
            &[2],
        );
        check_record(
            0x0087_b090,
            &[
                0x1234_5678,
                0x7f80_1234,
                0xff80_2345,
                0x7fc0_3456,
                0x8000_0000,
            ],
            16,
            &[
                0x1234_5678,
                0x7fc0_1234,
                0xffc0_2345,
                0x7fc0_3456,
                0x8000_0000,
            ],
            &[],
            Some(0),
            &[],
        );
        for (address, tag) in [(0x0087_b4a0, 0x19), (0x0087_b4f0, 0x1a)] {
            check_record(
                address,
                &[0x7f80_0001, 0xff80_0002, 0xff80_0003],
                tag,
                &[0x7f80_0001, 0xff80_0002, 0xffc0_0003],
                &[],
                None,
                &[],
            );
        }
    }
    #[test]
    fn unconditional_retaining_producers_forward_null_inputs() {
        check_record(
            0x0087_abb0,
            &[0],
            6,
            &[0],
            &[(0x0040_f6e0, vec![0])],
            None,
            &[],
        );
        check_record(
            0x0087_ac00,
            &[0, 0, 0xff, 0x81],
            7,
            &[0, 0, 0xff, 0x81],
            &[(0x0040_f6e0, vec![0]), (0x0040_f6e0, vec![0])],
            None,
            &[2, 3],
        );
        check_record(
            0x0087_b300,
            &[0, 0x81, 0xff, 0x80],
            0x31,
            &[0, 0x81, 0xff, 0x80],
            &[(0x0040_f6e0, vec![0])],
            None,
            &[1, 2, 3],
        );
    }
    fn check_vector_producer(address: u32, tag: u32, float_value: bool) {
        let mut e = Engine::new();
        let this = e.new_object::<TaskQueueInterface>();
        let vector = e.mem.alloc(12);
        for (i, bits) in [0x7f80_1234, 0x8000_0000, 0xff80_1234]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(vector + i as u32 * 4, bits);
        }
        capture_tasks(&mut e, this, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), tag);
            assert_eq!(
                e.mem.u32(a[1] + 4),
                if float_value {
                    0x7fc0_2222
                } else {
                    0x7f80_2222
                }
            );
            assert_eq!(e.mem.u32(a[1] + 8), 0x7fc0_1234);
            assert_eq!(e.mem.u32(a[1] + 12), 0x8000_0000);
            assert_eq!(e.mem.u32(a[1] + 16), 0xffc0_1234);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(address, &args![this, 0x7f80_2222u32, vector]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }
    #[test]
    fn record_0087b650() {
        check_vector_producer(0x0087b650, 0x21, false);
    }
    #[test]
    fn record_0087b700() {
        check_vector_producer(0x0087b700, 0x24, true);
    }
    #[test]
    fn record_0087b760() {
        check_vector_producer(0x0087b760, 0x25, true);
    }
    #[test]
    fn record_0087b6b0() {
        check_record(
            0x0087b6b0,
            &[0, 0xffffff81],
            0x23,
            &[0, 0x81],
            &[],
            None,
            &[1],
        );
    }
    #[test]
    fn record_0087b7c0() {
        check_record(
            0x0087b7c0,
            &[0, 0xffffff80, 0xaabbcc01],
            0x26,
            &[0, 0x80, 1],
            &[],
            None,
            &[1, 2],
        );
    }
    #[test]
    fn record_0087b810() {
        check_record(0x0087b810, &[0], 0x27, &[0], &[], None, &[]);
    }
    #[test]
    fn record_0087b850() {
        check_record(0x0087b850, &[0], 0x28, &[0], &[], None, &[]);
    }
    #[test]
    fn record_0087b890() {
        check_record(0x0087b890, &[0], 0x2e, &[0], &[], None, &[]);
    }

    #[test]
    fn global_queue_creation_handles_allocation_failure() {
        for success in [false, true] {
            let mut e = Engine::new();
            let storage = e.mem.alloc(16);
            e.register_double(0x00401000, move |_, a| {
                assert_eq!(a, &[16]);
                (if success { storage } else { 0 }).into_ret()
            });
            e.map(0x011df1a8, 8);
            e.set_global(0x011df1a8, 0xdeadbeefu32);
            e.call(0x0087b8d0, &[0x1234, 0x5678]);
            assert_eq!(
                e.global::<u32>(0x011df1a8),
                if success { storage } else { 0 }
            );
            if success {
                assert_eq!(
                    (0..4)
                        .map(|i| e.mem.u32(storage + i * 4))
                        .collect::<Vec<_>>(),
                    [0x1234, 0x5678, 0x1234, 0]
                );
            }
        }
    }
    #[test]
    fn global_queue_destruction_clears_after_callback() {
        for object in [0u32, 0x12345678] {
            let mut e = Engine::new();
            e.map(0x011df1a8, 8);
            e.set_global(0x011df1a8, object);
            e.register_double(0x007b3fa0, move |e, a| {
                assert_eq!(a, &[object, 1]);
                assert_eq!(e.global::<u32>(0x011df1a8), object);
                e.map(0x011df1a8, 8);
                e.set_global(0x011df1a8, 0xfeedu32);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087b950, &[]);
            assert_eq!(
                e.call_log.take().unwrap().len(),
                if object == 0 { 1 } else { 2 }
            );
            assert_eq!(e.global::<u32>(0x011df1a8), 0);
        }
    }
    #[test]
    fn bound_value_preserves_st0_without_float_rounding() {
        let mut e = Engine::new();
        e.register(0x0043d450, |_, a| {
            assert_eq!(a, &[0x1234]);
            0x5678u32.into_ret()
        });
        e.register(0x0084d030, |_, a| {
            assert_eq!(a, &[0x5678]);
            1.0000000001f64.into_ret()
        });
        assert_eq!(e.call(0x0087ce50, &[0x1234]).f64(), 1.0000000001);
    }
    #[test]
    fn global_collection_address_needs_no_memory() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0087ce70, &[]).u32(), 0x011c4104);
    }
    #[test]
    fn member_setter_preserves_neighbor_bytes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x48);
        e.mem.write(object, &[0xa5; 0x48]);
        e.call(0x0087ce80, &[object, 0x12345678]);
        assert_eq!(e.mem.u32(object + 0x3c), 0xa5a5a5a5);
        assert_eq!(e.mem.u32(object + 0x40), 0x12345678);
        assert_eq!(e.mem.u32(object + 0x44), 0xa5a5a5a5);
    }
    #[test]
    fn release_helper_uses_full_word_zero_test() {
        for value in [0u32, 0x100, u32::MAX] {
            let mut e = Engine::new();
            e.register_double(0x004019a0, move |_, a| {
                assert_eq!(a, &[0x20]);
                value.into_ret()
            });
            e.register(0x00401030, |_, a| {
                assert_eq!(a, &[0xffffffc0]);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087cea0, &[0xffffffc0]);
            assert_eq!(
                e.call_log.take().unwrap().len(),
                if value == 0 { 3 } else { 2 }
            );
        }
    }
    #[test]
    fn controller_mask_normalizes_nonzero_word() {
        for value in [0u32, 0x100, u32::MAX] {
            let mut e = Engine::new();
            e.register_double(0x00621270, move |_, a| {
                assert_eq!(a, &[0x410, 0x80]);
                value.into_ret()
            });
            assert_eq!(e.call(0x0087cee0, &[0]).u32(), u32::from(value != 0));
        }
    }
    #[test]
    fn task_descriptor_keeps_order_and_clears_last_word() {
        let mut e = Engine::new();
        let storage = e.mem.alloc(24);
        e.mem.write(storage, &[0xcc; 24]);
        assert_eq!(
            e.call(0x0087cf10, &[storage + 4, 0x1234, 0x5678, 0xffffffff])
                .u32(),
            storage + 4
        );
        assert_eq!(
            (0..6)
                .map(|i| e.mem.u32(storage + i * 4))
                .collect::<Vec<_>>(),
            [0xcccccccc, 0x1234, 0x5678, 0xffffffff, 0, 0xcccccccc]
        );
    }
    #[test]
    fn node_callback_skips_null_and_forwards_constructed_update() {
        for present in [false, true] {
            let mut e = Engine::new();
            let node = virtual_object(&mut e, 0xc0, 0x0f002000);
            e.register(0x006a7ad0, |_, a| {
                assert_eq!(a, &[0x100, 7]);
                0x200u32.into_ret()
            });
            e.register_double(0x00559450, move |_, a| {
                assert_eq!(a, &[0x200]);
                (if present { node.addr() } else { 0 }).into_ret()
            });
            e.register(0x0043d410, |e, a| {
                assert_eq!(&a[1..], &[0, 0, 0]);
                e.mem.set_u32(a[0], 0xbeef);
                Ret::default()
            });
            e.register_double(0x0f002000, move |e, a| {
                assert_eq!(a[0], node.addr());
                assert_eq!(e.mem.u32(a[1]), 0xbeef);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087cf50, &[0x100, 7]);
            assert_eq!(
                e.call_log.take().unwrap().len(),
                if present { 5 } else { 3 }
            );
        }
    }
    #[test]
    fn actor_sync_callback_checks_gates_and_reloads_live_entries() {
        for stop in 0..6 {
            let mut e = Engine::new();
            let entry = e.mem.alloc(8);
            let actor = e.mem.alloc(0xb0);
            e.mem.set_u32(entry, if stop == 0 { 0 } else { actor });
            e.mem.set_u32(entry + 4, 0x200);
            e.mem
                .set_u32(actor + 0xac, if stop == 1 { 0 } else { 0x100 });
            e.register_double(0x00844160, move |_, a| {
                assert_eq!(a, &[0x10, 2]);
                entry.into_ret()
            });
            e.register_double(0x00552490, move |e, a| {
                assert_eq!(a, &[0x100]);
                e.mem.set_u32(entry + 4, 0x300);
                (if stop == 2 { 0x100u32 } else { 0x181 }).into_ret()
            });
            e.register_double(0x00552470, move |_, a| {
                assert_eq!(a, &[0x300]);
                (if stop == 3 { 1u32 } else { 0x100 }).into_ret()
            });
            e.register_double(0x00933840, move |e, a| {
                assert_eq!(a, &[actor]);
                e.mem.set_u32(entry + 4, 0x400);
                (if stop == 4 { 1u32 } else { 0x100 }).into_ret()
            });
            e.register(0x00c6c3d0, |_, a| {
                assert_eq!(a, &[0x400, 2]);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087cfb0, &[0x10, 2]);
            assert_eq!(e.call_log.take().unwrap().len(), [2, 2, 3, 4, 5, 6][stop]);
        }
    }
    #[test]
    fn expand_groups_retains_small_roots_large_children_and_cleans_temporaries() {
        let mut e = Engine::new();
        e.register(0x0044ddc0, |_, a| {
            assert_eq!(a, &[0x011df1c4]);
            2u32.into_ret()
        });
        e.register(0x006a7ad0, |_, a| {
            assert_eq!(a[0], 0x011df1c4);
            (0x100 + a[1]).into_ret()
        });
        e.register(0x00559450, |_, a| a[0].into_ret());
        e.register(0x0043b480, |_, a| {
            (if a[0] == 0x100 { 10u32 } else { 11 }).into_ret()
        });
        e.register(0x0043b4a0, |_, a| {
            assert_eq!(a[0], 0x101);
            (if a[1] == 3 { 0x200u32 } else { 0 }).into_ret()
        });
        e.register(0x00633c90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let values = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let observed = values.clone();
        e.register_double(0x006c61b0, move |e, a| {
            assert_eq!(a[0], 0x999);
            observed.borrow_mut().push(e.mem.u32(a[1]));
            e.mem.set_u32(a[1], 0xabc);
            Ret::default()
        });
        e.register(0x0045cec0, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0xabc);
            Ret::default()
        });
        e.register(0x006c6200, |_, a| {
            assert_eq!(a, &[0x011df1c4, 0]);
            Ret::default()
        });
        e.call(0x0087d030, &[0x999]);
        assert_eq!(*values.borrow(), [0x100, 0x200]);
    }
    #[test]
    fn queue_gate_enqueues_with_low_byte_failure_or_runs_synchronously() {
        for result in [0u32, 0x100, 0x181] {
            let mut e = Engine::new();
            let queue = virtual_object(&mut e, 4, 0x0f002004);
            e.map(0x011a31f4, 8);
            e.set_global(0x011a31f4, 0x80u8);
            e.register_double(0x0044ddc0, move |_, a| {
                assert_eq!(a, &[0x111]);
                queue.into_ret()
            });
            e.register_double(0x0f002004, move |_, a| {
                assert_eq!(a, &[queue.addr(), 0x222]);
                result.into_ret()
            });
            e.register(0x00fdf0c8, |_, a| {
                assert!(a.is_empty());
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087d160, &[0x111, 0x222]);
            assert_eq!(
                e.call_log.take().unwrap().len(),
                if result as u8 == 0 { 4 } else { 3 }
            );
        }
        let mut e = Engine::new();
        let task = e.mem.alloc(8);
        e.mem.set_u32(task, 3);
        e.mem.set_u32(task + 4, 0xbeef);
        e.map(0x011a31f4, 8);
        e.set_global(0x011a31f4, 0u8);
        e.register(0x00552390, |_, a| {
            assert_eq!(a, &[0xbeef]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0087d160, &[0, task]);
        assert_eq!(
            e.call_log
                .take()
                .unwrap()
                .iter()
                .map(|c| c.0)
                .collect::<Vec<_>>(),
            [0x0087d160, 0x00552390]
        );
    }
    #[test]
    fn scrap_constructor_installs_vtable_heap_and_empty_storage_in_order() {
        let mut e = Engine::new();
        let object = e.mem.alloc(20);
        e.register(0x00822580, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            Ret::default()
        });
        e.register_double(0x00401020, move |e, a| {
            assert!(a.is_empty());
            assert_eq!(e.mem.u32(object), 0x01084074);
            0x100u32.into_ret()
        });
        e.register(0x00aa42e0, |_, a| {
            assert_eq!(a, &[0x100]);
            0x200u32.into_ret()
        });
        e.register(0x00822860, |e, a| {
            assert_eq!(&a[1..], &[0, 0]);
            assert_eq!(e.mem.u32(a[0] + 16), 0x200);
            Ret::default()
        });
        assert_eq!(e.call(0x0087d1b0, &[object]).u32(), object);
    }
    fn install_scrap_destructor_doubles(e: &mut Engine) {
        e.register(0x006c6200, |e, a| {
            assert_eq!(a[1], 1);
            assert_eq!(e.mem.u32(a[0]), 0x01084074);
            e.mem.set_u32(a[0] + 4, 0xabc);
            Ret::default()
        });
        e.register(0x008225b0, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 4), 0xabc);
            Ret::default()
        });
        e.register(0x00401030, |_, _| Ret::default());
    }
    #[test]
    fn scrap_destructor_restores_table_before_storage_release() {
        let mut e = Engine::new();
        let object = e.mem.alloc(20);
        install_scrap_destructor_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0087d230, &[object]);
        assert_eq!(
            e.call_log
                .take()
                .unwrap()
                .iter()
                .map(|c| c.0)
                .collect::<Vec<_>>(),
            [0x0087d230, 0x006c6200, 0x008225b0]
        );
    }
    #[test]
    fn scrap_deleting_destructor_tests_only_flag_bit_zero() {
        for flags in [0u32, 2, 1, 3] {
            let mut e = Engine::new();
            let object = e.mem.alloc(20);
            install_scrap_destructor_doubles(&mut e);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0087d290, &[object, flags]).u32(), object);
            assert_eq!(e.call_log.take().unwrap().len(), 3 + (flags as usize & 1));
        }
    }
    #[test]
    fn actor_sync_array_constructor_sets_empty_shared_layout() {
        let mut e = Engine::new();
        let array = e.new_object::<BSSimpleArray>();
        e.mem.write(array.addr(), &[0xcc; 16]);
        assert_eq!(e.call(0x0087d2c0, &args![array]).u32(), array.addr());
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u32(array.addr() + i * 4))
                .collect::<Vec<_>>(),
            [0x010840d0, 0, 0, 0]
        );
    }
    #[test]
    fn actor_array_deleting_destructor_obeys_delete_bit() {
        for flags in [0u32, 2, 1] {
            let mut e = Engine::new();
            let array = e.mem.alloc(16);
            e.register(0x008454f0, |e, a| {
                assert_eq!(a[1], 1);
                assert_eq!(e.mem.u32(a[0]), 0x010840d0);
                Ret::default()
            });
            e.register(0x00401030, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0087d2f0, &[array, flags]).u32(), array);
            assert_eq!(e.call_log.take().unwrap().len(), 2 + (flags as usize & 1));
        }
    }
    #[test]
    fn actor_array_destructor_restores_vtable_and_releases_storage() {
        let mut e = Engine::new();
        let array = e.mem.alloc(16);
        e.register(0x008454f0, |e, a| {
            assert_eq!(a[1], 1);
            assert_eq!(e.mem.u32(a[0]), 0x010840d0);
            Ret::default()
        });
        e.call(0x0087d320, &[array]);
    }
    #[test]
    fn array_initializer_reserves_unsigned_max_and_constructs_exact_entries() {
        for (capacity, size) in [(0u32, 0u32), (4, 0), (1, 3), (5, 2), (u32::MAX, 1)] {
            let mut e = Engine::new();
            let array: Ptr = Ptr::new(e.mem.alloc(16));
            let table = e.mem.alloc(8);
            e.mem.set_u32(array.addr(), table);
            e.mem.set_u32(table + 4, 0x0f003000);
            e.register_double(0x0f003000, move |e, a| {
                assert_eq!(a, &[array.addr(), capacity.max(size)]);
                assert_eq!(e.mem.u32(a[0] + 4), 0);
                assert_eq!(e.mem.u32(a[0] + 8), 0);
                assert_eq!(e.mem.u32(a[0] + 12), 0);
                0x1234u32.into_ret()
            });
            e.register(0x006e6da0, |_, a| {
                assert_eq!(a[0], 8);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0087d340, &args![array, capacity, size]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                e.mem.u32(array.addr() + 4),
                if capacity.max(size) == 0 { 0 } else { 0x1234 }
            );
            assert_eq!(e.mem.u32(array.addr() + 8), size);
            assert_eq!(e.mem.u32(array.addr() + 12), capacity.max(size));
            assert_eq!(
                log.iter()
                    .filter(|c| c.0 == 0x006e6da0)
                    .map(|c| c.1.clone())
                    .collect::<Vec<_>>(),
                (0..size)
                    .map(|i| vec![8, 0x1234 + i * 8])
                    .collect::<Vec<_>>()
            );
        }
    }
    #[test]
    fn construct_entries_uses_eight_byte_stride_and_wraps_pointer() {
        let mut e = Engine::new();
        e.register(0x006e6da0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0087d3d0, &[0xabc, 0xfffffffc, 3]);
        assert_eq!(
            e.call_log.take().unwrap()[1..]
                .iter()
                .map(|c| c.1.clone())
                .collect::<Vec<_>>(),
            [vec![8, 0xfffffffc], vec![8, 4], vec![8, 12]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0087d3d0, &[0, 0, 0]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
    #[test]
    fn node_array_constructor_uses_its_own_vtable() {
        let mut e = Engine::new();
        let array = e.mem.alloc(16);
        e.register(0x00822860, |e, a| {
            assert_eq!(&a[1..], &[0, 0]);
            assert_eq!(e.mem.u32(a[0]), 0x010840e4);
            Ret::default()
        });
        assert_eq!(e.call(0x0087d420, &[array]).u32(), array);
    }
    #[test]
    fn node_array_deleting_destructor_calls_external_body_then_optional_free() {
        for flags in [0u32, 2, 1] {
            let mut e = Engine::new();
            e.register(0x0087d480, |_, a| {
                assert_eq!(a, &[0x1234]);
                Ret::default()
            });
            e.register(0x00401030, |_, a| {
                assert_eq!(a, &[0x1234]);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0087d450, &[0x1234, flags]).u32(), 0x1234);
            assert_eq!(e.call_log.take().unwrap().len(), 2 + (flags as usize & 1));
        }
    }

    fn dispatch_setup(tag: u32, payload: &[u32]) -> (Engine, u32) {
        let mut e = Engine::new();
        let task = e.mem.alloc(32);
        e.mem.set_u32(task, tag);
        for (i, value) in payload.iter().enumerate() {
            e.mem.set_u32(task + 4 + i as u32 * 4, *value);
        }
        e.call_log = Some(vec![]);
        (e, task)
    }
    fn recorded(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap().into_iter().skip(1).collect()
    }
    #[test]
    fn dispatch_direct_cases_keep_exact_arguments_and_release_order() {
        let cases = vec![
            (
                1,
                vec![0x1234],
                vec![(0x005b5e40, vec![0x0108405c, 0x1234])],
            ),
            (3, vec![0x1234], vec![(0x00552390, vec![0x1234])]),
            (
                6,
                vec![0x1234],
                vec![(0x00552790, vec![0x1234]), (0x00401970, vec![0x1234])],
            ),
            (
                15,
                vec![0x1234, 0xffffff80, 0xabcdef01],
                vec![(0x00899200, vec![0x1234, 0x80, 1])],
            ),
            (
                20,
                vec![0x1234],
                vec![
                    (0x00c6a270, vec![0x1234, 1, 1, 0]),
                    (0x00c69ee0, vec![0x1234, 1, 0]),
                    (0x00401970, vec![0x1234]),
                ],
            ),
            (
                21,
                vec![0x1234, 0x5678],
                vec![(0x00571760, vec![0x1234, 0x5678])],
            ),
            // The PC deliberately reuses slot +12 both as a word and as a byte.
            (
                24,
                vec![0x1234, 0x5678, 0xabcdef81, 0xffffff02],
                vec![(0x008b52a0, vec![0x1234, 0x5678, 0xabcdef81, 0x81, 2, 1, 0])],
            ),
            (29, vec![0x1234], vec![(0x008d3fa0, vec![0x1234])]),
            (
                30,
                vec![0x1234, 0xffffff80, 0xffffff02],
                vec![(0x0047aec0, vec![0x1234, 0x80, 2])],
            ),
            (
                32,
                vec![0x1234, 0x5678],
                vec![(0x00523150, vec![0x1234, 0x5678])],
            ),
            (
                34,
                vec![0x1234, 7],
                vec![
                    (0x00c6a350, vec![0x1234, 7, 1, 0, 1]),
                    (0x00401970, vec![0x1234]),
                ],
            ),
            (
                35,
                vec![0x1234, 0xffffff80],
                vec![(0x008a5f20, vec![0x1234, 0x80])],
            ),
            (
                38,
                vec![0x1234, 0xffffff80, 0xffffff02],
                vec![(0x008a7d50, vec![0x1234, 0x80, 2])],
            ),
            (39, vec![0x1234], vec![(0x008a1a70, vec![0x1234])]),
            (40, vec![0x1234], vec![(0x00887d00, vec![0x1234])]),
            (
                47,
                vec![0x1234],
                vec![(0x004ab0c0, vec![0x1234]), (0x00401970, vec![0x1234])],
            ),
            (
                48,
                vec![0x1234, 0xffffff80],
                vec![(0x00892460, vec![0x1234, 0x80])],
            ),
            (
                49,
                vec![0x1234, 0xffffff80, 0xffffff02, 0xabcdef00],
                vec![
                    (0x00c69fc0, vec![0x1234, 0x80, 2, 0]),
                    (0x00401970, vec![0x1234]),
                ],
            ),
        ];
        for (tag, payload, expected) in cases {
            let (mut e, task) = dispatch_setup(tag, &payload);
            for (address, _) in &expected {
                e.register(*address, |_, _| Ret::default());
            }
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e), expected, "tag {tag:x}");
        }
    }
    #[test]
    fn dispatch_null_gates_and_unused_tags_do_not_call_external_code() {
        for tag in [
            18u32,
            19,
            20,
            29,
            30,
            34,
            35,
            38,
            39,
            40,
            43,
            46,
            50,
            u32::MAX,
        ] {
            let (mut e, task) = dispatch_setup(tag, &[0; 7]);
            e.call(0x0087b990, &[task]);
            assert!(recorded(&mut e).is_empty(), "tag {tag:x}");
        }
    }
    #[test]
    fn dispatch_numeric_logging_and_constructed_vectors_preserve_abi() {
        let (mut e, task) = dispatch_setup(0, &[0x80000000]);
        e.register(0x005b5e40, |_, a| {
            assert_eq!(a, &[0x01084064, 0, 0x80000000]);
            Ret::default()
        });
        e.call(0x0087b990, &[task]);
        for tag in [2u32, 16, 33, 36, 37] {
            let (mut e, task) =
                dispatch_setup(tag, &[0, 0x7f800001, 0x80000000, 0xff800001, 0x7f800002]);
            e.map(0x011dea10, 8);
            e.set_global(0x011dea10, 0x555u32);
            e.register(0x00416870, |e, a| {
                assert_eq!(&a[1..], &[0x7fc00001, 0x80000000, 0xffc00001]);
                e.mem.set_u32(a[0], 11);
                e.mem.set_u32(a[0] + 4, 22);
                e.mem.set_u32(a[0] + 8, 33);
                Ret::default()
            });
            e.register(0x00b553d0, |e, a| {
                assert_eq!(a[0], 0);
                assert_eq!(e.mem.u32(a[1]), 11);
                Ret::default()
            });
            e.register(0x0062eb50, |_, a| {
                assert_eq!(a, &[0, 11, 22, 33]);
                Ret::default()
            });
            e.register(0x0070ec90, |_, a| {
                assert_eq!(a, &[0x555]);
                0x666u32.into_ret()
            });
            e.register(0x004e8510, |_, a| {
                assert_eq!(a, &[0x666, 11, 22, 33, 0]);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            let log = recorded(&mut e);
            assert_eq!(log[0].0, 0x00416870);
            assert_eq!(
                log.len(),
                match tag {
                    16 | 33 => 1,
                    37 => 3,
                    _ => 2,
                }
            );
        }
    }
    #[test]
    fn dispatch_attach_runs_both_virtual_calls_optional_properties_and_releases() {
        for properties in [0u32, 0x100, 0x180] {
            let (mut e, task) = dispatch_setup(7, &[]);
            let child = virtual_object(&mut e, 0xd0, 0x0f004000);
            let parent = virtual_object(&mut e, 0xdc, 0x0f004004);
            e.mem.set_u32(task + 4, child.addr());
            e.mem.set_u32(task + 8, parent.addr());
            e.mem.set_u32(task + 12, 0xffffff81);
            e.mem.set_u32(task + 16, properties);
            for address in [0x0f004000, 0x0f004004, 0x00a5a040, 0x00401970] {
                e.register(address, |_, _| Ret::default());
            }
            e.call(0x0087b990, &[task]);
            let mut expected = vec![
                (0x0f004004, vec![parent.addr(), child.addr(), 0x81]),
                (0x0f004000, vec![child.addr()]),
            ];
            if properties as u8 != 0 {
                expected.push((0x00a5a040, vec![child.addr()]));
            }
            expected.extend([
                (0x00401970, vec![child.addr()]),
                (0x00401970, vec![parent.addr()]),
            ]);
            assert_eq!(recorded(&mut e), expected);
        }
    }
    #[test]
    fn dispatch_detach_shadow_and_zero_update_paths_release_even_when_lookup_missing() {
        for present in [false, true] {
            let (mut e, task) = dispatch_setup(8, &[0x123]);
            let node = virtual_object(&mut e, 0xe8, 0x0f004008);
            e.register_double(0x009611e0, move |_, a| {
                assert_eq!(a, &[0x123]);
                (if present { node.addr() } else { 0 }).into_ret()
            });
            e.register_double(0x0f004008, move |_, a| {
                assert_eq!(a, &[node.addr(), 0x123]);
                Ret::default()
            });
            e.register(0x00401970, |_, a| {
                assert_eq!(a, &[0x123]);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e).len(), if present { 3 } else { 2 });
            let (mut e, task) = dispatch_setup(9, &[0x123, 0x456]);
            e.register_double(0x00450b80, move |_, a| {
                assert_eq!(a, &[0x456]);
                (if present { 0x789u32 } else { 0 }).into_ret()
            });
            e.register(0x00b5b1c0, |_, a| {
                assert_eq!(a, &[0x789, 0x123]);
                Ret::default()
            });
            e.register(0x00401970, |_, a| {
                assert_eq!(a, &[0x123]);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e).len(), if present { 3 } else { 2 });
        }
        let (mut e, task) = dispatch_setup(10, &[0x123]);
        e.register(0x0043d410, |e, a| {
            assert_eq!(&a[1..], &[0, 0, 0]);
            e.mem.set_u32(a[0], 0xabc);
            Ret::default()
        });
        e.register(0x00a59c60, |e, a| {
            assert_eq!(a[0], 0x123);
            assert_eq!(e.mem.u32(a[1]), 0xabc);
            Ret::default()
        });
        e.register(0x00401970, |_, a| {
            assert_eq!(a, &[0x123]);
            Ret::default()
        });
        e.call(0x0087b990, &[task]);
        assert_eq!(recorded(&mut e).len(), 3);
    }
    #[test]
    fn dispatch_hit_data_reloads_helper_mutations_and_frees_only_at_zero() {
        for associated in [false, true] {
            let (mut e, task) = dispatch_setup(11, &[0x123]);
            let data = e.mem.alloc(0x64);
            e.mem.set_u32(task + 8, data);
            e.mem.set_u32(data + 0x14, 0x7f800001);
            e.register_double(0x0088e8d0, move |e, a| {
                assert_eq!(a, &[0x123, 0x7fc00001, data]);
                e.mem.set_u32(data, if associated { 0x456 } else { 0 });
                e.mem.set_u32(data + 0x14, 0xff800002);
                Ret::default()
            });
            e.register_double(0x0088fb00, move |_, a| {
                assert_eq!(a, &[0x456, 0xffc00002, data, 0x123]);
                Ret::default()
            });
            e.register_double(0x004019a0, move |_, a| {
                assert_eq!(a, &[data + 0x60]);
                0x100u32.into_ret()
            });
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e).len(), if associated { 3 } else { 2 });
        }
        for actor in [0u32, 0x123] {
            let (mut e, task) = dispatch_setup(17, &[actor, 0x456]);
            e.register(0x0089a760, |_, a| {
                assert_eq!(a, &[0x123, 0x456, 0]);
                Ret::default()
            });
            e.register(0x004019a0, |_, a| {
                assert_eq!(a, &[0x4b6]);
                0u32.into_ret()
            });
            e.register(0x00401030, |_, a| {
                assert_eq!(a, &[0x456]);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e).len(), if actor == 0 { 2 } else { 3 });
        }
    }
    #[test]
    fn dispatch_membership_fallthrough_uses_iterator_outputs_and_low_byte_type() {
        for (tag, found, actor) in [
            (12u32, true, true),
            (12, true, false),
            (13, true, true),
            (13, false, false),
        ] {
            let (mut e, task) = dispatch_setup(tag, &[]);
            let object = e.mem.alloc(4);
            let table = e.mem.alloc(0x33c);
            e.mem.set_u32(object, table);
            for (slot, address) in [
                (0x220, 0x0f005000),
                (0x304, 0x0f005004),
                (0x100, 0x0f005008),
                (0x338, 0x0f00500c),
                (0x144, 0x0f005010),
            ] {
                e.mem.set_u32(table + slot, address);
            }
            e.mem.set_u32(task + 4, object);
            e.mem.set_u32(task + 8, 0x7f800001);
            e.mem.set_u32(task + 12, 0xffffff81);
            e.register(0x0f005000, |_, _| 0x180u32.into_ret());
            e.register(0x0f005004, |_, _| Ret::default());
            e.register_double(0x0f005008, move |_, _| {
                (if actor { 0x181u32 } else { 0x100 }).into_ret()
            });
            e.register_double(0x0f00500c, move |_, a| {
                assert_eq!(a, &[object, 0x7fc00001, 0, 0]);
                Ret::default()
            });
            e.register_double(0x0f005010, move |_, a| {
                assert_eq!(a, &[object, 0x7fc00001, 0x81]);
                Ret::default()
            });
            e.register(0x004b9ba0, |_, a| {
                assert_eq!(a, &[0x011c4104]);
                1u32.into_ret()
            });
            let mut calls = 0;
            e.register_double(0x006b7f20, move |e, a| {
                assert_eq!(a[0], 0x011c4104);
                assert_eq!(e.mem.u32(a[2]), 0);
                calls += 1;
                e.mem.set_u32(a[1], if calls == 1 { 2 } else { 0 });
                e.mem.set_u32(
                    a[2],
                    if found && calls == 2 {
                        object
                    } else {
                        0xabcdef
                    },
                );
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            let log = recorded(&mut e);
            assert_eq!(log.iter().filter(|c| c.0 == 0x006b7f20).count(), 2);
            assert_eq!(
                log.iter().filter(|c| c.0 == 0x0f005004).count(),
                usize::from(tag == 12)
            );
            assert_eq!(
                log.iter().filter(|c| c.0 == 0x0f005008).count(),
                usize::from(found)
            );
        }
    }
    #[test]
    fn dispatch_kill_reads_dismember_after_callback_and_frees_both_payloads() {
        for dismember in [false, true] {
            let (mut e, task) = dispatch_setup(14, &[]);
            let actor = e.mem.alloc(0xc4);
            let value = e.mem.alloc(4);
            let limbs = e.mem.alloc(12);
            e.mem.set_u32(task + 4, actor);
            e.mem.set_u32(task + 8, value);
            e.mem.set_u32(actor + 0xc0, 0x456);
            e.mem.set_u32(value, 0x7f800001);
            for (i, v) in [11, 22, 33].into_iter().enumerate() {
                e.mem.set_u32(limbs + i as u32 * 4, v);
            }
            e.register_double(0x0089d900, move |e, a| {
                assert_eq!(a, &[actor, 0x456, 0x7fc00001]);
                e.mem.set_u32(task + 12, if dismember { limbs } else { 0 });
                Ret::default()
            });
            e.register_double(0x008b51b0, move |_, a| {
                assert_eq!(a, &[actor, 22, 11, 33]);
                Ret::default()
            });
            e.register(0x00401030, |_, _| Ret::default());
            e.call(0x0087b990, &[task]);
            let log = recorded(&mut e);
            assert_eq!(log.last().unwrap().1, [value]);
            assert_eq!(log.len(), if dismember { 4 } else { 2 });
        }
    }
    #[test]
    fn dispatch_instantiation_stores_return_and_reference_lookup_honors_null_chain() {
        let (mut e, task) = dispatch_setup(18, &[]);
        let object = e.mem.alloc(0x44);
        e.mem.set_u32(task + 4, object);
        e.register_double(0x0081b8e0, move |_, a| {
            assert_eq!(a, &[object]);
            0xabcdefu32.into_ret()
        });
        e.call(0x0087b990, &[task]);
        assert_eq!(e.mem.u32(object + 0x40), 0xabcdef);
        for stop in 0..4 {
            let (mut e, task) = dispatch_setup(19, &[0x123]);
            let node = virtual_object(&mut e, 0xd0, 0x0f005020);
            e.register_double(0x0056f930, move |_, a| {
                assert_eq!(a, &[0x123]);
                (if stop == 0 { 0u32 } else { 0x234 }).into_ret()
            });
            e.register_double(0x008d6f30, move |_, a| {
                assert_eq!(a, &[0x234]);
                (if stop == 1 { 0u32 } else { 0x345 }).into_ret()
            });
            e.register_double(0x004543c0, move |_, a| {
                assert_eq!(a, &[0x345]);
                (if stop == 2 { 0 } else { node.addr() }).into_ret()
            });
            e.register_double(0x0f005020, move |_, a| {
                assert_eq!(a, &[node.addr(), 0x123, 1, 1, 0, 0]);
                Ret::default()
            });
            e.register(0x00401970, |_, a| {
                assert_eq!(a, &[0x123]);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e).len(), stop + 2);
        }
    }
    #[test]
    fn dispatch_water_and_door_getters_do_not_steal_pending_arguments() {
        for tag in [25u32, 26, 27, 28] {
            let (mut e, task) = dispatch_setup(tag, &[0x123, 0x456, 0x7f800001]);
            e.map(0x011dea10, 8);
            e.set_global(0x011dea10, 0x111u32);
            e.register(0x0070ec90, |_, a| {
                assert_eq!(a, &[0x111]);
                0x222u32.into_ret()
            });
            e.register(0x006c0720, |_, a| {
                assert!(a.is_empty());
                0x333u32.into_ret()
            });
            for address in [0x004e5c80, 0x004e5df0] {
                e.register(address, |_, a| {
                    assert_eq!(a, &[0x222, 0x123, 0x456, 0x7fc00001]);
                    Ret::default()
                });
            }
            for address in [0x006c0dc0, 0x006c0f10] {
                e.register(address, |_, a| {
                    assert_eq!(a, &[0x333, 0x123]);
                    Ret::default()
                });
            }
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e).len(), 2);
        }
    }
    #[test]
    fn dispatch_virtual_actions_pass_exact_flags() {
        for (tag, slot, args) in [
            (23u32, 0x1cc, vec![0, 0]),
            (41, 0x1fc, vec![0]),
            (45, 0x1c4, vec![]),
            (46, 0, vec![1]),
        ] {
            let (mut e, task) = dispatch_setup(tag, &[]);
            let object = virtual_object(&mut e, slot, 0x0f005030);
            e.mem.set_u32(task + 4, object.addr());
            let mut expected = vec![object.addr()];
            expected.extend(args);
            e.register_double(0x0f005030, move |_, a| {
                assert_eq!(a, expected);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e).len(), 1);
        }
    }
    #[test]
    fn dispatch_resolved_virtual_gate_uses_low_byte() {
        for returned in [0u32, 0x100, 0x181] {
            let (mut e, task) = dispatch_setup(42, &[0x111]);
            let object = e.mem.alloc(4);
            let table = e.mem.alloc(0x1c4);
            e.mem.set_u32(object, table);
            e.mem.set_u32(table + 0xf0, 0x0f005040);
            e.mem.set_u32(table + 0x1c0, 0x0f005044);
            e.register_double(0x004839c0, move |_, a| {
                assert_eq!(a, &[0x111]);
                object.into_ret()
            });
            e.register_double(0x0f005040, move |_, _| returned.into_ret());
            e.register(0x0f005044, |_, _| Ret::default());
            e.call(0x0087b990, &[task]);
            assert_eq!(
                recorded(&mut e).len(),
                if returned as u8 == 0 { 2 } else { 3 }
            );
        }
    }
    #[test]
    fn dispatch_controller_keeps_update_order_and_ignores_mask_result() {
        for present in [false, true] {
            let (mut e, task) = dispatch_setup(44, &[]);
            let actor = virtual_object(&mut e, 0x1f4, 0x0f005050);
            e.mem.set_u32(task + 4, actor.addr());
            e.register_double(0x009306d0, move |_, a| {
                assert_eq!(a, &[actor.addr()]);
                (if present { 0x123u32 } else { 0 }).into_ret()
            });
            e.register(0x0f005050, |_, _| 0x234u32.into_ret());
            e.register(0x005620e0, |_, a| {
                assert_eq!(a, &[0x123, 0x234]);
                Ret::default()
            });
            e.register(0x00458b20, |_, a| {
                assert!(a.is_empty());
                0x3f800000u32.into_ret()
            });
            e.register(0x0080fb00, |_, a| {
                assert_eq!(a, &[0x123, 0x3f800000]);
                Ret::default()
            });
            e.register(0x005d10f0, |_, a| {
                assert_eq!(a, &[0x123]);
                1.0f32.into_ret()
            });
            e.register(0x00621270, |_, a| {
                assert_eq!(a, &[0x533, 0x80]);
                0u32.into_ret()
            });
            e.register(0x00c6e030, |_, a| {
                assert_eq!(a, &[0x123]);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            assert_eq!(
                recorded(&mut e).iter().map(|c| c.0).collect::<Vec<_>>(),
                if present {
                    vec![
                        0x009306d0, 0x0f005050, 0x005620e0, 0x00458b20, 0x0080fb00, 0x005d10f0,
                        0x00621270, 0x00c6e030,
                    ]
                } else {
                    vec![0x009306d0]
                }
            );
        }
    }

    #[test]
    fn dispatch_bound_transition_refreshes_cell_only_from_zero_to_positive() {
        for (old, new) in [
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 2.0),
            (f64::NAN, 2.0),
            (0.0, f64::NAN),
        ] {
            let (mut e, task) = dispatch_setup(5, &[0x111]);
            e.map(0x01012060, 8);
            e.set_global(0x01012060, 0.0f64);
            e.map(0x011dea10, 8);
            e.set_global(0x011dea10, 0x222u32);
            e.register(0x0043d450, |_, a| {
                assert_eq!(a, &[0x111]);
                0x333u32.into_ret()
            });
            let mut calls = 0;
            e.register_double(0x0084d030, move |_, a| {
                assert_eq!(a, &[0x333]);
                calls += 1;
                (if calls == 1 { old } else { new }).into_ret()
            });
            e.register(0x005526f0, |_, a| {
                assert_eq!(a, &[0x111]);
                Ret::default()
            });
            e.register(0x0056f930, |_, a| {
                assert_eq!(a, &[0x111]);
                0x444u32.into_ret()
            });
            e.register(0x00457070, |_, a| {
                assert_eq!(a, &[0x222]);
                0x555u32.into_ret()
            });
            e.register(0x0054a070, |_, a| {
                assert_eq!(a, &[0x555, 0x444, 0, 0]);
                Ret::default()
            });
            e.register(0x00401970, |_, a| {
                assert_eq!(a, &[0x111]);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            let log = recorded(&mut e);
            assert_eq!(
                log.iter().filter(|c| c.0 == 0x0054a070).count(),
                usize::from(old == 0.0 && new > 0.0)
            );
            assert_eq!(log.last().unwrap().0, 0x00401970);
        }
    }
    #[test]
    fn dispatch_vector_virtuals_use_constructor_output_and_repeated_process_getter() {
        for tag in [16u32, 33] {
            let (mut e, task) = dispatch_setup(tag, &[]);
            let actor = virtual_object(&mut e, 0x2a8, 0x0f006000);
            let process = virtual_object(&mut e, 0x418, 0x0f006004);
            e.mem.set_u32(task + 4, actor.addr());
            e.mem.set_u32(task + 20, 0x7f800001);
            e.register(0x00416870, |e, a| {
                assert_eq!(&a[1..], &[0, 0, 0]);
                e.mem.set_u32(a[0], 11);
                e.mem.set_u32(a[0] + 4, 22);
                e.mem.set_u32(a[0] + 8, 33);
                Ret::default()
            });
            let mut calls = 0;
            e.register_double(0x008d8520, move |_, a| {
                assert_eq!(a, &[actor.addr()]);
                calls += 1;
                (if calls == 1 { 0x1234 } else { process.addr() }).into_ret()
            });
            e.register_double(0x0f006004, move |_, a| {
                assert_eq!(a, &[process.addr(), actor.addr(), 11, 22, 33, 0x7fc00001]);
                Ret::default()
            });
            e.register_double(0x0f006000, move |e, a| {
                assert_eq!(a[0], actor.addr());
                assert_eq!(
                    (0..3).map(|i| e.mem.u32(a[1] + i * 4)).collect::<Vec<_>>(),
                    [11, 22, 33]
                );
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e).len(), if tag == 16 { 4 } else { 2 });
        }
    }
    #[test]
    fn dispatch_weapon_removal_reloads_process_and_equipment_at_every_step() {
        for stop in 0..6 {
            let (mut e, task) = dispatch_setup(22, &[0x111, 0x222]);
            let process = e.mem.alloc(4);
            let table = e.mem.alloc(0x164);
            e.mem.set_u32(process, table);
            e.mem.set_u32(table + 0x148, 0x0f006010);
            e.mem.set_u32(table + 0x160, 0x0f006014);
            let mut process_calls = 0;
            e.register_double(0x008d8520, move |_, a| {
                assert_eq!(a, &[0x111]);
                process_calls += 1;
                (if (stop == 0 && process_calls == 1) || (stop == 3 && process_calls == 4) {
                    0
                } else {
                    process
                })
                .into_ret()
            });
            let mut equip_calls = 0;
            e.register_double(0x0f006010, move |_, a| {
                assert_eq!(a, &[process]);
                equip_calls += 1;
                (if stop == 1 {
                    0u32
                } else if equip_calls == 1 {
                    0x123
                } else {
                    0x456
                })
                .into_ret()
            });
            e.register_double(0x0044ddc0, move |_, a| {
                assert_eq!(a, &[0x456]);
                (if stop == 2 { 0x999u32 } else { 0x222 }).into_ret()
            });
            e.register(0x00571b50, |_, a| {
                assert_eq!(a, &[0x111]);
                Ret::default()
            });
            e.register_double(0x0f006014, move |_, a| {
                assert_eq!(a, &[process, 0, 0, 0]);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            let log = recorded(&mut e);
            assert_eq!(
                log.iter().filter(|c| c.0 == 0x00571b50).count(),
                usize::from(stop >= 3)
            );
            assert_eq!(
                log.iter().filter(|c| c.0 == 0x0f006014).count(),
                usize::from(stop >= 4)
            );
            if stop >= 4 {
                assert_eq!(log.iter().filter(|c| c.0 == 0x008d8520).count(), 5);
            }
        }
    }
    #[test]
    fn dispatch_script_commands_resolve_ids_and_preserve_word_and_float_arguments() {
        for (command, expected) in [
            (0x1007u32, vec![0x005c0740, 0x444, 0x81, 0x7fc00002]),
            (0x1009, vec![0x005c0b10, 0x444, 0x81, 0x7fc00002]),
            (0x113c, vec![0x005c1c10, 0x444, 0x7fc00081]),
            (0x1156, vec![0x005d3c20, 0x444, 0x7fc00081]),
            (0x116c, vec![0x005d4c20, 0x444, 0x7fc00081]),
            (0x11df, vec![0x005d9bc0, 0x444, 0x555, 0x7f800002]),
        ] {
            let (mut e, task) = dispatch_setup(31, &[command, 0x111, 0x7f800081, 0x7f800002]);
            e.register(0x004839c0, |_, a| match a {
                [0x111] => 0x444u32.into_ret(),
                [0x7f800081] => 0x555u32.into_ret(),
                _ => panic!("unexpected resolver input"),
            });
            e.register(expected[0], |_, _| Ret::default());
            e.call(0x0087b990, &[task]);
            let log = recorded(&mut e);
            assert_eq!(log.last().unwrap(), &(expected[0], expected[1..].to_vec()));
        }
        for (command, resolved) in [(0xffffffffu32, 0x444u32), (0x1007, 0)] {
            let (mut e, task) = dispatch_setup(31, &[command, 0x111]);
            e.register_double(0x004839c0, move |_, _| resolved.into_ret());
            e.call(0x0087b990, &[task]);
            assert_eq!(recorded(&mut e).len(), 1);
        }
    }
    #[test]
    fn dispatch_script_move_fades_only_for_matching_actor_and_ordered_positive_tests() {
        for (command, stop) in [
            (0x109eu32, 9),
            (0x124f, 0),
            (0x124f, 1),
            (0x124f, 2),
            (0x124f, 3),
            (0x124f, 4),
            (0x124f, 5),
            (0x124f, 9),
        ] {
            let (mut e, task) = dispatch_setup(
                31,
                &[command, 0x111, 0x222, 0x7f800001, 0x80000000, 0xff800002],
            );
            let object = e.mem.alloc(4);
            let table = e.mem.alloc(0x424);
            e.mem.set_u32(object, table);
            e.mem.set_u32(table + 0x100, 0x0f006020);
            e.mem.set_u32(table + 0x420, 0x0f006024);
            e.map(0x01012060, 8);
            e.set_global(0x01012060, 0.0f64);
            e.map(0x011dea3c, 8);
            e.set_global(0x011dea3c, if stop == 1 { object } else { 0u32 });
            e.register_double(0x004839c0, move |_, a| match a {
                [0x111] => object.into_ret(),
                [0x222] => 0x333u32.into_ret(),
                _ => panic!("unexpected resolver input"),
            });
            e.register_double(0x0f006020, move |_, _| {
                (if stop == 0 { 0x100u32 } else { 0x181 }).into_ret()
            });
            let mut calls = 0;
            e.register_double(0x008d8520, move |_, a| {
                assert_eq!(a, &[object]);
                calls += 1;
                (if stop == 2 {
                    0u32
                } else if calls == 1 {
                    0x444
                } else {
                    0x555
                })
                .into_ret()
            });
            e.register_double(0x00931850, move |_, _| {
                (if stop == 3 { 1u32 } else { 0 }).into_ret()
            });
            e.register_double(0x0f006024, move |_, _| {
                (if stop == 4 {
                    0.0f64
                } else if stop == 5 {
                    f64::NAN
                } else {
                    1.0
                })
                .into_ret()
            });
            e.register_double(0x005ccb20, move |_, a| {
                assert_eq!(a, &[object, 0x333, 0x7fc00001, 0x80000000, 0xffc00002]);
                Ret::default()
            });
            e.register_double(0x008fe9e0, move |_, a| {
                assert_eq!(
                    a,
                    &[0x555, object, 0x333, 0x7fc00001, 0x80000000, 0xffc00002]
                );
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            let log = recorded(&mut e);
            assert_eq!(
                log.last().unwrap().0,
                if command == 0x124f && stop == 9 {
                    0x008fe9e0
                } else {
                    0x005ccb20
                }
            );
        }
    }
    #[test]
    fn dispatch_parallel_batches_build_descriptors_and_destroy_scrap_storage() {
        for (actor_count, node_count, enabled) in
            [(0u32, 0u32, 0u8), (2, 0, 0x80), (0, 1, 0x80), (2, 1, 0)]
        {
            let (mut e, task) = dispatch_setup(4, &[]);
            e.map(0x01202df4, 8);
            e.set_global(0x01202df4, 0x111u32);
            e.map(0x011dea10, 8);
            e.set_global(0x011dea10, 0x222u32);
            let gate = e.mem.alloc(1);
            e.mem.set_u8(gate, enabled);
            e.register_double(0x0044ddc0, move |_, a| match a {
                [0x011df244] => actor_count.into_ret(),
                [0x011df1c4] => node_count.into_ret(),
                _ => 3u32.into_ret(),
            });
            e.register(0x008454f0, |_, a| {
                assert_eq!(a, &[0x011df244, 0]);
                Ret::default()
            });
            e.register(0x00822580, |_, _| Ret::default());
            e.register(0x00401020, |_, a| {
                assert!(a.is_empty());
                0x333u32.into_ret()
            });
            e.register(0x00aa42e0, |_, a| {
                assert_eq!(a, &[0x333]);
                0x444u32.into_ret()
            });
            e.register(0x00822860, |_, a| {
                assert_eq!(&a[1..], &[0, 0]);
                Ret::default()
            });
            e.register(0x006a7ad0, |_, a| {
                assert_eq!(a, &[0x011df1c4, 0]);
                0x555u32.into_ret()
            });
            e.register(0x00559450, |_, a| {
                assert_eq!(a, &[0x555]);
                0x666u32.into_ret()
            });
            e.register(0x0043b480, |_, a| {
                assert_eq!(a, &[0x666]);
                0u32.into_ret()
            });
            e.register(0x00633c90, |e, a| {
                assert_eq!(a[1], 0x666);
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
            e.register(0x006c61b0, |e, a| {
                assert_eq!(e.mem.u32(a[1]), 0x666);
                Ret::default()
            });
            e.register(0x0045cec0, |_, _| Ret::default());
            e.register(0x006c6200, |e, a| {
                if a[0] == 0x011df1c4 {
                    assert_eq!(a[1], 0);
                } else {
                    assert_eq!(a[1], 1);
                    assert_eq!(e.mem.u32(a[0]), 0x01084074);
                }
                Ret::default()
            });
            e.register(0x008225b0, |_, _| Ret::default());
            e.register_double(0x00c44c90, move |e, a| {
                assert_eq!(a[0], 0x111);
                let descriptor = a[1];
                let callback = e.mem.u32(descriptor);
                assert_eq!(e.mem.u32(descriptor + 12), 0);
                if callback == 0x0087cfb0 {
                    assert_eq!(e.mem.u32(descriptor + 4), 0x011df244);
                    assert_eq!(e.mem.u32(descriptor + 8), actor_count);
                } else {
                    assert_eq!(callback, 0x0087cf50);
                    let array = e.mem.u32(descriptor + 4);
                    assert_eq!(e.mem.u32(array), 0x01084074);
                    assert_eq!(e.mem.u32(array + 16), 0x444);
                    assert_eq!(e.mem.u32(descriptor + 8), 3);
                }
                Ret::default()
            });
            e.register_double(0x00408d60, move |_, a| {
                assert_eq!(a, &[0x011df1b8]);
                gate.into_ret()
            });
            e.register(0x00453900, |_, a| {
                assert_eq!(a, &[0x222]);
                Ret::default()
            });
            e.call(0x0087b990, &[task]);
            let log = recorded(&mut e);
            assert_eq!(
                log.iter().filter(|c| c.0 == 0x00c44c90).count(),
                usize::from(actor_count != 0) + usize::from(node_count != 0)
            );
            assert_eq!(
                log.iter().filter(|c| c.0 == 0x008225b0).count(),
                usize::from(node_count != 0)
            );
            assert_eq!(
                log.iter().filter(|c| c.0 == 0x00453900).count(),
                usize::from(enabled == 0)
            );
        }
    }
}
