//! `Havok SDK/run_00d51760` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The 28 functions are one builder (the callee `00d51720` that sets the same object up is
//! named `hkpConstraintConstructionKit::begin` in the Ghidra project; the other names are not
//! confirmed): a 0x1c-byte working object whose word at `+4` points at the scheme being
//! written. The scheme has four counters (`+0` unused here, `+4` size, `+8` and `+0xc` the two
//! small counts), an `hkArray` of 16-byte vectors at `+0x10`, an `hkArray` of command words at
//! `+0x1c`, an `hkArray` of words at `+0x28` and one of object pointers at `+0x34` (each array:
//! data, size, capacity word). Every function appends a command word and its operands with the
//! compiler's inline push-back (the memory router `00c85750`, then `00c90940` to grow the array
//! when its size equals its capacity) and adds to the counters; the first one (`00d51760`)
//! walks a command array and adds up what each command costs. The builder keeps ten flag bytes
//! from `+0x10` and two counters at `+8`/`+0xc` that `00d52ae0` and `00d52ba0` change.
//!
//! The shared helpers (`pub(crate)`) are the SSE models the other run units of this slot use.

#[allow(unused_imports)]
use crate::prelude::*;

/// Four lanes of an SSE register, as their bit patterns.
pub(crate) type Lanes = [u32; 4];

/// The memory router (`00c85750`, no arguments): its `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The array growth routine the inline push-backs call (`00c90940`, cdecl: the allocator, the
/// array, the element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkReferencedObject::addReference` (`00c90510`, thiscall on the object).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkVector4::setTransformedInversePos` (`00c7f690`: this, the transform, the vector).
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// The CRT cosine and sine helpers (value in and out in `ST0`).
const CRT_COSINE: u32 = 0x00ec_9f70;
const CRT_SINE: u32 = 0x00ec_a0a0;
/// The float the unused lanes take (`01011d78`, zero in the exe), the float in the second lane
/// of the cosine vector (`01016410`, 100.0 in the exe) and the two floats of the normalising
/// step (`01016248`, 0.5, and `01017718`, 3.0, in the exe).
const FILL_LANE: u32 = 0x0101_1d78;
const COSINE_SECOND_LANE: u32 = 0x0101_6410;
const NORMALISE_HALF: u32 = 0x0101_6248;
const NORMALISE_THREE: u32 = 0x0101_7718;
/// The low 30 bits of an `hkArray` capacity word are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// Scheme counters and arrays (offsets from `[kit + 4]`).
const SCHEME_SIZE: u32 = 0x04;
const SCHEME_COUNT_A: u32 = 0x08;
const SCHEME_COUNT_B: u32 = 0x0c;
const SCHEME_VECTORS: u32 = 0x10;
const SCHEME_VECTOR_COUNT: u32 = 0x14;
const SCHEME_COMMANDS: u32 = 0x1c;
const SCHEME_WORDS: u32 = 0x28;
const SCHEME_OBJECTS: u32 = 0x34;
const SCHEME_OBJECT_COUNT: u32 = 0x38;

/// `MOVAPS` load of a vector as its four words.
pub(crate) fn load_lanes(e: &Engine, address: u32) -> Lanes {
    [
        e.mem.u32(address),
        e.mem.u32(address.wrapping_add(4)),
        e.mem.u32(address.wrapping_add(8)),
        e.mem.u32(address.wrapping_add(12)),
    ]
}

/// `MOVAPS` store of a vector.
pub(crate) fn store_lanes(e: &mut Engine, address: u32, lanes: Lanes) {
    for (index, lane) in lanes.into_iter().enumerate() {
        e.mem.set_u32(address.wrapping_add(4 * index as u32), lane);
    }
}

/// One lane of an SSE arithmetic instruction: a NaN in the destination operand wins
/// (quieted), then a NaN in the source operand, otherwise `op`.
pub(crate) fn sse_lane(dest: u32, source: u32, op: fn(f32, f32) -> f32) -> u32 {
    let quiet = 0x0040_0000;
    if f32::from_bits(dest).is_nan() {
        dest | quiet
    } else if f32::from_bits(source).is_nan() {
        source | quiet
    } else {
        op(f32::from_bits(dest), f32::from_bits(source)).to_bits()
    }
}

/// `ADDSS` / `ADDPS` on one lane.
pub(crate) fn sse_add(dest: u32, source: u32) -> u32 {
    sse_lane(dest, source, |a, b| a + b)
}

/// `SUBSS` / `SUBPS` on one lane.
pub(crate) fn sse_sub(dest: u32, source: u32) -> u32 {
    sse_lane(dest, source, |a, b| a - b)
}

/// `MULSS` / `MULPS` on one lane.
pub(crate) fn sse_mul(dest: u32, source: u32) -> u32 {
    sse_lane(dest, source, |a, b| a * b)
}

/// `MULPS dest, source`.
pub(crate) fn mulps(dest: Lanes, source: Lanes) -> Lanes {
    std::array::from_fn(|lane| sse_mul(dest[lane], source[lane]))
}

/// `SUBPS dest, source`.
pub(crate) fn subps(dest: Lanes, source: Lanes) -> Lanes {
    std::array::from_fn(|lane| sse_sub(dest[lane], source[lane]))
}

/// `SHUFPS dest, source, imm`: lanes 0 and 1 come from `dest`, lanes 2 and 3 from `source`.
pub(crate) fn shufps(dest: Lanes, source: Lanes, imm: u8) -> Lanes {
    let pick = |from: Lanes, shift: u8| from[usize::from((imm >> shift) & 3)];
    [
        pick(dest, 0),
        pick(dest, 2),
        pick(source, 4),
        pick(source, 6),
    ]
}

/// `RSQRTSS` on one lane: the host CPU's estimate of `1 / sqrt(x)` on x86 (the estimate
/// differs between Intel and AMD, so tests allow 1 ULP), exact elsewhere.
#[cfg(target_arch = "x86_64")]
pub(crate) fn rsqrt_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn rsqrt_estimate(x: f32) -> f32 {
    1.0 / x.sqrt()
}

/// The compiler's inline `hkArray::pushBack` prologue: asks the memory router and, when the
/// array's size equals its capacity, grows it (`00c90940` with the router's allocator at
/// `+allocator_slot`, the array and the element size).
pub(crate) fn make_room_with(e: &mut Engine, array: u32, element_size: u32, allocator_slot: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array.wrapping_add(8)) & CAPACITY_MASK;
    if e.mem.u32(array.wrapping_add(4)) == capacity {
        let allocator = e.mem.u32(router.wrapping_add(allocator_slot));
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element_size]);
    }
}

/// [`make_room_with`] for the router's usual allocator (`+0x10`).
fn make_room(e: &mut Engine, array: u32, element_size: u32) {
    make_room_with(e, array, element_size, 0x10);
}

/// Pushes one word on an array of words, growing it through the router's allocator at
/// `+allocator_slot` when full.
pub(crate) fn push_word_with(e: &mut Engine, array: u32, value: u32, allocator_slot: u32) {
    make_room_with(e, array, 4, allocator_slot);
    let size = e.mem.u32(array.wrapping_add(4));
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array.wrapping_add(4), size.wrapping_add(1));
}

/// Pushes one word on an array of words (after [`make_room`]).
fn push_word(e: &mut Engine, array: u32, value: u32) {
    push_word_with(e, array, value, 0x10);
}

/// Pushes a computed vector on an array of 16-byte vectors (after [`make_room`]).
fn push_lanes(e: &mut Engine, array: u32, lanes: Lanes) {
    make_room(e, array, 16);
    store_pushed(e, array, lanes);
}

/// Pushes the vector at `source` (read after [`make_room`], as the exe's `MOVAPS` is).
fn push_vector_from(e: &mut Engine, array: u32, source: u32) {
    make_room(e, array, 16);
    let lanes = load_lanes(e, source);
    store_pushed(e, array, lanes);
}

