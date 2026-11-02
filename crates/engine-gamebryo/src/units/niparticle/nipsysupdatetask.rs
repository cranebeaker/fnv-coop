//! `NiParticle/nipsysupdatetask.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c25870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateTask::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2580.
pub fn ni_p_sys_update_task_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2580
}

/// The words `011aeba0` (cleared), `011aeba4` (the object freed) and `011aeba8` (set from
/// `011aebac`) `fn_00c25ac0` works on.
const CLEARED_011AEBA0: u32 = 0x011a_eba0;
const OBJECT_011AEBA4: u32 = 0x011a_eba4;
const TARGET_011AEBA8: u32 = 0x011a_eba8;
const SOURCE_011AEBAC: u32 = 0x011a_ebac;
/// The memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `00e7f900` (unit `nid3dpass.cpp`, unnamed in the engine map; `thiscall`, one word).
const RELEASE_00E7F900: u32 = 0x00e7_f900;

// Translated from 00c25ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function, read from its bytes; no arguments; the class
/// unregistration `00c1a660` ends in a jump to it). Clears the word at `011aeba0`. When the object at
/// `011aeba4` is not null: its array at +0 (an `operator new[]` block with the count in the
/// word before it), when not null, is deleted through the first element's vtable slot 0 with
/// 3 when the count is not 0, else its block (the count word's address) freed through the
/// memory manager; the object at +8, when not null, gets `00e7f900` with 1; and the object is
/// freed through the memory manager (`011aeba4` keeps its value). In both cases `011aeba8`
/// takes the word at `011aebac`.
pub fn fn_00c25ac0(e: &mut Engine) {
    let object = e.mem.u32(OBJECT_011AEBA4);
    e.mem.set_u32(CLEARED_011AEBA0, 0);
    if object != 0 {
        let array = e.mem.u32(object);
        if array != 0 {
            let block = array.wrapping_sub(4);
            if e.mem.u32(block) != 0 {
                e.vcall(array, 0, &args![3u32]);
            } else {
                e.call(MEMORY_FREE, &args![MEMORY_MANAGER, block]);
            }
        }
        let member = e.mem.u32(object.wrapping_add(8));
        if member != 0 {
            e.call(RELEASE_00E7F900, &args![member, 1u32]);
        }
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, object]);
    }
    let source = e.mem.u32(SOURCE_011AEBAC);
    e.mem.set_u32(TARGET_011AEBA8, source);
}

/// The `NiPSysUpdateTask` vtable (RTTI `.?AVNiPSysUpdateTask@@`) and the
/// `NiTask` one (`.?AVNiTask@@`).
const UPDATE_TASK_VTABLE: u32 = 0x010b_e45c;
const TASK_VTABLE: u32 = 0x0106_9674;
/// `00a5d3d0` (unnamed, `NiMain/niobject.cpp`, `thiscall`, no words): the
/// `NiTask` destructor body.
const TASK_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Imported `EnterCriticalSection` / `LeaveCriticalSection` and the pool's
/// critical section (`01202600`).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const POOL_LOCK: u32 = 0x0120_2600;
/// The update-task pool (`011aeb98`) and `00e7ec00` (named
/// `NiControllerSequence::SetState` in the engine map, a folded body;
/// `thiscall` on the pool, one word: the address of the task to add).
const POOL: u32 = 0x011a_eb98;
const POOL_ADD: u32 = 0x00e7_ec00;
/// `00aa3e40` (`BSCore/memorymanager.cpp`, `thiscall` on the memory manager,
/// one word: the size), `00aa1420` (`cdecl`, one word: the size), the sized
/// deletes `00aa1460` and `00aa1490` (`cdecl`, `(block, size)`), and the
/// vector constructor iterator `00401050` (`stdcall`, `(array, size, count,
/// constructor)`).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const ARRAY_ALLOCATE: u32 = 0x00aa_1420;
const SIZED_DELETE: u32 = 0x00aa_1460;
const ARRAY_SIZED_DELETE: u32 = 0x00aa_1490;
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// The `NiPSysUpdateTask` constructor (`00c25850`).
const UPDATE_TASK_CONSTRUCTOR: u32 = 0x00c2_5850;
/// `00a311e0` (named `NiTSet<NiNode*,..>::Realloc` in the engine map, a
/// folded body; `thiscall` on the set, one word: the new capacity).
const SET_REALLOC: u32 = 0x00a3_11e0;
/// The byte at `011f5b08`: while set, `DoTask` skips the system update.
const SKIP_UPDATE: u32 = 0x011f_5b08;
/// The task's system pointer (+0xC, reference counted) and update time
/// (+0x10).
const TASK_SYSTEM: u32 = 0xc;
const TASK_TIME: u32 = 0x10;

