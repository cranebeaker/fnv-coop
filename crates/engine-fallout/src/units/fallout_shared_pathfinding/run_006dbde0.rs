//! `fallout shared/pathfinding/run_006dbde0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The member functions of a `BSSimpleArray` whose elements are 0x54 bytes
//! (`BSSimpleArray<T, 1024>`: vtable, buffer, size, reserved size). The
//! engine map does not name the instance. The element constructor `006d3200`
//! stores the vtable `0x0106cbc0`, whose RTTI name is `PathingCoverLocation`;
//! the element destructor is `006dc4d0`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// Size in bytes of one element.
const ELEMENT_SIZE: u32 = 0x54;
/// `006a8500(this)`: frees the buffer through the array's vtable (`+8`) and
/// clears the buffer pointer.
const ARRAY_FREE_BUFFER: u32 = 0x006a_8500;
/// `006dc4d0(element, flags)`: the element's scalar deleting destructor
/// (`flags` bit 0 frees the memory).
const ELEMENT_DESTROY: u32 = 0x006d_c4d0;
/// `006d3200(element)`: the element constructor.
const ELEMENT_CONSTRUCT: u32 = 0x006d_3200;
/// `006e6da0(size, address)`: placement `new`, which gives `address` back.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `memmove(destination, source, size)` (`00ec7230`, cdecl).
const MEMORY_MOVE: u32 = 0x00ec_7230;

// Translated from 006dbde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the elements of the array (`006dbee0`) and sets its size to 0.
/// With `free_buffer` nonzero it also frees the buffer (`006a8500`) and
/// clears the buffer pointer and the reserved size. An array without a
/// buffer is left alone. The map has no name for it.
pub fn fn_006dbde0(e: &mut Engine, this: Ptr<BSSimpleArray>, free_buffer: u8) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    fn_006dbee0(e, this, buffer, size);
    if free_buffer != 0 {
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

// Translated from 006dbe40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006dbe40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructs `count` elements in place from `elements` on: placement `new`
/// (`006e6da0`) at each slot and, when it gives an address back, the element
/// constructor (`006d3200`). `this` (the array) is not read. The
/// exception-unwinding frame is not translated.
pub fn fn_006dbe40(e: &mut Engine, _this: Ptr<BSSimpleArray>, elements: u32, count: u32) {
    for index in 0..count {
        let slot = index.wrapping_mul(ELEMENT_SIZE).wrapping_add(elements);
        let placed = e.call(PLACEMENT_NEW, &args![ELEMENT_SIZE, slot]).u32();
        if placed != 0 {
            e.call(ELEMENT_CONSTRUCT, &args![placed]);
        }
    }
}

// Translated from 006dbee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `count` elements from `elements` on, each through the element
/// destructor `006dc4d0` with flags 0 (the memory stays). `this` (the array)
/// is not read.
pub fn fn_006dbee0(e: &mut Engine, _this: Ptr<BSSimpleArray>, elements: u32, count: u32) {
    for index in 0..count {
        let slot = index.wrapping_mul(ELEMENT_SIZE).wrapping_add(elements);
        e.call(ELEMENT_DESTROY, &args![slot, 0u32]);
    }
}

// Translated from 006dbf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` elements from `source` to `destination`, element by
/// element with `memmove`: first to last when the destination is below the
/// source, last to first when it is above, nothing when they are equal.
/// `this` (the array) is not read.
pub fn fn_006dbf20(
    e: &mut Engine,
    _this: Ptr<BSSimpleArray>,
    destination: u32,
    source: u32,
    count: u32,
) {
    if count == 0 {
        return;
    }
    if destination < source {
        for index in 0..count {
            let offset = index.wrapping_mul(ELEMENT_SIZE);
            e.call(
                MEMORY_MOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    ELEMENT_SIZE
                ],
            );
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let offset = (index as u32).wrapping_mul(ELEMENT_SIZE);
            e.call(
                MEMORY_MOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    ELEMENT_SIZE
                ],
            );
            index = index.wrapping_sub(1);
        }
    }
}

