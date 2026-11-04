//! `fallout/ai/ailineartaskthread.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `AILinearTaskThread` fields as the code uses them: the task callback
/// +0x4c (`RunLinearTasks`), the thread's Havok memory record +0x50 (its
/// address is what the Havok calls take) and the stack block +0x74.
const TASK_CALLBACK: u32 = 0x4c;
const HAVOK_THREAD: u32 = 0x50;
const STACK_BLOCK: u32 = 0x74;
/// The stack block size.
const STACK_SIZE: u32 = 0x32000;
/// `"AILinearTaskThread"`.
const THREAD_NAME: u32 = 0x0108_55c4;
/// The heap range `008c70c0` fills: begin +0x00, current +0x04, end +0x08.
const RANGE_BEGIN: u32 = 0x00;
const RANGE_CURRENT: u32 = 0x04;
const RANGE_END: u32 = 0x08;

/// Callees by address: `hkMemorySystem::getInstance` (`00cbbfc0`) and its
/// virtual slots `0x0c` (`(record, name, flags)`) and `0x10` (`(record,
/// flags)`); `008c71b0` (`thiscall` on a stack word, given 1) makes the
/// flags word; `hkBaseSystem::initThread` (`00cbb930`, `cdecl`) and
/// `quitThread` (`00cbb990`); `0044edb0` (`thiscall` on the record) gives
/// the record's router, whose slots `0x04` (`(size)`, a block) and `0x08`
/// (`(block, size)`) allocate and free the stack; `0041fd00` and
/// `00984f60` (`thiscall` on the record, one word) take the router;
/// `006815c0` (`thiscall`) returns the stack range object; `00537e90`
/// (`thiscall`, one word) stores a word; `005e0b50` (`cdecl`) adds;
/// `TESDataHandler::CreateThreadSafeFiles` (`00462ee0`, `thiscall` on the
/// data handler at `011c3f2c`).
const MEMORY_SYSTEM_INSTANCE: u32 = 0x00cb_bfc0;
const SLOT_INIT_THREAD: u32 = 0x0c;
const SLOT_QUIT_THREAD: u32 = 0x10;
const MAKE_FLAGS: u32 = 0x008c_71b0;
const BASE_INIT_THREAD: u32 = 0x00cb_b930;
const BASE_QUIT_THREAD: u32 = 0x00cb_b990;
const THREAD_ROUTER: u32 = 0x0044_edb0;
const SLOT_ALLOCATE: u32 = 0x04;
const SLOT_DEALLOCATE: u32 = 0x08;
const SET_ROUTER_A: u32 = 0x0041_fd00;
const SET_ROUTER_B: u32 = 0x0098_4f60;
const STACK_RANGE: u32 = 0x0068_15c0;
const STORE_WORD: u32 = 0x0053_7e90;
const ADD: u32 = 0x005e_0b50;
const DATA_HANDLER: u32 = 0x011c_3f2c;
const CREATE_THREAD_SAFE_FILES: u32 = 0x0046_2ee0;