/// Drops a reference to `object` (`InterlockedDecrement` on +4), deleting
/// it through its vtable slot +4 when that reaches 0.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The destructor body of one task: the vtable, the system pointer released
/// and cleared (then checked again, as the exe does), the `NiTask` vtable
/// and its destructor (`00a5d3d0`).
fn destroy_task(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, UPDATE_TASK_VTABLE);
    let system = e.mem.u32(this + TASK_SYSTEM);
    if system != 0 {
        release(e, system);
        e.mem.set_u32(this + TASK_SYSTEM, 0);
    }
    let system = e.mem.u32(this + TASK_SYSTEM);
    if system != 0 {
        release(e, system);
    }
    e.mem.set_u32(this, TASK_VTABLE);
    e.call(TASK_DESTRUCTOR, &args![this]);
}

// Translated from 00c25880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysUpdateTask` destructor
/// (`thiscall`, no words): releases the system pointer (+0xC) and ends in
/// the `NiTask` destructor (`00a5d3d0`, a tail jump).
pub fn fn_00c25880(e: &mut Engine, this: Ptr) {
    destroy_task(e, this.addr());
}

// Translated from 00c258e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): sets the task's system
/// (a reference-counted assignment at +0xC: the old one released unless it
/// is the same, the new one's count incremented when not null) and its time
/// (+0x10, `FLD`/`FSTP`, quieting a signalling NaN).
pub fn fn_00c258e0(e: &mut Engine, this: Ptr, system: Ptr, time: u32) {
    let t = this.addr();
    let old = e.mem.u32(t + TASK_SYSTEM);
    if old != system.addr() {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(t + TASK_SYSTEM, system.addr());
        if !system.is_null() {
            e.call(INTERLOCKED_INCREMENT, &args![system.addr() + 4]);
        }
    }
    e.mem.set_u32(t + TASK_TIME, x87_quiet_bits(time));
}

// Translated from 00c25930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateTask::DoTask` (Xbox PDB): when the task has a system and
/// the byte at `011f5b08` is clear, runs the system's slot +0xF4 with the
/// time (`FLD`/`FSTP`, quieting a signalling NaN), then releases and clears
/// the system pointer (read again after the call). Ends in a tail jump to
/// the task's own slot +0x94.
pub fn ni_p_sys_update_task_do_task(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let system = e.mem.u32(t + TASK_SYSTEM);
    if system != 0 && e.mem.u8(SKIP_UPDATE) == 0 {
        let time = x87_quiet_bits(e.mem.u32(t + TASK_TIME));
        e.vcall(system, 0xf4, &args![time]);
        let system = e.mem.u32(t + TASK_SYSTEM);
        if system != 0 {
            release(e, system);
            e.mem.set_u32(t + TASK_SYSTEM, 0);
        }
    }
    e.vcall(t, 0x94, &args![]);
}

