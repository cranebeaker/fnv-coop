//! `NiAnimation/nigeommorpherupdatetask.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeomMorpherUpdateTask::ms_RTTI` (the class's `NiRTTI` record).
const NI_GEOM_MORPHER_UPDATE_TASK_RTTI: u32 = 0x011f_4100;

// Translated from 00a56c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherUpdateTask::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_geom_morpher_update_task_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_GEOM_MORPHER_UPDATE_TASK_RTTI)
}

/// The memory manager singleton (`thiscall` object of `00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `00aa4060` (`thiscall` on the memory manager, one stack word): deallocate.
const DEALLOCATE: u32 = 0x00aa_4060;
/// `00e7f900` (`thiscall`, one stack word; unit `NiXenonRenderer/nid3dpass.cpp`).
const D3D_PASS_RELEASE: u32 = 0x00e7_f900;
/// The globals `00a56df0` works on: a word it clears, the block pointer it tears down (not
/// cleared), and the word copied from `011a9420` to `011a941c`.
const CLEARED_WORD: u32 = 0x011a_9414;
const TASK_BLOCK: u32 = 0x011a_9418;
const COPY_DESTINATION: u32 = 0x011a_941c;
const COPY_SOURCE: u32 = 0x011a_9420;

// Translated from 00a56df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeommorpherupdatetask.cpp`, no arguments): clears
/// `011a9414`; when `011a9418` points to a block, that block's first word is disposed of
/// (when the word in front of it is 0 the block from that word on goes back to the memory
/// manager, otherwise its vtable slot 0 runs with 3), `00e7f900(1)` runs on its third word
/// when that is set, and the block goes back to the memory manager (the pointer is not
/// cleared). Last, copies `011a9420` to `011a941c`.
pub fn fn_00a56df0(e: &mut Engine) {
    let block = e.mem.u32(TASK_BLOCK);
    e.mem.set_u32(CLEARED_WORD, 0);
    if block != 0 {
        let first = e.mem.u32(block);
        if first != 0 {
            let header = first.wrapping_sub(4);
            if e.mem.u32(header) == 0 {
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, header]);
            } else {
                e.vcall(first, 0, &args![3u32]);
            }
        }
        let third = e.mem.u32(block + 8);
        if third != 0 {
            e.call(D3D_PASS_RELEASE, &args![third, 1u32]);
        }
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    let value = e.mem.u32(COPY_SOURCE);
    e.mem.set_u32(COPY_DESTINATION, value);
}

/// `NiGeomMorpherUpdateTask`'s vtable (RTTI `.?AVNiGeomMorpherUpdateTask@@`, the address the
/// constructor stores), the base vtable its destructor stores (`01069674`) and the size.
const NI_GEOM_MORPHER_UPDATE_TASK_VTABLE: u32 = 0x0109_a864;
const BASE_VTABLE: u32 = 0x0106_9674;
const NI_GEOM_MORPHER_UPDATE_TASK_SIZE: u32 = 0x14;
/// Fields (Xbox PDB names, PC offsets): `NiTask::m_eStatus` +0x08, `m_spMorphController`
/// +0x0c and `m_spTarget` +0x10.
const STATUS: u32 = 0x08;
const MORPH_CONTROLLER: u32 = 0x0c;
const MORPH_TARGET: u32 = 0x10;
/// The byte `DoTask` tests (`011f5b08`), the pool's critical section (`011f4180`) and the
/// object pool (`011a940c`) `Clear` hands the task back to (`00e7ec00`, thiscall, the task's
/// address).
const DO_TASK_SKIP_PRE_DISPLAY: u32 = 0x011f_5b08;
const POOL_CRITICAL_SECTION: u32 = 0x011f_4180;
const TASK_POOL: u32 = 0x011a_940c;
const POOL_RELEASE: u32 = 0x00e7_ec00;
/// `EnterCriticalSection` / `LeaveCriticalSection` and `InterlockedIncrement` /
/// `InterlockedDecrement` (import slots).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiObject`'s constructor (`00a5d3a0`) and destructor (`00a5d3d0`); `NiTSet::Realloc`
/// (`00a311e0`, thiscall, the new capacity); the allocations: the memory manager's
/// `Allocate` (`00aa3e40`), `00aa1420` (cdecl, size), the sized and array deletes
/// (`00aa1460`, `00aa1490`, cdecl `(block, size)`); the vector constructor iterator
/// (`00401050`, stdcall `(array, size, count, constructor)`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const SET_REALLOC: u32 = 0x00a3_11e0;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const NI_ALLOCATE_ARRAY: u32 = 0x00aa_1420;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
const NI_ARRAY_DELETE: u32 = 0x00aa_1490;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// The task constructor (`00a56be0`, in this file), the address handed to the vector
/// constructor iterator.
const TASK_CONSTRUCT: u32 = 0x00a5_6be0;
/// Virtual slots: `DeleteThis` (`0x04`); the morph controller's `OnPreDisplay` (`0xa8`); the
/// task's `Clear` (`0x94`).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_ON_PRE_DISPLAY: u32 = 0xa8;
const SLOT_CLEAR: u32 = 0x94;

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at +4 and, when it
/// reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// The inline `NiPointer::operator=` for the pointer at `slot`.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
        }
    }
}

