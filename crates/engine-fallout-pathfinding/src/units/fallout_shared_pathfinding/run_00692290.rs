//! `fallout shared/pathfinding/run_00692290` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Array helpers of the path-finding code. The header these functions use
//! has a vtable at +0 (slot 1, `+0x4`, is called with an element count and
//! its result kept at +4, the storage), the element count at +8 and the
//! capacity at +0xc. The engine map gives none of them a name.

#[allow(unused_imports)]
use crate::prelude::*;

/// Virtual slot `0x4` of the array: called with an element count (one word);
/// its result becomes the storage.
const SLOT_ALLOCATE: u32 = 0x4;
/// `005ef650` (`ECX` = array; storage, count; `RET 8`): called with the storage and
/// the count.
const CONSTRUCT_ELEMENTS_005EF650: u32 = 0x005e_f650;
/// `006927d0` (`ECX` = array; storage, count; `RET 8`): called with the storage and
/// the count.
const CONSTRUCT_ELEMENTS_006927D0: u32 = 0x0069_27d0;
/// `006e6da0` (cdecl: size, memory): placement new, returns the memory.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `00692870` (`ECX` = the element's +4).
const ELEMENT_TAIL_CONSTRUCT: u32 = 0x0069_2870;
/// `memmove` (`00ec7230`, cdecl: destination, source, size).
const MEMMOVE: u32 = 0x00ec_7230;
/// `00438b90` (`ECX` = array): a byte; nonzero makes room.
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// `009a3910` (`ECX` = array): the capacity to grow to.
const NEXT_CAPACITY: u32 = 0x009a_3910;
/// `005efa80` (`ECX` = array; capacity, count; `RET 8`).
const GROW: u32 = 0x005e_fa80;
/// `006815c0` (`ECX` = object).
const PAIR_HALF_CONSTRUCT: u32 = 0x0068_15c0;

/// The shared body of `00692290`, `006923f0` and `00692520`: empties the
/// header, allocates storage for the larger of the capacity and the count
/// (virtual slot 4) when that is not zero, then runs `construct` on the storage when the count is not zero.
fn initialise(
    e: &mut Engine,
    this: Ptr,
    capacity: u32,
    count: u32,
    construct: impl FnOnce(&mut Engine, u32),
) {
    let base = this.addr();
    e.mem.set_u32(base + 0x4, 0);
    e.mem.set_u32(base + 0x8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let mut capacity = capacity;
    if count > capacity {
        capacity = count;
    }
    if capacity != 0 {
        let storage = e.vcall(base, SLOT_ALLOCATE, &args![capacity]).u32();
        e.mem.set_u32(base + 0x4, storage);
        e.mem.set_u32(base + 0xc, capacity);
    }
    if count != 0 {
        let storage = e.mem.u32(base + 0x4);
        construct(e, storage);
        e.mem.set_u32(base + 0x8, count);
    }
}

// Translated from 00692290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the array (`thiscall`; capacity, count; `RET 8`): storage for
/// the larger of the two, then `005ef650` with the storage and the count.
pub fn fn_00692290(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    initialise(e, this, capacity, count, |e, storage| {
        e.call(CONSTRUCT_ELEMENTS_005EF650, &args![this, storage, count]);
    });
}

// Translated from 00692320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00692320 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs `count` 12-byte elements at `storage` (`thiscall`, `ECX` not
/// read; storage, count; `RET 8`): each through placement new (`006e6da0`
/// with 12) and, when that gives memory, `006923c0`.
pub fn fn_00692320(e: &mut Engine, _unused_0: Ptr, storage: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let memory = e
            .call(
                PLACEMENT_NEW,
                &args![0xcu32, index.wrapping_mul(0xc).wrapping_add(storage)],
            )
            .u32();
        if memory != 0 {
            fn_006923c0(e, Ptr::new(memory));
        }
        index += 1;
    }
}

// Translated from 006923c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a 12-byte element: `00692870` on its +4, then the word at +0
/// is -1. Returns `this`.
pub fn fn_006923c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ELEMENT_TAIL_CONSTRUCT, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), 0xffff_ffff);
    this
}

