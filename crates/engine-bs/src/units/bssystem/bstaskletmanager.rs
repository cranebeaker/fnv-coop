//! `BSSystem/bstaskletmanager.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b00a80 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSTaskletManager::CreateTaskGroup` (engine map; `thiscall`, one stack word, `RET 4`: where to
/// put the group). When the manager runs (the byte at `+4`), its virtual slot `+0x10` makes the
/// group into a local and its answer (a byte) is the result, the local copied to `*out`;
/// otherwise `*out` is 0 and the result 0.
pub fn bs_tasklet_manager_create_task_group(e: &mut Engine, this: Ptr, out: u32) -> u8 {
    if e.mem.u8(this.addr() + 4) == 0 {
        e.mem.set_u32(out, 0);
        return 0;
    }
    e.with_stack(4, |e, group| {
        let made = e.vcall(this.addr(), 0x10, &args![group]).u8();
        let value = e.mem.u32(group.addr());
        e.mem.set_u32(out, value);
        made
    })
}

/// `MemoryManager::Allocate` (`00aa3e40`, `thiscall` on the manager `011f6238`, one word), the
/// task group's constructor `00b027c0` (`thiscall`), and the KERNEL32 import slots
/// `CreateSemaphoreA` and `InitializeCriticalSection`.
const ALLOCATE_00AA3E40: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_011F6238: u32 = 0x011f_6238;
const GROUP_CONSTRUCT_00B027C0: u32 = 0x00b0_27c0;
const IMPORT_CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// The vtables the group's constructor stores, the base's first (`010a48e8`) then its own
/// (`010a48d8`).
const GROUP_BASE_VTABLE_010A48E8: u32 = 0x010a_48e8;
const GROUP_VTABLE_010A48D8: u32 = 0x010a_48d8;

// Translated from 00b02010 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b02010 - the C++ exception frame (FS:[0] chain, handler 00f238fb, the state word around the constructor) is not translated; the security cookie is pushed but never checked.
/// Unnamed in the engine map: the manager's virtual slot `+0x10` that
/// [`bs_tasklet_manager_create_task_group`] calls (`thiscall`, one stack word, `RET 4`: where to
/// put the group; `this` is not read). Allocates 0x3c bytes (`00aa3e40` on `011f6238`) and, when
/// that gives memory, constructs a task group there (`00b027c0`); stores the group (or null) in
/// `*out` and answers whether there is one.
pub fn fn_00b02010(e: &mut Engine, _this: Ptr, out: u32) -> u8 {
    let block = e
        .call(ALLOCATE_00AA3E40, &args![MEMORY_MANAGER_011F6238, 0x3cu32])
        .u32();
    let group = if block == 0 {
        0
    } else {
        e.call(GROUP_CONSTRUCT_00B027C0, &args![block]).u32()
    };
    e.mem.set_u32(out, group);
    u8::from(group != 0)
}

// Translated from 00b027c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a task group's constructor (`thiscall`, no stack words; answers
/// `this`). The base's vtable `010a48e8` and the byte `+4` = 1, then its own vtable
/// `010a48d8`; at `+8` a semaphore block: `+8` 0, `+0x10` 1 and `+0xc` a new semaphore
/// (`CreateSemaphoreA(NULL, 0, 1, NULL)`); the critical section at `+0x14` initialized; the
/// bytes `+0x2c`, `+0x2d`, `+0x2e` and the words `+0x30`, `+0x34`, `+0x38` cleared.
pub fn fn_00b027c0(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, GROUP_BASE_VTABLE_010A48E8);
    e.mem.set_u8(t + 4, 1);
    e.mem.set_u32(t, GROUP_VTABLE_010A48D8);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0x10, 1);
    let semaphore = e
        .call(IMPORT_CREATE_SEMAPHORE_A, &args![0u32, 0u32, 1u32, 0u32])
        .u32();
    e.mem.set_u32(t + 0xc, semaphore);
    e.call(IMPORT_INITIALIZE_CRITICAL_SECTION, &args![t + 0x14]);
    e.mem.set_u8(t + 0x2c, 0);
    e.mem.set_u8(t + 0x2d, 0);
    e.mem.set_u8(t + 0x2e, 0);
    e.mem.set_u32(t + 0x30, 0);
    e.mem.set_u32(t + 0x34, 0);
    e.mem.set_u32(t + 0x38, 0);
    this
}