fn store_pushed(e: &mut Engine, array: u32, lanes: Lanes) {
    let size = e.mem.u32(array.wrapping_add(4));
    let data = e.mem.u32(array);
    store_lanes(e, data.wrapping_add(size.wrapping_shl(4)), lanes);
    e.mem.set_u32(array.wrapping_add(4), size.wrapping_add(1));
}

/// `ADD dword ptr [address], amount`.
fn add_to(e: &mut Engine, address: u32, amount: u32) {
    let value = e.mem.u32(address);
    e.mem.set_u32(address, value.wrapping_add(amount));
}

/// The scheme the builder writes (`[kit + 4]`, read again each time as the exe does).
fn scheme_of(e: &Engine, kit: Ptr) -> u32 {
    e.mem.u32(kit.addr().wrapping_add(4))
}

/// Appends `first` and `second` to the command array (two push-backs, the scheme read again
/// between them).
fn push_command_pair(e: &mut Engine, kit: Ptr, first: u32, second: u32) {
    let scheme = scheme_of(e, kit);
    push_word(e, scheme.wrapping_add(SCHEME_COMMANDS), first);
    let scheme = scheme_of(e, kit);
    push_word(e, scheme.wrapping_add(SCHEME_COMMANDS), second);
}

/// Appends one command word.
fn push_command(e: &mut Engine, kit: Ptr, command: u32) {
    let scheme = scheme_of(e, kit);
    push_word(e, scheme.wrapping_add(SCHEME_COMMANDS), command);
}

/// Adds `size` to the scheme size and `a` / `b` to its two counts.
fn add_costs(e: &mut Engine, scheme: u32, size: u32, a: u32, b: u32) {
    add_to(e, scheme.wrapping_add(SCHEME_SIZE), size);
    add_to(e, scheme.wrapping_add(SCHEME_COUNT_A), a);
    add_to(e, scheme.wrapping_add(SCHEME_COUNT_B), b);
}

/// Pushes the vector `lanes` on the scheme's vectors and answers the vector count from before.
fn push_scheme_vector(e: &mut Engine, kit: Ptr, lanes: Lanes) -> u32 {
    let scheme = scheme_of(e, kit);
    let old = e.mem.u32(scheme.wrapping_add(SCHEME_VECTOR_COUNT));
    push_lanes(e, scheme.wrapping_add(SCHEME_VECTORS), lanes);
    old
}

/// `FSTP float` then `FLD float` then `FSTP float`: the value stored as a float, with a
/// signalling NaN quieted by the reload.
fn stored_twice(value: f64) -> u32 {
    x87_quiet_bits(x87_store32(value))
}

// Translated from 00d51760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds up the cost of the commands in an array of words (cdecl, no `this`): `commands` points
/// at the `hkArray` (data, size) and `info` at the counters, `+4` size, `+8` and `+0xc` counts.
/// Each command word is followed by its operand words and adds a fixed amount: command 0 ends
/// the walk; 1, 2, 8 to 11, 22 and 23 and every word above 0x18 are one word and cost nothing;
/// 3, 4, 5 and 0x18 are two words and cost nothing; 6, 0x11 and 0x12 (two words) add
/// `0x30, 1, 1`; 7 and 0xd (one word) `0x90, 3, 3`; 0xc (two words) `0x60, 2, 2`; 0xe (two
/// words) `0x40, 1, 3`; 0xf (two words) `0x50, 1, 3`; 0x10 (two words) `0x40, 1, 1`; 0x13
/// (three words) `0x30, 1, 1`; 0x14 (two words) `0x30, 1, 2`; 0x15 (two words) `0x40, 1, 2`.
/// The walk stops at the end of the array (an unsigned comparison of the cursor).
pub fn fn_00d51760(e: &mut Engine, commands: Ptr, info: Ptr) {
    let data = e.mem.u32(commands.addr());
    let size = e.mem.u32(commands.addr().wrapping_add(4));
    let end = data.wrapping_add(size.wrapping_mul(4));
    let mut cursor = data;
    if cursor >= end {
        return;
    }
    let info = info.addr();
    loop {
        let command = e.mem.u32(cursor);
        // (size, count a, count b) added, and the words the command takes.
        let (cost, words): (Option<(u32, u32, u32)>, u32) = match command {
            0 => return,
            3..=5 | 0x18 => (None, 2),
            6 | 0x11 | 0x12 => (Some((0x30, 1, 1)), 2),
            7 | 0xd => (Some((0x90, 3, 3)), 1),
            0xc => (Some((0x60, 2, 2)), 2),
            0xe => (Some((0x40, 1, 3)), 2),
            0xf => (Some((0x50, 1, 3)), 2),
            0x10 => (Some((0x40, 1, 1)), 2),
            0x13 => (Some((0x30, 1, 1)), 3),
            0x14 => (Some((0x30, 1, 2)), 2),
            0x15 => (Some((0x40, 1, 2)), 2),
            _ => (None, 1),
        };
        if let Some((size, a, b)) = cost {
            add_costs(e, info, size, a, b);
        }
        cursor = cursor.wrapping_add(4 * words);
        if cursor >= end {
            return;
        }
    }
}

// Translated from 00d51890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 3 and `index` to the commands, pushes the vector at `vector` on the
/// vectors and sets the flag byte `kit + 0x10 + index`. Answers the vector count from before
/// the push.
pub fn fn_00d51890(e: &mut Engine, this: Ptr, vector: Ptr, index: u32) -> u32 {
    push_command_pair(e, this, 3, index);
    let scheme = scheme_of(e, this);
    let old = e.mem.u32(scheme.wrapping_add(SCHEME_VECTOR_COUNT));
    push_vector_from(e, scheme.wrapping_add(SCHEME_VECTORS), vector.addr());
    e.mem
        .set_u8(this.addr().wrapping_add(index).wrapping_add(0x10), 1);
    old
}

// Translated from 00d51970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d51890` with command 4 and the flag byte `kit + 0x13 + index`.
pub fn fn_00d51970(e: &mut Engine, this: Ptr, vector: Ptr, index: u32) -> u32 {
    push_command_pair(e, this, 4, index);
    let scheme = scheme_of(e, this);
    let old = e.mem.u32(scheme.wrapping_add(SCHEME_VECTOR_COUNT));
    push_vector_from(e, scheme.wrapping_add(SCHEME_VECTORS), vector.addr());
    e.mem
        .set_u8(this.addr().wrapping_add(index).wrapping_add(0x13), 1);
    old
}

// Translated from 00d51a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d51890` with command 5 and both flag bytes, `kit + 0x10 + index` and
/// `kit + 0x13 + index`.
pub fn fn_00d51a50(e: &mut Engine, this: Ptr, vector: Ptr, index: u32) -> u32 {
    push_command_pair(e, this, 5, index);
    let scheme = scheme_of(e, this);
    let old = e.mem.u32(scheme.wrapping_add(SCHEME_VECTOR_COUNT));
    push_vector_from(e, scheme.wrapping_add(SCHEME_VECTORS), vector.addr());
    e.mem
        .set_u8(this.addr().wrapping_add(index).wrapping_add(0x10), 1);
    e.mem
        .set_u8(this.addr().wrapping_add(index).wrapping_add(0x13), 1);
    old
}

// Translated from 00d51b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 6 and `value`, then adds `0x30, 1, 1` to the scheme's size and counts.
pub fn fn_00d51b30(e: &mut Engine, this: Ptr, value: u32) {
    push_command_pair(e, this, 6, value);
    let scheme = scheme_of(e, this);
    add_to(e, scheme.wrapping_add(SCHEME_COUNT_A), 1);
    add_to(e, scheme.wrapping_add(SCHEME_COUNT_B), 1);
    add_to(e, scheme.wrapping_add(SCHEME_SIZE), 0x30);
}

