//! `Havok SDK/hkafootstepanalysisinfo.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaFootstepAnalysisInfo` (Xbox PDB name) is a 0x98-byte object (vtable `010ebd64`, a 16-bit
//! reference count at `+6`). The class is not in the Xbox PDB's type list, so the fields are read
//! off the code. It holds five `hkArray`s of characters (data, size, capacity word: at `+8`,
//! `+0x14`, `+0x20`, `+0x2c` and `+0x38`), six `hkArray`s of `float` (at `+0x44`, `+0x50`,
//! `+0x5c`, `+0x68`, `+0x74` and `+0x80`) and three `float`s (`+0x8c`, `+0x90`, `+0x94`). The
//! container (vtable `010e56d4`) keeps an `hkArray` of pointers to such objects at `+8`.
//! Nothing here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator. The allocator's vtable
/// offsets used here: `4` allocates a block, `0xc` allocates and reports the size it really
/// gave (through the pointer it is passed), `0x10` frees an array buffer `(data, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const SLOT_ALLOCATE: u32 = 4;
const SLOT_ALLOCATE_SIZED: u32 = 0xc;
const SLOT_FREE_ARRAY: u32 = 0x10;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// `_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkReferencedObject` add and remove reference (`00c90510`, `00c905b0`; `this` only).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkString::strLen` (`00cb10d0`, cdecl `(text)`) and `hkString::strCpy` (`00cb1090`, cdecl
/// `(destination, source)`).
const STRING_LENGTH: u32 = 0x00cb_10d0;
const STRING_COPY: u32 = 0x00cb_1090;
/// The vtables written: the object's (`010ebd64`), the container's (`010e56d4`) and the base
/// one both end with (`0102e388`).
const INFO_VTABLE: u32 = 0x010e_bd64;
const CONTAINER_VTABLE: u32 = 0x010e_56d4;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The top bit of a capacity word marks storage that is not owned; the rest is the capacity.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The size of the object, which the container's creator stores at `+4`.
const INFO_SIZE: u32 = 0x98;

/// The allocator of the thread's memory router.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Gives the buffer of the `hkArray` at `array` (data, size, capacity word) back to the
/// allocator when it is owned (`capacity * element` bytes), clears its size and leaves it empty
/// and not owned.
fn release_array(e: &mut Engine, array: u32, element: u32) {
    let allocator = router_allocator(e);
    let flags = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if flags & NOT_OWNED == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            SLOT_FREE_ARRAY,
            &args![data, (flags & CAPACITY_MASK).wrapping_mul(element)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// Sets up the `float` array at `array` for `count` entries: empty first, then, for a non-zero
/// `count`, a buffer of `count * 4` bytes from the allocator's sized allocation (slot `0xc`,
/// which may report a different size back); the capacity is the size it reports divided by 4
/// (toward zero), the array's size is that capacity and every entry is set to the bits `fill`.
fn new_float_array(e: &mut Engine, array: u32, count: u32, fill: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
    let allocator = router_allocator(e);
    let (data, capacity) = if count == 0 {
        (0, 0)
    } else {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), count.wrapping_mul(4));
            let data = e.vcall(allocator, SLOT_ALLOCATE_SIZED, &args![slot]).u32();
            let bytes = e.mem.u32(slot.addr()) as i32;
            (data, bytes / 4)
        })
    };
    let flags = if capacity == 0 {
        NOT_OWNED
    } else {
        capacity as u32
    };
    e.mem.set_u32(array, data);
    e.mem.set_u32(array + 4, capacity as u32);
    e.mem.set_u32(array + 8, flags);
    if capacity > 0 {
        for index in 0..capacity as u32 {
            e.mem.set_u32(data.wrapping_add(4 * index), fill);
        }
    }
}

