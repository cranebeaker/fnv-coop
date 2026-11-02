//! `fallout shared/pathfinding/run_006f31f0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The array's "full" test (`00438b90`, `thiscall`, `bool` in `AL`), its
/// next-capacity rule (`009a3910`, `thiscall`) and its resize (`009f6540`,
/// `thiscall (this, capacity, count)`, `RET 8`); the engine map has no
/// names for them.
const IS_FULL: u32 = 0x0043_8b90;
const NEXT_CAPACITY: u32 = 0x009a_3910;
const RESIZE: u32 = 0x009f_6540;

// Translated from 006f31f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes room for one more element of a growable array (data +4, count
/// +8, capacity +0xC; the engine map has no name) and answers its index:
/// when the array is full, an empty one allocates 4 elements (vtable +4
/// with 4, the answer stored as the data) and a non-empty one is resized
/// to the next capacity (`009a3910`, `009f6540`); then the count grows by
/// one and the old count is answered.
pub fn fn_006f31f0(e: &mut Engine, this: Ptr) -> u32 {
    let array = this.addr();
    if e.call(IS_FULL, &args![this]).bool() {
        if e.mem.u32(array + 0xc) == 0 {
            let capacity = 4u32;
            let data = e.vcall(array, 4, &args![capacity]).u32();
            e.mem.set_u32(array + 4, data);
            e.mem.set_u32(array + 0xc, capacity);
        } else {
            let capacity = e.call(NEXT_CAPACITY, &args![this]).u32();
            let count = e.mem.u32(array + 8);
            e.call(RESIZE, &args![this, capacity, count]);
            e.mem.set_u32(array + 0xc, capacity);
        }
    }
    let count = e.mem.u32(array + 8).wrapping_add(1);
    e.mem.set_u32(array + 8, count);
    e.mem.u32(array + 8).wrapping_sub(1)
}

/// `006e6da0` (cdecl: size, memory): placement new, returns the memory.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `006240d0` (`thiscall`): the 16-byte element's constructor.
const ELEMENT_CONSTRUCT_006240D0: u32 = 0x0062_40d0;

// Translated from 006f3280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f3280 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs `count` 16-byte elements at `first` (`thiscall`, `ECX` not
/// read; first, count; `RET 8`; no name in the engine map): each through
/// placement new (`006e6da0` with 0x10) and, when that gives memory,
/// `006240d0`.
pub fn fn_006f3280(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = e
            .call(
                PLACEMENT_NEW,
                &args![0x10u32, (index << 4).wrapping_add(first)],
            )
            .u32();
        if place != 0 {
            e.call(ELEMENT_CONSTRUCT_006240D0, &args![place]);
        }
        index += 1;
    }
}

// Translated from 006f3320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the array (`thiscall`; reserve, size; `RET 8`; no name in
/// the engine map): data, count and capacity cleared; the reserve raised
/// to `size` (unsigned); with a non-zero reserve the data comes from the
/// array's virtual slot 4 and the capacity is the reserve; with a non-zero
/// size that many elements are constructed ([`fn_006f3280`]) and the count
/// is `size`.
pub fn fn_006f3320(e: &mut Engine, this: Ptr, reserve: u32, size: u32) {
    let array = this.addr();
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0);
    e.mem.set_u32(array + 0xc, 0);
    let reserve = if size > reserve { size } else { reserve };
    if reserve > 0 {
        let data = e.vcall(array, 4, &args![reserve]).u32();
        e.mem.set_u32(array + 4, data);
        e.mem.set_u32(array + 0xc, reserve);
    }
    if size > 0 {
        let data = e.mem.u32(array + 4);
        fn_006f3280(e, this, data, size);
        e.mem.set_u32(array + 8, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_31f0, fn_006f31f0(Ptr) -> u32),
        entry!(0x006f_3280, fn_006f3280(Ptr, u32, u32)),
        entry!(0x006f_3320, fn_006f3320(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_grows_a_full_array_and_answers_the_new_index() {
        const ALLOCATE: u32 = 0x0300_0000;
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, a| (0x9000 + a[1]).into_ret());
        e.register(NEXT_CAPACITY, |e, a| (2 * e.mem.u32(a[0] + 0xc)).into_ret());
        e.register(RESIZE, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        // Not full: only the count.
        e.register(IS_FULL, |_, _| Ret::default());
        e.mem.set_u32(this + 8, 2);
        assert_eq!(e.call(0x006f_31f0, &args![this]).u32(), 2);
        assert_eq!(e.mem.u32(this + 8), 3);
        // Full and empty: four elements allocated.
        e.register(IS_FULL, |_, _| 1u32.into_ret());
        e.mem.set_u32(this + 8, 0);
        assert_eq!(e.call(0x006f_31f0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 0x9004);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        // Full with a capacity: resized to the next one.
        e.mem.set_u32(this + 8, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_31f0, &args![this]).u32(), 4);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == RESIZE && call.1 == vec![this, 8, 4]));
    }

    #[test]
    fn fn_006f3320_reserves_and_constructs() {
        let mut e = Engine::new();
        e.register(0x0f00_0004, |e, a| e.mem.alloc(a[1] * 0x10).into_ret());
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004]);
        let mut placed = 0u32;
        e.register_double(PLACEMENT_NEW, move |_, a| {
            placed += 1;
            (if placed == 2 { 0 } else { a[1] }).into_ret()
        });
        e.register(ELEMENT_CONSTRUCT_006240D0, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0f00_1000);
        e.mem.set_u32(this + 8, 9);
        // Reserve 1, size 2: the reserve is raised to 2.
        e.call_log = Some(vec![]);
        e.call(0x006f_3320, &args![this, 1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        let data = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let calls = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(calls(0x0f00_0004), vec![vec![this, 2]]);
        assert_eq!(
            calls(PLACEMENT_NEW),
            vec![vec![0x10, data], vec![0x10, data + 0x10]]
        );
        // The second placement gives nothing.
        assert_eq!(calls(ELEMENT_CONSTRUCT_006240D0), vec![vec![data]]);
        // Nothing at all: everything cleared, no calls.
        e.call_log = Some(vec![]);
        e.call(0x006f_3320, &args![this, 0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(
            [
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ],
            [0, 0, 0]
        );
    }
}