// Translated from 00d51bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 7, then three times adds `0x30, 1, 1` to the scheme's size and counts.
pub fn fn_00d51bc0(e: &mut Engine, this: Ptr) {
    push_command(e, this, 7);
    for _ in 0..3 {
        let scheme = scheme_of(e, this);
        add_costs(e, scheme, 0x30, 1, 1);
    }
}

// Translated from 00d51c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 8, pushes the three vectors at `vectors` (`+0`, `+0x10`, `+0x20`) on the
/// vectors and sets the flag byte `kit + 0x16`. Answers the vector count from before the pushes.
pub fn fn_00d51c30(e: &mut Engine, this: Ptr, vectors: Ptr) -> u32 {
    push_command(e, this, 8);
    let scheme = scheme_of(e, this);
    let old = e.mem.u32(scheme.wrapping_add(SCHEME_VECTOR_COUNT));
    push_vector_from(e, scheme.wrapping_add(SCHEME_VECTORS), vectors.addr());
    for offset in [0x10, 0x20] {
        let scheme = scheme_of(e, this);
        push_vector_from(
            e,
            scheme.wrapping_add(SCHEME_VECTORS),
            vectors.addr().wrapping_add(offset),
        );
    }
    e.mem.set_u8(this.addr().wrapping_add(0x16), 1);
    old
}

// Translated from 00d51d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d51c30` with command 9 and the flag byte `kit + 0x17`.
pub fn fn_00d51d40(e: &mut Engine, this: Ptr, vectors: Ptr) -> u32 {
    push_command(e, this, 9);
    let scheme = scheme_of(e, this);
    let old = e.mem.u32(scheme.wrapping_add(SCHEME_VECTOR_COUNT));
    push_vector_from(e, scheme.wrapping_add(SCHEME_VECTORS), vectors.addr());
    for offset in [0x10, 0x20] {
        let scheme = scheme_of(e, this);
        push_vector_from(
            e,
            scheme.wrapping_add(SCHEME_VECTORS),
            vectors.addr().wrapping_add(offset),
        );
    }
    e.mem.set_u8(this.addr().wrapping_add(0x17), 1);
    old
}

// Translated from 00d51e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0xa and sets the flag byte `kit + 0x16`.
pub fn fn_00d51e50(e: &mut Engine, this: Ptr) {
    push_command(e, this, 0xa);
    e.mem.set_u8(this.addr().wrapping_add(0x16), 1);
}

// Translated from 00d51ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0xb and sets the flag byte `kit + 0x17`.
pub fn fn_00d51ea0(e: &mut Engine, this: Ptr) {
    push_command(e, this, 0xb);
    e.mem.set_u8(this.addr().wrapping_add(0x17), 1);
}

// Translated from 00d51ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0xc and `value`, then twice adds `0x30, 1, 1` to the scheme's size and
/// counts.
pub fn fn_00d51ef0(e: &mut Engine, this: Ptr, value: u32) {
    push_command_pair(e, this, 0xc, value);
    for _ in 0..2 {
        let scheme = scheme_of(e, this);
        add_to(e, scheme.wrapping_add(SCHEME_COUNT_A), 1);
        add_to(e, scheme.wrapping_add(SCHEME_COUNT_B), 1);
        add_to(e, scheme.wrapping_add(SCHEME_SIZE), 0x30);
    }
}

// Translated from 00d51f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0xd, then three times adds `0x30, 1, 1` to the scheme's size and counts.
pub fn fn_00d51f90(e: &mut Engine, this: Ptr) {
    push_command(e, this, 0xd);
    for _ in 0..3 {
        let scheme = scheme_of(e, this);
        add_costs(e, scheme, 0x30, 1, 1);
    }
}

// Translated from 00d52000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 1, pushes the vector at `vector` on the vectors and sets the flag byte
/// `kit + 0x18`. Answers the vector count from before the push.
pub fn fn_00d52000(e: &mut Engine, this: Ptr, vector: Ptr) -> u32 {
    push_command(e, this, 1);
    let scheme = scheme_of(e, this);
    let old = e.mem.u32(scheme.wrapping_add(SCHEME_VECTOR_COUNT));
    push_vector_from(e, scheme.wrapping_add(SCHEME_VECTORS), vector.addr());
    e.mem.set_u8(this.addr().wrapping_add(0x18), 1);
    old
}

// Translated from 00d520a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d52000` with command 2 and the flag byte `kit + 0x19`.
pub fn fn_00d520a0(e: &mut Engine, this: Ptr, vector: Ptr) -> u32 {
    push_command(e, this, 2);
    let scheme = scheme_of(e, this);
    let old = e.mem.u32(scheme.wrapping_add(SCHEME_VECTOR_COUNT));
    push_vector_from(e, scheme.wrapping_add(SCHEME_VECTORS), vector.addr());
    e.mem.set_u8(this.addr().wrapping_add(0x19), 1);
    old
}

// Translated from 00d52140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the point `point` into the frame of each of two objects (`hkVector4::
/// setTransformedInversePos`, `00c7f690`, with the transform at `+0xf0` of `first` and of
/// `second`) into two local vectors, hands them to `00d52000` and `00d520a0`, and sets the
/// flag bytes `kit + 0x18` and `kit + 0x19`. Answers what `00d52000` answers (the vector count
/// from before its push).
pub fn fn_00d52140(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, point: Ptr) -> u32 {
    e.with_stack(0x20, |e, frame| {
        let (first_local, second_local) = (frame, frame.byte_add(0x10));
        e.call(
            SET_TRANSFORMED_INVERSE_POS,
            &args![first_local, first.addr().wrapping_add(0xf0), point],
        );
        e.call(
            SET_TRANSFORMED_INVERSE_POS,
            &args![second_local, second.addr().wrapping_add(0xf0), point],
        );
        let old = fn_00d52000(e, this, first_local);
        fn_00d520a0(e, this, second_local);
        e.mem.set_u8(this.addr().wrapping_add(0x18), 1);
        e.mem.set_u8(this.addr().wrapping_add(0x19), 1);
        old
    })
}

/// The vector `[x, y, FILL, FILL]` that `00d521b0` and `00d522b0` push, and the same shape
/// for the other builders: lanes from the bit patterns given, then the fill float twice.
fn vector_with_fill(e: &Engine, x: u32, y: u32) -> Lanes {
    let fill = e.mem.u32(FILL_LANE);
    [x, y, fill, fill]
}

// Translated from 00d521b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0x10 and `kind`, pushes the vector `[x, y, f, f]` (`f` the float at
/// `01011d78`) on the vectors and adds `0x40, 1, 1` to the scheme's size and counts. Answers
/// the vector count from before the push. (The decompiler shows two plain words stored at
/// `+0` and `+4`; the code builds the vector with `UNPCKLPS`.)
pub fn fn_00d521b0(e: &mut Engine, this: Ptr, kind: u8, x: f32, y: f32) -> u32 {
    push_command_pair(e, this, 0x10, u32::from(kind));
    let lanes = vector_with_fill(e, x.to_bits(), y.to_bits());
    let old = push_scheme_vector(e, this, lanes);
    let scheme = scheme_of(e, this);
    add_costs(e, scheme, 0x40, 1, 1);
    old
}

// Translated from 00d522b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d521b0` with command 0x11 and `0x30, 1, 1` added.
pub fn fn_00d522b0(e: &mut Engine, this: Ptr, kind: u8, x: f32, y: f32) -> u32 {
    push_command_pair(e, this, 0x11, u32::from(kind));
    let lanes = vector_with_fill(e, x.to_bits(), y.to_bits());
    let old = push_scheme_vector(e, this, lanes);
    let scheme = scheme_of(e, this);
    add_costs(e, scheme, 0x30, 1, 1);
    old
}