/// Stores a copy of the C string `source` in the character array at `array`: the capacity is
/// raised (to the larger of twice the old one and the length plus one) through `_reserve` when
/// it is smaller than the length plus one, the size becomes the length plus one and the text
/// (with its terminator) is copied with `strCpy`.
fn set_string(e: &mut Engine, array: u32, source: u32) {
    let needed = e.call(STRING_LENGTH, &args![source]).u32().wrapping_add(1);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if (capacity as i32) < (needed as i32) {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if (needed as i32) < (doubled as i32) {
            doubled
        } else {
            needed
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 1u32]);
    }
    e.mem.set_u32(array + 4, needed);
    let data = e.mem.u32(array);
    e.call(STRING_COPY, &args![data, source]);
}

// Translated from 00e40800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaFootstepAnalysisInfo::hkaFootstepAnalysisInfo` (Xbox PDB), `this` = the object, then
/// `(count, a, b, c, name, ..., ...)` (`RET 0x24`; the float arguments go through `FLD`/`FSTP`).
/// Writes the vtable and the reference count `1`, empties the five character arrays, sets up the
/// six `float` arrays for `count` entries each (`new_float_array`; filled with `0x7f7fffee`,
/// `0xff7fffee`, `0x7f7fffee`, `0xff7fffee`, `0` and `0x3f800000`), stores `b` at `+0x8c`, `c`
/// at `+0x90` and `a` at `+0x94`, and copies the five C strings into the character arrays in
/// order. Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn hka_footstep_analysis_info(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    a: f32,
    b: f32,
    c: f32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, INFO_VTABLE);
    e.mem.set_u16(t + 6, 1);
    for array in [0x8, 0x14, 0x20, 0x2c, 0x38, 0x44] {
        e.mem.set_u32(t + array, 0);
        e.mem.set_u32(t + array + 4, 0);
        e.mem.set_u32(t + array + 8, NOT_OWNED);
    }
    new_float_array(e, t + 0x44, count, 0x7f7f_ffee);
    new_float_array(e, t + 0x50, count, 0xff7f_ffee);
    new_float_array(e, t + 0x5c, count, 0x7f7f_ffee);
    new_float_array(e, t + 0x68, count, 0xff7f_ffee);
    new_float_array(e, t + 0x74, count, 0);
    new_float_array(e, t + 0x80, count, 0x3f80_0000);
    e.mem.set_f32(t + 0x8c, x87_quiet(b));
    e.mem.set_f32(t + 0x90, x87_quiet(c));
    e.mem.set_f32(t + 0x94, x87_quiet(a));
    set_string(e, t + 0x8, first);
    set_string(e, t + 0x14, second);
    set_string(e, t + 0x20, third);
    set_string(e, t + 0x2c, fourth);
    set_string(e, t + 0x38, fifth);
    this
}

// Translated from 00e40ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor that only sets the vtable (`010ebd64`) of the object in `this` and returns it
/// (`RET 4`, the stack word is not read; the signature is unconfirmed).
pub fn constructor_00e40ca0(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    e.mem.set_u32(this.addr(), INFO_VTABLE);
    this
}

// Translated from 00e40cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaFootstepAnalysisInfo` destructor, `this` = the object: writes the vtable, then empties
/// the arrays from the last to the first, each by giving an owned buffer back to the allocator
/// (the six `float` arrays at 4 bytes per entry, the five character arrays at 1) and ending with
/// size `0`, data `0` and the capacity word `0x80000000`; the base vtable `0102e388` ends it.
pub fn hka_footstep_analysis_info_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, INFO_VTABLE);
    for array in [0x80, 0x74, 0x68, 0x5c, 0x50, 0x44] {
        release_array(e, t + array, 4);
    }
    for array in [0x38, 0x2c, 0x20, 0x14, 0x8] {
        release_array(e, t + array, 1);
    }
    e.mem.set_u32(t, BASE_VTABLE);
}

