//! `fallout shared/pathfinding/run_006cfe30` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006d0510` (unit `run_006d03a0`, `thiscall`, `RET 4`): an element's
/// scalar deleting destructor; called with flags 0 it only destroys.
const ELEMENT_DESTROY: u32 = 0x006d_0510;
/// `006d0540` (`thiscall`, `RET 0xc`): `(array, new buffer, old buffer,
/// count)`, moves the elements to a new buffer.
const MOVE_ELEMENTS: u32 = 0x006d_0540;
/// `006a8500` (`thiscall`): releases the array's current buffer.
const FREE_BUFFER: u32 = 0x006a_8500;
/// `006e6da0` (`cdecl`: size, place): placement `new`, answers the place.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `PathingLocation::PathingLocation` (engine map), `thiscall`.
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;

/// Destroys `count` elements of `stride` bytes from `first` on with
/// `006d0510(element, 0)`.
fn destroy_elements(e: &mut Engine, first: u32, count: u32, stride: u32) {
    let mut index = 0u32;
    while index < count {
        let element = index.wrapping_mul(stride).wrapping_add(first);
        e.call(ELEMENT_DESTROY, &args![element, 0u32]);
        index = index.wrapping_add(1);
    }
}

// Translated from 006cfe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `count` 0x30-byte elements (`VirtualPathingNode`s) from
/// `first` on (`thiscall`, `ECX` not read, `RET 8`; no name in the engine
/// map).
pub fn fn_006cfe30(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    destroy_elements(e, first, count, 0x30);
}

