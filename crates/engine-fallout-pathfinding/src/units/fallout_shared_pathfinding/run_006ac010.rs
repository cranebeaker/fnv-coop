//! `fallout shared/pathfinding/run_006ac010` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The member functions of a `BSSimpleArray` whose elements are 0x28 bytes
//! (the element destructor `004ff7e0` is the one of a `PathingLocation`, which
//! is 0x28 bytes), and the constructor of `NavMeshSearchSLPoint`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// Size in bytes of one element.
const ELEMENT_SIZE: u32 = 0x28;
/// `006a8500(this)`: frees the buffer through the array's vtable (`+8`) and
/// clears the buffer pointer.
const ARRAY_FREE_BUFFER: u32 = 0x006a_8500;
/// `004ff7e0(element)`: the destructor body of a `PathingLocation`.
const ELEMENT_DESTRUCT: u32 = 0x004f_f7e0;
/// `operator delete(block)` (`00401030`, cdecl).
const DELETE: u32 = 0x0040_1030;
/// `006a6a40(this)`: the base class constructor of `NavMeshSearchSLPoint`.
const SEARCH_BASE_CONSTRUCT: u32 = 0x006a_6a40;
/// `006dcbb0(this)`: the `PathingLocation` constructor.
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
/// `006815c0(this)`: constructor of the 12-byte member (three of them).
const MEMBER_CONSTRUCT: u32 = 0x0068_15c0;
/// The vtable of `NavMeshSearchSLPoint` (RTTI name).
const SEARCH_SL_POINT_VTABLE: u32 = 0x0106_b96c;

// Translated from 006ac010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the elements of the array (`006ac070`) and sets its size to 0.
/// With `free_buffer` nonzero it also frees the buffer (`006a8500`) and
/// clears the buffer pointer and the reserved size. An array without a
/// buffer is left alone. The map has no name for it.
pub fn fn_006ac010(e: &mut Engine, this: Ptr<BSSimpleArray>, free_buffer: u8) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    fn_006ac070(e, this, buffer, size);
    if free_buffer != 0 {
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

// Translated from 006ac070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `count` elements from `elements` on, each through the element's
/// scalar deleting destructor `006ac0b0` with flags 0 (the memory stays).
/// `this` (the array) is not read.
pub fn fn_006ac070(e: &mut Engine, _this: Ptr<BSSimpleArray>, elements: u32, count: u32) {
    for index in 0..count {
        let slot = elements.wrapping_add(index.wrapping_mul(ELEMENT_SIZE));
        fn_006ac0b0(e, Ptr::new(slot), 0);
    }
}

// Translated from 006ac0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the 0x28-byte element: runs the
/// `PathingLocation` destructor body (`004ff7e0`) and, when bit 0 of `flags`
/// is set, frees the memory (`00401030`). Returns `this`.
pub fn fn_006ac0b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ELEMENT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DELETE, &args![this]);
    }
    this
}