// Translated from 00e40ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of the container (`this`, its pointer array at `+8`; `RET 0x24`) that makes a new
/// `hkaFootstepAnalysisInfo` out of its nine stack words: takes `0x98` bytes from the allocator
/// (slot `4`), stores the size in the 16-bit field at `+4`, runs the constructor `00e40800` on
/// them (the float words pass through `FLD`/`FSTP`), adds a reference, appends the object to the
/// pointer array (`_reserveMore` when it is full; the entry takes a reference of its own) and
/// drops the creator's reference. Returns the object read back from the array's last entry.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e40ec0(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    a: f32,
    b: f32,
    c: f32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
) -> u32 {
    let t = this.addr();
    let allocator = router_allocator(e);
    let block = e.vcall(allocator, SLOT_ALLOCATE, &args![INFO_SIZE]).u32();
    e.mem.set_u16(block + 4, INFO_SIZE as u16);
    let object = hka_footstep_analysis_info(
        e,
        Ptr::new(block),
        count,
        x87_quiet(a),
        x87_quiet(b),
        x87_quiet(c),
        first,
        second,
        third,
        fourth,
        fifth,
    )
    .addr();
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    let array = t + 8;
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let slot = e
        .mem
        .u32(array)
        .wrapping_add(e.mem.u32(array + 4).wrapping_mul(4));
    if slot != 0 {
        if object != 0 {
            e.call(ADD_REFERENCE, &args![object]);
        }
        e.mem.set_u32(slot, object);
    }
    e.mem
        .set_u32(array + 4, e.mem.u32(array + 4).wrapping_add(1));
    e.call(REMOVE_REFERENCE, &args![object]);
    let last = e.mem.u32(
        e.mem
            .u32(array)
            .wrapping_add(e.mem.u32(t + 0xc).wrapping_mul(4))
            .wrapping_sub(4),
    );
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    last
}