// Translated from 00a56bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherUpdateTask::DoTask` (Xbox PDB): unless the byte at `011f5b08` is set, the
/// morph controller's `OnPreDisplay` (slot `0xa8`) runs when the target's reference count is
/// above 1; then a jump to the task's `Clear` (slot `0x94`).
pub fn ni_geom_morpher_update_task_do_task(e: &mut Engine, this: Ptr) {
    let task = this.addr();
    if e.mem.u8(DO_TASK_SKIP_PRE_DISPLAY) == 0 {
        let target = e.mem.u32(task.wrapping_add(MORPH_TARGET));
        if e.mem.u32(target.wrapping_add(4)) > 1 {
            let controller = e.mem.u32(task.wrapping_add(MORPH_CONTROLLER));
            e.vcall(controller, SLOT_ON_PRE_DISPLAY, &args![]);
        }
    }
    e.vcall(task, SLOT_CLEAR, &args![]);
}

// Translated from 00a56be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiGeomMorpherUpdateTask` constructor: `NiObject`'s, status
/// 0, the vtable, no morph controller and no target. Returns `this`.
pub fn fn_00a56be0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    let task = this.addr();
    e.mem.set_u32(task.wrapping_add(STATUS), 0);
    e.mem.set_u32(task, NI_GEOM_MORPHER_UPDATE_TASK_VTABLE);
    e.mem.set_u32(task.wrapping_add(MORPH_CONTROLLER), 0);
    e.mem.set_u32(task.wrapping_add(MORPH_TARGET), 0);
    this
}

/// The destructor body without its last step: releases the target, then the morph
/// controller, and stores the base vtable.
fn destruct_members(e: &mut Engine, task: u32) {
    let target = e.mem.u32(task.wrapping_add(MORPH_TARGET));
    if target != 0 {
        release(e, target);
    }
    let controller = e.mem.u32(task.wrapping_add(MORPH_CONTROLLER));
    if controller != 0 {
        release(e, controller);
    }
    e.mem.set_u32(task, BASE_VTABLE);
}

