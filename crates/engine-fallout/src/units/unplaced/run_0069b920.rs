//! `(unplaced)/run_0069b920` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The run holds three groups of functions:
//! - `BSSimpleArray` helpers of pathfinding arrays: the door-entry
//!   constructor wrapper (`0069b920`), two array constructor bodies
//!   (`0069b940`, `0069ba70`) and the in-place construction of 12-byte
//!   entries (`0069b9d0`);
//! - functions of an array of one-word smart pointers (the `NavMeshArray`
//!   that `TESObjectCELL` clears through `0069bca0`): find, remove and clear;
//! - `NavMeshBounds` (Xbox PDB class, confirmed by the RTTI of the vtable its
//!   constructor stores at +0x18, `BSSimpleArray<NavMeshBounds::
//!   NavMeshBoundsTriangle,1024>`): constructor, destructor, reset, the
//!   containment test, the offset to the bounds, the save and load of its
//!   data, the copy, and its two arrays' constructor, destructor, element
//!   address and `SetSize`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiPoint3};

layout! {
    /// `NavMeshBounds` (Xbox PDB), 0x38 bytes; the PC offsets are the
    /// Xbox ones (checked against the constructor `0069bcf0` and the
    /// functions below).
    pub struct NavMeshBounds: 0x38 {
        /// `BoundingBoxMin` (Xbox PDB).
        0x00 BoundingBoxMin: Inline<NiPoint3>,
        /// `BoundingBoxMax` (Xbox PDB).
        0x0C BoundingBoxMax: Inline<NiPoint3>,
        /// `Triangles` (Xbox PDB): `BSSimpleArray<NavMeshBounds::
        /// NavMeshBoundsTriangle,1024>`, three `u16` vertex indices (6 bytes)
        /// per element.
        0x18 Triangles: Inline<BSSimpleArray>,
        /// `Vertices` (Xbox PDB): `BSSimpleArray<NiPoint3,1024>`.
        0x28 Vertices: Inline<BSSimpleArray>,
    }
}

/// The vtable of `BSSimpleArray<NavMeshBounds::NavMeshBoundsTriangle,1024>`
/// (its RTTI names the class).
const TRIANGLE_ARRAY_VTABLE: u32 = 0x0106_b28c;

/// `float` constants: `-FLT_MAX` (`0xff7fffff`) and `FLT_MAX`
/// (`0x7f7fffff`), as the exe's data holds them.
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
const HIGHEST_FLOAT: u32 = 0x0101_6970;
/// `double` 180.0: the margin the containment test allows below the lower
/// and above the upper `z` bound.
const VERTICAL_MARGIN: u32 = 0x0104_ed58;
/// A global three-float vector (the zero vector `0069c070` returns for a
/// position inside empty bounds), read as three words.
const ZERO_VECTOR: u32 = 0x011f_426c;

/// The door entry constructor (`fn_006d5320`).
const DOOR_ENTRY_CONSTRUCT: u32 = 0x006d_5320;
/// `006e1280(this, address, count)`: builds `count` elements of the byte
/// array in place.
const BYTE_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x006e_1280;
/// `006e6da0(size, address)`: placement `new` (gives `address` back).
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `006240d0`: the constructor of the 12-byte entry.
const ENTRY_CONSTRUCT: u32 = 0x0062_40d0;
/// The one-word smart pointer's copy constructor (`this`, address of the
/// source) and destructor.
const POINTER_COPY: u32 = 0x0042_fa20;
const POINTER_DESTRUCT: u32 = 0x0042_fa40;
/// `0042ff30(this, other)`: whether two smart pointers hold the same word.
const POINTER_EQUAL: u32 = 0x0042_ff30;
/// `0044ddc0(this)`: the word at +8 of an array (`iSize`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `0076b610(this)`: whether the word at +8 of an array is zero.
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// `00877a30(this, index)`: the address of a one-word element.
const POINTER_ELEMENT_AT: u32 = 0x0087_7a30;
/// `00559450(this)`: the word at +0.
const FIRST_WORD: u32 = 0x0055_9450;
/// `007037c0(this, value)`: stores a word at +0x24 (the body is shared by
/// unrelated callers).
const STORE_WORD_24: u32 = 0x0070_37c0;
/// `00868790(element)`: the engine map's `GarbageCollector::Add_ov6`.
const GARBAGE_COLLECTOR_ADD: u32 = 0x0086_8790;
/// `0042f8a0(this, index, shrink)`: removes one element of the pointer
/// array.
const POINTER_ARRAY_REMOVE: u32 = 0x0042_f8a0;
/// `0042f9c0(this, free_buffer)`: destroys the pointer array's elements.
const POINTER_ARRAY_CLEAR: u32 = 0x0042_f9c0;
/// `00416870(this, x, y, z)`: the `NiPoint3` constructor; returns `this`.
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `006815c0(this)`: the `NiPoint3` default constructor (the function the
/// vector constructor iterator is given).
const POINT3_DEFAULT_CONSTRUCT: u32 = 0x0068_15c0;
/// The vertex array's constructor and destructor.
const VERTEX_ARRAY_CONSTRUCT: u32 = 0x0049_f1c0;
const VERTEX_ARRAY_DESTRUCT: u32 = 0x0049_f1f0;
/// `008454f0(this, free_buffer)`: empties a `BSSimpleArray`.
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `0069d140(this, reserve, size)`: the triangle array's initialization.
const TRIANGLE_ARRAY_INITIALIZE: u32 = 0x0069_d140;
/// `006a1440(this, index)`: the address of vertex `index`
/// (`pBuffer + index * 12`).
const VERTEX_AT: u32 = 0x006a_1440;
/// `006b9cf0(a, b, c, position, out, height, height)` (cdecl): the test of
/// a position against one triangle of the bounds; the result is in `AL`,
/// a float is written to `out`. Unnamed in the engine map.
const TRIANGLE_TEST: u32 = 0x006b_9cf0;
/// `006b9c20(out, position, from, to)` (cdecl): an edge helper (its body
/// projects onto the edge `from`-`to`); unnamed in the engine map.
const EDGE_POINT: u32 = 0x006b_9c20;
/// `00439ef0(this, out, other)`: `out = this - other` for three-float
/// vectors; returns `out`.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// `004a7290(this)`: the squared length of a three-float vector (ST0).
const VECTOR_SQUARED_LENGTH: u32 = 0x004a_7290;
/// `00401050(address, size, count, constructor)`: the compiler's vector
/// constructor iterator (stdcall).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `operator new` and `operator delete`.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The game's `memcpy` wrapper (`destination, source, count`).
const MEMCPY: u32 = 0x0040_1460;
/// `memcpy_s(destination, destination size, source, count)`.
const MEMCPY_S: u32 = 0x004b_7580;
/// `00401080(address, unused byte)`: reverses the byte order of the word at
/// `address`; `00407a90` does it for a `u16`.
const SWAP_WORD: u32 = 0x0040_1080;
const SWAP_HALF: u32 = 0x0040_7a90;
/// `00401500()`: the global byte at `011c54ba` (whether saved data is
/// byte-swapped).
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
/// `00401680(stream)`: the byte at +0x299 of the stream (whether its data
/// is byte-swapped).
const STREAM_SWAPS_BYTES: u32 = 0x0040_1680;
/// The triangle array's reallocation (`this`, new capacity, count to keep),
/// element construction and destruction (`this`, address, count).
const TRIANGLE_ARRAY_REALLOCATE: u32 = 0x005e_fc90;
const TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0069_d0f0;
const ARRAY_DESTRUCT_ELEMENTS: u32 = 0x0072_ba80;
/// The vertex array's reallocation and element construction.
const VERTEX_ARRAY_REALLOCATE: u32 = 0x006d_c590;
const VERTEX_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0049_f8f0;
/// The copies of the vertex array (`0069cfe0`, the engine map's
/// `BSSimpleArray<CombatSearchDoor_1024>::SetSize`, whose body copies a
/// 12-byte-element array) and of the triangle array (`0069d050`) into
/// another array.
const VERTEX_ARRAY_COPY: u32 = 0x0069_cfe0;
const TRIANGLE_ARRAY_COPY: u32 = 0x0069_d050;

/// The bits of a `float` copied through `FLD`/`FSTP` (a signalling NaN
/// comes out quiet), as a float argument word.
fn pushed_float(value: f32) -> u32 {
    x87_quiet_bits(value.to_bits())
}

/// The `float` global at `address` as the exe pushes it (`FLD`, `FSTP`).
fn pushed_global(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(address))
}

/// Copies three words.
fn copy_vector(e: &mut Engine, to: u32, from: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from.wrapping_add(4 * word));
        e.mem.set_u32(to.wrapping_add(4 * word), value);
    }
}

fn array_size(e: &mut Engine, array: Ptr<BSSimpleArray>) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

fn array_is_empty(e: &mut Engine, array: Ptr<BSSimpleArray>) -> bool {
    e.call(ARRAY_IS_EMPTY, &args![array]).bool()
}

fn swap_word(e: &mut Engine, address: u32) {
    e.call(SWAP_WORD, &args![address, 0u32]);
}

fn swap_half(e: &mut Engine, address: u32) {
    e.call(SWAP_HALF, &args![address, 0u32]);
}

