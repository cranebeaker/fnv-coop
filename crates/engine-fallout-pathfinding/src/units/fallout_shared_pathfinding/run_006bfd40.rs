//! `fallout shared/pathfinding/run_006bfd40` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::BSSimpleArray;

/// The vtables of `BSSimpleArray<NavMeshObstacleVertex *, 1024>` and
/// `BSSimpleArray<NavMeshObstacleEdge *, 1024>` (their classes confirmed by the RTTI before
/// them), the array's `006b3eb0` (`thiscall`, two words; the destructors call it with 0, 0)
/// and its element fill `006bfdd0` (`thiscall`, `(buffer, count)`).
const OBSTACLE_VERTEX_ARRAY_VTABLE: u32 = 0x0106_c324;
const OBSTACLE_EDGE_ARRAY_VTABLE: u32 = 0x0106_c34c;
const ARRAY_RESET_006B3EB0: u32 = 0x006b_3eb0;
const ARRAY_FILL_006BFDD0: u32 = 0x006b_fdd0;

// Translated from 006bfe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray` initialisation (unnamed in the map; `thiscall`, `(reserve, size)`):
/// buffer, size and reserve cleared; with a non-zero reserve (raised to `size` when that is
/// larger) the buffer comes from the array's virtual slot +4 (`(reserve)`) and the reserve
/// is kept; with a non-zero size the elements are filled (`006bfdd0`) and the size kept.
pub fn fn_006bfe70(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let reserve = if size > reserve { size } else { reserve };
    if reserve != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(ARRAY_FILL_006BFDD0, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 006bff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshObstacleVertex *, 1024>`'s destructor body (unnamed in the map):
/// the class's vtable, then `006b3eb0(0, 0)`. Answers `this`.
pub fn fn_006bff00(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), OBSTACLE_VERTEX_ARRAY_VTABLE);
    e.call(ARRAY_RESET_006B3EB0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006bff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshObstacleEdge *, 1024>`'s destructor body (unnamed in the map): the
/// class's vtable, then `006b3eb0(0, 0)`. Answers `this`.
pub fn fn_006bff30(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), OBSTACLE_EDGE_ARRAY_VTABLE);
    e.call(ARRAY_RESET_006B3EB0, &args![this, 0u32, 0u32]);
    this
}

/// `00438b90` (`thiscall`): a byte; set when the array must grow.
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// `009a3910` (`thiscall`): the capacity to grow to.
const NEXT_CAPACITY: u32 = 0x009a_3910;
/// `006e6da0` (cdecl: size, memory): placement new, returns the memory.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `006bc8b0` (`thiscall`): the 0x1c-byte element's constructor.
const ELEMENT_CONSTRUCT_006BC8B0: u32 = 0x006b_c8b0;
/// `006a8500` (`thiscall`): run on the array once the elements have moved to
/// the new buffer.
const ARRAY_RELEASE_006A8500: u32 = 0x006a_8500;
/// `memmove` (`00ec7230`, cdecl: destination, source, size).
const MEMMOVE: u32 = 0x00ec_7230;

// Translated from 006bfd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes room for one more element of the `BSSimpleArray` and returns its
/// index (the old size) (`thiscall`, plain `RET`): when `00438b90` reports
/// true, an empty reserve gets a buffer of 4 (virtual slot 4) and reserve
/// 4, any other grows to `009a3910`'s capacity (`006bff60` with the
/// capacity and the size); then the size goes up by one. The engine map
/// gives it no name.
pub fn fn_006bfd40(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let buffer = e.vcall(this.addr(), 4, &args![4u32]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, 4);
        } else {
            let capacity = e.call(NEXT_CAPACITY, &args![this]).u32();
            let size = e.get(this, BSSimpleArray::iSize);
            fn_006bff60(e, this, capacity, size);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        }
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, size);
    e.get(this, BSSimpleArray::iSize).wrapping_sub(1)
}

// Translated from 006bfdd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bfdd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs `count` 0x1c-byte elements at `buffer` (`thiscall`, `ECX` not
/// read; buffer, count; `RET 8`): each through placement new (`006e6da0`
/// with 0x1c) and, when that gives memory, `006bc8b0`.
pub fn fn_006bfdd0(e: &mut Engine, _unused_0: Ptr, buffer: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let memory = e
            .call(
                PLACEMENT_NEW,
                &args![0x1cu32, index.wrapping_mul(0x1c).wrapping_add(buffer)],
            )
            .u32();
        if memory != 0 {
            e.call(ELEMENT_CONSTRUCT_006BC8B0, &args![memory]);
        }
        index += 1;
    }
}

