//! `(unplaced)/run_0066dab0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0066db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): calls `this`'s vtable slot +0x10 with
/// `first` and `count * 16` (`count << 4`); its `EAX` is passed on.
pub fn fn_0066db70(e: &mut Engine, this: Ptr, first: u32, count: u32) -> u32 {
    e.vcall(this.addr(), 0x10, &args![first, count << 4]).u32()
}

/// `004a4840` (thiscall on a stack byte, one word: the array; answers a word), the array's
/// element destruction `0062a120` and capacity `0062a100` (thiscall), and the element
/// constructor `004a3f10` (thiscall on the element, one word: the value).
const ARRAY_ALLOCATOR_004A4840: u32 = 0x004a_4840;
const ARRAY_CLEAR_0062A120: u32 = 0x0062_a120;
const ARRAY_CAPACITY: u32 = 0x0062_a100;
const ELEMENT_CONSTRUCT_004A3F10: u32 = 0x004a_3f10;
/// The capacity word's flag bit 31: the array does not own its buffer.
const DONT_DEALLOCATE: u32 = 0x8000_0000;

// Translated from 0066dab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; called from a destructor at
/// `0066da80`): asks `004a4840` (on a zeroed byte on its stack, with `this`) for the word it
/// passes to [`fn_0066dae0`] on `this`.
pub fn fn_0066dab0(e: &mut Engine, this: Ptr) {
    let allocator = e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), 0);
        e.call(ARRAY_ALLOCATOR_004A4840, &args![byte, this]).u32()
    });
    fn_0066dae0(e, this, allocator);
}

// Translated from 0066dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the Havok-style array's release
/// (data `+0`, capacity and flags `+8`): destroys the elements (`0062a120`); unless the
/// capacity word has bit 31 set, frees the buffer through `allocator`
/// ([`fn_0066db70`] with the data and the capacity, `0062a100`); then clears the data and
/// sets the capacity word to `0x80000000`.
pub fn fn_0066dae0(e: &mut Engine, this: Ptr, allocator: u32) {
    let base = this.addr();
    e.call(ARRAY_CLEAR_0062A120, &args![this]);
    if e.mem.u32(base + 8) & DONT_DEALLOCATE == 0 {
        let capacity = e.call(ARRAY_CAPACITY, &args![this]).u32();
        let data = e.mem.u32(base);
        fn_0066db70(e, Ptr::new(allocator), data, capacity);
    }
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 8, DONT_DEALLOCATE);
}

// Translated from 0066db30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words; the fourth, a tag byte, is not read):
/// constructs `count` 16-byte elements from `first` on (signed count), each with `value`
/// (`004a3f10`).
pub fn fn_0066db30(e: &mut Engine, first: u32, count: i32, value: u32, _unused_4: u32) {
    let mut index = 0i32;
    while index < count {
        let element = ((index as u32) << 4).wrapping_add(first);
        e.call(ELEMENT_CONSTRUCT_004A3F10, &args![element, value]);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_db70, fn_0066db70(Ptr, u32, u32) -> u32),
        entry!(0x0066_dab0, fn_0066dab0(Ptr)),
        entry!(0x0066_dae0, fn_0066dae0(Ptr, u32)),
        entry!(0x0066_db30, fn_0066db30(u32, i32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_the_count_in_sixteen_byte_units() {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| 0x31u32.into_ret());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0150_0000);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0066_db70, &args![this, 0x7000u32, 3u32]).u32(),
            0x31
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0150_1000, vec![this, 0x7000, 0x30])]);
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An array (data 0x4000, capacity word 5) and an allocator whose vtable slot +0x10
    /// is a recorded double.
    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(ARRAY_CLEAR_0062A120, |_, _| Ret::default());
        e.register(ARRAY_CAPACITY, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x3fff_ffff).into_ret()
        });
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, 0x0150_0000);
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array, 0x4000);
        e.mem.set_u32(array + 8, 5);
        e.call_log = Some(vec![]);
        (e, array, allocator)
    }

    #[test]
    fn release_frees_an_owned_buffer() {
        let (mut e, array, allocator) = engine();
        e.call(0x0066_dae0, &args![array, allocator]);
        assert_eq!(
            calls_to(&e, 0x0150_1000),
            vec![vec![allocator, 0x4000, 0x50]]
        );
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 8), DONT_DEALLOCATE);
        assert_eq!(calls_to(&e, ARRAY_CLEAR_0062A120), vec![vec![array]]);
    }

    #[test]
    fn release_keeps_a_borrowed_buffer() {
        let (mut e, array, allocator) = engine();
        e.mem.set_u32(array + 8, DONT_DEALLOCATE | 5);
        e.call(0x0066_dae0, &args![array, allocator]);
        assert!(calls_to(&e, ARRAY_CAPACITY).is_empty());
        assert!(calls_to(&e, 0x0150_1000).is_empty());
        assert_eq!(e.mem.u32(array), 0);
    }

    #[test]
    fn release_through_the_asked_allocator() {
        let (mut e, array, allocator) = engine();
        e.register_double(ARRAY_ALLOCATOR_004A4840, move |e, a| {
            assert_eq!(e.mem.u8(a[0]), 0);
            allocator.into_ret()
        });
        e.call(0x0066_dab0, &args![array]);
        assert_eq!(
            calls_to(&e, 0x0150_1000),
            vec![vec![allocator, 0x4000, 0x50]]
        );
        assert_eq!(calls_to(&e, ARRAY_ALLOCATOR_004A4840)[0][1], array);
    }

    #[test]
    fn constructs_count_elements() {
        let mut e = Engine::new();
        e.register(ELEMENT_CONSTRUCT_004A3F10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0066_db30, &args![0x4000u32, 2i32, 7u32, 0u32]);
        e.call(0x0066_db30, &args![0x4000u32, -1i32, 7u32, 0u32]);
        assert_eq!(
            calls_to(&e, ELEMENT_CONSTRUCT_004A3F10),
            vec![vec![0x4000, 7], vec![0x4010, 7]]
        );
    }
}