/// The byte count `operator new` is asked for: `count * size`, or
/// `0xffffffff` when the product overflows (`MUL`, `SETO`, `NEG`, `OR`).
fn allocation_size(count: u32, size: u32) -> u32 {
    let product = u64::from(count) * u64::from(size);
    if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    }
}

// Translated from 0069b920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a door entry (`006d5320`) and returns `this`.
pub fn fn_0069b920(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(DOOR_ENTRY_CONSTRUCT, &args![this]);
    this
}

// Translated from 0069b940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor body of a `BSSimpleArray` of bytes: clears `pBuffer`,
/// `iSize` and `iReservedSize`; with a capacity of `max(reserve, size)`
/// above 0 the buffer comes from the virtual slot at +4 and the capacity is
/// stored; with `size` above 0 the first `size` elements are built in place
/// (`006e1280`) and `iSize` is set.
pub fn fn_0069b940(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = if size > reserve { size } else { reserve };
    if capacity != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(BYTE_ARRAY_CONSTRUCT_ELEMENTS, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 0069b9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds `count` 12-byte entries in place from `address` on: for each slot,
/// placement `new` (`006e6da0`) and, when it gives a non-null address, the
/// entry constructor `006240d0`. The array `this` is not used.
// Translation incomplete: 0069b9d0 - the compiler's exception frame (FS:[0]) is not translated
pub fn fn_0069b9d0(e: &mut Engine, _this: Ptr, address: u32, count: u32) {
    for index in 0..count {
        let slot = index.wrapping_mul(0xc).wrapping_add(address);
        let block = e.call(PLACEMENT_NEW, &args![0xcu32, slot]).u32();
        if block != 0 {
            e.call(ENTRY_CONSTRUCT, &args![block]);
        }
    }
}

// Translated from 0069ba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor body of a `BSSimpleArray` of 12-byte entries: as
/// `fn_0069b940`, with the entries built by `fn_0069b9d0`.
pub fn fn_0069ba70(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = if size > reserve { size } else { reserve };
    if capacity != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_0069b9d0(e, this.cast(), buffer, size);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 0069bb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `pointer` (a one-word smart pointer passed by value) from the
/// pointer array: finds it (`fn_0069bb90`, given a copy made by `0042fa20`)
/// and, when found, removes that element (`fn_0069bc50`). The by-value
/// argument is destroyed (`0042fa40`) at the end.
// Translation incomplete: 0069bb00 - the compiler's exception frame (FS:[0]) is not translated
pub fn fn_0069bb00(e: &mut Engine, this: Ptr<BSSimpleArray>, pointer: u32) {
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), pointer);
        let index = e.with_stack(4, |e, copy| {
            e.call(POINTER_COPY, &args![copy, argument]);
            let value = e.mem.u32(copy.addr());
            fn_0069bb90(e, this, value)
        });
        if index != u32::MAX {
            fn_0069bc50(e, this, index);
        }
        e.call(POINTER_DESTRUCT, &args![argument]);
    });
}

// Translated from 0069bb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index of the first element of the pointer array equal to `pointer`
/// (a one-word smart pointer passed by value; `0042ff30` compares), or
/// `0xffffffff`. The by-value argument is destroyed (`0042fa40`) before
/// returning.
// Translation incomplete: 0069bb90 - the compiler's exception frame (FS:[0]) is not translated
pub fn fn_0069bb90(e: &mut Engine, this: Ptr<BSSimpleArray>, pointer: u32) -> u32 {
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), pointer);
        let mut index = 0u32;
        loop {
            if index >= array_size(e, this) {
                e.call(POINTER_DESTRUCT, &args![argument]);
                return u32::MAX;
            }
            let element = e.call(POINTER_ELEMENT_AT, &args![this, index]).u32();
            if e.call(POINTER_EQUAL, &args![element, argument]).bool() {
                e.call(POINTER_DESTRUCT, &args![argument]);
                return index;
            }
            index = index.wrapping_add(1);
        }
    })
}

// Translated from 0069bc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes element `index` of the pointer array: stores 0 at +0x24 of the
/// object it points to (`00559450` reads the pointer, `007037c0` stores),
/// hands the element to `00868790` (the engine map's
/// `GarbageCollector::Add_ov6`) and removes it with shrinking (`0042f8a0`).
pub fn fn_0069bc50(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32) {
    let element = e.call(POINTER_ELEMENT_AT, &args![this, index]).u32();
    let object = e.call(FIRST_WORD, &args![element]).u32();
    e.call(STORE_WORD_24, &args![object, 0u32]);
    let element = e.call(POINTER_ELEMENT_AT, &args![this, index]).u32();
    e.call(GARBAGE_COLLECTOR_ADD, &args![element]);
    e.call(POINTER_ARRAY_REMOVE, &args![this, index, 1u32]);
}