// Translated from 006923f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the array (`thiscall`; capacity, count; `RET 8`): storage for
/// the larger of the two, then `count` 12-byte elements constructed by
/// `00692320`.
pub fn fn_006923f0(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    initialise(e, this, capacity, count, |e, storage| {
        fn_00692320(e, this, storage, count);
    });
}

// Translated from 00692480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` 8-byte elements from `source` to `destination`
/// (`thiscall`, `ECX` not read; `RET 0xc`), one `memmove` of 8 bytes each:
/// first to last when the destination is below the source, last to first
/// (a signed index, so nothing for a count above 0x8000_0000) when it is
/// above, nothing when they are the same.
pub fn fn_00692480(e: &mut Engine, _unused_0: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index = 0u32;
        while index < count {
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(index.wrapping_mul(8)),
                    source.wrapping_add(index.wrapping_mul(8)),
                    8u32
                ],
            );
            index += 1;
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let offset = (index as u32).wrapping_mul(8);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(offset),
                    source.wrapping_add(offset),
                    8u32
                ],
            );
            index -= 1;
        }
    }
}

// Translated from 00692520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the array (`thiscall`; capacity, count; `RET 8`): storage for
/// the larger of the two, then `006927d0` with the storage and the count.
pub fn fn_00692520(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    initialise(e, this, capacity, count, |e, storage| {
        e.call(CONSTRUCT_ELEMENTS_006927D0, &args![this, storage, count]);
    });
}

// Translated from 006925b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes room for one more element and returns its index (the old count):
/// when `00438b90` reports true, an empty array gets storage for 4
/// (virtual slot 4) and capacity 4, any other grows to `009a3910`'s capacity
/// (`005efa80` with the capacity and the count); then the count goes up by
/// one.
pub fn fn_006925b0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        if e.mem.u32(base + 0xc) == 0 {
            let storage = e.vcall(base, SLOT_ALLOCATE, &args![4u32]).u32();
            e.mem.set_u32(base + 0x4, storage);
            e.mem.set_u32(base + 0xc, 4);
        } else {
            let capacity = e.call(NEXT_CAPACITY, &args![this]).u32();
            let count = e.mem.u32(base + 0x8);
            e.call(GROW, &args![this, capacity, count]);
            e.mem.set_u32(base + 0xc, capacity);
        }
    }
    let count = e.mem.u32(base + 0x8).wrapping_add(1);
    e.mem.set_u32(base + 0x8, count);
    e.mem.u32(base + 0x8).wrapping_sub(1)
}

// Translated from 00692710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `006815c0` on `this` and on `this + 0xc`; returns `this`.
pub fn fn_00692710(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PAIR_HALF_CONSTRUCT, &args![this]);
    e.call(PAIR_HALF_CONSTRUCT, &args![this.addr() + 0xc]);
    this
}

// Translated from 00692640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00692640 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs `count` 0x28-byte elements at `storage` (`thiscall`, `ECX` not
/// read; storage, count; `RET 8`): each through placement new (`006e6da0`
/// with 0x28) and, when that gives memory, `006926e0`.
pub fn fn_00692640(e: &mut Engine, _unused_0: Ptr, storage: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let memory = e
            .call(
                PLACEMENT_NEW,
                &args![0x28u32, index.wrapping_mul(0x28).wrapping_add(storage)],
            )
            .u32();
        if memory != 0 {
            fn_006926e0(e, Ptr::new(memory));
        }
        index += 1;
    }
}

