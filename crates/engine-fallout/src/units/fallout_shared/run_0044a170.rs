//! `fallout shared/run_0044a170` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0044a380` calls on the member (`0044a560`).
const CALLEE_0044A560: u32 = 0x0044_a560;

// Translated from 0044a380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044a560` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_0044a380(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_0044A560, &args![this.byte_add(0xc)]).ptr()
}

// Translated from 0044a3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044a3d0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044a3a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_a3d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0044a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044a400` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044a420(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_a400, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The vector's words (an `std::vector` of words, MSVC layout): the
/// allocator at +8, first +0xC, last +0x10, end of storage +0x14.
const ALLOCATOR: u32 = 0x8;
const FIRST: u32 = 0xc;
const LAST: u32 = 0x10;
const END: u32 = 0x14;
/// The iterator constructor (`0065e000`, `thiscall (iterator, pointer,
/// vector)`), the iterator from a position (`0065ded0`, `thiscall (vector,
/// &result, position, vector word)`), the iterator compare (`0065dfd0`,
/// `thiscall (iterator, other)`, answers a byte), the element move
/// (`0044d600`, `cdecl (from, to, destination)`, answers the new end), the
/// element destruction (`0065f2b0`, `thiscall (vector, from, to)`), the
/// length error (`0065f640`), the allocator's allocate (`0044abd0`,
/// `thiscall (allocator, count)`) and its `max_size` (`0044abf0`).
const ITERATOR_CONSTRUCT: u32 = 0x0065_e000;
const ITERATOR_AT: u32 = 0x0065_ded0;
const ITERATOR_DIFFERS: u32 = 0x0065_dfd0;
const MOVE_ELEMENTS: u32 = 0x0044_d600;
const DESTROY_ELEMENTS: u32 = 0x0065_f2b0;
const LENGTH_ERROR: u32 = 0x0065_f640;
const ALLOCATE: u32 = 0x0044_abd0;
const ALLOCATOR_MAX_SIZE: u32 = 0x0044_abf0;

// Translated from 0044a170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`std::vector::begin` shape): constructs the
/// iterator at `result` from the first element and the vector (`0065e000`)
/// and answers `result`.
pub fn fn_0044a170(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let first = e.mem.u32(this.addr().wrapping_add(FIRST));
    e.call(ITERATOR_CONSTRUCT, &args![result, first, this]);
    result
}

// Translated from 0044a1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`std::vector::erase(first, last)` shape): makes
/// iterators of the two positions (`0065ded0`, each from its two words);
/// when they differ (`0065dfd0`) moves the elements after the range down
/// (`0044d600`), destroys the left-over tail (`0065f2b0`) and moves the end;
/// answers `result` holding the first iterator.
pub fn fn_0044a1a0(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    first_word_1: u32,
    first_word_2: u32,
    last_word_1: u32,
    last_word_2: u32,
) -> Ptr {
    e.with_stack(0x10, |e, frame| {
        let first = frame.addr() + 8;
        let last = frame.addr();
        e.call(ITERATOR_AT, &args![this, first, first_word_1, first_word_2]);
        e.call(ITERATOR_AT, &args![this, last, last_word_1, last_word_2]);
        if e.call(ITERATOR_DIFFERS, &args![first, last]).u8() != 0 {
            let end = e.mem.u32(this.addr() + LAST);
            let new_end = e
                .call(
                    MOVE_ELEMENTS,
                    &args![e.mem.u32(last + 4), end, e.mem.u32(first + 4)],
                )
                .u32();
            let end = e.mem.u32(this.addr() + LAST);
            e.call(DESTROY_ELEMENTS, &args![this, new_end, end]);
            e.mem.set_u32(this.addr() + LAST, new_end);
        }
        let words = [e.mem.u32(first), e.mem.u32(first + 4)];
        e.mem.set_u32(result.addr(), words[0]);
        e.mem.set_u32(result.addr() + 4, words[1]);
    });
    result
}

