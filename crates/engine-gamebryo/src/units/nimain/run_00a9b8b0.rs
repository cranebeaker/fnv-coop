//! `NiMain/run_00a9b8b0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a9b8b0` (no stack words): the first start-up step `00a9ba80` runs.
const FIRST_STEP: u32 = 0x00a9_b8b0;
/// `00a9b9b0` (no stack words): the step `00a9ba80` ends with, by a jump.
const SECOND_STEP: u32 = 0x00a9_b9b0;

// Translated from 00a9ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; run by the start-up
/// `00a89cc0`, no arguments): runs `00a9b8b0`, then jumps to `00a9b9b0` (the exe leaves
/// that function's result in `EAX`; neither is read here).
pub fn fn_00a9ba80(e: &mut Engine) {
    e.call(FIRST_STEP, &args![]);
    e.call(SECOND_STEP, &args![]);
}

// Translated from 00a9b8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; no arguments): fills the
/// dword table at `011f5fc4` with 0 to 13 (14 words), the byte table at `011f5fb4` with 0 to
/// 13 (14 bytes) and sets the byte at `011f6000` to 1.
pub fn fn_00a9b8b0(e: &mut Engine) {
    for index in 0..14u32 {
        e.mem.set_u32(0x011f_5fc4 + 4 * index, index);
    }
    for index in 0..14u32 {
        e.mem.set_u8(0x011f_5fb4 + index, index as u8);
    }
    e.mem.set_u8(0x011f_6000, 1);
}

// Translated from 00a9b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; no arguments): fills the
/// word table at `011f5f7c` with `0x100 * i` for i = 0 to 8 (9 words), the dword table at
/// `011f5f90` with 0 to 8 (9 dwords) and sets the byte at `011f6001` to 1.
pub fn fn_00a9b9b0(e: &mut Engine) {
    for index in 0..9u32 {
        e.mem
            .set_u16(0x011f_5f7c + 2 * index, (index * 0x100) as u16);
    }
    for index in 0..9u32 {
        e.mem.set_u32(0x011f_5f90 + 4 * index, index);
    }
    e.mem.set_u8(0x011f_6001, 1);
}

/// The critical section `00a9bb60` holds (`011f600c`).
const SEQUENCE_LOCK: u32 = 0x011f_600c;
/// Import slots of `EnterCriticalSection` and `LeaveCriticalSection` (`stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The global `NiTArray` `00a9bb60` empties (`011ac040`; its 16-bit used size at +0xa).
const SEQUENCE_ARRAY: u32 = 0x011a_c040;
/// `00a9ba90` (`thiscall` on the array, `RET 8`: the index and the address of a word).
const REMOVE_AT: u32 = 0x00a9_ba90;
/// `00a47770` (`thiscall`, `RET 4`; engine map
/// `NiTArray<NiPointer<NiControllerSequence>,...>::SetSize`).
const ARRAY_SET_SIZE: u32 = 0x00a4_7770;

// Translated from 00a9bb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): inside the critical section `011f600c`, calls `00a9ba90` on the array at
/// `011ac040` for each index below its used size (read again after each call), passing the
/// address of one stack word set to 0 once before the loop, then gives the array size 0
/// (`00a47770`).
pub fn fn_00a9bb60(e: &mut Engine) {
    e.call(ENTER_CRITICAL_SECTION, &args![SEQUENCE_LOCK]);
    e.with_stack(4, |e, word| {
        let mut index = 0u32;
        if e.mem.u16(SEQUENCE_ARRAY + 0xa) > 0 {
            e.mem.set_u32(word.addr(), 0);
            loop {
                e.call(REMOVE_AT, &args![SEQUENCE_ARRAY, index, word]);
                index = index.wrapping_add(1);
                if index >= e.mem.u16(SEQUENCE_ARRAY + 0xa) as u32 {
                    break;
                }
            }
        }
    });
    e.call(ARRAY_SET_SIZE, &args![SEQUENCE_ARRAY, 0u32]);
    e.call(LEAVE_CRITICAL_SECTION, &args![SEQUENCE_LOCK]);
}

