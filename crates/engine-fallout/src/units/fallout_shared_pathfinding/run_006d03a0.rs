//! `fallout shared/pathfinding/run_006d03a0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0` (`cdecl`: size, place): placement `new`, answers the place.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// Bytes per element of the arrays `fn_006d03d0` prepares.
const ELEMENT_SIZE: u32 = 0xc;

// Translated from 006d03d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs `count` 12-byte elements from `first` on (unnamed in the map;
/// `ECX` not read, two stack words, `RET 8`): for each element, placement
/// `new` (`006e6da0`, size 12) and nothing else, the element having no
/// constructor of its own.
pub fn fn_006d03d0(e: &mut Engine, _this: Ptr, first: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = index.wrapping_mul(ELEMENT_SIZE).wrapping_add(first);
        e.call(PLACEMENT_NEW, &args![ELEMENT_SIZE, place]);
        index += 1;
    }
}

/// `006b3f40` (`thiscall`, `RET 8`): the initialisation of the array
/// `006d03a0` constructs (size, used).
const ARRAY_A_INIT: u32 = 0x006b_3f40;
/// `0069b940` (`thiscall`, `RET 8`): the initialisation of the array
/// `006d0420` constructs (size, used).
const ARRAY_B_INIT: u32 = 0x0069_b940;
/// `00438b90` (`thiscall`): whether the array is full and must grow (`AL`).
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// `009a3910` (`thiscall`): the grown capacity.
const ARRAY_GROWN_CAPACITY: u32 = 0x009a_3910;
/// `006e12c0` (`thiscall`, `RET 8`): reallocates the array (capacity,
/// count).
const ARRAY_REALLOCATE: u32 = 0x006e_12c0;
/// `006e22d0` (`thiscall`): an element's destructor body.
const ELEMENT_DESTRUCT: u32 = 0x006e_22d0;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 006d03a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An array constructor (no name in the engine map): the vtable `0106c9b8`
/// and the empty initialisation `006b3f40(0, 0)`. Returns `this`.
pub fn fn_006d03a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106_c9b8);
    e.call(ARRAY_A_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006d0420 (decompiled, FalloutNV.exe 1.4.0.525)
/// An array constructor (no name in the engine map): the vtable `0106b1cc`
/// and the empty initialisation `0069b940(0, 0)`. Returns `this`.
pub fn fn_006d0420(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106_b1cc);
    e.call(ARRAY_B_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006d0450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the next slot of the array (no name in the engine map): when the
/// array must grow (`00438b90`), a first buffer of 4 from slot +4 when it
/// has no capacity, otherwise the grown capacity (`009a3910`) through
/// `006e12c0` with the size; then the size (+8) goes up by one and the old
/// size is the answer.
pub fn fn_006d0450(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        if e.mem.u32(base + 0xc) == 0 {
            let capacity = 4u32;
            let buffer = e.vcall(base, 4, &args![capacity]).u32();
            e.mem.set_u32(base + 4, buffer);
            e.mem.set_u32(base + 0xc, capacity);
        } else {
            let capacity = e.call(ARRAY_GROWN_CAPACITY, &args![this]).u32();
            let size = e.mem.u32(base + 8);
            e.call(ARRAY_REALLOCATE, &args![this, capacity, size]);
            e.mem.set_u32(base + 0xc, capacity);
        }
    }
    let size = e.mem.u32(base + 8);
    e.mem.set_u32(base + 8, size.wrapping_add(1));
    e.mem.u32(base + 8).wrapping_sub(1)
}

// Translated from 006d0510 (decompiled, FalloutNV.exe 1.4.0.525)
/// An element's scalar deleting destructor (no name in the engine map): the
/// destructor body (`006e22d0`), then `00401030` when `flags & 1`. Returns
/// `this`.
pub fn fn_006d0510(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ELEMENT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// `BSSimpleArray<FastNavMeshTriHandle,1024>`'s vtable (`0106c9e0`, RTTI).
const FAST_TRI_HANDLE_ARRAY_VTABLE: u32 = 0x0106_c9e0;
/// `006dc440` (`thiscall`, `RET 8`): run with 0, 0 by the destructor.
const ARRAY_RESET_006DC440: u32 = 0x006d_c440;
/// `memmove` (`00ec7230`, cdecl: destination, source, size).
const MEMMOVE: u32 = 0x00ec_7230;
/// `006a0480` (`thiscall`): the 0x18-byte element's constructor.
const ELEMENT_CONSTRUCT_006A0480: u32 = 0x006a_0480;
/// `006a8500` (`thiscall`): run on the array once the elements have moved
/// to the new buffer.
const ARRAY_RELEASE_006A8500: u32 = 0x006a_8500;

/// Moves `count` elements of `stride` bytes from `source` to `destination`,
/// one `memmove` each: first to last when the destination is below the
/// source, last to first (a signed index, so nothing for a count above
/// 0x8000_0000) when it is above, nothing when they are the same.
fn move_elements(e: &mut Engine, destination: u32, source: u32, count: u32, stride: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index = 0u32;
        while index < count {
            let offset = index.wrapping_mul(stride);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    stride
                ],
            );
            index += 1;
        }
    } else if source < destination {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let offset = (index as u32).wrapping_mul(stride);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    stride
                ],
            );
            index -= 1;
        }
    }
}