// Translated from 0069bca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the pointer array: hands every element to `00868790` (the engine
/// map's `GarbageCollector::Add_ov6`), then destroys the elements and frees
/// the buffer (`0042f9c0` with 1).
pub fn fn_0069bca0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    let mut index = 0u32;
    while index < array_size(e, this) {
        let element = e.call(POINTER_ELEMENT_AT, &args![this, index]).u32();
        e.call(GARBAGE_COLLECTOR_ADD, &args![element]);
        index = index.wrapping_add(1);
    }
    e.call(POINTER_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0069bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NavMeshBounds` constructor: `BoundingBoxMin` is (-FLT_MAX, -FLT_MAX,
/// -FLT_MAX) and `BoundingBoxMax` (FLT_MAX, FLT_MAX, FLT_MAX), as the code
/// loads them; then the triangle array (`fn_0069cd10`) and the vertex array
/// (`0049f1c0`) are constructed. Returns `this`.
// Translation incomplete: 0069bcf0 - the compiler's exception frame (FS:[0]) is not translated
pub fn fn_0069bcf0(e: &mut Engine, this: Ptr<NavMeshBounds>) -> Ptr<NavMeshBounds> {
    let lowest = pushed_global(e, LOWEST_FLOAT);
    let minimum = this.at(NavMeshBounds::BoundingBoxMin);
    e.call(POINT3_CONSTRUCT, &args![minimum, lowest, lowest, lowest]);
    let highest = pushed_global(e, HIGHEST_FLOAT);
    let maximum = this.at(NavMeshBounds::BoundingBoxMax);
    e.call(POINT3_CONSTRUCT, &args![maximum, highest, highest, highest]);
    fn_0069cd10(e, this.at(NavMeshBounds::Triangles));
    let vertices = this.at(NavMeshBounds::Vertices);
    e.call(VERTEX_ARRAY_CONSTRUCT, &args![vertices]);
    this
}

// Translated from 0069bda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NavMeshBounds` destructor: destroys the vertex array (`0049f1f0`),
/// then the triangle array (`fn_0069cd40`).
// Translation incomplete: 0069bda0 - the compiler's exception frame (FS:[0]) is not translated
pub fn fn_0069bda0(e: &mut Engine, this: Ptr<NavMeshBounds>) {
    let vertices = this.at(NavMeshBounds::Vertices);
    e.call(VERTEX_ARRAY_DESTRUCT, &args![vertices]);
    fn_0069cd40(e, this.at(NavMeshBounds::Triangles));
}

// Translated from 0069be00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the bounds: `BoundingBoxMin` and `BoundingBoxMax` get the
/// constructor's values (built in temporaries by `00416870` and copied),
/// then the vertex array and the triangle array are emptied with their
/// buffers freed (`008454f0` with 1).
pub fn fn_0069be00(e: &mut Engine, this: Ptr<NavMeshBounds>) {
    e.with_stack(0x18, |e, temporaries| {
        let lowest = pushed_global(e, LOWEST_FLOAT);
        let low = temporaries.addr() + 0xc;
        let built = e
            .call(POINT3_CONSTRUCT, &args![low, lowest, lowest, lowest])
            .u32();
        copy_vector(e, this.addr(), built);
        let highest = pushed_global(e, HIGHEST_FLOAT);
        let high = temporaries.addr();
        let built = e
            .call(POINT3_CONSTRUCT, &args![high, highest, highest, highest])
            .u32();
        copy_vector(e, this.addr() + 0xc, built);
    });
    let vertices = this.at(NavMeshBounds::Vertices);
    e.call(ARRAY_CLEAR, &args![vertices, 1u32]);
    let triangles = this.at(NavMeshBounds::Triangles);
    e.call(ARRAY_CLEAR, &args![triangles, 1u32]);
}

/// The three vertices of triangle `index` copied into the 36 bytes at
/// `points` (vertex 0, 1, 2 at +0, +0xc, +0x18).
fn copy_triangle(e: &mut Engine, this: Ptr<NavMeshBounds>, index: u32, points: u32) {
    let triangles = this.at(NavMeshBounds::Triangles);
    let vertices = this.at(NavMeshBounds::Vertices);
    for corner in 0..3u32 {
        let triangle = fn_0069ccf0(e, triangles, index);
        let vertex_index = e.mem.u16(triangle + 2 * corner);
        let vertex = e
            .call(VERTEX_AT, &args![vertices, u32::from(vertex_index)])
            .u32();
        copy_vector(e, points + 0xc * corner, vertex);
    }
}

// Translated from 0069bea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the bounds hold `position`: false unless `x` and `y` lie within
/// the box and `z` within the box widened by 180 below and above (each
/// comparison false for NaN, as the exe's flag tests are); then true when
/// there are no vertices (`0076b610`), otherwise true when one triangle's
/// test (`006b9cf0`, with the box height twice) succeeds.
pub fn fn_0069bea0(e: &mut Engine, this: Ptr<NavMeshBounds>, position: Ptr<NiPoint3>) -> bool {
    let bounds = this.addr();
    let point = position.addr();
    let low = |e: &Engine, i: u32| e.mem.f32(bounds + 4 * i);
    let high = |e: &Engine, i: u32| e.mem.f32(bounds + 0xc + 4 * i);
    let coordinate = |e: &Engine, i: u32| e.mem.f32(point + 4 * i);
    // `TEST AH,0x41; JP` leaves unless `min <= p`; `TEST AH,1; JNZ`
    // leaves unless `max >= p` (both false for NaN).
    for axis in 0..2 {
        let p = coordinate(e, axis);
        let within = low(e, axis) <= p && high(e, axis) >= p;
        if !within {
            return false;
        }
    }
    let margin: f64 = e.global(VERTICAL_MARGIN);
    let floor = low(e, 2) as f64 - margin;
    let above_floor = coordinate(e, 2) as f64 >= floor;
    if !above_floor {
        return false;
    }
    let ceiling = high(e, 2) as f64 + margin;
    let below_ceiling = coordinate(e, 2) as f64 <= ceiling;
    if !below_ceiling {
        return false;
    }
    if array_is_empty(e, this.at(NavMeshBounds::Vertices)) {
        return true;
    }
    let height = (high(e, 2) as f64 - low(e, 2) as f64) as f32;
    let triangles = this.at(NavMeshBounds::Triangles);
    e.with_stack(0x28, |e, locals| {
        // Vertex 0, 1, 2 at +0, +0xc, +0x18; the test's output at +0x24.
        let points = locals.addr();
        let output = points + 0x24;
        let mut index = 0u32;
        while index < array_size(e, triangles) {
            copy_triangle(e, this, index, points);
            let found = e
                .call(
                    TRIANGLE_TEST,
                    &args![
                        points,
                        points + 0xc,
                        points + 0x18,
                        position,
                        output,
                        pushed_float(height),
                        pushed_float(height)
                    ],
                )
                .bool();
            if found {
                return true;
            }
            index = index.wrapping_add(1);
        }
        false
    })
}

// Translated from 0069c070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The offset from `position` to the bounds, written to `result` (returned).
///
/// Without vertices (`0076b610`): inside the box (each `min <= p` and
/// `max >= p`, false for NaN) the global vector at `011f426c`; outside, per
/// axis `p - max` when `max < p`, else `p - min` when `min > p`, else 0.
///
/// With vertices: for each triangle, when its test (`006b9cf0`, with the box
/// height twice) succeeds the result is `(0, 0, h)` with the `h` it wrote;
/// otherwise each of the three edges gives a point (`006b9c20`) and the one
/// with the smallest squared distance to the position (`00439ef0`,
/// `004a7290`; strictly smaller, starting from FLT_MAX) is kept. Without a
/// hit the result is that point minus the position (`00439ef0`).
pub fn fn_0069c070(
    e: &mut Engine,
    this: Ptr<NavMeshBounds>,
    result: Ptr<NiPoint3>,
    position: Ptr<NiPoint3>,
) -> Ptr<NiPoint3> {
    let bounds = this.addr();
    let point = position.addr();
    let low = |e: &Engine, i: u32| e.mem.f32(bounds + 4 * i);
    let high = |e: &Engine, i: u32| e.mem.f32(bounds + 0xc + 4 * i);
    let coordinate = |e: &Engine, i: u32| e.mem.f32(point + 4 * i);

    if array_is_empty(e, this.at(NavMeshBounds::Vertices)) {
        let inside =
            (0..3).all(|i| low(e, i) <= coordinate(e, i) && high(e, i) >= coordinate(e, i));
        if inside {
            copy_vector(e, result.addr(), ZERO_VECTOR);
            return result;
        }
        for axis in 0..3 {
            let p = coordinate(e, axis);
            // `TEST AH,5; JP`: the first branch only for an ordered `max < p`;
            // then `TEST AH,0x41; JNZ`: the second only for `min > p`.
            let offset = if high(e, axis) < p {
                (p as f64 - high(e, axis) as f64) as f32
            } else if low(e, axis) > p {
                (p as f64 - low(e, axis) as f64) as f32
            } else {
                0.0
            };
            e.mem.set_f32(result.addr() + 4 * axis, offset);
        }
        return result;
    }

    // Locals: the closest point (+0), the triangle's points (+0xc, 36
    // bytes), the edge point (+0x30), the difference (+0x3c), the test's
    // output (+0x48).
    e.with_stack(0x4c, |e, locals| {
        let closest = locals.addr();
        let points = closest + 0xc;
        let edge_point = closest + 0x30;
        let difference = closest + 0x3c;
        let output = closest + 0x48;
        let mut best = f32::from_bits(pushed_global(e, HIGHEST_FLOAT));
        e.call(POINT3_DEFAULT_CONSTRUCT, &args![closest]);
        let triangles = this.at(NavMeshBounds::Triangles);
        let mut index = 0u32;
        while index < array_size(e, triangles) {
            e.call(
                VECTOR_CONSTRUCTOR_ITERATOR,
                &args![points, 0xcu32, 3u32, POINT3_DEFAULT_CONSTRUCT],
            );
            copy_triangle(e, this, index, points);
            let height = (high(e, 2) as f64 - low(e, 2) as f64) as f32;
            let found = e
                .call(
                    TRIANGLE_TEST,
                    &args![
                        points,
                        points + 0xc,
                        points + 0x18,
                        position,
                        output,
                        pushed_float(height),
                        pushed_float(height)
                    ],
                )
                .bool();
            if found {
                let z = e.mem.u32(output);
                let zero = pushed_float(0.0);
                e.call(
                    POINT3_CONSTRUCT,
                    &args![result, zero, zero, x87_quiet_bits(z)],
                );
                return;
            }
            for edge in 0..3u32 {
                let next = if edge + 1 == 3 { 0 } else { edge + 1 };
                e.call(
                    EDGE_POINT,
                    &args![
                        edge_point,
                        position,
                        points + edge * 0xc,
                        points + next * 0xc
                    ],
                );
                let vector = e
                    .call(VECTOR_SUBTRACT, &args![position, difference, edge_point])
                    .u32();
                let distance = e.call(VECTOR_SQUARED_LENGTH, &args![vector]).f32();
                // `TEST AH,0x41; JNZ` skips unless `best > distance`.
                if best > distance {
                    copy_vector(e, closest, edge_point);
                    best = f32::from_bits(pushed_float(distance));
                }
            }
            index = index.wrapping_add(1);
        }
        e.call(VECTOR_SUBTRACT, &args![closest, result, position]);
    });
    result
}

// Translated from 0069c460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size of the bounds' saved data: 0x1c (the box and the two counts)
/// plus 12 bytes per vertex and 6 per triangle.
pub fn fn_0069c460(e: &mut Engine, this: Ptr<NavMeshBounds>) -> u32 {
    let vertices = array_size(e, this.at(NavMeshBounds::Vertices));
    let size = vertices.wrapping_mul(0xc).wrapping_add(0x1c);
    let triangles = array_size(e, this.at(NavMeshBounds::Triangles));
    triangles.wrapping_mul(6).wrapping_add(size)
}

// Translated from 0069c4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the bounds to `buffer`: `BoundingBoxMin`, `BoundingBoxMax`, the
/// vertex count and the triangle count as `u16`, then (when there are
/// vertices, `0076b610`) the vertices and the triangles. Everything is
/// copied through temporaries (`operator new`, the vertices' built by the
/// vector constructor iterator) whose words and halves are byte-swapped
/// when `00401500` says so, then copied out (`00401460`) and freed.
// Translation incomplete: 0069c4c0 - the compiler's exception frame (FS:[0]) is not translated
pub fn fn_0069c4c0(e: &mut Engine, this: Ptr<NavMeshBounds>, buffer: u32) {
    let mut out = buffer;
    e.with_stack(0x10, |e, locals| {
        let vector = locals.addr();
        let count = vector + 0xc;
        for offset in [0u32, 0xc] {
            copy_vector(e, vector, this.addr() + offset);
            if e.call(SAVE_SWAPS_BYTES, &[]).bool() {
                swap_word(e, vector);
                swap_word(e, vector + 4);
                swap_word(e, vector + 8);
            }
            e.call(MEMCPY, &args![out, vector, 0xcu32]);
            out = out.wrapping_add(0xc);
        }
        for array in [NavMeshBounds::Vertices, NavMeshBounds::Triangles] {
            let size = array_size(e, this.at(array));
            e.mem.set_u16(count, size as u16);
            if e.call(SAVE_SWAPS_BYTES, &[]).bool() {
                swap_half(e, count);
            }
            e.call(MEMCPY, &args![out, count, 2u32]);
            out = out.wrapping_add(2);
        }
    });
    let vertices = this.at(NavMeshBounds::Vertices);
    if array_is_empty(e, vertices) {
        return;
    }

    let count = array_size(e, vertices);
    let bytes = count.wrapping_mul(0xc);
    let block = e
        .call(OPERATOR_NEW, &args![allocation_size(count, 0xc)])
        .u32();
    if block != 0 {
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![block, 0xcu32, count, POINT3_DEFAULT_CONSTRUCT],
        );
    }
    let first = e.call(VERTEX_AT, &args![vertices, 0u32]).u32();
    e.call(MEMCPY, &args![block, first, bytes]);
    if e.call(SAVE_SWAPS_BYTES, &[]).bool() {
        for index in 0..count {
            let vertex = index.wrapping_mul(0xc).wrapping_add(block);
            swap_word(e, vertex);
            swap_word(e, vertex.wrapping_add(4));
            swap_word(e, vertex.wrapping_add(8));
        }
    }
    e.call(MEMCPY, &args![out, block, bytes]);
    out = out.wrapping_add(bytes);
    e.call(OPERATOR_DELETE, &args![block]);

    let triangles = this.at(NavMeshBounds::Triangles);
    let count = array_size(e, triangles);
    let bytes = count.wrapping_mul(6);
    let block = e
        .call(OPERATOR_NEW, &args![allocation_size(count, 6)])
        .u32();
    let first = fn_0069ccf0(e, triangles, 0);
    e.call(MEMCPY, &args![block, first, bytes]);
    if e.call(SAVE_SWAPS_BYTES, &[]).bool() {
        for index in 0..count {
            for corner in 0..3u32 {
                let half = index
                    .wrapping_mul(6)
                    .wrapping_add(block)
                    .wrapping_add(corner * 2);
                swap_half(e, half);
            }
        }
    }
    e.call(MEMCPY, &args![out, block, bytes]);
    e.call(OPERATOR_DELETE, &args![block]);
}