// Translated from 00d523b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0x12 and `kind`, takes the cosine of `angle` through the CRT helper
/// (`00ec9f70`, the float stored, reloaded and stored again), pushes the vector `[cos, g, f, f]`
/// (`g` the float at `01016410`, `f` the one at `01011d78`) and adds `0x30, 1, 1` to the
/// scheme's size and counts. Answers the vector count from before the push.
pub fn fn_00d523b0(e: &mut Engine, this: Ptr, kind: u8, angle: f32) -> u32 {
    push_command_pair(e, this, 0x12, u32::from(kind));
    let cosine = e
        .call(CRT_COSINE, &args![x87_load32(angle.to_bits())])
        .f64();
    let second = e.mem.u32(COSINE_SECOND_LANE);
    let lanes = vector_with_fill(e, stored_twice(cosine), second);
    let old = push_scheme_vector(e, this, lanes);
    let scheme = scheme_of(e, this);
    add_costs(e, scheme, 0x30, 1, 1);
    old
}

// Translated from 00d524d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0x13, `first_kind` and `second_kind`, takes the sine of `second_angle` and of
/// `first_angle` through the CRT helper (`00eca0a0`, in that order), pushes the vector
/// `[sin(first_angle), sin(second_angle), f, f]` (`f` the float at `01011d78`) and adds `0x30,
/// 1, 1` to the scheme's size and counts. Answers the vector count from before the push.
pub fn fn_00d524d0(
    e: &mut Engine,
    this: Ptr,
    first_kind: u8,
    second_kind: u8,
    first_angle: f32,
    second_angle: f32,
) -> u32 {
    let scheme = scheme_of(e, this);
    push_word(e, scheme.wrapping_add(SCHEME_COMMANDS), 0x13);
    push_command_pair(e, this, u32::from(first_kind), u32::from(second_kind));
    let second_sine = e
        .call(CRT_SINE, &args![x87_load32(second_angle.to_bits())])
        .f64();
    let second = stored_twice(second_sine);
    let first_sine = e
        .call(CRT_SINE, &args![x87_load32(first_angle.to_bits())])
        .f64();
    let first = stored_twice(first_sine);
    let lanes = vector_with_fill(e, first, second);
    let old = push_scheme_vector(e, this, lanes);
    let scheme = scheme_of(e, this);
    add_costs(e, scheme, 0x30, 1, 1);
    old
}

/// The tail of `00d52640` and `00d52790`: pushes `[count, f, f, f]` (`count` the object count
/// converted to a float, `f` the float at `01011d78`), takes a reference to `object`, pushes
/// the vector and the object, and adds to the scheme.
fn push_object_command(
    e: &mut Engine,
    this: Ptr,
    command: u32,
    kind: u8,
    object: u32,
    size: u32,
) -> u32 {
    push_command_pair(e, this, command, u32::from(kind));
    let scheme = scheme_of(e, this);
    let count = e.mem.i32(scheme.wrapping_add(SCHEME_OBJECT_COUNT)) as f32;
    let old = e.mem.u32(scheme.wrapping_add(SCHEME_VECTOR_COUNT));
    let fill = e.mem.u32(FILL_LANE);
    let lanes = [count.to_bits(), fill, fill, fill];
    e.call(ADD_REFERENCE, &args![object]);
    let scheme = scheme_of(e, this);
    push_lanes(e, scheme.wrapping_add(SCHEME_VECTORS), lanes);
    let scheme = scheme_of(e, this);
    push_word(e, scheme.wrapping_add(SCHEME_OBJECTS), object);
    let scheme = scheme_of(e, this);
    add_to(e, scheme.wrapping_add(SCHEME_COUNT_A), 1);
    add_to(e, scheme.wrapping_add(SCHEME_SIZE), size);
    add_to(e, scheme.wrapping_add(SCHEME_COUNT_B), 3);
    old
}

// Translated from 00d52640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0xe and `kind`, pushes the vector `[n, f, f, f]` (`n` the number of objects
/// already kept, at `scheme + 0x38`, converted to a float; `f` the float at `01011d78`), takes a
/// reference to `object` (`hkReferencedObject::addReference`, `00c90510`), keeps `object` in
/// the array at `scheme + 0x34` and adds `0x40, 1, 3` to the scheme's size and counts. Answers
/// the vector count from before the push.
pub fn fn_00d52640(e: &mut Engine, this: Ptr, kind: u8, object: u32) -> u32 {
    push_object_command(e, this, 0xe, kind, object, 0x40)
}

// Translated from 00d52790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d52640` with command 0xf and `0x50, 1, 3` added.
pub fn fn_00d52790(e: &mut Engine, this: Ptr, kind: u8, object: u32) -> u32 {
    push_object_command(e, this, 0xf, kind, object, 0x50)
}

/// The shared body of `00d528e0` and `00d529e0`.
fn push_value_command(
    e: &mut Engine,
    this: Ptr,
    command: u32,
    kind: u8,
    value: f32,
    size: u32,
) -> u32 {
    push_command_pair(e, this, command, u32::from(kind));
    let fill = e.mem.u32(FILL_LANE);
    let lanes = [value.to_bits(), fill, fill, fill];
    let old = push_scheme_vector(e, this, lanes);
    let scheme = scheme_of(e, this);
    add_to(e, scheme.wrapping_add(SCHEME_SIZE), size);
    add_to(e, scheme.wrapping_add(SCHEME_COUNT_A), 1);
    add_to(e, scheme.wrapping_add(SCHEME_COUNT_B), 2);
    old
}

// Translated from 00d528e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0x14 and `kind`, pushes the vector `[value, f, f, f]` (`f` the float at
/// `01011d78`) and adds `0x30, 1, 2` to the scheme's size and counts. Answers the vector count
/// from before the push.
pub fn fn_00d528e0(e: &mut Engine, this: Ptr, kind: u8, value: f32) -> u32 {
    push_value_command(e, this, 0x14, kind, value, 0x30)
}

// Translated from 00d529e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d528e0` with command 0x15 and `0x40, 1, 2` added.
pub fn fn_00d529e0(e: &mut Engine, this: Ptr, kind: u8, value: f32) -> u32 {
    push_value_command(e, this, 0x15, kind, value, 0x40)
}

// Translated from 00d52ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds one to the builder's own counter at `kit + 8`, appends command 0x16 and pushes the
/// vector `[value, f, f, f]` (`f` the float at `01011d78`). Answers the vector count from
/// before the push.
pub fn fn_00d52ae0(e: &mut Engine, this: Ptr, value: f32) -> u32 {
    let scheme = scheme_of(e, this);
    add_to(e, this.addr().wrapping_add(8), 1);
    push_word(e, scheme.wrapping_add(SCHEME_COMMANDS), 0x16);
    let fill = e.mem.u32(FILL_LANE);
    let lanes = [value.to_bits(), fill, fill, fill];
    push_scheme_vector(e, this, lanes)
}

// Translated from 00d52ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts one from the builder's own counter at `kit + 8` and appends command 0x17.
pub fn fn_00d52ba0(e: &mut Engine, this: Ptr) {
    let value = e.mem.u32(this.addr().wrapping_add(8));
    e.mem
        .set_u32(this.addr().wrapping_add(8), value.wrapping_sub(1));
    push_command(e, this, 0x17);
}

// Translated from 00d52be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0x18, keeps `word` in the words array at `scheme + 0x28` and appends
/// `command` to the commands.
pub fn fn_00d52be0(e: &mut Engine, this: Ptr, word: u32, command: u32) {
    push_command(e, this, 0x18);
    let scheme = scheme_of(e, this);
    push_word(e, scheme.wrapping_add(SCHEME_WORDS), word);
    push_command(e, this, command);
}

// Translated from 00d52ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends command 0 (the end marker).
pub fn fn_00d52ca0(e: &mut Engine, this: Ptr) {
    push_command(e, this, 0);
}