// Translated from 006cfe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reallocates the 0x30-byte-element array (`RET 8`; buffer at +4,
/// capacity at +0xc; no name in the engine map): with no buffer, the buffer
/// is the array's slot +4 with `capacity` and the capacity becomes
/// `capacity`; when `count` equals the capacity, slot +0xc reallocates the
/// buffer; otherwise slot +4 gives a new buffer, `006d0540` moves the
/// `count` elements, `006a8500` frees the old buffer and the new one is
/// stored.
pub fn fn_006cfe70(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
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

// Translated from 006cff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `count` 0x3c-byte elements from `first` on (`thiscall`, `ECX`
/// not read, `RET 8`; no name in the engine map).
pub fn fn_006cff20(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    destroy_elements(e, first, count, 0x3c);
}

// Translated from 006d00a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006d00a0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Constructs `count` `PathingLocation`s (0x28 bytes) from `first` on
/// (`thiscall`, `ECX` not read, `RET 8`; no name in the engine map):
/// placement `new` (`006e6da0`) and, when it gives an address, the
/// `PathingLocation` constructor (`006dcbb0`).
pub fn fn_006d00a0(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = e
            .call(
                PLACEMENT_NEW,
                &args![0x28u32, index.wrapping_mul(0x28).wrapping_add(first)],
            )
            .u32();
        if place != 0 {
            e.call(LOCATION_CONSTRUCT, &args![place]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 006d0140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PathingLocation` array's initialisation (`RET 8`; no name in the
/// engine map): clears the buffer, size and capacity; `size` is raised to
/// `used` (unsigned); with a non-zero size the buffer is slot +4 (allocate)
/// and the capacity `size`; with a non-zero `used` that many locations are
/// constructed (`006d00a0`) and the size is `used`.
pub fn fn_006d0140(e: &mut Engine, this: Ptr, size: u32, used: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let size = size.max(used);
    if size != 0 {
        let buffer = e.vcall(base, 4, &args![size]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, size);
    }
    if used != 0 {
        let buffer = e.mem.u32(base + 4);
        fn_006d00a0(e, this, buffer, used);
        e.mem.set_u32(base + 8, used);
    }
}

/// The callees of [`fn_006cff60`] (0x28-byte `PathingLocation`s) and
/// [`fn_006d01d0`] (0x18-byte elements), all `thiscall` on the array:
/// emptying it (`RET 4`: the byte), growing the buffer (`RET 8`: capacity,
/// size) and destroying a range (`RET 8`: first, count).
const EMPTY_006AC010: u32 = 0x006a_c010;
const GROW_005EFA80: u32 = 0x005e_fa80;
const DESTROY_RANGE_006AC070: u32 = 0x006a_c070;
const EMPTY_008454F0: u32 = 0x0084_54f0;
const GROW_006D0680: u32 = 0x006d_0680;
const CONSTRUCT_RANGE_006D05E0: u32 = 0x006d_05e0;
const VISIT_RANGE_0072BA80: u32 = 0x0072_ba80;

/// The shared body of [`fn_006cff60`] and [`fn_006d01d0`]: sets the size of
/// the array whose buffer is at `+4`, size at `+8` and capacity at `+0xc`
/// to `size`, with elements of `stride` bytes.
fn set_size(
    e: &mut Engine,
    this: Ptr,
    size: u32,
    shrink: u8,
    stride: u32,
    empty: u32,
    grow: u32,
    construct: fn(&mut Engine, Ptr, u32, u32),
    destroy: u32,
) {
    let base = this.addr();
    if size == 0 {
        e.call(empty, &args![this, u32::from(shrink)]);
    } else if size > e.mem.u32(base + 0xc) {
        if e.mem.u32(base + 0xc) == 0 {
            let buffer = e.vcall(base, 4, &args![size]).u32();
            e.mem.set_u32(base + 4, buffer);
            e.mem.set_u32(base + 0xc, size);
        } else {
            let count = e.mem.u32(base + 8);
            e.call(grow, &args![this, size, count]);
            e.mem.set_u32(base + 0xc, size);
        }
        let added = size.wrapping_sub(e.mem.u32(base + 8));
        let first = e
            .mem
            .u32(base + 8)
            .wrapping_mul(stride)
            .wrapping_add(e.mem.u32(base + 4));
        construct(e, this, first, added);
        e.mem.set_u32(base + 8, size);
    } else if size < e.mem.u32(base + 8) {
        let removed = e.mem.u32(base + 8).wrapping_sub(size);
        let first = size.wrapping_mul(stride).wrapping_add(e.mem.u32(base + 4));
        e.call(destroy, &args![this, first, removed]);
        e.mem.set_u32(base + 8, size);
        if shrink != 0 && size <= e.mem.u32(base + 0xc) >> 2 {
            e.call(grow, &args![this, size, size]);
            e.mem.set_u32(base + 0xc, size);
        }
    } else {
        let added = size.wrapping_sub(e.mem.u32(base + 8));
        let first = e
            .mem
            .u32(base + 8)
            .wrapping_mul(stride)
            .wrapping_add(e.mem.u32(base + 4));
        construct(e, this, first, added);
        e.mem.set_u32(base + 8, size);
    }
}

// Translated from 006cff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the size of the `PathingLocation` array (`thiscall`; size, a
/// byte; `RET 8`; no name in the engine map): size 0 empties it
/// (`006ac010` with the byte); a size above the capacity first gets a
/// buffer (slot +4, when the capacity is 0) or grows it (`005efa80` with
/// the size and the old count), the capacity becoming the size; growing
/// constructs the new locations (`006d00a0`); shrinking destroys the
/// dropped ones (`006ac070`) and, when the byte is set and the size is at
/// most a quarter of the capacity, reallocates to the size (`005efa80`).
pub fn fn_006cff60(e: &mut Engine, this: Ptr, size: u32, shrink: u8) {
    set_size(
        e,
        this,
        size,
        shrink,
        0x28,
        EMPTY_006AC010,
        GROW_005EFA80,
        fn_006d00a0,
        DESTROY_RANGE_006AC070,
    );
}

/// `006d05e0` by address, as [`set_size`] calls a constructor.
fn construct_range_006d05e0(e: &mut Engine, this: Ptr, first: u32, count: u32) {
    e.call(CONSTRUCT_RANGE_006D05E0, &args![this, first, count]);
}

// Translated from 006d01d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same for an array of 0x18-byte elements (`thiscall`; size, a byte;
/// `RET 8`; no name in the engine map): emptying through `008454f0`,
/// growing through `006d0680`, constructing through `006d05e0` and the
/// dropped range through `0072ba80`.
pub fn fn_006d01d0(e: &mut Engine, this: Ptr, size: u32, shrink: u8) {
    set_size(
        e,
        this,
        size,
        shrink,
        0x18,
        EMPTY_008454F0,
        GROW_006D0680,
        construct_range_006d05e0,
        VISIT_RANGE_0072BA80,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c_fe30, fn_006cfe30(Ptr, u32, u32)),
        entry!(0x006c_fe70, fn_006cfe70(Ptr, u32, u32)),
        entry!(0x006c_ff20, fn_006cff20(Ptr, u32, u32)),
        entry!(0x006d_00a0, fn_006d00a0(Ptr, u32, u32)),
        entry!(0x006d_0140, fn_006d0140(Ptr, u32, u32)),
        entry!(0x006c_ff60, fn_006cff60(Ptr, u32, u8)),
        entry!(0x006d_01d0, fn_006d01d0(Ptr, u32, u8)),
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_006cfe30_destroys_0x30_byte_elements() {
        let mut e = Engine::new();
        e.register(ELEMENT_DESTROY, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x006c_fe30, &args![0u32, 0x1000u32, 3u32]);
        assert_eq!(
            calls_to(&e, ELEMENT_DESTROY),
            vec![vec![0x1000, 0], vec![0x1030, 0], vec![0x1060, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x006c_fe30, &args![0u32, 0x1000u32, 0u32]);
        assert!(calls_to(&e, ELEMENT_DESTROY).is_empty());
    }

    #[test]
    fn fn_006cff20_destroys_0x3c_byte_elements() {
        let mut e = Engine::new();
        e.register(ELEMENT_DESTROY, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x006c_ff20, &args![0u32, 0x1000u32, 2u32]);
        assert_eq!(
            calls_to(&e, ELEMENT_DESTROY),
            vec![vec![0x1000, 0], vec![0x103c, 0]]
        );
    }

    #[test]
    fn fn_006cfe70_takes_each_of_its_three_paths() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0, 0x0300_1000, 0, 0x0300_2000]);
        e.mem.set_u32(this, 0x0300_0000);
        e.register(0x0300_1000, |_, a| word(0x7000 + a[1]));
        e.register(0x0300_2000, |_, a| word(a[1] + a[2]));
        e.register(MOVE_ELEMENTS, |_, _| Ret::default());
        e.register(FREE_BUFFER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006c_fe70, &args![this, 8u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7008);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        e.call(0x006c_fe70, &args![this, 0x10u32, 8u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7018);
        assert!(calls_to(&e, MOVE_ELEMENTS).is_empty());
        e.call(0x006c_fe70, &args![this, 0x20u32, 4u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7020);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(
            calls_to(&e, MOVE_ELEMENTS),
            vec![vec![this, 0x7020, 0x7018, 4]]
        );
        assert_eq!(calls_to(&e, FREE_BUFFER), vec![vec![this]]);
    }

    #[test]
    fn fn_006d00a0_constructs_where_the_placement_gives() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| {
            word(if a[1] == 0x1028 { 0 } else { a[1] })
        });
        e.register(LOCATION_CONSTRUCT, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x006d_00a0, &args![0u32, 0x1000u32, 3u32]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x28, 0x1000], vec![0x28, 0x1028], vec![0x28, 0x1050]]
        );
        assert_eq!(
            calls_to(&e, LOCATION_CONSTRUCT),
            vec![vec![0x1000], vec![0x1050]]
        );
    }

    #[test]
    fn fn_006d0140_allocates_and_constructs() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0, 0x0300_1000]);
        e.mem.set_u32(this, 0x0300_0000);
        e.register(0x0300_1000, |_, a| word(0x7000 + a[1]));
        e.register(PLACEMENT_NEW, |_, a| word(a[1]));
        e.register(LOCATION_CONSTRUCT, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x006d_0140, &args![this, 1u32, 2u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7002);
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(
            calls_to(&e, LOCATION_CONSTRUCT),
            vec![vec![0x7002], vec![0x702a]]
        );
        e.call_log = Some(vec![]);
        e.call(0x006d_0140, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert!(calls_to(&e, 0x0300_1000).is_empty());
    }

    /// An array with slot 4 (`0f00_0004`) handing out `0x9000 + count`,
    /// the given buffer, size and capacity, and doubles for every callee
    /// of the two size setters.
    fn sized_array(e: &mut Engine, buffer: u32, size: u32, capacity: u32) -> u32 {
        e.register(0x0f00_0004, |_, a| word(0x9000 + a[1]));
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004]);
        for callee in [
            EMPTY_006AC010,
            GROW_005EFA80,
            DESTROY_RANGE_006AC070,
            EMPTY_008454F0,
            GROW_006D0680,
            CONSTRUCT_RANGE_006D05E0,
            VISIT_RANGE_0072BA80,
            LOCATION_CONSTRUCT,
        ] {
            e.register(callee, |_, _| word(0));
        }
        e.register(PLACEMENT_NEW, |_, a| word(a[1]));
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0f00_1000);
        e.mem.set_u32(this + 4, buffer);
        e.mem.set_u32(this + 8, size);
        e.mem.set_u32(this + 0xc, capacity);
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
    fn fn_006cff60_empties_grows_and_shrinks() {
        let mut e = Engine::new();
        // Size 0: emptied with the byte (zero-extended).
        let this = sized_array(&mut e, 0x5000, 3, 4);
        e.call_log = Some(vec![]);
        e.call(0x006c_ff60, &args![this, 0u32, 0x101u32]);
        assert_eq!(calls_to(&e, EMPTY_006AC010), vec![vec![this, 1]]);
        // Above a zero capacity: a buffer from slot 4, two locations.
        let this = sized_array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x006c_ff60, &args![this, 2u32, 0u32]);
        assert_eq!(header(&e, this), [0x9002, 2, 2]);
        assert_eq!(
            calls_to(&e, LOCATION_CONSTRUCT),
            vec![vec![0x9002], vec![0x902a]]
        );
        // Above a non-zero capacity: grown, the new locations constructed
        // after the old ones.
        let this = sized_array(&mut e, 0x5000, 1, 1);
        e.call_log = Some(vec![]);
        e.call(0x006c_ff60, &args![this, 3u32, 0u32]);
        assert_eq!(calls_to(&e, GROW_005EFA80), vec![vec![this, 3, 1]]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x28, 0x5028], vec![0x28, 0x5050]]
        );
        assert_eq!(header(&e, this), [0x5000, 3, 3]);
        // Below the size: the dropped ones destroyed; with the byte and at
        // most a quarter of the capacity, reallocated to the size.
        let this = sized_array(&mut e, 0x5000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x006c_ff60, &args![this, 2u32, 1u32]);
        assert_eq!(
            calls_to(&e, DESTROY_RANGE_006AC070),
            vec![vec![this, 0x5050, 4]]
        );
        assert_eq!(calls_to(&e, GROW_005EFA80), vec![vec![this, 2, 2]]);
        assert_eq!(header(&e, this), [0x5000, 2, 2]);
        // Without the byte, or above a quarter, the capacity stays.
        let this = sized_array(&mut e, 0x5000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x006c_ff60, &args![this, 3u32, 1u32]);
        assert!(calls_to(&e, GROW_005EFA80).is_empty());
        assert_eq!(header(&e, this), [0x5000, 3, 8]);
        // Between the size and the capacity: constructed in place.
        let this = sized_array(&mut e, 0x5000, 1, 8);
        e.call_log = Some(vec![]);
        e.call(0x006c_ff60, &args![this, 2u32, 1u32]);
        assert_eq!(calls_to(&e, PLACEMENT_NEW), vec![vec![0x28, 0x5028]]);
        assert_eq!(header(&e, this), [0x5000, 2, 8]);
    }

    #[test]
    fn fn_006d01d0_uses_0x18_byte_elements() {
        let mut e = Engine::new();
        let this = sized_array(&mut e, 0x5000, 3, 4);
        e.call_log = Some(vec![]);
        e.call(0x006d_01d0, &args![this, 0u32, 1u32]);
        assert_eq!(calls_to(&e, EMPTY_008454F0), vec![vec![this, 1]]);
        let this = sized_array(&mut e, 0x5000, 1, 1);
        e.call_log = Some(vec![]);
        e.call(0x006d_01d0, &args![this, 3u32, 0u32]);
        assert_eq!(calls_to(&e, GROW_006D0680), vec![vec![this, 3, 1]]);
        assert_eq!(
            calls_to(&e, CONSTRUCT_RANGE_006D05E0),
            vec![vec![this, 0x5018, 2]]
        );
        let this = sized_array(&mut e, 0x5000, 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x006d_01d0, &args![this, 2u32, 1u32]);
        assert_eq!(
            calls_to(&e, VISIT_RANGE_0072BA80),
            vec![vec![this, 0x5030, 6]]
        );
        assert_eq!(calls_to(&e, GROW_006D0680), vec![vec![this, 2, 2]]);
        assert_eq!(header(&e, this), [0x5000, 2, 2]);
        let this = sized_array(&mut e, 0x5000, 2, 8);
        e.call_log = Some(vec![]);
        e.call(0x006d_01d0, &args![this, 2u32, 0u32]);
        assert_eq!(
            calls_to(&e, CONSTRUCT_RANGE_006D05E0),
            vec![vec![this, 0x5030, 0]]
        );
    }
}