// Translated from 0069c8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the bounds from `data` (the layout `fn_0069c4c0` writes): resets
/// them (`fn_0069be00`), reads the box, sizes the vertex and triangle arrays
/// to the saved counts (`fn_0069cea0`, `fn_0069cd60`, shrinking), and when
/// there are vertices copies the vertices and the triangles in
/// (`memcpy_s`). Words and halves are byte-swapped when the stream's flag
/// (`00401680`) says so. The third argument is not read.
pub fn fn_0069c8f0(
    e: &mut Engine,
    this: Ptr<NavMeshBounds>,
    data: u32,
    stream: Ptr,
    _unused_3: u32,
) {
    fn_0069be00(e, this);
    let mut data = data;
    for offset in [0u32, 0xc] {
        let field = this.addr() + offset;
        copy_vector(e, field, data);
        if e.call(STREAM_SWAPS_BYTES, &args![stream]).bool() {
            swap_word(e, field);
            swap_word(e, field + 4);
            swap_word(e, field + 8);
        }
        data = data.wrapping_add(0xc);
    }
    let vertices = this.at(NavMeshBounds::Vertices);
    let triangles = this.at(NavMeshBounds::Triangles);
    e.with_stack(8, |e, locals| {
        let vertex_count = locals.addr();
        let triangle_count = vertex_count + 4;

        let saved = e.mem.u16(data);
        e.mem.set_u16(vertex_count, saved);
        if e.call(STREAM_SWAPS_BYTES, &args![stream]).bool() {
            swap_half(e, vertex_count);
        }
        let count = u32::from(e.mem.u16(vertex_count));
        fn_0069cea0(e, vertices, count, 1);
        data = data.wrapping_add(2);

        let saved = e.mem.u16(data);
        e.mem.set_u16(triangle_count, saved);
        if e.call(STREAM_SWAPS_BYTES, &args![stream]).bool() {
            swap_half(e, triangle_count);
        }
        let count = u32::from(e.mem.u16(triangle_count));
        fn_0069cd60(e, triangles, count, 1);
        data = data.wrapping_add(2);

        if array_is_empty(e, vertices) {
            return;
        }
        let bytes = array_size(e, vertices).wrapping_mul(0xc);
        let room = array_size(e, vertices).wrapping_mul(0xc);
        let first = e.call(VERTEX_AT, &args![vertices, 0u32]).u32();
        e.call(MEMCPY_S, &args![first, room, data, bytes]);
        if e.call(STREAM_SWAPS_BYTES, &args![stream]).bool() {
            let mut index = 0u32;
            while index < u32::from(e.mem.u16(vertex_count)) {
                for offset in [0u32, 4, 8] {
                    let vertex = e.call(VERTEX_AT, &args![vertices, index]).u32();
                    swap_word(e, vertex.wrapping_add(offset));
                }
                index = index.wrapping_add(1);
            }
        }
        let advance = array_size(e, vertices).wrapping_mul(0xc);
        data = data.wrapping_add(advance);

        let bytes = array_size(e, triangles).wrapping_mul(6);
        let room = array_size(e, triangles).wrapping_mul(6);
        let first = fn_0069ccf0(e, triangles, 0);
        e.call(MEMCPY_S, &args![first, room, data, bytes]);
        if e.call(STREAM_SWAPS_BYTES, &args![stream]).bool() {
            let mut index = 0u32;
            while index < u32::from(e.mem.u16(triangle_count)) {
                for offset in [0u32, 2, 4] {
                    let triangle = fn_0069ccf0(e, triangles, index);
                    swap_half(e, triangle.wrapping_add(offset));
                }
                index = index.wrapping_add(1);
            }
        }
        // The data pointer is advanced past the triangles; nothing reads it.
        array_size(e, triangles);
    });
}

// Translated from 0069cc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the bounds into `other`: the box words, then the vertex array
/// (`0069cfe0`) and the triangle array (`0069d050`).
pub fn fn_0069cc80(e: &mut Engine, this: Ptr<NavMeshBounds>, other: Ptr<NavMeshBounds>) {
    copy_vector(e, other.addr(), this.addr());
    copy_vector(e, other.addr() + 0xc, this.addr() + 0xc);
    let from = this.at(NavMeshBounds::Vertices);
    let to = other.at(NavMeshBounds::Vertices);
    e.call(VERTEX_ARRAY_COPY, &args![from, to]);
    let from = this.at(NavMeshBounds::Triangles);
    let to = other.at(NavMeshBounds::Triangles);
    e.call(TRIANGLE_ARRAY_COPY, &args![from, to]);
}

// Translated from 0069ccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of triangle `index`: `pBuffer + index * 6`.
pub fn fn_0069ccf0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32) -> u32 {
    index
        .wrapping_mul(6)
        .wrapping_add(e.get(this, BSSimpleArray::pBuffer))
}