// Translated from 006dbfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes room for `capacity` elements, through the array's vtable (`+4`
/// allocates `capacity` elements, `+0xc` reallocates a buffer to `capacity`
/// elements). Without a buffer it allocates one and records `capacity`
/// as the reserved size at `+0xC`. With a buffer and `kept` equal to the
/// reserved size it reallocates in place and stores the new buffer pointer
/// (the reserved size is not changed here). Otherwise it allocates a new
/// buffer, moves `kept` elements into it (`006dbf20`), frees the old buffer
/// (`006a8500`) and installs the new one. The map has no name for it.
pub fn fn_006dbfc0(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, kept: u32) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        let new_buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else if kept == e.get(this, BSSimpleArray::iReservedSize) {
        let new_buffer = e.vcall(this.addr(), 0xc, &args![buffer, capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
    } else {
        let new_buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_006dbf20(e, this, new_buffer, buffer, kept);
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006dbde0, fn_006dbde0(Ptr<BSSimpleArray>, u8)),
        entry!(0x006dbe40, fn_006dbe40(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006dbee0, fn_006dbee0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006dbf20, fn_006dbf20(Ptr<BSSimpleArray>, u32, u32, u32)),
        entry!(0x006dbfc0, fn_006dbfc0(Ptr<BSSimpleArray>, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0900_0000;
    const ALLOCATE: u32 = 0x0900_0100;
    const REALLOCATE: u32 = 0x0900_0200;

    /// An array object with a vtable whose slots +4 and +0xC are doubles
    /// that give back 0x5000 and 0x6000.
    fn array_with_vtable(
        e: &mut Engine,
        buffer: u32,
        size: u32,
        reserved: u32,
    ) -> Ptr<BSSimpleArray> {
        e.put_vtable(VTABLE, &[0, ALLOCATE, 0, REALLOCATE]);
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        e.register(REALLOCATE, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        let this: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(this.addr(), VTABLE);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iSize, size);
        e.set(this, BSSimpleArray::iReservedSize, reserved);
        this
    }

    fn silent(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    #[test]
    fn clear_destroys_the_elements_and_empties_the_array() {
        let mut e = Engine::new();
        silent(&mut e, &[ELEMENT_DESTROY, ARRAY_FREE_BUFFER]);
        let this = array_with_vtable(&mut e, 0x4000, 2, 8);
        e.call_log = Some(vec![]);
        e.call(0x006dbde0, &args![this, 0u8]);
        assert_eq!(
            calls(&e),
            vec![
                (0x006dbde0, vec![this.addr(), 0]),
                (ELEMENT_DESTROY, vec![0x4000, 0]),
                (ELEMENT_DESTROY, vec![0x4054, 0]),
            ]
        );
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x4000);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
    }

    #[test]
    fn clear_with_free_releases_the_buffer() {
        let mut e = Engine::new();
        silent(&mut e, &[ELEMENT_DESTROY, ARRAY_FREE_BUFFER]);
        let this = array_with_vtable(&mut e, 0x4000, 1, 8);
        e.call_log = Some(vec![]);
        e.call(0x006dbde0, &args![this, 1u8]);
        let log = calls(&e);
        assert_eq!(log[1], (ELEMENT_DESTROY, vec![0x4000, 0]));
        assert_eq!(log[2], (ARRAY_FREE_BUFFER, vec![this.addr()]));
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 0);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
    }

    #[test]
    fn clear_leaves_an_array_without_buffer_alone() {
        let mut e = Engine::new();
        let this = array_with_vtable(&mut e, 0, 3, 0);
        e.call_log = Some(vec![]);
        e.call(0x006dbde0, &args![this, 1u8]);
        assert_eq!(calls(&e).len(), 1);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 3);
    }

    #[test]
    fn construct_elements_runs_the_constructor_where_new_gives_an_address() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: if a[1] == 0x4000 { 0 } else { a[1] },
            ..Ret::default()
        });
        silent(&mut e, &[ELEMENT_CONSTRUCT]);
        e.call_log = Some(vec![]);
        e.call(0x006dbe40, &args![0u32, 0x4000u32, 2u32]);
        assert_eq!(
            calls(&e),
            vec![
                (0x006dbe40, vec![0, 0x4000, 2]),
                (PLACEMENT_NEW, vec![0x54, 0x4000]),
                (PLACEMENT_NEW, vec![0x54, 0x4054]),
                (ELEMENT_CONSTRUCT, vec![0x4054]),
            ]
        );
    }

    #[test]
    fn destroy_elements_uses_flags_zero() {
        let mut e = Engine::new();
        silent(&mut e, &[ELEMENT_DESTROY]);
        e.call_log = Some(vec![]);
        e.call(0x006dbee0, &args![0u32, 0x4000u32, 3u32]);
        let log = calls(&e);
        assert_eq!(log.len(), 4);
        assert_eq!(log[3], (ELEMENT_DESTROY, vec![0x4000 + 2 * 0x54, 0]));
    }

    #[test]
    fn move_elements_goes_up_or_down_by_direction() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMORY_MOVE]);
        e.call_log = Some(vec![]);
        e.call(0x006dbf20, &args![0u32, 0x1000u32, 0x2000u32, 2u32]);
        e.call(0x006dbf20, &args![0u32, 0x2000u32, 0x1000u32, 2u32]);
        e.call(0x006dbf20, &args![0u32, 0x1000u32, 0x1000u32, 2u32]);
        e.call(0x006dbf20, &args![0u32, 0x2000u32, 0x1000u32, 0u32]);
        let moves: Vec<_> = calls(&e)
            .into_iter()
            .filter(|c| c.0 == MEMORY_MOVE)
            .map(|c| (c.1[0], c.1[1]))
            .collect();
        assert_eq!(
            moves,
            vec![
                (0x1000, 0x2000),
                (0x1054, 0x2054),
                (0x2054, 0x1054),
                (0x2000, 0x1000),
            ]
        );
    }

    #[test]
    fn grow_allocates_when_there_is_no_buffer() {
        let mut e = Engine::new();
        let this = array_with_vtable(&mut e, 0, 0, 0);
        e.call(0x006dbfc0, &args![this, 16u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x5000);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 16);
    }

    #[test]
    fn grow_reallocates_in_place_when_full() {
        let mut e = Engine::new();
        let this = array_with_vtable(&mut e, 0x4000, 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x006dbfc0, &args![this, 16u32, 8u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x6000);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        assert_eq!(calls(&e)[1], (REALLOCATE, vec![this.addr(), 0x4000, 16]));
    }

    #[test]
    fn grow_moves_into_a_new_buffer_otherwise() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMORY_MOVE, ARRAY_FREE_BUFFER]);
        let this = array_with_vtable(&mut e, 0x4000, 3, 8);
        e.call_log = Some(vec![]);
        e.call(0x006dbfc0, &args![this, 16u32, 3u32]);
        let log = calls(&e);
        assert_eq!(log[1], (ALLOCATE, vec![this.addr(), 16]));
        assert_eq!(log[2], (MEMORY_MOVE, vec![0x50a8, 0x40a8, 0x54]));
        assert_eq!(log[4], (MEMORY_MOVE, vec![0x5000, 0x4000, 0x54]));
        assert_eq!(log[5], (ARRAY_FREE_BUFFER, vec![this.addr()]));
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x5000);
    }
}