// Translated from 006926e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a 0x28-byte element: `00692710` on it, then the 16-bit word
/// at +0x24 is 0xFFFF. Returns `this`.
pub fn fn_006926e0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00692710(e, this);
    e.mem.set_u16(this.addr() + 0x24, 0xffff);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069_2290, fn_00692290(Ptr, u32, u32)),
        entry!(0x0069_2320, fn_00692320(Ptr, u32, u32)),
        entry!(0x0069_23c0, fn_006923c0(Ptr) -> Ptr),
        entry!(0x0069_23f0, fn_006923f0(Ptr, u32, u32)),
        entry!(0x0069_2480, fn_00692480(Ptr, u32, u32, u32)),
        entry!(0x0069_2520, fn_00692520(Ptr, u32, u32)),
        entry!(0x0069_25b0, fn_006925b0(Ptr) -> u32),
        entry!(0x0069_2710, fn_00692710(Ptr) -> Ptr),
        entry!(0x0069_2640, fn_00692640(Ptr, u32, u32)),
        entry!(0x0069_26e0, fn_006926e0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    /// An array header whose slot 4 (`0f00_1000`) hands out 0x9000.
    fn array(e: &mut Engine) -> u32 {
        e.register(0x0f00_1000, |_, _| ret(0x9000));
        e.put_vtable(0x0f00_0000, &[0, 0x0f00_1000]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0f00_0000);
        this
    }

    fn header(e: &Engine, this: u32) -> [u32; 3] {
        [
            e.mem.u32(this + 4),
            e.mem.u32(this + 8),
            e.mem.u32(this + 0xc),
        ]
    }

    #[test]
    fn initialisers_allocate_the_larger_and_construct_the_count() {
        for (addr, constructor) in [
            (0x0069_2290, CONSTRUCT_ELEMENTS_005EF650),
            (0x0069_2520, CONSTRUCT_ELEMENTS_006927D0),
        ] {
            let mut e = Engine::new();
            e.register(constructor, |_, _| ret(0));
            let this = array(&mut e);
            let (_, log) = run(&mut e, addr, &[this, 2, 5]);
            assert_eq!(
                log,
                vec![
                    (0x0f00_1000, vec![this, 5]),
                    (constructor, vec![this, 0x9000, 5])
                ]
            );
            assert_eq!(header(&e, this), [0x9000, 5, 5]);
            let (_, log) = run(&mut e, addr, &[this, 3, 0]);
            assert_eq!(log, vec![(0x0f00_1000, vec![this, 3])]);
            assert_eq!(header(&e, this), [0x9000, 0, 3]);
            e.mem.set_u32(this + 4, 0x1234);
            let (_, log) = run(&mut e, addr, &[this, 0, 0]);
            assert!(log.is_empty());
            assert_eq!(header(&e, this), [0, 0, 0]);
        }
    }

    #[test]
    fn at_006923f0_constructs_twelve_byte_elements() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| ret(a[1]));
        e.register(ELEMENT_TAIL_CONSTRUCT, |_, _| ret(0));
        e.register(0x0f00_1000, |e, a| ret(e.mem.alloc(a[1] * 0xc)));
        e.put_vtable(0x0f00_0000, &[0, 0x0f00_1000]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0f00_0000);
        let (_, log) = run(&mut e, 0x0069_23f0, &[this, 0, 2]);
        let storage = e.mem.u32(this + 4);
        assert_eq!(header(&e, this), [storage, 2, 2]);
        assert_eq!(
            log,
            vec![
                (0x0f00_1000, vec![this, 2]),
                (PLACEMENT_NEW, vec![0xc, storage]),
                (ELEMENT_TAIL_CONSTRUCT, vec![storage + 4]),
                (PLACEMENT_NEW, vec![0xc, storage + 0xc]),
                (ELEMENT_TAIL_CONSTRUCT, vec![storage + 0x10]),
            ]
        );
        assert_eq!(e.mem.u32(storage), 0xffff_ffff);
        assert_eq!(e.mem.u32(storage + 0xc), 0xffff_ffff);
    }

    #[test]
    fn at_00692320_skips_a_failed_placement() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, _| ret(0));
        e.register(ELEMENT_TAIL_CONSTRUCT, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0069_2320, &[0, 0x9000, 1]);
        assert_eq!(log, vec![(PLACEMENT_NEW, vec![0xc, 0x9000])]);
    }

    #[test]
    fn at_00692480_moves_in_the_safe_direction() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0069_2480, &[0, 0x1000, 0x2000, 2]);
        assert_eq!(
            log,
            vec![
                (MEMMOVE, vec![0x1000, 0x2000, 8]),
                (MEMMOVE, vec![0x1008, 0x2008, 8]),
            ]
        );
        let (_, log) = run(&mut e, 0x0069_2480, &[0, 0x2000, 0x1000, 2]);
        assert_eq!(
            log,
            vec![
                (MEMMOVE, vec![0x2008, 0x1008, 8]),
                (MEMMOVE, vec![0x2000, 0x1000, 8]),
            ]
        );
        let (_, log) = run(&mut e, 0x0069_2480, &[0, 0x2000, 0x2000, 2]);
        assert!(log.is_empty());
        let (_, log) = run(&mut e, 0x0069_2480, &[0, 0x2000, 0x1000, 0]);
        assert!(log.is_empty());
        // A count whose last index is negative as a signed word moves nothing.
        let (_, log) = run(&mut e, 0x0069_2480, &[0, 0x2000, 0x1000, 0x8000_0001]);
        assert!(log.is_empty());
    }

    #[test]
    fn at_006925b0_grows_when_full() {
        let mut e = Engine::new();
        e.register(NEXT_CAPACITY, |_, _| ret(8));
        e.register(GROW, |_, _| ret(0));
        e.register(ARRAY_IS_FULL, |_, _| ret(0x100));
        let this = array(&mut e);
        e.mem.set_u32(this + 8, 3);
        let (result, log) = run(&mut e, 0x0069_25b0, &[this]);
        assert_eq!(result.u32(), 3);
        assert_eq!(log, vec![(ARRAY_IS_FULL, vec![this])]);
        e.register(ARRAY_IS_FULL, |_, _| ret(1));
        e.mem.set_u32(this + 8, 0);
        let (result, log) = run(&mut e, 0x0069_25b0, &[this]);
        assert_eq!(result.u32(), 0);
        assert_eq!(
            log,
            vec![(ARRAY_IS_FULL, vec![this]), (0x0f00_1000, vec![this, 4])]
        );
        assert_eq!(header(&e, this), [0x9000, 1, 4]);
        let (result, log) = run(&mut e, 0x0069_25b0, &[this]);
        assert_eq!(result.u32(), 1);
        assert_eq!(
            log,
            vec![
                (ARRAY_IS_FULL, vec![this]),
                (NEXT_CAPACITY, vec![this]),
                (GROW, vec![this, 8, 1]),
            ]
        );
        assert_eq!(header(&e, this), [0x9000, 2, 8]);
    }

    #[test]
    fn at_00692710_constructs_both_halves() {
        let mut e = Engine::new();
        e.register(PAIR_HALF_CONSTRUCT, |_, _| ret(0));
        let (result, log) = run(&mut e, 0x0069_2710, &[0x4000]);
        assert_eq!(result.u32(), 0x4000);
        assert_eq!(
            log,
            vec![
                (PAIR_HALF_CONSTRUCT, vec![0x4000]),
                (PAIR_HALF_CONSTRUCT, vec![0x400c]),
            ]
        );
    }

    #[test]
    fn at_00692640_constructs_0x28_byte_elements_and_skips_failed_ones() {
        let mut e = Engine::new();
        e.register(PAIR_HALF_CONSTRUCT, |_, _| ret(0));
        e.register(PLACEMENT_NEW, |_, a| ret(a[1]));
        let storage = e.mem.alloc(0x50);
        let (_, log) = run(&mut e, 0x0069_2640, &[0, storage, 2]);
        assert_eq!(
            log,
            vec![
                (PLACEMENT_NEW, vec![0x28, storage]),
                (PAIR_HALF_CONSTRUCT, vec![storage]),
                (PAIR_HALF_CONSTRUCT, vec![storage + 0xc]),
                (PLACEMENT_NEW, vec![0x28, storage + 0x28]),
                (PAIR_HALF_CONSTRUCT, vec![storage + 0x28]),
                (PAIR_HALF_CONSTRUCT, vec![storage + 0x34]),
            ]
        );
        assert_eq!(e.mem.u16(storage + 0x24), 0xffff);
        assert_eq!(e.mem.u16(storage + 0x4c), 0xffff);
        e.register(PLACEMENT_NEW, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0069_2640, &[0, storage, 1]);
        assert_eq!(log, vec![(PLACEMENT_NEW, vec![0x28, storage])]);
    }

    #[test]
    fn at_006926e0_constructs_both_halves_and_marks_the_word() {
        let mut e = Engine::new();
        e.register(PAIR_HALF_CONSTRUCT, |_, _| ret(0));
        let this = e.mem.alloc(0x28);
        let (result, log) = run(&mut e, 0x0069_26e0, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(
            log,
            vec![
                (PAIR_HALF_CONSTRUCT, vec![this]),
                (PAIR_HALF_CONSTRUCT, vec![this + 0xc]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x24), 0xffff);
    }
}