/// The flags word `008c71b0` builds in a stack slot (the by-value argument
/// the memory system's slots take).
fn flags_word(e: &mut Engine) -> u32 {
    e.with_stack(4, |e, slot| {
        e.call(MAKE_FLAGS, &args![slot, 1u32]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 008c6ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AILinearTaskThread::OnStartup` (Xbox PDB): registers the thread with
/// Havok's memory system (slot `0x0c` with the record at +0x50, the name
/// `"AILinearTaskThread"` and the flags word `008c71b0` makes from 1),
/// `hkBaseSystem::initThread`, hands the record's router to `0041fd00` and
/// `00984f60`, allocates a 0x32000-byte stack block through the router
/// (slot `0x04`, kept at +0x74) and sets it as the stack range
/// (`006815c0`, `008c70c0`), then `TESDataHandler::CreateThreadSafeFiles`.
pub fn ai_linear_task_thread_on_startup(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let record = this.wrapping_add(HAVOK_THREAD);
    let memory = e.call(MEMORY_SYSTEM_INSTANCE, &args![]).u32();
    let flags = flags_word(e);
    e.vcall(memory, SLOT_INIT_THREAD, &args![record, THREAD_NAME, flags]);
    e.call(BASE_INIT_THREAD, &args![record]);
    let router = e.call(THREAD_ROUTER, &args![record]).u32();
    e.call(SET_ROUTER_A, &args![record, router]);
    let router = e.call(THREAD_ROUTER, &args![record]).u32();
    e.call(SET_ROUTER_B, &args![record, router]);
    let router = e.call(THREAD_ROUTER, &args![record]).u32();
    let block = e.vcall(router, SLOT_ALLOCATE, &args![STACK_SIZE]).u32();
    e.mem.set_u32(this.wrapping_add(STACK_BLOCK), block);
    let block = e.mem.u32(this.wrapping_add(STACK_BLOCK));
    let range = e.call(STACK_RANGE, &args![record]).u32();
    fn_008c70c0(e, Ptr::new(range), block, STACK_SIZE);
    let data_handler = e.mem.u32(DATA_HANDLER);
    e.call(CREATE_THREAD_SAFE_FILES, &args![data_handler]);
}

// Translated from 008c70c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `ailineartaskthread.cpp`): sets a range
/// to the block at `base` of `size` bytes: current (+4) and begin (+0) to
/// `base`, end (+8) to `base + size` (`005e0b50`), each stored by
/// `00537e90`.
pub fn fn_008c70c0(e: &mut Engine, this: Ptr, base: u32, size: u32) {
    let range = this.addr();
    e.call(STORE_WORD, &args![range.wrapping_add(RANGE_CURRENT), base]);
    e.call(STORE_WORD, &args![range.wrapping_add(RANGE_BEGIN), base]);
    let end = e.call(ADD, &args![base, size]).u32();
    e.call(STORE_WORD, &args![range.wrapping_add(RANGE_END), end]);
}

// Translated from 008c7110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AILinearTaskThread::OnShutdown` (Xbox PDB): frees the stack block
/// through the router (slot `0x08`, the block and 0x32000),
/// `hkBaseSystem::quitThread`, unregisters the thread from the memory
/// system (slot `0x10` with the record and the flags word from 1), and
/// clears the stack range (`008c70c0` with 0 and 0).
pub fn ai_linear_task_thread_on_shutdown(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let record = this.wrapping_add(HAVOK_THREAD);
    let router = e.call(THREAD_ROUTER, &args![record]).u32();
    let block = e.mem.u32(this.wrapping_add(STACK_BLOCK));
    e.vcall(router, SLOT_DEALLOCATE, &args![block, STACK_SIZE]);
    e.call(BASE_QUIT_THREAD, &args![]);
    let memory = e.call(MEMORY_SYSTEM_INSTANCE, &args![]).u32();
    let flags = flags_word(e);
    e.vcall(memory, SLOT_QUIT_THREAD, &args![record, flags]);
    let range = e.call(STACK_RANGE, &args![record]).u32();
    fn_008c70c0(e, Ptr::new(range), 0, 0);
}

// Translated from 008c7190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AILinearTaskThread::RunLinearTasks` (Xbox PDB): calls the task callback
/// at +0x4c (with `this` in `ECX`) when there is one.
pub fn ai_linear_task_thread_run_linear_tasks(e: &mut Engine, this: Ptr) {
    let callback = e.mem.u32(this.addr().wrapping_add(TASK_CALLBACK));
    if callback != 0 {
        e.call(callback, &args![this]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c_6ff0, ai_linear_task_thread_on_startup(Ptr)),
        entry!(0x008c_70c0, fn_008c70c0(Ptr, u32, u32)),
        entry!(0x008c_7110, ai_linear_task_thread_on_shutdown(Ptr)),
        entry!(0x008c_7190, ai_linear_task_thread_run_linear_tasks(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const INIT_THREAD: u32 = 0x00ee_0100;
    const QUIT_THREAD: u32 = 0x00ee_0200;
    const ALLOCATE: u32 = 0x00ee_0300;
    const DEALLOCATE: u32 = 0x00ee_0400;
    const CALLBACK: u32 = 0x00ee_0500;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    /// A thread object with the memory system, its router and the range
    /// object doubled.
    fn thread() -> (Engine, Log, u32, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        let vtable = e.mem.alloc(0x20);
        for (slot, target) in [
            (SLOT_INIT_THREAD, INIT_THREAD),
            (SLOT_QUIT_THREAD, QUIT_THREAD),
            (SLOT_ALLOCATE, ALLOCATE),
            (SLOT_DEALLOCATE, DEALLOCATE),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        let system = e.mem.alloc(4);
        e.mem.set_u32(system, vtable);
        let range = e.mem.alloc(12);
        logged(&mut e, &log, MEMORY_SYSTEM_INSTANCE, system);
        logged(&mut e, &log, THREAD_ROUTER, system);
        logged(&mut e, &log, STACK_RANGE, range);
        e.register(MAKE_FLAGS, |e, a| {
            e.mem.set_u32(a[0], 0x50 + a[1]);
            Ret::default()
        });
        for callee in [
            INIT_THREAD,
            QUIT_THREAD,
            DEALLOCATE,
            BASE_INIT_THREAD,
            BASE_QUIT_THREAD,
            SET_ROUTER_A,
            SET_ROUTER_B,
            CREATE_THREAD_SAFE_FILES,
        ] {
            logged(&mut e, &log, callee, 0);
        }
        logged(&mut e, &log, ALLOCATE, 0x0090_0000);
        e.register(STORE_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(ADD, |_, a| a[0].wrapping_add(a[1]).into_ret());
        e.map(DATA_HANDLER, 4);
        e.mem.set_u32(DATA_HANDLER, 0x3f2c);
        let this = e.mem.alloc(0x80);
        (e, log, this, system, range)
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn on_startup_registers_the_thread_and_its_stack() {
        let (mut e, log, this, system, range) = thread();
        e.call(0x008c_6ff0, &args![this]);
        let record = this + HAVOK_THREAD;
        assert_eq!(
            calls(&log, INIT_THREAD),
            [vec![system, record, THREAD_NAME, 0x51]]
        );
        assert_eq!(calls(&log, BASE_INIT_THREAD), [vec![record]]);
        assert_eq!(calls(&log, SET_ROUTER_A), [vec![record, system]]);
        assert_eq!(calls(&log, SET_ROUTER_B), [vec![record, system]]);
        assert_eq!(calls(&log, ALLOCATE), [vec![system, STACK_SIZE]]);
        assert_eq!(e.mem.u32(this + STACK_BLOCK), 0x0090_0000);
        assert_eq!(
            (e.mem.u32(range), e.mem.u32(range + 4), e.mem.u32(range + 8)),
            (0x0090_0000, 0x0090_0000, 0x0093_2000)
        );
        assert_eq!(calls(&log, CREATE_THREAD_SAFE_FILES), [vec![0x3f2c]]);
    }

    #[test]
    fn on_shutdown_frees_the_stack_and_unregisters() {
        let (mut e, log, this, system, range) = thread();
        e.mem.set_u32(this + STACK_BLOCK, 0x0090_0000);
        e.mem.set_u32(range + 8, 7);
        e.call(0x008c_7110, &args![this]);
        let record = this + HAVOK_THREAD;
        assert_eq!(
            calls(&log, DEALLOCATE),
            [vec![system, 0x0090_0000, STACK_SIZE]]
        );
        assert_eq!(calls(&log, BASE_QUIT_THREAD), [vec![]]);
        assert_eq!(calls(&log, QUIT_THREAD), [vec![system, record, 0x51]]);
        assert_eq!(e.mem.bytes(range, 12), vec![0; 12]);
    }

    #[test]
    fn run_linear_tasks_calls_the_callback_when_set() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, CALLBACK, 0);
        let this = e.mem.alloc(0x80);
        e.call(0x008c_7190, &args![this]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(this + TASK_CALLBACK, CALLBACK);
        e.call(0x008c_7190, &args![this]);
        assert_eq!(*log.borrow(), [(CALLBACK, vec![this])]);
    }
}
