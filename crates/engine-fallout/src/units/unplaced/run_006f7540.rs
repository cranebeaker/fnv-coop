//! `(unplaced)/run_006f7540` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructor and destructor here belong to the distant tree block
//! (the constructor's memory context names
//! `...\Distant Terrain System\BGSDistantTreeBlock.cpp`, and its member
//! functions are in that unit); the class layout is not taken from the
//! Xbox PDB, so the members are described by offset.

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor (`00528cb0`, thiscall, one word: 0) and destructor
/// (`0044cbf0`).
const BASE_CONSTRUCT_00528CB0: u32 = 0x0052_8cb0;
const BASE_DESTRUCT_0044CBF0: u32 = 0x0044_cbf0;
/// Member constructors (`bgsdistanttreeblock.cpp` / `extradataobjects.cpp`):
/// +0x04 (`006f9870`), +0x14 (`006f9680`, 0x25), +0x24 (`006f96b0`, 0x25),
/// +0x34 (`00438660`); and their destructors in reverse order: +0x34
/// (`00438690`), +0x24 (`006f9b80`), +0x14 (`006f9940`), +0x04 (`006f98a0`).
const MEMBER_04_CONSTRUCT: u32 = 0x006f_9870;
const MEMBER_14_CONSTRUCT: u32 = 0x006f_9680;
const MEMBER_24_CONSTRUCT: u32 = 0x006f_96b0;
const MEMBER_34_CONSTRUCT: u32 = 0x0043_8660;
const MEMBER_34_DESTRUCT: u32 = 0x0043_8690;
const MEMBER_24_DESTRUCT: u32 = 0x006f_9b80;
const MEMBER_14_DESTRUCT: u32 = 0x006f_9940;
const MEMBER_04_DESTRUCT: u32 = 0x006f_98a0;
/// The stack memory context (`00404eb0`, thiscall: pool 0x25, 1, the file
/// name, the line; `00404ee0` ends it).
const MEMORY_CONTEXT_BEGIN: u32 = 0x0040_4eb0;
const MEMORY_CONTEXT_END: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\Distant Terrain
/// System\BGSDistantTreeBlock.cpp"` (`0106dd30`).
const SOURCE_FILE_NAME: u32 = 0x0106_dd30;
/// The live-block count (`011d8598`) and the last value of the
/// constructor's second argument (`011d859c`).
const BLOCK_COUNT: u32 = 0x011d_8598;
const LAST_SECOND_ARGUMENT: u32 = 0x011d_859c;
/// Reads the word at its `this` (`00559450`).
const READ_POINTER: u32 = 0x0055_9450;
/// The game's allocator (`00401000`, cdecl).
const ALLOCATE: u32 = 0x0040_1000;
/// The queued task's constructor (`006f9360`, thiscall, six words).
const TASK_CONSTRUCT_006F9360: u32 = 0x006f_9360;
/// `NiPointer<QueuedFile>::operator=` (`006f74f0`, engine map; thiscall on
/// `this`, the new object).
const SET_TASK: u32 = 0x006f_74f0;
/// The task's virtual slot +0x20 the constructor calls.
const TASK_SLOT_20: u32 = 0x20;
/// `006f86e0` (thiscall on `this`, run first by the destructor).
const RELEASE_006F86E0: u32 = 0x006f_86e0;
/// The task manager singleton (`01202d98`) and
/// `BSTaskManager<__int64>::CancelTask` (`0044ac40`, two words).
const TASK_MANAGER: u32 = 0x0120_2d98;
const CANCEL_TASK: u32 = 0x0044_ac40;
/// The array's release (`008454f0`, thiscall, one word).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// Fields written by the constructor: the first argument (+0x44) and two
/// bytes (+0x48, +0x49).
const FIRST_ARGUMENT: u32 = 0x44;
const BYTE_48: u32 = 0x48;
const BYTE_49: u32 = 0x49;
/// The array member `006f7780` releases (+0x34).
const ARRAY_34: u32 = 0x34;

