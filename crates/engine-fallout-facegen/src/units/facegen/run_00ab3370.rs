//! `FaceGen/run_00ab3370` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::VECTOR_LAST;
use crate::units::facegen::run_00ab0d50::{advance_checked, reallocate};
use crate::units::facegen::run_00aba3c0::{
    max_or_one, vector_capacity, vector_size, ElementSize, VECTOR_ALLOCATOR,
};

/// The vectors' "too long" throw (`0065f640`, no arguments; it does not return in the game).
const THROW_LENGTH: u32 = 0x0065_f640;
/// The byte vector's helpers: allocation (`0065fa30`, `cdecl`: count, hint), the copy
/// (`0065fea0`, `cdecl`: first, last, destination, allocator), the fill (`00ac3300`, `cdecl`:
/// destination, count, value address), the fill past the end (`00ad1740`, `thiscall` on the
/// vector: destination, count, value address), the copy past the end (`00ad5e20`, `cdecl`,
/// as the copy) and the backward move (`00ab4490`, `cdecl`: first, last, end).
const ALLOCATE_BYTES: u32 = 0x0065_fa30;
const COPY_BYTES: u32 = 0x0065_fea0;
const FILL_BYTES: u32 = 0x00ac_3300;
const FILL_BYTES_TAIL: u32 = 0x00ad_1740;
const COPY_TAIL_BYTES: u32 = 0x00ad_5e20;
const MOVE_BACKWARD_BYTES: u32 = 0x00ab_4490;
/// `memmove_s` (`00ec7ce3`, `cdecl`: destination, size, source, count).
const MEMMOVE_S: u32 = 0x00ec_7ce3;
/// The 0x58-byte element (a texture morph: four words and three byte vectors from +0x10)
/// vector's helpers: allocation (`0065e700`), the copies (`006603a0` before the gap and to
/// move the tail up, `0065ffe0` after it and past the end), the element destructor
/// (`0065eb50`, `cdecl`), the backward move (`00ab4520`, `cdecl`, six words), the element's
/// copy constructor (`0065fab0`, `thiscall`: the source) and assignment (`0065eb90`,
/// `thiscall`: the source).
const ALLOCATE_58: u32 = 0x0065_e700;
const COPY_58: u32 = 0x0066_03a0;
const COPY_AFTER_58: u32 = 0x0065_ffe0;
const DESTRUCT_58: u32 = 0x0065_eb50;
const MOVE_BACKWARD_58: u32 = 0x00ab_4520;
const ELEMENT_COPY_58: u32 = 0x0065_fab0;
const ELEMENT_ASSIGN_58: u32 = 0x0065_eb90;
/// The CRT's `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size,
/// count, destructor) and the byte vector destructor it runs on the three vectors of a
/// 0x58-byte element (`0065f060`).
const EH_VECTOR_DESTRUCTOR: u32 = 0x00ec_5fce;
const BYTE_VECTOR_DESTRUCT: u32 = 0x0065_f060;

/// The byte reallocation's fill (`00ac3300`, three words).
fn fill_bytes(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _allocator: u32,
    _tag_0: u32,
    _tag_1: u32,
) {
    e.call(FILL_BYTES, &args![destination, count, value]);
}

/// The 0x58-byte reallocation's destruction of an old element (`0065eb50`, `cdecl`).
fn destruct_58(e: &mut Engine, element: u32) {
    e.call(DESTRUCT_58, &args![element]);
}

/// The destruction of a 0x58-byte temporary: its three byte vectors (+0x10, 0x18 bytes each).
fn destroy_temporary_58(e: &mut Engine, temporary: u32) {
    e.call(
        EH_VECTOR_DESTRUCTOR,
        &args![temporary + 0x10, 0x18u32, 3u32, BYTE_VECTOR_DESTRUCT],
    );
}