// Translated from 0044a2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`std::vector::max_size` shape): the
/// allocator's (`0044abf0` on `this + 8`).
pub fn fn_0044a2c0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(
        ALLOCATOR_MAX_SIZE,
        &args![this.addr().wrapping_add(ALLOCATOR)],
    )
    .u32()
}

// Translated from 0044a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`std::vector::_Buy` shape): clears first,
/// last and end; false for a count of 0; above `max_size` the length error
/// (`0065f640`, which throws; the exe answers true after it); otherwise
/// storage for `count` words from the allocator (`0044abd0`) with last at
/// its start and end `count` words on. Answers true.
pub fn fn_0044a240(e: &mut Engine, this: Ptr, count: u32) -> bool {
    let at = |offset: u32| this.addr().wrapping_add(offset);
    e.mem.set_u32(at(FIRST), 0);
    e.mem.set_u32(at(LAST), 0);
    e.mem.set_u32(at(END), 0);
    if count == 0 {
        return false;
    }
    if fn_0044a2c0(e, this) < count {
        e.call(LENGTH_ERROR, &args![]);
        return true;
    }
    let storage = e.call(ALLOCATE, &args![at(ALLOCATOR), count]).u32();
    e.mem.set_u32(at(FIRST), storage);
    e.mem.set_u32(at(LAST), storage);
    e.mem
        .set_u32(at(END), storage.wrapping_add(count.wrapping_mul(4)));
    true
}

/// `std::logic_error`'s vtable (`010173e8`; RTTI `std::logic_error`, read
/// through its locator) and `std::length_error`'s (`010173dc`).
const LOGIC_ERROR_VTABLE: u32 = 0x0101_73e8;
const LENGTH_ERROR_VTABLE: u32 = 0x0101_73dc;
/// `00ec78e0` (`thiscall`, no stack word): the `std::exception` base
/// constructor.
const EXCEPTION_CONSTRUCT: u32 = 0x00ec_78e0;
/// `00ec7944` (`thiscall`, `RET 4`): the `std::exception` copy constructor.
const EXCEPTION_COPY: u32 = 0x00ec_7944;
/// `std::exception::~exception` (engine map, `00ec79a1`, `thiscall`).
const EXCEPTION_DESTRUCT: u32 = 0x00ec_79a1;
/// `0044a500` (`thiscall`, `RET 4`): copies a message string into the
/// string at `this`.
const MESSAGE_COPY: u32 = 0x0044_a500;
/// `0044a540` (`thiscall`, no stack word): the message string's destructor.
const MESSAGE_DESTRUCT: u32 = 0x0044_a540;

// Translated from 0044a2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::length_error`'s constructor from a message (`thiscall`, `RET 4`;
/// the class from the vtable's RTTI): the `std::logic_error` constructor
/// `0044a310` with the message, then the vtable `010173dc`. Returns `this`.
pub fn fn_0044a2e0(e: &mut Engine, this: Ptr, message: u32) -> Ptr {
    fn_0044a310(e, this, message);
    e.mem.set_u32(this.addr(), LENGTH_ERROR_VTABLE);
    this
}

// Translated from 0044a310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044a310 - the exception-unwinding frame (FS:[0]) that destroys the base on a throw is not translated.
/// `std::logic_error`'s constructor from a message (`thiscall`, `RET 4`):
/// the `std::exception` base (`00ec78e0`), the vtable `010173e8`, then the
/// message copied into the string at +0xc (`0044a500`). Returns `this`.
pub fn fn_0044a310(e: &mut Engine, this: Ptr, message: u32) -> Ptr {
    e.call(EXCEPTION_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), LOGIC_ERROR_VTABLE);
    e.call(MESSAGE_COPY, &args![this.addr().wrapping_add(0xc), message]);
    this
}

// Translated from 0044a3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::logic_error`'s destructor body (no stack word): the vtable
/// `010173e8`, the message string's destructor (`0044a540` on +0xc), then
/// `std::exception::~exception` (`00ec79a1`).
pub fn fn_0044a3d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LOGIC_ERROR_VTABLE);
    e.call(MESSAGE_DESTRUCT, &args![this.addr().wrapping_add(0xc)]);
    e.call(EXCEPTION_DESTRUCT, &args![this]);
}