/// The two hook lists the workers run (`BSTaskletManager`'s statics): the counts at `011f8af0` /
/// `011f8af4` and the object arrays at `011f8af8` / `011f8b38`.
const ENTER_COUNT: u32 = 0x011f_8af0;
const LEAVE_COUNT: u32 = 0x011f_8af4;
const ENTER_HOOKS: u32 = 0x011f_8af8;
const LEAVE_HOOKS: u32 = 0x011f_8b38;

/// Slot `+4` of each set object of a hook array, for the first `count` (compared signed, `JGE`).
fn run_hooks(e: &mut Engine, count: u32, hooks: u32) {
    let mut i: i32 = 0;
    while i < e.mem.i32(count) {
        let hook = e.mem.u32(hooks.wrapping_add((i as u32).wrapping_mul(4)));
        if hook != 0 {
            e.vcall(hook, 4, &args![]);
        }
        i = i.wrapping_add(1);
    }
}

// Translated from 00b00c90 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `ECX` unused, plain `RET`; a worker thread runs it as it
/// starts, `00b023f0`): slot `+4` of each set object of the first `[011f8af0]` at `011f8af8`.
pub fn fn_00b00c90(e: &mut Engine, _this: Ptr) {
    run_hooks(e, ENTER_COUNT, ENTER_HOOKS);
}

// Translated from 00b00cf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `ECX` unused, plain `RET`; a worker thread runs it as it
/// ends): slot `+4` of each set object of the first `[011f8af4]` at `011f8b38`.
pub fn fn_00b00cf0(e: &mut Engine, _this: Ptr) {
    run_hooks(e, LEAVE_COUNT, LEAVE_HOOKS);
}

// Translated from 00b00ae0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, `RET 4`: the address of a task group
/// pointer): when the manager runs (the byte at `+4`) and the group is set, the manager's
/// virtual slot `+0x14` with the group answers (a byte); otherwise 0.
pub fn fn_00b00ae0(e: &mut Engine, this: Ptr, group: u32) -> u8 {
    if e.mem.u8(this.addr() + 4) == 0 || e.mem.u32(group) == 0 {
        return 0;
    }
    let g = e.mem.u32(group);
    e.vcall(this.addr(), 0x14, &args![g]).u8()
}

// Translated from 00b00b40 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three stack words, `RET 0xc`: the address of a task
/// group pointer, an object whose word `+4` is the task, and a byte): when the manager runs (the
/// byte at `+4`), the group is set and the task is set, the manager's virtual slot `+0x18` with
/// (group, task, the byte zero-extended) answers (a byte); otherwise 0.
pub fn fn_00b00b40(e: &mut Engine, this: Ptr, group: u32, holder: u32, flag: u8) -> u8 {
    if e.mem.u8(this.addr() + 4) == 0
        || e.mem.u32(group) == 0
        || e.mem.u32(holder.wrapping_add(4)) == 0
    {
        return 0;
    }
    let task = e.mem.u32(holder.wrapping_add(4));
    let g = e.mem.u32(group);
    e.vcall(this.addr(), 0x18, &args![g, task, u32::from(flag)])
        .u8()
}

// Translated from 00b00bc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, `RET 4`: the address of a task group
/// pointer): when the manager runs (the byte at `+4`) and the group is set, the manager's
/// virtual slot `+0x1c` with the group answers (a byte); otherwise 0.
pub fn fn_00b00bc0(e: &mut Engine, this: Ptr, group: u32) -> u8 {
    if e.mem.u8(this.addr() + 4) == 0 || e.mem.u32(group) == 0 {
        return 0;
    }
    let g = e.mem.u32(group);
    e.vcall(this.addr(), 0x1c, &args![g]).u8()
}