// Translated from 00a56c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiGeomMorpherUpdateTask` destructor body: releases the
/// target and the morph controller, stores the base vtable (`01069674`) and ends in the
/// `NiObject` destructor (`00a5d3d0`).
pub fn fn_00a56c10(e: &mut Engine, this: Ptr) {
    destruct_members(e, this.addr());
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a56c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeommorpherupdatetask.cpp`): sets the morph controller
/// and the target (both counted; the old ones released).
pub fn fn_00a56c60(e: &mut Engine, this: Ptr, controller: u32, target: u32) {
    let task = this.addr();
    assign_pointer(e, task.wrapping_add(MORPH_CONTROLLER), controller);
    assign_pointer(e, task.wrapping_add(MORPH_TARGET), target);
}

// Translated from 00a56ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, the count): a new array of `count` tasks behind a count
/// word: 4 bytes from the memory manager for 0, otherwise `count * 0x14 + 4` bytes from
/// `00aa1420` (all ones on overflow or carry); each task constructed (`00a56be0` through the
/// vector constructor iterator). Returns the first task, or 0 when the allocation fails.
pub fn fn_00a56ce0(e: &mut Engine, count: u32) -> u32 {
    let block = if count == 0 {
        e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 4u32])
            .u32()
    } else {
        let bytes = count
            .checked_mul(NI_GEOM_MORPHER_UPDATE_TASK_SIZE)
            .unwrap_or(u32::MAX);
        let bytes = bytes.checked_add(4).unwrap_or(u32::MAX);
        e.call(NI_ALLOCATE_ARRAY, &args![bytes]).u32()
    };
    if block == 0 {
        return 0;
    }
    let tasks = block.wrapping_add(4);
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            tasks,
            NI_GEOM_MORPHER_UPDATE_TASK_SIZE,
            count,
            TASK_CONSTRUCT
        ],
    );
    tasks
}

// Translated from 00a56d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherUpdateTask::_vector_deleting_destructor_` (Xbox PDB): with bit 1 of `flags`,
/// `this` is an array whose count is the word before it: from the last task to the first the
/// destructor body runs (target and controller released, the base vtable, `NiObject`'s
/// destructor), then, with bit 0, the block (from the count word) goes to the array delete
/// (`00aa1490`, 0x14 bytes each) and the block's address is returned. Without bit 1 the one
/// task's destructor body (`00a56c10`) runs and, with bit 0, the sized delete of 0x14 bytes;
/// `this` is returned.
pub fn ni_geom_morpher_update_task_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> u32 {
    let array = this.addr();
    if flags & 2 == 0 {
        fn_00a56c10(e, this);
        if flags & 1 != 0 {
            e.call(
                NI_SIZED_DELETE,
                &args![this, NI_GEOM_MORPHER_UPDATE_TASK_SIZE],
            );
        }
        return array;
    }
    let block = array.wrapping_sub(4);
    let count = e.mem.u32(block);
    let mut task = array.wrapping_add(count.wrapping_mul(NI_GEOM_MORPHER_UPDATE_TASK_SIZE));
    let mut remaining = count as i32;
    loop {
        remaining = remaining.wrapping_sub(1);
        if remaining < 0 {
            break;
        }
        task = task.wrapping_sub(NI_GEOM_MORPHER_UPDATE_TASK_SIZE);
        destruct_members(e, task);
        e.call(NI_OBJECT_DESTRUCT, &args![task]);
    }
    if flags & 1 != 0 {
        e.call(
            NI_ARRAY_DELETE,
            &args![block, NI_GEOM_MORPHER_UPDATE_TASK_SIZE],
        );
    }
    block
}

// Translated from 00a56e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeommorpherupdatetask.cpp`; thiscall on the task pool,
/// the count): a new block of `count` tasks (a 12-byte header from the memory manager: the
/// array (`00a56ce0`, null for 0), the count and the next block), each task added to the
/// pool's free set (data +0, capacity +4, size +8; grown to twice the capacity, or 1, by
/// `NiTSet::Realloc` when full), then the block is linked in front of the pool's list (+0xc).
/// A failed header allocation leaves a null block the exe then reads and writes through.
pub fn fn_00a56e60(e: &mut Engine, this: Ptr, count: u32) {
    let pool = this.addr();
    let header = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32();
    if header != 0 {
        e.mem.set_u32(header.wrapping_add(4), count);
        e.mem.set_u32(header.wrapping_add(8), 0);
        let tasks = if count != 0 { fn_00a56ce0(e, count) } else { 0 };
        e.mem.set_u32(header, tasks);
    }
    let mut index = 0u32;
    let mut offset = 0u32;
    while index < count {
        let task = if index < e.mem.u32(header.wrapping_add(4)) {
            e.mem.u32(header).wrapping_add(offset)
        } else {
            0
        };
        let capacity = e.mem.u32(pool.wrapping_add(4));
        if e.mem.u32(pool.wrapping_add(8)) == capacity {
            let grown = if capacity != 0 {
                capacity.wrapping_add(capacity)
            } else {
                1
            };
            e.call(SET_REALLOC, &args![pool, grown]);
        }
        let size = e.mem.u32(pool.wrapping_add(8));
        let data = e.mem.u32(pool);
        e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), task);
        e.mem.set_u32(pool.wrapping_add(8), size.wrapping_add(1));
        index += 1;
        offset = offset.wrapping_add(NI_GEOM_MORPHER_UPDATE_TASK_SIZE);
    }
    let list = e.mem.u32(pool.wrapping_add(0xc));
    e.mem.set_u32(header.wrapping_add(8), list);
    e.mem.set_u32(pool.wrapping_add(0xc), header);
}

