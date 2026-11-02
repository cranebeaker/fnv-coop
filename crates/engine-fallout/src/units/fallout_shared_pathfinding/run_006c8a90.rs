//! `fallout shared/pathfinding/run_006c8a90` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The member functions of a `BSSimpleArray` whose elements are 0x10 bytes
//! (constructor loop, element constructor, destructor loop and mover). The
//! map does not name the instance.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// Size in bytes of one element.
const ELEMENT_SIZE: u32 = 0x10;
/// `006e6da0(size, address)`: placement `new`, which gives `address` back.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `00633c90(this, word)`: constructor of the member at the start of the
/// element.
const MEMBER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0042fb00(this)`: constructor of the member at `+0xC` of the element.
const TAIL_CONSTRUCT: u32 = 0x0042_fb00;
/// `006c8d00(element, flags)`: the element's scalar deleting destructor.
const ELEMENT_DESTROY: u32 = 0x006c_8d00;
/// `memmove(destination, source, size)` (`00ec7230`, cdecl).
const MEMORY_MOVE: u32 = 0x00ec_7230;

// Translated from 006c8a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c8a90 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructs `count` elements in place from `elements` on: placement `new`
/// (`006e6da0`) at each slot and, when it gives an address back, the element
/// constructor `006c8b30`. `this` (the array) is not read. The
/// exception-unwinding frame is not translated.
pub fn fn_006c8a90(e: &mut Engine, _this: Ptr<BSSimpleArray>, elements: u32, count: u32) {
    for index in 0..count {
        let slot = index.wrapping_shl(4).wrapping_add(elements);
        let placed = e.call(PLACEMENT_NEW, &args![ELEMENT_SIZE, slot]).u32();
        if placed != 0 {
            fn_006c8b30(e, Ptr::new(placed));
        }
    }
}

// Translated from 006c8b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c8b30 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Element constructor: `00633c90(this, 0)`, then `0042fb00(this + 0xC)`.
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_006c8b30(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMBER_CONSTRUCT, &args![this, 0u32]);
    e.call(TAIL_CONSTRUCT, &args![this.addr().wrapping_add(0xc)]);
    this
}

// Translated from 006c8b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `count` elements from `elements` on, each through the element
/// destructor `006c8d00` with flags 0 (the memory stays). `this` (the array)
/// is not read.
pub fn fn_006c8b90(e: &mut Engine, _this: Ptr<BSSimpleArray>, elements: u32, count: u32) {
    for index in 0..count {
        let slot = index.wrapping_shl(4).wrapping_add(elements);
        e.call(ELEMENT_DESTROY, &args![slot, 0u32]);
    }
}

// Translated from 006c8bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` elements from `source` to `destination`, element by
/// element with `memmove`: first to last when the destination is below the
/// source, last to first when it is above, nothing when they are equal.
/// `this` (the array) is not read.
pub fn fn_006c8bd0(
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
            let offset = index.wrapping_shl(4);
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
            let offset = (index as u32).wrapping_shl(4);
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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c8a90, fn_006c8a90(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006c8b30, fn_006c8b30(Ptr) -> Ptr),
        entry!(0x006c8b90, fn_006c8b90(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006c8bd0, fn_006c8bd0(Ptr<BSSimpleArray>, u32, u32, u32)),
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

    #[test]
    fn element_constructor_builds_both_members() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMBER_CONSTRUCT, TAIL_CONSTRUCT]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c8b30, &args![0x4000u32]).u32(), 0x4000);
        assert_eq!(
            calls(&e)[1..],
            [
                (MEMBER_CONSTRUCT, vec![0x4000, 0]),
                (TAIL_CONSTRUCT, vec![0x400c])
            ]
        );
    }

    #[test]
    fn construct_elements_skips_a_refused_slot() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMBER_CONSTRUCT, TAIL_CONSTRUCT]);
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: if a[1] == 0x4000 { 0 } else { a[1] },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006c8a90, &args![0u32, 0x4000u32, 2u32]);
        assert_eq!(
            calls(&e)[1..],
            [
                (PLACEMENT_NEW, vec![0x10, 0x4000]),
                (PLACEMENT_NEW, vec![0x10, 0x4010]),
                (MEMBER_CONSTRUCT, vec![0x4010, 0]),
                (TAIL_CONSTRUCT, vec![0x401c]),
            ]
        );
    }

    #[test]
    fn destroy_elements_uses_flags_zero() {
        let mut e = Engine::new();
        silent(&mut e, &[ELEMENT_DESTROY]);
        e.call_log = Some(vec![]);
        e.call(0x006c8b90, &args![0u32, 0x4000u32, 3u32]);
        assert_eq!(
            calls(&e)[1..],
            [
                (ELEMENT_DESTROY, vec![0x4000, 0]),
                (ELEMENT_DESTROY, vec![0x4010, 0]),
                (ELEMENT_DESTROY, vec![0x4020, 0]),
            ]
        );
    }

    #[test]
    fn move_elements_goes_up_or_down_by_direction() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMORY_MOVE]);
        e.call_log = Some(vec![]);
        e.call(0x006c8bd0, &args![0u32, 0x1000u32, 0x2000u32, 2u32]);
        e.call(0x006c8bd0, &args![0u32, 0x2000u32, 0x1000u32, 2u32]);
        e.call(0x006c8bd0, &args![0u32, 0x1000u32, 0x1000u32, 2u32]);
        e.call(0x006c8bd0, &args![0u32, 0x2000u32, 0x1000u32, 0u32]);
        let moves: Vec<_> = calls(&e)
            .into_iter()
            .filter(|c| c.0 == MEMORY_MOVE)
            .collect();
        assert_eq!(
            moves,
            vec![
                (MEMORY_MOVE, vec![0x1000, 0x2000, 0x10]),
                (MEMORY_MOVE, vec![0x1010, 0x2010, 0x10]),
                (MEMORY_MOVE, vec![0x2010, 0x1010, 0x10]),
                (MEMORY_MOVE, vec![0x2000, 0x1000, 0x10]),
            ]
        );
    }
}