// Translated from 00b00c20 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSTaskletManager::DestroyTaskGroup` (engine map; `thiscall`, one stack word, `RET 4`: the
/// address of a task group pointer). A set group whose byte `+4` is clear first gets its own
/// virtual slot `+4` with 0; then the manager's virtual slot `+0x20` takes the group and the
/// pointer is cleared. A null group does nothing.
pub fn bs_tasklet_manager_destroy_task_group(e: &mut Engine, this: Ptr, group: u32) {
    if e.mem.u32(group) == 0 {
        return;
    }
    let g = e.mem.u32(group);
    if g != 0 && e.mem.u8(g + 4) == 0 {
        e.vcall(g, 4, &args![0u32]);
    }
    let g = e.mem.u32(group);
    e.vcall(this.addr(), 0x20, &args![g]);
    e.mem.set_u32(group, 0);
}

/// The KERNEL32 import slots `ReleaseSemaphore` and `InterlockedIncrement`.
const IMPORT_RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00b00df0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; `this` is a task group as
/// `00b027c0` builds it, its semaphore block at `+8`): `ReleaseSemaphore(handle at +0xc, 1,
/// NULL)`, `InterlockedIncrement` of the count at `+8`, the byte `+5` set, then its own virtual
/// slot `+0x24`.
pub fn fn_00b00df0(e: &mut Engine, this: Ptr) {
    let block = this.addr() + 8;
    let handle = e.mem.u32(block + 4);
    e.call(IMPORT_RELEASE_SEMAPHORE, &args![handle, 1u32, 0u32]);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![block]);
    e.mem.set_u8(this.addr() + 5, 1);
    e.vcall(this.addr(), 0x24, &args![]);
}

/// `EnterCriticalSection`, `LeaveCriticalSection` and `WaitForSingleObject` (KERNEL32).
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const IMPORT_WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;

// Translated from 00b020a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; `this` is not read): under the group's
/// lock (+0x14), a group not yet started (byte +0x2C clear) polls its semaphore (+0xC,
/// `WaitForSingleObject` with no wait), is marked started (+0x2C 1), its byte +4 cleared,
/// +0x2D set and +0x2E cleared and its words +0x34 and +0x38 cleared: answers 1; an already
/// started group answers 0.
pub fn fn_00b020a0(e: &mut Engine, _this: Ptr, group: u32) -> u8 {
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![group + 0x14]);
    let started = if e.mem.u8(group + 0x2c) == 0 {
        let semaphore = e.mem.u32(group + 0xc);
        e.call(IMPORT_WAIT_FOR_SINGLE_OBJECT, &args![semaphore, 0u32]);
        e.mem.set_u8(group + 0x2c, 1);
        e.mem.set_u8(group + 4, 0);
        e.mem.set_u8(group + 0x2d, 1);
        e.mem.set_u8(group + 0x2e, 0);
        e.mem.set_u32(group + 0x34, 0);
        e.mem.set_u32(group + 0x38, 0);
        1
    } else {
        0
    };
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![group + 0x14]);
    started
}

/// `BSTaskletManager::QInstance` (`00b00a00`, no arguments), the tasklet callback's vtable
/// (`010a47b8`), and the memory manager's release (`00aa4060`, `thiscall`, one word).
const Q_INSTANCE_00B00A00: u32 = 0x00b0_0a00;
const CALLBACK_VTABLE_010A47B8: u32 = 0x010a_47b8;
const DEALLOCATE_00AA4060: u32 = 0x00aa_4060;

/// Adds `hook` to a hook array while the manager's word at +8 is 0 and the
/// array holds fewer than 16 (signed compare, `JGE`); answers whether it was added.
fn add_hook(e: &mut Engine, count: u32, hooks: u32, hook: u32) -> u8 {
    let manager = e.call(Q_INSTANCE_00B00A00, &args![]).u32();
    if e.mem.u32(manager.wrapping_add(8)) == 0 && e.mem.i32(count) < 0x10 {
        let n = e.mem.u32(count);
        e.mem.set_u32(hooks.wrapping_add(n.wrapping_mul(4)), hook);
        let n = e.mem.u32(count).wrapping_add(1);
        e.mem.set_u32(count, n);
        1
    } else {
        0
    }
}