/// The normalise step the exe has inline in `00d52ce0` on lanes 0 to 2 of `v` (all four lanes
/// are scaled): `scale = (0.5 * r) * (3.0 - (len2 * r) * r)` with `r = RSQRTSS(len2)` and
/// `len2 = (z*z + (y*y + x*x))`, and `0` where `len2` is equal to the float at `01011d78`.
fn normalised(v: Lanes, half: u32, three: u32, zero: u32) -> Lanes {
    let squares = mulps(v, v);
    let len2 = sse_add(squares[2], sse_add(squares[1], squares[0]));
    let r = rsqrt_estimate(f32::from_bits(len2)).to_bits();
    let product = sse_mul(sse_mul(len2, r), r);
    let correction = sse_sub(three, product);
    let scaled = sse_mul(sse_mul(half, r), correction);
    let scale = if f32::from_bits(len2) == f32::from_bits(zero) {
        0
    } else {
        scaled
    };
    mulps([scale; 4], v)
}

// Translated from 00d52ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets up eight vectors at `this + 0x20 ..` from four input vectors (thiscall on an unnamed
/// 0x80-byte-plus object, `RET 0x10`; `RSQRTSS` is the host's estimate): `+0x20` is `first`
/// normalised, `+0x30` is `third` normalised, `+0x40` the cross product of the raw `first`
/// and `third` (`third x first` in the order of the shuffles), `+0x50` is `second`
/// normalised, `+0x60` is `fourth` normalised and `+0x70` the cross product of the raw
/// `second` and `fourth`. Normalising is the SSE sequence above ([`normalised`], with the
/// floats `01016248`, `01017718`, `01011d78`); lane 3 of the cross products is
/// `w * w - w * w` of the two inputs.
pub fn fn_00d52ce0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, third: Ptr, fourth: Ptr) {
    let at = this.addr();
    let first_raw = load_lanes(e, first.addr());
    store_lanes(e, at.wrapping_add(0x20), first_raw);
    let first_copy = load_lanes(e, at.wrapping_add(0x20));
    let zero = e.mem.u32(FILL_LANE);
    let half = e.mem.u32(NORMALISE_HALF);
    let three = e.mem.u32(NORMALISE_THREE);
    let result = normalised(first_copy, half, three, zero);
    store_lanes(e, at.wrapping_add(0x20), result);

    let third_raw = load_lanes(e, third.addr());
    store_lanes(e, at.wrapping_add(0x30), third_raw);
    let third_copy = load_lanes(e, at.wrapping_add(0x30));
    let result = normalised(third_copy, half, three, zero);
    store_lanes(e, at.wrapping_add(0x30), result);

    // Cross product of the raw first (a) and third (b) vectors.
    let a = load_lanes(e, first.addr());
    let b = load_lanes(e, third.addr());
    let a_zxy = shufps(a, a, 0xd2);
    let b_yzx = shufps(b, b, 0xc9);
    let positive = mulps(b_yzx, a_zxy);
    let b_zxy = shufps(b, b, 0xd2);
    let a_yzx = shufps(a, a, 0xc9);
    let negative = mulps(b_zxy, a_yzx);
    store_lanes(e, at.wrapping_add(0x40), subps(negative, positive));

    let second_raw = load_lanes(e, second.addr());
    store_lanes(e, at.wrapping_add(0x50), second_raw);
    let second_copy = load_lanes(e, at.wrapping_add(0x50));
    let result = normalised(second_copy, half, three, zero);
    store_lanes(e, at.wrapping_add(0x50), result);

    let fourth_raw = load_lanes(e, fourth.addr());
    store_lanes(e, at.wrapping_add(0x60), fourth_raw);
    let fourth_copy = load_lanes(e, at.wrapping_add(0x60));
    let result = normalised(fourth_copy, half, three, zero);
    store_lanes(e, at.wrapping_add(0x60), result);

    // Cross product of the raw second (a) and fourth (b) vectors.
    let a = load_lanes(e, second.addr());
    let b = load_lanes(e, fourth.addr());
    let a_yzx = shufps(a, a, 0xc9);
    let b_zxy = shufps(b, b, 0xd2);
    let first_product = mulps(a_yzx, b_zxy);
    let a_zxy = shufps(a, a, 0xd2);
    let b_yzx = shufps(b, b, 0xc9);
    let second_product = mulps(a_zxy, b_yzx);
    store_lanes(
        e,
        at.wrapping_add(0x70),
        subps(first_product, second_product),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_1760, fn_00d51760(Ptr, Ptr)),
        entry!(0x00d5_1890, fn_00d51890(Ptr, Ptr, u32) -> u32),
        entry!(0x00d5_1970, fn_00d51970(Ptr, Ptr, u32) -> u32),
        entry!(0x00d5_1a50, fn_00d51a50(Ptr, Ptr, u32) -> u32),
        entry!(0x00d5_1b30, fn_00d51b30(Ptr, u32)),
        entry!(0x00d5_1bc0, fn_00d51bc0(Ptr)),
        entry!(0x00d5_1c30, fn_00d51c30(Ptr, Ptr) -> u32),
        entry!(0x00d5_1d40, fn_00d51d40(Ptr, Ptr) -> u32),
        entry!(0x00d5_1e50, fn_00d51e50(Ptr)),
        entry!(0x00d5_1ea0, fn_00d51ea0(Ptr)),
        entry!(0x00d5_1ef0, fn_00d51ef0(Ptr, u32)),
        entry!(0x00d5_1f90, fn_00d51f90(Ptr)),
        entry!(0x00d5_2000, fn_00d52000(Ptr, Ptr) -> u32),
        entry!(0x00d5_20a0, fn_00d520a0(Ptr, Ptr) -> u32),
        entry!(0x00d5_2140, fn_00d52140(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d5_21b0, fn_00d521b0(Ptr, u8, f32, f32) -> u32),
        entry!(0x00d5_22b0, fn_00d522b0(Ptr, u8, f32, f32) -> u32),
        entry!(0x00d5_23b0, fn_00d523b0(Ptr, u8, f32) -> u32),
        entry!(0x00d5_24d0, fn_00d524d0(Ptr, u8, u8, f32, f32) -> u32),
        entry!(0x00d5_2640, fn_00d52640(Ptr, u8, u32) -> u32),
        entry!(0x00d5_2790, fn_00d52790(Ptr, u8, u32) -> u32),
        entry!(0x00d5_28e0, fn_00d528e0(Ptr, u8, f32) -> u32),
        entry!(0x00d5_29e0, fn_00d529e0(Ptr, u8, f32) -> u32),
        entry!(0x00d5_2ae0, fn_00d52ae0(Ptr, f32) -> u32),
        entry!(0x00d5_2ba0, fn_00d52ba0(Ptr)),
        entry!(0x00d5_2be0, fn_00d52be0(Ptr, u32, u32)),
        entry!(0x00d5_2ca0, fn_00d52ca0(Ptr)),
        entry!(0x00d5_2ce0, fn_00d52ce0(Ptr, Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A kit, its scheme (all four arrays with room for `capacity` elements) and doubles for
    /// the memory router, the array growth, `addReference` and the CRT helpers.
    struct Frame {
        e: Engine,
        kit: u32,
        scheme: u32,
        calls: Calls,
    }

    const ALLOCATOR: u32 = 0x7777;

    fn frame(capacity: u32) -> Frame {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(FILL_LANE, 0);
        e.mem.set_u32(COSINE_SECOND_LANE, 100f32.to_bits());
        e.mem.set_u32(NORMALISE_HALF, 0.5f32.to_bits());
        e.mem.set_u32(NORMALISE_THREE, 3.0f32.to_bits());
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let calls: Calls = Rc::default();
        let log = Rc::clone(&calls);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            log.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let (array, element) = (a[1], a[2]);
            let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
            let grown = (capacity * 2).max(4);
            let data = e.mem.alloc(grown * element);
            let size = e.mem.u32(array + 4);
            if size > 0 {
                let old = e.mem.u32(array);
                let bytes = e.mem.bytes(old, size * element);
                e.mem.write(data, &bytes);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, grown);
            Ret::default()
        });
        let log = Rc::clone(&calls);
        e.register_double(ADD_REFERENCE, move |_, a| {
            log.borrow_mut().push((ADD_REFERENCE, a.to_vec()));
            Ret::default()
        });
        e.register(CRT_COSINE, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).cos(),
            ..Ret::default()
        });
        e.register(CRT_SINE, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).sin(),
            ..Ret::default()
        });
        let scheme = e.mem.alloc(0x40);
        for (offset, element) in [
            (SCHEME_VECTORS, 16),
            (SCHEME_COMMANDS, 4),
            (SCHEME_WORDS, 4),
            (SCHEME_OBJECTS, 4),
        ] {
            let data = if capacity > 0 {
                e.mem.alloc(capacity * element)
            } else {
                0
            };
            e.mem.set_u32(scheme + offset, data);
            e.mem.set_u32(scheme + offset + 8, capacity);
        }
        let kit = e.mem.alloc(0x20);
        e.mem.set_u32(kit + 4, scheme);
        Frame {
            e,
            kit,
            scheme,
            calls,
        }
    }

    impl Frame {
        fn words(&self, offset: u32) -> Vec<u32> {
            let array = self.scheme + offset;
            let (data, size) = (self.e.mem.u32(array), self.e.mem.u32(array + 4));
            (0..size).map(|i| self.e.mem.u32(data + 4 * i)).collect()
        }
        fn commands(&self) -> Vec<u32> {
            self.words(SCHEME_COMMANDS)
        }
        fn vectors(&self) -> Vec<Lanes> {
            let array = self.scheme + SCHEME_VECTORS;
            let (data, size) = (self.e.mem.u32(array), self.e.mem.u32(array + 4));
            (0..size)
                .map(|i| load_lanes(&self.e, data + 16 * i))
                .collect()
        }
        fn counters(&self) -> [u32; 3] {
            [
                self.e.mem.u32(self.scheme + SCHEME_SIZE),
                self.e.mem.u32(self.scheme + SCHEME_COUNT_A),
                self.e.mem.u32(self.scheme + SCHEME_COUNT_B),
            ]
        }
        fn put_vector(&mut self, values: [f32; 4]) -> u32 {
            let at = self.e.mem.alloc(16);
            for (i, v) in values.iter().enumerate() {
                self.e.mem.set_f32(at + 4 * i as u32, *v);
            }
            at
        }
        fn calls_to(&self, address: u32) -> Vec<Vec<u32>> {
            self.calls
                .borrow()
                .iter()
                .filter(|(a, _)| *a == address)
                .map(|(_, args)| args.clone())
                .collect()
        }
    }

    fn bits(values: [f32; 4]) -> Lanes {
        values.map(f32::to_bits)
    }

    fn put_words(f: &mut Frame, words: &[u32]) -> u32 {
        let array = f.e.mem.alloc(0x10);
        let data = f.e.mem.alloc(4 * words.len() as u32);
        for (i, w) in words.iter().enumerate() {
            f.e.mem.set_u32(data + 4 * i as u32, *w);
        }
        f.e.mem.set_u32(array, data);
        f.e.mem.set_u32(array + 4, words.len() as u32);
        array
    }

    fn walk(f: &mut Frame, array: u32) -> [u32; 3] {
        let info = f.e.mem.alloc(0x10);
        f.e.call(0x00d5_1760, &args![array, info]);
        [
            f.e.mem.u32(info + 4),
            f.e.mem.u32(info + 8),
            f.e.mem.u32(info + 0xc),
        ]
    }

    #[test]
    fn fn_00d51760_adds_up_the_cost_of_each_command() {
        let mut f = frame(0);
        // 6, 7, 0xc, 0xe, 3, 0x13 and an unknown word (the walk stops at the command 0).
        let array = put_words(
            &mut f,
            &[6, 9, 7, 0xc, 9, 0xe, 9, 3, 9, 0x13, 9, 9, 0x30, 0, 6, 9],
        );
        assert_eq!(walk(&mut f, array), [0x190, 8, 10]);
        // The other commands, ending at the end of the array.
        let array = put_words(
            &mut f,
            &[
                0xf, 9, 0x10, 9, 0x14, 9, 0x15, 9, 0x11, 9, 0x12, 9, 0xd, 1, 2, 8, 9, 0xa, 0xb,
                0x16, 0x17, 0x18, 9, 0x19,
            ],
        );
        assert_eq!(walk(&mut f, array), [0x1f0, 9, 13]);
        // An empty array adds nothing.
        let array = put_words(&mut f, &[]);
        assert_eq!(walk(&mut f, array), [0, 0, 0]);
    }

    /// The `00d51890` family: command, index, vector and flag bytes; the count before the push.
    fn check_vector_command(address: u32, command: u32, flags: &[u32]) {
        let mut f = frame(8);
        let vector = f.put_vector([1.0, 2.0, 3.0, 4.0]);
        let first = f.e.call(address, &args![f.kit, vector, 2u32]).u32();
        let second = f.e.call(address, &args![f.kit, vector, 1u32]).u32();
        assert_eq!((first, second), (0, 1));
        assert_eq!(f.commands(), vec![command, 2, command, 1]);
        assert_eq!(f.vectors(), vec![bits([1.0, 2.0, 3.0, 4.0]); 2]);
        for flag in flags {
            assert_eq!(f.e.mem.u8(f.kit + flag + 2), 1);
            assert_eq!(f.e.mem.u8(f.kit + flag + 1), 1);
        }
        assert_eq!(f.counters(), [0, 0, 0]);
    }

    #[test]
    fn fn_00d51890_appends_command_three_and_flags_the_index() {
        check_vector_command(0x00d5_1890, 3, &[0x10]);
    }

    #[test]
    fn fn_00d51970_appends_command_four_and_flags_the_index() {
        check_vector_command(0x00d5_1970, 4, &[0x13]);
    }

    #[test]
    fn fn_00d51a50_appends_command_five_and_flags_both_indices() {
        check_vector_command(0x00d5_1a50, 5, &[0x10, 0x13]);
        let mut f = frame(8);
        let vector = f.put_vector([0.0; 4]);
        f.e.call(0x00d5_1a50, &args![f.kit, vector, 0u32]);
        assert_eq!(f.e.mem.u8(f.kit + 0x10), 1);
        assert_eq!(f.e.mem.u8(f.kit + 0x13), 1);
        assert_eq!(f.e.mem.u8(f.kit + 0x11), 0);
    }

    #[test]
    fn a_full_array_is_grown_through_the_array_routine_first() {
        let mut f = frame(0);
        let vector = f.put_vector([5.0, 6.0, 7.0, 8.0]);
        f.e.call(0x00d5_1890, &args![f.kit, vector, 7u32]);
        let grown = f.calls_to(ARRAY_RESERVE_MORE);
        assert_eq!(
            grown,
            // The commands array grows once (the second push finds room), the vectors once.
            vec![
                vec![ALLOCATOR, f.scheme + SCHEME_COMMANDS, 4],
                vec![ALLOCATOR, f.scheme + SCHEME_VECTORS, 16],
            ]
        );
        assert_eq!(f.commands(), vec![3, 7]);
        assert_eq!(f.vectors(), vec![bits([5.0, 6.0, 7.0, 8.0])]);
    }

    #[test]
    fn fn_00d51b30_appends_command_six_and_its_word() {
        let mut f = frame(8);
        f.e.call(0x00d5_1b30, &args![f.kit, 0x1234u32]);
        assert_eq!(f.commands(), vec![6, 0x1234]);
        assert_eq!(f.counters(), [0x30, 1, 1]);
    }

    #[test]
    fn fn_00d51bc0_appends_command_seven_and_three_costs() {
        let mut f = frame(8);
        f.e.call(0x00d5_1bc0, &args![f.kit]);
        assert_eq!(f.commands(), vec![7]);
        assert_eq!(f.counters(), [0x90, 3, 3]);
    }

    /// `00d51c30` and `00d51d40`: command, three vectors, the flag byte and the count before.
    fn check_three_vectors(address: u32, command: u32, flag: u32) {
        let mut f = frame(8);
        let source = f.e.mem.alloc(0x30);
        let rows = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
        ];
        for (i, row) in rows.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                f.e.mem.set_f32(source + 16 * i as u32 + 4 * j as u32, *v);
            }
        }
        let first = f.e.call(address, &args![f.kit, source]).u32();
        let second = f.e.call(address, &args![f.kit, source]).u32();
        assert_eq!((first, second), (0, 3));
        assert_eq!(f.commands(), vec![command, command]);
        let want: Vec<Lanes> = rows.iter().chain(rows.iter()).map(|r| bits(*r)).collect();
        assert_eq!(f.vectors(), want);
        assert_eq!(f.e.mem.u8(f.kit + flag), 1);
    }

    #[test]
    fn fn_00d51c30_appends_command_eight_and_three_vectors() {
        check_three_vectors(0x00d5_1c30, 8, 0x16);
    }

    #[test]
    fn fn_00d51d40_appends_command_nine_and_three_vectors() {
        check_three_vectors(0x00d5_1d40, 9, 0x17);
    }

    #[test]
    fn fn_00d51e50_appends_command_ten_and_flags_0x16() {
        let mut f = frame(8);
        f.e.call(0x00d5_1e50, &args![f.kit]);
        assert_eq!(f.commands(), vec![0xa]);
        assert_eq!(f.e.mem.u8(f.kit + 0x16), 1);
        assert_eq!(f.e.mem.u8(f.kit + 0x17), 0);
    }

    #[test]
    fn fn_00d51ea0_appends_command_eleven_and_flags_0x17() {
        let mut f = frame(8);
        f.e.call(0x00d5_1ea0, &args![f.kit]);
        assert_eq!(f.commands(), vec![0xb]);
        assert_eq!(f.e.mem.u8(f.kit + 0x17), 1);
        assert_eq!(f.e.mem.u8(f.kit + 0x16), 0);
    }

    #[test]
    fn fn_00d51ef0_appends_command_twelve_and_two_costs() {
        let mut f = frame(8);
        f.e.call(0x00d5_1ef0, &args![f.kit, 0x55u32]);
        assert_eq!(f.commands(), vec![0xc, 0x55]);
        assert_eq!(f.counters(), [0x60, 2, 2]);
    }

    #[test]
    fn fn_00d51f90_appends_command_thirteen_and_three_costs() {
        let mut f = frame(8);
        f.e.call(0x00d5_1f90, &args![f.kit]);
        assert_eq!(f.commands(), vec![0xd]);
        assert_eq!(f.counters(), [0x90, 3, 3]);
    }

    /// `00d52000` and `00d520a0`: command, one vector, the flag byte, the count before.
    fn check_one_vector(address: u32, command: u32, flag: u32) {
        let mut f = frame(8);
        let vector = f.put_vector([1.5, 2.5, 3.5, 4.5]);
        let first = f.e.call(address, &args![f.kit, vector]).u32();
        let second = f.e.call(address, &args![f.kit, vector]).u32();
        assert_eq!((first, second), (0, 1));
        assert_eq!(f.commands(), vec![command, command]);
        assert_eq!(f.vectors(), vec![bits([1.5, 2.5, 3.5, 4.5]); 2]);
        assert_eq!(f.e.mem.u8(f.kit + flag), 1);
    }

    #[test]
    fn fn_00d52000_appends_command_one_and_a_vector() {
        check_one_vector(0x00d5_2000, 1, 0x18);
    }

    #[test]
    fn fn_00d520a0_appends_command_two_and_a_vector() {
        check_one_vector(0x00d5_20a0, 2, 0x19);
    }

    #[test]
    fn fn_00d52140_takes_the_point_into_both_frames() {
        let mut f = frame(8);
        // The double writes its transform and vector arguments into its target.
        f.e.register(SET_TRANSFORMED_INVERSE_POS, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret::default()
        });
        let (first, second, point) = (0x0200_0000u32, 0x0300_0000u32, 0x0400_0000u32);
        let old =
            f.e.call(0x00d5_2140, &args![f.kit, first, second, point])
                .u32();
        assert_eq!(old, 0);
        assert_eq!(f.commands(), vec![1, 2]);
        let vectors = f.vectors();
        assert_eq!(vectors[0][..2], [first + 0xf0, point]);
        assert_eq!(vectors[1][..2], [second + 0xf0, point]);
        assert_eq!(f.e.mem.u8(f.kit + 0x18), 1);
        assert_eq!(f.e.mem.u8(f.kit + 0x19), 1);
    }

    #[test]
    fn fn_00d521b0_pushes_two_floats_and_the_fill() {
        let mut f = frame(8);
        f.e.mem.set_u32(FILL_LANE, 0x4040_0000);
        let words = args![f.kit, 5u32, 1.5f32, f32::from_bits(0x7fa0_0001)];
        let old = f.e.call(0x00d5_21b0, &words).u32();
        assert_eq!(old, 0);
        assert_eq!(f.commands(), vec![0x10, 5]);
        assert_eq!(
            f.vectors(),
            vec![[1.5f32.to_bits(), 0x7fa0_0001, 0x4040_0000, 0x4040_0000]]
        );
        assert_eq!(f.counters(), [0x40, 1, 1]);
    }

    #[test]
    fn fn_00d522b0_pushes_two_floats_and_the_fill() {
        let mut f = frame(8);
        f.e.call(0x00d5_22b0, &args![f.kit, 0x105u32, 2.5f32, -1.0f32]);
        assert_eq!(f.commands(), vec![0x11, 5]);
        assert_eq!(
            f.vectors(),
            vec![[2.5f32.to_bits(), (-1.0f32).to_bits(), 0, 0]]
        );
        assert_eq!(f.counters(), [0x30, 1, 1]);
    }

    #[test]
    fn fn_00d523b0_pushes_the_cosine_and_the_constants() {
        let mut f = frame(8);
        let angle = 0.5f32;
        f.e.call(0x00d5_23b0, &args![f.kit, 3u32, angle]);
        assert_eq!(f.commands(), vec![0x12, 3]);
        let cosine = (f64::from(angle).cos() as f32).to_bits();
        assert_eq!(f.vectors(), vec![[cosine, 100f32.to_bits(), 0, 0]]);
        assert_eq!(f.counters(), [0x30, 1, 1]);
        // A signalling NaN angle reaches the helper quieted; its NaN comes back quiet.
        let mut f = frame(8);
        f.e.register(CRT_COSINE, |_, a| {
            assert_eq!(a[1] & 0x7ff8_0000, 0x7ff8_0000);
            Ret {
                st0: f64::from_bits(0x7ff8_0000_0000_0001),
                ..Ret::default()
            }
        });
        f.e.call(
            0x00d5_23b0,
            &args![f.kit, 0u32, f32::from_bits(0x7f80_0001)],
        );
        assert_eq!(f.vectors()[0][0], 0x7fc0_0000);
    }

    #[test]
    fn fn_00d524d0_pushes_both_sines_in_the_exe_order() {
        let mut f = frame(8);
        let (first, second) = (0.25f32, 1.0f32);
        f.e.call(0x00d5_24d0, &args![f.kit, 1u32, 2u32, first, second]);
        assert_eq!(f.commands(), vec![0x13, 1, 2]);
        let sine = |x: f32| (f64::from(x).sin() as f32).to_bits();
        assert_eq!(f.vectors(), vec![[sine(first), sine(second), 0, 0]]);
        assert_eq!(f.counters(), [0x30, 1, 1]);
        // The second angle goes to the helper first.
        let mut f = frame(8);
        let order = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&order);
        f.e.register_double(CRT_SINE, move |_, a| {
            seen.borrow_mut()
                .push(f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)));
            Ret::default()
        });
        f.e.call(0x00d5_24d0, &args![f.kit, 0u32, 0u32, first, second]);
        assert_eq!(*order.borrow(), vec![1.0, 0.25]);
    }

    /// `00d52640` and `00d52790`: the object count as a float, the reference, the kept object.
    fn check_object_command(address: u32, command: u32, size: u32) {
        let mut f = frame(8);
        f.e.mem.set_u32(f.scheme + SCHEME_OBJECT_COUNT, 3);
        let object = 0x1234_5678u32;
        let old = f.e.call(address, &args![f.kit, 9u32, object]).u32();
        assert_eq!(old, 0);
        assert_eq!(f.commands(), vec![command, 9]);
        // The first lane is the object count (3) converted from the integer.
        assert_eq!(f.vectors(), vec![[3f32.to_bits(), 0, 0, 0]]);
        assert_eq!(f.calls_to(ADD_REFERENCE), vec![vec![object]]);
        assert_eq!(f.counters(), [size, 1, 3]);
        // The object went in after the count was read: 3 + 1 entries.
        assert_eq!(f.e.mem.u32(f.scheme + SCHEME_OBJECT_COUNT), 4);
        let data = f.e.mem.u32(f.scheme + SCHEME_OBJECTS);
        assert_eq!(f.e.mem.u32(data + 12), object);
    }

    #[test]
    fn fn_00d52640_keeps_the_object_and_takes_a_reference() {
        check_object_command(0x00d5_2640, 0xe, 0x40);
    }

    #[test]
    fn fn_00d52790_keeps_the_object_and_takes_a_reference() {
        check_object_command(0x00d5_2790, 0xf, 0x50);
    }

    #[test]
    fn fn_00d528e0_pushes_a_float_and_the_fill() {
        let mut f = frame(8);
        f.e.call(0x00d5_28e0, &args![f.kit, 4u32, 7.5f32]);
        assert_eq!(f.commands(), vec![0x14, 4]);
        assert_eq!(f.vectors(), vec![[7.5f32.to_bits(), 0, 0, 0]]);
        assert_eq!(f.counters(), [0x30, 1, 2]);
    }

    #[test]
    fn fn_00d529e0_pushes_a_float_and_the_fill() {
        let mut f = frame(8);
        f.e.call(0x00d5_29e0, &args![f.kit, 4u32, -7.5f32]);
        assert_eq!(f.commands(), vec![0x15, 4]);
        assert_eq!(f.vectors(), vec![[(-7.5f32).to_bits(), 0, 0, 0]]);
        assert_eq!(f.counters(), [0x40, 1, 2]);
    }

    #[test]
    fn fn_00d52ae0_counts_up_and_pushes_a_float() {
        let mut f = frame(8);
        f.e.mem.set_u32(f.kit + 8, 5);
        let old = f.e.call(0x00d5_2ae0, &args![f.kit, 0.125f32]).u32();
        assert_eq!(old, 0);
        assert_eq!(f.e.mem.u32(f.kit + 8), 6);
        assert_eq!(f.commands(), vec![0x16]);
        assert_eq!(f.vectors(), vec![[0.125f32.to_bits(), 0, 0, 0]]);
        assert_eq!(f.counters(), [0, 0, 0]);
    }

    #[test]
    fn fn_00d52ba0_counts_down_and_appends_command_23() {
        let mut f = frame(8);
        f.e.mem.set_u32(f.kit + 8, 5);
        f.e.call(0x00d5_2ba0, &args![f.kit]);
        assert_eq!(f.e.mem.u32(f.kit + 8), 4);
        assert_eq!(f.commands(), vec![0x17]);
    }

    #[test]
    fn fn_00d52be0_keeps_the_word_between_two_commands() {
        let mut f = frame(8);
        f.e.call(0x00d5_2be0, &args![f.kit, 0xaaaau32, 0xbbbbu32]);
        assert_eq!(f.commands(), vec![0x18, 0xbbbb]);
        assert_eq!(f.words(SCHEME_WORDS), vec![0xaaaa]);
    }

    #[test]
    fn fn_00d52ca0_appends_the_end_marker() {
        let mut f = frame(8);
        f.e.call(0x00d5_2ca0, &args![f.kit]);
        assert_eq!(f.commands(), vec![0]);
    }

    fn ulps(a: u32, b: u32) -> u32 {
        if a == b {
            return 0;
        }
        assert_eq!(a >> 31, b >> 31, "signs differ: {a:08x} {b:08x}");
        (a & 0x7fff_ffff).abs_diff(b & 0x7fff_ffff)
    }

    #[test]
    fn fn_00d52ce0_normalises_four_vectors_and_crosses_two_pairs() {
        let mut f = frame(0);
        let rows = [
            [3.0, 0.0, 4.0, 2.0],
            [1.0, 2.0, 2.0, 1.0],
            [0.0, 2.0, 0.0, 5.0],
            [0.0, 0.0, 5.0, 0.0],
        ];
        let first = f.put_vector(rows[0]);
        let second = f.put_vector(rows[1]);
        let third = f.put_vector(rows[2]);
        let fourth = f.put_vector(rows[3]);
        let this = f.e.mem.alloc(0x80);
        f.e.call(0x00d5_2ce0, &args![this, first, second, third, fourth]);
        // first, third, second and fourth are stored at +0x20, +0x30, +0x50 and +0x60.
        for (at, raw) in [
            (0x20, rows[0]),
            (0x30, rows[2]),
            (0x50, rows[1]),
            (0x60, rows[3]),
        ] {
            let len = (raw[0] * raw[0] + raw[1] * raw[1] + raw[2] * raw[2]).sqrt();
            let want = normalised(bits(raw), 0.5f32.to_bits(), 3.0f32.to_bits(), 0);
            for lane in 0..4usize {
                let got = f.e.mem.u32(this + at + 4 * lane as u32);
                assert!(ulps(got, want[lane]) <= 1);
                // Near the exact normalised value (the estimate is refined by one step).
                let exact = raw[lane] / len;
                assert!((f32::from_bits(got) - exact).abs() <= 1e-5);
            }
        }
        assert_eq!(load_lanes(&f.e, this + 0x40), bits([-8.0, 0.0, 6.0, 0.0]));
        assert_eq!(load_lanes(&f.e, this + 0x70), bits([10.0, -5.0, 0.0, 0.0]));
    }

    #[test]
    fn fn_00d52ce0_scales_a_zero_length_vector_to_zero() {
        let mut f = frame(0);
        let zero = f.put_vector([0.0, 0.0, 0.0, 7.0]);
        let unit = f.put_vector([0.0, 1.0, 0.0, 1.0]);
        let this = f.e.mem.alloc(0x80);
        f.e.call(0x00d5_2ce0, &args![this, zero, unit, zero, unit]);
        assert_eq!(load_lanes(&f.e, this + 0x20), [0; 4]);
        assert_eq!(load_lanes(&f.e, this + 0x30), [0; 4]);
        let got = load_lanes(&f.e, this + 0x50);
        assert!((f32::from_bits(got[1]) - 1.0).abs() <= 1e-6);
    }

    #[test]
    fn shufps_picks_lanes_of_dest_then_source() {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];
        assert_eq!(shufps(a, b, 0xd2), [3, 1, 6, 8]);
        assert_eq!(shufps(a, a, 0xc9), [2, 3, 1, 4]);
    }
}
