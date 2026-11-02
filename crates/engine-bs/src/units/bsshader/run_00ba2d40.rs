//! `BSShader/run_00ba2d40` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CreateSemaphoreA` (import slot): attributes, initial count, maximum
/// count, name.
const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
/// `CloseHandle` (import slot).
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// `ReleaseSemaphore` (import slot): handle, count, previous count.
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `WaitForSingleObject` (import slot): handle, milliseconds.
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
/// The byte that enables the rendering task threads (`011f94b4`, the one
/// `MTRenderingSystem::CreateThreads` tests).
const THREADS_ENABLED: u32 = 0x011f_94b4;

// Translated from 00ba2d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a semaphore holder's constructor: the word at
/// +0 cleared, +8 set to 0x14 (the maximum count), and +4 the semaphore
/// `CreateSemaphoreA(null, word +0, 0x14, null)` makes. Returns `this`.
pub fn fn_00ba2d40(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    let initial = e.mem.u32(this.addr());
    e.mem.set_u32(this.addr() + 8, 0x14);
    let handle = e
        .call(CREATE_SEMAPHORE_A, &args![0u32, initial, 0x14u32, 0u32])
        .u32();
    e.mem.set_u32(this.addr() + 4, handle);
    this
}

// Translated from 00ba2d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here): the semaphore holder's
/// destructor: `CloseHandle` on the semaphore (+4); returns its `EAX`.
pub fn fn_00ba2d70(e: &mut Engine, this: Ptr) -> u32 {
    let handle = e.mem.u32(this.addr() + 4);
    e.call(CLOSE_HANDLE, &args![handle]).u32()
}

// Translated from 00ba2d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the rendering task threads are enabled
/// (`011f94b4`), stops each of the two thread objects at `this[0]` and
/// `this[1]`: sets its byte +0x48, releases its semaphore (+0x34) once and
/// counts it (+0x30), waits for its handle (+0x40) without a time-out and
/// counts that down (+0x3C), deletes it (vtable slot 0 with 1) and clears
/// the slot.
pub fn fn_00ba2d80(e: &mut Engine, this: Ptr) {
    if e.global::<u8>(THREADS_ENABLED) == 0 {
        return;
    }
    for index in 0..2u32 {
        let slot = this.addr() + 4 * index;
        let thread = e.mem.u32(slot);
        if thread == 0 {
            continue;
        }
        e.mem.set_u8(thread + 0x48, 1);
        let semaphore = e.mem.u32(thread + 0x34);
        e.call(RELEASE_SEMAPHORE, &args![semaphore, 1u32, 0u32]);
        e.call(INTERLOCKED_INCREMENT, &args![thread + 0x30]);
        let thread = e.mem.u32(slot);
        let handle = e.mem.u32(thread + 0x40);
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
        e.call(INTERLOCKED_DECREMENT, &args![thread + 0x3c]);
        let thread = e.mem.u32(slot);
        if thread != 0 {
            e.vcall(thread, 0, &args![1u32]);
        }
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 00ba2d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: a slot of the vtable that holds it at `010b7f60`
/// (`thiscall`, no stack words). When the word at `+0x4c` (a function) is set, jumps to it
/// with `this` and answers its `EAX`; otherwise returns at once, `EAX` holding that null word.
pub fn fn_00ba2d30(e: &mut Engine, this: Ptr) -> u32 {
    let function = e.mem.u32(this.addr() + 0x4c);
    if function == 0 {
        return 0;
    }
    e.call(function, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_2d30, fn_00ba2d30(Ptr) -> u32),
        entry!(0x00ba_2d40, fn_00ba2d40(Ptr) -> Ptr),
        entry!(0x00ba_2d70, fn_00ba2d70(Ptr) -> u32),
        entry!(0x00ba_2d80, fn_00ba2d80(Ptr)),
    ]
}

#[cfg(test)]
mod semaphore_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00ba2d40_creates_the_semaphore() {
        let mut e = Engine::new();
        e.register(CREATE_SEMAPHORE_A, |_, _| Ret {
            eax: 0x44,
            ..Ret::default()
        });
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_2d40, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, CREATE_SEMAPHORE_A), vec![vec![0, 0, 0x14, 0]]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0x44);
        assert_eq!(e.mem.u32(this + 8), 0x14);
    }

    #[test]
    fn fn_00ba2d70_closes_the_semaphore() {
        let mut e = Engine::new();
        e.register(CLOSE_HANDLE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 4, 0x44);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_2d70, &args![this]).u32(), 1);
        assert_eq!(calls_to(&e, CLOSE_HANDLE), vec![vec![0x44]]);
    }

    #[test]
    fn fn_00ba2d80_stops_the_threads_when_enabled() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        for addr in [
            RELEASE_SEMAPHORE,
            INTERLOCKED_INCREMENT,
            INTERLOCKED_DECREMENT,
            WAIT_FOR_SINGLE_OBJECT,
            0x0000_7000,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.put_vtable(0x0f00_0000, &[0x0000_7000]);
        let thread = e.mem.alloc(0x50);
        e.mem.set_u32(thread, 0x0f00_0000);
        e.mem.set_u32(thread + 0x34, 0x51);
        e.mem.set_u32(thread + 0x40, 0x52);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, thread);
        e.call_log = Some(vec![]);
        e.call(0x00ba_2d80, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 4), thread);

        e.set_global(THREADS_ENABLED, 1u8);
        e.call(0x00ba_2d80, &args![this]);
        assert_eq!(e.mem.u8(thread + 0x48), 1);
        assert_eq!(calls_to(&e, RELEASE_SEMAPHORE), vec![vec![0x51, 1, 0]]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![thread + 0x30]]
        );
        assert_eq!(
            calls_to(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x52, 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![thread + 0x3c]]
        );
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![thread, 1]]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00ba2d30_jumps_to_the_function_when_set() {
        const FUNCTION: u32 = 0x0e00_5000;
        let mut e = Engine::new();
        e.register(FUNCTION, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x50);
        assert_eq!(e.call(0x00ba_2d30, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x4c, FUNCTION);
        assert_eq!(e.call(0x00ba_2d30, &args![this]).u32(), this + 1);
    }
}