// Translated from 00c25990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the array `new` of `count` update tasks
/// (`cdecl`, one word). The block is `count * 0x14 + 4` bytes (all ones on
/// overflow) from `00aa1420`, or 4 bytes from the memory manager
/// (`00aa3e40`) when `count` is 0; its first word takes the count and the
/// tasks after it are constructed (`00401050` with `00c25850`). Answers the
/// first task, or null when the allocation fails.
pub fn fn_00c25990(e: &mut Engine, count: u32) -> u32 {
    let block = if count == 0 {
        e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32()
    } else {
        let bytes = count.checked_mul(0x14).unwrap_or(u32::MAX);
        let size = bytes.checked_add(4).unwrap_or(u32::MAX);
        e.call(ARRAY_ALLOCATE, &args![size]).u32()
    };
    if block == 0 {
        return 0;
    }
    let tasks = block + 4;
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCTOR,
        &args![tasks, 0x14u32, count, UPDATE_TASK_CONSTRUCTOR],
    );
    tasks
}

// Translated from 00c259f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateTask::_vector_deleting_destructor_` (Xbox PDB): with bit 1
/// of `flags`, `this` is an array whose count is the word before it: every
/// task is destroyed from the last to the first (the destructor inlined),
/// the block freed with bit 0 (`00aa1490`, `(block, 0x14)`), and the block
/// is the answer. Without it, the destructor (`00c25880`) and, with bit 0,
/// the sized delete (`00aa1460`, `(this, 0x14)`); the answer is `this`.
pub fn ni_p_sys_update_task_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> u32 {
    let flags = flags as u8;
    let t = this.addr();
    if flags & 2 == 0 {
        fn_00c25880(e, this);
        if flags & 1 != 0 {
            e.call(SIZED_DELETE, &args![t, 0x14u32]);
        }
        return t;
    }
    let block = t.wrapping_sub(4);
    let count = e.mem.u32(block);
    let mut task = t.wrapping_add(count.wrapping_mul(0x14));
    let mut left = count as i32;
    loop {
        left = left.wrapping_sub(1);
        if left < 0 {
            break;
        }
        task = task.wrapping_sub(0x14);
        destroy_task(e, task);
    }
    if flags & 1 != 0 {
        e.call(ARRAY_SIZED_DELETE, &args![block, 0x14u32]);
    }
    block
}

// Translated from 00c25b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the update-task pool, `RET 4`):
/// refills the pool with `count` new tasks. A 0xC-byte chunk from the memory
/// manager holds the task array (`00c25990`, null for 0 tasks), the count
/// and the next chunk; each task (null past the chunk's count) is pushed on
/// the pool's free list (array +0, capacity +4, count +8; grown through
/// `00a311e0` to twice the capacity, or 1, when full); then the chunk is
/// linked in front of the pool's chunk list (+0xC). A failed chunk
/// allocation is used as null, as the exe does.
pub fn fn_00c25b30(e: &mut Engine, this: Ptr, count: u32) {
    let pool = this.addr();
    let chunk = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32();
    if chunk != 0 {
        e.mem.set_u32(chunk + 4, count);
        e.mem.set_u32(chunk + 8, 0);
        if count != 0 {
            let tasks = fn_00c25990(e, count);
            e.mem.set_u32(chunk, tasks);
        } else {
            e.mem.set_u32(chunk, 0);
        }
    }
    let mut index = 0u32;
    while index < count {
        let task = if index < e.mem.u32(chunk.wrapping_add(4)) {
            e.mem.u32(chunk).wrapping_add(index.wrapping_mul(0x14))
        } else {
            0
        };
        let capacity = e.mem.u32(pool + 4);
        if e.mem.u32(pool + 8) == capacity {
            let grown = if capacity == 0 {
                1
            } else {
                capacity.wrapping_add(capacity)
            };
            e.call(SET_REALLOC, &args![pool, grown]);
        }
        let size = e.mem.u32(pool + 8);
        let array = e.mem.u32(pool);
        e.mem
            .set_u32(array.wrapping_add(size.wrapping_mul(4)), task);
        e.mem.set_u32(pool + 8, size.wrapping_add(1));
        index += 1;
    }
    let chain = e.mem.u32(pool + 0xc);
    e.mem.set_u32(chunk.wrapping_add(8), chain);
    e.mem.set_u32(pool + 0xc, chunk);
}