// Translated from 006f7540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f7540 - the function's C++ exception frame (FS:[0], unwind states of the members) is not translated
/// The distant tree block's constructor (thiscall, `RET 0x10`; unnamed in
/// the engine map): the base (`00528cb0(this, 0)`) and the four members,
/// then inside a memory context (pool 0x25, this file, line 0x2e): the
/// second argument is stored at `011d859c` and the block count at
/// `011d8598` raised; `first` goes to +0x44, the bytes at +0x48 and +0x49
/// are cleared; a 0x48-byte task (`006f9360(task, this, **first, x, y,
/// second, 0)`, `x` and `y` sign-extended; null when the allocation fails)
/// is held through `006f74f0`, and the held task's virtual slot +0x20 is
/// called. Returns `this`.
pub fn fn_006f7540(e: &mut Engine, this: Ptr, first: u32, second: u32, x: i16, y: i16) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00528CB0, &args![this, 0u32]);
    e.call(MEMBER_04_CONSTRUCT, &args![base + 0x04]);
    e.call(MEMBER_14_CONSTRUCT, &args![base + 0x14, 0x25u32]);
    e.call(MEMBER_24_CONSTRUCT, &args![base + 0x24, 0x25u32]);
    e.call(MEMBER_34_CONSTRUCT, &args![base + 0x34]);
    e.with_stack(4, |e, context| {
        e.call(
            MEMORY_CONTEXT_BEGIN,
            &args![context, 0x25u32, 1u32, SOURCE_FILE_NAME, 0x2eu32],
        );
        e.set_global(LAST_SECOND_ARGUMENT, second);
        let count = e.global::<u32>(BLOCK_COUNT).wrapping_add(1);
        e.set_global(BLOCK_COUNT, count);
        e.mem.set_u32(base + FIRST_ARGUMENT, first);
        e.mem.set_u8(base + BYTE_48, 0);
        e.mem.set_u8(base + BYTE_49, 0);
        let inner = e.call(READ_POINTER, &args![first]).u32();
        let target = e.call(READ_POINTER, &args![inner]).u32();
        let memory = e.call(ALLOCATE, &args![0x48u32]).u32();
        let task = if memory == 0 {
            0
        } else {
            e.call(
                TASK_CONSTRUCT_006F9360,
                &args![
                    memory,
                    this,
                    target,
                    i32::from(x) as u32,
                    i32::from(y) as u32,
                    second,
                    0u32
                ],
            )
            .u32()
        };
        e.call(SET_TASK, &args![this, task]);
        let held = e.call(READ_POINTER, &args![this]).u32();
        e.vcall(held, TASK_SLOT_20, &args![]);
        e.call(MEMORY_CONTEXT_END, &args![context]);
    });
    this
}

// Translated from 006f76b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f76b0 - the function's C++ exception frame (FS:[0], unwind states of the members) is not translated
/// The distant tree block's destructor body (unnamed in the engine map):
/// lowers the block count at `011d8598`, runs `006f86e0` and `006f7780`,
/// cancels a held task with the task manager at `01202d98`
/// (`CancelTask(task, 0)`), drops the task (`006f74f0(this, 0)`), then
/// destroys the members (+0x34, +0x24, +0x14, +0x04) and the base.
pub fn fn_006f76b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let count = e.global::<u32>(BLOCK_COUNT).wrapping_sub(1);
    e.set_global(BLOCK_COUNT, count);
    e.call(RELEASE_006F86E0, &args![this]);
    fn_006f7780(e, this);
    if e.call(READ_POINTER, &args![this]).u32() != 0 {
        let task = e.call(READ_POINTER, &args![this]).u32();
        let manager = e.global::<u32>(TASK_MANAGER);
        e.call(CANCEL_TASK, &args![manager, task, 0u32]);
    }
    e.call(SET_TASK, &args![this, 0u32]);
    e.call(MEMBER_34_DESTRUCT, &args![base + 0x34]);
    e.call(MEMBER_24_DESTRUCT, &args![base + 0x24]);
    e.call(MEMBER_14_DESTRUCT, &args![base + 0x14]);
    e.call(MEMBER_04_DESTRUCT, &args![base + 0x04]);
    e.call(BASE_DESTRUCT_0044CBF0, &args![this]);
}