// Translated from 006ac0e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ac0e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of `NavMeshSearchSLPoint` (the RTTI name of the vtable it
/// stores; no Xbox PDB name is mapped): runs the base constructor
/// (`006a6a40`), stores the vtable, builds the `PathingLocation` at `+0x20A4`
/// (`006dcbb0`) and the three 12-byte members at `+0x20CC`, `+0x20D8` and
/// `+0x20E4` (`006815c0`). Returns `this`. The exception-unwinding frame is
/// not translated.
pub fn fn_006ac0e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SEARCH_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SEARCH_SL_POINT_VTABLE);
    e.call(
        PATHING_LOCATION_CONSTRUCT,
        &args![this.addr().wrapping_add(0x20a4)],
    );
    for offset in [0x20cc_u32, 0x20d8, 0x20e4] {
        e.call(MEMBER_CONSTRUCT, &args![this.addr().wrapping_add(offset)]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006ac010, fn_006ac010(Ptr<BSSimpleArray>, u8)),
        entry!(0x006ac070, fn_006ac070(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006ac0b0, fn_006ac0b0(Ptr, u32) -> Ptr),
        entry!(0x006ac0e0, fn_006ac0e0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn silent(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    fn array(e: &mut Engine, buffer: u32, size: u32, reserved: u32) -> Ptr<BSSimpleArray> {
        let this: Ptr<BSSimpleArray> = e.new_object();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iSize, size);
        e.set(this, BSSimpleArray::iReservedSize, reserved);
        this
    }

    #[test]
    fn clear_destroys_each_element_without_freeing_it() {
        let mut e = Engine::new();
        silent(&mut e, &[ELEMENT_DESTRUCT, DELETE, ARRAY_FREE_BUFFER]);
        let this = array(&mut e, 0x4000, 2, 8);
        e.call_log = Some(vec![]);
        e.call(0x006ac010, &args![this, 0u8]);
        assert_eq!(
            calls(&e),
            vec![
                (0x006ac010, vec![this.addr(), 0]),
                (ELEMENT_DESTRUCT, vec![0x4000]),
                (ELEMENT_DESTRUCT, vec![0x4028]),
            ]
        );
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x4000);
    }

    #[test]
    fn clear_with_free_releases_the_buffer_and_ignores_a_missing_one() {
        let mut e = Engine::new();
        silent(&mut e, &[ELEMENT_DESTRUCT, ARRAY_FREE_BUFFER]);
        let this = array(&mut e, 0x4000, 1, 8);
        e.call_log = Some(vec![]);
        e.call(0x006ac010, &args![this, 1u8]);
        assert_eq!(calls(&e)[2], (ARRAY_FREE_BUFFER, vec![this.addr()]));
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 0);
        let empty = array(&mut e, 0, 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x006ac010, &args![empty, 1u8]);
        assert_eq!(calls(&e).len(), 1);
        assert_eq!(e.get(empty, BSSimpleArray::iSize), 4);
    }

    #[test]
    fn destroy_elements_walks_the_buffer() {
        let mut e = Engine::new();
        silent(&mut e, &[ELEMENT_DESTRUCT]);
        e.call_log = Some(vec![]);
        e.call(0x006ac070, &args![0u32, 0x4000u32, 3u32]);
        let log = calls(&e);
        assert_eq!(log.len(), 4);
        assert_eq!(log[3], (ELEMENT_DESTRUCT, vec![0x4000 + 2 * 0x28]));
    }

    #[test]
    fn element_destructor_frees_only_when_asked() {
        let mut e = Engine::new();
        silent(&mut e, &[ELEMENT_DESTRUCT, DELETE]);
        e.call_log = Some(vec![]);
        let result = e.call(0x006ac0b0, &args![0x4000u32, 0u32]).u32();
        assert_eq!(result, 0x4000);
        assert_eq!(calls(&e).len(), 2);
        e.call_log = Some(vec![]);
        e.call(0x006ac0b0, &args![0x4000u32, 1u32]);
        assert_eq!(calls(&e)[2], (DELETE, vec![0x4000]));
    }

    #[test]
    fn constructor_builds_the_base_and_the_members() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[
                SEARCH_BASE_CONSTRUCT,
                PATHING_LOCATION_CONSTRUCT,
                MEMBER_CONSTRUCT,
            ],
        );
        let this = Ptr::<()>::new(e.mem.alloc(0x2100));
        e.call_log = Some(vec![]);
        let result = e.call(0x006ac0e0, &args![this]).ptr::<()>();
        assert_eq!(result, this);
        let base = this.addr();
        assert_eq!(
            calls(&e)[1..],
            [
                (SEARCH_BASE_CONSTRUCT, vec![base]),
                (PATHING_LOCATION_CONSTRUCT, vec![base + 0x20a4]),
                (MEMBER_CONSTRUCT, vec![base + 0x20cc]),
                (MEMBER_CONSTRUCT, vec![base + 0x20d8]),
                (MEMBER_CONSTRUCT, vec![base + 0x20e4]),
            ]
        );
        assert_eq!(e.mem.u32(base), 0x0106_b96c);
    }
}