// Translated from 00c25be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateTask::Clear` (Xbox PDB): releases and clears the system
/// pointer, then, under the pool's lock, returns the task to the pool
/// (`00e7ec00` with the address of a local holding `this`). Answers true.
pub fn ni_p_sys_update_task_clear(e: &mut Engine, this: Ptr) -> bool {
    let t = this.addr();
    let system = e.mem.u32(t + TASK_SYSTEM);
    if system != 0 {
        release(e, system);
        e.mem.set_u32(t + TASK_SYSTEM, 0);
    }
    e.call(ENTER_CRITICAL_SECTION, &args![POOL_LOCK]);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), t);
        e.call(POOL_ADD, &args![POOL, local]);
    });
    e.call(LEAVE_CRITICAL_SECTION, &args![POOL_LOCK]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2_5880, fn_00c25880(Ptr)),
        entry!(0x00c2_58e0, fn_00c258e0(Ptr, Ptr, u32)),
        entry!(0x00c2_5930, ni_p_sys_update_task_do_task(Ptr)),
        entry!(0x00c2_5990, fn_00c25990(u32) -> u32),
        entry!(
            0x00c2_59f0,
            ni_p_sys_update_task_vector_deleting_destructor(Ptr, u32) -> u32
        ),
        entry!(0x00c2_5b30, fn_00c25b30(Ptr, u32)),
        entry!(0x00c2_5be0, ni_p_sys_update_task_clear(Ptr) -> bool),
        entry!(0x00c25870, ni_p_sys_update_task_get_r_t_t_i() -> u32),
        entry!(0x00c2_5ac0, fn_00c25ac0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_update_task_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_5870, &args![]).u32(), 0x0120_2580);
    }
}

#[cfg(test)]
mod release_tests {
    use super::*;

    const ELEMENT_DELETE: u32 = 0x0e20_0000;