// Translated from 00b00d50 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word: a hook object): while the manager
/// (`00b00a00`) has 0 in its word at +8 and the enter-hook array holds fewer than
/// 16 (`011f8af0`, signed), the object goes into the array (`011f8af8`) and the count up;
/// answers 1 then, else 0.
pub fn fn_00b00d50(e: &mut Engine, hook: u32) -> u8 {
    add_hook(e, ENTER_COUNT, ENTER_HOOKS, hook)
}

// Translated from 00b00da0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): as [`fn_00b00d50`] for the leave hooks
/// (`011f8af4`, `011f8b38`).
pub fn fn_00b00da0(e: &mut Engine, hook: u32) -> u8 {
    add_hook(e, LEAVE_COUNT, LEAVE_HOOKS, hook)
}

// Translated from 00b00e40 (disassembled, FalloutNV.exe 1.4.0.525)
/// The tasklet callback's constructor (unnamed in the engine map; `thiscall`, one word: 0 for
/// an enter hook, otherwise a leave hook, `RET 4`): stores the vtable `010a47b8` and adds the
/// object to that hook array ([`fn_00b00d50`] or [`fn_00b00da0`]; the answer is kept in a
/// local nobody reads). Answers `this`.
pub fn fn_00b00e40(e: &mut Engine, this: u32, leave: u32) -> u32 {
    e.mem.set_u32(this, CALLBACK_VTABLE_010A47B8);
    if leave == 0 {
        fn_00b00d50(e, this);
    } else {
        fn_00b00da0(e, this);
    }
    this
}

// Translated from 00b00e90 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSTaskletManagerCallback`'s scalar deleting destructor (engine map; `thiscall`, one word:
/// the flags, `RET 4`): stores the vtable `010a47b8`, then with bit 0 of the flags the block
/// is freed (`00aa4060`). Answers `this`.
pub fn bs_tasklet_manager_callback_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.mem.set_u32(this, CALLBACK_VTABLE_010A47B8);
    if flags & 1 != 0 {
        e.call(DEALLOCATE_00AA4060, &args![MEMORY_MANAGER_011F6238, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_0d50, fn_00b00d50(u32) -> u8),
        entry!(0x00b0_0da0, fn_00b00da0(u32) -> u8),
        entry!(0x00b0_0e40, fn_00b00e40(u32, u32) -> u32),
        entry!(
            0x00b0_0e90,
            bs_tasklet_manager_callback_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(0x00b0_0a80, bs_tasklet_manager_create_task_group(Ptr, u32) -> u8),
        entry!(0x00b0_2010, fn_00b02010(Ptr, u32) -> u8),
        entry!(0x00b0_27c0, fn_00b027c0(Ptr) -> Ptr),
        entry!(0x00b0_0c90, fn_00b00c90(Ptr)),
        entry!(0x00b0_0cf0, fn_00b00cf0(Ptr)),
        entry!(0x00b0_0ae0, fn_00b00ae0(Ptr, u32) -> u8),
        entry!(0x00b0_20a0, fn_00b020a0(Ptr, u32) -> u8),
        entry!(0x00b0_0b40, fn_00b00b40(Ptr, u32, u32, u8) -> u8),
        entry!(0x00b0_0bc0, fn_00b00bc0(Ptr, u32) -> u8),
        entry!(0x00b0_0c20, bs_tasklet_manager_destroy_task_group(Ptr, u32)),
        entry!(0x00b0_0df0, fn_00b00df0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_running_manager_makes_the_group_through_slot_0x10() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x100);
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0x00ab_0000]);
        e.mem.set_u32(0x1000, 0x0300_0000);
        e.register(0x00ab_0000, |e, a| {
            e.mem.set_u32(a[1], 0x4444);
            1u32.into_ret()
        });
        e.mem.set_u32(0x1080, 0xffff_ffff);
        assert_eq!(
            e.call(0x00b0_0a80, &args![0x1000u32, 0x1080u32]).u32() & 0xff,
            0
        );
        assert_eq!(e.mem.u32(0x1080), 0);
        e.mem.set_u8(0x1004, 1);
        assert_eq!(
            e.call(0x00b0_0a80, &args![0x1000u32, 0x1080u32]).u32() & 0xff,
            1
        );
        assert_eq!(e.mem.u32(0x1080), 0x4444);
    }
}