// Translated from 0069cd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The triangle array's constructor: the vtable, then the initialization
/// with `(0, 0)` (`0069d140`). Returns `this`.
pub fn fn_0069cd10(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), TRIANGLE_ARRAY_VTABLE);
    e.call(TRIANGLE_ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0069cd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The triangle array's destructor: the vtable, then emptying with the
/// buffer freed (`008454f0` with 1).
pub fn fn_0069cd40(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), TRIANGLE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

/// `BSSimpleArray::SetSize` for elements of `element_size` bytes, with the
/// array's reallocation and element construction functions.
fn set_size(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    count: u32,
    shrink: u8,
    element_size: u32,
    reallocate: u32,
    construct: u32,
) {
    if count == 0 {
        e.call(ARRAY_CLEAR, &args![this, shrink]);
        return;
    }
    let grow = |e: &mut Engine| {
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let at = size.wrapping_mul(element_size).wrapping_add(buffer);
        e.call(construct, &args![this, at, count.wrapping_sub(size)]);
        e.set(this, BSSimpleArray::iSize, count);
    };
    if count > e.get(this, BSSimpleArray::iReservedSize) {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let buffer = e.vcall(this.addr(), 4, &args![count]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
        } else {
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(reallocate, &args![this, count, size]);
        }
        e.set(this, BSSimpleArray::iReservedSize, count);
        grow(e);
    } else if count < e.get(this, BSSimpleArray::iSize) {
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let at = count.wrapping_mul(element_size).wrapping_add(buffer);
        e.call(
            ARRAY_DESTRUCT_ELEMENTS,
            &args![this, at, size.wrapping_sub(count)],
        );
        e.set(this, BSSimpleArray::iSize, count);
        if shrink != 0 && count <= e.get(this, BSSimpleArray::iReservedSize) >> 2 {
            e.call(reallocate, &args![this, count, count]);
            e.set(this, BSSimpleArray::iReservedSize, count);
        }
    } else {
        grow(e);
    }
}

// Translated from 0069cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetSize` of the triangle array (6-byte elements): 0 empties it
/// (`008454f0` with `shrink`); above the capacity the buffer is allocated
/// (virtual slot +4, when there is none) or reallocated (`005efc90`) and the
/// new elements are built (`0069d0f0`); below the size the surplus is
/// destroyed (`0072ba80`) and, with `shrink` and the count at most a quarter
/// of the capacity, the buffer is reallocated to the count; otherwise the
/// new elements are built.
pub fn fn_0069cd60(e: &mut Engine, this: Ptr<BSSimpleArray>, count: u32, shrink: u8) {
    set_size(
        e,
        this,
        count,
        shrink,
        6,
        TRIANGLE_ARRAY_REALLOCATE,
        TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS,
    );
}

// Translated from 0069cea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetSize` of the vertex array (`BSSimpleArray<NiPoint3>`, 12-byte
/// elements): as `fn_0069cd60`, with the reallocation `006dc590` and the
/// element construction `0049f8f0`.
pub fn fn_0069cea0(e: &mut Engine, this: Ptr<BSSimpleArray>, count: u32, shrink: u8) {
    set_size(
        e,
        this,
        count,
        shrink,
        0xc,
        VERTEX_ARRAY_REALLOCATE,
        VERTEX_ARRAY_CONSTRUCT_ELEMENTS,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069b920, fn_0069b920(Ptr) -> Ptr),
        entry!(0x0069b940, fn_0069b940(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0069b9d0, fn_0069b9d0(Ptr, u32, u32)),
        entry!(0x0069ba70, fn_0069ba70(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0069bb00, fn_0069bb00(Ptr<BSSimpleArray>, u32)),
        entry!(0x0069bb90, fn_0069bb90(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(0x0069bc50, fn_0069bc50(Ptr<BSSimpleArray>, u32)),
        entry!(0x0069bca0, fn_0069bca0(Ptr<BSSimpleArray>)),
        entry!(
            0x0069bcf0,
            fn_0069bcf0(Ptr<NavMeshBounds>) -> Ptr<NavMeshBounds>
        ),
        entry!(0x0069bda0, fn_0069bda0(Ptr<NavMeshBounds>)),
        entry!(0x0069be00, fn_0069be00(Ptr<NavMeshBounds>)),
        entry!(
            0x0069bea0,
            fn_0069bea0(Ptr<NavMeshBounds>, Ptr<NiPoint3>) -> bool
        ),
        entry!(
            0x0069c070,
            fn_0069c070(Ptr<NavMeshBounds>, Ptr<NiPoint3>, Ptr<NiPoint3>) -> Ptr<NiPoint3>
        ),
        entry!(0x0069c460, fn_0069c460(Ptr<NavMeshBounds>) -> u32),
        entry!(0x0069c4c0, fn_0069c4c0(Ptr<NavMeshBounds>, u32)),
        entry!(0x0069c8f0, fn_0069c8f0(Ptr<NavMeshBounds>, u32, Ptr, u32)),
        entry!(
            0x0069cc80,
            fn_0069cc80(Ptr<NavMeshBounds>, Ptr<NavMeshBounds>)
        ),
        entry!(0x0069ccf0, fn_0069ccf0(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(
            0x0069cd10,
            fn_0069cd10(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x0069cd40, fn_0069cd40(Ptr<BSSimpleArray>)),
        entry!(0x0069cd60, fn_0069cd60(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x0069cea0, fn_0069cea0(Ptr<BSSimpleArray>, u32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;
    type ArrayFields = (u32, u32, u32);

    /// A virtual allocator (slot +4) for arrays built in the tests.
    const ALLOCATE_SLOT: u32 = 0x7000_0010;
    const ARRAY_VTABLE: u32 = 0x0200_0000;
    /// The global byte `00401500` returns.
    const SAVE_SWAP_FLAG: u32 = 0x011c_54ba;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The engine with the constants mapped and doubles for the array
    /// accessors other units translate.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_5000,
            0x0101_6000,
            0x0104_e000,
            0x011c_5000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(LOWEST_FLOAT, f32::MIN);
        e.set_global(HIGHEST_FLOAT, f32::MAX);
        e.set_global(VERTICAL_MARGIN, 180.0f64);
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_IS_EMPTY, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        e.register(VERTEX_AT, |e, a| {
            e.mem.u32(a[0] + 4).wrapping_add(a[1] * 12).into_ret()
        });
        e.register(POINTER_ELEMENT_AT, |e, a| {
            e.mem.u32(a[0] + 4).wrapping_add(a[1] * 4).into_ret()
        });
        e.register(ALLOCATE_SLOT, |e, a| e.mem.alloc(a[1] * 16).into_ret());
        e.put_vtable(ARRAY_VTABLE, &[0, ALLOCATE_SLOT]);
        e
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The callees among `addresses`, in call order.
    fn order(log: &CallLog, addresses: &[u32]) -> Vec<u32> {
        log.iter()
            .filter(|(callee, _)| addresses.contains(callee))
            .map(|(callee, _)| *callee)
            .collect()
    }

    fn array(e: &mut Engine, at: Ptr<BSSimpleArray>, fields: ArrayFields) {
        e.mem.set_u32(at.addr(), ARRAY_VTABLE);
        e.set(at, BSSimpleArray::pBuffer, fields.0);
        e.set(at, BSSimpleArray::iSize, fields.1);
        e.set(at, BSSimpleArray::iReservedSize, fields.2);
    }

    fn bounds(
        e: &mut Engine,
        min: [f32; 3],
        max: [f32; 3],
        vertices: &[[f32; 3]],
        triangles: &[[u16; 3]],
    ) -> Ptr<NavMeshBounds> {
        let this: Ptr<NavMeshBounds> = e.new_object();
        set_vector(e, this.addr(), min);
        set_vector(e, this.addr() + 0xc, max);
        let vertex_buffer = e.mem.alloc(12 * vertices.len() as u32);
        for (i, vertex) in vertices.iter().enumerate() {
            set_vector(e, vertex_buffer + 12 * i as u32, *vertex);
        }
        let triangle_buffer = e.mem.alloc(6 * triangles.len() as u32);
        for (i, value) in triangles.iter().flatten().enumerate() {
            e.mem.set_u16(triangle_buffer + 2 * i as u32, *value);
        }
        let count = vertices.len() as u32;
        let at = this.at(NavMeshBounds::Vertices);
        array(e, at, (vertex_buffer, count, count));
        let count = triangles.len() as u32;
        let at = this.at(NavMeshBounds::Triangles);
        array(e, at, (triangle_buffer, count, count));
        this
    }

    fn vector(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn set_vector(e: &mut Engine, at: u32, value: [f32; 3]) {
        for (i, v) in value.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn point(e: &mut Engine, value: [f32; 3]) -> Ptr<NiPoint3> {
        let p: Ptr<NiPoint3> = e.new_object();
        set_vector(e, p.addr(), value);
        p
    }

    #[test]
    fn door_entry_wrapper_constructs_and_returns_this() {
        let mut e = engine();
        e.register(DOOR_ENTRY_CONSTRUCT, noop);
        let entry = Ptr::<()>::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        let back = e.call(0x0069_b920, &args![entry]).u32();
        assert_eq!(back, entry.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, DOOR_ENTRY_CONSTRUCT),
            vec![vec![entry.addr()]]
        );
    }

    #[test]
    fn byte_array_constructor_allocates_the_larger_count_and_builds_size() {
        let mut e = engine();
        e.register(BYTE_ARRAY_CONSTRUCT_ELEMENTS, noop);
        let this: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(this.addr(), ARRAY_VTABLE);
        e.set(this, BSSimpleArray::iSize, 9);
        e.call_log = Some(vec![]);
        e.call(0x0069_b940, &args![this, 2u32, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_SLOT), vec![vec![this.addr(), 5]]);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        assert_ne!(buffer, 0);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 5);
        assert_eq!(
            calls_to(&log, BYTE_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![this.addr(), buffer, 5]]
        );

        // A larger reserve wins; no size builds nothing.
        e.call_log = Some(vec![]);
        e.call(0x0069_b940, &args![this, 7u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_SLOT), vec![vec![this.addr(), 7]]);
        assert!(calls_to(&log, BYTE_ARRAY_CONSTRUCT_ELEMENTS).is_empty());
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);

        // Nothing at all: the fields are cleared.
        e.call(0x0069_b940, &args![this, 0u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn entries_are_built_where_placement_new_gives_an_address() {
        let mut e = engine();
        e.register(PLACEMENT_NEW, |_, a| {
            // The second slot comes back null.
            let slot = if a[1] == 0x0300_000c { 0 } else { a[1] };
            slot.into_ret()
        });
        e.register(ENTRY_CONSTRUCT, noop);
        e.call_log = Some(vec![]);
        e.call(0x0069_b9d0, &args![0u32, 0x0300_0000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PLACEMENT_NEW),
            vec![
                vec![0xc, 0x0300_0000],
                vec![0xc, 0x0300_000c],
                vec![0xc, 0x0300_0018]
            ]
        );
        assert_eq!(
            calls_to(&log, ENTRY_CONSTRUCT),
            vec![vec![0x0300_0000], vec![0x0300_0018]]
        );
    }

    #[test]
    fn entry_array_constructor_builds_its_entries() {
        let mut e = engine();
        e.register(PLACEMENT_NEW, |_, a| a[1].into_ret());
        e.register(ENTRY_CONSTRUCT, noop);
        let this: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(this.addr(), ARRAY_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0069_ba70, &args![this, 4u32, 2u32]);
        let log = e.call_log.take().unwrap();
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        assert_eq!(calls_to(&log, ALLOCATE_SLOT), vec![vec![this.addr(), 4]]);
        assert_eq!(
            calls_to(&log, ENTRY_CONSTRUCT),
            vec![vec![buffer], vec![buffer + 12]]
        );
        assert_eq!(e.get(this, BSSimpleArray::iSize), 2);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 4);
    }

    /// A pointer array holding `words`, with doubles for the smart pointer
    /// and the removal callees.
    fn pointer_array(e: &mut Engine, words: &[u32]) -> Ptr<BSSimpleArray> {
        e.register(POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            a[0].into_ret()
        });
        e.register(POINTER_DESTRUCT, noop);
        e.register(POINTER_EQUAL, |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STORE_WORD_24, noop);
        e.register(GARBAGE_COLLECTOR_ADD, noop);
        e.register(POINTER_ARRAY_REMOVE, noop);
        e.register(POINTER_ARRAY_CLEAR, noop);
        let this: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(4 * words.len() as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *word);
        }
        let count = words.len() as u32;
        array(e, this, (buffer, count, count));
        this
    }

    #[test]
    fn find_gives_the_index_or_minus_one_and_destroys_its_argument() {
        let mut e = engine();
        let this = pointer_array(&mut e, &[0x0500_0000, 0x0500_0100, 0x0500_0200]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_bb90, &args![this, 0x0500_0100u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, POINTER_EQUAL).len(), 2);
        assert_eq!(calls_to(&log, POINTER_DESTRUCT).len(), 1);

        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0069_bb90, &args![this, 0x0500_0300u32]).u32(),
            u32::MAX
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, POINTER_EQUAL).len(), 3);
        assert_eq!(calls_to(&log, POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn remove_by_pointer_removes_the_found_element() {
        let mut e = engine();
        let this = pointer_array(&mut e, &[0x0500_0000, 0x0500_0100, 0x0500_0200]);
        e.call_log = Some(vec![]);
        e.call(0x0069_bb00, &args![this, 0x0500_0200u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, POINTER_ARRAY_REMOVE),
            vec![vec![this.addr(), 2, 1]]
        );
        // The copy (in the search) and the argument are destroyed.
        assert_eq!(calls_to(&log, POINTER_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&log, POINTER_COPY).len(), 1);

        e.call_log = Some(vec![]);
        e.call(0x0069_bb00, &args![this, 0x0500_0300u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, POINTER_ARRAY_REMOVE).is_empty());
        assert_eq!(calls_to(&log, POINTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn remove_at_clears_the_object_field_and_hands_the_element_over() {
        let mut e = engine();
        let this = pointer_array(&mut e, &[0x0500_0000, 0x0500_0100]);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call_log = Some(vec![]);
        e.call(0x0069_bc50, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STORE_WORD_24), vec![vec![0x0500_0100, 0]]);
        assert_eq!(
            calls_to(&log, GARBAGE_COLLECTOR_ADD),
            vec![vec![buffer + 4]]
        );
        assert_eq!(
            calls_to(&log, POINTER_ARRAY_REMOVE),
            vec![vec![this.addr(), 1, 1]]
        );
        assert_eq!(
            order(
                &log,
                &[STORE_WORD_24, GARBAGE_COLLECTOR_ADD, POINTER_ARRAY_REMOVE]
            ),
            vec![STORE_WORD_24, GARBAGE_COLLECTOR_ADD, POINTER_ARRAY_REMOVE]
        );
    }

    #[test]
    fn clear_hands_every_element_over_then_clears() {
        let mut e = engine();
        let this = pointer_array(&mut e, &[0x0500_0000, 0x0500_0100]);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call_log = Some(vec![]);
        e.call(0x0069_bca0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, GARBAGE_COLLECTOR_ADD),
            vec![vec![buffer], vec![buffer + 4]]
        );
        assert_eq!(
            calls_to(&log, POINTER_ARRAY_CLEAR),
            vec![vec![this.addr(), 1]]
        );
    }

    fn point_constructor(e: &mut Engine, a: &[u32]) -> Ret {
        for i in 0..3 {
            e.mem.set_u32(a[0] + 4 * i as u32, a[1 + i]);
        }
        a[0].into_ret()
    }

    #[test]
    fn constructor_sets_the_extreme_box_and_builds_both_arrays() {
        let mut e = engine();
        e.register(POINT3_CONSTRUCT, point_constructor);
        e.register(TRIANGLE_ARRAY_INITIALIZE, noop);
        e.register(VERTEX_ARRAY_CONSTRUCT, noop);
        let this: Ptr<NavMeshBounds> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0069_bcf0, &args![this]).u32();
        assert_eq!(back, this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, POINT3_CONSTRUCT),
            vec![
                vec![this.addr(), 0xff7f_ffff, 0xff7f_ffff, 0xff7f_ffff],
                vec![this.addr() + 0xc, 0x7f7f_ffff, 0x7f7f_ffff, 0x7f7f_ffff]
            ]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x18), TRIANGLE_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&log, TRIANGLE_ARRAY_INITIALIZE),
            vec![vec![this.addr() + 0x18, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, VERTEX_ARRAY_CONSTRUCT),
            vec![vec![this.addr() + 0x28]]
        );

        // A signalling NaN constant is pushed quiet (`FLD`/`FSTP`).
        e.set_global(LOWEST_FLOAT, 0xff80_0001u32);
        e.call_log = Some(vec![]);
        e.call(0x0069_bcf0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, POINT3_CONSTRUCT)[0][1], 0xffc0_0001);
    }

    #[test]
    fn destructor_destroys_vertices_then_triangles() {
        let mut e = engine();
        e.register(VERTEX_ARRAY_DESTRUCT, noop);
        e.register(ARRAY_CLEAR, noop);
        let this: Ptr<NavMeshBounds> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0069_bda0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            order(&log, &[VERTEX_ARRAY_DESTRUCT, ARRAY_CLEAR]),
            vec![VERTEX_ARRAY_DESTRUCT, ARRAY_CLEAR]
        );
        assert_eq!(
            calls_to(&log, VERTEX_ARRAY_DESTRUCT),
            vec![vec![this.addr() + 0x28]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_CLEAR),
            vec![vec![this.addr() + 0x18, 1]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x18), TRIANGLE_ARRAY_VTABLE);
    }

    #[test]
    fn reset_restores_the_box_and_empties_both_arrays() {
        let mut e = engine();
        e.register(POINT3_CONSTRUCT, point_constructor);
        e.register(ARRAY_CLEAR, noop);
        let this = bounds(&mut e, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], &[], &[]);
        e.call_log = Some(vec![]);
        e.call(0x0069_be00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(vector(&e, this.addr()), [f32::MIN; 3]);
        assert_eq!(vector(&e, this.addr() + 0xc), [f32::MAX; 3]);
        assert_eq!(
            calls_to(&log, ARRAY_CLEAR),
            vec![vec![this.addr() + 0x28, 1], vec![this.addr() + 0x18, 1]]
        );
    }

    /// Vertices of two triangles; the second one's first vertex has x = 7.
    const VERTICES: [[f32; 3]; 4] = [
        [1.0, 1.0, 0.0],
        [2.0, 1.0, 0.0],
        [1.0, 2.0, 0.0],
        [7.0, 7.0, 1.0],
    ];
    const TRIANGLES: [[u16; 3]; 2] = [[0, 1, 2], [3, 1, 2]];

    #[test]
    fn contains_tests_the_box_then_the_triangles() {
        let mut e = engine();
        e.register(TRIANGLE_TEST, |e, a| (e.mem.f32(a[0]) == 7.0).into_ret());
        let min = [0.0, 0.0, 0.0];
        let max = [10.0, 10.0, 4.0];

        // No vertices: the box decides (z widened by 180).
        let empty = bounds(&mut e, min, max, &[], &[]);
        for (position, inside) in [
            ([5.0, 5.0, 2.0], true),
            ([10.0, 0.0, -180.0], true),
            ([5.0, 5.0, 184.0], true),
            ([10.5, 5.0, 2.0], false),
            ([5.0, -0.5, 2.0], false),
            ([5.0, 5.0, -180.5], false),
            ([5.0, 5.0, 184.5], false),
            ([f32::NAN, 5.0, 2.0], false),
            ([5.0, f32::NAN, 2.0], false),
            ([5.0, 5.0, f32::NAN], false),
        ] {
            let p = point(&mut e, position);
            let found = e.call(0x0069_bea0, &args![empty, p]).bool();
            assert_eq!(found, inside, "{position:?}");
        }

        // With triangles: the second triangle's test succeeds.
        let full = bounds(&mut e, min, max, &VERTICES, &TRIANGLES);
        let p = point(&mut e, [5.0, 5.0, 2.0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0069_bea0, &args![full, p]).bool());
        let log = e.call_log.take().unwrap();
        let tests = calls_to(&log, TRIANGLE_TEST);
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[1][3], p.addr());
        assert_eq!(tests[1][1], tests[1][0] + 0xc);
        assert_eq!(tests[1][2], tests[1][0] + 0x18);
        // The box height, twice.
        assert_eq!(&tests[1][5..], &[4.0f32.to_bits(), 4.0f32.to_bits()]);

        // No triangle succeeds.
        let none = bounds(&mut e, min, max, &VERTICES[..3], &TRIANGLES[..1]);
        assert!(!e.call(0x0069_bea0, &args![none, p]).bool());
    }

    #[test]
    fn offset_without_vertices_measures_to_the_box() {
        let mut e = engine();
        set_vector(&mut e, ZERO_VECTOR, [0.25, 0.5, 0.75]);
        let this = bounds(&mut e, [0.0, 0.0, 0.0], [10.0, 10.0, 10.0], &[], &[]);
        let result: Ptr<NiPoint3> = e.new_object();
        let inside = point(&mut e, [5.0, 10.0, 0.0]);
        let back = e.call(0x0069_c070, &args![this, result, inside]).u32();
        assert_eq!(back, result.addr());
        assert_eq!(vector(&e, result.addr()), [0.25, 0.5, 0.75]);

        let outside = point(&mut e, [15.0, -3.0, 5.0]);
        e.call(0x0069_c070, &args![this, result, outside]);
        assert_eq!(vector(&e, result.addr()), [5.0, -3.0, 0.0]);

        // A NaN coordinate is outside, and its axis measures 0.
        let nan = point(&mut e, [f32::NAN, 12.0, 5.0]);
        e.call(0x0069_c070, &args![this, result, nan]);
        assert_eq!(vector(&e, result.addr()), [0.0, 2.0, 0.0]);
    }

    fn offset_engine() -> Engine {
        let mut e = engine();
        e.register(POINT3_CONSTRUCT, point_constructor);
        e.register(POINT3_DEFAULT_CONSTRUCT, noop);
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, noop);
        e.register(EDGE_POINT, |e, a| {
            // The edge's first vertex.
            let from = vector(e, a[2]);
            set_vector(e, a[0], from);
            a[0].into_ret()
        });
        e.register(VECTOR_SUBTRACT, |e, a| {
            let left = vector(e, a[0]);
            let right = vector(e, a[2]);
            let difference = [left[0] - right[0], left[1] - right[1], left[2] - right[2]];
            set_vector(e, a[1], difference);
            a[1].into_ret()
        });
        e.register(VECTOR_SQUARED_LENGTH, |e, a| {
            let v = vector(e, a[0]);
            (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).into_ret()
        });
        e
    }

    #[test]
    fn offset_with_a_hit_is_vertical() {
        let mut e = offset_engine();
        e.register(TRIANGLE_TEST, |e, a| {
            e.mem.set_f32(a[4], 2.5);
            (e.mem.f32(a[0]) == 7.0).into_ret()
        });
        let this = bounds(&mut e, [0.0; 3], [10.0, 10.0, 4.0], &VERTICES, &TRIANGLES);
        let result: Ptr<NiPoint3> = e.new_object();
        let p = point(&mut e, [5.0, 5.0, 2.0]);
        e.call_log = Some(vec![]);
        let back = e.call(0x0069_c070, &args![this, result, p]).u32();
        assert_eq!(back, result.addr());
        assert_eq!(vector(&e, result.addr()), [0.0, 0.0, 2.5]);
        let log = e.call_log.take().unwrap();
        // The first triangle's three edges were measured before the hit.
        assert_eq!(calls_to(&log, EDGE_POINT).len(), 3);
        assert_eq!(calls_to(&log, TRIANGLE_TEST)[0][5], 4.0f32.to_bits());
    }

    #[test]
    fn offset_without_a_hit_goes_to_the_nearest_edge_point() {
        let mut e = offset_engine();
        e.register(TRIANGLE_TEST, noop);
        let this = bounds(&mut e, [0.0; 3], [10.0, 10.0, 4.0], &VERTICES, &TRIANGLES);
        let result: Ptr<NiPoint3> = e.new_object();
        let p = point(&mut e, [6.0, 6.0, 1.0]);
        e.call_log = Some(vec![]);
        e.call(0x0069_c070, &args![this, result, p]);
        // The nearest edge start is (7, 7, 1).
        assert_eq!(vector(&e, result.addr()), [1.0, 1.0, 0.0]);
        let log = e.call_log.take().unwrap();
        let edges = calls_to(&log, EDGE_POINT);
        assert_eq!(edges.len(), 6);
        // Edges 0-1, 1-2, 2-0.
        assert_eq!(edges[0][3], edges[0][2] + 0xc);
        assert_eq!(edges[2][2], edges[0][2] + 0x18);
        assert_eq!(edges[2][3], edges[0][2]);
        let last = calls_to(&log, VECTOR_SUBTRACT).pop().unwrap();
        assert_eq!(&last[1..], &[result.addr(), p.addr()]);
    }

    #[test]
    fn saved_size_counts_box_vertices_and_triangles() {
        let mut e = engine();
        let this = bounds(&mut e, [0.0; 3], [1.0; 3], &VERTICES, &TRIANGLES);
        assert_eq!(
            e.call(0x0069_c460, &args![this]).u32(),
            0x1c + 4 * 12 + 2 * 6
        );
    }

    /// Doubles for saving and loading: memory copies, allocation, the byte
    /// swaps, the swap flags.
    fn save_engine() -> Engine {
        let mut e = engine();
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            a[0].into_ret()
        });
        e.register(MEMCPY_S, |e, a| {
            for i in 0..a[3] {
                let byte = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            0u32.into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_DELETE, noop);
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, noop);
        e.register(SWAP_WORD, |e, a| {
            let value = e.mem.u32(a[0]).swap_bytes();
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(SWAP_HALF, |e, a| {
            let value = e.mem.u16(a[0]).swap_bytes();
            e.mem.set_u16(a[0], value);
            Ret::default()
        });
        e.register(SAVE_SWAPS_BYTES, |e, _| {
            e.global::<u8>(SAVE_SWAP_FLAG).into_ret()
        });
        e.register(STREAM_SWAPS_BYTES, |e, a| e.mem.u8(a[0] + 0x299).into_ret());
        e
    }

    /// The saved form of the test bounds, optionally byte-swapped.
    fn saved(min: [f32; 3], max: [f32; 3], swapped: bool) -> Vec<u8> {
        let mut bytes = vec![];
        for v in min.iter().chain(max.iter()) {
            let bits = v.to_bits();
            let bits = if swapped { bits.swap_bytes() } else { bits };
            bytes.extend_from_slice(&bits.to_le_bytes());
        }
        let mut halves = vec![VERTICES.len() as u16, TRIANGLES.len() as u16];
        let words: Vec<u32> = VERTICES.iter().flatten().map(|v| v.to_bits()).collect();
        for half in halves.drain(..) {
            let half = if swapped { half.swap_bytes() } else { half };
            bytes.extend_from_slice(&half.to_le_bytes());
        }
        for word in words {
            let word = if swapped { word.swap_bytes() } else { word };
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        for half in TRIANGLES.iter().flatten() {
            let half = if swapped { half.swap_bytes() } else { *half };
            bytes.extend_from_slice(&half.to_le_bytes());
        }
        bytes
    }

    fn read_bytes(e: &Engine, at: u32, count: usize) -> Vec<u8> {
        (0..count as u32).map(|i| e.mem.u8(at + i)).collect()
    }

    #[test]
    fn save_writes_box_counts_vertices_and_triangles() {
        let min = [-1.0, -2.0, -3.0];
        let max = [4.0, 5.0, 6.0];
        for swapped in [false, true] {
            let mut e = save_engine();
            e.set_global(SAVE_SWAP_FLAG, u8::from(swapped));
            let this = bounds(&mut e, min, max, &VERTICES, &TRIANGLES);
            let expected = saved(min, max, swapped);
            let buffer = e.mem.alloc(expected.len() as u32);
            e.call_log = Some(vec![]);
            e.call(0x0069_c4c0, &args![this, buffer]);
            assert_eq!(read_bytes(&e, buffer, expected.len()), expected);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, OPERATOR_NEW),
                vec![vec![4 * 12], vec![2 * 6]]
            );
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), 2);
            // The vertex temporary is built as `NiPoint3`s.
            let built = calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR);
            assert_eq!(&built[0][1..], &[0xc, 4, POINT3_DEFAULT_CONSTRUCT]);
            // The bounds are left as they were.
            assert_eq!(vector(&e, this.addr()), min);
        }
    }

    #[test]
    fn save_without_vertices_writes_only_the_header() {
        let mut e = save_engine();
        let this = bounds(&mut e, [0.0; 3], [1.0; 3], &[], &TRIANGLES);
        let buffer = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0069_c4c0, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, MEMCPY).len(), 4);
        // Vertex count 0, then triangle count 2.
        assert_eq!(e.mem.u16(buffer + 0x18), 0);
        assert_eq!(e.mem.u16(buffer + 0x1a), 2);
    }

    #[test]
    fn load_reads_what_save_writes() {
        let min = [-1.0, -2.0, -3.0];
        let max = [4.0, 5.0, 6.0];
        for swapped in [false, true] {
            let mut e = save_engine();
            e.register(POINT3_CONSTRUCT, point_constructor);
            // Emptying keeps the buffers (the test's arrays are big enough).
            e.register(ARRAY_CLEAR, |e, a| {
                e.mem.set_u32(a[0] + 8, 0);
                Ret::default()
            });
            e.register(VERTEX_ARRAY_CONSTRUCT_ELEMENTS, noop);
            e.register(TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS, noop);
            let this = bounds(&mut e, [9.0; 3], [9.0; 3], &[[0.0; 3]; 8], &[[0; 3]; 8]);
            let data = saved(min, max, swapped);
            let at = e.mem.alloc(data.len() as u32);
            for (i, byte) in data.iter().enumerate() {
                e.mem.set_u8(at + i as u32, *byte);
            }
            let stream = Ptr::<()>::new(e.mem.alloc(0x300));
            e.mem.set_u8(stream.addr() + 0x299, u8::from(swapped));
            e.call_log = Some(vec![]);
            e.call(0x0069_c8f0, &args![this, at, stream, 0u32]);
            assert_eq!(vector(&e, this.addr()), min);
            assert_eq!(vector(&e, this.addr() + 0xc), max);
            let vertices = this.at(NavMeshBounds::Vertices);
            let triangles = this.at(NavMeshBounds::Triangles);
            assert_eq!(e.get(vertices, BSSimpleArray::iSize), 4);
            assert_eq!(e.get(triangles, BSSimpleArray::iSize), 2);
            let vertex_buffer = e.get(vertices, BSSimpleArray::pBuffer);
            for (i, v) in VERTICES.iter().enumerate() {
                assert_eq!(vector(&e, vertex_buffer + 12 * i as u32), *v);
            }
            let triangle_buffer = e.get(triangles, BSSimpleArray::pBuffer);
            for (i, t) in TRIANGLES.iter().flatten().enumerate() {
                assert_eq!(e.mem.u16(triangle_buffer + 2 * i as u32), *t);
            }
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, MEMCPY_S),
                vec![
                    vec![vertex_buffer, 48, at + 0x1c, 48],
                    vec![triangle_buffer, 12, at + 0x1c + 48, 12]
                ]
            );
            // The arrays were sized by building the new elements.
            assert_eq!(
                calls_to(&log, VERTEX_ARRAY_CONSTRUCT_ELEMENTS),
                vec![vec![vertices.addr(), vertex_buffer, 4]]
            );
            assert_eq!(
                calls_to(&log, TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS),
                vec![vec![triangles.addr(), triangle_buffer, 2]]
            );
        }
    }

    #[test]
    fn copy_copies_the_box_and_both_arrays() {
        let mut e = engine();
        e.register(VERTEX_ARRAY_COPY, noop);
        e.register(TRIANGLE_ARRAY_COPY, noop);
        let this = bounds(&mut e, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], &[], &[]);
        let other: Ptr<NavMeshBounds> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0069_cc80, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(vector(&e, other.addr()), [1.0, 2.0, 3.0]);
        assert_eq!(vector(&e, other.addr() + 0xc), [4.0, 5.0, 6.0]);
        assert_eq!(
            calls_to(&log, VERTEX_ARRAY_COPY),
            vec![vec![this.addr() + 0x28, other.addr() + 0x28]]
        );
        assert_eq!(
            calls_to(&log, TRIANGLE_ARRAY_COPY),
            vec![vec![this.addr() + 0x18, other.addr() + 0x18]]
        );
        assert_eq!(
            order(&log, &[VERTEX_ARRAY_COPY, TRIANGLE_ARRAY_COPY]),
            vec![VERTEX_ARRAY_COPY, TRIANGLE_ARRAY_COPY]
        );
    }

    #[test]
    fn triangle_address_is_six_bytes_per_index() {
        let mut e = engine();
        let this: Ptr<BSSimpleArray> = e.new_object();
        e.set(this, BSSimpleArray::pBuffer, 0x0400_0000);
        assert_eq!(e.call(0x0069_ccf0, &args![this, 5u32]).u32(), 0x0400_001e);
    }

    #[test]
    fn triangle_array_constructor_and_destructor() {
        let mut e = engine();
        e.register(TRIANGLE_ARRAY_INITIALIZE, noop);
        e.register(ARRAY_CLEAR, noop);
        let this: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_cd10, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), TRIANGLE_ARRAY_VTABLE);
        e.mem.set_u32(this.addr(), 0);
        e.call(0x0069_cd40, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), TRIANGLE_ARRAY_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, TRIANGLE_ARRAY_INITIALIZE),
            vec![vec![this.addr(), 0, 0]]
        );
        assert_eq!(calls_to(&log, ARRAY_CLEAR), vec![vec![this.addr(), 1]]);
    }

    /// What one `SetSize` run leaves: the array's fields, the call log and
    /// the array's address.
    struct SetSizeRun {
        fields: ArrayFields,
        log: CallLog,
        this: u32,
    }

    /// Runs `SetSize` at `address` on an array starting as `start` (buffer,
    /// size, capacity).
    fn run_set_size(address: u32, start: ArrayFields, count: u32, shrink: u8) -> SetSizeRun {
        let mut e = engine();
        for callee in [
            ARRAY_CLEAR,
            TRIANGLE_ARRAY_REALLOCATE,
            TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS,
            VERTEX_ARRAY_REALLOCATE,
            VERTEX_ARRAY_CONSTRUCT_ELEMENTS,
            ARRAY_DESTRUCT_ELEMENTS,
        ] {
            e.register(callee, noop);
        }
        let this: Ptr<BSSimpleArray> = e.new_object();
        array(&mut e, this, start);
        e.call_log = Some(vec![]);
        e.call(address, &args![this, count, shrink]);
        SetSizeRun {
            fields: (
                e.get(this, BSSimpleArray::pBuffer),
                e.get(this, BSSimpleArray::iSize),
                e.get(this, BSSimpleArray::iReservedSize),
            ),
            log: e.call_log.take().unwrap(),
            this: this.addr(),
        }
    }

    #[test]
    fn triangle_set_size_branches() {
        let address = 0x0069_cd60;
        // 0 empties the array with the given flag.
        let run = run_set_size(address, (0x0400_0000, 3, 4), 0, 0);
        assert_eq!(calls_to(&run.log, ARRAY_CLEAR), vec![vec![run.this, 0]]);

        // Growing without a buffer allocates through the vtable.
        let run = run_set_size(address, (0, 0, 0), 5, 1);
        assert_eq!(calls_to(&run.log, ALLOCATE_SLOT), vec![vec![run.this, 5]]);
        assert_eq!((run.fields.1, run.fields.2), (5, 5));
        assert_eq!(
            calls_to(&run.log, TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![run.this, run.fields.0, 5]]
        );

        // Growing past the capacity reallocates, keeping the size.
        let run = run_set_size(address, (0x0400_0000, 2, 3), 6, 1);
        assert_eq!(
            calls_to(&run.log, TRIANGLE_ARRAY_REALLOCATE),
            vec![vec![run.this, 6, 2]]
        );
        assert_eq!(
            calls_to(&run.log, TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![run.this, 0x0400_000c, 4]]
        );
        assert_eq!((run.fields.1, run.fields.2), (6, 6));

        // Shrinking to a quarter of the capacity reallocates when asked.
        let run = run_set_size(address, (0x0400_0000, 10, 16), 4, 1);
        assert_eq!(
            calls_to(&run.log, ARRAY_DESTRUCT_ELEMENTS),
            vec![vec![run.this, 0x0400_0018, 6]]
        );
        assert_eq!(
            calls_to(&run.log, TRIANGLE_ARRAY_REALLOCATE),
            vec![vec![run.this, 4, 4]]
        );
        assert_eq!((run.fields.1, run.fields.2), (4, 4));

        // Not when the count is above a quarter, nor without the flag.
        let run = run_set_size(address, (0x0400_0000, 10, 16), 5, 1);
        assert!(calls_to(&run.log, TRIANGLE_ARRAY_REALLOCATE).is_empty());
        assert_eq!((run.fields.1, run.fields.2), (5, 16));
        let run = run_set_size(address, (0x0400_0000, 10, 16), 4, 0);
        assert!(calls_to(&run.log, TRIANGLE_ARRAY_REALLOCATE).is_empty());
        assert_eq!(run.fields.2, 16);

        // Growing within the capacity builds the new elements.
        let run = run_set_size(address, (0x0400_0000, 2, 8), 5, 0);
        assert_eq!(
            calls_to(&run.log, TRIANGLE_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![run.this, 0x0400_000c, 3]]
        );
        assert_eq!(run.fields.1, 5);
    }

    #[test]
    fn vertex_set_size_branches() {
        let address = 0x0069_cea0;
        let run = run_set_size(address, (0x0400_0000, 3, 4), 0, 1);
        assert_eq!(calls_to(&run.log, ARRAY_CLEAR), vec![vec![run.this, 1]]);

        let run = run_set_size(address, (0, 0, 0), 2, 0);
        assert_eq!(calls_to(&run.log, ALLOCATE_SLOT), vec![vec![run.this, 2]]);
        assert_eq!(
            calls_to(&run.log, VERTEX_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![run.this, run.fields.0, 2]]
        );

        let run = run_set_size(address, (0x0400_0000, 2, 3), 6, 0);
        assert_eq!(
            calls_to(&run.log, VERTEX_ARRAY_REALLOCATE),
            vec![vec![run.this, 6, 2]]
        );
        assert_eq!(
            calls_to(&run.log, VERTEX_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![run.this, 0x0400_0018, 4]]
        );
        assert_eq!((run.fields.1, run.fields.2), (6, 6));

        let run = run_set_size(address, (0x0400_0000, 10, 16), 4, 1);
        assert_eq!(
            calls_to(&run.log, ARRAY_DESTRUCT_ELEMENTS),
            vec![vec![run.this, 0x0400_0030, 6]]
        );
        assert_eq!(
            calls_to(&run.log, VERTEX_ARRAY_REALLOCATE),
            vec![vec![run.this, 4, 4]]
        );
        assert_eq!((run.fields.1, run.fields.2), (4, 4));

        let run = run_set_size(address, (0x0400_0000, 2, 8), 5, 0);
        assert_eq!(
            calls_to(&run.log, VERTEX_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![run.this, 0x0400_0018, 3]]
        );
        assert_eq!(run.fields.1, 5);
    }
}