// Translated from 00ab3370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a byte vector's `_Insert_n(where, count, value)` (`thiscall`, four words: the
/// iterator's proxy and pointer, the count, the value's address). Nothing for 0; more than the
/// maximum (0xFFFFFFFF) calls the length throw (`0065f640`). Past the capacity, a block of half
/// as much again (at least enough; `0065fa30`) gets the bytes before `where` (`0065fea0`),
/// `count` copies (`00ac3300`) and the rest, and the old block is freed (no walk). Otherwise,
/// when fewer than `count` bytes follow `where`, they move up (`0065fea0`), the gap past the old
/// end is filled (`00ad1740`) and the rest set; else the last `count` bytes are copied past the
/// end (`00ad5e20`), the others move up (`00ab4490`) and `count` bytes from `where` are set. The
/// value is read before anything moves.
// Translation incomplete: 00ab3370 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f1f930, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab3370(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let vector = this.addr();
    let size = ElementSize::Shift(0);
    let maximum = 0xffff_ffff;
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, size);
    if count == 0 {
        return;
    }
    if max_or_one(maximum).wrapping_sub(vector_size(e, vector, size)) < count {
        e.call(THROW_LENGTH, &[]);
        return;
    }
    if capacity < vector_size(e, vector, size).wrapping_add(count) {
        reallocate(
            e,
            vector,
            at,
            count,
            value,
            size,
            maximum,
            ALLOCATE_BYTES,
            COPY_BYTES,
            fill_bytes,
            COPY_BYTES,
            None,
        );
        return;
    }
    let after = e.mem.u32(vector + VECTOR_LAST).wrapping_sub(at);
    let byte = e.mem.u8(value);
    if after < count {
        e.with_stack(4, |e, local| {
            e.mem.set_u8(local.addr(), byte);
            let last = e.mem.u32(vector + VECTOR_LAST);
            e.call(
                COPY_BYTES,
                &args![at, last, at.wrapping_add(count), allocator],
            );
            let last = e.mem.u32(vector + VECTOR_LAST);
            let missing = count.wrapping_sub(last.wrapping_sub(at));
            e.call(FILL_BYTES_TAIL, &args![this, last, missing, local.addr()]);
        });
        let last = e.mem.u32(vector + VECTOR_LAST).wrapping_add(count);
        e.mem.set_u32(vector + VECTOR_LAST, last);
        let mut element = at;
        while element != last.wrapping_sub(count) {
            e.mem.set_u8(element, byte);
            element = element.wrapping_add(1);
        }
    } else {
        let old_last = e.mem.u32(vector + VECTOR_LAST);
        let tail = old_last.wrapping_sub(count);
        let last = e.mem.u32(vector + VECTOR_LAST);
        let new_last = e
            .call(COPY_TAIL_BYTES, &args![tail, old_last, last, allocator])
            .u32();
        e.mem.set_u32(vector + VECTOR_LAST, new_last);
        e.call(MOVE_BACKWARD_BYTES, &args![at, tail, old_last]);
        let mut element = at;
        while element != at.wrapping_add(count) {
            e.mem.set_u8(element, byte);
            element = element.wrapping_add(1);
        }
    }
}

// Translated from 00ab3a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a 0x58-byte-element vector's `_Insert_n(where, count, value)`, as `00ab3370` with
/// the maximum 0x2E8BA2E, the allocation `0065e700`, the copies `006603a0` and `0065ffe0`, the
/// fill `00ab45f0`, the element destructor `0065eb50` (run on every old element of a
/// reallocation) and the backward move `00ab4520`. Without a reallocation the value is first
/// copied into a temporary (`0065fab0`), the elements are set by assignment (`0065eb90`) and the
/// temporary's byte vectors are destroyed after (`00ec5fce` with `0065f060`).
// Translation incomplete: 00ab3a40 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f1f963, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab3a40(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let vector = this.addr();
    let size = ElementSize::Divide(0x58);
    let maximum = 0x02e8_ba2e;
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, size);
    if count == 0 {
        return;
    }
    if max_or_one(maximum).wrapping_sub(vector_size(e, vector, size)) < count {
        e.call(THROW_LENGTH, &[]);
        return;
    }
    if capacity < vector_size(e, vector, size).wrapping_add(count) {
        reallocate(
            e,
            vector,
            at,
            count,
            value,
            size,
            maximum,
            ALLOCATE_58,
            COPY_58,
            fn_00ab45f0,
            COPY_AFTER_58,
            Some(destruct_58),
        );
        return;
    }
    let after = size.count(e.mem.u32(vector + VECTOR_LAST).wrapping_sub(at));
    let span = count.wrapping_mul(0x58);
    e.with_stack(0x58, |e, temporary| {
        let temporary = temporary.addr();
        e.call(ELEMENT_COPY_58, &args![temporary, value]);
        let set = |e: &mut Engine, from: u32, to: u32| {
            let mut element = from;
            while element != to {
                e.call(ELEMENT_ASSIGN_58, &args![element, temporary]);
                element = element.wrapping_add(0x58);
            }
        };
        if after < count {
            let last = e.mem.u32(vector + VECTOR_LAST);
            e.call(COPY_58, &args![at, last, at.wrapping_add(span), allocator]);
            let last = e.mem.u32(vector + VECTOR_LAST);
            let missing = count.wrapping_sub(size.count(last.wrapping_sub(at)));
            fn_00ab45f0(e, last, missing, temporary, allocator, 0, 0);
            let last = span.wrapping_add(e.mem.u32(vector + VECTOR_LAST));
            e.mem.set_u32(vector + VECTOR_LAST, last);
            set(e, at, last.wrapping_sub(span));
        } else {
            let old_last = e.mem.u32(vector + VECTOR_LAST);
            let tail = old_last.wrapping_sub(span);
            let last = e.mem.u32(vector + VECTOR_LAST);
            let new_last = e
                .call(COPY_AFTER_58, &args![tail, old_last, last, allocator])
                .u32();
            e.mem.set_u32(vector + VECTOR_LAST, new_last);
            e.call(
                MOVE_BACKWARD_58,
                &args![at, tail, old_last, 0u32, 0u32, 0u32],
            );
            set(e, at, at.wrapping_add(span));
        }
        destroy_temporary_58(e, temporary);
    });
}