#[cfg(test)]
mod hook_tests {
    use super::*;

    /// The set objects among the first `count` get their slot `+4`; the count is read signed.
    #[test]
    fn the_hooks_run_their_slot_4() {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        e.map(0x0000_1000, 0x100);
        e.put_vtable(0x0300_0000, &[0, 0x00ab_0000]);
        e.mem.set_u32(0x1000, 0x0300_0000);
        e.mem.set_u32(0x1010, 0x0300_0000);
        e.register(0x00ab_0000, |e, a| {
            let n = e.mem.u32(0x1080);
            e.mem.set_u32(0x1084 + 4 * n, a[0]);
            e.mem.set_u32(0x1080, n + 1);
            Ret::default()
        });
        e.mem.set_u32(ENTER_COUNT, 3);
        e.mem.set_u32(ENTER_HOOKS, 0x1000);
        e.mem.set_u32(ENTER_HOOKS + 8, 0x1010);
        e.call(0x00b0_0c90, &args![0u32]);
        assert_eq!(e.mem.u32(0x1080), 2);
        assert_eq!([e.mem.u32(0x1084), e.mem.u32(0x1088)], [0x1000, 0x1010]);
        // A negative count runs nothing.
        e.mem.set_u32(LEAVE_COUNT, 0xffff_ffff);
        e.mem.set_u32(LEAVE_HOOKS, 0x1000);
        e.call(0x00b0_0cf0, &args![0u32]);
        assert_eq!(e.mem.u32(0x1080), 2);
    }

