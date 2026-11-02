//! `(unplaced)/run_00b9f0c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedDecrement` (kernel32 import slot `00fdf1c4`).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The vtable `010b7eb8` the destructor stores first.
const OWN_VTABLE: u32 = 0x010b_7eb8;
/// `00b9ef30` (`thiscall`): detaches the lights and empties the lists at
/// `+0xe0` and `+0x12c`.
const DETACH_LIGHTS: u32 = 0x00b9_ef30;
/// `00b6d4c0` (`thiscall` on the object at `011f91a8`, one word): called with
/// the pointer at `+0x10c` before it is released.
const RELEASE_HOOK: u32 = 0x00b6_d4c0;
const RELEASE_HOOK_OWNER: u32 = 0x011f_91a8;
/// `00e74d40` (`thiscall`; engine map: `NiTPointerListBase<..>::RemoveAll`).
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// The vtables the three arrays get before their destructor bodies, and the
/// words holding those vtables' slot `+8` (called through memory).
const NODE_ARRAY_VTABLE: u32 = 0x0101_a8e8;
const NODE_ARRAY_FREE_SLOT: u32 = 0x0101_a8f0;
const ARRAY_VTABLE: u32 = 0x010b_7ea8;
const ARRAY_FREE_SLOT: u32 = 0x010b_7eb0;
/// The base vtable `0101dce4` (RTTI `.?AVNiRefObject@@`) and the object count
/// it decrements (`011f4400`).
const REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT: u32 = 0x011f_4400;
/// The virtual slot a reference-counted object's last release calls.
const SLOT_DELETE_THIS: u32 = 0x4;
/// The virtual slot of an array that frees its data (`+8`).
const SLOT_ARRAY_FREE: u32 = 0x8;

/// Releases the reference-counted object at `object` (if any): its count at
/// `+4` is decremented and virtual slot `+4` runs when it reaches zero.
fn release(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &[]);
    }
}

/// Releases the pointer at `this + offset` and clears it.
fn release_and_clear(e: &mut Engine, this: Ptr, offset: u32) {
    let slot = this.addr().wrapping_add(offset);
    let object = e.mem.u32(slot);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(slot, 0);
    }
}

/// Clears the array at `array` (vtable, data, two counts): when it has data,
/// `free(array, data)`, then the data word and both counts are zeroed.
fn clear_array(e: &mut Engine, array: u32, free: impl FnOnce(&mut Engine, u32, u32)) {
    let data = e.mem.u32(array.wrapping_add(4));
    if data != 0 {
        free(e, array, data);
        e.mem.set_u32(array.wrapping_add(4), 0);
        e.mem.set_u32(array.wrapping_add(0xc), 0);
        e.mem.set_u32(array.wrapping_add(8), 0);
    }
}