// Translated from 006d04e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<FastNavMeshTriHandle,1024>`'s destructor body (no name in
/// the engine map; `thiscall`, plain `RET`): the class's vtable, then
/// `006dc440(0, 0)`. Returns `this`.
pub fn fn_006d04e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), FAST_TRI_HANDLE_ARRAY_VTABLE);
    e.call(ARRAY_RESET_006DC440, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006d0540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` 0x30-byte elements from `source` to `destination` in the
/// safe direction, one `memmove` each (`thiscall`, `ECX` not read;
/// `RET 0xc`; no name in the engine map).
pub fn fn_006d0540(e: &mut Engine, _unused_0: Ptr, destination: u32, source: u32, count: u32) {
    move_elements(e, destination, source, count, 0x30);
}

// Translated from 006d05e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006d05e0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs `count` 0x18-byte elements at `first` (`thiscall`, `ECX` not
/// read; first, count; `RET 8`; no name in the engine map): each through
/// placement new (`006e6da0` with 0x18) and, when that gives memory,
/// `006a0480`.
pub fn fn_006d05e0(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = e
            .call(
                PLACEMENT_NEW,
                &args![0x18u32, index.wrapping_mul(0x18).wrapping_add(first)],
            )
            .u32();
        if place != 0 {
            e.call(ELEMENT_CONSTRUCT_006A0480, &args![place]);
        }
        index += 1;
    }
}

// Translated from 006d0680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the array of 0x18-byte elements (buffer `+4`, capacity `+0xc`) a
/// buffer of `capacity` (`thiscall`; capacity, count; `RET 8`; no name in
/// the engine map): without a buffer, virtual slot 4 with the capacity,
/// which is also stored; when `count` equals the capacity, slot 0xc with the
/// old buffer and the capacity; else a new buffer from slot 4, the `count`
/// elements moved (`006d0730`), `006a8500` on the array and the new buffer
/// kept.
pub fn fn_006d0680(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    } else if count == e.mem.u32(base + 0xc) {
        let old = e.mem.u32(base + 4);
        let buffer = e.vcall(base, 0xc, &args![old, capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
    } else {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        let old = e.mem.u32(base + 4);
        fn_006d0730(e, this, buffer, old, count);
        e.call(ARRAY_RELEASE_006A8500, &args![this]);
        e.mem.set_u32(base + 4, buffer);
    }
}

// Translated from 006d0730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` 0x18-byte elements from `source` to `destination` in the
/// safe direction, one `memmove` each (`thiscall`, `ECX` not read;
/// `RET 0xc`; no name in the engine map).
pub fn fn_006d0730(e: &mut Engine, _unused_0: Ptr, destination: u32, source: u32, count: u32) {
    move_elements(e, destination, source, count, 0x18);
}

// Translated from 006d07d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` bytes from `source` to `destination` in the safe
/// direction, one 1-byte `memmove` each (`thiscall`, `ECX` not read;
/// `RET 0xc`; no name in the engine map).
pub fn fn_006d07d0(e: &mut Engine, _unused_0: Ptr, destination: u32, source: u32, count: u32) {
    move_elements(e, destination, source, count, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006d03d0, fn_006d03d0(Ptr, u32, u32)),
        entry!(0x006d_03a0, fn_006d03a0(Ptr) -> Ptr),
        entry!(0x006d_0420, fn_006d0420(Ptr) -> Ptr),
        entry!(0x006d_0450, fn_006d0450(Ptr) -> u32),
        entry!(0x006d_0510, fn_006d0510(Ptr, u32) -> Ptr),
        entry!(0x006d_04e0, fn_006d04e0(Ptr) -> Ptr),
        entry!(0x006d_0540, fn_006d0540(Ptr, u32, u32, u32)),
        entry!(0x006d_05e0, fn_006d05e0(Ptr, u32, u32)),
        entry!(0x006d_0680, fn_006d0680(Ptr, u32, u32)),
        entry!(0x006d_0730, fn_006d0730(Ptr, u32, u32, u32)),
        entry!(0x006d_07d0, fn_006d07d0(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn places_each_of_the_elements() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = Ptr::<()>::new(0x0800_0000);
        e.call(0x006d_03d0, &args![this, 0x0900_0000u32, 3u32]);
        let places: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == PLACEMENT_NEW)
            .map(|c| c.1)
            .collect();
        assert_eq!(
            places,
            vec![
                vec![12, 0x0900_0000],
                vec![12, 0x0900_000c],
                vec![12, 0x0900_0018],
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x006d_03d0, &args![this, 0x0900_0000u32, 0u32]);
        let log = e.call_log.unwrap();
        assert!(log.iter().all(|c| c.0 != PLACEMENT_NEW));
    }
}

#[cfg(test)]
mod b039_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_006d03a0_sets_the_vtable_and_initialises() {
        let mut e = Engine::new();
        e.register(ARRAY_A_INIT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006d_03a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0106_c9b8);
        assert_eq!(calls_to(&e, ARRAY_A_INIT), vec![vec![this, 0, 0]]);
    }

    #[test]
    fn fn_006d0420_sets_the_vtable_and_initialises() {
        let mut e = Engine::new();
        e.register(ARRAY_B_INIT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006d_0420, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0106_b1cc);
        assert_eq!(calls_to(&e, ARRAY_B_INIT), vec![vec![this, 0, 0]]);
    }

    #[test]
    fn fn_006d0450_grows_when_full_and_answers_the_slot() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0, 0x0300_1000]);
        e.mem.set_u32(this, 0x0300_0000);
        e.register(0x0300_1000, |_, a| word(0x7000 + a[1]));
        e.register(ARRAY_IS_FULL, |e, a| {
            word(u32::from(e.mem.u32(a[0] + 8) == e.mem.u32(a[0] + 0xc)))
        });
        e.register(ARRAY_GROWN_CAPACITY, |e, a| word(e.mem.u32(a[0] + 0xc) * 2));
        e.register(ARRAY_REALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        // Empty: the first buffer of 4.
        assert_eq!(e.call(0x006d_0450, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 0x7004);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        for slot in 1..4 {
            assert_eq!(e.call(0x006d_0450, &args![this]).u32(), slot);
        }
        assert!(calls_to(&e, ARRAY_REALLOCATE).is_empty());
        // Full at 4: grows to 8.
        assert_eq!(e.call(0x006d_0450, &args![this]).u32(), 4);
        assert_eq!(calls_to(&e, ARRAY_REALLOCATE), vec![vec![this, 8, 4]]);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(e.mem.u32(this + 8), 5);
    }

    #[test]
    fn fn_006d0510_frees_with_bit_zero() {
        let mut e = Engine::new();
        e.register(ELEMENT_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006d_0510, &args![0x5000u32, 0u32]).u32(), 0x5000);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x006d_0510, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x5000]]);
        assert_eq!(calls_to(&e, ELEMENT_DESTRUCT).len(), 2);
    }

    #[test]
    fn fn_006d04e0_sets_the_vtable_and_resets() {
        let mut e = Engine::new();
        e.register(ARRAY_RESET_006DC440, |_, _| word(0));
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006d_04e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), FAST_TRI_HANDLE_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_RESET_006DC440), vec![vec![this, 0, 0]]);
    }

    #[test]
    fn element_moves_use_their_strides() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| word(0));
        for (addr, stride) in [
            (0x006d_0540u32, 0x30u32),
            (0x006d_0730, 0x18),
            (0x006d_07d0, 1),
        ] {
            e.call_log = Some(vec![]);
            e.call(addr, &args![0u32, 0x1000u32, 0x2000u32, 2u32]);
            assert_eq!(
                calls_to(&e, MEMMOVE),
                vec![
                    vec![0x1000, 0x2000, stride],
                    vec![0x1000 + stride, 0x2000 + stride, stride]
                ]
            );
            e.call_log = Some(vec![]);
            e.call(addr, &args![0u32, 0x2000u32, 0x1000u32, 2u32]);
            assert_eq!(
                calls_to(&e, MEMMOVE),
                vec![
                    vec![0x2000 + stride, 0x1000 + stride, stride],
                    vec![0x2000, 0x1000, stride]
                ]
            );
            for (destination, source, count) in [
                (0x2000u32, 0x2000u32, 2u32),
                (0x2000, 0x1000, 0),
                (0x2000, 0x1000, 0x8000_0001),
            ] {
                e.call_log = Some(vec![]);
                e.call(addr, &args![0u32, destination, source, count]);
                assert!(calls_to(&e, MEMMOVE).is_empty());
            }
        }
    }

    #[test]
    fn fn_006d05e0_constructs_where_the_placement_gives() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| {
            word(if a[1] == 0x9018 { 0 } else { a[1] })
        });
        e.register(ELEMENT_CONSTRUCT_006A0480, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x006d_05e0, &args![0u32, 0x9000u32, 3u32]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x18, 0x9000], vec![0x18, 0x9018], vec![0x18, 0x9030]]
        );
        assert_eq!(
            calls_to(&e, ELEMENT_CONSTRUCT_006A0480),
            vec![vec![0x9000], vec![0x9030]]
        );
    }

    #[test]
    fn fn_006d0680_allocates_reallocates_or_moves() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| word(0));
        e.register(ARRAY_RELEASE_006A8500, |_, _| word(0));
        e.register(0x0f00_0004, |_, a| word(0x9000 + a[1]));
        e.register(0x0f00_000c, |_, a| word(0xa000 + a[2]));
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004, 0, 0x0f00_000c]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0f00_1000);
        e.call(0x006d_0680, &args![this, 6u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x9006);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        e.call_log = Some(vec![]);
        e.call(0x006d_0680, &args![this, 9u32, 6u32]);
        assert_eq!(calls_to(&e, 0x0f00_000c), vec![vec![this, 0x9006, 9]]);
        assert_eq!(e.mem.u32(this + 4), 0xa009);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        e.call_log = Some(vec![]);
        e.call(0x006d_0680, &args![this, 0x10u32, 1u32]);
        assert_eq!(calls_to(&e, 0x0f00_0004), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, MEMMOVE), vec![vec![0x9010, 0xa009, 0x18]]);
        assert_eq!(calls_to(&e, ARRAY_RELEASE_006A8500), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), 0x9010);
    }
}
