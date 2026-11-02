//! `(unplaced)/run_005eefc0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFootIK_Leg_DATA@FootIK_DATA@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_010472F8: u32 = 0x0104_72f8;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 005eeff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFootIK_Leg_DATA@FootIK_DATA@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_005eeff0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_010472F8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `BSSimpleArray` construction of `count` elements (0x38 bytes each) at
/// an address (`005ef750`, `RET 8`), their destruction (`0072ba80`), and
/// the buffer resize `(new capacity, count to keep)` (`005ef860`).
const CONSTRUCT_ELEMENTS: u32 = 0x005e_f750;
const DESTROY_ELEMENTS: u32 = 0x0072_ba80;
const RESIZE_BUFFER: u32 = 0x005e_f860;
/// The leg data element size.
const ELEMENT_SIZE: u32 = 0x38;

// Translated from 005eefc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance with RTTI name
/// `.?AV?$BSSimpleArray@UFootIK_Leg_DATA@FootIK_DATA@@$0EAA@@`: stores the
/// instance's vtable and runs `005ef910(0, 0)`. Returns `this`.
pub fn fn_005eefc0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_010472F8);
    e.call(0x005e_f910, &args![this, 0u32, 0u32]);
    this
}

// Translated from 005ef010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the leg data array's size to `size` (`RET 8`; `release` is a byte):
/// 0 releases it (`008454f0(release)`); a size above the capacity
/// allocates (vtable +4, when there is no buffer) or resizes the buffer
/// and constructs the new elements; a smaller size destroys the elements
/// past it and, with `release` set and at most a quarter of the capacity
/// left, shrinks the buffer to the size; a larger size within the
/// capacity constructs the new elements.
pub fn fn_005ef010(e: &mut Engine, this: Ptr<BSSimpleArray>, size: u32, release: u8) {
    if size == 0 {
        e.call(ARRAY_RELEASE_008454F0, &args![this, u32::from(release)]);
        return;
    }
    let capacity = e.get(this, BSSimpleArray::iReservedSize);
    if size > capacity {
        if capacity == 0 {
            let buffer = e.vcall(this.addr(), 4, &args![size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, size);
        } else {
            let count = e.get(this, BSSimpleArray::iSize);
            e.call(RESIZE_BUFFER, &args![this, size, count]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
        construct_from_count(e, this, size);
    } else if size < e.get(this, BSSimpleArray::iSize) {
        let count = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let first = size.wrapping_mul(ELEMENT_SIZE).wrapping_add(buffer);
        e.call(
            DESTROY_ELEMENTS,
            &args![this, first, count.wrapping_sub(size)],
        );
        e.set(this, BSSimpleArray::iSize, size);
        if release != 0 && size <= e.get(this, BSSimpleArray::iReservedSize) >> 2 {
            e.call(RESIZE_BUFFER, &args![this, size, size]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
    } else {
        construct_from_count(e, this, size);
    }
}

/// Constructs the elements from the current count up to `size` and makes
/// `size` the count.
fn construct_from_count(e: &mut Engine, this: Ptr<BSSimpleArray>, size: u32) {
    let count = e.get(this, BSSimpleArray::iSize);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let first = count.wrapping_mul(ELEMENT_SIZE).wrapping_add(buffer);
    e.call(
        CONSTRUCT_ELEMENTS,
        &args![this, first, size.wrapping_sub(count)],
    );
    e.set(this, BSSimpleArray::iSize, size);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_eff0, fn_005eeff0(Ptr)),
        entry!(
            0x005e_efc0,
            fn_005eefc0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x005e_f010, fn_005ef010(Ptr<BSSimpleArray>, u32, u8)),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_005eeff0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x005e_eff0, ARRAY_VTABLE_010472F8, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod leg_array_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An array with the given buffer, count and capacity; its vtable +4
    /// allocates 0x9000.
    fn array(e: &mut Engine, buffer: u32, count: u32, capacity: u32) -> Ptr<BSSimpleArray> {
        let this: Ptr<BSSimpleArray> = e.new_object();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x7000_0004);
        e.mem.set_u32(this.addr(), vtable);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iSize, count);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
        e.register(0x7000_0004, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        for addr in [
            ARRAY_RELEASE_008454F0,
            CONSTRUCT_ELEMENTS,
            DESTROY_ELEMENTS,
            RESIZE_BUFFER,
            0x005e_f910,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        this
    }

    fn state(e: &Engine, this: Ptr<BSSimpleArray>) -> (u32, u32, u32) {
        (
            e.get(this, BSSimpleArray::pBuffer),
            e.get(this, BSSimpleArray::iSize),
            e.get(this, BSSimpleArray::iReservedSize),
        )
    }

    #[test]
    fn fn_005eefc0_sets_the_vtable_and_starts_empty() {
        let mut e = Engine::new();
        let this = array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_efc0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), ARRAY_VTABLE_010472F8);
        assert_eq!(calls(&e, 0x005e_f910), vec![vec![this.addr(), 0, 0]]);
    }

    #[test]
    fn fn_005ef010_grows_shrinks_and_releases() {
        // Zero: release with the flag byte.
        let mut e = Engine::new();
        let this = array(&mut e, 0x5000, 2, 4);
        e.call_log = Some(vec![]);
        e.call(0x005e_f010, &args![this, 0u32, 0x101u32]);
        assert_eq!(
            calls(&e, ARRAY_RELEASE_008454F0),
            vec![vec![this.addr(), 1]]
        );

        // Growing past an empty capacity: allocate, construct all.
        let mut e = Engine::new();
        let this = array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_f010, &args![this, 3u32, 0u32]);
        assert_eq!(state(&e, this), (0x9000, 3, 3));
        assert_eq!(
            calls(&e, CONSTRUCT_ELEMENTS),
            vec![vec![this.addr(), 0x9000, 3]]
        );

        // Growing past a capacity: resize, construct the new ones.
        let mut e = Engine::new();
        let this = array(&mut e, 0x5000, 2, 2);
        e.call_log = Some(vec![]);
        e.call(0x005e_f010, &args![this, 5u32, 0u32]);
        assert_eq!(calls(&e, RESIZE_BUFFER), vec![vec![this.addr(), 5, 2]]);
        assert_eq!(
            calls(&e, CONSTRUCT_ELEMENTS),
            vec![vec![this.addr(), 0x5000 + 2 * 0x38, 3]]
        );
        assert_eq!(state(&e, this), (0x5000, 5, 5));

        // Within the capacity: construct only.
        let mut e = Engine::new();
        let this = array(&mut e, 0x5000, 1, 8);
        e.call_log = Some(vec![]);
        e.call(0x005e_f010, &args![this, 3u32, 1u32]);
        assert_eq!(
            calls(&e, CONSTRUCT_ELEMENTS),
            vec![vec![this.addr(), 0x5000 + 0x38, 2]]
        );
        assert!(calls(&e, RESIZE_BUFFER).is_empty());
        assert_eq!(state(&e, this), (0x5000, 3, 8));

        // Shrinking: destroy; with the flag and a quarter left, resize.
        for (release, size, resized) in [(1u32, 2u32, true), (1, 3, false), (0, 2, false)] {
            let mut e = Engine::new();
            let this = array(&mut e, 0x5000, 6, 8);
            e.call_log = Some(vec![]);
            e.call(0x005e_f010, &args![this, size, release]);
            assert_eq!(
                calls(&e, DESTROY_ELEMENTS),
                vec![vec![this.addr(), 0x5000 + size * 0x38, 6 - size]]
            );
            assert_eq!(e.get(this, BSSimpleArray::iSize), size);
            if resized {
                assert_eq!(
                    calls(&e, RESIZE_BUFFER),
                    vec![vec![this.addr(), size, size]]
                );
                assert_eq!(e.get(this, BSSimpleArray::iReservedSize), size);
            } else {
                assert!(calls(&e, RESIZE_BUFFER).is_empty());
                assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
            }
        }
    }
}
