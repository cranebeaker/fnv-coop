//! `fallout shared/pathfinding/run_006927d0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Member functions of two `BSSimpleArray` instances (the 8-byte element
//! array's constructor loop and the 0x28-byte element array's mover) and a
//! thunk that adjusts `this` by -0x1C and continues in a `NavMesh`
//! destructor.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// Size in bytes of the elements `006927d0` constructs.
const SMALL_ELEMENT_SIZE: u32 = 8;
/// Size in bytes of the elements `006928a0` moves.
const MOVED_ELEMENT_SIZE: u32 = 0x28;
/// `006e6da0(size, address)`: placement `new`, which gives `address` back.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `memmove(destination, source, size)` (`00ec7230`, cdecl).
const MEMORY_MOVE: u32 = 0x00ec_7230;
/// `0068ed30(this, flags)`: the `NavMesh` scalar deleting destructor.
const NAV_MESH_DESTROY: u32 = 0x0068_ed30;

// Translated from 006927d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006927d0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructs `count` 8-byte elements in place from `elements` on: placement
/// `new` (`006e6da0`) at each slot and, when it gives an address back, the
/// element constructor `00692870`. `this` (the array) is not read. The
/// exception-unwinding frame is not translated.
pub fn fn_006927d0(e: &mut Engine, _this: Ptr<BSSimpleArray>, elements: u32, count: u32) {
    for index in 0..count {
        let slot = elements.wrapping_add(index.wrapping_mul(SMALL_ELEMENT_SIZE));
        let placed = e
            .call(PLACEMENT_NEW, &args![SMALL_ELEMENT_SIZE, slot])
            .u32();
        if placed != 0 {
            fn_00692870(e, Ptr::new(placed));
        }
    }
}

// Translated from 00692870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 8-byte element (no Xbox PDB name): a word set to 0
/// and a `u16` at `+4` set to `0xFFFF`. Returns `this`.
pub fn fn_00692870(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u16(this.addr().wrapping_add(4), 0xffff);
    this
}

// Translated from 006928a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` 0x28-byte elements from `source` to `destination`, element
/// by element with `memmove`: first to last when the destination is below
/// the source, last to first when it is above, nothing when they are equal.
/// `this` (the array) is not read.
pub fn fn_006928a0(
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
            let offset = index.wrapping_mul(MOVED_ELEMENT_SIZE);
            e.call(
                MEMORY_MOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    MOVED_ELEMENT_SIZE
                ],
            );
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let offset = (index as u32).wrapping_mul(MOVED_ELEMENT_SIZE);
            e.call(
                MEMORY_MOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    MOVED_ELEMENT_SIZE
                ],
            );
            index = index.wrapping_sub(1);
        }
    }
}

// Translated from 00692940 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Thunk: subtracts 0x1C from `this` (the address of a secondary base) and
/// jumps to the `NavMesh` scalar deleting destructor `0068ed30`, which
/// takes the same `flags` word. Gives back what the destructor gives back.
/// The map has no name for it.
pub fn fn_00692940(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(
        NAV_MESH_DESTROY,
        &args![this.addr().wrapping_sub(0x1c), flags],
    )
    .ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006927d0, fn_006927d0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x00692870, fn_00692870(Ptr) -> Ptr),
        entry!(0x006928a0, fn_006928a0(Ptr<BSSimpleArray>, u32, u32, u32)),
        entry!(0x00692940, fn_00692940(Ptr, u32) -> Ptr),
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
    fn element_constructor_sets_the_word_and_the_short() {
        let mut e = Engine::new();
        let element = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(element.addr(), 7);
        e.mem.set_u16(element.addr() + 4, 3);
        let result = e.call(0x00692870, &args![element]).ptr::<()>();
        assert_eq!(result, element);
        assert_eq!(e.mem.u32(element.addr()), 0);
        assert_eq!(e.mem.u16(element.addr() + 4), 0xffff);
    }

    #[test]
    fn construct_elements_skips_a_slot_new_refuses() {
        let mut e = Engine::new();
        let block = e.mem.alloc(16);
        e.mem.set_u32(block, 5);
        e.mem.set_u32(block + 8, 5);
        e.register_double(PLACEMENT_NEW, move |_, a| Ret {
            eax: if a[1] == block { 0 } else { a[1] },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006927d0, &args![0u32, block, 2u32]);
        assert_eq!(
            calls(&e),
            vec![
                (0x006927d0, vec![0, block, 2]),
                (PLACEMENT_NEW, vec![8, block]),
                (PLACEMENT_NEW, vec![8, block + 8]),
            ]
        );
        // The first slot was refused and keeps its contents; the second was built.
        assert_eq!(e.mem.u32(block), 5);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u16(block + 12), 0xffff);
    }

    #[test]
    fn move_elements_goes_up_or_down_by_direction() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMORY_MOVE]);
        e.call_log = Some(vec![]);
        e.call(0x006928a0, &args![0u32, 0x1000u32, 0x2000u32, 2u32]);
        e.call(0x006928a0, &args![0u32, 0x2000u32, 0x1000u32, 2u32]);
        e.call(0x006928a0, &args![0u32, 0x1000u32, 0x1000u32, 2u32]);
        e.call(0x006928a0, &args![0u32, 0x2000u32, 0x1000u32, 0u32]);
        let moves: Vec<_> = calls(&e)
            .into_iter()
            .filter(|c| c.0 == MEMORY_MOVE)
            .collect();
        assert_eq!(
            moves,
            vec![
                (MEMORY_MOVE, vec![0x1000, 0x2000, 0x28]),
                (MEMORY_MOVE, vec![0x1028, 0x2028, 0x28]),
                (MEMORY_MOVE, vec![0x2028, 0x1028, 0x28]),
                (MEMORY_MOVE, vec![0x2000, 0x1000, 0x28]),
            ]
        );
    }

    #[test]
    fn thunk_continues_in_the_nav_mesh_destructor_with_an_adjusted_this() {
        let mut e = Engine::new();
        e.register(NAV_MESH_DESTROY, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x00692940, &args![0x5000u32, 1u32]).u32();
        assert_eq!(result, 0x5000 - 0x1c + 1);
        assert_eq!(calls(&e)[1], (NAV_MESH_DESTROY, vec![0x5000 - 0x1c, 1]));
    }
}