// Translated from 006bff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the `BSSimpleArray` a buffer of `capacity` elements (`thiscall`;
/// capacity, size; `RET 8`): without a buffer, virtual slot 4 with the
/// capacity, which is also stored as the reserve; when `size` equals the
/// reserve, virtual slot 0xc with the old buffer and the capacity; else a
/// new buffer from slot 4, the `size` elements moved into it (`006c0010`),
/// `006a8500` on the array and the new buffer kept. The engine map gives it
/// no name.
pub fn fn_006bff60(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, size: u32) {
    if e.get(this, BSSimpleArray::pBuffer) == 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else if size == e.get(this, BSSimpleArray::iReservedSize) {
        let old = e.get(this, BSSimpleArray::pBuffer);
        let buffer = e.vcall(this.addr(), 0xc, &args![old, capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
    } else {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        fn_006c0010(e, this.cast(), buffer, old, size);
        e.call(ARRAY_RELEASE_006A8500, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, buffer);
    }
}

// Translated from 006c0010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` 0x1c-byte elements from `source` to `destination`
/// (`thiscall`, `ECX` not read; `RET 0xc`), one `memmove` of 0x1c bytes
/// each: first to last when the destination is below the source, last to
/// first (a signed index, so nothing for a count above 0x8000_0000) when it
/// is above, nothing when they are the same.
pub fn fn_006c0010(e: &mut Engine, _unused_0: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index = 0u32;
        while index < count {
            let offset = index.wrapping_mul(0x1c);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    0x1cu32
                ],
            );
            index += 1;
        }
    } else if source < destination {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let offset = (index as u32).wrapping_mul(0x1c);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    0x1cu32
                ],
            );
            index -= 1;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006bfe70, fn_006bfe70(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x006bff00,
            fn_006bff00(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006bff30,
            fn_006bff30(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006b_fd40, fn_006bfd40(Ptr<BSSimpleArray>) -> u32),
        entry!(0x006b_fdd0, fn_006bfdd0(Ptr, u32, u32)),
        entry!(0x006b_ff60, fn_006bff60(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006c_0010, fn_006c0010(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_006bfe70_reserves_and_fills() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0090_0004, |_, a| Ret {
            eax: 0x7000 + a[1],
            ..Ret::default()
        });
        e.register(ARRAY_FILL_006BFDD0, |_, _| Ret::default());
        e.put_vtable(0x0090_1000, &[0, 0x0090_0004]);
        let this = e.new_object::<BSSimpleArray>();
        e.mem.set_u32(this.addr(), 0x0090_1000);
        e.set(this, BSSimpleArray::iSize, 9);
        e.call(0x006bfe70, &args![this, 0u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert!(calls_at(&e, 0x0090_0004).is_empty());
        // The reserve is raised to the size.
        e.call(0x006bfe70, &args![this, 2u32, 5u32]);
        assert_eq!(calls_at(&e, 0x0090_0004), vec![vec![this.addr(), 5]]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x7005);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 5);
        assert_eq!(
            calls_at(&e, ARRAY_FILL_006BFDD0),
            vec![vec![this.addr(), 0x7005, 5]]
        );
        // Reserve only.
        e.call(0x006bfe70, &args![this, 8u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert_eq!(calls_at(&e, ARRAY_FILL_006BFDD0).len(), 1);
    }

    #[test]
    fn obstacle_array_destructor_bodies_reset_the_array() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ARRAY_RESET_006B3EB0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x006bff00, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), OBSTACLE_VERTEX_ARRAY_VTABLE);
        assert_eq!(e.call(0x006bff30, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), OBSTACLE_EDGE_ARRAY_VTABLE);
        assert_eq!(
            calls_at(&e, ARRAY_RESET_006B3EB0),
            vec![vec![this, 0, 0]; 2]
        );
    }

    /// An array whose slot 4 (`0f00_0004`) hands out `0x9000 + count` and
    /// slot 0xc (`0f00_000c`) `0xa000 + count`.
    fn growing_array(e: &mut Engine) -> Ptr<BSSimpleArray> {
        e.register(0x0f00_0004, |_, a| Ret {
            eax: 0x9000 + a[1],
            ..Ret::default()
        });
        e.register(0x0f00_000c, |_, a| Ret {
            eax: 0xa000 + a[2],
            ..Ret::default()
        });
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004, 0, 0x0f00_000c]);
        let this = e.new_object::<BSSimpleArray>();
        e.mem.set_u32(this.addr(), 0x0f00_1000);
        this
    }

    fn log_without(e: &mut Engine, addr: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect()
    }

    #[test]
    fn fn_006bfd40_grows_when_full() {
        let mut e = Engine::new();
        let this = growing_array(&mut e);
        e.register(ARRAY_IS_FULL, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        e.register(NEXT_CAPACITY, |_, _| Ret {
            eax: 8,
            ..Ret::default()
        });
        e.set(this, BSSimpleArray::iSize, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_fd40, &args![this]).u32(), 3);
        assert_eq!(
            log_without(&mut e, 0x006b_fd40),
            vec![(ARRAY_IS_FULL, vec![this.addr()])]
        );
        // Full with no reserve: a buffer of 4.
        e.register(ARRAY_IS_FULL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.set(this, BSSimpleArray::iSize, 0);
        assert_eq!(e.call(0x006b_fd40, &args![this]).u32(), 0);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x9004);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 4);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 1);
        // Full with a reserve: grows through 006bff60 (size 1 is not the
        // reserve 4, so a new buffer and a move).
        e.register(MEMMOVE, |_, _| Ret::default());
        e.register(ARRAY_RELEASE_006A8500, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_fd40, &args![this]).u32(), 1);
        let log = log_without(&mut e, 0x006b_fd40);
        assert_eq!(
            log,
            vec![
                (ARRAY_IS_FULL, vec![this.addr()]),
                (NEXT_CAPACITY, vec![this.addr()]),
                (0x0f00_0004, vec![this.addr(), 8]),
                (MEMMOVE, vec![0x9008, 0x9004, 0x1c]),
                (ARRAY_RELEASE_006A8500, vec![this.addr()]),
            ]
        );
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x9008);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 2);
    }

    #[test]
    fn fn_006bfdd0_constructs_each_element() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: if a[1] == 0x901c { 0 } else { a[1] },
            ..Ret::default()
        });
        e.register(ELEMENT_CONSTRUCT_006BC8B0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006b_fdd0, &args![0u32, 0x9000u32, 3u32]);
        assert_eq!(
            calls_at(&e, PLACEMENT_NEW),
            vec![vec![0x1c, 0x9000], vec![0x1c, 0x901c], vec![0x1c, 0x9038]]
        );
        assert_eq!(
            calls_at(&e, ELEMENT_CONSTRUCT_006BC8B0),
            vec![vec![0x9000], vec![0x9038]]
        );
    }

    #[test]
    fn fn_006bff60_allocates_reallocates_or_moves() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        e.register(ARRAY_RELEASE_006A8500, |_, _| Ret::default());
        let this = growing_array(&mut e);
        // No buffer: slot 4, the capacity becomes the reserve.
        e.call_log = Some(vec![]);
        e.call(0x006b_ff60, &args![this, 6u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x9006);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 6);
        // The size equals the reserve: slot 0xc with the old buffer.
        e.call_log = Some(vec![]);
        e.call(0x006b_ff60, &args![this, 9u32, 6u32]);
        assert_eq!(
            log_without(&mut e, 0x006b_ff60),
            vec![(0x0f00_000c, vec![this.addr(), 0x9006, 9])]
        );
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0xa009);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 6);
        // Otherwise: a new buffer, the move, the release.
        e.call_log = Some(vec![]);
        e.call(0x006b_ff60, &args![this, 0x10u32, 2u32]);
        assert_eq!(
            log_without(&mut e, 0x006b_ff60),
            vec![
                (0x0f00_0004, vec![this.addr(), 0x10]),
                (MEMMOVE, vec![0x9010, 0xa009, 0x1c]),
                (MEMMOVE, vec![0x902c, 0xa025, 0x1c]),
                (ARRAY_RELEASE_006A8500, vec![this.addr()]),
            ]
        );
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x9010);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 6);
    }

    #[test]
    fn fn_006c0010_moves_in_the_safe_direction() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006c_0010, &args![0u32, 0x1000u32, 0x2000u32, 2u32]);
        assert_eq!(
            calls_at(&e, MEMMOVE),
            vec![vec![0x1000, 0x2000, 0x1c], vec![0x101c, 0x201c, 0x1c]]
        );
        e.call_log = Some(vec![]);
        e.call(0x006c_0010, &args![0u32, 0x2000u32, 0x1000u32, 2u32]);
        assert_eq!(
            calls_at(&e, MEMMOVE),
            vec![vec![0x201c, 0x101c, 0x1c], vec![0x2000, 0x1000, 0x1c]]
        );
        for (destination, source, count) in [
            (0x2000u32, 0x2000u32, 2u32),
            (0x2000, 0x1000, 0),
            (0x2000, 0x1000, 0x8000_0001),
        ] {
            e.call_log = Some(vec![]);
            e.call(0x006c_0010, &args![0u32, destination, source, count]);
            assert!(calls_at(&e, MEMMOVE).is_empty());
        }
    }
}