    /// The group is made in the manager's memory and constructed; without memory `*out` is null.
    #[test]
    fn the_task_group_is_allocated_and_constructed() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x3c);
        let given = std::rc::Rc::new(std::cell::Cell::new(block));
        let g = given.clone();
        e.register_double(ALLOCATE_00AA3E40, move |_, a| {
            assert_eq!(&a[..2], &[MEMORY_MANAGER_011F6238, 0x3c]);
            g.get().into_ret()
        });
        e.register(IMPORT_CREATE_SEMAPHORE_A, |_, a| {
            assert_eq!(a, &[0, 0, 1, 0]);
            0x77u32.into_ret()
        });
        e.register(IMPORT_INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00b0_2010, &args![0u32, out]).u8(), 1);
        assert_eq!(e.mem.u32(out), block);
        assert_eq!(e.mem.u32(block), GROUP_VTABLE_010A48D8);
        assert_eq!(
            (
                e.mem.u8(block + 4),
                e.mem.u32(block + 0xc),
                e.mem.u32(block + 0x10)
            ),
            (1, 0x77, 1)
        );
        given.set(0);
        assert_eq!(e.call(0x00b0_2010, &args![0u32, out]).u8(), 0);
        assert_eq!(e.mem.u32(out), 0);
    }

    /// A manager at `0x1000` (running byte `+4` as given) whose vtable at `0x0300_0000` has the
    /// recording double `0x00ab_0000` in slots `+0x14` to `+0x24`; the double logs (slot, words)
    /// at `0x1100` and answers 1.
    fn manager(running: u8) -> Engine {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x200);
        e.put_vtable(
            0x0300_0000,
            &[
                0,
                0,
                0,
                0,
                0,
                0x00ab_0014,
                0x00ab_0018,
                0x00ab_001c,
                0x00ab_0020,
                0x00ab_0024,
            ],
        );
        e.mem.set_u32(0x1000, 0x0300_0000);
        e.mem.set_u8(0x1004, running);
        for slot in [0x14u32, 0x18, 0x1c, 0x20, 0x24] {
            e.register(0x00ab_0000 + slot, |_, _| 1u32.into_ret());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn the_manager_slots_run_only_while_it_runs_with_a_group() {
        let mut e = manager(1);
        e.mem.set_u32(0x1080, 0x4444);
        e.mem.set_u32(0x1090, 0x5555);
        assert_eq!(e.call(0x00b0_0ae0, &args![0x1000u32, 0x1080u32]).u8(), 1);
        assert_eq!(e.call(0x00b0_0bc0, &args![0x1000u32, 0x1080u32]).u8(), 1);
        assert_eq!(
            e.call(
                0x00b0_0b40,
                &args![0x1000u32, 0x1080u32, 0x108cu32, 0x1ffu32]
            )
            .u8(),
            1
        );
        assert_eq!(calls(&e, 0x00ab_0014), vec![vec![0x1000, 0x4444]]);
        assert_eq!(calls(&e, 0x00ab_001c), vec![vec![0x1000, 0x4444]]);
        // The byte argument goes on zero-extended.
        assert_eq!(
            calls(&e, 0x00ab_0018),
            vec![vec![0x1000, 0x4444, 0x5555, 0xff]]
        );
        // No task, no group, or a stopped manager: 0 without a call.
        e.mem.set_u32(0x1090, 0);
        assert_eq!(
            e.call(0x00b0_0b40, &args![0x1000u32, 0x1080u32, 0x108cu32, 1u32])
                .u8(),
            0
        );
        e.mem.set_u32(0x1080, 0);
        assert_eq!(e.call(0x00b0_0ae0, &args![0x1000u32, 0x1080u32]).u8(), 0);
        assert_eq!(e.call(0x00b0_0bc0, &args![0x1000u32, 0x1080u32]).u8(), 0);
        let mut stopped = manager(0);
        stopped.mem.set_u32(0x1080, 0x4444);
        stopped.mem.set_u32(0x1090, 0x5555);
        assert_eq!(
            stopped.call(0x00b0_0ae0, &args![0x1000u32, 0x1080u32]).u8(),
            0
        );
        assert_eq!(
            stopped.call(0x00b0_0bc0, &args![0x1000u32, 0x1080u32]).u8(),
            0
        );
        assert_eq!(
            stopped
                .call(0x00b0_0b40, &args![0x1000u32, 0x1080u32, 0x108cu32, 1u32])
                .u8(),
            0
        );
        assert_eq!(calls(&e, 0x00ab_0014).len(), 1);
        for slot in [0x14u32, 0x18, 0x1c] {
            assert!(calls(&stopped, 0x00ab_0000 + slot).is_empty());
        }
    }

    #[test]
    fn destroy_task_group_releases_an_idle_group_and_clears_the_pointer() {
        let mut e = manager(0);
        // A group at 0x1040 with its own vtable at 0x0300_0100 (slot +4 a double).
        e.put_vtable(0x0300_0100, &[0, 0x00ab_0104]);
        e.register(0x00ab_0104, |_, _| Ret::default());
        e.mem.set_u32(0x1040, 0x0300_0100);
        e.mem.set_u32(0x1080, 0x1040);
        e.call(0x00b0_0c20, &args![0x1000u32, 0x1080u32]);
        assert_eq!(calls(&e, 0x00ab_0104), vec![vec![0x1040, 0]]);
        assert_eq!(calls(&e, 0x00ab_0020), vec![vec![0x1000, 0x1040]]);
        assert_eq!(e.mem.u32(0x1080), 0);
        // A busy group (byte +4 set) only goes to the manager; a null one does nothing.
        e.mem.set_u8(0x1044, 1);
        e.mem.set_u32(0x1080, 0x1040);
        e.call(0x00b0_0c20, &args![0x1000u32, 0x1080u32]);
        assert_eq!(calls(&e, 0x00ab_0104).len(), 1);
        assert_eq!(calls(&e, 0x00ab_0020).len(), 2);
        e.call(0x00b0_0c20, &args![0x1000u32, 0x1080u32]);
        assert_eq!(calls(&e, 0x00ab_0020).len(), 2);
    }

    #[test]
    fn the_group_signal_releases_counts_marks_and_calls_slot_0x24() {
        let mut e = manager(0);
        e.register(IMPORT_RELEASE_SEMAPHORE, |_, _| 1u32.into_ret());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.mem.set_u32(0x1008, 2);
        e.mem.set_u32(0x100c, 0x77);
        e.call(0x00b0_0df0, &args![0x1000u32]);
        assert_eq!(calls(&e, IMPORT_RELEASE_SEMAPHORE), vec![vec![0x77, 1, 0]]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_INCREMENT), vec![vec![0x1008]]);
        assert_eq!(e.mem.u32(0x1008), 3);
        assert_eq!(e.mem.u8(0x1005), 1);
        assert_eq!(calls(&e, 0x00ab_0024), vec![vec![0x1000]]);
    }
}