/// The function-local static `00a9ba90` compares against (a null counted pointer at
/// `011f6004`), its guard word (bit 0 at `011f6008`) and the destructor given to `_atexit`
/// (`00ec658f`, `cdecl`) when it is first set (`00fdb230`).
const NULL_POINTER: u32 = 0x011f_6004;
const NULL_POINTER_GUARD: u32 = 0x011f_6008;
const ATEXIT: u32 = 0x00ec_658f;
const NULL_POINTER_DESTRUCTOR: u32 = 0x00fd_b230;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement` (`stdcall`, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiTArray<NiPointer<NiMetricsOutputModule>,...>`'s vtable (its RTTI locator names that
/// class), `00a47600` (`cdecl`, the array's data: its release) and the memory manager's
/// deallocate (`00aa4060`, `thiscall` on `011f6238`).
const METRICS_ARRAY_VTABLE: u32 = 0x010a_1458;
const ARRAY_FREE: u32 = 0x00a4_7600;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00a9ba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiTArray` of counted pointers: an index and
/// the address of a pointer; `RET 8`; `00a9bb60` calls it to empty its array): the array's
/// `SetAt` by its body. Its function-local static null pointer (`011f6004`) is set to 0 on
/// the first call (guard bit 0 of `011f6008`, destructor `00fdb230` given to `_atexit`).
/// An index at or past the used size (+0xa, 16 bits) makes the used size `index + 1` and
/// counts a non-null pointer in the element count (+0xc, 16 bits); inside it, the element
/// count goes up when a null element gets a pointer and down when a pointer is replaced by
/// null. Then the element (data at +4) is assigned as a counted pointer: the old one
/// released (its virtual `+4` when its count at +4 reaches 0), the new one stored and
/// counted.
pub fn fn_00a9ba90(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    let base = this.addr();
    if e.mem.u8(NULL_POINTER_GUARD) & 1 == 0 {
        let guard = e.mem.u32(NULL_POINTER_GUARD) | 1;
        e.mem.set_u32(NULL_POINTER_GUARD, guard);
        e.mem.set_u32(NULL_POINTER, 0);
        e.call(ATEXIT, &args![NULL_POINTER_DESTRUCTOR]);
    }
    let value = value.addr();
    if index >= u32::from(e.mem.u16(base + 0xa)) {
        e.mem.set_u16(base + 0xa, index.wrapping_add(1) as u16);
        if e.mem.u32(value) != e.mem.u32(NULL_POINTER) {
            let count = e.mem.u16(base + 0xc).wrapping_add(1);
            e.mem.set_u16(base + 0xc, count);
        }
    } else {
        let null = e.mem.u32(NULL_POINTER);
        let element = e.mem.u32(e.mem.u32(base + 4).wrapping_add(4 * index));
        if e.mem.u32(value) != null {
            if element == null {
                let count = e.mem.u16(base + 0xc).wrapping_add(1);
                e.mem.set_u16(base + 0xc, count);
            }
        } else if element != null {
            let count = e.mem.u16(base + 0xc).wrapping_add(0xffff);
            e.mem.set_u16(base + 0xc, count);
        }
    }
    let slot = e.mem.u32(base + 4).wrapping_add(4 * index);
    let old = e.mem.u32(slot);
    if old == e.mem.u32(value) {
        return;
    }
    if old != 0
        && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(old, 4, &args![]);
    }
    let new = e.mem.u32(value);
    e.mem.set_u32(slot, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
    }
}

// Translated from 00a9bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the flags; `RET 4`): the scalar deleting destructor
/// of `NiTArray<NiPointer<NiMetricsOutputModule>,...>` (its vtable's RTTI): the vtable, the
/// data (+4) released (`00a47600`), then, with bit 0 of the flags, the object given back to
/// the memory manager. Answers `this`.
pub fn fn_00a9bbc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let data = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), METRICS_ARRAY_VTABLE);
    e.call(ARRAY_FREE, &args![data]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_ba80, fn_00a9ba80()),
        entry!(0x00a9_b8b0, fn_00a9b8b0()),
        entry!(0x00a9_b9b0, fn_00a9b9b0()),
        entry!(0x00a9_bb60, fn_00a9bb60()),
        entry!(0x00a9_ba90, fn_00a9ba90(Ptr, u32, Ptr)),
        entry!(0x00a9_bbc0, fn_00a9bbc0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn runs_the_two_steps_in_order() {
        let mut e = Engine::new();
        e.register(FIRST_STEP, |_, _| Ret::default());
        e.register(SECOND_STEP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a9_ba80, &args![]);
        let order: Vec<u32> = e.call_log.unwrap().into_iter().map(|(a, _)| a).collect();
        assert_eq!(order, vec![0x00a9_ba80, FIRST_STEP, SECOND_STEP]);
    }
}

#[cfg(test)]
mod table_tests {
    use super::*;

    #[test]
    fn first_tables_hold_their_counting_values() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x2000);
        e.call(0x00a9_b8b0, &args![]);
        for index in 0..14u32 {
            assert_eq!(e.mem.u32(0x011f_5fc4 + 4 * index), index);
            assert_eq!(e.mem.u8(0x011f_5fb4 + index), index as u8);
        }
        assert_eq!(e.mem.u8(0x011f_6000), 1);
        assert_eq!(e.mem.u32(0x011f_5fc4 + 4 * 14), 0);
    }

    #[test]
    fn second_tables_hold_their_counting_values() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x2000);
        e.call(0x00a9_b9b0, &args![]);
        for index in 0..9u32 {
            assert_eq!(e.mem.u16(0x011f_5f7c + 2 * index), (index * 0x100) as u16);
            assert_eq!(e.mem.u32(0x011f_5f90 + 4 * index), index);
        }
        assert_eq!(e.mem.u16(0x011f_5f7c + 18), 0);
        assert_eq!(e.mem.u8(0x011f_6001), 1);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a9bb60_removes_every_index_inside_the_lock() {
        let mut e = Engine::new();
        e.map(SEQUENCE_ARRAY & !0xfff, 0x1000);
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        // Each removal takes the last element away, as a shrinking array would; the
        // word passed stays 0 until the double changes it.
        e.register(REMOVE_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), if a[1] == 0 { 0 } else { 9 });
            e.mem.set_u32(a[2], 9);
            Ret::default()
        });
        e.mem.set_u16(SEQUENCE_ARRAY + 0xa, 3);
        e.call_log = Some(vec![]);
        e.call(0x00a9_bb60, &args![]);
        let log = e.call_log.take().unwrap();
        let indices: Vec<u32> = log
            .iter()
            .filter(|call| call.0 == REMOVE_AT)
            .map(|call| call.1[1])
            .collect();
        assert_eq!(indices, [0, 1, 2]);
        assert_eq!(log[1], (ENTER_CRITICAL_SECTION, vec![SEQUENCE_LOCK]));
        assert_eq!(
            log[log.len() - 2],
            (ARRAY_SET_SIZE, vec![SEQUENCE_ARRAY, 0])
        );
        assert_eq!(
            log[log.len() - 1],
            (LEAVE_CRITICAL_SECTION, vec![SEQUENCE_LOCK])
        );

        // An empty array: no removal.
        e.mem.set_u16(SEQUENCE_ARRAY + 0xa, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a9_bb60, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != REMOVE_AT));
        assert_eq!(log.len(), 4);
    }
}

#[cfg(test)]
mod set_at_tests {
    use super::*;

    const RELEASE: u32 = 0x0300_0004;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.register(ATEXIT, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.put_vtable(0x0300_0000, &[0, RELEASE]);
        e.register(RELEASE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0300_0000);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn set_at_counts_and_assigns() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, data);
        let first = object(&mut e, 1);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, first);
        // Past the used size: it grows and counts the pointer.
        e.call(0x00a9_ba90, &args![array, 2u32, value]);
        assert_eq!(calls(&e, ATEXIT), vec![vec![NULL_POINTER_DESTRUCTOR]]);
        assert_eq!(e.mem.u32(NULL_POINTER_GUARD), 1);
        assert_eq!((e.mem.u16(array + 0xa), e.mem.u16(array + 0xc)), (3, 1));
        assert_eq!(e.mem.u32(data + 8), first);
        assert_eq!(e.mem.u32(first + 4), 2);
        // The same pointer again: nothing changes but the count check.
        e.call(0x00a9_ba90, &args![array, 2u32, value]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(e.mem.u32(first + 4), 2);
        assert_eq!(calls(&e, ATEXIT).len(), 1);
        // Null over it: the count goes down, the old pointer is released.
        e.mem.set_u32(value, 0);
        e.mem.set_u32(first + 4, 1);
        e.call(0x00a9_ba90, &args![array, 2u32, value]);
        assert_eq!(e.mem.u16(array + 0xc), 0);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(calls(&e, RELEASE), vec![vec![first]]);
        // A pointer into a null element inside the size: the count goes up.
        let second = object(&mut e, 5);
        e.mem.set_u32(value, second);
        e.call(0x00a9_ba90, &args![array, 0u32, value]);
        assert_eq!((e.mem.u16(array + 0xa), e.mem.u16(array + 0xc)), (3, 1));
        assert_eq!(e.mem.u32(second + 4), 6);
        // A pointer over a pointer: the count stays.
        let third = object(&mut e, 0);
        e.mem.set_u32(value, third);
        e.call(0x00a9_ba90, &args![array, 0u32, value]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(e.mem.u32(second + 4), 5);
        assert_eq!(e.mem.u32(data), third);
    }

    #[test]
    fn deleting_destructor_frees_the_data_and_on_bit_0_the_array() {
        let mut e = engine();
        e.register(ARRAY_FREE, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x4444);
        assert_eq!(e.call(0x00a9_bbc0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), METRICS_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_FREE), vec![vec![0x4444]]);
        assert!(calls(&e, DEALLOCATE).is_empty());
        e.call(0x00a9_bbc0, &args![this, 1u32]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, this]]);
    }
}
