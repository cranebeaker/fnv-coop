//! `BSSystem/run_00b027c0` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable `00b029a0` stores (`010a48e8`).
const VTABLE_010A48E8: u32 = 0x010a_48e8;

// Translated from 00b029a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, no stack words): a destructor body that only stores
/// the vtable `010a48e8` at +0 (EAX is left holding `this`).
pub fn fn_00b029a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_010A48E8);
    this
}

/// The task group's vtable (`010a48d8`), the import slots of KERNEL32
/// `DeleteCriticalSection`, `CloseHandle`, `WaitForSingleObject` and `InterlockedDecrement`,
/// the memory manager (`011f6238`) and its release (`00aa4060`, `thiscall`, one word).
const GROUP_VTABLE: u32 = 0x010a_48d8;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00b02860 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b02860 - the SEH frame (FS:[0] handler 00f23946, states 2/3/1/0/-1 between the member destructors) is not translated.
/// The task group's destructor body (unnamed in the engine map; `thiscall`, plain `RET`): the
/// vtable `010a48d8`; through a local pointer to `this`, when `this` is not null and its byte
/// at +4 is clear, its own vtable slot +4 with 0 (`thiscall`, one word). Then the critical
/// section at +0x14 is deleted, the handle at +0xc closed, and the vtable `010a48e8` stored.
pub fn fn_00b02860(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, GROUP_VTABLE);
    if this != 0 && e.mem.u8(this.wrapping_add(4)) == 0 {
        e.vcall(this, 0x4, &args![0u32]);
    }
    e.call(DELETE_CRITICAL_SECTION, &args![this.wrapping_add(0x14)]);
    let handle = e.mem.u32(this.wrapping_add(0xc));
    e.call(CLOSE_HANDLE, &args![handle]);
    e.mem.set_u32(this, VTABLE_010A48E8);
}

// Translated from 00b02920 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word it does not read, `RET 4`): when the words
/// at +0x38 and +0x34 differ, waits on the handle at +0xc (`WaitForSingleObject`, `INFINITE`)
/// and counts the word at +8 down (`InterlockedDecrement`). Then the byte at +4 is set and the
/// byte at +0x2c cleared.
pub fn fn_00b02920(e: &mut Engine, this: u32, _unused: u32) {
    if e.mem.u32(this.wrapping_add(0x38)) != e.mem.u32(this.wrapping_add(0x34)) {
        let counter = this.wrapping_add(8);
        let handle = e.mem.u32(counter.wrapping_add(4));
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
        e.call(INTERLOCKED_DECREMENT, &args![counter]);
    }
    e.mem.set_u8(this.wrapping_add(4), 1);
    e.mem.set_u8(this.wrapping_add(0x2c), 0);
}

// Translated from 00b02970 (disassembled, FalloutNV.exe 1.4.0.525)
/// The task group's scalar deleting destructor (unnamed in the engine map; `thiscall`, one
/// word: the flags, `RET 4`): [`fn_00b02860`], then with bit 0 of the flags the block is freed
/// (`00aa4060`). Answers `this`.
pub fn fn_00b02970(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_00b02860(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_29a0, fn_00b029a0(Ptr) -> Ptr),
        entry!(0x00b0_2860, fn_00b02860(u32)),
        entry!(0x00b0_2920, fn_00b02920(u32, u32)),
        entry!(0x00b0_2970, fn_00b02970(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    #[test]
    fn stores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 5);
        assert_eq!(e.call(0x00b0_29a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010A48E8);
        assert_eq!(e.mem.u32(this + 4), 5);
    }
}

#[cfg(test)]
mod group_tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for slot in [
            DELETE_CRITICAL_SECTION,
            CLOSE_HANDLE,
            WAIT_FOR_SINGLE_OBJECT,
            INTERLOCKED_DECREMENT,
            DEALLOCATE,
        ] {
            e.register(slot, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn the_destructor_stops_an_unstopped_group_then_releases_it() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x88);
        // The game's vtable 010a48d8, slot +4 the stop.
        e.put_vtable(GROUP_VTABLE, &[0, 0x0a00_0000]);
        e.register(0x0a00_0000, |e, a| {
            assert_eq!(e.mem.u32(a[0]), GROUP_VTABLE);
            Ret::default()
        });
        assert_eq!(e.call(0x00b0_2970, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, 0x0a00_0000), vec![vec![this, 0]]);
        assert_eq!(calls(&e, DELETE_CRITICAL_SECTION), vec![vec![this + 0x14]]);
        assert_eq!(calls(&e, CLOSE_HANDLE), vec![vec![0x88]]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, this]]);
        assert_eq!(e.mem.u32(this), VTABLE_010A48E8);
        // A stopped group (byte +4 set) is not stopped again; no free without bit 0.
        e.mem.set_u8(this + 4, 1);
        e.call_log = Some(vec![]);
        fn_00b02970(&mut e, this, 0);
        assert!(calls(&e, 0x0a00_0000).is_empty());
        assert!(calls(&e, DEALLOCATE).is_empty());
        assert_eq!(calls(&e, CLOSE_HANDLE).len(), 1);
    }

    #[test]
    fn the_stop_waits_only_for_unfinished_work() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x99);
        e.mem.set_u8(this + 0x2c, 1);
        e.mem.set_u32(this + 0x34, 2);
        e.mem.set_u32(this + 0x38, 2);
        e.call(0x00b0_2920, &args![this, 0u32]);
        assert!(calls(&e, WAIT_FOR_SINGLE_OBJECT).is_empty());
        assert_eq!((e.mem.u8(this + 4), e.mem.u8(this + 0x2c)), (1, 0));
        e.mem.set_u32(this + 0x38, 3);
        fn_00b02920(&mut e, this, 0);
        assert_eq!(
            calls(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x99, 0xffff_ffff]]
        );
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![this + 8]]);
    }
}