// Translated from 00a56f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherUpdateTask::Clear` (Xbox PDB): releases and clears the morph controller and
/// the target, then hands the task back to the pool at `011a940c` (`00e7ec00` with the
/// address of a stack word holding the task) inside the pool's critical section
/// (`011f4180`). True.
pub fn ni_geom_morpher_update_task_clear(e: &mut Engine, this: Ptr) -> bool {
    let task = this.addr();
    let controller = e.mem.u32(task.wrapping_add(MORPH_CONTROLLER));
    if controller != 0 {
        release(e, controller);
        e.mem.set_u32(task.wrapping_add(MORPH_CONTROLLER), 0);
    }
    let target = e.mem.u32(task.wrapping_add(MORPH_TARGET));
    if target != 0 {
        release(e, target);
        e.mem.set_u32(task.wrapping_add(MORPH_TARGET), 0);
    }
    e.call(ENTER_CRITICAL_SECTION, &args![POOL_CRITICAL_SECTION]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), task);
        e.call(POOL_RELEASE, &args![TASK_POOL, slot]);
    });
    e.call(LEAVE_CRITICAL_SECTION, &args![POOL_CRITICAL_SECTION]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a56c00, ni_geom_morpher_update_task_get_rtti() -> Ptr),
        entry!(0x00a5_6df0, fn_00a56df0()),
        entry!(0x00a5_6bb0, ni_geom_morpher_update_task_do_task(Ptr)),
        entry!(0x00a5_6be0, fn_00a56be0(Ptr) -> Ptr),
        entry!(0x00a5_6c10, fn_00a56c10(Ptr)),
        entry!(0x00a5_6c60, fn_00a56c60(Ptr, u32, u32)),
        entry!(0x00a5_6ce0, fn_00a56ce0(u32) -> u32),
        entry!(
            0x00a5_6d40,
            ni_geom_morpher_update_task_vector_deleting_destructor(Ptr, u32) -> u32
        ),
        entry!(0x00a5_6e60, fn_00a56e60(Ptr, u32)),
        entry!(0x00a5_6f10, ni_geom_morpher_update_task_clear(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_geom_morpher_update_task_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a56c00, &args![]).u32(),
            NI_GEOM_MORPHER_UPDATE_TASK_RTTI
        );
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 != 0x00a5_6df0)
            .collect()
    }

    #[test]
    fn fn_00a56df0_tears_down_the_block_and_copies_the_word() {
        let mut e = Engine::new();
        e.map(CLEARED_WORD & !0xfff, 0x1000);
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(D3D_PASS_RELEASE, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        e.mem.set_u32(CLEARED_WORD, 7);
        e.mem.set_u32(COPY_SOURCE, 0x1234);
        // No block: only the two globals.
        e.call_log = Some(vec![]);
        e.call(0x00a5_6df0, &args![]);
        assert!(calls(&mut e).is_empty());
        assert_eq!(e.mem.u32(CLEARED_WORD), 0);
        assert_eq!(e.mem.u32(COPY_DESTINATION), 0x1234);

        let block = e.mem.alloc(0xc);
        let counted = e.mem.alloc(8);
        e.mem.set_u32(counted, 1);
        e.mem.set_u32(counted + 4, 0x0150_0000);
        e.mem.set_u32(block, counted + 4);
        e.mem.set_u32(block + 8, 0x6000);
        e.mem.set_u32(TASK_BLOCK, block);
        e.call_log = Some(vec![]);
        e.call(0x00a5_6df0, &args![]);
        assert_eq!(
            calls(&mut e),
            vec![
                (0x0150_1000, vec![counted + 4, 3]),
                (D3D_PASS_RELEASE, vec![0x6000, 1]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
        assert_eq!(e.mem.u32(TASK_BLOCK), block);

        e.mem.set_u32(counted, 0);
        e.mem.set_u32(block + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_6df0, &args![]);
        assert_eq!(
            calls(&mut e),
            vec![
                (DEALLOCATE, vec![MEMORY_MANAGER, counted]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The interlocked import slots, logged and counting in memory.
    fn interlocked(e: &mut Engine, log: &Log) {
        for (address, step) in [
            (INTERLOCKED_INCREMENT, 1u32),
            (INTERLOCKED_DECREMENT, u32::MAX),
        ] {
            let log = log.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let count = e.mem.u32(a[0]).wrapping_add(step);
                e.mem.set_u32(a[0], count);
                ret(count)
            });
        }
    }

    /// A counted object (count `count`) whose `DeleteThis` and slot `0xa8` log.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, 0x00ee_b000);
        e.mem.set_u32(vtable + SLOT_ON_PRE_DISPLAY, 0x00ee_b010);
        e.mem.set_u32(vtable + SLOT_CLEAR, 0x00ee_b020);
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn do_task_runs_the_pre_display_of_shared_targets() {
        let mut e = Engine::new();
        let log = Log::default();
        for address in [0x00ee_b010u32, 0x00ee_b020] {
            logged(&mut e, &log, address, 0);
        }
        e.map(DO_TASK_SKIP_PRE_DISPLAY, 1);
        let controller = counted(&mut e, 1);
        let target = counted(&mut e, 1);
        let task = counted(&mut e, 1);
        e.mem.set_u32(task + MORPH_CONTROLLER, controller);
        e.mem.set_u32(task + MORPH_TARGET, target);
        e.call(0x00a5_6bb0, &args![task]);
        assert!(calls(&log, 0x00ee_b010).is_empty());
        e.mem.set_u32(target + 4, 2);
        e.call(0x00a5_6bb0, &args![task]);
        assert_eq!(calls(&log, 0x00ee_b010), [vec![controller]]);
        e.mem.set_u8(DO_TASK_SKIP_PRE_DISPLAY, 1);
        e.call(0x00a5_6bb0, &args![task]);
        assert_eq!(calls(&log, 0x00ee_b010).len(), 1);
        assert_eq!(
            calls(&log, 0x00ee_b020),
            [vec![task], vec![task], vec![task]]
        );
    }

    #[test]
    fn constructor_destructor_and_setter_manage_the_pointers() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        logged(&mut e, &log, NI_OBJECT_CONSTRUCT, 0);
        logged(&mut e, &log, NI_OBJECT_DESTRUCT, 0);
        logged(&mut e, &log, 0x00ee_b000, 0);
        let task = e.mem.alloc(0x14);
        e.mem.write(task, &[0xaa; 0x14]);
        assert_eq!(e.call(0x00a5_6be0, &args![task]).u32(), task);
        assert_eq!(e.mem.u32(task), NI_GEOM_MORPHER_UPDATE_TASK_VTABLE);
        assert_eq!(e.mem.bytes(task + 8, 0xc), vec![0; 0xc]);
        let controller = counted(&mut e, 0);
        let target = counted(&mut e, 0);
        e.call(0x00a5_6c60, &args![task, controller, target]);
        assert_eq!(e.mem.u32(task + MORPH_CONTROLLER), controller);
        assert_eq!(e.mem.u32(controller + 4), 1);
        assert_eq!(e.mem.u32(target + 4), 1);
        e.call(0x00a5_6c10, &args![task]);
        assert_eq!(e.mem.u32(task), BASE_VTABLE);
        // The target is released first, then the controller.
        assert_eq!(calls(&log, 0x00ee_b000), [vec![target], vec![controller]]);
        assert_eq!(calls(&log, NI_OBJECT_DESTRUCT), [vec![task]]);
    }

    #[test]
    fn fn_00a56ce0_allocates_and_constructs_the_array() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(4 + 3 * 0x14);
        logged(&mut e, &log, NI_ALLOCATE_ARRAY, block);
        logged(&mut e, &log, MEMORY_MANAGER_ALLOCATE, 0);
        logged(&mut e, &log, VECTOR_CONSTRUCTOR_ITERATOR, 0);
        assert_eq!(e.call(0x00a5_6ce0, &args![3u32]).u32(), block + 4);
        assert_eq!(e.mem.u32(block), 3);
        assert_eq!(calls(&log, NI_ALLOCATE_ARRAY), [vec![4 + 3 * 0x14]]);
        assert_eq!(
            calls(&log, VECTOR_CONSTRUCTOR_ITERATOR),
            [vec![block + 4, 0x14, 3, TASK_CONSTRUCT]]
        );
        // Zero tasks come from the memory manager; a failed allocation gives 0.
        assert_eq!(e.call(0x00a5_6ce0, &args![0u32]).u32(), 0);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_ALLOCATE),
            [vec![MEMORY_MANAGER, 4]]
        );
        e.call(0x00a5_6ce0, &args![0x1000_0000u32]);
        assert_eq!(calls(&log, NI_ALLOCATE_ARRAY)[1], [u32::MAX]);
    }

    #[test]
    fn vector_deleting_destructor_destroys_one_or_an_array() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        for address in [
            NI_OBJECT_DESTRUCT,
            NI_SIZED_DELETE,
            NI_ARRAY_DELETE,
            0x00ee_b000,
        ] {
            logged(&mut e, &log, address, 0);
        }
        let block = e.mem.alloc(4 + 2 * 0x14);
        e.mem.set_u32(block, 2);
        let first = block + 4;
        let second = first + 0x14;
        let target = counted(&mut e, 1);
        e.mem.set_u32(first + MORPH_TARGET, target);
        let controller = counted(&mut e, 1);
        e.mem.set_u32(second + MORPH_CONTROLLER, controller);
        assert_eq!(e.call(0x00a5_6d40, &args![first, 3u32]).u32(), block);
        assert_eq!(calls(&log, NI_OBJECT_DESTRUCT), [vec![second], vec![first]]);
        assert_eq!(calls(&log, 0x00ee_b000), [vec![controller], vec![target]]);
        assert_eq!(e.mem.u32(first), BASE_VTABLE);
        assert_eq!(calls(&log, NI_ARRAY_DELETE), [vec![block, 0x14]]);
        let one = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00a5_6d40, &args![one, 1u32]).u32(), one);
        assert_eq!(calls(&log, NI_SIZED_DELETE), [vec![one, 0x14]]);
        e.call(0x00a5_6d40, &args![one, 0u32]);
        assert_eq!(calls(&log, NI_SIZED_DELETE).len(), 1);
        assert_eq!(calls(&log, NI_OBJECT_DESTRUCT).len(), 4);
    }

    #[test]
    fn fn_00a56e60_adds_a_block_of_tasks_to_the_pool() {
        let mut e = Engine::new();
        let log = Log::default();
        let header = e.mem.alloc(0xc);
        logged(&mut e, &log, MEMORY_MANAGER_ALLOCATE, header);
        let tasks = e.mem.alloc(4 + 2 * 0x14);
        logged(&mut e, &log, NI_ALLOCATE_ARRAY, tasks);
        logged(&mut e, &log, VECTOR_CONSTRUCTOR_ITERATOR, 0);
        let grows = log.clone();
        e.register_double(SET_REALLOC, move |e, a| {
            grows.borrow_mut().push((SET_REALLOC, a.to_vec()));
            let data = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0]);
            for i in 0..e.mem.u32(a[0] + 8) {
                let word = e.mem.u32(old + 4 * i);
                e.mem.set_u32(data + 4 * i, word);
            }
            e.mem.set_u32(a[0], data);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let pool = e.mem.alloc(0x10);
        e.mem.set_u32(pool + 0xc, 0x5000);
        e.call(0x00a5_6e60, &args![pool, 2u32]);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_ALLOCATE),
            [vec![MEMORY_MANAGER, 0xc]]
        );
        assert_eq!(e.mem.u32(header), tasks + 4);
        assert_eq!(e.mem.u32(header + 4), 2);
        assert_eq!(e.mem.u32(header + 8), 0x5000);
        assert_eq!(e.mem.u32(pool + 0xc), header);
        assert_eq!(calls(&log, SET_REALLOC), [vec![pool, 1], vec![pool, 2]]);
        let data = e.mem.u32(pool);
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4)],
            [tasks + 4, tasks + 0x18]
        );
        assert_eq!(e.mem.u32(pool + 8), 2);
    }

    #[test]
    fn clear_releases_and_returns_the_task_to_the_pool() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        logged(&mut e, &log, ENTER_CRITICAL_SECTION, 0);
        logged(&mut e, &log, LEAVE_CRITICAL_SECTION, 0);
        logged(&mut e, &log, 0x00ee_b000, 0);
        let handed = Rc::new(Cell::new(0u32));
        let seen = handed.clone();
        let order = log.clone();
        e.register_double(POOL_RELEASE, move |e, a| {
            order.borrow_mut().push((POOL_RELEASE, a.to_vec()));
            seen.set(e.mem.u32(a[1]));
            Ret::default()
        });
        let task = e.mem.alloc(0x14);
        let controller = counted(&mut e, 1);
        e.mem.set_u32(task + MORPH_CONTROLLER, controller);
        assert!(e.call(0x00a5_6f10, &args![task]).bool());
        assert_eq!(e.mem.u32(task + MORPH_CONTROLLER), 0);
        assert_eq!(calls(&log, 0x00ee_b000), [vec![controller]]);
        assert_eq!(handed.get(), task);
        let order: Vec<u32> = log
            .borrow()
            .iter()
            .map(|c| c.0)
            .filter(|a| [ENTER_CRITICAL_SECTION, POOL_RELEASE, LEAVE_CRITICAL_SECTION].contains(a))
            .collect();
        assert_eq!(
            order,
            [ENTER_CRITICAL_SECTION, POOL_RELEASE, LEAVE_CRITICAL_SECTION]
        );
        assert_eq!(calls(&log, POOL_RELEASE)[0][0], TASK_POOL);
        assert_eq!(
            calls(&log, ENTER_CRITICAL_SECTION),
            [vec![POOL_CRITICAL_SECTION]]
        );
    }
}