// Translated from 006f7780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the array member at +0x34 (`008454f0(this + 0x34, 1)`;
/// unnamed in the engine map).
pub fn fn_006f7780(e: &mut Engine, this: Ptr) {
    e.call(ARRAY_RELEASE_008454F0, &args![this.addr() + ARRAY_34, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_7540, fn_006f7540(Ptr, u32, u32, i16, i16) -> Ptr),
        entry!(0x006f_76b0, fn_006f76b0(Ptr)),
        entry!(0x006f_7780, fn_006f7780(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TASK_VTABLE: u32 = 0x0712_0000;
    const TASK_SLOT_TARGET: u32 = 0x0055_0000;

    fn others(e: &Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a != top && *a != READ_POINTER)
            .cloned()
            .collect()
    }

    fn quiet_doubles(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn constructor_builds_members_and_queues_its_task() {
        let mut e = Engine::new();
        quiet_doubles(
            &mut e,
            &[
                BASE_CONSTRUCT_00528CB0,
                MEMBER_04_CONSTRUCT,
                MEMBER_14_CONSTRUCT,
                MEMBER_24_CONSTRUCT,
                MEMBER_34_CONSTRUCT,
                MEMORY_CONTEXT_BEGIN,
                MEMORY_CONTEXT_END,
                TASK_SLOT_TARGET,
            ],
        );
        e.register(READ_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ALLOCATE, |_, _| 0x0600_0000u32.into_ret());
        e.register(TASK_CONSTRUCT_006F9360, |_, a| a[0].into_ret());
        e.register(SET_TASK, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.map(BLOCK_COUNT, 8);
        e.mem.set_u32(BLOCK_COUNT, 4);
        e.map(0x0600_0000, 4);
        e.mem.set_u32(0x0600_0000, TASK_VTABLE);
        let mut slots = vec![0; 9];
        slots[8] = TASK_SLOT_TARGET;
        e.put_vtable(TASK_VTABLE, &slots);
        // `first` points at a pointer to the target.
        let target_holder = e.mem.alloc(4);
        e.mem.set_u32(target_holder, 0x0abc_0000);
        let first = e.mem.alloc(4);
        e.mem.set_u32(first, target_holder);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + BYTE_48, 0xffff_ffff);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x006f_7540, &args![this, first, 0x77u32, 0xfffeu32, 3u32])
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this + FIRST_ARGUMENT), first);
        assert_eq!(e.mem.u32(this + BYTE_48), 0xffff_0000);
        assert_eq!(e.global::<u32>(BLOCK_COUNT), 5);
        assert_eq!(e.global::<u32>(LAST_SECOND_ARGUMENT), 0x77);
        let calls = others(&e, 0x006f_7540);
        let context = calls[5].1[0];
        assert_eq!(
            calls,
            vec![
                (BASE_CONSTRUCT_00528CB0, vec![this, 0]),
                (MEMBER_04_CONSTRUCT, vec![this + 4]),
                (MEMBER_14_CONSTRUCT, vec![this + 0x14, 0x25]),
                (MEMBER_24_CONSTRUCT, vec![this + 0x24, 0x25]),
                (MEMBER_34_CONSTRUCT, vec![this + 0x34]),
                (
                    MEMORY_CONTEXT_BEGIN,
                    vec![context, 0x25, 1, SOURCE_FILE_NAME, 0x2e]
                ),
                (ALLOCATE, vec![0x48]),
                (
                    TASK_CONSTRUCT_006F9360,
                    vec![0x0600_0000, this, 0x0abc_0000, 0xffff_fffe, 3, 0x77, 0]
                ),
                (SET_TASK, vec![this, 0x0600_0000]),
                (TASK_SLOT_TARGET, vec![0x0600_0000]),
                (MEMORY_CONTEXT_END, vec![context]),
            ]
        );
    }

    #[test]
    fn constructor_holds_a_null_task_when_the_allocation_fails() {
        let mut e = Engine::new();
        quiet_doubles(
            &mut e,
            &[
                BASE_CONSTRUCT_00528CB0,
                MEMBER_04_CONSTRUCT,
                MEMBER_14_CONSTRUCT,
                MEMBER_24_CONSTRUCT,
                MEMBER_34_CONSTRUCT,
                MEMORY_CONTEXT_BEGIN,
                MEMORY_CONTEXT_END,
                TASK_SLOT_TARGET,
                SET_TASK,
                ALLOCATE,
            ],
        );
        e.map(BLOCK_COUNT, 8);
        // Every read gives the object the slot call goes to.
        e.register(READ_POINTER, |_, _| 0x0600_0000u32.into_ret());
        e.map(0x0600_0000, 4);
        e.mem.set_u32(0x0600_0000, TASK_VTABLE);
        let mut slots = vec![0; 9];
        slots[8] = TASK_SLOT_TARGET;
        e.put_vtable(TASK_VTABLE, &slots);
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x006f_7540, &args![this, 0x0500_0000u32, 1u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == TASK_CONSTRUCT_006F9360));
        let set: Vec<_> = log.iter().filter(|(a, _)| *a == SET_TASK).collect();
        assert_eq!(set[0].1, vec![this, 0]);
    }

    #[test]
    fn destructor_cancels_the_task_and_destroys_the_members() {
        let mut e = Engine::new();
        quiet_doubles(
            &mut e,
            &[
                RELEASE_006F86E0,
                ARRAY_RELEASE_008454F0,
                CANCEL_TASK,
                SET_TASK,
                MEMBER_34_DESTRUCT,
                MEMBER_24_DESTRUCT,
                MEMBER_14_DESTRUCT,
                MEMBER_04_DESTRUCT,
                BASE_DESTRUCT_0044CBF0,
            ],
        );
        e.register(READ_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        e.map(BLOCK_COUNT, 4);
        e.mem.set_u32(BLOCK_COUNT, 2);
        e.map(TASK_MANAGER, 4);
        e.mem.set_u32(TASK_MANAGER, 0x0444_0000);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, 0x0600_0000);
        e.call_log = Some(vec![]);
        e.call(0x006f_76b0, &args![this]);
        assert_eq!(e.global::<u32>(BLOCK_COUNT), 1);
        assert_eq!(
            others(&e, 0x006f_76b0),
            vec![
                (RELEASE_006F86E0, vec![this]),
                (ARRAY_RELEASE_008454F0, vec![this + 0x34, 1]),
                (CANCEL_TASK, vec![0x0444_0000, 0x0600_0000, 0]),
                (SET_TASK, vec![this, 0]),
                (MEMBER_34_DESTRUCT, vec![this + 0x34]),
                (MEMBER_24_DESTRUCT, vec![this + 0x24]),
                (MEMBER_14_DESTRUCT, vec![this + 0x14]),
                (MEMBER_04_DESTRUCT, vec![this + 0x04]),
                (BASE_DESTRUCT_0044CBF0, vec![this]),
            ]
        );
        // Without a task nothing is cancelled.
        e.mem.set_u32(this, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_76b0, &args![this]);
        assert!(!others(&e, 0x006f_76b0)
            .iter()
            .any(|(a, _)| *a == CANCEL_TASK));
    }

    #[test]
    fn fn_006f7780_releases_the_array_at_0x34() {
        let mut e = Engine::new();
        e.register(ARRAY_RELEASE_008454F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_7780, &args![0x5000u32]);
        assert_eq!(
            others(&e, 0x006f_7780),
            vec![(ARRAY_RELEASE_008454F0, vec![0x5034, 1])]
        );
    }
}