// Translated from 00ab45f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, six words, the last three not read: the allocator and two tags): `count`
/// copies of the 0x58-byte value at `value` (the copy constructor `0065fab0`) from
/// `destination` on (a null destination skips the copy but still steps).
// Translation incomplete: 00ab45f0 - the C++ exception frame (FS:[0] chain, handler 00f1f9a1) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab45f0(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count != 0 {
        if destination != 0 {
            e.call(ELEMENT_COPY_58, &args![destination, value]);
        }
        count -= 1;
        destination = destination.wrapping_add(0x58);
    }
}

// Translated from 00ab4310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x58-byte element iterator's `+=` (`thiscall`, the count;
/// see [`advance_checked`]). Returns `this`.
pub fn fn_00ab4310(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_checked(e, this.addr(), count, 0x58);
    this
}

// Translated from 00ab43d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte iterator's `+=` (`thiscall`, the count; see
/// [`advance_checked`], an element size of 1). Returns `this`.
pub fn fn_00ab43d0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_checked(e, this.addr(), count, 1);
    this
}

// Translated from 00ab4490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last, end): the bytes' backward move: for a
/// positive (signed) `last - first`, `memmove_s(end - n, n, first, n)`. Returns `end - n`.
pub fn fn_00ab4490(e: &mut Engine, first: u32, last: u32, end: u32) -> u32 {
    let count = last.wrapping_sub(first);
    let start = end.wrapping_sub(count);
    if count as i32 > 0 {
        e.call(MEMMOVE_S, &args![start, count, first, count]);
    }
    start
}