    /// The globals mapped with 0x1111 in `011aebac` and `011aeba0` set; answers the engine.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_e000, 0x1000);
        e.set_global(CLEARED_011AEBA0, 0x2222u32);
        e.set_global(SOURCE_011AEBAC, 0x1111u32);
        for callee in [MEMORY_FREE, RELEASE_00E7F900, ELEMENT_DELETE] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// An object with an array (count word `count`, one element of a vtable whose slot 0 is
    /// [`ELEMENT_DELETE`]) and the member at +8; answers the object and the array.
    fn object(e: &mut Engine, count: u32, member: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, ELEMENT_DELETE);
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, count);
        e.mem.set_u32(block + 4, vtable);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, block + 4);
        e.mem.set_u32(object + 8, member);
        e.set_global(OBJECT_011AEBA4, object);
        (object, block + 4)
    }

    #[test]
    fn fn_00c25ac0_deletes_a_counted_array_and_releases_the_member() {
        let mut e = engine();
        let (object, array) = object(&mut e, 2, 0x0e30_0000);
        e.call(0x00c2_5ac0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (ELEMENT_DELETE, vec![array, 3]),
                (RELEASE_00E7F900, vec![0x0e30_0000, 1]),
                (MEMORY_FREE, vec![MEMORY_MANAGER, object]),
            ]
        );
        assert_eq!(e.mem.u32(CLEARED_011AEBA0), 0);
        assert_eq!(e.mem.u32(OBJECT_011AEBA4), object);
        assert_eq!(e.mem.u32(TARGET_011AEBA8), 0x1111);
    }

    #[test]
    fn fn_00c25ac0_frees_an_empty_array_block_and_skips_a_null_member() {
        let mut e = engine();
        let (object, array) = object(&mut e, 0, 0);
        e.call(0x00c2_5ac0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (MEMORY_FREE, vec![MEMORY_MANAGER, array - 4]),
                (MEMORY_FREE, vec![MEMORY_MANAGER, object]),
            ]
        );
    }

    #[test]
    fn fn_00c25ac0_without_an_object_only_copies_the_word() {
        let mut e = engine();
        e.call(0x00c2_5ac0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(CLEARED_011AEBA0), 0);
        assert_eq!(e.mem.u32(TARGET_011AEBA8), 0x1111);
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;

    const DELETE_THIS: u32 = 0x0100_0070;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Doubles for the interlocked imports, the `NiTask` destructor and the
    /// objects' delete slot; answers the engine.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(TASK_DESTRUCTOR, |_, _| Ret::default());
        e.register(DELETE_THIS, |_, _| Ret::default());
        e
    }

    /// A reference-counted object with `count` references.
    fn object(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn fn_00c25880_releases_the_system_and_destroys_the_task() {
        let mut e = engine();
        let system = object(&mut e, 1);
        let task = e.mem.alloc(0x14);
        e.mem.set_u32(task + TASK_SYSTEM, system);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5880, &args![task]);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![system]]);
        assert_eq!(e.mem.u32(task + TASK_SYSTEM), 0);
        assert_eq!(e.mem.u32(task), TASK_VTABLE);
        assert_eq!(calls_to(&e, TASK_DESTRUCTOR), vec![vec![task]]);
        // A shared system is only released.
        let system = object(&mut e, 2);
        e.mem.set_u32(task + TASK_SYSTEM, system);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5880, &args![task]);
        assert!(calls_to(&e, DELETE_THIS).is_empty());
        assert_eq!(e.mem.u32(system + 4), 1);
    }

    #[test]
    fn fn_00c258e0_assigns_the_system_and_the_time() {
        let mut e = engine();
        let old = object(&mut e, 1);
        let new = object(&mut e, 1);
        let task = e.mem.alloc(0x14);
        e.mem.set_u32(task + TASK_SYSTEM, old);
        e.call_log = Some(vec![]);
        e.call(0x00c2_58e0, &args![task, new, 0x7f80_0002u32]);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![old]]);
        assert_eq!(e.mem.u32(task + TASK_SYSTEM), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(e.mem.u32(task + TASK_TIME), 0x7fc0_0002);
        // The same system: no count changes.
        e.call(0x00c2_58e0, &args![task, new, 1.0f32]);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(e.mem.f32(task + TASK_TIME), 1.0);
        // Null: the old one released, nothing incremented.
        e.call(0x00c2_58e0, &args![task, 0u32, 1.0f32]);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(e.mem.u32(task + TASK_SYSTEM), 0);
    }

    #[test]
    fn do_task_updates_the_system_unless_skipped_then_runs_slot_0x94() {
        const UPDATE: u32 = 0x0100_0080;
        const AFTER: u32 = 0x0100_0090;
        for skip in [0u8, 1] {
            let mut e = engine();
            e.map(SKIP_UPDATE, 1);
            e.set_global(SKIP_UPDATE, skip);
            e.register(UPDATE, |_, _| Ret::default());
            e.register(AFTER, |_, _| Ret::default());
            let system = object(&mut e, 2);
            let system_vtable = e.mem.u32(system);
            e.mem.set_u32(system_vtable + 0xf4, UPDATE);
            let task_vtable = e.mem.alloc(0x100);
            e.mem.set_u32(task_vtable + 0x94, AFTER);
            let task = e.mem.alloc(0x14);
            e.mem.set_u32(task, task_vtable);
            e.mem.set_u32(task + TASK_SYSTEM, system);
            e.mem.set_u32(task + TASK_TIME, 0x7f80_0004);
            e.call_log = Some(vec![]);
            e.call(0x00c2_5930, &args![task]);
            assert_eq!(calls_to(&e, AFTER), vec![vec![task]]);
            if skip == 0 {
                assert_eq!(calls_to(&e, UPDATE), vec![vec![system, 0x7fc0_0004]]);
                assert_eq!(e.mem.u32(task + TASK_SYSTEM), 0);
                assert_eq!(e.mem.u32(system + 4), 1);
            } else {
                assert!(calls_to(&e, UPDATE).is_empty());
                assert_eq!(e.mem.u32(task + TASK_SYSTEM), system);
            }
        }
    }

    #[test]
    fn fn_00c25990_allocates_and_constructs_the_tasks() {
        let mut e = engine();
        e.register(ARRAY_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let tasks = e.call(0x00c2_5990, &args![3u32]).u32();
        assert_eq!(calls_to(&e, ARRAY_ALLOCATE), vec![vec![0x40]]);
        assert_eq!(e.mem.u32(tasks - 4), 3);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR),
            vec![vec![tasks, 0x14, 3, UPDATE_TASK_CONSTRUCTOR]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00c2_5990, &args![0u32]);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 4]]);
        e.call_log = Some(vec![]);
        e.register(ARRAY_ALLOCATE, |_, _| word(0));
        assert_eq!(e.call(0x00c2_5990, &args![0x1000_0000u32]).u32(), 0);
        assert_eq!(calls_to(&e, ARRAY_ALLOCATE), vec![vec![u32::MAX]]);
        assert!(calls_to(&e, VECTOR_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn vector_deleting_destructor_destroys_one_or_an_array() {
        let mut e = engine();
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.register(ARRAY_SIZED_DELETE, |_, _| Ret::default());
        let task = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_59f0, &args![task, 0x101u32]).u32(), task);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![task, 0x14]]);
        assert_eq!(calls_to(&e, TASK_DESTRUCTOR), vec![vec![task]]);
        let block = e.mem.alloc(4 + 3 * 0x14);
        e.mem.set_u32(block, 3);
        let first = block + 4;
        let system = object(&mut e, 1);
        e.mem.set_u32(first + 0x14 + TASK_SYSTEM, system);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_59f0, &args![first, 3u32]).u32(), block);
        assert_eq!(
            calls_to(&e, TASK_DESTRUCTOR),
            vec![vec![first + 0x28], vec![first + 0x14], vec![first]]
        );
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![system]]);
        assert_eq!(calls_to(&e, ARRAY_SIZED_DELETE), vec![vec![block, 0x14]]);
        e.mem.set_u32(block, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_59f0, &args![first, 2u32]).u32(), block);
        assert!(calls_to(&e, TASK_DESTRUCTOR).is_empty());
        assert!(calls_to(&e, ARRAY_SIZED_DELETE).is_empty());
    }

    #[test]
    fn fn_00c25b30_pushes_new_tasks_and_links_the_chunk() {
        let mut e = engine();
        e.register(ARRAY_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(SET_REALLOC, |e, a| {
            let array = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0]);
            for i in 0..e.mem.u32(a[0] + 8) {
                let v = e.mem.u32(old + 4 * i);
                e.mem.set_u32(array + 4 * i, v);
            }
            e.mem.set_u32(a[0], array);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let pool = e.mem.alloc(0x10);
        e.mem.set_u32(pool + 0xc, 0x0bad_0000);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5b30, &args![pool, 3u32]);
        assert_eq!(
            calls_to(&e, SET_REALLOC),
            vec![vec![pool, 1], vec![pool, 2], vec![pool, 4]]
        );
        let chunk = e.mem.u32(pool + 0xc);
        let tasks = e.mem.u32(chunk);
        assert_eq!(e.mem.u32(chunk + 4), 3);
        assert_eq!(e.mem.u32(chunk + 8), 0x0bad_0000);
        assert_eq!(e.mem.u32(pool + 8), 3);
        let array = e.mem.u32(pool);
        for i in 0..3 {
            assert_eq!(e.mem.u32(array + 4 * i), tasks + 0x14 * i);
        }
    }

    #[test]
    fn clear_releases_the_system_and_returns_the_task_to_the_pool() {
        let mut e = engine();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(POOL_ADD, |e, a| word(e.mem.u32(a[1])));
        let system = object(&mut e, 1);
        let task = e.mem.alloc(0x14);
        e.mem.set_u32(task + TASK_SYSTEM, system);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c2_5be0, &args![task]).bool());
        assert_eq!(e.mem.u32(task + TASK_SYSTEM), 0);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00c2_5be0,
                INTERLOCKED_DECREMENT,
                DELETE_THIS,
                ENTER_CRITICAL_SECTION,
                POOL_ADD,
                LEAVE_CRITICAL_SECTION
            ]
        );
        assert_eq!(log[4].1[0], POOL);
    }
}