// Translated from 0044a400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::length_error`'s destructor body (no stack word): the vtable
/// `010173dc`, then the `std::logic_error` one (`0044a3d0`).
pub fn fn_0044a400(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LENGTH_ERROR_VTABLE);
    fn_0044a3d0(e, this);
}

// Translated from 0044a450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044a450 - the exception-unwinding frame (FS:[0]) that destroys the base on a throw is not translated.
/// `std::logic_error`'s copy constructor (`thiscall`, `RET 4`): the
/// `std::exception` copy (`00ec7944`), the vtable `010173e8`, then the
/// other's message (+0xc) copied into this one's (`0044a500`). Returns
/// `this`.
pub fn fn_0044a450(e: &mut Engine, this: Ptr, other: u32) -> Ptr {
    e.call(EXCEPTION_COPY, &args![this, other]);
    e.mem.set_u32(this.addr(), LOGIC_ERROR_VTABLE);
    e.call(
        MESSAGE_COPY,
        &args![this.addr().wrapping_add(0xc), other.wrapping_add(0xc)],
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_a2e0, fn_0044a2e0(Ptr, u32) -> Ptr),
        entry!(0x0044_a310, fn_0044a310(Ptr, u32) -> Ptr),
        entry!(0x0044_a3d0, fn_0044a3d0(Ptr)),
        entry!(0x0044_a400, fn_0044a400(Ptr)),
        entry!(0x0044_a450, fn_0044a450(Ptr, u32) -> Ptr),
        entry!(0x0044_a380, fn_0044a380(Ptr) -> Ptr),
        entry!(0x0044_a3a0, fn_0044a3a0(Ptr, u32) -> Ptr),
        entry!(0x0044_a420, fn_0044a420(Ptr, u32) -> Ptr),
        entry!(0x0044a170, fn_0044a170(Ptr, Ptr) -> Ptr),
        entry!(0x0044a1a0, fn_0044a1a0(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x0044a2c0, fn_0044a2c0(Ptr) -> u32),
        entry!(0x0044a240, fn_0044a240(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0044a380_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044A560, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0044_a380, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044A560).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
    #[test]
    fn at_0044a3a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_a3d0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_a3a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_a3a0)
                .collect();
            let mut expected = vec![(0x0044_a3d0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0044a420_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_a400, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_a420, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_a420)
                .collect();
            let mut expected = vec![(0x0044_a400, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn begin_builds_the_iterator_from_the_first_element() {
        let mut e = Engine::new();
        e.register(ITERATOR_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + FIRST, 0x4444);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044a170, &args![this, 0x9000u32]).u32(), 0x9000);
        assert_eq!(
            calls(&e, ITERATOR_CONSTRUCT),
            vec![vec![0x9000, 0x4444, this]]
        );
    }

    #[test]
    fn erase_moves_the_tail_down_when_the_range_is_not_empty() {
        for differs in [true, false] {
            let mut e = Engine::new();
            // An iterator is (vector, position): the position is the second word.
            e.register(ITERATOR_AT, |e, a| {
                e.mem.set_u32(a[1], a[2]);
                e.mem.set_u32(a[1] + 4, a[3]);
                Ret::default()
            });
            e.map(0x7300_0000, 4);
            e.mem.set_u32(0x7300_0000, u32::from(differs));
            e.register(ITERATOR_DIFFERS, |e, _| e.mem.u32(0x7300_0000).into_ret());
            e.register(MOVE_ELEMENTS, |_, a| (a[2] + (a[1] - a[0])).into_ret());
            e.register(DESTROY_ELEMENTS, |_, _| Ret::default());
            let this = e.mem.alloc(0x18);
            e.mem.set_u32(this + LAST, 0x1040);
            let result = e.mem.alloc(8);
            e.call_log = Some(vec![]);
            let r = e
                .call(
                    0x0044a1a0,
                    &args![this, result, this, 0x1010u32, this, 0x1020u32],
                )
                .u32();
            assert_eq!(r, result);
            assert_eq!((e.mem.u32(result), e.mem.u32(result + 4)), (this, 0x1010));
            if differs {
                assert_eq!(calls(&e, MOVE_ELEMENTS), vec![vec![0x1020, 0x1040, 0x1010]]);
                assert_eq!(
                    calls(&e, DESTROY_ELEMENTS),
                    vec![vec![this, 0x1030, 0x1040]]
                );
                assert_eq!(e.mem.u32(this + LAST), 0x1030);
            } else {
                assert!(calls(&e, MOVE_ELEMENTS).is_empty());
                assert_eq!(e.mem.u32(this + LAST), 0x1040);
            }
        }
    }

    #[test]
    fn buy_allocates_the_storage() {
        let mut e = Engine::new();
        e.register(ALLOCATOR_MAX_SIZE, |_, _| 100u32.into_ret());
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(LENGTH_ERROR, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + FIRST, 1);
        assert!(!e.call(0x0044a240, &args![this, 0u32]).bool());
        assert_eq!(e.mem.u32(this + FIRST), 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044a240, &args![this, 3u32]).bool());
        assert_eq!(calls(&e, ALLOCATE), vec![vec![this + 8, 3]]);
        assert_eq!(calls(&e, ALLOCATOR_MAX_SIZE), vec![vec![this + 8]]);
        assert_eq!(
            (
                e.mem.u32(this + FIRST),
                e.mem.u32(this + LAST),
                e.mem.u32(this + END)
            ),
            (0x5000, 0x5000, 0x500c)
        );
        // Above max_size: the length error, nothing allocated.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044a240, &args![this, 101u32]).bool());
        assert_eq!(calls(&e, LENGTH_ERROR).len(), 1);
        assert!(calls(&e, ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(this + FIRST), 0);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            EXCEPTION_CONSTRUCT,
            EXCEPTION_COPY,
            EXCEPTION_DESTRUCT,
            MESSAGE_COPY,
            MESSAGE_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
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

    #[test]
    fn at_0044a310_and_0044a2e0_construct_from_a_message() {
        let mut e = engine();
        let this = e.mem.alloc(0x28);
        let (result, log) = run(&mut e, 0x0044_a310, &[this, 0x5000]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), LOGIC_ERROR_VTABLE);
        assert_eq!(
            log,
            vec![
                (EXCEPTION_CONSTRUCT, vec![this]),
                (MESSAGE_COPY, vec![this + 0xc, 0x5000]),
            ]
        );
        let (result, log) = run(&mut e, 0x0044_a2e0, &[this, 0x5000]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), LENGTH_ERROR_VTABLE);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn at_0044a3d0_and_0044a400_destroy() {
        let mut e = engine();
        let this = e.mem.alloc(0x28);
        let (_, log) = run(&mut e, 0x0044_a400, &[this]);
        assert_eq!(e.mem.u32(this), LOGIC_ERROR_VTABLE);
        assert_eq!(
            log,
            vec![
                (MESSAGE_DESTRUCT, vec![this + 0xc]),
                (EXCEPTION_DESTRUCT, vec![this]),
            ]
        );
        e.mem.set_u32(this, 0);
        let (_, log) = run(&mut e, 0x0044_a3d0, &[this]);
        assert_eq!(e.mem.u32(this), LOGIC_ERROR_VTABLE);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn at_0044a450_copies() {
        let mut e = engine();
        let this = e.mem.alloc(0x28);
        let (result, log) = run(&mut e, 0x0044_a450, &[this, 0x5000]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), LOGIC_ERROR_VTABLE);
        assert_eq!(
            log,
            vec![
                (EXCEPTION_COPY, vec![this, 0x5000]),
                (MESSAGE_COPY, vec![this + 0xc, 0x500c]),
            ]
        );
    }
}