// Translated from 00b9f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b9f0c0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// A destructor (unnamed in the engine map; `thiscall`, plain `RET`):
/// stores the vtable `010b7eb8` and runs `00b9ef30`; releases and clears
/// the pointers at `+0xf0` and `+0xf8`; hands the pointer at `+0x10c` (if
/// any) to `00b6d4c0` on the object at `011f91a8`, then releases and clears
/// it; clears the half-word at `+0x110`; releases and clears `+0x114`,
/// `+0x140`, `+0x128` and `+0x20c`; clears `+0x138`; releases and clears
/// `+0x13c`. The arrays at `+0x210`, `+0x220`, `+0x230` free their data
/// through their own vtable slot `+8`. Then each array's destructor body
/// runs, `+0x230` first: vtable `0101a8e8` (resp. `010b7ea8` for `+0x220`
/// and `+0x210`), and when data is left the function at slot `+8` of that
/// vtable (read from `0101a8f0`/`010b7eb0`). The member destructors follow:
/// releases of `+0x20c`, `+0x140`, `+0x13c` (not cleared), `00e74d40` on
/// `+0x12c`, releases of `+0x128`, `+0x114`, `+0x10c`, `+0xf8`, `+0xf0`,
/// `00e74d40` on `+0xe0`; last the base vtable `0101dce4` and
/// `InterlockedDecrement` of the object count at `011f4400`.
pub fn fn_00b9f0c0(e: &mut Engine, this: Ptr) {
    let at = |offset: u32| this.addr().wrapping_add(offset);
    e.mem.set_u32(this.addr(), OWN_VTABLE);
    e.call(DETACH_LIGHTS, &args![this]);
    release_and_clear(e, this, 0xf0);
    release_and_clear(e, this, 0xf8);
    let hooked = e.mem.u32(at(0x10c));
    if hooked != 0 {
        let owner = e.mem.u32(RELEASE_HOOK_OWNER);
        e.call(RELEASE_HOOK, &args![owner, hooked]);
    }
    release_and_clear(e, this, 0x10c);
    e.mem.set_u16(at(0x110), 0);
    release_and_clear(e, this, 0x114);
    release_and_clear(e, this, 0x140);
    release_and_clear(e, this, 0x128);
    release_and_clear(e, this, 0x20c);
    e.mem.set_u32(at(0x138), 0);
    release_and_clear(e, this, 0x13c);
    for offset in [0x210, 0x220, 0x230] {
        clear_array(e, at(offset), |e, array, data| {
            e.vcall(array, SLOT_ARRAY_FREE, &args![data]);
        });
    }
    // The arrays' own destructor bodies, last member first. The pass above
    // has already zeroed every data word, so their free calls run only if
    // a free call stored data again; the exe keeps them.
    for (offset, vtable, free_slot) in [
        (0x230, NODE_ARRAY_VTABLE, NODE_ARRAY_FREE_SLOT),
        (0x220, ARRAY_VTABLE, ARRAY_FREE_SLOT),
        (0x210, ARRAY_VTABLE, ARRAY_FREE_SLOT),
    ] {
        let array = at(offset);
        e.mem.set_u32(array, vtable);
        clear_array(e, array, |e, array, data| {
            let free = e.mem.u32(free_slot);
            e.call(free, &args![array, data]);
        });
    }
    for offset in [0x20c, 0x140, 0x13c] {
        let object = e.mem.u32(at(offset));
        release(e, object);
    }
    e.call(LIST_REMOVE_ALL, &args![at(0x12c)]);
    for offset in [0x128, 0x114, 0x10c, 0xf8, 0xf0] {
        let object = e.mem.u32(at(offset));
        release(e, object);
    }
    e.call(LIST_REMOVE_ALL, &args![at(0xe0)]);
    e.mem.set_u32(this.addr(), REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![OBJECT_COUNT]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b9_f0c0, fn_00b9f0c0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decrementing(e: &mut Engine) {
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    fn calls_to(log: &[(u32, Vec<u32>)], callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn releases_members_frees_arrays_and_ends_as_a_ref_object() {
        let mut e = Engine::new();
        decrementing(&mut e);
        for a in [
            DETACH_LIGHTS,
            RELEASE_HOOK,
            LIST_REMOVE_ALL,
            0x0090_1000,
            0x0090_2000,
            0x0090_3000,
            0x0090_4000,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.map(RELEASE_HOOK_OWNER, 4);
        e.set_global(RELEASE_HOOK_OWNER, 0x0055_0000u32);
        e.map(OBJECT_COUNT, 4);
        e.set_global(OBJECT_COUNT, 10u32);
        e.put_vtable(NODE_ARRAY_VTABLE, &[0, 0, 0x0090_3000]);
        e.put_vtable(ARRAY_VTABLE, &[0, 0, 0x0090_4000]);
        // An object with two references (at +0x10c) and one with one (+0xf0).
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        let twice = e.mem.alloc(8);
        e.mem.set_u32(twice, 0x0090_0000);
        e.mem.set_u32(twice + 4, 2);
        let once = e.mem.alloc(8);
        e.mem.set_u32(once, 0x0090_0000);
        e.mem.set_u32(once + 4, 1);
        let this = e.mem.alloc(0x240);
        e.mem.set_u32(this + 0x10c, twice);
        e.mem.set_u32(this + 0xf0, once);
        e.mem.set_u16(this + 0x110, 0xffff);
        e.mem.set_u32(this + 0x138, 0x1234);
        // The array at +0x220 has data; its own vtable frees it.
        e.put_vtable(0x0090_5000, &[0, 0, 0x0090_2000]);
        e.mem.set_u32(this + 0x220, 0x0090_5000);
        e.mem.set_u32(this + 0x224, 0x0066_0000);
        e.mem.set_u32(this + 0x228, 3);
        e.mem.set_u32(this + 0x22c, 4);
        e.call_log = Some(vec![]);
        e.call(0x00b9_f0c0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, DETACH_LIGHTS), vec![vec![this]]);
        assert_eq!(calls_to(&log, RELEASE_HOOK), vec![vec![0x0055_0000, twice]]);
        // Both objects lost one reference; the one at +0xf0 was deleted.
        assert_eq!(e.mem.u32(twice + 4), 1);
        assert_eq!(e.mem.u32(once + 4), 0);
        assert_eq!(calls_to(&log, 0x0090_1000), vec![vec![once]]);
        assert_eq!(
            calls_to(&log, 0x0090_2000),
            vec![vec![this + 0x220, 0x0066_0000]]
        );
        assert!(calls_to(&log, 0x0090_3000).is_empty());
        assert!(calls_to(&log, 0x0090_4000).is_empty());
        for offset in [0x10c, 0xf0, 0x224, 0x228, 0x22c, 0x138] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u16(this + 0x110), 0);
        assert_eq!(e.mem.u32(this + 0x230), NODE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x220), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x210), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this), REF_OBJECT_VTABLE);
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 9);
        assert_eq!(
            calls_to(&log, LIST_REMOVE_ALL),
            vec![vec![this + 0x12c], vec![this + 0xe0]]
        );
    }

    #[test]
    fn members_cleared_in_the_body_are_not_released_again() {
        // +0x13c and +0x20c are released and cleared in the body, so the
        // member destructors find them null: one decrement each.
        let mut e = Engine::new();
        decrementing(&mut e);
        for a in [DETACH_LIGHTS, LIST_REMOVE_ALL] {
            e.register(a, |_, _| Ret::default());
        }
        e.map(OBJECT_COUNT, 4);
        let this = e.mem.alloc(0x240);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 5);
        e.mem.set_u32(this + 0x13c, object);
        e.mem.set_u32(this + 0x20c, object);
        e.call(0x00b9_f0c0, &[this]);
        assert_eq!(e.mem.u32(object + 4), 3);
    }
}