// Translated from 00ab4520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: first, last, the destination's end and three
/// tag words it does not read): assigns the 0x58-byte elements before `last` down to `first` to
/// the slots before `end`, from the back (`0065eb90`). Returns the destination's new start,
/// `end - (last - first) / 0x58 * 0x58` (a signed division).
pub fn fn_00ab4520(
    e: &mut Engine,
    first: u32,
    last: u32,
    end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start =
        end.wrapping_sub(((last.wrapping_sub(first) as i32 / 0x58) as u32).wrapping_mul(0x58));
    let mut target = end;
    let mut source = last;
    while first != source {
        source = source.wrapping_sub(0x58);
        target = target.wrapping_sub(0x58);
        e.call(ELEMENT_ASSIGN_58, &args![target, source]);
    }
    start
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ab_3370, fn_00ab3370(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_3a40, fn_00ab3a40(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_45f0, fn_00ab45f0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ab_4310, fn_00ab4310(Ptr, u32) -> Ptr),
        entry!(0x00ab_43d0, fn_00ab43d0(Ptr, u32) -> Ptr),
        entry!(0x00ab_4490, fn_00ab4490(u32, u32, u32) -> u32),
        entry!(0x00ab_4520, fn_00ab4520(u32, u32, u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::facegen::binaryfile::{
        INVALID_PARAMETER, MEMORY_MANAGER_FREE, MEMORY_MANAGER_OBJECT, VECTOR_FIRST, VECTOR_PROXY,
    };

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A forward copy of `[first, last)` to `destination` returning its end.
    fn copy(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
        if last > first {
            let bytes = e.mem.bytes(first, last - first);
            e.mem.write(destination, &bytes);
        }
        destination + (last - first)
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE_BYTES, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(ALLOCATE_58, |e, a| e.mem.alloc(0x58 * a[0]).into_ret());
        for addr in [COPY_BYTES, COPY_TAIL_BYTES, COPY_58, COPY_AFTER_58] {
            e.register(addr, |e, a| copy(e, a[0], a[1], a[2]).into_ret());
        }
        e.register(FILL_BYTES, |e, a| {
            let byte = e.mem.u8(a[2]);
            for i in 0..a[1] {
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.register(FILL_BYTES_TAIL, |e, a| {
            let byte = e.mem.u8(a[3]);
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, byte);
            }
            Ret::default()
        });
        e.register(MOVE_BACKWARD_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2] - (a[1] - a[0]), &bytes);
            Ret::default()
        });
        for addr in [
            THROW_LENGTH,
            MEMORY_MANAGER_FREE,
            DESTRUCT_58,
            MOVE_BACKWARD_58,
            ELEMENT_COPY_58,
            ELEMENT_ASSIGN_58,
            EH_VECTOR_DESTRUCTOR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn vector(e: &mut Engine, bytes: u32, count: u32, capacity: u32) -> Ptr {
        let vector = e.mem.alloc(0x18);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(vector + VECTOR_PROXY, proxy);
        let block = e.mem.alloc((bytes * capacity).max(4));
        for i in 0..count * bytes {
            e.mem.set_u8(block + i, (i / bytes) as u8 + 1);
        }
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        e.mem.set_u32(vector + VECTOR_LAST, block + bytes * count);
        e.mem.set_u32(vector + 0x14, block + bytes * capacity);
        Ptr::new(vector)
    }

    fn bytes_of(e: &Engine, vector: Ptr) -> Vec<u8> {
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let last = e.mem.u32(vector.addr() + VECTOR_LAST);
        e.mem.bytes(first, last - first)
    }

    #[test]
    fn byte_insert_reallocates_or_moves_in_place() {
        let mut e = engine();
        let value = e.mem.alloc(4);
        e.mem.set_u8(value, 9);
        let this = vector(&mut e, 1, 3, 3);
        let old = e.mem.u32(this.addr() + VECTOR_FIRST);
        fn_00ab3370(&mut e, this, 0, old + 1, 2, value);
        assert_eq!(bytes_of(&e, this), vec![1, 9, 9, 2, 3]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_BYTES), vec![vec![5, 0]]);
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, old]]
        );
        // Room, fewer after `where` than the count.
        let this = vector(&mut e, 1, 2, 8);
        let first = e.mem.u32(this.addr() + VECTOR_FIRST);
        fn_00ab3370(&mut e, this, 0, first + 1, 3, value);
        assert_eq!(bytes_of(&e, this), vec![1, 9, 9, 9, 2]);
        // Room, more after `where`.
        let this = vector(&mut e, 1, 4, 8);
        let first = e.mem.u32(this.addr() + VECTOR_FIRST);
        fn_00ab3370(&mut e, this, 0, first + 1, 1, value);
        assert_eq!(bytes_of(&e, this), vec![1, 9, 2, 3, 4]);
        // Nothing for 0.
        fn_00ab3370(&mut e, this, 0, first + 1, 0, value);
        assert_eq!(bytes_of(&e, this).len(), 5);
    }

    #[test]
    fn texture_morph_insert_copies_assigns_and_destroys_its_temporary() {
        let mut e = engine();
        let value = e.mem.alloc(0x58);
        // Room, more after `where`.
        let this = vector(&mut e, 0x58, 3, 8);
        let first = e.mem.u32(this.addr() + VECTOR_FIRST);
        fn_00ab3a40(&mut e, this, 0, first, 1, value);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + VECTOR_LAST), first + 4 * 0x58);
        let copies = calls_to(&log, ELEMENT_COPY_58);
        assert_eq!(copies.len(), 1);
        let temporary = copies[0][0];
        assert_eq!(copies[0][1], value);
        assert_eq!(
            calls_to(&log, ELEMENT_ASSIGN_58),
            vec![vec![first, temporary]]
        );
        assert_eq!(calls_to(&log, MOVE_BACKWARD_58).len(), 1);
        assert_eq!(
            calls_to(&log, EH_VECTOR_DESTRUCTOR),
            vec![vec![temporary + 0x10, 0x18, 3, BYTE_VECTOR_DESTRUCT]]
        );
        // Past the capacity: old elements destroyed, copies made by the fill.
        let this = vector(&mut e, 0x58, 2, 2);
        let old = e.mem.u32(this.addr() + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        fn_00ab3a40(&mut e, this, 0, old + 0x58, 2, value);
        let log = e.call_log.take().unwrap();
        let first = e.mem.u32(this.addr() + VECTOR_FIRST);
        assert_eq!(e.mem.u32(this.addr() + VECTOR_LAST), first + 4 * 0x58);
        assert_eq!(
            calls_to(&log, DESTRUCT_58),
            vec![vec![old], vec![old + 0x58]]
        );
        assert_eq!(
            calls_to(&log, ELEMENT_COPY_58),
            vec![vec![first + 0x58, value], vec![first + 0xb0, value]]
        );
        // Fewer after `where`: the gap filled, one element assigned.
        let this = vector(&mut e, 0x58, 1, 8);
        let first = e.mem.u32(this.addr() + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        fn_00ab3a40(&mut e, this, 0, first, 2, value);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + VECTOR_LAST), first + 3 * 0x58);
        assert_eq!(calls_to(&log, ELEMENT_COPY_58).len(), 2);
        assert_eq!(calls_to(&log, ELEMENT_ASSIGN_58).len(), 1);
        assert_eq!(calls_to(&log, EH_VECTOR_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn texture_morph_fill_skips_null_destinations() {
        let mut e = engine();
        let block = e.mem.alloc(0xb0);
        fn_00ab45f0(&mut e, block, 2, 0x1234, 0, 0, 0);
        fn_00ab45f0(&mut e, 0, 1, 0x1234, 0, 0, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ELEMENT_COPY_58),
            vec![vec![block, 0x1234], vec![block + 0x58, 0x1234]]
        );
    }

    #[test]
    fn iterator_advances_by_element_size() {
        let mut e = engine();
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        let block = e.mem.alloc(0x58 * 2);
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        e.mem.set_u32(vector + VECTOR_LAST, block + 0xb0);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, vector);
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, proxy);
        e.mem.set_u32(iterator + 4, block);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ab_4310, &args![iterator, 2u32]).u32(), iterator);
        assert_eq!(e.mem.u32(iterator + 4), block + 0xb0);
        assert_eq!(
            e.call(0x00ab_43d0, &args![iterator, u32::MAX]).u32(),
            iterator
        );
        assert_eq!(e.mem.u32(iterator + 4), block + 0xaf);
        assert!(calls_to(e.call_log.as_ref().unwrap(), INVALID_PARAMETER).is_empty());
        e.call(0x00ab_43d0, &args![iterator, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(iterator + 4), block + 0xb1);
        assert_eq!(calls_to(&log, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn backward_moves_of_bytes_and_elements() {
        let mut e = Engine::new();
        e.register(MEMMOVE_S, |_, _| Ret::default());
        e.register(ELEMENT_ASSIGN_58, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ab_4490, &args![0x100u32, 0x110u32, 0x200u32])
                .u32(),
            0x1f0
        );
        assert_eq!(
            e.call(0x00ab_4490, &args![0x110u32, 0x110u32, 0x200u32])
                .u32(),
            0x200
        );
        let start = e
            .call(
                0x00ab_4520,
                &args![0x1000u32, 0x10b0u32, 0x2000u32, 0u32, 0u32, 0u32],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMMOVE_S),
            vec![vec![0x1f0, 0x10, 0x100, 0x10]]
        );
        assert_eq!(start, 0x2000 - 0xb0);
        assert_eq!(
            calls_to(&log, ELEMENT_ASSIGN_58),
            vec![vec![0x2000 - 0x58, 0x1058], vec![0x2000 - 0xb0, 0x1000]]
        );
    }
}