#[cfg(test)]
mod start_tests {
    use super::*;

    #[test]
    fn a_group_starts_once_under_its_lock() {
        let mut e = Engine::new();
        for slot in [
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
            IMPORT_WAIT_FOR_SINGLE_OBJECT,
        ] {
            e.register(slot, |_, _| Ret::default());
        }
        let group = e.mem.alloc(0x40);
        e.mem.set_u32(group + 0xc, 0x99);
        e.mem.set_u8(group + 4, 7);
        e.mem.set_u32(group + 0x34, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_20a0, &args![0u32, group]).u8(), 1);
        assert_eq!(
            [
                e.mem.u8(group + 0x2c),
                e.mem.u8(group + 4),
                e.mem.u8(group + 0x2d)
            ],
            [1, 0, 1]
        );
        assert_eq!(e.mem.u32(group + 0x34), 0);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == IMPORT_WAIT_FOR_SINGLE_OBJECT && c.1 == vec![0x99, 0]));
        assert_eq!(
            e.call(0x00b0_20a0, &args![0u32, group]).u8(),
            0,
            "already started"
        );
    }
}

#[cfg(test)]
mod callback_tests {
    use super::*;

    fn engine(manager: u32) -> Engine {
        let mut e = Engine::new();
        e.map(ENTER_COUNT, 0x88);
        e.register_double(Q_INSTANCE_00B00A00, move |_, _| manager.into_ret());
        e
    }

    #[test]
    fn a_callback_joins_its_hook_array_until_16_or_a_running_manager() {
        let manager = 0x0800_0000;
        let mut e = engine(manager);
        e.mem.map(manager, 0x10);
        let a = e.mem.alloc(4);
        assert_eq!(e.call(0x00b0_0e40, &args![a, 0u32]).u32(), a);
        assert_eq!(e.mem.u32(a), CALLBACK_VTABLE_010A47B8);
        assert_eq!((e.mem.u32(ENTER_COUNT), e.mem.u32(ENTER_HOOKS)), (1, a));
        let b = e.mem.alloc(4);
        fn_00b00e40(&mut e, b, 7);
        assert_eq!((e.mem.u32(LEAVE_COUNT), e.mem.u32(LEAVE_HOOKS)), (1, b));
        // Full at 16 (and a negative count is below 16).
        e.mem.set_u32(ENTER_COUNT, 0x10);
        assert_eq!(e.call(0x00b0_0d50, &args![0x55u32]).u8(), 0);
        e.mem.set_u32(LEAVE_COUNT, 0xffff_ffff);
        e.mem.set_u32(LEAVE_HOOKS - 4, 0);
        assert_eq!(fn_00b00da0(&mut e, 0x66), 1);
        assert_eq!(e.mem.u32(LEAVE_HOOKS - 4), 0x66);
        assert_eq!(e.mem.u32(LEAVE_COUNT), 0);
        // A manager with its word at +8 set takes none.
        e.mem.set_u32(manager + 8, 1);
        e.mem.set_u32(ENTER_COUNT, 0);
        assert_eq!(fn_00b00d50(&mut e, 0x77), 0);
        assert_eq!(e.mem.u32(ENTER_COUNT), 0);
    }

    #[test]
    fn the_callback_deleting_destructor_frees_with_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(DEALLOCATE_00AA4060, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x00b0_0e90, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), CALLBACK_VTABLE_010A47B8);
        assert_eq!(e.call(0x00b0_0e90, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == DEALLOCATE_00AA4060).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, vec![MEMORY_MANAGER_011F6238, this]);
    }
}