// Translated from 00e40fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the container (vtable `010e56d4`, pointer array at `+8`, `this`): drops the
/// reference of every non-null entry (`00c905b0`) and clears the entry, then gives the array
/// back to the allocator when it is owned (slot `0x10`, `capacity * 4` bytes), leaving it empty
/// and not owned, and ends with the base vtable `0102e388`. The allocator is asked for once,
/// before the loop.
pub fn container_destructor_00e40fd0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, CONTAINER_VTABLE);
    let allocator = router_allocator(e);
    let count = e.mem.u32(t + 0xc) as i32;
    let data = e.mem.u32(t + 8);
    let mut index = 0u32;
    if count > 0 {
        loop {
            let entry = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
            if entry != 0 {
                e.call(REMOVE_REFERENCE, &args![entry]);
            }
            e.mem.set_u32(data.wrapping_add(index.wrapping_mul(4)), 0);
            index = index.wrapping_add(1);
            if (index as i32) >= count {
                break;
            }
        }
    }
    let flags = e.mem.u32(t + 0x10);
    e.mem.set_u32(t + 0xc, 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(t + 8);
        e.vcall(
            allocator,
            SLOT_FREE_ARRAY,
            &args![data, (flags & CAPACITY_MASK).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0x10, NOT_OWNED);
    e.mem.set_u32(t, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_0800, hka_footstep_analysis_info(Ptr, u32, f32, f32, f32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x00e4_0ca0, constructor_00e40ca0(Ptr, u32) -> Ptr),
        entry!(0x00e4_0cb0, hka_footstep_analysis_info_destructor(Ptr)),
        entry!(0x00e4_0ec0, fn_00e40ec0(Ptr, u32, f32, f32, f32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e4_0fd0, container_destructor_00e40fd0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Vec<u32>;
    type Log = Rc<RefCell<Vec<(u32, Words)>>>;

    const ALLOCATE_SLOT: u32 = 0x0900_0021;
    const ALLOCATE_SIZED_SLOT: u32 = 0x0900_0022;
    const FREE_ARRAY_SLOT: u32 = 0x0900_0023;
    /// How many bytes the sized allocation reports back, relative to what was asked for.
    const REPORTED_EXTRA: u32 = 4;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// The words of every logged call with the tag `tag`.
    fn logged(log: &Log, tag: u32) -> Vec<Words> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == tag)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An engine with the memory router, its allocator (slot `4` hands out fresh blocks, slot
    /// `0xc` the same but reports [`REPORTED_EXTRA`] more bytes than asked for, slot `0x10`
    /// logs), `strLen` and `strCpy` that work on C strings, `_reserve` (moves the array to a
    /// buffer of the capacity asked for and sets the capacity word to it), `_reserveMore`
    /// (doubles the capacity, at least 2) and logging add and remove reference routines. Returns
    /// the log and the allocator.
    fn fixture() -> (Engine, Log, u32) {
        let mut e = engine();
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 0xc, ALLOCATE_SIZED_SLOT);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            l.borrow_mut().push((4, a[1..].to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let l = Rc::clone(&log);
        e.register_double(ALLOCATE_SIZED_SLOT, move |e, a| {
            let asked = e.mem.u32(a[1]);
            l.borrow_mut().push((0xc, vec![asked]));
            e.mem.set_u32(a[1], asked + REPORTED_EXTRA);
            Ret {
                eax: e.mem.alloc(asked + REPORTED_EXTRA),
                ..Ret::default()
            }
        });
        let l = Rc::clone(&log);
        e.register_double(FREE_ARRAY_SLOT, move |_, a| {
            l.borrow_mut().push((0x10, a[1..].to_vec()));
            Ret::default()
        });
        e.register_double(STRING_LENGTH, move |e, a| {
            let mut length = 0;
            while e.mem.u8(a[0] + length) != 0 {
                length += 1;
            }
            Ret {
                eax: length,
                ..Ret::default()
            }
        });
        e.register_double(STRING_COPY, move |e, a| {
            let mut at = 0;
            loop {
                let byte = e.mem.u8(a[1] + at);
                e.mem.set_u8(a[0] + at, byte);
                if byte == 0 {
                    break;
                }
                at += 1;
            }
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            l.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            regrow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            l.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let doubled = ((e.mem.u32(a[1] + 8) & CAPACITY_MASK) * 2).max(2);
            regrow(e, a[1], doubled, a[2]);
            Ret::default()
        });
        for address in [ADD_REFERENCE, REMOVE_REFERENCE] {
            let l = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                l.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (e, log, allocator)
    }

    /// Moves the `hkArray` at `array` to a fresh buffer of `capacity` entries of `element`
    /// bytes holding the old contents and records the capacity.
    fn regrow(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let old = e.mem.u32(array);
        let used = e.mem.u32(array + 4) * element;
        let fresh = e.mem.alloc(capacity * element);
        let kept = e.mem.bytes(old, used);
        e.mem.write(fresh, &kept);
        e.mem.set_u32(array, fresh);
        e.mem.set_u32(array + 8, capacity);
    }

    /// A C string in memory.
    fn text(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(at, text);
        at
    }

    /// Builds an object with `count` entries per `float` array and the strings
    /// `"one"` to `"five"`, and the floats `1.5`, `2.5` and `3.5`.
    fn build(e: &mut Engine, count: u32) -> u32 {
        let this = e.mem.alloc(INFO_SIZE);
        let names: Vec<u32> = [&b"one"[..], b"two", b"three", b"four", b"five"]
            .iter()
            .map(|name| text(e, name))
            .collect();
        e.call(
            0x00e4_0800,
            &args![
                this, count, 1.5f32, 2.5f32, 3.5f32, names[0], names[1], names[2], names[3],
                names[4]
            ],
        );
        this
    }

    #[test]
    fn the_constructor_sets_up_the_arrays_the_floats_and_the_strings() {
        let (mut e, log, allocator) = fixture();
        let this = build(&mut e, 3);
        assert_eq!(e.mem.u32(this), INFO_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        // 12 bytes asked for, 16 reported: capacity 4, and the size is that capacity.
        assert_eq!(logged(&log, 0xc), vec![vec![12]; 6]);
        for (array, fill) in [
            (0x44, 0x7f7f_ffee),
            (0x50, 0xff7f_ffee),
            (0x5c, 0x7f7f_ffee),
            (0x68, 0xff7f_ffee),
            (0x74, 0),
            (0x80, 0x3f80_0000),
        ] {
            let data = e.mem.u32(this + array);
            assert_ne!(data, 0);
            assert_eq!(e.mem.u32(this + array + 4), 4);
            assert_eq!(e.mem.u32(this + array + 8), 4);
            for index in 0..4 {
                assert_eq!(e.mem.u32(data + 4 * index), fill, "{array:#x}");
            }
        }
        assert_eq!(e.mem.f32(this + 0x8c), 2.5);
        assert_eq!(e.mem.f32(this + 0x90), 3.5);
        assert_eq!(e.mem.f32(this + 0x94), 1.5);
        // Each string gets its length plus one reserved (the capacity was zero) and is copied.
        let reserves = logged(&log, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 5);
        assert_eq!(reserves[0], [allocator, this + 8, 4, 1]);
        assert_eq!(reserves[2], [allocator, this + 0x20, 6, 1]);
        for (array, word) in [
            (0x8, &b"one\0"[..]),
            (0x14, b"two\0"),
            (0x20, b"three\0"),
            (0x2c, b"four\0"),
            (0x38, b"five\0"),
        ] {
            let data = e.mem.u32(this + array);
            assert_eq!(e.mem.bytes(data, word.len() as u32), word);
            assert_eq!(e.mem.u32(this + array + 4), word.len() as u32);
        }
    }

    #[test]
    fn a_zero_count_or_a_tiny_allocation_leaves_the_float_arrays_empty() {
        let (mut e, log, _) = fixture();
        let this = build(&mut e, 0);
        assert!(logged(&log, 0xc).is_empty());
        for array in [0x44, 0x50, 0x5c, 0x68, 0x74, 0x80] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), NOT_OWNED);
        }
        // The allocator answering less than four bytes gives a capacity of zero but keeps the
        // buffer it returned.
        e.register_double(ALLOCATE_SIZED_SLOT, move |e, a| {
            e.mem.set_u32(a[1], 3);
            Ret {
                eax: e.mem.alloc(8),
                ..Ret::default()
            }
        });
        let this = build(&mut e, 1);
        let data = e.mem.u32(this + 0x44);
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x4c), NOT_OWNED);
        assert_eq!(e.mem.u32(data), 0);
    }

    #[test]
    fn the_signalling_nan_floats_are_quieted() {
        let (mut e, _log, _) = fixture();
        let this = e.mem.alloc(INFO_SIZE);
        let name = text(&mut e, b"x");
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(
            0x00e4_0800,
            &args![this, 0u32, signalling, signalling, signalling, name, name, name, name, name],
        );
        for field in [0x8c, 0x90, 0x94] {
            assert_eq!(e.mem.u32(this + field), 0x7fc0_0001);
        }
    }

    #[test]
    fn constructor_00e40ca0_only_sets_the_vtable() {
        let mut e = engine();
        let this = e.mem.alloc(INFO_SIZE);
        e.mem.set_u32(this + 8, 0x1234);
        let got = e.call(0x00e4_0ca0, &args![this, 7u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), INFO_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x1234);
    }

    #[test]
    fn the_destructor_frees_owned_arrays_from_the_last_to_the_first() {
        let (mut e, log, _) = fixture();
        let this = build(&mut e, 3);
        // Make the array at +0x5c and the character array at +0x2c unowned.
        let unowned_float = e.mem.u32(this + 0x5c);
        e.mem.set_u32(this + 0x64, NOT_OWNED | 4);
        let unowned_text = e.mem.u32(this + 0x2c);
        e.mem.set_u32(this + 0x34, NOT_OWNED | 5);
        let owned: Vec<u32> = [0x80, 0x74, 0x68, 0x50, 0x44, 0x38, 0x20, 0x14, 0x8]
            .iter()
            .map(|array| e.mem.u32(this + array))
            .collect();
        log.borrow_mut().clear();
        e.call(0x00e4_0cb0, &args![this]);
        // The float arrays hold 16 bytes; the character arrays were reserved with the
        // capacities 5, 6, 4 and 4 (in this order of freeing).
        let sizes = [16, 16, 16, 16, 16, 5, 6, 4, 4];
        let expected: Vec<Words> = owned
            .iter()
            .zip(sizes)
            .map(|(data, size)| vec![*data, size])
            .collect();
        let freed = logged(&log, 0x10);
        assert_eq!(freed, expected);
        assert!(freed.iter().all(|words| words[0] != unowned_float));
        assert!(freed.iter().all(|words| words[0] != unowned_text));
        // Every array ends empty and unowned, and the base vtable is left.
        for array in [
            0x8, 0x14, 0x20, 0x2c, 0x38, 0x44, 0x50, 0x5c, 0x68, 0x74, 0x80,
        ] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), NOT_OWNED);
        }
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    /// A container (vtable and a pointer array of capacity `capacity`) for the tests.
    fn container(e: &mut Engine, capacity: u32) -> u32 {
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(4 * capacity);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0x10, capacity);
        this
    }

    #[test]
    fn creating_an_info_adds_it_to_the_container() {
        let (mut e, log, allocator) = fixture();
        let this = container(&mut e, 1);
        let names: Vec<u32> = [&b"a"[..], b"bb", b"c", b"d", b"e"]
            .iter()
            .map(|name| text(&mut e, name))
            .collect();
        let args_list = args![
            this, 2u32, 1.5f32, 2.5f32, 3.5f32, names[0], names[1], names[2], names[3], names[4]
        ];
        let first = e.call(0x00e4_0ec0, &args_list).u32();
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(e.mem.u32(this + 8)), first);
        assert_eq!(logged(&log, 4), vec![vec![0x98]]);
        assert_eq!(e.mem.u16(first + 4), 0x98);
        assert_eq!(e.mem.u32(first), INFO_VTABLE);
        assert_eq!(e.mem.f32(first + 0x94), 1.5);
        assert!(logged(&log, ARRAY_RESERVE_MORE).is_empty());
        assert_eq!(logged(&log, ADD_REFERENCE), vec![vec![first]; 2]);
        assert_eq!(logged(&log, REMOVE_REFERENCE), vec![vec![first]; 2]);
        // The array is full now: it grows before the next entry.
        let second = e.call(0x00e4_0ec0, &args_list).u32();
        assert_ne!(second, first);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let grown = logged(&log, ARRAY_RESERVE_MORE);
        assert_eq!(grown, [vec![allocator, this + 8, 4]]);
        let entries = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(entries), first);
        assert_eq!(e.mem.u32(entries + 4), second);
    }

    #[test]
    fn the_container_destructor_releases_its_entries() {
        let (mut e, log, _) = fixture();
        let this = container(&mut e, 4);
        let data = e.mem.u32(this + 8);
        e.mem.set_u32(data, 0x1111);
        e.mem.set_u32(data + 8, 0x2222);
        e.mem.set_u32(this + 0xc, 3);
        e.call(0x00e4_0fd0, &args![this]);
        assert_eq!(logged(&log, REMOVE_REFERENCE), [vec![0x1111], vec![0x2222]]);
        assert_eq!(e.mem.u32(data), 0);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(logged(&log, 0x10), [vec![data, 16]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x10), NOT_OWNED);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        // An empty container with storage that is not owned frees nothing.
        log.borrow_mut().clear();
        let bare = container(&mut e, 0);
        e.mem.set_u32(bare + 0x10, NOT_OWNED);
        e.call(0x00e4_0fd0, &args![bare]);
        assert!(log.borrow().is_empty());
    }
}
