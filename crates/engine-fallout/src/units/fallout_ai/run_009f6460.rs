//! `fallout/ai/run_009f6460` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three helpers of an array of 16-byte elements (no names in the engine
//! map): the element constructor (`009f6500`), the loop that builds a range
//! of them in place (`009f6460`) and the array's reallocation (`009f6540`,
//! the shape of a `BSSimpleArray` resize through the array's own vtable).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0` (`cdecl`: size, place): placement `new`, answers the place.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `006c8bd0` (`thiscall`, `RET 0xc`): `(array, new buffer, old buffer,
/// count)`, called when the array moves its elements to a new buffer.
const MOVE_ELEMENTS: u32 = 0x006c_8bd0;
/// `006a8500` (`thiscall`, no stack words): releases the array's current
/// buffer.
const FREE_BUFFER: u32 = 0x006a_8500;

// Translated from 009f6460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f6460 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Builds `count` 16-byte elements from `first` on (`thiscall`, `ECX` not
/// read, `RET 8`): for each index below `count` (unsigned) placement `new`
/// (`006e6da0`, size 0x10) and, when it gives an address, the element
/// constructor `009f6500` on it.
pub fn fn_009f6460(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = e
            .call(
                PLACEMENT_NEW,
                &args![0x10u32, (index << 4).wrapping_add(first)],
            )
            .u32();
        if place != 0 {
            fn_009f6500(e, Ptr::new(place));
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 009f6500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element constructor: stores 0, -1, 0, 0 in the four words of `this` and
/// returns `this`.
pub fn fn_009f6500(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0xffff_ffff);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    this
}

// Translated from 009f6540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reallocates the array (`thiscall`, `RET 8`; buffer at +4, capacity at
/// +0xc): with no buffer, the buffer is the array's virtual slot +4 called
/// with `capacity` and the capacity becomes `capacity`; when `count` equals
/// the current capacity, the buffer is slot +0xc called with the buffer and
/// `capacity`; otherwise slot +4 gives a new buffer, `006c8bd0` moves the
/// `count` elements over, `006a8500` frees the old buffer and the new one is
/// stored (the capacity word is left as it was).
pub fn fn_009f6540(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
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
        e.call(MOVE_ELEMENTS, &args![this, buffer, old, count]);
        e.call(FREE_BUFFER, &args![this]);
        e.mem.set_u32(base + 4, buffer);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_6460, fn_009f6460(Ptr, u32, u32)),
        entry!(0x009f_6500, fn_009f6500(Ptr) -> Ptr),
        entry!(0x009f_6540, fn_009f6540(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn fn_009f6460_builds_each_element_the_placement_gives() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x30);
        for i in 0..12 {
            e.mem.set_u32(block + 4 * i, 0x5555_5555);
        }
        // The placement of the second element answers null.
        let refused = block + 0x10;
        e.register_double(PLACEMENT_NEW, move |_, a| Ret {
            eax: if a[1] == refused { 0 } else { a[1] },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009f_6460, &args![0u32, block, 3u32]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![
                vec![0x10, block],
                vec![0x10, block + 0x10],
                vec![0x10, block + 0x20]
            ]
        );
        assert_eq!(e.mem.u32(block + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + 0x14), 0x5555_5555);
        assert_eq!(e.mem.u32(block + 0x24), 0xffff_ffff);
        // Count 0 places nothing.
        e.call_log = Some(vec![]);
        e.call(0x009f_6460, &args![0u32, block, 0u32]);
        assert!(calls_to(&e, PLACEMENT_NEW).is_empty());
    }

    #[test]
    fn fn_009f6500_sets_the_four_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(this + 4 * i, 0x5555_5555);
        }
        assert_eq!(e.call(0x009f_6500, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn fn_009f6540_takes_each_of_its_three_paths() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0, 0x0300_1000, 0, 0x0300_2000]);
        e.mem.set_u32(this, 0x0300_0000);
        e.register(0x0300_1000, |_, a| Ret {
            eax: 0x7000 + a[1],
            ..Ret::default()
        });
        e.register(0x0300_2000, |_, a| Ret {
            eax: a[1] + a[2],
            ..Ret::default()
        });
        e.register(MOVE_ELEMENTS, |_, _| Ret::default());
        e.register(FREE_BUFFER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        // No buffer: slot +4, capacity stored.
        e.call(0x009f_6540, &args![this, 8u32, 3u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7008);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        // Count equal to the capacity: slot +0xc with the old buffer.
        e.call(0x009f_6540, &args![this, 0x10u32, 8u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7018);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert!(calls_to(&e, MOVE_ELEMENTS).is_empty());
        // Otherwise: new buffer, move, free; the capacity stays.
        e.call(0x009f_6540, &args![this, 0x20u32, 5u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7020);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(
            calls_to(&e, MOVE_ELEMENTS),
            vec![vec![this, 0x7020, 0x7018, 5]]
        );
        assert_eq!(calls_to(&e, FREE_BUFFER), vec![vec![this]]);
    }
}
